//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/hypfs.h
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
// Structures for hypfs interface
//
// Copyright IBM Corp. 2013
//
// Author: Martin Schwidefsky <schwidefsky@de.ibm.com>
//

//
// IOCTL for binary interface /sys/kernel/debug/diag_304
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypfs_diag304 {
    pub args: [__u32; 2],
    pub data: __u64,
    pub rc: __u64,
    pub __attribute__((packed)): },
pub const HYPFS_IOCTL_MAGIC: c_uint = 0x10;

//
// Structures for binary interface /sys/kernel/debug/diag_0c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypfs_diag0c_hdr {
    pub /: *mut *mut __u64 len; / Length of diag0c buffer without header,
    pub /: *mut *mut __u16 version; / Version of header,
    pub /: *mut *mut char reserved1[6]; / Reserved,
    pub /: *mut *mut char tod_ext[16]; / TOD clock for diag0c,
    pub /: *mut *mut __u64 count; / Number of entries (CPUs) in diag0c array,
    pub /: *mut *mut char reserved2[24]; / Reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypfs_diag0c_entry {
    pub /: *mut *mut char date[8]; / MM/DD/YY in EBCDIC,
    pub /: *mut *mut char time[8]; / HH:MM:SS in EBCDIC,
    pub /: *mut *mut __u64 virtcpu; / Virtual time consumed by the virt CPU (us),
    pub /: *mut *mut __u64 totalproc; / Total of virtual and simulation time (us),
    pub /: *mut *mut __u32 cpu; / Linux logical CPU number,
    pub /: *mut *mut __u32 reserved; / Align to 8 byte,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypfs_diag0c_data {
    pub /: *mut *mut hypfs_diag0c_hdr hdr; / 64 byte header,
    pub /: *mut *mut hypfs_diag0c_entry entry[]; / diag0c entry array,
}
