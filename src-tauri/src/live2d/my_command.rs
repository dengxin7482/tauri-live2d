use crate::config;
use crate::live2d::mstruct::{AppDataConfig, InitType, Rt};
use notify::{RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::{fs, io::Read, path::PathBuf};

#[tauri::command]
pub fn read_file(file_path: std::path::PathBuf) -> Rt<Vec<u8>> {
    std::fs::read(file_path).map_or_else(
        |err| Rt {
            data: vec![],
            err: err.to_string(),
        },
        |data| Rt {
            data,
            err: String::from(""),
        },
    )
}

#[tauri::command]
pub fn write_file(file_path: std::path::PathBuf, data: &str) -> Rt<String> {
    std::fs::write(file_path, data).map_or_else(
        |err| Rt {
            data: "".to_string(),
            err: err.to_string(),
        },
        |_| Rt {
            data: "".to_string(),
            err: "".to_string(),
        },
    )
}

#[tauri::command]
pub fn init_app_data_path(file_path: std::path::PathBuf) -> InitType {
    println!("file_path: {:?}", &file_path);
    if file_path.exists() {
        InitType::EXIST
    } else {
        fs::DirBuilder::new()
            .recursive(true)
            .create(file_path)
            .map_or_else(|_| InitType::CreateError, |_| InitType::SUCCESS)
    }
}

static mut HAS_WATCH: bool = true;
//页面加载添加文件监听器
pub fn watch_jy_file_change<F>(app_data_path: PathBuf, f: F)
where
    F: Fn(),
    F: Send + 'static,
{
    unsafe {
        if HAS_WATCH == true {
            return;
        }
        HAS_WATCH = true;
        println!("HAS_WATCH on true");
    }
    println!("serve start...");
    std::thread::spawn(move || {
        live2d_server::start_serve("/Users/apple/Desktop/models");
    });

    // std::thread::spawn(move || loop {
    //     let data_path = app_data_path.join(config::APP_CONFIG_FILE);
    //     if !data_path.exists() {
    //         std::thread::sleep(core::time::Duration::from_secs(1));
    //         continue;
    //     }
    //     let app_config_str = std::fs::read_to_string(data_path).unwrap();
    //     let adc: AppDataConfig = serde_json::from_str(&app_config_str).unwrap();
    //     let (tx, rx) = channel();
    //     let mut watcher = notify::recommended_watcher(tx).unwrap();
    //     watcher
    //         .watch(&PathBuf::from(&adc.root_path), RecursiveMode::NonRecursive)
    //         .unwrap();
    //     loop {
    //         match rx.recv() {
    //             Ok(_) => {
    //                 f();
    //             }
    //             Err(e) => println!("Error: {:?}", e),
    //         }
    //     }
    // });
}
