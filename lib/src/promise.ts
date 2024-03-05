// TODO: use rust type
type AlpacaError = string;

export type ResolverCallback<T> = (error: AlpacaError, result: T) => void;
export function createPromise<T, A extends unknown[]>(fn: (callback: ResolverCallback<T>, ...args: A) => void, ...args: A): Promise<T> {
    return new Promise<T>((resolve, reject) => {
        fn(function (err: unknown, result: T) {
            if (err) {
                reject(err);
            } else {
                resolve(result);
            }
        }, ...args);
    });
}
