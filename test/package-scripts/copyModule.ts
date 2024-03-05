import fs from 'fs-extra';
import path from 'path';

console.log('[i] Copying alpaca package into node_modules folder');
fs.copySync(path.join('..', 'deliver'), path.join('node_modules', 'alpaca-addon'));
