//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/umh.h
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


pub const UMH_NO_WAIT: c_uint = 0x00	/* don't wait at all */;
pub const UMH_WAIT_EXEC: c_uint = 0x01	/* wait for the exec, but not the process */;
pub const UMH_WAIT_PROC: c_uint = 0x02	/* wait for the process to complete */;
pub const UMH_KILLABLE: c_uint = 0x04	/* wait for EXEC/PROC killable */;
pub const UMH_FREEZABLE: c_uint = 0x08	/* wait for EXEC/PROC freezable */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subprocess_info {
    pub work: work_struct,
    pub complete: *mut completion,
    pub path: *const c_char,
    pub argv: *mut c_char,
    pub envp: *mut c_char,
    pub wait: c_int,
    pub retval: c_int,
    pub new): *mut *mut *mut int (init)(struct subprocess_info info, struct cred,
    pub info): *mut *mut void (cleanup)(struct subprocess_info,
    pub data: *mut c_void,
    pub __randomize_layout: },
    pub wait): *const *const *const *const *const *const call_usermodehelper(char path, char argv, char envp, int,
    pub data): *mut *mut *mut void (cleanup)(struct subprocess_info ), void,
    pub wait): *mut *mut call_usermodehelper_exec(struct subprocess_info info, int,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum umh_disable_depth {
    UMH_ENABLED = 0,
    UMH_FREEZING,
    UMH_DISABLED,
}

    pub depth): extern int __usermodehelper_disable(enum umh_disable_depth,
    pub depth): extern void __usermodehelper_set_disable_depth(enum umh_disable_depth,
    pub __usermodehelper_disable(UMH_DISABLED): return,
    pub usermodehelper_read_trylock(void): extern int,
    pub timeout): extern long usermodehelper_read_lock_wait(long,
    pub usermodehelper_read_unlock(void): extern void,
