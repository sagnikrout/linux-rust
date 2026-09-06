//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hns/hns_roce_bond.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2025 Hisilicon Limited.
//

pub const ROCE_BOND_FUNC_MAX: c_int = 4;
pub const ROCE_BOND_NUM_MAX: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_bond_hashtype {
    BOND_HASH_L2,
    BOND_HASH_L34,
    BOND_HASH_L23,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bond_support_type {
    BOND_NOT_SUPPORT,
//
// bond_grp already exists, but in the current
// conditions it's no longer supported
//
    BOND_EXISTING_NOT_SUPPORT,
    BOND_SUPPORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_bond_state {
    HNS_ROCE_BOND_NOT_ATTACHED,
    HNS_ROCE_BOND_NOT_BONDED,
    HNS_ROCE_BOND_IS_BONDED,
    HNS_ROCE_BOND_SLAVE_CHANGE_NUM,
    HNS_ROCE_BOND_SLAVE_CHANGESTATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_bond_cmd_type {
    HNS_ROCE_SET_BOND,
    HNS_ROCE_CHANGE_BOND,
    HNS_ROCE_CLEAR_BOND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_func_info {
    pub net_dev: *mut net_device,
    pub handle: *mut hnae3_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_bond_group {
    pub upper_dev: *mut net_device,
    pub main_hr_dev: *mut hns_roce_dev,
    pub active_slave_num: u8,
    pub slave_map: u32,
    pub active_slave_map: u32,
    pub bond_id: u8,
    pub bus_num: u8,
    pub bond_func_info: [hns_roce_func_info; ROCE_BOND_FUNC_MAX],
    pub bond_ready: bool,
    pub bond_state: hns_roce_bond_state,
    pub tx_type: netdev_lag_tx_type,
    pub hash_type: netdev_lag_hash,
    pub bond_mutex: mutex,
    pub bond_nb: notifier_block,
    pub bond_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_die_info {
    pub bond_id_mask: u8,
    pub bgrps: [*mut hns_roce_bond_group; ROCE_BOND_NUM_MAX],
    pub die_mutex: mutex,
    pub suspend_cnt: u8,
}

extern "C" {
    pub fn hns_roce_alloc_bond_grp(hr_dev: *mut hns_roce_dev) -> c_int;
}
extern "C" {
    pub fn hns_roce_dealloc_bond_grp();
}
extern "C" {
    pub fn hns_roce_cleanup_bond(bond_grp: *mut hns_roce_bond_group);
}
extern "C" {
    pub fn hns_roce_bond_is_active(hr_dev: *mut hns_roce_dev) -> bool;
}
extern "C" {
    pub fn hns_roce_bond_init(hr_dev: *mut hns_roce_dev) -> c_int;
}
extern "C" {
    pub fn hns_roce_bond_suspend(handle: *mut hnae3_handle);
}
extern "C" {
    pub fn hns_roce_bond_resume(handle: *mut hnae3_handle);
}
