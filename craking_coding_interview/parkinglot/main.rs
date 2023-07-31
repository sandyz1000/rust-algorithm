#![allow(unused)]

//! Parking Lot
//! ----------
//!
//!
// Process Brief :
//
// This is a Parking lot system designed using object oriented principles in Python.
//
// The objects in my design are ParkingLot, Levels, Slots and Vehicle
//
// Following objects has been considered in the design:
//
// - ParkingLot: A parking lot is made up of 'n' number of levels/floors and 'm' number of slots per floor.
//
// - Levels: Each level is an independent entity with a floor number, its lanes and the slots within it. 
// The number of lanes are designed based on the number of slots. 10 slots make one lane.
//
// - Slots: The slots are considered as the independent entities to each other where in the type of the 
// vehicle is considered to fill the slot.
//
// - Vehicles: Object with plate no., company name and their type. A vehicle has the attributes of license 
// plate and the company it is from.
//
// I have considered Levels and Slots as entities that are independent so that any level can be added with 
// a desired number of spots later. Each time a vehicle comes in or goes out, a list of vehicles for the particular 
// company is updated. Also the available spots are updated in the particular level.
//
// Methods:
//
// - ParkVehicle: This operation inserts a vehicle accordingly, also takes care of what company vehicle it is.
//
// - Leave Operation: This operations exits a vehicle 'C' in a level 'm'.
//
// - CompanyParked: This operation allows the user to view the list of vehicles parked for a particular company.
//
use std::cmp::PartialEq;
use rand::prelude::*;
use std::collections::HashSet;


// Vehicle type enum for what all types of vehicles can come for parking
#[derive(PartialEq, Clone, Eq, Hash, Debug)]
enum VehicleType {
    CAR = 1,
    BIKE = 2,
    BUS = 3,
}

trait VehicleInterface {
    
}

// Vehicle struct for license plate, company name and their type
#[derive(Clone, Eq, Hash, Debug)]
struct Vehicle {
    license_plate: i32,
    company_name: String,
    type_of_vehicle: VehicleType,
}

impl Vehicle {
    fn new(license_plate: i32, company_name: String, type_of_vehicle: VehicleType) -> Self {
        Self { license_plate, company_name, type_of_vehicle }
    }
    fn get_type(&self) -> VehicleType {
        self.type_of_vehicle.clone()
    }
}

impl PartialEq for Vehicle {
    fn eq(&self, other: &Vehicle) -> bool {
        if self.license_plate != other.license_plate {
            return false;
        }
        if self.company_name != other.company_name {
            return false;
        }
        if self.type_of_vehicle != other.type_of_vehicle {
            return false;
        }
        true
    }
}

// Car struct inherited from Vehicle struct for license plate, company name and their type
struct Car {
    vehicle: Vehicle,
}

impl Car {
    fn new(license_plate: i32, company_name: String) -> Self {
        Self {
            vehicle: Vehicle {
                license_plate,
                company_name,
                type_of_vehicle: VehicleType::CAR,
            },
        }
    }
}

// Bike struct inherited from Vehicle struct for license plate, company name and their type
struct Bike {
    vehicle: Vehicle,
}

impl Bike {
    fn new(license_plate: i32, company_name: String) -> Self {
        Self {
            vehicle: Vehicle {
                license_plate,
                company_name,
                type_of_vehicle: VehicleType::BIKE,
            },
        }
    }
}

// Bus struct inherited from Vehicle struct for license plate, company name and their type
struct Bus {
    vehicle: Vehicle,
}

impl Bus {
    fn new(license_plate: i32, company_name: String) -> Self {
        Self {
            vehicle: Vehicle {
                license_plate,
                company_name,
                type_of_vehicle: VehicleType::BUS,
            },
        }
    }
}

/// Slots struct for lane, spot number, and type of vehicle
#[derive(Clone, Eq, Hash, PartialEq)]
struct Slots {
    lane: String,
    spot_number: u32,
    vehicle: Option<Vehicle>,
    type_of_vehicle: VehicleType,
}

impl Slots {
    fn new(lane: String, spot_number: u32, type_of_vehicle: VehicleType) -> Self {
        Self {
            lane,
            spot_number,
            vehicle: None,
            type_of_vehicle,
        }
    }

    fn is_available(&self) -> bool {
        self.vehicle.is_none()
    }

    fn park(&mut self, vehicle: Vehicle) -> bool {
        if vehicle.type_of_vehicle == self.type_of_vehicle {
            self.vehicle.replace(vehicle);
            true
        } else {
            false
        }
    }

    fn remove_vehicle(&mut self) -> Option<Vehicle> {
        self.vehicle.take()
    }

    fn get_vehicle(&self) -> Option<&Vehicle> {
        self.vehicle.as_ref()
    }
}

/// Levels struct for floor number and number of slots
struct Levels {
    floor_number: u32,
    spots_per_lane: u32,
    lanes: u32,
    parking_slots: HashSet<Slots>,
    available_spots: Vec<Slots>,
}

impl Levels {
    fn new(floor_number: u32, no_of_slots: u32) -> Self {
        let spots_per_lane = 10;
        let lanes = no_of_slots / spots_per_lane;
        let mut available_spots = Vec::with_capacity((lanes * spots_per_lane) as usize);

        // Check available spots in a lane
        for lane in 0..lanes {
            for i in 0..spots_per_lane {
                // We will randomly assign a type to each slot.
                let type_of_vehicle = match thread_rng().gen_range(1..=3) {
                    1 => VehicleType::CAR,
                    2 => VehicleType::BIKE,
                    3 => VehicleType::BUS,
                    _ => unreachable!(),
                };
                available_spots.push(Slots::new(lane.to_string(), i, type_of_vehicle));
            }
        }

        Self {
            floor_number,
            spots_per_lane,
            lanes,
            parking_slots: HashSet::new(),
            available_spots,
        }
    }

    /// Park vehicle is spot is available
    fn park(&mut self, vehicle: Vehicle) -> bool {
        // Create a immutable reference to the available spots
        let available_spots = self.available_spots.clone();
        for slot in self.available_spots.iter_mut() {
            if slot.park(vehicle.clone()) {
                self.parking_slots.insert(slot.clone());
                if let Some(index) = available_spots.iter().position(|s| *s == *slot) {
                    self.available_spots.remove(index);
                }
                return true;
            }
        }
        false
    }

    /// Remove vehicle from a spot
    fn remove(&mut self, vehicle: &Vehicle) -> bool {
        let mut found = false;
        let mut slots_to_remove = Vec::new();

        for slot in self.parking_slots.iter() {
            if let Some(v) = slot.get_vehicle() {
                if *v == *vehicle {
                    self.available_spots.push(slot.clone());
                    slots_to_remove.push(slot.clone());
                    found = true;
                    break;
                }
            }
        }

        // Remove slots from the HashSet
        for mut slot in slots_to_remove {
            self.parking_slots.remove(&slot);
            slot.remove_vehicle();
        }

        found
    }

    // Company name for the vehicle parked at the available spots
    fn company_parked(&self, company_name: &str) -> Vec<Vehicle> {
        let mut all_vehicles = Vec::new();
        for slot in self.parking_slots.iter() {
            if let Some(vehicle) = slot.get_vehicle() {
                if vehicle.company_name == company_name {
                    all_vehicles.push(vehicle.clone());
                }
            }
        }
        all_vehicles
    }
}

// A parking lot is made up of 'n' number of levels/floors and 'm' number of slots per floor.
struct ParkingLot {
    levels: Vec<Levels>,
}

impl ParkingLot {
    fn new(no_of_floor: u32, no_of_slots: u32) -> Self {
        let mut levels = Vec::with_capacity(no_of_floor as usize);
        for i in 0..no_of_floor {
            levels.push(Levels::new(i, no_of_slots));
        }
        Self { levels }
    }

    /// This operation inserts a vehicle accordingly, also takes care 
    /// of what company vehicle it is.
    fn park_vehicle(&mut self, vehicle: Vehicle) -> bool {
        for level in self.levels.iter_mut() {
            if level.park(vehicle.clone()) {
                return true;
            }
        }
        false
    }

    /// This operation exits a vehicle 'C' in a level 'm'.
    fn leave_operation(&mut self, vehicle: &Vehicle) -> bool {
        for level in self.levels.iter_mut() {
            if level.remove(vehicle){
                return true;
            }
        }
        false
    }

    /// This operation allows the user to view the list of vehicles parked 
    /// for a particular company.
    fn company_parked(&self, company_name: &str) -> Vec<Vehicle> {
        let mut all_vehicles = Vec::new();
        for level in self.levels.iter() {
            all_vehicles.extend(level.company_parked(company_name));
        }
        all_vehicles
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park() {
        let mut parking_lot_obj = ParkingLot::new(6, 30);
        let res2 = parking_lot_obj.park_vehicle(Vehicle::new(
            10, 
            "Amazon".to_string(), 
            VehicleType::CAR
        ));
        let res3 = parking_lot_obj.park_vehicle(Vehicle::new(
            20, 
            "Amazon".to_string(), 
            VehicleType::BIKE
        ));
        let res4 = parking_lot_obj.park_vehicle(Vehicle::new(
            30, 
            "Microsoft".to_string(), 
            VehicleType::BUS
        ));

        assert_eq!(res2, true);
        assert_eq!(res3, true);
        assert_eq!(res4, true);
    }

    #[test]
    fn test_leave_operation() {
        let mut parking_lot_obj = ParkingLot::new(6, 30);
        assert!(parking_lot_obj.park_vehicle(Vehicle::new(
            20, 
            "Google".to_string(), 
            VehicleType::CAR
        )));
        assert!(parking_lot_obj.leave_operation(&Vehicle::new(
            20, "Google".to_string(), VehicleType::CAR
        )));
        assert_eq!(parking_lot_obj.leave_operation(&Vehicle::new(
            20, "Google".to_string(), VehicleType::CAR
        )), false);
    }

    #[test]
    fn test_company_parked() {
        let mut parking_lot_obj = ParkingLot::new(6, 30);
        assert!(parking_lot_obj.park_vehicle(Vehicle::new(
            20, "Google".to_string(), VehicleType::CAR
        )));
        assert_eq!(parking_lot_obj.company_parked("Google"), vec![
            Vehicle::new(20, "Google".to_string(), VehicleType::CAR)
        ]);
        assert_eq!(parking_lot_obj.company_parked("Microsoft"), vec![]);
    }

    #[test]
    fn test_all() {
        let mut parking_lot_obj = ParkingLot::new(3, 10);
        // Atleast 1 parking spot for car.
        // First park a car, it should return true.
        assert!(parking_lot_obj.park_vehicle(Vehicle::new(
            10, "Google".to_string(), VehicleType::CAR
        )));
        // Get the list of cars, it should give one car we parked.
        assert_eq!(parking_lot_obj.company_parked("Google"), vec![
            Vehicle::new(10, "Google".to_string(), VehicleType::CAR)
        ]);
        // Remove that car successfully.
        assert!(parking_lot_obj.leave_operation(&Vehicle::new(
            10, "Google".to_string(), VehicleType::CAR
        )));
        // Now the list of cars should be empty.
        assert_eq!(parking_lot_obj.company_parked("Google"), vec![]);
    }
}

fn main() {
    // 
}