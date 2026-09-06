//! Automatically rewritten from C to Rust
//! Source: net/ipv6/exthdrs_offload.c
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
// IPV6 GSO/GRO offload support
// Linux INET6 implementation
//
// IPV6 Extension Header GSO/GRO support
//

    static const struct net_offload rthdr_offload = {
    .flags		=	INET6_PROTO_GSO_EXTHDR,
    };
    static const struct net_offload dstopt_offload = {
    .flags		=	INET6_PROTO_GSO_EXTHDR,
    };
    static const struct net_offload hbh_offload = {
    .flags		=	INET6_PROTO_GSO_EXTHDR,
    };
#[no_mangle]
pub unsafe extern "C" fn ipv6_exthdrs_offload_init() -> int __init {
    int __init ipv6_exthdrs_offload_init(void)
    {
    int ret;
    ret = inet6_add_offload(&rthdr_offload, IPPROTO_ROUTING);
    if (ret)
    goto out;
    ret = inet6_add_offload(&dstopt_offload, IPPROTO_DSTOPTS);
    if (ret)
    goto out_rt;
    ret = inet6_add_offload(&hbh_offload, IPPROTO_HOPOPTS);
    if (ret)
    goto out_dstopts;
    out:
    return ret;
    out_dstopts:
    inet6_del_offload(&dstopt_offload, IPPROTO_DSTOPTS);
    out_rt:
    inet6_del_offload(&rthdr_offload, IPPROTO_ROUTING);
    goto out;
    }
