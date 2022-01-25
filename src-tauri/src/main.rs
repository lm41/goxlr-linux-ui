#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use goxlr_types::{ChannelName, FaderName, InputDevice, OutputDevice};

use tokio::net::UnixStream;
use tokio::runtime::Runtime;
use goxlr_ipc::{DeviceStatus, DeviceType, GoXLRCommand, MixerStatus, Socket, UsbProductInformation};
use crate::client::Client;

use strum::IntoEnumIterator;

mod client;

struct DaemonConnection {
    client: Mutex<Client>,
    rt: Runtime
}

#[derive(Serialize, Deserialize)]
struct DeviceStatusResponse {
    deviceStatus: DeviceStatus,
    routingMap: [[u8;8]; 5]
}

#[tauri::command]
fn test_command(param: String, clientState: tauri::State<'_, DaemonConnection>) -> Result<DeviceStatusResponse, String> {
    println!("{}", param);

    let mut client = clientState.inner().client.lock().unwrap();
    clientState.inner().rt.block_on(client.send(GoXLRCommand::GetStatus));

    // We need to pull the client out of scope ASAP, so clone the Device Status for response..
    let device_status = client.device().clone();
    let mixer_status = device_status.mixer.clone();



    //Err("Failed".into())
    Ok(DeviceStatusResponse{ deviceStatus: device_status, routingMap: get_mixer_array(mixer_status) }.into())
}

#[tauri::command]
fn set_channel_volume(id: u8, volume: u8, clientState: tauri::State<'_, DaemonConnection>)  {
    //let channelName: ChannelName::from_id(id);
    let channelName: ChannelName;
    channelName = match id {
        1 => ChannelName::Mic,
        2 => ChannelName::Chat,
        3 => ChannelName::Music,
        4 => ChannelName::Game,
        5 => ChannelName::Console,
        6 => ChannelName::LineIn,
        7 => ChannelName::LineOut,
        8 => ChannelName::System,
        9 => ChannelName::Sample,
        10 => ChannelName::Headphones,
        11 => ChannelName::MicMonitor,
        _ => ChannelName::Game
    };

    let mut client = clientState.inner().client.lock().unwrap();
    clientState.inner().rt.block_on(client.send(GoXLRCommand::SetVolume(channelName, volume)));
}

#[tauri::command]
fn set_fader_channel(id: u8, channel_id: u8, clientState: tauri::State<'_, DaemonConnection>) {
    let channelName: ChannelName;
    channelName = match channel_id {
        0 => ChannelName::Mic,
        1 => ChannelName::Chat,
        2 => ChannelName::Music,
        3 => ChannelName::Game,
        4 => ChannelName::Console,
        5 => ChannelName::LineIn,
        9 => ChannelName::LineOut,
        6 => ChannelName::System,
        7 => ChannelName::Sample,
        8 => ChannelName::Headphones,
        _ => ChannelName::Game
    };

    let faderName: FaderName;
    faderName = match id {
        0 => FaderName::A,
        1 => FaderName::B,
        2 => FaderName::C,
        3 => FaderName::D,
        _ => FaderName::A,
    };

    let mut client = clientState.inner().client.lock().unwrap();
    clientState.inner().rt.block_on(client.send(GoXLRCommand::AssignFader(faderName, channelName)));
}

fn main() -> Result<()> {
    // Establish and Connect the Client to the Daemon (we might wanna do this with a splash
    // screen and tauri::async_runtime in future), for now, we'll just block while we connect..
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let client = rt.block_on(connect_to_device())?;

    let daemon_connection = DaemonConnection {
        client: Mutex::new(client),
        rt
    };

    tauri::Builder::default()
        .manage(daemon_connection)
        .invoke_handler(tauri::generate_handler![test_command,set_channel_volume, set_fader_channel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn connect_to_device() -> Result<Client> {
    let stream = UnixStream::connect("/tmp/goxlr.socket")
        .await
        .context("Could not connect to the GoXLR Daemon Socket")?;

    let address = stream
        .peer_addr()
        .context("Could not get the address of the GoXLR daemon process")?;

    //SOCKET = Some(Socket::new(address, STREAM.borrow_mut().as_mut().unwrap()));
    let socket = Socket::new(address, stream);
    let mut client = Client::new(socket);
    client
        .send(GoXLRCommand::GetStatus)
        .await
        .context("Couldn't retrieve device status..")?;

    print_device(client.device());
    Ok(client)
}

fn print_device(device: &DeviceStatus) {
    println!(
        "Device type: {}",
        match device.device_type {
            DeviceType::Unknown => "Unknown",
            DeviceType::Full => "GoXLR (Full)",
            DeviceType::Mini => "GoXLR (Mini)",
        }
    );

    if let Some(usb) = &device.usb_device {
        print_usb_info(usb);
    }

    if let Some(mixer) = &device.mixer {
        print_mixer_info(mixer);
    }
}

fn get_mixer_array(mixer: Option<MixerStatus>) -> [[u8; 8]; 5] {
    let mut state: [[u8;8]; 5] = [[0; 8]; 5];
    let mixerStatus: MixerStatus;

    match mixer {
        Some(x) => mixerStatus = x,
        None => { return state }
    }

    for output in OutputDevice::iter() {
        for input in InputDevice::iter() {
            if mixerStatus.router[input as usize].contains(output) {
                state[output as usize][input as usize] = 1;
            } else {
                state[output as usize][input as usize] = 0;
            }
        }
    }

    return state;
}

fn print_usb_info(usb: &UsbProductInformation) {
    println!(
        "USB Device version: {}.{}.{}",
        usb.version.0, usb.version.1, usb.version.2
    );
    println!("USB Device manufacturer: {}", usb.manufacturer_name);
    println!("USB Device name: {}", usb.product_name);
    println!("USB Device is claimed by Daemon: {}", usb.is_claimed);
    println!(
        "USB Device has kernel driver attached: {}",
        usb.has_kernel_driver_attached
    );
    println!(
        "USB Address: bus {}, address {}",
        usb.bus_number, usb.address
    );
}

fn print_mixer_info(mixer: &MixerStatus) {
    println!("Mixer firmware: {}", mixer.hardware.versions.firmware);
    println!("Mixer dice: {}", mixer.hardware.versions.dice);
    println!("Mixer FPGA count: {}", mixer.hardware.versions.fpga_count);
    println!("Mixer serial number: {}", mixer.hardware.serial_number);
    println!(
        "Mixer manufacture date: {}",
        mixer.hardware.manufactured_date
    );

    for fader in FaderName::iter() {
        println!(
            "Fader {} assignment: {}",
            fader,
            mixer.get_fader_assignment(fader)
        )
    }

    for channel in ChannelName::iter() {
        let pct = (mixer.get_channel_volume(channel) as f32 / 255.0) * 100.0;
        if mixer.get_channel_muted(channel) {
            println!("{} volume: {:.0}% (Muted)", channel, pct);
        } else {
            println!("{} volume: {:.0}%", channel, pct);
        }
    }

    let max_col_len = OutputDevice::iter()
        .map(|s| s.to_string().len())
        .max()
        .unwrap_or_default();
    let mut table_width = max_col_len + 1;
    print!(" {}", " ".repeat(max_col_len));
    for input in InputDevice::iter() {
        let col_name = input.to_string();
        print!(" |{}|", col_name);
        table_width += col_name.len() + 3;
    }
    println!();
    println!("{}", "-".repeat(table_width));

    for output in OutputDevice::iter() {
        let row_name = output.to_string();
        print!("|{}{}|", " ".repeat(max_col_len - row_name.len()), row_name,);
        for input in InputDevice::iter() {
            let col_name = input.to_string();
            if mixer.router[input as usize].contains(output) {
                let len = col_name.len() + 1;
                print!("{}X{} ", " ".repeat(len / 2), " ".repeat(len - (len / 2)));
            } else {
                let len = col_name.len() + 2;
                print!("{} ", " ".repeat(len));
            }
        }
        println!();
    }
    println!("{}", "-".repeat(table_width));
}