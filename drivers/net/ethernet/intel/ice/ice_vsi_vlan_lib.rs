//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_vsi_vlan_lib.h
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
// Copyright (C) 2019-2021, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsi_vlan_info {
    pub sw_flags2: u8,
    pub inner_vlan_flags: u8,
    pub outer_vlan_flags: u8,
}

extern "C" {
    pub fn ice_vsi_add_vlan(vsi: *mut ice_vsi, vlan: *mut ice_vlan) -> c_int;
}
extern "C" {
    pub fn ice_vsi_del_vlan(vsi: *mut ice_vsi, vlan: *mut ice_vlan) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ena_inner_stripping(vsi: *mut ice_vsi, tpid: u16) -> c_int;
}
extern "C" {
    pub fn ice_vsi_dis_inner_stripping(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ena_inner_insertion(vsi: *mut ice_vsi, tpid: u16) -> c_int;
}
extern "C" {
    pub fn ice_vsi_dis_inner_insertion(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_set_inner_port_vlan(vsi: *mut ice_vsi, vlan: *mut ice_vlan) -> c_int;
}
extern "C" {
    pub fn ice_vsi_clear_inner_port_vlan(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ena_rx_vlan_filtering(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_dis_rx_vlan_filtering(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ena_tx_vlan_filtering(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_dis_tx_vlan_filtering(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ena_outer_stripping(vsi: *mut ice_vsi, tpid: u16) -> c_int;
}
extern "C" {
    pub fn ice_vsi_dis_outer_stripping(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ena_outer_insertion(vsi: *mut ice_vsi, tpid: u16) -> c_int;
}
extern "C" {
    pub fn ice_vsi_dis_outer_insertion(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_set_outer_port_vlan(vsi: *mut ice_vsi, vlan: *mut ice_vlan) -> c_int;
}
extern "C" {
    pub fn ice_vsi_clear_outer_port_vlan(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_clear_port_vlan(vsi: *mut ice_vsi) -> c_int;
}
