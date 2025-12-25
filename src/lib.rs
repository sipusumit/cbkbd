extern crate hidapi;

use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};
use serde::{Deserialize, Serialize};

const VENDOR_ID: u16 = 0x04D9;
const PRODUCT_ID: u16 = 0xA1CD;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum CbColor {
    Color1 = 0x00,
    Color2,
    Color3,
    Color4,
    #[default]
    Color5,
    Color6,
    Color7,
    ColorLoop,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum CbEffect {
    Static = 0x00,
    Breathe,
    Fade,
    GettingOff,
    LittleStars,
    Laser,
    #[default]
    Wave,
    Neon,
    RainDrop,
    Ripple,
    Wave2,
    Swirl,
    USERDEFINE1 = 0x33,
    USERDEFINE2,
    USERDEFINE3,
    USERDEFINE4,
    USERDEFINE5,
}

impl CbEffect {
    // Helper to convert simple slider index (0-7) to your Hardware Enum
    pub fn from_index(index: u8) -> Self {
        match index {
            1 => CbEffect::Breathe,
            2 => CbEffect::Fade,
            3 => CbEffect::GettingOff,
            4 => CbEffect::LittleStars,
            5 => CbEffect::Laser,
            6 => CbEffect::Wave,
            7 => CbEffect::Neon,
            8 => CbEffect::RainDrop,
            9 => CbEffect::Ripple,
            10 => CbEffect::Wave2,
            11 => CbEffect::Swirl,
            12 => CbEffect::USERDEFINE1,
            13 => CbEffect::USERDEFINE2,
            14 => CbEffect::USERDEFINE3,
            15 => CbEffect::USERDEFINE4,
            16 => CbEffect::USERDEFINE5,
            _ => CbEffect::Static, // Index 0 
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum CbBrightness {
    Level0 = 0x00,
    Level1 = 0x09,
    Level2 = 0x12,
    Level3 = 0x1B,
    Level4 = 0x24,
    Level5 = 0x2D,
    Level6 = 0x36,
    #[default]
    Level7 = 0x3F,
}

impl CbBrightness {
    // Helper to convert simple slider index (0-7) to your Hardware Enum
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => CbBrightness::Level0,
            1 => CbBrightness::Level1,
            2 => CbBrightness::Level2,
            3 => CbBrightness::Level3,
            4 => CbBrightness::Level4,
            5 => CbBrightness::Level5,
            6 => CbBrightness::Level6,
            _ => CbBrightness::Level7, // Default catch-all
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct RGB{
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_6bit_bytes(&self) -> [u8; 3] {
        [
            (self.red >> 2) & 0x3F,
            (self.green >> 2) & 0x3F,
            (self.blue >> 2) & 0x3F,
        ]
    }

    pub fn from_6bit_rgb(r: u8, g: u8, b: u8) -> Self{
        Self {
            red: (r << 2) | (r >> 4),
            green: (g << 2) | (g >> 4),
            blue: (b << 2) | (b >> 4),
        }
    }
}

pub trait RGBArrayExt<const N: usize> {
    fn to_planer_format(&self) -> ([u8; N], [u8; N], [u8; N]);
}

impl<const N: usize> RGBArrayExt<N> for [RGB; N] {
    fn to_planer_format(&self) -> ([u8; N], [u8; N], [u8; N]) {
        let mut red: [u8; N] = [0; N];
        let mut green: [u8; N] = [0; N];
        let mut blue: [u8; N] = [0; N];
        for (col_index, color) in self.iter().enumerate() {
            let rgb_bytes = color.to_6bit_bytes();
            red[col_index] = rgb_bytes[0];
            green[col_index] = rgb_bytes[1];
            blue[col_index] = rgb_bytes[2];
        }
        (red, green, blue)
    }
}

pub struct CosmicByteDevice {
    pub device: HidDevice,
}

impl CosmicByteDevice{
    pub fn new() -> Result<Self, HidError> {
        let api = HidApi::new()?;

        let devices = Self::filter_cb_gk_37_interface(&api);
        // let rgb = devices.iter().find(|dev| dev.interface_number() == 2).cloned();
        let mut rgb: Option<DeviceInfo> = None;
        if devices.len() > 0 {
            for device_info in devices {
                if device_info.interface_number() == 2{
                    rgb = Some(device_info.clone());
                }
            }
            let rgb = rgb.expect("RGB interface not found");
            let rgb_device = rgb.open_device(&api)?;
            rgb_device.set_blocking_mode(true)
                .expect("Failed to enable blocking mode");
            return Ok(CosmicByteDevice {
                device: rgb_device
            });
        }


        return Err(HidError::HidApiError { message: "Device not found".to_owned() });
    }

    fn filter_cb_gk_37_interface(api: &HidApi) -> Vec<&DeviceInfo> {
        return api
            .device_list()
            .filter(|info|
                info.vendor_id() == VENDOR_ID &&
                info.product_id() == PRODUCT_ID && info.interface_number() == 2
            )
            .collect();
    }

    pub fn set_colors(&self, colors: [RGB; 7]) -> Result<(), HidError> {
        const COMMAND: [u8; 9] = [
            0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x55, 0xAA, 0x00
        ];
        self.device.send_feature_report(&COMMAND)?;
        let mut _response: [u8; 9] = [0; 9];
        self.device.get_feature_report(&mut _response)?;
        let mut buf: [u8; 64] = [0; 64];
        for (i, color) in colors.iter().enumerate() {
            let rgb_bytes = color.to_6bit_bytes();
            let offset = i * 3;
            buf[1 + offset] = rgb_bytes[0];
            buf[1 + offset + 1] = rgb_bytes[1];
            buf[1 + offset + 2] = rgb_bytes[2];
        }
        self.device.write(&buf)?;
        std::thread::sleep(std::time::Duration::from_millis(5));
        Ok(())
    }

    pub fn get_colors(&self) -> Result<[RGB; 7], String>{
        const COMMAND: [u8; 9] = [
            0x00, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x55, 0xAA, 0x00
        ];
        self.device.send_feature_report(&COMMAND).map_err(|err| err.to_string())?;
        let mut _response: [u8; 9] = [0; 9];
        self.device.get_feature_report(&mut _response).map_err(|err| err.to_string())?;
        let mut buf: [u8; 21] = [0; 21];
        let len = self.device.read(&mut buf).map_err(|err| err.to_string())?;
        // Check for at least 21 bytes (the amount we skip + 7 * 3)
        #[cfg(debug_assertions)]
        println!("[get_colors]: Len = {len}");
        if len < 21 {
            return Err("Length less than 22 (need index 0 skipped + 21 bytes)".into());
        }

        // Skips index 0 and creates 7 RGB objects directly into the array
        // let mut chunks = buf[1..22].chunks_exact(3);
        let mut chunks = buf.chunks_exact(3);
        let data: [RGB; 7] = std::array::from_fn(|_| {
            let group = chunks.next().expect("Checked length ensures this exists");
            RGB::from_6bit_rgb(group[0], group[1], group[2])
        });

        Ok(data)
    }

    pub fn set_led_type(&self, effect: CbEffect, brightness:CbBrightness , speed:u8, color:CbColor) -> Result<(), HidError>{
        let command: [u8; 9] = [
            0x00,       // Report ID
            0x08,       // command
            effect as u8,     // effect mod
            brightness as u8,
            speed,
            0x00,       // reserved
            color as u8, // color
            0xC4,
            0x3B
        ];

        self.device.send_feature_report(&command)?;
        Ok(())

    }

    pub fn set_led_matrix(&self, _matrix: [RGB; 84], save_to_fw: bool) -> Result<(), HidError>{
        // Not implemented yet
        let (red, green, blue) = _matrix.to_planer_format();
        let mode: [u8; 128] = [
            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 
            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 
            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 
            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, // 64 bytes 

            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 
            0x10, 0x10, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 64 bytes 
            
        ];

        let mut command = [
            0x00,
            0x12, // command set user picture
            0x00,
            if save_to_fw { 0x00 } else { 0xFF }, // 0 or 255
            0x00, // plane index
            0x00,
            0x55,
            0xaa,
            0x00
        ];

        let mut buf: [u8; 128] = [0; 128];
        for plane_index in 0..4{
            match plane_index {
                0 => {// mode buffer
                    for i in 0..128{
                        buf[i] = mode[i];
                    }
                },
                1 => {// red buffer
                    for i in 0..84{
                        buf[i] = red[i];
                    }
                },
                2 => {// green buffer
                    for i in 0..84{
                        buf[i] = green[i];
                    }
                },
                3 => {// blue buffer
                    for i in 0..84{
                        buf[i] = blue[i];
                    }
                },
                _ => {panic!("Invalid plane index");}
            }
            command[4] = plane_index as u8;
            self.device.send_feature_report(&command)?;
            // write buf in chunks of 65 bytes appended with 0 at start
            for chunk in buf.chunks(64) {
                let mut write_buf: [u8; 65] = [0; 65];
                write_buf[1..].copy_from_slice(chunk);
                self.device.write(&write_buf)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }
        // self.set_led_type(CbEffect::USERDEFINE1, CbBrightness::Level7, 0x04, CbColor::Color2).unwrap();
        Ok(())
    }
}