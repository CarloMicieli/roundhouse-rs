use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::length_over_buffer::LengthOverBuffer;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::tech_specs::TechSpecs;
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
        category: ElectricMultipleUnitType,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
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
        tech_specs: Option<TechSpecs>,
    },
    FreightCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        tech_specs: Option<TechSpecs>,
    },
    PassengerCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        tech_specs: Option<TechSpecs>,
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
        tech_specs: Option<TechSpecs>,
    },
}

impl RollingStock {
    pub fn epoch(&self) -> &Epoch {
        match self {
            RollingStock::ElectricMultipleUnit { epoch, .. } => &epoch,
            RollingStock::Locomotive { epoch, .. } => &epoch,
            RollingStock::FreightCar { epoch, .. } => &epoch,
            RollingStock::PassengerCar { epoch, .. } => &epoch,
            RollingStock::Railcar { epoch, .. } => &epoch,
        }
    }

    pub fn length_over_buffer(&self) -> Option<&LengthOverBuffer> {
        match self {
            RollingStock::ElectricMultipleUnit {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::Locomotive {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::FreightCar {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::PassengerCar {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::Railcar {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
        }
    }

    pub fn railway(&self) -> &Railway {
        match self {
            RollingStock::ElectricMultipleUnit { railway, .. } => &railway,
            RollingStock::Locomotive { railway, .. } => &railway,
            RollingStock::FreightCar { railway, .. } => &railway,
            RollingStock::PassengerCar { railway, .. } => &railway,
            RollingStock::Railcar { railway, .. } => &railway,
        }
    }

    pub fn road_number(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { road_number, .. } => road_number.as_deref(),
            RollingStock::Locomotive { road_number, .. } => Some(road_number),
            RollingStock::FreightCar { road_number, .. } => road_number.as_deref(),
            RollingStock::PassengerCar { road_number, .. } => road_number.as_deref(),
            RollingStock::Railcar { road_number, .. } => road_number.as_deref(),
        }
    }

    pub fn tech_specs(&self) -> Option<&TechSpecs> {
        match self {
            RollingStock::ElectricMultipleUnit { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::Locomotive { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::FreightCar { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::PassengerCar { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::Railcar { tech_specs, .. } => tech_specs.as_ref(),
        }
    }
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
