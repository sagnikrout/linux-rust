//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/l4lb_all.c
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
unsafe extern "C" fn test_l4lb(file: *const c_char) {
    static void test_l4lb(const char *file)
    {
    let mut nr_cpus: c_uint = bpf_num_possible_cpus();
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
    int err, i, prog_fd, map_fd;
    let mut bytes: __u64 = 0, pkts = 0;
    struct bpf_object *obj;
    char buf[128];
    u32 *magic = (u32 *)buf;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = NUM_ITER,
    );
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_SCHED_CLS, &obj, &prog_fd);
    if (CHECK_FAIL(err))
    return;
    map_fd = bpf_find_map(__func__, obj, "vip_map");
    if (map_fd < 0)
    goto out;
    bpf_map_update_elem(map_fd, &key, &value, 0);
    map_fd = bpf_find_map(__func__, obj, "ch_rings");
    if (map_fd < 0)
    goto out;
    bpf_map_update_elem(map_fd, &ch_key, &real_num, 0);
    map_fd = bpf_find_map(__func__, obj, "reals");
    if (map_fd < 0)
    goto out;
    bpf_map_update_elem(map_fd, &real_num, &real_def, 0);
    topts.data_in = &pkt_v4;
    topts.data_size_in = sizeof(pkt_v4);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 7 /*TC_ACT_REDIRECT*/, "ipv4 test_run retval");
    ASSERT_EQ(topts.data_size_out, 54, "ipv4 test_run data_size_out");
    ASSERT_EQ(*magic, MAGIC_VAL, "ipv4 magic");
    topts.data_in = &pkt_v6;
    topts.data_size_in = sizeof(pkt_v6);
    topts.data_size_out = sizeof(buf); /* reset out size */
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 7 /*TC_ACT_REDIRECT*/, "ipv6 test_run retval");
    ASSERT_EQ(topts.data_size_out, 74, "ipv6 test_run data_size_out");
    ASSERT_EQ(*magic, MAGIC_VAL, "ipv6 magic");
    map_fd = bpf_find_map(__func__, obj, "stats");
    if (map_fd < 0)
    goto out;
    bpf_map_lookup_elem(map_fd, &stats_key, stats);
    for (i = 0; i < nr_cpus; i++) {
    bytes += stats[i].bytes;
    pkts += stats[i].pkts;
    }
    if (CHECK_FAIL(bytes != MAGIC_BYTES * NUM_ITER * 2 ||
    pkts != NUM_ITER * 2))
    printf("test_l4lb:FAIL:stats %lld %lld\n", bytes, pkts);
    out:
    bpf_object__close(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn test_l4lb_all() {
    void test_l4lb_all(void)
    {
    if (test__start_subtest("l4lb_inline"))
    test_l4lb("test_l4lb.bpf.o");
    if (test__start_subtest("l4lb_noinline"))
    test_l4lb("test_l4lb_noinline.bpf.o");
    if (test__start_subtest("l4lb_noinline_dynptr"))
    test_l4lb("test_l4lb_noinline_dynptr.bpf.o");
    }
