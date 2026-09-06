//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mdio-bitbang.h
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
pub struct mdiobb_ops {
    pub owner: *mut module,
// Set the Management Data Clock high if level is one,
// low if level is zero.
//
    pub level): *mut *mut *mut void (set_mdc)(struct mdiobb_ctrl ctrl, int,
// Configure the Management Data I/O pin as an input if
// "output" is zero, or an output if "output" is one.
//
    pub output): *mut *mut *mut void (set_mdio_dir)(struct mdiobb_ctrl ctrl, int,
// Set the Management Data I/O pin high if value is one,
// low if "value" is zero.  This may only be called
// when the MDIO pin is configured as an output.
//
    pub value): *mut *mut *mut void (set_mdio_data)(struct mdiobb_ctrl ctrl, int,
// Retrieve the state Management Data I/O pin.
    pub ctrl): *mut *mut int (get_mdio_data)(struct mdiobb_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdiobb_ctrl {
    pub ops: *const mdiobb_ops,
    pub override_op_c22: c_uint,
    pub op_c22_read: u8,
    pub op_c22_write: u8,
}

extern "C" {
    pub fn mdiobb_read_c22(bus: *mut mii_bus, phy: c_int, reg: c_int) -> c_int;
}
extern "C" {
    pub fn mdiobb_write_c22(bus: *mut mii_bus, phy: c_int, reg: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn mdiobb_read_c45(bus: *mut mii_bus, devad: c_int, phy: c_int, reg: c_int) -> c_int;
}
extern "C" {
    pub fn mdiobb_write_c45(bus: *mut mii_bus, devad: c_int, phy: c_int, reg: c_int, val: u16) -> c_int;
}
// The returned bus is not yet registered with the phy layer.
// The bus must already have been unregistered.
extern "C" {
    pub fn free_mdio_bitbang(bus: *mut mii_bus);
}
