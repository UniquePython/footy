pub(crate) const CAMERA_VERTICAL_FOV: f32 = 60.0; // degrees

// Altitude range. Higher shows more pitch but makes everything smaller.
pub(crate) const CAMERA_MIN_ALTITUDE: f32 = 25.0; // meters
pub(crate) const CAMERA_MAX_ALTITUDE: f32 = 40.0; // meters
pub(crate) const CAMERA_INITIAL_ALTITUDE: f32 = 30.0; // meters

// Position spring Tune these by playing.
pub(crate) const CAMERA_SPRING_FREQUENCY: f32 = 4.0; // rad/s, higher = stiffer, chases harder
pub(crate) const CAMERA_SPRING_DAMPING: f32 = 0.85; // 1.0 = critically damped, lower = floatier

// Altitude spring: critically damped, so the camera never bobs up and down
pub(crate) const CAMERA_ALTITUDE_SPRING_FREQUENCY: f32 = 2.0; // rad/s
pub(crate) const CAMERA_ALTITUDE_SPRING_DAMPING: f32 = 1.0;

// The spring aims slightly ahead of each subject, which cancels most of its steady-state lag
pub(crate) const CAMERA_LOOKAHEAD_TIME: f32 = 0.3; // seconds of subject velocity
pub(crate) const CAMERA_MAX_LOOKAHEAD: f32 = 10.0; // meters

// Margins, as a fraction of the half-screen size
pub(crate) const CAMERA_FRAMING_MARGIN: f32 = 0.2; // targets keep subjects inside the central 80%
pub(crate) const CAMERA_PRIMARY_SAFETY_MARGIN: f32 = 0.08; // hard limit: the primary subject stays inside the central 92%

// Keeps the springs stable if a frame takes unusually long
pub(crate) const CAMERA_MAX_UPDATE_DT: f32 = 1.0 / 30.0; // seconds
