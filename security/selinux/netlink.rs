//! Automatically rewritten from C to Rust
//! Source: security/selinux/netlink.c
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
// Netlink event notifications for SELinux.
//
// Author: James Morris <jmorris@redhat.com>
//
// Copyright (C) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
//

    static struct sock *selnl __ro_after_init;
#[no_mangle]
unsafe extern "C" fn selnl_msglen(msgtype: c_int) -> c_int {
    static int selnl_msglen(int msgtype)
    {
    let mut ret: c_int = 0;
    switch (msgtype) {
    case SELNL_MSG_SETENFORCE:
    ret = sizeof(struct selnl_msg_setenforce);
    break;
    case SELNL_MSG_POLICYLOAD:
    ret = sizeof(struct selnl_msg_policyload);
    break;
    default:
    BUG();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn selnl_add_payload(nlh: *mut nlmsghdr, len: c_int, msgtype: c_int, data: *mut c_void) {
    static void selnl_add_payload(struct nlmsghdr *nlh, int len, int msgtype, void *data)
    {
    switch (msgtype) {
    case SELNL_MSG_SETENFORCE: {
    struct selnl_msg_setenforce *msg = nlmsg_data(nlh);
    memset(msg, 0, len);
    msg.val = *((int *)data);
    break;
    }
    case SELNL_MSG_POLICYLOAD: {
    struct selnl_msg_policyload *msg = nlmsg_data(nlh);
    memset(msg, 0, len);
    msg.seqno = *((u32 *)data);
    break;
    }
    default:
    BUG();
    }
    }
#[no_mangle]
unsafe extern "C" fn selnl_notify(msgtype: c_int, data: *mut c_void) {
    static void selnl_notify(int msgtype, void *data)
    {
    int len;
    sk_buff_data_t tmp;
    struct sk_buff *skb;
    struct nlmsghdr *nlh;
    len = selnl_msglen(msgtype);
    skb = nlmsg_new(len, GFP_USER);
    if (!skb)
    goto oom;
    tmp = skb.tail;
    nlh = nlmsg_put(skb, 0, 0, msgtype, len, 0);
    if (!nlh)
    goto out_kfree_skb;
    selnl_add_payload(nlh, len, msgtype, data);
    nlh.nlmsg_len = skb.tail - tmp;
    NETLINK_CB(skb).dst_group = SELNLGRP_AVC;
    netlink_broadcast(selnl, skb, 0, SELNLGRP_AVC, GFP_USER);
    out:
    return;
    out_kfree_skb:
    kfree_skb(skb);
    oom:
    pr_err("SELinux:  OOM in %s\n", __func__);
    goto out;
    }
#[no_mangle]
pub unsafe extern "C" fn selnl_notify_setenforce(val: c_int) {
    void selnl_notify_setenforce(int val)
    {
    selnl_notify(SELNL_MSG_SETENFORCE, &val);
    }
#[no_mangle]
pub unsafe extern "C" fn selnl_notify_policyload(seqno: u32) {
    void selnl_notify_policyload(u32 seqno)
    {
    selnl_notify(SELNL_MSG_POLICYLOAD, &seqno);
    }
#[no_mangle]
pub unsafe extern "C" fn sel_netlink_init() -> int __init {
    int __init sel_netlink_init(void)
    {
    struct netlink_kernel_cfg cfg = {
    .groups	= SELNLGRP_MAX,
    .flags	= NL_CFG_F_NONROOT_RECV,
    };
    selnl = netlink_kernel_create(&init_net, NETLINK_SELINUX, &cfg);
    if (selnl == core::ptr::null_mut())
    panic("SELinux:  Cannot create netlink socket.");
    return 0;
    }
