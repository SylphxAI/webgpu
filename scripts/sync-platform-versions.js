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

const npmDir = path.join(__dirname, '..', 'npm');

if (fs.existsSync(npmDir)) {
  for (const entry of fs.readdirSync(npmDir)) {
    const platformPackageJsonPath = path.join(npmDir, entry, 'package.json');

    if (!fs.existsSync(platformPackageJsonPath)) {
      continue;
    }

    const platformPkg = JSON.parse(
      fs.readFileSync(platformPackageJsonPath, 'utf8'),
    );

    if (platformPkg.version !== currentVersion) {
      console.log(
        `Updating ${platformPkg.name}: ${platformPkg.version} -> ${currentVersion}`,
      );
      platformPkg.version = currentVersion;
      fs.writeFileSync(
        platformPackageJsonPath,
        JSON.stringify(platformPkg, null, 2) + '\n',
      );
      updated = true;
    }
  }
}

if (updated) {
  console.log(`✅ Platform package manifests synced to version ${currentVersion}`);
} else {
  console.log(`✅ Platform package manifests already at version ${currentVersion}`);
}
