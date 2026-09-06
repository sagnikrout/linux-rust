//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtio.h
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
// helper functions for implementing compat ioctls on the four tape
// drivers: we define the 32-bit layout of each incompatible structure,
// plus a wrapper function to copy it to user space in either format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtget32 {
    pub mt_type: i32,
    pub mt_resid: i32,
    pub mt_dsreg: i32,
    pub mt_gstat: i32,
    pub mt_erreg: i32,
    pub mt_fileno: i32,
    pub mt_blkno: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtpos32 {
    pub mt_blkno: i32,
}

extern "C" {
    pub fn put_user(_arg: k->mt_blkno, )u: *mut (u32 __user) -> return;
}
extern "C" {
    pub fn put_user(_arg: k->mt_blkno, )u: *mut (long __user) -> return;
}
