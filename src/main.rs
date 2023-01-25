use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    _dept_list: HashMap::<String, Vec<String>>
}

impl Company {
    pub fn new(&mut self) -> &mut Company {
        self
    }
    pub fn insert_employee<'a>(comp:&'a mut Company, 
        dept_name: String, 
        emp_name: String) {
        let emp_vect = comp
            ._dept_list
            .entry(dept_name)
            .or_insert(Vec::new());
        emp_vect.push(emp_name.to_string());
    }
}

fn main() {
    let mut employee_name = String::new();
    let mut dept_name = String::new();
    let mut company = Company{_dept_list:HashMap::new()};
    let company = Company::new(&mut company);
    while employee_name != "q".to_string() {
        println!("Enter an employee name or q to quit");
        employee_name = "".to_string();
        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read line");
        employee_name = employee_name.trim().to_string();
        if employee_name != "q".to_string() {
            println!("Enter the department of the employee");
            io::stdin()
                .read_line(&mut dept_name)
                .expect("Failed to read line");
            dept_name = dept_name.trim().to_string();
            Company::insert_employee(company, 
                dept_name.to_string(), 
                employee_name.to_string());
            dept_name = "".to_string();
        }
    }
    for (dept, emply) in &company._dept_list {
        println!("Department: {dept} has the following employees");
        let mut employees: Vec<String> = Vec::new();
        for name in emply {
            employees.push(name.to_string());
        }
        employees.sort_by_key(|n| n.to_lowercase());
        for name in employees {
            println!("   Employee name: {name}");
        }
    }
}
