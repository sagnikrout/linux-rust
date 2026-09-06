//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/davinci_emac.h
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


//
// TI DaVinci EMAC platform support
//
// Author: Kevin Hilman, Deep Root Systems, LLC
//
// 2007 (c) Deep Root Systems, LLC. This file is licensed under
// the terms of the GNU General Public License version 2. This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_platform_data {
    pub bus_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_platform_data {
    pub mac_addr: [c_char; ETH_ALEN],
    pub ctrl_reg_offset: u32,
    pub ctrl_mod_reg_offset: u32,
    pub ctrl_ram_offset: u32,
    pub hw_ram_addr: u32,
    pub ctrl_ram_size: u32,
//
// phy_id can be one of the following:
// - NULL		: use the first phy on the bus,
// - ""		: force to 100/full, no mdio control
// - "<bus>:<addr>"	: use the specified bus and phy
//
    pub phy_id: *const c_char,
    pub rmii_en: u8,
    pub version: u8,
    pub no_bd_ram: bool,
    pub (void): *mut *mut void (interrupt_enable),
    pub (void): *mut *mut void (interrupt_disable),
}
