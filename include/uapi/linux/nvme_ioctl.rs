//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nvme_ioctl.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Definitions for the NVM Express ioctl interface
// Copyright (c) 2011-2014, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_user_io {
    pub opcode: __u8,
    pub flags: __u8,
    pub control: __u16,
    pub nblocks: __u16,
    pub rsvd: __u16,
    pub metadata: __u64,
    pub addr: __u64,
    pub slba: __u64,
    pub dsmgmt: __u32,
    pub reftag: __u32,
    pub apptag: __u16,
    pub appmask: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_passthru_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub rsvd1: __u16,
    pub nsid: __u32,
    pub cdw2: __u32,
    pub cdw3: __u32,
    pub metadata: __u64,
    pub addr: __u64,
    pub metadata_len: __u32,
    pub data_len: __u32,
    pub cdw10: __u32,
    pub cdw11: __u32,
    pub cdw12: __u32,
    pub cdw13: __u32,
    pub cdw14: __u32,
    pub cdw15: __u32,
    pub timeout_ms: __u32,
    pub result: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_passthru_cmd64 {
    pub opcode: __u8,
    pub flags: __u8,
    pub rsvd1: __u16,
    pub nsid: __u32,
    pub cdw2: __u32,
    pub cdw3: __u32,
    pub metadata: __u64,
    pub addr: __u64,
    pub metadata_len: __u32,
    pub /: *mut *mut __u32 data_len; / for non-vectored io,
    pub /: *mut *mut __u32 vec_cnt; / for vectored io,
}

// same as struct nvme_passthru_cmd64, minus the 8b result field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_uring_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub rsvd1: __u16,
    pub nsid: __u32,
    pub cdw2: __u32,
    pub cdw3: __u32,
    pub metadata: __u64,
    pub addr: __u64,
    pub metadata_len: __u32,
    pub data_len: __u32,
    pub cdw10: __u32,
    pub cdw11: __u32,
    pub cdw12: __u32,
    pub cdw13: __u32,
    pub cdw14: __u32,
    pub cdw15: __u32,
    pub timeout_ms: __u32,
    pub rsvd2: __u32,
}

// io_uring async commands:

