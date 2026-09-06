//! Automatically rewritten from C to Rust
//! Source: net/ethtool/phc_vclocks.c
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
// Copyright 2021 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phc_vclocks_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phc_vclocks_reply_data {
    pub base: ethnl_reply_data,
    pub num: c_int,
    pub index: *mut c_int,
}

    container_of(__reply_base, struct phc_vclocks_reply_data, base)
    const struct nla_policy ethnl_phc_vclocks_get_policy[] = {
    [ETHTOOL_A_PHC_VCLOCKS_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy),
    };
    static int phc_vclocks_prepare_data(const struct ethnl_req_info *req_base,
    struct ethnl_reply_data *reply_base,
    const struct genl_info *info)
    {
    struct phc_vclocks_reply_data *data = PHC_VCLOCKS_REPDATA(reply_base);
    struct net_device *dev = reply_base.dev;
    int ret;
    ret = ethnl_ops_begin(dev);
    if (ret < 0)
    return ret;
    data.num = ethtool_get_phc_vclocks(dev, &data.index);
    ethnl_ops_complete(dev);
    return ret;
    }
    static int phc_vclocks_reply_size(const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct phc_vclocks_reply_data *data =
    PHC_VCLOCKS_REPDATA(reply_base);
    let mut len: c_int = 0;
    if (data.num > 0) {
    len += nla_total_size(sizeof(u32));
    len += nla_total_size(sizeof(s32) * data.num);
    }
    return len;
    }
    static int phc_vclocks_fill_reply(struct sk_buff *skb,
    const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct phc_vclocks_reply_data *data =
    PHC_VCLOCKS_REPDATA(reply_base);
    if (data.num <= 0)
    return 0;
    if (nla_put_u32(skb, ETHTOOL_A_PHC_VCLOCKS_NUM, data.num) ||
    nla_put(skb, ETHTOOL_A_PHC_VCLOCKS_INDEX,
    sizeof(s32) * data.num, data.index))
    return -EMSGSIZE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phc_vclocks_cleanup_data(reply_base: *mut ethnl_reply_data) {
    static void phc_vclocks_cleanup_data(struct ethnl_reply_data *reply_base)
    {
    const struct phc_vclocks_reply_data *data =
    PHC_VCLOCKS_REPDATA(reply_base);
    kfree(data.index);
    }
    const struct ethnl_request_ops ethnl_phc_vclocks_request_ops = {
    .request_cmd		= ETHTOOL_MSG_PHC_VCLOCKS_GET,
    .reply_cmd		= ETHTOOL_MSG_PHC_VCLOCKS_GET_REPLY,
    .hdr_attr		= ETHTOOL_A_PHC_VCLOCKS_HEADER,
    .req_info_size		= sizeof(struct phc_vclocks_req_info),
    .reply_data_size	= sizeof(struct phc_vclocks_reply_data),
    .prepare_data		= phc_vclocks_prepare_data,
    .reply_size		= phc_vclocks_reply_size,
    .fill_reply		= phc_vclocks_fill_reply,
    .cleanup_data		= phc_vclocks_cleanup_data,
    };
