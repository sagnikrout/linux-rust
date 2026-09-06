//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kfree_skb.c
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
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meta {
    pub ifindex: c_int,
    pub cb32_0: __u32,
    pub cb8_0: __u8,
}

    static union {
    __u32 cb32[5];
    __u8 cb8[20];
    } cb = {
    .cb32[0] = 0x81828384,
    };
#[no_mangle]
unsafe extern "C" fn on_sample(ctx: *mut c_void, cpu: c_int, data: *mut c_void, size: __u32) {
    static void on_sample(void *ctx, int cpu, void *data, __u32 size)
    {
    struct meta *meta = (struct meta *)data;
    struct ipv6_packet *pkt_v6 = data + sizeof(*meta);
    let mut duration: c_int = 0;
    if (CHECK(size != 72 + sizeof(*meta), "check_size", "size %u != %zu\n",
    size, 72 + sizeof(*meta)))
    return;
    if (CHECK(meta.ifindex != 1, "check_meta_ifindex",
    "meta.ifindex = %d\n", meta.ifindex))
// spurious kfree_skb not on loopback device
    return;
    if (CHECK(meta.cb8_0 != cb.cb8[0], "check_cb8_0", "cb8_0 %x != %x\n",
    meta.cb8_0, cb.cb8[0]))
    return;
    if (CHECK(meta.cb32_0 != cb.cb32[0], "check_cb32_0",
    "cb32_0 %x != %x\n",
    meta.cb32_0, cb.cb32[0]))
    return;
    if (CHECK(pkt_v6.eth.h_proto != htons(ETH_P_IPV6), "check_eth",
    "h_proto %x\n", pkt_v6.eth.h_proto))
    return;
    if (CHECK(pkt_v6.iph.nexthdr != 6, "check_ip",
    "iph.nexthdr %x\n", pkt_v6.iph.nexthdr))
    return;
    if (CHECK(pkt_v6.tcp.doff != 5, "check_tcp",
    "tcp.doff %x\n", pkt_v6.tcp.doff))
    return;
// (bool *)ctx = true;
    }
// TODO: fix kernel panic caused by this test in parallel mode
#[no_mangle]
pub unsafe extern "C" fn serial_test_kfree_skb() {
    void serial_test_kfree_skb(void)
    {
    let mut skb: __sk_buff = {};
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v6,
    .data_size_in = sizeof(pkt_v6),
    .ctx_in = &skb,
    .ctx_size_in = sizeof(skb),
    );
    struct kfree_skb *skel = core::ptr::null_mut();
    struct bpf_link *link;
    struct bpf_object *obj;
    struct perf_buffer *pb = core::ptr::null_mut();
    int err, prog_fd;
    let mut passed: bool = false;
    let mut duration: __u32 = 0;
    let mut zero: c_int = 0;
    bool test_ok[2];
    err = bpf_prog_test_load("./test_pkt_access.bpf.o", BPF_PROG_TYPE_SCHED_CLS,
    &obj, &prog_fd);
    if (CHECK(err, "prog_load sched cls", "err %d errno %d\n", err, errno))
    return;
    skel = kfree_skb__open_and_load();
    if (!ASSERT_OK_PTR(skel, "kfree_skb_skel"))
    goto close_prog;
    link = bpf_program__attach_raw_tracepoint(skel.progs.trace_kfree_skb, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(link, "attach_raw_tp"))
    goto close_prog;
    skel.links.trace_kfree_skb = link;
    link = bpf_program__attach_trace(skel.progs.fentry_eth_type_trans);
    if (!ASSERT_OK_PTR(link, "attach fentry"))
    goto close_prog;
    skel.links.fentry_eth_type_trans = link;
    link = bpf_program__attach_trace(skel.progs.fexit_eth_type_trans);
    if (!ASSERT_OK_PTR(link, "attach fexit"))
    goto close_prog;
    skel.links.fexit_eth_type_trans = link;
// set up perf buffer
    pb = perf_buffer__new(bpf_map__fd(skel.maps.perf_buf_map), 1,
    on_sample, core::ptr::null_mut(), &passed, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(pb, "perf_buf__new"))
    goto close_prog;
    memcpy(skb.cb, &cb, sizeof(cb));
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "ipv6 test_run");
    ASSERT_OK(topts.retval, "ipv6 test_run retval");
// read perf buffer
    err = perf_buffer__poll(pb, 100);
    if (CHECK(err < 0, "perf_buffer__poll", "err %d\n", err))
    goto close_prog;
// make sure kfree_skb program was triggered
// and it sent expected skb into ring buffer
//
    ASSERT_TRUE(passed, "passed");
    err = bpf_map_lookup_elem(bpf_map__fd(skel.maps.bss), &zero, test_ok);
    if (CHECK(err, "get_result",
    "failed to get output data: %d\n", err))
    goto close_prog;
    CHECK_FAIL(!test_ok[0] || !test_ok[1]);
    close_prog:
    perf_buffer__free(pb);
    bpf_object__close(obj);
    kfree_skb__destroy(skel);
    }
