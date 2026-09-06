//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hnae3.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2016-2017 Hisilicon Limited.
// Names used in this framework:
// ae handle (handle):
// a set of queues provided by AE
// ring buffer queue (rbq):
// the channel between upper layer and the AE, can do tx and rx
// ring:
// a tx or rx channel within a rbq
// ring description (desc):
// an element in the ring with packet information
// buffer:
// a memory region referred by desc with the full packet payload
//
// "num" means a static number set as a parameter, "count" mean a dynamic
// number set while running
// "cb" means control block
//

// Device version
pub const HNAE3_DEVICE_VERSION_V1: c_uint = 0x00020;
pub const HNAE3_DEVICE_VERSION_V2: c_uint = 0x00021;
pub const HNAE3_DEVICE_VERSION_V3: c_uint = 0x00030;
pub const HNAE3_PCI_REVISION_BIT_SIZE: c_int = 8;
// Device IDs
pub const HNAE3_DEV_ID_GE: c_uint = 0xA220;
pub const HNAE3_DEV_ID_25GE: c_uint = 0xA221;
pub const HNAE3_DEV_ID_25GE_RDMA: c_uint = 0xA222;
pub const HNAE3_DEV_ID_25GE_RDMA_MACSEC: c_uint = 0xA223;
pub const HNAE3_DEV_ID_50GE_RDMA: c_uint = 0xA224;
pub const HNAE3_DEV_ID_50GE_RDMA_MACSEC: c_uint = 0xA225;
pub const HNAE3_DEV_ID_100G_RDMA_MACSEC: c_uint = 0xA226;
pub const HNAE3_DEV_ID_200G_RDMA: c_uint = 0xA228;
pub const HNAE3_DEV_ID_VF: c_uint = 0xA22E;
pub const HNAE3_DEV_ID_RDMA_DCB_PFC_VF: c_uint = 0xA22F;
pub const HNAE3_CLASS_NAME_SIZE: c_int = 16;
pub const HNAE3_DEV_INITED_B: c_uint = 0x0;
pub const HNAE3_DEV_SUPPORT_ROCE_B: c_uint = 0x1;
pub const HNAE3_DEV_SUPPORT_DCB_B: c_uint = 0x2;
pub const HNAE3_KNIC_CLIENT_INITED_B: c_uint = 0x3;
pub const HNAE3_UNIC_CLIENT_INITED_B: c_uint = 0x4;
pub const HNAE3_ROCE_CLIENT_INITED_B: c_uint = 0x5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HNAE3_DEV_CAP_BITS {
    HNAE3_DEV_SUPPORT_FD_B,
    HNAE3_DEV_SUPPORT_GRO_B,
    HNAE3_DEV_SUPPORT_FEC_B,
    HNAE3_DEV_SUPPORT_UDP_GSO_B,
    HNAE3_DEV_SUPPORT_QB_B,
    HNAE3_DEV_SUPPORT_FD_FORWARD_TC_B,
    HNAE3_DEV_SUPPORT_PTP_B,
    HNAE3_DEV_SUPPORT_INT_QL_B,
    HNAE3_DEV_SUPPORT_HW_TX_CSUM_B,
    HNAE3_DEV_SUPPORT_TX_PUSH_B,
    HNAE3_DEV_SUPPORT_PHY_IMP_B,
    HNAE3_DEV_SUPPORT_TQP_TXRX_INDEP_B,
    HNAE3_DEV_SUPPORT_HW_PAD_B,
    HNAE3_DEV_SUPPORT_STASH_B,
    HNAE3_DEV_SUPPORT_UDP_TUNNEL_CSUM_B,
    HNAE3_DEV_SUPPORT_PAUSE_B,
    HNAE3_DEV_SUPPORT_RAS_IMP_B,
    HNAE3_DEV_SUPPORT_RXD_ADV_LAYOUT_B,
    HNAE3_DEV_SUPPORT_PORT_VLAN_BYPASS_B,
    HNAE3_DEV_SUPPORT_VLAN_FLTR_MDF_B,
    HNAE3_DEV_SUPPORT_MC_MAC_MNG_B,
    HNAE3_DEV_SUPPORT_CQ_B,
    HNAE3_DEV_SUPPORT_FEC_STATS_B,
    HNAE3_DEV_SUPPORT_LANE_NUM_B,
    HNAE3_DEV_SUPPORT_WOL_B,
    HNAE3_DEV_SUPPORT_TM_FLUSH_B,
    HNAE3_DEV_SUPPORT_VF_FAULT_B,
    HNAE3_DEV_SUPPORT_ERR_MOD_GEN_REG_B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HNAE3_PF_CAP_BITS {
    HNAE3_PF_SUPPORT_VLAN_FLTR_MDF_B = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_queue {
    pub io_base: *mut void __iomem,
    pub mem_base: *mut void __iomem,
    pub ae_algo: *mut hnae3_ae_algo,
    pub handle: *mut hnae3_handle,
    pub /: *mut *mut int tqp_index; / index in a handle,
    pub /: *mut *mut u32 buf_size; / size for hnae_desc->addr, preset by AE,
    pub /: *mut *mut u16 tx_desc_num; / total number of tx desc,
    pub /: *mut *mut u16 rx_desc_num; / total number of rx desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_mac_stats {
    pub tx_pause_cnt: u64,
    pub rx_pause_cnt: u64,
}

// hnae3 loop mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_loop {
    HNAE3_LOOP_EXTERNAL,
    HNAE3_LOOP_APP,
    HNAE3_LOOP_SERIAL_SERDES,
    HNAE3_LOOP_PARALLEL_SERDES,
    HNAE3_LOOP_PHY,
    HNAE3_LOOP_NONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_client_type {
    HNAE3_CLIENT_KNIC,
    HNAE3_CLIENT_ROCE,
}

// mac media type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_media_type {
    HNAE3_MEDIA_TYPE_UNKNOWN,
    HNAE3_MEDIA_TYPE_FIBER,
    HNAE3_MEDIA_TYPE_COPPER,
    HNAE3_MEDIA_TYPE_BACKPLANE,
    HNAE3_MEDIA_TYPE_NONE,
}

// must be consistent with definition in firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_module_type {
    HNAE3_MODULE_TYPE_UNKNOWN	= 0x00,
    HNAE3_MODULE_TYPE_FIBRE_LR	= 0x01,
    HNAE3_MODULE_TYPE_FIBRE_SR	= 0x02,
    HNAE3_MODULE_TYPE_AOC		= 0x03,
    HNAE3_MODULE_TYPE_CR		= 0x04,
    HNAE3_MODULE_TYPE_KR		= 0x05,
    HNAE3_MODULE_TYPE_TP		= 0x06,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_fec_mode {
    HNAE3_FEC_AUTO = 0,
    HNAE3_FEC_BASER,
    HNAE3_FEC_RS,
    HNAE3_FEC_LLRS,
    HNAE3_FEC_NONE,
    HNAE3_FEC_USER_DEF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_reset_notify_type {
    HNAE3_UP_CLIENT,
    HNAE3_DOWN_CLIENT,
    HNAE3_INIT_CLIENT,
    HNAE3_UNINIT_CLIENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_hw_error_type {
    HNAE3_PPU_POISON_ERROR,
    HNAE3_CMDQ_ECC_ERROR,
    HNAE3_IMP_RD_POISON_ERROR,
    HNAE3_ROCEE_AXI_RESP_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_reset_type {
    HNAE3_VF_RESET,
    HNAE3_VF_FUNC_RESET,
    HNAE3_VF_PF_FUNC_RESET,
    HNAE3_VF_FULL_RESET,
    HNAE3_FLR_RESET,
    HNAE3_FUNC_RESET,
    HNAE3_GLOBAL_RESET,
    HNAE3_IMP_RESET,
    HNAE3_NONE_RESET,
    HNAE3_VF_EXP_RESET,
    HNAE3_MAX_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_port_base_vlan_state {
    HNAE3_PORT_BASE_VLAN_DISABLE,
    HNAE3_PORT_BASE_VLAN_ENABLE,
    HNAE3_PORT_BASE_VLAN_MODIFY,
    HNAE3_PORT_BASE_VLAN_NOCHANGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_dbg_cmd {
    HNAE3_DBG_CMD_TM_NODES,
    HNAE3_DBG_CMD_TM_PRI,
    HNAE3_DBG_CMD_TM_QSET,
    HNAE3_DBG_CMD_TM_MAP,
    HNAE3_DBG_CMD_TM_PG,
    HNAE3_DBG_CMD_TM_PORT,
    HNAE3_DBG_CMD_TC_SCH_INFO,
    HNAE3_DBG_CMD_QOS_PAUSE_CFG,
    HNAE3_DBG_CMD_QOS_PRI_MAP,
    HNAE3_DBG_CMD_QOS_DSCP_MAP,
    HNAE3_DBG_CMD_QOS_BUF_CFG,
    HNAE3_DBG_CMD_DEV_INFO,
    HNAE3_DBG_CMD_TX_BD,
    HNAE3_DBG_CMD_RX_BD,
    HNAE3_DBG_CMD_MAC_UC,
    HNAE3_DBG_CMD_MAC_MC,
    HNAE3_DBG_CMD_MNG_TBL,
    HNAE3_DBG_CMD_LOOPBACK,
    HNAE3_DBG_CMD_PTP_INFO,
    HNAE3_DBG_CMD_INTERRUPT_INFO,
    HNAE3_DBG_CMD_RESET_INFO,
    HNAE3_DBG_CMD_IMP_INFO,
    HNAE3_DBG_CMD_NCL_CONFIG,
    HNAE3_DBG_CMD_REG_BIOS_COMMON,
    HNAE3_DBG_CMD_REG_SSU,
    HNAE3_DBG_CMD_REG_IGU_EGU,
    HNAE3_DBG_CMD_REG_RPU,
    HNAE3_DBG_CMD_REG_NCSI,
    HNAE3_DBG_CMD_REG_RTC,
    HNAE3_DBG_CMD_REG_PPP,
    HNAE3_DBG_CMD_REG_RCB,
    HNAE3_DBG_CMD_REG_TQP,
    HNAE3_DBG_CMD_REG_MAC,
    HNAE3_DBG_CMD_REG_DCB,
    HNAE3_DBG_CMD_VLAN_CONFIG,
    HNAE3_DBG_CMD_QUEUE_MAP,
    HNAE3_DBG_CMD_RX_QUEUE_INFO,
    HNAE3_DBG_CMD_TX_QUEUE_INFO,
    HNAE3_DBG_CMD_FD_TCAM,
    HNAE3_DBG_CMD_FD_COUNTER,
    HNAE3_DBG_CMD_FD_RULE,
    HNAE3_DBG_CMD_MAC_TNL_STATUS,
    HNAE3_DBG_CMD_SERV_INFO,
    HNAE3_DBG_CMD_UMV_INFO,
    HNAE3_DBG_CMD_PAGE_POOL_INFO,
    HNAE3_DBG_CMD_COAL_INFO,
    HNAE3_DBG_CMD_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_tc_map_mode {
    HNAE3_TC_MAP_MODE_PRIO,
    HNAE3_TC_MAP_MODE_DSCP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_vector_info {
    pub io_addr: *mut u8 __iomem,
    pub vector: c_int,
}

pub const HNAE3_RING_TYPE_B: c_int = 0;
pub const HNAE3_RING_TYPE_TX: c_int = 0;
pub const HNAE3_RING_TYPE_RX: c_int = 1;
pub const HNAE3_RING_GL_IDX_S: c_int = 0;

pub const HNAE3_RING_GL_RX: c_int = 0;
pub const HNAE3_RING_GL_TX: c_int = 1;
pub const HNAE3_FW_VERSION_BYTE3_SHIFT: c_int = 24;

pub const HNAE3_FW_VERSION_BYTE2_SHIFT: c_int = 16;

pub const HNAE3_FW_VERSION_BYTE1_SHIFT: c_int = 8;

pub const HNAE3_FW_VERSION_BYTE0_SHIFT: c_int = 0;

pub const HNAE3_SCC_VERSION_BYTE3_SHIFT: c_int = 24;

pub const HNAE3_SCC_VERSION_BYTE2_SHIFT: c_int = 16;

pub const HNAE3_SCC_VERSION_BYTE1_SHIFT: c_int = 8;

pub const HNAE3_SCC_VERSION_BYTE0_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_ring_chain_node {
    pub next: *mut hnae3_ring_chain_node,
    pub tqp_index: u32,
    pub flag: u32,
    pub int_gl_idx: u32,
}

// device specification info from firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_dev_specs {
    pub /: *mut *mut u32 mac_entry_num; / number of mac-vlan table entry,
    pub /: *mut *mut u32 mng_entry_num; / number of manager table entry,
    pub max_tm_rate: u32,
    pub rss_ind_tbl_size: u16,
    pub rss_key_size: u16,
    pub /: *mut *mut u16 int_ql_max; / max value of interrupt coalesce based on INT_QL,
    pub /: *mut *mut u16 max_int_gl; / max value of interrupt coalesce based on INT_GL,
    pub /: *mut *mut u8 max_non_tso_bd_num; / max BD number of one non-TSO packet,
    pub max_frm_size: u16,
    pub max_qset_num: u16,
    pub umv_size: u16,
    pub mc_mac_size: u16,
    pub mac_stats_num: u32,
    pub tnl_num: u8,
    pub hilink_version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_client_ops {
    pub handle): *mut *mut int (init_instance)(struct hnae3_handle,
    pub reset): *mut *mut *mut void (uninit_instance)(struct hnae3_handle handle, bool,
    pub state): *mut *mut *mut void (link_status_change)(struct hnae3_handle handle, bool,
    pub type): hnae3_reset_notify_type,
    pub hnae3_hw_error_type): enum,
}

pub const HNAE3_CLIENT_NAME_LENGTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_client {
    pub name: [c_char; HNAE3_CLIENT_NAME_LENGTH],
    pub state: c_ulong,
    pub type: hnae3_client_type,
    pub ops: *const hnae3_client_ops,
    pub node: list_head,
}

pub const HNAE3_DEV_CAPS_MAX_NUM: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_ae_dev {
    pub pdev: *mut pci_dev,
    pub ops: *const hnae3_ae_ops,
    pub node: list_head,
    pub flag: u32,
    pub hw_err_reset_req: c_ulong,
    pub dev_specs: hnae3_dev_specs,
    pub dev_version: u32,
    pub HNAE3_DEV_CAPS_MAX_NUM): DECLARE_BITMAP(caps,,
    pub priv: *mut c_void,
    pub handle: *mut hnae3_handle,
}

extern "C" {
    pub fn int(s: *mut *mut read_func)(struct seq_file, data: *mut c_void) -> typedef;
}
// This struct defines the operation on the handle.
//
// init_ae_dev(): (mandatory)
// Get PF configure from pci_dev and initialize PF hardware
// uninit_ae_dev()
// Disable PF device and release PF resource
// register_client
// Register client to ae_dev
// unregister_client()
// Unregister client from ae_dev
// start()
// Enable the hardware
// stop()
// Disable the hardware
// start_client()
// Inform the hclge that client has been started
// stop_client()
// Inform the hclge that client has been stopped
// get_status()
// Get the carrier state of the back channel of the handle, 1 for ok, 0 for
// non-ok
// get_ksettings_an_result()
// Get negotiation status,speed and duplex
// get_media_type()
// Get media type of MAC
// check_port_speed()
// Check target speed whether is supported
// adjust_link()
// Adjust link status
// set_loopback()
// Set loopback
// set_promisc_mode
// Set promisc mode
// request_update_promisc_mode
// request to hclge(vf) to update promisc mode
// set_mtu()
// set mtu
// get_pauseparam()
// get tx and rx of pause frame use
// set_pauseparam()
// set tx and rx of pause frame use
// set_autoneg()
// set auto autonegotiation of pause frame use
// get_autoneg()
// get auto autonegotiation of pause frame use
// restart_autoneg()
// restart autonegotiation
// halt_autoneg()
// halt/resume autonegotiation when autonegotiation on
// get_coalesce_usecs()
// get usecs to delay a TX interrupt after a packet is sent
// get_rx_max_coalesced_frames()
// get Maximum number of packets to be sent before a TX interrupt.
// set_coalesce_usecs()
// set usecs to delay a TX interrupt after a packet is sent
// set_coalesce_frames()
// set Maximum number of packets to be sent before a TX interrupt.
// get_mac_addr()
// get mac address
// set_mac_addr()
// set mac address
// add_uc_addr
// Add unicast addr to mac table
// rm_uc_addr
// Remove unicast addr from mac table
// set_mc_addr()
// Set multicast address
// add_mc_addr
// Add multicast address to mac table
// rm_mc_addr
// Remove multicast address from mac table
// update_stats()
// Update Old network device statistics
// get_mac_stats()
// get mac pause statistics including tx_cnt and rx_cnt
// get_ethtool_stats()
// Get ethtool network device statistics
// get_strings()
// Get a set of strings that describe the requested objects
// get_sset_count()
// Get number of strings that @get_strings will write
// update_led_status()
// Update the led status
// set_led_id()
// Set led id
// get_regs()
// Get regs dump
// get_regs_len()
// Get the len of the regs dump
// get_rss_key_size()
// Get rss key size
// get_rss()
// Get rss table
// set_rss()
// Set rss table
// get_tc_size()
// Get tc size of handle
// get_vector()
// Get vector number and vector information
// put_vector()
// Put the vector in hdev
// map_ring_to_vector()
// Map rings to vector
// unmap_ring_from_vector()
// Unmap rings from vector
// reset_queue()
// Reset queue
// get_fw_version()
// Get firmware version
// get_mdix_mode()
// Get media typr of phy
// enable_vlan_filter()
// Enable vlan filter
// set_vlan_filter()
// Set vlan filter config of Ports
// set_vf_vlan_filter()
// Set vlan filter config of vf
// enable_hw_strip_rxvtag()
// Enable/disable hardware strip vlan tag of packets received
// set_gro_en
// Enable/disable HW GRO
// add_arfs_entry
// Check the 5-tuples of flow, and create flow director rule
// get_vf_config
// Get the VF configuration setting by the host
// set_vf_link_state
// Set VF link status
// set_vf_spoofchk
// Enable/disable spoof check for specified vf
// set_vf_trust
// Enable/disable trust for specified vf, if the vf being trusted, then
// it can enable promisc mode
// set_vf_rate
// Set the max tx rate of specified vf.
// set_vf_mac
// Configure the default MAC for specified VF
// get_module_eeprom
// Get the optical module eeprom info.
// add_cls_flower
// Add clsflower rule
// del_cls_flower
// Delete clsflower rule
// cls_flower_active
// Check if any cls flower rule exist
// set_tx_hwts_info
// Save information for 1588 tx packet
// get_rx_hwts
// Get 1588 rx hwstamp
// get_ts_info
// Get phc info
// clean_vf_config
// Clean residual vf info after disable sriov
// get_wol
// Get wake on lan info
// set_wol
// Config wake on lan
// dbg_get_read_func
// Return the read func for debugfs seq file
// set_pfc_prevention_tout
// Set PFC storm prevention timeout
// get_pfc_prevention_tout
// Get PFC storm prevention timeout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_ae_ops {
    pub ae_dev): *mut *mut int (init_ae_dev)(struct hnae3_ae_dev,
    pub ae_dev): *mut *mut void (uninit_ae_dev)(struct hnae3_ae_dev,
    pub rst_type): hnae3_reset_type,
    pub ae_dev): *mut *mut void (reset_done)(struct hnae3_ae_dev,
    pub ae_dev): *mut hnae3_ae_dev,
    pub ae_dev): *mut hnae3_ae_dev,
    pub handle): *mut *mut int (start)(struct hnae3_handle,
    pub handle): *mut *mut void (stop)(struct hnae3_handle,
    pub handle): *mut *mut int (client_start)(struct hnae3_handle,
    pub handle): *mut *mut void (client_stop)(struct hnae3_handle,
    pub handle): *mut *mut int (get_status)(struct hnae3_handle,
    pub lane_num): *mut u32,
    pub lane_num): u8 duplex, u8,
    pub module_type): *mut u8,
    pub speed): *mut *mut *mut int (check_port_speed)(struct hnae3_handle handle, u32,
    pub fec_stats): *mut ethtool_fec_stats,
    pub fec_mode): *mut u8,
    pub fec_mode): *mut *mut *mut int (set_fec)(struct hnae3_handle handle, u32,
    pub duplex): *mut *mut *mut void (adjust_link)(struct hnae3_handle handle, int speed, int,
    pub en): hnae3_loop loop_mode, bool,
    pub en_mc_pmc): bool,
    pub handle): *mut *mut void (request_update_promisc_mode)(struct hnae3_handle,
    pub new_mtu): *mut *mut *mut int (set_mtu)(struct hnae3_handle handle, int,
    pub tx_en): *mut *mut *mut u32 auto_neg, u32 rx_en, u32,
    pub tx_en): u32 auto_neg, u32 rx_en, u32,
    pub enable): *mut *mut *mut int (set_autoneg)(struct hnae3_handle handle, bool,
    pub handle): *mut *mut int (get_autoneg)(struct hnae3_handle,
    pub handle): *mut *mut int (restart_autoneg)(struct hnae3_handle,
    pub halt): *mut *mut *mut int (halt_autoneg)(struct hnae3_handle handle, bool,
    pub rx_usecs): *mut *mut u32 tx_usecs, u32,
    pub rx_frames): *mut *mut u32 tx_frames, u32,
    pub timeout): *mut *mut *mut int (set_coalesce_usecs)(struct hnae3_handle handle, u32,
    pub coalesce_frames): u32,
    pub rx_usecs_high): *mut *mut u32 tx_usecs_high, u32,
    pub p): *mut *mut *mut void (get_mac_addr)(struct hnae3_handle handle, u8,
    pub is_first): bool,
    pub cmd): *mut *mut ifreq ifr, int,
    pub addr): *const c_uchar,
    pub addr): *const c_uchar,
    pub addr): *mut *mut *mut int (set_mc_addr)(struct hnae3_handle handle, void,
    pub addr): *const c_uchar,
    pub addr): *const c_uchar,
    pub enable): *mut *mut *mut void (set_tso_stats)(struct hnae3_handle handle, int,
    pub handle): *mut *mut void (update_stats)(struct hnae3_handle,
    pub data): *mut *mut *mut void (get_stats)(struct hnae3_handle handle, u64,
    pub mac_stats): *mut hns3_mac_stats,
    pub data): *mut u32 stringset, u8,
    pub stringset): *mut *mut *mut int (get_sset_count)(struct hnae3_handle handle, int,
    pub data): *mut c_void,
    pub handle): *mut *mut int (get_regs_len)(struct hnae3_handle,
    pub handle): *mut *mut u32 (get_rss_key_size)(struct hnae3_handle,
    pub hfunc): *mut u8,
    pub hfunc): *const *const u8 key, u8,
    pub cmd): *const ethtool_rxfh_fields,
    pub cmd): *mut ethtool_rxfh_fields,
    pub handle): *mut *mut int (get_tc_size)(struct hnae3_handle,
    pub vector_info): *mut hnae3_vector_info,
    pub vector_num): *mut *mut *mut int (put_vector)(struct hnae3_handle handle, int,
    pub vr_chain): *mut hnae3_ring_chain_node,
    pub vr_chain): *mut hnae3_ring_chain_node,
    pub handle): *mut *mut int (reset_queue)(struct hnae3_handle,
    pub handle): *mut *mut u32 (get_fw_version)(struct hnae3_handle,
    pub tp_mdix): *mut *mut u8 tp_mdix_ctrl, u8,
    pub enable): *mut *mut *mut int (enable_vlan_filter)(struct hnae3_handle handle, bool,
    pub is_kill): u16 vlan_id, bool,
    pub proto): u16 vlan, u8 qos, __be16,
    pub enable): *mut *mut *mut int (enable_hw_strip_rxvtag)(struct hnae3_handle handle, bool,
    pub handle): *mut *mut *mut void (reset_event)(struct pci_dev pdev, struct hnae3_handle,
    pub addr): *mut c_ulong,
    pub rst_type): hnae3_reset_type,
    pub ch): *mut ethtool_channels,
    pub max_rss_size): *mut *mut u16 alloc_tqps, u16,
    pub rxfh_configured): bool,
    pub flowctrl_adv): *mut u32,
    pub status): ethtool_phys_id_state,
    pub advertising): *mut c_ulong,
    pub cmd): *mut ethtool_rxnfc,
    pub cmd): *mut ethtool_rxnfc,
    pub cmd): *mut ethtool_rxnfc,
    pub cmd): *mut ethtool_rxnfc,
    pub rule_locs): *mut *mut ethtool_rxnfc cmd, u32,
    pub enable): *mut *mut *mut void (enable_fd)(struct hnae3_handle handle, bool,
    pub fkeys): *mut u16 flow_id, struct flow_keys,
    pub ae_dev): *mut *mut pci_ers_result_t (handle_hw_ras_error)(struct hnae3_ae_dev,
    pub handle): *mut *mut bool (get_hw_reset_stat)(struct hnae3_handle,
    pub handle): *mut *mut bool (ae_dev_resetting)(struct hnae3_handle,
    pub handle): *mut *mut unsigned long (ae_dev_reset_cnt)(struct hnae3_handle,
    pub enable): *mut *mut *mut int (set_gro_en)(struct hnae3_handle handle, bool,
    pub queue_id): *mut *mut *mut u16 (get_global_queue_id)(struct hnae3_handle handle, u16,
    pub enable): *mut *mut *mut void (set_timer_task)(struct hnae3_handle handle, bool,
    pub handle): *mut *mut int (mac_connect_phy)(struct hnae3_handle,
    pub handle): *mut *mut void (mac_disconnect_phy)(struct hnae3_handle,
    pub ivf): *mut ifla_vf_info,
    pub link_state): c_int,
    pub enable): bool,
    pub enable): *mut *mut *mut int (set_vf_trust)(struct hnae3_handle handle, int vf, bool,
    pub force): int min_tx_rate, int max_tx_rate, bool,
    pub p): *mut *mut *mut int (set_vf_mac)(struct hnae3_handle handle, int vf, u8,
    pub data): *mut u32 len, u8,
    pub handle): *mut *mut bool (get_cmdq_stat)(struct hnae3_handle,
    pub cls_flower): *mut flow_cls_offload,
    pub cls_flower): *mut flow_cls_offload,
    pub handle): *mut *mut bool (cls_flower_active)(struct hnae3_handle,
    pub cmd): *mut ethtool_link_ksettings,
    pub cmd): *const ethtool_link_ksettings,
    pub skb): *mut sk_buff,
    pub sec): u32 nsec, u32,
    pub info): *mut kernel_ethtool_ts_info,
    pub status_code): *mut u32,
    pub num_vfs): *mut *mut *mut void (clean_vf_config)(struct hnae3_ae_dev ae_dev, int,
    pub priority): *mut *mut u8 tc_map_mode, u8,
    pub wol): *mut ethtool_wolinfo,
    pub wol): *mut ethtool_wolinfo,
    pub func): *mut read_func,
    pub config): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,
    pub times): *mut *mut *mut int (set_pfc_prevention_tout)(struct hnae3_handle handle, u16,
    pub times): *mut *mut *mut int (get_pfc_prevention_tout)(struct hnae3_handle handle, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_dcb_ops {
// IEEE 802.1Qaz std
    pub ): *mut *mut *mut int (ieee_getets)(struct hnae3_handle , struct ieee_ets,
    pub ): *mut *mut *mut int (ieee_setets)(struct hnae3_handle , struct ieee_ets,
    pub ): *mut *mut *mut int (ieee_getpfc)(struct hnae3_handle , struct ieee_pfc,
    pub ): *mut *mut *mut int (ieee_setpfc)(struct hnae3_handle , struct ieee_pfc,
    pub app): *mut *mut *mut int (ieee_setapp)(struct hnae3_handle h, struct dcb_app,
    pub app): *mut *mut *mut int (ieee_delapp)(struct hnae3_handle h, struct dcb_app,
// DCBX configuration
    pub ): *mut *mut u8 (getdcbx)(struct hnae3_handle,
    pub u8): *mut *mut *mut u8 (setdcbx)(struct hnae3_handle ,,
    pub mqprio_qopt): *mut tc_mqprio_qopt_offload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_ae_algo {
    pub ops: *const hnae3_ae_ops,
    pub node: list_head,
    pub pdev_id_table: *const pci_device_id,
}

pub const HNAE3_INT_NAME_LEN: c_int = 32;
pub const HNAE3_ITR_COUNTDOWN_START: c_int = 100;
pub const HNAE3_MAX_TC: c_int = 8;
pub const HNAE3_MAX_USER_PRIO: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_tc_info {
    pub /: *mut *mut u8 prio_tc[HNAE3_MAX_USER_PRIO]; / TC indexed by prio,
    pub tqp_count: [u16; HNAE3_MAX_TC],
    pub tqp_offset: [u16; HNAE3_MAX_TC],
    pub /: *mut *mut u8 max_tc; / Total number of TCs,
    pub /: *mut *mut u8 num_tc; / Total number of enabled TCs,
    pub mqprio_active: bool,
    pub mqprio_destroy: bool,
    pub dcb_ets_active: bool,
}

pub const HNAE3_MAX_DSCP: c_int = 64;
pub const HNAE3_PRIO_ID_INVALID: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_knic_private_info {
    pub /: *mut *mut *mut net_device netdev; / Set by KNIC client when init instance,
    pub /: *mut *mut u16 rss_size; / Allocated RSS queues,
    pub req_rss_size: u16,
    pub rx_buf_len: u16,
    pub num_tx_desc: u16,
    pub num_rx_desc: u16,
    pub tx_spare_buf_size: u32,
    pub tc_info: hnae3_tc_info,
    pub tc_map_mode: u8,
    pub dscp_app_cnt: u8,
    pub dscp_prio: [u8; HNAE3_MAX_DSCP],
    pub /: *mut *mut u16 num_tqps; / total number of TQPs in this handle,
    pub /: *mut *mut *mut *mut hnae3_queue tqp; / array base of all TQPs in this instance,
    pub dcb_ops: *const hnae3_dcb_ops,
    pub int_rl_setting: u16,
    pub io_base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_roce_private_info {
    pub netdev: *mut net_device,
    pub roce_io_base: *mut void __iomem,
    pub roce_mem_base: *mut void __iomem,
    pub base_vector: c_int,
    pub num_vectors: c_int,
// The below attributes defined for RoCE client, hnae3 gives
// initial values to them, and RoCE client can modify and use
// them.
//
    pub reset_state: c_ulong,
    pub instance_state: c_ulong,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_pfc_storm_para {
    pub dir: u32,
    pub enable: u32,
    pub period_ms: u32,
    pub times: u32,
    pub recovery_period_ms: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae3_pflag {
    HNAE3_PFLAG_LIMIT_PROMISC,
    HNAE3_PFLAG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae3_handle {
    pub client: *mut hnae3_client,
    pub pdev: *mut pci_dev,
    pub priv: *mut c_void,
    pub /: *mut *mut *mut hnae3_ae_algo ae_algo; / the class who provides this handle,
    pub /: *mut *mut u64 flags; / Indicate the capabilities for this handle,
    pub /: *mut *mut *mut net_device netdev; / first member,
    pub kinfo: hnae3_knic_private_info,
    pub rinfo: hnae3_roce_private_info,
}

// Network interface message level enabled bits

pub const HNAE3_FORMAT_MAC_ADDR_LEN: c_int = 18;
pub const HNAE3_FORMAT_MAC_ADDR_OFFSET_0: c_int = 0;
pub const HNAE3_FORMAT_MAC_ADDR_OFFSET_4: c_int = 4;
pub const HNAE3_FORMAT_MAC_ADDR_OFFSET_5: c_int = 5;
extern "C" {
    pub fn hnae3_register_ae_dev(ae_dev: *mut hnae3_ae_dev) -> c_int;
}
extern "C" {
    pub fn hnae3_unregister_ae_dev(ae_dev: *mut hnae3_ae_dev);
}
extern "C" {
    pub fn hnae3_unregister_ae_algo_prepare(ae_algo: *mut hnae3_ae_algo);
}
extern "C" {
    pub fn hnae3_unregister_ae_algo(ae_algo: *mut hnae3_ae_algo);
}
extern "C" {
    pub fn hnae3_register_ae_algo(ae_algo: *mut hnae3_ae_algo);
}
extern "C" {
    pub fn hnae3_unregister_client(client: *mut hnae3_client);
}
extern "C" {
    pub fn hnae3_register_client(client: *mut hnae3_client) -> c_int;
}
extern "C" {
    pub fn hnae3_acquire_unload_lock();
}
extern "C" {
    pub fn hnae3_release_unload_lock();
}
