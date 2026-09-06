//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_if.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

pub const QED_TX_SWS_TIMER_DFLT: c_int = 500;
pub const QED_TWO_MSL_TIMER_DFLT: c_int = 4000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbx_protocol_type {
    DCBX_PROTOCOL_ISCSI,
    DCBX_PROTOCOL_FCOE,
    DCBX_PROTOCOL_ROCE,
    DCBX_PROTOCOL_ROCE_V2,
    DCBX_PROTOCOL_ETH,
    DCBX_MAX_PROTOCOL_TYPE
}

pub const QED_LLDP_CHASSIS_ID_STAT_LEN: c_int = 4;
pub const QED_LLDP_PORT_ID_STAT_LEN: c_int = 4;
pub const QED_DCBX_MAX_APP_PROTOCOL: c_int = 32;
pub const QED_MAX_PFC_PRIORITIES: c_int = 8;
pub const QED_DCBX_DSCP_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_lldp_remote {
    pub peer_chassis_id: [u32; QED_LLDP_CHASSIS_ID_STAT_LEN],
    pub peer_port_id: [u32; QED_LLDP_PORT_ID_STAT_LEN],
    pub enable_rx: bool,
    pub enable_tx: bool,
    pub tx_interval: u32,
    pub max_credit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_lldp_local {
    pub local_chassis_id: [u32; QED_LLDP_CHASSIS_ID_STAT_LEN],
    pub local_port_id: [u32; QED_LLDP_PORT_ID_STAT_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_app_prio {
    pub roce: u8,
    pub roce_v2: u8,
    pub fcoe: u8,
    pub iscsi: u8,
    pub eth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dbcx_pfc_params {
    pub willing: bool,
    pub enabled: bool,
    pub prio: [u8; QED_MAX_PFC_PRIORITIES],
    pub max_tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_dcbx_sf_ieee_type {
    QED_DCBX_SF_IEEE_ETHTYPE,
    QED_DCBX_SF_IEEE_TCP_PORT,
    QED_DCBX_SF_IEEE_UDP_PORT,
    QED_DCBX_SF_IEEE_TCP_UDP_PORT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_app_entry {
    pub ethtype: bool,
    pub sf_ieee: qed_dcbx_sf_ieee_type,
    pub enabled: bool,
    pub prio: u8,
    pub proto_id: u16,
    pub proto_type: dcbx_protocol_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_params {
    pub app_entry: [qed_app_entry; QED_DCBX_MAX_APP_PROTOCOL],
    pub num_app_entries: u16,
    pub app_willing: bool,
    pub app_valid: bool,
    pub app_error: bool,
    pub ets_willing: bool,
    pub ets_enabled: bool,
    pub ets_cbs: bool,
    pub valid: bool,
    pub ets_pri_tc_tbl: [u8; QED_MAX_PFC_PRIORITIES],
    pub ets_tc_bw_tbl: [u8; QED_MAX_PFC_PRIORITIES],
    pub ets_tc_tsa_tbl: [u8; QED_MAX_PFC_PRIORITIES],
    pub pfc: qed_dbcx_pfc_params,
    pub max_ets_tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_admin_params {
    pub params: qed_dcbx_params,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_remote_params {
    pub params: qed_dcbx_params,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_operational_params {
    pub app_prio: qed_dcbx_app_prio,
    pub params: qed_dcbx_params,
    pub valid: bool,
    pub enabled: bool,
    pub ieee: bool,
    pub cee: bool,
    pub local: bool,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_get {
    pub operational: qed_dcbx_operational_params,
    pub lldp_remote: qed_dcbx_lldp_remote,
    pub lldp_local: qed_dcbx_lldp_local,
    pub remote: qed_dcbx_remote_params,
    pub local: qed_dcbx_admin_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_nvm_images {
    QED_NVM_IMAGE_ISCSI_CFG,
    QED_NVM_IMAGE_FCOE_CFG,
    QED_NVM_IMAGE_MDUMP,
    QED_NVM_IMAGE_NVM_CFG1,
    QED_NVM_IMAGE_DEFAULT_CFG,
    QED_NVM_IMAGE_NVM_META,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_link_eee_params {
    pub tx_lpi_timer: u32,

// Capabilities are represented using QED_EEE_*_ADV values
    pub adv_caps: u8,
    pub lp_adv_caps: u8,
    pub enable: bool,
    pub tx_lpi_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_led_mode {
    QED_LED_MODE_OFF,
    QED_LED_MODE_ON,
    QED_LED_MODE_RESTORE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mfw_tlv_eth {
    pub lso_maxoff_size: u16,
    pub lso_maxoff_size_set: bool,
    pub lso_minseg_size: u16,
    pub lso_minseg_size_set: bool,
    pub prom_mode: u8,
    pub prom_mode_set: bool,
    pub tx_descr_size: u16,
    pub tx_descr_size_set: bool,
    pub rx_descr_size: u16,
    pub rx_descr_size_set: bool,
    pub netq_count: u16,
    pub netq_count_set: bool,
    pub tcp4_offloads: u32,
    pub tcp4_offloads_set: bool,
    pub tcp6_offloads: u32,
    pub tcp6_offloads_set: bool,
    pub tx_descr_qdepth: u16,
    pub tx_descr_qdepth_set: bool,
    pub rx_descr_qdepth: u16,
    pub rx_descr_qdepth_set: bool,
    pub iov_offload: u8,

    pub iov_offload_set: bool,
    pub txqs_empty: u8,
    pub txqs_empty_set: bool,
    pub rxqs_empty: u8,
    pub rxqs_empty_set: bool,
    pub num_txqs_full: u8,
    pub num_txqs_full_set: bool,
    pub num_rxqs_full: u8,
    pub num_rxqs_full_set: bool,
}

pub const QED_MFW_TLV_TIME_SIZE: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mfw_tlv_time {
    pub b_set: bool,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub msec: u16,
    pub usec: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mfw_tlv_fcoe {
    pub scsi_timeout: u8,
    pub scsi_timeout_set: bool,
    pub rt_tov: u32,
    pub rt_tov_set: bool,
    pub ra_tov: u32,
    pub ra_tov_set: bool,
    pub ed_tov: u32,
    pub ed_tov_set: bool,
    pub cr_tov: u32,
    pub cr_tov_set: bool,
    pub boot_type: u8,
    pub boot_type_set: bool,
    pub npiv_state: u8,
    pub npiv_state_set: bool,
    pub num_npiv_ids: u32,
    pub num_npiv_ids_set: bool,
    pub switch_name: [u8; 8],
    pub switch_name_set: bool,
    pub switch_portnum: u16,
    pub switch_portnum_set: bool,
    pub switch_portid: [u8; 3],
    pub switch_portid_set: bool,
    pub vendor_name: [u8; 8],
    pub vendor_name_set: bool,
    pub switch_model: [u8; 8],
    pub switch_model_set: bool,
    pub switch_fw_version: [u8; 8],
    pub switch_fw_version_set: bool,
    pub qos_pri: u8,
    pub qos_pri_set: bool,
    pub port_alias: [u8; 3],
    pub port_alias_set: bool,
    pub port_state: u8,

    pub port_state_set: bool,
    pub fip_tx_descr_size: u16,
    pub fip_tx_descr_size_set: bool,
    pub fip_rx_descr_size: u16,
    pub fip_rx_descr_size_set: bool,
    pub link_failures: u16,
    pub link_failures_set: bool,
    pub fcoe_boot_progress: u8,
    pub fcoe_boot_progress_set: bool,
    pub rx_bcast: u64,
    pub rx_bcast_set: bool,
    pub tx_bcast: u64,
    pub tx_bcast_set: bool,
    pub fcoe_txq_depth: u16,
    pub fcoe_txq_depth_set: bool,
    pub fcoe_rxq_depth: u16,
    pub fcoe_rxq_depth_set: bool,
    pub fcoe_rx_frames: u64,
    pub fcoe_rx_frames_set: bool,
    pub fcoe_rx_bytes: u64,
    pub fcoe_rx_bytes_set: bool,
    pub fcoe_tx_frames: u64,
    pub fcoe_tx_frames_set: bool,
    pub fcoe_tx_bytes: u64,
    pub fcoe_tx_bytes_set: bool,
    pub crc_count: u16,
    pub crc_count_set: bool,
    pub crc_err_src_fcid: [u32; 5],
    pub crc_err_src_fcid_set: [bool; 5],
    pub crc_err: [qed_mfw_tlv_time; 5],
    pub losync_err: u16,
    pub losync_err_set: bool,
    pub losig_err: u16,
    pub losig_err_set: bool,
    pub primtive_err: u16,
    pub primtive_err_set: bool,
    pub disparity_err: u16,
    pub disparity_err_set: bool,
    pub code_violation_err: u16,
    pub code_violation_err_set: bool,
    pub flogi_param: [u32; 4],
    pub flogi_param_set: [bool; 4],
    pub flogi_tstamp: qed_mfw_tlv_time,
    pub flogi_acc_param: [u32; 4],
    pub flogi_acc_param_set: [bool; 4],
    pub flogi_acc_tstamp: qed_mfw_tlv_time,
    pub flogi_rjt: u32,
    pub flogi_rjt_set: bool,
    pub flogi_rjt_tstamp: qed_mfw_tlv_time,
    pub fdiscs: u32,
    pub fdiscs_set: bool,
    pub fdisc_acc: u8,
    pub fdisc_acc_set: bool,
    pub fdisc_rjt: u8,
    pub fdisc_rjt_set: bool,
    pub plogi: u8,
    pub plogi_set: bool,
    pub plogi_acc: u8,
    pub plogi_acc_set: bool,
    pub plogi_rjt: u8,
    pub plogi_rjt_set: bool,
    pub plogi_dst_fcid: [u32; 5],
    pub plogi_dst_fcid_set: [bool; 5],
    pub plogi_tstamp: [qed_mfw_tlv_time; 5],
    pub plogi_acc_src_fcid: [u32; 5],
    pub plogi_acc_src_fcid_set: [bool; 5],
    pub plogi_acc_tstamp: [qed_mfw_tlv_time; 5],
    pub tx_plogos: u8,
    pub tx_plogos_set: bool,
    pub plogo_acc: u8,
    pub plogo_acc_set: bool,
    pub plogo_rjt: u8,
    pub plogo_rjt_set: bool,
    pub plogo_src_fcid: [u32; 5],
    pub plogo_src_fcid_set: [bool; 5],
    pub plogo_tstamp: [qed_mfw_tlv_time; 5],
    pub rx_logos: u8,
    pub rx_logos_set: bool,
    pub tx_accs: u8,
    pub tx_accs_set: bool,
    pub tx_prlis: u8,
    pub tx_prlis_set: bool,
    pub rx_accs: u8,
    pub rx_accs_set: bool,
    pub tx_abts: u8,
    pub tx_abts_set: bool,
    pub rx_abts_acc: u8,
    pub rx_abts_acc_set: bool,
    pub rx_abts_rjt: u8,
    pub rx_abts_rjt_set: bool,
    pub abts_dst_fcid: [u32; 5],
    pub abts_dst_fcid_set: [bool; 5],
    pub abts_tstamp: [qed_mfw_tlv_time; 5],
    pub rx_rscn: u8,
    pub rx_rscn_set: bool,
    pub rx_rscn_nport: [u32; 4],
    pub rx_rscn_nport_set: [bool; 4],
    pub tx_lun_rst: u8,
    pub tx_lun_rst_set: bool,
    pub abort_task_sets: u8,
    pub abort_task_sets_set: bool,
    pub tx_tprlos: u8,
    pub tx_tprlos_set: bool,
    pub tx_nos: u8,
    pub tx_nos_set: bool,
    pub rx_nos: u8,
    pub rx_nos_set: bool,
    pub ols: u8,
    pub ols_set: bool,
    pub lr: u8,
    pub lr_set: bool,
    pub lrr: u8,
    pub lrr_set: bool,
    pub tx_lip: u8,
    pub tx_lip_set: bool,
    pub rx_lip: u8,
    pub rx_lip_set: bool,
    pub eofa: u8,
    pub eofa_set: bool,
    pub eofni: u8,
    pub eofni_set: bool,
    pub scsi_chks: u8,
    pub scsi_chks_set: bool,
    pub scsi_cond_met: u8,
    pub scsi_cond_met_set: bool,
    pub scsi_busy: u8,
    pub scsi_busy_set: bool,
    pub scsi_inter: u8,
    pub scsi_inter_set: bool,
    pub scsi_inter_cond_met: u8,
    pub scsi_inter_cond_met_set: bool,
    pub scsi_rsv_conflicts: u8,
    pub scsi_rsv_conflicts_set: bool,
    pub scsi_tsk_full: u8,
    pub scsi_tsk_full_set: bool,
    pub scsi_aca_active: u8,
    pub scsi_aca_active_set: bool,
    pub scsi_tsk_abort: u8,
    pub scsi_tsk_abort_set: bool,
    pub scsi_rx_chk: [u32; 5],
    pub scsi_rx_chk_set: [bool; 5],
    pub scsi_chk_tstamp: [qed_mfw_tlv_time; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mfw_tlv_iscsi {
    pub target_llmnr: u8,
    pub target_llmnr_set: bool,
    pub header_digest: u8,
    pub header_digest_set: bool,
    pub data_digest: u8,
    pub data_digest_set: bool,
    pub auth_method: u8,

    pub auth_method_set: bool,
    pub boot_taget_portal: u16,
    pub boot_taget_portal_set: bool,
    pub frame_size: u16,
    pub frame_size_set: bool,
    pub tx_desc_size: u16,
    pub tx_desc_size_set: bool,
    pub rx_desc_size: u16,
    pub rx_desc_size_set: bool,
    pub boot_progress: u8,
    pub boot_progress_set: bool,
    pub tx_desc_qdepth: u16,
    pub tx_desc_qdepth_set: bool,
    pub rx_desc_qdepth: u16,
    pub rx_desc_qdepth_set: bool,
    pub rx_frames: u64,
    pub rx_frames_set: bool,
    pub rx_bytes: u64,
    pub rx_bytes_set: bool,
    pub tx_frames: u64,
    pub tx_frames_set: bool,
    pub tx_bytes: u64,
    pub tx_bytes_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_db_rec_width {
    DB_REC_WIDTH_32B,
    DB_REC_WIDTH_64B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_db_rec_space {
    DB_REC_KERNEL,
    DB_REC_USER,
}

pub const QED_COALESCE_MAX: c_uint = 0x1FF;
pub const QED_DEFAULT_RX_USECS: c_int = 12;
pub const QED_DEFAULT_TX_USECS: c_int = 48;
// forward
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_pf_params {
// The following parameters are used during HW-init
// and these parameters need to be passed as arguments
// to update_pf_params routine invoked before slowpath start
//
    pub num_cons: u16,
// per-VF number of CIDs
    pub num_vf_cons: u8,

// To enable arfs, previous to HW-init a positive number needs to be
// set [as filters require allocated searcher ILT memory].
// This will set the maximal number of configured steering-filters.
//
    pub num_arfs_filters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_pf_params {
// The following parameters are used during protocol-init
    pub glbl_q_params_addr: u64,
    pub bdq_pbl_base_addr: [u64; 2],
// The following parameters are used during HW-init
// and these parameters need to be passed as arguments
// to update_pf_params routine invoked before slowpath start
//
    pub num_cons: u16,
    pub num_tasks: u16,
// The following parameters are used during protocol-init
    pub sq_num_pbl_pages: u16,
    pub cq_num_entries: u16,
    pub cmdq_num_entries: u16,
    pub rq_buffer_log_size: u16,
    pub mtu: u16,
    pub dummy_icid: u16,
    pub bdq_xoff_threshold: [u16; 2],
    pub bdq_xon_threshold: [u16; 2],
    pub rq_buffer_size: u16,
    pub /: *mut *mut u8 num_cqs; / num of global CQs,
    pub log_page_size: u8,
    pub gl_rq_pi: u8,
    pub gl_cmd_pi: u8,
    pub debug_mode: u8,
    pub is_target: u8,
    pub bdq_pbl_num_entries: [u8; 2],
}

// Most of the parameters below are described in the FW iSCSI / TCP HSI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_pf_params {
    pub glbl_q_params_addr: u64,
    pub bdq_pbl_base_addr: [u64; 3],
    pub cq_num_entries: u16,
    pub cmdq_num_entries: u16,
    pub two_msl_timer: u32,
    pub tx_sws_timer: u16,
// The following parameters are used during HW-init
// and these parameters need to be passed as arguments
// to update_pf_params routine invoked before slowpath start
//
    pub num_cons: u16,
    pub num_tasks: u16,
// The following parameters are used during protocol-init
    pub half_way_close_timeout: u16,
    pub bdq_xoff_threshold: [u16; 3],
    pub bdq_xon_threshold: [u16; 3],
    pub cmdq_xoff_threshold: u16,
    pub cmdq_xon_threshold: u16,
    pub rq_buffer_size: u16,
    pub num_sq_pages_in_ring: u8,
    pub num_r2tq_pages_in_ring: u8,
    pub num_uhq_pages_in_ring: u8,
    pub num_queues: u8,
    pub log_page_size: u8,
    pub rqe_log_size: u8,
    pub max_fin_rt: u8,
    pub gl_rq_pi: u8,
    pub gl_cmd_pi: u8,
    pub debug_mode: u8,
    pub ll2_ooo_queue_id: u8,
    pub is_target: u8,
    pub is_soc_en: u8,
    pub soc_num_of_blocks_log: u8,
    pub bdq_pbl_num_entries: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_pf_params {
    pub glbl_q_params_addr: u64,
    pub cq_num_entries: u16,
    pub num_cons: u16,
    pub num_tasks: u16,
    pub num_sq_pages_in_ring: u8,
    pub num_r2tq_pages_in_ring: u8,
    pub num_uhq_pages_in_ring: u8,
    pub num_queues: u8,
    pub gl_rq_pi: u8,
    pub gl_cmd_pi: u8,
    pub debug_mode: u8,
    pub ll2_ooo_queue_id: u8,
    pub min_rto: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_pf_params {
// Supplied to QED during resource allocation (may affect the ILT and
// the doorbell BAR).
//
    pub /: *mut *mut u32 min_dpis; / number of requested DPIs,
    pub /: *mut *mut u32 num_qps; / number of requested Queue Pairs,
    pub /: *mut *mut u32 num_srqs; / number of requested SRQ,
    pub /: *mut *mut u8 roce_edpm_mode; / see QED_ROCE_EDPM_MODE_ENABLE,
    pub /: *mut *mut u8 gl_pi; / protocol index,
// Will allocate rate limiters to be used with QPs
    pub enable_dcqcn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_pf_params {
    pub eth_pf_params: qed_eth_pf_params,
    pub fcoe_pf_params: qed_fcoe_pf_params,
    pub iscsi_pf_params: qed_iscsi_pf_params,
    pub nvmetcp_pf_params: qed_nvmetcp_pf_params,
    pub rdma_pf_params: qed_rdma_pf_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_int_mode {
    QED_INT_MODE_INTA,
    QED_INT_MODE_MSIX,
    QED_INT_MODE_MSI,
    QED_INT_MODE_POLL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sb_info {
    pub sb_virt: *mut status_block,
    pub sb_phys: dma_addr_t,
    pub /: *mut *mut u32 sb_ack; / Last given ack,
    pub igu_sb_id: u16,
    pub igu_addr: *mut void __iomem,
    pub flags: u8,
pub const QED_SB_INFO_INIT: c_uint = 0x1;
pub const QED_SB_INFO_SETUP: c_uint = 0x2;
    pub cdev: *mut qed_dev,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_hw_err_type {
    QED_HW_ERR_FAN_FAIL,
    QED_HW_ERR_MFW_RESP_FAIL,
    QED_HW_ERR_HW_ATTN,
    QED_HW_ERR_DMAE_FAIL,
    QED_HW_ERR_RAMROD_FAIL,
    QED_HW_ERR_FW_ASSERT,
    QED_HW_ERR_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_dev_type {
    QED_DEV_TYPE_BB,
    QED_DEV_TYPE_AH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev_info {
    pub pci_mem_start: c_ulong,
    pub pci_mem_end: c_ulong,
    pub pci_irq: c_uint,
    pub num_hwfns: u8,
    pub hw_mac: [u8; ETH_ALEN],
// FW version
    pub fw_major: u16,
    pub fw_minor: u16,
    pub fw_rev: u16,
    pub fw_eng: u16,
// MFW version
    pub mfw_rev: u32,
pub const QED_MFW_VERSION_0_MASK: c_uint = 0x000000FF;
pub const QED_MFW_VERSION_0_OFFSET: c_int = 0;
pub const QED_MFW_VERSION_1_MASK: c_uint = 0x0000FF00;
pub const QED_MFW_VERSION_1_OFFSET: c_int = 8;
pub const QED_MFW_VERSION_2_MASK: c_uint = 0x00FF0000;
pub const QED_MFW_VERSION_2_OFFSET: c_int = 16;
pub const QED_MFW_VERSION_3_MASK: c_uint = 0xFF000000;
pub const QED_MFW_VERSION_3_OFFSET: c_int = 24;
    pub flash_size: u32,
    pub b_arfs_capable: bool,
    pub b_inter_pf_switch: bool,
    pub tx_switching: bool,
    pub rdma_supported: bool,
    pub mtu: u16,
    pub wol_support: bool,
    pub smart_an: bool,
    pub esl: bool,
// MBI version
    pub mbi_version: u32,
pub const QED_MBI_VERSION_0_MASK: c_uint = 0x000000FF;
pub const QED_MBI_VERSION_0_OFFSET: c_int = 0;
pub const QED_MBI_VERSION_1_MASK: c_uint = 0x0000FF00;
pub const QED_MBI_VERSION_1_OFFSET: c_int = 8;
pub const QED_MBI_VERSION_2_MASK: c_uint = 0x00FF0000;
pub const QED_MBI_VERSION_2_OFFSET: c_int = 16;
    pub dev_type: qed_dev_type,
// Output parameters for qede
    pub vxlan_enable: bool,
    pub gre_enable: bool,
    pub geneve_enable: bool,
    pub abs_pf_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_sb_type {
    QED_SB_TYPE_L2_QUEUE,
    QED_SB_TYPE_CNQ,
    QED_SB_TYPE_STORAGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_protocol {
    QED_PROTOCOL_ETH,
    QED_PROTOCOL_ISCSI,
    QED_PROTOCOL_NVMETCP = QED_PROTOCOL_ISCSI,
    QED_PROTOCOL_FCOE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_fec_mode {
    QED_FEC_MODE_NONE			= BIT(0),
    QED_FEC_MODE_FIRECODE			= BIT(1),
    QED_FEC_MODE_RS				= BIT(2),
    QED_FEC_MODE_AUTO			= BIT(3),
    QED_FEC_MODE_UNSUPPORTED		= BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_link_params {
    pub link_up: bool,
    pub override_flags: u32,

    pub autoneg: bool,
    pub forced_speed: u32,
    pub pause_config: u32,

    pub loopback_mode: u32,

    pub eee: qed_link_eee_params,
    pub fec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_link_output {
    pub link_up: bool,
    pub /: *mut *mut u32 speed; / In Mb/s,
    pub /: *mut *mut u8 duplex; / In DUPLEX defs,
    pub /: *mut *mut u8 port; / In PORT defs,
    pub autoneg: bool,
    pub pause_config: u32,
// EEE - capability & param
    pub eee_supported: bool,
    pub eee_active: bool,
    pub sup_caps: u8,
    pub eee: qed_link_eee_params,
    pub sup_fec: u32,
    pub active_fec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_probe_params {
    pub protocol: qed_protocol,
    pub dp_module: u32,
    pub dp_level: u8,
    pub is_vf: bool,
    pub recov_in_prog: bool,
}

pub const QED_DRV_VER_STR_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_slowpath_params {
    pub int_mode: u32,
    pub drv_major: u8,
    pub drv_minor: u8,
    pub drv_rev: u8,
    pub drv_eng: u8,
    pub name: [u8; QED_DRV_VER_STR_SIZE],
}

pub const ILT_PAGE_SIZE_TCFC: c_uint = 0x8000 /* 32KB */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_int_info {
    pub msix: *mut msix_entry,
    pub msix_cnt: u8,
// This should be updated by the protocol driver
    pub used_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_generic_tlvs {

    pub feat_flags: u16,
pub const QED_TLV_MAC_COUNT: c_int = 3;
    pub mac: [u8; QED_TLV_MAC_COUNT][ETH_ALEN],
}

pub const QED_I2C_DEV_ADDR_A0: c_uint = 0xA0;
pub const QED_I2C_DEV_ADDR_A2: c_uint = 0xA2;
pub const QED_NVM_SIGNATURE: c_uint = 0x12435687;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_nvm_flash_cmd {
    QED_NVM_FLASH_CMD_FILE_DATA = 0x2,
    QED_NVM_FLASH_CMD_FILE_START = 0x3,
    QED_NVM_FLASH_CMD_NVM_CHANGE = 0x4,
    QED_NVM_FLASH_CMD_NVM_CFG_ID = 0x5,
    QED_NVM_FLASH_CMD_NVM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_devlink {
    pub cdev: *mut qed_dev,
    pub fw_reporter: *mut devlink_health_reporter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sb_info_dbg {
    pub igu_prod: u32,
    pub igu_cons: u32,
    pub pi: [u16; PIS_PER_SB],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_common_cb_ops {
    pub fw_rc): *mut *mut *mut *mut void (arfs_filter_op)(void dev, void fltr, u8,
    pub link): *mut *mut *mut void (link_update)(void dev, struct qed_link_output,
    pub dev): *mut *mut void (schedule_recovery_handler)(void,
    pub err_type): qed_hw_err_type,
    pub mib_type): *mut *mut *mut *mut void (dcbx_aen)(void dev, struct qed_dcbx_get get, u32,
    pub data): *mut *mut *mut void (get_generic_tlv_data)(void dev, struct qed_generic_tlvs,
    pub data): *mut *mut *mut void (get_protocol_tlv_data)(void dev, void,
    pub dev): *mut *mut void (bw_update)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_selftest_ops {
//
// selftest_interrupt(): Perform interrupt test.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (selftest_interrupt)(struct qed_dev,
//
// selftest_memory(): Perform memory test.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (selftest_memory)(struct qed_dev,
//
// selftest_register(): Perform register test.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (selftest_register)(struct qed_dev,
//
// selftest_clock(): Perform clock test.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (selftest_clock)(struct qed_dev,
//
// selftest_nvram(): Perform nvram test.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (selftest_nvram) (struct qed_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_common_ops {
    pub selftest: *mut qed_selftest_ops,
    pub params): *mut qed_probe_params,
    pub cdev): *mut *mut void (remove)(struct qed_dev,
    pub state): *mut *mut *mut int (set_power_state)(struct qed_dev cdev, pci_power_t,
    pub name[]): *mut *mut *mut void (set_name) (struct qed_dev cdev, char,
// Client drivers need to make this call before slowpath_start.
// PF params required for the call before slowpath_start is
// documented within the qed_pf_params structure definition.
//
    pub params): *mut qed_pf_params,
    pub params): *mut qed_slowpath_params,
    pub cdev): *mut *mut int (slowpath_stop)(struct qed_dev,
// Requests to use `cnt' interrupts for fastpath.
// upon success, returns number of interrupts allocated for fastpath.
//
    pub cnt): *mut *mut *mut int (set_fp_int)(struct qed_dev cdev, u16,
// Fills `info' with pointers required for utilizing interrupts
    pub info): *mut *mut *mut int (get_fp_int)(struct qed_dev cdev, struct qed_int_info,
    pub type): qed_sb_type,
    pub type): qed_sb_type,
    pub )): *mut *mut void (handler)(void,
    pub index): *mut *mut *mut void (simd_handler_clean)(struct qed_dev cdev, int,
    pub num_dumped_bytes): *mut *mut *mut *mut int (dbg_grc)(struct qed_dev cdev, void buffer, u32,
    pub cdev): *mut *mut int (dbg_grc_size)(struct qed_dev,
    pub buffer): *mut *mut *mut int (dbg_all_data)(struct qed_dev cdev, void,
    pub cdev): *mut *mut int (dbg_all_data_size)(struct qed_dev,
    pub err_type): qed_hw_err_type,
//
// can_link_change(): can the instance change the link or not.
//
// @cdev: Qed dev pointer.
//
// Return: true if link-change is allowed, false otherwise.
//
    pub cdev): *mut *mut bool (can_link_change)(struct qed_dev,
//
// set_link(): set links according to params.
//
// @cdev: Qed dev pointer.
// @params: values used to override the default link configuration.
//
// Return: 0 on success, error otherwise.
//
    pub params): *mut qed_link_params,
//
// get_link(): returns the current link state.
//
// @cdev: Qed dev pointer.
// @if_link: structure to be filled with current link configuration.
//
// Return: Void.
//
    pub if_link): *mut qed_link_output,
//
// drain(): drains chip in case Tx completions fail to arrive due to pause.
//
// @cdev: Qed dev pointer.
//
// Return: Int.
//
    pub cdev): *mut *mut int (drain)(struct qed_dev,
//
// update_msglvl(): update module debug level.
//
// @cdev: Qed dev pointer.
// @dp_module: Debug module.
// @dp_level: Debug level.
//
// Return: Void.
//
    pub dp_level): u8,
    pub params): *mut qed_chain_init_params,
    pub p_chain): *mut qed_chain,
//
// nvm_flash(): Flash nvm data.
//
// @cdev: Qed dev pointer.
// @name: file containing the data.
//
// Return: 0 on success, error otherwise.
//
    pub name): *const *const *const int (nvm_flash)(struct qed_dev cdev, char,
//
// nvm_get_image(): reads an entire image from nvram.
//
// @cdev: Qed dev pointer.
// @type: type of the request nvram image.
// @buf: preallocated buffer to fill with the image.
// @len: length of the allocated buffer.
//
// Return: 0 on success, error otherwise.
//
    pub len): *mut *mut qed_nvm_images type, u8 buf, u16,
//
// set_coalesce(): Configure Rx coalesce value in usec.
//
// @cdev: Qed dev pointer.
// @rx_coal: Rx coalesce value in usec.
// @tx_coal: Tx coalesce value in usec.
// @handle: Handle.
//
// Return: 0 on success, error otherwise.
//
    pub handle): *mut u16 rx_coal, u16 tx_coal, void,
//
// set_led() - Configure LED mode.
//
// @cdev: Qed dev pointer.
// @mode: LED mode.
//
// Return: 0 on success, error otherwise.
//
    pub mode): qed_led_mode,
//
// attn_clr_enable(): Prevent attentions from being reasserted.
//
// @cdev: Qed dev pointer.
// @clr_enable: Clear enable.
//
// Return: Void.
//
    pub clr_enable): *mut *mut *mut void (attn_clr_enable)(struct qed_dev cdev, bool,
//
// db_recovery_add(): add doorbell information to the doorbell
// recovery mechanism.
//
// @cdev: Qed dev pointer.
// @db_addr: Doorbell address.
// @db_data: Dddress of where db_data is stored.
// @db_width: Doorbell is 32b or 64b.
// @db_space: Doorbell recovery addresses are user or kernel space.
//
// Return: Int.
//
    pub db_space): qed_db_rec_space,
//
// db_recovery_del(): remove doorbell information from the doorbell
// recovery mechanism. db_data serves as key (db_addr is not unique).
//
// @cdev: Qed dev pointer.
// @db_addr: Doorbell address.
// @db_data: Address where db_data is stored. Serves as key for the
// entry to delete.
//
// Return: Int.
//
    pub db_data): *mut *mut void __iomem db_addr, void,
//
// recovery_process(): Trigger a recovery process.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (recovery_process)(struct qed_dev,
//
// recovery_prolog(): Execute the prolog operations of a recovery process.
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, error otherwise.
//
    pub cdev): *mut *mut int (recovery_prolog)(struct qed_dev,
//
// update_drv_state(): API to inform the change in the driver state.
//
// @cdev: Qed dev pointer.
// @active: Active
//
// Return: Int.
//
    pub active): *mut *mut *mut int (update_drv_state)(struct qed_dev cdev, bool,
//
// update_mac(): API to inform the change in the mac address.
//
// @cdev: Qed dev pointer.
// @mac: MAC.
//
// Return: Int.
//
    pub mac): *const *const *const int (update_mac)(struct qed_dev cdev, u8,
//
// update_mtu(): API to inform the change in the mtu.
//
// @cdev: Qed dev pointer.
// @mtu: MTU.
//
// Return: Int.
//
    pub mtu): *mut *mut *mut int (update_mtu)(struct qed_dev cdev, u16,
//
// update_wol(): Update of changes in the WoL configuration.
//
// @cdev: Qed dev pointer.
// @enabled: true iff WoL should be enabled.
//
// Return: Int.
//
    pub enabled): *mut *mut *mut int (update_wol) (struct qed_dev cdev, bool,
//
// read_module_eeprom(): Read EEPROM.
//
// @cdev: Qed dev pointer.
// @buf: buffer.
// @dev_addr: PHY device memory region.
// @offset: offset into eeprom contents to be read.
// @len: buffer length, i.e., max bytes to be read.
//
// Return: Int.
//
    pub len): *mut *mut char buf, u8 dev_addr, u32 offset, u32,
//
// get_affin_hwfn_idx(): Get affine HW function.
//
// @cdev: Qed dev pointer.
//
// Return: u8.
//
    pub cdev): *mut *mut u8 (get_affin_hwfn_idx)(struct qed_dev,
//
// read_nvm_cfg(): Read NVM config attribute value.
//
// @cdev: Qed dev pointer.
// @buf: Buffer.
// @cmd: NVM CFG command id.
// @entity_id: Entity id.
//
// Return: Int.
//
    pub entity_id): u32,
//
// read_nvm_cfg_len(): Read NVM config attribute value.
//
// @cdev: Qed dev pointer.
// @cmd: NVM CFG command id.
//
// Return: config id length, 0 on error.
//
    pub cmd): *mut *mut *mut int (read_nvm_cfg_len)(struct qed_dev cdev, u32,
//
// set_grc_config(): Configure value for grc config id.
//
// @cdev: Qed dev pointer.
// @cfg_id: grc config id
// @val: grc config value
//
// Return: Int.
//
    pub val): *mut *mut *mut int (set_grc_config)(struct qed_dev cdev, u32 cfg_id, u32,
    pub cdev): *mut *mut *mut devlink (devlink_register)(qed_dev,
    pub devlink): *mut *mut void (devlink_unregister)(struct devlink,
    pub ...): *mut *mut *mut *mut __printf(2, 3) void (mfw_report)(struct qed_dev cdev, char fmt,,
    pub sb_dbg): *mut u16 qid, struct qed_sb_info_dbg,
    pub esl_active): *mut *mut *mut int (get_esl_status)(struct qed_dev cdev, bool,
}

// Debug print definitions

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DP_LEVEL {
    QED_LEVEL_VERBOSE	= 0x0,
    QED_LEVEL_INFO		= 0x1,
    QED_LEVEL_NOTICE	= 0x2,
    QED_LEVEL_ERR		= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DP_MODULE {
    QED_MSG_SPQ	= 0x10000,
    QED_MSG_STATS	= 0x20000,
    QED_MSG_DCB	= 0x40000,
    QED_MSG_IOV	= 0x80000,
    QED_MSG_SP	= 0x100000,
    QED_MSG_STORAGE = 0x200000,
    QED_MSG_CXT	= 0x800000,
    QED_MSG_LL2	= 0x1000000,
    QED_MSG_ILT	= 0x2000000,
    QED_MSG_RDMA	= 0x4000000,
    QED_MSG_DEBUG	= 0x8000000,
// to be added...up to 0x8000000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_mf_mode {
    QED_MF_DEFAULT,
    QED_MF_OVLAN,
    QED_MF_NPAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_stats_common {
    pub no_buff_discards: u64,
    pub packet_too_big_discard: u64,
    pub ttl0_discard: u64,
    pub rx_ucast_bytes: u64,
    pub rx_mcast_bytes: u64,
    pub rx_bcast_bytes: u64,
    pub rx_ucast_pkts: u64,
    pub rx_mcast_pkts: u64,
    pub rx_bcast_pkts: u64,
    pub mftag_filter_discards: u64,
    pub mac_filter_discards: u64,
    pub gft_filter_drop: u64,
    pub tx_ucast_bytes: u64,
    pub tx_mcast_bytes: u64,
    pub tx_bcast_bytes: u64,
    pub tx_ucast_pkts: u64,
    pub tx_mcast_pkts: u64,
    pub tx_bcast_pkts: u64,
    pub tx_err_drop_pkts: u64,
    pub tpa_coalesced_pkts: u64,
    pub tpa_coalesced_events: u64,
    pub tpa_aborts_num: u64,
    pub tpa_not_coalesced_pkts: u64,
    pub tpa_coalesced_bytes: u64,
// port
    pub rx_64_byte_packets: u64,
    pub rx_65_to_127_byte_packets: u64,
    pub rx_128_to_255_byte_packets: u64,
    pub rx_256_to_511_byte_packets: u64,
    pub rx_512_to_1023_byte_packets: u64,
    pub rx_1024_to_1518_byte_packets: u64,
    pub rx_crc_errors: u64,
    pub rx_mac_crtl_frames: u64,
    pub rx_pause_frames: u64,
    pub rx_pfc_frames: u64,
    pub rx_align_errors: u64,
    pub rx_carrier_errors: u64,
    pub rx_oversize_packets: u64,
    pub rx_jabbers: u64,
    pub rx_undersize_packets: u64,
    pub rx_fragments: u64,
    pub tx_64_byte_packets: u64,
    pub tx_65_to_127_byte_packets: u64,
    pub tx_128_to_255_byte_packets: u64,
    pub tx_256_to_511_byte_packets: u64,
    pub tx_512_to_1023_byte_packets: u64,
    pub tx_1024_to_1518_byte_packets: u64,
    pub tx_pause_frames: u64,
    pub tx_pfc_frames: u64,
    pub brb_truncates: u64,
    pub brb_discards: u64,
    pub rx_mac_bytes: u64,
    pub rx_mac_uc_packets: u64,
    pub rx_mac_mc_packets: u64,
    pub rx_mac_bc_packets: u64,
    pub rx_mac_frames_ok: u64,
    pub tx_mac_bytes: u64,
    pub tx_mac_uc_packets: u64,
    pub tx_mac_mc_packets: u64,
    pub tx_mac_bc_packets: u64,
    pub tx_mac_ctrl_frames: u64,
    pub link_change_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_stats_bb {
    pub rx_1519_to_1522_byte_packets: u64,
    pub rx_1519_to_2047_byte_packets: u64,
    pub rx_2048_to_4095_byte_packets: u64,
    pub rx_4096_to_9216_byte_packets: u64,
    pub rx_9217_to_16383_byte_packets: u64,
    pub tx_1519_to_2047_byte_packets: u64,
    pub tx_2048_to_4095_byte_packets: u64,
    pub tx_4096_to_9216_byte_packets: u64,
    pub tx_9217_to_16383_byte_packets: u64,
    pub tx_lpi_entry_count: u64,
    pub tx_total_collisions: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_stats_ah {
    pub rx_1519_to_max_byte_packets: u64,
    pub tx_1519_to_max_byte_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_stats {
    pub common: qed_eth_stats_common,
    pub bb: qed_eth_stats_bb,
    pub ah: qed_eth_stats_ah,
}

pub const QED_SB_IDX: c_uint = 0x0002;
pub const RX_PI: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sb_cnt_info {
// Original, current, and free SBs for PF
    pub orig: c_int,
    pub cnt: c_int,
    pub free_cnt: c_int,
// Original, current and free SBS for child VFs
    pub iov_orig: c_int,
    pub iov_cnt: c_int,
    pub free_cnt_iov: c_int,
}

// Let SB update
//
// qed_sb_ack(): This function creates an update command for interrupts
// that is  written to the IGU.
//
// @sb_info: This is the structure allocated and
// initialized per status block. Assumption is
// that it was initialized using qed_sb_init
// @int_cmd: Enable/Disable/Nop
// @upd_flg: Whether igu consumer should be updated.
//
// Return: inline void.
//
// Both segments (interrupts & acks) are written to same place address;
// Need to guarantee all commands will be received (in-order) by HW.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_rss_caps {
    QED_RSS_IPV4		= 0x1,
    QED_RSS_IPV6		= 0x2,
    QED_RSS_IPV4_TCP	= 0x4,
    QED_RSS_IPV6_TCP	= 0x8,
    QED_RSS_IPV4_UDP	= 0x10,
    QED_RSS_IPV6_UDP	= 0x20,
}

pub const QED_RSS_IND_TABLE_SIZE: c_int = 128;

