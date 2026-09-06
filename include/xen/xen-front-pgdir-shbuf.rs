//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/xen-front-pgdir-shbuf.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Xen frontend/backend page directory based shared buffer
// helper module.
//
// Copyright (C) 2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_front_pgdir_shbuf {
//
// Number of references granted for the backend use:
//
// - for frontend allocated/imported buffers this holds the number
// of grant references for the page directory and the pages
// of the buffer
//
// - for the buffer provided by the backend this only holds the number
// of grant references for the page directory itself as grant
// references for the buffer will be provided by the backend.
//
    pub num_grefs: c_int,
    pub grefs: *mut grant_ref_t,
// Page directory backing storage.
    pub directory: *mut u8,
//
// Number of pages for the shared buffer itself (excluding the page
// directory).
//
    pub num_pages: c_int,
//
// Backing storage of the shared buffer: these are the pages being
// shared.
//
    pub pages: *mut page,
    pub xb_dev: *mut xenbus_device,
// These are the ops used internally depending on be_alloc mode.
    pub ops: *const xen_front_pgdir_shbuf_ops,
// Xen map handles for the buffer allocated by the backend.
    pub backend_map_handles: *mut grant_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_front_pgdir_shbuf_cfg {
    pub xb_dev: *mut xenbus_device,
// Number of pages of the buffer backing storage.
    pub num_pages: c_int,
// Pages of the buffer to be shared.
    pub pages: *mut page,
//
// This is allocated outside because there are use-cases when
// the buffer structure is allocated as a part of a bigger one.
//
    pub pgdir: *mut xen_front_pgdir_shbuf,
//
// Mode of grant reference sharing: if set then backend will share
// grant references to the buffer with the frontend.
//
    pub be_alloc: c_int,
}

extern "C" {
    pub fn xen_front_pgdir_shbuf_alloc(cfg: *mut xen_front_pgdir_shbuf_cfg) -> c_int;
}
extern "C" {
    pub fn xen_front_pgdir_shbuf_map(buf: *mut xen_front_pgdir_shbuf) -> c_int;
}
extern "C" {
    pub fn xen_front_pgdir_shbuf_unmap(buf: *mut xen_front_pgdir_shbuf) -> c_int;
}
extern "C" {
    pub fn xen_front_pgdir_shbuf_free(buf: *mut xen_front_pgdir_shbuf);
}
