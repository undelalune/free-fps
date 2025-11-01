'use strict';

const VERSION_RE = /^\s*version\s*=\s*"([^"]+)"/m;

module.exports.readVersion = function readVersion(contents) {
    const match = contents.match(VERSION_RE);
    if (!match) {
        throw new Error('version not found in Cargo.toml');
    }
    return match[1];
};

module.exports.writeVersion = function writeVersion(contents, version) {
    if (!VERSION_RE.test(contents)) {
        throw new Error('version not found in Cargo.toml');
    }
    return contents.replace(VERSION_RE, `version = "${version}"`);
};