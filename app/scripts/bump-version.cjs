'use strict';

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const releaseType = process.argv[2];
if (!['patch', 'minor', 'major'].includes(releaseType)) {
  console.error('Usage: node bump-version.cjs <patch|minor|major>');
  process.exit(1);
}

const root = path.resolve(__dirname, '..');

// Read and bump version
const pkgPath = path.join(root, 'package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
const [maj, min, pat] = pkg.version.split('.').map(Number);

let newVersion;
if (releaseType === 'major')      newVersion = `${maj + 1}.0.0`;
else if (releaseType === 'minor') newVersion = `${maj}.${min + 1}.0`;
else                               newVersion = `${maj}.${min}.${pat + 1}`;

console.log(`Bumping ${pkg.version} → ${newVersion}`);

// package.json
pkg.version = newVersion;
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');

// src-tauri/tauri.conf.json
const tauriConfPath = path.join(root, 'src-tauri', 'tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
tauriConf.version = newVersion;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');

// src-tauri/Cargo.toml  (replace first `version = "..."` line, per existing updater logic)
const cargoPath = path.join(root, 'src-tauri', 'Cargo.toml');
const cargo = fs.readFileSync(cargoPath, 'utf8');
const updated = cargo.replace(/^(\s*)version\s*=\s*"[^"]+"/m, `$1version = "${newVersion}"`);
if (updated === cargo) throw new Error('version not found in Cargo.toml');
fs.writeFileSync(cargoPath, updated);

// src-tauri/Cargo.lock  (sync only our own package version, other deps stay locked)
const cargoLockPath = path.join(root, 'src-tauri', 'Cargo.lock');
execSync('cargo update -p free-fps', { cwd: path.join(root, 'src-tauri'), stdio: 'inherit' });

// Git commit + tag (same convention standard-version used)
const files = [pkgPath, tauriConfPath, cargoPath, cargoLockPath].map(f => `"${f}"`).join(' ');
execSync(`git add ${files}`, { stdio: 'inherit' });
execSync(`git commit -m "chore(release): ${newVersion}"`, { stdio: 'inherit' });
execSync(`git tag v${newVersion}`, { stdio: 'inherit' });

console.log(`Released v${newVersion}`);
