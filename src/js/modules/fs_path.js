const { invoke } = window.__TAURI__.core;

// 递归地列出目录下的所有文件和子目录
async function listFilesAndDirectories(dirPath) {
    return await invoke('list_files_and_directories', { dirPath });
}

// 读取文件内容
async function readFileContent(filePath, encoding = null) {
    return await invoke('read_file_content', { filePath, encodingName: encoding });
}

async function writeFileContent(filePath, content, encoding = null) {
    return await invoke('write_file_content', { filePath, content, encodingName: encoding });
}

// 创建目录
async function createDirectory(dirPath) {
    return await invoke('create_directory', { dirPath });
}

// 删除文件
async function deleteFile(filePath) {
    return await invoke('delete_file', { filePath });
}

// 删除目录
async function deleteDirectory(dirPath) {
    return await invoke('delete_directory', { dirPath });
}

// 解析路径
async function resolvePath(path) {
    return await invoke('resolve_path', { path });
}

// 拼接路径
async function joinPaths(basePath, paths) {
    return await invoke('join_paths', { basePath, paths });
}

// 检查路径是否存在
async function pathExists(path) {
    return await invoke('path_exists', { path });
}

// 获取文件信息
async function getFileInfo(path) {
    return await invoke('get_file_info', { path });
}

// 重命名路径
async function renamePath(oldPath, newPath) {
    return await invoke('rename_path', { oldPath, newPath });
}

const fs = {
    listFilesAndDirectories,
    writeFileContent,
    readFileContent,
    createDirectory,
    deleteFile,
    deleteDirectory,
    getFileInfo
}
const path = {
    resolvePath,
    joinPaths,
    pathExists,
    renamePath
}
export {
    fs,
    path
};