//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp.c
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
pub unsafe extern "C" fn test_xdp() {
    void test_xdp(void)
    {
    let mut key4: vip = {.protocol = 6, .family = AF_INET};
    let mut key6: vip = {.protocol = 6, .family = AF_INET6};
    let mut value4: iptnl_info = {.family = AF_INET};
    let mut value6: iptnl_info = {.family = AF_INET6};
    const char *file = "./test_xdp.bpf.o";
    struct bpf_object *obj;
    char buf[128];
    struct ipv6hdr iph6;
    struct iphdr iph;
    int err, prog_fd, map_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = 1,
    );
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &obj, &prog_fd);
    if (CHECK_FAIL(err))
    return;
    map_fd = bpf_find_map(__func__, obj, "vip2tnl");
    if (map_fd < 0)
    goto out;
    bpf_map_update_elem(map_fd, &key4, &value4, 0);
    bpf_map_update_elem(map_fd, &key6, &value6, 0);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    memcpy(&iph, buf + sizeof(struct ethhdr), sizeof(iph));
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, XDP_TX, "ipv4 test_run retval");
    ASSERT_EQ(topts.data_size_out, 74, "ipv4 test_run data_size_out");
    ASSERT_EQ(iph.protocol, IPPROTO_IPIP, "ipv4 test_run iph.protocol");
    topts.data_in = &pkt_v6;
    topts.data_size_in = sizeof(pkt_v6);
    topts.data_size_out = sizeof(buf);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    memcpy(&iph6, buf + sizeof(struct ethhdr), sizeof(iph6));
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, XDP_TX, "ipv6 test_run retval");
    ASSERT_EQ(topts.data_size_out, 114, "ipv6 test_run data_size_out");
    ASSERT_EQ(iph6.nexthdr, IPPROTO_IPV6, "ipv6 test_run iph6.nexthdr");
    out:
    bpf_object__close(obj);
    }
