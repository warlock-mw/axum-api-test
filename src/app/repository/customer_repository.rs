use crate::domain::entity::customer::Customer;

pub fn get_all() -> Vec<Customer> {
    vec![
        Customer {
            id: 1, 
            name: String::from("太郎")
        },
        Customer {
            id: 2,
            name: String::from("次郎")
        },
        Customer {
            id: 3, 
            name: String::from("三郎")
        }
    ]
}

pub fn add_all(list: Vec<Customer>) -> usize {
    list.into_iter().count()
}