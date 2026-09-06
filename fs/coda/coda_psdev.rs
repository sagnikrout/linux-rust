//! Automatically rewritten from C Header to Rust Module
//! Source: fs/coda/coda_psdev.h
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

pub const CODA_PSDEV_MAJOR: c_int = 67;

// messages between coda filesystem in kernel and Venus
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upc_req {
    pub uc_chain: list_head,
    pub uc_data: caddr_t,
    pub uc_flags: u_short,
    pub /: *mut *mut u_short uc_inSize; / Size is at most 5000 bytes,
    pub uc_outSize: u_short,
    pub /: *mut *mut u_short uc_opcode; / copied from data to save lookup,
    pub uc_unique: c_int,
    pub /: *mut *mut wait_queue_head_t uc_sleep; / process' wait queue,
}

pub const CODA_REQ_ASYNC: c_uint = 0x1;
pub const CODA_REQ_READ: c_uint = 0x2;
pub const CODA_REQ_WRITE: c_uint = 0x4;
pub const CODA_REQ_ABORT: c_uint = 0x8;
// communication pending/processing queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_comm {
    pub vc_seq: u_long,
    pub /: *mut *mut wait_queue_head_t vc_waitq; / Venus wait queue,
    pub vc_pending: list_head,
    pub vc_processing: list_head,
    pub vc_inuse: c_int,
    pub vc_sb: *mut super_block,
    pub vc_mutex: mutex,
}

// upcalls
extern "C" {
    pub fn venus_rootfid(sb: *mut super_block, fidp: *mut CodaFid) -> c_int;
}
extern "C" {
    pub fn venus_setattr(: *mut super_block, : *mut CodaFid, : *mut coda_vattr) -> c_int;
}
extern "C" {
    pub fn venus_access(sb: *mut super_block, fid: *mut CodaFid, mask: c_int) -> c_int;
}
extern "C" {
    pub fn venus_fsync(sb: *mut super_block, fid: *mut CodaFid) -> c_int;
}
extern "C" {
    pub fn venus_statfs(dentry: *mut dentry, sfs: *mut kstatfs) -> c_int;
}
//
// Statistics
//
