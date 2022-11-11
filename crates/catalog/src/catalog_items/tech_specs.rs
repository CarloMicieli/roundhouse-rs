use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq, Clone, Default, Builder)]
#[builder(default)]
pub struct TechSpecs {
    minimum_radius: Option<Radius>,
    coupling: Coupling,
    flywheel_fitted: FeatureFlag,
    close_couplers: FeatureFlag,
    metal_body: FeatureFlag,
    interior_lights: FeatureFlag,
    lights: FeatureFlag,
    spring_buffers: FeatureFlag,
    digital_shunting_coupling: FeatureFlag,
}

impl TechSpecs {
    pub fn minimum_radius(&self) -> Option<Radius> {
        self.minimum_radius
    }

    pub fn coupling(&self) -> Coupling {
        self.coupling
    }

    pub fn flywheel_fitted(&self) -> FeatureFlag {
        self.flywheel_fitted
    }

    pub fn close_couplers(&self) -> FeatureFlag {
        self.close_couplers
    }

    pub fn metal_body(&self) -> FeatureFlag {
        self.metal_body
    }

    pub fn interior_lights(&self) -> FeatureFlag {
        self.interior_lights
    }

    pub fn lights(&self) -> FeatureFlag {
        self.lights
    }

    pub fn spring_buffers(&self) -> FeatureFlag {
        self.spring_buffers
    }

    pub fn digital_shunting_coupling(&self) -> FeatureFlag {
        self.digital_shunting_coupling
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Coupling {
    None,

    /// Receptacle for Replaceable Coupling Heads in Scales TT and N
    Nem355,

    /// Coupler Head for Scale N
    Nem356,

    /// Coupler Head for Scale N
    Nem357,

    /// Coupler Head for Scale TT
    Nem359,

    /// Standard Coupling for Scale H0
    Nem360,

    /// NEM shaft 362 with close coupling mechanism
    Nem362,

    /// Coupler Head for Scale 0
    Nem365,
}

impl Default for Coupling {
    fn default() -> Self {
        Coupling::None
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FeatureFlag {
    Yes,
    No,
}

impl Default for FeatureFlag {
    fn default() -> Self {
        FeatureFlag::No
    }
}

/// Minimum drivable radius
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Radius(Decimal);

impl Radius {
    pub fn new(value: f32) -> Option<Radius> {
        if value.is_sign_negative() {
            None
        } else {
            let v = Decimal::from_f32(value)?;
            Some(Radius(v))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod tech_specs {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_tech_specs() {
            let tech_specs = TechSpecsBuilder::default()
                .coupling(Coupling::Nem362)
                .minimum_radius(Some(Radius(dec!(360))))
                .build()
                .unwrap();

            assert_eq!(Coupling::Nem362, tech_specs.coupling());
            assert_eq!(Some(Radius(dec!(360))), tech_specs.minimum_radius());
        }
    }
}
