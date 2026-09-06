//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/io_uring/mini_liburing.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_sq_ring {
    pub head: *mut c_uint,
    pub tail: *mut c_uint,
    pub ring_mask: *mut c_uint,
    pub ring_entries: *mut c_uint,
    pub flags: *mut c_uint,
    pub array: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cq_ring {
    pub head: *mut c_uint,
    pub tail: *mut c_uint,
    pub ring_mask: *mut c_uint,
    pub ring_entries: *mut c_uint,
    pub cqes: *mut io_uring_cqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kring_mask: *mut c_uint,
    pub kring_entries: *mut c_uint,
    pub kflags: *mut c_uint,
    pub kdropped: *mut c_uint,
    pub array: *mut c_uint,
    pub sqes: *mut io_uring_sqe,
    pub sqe_head: c_uint,
    pub sqe_tail: c_uint,
    pub ring_sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_cq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kring_mask: *mut c_uint,
    pub kring_entries: *mut c_uint,
    pub koverflow: *mut c_uint,
    pub cqes: *mut io_uring_cqe,
    pub ring_sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring {
    pub sq: io_uring_sq,
    pub cq: io_uring_cq,
    pub ring_fd: c_int,
    pub flags: unsigned,
}

extern "C" {
    pub fn syscall(_arg: __NR_io_uring_setup, _arg: entries, _arg: p) -> return;
}
extern "C" {
    pub fn io_uring_queue_init_params(_arg: entries, _arg: ring, _arg: &p) -> return;
}
// Get a sqe
// cqe_ptr = NULL;
// cqe_ptr = &cq->cqes[head & mask];
// sq->ktail = ktail;
// Prepare and send the SQE
// (&ring->cq)->khead += 1;
