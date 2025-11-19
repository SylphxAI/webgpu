#!/usr/bin/env node

/**
 * Publish script for @sylphx/webgpu
 *
 * This script is called by @changesets/action when there are changesets to publish.
 *
 * Publishing order:
 * 1. Publish all platform packages FIRST (they contain the .node binaries)
 * 2. Publish main package (which depends on platform packages via optionalDependencies)
 *
 * This ensures users can install the main package and get the correct platform binaries.
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function run(command, cwd = process.cwd()) {
  console.log(`Running: ${command}`);
  try {
    execSync(command, { cwd, stdio: 'inherit' });
  } catch (error) {
    console.error(`Failed to run: ${command}`);
    throw error;
  }
}

console.log('📦 Publishing @sylphx/webgpu packages...\n');

// Step 1: Publish all platform packages
console.log('Step 1: Publishing platform packages...');
const npmDir = path.join(__dirname, '..', 'npm');

if (!fs.existsSync(npmDir)) {
  console.error('❌ npm/ directory not found. Platform packages not built?');
  process.exit(1);
}

const platforms = fs.readdirSync(npmDir).filter(name => {
  const dirPath = path.join(npmDir, name);
  return fs.statSync(dirPath).isDirectory();
});

if (platforms.length === 0) {
  console.error('❌ No platform packages found in npm/ directory');
  process.exit(1);
}

console.log(`Found ${platforms.length} platform packages: ${platforms.join(', ')}\n`);

for (const platform of platforms) {
  const platformDir = path.join(npmDir, platform);
  console.log(`Publishing @sylphx/webgpu-${platform}...`);
  run('npm publish --access public', platformDir);
  console.log(`✅ Published @sylphx/webgpu-${platform}\n`);
}

// Step 2: Publish main package
console.log('Step 2: Publishing main package...');
run('npm publish --access public');
console.log('✅ Published @sylphx/webgpu\n');

console.log('🎉 All packages published successfully!');
