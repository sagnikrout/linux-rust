//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/blktrans.h
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
// Copyright © 2003-2010 David Woodhouse <dwmw2@infradead.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_blktrans_dev {
    pub tr: *mut mtd_blktrans_ops,
    pub list: list_head,
    pub mtd: *mut mtd_info,
    pub lock: mutex,
    pub devnum: c_int,
    pub bg_stop: bool,
    pub size: c_ulong,
    pub readonly: c_int,
    pub open: c_int,
    pub ref: kref,
    pub disk: *mut gendisk,
    pub disk_attributes: *mut attribute_group,
    pub rq: *mut request_queue,
    pub rq_list: list_head,
    pub tag_set: *mut blk_mq_tag_set,
    pub queue_lock: spinlock_t,
    pub priv: *mut c_void,
    pub writable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_blktrans_ops {
    pub name: *mut c_char,
    pub major: c_int,
    pub part_bits: c_int,
    pub blksize: c_int,
    pub blkshift: c_int,
// Access functions
    pub buffer): *mut unsigned long block, char,
    pub buffer): *mut unsigned long block, char,
    pub nr_blocks): unsigned long block, unsigned,
    pub dev): *mut *mut void (background)(struct mtd_blktrans_dev,
// Block layer ioctls
    pub geo): *mut *mut *mut int (getgeo)(struct mtd_blktrans_dev dev, struct hd_geometry,
    pub dev): *mut *mut int (flush)(struct mtd_blktrans_dev,
// Called with mtd_table_mutex held; no race with add/remove
    pub dev): *mut *mut int (open)(struct mtd_blktrans_dev,
    pub dev): *mut *mut void (release)(struct mtd_blktrans_dev,
// Called on {de,}registration and on subsequent addition/removal
    pub mtd): *mut *mut *mut void (add_mtd)(struct mtd_blktrans_ops tr, struct mtd_info,
    pub dev): *mut *mut void (remove_dev)(struct mtd_blktrans_dev,
    pub devs: list_head,
    pub list: list_head,
    pub owner: *mut module,
}

extern "C" {
    pub fn register_mtd_blktrans(tr: *mut mtd_blktrans_ops) -> c_int;
}
extern "C" {
    pub fn deregister_mtd_blktrans(tr: *mut mtd_blktrans_ops) -> c_int;
}
extern "C" {
    pub fn add_mtd_blktrans_dev(dev: *mut mtd_blktrans_dev) -> c_int;
}
extern "C" {
    pub fn del_mtd_blktrans_dev(dev: *mut mtd_blktrans_dev) -> c_int;
}
extern "C" {
    pub fn mtd_blktrans_cease_background(dev: *mut mtd_blktrans_dev) -> c_int;
}
//
// module_mtd_blktrans() - Helper macro for registering a mtd blktrans driver
// @__mtd_blktrans: mtd_blktrans_ops struct
//
// Helper macro for mtd blktrans drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

