import path from 'path';
import fs from 'fs';
import { getBuildConfigurationName, getTargetArchName, getTargetPlatformName, obtainCrossCompilationTarget, SupportedArch } from './utils';
import { BuildOptions } from './build';
const { execSync } = require('child_process');
const npsUtils = require('nps-utils');

function getCrossCompilationVar(targetArch: SupportedArch): string {
    switch (process.platform) {
        case 'linux':
            return getGccCrossCompilationFlags(targetArch);

        case 'darwin':
            return getClangCrossCompilationFlags(targetArch);

        default:
            throw new Error(`Cross compilation for ${process.platform} isn't supported`);
    }
}

function getGccCrossCompilerName(targetArch: SupportedArch): string {
    const archName = getTargetArchName(targetArch);
    return `${archName}-linux-gnu-gcc`;
}

function getGccCrossCompilationFlags(targetArch: SupportedArch) {
    if (process.platform !== 'linux') {
        throw new Error('GCC cross-compilation is only supported for linux(here)');
    }

    const compiler = getGccCrossCompilerName(targetArch);

    const tmp = `${getTargetArchName(targetArch)}_unknown_linux_gnu`;
    const ccEnvVar = `CC_${tmp}=${compiler}`;
    const cargoEnvVar = `CARGO_TARGET_${tmp.toUpperCase()}_LINKER=${compiler}`;

    return `${ccEnvVar} ${cargoEnvVar}`;
}

function getClangCrossCompilationFlags(targetArch: SupportedArch) {
    if (process.platform !== 'darwin') {
        throw new Error('Clang cross-compilation is only supported for darwin(here)');
    }

    // Need to specify: CARGO_TARGET_${arch}_APPLE_DARWIN_LINKER/CC_${arch}_apple_darwin?
    const ccEnvVar = `CC="clang --target=${obtainCrossCompilationTarget(targetArch)}"`;
    return `${ccEnvVar}`;
}

function prepareRustFlags(isDev: boolean) {
    // Uncomment in case of need to publish debug symbols
    // Note: they will have to be stripped from binary
    // let rustFlags = process.env.RUSTFLAGS || ' -g';
    let rustFlags = process.env.RUSTFLAGS || '';

    let linkArgs: string[] = [];
    let targetFeatures: string[] = [];

    if (process.platform === 'win32') {
        linkArgs = ['/PDBALTPATH:alpaca.pdb'];
        targetFeatures = targetFeatures.concat(['+crt-static']);
    } else {
        if (process.platform === 'darwin') {
            linkArgs = linkArgs.concat(['-framework', 'CoreFoundation', '-framework', 'IOKit', '-framework', 'Security']);
            // https://stackoverflow.com/questions/61195867/library-not-loaded-code-signing-blocked-on-macos-10-15-4
            const sdkPath = process.env.SDKROOT || execSync('xcrun --show-sdk-path', { encoding: 'utf-8' }).trim();
            if (!fs.existsSync(sdkPath)) {
                throw new Error(`MacOS SDK path does not exist: ${sdkPath}`);
            }

            linkArgs = linkArgs.concat(['-isysroot', sdkPath]);
            linkArgs = linkArgs.concat(['-Wl,-install_name,alpaca.node']);

            const configuration_folder = getBuildConfigurationName(isDev);
            // If decided to use .map for linux don't forget that its case-sensitive: -Map!
            linkArgs = linkArgs.concat(['-Xlinker', '-map', '-Xlinker', `target/${configuration_folder}/alpaca.map`]);
        }
    }

    const linkArgsFlags = linkArgs.map((arg) => `-Clink-arg=${arg}`).join(' ');
    const targetFeaturesFlags = targetFeatures.map((feature) => `-Ctarget-feature=${feature}`).join(' ');

    return `RUSTFLAGS="${rustFlags} ${linkArgsFlags} ${targetFeaturesFlags}"`;
}

function getOptimisationLevel(): string {
    return '';
}

export default function prepareEnv(options: BuildOptions) {
    const rustFlags = prepareRustFlags(options.isDev);
    const optimisationLevel = getOptimisationLevel();

    let env;
    if (options.arch && options.arch !== process.arch) {
        env = npsUtils.crossEnv(`${optimisationLevel} ${rustFlags} ${getCrossCompilationVar(options.arch)}`);
    } else {
        env = npsUtils.crossEnv(`${optimisationLevel} ${rustFlags}`);
    }

    return env.replace('node_modules', path.join('..', 'node_modules'));
}
