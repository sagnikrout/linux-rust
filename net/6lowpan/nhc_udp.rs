//! Automatically rewritten from C to Rust
//! Source: net/6lowpan/nhc_udp.c
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
// 6LoWPAN IPv6 UDP compression according to RFC6282
//
// Authors:
// Alexander Aring	<aar@pengutronix.de>
//
// Original written by:
// Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
// Jon Smirl <jonsmirl@gmail.com>
//

pub const LOWPAN_NHC_UDP_MASK: c_uint = 0xF8;
pub const LOWPAN_NHC_UDP_ID: c_uint = 0xF0;
pub const LOWPAN_NHC_UDP_4BIT_PORT: c_uint = 0xF0B0;
pub const LOWPAN_NHC_UDP_4BIT_MASK: c_uint = 0xFFF0;
pub const LOWPAN_NHC_UDP_8BIT_PORT: c_uint = 0xF000;
pub const LOWPAN_NHC_UDP_8BIT_MASK: c_uint = 0xFF00;
// values for port compression, _with checksum_ ie bit 5 set to 0
// all inline
pub const LOWPAN_NHC_UDP_CS_P_00: c_uint = 0xF0;
// source 16bit inline, dest = 0xF0 + 8 bit inline
pub const LOWPAN_NHC_UDP_CS_P_01: c_uint = 0xF1;
// source = 0xF0 + 8bit inline, dest = 16 bit inline
pub const LOWPAN_NHC_UDP_CS_P_10: c_uint = 0xF2;
// source & dest = 0xF0B + 4bit inline
pub const LOWPAN_NHC_UDP_CS_P_11: c_uint = 0xF3;
// checksum elided
pub const LOWPAN_NHC_UDP_CS_C: c_uint = 0x04;
#[no_mangle]
unsafe extern "C" fn udp_uncompress(skb: *mut sk_buff, needed: usize) -> c_int {
    static int udp_uncompress(struct sk_buff *skb, size_t needed)
    {
    let mut tmp: u8 = 0, val = 0;
    struct udphdr uh;
    bool fail;
    int err;
    fail = lowpan_fetch_skb(skb, &tmp, sizeof(tmp));
    pr_debug("UDP header uncompression\n");
    switch (tmp & LOWPAN_NHC_UDP_CS_P_11) {
    case LOWPAN_NHC_UDP_CS_P_00:
    fail |= lowpan_fetch_skb(skb, &uh.source, sizeof(uh.source));
    fail |= lowpan_fetch_skb(skb, &uh.dest, sizeof(uh.dest));
    break;
    case LOWPAN_NHC_UDP_CS_P_01:
    fail |= lowpan_fetch_skb(skb, &uh.source, sizeof(uh.source));
    fail |= lowpan_fetch_skb(skb, &val, sizeof(val));
    uh.dest = htons(val + LOWPAN_NHC_UDP_8BIT_PORT);
    break;
    case LOWPAN_NHC_UDP_CS_P_10:
    fail |= lowpan_fetch_skb(skb, &val, sizeof(val));
    uh.source = htons(val + LOWPAN_NHC_UDP_8BIT_PORT);
    fail |= lowpan_fetch_skb(skb, &uh.dest, sizeof(uh.dest));
    break;
    case LOWPAN_NHC_UDP_CS_P_11:
    fail |= lowpan_fetch_skb(skb, &val, sizeof(val));
    uh.source = htons(LOWPAN_NHC_UDP_4BIT_PORT + (val >> 4));
    uh.dest = htons(LOWPAN_NHC_UDP_4BIT_PORT + (val & 0x0f));
    break;
    default:
    BUG();
    }
    pr_debug("uncompressed UDP ports: src = %d, dst = %d\n",
    ntohs(uh.source), ntohs(uh.dest));
// checksum
    if (tmp & LOWPAN_NHC_UDP_CS_C) {
    pr_debug_ratelimited("checksum elided currently not supported\n");
    fail = true;
    } else {
    fail |= lowpan_fetch_skb(skb, &uh.check, sizeof(uh.check));
    }
    if (fail)
    return -EINVAL;
// UDP length needs to be inferred from the lower layers
// here, we obtain the hint from the remaining size of the
// frame
//
    switch (lowpan_dev(skb.dev).lltype) {
    case LOWPAN_LLTYPE_IEEE802154:
    if (lowpan_802154_cb(skb).d_size)
    udp_set_len_short(&uh, lowpan_802154_cb(skb).d_size -
    sizeof(struct ipv6hdr));
    else
    udp_set_len_short(&uh, skb.len + sizeof(struct udphdr));
    break;
    default:
    udp_set_len_short(&uh, skb.len + sizeof(struct udphdr));
    break;
    }
    pr_debug("uncompressed UDP length: src = %d", udp_get_len_short(&uh));
// replace the compressed UDP head by the uncompressed UDP
// header
//
    err = skb_cow(skb, needed);
    if (unlikely(err))
    return err;
    skb_push(skb, sizeof(struct udphdr));
    skb_copy_to_linear_data(skb, &uh, sizeof(struct udphdr));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udp_compress(skb: *mut sk_buff, hc_ptr: *mut u8) -> c_int {
    static int udp_compress(struct sk_buff *skb, u8 **hc_ptr)
    {
    const struct udphdr *uh = udp_hdr(skb);
    u8 tmp;
    if (((ntohs(uh.source) & LOWPAN_NHC_UDP_4BIT_MASK) ==
    LOWPAN_NHC_UDP_4BIT_PORT) &&
    ((ntohs(uh.dest) & LOWPAN_NHC_UDP_4BIT_MASK) ==
    LOWPAN_NHC_UDP_4BIT_PORT)) {
    pr_debug("UDP header: both ports compression to 4 bits\n");
// compression value
    tmp = LOWPAN_NHC_UDP_CS_P_11;
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
// source and destination port
    tmp = ntohs(uh.dest) - LOWPAN_NHC_UDP_4BIT_PORT +
    ((ntohs(uh.source) - LOWPAN_NHC_UDP_4BIT_PORT) << 4);
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
    } else if ((ntohs(uh.dest) & LOWPAN_NHC_UDP_8BIT_MASK) ==
    LOWPAN_NHC_UDP_8BIT_PORT) {
    pr_debug("UDP header: remove 8 bits of dest\n");
// compression value
    tmp = LOWPAN_NHC_UDP_CS_P_01;
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
// source port
    lowpan_push_hc_data(hc_ptr, &uh.source, sizeof(uh.source));
// destination port
    tmp = ntohs(uh.dest) - LOWPAN_NHC_UDP_8BIT_PORT;
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
    } else if ((ntohs(uh.source) & LOWPAN_NHC_UDP_8BIT_MASK) ==
    LOWPAN_NHC_UDP_8BIT_PORT) {
    pr_debug("UDP header: remove 8 bits of source\n");
// compression value
    tmp = LOWPAN_NHC_UDP_CS_P_10;
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
// source port
    tmp = ntohs(uh.source) - LOWPAN_NHC_UDP_8BIT_PORT;
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
// destination port
    lowpan_push_hc_data(hc_ptr, &uh.dest, sizeof(uh.dest));
    } else {
    pr_debug("UDP header: can't compress\n");
// compression value
    tmp = LOWPAN_NHC_UDP_CS_P_00;
    lowpan_push_hc_data(hc_ptr, &tmp, sizeof(tmp));
// source port
    lowpan_push_hc_data(hc_ptr, &uh.source, sizeof(uh.source));
// destination port
    lowpan_push_hc_data(hc_ptr, &uh.dest, sizeof(uh.dest));
    }
// checksum is always inline
    lowpan_push_hc_data(hc_ptr, &uh.check, sizeof(uh.check));
    return 0;
    }
    LOWPAN_NHC(nhc_udp, "RFC6282 UDP", NEXTHDR_UDP, sizeof(struct udphdr),
    LOWPAN_NHC_UDP_ID, LOWPAN_NHC_UDP_MASK, udp_uncompress, udp_compress);
    module_lowpan_nhc(nhc_udp);
    MODULE_DESCRIPTION("6LoWPAN next header RFC6282 UDP compression");
    MODULE_LICENSE("GPL");
