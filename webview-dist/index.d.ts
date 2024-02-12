export declare function getItem(key: string): Promise<string | null>;
export declare function setItem(key: string, value: string): Promise<void>;
export declare function removeItem(key: string): Promise<void>;
export declare const ZustandStorage: {
    getItem: typeof getItem;
    setItem: typeof setItem;
    removeItem: typeof removeItem;
};
