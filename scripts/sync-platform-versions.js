#!/usr/bin/env node

/**
 * Syncs optionalDependencies versions to match the current package.json version
 * Run after `changeset version` to ensure platform packages match main package version
 */

const fs = require('fs');
const path = require('path');

const packageJsonPath = path.join(__dirname, '..', 'package.json');
const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

const currentVersion = pkg.version;
const optionalDeps = pkg.optionalDependencies || {};

let updated = false;

for (const [name, version] of Object.entries(optionalDeps)) {
  if (version !== currentVersion) {
    console.log(`Updating ${name}: ${version} -> ${currentVersion}`);
    optionalDeps[name] = currentVersion;
    updated = true;
  }
}

if (updated) {
  pkg.optionalDependencies = optionalDeps;
  fs.writeFileSync(packageJsonPath, JSON.stringify(pkg, null, 2) + '\n');
  console.log(`✅ Updated optionalDependencies to version ${currentVersion}`);
} else {
  console.log(`✅ optionalDependencies already at version ${currentVersion}`);
}
