import {DevnetConfig} from "./src/types";
import {ResolverCallback} from "./src/promise";

export type ProviderCallback = (val: string) => void;
export function createDevnetServer(callback: ResolverCallback<string[]>, config: DevnetConfig, provider: ProviderCallback): void;
