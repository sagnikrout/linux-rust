//! Automatically rewritten from C to Rust
//! Source: rust/helpers/serdev.c
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

    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_serdev_device_driver_unregister(sdrv: *mut serdev_device_driver) {
    void rust_helper_serdev_device_driver_unregister(struct serdev_device_driver *sdrv)
    {
    serdev_device_driver_unregister(sdrv);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_serdev_device_put(serdev: *mut serdev_device) {
    void rust_helper_serdev_device_put(struct serdev_device *serdev)
    {
    serdev_device_put(serdev);
    }
    __rust_helper
    void rust_helper_serdev_device_set_client_ops(struct serdev_device *serdev,
    const struct serdev_device_ops *ops)
    {
    serdev_device_set_client_ops(serdev, ops);
    }
