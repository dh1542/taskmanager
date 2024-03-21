
import { ref } from 'vue';

import type { MetaInfo } from '@/types/metaInfo';


export async function useSystemMetaInfo(serverUrl: string): Promise<MetaInfo[]> {
    const metaInfo: MetaInfo[] = await getSystemMetaInfo(serverUrl);
    console.log(metaInfo)
    return metaInfo;
}







async function getSystemMetaInfo(serverUrl: string) {
    const response = await fetch(serverUrl.toString()).then(
        response => {
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`)
            }

            return response.json()
        }
    );

    return response
}
