//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_link.h
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


// Copyright 2008-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// Unless you and QLogic execute a separate written software license
// agreement governing use of this software, this software is licensed to you
// under the terms of the GNU General Public License version 2, available
// at http://www.gnu.org/licenses/gpl-2.0.html (the "GPL").
//
// Notwithstanding the above, under no circumstances may you combine this
// software in any way with any other Qlogic software provided under a
// license other than the GPL, without Qlogic's express prior written
// consent.
//
// Written by Yaniv Rosner
//
// Defines
//
pub const DEFAULT_PHY_DEV_ADDR: c_int = 3;
pub const E2_DEFAULT_PHY_DEV_ADDR: c_int = 5;

pub const NET_SERDES_IF_XFI: c_int = 1;
pub const NET_SERDES_IF_SFI: c_int = 2;
pub const NET_SERDES_IF_KR: c_int = 3;
pub const NET_SERDES_IF_DXGXS: c_int = 4;
pub const SPEED_AUTO_NEG: c_int = 0;
pub const SPEED_20000: c_int = 20000;
pub const I2C_DEV_ADDR_A0: c_uint = 0xa0;
pub const I2C_DEV_ADDR_A2: c_uint = 0xa2;
pub const SFP_EEPROM_PAGE_SIZE: c_int = 16;
pub const SFP_EEPROM_VENDOR_NAME_ADDR: c_uint = 0x14;
pub const SFP_EEPROM_VENDOR_NAME_SIZE: c_int = 16;
pub const SFP_EEPROM_VENDOR_OUI_ADDR: c_uint = 0x25;
pub const SFP_EEPROM_VENDOR_OUI_SIZE: c_int = 3;
pub const SFP_EEPROM_PART_NO_ADDR: c_uint = 0x28;
pub const SFP_EEPROM_PART_NO_SIZE: c_int = 16;
pub const SFP_EEPROM_REVISION_ADDR: c_uint = 0x38;
pub const SFP_EEPROM_REVISION_SIZE: c_int = 4;
pub const SFP_EEPROM_SERIAL_ADDR: c_uint = 0x44;
pub const SFP_EEPROM_SERIAL_SIZE: c_int = 16;
pub const SFP_EEPROM_DATE_ADDR: c_uint = 0x54 /* ASCII YYMMDD */;
pub const SFP_EEPROM_DATE_SIZE: c_int = 6;
pub const SFP_EEPROM_DIAG_TYPE_ADDR: c_uint = 0x5c;
pub const SFP_EEPROM_DIAG_TYPE_SIZE: c_int = 1;

pub const SFP_EEPROM_SFF_8472_COMP_ADDR: c_uint = 0x5e;
pub const SFP_EEPROM_SFF_8472_COMP_SIZE: c_int = 1;
pub const SFP_EEPROM_A2_CHECKSUM_RANGE: c_uint = 0x5e;
pub const SFP_EEPROM_A2_CC_DMI_ADDR: c_uint = 0x5f;
pub const PWR_FLT_ERR_MSG_LEN: c_int = 250;

// Single Media Direct board is the plain 577xx board with CX4/RJ45 jacks

// Single Media board contains single external phy

// Dual Media board contains two external phy with different media

pub const FW_PARAM_PHY_ADDR_MASK: c_uint = 0x000000FF;
pub const FW_PARAM_PHY_TYPE_MASK: c_uint = 0x0000FF00;
pub const FW_PARAM_MDIO_CTRL_MASK: c_uint = 0xFFFF0000;
pub const FW_PARAM_MDIO_CTRL_OFFSET: c_int = 16;

pub const PFC_BRB_FULL_LB_XOFF_THRESHOLD: c_int = 170;
pub const PFC_BRB_FULL_LB_XON_THRESHOLD: c_int = 250;

pub const BMAC_CONTROL_RX_ENABLE: c_int = 2;
//
// Structs
//
pub const INT_PHY: c_int = 0;
pub const EXT_PHY1: c_int = 1;
pub const EXT_PHY2: c_int = 2;
pub const MAX_PHYS: c_int = 3;
// Same configuration is shared between the XGXS and the first external phy

//
// bnx2x_phy struct
// Defines the required arguments and function per phy
//
extern "C" {
    pub fn int(raw: *mut *mut format_fw_ver_t)(u32, str: *mut u8, len: *mut u16) -> typedef;
}
extern "C" {
    pub fn void(phy: *mut *mut hw_reset_t)(struct bnx2x_phy, params: *mut link_params) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_reg_set {
    pub devad: u8,
    pub reg: u16,
    pub val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_phy {
    pub type: u32,
// Loaded during init
    pub addr: u8,
    pub def_md_devad: u8,
    pub flags: u16,
// No Over-Current detection

// Fan failure detection required

// Initialize first the XGXS and only then the phy itself

// preemphasis values for the rx side
    pub rx_preemphasis: [u16; 4],
// preemphasis values for the tx side
    pub tx_preemphasis: [u16; 4],
// EMAC address for access MDIO
    pub mdio_ctrl: u32,
    pub supported: u32,
    pub media_type: u32,
pub const ETH_PHY_UNSPECIFIED: c_uint = 0x0;
pub const ETH_PHY_SFPP_10G_FIBER: c_uint = 0x1;
pub const ETH_PHY_XFP_FIBER: c_uint = 0x2;
pub const ETH_PHY_DA_TWINAX: c_uint = 0x3;
pub const ETH_PHY_BASE_T: c_uint = 0x4;
pub const ETH_PHY_SFP_1G_FIBER: c_uint = 0x5;
pub const ETH_PHY_KR: c_uint = 0xf0;
pub const ETH_PHY_CX4: c_uint = 0xf1;
pub const ETH_PHY_NOT_PRESENT: c_uint = 0xff;
// The address in which version is located
    pub ver_addr: u32,
    pub req_flow_ctrl: u16,
    pub req_line_speed: u16,
    pub speed_cap_mask: u32,
    pub req_duplex: u16,
    pub rsrv: u16,
// Called per phy/port init, and it configures LASI, speed, autoneg,
    pub config_init: config_init_t,
// Called due to interrupt. It determines the link, speed
    pub read_status: read_status_t,
// Called when driver is unloading. Should reset the phy
    pub link_reset: link_reset_t,
// Set the loopback configuration for the phy
    pub config_loopback: config_loopback_t,
// Format the given raw number into str up to len
    pub format_fw_ver: format_fw_ver_t,
// Reset the phy (both ports)
    pub hw_reset: hw_reset_t,
// Set link led mode (on/off/oper)
    pub set_link_led: set_link_led_t,
// PHY Specific tasks
    pub phy_specific_func: phy_specific_func_t,
pub const DISABLE_TX: c_int = 1;
pub const ENABLE_TX: c_int = 2;
pub const PHY_INIT: c_int = 3;
}

// Inputs parameters to the CLC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_params {
    pub port: u8,
// Default / User Configuration
    pub loopback_mode: u8,
pub const LOOPBACK_NONE: c_int = 0;
pub const LOOPBACK_EMAC: c_int = 1;
pub const LOOPBACK_BMAC: c_int = 2;
pub const LOOPBACK_XGXS: c_int = 3;
pub const LOOPBACK_EXT_PHY: c_int = 4;
pub const LOOPBACK_EXT: c_int = 5;
pub const LOOPBACK_UMAC: c_int = 6;
pub const LOOPBACK_XMAC: c_int = 7;
// Device parameters
    pub mac_addr: [u8; 6],
    pub req_duplex: [u16; LINK_CONFIG_SIZE],
    pub req_flow_ctrl: [u16; LINK_CONFIG_SIZE],
    pub /: *mut *mut u16 req_line_speed[LINK_CONFIG_SIZE]; / Also determine AutoNeg,
// shmem parameters
    pub shmem_base: u32,
    pub shmem2_base: u32,
    pub speed_cap_mask: [u32; LINK_CONFIG_SIZE],
    pub switch_cfg: u32,

    pub lane_config: u32,
// Phy register parameter
    pub chip_id: u32,
// features
    pub feature_config_flags: u32,

// Will be populated during common init
    pub phy: [bnx2x_phy; MAX_PHYS],
// Will be populated during common init
    pub num_phys: u8,
    pub rsrv: u8,
// Used to configure the EEE Tx LPI timer, has several modes of
// operation, according to bits 29:28 -
// 2'b00: Timer will be configured by nvram, output will be the value
// from nvram.
// 2'b01: Timer will be configured by nvram, output will be in
// microseconds.
// 2'b10: bits 1:0 contain an nvram value which will be used instead
// of the one located in the nvram. Output will be that value.
// 2'b11: bits 19:0 contain the idle timer in microseconds; output
// will be in microseconds.
// Bits 31:30 should be 2'b11 in order for EEE to be enabled.
//
    pub eee_mode: u32,

    pub /: *mut *mut u16 hw_led_mode; / part of the hw_config read from the shmem,
    pub multi_phy_config: u32,
// Device pointer passed to all callback functions
    pub bp: *mut bnx2x,
    pub when: *mut *mut u16 req_fc_auto_adv; / Should be set to TX / BOTH,
    pub link_flags: u16,

    pub lfa_base: u32,
// The same definitions as the shmem2 parameter
    pub link_attr_sync: u32,
}

// Output parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_vars {
    pub phy_flags: u8,

    pub mac_type: u8,
pub const MAC_TYPE_NONE: c_int = 0;
pub const MAC_TYPE_EMAC: c_int = 1;
pub const MAC_TYPE_BMAC: c_int = 2;
pub const MAC_TYPE_UMAC: c_int = 3;
pub const MAC_TYPE_XMAC: c_int = 4;
    pub /: *mut *mut u8 phy_link_up; / internal phy link indication,
    pub link_up: u8,
    pub line_speed: u16,
    pub duplex: u16,
    pub flow_ctrl: u16,
    pub ieee_fc: u16,
// The same definitions as the shmem parameter
    pub link_status: u32,
    pub eee_status: u32,
    pub fault_detected: u8,
    pub check_kr2_recovery_cnt: u8,
pub const CHECK_KR2_RECOVERY_CNT: c_int = 5;
    pub periodic_flags: u16,
pub const PERIODIC_FLAGS_LINK_EVENT: c_uint = 0x0001;
    pub aeu_int_mask: u32,
    pub rx_tx_asic_rst: u8,
    pub turn_to_run_wc_rt: u8,
    pub rsrv2: u16,
}

//
// Functions
//
extern "C" {
    pub fn bnx2x_phy_init(params: *mut link_params, vars: *mut link_vars) -> c_int;
}
// Reset the link. Should be called when driver or interface goes down
extern "C" {
    pub fn bnx2x_lfa_reset(params: *mut link_params, vars: *mut link_vars) -> c_int;
}
// bnx2x_link_update should be called upon link interrupt
extern "C" {
    pub fn bnx2x_link_update(params: *mut link_params, vars: *mut link_vars) -> c_int;
}
// use the following phy functions to read/write from external_phy
// Reads the link_status from the shmem,
// returns string representing the fw_version of the external phy
// Set/Unset the led
pub const LED_MODE_OFF: c_int = 0;
pub const LED_MODE_ON: c_int = 1;
pub const LED_MODE_OPER: c_int = 2;
pub const LED_MODE_FRONT_PANEL_OFF: c_int = 3;
// bnx2x_handle_module_detect_int should be called upon module detection
extern "C" {
    pub fn bnx2x_handle_module_detect_int(params: *mut link_params);
}
// Get the actual link status. In case it returns 0, link is up,
// One-time initialization for external phy after power up
// Reset the external PHY using GPIO
extern "C" {
    pub fn bnx2x_ext_phy_hw_reset(bp: *mut bnx2x, port: u8);
}
// Reset the external of SFX7101
extern "C" {
    pub fn bnx2x_sfx7101_sp_sw_reset(bp: *mut bnx2x, phy: *mut bnx2x_phy);
}
// Read "byte_cnt" bytes from address "addr" from the SFP+ EEPROM
extern "C" {
    pub fn bnx2x_hw_reset_phy(params: *mut link_params);
}
// Check swap bit and adjust PHY order
extern "C" {
    pub fn bnx2x_phy_selection(params: *mut link_params) -> u32;
}
// Probe the phys on board, and populate them in "params"
extern "C" {
    pub fn bnx2x_phy_probe(params: *mut link_params) -> c_int;
}
// Checks if fan failure detection is required on one of the phys on board
// Open / close the gate between the NIG and the BRB
extern "C" {
    pub fn bnx2x_set_rx_filter(params: *mut link_params, en: u8);
}
// DCBX structs
// Number of maximum COS per chip

// PFC port configuration params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_nig_brb_pfc_port_params {
// NIG
    pub pause_enable: u32,
    pub llfc_out_en: u32,
    pub llfc_enable: u32,
    pub pkt_priority_to_cos: u32,
    pub num_of_rx_cos_priority_mask: u8,
    pub rx_cos_priority_mask: [u32; DCBX_MAX_NUM_COS],
    pub llfc_high_priority_classes: u32,
    pub llfc_low_priority_classes: u32,
}

// ETS port configuration params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_ets_bw_params {
    pub bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_ets_sp_params {
//
// valid values are 0 - 5. 0 is highest strict priority.
// There can't be two COS's with the same pri.
//
    pub pri: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_cos_state {
    bnx2x_cos_state_strict = 0,
    bnx2x_cos_state_bw = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_ets_cos_params {
    pub state: bnx2x_cos_state,
    pub bw_params: bnx2x_ets_bw_params,
    pub sp_params: bnx2x_ets_sp_params,
    pub params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_ets_params {
    pub entries*/: *mut *mut u8 num_of_cos; / Number of valid COS,
    pub cos: [bnx2x_ets_cos_params; DCBX_MAX_NUM_COS],
}

// Used to update the PFC attributes in EMAC, BMAC, NIG and BRB
// when link is already up
//
// Used to configure the ETS to disable
// Used to configure the ETS to BW limited
// Used to configure the ETS to strict
extern "C" {
    pub fn bnx2x_ets_strict(params: *const link_params, strict_cos: u8) -> c_int;
}
// Configure the COS to ETS according to BW and SP settings.
extern "C" {
    pub fn bnx2x_period_func(params: *mut link_params, vars: *mut link_vars);
}
