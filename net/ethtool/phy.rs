//! Automatically rewritten from C to Rust
//! Source: net/ethtool/phy.c
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
// Copyright 2023 Bootlin
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_reply_data {
    pub base: ethnl_reply_data,
    pub phyindex: u32,
    pub drvname: *mut c_char,
    pub name: *mut c_char,
    pub upstream_type: c_uint,
    pub upstream_sfp_name: *mut c_char,
    pub upstream_index: c_uint,
    pub downstream_sfp_name: *mut c_char,
}

    container_of(__reply_base, struct phy_reply_data, base)
    const struct nla_policy ethnl_phy_get_policy[ETHTOOL_A_PHY_HEADER + 1] = {
    [ETHTOOL_A_PHY_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy),
    };
    static int phy_reply_size(const struct ethnl_req_info *req_info,
    const struct ethnl_reply_data *reply_data)
    {
    struct phy_reply_data *rep_data = PHY_REPDATA(reply_data);
    let mut size: usize = 0;
// ETHTOOL_A_PHY_INDEX
    size += nla_total_size(sizeof(u32));
// ETHTOOL_A_DRVNAME
    if (rep_data.drvname)
    size += nla_total_size(strlen(rep_data.drvname) + 1);
// ETHTOOL_A_NAME
    size += nla_total_size(strlen(rep_data.name) + 1);
// ETHTOOL_A_PHY_UPSTREAM_TYPE
    size += nla_total_size(sizeof(u32));
// ETHTOOL_A_PHY_UPSTREAM_SFP_NAME
    if (rep_data.upstream_sfp_name)
    size += nla_total_size(strlen(rep_data.upstream_sfp_name) + 1);
// ETHTOOL_A_PHY_UPSTREAM_INDEX
    if (rep_data.upstream_index)
    size += nla_total_size(sizeof(u32));
// ETHTOOL_A_PHY_DOWNSTREAM_SFP_NAME
    if (rep_data.downstream_sfp_name)
    size += nla_total_size(strlen(rep_data.downstream_sfp_name) + 1);
    return size;
    }
    static int phy_prepare_data(const struct ethnl_req_info *req_info,
    struct ethnl_reply_data *reply_data,
    const struct genl_info *info)
    {
    struct phy_link_topology *topo = reply_data.dev.link_topo;
    struct phy_reply_data *rep_data = PHY_REPDATA(reply_data);
    struct nlattr **tb = info.attrs;
    struct phy_device_node *pdn;
    struct phy_device *phydev;
    int ret;
    phydev = ethnl_req_get_phydev(req_info, tb, ETHTOOL_A_PHY_HEADER,
    info.extack);
    if (IS_ERR_OR_NULL(phydev))
    return -EOPNOTSUPP;
    pdn = xa_load(&topo.phys, phydev.phyindex);
    if (!pdn)
    return -EOPNOTSUPP;
    rep_data.phyindex = phydev.phyindex;
    rep_data.name = kstrdup(dev_name(&phydev.mdio.dev), GFP_KERNEL);
    if (!rep_data.name)
    return -ENOMEM;
    if (phydev.drv) {
    rep_data.drvname = kstrdup(phydev.drv.name, GFP_KERNEL);
    if (!rep_data.drvname) {
    ret = -ENOMEM;
    goto err_free_name;
    }
    }
    rep_data.upstream_type = pdn.upstream_type;
    if (pdn.upstream_type == PHY_UPSTREAM_PHY) {
    struct phy_device *upstream = pdn.upstream.phydev;
    rep_data.upstream_index = upstream.phyindex;
    }
    if (pdn.parent_sfp_bus) {
    rep_data.upstream_sfp_name = kstrdup(sfp_get_name(pdn.parent_sfp_bus),
    GFP_KERNEL);
    if (!rep_data.upstream_sfp_name) {
    ret = -ENOMEM;
    goto err_free_drvname;
    }
    }
    if (phydev.sfp_bus) {
    rep_data.downstream_sfp_name = kstrdup(sfp_get_name(phydev.sfp_bus),
    GFP_KERNEL);
    if (!rep_data.downstream_sfp_name) {
    ret = -ENOMEM;
    goto err_free_upstream_sfp;
    }
    }
    return 0;
    err_free_upstream_sfp:
    kfree(rep_data.upstream_sfp_name);
    err_free_drvname:
    kfree(rep_data.drvname);
    err_free_name:
    kfree(rep_data.name);
    return ret;
    }
    static int phy_fill_reply(struct sk_buff *skb,
    const struct ethnl_req_info *req_info,
    const struct ethnl_reply_data *reply_data)
    {
    struct phy_reply_data *rep_data = PHY_REPDATA(reply_data);
    if (nla_put_u32(skb, ETHTOOL_A_PHY_INDEX, rep_data.phyindex) ||
    nla_put_string(skb, ETHTOOL_A_PHY_NAME, rep_data.name) ||
    nla_put_u32(skb, ETHTOOL_A_PHY_UPSTREAM_TYPE, rep_data.upstream_type))
    return -EMSGSIZE;
    if (rep_data.drvname &&
    nla_put_string(skb, ETHTOOL_A_PHY_DRVNAME, rep_data.drvname))
    return -EMSGSIZE;
    if (rep_data.upstream_index &&
    nla_put_u32(skb, ETHTOOL_A_PHY_UPSTREAM_INDEX,
    rep_data.upstream_index))
    return -EMSGSIZE;
    if (rep_data.upstream_sfp_name &&
    nla_put_string(skb, ETHTOOL_A_PHY_UPSTREAM_SFP_NAME,
    rep_data.upstream_sfp_name))
    return -EMSGSIZE;
    if (rep_data.downstream_sfp_name &&
    nla_put_string(skb, ETHTOOL_A_PHY_DOWNSTREAM_SFP_NAME,
    rep_data.downstream_sfp_name))
    return -EMSGSIZE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_cleanup_data(reply_data: *mut ethnl_reply_data) {
    static void phy_cleanup_data(struct ethnl_reply_data *reply_data)
    {
    struct phy_reply_data *rep_data = PHY_REPDATA(reply_data);
    kfree(rep_data.drvname);
    kfree(rep_data.name);
    kfree(rep_data.upstream_sfp_name);
    kfree(rep_data.downstream_sfp_name);
    }
    const struct ethnl_request_ops ethnl_phy_request_ops = {
    .request_cmd		= ETHTOOL_MSG_PHY_GET,
    .reply_cmd		= ETHTOOL_MSG_PHY_GET_REPLY,
    .hdr_attr		= ETHTOOL_A_PHY_HEADER,
    .req_info_size		= sizeof(struct phy_req_info),
    .reply_data_size	= sizeof(struct phy_reply_data),
    .prepare_data		= phy_prepare_data,
    .reply_size		= phy_reply_size,
    .fill_reply		= phy_fill_reply,
    .cleanup_data		= phy_cleanup_data,
    };
