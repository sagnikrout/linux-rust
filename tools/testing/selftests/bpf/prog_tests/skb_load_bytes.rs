//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/skb_load_bytes.c
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

#[no_mangle]
pub unsafe extern "C" fn test_skb_load_bytes() {
    void test_skb_load_bytes(void)
    {
    struct skb_load_bytes *skel;
    int err, prog_fd, test_result;
    let mut skb: __sk_buff = { 0 };
    LIBBPF_OPTS(bpf_test_run_opts, tattr,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .ctx_in = &skb,
    .ctx_size_in = sizeof(skb),
    );
    skel = skb_load_bytes__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    prog_fd = bpf_program__fd(skel.progs.skb_process);
    if (!ASSERT_GE(prog_fd, 0, "prog_fd"))
    goto out;
    skel.bss.load_offset = (uint32_t)(-1);
    err = bpf_prog_test_run_opts(prog_fd, &tattr);
    if (!ASSERT_OK(err, "bpf_prog_test_run_opts"))
    goto out;
    test_result = skel.bss.test_result;
    if (!ASSERT_EQ(test_result, -EFAULT, "offset -1"))
    goto out;
    skel.bss.load_offset = (uint32_t)10;
    err = bpf_prog_test_run_opts(prog_fd, &tattr);
    if (!ASSERT_OK(err, "bpf_prog_test_run_opts"))
    goto out;
    test_result = skel.bss.test_result;
    if (!ASSERT_EQ(test_result, 0, "offset 10"))
    goto out;
    out:
    skb_load_bytes__destroy(skel);
    }
