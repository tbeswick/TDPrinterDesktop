

export interface FileItem {
    name: string;
    display_name: string;
    file_type: string;
    last_modified_timestamp: number;
}


export interface FileList {
    children?: FileItem[];
}