//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-amd-mp2.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// AMD MP2 I2C adapter driver
//
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Elie Morisse <syniurge@gmail.com>
//

pub const PCI_DEVICE_ID_AMD_MP2: c_uint = 0x15E6;
// MP2 C2P Message Registers
// MP2 P2C Message Registers
// Command register data structures

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_cmd {
    i2c_read = 0,
    i2c_write,
    i2c_enable,
    i2c_disable,
    number_of_sensor_discovered,
    is_mp2_active,
    invalid_cmd = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum speed_enum {
    speed100k = 0,
    speed400k = 1,
    speed1000k = 2,
    speed1400k = 3,
    speed3400k = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_type {
    use_dram = 0,
    use_c2pmsg = 1,
}

//
// union i2c_cmd_base : bit access of C2P commands
// @i2c_cmd: bit 0..3 i2c R/W command
// @bus_id: bit 4..7 i2c bus index
// @slave_addr: bit 8..15 slave address
// @length: bit 16..27 read/write length
// @i2c_speed: bit 28..30 bus speed
// @mem_type: bit 31 0-DRAM; 1-C2P msg o/p
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union i2c_cmd_base {
    pub ul: u32,
    pub 4: i2c_cmd i2c_cmd :,
    pub 4: u8 bus_id :,
    pub 8: u32 slave_addr :,
    pub 12: u32 length :,
    pub 3: speed_i2c_speed :,
    pub 1: mem_type mem_type :,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum response_type {
    invalid_response = 0,
    command_success = 1,
    command_failed = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum status_type {
    i2c_readcomplete_event = 0,
    i2c_readfail_event = 1,
    i2c_writecomplete_event = 2,
    i2c_writefail_event = 3,
    i2c_busenable_complete = 4,
    i2c_busenable_failed = 5,
    i2c_busdisable_complete = 6,
    i2c_busdisable_failed = 7,
    invalid_data_length = 8,
    invalid_slave_address = 9,
    invalid_i2cbus_id = 10,
    invalid_dram_addr = 11,
    invalid_command = 12,
    mp2_active = 13,
    numberof_sensors_discovered_resp = 14,
    i2c_bus_notinitialized
}

//
// union i2c_event : bit access of P2C events
// @response: bit 0..1 i2c response type
// @status: bit 2..6 status_type
// @mem_type: bit 7 0-DRAM; 1-C2P msg o/p
// @bus_id: bit 8..11 i2c bus id
// @length: bit 12..23 message length
// @slave_addr: bit 24-31 slave address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union i2c_event {
    pub ul: u32,
    pub 2: response_type response :,
    pub 5: status_type status :,
    pub 1: mem_type mem_type :,
    pub 4: u8 bus_id :,
    pub 12: u32 length :,
    pub 8: u32 slave_addr :,
    pub r: },
}

//
// struct amd_i2c_common - per bus/i2c adapter context, shared
// between the pci and the platform driver
// @eventval: MP2 event value set by the IRQ handler
// @mp2_dev: MP2 pci device this adapter is part of
// @msg: i2c message
// @cmd_completion: function called by the IRQ handler to signal
// the platform driver
// @reqcmd: requested i2c command type
// @cmd_success: set to true if the MP2 responded to a command with
// the expected status and response type
// @bus_id: bus index
// @i2c_speed: i2c bus speed determined by the slowest slave
// @dma_buf: if msg length > 32, holds the DMA buffer virtual address
// @dma_addr: if msg length > 32, holds the DMA buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_i2c_common {
    pub eventval: i2c_event,
    pub mp2_dev: *mut amd_mp2_dev,
    pub msg: *mut i2c_msg,
    pub i2c_common): *mut *mut void (cmd_completion)(struct amd_i2c_common,
    pub reqcmd: i2c_cmd,
    pub cmd_success: u8,
    pub bus_id: u8,
    pub i2c_speed: speed_enum,
    pub dma_buf: *mut u8,
    pub dma_addr: dma_addr_t,

    pub i2c_common): *mut *mut int (suspend)(struct amd_i2c_common,
    pub i2c_common): *mut *mut int (resume)(struct amd_i2c_common,

}

//
// struct amd_mp2_dev - per PCI device context
// @pci_dev: PCI driver node
// @busses: MP2 devices may have up to two busses,
// each bus corresponding to an i2c adapter
// @mmio: iommapped registers
// @c2p_lock: controls access to the C2P mailbox shared between
// the two adapters
// @c2p_lock_busid: id of the adapter which locked c2p_lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mp2_dev {
    pub pci_dev: *mut pci_dev,
    pub busses: [*mut amd_i2c_common; 2],
    pub mmio: *mut void __iomem,
    pub c2p_lock: mutex,
    pub c2p_lock_busid: u8,
    pub probed: c_uint,
    pub dev_irq: c_int,
}

// PCIe communication driver
extern "C" {
    pub fn amd_mp2_rw(i2c_common: *mut amd_i2c_common, reqcmd: i2c_cmd) -> c_int;
}
extern "C" {
    pub fn amd_mp2_bus_enable_set(i2c_common: *mut amd_i2c_common, enable: bool) -> c_int;
}
extern "C" {
    pub fn amd_mp2_process_event(i2c_common: *mut amd_i2c_common);
}
extern "C" {
    pub fn amd_mp2_rw_timeout(i2c_common: *mut amd_i2c_common);
}
extern "C" {
    pub fn amd_mp2_register_cb(i2c_common: *mut amd_i2c_common) -> c_int;
}
extern "C" {
    pub fn amd_mp2_unregister_cb(i2c_common: *mut amd_i2c_common) -> c_int;
}
