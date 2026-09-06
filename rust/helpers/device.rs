//! Automatically rewritten from C to Rust
//! Source: rust/helpers/device.c
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

    __rust_helper int rust_helper_devm_add_action(struct device *dev,
    void (*action)(void *),
    void *data)
    {
    return devm_add_action(dev, action, data);
    }
    __rust_helper int rust_helper_devm_add_action_or_reset(struct device *dev,
    void (*action)(void *),
    void *data)
    {
    return devm_add_action_or_reset(dev, action, data);
    }
    __rust_helper void *rust_helper_dev_get_drvdata(const struct device *dev)
    {
    return dev_get_drvdata(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dev_set_drvdata(dev: *mut device, data: *mut c_void) -> __rust_helper void {
    __rust_helper void rust_helper_dev_set_drvdata(struct device *dev, void *data)
    {
    dev_set_drvdata(dev, data);
    }
    __rust_helper const char *rust_helper_dev_name(const struct device *dev)
    {
    return dev_name(dev);
    }
