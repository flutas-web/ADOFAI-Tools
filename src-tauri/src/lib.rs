use std::{fs, io};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use encoding_rs::{Encoding, UTF_8};

#[derive(Debug, Serialize, Deserialize)]
struct FileOrDir {
    name: String,
    path: String,
    is_directory: bool,
    children: Vec<FileOrDir>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    is_file: bool,
    is_dir: bool,
    len: u64,
}

#[tauri::command]
/// 递归地列出目录下的所有文件和子目录，并返回一个包含文件和目录信息的向量
fn list_files_and_directories_internal<P: AsRef<Path>>(dir: P) -> io::Result<FileOrDir> {
    let path = dir.as_ref();
    let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let is_directory = path.is_dir();
    let mut children = Vec::new();
    if is_directory {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            children.push(list_files_and_directories_internal(entry_path)?);
        }
    }
    Ok(FileOrDir {
        name,
        path: path.to_string_lossy().into_owned(),
        is_directory,
        children,
    })
}

#[tauri::command]
fn list_files_and_directories(dir_path: &str) -> Result<FileOrDir, String> {
    list_files_and_directories_internal(dir_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_file_content(file_path: &str, encoding_name: Option<&str>) -> Result<String, String> {
    let encoding = match encoding_name {
        Some(name) => Encoding::for_label(name.as_bytes()).unwrap_or(UTF_8),
        None => UTF_8,
    };
    let bytes = fs::read(file_path).map_err(|e| e.to_string())?;
    let (result, _, had_errors) = encoding.decode(&bytes);
    if had_errors {
        Err(format!("Decoding file content with {} encoding failed", encoding.name()))
    } else {
        Ok(result.into_owned())
    }
}

#[tauri::command]
fn write_file_content(file_path: &str, content: &str, encoding_name: Option<&str>) -> Result<(), String> {
    let encoding = match encoding_name {
        Some(name) => Encoding::for_label(name.as_bytes()).unwrap_or(UTF_8),
        None => UTF_8,
    };
    let (encoded, _, had_errors) = encoding.encode(content);
    if had_errors {
        Err(format!("Encoding file content with {} encoding failed", encoding.name()))
    } else {
        fs::write(file_path, &encoded).map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn create_directory(dir_path: &str) -> Result<(), String> {
    fs::create_dir_all(dir_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_file(file_path: &str) -> Result<(), String> {
    fs::remove_file(file_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_directory(dir_path: &str) -> Result<(), String> {
    fs::remove_dir_all(dir_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn resolve_path(path: &str) -> Result<String, String> {
    fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn join_paths(base_path: &str, paths: Vec<&str>) -> Result<String, String> {
    let mut path_buf = PathBuf::from(base_path);
    for p in paths {
        path_buf.push(p);
    }
    Ok(path_buf.to_string_lossy().into_owned())
}

#[tauri::command]
fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[tauri::command]
fn get_file_info(path: &str) -> Result<FileInfo, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    Ok(FileInfo {
        is_file: metadata.is_file(),
        is_dir: metadata.is_dir(),
        len: metadata.len(),
    })
}

#[tauri::command]
fn rename_path(old_path: &str, new_path: &str) -> Result<(), String> {
    fs::rename(old_path, new_path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_files_and_directories,
            read_file_content,
            write_file_content,
            create_directory,
            delete_file,
            delete_directory,
            resolve_path,
            join_paths,
            path_exists,
            get_file_info,
            rename_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}