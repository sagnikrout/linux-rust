//! Automatically rewritten from C to Rust
//! Source: lib/test_fpu_glue.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Test cases for using floating point operations inside a kernel module.
//
// This tests kernel_fpu_begin() and kernel_fpu_end() functions, especially
// when userland has modified the floating point control registers. The kernel
// state might depend on the state set by the userland thread that was active
// before a syscall.
//
// To facilitate the test, this module registers file
// /sys/kernel/debug/selftest_helpers/test_fpu, which when read causes a
// sequence of floating point operations. If the operations fail, either the
// read returns error status or the kernel crashes.
// If the operations succeed, the read returns "1\n".
//

#[no_mangle]
unsafe extern "C" fn test_fpu_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int test_fpu_get(void *data, u64 *val)
    {
    let mut status: c_int = -EINVAL;
    kernel_fpu_begin();
    status = test_fpu();
    kernel_fpu_end();
// val = 1;
    return status;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(test_fpu_fops, test_fpu_get, core::ptr::null_mut(), "%lld\n");
    static struct dentry *selftest_dir;
#[no_mangle]
unsafe extern "C" fn test_fpu_init() -> int __init {
    static int __init test_fpu_init(void)
    {
    if (!kernel_fpu_available())
    return -EINVAL;
    selftest_dir = debugfs_create_dir("selftest_helpers", core::ptr::null_mut());
    if (IS_ERR(selftest_dir))
    return -ENOMEM;
    debugfs_create_file_unsafe("test_fpu", 0444, selftest_dir, core::ptr::null_mut(),
    &test_fpu_fops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_fpu_exit() -> void __exit {
    static void __exit test_fpu_exit(void)
    {
    debugfs_remove(selftest_dir);
    }
    module_init(test_fpu_init);
    module_exit(test_fpu_exit);
    MODULE_DESCRIPTION("Test cases for floating point operations");
    MODULE_LICENSE("GPL");
