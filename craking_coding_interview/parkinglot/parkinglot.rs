#![allow(unused)]
use std::{collections::HashMap, marker::PhantomData};
use strum_macros::Display;


#[derive(Debug, Clone, Copy, Display)]
pub enum VehicleType {
    Car,
    Truck,
    Electric,
    Van,
    Motorbike,
}


#[derive(Debug, Clone, Copy, Display)]
pub enum ParkingSpotType {
    Handicapped,
    Compact,
    Large,
    Motorbike,
    Electric,
}

#[derive(Debug, Display)]
enum AccountStatus {
    Active,
    Blocked,
    Banned,
    Compromised,
    Archived,
    Unknown,
}


#[derive(Debug, Display)]
enum ParkingTicketStatus {
    Active,
    Paid,
    Lost,
}

#[derive(Debug)]
struct Address {
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    address: Address,
    email: String,
    phone: String,
}


#[derive(Debug)]
struct ParkingDisplayBoard<S, T> {
    id: u32,
    handicapped_free_spot: Option<S>,
    compact_free_spot: Option<S>,
    large_free_spot: Option<S>,
    motorbike_free_spot: Option<S>,
    electric_free_spot: Option<S>,
    // PhantomData is needed to perform type checking at compile time
    // These extra parameters hold no storage values, and have no runtime behavior.
    _marker: PhantomData<T>,
}

impl<S, T> ParkingDisplayBoard<S, T> 
    where 
    S: ParkingSpot<T>,
    T: Vehicle {
    
    fn new(id: u32) -> Self {
        ParkingDisplayBoard {
            id,
            handicapped_free_spot: None,
            compact_free_spot: None,
            large_free_spot: None,
            motorbike_free_spot: None,
            electric_free_spot: None,
            _marker: PhantomData,
        }
    }
    pub fn get_compact_free_spot(&self) -> &S {
        unimplemented!()
    }

    pub fn set_compact_free_spot(&mut self, spot: &S) {
        unimplemented!()
    }
    
    pub fn get_handicapped_free_spot(&self) -> &S {
        unimplemented!()
    }

    pub fn set_handicapped_free_spot(&mut self, spot: &S) {
        unimplemented!()
    }

    fn show_empty_spot_number(&self) {
        let mut message = String::new();

        if let Some(handicapped_spot) = &self.handicapped_free_spot {
            if handicapped_spot.is_free() {
                message += &format!("Free Handicapped: {}\n", handicapped_spot.get_number());
            } else {
                message += "Handicapped is full\n";
            }
        }

        if let Some(compact_spot) = &self.compact_free_spot {
            if compact_spot.is_free() {
                message += &format!("Free Compact: {}\n", compact_spot.get_number());
            } else {
                message += "Compact is full\n";
            }
        }

        if let Some(large_spot) = &self.large_free_spot {
            if large_spot.is_free() {
                message += &format!("Free Large: {}\n", large_spot.get_number());
            } else {
                message += "Large is full\n";
            }
        }

        if let Some(motorbike_spot) = &self.motorbike_free_spot {
            if motorbike_spot.is_free() {
                message += &format!("Free Motorbike: {}\n", motorbike_spot.get_number());
            } else {
                message += "Motorbike is full\n";
            }
        }

        if let Some(electric_spot) = &self.electric_free_spot {
            if electric_spot.is_free() {
                message += &format!("Free Electric: {}\n", electric_spot.get_number());
            } else {
                message += "Electric is full\n";
            }
        }

        println!("{}", message);
    }
}


struct Account {
    user_name: String,
    password: String,
    person: Person,
    status: AccountStatus,
}

impl Account {
    fn new(user_name: String, password: String, person: Person, status: AccountStatus) -> Self {
        Account {
            user_name,
            password,
            person,
            status,
        }
    }

    fn reset_password(&self) {
        // Implementation of resetting the password
        unimplemented!()
    }
}

struct Admin<S, T> {
    account: Account,
    _marker1: PhantomData<S>,
    _marker2: PhantomData<T>,
}

struct CustomerInfoPanel;

struct EntrancePanel;

struct ExitPanel;

impl<S, T> Admin<S, T> 
where 
    S: ParkingSpot<T>,
    T: Vehicle,
 {
    fn new(user_name: String, password: String, person: Person, status: AccountStatus) -> Self {
        Admin {
            account: Account::new(user_name, password, person, status),
            _marker1: PhantomData,
            _marker2: PhantomData,
        }
    }

    fn add_parking_floor(&self, floor: ParkingFloor<S, T>) {
        // Implementation of adding a parking floor
        unimplemented!()
    }

    fn add_parking_spot(&self, floor_name: String, spot: S) {
        // Implementation of adding a parking spot
        unimplemented!()
    }

    fn add_parking_display_board(&self, floor_name: String, display_board: ParkingDisplayBoard<S, T>) {
        // Implementation of adding a parking display board
        unimplemented!()
    }

    fn add_customer_info_panel(&self, floor_name: String, info_panel: CustomerInfoPanel) {
        // Implementation of adding a customer info panel
        unimplemented!()
    }

    fn add_entrance_panel(&self, entrance_panel: EntrancePanel) {
        // Implementation of adding an entrance panel
        unimplemented!()
    }

    fn add_exit_panel(&self, exit_panel: ExitPanel) {
        // Implementation of adding an exit panel
        unimplemented!()
    }
}

struct ParkingAttendant {
    account: Account,
}

impl ParkingAttendant {
    fn new(user_name: String, password: String, person: Person, status: AccountStatus) -> Self {
        ParkingAttendant {
            account: Account::new(user_name, password, person, status),
        }
    }

    fn process_ticket(&self, ticket_number: u32) {
        // Implementation of processing a ticket
        unimplemented!()
    }
}

struct InfoPortal;

struct ParkingFloor<S, T> {
    name: String,
    handicapped_spots: HashMap<u32, S>,
    compact_spots: HashMap<u32, S>,
    large_spots: HashMap<u32, S>,
    motorbike_spots: HashMap<u32, S>,
    electric_spots: HashMap<u32, S>,
    info_portals: HashMap<u32, InfoPortal>,
    free_handicapped_spot_count: u32,
    free_compact_spot_count: u32,
    free_large_spot_count: u32,
    free_motorbike_spot_count: u32,
    free_electric_spot_count: u32,
    display_board: ParkingDisplayBoard<S, T>,
    _marker: PhantomData<T>
}

impl<S, T> ParkingFloor<S, T> 
where 
    S: ParkingSpot<T>,
    T: Vehicle {
    fn new(name: String) -> Self {
        ParkingFloor {
            name,
            handicapped_spots: HashMap::new(),
            compact_spots: HashMap::new(),
            large_spots: HashMap::new(),
            motorbike_spots: HashMap::new(),
            electric_spots: HashMap::new(),
            info_portals: HashMap::new(),
            free_handicapped_spot_count: 0,
            free_compact_spot_count: 0,
            free_large_spot_count: 0,
            free_motorbike_spot_count: 0,
            free_electric_spot_count: 0,
            // Generate an id for parking display board
            display_board: ParkingDisplayBoard::new(0),
            _marker: PhantomData,
        }
    }

    fn add_parking_spot(&mut self, spot: S) {
        match spot.get_type() {
            ParkingSpotType::Handicapped => {
                self.handicapped_spots.insert(spot.get_number(), spot);
            }
            ParkingSpotType::Compact => {
                self.compact_spots.insert(spot.get_number(), spot);
            }
            ParkingSpotType::Large => {
                self.large_spots.insert(spot.get_number(), spot);
            }
            ParkingSpotType::Motorbike => {
                self.motorbike_spots.insert(spot.get_number(), spot);
            }
            ParkingSpotType::Electric => {
                self.electric_spots.insert(spot.get_number(), spot);
            }
        }
    }

    fn assign_vehicle_to_spot(&mut self, vehicle: T, spot: &mut S) {
        spot.assign_vehicle(vehicle);
        match spot.get_type() {
            ParkingSpotType::Handicapped => {
                self.update_display_board_for_handicapped(spot);
            }
            ParkingSpotType::Compact => {
                self.update_display_board_for_compact(spot);
            }
            ParkingSpotType::Large => {
                self.update_display_board_for_large(spot);
            }
            ParkingSpotType::Motorbike => {
                self.update_display_board_for_motorbike(spot);
            }
            ParkingSpotType::Electric => {
                self.update_display_board_for_electric(spot);
            }
        }
    }

    fn update_display_board_for_large(&mut self, spot: &S) {
        unimplemented!()
    }

    fn update_display_board_for_motorbike(&mut self, spot: &S) {
        unimplemented!()
    }

    fn update_display_board_for_electric(&mut self, spot: &S) {
        unimplemented!()
    }

    fn update_display_board_for_handicapped(&mut self, spot: &S) {
        if self.display_board.get_handicapped_free_spot().get_number() == spot.get_number() {
            // find another free handicapped parking and assign to display_board
            for (_, value) in &self.handicapped_spots {
                if value.is_free() {
                    self.display_board.set_handicapped_free_spot(value);
                    break;
                }
            }

            self.display_board.show_empty_spot_number();
        }
    }

    fn update_display_board_for_compact(&mut self, spot: &S) {
        if self.display_board.get_compact_free_spot().get_number() == spot.get_number() {
            // find another free compact parking and assign to display_board
            for (_, value) in &self.compact_spots {
                if value.is_free() {
                    self.display_board.set_compact_free_spot(value);
                    break;
                }
            }

            self.display_board.show_empty_spot_number();
        }
    }

    fn free_spot(&mut self, spot: &mut S) {
        spot.remove_vehicle();
        match spot.get_type() {
            ParkingSpotType::Handicapped => {
                self.free_handicapped_spot_count += 1;
            }
            ParkingSpotType::Compact => {
                self.free_compact_spot_count += 1;
            }
            ParkingSpotType::Large => {
                self.free_large_spot_count += 1;
            }
            ParkingSpotType::Motorbike => {
                self.free_motorbike_spot_count += 1;
            }
            ParkingSpotType::Electric => {
                self.free_electric_spot_count += 1;
            }
        }
    }
}

pub trait Vehicle {
    fn get_license_number(&self) -> &str;
    
    fn get_type(&self) -> VehicleType;
    
    fn get_ticket(&self) -> Option<&Ticket>;
    
    fn assign_ticket(&mut self, ticket: Ticket);
}

#[derive(Debug)]
pub struct Car {
    license_number: String,
    ticket: Option<Ticket>,
}

impl Car {
    pub fn new(license_number: String, ticket: Option<Ticket>) -> Self {
        Car {
            license_number,
            ticket,
        }
    }
}

impl Vehicle for Car {
    fn get_license_number(&self) -> &str {
        &self.license_number
    }

    fn get_type(&self) -> VehicleType {
        VehicleType::Car
    }

    fn get_ticket(&self) -> Option<&Ticket> {
        self.ticket.as_ref()
    }

    fn assign_ticket(&mut self, ticket: Ticket) {
        self.ticket = Some(ticket);
    }
}

#[derive(Debug)]
pub struct Van {
    license_number: String,
    ticket: Option<Ticket>,
}

impl Van {
    pub fn new(license_number: String, ticket: Option<Ticket>) -> Self {
        Van {
            license_number,
            ticket,
        }
    }
}

impl Vehicle for Van {
    fn get_license_number(&self) -> &str {
        &self.license_number
    }

    fn get_type(&self) -> VehicleType {
        VehicleType::Van
    }

    fn get_ticket(&self) -> Option<&Ticket> {
        self.ticket.as_ref()
    }

    fn assign_ticket(&mut self, ticket: Ticket) {
        self.ticket = Some(ticket);
    }
}

#[derive(Debug)]
pub struct Truck {
    license_number: String,
    ticket: Option<Ticket>,
}

impl Truck {
    pub fn new(license_number: String, ticket: Option<Ticket>) -> Self {
        Truck {
            license_number,
            ticket,
        }
    }
}

impl Vehicle for Truck {
    fn get_license_number(&self) -> &str {
        &self.license_number
    }

    fn get_type(&self) -> VehicleType {
        VehicleType::Truck
    }

    fn get_ticket(&self) -> Option<&Ticket> {
        self.ticket.as_ref()
    }

    fn assign_ticket(&mut self, ticket: Ticket) {
        self.ticket = Some(ticket);
    }
}

#[derive(Debug)]
pub struct Ticket {
    // Ticket details
}

pub trait ParkingSpot<T> {
    /// Implementation of checking if the spot is free
    fn is_free(&self) -> bool;
    
    fn assign_vehicle(&mut self, vehicle: T);
    
    fn remove_vehicle(&mut self);
    
    fn get_type(&self) -> ParkingSpotType;

    fn get_number(&self) -> u32;
}

#[derive(Debug, Clone)]
pub struct HandicappedSpot<T: Vehicle> {
    number: u32,
    free: bool,
    vehicle: Option<T>,
}

impl<T: Vehicle> HandicappedSpot<T> {
    pub fn new(number: u32) -> Self {
        HandicappedSpot {
            number,
            free: true,
            vehicle: None,
        }
    }
}

impl<T: Vehicle> ParkingSpot<T> for HandicappedSpot<T> {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn is_free(&self) -> bool {
        self.free
    }

    fn assign_vehicle(&mut self, vehicle: T) {
        self.vehicle = Some(vehicle);
        self.free = false;
    }

    fn remove_vehicle(&mut self) {
        self.vehicle = None;
        self.free = true;
    }

    fn get_type(&self) -> ParkingSpotType {
        ParkingSpotType::Handicapped
    }
}

#[derive(Debug, Clone)]
pub struct CompactSpot<T> {
    number: u32,
    free: bool,
    vehicle: Option<T>,
}

impl<T: Vehicle> CompactSpot<T> {
    pub fn new(number: u32) -> Self {
        CompactSpot {
            number,
            free: true,
            vehicle: None,
        }
    }
}

impl<T: Vehicle> ParkingSpot<T> for CompactSpot<T> {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn is_free(&self) -> bool {
        self.free
    }

    fn assign_vehicle(&mut self, vehicle: T) {
        self.vehicle = Some(vehicle);
        self.free = false;
    }

    fn remove_vehicle(&mut self) {
        self.vehicle = None;
        self.free = true;
    }

    fn get_type(&self) -> ParkingSpotType {
        ParkingSpotType::Compact
    }
}

#[derive(Debug, Clone)]
pub struct LargeSpot<T> {
    number: u32,
    free: bool,
    vehicle: Option<T>,
}

impl<T: Vehicle> LargeSpot<T> {
    pub fn new(number: u32) -> Self {
        LargeSpot {
            number,
            free: true,
            vehicle: None,
        }
    }
}


impl<T: Vehicle> ParkingSpot<T> for LargeSpot<T> {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn is_free(&self) -> bool {
        self.free
    }

    fn assign_vehicle(&mut self, vehicle: T) {
        self.vehicle = Some(vehicle);
        self.free = false;
    }

    fn remove_vehicle(&mut self) {
        self.vehicle = None;
        self.free = true;
    }

    fn get_type(&self) -> ParkingSpotType {
        ParkingSpotType::Large
    }
}

#[derive(Debug, Clone)]
pub struct MotorbikeSpot<T> {
    number: u32,
    free: bool,
    vehicle: Option<T>,
}

impl<T: Vehicle> MotorbikeSpot<T> {
    pub fn new(number: u32) -> Self {
        MotorbikeSpot {
            number,
            free: true,
            vehicle: None,
        }
    }
}

impl<T: Vehicle> ParkingSpot<T> for MotorbikeSpot<T> {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn is_free(&self) -> bool {
        self.free
    }

    fn assign_vehicle(&mut self, vehicle: T) {
        self.vehicle = Some(vehicle);
        self.free = false;
    }

    fn remove_vehicle(&mut self) {
        self.vehicle = None;
        self.free = true;
    }

    fn get_type(&self) -> ParkingSpotType {
        ParkingSpotType::Motorbike
    }
}

#[derive(Debug, Clone)]
pub struct ElectricSpot<T: Vehicle> {
    number: u32,
    free: bool,
    vehicle: Option<T>,
}

impl<T: Vehicle> ElectricSpot<T> {
    pub fn new(number: u32) -> Self {
        ElectricSpot {
            number,
            free: true,
            vehicle: None,
        }
    }
}

impl<T: Vehicle> ParkingSpot<T> for ElectricSpot<T> {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn is_free(&self) -> bool {
        self.free
    }

    fn assign_vehicle(&mut self, vehicle: T) {
        self.vehicle = Some(vehicle);
        self.free = false;
    }

    fn remove_vehicle(&mut self) {
        self.vehicle = None;
        self.free = true;
    }

    fn get_type(&self) -> ParkingSpotType {
        ParkingSpotType::Electric
    }
}


fn main() {
    let person = Person {
        name: String::from("John Doe"),
        address: Address {
            street_address: String::from("123 Main St"),
            city: String::from("City"),
            state: String::from("State"),
            zip_code: String::from("12345"),
            country: String::from("Country"),
        },
        email: String::from("john.doe@example.com"),
        phone: String::from("123-456-7890"),
    };

    // let admin = Admin::new(String::from("admin"), String::from("password"), person.clone(), AccountStatus::Active);
    // let parking_attendant = ParkingAttendant::new(String::from("attendant"), String::from("password"), person.clone(), AccountStatus::Active);
}
