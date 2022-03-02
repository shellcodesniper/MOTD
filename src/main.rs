extern crate reqwest;
extern crate colour;
extern crate sys_info;
extern crate chrono;
extern crate terminal_size;

use chrono::prelude::*;

const TAB_WIDTH: f32 = 8.0;
// ? TAB SIZE, 8을 기준으로 맞춰진 상태라 8 권장.
const MOTD_WIDTH: f32 = 66.0;
// ? MOTD 총 길이 ( MOTD 가로 글자수만 포함하면됨 <계산결과 64> ( TAB 8 개))

use std::io::{ Read };
use sys_info::*;
use colour::{ blue_ln, blue, dark_yellow, dark_red, cyan, green, white };
use terminal_size::{Width, Height, terminal_size};

fn pad(pad: &str) {
  print!("{}", pad);
}

fn kb_to_mb(kb: u64) -> f64 {
  (kb as f64) / 1024.0
}

fn kb_to_gb(kb: u64) -> f64 {
  (kb_to_mb(kb) / 1024.0) as f64
}

fn kb_to_tb(kb: u64) -> f64 {
  (kb_to_gb(kb) / 1024.0) as f64
}

fn title(title: &str) {
  {
    blue!("||\t\t\t\t\t");
    white!("[[  {}  ]]", title);
    blue_ln!("\t\t\t\t\t||");
  }
}
fn empty_line() {
  blue_ln!("||\t\t\t\t\t\t\t\t\t\t\t\t||");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut temp_string: String = String::new();
  let mut ps: &str = "";
  let size = terminal_size();
  if let Some((Width(w), Height(_))) = size {
    let temp: f32 = (((w as f32 / TAB_WIDTH) - (MOTD_WIDTH / TAB_WIDTH)) / 4.0).ceil() * TAB_WIDTH;
    let pst = temp as u16;
    temp_string.push_str(" ".repeat(pst.into()).as_str());
    ps = temp_string.as_str();
  } else {
      println!("Unable to get terminal size");
  }

  
  let public_ip = match reqwest::blocking::get("https://api.bdev.io/ip") {
    Ok(mut res) => {
      let mut body = String::new();
      let status = res.read_to_string(&mut body).is_ok();
      if status == false {
        body.push_str("ERROR");
      }

      let split = body.split("data\":").clone();
      //let vec = split.collect::<Vec<&str>>();
      let vec: Vec<&str> = split.collect();
      if vec.len() > 0 {
        String::from(vec[1].replace('"', "").replace('}', ""))
      } else {
        body.clone()
      }
    },
    Err(_) => String::from("ERROR"),
  };

  let boot_time = boottime().unwrap();
  let load = loadavg().unwrap();
  let mem = mem_info().unwrap();
  let disk = disk_info().unwrap();

  println!("\n\n");
 
  pad(ps);
  blue_ln!("  ==============================================================================================  ");

  pad(ps);
  empty_line();

  pad(ps);
  title("NETWORK INFO");

  pad(ps);
  empty_line();

  pad(ps);
  {
    blue!("||\t");
    dark_yellow!("공인 아이피\t:\t");
    dark_red!("{}\t\t\t\t\t\t", public_ip);
    blue_ln!("\t||");
  }

  
  pad(ps);
  {
    blue!("||\t");
    dark_yellow!("HOST NAME\t:\t");
    white!("{}\t\t\t\t\t\t", hostname().unwrap());
    blue_ln!("\t||");
  }

  pad(ps);
  empty_line();

  pad(ps);
  title("SYSTEM INFO");

  pad(ps);
  empty_line();

  pad(ps);
  {
    let timestamp = boot_time.tv_sec;

    let naive = NaiveDateTime::from_timestamp(timestamp, 0);

    let bootup: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let now: DateTime<Utc> = Utc::now();
    let differ = now.signed_duration_since(bootup).num_seconds();
    let h = differ / 3600;
    let m = (differ % 3600) / 60;
    let s = differ % 60;
    blue!("||\t");
    dark_yellow!("BOOT SINCE\t:\t");
    cyan!("{}h {}m {}s", h, m, s);
    blue_ln!("\t\t\t\t\t\t\t||");
  }

  pad(ps);
  {
    blue!("||\t");
    dark_yellow!("CPU INFO\t:\t");
    cyan!("{} CORE / {} MHZ", cpu_num().unwrap(), cpu_speed().unwrap());
    blue_ln!("\t\t\t\t\t\t||");
  }
  
  pad(ps);
  {
    blue!("||\t");
    dark_yellow!("PROC INFO\t:\t");
    cyan!("{} PROC", proc_total().unwrap());
    blue_ln!("\t\t\t\t\t\t\t||");
  }

  pad(ps);
  {
    blue!("||\t");
    dark_yellow!("LOAD AVG\t:\t");
    if load.one > 1.0 {
      dark_red!("{:.1}", load.one);
    } else {
      dark_yellow!("{:.1}", load.one);
    }
    cyan!("/1M ");

    if load.five > 1.0 {
      dark_red!("{:.1}", load.five);
    } else {
      dark_yellow!("{:.1}", load.five);
    }
    cyan!("/5M ");
    
    if load.fifteen > 1.0 {
      dark_red!("{:.1}", load.fifteen);
    } else {
      dark_yellow!("{:.1}", load.fifteen);
    }
    cyan!("/15M\t");
    green!("[대기시간 (1이하: 이상적)]");
    blue_ln!("\t\t||");
  }

  pad(ps);
  empty_line();

  

  pad(ps);
  {
    blue!("||\t\t\t\t");
    white!("[[  MEMORY INFO ");
    cyan!("<TOTAL: {:.2} GB>", kb_to_gb(mem.total));
    white!("  ]]");
    blue_ln!("\t\t\t\t||");
  }

  pad(ps);
  empty_line();

  pad(ps);
  {
    let mem_data = mem.avail;
    blue!("||\t");
    dark_yellow!("AVAILABLE MEM\t:\t");
    cyan!("{:.2} GB\t\t", kb_to_gb(mem_data));
    green!("{:.0} MB", kb_to_mb(mem_data));
    blue_ln!("\t\t\t\t||");
  }

  pad(ps);
  {
    let mem_data = mem.total - mem.avail;
    blue!("||\t");
    dark_yellow!("USED MEM\t:\t");
    dark_red!("{:.2} GB\t\t", kb_to_gb(mem_data));
    cyan!("{:.0} MB", kb_to_mb(mem_data));
    blue_ln!("\t\t\t\t||");
  }
  
  pad(ps);
  empty_line();

  pad(ps);
  title("DISK INFO");

  pad(ps);
  empty_line();

  pad(ps);
  {
    let disk_data = disk.total;
    blue!("||\t");
    dark_yellow!("DISK TOTAL\t:\t");
    cyan!("{:.2} TB\t\t\t", kb_to_tb(disk_data));
    white!("{:.0} GB", kb_to_gb(disk_data));
    blue_ln!("\t\t\t\t\t||");
  }

  pad(ps);
  {
    let disk_data = disk.free;
    blue!("||\t");
    dark_yellow!("DISK FREE\t:\t");
    dark_red!("{:.2} TB\t\t\t", kb_to_tb(disk_data));
    cyan!("{:.0} GB", kb_to_gb(disk_data));
    blue_ln!("\t\t\t\t\t||");
  }

  pad(ps);
  {
    let disk_data = disk.total - disk.free;
    blue!("||\t");
    dark_yellow!("DISK USED\t:\t");
    cyan!("{:.2} TB\t\t\t", kb_to_tb(disk_data));
    green!("{:.0} GB", kb_to_gb(disk_data));
    blue_ln!("\t\t\t\t\t||");
  }

  pad(ps);
  blue_ln!("  ==============================================================================================  \n\n\n");

  Ok(())
}
