pub mod ram {
  use sys_info::mem_info;
  use std::string::String;
  use sysinfo::{System, SystemExt, Pid, ProcessExt};
  use terminal_menu::{menu, run, button, mut_menu, TerminalMenuItem};

use crate::terminal::terminal::terminal::{go_back, show_cpu_options};
  pub fn show_free_memory() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let mem = mem_info().unwrap();
        println!("{} free bytes", mem.free); 
    }
  }

  pub fn show_processes() {
    let sys = System::new_all();

    print!("\x1B[2J\x1B[1;1H");
    println!("Select a process to see the info:");

    let mut menu_buttons: Vec<TerminalMenuItem> = 
      sys.processes().iter().map(|process| {
        button(String::from(format!("{}-{}", process.0,process.1.name())))
      }).collect();

    menu_buttons.push(button("Go back"));
    menu_buttons.push(button("Close"));

    let menu = menu(menu_buttons);
    run(&menu);

    let mutmen = mut_menu(&menu);
    
    match mutmen.selected_item_name() {
        "Go back" => show_cpu_options(),
        "Close" => std::process::exit(0),
        _ => {
          let item: Vec<&str> = mutmen.selected_item_name().split("-").collect();
          let pid:Pid = Pid::from((item[0].parse::<u32>().unwrap() as u32) as usize);
          show_process_info(pid);
        }
    }
  }

  fn show_process_info(pid: Pid) {
    let sys = System::new_all();
    let process = sys.process(pid).unwrap();
    print!("\x1B[2J\x1B[1;1H");
    println!("Process '{}' info", process.name());
    println!("Name: {}",process.name());
    println!("Process id: {}", process.pid());
    println!("Root path: {:?}", process.root());
    println!("Cpu usage: {}", process.cpu_usage());
    println!("Disk usage: {:?}", process.disk_usage());
    println!("Memory usage: {}mb", process.memory()/1000000);
    println!("Vistual memory usage: {}mb", process.virtual_memory()/1000000);
    go_back(show_processes);
  }
}

