//! Automatically rewritten from C to Rust
//! Source: lib/fault-inject-usercopy.c
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


// SPDX-License-Identifier: GPL-2.0-only

    static struct {
    struct fault_attr attr;
    } fail_usercopy = {
    .attr = FAULT_ATTR_INITIALIZER,
    };
#[no_mangle]
unsafe extern "C" fn setup_fail_usercopy(str: *mut c_char) -> int __init {
    static int __init setup_fail_usercopy(char *str)
    {
    return setup_fault_attr(&fail_usercopy.attr, str);
    }
    __setup("fail_usercopy=", setup_fail_usercopy);

#[no_mangle]
unsafe extern "C" fn fail_usercopy_debugfs() -> int __init {
    static int __init fail_usercopy_debugfs(void)
    {
    struct dentry *dir;
    dir = fault_create_debugfs_attr("fail_usercopy", core::ptr::null_mut(),
    &fail_usercopy.attr);
    return PTR_ERR_OR_ZERO(dir);
    }
    late_initcall(fail_usercopy_debugfs);

#[no_mangle]
pub unsafe extern "C" fn should_fail_usercopy() -> bool {
    bool should_fail_usercopy(void)
    {
    return should_fail(&fail_usercopy.attr, 1);
    }
    EXPORT_SYMBOL_GPL(should_fail_usercopy);
