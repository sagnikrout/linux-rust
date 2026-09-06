//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/vendor-cmd.c
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
// Copyright (C) 2021 Intel Corporation
//

    static const struct nla_policy
    iwl_mvm_vendor_attr_policy[NUM_IWL_MVM_VENDOR_ATTR] = {
    [IWL_MVM_VENDOR_ATTR_ROAMING_FORBIDDEN] = { .type = NLA_U8 },
    [IWL_MVM_VENDOR_ATTR_AUTH_MODE] = { .type = NLA_U32 },
    [IWL_MVM_VENDOR_ATTR_CHANNEL_NUM] = { .type = NLA_U8 },
    [IWL_MVM_VENDOR_ATTR_SSID] = { .type = NLA_BINARY,
    .len = IEEE80211_MAX_SSID_LEN },
    [IWL_MVM_VENDOR_ATTR_BAND] = { .type = NLA_U8 },
    [IWL_MVM_VENDOR_ATTR_COLLOC_CHANNEL] = { .type = NLA_U8 },
    [IWL_MVM_VENDOR_ATTR_COLLOC_ADDR] = { .type = NLA_BINARY, .len = ETH_ALEN },
    };
    static int iwl_mvm_vendor_get_csme_conn_info(struct wiphy *wiphy,
    struct wireless_dev *wdev,
    const void *data, int data_len)
    {
    struct ieee80211_hw *hw = wiphy_to_ieee80211_hw(wiphy);
    struct iwl_mvm *mvm = IWL_MAC80211_GET_MVM(hw);
    struct iwl_mvm_csme_conn_info *csme_conn_info;
    struct sk_buff *skb;
    let mut err: c_int = 0;
    mutex_lock(&mvm.mutex);
    csme_conn_info = iwl_mvm_get_csme_conn_info(mvm);
    if (!csme_conn_info) {
    err = -EINVAL;
    goto out_unlock;
    }
    skb = cfg80211_vendor_cmd_alloc_reply_skb(wiphy, 200);
    if (!skb) {
    err = -ENOMEM;
    goto out_unlock;
    }
    if (nla_put_u32(skb, IWL_MVM_VENDOR_ATTR_AUTH_MODE,
    csme_conn_info.conn_info.auth_mode) ||
    nla_put(skb, IWL_MVM_VENDOR_ATTR_SSID,
    csme_conn_info.conn_info.ssid_len,
    csme_conn_info.conn_info.ssid) ||
    nla_put_u32(skb, IWL_MVM_VENDOR_ATTR_STA_CIPHER,
    csme_conn_info.conn_info.pairwise_cipher) ||
    nla_put_u8(skb, IWL_MVM_VENDOR_ATTR_CHANNEL_NUM,
    csme_conn_info.conn_info.channel) ||
    nla_put(skb, IWL_MVM_VENDOR_ATTR_ADDR, ETH_ALEN,
    csme_conn_info.conn_info.bssid)) {
    kfree_skb(skb);
    err = -ENOBUFS;
    }
    out_unlock:
    mutex_unlock(&mvm.mutex);
    if (err)
    return err;
    return cfg80211_vendor_cmd_reply(skb);
    }
    static int iwl_mvm_vendor_host_get_ownership(struct wiphy *wiphy,
    struct wireless_dev *wdev,
    const void *data, int data_len)
    {
    struct ieee80211_hw *hw = wiphy_to_ieee80211_hw(wiphy);
    struct iwl_mvm *mvm = IWL_MAC80211_GET_MVM(hw);
    int ret;
    mutex_lock(&mvm.mutex);
    ret = iwl_mvm_mei_get_ownership(mvm);
    mutex_unlock(&mvm.mutex);
    return ret;
    }
    static const struct wiphy_vendor_command iwl_mvm_vendor_commands[] = {
    {
    .info = {
    .vendor_id = INTEL_OUI,
    .subcmd = IWL_MVM_VENDOR_CMD_GET_CSME_CONN_INFO,
    },
    .doit = iwl_mvm_vendor_get_csme_conn_info,
    .flags = WIPHY_VENDOR_CMD_NEED_WDEV,
    .policy = iwl_mvm_vendor_attr_policy,
    .maxattr = MAX_IWL_MVM_VENDOR_ATTR,
    },
    {
    .info = {
    .vendor_id = INTEL_OUI,
    .subcmd = IWL_MVM_VENDOR_CMD_HOST_GET_OWNERSHIP,
    },
    .doit = iwl_mvm_vendor_host_get_ownership,
    .flags = WIPHY_VENDOR_CMD_NEED_WDEV,
    .policy = iwl_mvm_vendor_attr_policy,
    .maxattr = MAX_IWL_MVM_VENDOR_ATTR,
    },
    };
    enum iwl_mvm_vendor_events_idx {
// 0x0 - 0x3 are deprecated
    IWL_MVM_VENDOR_EVENT_IDX_ROAMING_FORBIDDEN = 4,
    NUM_IWL_MVM_VENDOR_EVENT_IDX
    };
    static const struct nl80211_vendor_cmd_info
    iwl_mvm_vendor_events[NUM_IWL_MVM_VENDOR_EVENT_IDX] = {
    [IWL_MVM_VENDOR_EVENT_IDX_ROAMING_FORBIDDEN] = {
    .vendor_id = INTEL_OUI,
    .subcmd = IWL_MVM_VENDOR_CMD_ROAMING_FORBIDDEN_EVENT,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn iwl_mvm_vendor_cmds_register(mvm: *mut iwl_mvm) {
    void iwl_mvm_vendor_cmds_register(struct iwl_mvm *mvm)
    {
    mvm.hw.wiphy.vendor_commands = iwl_mvm_vendor_commands;
    mvm.hw.wiphy.n_vendor_commands = ARRAY_SIZE(iwl_mvm_vendor_commands);
    mvm.hw.wiphy.vendor_events = iwl_mvm_vendor_events;
    mvm.hw.wiphy.n_vendor_events = ARRAY_SIZE(iwl_mvm_vendor_events);
    }
    void iwl_mvm_send_roaming_forbidden_event(struct iwl_mvm *mvm,
    struct ieee80211_vif *vif,
    bool forbidden)
    {
    struct sk_buff *msg =
    cfg80211_vendor_event_alloc(mvm.hw.wiphy,
    ieee80211_vif_to_wdev(vif),
    200, IWL_MVM_VENDOR_EVENT_IDX_ROAMING_FORBIDDEN,
    GFP_ATOMIC);
    if (!msg)
    return;
    if (WARN_ON(!vif))
    return;
    if (nla_put(msg, IWL_MVM_VENDOR_ATTR_VIF_ADDR,
    ETH_ALEN, vif.addr) ||
    nla_put_u8(msg, IWL_MVM_VENDOR_ATTR_ROAMING_FORBIDDEN, forbidden))
    goto nla_put_failure;
    cfg80211_vendor_event(msg, GFP_ATOMIC);
    return;
    nla_put_failure:
    kfree_skb(msg);
    }
