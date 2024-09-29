use smart_house::devices::socket::Socket;
use smart_house::devices::thermometer::Thermometer;
use smart_house::sh::SmartHouse;

fn main() {
    let mut house = SmartHouse::new("Загородный дом");

    let socket = Socket::new()
        .set_description("для холодильника")
        .switch_on()
        .build();
    house.add_device("Кухня", socket);
    let socket = Socket::new()
        .set_description("для электроплиты")
        .switch_on()
        .build();
    house.add_device("Кухня", socket);
    house.add_device("Кухня", Thermometer::new());

    house.add_device("Столовая", Thermometer::new());

    let socket = Socket::new()
        .set_description("для торшера")
        .switch_on()
        .build();
    house.add_device("Спальня", socket);
    house.add_device("Спальня", Thermometer::new());

    println!("Список помещений: {:?}", house.room_list());

    println!(
        "Устройства на кухне: {:?}",
        house
            .get_room_devices("Кухня")
            .unwrap()
            .iter()
            .map(|d| d.get_device_name())
            .collect::<Vec<_>>()
    );

    println!("{}", house.report());
}
