use std::{ptr::addr_of, task::Waker};

/*
pub fn get_waker_id(waker: &Waker) -> (usize, usize)
{

    let data_addr = waker.data() as usize; //.addr();

    let vtable_addr = addr_of!(*waker.vtable()) as usize; //.addr();

    (data_addr, vtable_addr)    

}
*/

pub fn get_waker_id(waker: &Waker) -> u128
{

    let data_addr = waker.data() as usize; //.addr();

    let vtable_addr = addr_of!(*waker.vtable()) as usize; //.addr();

    //let val = data_addr as u128;

    //let mut bytes = val.to_le_bytes();

    let data_adr_be_bytes = data_addr.to_be_bytes();

    let vtable_adr_be_bytes = vtable_addr.to_be_bytes();

    let mut bytes: [u8; 16] = [0; 16];

    //data_adr_be_bytes

    bytes[0] = data_adr_be_bytes[0];

    bytes[1] = data_adr_be_bytes[1];

    bytes[2] = data_adr_be_bytes[2];

    bytes[3] = data_adr_be_bytes[3];

    bytes[4] = data_adr_be_bytes[4];

    bytes[5] = data_adr_be_bytes[5];

    bytes[6] = data_adr_be_bytes[6];

    bytes[7] = data_adr_be_bytes[7];

    //vtable_adr_be_bytes

    bytes[8] = vtable_adr_be_bytes[0];

    bytes[9] = vtable_adr_be_bytes[1];

    bytes[10] = vtable_adr_be_bytes[2];

    bytes[11] = vtable_adr_be_bytes[3];

    bytes[12] = vtable_adr_be_bytes[4];

    bytes[13] = vtable_adr_be_bytes[5];

    bytes[14] = vtable_adr_be_bytes[6];

    bytes[15] = vtable_adr_be_bytes[7];

    u128::from_be_bytes(bytes)

}
