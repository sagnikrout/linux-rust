//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/shm.h
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
// SHMMNI, SHMMAX and SHMALL are default upper limits which can be
// modified by sysctl. The SHMMAX and SHMALL values have been chosen to
// be as large possible without facilitating scenarios where userspace
// causes overflows when adjusting the limits via operations of the form
// "retrieve current limit; add X; update limit". It is therefore not
// advised to make SHMMAX and SHMALL any larger. These limits are
// suitable for both 32 and 64-bit systems.
//

// Obsolete, used only for backwards compatibility and libc5 compiles
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_ds {
    pub /: *mut *mut ipc_perm shm_perm; / operation perms,
    pub /: *mut *mut int shm_segsz; / size of segment (bytes),
    pub /: *mut *mut __kernel_old_time_t shm_atime; / last attach time,
    pub /: *mut *mut __kernel_old_time_t shm_dtime; / last detach time,
    pub /: *mut *mut __kernel_old_time_t shm_ctime; / last change time,
    pub /: *mut *mut __kernel_ipc_pid_t shm_cpid; / pid of creator,
    pub /: *mut *mut __kernel_ipc_pid_t shm_lpid; / pid of last operator,
    pub /: *mut *mut unsigned short shm_nattch; / no. of current attaches,
    pub /: *mut *mut unsigned short shm_unused; / compatibility,
    pub /: *mut *mut *mut void shm_unused2; / ditto - used by DIPC,
    pub /: *mut *mut *mut void shm_unused3; / unused,
}

// Include the definition of shmid64_ds and shminfo64

//
// shmget() shmflg values.
//
// The bottom nine bits are the same as open(2) mode flags

// Bits 9 & 10 are IPC_CREAT and IPC_EXCL

//
// Huge page size encoding when SHM_HUGETLB is specified, and a huge page
// size other than the default is desired.  See hugetlb_encode.h
//

//
// shmat() shmflg values
//

// super user shmctl commands
pub const SHM_LOCK: c_int = 11;
pub const SHM_UNLOCK: c_int = 12;
// ipcs ctl commands
pub const SHM_STAT: c_int = 13;
pub const SHM_INFO: c_int = 14;
pub const SHM_STAT_ANY: c_int = 15;
// Obsolete, used only for backwards compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shminfo {
    pub shmmax: c_int,
    pub shmmin: c_int,
    pub shmmni: c_int,
    pub shmseg: c_int,
    pub shmall: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_info {
    pub used_ids: c_int,
    pub /: *mut *mut __kernel_ulong_t shm_tot; / total allocated shm,
    pub /: *mut *mut __kernel_ulong_t shm_rss; / total resident shm,
    pub /: *mut *mut __kernel_ulong_t shm_swp; / total swapped shm,
    pub swap_attempts: __kernel_ulong_t,
    pub swap_successes: __kernel_ulong_t,
}
