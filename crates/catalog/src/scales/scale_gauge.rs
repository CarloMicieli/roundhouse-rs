use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Gauge {
    track_gauge: TrackGauge,
    millimeters: Decimal,
    inches: Decimal,
}

impl Gauge {
    pub fn new(
        track_gauge: TrackGauge,
        millimeters: Decimal,
        inches: Decimal,
    ) -> Self {
        Gauge {
            track_gauge,
            millimeters,
            inches,
        }
    }
}

/// In rail transport, track gauge is the distance between the two rails of a railway track.
/// All vehicles on a rail network must have wheel sets that are compatible with the track gauge.
///
/// Since many different track gauges exist worldwide, gauge differences often present a barrier to wider operation on
/// railway networks.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TrackGauge {
    /// In modern usage, the term "broad gauge" generally refers to track spaced significantly wider than
    /// 1,435 mm (4 ft 8+1⁄2 inches).
    ///
    /// Broad gauge is the dominant gauge in countries in Indian subcontinent, the former Soviet Union (CIS states,
    /// Baltic states, Georgia and Ukraine), Mongolia and Finland, Spain, Portugal, Argentina, Chile and Ireland.
    /// It is also use for the suburban railway systems in South Australia, and Victoria, Australia.
    Broad,

    /// The term "medium gauge" had different meanings throughout history, depending on the local dominant gauge in use.
    Medium,

    /// Very narrow gauges of under 2 feet (610 mm) were used for some industrial railways in space-restricted
    /// environments such as mines or farms. The French company Decauville developed 500 mm (19+3⁄4 in) and
    /// 400 mm (15+3⁄4 in) tracks, mainly for mines; Heywood developed 15 in (381 mm) gauge for estate railways.
    /// The most common minimum-gauges were 15 in (381 mm), 400 mm (15+3⁄4 in), 16 in (406 mm), 18 in (457 mm),
    /// 500 mm (19+3⁄4 in) or 20 in (508 mm).
    Minimum,

    /// In modern usage, the term "narrow gauge" generally refers to track spaced significantly narrower than 1,435 mm
    /// (4 ft 8+1⁄2 in).
    Narrow,

    /// In modern usage the term "standard gauge" refers to 1,435 mm (4 ft 8+1⁄2 inches).
    /// Standard gauge is dominant in a majority of countries, including those in North America, most of western Europe,
    /// North Africa and the Middle east, and in China.
    Standard,
}
