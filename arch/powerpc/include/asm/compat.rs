//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/compat.h
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
// Architecture specific compatibility types
//

pub type compat_ipc_pid_t = u16;

pub type compat_nlink_t = i16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_stat {
    pub st_dev: compat_dev_t,
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_nlink_t,
    pub st_uid: __compat_uid32_t,
    pub st_gid: __compat_gid32_t,
    pub st_rdev: compat_dev_t,
    pub st_size: compat_off_t,
    pub st_blksize: compat_off_t,
    pub st_blocks: compat_off_t,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: u32,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: u32,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: u32,
    pub __unused4: [u32; 2],
}

//
// ipc64_perm is actually 32/64bit clean but since the compat layer refers to
// it we may as well define it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm {
    pub key: compat_key_t,
    pub uid: __compat_uid_t,
    pub gid: __compat_gid_t,
    pub cuid: __compat_uid_t,
    pub cgid: __compat_gid_t,
    pub mode: compat_mode_t,
    pub seq: c_uint,
    pub __pad2: c_uint,
    pub /: *mut *mut unsigned long __unused1; / yes they really are 64bit pads,
    pub __unused2: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_semid64_ds {
    pub sem_perm: compat_ipc64_perm,
    pub sem_otime_high: c_uint,
    pub sem_otime: c_uint,
    pub sem_ctime_high: c_uint,
    pub sem_ctime: c_uint,
    pub sem_nsems: compat_ulong_t,
    pub __unused3: compat_ulong_t,
    pub __unused4: compat_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_msqid64_ds {
    pub msg_perm: compat_ipc64_perm,
    pub msg_stime_high: c_uint,
    pub msg_stime: c_uint,
    pub msg_rtime_high: c_uint,
    pub msg_rtime: c_uint,
    pub msg_ctime_high: c_uint,
    pub msg_ctime: c_uint,
    pub msg_cbytes: compat_ulong_t,
    pub msg_qnum: compat_ulong_t,
    pub msg_qbytes: compat_ulong_t,
    pub msg_lspid: compat_pid_t,
    pub msg_lrpid: compat_pid_t,
    pub __unused4: compat_ulong_t,
    pub __unused5: compat_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_shmid64_ds {
    pub shm_perm: compat_ipc64_perm,
    pub shm_atime_high: c_uint,
    pub shm_atime: c_uint,
    pub shm_dtime_high: c_uint,
    pub shm_dtime: c_uint,
    pub shm_ctime_high: c_uint,
    pub shm_ctime: c_uint,
    pub __unused4: c_uint,
    pub shm_segsz: compat_size_t,
    pub shm_cpid: compat_pid_t,
    pub shm_lpid: compat_pid_t,
    pub shm_nattch: compat_ulong_t,
    pub __unused5: compat_ulong_t,
    pub __unused6: compat_ulong_t,
}

extern "C" {
    pub fn is_32bit_task() -> return;
}

