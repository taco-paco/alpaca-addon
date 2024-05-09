import path from 'path';

export enum SupportedArch {
    x64 = 'x64',
    arm64 = 'arm64',
}

export function getTargetArchName(arch: SupportedArch): string {
    switch (arch) {
        case SupportedArch.x64:
            return 'x86_64';
        case SupportedArch.arm64:
            return 'aarch64';
    }
}

export function getTargetPlatformName() {
    switch (process.platform) {
        case 'darwin':
            return 'apple-darwin';

        case 'linux':
            return 'unknown-linux-gnu';

        case 'win32':
            return 'pc-windows-msvc';

        default:
            throw Error(`unsupported cross-compilation platform ${process.platform}`);
    }
}

export function obtainCrossCompilationTarget(arch?: SupportedArch): string | null {
    if (!arch) {
        return null;
    }

    if (process.arch === arch) {
        // Native arch is the same as target arch
        // TODO: throw new Error("") instead?
        return null;
    }

    const targetArchName = getTargetArchName(arch);
    const targetPlatform = getTargetPlatformName();

    return `${targetArchName}-${targetPlatform}`;
}

export function getBuildConfigurationName(isDev: boolean) {
    return isDev ? 'debug' : 'release';
}

export function isDebugBuildConfiguration(configuration: string) {
    return configuration === getBuildConfigurationName(true);
}

function getArtifactsFolder(isDev: boolean, arch?: SupportedArch) {
    let pathSegments = ['node', 'target'];
    const target = obtainCrossCompilationTarget(arch);
    if (target) {
        pathSegments.push(target);
    }

    pathSegments.push(getBuildConfigurationName(isDev));

    return path.join(...pathSegments);
}

export function getTargetModuleFilePath(isDev: boolean, arch?: SupportedArch) {
    const moduleFileName = choosePlatformSpecificOption({
        win: 'node_module.dll',
        mac: 'libnode_module.dylib',
        linux: 'libnode_module.so',
    });

    return path.join(getArtifactsFolder(isDev, arch), moduleFileName);
}

export function choosePlatformSpecificOption<T>(options: { win?: T; mac?: T; linux?: T }): T {
    if (options.win && process.platform === 'win32') {
        return options.win;
    } else if (options.mac && process.platform === 'darwin') {
        return options.mac;
    } else if (options.linux && process.platform === 'linux') {
        return options.linux;
    }

    throw new Error(`Unsupported platform: ${process.platform}`);
}
