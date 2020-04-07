use keccak_hash::write_keccak;
use ethereum_types::{Address, H256};

pub fn keccak<T: AsRef<[u8]>>(s: T) -> H256 {
	let mut result = [0u8; 32];
	write_keccak(s, &mut result);
	H256(result)
}


pub fn contract_address(sender: &Address, code: &[u8], salt: &H256) -> Address {
    let code_hash = keccak(code);
    let mut buffer = [0u8; 1 + 20 + 32 + 32];
    buffer[0] = 0xff;
    &mut buffer[1..(1+20)].copy_from_slice(&sender[..]);
    &mut buffer[(1+20)..(1+20+32)].copy_from_slice(&salt[..]);
    &mut buffer[(1+20+32)..].copy_from_slice(&code_hash[..]);
    From::from(keccak(&buffer[..]))
}
