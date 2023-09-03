#![allow(unused)]

use std::cmp::Eq;
use std::fmt;
use std::marker::PhantomData;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ReturnStatus {
    Success,
    Fail,
    InsufficientFunds,
    InsufficientQuantity,
    NoStockPosition,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum OrderStatus {
    Open,
    Filled,
    PartiallyFilled,
    Cancelled,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TimeEnforcementType {
    GoodTillCancelled,
    FillOrKill,
    ImmediateOrCancel,
    OnTheOpen,
    OnTheClose,
}

#[derive(Debug, PartialEq, Eq)]
enum AccountStatus {
    Active,
    Closed,
    Cancelled,
    Blacklisted,
    None,
}

struct Location {
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

impl Location {
    fn new(street: String, city: String, state: String, zip_code: String, country: String) -> Self {
        Location {
            street_address: street,
            city,
            state,
            zip_code,
            country,
        }
    }
}

struct Constants {
    money_transfer_limit: i32,
}

impl Constants {
    fn new() -> Self {
        Constants {
            money_transfer_limit: 100000,
        }
    }
}

#[derive(Debug, Clone)]
struct OrderI {
    order_id: u32,
    quantity: i32,
    is_buy_order: bool,
    status: OrderStatus,
    time_enforcement: TimeEnforcementType,
    parts: Vec<Part>,
    creation_time: DateTime<Utc>,
}

trait Order {
    fn set_status(&mut self, status: OrderStatus) -> bool;
    fn save_in_db(&self) -> bool;
    fn add_order_parts(&mut self, parts: Vec<Part>);
    fn update_in_db(&mut self ) -> bool;
    fn get_order_id(&self) -> u32;
}

#[derive(Debug, Clone)]
struct Part {
    // Define the Part struct as needed
}

#[derive(Debug, Clone)]
struct StopLossOrder {
    order: OrderI,
    price_limit: f64,
}

#[derive(Debug, Clone)]
struct StopLimitOrder {
    order: OrderI,
    price_limit: f64,
}

#[derive(Debug, Clone)]
struct MarketOrder {
    order: OrderI,
    price_limit: f64,
}

#[derive(Debug, Clone)]
struct LimitOrder {
    order: OrderI,
    price_limit: f64,
}

impl Order for LimitOrder {
    fn get_order_id(&self) -> u32 {
        unimplemented!()
    }

    fn set_status(&mut self, status: OrderStatus) -> bool {
        self.order.status = status;
        true
    }

    fn save_in_db(&self) -> bool {
        // Implement the logic to save the order in the database
        false
    }

    fn add_order_parts(&mut self, parts: Vec<Part>) {
        self.order.parts.extend(parts);
    }

    fn update_in_db(&mut self ) -> bool {
        false
    }
}

impl LimitOrder {
    fn new(
        id: u32, 
        quantity: i32, 
        price_limit: f64,
        time_enforcement: TimeEnforcementType, 
    ) -> Self {
        LimitOrder {
            order: OrderI {
                order_id: id,
                quantity,
                is_buy_order: true,
                status: OrderStatus::Open,
                time_enforcement,
                parts: Vec::new(),
                creation_time: Utc::now(),
            },
            price_limit,
        }
    }
}

#[derive(Debug, Clone)]
struct StockInventory {
    inventory_name: String,
    last_updated_date: chrono::DateTime<Utc>
}

impl StockInventory {
    fn new(name: String, last_updated_date: chrono::DateTime<Utc>) -> Self {
        StockInventory {
            inventory_name: name,
            last_updated_date
        }
    }

    fn search_symbol(&self, symbol: &str) -> Stock {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct Stock {
    symbol: String,
    price: f64,
}

impl Stock {
    fn new(symbol: String, price: f64) -> Self {
        Stock {
            symbol, price
        }
    }

    fn get_price(&self) -> f64 {
        self.price
    }
}

#[derive(Debug, Clone)]
struct OrderPart {
    price: f64,
    quantity: i32,
    execution_date: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct StockExchange<O> {
    instance: Option<Box<OnlyOne<O>>>,
}

#[derive(Debug, Clone)]
struct OnlyOne<O> {
    _marker: PhantomData<O>,
    instance: StockExchange<O>
    // Define the OnlyOne struct as needed
}

impl<O: Order> StockExchange<O> {
    fn new() -> Self {
        StockExchange {
            instance: None
        }
    }

    fn get_instance(&self) -> &OnlyOne<O> {
        self.instance.as_ref().unwrap()
    }

    fn place_order(&self, order: &O) -> ReturnStatus {
        let return_status = self.get_instance().submit_order(order);
        return_status
    }
}

impl<O: Order> Default for StockExchange<O> {
    fn default() -> Self {
        StockExchange::new()
    }
}

impl<O: Order> OnlyOne<O> {
    fn new(instance: StockExchange<O>) -> Self {
        Self {
            instance,
            _marker: PhantomData,
            // Initialize the OnlyOne struct as needed
        }
    }

    fn submit_order(&self, order: &O) -> ReturnStatus {
        // Implement the logic to submit the order
        ReturnStatus::Success
    }
}

trait Account {
    fn reset_password(&self);
}

impl AccountStatus {
    const NONE: u32 = 1;
}

struct Admin {
    id: u32,
    password: String,
    name: String,
    address: String,
    email: String,
    phone: String,
    status: u32,
    available_funds_for_trading: f64,
    date_of_membership: chrono::DateTime<Utc>,
    stock_positions: HashMap<u32, StockPosition>,
    active_orders: HashMap<u32, LimitOrder>,
}

impl Account for Admin {
    fn reset_password(&self) {
        todo!()
    }
}

impl Admin {
    fn block_member(&self) -> bool {
        // Implement the logic to block the member
        false
    }

    fn unblock_member(&self) -> bool {
        // Implement the logic to unblock the member
        false
    }
}

#[derive(Debug, Clone)]
struct Member {
    id: u32,
    password: String,
    name: String,
    address: String,
    email: String,
    phone: String,
    status: u32,
    available_funds_for_trading: f64,
    date_of_membership: chrono::DateTime<Utc>,
    stock_positions: HashMap<u32, StockPosition>,
    active_orders: HashMap<u32, LimitOrder>,
}

impl Account for Member {
    fn reset_password(&self) {
        // Implement the logic to reset the password
    }
}

impl Member {
    fn new(id: u32, password: String, name: String, address: String, email: String, phone: String) -> Self {
        Member {
            id,
            password,
            name,
            address,
            email,
            phone,
            status: AccountStatus::NONE,
            available_funds_for_trading: 0.0,
            date_of_membership: chrono::Utc::now(),
            stock_positions: HashMap::new(),
            active_orders: HashMap::new(),
        }
    }

    fn place_sell_limit_order(&mut self, stock_id: u32, quantity: i32, limit_price: f64, enforcement_type: TimeEnforcementType) -> ReturnStatus {
        // check if member has this stock position
        if !self.stock_positions.contains_key(&stock_id) {
            return ReturnStatus::NoStockPosition;
        }

        let stock_position = self.stock_positions.get(&stock_id).unwrap();
        // check if the member has enough quantity available to sell
        if stock_position.get_quantity() < quantity {
            return ReturnStatus::InsufficientQuantity;
        }

        let mut order = LimitOrder::new(stock_id, quantity, limit_price, enforcement_type);
        order.order.is_buy_order = false;
        order.save_in_db();
        let stock_exchange = StockExchange::<LimitOrder>::new();
        let success = stock_exchange.place_order(&order);
        if success == ReturnStatus::Success {
            order.set_status(OrderStatus::Filled);
            order.save_in_db();
        } else {
            self.active_orders.insert(order.get_order_id(), order);
        }
        success
    }

    fn place_buy_limit_order(
        &mut self, 
        stock_id: u32, quantity: i32, 
        limit_price: f64, enforcement_type: TimeEnforcementType
    ) -> ReturnStatus {
        // check if the member has enough funds to buy this stock
        if self.available_funds_for_trading < quantity as f64 * limit_price {
            return ReturnStatus::InsufficientFunds;
        }

        let mut order = LimitOrder::new(stock_id, quantity, limit_price, enforcement_type);
        order.order.is_buy_order = true;
        order.save_in_db();
        let stock_exchange = StockExchange::new();
        let success = stock_exchange.place_order(&order);
        if success != ReturnStatus::Success {
            order.set_status(OrderStatus::Filled);
            order.save_in_db();
        } else {
            // self.active_orders.insert(order.get_order_id(), order);
        }
        success
    }

    fn callback_stock_exchange(&mut self, order_id: u32, order_parts: Vec<Part>, status: OrderStatus) {
        let order = self.active_orders.get_mut(&order_id).unwrap();
        order.add_order_parts(order_parts);
        order.set_status(status.clone());
        order.update_in_db();

        if status == OrderStatus::Filled || status == OrderStatus::Cancelled {
            self.active_orders.remove(&order_id);
        }
    }
}

#[derive(Debug, Clone)]
struct StockPosition {
    // Define the StockPosition struct as needed
    symbol: String,
    quantity: f64,
}

impl StockPosition {
    fn get_quantity(&self) -> i32 {
        unimplemented!()
    }
}

struct StockLot<O: Order> {
    lot_number: String,
    buying_order: O
}

impl <O: Order> StockLot<O> {
    fn new(lot_number: String, buying_order: O) -> Self {
        StockLot {
            lot_number, buying_order
        }
    }

    fn get_buying_price(&self) -> f64 {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct WatchList {
    name: String,
    stocks: Vec<Stock>,
}

#[derive(Debug, Clone)]
struct Statement {
    name: String,
    description: String
}

impl Statement {
    fn get_statement(&self) -> String {
        unimplemented!()
    }
}

struct QuarterlyStatement {
    statement: Statement,
    quarter_no: u32,
    year: u32,
}

struct AnnualStatement {
    statement: Statement,
    year: u32,
}

#[derive(Debug, Clone)]
struct TransferMoney {
    amount: f64,
    from_account_id: u32,
    to_account_id: u32,
    created_on: chrono::DateTime<Utc>,
}

struct ElectronicWireTransfer {
    transfer: TransferMoney,
    bank_name: String,
    wire_id: u64
}

struct CheckTransfer {
    transfer: TransferMoney,
    bank_name: String,
    check_number: String,
}

#[derive(Debug, Clone)]
struct Notification {
    id: u32,
    created_on: chrono::DateTime<Utc>,
    message: String,
}

impl Notification {
    fn send(&self) -> bool {
        false
    }
}

struct SMSNotification {
    _notification: Notification,
    phone_number: String,
}

struct EmailNotification {
    _notification: Notification,
    email_address: String,
}

struct PushNotification {
    _notification: Notification,
    phone_number: String,
}

fn main() {

}