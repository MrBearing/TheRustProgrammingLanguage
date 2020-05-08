use std::collections::HashMap;

type Employees = HashMap<String,String>;


fn add(employees :&mut Employees,command:&str){
    let obj :Vec<&str>= command.splitn(2," to ").collect();
    println!("add {:?} as {:?}",obj[0].to_string(), obj[1].to_string());
    employees.insert(obj[0].to_string(), obj[1].to_string());
}

fn list(employees :&Employees,command:&str){
    if employees.is_empty() {
        println!("No Data");
        return ()
    }
    println!("list {:?}",command);
    if command == "All" {
        for name in employees.keys(){
            println!("{:?}",name);
        }
        return ()
    }
    for (name , depart) in employees.iter(){
        if command == depart {
            println!("{:?}",name);
        }
    }
}

fn help(){
    println!("**********This is help************")
}

pub fn command(employees :&mut Employees,command_string: String) -> bool{
    let command: Vec<&str> = command_string.splitn(2,' ').collect();
    println!("{:?}",command[0]);
    match command[0].trim() {
        "Exit" => false,
        "Add" => {
            if command.len() > 1 {
                add(employees,command[1].trim());
            }
            true
        },
        "List" => {
            if command.len() > 1 {
                list(employees,command[1].trim());
            }
            true
        },
        _ => {
            help();
            true
        },
    }
}

use std::io;
pub fn run(){
    let mut employees = Employees::new();
    loop {
        println!("command? >>");
        let mut command_string = String::new();
        io::stdin().read_line(&mut command_string)
            .expect("Failed to read line");
        if ! command(&mut employees, command_string) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_command() {
        let mut employees = Employees::new();
        assert_eq!(command(&mut employees,"Add a to B".to_string()),true);
        assert_eq!(command(&mut employees,"List All".to_string()),true);
        assert_eq!(command(&mut employees,"List B".to_string()),true);
        assert_eq!(command(&mut employees,"Exit".to_string()),false);
        assert_eq!(command(&mut employees,"".to_string()),true);
    }

    #[test]
    fn test_add(){
        let mut employees = Employees::new();
        add(&mut employees,&"a to B".to_string());
        println!("{:?}",employees);
        assert_eq!(employees.get(&"a".to_string()), Some(&"B".to_string()));
    }

}