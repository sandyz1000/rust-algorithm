#![allow(unused)]

use std::fmt;
use strum_macros::Display;
use std::collections::HashMap;


#[derive(Debug, Display)]
enum BillItemType {
    BaseCharge,
    AdditionalService,
    Fine,
    Other,
}


#[derive(Debug, Display)]
enum VehicleLogType {
    Accident,
    Fueling,
    CleaningService,
    OilChange,
    Repair,
    Other,
}

#[derive(Debug, Display)]
enum VanType {
    Passenger,
    Cargo,
}


#[derive(Debug, Display)]
enum CarType {
    Economy,
    Compact,
    Intermediate,
    Standard,
    FullSize,
    Premium,
    Luxury,
}


#[derive(Debug, Display)]
enum VehicleStatus {
    Available,
    Reserved,
    Loaned,
    Lost,
    BeingServiced,
    Other,
}


#[derive(Debug, Display)]
enum ReservationStatus {
    Active,
    Inactive,
    Pending,
    Confirmed,
    Completed,
    Cancelled,
    None,
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


struct Person {
    name: String,
    address: Address,
    email: String,
    phone: String,
}

struct Account {
    id: String,
    password: String,
    status: AccountStatus,
    person: Person,
}

impl Account {
    fn new(id: String, password: String, person: Person, status: AccountStatus) -> Self {
        Account {
            id,
            password,
            status,
            person,
        }
    }

    fn reset_password(&self) {
        // Implement reset password logic here
    }
}

struct Member {
    total_vehicles_reserved: u32,
    account: Account,
}

impl Member {
    fn new(account: Account) -> Self {
        Member {
            total_vehicles_reserved: 0,
            account,
        }
    }

    fn get_reservations(&self) {
        // Implement get reservations logic here
    }
}

struct Receptionist {
    date_joined: String,
    account: Account,
}

impl Receptionist {
    fn new(date_joined: String, account: Account) -> Self {
        Receptionist {
            date_joined,
            account,
        }
    }

    fn search_member(&self, name: String) {
        // Implement search member logic here
    }
}

struct AdditionalDriver {
    driver_id: String,
    person: Person,
}

impl AdditionalDriver {
    fn new(driver_id: String, person: Person) -> Self {
        AdditionalDriver {
            driver_id,
            person,
        }
    }
}

struct Address {
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}


struct CarRentalLocation {
    name: String,
    location: String,
}

impl CarRentalLocation {
    fn new(name: String, location: String) -> Self {
        CarRentalLocation {
            name,
            location,
        }
    }

    fn get_location(&self) -> &str {
        &self.location
    }
}

struct CarRentalSystem {
    name: String,
    locations: Vec<CarRentalLocation>,
}

impl CarRentalSystem {
    fn new(name: String) -> Self {
        CarRentalSystem {
            name,
            locations: vec![],
        }
    }

    fn add_new_location(&mut self, location: CarRentalLocation) {
        self.locations.push(location);
    }
}


trait Search {
    fn search_by_type(&self, type_query: &str) -> Option<&Vec<String>>;
    fn search_by_model(&self, model_query: &str) -> Option<&Vec<String>>;
}

struct VehicleInventory {
    vehicle_types: HashMap<String, Vec<String>>,
    vehicle_models: HashMap<String, Vec<String>>,
}

impl VehicleInventory {
    fn new() -> Self {
        VehicleInventory {
            vehicle_types: HashMap::new(),
            vehicle_models: HashMap::new(),
        }
    }
}

impl Search for VehicleInventory {
    fn search_by_type(&self, type_query: &str) -> Option<&Vec<String>> {
        self.vehicle_types.get(type_query)
    }

    fn search_by_model(&self, model_query: &str) -> Option<&Vec<String>> {
        self.vehicle_models.get(model_query)
    }
}

struct Vehicle {
    license_number: String,
    stock_number: String,
    passenger_capacity: u32,
    barcode: String,
    has_sunroof: bool,
    status: String,
    model: String,
    make: String,
    manufacturing_year: u32,
    mileage: u32,
    log: Vec<VehicleLog>,
}

impl Vehicle {
    fn new(
        license_number: String,
        stock_number: String,
        passenger_capacity: u32,
        barcode: String,
        has_sunroof: bool,
        status: String,
        model: String,
        make: String,
        manufacturing_year: u32,
        mileage: u32,
    ) -> Self {
        Vehicle {
            license_number,
            stock_number,
            passenger_capacity,
            barcode,
            has_sunroof,
            status,
            model,
            make,
            manufacturing_year,
            mileage,
            log: vec![],
        }
    }

    fn reserve_vehicle(&self) {
        // Implement reserve_vehicle logic here
    }

    fn return_vehicle(&self) {
        // Implement return_vehicle logic here
    }
}

struct Car {
    vehicle: Vehicle,
    type_: String,
}

impl Car {
    fn new(
        license_number: String,
        stock_number: String,
        passenger_capacity: u32,
        barcode: String,
        has_sunroof: bool,
        status: String,
        model: String,
        make: String,
        manufacturing_year: u32,
        mileage: u32,
        type_: String,
    ) -> Self {
        Car {
            vehicle: Vehicle::new(
                license_number,
                stock_number,
                passenger_capacity,
                barcode,
                has_sunroof,
                status,
                model,
                make,
                manufacturing_year,
                mileage,
            ),
            type_,
        }
    }
}

struct Van {
    vehicle: Vehicle,
    type_: String,
}

impl Van {
    fn new(
        license_number: String,
        stock_number: String,
        passenger_capacity: u32,
        barcode: String,
        has_sunroof: bool,
        status: String,
        model: String,
        make: String,
        manufacturing_year: u32,
        mileage: u32,
        type_: String,
    ) -> Self {
        Van {
            vehicle: Vehicle::new(
                license_number,
                stock_number,
                passenger_capacity,
                barcode,
                has_sunroof,
                status,
                model,
                make,
                manufacturing_year,
                mileage,
            ),
            type_,
        }
    }
}

struct Truck {
    vehicle: Vehicle,
    type_: String,
}

impl Truck {
    fn new(
        license_number: String,
        stock_number: String,
        passenger_capacity: u32,
        barcode: String,
        has_sunroof: bool,
        status: String,
        model: String,
        make: String,
        manufacturing_year: u32,
        mileage: u32,
        type_: String,
    ) -> Self {
        Truck {
            vehicle: Vehicle::new(
                license_number,
                stock_number,
                passenger_capacity,
                barcode,
                has_sunroof,
                status,
                model,
                make,
                manufacturing_year,
                mileage,
            ),
            type_,
        }
    }
}

struct VehicleLog {
    id: String,
    type_: String,
    description: String,
    creation_date: String,
}

impl VehicleLog {
    fn update(&self) {
        // Implement update logic here
    }

    fn search_by_log_type(&self, type_: String) {
        // Implement search_by_log_type logic here
    }
}

struct VehicleReservation {
    reservation_number: String,
    creation_date: String,
    status: ReservationStatus,
    due_date: String,
    return_date: String,
    pickup_location_name: String,
    return_location_name: String,
    customer_id: u32,
    vehicle: Option<Vehicle>,
    bill: Option<Bill>,
    additional_drivers: Vec<Driver>,
    notifications: Vec<Notification>,
    insurances: Vec<Insurance>,
    equipments: Vec<Equipment>,
    services: Vec<Service>,
}

impl VehicleReservation {
    fn new(reservation_number: String) -> Self {
        VehicleReservation {
            reservation_number,
            creation_date: String::from(""),
            status: ReservationStatus::Active,
            due_date: String::from(""),
            return_date: String::from(""),
            pickup_location_name: String::from(""),
            return_location_name: String::from(""),
            customer_id: 0,
            vehicle: None,
            bill: None,
            additional_drivers: vec![],
            notifications: vec![],
            insurances: vec![],
            equipments: vec![],
            services: vec![],
        }
    }

    fn fetch_reservation_details(&self, reservation_number: String) {
        // Implement fetch_reservation_details logic here
    }

    fn get_additional_drivers(&self) -> &Vec<Driver> {
        &self.additional_drivers
    }
}

struct Driver {
    // Driver fields here
}

struct Bill {
    // Bill fields here
}

struct Notification {
    // Notification fields here
}

struct Insurance {
    // Insurance fields here
}

struct Equipment {
    // Equipment fields here
}

struct Service {
    // Service fields here
}


fn main () {

}