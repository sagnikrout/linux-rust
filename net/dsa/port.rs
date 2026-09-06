//! Automatically rewritten from C Header to Rust Module
//! Source: net/dsa/port.h
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

extern "C" {
    pub fn dsa_port_supports_hwtstamp(dp: *mut dsa_port) -> bool;
}
extern "C" {
    pub fn dsa_port_set_state(dp: *mut dsa_port, state: u8, do_fast_age: bool) -> c_int;
}
extern "C" {
    pub fn dsa_port_enable_rt(dp: *mut dsa_port, phy: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn dsa_port_enable(dp: *mut dsa_port, phy: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn dsa_port_disable_rt(dp: *mut dsa_port);
}
extern "C" {
    pub fn dsa_port_disable(dp: *mut dsa_port);
}
extern "C" {
    pub fn dsa_port_pre_bridge_leave(dp: *mut dsa_port, br: *mut net_device);
}
extern "C" {
    pub fn dsa_port_bridge_leave(dp: *mut dsa_port, br: *mut net_device);
}
extern "C" {
    pub fn dsa_port_pre_lag_leave(dp: *mut dsa_port, lag_dev: *mut net_device);
}
extern "C" {
    pub fn dsa_port_lag_leave(dp: *mut dsa_port, lag_dev: *mut net_device);
}
extern "C" {
    pub fn dsa_port_skip_vlan_configuration(dp: *mut dsa_port) -> bool;
}
extern "C" {
    pub fn dsa_port_ageing_time(dp: *mut dsa_port, ageing_clock: clock_t) -> c_int;
}
extern "C" {
    pub fn dsa_port_mtu_change(dp: *mut dsa_port, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn dsa_port_fdb_dump(dp: *mut dsa_port, cb: *mut dsa_fdb_dump_cb_t, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dsa_port_phylink_create(dp: *mut dsa_port) -> c_int;
}
extern "C" {
    pub fn dsa_port_phylink_destroy(dp: *mut dsa_port);
}
extern "C" {
    pub fn dsa_shared_port_link_register_of(dp: *mut dsa_port) -> c_int;
}
extern "C" {
    pub fn dsa_shared_port_link_unregister_of(dp: *mut dsa_port);
}
extern "C" {
    pub fn dsa_port_hsr_leave(dp: *mut dsa_port, hsr: *mut net_device);
}
extern "C" {
    pub fn dsa_port_tag_8021q_vlan_add(dp: *mut dsa_port, vid: u16, broadcast: bool) -> c_int;
}
extern "C" {
    pub fn dsa_port_tag_8021q_vlan_del(dp: *mut dsa_port, vid: u16, broadcast: bool);
}
extern "C" {
    pub fn dsa_port_set_host_flood(dp: *mut dsa_port, uc: bool, mc: bool);
}
