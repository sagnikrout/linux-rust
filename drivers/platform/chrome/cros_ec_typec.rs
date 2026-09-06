//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/chrome/cros_ec_typec.h
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


// SPDX-License-Identifier: GPL-2.0-only

// Supported alt modes.
// Container for altmode pointer nodes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_typec_altmode_node {
    pub amode: *mut typec_altmode,
    pub list: list_head,
}

// Platform-specific data for the Chrome OS EC Type C controller.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_typec_data {
    pub dev: *mut device,
    pub ec: *mut cros_ec_device,
    pub num_ports: c_int,
    pub pd_ctrl_ver: c_uint,
// Array of ports, indexed by port number.
    pub ports: [*mut cros_typec_port; EC_USB_PD_MAX_PORTS],
    pub nb: notifier_block,
    pub port_work: work_struct,
    pub typec_cmd_supported: bool,
    pub needs_mux_ack: bool,
    pub ap_driven_altmode: bool,
}

// Per port data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_typec_port {
    pub port: *mut typec_port,
    pub port_num: c_int,
// Initial capabilities for the port.
    pub caps: typec_capability,
    pub partner: *mut typec_partner,
    pub cable: *mut typec_cable,
// SOP' plug.
    pub plug: *mut typec_plug,
// Port partner PD identity info.
    pub p_identity: usb_pd_identity,
// Port cable PD identity info.
    pub c_identity: usb_pd_identity,
    pub ori_sw: *mut typec_switch,
    pub mux: *mut typec_mux,
    pub retimer: *mut typec_retimer,
    pub role_sw: *mut usb_role_switch,
// Variables keeping track of switch state.
    pub state: typec_mux_state,
    pub mux_flags: u8,
    pub role: u8,
    pub port_altmode: [*mut typec_altmode; CROS_EC_ALTMODE_MAX],
// Flag indicating that PD partner discovery data parsing is completed.
    pub sop_disc_done: bool,
    pub sop_prime_disc_done: bool,
    pub disc_data: *mut ec_response_typec_discovery,
    pub partner_mode_list: list_head,
    pub plug_mode_list: list_head,
// PDO-related structs
    pub partner_pd: *mut usb_power_delivery,
    pub partner_src_caps: *mut usb_power_delivery_capabilities,
    pub partner_sink_caps: *mut usb_power_delivery_capabilities,
    pub typec_data: *mut cros_typec_data,
}
