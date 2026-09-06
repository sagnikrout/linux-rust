//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/i2c.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_i2c_device {
    pub list: list_head,
    pub /: *mut *mut *mut snd_i2c_bus bus; / I2C bus,
    pub /: *mut *mut char name[32]; / some useful device name,
    pub /: *mut *mut unsigned short flags; / device flags,
    pub /: *mut *mut unsigned short addr; / device address (might be 10-bit),
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub device): *mut *mut void (private_free)(struct snd_i2c_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_i2c_bit_ops {
    pub /: *mut *mut *mut *mut void (start)(struct snd_i2c_bus bus); / transfer start,
    pub /: *mut *mut *mut *mut void (stop)(struct snd_i2c_bus bus); / transfer stop,
    pub /: *mut *mut *mut *mut void (direction)(struct snd_i2c_bus bus, int clock, int data); / set line direction (0 = write, 1 = read),
    pub data): *mut *mut *mut void (setlines)(struct snd_i2c_bus bus, int clock, int,
    pub bus): *mut *mut int (getclock)(struct snd_i2c_bus,
    pub ack): *mut *mut *mut int (getdata)(struct snd_i2c_bus bus, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_i2c_ops {
    pub count): *mut *mut *mut *mut int (sendbytes)(struct snd_i2c_device device, unsigned char bytes, int,
    pub count): *mut *mut *mut *mut int (readbytes)(struct snd_i2c_device device, unsigned char bytes, int,
    pub addr): *mut *mut *mut int (probeaddr)(struct snd_i2c_bus bus, unsigned short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_i2c_bus {
    pub /: *mut *mut *mut snd_card card; / card which I2C belongs to,
    pub /: *mut *mut char name[32]; / some useful label,
    pub lock_mutex: mutex,
    pub /: *mut *mut *mut snd_i2c_bus master; / master bus when SCK/SCL is shared,
    pub /: *mut *mut list_head buses; / master: slave buses sharing SCK/SCL, slave: link list,
    pub /: *mut *mut list_head devices; / attached devices to this bus,
    pub bit: *mut snd_i2c_bit_ops,
    pub ops: *mut c_void,
    pub /: *mut *mut } hw_ops; / lowlevel operations,
    pub /: *const *const *const snd_i2c_ops ops; / midlevel operations,
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub bus): *mut *mut void (private_free)(struct snd_i2c_bus,
}

extern "C" {
    pub fn snd_i2c_device_free(device: *mut snd_i2c_device) -> c_int;
}
extern "C" {
    pub fn snd_i2c_sendbytes(device: *mut snd_i2c_device, bytes: *mut c_uchar, count: c_int) -> c_int;
}
extern "C" {
    pub fn snd_i2c_readbytes(device: *mut snd_i2c_device, bytes: *mut c_uchar, count: c_int) -> c_int;
}
extern "C" {
    pub fn snd_i2c_probeaddr(bus: *mut snd_i2c_bus, addr: c_ushort) -> c_int;
}
