//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ssb/ssb_driver_mips.h
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
pub struct ssb_serial_port {
    pub regs: *mut c_void,
    pub clockspeed: c_ulong,
    pub irq: c_uint,
    pub baud_base: c_uint,
    pub reg_shift: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_pflash {
    pub present: bool,
    pub buswidth: u8,
    pub window: u32,
    pub window_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_sflash {
    pub present: bool,
    pub window: u32,
    pub blocksize: u32,
    pub numblocks: u16,
    pub size: u32,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_mipscore {
    pub dev: *mut ssb_device,
    pub nr_serial_ports: c_int,
    pub serial_ports: [ssb_serial_port; 4],
    pub pflash: ssb_pflash,

    pub sflash: ssb_sflash,

}

extern "C" {
    pub fn ssb_mipscore_init(mcore: *mut ssb_mipscore);
}
extern "C" {
    pub fn ssb_cpu_clock(mcore: *mut ssb_mipscore) -> u32;
}
extern "C" {
    pub fn ssb_mips_irq(dev: *mut ssb_device) -> c_uint;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_mipscore {
}

