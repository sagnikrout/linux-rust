//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sockmap_ktls.c
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
// Copyright (c) 2020 Cloudflare
//
// Tests for sockmap/sockhash holding kTLS sockets.
//

pub const MAX_TEST_NAME: c_int = 80;
pub const TCP_ULP: c_int = 31;
#[no_mangle]
unsafe extern "C" fn init_ktls_pairs(c: c_int, p: c_int) -> c_int {
    static int init_ktls_pairs(int c, int p)
    {
    int err;
    struct tls12_crypto_info_aes_gcm_128 crypto_rx;
    struct tls12_crypto_info_aes_gcm_128 crypto_tx;
    err = setsockopt(c, IPPROTO_TCP, TCP_ULP, "tls", strlen("tls"));
    if (!ASSERT_OK(err, "setsockopt(TCP_ULP)"))
    goto out;
    err = setsockopt(p, IPPROTO_TCP, TCP_ULP, "tls", strlen("tls"));
    if (!ASSERT_OK(err, "setsockopt(TCP_ULP)"))
    goto out;
    memset(&crypto_rx, 0, sizeof(crypto_rx));
    memset(&crypto_tx, 0, sizeof(crypto_tx));
    crypto_rx.info.version = TLS_1_2_VERSION;
    crypto_tx.info.version = TLS_1_2_VERSION;
    crypto_rx.info.cipher_type = TLS_CIPHER_AES_GCM_128;
    crypto_tx.info.cipher_type = TLS_CIPHER_AES_GCM_128;
    err = setsockopt(c, SOL_TLS, TLS_TX, &crypto_tx, sizeof(crypto_tx));
    if (!ASSERT_OK(err, "setsockopt(TLS_TX)"))
    goto out;
    err = setsockopt(p, SOL_TLS, TLS_RX, &crypto_rx, sizeof(crypto_rx));
    if (!ASSERT_OK(err, "setsockopt(TLS_RX)"))
    goto out;
    return 0;
    out:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn create_ktls_pairs(family: c_int, sotype: c_int, c: *mut c_int, p: *mut c_int) -> c_int {
    static int create_ktls_pairs(int family, int sotype, int *c, int *p)
    {
    int err;
    err = create_pair(family, sotype, c, p);
    if (!ASSERT_OK(err, "create_pair()"))
    return -1;
    err = init_ktls_pairs(*c, *p);
    if (!ASSERT_OK(err, "init_ktls_pairs(c, p)"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_ktls_update_fails_when_sock_has_ulp(family: c_int, map: c_int) {
    static void test_sockmap_ktls_update_fails_when_sock_has_ulp(int family, int map)
    {
    let mut addr: sockaddr_storage = {};
    let mut len: socklen_t = sizeof(addr);
    struct sockaddr_in6 *v6;
    struct sockaddr_in *v4;
    int err, s, zero = 0;
    switch (family) {
    case AF_INET:
    v4 = (struct sockaddr_in *)&addr;
    v4.sin_family = AF_INET;
    break;
    case AF_INET6:
    v6 = (struct sockaddr_in6 *)&addr;
    v6.sin6_family = AF_INET6;
    break;
    default:
    PRINT_FAIL("unsupported socket family %d", family);
    return;
    }
    s = socket(family, SOCK_STREAM, 0);
    if (!ASSERT_GE(s, 0, "socket"))
    return;
    err = bind(s, (struct sockaddr *)&addr, len);
    if (!ASSERT_OK(err, "bind"))
    goto close;
    err = getsockname(s, (struct sockaddr *)&addr, &len);
    if (!ASSERT_OK(err, "getsockname"))
    goto close;
    err = connect(s, (struct sockaddr *)&addr, len);
    if (!ASSERT_OK(err, "connect"))
    goto close;
// save sk->sk_prot and set it to tls_prots
    err = setsockopt(s, IPPROTO_TCP, TCP_ULP, "tls", strlen("tls"));
    if (!ASSERT_OK(err, "setsockopt(TCP_ULP)"))
    goto close;
// sockmap update should not affect saved sk_prot
    err = bpf_map_update_elem(map, &zero, &s, BPF_ANY);
    if (!ASSERT_ERR(err, "sockmap update elem"))
    goto close;
// call sk->sk_prot->setsockopt to dispatch to saved sk_prot
    err = setsockopt(s, IPPROTO_TCP, TCP_NODELAY, &zero, sizeof(zero));
    ASSERT_OK(err, "setsockopt(TCP_NODELAY)");
    close:
    close(s);
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_ktls_enable_fails_when_in_sockmap(family: c_int, map: c_int) {
    static void test_sockmap_ktls_enable_fails_when_in_sockmap(int family, int map)
    {
    struct tls12_crypto_info_aes_gcm_128 crypto = {
    .info = {
    .version     = TLS_1_2_VERSION,
    .cipher_type = TLS_CIPHER_AES_GCM_128,
    },
    };
    let mut addr: sockaddr_storage = {};
    let mut len: socklen_t = sizeof(addr);
    struct sockaddr_in6 *v6;
    struct sockaddr_in *v4;
    int err, s, zero = 0;
    switch (family) {
    case AF_INET:
    v4 = (struct sockaddr_in *)&addr;
    v4.sin_family = AF_INET;
    break;
    case AF_INET6:
    v6 = (struct sockaddr_in6 *)&addr;
    v6.sin6_family = AF_INET6;
    break;
    default:
    PRINT_FAIL("unsupported socket family %d", family);
    return;
    }
    s = socket(family, SOCK_STREAM, 0);
    if (!ASSERT_GE(s, 0, "socket"))
    return;
    err = bind(s, (struct sockaddr *)&addr, len);
    if (!ASSERT_OK(err, "bind"))
    goto close;
    err = getsockname(s, (struct sockaddr *)&addr, &len);
    if (!ASSERT_OK(err, "getsockname"))
    goto close;
    err = connect(s, (struct sockaddr *)&addr, len);
    if (!ASSERT_OK(err, "connect"))
    goto close;
// Add the socket to the sockmap, attaching a psock.
    err = bpf_map_update_elem(map, &zero, &s, BPF_ANY);
    if (!ASSERT_OK(err, "sockmap update elem"))
    goto close;
// Installing the TLS ULP is allowed, it does not touch the datapath.
    err = setsockopt(s, IPPROTO_TCP, TCP_ULP, "tls", strlen("tls"));
    if (!ASSERT_OK(err, "setsockopt(TCP_ULP)"))
    goto close;
// Enabling the TLS crypto datapath must be rejected.
    err = setsockopt(s, SOL_TLS, TLS_TX, &crypto, sizeof(crypto));
    ASSERT_ERR(err, "setsockopt(TLS_TX)");
    close:
    close(s);
    }
    static const char *fmt_test_name(const char *subtest_name, int family,
    enum bpf_map_type map_type)
    {
    const char *map_type_str = BPF_MAP_TYPE_SOCKMAP ? "SOCKMAP" : "SOCKHASH";
    const char *family_str = AF_INET ? "IPv4" : "IPv6";
    static char test_name[MAX_TEST_NAME];
    snprintf(test_name, MAX_TEST_NAME,
    "sockmap_ktls %s %s %s",
    subtest_name, family_str, map_type_str);
    return test_name;
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_ktls_offload(family: c_int, sotype: c_int) {
    static void test_sockmap_ktls_offload(int family, int sotype)
    {
    int err;
    let mut c: c_int = 0, p = 0, sent, recvd;
    char msg[12] = "hello world\0";
    char rcv[13];
    err = create_ktls_pairs(family, sotype, &c, &p);
    if (!ASSERT_OK(err, "create_ktls_pairs()"))
    goto out;
    sent = send(c, msg, sizeof(msg), 0);
    if (!ASSERT_OK(err, "send(msg)"))
    goto out;
    recvd = recv(p, rcv, sizeof(rcv), 0);
    if (!ASSERT_OK(err, "recv(msg)") ||
    !ASSERT_EQ(recvd, sent, "length mismatch"))
    goto out;
    ASSERT_OK(memcmp(msg, rcv, sizeof(msg)), "data mismatch");
    out:
    if (c)
    close(c);
    if (p)
    close(p);
    }
#[no_mangle]
unsafe extern "C" fn run_tests(family: c_int, map_type: enum bpf_map_type) {
    static void run_tests(int family, enum bpf_map_type map_type)
    {
    int map;
    map = bpf_map_create(map_type, core::ptr::null_mut(), sizeof(int), sizeof(int), 1, core::ptr::null_mut());
    if (!ASSERT_GE(map, 0, "bpf_map_create"))
    return;
    if (test__start_subtest(fmt_test_name("update_fails_when_sock_has_ulp", family, map_type)))
    test_sockmap_ktls_update_fails_when_sock_has_ulp(family, map);
    if (test__start_subtest(fmt_test_name("enable_fails_when_in_sockmap", family, map_type)))
    test_sockmap_ktls_enable_fails_when_in_sockmap(family, map);
    close(map);
    }
#[no_mangle]
unsafe extern "C" fn run_ktls_test(family: c_int, sotype: c_int) {
    static void run_ktls_test(int family, int sotype)
    {
    if (test__start_subtest("tls simple offload"))
    test_sockmap_ktls_offload(family, sotype);
    }
#[no_mangle]
pub unsafe extern "C" fn test_sockmap_ktls() {
    void test_sockmap_ktls(void)
    {
    run_tests(AF_INET, BPF_MAP_TYPE_SOCKMAP);
    run_tests(AF_INET, BPF_MAP_TYPE_SOCKHASH);
    run_tests(AF_INET6, BPF_MAP_TYPE_SOCKMAP);
    run_tests(AF_INET6, BPF_MAP_TYPE_SOCKHASH);
    run_ktls_test(AF_INET, SOCK_STREAM);
    run_ktls_test(AF_INET6, SOCK_STREAM);
    }
