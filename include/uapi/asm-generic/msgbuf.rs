//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/msgbuf.h
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
// generic msqid64_ds structure.
//
// Note extra padding because this structure is passed back and forth
// between kernel and user space.
//
// msqid64_ds was originally meant to be architecture specific, but
// everyone just ended up making identical copies without specific
// optimizations, so we may just as well all use the same one.
//
// 64 bit architectures use a 64-bit long time field here, while
// 32 bit architectures have a pair of unsigned long values.
// On big-endian systems, the lower half is in the wrong place.
//
// Pad space is left for:
// - 2 miscellaneous 32-bit values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,

    pub /: *mut *mut long msg_stime; / last msgsnd time,
    pub /: *mut *mut long msg_rtime; / last msgrcv time,
    pub /: *mut *mut long msg_ctime; / last change time,

    pub /: *mut *mut unsigned long msg_stime; / last msgsnd time,
    pub msg_stime_high: c_ulong,
    pub /: *mut *mut unsigned long msg_rtime; / last msgrcv time,
    pub msg_rtime_high: c_ulong,
    pub /: *mut *mut unsigned long msg_ctime; / last change time,
    pub msg_ctime_high: c_ulong,

    pub /: *mut *mut unsigned long msg_cbytes; / current number of bytes on queue,
    pub /: *mut *mut unsigned long msg_qnum; / number of messages in queue,
    pub /: *mut *mut unsigned long msg_qbytes; / max number of bytes on queue,
    pub /: *mut *mut __kernel_pid_t msg_lspid; / pid of last msgsnd,
    pub /: *mut *mut __kernel_pid_t msg_lrpid; / last receive pid,
    pub __unused4: c_ulong,
    pub __unused5: c_ulong,
}
