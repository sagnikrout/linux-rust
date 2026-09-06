//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_type.h
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
// Copyright(c) 2013 - 2021 Intel Corporation.

pub const I40E_MAX_VSI_QP: c_int = 16;
pub const I40E_MAX_VF_VSI: c_int = 4;
pub const I40E_MAX_CHAINED_RX_BUFFERS: c_int = 5;
pub const I40E_MAX_PF_UDP_OFFLOAD_PORTS: c_int = 16;
// Max default timeout in ms,
pub const I40E_MAX_NVM_TIMEOUT: c_int = 18000;
// Max timeout in ms for the phy to respond
pub const I40E_MAX_PHY_TIMEOUT: c_int = 500;
// Switch from ms to the 1usec global time (this is the GTIME resolution)

// forward declaration
extern "C" {
    pub fn void(: *mut *mut I40E_ADMINQ_CALLBACK)(struct i40e_hw, : *mut libie_aq_desc) -> typedef;
}
// Data type manipulation macros.

// bitfields for Tx queue mapping in QTX_CTL
pub const I40E_QTX_CTL_VF_QUEUE: c_uint = 0x0;
pub const I40E_QTX_CTL_VM_QUEUE: c_uint = 0x1;
pub const I40E_QTX_CTL_PF_QUEUE: c_uint = 0x2;

pub const I40E_PHY_COM_REG_PAGE: c_uint = 0x1E;
pub const I40E_PHY_LED_LINK_MODE_MASK: c_uint = 0xF0;
pub const I40E_PHY_LED_MANUAL_ON: c_uint = 0x100;
pub const I40E_PHY_LED_PROV_REG_1: c_uint = 0xC430;
pub const I40E_PHY_LED_MODE_MASK: c_uint = 0xFFFF;
pub const I40E_PHY_LED_MODE_ORIG: c_uint = 0x80000000;
// These are structs for managing the hardware information and the operations.
// The structures of function pointers are filled out at init time when we
// know for sure exactly which hardware we're working with.  This gives us the
// flexibility of using the same main driver code but adapting to slightly
// different hardware needs as new parts are developed.  For this architecture,
// the Firmware and AdminQ are intended to insulate the driver from most of the
// future changes, but these structures will also do part of the job.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_mac_type {
    I40E_MAC_UNKNOWN = 0,
    I40E_MAC_XL710,
    I40E_MAC_X722,
    I40E_MAC_GENERIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_media_type {
    I40E_MEDIA_TYPE_UNKNOWN = 0,
    I40E_MEDIA_TYPE_FIBER,
    I40E_MEDIA_TYPE_BASET,
    I40E_MEDIA_TYPE_BACKPLANE,
    I40E_MEDIA_TYPE_CX4,
    I40E_MEDIA_TYPE_DA,
    I40E_MEDIA_TYPE_VIRTUAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_fc_mode {
    I40E_FC_NONE = 0,
    I40E_FC_RX_PAUSE,
    I40E_FC_TX_PAUSE,
    I40E_FC_FULL,
    I40E_FC_PFC,
    I40E_FC_DEFAULT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_set_fc_aq_failures {
    I40E_SET_FC_AQ_FAIL_NONE = 0,
    I40E_SET_FC_AQ_FAIL_GET = 1,
    I40E_SET_FC_AQ_FAIL_SET = 2,
    I40E_SET_FC_AQ_FAIL_UPDATE = 4,
    I40E_SET_FC_AQ_FAIL_SET_UPDATE = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_vsi_type {
    I40E_VSI_MAIN	= 0,
    I40E_VSI_VMDQ1	= 1,
    I40E_VSI_VMDQ2	= 2,
    I40E_VSI_CTRL	= 3,
    I40E_VSI_FCOE	= 4,
    I40E_VSI_MIRROR	= 5,
    I40E_VSI_SRIOV	= 6,
    I40E_VSI_FDIR	= 7,
    I40E_VSI_IWARP	= 8,
    I40E_VSI_TYPE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_queue_type {
    I40E_QUEUE_TYPE_RX = 0,
    I40E_QUEUE_TYPE_TX,
    I40E_QUEUE_TYPE_PE_CEQ,
    I40E_QUEUE_TYPE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_link_status {
    pub phy_type: i40e_aq_phy_type,
    pub link_speed: i40e_aq_link_speed,
    pub link_info: u8,
    pub an_info: u8,
    pub req_fec_info: u8,
    pub fec_info: u8,
    pub ext_info: u8,
    pub loopback: u8,
// is Link Status Event notification to SW enabled
    pub lse_enable: bool,
    pub max_frame_size: u16,
    pub crc_enable: bool,
    pub pacing: u8,
    pub requested_speeds: u8,
    pub module_type: [u8; 3],
// 1st byte: module identifier
pub const I40E_MODULE_TYPE_SFP: c_uint = 0x03;
// 3rd byte: ethernet compliance codes for 1G
pub const I40E_MODULE_TYPE_1000BASE_SX: c_uint = 0x01;
pub const I40E_MODULE_TYPE_1000BASE_LX: c_uint = 0x02;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_phy_info {
    pub link_info: i40e_link_status,
    pub link_info_old: i40e_link_status,
    pub get_link_info: bool,
    pub media_type: i40e_media_type,
// all the phy types the NVM is capable of
    pub phy_types: u64,
}

// Defining the macro I40E_TYPE_OFFSET to implement a bit shift for some
// PHY types. There is an unused bit (31) in the I40E_CAP_PHY_TYPE_* bit
// fields but no corresponding gap in the i40e_aq_phy_type enumeration. So,
// a shift is needed to adjust for this with values larger than 31. The
// only affected values are I40E_PHY_TYPE_25GBASE_*.
//
pub const I40E_PHY_TYPE_OFFSET: c_int = 1;

// Offset for 2.5G/5G PHY Types value to bit number conversion

pub const I40E_HW_CAP_MAX_GPIO: c_int = 30;
// Capabilities of a PF or a VF or the whole device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hw_capabilities {
    pub switch_mode: u32,
// Cloud filter modes:
// Mode1: Filter on L4 port only
// Mode2: Filter for non-tunneled traffic
// Mode3: Filter for tunnel traffic
//
pub const I40E_CLOUD_FILTER_MODE1: c_uint = 0x6;
pub const I40E_CLOUD_FILTER_MODE2: c_uint = 0x7;
pub const I40E_SWITCH_MODE_MASK: c_uint = 0xF;
    pub management_mode: u32,
    pub mng_protocols_over_mctp: u32,
    pub npar_enable: u32,
    pub os2bmc: u32,
    pub valid_functions: u32,
    pub sr_iov_1_1: bool,
    pub vmdq: bool,
    pub /: *mut *mut bool evb_802_1_qbg; / Edge Virtual Bridging,
    pub /: *mut *mut bool evb_802_1_qbh; / Bridge Port Extension,
    pub dcb: bool,
    pub fcoe: bool,
    pub /: *mut *mut bool iscsi; / Indicates iSCSI enabled,
    pub flex10_enable: bool,
    pub flex10_capable: bool,
    pub flex10_mode: u32,
    pub flex10_status: u32,
    pub sec_rev_disabled: bool,
    pub update_disabled: bool,
pub const I40E_NVM_MGMT_SEC_REV_DISABLED: c_uint = 0x1;
pub const I40E_NVM_MGMT_UPDATE_DISABLED: c_uint = 0x2;
    pub mgmt_cem: bool,
    pub ieee_1588: bool,
    pub iwarp: bool,
    pub fd: bool,
    pub fd_filters_guaranteed: u32,
    pub fd_filters_best_effort: u32,
    pub rss: bool,
    pub rss_table_size: u32,
    pub rss_table_entry_width: u32,
    pub led: [bool; I40E_HW_CAP_MAX_GPIO],
    pub sdp: [bool; I40E_HW_CAP_MAX_GPIO],
    pub nvm_image_type: u32,
    pub num_flow_director_filters: u32,
    pub num_vfs: u32,
    pub vf_base_id: u32,
    pub num_vsis: u32,
    pub num_rx_qp: u32,
    pub num_tx_qp: u32,
    pub base_queue: u32,
    pub num_msix_vectors: u32,
    pub num_msix_vectors_vf: u32,
    pub led_pin_num: u32,
    pub sdp_pin_num: u32,
    pub mdio_port_num: u32,
    pub mdio_port_mode: u32,
    pub rx_buf_chain_len: u8,
    pub enabled_tcmap: u32,
    pub maxtc: u32,
    pub wr_csr_prot: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_mac_info {
    pub type: i40e_mac_type,
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub port_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_aq_resources_ids {
    I40E_NVM_RESOURCE_ID = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_aq_resource_access_type {
    I40E_RESOURCE_READ = 1,
    I40E_RESOURCE_WRITE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_nvm_info {
    pub /: *mut *mut u64 hw_semaphore_timeout; / usec global time (GTIME resolution),
    pub /: *mut *mut u32 timeout; / [ms],
    pub /: *mut *mut u16 sr_size; / Shadow RAM size in words,
    pub present)*/: *mut *mut bool blank_nvm_mode; / is NVM empty (no FW,
    pub /: *mut *mut u16 version; / NVM package version,
    pub /: *mut *mut u32 eetrack; / NVM data version,
    pub /: *mut *mut u32 oem_ver; / OEM version info,
}

// definitions used in NVM update support
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_nvmupd_cmd {
    I40E_NVMUPD_INVALID,
    I40E_NVMUPD_READ_CON,
    I40E_NVMUPD_READ_SNT,
    I40E_NVMUPD_READ_LCB,
    I40E_NVMUPD_READ_SA,
    I40E_NVMUPD_WRITE_ERA,
    I40E_NVMUPD_WRITE_CON,
    I40E_NVMUPD_WRITE_SNT,
    I40E_NVMUPD_WRITE_LCB,
    I40E_NVMUPD_WRITE_SA,
    I40E_NVMUPD_CSUM_CON,
    I40E_NVMUPD_CSUM_SA,
    I40E_NVMUPD_CSUM_LCB,
    I40E_NVMUPD_STATUS,
    I40E_NVMUPD_EXEC_AQ,
    I40E_NVMUPD_GET_AQ_RESULT,
    I40E_NVMUPD_GET_AQ_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_nvmupd_state {
    I40E_NVMUPD_STATE_INIT,
    I40E_NVMUPD_STATE_READING,
    I40E_NVMUPD_STATE_WRITING,
    I40E_NVMUPD_STATE_INIT_WAIT,
    I40E_NVMUPD_STATE_WRITE_WAIT,
    I40E_NVMUPD_STATE_ERROR
}

// nvm_access definition and its masks/shifts need to be accessible to
// application, core driver, and shared code.  Where is the right file?
//
pub const I40E_NVM_READ: c_uint = 0xB;
pub const I40E_NVM_WRITE: c_uint = 0xC;
pub const I40E_NVM_MOD_PNT_MASK: c_uint = 0xFF;
pub const I40E_NVM_TRANS_SHIFT: c_int = 8;

pub const I40E_NVM_PRESERVATION_FLAGS_SHIFT: c_int = 12;

pub const I40E_NVM_PRESERVATION_FLAGS_SELECTED: c_uint = 0x01;
pub const I40E_NVM_PRESERVATION_FLAGS_ALL: c_uint = 0x02;
pub const I40E_NVM_CON: c_uint = 0x0;
pub const I40E_NVM_SNT: c_uint = 0x1;
pub const I40E_NVM_LCB: c_uint = 0x2;

pub const I40E_NVM_ERA: c_uint = 0x4;
pub const I40E_NVM_CSUM: c_uint = 0x8;
pub const I40E_NVM_AQE: c_uint = 0xe;
pub const I40E_NVM_EXEC: c_uint = 0xf;
pub const I40E_NVMUPD_MAX_DATA: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_nvm_access {
    pub command: u32,
    pub config: u32,
    pub /: *mut *mut u32 offset; / in bytes,
    pub /: *mut *mut u32 data_size; / in bytes,
    pub data: [u8; 1],
}

// (Q)SFP module access definitions
pub const I40E_I2C_EEPROM_DEV_ADDR: c_uint = 0xA0;
pub const I40E_I2C_EEPROM_DEV_ADDR2: c_uint = 0xA2;
pub const I40E_MODULE_REVISION_ADDR: c_uint = 0x01;
pub const I40E_MODULE_SFF_8472_COMP: c_uint = 0x5E;
pub const I40E_MODULE_SFF_8472_SWAP: c_uint = 0x5C;
pub const I40E_MODULE_SFF_ADDR_MODE: c_uint = 0x04;
pub const I40E_MODULE_SFF_DDM_IMPLEMENTED: c_uint = 0x40;
pub const I40E_MODULE_TYPE_QSFP_PLUS: c_uint = 0x0D;
pub const I40E_MODULE_TYPE_QSFP28: c_uint = 0x11;
pub const I40E_MODULE_QSFP_MAX_LEN: c_int = 640;
// PCI bus types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_bus_type {
    i40e_bus_type_unknown = 0,
    i40e_bus_type_pci,
    i40e_bus_type_pcix,
    i40e_bus_type_pci_express,
    i40e_bus_type_reserved
}

// PCI bus speeds
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_bus_speed {
    i40e_bus_speed_unknown	= 0,
    i40e_bus_speed_33	= 33,
    i40e_bus_speed_66	= 66,
    i40e_bus_speed_100	= 100,
    i40e_bus_speed_120	= 120,
    i40e_bus_speed_133	= 133,
    i40e_bus_speed_2500	= 2500,
    i40e_bus_speed_5000	= 5000,
    i40e_bus_speed_8000	= 8000,
    i40e_bus_speed_reserved
}

// PCI bus widths
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_bus_width {
    i40e_bus_width_unknown	= 0,
    i40e_bus_width_pcie_x1	= 1,
    i40e_bus_width_pcie_x2	= 2,
    i40e_bus_width_pcie_x4	= 4,
    i40e_bus_width_pcie_x8	= 8,
    i40e_bus_width_32	= 32,
    i40e_bus_width_64	= 64,
    i40e_bus_width_reserved
}

// Bus parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_bus_info {
    pub speed: i40e_bus_speed,
    pub width: i40e_bus_width,
    pub type: i40e_bus_type,
    pub func: u16,
    pub device: u16,
    pub lan_id: u16,
    pub bus_id: u16,
}

// Flow control (FC) parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_fc_info {
    pub /: *mut *mut i40e_fc_mode current_mode; / FC mode in effect,
    pub /: *mut *mut i40e_fc_mode requested_mode; / FC mode requested by caller,
}

pub const I40E_MAX_TRAFFIC_CLASS: c_int = 8;
pub const I40E_MAX_USER_PRIORITY: c_int = 8;
pub const I40E_DCBX_MAX_APPS: c_int = 32;
pub const I40E_LLDPDU_SIZE: c_int = 1500;
pub const I40E_TLV_STATUS_OPER: c_uint = 0x1;
pub const I40E_TLV_STATUS_SYNC: c_uint = 0x2;
pub const I40E_TLV_STATUS_ERR: c_uint = 0x4;
pub const I40E_CEE_OPER_MAX_APPS: c_int = 3;
pub const I40E_APP_PROTOID_FCOE: c_uint = 0x8906;
pub const I40E_APP_PROTOID_ISCSI: c_uint = 0x0cbc;
pub const I40E_APP_PROTOID_FIP: c_uint = 0x8914;
pub const I40E_APP_SEL_ETHTYPE: c_uint = 0x1;
pub const I40E_APP_SEL_TCPIP: c_uint = 0x2;
pub const I40E_CEE_APP_SEL_ETHTYPE: c_uint = 0x0;
pub const I40E_CEE_APP_SEL_TCPIP: c_uint = 0x1;
// CEE or IEEE 802.1Qaz ETS Configuration data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_dcb_ets_config {
    pub willing: u8,
    pub cbs: u8,
    pub maxtcs: u8,
    pub prioritytable: [u8; I40E_MAX_TRAFFIC_CLASS],
    pub tcbwtable: [u8; I40E_MAX_TRAFFIC_CLASS],
    pub tsatable: [u8; I40E_MAX_TRAFFIC_CLASS],
}

// CEE or IEEE 802.1Qaz PFC Configuration data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_dcb_pfc_config {
    pub willing: u8,
    pub mbc: u8,
    pub pfccap: u8,
    pub pfcenable: u8,
}

// CEE or IEEE 802.1Qaz Application Priority data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_dcb_app_priority_table {
    pub priority: u8,
    pub selector: u8,
    pub protocolid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_dcbx_config {
    pub dcbx_mode: u8,
pub const I40E_DCBX_MODE_CEE: c_uint = 0x1;
pub const I40E_DCBX_MODE_IEEE: c_uint = 0x2;
    pub app_mode: u8,
pub const I40E_DCBX_APPS_NON_WILLING: c_uint = 0x1;
    pub numapps: u32,
    pub /: *mut *mut u32 tlv_status; / CEE mode TLV status,
    pub etscfg: i40e_dcb_ets_config,
    pub etsrec: i40e_dcb_ets_config,
    pub pfc: i40e_dcb_pfc_config,
    pub app: [i40e_dcb_app_priority_table; I40E_DCBX_MAX_APPS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hw_flags {
    I40E_HW_CAP_AQ_SRCTL_ACCESS_ENABLE,
    I40E_HW_CAP_802_1AD,
    I40E_HW_CAP_AQ_PHY_ACCESS,
    I40E_HW_CAP_NVM_READ_REQUIRES_LOCK,
    I40E_HW_CAP_FW_LLDP_STOPPABLE,
    I40E_HW_CAP_FW_LLDP_PERSISTENT,
    I40E_HW_CAP_AQ_PHY_ACCESS_EXTENDED,
    I40E_HW_CAP_X722_FEC_REQUEST,
    I40E_HW_CAP_RSS_AQ,
    I40E_HW_CAP_128_QP_RSS,
    I40E_HW_CAP_ATR_EVICT,
    I40E_HW_CAP_WB_ON_ITR,
    I40E_HW_CAP_MULTI_TCP_UDP_RSS_PCTYPE,
    I40E_HW_CAP_NO_PCI_LINK_CHECK,
    I40E_HW_CAP_100M_SGMII,
    I40E_HW_CAP_NO_DCB_SUPPORT,
    I40E_HW_CAP_USE_SET_LLDP_MIB,
    I40E_HW_CAP_GENEVE_OFFLOAD,
    I40E_HW_CAP_PTP_L4,
    I40E_HW_CAP_WOL_MC_MAGIC_PKT_WAKE,
    I40E_HW_CAP_CRT_RETIMER,
    I40E_HW_CAP_OUTER_UDP_CSUM,
    I40E_HW_CAP_PHY_CONTROLS_LEDS,
    I40E_HW_CAP_STOP_FW_LLDP,
    I40E_HW_CAP_PORT_ID_VALID,
    I40E_HW_CAP_RESTART_AUTONEG,
    I40E_HW_CAPS_NBITS,
}

// Port hardware description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hw {
    pub hw_addr: *mut u8 __iomem,
// subsystem structs
    pub phy: i40e_phy_info,
    pub mac: i40e_mac_info,
    pub bus: i40e_bus_info,
    pub nvm: i40e_nvm_info,
    pub fc: i40e_fc_info,
// PBA ID
    pub pba_id: *const c_char,
// pci info
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub port: u8,
    pub adapter_stopped: bool,
// capabilities for entire device and PCI func
    pub dev_caps: i40e_hw_capabilities,
    pub func_caps: i40e_hw_capabilities,
// Flow Director shared filter space
    pub fdir_shared_filter_count: u16,
// device profile info
    pub pf_id: u8,
    pub main_vsi_seid: u16,
// for multi-function MACs
    pub partition_id: u16,
    pub num_partitions: u16,
    pub num_ports: u16,
// Closest numa node to the device
    pub numa_node: u16,
// Admin Queue info
    pub aq: i40e_adminq_info,
// state of nvm update process
    pub nvmupd_state: i40e_nvmupd_state,
    pub nvm_wb_desc: libie_aq_desc,
    pub nvm_aq_event_desc: libie_aq_desc,
    pub nvm_buff: i40e_virt_mem,
    pub nvm_release_on_done: bool,
    pub nvm_wait_opcode: u16,
// HMC info
    pub /: *mut *mut i40e_hmc_info hmc; / HMC info struct,
// LLDP/DCBX Status
    pub dcbx_status: u16,
// DCBX info
    pub /: *mut *mut i40e_dcbx_config local_dcbx_config; / Oper/Local Cfg,
    pub /: *mut *mut i40e_dcbx_config remote_dcbx_config; / Peer Cfg,
    pub /: *mut *mut i40e_dcbx_config desired_dcbx_config; / CEE Desired Cfg,
    pub I40E_HW_CAPS_NBITS): DECLARE_BITMAP(caps,,
// Used in set switch config AQ command
    pub switch_tag: u16,
    pub first_tag: u16,
    pub second_tag: u16,
// debug mask
    pub debug_mask: u32,
    pub err_str: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_driver_version {
    pub major_version: u8,
    pub minor_version: u8,
    pub build_version: u8,
    pub subbuild_version: u8,
    pub driver_string: [u8; 32],
}

// RX Descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub union i40e_16byte_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_16b_rx_wb_qw0 {
    pub mirroring_status: __le16,
    pub fcoe_ctx_id: __le16,
    pub mirr_fcoe: },
    pub l2tag1: __le16,
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le32 fd_id; / Flow director filter id,
    pub /: *mut *mut __le32 fcoe_param; / FCoE DDP Context id,
    pub hi_dword: },
    pub qword0: },
// ext status/error/pktype/length
    pub status_error_len: __le64,
    pub qword1: },
    pub /: *mut *mut } wb; / writeback,
    pub qword: [u64; 2],
    pub raw: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union i40e_32byte_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
// bit 0 of hdr_buffer_addr is DD bit
    pub rsvd1: __le64,
    pub rsvd2: __le64,
    pub read: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_32b_rx_wb_qw0 {
    pub mirroring_status: __le16,
    pub fcoe_ctx_id: __le16,
    pub mirr_fcoe: },
    pub l2tag1: __le16,
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le32 fcoe_param; / FCoE DDP Context id,
// Flow director filter id in case of
// Programming status desc WB
//
    pub fd_id: __le32,
    pub hi_dword: },
    pub qword0: },
// status/error/pktype/length
    pub status_error_len: __le64,
    pub qword1: },
    pub /: *mut *mut __le16 ext_status; / extended status,
    pub rsvd: __le16,
    pub l2tag2_1: __le16,
    pub l2tag2_2: __le16,
    pub qword2: },
    pub flex_bytes_lo: __le32,
    pub pe_status: __le32,
    pub lo_dword: },
    pub flex_bytes_hi: __le32,
    pub fd_id: __le32,
    pub hi_dword: },
    pub qword3: },
    pub /: *mut *mut } wb; / writeback,
    pub qword: [u64; 4],
    pub raw: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_desc_status_bits {
// Note: These are predefined bit offsets
    I40E_RX_DESC_STATUS_DD_SHIFT		= 0,
    I40E_RX_DESC_STATUS_EOF_SHIFT		= 1,
    I40E_RX_DESC_STATUS_L2TAG1P_SHIFT	= 2,
    I40E_RX_DESC_STATUS_L3L4P_SHIFT		= 3,
    I40E_RX_DESC_STATUS_CRCP_SHIFT		= 4,
    I40E_RX_DESC_STATUS_TSYNINDX_SHIFT	= 5, /* 2 BITS */
    I40E_RX_DESC_STATUS_TSYNVALID_SHIFT	= 7,
// Note: Bit 8 is reserved in X710 and XL710
    I40E_RX_DESC_STATUS_EXT_UDP_0_SHIFT	= 8,
    I40E_RX_DESC_STATUS_UMBCAST_SHIFT	= 9, /* 2 BITS */
    I40E_RX_DESC_STATUS_FLM_SHIFT		= 11,
    I40E_RX_DESC_STATUS_FLTSTAT_SHIFT	= 12, /* 2 BITS */
    I40E_RX_DESC_STATUS_LPBK_SHIFT		= 14,
    I40E_RX_DESC_STATUS_IPV6EXADD_SHIFT	= 15,
    I40E_RX_DESC_STATUS_RESERVED_SHIFT	= 16, /* 2 BITS */
// Note: For non-tunnel packets INT_UDP_0 is the right status for
// UDP header
//
    I40E_RX_DESC_STATUS_INT_UDP_0_SHIFT	= 18,
    I40E_RX_DESC_STATUS_LAST /* this entry must be last!!! */
}

pub const I40E_RXD_QW1_STATUS_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_desc_fltstat_values {
    I40E_RX_DESC_FLTSTAT_NO_DATA	= 0,
    I40E_RX_DESC_FLTSTAT_RSV_FD_ID	= 1, /* 16byte desc? FD_ID : RSV */
    I40E_RX_DESC_FLTSTAT_RSV	= 2,
    I40E_RX_DESC_FLTSTAT_RSS_HASH	= 3,
}

pub const I40E_RXD_QW1_ERROR_SHIFT: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_desc_error_bits {
// Note: These are predefined bit offsets
    I40E_RX_DESC_ERROR_RXE_SHIFT		= 0,
    I40E_RX_DESC_ERROR_RECIPE_SHIFT		= 1,
    I40E_RX_DESC_ERROR_HBO_SHIFT		= 2,
    I40E_RX_DESC_ERROR_L3L4E_SHIFT		= 3, /* 3 BITS */
    I40E_RX_DESC_ERROR_IPE_SHIFT		= 3,
    I40E_RX_DESC_ERROR_L4E_SHIFT		= 4,
    I40E_RX_DESC_ERROR_EIPE_SHIFT		= 5,
    I40E_RX_DESC_ERROR_OVERSIZE_SHIFT	= 6,
    I40E_RX_DESC_ERROR_PPRS_SHIFT		= 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_desc_error_l3l4e_fcoe_masks {
    I40E_RX_DESC_ERROR_L3L4E_NONE		= 0,
    I40E_RX_DESC_ERROR_L3L4E_PROT		= 1,
    I40E_RX_DESC_ERROR_L3L4E_FC		= 2,
    I40E_RX_DESC_ERROR_L3L4E_DMAC_ERR	= 3,
    I40E_RX_DESC_ERROR_L3L4E_DMAC_WARN	= 4
}

pub const I40E_RXD_QW1_PTYPE_SHIFT: c_int = 30;

pub const I40E_RXD_QW1_LENGTH_PBUF_SHIFT: c_int = 38;

pub const I40E_RXD_QW1_LENGTH_SPH_SHIFT: c_int = 63;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_desc_ext_status_bits {
// Note: These are predefined bit offsets
    I40E_RX_DESC_EXT_STATUS_L2TAG2P_SHIFT	= 0,
    I40E_RX_DESC_EXT_STATUS_L2TAG3P_SHIFT	= 1,
    I40E_RX_DESC_EXT_STATUS_FLEXBL_SHIFT	= 2, /* 2 BITS */
    I40E_RX_DESC_EXT_STATUS_FLEXBH_SHIFT	= 4, /* 2 BITS */
    I40E_RX_DESC_EXT_STATUS_FDLONGB_SHIFT	= 9,
    I40E_RX_DESC_EXT_STATUS_FCOELONGB_SHIFT	= 10,
    I40E_RX_DESC_EXT_STATUS_PELONGB_SHIFT	= 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_desc_pe_status_bits {
// Note: These are predefined bit offsets
    I40E_RX_DESC_PE_STATUS_QPID_SHIFT	= 0, /* 18 BITS */
    I40E_RX_DESC_PE_STATUS_L4PORT_SHIFT	= 0, /* 16 BITS */
    I40E_RX_DESC_PE_STATUS_IPINDEX_SHIFT	= 16, /* 8 BITS */
    I40E_RX_DESC_PE_STATUS_QPIDHIT_SHIFT	= 24,
    I40E_RX_DESC_PE_STATUS_APBVTHIT_SHIFT	= 25,
    I40E_RX_DESC_PE_STATUS_PORTV_SHIFT	= 26,
    I40E_RX_DESC_PE_STATUS_URG_SHIFT	= 27,
    I40E_RX_DESC_PE_STATUS_IPFRAG_SHIFT	= 28,
    I40E_RX_DESC_PE_STATUS_IPOPT_SHIFT	= 29
}

pub const I40E_RX_PROG_STATUS_DESC_LENGTH: c_uint = 0x2000000;
pub const I40E_RX_PROG_STATUS_DESC_QW1_PROGID_SHIFT: c_int = 2;

pub const I40E_RX_PROG_STATUS_DESC_QW1_ERROR_SHIFT: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_prog_status_desc_status_bits {
// Note: These are predefined bit offsets
    I40E_RX_PROG_STATUS_DESC_DD_SHIFT	= 0,
    I40E_RX_PROG_STATUS_DESC_PROG_ID_SHIFT	= 2 /* 3 BITS */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_prog_status_desc_prog_id_masks {
    I40E_RX_PROG_STATUS_DESC_FD_FILTER_STATUS	= 1,
    I40E_RX_PROG_STATUS_DESC_FCOE_CTXT_PROG_STATUS	= 2,
    I40E_RX_PROG_STATUS_DESC_FCOE_CTXT_INVL_STATUS	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_rx_prog_status_desc_error_bits {
// Note: These are predefined bit offsets
    I40E_RX_PROG_STATUS_DESC_FD_TBL_FULL_SHIFT	= 0,
    I40E_RX_PROG_STATUS_DESC_NO_FD_ENTRY_SHIFT	= 1,
    I40E_RX_PROG_STATUS_DESC_FCOE_TBL_FULL_SHIFT	= 2,
    I40E_RX_PROG_STATUS_DESC_FCOE_CONFLICT_SHIFT	= 3
}

// TX Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_offset_bsz: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_tx_desc_dtype_value {
    I40E_TX_DESC_DTYPE_DATA		= 0x0,
    I40E_TX_DESC_DTYPE_NOP		= 0x1, /* same as Context desc */
    I40E_TX_DESC_DTYPE_CONTEXT	= 0x1,
    I40E_TX_DESC_DTYPE_FCOE_CTX	= 0x2,
    I40E_TX_DESC_DTYPE_FILTER_PROG	= 0x8,
    I40E_TX_DESC_DTYPE_DDP_CTX	= 0x9,
    I40E_TX_DESC_DTYPE_FLEX_DATA	= 0xB,
    I40E_TX_DESC_DTYPE_FLEX_CTX_1	= 0xC,
    I40E_TX_DESC_DTYPE_FLEX_CTX_2	= 0xD,
    I40E_TX_DESC_DTYPE_DESC_DONE	= 0xF
}

pub const I40E_TXD_QW1_CMD_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_tx_desc_cmd_bits {
    I40E_TX_DESC_CMD_EOP			= 0x0001,
    I40E_TX_DESC_CMD_RS			= 0x0002,
    I40E_TX_DESC_CMD_ICRC			= 0x0004,
    I40E_TX_DESC_CMD_IL2TAG1		= 0x0008,
    I40E_TX_DESC_CMD_DUMMY			= 0x0010,
    I40E_TX_DESC_CMD_IIPT_NONIP		= 0x0000, /* 2 BITS */
    I40E_TX_DESC_CMD_IIPT_IPV6		= 0x0020, /* 2 BITS */
    I40E_TX_DESC_CMD_IIPT_IPV4		= 0x0040, /* 2 BITS */
    I40E_TX_DESC_CMD_IIPT_IPV4_CSUM		= 0x0060, /* 2 BITS */
    I40E_TX_DESC_CMD_FCOET			= 0x0080,
    I40E_TX_DESC_CMD_L4T_EOFT_UNK		= 0x0000, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_TCP		= 0x0100, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_SCTP		= 0x0200, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_UDP		= 0x0300, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_EOF_N		= 0x0000, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_EOF_T		= 0x0100, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_EOF_NI	= 0x0200, /* 2 BITS */
    I40E_TX_DESC_CMD_L4T_EOFT_EOF_A		= 0x0300, /* 2 BITS */
}

pub const I40E_TXD_QW1_OFFSET_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_tx_desc_length_fields {
// Note: These are predefined bit offsets
    I40E_TX_DESC_LENGTH_MACLEN_SHIFT	= 0, /* 7 BITS */
    I40E_TX_DESC_LENGTH_IPLEN_SHIFT		= 7, /* 7 BITS */
    I40E_TX_DESC_LENGTH_L4_FC_LEN_SHIFT	= 14 /* 4 BITS */
}

pub const I40E_TXD_QW1_TX_BUF_SZ_SHIFT: c_int = 34;
pub const I40E_TXD_QW1_L2TAG1_SHIFT: c_int = 48;
// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_tx_context_desc {
    pub tunneling_params: __le32,
    pub l2tag2: __le16,
    pub rsvd: __le16,
    pub type_cmd_tso_mss: __le64,
}

pub const I40E_TXD_CTX_QW1_CMD_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_tx_ctx_desc_cmd_bits {
    I40E_TX_CTX_DESC_TSO		= 0x01,
    I40E_TX_CTX_DESC_TSYN		= 0x02,
    I40E_TX_CTX_DESC_IL2TAG2	= 0x04,
    I40E_TX_CTX_DESC_IL2TAG2_IL2H	= 0x08,
    I40E_TX_CTX_DESC_SWTCH_NOTAG	= 0x00,
    I40E_TX_CTX_DESC_SWTCH_UPLINK	= 0x10,
    I40E_TX_CTX_DESC_SWTCH_LOCAL	= 0x20,
    I40E_TX_CTX_DESC_SWTCH_VSI	= 0x30,
    I40E_TX_CTX_DESC_SWPE		= 0x40
}

pub const I40E_TXD_CTX_QW1_TSO_LEN_SHIFT: c_int = 30;
pub const I40E_TXD_CTX_QW1_MSS_SHIFT: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_tx_ctx_desc_eipt_offload {
    I40E_TX_CTX_EXT_IP_NONE		= 0x0,
    I40E_TX_CTX_EXT_IP_IPV6		= 0x1,
    I40E_TX_CTX_EXT_IP_IPV4_NO_CSUM	= 0x2,
    I40E_TX_CTX_EXT_IP_IPV4		= 0x3
}

pub const I40E_TXD_CTX_QW0_EXT_IPLEN_SHIFT: c_int = 2;
pub const I40E_TXD_CTX_QW0_NATT_SHIFT: c_int = 9;

pub const I40E_TXD_CTX_QW0_NATLEN_SHIFT: c_int = 12;
pub const I40E_TXD_CTX_QW0_L4T_CS_SHIFT: c_int = 23;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_filter_program_desc {
    pub qindex_flex_ptype_vsi: __le32,
    pub rsvd: __le32,
    pub dtype_cmd_cntindex: __le32,
    pub fd_id: __le32,
}

pub const I40E_TXD_FLTR_QW0_QINDEX_SHIFT: c_int = 0;

pub const I40E_TXD_FLTR_QW0_FLEXOFF_SHIFT: c_int = 11;

pub const I40E_TXD_FLTR_QW0_PCTYPE_SHIFT: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_filter_program_desc_dest {
    I40E_FILTER_PROGRAM_DESC_DEST_DROP_PACKET		= 0x0,
    I40E_FILTER_PROGRAM_DESC_DEST_DIRECT_PACKET_QINDEX	= 0x1,
    I40E_FILTER_PROGRAM_DESC_DEST_DIRECT_PACKET_OTHER	= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_filter_program_desc_fd_status {
    I40E_FILTER_PROGRAM_DESC_FD_STATUS_NONE			= 0x0,
    I40E_FILTER_PROGRAM_DESC_FD_STATUS_FD_ID		= 0x1,
    I40E_FILTER_PROGRAM_DESC_FD_STATUS_FD_ID_4FLEX_BYTES	= 0x2,
    I40E_FILTER_PROGRAM_DESC_FD_STATUS_8FLEX_BYTES		= 0x3,
}

pub const I40E_TXD_FLTR_QW0_DEST_VSI_SHIFT: c_int = 23;

pub const I40E_TXD_FLTR_QW1_CMD_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_filter_program_desc_pcmd {
    I40E_FILTER_PROGRAM_DESC_PCMD_ADD_UPDATE	= 0x1,
    I40E_FILTER_PROGRAM_DESC_PCMD_REMOVE		= 0x2,
}

pub const I40E_TXD_FLTR_QW1_CNTINDEX_SHIFT: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_filter_type {
    I40E_FLOW_DIRECTOR_FLTR = 0,
    I40E_PE_QUAD_HASH_FLTR = 1,
    I40E_ETHERTYPE_FLTR,
    I40E_FCOE_CTX_FLTR,
    I40E_MAC_VLAN_FLTR,
    I40E_HASH_FLTR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_vsi_context {
    pub seid: u16,
    pub uplink_seid: u16,
    pub vsi_number: u16,
    pub vsis_allocated: u16,
    pub vsis_unallocated: u16,
    pub flags: u16,
    pub pf_num: u8,
    pub vf_num: u8,
    pub connection_type: u8,
    pub info: i40e_aqc_vsi_properties_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_veb_context {
    pub seid: u16,
    pub uplink_seid: u16,
    pub veb_number: u16,
    pub vebs_allocated: u16,
    pub vebs_unallocated: u16,
    pub flags: u16,
    pub info: i40e_aqc_get_veb_parameters_completion,
}

// Statistics collected by each port, VSI, VEB, and S-channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_eth_stats {
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
    pub /: *mut *mut u64 rx_discards_other; / rxerr1,
}

// Statistics collected per VEB per TC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_veb_tc_stats {
    pub tc_rx_packets: [u64; I40E_MAX_TRAFFIC_CLASS],
    pub tc_rx_bytes: [u64; I40E_MAX_TRAFFIC_CLASS],
    pub tc_tx_packets: [u64; I40E_MAX_TRAFFIC_CLASS],
    pub tc_tx_bytes: [u64; I40E_MAX_TRAFFIC_CLASS],
}

// Statistics collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hw_port_stats {
// eth stats collected by the port
    pub eth: i40e_eth_stats,
// additional port specific stats
    pub /: *mut *mut u64 tx_dropped_link_down; / tdold,
    pub /: *mut *mut u64 crc_errors; / crcerrs,
    pub /: *mut *mut u64 illegal_bytes; / illerrc,
    pub /: *mut *mut u64 error_bytes; / errbc,
    pub /: *mut *mut u64 mac_local_faults; / mlfc,
    pub /: *mut *mut u64 mac_remote_faults; / mrfc,
    pub /: *mut *mut u64 rx_length_errors; / rlec,
    pub /: *mut *mut u64 link_xon_rx; / lxonrxc,
    pub /: *mut *mut u64 link_xoff_rx; / lxoffrxc,
    pub /: *mut *mut u64 priority_xon_rx[8]; / pxonrxc[8],
    pub /: *mut *mut u64 priority_xoff_rx[8]; / pxoffrxc[8],
    pub /: *mut *mut u64 link_xon_tx; / lxontxc,
    pub /: *mut *mut u64 link_xoff_tx; / lxofftxc,
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
    pub /: *mut *mut u64 mac_short_packet_dropped; / mspdc,
    pub /: *mut *mut u64 checksum_error; / xec,
// flow director stats
    pub fd_atr_match: u64,
    pub fd_sb_match: u64,
    pub fd_atr_tunnel_match: u64,
    pub fd_atr_status: u32,
    pub fd_sb_status: u32,
// EEE LPI
    pub tx_lpi_status: u32,
    pub rx_lpi_status: u32,
    pub /: *mut *mut u64 tx_lpi_count; / etlpic,
    pub /: *mut *mut u64 rx_lpi_count; / erlpic,
}

// Checksum and Shadow RAM pointers
pub const I40E_SR_NVM_CONTROL_WORD: c_uint = 0x00;
pub const I40E_EMP_MODULE_PTR: c_uint = 0x0F;
pub const I40E_SR_EMP_MODULE_PTR: c_uint = 0x48;
pub const I40E_SR_PBA_FLAGS: c_uint = 0x15;
pub const I40E_SR_PBA_BLOCK_PTR: c_uint = 0x16;
pub const I40E_SR_BOOT_CONFIG_PTR: c_uint = 0x17;
pub const I40E_NVM_OEM_VER_OFF: c_uint = 0x83;
pub const I40E_SR_NVM_DEV_STARTER_VERSION: c_uint = 0x18;
pub const I40E_SR_NVM_WAKE_ON_LAN: c_uint = 0x19;
pub const I40E_SR_NVM_EETRACK_LO: c_uint = 0x2D;
pub const I40E_SR_NVM_EETRACK_HI: c_uint = 0x2E;
pub const I40E_SR_VPD_PTR: c_uint = 0x2F;
pub const I40E_SR_PCIE_ALT_AUTO_LOAD_PTR: c_uint = 0x3E;
pub const I40E_SR_SW_CHECKSUM_WORD: c_uint = 0x3F;
pub const I40E_SR_EMP_SR_SETTINGS_PTR: c_uint = 0x48;
// Auxiliary field, mask and shift definition for Shadow RAM and NVM Flash
pub const I40E_SR_VPD_MODULE_MAX_SIZE: c_int = 1024;
pub const I40E_SR_PCIE_ALT_MODULE_MAX_SIZE: c_int = 1024;
pub const I40E_SR_CONTROL_WORD_1_SHIFT: c_uint = 0x06;

pub const I40E_SR_OCP_CFG_WORD0: c_uint = 0x2B;

// Shadow RAM related
pub const I40E_SR_SECTOR_SIZE_IN_WORDS: c_uint = 0x800;
pub const I40E_SR_WORDS_IN_1KB: c_int = 512;
// Checksum should be calculated such that after adding all the words,
// including the checksum word itself, the sum should be 0xBABA.
//
pub const I40E_SR_SW_CHECKSUM_BASE: c_uint = 0xBABA;
pub const I40E_SRRD_SRCTL_ATTEMPTS: c_int = 100000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_switch_element_types {
    I40E_SWITCH_ELEMENT_TYPE_MAC	= 1,
    I40E_SWITCH_ELEMENT_TYPE_PF	= 2,
    I40E_SWITCH_ELEMENT_TYPE_VF	= 3,
    I40E_SWITCH_ELEMENT_TYPE_EMP	= 4,
    I40E_SWITCH_ELEMENT_TYPE_BMC	= 6,
    I40E_SWITCH_ELEMENT_TYPE_PE	= 16,
    I40E_SWITCH_ELEMENT_TYPE_VEB	= 17,
    I40E_SWITCH_ELEMENT_TYPE_PA	= 18,
    I40E_SWITCH_ELEMENT_TYPE_VSI	= 19,
}

// Supported EtherType filters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_ether_type_index {
    I40E_ETHER_TYPE_1588		= 0,
    I40E_ETHER_TYPE_FIP		= 1,
    I40E_ETHER_TYPE_OUI_EXTENDED	= 2,
    I40E_ETHER_TYPE_MAC_CONTROL	= 3,
    I40E_ETHER_TYPE_LLDP		= 4,
    I40E_ETHER_TYPE_EVB_PROTOCOL1	= 5,
    I40E_ETHER_TYPE_EVB_PROTOCOL2	= 6,
    I40E_ETHER_TYPE_QCN_CNM		= 7,
    I40E_ETHER_TYPE_8021X		= 8,
    I40E_ETHER_TYPE_ARP		= 9,
    I40E_ETHER_TYPE_RSV1		= 10,
    I40E_ETHER_TYPE_RSV2		= 11,
}

// Filter context base size is 1K
pub const I40E_HASH_FILTER_BASE_SIZE: c_int = 1024;
// Supported Hash filter values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hash_filter_size {
    I40E_HASH_FILTER_SIZE_1K	= 0,
    I40E_HASH_FILTER_SIZE_2K	= 1,
    I40E_HASH_FILTER_SIZE_4K	= 2,
    I40E_HASH_FILTER_SIZE_8K	= 3,
    I40E_HASH_FILTER_SIZE_16K	= 4,
    I40E_HASH_FILTER_SIZE_32K	= 5,
    I40E_HASH_FILTER_SIZE_64K	= 6,
    I40E_HASH_FILTER_SIZE_128K	= 7,
    I40E_HASH_FILTER_SIZE_256K	= 8,
    I40E_HASH_FILTER_SIZE_512K	= 9,
    I40E_HASH_FILTER_SIZE_1M	= 10,
}

// DMA context base size is 0.5K
pub const I40E_DMA_CNTX_BASE_SIZE: c_int = 512;
// Supported DMA context values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_dma_cntx_size {
    I40E_DMA_CNTX_SIZE_512		= 0,
    I40E_DMA_CNTX_SIZE_1K		= 1,
    I40E_DMA_CNTX_SIZE_2K		= 2,
    I40E_DMA_CNTX_SIZE_4K		= 3,
    I40E_DMA_CNTX_SIZE_8K		= 4,
    I40E_DMA_CNTX_SIZE_16K		= 5,
    I40E_DMA_CNTX_SIZE_32K		= 6,
    I40E_DMA_CNTX_SIZE_64K		= 7,
    I40E_DMA_CNTX_SIZE_128K		= 8,
    I40E_DMA_CNTX_SIZE_256K		= 9,
}

// Supported Hash look up table (LUT) sizes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hash_lut_size {
    I40E_HASH_LUT_SIZE_128		= 0,
    I40E_HASH_LUT_SIZE_512		= 1,
}

// Structure to hold a per PF filter control settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_filter_control_settings {
// number of PE Quad Hash filter buckets
    pub pe_filt_num: i40e_hash_filter_size,
// number of PE Quad Hash contexts
    pub pe_cntx_num: i40e_dma_cntx_size,
// number of FCoE filter buckets
    pub fcoe_filt_num: i40e_hash_filter_size,
// number of FCoE DDP contexts
    pub fcoe_cntx_num: i40e_dma_cntx_size,
// size of the Hash LUT
    pub hash_lut_size: i40e_hash_lut_size,
// enable FDIR filters for PF and its VFs
    pub enable_fdir: bool,
// enable Ethertype filters for PF and its VFs
    pub enable_ethtype: bool,
// enable MAC/VLAN filters for PF and its VFs
    pub enable_macvlan: bool,
}

// Structure to hold device level control filter counts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_control_filter_stats {
    pub /: *mut *mut u16 mac_etype_used; / Used perfect match MAC/EtherType filters,
    pub /: *mut *mut u16 etype_used; / Used perfect EtherType filters,
    pub /: *mut *mut u16 mac_etype_free; / Un-used perfect match MAC/EtherType filters,
    pub /: *mut *mut u16 etype_free; / Un-used perfect EtherType filters,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_reset_type {
    I40E_RESET_POR		= 0,
    I40E_RESET_CORER	= 1,
    I40E_RESET_GLOBR	= 2,
    I40E_RESET_EMPR		= 3,
}

// IEEE 802.1AB LLDP Agent Variables from NVM
pub const I40E_NVM_LLDP_CFG_PTR: c_uint = 0x06;
pub const I40E_SR_LLDP_CFG_PTR: c_uint = 0x31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_lldp_variables {
    pub length: u16,
    pub adminstatus: u16,
    pub msgfasttx: u16,
    pub msgtxinterval: u16,
    pub txparams: u16,
    pub timers: u16,
    pub crc8: u16,
}

// Offsets into Alternate Ram

pub const I40E_ALT_STRUCT_MIN_BW_OFFSET: c_uint = 0xE  /* in dwords */;
pub const I40E_ALT_STRUCT_MAX_BW_OFFSET: c_uint = 0xF  /* in dwords */;
// Alternate Ram Bandwidth Masks
pub const I40E_ALT_BW_VALUE_MASK: c_uint = 0xFF;
pub const I40E_ALT_BW_VALID_MASK: c_uint = 0x80000000;
// RSS Hash Table Size
pub const I40E_PFQF_CTL_0_HASHLUTSIZE_512: c_uint = 0x00010000;
// INPUT SET MASK for RSS, flow director, and flexible payload
pub const I40E_X722_L3_SRC_SHIFT: c_int = 49;

pub const I40E_X722_L3_DST_SHIFT: c_int = 41;

pub const I40E_L3_SRC_SHIFT: c_int = 47;

pub const I40E_L3_V6_SRC_SHIFT: c_int = 43;

pub const I40E_L3_DST_SHIFT: c_int = 35;

pub const I40E_L3_V6_DST_SHIFT: c_int = 35;

pub const I40E_L4_SRC_SHIFT: c_int = 34;

pub const I40E_L4_DST_SHIFT: c_int = 33;

pub const I40E_VERIFY_TAG_SHIFT: c_int = 31;

pub const I40E_VLAN_SRC_SHIFT: c_int = 55;

pub const I40E_FLEX_50_SHIFT: c_int = 13;

pub const I40E_FLEX_51_SHIFT: c_int = 12;

pub const I40E_FLEX_52_SHIFT: c_int = 11;

pub const I40E_FLEX_53_SHIFT: c_int = 10;

pub const I40E_FLEX_54_SHIFT: c_int = 9;

pub const I40E_FLEX_55_SHIFT: c_int = 8;

pub const I40E_FLEX_56_SHIFT: c_int = 7;

pub const I40E_FLEX_57_SHIFT: c_int = 6;

// Version format for Dynamic Device Personalization(DDP)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_ddp_version {
    pub major: u8,
    pub minor: u8,
    pub update: u8,
    pub draft: u8,
}

pub const I40E_DDP_NAME_SIZE: c_int = 32;
// Package header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_package_header {
    pub version: i40e_ddp_version,
    pub segment_count: u32,
    pub segment_offset: [u32; ],
}

// Generic segment header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_generic_seg_header {
pub const SEGMENT_TYPE_METADATA: c_uint = 0x00000001;
pub const SEGMENT_TYPE_I40E: c_uint = 0x00000011;
    pub type: u32,
    pub version: i40e_ddp_version,
    pub size: u32,
    pub name: [c_char; I40E_DDP_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_metadata_segment {
    pub header: i40e_generic_seg_header,
    pub version: i40e_ddp_version,
pub const I40E_DDP_TRACKID_INVALID: c_uint = 0xFFFFFFFF;
    pub track_id: u32,
    pub name: [c_char; I40E_DDP_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_device_id_entry {
    pub vendor_dev_id: u32,
    pub sub_vendor_dev_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_profile_segment {
    pub header: i40e_generic_seg_header,
    pub version: i40e_ddp_version,
    pub name: [c_char; I40E_DDP_NAME_SIZE],
    pub device_table_count: u32,
    pub device_table: [i40e_device_id_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_section_table {
    pub section_count: u32,
    pub section_offset: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_profile_section_header {
    pub tbl_size: u16,
    pub data_end: u16,
pub const SECTION_TYPE_INFO: c_uint = 0x00000010;
pub const SECTION_TYPE_MMIO: c_uint = 0x00000800;
pub const SECTION_TYPE_RB_MMIO: c_uint = 0x00001800;
pub const SECTION_TYPE_AQ: c_uint = 0x00000801;
pub const SECTION_TYPE_RB_AQ: c_uint = 0x00001801;
pub const SECTION_TYPE_NOTE: c_uint = 0x80000000;
    pub type: u32,
    pub offset: u32,
    pub size: u32,
    pub section: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_profile_tlv_section_record {
    pub rtype: u8,
    pub type: u8,
    pub len: u16,
    pub data: [u8; 12],
}

// Generic AQ section in proflie
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_profile_aq_section {
    pub opcode: u16,
    pub flags: u16,
    pub param: [u8; 16],
    pub datalen: u16,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_profile_info {
    pub track_id: u32,
    pub version: i40e_ddp_version,
    pub op: u8,
pub const I40E_DDP_ADD_TRACKID: c_uint = 0x01;
pub const I40E_DDP_REMOVE_TRACKID: c_uint = 0x02;
    pub reserved: [u8; 7],
    pub name: [u8; I40E_DDP_NAME_SIZE],
}
