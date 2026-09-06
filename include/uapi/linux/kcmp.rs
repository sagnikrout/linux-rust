//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kcmp.h
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

// Comparison type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kcmp_type {
    KCMP_FILE,
    KCMP_VM,
    KCMP_FILES,
    KCMP_FS,
    KCMP_SIGHAND,
    KCMP_IO,
    KCMP_SYSVSEM,
    KCMP_EPOLL_TFD,

    KCMP_TYPES,
}

// Slot for KCMP_EPOLL_TFD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcmp_epoll_slot {
    pub /: *mut *mut __u32 efd; / epoll file descriptor,
    pub /: *mut *mut __u32 tfd; / target file number,
    pub /: *mut *mut __u32 toff; / target offset within same numbered sequence,
}
