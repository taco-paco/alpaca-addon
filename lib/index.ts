import { createDevnetServer, ProviderCallback } from './getAlpaca';
import { createPromise } from './src/promise';
import { DevnetConfig } from './src/types';

export * from './src/types';
export * from './src/promise';
export * from './src/error'

export class Devnet {
    // TODO: proper return type
    static start(config: DevnetConfig, provider: ProviderCallback): Promise<string[]> {
        return createPromise(createDevnetServer, config, provider);
    }
}
