import { spawn } from 'node:child_process';
import net from 'node:net';

const HOST = '127.0.0.1';
const PORT = 9000;
const pnpmCommand = process.platform === 'win32' ? (process.env.ComSpec ?? 'cmd.exe') : 'pnpm';
const pnpmArgs =
  process.platform === 'win32'
    ? ['/d', '/s', '/c', `pnpm exec quasar dev --port ${PORT} --hostname ${HOST}`]
    : ['exec', 'quasar', 'dev', '--port', String(PORT), '--hostname', HOST];

function isPortInUse(port, host) {
  return new Promise((resolve) => {
    const socket = net.connect({ port, host });

    socket.once('connect', () => {
      socket.destroy();
      resolve(true);
    });

    socket.once('error', () => {
      resolve(false);
    });
  });
}

if (await isPortInUse(PORT, HOST)) {
  console.error(
    `[tauri-dev] ${HOST}:${PORT} is already in use. ` +
      'Stop the existing dev server before running "pnpm tauri dev".',
  );
  process.exit(1);
}

const child = spawn(pnpmCommand, pnpmArgs, {
  stdio: 'inherit',
  env: {
    ...process.env,
    TAURI_DEV: '1',
  },
});

const forwardSignal = (signal) => {
  if (child.killed === false) {
    child.kill(signal);
  }
};

process.on('SIGINT', () => forwardSignal('SIGINT'));
process.on('SIGTERM', () => forwardSignal('SIGTERM'));

child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }

  process.exit(code ?? 0);
});

child.on('error', (error) => {
  console.error('[tauri-dev] failed to start Quasar dev server:', error);
  process.exit(1);
});
