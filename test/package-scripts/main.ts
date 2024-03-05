import { runMochaTests } from './runMochaTests';

const npsUtils = require('nps-utils');
const nodeExe = 'node -r ./babel-register.js';

export default {
    scripts: {
        prepare: `${nodeExe} package-scripts/copyModule.ts`,
        tests: runMochaTests(),
        tsc: npsUtils.series('tsc'),
    },
};
