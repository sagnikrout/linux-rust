//! Automatically rewritten from C Header to Rust Module
//! Source: net/nfc/hci/llc.h
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
// Link Layer Control manager
//
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llc_ops {
    pub llc_failure): llc_failure_t,
    pub llc): *mut *mut void (deinit) (struct nfc_llc,
    pub llc): *mut *mut int (start) (struct nfc_llc,
    pub llc): *mut *mut int (stop) (struct nfc_llc,
    pub skb): *mut *mut *mut void (rcv_from_drv) (struct nfc_llc llc, struct sk_buff,
    pub skb): *mut *mut *mut int (xmit_from_hci) (struct nfc_llc llc, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llc_engine {
    pub name: *const c_char,
    pub ops: *const nfc_llc_ops,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llc {
    pub data: *mut c_void,
    pub ops: *const nfc_llc_ops,
    pub rx_headroom: c_int,
    pub rx_tailroom: c_int,
}

extern "C" {
    pub fn nfc_llc_register(name: *const c_char, ops: *const nfc_llc_ops) -> c_int;
}
extern "C" {
    pub fn nfc_llc_nop_register() -> c_int;
}

extern "C" {
    pub fn nfc_llc_shdlc_register() -> c_int;
}

