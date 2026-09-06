//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sem.h
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

// semop flags
pub const SEM_UNDO: c_uint = 0x1000  /* undo the operation on exit */;
// semctl Command Definitions.

// ipcs ctl cmds
pub const SEM_STAT: c_int = 18;
pub const SEM_INFO: c_int = 19;
pub const SEM_STAT_ANY: c_int = 20;
// Obsolete, used only for backwards compatibility and libc5 compiles
#[repr(C)]
#[derive(Copy, Clone)]
pub struct semid_ds {
    pub /: *mut *mut ipc_perm sem_perm; / permissions .. see ipc.h,
    pub /: *mut *mut __kernel_old_time_t sem_otime; / last semop time,
    pub /: *mut *mut __kernel_old_time_t sem_ctime; / create/last semctl() time,
    pub /: *mut *mut *mut sem sem_base; / ptr to first semaphore in array,
    pub /: *mut *mut *mut sem_queue sem_pending; / pending operations to be processed,
    pub /: *mut *mut *mut *mut sem_queue sem_pending_last; / last pending operation,
    pub /: *mut *mut *mut sem_undo undo; / undo requests on this array,
    pub /: *mut *mut unsigned short sem_nsems; / no. of semaphores in array,
}

// Include the definition of semid64_ds

// semop system calls takes an array of these.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf {
    pub /: *mut *mut unsigned short sem_num; / semaphore index in array,
    pub /: *mut *mut short sem_op; / semaphore operation,
    pub /: *mut *mut short sem_flg; / operation flags,
}

// arg for semctl system calls.
#[repr(C)]
#[derive(Copy, Clone)]
pub union semun {
    pub /: *mut *mut int val; / value for SETVAL,
    pub /: *mut *mut *mut semid_ds __user buf; / buffer for IPC_STAT & IPC_SET,
    pub /: *mut *mut *mut unsigned short __user array; / array for GETALL & SETALL,
    pub /: *mut *mut *mut seminfo __user __buf; / buffer for IPC_INFO,
    pub __pad: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seminfo {
    pub semmap: c_int,
    pub semmni: c_int,
    pub semmns: c_int,
    pub semmnu: c_int,
    pub semmsl: c_int,
    pub semopm: c_int,
    pub semume: c_int,
    pub semusz: c_int,
    pub semvmx: c_int,
    pub semaem: c_int,
}

//
// SEMMNI, SEMMSL and SEMMNS are default values which can be
// modified by sysctl.
// The values has been chosen to be larger than necessary for any
// known configuration.
//
// SEMOPM should not be increased beyond 1000, otherwise there is the
// risk that semop()/semtimedop() fails due to kernel memory fragmentation when
// allocating the sop array.
//

// unused

