//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_type.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018-2023, Intel Corporation.
pub const ICE_BYTES_PER_WORD: c_int = 2;
pub const ICE_BYTES_PER_DWORD: c_int = 4;
pub const ICE_CHNL_MAX_TC: c_int = 16;

extern "C" {
    pub fn test_bit(_arg: tc, _arg: &bitmap) -> return;
}
extern "C" {
    pub fn div64_long(2): ((a) + (b) /, _arg: (b)) -> return;
}
// Driver always calls main vsi_handle first
pub const ICE_MAIN_VSI_HANDLE: c_int = 0;
// debug masks - set these bits in hw->debug_mask to control output

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aq_res_ids {
    ICE_NVM_RES_ID = 1,
    ICE_SPD_RES_ID,
    ICE_CHANGE_LOCK_RES_ID,
    ICE_GLOBAL_CFG_LOCK_RES_ID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fec_stats_types {
    ICE_FEC_CORR_LOW,
    ICE_FEC_CORR_HIGH,
    ICE_FEC_UNCORR_LOW,
    ICE_FEC_UNCORR_HIGH,
    ICE_FEC_MAX
}

// FW update timeout definitions are in milliseconds
pub const ICE_NVM_TIMEOUT: c_int = 180000;
pub const ICE_CHANGE_LOCK_TIMEOUT: c_int = 1000;
pub const ICE_GLOBAL_CFG_LOCK_TIMEOUT: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aq_res_access_type {
    ICE_RES_READ = 1,
    ICE_RES_WRITE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_driver_ver {
    pub major_ver: u8,
    pub minor_ver: u8,
    pub build_ver: u8,
    pub subbuild_ver: u8,
    pub driver_string: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fc_mode {
    ICE_FC_NONE = 0,
    ICE_FC_RX_PAUSE,
    ICE_FC_TX_PAUSE,
    ICE_FC_FULL,
    ICE_FC_PFC,
    ICE_FC_DFLT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_phy_cache_mode {
    ICE_FC_MODE = 0,
    ICE_SPEED_MODE,
    ICE_FEC_MODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fec_mode {
    ICE_FEC_NONE = 0,
    ICE_FEC_RS,
    ICE_FEC_BASER,
    ICE_FEC_AUTO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_phy_cache_mode_data {
    pub curr_user_fec_req: ice_fec_mode,
    pub curr_user_fc_req: ice_fc_mode,
    pub curr_user_speed_req: u16,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_set_fc_aq_failures {
    ICE_SET_FC_AQ_FAIL_NONE = 0,
    ICE_SET_FC_AQ_FAIL_GET,
    ICE_SET_FC_AQ_FAIL_SET,
    ICE_SET_FC_AQ_FAIL_UPDATE
}

// Various MAC types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_mac_type {
    ICE_MAC_UNKNOWN = 0,
    ICE_MAC_E810,
    ICE_MAC_E830,
    ICE_MAC_GENERIC,
    ICE_MAC_GENERIC_3K_E825,
}

// Media Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_media_type {
    ICE_MEDIA_UNKNOWN = 0,
    ICE_MEDIA_FIBER,
    ICE_MEDIA_BASET,
    ICE_MEDIA_BACKPLANE,
    ICE_MEDIA_DA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_vsi_type {
    ICE_VSI_PF = 0,
    ICE_VSI_VF = 1,
    ICE_VSI_CTRL = 3,	/* equates to ICE_VSI_PF with 1 queue pair */
    ICE_VSI_CHNL = 4,
    ICE_VSI_LB = 6,
    ICE_VSI_SF = 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_link_status {
// Refer to ice_aq_phy_type for bits definition
    pub phy_type_low: u64,
    pub phy_type_high: u64,
    pub topo_media_conflict: u8,
    pub max_frame_size: u16,
    pub link_speed: u16,
    pub req_speeds: u16,
    pub link_cfg_err: u8,
    pub /: *mut *mut u8 lse_ena; / Link Status Event notification,
    pub link_info: u8,
    pub an_info: u8,
    pub ext_info: u8,
    pub fec_info: u8,
    pub pacing: u8,
// Refer to #define from module_type[ICE_MODULE_TYPE_TOTAL_BYTE] of
// ice_aqc_get_phy_caps structure
//
    pub module_type: [u8; ICE_MODULE_TYPE_TOTAL_BYTE],
}

// Different reset sources for which a disable queue AQ call has to be made in
// order to clean the Tx scheduler as a part of the reset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_disq_rst_src {
    ICE_NO_RESET = 0,
    ICE_VM_RESET,
    ICE_VF_RESET,
}

// PHY info such as phy_type, etc...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_phy_info {
    pub link_info: ice_link_status,
    pub link_info_old: ice_link_status,
    pub phy_type_low: u64,
    pub phy_type_high: u64,
    pub media_type: ice_media_type,
    pub get_link_info: u8,
// Please refer to struct ice_aqc_get_link_status_data to get
// detail of enable bit in curr_user_speed_req
//
    pub curr_user_speed_req: u16,
    pub curr_user_fec_req: ice_fec_mode,
    pub curr_user_fc_req: ice_fc_mode,
    pub curr_user_phy_cfg: ice_aqc_set_phy_cfg_data,
}

// protocol enumeration for filters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fltr_ptype {
// NONE - used for undef/error
    ICE_FLTR_PTYPE_NONF_NONE = 0,
    ICE_FLTR_PTYPE_NONF_ETH,
    ICE_FLTR_PTYPE_NONF_IPV4_UDP,
    ICE_FLTR_PTYPE_NONF_IPV4_TCP,
    ICE_FLTR_PTYPE_NONF_IPV4_SCTP,
    ICE_FLTR_PTYPE_NONF_IPV4_OTHER,
    ICE_FLTR_PTYPE_NONF_IPV4_GTPU_IPV4_UDP,
    ICE_FLTR_PTYPE_NONF_IPV4_GTPU_IPV4_TCP,
    ICE_FLTR_PTYPE_NONF_IPV4_GTPU_IPV4_ICMP,
    ICE_FLTR_PTYPE_NONF_IPV4_GTPU_IPV4_OTHER,
    ICE_FLTR_PTYPE_NONF_IPV6_GTPU_IPV6_OTHER,
    ICE_FLTR_PTYPE_NONF_IPV4_L2TPV3,
    ICE_FLTR_PTYPE_NONF_IPV6_L2TPV3,
    ICE_FLTR_PTYPE_NONF_IPV4_ESP,
    ICE_FLTR_PTYPE_NONF_IPV6_ESP,
    ICE_FLTR_PTYPE_NONF_IPV4_AH,
    ICE_FLTR_PTYPE_NONF_IPV6_AH,
    ICE_FLTR_PTYPE_NONF_IPV4_NAT_T_ESP,
    ICE_FLTR_PTYPE_NONF_IPV6_NAT_T_ESP,
    ICE_FLTR_PTYPE_NONF_IPV4_PFCP_NODE,
    ICE_FLTR_PTYPE_NONF_IPV4_PFCP_SESSION,
    ICE_FLTR_PTYPE_NONF_IPV6_PFCP_NODE,
    ICE_FLTR_PTYPE_NONF_IPV6_PFCP_SESSION,
    ICE_FLTR_PTYPE_NON_IP_L2,
    ICE_FLTR_PTYPE_FRAG_IPV4,
    ICE_FLTR_PTYPE_NONF_IPV6_UDP,
    ICE_FLTR_PTYPE_NONF_IPV6_TCP,
    ICE_FLTR_PTYPE_NONF_IPV6_SCTP,
    ICE_FLTR_PTYPE_NONF_IPV6_OTHER,
    ICE_FLTR_PTYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fd_hw_seg {
    ICE_FD_HW_SEG_NON_TUN = 0,
    ICE_FD_HW_SEG_TUN,
    ICE_FD_HW_SEG_MAX,
}

// 1 ICE_VSI_PF + 1 ICE_VSI_CTRL + ICE_CHNL_MAX_TC

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fd_hw_prof {
    pub fdir_seg: [*mut ice_flow_seg_info; ICE_FD_HW_SEG_MAX],
    pub cnt: c_int,
    pub entry_h: [u64; ICE_MAX_FDIR_VSI_PER_FILTER][ICE_FD_HW_SEG_MAX],
    pub vsi_h: [u16; ICE_MAX_FDIR_VSI_PER_FILTER],
    pub prof_id: [u64; ICE_FD_HW_SEG_MAX],
}

// Common HW capabilities for SW use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_hw_common_caps {
    pub valid_functions: u32,
// DCB capabilities
    pub active_tc_bitmap: u32,
    pub maxtc: u32,
// Tx/Rx queues
    pub /: *mut *mut u16 num_rxq; / Number/Total Rx queues,
    pub /: *mut *mut u16 rxq_first_id; / First queue ID for Rx queues,
    pub /: *mut *mut u16 num_txq; / Number/Total Tx queues,
    pub /: *mut *mut u16 txq_first_id; / First queue ID for Tx queues,
// MSI-X vectors
    pub num_msix_vectors: u16,
    pub msix_vector_first_id: u16,
// Max MTU for function or device
    pub max_mtu: u16,
// Virtualization support
    pub /: *mut *mut u8 sr_iov_1_1; / SR-IOV enabled,
// RSS related capabilities
    pub /: *mut *mut u16 rss_table_size; / 512 for PFs and 64 for VFs,
    pub /: *mut *mut u8 rss_table_entry_width; / RSS Entry width in bits,
    pub dcb: u8,
    pub ieee_1588: u8,
    pub rdma: u8,
    pub roce_lag: bool,
    pub sriov_lag: bool,
    pub sriov_aa_lag: bool,
    pub nvm_update_pending_nvm: bool,
    pub nvm_update_pending_orom: bool,
    pub nvm_update_pending_netlist: bool,

    pub nvm_unified_update: bool,

// PCIe reset avoidance
    pub pcie_reset_avoidance: bool,
// Post update reset restriction
    pub reset_restrict_support: bool,
    pub tx_sched_topo_comp_mode_en: bool,
}

// IEEE 1588 TIME_SYNC specific info
// Function specific definitions

pub const ICE_TS_TMR_IDX_OWND_S: c_int = 4;

pub const ICE_TS_CLK_FREQ_S: c_int = 16;

pub const ICE_TS_CLK_SRC_S: c_int = 20;

pub const ICE_TS_TMR_IDX_ASSOC_S: c_int = 24;

// TIME_REF clock rate specification
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tspll_freq {
    ICE_TSPLL_FREQ_25_000	= 0,
    ICE_TSPLL_FREQ_122_880	= 1,
    ICE_TSPLL_FREQ_125_000	= 2,
    ICE_TSPLL_FREQ_153_600	= 3,
    ICE_TSPLL_FREQ_156_250	= 4,
    ICE_TSPLL_FREQ_245_760	= 5,

    NUM_ICE_TSPLL_FREQ,

    ICE_TSPLL_FREQ_INVALID	= -1,
}

// Clock source specification
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_clk_src {
    ICE_CLK_SRC_TCXO	= 0, /* Temperature compensated oscillator */
    ICE_CLK_SRC_TIME_REF	= 1, /* Use TIME_REF reference clock */

    NUM_ICE_CLK_SRC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_synce_clk {
    ICE_SYNCE_CLK0,
    ICE_SYNCE_CLK1,
    ICE_SYNCE_CLK_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ts_func_info {
// Function specific info
    pub time_ref: ice_tspll_freq,
    pub clk_freq: u8,
    pub clk_src: u8,
    pub tmr_index_assoc: u8,
    pub ena: u8,
    pub tmr_index_owned: u8,
    pub src_tmr_owned: u8,
    pub tmr_ena: u8,
}

// Device specific definitions
pub const ICE_TS_TMR0_OWNR_M: c_uint = 0x7;

pub const ICE_TS_TMR1_OWNR_S: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ts_dev_info {
// Device specific info
    pub ena_ports: u32,
    pub tmr_own_map: u32,
    pub tmr0_owner: u32,
    pub tmr1_owner: u32,
    pub tmr0_owned: u8,
    pub tmr1_owned: u8,
    pub ena: u8,
    pub tmr0_ena: u8,
    pub tmr1_ena: u8,
    pub ts_ll_read: u8,
    pub ts_ll_int_read: u8,
    pub ll_phy_tmr_update: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_nac_topology {
    pub mode: u32,
    pub id: u8,
}

// Function specific capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_hw_func_caps {
    pub common_cap: ice_hw_common_caps,
    pub /: *mut *mut u32 num_allocd_vfs; / Number of allocated VFs,
    pub /: *mut *mut u32 vf_base_id; / Logical ID of the first VF,
    pub guar_num_vsi: u32,
    pub /: *mut *mut u32 fd_fltr_guar; / Number of filters guaranteed,
    pub /: *mut *mut u32 fd_fltr_best_effort; / Number of best effort filters,
    pub ts_func_info: ice_ts_func_info,
}

pub const ICE_SENSOR_SUPPORT_E810_INT_TEMP_BIT: c_int = 0;
// Device wide capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_hw_dev_caps {
    pub common_cap: ice_hw_common_caps,
    pub /: *mut *mut u32 num_vfs_exposed; / Total number of VFs exposed,
    pub /: *mut *mut u32 num_vsi_allocd_to_host; / Excluding EMP VSI,
    pub /: *mut *mut u32 num_flow_director_fltr; / Number of FD filters available,
    pub ts_dev_info: ice_ts_dev_info,
    pub num_funcs: u32,
    pub nac_topo: ice_nac_topology,
// bitmap of supported sensors
// bit 0 - internal temperature sensor
// bit 31:1 - Reserved
//
    pub supported_sensors: u32,
}

// MAC info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mac_info {
    pub lan_addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
}

// Reset types used to determine which kind of reset was requested. These
// defines match what the RESET_TYPE field of the GLGEN_RSTAT register.
// ICE_RESET_PFR does not match any RESET_TYPE field in the GLGEN_RSTAT register
// because its reset source is different than the other types listed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_reset_req {
    ICE_RESET_POR	= 0,
    ICE_RESET_INVAL	= 0,
    ICE_RESET_CORER	= 1,
    ICE_RESET_GLOBR	= 2,
    ICE_RESET_EMPR	= 3,
    ICE_RESET_PFR	= 4,
}

// Bus parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bus_info {
    pub device: u16,
    pub func: u8,
}

// Flow control (FC) parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fc_info {
    pub /: *mut *mut ice_fc_mode current_mode; / FC mode in effect,
    pub /: *mut *mut ice_fc_mode req_mode; / FC mode requested by caller,
}

// Option ROM version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_orom_info {
    pub /: *mut *mut u8 major; / Major version of OROM,
    pub /: *mut *mut u8 patch; / Patch version of OROM,
    pub /: *mut *mut u16 build; / Build version of OROM,
}

// NVM version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_nvm_info {
    pub eetrack: u32,
    pub major: u8,
    pub minor: u8,
}

// netlist version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_netlist_info {
    pub /: *mut *mut u32 major; / major high/low,
    pub /: *mut *mut u32 minor; / minor high/low,
    pub /: *mut *mut u32 type; / type high/low,
    pub /: *mut *mut u32 rev; / revision high/low,
    pub /: *mut *mut u32 hash; / SHA-1 hash word,
    pub /: *mut *mut u16 cust_ver; / customer version,
}

// Enumeration of possible flash banks for the NVM, OROM, and Netlist modules
// of the flash image.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flash_bank {
    ICE_INVALID_FLASH_BANK,
    ICE_1ST_FLASH_BANK,
    ICE_2ND_FLASH_BANK,
}

// Enumeration of which flash bank is desired to read from, either the active
// bank or the inactive bank. Used to abstract 1st and 2nd bank notion from
// code which just wants to read the active or inactive flash bank.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_bank_select {
    ICE_ACTIVE_FLASH_BANK,
    ICE_INACTIVE_FLASH_BANK,
}

// information for accessing NVM, OROM, and Netlist flash banks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bank_info {
    pub /: *mut *mut u32 nvm_ptr; / Pointer to 1st NVM bank,
    pub /: *mut *mut u32 nvm_size; / Size of NVM bank,
    pub /: *mut *mut u32 orom_ptr; / Pointer to 1st OROM bank,
    pub /: *mut *mut u32 orom_size; / Size of OROM bank,
    pub /: *mut *mut u32 netlist_ptr; / Pointer to 1st Netlist bank,
    pub /: *mut *mut u32 netlist_size; / Size of Netlist bank,
    pub /: *mut *mut u32 active_css_hdr_len; / Active CSS header length,
    pub /: *mut *mut u32 inactive_css_hdr_len; / Inactive CSS header length,
    pub /: *mut *mut ice_flash_bank nvm_bank; / Active NVM bank,
    pub /: *mut *mut ice_flash_bank orom_bank; / Active OROM bank,
    pub /: *mut *mut ice_flash_bank netlist_bank; / Active Netlist bank,
}

// Flash Chip Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flash_info {
    pub /: *mut *mut ice_orom_info orom; / Option ROM version info,
    pub /: *mut *mut ice_nvm_info nvm; / NVM version information,
    pub /: *mut *mut ice_netlist_info netlist;/ Netlist version info,
    pub /: *mut *mut ice_bank_info banks; / Flash Bank information,
    pub /: *mut *mut u16 sr_words; / Shadow RAM size in words,
    pub /: *mut *mut u32 flash_size; / Size of available flash in bytes,
    pub /: *mut *mut u8 blank_nvm_mode; / is NVM empty (no FW present),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_link_default_override_tlv {
    pub options: u8,
pub const ICE_LINK_OVERRIDE_OPT_M: c_uint = 0x3F;

    pub phy_config: u8,
pub const ICE_LINK_OVERRIDE_PHY_CFG_S: c_int = 8;

pub const ICE_LINK_OVERRIDE_PAUSE_M: c_uint = 0x3;

    pub fec_options: u8,
pub const ICE_LINK_OVERRIDE_FEC_OPT_M: c_uint = 0xFF;
    pub rsvd1: u8,
    pub phy_type_low: u64,
    pub phy_type_high: u64,
}

pub const ICE_NVM_VER_LEN: c_int = 32;
// Max number of port to queue branches w.r.t topology
pub const ICE_MAX_TRAFFIC_CLASS: c_int = 8;

// ICE_DFLT_AGG_ID means that all new VM(s)/VSI node connects
// to driver defined policy for default aggregator
//
pub const ICE_INVAL_TEID: c_uint = 0xFFFFFFFF;
pub const ICE_DFLT_AGG_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sched_node {
    pub parent: *mut ice_sched_node,
    pub /: *mut *mut *mut ice_sched_node sibling; / next sibling in the same layer,
    pub children: *mut ice_sched_node,
    pub info: ice_aqc_txsched_elem_data,
    pub name: *mut c_char,
    pub rate_node: *mut devlink_rate,
    pub tx_max: u64,
    pub tx_share: u64,
    pub /: *mut *mut u32 agg_id; / aggregator group ID,
    pub id: u32,
    pub tx_priority: u32,
    pub tx_weight: u32,
    pub vsi_handle: u16,
    pub /: *mut *mut u8 in_use; / suspended or in use,
    pub /: *mut *mut u8 tx_sched_layer; / Logical Layer (1-9),
    pub num_children: u8,
    pub tc_num: u8,
    pub owner: u8,
pub const ICE_SCHED_NODE_OWNER_LAN: c_int = 0;
pub const ICE_SCHED_NODE_OWNER_RDMA: c_int = 2;
}

// Access Macros for Tx Sched Elements data

// The aggregator type determines if identifier is for a VSI group,
// aggregator group, aggregator of queues, or queue group.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_agg_type {
    ICE_AGG_TYPE_UNKNOWN = 0,
    ICE_AGG_TYPE_VSI,
    ICE_AGG_TYPE_AGG, /* aggregator */
    ICE_AGG_TYPE_Q,
    ICE_AGG_TYPE_QG
}

// Rate limit types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rl_type {
    ICE_UNKNOWN_BW = 0,
    ICE_MIN_BW,		/* for CIR profile */
    ICE_MAX_BW,		/* for EIR profile */
    ICE_SHARED_BW		/* for shared profile */
}

pub const ICE_SCHED_DFLT_BW: c_uint = 0xFFFFFFFF	/* unlimited */;
pub const ICE_SCHED_DFLT_RL_PROF_ID: c_int = 0;
pub const ICE_SCHED_NO_SHARED_RL_PROF_ID: c_uint = 0xFFFF;
pub const ICE_SCHED_DFLT_BW_WT: c_int = 4;
pub const ICE_SCHED_INVAL_PROF_ID: c_uint = 0xFFFF;

pub const ICE_MAX_PORT_PER_PCI_DEV: c_int = 8;
// Data structure for saving BW information
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_bw_type {
    ICE_BW_TYPE_PRIO,
    ICE_BW_TYPE_CIR,
    ICE_BW_TYPE_CIR_WT,
    ICE_BW_TYPE_EIR,
    ICE_BW_TYPE_EIR_WT,
    ICE_BW_TYPE_SHARED,
    ICE_BW_TYPE_CNT		/* This must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bw {
    pub bw: u32,
    pub bw_alloc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bw_type_info {
    pub ICE_BW_TYPE_CNT): DECLARE_BITMAP(bw_t_bitmap,,
    pub generic: u8,
    pub cir_bw: ice_bw,
    pub eir_bw: ice_bw,
    pub shared_bw: u32,
}

// VSI queue context structure for given TC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_q_ctx {
    pub q_handle: u16,
    pub q_teid: u32,
// bw_t_info saves queue BW information
    pub bw_t_info: ice_bw_type_info,
}

// VSI type list entry to locate corresponding VSI/aggregator nodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sched_vsi_info {
    pub vsi_node: [*mut ice_sched_node; ICE_MAX_TRAFFIC_CLASS],
    pub ag_node: [*mut ice_sched_node; ICE_MAX_TRAFFIC_CLASS],
    pub list_entry: list_head,
    pub max_lanq: [u16; ICE_MAX_TRAFFIC_CLASS],
    pub max_rdmaq: [u16; ICE_MAX_TRAFFIC_CLASS],
// bw_t_info saves VSI BW information
    pub bw_t_info: [ice_bw_type_info; ICE_MAX_TRAFFIC_CLASS],
}

// driver defines the policy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sched_tx_policy {
    pub max_num_vsis: u16,
    pub max_num_lan_qs_per_tc: [u8; ICE_MAX_TRAFFIC_CLASS],
    pub rdma_ena: u8,
}

// CEE or IEEE 802.1Qaz ETS Configuration data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dcb_ets_cfg {
    pub willing: u8,
    pub cbs: u8,
    pub maxtcs: u8,
    pub prio_table: [u8; ICE_MAX_TRAFFIC_CLASS],
    pub tcbwtable: [u8; ICE_MAX_TRAFFIC_CLASS],
    pub tsatable: [u8; ICE_MAX_TRAFFIC_CLASS],
}

// CEE or IEEE 802.1Qaz PFC Configuration data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dcb_pfc_cfg {
    pub willing: u8,
    pub mbc: u8,
    pub pfccap: u8,
    pub pfcena: u8,
}

// CEE or IEEE 802.1Qaz Application Priority data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dcb_app_priority_table {
    pub prot_id: u16,
    pub priority: u8,
    pub selector: u8,
}

pub const ICE_MAX_USER_PRIORITY: c_int = 8;
pub const ICE_DCBX_MAX_APPS: c_int = 64;
pub const ICE_LLDPDU_SIZE: c_int = 1500;
pub const ICE_TLV_STATUS_OPER: c_uint = 0x1;
pub const ICE_TLV_STATUS_SYNC: c_uint = 0x2;
pub const ICE_TLV_STATUS_ERR: c_uint = 0x4;
pub const ICE_APP_PROT_ID_ISCSI_860: c_uint = 0x035c;
pub const ICE_APP_SEL_ETHTYPE: c_uint = 0x1;
pub const ICE_APP_SEL_TCPIP: c_uint = 0x2;
pub const ICE_CEE_APP_SEL_ETHTYPE: c_uint = 0x0;
pub const ICE_SR_LINK_DEFAULT_OVERRIDE_PTR: c_uint = 0x134;
pub const ICE_CEE_APP_SEL_TCPIP: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dcbx_cfg {
    pub numapps: u32,
    pub /: *mut *mut u32 tlv_status; / CEE mode TLV status,
    pub etscfg: ice_dcb_ets_cfg,
    pub etsrec: ice_dcb_ets_cfg,
    pub pfc: ice_dcb_pfc_cfg,
pub const ICE_QOS_MODE_VLAN: c_uint = 0x0;
pub const ICE_QOS_MODE_DSCP: c_uint = 0x1;
    pub pfc_mode: u8,
    pub app: [ice_dcb_app_priority_table; ICE_DCBX_MAX_APPS],
// when DSCP mapping defined by user set its bit to 1
    pub DSCP_MAX): DECLARE_BITMAP(dscp_mapped,,
// array holding DSCP -> UP/TC values for DSCP L3 QoS mode
    pub dscp_map: [u8; DSCP_MAX],
    pub dcbx_mode: u8,
pub const ICE_DCBX_MODE_CEE: c_uint = 0x1;
pub const ICE_DCBX_MODE_IEEE: c_uint = 0x2;
    pub app_mode: u8,
pub const ICE_DCBX_APPS_NON_WILLING: c_uint = 0x1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_qos_cfg {
    pub /: *mut *mut ice_dcbx_cfg local_dcbx_cfg; / Oper/Local Cfg,
    pub /: *mut *mut ice_dcbx_cfg desired_dcbx_cfg; / CEE Desired Cfg,
    pub /: *mut *mut ice_dcbx_cfg remote_dcbx_cfg; / Peer Cfg,
    pub /: *mut *mut u8 dcbx_status : 3; / see ICE_DCBX_STATUS_DIS,
    pub 1: u8 is_sw_lldp :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_port_info {
    pub /: *mut *mut *mut ice_sched_node root; / Root Node per Port,
    pub /: *mut *mut *mut ice_hw hw; / back pointer to HW instance,
    pub /: *mut *mut u32 last_node_teid; / scheduler last node info,
    pub /: *mut *mut u16 sw_id; / Initial switch ID belongs to port,
    pub pf_vf_num: u16,
    pub port_state: u8,
    pub local_fwd_mode: u8,
pub const ICE_SCHED_PORT_STATE_INIT: c_uint = 0x0;
pub const ICE_SCHED_PORT_STATE_READY: c_uint = 0x1;
    pub lport: u8,
pub const ICE_LPORT_MASK: c_uint = 0xff;
    pub fc: ice_fc_info,
    pub mac: ice_mac_info,
    pub phy: ice_phy_info,
    pub /: *mut *mut mutex sched_lock; / protect access to TXSched tree,
// List contain profile ID(s) and other params per layer
    pub rl_prof_list: [list_head; ICE_AQC_TOPO_MAX_LEVEL_NUM],
    pub qos_cfg: ice_qos_cfg,
    pub sched_node_ids: xarray,
    pub is_vf:1: u8,
    pub is_custom_tx_enabled:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_switch_info {
    pub vsi_list_map_head: list_head,
    pub recp_list: *mut ice_sw_recipe,
    pub prof_res_bm_init: u16,
    pub max_used_prof_index: u16,
    pub rule_cnt: u16,
    pub recp_cnt: u8,
    pub ICE_MAX_FV_WORDS): DECLARE_BITMAP(prof_res_bm[ICE_MAX_NUM_PROFILES],,
}

// Enum defining the different states of the mailbox snapshot in the
// PF-VF mailbox overflow detection algorithm. The snapshot can be in
// states:
// 1. ICE_MAL_VF_DETECT_STATE_NEW_SNAPSHOT - generate a new static snapshot
// within the mailbox buffer.
// 2. ICE_MAL_VF_DETECT_STATE_TRAVERSE - iterate through the mailbox snaphot
// 3. ICE_MAL_VF_DETECT_STATE_DETECT - track the messages sent per VF via the
// mailbox and mark any VFs sending more messages than the threshold limit set.
// 4. ICE_MAL_VF_DETECT_STATE_INVALID - Invalid mailbox state set to 0xFFFFFFFF.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_mbx_snapshot_state {
    ICE_MAL_VF_DETECT_STATE_NEW_SNAPSHOT = 0,
    ICE_MAL_VF_DETECT_STATE_TRAVERSE,
    ICE_MAL_VF_DETECT_STATE_DETECT,
    ICE_MAL_VF_DETECT_STATE_INVALID = 0xFFFFFFFF,
}

// Structure to hold information of the static snapshot and the mailbox
// buffer data used to generate and track the snapshot.
// 1. state: the state of the mailbox snapshot in the malicious VF
// detection state handler ice_mbx_vf_state_handler()
// 2. head: head of the mailbox snapshot in a circular mailbox buffer
// 3. tail: tail of the mailbox snapshot in a circular mailbox buffer
// 4. num_iterations: number of messages traversed in circular mailbox buffer
// 5. num_msg_proc: number of messages processed in mailbox
// 6. num_pending_arq: number of pending asynchronous messages
// 7. max_num_msgs_mbx: maximum messages in mailbox for currently
// serviced work item or interrupt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mbx_snap_buffer_data {
    pub state: ice_mbx_snapshot_state,
    pub head: u32,
    pub tail: u32,
    pub num_iterations: u32,
    pub num_msg_proc: u16,
    pub num_pending_arq: u16,
    pub max_num_msgs_mbx: u16,
}

// Structure used to track a single VF's messages on the mailbox:
// 1. list_entry: linked list entry node
// 2. msg_count: the number of asynchronous messages sent by this VF
// 3. malicious: whether this VF has been detected as malicious before
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mbx_vf_info {
    pub list_entry: list_head,
    pub msg_count: u32,
    pub 1: u8 malicious :,
}

// Structure to hold data relevant to the captured static snapshot
// of the PF-VF mailbox.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mbx_snapshot {
    pub mbx_buf: ice_mbx_snap_buffer_data,
    pub mbx_vf: list_head,
}

// Structure to hold data to be used for capturing or updating a
// static snapshot.
// 1. num_msg_proc: number of messages processed in mailbox
// 2. num_pending_arq: number of pending asynchronous messages
// 3. max_num_msgs_mbx: maximum messages in mailbox for currently
// serviced work item or interrupt.
// 4. async_watermark_val: An upper threshold set by caller to determine
// if the pending arq count is large enough to assume that there is
// the possibility of a mailicious VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mbx_data {
    pub num_msg_proc: u16,
    pub num_pending_arq: u16,
    pub max_num_msgs_mbx: u16,
    pub async_watermark_val: u16,
}

pub const ICE_PORTS_PER_QUAD: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_e810_params {
// The wait queue lock also protects the low latency interface
    pub atqbal_wq: wait_queue_head_t,
    pub atqbal_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_eth56g_params {
    pub num_phys: u8,
    pub onestep_ena: bool,
    pub sfd_ena: bool,
    pub peer_delay: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ice_phy_params {
    pub e810: ice_e810_params,
    pub eth56g: ice_eth56g_params,
}

// Global Link Topology
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_global_link_topo {
    ICE_LINK_TOPO_UP_TO_2_LINKS,
    ICE_LINK_TOPO_UP_TO_4_LINKS,
    ICE_LINK_TOPO_UP_TO_8_LINKS,
    ICE_LINK_TOPO_RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptp_hw {
    pub phy: ice_phy_params,
    pub num_lports: u8,
    pub ports_per_phy: u8,
}

pub const ICE_E825_MAX_PHYS: c_int = 2;
// Port hardware description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_hw {
    pub hw_addr: *mut u8 __iomem,
    pub back: *mut c_void,
    pub layer_info: *mut ice_aqc_layer_props,
    pub port_info: *mut ice_port_info,
// PSM clock frequency for calculating RL profile params
    pub psm_clk_freq: u32,
    pub /: *mut *mut u64 debug_mask; / bitmap for debug mask,
    pub mac_type: ice_mac_type,
    pub /: *mut *mut u16 fd_ctr_base; / FD counter base index,
// pci info
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub /: *mut *mut u8 pf_id; / device profile info,
    pub logical_pf_id: u8,
    pub /: *mut *mut u16 max_burst_size; / driver sets this value,
    pub /: *mut *mut u8 recp_reuse:1; / indicates whether FW supports recipe reuse,
// Tx Scheduler values
    pub num_tx_sched_layers: u8,
    pub num_tx_sched_phys_layers: u8,
    pub flattened_layers: u8,
    pub max_cgds: u8,
    pub sw_entry_point_layer: u8,
    pub max_children: [u16; ICE_AQC_TOPO_MAX_LEVEL_NUM],
    pub /: *mut *mut list_head agg_list; / lists all aggregator,
    pub vsi_ctx: [*mut ice_vsi_ctx; ICE_MAX_VSI],
    pub /: *mut *mut u8 evb_veb; / true for VEB, false for VEPA,
    pub /: *mut *mut u8 reset_ongoing; / true if HW is in reset, false otherwise,
    pub bus: ice_bus_info,
    pub flash: ice_flash_info,
    pub /: *mut *mut ice_hw_dev_caps dev_caps; / device capabilities,
    pub /: *mut *mut ice_hw_func_caps func_caps; / function capabilities,
    pub /: *mut *mut *mut ice_switch_info switch_info; / switch filter lists,
// Control Queue info
    pub adminq: ice_ctl_q_info,
    pub sbq: ice_ctl_q_info,
    pub mailboxq: ice_ctl_q_info,
    pub /: *mut *mut u8 api_branch; / API branch version,
    pub /: *mut *mut u8 api_maj_ver; / API major version,
    pub /: *mut *mut u8 api_min_ver; / API minor version,
    pub /: *mut *mut u8 api_patch; / API patch version,
    pub /: *mut *mut u8 fw_branch; / firmware branch version,
    pub /: *mut *mut u8 fw_maj_ver; / firmware major version,
    pub /: *mut *mut u8 fw_min_ver; / firmware minor version,
    pub /: *mut *mut u8 fw_patch; / firmware patch version,
    pub /: *mut *mut u32 fw_build; / firmware build number,
    pub fwlog: libie_fwlog,
// Device max aggregate bandwidths corresponding to the GL_PWR_MODE_CTL
// register. Used for determining the ITR/INTRL granularity during
// initialization.
//
pub const ICE_MAX_AGG_BW_200G: c_uint = 0x0;

pub const ICE_MAX_AGG_BW_50G: c_uint = 0x2;
pub const ICE_MAX_AGG_BW_25G: c_uint = 0x3;
// ITR granularity for different speeds
pub const ICE_ITR_GRAN_ABOVE_25: c_int = 2;
pub const ICE_ITR_GRAN_MAX_25: c_int = 4;
// ITR granularity in 1 us
    pub itr_gran: u8,
// INTRL granularity for different speeds
pub const ICE_INTRL_GRAN_ABOVE_25: c_int = 4;
pub const ICE_INTRL_GRAN_MAX_25: c_int = 8;
// INTRL granularity in 1 us
    pub intrl_gran: u8,
    pub ptp: ice_ptp_hw,
    pub lane_num: i8,
// Active package version (currently active)
    pub active_pkg_ver: ice_pkg_ver,
    pub pkg_seg_id: u32,
    pub pkg_sign_type: u32,
    pub active_track_id: u32,
    pub pkg_has_signing_seg:1: u8,
    pub active_pkg_name: [u8; ICE_PKG_NAME_SIZE],
    pub active_pkg_in_nvm: u8,
// Driver's package ver - (from the Ice Metadata section)
    pub pkg_ver: ice_pkg_ver,
    pub pkg_name: [u8; ICE_PKG_NAME_SIZE],
// Driver's Ice segment format version and ID (from the Ice seg)
    pub ice_seg_fmt_ver: ice_pkg_ver,
    pub ice_seg_id: [u8; ICE_SEG_ID_SIZE],
// Pointer to the ice segment
    pub seg: *mut ice_seg,
// Pointer to allocated copy of pkg memory
    pub pkg_copy: *mut u8,
    pub pkg_size: u32,
// tunneling info
    pub tnl_lock: mutex,
    pub tnl: ice_tunnel_table,
    pub udp_tunnel_shared: udp_tunnel_nic_shared,
    pub udp_tunnel_nic: udp_tunnel_nic_info,
// dvm boost update information
    pub dvm_upd: ice_dvm_table,
// HW block tables
    pub blk: [ice_blk_info; ICE_BLK_COUNT],
    pub /: *mut *mut mutex fl_profs_locks[ICE_BLK_COUNT]; / lock fltr profiles,
    pub fl_profs: [list_head; ICE_BLK_COUNT],
// Flow Director filter info
    pub fdir_active_fltr: c_int,
    pub /: *mut *mut mutex fdir_fltr_lock; / protect Flow Director,
    pub fdir_list_head: list_head,
// Book-keeping of side-band filter count per flow-type.
// This is used to detect and handle input set changes for
// respective flow-type.
//
    pub fdir_fltr_cnt: [u16; ICE_FLTR_PTYPE_MAX],
    pub fdir_prof: *mut ice_fd_hw_prof,
    pub ICE_FLTR_PTYPE_MAX): DECLARE_BITMAP(fdir_perfect_fltr,,
    pub /: *mut *mut mutex rss_locks; / protect RSS configuration,
    pub rss_list_head: list_head,
    pub mbx_snapshot: ice_mbx_snapshot,
    pub ICE_FLOW_PTYPE_MAX): DECLARE_BITMAP(hw_ptype,,
    pub dvm_ena: u8,
    pub io_expander_handle: u16,
    pub cgu_part_number: u8,
}

// Statistics collected by each port, VSI, VEB, and S-channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_eth_stats {
    pub /: *mut *mut u64 rx_bytes; / gorc,
    pub /: *mut *mut u64 rx_unicast; / uprc,
    pub /: *mut *mut u64 rx_multicast; / mprc,
    pub /: *mut *mut u64 rx_broadcast; / bprc,
    pub /: *mut *mut u64 rx_discards; / rdpc,
    pub /: *mut *mut u64 rx_unknown_protocol; / rupp,
    pub /: *mut *mut u64 tx_bytes; / gotc,
    pub /: *mut *mut u64 tx_unicast; / uptc,
    pub /: *mut *mut u64 tx_multicast; / mptc,
    pub /: *mut *mut u64 tx_broadcast; / bptc,
    pub /: *mut *mut u64 tx_discards; / tdpc,
    pub /: *mut *mut u64 tx_errors; / tepc,
}

pub const ICE_MAX_UP: c_int = 8;
// Statistics collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_hw_port_stats {
// eth stats collected by the port
    pub eth: ice_eth_stats,
// additional port specific stats
    pub /: *mut *mut u64 tx_dropped_link_down; / tdold,
    pub /: *mut *mut u64 crc_errors; / crcerrs,
    pub /: *mut *mut u64 illegal_bytes; / illerrc,
    pub /: *mut *mut u64 error_bytes; / errbc,
    pub /: *mut *mut u64 mac_local_faults; / mlfc,
    pub /: *mut *mut u64 mac_remote_faults; / mrfc,
    pub /: *mut *mut u64 rx_len_errors; / rlec,
    pub /: *mut *mut u64 link_xon_rx; / lxonrxc,
    pub /: *mut *mut u64 link_xoff_rx; / lxoffrxc,
    pub /: *mut *mut u64 link_xon_tx; / lxontxc,
    pub /: *mut *mut u64 link_xoff_tx; / lxofftxc,
    pub /: *mut *mut u64 priority_xon_rx[8]; / pxonrxc[8],
    pub /: *mut *mut u64 priority_xoff_rx[8]; / pxoffrxc[8],
    pub /: *mut *mut u64 priority_xon_tx[8]; / pxontxc[8],
    pub /: *mut *mut u64 priority_xoff_tx[8]; / pxofftxc[8],
    pub /: *mut *mut u64 priority_xon_2_xoff[8]; / pxon2offc[8],
    pub /: *mut *mut u64 rx_size_64; / prc64,
    pub /: *mut *mut u64 rx_size_127; / prc127,
    pub /: *mut *mut u64 rx_size_255; / prc255,
    pub /: *mut *mut u64 rx_size_511; / prc511,
    pub /: *mut *mut u64 rx_size_1023; / prc1023,
    pub /: *mut *mut u64 rx_size_1522; / prc1522,
    pub /: *mut *mut u64 rx_size_big; / prc9522,
    pub /: *mut *mut u64 rx_undersize; / ruc,
    pub /: *mut *mut u64 rx_fragments; / rfc,
    pub /: *mut *mut u64 rx_oversize; / roc,
    pub /: *mut *mut u64 rx_jabber; / rjc,
    pub /: *mut *mut u64 tx_size_64; / ptc64,
    pub /: *mut *mut u64 tx_size_127; / ptc127,
    pub /: *mut *mut u64 tx_size_255; / ptc255,
    pub /: *mut *mut u64 tx_size_511; / ptc511,
    pub /: *mut *mut u64 tx_size_1023; / ptc1023,
    pub /: *mut *mut u64 tx_size_1522; / ptc1522,
    pub /: *mut *mut u64 tx_size_big; / ptc9522,
// flow director stats
    pub fd_sb_status: u32,
    pub fd_sb_match: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_sw_fwd_act_type {
    ICE_FWD_TO_VSI = 0,
    ICE_FWD_TO_VSI_LIST, /* Do not use this when adding filter */
    ICE_FWD_TO_Q,
    ICE_FWD_TO_QGRP,
    ICE_DROP_PACKET,
    ICE_MIRROR_PACKET,
    ICE_NOP,
    ICE_INVAL_ACT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aq_get_set_rss_lut_params {
    pub /: *mut *mut *mut u8 lut; / input RSS LUT for set and output RSS LUT for get,
    pub /: *mut *mut ice_lut_size lut_size; / size of the LUT buffer,
    pub /: *mut *mut ice_lut_type lut_type; / type of the LUT (i.e. VSI, PF, Global),
    pub /: *mut *mut u16 vsi_handle; / software VSI handle,
    pub /: *mut *mut u8 global_lut_id; / only valid when lut_type is global,
}

// Checksum and Shadow RAM pointers
pub const ICE_SR_NVM_CTRL_WORD: c_uint = 0x00;
pub const ICE_SR_BOOT_CFG_PTR: c_uint = 0x132;
pub const ICE_SR_NVM_WOL_CFG: c_uint = 0x19;
pub const ICE_NVM_OROM_VER_OFF: c_uint = 0x02;
pub const ICE_SR_PBA_BLOCK_PTR: c_uint = 0x16;
pub const ICE_SR_NVM_DEV_STARTER_VER: c_uint = 0x18;
pub const ICE_SR_NVM_EETRACK_LO: c_uint = 0x2D;
pub const ICE_SR_NVM_EETRACK_HI: c_uint = 0x2E;
pub const ICE_NVM_VER_LO_SHIFT: c_int = 0;

pub const ICE_NVM_VER_HI_SHIFT: c_int = 12;

pub const ICE_OROM_VER_PATCH_SHIFT: c_int = 0;

pub const ICE_OROM_VER_BUILD_SHIFT: c_int = 8;

pub const ICE_OROM_VER_SHIFT: c_int = 24;

pub const ICE_SR_PFA_PTR: c_uint = 0x40;
pub const ICE_SR_1ST_NVM_BANK_PTR: c_uint = 0x42;
pub const ICE_SR_NVM_BANK_SIZE: c_uint = 0x43;
pub const ICE_SR_1ST_OROM_BANK_PTR: c_uint = 0x44;
pub const ICE_SR_OROM_BANK_SIZE: c_uint = 0x45;
pub const ICE_SR_NETLIST_BANK_PTR: c_uint = 0x46;
pub const ICE_SR_NETLIST_BANK_SIZE: c_uint = 0x47;
pub const ICE_SR_SECTOR_SIZE_IN_WORDS: c_uint = 0x800;
// CSS Header words
pub const ICE_NVM_CSS_HDR_LEN_L: c_uint = 0x02;
pub const ICE_NVM_CSS_HDR_LEN_H: c_uint = 0x03;
pub const ICE_NVM_CSS_SREV_L: c_uint = 0x14;
pub const ICE_NVM_CSS_SREV_H: c_uint = 0x15;
// Length of Authentication header section in words
pub const ICE_NVM_AUTH_HEADER_LEN: c_uint = 0x08;
// The Link Topology Netlist section is stored as a series of words. It is
// stored in the NVM as a TLV, with the first two words containing the type
// and length.
//
pub const ICE_NETLIST_LINK_TOPO_MOD_ID: c_uint = 0x011B;
pub const ICE_NETLIST_TYPE_OFFSET: c_uint = 0x0000;
pub const ICE_NETLIST_LEN_OFFSET: c_uint = 0x0001;
// The Link Topology section follows the TLV header. When reading the netlist
// using ice_read_netlist_module, we need to account for the 2-word TLV
// header.
//

// The Netlist ID Block is located after all of the Link Topology nodes.
pub const ICE_NETLIST_ID_BLK_SIZE: c_uint = 0x30;

// netlist ID block field offsets (word offsets)
pub const ICE_NETLIST_ID_BLK_MAJOR_VER_LOW: c_uint = 0x02;
pub const ICE_NETLIST_ID_BLK_MAJOR_VER_HIGH: c_uint = 0x03;
pub const ICE_NETLIST_ID_BLK_MINOR_VER_LOW: c_uint = 0x04;
pub const ICE_NETLIST_ID_BLK_MINOR_VER_HIGH: c_uint = 0x05;
pub const ICE_NETLIST_ID_BLK_TYPE_LOW: c_uint = 0x06;
pub const ICE_NETLIST_ID_BLK_TYPE_HIGH: c_uint = 0x07;
pub const ICE_NETLIST_ID_BLK_REV_LOW: c_uint = 0x08;
pub const ICE_NETLIST_ID_BLK_REV_HIGH: c_uint = 0x09;

pub const ICE_NETLIST_ID_BLK_CUST_VER: c_uint = 0x2F;
// Auxiliary field, mask, and shift definition for Shadow RAM and NVM Flash
pub const ICE_SR_CTRL_WORD_1_S: c_uint = 0x06;

pub const ICE_SR_CTRL_WORD_VALID: c_uint = 0x1;

// Link override related
pub const ICE_SR_PFA_LINK_OVERRIDE_WORDS: c_int = 10;
pub const ICE_SR_PFA_LINK_OVERRIDE_PHY_WORDS: c_int = 4;
pub const ICE_SR_PFA_LINK_OVERRIDE_OFFSET: c_int = 2;
pub const ICE_SR_PFA_LINK_OVERRIDE_FEC_OFFSET: c_int = 1;
pub const ICE_SR_PFA_LINK_OVERRIDE_PHY_OFFSET: c_int = 2;
pub const ICE_FW_API_LINK_OVERRIDE_MAJ: c_int = 1;
pub const ICE_FW_API_LINK_OVERRIDE_MIN: c_int = 5;
pub const ICE_FW_API_LINK_OVERRIDE_PATCH: c_int = 2;
pub const ICE_SR_WORDS_IN_1KB: c_int = 512;
// AQ API version for LLDP_FILTER_CONTROL
pub const ICE_FW_API_LLDP_FLTR_MAJ: c_int = 1;
pub const ICE_FW_API_LLDP_FLTR_MIN: c_int = 7;
pub const ICE_FW_API_LLDP_FLTR_PATCH: c_int = 1;
// AQ API version for report default configuration
pub const ICE_FW_API_REPORT_DFLT_CFG_MAJ: c_int = 1;
pub const ICE_FW_API_REPORT_DFLT_CFG_MIN: c_int = 7;
pub const ICE_FW_API_REPORT_DFLT_CFG_PATCH: c_int = 3;
// AQ API version for Health Status support
pub const ICE_FW_API_HEALTH_REPORT_MAJ: c_int = 1;
pub const ICE_FW_API_HEALTH_REPORT_MIN: c_int = 7;
pub const ICE_FW_API_HEALTH_REPORT_PATCH: c_int = 6;
