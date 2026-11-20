import { spawn, execFileSync } from 'node:child_process';
import { createInterface } from 'node:readline';
import { readFileSync } from 'node:fs';

const [, , root, executable, ...args] = process.argv;
const git = (...args) => execFileSync('git', args, { cwd: root, encoding: 'utf8' });
const source = /\.(rs|java|kt|ts|tsx|vue|js|jsx|mjs|cjs|xml|ya?ml|sql)$/;
const staged = git('diff', '--cached', '--name-only', '-z').split('\0').filter(file => source.test(file));

function detectChanges() {
    return new Promise((resolve, reject) => {
        const child = spawn(executable, [...args, 'mcp'], { cwd: root, stdio: ['pipe', 'pipe', 'inherit'] });
        const lines = createInterface({ input: child.stdout });
        const timer = setTimeout(() => finish(new Error('GitNexus MCP timed out')), 60_000);

        function finish(error, result) {
            clearTimeout(timer);
            lines.close();
            child.kill();
            if (error) reject(error);
            else resolve(result);
        }

        child.once('error', finish);
        child.once('exit', code => finish(new Error(`GitNexus MCP exited (${code})`)));
        child.stdin.on('error', error => finish(error));
        lines.on('line', line => {
            try {
                const reply = JSON.parse(line);
                if (reply.id !== 1) return;
                if (reply.error || reply.result?.isError) throw new Error(JSON.stringify(reply.error ?? reply.result));
                finish(null, reply.result.structuredContent ?? JSON.parse(reply.result.content[0].text.split('\n\n---\n', 1)[0]));
            } catch (error) {
                finish(error);
            }
        });

        const linked = git('rev-parse', '--absolute-git-dir').trim()
            !== git('rev-parse', '--path-format=absolute', '--git-common-dir').trim();
        child.stdin.write(JSON.stringify({
            jsonrpc: '2.0', id: 1, method: 'tools/call',
            params: { name: 'detect_changes', arguments: { scope: 'staged', repo: root, ...(linked && { worktree: root }) } },
        }) + '\n');
    });
}

try {
    const { lastCommit } = JSON.parse(readFileSync(`${root}/.gitnexus/meta.json`, 'utf8'));
    if (!lastCommit || lastCommit !== git('rev-parse', 'HEAD').trim()) {
        throw new Error('索引提交与当前 worktree HEAD 不一致');
    }

    const result = await detectChanges();
    if (result.error) throw new Error(result.error);
    const mapped = new Set(result.changed_symbols?.map(symbol => symbol.filePath));
    const unstaged = new Set(git('diff', '--name-only', '-z').split('\0'));
    const uncovered = staged.filter(file => !mapped.has(file) || unstaged.has(file));
    if (result.partial || result.truncated || uncovered.length) {
        throw new Error(`分析不完整。未确认文件：${JSON.stringify(uncovered)}`);
    }
    console.log(`[gitnexus] 已映射 ${staged.length} 个源码文件 worktree=${root} index=${lastCommit} risk=${result.summary.risk_level}`);
} catch (error) {
    console.error(`[gitnexus] ${error.message}`);
    process.exitCode = 1;
}
