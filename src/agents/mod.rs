use geometry::Space;


pub use self::control::ControlAgent;
pub use self::prediction::PredictionAgent;


pub mod control;
pub mod prediction;

mod trace;
pub use self::trace::Trace;
