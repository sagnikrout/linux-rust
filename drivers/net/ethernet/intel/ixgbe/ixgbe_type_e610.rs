//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_type_e610.h
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
// Copyright(c) 2024 Intel Corporation.

pub const BYTES_PER_DWORD: c_int = 4;
// General E610 defines
pub const IXGBE_MAX_VSI: c_int = 768;
// Checksum and Shadow RAM pointers
pub const IXGBE_E610_SR_NVM_CTRL_WORD: c_uint = 0x00;
pub const IXGBE_E610_SR_PBA_BLOCK_PTR: c_uint = 0x16;

pub const IXGBE_E610_SR_NVM_DEV_STARTER_VER: c_uint = 0x18;
pub const IXGBE_E610_SR_NVM_EETRACK_LO: c_uint = 0x2D;
pub const IXGBE_E610_SR_NVM_EETRACK_HI: c_uint = 0x2E;

pub const IXGBE_E610_SR_SW_CHECKSUM_WORD: c_uint = 0x3F;
pub const IXGBE_E610_SR_PFA_PTR: c_uint = 0x40;
pub const IXGBE_E610_SR_1ST_NVM_BANK_PTR: c_uint = 0x42;
pub const IXGBE_E610_SR_NVM_BANK_SIZE: c_uint = 0x43;
pub const IXGBE_E610_SR_1ST_OROM_BANK_PTR: c_uint = 0x44;
pub const IXGBE_E610_SR_OROM_BANK_SIZE: c_uint = 0x45;
pub const IXGBE_E610_SR_NETLIST_BANK_PTR: c_uint = 0x46;
pub const IXGBE_E610_SR_NETLIST_BANK_SIZE: c_uint = 0x47;
// The OROM version topology

// CSS Header words
pub const IXGBE_NVM_CSS_HDR_LEN_L: c_uint = 0x02;
pub const IXGBE_NVM_CSS_HDR_LEN_H: c_uint = 0x03;
pub const IXGBE_NVM_CSS_SREV_L: c_uint = 0x14;
pub const IXGBE_NVM_CSS_SREV_H: c_uint = 0x15;
pub const IXGBE_HDR_LEN_ROUNDUP: c_int = 32;
// Length of Authentication header section in words
pub const IXGBE_NVM_AUTH_HEADER_LEN: c_uint = 0x08;
// Shadow RAM related
pub const IXGBE_SR_WORDS_IN_1KB: c_int = 512;
// The Netlist ID Block is located after all of the Link Topology nodes.
pub const IXGBE_NETLIST_ID_BLK_SIZE: c_uint = 0x30;

// netlist ID block field offsets (word offsets)
pub const IXGBE_NETLIST_ID_BLK_MAJOR_VER_LOW: c_uint = 0x02;
pub const IXGBE_NETLIST_ID_BLK_MAJOR_VER_HIGH: c_uint = 0x03;
pub const IXGBE_NETLIST_ID_BLK_MINOR_VER_LOW: c_uint = 0x04;
pub const IXGBE_NETLIST_ID_BLK_MINOR_VER_HIGH: c_uint = 0x05;
pub const IXGBE_NETLIST_ID_BLK_TYPE_LOW: c_uint = 0x06;
pub const IXGBE_NETLIST_ID_BLK_TYPE_HIGH: c_uint = 0x07;
pub const IXGBE_NETLIST_ID_BLK_REV_LOW: c_uint = 0x08;
pub const IXGBE_NETLIST_ID_BLK_REV_HIGH: c_uint = 0x09;

pub const IXGBE_NETLIST_ID_BLK_CUST_VER: c_uint = 0x2F;
// The Link Topology Netlist section is stored as a series of words. It is
// stored in the NVM as a TLV, with the first two words containing the type
// and length.
//
pub const IXGBE_NETLIST_LINK_TOPO_MOD_ID: c_uint = 0x011B;
pub const IXGBE_NETLIST_TYPE_OFFSET: c_uint = 0x0000;
pub const IXGBE_NETLIST_LEN_OFFSET: c_uint = 0x0001;
// The Link Topology section follows the TLV header. When reading the netlist
// using ixgbe_read_netlist_module, we need to account for the 2-word TLV
// header.
//

// Firmware Status Register (GL_FWSTS)
pub const GL_FWSTS: c_uint = 0x00083048 /* Reset Source: POR */;

// Global NVM General Status Register
pub const GLNVM_GENS: c_uint = 0x000B6100 /* Reset Source: POR */;

pub const IXGBE_GL_MNG_FWSM: c_uint = 0x000B6134 /* Reset Source: POR */;

// Flash Access Register
pub const IXGBE_GLNVM_FLA: c_uint = 0x000B6108 /* Reset Source: POR */;
pub const IXGBE_GLNVM_FLA_LOCKED_S: c_int = 6;

// Auxiliary field, mask and shift definition for Shadow RAM and NVM Flash

// Admin Command Interface (ACI) registers

pub const IXGBE_PF_HICR: c_uint = 0x00082048;

pub const IXGBE_FW_API_VER_MAJOR: c_uint = 0x01;
pub const IXGBE_FW_API_VER_MINOR: c_uint = 0x07;
pub const IXGBE_FW_API_VER_DIFF_ALLOWED: c_uint = 0x02;
pub const IXGBE_ACI_DESC_SIZE: c_int = 32;

pub const IXGBE_ACI_SEND_DELAY_TIME_MS: c_int = 10;
pub const IXGBE_ACI_SEND_MAX_EXECUTE: c_int = 3;

// [ms] timeout of waiting for sync response
pub const IXGBE_ACI_SYNC_RESPONSE_TIMEOUT: c_int = 100000;
// [ms] timeout of waiting for async response
pub const IXGBE_ACI_ASYNC_RESPONSE_TIMEOUT: c_int = 150000;
// [ms] timeout of waiting for resource release
pub const IXGBE_ACI_RELEASE_RES_TIMEOUT: c_int = 10000;
// Admin Command Interface (ACI) opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_aci_opc {
    ixgbe_aci_opc_get_ver				= 0x0001,
    ixgbe_aci_opc_driver_ver			= 0x0002,
    ixgbe_aci_opc_get_exp_err			= 0x0005,

// resource ownership
    ixgbe_aci_opc_req_res				= 0x0008,
    ixgbe_aci_opc_release_res			= 0x0009,

// device/function capabilities
    ixgbe_aci_opc_list_func_caps			= 0x000A,
    ixgbe_aci_opc_list_dev_caps			= 0x000B,

// safe disable of RXEN
    ixgbe_aci_opc_disable_rxen			= 0x000C,

// FW events
    ixgbe_aci_opc_get_fw_event			= 0x0014,

// PHY commands
    ixgbe_aci_opc_get_phy_caps			= 0x0600,
    ixgbe_aci_opc_set_phy_cfg			= 0x0601,
    ixgbe_aci_opc_restart_an			= 0x0605,
    ixgbe_aci_opc_get_link_status			= 0x0607,
    ixgbe_aci_opc_set_event_mask			= 0x0613,
    ixgbe_aci_opc_get_link_topo			= 0x06E0,
    ixgbe_aci_opc_get_link_topo_pin			= 0x06E1,
    ixgbe_aci_opc_read_i2c				= 0x06E2,
    ixgbe_aci_opc_write_i2c				= 0x06E3,
    ixgbe_aci_opc_read_mdio				= 0x06E4,
    ixgbe_aci_opc_write_mdio			= 0x06E5,
    ixgbe_aci_opc_set_gpio_by_func			= 0x06E6,
    ixgbe_aci_opc_get_gpio_by_func			= 0x06E7,
    ixgbe_aci_opc_set_port_id_led			= 0x06E9,
    ixgbe_aci_opc_set_gpio				= 0x06EC,
    ixgbe_aci_opc_get_gpio				= 0x06ED,
    ixgbe_aci_opc_sff_eeprom			= 0x06EE,
    ixgbe_aci_opc_prog_topo_dev_nvm			= 0x06F2,
    ixgbe_aci_opc_read_topo_dev_nvm			= 0x06F3,

// NVM commands
    ixgbe_aci_opc_nvm_read				= 0x0701,
    ixgbe_aci_opc_nvm_erase				= 0x0702,
    ixgbe_aci_opc_nvm_write				= 0x0703,
    ixgbe_aci_opc_nvm_cfg_read			= 0x0704,
    ixgbe_aci_opc_nvm_cfg_write			= 0x0705,
    ixgbe_aci_opc_nvm_checksum			= 0x0706,
    ixgbe_aci_opc_nvm_write_activate		= 0x0707,
    ixgbe_aci_opc_nvm_sr_dump			= 0x0707,
    ixgbe_aci_opc_nvm_save_factory_settings		= 0x0708,
    ixgbe_aci_opc_nvm_update_empr			= 0x0709,
    ixgbe_aci_opc_nvm_pkg_data			= 0x070A,
    ixgbe_aci_opc_nvm_pass_component_tbl		= 0x070B,

// Alternate Structure Commands
    ixgbe_aci_opc_write_alt_direct			= 0x0900,
    ixgbe_aci_opc_write_alt_indirect		= 0x0901,
    ixgbe_aci_opc_read_alt_direct			= 0x0902,
    ixgbe_aci_opc_read_alt_indirect			= 0x0903,
    ixgbe_aci_opc_done_alt_write			= 0x0904,
    ixgbe_aci_opc_clear_port_alt_write		= 0x0906,

// TCA Events
    ixgbe_aci_opc_temp_tca_event                    = 0x0C94,

// debug commands
    ixgbe_aci_opc_debug_dump_internals		= 0xFF08,

// SystemDiagnostic commands
    ixgbe_aci_opc_set_health_status_config		= 0xFF20,
    ixgbe_aci_opc_get_supported_health_status_codes	= 0xFF21,
    ixgbe_aci_opc_get_health_status			= 0xFF22,
    ixgbe_aci_opc_clear_health_status		= 0xFF23,
}

pub const IXGBE_DRV_VER_STR_LEN_E610: c_int = 32;
// Get Expanded Error Code (0x0005, direct)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_exp_err {
    pub reason: __le32,
pub const IXGBE_ACI_EXPANDED_ERROR_NOT_PROVIDED: c_uint = 0xFFFFFFFF;
    pub identifier: __le32,
    pub rsvd: [u8; 8],
}

// FW update timeout definitions are in milliseconds
pub const IXGBE_NVM_TIMEOUT: c_int = 180000;
// Disable RXEN (direct 0x000C)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_disable_rxen {
    pub lport_num: u8,
    pub reserved: [u8; 15],
}

// Get PHY capabilities (indirect 0x0600)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_phy_caps {
    pub lport_num: u8,
    pub reserved: u8,
    pub param0: __le16,
// 18.0 - Report qualified modules

// 18.1 - 18.3 : Report mode
// 000b - Report topology capabilities, without media
// 001b - Report topology capabilities, with media
// 010b - Report Active configuration
// 011b - Report PHY Type and FEC mode capabilities
// 100b - Report Default capabilities
//

pub const IXGBE_ACI_REPORT_TOPO_CAP_NO_MEDIA: c_int = 0;

    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// This is #define of PHY type (Extended):
// The first set of defines is for phy_type_low.
//

pub const IXGBE_PHY_TYPE_LOW_MAX_INDEX: c_int = 29;
// The second set of defines is for phy_type_high.

pub const IXGBE_PHY_TYPE_HIGH_MAX_INDEX: c_int = 61;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_phy_caps_data {
    pub /: *mut *mut *mut __le64 phy_type_low; / Use values from IXGBE_PHY_TYPE_LOW_,
    pub /: *mut *mut *mut __le64 phy_type_high; / Use values from IXGBE_PHY_TYPE_HIGH_,
    pub caps: u8,

    pub low_power_ctrl_an: u8,

    pub eee_cap: __le16,

    pub eeer_value: __le16,
    pub /: *mut *mut u8 phy_id_oui[4]; / PHY/Module ID connected on the port,
    pub phy_fw_ver: [u8; 8],
    pub link_fec_options: u8,

pub const IXGBE_ACI_PHY_FEC_MASK: c_uint = 0xdf;
    pub module_compliance_enforcement: u8,

    pub extended_compliance_code: u8,
pub const IXGBE_ACI_MODULE_TYPE_TOTAL_BYTE: c_int = 3;
    pub module_type: [u8; IXGBE_ACI_MODULE_TYPE_TOTAL_BYTE],
pub const IXGBE_ACI_MOD_TYPE_BYTE0_SFP_PLUS: c_uint = 0xA0;
pub const IXGBE_ACI_MOD_TYPE_BYTE0_QSFP_PLUS: c_uint = 0x80;
pub const IXGBE_ACI_MOD_TYPE_IDENT: c_int = 1;

pub const IXGBE_ACI_MOD_TYPE_BYTE2_SFP_PLUS: c_uint = 0xA0;
pub const IXGBE_ACI_MOD_TYPE_BYTE2_QSFP_PLUS: c_uint = 0x86;
    pub qualified_module_count: u8,
    pub rsvd2: u8,
    pub eee_entry_delay: __le16,
    pub rsvd3: [u8; 4],
pub const IXGBE_ACI_QUAL_MOD_COUNT_MAX: c_int = 16;
    pub v_oui: [u8; 3],
    pub rsvd3: u8,
    pub v_part: [u8; 16],
    pub v_rev: __le32,
    pub rsvd4: __le64,
    pub qual_modules: [}; IXGBE_ACI_QUAL_MOD_COUNT_MAX],
}

// Set PHY capabilities (direct 0x0601)
// NOTE: This command must be followed by setup link and restart auto-neg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_set_phy_cfg {
    pub lport_num: u8,
    pub reserved: [u8; 7],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Set PHY config command data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_set_phy_cfg_data {
    pub /: *mut *mut *mut __le64 phy_type_low; / Use values from IXGBE_PHY_TYPE_LOW_,
    pub /: *mut *mut *mut __le64 phy_type_high; / Use values from IXGBE_PHY_TYPE_HIGH_,
    pub caps: u8,
    pub low_power_ctrl_an: u8,
    pub /: *mut *mut __le16 eee_cap; / Value from ixgbe_aci_get_phy_caps,
    pub /: *mut *mut __le16 eeer_value; / Use defines from ixgbe_aci_get_phy_caps,
    pub /: *mut *mut u8 link_fec_opt; / Use defines from ixgbe_aci_get_phy_caps,
    pub module_compliance_enforcement: u8,
    pub eee_entry_delay: __le16,
    pub __packed: },
// Set PHY config capabilities (@caps) defines
pub const IXGBE_ACI_PHY_ENA_VALID_MASK: c_uint = 0xef;

// Restart AN command data structure (direct 0x0605)
// Also used for response, with only the lport_num field present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_restart_an {
    pub lport_num: u8,
    pub reserved: u8,
    pub cmd_flags: u8,
    pub reserved2: [u8; 13],
}

// Get link status (indirect 0x0607), also used for Link Status Event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_link_status {
    pub lport_num: u8,
    pub reserved: u8,
    pub cmd_flags: __le16,

pub const IXGBE_ACI_LSE_NOP: c_uint = 0x0;
pub const IXGBE_ACI_LSE_DIS: c_uint = 0x2;
pub const IXGBE_ACI_LSE_ENA: c_uint = 0x3;
// only response uses this flag
pub const IXGBE_ACI_LSE_IS_ENABLED: c_uint = 0x1;
    pub reserved2: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get link status response data structure, also used for Link Status Event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_link_status_data {
    pub topo_media_conflict: u8,

    pub link_cfg_err: u8,

    pub link_info: u8,

    pub an_info: u8,

    pub ext_info: u8,

// Port Tx Suspended
pub const IXGBE_ACI_LINK_TX_ACTIVE: c_int = 0;
pub const IXGBE_ACI_LINK_TX_DRAINED: c_int = 1;
pub const IXGBE_ACI_LINK_TX_FLUSHED: c_int = 3;
    pub lb_status: u8,

    pub max_frame_size: __le16,
    pub cfg: u8,

// Pacing Config

pub const IXGBE_ACI_CFG_PACING_TYPE_AVG: c_int = 0;

// External Device Power Ability
    pub power_desc: u8,

pub const IXGBE_ACI_LINK_PWR_BASET_LOW_HIGH: c_int = 0;
pub const IXGBE_ACI_LINK_PWR_BASET_HIGH: c_int = 1;
pub const IXGBE_ACI_LINK_PWR_QSFP_CLASS_1: c_int = 0;
pub const IXGBE_ACI_LINK_PWR_QSFP_CLASS_2: c_int = 1;
pub const IXGBE_ACI_LINK_PWR_QSFP_CLASS_3: c_int = 2;
pub const IXGBE_ACI_LINK_PWR_QSFP_CLASS_4: c_int = 3;
    pub link_speed: __le16,

    pub reserved3: __le16,
    pub eee_status: u8,

    pub reserved4: u8,
    pub /: *mut *mut *mut __le64 phy_type_low; / Use values from ICE_PHY_TYPE_LOW_,
    pub /: *mut *mut *mut __le64 phy_type_high; / Use values from ICE_PHY_TYPE_HIGH_,
// Get link status version 2 link partner data
    pub /: *mut *mut *mut __le64 lp_phy_type_low; / Use values from ICE_PHY_TYPE_LOW_,
    pub /: *mut *mut *mut __le64 lp_phy_type_high; / Use values from ICE_PHY_TYPE_HIGH_,
    pub lp_fec_adv: u8,

    pub lp_fec_req: u8,

    pub lp_flowcontrol: u8,
    pub reserved5: [u8; 5],
    pub __packed: },
// Set event mask command (direct 0x0613)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_set_event_mask {
    pub lport_num: u8,
    pub reserved: [u8; 7],
    pub event_mask: __le16,
    pub reserved1: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_link_topo_params {
    pub lport_num: u8,
    pub lport_num_valid: u8,

    pub node_type_ctx: u8,

pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_PHY: c_int = 0;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_GPIO_CTRL: c_int = 1;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_MUX_CTRL: c_int = 2;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_LED_CTRL: c_int = 3;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_LED: c_int = 4;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_THERMAL: c_int = 5;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_CAGE: c_int = 6;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_MEZZ: c_int = 7;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_ID_EEPROM: c_int = 8;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_CLK_CTRL: c_int = 9;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_CLK_MUX: c_int = 10;
pub const IXGBE_ACI_LINK_TOPO_NODE_TYPE_GPS: c_int = 11;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_S: c_int = 4;

pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_GLOBAL: c_int = 0;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_BOARD: c_int = 1;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_PORT: c_int = 2;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_NODE: c_int = 3;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_NODE_HANDLE: c_int = 4;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_DIRECT_BUS_ACCESS: c_int = 5;
pub const IXGBE_ACI_LINK_TOPO_NODE_CTX_NODE_HANDLE_BUS_ADDRESS: c_int = 6;
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_link_topo_addr {
    pub topo_params: ixgbe_aci_cmd_link_topo_params,
    pub handle: __le16,
// Used to decode the handle field

pub const IXGBE_ACI_LINK_TOPO_HANDLE_BRD_TYPE_MEZZ: c_int = 0;
}

// Get Link Topology Handle (direct, 0x06E0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_link_topo {
    pub addr: ixgbe_aci_cmd_link_topo_addr,
    pub node_part_num: u8,
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_PCA9575: c_uint = 0x21;
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_ZL30632_80032: c_uint = 0x24;
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_SI5384: c_uint = 0x25;
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_C827: c_uint = 0x31;
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_GEN_CLK_MUX: c_uint = 0x47;
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_GEN_GPS: c_uint = 0x48;
pub const IXGBE_ACI_GET_LINK_TOPO_NODE_NR_E610_PTC: c_uint = 0x49;
    pub rsvd: [u8; 9],
}

// Get Link Topology Pin (direct, 0x06E1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_get_link_topo_pin {
    pub addr: ixgbe_aci_cmd_link_topo_addr,
    pub input_io_params: u8,
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_GPIO: c_int = 0;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_RESET_N: c_int = 1;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_INT_N: c_int = 2;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_PRESENT_N: c_int = 3;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_TX_DIS: c_int = 4;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_MODSEL_N: c_int = 5;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_LPMODE: c_int = 6;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_TX_FAULT: c_int = 7;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_RX_LOSS: c_int = 8;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_RS0: c_int = 9;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_RS1: c_int = 10;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_EEPROM_WP: c_int = 11;
// 12 repeats intentionally due to two different uses depending on context
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_LED: c_int = 12;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_RED_LED: c_int = 12;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_GREEN_LED: c_int = 13;
pub const IXGBE_ACI_LINK_TOPO_IO_FUNC_BLUE_LED: c_int = 14;
pub const IXGBE_ACI_LINK_TOPO_INPUT_IO_TYPE_GPIO: c_int = 3;
// Use IXGBE_ACI_LINK_TOPO_NODE_TYPE_* for the type values
    pub output_io_params: u8,
// Use IXGBE_ACI_LINK_TOPO_NODE_TYPE_* for the type values
    pub output_io_flags: u8,
    pub rsvd: [u8; 7],
}

// Set Port Identification LED (direct, 0x06E9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_set_port_id_led {
    pub lport_num: u8,
    pub lport_num_valid: u8,
    pub ident_mode: u8,
    pub rsvd: [u8; 13],
}

pub const IXGBE_ACI_PORT_IDENT_LED_ORIG: c_int = 0;

// Read/Write SFF EEPROM command (indirect 0x06EE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_sff_eeprom {
    pub lport_num: u8,
    pub lport_num_valid: u8,

    pub i2c_bus_addr: __le16,

pub const IXGBE_ACI_SFF_I2CBUS_TYPE_7BIT: c_int = 0;

pub const IXGBE_ACI_SFF_NO_PAGE_BANK_UPDATE: c_int = 0;
pub const IXGBE_ACI_SFF_UPDATE_PAGE: c_int = 1;
pub const IXGBE_ACI_SFF_UPDATE_BANK: c_int = 2;
pub const IXGBE_ACI_SFF_UPDATE_PAGE_BANK: c_int = 3;

    pub i2c_offset: __le16,
    pub module_bank: u8,
    pub module_page: u8,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// NVM Read command (indirect 0x0701)
// NVM Erase commands (direct 0x0702)
// NVM Write commands (indirect 0x0703)
// NVM Write Activate commands (direct 0x0707)
// NVM Shadow RAM Dump commands (direct 0x0707)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_nvm {
pub const IXGBE_ACI_NVM_MAX_OFFSET: c_uint = 0xFFFFFF;
    pub offset_low: __le16,
    pub /: *mut *mut u8 offset_high; / For Write Activate offset_high is used as flags2,

    pub cmd_flags: u8,

pub const IXGBE_ACI_NVM_POR_FLAG: c_int = 0;
pub const IXGBE_ACI_NVM_PERST_FLAG: c_int = 1;
pub const IXGBE_ACI_NVM_EMPR_FLAG: c_int = 2;

pub const IXGBE_ACI_NVM_NO_PRESERVATION: c_uint = 0x0;
pub const IXGBE_ACI_NVM_PRESERVE_SELECTED: c_uint = 0x6;
// For Write Activate, several flags are sent as part of a separate
// flags2 field using a separate byte. For simplicity of the software
// interface, we pass the flags as a 16 bit value so these flags are
// all offset by 8 bits
//

    pub module_typeid: __le16,
    pub length: __le16,
pub const IXGBE_ACI_NVM_ERASE_LEN: c_uint = 0xFFFF;
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// NVM Module_Type ID, needed offset and read_len for
// struct ixgbe_aci_cmd_nvm.
//
pub const IXGBE_ACI_NVM_START_POINT: c_int = 0;
// NVM Checksum Command (direct, 0x0706)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_nvm_checksum {
    pub flags: u8,

    pub rsvd: u8,
    pub /: *mut *mut __le16 checksum; / Used only by response,
pub const IXGBE_ACI_NVM_CHECKSUM_CORRECT: c_uint = 0xBABA;
    pub rsvd2: [u8; 12],
}

// Used for NVM Set Package Data command - 0x070A
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_nvm_pkg_data {
    pub reserved: [u8; 3],
    pub cmd_flags: u8,

    pub reserved1: u32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Used for Pass Component Table command - 0x070B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_nvm_pass_comp_tbl {
    pub /: *mut *mut u8 component_response; / Response only,
pub const IXGBE_ACI_NVM_PASS_COMP_CAN_BE_UPDATED: c_uint = 0x0;
pub const IXGBE_ACI_NVM_PASS_COMP_CAN_MAY_BE_UPDATEABLE: c_uint = 0x1;
pub const IXGBE_ACI_NVM_PASS_COMP_CAN_NOT_BE_UPDATED: c_uint = 0x2;
pub const IXGBE_ACI_NVM_PASS_COMP_PARTIAL_CHECK: c_uint = 0x3;
    pub /: *mut *mut u8 component_response_code; / Response only,
pub const IXGBE_ACI_NVM_PASS_COMP_CAN_BE_UPDATED_CODE: c_uint = 0x0;
pub const IXGBE_ACI_NVM_PASS_COMP_STAMP_IDENTICAL_CODE: c_uint = 0x1;
pub const IXGBE_ACI_NVM_PASS_COMP_STAMP_LOWER: c_uint = 0x2;
pub const IXGBE_ACI_NVM_PASS_COMP_INVALID_STAMP_CODE: c_uint = 0x3;
pub const IXGBE_ACI_NVM_PASS_COMP_CONFLICT_CODE: c_uint = 0x4;
pub const IXGBE_ACI_NVM_PASS_COMP_PRE_REQ_NOT_MET_CODE: c_uint = 0x5;
pub const IXGBE_ACI_NVM_PASS_COMP_NOT_SUPPORTED_CODE: c_uint = 0x6;
pub const IXGBE_ACI_NVM_PASS_COMP_CANNOT_DOWNGRADE_CODE: c_uint = 0x7;
pub const IXGBE_ACI_NVM_PASS_COMP_INCOMPLETE_IMAGE_CODE: c_uint = 0x8;
pub const IXGBE_ACI_NVM_PASS_COMP_VER_STR_IDENTICAL_CODE: c_uint = 0xA;
pub const IXGBE_ACI_NVM_PASS_COMP_VER_STR_LOWER_CODE: c_uint = 0xB;
    pub reserved: u8,
    pub transfer_flag: u8,
    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_cmd_nvm_comp_tbl {
    pub comp_class: __le16,
pub const NVM_COMP_CLASS_ALL_FW: c_uint = 0x000A;
    pub comp_id: __le16,
pub const NVM_COMP_ID_OROM: c_uint = 0x5;
pub const NVM_COMP_ID_NVM: c_uint = 0x6;
pub const NVM_COMP_ID_NETLIST: c_uint = 0x8;
    pub comp_class_idx: u8,
pub const FWU_COMP_CLASS_IDX_NOT_USE: c_uint = 0x0;
    pub comp_cmp_stamp: __le32,
    pub cvs_type: u8,
pub const NVM_CVS_TYPE_ASCII: c_uint = 0x1;
    pub cvs_len: u8,
    pub /: *mut *mut u8 cvs[]; / Component Version String,
    pub __packed: },
// E610-specific adapter context structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_link_status {
// Refer to ixgbe_aci_phy_type for bits definition
    pub phy_type_low: u64,
    pub phy_type_high: u64,
    pub max_frame_size: u16,
    pub link_speed: u16,
    pub req_speeds: u16,
    pub topo_media_conflict: u8,
    pub link_cfg_err: u8,
    pub /: *mut *mut u8 lse_ena; / Link Status Event notification,
    pub link_info: u8,
    pub an_info: u8,
    pub ext_info: u8,
    pub fec_info: u8,
    pub pacing: u8,
// Refer to #define from module_type[IXGBE_ACI_MODULE_TYPE_TOTAL_BYTE]
// of ixgbe_aci_get_phy_caps structure
//
    pub module_type: [u8; IXGBE_ACI_MODULE_TYPE_TOTAL_BYTE],
    pub eee_status: u8,
}

// Common HW capabilities for SW use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hw_caps {
// Write CSR protection
    pub wr_csr_prot: u64,
    pub switching_mode: u32,
// switching mode supported - EVB switching (including cloud)
pub const IXGBE_NVM_IMAGE_TYPE_EVB: c_uint = 0x0;
// Manageability mode & supported protocols over MCTP
    pub mgmt_mode: u32,

    pub mgmt_protocols_mctp: u32,

    pub os2bmc: u32,
    pub valid_functions: u32,
// DCB capabilities
    pub active_tc_bitmap: u32,
    pub maxtc: u32,
// RSS related capabilities
    pub /: *mut *mut u32 rss_table_size; / 512 for PFs and 64 for VFs,
    pub /: *mut *mut u32 rss_table_entry_width; / RSS Entry width in bits,
// Tx/Rx queues
    pub /: *mut *mut u32 num_rxq; / Number/Total Rx queues,
    pub /: *mut *mut u32 rxq_first_id; / First queue ID for Rx queues,
    pub /: *mut *mut u32 num_txq; / Number/Total Tx queues,
    pub /: *mut *mut u32 txq_first_id; / First queue ID for Tx queues,
// MSI-X vectors
    pub num_msix_vectors: u32,
    pub msix_vector_first_id: u32,
// Max MTU for function or device
    pub max_mtu: u32,
// WOL related
    pub num_wol_proxy_fltr: u32,
    pub wol_proxy_vsi_seid: u32,
// LED/SDP pin count
    pub led_pin_num: u32,
    pub sdp_pin_num: u32,
// LED/SDP - Supports up to 12 LED pins and 8 SDP signals
pub const IXGBE_MAX_SUPPORTED_GPIO_LED: c_int = 12;
pub const IXGBE_MAX_SUPPORTED_GPIO_SDP: c_int = 8;
    pub led: [u8; IXGBE_MAX_SUPPORTED_GPIO_LED],
    pub sdp: [u8; IXGBE_MAX_SUPPORTED_GPIO_SDP],
// SR-IOV virtualization
    pub /: *mut *mut u8 sr_iov_1_1; / SR-IOV enabled,
// VMDQ
    pub /: *mut *mut u8 vmdq; / VMDQ supported,
// EVB capabilities
    pub /: *mut *mut u8 evb_802_1_qbg; / Edge Virtual Bridging,
    pub /: *mut *mut u8 evb_802_1_qbh; / Bridge Port Extension,
    pub dcb: u8,
    pub iscsi: u8,
    pub ieee_1588: u8,
    pub mgmt_cem: u8,
// WoL and APM support

    pub apm_wol_support: u8,
    pub acpi_prog_mthd: u8,
    pub proxy_support: u8,
    pub eee_support: u8,
    pub nvm_update_pending_nvm: bool,
    pub nvm_update_pending_orom: bool,
    pub nvm_update_pending_netlist: bool,

    pub sec_rev_disabled: bool,
    pub update_disabled: bool,
    pub nvm_unified_update: bool,
    pub netlist_auth: bool,

    pub no_drop_policy_support: bool,
// PCIe reset avoidance
    pub /: *mut *mut bool pcie_reset_avoidance; / false: not supported, true: supported,
// Post update reset restriction
    pub /: *mut *mut bool reset_restrict_support; / false: not supported, true: supported,
// External topology device images within the NVM
pub const IXGBE_EXT_TOPO_DEV_IMG_COUNT: c_int = 4;
    pub ext_topo_dev_img_ver_high: [u32; IXGBE_EXT_TOPO_DEV_IMG_COUNT],
    pub ext_topo_dev_img_ver_low: [u32; IXGBE_EXT_TOPO_DEV_IMG_COUNT],
    pub ext_topo_dev_img_part_num: [u8; IXGBE_EXT_TOPO_DEV_IMG_COUNT],
pub const IXGBE_EXT_TOPO_DEV_IMG_PART_NUM_S: c_int = 8;
    pub ext_topo_dev_img_load_en: [bool; IXGBE_EXT_TOPO_DEV_IMG_COUNT],    pub ext_topo_dev_img_prog_en: [bool; IXGBE_EXT_TOPO_DEV_IMG_COUNT],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_orom_civd_info {
    pub /: *mut *mut u8 signature[4]; / Must match ASCII '$CIV' characters,
    pub /: *mut *mut u8 checksum; / Simple modulo 256 sum of all structure bytes must equal 0,
    pub /: *mut *mut __le32 combo_ver; / Combo Image Version number,
    pub /: *mut *mut u8 combo_name_len; / Length of the unicode combo image version string, max of 32,
    pub /: *mut *mut __le16 combo_name[32]; / Unicode string representing the Combo Image version,
    pub __packed: },
// Function specific capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hw_func_caps {
    pub /: *mut *mut u32 num_allocd_vfs; / Number of allocated VFs,
    pub /: *mut *mut u32 vf_base_id; / Logical ID of the first VF,
    pub guar_num_vsi: u32,
    pub common_cap: ixgbe_hw_caps,
    pub no_drop_policy_ena: bool,
}

// Device wide capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hw_dev_caps {
    pub common_cap: ixgbe_hw_caps,
    pub /: *mut *mut u32 num_vfs_exposed; / Total number of VFs exposed,
    pub /: *mut *mut u32 num_vsi_allocd_to_host; / Excluding EMP VSI,
    pub /: *mut *mut u32 num_flow_director_fltr; / Number of FD filters available,
    pub num_funcs: u32,
}

// ACI event information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_event {
    pub desc: libie_aq_desc,
    pub msg_buf: *mut u8,
    pub msg_len: u16,
    pub buf_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_aci_info {
    pub /: *mut *mut mutex lock; / admin command interface lock,
    pub /: *mut *mut libie_aq_err last_status; / last status of sent admin command,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_bank_select {
    IXGBE_ACTIVE_FLASH_BANK,
    IXGBE_INACTIVE_FLASH_BANK,
}

// Option ROM version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_orom_info {
    pub /: *mut *mut u8 major; / Major version of OROM,
    pub /: *mut *mut u8 patch; / Patch version of OROM,
    pub /: *mut *mut u16 build; / Build version of OROM,
    pub /: *mut *mut u32 srev; / Security revision,
}

// NVM version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_nvm_info {
    pub eetrack: u32,
    pub srev: u32,
    pub major: u8,
    pub minor: u8,
    pub __packed: },
// netlist version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_netlist_info {
    pub /: *mut *mut u32 major; / major high/low,
    pub /: *mut *mut u32 minor; / minor high/low,
    pub /: *mut *mut u32 type; / type high/low,
    pub /: *mut *mut u32 rev; / revision high/low,
    pub /: *mut *mut u32 hash; / SHA-1 hash word,
    pub /: *mut *mut u16 cust_ver; / customer version,
    pub __packed: },
// Enumeration of possible flash banks for the NVM, OROM, and Netlist modules
// of the flash image.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_flash_bank {
    IXGBE_INVALID_FLASH_BANK,
    IXGBE_1ST_FLASH_BANK,
    IXGBE_2ND_FLASH_BANK,
}

// information for accessing NVM, OROM, and Netlist flash banks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_bank_info {
    pub /: *mut *mut u32 nvm_ptr; / Pointer to 1st NVM bank,
    pub /: *mut *mut u32 nvm_size; / Size of NVM bank,
    pub /: *mut *mut u32 orom_ptr; / Pointer to 1st OROM bank,
    pub /: *mut *mut u32 orom_size; / Size of OROM bank,
    pub /: *mut *mut u32 netlist_ptr; / Ptr to 1st Netlist bank,
    pub /: *mut *mut u32 netlist_size; / Size of Netlist bank,
    pub /: *mut *mut ixgbe_flash_bank nvm_bank; / Active NVM bank,
    pub /: *mut *mut ixgbe_flash_bank orom_bank; / Active OROM bank,
    pub /: *mut *mut ixgbe_flash_bank netlist_bank; / Active Netlist bank,
}

// Flash Chip Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_flash_info {
    pub /: *mut *mut ixgbe_orom_info orom; / Option ROM version info,
    pub /: *mut *mut u32 flash_size; / Available flash size in bytes,
    pub /: *mut *mut ixgbe_nvm_info nvm; / NVM version information,
    pub /: *mut *mut ixgbe_netlist_info netlist; / Netlist version info,
    pub /: *mut *mut ixgbe_bank_info banks; / Flash Bank information,
    pub /: *mut *mut u16 sr_words; / Shadow RAM size in words,
    pub /: *mut *mut u8 blank_nvm_mode; / is NVM empty (no FW present),
}
