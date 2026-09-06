//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/striper.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_object_extent {
    pub oe_item: list_head,
    pub oe_objno: u64,
    pub oe_off: u64,
    pub oe_len: u64,
}

//
// Called for each mapped stripe unit.
//
// @bytes: number of bytes mapped, i.e. the minimum of the full length
// requested (file extent length) or the remainder of the stripe
// unit within an object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_file_extent {
    pub fe_off: u64,
    pub fe_len: u64,
}

extern "C" {
    pub fn ceph_get_num_objects(l: *mut ceph_file_layout, size: u64) -> u64;
}
