//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/binding.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2012-2014, 2020 Intel Corporation
// Copyright (C) 2016 Intel Deutschland GmbH
// Copyright (C) 2022, 2024, 2026 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_iface_iterator_data {
    pub ignore_vif: *mut ieee80211_vif,
    pub idx: c_int,
    pub phyctxt: *mut iwl_mvm_phy_ctxt,
    pub ids: [u16; MAX_MACS_IN_BINDING],
    pub colors: [u16; MAX_MACS_IN_BINDING],
}

    static int iwl_mvm_binding_cmd(struct iwl_mvm *mvm, u32 action,
    struct iwl_mvm_iface_iterator_data *data)
    {
    struct iwl_binding_cmd cmd;
    struct iwl_mvm_phy_ctxt *phyctxt = data.phyctxt;
    int i, ret;
    u32 status;
    int size;
    memset(&cmd, 0, sizeof(cmd));
    if (fw_has_capa(&mvm.fw.ucode_capa,
    IWL_UCODE_TLV_CAPA_BINDING_CDB_SUPPORT)) {
    size = sizeof(cmd);
    cmd.lmac_id = cpu_to_le32(iwl_mvm_get_lmac_id(mvm,
    phyctxt.channel.band));
    } else {
    size = IWL_BINDING_CMD_SIZE_V1;
    }
    cmd.id_and_color = cpu_to_le32(FW_CMD_ID_AND_COLOR(phyctxt.id,
    phyctxt.color));
    cmd.action = cpu_to_le32(action);
    cmd.phy = cpu_to_le32(FW_CMD_ID_AND_COLOR(phyctxt.id,
    phyctxt.color));
    for (i = 0; i < MAX_MACS_IN_BINDING; i++)
    cmd.macs[i] = cpu_to_le32(FW_CTXT_INVALID);
    for (i = 0; i < data.idx; i++)
    cmd.macs[i] = cpu_to_le32(FW_CMD_ID_AND_COLOR(data.ids[i],
    data.colors[i]));
    status = 0;
    ret = iwl_mvm_send_cmd_pdu_status(mvm, BINDING_CONTEXT_CMD,
    size, &cmd, &status);
    if (ret) {
    IWL_ERR(mvm, "Failed to send binding (action:%d): %d\n",
    action, ret);
    return ret;
    }
    if (status) {
    IWL_ERR(mvm, "Binding command failed: %u\n", status);
    ret = -EIO;
    }
    return ret;
    }
    static void iwl_mvm_iface_iterator(void *_data, u8 *mac,
    struct ieee80211_vif *vif)
    {
    struct iwl_mvm_iface_iterator_data *data = _data;
    struct iwl_mvm_vif *mvmvif = iwl_mvm_vif_from_mac80211(vif);
    if (vif == data.ignore_vif)
    return;
    if (vif.type == NL80211_IFTYPE_P2P_DEVICE && !mvmvif.p2p_in_binding)
    return;
    if (mvmvif.deflink.phy_ctxt != data.phyctxt)
    return;
    if (WARN_ON_ONCE(data.idx >= MAX_MACS_IN_BINDING))
    return;
    data.ids[data.idx] = mvmvif.id;
    data.colors[data.idx] = mvmvif.color;
    data.idx++;
    }
    static int iwl_mvm_binding_update(struct iwl_mvm *mvm,
    struct ieee80211_vif *vif,
    struct iwl_mvm_phy_ctxt *phyctxt,
    bool add)
    {
    struct iwl_mvm_vif *mvmvif = iwl_mvm_vif_from_mac80211(vif);
    struct iwl_mvm_iface_iterator_data data = {
    .ignore_vif = vif,
    .phyctxt = phyctxt,
    };
    let mut action: u32 = FW_CTXT_ACTION_MODIFY;
    lockdep_assert_held(&mvm.mutex);
    ieee80211_iterate_active_interfaces_atomic(mvm.hw,
    IEEE80211_IFACE_ITER_NORMAL,
    iwl_mvm_iface_iterator,
    &data);
//
// If there are no other interfaces yet we
// need to create a new binding.
//
    if (data.idx == 0) {
    if (add)
    action = FW_CTXT_ACTION_ADD;
    else
    action = FW_CTXT_ACTION_REMOVE;
    }
    if (add) {
    if (WARN_ON_ONCE(data.idx >= MAX_MACS_IN_BINDING))
    return -EINVAL;
    data.ids[data.idx] = mvmvif.id;
    data.colors[data.idx] = mvmvif.color;
    data.idx++;
    }
    return iwl_mvm_binding_cmd(mvm, action, &data);
    }
#[no_mangle]
pub unsafe extern "C" fn iwl_mvm_binding_add_vif(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int {
    int iwl_mvm_binding_add_vif(struct iwl_mvm *mvm, struct ieee80211_vif *vif)
    {
    struct iwl_mvm_vif *mvmvif = iwl_mvm_vif_from_mac80211(vif);
    if (WARN_ON_ONCE(!mvmvif.deflink.phy_ctxt))
    return -EINVAL;
//
// Update SF - Disable if needed. if this fails, SF might still be on
// while many macs are bound, which is forbidden - so fail the binding.
//
    if (iwl_mvm_sf_update(mvm, vif, false))
    return -EINVAL;
    return iwl_mvm_binding_update(mvm, vif, mvmvif.deflink.phy_ctxt,
    true);
    }
#[no_mangle]
pub unsafe extern "C" fn iwl_mvm_binding_remove_vif(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int {
    int iwl_mvm_binding_remove_vif(struct iwl_mvm *mvm, struct ieee80211_vif *vif)
    {
    struct iwl_mvm_vif *mvmvif = iwl_mvm_vif_from_mac80211(vif);
    int ret;
    if (WARN_ON_ONCE(!mvmvif.deflink.phy_ctxt))
    return -EINVAL;
    ret = iwl_mvm_binding_update(mvm, vif, mvmvif.deflink.phy_ctxt,
    false);
    if (!ret && iwl_mvm_sf_update(mvm, vif, true))
    IWL_ERR(mvm, "Failed to update SF state\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn iwl_mvm_get_lmac_id(mvm: *mut iwl_mvm, band: enum nl80211_band) -> u32 {
    u32 iwl_mvm_get_lmac_id(struct iwl_mvm *mvm, enum nl80211_band band)
    {
    if (!fw_has_capa(&mvm.fw.ucode_capa, IWL_UCODE_TLV_CAPA_CDB_SUPPORT) ||
    band == NL80211_BAND_2GHZ)
    return IWL_LMAC_24G_INDEX;
    return IWL_LMAC_5G_INDEX;
    }
