pub mod terminal {
  use crate::ram::ram::ram::{show_processes, show_free_memory};
  use terminal_menu::{menu, run, button, mut_menu};
  use crate::{cpu::cpu::cpu::{show_cpu_usage, show_system_info}, memory::memory::memory::{show_disk, show_networks}};
  pub fn show_principal_menu() {
    print!("\x1B[2J\x1B[1;1H");
    let menu = menu(vec![
      button("Disks"),
      button("Ram"),
      button("Cpu"),
      button("System Info"),
      button("Close")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Disks" => show_disk_options(),
        "Ram" => show_ram_options(),
        "Cpu" => show_cpu_options(),
        "System Info" => show_system_info(),
        "Close" => end_terminal(),
        &_ => println!("Select a valid option")
    }
  }

  pub fn show_ram_options() {
    print!("\x1B[2J\x1B[1;1H");
    let menu = menu(vec![
      button("Show processes"),
      button("Show free memory"),
      button("Go Back"),
      button("Close")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Show processes" => show_processes(),
        "Show free memory" => show_free_memory(),
        "Go Back" => show_principal_menu(),
        "Close" => end_terminal(),
        &_ => println!("Select a valid option")
    }
  }

  pub fn show_disk_options() {
    print!("\x1B[2J\x1B[1;1H");
    let menu = menu(vec![
      button("Disk usage"),
      button("Dirs"),
      button("Go Back"),
      button("Close")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Disk usage" => show_disk(),
        "Dirs" => println!("Show Directories and options"),
        "Go Back" => show_principal_menu(),
        "Close" => end_terminal(),
        &_ => println!("Select a valid option")
    }
  }

  pub fn show_cpu_options () {
    print!("\x1B[2J\x1B[1;1H");
    let menu = menu(vec![
      button("show cpus usage"),
      button("show networks"),
      button("Go Back"),
      button("Close")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
      "show cpus usage" => show_cpu_usage(),
      "show networks" => show_networks(),
      "Go Back" => show_principal_menu(),
      "Close" => end_terminal(),
      &_ => {println!("Select a valid option"); show_cpu_options()}
    }
  }

  fn end_terminal() {
    use std::process;
    process::exit(0)
  }

  pub fn go_back(terminal_stage: fn()) {
    let menu = menu(vec![
      button("Go Back"),
      button("Close")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
      "Go Back" => terminal_stage(),
      "Close" => end_terminal(),
      &_ => println!("Select a valid option")
    }
  }
}
