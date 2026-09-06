//! Automatically rewritten from C to Rust
//! Source: net/ieee802154/netlink.c
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
// Netlink interface for IEEE 802.15.4 stack
//
// Copyright 2007, 2008 Siemens AG
//
// Written by:
// Sergey Lapin <slapin@ossfans.org>
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
// Maxim Osipov <maxim.osipov@siemens.com>
//

    static unsigned int ieee802154_seq_num;
    static DEFINE_SPINLOCK(ieee802154_seq_lock);
// Requests to userspace
    struct sk_buff *ieee802154_nl_create(int flags, u8 req)
    {
    void *hdr;
    struct sk_buff *msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC);
    unsigned long f;
    if (!msg)
    return core::ptr::null_mut();
    spin_lock_irqsave(&ieee802154_seq_lock, f);
    hdr = genlmsg_put(msg, 0, ieee802154_seq_num++,
    &nl802154_family, flags, req);
    spin_unlock_irqrestore(&ieee802154_seq_lock, f);
    if (!hdr) {
    nlmsg_free(msg);
    return core::ptr::null_mut();
    }
    return msg;
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_nl_mcast(msg: *mut sk_buff, group: c_uint) -> c_int {
    int ieee802154_nl_mcast(struct sk_buff *msg, unsigned int group)
    {
    struct nlmsghdr *nlh = nlmsg_hdr(msg);
    void *hdr = genlmsg_data(nlmsg_data(nlh));
    genlmsg_end(msg, hdr);
    return genlmsg_multicast(&nl802154_family, msg, 0, group, GFP_ATOMIC);
    }
    struct sk_buff *ieee802154_nl_new_reply(struct genl_info *info,
    int flags, u8 req)
    {
    void *hdr;
    struct sk_buff *msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC);
    if (!msg)
    return core::ptr::null_mut();
    hdr = genlmsg_put_reply(msg, info,
    &nl802154_family, flags, req);
    if (!hdr) {
    nlmsg_free(msg);
    return core::ptr::null_mut();
    }
    return msg;
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_nl_reply(msg: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ieee802154_nl_reply(struct sk_buff *msg, struct genl_info *info)
    {
    struct nlmsghdr *nlh = nlmsg_hdr(msg);
    void *hdr = genlmsg_data(nlmsg_data(nlh));
    genlmsg_end(msg, hdr);
    return genlmsg_reply(msg, info);
    }
    static const struct genl_small_ops ieee802154_ops[] = {
// see nl-phy.c
    IEEE802154_DUMP(IEEE802154_LIST_PHY, ieee802154_list_phy,
    ieee802154_dump_phy),
    IEEE802154_OP(IEEE802154_ADD_IFACE, ieee802154_add_iface),
    IEEE802154_OP(IEEE802154_DEL_IFACE, ieee802154_del_iface),
// see nl-mac.c
    IEEE802154_OP(IEEE802154_ASSOCIATE_REQ, ieee802154_associate_req),
    IEEE802154_OP(IEEE802154_ASSOCIATE_RESP, ieee802154_associate_resp),
    IEEE802154_OP(IEEE802154_DISASSOCIATE_REQ, ieee802154_disassociate_req),
    IEEE802154_OP(IEEE802154_SCAN_REQ, ieee802154_scan_req),
    IEEE802154_OP(IEEE802154_START_REQ, ieee802154_start_req),
    IEEE802154_DUMP(IEEE802154_LIST_IFACE, ieee802154_list_iface,
    ieee802154_dump_iface),
    IEEE802154_OP(IEEE802154_SET_MACPARAMS, ieee802154_set_macparams),
    IEEE802154_OP(IEEE802154_LLSEC_GETPARAMS, ieee802154_llsec_getparams),
    IEEE802154_OP(IEEE802154_LLSEC_SETPARAMS, ieee802154_llsec_setparams),
    IEEE802154_DUMP_PRIV(IEEE802154_LLSEC_LIST_KEY, core::ptr::null_mut(),
    ieee802154_llsec_dump_keys),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_ADD_KEY, ieee802154_llsec_add_key),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_DEL_KEY, ieee802154_llsec_del_key),
    IEEE802154_DUMP_PRIV(IEEE802154_LLSEC_LIST_DEV, core::ptr::null_mut(),
    ieee802154_llsec_dump_devs),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_ADD_DEV, ieee802154_llsec_add_dev),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_DEL_DEV, ieee802154_llsec_del_dev),
    IEEE802154_DUMP_PRIV(IEEE802154_LLSEC_LIST_DEVKEY, core::ptr::null_mut(),
    ieee802154_llsec_dump_devkeys),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_ADD_DEVKEY, ieee802154_llsec_add_devkey),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_DEL_DEVKEY, ieee802154_llsec_del_devkey),
    IEEE802154_DUMP_PRIV(IEEE802154_LLSEC_LIST_SECLEVEL, core::ptr::null_mut(),
    ieee802154_llsec_dump_seclevels),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_ADD_SECLEVEL,
    ieee802154_llsec_add_seclevel),
    IEEE802154_OP_RELAXED(IEEE802154_LLSEC_DEL_SECLEVEL,
    ieee802154_llsec_del_seclevel),
    };
    static const struct genl_multicast_group ieee802154_mcgrps[] = {
    [IEEE802154_COORD_MCGRP] = { .name = IEEE802154_MCAST_COORD_NAME, },
    [IEEE802154_BEACON_MCGRP] = { .name = IEEE802154_MCAST_BEACON_NAME, },
    };
    struct genl_family nl802154_family __ro_after_init = {
    .hdrsize	= 0,
    .name		= IEEE802154_NL_NAME,
    .version	= 1,
    .maxattr	= IEEE802154_ATTR_MAX,
    .policy		= ieee802154_policy,
    .module		= THIS_MODULE,
    .small_ops	= ieee802154_ops,
    .n_small_ops	= ARRAY_SIZE(ieee802154_ops),
    .resv_start_op	= IEEE802154_LLSEC_DEL_SECLEVEL + 1,
    .mcgrps		= ieee802154_mcgrps,
    .n_mcgrps	= ARRAY_SIZE(ieee802154_mcgrps),
    };
#[no_mangle]
pub unsafe extern "C" fn ieee802154_nl_init() -> int __init {
    int __init ieee802154_nl_init(void)
    {
    return genl_register_family(&nl802154_family);
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_nl_exit() {
    void ieee802154_nl_exit(void)
    {
    genl_unregister_family(&nl802154_family);
    }
