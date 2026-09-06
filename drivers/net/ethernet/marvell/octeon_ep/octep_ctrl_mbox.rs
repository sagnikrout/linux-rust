//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_ctrl_mbox.h
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
// Marvell Octeon EP (EndPoint) Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//
// barmem structure
// |===========================================|
// |Info (16 + 120 + 120 = 256 bytes)          |
// |-------------------------------------------|
// |magic number (8 bytes)                     |
// |bar memory size (4 bytes)                  |
// |reserved (4 bytes)                         |
// |-------------------------------------------|
// |host version (8 bytes)                     |
// |    low 32 bits                            |
// |host status (8 bytes)                      |
// |host reserved (104 bytes)                  |
// |-------------------------------------------|
// |fw version's (8 bytes)                     |
// |    min=high 32 bits, max=low 32 bits      |
// |fw status (8 bytes)                        |
// |fw reserved (104 bytes)                    |
// |===========================================|
// |Host to Fw Queue info (16 bytes)           |
// |-------------------------------------------|
// |producer index (4 bytes)                   |
// |consumer index (4 bytes)                   |
// |max element size (4 bytes)                 |
// |reserved (4 bytes)                         |
// |===========================================|
// |Fw to Host Queue info (16 bytes)           |
// |-------------------------------------------|
// |producer index (4 bytes)                   |
// |consumer index (4 bytes)                   |
// |max element size (4 bytes)                 |
// |reserved (4 bytes)                         |
// |===========================================|
// |Host to Fw Queue ((total size-288/2) bytes)|
// |-------------------------------------------|
// |                                           |
// |===========================================|
// |Fw to Host Queue ((total size-288/2) bytes)|
// |-------------------------------------------|
// |                                           |
// |===========================================|
//
pub const OCTEP_CTRL_MBOX_MAGIC_NUMBER: c_uint = 0xdeaddeadbeefbeefull;
// Valid request message

// Valid response message

// Valid notification, no response required

// Valid custom message

pub const OCTEP_CTRL_MBOX_MSG_DESC_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_ctrl_mbox_status {
    OCTEP_CTRL_MBOX_STATUS_INVALID = 0,
    OCTEP_CTRL_MBOX_STATUS_INIT,
    OCTEP_CTRL_MBOX_STATUS_READY,
    OCTEP_CTRL_MBOX_STATUS_UNINIT
}

// mbox message
#[repr(C)]
#[derive(Copy, Clone)]
pub union octep_ctrl_mbox_msg_hdr {
    pub words: [u64; 2],
// must be 0
    pub reserved1:15: u16,
// vf_idx is valid if 1
    pub is_vf:1: u16,
// sender vf index 0-(n-1), 0 if (is_vf==0)
    pub vf_idx: u16,
// total size of message excluding header
    pub sz: u32,
// OCTEP_CTRL_MBOX_MSG_HDR_FLAG_*
    pub flags: u32,
// identifier to match responses
    pub msg_id: u16,
    pub reserved2: u16,
    pub s: },
}

// mbox message buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_mbox_msg_buf {
    pub reserved1: u32,
    pub reserved2: u16,
// size of buffer
    pub sz: u16,
// pointer to message buffer
    pub msg: *mut c_void,
}

// mbox message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_mbox_msg {
// mbox transaction header
    pub hdr: octep_ctrl_mbox_msg_hdr,
// number of sg buffer's
    pub sg_num: c_int,
// message buffer's
    pub sg_list: [octep_ctrl_mbox_msg_buf; OCTEP_CTRL_MBOX_MSG_DESC_MAX],
}

// Mbox queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_mbox_q {
// size of queue buffer
    pub sz: u32,
// producer address in bar mem
    pub hw_prod: *mut u8 __iomem,
// consumer address in bar mem
    pub hw_cons: *mut u8 __iomem,
// q base address in bar mem
    pub hw_q: *mut u8 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_mbox {
// control plane version
    pub version: u64,
// size of bar memory
    pub barmem_sz: u32,
// pointer to BAR memory
    pub barmem: *mut u8 __iomem,
// host-to-fw queue
    pub h2fq: octep_ctrl_mbox_q,
// fw-to-host queue
    pub f2hq: octep_ctrl_mbox_q,
// lock for h2fq
    pub h2fq_lock: mutex,
// lock for f2hq
    pub f2hq_lock: mutex,
// Min control plane version supported by firmware
    pub min_fw_version: u32,
// Max control plane version supported by firmware
    pub max_fw_version: u32,
}

// Initialize control mbox.
//
// @param mbox: non-null pointer to struct octep_ctrl_mbox.
//
// return value: 0 on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_mbox_init(mbox: *mut octep_ctrl_mbox) -> c_int;
}
// Send mbox message.
//
// @param mbox: non-null pointer to struct octep_ctrl_mbox.
// @param msg:  non-null pointer to struct octep_ctrl_mbox_msg.
// Caller should fill msg.sz and msg.desc.sz for each message.
//
// return value: 0 on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_mbox_send(mbox: *mut octep_ctrl_mbox, msg: *mut octep_ctrl_mbox_msg) -> c_int;
}
// Retrieve mbox message.
//
// @param mbox: non-null pointer to struct octep_ctrl_mbox.
// @param msg:  non-null pointer to struct octep_ctrl_mbox_msg.
// Caller should fill msg.sz and msg.desc.sz for each message.
//
// return value: 0 on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_mbox_recv(mbox: *mut octep_ctrl_mbox, msg: *mut octep_ctrl_mbox_msg) -> c_int;
}
// Uninitialize control mbox.
//
// @param mbox: non-null pointer to struct octep_ctrl_mbox.
//
// return value: 0 on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_mbox_uninit(mbox: *mut octep_ctrl_mbox) -> c_int;
}
