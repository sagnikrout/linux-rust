//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt7601u/util.c
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
//

#[no_mangle]
pub unsafe extern "C" fn mt76_remove_hdr_pad(skb: *mut sk_buff) {
    void mt76_remove_hdr_pad(struct sk_buff *skb)
    {
    let mut len: c_int = ieee80211_get_hdrlen_from_skb(skb);
    memmove(skb.data + 2, skb.data, len);
    skb_pull(skb, 2);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76_insert_hdr_pad(skb: *mut sk_buff) -> c_int {
    int mt76_insert_hdr_pad(struct sk_buff *skb)
    {
    let mut len: c_int = ieee80211_get_hdrlen_from_skb(skb);
    int ret;
    if (len % 4 == 0)
    return 0;
    ret = skb_cow(skb, 2);
    if (ret)
    return ret;
    skb_push(skb, 2);
    memmove(skb.data, skb.data + 2, len);
    skb.data[len] = 0;
    skb.data[len + 1] = 0;
    return 0;
    }
