// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::collections::HashMap;
use std::{sync::Arc, thread};
use tauri::{AppHandle, Manager};
use std::sync::Mutex;
mod modbus;
use std::time::{SystemTime, UNIX_EPOCH};


use std::future::Future;
use modbus::server::ModbusServer;
use tokio::net::TcpListener;

use rand::Rng;
use std::net::SocketAddr;
use futures::executor::block_on;
use tokio::time;
use tokio_modbus::{
    prelude::*,
    server::tcp::{accept_tcp_connection, Server},
};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct RegisterFan {
  id: u8,
}



#[tokio::main]
async fn main() {

    let server = ModbusServer::new();
    //let holding = server.holding_registers.clone();
    //let input = server.input_registers.clone();

    let fan_ids: Vec<u8> = Vec::new();

    tauri::Builder::default().setup(|app| {
        setup(app.handle().clone(), fan_ids, server);
        println!("Done!");
        Ok(())
      })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn setup(app_handle: AppHandle, fan_ids: Vec<u8>, server: ModbusServer) {
    let fan_ids_shared = Arc::new(Mutex::new(fan_ids));

    let evt_clone = fan_ids_shared.clone();

    let holding_registers = server.holding_registers.clone();

    app_handle.listen_global("register-fan", move|event| {
        let payload = event.payload().unwrap().to_string();
        let fan: RegisterFan = serde_json::from_str(&payload).unwrap();

        let mut fan_ids = evt_clone.lock().unwrap();
        fan_ids.push(fan.id);
    });

    tokio::spawn(async move {
        let socket_addr = "0.0.0.0:502".parse().unwrap();
        server_context(socket_addr, server).await.unwrap();
    });

    let fan_ids_ = fan_ids_shared.clone();


    thread::spawn(move || {
        fan_speed_update(app_handle, fan_ids_.clone(), holding_registers);
    }); 
}

fn fan_speed_update(app_handle: AppHandle, fan_ids: Arc<Mutex<Vec<u8>>>, holding_reg: Arc<Mutex<HashMap<u16, u16>>>) {
    // So, we might prevent the event from getting the lock here if we don't wait.
    // very bad solution, should probably use signaling or something.
    thread::sleep(std::time::Duration::from_secs(3));

    loop {
        let fan_ids = fan_ids.lock().unwrap();    
        for id in fan_ids.iter() {
             let start = SystemTime::now();
             let since_the_epoch = start
                 .duration_since(UNIX_EPOCH)
                 .expect("Time went backwards");

            let time_ms = since_the_epoch.as_millis();
            let factor = std::f64::consts::TAU / (1e2 * 30.0);
            let integer_part = (f64::sin(time_ms as f64 * factor + *id as f64) * 70.0) as u16;

            let mut holding = holding_reg.lock().unwrap();
            holding.insert(*id as u16, integer_part);
            app_handle.emit_all(&format!("fan-val-{}", id), integer_part).unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}

async fn server_context(socket_addr: SocketAddr, modbus_server: ModbusServer) -> anyhow::Result<()> {
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);
    let new_service = |_socket_addr| Ok(Some(modbus_server.clone()));
    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };

    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}