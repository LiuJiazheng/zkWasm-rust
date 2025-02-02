#![cfg_attr(
    feature = "witness",
    feature(ptr_sub_ptr)
)]

extern "C" {
    pub fn wasm_input(is_public: u32) -> u64;
    pub fn wasm_output(v: u64);
    pub fn wasm_read_context() -> u64;
    pub fn wasm_write_context(v: u64);
    pub fn require(cond: bool);
    pub fn wasm_dbg(v: u64);
    pub fn wasm_dbg_char(v: u64);

    pub fn merkle_setroot(x: u64);
    pub fn merkle_address(x: u64);
    pub fn merkle_set(x: u64);
    pub fn merkle_get() -> u64;
    pub fn merkle_getroot() -> u64;
    pub fn merkle_fetch_data() -> u64;
    pub fn merkle_put_data(x: u64);
    pub fn poseidon_new(x: u64);
    pub fn poseidon_push(x: u64);
    pub fn poseidon_finalize() -> u64;

    pub fn babyjubjub_sum_new(x: u64);
    pub fn babyjubjub_sum_push(x: u64);
    pub fn babyjubjub_sum_finalize() -> u64;

}

pub mod jubjub;
pub mod merkle;
pub mod poseidon;

#[cfg(feature = "witness")]
pub mod witness;

pub use jubjub::*;
pub use merkle::*;
pub use poseidon::*;

#[cfg(feature = "witness")]
pub use witness::*;

pub fn wasm_dbg_str(s: &str) {
    unsafe {
        require(s.len() < usize::MAX);
    }
    for i in s.as_bytes() {
        unsafe { wasm_dbg_char(*i as u64) }
    }
}

#[macro_export]
macro_rules! dbg {
    ($fmt:literal $(, $args:tt)* $(,)?)
        => {
            let _ = $crate::wasm_dbg_str(&format!($fmt $(, $args)*));
        };
}
#[macro_export]
macro_rules! dbgln {
    ($fmt:literal $(, $args:tt)* $(,)?) => {
        $dbg!($fmt $(, $args)*);
        $dbg!("\n");
    };
}

#[cfg(feature = "wasmbind")]
mod test;
