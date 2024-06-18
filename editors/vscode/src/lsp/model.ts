// Common types for communication between the server and client

export interface ContentInfo {
    contentUri: string,
    scriptsRootUri: string,
    contentName: string,
    isInWorkspace: boolean,
    isInRepository: boolean
}
