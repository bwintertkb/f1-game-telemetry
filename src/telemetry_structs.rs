use binread::{self, BinRead};

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
pub struct PacketCarStatusData {
    pub header: PacketHeader, // Header
    pub car_status_data: [CarStatusData; 22],
}

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,
    pub num_cars: u8,
    pub classification_data: [FinalClassificationData; 22],
}

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
pub struct PacketLapData {
    pub header: PacketHeader,         // Header
    pub lap_data: [LapData; 22],      // Lap data for all cars on track
    pub time_trial_pbcar_idx: u8,     // Index of Personal Best car in time trial (255 if invalid)
    pub time_trial_rival_car_idx: u8, // Index of Rival car in time trial (255 if invalid)
}

#[derive(Debug, BinRead)]
pub struct ParticipantData {
    pub ai_controlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub driver_id: u8,     // Driver id - see appendix, 255 if network human
    pub network_id: u8,    // Network id – unique identifier for network players
    pub team_id: u8,       // Team id - see appendix
    pub my_team: u8,       // My team flag – 1 = My Team, 0 = otherwise
    pub race_number: u8,   // Race number of the car
    pub nationality: u8,   // Nationality of the driver
    #[br(little, count = 48)]
    pub name: Vec<char>, // TODO should be [char; 48] Name of participant in UTF-8 format – null terminated
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

#[derive(Debug, BinRead)]
pub struct PacketParticipantsData {
    pub header: PacketHeader, // Header
    pub num_active_cars: u8,  // Number of active cars in the data – should match number of
    // cars on HUD
    pub participants: [ParticipantData; 22],
}
