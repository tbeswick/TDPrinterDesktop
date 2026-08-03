

export interface FileItem {
    name: string;
    display_name: string;
    type: string;
    m_timestamp: number;
    image?: number[]; // Optional property for the image bytes
    thumbnail_path?: string; // Optional property for the thumbnail path
}


export interface FileList {
    children?: FileItem[];
}


export interface ThumbnailEvent {
    path: string;
    image: number[];
}

export function imageBytesToUrl(bytes: number[]) {

    const blob = new Blob(
        [new Uint8Array(bytes)],
        { type: "image/png" }
    );

    return URL.createObjectURL(blob);
}