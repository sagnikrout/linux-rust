//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/snprintf_btf.c
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

// Demonstrate that bpf_snprintf_btf succeeds and that various data types
// are formatted correctly.
//
#[no_mangle]
pub unsafe extern "C" fn serial_test_snprintf_btf() {
    void serial_test_snprintf_btf(void)
    {
    struct netif_receive_skb *skel;
    struct netif_receive_skb__bss *bss;
    int err, duration = 0;
    skel = netif_receive_skb__open();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
    err = netif_receive_skb__load(skel);
    if (CHECK(err, "skel_load", "failed to load skeleton: %d\n", err))
    goto cleanup;
    bss = skel.bss;
    err = netif_receive_skb__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attach failed: %d\n", err))
    goto cleanup;
// generate receive event
    err = system("ping -c 1 127.0.0.1 > /dev/null");
    if (CHECK(err, "system", "ping failed: %d\n", err))
    goto cleanup;
    if (bss.skip) {
    printf("%s:SKIP:no __builtin_btf_type_id\n", __func__);
    test__skip();
    goto cleanup;
    }
//
// Make sure netif_receive_skb program was triggered
// and it set expected return values from bpf_trace_printk()s
// and all tests ran.
//
    if (!ASSERT_GT(bss.ret, 0, "bpf_snprintf_ret"))
    goto cleanup;
    if (CHECK(bss.ran_subtests == 0, "check if subtests ran",
    "no subtests ran, did BPF program run?"))
    goto cleanup;
    if (CHECK(bss.num_subtests != bss.ran_subtests,
    "check all subtests ran",
    "only ran %d of %d tests\n", bss.num_subtests,
    bss.ran_subtests))
    goto cleanup;
    cleanup:
    netif_receive_skb__destroy(skel);
    }
