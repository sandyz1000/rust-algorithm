//! How to handle concurrency; such that no two users are able to book the same seat?
//!
//! We can use transactions in SQL databases to avoid any clashes. For example, if we are using SQL server we can utilize 
//! Transaction Isolation Levels to lock the rows before we update them. Note: within a transaction, if we read rows we 
//! get a write-lock on them so that they can’t be updated by anyone else. Here is the sample code:
//!
//! SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;
//!
//! BEGIN TRANSACTION;
//!
//!     -- Suppose we intend to reserve three seats (IDs: 54, 55, 56) for ShowID=99 
//!     Select * From ShowSeat where ShowID=99 && ShowSeatID in (54, 55, 56) && isReserved=0 
//!
//!     -- if the number of rows returned by the above statement is NOT three, we can return failure to the user.
//!     update ShowSeat table...
//!     update Booking table ...
//!
//! COMMIT TRANSACTION;
//! ‘Serializable’ is the highest isolation level and guarantees safety from Dirty, Nonrepeatable, and Phantoms reads.
//!
//! Once the above database transaction is successful, we can safely assume that the reservation has been marked successfully 
//! and no two customers will be able to reserve the same seat.
#![allow(unused)]

use strum_macros::Display;
use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Debug, Display)]
enum BookingStatus {
    Requested,
    Pending,
    Confirmed,
    CheckedIn,
    Cancelled,
    Abandoned,
}


#[derive(Debug, Display, Clone)]
enum SeatType {
    Regular,
    Premium,
    Accessible,
    Shipped,
    EmergencyExit,
    Other,
}

enum AccountStatus {
    Active,
    Blocked,
    Banned,
    Compromised,
    Archived,
    Unknown,
}

#[derive(Debug, Display, Clone)]
enum PaymentStatus {
    UNpaid,
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

#[derive(Debug, Clone)]
struct City {
    name: String,
    state: String,
    zip_code: String
}

#[derive(Debug, Clone)]
struct Cinema {
    name: String,
    total_cinema_hall: i32,
    location: Address,
    halls: Vec<CinemaHall>,
}

#[derive(Debug, Clone)]
struct CinemaHall {
    name: String,
    total_seat: i32,
}

#[derive(Debug, Clone)]
struct CinemaHallSeat {
    seat_id: i32,
    seat_row: i32,
    seat_column: i32,
    seat_type: SeatType,
}

#[derive(Debug, Clone)]
struct ShowSeat {
    show_seat_id: String,
    is_reserved: bool,
    price: f64,
}

impl ShowSeat {
    fn new(id: String, is_reserved: bool, price: f64) -> Self {
        Self {
            show_seat_id: id,
            is_reserved,
            price,
        }
    }
}

#[derive(Debug, Clone)]
struct Address {
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

#[derive(Debug, Clone)]
struct Show {
    show_id: u32,
    created_on: chrono::DateTime<Utc>,
    start_time: chrono::DateTime<Utc>,
    end_time: chrono::DateTime<Utc>,
    played_at: String,
    movie: Movie,
}

#[derive(Debug, Clone)]
struct Movie {
    title: String,
    description: String,
    duration_in_mins: u32,
    language: String,
    release_date: chrono::DateTime<Utc>,
    country: String,
    genre: String,
    movie_added_by: String,
    shows: Vec<Show>,
}

impl Movie {
    fn new(
        title: String,
        description: String,
        duration_in_mins: u32,
        language: String,
        release_date: chrono::DateTime<Utc>,
        country: String,
        genre: String,
        movie_added_by: String,
    ) -> Self {
        Movie {
            title,
            description,
            duration_in_mins,
            language,
            release_date,
            country,
            genre,
            movie_added_by,
            shows: vec![],
        }
    }

    fn get_shows(&self) -> &Vec<Show> {
        &self.shows
    }
}


trait Search {
    fn search_by_title(&self, title: &str)  -> Vec<Movie>;
    fn search_by_language(&self, language: &str)  -> Vec<Movie>;
    fn search_by_genre(&self, genre: &str)  -> Vec<Movie>;
    fn search_by_release_date(&self, rel_date: &str) -> Vec<Movie>;
    fn search_by_city(&self, city_name: &str) -> Vec<Movie> ;
}

/// TODO: This should be mapping of movie titles to movie details 
struct Catalog {
    movie_titles: HashMap<String, Movie>,
    movie_languages: HashMap<String, Movie>,
    movie_genres: HashMap<String, Movie>,
    movie_release_dates: HashMap<String, Movie>,
    movie_cities: HashMap<String, String>,
}

impl Search for Catalog {
    fn search_by_title(&self, title: &str) -> Vec<Movie> {
        unimplemented!()
    }

    fn search_by_language(&self, language: &str) -> Vec<Movie> {
        unimplemented!()
    }

    fn search_by_genre(&self, genre: &str) -> Vec<Movie> {
        unimplemented!()
    }

    fn search_by_release_date(&self, rel_date: &str) -> Vec<Movie> {
        unimplemented!()
    }

    fn search_by_city(&self, city_name: &str) -> Vec<Movie> {
        unimplemented!()
    }
}

struct Account {
    id: String,
    password: String,
    status: AccountStatus,
}

impl Account {
    fn new(id: String, password: String, status: AccountStatus) -> Self {
        Self {
            id,
            password,
            status,
        }
    }

    fn reset_password(&self) {
        // Implement reset_password functionality
    }
}

trait Person {
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn email(&self) -> &str;
    fn phone(&self) -> &str;
    fn account(&self) -> &Account;
}

struct Customer {
    name: String,
    address: String,
    email: String,
    phone: String,
    account: Account,
}

impl Person for Customer {
    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn phone(&self) -> &str {
        &self.phone
    }

    fn account(&self) -> &Account {
        &self.account
    }
}

impl Customer {
    fn make_booking(&self, booking: Booking) -> bool {
        unimplemented!()
    }

    fn get_bookings(&self) -> Vec<Booking> {
        unimplemented!()
    }
}

struct Admin {
    name: String,
    address: String,
    email: String,
    phone: String,
    account: Account,
}

impl Person for Admin {
    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn phone(&self) -> &str {
        &self.phone
    }

    fn account(&self) -> &Account {
        &self.account
    }
}

impl Admin {
    fn add_movie(&self, movie: Movie) -> bool {
        unimplemented!()
    }

    fn add_show(&self, show: Show) -> Show {
        // Implement add_show functionality
        unimplemented!()
    }

    fn block_user(&self, customer: &Customer) -> bool {
        false
    }
}

struct FrontDeskOfficer {
    name: String,
    address: String,
    email: String,
    phone: String,
    account: Account,
}

impl Person for FrontDeskOfficer {
    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn phone(&self) -> &str {
        &self.phone
    }

    fn account(&self) -> &Account {
        &self.account
    }
}

impl FrontDeskOfficer {
    fn create_booking(&self, booking: Booking) -> bool {
        // Implement create_booking functionality
        true
    }
}

struct Guest {}

impl Guest {
    fn register_account(&self) -> bool {
        // Implement register_account functionality
        true
    }
}

struct Booking {
    booking_number: String,
    number_of_seats: i32,
    created_on: chrono::DateTime<Utc>,
    status: BookingStatus,
    show: Show,
    seats: Vec<ShowSeat>,
    payment: Payment,
}

impl Booking {
    fn new(
        booking_number: String,
        number_of_seats: i32,
        status: BookingStatus,
        show: Show,
        show_seats: Vec<ShowSeat>,
        payment: Payment,
    ) -> Self {
        Self {
            booking_number,
            number_of_seats,
            created_on: Utc::now(),
            status,
            show,
            seats: show_seats,
            payment,
        }
    }

    fn make_payment(&self, payment: Payment) {
        // Implement make_payment functionality
    }

    fn cancel(&self) -> bool {
        // Implement cancel functionality
        true
    }

    fn assign_seats(&self, seats: Vec<ShowSeat>) {
        // Implement assign_seats functionality
    }
}

#[derive(Debug, Clone)]
struct Coupon {
    id: i32,
    code: String,
    discount: f64,
    expiry_date: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct Payment {
    amount: f64,
    created_on: chrono::DateTime<Utc>,
    transaction_id: uuid::Uuid,
    payment_status: PaymentStatus,
}

impl Payment {
    fn new(amount: f64, transaction_id: uuid::Uuid, payment_status: PaymentStatus) -> Self {
        Self {
            amount,
            created_on: Utc::now(),
            transaction_id,
            payment_status,
        }
    }
}

#[derive(Debug, Clone)]
struct CreditCardTransaction {
    payment: Payment,
    name_on_card: String,
    card_number: String,
    expiration_date: String,
    cvv: String,
}

impl CreditCardTransaction {
    fn new(
        payment: Payment, name_on_card: String, card_number: String, expiration_date: String, cvv: String,
    ) -> Self {
        Self {
            payment,
            name_on_card,
            card_number,
            expiration_date,
            cvv,
        }
    }
}

#[derive(Debug, Clone)]
struct CashTransaction {
    payment: Payment,
    cash_amount: f64,
}

impl CashTransaction {
    fn new(payment: Payment, cash_amount: f64) -> Self {
        Self {
            payment,
            cash_amount,
        }
    }
}

#[derive(Debug, Clone)]
struct Notification {
    notification_id: i32,
    created_on: chrono::DateTime<Utc>,
    content: String,
}

impl Notification {
    fn send_notification(&self) -> bool {
        // Implement send_notification functionality
        true
    }
}

#[derive(Debug, Clone)]
struct EmailNotification {
    notification: Notification,
    email: String,
}

impl EmailNotification {
    fn send_email(&self) -> bool {
        // Implement send_email functionality
        true
    }
}

#[derive(Debug, Clone)]
struct SMSNotification {
    notification: Notification,
    phone: String,
}

impl SMSNotification {
    fn send_sms(&self) -> bool {
        // Implement send_sms functionality
        true
    }
}

fn main() {
    let release_date = Utc.with_ymd_and_hms(2014, 11, 7, 0, 0, 0).unwrap();
    let movie = Movie::new(
        "Interstellar".to_string(),
        "A team of explorers travel through a wormhole in space in an attempt to ensure humanity's survival.".to_string(),
        169,
        "English".to_string(),
        release_date,
        "United States".to_string(),
        "Science Fiction".to_string(),
        "John Doe".to_string(),
    );

    let shows = movie.get_shows();
    println!("Shows: {:?}", shows);
}
