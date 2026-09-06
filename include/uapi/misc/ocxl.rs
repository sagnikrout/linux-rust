//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/misc/ocxl.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
// Copyright 2017 IBM Corp.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocxl_event_type {
    OCXL_AFU_EVENT_XSL_FAULT_ERROR = 0,
}

pub const OCXL_KERNEL_EVENT_FLAG_LAST: c_uint = 0x0001  /* This is the last event pending */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_kernel_event_header {
    pub type: __u16,
    pub flags: __u16,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_kernel_event_xsl_fault_error {
    pub addr: __u64,
    pub dsisr: __u64,
    pub count: __u64,
    pub reserved: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_attach {
    pub amr: __u64,
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_metadata {
    pub /: *mut *mut __u16 version; / struct version, always backwards compatible,
// Version 0 fields
    pub afu_version_major: __u8,
    pub afu_version_minor: __u8,
    pub /: *mut *mut __u32 pasid; / PASID assigned to the current context,
    pub /: *mut *mut __u64 pp_mmio_size; / Per PASID MMIO size,
    pub global_mmio_size: __u64,
// End version 0 fields
    pub /: *mut *mut *mut __u64 reserved[13]; / Total of 16u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_p9_wait {
    pub /: *mut *mut __u16 thread_id; / The thread ID required to wake this thread,
    pub reserved1: __u16,
    pub reserved2: __u32,
    pub reserved3: [__u64; 3],
}

pub const OCXL_IOCTL_FEATURES_FLAGS0_P9_WAIT: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_features {
    pub flags: [__u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_irq_fd {
    pub irq_offset: __u64,
    pub eventfd: __s32,
    pub reserved: __u32,
}

// ioctl numbers
pub const OCXL_MAGIC: c_uint = 0xCA;
// AFU devices

