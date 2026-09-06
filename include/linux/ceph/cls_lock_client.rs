//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/cls_lock_client.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_cls_lock_type {
    CEPH_CLS_LOCK_NONE = 0,
    CEPH_CLS_LOCK_EXCLUSIVE = 1,
    CEPH_CLS_LOCK_SHARED = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_locker_id {
    pub /: *mut *mut ceph_entity_name name; / locker's client name,
    pub /: *mut *mut *mut char cookie; / locker's cookie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_locker_info {
    pub /: *mut *mut ceph_entity_addr addr; / locker's address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_locker {
    pub id: ceph_locker_id,
    pub info: ceph_locker_info,
}

extern "C" {
    pub fn ceph_free_lockers(lockers: *mut ceph_locker, num_lockers: u32);
}
