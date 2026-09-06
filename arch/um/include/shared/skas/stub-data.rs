//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/skas/stub-data.h
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
//
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2005 Jeff Dike (jdike@karaya.com)
//

pub const FUTEX_IN_CHILD: c_int = 0;
pub const FUTEX_IN_KERN: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stub_init_data {
    pub seccomp: c_int,
    pub stub_start: c_ulong,
    pub stub_code_fd: c_int,
    pub stub_code_offset: c_ulong,
    pub stub_data_fd: c_int,
    pub stub_data_offset: c_ulong,
    pub signal_handler: c_ulong,
    pub signal_restorer: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stub_syscall_type {
    STUB_SYSCALL_UNSET = 0,
    STUB_SYSCALL_MMAP,
    STUB_SYSCALL_MUNMAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stub_syscall {
    pub addr: c_ulong,
    pub length: c_ulong,
    pub offset: c_ulong,
    pub fd: c_int,
    pub prot: c_int,
    pub mem: },
    pub syscall: stub_syscall_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stub_data {
    pub err: c_long,
    pub syscall_data_len: c_int,
// 128 leaves enough room for additional fields in the struct
    pub __aligned(16): stub_syscall syscall_data[(UM_KERN_PAGE_SIZE - 128) / sizeof(stub_syscall)],
// data shared with signal handler (only used in seccomp mode)
    pub restart_wait: c_short,
    pub futex: c_uint,
    pub signal: c_int,
    pub si_offset: c_ushort,
    pub mctx_offset: c_ushort,
// seccomp architecture specific state restore
    pub arch_data: stub_data_arch,
// Stack for our signal handlers and for calling into .
    pub __aligned(UM_KERN_PAGE_SIZE): unsigned char sigstack[UM_KERN_PAGE_SIZE],
}
