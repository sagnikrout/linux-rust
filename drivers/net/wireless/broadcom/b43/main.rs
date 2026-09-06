//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/main.h
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

// Magic helper macro to pad structures. Ignore those above. It's magic.

// Logmessage verbosity levels. Update the b43_modparam_verbose helptext, if
// you add or remove levels.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_verbosity {
    B43_VERBOSITY_ERROR,
    B43_VERBOSITY_WARN,
    B43_VERBOSITY_INFO,
    B43_VERBOSITY_DEBUG,
    __B43_VERBOSITY_AFTERLAST, /* keep last */

    B43_VERBOSITY_MAX = __B43_VERBOSITY_AFTERLAST - 1,

    B43_VERBOSITY_DEFAULT = B43_VERBOSITY_DEBUG,

    B43_VERBOSITY_DEFAULT = B43_VERBOSITY_INFO,

}

extern "C" {
    pub fn b43_tsf_read(dev: *mut b43_wldev, tsf: *mut *mut u64);
}
extern "C" {
    pub fn b43_tsf_write(dev: *mut b43_wldev, tsf: u64);
}
extern "C" {
    pub fn b43_shm_read32(dev: *mut b43_wldev, routing: u16, offset: u16) -> u32;
}
extern "C" {
    pub fn b43_shm_read16(dev: *mut b43_wldev, routing: u16, offset: u16) -> u16;
}
extern "C" {
    pub fn b43_shm_write32(dev: *mut b43_wldev, routing: u16, offset: u16, value: u32);
}
extern "C" {
    pub fn b43_shm_write16(dev: *mut b43_wldev, routing: u16, offset: u16, value: u16);
}
extern "C" {
    pub fn b43_hf_read(dev: *mut b43_wldev) -> u64;
}
extern "C" {
    pub fn b43_hf_write(dev: *mut b43_wldev, value: u64);
}
extern "C" {
    pub fn b43_dummy_transmission(dev: *mut b43_wldev, ofdm: bool, pa_on: bool);
}
extern "C" {
    pub fn b43_wireless_core_reset(dev: *mut b43_wldev, gmode: bool);
}
extern "C" {
    pub fn b43_controller_restart(dev: *mut b43_wldev, reason: *const c_char);
}

extern "C" {
    pub fn b43_power_saving_ctl_bits(dev: *mut b43_wldev, ps_flags: c_uint);
}
extern "C" {
    pub fn b43_wireless_core_phy_pll_reset(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_mac_suspend(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_mac_enable(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_mac_phy_clock_set(dev: *mut b43_wldev, on: bool);
}
extern "C" {
    pub fn b43_mac_switch_freq(dev: *mut b43_wldev, spurmode: u8);
}
extern "C" {
    pub fn b43_do_release_fw(fw: *mut b43_firmware_file);
}
