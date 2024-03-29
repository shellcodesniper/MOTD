extern crate reqwest;
extern crate colour;
extern crate sys_info;
extern crate chrono;
extern crate terminal_size;
extern crate interfaces;

use chrono::prelude::*;
use colour::dark_grey;
use std::net;
use std::io::{ Read };
use sys_info::*;
use colour::{ blue_ln, blue, dark_yellow, dark_red, cyan, green, white };
use terminal_size::{Width, Height, terminal_size};

use interfaces::{
    flags::{InterfaceFlags},
    Interface, Kind,
};

// ? TAB SIZE, 8을 기준으로 맞춰진 상태라 8 권장.
const MOTD_WIDTH: f32 = 98.0;
// ? MOTD 총 길이 ( MOTD 가로 글자수만 포함하면됨 <계산결과 64> ( TAB 8 개))


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

    blue!("||\t");
    
    cyan!("[[\t\t\t\t");
    white!("{}", title);
    cyan!("\t\t\t\t]]");
    blue_ln!("\t||");
  }
}
fn empty_line() {
  blue_ln!("||\t\t\t\t\t\t\t\t\t\t\t||");
}


fn format_addr(addr: &net::SocketAddr) -> String {
    match addr {
        &net::SocketAddr::V4(ref a) => format!("{}", a.ip()),
        &net::SocketAddr::V6(ref a) => format!("{}", a.ip()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut temp_string: String = String::new();
  let mut ps: &str = "";
  let size = terminal_size();
  if let Some((Width(w), Height(_))) = size {
    let temp: f32 = (((w as f32) / 2.0 - (MOTD_WIDTH / 2.0))).ceil();
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
  blue_ln!("  ========================================================================================  ");

  pad(ps);
  empty_line();

  pad(ps);
  title("NETWORK INFO");

  pad(ps);
  empty_line();

  pad(ps);
  {
    blue!("||\t\t");
    dark_yellow!("공인 아이피\t:\t");
    dark_red!("{}\t\t\t\t", public_ip);
    blue_ln!("\t||");
  }

  
  pad(ps);
  empty_line();
  pad(ps);
  title("INTERFACE LIST");
  pad(ps);
  empty_line();
  {
    let mut ifs = Interface::get_all().expect("could not get interfaces");
    ifs.sort_by(|a, b| a.name.cmp(&b.name));

    for i in ifs.iter() {

        let mut prefix_list: Vec<String> = Vec::new();
        let mut addr_list: Vec<String> = Vec::new();
        for addr in i.addresses.iter() {
            let raddr = match addr.addr {
                Some(a) => a,
                None => continue,
            };

            let prefix = match addr.kind {
                Kind::Ipv4 => "ipv4",
                Kind::Ipv6 => continue,
                _ => continue,
            };
            let x = format_addr(&raddr);
            addr_list.push(x);
            prefix_list.push(String::from(prefix));
        }

        if addr_list.len() == 0 {
          continue;
        }

        let name = i.name.clone();
        if name.starts_with("en") || name.starts_with("ipsec") {
          if i.flags.contains(InterfaceFlags::IFF_LOOPBACK) {
              continue;
          } else {
              if let Ok(addr) = i.hardware_addr() {
                  pad(ps);
                  blue!("||\t\t");
                  green!("{}\t", name.to_uppercase());
                  cyan!("[  ");
                  white!("{}", addr);
                  cyan!("  ]");
                  blue_ln!("\t\t\t\t\t\t||");
              }

              for _ in 0..addr_list.len() {
                let x = addr_list.pop();
                let y = prefix_list.pop();
                if let Some(s) = x {
                  if let Some(d) = y {
                    pad(ps);
                    blue!("||\t\t");
                    dark_grey!("  {}\t:", d);
                    dark_grey!("    {}", s);
                    blue_ln!("\t\t\t\t\t\t||");
                  }
                }
              }
              pad(ps);
              empty_line();
          }

        }
    }
  }

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
    blue!("||\t\t");
    dark_yellow!("BOOT SINCE\t:\t");
    cyan!("{}h {}m {}s", h, m, s);
    blue_ln!("\t\t\t\t\t||");
  }

  pad(ps);
  {
    blue!("||\t\t");
    dark_yellow!("CPU INFO\t:\t");
    cyan!("{} CORE / {} MHZ", cpu_num().unwrap(), cpu_speed().unwrap_or(0));
    blue_ln!("\t\t\t\t||");
  }
  
  pad(ps);
  {
    blue!("||\t\t");
    dark_yellow!("PROC INFO\t:\t");
    cyan!("{} PROC", proc_total().unwrap());
    blue_ln!("\t\t\t\t\t||");
  }

  pad(ps);
  {
    blue!("||\t\t");
    dark_yellow!("LOAD AVG\t:\t");
    if load.one > 2.5 {
      dark_red!("{:.1}", load.one);
    } else {
      green!("{:.1}", load.one);
    }
    cyan!("/1M ");

    if load.five > 4.0 {
      dark_red!("{:.1}", load.five);
    } else {
      green!("{:.1}", load.five);
    }
    cyan!("/5M ");
    
    if load.fifteen > 5.0 {
      dark_red!("{:.1}", load.fifteen);
    } else {
      green!("{:.1}", load.fifteen);
    }
    cyan!("/15M\t");
    green!("[LOW IS BETTER]");
    blue_ln!("\t\t||");
  }

  pad(ps);
  empty_line();

  

  pad(ps);
  {
    blue!("||\t");
    cyan!("[[\t\t\t");
    white!("MEMORY INFO\t");
    cyan!("<TOTAL: {:.0} GB>", kb_to_gb(mem.total));
    cyan!("\t\t\t]]");
    blue_ln!("\t||");
  }

  pad(ps);
  empty_line();

  pad(ps);
  {
    let mem_data = mem.avail;
    blue!("||\t\t");
    dark_yellow!("AVAILABLE MEM\t:\t");
    cyan!("{:.2} GB\t\t", kb_to_gb(mem_data));
    green!("{:.0} MB", kb_to_mb(mem_data));
    blue_ln!("\t\t||");
  }

  pad(ps);
  {
    let mem_data = mem.total - mem.avail;
    blue!("||\t\t");
    dark_yellow!("USED MEM\t:\t");
    dark_red!("{:.2} GB\t\t", kb_to_gb(mem_data));
    cyan!("{:.0} MB", kb_to_mb(mem_data));
    blue_ln!("\t\t||");
  }
  
  pad(ps);
  empty_line();

  pad(ps);
  title("DISK INFO");

  pad(ps);
  empty_line();

  pad(ps);
  {
    let disk_data: u64 = 38947782755;
    blue!("||\t\t");
    dark_yellow!("DISK TOTAL\t:\t");
    let disk_tb = format!("{:.2} TB", kb_to_tb(disk_data));
    cyan!("{}", disk_tb);
    if disk_tb.len() < 8 {
      print!("\t\t\t");
    } else {
      print!("\t\t");
    }
    let disk_gb = format!("{:.0} GB", kb_to_gb(disk_data));
    cyan!("{}", disk_gb);
    if disk_gb.len() < 8 {
      blue_ln!("\t\t\t||");
    } else {
      blue_ln!("\t\t||");
    }
  }

  pad(ps);
  {
    let disk_data = disk.free;
    blue!("||\t\t");
    dark_yellow!("DISK FREE\t:\t");
    let disk_tb = format!("{:.2} TB", kb_to_tb(disk_data));
    cyan!("{}", disk_tb);
    if disk_tb.len() < 8 {
      print!("\t\t\t");
    } else {
      print!("\t\t");
    }
    let disk_gb = format!("{:.0} GB", kb_to_gb(disk_data));
    cyan!("{}", disk_gb);
    if disk_gb.len() < 8 {
      blue_ln!("\t\t\t||");
    } else {
      blue_ln!("\t\t||");
    }
  }

  pad(ps);
  {
    let disk_data = disk.total - disk.free;
    blue!("||\t\t");
    dark_yellow!("DISK USED\t:\t");
    let disk_tb = format!("{:.2} TB", kb_to_tb(disk_data));
    cyan!("{}", disk_tb);
    if disk_tb.len() < 8 {
      print!("\t\t\t");
    } else {
      print!("\t\t");
    }
    let disk_gb = format!("{:.0} GB", kb_to_gb(disk_data));
    cyan!("{}", disk_gb);
    if disk_gb.len() < 8 {
      blue_ln!("\t\t\t||");
    } else {
      blue_ln!("\t\t||");
    }
  }

  pad(ps);
  blue_ln!("  ========================================================================================  \n\n\n");

  Ok(())
}
