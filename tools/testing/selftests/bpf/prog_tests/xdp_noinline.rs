//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_noinline.c
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
pub unsafe extern "C" fn test_xdp_noinline() {
    void test_xdp_noinline(void)
    {
    let mut nr_cpus: c_uint = bpf_num_possible_cpus();
    struct test_xdp_noinline *skel;
    let mut key: vip = {.protocol = 6};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_meta {
    pub flags: __u32,
    pub vip_num: __u32,
    pub VIP_NUM}: } value = {.vip_num =,
    pub VIP_NUM: __u32 stats_key =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_stats {
    pub bytes: __u64,
    pub pkts: __u64,
    pub stats: [}; nr_cpus],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_definition {
    union {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

    __u8 flags;
    } real_def = {.dst = MAGIC_VAL};
    let mut ch_key: __u32 = 11, real_num = 3;
    int err, i;
    let mut bytes: __u64 = 0, pkts = 0;
    char buf[128];
    u32 *magic = (u32 *)buf;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = NUM_ITER,
    );
    skel = test_xdp_noinline__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    bpf_map_update_elem(bpf_map__fd(skel.maps.vip_map), &key, &value, 0);
    bpf_map_update_elem(bpf_map__fd(skel.maps.ch_rings), &ch_key, &real_num, 0);
    bpf_map_update_elem(bpf_map__fd(skel.maps.reals), &real_num, &real_def, 0);
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.balancer_ingress_v4), &topts);
    ASSERT_OK(err, "ipv4 test_run");
    ASSERT_EQ(topts.retval, 1, "ipv4 test_run retval");
    ASSERT_EQ(topts.data_size_out, 54, "ipv4 test_run data_size_out");
    ASSERT_EQ(*magic, MAGIC_VAL, "ipv4 test_run magic");
    topts.data_in = &pkt_v6;
    topts.data_size_in = sizeof(pkt_v6);
    topts.data_out = buf;
    topts.data_size_out = sizeof(buf);
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.balancer_ingress_v6), &topts);
    ASSERT_OK(err, "ipv6 test_run");
    ASSERT_EQ(topts.retval, 1, "ipv6 test_run retval");
    ASSERT_EQ(topts.data_size_out, 74, "ipv6 test_run data_size_out");
    ASSERT_EQ(*magic, MAGIC_VAL, "ipv6 test_run magic");
    bpf_map_lookup_elem(bpf_map__fd(skel.maps.stats), &stats_key, stats);
    for (i = 0; i < nr_cpus; i++) {
    bytes += stats[i].bytes;
    pkts += stats[i].pkts;
    }
    ASSERT_EQ(bytes, MAGIC_BYTES * NUM_ITER * 2, "stats bytes");
    ASSERT_EQ(pkts, NUM_ITER * 2, "stats pkts");
    test_xdp_noinline__destroy(skel);
    }
