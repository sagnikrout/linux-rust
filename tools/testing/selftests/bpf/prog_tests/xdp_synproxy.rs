//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_synproxy.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// Macro flag: #define _GNU_SOURCE

pub const CMD_OUT_BUF_SIZE: c_int = 1023;

    char buf[1024]; \
    snprintf(buf, sizeof(buf), (cmd), ##__VA_ARGS__); \
    FILE *f = popen(buf, "r"); \
    if (!ASSERT_OK_PTR(f, buf)) \
    goto out; \
    f; \
    })
// out must be at least `size * 4 + 1` bytes long
#[no_mangle]
unsafe extern "C" fn escape_str(out: *mut c_char, in: *const c_char, size: usize) {
    static void escape_str(char *out, const char *in, size_t size)
    {
    static const char *hex = "0123456789ABCDEF";
    size_t i;
    for (i = 0; i < size; i++) {
    if (isprint(in[i]) && in[i] != '\\' && in[i] != '\'') {
// out++ = in[i];
    } else {
// out++ = '\\';
// out++ = 'x';
// out++ = hex[(in[i] >> 4) & 0xf];
// out++ = hex[in[i] & 0xf];
    }
    }
// out++ = '\0';
    }
#[no_mangle]
unsafe extern "C" fn expect_str(buf: *mut c_char, size: usize, str: *const c_char, name: *const c_char) -> bool {
    static bool expect_str(char *buf, size_t size, const char *str, const char *name)
    {
    static char escbuf_expected[CMD_OUT_BUF_SIZE * 4];
    static char escbuf_actual[CMD_OUT_BUF_SIZE * 4];
    let mut duration: static int = 0;
    bool ok;
    ok = size == strlen(str) && !memcmp(buf, str, size);
    if (!ok) {
    escape_str(escbuf_expected, str, strlen(str));
    escape_str(escbuf_actual, buf, size);
    }
    CHECK(!ok, name, "unexpected %s: actual '%s' != expected '%s'\n",
    name, escbuf_actual, escbuf_expected);
    return ok;
    }
#[no_mangle]
unsafe extern "C" fn test_synproxy(xdp: bool) {
    static void test_synproxy(bool xdp)
    {
    let mut server_fd: c_int = -1, client_fd = -1, accept_fd = -1;
    char *prog_id = core::ptr::null_mut(), *prog_id_end;
    struct nstoken *ns = core::ptr::null_mut();
    FILE *ctrl_file = core::ptr::null_mut();
    char buf[CMD_OUT_BUF_SIZE];
    size_t size;
    SYS(out, "ip netns add synproxy");
    SYS(out, "ip link add tmp0 type veth peer name tmp1");
    SYS(out, "ip link set tmp1 netns synproxy");
    SYS(out, "ip link set tmp0 up");
    SYS(out, "ip addr replace 198.18.0.1/24 dev tmp0");
// When checksum offload is enabled, the XDP program sees wrong
// checksums and drops packets.
//
    SYS(out, "ethtool -K tmp0 tx off");
    if (xdp)
// Workaround required for veth.
    SYS(out, "ip link set tmp0 xdp object xdp_dummy.bpf.o section xdp 2> /dev/null");
    ns = open_netns("synproxy");
    if (!ASSERT_OK_PTR(ns, "setns"))
    goto out;
    SYS(out, "ip link set lo up");
    SYS(out, "ip link set tmp1 up");
    SYS(out, "ip addr replace 198.18.0.2/24 dev tmp1");
    SYS(out, "sysctl -w net.ipv4.tcp_syncookies=2");
    SYS(out, "sysctl -w net.ipv4.tcp_timestamps=1");
    SYS(out, "sysctl -w net.netfilter.nf_conntrack_tcp_loose=0");
    SYS(out, "iptables-legacy -t raw -I PREROUTING \
    -i tmp1 -p tcp -m tcp --syn --dport 8080 -j CT --notrack");
    SYS(out, "iptables-legacy -t filter -A INPUT \
    -i tmp1 -p tcp -m tcp --dport 8080 -m state --state INVALID,UNTRACKED \
    -j SYNPROXY --sack-perm --timestamp --wscale 7 --mss 1460");
    SYS(out, "iptables-legacy -t filter -A INPUT \
    -i tmp1 -m state --state INVALID -j DROP");
    ctrl_file = SYS_OUT("./xdp_synproxy --iface tmp1 --ports 8080 \
    --single --mss4 1460 --mss6 1440 \
    --wscale 7 --ttl 64%s", xdp ? "" : " --tc");
    size = fread(buf, 1, sizeof(buf), ctrl_file);
    pclose(ctrl_file);
    if (!expect_str(buf, size, "Total SYNACKs generated: 0\n",
    "initial SYNACKs"))
    goto out;
    if (!xdp) {
    ctrl_file = SYS_OUT("tc filter show dev tmp1 ingress");
    size = fread(buf, 1, sizeof(buf), ctrl_file);
    pclose(ctrl_file);
    prog_id = memmem(buf, size, " id ", 4);
    if (!ASSERT_OK_PTR(prog_id, "find prog id"))
    goto out;
    prog_id += 4;
    if (!ASSERT_LT(prog_id, buf + size, "find prog id begin"))
    goto out;
    prog_id_end = prog_id;
    while (prog_id_end < buf + size && *prog_id_end >= '0' &&
// prog_id_end <= '9')
    prog_id_end++;
    if (!ASSERT_LT(prog_id_end, buf + size, "find prog id end"))
    goto out;
// prog_id_end = '\0';
    }
    server_fd = start_server(AF_INET, SOCK_STREAM, "198.18.0.2", 8080, 0);
    if (!ASSERT_GE(server_fd, 0, "start_server"))
    goto out;
    close_netns(ns);
    ns = core::ptr::null_mut();
    client_fd = connect_to_fd(server_fd, 10000);
    if (!ASSERT_GE(client_fd, 0, "connect_to_fd"))
    goto out;
    accept_fd = accept(server_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_GE(accept_fd, 0, "accept"))
    goto out;
    ns = open_netns("synproxy");
    if (!ASSERT_OK_PTR(ns, "setns"))
    goto out;
    if (xdp)
    ctrl_file = SYS_OUT("./xdp_synproxy --iface tmp1 --single");
    else
    ctrl_file = SYS_OUT("./xdp_synproxy --prog %s --single",
    prog_id);
    size = fread(buf, 1, sizeof(buf), ctrl_file);
    pclose(ctrl_file);
    if (!expect_str(buf, size, "Total SYNACKs generated: 1\n",
    "SYNACKs after connection"))
    goto out;
    out:
    if (accept_fd >= 0)
    close(accept_fd);
    if (client_fd >= 0)
    close(client_fd);
    if (server_fd >= 0)
    close(server_fd);
    if (ns)
    close_netns(ns);
    SYS_NOFAIL("ip link del tmp0");
    SYS_NOFAIL("ip netns del synproxy");
    }
#[no_mangle]
pub unsafe extern "C" fn test_xdp_synproxy() {
    void test_xdp_synproxy(void)
    {
    if (test__start_subtest("xdp"))
    test_synproxy(true);
    if (test__start_subtest("tc"))
    test_synproxy(false);
    }
