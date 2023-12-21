mod config;

use json;

fn config_sensor(host: &str, config: &json::object) {
    todo!()
}

struct config {
    operating_mode: operating_mode,

}

enum operating_mode {
    NORMAL,
    STANDBY,
}

enum lidar_mode { 
    //This cannot be the right way to do this... but
    TenTwentyFourBy10 = "1024x10",
    TenTwentyFourBy20 = "1024x20",
}
