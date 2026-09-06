//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/wmi-ops.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ops {
    pub skb): *mut *mut *mut void (rx)(struct ath10k ar, struct sk_buff,
    pub len): *const *const *const *const void (map_svc)(__le32 in, unsigned long out, size_t,
    pub len): *const *const *const *const void (map_svc_ext)(__le32 in, unsigned long out, size_t,
    pub arg): *mut wmi_scan_ev_arg,
    pub arg): *mut wmi_mgmt_rx_ev_arg,
    pub arg): *mut wmi_tlv_mgmt_tx_compl_ev_arg,
    pub arg): *mut wmi_tlv_mgmt_tx_bundle_compl_ev_arg,
    pub arg): *mut wmi_ch_info_ev_arg,
    pub arg): *mut wmi_vdev_start_ev_arg,
    pub arg): *mut wmi_peer_kick_ev_arg,
    pub arg): *mut wmi_swba_ev_arg,
    pub arg): *mut wmi_phyerr_hdr_arg,
    pub arg): *mut int left_len, struct wmi_phyerr_ev_arg,
    pub arg): *mut wmi_svc_rdy_ev_arg,
    pub arg): *mut wmi_rdy_ev_arg,
    pub stats): *mut ath10k_fw_stats,
    pub arg): *mut wmi_roam_ev_arg,
    pub arg): *mut wmi_wow_ev_arg,
    pub arg): *mut wmi_echo_ev_arg,
    pub arg): *mut wmi_dfs_status_ev_arg,
    pub arg): *mut wmi_svc_avail_ev_arg,
    pub ar): *mut *mut wmi_txbf_conf (get_txbf_conf_scheme)(struct ath10k,
    pub suspend_opt): *mut *mut *mut *mut sk_buff (gen_pdev_suspend)(ath10k ar, u32,
    pub ar): *mut *mut *mut sk_buff (gen_pdev_resume)(ath10k,
    pub macaddr[ETH_ALEN]): u8,
    pub dfs_reg): wmi_dfs_region,
    pub value): u32,
    pub ar): *mut *mut *mut sk_buff (gen_init)(ath10k,
    pub arg): *const wmi_start_scan_arg,
    pub arg): *const wmi_stop_scan_arg,
    pub macaddr[ETH_ALEN]): u8,
    pub vdev_id): *mut *mut *mut *mut sk_buff (gen_vdev_delete)(ath10k ar, u32,
    pub restart): bool,
    pub vdev_id): *mut *mut *mut *mut sk_buff (gen_vdev_stop)(ath10k ar, u32,
    pub bssid): *const u8,
    pub vdev_id): *mut *mut *mut *mut sk_buff (gen_vdev_down)(ath10k ar, u32,
    pub param_value): u32 param_id, u32,
    pub arg): *const wmi_vdev_install_key_arg,
    pub arg): *const wmi_vdev_spectral_conf_arg,
    pub enable): u32 trigger, u32,
    pub arg): *const wmi_wmm_params_all_arg,
    pub peer_type): wmi_peer_type,
    pub peer_addr[ETH_ALEN]): u8,
    pub tid_bitmap): u32,
    pub param_value): u32,
    pub arg): *const wmi_peer_assoc_complete_arg,
    pub psmode): wmi_sta_ps_mode,
    pub value): u32,
    pub value): u32,
    pub arg): *const wmi_scan_chan_list_arg,
    pub prob_req_oui): u32,
    pub deliver_cab): bool,
    pub arg): *const wmi_wmm_params_all_arg,
    pub stats_mask): *mut *mut *mut *mut sk_buff (gen_request_stats)(ath10k ar, u32,
    pub reset): u32,
    pub delay_ms): u32,
    pub skb): *mut *mut *mut *mut sk_buff (gen_mgmt_tx)(ath10k ar, sk_buff,
    pub paddr): dma_addr_t,
    pub msdu): *mut *mut *mut int (cleanup_mgmt_tx_send)(struct ath10k ar, struct sk_buff,
    pub log_level): u32,
    pub filter): *mut *mut *mut *mut sk_buff (gen_pktlog_enable)(ath10k ar, u32,
    pub ar): *mut *mut *mut sk_buff (gen_pktlog_disable)(ath10k,
    pub enabled): u32,
    pub ar): *mut *mut *mut sk_buff (gen_pdev_get_temperature)(ath10k,
    pub mac): *const u8,
    pub buf_size): *const *const u8 mac, u32 tid, u32,
    pub status): u32,
    pub reason): u32,
    pub prb_ies_len): *mut *mut void prb_ies, size_t,
    pub bcn): *mut sk_buff,
    pub p2p_ie): *const u8,
    pub num_ac): u32,
    pub arg): *const wmi_sta_keepalive_arg,
    pub ar): *mut *mut *mut sk_buff (gen_wow_enable)(ath10k,
    pub enable): u32,
    pub ar): *mut *mut *mut sk_buff (gen_wow_host_wakeup_ind)(ath10k,
    pub pattern_offset): c_int,
    pub pattern_id): u32,
    pub state): wmi_tdls_state,
    pub chan): *const wmi_channel_arg,
    pub arg): *const ath10k_radar_found_info,
    pub enable): *mut *mut *mut *mut sk_buff (gen_adaptive_qcs)(ath10k ar, bool,
    pub param): u32,
    pub buf): *mut c_char,
    pub detect_margin): u32,
    pub fw_feature_bitmap): u32,
    pub subtype): wmi_vdev_subtype,
    pub pno_scan): *mut wmi_pno_scan_req,
    pub type): wmi_bss_survey_req_type,
    pub value): *mut *mut *mut *mut sk_buff (gen_echo)(ath10k ar, u32,
    pub param): u32,
    pub arg): *const wmi_bb_timing_cfg_arg,
    pub arg): *const wmi_per_peer_per_tid_cfg_arg,
    pub intr_mode): u32 input, u32 pull_type, u32,
    pub set): *mut *mut *mut *mut sk_buff (gen_gpio_output)(ath10k ar, u32 gpio_num, u32,
}

extern "C" {
    pub fn ath10k_wmi_cmd_send(ar: *mut ath10k, skb: *mut sk_buff, cmd_id: u32) -> c_int;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
// FIXME There's no ACK event for Management Tx. This probably
// shouldn't be called here either.
//
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->pdev_suspend_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->pdev_resume_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->pdev_set_param_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->init_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->start_scan_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->stop_scan_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->vdev_create_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->vdev_delete_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->vdev_stop_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->vdev_up_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->vdev_down_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->vdev_set_param_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->peer_create_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->peer_delete_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->peer_flush_tids_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->peer_set_param_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->scan_chan_list_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->peer_assoc_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->request_stats_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->request_peer_stats_info_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->force_fw_hang_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->gpio_config_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->gpio_output_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->dbglog_cfg_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->pdev_pktlog_enable_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->bcn_tmpl_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->prb_tmpl_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->p2p_go_set_beacon_ie) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: cmd_id) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->tdls_set_state_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: ar->wmi.cmd->adaptive_qcs_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn ath10k_wmi_cmd_send(_arg: ar, _arg: skb, _arg: wmi->cmd->echo_cmdid) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: skb) -> return;
}
