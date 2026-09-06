//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/prestera/prestera_hw.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2019-2020 Marvell International Ltd. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_accept_frm_type {
    PRESTERA_ACCEPT_FRAME_TYPE_TAGGED,
    PRESTERA_ACCEPT_FRAME_TYPE_UNTAGGED,
    PRESTERA_ACCEPT_FRAME_TYPE_ALL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_fdb_flush_mode {
    PRESTERA_FDB_FLUSH_MODE_DYNAMIC = BIT(0),
    PRESTERA_FDB_FLUSH_MODE_STATIC = BIT(1),
    PRESTERA_FDB_FLUSH_MODE_ALL = PRESTERA_FDB_FLUSH_MODE_DYNAMIC
    | PRESTERA_FDB_FLUSH_MODE_STATIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_hw_cpu_code_cnt_t {
    PRESTERA_HW_CPU_CODE_CNT_TYPE_DROP = 0,
    PRESTERA_HW_CPU_CODE_CNT_TYPE_TRAP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_hw_vtcam_direction_t {
    PRESTERA_HW_VTCAM_DIR_INGRESS = 0,
    PRESTERA_HW_VTCAM_DIR_EGRESS = 1,
}

// Switch API
extern "C" {
    pub fn prestera_hw_switch_init(sw: *mut prestera_switch) -> c_int;
}
extern "C" {
    pub fn prestera_hw_switch_fini(sw: *mut prestera_switch);
}
extern "C" {
    pub fn prestera_hw_switch_ageing_set(sw: *mut prestera_switch, ageing_ms: u32) -> c_int;
}
extern "C" {
    pub fn prestera_hw_switch_mac_set(sw: *mut prestera_switch, mac: *const c_char) -> c_int;
}
// Port API
extern "C" {
    pub fn prestera_hw_port_mtu_set(port: *const prestera_port, mtu: u32) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_mtu_get(port: *const prestera_port, mtu: *mut u32) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_mac_set(port: *const prestera_port, mac: *const c_char) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_mac_get(port: *const prestera_port, mac: *mut c_char) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_type_get(port: *const prestera_port, type: *mut u8) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_autoneg_restart(port: *mut prestera_port) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_speed_get(port: *const prestera_port, speed: *mut u32) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_learning_set(port: *mut prestera_port, enable: bool) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_uc_flood_set(port: *const prestera_port, flood: bool) -> c_int;
}
extern "C" {
    pub fn prestera_hw_port_mc_flood_set(port: *const prestera_port, flood: bool) -> c_int;
}
// Vlan API
extern "C" {
    pub fn prestera_hw_vlan_create(sw: *mut prestera_switch, vid: u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_vlan_delete(sw: *mut prestera_switch, vid: u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_vlan_port_vid_set(port: *mut prestera_port, vid: u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_vlan_port_stp_set(port: *mut prestera_port, vid: u16, state: u8) -> c_int;
}
// FDB API
extern "C" {
    pub fn prestera_hw_fdb_flush_port(port: *mut prestera_port, mode: u32) -> c_int;
}
extern "C" {
    pub fn prestera_hw_fdb_flush_vlan(sw: *mut prestera_switch, vid: u16, mode: u32) -> c_int;
}
// Bridge API
extern "C" {
    pub fn prestera_hw_bridge_create(sw: *mut prestera_switch, bridge_id: *mut u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_bridge_delete(sw: *mut prestera_switch, bridge_id: u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_bridge_port_add(port: *mut prestera_port, bridge_id: u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_bridge_port_delete(port: *mut prestera_port, bridge_id: u16) -> c_int;
}
// vTCAM API
extern "C" {
    pub fn prestera_hw_vtcam_destroy(sw: *mut prestera_switch, vtcam_id: u32) -> c_int;
}
// Counter API
extern "C" {
    pub fn prestera_hw_counter_trigger(sw: *mut prestera_switch, block_id: u32) -> c_int;
}
extern "C" {
    pub fn prestera_hw_counter_abort(sw: *mut prestera_switch) -> c_int;
}
// SPAN API
extern "C" {
    pub fn prestera_hw_span_get(port: *const prestera_port, span_id: *mut u8) -> c_int;
}
extern "C" {
    pub fn prestera_hw_span_unbind(port: *const prestera_port, ingress: bool) -> c_int;
}
extern "C" {
    pub fn prestera_hw_span_release(sw: *mut prestera_switch, span_id: u8) -> c_int;
}
// Router API
// Virtual Router API
extern "C" {
    pub fn prestera_hw_vr_create(sw: *mut prestera_switch, vr_id: *mut u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_vr_delete(sw: *mut prestera_switch, vr_id: u16) -> c_int;
}
// LPM PI
// NH API
// Event handlers
// RX/TX
// LAG API
extern "C" {
    pub fn prestera_hw_lag_member_add(port: *mut prestera_port, lag_id: u16) -> c_int;
}
extern "C" {
    pub fn prestera_hw_lag_member_del(port: *mut prestera_port, lag_id: u16) -> c_int;
}
// HW trap/drop counters API
// Policer API
// Flood domain / MDB API
extern "C" {
    pub fn prestera_hw_flood_domain_create(domain: *mut prestera_flood_domain) -> c_int;
}
extern "C" {
    pub fn prestera_hw_flood_domain_destroy(domain: *mut prestera_flood_domain) -> c_int;
}
extern "C" {
    pub fn prestera_hw_flood_domain_ports_set(domain: *mut prestera_flood_domain) -> c_int;
}
extern "C" {
    pub fn prestera_hw_flood_domain_ports_reset(domain: *mut prestera_flood_domain) -> c_int;
}
extern "C" {
    pub fn prestera_hw_mdb_create(mdb: *mut prestera_mdb_entry) -> c_int;
}
extern "C" {
    pub fn prestera_hw_mdb_destroy(mdb: *mut prestera_mdb_entry) -> c_int;
}
