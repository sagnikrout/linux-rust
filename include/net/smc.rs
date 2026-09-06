//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/smc.h
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
// Definitions for the SMC module (socket related)
//
// Copyright IBM Corp. 2016
//
// Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_hashinfo {
    pub lock: rwlock_t,
    pub ht: hlist_head,
}

// SMCD/ISM device driver interface
pub const ISM_RESERVED_VLANID: c_uint = 0x1FFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcd_gid {
    pub gid: u64,
    pub gid_ext: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcd_dev {
    pub dibs: *mut dibs_dev,
    pub list: list_head,
    pub lock: spinlock_t,
    pub vlan: list_head,
    pub event_wq: *mut workqueue_struct,
    pub pnetid: [u8; SMC_MAX_PNETID_LEN],
    pub pnetid_by_user: bool,
    pub lgr_list: list_head,
    pub lgr_lock: spinlock_t,
    pub lgr_cnt: core::sync::atomic::AtomicI32,
    pub lgrs_deleted: wait_queue_head_t,
    pub 1: u8 going_away :,
    pub conn: [*mut smc_connection; ],
}

pub const SMC_HS_CTRL_NAME_MAX: c_int = 16;
// ops can be inherit from init_net
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_hs_ctrl {
// private
    pub list: list_head,
    pub owner: *mut module,
// public
// unique name
    pub name: [c_char; SMC_HS_CTRL_NAME_MAX],
    pub flags: c_int,
// Invoked before computing SMC option for SYN packets.
// We can control whether to set SMC options by returning various value.
// Return 0 to disable SMC, or return any other value to enable it.
//
    pub tp): *mut *mut int (syn_option)(struct tcp_sock,
// Invoked before Set up SMC options for SYN-ACK packets
// We can control whether to respond SMC options by returning various
// value. Return 0 to disable SMC, or return any other value to enable
// it.
//
    pub ireq): *mut inet_request_sock,
}

