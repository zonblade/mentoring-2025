
#[cfg(feature = "penambahan_add")]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "pengurangan")]
pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(feature = "penambahan_add2")]
pub fn tambah(left: u64, right: u64) -> u64 {
    left + right
}