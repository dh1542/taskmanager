declare module '@/types/metaInfo' {
    export type MetaInfo = import('@/types/metaInfo').MetaInfo;
}

export type MetaInfo = {
    name: string,
    value_i: number,
    value_s: string
}