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


#[derive(Debug, Display, Clone)]
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

#[derive(Debug, Display, Clone)]
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

struct Person {
    name: String,
    address: Address,
    email: String,
    phone: String,
}

/// **Account, Member, Receptionist, and Additional Driver:** 
/// These classes represent different people that interact with our system:
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

/// **CarRentalSystem and CarRentalLocation:** These classes represent the top level classes:
struct CarRentalLocation {
    name: String,
    location: Address,
}

impl CarRentalLocation {
    fn new(name: String, location: Address) -> Self {
        CarRentalLocation {
            name,
            location,
        }
    }

    fn get_location(&self) -> &Address {
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
    fn search_by_type(&self, type_query: &str) -> Option<&VehicleE>;
    fn search_by_model(&self, model_query: &str) -> Option<&VehicleE>;
}

struct VehicleInventory {
    vehicle_types: HashMap<String, VehicleE>,
    vehicle_models: HashMap<String, VehicleE>,
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
    fn search_by_type(&self, type_query: &str) -> Option<&VehicleE> {
        self.vehicle_types.get(type_query)
    }

    fn search_by_model(&self, model_query: &str) -> Option<&VehicleE> {
        self.vehicle_models.get(model_query)
    }
}

struct Barcode {
    barcode: String,
    issued_at: chrono::DateTime<chrono::Utc>,
    active: bool,
}

impl Barcode {
    fn new(barcode: String, issued_at: chrono::DateTime<chrono::Utc>, active: bool) -> Self {
        Barcode {
            barcode, issued_at, active,
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}
struct BarcodeReader {
    id: String,
    registered_at: chrono::DateTime<chrono::Utc>,
    active: bool,
}

impl BarcodeReader {
    fn new(id: String, registered_at: chrono::DateTime<chrono::Utc>, active: bool) -> Self {
        BarcodeReader {
            id,
            registered_at,
            active,
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

/// **Vehicle, VehicleLog, and VehicleReservation:** 
/// To encapsulate a vehicle, log, and reservation. The VehicleReservation class will be responsible 
/// for processing the reservation and return of a vehicle:

#[derive(Debug, Clone)]
enum VehicleType {
    Truck,
    Van,
    Car,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
enum VehicleE {
    Car(Car),
    Van(Van),
    Truck(Truck),
}

impl VehicleE {
    fn reserve_vehicle(&self) -> bool {
        // Implement reserve_vehicle logic here
        false
    }

    fn return_vehicle(&self) -> bool {
        // Implement return_vehicle logic here
        false
    }
}

#[derive(Debug, Clone)]
struct Car {
    vehicle: Vehicle,
    vehicle_type: VehicleType,
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
        mileage: u32
    ) -> Self {
        Car {
            vehicle: Vehicle {
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
            },
            vehicle_type: VehicleType::Car,
        }
    }
}

#[derive(Debug, Clone)]
struct Van {
    vehicle: Vehicle,
    vehicle_type: VehicleType,
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
        mileage: u32
    ) -> Self {
        Van {
            vehicle: Vehicle {
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
            },
            vehicle_type: VehicleType::Van,
        }
    }
}

#[derive(Debug, Clone)]
struct Truck {
    vehicle: Vehicle,
    vehicle_type: VehicleType,
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
            vehicle: Vehicle {
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
            },
            vehicle_type: VehicleType::Truck,
        }
    }
}

#[derive(Debug, Clone)]
struct VehicleLog {
    id: String,
    vehicle_type: VehicleType,
    description: String,
    creation_date: String,
}

impl VehicleLog {
    fn update(&self) {
        // Implement update logic here
    }

    fn search_by_log_type(&self, vehicle_type: VehicleType) {
        // Implement search_by_log_type logic here
    }
}

#[derive(Debug, Clone)]
struct VehicleReservation {
    reservation_number: String,
    creation_date: chrono::NaiveDateTime,
    status: ReservationStatus,
    due_date: String,
    return_date: String,
    pickup_location_name: String,
    return_location_name: String,
    customer_id: u32,
    vehicle: Option<VehicleE>,
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
            creation_date: chrono::Utc::now().naive_utc(),
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

    fn fetch_reservation_details(&self, reservation_number: String) -> VehicleReservation {
        // Implement fetch_reservation_details logic here
        unimplemented!()
    }

    fn get_additional_drivers(&self) -> &Vec<Driver> {
        &self.additional_drivers
    }
}

#[derive(Debug, Clone)]
struct Payment {
    id: u32,
    amount: f32,
    status: PaymentStatus,
    creation_date: chrono::NaiveDateTime,
}

impl Payment {
    fn create_transaction(&self) -> bool {
        // Implement create_transaction logic here
        false
    }
}

struct CreditCardTransaction {
    id: u32,
    card_number: String,
    expiration_date: String,
    cvv: String,    
    payment: Payment,
}

struct CashTransaction {
    amount: f32,
    payment: Payment,
}

struct CheckTransaction {
    bank: String,
    check_number: String,
    payment: Payment,
}

#[derive(Debug, Clone)]
struct Driver {
    // Driver fields here
}

#[derive(Debug, Clone)]
struct Bill {
    // Bill fields here
}

#[derive(Debug, Clone)]
struct Notification {
    id: u32,
    message: String,
    creation_date: chrono::NaiveDateTime,
}

impl Notification {
    fn send_notification(&self) -> bool {
        false
    }
}
struct SMSNotification {
    notification: Notification,
    phone: String,
}

struct EmailNotification {
    notification: Notification,
    email: String,
}

#[derive(Debug, Clone)]
struct Insurance {
    id: u32,
    name: String,
}

impl Insurance {
    fn add_insurance(&self) -> bool {
        // Implement add_insurance logic here
        false
    }
}

struct RentalInsurance {
    insurance: Insurance,
}

struct PersonalInsurance {
    insurance: Insurance,
}

struct LiabilityInsurance {
    insurance: Insurance,
}

#[derive(Debug, Clone)]
struct Equipment {
    id: u32,
    name: String,
}

impl Equipment {
    fn new(id: u32, name: String) -> Self {
        Equipment {
            id, name
        }
    }

    fn add_equipment(&self) -> bool {
        // Implement add_equipment logic here
        false
    }
}

#[derive(Debug, Clone)]
struct Service {
    id: u32,
    name: String,
}

impl Service {
    fn new(id: u32) -> Self {
        Service {
            id, name: String::from(""),
        }
    }

    fn add_service(&self) -> bool {
        // Implement add_service logic here
        false
    }
}


fn main () {

}