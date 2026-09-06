//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/usb-omap1.h
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
// Platform data for OMAP1 USB
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file "COPYING" in the main directory of this archive for
// more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_usb_config {
// Configure drivers according to the connectors on your board:
// - "A" connector (rectagular)
// ... for host/OHCI use, set "register_host".
// - "B" connector (squarish) or "Mini-B"
// ... for device/gadget use, set "register_dev".
// - "Mini-AB" connector (very similar to Mini-B)
// ... for OTG use as device OR host, initialize "otg"
//
    pub register_host:1: unsigned,
    pub register_dev:1: unsigned,
    pub /: *mut *mut u8 otg; / port number, 1-based: usb1 == 2,
    pub /: *const *const *const char extcon; / extcon device for OTG,
    pub hmc_mode: u8,
// implicitly true if otg:  host supports remote wakeup?
    pub rwc: u8,
// signaling pins used to talk to transceiver on usbN:
// 0 == usbN unused
// 2 == usb0-only, using internal transceiver
// 3 == 3 wire bidirectional
// 4 == 4 wire bidirectional
// 6 == 6 wire unidirectional (or TLL)
//
    pub pins: [u8; 3],
    pub udc_device: *mut platform_device,
    pub ohci_device: *mut platform_device,
    pub otg_device: *mut platform_device,
    pub is_device): *mut *mut u32 (usb0_init)(unsigned nwires, unsigned,
    pub nwires): *mut *mut u32 (usb1_init)(unsigned,
    pub alt_pingroup): *mut *mut u32 (usb2_init)(unsigned nwires, unsigned,
    pub (*ocpi_enable)(void): *mut c_int,
    pub (*lb_reset)(void): *mut c_void,
    pub on): *mut *mut int (transceiver_power)(int,
}
