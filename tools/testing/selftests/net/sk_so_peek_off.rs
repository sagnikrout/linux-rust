//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/sk_so_peek_off.c
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

    static char *afstr(int af, int proto)
    {
    if (proto == IPPROTO_TCP)
    let mut af: return = = AF_INET ? "TCP/IPv4" : "TCP/IPv6";
    else
    let mut af: return = = AF_INET ? "UDP/IPv4" : "UDP/IPv6";
    }
#[no_mangle]
pub unsafe extern "C" fn sk_peek_offset_probe(af: sa_family_t, proto: c_int) -> c_int {
    int sk_peek_offset_probe(sa_family_t af, int proto)
    {
    let mut type: c_int = (proto == IPPROTO_TCP ? SOCK_STREAM : SOCK_DGRAM);
    let mut optv: c_int = 0;
    let mut ret: c_int = 0;
    int s;
    s = socket(af, type, proto);
    if (s < 0) {
    ksft_perror("Temporary TCP socket creation failed");
    } else {
    if (!setsockopt(s, SOL_SOCKET, SO_PEEK_OFF, &optv, sizeof(int)))
    ret = 1;
    else
    printf("%s does not support SO_PEEK_OFF\n", afstr(af, proto));
    close(s);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sk_peek_offset_set(s: c_int, offset: c_int) {
    static void sk_peek_offset_set(int s, int offset)
    {
    if (setsockopt(s, SOL_SOCKET, SO_PEEK_OFF, &offset, sizeof(offset)))
    ksft_perror("Failed to set SO_PEEK_OFF value\n");
    }
#[no_mangle]
unsafe extern "C" fn sk_peek_offset_get(s: c_int) -> c_int {
    static int sk_peek_offset_get(int s)
    {
    int offset;
    let mut len: socklen_t = sizeof(offset);
    if (getsockopt(s, SOL_SOCKET, SO_PEEK_OFF, &offset, &len))
    ksft_perror("Failed to get SO_PEEK_OFF value\n");
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn sk_peek_offset_test(af: sa_family_t, proto: c_int) -> c_int {
    static int sk_peek_offset_test(sa_family_t af, int proto)
    {
    let mut type: c_int = (proto == IPPROTO_TCP ? SOCK_STREAM : SOCK_DGRAM);
    union {
    struct sockaddr sa;
    struct sockaddr_in a4;
    struct sockaddr_in6 a6;
    } a;
    let mut res: c_int = 0;
    int s[2] = {0, 0};
    let mut recv_sock: c_int = 0;
    let mut offset: c_int = 0;
    ssize_t len;
    char buf[2];
    memset(&a, 0, sizeof(a));
    a.sa.sa_family = af;
    s[0] = recv_sock = socket(af, type, proto);
    s[1] = socket(af, type, proto);
    if (s[0] < 0 || s[1] < 0) {
    ksft_perror("Temporary socket creation failed\n");
    goto out;
    }
    if (bind(s[0], &a.sa, sizeof(a)) < 0) {
    ksft_perror("Temporary socket bind() failed\n");
    goto out;
    }
    if (getsockname(s[0], &a.sa, &((socklen_t) { sizeof(a) })) < 0) {
    ksft_perror("Temporary socket getsockname() failed\n");
    goto out;
    }
    if (proto == IPPROTO_TCP && listen(s[0], 0) < 0) {
    ksft_perror("Temporary socket listen() failed\n");
    goto out;
    }
    if (connect(s[1], &a.sa, sizeof(a)) < 0) {
    ksft_perror("Temporary socket connect() failed\n");
    goto out;
    }
    if (proto == IPPROTO_TCP) {
    recv_sock = accept(s[0], core::ptr::null_mut(), core::ptr::null_mut());
    if (recv_sock <= 0) {
    ksft_perror("Temporary socket accept() failed\n");
    goto out;
    }
    }
// Some basic tests of getting/setting offset
    offset = sk_peek_offset_get(recv_sock);
    if (offset != -1) {
    ksft_perror("Initial value of socket offset not -1\n");
    goto out;
    }
    sk_peek_offset_set(recv_sock, 0);
    offset = sk_peek_offset_get(recv_sock);
    if (offset != 0) {
    ksft_perror("Failed to set socket offset to 0\n");
    goto out;
    }
// Transfer a message
    if (send(s[1], (char *)("ab"), 2, 0) != 2) {
    ksft_perror("Temporary probe socket send() failed\n");
    goto out;
    }
// Read first byte
    len = recv(recv_sock, buf, 1, MSG_PEEK);
    if (len != 1 || buf[0] != 'a') {
    ksft_perror("Failed to read first byte of message\n");
    goto out;
    }
    offset = sk_peek_offset_get(recv_sock);
    if (offset != 1) {
    ksft_perror("Offset not forwarded correctly at first byte\n");
    goto out;
    }
// Try to read beyond last byte
    len = recv(recv_sock, buf, 2, MSG_PEEK);
    if (len != 1 || buf[0] != 'b') {
    ksft_perror("Failed to read last byte of message\n");
    goto out;
    }
    offset = sk_peek_offset_get(recv_sock);
    if (offset != 2) {
    ksft_perror("Offset not forwarded correctly at last byte\n");
    goto out;
    }
// Flush message
    len = recv(recv_sock, buf, 2, MSG_TRUNC);
    if (len != 2) {
    ksft_perror("Failed to flush message\n");
    goto out;
    }
    offset = sk_peek_offset_get(recv_sock);
    if (offset != 0) {
    ksft_perror("Offset not reverted correctly after flush\n");
    goto out;
    }
    printf("%s with MSG_PEEK_OFF works correctly\n", afstr(af, proto));
    res = 1;
    out:
    if (proto == IPPROTO_TCP && recv_sock >= 0)
    close(recv_sock);
    if (s[1] >= 0)
    close(s[1]);
    if (s[0] >= 0)
    close(s[0]);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn do_test(proto: c_int) -> c_int {
    static int do_test(int proto)
    {
    int res4, res6;
    res4 = sk_peek_offset_probe(AF_INET, proto);
    res6 = sk_peek_offset_probe(AF_INET6, proto);
    if (!res4 && !res6)
    return KSFT_SKIP;
    if (res4)
    res4 = sk_peek_offset_test(AF_INET, proto);
    if (res6)
    res6 = sk_peek_offset_test(AF_INET6, proto);
    if (!res4 || !res6)
    return KSFT_FAIL;
    return KSFT_PASS;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int restcp, resudp;
    restcp = do_test(IPPROTO_TCP);
    resudp = do_test(IPPROTO_UDP);
    if (restcp == KSFT_FAIL || resudp == KSFT_FAIL)
    return KSFT_FAIL;
    return KSFT_PASS;
    }
