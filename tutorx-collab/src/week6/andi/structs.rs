pub enum CrewStatus {
    Planned,
    OnBoard,
    SignOff
}

pub struct Crew {
    pub id: u32,
    pub age: u8,
    pub first_name: String,
    pub last_name: String,
    pub staff_number: String,
    pub status: CrewStatus,
}

pub enum ShipStatus {
    Active, // << tipe jangan lupa :D bukan enum soalnya
    Inactive
}

pub struct Ship {
    pub id: String,
    pub name: String,
    pub status: ShipStatus,
}