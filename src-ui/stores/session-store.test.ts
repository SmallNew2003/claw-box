import test from 'node:test';
import assert from 'node:assert/strict';
import { createLocalSessionApi } from '../api/sessions.ts';
import { SessionStore } from './session-store.ts';

test('init loads sessions and default selection', async () => {
  const store = new SessionStore(createLocalSessionApi());

  await store.init();
  const state = store.getState();

  assert.ok(state.sessions.length > 0);
  assert.ok(state.selectedSessionId);
  assert.equal(state.pending, false);
});

test('send message updates selected session execution', async () => {
  const store = new SessionStore(createLocalSessionApi());
  await store.init();

  await store.sendMessage('hello');
  const state = store.getState();

  assert.equal(state.selectedSession?.status, 'completed');
  assert.ok(state.events.length > 0);
});
