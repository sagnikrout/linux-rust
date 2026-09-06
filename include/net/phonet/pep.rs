//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/phonet/pep.h
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
// File: pep.h
//
// Phonet Pipe End Point sockets definitions
//
// Copyright (C) 2008 Nokia Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pep_sock {
    pub pn_sk: pn_sock,
// XXX: union-ify listening vs connected stuff ?
// Listening socket stuff:
    pub hlist: hlist_head,
// Connected socket stuff:
    pub listener: *mut sock,
    pub ctrlreq_queue: sk_buff_head,
pub const PNPIPE_CTRLREQ_MAX: c_int = 10;
    pub tx_credits: core::sync::atomic::AtomicI32,
    pub ifindex: c_int,
    pub /: *mut *mut u16 peer_type; / peer type/subtype,
    pub pipe_handle: u8,
    pub rx_credits: u8,
    pub /: *mut *mut u8 rx_fc; / RX flow control,
    pub /: *mut *mut u8 tx_fc; / TX flow control,
    pub /: *mut *mut u8 init_enable; / auto-enable at creation,
    pub aligned: u8,
}

// Pipe protocol definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnpipehdr {
    pub /: *mut *mut u8 utid; / transaction ID,
    pub message_id: u8,
    pub pipe_handle: u8,
    pub /: *mut *mut u8 state_after_connect; / connect request,
    pub /: *mut *mut u8 state_after_reset; / reset request,
    pub /: *mut *mut u8 error_code; / any response,
    pub /: *mut *mut u8 pep_type; / status indication,
    pub /: *mut *mut u8 data0; / anything else,
}

pub const PN_PIPE_INVALID_HANDLE: c_uint = 0xff;
pub const PN_PEP_TYPE_COMMON: c_uint = 0x00;
// Phonet pipe status indication
// Phonet pipe error codes
// Phonet pipe states
// Phonet pipe sub-block types
// Phonet pipe flow control models

// Phonet pipe flow control states
