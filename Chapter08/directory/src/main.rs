// Usage example:
// cargo run -- list
// cargo run -- list -d Department
// cargo run -- find -e Employee
// cargo run -- add -e Employee -d Department
// cargo run -- remove -e Employee
// cargo run -- remove -d Department
// cargo run -- remove -e Employee -d Department

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::collections::HashMap;
use std::fs;

fn main() {
  // all the above commands appear to work, but none of the arguments are marked as required
  let matches = App::new("Employee directory tool.")
    .version("1.0.0")
    .author("Brendan A. R. Sechter <sgeos [at] hotmail [dot] com>")
    .about("Tool to edit the department user directory.")
    .subcommand(SubCommand::with_name("list")
      .about("List employees in department or entire directory.")
      .arg(Arg::with_name("DEPARTMENT")
        .help("Department for operation.")
        .short("d")
        .long("department")
        .takes_value(true)))
    .subcommand(SubCommand::with_name("find")
      .about("Find an employee in directory.")
      .arg(Arg::with_name("EMPLOYEE")
        .help("Employee for operation.")
        .short("e")
        .long("employee")
        .takes_value(true)))
    .subcommand(SubCommand::with_name("add")
      .about("Add an employee to the directory.")
      .arg(Arg::with_name("DEPARTMENT")
        .help("Department for operation.")
        .short("d")
        .long("department")
        .takes_value(true))
      .arg(Arg::with_name("EMPLOYEE")
        .help("Employee for operation.")
        .short("e")
        .long("employee")
        .takes_value(true)))
    .subcommand(SubCommand::with_name("remove")
      .about("Remove an employee to the directory.")
      .arg(Arg::with_name("DEPARTMENT")
        .help("Department for operation.")
        .short("d")
        .long("department")
        .takes_value(true))
      .arg(Arg::with_name("EMPLOYEE")
        .help("Employee for operation.")
        .short("e")
        .long("employee")
        .takes_value(true)))
    .arg(Arg::with_name("FILENAME")
       .help("Name of directory file to use for operation.")
       .short("f")
       .long("filename")
       .takes_value(true))
    .get_matches();

  let filename = matches.value_of("FILENAME").unwrap_or("employee_data.sav");
  let mut directory = load(filename);

  let (subcommand, subcommand_matches) = matches.subcommand();
  let (department, employee) = match subcommand_matches {
    Some(matches) => (matches.value_of("DEPARTMENT"), matches.value_of("EMPLOYEE")),
    None => (None, None),
  };

  match (subcommand, department, employee) {
    ("list", Some(department), None) => list_employees_for_directory(&directory, &String::from(department)),
    ("list", None, None) => list_all_employees(&directory),
    ("find", None, Some(employee)) => search_for_employee(&directory, &String::from(employee)),
    ("add", Some(department), Some(employee)) => {
      add_employee_to_department(&mut directory, &String::from(department), &String::from(employee));
      println!("Added employee \"{}\" to department \"{}\".", employee, department);
    },
    ("remove", Some(department), Some(employee)) => {
      remove_employee_from_department(&mut directory, &String::from(department), &String::from(employee));
      println!("Removed employee \"{}\" from department \"{}\".", employee, department);
    },
    ("remove", Some(department), None) => {
      remove_department(&mut directory, &String::from(department));
      println!("Removed department \"{}\".", department);
    },
    ("remove", None, Some(employee)) => {
      remove_employee(&mut directory, &String::from(employee));
      println!("Removed employee \"{}\" from all departments.", employee);
    },
    _ => {
      println!("Invalid arguments");
      list_all_employees(&directory);
    },
  }
  save(filename, &directory);
}

fn save(filename: &str, directory: &HashMap<String, Vec<String>>) {
  let mut data = Vec::new();
  let mut department_list = directory.keys().collect::<Vec<_>>();
  department_list.sort();
  for department in &department_list {
    for employee in directory.get::<String>(department).unwrap_or(&vec![]) {
      data.push(format!("{}\x1F{}", department, employee));
    }
  }
  let data = data.join("\x1E");
  match fs::write(filename, data) {
    Ok(_) => (),
    Err(error) => println!("Error writing to file \"{}\": {:?}", filename, error),
  }
}

fn load(filename: &str) -> HashMap<String, Vec<String>> {
  let data = match fs::read_to_string(filename) {
    Ok(data) => data,
    Err(error) => {
      println!("Error opening file \"{}\": {:?}", filename, error);
      String::from("")
    },
  };
  let mut directory = HashMap::new();
  for line in data.split("\x1E").collect::<Vec<_>>() {
    let data = line.split("\x1F").collect::<Vec<_>>();
    match data.len() {
      0 | 1 => continue,
      2 => {
        let department = String::from(data[0]);
        let employee = String::from(data[1]);
        add_employee_to_department(&mut directory, &department, &employee);
      },
      n => {
        println!("Error parsing file \"{}\": Expected 2 items on line but found {}.", filename, n);
        continue;
      }
    }
  }
  directory
}

fn list_all_employees(directory: &HashMap<String, Vec<String>>) {
  let mut department_list = directory.keys().collect::<Vec<_>>();
  department_list.sort();
  if 0 < department_list.len() {
    for department in &department_list {
      list_employees_for_directory(directory, department);
    }
  } else {
    println!("(No departments.)");
  }
}

fn list_employees_for_directory(directory: &HashMap<String, Vec<String>>, department: &String) {
  let employee_list = &Vec::new();
  let employee_list = directory.get(department).unwrap_or(employee_list);
  println!("--- {} ---", department);
  match employee_list.len() {
    0 => println!("(No employees.)"),
    _n => {
      for employee in employee_list.iter() {
        println!("{}", employee);
      }
    }
  }
}

fn search_for_employee(directory: &HashMap<String, Vec<String>>, employee: &String) {
  let mut department_list = directory.keys().collect::<Vec<_>>();
  department_list.sort();
  let mut search_result_list = Vec::new();
  for department in &department_list {
    if directory.get::<String>(department).unwrap_or(&vec![]).contains(employee) {
      search_result_list.push(department);
    }
  }
  match search_result_list.len() {
    0 => println!("No employee \"{}\" was found.", employee),
    _n => {
      println!("Employee \"{}\" works in the following departments:", employee);
      for department in search_result_list.iter() {
        println!("{}", department);
      }
    }
  }
}

fn add_employee_to_department(directory: &mut HashMap<String, Vec<String>>, department: &String, employee: &String) {
  let employee_list = &mut directory.entry(department.to_string()).or_insert(vec![]);
  if !employee_list.contains(employee) {
    employee_list.push(employee.to_string());
    employee_list.sort();
  }
}

fn remove_department(directory: &mut HashMap<String, Vec<String>>, department: &String) {
  directory.remove(department);
}

fn remove_employee(directory: &mut HashMap<String, Vec<String>>, employee: &String) {
  let department_list = directory.keys().cloned().collect::<Vec<String>>();
  for department in department_list.iter() {
    remove_employee_from_department(directory, department, employee);
  }
}

fn remove_employee_from_department(directory: &mut HashMap<String, Vec<String>>, department: &String, employee: &String) {
  let employee_list = directory
    .entry(department.to_string())
    .or_insert(vec![])
    .into_iter()
    .filter_map( |current| if current.to_string() == employee.to_string() { None } else { Some(current.to_string()) } )
    .collect::<Vec<String>>();
  if 0 < employee_list.len() {
    directory.insert(department.to_string(), employee_list);
  } else {
    remove_department(directory, department);
  }
}

