//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_dcb.h
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

pub const I40E_DCBX_STATUS_NOT_STARTED: c_int = 0;
pub const I40E_DCBX_STATUS_IN_PROGRESS: c_int = 1;
pub const I40E_DCBX_STATUS_DONE: c_int = 2;
pub const I40E_DCBX_STATUS_MULTIPLE_PEERS: c_int = 3;
pub const I40E_DCBX_STATUS_DISABLED: c_int = 7;
pub const I40E_TLV_TYPE_END: c_int = 0;
pub const I40E_TLV_TYPE_ORG: c_int = 127;
pub const I40E_IEEE_8021QAZ_OUI: c_uint = 0x0080C2;
pub const I40E_IEEE_SUBTYPE_ETS_CFG: c_int = 9;
pub const I40E_IEEE_SUBTYPE_ETS_REC: c_int = 10;
pub const I40E_IEEE_SUBTYPE_PFC_CFG: c_int = 11;
pub const I40E_IEEE_SUBTYPE_APP_PRI: c_int = 12;
pub const I40E_CEE_DCBX_OUI: c_uint = 0x001b21;
pub const I40E_CEE_DCBX_TYPE: c_int = 2;
pub const I40E_CEE_SUBTYPE_CTRL: c_int = 1;
pub const I40E_CEE_SUBTYPE_PG_CFG: c_int = 2;
pub const I40E_CEE_SUBTYPE_PFC_CFG: c_int = 3;
pub const I40E_CEE_SUBTYPE_APP_PRI: c_int = 4;
pub const I40E_CEE_MAX_FEAT_TYPE: c_int = 3;
pub const I40E_LLDP_CURRENT_STATUS_XL710_OFFSET: c_uint = 0x2B;
pub const I40E_LLDP_CURRENT_STATUS_X722_OFFSET: c_uint = 0x31;
pub const I40E_LLDP_CURRENT_STATUS_OFFSET: c_int = 1;
pub const I40E_LLDP_CURRENT_STATUS_SIZE: c_int = 1;
// Defines for LLDP TLV header
pub const I40E_LLDP_TLV_LEN_SHIFT: c_int = 0;

pub const I40E_LLDP_TLV_TYPE_SHIFT: c_int = 9;

pub const I40E_LLDP_TLV_SUBTYPE_SHIFT: c_int = 0;

pub const I40E_LLDP_TLV_OUI_SHIFT: c_int = 8;

// Defines for IEEE ETS TLV
pub const I40E_IEEE_ETS_MAXTC_SHIFT: c_int = 0;

pub const I40E_IEEE_ETS_CBS_SHIFT: c_int = 6;

pub const I40E_IEEE_ETS_WILLING_SHIFT: c_int = 7;

pub const I40E_IEEE_ETS_PRIO_0_SHIFT: c_int = 0;

pub const I40E_IEEE_ETS_PRIO_1_SHIFT: c_int = 4;

pub const I40E_CEE_PGID_PRIO_0_SHIFT: c_int = 0;

pub const I40E_CEE_PGID_PRIO_1_SHIFT: c_int = 4;

pub const I40E_CEE_PGID_STRICT: c_int = 15;
// Defines for IEEE TSA types
pub const I40E_IEEE_TSA_STRICT: c_int = 0;
pub const I40E_IEEE_TSA_ETS: c_int = 2;
// Defines for IEEE PFC TLV
pub const I40E_DCB_PFC_ENABLED: c_int = 2;
pub const I40E_DCB_PFC_FORCED_NUM_TC: c_int = 2;
pub const I40E_IEEE_PFC_CAP_SHIFT: c_int = 0;

pub const I40E_IEEE_PFC_MBC_SHIFT: c_int = 6;

pub const I40E_IEEE_PFC_WILLING_SHIFT: c_int = 7;

// Defines for IEEE APP TLV
pub const I40E_IEEE_APP_SEL_SHIFT: c_int = 0;

pub const I40E_IEEE_APP_PRIO_SHIFT: c_int = 5;

// TLV definitions for preparing MIB
pub const I40E_TLV_ID_CHASSIS_ID: c_int = 0;
pub const I40E_TLV_ID_PORT_ID: c_int = 1;
pub const I40E_TLV_ID_TIME_TO_LIVE: c_int = 2;
pub const I40E_IEEE_TLV_ID_ETS_CFG: c_int = 3;
pub const I40E_IEEE_TLV_ID_ETS_REC: c_int = 4;
pub const I40E_IEEE_TLV_ID_PFC_CFG: c_int = 5;
pub const I40E_IEEE_TLV_ID_APP_PRI: c_int = 6;
pub const I40E_TLV_ID_END_OF_LLDPPDU: c_int = 7;

pub const I40E_IEEE_TLV_HEADER_LENGTH: c_int = 2;
pub const I40E_IEEE_ETS_TLV_LENGTH: c_int = 25;
pub const I40E_IEEE_PFC_TLV_LENGTH: c_int = 6;
pub const I40E_IEEE_APP_TLV_LENGTH: c_int = 11;
// Defines for default SW DCB config
pub const I40E_IEEE_DEFAULT_ETS_TCBW: c_int = 100;
pub const I40E_IEEE_DEFAULT_ETS_WILLING: c_int = 1;
pub const I40E_IEEE_DEFAULT_PFC_WILLING: c_int = 1;
pub const I40E_IEEE_DEFAULT_NUM_APPS: c_int = 1;
pub const I40E_IEEE_DEFAULT_APP_PRIO: c_int = 3;

// IEEE 802.1AB LLDP Organization specific TLV
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_lldp_org_tlv {
    pub typelength: __be16,
    pub ouisubtype: __be32,
    pub tlvinfo: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_cee_tlv_hdr {
    pub typelen: __be16,
    pub operver: u8,
    pub maxver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_cee_ctrl_tlv {
    pub hdr: i40e_cee_tlv_hdr,
    pub seqno: __be32,
    pub ackno: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_cee_feat_tlv {
    pub hdr: i40e_cee_tlv_hdr,
    pub /: *mut *mut u8 en_will_err; / Bits: |En|Will|Err|Reserved(5)|,
pub const I40E_CEE_FEAT_TLV_ENABLE_MASK: c_uint = 0x80;
pub const I40E_CEE_FEAT_TLV_WILLING_MASK: c_uint = 0x40;
pub const I40E_CEE_FEAT_TLV_ERR_MASK: c_uint = 0x20;
    pub subtype: u8,
    pub tlvinfo: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_cee_app_prio {
    pub protocol: __be16,
    pub /: *mut *mut u8 upper_oui_sel; / Bits: |Upper OUI(6)|Selector(2)|,
pub const I40E_CEE_APP_SELECTOR_MASK: c_uint = 0x03;
    pub lower_oui: __be16,
    pub prio_map: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_get_fw_lldp_status_resp {
    I40E_GET_FW_LLDP_STATUS_DISABLED = 0,
    I40E_GET_FW_LLDP_STATUS_ENABLED = 1
}

// Data structures to pass for SW DCBX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_rx_pb_config {
    pub shared_pool_size: u32,
    pub shared_pool_high_wm: u32,
    pub shared_pool_low_wm: u32,
    pub shared_pool_high_thresh: [u32; I40E_MAX_TRAFFIC_CLASS],
    pub shared_pool_low_thresh: [u32; I40E_MAX_TRAFFIC_CLASS],
    pub tc_pool_size: [u32; I40E_MAX_TRAFFIC_CLASS],
    pub tc_pool_high_wm: [u32; I40E_MAX_TRAFFIC_CLASS],
    pub tc_pool_low_wm: [u32; I40E_MAX_TRAFFIC_CLASS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_dcb_arbiter_mode {
    I40E_DCB_ARB_MODE_STRICT_PRIORITY = 0,
    I40E_DCB_ARB_MODE_ROUND_ROBIN = 1
}

pub const I40E_DCB_DEFAULT_MAX_EXPONENT: c_uint = 0xB;
pub const I40E_DEFAULT_PAUSE_TIME: c_uint = 0xffff;

// BitTimes (BT) conversion

// Max Frame(TC) = MFS(max) + MFS(TC)

// EEE Tx LPI Exit time in Bit Times
pub const I40E_EEE_TX_LPI_EXIT_TIME: c_int = 142500;
// PCI Round Trip Time in Bit Times
pub const I40E_PCIRTT_LINK_SPEED_10G: c_int = 20000;
pub const I40E_PCIRTT_BYTE_LINK_SPEED_20G: c_int = 40000;
pub const I40E_PCIRTT_BYTE_LINK_SPEED_40G: c_int = 80000;
// PFC Frame Delay Bit Times
pub const I40E_PFC_FRAME_DELAY: c_int = 672;
// Worst case Cable (10GBase-T) Delay Bit Times
pub const I40E_CABLE_DELAY: c_int = 5556;
// Higher Layer Delay @10G Bit Times
pub const I40E_HIGHER_LAYER_DELAY_10G: c_int = 6144;
// Interface Delays in Bit Times
// TODO: Add for other link speeds 20G/40G/etc.
pub const I40E_INTERFACE_DELAY_10G_MAC_CONTROL: c_int = 8192;
pub const I40E_INTERFACE_DELAY_10G_MAC: c_int = 8192;
pub const I40E_INTERFACE_DELAY_10G_RS: c_int = 8192;
pub const I40E_INTERFACE_DELAY_XGXS: c_int = 2048;
pub const I40E_INTERFACE_DELAY_XAUI: c_int = 2048;
pub const I40E_INTERFACE_DELAY_10G_BASEX_PCS: c_int = 2048;
pub const I40E_INTERFACE_DELAY_10G_BASER_PCS: c_int = 3584;
pub const I40E_INTERFACE_DELAY_LX4_PMD: c_int = 512;
pub const I40E_INTERFACE_DELAY_CX4_PMD: c_int = 512;
pub const I40E_INTERFACE_DELAY_SERIAL_PMA: c_int = 512;
pub const I40E_INTERFACE_DELAY_PMD: c_int = 512;
pub const I40E_INTERFACE_DELAY_10G_BASET: c_int = 25600;
// Hardware RX DCB config related defines
pub const I40E_DCB_1_PORT_THRESHOLD: c_uint = 0xF;
pub const I40E_DCB_1_PORT_FIFO_SIZE: c_uint = 0x10;
pub const I40E_DCB_2_PORT_THRESHOLD_LOW_NUM_TC: c_uint = 0xF;
pub const I40E_DCB_2_PORT_FIFO_SIZE_LOW_NUM_TC: c_uint = 0x10;
pub const I40E_DCB_2_PORT_THRESHOLD_HIGH_NUM_TC: c_uint = 0xC;
pub const I40E_DCB_2_PORT_FIFO_SIZE_HIGH_NUM_TC: c_uint = 0x8;
pub const I40E_DCB_4_PORT_THRESHOLD_LOW_NUM_TC: c_uint = 0x9;
pub const I40E_DCB_4_PORT_FIFO_SIZE_LOW_NUM_TC: c_uint = 0x8;
pub const I40E_DCB_4_PORT_THRESHOLD_HIGH_NUM_TC: c_uint = 0x6;
pub const I40E_DCB_4_PORT_FIFO_SIZE_HIGH_NUM_TC: c_uint = 0x4;
pub const I40E_DCB_WATERMARK_START_FACTOR: c_uint = 0x2;
// delay values for with 10G BaseT in Bit Times

extern "C" {
    pub fn I40E_DV_TC(_arg: mfs_max, I40E_B2BT(mfs_max: mfs_tc) +) -> return;
}
// APIs for SW DCBX
extern "C" {
    pub fn i40e_dcb_hw_set_num_tc(hw: *mut i40e_hw, num_tc: u8);
}
extern "C" {
    pub fn i40e_dcb_hw_rx_up2tc_config(hw: *mut i40e_hw, prio_tc: *mut u8);
}
extern "C" {
    pub fn i40e_get_dcb_config(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_set_dcb_config(hw: *mut i40e_hw) -> c_int;
}
