
import { ref } from 'vue';

import type { MetaInfo } from '@/types/metaInfo';


export async function useSystemMetaInfo(serverUrl: string) {
    const metaInfo: MetaInfo[] = await getSystemMetaInfo(serverUrl);
    console.log(metaInfo);
}



async function getSystemMetaInfo(serverUrl: string) {
    const response = await fetch(serverUrl.toString()).then(
        response => {
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return response.json(); // Parse the response as JSON
        }
    );

    return response// Return the parsed JSON object
}
