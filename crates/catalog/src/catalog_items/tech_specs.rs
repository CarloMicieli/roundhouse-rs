use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TechSpecs {
    minimum_radius: Radius,
    coupling: Coupling,
    flywheel_fitted: FeatureFlag,
    close_couplers: FeatureFlag,
    metal_body: FeatureFlag,
    interior_lights: FeatureFlag,
    lights: FeatureFlag,
    spring_buffers: FeatureFlag,
    digital_shunting_coupling: FeatureFlag,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Coupling {
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FeatureFlag {
    Yes,
    No,
}

/// Minimum drivable radius
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Radius(Decimal);
