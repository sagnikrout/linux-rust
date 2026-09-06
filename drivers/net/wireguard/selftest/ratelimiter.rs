//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireguard/selftest/ratelimiter.c
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

    static const struct {
    bool result;
    unsigned int msec_to_sleep_before;
    } expected_results[] __initconst = {
    [0 ... PACKETS_BURSTABLE - 1] = { true, 0 },
    [PACKETS_BURSTABLE] = { false, 0 },
    [PACKETS_BURSTABLE + 1] = { true, MSEC_PER_SEC / PACKETS_PER_SECOND },
    [PACKETS_BURSTABLE + 2] = { false, 0 },
    [PACKETS_BURSTABLE + 3] = { true, (MSEC_PER_SEC / PACKETS_PER_SECOND) * 2 },
    [PACKETS_BURSTABLE + 4] = { true, 0 },
    [PACKETS_BURSTABLE + 5] = { false, 0 }
    };
#[no_mangle]
unsafe extern "C" fn maximum_jiffies_at_index(index: c_int) -> __init unsigned int {
    static __init unsigned int maximum_jiffies_at_index(int index)
    {
    let mut total_msecs: c_uint = 2 * MSEC_PER_SEC / PACKETS_PER_SECOND / 3;
    int i;
    for (i = 0; i <= index; ++i)
    total_msecs += expected_results[i].msec_to_sleep_before;
    return msecs_to_jiffies(total_msecs);
    }
    static __init int timings_test(struct sk_buff *skb4, struct iphdr *hdr4,
    struct sk_buff *skb6, struct ipv6hdr *hdr6,
    int *test)
    {
    unsigned long loop_start_time;
    int i;
    wg_ratelimiter_gc_entries(core::ptr::null_mut());
    rcu_barrier();
    loop_start_time = jiffies;
    for (i = 0; i < ARRAY_SIZE(expected_results); ++i) {
    if (expected_results[i].msec_to_sleep_before)
    msleep(expected_results[i].msec_to_sleep_before);
    if (time_is_before_jiffies(loop_start_time +
    maximum_jiffies_at_index(i)))
    return -ETIMEDOUT;
    if (wg_ratelimiter_allow(skb4, &init_net) !=
    expected_results[i].result)
    return -EXFULL;
    ++(*test);
    hdr4.saddr = htonl(ntohl(hdr4.saddr) + i + 1);
    if (time_is_before_jiffies(loop_start_time +
    maximum_jiffies_at_index(i)))
    return -ETIMEDOUT;
    if (!wg_ratelimiter_allow(skb4, &init_net))
    return -EXFULL;
    ++(*test);
    hdr4.saddr = htonl(ntohl(hdr4.saddr) - i - 1);

    hdr6.saddr.in6_u.u6_addr32[2] = htonl(i);
    hdr6.saddr.in6_u.u6_addr32[3] = htonl(i);
    if (time_is_before_jiffies(loop_start_time +
    maximum_jiffies_at_index(i)))
    return -ETIMEDOUT;
    if (wg_ratelimiter_allow(skb6, &init_net) !=
    expected_results[i].result)
    return -EXFULL;
    ++(*test);
    hdr6.saddr.in6_u.u6_addr32[0] =
    htonl(ntohl(hdr6.saddr.in6_u.u6_addr32[0]) + i + 1);
    if (time_is_before_jiffies(loop_start_time +
    maximum_jiffies_at_index(i)))
    return -ETIMEDOUT;
    if (!wg_ratelimiter_allow(skb6, &init_net))
    return -EXFULL;
    ++(*test);
    hdr6.saddr.in6_u.u6_addr32[0] =
    htonl(ntohl(hdr6.saddr.in6_u.u6_addr32[0]) - i - 1);
    if (time_is_before_jiffies(loop_start_time +
    maximum_jiffies_at_index(i)))
    return -ETIMEDOUT;

    }
    return 0;
    }
    static __init int capacity_test(struct sk_buff *skb4, struct iphdr *hdr4,
    int *test)
    {
    int i;
    wg_ratelimiter_gc_entries(core::ptr::null_mut());
    rcu_barrier();
    if (atomic_read(&total_entries))
    return -EXFULL;
    ++(*test);
    for (i = 0; i <= max_entries; ++i) {
    hdr4.saddr = htonl(i);
    if (wg_ratelimiter_allow(skb4, &init_net) != (i != max_entries))
    return -EXFULL;
    ++(*test);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn wg_ratelimiter_selftest() -> bool __init {
    bool __init wg_ratelimiter_selftest(void)
    {
    enum { TRIALS_BEFORE_GIVING_UP = 5000 };
    let mut success: bool = false;
    let mut test: c_int = 0, trials;
    struct sk_buff *skb4, *skb6 = core::ptr::null_mut();
    struct iphdr *hdr4;
    struct ipv6hdr *hdr6 = core::ptr::null_mut();
    if (IS_ENABLED(CONFIG_KASAN) || IS_ENABLED(CONFIG_UBSAN))
    return true;
    BUILD_BUG_ON(MSEC_PER_SEC % PACKETS_PER_SECOND != 0);
    if (wg_ratelimiter_init())
    goto out;
    ++test;
    if (wg_ratelimiter_init()) {
    wg_ratelimiter_uninit();
    goto out;
    }
    ++test;
    if (wg_ratelimiter_init()) {
    wg_ratelimiter_uninit();
    wg_ratelimiter_uninit();
    goto out;
    }
    ++test;
    skb4 = alloc_skb(sizeof(struct iphdr), GFP_KERNEL);
    if (unlikely(!skb4))
    goto err_nofree;
    skb4.protocol = htons(ETH_P_IP);
    hdr4 = (struct iphdr *)skb_put(skb4, sizeof(*hdr4));
    hdr4.saddr = htonl(8182);
    skb_reset_network_header(skb4);
    ++test;

    skb6 = alloc_skb(sizeof(struct ipv6hdr), GFP_KERNEL);
    if (unlikely(!skb6)) {
    kfree_skb(skb4);
    goto err_nofree;
    }
    skb6.protocol = htons(ETH_P_IPV6);
    hdr6 = (struct ipv6hdr *)skb_put(skb6, sizeof(*hdr6));
    hdr6.saddr.in6_u.u6_addr32[0] = htonl(1212);
    hdr6.saddr.in6_u.u6_addr32[1] = htonl(289188);
    skb_reset_network_header(skb6);
    ++test;

    for (trials = TRIALS_BEFORE_GIVING_UP; IS_ENABLED(DEBUG_RATELIMITER_TIMINGS);) {
    let mut test_count: c_int = 0, ret;
    ret = timings_test(skb4, hdr4, skb6, hdr6, &test_count);
    if (ret == -ETIMEDOUT) {
    if (!trials--) {
    test += test_count;
    goto err;
    }
    continue;
    } else if (ret < 0) {
    test += test_count;
    goto err;
    } else {
    test += test_count;
    break;
    }
    }
    for (trials = TRIALS_BEFORE_GIVING_UP;;) {
    let mut test_count: c_int = 0;
    if (capacity_test(skb4, hdr4, &test_count) < 0) {
    if (!trials--) {
    test += test_count;
    goto err;
    }
    continue;
    }
    test += test_count;
    break;
    }
    success = true;
    err:
    kfree_skb(skb4);

    kfree_skb(skb6);

    err_nofree:
    wg_ratelimiter_uninit();
    wg_ratelimiter_uninit();
    wg_ratelimiter_uninit();
// Uninit one extra time to check underflow detection.
    wg_ratelimiter_uninit();
    out:
    if (success)
    pr_info("ratelimiter self-tests: pass\n");
    else
    pr_err("ratelimiter self-test %d: FAIL\n", test);
    return success;
    }
