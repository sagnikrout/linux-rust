//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps6586x.h
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
pub const TPS6586X_SLEW_RATE_INSTANTLY: c_uint = 0x00;
pub const TPS6586X_SLEW_RATE_110UV: c_uint = 0x01;
pub const TPS6586X_SLEW_RATE_220UV: c_uint = 0x02;
pub const TPS6586X_SLEW_RATE_440UV: c_uint = 0x03;
pub const TPS6586X_SLEW_RATE_880UV: c_uint = 0x04;
pub const TPS6586X_SLEW_RATE_1760UV: c_uint = 0x05;
pub const TPS6586X_SLEW_RATE_3520UV: c_uint = 0x06;
pub const TPS6586X_SLEW_RATE_7040UV: c_uint = 0x07;
pub const TPS6586X_SLEW_RATE_SET: c_uint = 0x08;
pub const TPS6586X_SLEW_RATE_MASK: c_uint = 0x07;
// VERSION CRC
pub const TPS658621A: c_uint = 0x15;
pub const TPS658621CD: c_uint = 0x2c;
pub const TPS658623: c_uint = 0x1b;
pub const TPS658624: c_uint = 0x0a;
pub const TPS658640: c_uint = 0x01;
pub const TPS658640v2: c_uint = 0x02;
pub const TPS658643: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6586x_settings {
    pub slew_rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6586x_subdev_info {
    pub id: c_int,
    pub name: *const c_char,
    pub platform_data: *mut c_void,
    pub of_node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6586x_platform_data {
    pub num_subdevs: c_int,
    pub subdevs: *mut tps6586x_subdev_info,
    pub gpio_base: c_int,
    pub irq_base: c_int,
    pub pm_off: bool,
    pub reg_init_data: [*mut regulator_init_data; TPS6586X_ID_MAX_REGULATOR],
}

//
// NOTE: the functions below are not intended for use outside
// of the TPS6586X sub-device drivers
//
extern "C" {
    pub fn tps6586x_write(dev: *mut device, reg: c_int, val: u8) -> c_int;
}
extern "C" {
    pub fn tps6586x_writes(dev: *mut device, reg: c_int, len: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn tps6586x_read(dev: *mut device, reg: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn tps6586x_reads(dev: *mut device, reg: c_int, len: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn tps6586x_set_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
}
extern "C" {
    pub fn tps6586x_clr_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
}
extern "C" {
    pub fn tps6586x_irq_get_virq(dev: *mut device, irq: c_int) -> c_int;
}
extern "C" {
    pub fn tps6586x_get_version(dev: *mut device) -> c_int;
}
