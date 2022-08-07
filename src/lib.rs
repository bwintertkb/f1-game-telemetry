use binread::{self, io::Cursor, BinRead, BinReaderExt};
use serde::Serialize;
use telemetry::{EventButtons, EventFastestLap, EventFlashback, PacketEventData};
use tokio::{
    net::UdpSocket,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

use crate::telemetry::{
    EventDriveThroughPenaltyServed, EventPenalty, EventRaceWinner, EventRetirement, EventSpeedTrap,
    EventStartLights, EventStopGoPenaltyServed, EventTeamMateInPits, PacketCarDamageData,
    PacketCarSetupData, PacketCarStatusData, PacketCarTelemetryData, PacketFinalClassificationData,
    PacketLapData, PacketLobbyInfoData, PacketMotionData, PacketParticipantsData,
    PacketSessionData, PacketSessionHistoryData,
};

pub mod telemetry;

const BUFFER_SIZE: usize = 10024;
const UNSUPPORTED_EVENT: [&str; 6] = ["SSTA", "SEND", "DRSE", "DRSD", "CHQF", "LGOT"];

pub struct Telemetry {
    endpoint: String,
    data: Vec<u8>,
    pub listener: Option<UnboundedReceiver<Vec<String>>>,
}

fn read_telemetry<T: BinRead + Serialize>(buffer: [u8; BUFFER_SIZE]) -> String {
    let mut reader = Cursor::new(buffer);
    let tel: T = reader.read_le().unwrap();
    serde_json::to_string(&tel).unwrap()
}

fn read_event_telemetry(buffer: [u8; BUFFER_SIZE]) -> Option<String> {
    let mut reader = Cursor::new(buffer.clone());
    let pkt_hdr: PacketEventData<EventFlashback> = reader.read_le().unwrap();
    let event_type = chars_to_string(&pkt_hdr.event_string_code);
    if UNSUPPORTED_EVENT.contains(&&event_type[..]) {
        return None;
    }

    let tel: Option<String> = match &event_type[..] {
        "SSTA" => None,
        "SEND" => None,
        "FTLP" => Some(read_telemetry::<PacketEventData<EventFastestLap>>(buffer)),
        "RTMT" => Some(read_telemetry::<PacketEventData<EventRetirement>>(buffer)),
        "DRSE" => None,
        "DRSD" => None,
        "TMPT" => Some(read_telemetry::<PacketEventData<EventTeamMateInPits>>(
            buffer,
        )),
        "CHQF" => None,
        "RCWN" => Some(read_telemetry::<PacketEventData<EventRaceWinner>>(buffer)),
        "PENA" => Some(read_telemetry::<PacketEventData<EventPenalty>>(buffer)),
        "SPTP" => Some(read_telemetry::<PacketEventData<EventSpeedTrap>>(buffer)),
        "STLG" => Some(read_telemetry::<PacketEventData<EventStartLights>>(buffer)),
        "LGOT" => None,
        "DTSV" => Some(read_telemetry::<
            PacketEventData<EventDriveThroughPenaltyServed>,
        >(buffer)),
        "SGSV" => Some(read_telemetry::<PacketEventData<EventStopGoPenaltyServed>>(
            buffer,
        )),
        "FLBK" => Some(read_telemetry::<PacketEventData<EventFlashback>>(buffer)),
        "BUTN" => Some(read_telemetry::<PacketEventData<EventButtons>>(buffer)),
        _ => None,
    };

    tel
}

impl Telemetry {
    pub async fn record(&mut self) -> UnboundedReceiver<String> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(Telemetry::transmitter(
            tx,
            self.endpoint.clone(),
            self.data.clone(),
        ));
        rx
    }

    async fn transmitter(tx: UnboundedSender<String>, endpoint: String, data: Vec<u8>) {
        let socket = UdpSocket::bind(&endpoint).await.unwrap();
        println!("Listening on: {}", socket.local_addr().unwrap());
        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        loop {
            socket.recv(&mut buf).await.unwrap();

            let mut reader = Cursor::new(buf.clone());
            let pkt_hdr: telemetry::PacketHeader = reader.read_le().unwrap();
            if !data.contains(&pkt_hdr.packet_id) {
                //Not interested in this packet_id
                continue;
            }
            let tel = match pkt_hdr.packet_id {
                0 => read_telemetry::<PacketMotionData>(buf.clone()),
                1 => read_telemetry::<PacketSessionData>(buf.clone()),
                2 => read_telemetry::<PacketLapData>(buf.clone()),
                3 => match read_event_telemetry(buf.clone()) {
                    Some(tel) => tel,
                    None => continue,
                },
                4 => read_telemetry::<PacketParticipantsData>(buf.clone()),
                5 => read_telemetry::<PacketCarSetupData>(buf.clone()),
                6 => read_telemetry::<PacketCarTelemetryData>(buf.clone()),
                7 => read_telemetry::<PacketCarStatusData>(buf.clone()),
                8 => read_telemetry::<PacketFinalClassificationData>(buf.clone()),
                9 => read_telemetry::<PacketLobbyInfoData>(buf.clone()),
                10 => read_telemetry::<PacketCarDamageData>(buf.clone()),
                11 => read_telemetry::<PacketSessionHistoryData>(buf.clone()),
                _ => continue,
            };
            match tx.send(tel) {
                Ok(_) => continue,
                Err(e) => {
                    println!("ERROR: {}", e);
                    continue;
                }
            }
        }
    }
}

fn chars_to_string(chars: &[char]) -> String {
    let mut str = String::with_capacity(chars.len());
    chars.iter().for_each(|c| str.push(*c));
    str
}

pub struct TelemetryBuilder {
    endpoint: String,
    events_data: Option<u8>,
    car_status_data: Option<u8>,
    motion_data: Option<u8>,
    final_classification_data: Option<u8>,
    session_data: Option<u8>,
    lap_data: Option<u8>,
    participants_data: Option<u8>,
    car_setup_data: Option<u8>,
    car_telemetry_data: Option<u8>,
    lobby_info_data: Option<u8>,
    car_damage_data: Option<u8>,
    session_history_data: Option<u8>,
}
impl TelemetryBuilder {
    pub fn new(endpoint: String) -> Self {
        TelemetryBuilder {
            endpoint,
            events_data: None,
            car_status_data: None,
            motion_data: None,
            final_classification_data: None,
            session_data: None,
            lap_data: None,
            participants_data: None,
            car_setup_data: None,
            car_telemetry_data: None,
            lobby_info_data: None,
            car_damage_data: None,
            session_history_data: None,
        }
    }

    pub fn add_events_data(mut self) -> Self {
        self.events_data = Some(3);
        self
    }

    pub fn add_car_status_data(mut self) -> Self {
        self.car_status_data = Some(7);
        self
    }

    pub fn add_motion_data(mut self) -> Self {
        self.motion_data = Some(0);
        self
    }

    pub fn add_final_classification_data(mut self) -> Self {
        self.final_classification_data = Some(8);
        self
    }

    pub fn add_session_data(mut self) -> Self {
        self.session_data = Some(1);
        self
    }

    pub fn add_lap_data(mut self) -> Self {
        self.lap_data = Some(2);
        self
    }

    pub fn add_participant_data(mut self) -> Self {
        self.participants_data = Some(4);
        self
    }

    pub fn add_car_setup_data(mut self) -> Self {
        self.car_setup_data = Some(5);
        self
    }

    pub fn add_car_telemetry_data(mut self) -> Self {
        self.car_telemetry_data = Some(6);
        self
    }

    pub fn add_lobby_info_data(mut self) -> Self {
        self.lobby_info_data = Some(9);
        self
    }

    pub fn add_car_damage_data(mut self) -> Self {
        self.car_damage_data = Some(10);
        self
    }

    pub fn add_session_history_data(mut self) -> Self {
        self.session_history_data = Some(11);
        self
    }

    pub fn add_all_data(self) -> Self {
        self.add_car_status_data()
            .add_motion_data()
            .add_final_classification_data()
            .add_session_data()
            .add_lap_data()
            .add_participant_data()
            .add_car_setup_data()
            .add_car_telemetry_data()
            .add_lobby_info_data()
            .add_car_damage_data()
            .add_session_history_data()
    }

    fn as_array(&self) -> [Option<u8>; 12] {
        [
            self.events_data,
            self.car_damage_data,
            self.motion_data,
            self.final_classification_data,
            self.session_data,
            self.lap_data,
            self.participants_data,
            self.car_setup_data,
            self.car_telemetry_data,
            self.lobby_info_data,
            self.car_damage_data,
            self.session_history_data,
        ]
    }

    pub fn build(self) -> Telemetry {
        let data = self.as_array().into_iter().flatten().collect();
        Telemetry {
            endpoint: self.endpoint,
            data,
            listener: None,
        }
    }
}
