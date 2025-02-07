use libloading::{Library, Symbol};

type TurnOnFn = unsafe extern "C" fn() -> ReturnCode;
type TurnOffFn = unsafe extern "C" fn() -> ReturnCode;
type GetSocketInfoFn = unsafe extern "C" fn(&mut SocketInfo) -> ReturnCode;

#[repr(u8)]
#[derive(Debug)]
pub enum ReturnCode {
    Ok = 0,
    CreateSocketFailed = 1,
    CommandError = 2,
    AsyncRuntimeError = 3,
}

#[derive(Debug)]
pub struct SocketInfo {
    pub is_turned_on: bool,
    pub power: f64,
}

#[tokio::main]
async fn main() {
    let lib = unsafe { Library::new("../target/debug/ffi_socket_lib.dll") }.unwrap();
    let turn_on: Symbol<'_, TurnOnFn> = unsafe { lib.get(b"turn_on") }.unwrap();
    let turn_off: Symbol<'_, TurnOffFn> = unsafe { lib.get(b"turn_off") }.unwrap();
    let get_socket_info: Symbol<'_, GetSocketInfoFn> = unsafe { lib.get(b"get_socket_info") }.unwrap();

    let mut info = SocketInfo {
        is_turned_on: false,
        power: 0.0,
    };

    let result = unsafe { turn_on() };
    println!("turn_on result {:?}", result);

    let result = unsafe { get_socket_info(&mut info) };
    println!("Result: {result:?} Info: {info:#?}");

    let result = unsafe { turn_off() };
    println!("turn_off result{:?}", result);

    let result = unsafe { get_socket_info(&mut info) };
    println!("Result: {result:?} Info: {info:#?}");
}
