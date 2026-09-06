//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/hw.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// Target configuration defines
// Num VDEVS per radio

// Num of peers for Single Radio mode

// Num of peers for DBS

// Num of peers for DBS_SBS

// Max num of stations (per radio)

pub const TARGET_NUM_PEER_KEYS: c_int = 2;

pub const TARGET_AST_SKID_LIMIT: c_int = 16;
pub const TARGET_NUM_OFFLD_PEERS: c_int = 4;
pub const TARGET_NUM_OFFLD_REORDER_BUFFS: c_int = 4;

pub const TARGET_RX_TIMEOUT_LO_PRI: c_int = 100;
pub const TARGET_RX_TIMEOUT_HI_PRI: c_int = 40;
pub const TARGET_DECAP_MODE_RAW: c_int = 0;
pub const TARGET_DECAP_MODE_NATIVE_WIFI: c_int = 1;
pub const TARGET_DECAP_MODE_ETH: c_int = 2;
pub const TARGET_SCAN_MAX_PENDING_REQS: c_int = 4;
pub const TARGET_BMISS_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_ROAM_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_ROAM_OFFLOAD_MAX_AP_PROFILES: c_int = 8;
pub const TARGET_GTK_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_NUM_MCAST_GROUPS: c_int = 12;
pub const TARGET_NUM_MCAST_TABLE_ELEMS: c_int = 64;
pub const TARGET_MCAST2UCAST_MODE: c_int = 2;
pub const TARGET_TX_DBG_LOG_SIZE: c_int = 1024;
pub const TARGET_RX_SKIP_DEFRAG_TIMEOUT_DUP_DETECTION_CHECK: c_int = 1;
pub const TARGET_VOW_CONFIG: c_int = 0;

pub const TARGET_MAX_FRAG_ENTRIES: c_int = 6;
pub const TARGET_MAX_BCN_OFFLD: c_int = 16;
pub const TARGET_NUM_WDS_ENTRIES: c_int = 32;
pub const TARGET_DMA_BURST_SIZE: c_int = 1;
pub const TARGET_RX_BATCHMODE: c_int = 1;
pub const TARGET_EMA_MAX_PROFILE_PERIOD: c_int = 8;
pub const ATH11K_HW_MAX_QUEUES: c_int = 4;
pub const ATH11K_QUEUE_LEN: c_int = 4096;
pub const ATH11k_HW_RATECODE_CCK_SHORT_PREAM_MASK: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_hw_rate_cck {
    ATH11K_HW_RATE_CCK_LP_11M = 0,
    ATH11K_HW_RATE_CCK_LP_5_5M,
    ATH11K_HW_RATE_CCK_LP_2M,
    ATH11K_HW_RATE_CCK_LP_1M,
    ATH11K_HW_RATE_CCK_SP_11M,
    ATH11K_HW_RATE_CCK_SP_5_5M,
    ATH11K_HW_RATE_CCK_SP_2M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_hw_rate_ofdm {
    ATH11K_HW_RATE_OFDM_48M = 0,
    ATH11K_HW_RATE_OFDM_24M,
    ATH11K_HW_RATE_OFDM_12M,
    ATH11K_HW_RATE_OFDM_6M,
    ATH11K_HW_RATE_OFDM_54M,
    ATH11K_HW_RATE_OFDM_36M,
    ATH11K_HW_RATE_OFDM_18M,
    ATH11K_HW_RATE_OFDM_9M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_bus {
    ATH11K_BUS_AHB,
    ATH11K_BUS_PCI,
}

pub const ATH11K_EXT_IRQ_GRP_NUM_MAX: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hw_ring_mask {
    pub tx: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_mon_status: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub rx: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_err: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_wbm_rel: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub reo_status: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub rxdma2host: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub host2rxdma: [u8; ATH11K_EXT_IRQ_GRP_NUM_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hw_tcl2wbm_rbm_map {
    pub tcl_ring_num: u8,
    pub wbm_ring_num: u8,
    pub rbm_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hw_hal_params {
    pub rx_buf_rbm: hal_rx_buf_return_buf_manager,
    pub tcl2wbm_rbm_map: *const ath11k_hw_tcl2wbm_rbm_map,
    pub num_tx_rings: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hw_params {
    pub name: *const c_char,
    pub hw_rev: u16,
    pub max_radios: u8,
    pub bdf_addr: u32,
    pub dir: *const c_char,
    pub board_size: usize,
    pub cal_offset: usize,
    pub fw: },
    pub hw_ops: *const ath11k_hw_ops,
    pub ring_mask: *const ath11k_hw_ring_mask,
    pub internal_sleep_clock: bool,
    pub regs: *const ath11k_hw_regs,
    pub qmi_service_ins_id: u32,
    pub host_ce_config: *const ce_attr,
    pub ce_count: u32,
    pub target_ce_config: *const ce_pipe_config,
    pub target_ce_count: u32,
    pub svc_to_ce_map: *const service_to_pipe,
    pub svc_to_ce_map_len: u32,
    pub ce_ie_addr: *const ce_ie_addr,
    pub ce_remap: *const ce_remap,
    pub single_pdev_only: bool,
    pub rxdma1_enable: bool,
    pub num_rxdma_per_pdev: c_int,
    pub rx_mac_buf_ring: bool,
    pub vdev_start_delay: bool,
    pub htt_peer_map_v2: bool,
    pub fft_sz: u8,
    pub fft_pad_sz: u8,
    pub summary_pad_sz: u8,
    pub fft_hdr_len: u8,
    pub max_fft_bins: u16,
    pub fragment_160mhz: bool,
    pub spectral: },
    pub interface_modes: u16,
    pub supports_monitor: bool,
    pub full_monitor_mode: bool,
    pub supports_shadow_regs: bool,
    pub idle_ps: bool,
    pub supports_sta_ps: bool,
    pub coldboot_cal_mm: bool,
    pub coldboot_cal_ftm: bool,
    pub cbcal_restart_fw: bool,
    pub fw_mem_mode: c_int,
    pub num_vdevs: u32,
    pub num_peers: u32,
    pub supports_suspend: bool,
    pub hal_desc_sz: u32,
    pub supports_regdb: bool,
    pub fix_l1ss: bool,
    pub credit_flow: bool,
    pub hal_params: *const ath11k_hw_hal_params,
    pub supports_dynamic_smps_6ghz: bool,
    pub alloc_cacheable_memory: bool,
    pub supports_rssi_stats: bool,
    pub fw_wmi_diag_event: bool,
    pub current_cc_support: bool,
    pub dbr_debug_support: bool,
    pub global_reset: bool,
    pub bios_sar_capa: *const cfg80211_sar_capa,
    pub m3_fw_support: bool,
    pub fixed_bdf_addr: bool,
    pub fixed_mem_region: bool,
    pub static_window_map: bool,
    pub hybrid_bus_type: bool,
    pub fixed_fw_mem: bool,
    pub support_off_channel_tx: bool,
    pub supports_multi_bssid: bool,
    pub start: u32,
    pub end: u32,
    pub sram_dump: },
    pub tcl_ring_retry: bool,
    pub tx_ring_size: u32,
    pub smp2p_wow_exit: bool,
    pub support_fw_mac_sequence: bool,
    pub support_dual_stations: bool,
    pub pdev_suspend: bool,
    pub cfr_support: bool,
    pub cfr_num_stream_bufs: u32,
    pub cfr_stream_buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hw_ops {
    pub pdev_id): *mut *mut u8 (get_hw_mac_from_pdev_id)(int,
    pub config): *mut target_resource_config,
    pub mac_id): *mut *mut *mut int (mac_id_to_pdev_id)(struct ath11k_hw_params hw, int,
    pub mac_id): *mut *mut *mut int (mac_id_to_srng_id)(struct ath11k_hw_params hw, int,
    pub tcl_cmd): *mut hal_tcl_data_cmd,
    pub desc): *mut *mut bool (rx_desc_get_first_msdu)(struct hal_rx_desc,
    pub desc): *mut *mut bool (rx_desc_get_last_msdu)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_l3_pad_bytes)(struct hal_rx_desc,
    pub desc): *mut *mut *mut u8 (rx_desc_get_hdr_status)(struct hal_rx_desc,
    pub desc): *mut *mut bool (rx_desc_encrypt_valid)(struct hal_rx_desc,
    pub desc): *mut *mut u32 (rx_desc_get_encrypt_type)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_decap_type)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_mesh_ctl)(struct hal_rx_desc,
    pub desc): *mut *mut bool (rx_desc_get_ldpc_support)(struct hal_rx_desc,
    pub desc): *mut *mut bool (rx_desc_get_mpdu_seq_ctl_vld)(struct hal_rx_desc,
    pub desc): *mut *mut bool (rx_desc_get_mpdu_fc_valid)(struct hal_rx_desc,
    pub desc): *mut *mut u16 (rx_desc_get_mpdu_start_seq_no)(struct hal_rx_desc,
    pub desc): *mut *mut u16 (rx_desc_get_msdu_len)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_msdu_sgi)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_msdu_rate_mcs)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_msdu_rx_bw)(struct hal_rx_desc,
    pub desc): *mut *mut u32 (rx_desc_get_msdu_freq)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_msdu_pkt_type)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_msdu_nss)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_mpdu_tid)(struct hal_rx_desc,
    pub desc): *mut *mut u16 (rx_desc_get_mpdu_peer_id)(struct hal_rx_desc,
    pub ldesc): *mut hal_rx_desc,
    pub desc): *mut *mut u32 (rx_desc_get_mpdu_start_tag)(struct hal_rx_desc,
    pub desc): *mut *mut u32 (rx_desc_get_mpdu_ppdu_id)(struct hal_rx_desc,
    pub len): *mut *mut *mut void (rx_desc_set_msdu_len)(struct hal_rx_desc desc, u16,
    pub desc): *mut *mut *mut rx_attention (rx_desc_get_attention)(hal_rx_desc,
    pub desc): *mut *mut *mut u8 (rx_desc_get_msdu_payload)(struct hal_rx_desc,
    pub ab): *mut *mut void (reo_setup)(struct ath11k_base,
    pub mpdu_info): *mut *mut u16 (mpdu_info_get_peerid)(struct hal_rx_mpdu_info,
    pub desc): *mut *mut bool (rx_desc_mac_addr2_valid)(struct hal_rx_desc,
    pub desc): *mut *mut *mut u8 (rx_desc_mpdu_start_addr2)(struct hal_rx_desc,
    pub skb): *mut *mut u32 (get_ring_selector)(struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_fw_ie {
    pub id: __le32,
    pub len: __le32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_bd_ie_board_type {
    ATH11K_BD_IE_BOARD_NAME = 0,
    ATH11K_BD_IE_BOARD_DATA = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_bd_ie_regdb_type {
    ATH11K_BD_IE_REGDB_NAME = 0,
    ATH11K_BD_IE_REGDB_DATA = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_bd_ie_type {
// contains sub IEs of enum ath11k_bd_ie_board_type
    ATH11K_BD_IE_BOARD = 0,
// contains sub IEs of enum ath11k_bd_ie_regdb_type
    ATH11K_BD_IE_REGDB = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hw_regs {
    pub hal_tcl1_ring_base_lsb: u32,
    pub hal_tcl1_ring_base_msb: u32,
    pub hal_tcl1_ring_id: u32,
    pub hal_tcl1_ring_misc: u32,
    pub hal_tcl1_ring_tp_addr_lsb: u32,
    pub hal_tcl1_ring_tp_addr_msb: u32,
    pub hal_tcl1_ring_consumer_int_setup_ix0: u32,
    pub hal_tcl1_ring_consumer_int_setup_ix1: u32,
    pub hal_tcl1_ring_msi1_base_lsb: u32,
    pub hal_tcl1_ring_msi1_base_msb: u32,
    pub hal_tcl1_ring_msi1_data: u32,
    pub hal_tcl2_ring_base_lsb: u32,
    pub hal_tcl_ring_base_lsb: u32,
    pub hal_tcl_status_ring_base_lsb: u32,
    pub hal_reo1_ring_base_lsb: u32,
    pub hal_reo1_ring_base_msb: u32,
    pub hal_reo1_ring_id: u32,
    pub hal_reo1_ring_misc: u32,
    pub hal_reo1_ring_hp_addr_lsb: u32,
    pub hal_reo1_ring_hp_addr_msb: u32,
    pub hal_reo1_ring_producer_int_setup: u32,
    pub hal_reo1_ring_msi1_base_lsb: u32,
    pub hal_reo1_ring_msi1_base_msb: u32,
    pub hal_reo1_ring_msi1_data: u32,
    pub hal_reo2_ring_base_lsb: u32,
    pub hal_reo1_aging_thresh_ix_0: u32,
    pub hal_reo1_aging_thresh_ix_1: u32,
    pub hal_reo1_aging_thresh_ix_2: u32,
    pub hal_reo1_aging_thresh_ix_3: u32,
    pub hal_reo1_ring_hp: u32,
    pub hal_reo1_ring_tp: u32,
    pub hal_reo2_ring_hp: u32,
    pub hal_reo_tcl_ring_base_lsb: u32,
    pub hal_reo_tcl_ring_hp: u32,
    pub hal_reo_status_ring_base_lsb: u32,
    pub hal_reo_status_hp: u32,
    pub hal_reo_cmd_ring_base_lsb: u32,
    pub hal_reo_cmd_ring_hp: u32,
    pub hal_sw2reo_ring_base_lsb: u32,
    pub hal_sw2reo_ring_hp: u32,
    pub hal_seq_wcss_umac_ce0_src_reg: u32,
    pub hal_seq_wcss_umac_ce0_dst_reg: u32,
    pub hal_seq_wcss_umac_ce1_src_reg: u32,
    pub hal_seq_wcss_umac_ce1_dst_reg: u32,
    pub hal_wbm_idle_link_ring_base_lsb: u32,
    pub hal_wbm_idle_link_ring_misc: u32,
    pub hal_wbm_release_ring_base_lsb: u32,
    pub hal_wbm0_release_ring_base_lsb: u32,
    pub hal_wbm1_release_ring_base_lsb: u32,
    pub pcie_qserdes_sysclk_en_sel: u32,
    pub pcie_pcs_osc_dtct_config_base: u32,
    pub hal_shadow_base_addr: u32,
    pub hal_reo1_misc_ctl: u32,
}
