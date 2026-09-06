//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pxa168_eth.h
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
// pxa168 ethernet platform device data definition file.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa168_eth_platform_data {
    pub port_number: c_int,
    pub phy_addr: c_int,
//
// If speed is 0, then speed and duplex are autonegotiated.
//
    pub /: *mut *mut int speed; / 0, SPEED_10, SPEED_100,
    pub /: *mut *mut int duplex; / DUPLEX_HALF or DUPLEX_FULL,
    pub intf: phy_interface_t,
//
// Override default RX/TX queue sizes if nonzero.
//
    pub rx_queue_size: c_int,
    pub tx_queue_size: c_int,
//
// init callback is used for board specific initialization
// e.g on Aspenite its used to initialize the PHY transceiver.
//
    pub (*init)(void): *mut c_int,
}
