struct Crew {
    pub id: u32,
    pub age: u8,
    pub first_name: String,
    pub last_name: String,
    pub staff_number: String
}

pub fn crewDetail() -> Result<Crew, String> {
    let legendCrew: Crew = Crew { 
        id: 1, 
        age: 32, 
        first_name: String::from("Jack"), 
        last_name: String::from("Separo"), 
        staff_number: String::from("JS1") 
    };

    OK(legendCrew)
}