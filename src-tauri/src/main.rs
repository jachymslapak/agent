#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;
use std::process::exit;
use std::{thread, time, env, fs};
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use std::cmp::min;
use std::io::Write;
use std::io::Cursor;
use std::str;
use std::process::Command;

use rodio::{Decoder, OutputStream, source::Source};
use serde::{Deserialize, Serialize};
use tar::Archive;
use reqwest::Client;
use futures_util::StreamExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static AGENT_VERSION_X: u8 = 1;
static AGENT_VERSION_Y: u8 = 0;
static AGENT_VERSION_Z: u8 = 0;

static mut PROGRESS: i32 = -100;
static mut CONNECTED: bool = true;
static mut MUTED: bool = false;
static mut DARKNESS: bool = true;

static mut UPDATE_NETR: bool = false;
static mut UPDATE_LITLCOW: bool = true;

//INITIALIZATION SCRIPTS
#[tauri::command]
async fn get_updates() -> String {
  let message: String;
  let mut connected: bool = true;

  println!("early init start");
  let doc_folder = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files"),
    None => panic!("failed to get home folder"),
  };
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
    None => panic!("failed to get home folder"),
  };
  let e_i = download_json(doc_folder, json_path).await;
  match e_i {
    Ok(file) => {unsafe {PROGRESS = -10}; file},
    Err(_) => {
      unsafe {
        PROGRESS = -100;
      }; 
      connected = false;
      unsafe {CONNECTED = false};
    },
  }


  if connected {
    unsafe {CONNECTED = true};
    println!("file downloaded");

  
    let json_path = match dirs::config_dir() {
      Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
      None => panic!("failed to get home folder"),
    };
    
    //CREATES SETTINGS FILE
    create_settings_file().expect("failed to create settings file");
    
    //INITIALIZING VERSION AND SETTINGS VARIABLES
    let buf = fs::read(json_path).expect("failed to read json file");
    let buf_string: &str = str::from_utf8(&buf).unwrap();
    let file: FileStruct = serde_json::from_str(buf_string).unwrap();


    //CONVERTING VERSIONS
    #[derive(Deserialize, Serialize)]
    struct FileStruct {
      netrv_x: u8,
      netrv_y: u8,
      netrv_z: u8,
  
      lcowv_x: u8,
      lcowv_y: u8,
      lcowv_z: u8,
  
      agentv_x: u8,
      agentv_y: u8,
      agentv_z: u8,
  
      killswitch: bool,
      msg: bool,
      msg_header: String,
      msg_text: String
    }

    let netr_version_number: u16;
    let litlcow_version_number: u16;
    let version_table: String = "xyz".to_string();

    {
      let netr_version_x = version_table.replace("x", format!("{}", file.netrv_x).as_str());
      let netr_version_xy = netr_version_x.replace("y", format!("{}", file.netrv_y).as_str());
      let netr_version_xyz = netr_version_xy.replace("z", format!("{}", file.netrv_z).as_str());
      
      let litlcow_version_x = version_table.replace("x", format!("{}", file.lcowv_x).as_str());
      let litlcow_version_xy = litlcow_version_x.replace("y", format!("{}", file.lcowv_y).as_str());
      let litlcow_version_xyz = litlcow_version_xy.replace("z", format!("{}", file.lcowv_z).as_str());
      
      netr_version_number = netr_version_xyz.parse::<u16>().unwrap();
      litlcow_version_number = litlcow_version_xyz.parse::<u16>().unwrap();
    }

    #[derive(Deserialize, Serialize)]
    struct SettingsFileStruct {
      installed_netrv_x: u8,
      installed_netrv_y: u8,
      installed_netrv_z: u8,
  
      installed_lcowv_x: u8,
      installed_lcowv_y: u8,
      installed_lcowv_z: u8,
  
      installed_agentv_x: u8,
      installed_agentv_y: u8,
      installed_agentv_z: u8,
  
      mute: bool,
      darkness: bool
    }

    let json_path = match dirs::config_dir() {
      Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
      None => panic!("failed to get home folder"),
    };

    let installed_netr_version_number: u16;
    let installed_lcow_version_number: u16;

    {
      let buf = fs::read(json_path).expect("error with reading agent.json");
      let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
      let structure: SettingsFileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

      let installed_netr_version_x = version_table.replace("x", format!("{}", structure.installed_netrv_x).as_str());
      let installed_netr_version_xy = installed_netr_version_x.replace("y", format!("{}", structure.installed_netrv_y).as_str());
      let installed_netr_version_xyz = installed_netr_version_xy.replace("z", format!("{}", structure.installed_netrv_z).as_str());

      let installed_litlcow_version_x = version_table.replace("x", format!("{}", structure.installed_netrv_x).as_str());
      let installed_litlcow_version_xy = installed_litlcow_version_x.replace("y", format!("{}", structure.installed_netrv_y).as_str());
      let installed_litlcow_version_xyz = installed_litlcow_version_xy.replace("z", format!("{}", structure.installed_netrv_z).as_str());

      installed_netr_version_number = installed_netr_version_xyz.parse::<u16>().unwrap();
      installed_lcow_version_number = installed_litlcow_version_xyz.parse::<u16>().unwrap();
    }

    // COMPARING VERSIONS
    if installed_netr_version_number == netr_version_number {
      unsafe {UPDATE_NETR = false}
      println!("netr: same version");
      println!("{}:{}", installed_lcow_version_number, netr_version_number);
      println!("");
    } else if installed_netr_version_number < netr_version_number {
      unsafe {UPDATE_NETR = true}
      println!("netr: update available");
      println!("{}:{}", installed_lcow_version_number, netr_version_number);
      println!("");
    } else if installed_netr_version_number > netr_version_number {
      unsafe {UPDATE_NETR = false}
      println!("netr: downgrade");
      println!("{}:{}", installed_lcow_version_number, netr_version_number);
      println!("");
    } else {
      unsafe {UPDATE_NETR = false}
      println!("netr: version compare error");
      println!("{}:{}", installed_lcow_version_number, netr_version_number);
      println!("");
    }

    if installed_lcow_version_number == litlcow_version_number {
      unsafe {UPDATE_LITLCOW = false}
      println!("litlcow: same version")
    } else if installed_lcow_version_number < litlcow_version_number {
      unsafe {UPDATE_LITLCOW = true}
      println!("litlcow: update available")
    } else if installed_lcow_version_number > litlcow_version_number {
      unsafe {UPDATE_LITLCOW = false}
      println!("litlcow: downgrade")
    } else {
      unsafe {UPDATE_LITLCOW = false}
      println!("litlcow: version compare error") 
    }

    println!("{}.{}.{}", file.netrv_x, file.netrv_y, file.netrv_z);
    println!("{}.{}.{}", file.lcowv_x, file.lcowv_y, file.lcowv_z);
    println!("{}.{}.{}", file.agentv_x, file.agentv_y, file.agentv_z);
    
    if file.killswitch == true {exit(0)};
    message = format!("done");

  } else {
      message = format!("!download");
  }

  message.into()
}


async fn download_json(doc_folder: PathBuf, json_path: PathBuf) -> Result<()> {
  fs::create_dir_all(doc_folder)?;

  let response = reqwest::get("https://get.enzete.com/agent.json").await?;

  let mut file = std::fs::File::create(json_path)?;
  let mut content =  Cursor::new(response.bytes().await?);

  std::io::copy(&mut content, &mut file)?;

  Ok(())
}


fn create_settings_file() -> Result<()> {
  let settings_file_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };
  let settings_file_path_2 = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }  
  
  if settings_file_path.exists() {
    let buf = fs::read(settings_file_path)?;

    let buf_string: &str = str::from_utf8(&buf)?;
    let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;

    changed_file.installed_agentv_x = AGENT_VERSION_X;
    changed_file.installed_agentv_y = AGENT_VERSION_Y;
    changed_file.installed_agentv_z = AGENT_VERSION_Z;

    let changed_file_str = serde_json::to_string_pretty(&changed_file)?;

    let mut file = fs::File::create(settings_file_path_2)?;
    file.write_all(changed_file_str.as_bytes())?;
  
    Ok(())
  } else {
    let buf = br#"{
      "installed_netrv_x": 0,
      "installed_netrv_y": 0,
      "installed_netrv_z": 0,

      "installed_lcowv_x": 0,
      "installed_lcowv_y": 0,
      "installed_lcowv_z": 0,

      "installed_agentv_x": 0,
      "installed_agentv_y": 0,
      "installed_agentv_z": 0,

      "mute": false,
      "darkness": true
    }"#;
    
    let buf_string: &str = str::from_utf8(buf)?;
    let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;
    
    changed_file.installed_agentv_x = AGENT_VERSION_X;
    changed_file.installed_agentv_y = AGENT_VERSION_Y;
    changed_file.installed_agentv_z = AGENT_VERSION_Z;

    let changed_file_str = serde_json::to_string_pretty(&changed_file)?;

    let mut file = fs::File::create(settings_file_path)?;
    file.write_all(changed_file_str.as_bytes())?;

    Ok(())
  }
}


#[tauri::command]
fn get_msg_status() -> bool {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    netrv_x: u8,
    netrv_y: u8,
    netrv_z: u8,

    lcowv_x: u8,
    lcowv_y: u8,
    lcowv_z: u8,

    agentv_x: u8,
    agentv_y: u8,
    agentv_z: u8,

    killswitch: bool,
    msg: bool,
    msg_header: String,
    msg_text: String
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let file: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  file.msg
}


#[tauri::command]
fn get_msg_header() -> String {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    netrv_x: u8,
    netrv_y: u8,
    netrv_z: u8,

    lcowv_x: u8,
    lcowv_y: u8,
    lcowv_z: u8,

    agentv_x: u8,
    agentv_y: u8,
    agentv_z: u8,

    killswitch: bool,
    msg: bool,
    msg_header: String,
    msg_text: String
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let file: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  file.msg_header
}


#[tauri::command]
fn get_msg_text() -> String {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    netrv_x: u8,
    netrv_y: u8,
    netrv_z: u8,

    lcowv_x: u8,
    lcowv_y: u8,
    lcowv_z: u8,

    agentv_x: u8,
    agentv_y: u8,
    agentv_z: u8,

    killswitch: bool,
    msg: bool,
    msg_header: String,
    msg_text: String
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let file: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  file.msg_text
}


//SETTINGS SCRIPTS
#[tauri::command]
fn set_theme_init(value: bool) {
  println!("setting theme");
  set_theme(value).expect("error");
}


#[tauri::command]
fn switch_mute_init() {
  switch_mute().expect("error");
}


fn set_theme(value: bool) -> Result<()> {
  let settings_file_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  let settings_file_path_2 = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(settings_file_path)?;

  let buf_string: &str = str::from_utf8(&buf)?;
  let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;

  changed_file.darkness = value;
  let changed_file_str = serde_json::to_string_pretty(&changed_file)?;

  let mut file = fs::File::create(settings_file_path_2)?;
  file.write_all(changed_file_str.as_bytes())?;
  
  Ok(())
}


fn switch_mute() -> Result<()> {
  let settings_file_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  let settings_file_path_2 = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(settings_file_path)?;

  let buf_string: &str = str::from_utf8(&buf)?;
  let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;

  changed_file.mute = !changed_file.mute;
  let changed_file_str = serde_json::to_string_pretty(&changed_file)?;

  let mut file = fs::File::create(settings_file_path_2)?;
  file.write_all(changed_file_str.as_bytes())?;
  
  Ok(())
}


//GETTING SCRIPTS
//settings
#[tauri::command]
fn get_settings() -> String {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let structure: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");
  
  unsafe {
    MUTED = structure.mute;
    DARKNESS = structure.darkness;
  }

  let message: &str;

  unsafe {
    if MUTED && DARKNESS {
      message = "m1d1";
    }
    else if !MUTED && !DARKNESS {
      message = "m0d0";
    }
    else if !MUTED && DARKNESS {
      message = "m0d1";
    }
    else if MUTED && !DARKNESS {
      message = "m1d0";
    } else {
        message = "error";
    }
  }

  println!("{}", message);
  message.to_string().into()
}


#[tauri::command]
fn get_mute() -> bool {
  println!("getting mute state");
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let structure: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  structure.darkness
}


//games
#[tauri::command]
fn get_netr_update_state() -> bool {
  unsafe {UPDATE_NETR}
}


#[tauri::command]
fn get_litlcow_update_state() -> bool {
  unsafe {UPDATE_LITLCOW}
}


fn get_installed_game_versions() -> Result<(u8, u8, u8, u8, u8, u8)> {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }  

  let buf = fs::read(json_path)?;
  let buf_str = str::from_utf8(&buf)?;
  let file: FileStruct = serde_json::from_str(buf_str)?;


  Ok((file.installed_netrv_x, file.installed_netrv_y, file.installed_netrv_z, file.installed_lcowv_x, file.installed_lcowv_y, file.installed_lcowv_z))
}


fn get_installed_game_versions_strings() -> Result<(String, String)> {
  let numbers = get_installed_game_versions()?;

  let netr_version_string = format!("{}.{}.{}", numbers.0, numbers.1, numbers.2); 
  let lcow_version_string = format!("{}.{}.{}", numbers.3, numbers.4, numbers.5); 

  Ok((netr_version_string, lcow_version_string))
}


#[tauri::command]
fn get_netr_version_string() -> String {
  let versions = match get_installed_game_versions_strings() {
    Ok(result) => result,
    Err(_) => return format!("no version")
  };
  let netr_version = versions.0;
  println!("getting netr version: {}", netr_version);
  netr_version
}


#[tauri::command]
fn get_lcow_version_string() -> String {
  let versions = match get_installed_game_versions_strings() {
    Ok(result) => result,
    Err(_) => return format!("none")
  };
  let lcow_version = versions.1;
  println!("getting litlcow version: {}", lcow_version);
  lcow_version
}


fn reset_netr_version() -> Result<()> {
  let settings_file_path = match dirs::document_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };
  let settings_file_path_2 = match dirs::document_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(settings_file_path)?;

  let buf_string: &str = str::from_utf8(&buf)?;
  let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;

  changed_file.installed_netrv_x = 0;
  changed_file.installed_netrv_y = 0;
  changed_file.installed_netrv_z = 0;

  let changed_file_str = serde_json::to_string_pretty(&changed_file)?;

  let mut file = fs::File::create(settings_file_path_2)?;
  file.write_all(changed_file_str.as_bytes())?;
  
  Ok(())
}


fn reset_lcow_version() -> Result<()> {
  let settings_file_path = match dirs::document_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };
  let settings_file_path_2 = match dirs::document_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(settings_file_path)?;

  let buf_string: &str = str::from_utf8(&buf)?;
  let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;

  changed_file.installed_lcowv_x = 0;
  changed_file.installed_lcowv_y = 0;
  changed_file.installed_lcowv_z = 0;

  let changed_file_str = serde_json::to_string_pretty(&changed_file)?;

  let mut file = fs::File::create(settings_file_path_2)?;
  file.write_all(changed_file_str.as_bytes())?;
  
  Ok(())
}


//system
#[tauri::command]
fn get_os() -> String {
  let os_type = env::consts::OS;
  format!("{}", os_type)
}


//INSTALLATION SCRIPTS
#[tauri::command]
async fn install_init(game_name: String) { 
  let netr_version = get_download_versions().0;
  let lcow_version = get_download_versions().1;

  println!("netr version: {}, lcow version: {}", netr_version, lcow_version);

  println!("downloading game: {}", game_name);
  let game_title = game_name.as_str();
  
  let ostype: &str = env::consts::OS;
  let url: String;
  let ending: &str;
  
  if game_title == "netr" {
    if ostype == "macos" {
      url = format!("https://get.enzete.com/game_files/netr/{}/netr.app.tar.xz", netr_version);
      ending = "app";
    }     
    else if ostype == "windows" {
      url = format!("https://get.enzete.com/game_files/netr/{}/netr.exe.tar.xz", netr_version);
      ending = "exe";
    }  
    else if ostype == "linux" {
      url = format!("https://get.enzete.com/game_files/netr/{}/netr.x86_64.tar.xz", netr_version);
      ending = "x86_64";
    } else {
      panic!("failed to detect your os")
    }
  } else if game_title == "litlcow" {
    if ostype == "macos" {
      url = format!("https://get.enzete.com/game_files/litlcow/{}/litlcow.app.tar.xz", lcow_version);
      ending = "app";
    }     
    else if ostype == "windows" {
      url = format!("https://get.enzete.com/game_files/litlcow/{}/litlcow.exe.tar.xz", lcow_version);
      ending = "exe";
    }  
    else if ostype == "linux" {
      url = format!("https://get.enzete.com/game_files/litlcow/{}/litlcow.x86_64.tar.xz", lcow_version);
      ending = "x86_64";
    } else {
      panic!("failed to detect your os")
    }
  } else {
    unsafe {PROGRESS = -100;};
    panic!("failed to start download");
  }
  let home_dir = env::temp_dir();
  
  println!("{}", home_dir.display());
  
  //DIRECTORIES
  let downloads_folder = home_dir.join("enzete-agent").join("downloads");
  let extract_folder = home_dir.join("enzete-agent").join("downloads").join(game_title);
  let path = home_dir.join("enzete-agent").join("downloads").join(format!("{game_title}.tar.xz"));
  let path_to_extract = home_dir.join("enzete-agent").join("downloads").join(game_title).join(format!("{game_title}.tar"));
  let path_final = home_dir.join("enzete-agent").join("downloads").join(game_title).join(format!("{game_title}.{ending}"));
  
  let client = reqwest::Client::new();

  {
    let work1 = download_file(&client, url, &path, &downloads_folder).await;
    match work1 {
      Ok(file) => file,
      Err(err) => {unsafe {PROGRESS = -100;}; panic!("{} failed download", err);},
    }
  }
  {
    let work2 = extract_file(&extract_folder, &path, &path_to_extract).await;
    match work2 {
      Ok(file) => file,
      Err(err) => {unsafe {PROGRESS = -100;}; panic!("{} failed extracting", err);},
    }
  }
  {
    let work3 = install_file(&path_final, &game_title).await;
    match work3 {
      Ok(file) => {unsafe {PROGRESS = -10}; file},
      Err(err) => {unsafe {PROGRESS = -100;}; panic!("{} failed instalation", err);},
    }
  }
  //add cleaning!
}


fn get_download_versions() -> (String, String) {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    netrv_x: u8,
    netrv_y: u8,
    netrv_z: u8,

    lcowv_x: u8,
    lcowv_y: u8,
    lcowv_z: u8,

    agentv_x: u8,
    agentv_y: u8,
    agentv_z: u8,

    killswitch: bool,
    msg: bool,
    msg_header: String,
    msg_text: String
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let file: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  let netr_version_string = format!("{}.{}.{}", file.netrv_x, file.netrv_y, file.netrv_z); 
  let lcow_version_string = format!("{}.{}.{}", file.lcowv_x, file.lcowv_y, file.lcowv_z); 


  (netr_version_string, lcow_version_string).into()
}


fn get_download_versions_as_numbers() -> (u8, u8, u8, u8, u8, u8) {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    netrv_x: u8,
    netrv_y: u8,
    netrv_z: u8,

    lcowv_x: u8,
    lcowv_y: u8,
    lcowv_z: u8,

    agentv_x: u8,
    agentv_y: u8,
    agentv_z: u8,

    killswitch: bool,
    msg: bool,
    msg_header: String,
    msg_text: String
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let file: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  (file.netrv_x, file.netrv_y, file.netrv_z, file.lcowv_x, file.lcowv_y, file.lcowv_z).into()
}


async fn download_file(client: &Client, url: String, path: &PathBuf, downloads_folder: &PathBuf) -> Result<()> {
  unsafe {
    PROGRESS = -1;
  }
  
  println!("downloading");
  std::fs::create_dir_all(downloads_folder)?;
  
  let res = client
  .get(url.as_str())
  .send()
  .await.or(Err("GET request fail"))?;

let total_size = res
.content_length()
.ok_or("failed to get file length")?;

let total_size128: u128 = total_size as u128;
println!("total size {}", total_size);

let mut file = File::create(path).or(Err("failed to create file"))?;
let mut downloaded: u128 = 0;

let mut stream = res.bytes_stream();
let mut remaining_mb_old: i32 = 0;

while let Some(item) = stream.next().await {
  let chunk = item.or(Err(format!("fail while downloading file"))).unwrap();
  file.write_all(&chunk)
  .or(Err("fail while writing to file"))?;
let new = min(downloaded + (chunk.len() as u128), total_size128);
downloaded = new;
let remaining_mb = ((total_size128 - downloaded)/1048576) as i32;
if remaining_mb != remaining_mb_old {
  println!("{}", remaining_mb);
  remaining_mb_old = remaining_mb;
  unsafe {
    PROGRESS = remaining_mb;
  }
}
}

Ok(())
}


async fn install_file(path_final: &PathBuf, game_title: &str) -> Result<()> {
  unsafe {
    PROGRESS = -3;
  };
  let os_type: &str = env::consts::OS;

  let settings_file_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  let settings_file_path_2 = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  
  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,
    
    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,
    
    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,
    
    mute: bool,
    darkness: bool
  }
  
  let buf = fs::read(settings_file_path)?;

  let buf_string: &str = str::from_utf8(&buf)?;
  let mut changed_file: FileStruct = serde_json::from_str(buf_string)?;

  let versions = get_download_versions_as_numbers();

  if game_title == "netr" {
    if os_type == "macos" {
      let netr_path_mac = match dirs::home_dir() {
        Some(var) => var.join("Applications").join("netr.app"),
        None => panic!("failed to get home folder"),
      };

      println!("installing netr macOS...");
      std::fs::rename(path_final, netr_path_mac)?;
      
    } else if os_type == "linux" {
      let linux_folder = match dirs::executable_dir() {
        Some(var) => var,
        None => panic!("failed to get home folder"),
      };
      let netr_path_folder = match dirs::executable_dir() {
        Some(var) => var.join("netr.x86_64"),
        None => panic!("failed to get home folder"),
      };
      let exec = match dirs::executable_dir() {
        Some(var) => var.join("netr.x86_64"),
        None => panic!("failed to get home folder"),
      };

      println!("installing netr linux...");
      println!("{:?}", path_final);
      println!("{:?}", netr_path_folder);
      
      std::fs::create_dir_all(linux_folder)?;
      std::fs::copy(path_final, netr_path_folder).expect("error with copying");

      let username = whoami::username();
      let desktop_entry_path = format!("/home/{}/.local/share/applications/netr.desktop", username);
      println!("{}", desktop_entry_path);

      let mut file = fs::File::create(desktop_entry_path)?;
      let buf = format!("[Desktop Entry]
      Version=1.0
      Name=netr
      GenericName=netr
      Exec={}
      Terminal=false
      Type=Application", exec.display());
      file.write_all(buf.as_bytes()).expect("error with writing file");

      let output = Command::new("chmod").arg("+x").arg(exec).output().expect("failed to execute process");
      let hello = output.stdout;

      println!("{:?}", hello)

    } else if os_type == "windows" {
      let netr_path_win = match dirs::desktop_dir() {
        Some(var) => var.join("netr.exe"),
        None => panic!("failed to get home folder"),
      };

      println!("installing netr windows...");
      std::fs::rename(path_final, netr_path_win)?;

    } else {
        panic!("error with getting os")
    }

    let netrv_x = versions.0;
    let netrv_y = versions.1;
    let netrv_z = versions.2;

    changed_file.installed_netrv_x = netrv_x;
    changed_file.installed_netrv_y = netrv_y;
    changed_file.installed_netrv_z = netrv_z;

    let changed_file_str = serde_json::to_string_pretty(&changed_file)?;
  
    let mut file = fs::File::create(settings_file_path_2)?;
    println!("writing to settings netr: {}.{}.{}", netrv_x, netrv_x, netrv_z);
    file.write_all(changed_file_str.as_bytes())?;



  } else if game_title == "litlcow" {
    //mac only
    if os_type == "macos" {
      let lcow_path_mac = match dirs::home_dir() {
        Some(var) => var.join("Applications").join("litlcow.app"),
        None => panic!("failed to get home folder"),
      };

      std::fs::rename(path_final, lcow_path_mac)?;
      
    } else if os_type == "windows" {
      let lcow_path_win = match dirs::desktop_dir() {
        Some(var) => var.join("litlcow.exe"),
        None => panic!("failed to get home folder"),
      };

      std::fs::rename(path_final, lcow_path_win)?;
      
    } else if os_type == "linux" {
      let litlcow_path_linux = match dirs::executable_dir() {
        Some(var) => var.join("litlcow.x86_64"),
        None => panic!("failed to get home folder"),
      };
      let linux_folder = match dirs::executable_dir() {
        Some(var) => var,
        None => panic!("failed to get home folder"),
      };

      println!("installing litlcow linux...");

      std::fs::create_dir_all(linux_folder)?;
      std::fs::copy(path_final, litlcow_path_linux)?;

    } else {
      panic!("usupported OS");
    }

    let lcowv_x = versions.3;
    let lcowv_y = versions.4;
    let lcowv_z = versions.5;

    changed_file.installed_lcowv_x = lcowv_x;
    changed_file.installed_lcowv_y = lcowv_y;
    changed_file.installed_lcowv_z = lcowv_z;

    let changed_file_str = serde_json::to_string_pretty(&changed_file)?;
  
    let mut file = fs::File::create(settings_file_path_2)?;
    println!("writing to settings lcow: {}.{}.{}", lcowv_x, lcowv_y, lcowv_z);
    file.write_all(changed_file_str.as_bytes())?;
  }
  
  Ok(())
}


async fn extract_file(extract_folder: &PathBuf, path: &PathBuf, path_to_extract: &PathBuf) -> Result<()> {
  unsafe {
    PROGRESS = -2;
  }
  
  println!("decompressing");
  std::fs::create_dir_all(extract_folder)?;
  
  let mut file_to_extract = std::fs::read(path)?;
  let decompressed = lzma::decompress(&mut file_to_extract)?;
  std::fs::write(path_to_extract, decompressed)?;
  
  let mut ar = Archive::new(File::open(path_to_extract)?);
  ar.unpack(extract_folder)?;
  
  Ok(())
}


//UNINSTALLATION SCRIPTS
#[tauri::command]
async fn delete_init(game_name: String) -> String {
  println!("deleting {}", game_name);
  let os_type = env::consts::OS;
  if os_type == "macos" {
    format!("mak").into()
  } else {
    println!("deleting");
    
    let delete = delete(game_name).await;
    
    match delete {
      Ok(file) => {unsafe {PROGRESS = -10}; file},
      Err(err) => {
        unsafe {PROGRESS = -100;}; println!("{}", err); panic!("failed deleting");
      },
    }
    println!("deleting done");
    format!("d").into()
  }
}


async fn delete(game_title: String) -> Result<()> {
  let os_type = env::consts::OS;
  if os_type == "macos" {unsafe{PROGRESS = -5}}
  
  else {
    unsafe {PROGRESS = -4;}
    
    if game_title == "netr" {
      reset_netr_version().expect("error with reseting version");
      
      if os_type == "linux" {
        let netr_path_linux = match dirs::executable_dir() {
          Some(var) => var.join("netr.x86_64"),
          None => panic!("failed to get home folder"),
        };
        std::fs::remove_file(netr_path_linux)?;

      } else if os_type == "windows" {
        let netr_path_win = match dirs::desktop_dir() {
          Some(var) => var.join("netr.exe"),
          None => panic!("failed to get home folder"),
        };
        std::fs::remove_file(netr_path_win).unwrap();
      }
    }

    if game_title == "litlcow" {
      reset_lcow_version().expect("error with reseting version");

      if os_type == "linux" {
        let litlcow_path_linux = match dirs::executable_dir() {
          Some(var) => var.join("litlcow.x86_64"),
          None => panic!("failed to get home folder"),
        };

        std::fs::remove_file(litlcow_path_linux)?;
      } else if os_type == "windows" {
        let litlcow_path_win = match dirs::desktop_dir() {
          Some(var) => var.join("litlcow.exe"),
          None => panic!("failed to get home folder"),
        };
        std::fs::remove_file(litlcow_path_win)?;
      }   
    }
  }
  Ok(())
}


//INSTALLATION COMMANDS
#[tauri::command]
fn disconnected() {
  unsafe {
    CONNECTED = false;
  }
  println!("disconnected");
}


#[tauri::command]
fn is_disconnected() -> bool {
  println!("reqwested");
  unsafe {
    if CONNECTED {
      true
    } else {
      false
    }
  }
}


#[tauri::command]
async fn retry() {
  let string = get_updates().await;
  println!("{}", string);
}


#[tauri::command]
fn get_progress() -> String {
  // codes
  // -1 initiaizing
  // -2 extracting
  // -3 installing
  // -4 deleting
  // -5 mac delete 
  
  // -10 done

  let message: String;

  unsafe {
    if PROGRESS == -100 {
      message = "error".to_string();
    }
    else if PROGRESS == -1 {
      message = "...".to_string();
    }
    else if PROGRESS == -2 {
      message = "expanding".to_string();
    }
    else if PROGRESS == -3 {
      message = "installing".to_string();
    }
    else if PROGRESS == -4 {
      message = "deleting".to_string();
    }
    else if PROGRESS == -10 {
      message = "d".to_string();
    }
    else {
      message = format!("{} MB", PROGRESS.to_string())
    }
  }
  message.into()
}


#[tauri::command]
fn is_netr_installed() -> bool {
  let os_type = env::consts::OS;
  let netr_exist: bool;

  let netr_path: PathBuf;
  
  if os_type == "macos" {
    netr_path = match dirs::home_dir() {
      Some(var) => var.join("Applications").join("netr.app"),
      None => panic!("failed to get home folder"),
    };
  } else if os_type == "windows" {
    netr_path = match dirs::desktop_dir() {
      Some(var) => var.join("netr.exe"),
      None => panic!("failed to get home folder"),
    };
  } else if os_type == "linux" {
    //DODĚLAT!!
    netr_path = match dirs::executable_dir() {
      Some(var) => var.join("netr.x86_64"),
      None => panic!("failed to get home folder"),
    };
  } else {
    panic!("failed to get os type")
  }

  if netr_path.exists() {netr_exist = true} else {netr_exist = false};
  netr_exist
}


#[tauri::command]
fn is_litlcow_installed() -> bool {
  let os_type = env::consts::OS;
  let litlcow_exist: bool;

  let litlcow_path: PathBuf;
  
  if os_type == "macos" {
    litlcow_path = match dirs::home_dir() {
      Some(var) => var.join("Applications").join("litlcow.app"),
      None => panic!("failed to get home folder"),
    };
  } else if os_type == "windows" {
    litlcow_path = match dirs::desktop_dir() {
      Some(var) => var.join("litlcow.exe"),
      None => panic!("failed to get home folder"),
    };
  } else if os_type == "linux" {
    //DODĚLAT!!
    litlcow_path = match dirs::executable_dir() {
      Some(var) => var.join("litlcow.x86_64"),
      None => panic!("failed to get home folder"),
    };
  } else {
    panic!("failed to get os type")
  }

  if litlcow_path.exists() {litlcow_exist = true} else {litlcow_exist = false};
  litlcow_exist
}


// SOUND PLAYER
fn player(file: File) {
  let json_path = match dirs::config_dir() {
    Some(var) => var.join("enzete agent").join("agent files").join("agent_settings.json"),
    None => panic!("failed to get home folder"),
  };

  #[derive(Deserialize, Serialize)]
  struct FileStruct {
    installed_netrv_x: u8,
    installed_netrv_y: u8,
    installed_netrv_z: u8,

    installed_lcowv_x: u8,
    installed_lcowv_y: u8,
    installed_lcowv_z: u8,

    installed_agentv_x: u8,
    installed_agentv_y: u8,
    installed_agentv_z: u8,

    mute: bool,
    darkness: bool
  }

  let buf = fs::read(json_path).expect("error with reading agent.json");
  let buf_str = str::from_utf8(&buf).expect("failed to create string from bytes from agent.json");
  let structure: FileStruct = serde_json::from_str(buf_str).expect("failed to create structure from agent.json");

  if structure.mute == false {
    thread::sleep(time::Duration::from_millis(500));
    let (_stream, stream_handle) = OutputStream::try_default().unwrap(); 
    let buffer = BufReader::new(file);
    let source = Decoder::new(buffer).unwrap();
    stream_handle
      .play_raw(source
        .convert_samples().amplify(0.20))
      .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
  } 
}


#[tauri::command]
fn system_exit() {
  exit(0);
}


fn main() { 
  println!("{:#?}", dirs::home_dir());
    tauri::Builder::default()
    .setup(|app| {Ok({
      println!("{:?}", dirs::config_dir());
      let resource_path = app.path_resolver().resolve_resource("resources/welcome.ogg").expect("failed to resolve resource");
      let file = std::fs::File::open(resource_path).unwrap();
      thread::spawn(move || player(file));
      let username = whoami::username();
      println!("{}", username);
    })
    })
    .invoke_handler(tauri::generate_handler![
      //INSTALLATION FUNCTIONS
      install_init, 
      get_progress, 
      delete_init, 
      get_updates,
      disconnected,
      is_disconnected,
      retry,
      //SETTINGS
      set_theme_init,
      switch_mute_init,
      get_settings,
      get_mute,
      //GAME GETTERS
      get_netr_update_state,
      get_litlcow_update_state,
      is_netr_installed,
      is_litlcow_installed,
      get_netr_version_string,
      get_lcow_version_string,
      //SETTINGS GETTERS
      get_msg_status,
      get_msg_header,
      get_msg_text,
      //SYSTEM COMMANDS
      system_exit,
      get_os
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}