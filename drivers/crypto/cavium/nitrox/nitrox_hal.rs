//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/nitrox/nitrox_hal.h
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

extern "C" {
    pub fn nitrox_config_aqm_rings(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_aqm_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_emu_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_pkt_input_rings(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_pkt_solicit_ports(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_nps_core_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_nps_pkt_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_pom_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_rand_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_efl_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_bmi_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_bmo_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn nitrox_config_lbc_unit(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn invalidate_lbc(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn enable_aqm_ring(ndev: *mut nitrox_device, qno: c_int);
}
extern "C" {
    pub fn enable_pkt_input_ring(ndev: *mut nitrox_device, ring: c_int);
}
extern "C" {
    pub fn enable_pkt_solicit_port(ndev: *mut nitrox_device, port: c_int);
}
extern "C" {
    pub fn config_nps_core_vfcfg_mode(ndev: *mut nitrox_device, mode: vf_mode);
}
extern "C" {
    pub fn nitrox_get_hwinfo(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn enable_pf2vf_mbox_interrupts(ndev: *mut nitrox_device);
}
extern "C" {
    pub fn disable_pf2vf_mbox_interrupts(ndev: *mut nitrox_device);
}
