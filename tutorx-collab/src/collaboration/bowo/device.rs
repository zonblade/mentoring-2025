struct Device {
    id: u32,
    name: String,
    online: bool
}

fn get_device(found: bool) -> Option<Device>{
    let device = Device{
        id: 100,
        name: "Hikvison-cam_1".to_string(),
        online: true,
    };

    if found{
        Some(device)
    }else{
        None
    }
}

fn main(){
    let device = get_device(true);
    match device{
        Some(d) => println!("{} - {} - {}", d.id, d.name, d.online),
        None => println!("Device tidak di temukab")
    }
}