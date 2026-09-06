//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/parse_tcp_hdr_opt.c
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
pub struct test_pkt {
    pub pk6_v6: ipv6_packet,
    pub options: [u8; 16],
    pub __packed: },
    struct test_pkt pkt = {
    .pk6_v6.eth.h_proto = __bpf_constant_htons(ETH_P_IPV6),
    .pk6_v6.iph.nexthdr = IPPROTO_TCP,
    .pk6_v6.iph.payload_len = __bpf_constant_htons(MAGIC_BYTES),
    .pk6_v6.tcp.urg_ptr = 123,
    .pk6_v6.tcp.doff = 9, /* 16 bytes of options */
    .options = {
    TCPOPT_MSS, 4, 0x05, 0xB4, TCPOPT_NOP, TCPOPT_NOP,
    0, 6, 0xBB, 0xBB, 0xBB, 0xBB, TCPOPT_EOL
    },
}

#[no_mangle]
unsafe extern "C" fn test_parse_opt() {
    static void test_parse_opt(void)
    {
    struct test_parse_tcp_hdr_opt *skel;
    struct bpf_program *prog;
    char buf[128];
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt,
    .data_size_in = sizeof(pkt),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = 3,
    );
    skel = test_parse_tcp_hdr_opt__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    pkt.options[6] = skel.rodata.tcp_hdr_opt_kind_tpr;
    prog = skel.progs.xdp_ingress_v6;
    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &topts);
    ASSERT_OK(err, "ipv6 test_run");
    ASSERT_EQ(topts.retval, XDP_PASS, "ipv6 test_run retval");
    ASSERT_EQ(skel.bss.server_id, 0xBBBBBBBB, "server id");
    test_parse_tcp_hdr_opt__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_parse_opt_dynptr() {
    static void test_parse_opt_dynptr(void)
    {
    struct test_parse_tcp_hdr_opt_dynptr *skel;
    struct bpf_program *prog;
    char buf[128];
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt,
    .data_size_in = sizeof(pkt),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = 3,
    );
    skel = test_parse_tcp_hdr_opt_dynptr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    pkt.options[6] = skel.rodata.tcp_hdr_opt_kind_tpr;
    prog = skel.progs.xdp_ingress_v6;
    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &topts);
    ASSERT_OK(err, "ipv6 test_run");
    ASSERT_EQ(topts.retval, XDP_PASS, "ipv6 test_run retval");
    ASSERT_EQ(skel.bss.server_id, 0xBBBBBBBB, "server id");
    test_parse_tcp_hdr_opt_dynptr__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_parse_tcp_hdr_opt() {
    void test_parse_tcp_hdr_opt(void)
    {
    if (test__start_subtest("parse_tcp_hdr_opt"))
    test_parse_opt();
    if (test__start_subtest("parse_tcp_hdr_opt_dynptr"))
    test_parse_opt_dynptr();
    }
