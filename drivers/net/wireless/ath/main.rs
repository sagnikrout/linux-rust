//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ath/main.c
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


//
// Copyright (c) 2009 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

    MODULE_AUTHOR("Atheros Communications");
    MODULE_DESCRIPTION("Shared library for Atheros wireless LAN cards.");
    MODULE_LICENSE("Dual BSD/GPL");
    struct sk_buff *ath_rxbuf_alloc(struct ath_common *common,
    u32 len,
    gfp_t gfp_mask)
    {
    struct sk_buff *skb;
    u32 off;
//
// Cache-line-align.  This is important (for the
// 5210 at least) as not doing so causes bogus data
// in rx'd frames.
//
// Note: the kernel can allocate a value greater than
// what we ask it to give us. We really only need 4 KB as that
// is this hardware supports and in fact we need at least 3849
// as that is the MAX AMSDU size this hardware supports.
// Unfortunately this means we may get 8 KB here from the
// kernel... and that is actually what is observed on some
// systems :(
    skb = __dev_alloc_skb(len + common.cachelsz - 1, gfp_mask);
    if (skb != core::ptr::null_mut()) {
    off = ((unsigned long) skb.data) % common.cachelsz;
    if (off != 0)
    skb_reserve(skb, common.cachelsz - off);
    } else {
    pr_err("skbuff alloc of size %u failed\n", len);
    return core::ptr::null_mut();
    }
    return skb;
    }
    EXPORT_SYMBOL(ath_rxbuf_alloc);
#[no_mangle]
pub unsafe extern "C" fn ath_is_mybeacon(common: *mut ath_common, hdr: *mut ieee80211_hdr) -> bool {
    bool ath_is_mybeacon(struct ath_common *common, struct ieee80211_hdr *hdr)
    {
    return ieee80211_is_beacon(hdr.frame_control) &&
    !is_zero_ether_addr(common.curbssid) &&
    ether_addr_equal_64bits(hdr.addr3, common.curbssid);
    }
    EXPORT_SYMBOL(ath_is_mybeacon);
    void ath_printk(const char *level, const struct ath_common* common,
    const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    if (common && common.hw && common.hw.wiphy) {
    printk("%sath: %s: %pV",
    level, wiphy_name(common.hw.wiphy), &vaf);
    trace_ath_log(common.hw.wiphy, &vaf);
    } else {
    printk("%sath: %pV", level, &vaf);
    }
    va_end(args);
    }
    EXPORT_SYMBOL(ath_printk);
    const char *ath_bus_type_strings[] = {
    [ATH_PCI] = "pci",
    [ATH_AHB] = "ahb",
    [ATH_USB] = "usb",
    };
    EXPORT_SYMBOL(ath_bus_type_strings);
