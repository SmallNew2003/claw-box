import { defineConfig } from 'electron-vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';

export default defineConfig({
  main: {
    entry: 'src/main/index.ts'
  },
  preload: {
    entry: 'src/preload/index.ts'
  },
  renderer: {
    root: 'src/renderer',
    build: {
      outDir: resolve(__dirname, 'dist/renderer')
    },
    plugins: [react()]
  }
});
