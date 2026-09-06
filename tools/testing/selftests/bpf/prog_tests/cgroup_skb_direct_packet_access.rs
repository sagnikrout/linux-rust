//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_skb_direct_packet_access.c
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
pub unsafe extern "C" fn test_cgroup_skb_prog_run_direct_packet_access() {
    void test_cgroup_skb_prog_run_direct_packet_access(void)
    {
    int err;
    struct cgroup_skb_direct_packet_access *skel;
    char test_skb[64] = {};
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = test_skb,
    .data_size_in = sizeof(test_skb),
    );
    skel = cgroup_skb_direct_packet_access__open_and_load();
    if (!ASSERT_OK_PTR(skel, "cgroup_skb_direct_packet_access__open_and_load"))
    return;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.direct_packet_access), &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts err");
    ASSERT_EQ(topts.retval, 1, "retval");
    ASSERT_NEQ(skel.bss.data_end, 0, "data_end");
    cgroup_skb_direct_packet_access__destroy(skel);
    }
