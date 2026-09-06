//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib9000.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib9000_config {
    pub dvbt_mode: u8,
    pub output_mpeg2_in_188_bytes: u8,
    pub hostbus_diversity: u8,
    pub bw: *mut dibx000_bandwidth_config,
    pub if_drives: u16,
    pub timing_frequency: u32,
    pub xtal_clock_khz: u32,
    pub vcxo_timer: u32,
    pub demod_clock_khz: u32,
    pub microcode_B_fe_buffer: *const u8,
    pub microcode_B_fe_size: u32,
    pub gpio_function: [dibGPIOFunction; 2],
    pub subband: dibSubbandSelection,
    pub output_mode: u8,
}

pub const DEFAULT_DIB9000_I2C_ADDRESS: c_int = 18;

extern "C" {
    pub fn dib9000_i2c_enumeration(host: *mut i2c_adapter, no_of_demods: c_int, default_addr: u8, first_addr: u8) -> c_int;
}
extern "C" {
    pub fn dib9000_set_gpio(fe: *mut dvb_frontend, num: u8, dir: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn dib9000_fw_pid_filter_ctrl(fe: *mut dvb_frontend, onoff: u8) -> c_int;
}
extern "C" {
    pub fn dib9000_fw_pid_filter(fe: *mut dvb_frontend, id: u8, pid: u16, onoff: u8) -> c_int;
}
extern "C" {
    pub fn dib9000_firmware_post_pll_init(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn dib9000_set_slave_frontend(fe: *mut dvb_frontend, fe_slave: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn dib9000_set_i2c_adapter(fe: *mut dvb_frontend, i2c: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn dib9000_fw_set_component_bus_speed(fe: *mut dvb_frontend, speed: u16) -> c_int;
}

