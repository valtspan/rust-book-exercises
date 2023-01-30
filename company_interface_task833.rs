use std::collections::{hash_map::Entry, HashMap};

struct Company<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> Company<'a> {
    fn new() -> Self {
        Self(HashMap::from([("Sales", vec![]), ("Engineering", vec![]), ]))
    }

    fn decipher_expr(&mut self, expr: &'a str) {
        let v: Vec<&str> = expr.split_whitespace().collect();
        match v[0] {
            "Add" => self.add_employee(v[1], v[3]),
            "Remove" => self.remove_employee(v[1], v[3]),
            "Get" => self.print_employees(v[1]),
            _ => {}
        }
    }

    fn add_employee(&mut self, name: &'a str, dept: &'a str) {
        if let Entry::Vacant(_) = self.0.entry(dept).and_modify(|v| v.push(name)) {    
           panic!("There is no department {:?}", dept); 
        }
    }
    
    // removes all instances of /name/ in a department
    // name has to be unique
    fn remove_employee(&mut self, name: &'a str, dept: &'a str) {
        if let Entry::Vacant(_) = self.0.entry(dept).and_modify(|v| v.retain(|x| *x != name)) { 
            panic!("There is no department {:?}", dept);

        }
    }

    fn print_employees(&self, dept: &str) {
        if let Some(values) = self.0.get(dept) {
            let mut to_sorted = values.clone();
            to_sorted.sort();       
            for e in &to_sorted {
                println!("{e}");
            }
        } else {
           println!("There are no employees in this department yet.");
        }
    }

}

fn main() {
    let mut company = Company::new();

    company.decipher_expr("Add Sally to Engineering");
    company.decipher_expr("Add Bob to Engineering");
    company.decipher_expr("Remove Sally from Engineering");
    company.decipher_expr("Add Megan to Engineering");
    company.decipher_expr("Add John to Sales");
    company.decipher_expr("Get Engineering");
    dbg!(company.0);
}
