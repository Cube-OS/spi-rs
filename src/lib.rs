//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// A generalised HAL for communicating over SPI

use std::io::Result;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

/// High level read/write trait for SPI connections to implement
pub trait Stream {
    /// Write data to a SPI device
    /// 
    /// # Argument
    /// 
    /// `data` - Data to write
    fn write(&mut self, data: &[u8]) -> Result<()>;

    /// Read data from a SPI device
    /// 
    /// # Argument
    /// 
    /// `len` - Amount of Data to read
    fn read(&mut self, len: usize) -> Result<Vec<u8>>;

    /// Write data to a SPI device and read the results
    /// 
    /// # Argument
    /// 
    /// `data` - Data to write
    fn transfer(&self, data: &[u8]) -> Result<Vec<u8>>;
}

/// Struct for communicating with an SPI device
pub struct Connection {
    stream: Box<dyn Stream + Send>,
}

impl Connection {
    /// SPI connection constructor
    pub fn new(stream: Box<dyn Stream + Send>) -> Self {
        Self { stream }
    }

    /// Convenience constructor for creating a Connection with a SPIDEV
    /// 
    /// # Arguments
    /// 
    /// `path` - Path to SPI device
    /// `bpw` - Bits per word
    /// `max_speed` - Max speed in Hz
    /// `mode` - SPI Mode
    pub fn from_path(
        path: String,
        bpw: u8,
        max_speed: u32,
        mode: SpiModeFlags,
    ) -> Result<Connection> {        
        Ok(Connection {
            stream: Box::new(SpiStream::new(path, bpw, max_speed, mode)?)
        })
    }

    /// Write data to a SPI device
    /// 
    /// # Argument
    /// 
    /// `data` - Data to write
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write(data)
    }

    /// Read data from a SPI device
    /// 
    /// # Argument
    /// 
    /// `len` - Amount of Data to read
    pub fn read(&mut self, len: usize) -> Result<Vec<u8>> {
        self.stream.read(len)
    }

    /// Write data to a SPI device and read the results
    /// 
    /// # Argument
    /// 
    /// `data` - Data to write
    pub fn transfer(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.stream.transfer(data)
    }
}

pub struct SpiStream {
    spidev: spidev::Spidev,
}
impl SpiStream {
    fn new(
        path: String,
        bpw: u8,
        max_speed: u32,
        mode: SpiModeFlags,
    ) -> Result<Self> {
        let mut spi = Spidev::open(path)?;
        let options = SpidevOptions::new()
            .bits_per_word(bpw)
            .max_speed_hz(max_speed)
            .mode(mode)
            .build();
        spi.configure(&options)?;
        Ok(SpiStream{
            spidev: spi,
        })
    }
}
// Read and write implementations for the SpiStream
impl Stream for SpiStream {
    fn write(&mut self, data: &[u8]) -> Result<()> {
        match self.spidev.write(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8;len];
        self.spidev.read(&mut buf)?;
        Ok(buf)
    }

    fn transfer(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut buf = vec![0u8;data.len()];
        let mut transfer = SpidevTransfer::read_write(data, &mut buf);
        self.spidev.transfer(&mut transfer)?;
        Ok(buf)
    }
}
