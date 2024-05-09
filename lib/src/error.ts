export enum ErrorType {
    InternalError,
    DevnetError
}

export interface Error {
    type: ErrorType;
    engine: string;
    message: string;
    backtrace?: string;
}
