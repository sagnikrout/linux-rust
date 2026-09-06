//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/ksz9477.h
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
//
// Microchip KSZ9477 series Header file
//
// Copyright (C) 2017-2025 Microchip Technology Inc.
//

extern "C" {
    pub fn ksz9477_get_port_addr(port: c_int, offset: c_int) -> u32;
}
extern "C" {
    pub fn ksz9477_cfg_port_member(dev: *mut ksz_device, port: c_int, member: u8);
}
extern "C" {
    pub fn ksz9477_flush_dyn_mac_table(ds: *mut dsa_switch, port: c_int);
}
extern "C" {
    pub fn ksz9477_r_mib_cnt(dev: *mut ksz_device, port: c_int, addr: u16, cnt: *mut u64);
}
extern "C" {
    pub fn ksz9477_freeze_mib(dev: *mut ksz_device, port: c_int, freeze: bool);
}
extern "C" {
    pub fn ksz9477_port_init_cnt(dev: *mut ksz_device, port: c_int);
}
extern "C" {
    pub fn ksz9477_max_mtu(ds: *mut dsa_switch, port: c_int) -> c_int;
}
extern "C" {
    pub fn ksz9477_enable_stp_addr(dev: *mut ksz_device) -> c_int;
}
extern "C" {
    pub fn ksz9477_port_queue_split(dev: *mut ksz_device, port: c_int);
}
extern "C" {
    pub fn ksz9477_set_default_prio_queue_mapping(dev: *mut ksz_device, port: c_int) -> c_int;
}
extern "C" {
    pub fn ksz9477_port_acl_init(dev: *mut ksz_device, port: c_int) -> c_int;
}
extern "C" {
    pub fn ksz9477_port_acl_free(dev: *mut ksz_device, port: c_int);
}
pub const KSZ9477_ACL_ENTRY_SIZE: c_int = 18;
pub const KSZ9477_ACL_MAX_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz9477_acl_entry {
    pub entry: [u8; KSZ9477_ACL_ENTRY_SIZE],
    pub cookie: c_ulong,
    pub prio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz9477_acl_entries {
    pub entries: [ksz9477_acl_entry; KSZ9477_ACL_MAX_ENTRIES],
    pub entries_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz9477_acl_priv {
    pub acles: ksz9477_acl_entries,
}

extern "C" {
    pub fn ksz9477_acl_write_list(dev: *mut ksz_device, port: c_int) -> c_int;
}
extern "C" {
    pub fn ksz9477_sort_acl_entries(dev: *mut ksz_device, port: c_int) -> c_int;
}
extern "C" {
    pub fn ksz9477_acl_action_rule_cfg(entry: *mut u8, force_prio: bool, prio_val: u8);
}
extern "C" {
    pub fn ksz9477_acl_processing_rule_set_action(entry: *mut u8, action_idx: u8);
}
