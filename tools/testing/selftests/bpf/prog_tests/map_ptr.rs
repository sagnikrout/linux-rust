//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/map_ptr.c
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
// Copyright (c) 2020 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_map_ptr() {
    void test_map_ptr(void)
    {
    struct map_ptr_kern_lskel *skel;
    char buf[128];
    int err;
    let mut page_size: c_int = getpagesize();
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = 1,
    );
    skel = map_ptr_kern_lskel__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.maps.m_ringbuf.max_entries = page_size;
    err = map_ptr_kern_lskel__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    skel.bss.page_size = page_size;
    err = bpf_prog_test_run_opts(skel.progs.cg_skb.prog_fd, &topts);
    if (!ASSERT_OK(err, "test_run"))
    goto cleanup;
    if (!ASSERT_NEQ(topts.retval, 0, "test_run retval"))
    goto cleanup;
    cleanup:
    map_ptr_kern_lskel__destroy(skel);
    }
