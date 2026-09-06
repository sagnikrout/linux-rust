//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/io_uring/bpf_filter.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
//
// Header file for the io_uring BPF filters.
//

//
// Struct passed to filters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_bpf_ctx {
    pub user_data: __u64,
    pub opcode: __u8,
    pub sqe_flags: __u8,
    pub /: *mut *mut __u8 pdu_size; / size of aux data for filter,
    pub pad: [__u8; 5],
    pub family: __u32,
    pub type: __u32,
    pub protocol: __u32,
    pub socket: },
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
    pub open: },
//
// For CONNECT: fields are populated only when addr_len covers
// them; unpopulated fields are zero from the caller-side memset
// in io_uring_populate_bpf_ctx(). port and v4_addr are network
// byte order. Filters may only issue BPF_LD|BPF_W|BPF_ABS at
// 4-byte aligned offsets; load + mask for sub-word fields.
//
    pub /: *mut *mut __u32 family; / sa_family_t zero-extended,
    pub port: __be16,
    pub pad: [__u8; 2],
    pub v4_addr: __be32,
    pub v6_addr: [__u8; 16],
}

//
// If set, any currently unset opcode will have a deny filter attached
//
// If set, if kernel and application don't agree on pdu_size for
// the given opcode, fail the registration of the filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_bpf_filter {
    pub /: *mut *mut __u32 opcode; / io_uring opcode to filter,
    pub flags: __u32,
    pub /: *mut *mut __u32 filter_len; / number of BPF instructions,
    pub /: *mut *mut __u8 pdu_size; / expected pdu size for opcode,
    pub resv: [__u8; 3],
    pub /: *mut *mut __u64 filter_ptr; / pointer to BPF filter,
    pub resv2: [__u64; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_bpf {
    pub /: *mut *mut *mut __u16 cmd_type; / IO_URING_BPF_ values,
    pub /: *mut *mut __u16 cmd_flags; / none so far,
    pub resv: __u32,
    pub filter: io_uring_bpf_filter,
}
