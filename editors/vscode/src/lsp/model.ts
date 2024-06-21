// Common types for communication between the server and client

export interface ContentInfo {
    contentUri: string,
    contentKind: ContentKind,
    contentName: string,
    scriptsRootUri: string,
    isInWorkspace: boolean,
    isInRepository: boolean
}

export enum ContentKind {
    Raw = 0,
    WideProject = 1,
    RedkitProject = 2
}