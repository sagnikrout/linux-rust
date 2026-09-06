//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_llc.h
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
// Definitions for LLC (link layer control) message handling
//
// Copyright IBM Corp. 2016
//
// Author(s):  Klaus Wacker <Klaus.Wacker@de.ibm.com>
// Ursula Braun <ubraun@linux.vnet.ibm.com>
//

pub const SMC_LLC_FLAG_RESP: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_llc_reqresp {
    SMC_LLC_REQ,
    SMC_LLC_RESP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_llc_msg_type {
    SMC_LLC_CONFIRM_LINK		= 0x01,
    SMC_LLC_ADD_LINK		= 0x02,
    SMC_LLC_ADD_LINK_CONT		= 0x03,
    SMC_LLC_DELETE_LINK		= 0x04,
    SMC_LLC_REQ_ADD_LINK		= 0x05,
    SMC_LLC_CONFIRM_RKEY		= 0x06,
    SMC_LLC_TEST_LINK		= 0x07,
    SMC_LLC_CONFIRM_RKEY_CONT	= 0x08,
    SMC_LLC_DELETE_RKEY		= 0x09,
// V2 types
    SMC_LLC_CONFIRM_LINK_V2		= 0x21,
    SMC_LLC_ADD_LINK_V2		= 0x22,
    SMC_LLC_DELETE_LINK_V2		= 0x24,
    SMC_LLC_REQ_ADD_LINK_V2		= 0x25,
    SMC_LLC_CONFIRM_RKEY_V2		= 0x26,
    SMC_LLC_TEST_LINK_V2		= 0x27,
    SMC_LLC_DELETE_RKEY_V2		= 0x29,
}

// LLC DELETE LINK Request Reason Codes
pub const SMC_LLC_DEL_LOST_PATH: c_uint = 0x00010000;
pub const SMC_LLC_DEL_OP_INIT_TERM: c_uint = 0x00020000;
pub const SMC_LLC_DEL_PROG_INIT_TERM: c_uint = 0x00030000;
pub const SMC_LLC_DEL_PROT_VIOL: c_uint = 0x00040000;
pub const SMC_LLC_DEL_NO_ASYM_NEEDED: c_uint = 0x00050000;
// LLC DELETE LINK Response Reason Codes
pub const SMC_LLC_DEL_NOLNK: c_uint = 0x00100000  /* Unknown Link ID (no link) */;
pub const SMC_LLC_DEL_NOLGR: c_uint = 0x00200000  /* Unknown Link Group */;
// returns a usable link of the link group, or NULL
// set the termination reason code for the link group
// transmit
extern "C" {
    pub fn smc_llc_srv_delete_link_local(link: *mut smc_link, del_link_id: u8);
}
extern "C" {
    pub fn smc_llc_lgr_init(lgr: *mut smc_link_group, smc: *mut smc_sock);
}
extern "C" {
    pub fn smc_llc_lgr_clear(lgr: *mut smc_link_group);
}
extern "C" {
    pub fn smc_llc_link_init(link: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_llc_link_active(link: *mut smc_link);
}
extern "C" {
    pub fn smc_llc_link_clear(link: *mut smc_link, log: bool);
}
extern "C" {
    pub fn smc_llc_flow_stop(lgr: *mut smc_link_group, flow: *mut smc_llc_flow);
}
extern "C" {
    pub fn smc_llc_link_set_uid(link: *mut smc_link);
}
extern "C" {
    pub fn smc_llc_save_peer_uid(qentry: *mut smc_llc_qentry);
}
extern "C" {
    pub fn smc_llc_flow_qentry_del(flow: *mut smc_llc_flow);
}
extern "C" {
    pub fn smc_llc_cli_add_link(link: *mut smc_link, qentry: *mut smc_llc_qentry) -> c_int;
}
extern "C" {
    pub fn smc_llc_add_link_local(link: *mut smc_link);
}
