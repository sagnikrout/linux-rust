//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/shmbuf.h
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
// The shmid64_ds structure for x86 architecture with x32 ABI.
//
// On x86-32 and x86-64 we can just use the generic definition, but
// x32 uses the same binary layout as x86_64, which is different
// from other 32-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid64_ds {
    pub /: *mut *mut ipc64_perm shm_perm; / operation perms,
    pub /: *mut *mut __kernel_size_t shm_segsz; / size of segment (bytes),
    pub /: *mut *mut __kernel_long_t shm_atime; / last attach time,
    pub /: *mut *mut __kernel_long_t shm_dtime; / last detach time,
    pub /: *mut *mut __kernel_long_t shm_ctime; / last change time,
    pub /: *mut *mut __kernel_pid_t shm_cpid; / pid of creator,
    pub /: *mut *mut __kernel_pid_t shm_lpid; / pid of last operator,
    pub /: *mut *mut __kernel_ulong_t shm_nattch; / no. of current attaches,
    pub __unused4: __kernel_ulong_t,
    pub __unused5: __kernel_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shminfo64 {
    pub shmmax: __kernel_ulong_t,
    pub shmmin: __kernel_ulong_t,
    pub shmmni: __kernel_ulong_t,
    pub shmseg: __kernel_ulong_t,
    pub shmall: __kernel_ulong_t,
    pub __unused1: __kernel_ulong_t,
    pub __unused2: __kernel_ulong_t,
    pub __unused3: __kernel_ulong_t,
    pub __unused4: __kernel_ulong_t,
}

