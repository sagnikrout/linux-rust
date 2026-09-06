//! Automatically rewritten from C to Rust
//! Source: net/ethtool/stats.c
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
pub struct stats_req_info {
    pub base: ethnl_req_info,
    pub __ETHTOOL_STATS_CNT): DECLARE_BITMAP(stat_mask,,
    pub src: enum ethtool_mac_stats_src,
}

    container_of(__req_base, struct stats_req_info, base)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_reply_data {
    pub base: ethnl_reply_data,
    struct_group(stats,
    pub phy_stats: ethtool_eth_phy_stats,
    pub mac_stats: ethtool_eth_mac_stats,
    pub ctrl_stats: ethtool_eth_ctrl_stats,
    pub rmon_stats: ethtool_rmon_stats,
    pub phydev_stats: ethtool_phy_stats,
    pub rmon_ranges: *const ethtool_rmon_hist_range,
}

    container_of(__reply_base, struct stats_reply_data, base)
    const char stats_std_names[__ETHTOOL_STATS_CNT][ETH_GSTRING_LEN] = {
    [ETHTOOL_STATS_ETH_PHY]			= "eth-phy",
    [ETHTOOL_STATS_ETH_MAC]			= "eth-mac",
    [ETHTOOL_STATS_ETH_CTRL]		= "eth-ctrl",
    [ETHTOOL_STATS_RMON]			= "rmon",
    [ETHTOOL_STATS_PHY]			= "phydev",
    };
    const char stats_eth_phy_names[__ETHTOOL_A_STATS_ETH_PHY_CNT][ETH_GSTRING_LEN] = {
    [ETHTOOL_A_STATS_ETH_PHY_5_SYM_ERR]	= "SymbolErrorDuringCarrier",
    };
    const char stats_eth_mac_names[__ETHTOOL_A_STATS_ETH_MAC_CNT][ETH_GSTRING_LEN] = {
    [ETHTOOL_A_STATS_ETH_MAC_2_TX_PKT]	= "FramesTransmittedOK",
    [ETHTOOL_A_STATS_ETH_MAC_3_SINGLE_COL]	= "SingleCollisionFrames",
    [ETHTOOL_A_STATS_ETH_MAC_4_MULTI_COL]	= "MultipleCollisionFrames",
    [ETHTOOL_A_STATS_ETH_MAC_5_RX_PKT]	= "FramesReceivedOK",
    [ETHTOOL_A_STATS_ETH_MAC_6_FCS_ERR]	= "FrameCheckSequenceErrors",
    [ETHTOOL_A_STATS_ETH_MAC_7_ALIGN_ERR]	= "AlignmentErrors",
    [ETHTOOL_A_STATS_ETH_MAC_8_TX_BYTES]	= "OctetsTransmittedOK",
    [ETHTOOL_A_STATS_ETH_MAC_9_TX_DEFER]	= "FramesWithDeferredXmissions",
    [ETHTOOL_A_STATS_ETH_MAC_10_LATE_COL]	= "LateCollisions",
    [ETHTOOL_A_STATS_ETH_MAC_11_XS_COL]	= "FramesAbortedDueToXSColls",
    [ETHTOOL_A_STATS_ETH_MAC_12_TX_INT_ERR]	= "FramesLostDueToIntMACXmitError",
    [ETHTOOL_A_STATS_ETH_MAC_13_CS_ERR]	= "CarrierSenseErrors",
    [ETHTOOL_A_STATS_ETH_MAC_14_RX_BYTES]	= "OctetsReceivedOK",
    [ETHTOOL_A_STATS_ETH_MAC_15_RX_INT_ERR]	= "FramesLostDueToIntMACRcvError",
    [ETHTOOL_A_STATS_ETH_MAC_18_TX_MCAST]	= "MulticastFramesXmittedOK",
    [ETHTOOL_A_STATS_ETH_MAC_19_TX_BCAST]	= "BroadcastFramesXmittedOK",
    [ETHTOOL_A_STATS_ETH_MAC_20_XS_DEFER]	= "FramesWithExcessiveDeferral",
    [ETHTOOL_A_STATS_ETH_MAC_21_RX_MCAST]	= "MulticastFramesReceivedOK",
    [ETHTOOL_A_STATS_ETH_MAC_22_RX_BCAST]	= "BroadcastFramesReceivedOK",
    [ETHTOOL_A_STATS_ETH_MAC_23_IR_LEN_ERR]	= "InRangeLengthErrors",
    [ETHTOOL_A_STATS_ETH_MAC_24_OOR_LEN]	= "OutOfRangeLengthField",
    [ETHTOOL_A_STATS_ETH_MAC_25_TOO_LONG_ERR]	= "FrameTooLongErrors",
    };
    const char stats_eth_ctrl_names[__ETHTOOL_A_STATS_ETH_CTRL_CNT][ETH_GSTRING_LEN] = {
    [ETHTOOL_A_STATS_ETH_CTRL_3_TX]		= "MACControlFramesTransmitted",
    [ETHTOOL_A_STATS_ETH_CTRL_4_RX]		= "MACControlFramesReceived",
    [ETHTOOL_A_STATS_ETH_CTRL_5_RX_UNSUP]	= "UnsupportedOpcodesReceived",
    };
    const char stats_rmon_names[__ETHTOOL_A_STATS_RMON_CNT][ETH_GSTRING_LEN] = {
    [ETHTOOL_A_STATS_RMON_UNDERSIZE]	= "etherStatsUndersizePkts",
    [ETHTOOL_A_STATS_RMON_OVERSIZE]		= "etherStatsOversizePkts",
    [ETHTOOL_A_STATS_RMON_FRAG]		= "etherStatsFragments",
    [ETHTOOL_A_STATS_RMON_JABBER]		= "etherStatsJabbers",
    };
    const char stats_phy_names[__ETHTOOL_A_STATS_PHY_CNT][ETH_GSTRING_LEN] = {
    [ETHTOOL_A_STATS_PHY_RX_PKTS]		= "RxFrames",
    [ETHTOOL_A_STATS_PHY_RX_BYTES]		= "RxOctets",
    [ETHTOOL_A_STATS_PHY_RX_ERRORS]		= "RxErrors",
    [ETHTOOL_A_STATS_PHY_TX_PKTS]		= "TxFrames",
    [ETHTOOL_A_STATS_PHY_TX_BYTES]		= "TxOctets",
    [ETHTOOL_A_STATS_PHY_TX_ERRORS]		= "TxErrors",
    };
    const struct nla_policy ethnl_stats_get_policy[ETHTOOL_A_STATS_SRC + 1] = {
    [ETHTOOL_A_STATS_HEADER]	=
    NLA_POLICY_NESTED(ethnl_header_policy),
    [ETHTOOL_A_STATS_GROUPS]	= { .type = NLA_NESTED },
    [ETHTOOL_A_STATS_SRC]		=
    NLA_POLICY_MAX(NLA_U32, ETHTOOL_MAC_STATS_SRC_PMAC),
    };
    static int stats_parse_request(struct ethnl_req_info *req_base,
    const struct genl_info *info,
    struct nlattr **tb,
    struct netlink_ext_ack *extack)
    {
    let mut src: enum ethtool_mac_stats_src = ETHTOOL_MAC_STATS_SRC_AGGREGATE;
    struct stats_req_info *req_info = STATS_REQINFO(req_base);
    let mut mod: bool = false;
    int err;
    err = ethnl_update_bitset(req_info.stat_mask, __ETHTOOL_STATS_CNT,
    tb[ETHTOOL_A_STATS_GROUPS], stats_std_names,
    extack, &mod);
    if (err)
    return err;
    if (!mod) {
    NL_SET_ERR_MSG(extack, "no stats requested");
    return -EINVAL;
    }
    if (tb[ETHTOOL_A_STATS_SRC])
    src = nla_get_u32(tb[ETHTOOL_A_STATS_SRC]);
    req_info.src = src;
    return 0;
    }
    static int stats_prepare_data(const struct ethnl_req_info *req_base,
    struct ethnl_reply_data *reply_base,
    const struct genl_info *info)
    {
    const struct stats_req_info *req_info = STATS_REQINFO(req_base);
    struct stats_reply_data *data = STATS_REPDATA(reply_base);
    let mut src: enum ethtool_mac_stats_src = req_info.src;
    struct net_device *dev = reply_base.dev;
    struct nlattr **tb = info.attrs;
    struct phy_device *phydev;
    int ret;
    phydev = ethnl_req_get_phydev(req_base, tb, ETHTOOL_A_STATS_HEADER,
    info.extack);
    if (IS_ERR(phydev))
    return PTR_ERR(phydev);
    ret = ethnl_ops_begin(dev);
    if (ret < 0)
    return ret;
    if ((src == ETHTOOL_MAC_STATS_SRC_EMAC ||
    src == ETHTOOL_MAC_STATS_SRC_PMAC) &&
    !__ethtool_dev_mm_supported(dev)) {
    NL_SET_ERR_MSG_MOD(info.extack,
    "Device does not support MAC merge layer");
    ethnl_ops_complete(dev);
    return -EOPNOTSUPP;
    }
// Mark all stats as unset (see ETHTOOL_STAT_NOT_SET) to prevent them
// from being reported to user space in case driver did not set them.
//
    memset(&data.stats, 0xff, sizeof(data.stats));
    data.phy_stats.src = src;
    data.mac_stats.src = src;
    data.ctrl_stats.src = src;
    data.rmon_stats.src = src;
    if ((test_bit(ETHTOOL_STATS_PHY, req_info.stat_mask) ||
    test_bit(ETHTOOL_STATS_ETH_PHY, req_info.stat_mask)) &&
    src == ETHTOOL_MAC_STATS_SRC_AGGREGATE) {
    if (phydev)
    phy_ethtool_get_phy_stats(phydev, &data.phy_stats,
    &data.phydev_stats);
    }
    if (test_bit(ETHTOOL_STATS_ETH_PHY, req_info.stat_mask) &&
    dev.ethtool_ops.get_eth_phy_stats)
    dev.ethtool_ops.get_eth_phy_stats(dev, &data.phy_stats);
    if (test_bit(ETHTOOL_STATS_ETH_MAC, req_info.stat_mask) &&
    dev.ethtool_ops.get_eth_mac_stats)
    dev.ethtool_ops.get_eth_mac_stats(dev, &data.mac_stats);
    if (test_bit(ETHTOOL_STATS_ETH_CTRL, req_info.stat_mask) &&
    dev.ethtool_ops.get_eth_ctrl_stats)
    dev.ethtool_ops.get_eth_ctrl_stats(dev, &data.ctrl_stats);
    if (test_bit(ETHTOOL_STATS_RMON, req_info.stat_mask) &&
    dev.ethtool_ops.get_rmon_stats)
    dev.ethtool_ops.get_rmon_stats(dev, &data.rmon_stats,
    &data.rmon_ranges);
    ethnl_ops_complete(dev);
    return 0;
    }
    static int stats_reply_size(const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct stats_req_info *req_info = STATS_REQINFO(req_base);
    let mut n_grps: c_uint = 0, n_stats = 0;
    let mut len: c_int = 0;
    len += nla_total_size(sizeof(u32)); /* _STATS_SRC */
    if (test_bit(ETHTOOL_STATS_ETH_PHY, req_info.stat_mask)) {
    n_stats += sizeof(struct ethtool_eth_phy_stats) / sizeof(u64);
    n_grps++;
    }
    if (test_bit(ETHTOOL_STATS_ETH_MAC, req_info.stat_mask)) {
    n_stats += sizeof(struct ethtool_eth_mac_stats) / sizeof(u64);
    n_grps++;
    }
    if (test_bit(ETHTOOL_STATS_ETH_CTRL, req_info.stat_mask)) {
    n_stats += sizeof(struct ethtool_eth_ctrl_stats) / sizeof(u64);
    n_grps++;
    }
    if (test_bit(ETHTOOL_STATS_RMON, req_info.stat_mask)) {
    n_stats += sizeof(struct ethtool_rmon_stats) / sizeof(u64);
    n_grps++;
// Above includes the space for _A_STATS_GRP_HIST_VALs
    len += (nla_total_size(0) +	/* _A_STATS_GRP_HIST */
    nla_total_size(4) +	/* _A_STATS_GRP_HIST_BKT_LOW */
    nla_total_size(4)) *	/* _A_STATS_GRP_HIST_BKT_HI */
    ETHTOOL_RMON_HIST_MAX * 2;
    }
    if (test_bit(ETHTOOL_STATS_PHY, req_info.stat_mask)) {
    n_stats += sizeof(struct ethtool_phy_stats) / sizeof(u64);
    n_grps++;
    }
    len += n_grps * (nla_total_size(0) + /* _A_STATS_GRP */
    nla_total_size(4) + /* _A_STATS_GRP_ID */
    nla_total_size(4)); /* _A_STATS_GRP_SS_ID */
    len += n_stats * (nla_total_size(0) + /* _A_STATS_GRP_STAT */
    nla_total_size_64bit(sizeof(u64)));
    return len;
    }
#[no_mangle]
unsafe extern "C" fn stat_put(skb: *mut sk_buff, attrtype: u16, val: u64) -> c_int {
    static int stat_put(struct sk_buff *skb, u16 attrtype, u64 val)
    {
    struct nlattr *nest;
    int ret;
    if (val == ETHTOOL_STAT_NOT_SET)
    return 0;
// We want to start stats attr types from 0, so we don't have a type
// for pad inside ETHTOOL_A_STATS_GRP_STAT. Pad things on the outside
// of ETHTOOL_A_STATS_GRP_STAT. Since we're one nest away from the
// actual attr we're 4B off - nla_need_padding_for_64bit() & co.
// can't be used.
//

    if (!IS_ALIGNED((unsigned long)skb_tail_pointer(skb), 8))
    if (!nla_reserve(skb, ETHTOOL_A_STATS_GRP_PAD, 0))
    return -EMSGSIZE;

    nest = nla_nest_start(skb, ETHTOOL_A_STATS_GRP_STAT);
    if (!nest)
    return -EMSGSIZE;
    ret = nla_put_u64_64bit(skb, attrtype, val, -1 /* not used */);
    if (ret) {
    nla_nest_cancel(skb, nest);
    return ret;
    }
    nla_nest_end(skb, nest);
    return 0;
    }
    static int stats_put_phy_stats(struct sk_buff *skb,
    const struct stats_reply_data *data)
    {
    if (stat_put(skb, ETHTOOL_A_STATS_ETH_PHY_5_SYM_ERR,
    data.phy_stats.SymbolErrorDuringCarrier))
    return -EMSGSIZE;
    return 0;
    }
    static int stats_put_phydev_stats(struct sk_buff *skb,
    const struct stats_reply_data *data)
    {
    if (stat_put(skb, ETHTOOL_A_STATS_PHY_RX_PKTS,
    data.phydev_stats.rx_packets) ||
    stat_put(skb, ETHTOOL_A_STATS_PHY_RX_BYTES,
    data.phydev_stats.rx_bytes) ||
    stat_put(skb, ETHTOOL_A_STATS_PHY_RX_ERRORS,
    data.phydev_stats.rx_errors) ||
    stat_put(skb, ETHTOOL_A_STATS_PHY_TX_PKTS,
    data.phydev_stats.tx_packets) ||
    stat_put(skb, ETHTOOL_A_STATS_PHY_TX_BYTES,
    data.phydev_stats.tx_bytes) ||
    stat_put(skb, ETHTOOL_A_STATS_PHY_TX_ERRORS,
    data.phydev_stats.tx_errors))
    return -EMSGSIZE;
    return 0;
    }
    static int stats_put_mac_stats(struct sk_buff *skb,
    const struct stats_reply_data *data)
    {
    if (stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_2_TX_PKT,
    data.mac_stats.FramesTransmittedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_3_SINGLE_COL,
    data.mac_stats.SingleCollisionFrames) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_4_MULTI_COL,
    data.mac_stats.MultipleCollisionFrames) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_5_RX_PKT,
    data.mac_stats.FramesReceivedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_6_FCS_ERR,
    data.mac_stats.FrameCheckSequenceErrors) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_7_ALIGN_ERR,
    data.mac_stats.AlignmentErrors) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_8_TX_BYTES,
    data.mac_stats.OctetsTransmittedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_9_TX_DEFER,
    data.mac_stats.FramesWithDeferredXmissions) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_10_LATE_COL,
    data.mac_stats.LateCollisions) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_11_XS_COL,
    data.mac_stats.FramesAbortedDueToXSColls) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_12_TX_INT_ERR,
    data.mac_stats.FramesLostDueToIntMACXmitError) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_13_CS_ERR,
    data.mac_stats.CarrierSenseErrors) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_14_RX_BYTES,
    data.mac_stats.OctetsReceivedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_15_RX_INT_ERR,
    data.mac_stats.FramesLostDueToIntMACRcvError) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_18_TX_MCAST,
    data.mac_stats.MulticastFramesXmittedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_19_TX_BCAST,
    data.mac_stats.BroadcastFramesXmittedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_20_XS_DEFER,
    data.mac_stats.FramesWithExcessiveDeferral) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_21_RX_MCAST,
    data.mac_stats.MulticastFramesReceivedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_22_RX_BCAST,
    data.mac_stats.BroadcastFramesReceivedOK) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_23_IR_LEN_ERR,
    data.mac_stats.InRangeLengthErrors) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_24_OOR_LEN,
    data.mac_stats.OutOfRangeLengthField) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_MAC_25_TOO_LONG_ERR,
    data.mac_stats.FrameTooLongErrors))
    return -EMSGSIZE;
    return 0;
    }
    static int stats_put_ctrl_stats(struct sk_buff *skb,
    const struct stats_reply_data *data)
    {
    if (stat_put(skb, ETHTOOL_A_STATS_ETH_CTRL_3_TX,
    data.ctrl_stats.MACControlFramesTransmitted) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_CTRL_4_RX,
    data.ctrl_stats.MACControlFramesReceived) ||
    stat_put(skb, ETHTOOL_A_STATS_ETH_CTRL_5_RX_UNSUP,
    data.ctrl_stats.UnsupportedOpcodesReceived))
    return -EMSGSIZE;
    return 0;
    }
    static int stats_put_rmon_hist(struct sk_buff *skb, u32 attr, const u64 *hist,
    const struct ethtool_rmon_hist_range *ranges)
    {
    struct nlattr *nest;
    int i;
    if (!ranges)
    return 0;
    for (i = 0; i <	ETHTOOL_RMON_HIST_MAX; i++) {
    if (!ranges[i].low && !ranges[i].high)
    break;
    if (hist[i] == ETHTOOL_STAT_NOT_SET)
    continue;
    nest = nla_nest_start(skb, attr);
    if (!nest)
    return -EMSGSIZE;
    if (nla_put_u32(skb, ETHTOOL_A_STATS_GRP_HIST_BKT_LOW,
    ranges[i].low) ||
    nla_put_u32(skb, ETHTOOL_A_STATS_GRP_HIST_BKT_HI,
    ranges[i].high) ||
    nla_put_u64_64bit(skb, ETHTOOL_A_STATS_GRP_HIST_VAL,
    hist[i], ETHTOOL_A_STATS_GRP_PAD))
    goto err_cancel_hist;
    nla_nest_end(skb, nest);
    }
    return 0;
    err_cancel_hist:
    nla_nest_cancel(skb, nest);
    return -EMSGSIZE;
    }
    static int stats_put_rmon_stats(struct sk_buff *skb,
    const struct stats_reply_data *data)
    {
    if (stats_put_rmon_hist(skb, ETHTOOL_A_STATS_GRP_HIST_RX,
    data.rmon_stats.hist, data.rmon_ranges) ||
    stats_put_rmon_hist(skb, ETHTOOL_A_STATS_GRP_HIST_TX,
    data.rmon_stats.hist_tx, data.rmon_ranges))
    return -EMSGSIZE;
    if (stat_put(skb, ETHTOOL_A_STATS_RMON_UNDERSIZE,
    data.rmon_stats.undersize_pkts) ||
    stat_put(skb, ETHTOOL_A_STATS_RMON_OVERSIZE,
    data.rmon_stats.oversize_pkts) ||
    stat_put(skb, ETHTOOL_A_STATS_RMON_FRAG,
    data.rmon_stats.fragments) ||
    stat_put(skb, ETHTOOL_A_STATS_RMON_JABBER,
    data.rmon_stats.jabbers))
    return -EMSGSIZE;
    return 0;
    }
    static int stats_put_stats(struct sk_buff *skb,
    const struct stats_reply_data *data,
    u32 id, u32 ss_id,
    int (*cb)(struct sk_buff *skb,
    const struct stats_reply_data *data))
    {
    struct nlattr *nest;
    nest = nla_nest_start(skb, ETHTOOL_A_STATS_GRP);
    if (!nest)
    return -EMSGSIZE;
    if (nla_put_u32(skb, ETHTOOL_A_STATS_GRP_ID, id) ||
    nla_put_u32(skb, ETHTOOL_A_STATS_GRP_SS_ID, ss_id))
    goto err_cancel;
    if (cb(skb, data))
    goto err_cancel;
    nla_nest_end(skb, nest);
    return 0;
    err_cancel:
    nla_nest_cancel(skb, nest);
    return -EMSGSIZE;
    }
    static int stats_fill_reply(struct sk_buff *skb,
    const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct stats_req_info *req_info = STATS_REQINFO(req_base);
    const struct stats_reply_data *data = STATS_REPDATA(reply_base);
    let mut ret: c_int = 0;
    if (nla_put_u32(skb, ETHTOOL_A_STATS_SRC, req_info.src))
    return -EMSGSIZE;
    if (!ret && test_bit(ETHTOOL_STATS_ETH_PHY, req_info.stat_mask))
    ret = stats_put_stats(skb, data, ETHTOOL_STATS_ETH_PHY,
    ETH_SS_STATS_ETH_PHY,
    stats_put_phy_stats);
    if (!ret && test_bit(ETHTOOL_STATS_ETH_MAC, req_info.stat_mask))
    ret = stats_put_stats(skb, data, ETHTOOL_STATS_ETH_MAC,
    ETH_SS_STATS_ETH_MAC,
    stats_put_mac_stats);
    if (!ret && test_bit(ETHTOOL_STATS_ETH_CTRL, req_info.stat_mask))
    ret = stats_put_stats(skb, data, ETHTOOL_STATS_ETH_CTRL,
    ETH_SS_STATS_ETH_CTRL,
    stats_put_ctrl_stats);
    if (!ret && test_bit(ETHTOOL_STATS_RMON, req_info.stat_mask))
    ret = stats_put_stats(skb, data, ETHTOOL_STATS_RMON,
    ETH_SS_STATS_RMON, stats_put_rmon_stats);
    if (!ret && test_bit(ETHTOOL_STATS_PHY, req_info.stat_mask))
    ret = stats_put_stats(skb, data, ETHTOOL_STATS_PHY,
    ETH_SS_STATS_PHY, stats_put_phydev_stats);
    return ret;
    }
    const struct ethnl_request_ops ethnl_stats_request_ops = {
    .request_cmd		= ETHTOOL_MSG_STATS_GET,
    .reply_cmd		= ETHTOOL_MSG_STATS_GET_REPLY,
    .hdr_attr		= ETHTOOL_A_STATS_HEADER,
    .req_info_size		= sizeof(struct stats_req_info),
    .reply_data_size	= sizeof(struct stats_reply_data),
    .parse_request		= stats_parse_request,
    .prepare_data		= stats_prepare_data,
    .reply_size		= stats_reply_size,
    .fill_reply		= stats_fill_reply,
    };
#[no_mangle]
unsafe extern "C" fn ethtool_stats_sum(a: u64, b: u64) -> u64 {
    static u64 ethtool_stats_sum(u64 a, u64 b)
    {
    if (a == ETHTOOL_STAT_NOT_SET)
    return b;
    if (b == ETHTOOL_STAT_NOT_SET)
    return a;
    return a + b;
    }
// Avoid modifying the aggregation procedure every time a new counter is added
// by treating the structures as an array of u64 statistics.
//
    static void ethtool_aggregate_stats(void *aggr_stats, const void *emac_stats,
    const void *pmac_stats, size_t stats_size,
    size_t stats_offset)
    {
    let mut num_stats: usize = stats_size / sizeof(u64);
    const u64 *s1 = emac_stats + stats_offset;
    const u64 *s2 = pmac_stats + stats_offset;
    u64 *s = aggr_stats + stats_offset;
    int i;
    for (i = 0; i < num_stats; i++)
    s[i] = ethtool_stats_sum(s1[i], s2[i]);
    }
    void ethtool_aggregate_mac_stats(struct net_device *dev,
    struct ethtool_eth_mac_stats *mac_stats)
    {
    const struct ethtool_ops *ops = dev.ethtool_ops;
    struct ethtool_eth_mac_stats pmac, emac;
    memset(&emac, 0xff, sizeof(emac));
    memset(&pmac, 0xff, sizeof(pmac));
    emac.src = ETHTOOL_MAC_STATS_SRC_EMAC;
    pmac.src = ETHTOOL_MAC_STATS_SRC_PMAC;
    ops.get_eth_mac_stats(dev, &emac);
    ops.get_eth_mac_stats(dev, &pmac);
    ethtool_aggregate_stats(mac_stats, &emac, &pmac,
    sizeof(mac_stats.stats),
    offsetof(struct ethtool_eth_mac_stats, stats));
    }
    EXPORT_SYMBOL(ethtool_aggregate_mac_stats);
    void ethtool_aggregate_phy_stats(struct net_device *dev,
    struct ethtool_eth_phy_stats *phy_stats)
    {
    const struct ethtool_ops *ops = dev.ethtool_ops;
    struct ethtool_eth_phy_stats pmac, emac;
    memset(&emac, 0xff, sizeof(emac));
    memset(&pmac, 0xff, sizeof(pmac));
    emac.src = ETHTOOL_MAC_STATS_SRC_EMAC;
    pmac.src = ETHTOOL_MAC_STATS_SRC_PMAC;
    ops.get_eth_phy_stats(dev, &emac);
    ops.get_eth_phy_stats(dev, &pmac);
    ethtool_aggregate_stats(phy_stats, &emac, &pmac,
    sizeof(phy_stats.stats),
    offsetof(struct ethtool_eth_phy_stats, stats));
    }
    EXPORT_SYMBOL(ethtool_aggregate_phy_stats);
    void ethtool_aggregate_ctrl_stats(struct net_device *dev,
    struct ethtool_eth_ctrl_stats *ctrl_stats)
    {
    const struct ethtool_ops *ops = dev.ethtool_ops;
    struct ethtool_eth_ctrl_stats pmac, emac;
    memset(&emac, 0xff, sizeof(emac));
    memset(&pmac, 0xff, sizeof(pmac));
    emac.src = ETHTOOL_MAC_STATS_SRC_EMAC;
    pmac.src = ETHTOOL_MAC_STATS_SRC_PMAC;
    ops.get_eth_ctrl_stats(dev, &emac);
    ops.get_eth_ctrl_stats(dev, &pmac);
    ethtool_aggregate_stats(ctrl_stats, &emac, &pmac,
    sizeof(ctrl_stats.stats),
    offsetof(struct ethtool_eth_ctrl_stats, stats));
    }
    EXPORT_SYMBOL(ethtool_aggregate_ctrl_stats);
    void ethtool_aggregate_pause_stats(struct net_device *dev,
    struct ethtool_pause_stats *pause_stats)
    {
    const struct ethtool_ops *ops = dev.ethtool_ops;
    struct ethtool_pause_stats pmac, emac;
    memset(&emac, 0xff, sizeof(emac));
    memset(&pmac, 0xff, sizeof(pmac));
    emac.src = ETHTOOL_MAC_STATS_SRC_EMAC;
    pmac.src = ETHTOOL_MAC_STATS_SRC_PMAC;
    ops.get_pause_stats(dev, &emac);
    ops.get_pause_stats(dev, &pmac);
    ethtool_aggregate_stats(pause_stats, &emac, &pmac,
    sizeof(pause_stats.stats),
    offsetof(struct ethtool_pause_stats, stats));
    }
    EXPORT_SYMBOL(ethtool_aggregate_pause_stats);
    void ethtool_aggregate_rmon_stats(struct net_device *dev,
    struct ethtool_rmon_stats *rmon_stats)
    {
    const struct ethtool_ops *ops = dev.ethtool_ops;
    const struct ethtool_rmon_hist_range *dummy;
    struct ethtool_rmon_stats pmac, emac;
    memset(&emac, 0xff, sizeof(emac));
    memset(&pmac, 0xff, sizeof(pmac));
    emac.src = ETHTOOL_MAC_STATS_SRC_EMAC;
    pmac.src = ETHTOOL_MAC_STATS_SRC_PMAC;
    ops.get_rmon_stats(dev, &emac, &dummy);
    ops.get_rmon_stats(dev, &pmac, &dummy);
    ethtool_aggregate_stats(rmon_stats, &emac, &pmac,
    sizeof(rmon_stats.stats),
    offsetof(struct ethtool_rmon_stats, stats));
    }
    EXPORT_SYMBOL(ethtool_aggregate_rmon_stats);
