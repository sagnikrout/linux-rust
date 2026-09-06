//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/msgbuf.h
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
// The msqid64_ds structure for x86 architecture with x32 ABI.
//
// On x86-32 and x86-64 we can just use the generic definition, but
// x32 uses the same binary layout as x86_64, which is different
// from other 32-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    pub /: *mut *mut __kernel_long_t msg_stime; / last msgsnd time,
    pub /: *mut *mut __kernel_long_t msg_rtime; / last msgrcv time,
    pub /: *mut *mut __kernel_long_t msg_ctime; / last change time,
    pub /: *mut *mut __kernel_ulong_t msg_cbytes; / current number of bytes on queue,
    pub /: *mut *mut __kernel_ulong_t msg_qnum; / number of messages in queue,
    pub /: *mut *mut __kernel_ulong_t msg_qbytes; / max number of bytes on queue,
    pub /: *mut *mut __kernel_pid_t msg_lspid; / pid of last msgsnd,
    pub /: *mut *mut __kernel_pid_t msg_lrpid; / last receive pid,
    pub __unused4: __kernel_ulong_t,
    pub __unused5: __kernel_ulong_t,
}

