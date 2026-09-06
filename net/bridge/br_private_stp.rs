//! Automatically rewritten from C Header to Rust Module
//! Source: net/bridge/br_private_stp.h
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
// Linux ethernet bridge
//
// Authors:
// Lennert Buytenhek		<buytenh@gnu.org>
//
pub const BPDU_TYPE_CONFIG: c_int = 0;
pub const BPDU_TYPE_TCN: c_uint = 0x80;
// IEEE 802.1D-1998 timer values

pub const BR_MIN_PATH_COST: c_int = 1;
pub const BR_MAX_PATH_COST: c_int = 65535;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_config_bpdu {
    pub topology_change:1: c_uint,
    pub topology_change_ack:1: c_uint,
    pub root: bridge_id,
    pub root_path_cost: c_int,
    pub bridge_id: bridge_id,
    pub port_id: port_id,
    pub message_age: c_int,
    pub max_age: c_int,
    pub hello_time: c_int,
    pub forward_delay: c_int,
}

// called under bridge lock
// br_stp.c
extern "C" {
    pub fn br_become_root_bridge(br: *mut net_bridge);
}
extern "C" {
    pub fn br_config_bpdu_generation(: *mut net_bridge);
}
extern "C" {
    pub fn br_configuration_update(: *mut net_bridge);
}
extern "C" {
    pub fn br_port_state_selection(: *mut net_bridge);
}
extern "C" {
    pub fn br_received_tcn_bpdu(p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_transmit_config(p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_transmit_tcn(br: *mut net_bridge);
}
extern "C" {
    pub fn br_topology_change_detection(br: *mut net_bridge);
}
extern "C" {
    pub fn __br_set_topology_change(br: *mut net_bridge, val: c_uchar);
}
// br_stp_bpdu.c
extern "C" {
    pub fn br_send_config_bpdu(: *mut net_bridge_port, : *mut br_config_bpdu);
}
extern "C" {
    pub fn br_send_tcn_bpdu(: *mut net_bridge_port);
}
