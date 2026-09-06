//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_wr.h
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
// Work Requests exploiting Infiniband API
//
// Copyright IBM Corp. 2016
//
// Author(s):  Steffen Maier <maier@linux.vnet.ibm.com>
//

pub const SMC_WR_TX_PEND_PRIV_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_wr_tx_pend_priv {
    pub priv: [u8; SMC_WR_TX_PEND_PRIV_SIZE],
}

extern "C" {
    pub fn void(: *mut *mut smc_wr_tx_dismisser)(struct smc_wr_tx_pend_priv) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_wr_rx_handler {
    pub /: *mut *mut hlist_node list; / hash table collision resolution,
    pub ): *mut *mut *mut void (handler)(struct ib_wc , void,
    pub type: u8,
}

// Only used by RDMA write WRs.
// All other WRs (CDC/LLC) use smc_wr_tx_send handling WR_ID implicitly
//
extern "C" {
    pub fn atomic_long_inc_return(_arg: &link->wr_tx_id) -> return;
}
// post a new receive work request to fill a completed old work request entry
extern "C" {
    pub fn smc_wr_create_link(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_wr_alloc_link_mem(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_wr_alloc_lgr_mem(lgr: *mut smc_link_group) -> c_int;
}
extern "C" {
    pub fn smc_wr_free_link(lnk: *mut smc_link);
}
extern "C" {
    pub fn smc_wr_free_link_mem(lnk: *mut smc_link);
}
extern "C" {
    pub fn smc_wr_free_lgr_mem(lgr: *mut smc_link_group);
}
extern "C" {
    pub fn smc_wr_remember_qp_attr(lnk: *mut smc_link);
}
extern "C" {
    pub fn smc_wr_remove_dev(smcibdev: *mut smc_ib_device);
}
extern "C" {
    pub fn smc_wr_add_dev(smcibdev: *mut smc_ib_device);
}
extern "C" {
    pub fn smc_wr_tx_cq_handler(ib_cq: *mut ib_cq, cq_context: *mut c_void);
}
extern "C" {
    pub fn smc_wr_tx_wait_no_pending_sends(link: *mut smc_link);
}
extern "C" {
    pub fn smc_wr_rx_register_handler(handler: *mut smc_wr_rx_handler) -> c_int;
}
extern "C" {
    pub fn smc_wr_rx_post_init(link: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_wr_rx_cq_handler(ib_cq: *mut ib_cq, cq_context: *mut c_void);
}
extern "C" {
    pub fn smc_wr_reg_send(link: *mut smc_link, mr: *mut ib_mr) -> c_int;
}
