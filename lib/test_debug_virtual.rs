//! Automatically rewritten from C to Rust
//! Source: lib/test_debug_virtual.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub bar: c_uint,
}

    static struct foo *foo;
#[no_mangle]
unsafe extern "C" fn test_debug_virtual_init() -> int __init {
    static int __init test_debug_virtual_init(void)
    {
    phys_addr_t pa;
    void *va;
    va = (void *)VMALLOC_START;
    pa = virt_to_phys(va);
    pr_info("PA: %pa for VA: 0x%lx\n", &pa, (unsigned long)va);
    foo = kzalloc_obj(*foo);
    if (!foo)
    return -ENOMEM;
    pa = virt_to_phys(foo);
    va = foo;
    pr_info("PA: %pa for VA: 0x%lx\n", &pa, (unsigned long)va);
    return 0;
    }
    module_init(test_debug_virtual_init);
#[no_mangle]
unsafe extern "C" fn test_debug_virtual_exit() -> void __exit {
    static void __exit test_debug_virtual_exit(void)
    {
    kfree(foo);
    }
    module_exit(test_debug_virtual_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Test module for CONFIG_DEBUG_VIRTUAL");
