

export type FileRefs = {
    icon?:string,
    thumbnail?:string,
    download?:string
}


export type PrintFile = {
    refs?: FileRefs,
    name?: string,
    display_name?: string,
    path?: string,
    size: number,
    m_timestamp: number,    
}




export type PrintJob ={
    id: number,        
    state?: string,
    progress: number,
    time_remaining: number,        
    time_printing: number,
    file?: PrintFile ,
}