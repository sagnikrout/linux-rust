//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_none.c
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
// net/dsa/tag_none.c - Traffic handling for switches with no tag
// Copyright (c) 2008-2009 Marvell Semiconductor
// Copyright (c) 2013 Florian Fainelli <florian@openwrt.org>
//
// WARNING: do not use this for new switches. In case of no hardware
// tagging support, look at tag_8021q.c instead.
//

    static struct sk_buff *dsa_user_notag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
// Just return the original SKB
    return skb;
    }
    static const struct dsa_device_ops none_ops = {
    .name	= NONE_NAME,
    .proto	= DSA_TAG_PROTO_NONE,
    .xmit	= dsa_user_notag_xmit,
    };
    module_dsa_tag_driver(none_ops);
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_NONE, NONE_NAME);
    MODULE_DESCRIPTION("DSA no-op tag driver");
    MODULE_LICENSE("GPL");
