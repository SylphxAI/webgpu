#!/usr/bin/env bun

const fs = require('fs');
const path = require('path');

const packagePath = path.join(__dirname, '..', 'package.json');
const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));

const packages = [
  { name: pkg.name, version: pkg.version },
  ...Object.entries(pkg.optionalDependencies || {}).map(([name, version]) => ({
    name,
    version,
  })),
];

function readRegistryVersion(name, version) {
  const selector = `${name}@${version}`;
  const result = Bun.spawnSync([process.execPath, 'pm', 'view', selector, 'version', '--json'], {
    stdout: 'pipe',
    stderr: 'pipe',
  });

  if (!result.success) {
    const stdout = new TextDecoder().decode(result.stdout).trim();
    const stderr = new TextDecoder().decode(result.stderr).trim();
    throw new Error(
      [
        `npm registry readback failed for ${selector}`,
        stdout ? `stdout:\n${stdout}` : '',
        stderr ? `stderr:\n${stderr}` : '',
      ]
        .filter(Boolean)
        .join('\n'),
    );
  }

  return JSON.parse(new TextDecoder().decode(result.stdout).trim());
}

for (const item of packages) {
  if (!item.name || !item.version) {
    throw new Error(`Invalid package readback target: ${JSON.stringify(item)}`);
  }

  const registryVersion = readRegistryVersion(item.name, item.version);
  if (registryVersion !== item.version) {
    throw new Error(
      `Registry readback mismatch for ${item.name}@${item.version}: got ${registryVersion}`,
    );
  }

  console.log(`npm registry readback passed: ${item.name}@${item.version}`);
}
