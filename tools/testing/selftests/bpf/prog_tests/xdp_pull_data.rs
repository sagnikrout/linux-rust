//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_pull_data.c
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

pub const XDP_PACKET_HEADROOM: c_int = 256;
// Find headroom and tailroom occupied by struct xdp_frame and struct
// skb_shared_info so that we can calculate the maximum pull lengths for
// test cases. They might not be the real size of the structures due to
// cache alignment.
//
#[no_mangle]
unsafe extern "C" fn find_xdp_sizes(skel: *mut test_xdp_pull_data, frame_sz: c_int) -> c_int {
    static int find_xdp_sizes(struct test_xdp_pull_data *skel, int frame_sz)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    let mut ctx: xdp_md = {};
    int prog_fd, err;
    __u8 *buf;
    buf = calloc(frame_sz, sizeof(__u8));
    if (!ASSERT_OK_PTR(buf, "calloc buf"))
    return -ENOMEM;
    topts.data_in = buf;
    topts.data_out = buf;
    topts.data_size_in = frame_sz;
    topts.data_size_out = frame_sz;
// Pass a data_end larger than the linear space available to make sure
// bpf_prog_test_run_xdp() will fill the linear data area so that
// xdp_find_sizes can infer the size of struct skb_shared_info
//
    ctx.data_end = frame_sz;
    topts.ctx_in = &ctx;
    topts.ctx_out = &ctx;
    topts.ctx_size_in = sizeof(ctx);
    topts.ctx_size_out = sizeof(ctx);
    prog_fd = bpf_program__fd(skel.progs.xdp_find_sizes);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    free(buf);
    return err;
    }
// xdp_pull_data_prog will directly read a marker 0xbb stored at buf[1024]
// so caller expecting XDP_PASS should always pass pull_len no less than 1024
//
    static void run_test(struct test_xdp_pull_data *skel, int retval,
    int frame_sz, int buff_len, int meta_len, int data_len,
    int pull_len)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    let mut ctx: xdp_md = {};
    int prog_fd, err;
    __u8 *buf;
    buf = calloc(buff_len, sizeof(__u8));
    if (!ASSERT_OK_PTR(buf, "calloc buf"))
    return;
    buf[meta_len + 1023] = 0xaa;
    buf[meta_len + 1024] = 0xbb;
    buf[meta_len + 1025] = 0xcc;
    topts.data_in = buf;
    topts.data_out = buf;
    topts.data_size_in = buff_len;
    topts.data_size_out = buff_len;
    ctx.data = meta_len;
    ctx.data_end = meta_len + data_len;
    topts.ctx_in = &ctx;
    topts.ctx_out = &ctx;
    topts.ctx_size_in = sizeof(ctx);
    topts.ctx_size_out = sizeof(ctx);
    skel.bss.data_len = data_len;
    if (pull_len & PULL_MAX) {
    let mut headroom: c_int = XDP_PACKET_HEADROOM - meta_len - skel.bss.xdpf_sz;
    int tailroom = frame_sz - XDP_PACKET_HEADROOM -
    data_len - skel.bss.sinfo_sz;
    pull_len = pull_len & PULL_PLUS_ONE ? 1 : 0;
    pull_len += headroom + tailroom + data_len;
    }
    skel.bss.pull_len = pull_len;
    prog_fd = bpf_program__fd(skel.progs.xdp_pull_data_prog);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(topts.retval, retval, "xdp_pull_data_prog retval");
    if (retval == XDP_DROP)
    goto out;
    ASSERT_EQ(ctx.data_end, meta_len + pull_len, "linear data size");
    ASSERT_EQ(topts.data_size_out, buff_len, "linear + non-linear data size");
// Make sure data around xdp->data_end was not messed up by
// bpf_xdp_pull_data()
//
    ASSERT_EQ(buf[meta_len + 1023], 0xaa, "data[1023]");
    ASSERT_EQ(buf[meta_len + 1024], 0xbb, "data[1024]");
    ASSERT_EQ(buf[meta_len + 1025], 0xcc, "data[1025]");
    out:
    free(buf);
    }
#[no_mangle]
unsafe extern "C" fn test_xdp_pull_data_basic() {
    static void test_xdp_pull_data_basic(void)
    {
    u32 pg_sz, max_meta_len, max_data_len;
    struct test_xdp_pull_data *skel;
    int buff_len;
    skel = test_xdp_pull_data__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_xdp_pull_data__open_and_load"))
    return;
    pg_sz = sysconf(_SC_PAGE_SIZE);
    buff_len = pg_sz + pg_sz / 2;
    if (find_xdp_sizes(skel, pg_sz))
    goto out;
    max_meta_len = XDP_PACKET_HEADROOM - skel.bss.xdpf_sz;
    max_data_len = pg_sz - XDP_PACKET_HEADROOM - skel.bss.sinfo_sz;
// linear xdp pkt, pull 0 byte
    run_test(skel, XDP_PASS, pg_sz, 2048, 0, 2048, 2048);
// multi-buf pkt, pull results in linear xdp pkt
    run_test(skel, XDP_PASS, pg_sz, 2048, 0, 1024, 2048);
// multi-buf pkt, pull 1 byte to linear data area
    run_test(skel, XDP_PASS, pg_sz, 9000, 0, 1024, 1025);
// multi-buf pkt, pull 0 byte to linear data area
    run_test(skel, XDP_PASS, pg_sz, 9000, 0, 1025, 1025);
// multi-buf pkt, empty linear data area, pull requires memmove
    run_test(skel, XDP_PASS, pg_sz, buff_len, 0, 0, PULL_MAX);
// multi-buf pkt, no headroom
    run_test(skel, XDP_PASS, pg_sz, buff_len, max_meta_len, 1024, PULL_MAX);
// multi-buf pkt, no tailroom, pull requires memmove
    run_test(skel, XDP_PASS, pg_sz, buff_len, 0, max_data_len, PULL_MAX);
// Test cases with invalid pull length
// linear xdp pkt, pull more than total data len
    run_test(skel, XDP_DROP, pg_sz, 2048, 0, 2048, 2049);
// multi-buf pkt with no space left in linear data area
    run_test(skel, XDP_DROP, pg_sz, buff_len, max_meta_len, max_data_len,
    PULL_MAX | PULL_PLUS_ONE);
// multi-buf pkt, empty linear data area
    run_test(skel, XDP_DROP, pg_sz, buff_len, 0, 0, PULL_MAX | PULL_PLUS_ONE);
// multi-buf pkt, no headroom
    run_test(skel, XDP_DROP, pg_sz, buff_len, max_meta_len, 1024,
    PULL_MAX | PULL_PLUS_ONE);
// multi-buf pkt, no tailroom
    run_test(skel, XDP_DROP, pg_sz, buff_len, 0, max_data_len,
    PULL_MAX | PULL_PLUS_ONE);
    out:
    test_xdp_pull_data__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_xdp_pull_data() {
    void test_xdp_pull_data(void)
    {
    if (test__start_subtest("xdp_pull_data"))
    test_xdp_pull_data_basic();
    }
