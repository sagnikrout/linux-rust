//! Automatically rewritten from C to Rust
//! Source: lib/test_memcat_p.c
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
//
// Test cases for memcat_p() in lib/memcat_p.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_struct {
    pub num: c_int,
    pub magic: c_uint,
}

pub const MAGIC: c_uint = 0xf00ff00f;
// Size of each of the NULL-terminated input arrays
pub const INPUT_MAX: c_int = 128;
// Expected number of non-NULL elements in the output array

#[no_mangle]
unsafe extern "C" fn test_memcat_p_init() -> int __init {
    static int __init test_memcat_p_init(void)
    {
    struct test_struct **in0, **in1, **out, **p;
    let mut err: c_int = -ENOMEM, i, r, total = 0;
    in0 = kzalloc_objs(*in0, INPUT_MAX);
    if (!in0)
    return err;
    in1 = kzalloc_objs(*in1, INPUT_MAX);
    if (!in1)
    goto err_free_in0;
    for (i = 0, r = 1; i < INPUT_MAX - 1; i++) {
    in0[i] = kmalloc_obj(**in0);
    if (!in0[i])
    goto err_free_elements;
    in1[i] = kmalloc_obj(**in1);
    if (!in1[i]) {
    kfree(in0[i]);
    goto err_free_elements;
    }
// lifted from test_sort.c
    r = (r * 725861) % 6599;
    in0[i].num = r;
    in1[i].num = -r;
    in0[i].magic = MAGIC;
    in1[i].magic = MAGIC;
    }
    in0[i] = in1[i] = core::ptr::null_mut();
    out = memcat_p(in0, in1);
    if (!out)
    goto err_free_all_elements;
    err = -EINVAL;
    for (i = 0, p = out; *p && (i < INPUT_MAX * 2 - 1); p++, i++) {
    total += (*p).num;
    if ((*p).magic != MAGIC) {
    pr_err("test failed: wrong magic at %d: %u\n", i,
    (*p).magic);
    goto err_free_out;
    }
    }
    if (total) {
    pr_err("test failed: expected zero total, got %d\n", total);
    goto err_free_out;
    }
    if (i != EXPECT) {
    pr_err("test failed: expected output size %d, got %d\n",
    EXPECT, i);
    goto err_free_out;
    }
    for (i = 0; i < INPUT_MAX - 1; i++)
    if (out[i] != in0[i] || out[i + INPUT_MAX - 1] != in1[i]) {
    pr_err("test failed: wrong element order at %d\n", i);
    goto err_free_out;
    }
    err = 0;
    pr_info("test passed\n");
    err_free_out:
    kfree(out);
    err_free_all_elements:
    i = INPUT_MAX;
    err_free_elements:
    for (i--; i >= 0; i--) {
    kfree(in1[i]);
    kfree(in0[i]);
    }
    kfree(in1);
    err_free_in0:
    kfree(in0);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_memcat_p_exit() -> void __exit {
    static void __exit test_memcat_p_exit(void)
    {
    }
    module_init(test_memcat_p_init);
    module_exit(test_memcat_p_exit);
    MODULE_DESCRIPTION("Test cases for memcat_p() in lib/memcat_p.c");
    MODULE_LICENSE("GPL");
