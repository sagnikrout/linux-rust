//! Automatically rewritten from C to Rust
//! Source: net/ipv6/rpl.c
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
// Authors:
// (C) 2020 Alexander Aring <alex.aring@gmail.com>
//

pub const IPV6_RPL_BEST_ADDR_COMPRESSION: c_int = 15;
    static void ipv6_rpl_addr_decompress(struct in6_addr *dst,
    const struct in6_addr *daddr,
    const void *post, unsigned char pfx)
    {
    memcpy(dst, daddr, pfx);
    memcpy(&dst.s6_addr[pfx], post, IPV6_PFXTAIL_LEN(pfx));
    }
    static void ipv6_rpl_addr_compress(void *dst, const struct in6_addr *addr,
    unsigned char pfx)
    {
    memcpy(dst, &addr.s6_addr[pfx], IPV6_PFXTAIL_LEN(pfx));
    }
    static void *ipv6_rpl_segdata_pos(const struct ipv6_rpl_sr_hdr *hdr, int i)
    {
    return (void *)&hdr.rpl_segdata[i * IPV6_PFXTAIL_LEN(hdr.cmpri)];
    }
    void ipv6_rpl_srh_decompress(struct ipv6_rpl_sr_hdr *outhdr,
    const struct ipv6_rpl_sr_hdr *inhdr,
    const struct in6_addr *daddr, unsigned char n)
    {
    int i;
    outhdr.nexthdr = inhdr.nexthdr;
    outhdr.hdrlen = (((n + 1) * sizeof(struct in6_addr)) >> 3);
    outhdr.pad = 0;
    outhdr.type = inhdr.type;
    outhdr.segments_left = inhdr.segments_left;
    outhdr.cmpri = 0;
    outhdr.cmpre = 0;
    for (i = 0; i < n; i++)
    ipv6_rpl_addr_decompress(&outhdr.rpl_segaddr[i], daddr,
    ipv6_rpl_segdata_pos(inhdr, i),
    inhdr.cmpri);
    ipv6_rpl_addr_decompress(&outhdr.rpl_segaddr[n], daddr,
    ipv6_rpl_segdata_pos(inhdr, n),
    inhdr.cmpre);
    }
    static unsigned char ipv6_rpl_srh_calc_cmpri(const struct ipv6_rpl_sr_hdr *inhdr,
    const struct in6_addr *daddr,
    unsigned char n)
    {
    unsigned char plen;
    int i;
    for (plen = 0; plen < sizeof(*daddr); plen++) {
    for (i = 0; i < n; i++) {
    if (daddr.s6_addr[plen] !=
    inhdr.rpl_segaddr[i].s6_addr[plen])
    return plen;
    }
    }
    return IPV6_RPL_BEST_ADDR_COMPRESSION;
    }
    static unsigned char ipv6_rpl_srh_calc_cmpre(const struct in6_addr *daddr,
    const struct in6_addr *last_segment)
    {
    unsigned int plen;
    for (plen = 0; plen < sizeof(*daddr); plen++) {
    if (daddr.s6_addr[plen] != last_segment.s6_addr[plen])
    return plen;
    }
    return IPV6_RPL_BEST_ADDR_COMPRESSION;
    }
    void ipv6_rpl_srh_compress(struct ipv6_rpl_sr_hdr *outhdr,
    const struct ipv6_rpl_sr_hdr *inhdr,
    const struct in6_addr *daddr, unsigned char n)
    {
    unsigned char cmpri, cmpre;
    size_t seglen;
    int i;
    cmpri = ipv6_rpl_srh_calc_cmpri(inhdr, daddr, n);
    cmpre = ipv6_rpl_srh_calc_cmpre(daddr, &inhdr.rpl_segaddr[n]);
    outhdr.nexthdr = inhdr.nexthdr;
    seglen = (n * IPV6_PFXTAIL_LEN(cmpri)) + IPV6_PFXTAIL_LEN(cmpre);
    outhdr.hdrlen = seglen >> 3;
    if (seglen & 0x7) {
    outhdr.hdrlen++;
    outhdr.pad = 8 - (seglen & 0x7);
    } else {
    outhdr.pad = 0;
    }
    outhdr.type = inhdr.type;
    outhdr.segments_left = inhdr.segments_left;
    outhdr.cmpri = cmpri;
    outhdr.cmpre = cmpre;
    for (i = 0; i < n; i++)
    ipv6_rpl_addr_compress(ipv6_rpl_segdata_pos(outhdr, i),
    &inhdr.rpl_segaddr[i], cmpri);
    ipv6_rpl_addr_compress(ipv6_rpl_segdata_pos(outhdr, n),
    &inhdr.rpl_segaddr[n], cmpre);
    }
