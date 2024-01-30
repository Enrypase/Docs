/*
    This is a new module!
    We can add private/public functions too!
*/

mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            return Pizza {
                dough: String::from("Regular dough"),
                topping: String::from(topping),
            };
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        fn serve_customer(pizza: super::Pizza) {
            println!("The customer was served with {}", pizza.topping);
        }
        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
