//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netlink.c
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

    int rtm_getroute_parse_ip_proto(struct nlattr *attr, u8 *ip_proto, u8 family,
    struct netlink_ext_ack *extack)
    {
// ip_proto = nla_get_u8(attr);
    switch (*ip_proto) {
    case IPPROTO_TCP:
    case IPPROTO_UDP:
    return 0;
    case IPPROTO_ICMP:
    if (family != AF_INET)
    break;
    return 0;

    case IPPROTO_ICMPV6:
    if (family != AF_INET6)
    break;
    return 0;

    }
    NL_SET_ERR_MSG(extack, "Unsupported ip proto");
    return -EOPNOTSUPP;
    }
    EXPORT_SYMBOL_GPL(rtm_getroute_parse_ip_proto);
