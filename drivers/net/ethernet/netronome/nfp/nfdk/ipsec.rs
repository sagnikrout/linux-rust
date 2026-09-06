//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfdk/ipsec.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2023 Corigine, Inc

#[no_mangle]
pub unsafe extern "C" fn nfp_nfdk_ipsec_tx(flags: u64, skb: *mut sk_buff) -> u64 {
    u64 nfp_nfdk_ipsec_tx(u64 flags, struct sk_buff *skb)
    {
    struct xfrm_state *x = xfrm_input_state(skb);
    struct iphdr *iph = ip_hdr(skb);
    if (x.xso.dev && (x.xso.dev.features & NETIF_F_HW_ESP_TX_CSUM)) {
    if (iph.version == 4)
    flags |= NFDK_DESC_TX_L3_CSUM;
    flags |= NFDK_DESC_TX_L4_CSUM;
    }
    return flags;
    }
