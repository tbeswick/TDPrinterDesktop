


export type VersionInfo = {
    api?: string,
    server?: string,
    nozzle_diameter?: number,
    text?: string,
    hostname?: string,
    firmware?: string,
    printer?: string,
    capabilities?: {
        "upload-by-put": boolean
    }
}