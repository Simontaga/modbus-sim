use std::{
    collections::HashMap,
    future,
    sync::{Arc, Mutex},
};

use tokio_modbus::{Exception, Request, Response};

#[derive(Clone)]
pub struct ModbusServer {
    pub input_registers: Arc<Mutex<HashMap<u16, u16>>>,
    pub holding_registers: Arc<Mutex<HashMap<u16, u16>>>,
    pub coils: Arc<Mutex<HashMap<u16, bool>>>,
}

impl tokio_modbus::server::Service for ModbusServer {
    type Request = Request<'static>;
    type Future = future::Ready<Result<Response, Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(addr, cnt) => {
                match register_read(&self.input_registers.lock().unwrap(), addr, cnt) {
                    Ok(values) => future::ready(Ok(Response::ReadInputRegisters(values))),
                    Err(err) => future::ready(Err(err)),
                }
            }
            Request::ReadHoldingRegisters(addr, cnt) => {
                match register_read(&self.holding_registers.lock().unwrap(), addr, cnt) {
                    Ok(values) => future::ready(Ok(Response::ReadHoldingRegisters(values))),
                    Err(err) => future::ready(Err(err)),
                }
            }
            Request::ReadCoils(addr, cnt) => {
                match coil_read(&self.coils.lock().unwrap(), addr, cnt) {
                    Ok(values) => future::ready(Ok(Response::ReadCoils(values))),
                    Err(err) => future::ready(Err(err)),
                }
            }
            Request::WriteMultipleRegisters(addr, values) => {
                match register_write(&mut self.holding_registers.lock().unwrap(), addr, &values) {
                    Ok(_) => future::ready(Ok(Response::WriteMultipleRegisters(
                        addr,
                        values.len() as u16,
                    ))),
                    Err(err) => future::ready(Err(err)),
                }
            }
            Request::WriteSingleRegister(addr, value) => {
                match register_write(
                    &mut self.holding_registers.lock().unwrap(),
                    addr,
                    std::slice::from_ref(&value),
                ) {
                    Ok(_) => future::ready(Ok(Response::WriteSingleRegister(addr, value))),
                    Err(err) => future::ready(Err(err)),
                }
            }
            _ => {
                println!("SERVER: Exception::IllegalFunction - Unimplemented function code in request: {req:?}");
                future::ready(Err(Exception::IllegalFunction))
            }
        }
    }
}

fn register_read(
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, Exception> {
    let mut response_values = vec![0; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        if let Some(r) = registers.get(&reg_addr) {
            response_values[i as usize] = *r;
        } else {
            // TODO: Return a Modbus Exception response `IllegalDataAddress` https://github.com/slowtec/tokio-modbus/issues/165
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(Exception::IllegalDataAddress);
        }
    }

    Ok(response_values)
}

fn register_write(
    registers: &mut HashMap<u16, u16>,
    addr: u16,
    values: &[u16],
) -> Result<(), Exception> {
    for (i, value) in values.iter().enumerate() {
        let reg_addr = addr + i as u16;
        if let Some(r) = registers.get_mut(&reg_addr) {
            *r = *value;
        } else {
            // TODO: Return a Modbus Exception response `IllegalDataAddress` https://github.com/slowtec/tokio-modbus/issues/165
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(Exception::IllegalDataAddress);
        }
    }

    Ok(())
}

fn coil_read(
    coils: &HashMap<u16, bool>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<bool>, Exception> {
    let mut response_values: Vec<bool> = vec![false; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        if let Some(r) = coils.get(&reg_addr) {
            response_values[i as usize] = *r;
        } else {
            // TODO: Return a Modbus Exception response `IllegalDataAddress` https://github.com/slowtec/tokio-modbus/issues/165
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(Exception::IllegalDataAddress);
        }
    }

    Ok(response_values)
}

impl ModbusServer {
    pub fn new() -> Self {
        let mut input_registers = HashMap::new();
        let mut holding_registers = HashMap::new();
        let mut coils = HashMap::new();

        // Initialize coils
        for i in 0..1000 {
            coils.insert(i, false);
        }

        // Initialize holding registers
        for i in 0..1000 {
            holding_registers.insert(i, 0);
        }

        // Initialize input registers
        for i in 0..1000 {
            input_registers.insert(i, 0);
        }

        Self {
            input_registers: Arc::new(Mutex::new(input_registers)),
            holding_registers: Arc::new(Mutex::new(holding_registers)),
            coils: Arc::new(Mutex::new(coils)),
        }
    }
}