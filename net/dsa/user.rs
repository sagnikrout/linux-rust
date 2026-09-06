//! Automatically rewritten from C Header to Rust Module
//! Source: net/dsa/user.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_user_priv {
// Copy of CPU port xmit for faster access in user transmit hot path
    pub dev): *mut net_device,
    pub gcells: gro_cells,
// DSA port data, such as switch, port index, etc.
    pub dp: *mut dsa_port,

    pub netpoll: *mut netpoll,

// TC context
    pub mall_tc_list: list_head,
}

extern "C" {
    pub fn dsa_user_mii_bus_init(ds: *mut dsa_switch);
}
extern "C" {
    pub fn dsa_user_create(dp: *mut dsa_port) -> c_int;
}
extern "C" {
    pub fn dsa_user_destroy(user_dev: *mut net_device);
}
extern "C" {
    pub fn dsa_user_suspend(user_dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dsa_user_resume(user_dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dsa_user_register_notifier() -> c_int;
}
extern "C" {
    pub fn dsa_user_unregister_notifier();
}
extern "C" {
    pub fn dsa_user_host_uc_install(dev: *mut net_device, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn dsa_user_host_uc_uninstall(dev: *mut net_device);
}
extern "C" {
    pub fn dsa_user_sync_ha(dev: *mut net_device);
}
extern "C" {
    pub fn dsa_user_unsync_ha(dev: *mut net_device);
}
extern "C" {
    pub fn dsa_user_setup_tagger(user: *mut net_device);
}
extern "C" {
    pub fn dsa_user_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn dsa_port_to_conduit(_arg: dp) -> return;
}
