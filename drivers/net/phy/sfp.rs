//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/sfp.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_quirk {
    pub vendor: *const c_char,
    pub part: *const c_char,
    pub caps): *mut sfp_module_caps,
    pub sfp): *mut *mut void (fixup)(struct sfp,
    pub part_prefix_match: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_socket_ops {
    pub sfp): *mut *mut void (attach)(struct sfp,
    pub sfp): *mut *mut void (detach)(struct sfp,
    pub sfp): *mut *mut void (start)(struct sfp,
    pub sfp): *mut *mut void (stop)(struct sfp,
    pub rate_kbd): *mut *mut *mut void (set_signal_rate)(struct sfp sfp, unsigned int,
    pub modinfo): *mut *mut *mut int (module_info)(struct sfp sfp, struct ethtool_modinfo,
    pub data): *mut u8,
    pub extack): *mut netlink_ext_ack,
}

extern "C" {
    pub fn sfp_add_phy(bus: *mut sfp_bus, phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn sfp_remove_phy(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_link_up(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_link_down(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_module_remove(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_module_start(bus: *mut sfp_bus) -> c_int;
}
extern "C" {
    pub fn sfp_module_stop(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_unregister_socket(bus: *mut sfp_bus);
}
