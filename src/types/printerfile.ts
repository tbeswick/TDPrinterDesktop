

export interface FileItem {
    name: string;
    display_name: string;
    type: string;
    m_timestamp: number;
}


export interface FileList {
    children?: FileItem[];
}


