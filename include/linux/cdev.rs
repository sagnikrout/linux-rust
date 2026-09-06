//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cdev.h
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
pub struct cdev {
    pub kobj: kobject,
    pub owner: *mut module,
    pub ops: *const file_operations,
    pub list: list_head,
    pub dev: dev_t,
    pub count: c_uint,
    pub __randomize_layout: },
    pub ): *const *const void cdev_init(struct cdev , struct file_operations,
    pub cdev_alloc(void): *mut cdev,
    pub p): *mut void cdev_put(struct cdev,
    pub unsigned): *mut *mut int cdev_add(struct cdev , dev_t,,
    pub kobj): *mut *mut void cdev_set_parent(struct cdev p, struct kobject,
    pub dev): *mut *mut int cdev_device_add(struct cdev cdev, struct device,
    pub dev): *mut *mut void cdev_device_del(struct cdev cdev, struct device,
    pub ): *mut void cdev_del(struct cdev,
    pub ): *mut void cd_forget(struct inode,
