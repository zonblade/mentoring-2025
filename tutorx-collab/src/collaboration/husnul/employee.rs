pub struct Employee {
    name: String,
    age: u32,
    company: Company,
}

#[derive(Clone)]
pub struct Company {
    name: String,
    address: String,
}

impl Employee {
    fn change_name(&mut self,  new_name: String)->Self {
        Self {
            name: new_name,
            age: self.age,
            company: self.company.clone(),
        }
    }
}

fn change_employee_name(employee: &mut Employee, new_name: String){
    employee.name = new_name
}

fn change_employee_company(employee: &mut Employee, new_company: Company){
    employee.company = new_company
}