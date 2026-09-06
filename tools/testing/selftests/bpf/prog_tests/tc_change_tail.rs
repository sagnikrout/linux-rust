//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tc_change_tail.c
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

pub const LO_IFINDEX: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn test_tc_change_tail() {
    void test_tc_change_tail(void)
    {
    LIBBPF_OPTS(bpf_tcx_opts, tcx_opts);
    struct test_tc_change_tail *skel = core::ptr::null_mut();
    struct bpf_link *link;
    int c1, p1;
    char buf[2];
    int ret;
    skel = test_tc_change_tail__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_tc_change_tail__open_and_load"))
    return;
    link = bpf_program__attach_tcx(skel.progs.change_tail, LO_IFINDEX,
    &tcx_opts);
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_tcx"))
    goto destroy;
    skel.links.change_tail = link;
    ret = create_pair(AF_INET, SOCK_DGRAM, &c1, &p1);
    if (!ASSERT_OK(ret, "create_pair"))
    goto destroy;
    ret = xsend(p1, "Tr", 2, 0);
    ASSERT_EQ(ret, 2, "xsend(p1)");
    ret = recv(c1, buf, 2, 0);
    ASSERT_EQ(ret, 2, "recv(c1)");
    ASSERT_EQ(skel.data.change_tail_ret, 0, "change_tail_ret");
    ret = xsend(p1, "G", 1, 0);
    ASSERT_EQ(ret, 1, "xsend(p1)");
    ret = recv(c1, buf, 2, 0);
    ASSERT_EQ(ret, 1, "recv(c1)");
    ASSERT_EQ(skel.data.change_tail_ret, 0, "change_tail_ret");
    ret = xsend(p1, "E", 1, 0);
    ASSERT_EQ(ret, 1, "xsend(p1)");
    ret = recv(c1, buf, 1, 0);
    ASSERT_EQ(ret, 1, "recv(c1)");
    ASSERT_EQ(skel.data.change_tail_ret, -EINVAL, "change_tail_ret");
    ret = xsend(p1, "Z", 1, 0);
    ASSERT_EQ(ret, 1, "xsend(p1)");
    ret = recv(c1, buf, 1, 0);
    ASSERT_EQ(ret, 1, "recv(c1)");
    ASSERT_EQ(skel.data.change_tail_ret, -EINVAL, "change_tail_ret");
    close(c1);
    close(p1);
    destroy:
    test_tc_change_tail__destroy(skel);
    }
