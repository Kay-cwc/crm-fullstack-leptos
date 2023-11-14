use leptos::*;

#[derive(Debug, Clone)]
pub struct Customer {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
}

impl Customer {
    pub fn new(id: usize, new_customer: &CustomerForm) -> Customer {
        Customer {
            id,
            first_name: new_customer.first_name.clone(),
            last_name: new_customer.last_name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomerForm {
    pub first_name: String,
    pub last_name: String,
}

impl CustomerForm {
    pub fn new(first_name: String, last_name: String) -> CustomerForm {
        CustomerForm {
            first_name,
            last_name,
        }
    }
    
    pub fn empty() -> CustomerForm {
        CustomerForm {
            first_name: "".to_string(),
            last_name: "".to_string(),
        }
    }
}
