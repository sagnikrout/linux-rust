//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/hw.h
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
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// Target configuration defines
// Num VDEVS per radio

// Max num of stations for Single Radio mode

// Max num of stations for DBS

// Max num of stations for DBS_SBS

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
pub const ATH12K_HW_DEFAULT_QUEUE: c_int = 0;
pub const ATH12K_HW_MAX_QUEUES: c_int = 4;
pub const ATH12K_QUEUE_LEN: c_int = 4096;
pub const ATH12K_HW_RATECODE_CCK_SHORT_PREAM_MASK: c_uint = 0x4;

pub const ATH12K_PCIE_MAX_PAYLOAD_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_hw_rate_cck {
    ATH12K_HW_RATE_CCK_LP_11M = 0,
    ATH12K_HW_RATE_CCK_LP_5_5M,
    ATH12K_HW_RATE_CCK_LP_2M,
    ATH12K_HW_RATE_CCK_LP_1M,
    ATH12K_HW_RATE_CCK_SP_11M,
    ATH12K_HW_RATE_CCK_SP_5_5M,
    ATH12K_HW_RATE_CCK_SP_2M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_hw_rate_ofdm {
    ATH12K_HW_RATE_OFDM_48M = 0,
    ATH12K_HW_RATE_OFDM_24M,
    ATH12K_HW_RATE_OFDM_12M,
    ATH12K_HW_RATE_OFDM_6M,
    ATH12K_HW_RATE_OFDM_54M,
    ATH12K_HW_RATE_OFDM_36M,
    ATH12K_HW_RATE_OFDM_18M,
    ATH12K_HW_RATE_OFDM_9M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_bus {
    ATH12K_BUS_PCI,
    ATH12K_BUS_AHB,
}

pub const ATH12K_EXT_IRQ_GRP_NUM_MAX: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_ring_mask {
    pub tx: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_mon_dest: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_mon_status: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub rx: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_err: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub rx_wbm_rel: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub reo_status: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub host2rxdma: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub tx_mon_dest: [u8; ATH12K_EXT_IRQ_GRP_NUM_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_m3_fw_loaders {
    ath12k_m3_fw_loader_driver,
    ath12k_m3_fw_loader_remoteproc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_params {
    pub name: *const c_char,
    pub hw_rev: u16,
    pub dir: *const c_char,
    pub board_size: usize,
    pub cal_offset: usize,
    pub m3_loader: ath12k_m3_fw_loaders,
    pub download_aux_ucode:1: bool,
    pub fw: },
    pub max_radios: u8,
    pub single_pdev_only:1: bool,
    pub qmi_service_ins_id: u32,
    pub internal_sleep_clock:1: bool,
    pub hw_ops: *const ath12k_hw_ops,
    pub ring_mask: *const ath12k_hw_ring_mask,
    pub host_ce_config: *const ce_attr,
    pub ce_count: u32,
    pub target_ce_config: *const ce_pipe_config,
    pub target_ce_count: u32,
    pub svc_to_ce_map: *const service_to_pipe,
    pub svc_to_ce_map_len: u32,
    pub rxdma1_enable:1: bool,
    pub num_rxdma_per_pdev: c_int,
    pub num_rxdma_dst_ring: c_int,
    pub rx_mac_buf_ring:1: bool,
    pub vdev_start_delay:1: bool,
    pub interface_modes: u16,
    pub supports_monitor:1: bool,
    pub idle_ps:1: bool,
    pub download_calib:1: bool,
    pub supports_suspend:1: bool,
    pub tcl_ring_retry:1: bool,
    pub reoq_lut_support:1: bool,
    pub supports_shadow_regs:1: bool,
    pub supports_aspm:1: bool,
    pub current_cc_support:1: bool,
    pub supports_cong_ctrl_max_msdus:1: bool,
    pub num_tcl_banks: u32,
    pub max_tx_ring: u32,
    pub mhi_config: *const mhi_controller_config,
    pub config): *mut ath12k_wmi_resource_config_arg,
    pub qmi_cnss_feature_bitmap: u64,
    pub rfkill_pin: u32,
    pub rfkill_cfg: u32,
    pub rfkill_on_level: u32,
    pub rddm_size: u32,
    pub def_num_link: u8,
    pub max_mlo_peer: u16,
    pub otp_board_id_register: u32,
    pub supports_sta_ps: bool,
    pub acpi_guid: *const guid_t,
    pub supports_dynamic_smps_6ghz: bool,
    pub iova_mask: u32,
    pub ce_ie_addr: *const ce_ie_addr,
    pub ce_remap: *const ce_remap,
    pub bdf_addr_offset: u32,
// setup REO queue, frag etc only for primary link peer
    pub dp_primary_link_only:1: bool,
    pub max_client_single: u32,
    pub max_client_dbs: u32,
    pub max_client_dbs_sbs: u32,
    pub client: },
    pub host_alloc_ml_id: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_ops {
    pub pdev_id): *mut *mut u8 (get_hw_mac_from_pdev_id)(int,
    pub mac_id): *const *const *const int (mac_id_to_pdev_id)(struct ath12k_hw_params hw, int,
    pub mac_id): *const *const *const int (mac_id_to_srng_id)(struct ath12k_hw_params hw, int,
    pub ab): *mut *mut int (rxdma_ring_sel_config)(struct ath12k_base,
    pub skb): *mut *mut u8 (get_ring_selector)(struct sk_buff,
    pub ring_num): *mut *mut bool (dp_srng_is_tx_comp_ring)(int,
    pub mgmt): *mut ieee80211_mgmt,
    pub status): *mut ieee80211_rx_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_fw_ie {
    pub id: __le32,
    pub len: __le32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_bd_ie_board_type {
    ATH12K_BD_IE_BOARD_NAME = 0,
    ATH12K_BD_IE_BOARD_DATA = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_bd_ie_regdb_type {
    ATH12K_BD_IE_REGDB_NAME = 0,
    ATH12K_BD_IE_REGDB_DATA = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_bd_ie_type {
// contains sub IEs of enum ath12k_bd_ie_board_type
    ATH12K_BD_IE_BOARD = 0,
// contains sub IEs of enum ath12k_bd_ie_regdb_type
    ATH12K_BD_IE_REGDB = 1,
}
