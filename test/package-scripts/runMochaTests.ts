const mochaRunner = [
    'electron-mocha',
    '--timeout 600000',
    // '--inspect-brk=5859',
    '--require ./babel-register.js',
    '--require regenerator-runtime/runtime',
    '--reporter mocha-multi-reporters',
].join(' ');

export function runMochaTests() {
    return `${mochaRunner} tests/test.ts`;
}
