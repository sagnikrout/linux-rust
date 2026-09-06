//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/opdef.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_issue_def {
// needs req->file assigned
    pub 1: unsigned needs_file :,
// should block plug
    pub 1: unsigned plug :,
// supports ioprio
    pub 1: unsigned ioprio :,
// supports iopoll
    pub 1: unsigned iopoll :,
// op supports buffer selection
    pub 1: unsigned buffer_select :,
// hash wq insertion if file is a regular file
    pub 1: unsigned hash_reg_file :,
// unbound wq insertion if file is a non-regular file
    pub 1: unsigned unbound_nonreg_file :,
// set if opcode supports polled "wait"
    pub 1: unsigned pollin :,
    pub 1: unsigned pollout :,
    pub 1: unsigned poll_exclusive :,
// skip auditing
    pub 1: unsigned audit_skip :,
// vectored opcode, set if 1) vectored, and 2) handler needs to know
    pub 1: unsigned vectored :,
// set to 1 if this opcode uses 128b sqes in a mixed sq
    pub 1: unsigned is_128 :,
// size of async data needed, if any
    pub async_size: c_ushort,
// bpf filter pdu size, if any
    pub filter_pdu_size: c_ushort,
    pub int): *mut *mut *mut int (issue)(struct io_kiocb , unsigned,
    pub ): *const *const *const int (prep)(struct io_kiocb , struct io_uring_sqe,
    pub ): *mut *mut *mut void (filter_populate)(struct io_uring_bpf_ctx , struct io_kiocb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cold_def {
    pub name: *const c_char,
    pub ): *mut *mut void (sqe_copy)(struct io_kiocb,
    pub ): *mut *mut void (cleanup)(struct io_kiocb,
    pub ): *mut *mut void (fail)(struct io_kiocb,
}

extern "C" {
    pub fn io_uring_op_supported(opcode: u8) -> bool;
}
extern "C" {
    pub fn io_uring_optable_init();
}
