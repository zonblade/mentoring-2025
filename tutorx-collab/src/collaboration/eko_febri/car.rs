struct Car {
    color: String,
    year: i32,
    name: String
}
// Result<_,Err> 
pub fn newCar(color: String, year: i32, name: String) ->Result<Car, i32> {
    Ok(Car { color, year, name })
}
