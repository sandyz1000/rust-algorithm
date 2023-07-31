#![allow(unused)]

use strum_macros::Display;

#[derive(Debug, Display)]
enum BookingStatus {
    Requested,
    Pending,
    Confirmed,
    CheckedIn,
    Cancelled,
    Abandoned,
}


#[derive(Debug, Display)]
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

#[derive(Debug, Display)]
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
struct Address {
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

use chrono::prelude::*;

#[derive(Debug, Clone)]
struct Show {
    show_id: u32,
    created_on: chrono::DateTime<Utc>,
    start_time: String,
    end_time: String,
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

use std::collections::HashMap;

trait Search {
    fn search_by_title(&self, title: &str);
    fn search_by_language(&self, language: &str);
    fn search_by_genre(&self, genre: &str);
    fn search_by_release_date(&self, rel_date: &str);
    fn search_by_city(&self, city_name: &str);
}

struct Catalog {
    movie_titles: HashMap<String, String>,
    movie_languages: HashMap<String, String>,
    movie_genres: HashMap<String, String>,
    movie_release_dates: HashMap<String, String>,
    movie_cities: HashMap<String, String>,
}

impl Search for Catalog {
    fn search_by_title(&self, title: &str) {
        self.movie_titles.get(title);
    }

    fn search_by_language(&self, language: &str) {
        self.movie_languages.get(language);
    }

    fn search_by_genre(&self, genre: &str) {
        self.movie_genres.get(genre);
    }

    fn search_by_release_date(&self, rel_date: &str) {
        self.movie_release_dates.get(rel_date);
    }

    fn search_by_city(&self, city_name: &str) {
        self.movie_cities.get(city_name);
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
    fn make_booking(&self, booking: Booking) {
        // Implement make_booking functionality
    }

    fn get_bookings(&self) {
        // Implement get_bookings functionality
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
    fn add_movie(&self, movie: Movie) {
        // Implement add_movie functionality
    }

    fn add_show(&self, show: Show) {
        // Implement add_show functionality
    }

    fn block_user(&self, customer: &Customer) {
        // Implement block_user functionality
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
    fn create_booking(&self, booking: Booking) {
        // Implement create_booking functionality
    }
}

struct Guest {}

impl Guest {
    fn register_account(&self) {
        // Implement register_account functionality
    }
}

use chrono::prelude::*;

struct Booking {
    booking_number: String,
    number_of_seats: i32,
    created_on: chrono::DateTime<Utc>,
    status: String,
    show: Show,
    seats: Vec<ShowSeat>,
    payment: Payment,
}

impl Booking {
    fn new(
        booking_number: String,
        number_of_seats: i32,
        status: String,
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

    fn cancel(&self) {
        // Implement cancel functionality
    }

    fn assign_seats(&self, seats: Vec<ShowSeat>) {
        // Implement assign_seats functionality
    }
}

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

struct Payment {
    amount: f64,
    created_on: chrono::DateTime<Utc>,
    transaction_id: String,
    status: String,
}

impl Payment {
    fn new(amount: f64, transaction_id: String, payment_status: String) -> Self {
        Self {
            amount,
            created_on: Utc::now(),
            transaction_id,
            status: payment_status,
        }
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
