//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl/hw_atl_utils.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File hw_atl_utils.h: Declaration of common functions for Atlantic hardware
// abstraction layer.
//

// Hardware tx descriptor
// Hardware tx context descriptor
// Hardware rx descriptor
// Hardware rx descriptor writeback
// Hardware rx HW TIMESTAMP writeback
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_pin_function {
    GPIO_PIN_FUNCTION_NC,
    GPIO_PIN_FUNCTION_VAUX_ENABLE,
    GPIO_PIN_FUNCTION_EFUSE_BURN_ENABLE,
    GPIO_PIN_FUNCTION_SFP_PLUS_DETECT,
    GPIO_PIN_FUNCTION_TX_DISABLE,
    GPIO_PIN_FUNCTION_RATE_SEL_0,
    GPIO_PIN_FUNCTION_RATE_SEL_1,
    GPIO_PIN_FUNCTION_TX_FAULT,
    GPIO_PIN_FUNCTION_PTP0,
    GPIO_PIN_FUNCTION_PTP1,
    GPIO_PIN_FUNCTION_PTP2,
    GPIO_PIN_FUNCTION_SIZE
}

// fw1x structures
// fw2x structures
// Mailbox FW Request interface
pub const HW_AQ_FW_REQUEST_PTP_GPIO_CTRL: c_uint = 0x11;
pub const HW_AQ_FW_REQUEST_PTP_ADJ_FREQ: c_uint = 0x12;
pub const HW_AQ_FW_REQUEST_PTP_ADJ_CLOCK: c_uint = 0x13;
// PTP FW Request
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_msg_type {
    macsec_cfg_msg = 0,
    macsec_add_rx_sc_msg,
    macsec_add_tx_sc_msg,
    macsec_add_rx_sa_msg,
    macsec_add_tx_sa_msg,
    macsec_get_stats_msg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_rx_action_with_traffic {
    HW_ATL_RX_DISCARD,
    HW_ATL_RX_HOST,
    HW_ATL_RX_MNGMNT,
    HW_ATL_RX_HOST_AND_MNGMNT,
    HW_ATL_RX_WOL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_rx_filter_vlan {
    pub enable: u8,
    pub location: u8,
    pub vlan_id: u16,
    pub queue: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_rx_filter_l2 {
    pub queue: i8,
    pub location: u8,
    pub user_priority_en: u8,
    pub user_priority: u8,
    pub ethertype: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_rx_filter_l3l4 {
    pub cmd: u32,
    pub location: u8,
    pub ip_dst: [u32; 4],
    pub ip_src: [u32; 4],
    pub p_dst: u16,
    pub p_src: u16,
    pub is_ipv6: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_rx_protocol_value_l3l4 {
    HW_ATL_RX_TCP,
    HW_ATL_RX_UDP,
    HW_ATL_RX_SCTP,
    HW_ATL_RX_ICMP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_rx_ctrl_registers_l3l4 {
    HW_ATL_RX_ENABLE_MNGMNT_QUEUE_L3L4 = BIT(22),
    HW_ATL_RX_ENABLE_QUEUE_L3L4        = BIT(23),
    HW_ATL_RX_ENABLE_ARP_FLTR_L3       = BIT(24),
    HW_ATL_RX_ENABLE_CMP_PROT_L4       = BIT(25),
    HW_ATL_RX_ENABLE_CMP_DEST_PORT_L4  = BIT(26),
    HW_ATL_RX_ENABLE_CMP_SRC_PORT_L4   = BIT(27),
    HW_ATL_RX_ENABLE_CMP_DEST_ADDR_L3  = BIT(28),
    HW_ATL_RX_ENABLE_CMP_SRC_ADDR_L3   = BIT(29),
    HW_ATL_RX_ENABLE_L3_IPV6           = BIT(30),
    HW_ATL_RX_ENABLE_FLTR_L3L4         = BIT(31)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_atl_utils_fw_state_e {
    MPI_DEINIT = 0,
    MPI_RESET = 1,
    MPI_INIT = 2,
    MPI_POWER = 4,
}

pub const HAL_ATLANTIC_UTILS_FW_MSG_WOL_ADD: c_uint = 0x4U;
pub const HAL_ATLANTIC_UTILS_FW_MSG_WOL_PRIOR: c_uint = 0x10000000U;
pub const HAL_ATLANTIC_UTILS_FW_MSG_WOL_PATTERN: c_uint = 0x1U;
pub const HAL_ATLANTIC_UTILS_FW_MSG_WOL_MAG_PKT: c_uint = 0x2U;
pub const HAL_ATLANTIC_UTILS_FW_MSG_WOL_DEL: c_uint = 0x5U;
pub const HAL_ATLANTIC_UTILS_FW_MSG_ENABLE_WAKEUP: c_uint = 0x6U;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_fw2x_rate {
    FW2X_RATE_100M    = 0x20,
    FW2X_RATE_1G      = 0x100,
    FW2X_RATE_2G5     = 0x200,
    FW2X_RATE_5G      = 0x400,
    FW2X_RATE_10G     = 0x800,
}

// 0x370
// Link capabilities resolution register
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_fw2x_caps_lo {
    CAPS_LO_10BASET_HD        = 0,
    CAPS_LO_10BASET_FD,
    CAPS_LO_100BASETX_HD,
    CAPS_LO_100BASET4_HD,
    CAPS_LO_100BASET2_HD,
    CAPS_LO_100BASETX_FD      = 5,
    CAPS_LO_100BASET2_FD,
    CAPS_LO_1000BASET_HD,
    CAPS_LO_1000BASET_FD,
    CAPS_LO_2P5GBASET_FD,
    CAPS_LO_5GBASET_FD        = 10,
    CAPS_LO_10GBASET_FD,
    CAPS_LO_AUTONEG,
    CAPS_LO_SMBUS_READ,
    CAPS_LO_SMBUS_WRITE,
    CAPS_LO_MACSEC            = 15,
    CAPS_LO_RESERVED1,
    CAPS_LO_WAKE_ON_LINK_FORCED,
    CAPS_LO_HIGH_TEMP_WARNING = 29,
    CAPS_LO_DRIVER_SCRATCHPAD = 30,
    CAPS_LO_GLOBAL_FAULT      = 31
}

// 0x374
// Status register
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_fw2x_caps_hi {
    CAPS_HI_TPO2EN            = 0,
    CAPS_HI_10BASET_EEE,
    CAPS_HI_RESERVED2,
    CAPS_HI_PAUSE,
    CAPS_HI_ASYMMETRIC_PAUSE,
    CAPS_HI_100BASETX_EEE     = 5,
    CAPS_HI_PHY_BUF_SEND,
    CAPS_HI_PHY_BUF_RECV,
    CAPS_HI_1000BASET_FD_EEE,
    CAPS_HI_2P5GBASET_FD_EEE,
    CAPS_HI_5GBASET_FD_EEE    = 10,
    CAPS_HI_10GBASET_FD_EEE,
    CAPS_HI_FW_REQUEST,
    CAPS_HI_PHY_LOG,
    CAPS_HI_EEE_AUTO_DISABLE_SETTINGS,
    CAPS_HI_PFC               = 15,
    CAPS_HI_WAKE_ON_LINK,
    CAPS_HI_CABLE_DIAG,
    CAPS_HI_TEMPERATURE,
    CAPS_HI_DOWNSHIFT,
    CAPS_HI_PTP_AVB_EN_FW2X   = 20,
    CAPS_HI_THERMAL_SHUTDOWN,
    CAPS_HI_LINK_DROP,
    CAPS_HI_SLEEP_PROXY,
    CAPS_HI_WOL,
    CAPS_HI_MAC_STOP          = 25,
    CAPS_HI_EXT_LOOPBACK,
    CAPS_HI_INT_LOOPBACK,
    CAPS_HI_EFUSE_AGENT,
    CAPS_HI_WOL_TIMER,
    CAPS_HI_STATISTICS        = 30,
    CAPS_HI_TRANSACTION_ID,
}

// 0x36C
// Control register
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_fw2x_ctrl {
    CTRL_RESERVED1            = 0,
    CTRL_RESERVED2,
    CTRL_RESERVED3,
    CTRL_PAUSE,
    CTRL_ASYMMETRIC_PAUSE,
    CTRL_RESERVED4            = 5,
    CTRL_RESERVED5,
    CTRL_RESERVED6,
    CTRL_1GBASET_FD_EEE,
    CTRL_2P5GBASET_FD_EEE,
    CTRL_5GBASET_FD_EEE       = 10,
    CTRL_10GBASET_FD_EEE,
    CTRL_THERMAL_SHUTDOWN,
    CTRL_PHY_LOGS,
    CTRL_EEE_AUTO_DISABLE,
    CTRL_PFC                  = 15,
    CTRL_WAKE_ON_LINK,
    CTRL_CABLE_DIAG,
    CTRL_TEMPERATURE,
    CTRL_DOWNSHIFT,
    CTRL_PTP_AVB              = 20,
    CTRL_RESERVED7,
    CTRL_LINK_DROP,
    CTRL_SLEEP_PROXY,
    CTRL_WOL,
    CTRL_MAC_STOP             = 25,
    CTRL_EXT_LOOPBACK,
    CTRL_INT_LOOPBACK,
    CTRL_RESERVED8,
    CTRL_WOL_TIMER,
    CTRL_STATISTICS           = 30,
    CTRL_FORCE_RECONNECT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_atl_caps_ex {
    CAPS_EX_LED_CONTROL       =  0,
    CAPS_EX_LED0_MODE_LO,
    CAPS_EX_LED0_MODE_HI,
    CAPS_EX_LED1_MODE_LO,
    CAPS_EX_LED1_MODE_HI,
    CAPS_EX_LED2_MODE_LO      =  5,
    CAPS_EX_LED2_MODE_HI,
    CAPS_EX_RESERVED07,
    CAPS_EX_RESERVED08,
    CAPS_EX_RESERVED09,
    CAPS_EX_RESERVED10        = 10,
    CAPS_EX_RESERVED11,
    CAPS_EX_RESERVED12,
    CAPS_EX_RESERVED13,
    CAPS_EX_RESERVED14,
    CAPS_EX_RESERVED15        = 15,
    CAPS_EX_PHY_PTP_EN,
    CAPS_EX_MAC_PTP_EN,
    CAPS_EX_EXT_CLK_EN,
    CAPS_EX_SCHED_DMA_EN,
    CAPS_EX_PTP_GPIO_EN       = 20,
    CAPS_EX_UPDATE_SETTINGS,
    CAPS_EX_PHY_CTRL_TS_PIN,
    CAPS_EX_SNR_OPERATING_MARGIN,
    CAPS_EX_RESERVED24,
    CAPS_EX_RESERVED25        = 25,
    CAPS_EX_RESERVED26,
    CAPS_EX_RESERVED27,
    CAPS_EX_RESERVED28,
    CAPS_EX_RESERVED29,
    CAPS_EX_RESERVED30        = 30,
    CAPS_EX_RESERVED31
}

extern "C" {
    pub fn hw_atl_utils_initfw(self: *mut aq_hw_s, fw_ops: *const aq_fw_ops) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_soft_reset(self: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_hw_chip_features_init(self: *mut aq_hw_s, p: *mut u32);
}
extern "C" {
    pub fn hw_atl_utils_mpi_get_link_status(self: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_mbps_2_speed_index(mbps: c_uint) -> c_uint;
}
extern "C" {
    pub fn hw_atl_utils_hw_deinit(self: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_get_fw_version(self: *mut aq_hw_s) -> u32;
}
extern "C" {
    pub fn hw_atl_utils_update_stats(self: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_write_fwcfg_dwords(self: *mut aq_hw_s, p: *mut u32, cnt: u32) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_fw_set_wol(self: *mut aq_hw_s, wol_enabled: bool, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_fw_rpc_call(self: *mut aq_hw_s, rpc_size: c_uint) -> c_int;
}
extern "C" {
    pub fn hw_atl_utils_ver_match(ver_expected: u32, ver_actual: u32) -> bool;
}
