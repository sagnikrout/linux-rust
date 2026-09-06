//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_msgq.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

pub const BFA_MSGQ_CMDQ_NUM_ENTRY: c_int = 128;

pub const BFA_MSGQ_RSPQ_NUM_ENTRY: c_int = 128;

extern "C" {
    pub fn void(cbarg: *mut *mut bfa_msgq_cmdcbfn_t)(void, status: bfa_status) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_msgq_cmd_entry {
    pub qe: list_head,
    pub cbfn: bfa_msgq_cmdcbfn_t,
    pub cbarg: *mut c_void,
    pub msg_size: usize,
    pub msg_hdr: *mut bfi_msgq_mhdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_msgq_cmdq_flags {
    BFA_MSGQ_CMDQ_F_DB_UPDATE	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_msgq_cmdq {
    pub e): *mut *mut *mut void (fsm)(struct bfa_msgq_cmdq s, enum cmdq_event,
    pub flags: bfa_msgq_cmdq_flags,
    pub producer_index: u16,
    pub consumer_index: u16,
    pub /: *mut *mut u16 depth; / FW Q depth is 16 bits,
    pub addr: bfa_dma,
    pub dbell_mb: bfa_mbox_cmd,
    pub token: u16,
    pub offset: c_int,
    pub bytes_to_copy: c_int,
    pub copy_mb: bfa_mbox_cmd,
    pub /: *mut *mut list_head pending_q; / pending command queue,
    pub msgq: *mut bfa_msgq,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_msgq_rspq_flags {
    BFA_MSGQ_RSPQ_F_DB_UPDATE	= 1,
}

extern "C" {
    pub fn void(cbarg: *mut *mut bfa_msgq_mcfunc_t)(void, mhdr: *mut bfi_msgq_mhdr) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_msgq_rspq {
    pub e): *mut *mut *mut void (fsm)(struct bfa_msgq_rspq s, enum rspq_event,
    pub flags: bfa_msgq_rspq_flags,
    pub producer_index: u16,
    pub consumer_index: u16,
    pub /: *mut *mut u16 depth; / FW Q depth is 16 bits,
    pub addr: bfa_dma,
    pub dbell_mb: bfa_mbox_cmd,
    pub nmclass: c_int,
    pub cbfn: bfa_msgq_mcfunc_t,
    pub cbarg: *mut c_void,
    pub rsphdlr: [}; BFI_MC_MAX],
    pub msgq: *mut bfa_msgq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_msgq {
    pub cmdq: bfa_msgq_cmdq,
    pub rspq: bfa_msgq_rspq,
    pub init_wc: bfa_wc,
    pub init_mb: bfa_mbox_cmd,
    pub ioc_notify: bfa_ioc_notify,
    pub ioc: *mut bfa_ioc,
}

extern "C" {
    pub fn bfa_msgq_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_msgq_memclaim(msgq: *mut bfa_msgq, kva: *mut u8, pa: u64);
}
extern "C" {
    pub fn bfa_msgq_attach(msgq: *mut bfa_msgq, ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_msgq_rsp_copy(msgq: *mut bfa_msgq, buf: *mut u8, buf_len: usize);
}
