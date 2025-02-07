use tcp_socket::{SocketConnector, SocketInfo};
use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn turn_on() -> ReturnCode {
    let rt = Runtime::new().unwrap();
    let res = rt.block_on(async {
        let client = tcp_socket::SocketClient::new("127.0.0.1:3456")?;
        client.turn_on().await
    });
    if res.is_err() {
        ReturnCode::CommandError
    } else {
        ReturnCode::Ok
    }
}

#[no_mangle]
pub extern "C" fn turn_off() -> ReturnCode {
    let rt = Runtime::new().unwrap();
    let res = rt.block_on(async {
        let client = tcp_socket::SocketClient::new("127.0.0.1:3456")?;
        client.turn_off().await
    });
    if res.is_err() {
        ReturnCode::CommandError
    } else {
        ReturnCode::Ok
    }
}

#[no_mangle]
pub extern "C" fn get_socket_info(info: &mut SocketInfo) -> ReturnCode {
    let rt = Runtime::new().unwrap();
    let res = rt.block_on(async {
        let client = tcp_socket::SocketClient::new("127.0.0.1:3456")?;
        client.get_socket_info().await
    });
    if res.is_err() {
        ReturnCode::CommandError
    } else {
        let d = res.unwrap();
        info.is_turned_on = d.is_turned_on;
        info.power = d.power;
        ReturnCode::Ok
    }
}

#[repr(u8)]
pub enum ReturnCode {
    Ok = 0,
    CreateSocketFailed = 1,
    CommandError = 2,
    AsyncRuntimeError = 3,
}
