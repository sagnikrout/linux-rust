//! Automatically rewritten from C to Rust
//! Source: lib/test_bitops.c
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
//
// Copyright (C) 2020 Intel Corporation
//

// a tiny module only meant to test
//
// set/clear_bit
// get_count_order/long
//
// use an enum because that's the most common BITMAP usage
    enum bitops_fun {
    BITOPS_4 = 4,
    BITOPS_7 = 7,
    BITOPS_11 = 11,
    BITOPS_31 = 31,
    BITOPS_88 = 88,
    BITOPS_LAST = 255,
    BITOPS_LENGTH = 256
    };
    static DECLARE_BITMAP(g_bitmap, BITOPS_LENGTH);
    static unsigned int order_comb[][2] = {
    {0x00000003,  2},
    {0x00000004,  2},
    {0x00001fff, 13},
    {0x00002000, 13},
    {0x50000000, 31},
    {0x80000000, 31},
    {0x80003000, 32},
    };

    static unsigned long order_comb_long[][2] = {
    {0x0000000300000000, 34},
    {0x0000000400000000, 34},
    {0x00001fff00000000, 45},
    {0x0000200000000000, 45},
    {0x5000000000000000, 63},
    {0x8000000000000000, 63},
    {0x8000300000000000, 64},
    };

#[no_mangle]
unsafe extern "C" fn test_fns() -> int __init {
    static int __init test_fns(void)
    {
    static volatile __always_used unsigned long tmp __initdata;
    unsigned long *buf __free(kfree) = core::ptr::null_mut();
    unsigned int i, n;
    ktime_t time;
    buf = kmalloc_array(10000, sizeof(unsigned long), GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    get_random_bytes(buf, 10000 * sizeof(unsigned long));
    time = ktime_get();
    for (n = 0; n < BITS_PER_LONG; n++)
    for (i = 0; i < 10000; i++)
    tmp = fns(buf[i], n);
    time = ktime_get() - time;
    pr_err("fns:  %18llu ns\n", time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_bitops_startup() -> int __init {
    static int __init test_bitops_startup(void)
    {
    int i, bit_set;
    pr_info("Starting bitops test\n");
    set_bit(BITOPS_4, g_bitmap);
    set_bit(BITOPS_7, g_bitmap);
    set_bit(BITOPS_11, g_bitmap);
    set_bit(BITOPS_31, g_bitmap);
    set_bit(BITOPS_88, g_bitmap);
    for (i = 0; i < ARRAY_SIZE(order_comb); i++) {
    if (order_comb[i][1] != get_count_order(order_comb[i][0]))
    pr_warn("get_count_order wrong for %x\n",
    order_comb[i][0]);
    }
    for (i = 0; i < ARRAY_SIZE(order_comb); i++) {
    if (order_comb[i][1] != get_count_order_long(order_comb[i][0]))
    pr_warn("get_count_order_long wrong for %x\n",
    order_comb[i][0]);
    }

    for (i = 0; i < ARRAY_SIZE(order_comb_long); i++) {
    if (order_comb_long[i][1] !=
    get_count_order_long(order_comb_long[i][0]))
    pr_warn("get_count_order_long wrong for %lx\n",
    order_comb_long[i][0]);
    }

    barrier();
    clear_bit(BITOPS_4, g_bitmap);
    clear_bit(BITOPS_7, g_bitmap);
    clear_bit(BITOPS_11, g_bitmap);
    clear_bit(BITOPS_31, g_bitmap);
    clear_bit(BITOPS_88, g_bitmap);
    bit_set = find_first_bit(g_bitmap, BITOPS_LAST);
    if (bit_set != BITOPS_LAST)
    pr_err("ERROR: FOUND SET BIT %d\n", bit_set);
    test_fns();
    pr_info("Completed bitops test\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_bitops_unstartup() -> void __exit {
    static void __exit test_bitops_unstartup(void)
    {
    }
    module_init(test_bitops_startup);
    module_exit(test_bitops_unstartup);
    MODULE_AUTHOR("Jesse Brandeburg <jesse.brandeburg@intel.com>, Wei Yang <richard.weiyang@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Bit testing module");
