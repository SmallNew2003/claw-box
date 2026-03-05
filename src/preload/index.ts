import { contextBridge } from 'electron';

contextBridge.exposeInMainWorld('claw', {
  ping: () => 'pong'
});
