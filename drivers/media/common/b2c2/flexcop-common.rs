//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/common/b2c2/flexcop-common.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Linux driver for digital TV devices equipped with B2C2 FlexcopII(b)/III
// flexcop-common.h - common header file for device-specific source files
// see flexcop.c for copyright information
//

pub const FC_MAX_FEED: c_int = 256;

// Steal from usb.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcop_dma {
    pub pdev: *mut pci_dev,
    pub cpu_addr0: *mut u8,
    pub dma_addr0: dma_addr_t,
    pub cpu_addr1: *mut u8,
    pub dma_addr1: dma_addr_t,
    pub /: *mut *mut u32 size; / size of each address in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcop_i2c_adapter {
    pub fc: *mut flexcop_device,
    pub i2c_adap: i2c_adapter,
    pub no_base_addr: u8,
    pub port: flexcop_i2c_port_t,
}

// Control structure for data definitions that are common to
// the B2C2-based PCI and USB devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcop_device {
// general
    pub /: *mut *mut *mut device dev; / for firmware_class,
pub const FC_STATE_DVB_INIT: c_uint = 0x01;
pub const FC_STATE_I2C_INIT: c_uint = 0x02;
pub const FC_STATE_FE_INIT: c_uint = 0x04;
    pub init_state: c_int,
// device information
    pub has_32_hw_pid_filter: c_int,
    pub rev: flexcop_revision_t,
    pub dev_type: flexcop_device_type_t,
    pub bus_type: flexcop_bus_t,
// dvb stuff
    pub dvb_adapter: dvb_adapter,
    pub fe: *mut dvb_frontend,
    pub dvbnet: dvb_net,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub ): *mut *mut int (fe_sleep) (struct dvb_frontend,
    pub fc_i2c_adap: [flexcop_i2c_adapter; 3],
    pub i2c_mutex: mutex,
    pub owner: *mut module,
// options and status
    pub extra_feedcount: c_int,
    pub feedcount: c_int,
    pub pid_filtering: c_int,
    pub fullts_streaming_state: c_int,
    pub skip_6_hw_pid_filter: c_int,
// bus specific callbacks
    pub flexcop_ibi_value): flexcop_ibi_register,,
    pub len): *mut *mut flexcop_access_op_t, u8 chipaddr, u8 addr, u8 buf, u16,
    pub int): *mut *mut *mut int (stream_control) (struct flexcop_device ,,
    pub extended): *mut *mut *mut int (get_mac_addr) (struct flexcop_device fc, int,
    pub bus_specific: *mut c_void,
}

// exported prototypes
// from flexcop.c
extern "C" {
    pub fn flexcop_pass_dmx_data(fc: *mut flexcop_device, buf: *mut u8, len: u32);
}
extern "C" {
    pub fn flexcop_pass_dmx_packets(fc: *mut flexcop_device, buf: *mut u8, no: u32);
}
extern "C" {
    pub fn flexcop_device_kfree(: *mut flexcop_device);
}
extern "C" {
    pub fn flexcop_device_initialize(: *mut flexcop_device) -> c_int;
}
extern "C" {
    pub fn flexcop_device_exit(fc: *mut flexcop_device);
}
extern "C" {
    pub fn flexcop_reset_block_300(fc: *mut flexcop_device);
}
// from flexcop-dma.c
extern "C" {
    pub fn flexcop_dma_free(dma: *mut flexcop_dma);
}
// from flexcop-eeprom.c
// the PCI part uses this call to get the MAC address, the USB part has its own
extern "C" {
    pub fn flexcop_eeprom_check_mac_addr(fc: *mut flexcop_device, extended: c_int) -> c_int;
}
// from flexcop-i2c.c
// the PCI part uses this a i2c_request callback, whereas the usb part has its own
// one. We have it in flexcop-i2c.c, because it is going via the actual
// I2C-channel of the flexcop.
//
// from flexcop-sram.c
extern "C" {
    pub fn flexcop_wan_set_speed(fc: *mut flexcop_device, s: flexcop_wan_speed_t);
}
// global prototypes for the flexcop-chip
// from flexcop-fe-tuner.c
extern "C" {
    pub fn flexcop_frontend_init(fc: *mut flexcop_device) -> c_int;
}
extern "C" {
    pub fn flexcop_frontend_exit(fc: *mut flexcop_device);
}
// from flexcop-i2c.c
extern "C" {
    pub fn flexcop_i2c_init(fc: *mut flexcop_device) -> c_int;
}
extern "C" {
    pub fn flexcop_i2c_exit(fc: *mut flexcop_device);
}
// from flexcop-sram.c
extern "C" {
    pub fn flexcop_sram_init(fc: *mut flexcop_device) -> c_int;
}
// from flexcop-misc.c
extern "C" {
    pub fn flexcop_determine_revision(fc: *mut flexcop_device);
}
// from flexcop-hw-filter.c
extern "C" {
    pub fn flexcop_hw_filter_init(fc: *mut flexcop_device);
}
extern "C" {
    pub fn flexcop_smc_ctrl(fc: *mut flexcop_device, onoff: c_int);
}
extern "C" {
    pub fn flexcop_set_mac_filter(fc: *mut flexcop_device, mac[6]: u8);
}
extern "C" {
    pub fn flexcop_mac_filter_ctrl(fc: *mut flexcop_device, onoff: c_int);
}
