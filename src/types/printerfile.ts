

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

