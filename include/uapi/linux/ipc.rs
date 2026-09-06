//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ipc.h
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

// Obsolete, used only for backwards compatibility and libc5 compiles
// Include the definition of ipc64_perm

// resource get request flags

// these fields are used by the DIPC package so the kernel as standard

//
// Control commands used with semctl, msgctl and shmctl
// see also specific commands in sem.h, msg.h and shm.h
//

//
// Version flags for semctl, msgctl, and shmctl commands
// These are passed as bitflags or-ed with the actual command
//

pub const IPC_64: c_uint = 0x0100  /* New version (support 32-bit UIDs, bigger;
//
// These are used to wrap system calls.
//
// See architecture code for ugly details..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_kludge {
    pub msgp: *mut msgbuf __user,
    pub msgtyp: c_long,
}

pub const SEMOP: c_int = 1;
pub const SEMGET: c_int = 2;
pub const SEMCTL: c_int = 3;
pub const SEMTIMEDOP: c_int = 4;
pub const MSGSND: c_int = 11;
pub const MSGRCV: c_int = 12;
pub const MSGGET: c_int = 13;
pub const MSGCTL: c_int = 14;
pub const SHMAT: c_int = 21;
pub const SHMDT: c_int = 22;
pub const SHMGET: c_int = 23;
pub const SHMCTL: c_int = 24;
// Used by the DIPC package, try and avoid reusing it
pub const DIPC: c_int = 25;

