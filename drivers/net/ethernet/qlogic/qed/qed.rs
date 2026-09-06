//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed.h
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

pub const NAME_SIZE: c_int = 16;
pub const VER_SIZE: c_int = 16;
pub const QED_WFQ_UNIT: c_int = 100;

pub const QED_LLH_DONT_CARE: c_int = 0;
// cau states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_coalescing_mode {
    QED_COAL_MODE_DISABLE,
    QED_COAL_MODE_ENABLE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_nvm_cmd {
    QED_PUT_FILE_BEGIN = DRV_MSG_CODE_NVM_PUT_FILE_BEGIN,
    QED_PUT_FILE_DATA = DRV_MSG_CODE_NVM_PUT_FILE_DATA,
    QED_NVM_WRITE_NVRAM = DRV_MSG_CODE_NVM_WRITE_NVRAM,
    QED_GET_MCP_NVM_RESP = 0xFFFFFF00
}

// helpers

// forward
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rt_data {
    pub init_val: *mut u32,
    pub b_valid: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_tunn_mode {
    QED_MODE_L2GENEVE_TUNN,
    QED_MODE_IPGENEVE_TUNN,
    QED_MODE_L2GRE_TUNN,
    QED_MODE_IPGRE_TUNN,
    QED_MODE_VXLAN_TUNN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_tunn_clss {
    QED_TUNN_CLSS_MAC_VLAN,
    QED_TUNN_CLSS_MAC_VNI,
    QED_TUNN_CLSS_INNER_MAC_VLAN,
    QED_TUNN_CLSS_INNER_MAC_VNI,
    QED_TUNN_CLSS_MAC_VLAN_DUAL_STAGE,
    MAX_QED_TUNN_CLSS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tunn_update_type {
    pub b_update_mode: bool,
    pub b_mode_enabled: bool,
    pub tun_cls: qed_tunn_clss,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tunn_update_udp_port {
    pub b_update_port: bool,
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tunnel_info {
    pub vxlan: qed_tunn_update_type,
    pub l2_geneve: qed_tunn_update_type,
    pub ip_geneve: qed_tunn_update_type,
    pub l2_gre: qed_tunn_update_type,
    pub ip_gre: qed_tunn_update_type,
    pub vxlan_port: qed_tunn_update_udp_port,
    pub geneve_port: qed_tunn_update_udp_port,
    pub b_update_rx_cls: bool,
    pub b_update_tx_cls: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tunn_start_params {
    pub tunn_mode: c_ulong,
    pub vxlan_udp_port: u16,
    pub geneve_udp_port: u16,
    pub update_vxlan_udp_port: u8,
    pub update_geneve_udp_port: u8,
    pub tunn_clss_vxlan: u8,
    pub tunn_clss_l2geneve: u8,
    pub tunn_clss_ipgeneve: u8,
    pub tunn_clss_l2gre: u8,
    pub tunn_clss_ipgre: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tunn_update_params {
    pub tunn_mode_update_mask: c_ulong,
    pub tunn_mode: c_ulong,
    pub vxlan_udp_port: u16,
    pub geneve_udp_port: u16,
    pub update_rx_pf_clss: u8,
    pub update_tx_pf_clss: u8,
    pub update_vxlan_udp_port: u8,
    pub update_geneve_udp_port: u8,
    pub tunn_clss_vxlan: u8,
    pub tunn_clss_l2geneve: u8,
    pub tunn_clss_ipgeneve: u8,
    pub tunn_clss_l2gre: u8,
    pub tunn_clss_ipgre: u8,
}

// The PCI personality is not quite synonymous to protocol ID:
// 1. All personalities need CORE connections
// 2. The Ethernet personality may support also the RoCE/iWARP protocol
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_pci_personality {
    QED_PCI_ETH,
    QED_PCI_FCOE,
    QED_PCI_ISCSI,
    QED_PCI_NVMETCP,
    QED_PCI_ETH_ROCE,
    QED_PCI_ETH_IWARP,
    QED_PCI_ETH_RDMA,
    QED_PCI_DEFAULT, /* default in shmem */
}

// All VFs are symmetric, all counters are PF + all VFs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_qm_iids {
    pub cids: u32,
    pub vf_cids: u32,
    pub tids: u32,
}

// HW / FW resources, output of features supported below, most information
// is received from MFW.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_resources {
    QED_SB,
    QED_L2_QUEUE,
    QED_VPORT,
    QED_RSS_ENG,
    QED_PQ,
    QED_RL,
    QED_MAC,
    QED_VLAN,
    QED_RDMA_CNQ_RAM,
    QED_ILT,
    QED_LL2_RAM_QUEUE,
    QED_LL2_CTX_QUEUE,
    QED_CMDQS_CQS,
    QED_RDMA_STATS_QUEUE,
    QED_BDQ,
    QED_MAX_RESC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum QED_FEATURE {
    QED_PF_L2_QUE,
    QED_VF,
    QED_RDMA_CNQ,
    QED_NVMETCP_CQ,
    QED_ISCSI_CQ,
    QED_FCOE_CQ,
    QED_VF_L2_QUE,
    QED_MAX_FEATURES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_dev_cap {
    QED_DEV_CAP_ETH,
    QED_DEV_CAP_FCOE,
    QED_DEV_CAP_ISCSI,
    QED_DEV_CAP_ROCE,
    QED_DEV_CAP_IWARP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_wol_support {
    QED_WOL_SUPPORT_NONE,
    QED_WOL_SUPPORT_PME,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_db_rec_exec {
    DB_REC_DRY_RUN,
    DB_REC_REAL_DEAL,
    DB_REC_ONCE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_hw_info {
// PCI personality
    pub personality: qed_pci_personality,

// Resource Allocation scheme results
    pub resc_start: [u32; QED_MAX_RESC],
    pub resc_num: [u32; QED_MAX_RESC],    pub feat_num: [u32; QED_MAX_FEATURES],
// Amount of traffic classes HW supports
    pub num_hw_tc: u8,
// Amount of TCs which should be active according to DCBx or upper
// layer driver configuration.
//
    pub num_active_tc: u8,
    pub offload_tc: u8,
    pub offload_tc_set: bool,
    pub multi_tc_roce_en: bool,

    pub concrete_fid: u32,
    pub opaque_fid: u16,
    pub ovlan: u16,
    pub part_num: [u32; 4],
    pub hw_mac_addr: [c_uchar; ETH_ALEN],
    pub node_wwn: u64,
    pub port_wwn: u64,
    pub num_fcoe_conns: u16,
    pub p_igu_info: *mut qed_igu_info,
    pub hw_mode: u32,
    pub device_capabilities: c_ulong,
    pub mtu: u16,
    pub b_wol_support: qed_wol_support,
}

// maximun size of read/write commands (HW limit)
pub const DMAE_MAX_RW_SIZE: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dmae_info {
// Mutex for synchronizing access to functions
    pub mutex: mutex,
    pub channel: u8,
    pub completion_word_phys_addr: dma_addr_t,
// The memory location where the DMAE writes the completion
// value when an operation is finished on this context.
//
    pub p_completion_word: *mut u32,
    pub intermediate_buffer_phys_addr: dma_addr_t,
// An intermediate buffer for DMAE operations that use virtual
// addresses - data is DMA'd to/from this buffer and then
// memcpy'd to/from the virtual address
//
    pub p_intermediate_buffer: *mut u32,
    pub dmae_cmd_phys_addr: dma_addr_t,
    pub p_dmae_cmd: *mut dmae_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_wfq_data {
// when feature is configured for at least 1 vport
    pub min_speed: u32,
    pub configured: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_qm_info {
    pub qm_pq_params: *mut init_qm_pq_params,
    pub qm_vport_params: *mut init_qm_vport_params,
    pub qm_port_params: *mut init_qm_port_params,
    pub start_pq: u16,
    pub start_vport: u8,
    pub pure_lb_pq: u16,
    pub first_ofld_pq: u16,
    pub first_llt_pq: u16,
    pub pure_ack_pq: u16,
    pub ooo_pq: u16,
    pub first_vf_pq: u16,
    pub first_mcos_pq: u16,
    pub first_rl_pq: u16,
    pub num_pqs: u16,
    pub num_vf_pqs: u16,
    pub num_vports: u8,
    pub max_phys_tcs_per_port: u8,
    pub ooo_tc: u8,
    pub pf_rl_en: bool,
    pub pf_wfq_en: bool,
    pub vport_rl_en: bool,
    pub vport_wfq_en: bool,
    pub pf_wfq: u8,
    pub pf_rl: u32,
    pub wfq_data: *mut qed_wfq_data,
    pub num_pf_rls: u8,
}

pub const QED_OVERFLOW_BIT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_db_recovery_info {
    pub list: list_head,
// Lock to protect the doorbell recovery mechanism list
    pub lock: spinlock_t,
    pub dorq_attn: bool,
    pub db_recovery_counter: u32,
    pub overflow: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct storm_stats {
    pub address: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_storm_stats {
    pub mstats: storm_stats,
    pub pstats: storm_stats,
    pub tstats: storm_stats,
    pub ustats: storm_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fw_data {
    pub fw_ver_info: *mut fw_ver_info,
    pub modes_tree_buf: *const u8,
    pub init_ops: *mut init_op,
    pub arr_data: *const u32,
    pub fw_overlays: *const u32,
    pub fw_overlays_len: u32,
    pub init_ops_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_mf_mode_bit {
// Supports PF-classification based on tag
    QED_MF_OVLAN_CLSS,

// Supports PF-classification based on MAC
    QED_MF_LLH_MAC_CLSS,

// Supports PF-classification based on protocol type
    QED_MF_LLH_PROTO_CLSS,

// Requires a default PF to be set
    QED_MF_NEED_DEF_PF,

// Allow LL2 to multicast/broadcast
    QED_MF_LL2_NON_UNICAST,

// Allow Cross-PF [& child VFs] Tx-switching
    QED_MF_INTER_PF_SWITCH,

// Unified Fabtic Port support enabled
    QED_MF_UFP_SPECIFIC,

// Disable Accelerated Receive Flow Steering (aRFS)
    QED_MF_DISABLE_ARFS,

// Use vlan for steering
    QED_MF_8021Q_TAGGING,

// Use stag for steering
    QED_MF_8021AD_TAGGING,

// Allow DSCP to TC mapping
    QED_MF_DSCP_TO_TC_MAP,

// Do not insert a vlan tag with id 0
    QED_MF_DONT_ADD_VLAN0_TAG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ufp_mode {
    QED_UFP_MODE_ETS,
    QED_UFP_MODE_VNIC_BW,
    QED_UFP_MODE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ufp_pri_type {
    QED_UFP_PRI_OS,
    QED_UFP_PRI_VNIC,
    QED_UFP_PRI_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ufp_info {
    pub pri_type: qed_ufp_pri_type,
    pub mode: qed_ufp_mode,
    pub tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BAR_ID {
    BAR_ID_0,		/* used for GRC */
    BAR_ID_1		/* Used for doorbells */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvm_image_info {
    pub num_images: u32,
    pub image_att: *mut bist_nvm_image_att,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_hsi_def_type {
    QED_HSI_DEF_MAX_NUM_VFS,
    QED_HSI_DEF_MAX_NUM_L2_QUEUES,
    QED_HSI_DEF_MAX_NUM_PORTS,
    QED_HSI_DEF_MAX_SB_PER_PATH,
    QED_HSI_DEF_MAX_NUM_PFS,
    QED_HSI_DEF_MAX_NUM_VPORTS,
    QED_HSI_DEF_NUM_ETH_RSS_ENGINE,
    QED_HSI_DEF_MAX_QM_TX_QUEUES,
    QED_HSI_DEF_NUM_PXP_ILT_RECORDS,
    QED_HSI_DEF_NUM_RDMA_STATISTIC_COUNTERS,
    QED_HSI_DEF_MAX_QM_GLOBAL_RLS,
    QED_HSI_DEF_MAX_PBF_CMD_LINES,
    QED_HSI_DEF_MAX_BTB_BLOCKS,
    QED_NUM_HSI_DEFS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_simd_fp_handler {
    pub token: *mut c_void,
    pub func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_slowpath_wq_flag {
    QED_SLOWPATH_MFW_TLV_REQ,
    QED_SLOWPATH_PERIODIC_DB_REC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_hwfn {
    pub cdev: *mut qed_dev,
    pub /: *mut *mut u8 my_id; / ID inside the PF,

    pub engine*/: *mut *mut u8 rel_pf_id; / Relative to,
    pub abs_pf_id: u8,

    pub port_id: u8,
    pub b_active: bool,
    pub dp_module: u32,
    pub dp_level: u8,
    pub name: [c_char; NAME_SIZE],
    pub hw_init_done: bool,
    pub num_funcs_on_engine: u8,
    pub enabled_func_idx: u8,
// BAR access
    pub regview: *mut void __iomem,
    pub doorbells: *mut void __iomem,
    pub db_phys_addr: u64,
    pub db_size: c_ulong,
// PTT pool
    pub p_ptt_pool: *mut qed_ptt_pool,
// HW info
    pub hw_info: qed_hw_info,
// rt_array (for init-tool)
    pub rt_data: qed_rt_data,
// SPQ
    pub p_spq: *mut qed_spq,
// EQ
    pub p_eq: *mut qed_eq,
// Consolidate Q
    pub p_consq: *mut qed_consq,
// Slow-Path definitions
    pub sp_dpc: tasklet_struct,
    pub b_sp_dpc_enabled: bool,
    pub p_main_ptt: *mut qed_ptt,
    pub p_dpc_ptt: *mut qed_ptt,
// PTP will be used only by the leading function.
// Usage of all PTP-apis should be synchronized as result.
//
    pub p_ptp_ptt: *mut qed_ptt,
    pub p_sp_sb: *mut qed_sb_sp_info,
    pub p_sb_attn: *mut qed_sb_attn_info,
// Protocol related
    pub using_ll2: bool,
    pub p_ll2_info: *mut qed_ll2_info,
    pub p_ooo_info: *mut qed_ooo_info,
    pub p_rdma_info: *mut qed_rdma_info,
    pub p_iscsi_info: *mut qed_iscsi_info,
    pub p_nvmetcp_info: *mut qed_nvmetcp_info,
    pub p_fcoe_info: *mut qed_fcoe_info,
    pub pf_params: qed_pf_params,
    pub b_rdma_enabled_in_prs: bool,
    pub rdma_prs_search_reg: u32,
    pub p_cxt_mngr: *mut qed_cxt_mngr,
// Flag indicating whether interrupts are enabled or not
    pub b_int_enabled: bool,
    pub b_int_requested: bool,
// True if the driver requests for the link
    pub b_drv_link_init: bool,
    pub vf_iov_info: *mut qed_vf_iov,
    pub pf_iov_info: *mut qed_pf_iov,
    pub mcp_info: *mut qed_mcp_info,
    pub p_dcbx_info: *mut qed_dcbx_info,
    pub ufp_info: qed_ufp_info,
    pub dmae_info: qed_dmae_info,
// QM init
    pub qm_info: qed_qm_info,
    pub storm_stats: qed_storm_stats,
// Buffer for unzipping firmware data
    pub unzip_buf: *mut c_void,
    pub dbg_info: dbg_tools_data,
    pub dbg_user_info: *mut c_void,
    pub dbg_arrays: [virt_mem_desc; MAX_BIN_DBG_BUFFER_TYPE],
// PWM region specific data
    pub wid_count: u16,
    pub dpi_size: u32,
    pub dpi_count: u32,
// This is used to calculate the doorbell address
    pub dpi_start_offset: u32,
// If one of the following is set then EDPM shouldn't be used
    pub dcbx_no_edpm: u8,
    pub db_bar_no_edpm: u8,
// L2-related
    pub p_l2_info: *mut qed_l2_info,
// Mechanism for recovering from doorbell drop
    pub db_recovery_info: qed_db_recovery_info,
// Nvm images number and attributes
    pub nvm_info: qed_nvm_image_info,
    pub fw_overlay_mem: *mut phys_mem_desc,
    pub p_arfs_ptt: *mut qed_ptt,
    pub simd_proto_handler: [qed_simd_fp_handler; 64],
    pub iov_wq: *mut workqueue_struct,
    pub iov_task: delayed_work,
    pub iov_task_flags: c_ulong,

    pub stream: *mut z_stream_s,
    pub slowpath_wq_active: bool,
    pub slowpath_wq: *mut workqueue_struct,
    pub slowpath_task: delayed_work,
    pub slowpath_task_flags: c_ulong,
    pub periodic_db_rec_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_params {
    pub mem_start: c_ulong,
    pub mem_end: c_ulong,
    pub irq: c_uint,
    pub pf_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_int_param {
    pub int_mode: u32,
    pub num_vectors: u8,
    pub /: *mut *mut u8 min_msix_cnt; / for minimal functionality,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_int_params {
    pub in: qed_int_param,
    pub out: qed_int_param,
    pub msix_table: *mut msix_entry,
    pub fp_initialized: bool,
    pub fp_msix_base: u8,
    pub fp_msix_cnt: u8,
    pub rdma_msix_base: u8,
    pub rdma_msix_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dbg_feature {
    pub dentry: *mut dentry,
    pub dump_buf: *mut u8,
    pub buf_size: u32,
    pub dumped_dwords: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev {
    pub dp_module: u32,
    pub dp_level: u8,
    pub name: [c_char; NAME_SIZE],
    pub type: qed_dev_type,
// Translate type/revision combo into the proper conditions

    pub vendor_id: u16,
    pub device_id: u16,
pub const QED_DEV_ID_MASK: c_uint = 0xff00;
pub const QED_DEV_ID_MASK_BB: c_uint = 0x1600;
pub const QED_DEV_ID_MASK_AH: c_uint = 0x8000;
    pub chip_num: u16,
pub const CHIP_NUM_MASK: c_uint = 0xffff;
pub const CHIP_NUM_SHIFT: c_int = 16;
    pub chip_rev: u16,
pub const CHIP_REV_MASK: c_uint = 0xf;
pub const CHIP_REV_SHIFT: c_int = 12;

    pub chip_metal: u16,
pub const CHIP_METAL_MASK: c_uint = 0xff;
pub const CHIP_METAL_SHIFT: c_int = 4;
    pub chip_bond_id: u16,
pub const CHIP_BOND_ID_MASK: c_uint = 0xf;
pub const CHIP_BOND_ID_SHIFT: c_int = 0;
    pub num_engines: u8,
    pub num_ports: u8,
    pub num_ports_in_engine: u8,
    pub num_funcs_in_port: u8,
    pub path_id: u8,
    pub mf_bits: c_ulong,
    pub pcie_width: c_int,
    pub pcie_speed: c_int,
// Add MF related configuration
    pub mcp_rev: u8,
    pub boot_mode: u8,
// WoL related configurations
    pub wol_config: u8,
    pub wol_mac: [u8; ETH_ALEN],
    pub int_mode: u32,
    pub int_coalescing_mode: qed_coalescing_mode,
    pub rx_coalesce_usecs: u16,
    pub tx_coalesce_usecs: u16,
// Start Bar offset of first hwfn
    pub regview: *mut void __iomem,
    pub doorbells: *mut void __iomem,
    pub db_phys_addr: u64,
    pub db_size: c_ulong,
// PCI
    pub cache_shift: u8,
// Init
    pub iro_arr: *const u32,

// HW functions
    pub num_hwfns: u8,
    pub hwfns: [qed_hwfn; MAX_HWFNS_PER_DEVICE],
// Engine affinity
    pub l2_affin_hint: u8,
    pub fir_affin: u8,
    pub iwarp_affin: u8,
// SRIOV
    pub p_iov_info: *mut qed_hw_sriov_info,

    pub tunnel: qed_tunnel_info,
    pub b_is_vf: bool,
    pub drv_type: u32,
    pub reset_stats: *mut qed_eth_stats,
    pub fw_data: *mut qed_fw_data,
    pub mcp_nvm_resp: u32,
// Recovery
    pub recov_in_prog: bool,
// Indicates whether should prevent attentions from being reasserted
    pub attn_clr_en: bool,
// LLH info
    pub ppfid_bitmap: u8,
    pub p_llh_info: *mut qed_llh_info,
// Linux specific here
    pub common_dev_info: qed_dev_info,
    pub edev: *mut qede_dev,
    pub pdev: *mut pci_dev,
    pub flags: u32,

    pub msg_enable: c_int,
    pub pci_params: pci_params,
    pub int_params: qed_int_params,
    pub protocol: u8,

// Callbacks to protocol driver
    pub common: *mut qed_common_cb_ops,
    pub eth: *mut qed_eth_cb_ops,
    pub fcoe: *mut qed_fcoe_cb_ops,
    pub iscsi: *mut qed_iscsi_cb_ops,
    pub nvmetcp: *mut qed_nvmetcp_cb_ops,
    pub protocol_ops: },
    pub ops_cookie: *mut c_void,

    pub ll2: *mut qed_cb_ll2_info,
    pub ll2_mac_address: [u8; ETH_ALEN],    pub dbg_features: [qed_dbg_feature; DBG_FEATURE_NUM],
    pub engine_for_debug: u8,
    pub disable_ilt_dump: bool,
    pub dbg_bin_dump: bool,
    pub 10): DECLARE_HASHTABLE(connections,,
    pub firmware: *const firmware,
    pub print_dbg_data: bool,
    pub rdma_max_sge: u32,
    pub rdma_max_inline: u32,
    pub rdma_max_srq_sge: u32,
    pub tunn_feature_mask: u16,
    pub iwarp_cmt: bool,
}

extern "C" {
    pub fn qed_get_hsi_def_val(cdev: *mut qed_dev, type: qed_hsi_def_type) -> u32;
}

//
// qed_concrete_to_sw_fid(): Get the sw function id from
// the concrete value.
//
// @cdev: Qed dev pointer.
// @concrete_fid: Concrete fid.
//
// Return: inline u8.
//
pub const PKT_LB_TC: c_int = 9;
extern "C" {
    pub fn qed_configure_vport_wfq(cdev: *mut qed_dev, vp_id: u16, rate: u32) -> c_int;
}
extern "C" {
    pub fn qed_clean_wfq_db(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt);
}

// Macros for getting the engine-affinitized hwfn (FIR: fcoe,iscsi,roce)

// Flags for indication of required queues

// physical queue index for cm context initialization
extern "C" {
    pub fn qed_get_cm_pq_idx(p_hwfn: *mut qed_hwfn, pq_flags: u32) -> u16;
}
extern "C" {
    pub fn qed_get_cm_pq_idx_mcos(p_hwfn: *mut qed_hwfn, tc: u8) -> u16;
}
extern "C" {
    pub fn qed_get_cm_pq_idx_vf(p_hwfn: *mut qed_hwfn, vf: u16) -> u16;
}
extern "C" {
    pub fn qed_get_cm_pq_idx_ofld_mtc(p_hwfn: *mut qed_hwfn, tc: u8) -> u16;
}
extern "C" {
    pub fn qed_get_cm_pq_idx_llt_mtc(p_hwfn: *mut qed_hwfn, tc: u8) -> u16;
}
// doorbell recovery mechanism
extern "C" {
    pub fn qed_db_recovery_execute(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_edpm_enabled(p_hwfn: *mut qed_hwfn) -> bool;
}

// Other Linux specific common definitions

extern "C" {
    pub fn qed_device_num_ports(cdev: *mut qed_dev) -> c_int;
}
// Prototypes
extern "C" {
    pub fn qed_link_update(hwfn: *mut qed_hwfn, ptt: *mut qed_ptt);
}
extern "C" {
    pub fn qed_bw_update(hwfn: *mut qed_hwfn, ptt: *mut qed_ptt);
}
extern "C" {
    pub fn qed_recovery_process(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_schedule_recovery_handler(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_slowpath_irq_req(hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_slowpath_irq_sync(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_mfw_tlv_req(hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_hw_info_set_offload_tc(p_info: *mut qed_hw_info, tc: u8);
}
extern "C" {
    pub fn qed_periodic_db_rec_start(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_llh_add_src_tcp_port_filter(cdev: *mut qed_dev, src_port: u16) -> c_int;
}
extern "C" {
    pub fn qed_llh_add_dst_tcp_port_filter(cdev: *mut qed_dev, dest_port: u16) -> c_int;
}
extern "C" {
    pub fn qed_llh_remove_src_tcp_port_filter(cdev: *mut qed_dev, src_port: u16);
}
extern "C" {
    pub fn qed_llh_remove_dst_tcp_port_filter(cdev: *mut qed_dev, src_port: u16);
}
extern "C" {
    pub fn qed_llh_clear_all_filters(cdev: *mut qed_dev);
}
extern "C" {
    pub fn qed_get_epoch_time() -> c_ulong;
}
