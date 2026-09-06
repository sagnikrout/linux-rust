//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_netlink.c
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
// scsi_netlink.c  - SCSI Transport Netlink Interface
//
// Copyright (C) 2006   James Smart, Emulex Corporation
//

    struct sock *scsi_nl_sock = core::ptr::null_mut();
    EXPORT_SYMBOL_GPL(scsi_nl_sock);
//
// scsi_nl_rcv_msg - Receive message handler.
// @skb:		socket receive buffer
//
// Description: Extracts message from a receive buffer.
// Validates message header and calls appropriate transport message handler
//
    static void
    scsi_nl_rcv_msg(struct sk_buff *skb)
    {
    struct nlmsghdr *nlh;
    struct scsi_nl_hdr *hdr;
    u32 rlen;
    int err, tport;
    while (skb.len >= NLMSG_HDRLEN) {
    err = 0;
    nlh = nlmsg_hdr(skb);
    if ((nlh.nlmsg_len < (sizeof(*nlh) + sizeof(*hdr))) ||
    (skb.len < nlh.nlmsg_len)) {
    printk(KERN_WARNING "%s: discarding partial skb\n",
    __func__);
    return;
    }
    rlen = NLMSG_ALIGN(nlh.nlmsg_len);
    if (rlen > skb.len)
    rlen = skb.len;
    if (nlh.nlmsg_type != SCSI_TRANSPORT_MSG) {
    err = -EBADMSG;
    goto next_msg;
    }
    hdr = nlmsg_data(nlh);
    if ((hdr.version != SCSI_NL_VERSION) ||
    (hdr.magic != SCSI_NL_MAGIC)) {
    err = -EPROTOTYPE;
    goto next_msg;
    }
    if (!netlink_capable(skb, CAP_SYS_ADMIN)) {
    err = -EPERM;
    goto next_msg;
    }
    if (nlh.nlmsg_len < (sizeof(*nlh) + hdr.msglen)) {
    printk(KERN_WARNING "%s: discarding partial message\n",
    __func__);
    goto next_msg;
    }
//
// Deliver message to the appropriate transport
//
    tport = hdr.transport;
    if (tport == SCSI_NL_TRANSPORT) {
    switch (hdr.msgtype) {
    case SCSI_NL_SHOST_VENDOR:
// Locate the driver that corresponds to the message
    err = -ESRCH;
    break;
    default:
    err = -EBADR;
    break;
    }
    if (err)
    printk(KERN_WARNING "%s: Msgtype %d failed - err %d\n",
    __func__, hdr.msgtype, err);
    }
    else
    err = -ENOENT;
    next_msg:
    if ((err) || (nlh.nlmsg_flags & NLM_F_ACK))
    netlink_ack(skb, nlh, err, core::ptr::null_mut());
    skb_pull(skb, rlen);
    }
    }
//
// scsi_netlink_init - Called by SCSI subsystem to initialize
// the SCSI transport netlink interface
//
    void
    scsi_netlink_init(void)
    {
    struct netlink_kernel_cfg cfg = {
    .input	= scsi_nl_rcv_msg,
    .groups	= SCSI_NL_GRP_CNT,
    };
    scsi_nl_sock = netlink_kernel_create(&init_net, NETLINK_SCSITRANSPORT,
    &cfg);
    if (!scsi_nl_sock) {
    printk(KERN_ERR "%s: register of receive handler failed\n",
    __func__);
    return;
    }
    return;
    }
//
// scsi_netlink_exit - Called by SCSI subsystem to disable the SCSI transport netlink interface
//
    void
    scsi_netlink_exit(void)
    {
    if (scsi_nl_sock) {
    netlink_kernel_release(scsi_nl_sock);
    }
    return;
    }
