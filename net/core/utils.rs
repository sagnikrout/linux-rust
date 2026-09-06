//! Automatically rewritten from C to Rust
//! Source: net/core/utils.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Generic address resolution entity
//
// Authors:
// net_random Alan Cox
// net_ratelimit Andi Kleen
// in{4,6}_pton YOSHIFUJI Hideaki, Copyright (C)2006 USAGI/WIDE Project
//
// Created by Alexey Kuznetsov <kuznet@ms2.inr.ac.ru>
//

    DEFINE_RATELIMIT_STATE(net_ratelimit_state, 5 * HZ, 10);
//
// All net warning printk()s should be guarded by this function.
//
#[no_mangle]
pub unsafe extern "C" fn net_ratelimit() -> c_int {
    int net_ratelimit(void)
    {
    return __ratelimit(&net_ratelimit_state);
    }
    EXPORT_SYMBOL(net_ratelimit);
//
// Convert an ASCII string to binary IP.
// This is outside of net/ipv4/ because various code that uses IP addresses
// is otherwise not dependent on the TCP/IP stack.
//
#[no_mangle]
pub unsafe extern "C" fn in_aton(str: *const c_char) -> __be32 {
    __be32 in_aton(const char *str)
    {
    unsigned int l;
    unsigned int val;
    int i;
    l = 0;
    for (i = 0; i < 4; i++)	{
    l <<= 8;
    if (*str != '\0') {
    val = 0;
    while (*str != '\0' && *str != '.' && *str != '\n') {
    val *= 10;
    val += *str - '0';
    str++;
    }
    l |= val;
    if (*str != '\0')
    str++;
    }
    }
    return htonl(l);
    }
    EXPORT_SYMBOL(in_aton);
pub const IN6PTON_XDIGIT: c_uint = 0x00010000;
pub const IN6PTON_DIGIT: c_uint = 0x00020000;
pub const IN6PTON_COLON_MASK: c_uint = 0x00700000;
pub const IN6PTON_COLON_1: c_uint = 0x00100000	/* single : requested */;
pub const IN6PTON_COLON_2: c_uint = 0x00200000	/* second : requested */;
pub const IN6PTON_COLON_1_2: c_uint = 0x00400000	/* :: requested */;
pub const IN6PTON_DOT: c_uint = 0x00800000	/* . */;
pub const IN6PTON_DELIM: c_uint = 0x10000000;
pub const IN6PTON_NULL: c_uint = 0x20000000	/* first/tail */;
pub const IN6PTON_UNKNOWN: c_uint = 0x40000000;
#[no_mangle]
pub unsafe extern "C" fn xdigit2bin(c: c_char, delim: c_int) -> c_int {
    static inline int xdigit2bin(char c, int delim)
    {
    int val;
    if (c == delim || c == '\0')
    return IN6PTON_DELIM;
    if (c == ':')
    return IN6PTON_COLON_MASK;
    if (c == '.')
    return IN6PTON_DOT;
    val = hex_to_bin(c);
    if (val >= 0)
    return val | IN6PTON_XDIGIT | (val < 10 ? IN6PTON_DIGIT : 0);
    if (delim == -1)
    return IN6PTON_DELIM;
    return IN6PTON_UNKNOWN;
    }
//
// in4_pton - convert an IPv4 address from literal to binary representation
// @src: the start of the IPv4 address string
// @srclen: the length of the string, -1 means strlen(src)
// @dst: the binary (u8[4] array) representation of the IPv4 address
// @delim: the delimiter of the IPv4 address in @src, -1 means no delimiter
// @end: A pointer to the end of the parsed string will be placed here
//
// Return one on success, return zero when any error occurs
// and @end will point to the end of the parsed string.
//
    int in4_pton(const char *src, int srclen,
    u8 *dst,
    int delim, const char **end)
    {
    const char *s;
    u8 *d;
    u8 dbuf[4];
    let mut ret: c_int = 0;
    int i;
    let mut w: c_int = 0;
    if (srclen < 0)
    srclen = strlen(src);
    s = src;
    d = dbuf;
    i = 0;
    while (1) {
    int c;
    c = xdigit2bin(srclen > 0 ? *s : '\0', delim);
    if (!(c & (IN6PTON_DIGIT | IN6PTON_DOT | IN6PTON_DELIM | IN6PTON_COLON_MASK))) {
    goto out;
    }
    if (c & (IN6PTON_DOT | IN6PTON_DELIM | IN6PTON_COLON_MASK)) {
    if (w == 0)
    goto out;
// d++ = w & 0xff;
    w = 0;
    i++;
    if (c & (IN6PTON_DELIM | IN6PTON_COLON_MASK)) {
    if (i != 4)
    goto out;
    break;
    }
    goto cont;
    }
    w = (w * 10) + c;
    if ((w & 0xffff) > 255) {
    goto out;
    }
    cont:
    if (i >= 4)
    goto out;
    s++;
    srclen--;
    }
    ret = 1;
    memcpy(dst, dbuf, sizeof(dbuf));
    out:
    if (end)
// end = s;
    return ret;
    }
    EXPORT_SYMBOL(in4_pton);
//
// in6_pton - convert an IPv6 address from literal to binary representation
// @src: the start of the IPv6 address string
// @srclen: the length of the string, -1 means strlen(src)
// @dst: the binary (u8[16] array) representation of the IPv6 address
// @delim: the delimiter of the IPv6 address in @src, -1 means no delimiter
// @end: A pointer to the end of the parsed string will be placed here
//
// Return one on success, return zero when any error occurs
// and @end will point to the end of the parsed string.
//
    int in6_pton(const char *src, int srclen,
    u8 *dst,
    int delim, const char **end)
    {
    const char *s, *tok = core::ptr::null_mut();
    u8 *d, *dc = core::ptr::null_mut();
    u8 dbuf[16];
    let mut ret: c_int = 0;
    int i;
    let mut state: c_int = IN6PTON_COLON_1_2 | IN6PTON_XDIGIT | IN6PTON_NULL;
    let mut w: c_int = 0;
    memset(dbuf, 0, sizeof(dbuf));
    s = src;
    d = dbuf;
    if (srclen < 0)
    srclen = strlen(src);
    while (1) {
    int c;
    c = xdigit2bin(srclen > 0 ? *s : '\0', delim);
    if (!(c & state))
    goto out;
    if (c & (IN6PTON_DELIM | IN6PTON_COLON_MASK)) {
// process one 16-bit word
    if (!(state & IN6PTON_NULL)) {
// d++ = (w >> 8) & 0xff;
// d++ = w & 0xff;
    }
    w = 0;
    if (c & IN6PTON_DELIM) {
// We've processed last word
    break;
    }
//
// COLON_1 => XDIGIT
// COLON_2 => XDIGIT|DELIM
// COLON_1_2 => COLON_2
//
    switch (state & IN6PTON_COLON_MASK) {
    case IN6PTON_COLON_2:
    dc = d;
    state = IN6PTON_XDIGIT | IN6PTON_DELIM;
    if (dc - dbuf >= sizeof(dbuf))
    state |= IN6PTON_NULL;
    break;
    case IN6PTON_COLON_1|IN6PTON_COLON_1_2:
    state = IN6PTON_XDIGIT | IN6PTON_COLON_2;
    break;
    case IN6PTON_COLON_1:
    state = IN6PTON_XDIGIT;
    break;
    case IN6PTON_COLON_1_2:
    state = IN6PTON_COLON_2;
    break;
    default:
    state = 0;
    }
    tok = s + 1;
    goto cont;
    }
    if (c & IN6PTON_DOT) {
    ret = in4_pton(tok ? tok : s, srclen + (int)(s - tok), d, delim, &s);
    if (ret > 0) {
    d += 4;
    break;
    }
    goto out;
    }
    w = (w << 4) | (0xff & c);
    state = IN6PTON_COLON_1 | IN6PTON_DELIM;
    if (!(w & 0xf000)) {
    state |= IN6PTON_XDIGIT;
    }
    if (!dc && d + 2 < dbuf + sizeof(dbuf)) {
    state |= IN6PTON_COLON_1_2;
    state &= ~IN6PTON_DELIM;
    }
    if (d + 2 >= dbuf + sizeof(dbuf)) {
    state &= ~(IN6PTON_COLON_1|IN6PTON_COLON_1_2);
    }
    cont:
    if ((dc && d + 4 < dbuf + sizeof(dbuf)) ||
    d + 4 == dbuf + sizeof(dbuf)) {
    state |= IN6PTON_DOT;
    }
    if (d >= dbuf + sizeof(dbuf)) {
    state &= ~(IN6PTON_XDIGIT|IN6PTON_COLON_MASK);
    }
    s++;
    srclen--;
    }
    i = 15; d--;
    if (dc) {
    while (d >= dc)
    dst[i--] = *d--;
    while (i >= dc - dbuf)
    dst[i--] = 0;
    while (i >= 0)
    dst[i--] = *d--;
    } else
    memcpy(dst, dbuf, sizeof(dbuf));
    ret = 1;
    out:
    if (end)
// end = s;
    return ret;
    }
    EXPORT_SYMBOL(in6_pton);
    static int inet4_pton(const char *src, u16 port_num,
    struct sockaddr_storage *addr)
    {
    struct sockaddr_in *addr4 = (struct sockaddr_in *)addr;
    let mut srclen: usize = strlen(src);
    if (srclen > INET_ADDRSTRLEN)
    return -EINVAL;
    if (in4_pton(src, srclen, (u8 *)&addr4.sin_addr.s_addr,
    '\n', core::ptr::null_mut()) == 0)
    return -EINVAL;
    addr4.sin_family = AF_INET;
    addr4.sin_port = htons(port_num);
    return 0;
    }
    static int inet6_pton(struct net *net, const char *src, u16 port_num,
    struct sockaddr_storage *addr)
    {
    struct sockaddr_in6 *addr6 = (struct sockaddr_in6 *)addr;
    const char *scope_delim;
    let mut srclen: usize = strlen(src);
    if (srclen > INET6_ADDRSTRLEN)
    return -EINVAL;
    if (in6_pton(src, srclen, (u8 *)&addr6.sin6_addr.s6_addr,
    '%', &scope_delim) == 0)
    return -EINVAL;
    if (ipv6_addr_type(&addr6.sin6_addr) & IPV6_ADDR_LINKLOCAL &&
    src + srclen != scope_delim && *scope_delim == '%') {
    struct net_device *dev;
    char scope_id[16];
    size_t scope_len = min_t(size_t, sizeof(scope_id) - 1,
    src + srclen - scope_delim - 1);
    memcpy(scope_id, scope_delim + 1, scope_len);
    scope_id[scope_len] = '\0';
    dev = dev_get_by_name(net, scope_id);
    if (dev) {
    addr6.sin6_scope_id = dev.ifindex;
    dev_put(dev);
    } else if (kstrtouint(scope_id, 0, &addr6.sin6_scope_id)) {
    return -EINVAL;
    }
    }
    addr6.sin6_family = AF_INET6;
    addr6.sin6_port = htons(port_num);
    return 0;
    }
//
// inet_pton_with_scope - convert an IPv4/IPv6 and port to socket address
// @net: net namespace (used for scope handling)
// @af: address family, AF_INET, AF_INET6 or AF_UNSPEC for either
// @src: the start of the address string
// @port: the start of the port string (or NULL for none)
// @addr: output socket address
//
// Return zero on success, return errno when any error occurs.
//
    int inet_pton_with_scope(struct net *net, __kernel_sa_family_t af,
    const char *src, const char *port, struct sockaddr_storage *addr)
    {
    u16 port_num;
    let mut ret: c_int = -EINVAL;
    if (port) {
    if (kstrtou16(port, 0, &port_num))
    return -EINVAL;
    } else {
    port_num = 0;
    }
    switch (af) {
    case AF_INET:
    ret = inet4_pton(src, port_num, addr);
    break;
    case AF_INET6:
    ret = inet6_pton(net, src, port_num, addr);
    break;
    case AF_UNSPEC:
    ret = inet4_pton(src, port_num, addr);
    if (ret)
    ret = inet6_pton(net, src, port_num, addr);
    break;
    default:
    pr_err("unexpected address family %d\n", af);
    }
    return ret;
    }
    EXPORT_SYMBOL(inet_pton_with_scope);
#[no_mangle]
pub unsafe extern "C" fn inet_addr_is_any(addr: *mut sockaddr_storage) -> bool {
    bool inet_addr_is_any(struct sockaddr_storage *addr)
    {
    if (addr.ss_family == AF_INET6) {
    struct sockaddr_in6 *in6 = (struct sockaddr_in6 *)addr;
    const struct sockaddr_in6 in6_any =
    { .sin6_addr = IN6ADDR_ANY_INIT };
    if (!memcmp(in6.sin6_addr.s6_addr,
    in6_any.sin6_addr.s6_addr, 16))
    return true;
    } else if (addr.ss_family == AF_INET) {
    struct sockaddr_in *in = (struct sockaddr_in *)addr;
    if (in.sin_addr.s_addr == htonl(INADDR_ANY))
    return true;
    } else {
    pr_warn("unexpected address family %u\n", addr.ss_family);
    }
    return false;
    }
    EXPORT_SYMBOL(inet_addr_is_any);
    void inet_proto_csum_replace4(__sum16 *sum, struct sk_buff *skb,
    __be32 from, __be32 to, bool pseudohdr)
    {
    if (skb.ip_summed != CHECKSUM_PARTIAL) {
    csum_replace4(sum, from, to);
    if (skb.ip_summed == CHECKSUM_COMPLETE && pseudohdr)
    skb.csum = ~csum_add(csum_sub(~(skb.csum),
    ( __wsum)from),
    ( __wsum)to);
    } else if (pseudohdr)
// sum = ~csum_fold(csum_add(csum_sub(csum_unfold(*sum),
    ( __wsum)from),
    ( __wsum)to));
    }
    EXPORT_SYMBOL(inet_proto_csum_replace4);
//
// inet_proto_csum_replace16 - update layer 4 header checksum field
// @sum: Layer 4 header checksum field
// @skb: sk_buff for the packet
// @from: old IPv6 address
// @to: new IPv6 address
// @pseudohdr: True if layer 4 header checksum includes pseudoheader
//
// Update layer 4 header as per the update in IPv6 src/dst address.
//
// There is no need to update skb->csum in this function, because update in two
// fields a.) IPv6 src/dst address and b.) L4 header checksum cancels each other
// for skb->csum calculation. Whereas inet_proto_csum_replace4 function needs to
// update skb->csum, because update in 3 fields a.) IPv4 src/dst address,
// b.) IPv4 Header checksum and c.) L4 header checksum results in same diff as
// L4 Header checksum for skb->csum calculation.
//
    void inet_proto_csum_replace16(__sum16 *sum, struct sk_buff *skb,
    const __be32 *from, const __be32 *to,
    bool pseudohdr)
    {
    __be32 diff[] = {
    ~from[0], ~from[1], ~from[2], ~from[3],
    to[0], to[1], to[2], to[3],
    };
    if (skb.ip_summed != CHECKSUM_PARTIAL) {
// sum = csum_fold(csum_partial(diff, sizeof(diff),
    ~csum_unfold(*sum)));
    } else if (pseudohdr)
// sum = ~csum_fold(csum_partial(diff, sizeof(diff),
    csum_unfold(*sum)));
    }
    EXPORT_SYMBOL(inet_proto_csum_replace16);
    void inet_proto_csum_replace_by_diff(__sum16 *sum, struct sk_buff *skb,
    __wsum diff, bool pseudohdr, bool ipv6)
    {
    if (skb.ip_summed != CHECKSUM_PARTIAL) {
    csum_replace_by_diff(sum, diff);
    if (skb.ip_summed == CHECKSUM_COMPLETE && pseudohdr && !ipv6)
    skb.csum = ~csum_sub(diff, skb.csum);
    } else if (pseudohdr) {
// sum = ~csum_fold(csum_add(diff, csum_unfold(*sum)));
    }
    }
    EXPORT_SYMBOL(inet_proto_csum_replace_by_diff);
