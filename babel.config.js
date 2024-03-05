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
};
