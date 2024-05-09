import path from 'path';
import fs from 'fs-extra';
import { choosePlatformSpecificOption, getTargetModuleFilePath, isDebugBuildConfiguration, SupportedArch } from './utils';

function parseArch(rawArch: string): SupportedArch | undefined {
    if (rawArch == process.arch) {
        return undefined;
    }

    switch (rawArch) {
        case 'x64':
            return SupportedArch.x64;
        case 'arm64':
            return SupportedArch.arm64;
        default:
            throw new Error(`Unsopported arch: ${rawArch}`);
    }
}

function getPackageName(arch: string) {
    const suffix = choosePlatformSpecificOption({
        win: 'win',
        mac: 'mac',
        linux: 'linux',
    });

    return `@taco-paco/alpaca-addon-${suffix}-${arch}`;
}

function makeNpmPackage() {
    const rootFolder = '.';
    const args = process.argv.slice(2);
    console.log('args', args);

    const packageFolder = path.join(rootFolder, args[0]);
    fs.ensureDirSync(packageFolder);

    const isDev = isDebugBuildConfiguration(args[1]);
    const arch = parseArch(args[2]);

    fs.copyFileSync(path.join(rootFolder, getTargetModuleFilePath(isDev, arch)), path.join(packageFolder, 'alpaca.node'));

    const packageInfo = {
        name: getPackageName(arch || process.arch),
        description: 'Rust addon that wraps around starknet-devnet-rs.',
        author: {
            name: 'Edwin Paco',
            email: 'edwinswatpako@gmail.com',
        },
        main: 'index.js',
        version: '0.0.2',
        os: [process.platform],
        cpu: [arch || process.arch],
        repository: {
            type: 'git',
            url: 'git@github.com:taco-paco/alpaca-addon.git',
        },
    };
    fs.writeJsonSync(path.join(packageFolder, 'package.json'), packageInfo, { spaces: 4 });
}

makeNpmPackage();
