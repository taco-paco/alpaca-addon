import { getBuildConfigurationName, SupportedArch } from './utils';

const npsUtils = require('nps-utils');

export interface PackOptions {
    isDev: boolean;
    arch?: SupportedArch;
}

export default function pack(nodeExe: string, options: PackOptions) {
    const deliverFolder = 'deliver';
    const configuration = getBuildConfigurationName(options.isDev);
    const arch = options.arch || process.arch;

    const series = [
        `tsc -p tsconfig.json`,
        npsUtils.copy(`lib/alpaca.d.ts ${deliverFolder}/`),
        `${nodeExe} package-scripts/makeNpmPackage.ts ${deliverFolder} ${configuration} ${arch}`,
        `cd ${deliverFolder} && npm pack && cd ..`,
    ];

    return npsUtils.series(...series);
}
