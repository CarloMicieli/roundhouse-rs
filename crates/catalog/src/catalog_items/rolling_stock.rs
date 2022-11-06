use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::length_over_buffer::LengthOverBuffer;
use crate::catalog_items::service_level::ServiceLevel;
use crate::railways::railway_id::RailwayId;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum RollingStock {
    ElectricMultipleUnit {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<ElectricMultipleUnitType>,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
    Locomotive {
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
    FreightCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    },
    PassengerCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    },
    Railcar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<RailcarType>,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Railway {
    railway_id: RailwayId,
    name: String,
}

impl Railway {
    /// Creates a new railway with the given name.
    pub fn new(railway_id: RailwayId, name: &str) -> Self {
        Railway {
            railway_id,
            name: name.to_owned(),
        }
    }

    /// Returns this railway unique identifier
    pub fn id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// Returns this railway name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Railway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}
