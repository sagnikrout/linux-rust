//! Automatically rewritten from C to Rust
//! Source: rust/helpers/net/genetlink.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2026 Google LLC.
//

    __rust_helper struct sk_buff *rust_helper_genlmsg_new(size_t payload, gfp_t flags)
    {
    return genlmsg_new(payload, flags);
    }
    __rust_helper
    int rust_helper_genlmsg_multicast(const struct genl_family *family,
    struct sk_buff *skb, u32 portid,
    unsigned int group, gfp_t flags)
    {
    return genlmsg_multicast(family, skb, portid, group, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_genlmsg_cancel(skb: *mut sk_buff, hdr: *mut c_void) -> __rust_helper void {
    __rust_helper void rust_helper_genlmsg_cancel(struct sk_buff *skb, void *hdr)
    {
    genlmsg_cancel(skb, hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_genlmsg_end(skb: *mut sk_buff, hdr: *mut c_void) -> __rust_helper void {
    __rust_helper void rust_helper_genlmsg_end(struct sk_buff *skb, void *hdr)
    {
    genlmsg_end(skb, hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_nlmsg_free(skb: *mut sk_buff) -> __rust_helper void {
    __rust_helper void rust_helper_nlmsg_free(struct sk_buff *skb)
    {
    nlmsg_free(skb);
    }
    __rust_helper
    int rust_helper_genl_has_listeners(const struct genl_family *family,
    struct net *net, unsigned int group)
    {
    return genl_has_listeners(family, net, group);
    }
