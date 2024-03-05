# Alpaca-addon

## About
The heart of [Alpaca](https://github.com/taco-paco/alpaca) UI. 
It works by tapping into JS [V8](https://v8.dev/) engine via bindings
to [N-API](https://github.com/nodejs/node-addon-api) provided by [Neon](https://github.com/neon-bindings/neon/tree/main). Essentially this is a DLL
that manipulates object within [NodeJS](https://nodejs.org/en) process of Alpaca called [Engine](https://github.com/taco-paco/alpaca/tree/master/app/engine) process 
and that can be assembled into an [NPM](https://www.npmjs.com/package/@taco-paco/alpaca-addon-mac-x64) module.
It creates a [Devnet](https://github.com/0xSpaceShard/starknet-devnet-rs) that directly passes results to JS engine.

## Setup
To work with [Alpaca](https://github.com/taco-paco/alpaca) project needs to be compiled within [electron-build](https://github.com/electron-userland/electron-builder) environment.

### Compilation
To install dependencies run:
```bash
yarn
```

To compile the project use:

```bash
yarn start build
```

For debug version run:

```bash
yarn start build.dev
```

For cross-compiling the project run:
```bash
yarn start build.<target-arch>
```

For now only _arm64_ and _x64_ options are supported.

### Creating an NPM package:
```bash
yarn start pack
```

If you cross-compiled the project run:
```bash
yarn start pack.<target-arch>
```

## Testing
After compilation and packing.
Within _test_ directory run:

```bash
yarn
yarn start prepare
yarn start test
```
