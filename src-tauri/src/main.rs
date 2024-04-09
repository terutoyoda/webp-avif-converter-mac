//main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{command, Builder, generate_context, Window};
use tauri::api::dialog::FileDialogBuilder;
use tokio::process::Command as AsyncCommand;
use std::{io::Write, path::PathBuf};
use base64::decode;
use tempfile::NamedTempFile;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashSet;
use std::path::Path;
use std::ffi::OsStr;
use dirs;

lazy_static! {
    static ref SELECTED_OUTPUT_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref USED_FILENAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

async fn async_main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![convert_image, select_output_directory])
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn select_output_directory() {
    FileDialogBuilder::new()
        .pick_folder(move |folder_path| {
            let mut selected_dir = SELECTED_OUTPUT_DIR.lock().unwrap();
            *selected_dir = folder_path.clone();
            if let Some(path) = folder_path {
                println!("Selected folder: {}", path.display());
            } else {
                println!("No folder selected");
            }
        });
}

#[command]
async fn convert_image(
    window: Window,
    file_data: String,
    format: String,
    original_name: String,
    original_extension: String
) -> Result<String, String> {
    println!("Received format: {}", format);
    let decoded = decode(&file_data).map_err(|e| e.to_string())?;
    println!("Decoded data successfully");

    let mut temp_file = NamedTempFile::new().map_err(|e| e.to_string())?;
    temp_file.write_all(&decoded).map_err(|e| e.to_string())?;
    let temp_path = temp_file.into_temp_path();

    let output_dir = SELECTED_OUTPUT_DIR.lock().unwrap()
        .as_ref()
        .cloned()
        .unwrap_or_else(|| dirs::desktop_dir().expect("Failed to find desktop directory"));

    let original_file_stem = Path::new(&original_name)
        .file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(|| "Failed to extract file stem".to_string())?;

    // 元の拡張子を含めた出力ファイル名を生成
    let output_filename = format!("{}_{}.{}", original_file_stem, original_extension, format);
    let output_path = output_dir.join(&output_filename);

    let temp_path_buf = temp_path.to_path_buf();

    // 変換処理
    match format.as_str() {
        "webp" => convert_to_webp_async(&window, &temp_path_buf, &output_path).await,
        "avif" => convert_to_avif_async(&window, &temp_path_buf, &output_path).await,
        _ => Err("Unsupported format".to_string()),
    }?;

    println!("Conversion successful, output path: {}", output_path.display());
    Ok(output_path.to_string_lossy().into_owned())
}

async fn convert_to_webp_async(_window: &Window, input_path: &PathBuf, output_path: &PathBuf) -> Result<(), String> {
    let status = AsyncCommand::new("cwebp")
        .arg(input_path.to_str().ok_or_else(|| "Invalid input path".to_string())?)
        .arg("-o")
        .arg(output_path.to_str().ok_or_else(|| "Invalid output path".to_string())?)
        .status()
        .await
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        let err_msg = "Failed to convert to webp".to_string();
        Err(err_msg)
    }
}

async fn convert_to_avif_async(_window: &Window, input_path: &PathBuf, output_path: &PathBuf) -> Result<(), String> {
    let status = AsyncCommand::new("avifenc")
        .arg(input_path.to_str().ok_or_else(|| "Invalid input path".to_string())?)
        .arg(output_path.to_str().ok_or_else(|| "Invalid output path".to_string())?)
        .status()
        .await
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        let err_msg = "Failed to convert to AVIF".to_string();
        Err(err_msg)
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async_main());
}
