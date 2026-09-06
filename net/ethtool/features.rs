//! Automatically rewritten from C to Rust
//! Source: net/ethtool/features.c
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
pub struct features_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct features_reply_data {
    pub base: ethnl_reply_data,
    pub hw: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub wanted: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub active: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub nochange: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub all: [u32; ETHTOOL_DEV_FEATURE_WORDS],
}

    container_of(__reply_base, struct features_reply_data, base)
    const struct nla_policy ethnl_features_get_policy[] = {
    [ETHTOOL_A_FEATURES_HEADER]	=
    NLA_POLICY_NESTED(ethnl_header_policy),
    };
#[no_mangle]
unsafe extern "C" fn ethnl_features_to_bitmap32(dest: *mut u32, src: netdev_features_t) {
    static void ethnl_features_to_bitmap32(u32 *dest, netdev_features_t src)
    {
    unsigned int i;
    for (i = 0; i < ETHTOOL_DEV_FEATURE_WORDS; i++)
    dest[i] = src >> (32 * i);
    }
    static int features_prepare_data(const struct ethnl_req_info *req_base,
    struct ethnl_reply_data *reply_base,
    const struct genl_info *info)
    {
    struct features_reply_data *data = FEATURES_REPDATA(reply_base);
    struct net_device *dev = reply_base.dev;
    netdev_features_t all_features;
    ethnl_features_to_bitmap32(data.hw, dev.hw_features);
    ethnl_features_to_bitmap32(data.wanted, dev.wanted_features);
    ethnl_features_to_bitmap32(data.active, dev.features);
    ethnl_features_to_bitmap32(data.nochange, NETIF_F_NEVER_CHANGE);
    all_features = GENMASK_ULL(NETDEV_FEATURE_COUNT - 1, 0);
    ethnl_features_to_bitmap32(data.all, all_features);
    return 0;
    }
    static int features_reply_size(const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct features_reply_data *data = FEATURES_REPDATA(reply_base);
    let mut compact: bool = req_base.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    let mut len: c_uint = 0;
    int ret;
    ret = ethnl_bitset32_size(data.hw, data.all, NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    len += ret;
    ret = ethnl_bitset32_size(data.wanted, core::ptr::null_mut(), NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    len += ret;
    ret = ethnl_bitset32_size(data.active, core::ptr::null_mut(), NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    len += ret;
    ret = ethnl_bitset32_size(data.nochange, core::ptr::null_mut(), NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    len += ret;
    return len;
    }
    static int features_fill_reply(struct sk_buff *skb,
    const struct ethnl_req_info *req_base,
    const struct ethnl_reply_data *reply_base)
    {
    const struct features_reply_data *data = FEATURES_REPDATA(reply_base);
    let mut compact: bool = req_base.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    int ret;
    ret = ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_HW, data.hw,
    data.all, NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    ret = ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_WANTED, data.wanted,
    core::ptr::null_mut(), NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    ret = ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_ACTIVE, data.active,
    core::ptr::null_mut(), NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    return ret;
    return ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_NOCHANGE,
    data.nochange, core::ptr::null_mut(), NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    }
    const struct ethnl_request_ops ethnl_features_request_ops = {
    .request_cmd		= ETHTOOL_MSG_FEATURES_GET,
    .reply_cmd		= ETHTOOL_MSG_FEATURES_GET_REPLY,
    .hdr_attr		= ETHTOOL_A_FEATURES_HEADER,
    .req_info_size		= sizeof(struct features_req_info),
    .reply_data_size	= sizeof(struct features_reply_data),
    .prepare_data		= features_prepare_data,
    .reply_size		= features_reply_size,
    .fill_reply		= features_fill_reply,
    };
// FEATURES_SET
    const struct nla_policy ethnl_features_set_policy[] = {
    [ETHTOOL_A_FEATURES_HEADER]	=
    NLA_POLICY_NESTED(ethnl_header_policy),
    [ETHTOOL_A_FEATURES_WANTED]	= { .type = NLA_NESTED },
    };
#[no_mangle]
unsafe extern "C" fn ethnl_features_to_bitmap(dest: *mut c_ulong, val: netdev_features_t) {
    static void ethnl_features_to_bitmap(unsigned long *dest, netdev_features_t val)
    {
    let mut words: c_uint = BITS_TO_LONGS(NETDEV_FEATURE_COUNT);
    unsigned int i;
    for (i = 0; i < words; i++)
    dest[i] = (unsigned long)(val >> (i * BITS_PER_LONG));
    }
#[no_mangle]
unsafe extern "C" fn ethnl_bitmap_to_features(src: *mut c_ulong) -> netdev_features_t {
    static netdev_features_t ethnl_bitmap_to_features(unsigned long *src)
    {
    let mut nft_bits: c_uint = sizeof(netdev_features_t) * BITS_PER_BYTE;
    let mut words: c_uint = BITS_TO_LONGS(NETDEV_FEATURE_COUNT);
    let mut ret: netdev_features_t = 0;
    unsigned int i;
    for (i = 0; i < words; i++)
    ret |= (netdev_features_t)(src[i]) << (i * BITS_PER_LONG);
    ret &= ~(netdev_features_t)0 >> (nft_bits - NETDEV_FEATURE_COUNT);
    return ret;
    }
    static int features_send_reply(struct net_device *dev, struct genl_info *info,
    const unsigned long *wanted,
    const unsigned long *wanted_mask,
    const unsigned long *active,
    const unsigned long *active_mask, bool compact)
    {
    struct sk_buff *rskb;
    void *reply_payload;
    let mut reply_len: c_int = 0;
    int ret;
    reply_len = ethnl_reply_header_size();
    ret = ethnl_bitset_size(wanted, wanted_mask, NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    goto err;
    reply_len += ret;
    ret = ethnl_bitset_size(active, active_mask, NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    goto err;
    reply_len += ret;
    ret = -ENOMEM;
    rskb = ethnl_reply_init(reply_len, dev, ETHTOOL_MSG_FEATURES_SET_REPLY,
    ETHTOOL_A_FEATURES_HEADER, info,
    &reply_payload);
    if (!rskb)
    goto err;
    ret = ethnl_put_bitset(rskb, ETHTOOL_A_FEATURES_WANTED, wanted,
    wanted_mask, NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    goto nla_put_failure;
    ret = ethnl_put_bitset(rskb, ETHTOOL_A_FEATURES_ACTIVE, active,
    active_mask, NETDEV_FEATURE_COUNT,
    netdev_features_strings, compact);
    if (ret < 0)
    goto nla_put_failure;
    genlmsg_end(rskb, reply_payload);
    ret = genlmsg_reply(rskb, info);
    return ret;
    nla_put_failure:
    nlmsg_free(rskb);
    WARN_ONCE(1, "calculated message payload length (%d) not sufficient\n",
    reply_len);
    err:
    GENL_SET_ERR_MSG(info, "failed to send reply message");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ethnl_set_features(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ethnl_set_features(struct sk_buff *skb, struct genl_info *info)
    {
    DECLARE_BITMAP(wanted_diff_mask, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(active_diff_mask, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(old_active, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(old_wanted, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(new_active, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(new_wanted, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(req_wanted, NETDEV_FEATURE_COUNT);
    DECLARE_BITMAP(req_mask, NETDEV_FEATURE_COUNT);
    let mut req_info: ethnl_req_info = {};
    struct nlattr **tb = info.attrs;
    struct net_device *dev;
    bool mod;
    int ret;
    if (!tb[ETHTOOL_A_FEATURES_WANTED])
    return -EINVAL;
    ret = ethnl_parse_header_dev_get(&req_info,
    tb[ETHTOOL_A_FEATURES_HEADER],
    genl_info_net(info), info.extack,
    true);
    if (ret < 0)
    return ret;
    dev = req_info.dev;
    rtnl_lock();
    netdev_lock_ops(dev);
    ret = ethnl_ops_begin(dev);
    if (ret < 0)
    goto out_unlock;
    ethnl_features_to_bitmap(old_active, dev.features);
    ethnl_features_to_bitmap(old_wanted, dev.wanted_features);
    ret = ethnl_parse_bitset(req_wanted, req_mask, NETDEV_FEATURE_COUNT,
    tb[ETHTOOL_A_FEATURES_WANTED],
    netdev_features_strings, info.extack);
    if (ret < 0)
    goto out_ops;
    if (ethnl_bitmap_to_features(req_mask) & ~NETIF_F_ETHTOOL_BITS) {
    GENL_SET_ERR_MSG(info, "attempt to change non-ethtool features");
    ret = -EINVAL;
    goto out_ops;
    }
// set req_wanted bits not in req_mask from old_wanted
    bitmap_and(req_wanted, req_wanted, req_mask, NETDEV_FEATURE_COUNT);
    bitmap_andnot(new_wanted, old_wanted, req_mask, NETDEV_FEATURE_COUNT);
    bitmap_or(req_wanted, new_wanted, req_wanted, NETDEV_FEATURE_COUNT);
    if (!bitmap_equal(req_wanted, old_wanted, NETDEV_FEATURE_COUNT)) {
    dev.wanted_features &= ~dev.hw_features;
    dev.wanted_features |= ethnl_bitmap_to_features(req_wanted) & dev.hw_features;
    __netdev_update_features(dev);
    }
    ethnl_features_to_bitmap(new_active, dev.features);
    mod = !bitmap_equal(old_active, new_active, NETDEV_FEATURE_COUNT);
    ret = 0;
    if (!(req_info.flags & ETHTOOL_FLAG_OMIT_REPLY)) {
    let mut compact: bool = req_info.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    bitmap_xor(wanted_diff_mask, req_wanted, new_active,
    NETDEV_FEATURE_COUNT);
    bitmap_xor(active_diff_mask, old_active, new_active,
    NETDEV_FEATURE_COUNT);
    bitmap_and(wanted_diff_mask, wanted_diff_mask, req_mask,
    NETDEV_FEATURE_COUNT);
    bitmap_and(req_wanted, req_wanted, wanted_diff_mask,
    NETDEV_FEATURE_COUNT);
    bitmap_and(new_active, new_active, active_diff_mask,
    NETDEV_FEATURE_COUNT);
    ret = features_send_reply(dev, info, req_wanted,
    wanted_diff_mask, new_active,
    active_diff_mask, compact);
    }
    if (mod)
    netdev_features_change(dev);
    out_ops:
    ethnl_ops_complete(dev);
    out_unlock:
    netdev_unlock_ops(dev);
    rtnl_unlock();
    ethnl_parse_header_dev_put(&req_info);
    return ret;
    }
