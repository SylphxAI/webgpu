#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const pkg = require('../package.json');

const platforms = [
  { name: 'darwin-x64', target: 'x86_64-apple-darwin', platform: 'darwin', arch: 'x64' },
  { name: 'darwin-arm64', target: 'aarch64-apple-darwin', platform: 'darwin', arch: 'arm64' },
  { name: 'linux-x64-gnu', target: 'x86_64-unknown-linux-gnu', platform: 'linux', arch: 'x64', abi: 'gnu' },
  { name: 'linux-arm64-gnu', target: 'aarch64-unknown-linux-gnu', platform: 'linux', arch: 'arm64', abi: 'gnu' },
  { name: 'win32-x64-msvc', target: 'x86_64-pc-windows-msvc', platform: 'win32', arch: 'x64' },
  { name: 'win32-arm64-msvc', target: 'aarch64-pc-windows-msvc', platform: 'win32', arch: 'arm64' },
];

for (const platform of platforms) {
  const platformDir = path.join(__dirname, '..', 'npm', platform.name);
  const packageJsonPath = path.join(platformDir, 'package.json');

  // Check if .node file exists
  const nodeFiles = fs.readdirSync(platformDir).filter(f => f.endsWith('.node'));
  if (nodeFiles.length === 0) {
    console.error(`No .node file found in ${platformDir}`);
    process.exit(1);
  }

  const platformPkg = {
    name: `@sylphx/webgpu-${platform.name}`,
    version: pkg.version,
    description: `WebGPU native bindings for ${platform.name}`,
    main: nodeFiles[0],
    files: [nodeFiles[0]],
    license: pkg.license,
    repository: pkg.repository,
    keywords: pkg.keywords,
    os: [platform.platform],
    cpu: [platform.arch],
    engines: pkg.engines
  };

  if (platform.abi) {
    platformPkg.libc = [platform.abi];
  }

  fs.writeFileSync(packageJsonPath, JSON.stringify(platformPkg, null, 2) + '\n');
  console.log(`Created ${packageJsonPath}`);
}

console.log('All platform packages created successfully');
