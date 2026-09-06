//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/st-nci/ndlc.h
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
//
// NCI based Driver for STMicroelectronics NFC Chip
//
// Copyright (C) 2014-2015  STMicroelectronics SAS. All rights reserved.
//

// Low Level Transport description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llt_ndlc {
    pub ndev: *mut nci_dev,
    pub ops: *const nfc_phy_ops,
    pub phy_id: *mut c_void,
    pub t1_timer: timer_list,
    pub t1_active: bool,
    pub t2_timer: timer_list,
    pub t2_active: bool,
    pub rcv_q: sk_buff_head,
    pub send_q: sk_buff_head,
    pub ack_pending_q: sk_buff_head,
    pub sm_work: work_struct,
    pub dev: *mut device,
//
// < 0 if hardware error occurred
// and prevents normal operation.
//
    pub hard_fault: c_int,
    pub powered: c_int,
}

extern "C" {
    pub fn ndlc_open(ndlc: *mut llt_ndlc) -> c_int;
}
extern "C" {
    pub fn ndlc_close(ndlc: *mut llt_ndlc);
}
extern "C" {
    pub fn ndlc_send(ndlc: *mut llt_ndlc, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ndlc_recv(ndlc: *mut llt_ndlc, skb: *mut sk_buff);
}
extern "C" {
    pub fn ndlc_remove(ndlc: *mut llt_ndlc);
}
