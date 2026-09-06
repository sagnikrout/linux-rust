//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/typec/class.h
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
pub struct typec_plug {
    pub dev: device,
    pub index: typec_plug_index,
    pub mode_ids: ida,
    pub num_altmodes: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_cable {
    pub dev: device,
    pub type: typec_plug_type,
    pub identity: *mut usb_pd_identity,
    pub active:1: c_uint,
    pub /: *mut *mut u16 pd_revision; / 0300H = "3.0",
    pub svdm_version: usb_pd_svdm_ver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_partner {
    pub dev: device,
    pub usb_pd:1: c_uint,
    pub identity: *mut usb_pd_identity,
    pub accessory: typec_accessory,
    pub mode_ids: ida,
    pub num_altmodes: c_int,
    pub /: *mut *mut u16 pd_revision; / 0300H = "3.0",
    pub svdm_version: usb_pd_svdm_ver,
    pub usb_mode: usb_mode,
    pub usb_capability: u8,
    pub pd: *mut usb_power_delivery,
    pub sel: *mut mode_selection,
    pub dev): *mut *mut *mut void (attach)(struct typec_partner partner, struct device,
    pub dev): *mut *mut *mut void (deattach)(struct typec_partner partner, struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_port {
    pub id: c_uint,
    pub dev: device,
    pub mode_ids: ida,
    pub pd: *mut usb_power_delivery,
    pub prefer_role: c_int,
    pub data_role: typec_data_role,
    pub pwr_role: typec_role,
    pub vconn_role: typec_role,
    pub pwr_opmode: typec_pwr_opmode,
    pub port_type: typec_port_type,
    pub usb_mode: usb_mode,
    pub port_type_lock: mutex,
    pub partner_link_lock: mutex,
    pub orientation: typec_orientation,
    pub mode_control: bool,
    pub sw: *mut typec_switch,
    pub mux: *mut typec_mux,
    pub retimer: *mut typec_retimer,
    pub cap: *const typec_capability,
    pub ops: *const typec_operations,
    pub con: typec_connector,
//
// REVISIT: Only USB devices for now. If there are others, these need to
// be converted into a list.
//
// NOTE: These may be registered first before the typec_partner, so they
// will always have to be kept here instead of struct typec_partner.
//
    pub usb2_dev: *mut device,
    pub usb3_dev: *mut device,
}

extern "C" {
    pub fn typec_link_ports(connector: *mut typec_port) -> c_int;
}
extern "C" {
    pub fn typec_unlink_ports(connector: *mut typec_port);
}

