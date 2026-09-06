//! Automatically rewritten from C to Rust
//! Source: net/ethtool/linkinfo.c
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
pub struct linkinfo_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linkinfo_reply_data {
    pub base: ethnl_reply_data,
    pub ksettings: ethtool_link_ksettings,
    pub lsettings: *mut ethtool_link_settings,
}

    container_of(__reply_base, struct linkinfo_reply_data, base)
    const struct nla_policy ethnl_linkinfo_get_policy[] = {
    [ETHTOOL_A_LINKINFO_HEADER]		=
    NLA_POLICY_NESTED(ethnl_header_policy),
    };
    static int linkinfo_prepare_data(const struct ethnl_req_info *req_base,
    struct ethnl_reply_data *reply_base,
    const struct genl_info *info)
    {
    struct linkinfo_reply_data *data = LINKINFO_REPDATA(reply_base);
    struct net_device *dev = reply_base.dev;
    int ret;
    data.lsettings = &data.ksettings.base;
    ret = ethnl_ops_begin(dev);
    if (ret < 0)
    return ret;
    ret = netif_get_link_ksettings(dev, &data.ksettings);
    if (ret < 0)
    GENL_SET_ERR_MSG(info, "failed to retrieve link settings");
    ethnl_ops_complete(dev);
    return ret;
    }
    static int linkinfo_reply_size(const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    return nla_total_size(sizeof(u8)) /* LINKINFO_PORT */
    + nla_total_size(sizeof(u8)) /* LINKINFO_PHYADDR */
    + nla_total_size(sizeof(u8)) /* LINKINFO_TP_MDIX */
    + nla_total_size(sizeof(u8)) /* LINKINFO_TP_MDIX_CTRL */
    + nla_total_size(sizeof(u8)) /* LINKINFO_TRANSCEIVER */
    + 0;
    }
    static int linkinfo_fill_reply(struct sk_buff *skb,
    const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct linkinfo_reply_data *data = LINKINFO_REPDATA(reply_base);
    if (nla_put_u8(skb, ETHTOOL_A_LINKINFO_PORT, data.lsettings.port) ||
    nla_put_u8(skb, ETHTOOL_A_LINKINFO_PHYADDR,
    data.lsettings.phy_address) ||
    nla_put_u8(skb, ETHTOOL_A_LINKINFO_TP_MDIX,
    data.lsettings.eth_tp_mdix) ||
    nla_put_u8(skb, ETHTOOL_A_LINKINFO_TP_MDIX_CTRL,
    data.lsettings.eth_tp_mdix_ctrl) ||
    nla_put_u8(skb, ETHTOOL_A_LINKINFO_TRANSCEIVER,
    data.lsettings.transceiver))
    return -EMSGSIZE;
    return 0;
    }
// LINKINFO_SET
    const struct nla_policy ethnl_linkinfo_set_policy[] = {
    [ETHTOOL_A_LINKINFO_HEADER]		=
    NLA_POLICY_NESTED(ethnl_header_policy),
    [ETHTOOL_A_LINKINFO_PORT]		= { .type = NLA_U8 },
    [ETHTOOL_A_LINKINFO_PHYADDR]		= { .type = NLA_U8 },
    [ETHTOOL_A_LINKINFO_TP_MDIX_CTRL]	= { .type = NLA_U8 },
    };
    static int
    ethnl_set_linkinfo_validate(struct ethnl_req_info *req_info,
    struct genl_info *info)
    {
    const struct ethtool_ops *ops = req_info.dev.ethtool_ops;
    if (!ops.get_link_ksettings || !ops.set_link_ksettings)
    return -EOPNOTSUPP;
    return 1;
    }
    static int
    ethnl_set_linkinfo(struct ethnl_req_info *req_info, struct genl_info *info)
    {
    let mut ksettings: ethtool_link_ksettings = {};
    struct ethtool_link_settings *lsettings;
    struct net_device *dev = req_info.dev;
    struct nlattr **tb = info.attrs;
    let mut mod: bool = false;
    int ret;
    ret = netif_get_link_ksettings(dev, &ksettings);
    if (ret < 0) {
    GENL_SET_ERR_MSG(info, "failed to retrieve link settings");
    return ret;
    }
    lsettings = &ksettings.base;
    ethnl_update_u8(&lsettings.port, tb[ETHTOOL_A_LINKINFO_PORT], &mod);
    ethnl_update_u8(&lsettings.phy_address, tb[ETHTOOL_A_LINKINFO_PHYADDR],
    &mod);
    ethnl_update_u8(&lsettings.eth_tp_mdix_ctrl,
    tb[ETHTOOL_A_LINKINFO_TP_MDIX_CTRL], &mod);
    if (!mod)
    return 0;
    ret = dev.ethtool_ops.set_link_ksettings(dev, &ksettings);
    if (ret < 0) {
    GENL_SET_ERR_MSG(info, "link settings update failed");
    return ret;
    }
    return 1;
    }
    const struct ethnl_request_ops ethnl_linkinfo_request_ops = {
    .request_cmd		= ETHTOOL_MSG_LINKINFO_GET,
    .reply_cmd		= ETHTOOL_MSG_LINKINFO_GET_REPLY,
    .hdr_attr		= ETHTOOL_A_LINKINFO_HEADER,
    .req_info_size		= sizeof(struct linkinfo_req_info),
    .reply_data_size	= sizeof(struct linkinfo_reply_data),
    .prepare_data		= linkinfo_prepare_data,
    .reply_size		= linkinfo_reply_size,
    .fill_reply		= linkinfo_fill_reply,
    .set_validate		= ethnl_set_linkinfo_validate,
    .set			= ethnl_set_linkinfo,
    .set_ntf_cmd		= ETHTOOL_MSG_LINKINFO_NTF,
    };
