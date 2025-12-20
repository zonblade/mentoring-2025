
#[derive(Clone, Debug)]
struct Animal {
    age: i32
}

fn ok()->(String, i32){
    ("gak ok".to_string(), 500)
}


//
// dyn Error/anyhow
//            ^^^^ not rekomended
//
fn ok2(){
    let x = Animal{
        age: 1
    };
}
