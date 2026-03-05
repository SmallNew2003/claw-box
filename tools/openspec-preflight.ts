#!/usr/bin/env node
import { execSync, spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
const dryRun = args.includes("--dry-run");
const changeIdArgIndex = args.findIndex((arg) => arg === "--change-id");
const changeIdArg = changeIdArgIndex >= 0 ? args[changeIdArgIndex + 1] : undefined;

function runGitDiff(command: string): string[] {
  try {
    const output = execSync(command, { encoding: "utf8" }).trim();
    if (!output) return [];
    return output.split("\n").map((line) => line.trim()).filter(Boolean);
  } catch (error) {
    return [];
  }
}

const stagedFiles = runGitDiff("git diff --name-only --cached");
const workingFiles = stagedFiles.length > 0 ? stagedFiles : runGitDiff("git diff --name-only");

const ignoreFile = (file: string): boolean => {
  return (
    file === "README.md" ||
    file.startsWith("docs/") ||
    file.startsWith("openspec/") ||
    file.endsWith(".md")
  );
};

const triggerFile = (file: string): boolean => {
  return (
    file.startsWith("src/") ||
    file.startsWith("platform/") ||
    file.startsWith("apps/") ||
    file.startsWith("tools/") ||
    file.startsWith("electron") ||
    file.startsWith("tsconfig") ||
    file === "package.json"
  );
};

const hasChanges = workingFiles.length > 0;
const allIgnored = hasChanges && workingFiles.every(ignoreFile);
const needsOpenSpec = hasChanges && !allIgnored && workingFiles.some(triggerFile);

function collectChangeIds(files: string[]): string[] {
  const ids = new Set<string>();
  for (const file of files) {
    const match = file.match(/^openspec\/changes\/([^/]+)\//);
    if (match) ids.add(match[1]);
  }
  return Array.from(ids);
}

function resolveChangeId(files: string[]): string {
  if (changeIdArg) return changeIdArg;
  const ids = collectChangeIds(files);
  if (ids.length === 1) return ids[0];
  if (ids.length > 1) {
    throw new Error(`检测到多个 change-id: ${ids.join(", ")}，请使用 --change-id 指定。`);
  }
  const latest = findLatestChangeId();
  if (latest) return latest;
  throw new Error("未检测到 change-id，请使用 --change-id 指定。");
}

function findSpecFiles(specsDir: string): string[] {
  if (!fs.existsSync(specsDir)) return [];
  const entries = fs.readdirSync(specsDir, { withFileTypes: true });
  const specFiles: string[] = [];
  for (const entry of entries) {
    if (entry.isDirectory()) {
      const specPath = path.join(specsDir, entry.name, "spec.md");
      if (fs.existsSync(specPath)) specFiles.push(specPath);
    }
  }
  return specFiles;
}

function findLatestChangeId(): string | null {
  const baseDir = path.join("openspec", "changes");
  if (!fs.existsSync(baseDir)) return null;
  const entries = fs.readdirSync(baseDir, { withFileTypes: true });
  let latestId: string | null = null;
  let latestTime = 0;
  for (const entry of entries) {
    if (!entry.isDirectory()) continue;
    const proposalPath = path.join(baseDir, entry.name, "proposal.md");
    if (!fs.existsSync(proposalPath)) continue;
    const stat = fs.statSync(proposalPath);
    if (stat.mtimeMs > latestTime) {
      latestTime = stat.mtimeMs;
      latestId = entry.name;
    }
  }
  return latestId;
}

function printDryRunSummary(): void {
  const lines = [
    "OpenSpec preflight (dry-run)",
    `变更文件数: ${workingFiles.length}`,
    `需要 OpenSpec: ${needsOpenSpec ? "是" : "否"}`,
  ];
  if (needsOpenSpec) {
    try {
      const changeId = resolveChangeId(workingFiles);
      const changeDir = path.join("openspec", "changes", changeId);
      const proposalPath = path.join(changeDir, "proposal.md");
      const tasksPath = path.join(changeDir, "tasks.md");
      const specsDir = path.join(changeDir, "specs");
      const specFiles = findSpecFiles(specsDir);
      lines.push(`change-id: ${changeId}`);
      lines.push(`proposal.md: ${fs.existsSync(proposalPath) ? "存在" : "缺失"}`);
      lines.push(`tasks.md: ${fs.existsSync(tasksPath) ? "存在" : "缺失"}`);
      lines.push(`specs: ${specFiles.length > 0 ? "存在" : "缺失"}`);
    } catch (error) {
      lines.push(`change-id: ${error instanceof Error ? error.message : "解析失败"}`);
    }
  }
  console.log(lines.join("\n"));
}

if (dryRun) {
  printDryRunSummary();
  process.exit(0);
}

if (!needsOpenSpec) {
  console.log("OpenSpec preflight: 未触发检查（未检测到功能/破坏性变更）。");
  process.exit(0);
}

let changeId = "";
try {
  changeId = resolveChangeId(workingFiles);
} catch (error) {
  console.error(`OpenSpec preflight 失败: ${error instanceof Error ? error.message : "无法解析 change-id"}`);
  process.exit(1);
}

const changeDir = path.join("openspec", "changes", changeId);
const proposalPath = path.join(changeDir, "proposal.md");
const tasksPath = path.join(changeDir, "tasks.md");
const specsDir = path.join(changeDir, "specs");
const specFiles = findSpecFiles(specsDir);

const missing: string[] = [];
if (!fs.existsSync(changeDir)) missing.push(`缺少 ${changeDir}`);
if (!fs.existsSync(proposalPath)) missing.push("缺少 proposal.md");
if (!fs.existsSync(tasksPath)) missing.push("缺少 tasks.md");
if (specFiles.length === 0) missing.push("缺少 specs/*/spec.md");

if (missing.length > 0) {
  console.error(`OpenSpec preflight 失败: ${missing.join("; ")}`);
  process.exit(1);
}

const result = spawnSync("openspec", ["validate", changeId, "--strict", "--no-interactive"], {
  stdio: "inherit",
});

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

console.log("OpenSpec preflight: 通过。");
