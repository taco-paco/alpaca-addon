import build, { BuildOptions } from './build';
import pack, { PackOptions } from './pack';
import { SupportedArch } from './utils';

const npsUtils = require('nps-utils');
const nodeExe = 'node -r ./babel-register.js';
const archs: SupportedArch[] = Object.values(SupportedArch);

type Scripts = {
    [key: string]: string | Scripts;
};

export default {
    scripts: {
        build: {
            default: build(nodeExe, { isDev: false }),
            ...buildModuleForArchs({ isDev: false }),
            dev: {
                default: build(nodeExe, { isDev: true }),
                ...buildModuleForArchs({ isDev: true }),
            },
        },

        pack: {
            default: pack(nodeExe, { isDev: false }),
            ...packModuleForEditions({ isDev: false }),
            dev: {
                default: pack(nodeExe, { isDev: true }),
                ...packModuleForEditions({ isDev: true }),
            },
        },

        tsc: npsUtils.series('tsc -p tsconfig.all.json'),
    },
};

type DevScriptOptions = Omit<BuildOptions, 'arch?'>;
function buildModuleForArchs(props: DevScriptOptions) {
    const { isDev } = props;
    const scripts: Scripts = {};

    archs.forEach((arch) => {
        scripts[arch] = build(nodeExe, { isDev, arch });
    });

    return scripts;
}

type DevPackOptions = Omit<PackOptions, 'arch'>;
function packModuleForEditions(props: DevPackOptions) {
    const scripts: Scripts = {};

    archs.forEach((arch) => {
        scripts[arch] = pack(nodeExe, { ...props, arch });
    });

    return scripts;
}
