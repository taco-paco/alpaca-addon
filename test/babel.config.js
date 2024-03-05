const alpacaAddon = "./node_modules/alpaca-addon";

module.exports = {
    presets: [
        [
            '@babel/preset-env',
            {
                targets: {
                    electron: '18.3.0',
                },
                useBuiltIns: 'usage',
                corejs: '3.22.7',
            },
        ],
        '@babel/preset-typescript',
    ],
    plugins: [
        'lodash',
        [
            'module-resolver',
            {
                alias: {
                    "alpaca-addon": alpacaAddon,
                },
            },
        ],
    ],
};
