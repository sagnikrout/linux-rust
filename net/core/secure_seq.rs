//! Automatically rewritten from C to Rust
//! Source: net/core/secure_seq.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2016 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

    static siphash_aligned_key_t net_secret;

#[no_mangle]
unsafe extern "C" fn net_secret_init() -> __always_inline void {
    static __always_inline void net_secret_init(void)
    {
    net_get_random_once(&net_secret, sizeof(net_secret));
    }

#[no_mangle]
unsafe extern "C" fn seq_scale(seq: u32) -> u32 {
    static u32 seq_scale(u32 seq)
    {
//
// As close as possible to RFC 793, which
// suggests using a 250 kHz clock.
// Further reading shows this assumes 2 Mb/s networks.
// For 10 Mb/s Ethernet, a 1 MHz clock is appropriate.
// For 10 Gb/s Ethernet, a 1 GHz clock should be ok, but
// we also need to limit the resolution so that the u32 seq
// overlaps less than one time per MSL (2 minutes).
// Choosing a clock of 64 ns period is OK. (period of 274 s)
//
    return seq + (ktime_get_real_ns() >> 6);
    }

    union tcp_seq_and_ts_off
    secure_tcpv6_seq_and_ts_off(const struct net *net, const __be32 *saddr,
    const __be32 *daddr, __be16 sport, __be16 dport)
    {
    const struct {
    struct in6_addr saddr;
    struct in6_addr daddr;
    __be16 sport;
    __be16 dport;
    } __aligned(SIPHASH_ALIGNMENT) combined = {
    .saddr = *(struct in6_addr *)saddr,
    .daddr = *(struct in6_addr *)daddr,
    .sport = sport,
    .dport = dport
    };
    union tcp_seq_and_ts_off st;
    net_secret_init();
    st.hash64 = siphash(&combined, offsetofend(typeof(combined), dport),
    &net_secret);
    if (READ_ONCE(net.ipv4.sysctl_tcp_timestamps) != 1)
    st.ts_off = 0;
    st.seq = seq_scale(st.seq);
    return st;
    }
    EXPORT_SYMBOL(secure_tcpv6_seq_and_ts_off);
    u64 secure_ipv6_port_ephemeral(const __be32 *saddr, const __be32 *daddr,
    __be16 dport)
    {
    const struct {
    struct in6_addr saddr;
    struct in6_addr daddr;
    unsigned int timeseed;
    __be16 dport;
    } __aligned(SIPHASH_ALIGNMENT) combined = {
    .saddr = *(struct in6_addr *)saddr,
    .daddr = *(struct in6_addr *)daddr,
    .timeseed = jiffies / EPHEMERAL_PORT_SHUFFLE_PERIOD,
    .dport = dport,
    };
    net_secret_init();
    return siphash(&combined, offsetofend(typeof(combined), dport),
    &net_secret);
    }
    EXPORT_SYMBOL(secure_ipv6_port_ephemeral);

// secure_tcp_seq_and_tsoff(a, b, 0, d) == secure_ipv4_port_ephemeral(a, b, d),
// but fortunately, `sport' cannot be 0 in any circumstances. If this changes,
// it would be easy enough to have the former function use siphash_4u32, passing
// the arguments as separate u32.
//
    union tcp_seq_and_ts_off
    secure_tcp_seq_and_ts_off(const struct net *net, __be32 saddr, __be32 daddr,
    __be16 sport, __be16 dport)
    {
    let mut ports: u32 = ( u32)sport << 16 | ( u32)dport;
    union tcp_seq_and_ts_off st;
    net_secret_init();
    st.hash64 = siphash_3u32(( u32)saddr, ( u32)daddr,
    ports, &net_secret);
    if (READ_ONCE(net.ipv4.sysctl_tcp_timestamps) != 1)
    st.ts_off = 0;
    st.seq = seq_scale(st.seq);
    return st;
    }
    EXPORT_SYMBOL_GPL(secure_tcp_seq_and_ts_off);
#[no_mangle]
pub unsafe extern "C" fn secure_ipv4_port_ephemeral(saddr: __be32, daddr: __be32, dport: __be16) -> u64 {
    u64 secure_ipv4_port_ephemeral(__be32 saddr, __be32 daddr, __be16 dport)
    {
    net_secret_init();
    return siphash_4u32(( u32)saddr, ( u32)daddr,
    ( u16)dport,
    jiffies / EPHEMERAL_PORT_SHUFFLE_PERIOD,
    &net_secret);
    }
    EXPORT_SYMBOL_GPL(secure_ipv4_port_ephemeral);
