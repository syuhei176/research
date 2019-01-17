// an ewasm test case, originally written in wasm, ported to Rust
// modify https://github.com/ewasm/rust-ewasm

extern "C" {
  pub fn ethereum_getAddress(resultOffset: *const u32);
  pub fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
  pub fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
}

#[no_mangle]
pub fn main() {
  //  32,32,32,32,32,32,32,32 = 256
  // [0, 0, 0, 0, 0, 0, 0, 0] fill 256 bits of memory with zeros

  let address : [u32;8] = [0;8];
  let raw_address = &address as *const u32;

  let sstore_key: [u32;8] = [0;8];
  let raw_sstore_key = &sstore_key as *const u32;

  let load_address : [u32;8] = [0;8];
  let load_raw_address = &load_address as *const u32;


  unsafe {
    ethereum_getAddress(raw_address);
    ethereum_storageStore(raw_sstore_key, raw_address);
  }

  unsafe {
    ethereum_storageLoad(raw_sstore_key, load_raw_address);
  }

  return;
}
