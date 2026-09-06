//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ath/debug.c
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

    const char *ath_opmode_to_string(enum nl80211_iftype opmode)
    {
    switch (opmode) {
    case NL80211_IFTYPE_UNSPECIFIED:
    return "UNSPEC";
    case NL80211_IFTYPE_ADHOC:
    return "ADHOC";
    case NL80211_IFTYPE_STATION:
    return "STATION";
    case NL80211_IFTYPE_AP:
    return "AP";
    case NL80211_IFTYPE_AP_VLAN:
    return "AP-VLAN";
    case NL80211_IFTYPE_WDS:
    return "WDS";
    case NL80211_IFTYPE_MONITOR:
    return "MONITOR";
    case NL80211_IFTYPE_MESH_POINT:
    return "MESH";
    case NL80211_IFTYPE_P2P_CLIENT:
    return "P2P-CLIENT";
    case NL80211_IFTYPE_P2P_GO:
    return "P2P-GO";
    case NL80211_IFTYPE_OCB:
    return "OCB";
    default:
    return "UNKNOWN";
    }
    }
    EXPORT_SYMBOL(ath_opmode_to_string);
