macro_rules! height {
    ($feet:literal feet $inches:literal inches) => {
        const { (($feet as f32) * 12.0 + ($inches as f32)) * 0.0254 }
    };
    ($feet:literal feet) => {
        height!($feet feet 0 inches)
    };
    ($inches:literal in) => {
        height!(0 feet $inches inches)
    };
}
