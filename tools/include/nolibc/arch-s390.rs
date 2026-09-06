//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-s390.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// s390 specific definitions for NOLIBC
//

// Syscalls for s390:
// - registers are 64-bit
// - syscall number is passed in r1
// - arguments are in r2-r7
// - the system call is performed by calling the svc instruction
// - syscall return value is in r2
// - r1 and r2 are clobbered, others are preserved.
//
// Link s390 ABI: https://github.com/IBM/s390x-abi
//

// startup code

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_mmap_arg_struct {
    pub addr: c_ulong,
    pub len: c_ulong,
    pub prot: c_ulong,
    pub flags: c_ulong,
    pub fd: c_ulong,
    pub offset: c_ulong,
}

extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_clone, _arg: 0, _arg: SIGCHLD, _arg: 0, _arg: 0, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_clone, _arg: 0, SIGCHLD: CLONE_VM | CLONE_VFORK |, _arg: 0, _arg: 0, _arg: 0) -> return;
}

