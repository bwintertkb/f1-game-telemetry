use std::fmt::Debug;

use binread::{self, BinRead};
use serde::{Deserialize, Serialize};

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct CarStatusData {
    pub traction_control: u8, // Traction control - 0 = off, 1 = medium, 2 = full
    pub anti_lock_brakes: u8, // 0 (off) - 1 (on)
    pub fuel_mix: u8,         // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub front_brake_bias: u8, // Front brake bias (percentage)
    pub pit_limiter_status: u8, // Pit limiter status - 0 = off, 1 = on
    pub fuel_in_tank: f32,    // Current fuel mass
    pub fuel_capacity: f32,   // Fuel capacity
    pub fuel_remaining_laps: f32, // Fuel remaining in terms of laps (value on MFD)
    pub max_rpm: u16,         // Cars max RPM, point of rev limiter
    pub idle_rpm: u16,        // Cars idle RPM
    pub max_gears: u8,        // Maximum number of gears
    pub drs_allowed: u8,      // 0 = not allowed, 1 = allowed
    pub drs_activation_distance: u16, // 0 = DRS not available, non-zero - DRS will be available
    // in [X] metres
    pub actual_tyre_compound: u8, // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    pub visual_tyre_compound: u8, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic – same as above
    // F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
    // 21 = medium , 22 = hard
    pub tyres_age_laps: u8,    // Age in laps of the current set of tyres
    pub vehicle_fia_flags: i8, // -1 = invalid/unknown, 0 = none, 1 = green
    // 2 = blue, 3 = yellow, 4 = red
    pub ers_store_energy: f32, // ERS energy store in Joules
    pub ers_deploy_mode: u8,   // ERS deployment mode, 0 = none, 1 = medium
    // 2 = hotlap, 3 = overtake
    pub ers_harvested_this_lap_mguk: f32, // ERS energy harvested this lap by MGU-K
    pub ers_harvested_this_lap_mguh: f32, // ERS energy harvested this lap by MGU-H
    pub ers_deployed_this_lap: f32,       // ERS energy deployed this lap
    pub network_paused: u8,               // Whether the car is paused in a network game
}

impl Default for CarStatusData {
    fn default() -> Self {
        CarStatusData {
            traction_control: 0,
            anti_lock_brakes: 0,
            fuel_mix: 0,
            front_brake_bias: 0,
            pit_limiter_status: 0,
            fuel_in_tank: 0.0,
            fuel_capacity: 0.0,
            fuel_remaining_laps: 0.0,
            max_rpm: 0,
            idle_rpm: 0,
            max_gears: 0,
            drs_allowed: 0,
            drs_activation_distance: 0,
            actual_tyre_compound: 0,
            visual_tyre_compound: 0,
            tyres_age_laps: 0,
            vehicle_fia_flags: 0,
            ers_store_energy: 0.0,
            ers_deploy_mode: 0,
            ers_harvested_this_lap_mguk: 0.0,
            ers_harvested_this_lap_mguh: 0.0,
            ers_deployed_this_lap: 0.0,
            network_paused: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketCarStatusData {
    pub header: PacketHeader, // Header
    pub car_status_data: [CarStatusData; 22],
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct CarMotionData {
    pub world_position_x: f32,
    pub world_position_y: f32,
    pub world_position_z: f32,
    pub world_velocity_x: f32,
    pub world_velocity_y: f32,
    pub world_velocity_z: f32,
    pub world_forward_dir_x: u16,
    pub world_forward_dir_y: u16,
    pub world_forward_dir_z: u16,
    pub world_right_dir_x: u16,
    pub world_right_dir_y: u16,
    pub world_right_dir_z: u16,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Default for CarMotionData {
    fn default() -> Self {
        CarMotionData {
            world_position_x: 0.0,
            world_position_y: 0.0,
            world_position_z: 0.0,
            world_velocity_x: 0.0,
            world_velocity_y: 0.0,
            world_velocity_z: 0.0,
            world_forward_dir_x: 0,
            world_forward_dir_y: 0,
            world_forward_dir_z: 0,
            world_right_dir_x: 0,
            world_right_dir_y: 0,
            world_right_dir_z: 0,
            g_force_lateral: 0.0,
            g_force_longitudinal: 0.0,
            g_force_vertical: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketMotionData {
    pub header: PacketHeader, // Header

    pub car_motion_data: [CarMotionData; 22], // Data for all cars on track

    // Extra player car ONLY data
    pub suspension_position: [f32; 4], // Note: All wheel arrays have the following order:
    pub suspension_velocity: [f32; 4], // RL, RR, FL, FR
    pub suspension_acceleration: [f32; 4], // RL, RR, FL, FR
    pub wheel_speed: [f32; 4],         // Speed of each wheel
    pub wheel_slip: [f32; 4],          // Slip ratio for each wheel
    pub local_velocity_x: f32,         // Velocity in local space
    pub local_velocity_y: f32,         // Velocity in local space
    pub local_velocity_z: f32,         // Velocity in local space
    pub angular_velocity_x: f32,       // Angular velocity x-component
    pub angular_velocity_y: f32,       // Angular velocity y-component
    pub angular_velocity_z: f32,       // Angular velocity z-component
    pub angular_acceleration_x: f32,   // Angular velocity x-component
    pub angular_acceleration_y: f32,   // Angular velocity y-component
    pub angular_acceleration_z: f32,   // Angular velocity z-component
    pub front_wheels_angle: f32,       // Current front wheels angle in radians
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct FinalClassificationData {
    pub position: u8,      // Finishing position
    pub num_laps: u8,      // Number of laps completed
    pub grid_position: u8, // Grid position of the car
    pub points: u8,        // Number of points scored
    pub num_pit_stops: u8, // Number of pit stops made
    pub result_status: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    pub best_lap_time_in_ms: u32, // Best lap time of the session in milliseconds
    pub total_race_time: f64,     // Total race time in seconds without penalties
    pub penalties_time: u8,       // Total penalties accumulated in seconds
    pub num_penalties: u8,        // Number of penalties applied to this driver
    pub num_tyre_stints: u8,      // Number of tyres stints up to maximum
    pub tyre_stints_actual: [u8; 8], // Actual tyres used by this driver
    pub tyre_stints_visual: [u8; 8], // Visual tyres used by this driver
    pub tyre_stints_end_laps: [u8; 8], // The lap number stints end on
}

impl Default for FinalClassificationData {
    fn default() -> Self {
        FinalClassificationData {
            position: 0,
            num_laps: 0,
            grid_position: 0,
            points: 0,
            num_pit_stops: 0,
            result_status: 0,
            best_lap_time_in_ms: 0,
            total_race_time: 0.0,
            penalties_time: 0,
            num_penalties: 0,
            num_tyre_stints: 0,
            tyre_stints_actual: [0; 8],
            tyre_stints_visual: [0; 8],
            tyre_stints_end_laps: [0; 8],
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,
    pub num_cars: u8,
    pub classification_data: [FinalClassificationData; 22],
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct MarshalZone {
    pub zone_start: f32, // Fraction (0..1) of way through the lap the marshal zone starts
    pub zone_flag: i8,   // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}

impl Default for MarshalZone {
    fn default() -> Self {
        MarshalZone {
            zone_start: 0.0,
            zone_flag: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct WeatherForecastSample {
    pub session_type: u8, // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P, 5 = Q1
    // 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2
    // 12 = R3, 13 = Time Trial
    pub time_offset: u8, // Time in minutes the forecast is for
    pub weather: u8,     // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8,        // Track temp. in degrees Celsius
    pub track_temperature_change: i8, // Track temp. change – 0 = up, 1 = down, 2 = no change
    pub air_temperature: i8,          // Air temp. in degrees celsius
    pub air_temperature_change: i8,   // Air temp. change – 0 = up, 1 = down, 2 = no change
    pub rain_percentage: u8,          // Rain percentage (0-100)
}

impl Default for WeatherForecastSample {
    fn default() -> Self {
        WeatherForecastSample {
            session_type: 0,
            time_offset: 0,
            weather: 0,
            track_temperature: 0,
            track_temperature_change: 0,
            air_temperature: 0,
            air_temperature_change: 0,
            rain_percentage: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketSessionData {
    pub header: PacketHeader, // Header

    pub weather: u8, // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8, // Track temp. in degrees celsius
    pub air_temperature: i8,   // Air temp. in degrees celsius
    pub total_laps: u8,        // Total number of laps in this race
    pub track_length: u16,     // Track length in metres
    pub session_type: u8,      // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    pub track_id: i8, // -1 for unknown, see appendix
    pub formula: u8,  // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    // 3 = F1 Generic, 4 = Beta, 5 = Supercars
    // 6 = Esports, 7 = F2 2021
    pub session_time_left: u16,     // Time left in session in seconds
    pub session_duration: u16,      // Session duration in seconds
    pub pit_speed_limit: u8,        // Pit speed limit in kilometres per hour
    pub game_paused: u8,            // Whether the game is paused – network game only
    pub is_spectating: u8,          // Whether the player is spectating
    pub spectator_car_index: u8,    // Index of the car being spectated
    pub sli_pro_native_support: u8, // SLI Pro support, 0 = inactive, 1 = active
    pub num_marshal_zones: u8,      // Number of marshal zones to follow
    pub marshal_zones: [MarshalZone; 21], // List of marshal zones – max 21
    pub safety_car_status: u8,      // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    pub network_game: u8,                 // 0 = offline, 1 = online
    pub num_weather_forecast_samples: u8, // Number of weather samples to follow
    pub weather_forecast_samples: [WeatherForecastSample; 21], // Array of weather forecast samples
    pub forecast_accuracy: u8,            // 0 = Perfect, 1 = Approximate
    pub ai_difficulty: u8,                // AI Difficulty rating – 0-110
    pub season_link_identifier: u32,      // Identifier for season - persists across saves
    pub weekend_link_identifier: u32,     // Identifier for weekend - persists across saves
    pub session_link_identifier: u32,     // Identifier for session - persists across saves
    pub pit_stop_window_ideal_lap: u8,    // Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8,   // Latest lap to pit on for current strategy (player)
    pub pit_stop_rejoin_position: u8,     // Predicted position to rejoin at (player)
    pub steering_assist: u8,              // 0 = off, 1 = on
    pub braking_assist: u8,               // 0 = off, 1 = low, 2 = medium, 3 = high
    pub gearbox_assist: u8,               // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub pit_assist: u8,                   // 0 = off, 1 = on
    pub pit_release_assist: u8,           // 0 = off, 1 = on
    pub ers_assist: u8,                   // 0 = off, 1 = on
    pub drs_assist: u8,                   // 0 = off, 1 = on
    pub dynamic_racing_line: u8,          // 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line_type: u8,     // 0 = 2D, 1 = 3D
    pub game_mode: u8,                    // Game mode id - see appendix
    pub rule_set: u8,                     // Ruleset - see appendix
    pub time_of_day: u32,                 // Local time of day - minutes since midnight
    pub session_length: u8,               // 0 = None, 2 = Very Short, 3 = Short, 4 = Medium
                                          // 5 = Medium Long, 6 = Long, 7 = Full
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,    // Last lap time in milliseconds
    pub current_lap_time_in_ms: u32, // Current time around the lap in milliseconds
    pub sector1_time_in_ms: u16,     // Sector 1 time in milliseconds
    pub sector2_time_in_ms: u16,     // Sector 2 time in milliseconds
    pub lap_distance: f32,           // Distance vehicle is around current lap in metres – could
    // be negative if line hasn’t been crossed yet
    pub total_distance: f32, // Total distance travelled in session in metres – could
    // be negative if line hasn’t been crossed yet
    pub safety_car_delta: f32,   // Delta in seconds for safety car
    pub car_position: u8,        // Car race position
    pub current_lap_num: u8,     // Current lap number
    pub pit_status: u8,          // 0 = none, 1 = pitting, 2 = in pit area
    pub num_pit_stops: u8,       // Number of pit stops taken in this race
    pub sector: u8,              // 0 = sector1, 1 = sector2, 2 = sector3
    pub current_lap_invalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    pub penalties: u8,           // Accumulated time penalties in seconds to be added
    pub warnings: u8,            // Accumulated number of warnings issued
    pub num_unserved_drive_through_pens: u8, // Num drive through pens left to serve
    pub num_unserved_stop_go_pens: u8, // Num stop go pens left to serve
    pub grid_position: u8,       // Grid position the vehicle started the race in
    pub driver_status: u8,       // Status of driver - 0 = in garage, 1 = flying lap
    // 2 = in lap, 3 = out lap, 4 = on track
    pub result_status: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    pub pit_lane_timer_active: u8, // Pit lane timing, 0 = inactive, 1 = active
    pub pit_lane_time_in_lane_in_ms: u16, // If active, the current time spent in the pit lane in ms
    pub pit_stop_timer_in_ms: u16, // Time of the actual pit stop in ms
    pub pit_stop_should_serve_pen: u8, // Whether the car should serve a penalty at this stop
}

impl Default for LapData {
    fn default() -> Self {
        LapData {
            last_lap_time_in_ms: 0,
            current_lap_time_in_ms: 0,
            sector1_time_in_ms: 0,
            sector2_time_in_ms: 0,
            lap_distance: 0.0,
            total_distance: 0.0,
            safety_car_delta: 0.0,
            car_position: 0,
            current_lap_num: 0,
            pit_status: 0,
            num_pit_stops: 0,
            sector: 0,
            current_lap_invalid: 0,
            penalties: 0,
            warnings: 0,
            num_unserved_drive_through_pens: 0,
            num_unserved_stop_go_pens: 0,
            grid_position: 0,
            driver_status: 0,
            result_status: 0,
            pit_lane_timer_active: 0,
            pit_lane_time_in_lane_in_ms: 0,
            pit_stop_timer_in_ms: 0,
            pit_stop_should_serve_pen: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketLapData {
    pub header: PacketHeader,         // Header
    pub lap_data: [LapData; 22],      // Lap data for all cars on track
    pub time_trial_pbcar_idx: u8,     // Index of Personal Best car in time trial (255 if invalid)
    pub time_trial_rival_car_idx: u8, // Index of Rival car in time trial (255 if invalid)
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct ParticipantData {
    pub ai_controlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub driver_id: u8,     // Driver id - see appendix, 255 if network human
    pub network_id: u8,    // Network id – unique identifier for network players
    pub team_id: u8,       // Team id - see appendix
    pub my_team: u8,       // My team flag – 1 = My Team, 0 = otherwise
    pub race_number: u8,   // Race number of the car
    pub nationality: u8,   // Nationality of the driver
    #[br(little, count = 48)]
    pub name: Vec<char>, // Name of participant in UTF-8 format – null terminated
    // Will be truncated with … (U+2026) if too long
    pub your_telemetry: u8, // The player's UDP setting, 0 = restricted, 1 = public
}

impl Default for ParticipantData {
    fn default() -> Self {
        ParticipantData {
            ai_controlled: 0,
            driver_id: 0,
            network_id: 0,
            team_id: 0,
            my_team: 0,
            race_number: 0,
            nationality: 0,
            name: Vec::with_capacity(48),
            your_telemetry: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketParticipantsData {
    pub header: PacketHeader, // Header
    pub num_active_cars: u8,  // Number of active cars in the data – should match number of
    // cars on HUD
    pub participants: [ParticipantData; 22],
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct CarSetupData {
    pub front_wing: u8,                 // Front wing aero
    pub rear_wing: u8,                  // Rear wing aero
    pub on_throttle: u8,                // Differential adjustment on throttle (percentage)
    pub off_throttle: u8,               // Differential adjustment off throttle (percentage)
    pub front_camber: f32,              // Front camber angle (suspension geometry)
    pub rear_camber: f32,               // Rear camber angle (suspension geometry)
    pub front_toe: f32,                 // Front toe angle (suspension geometry)
    pub rear_toe: f32,                  // Rear toe angle (suspension geometry)
    pub front_suspension: u8,           // Front suspension
    pub rear_suspension: u8,            // Rear suspension
    pub front_anti_roll_bar: u8,        // Front anti-roll bar
    pub rear_anti_roll_bar: u8,         // Front anti-roll bar
    pub front_suspension_height: u8,    // Front ride height
    pub rear_suspension_height: u8,     // Rear ride height
    pub brake_pressure: u8,             // Brake pressure (percentage)
    pub brake_bias: u8,                 // Brake bias (percentage)
    pub rear_left_tyre_pressure: f32,   // Rear left tyre pressure (PSI)
    pub rear_right_tyre_pressure: f32,  // Rear right tyre pressure (PSI)
    pub front_left_tyre_pressure: f32,  // Front left tyre pressure (PSI)
    pub front_right_tyre_pressure: f32, // Front right tyre pressure (PSI)
    pub ballast: u8,                    // Ballast
    pub fuel_load: f32,                 // Fuel load
}

impl Default for CarSetupData {
    fn default() -> Self {
        CarSetupData {
            front_wing: 0,
            rear_wing: 0,
            on_throttle: 0,
            off_throttle: 0,
            front_camber: 0.0,
            rear_camber: 0.0,
            front_toe: 0.0,
            rear_toe: 0.0,
            front_suspension: 0,
            rear_suspension: 0,
            front_anti_roll_bar: 0,
            rear_anti_roll_bar: 0,
            front_suspension_height: 0,
            rear_suspension_height: 0,
            brake_pressure: 0,
            brake_bias: 0,
            rear_left_tyre_pressure: 0.0,
            rear_right_tyre_pressure: 0.0,
            front_left_tyre_pressure: 0.0,
            front_right_tyre_pressure: 0.0,
            ballast: 0,
            fuel_load: 0.0,
        }
    }
}
#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,
    pub car_setups: [CarSetupData; 22],
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct CarTelemetryData {
    pub speed: u16,                         // Speed of car in kilometres per hour
    pub throttle: f32,                      // Amount of throttle applied (0.0 to 1.0)
    pub steer: f32,      // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    pub brake: f32,      // Amount of brake applied (0.0 to 1.0)
    pub clutch: u8,      // Amount of clutch applied (0 to 100)
    pub gear: i8,        // Gear selected (1-8, N=0, R=-1)
    pub engine_rpm: u16, // Engine RPM
    pub drs: u8,         // 0 = off, 1 = on
    pub rev_lights_percent: u8, // Rev lights indicator (percentage)
    pub rev_lights_bit_value: u16, // Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    pub brakes_temperature: [u16; 4], // Brakes temperature (celsius)
    pub tyres_surface_temperature: [u8; 4], // Tyres surface temperature (celsius)
    pub tyres_inner_temperature: [u8; 4], // Tyres inner temperature (celsius)
    pub engine_temperature: u16, // Engine temperature (celsius)
    pub tyres_pressure: [f32; 4], // Tyres pressure (PSI)
    pub surface_type: [u8; 4], // Driving surface, see appendices
}

impl Default for CarTelemetryData {
    fn default() -> Self {
        CarTelemetryData {
            speed: 0,
            throttle: 0.0,
            steer: 0.0,
            brake: 0.0,
            clutch: 0,
            gear: 0,
            engine_rpm: 0,
            drs: 0,
            rev_lights_percent: 0,
            rev_lights_bit_value: 0,
            brakes_temperature: [0; 4],
            tyres_surface_temperature: [0; 4],
            tyres_inner_temperature: [0; 4],
            engine_temperature: 0,
            tyres_pressure: [0.0; 4],
            surface_type: [0; 4],
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader,
    pub car_telemetry_data: [CarTelemetryData; 22],
    pub mfd_panel_index: u8, // Index of MFD panel open - 255 = MFD closed
    // Single player, race – 0 = Car setup, 1 = Pits
    // 2 = Damage, 3 =  Engine, 4 = Temperatures
    // May vary depending on game mode
    pub mfd_panel_index_secondary_player: u8, // See above
    pub suggested_gear: i8,                   // Suggested gear for the player (1-8)
                                              // 0 if no gear suggested
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct LobbyInfoData {
    pub ai_controlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub team_id: u8,       // Team id - see appendix (255 if no team currently selected)
    pub nationality: u8,   // Nationality of the driver
    #[br(little, count = 48)]
    pub name: Vec<char>, // Name of participant in UTF-8 format – null terminated
    // Will be truncated with ... (U+2026) if too long
    pub car_number: u8,   // Car number of the player
    pub ready_status: u8, // 0 = not ready, 1 = ready, 2 = spectating
}

impl Default for LobbyInfoData {
    fn default() -> Self {
        LobbyInfoData {
            ai_controlled: 0,
            team_id: 0,
            nationality: 0,
            name: Vec::with_capacity(48),
            car_number: 0,
            ready_status: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketLobbyInfoData {
    pub header: PacketHeader,
    pub num_players: u8,
    pub lobby_players: [LobbyInfoData; 22],
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct CarDamageData {
    pub tyres_wear: [f32; 4],        // Tyre wear (percentage)
    pub tyres_damage: [u8; 4],       // Tyre damage (percentage)
    pub brakes_damage: [u8; 4],      // Brakes damage (percentage)
    pub front_left_wing_damage: u8,  // Front left wing damage (percentage)
    pub front_right_wing_damage: u8, // Front right wing damage (percentage)
    pub rear_wing_damage: u8,        // Rear wing damage (percentage)
    pub floor_damage: u8,            // Floor damage (percentage)
    pub diffuser_damage: u8,         // Diffuser damage (percentage)
    pub sidepod_damage: u8,          // Sidepod damage (percentage)
    pub drs_fault: u8,               // Indicator for DRS fault, 0 = OK, 1 = fault
    pub ers_fault: u8,               // Indicator for ERS fault, 0 = OK, 1 = fault
    pub gear_box_damage: u8,         // Gear box damage (percentage)
    pub engine_damage: u8,           // Engine damage (percentage)
    pub engine_mguhwear: u8,         // Engine wear MGU-H (percentage)
    pub engine_eswear: u8,           // Engine wear ES (percentage)
    pub engine_cewear: u8,           // Engine wear CE (percentage)
    pub engine_icewear: u8,          // Engine wear ICE (percentage)
    pub engine_mgukwear: u8,         // Engine wear MGU-K (percentage)
    pub engine_tcwear: u8,           // Engine wear TC (percentage)
    pub engine_blown: u8,            // Engine blown, 0 = OK, 1 = fault
    pub engine_seized: u8,           // Engine seized, 0 = OK, 1 = fault
}
impl Default for CarDamageData {
    fn default() -> Self {
        CarDamageData {
            tyres_wear: [0.0; 4],
            tyres_damage: [0; 4],
            brakes_damage: [0; 4],
            front_left_wing_damage: 0,
            front_right_wing_damage: 0,
            rear_wing_damage: 0,
            floor_damage: 0,
            diffuser_damage: 0,
            sidepod_damage: 0,
            drs_fault: 0,
            ers_fault: 0,
            gear_box_damage: 0,
            engine_damage: 0,
            engine_mguhwear: 0,
            engine_eswear: 0,
            engine_cewear: 0,
            engine_icewear: 0,
            engine_mgukwear: 0,
            engine_tcwear: 0,
            engine_blown: 0,
            engine_seized: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,
    pub car_damage_data: [CarDamageData; 22],
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,     // Lap time in milliseconds
    pub sector1_time_in_ms: u16, // Sector 1 time in milliseconds
    pub sector2_time_in_ms: u16, // Sector 2 time in milliseconds
    pub sector3_time_in_ms: u16, // Sector 3 time in milliseconds
    pub lap_valid_bit_flags: u8, // 0x01 bit set-lap valid,      0x02 bit set-sector 1 valid
                                 // 0x04 bit set-sector 2 valid, 0x08 bit set-sector 3 valid
}

impl Default for LapHistoryData {
    fn default() -> Self {
        LapHistoryData {
            lap_time_in_ms: 0,
            sector1_time_in_ms: 0,
            sector2_time_in_ms: 0,
            sector3_time_in_ms: 0,
            lap_valid_bit_flags: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,              // Lap the tyre usage ends on (255 of current tyre)
    pub tyre_actual_compound: u8, // Actual tyres used by this driver
    pub tyre_visual_compound: u8, // Visual tyres used by this driver
}

impl Default for TyreStintHistoryData {
    fn default() -> Self {
        TyreStintHistoryData {
            end_lap: 0,
            tyre_actual_compound: 0,
            tyre_visual_compound: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketSessionHistoryData {
    pub header: PacketHeader,
    pub car_idx: u8,               // Index of the car this lap data relates to
    pub num_laps: u8,              // Num laps in the data (including current partial lap)
    pub num_tyre_stints: u8,       // Number of tyre stints in the data
    pub best_lap_time_lap_num: u8, // Lap the best lap time was achieved on
    pub best_sector1_lap_num: u8,  // Lap the best Sector 1 time was achieved on
    pub best_sector2_lap_num: u8,  // Lap the best Sector 2 time was achieved on
    pub best_sector3_lap_num: u8,  // Lap the best Sector 3 time was achieved on
    #[br(little, count = 100)]
    pub lap_history_data: Vec<LapHistoryData>,
    pub tyre_stint_history_data: [TyreStintHistoryData; 8],
}

trait Event {}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventFastestLap {
    pub vehicle_idx: u8, // Vehicle index of car achieving fastest lap
    pub lap_time: f32,   // Lap time is in seconds
}

impl Default for EventFastestLap {
    fn default() -> Self {
        EventFastestLap {
            vehicle_idx: 0,
            lap_time: 0.0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventRetirement {
    pub vehicle_idx: u8, // Vehicle index of car retiring
}

impl Default for EventRetirement {
    fn default() -> Self {
        EventRetirement { vehicle_idx: 0 }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventTeamMateInPits {
    pub vehicle_idx: u8, // Vehicle index of team mate
}

impl Default for EventTeamMateInPits {
    fn default() -> Self {
        EventTeamMateInPits { vehicle_idx: 0 }
    }
}
#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventRaceWinner {
    pub vehicle_idx: u8, // Vehicle index of the race winner
}

impl Default for EventRaceWinner {
    fn default() -> Self {
        EventRaceWinner { vehicle_idx: 0 }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventPenalty {
    pub penalty_type: u8,      // Penalty type – see Appendices
    pub infringement_type: u8, // Infringement type – see Appendices
    pub vehicle_idx: u8,       // Vehicle index of the car the penalty is applied to
    pub other_vehicle_idx: u8, // Vehicle index of the other car involved
    pub time: u8,              // Time gained, or time spent doing action in seconds
    pub lap_num: u8,           // Lap the penalty occurred on
    pub places_gained: u8,     // Number of places gained by this
}

impl Default for EventPenalty {
    fn default() -> Self {
        EventPenalty {
            penalty_type: 0,
            infringement_type: 0,
            vehicle_idx: 0,
            other_vehicle_idx: 0,
            time: 0,
            lap_num: 0,
            places_gained: 0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventSpeedTrap {
    pub vehicle_idx: u8, // Vehicle index of the vehicle triggering speed trap
    pub speed: f32,      // Top speed achieved in kilometres per hour
    pub is_overall_fastest_in_session: u8, // Overall fastest speed in session = 1, otherwise 0
    pub is_driver_fastest_in_session: u8, // Fastest speed for driver in session = 1, otherwise 0
    pub fastest_vehicle_idx_in_session: u8, // Vehicle index of the vehicle that is the fastest
    // in this session
    pub fastest_speed_in_session: f32, // Speed of the vehicle that is the fastest
                                       // in this session
}

impl Default for EventSpeedTrap {
    fn default() -> Self {
        EventSpeedTrap {
            vehicle_idx: 0,
            speed: 0.0,
            is_overall_fastest_in_session: 0,
            is_driver_fastest_in_session: 0,
            fastest_vehicle_idx_in_session: 0,
            fastest_speed_in_session: 0.0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventStartLights {
    pub num_lights: u8, // Number of lights showing
}

impl Default for EventStartLights {
    fn default() -> Self {
        EventStartLights { num_lights: 0 }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventDriveThroughPenaltyServed {
    pub vehicle_idx: u8, // Vehicle index of the vehicle serving drive through
}

impl Default for EventDriveThroughPenaltyServed {
    fn default() -> Self {
        EventDriveThroughPenaltyServed { vehicle_idx: 0 }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventStopGoPenaltyServed {
    pub vehicle_idx: u8, // Vehicle index of the vehicle serving stop go
}

impl Default for EventStopGoPenaltyServed {
    fn default() -> Self {
        EventStopGoPenaltyServed { vehicle_idx: 0 }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventFlashback {
    pub flashback_frame_identifier: u32, // Frame identifier flashed back to
    pub flashback_session_time: f32,     // Session time flashed back to
}

impl Default for EventFlashback {
    fn default() -> Self {
        EventFlashback {
            flashback_frame_identifier: 0,
            flashback_session_time: 0.0,
        }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct EventButtons {
    pub button_status: u32, // Bit flags specifying which buttons are being pressed
                            // currently - see appendices
}

impl Default for EventButtons {
    fn default() -> Self {
        EventButtons { button_status: 0 }
    }
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub struct PacketEventData {
    pub header: PacketHeader,         // Header
    pub event_string_code: [char; 4], // Event string code, see below
    pub event_details: EventDataDetails, // Event details - should be interpreted differently
                                      // for each type
}

#[derive(Debug, BinRead, Serialize, Deserialize)]
pub enum EventDataDetails {
    Buttons(EventButtons),
}

pub enum TelemetryTypes {
    CarStatus(PacketCarStatusData),
    Motion(PacketMotionData),
    FinalClassification(PacketFinalClassificationData),
    Session(PacketSessionData),
    LapData(PacketLapData),
    Participants(PacketParticipantsData),
    CarSetup(PacketCarSetupData),
    CarTelemetry(PacketCarTelemetryData),
    LobbyInfo(PacketLobbyInfoData),
    CarDamage(PacketCarDamageData),
    SessionHistory(PacketSessionHistoryData),
}
