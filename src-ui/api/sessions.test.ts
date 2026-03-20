import test from 'node:test';
import assert from 'node:assert/strict';
import { createLocalSessionApi } from './sessions.ts';

test('create session and run one execution', async () => {
  const api = createLocalSessionApi();
  const created = await api.createSession('测试会话');

  const result = await api.executeMessage(created.session_id, {
    content: 'hello runtime'
  });

  assert.equal(result.session.status, 'completed');
  assert.ok(result.events.some((event) => event.type === 'execution_completed'));
});

test('tool fail marks execution failed', async () => {
  const api = createLocalSessionApi();
  const created = await api.createSession('工具失败会话');

  const result = await api.executeMessage(created.session_id, {
    content: 'run tool',
    tool_name: 'fail'
  });

  assert.equal(result.session.status, 'failed');
  assert.ok(result.toolCalls.some((toolCall) => toolCall.status === 'failed'));
});
