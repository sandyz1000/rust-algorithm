#![allow(unused)]
use std::fmt;

use strum_macros::Display;

#[derive(Debug, Display)]
enum ReservationStatus {
    Requested,
    Pending,
    Confirmed,
    CheckedIn,
    Canceled,
    Abandoned,
}

#[derive(Debug, Display)]
enum SeatType {
    Regular,
    Kid,
    Accessible,
    Other,
}

#[derive(Debug, Display, Clone)]
enum OrderStatus {
    Received,
    Preparing,
    Completed,
    Canceled,
    None,
}

#[derive(Debug, Display)]
enum TableStatus {
    Free,
    Reserved,
    Occupied,
    Other,
}


#[derive(Debug, Display)]
enum AccountStatus {
    Active,
    Closed,
    Canceled,
    Blacklisted,
    Blocked,
}


#[derive(Debug, Display)]
enum PaymentStatus {
    Unpaid,
    Pending,
    Completed,
    Filled,
    Declined,
    Cancelled,
    Abandoned,
    Settling,
    Settled,
    Refunded,
}

struct Address {
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

impl Address {
    fn new(street: String, city: String, state: String, zip_code: String, country: String) -> Self {
        Address {
            street_address: street,
            city,
            state,
            zip_code,
            country,
        }
    }
}

use std::fmt::Display;

struct Account {
    id: String,
    password: String,
    address: Address,
    status: AccountStatus,
}

impl Account {
    fn new(id: String, password: String, address: Address, status: AccountStatus) -> Self {
        Account {
            id,
            password,
            address,
            status,
        }
    }

    fn reset_password(&self) {
        // Implementation goes here
    }
}

struct Person {
    name: String,
    email: String,
    phone: String,
}

impl Person {
    fn new(name: String, email: String, phone: String) -> Self {
        Person {
            name,
            email,
            phone,
        }
    }
}

trait Employee {
    fn employee_id(&self) -> &str;
    fn date_joined(&self) -> &str;
    fn account(&self) -> &Account;
}

struct Receptionist {
    id: String,
    account: Account,
    name: String,
    email: String,
    phone: String,
}

impl Employee for Receptionist {
    fn employee_id(&self) -> &str {
        &self.id
    }

    fn date_joined(&self) -> &str {
        // Implementation goes here
        unimplemented!()
    }

    fn account(&self) -> &Account {
        &self.account
    }
}

impl Receptionist {
    fn new(id: String, account: Account, name: String, email: String, phone: String) -> Self {
        Receptionist {
            id,
            account,
            name,
            email,
            phone,
        }
    }

    fn create_reservation(&self) {
        // Implementation goes here
    }

    fn search_customer(&self, name: &str) {
        // Implementation goes here
    }
}

struct Manager {
    id: String,
    account: Account,
    name: String,
    email: String,
    phone: String,
}

impl Employee for Manager {
    fn employee_id(&self) -> &str {
        &self.id
    }

    fn date_joined(&self) -> &str {
        // Implementation goes here
        unimplemented!()
    }

    fn account(&self) -> &Account {
        &self.account
    }
}

impl Manager {
    fn new(id: String, account: Account, name: String, email: String, phone: String) -> Self {
        Manager {
            id,
            account,
            name,
            email,
            phone,
        }
    }

    fn add_employee(&self) {
        // Implementation goes here
    }
}

struct Chef {
    id: String,
    account: Account,
    name: String,
    email: String,
    phone: String,
}

impl Employee for Chef {
    fn employee_id(&self) -> &str {
        &self.id
    }

    fn date_joined(&self) -> &str {
        // Implementation goes here
        unimplemented!()
    }

    fn account(&self) -> &Account {
        &self.account
    }
}

impl Chef {
    fn new(id: String, account: Account, name: String, email: String, phone: String) -> Self {
        Chef {
            id,
            account,
            name,
            email,
            phone,
        }
    }

    fn take_order(&self) {
        // Implementation goes here
    }
}


struct Table {
    table_id: String,
    max_capacity: u32,
    location_identifier: String,
    status: TableStatus,
    seats: Vec<TableSeat>,
}

impl Table {
    fn new(table_id: String, max_capacity: u32, location_identifier: String, status: TableStatus) -> Self {
        Table {
            table_id,
            max_capacity,
            location_identifier,
            status,
            seats: Vec::new(),
        }
    }

    fn is_table_free(&self) -> bool {
        // Implementation goes here
        false
    }

    fn add_reservation(&mut self) {
        // Implementation goes here
    }

    fn search(&self, capacity: u32, start_time: chrono::DateTime<chrono::Utc>) -> Vec<&Table> {
        // Implementation goes here
        Vec::new()
    }
}

struct TableSeat {
    table_seat_number: u32,
    seat_type: SeatType,
}

impl TableSeat {
    fn new() -> Self {
        TableSeat {
            table_seat_number: 0,
            seat_type: SeatType::Regular,
        }
    }

    fn update_seat_type(&mut self, seat_type: SeatType) {
        // Implementation goes here
    }
}

struct Notification;

struct Customer;


struct Reservation {
    reservation_id: String,
    time_of_reservation: chrono::DateTime<chrono::Utc>,
    people_count: u32,
    status: ReservationStatus,
    notes: String,
    checkin_time: chrono::DateTime<chrono::Utc>,
    customer: Customer,
    tables: Vec<Table>,
    notifications: Vec<Notification>,
}

impl Reservation {
    fn new(
        reservation_id: String,
        people_count: u32,
        notes: String,
        customer: Customer,
    ) -> Self {
        Reservation {
            reservation_id,
            time_of_reservation: chrono::Utc::now(),
            people_count,
            status: ReservationStatus::Requested,
            notes,
            checkin_time: chrono::Utc::now(),
            customer,
            tables: Vec::new(),
            notifications: Vec::new(),
        }
    }

    fn update_people_count(&mut self, count: u32) {
        // Implementation goes here
    }
}

struct Kitchen {
    name: String,
    chefs: Vec<Chef>,
}

impl Kitchen {
    fn new(name: String) -> Self {
        Kitchen {
            name,
            chefs: Vec::new(),
        }
    }

    fn assign_chef(&mut self, chef: Chef) {
        // Implementation goes here
    }
}

struct Branch {
    name: String,
    location: String,
    kitchen: Kitchen,
}

impl Branch {
    fn new(name: String, location: String, kitchen: Kitchen) -> Self {
        Branch {
            name,
            location,
            kitchen,
        }
    }

    fn add_table_chart(&self) {
        // Implementation goes here
    }
}

struct Restaurant {
    name: String,
    branches: Vec<Branch>,
}

impl Restaurant {
    fn new(name: String) -> Self {
        Restaurant {
            name,
            branches: Vec::new(),
        }
    }

    fn add_branch(&mut self, branch: Branch) {
        // Implementation goes here
    }
}

struct TableChart {
    table_chart_id: String,
    table_chart_image: Vec<u8>,
}

impl TableChart {
    fn new(id: String) -> Self {
        TableChart {
            table_chart_id: id,
            table_chart_image: Vec::new(),
        }
    }

    fn print(&self) {
        // Implementation goes here
    }
}

struct MenuItem {
    menu_item_id: String,
    title: String,
    description: String,
    price: f32,
}

impl MenuItem {
    fn new(id: String, title: String, description: String, price: f32) -> Self {
        MenuItem {
            menu_item_id: id,
            title,
            description,
            price,
        }
    }

    fn update_price(&mut self, price: f32) {
        // Implementation goes here
    }
}

struct MenuSection {
    menu_section_id: String,
    title: String,
    description: String,
    menu_items: Vec<MenuItem>,
}

impl MenuSection {
    fn new(id: String, title: String, description: String) -> Self {
        MenuSection {
            menu_section_id: id,
            title,
            description,
            menu_items: Vec::new(),
        }
    }

    fn add_menu_item(&mut self, menu_item: MenuItem) {
        // Implementation goes here
    }
}

struct Menu {
    menu_id: String,
    title: String,
    description: String,
    menu_sections: Vec<MenuSection>,
}

impl Menu {
    fn new(id: String, title: String, description: String) -> Self {
        Menu {
            menu_id: id,
            title,
            description,
            menu_sections: Vec::new(),
        }
    }

    fn add_menu_section(&mut self, menu_section: MenuSection) {
        // Implementation goes here
        unimplemented!()
    }

    fn print(&self) {
        // Implementation goes here
        unimplemented!()
    }
}

struct MealItem {
    meal_item_id: String,
    quantity: u32,
    menu_item: MenuItem,
}

impl MealItem {
    fn new(meal_item_id: String, quantity: u32, menu_item: MenuItem) -> Self {
        MealItem {
            meal_item_id,
            quantity,
            menu_item,
        }
    }

    fn update_quantity(&mut self, quantity: u32) {
        // Implementation goes here
    }
}

struct Seat;

struct Meal {
    meal_id: String,
    seat: Seat,
    meal_items: Vec<MealItem>,
}

impl Meal {
    fn new(meal_id: String, seat: Seat) -> Self {
        Meal {
            meal_id,
            seat,
            meal_items: Vec::new(),
        }
    }

    fn add_meal_item(&mut self, meal_item: MealItem) {
        // Implementation goes here
    }
}

struct Check;
struct Waiter;

struct Order {
    order_id: String,
    status: OrderStatus,
    creation_time: chrono::DateTime<chrono::Utc>,
    meals: Vec<Meal>,
    table: Table,
    waiter: Waiter,
    chef: Chef,
    check: Check,
}

impl Order {
    fn new(
        order_id: String,
        status: OrderStatus,
        table: Table,
        waiter: Waiter,
        chef: Chef,
    ) -> Self {
        Order {
            order_id,
            status,
            creation_time: chrono::Utc::now(),
            meals: Vec::new(),
            table,
            waiter,
            chef,
            check: Check {},
        }
    }

    fn add_meal(&mut self, meal: Meal) {
        // Implementation goes here
    }

    fn remove_meal(&mut self, meal: Meal) {
        // Implementation goes here
    }

    fn get_status(&self) -> OrderStatus {
        self.status.clone()
    }

    fn set_status(&mut self, status: OrderStatus) {
        // Implementation goes here
    }
}


fn main() {
    println!("Hello, world!");
}