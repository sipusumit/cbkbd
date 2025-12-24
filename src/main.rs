use cbkbd::CosmicByteDevice;
// use hidapi::MAX_REPORT_DESCRIPTOR_SIZE;

fn main(){
    let cb = CosmicByteDevice::new().unwrap();
    let info = cb.device.get_device_info().unwrap();
    println!("Vendor ID: {:04X}, Product ID: {:04X}, Interface Number: {:?}, Manufacturer: {:?}, Product String: {:?}",
        info.vendor_id(), info.product_id(), info.interface_number(), info.manufacturer_string().unwrap(), info.product_string().unwrap());
    // let colors = [
    //     cbkbd::RGB::new(255, 0, 0),
    //     // cbkbd::RGB::new(0, 255, 0), // 80 170 255
    //     cbkbd::RGB::new(80, 170, 255), // 80 170 255
    //     cbkbd::RGB::new(0, 0, 255),
    //     cbkbd::RGB::new(255, 255, 0),
    //     cbkbd::RGB::new(0, 255, 255),
    //     cbkbd::RGB::new(255, 0, 255),
    //     cbkbd::RGB::new(255, 255, 255),
    // ];
    // cb.set_colors(colors).unwrap();
    cb.set_led_type(cbkbd::CbEffect::RainDrop, cbkbd::CbBrightness::Level7, 0x04, cbkbd::CbColor::Color2).unwrap();
    // cb.set_user_effect(cbkbd::CbUserDefinedEffect::USERDEFINE1, cbkbd::CbBrightness::Level7, 0x04, cbkbd::CbColor::ColorLoop).unwrap();

    // let matrix = [cbkbd::RGB::new(255,0,0); 84];
    // cb.set_led_matrix(matrix, true).unwrap();
    // cb.set_led_type(cbkbd::CbEffect::USERDEFINE1, cbkbd::CbBrightness::Level7, 0x04, cbkbd::CbColor::Color2).unwrap();

}