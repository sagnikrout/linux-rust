//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireguard/selftest/counter.c
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[no_mangle]
pub unsafe extern "C" fn wg_packet_counter_selftest() -> bool __init {
    bool __init wg_packet_counter_selftest(void)
    {
    struct noise_replay_counter *counter;
    let mut test_num: c_uint = 0, i;
    let mut success: bool = true;
    counter = kmalloc_obj(*counter);
    if (unlikely(!counter)) {
    pr_err("nonce counter self-test malloc: FAIL\n");
    return false;
    }

    memset(counter, 0, sizeof(*counter));  \
    spin_lock_init(&counter.lock);        \
    } while (0)

    ++test_num;                                           \
    if (counter_validate(counter, n) != (v)) {            \
    pr_err("nonce counter self-test %u: FAIL\n",  \
    test_num);                             \
    success = false;                              \
    }                                                     \
    } while (0)
    T_INIT;
// 1 */ T(0, true);
// 2 */ T(1, true);
// 3 */ T(1, false);
// 4 */ T(9, true);
// 5 */ T(8, true);
// 6 */ T(7, true);
// 7 */ T(7, false);
// 8 */ T(T_LIM, true);
// 9 */ T(T_LIM - 1, true);
// 10 */ T(T_LIM - 1, false);
// 11 */ T(T_LIM - 2, true);
// 12 */ T(2, true);
// 13 */ T(2, false);
// 14 */ T(T_LIM + 16, true);
// 15 */ T(3, false);
// 16 */ T(T_LIM + 16, false);
// 17 */ T(T_LIM * 4, true);
// 18 */ T(T_LIM * 4 - (T_LIM - 1), true);
// 19 */ T(10, false);
// 20 */ T(T_LIM * 4 - T_LIM, false);
// 21 */ T(T_LIM * 4 - (T_LIM + 1), false);
// 22 */ T(T_LIM * 4 - (T_LIM - 2), true);
// 23 */ T(T_LIM * 4 + 1 - T_LIM, false);
// 24 */ T(0, false);
// 25 */ T(REJECT_AFTER_MESSAGES, false);
// 26 */ T(REJECT_AFTER_MESSAGES - 1, true);
// 27 */ T(REJECT_AFTER_MESSAGES, false);
// 28 */ T(REJECT_AFTER_MESSAGES - 1, false);
// 29 */ T(REJECT_AFTER_MESSAGES - 2, true);
// 30 */ T(REJECT_AFTER_MESSAGES + 1, false);
// 31 */ T(REJECT_AFTER_MESSAGES + 2, false);
// 32 */ T(REJECT_AFTER_MESSAGES - 2, false);
// 33 */ T(REJECT_AFTER_MESSAGES - 3, true);
// 34 */ T(0, false);
    T_INIT;
    for (i = 1; i <= COUNTER_WINDOW_SIZE; ++i)
    T(i, true);
    T(0, true);
    T(0, false);
    T_INIT;
    for (i = 2; i <= COUNTER_WINDOW_SIZE + 1; ++i)
    T(i, true);
    T(1, true);
    T(0, false);
    T_INIT;
    for (i = COUNTER_WINDOW_SIZE + 1; i-- > 0;)
    T(i, true);
    T_INIT;
    for (i = COUNTER_WINDOW_SIZE + 2; i-- > 1;)
    T(i, true);
    T(0, false);
    T_INIT;
    for (i = COUNTER_WINDOW_SIZE + 1; i-- > 1;)
    T(i, true);
    T(COUNTER_WINDOW_SIZE + 1, true);
    T(0, false);
    T_INIT;
    for (i = COUNTER_WINDOW_SIZE + 1; i-- > 1;)
    T(i, true);
    T(0, true);
    T(COUNTER_WINDOW_SIZE + 1, true);

    if (success)
    pr_info("nonce counter self-tests: pass\n");
    kfree(counter);
    return success;
    }
