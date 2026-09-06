//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_pwork.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2019 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//
extern "C" {
    pub fn int(mp: *mut *mut xfs_pwork_work_fn)(struct xfs_mount, pwork: *mut xfs_pwork) -> typedef;
}
//
// Parallel work coordination structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_pwork_ctl {
    pub wq: *mut workqueue_struct,
    pub mp: *mut xfs_mount,
    pub work_fn: xfs_pwork_work_fn,
    pub poll_wait: wait_queue_head,
    pub nr_work: core::sync::atomic::AtomicI32,
    pub error: c_int,
}

//
// Embed this parallel work control item inside your own work structure,
// then queue work with it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_pwork {
    pub work: work_struct,
    pub pctl: *mut xfs_pwork_ctl,
}

// Have we been told to abort?
extern "C" {
    pub fn xfs_pwork_ctl_want_abort(_arg: pwork->pctl) -> return;
}
extern "C" {
    pub fn xfs_pwork_queue(pctl: *mut xfs_pwork_ctl, pwork: *mut xfs_pwork);
}
extern "C" {
    pub fn xfs_pwork_destroy(pctl: *mut xfs_pwork_ctl) -> c_int;
}
extern "C" {
    pub fn xfs_pwork_poll(pctl: *mut xfs_pwork_ctl);
}
