//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/fcntl.h
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
// FMODE_EXEC is 0x20
// These cannot be used by userspace O_* until internal and external open
// flags are split.
// -Eric Paris
//
// When introducing new O_* bits, please check its uniqueness in fcntl_init().
//
pub const O_ACCMODE: c_int = 3;
pub const O_RDONLY: c_int = 0;

// (1 << 2) must not be used -- it collides with flags on alpha, sparc
// (1 << 3) must not be used -- it collides with flags on alpha, mips, parisc, sparc
// (1 << 4) must not be used -- it collides with flags on mips
// (1 << 5) is free

//
// Before Linux 2.6.33 only O_DSYNC semantics were implemented, but using
// the O_SYNC flag.  We continue to use the existing numerical value
// for O_DSYNC semantics now, but using the correct symbolic name for it.
// This new value is used to request true Posix O_SYNC semantics.  It is
// defined in this strange way to make sure applications compiled against
// new headers get at least O_DSYNC semantics on older kernels.
//
// This has the nice side-effect that we can simply test for O_DSYNC
// wherever we do not care if O_DSYNC or O_SYNC is used.
//
// Note: __O_SYNC must never be used directly.
//

// a horrid kludge trying to make sure that this will fail on old kernels

// (1 << 23) must not be used -- it collides with flags on alpha, parisc, sparc
// (1 << 24) must not be used -- it collides with flags on alpha, sparc
// (1 << 25) must not be used -- it collides with flags on sparc

pub const F_GETLK: c_int = 5;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;

pub const F_SETLK64: c_int = 13;
pub const F_SETLKW64: c_int = 14;

pub const F_SETOWN_EX: c_int = 15;
pub const F_GETOWN_EX: c_int = 16;

pub const F_GETOWNER_UIDS: c_int = 17;

//
// Open File Description Locks
//
// Usually record locks held by a process are released on *any* close and are
// not inherited across a fork().
//
// These cmd values will set locks that conflict with process-associated
// record  locks, but are "owned" by the open file description, not the
// process. This means that they are inherited across fork() like BSD (flock)
// locks, and they are only released automatically when the last reference to
// the the open file against which they were acquired is put.
//
pub const F_OFD_GETLK: c_int = 36;
pub const F_OFD_SETLK: c_int = 37;
pub const F_OFD_SETLKW: c_int = 38;
pub const F_OWNER_TID: c_int = 0;
pub const F_OWNER_PID: c_int = 1;
pub const F_OWNER_PGRP: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_owner_ex {
    pub type: c_int,
    pub pid: __kernel_pid_t,
}

// for F_[GET|SET]FL

// for posix fcntl() and lockf()

pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 1;
pub const F_UNLCK: c_int = 2;

// for old implementation of bsd flock ()

// operations for bsd flock(), also used by the kernel implementation

//
// LOCK_MAND support has been removed from the kernel. We leave the symbols
// here to not break legacy builds, but these should not be used in new code.
//

pub const F_LINUX_SPECIFIC_BASE: c_int = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: __kernel_off_t,
    pub l_len: __kernel_off_t,
    pub l_pid: __kernel_pid_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flock64 {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: __kernel_loff_t,
    pub l_len: __kernel_loff_t,
    pub l_pid: __kernel_pid_t,

}

