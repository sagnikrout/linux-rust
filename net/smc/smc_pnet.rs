//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_pnet.h
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
// Shared Memory Communications over RDMA (SMC-R) and RoCE
//
// PNET table queries
//
// Copyright IBM Corp. 2016
//
// Author(s):  Thomas Richter <tmricht@linux.vnet.ibm.com>
//

//
// struct smc_pnettable - SMC PNET table anchor
// @lock: Lock for list action
// @pnetlist: List of PNETIDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_pnettable {
    pub lock: mutex,
    pub pnetlist: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_pnetids_ndev {
    pub list: list_head,
    pub lock: rwlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_pnetids_ndev_entry {
    pub list: list_head,
    pub pnetid: [u8; SMC_MAX_PNETID_LEN],
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn pnet_id_by_dev_port(_arg: dev, _arg: port, _arg: pnetid) -> return;
}

extern "C" {
    pub fn smc_pnet_net_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn smc_pnet_exit();
}
extern "C" {
    pub fn smc_pnet_net_exit(net: *mut net);
}
extern "C" {
    pub fn smc_pnet_find_roce_resource(sk: *mut sock, ini: *mut smc_init_info);
}
extern "C" {
    pub fn smc_pnet_find_ism_resource(sk: *mut sock, ini: *mut smc_init_info);
}
extern "C" {
    pub fn smc_pnetid_by_table_ib(smcibdev: *mut smc_ib_device, ib_port: u8) -> c_int;
}
extern "C" {
    pub fn smc_pnetid_by_table_smcd(smcd: *mut smcd_dev) -> c_int;
}
extern "C" {
    pub fn smc_pnet_is_ndev_pnetid(net: *mut net, pnetid: *mut u8) -> bool;
}
extern "C" {
    pub fn smc_pnet_is_pnetid_set(pnetid: *mut u8) -> bool;
}
