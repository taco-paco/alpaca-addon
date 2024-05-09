import { Error } from './error';

export type ResolverCallback<T> = (error: Error, result: T) => void;
export function createPromise<T, A extends unknown[]>(fn: (callback: ResolverCallback<T>, ...args: A) => void, ...args: A): Promise<T> {
    return new Promise<T>((resolve, reject) => {
        // TODO: apply 'this' by default?
        fn(function (err: unknown, result: T) {
            if (err) {
                reject(err);
            } else {
                resolve(result);
            }
        }, ...args);
    });
}
