//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_info.c
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

pub const IFINDEX_LO: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn serial_test_xdp_info() {
    void serial_test_xdp_info(void)
    {
    let mut len: __u32 = sizeof(struct bpf_prog_info), duration = 0, prog_id;
    const char *file = "./xdp_dummy.bpf.o";
    LIBBPF_OPTS(bpf_xdp_query_opts, opts);
    let mut info: bpf_prog_info = {};
    struct bpf_object *obj;
    int err, prog_fd;
// Get prog_id for XDP_ATTACHED_NONE mode
    err = bpf_xdp_query_id(IFINDEX_LO, 0, &prog_id);
    if (CHECK(err, "get_xdp_none", "errno=%d\n", errno))
    return;
    if (CHECK(prog_id, "prog_id_none", "unexpected prog_id=%u\n", prog_id))
    return;
    err = bpf_xdp_query_id(IFINDEX_LO, XDP_FLAGS_SKB_MODE, &prog_id);
    if (CHECK(err, "get_xdp_none_skb", "errno=%d\n", errno))
    return;
    if (CHECK(prog_id, "prog_id_none_skb", "unexpected prog_id=%u\n",
    prog_id))
    return;
// Setup prog
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &obj, &prog_fd);
    if (CHECK_FAIL(err))
    return;
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &len);
    if (CHECK(err, "get_prog_info", "errno=%d\n", errno))
    goto out_close;
    err = bpf_xdp_attach(IFINDEX_LO, prog_fd, XDP_FLAGS_SKB_MODE, core::ptr::null_mut());
    if (CHECK(err, "set_xdp_skb", "errno=%d\n", errno))
    goto out_close;
// Get prog_id for single prog mode
    err = bpf_xdp_query_id(IFINDEX_LO, 0, &prog_id);
    if (CHECK(err, "get_xdp", "errno=%d\n", errno))
    goto out;
    if (CHECK(prog_id != info.id, "prog_id", "prog_id not available\n"))
    goto out;
    err = bpf_xdp_query_id(IFINDEX_LO, XDP_FLAGS_SKB_MODE, &prog_id);
    if (CHECK(err, "get_xdp_skb", "errno=%d\n", errno))
    goto out;
    if (CHECK(prog_id != info.id, "prog_id_skb", "prog_id not available\n"))
    goto out;
    err = bpf_xdp_query_id(IFINDEX_LO, XDP_FLAGS_DRV_MODE, &prog_id);
    if (CHECK(err, "get_xdp_drv", "errno=%d\n", errno))
    goto out;
    if (CHECK(prog_id, "prog_id_drv", "unexpected prog_id=%u\n", prog_id))
    goto out;
// Check xdp features supported by lo device
    opts.feature_flags = ~0;
    err = bpf_xdp_query(IFINDEX_LO, XDP_FLAGS_DRV_MODE, &opts);
    if (!ASSERT_OK(err, "bpf_xdp_query"))
    goto out;
    ASSERT_EQ(opts.feature_flags, 0, "opts.feature_flags");
    out:
    bpf_xdp_detach(IFINDEX_LO, 0, core::ptr::null_mut());
    out_close:
    bpf_object__close(obj);
    }
