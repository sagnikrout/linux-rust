//! Automatically rewritten from C to Rust
//! Source: net/ethtool/debug.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_reply_data {
    pub base: ethnl_reply_data,
    pub msg_mask: u32,
}

    container_of(__reply_base, struct debug_reply_data, base)
    const struct nla_policy ethnl_debug_get_policy[] = {
    [ETHTOOL_A_DEBUG_HEADER]	=
    NLA_POLICY_NESTED(ethnl_header_policy),
    };
    static int debug_prepare_data(const struct ethnl_req_info *req_base,
    struct ethnl_reply_data *reply_base,
    const struct genl_info *info)
    {
    struct debug_reply_data *data = DEBUG_REPDATA(reply_base);
    struct net_device *dev = reply_base.dev;
    int ret;
    if (!dev.ethtool_ops.get_msglevel)
    return -EOPNOTSUPP;
    ret = ethnl_ops_begin(dev);
    if (ret < 0)
    return ret;
    data.msg_mask = dev.ethtool_ops.get_msglevel(dev);
    ethnl_ops_complete(dev);
    return 0;
    }
    static int debug_reply_size(const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct debug_reply_data *data = DEBUG_REPDATA(reply_base);
    let mut compact: bool = req_base.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    return ethnl_bitset32_size(&data.msg_mask, core::ptr::null_mut(), NETIF_MSG_CLASS_COUNT,
    netif_msg_class_names, compact);
    }
    static int debug_fill_reply(struct sk_buff *skb,
    const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct debug_reply_data *data = DEBUG_REPDATA(reply_base);
    let mut compact: bool = req_base.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    return ethnl_put_bitset32(skb, ETHTOOL_A_DEBUG_MSGMASK, &data.msg_mask,
    core::ptr::null_mut(), NETIF_MSG_CLASS_COUNT,
    netif_msg_class_names, compact);
    }
// DEBUG_SET
    const struct nla_policy ethnl_debug_set_policy[] = {
    [ETHTOOL_A_DEBUG_HEADER]	=
    NLA_POLICY_NESTED(ethnl_header_policy),
    [ETHTOOL_A_DEBUG_MSGMASK]	= { .type = NLA_NESTED },
    };
    static int
    ethnl_set_debug_validate(struct ethnl_req_info *req_info,
    struct genl_info *info)
    {
    const struct ethtool_ops *ops = req_info.dev.ethtool_ops;
    return ops.get_msglevel && ops.set_msglevel ? 1 : -EOPNOTSUPP;
    }
    static int
    ethnl_set_debug(struct ethnl_req_info *req_info, struct genl_info *info)
    {
    struct net_device *dev = req_info.dev;
    struct nlattr **tb = info.attrs;
    let mut mod: bool = false;
    u32 msg_mask;
    int ret;
    msg_mask = dev.ethtool_ops.get_msglevel(dev);
    ret = ethnl_update_bitset32(&msg_mask, NETIF_MSG_CLASS_COUNT,
    tb[ETHTOOL_A_DEBUG_MSGMASK],
    netif_msg_class_names, info.extack, &mod);
    if (ret < 0 || !mod)
    return ret;
    dev.ethtool_ops.set_msglevel(dev, msg_mask);
    return 1;
    }
    const struct ethnl_request_ops ethnl_debug_request_ops = {
    .request_cmd		= ETHTOOL_MSG_DEBUG_GET,
    .reply_cmd		= ETHTOOL_MSG_DEBUG_GET_REPLY,
    .hdr_attr		= ETHTOOL_A_DEBUG_HEADER,
    .req_info_size		= sizeof(struct debug_req_info),
    .reply_data_size	= sizeof(struct debug_reply_data),
    .prepare_data		= debug_prepare_data,
    .reply_size		= debug_reply_size,
    .fill_reply		= debug_fill_reply,
    .set_validate		= ethnl_set_debug_validate,
    .set			= ethnl_set_debug,
    .set_ntf_cmd		= ETHTOOL_MSG_DEBUG_NTF,
    };
