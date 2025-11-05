'use strict';

const VERSION_RE = /^([^\S\r\n]*)version\s*=\s*"([^"]+)"/m;

module.exports.readVersion = function readVersion(contents) {
    const match = contents.match(VERSION_RE);
    if (!match) {
        throw new Error('version not found in Cargo.toml');
    }
    return match[2]; // captured version
};

module.exports.writeVersion = function writeVersion(contents, version) {
    if (!VERSION_RE.test(contents)) {
        throw new Error('version not found in Cargo.toml');
    }
    // Preserve original indentation
    return contents.replace(VERSION_RE, (_m, indent) => `${indent}version = "${version}"`);
};
