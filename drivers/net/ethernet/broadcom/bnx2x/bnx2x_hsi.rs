//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_hsi.h
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


// bnx2x_hsi.h: Qlogic Everest network driver.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const FW_ENCODE_32BIT_PATTERN: c_uint = 0x1e1e1e1e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct license_key {
    pub reserved: [u32; 6],
    pub max_iscsi_conn: u32,
pub const BNX2X_MAX_ISCSI_TRGT_CONN_MASK: c_uint = 0xFFFF;
pub const BNX2X_MAX_ISCSI_TRGT_CONN_SHIFT: c_int = 0;
pub const BNX2X_MAX_ISCSI_INIT_CONN_MASK: c_uint = 0xFFFF0000;
pub const BNX2X_MAX_ISCSI_INIT_CONN_SHIFT: c_int = 16;
    pub reserved_a: u32,
    pub max_fcoe_conn: u32,
pub const BNX2X_MAX_FCOE_TRGT_CONN_MASK: c_uint = 0xFFFF;
pub const BNX2X_MAX_FCOE_TRGT_CONN_SHIFT: c_int = 0;
pub const BNX2X_MAX_FCOE_INIT_CONN_MASK: c_uint = 0xFFFF0000;
pub const BNX2X_MAX_FCOE_INIT_CONN_SHIFT: c_int = 16;
    pub reserved_b: [u32; 4],
}

//
// Shared HW configuration
//
pub const PIN_CFG_NA: c_uint = 0x00000000;
pub const PIN_CFG_GPIO0_P0: c_uint = 0x00000001;
pub const PIN_CFG_GPIO1_P0: c_uint = 0x00000002;
pub const PIN_CFG_GPIO2_P0: c_uint = 0x00000003;
pub const PIN_CFG_GPIO3_P0: c_uint = 0x00000004;
pub const PIN_CFG_GPIO0_P1: c_uint = 0x00000005;
pub const PIN_CFG_GPIO1_P1: c_uint = 0x00000006;
pub const PIN_CFG_GPIO2_P1: c_uint = 0x00000007;
pub const PIN_CFG_GPIO3_P1: c_uint = 0x00000008;
pub const PIN_CFG_EPIO0: c_uint = 0x00000009;
pub const PIN_CFG_EPIO1: c_uint = 0x0000000a;
pub const PIN_CFG_EPIO2: c_uint = 0x0000000b;
pub const PIN_CFG_EPIO3: c_uint = 0x0000000c;
pub const PIN_CFG_EPIO4: c_uint = 0x0000000d;
pub const PIN_CFG_EPIO5: c_uint = 0x0000000e;
pub const PIN_CFG_EPIO6: c_uint = 0x0000000f;
pub const PIN_CFG_EPIO7: c_uint = 0x00000010;
pub const PIN_CFG_EPIO8: c_uint = 0x00000011;
pub const PIN_CFG_EPIO9: c_uint = 0x00000012;
pub const PIN_CFG_EPIO10: c_uint = 0x00000013;
pub const PIN_CFG_EPIO11: c_uint = 0x00000014;
pub const PIN_CFG_EPIO12: c_uint = 0x00000015;
pub const PIN_CFG_EPIO13: c_uint = 0x00000016;
pub const PIN_CFG_EPIO14: c_uint = 0x00000017;
pub const PIN_CFG_EPIO15: c_uint = 0x00000018;
pub const PIN_CFG_EPIO16: c_uint = 0x00000019;
pub const PIN_CFG_EPIO17: c_uint = 0x0000001a;
pub const PIN_CFG_EPIO18: c_uint = 0x0000001b;
pub const PIN_CFG_EPIO19: c_uint = 0x0000001c;
pub const PIN_CFG_EPIO20: c_uint = 0x0000001d;
pub const PIN_CFG_EPIO21: c_uint = 0x0000001e;
pub const PIN_CFG_EPIO22: c_uint = 0x0000001f;
pub const PIN_CFG_EPIO23: c_uint = 0x00000020;
pub const PIN_CFG_EPIO24: c_uint = 0x00000021;
pub const PIN_CFG_EPIO25: c_uint = 0x00000022;
pub const PIN_CFG_EPIO26: c_uint = 0x00000023;
pub const PIN_CFG_EPIO27: c_uint = 0x00000024;
pub const PIN_CFG_EPIO28: c_uint = 0x00000025;
pub const PIN_CFG_EPIO29: c_uint = 0x00000026;
pub const PIN_CFG_EPIO30: c_uint = 0x00000027;
pub const PIN_CFG_EPIO31: c_uint = 0x00000028;
// EPIO definition
pub const EPIO_CFG_NA: c_uint = 0x00000000;
pub const EPIO_CFG_EPIO0: c_uint = 0x00000001;
pub const EPIO_CFG_EPIO1: c_uint = 0x00000002;
pub const EPIO_CFG_EPIO2: c_uint = 0x00000003;
pub const EPIO_CFG_EPIO3: c_uint = 0x00000004;
pub const EPIO_CFG_EPIO4: c_uint = 0x00000005;
pub const EPIO_CFG_EPIO5: c_uint = 0x00000006;
pub const EPIO_CFG_EPIO6: c_uint = 0x00000007;
pub const EPIO_CFG_EPIO7: c_uint = 0x00000008;
pub const EPIO_CFG_EPIO8: c_uint = 0x00000009;
pub const EPIO_CFG_EPIO9: c_uint = 0x0000000a;
pub const EPIO_CFG_EPIO10: c_uint = 0x0000000b;
pub const EPIO_CFG_EPIO11: c_uint = 0x0000000c;
pub const EPIO_CFG_EPIO12: c_uint = 0x0000000d;
pub const EPIO_CFG_EPIO13: c_uint = 0x0000000e;
pub const EPIO_CFG_EPIO14: c_uint = 0x0000000f;
pub const EPIO_CFG_EPIO15: c_uint = 0x00000010;
pub const EPIO_CFG_EPIO16: c_uint = 0x00000011;
pub const EPIO_CFG_EPIO17: c_uint = 0x00000012;
pub const EPIO_CFG_EPIO18: c_uint = 0x00000013;
pub const EPIO_CFG_EPIO19: c_uint = 0x00000014;
pub const EPIO_CFG_EPIO20: c_uint = 0x00000015;
pub const EPIO_CFG_EPIO21: c_uint = 0x00000016;
pub const EPIO_CFG_EPIO22: c_uint = 0x00000017;
pub const EPIO_CFG_EPIO23: c_uint = 0x00000018;
pub const EPIO_CFG_EPIO24: c_uint = 0x00000019;
pub const EPIO_CFG_EPIO25: c_uint = 0x0000001a;
pub const EPIO_CFG_EPIO26: c_uint = 0x0000001b;
pub const EPIO_CFG_EPIO27: c_uint = 0x0000001c;
pub const EPIO_CFG_EPIO28: c_uint = 0x0000001d;
pub const EPIO_CFG_EPIO29: c_uint = 0x0000001e;
pub const EPIO_CFG_EPIO30: c_uint = 0x0000001f;
pub const EPIO_CFG_EPIO31: c_uint = 0x00000020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr {
    pub upper: u32,
    pub lower: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_hw_cfg {
// Up to 16 bytes of NULL-terminated string
    pub /: *mut *mut u8 part_num[16]; / 0x104,
    pub /: *mut *mut u32 config; / 0x114,
pub const SHARED_HW_CFG_MDIO_VOLTAGE_MASK: c_uint = 0x00000001;
pub const SHARED_HW_CFG_MDIO_VOLTAGE_SHIFT: c_int = 0;
pub const SHARED_HW_CFG_MDIO_VOLTAGE_1_2V: c_uint = 0x00000000;
pub const SHARED_HW_CFG_MDIO_VOLTAGE_2_5V: c_uint = 0x00000001;
pub const SHARED_HW_CFG_MCP_RST_ON_CORE_RST_EN: c_uint = 0x00000002;
pub const SHARED_HW_CFG_PORT_SWAP: c_uint = 0x00000004;
pub const SHARED_HW_CFG_BEACON_WOL_EN: c_uint = 0x00000008;
pub const SHARED_HW_CFG_PCIE_GEN3_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_PCIE_GEN3_ENABLED: c_uint = 0x00000010;
pub const SHARED_HW_CFG_MFW_SELECT_MASK: c_uint = 0x00000700;
pub const SHARED_HW_CFG_MFW_SELECT_SHIFT: c_int = 8;
// Whatever MFW found in NVM
pub const SHARED_HW_CFG_MFW_SELECT_DEFAULT: c_uint = 0x00000000;
pub const SHARED_HW_CFG_MFW_SELECT_NC_SI: c_uint = 0x00000100;
pub const SHARED_HW_CFG_MFW_SELECT_UMP: c_uint = 0x00000200;
pub const SHARED_HW_CFG_MFW_SELECT_IPMI: c_uint = 0x00000300;
// Use SPIO4 as an arbiter between: 0-NC_SI, 1-IPMI
pub const SHARED_HW_CFG_MFW_SELECT_SPIO4_NC_SI_IPMI: c_uint = 0x00000400;
// Use SPIO4 as an arbiter between: 0-UMP, 1-IPMI
pub const SHARED_HW_CFG_MFW_SELECT_SPIO4_UMP_IPMI: c_uint = 0x00000500;
// Use SPIO4 as an arbiter between: 0-NC-SI, 1-UMP
pub const SHARED_HW_CFG_MFW_SELECT_SPIO4_NC_SI_UMP: c_uint = 0x00000600;
pub const SHARED_HW_CFG_LED_MODE_MASK: c_uint = 0x000f0000;
pub const SHARED_HW_CFG_LED_MODE_SHIFT: c_int = 16;
pub const SHARED_HW_CFG_LED_MAC1: c_uint = 0x00000000;
pub const SHARED_HW_CFG_LED_PHY1: c_uint = 0x00010000;
pub const SHARED_HW_CFG_LED_PHY2: c_uint = 0x00020000;
pub const SHARED_HW_CFG_LED_PHY3: c_uint = 0x00030000;
pub const SHARED_HW_CFG_LED_MAC2: c_uint = 0x00040000;
pub const SHARED_HW_CFG_LED_PHY4: c_uint = 0x00050000;
pub const SHARED_HW_CFG_LED_PHY5: c_uint = 0x00060000;
pub const SHARED_HW_CFG_LED_PHY6: c_uint = 0x00070000;
pub const SHARED_HW_CFG_LED_MAC3: c_uint = 0x00080000;
pub const SHARED_HW_CFG_LED_PHY7: c_uint = 0x00090000;
pub const SHARED_HW_CFG_LED_PHY9: c_uint = 0x000a0000;
pub const SHARED_HW_CFG_LED_PHY11: c_uint = 0x000b0000;
pub const SHARED_HW_CFG_LED_MAC4: c_uint = 0x000c0000;
pub const SHARED_HW_CFG_LED_PHY8: c_uint = 0x000d0000;
pub const SHARED_HW_CFG_LED_EXTPHY1: c_uint = 0x000e0000;
pub const SHARED_HW_CFG_LED_EXTPHY2: c_uint = 0x000f0000;
pub const SHARED_HW_CFG_AN_ENABLE_MASK: c_uint = 0x3f000000;
pub const SHARED_HW_CFG_AN_ENABLE_SHIFT: c_int = 24;
pub const SHARED_HW_CFG_AN_ENABLE_CL37: c_uint = 0x01000000;
pub const SHARED_HW_CFG_AN_ENABLE_CL73: c_uint = 0x02000000;
pub const SHARED_HW_CFG_AN_ENABLE_BAM: c_uint = 0x04000000;
pub const SHARED_HW_CFG_AN_ENABLE_PARALLEL_DETECTION: c_uint = 0x08000000;
pub const SHARED_HW_CFG_AN_EN_SGMII_FIBER_AUTO_DETECT: c_uint = 0x10000000;
pub const SHARED_HW_CFG_AN_ENABLE_REMOTE_PHY: c_uint = 0x20000000;
pub const SHARED_HW_CFG_SRIOV_MASK: c_uint = 0x40000000;
pub const SHARED_HW_CFG_SRIOV_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_SRIOV_ENABLED: c_uint = 0x40000000;
pub const SHARED_HW_CFG_ATC_MASK: c_uint = 0x80000000;
pub const SHARED_HW_CFG_ATC_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_ATC_ENABLED: c_uint = 0x80000000;
    pub /: *mut *mut u32 config2; / 0x118,
// one time auto detect grace period (in sec)
pub const SHARED_HW_CFG_GRACE_PERIOD_MASK: c_uint = 0x000000ff;
pub const SHARED_HW_CFG_GRACE_PERIOD_SHIFT: c_int = 0;
pub const SHARED_HW_CFG_PCIE_GEN2_ENABLED: c_uint = 0x00000100;
pub const SHARED_HW_CFG_PCIE_GEN2_DISABLED: c_uint = 0x00000000;
// The default value for the core clock is 250MHz and it is
pub const SHARED_HW_CFG_CLOCK_CHANGE_MASK: c_uint = 0x00000e00;
pub const SHARED_HW_CFG_CLOCK_CHANGE_SHIFT: c_int = 9;
pub const SHARED_HW_CFG_SMBUS_TIMING_MASK: c_uint = 0x00001000;
pub const SHARED_HW_CFG_SMBUS_TIMING_100KHZ: c_uint = 0x00000000;
pub const SHARED_HW_CFG_SMBUS_TIMING_400KHZ: c_uint = 0x00001000;
pub const SHARED_HW_CFG_HIDE_PORT1: c_uint = 0x00002000;
pub const SHARED_HW_CFG_WOL_CAPABLE_MASK: c_uint = 0x00004000;
pub const SHARED_HW_CFG_WOL_CAPABLE_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_WOL_CAPABLE_ENABLED: c_uint = 0x00004000;
// Output low when PERST is asserted
pub const SHARED_HW_CFG_SPIO4_FOLLOW_PERST_MASK: c_uint = 0x00008000;
pub const SHARED_HW_CFG_SPIO4_FOLLOW_PERST_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_SPIO4_FOLLOW_PERST_ENABLED: c_uint = 0x00008000;
pub const SHARED_HW_CFG_PCIE_GEN2_PREEMPHASIS_MASK: c_uint = 0x00070000;
pub const SHARED_HW_CFG_PCIE_GEN2_PREEMPHASIS_SHIFT: c_int = 16;
pub const SHARED_HW_CFG_PCIE_GEN2_PREEMPHASIS_HW: c_uint = 0x00000000;
pub const SHARED_HW_CFG_PCIE_GEN2_PREEMPHASIS_0DB: c_uint = 0x00010000;
pub const SHARED_HW_CFG_PCIE_GEN2_PREEMPHASIS_3_5DB: c_uint = 0x00020000;
pub const SHARED_HW_CFG_PCIE_GEN2_PREEMPHASIS_6_0DB: c_uint = 0x00030000;
// The fan failure mechanism is usually related to the PHY type
pub const SHARED_HW_CFG_FAN_FAILURE_MASK: c_uint = 0x00180000;
pub const SHARED_HW_CFG_FAN_FAILURE_SHIFT: c_int = 19;
pub const SHARED_HW_CFG_FAN_FAILURE_PHY_TYPE: c_uint = 0x00000000;
pub const SHARED_HW_CFG_FAN_FAILURE_DISABLED: c_uint = 0x00080000;
pub const SHARED_HW_CFG_FAN_FAILURE_ENABLED: c_uint = 0x00100000;
// ASPM Power Management support
pub const SHARED_HW_CFG_ASPM_SUPPORT_MASK: c_uint = 0x00600000;
pub const SHARED_HW_CFG_ASPM_SUPPORT_SHIFT: c_int = 21;
pub const SHARED_HW_CFG_ASPM_SUPPORT_L0S_L1_ENABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_ASPM_SUPPORT_L0S_DISABLED: c_uint = 0x00200000;
pub const SHARED_HW_CFG_ASPM_SUPPORT_L1_DISABLED: c_uint = 0x00400000;
pub const SHARED_HW_CFG_ASPM_SUPPORT_L0S_L1_DISABLED: c_uint = 0x00600000;
// The value of PM_TL_IGNORE_REQS (bit0) in PCI register
pub const SHARED_HW_CFG_PREVENT_L1_ENTRY_MASK: c_uint = 0x00800000;
pub const SHARED_HW_CFG_PREVENT_L1_ENTRY_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_PREVENT_L1_ENTRY_ENABLED: c_uint = 0x00800000;
pub const SHARED_HW_CFG_PORT_MODE_MASK: c_uint = 0x01000000;
pub const SHARED_HW_CFG_PORT_MODE_2: c_uint = 0x00000000;
pub const SHARED_HW_CFG_PORT_MODE_4: c_uint = 0x01000000;
pub const SHARED_HW_CFG_PATH_SWAP_MASK: c_uint = 0x02000000;
pub const SHARED_HW_CFG_PATH_SWAP_DISABLED: c_uint = 0x00000000;
pub const SHARED_HW_CFG_PATH_SWAP_ENABLED: c_uint = 0x02000000;
// Set the MDC/MDIO access for the first external phy
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_MASK: c_uint = 0x1C000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_SHIFT: c_int = 26;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_PHY_TYPE: c_uint = 0x00000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_EMAC0: c_uint = 0x04000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_EMAC1: c_uint = 0x08000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_BOTH: c_uint = 0x0c000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS1_SWAPPED: c_uint = 0x10000000;
// Set the MDC/MDIO access for the second external phy
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_MASK: c_uint = 0xE0000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_SHIFT: c_int = 29;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_PHY_TYPE: c_uint = 0x00000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_EMAC0: c_uint = 0x20000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_EMAC1: c_uint = 0x40000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_BOTH: c_uint = 0x60000000;
pub const SHARED_HW_CFG_MDC_MDIO_ACCESS2_SWAPPED: c_uint = 0x80000000;
    pub /: *mut *mut u32 config_3; / 0x11C,
pub const SHARED_HW_CFG_EXTENDED_MF_MODE_MASK: c_uint = 0x00000F00;
pub const SHARED_HW_CFG_EXTENDED_MF_MODE_SHIFT: c_int = 8;
pub const SHARED_HW_CFG_EXTENDED_MF_MODE_NPAR1_DOT_5: c_uint = 0x00000000;
pub const SHARED_HW_CFG_EXTENDED_MF_MODE_NPAR2_DOT_0: c_uint = 0x00000100;
    pub /: *mut *mut u32 ump_nc_si_config; / 0x120,
pub const SHARED_HW_CFG_UMP_NC_SI_MII_MODE_MASK: c_uint = 0x00000003;
pub const SHARED_HW_CFG_UMP_NC_SI_MII_MODE_SHIFT: c_int = 0;
pub const SHARED_HW_CFG_UMP_NC_SI_MII_MODE_MAC: c_uint = 0x00000000;
pub const SHARED_HW_CFG_UMP_NC_SI_MII_MODE_PHY: c_uint = 0x00000001;
pub const SHARED_HW_CFG_UMP_NC_SI_MII_MODE_MII: c_uint = 0x00000000;
pub const SHARED_HW_CFG_UMP_NC_SI_MII_MODE_RMII: c_uint = 0x00000002;
pub const SHARED_HW_CFG_UMP_NC_SI_NUM_DEVS_MASK: c_uint = 0x00000f00;
pub const SHARED_HW_CFG_UMP_NC_SI_NUM_DEVS_SHIFT: c_int = 8;
pub const SHARED_HW_CFG_UMP_NC_SI_EXT_PHY_TYPE_MASK: c_uint = 0x00ff0000;
pub const SHARED_HW_CFG_UMP_NC_SI_EXT_PHY_TYPE_SHIFT: c_int = 16;
pub const SHARED_HW_CFG_UMP_NC_SI_EXT_PHY_TYPE_NONE: c_uint = 0x00000000;
pub const SHARED_HW_CFG_UMP_NC_SI_EXT_PHY_TYPE_BCM5221: c_uint = 0x00010000;
    pub /: *mut *mut u32 board; / 0x124,
pub const SHARED_HW_CFG_E3_I2C_MUX0_MASK: c_uint = 0x0000003F;
pub const SHARED_HW_CFG_E3_I2C_MUX0_SHIFT: c_int = 0;
pub const SHARED_HW_CFG_E3_I2C_MUX1_MASK: c_uint = 0x00000FC0;
pub const SHARED_HW_CFG_E3_I2C_MUX1_SHIFT: c_int = 6;
// Use the PIN_CFG_XXX defines on top
pub const SHARED_HW_CFG_BOARD_REV_MASK: c_uint = 0x00ff0000;
pub const SHARED_HW_CFG_BOARD_REV_SHIFT: c_int = 16;
pub const SHARED_HW_CFG_BOARD_MAJOR_VER_MASK: c_uint = 0x0f000000;
pub const SHARED_HW_CFG_BOARD_MAJOR_VER_SHIFT: c_int = 24;
pub const SHARED_HW_CFG_BOARD_MINOR_VER_MASK: c_uint = 0xf0000000;
pub const SHARED_HW_CFG_BOARD_MINOR_VER_SHIFT: c_int = 28;
    pub /: *mut *mut u32 wc_lane_config; / 0x128,
pub const SHARED_HW_CFG_LANE_SWAP_CFG_MASK: c_uint = 0x0000FFFF;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_SHIFT: c_int = 0;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_32103210: c_uint = 0x00001b1b;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_32100123: c_uint = 0x00001be4;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_01233210: c_uint = 0x0000e41b;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_01230123: c_uint = 0x0000e4e4;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_TX_MASK: c_uint = 0x000000FF;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_TX_SHIFT: c_int = 0;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_RX_MASK: c_uint = 0x0000FF00;
pub const SHARED_HW_CFG_LANE_SWAP_CFG_RX_SHIFT: c_int = 8;
// TX lane Polarity swap
pub const SHARED_HW_CFG_TX_LANE0_POL_FLIP_ENABLED: c_uint = 0x00010000;
pub const SHARED_HW_CFG_TX_LANE1_POL_FLIP_ENABLED: c_uint = 0x00020000;
pub const SHARED_HW_CFG_TX_LANE2_POL_FLIP_ENABLED: c_uint = 0x00040000;
pub const SHARED_HW_CFG_TX_LANE3_POL_FLIP_ENABLED: c_uint = 0x00080000;
// TX lane Polarity swap
pub const SHARED_HW_CFG_RX_LANE0_POL_FLIP_ENABLED: c_uint = 0x00100000;
pub const SHARED_HW_CFG_RX_LANE1_POL_FLIP_ENABLED: c_uint = 0x00200000;
pub const SHARED_HW_CFG_RX_LANE2_POL_FLIP_ENABLED: c_uint = 0x00400000;
pub const SHARED_HW_CFG_RX_LANE3_POL_FLIP_ENABLED: c_uint = 0x00800000;
// Selects the port layout of the board
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_MASK: c_uint = 0x0F000000;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_SHIFT: c_int = 24;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_2P_01: c_uint = 0x00000000;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_2P_10: c_uint = 0x01000000;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_4P_0123: c_uint = 0x02000000;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_4P_1032: c_uint = 0x03000000;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_4P_2301: c_uint = 0x04000000;
pub const SHARED_HW_CFG_E3_PORT_LAYOUT_4P_3210: c_uint = 0x05000000;
}

//
// Port HW configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_hw_cfg {
    pub pci_id: u32,
pub const PORT_HW_CFG_PCI_VENDOR_ID_MASK: c_uint = 0xffff0000;
pub const PORT_HW_CFG_PCI_DEVICE_ID_MASK: c_uint = 0x0000ffff;
    pub pci_sub_id: u32,
pub const PORT_HW_CFG_PCI_SUBSYS_DEVICE_ID_MASK: c_uint = 0xffff0000;
pub const PORT_HW_CFG_PCI_SUBSYS_VENDOR_ID_MASK: c_uint = 0x0000ffff;
    pub power_dissipated: u32,
pub const PORT_HW_CFG_POWER_DIS_D0_MASK: c_uint = 0x000000ff;
pub const PORT_HW_CFG_POWER_DIS_D0_SHIFT: c_int = 0;
pub const PORT_HW_CFG_POWER_DIS_D1_MASK: c_uint = 0x0000ff00;
pub const PORT_HW_CFG_POWER_DIS_D1_SHIFT: c_int = 8;
pub const PORT_HW_CFG_POWER_DIS_D2_MASK: c_uint = 0x00ff0000;
pub const PORT_HW_CFG_POWER_DIS_D2_SHIFT: c_int = 16;
pub const PORT_HW_CFG_POWER_DIS_D3_MASK: c_uint = 0xff000000;
pub const PORT_HW_CFG_POWER_DIS_D3_SHIFT: c_int = 24;
    pub power_consumed: u32,
pub const PORT_HW_CFG_POWER_CONS_D0_MASK: c_uint = 0x000000ff;
pub const PORT_HW_CFG_POWER_CONS_D0_SHIFT: c_int = 0;
pub const PORT_HW_CFG_POWER_CONS_D1_MASK: c_uint = 0x0000ff00;
pub const PORT_HW_CFG_POWER_CONS_D1_SHIFT: c_int = 8;
pub const PORT_HW_CFG_POWER_CONS_D2_MASK: c_uint = 0x00ff0000;
pub const PORT_HW_CFG_POWER_CONS_D2_SHIFT: c_int = 16;
pub const PORT_HW_CFG_POWER_CONS_D3_MASK: c_uint = 0xff000000;
pub const PORT_HW_CFG_POWER_CONS_D3_SHIFT: c_int = 24;
    pub mac_upper: u32,
pub const PORT_HW_CFG_UPPERMAC_MASK: c_uint = 0x0000ffff;
pub const PORT_HW_CFG_UPPERMAC_SHIFT: c_int = 0;
    pub mac_lower: u32,
    pub /: *mut *mut u32 iscsi_mac_upper; / Upper 16 bits are always zeroes,
    pub iscsi_mac_lower: u32,
    pub /: *mut *mut u32 rdma_mac_upper; / Upper 16 bits are always zeroes,
    pub rdma_mac_lower: u32,
    pub serdes_config: u32,
pub const PORT_HW_CFG_SERDES_TX_DRV_PRE_EMPHASIS_MASK: c_uint = 0x0000ffff;
pub const PORT_HW_CFG_SERDES_TX_DRV_PRE_EMPHASIS_SHIFT: c_int = 0;
pub const PORT_HW_CFG_SERDES_RX_DRV_EQUALIZER_MASK: c_uint = 0xffff0000;
pub const PORT_HW_CFG_SERDES_RX_DRV_EQUALIZER_SHIFT: c_int = 16;
// Default values: 2P-64, 4P-32
    pub /: *mut *mut u32 pf_config; / 0x158,
pub const PORT_HW_CFG_PF_NUM_VF_MASK: c_uint = 0x0000007F;
pub const PORT_HW_CFG_PF_NUM_VF_SHIFT: c_int = 0;
// Default values: 17
pub const PORT_HW_CFG_PF_NUM_MSIX_VECTORS_MASK: c_uint = 0x00007F00;
pub const PORT_HW_CFG_PF_NUM_MSIX_VECTORS_SHIFT: c_int = 8;
pub const PORT_HW_CFG_ENABLE_FLR_MASK: c_uint = 0x00010000;
pub const PORT_HW_CFG_FLR_ENABLED: c_uint = 0x00010000;
    pub /: *mut *mut u32 vf_config; / 0x15C,
pub const PORT_HW_CFG_VF_NUM_MSIX_VECTORS_MASK: c_uint = 0x0000007F;
pub const PORT_HW_CFG_VF_NUM_MSIX_VECTORS_SHIFT: c_int = 0;
pub const PORT_HW_CFG_VF_PCI_DEVICE_ID_MASK: c_uint = 0xFFFF0000;
pub const PORT_HW_CFG_VF_PCI_DEVICE_ID_SHIFT: c_int = 16;
    pub /: *mut *mut u32 mf_pci_id; / 0x160,
pub const PORT_HW_CFG_MF_PCI_DEVICE_ID_MASK: c_uint = 0x0000FFFF;
pub const PORT_HW_CFG_MF_PCI_DEVICE_ID_SHIFT: c_int = 0;
// Controls the TX laser of the SFP+ module
    pub /: *mut *mut u32 sfp_ctrl; / 0x164,
pub const PORT_HW_CFG_TX_LASER_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_TX_LASER_SHIFT: c_int = 0;
pub const PORT_HW_CFG_TX_LASER_MDIO: c_uint = 0x00000000;
pub const PORT_HW_CFG_TX_LASER_GPIO0: c_uint = 0x00000001;
pub const PORT_HW_CFG_TX_LASER_GPIO1: c_uint = 0x00000002;
pub const PORT_HW_CFG_TX_LASER_GPIO2: c_uint = 0x00000003;
pub const PORT_HW_CFG_TX_LASER_GPIO3: c_uint = 0x00000004;
// Controls the fault module LED of the SFP+
pub const PORT_HW_CFG_FAULT_MODULE_LED_MASK: c_uint = 0x0000FF00;
pub const PORT_HW_CFG_FAULT_MODULE_LED_SHIFT: c_int = 8;
pub const PORT_HW_CFG_FAULT_MODULE_LED_GPIO0: c_uint = 0x00000000;
pub const PORT_HW_CFG_FAULT_MODULE_LED_GPIO1: c_uint = 0x00000100;
pub const PORT_HW_CFG_FAULT_MODULE_LED_GPIO2: c_uint = 0x00000200;
pub const PORT_HW_CFG_FAULT_MODULE_LED_GPIO3: c_uint = 0x00000300;
pub const PORT_HW_CFG_FAULT_MODULE_LED_DISABLED: c_uint = 0x00000400;
// The output pin TX_DIS that controls the TX laser of the SFP+
    pub /: *mut *mut u32 e3_sfp_ctrl; / 0x168,
pub const PORT_HW_CFG_E3_TX_LASER_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_E3_TX_LASER_SHIFT: c_int = 0;
// The output pin for SFPP_TYPE which turns on the Fault module LED
pub const PORT_HW_CFG_E3_FAULT_MDL_LED_MASK: c_uint = 0x0000FF00;
pub const PORT_HW_CFG_E3_FAULT_MDL_LED_SHIFT: c_int = 8;
// The input pin MOD_ABS that indicates whether SFP+ module is
pub const PORT_HW_CFG_E3_MOD_ABS_MASK: c_uint = 0x00FF0000;
pub const PORT_HW_CFG_E3_MOD_ABS_SHIFT: c_int = 16;
// The output pin PWRDIS_SFP_X which disable the power of the SFP+
pub const PORT_HW_CFG_E3_PWR_DIS_MASK: c_uint = 0xFF000000;
pub const PORT_HW_CFG_E3_PWR_DIS_SHIFT: c_int = 24;
//
// The input pin which signals module transmit fault. Use the
// PIN_CFG_XXX defines on top
//
    pub /: *mut *mut u32 e3_cmn_pin_cfg; / 0x16C,
pub const PORT_HW_CFG_E3_TX_FAULT_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_E3_TX_FAULT_SHIFT: c_int = 0;
// The output pin which reset the PHY. Use the PIN_CFG_XXX defines on
pub const PORT_HW_CFG_E3_PHY_RESET_MASK: c_uint = 0x0000FF00;
pub const PORT_HW_CFG_E3_PHY_RESET_SHIFT: c_int = 8;
//
// The output pin which powers down the PHY. Use the PIN_CFG_XXX
// defines on top
//
pub const PORT_HW_CFG_E3_PWR_DOWN_MASK: c_uint = 0x00FF0000;
pub const PORT_HW_CFG_E3_PWR_DOWN_SHIFT: c_int = 16;
// The output pin values BSC_SEL which selects the I2C for this port
pub const PORT_HW_CFG_E3_I2C_MUX0_MASK: c_uint = 0x01000000;
pub const PORT_HW_CFG_E3_I2C_MUX1_MASK: c_uint = 0x02000000;
//
// The input pin I_FAULT which indicate over-current has occurred.
// Use the PIN_CFG_XXX defines on top
//
    pub /: *mut *mut u32 e3_cmn_pin_cfg1; / 0x170,
pub const PORT_HW_CFG_E3_OVER_CURRENT_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_E3_OVER_CURRENT_SHIFT: c_int = 0;
// pause on host ring
    pub /: *mut *mut u32 generic_features; / 0x174,
pub const PORT_HW_CFG_PAUSE_ON_HOST_RING_MASK: c_uint = 0x00000001;
pub const PORT_HW_CFG_PAUSE_ON_HOST_RING_SHIFT: c_int = 0;
pub const PORT_HW_CFG_PAUSE_ON_HOST_RING_DISABLED: c_uint = 0x00000000;
pub const PORT_HW_CFG_PAUSE_ON_HOST_RING_ENABLED: c_uint = 0x00000001;
// SFP+ Tx Equalization: NIC recommended and tested value is 0xBEB2
// LOM recommended and tested value is 0xBEB2. Using a different
// value means using a value not tested by BRCM
//
    pub /: *mut *mut u32 sfi_tap_values; / 0x178,
pub const PORT_HW_CFG_TX_EQUALIZATION_MASK: c_uint = 0x0000FFFF;
pub const PORT_HW_CFG_TX_EQUALIZATION_SHIFT: c_int = 0;
// SFP+ Tx driver broadcast IDRIVER: NIC recommended and tested
// value is 0x2. LOM recommended and tested value is 0x2. Using a
// different value means using a value not tested by BRCM
//
pub const PORT_HW_CFG_TX_DRV_BROADCAST_MASK: c_uint = 0x000F0000;
pub const PORT_HW_CFG_TX_DRV_BROADCAST_SHIFT: c_int = 16;
// Set non-default values for TXFIR in SFP mode.
pub const PORT_HW_CFG_TX_DRV_IFIR_MASK: c_uint = 0x00F00000;
pub const PORT_HW_CFG_TX_DRV_IFIR_SHIFT: c_int = 20;
// Set non-default values for IPREDRIVER in SFP mode.
pub const PORT_HW_CFG_TX_DRV_IPREDRIVER_MASK: c_uint = 0x0F000000;
pub const PORT_HW_CFG_TX_DRV_IPREDRIVER_SHIFT: c_int = 24;
// Set non-default values for POST2 in SFP mode.
pub const PORT_HW_CFG_TX_DRV_POST2_MASK: c_uint = 0xF0000000;
pub const PORT_HW_CFG_TX_DRV_POST2_SHIFT: c_int = 28;
    pub /: *mut *mut u32 reserved0[5]; / 0x17c,
    pub /: *mut *mut u32 aeu_int_mask; / 0x190,
    pub /: *mut *mut u32 media_type; / 0x194,
pub const PORT_HW_CFG_MEDIA_TYPE_PHY0_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_MEDIA_TYPE_PHY0_SHIFT: c_int = 0;
pub const PORT_HW_CFG_MEDIA_TYPE_PHY1_MASK: c_uint = 0x0000FF00;
pub const PORT_HW_CFG_MEDIA_TYPE_PHY1_SHIFT: c_int = 8;
pub const PORT_HW_CFG_MEDIA_TYPE_PHY2_MASK: c_uint = 0x00FF0000;
pub const PORT_HW_CFG_MEDIA_TYPE_PHY2_SHIFT: c_int = 16;
// 4 times 16 bits for all 4 lanes. In case external PHY is present
    pub /: *mut *mut u16 xgxs_config_rx[4]; / 0x198,
    pub /: *mut *mut u16 xgxs_config_tx[4]; / 0x1A0,
// For storing FCOE mac on shared memory
    pub fcoe_fip_mac_upper: u32,
pub const PORT_HW_CFG_FCOE_UPPERMAC_MASK: c_uint = 0x0000ffff;
pub const PORT_HW_CFG_FCOE_UPPERMAC_SHIFT: c_int = 0;
    pub fcoe_fip_mac_lower: u32,
    pub fcoe_wwn_port_name_upper: u32,
    pub fcoe_wwn_port_name_lower: u32,
    pub fcoe_wwn_node_name_upper: u32,
    pub fcoe_wwn_node_name_lower: u32,
    pub /: *mut *mut u32 Reserved1[49]; / 0x1C0,
// Enable RJ45 magjack pair swapping on 10GBase-T PHY (0=default),
    pub /: *mut *mut u32 xgbt_phy_cfg; / 0x284,
pub const PORT_HW_CFG_RJ45_PAIR_SWAP_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_RJ45_PAIR_SWAP_SHIFT: c_int = 0;
    pub /: *mut *mut u32 default_cfg; / 0x288,
pub const PORT_HW_CFG_GPIO0_CONFIG_MASK: c_uint = 0x00000003;
pub const PORT_HW_CFG_GPIO0_CONFIG_SHIFT: c_int = 0;
pub const PORT_HW_CFG_GPIO0_CONFIG_NA: c_uint = 0x00000000;
pub const PORT_HW_CFG_GPIO0_CONFIG_LOW: c_uint = 0x00000001;
pub const PORT_HW_CFG_GPIO0_CONFIG_HIGH: c_uint = 0x00000002;
pub const PORT_HW_CFG_GPIO0_CONFIG_INPUT: c_uint = 0x00000003;
pub const PORT_HW_CFG_GPIO1_CONFIG_MASK: c_uint = 0x0000000C;
pub const PORT_HW_CFG_GPIO1_CONFIG_SHIFT: c_int = 2;
pub const PORT_HW_CFG_GPIO1_CONFIG_NA: c_uint = 0x00000000;
pub const PORT_HW_CFG_GPIO1_CONFIG_LOW: c_uint = 0x00000004;
pub const PORT_HW_CFG_GPIO1_CONFIG_HIGH: c_uint = 0x00000008;
pub const PORT_HW_CFG_GPIO1_CONFIG_INPUT: c_uint = 0x0000000c;
pub const PORT_HW_CFG_GPIO2_CONFIG_MASK: c_uint = 0x00000030;
pub const PORT_HW_CFG_GPIO2_CONFIG_SHIFT: c_int = 4;
pub const PORT_HW_CFG_GPIO2_CONFIG_NA: c_uint = 0x00000000;
pub const PORT_HW_CFG_GPIO2_CONFIG_LOW: c_uint = 0x00000010;
pub const PORT_HW_CFG_GPIO2_CONFIG_HIGH: c_uint = 0x00000020;
pub const PORT_HW_CFG_GPIO2_CONFIG_INPUT: c_uint = 0x00000030;
pub const PORT_HW_CFG_GPIO3_CONFIG_MASK: c_uint = 0x000000C0;
pub const PORT_HW_CFG_GPIO3_CONFIG_SHIFT: c_int = 6;
pub const PORT_HW_CFG_GPIO3_CONFIG_NA: c_uint = 0x00000000;
pub const PORT_HW_CFG_GPIO3_CONFIG_LOW: c_uint = 0x00000040;
pub const PORT_HW_CFG_GPIO3_CONFIG_HIGH: c_uint = 0x00000080;
pub const PORT_HW_CFG_GPIO3_CONFIG_INPUT: c_uint = 0x000000c0;
// When KR link is required to be set to force which is not
pub const PORT_HW_CFG_FORCE_KR_ENABLER_MASK: c_uint = 0x00000F00;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_SHIFT: c_int = 8;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_NOT_FORCED: c_uint = 0x00000000;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO0_P0: c_uint = 0x00000100;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO1_P0: c_uint = 0x00000200;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO2_P0: c_uint = 0x00000300;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO3_P0: c_uint = 0x00000400;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO0_P1: c_uint = 0x00000500;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO1_P1: c_uint = 0x00000600;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO2_P1: c_uint = 0x00000700;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_GPIO3_P1: c_uint = 0x00000800;
pub const PORT_HW_CFG_FORCE_KR_ENABLER_FORCED: c_uint = 0x00000900;
// Enable to determine with which GPIO to reset the external phy
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_MASK: c_uint = 0x000F0000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_SHIFT: c_int = 16;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_PHY_TYPE: c_uint = 0x00000000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO0_P0: c_uint = 0x00010000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO1_P0: c_uint = 0x00020000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO2_P0: c_uint = 0x00030000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO3_P0: c_uint = 0x00040000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO0_P1: c_uint = 0x00050000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO1_P1: c_uint = 0x00060000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO2_P1: c_uint = 0x00070000;
pub const PORT_HW_CFG_EXT_PHY_GPIO_RST_GPIO3_P1: c_uint = 0x00080000;
// Enable BAM on KR
pub const PORT_HW_CFG_ENABLE_BAM_ON_KR_MASK: c_uint = 0x00100000;
pub const PORT_HW_CFG_ENABLE_BAM_ON_KR_SHIFT: c_int = 20;
pub const PORT_HW_CFG_ENABLE_BAM_ON_KR_DISABLED: c_uint = 0x00000000;
pub const PORT_HW_CFG_ENABLE_BAM_ON_KR_ENABLED: c_uint = 0x00100000;
// Enable Common Mode Sense
pub const PORT_HW_CFG_ENABLE_CMS_MASK: c_uint = 0x00200000;
pub const PORT_HW_CFG_ENABLE_CMS_SHIFT: c_int = 21;
pub const PORT_HW_CFG_ENABLE_CMS_DISABLED: c_uint = 0x00000000;
pub const PORT_HW_CFG_ENABLE_CMS_ENABLED: c_uint = 0x00200000;
// Determine the Serdes electrical interface
pub const PORT_HW_CFG_NET_SERDES_IF_MASK: c_uint = 0x0F000000;
pub const PORT_HW_CFG_NET_SERDES_IF_SHIFT: c_int = 24;
pub const PORT_HW_CFG_NET_SERDES_IF_SGMII: c_uint = 0x00000000;
pub const PORT_HW_CFG_NET_SERDES_IF_XFI: c_uint = 0x01000000;
pub const PORT_HW_CFG_NET_SERDES_IF_SFI: c_uint = 0x02000000;
pub const PORT_HW_CFG_NET_SERDES_IF_KR: c_uint = 0x03000000;
pub const PORT_HW_CFG_NET_SERDES_IF_DXGXS: c_uint = 0x04000000;
pub const PORT_HW_CFG_NET_SERDES_IF_KR2: c_uint = 0x05000000;
    pub /: *mut *mut u32 speed_capability_mask2; / 0x28C,
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_MASK: c_uint = 0x0000FFFF;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_SHIFT: c_int = 0;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_10M_FULL: c_uint = 0x00000001;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3__: c_uint = 0x00000002;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3___: c_uint = 0x00000004;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_100M_FULL: c_uint = 0x00000008;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_1G: c_uint = 0x00000010;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_2_DOT_5G: c_uint = 0x00000020;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_10G: c_uint = 0x00000040;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D3_20G: c_uint = 0x00000080;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_MASK: c_uint = 0xFFFF0000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_SHIFT: c_int = 16;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_10M_FULL: c_uint = 0x00010000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0__: c_uint = 0x00020000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0___: c_uint = 0x00040000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_100M_FULL: c_uint = 0x00080000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_1G: c_uint = 0x00100000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_2_DOT_5G: c_uint = 0x00200000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_10G: c_uint = 0x00400000;
pub const PORT_HW_CFG_SPEED_CAPABILITY2_D0_20G: c_uint = 0x00800000;
// In the case where two media types (e.g. copper and fiber) are
    pub /: *mut *mut u32 multi_phy_config; / 0x290,
pub const PORT_HW_CFG_PHY_SELECTION_MASK: c_uint = 0x00000007;
pub const PORT_HW_CFG_PHY_SELECTION_SHIFT: c_int = 0;
pub const PORT_HW_CFG_PHY_SELECTION_HARDWARE_DEFAULT: c_uint = 0x00000000;
pub const PORT_HW_CFG_PHY_SELECTION_FIRST_PHY: c_uint = 0x00000001;
pub const PORT_HW_CFG_PHY_SELECTION_SECOND_PHY: c_uint = 0x00000002;
pub const PORT_HW_CFG_PHY_SELECTION_FIRST_PHY_PRIORITY: c_uint = 0x00000003;
pub const PORT_HW_CFG_PHY_SELECTION_SECOND_PHY_PRIORITY: c_uint = 0x00000004;
// When enabled, all second phy nvram parameters will be swapped
pub const PORT_HW_CFG_PHY_SWAPPED_MASK: c_uint = 0x00000008;
pub const PORT_HW_CFG_PHY_SWAPPED_SHIFT: c_int = 3;
pub const PORT_HW_CFG_PHY_SWAPPED_DISABLED: c_uint = 0x00000000;
pub const PORT_HW_CFG_PHY_SWAPPED_ENABLED: c_uint = 0x00000008;
// Address of the second external phy
    pub /: *mut *mut u32 external_phy_config2; / 0x294,
pub const PORT_HW_CFG_XGXS_EXT_PHY2_ADDR_MASK: c_uint = 0x000000FF;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_ADDR_SHIFT: c_int = 0;
// The second XGXS external PHY type
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_MASK: c_uint = 0x0000FF00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_SHIFT: c_int = 8;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_DIRECT: c_uint = 0x00000000;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8071: c_uint = 0x00000100;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8072: c_uint = 0x00000200;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8073: c_uint = 0x00000300;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8705: c_uint = 0x00000400;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8706: c_uint = 0x00000500;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8726: c_uint = 0x00000600;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8481: c_uint = 0x00000700;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_SFX7101: c_uint = 0x00000800;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8727: c_uint = 0x00000900;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8727_NOC: c_uint = 0x00000a00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM84823: c_uint = 0x00000b00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM54640: c_uint = 0x00000c00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM84833: c_uint = 0x00000d00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM54618SE: c_uint = 0x00000e00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM8722: c_uint = 0x00000f00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM54616: c_uint = 0x00001000;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM84834: c_uint = 0x00001100;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_BCM84858: c_uint = 0x00001200;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_FAILURE: c_uint = 0x0000fd00;
pub const PORT_HW_CFG_XGXS_EXT_PHY2_TYPE_NOT_CONN: c_uint = 0x0000ff00;
// 4 times 16 bits for all 4 lanes. For some external PHYs (such as
    pub /: *mut *mut u16 xgxs_config2_rx[4]; / 0x296,
    pub /: *mut *mut u16 xgxs_config2_tx[4]; / 0x2A0,
    pub lane_config: u32,
pub const PORT_HW_CFG_LANE_SWAP_CFG_MASK: c_uint = 0x0000ffff;
pub const PORT_HW_CFG_LANE_SWAP_CFG_SHIFT: c_int = 0;
// AN and forced
pub const PORT_HW_CFG_LANE_SWAP_CFG_01230123: c_uint = 0x00001b1b;
// forced only
pub const PORT_HW_CFG_LANE_SWAP_CFG_01233210: c_uint = 0x00001be4;
// forced only
pub const PORT_HW_CFG_LANE_SWAP_CFG_31203120: c_uint = 0x0000d8d8;
// forced only
pub const PORT_HW_CFG_LANE_SWAP_CFG_32103210: c_uint = 0x0000e4e4;
pub const PORT_HW_CFG_LANE_SWAP_CFG_TX_MASK: c_uint = 0x000000ff;
pub const PORT_HW_CFG_LANE_SWAP_CFG_TX_SHIFT: c_int = 0;
pub const PORT_HW_CFG_LANE_SWAP_CFG_RX_MASK: c_uint = 0x0000ff00;
pub const PORT_HW_CFG_LANE_SWAP_CFG_RX_SHIFT: c_int = 8;
pub const PORT_HW_CFG_LANE_SWAP_CFG_MASTER_MASK: c_uint = 0x0000c000;
pub const PORT_HW_CFG_LANE_SWAP_CFG_MASTER_SHIFT: c_int = 14;
// Indicate whether to swap the external phy polarity
pub const PORT_HW_CFG_SWAP_PHY_POLARITY_MASK: c_uint = 0x00010000;
pub const PORT_HW_CFG_SWAP_PHY_POLARITY_DISABLED: c_uint = 0x00000000;
pub const PORT_HW_CFG_SWAP_PHY_POLARITY_ENABLED: c_uint = 0x00010000;
    pub external_phy_config: u32,
pub const PORT_HW_CFG_XGXS_EXT_PHY_ADDR_MASK: c_uint = 0x000000ff;
pub const PORT_HW_CFG_XGXS_EXT_PHY_ADDR_SHIFT: c_int = 0;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_MASK: c_uint = 0x0000ff00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_SHIFT: c_int = 8;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_DIRECT: c_uint = 0x00000000;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8071: c_uint = 0x00000100;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8072: c_uint = 0x00000200;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8073: c_uint = 0x00000300;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8705: c_uint = 0x00000400;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8706: c_uint = 0x00000500;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8726: c_uint = 0x00000600;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8481: c_uint = 0x00000700;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_SFX7101: c_uint = 0x00000800;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8727: c_uint = 0x00000900;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8727_NOC: c_uint = 0x00000a00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM84823: c_uint = 0x00000b00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM54640: c_uint = 0x00000c00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM84833: c_uint = 0x00000d00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM54618SE: c_uint = 0x00000e00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM8722: c_uint = 0x00000f00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM54616: c_uint = 0x00001000;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM84834: c_uint = 0x00001100;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_BCM84858: c_uint = 0x00001200;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_DIRECT_WC: c_uint = 0x0000fc00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_FAILURE: c_uint = 0x0000fd00;
pub const PORT_HW_CFG_XGXS_EXT_PHY_TYPE_NOT_CONN: c_uint = 0x0000ff00;
pub const PORT_HW_CFG_SERDES_EXT_PHY_ADDR_MASK: c_uint = 0x00ff0000;
pub const PORT_HW_CFG_SERDES_EXT_PHY_ADDR_SHIFT: c_int = 16;
pub const PORT_HW_CFG_SERDES_EXT_PHY_TYPE_MASK: c_uint = 0xff000000;
pub const PORT_HW_CFG_SERDES_EXT_PHY_TYPE_SHIFT: c_int = 24;
pub const PORT_HW_CFG_SERDES_EXT_PHY_TYPE_DIRECT: c_uint = 0x00000000;
pub const PORT_HW_CFG_SERDES_EXT_PHY_TYPE_BCM5482: c_uint = 0x01000000;
pub const PORT_HW_CFG_SERDES_EXT_PHY_TYPE_DIRECT_SD: c_uint = 0x02000000;
pub const PORT_HW_CFG_SERDES_EXT_PHY_TYPE_NOT_CONN: c_uint = 0xff000000;
    pub speed_capability_mask: u32,
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_MASK: c_uint = 0x0000ffff;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_SHIFT: c_int = 0;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_10M_FULL: c_uint = 0x00000001;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_10M_HALF: c_uint = 0x00000002;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_100M_HALF: c_uint = 0x00000004;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_100M_FULL: c_uint = 0x00000008;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_1G: c_uint = 0x00000010;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_2_5G: c_uint = 0x00000020;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_10G: c_uint = 0x00000040;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_20G: c_uint = 0x00000080;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D3_RESERVED: c_uint = 0x0000f000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_MASK: c_uint = 0xffff0000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_SHIFT: c_int = 16;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_10M_FULL: c_uint = 0x00010000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_10M_HALF: c_uint = 0x00020000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_100M_HALF: c_uint = 0x00040000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_100M_FULL: c_uint = 0x00080000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_1G: c_uint = 0x00100000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_2_5G: c_uint = 0x00200000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_10G: c_uint = 0x00400000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_20G: c_uint = 0x00800000;
pub const PORT_HW_CFG_SPEED_CAPABILITY_D0_RESERVED: c_uint = 0xf0000000;
// A place to hold the original MAC address as a backup
    pub /: *mut *mut u32 backup_mac_upper; / 0x2B4,
    pub /: *mut *mut u32 backup_mac_lower; / 0x2B8,
}

//
// Shared Feature configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_feat_cfg {
    pub /: *mut *mut u32 config; / 0x450,
pub const SHARED_FEATURE_BMC_ECHO_MODE_EN: c_uint = 0x00000001;
// Use NVRAM values instead of HW default values

pub const SHARED_FEAT_CFG_NCSI_ID_METHOD_MASK: c_uint = 0x00000008;
pub const SHARED_FEAT_CFG_NCSI_ID_METHOD_SPIO: c_uint = 0x00000000;
pub const SHARED_FEAT_CFG_NCSI_ID_METHOD_NVRAM: c_uint = 0x00000008;
pub const SHARED_FEAT_CFG_NCSI_ID_MASK: c_uint = 0x00000030;
pub const SHARED_FEAT_CFG_NCSI_ID_SHIFT: c_int = 4;
// Override the OTP back to single function mode. When using GPIO,
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_MASK: c_uint = 0x00000700;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_SHIFT: c_int = 8;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_MF_ALLOWED: c_uint = 0x00000000;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_FORCED_SF: c_uint = 0x00000100;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_SPIO4: c_uint = 0x00000200;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_SWITCH_INDEPT: c_uint = 0x00000300;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_AFEX_MODE: c_uint = 0x00000400;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_BD_MODE: c_uint = 0x00000500;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_UFP_MODE: c_uint = 0x00000600;
pub const SHARED_FEAT_CFG_FORCE_SF_MODE_EXTENDED_MODE: c_uint = 0x00000700;
// The interval in seconds between sending LLDP packets. Set to zero
pub const SHARED_FEAT_CFG_LLDP_XMIT_INTERVAL_MASK: c_uint = 0x00ff0000;
pub const SHARED_FEAT_CFG_LLDP_XMIT_INTERVAL_SHIFT: c_int = 16;
// The assigned device type ID for LLDP usage
pub const SHARED_FEAT_CFG_LLDP_DEVICE_TYPE_ID_MASK: c_uint = 0xff000000;
pub const SHARED_FEAT_CFG_LLDP_DEVICE_TYPE_ID_SHIFT: c_int = 24;
}

//
// Port Feature configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_feat_cfg {
    pub config: u32,
pub const PORT_FEATURE_BAR1_SIZE_MASK: c_uint = 0x0000000f;
pub const PORT_FEATURE_BAR1_SIZE_SHIFT: c_int = 0;
pub const PORT_FEATURE_BAR1_SIZE_DISABLED: c_uint = 0x00000000;
pub const PORT_FEATURE_BAR1_SIZE_64K: c_uint = 0x00000001;
pub const PORT_FEATURE_BAR1_SIZE_128K: c_uint = 0x00000002;
pub const PORT_FEATURE_BAR1_SIZE_256K: c_uint = 0x00000003;
pub const PORT_FEATURE_BAR1_SIZE_512K: c_uint = 0x00000004;
pub const PORT_FEATURE_BAR1_SIZE_1M: c_uint = 0x00000005;
pub const PORT_FEATURE_BAR1_SIZE_2M: c_uint = 0x00000006;
pub const PORT_FEATURE_BAR1_SIZE_4M: c_uint = 0x00000007;
pub const PORT_FEATURE_BAR1_SIZE_8M: c_uint = 0x00000008;
pub const PORT_FEATURE_BAR1_SIZE_16M: c_uint = 0x00000009;
pub const PORT_FEATURE_BAR1_SIZE_32M: c_uint = 0x0000000a;
pub const PORT_FEATURE_BAR1_SIZE_64M: c_uint = 0x0000000b;
pub const PORT_FEATURE_BAR1_SIZE_128M: c_uint = 0x0000000c;
pub const PORT_FEATURE_BAR1_SIZE_256M: c_uint = 0x0000000d;
pub const PORT_FEATURE_BAR1_SIZE_512M: c_uint = 0x0000000e;
pub const PORT_FEATURE_BAR1_SIZE_1G: c_uint = 0x0000000f;
pub const PORT_FEATURE_BAR2_SIZE_MASK: c_uint = 0x000000f0;
pub const PORT_FEATURE_BAR2_SIZE_SHIFT: c_int = 4;
pub const PORT_FEATURE_BAR2_SIZE_DISABLED: c_uint = 0x00000000;
pub const PORT_FEATURE_BAR2_SIZE_64K: c_uint = 0x00000010;
pub const PORT_FEATURE_BAR2_SIZE_128K: c_uint = 0x00000020;
pub const PORT_FEATURE_BAR2_SIZE_256K: c_uint = 0x00000030;
pub const PORT_FEATURE_BAR2_SIZE_512K: c_uint = 0x00000040;
pub const PORT_FEATURE_BAR2_SIZE_1M: c_uint = 0x00000050;
pub const PORT_FEATURE_BAR2_SIZE_2M: c_uint = 0x00000060;
pub const PORT_FEATURE_BAR2_SIZE_4M: c_uint = 0x00000070;
pub const PORT_FEATURE_BAR2_SIZE_8M: c_uint = 0x00000080;
pub const PORT_FEATURE_BAR2_SIZE_16M: c_uint = 0x00000090;
pub const PORT_FEATURE_BAR2_SIZE_32M: c_uint = 0x000000a0;
pub const PORT_FEATURE_BAR2_SIZE_64M: c_uint = 0x000000b0;
pub const PORT_FEATURE_BAR2_SIZE_128M: c_uint = 0x000000c0;
pub const PORT_FEATURE_BAR2_SIZE_256M: c_uint = 0x000000d0;
pub const PORT_FEATURE_BAR2_SIZE_512M: c_uint = 0x000000e0;
pub const PORT_FEATURE_BAR2_SIZE_1G: c_uint = 0x000000f0;
pub const PORT_FEAT_CFG_DCBX_MASK: c_uint = 0x00000100;
pub const PORT_FEAT_CFG_DCBX_DISABLED: c_uint = 0x00000000;
pub const PORT_FEAT_CFG_DCBX_ENABLED: c_uint = 0x00000100;
pub const PORT_FEAT_CFG_STORAGE_PERSONALITY_MASK: c_uint = 0x00000C00;
pub const PORT_FEAT_CFG_STORAGE_PERSONALITY_FCOE: c_uint = 0x00000400;
pub const PORT_FEAT_CFG_STORAGE_PERSONALITY_ISCSI: c_uint = 0x00000800;
pub const PORT_FEATURE_EN_SIZE_MASK: c_uint = 0x0f000000;
pub const PORT_FEATURE_EN_SIZE_SHIFT: c_int = 24;
pub const PORT_FEATURE_WOL_ENABLED: c_uint = 0x01000000;
pub const PORT_FEATURE_MBA_ENABLED: c_uint = 0x02000000;
pub const PORT_FEATURE_MFW_ENABLED: c_uint = 0x04000000;
// Advertise expansion ROM even if MBA is disabled
pub const PORT_FEAT_CFG_FORCE_EXP_ROM_ADV_MASK: c_uint = 0x08000000;
pub const PORT_FEAT_CFG_FORCE_EXP_ROM_ADV_DISABLED: c_uint = 0x00000000;
pub const PORT_FEAT_CFG_FORCE_EXP_ROM_ADV_ENABLED: c_uint = 0x08000000;
// Check the optic vendor via i2c against a list of approved modules
pub const PORT_FEAT_CFG_OPT_MDL_ENFRCMNT_MASK: c_uint = 0xe0000000;
pub const PORT_FEAT_CFG_OPT_MDL_ENFRCMNT_SHIFT: c_int = 29;

pub const PORT_FEAT_CFG_OPT_MDL_ENFRCMNT_WARNING_MSG: c_uint = 0x40000000;
pub const PORT_FEAT_CFG_OPT_MDL_ENFRCMNT_POWER_DOWN: c_uint = 0x60000000;
    pub wol_config: u32,
// Default is used when driver sets to "auto" mode
pub const PORT_FEATURE_WOL_DEFAULT_MASK: c_uint = 0x00000003;
pub const PORT_FEATURE_WOL_DEFAULT_SHIFT: c_int = 0;
pub const PORT_FEATURE_WOL_DEFAULT_DISABLE: c_uint = 0x00000000;
pub const PORT_FEATURE_WOL_DEFAULT_MAGIC: c_uint = 0x00000001;
pub const PORT_FEATURE_WOL_DEFAULT_ACPI: c_uint = 0x00000002;
pub const PORT_FEATURE_WOL_DEFAULT_MAGIC_AND_ACPI: c_uint = 0x00000003;
pub const PORT_FEATURE_WOL_RES_PAUSE_CAP: c_uint = 0x00000004;
pub const PORT_FEATURE_WOL_RES_ASYM_PAUSE_CAP: c_uint = 0x00000008;
pub const PORT_FEATURE_WOL_ACPI_UPON_MGMT: c_uint = 0x00000010;
    pub mba_config: u32,
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_MASK: c_uint = 0x00000007;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_SHIFT: c_int = 0;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_PXE: c_uint = 0x00000000;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_RPL: c_uint = 0x00000001;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_BOOTP: c_uint = 0x00000002;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_ISCSIB: c_uint = 0x00000003;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_FCOE_BOOT: c_uint = 0x00000004;
pub const PORT_FEATURE_MBA_BOOT_AGENT_TYPE_NONE: c_uint = 0x00000007;
pub const PORT_FEATURE_MBA_BOOT_RETRY_MASK: c_uint = 0x00000038;
pub const PORT_FEATURE_MBA_BOOT_RETRY_SHIFT: c_int = 3;
pub const PORT_FEATURE_MBA_RES_PAUSE_CAP: c_uint = 0x00000100;
pub const PORT_FEATURE_MBA_RES_ASYM_PAUSE_CAP: c_uint = 0x00000200;
pub const PORT_FEATURE_MBA_SETUP_PROMPT_ENABLE: c_uint = 0x00000400;
pub const PORT_FEATURE_MBA_HOTKEY_MASK: c_uint = 0x00000800;
pub const PORT_FEATURE_MBA_HOTKEY_CTRL_S: c_uint = 0x00000000;
pub const PORT_FEATURE_MBA_HOTKEY_CTRL_B: c_uint = 0x00000800;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_MASK: c_uint = 0x000ff000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_SHIFT: c_int = 12;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_DISABLED: c_uint = 0x00000000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_2K: c_uint = 0x00001000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_4K: c_uint = 0x00002000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_8K: c_uint = 0x00003000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_16K: c_uint = 0x00004000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_32K: c_uint = 0x00005000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_64K: c_uint = 0x00006000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_128K: c_uint = 0x00007000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_256K: c_uint = 0x00008000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_512K: c_uint = 0x00009000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_1M: c_uint = 0x0000a000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_2M: c_uint = 0x0000b000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_4M: c_uint = 0x0000c000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_8M: c_uint = 0x0000d000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_16M: c_uint = 0x0000e000;
pub const PORT_FEATURE_MBA_EXP_ROM_SIZE_32M: c_uint = 0x0000f000;
pub const PORT_FEATURE_MBA_MSG_TIMEOUT_MASK: c_uint = 0x00f00000;
pub const PORT_FEATURE_MBA_MSG_TIMEOUT_SHIFT: c_int = 20;
pub const PORT_FEATURE_MBA_BIOS_BOOTSTRAP_MASK: c_uint = 0x03000000;
pub const PORT_FEATURE_MBA_BIOS_BOOTSTRAP_SHIFT: c_int = 24;
pub const PORT_FEATURE_MBA_BIOS_BOOTSTRAP_AUTO: c_uint = 0x00000000;
pub const PORT_FEATURE_MBA_BIOS_BOOTSTRAP_BBS: c_uint = 0x01000000;
pub const PORT_FEATURE_MBA_BIOS_BOOTSTRAP_INT18H: c_uint = 0x02000000;
pub const PORT_FEATURE_MBA_BIOS_BOOTSTRAP_INT19H: c_uint = 0x03000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_MASK: c_uint = 0x3c000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_SHIFT: c_int = 26;
pub const PORT_FEATURE_MBA_LINK_SPEED_AUTO: c_uint = 0x00000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_10HD: c_uint = 0x04000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_10FD: c_uint = 0x08000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_100HD: c_uint = 0x0c000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_100FD: c_uint = 0x10000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_1GBPS: c_uint = 0x14000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_2_5GBPS: c_uint = 0x18000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_10GBPS_CX4: c_uint = 0x1c000000;
pub const PORT_FEATURE_MBA_LINK_SPEED_20GBPS: c_uint = 0x20000000;
    pub bmc_config: u32,
pub const PORT_FEATURE_BMC_LINK_OVERRIDE_MASK: c_uint = 0x00000001;
pub const PORT_FEATURE_BMC_LINK_OVERRIDE_DEFAULT: c_uint = 0x00000000;
pub const PORT_FEATURE_BMC_LINK_OVERRIDE_EN: c_uint = 0x00000001;
    pub mba_vlan_cfg: u32,
pub const PORT_FEATURE_MBA_VLAN_TAG_MASK: c_uint = 0x0000ffff;
pub const PORT_FEATURE_MBA_VLAN_TAG_SHIFT: c_int = 0;
pub const PORT_FEATURE_MBA_VLAN_EN: c_uint = 0x00010000;
    pub resource_cfg: u32,
pub const PORT_FEATURE_RESOURCE_CFG_VALID: c_uint = 0x00000001;
pub const PORT_FEATURE_RESOURCE_CFG_DIAG: c_uint = 0x00000002;
pub const PORT_FEATURE_RESOURCE_CFG_L2: c_uint = 0x00000004;
pub const PORT_FEATURE_RESOURCE_CFG_ISCSI: c_uint = 0x00000008;
pub const PORT_FEATURE_RESOURCE_CFG_RDMA: c_uint = 0x00000010;
    pub smbus_config: u32,
pub const PORT_FEATURE_SMBUS_ADDR_MASK: c_uint = 0x000000fe;
pub const PORT_FEATURE_SMBUS_ADDR_SHIFT: c_int = 1;
    pub vf_config: u32,
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_MASK: c_uint = 0x0000000f;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_SHIFT: c_int = 0;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_DISABLED: c_uint = 0x00000000;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_4K: c_uint = 0x00000001;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_8K: c_uint = 0x00000002;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_16K: c_uint = 0x00000003;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_32K: c_uint = 0x00000004;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_64K: c_uint = 0x00000005;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_128K: c_uint = 0x00000006;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_256K: c_uint = 0x00000007;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_512K: c_uint = 0x00000008;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_1M: c_uint = 0x00000009;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_2M: c_uint = 0x0000000a;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_4M: c_uint = 0x0000000b;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_8M: c_uint = 0x0000000c;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_16M: c_uint = 0x0000000d;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_32M: c_uint = 0x0000000e;
pub const PORT_FEAT_CFG_VF_BAR2_SIZE_64M: c_uint = 0x0000000f;
    pub /: *mut *mut u32 link_config; / Used as HW defaults for the driver,
pub const PORT_FEATURE_CONNECTED_SWITCH_MASK: c_uint = 0x03000000;
pub const PORT_FEATURE_CONNECTED_SWITCH_SHIFT: c_int = 24;
// (forced) low speed switch (< 10G)
pub const PORT_FEATURE_CON_SWITCH_1G_SWITCH: c_uint = 0x00000000;
// (forced) high speed switch (>= 10G)
pub const PORT_FEATURE_CON_SWITCH_10G_SWITCH: c_uint = 0x01000000;
pub const PORT_FEATURE_CON_SWITCH_AUTO_DETECT: c_uint = 0x02000000;
pub const PORT_FEATURE_CON_SWITCH_ONE_TIME_DETECT: c_uint = 0x03000000;
pub const PORT_FEATURE_LINK_SPEED_MASK: c_uint = 0x000f0000;
pub const PORT_FEATURE_LINK_SPEED_SHIFT: c_int = 16;
pub const PORT_FEATURE_LINK_SPEED_AUTO: c_uint = 0x00000000;
pub const PORT_FEATURE_LINK_SPEED_10M_FULL: c_uint = 0x00010000;
pub const PORT_FEATURE_LINK_SPEED_10M_HALF: c_uint = 0x00020000;
pub const PORT_FEATURE_LINK_SPEED_100M_HALF: c_uint = 0x00030000;
pub const PORT_FEATURE_LINK_SPEED_100M_FULL: c_uint = 0x00040000;
pub const PORT_FEATURE_LINK_SPEED_1G: c_uint = 0x00050000;
pub const PORT_FEATURE_LINK_SPEED_2_5G: c_uint = 0x00060000;
pub const PORT_FEATURE_LINK_SPEED_10G_CX4: c_uint = 0x00070000;
pub const PORT_FEATURE_LINK_SPEED_20G: c_uint = 0x00080000;
pub const PORT_FEATURE_FLOW_CONTROL_MASK: c_uint = 0x00000700;
pub const PORT_FEATURE_FLOW_CONTROL_SHIFT: c_int = 8;
pub const PORT_FEATURE_FLOW_CONTROL_AUTO: c_uint = 0x00000000;
pub const PORT_FEATURE_FLOW_CONTROL_TX: c_uint = 0x00000100;
pub const PORT_FEATURE_FLOW_CONTROL_RX: c_uint = 0x00000200;
pub const PORT_FEATURE_FLOW_CONTROL_BOTH: c_uint = 0x00000300;
pub const PORT_FEATURE_FLOW_CONTROL_NONE: c_uint = 0x00000400;
// The default for MCP link configuration,
    pub mfw_wol_link_cfg: u32,
// The default for the driver of the second external phy,
    pub /: *mut *mut u32 link_config2; / 0x47C,
// The default for MCP of the second external phy,
    pub /: *mut *mut u32 mfw_wol_link_cfg2; / 0x480,
// EEE power saving mode
    pub /: *mut *mut u32 eee_power_mode; / 0x484,
pub const PORT_FEAT_CFG_EEE_POWER_MODE_MASK: c_uint = 0x000000FF;
pub const PORT_FEAT_CFG_EEE_POWER_MODE_SHIFT: c_int = 0;
pub const PORT_FEAT_CFG_EEE_POWER_MODE_DISABLED: c_uint = 0x00000000;
pub const PORT_FEAT_CFG_EEE_POWER_MODE_BALANCED: c_uint = 0x00000001;
pub const PORT_FEAT_CFG_EEE_POWER_MODE_AGGRESSIVE: c_uint = 0x00000002;
pub const PORT_FEAT_CFG_EEE_POWER_MODE_LOW_LATENCY: c_uint = 0x00000003;
    pub /: *mut *mut u32 Reserved2[16]; / 0x488,
}

//
// Device Information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_dev_info {
    pub /: *mut *mut *mut *mut u32 bc_rev; / 8 bits each: major, minor, build / / 4,
    pub /: *mut *mut shared_hw_cfg shared_hw_config; / 40,
    pub /: *mut *mut *mut port_hw_cfg port_hw_config[PORT_MAX]; / 4002=800,
    pub /: *mut *mut shared_feat_cfg shared_feature_config; / 4,
    pub /: *mut *mut *mut port_feat_cfg port_feature_config[PORT_MAX];/ 1162=232,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extended_dev_info_shared_cfg {
    pub reserved: [u32; 18],
    pub mbi_version: u32,
    pub mbi_date: u32,
}

pub const FUNC_0: c_int = 0;
pub const FUNC_1: c_int = 1;
pub const FUNC_2: c_int = 2;
pub const FUNC_3: c_int = 3;
pub const FUNC_4: c_int = 4;
pub const FUNC_5: c_int = 5;
pub const FUNC_6: c_int = 6;
pub const FUNC_7: c_int = 7;
pub const E1_FUNC_MAX: c_int = 2;
pub const E1H_FUNC_MAX: c_int = 8;

pub const VN_0: c_int = 0;
pub const VN_1: c_int = 1;
pub const VN_2: c_int = 2;
pub const VN_3: c_int = 3;
pub const E1VN_MAX: c_int = 1;
pub const E1HVN_MAX: c_int = 4;

// This value (in milliseconds) determines the frequency of the driver
// issuing the PULSE message code.  The firmware monitors this periodic
// pulse to determine when to switch to an OS-absent mode.
pub const DRV_PULSE_PERIOD_MS: c_int = 250;
// This value (in milliseconds) determines how long the driver should
// wait for an acknowledgement from the firmware before timing out.  Once
// the firmware has timed out, the driver will assume there is no firmware
// running and there won't be any firmware-driver synchronization during a
// driver reset.
pub const FW_ACK_TIME_OUT_MS: c_int = 5000;
pub const FW_ACK_POLL_TIME_MS: c_int = 1;

pub const MFW_TRACE_SIGNATURE: c_uint = 0x54524342;
//
// Driver <-> FW Mailbox
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_port_mb {
    pub link_status: u32,
// Driver should update this field on any link change event

pub const LINK_STATUS_LINK_FLAG_MASK: c_uint = 0x00000001;
pub const LINK_STATUS_LINK_UP: c_uint = 0x00000001;
pub const LINK_STATUS_SPEED_AND_DUPLEX_MASK: c_uint = 0x0000001E;

pub const LINK_STATUS_AUTO_NEGOTIATE_FLAG_MASK: c_uint = 0x00000020;
pub const LINK_STATUS_AUTO_NEGOTIATE_ENABLED: c_uint = 0x00000020;
pub const LINK_STATUS_AUTO_NEGOTIATE_COMPLETE: c_uint = 0x00000040;
pub const LINK_STATUS_PARALLEL_DETECTION_FLAG_MASK: c_uint = 0x00000080;
pub const LINK_STATUS_PARALLEL_DETECTION_USED: c_uint = 0x00000080;
pub const LINK_STATUS_LINK_PARTNER_1000TFD_CAPABLE: c_uint = 0x00000200;
pub const LINK_STATUS_LINK_PARTNER_1000THD_CAPABLE: c_uint = 0x00000400;
pub const LINK_STATUS_LINK_PARTNER_100T4_CAPABLE: c_uint = 0x00000800;
pub const LINK_STATUS_LINK_PARTNER_100TXFD_CAPABLE: c_uint = 0x00001000;
pub const LINK_STATUS_LINK_PARTNER_100TXHD_CAPABLE: c_uint = 0x00002000;
pub const LINK_STATUS_LINK_PARTNER_10TFD_CAPABLE: c_uint = 0x00004000;
pub const LINK_STATUS_LINK_PARTNER_10THD_CAPABLE: c_uint = 0x00008000;
pub const LINK_STATUS_TX_FLOW_CONTROL_FLAG_MASK: c_uint = 0x00010000;
pub const LINK_STATUS_TX_FLOW_CONTROL_ENABLED: c_uint = 0x00010000;
pub const LINK_STATUS_RX_FLOW_CONTROL_FLAG_MASK: c_uint = 0x00020000;
pub const LINK_STATUS_RX_FLOW_CONTROL_ENABLED: c_uint = 0x00020000;
pub const LINK_STATUS_LINK_PARTNER_FLOW_CONTROL_MASK: c_uint = 0x000C0000;

pub const LINK_STATUS_SERDES_LINK: c_uint = 0x00100000;
pub const LINK_STATUS_LINK_PARTNER_2500XFD_CAPABLE: c_uint = 0x00200000;
pub const LINK_STATUS_LINK_PARTNER_2500XHD_CAPABLE: c_uint = 0x00400000;
pub const LINK_STATUS_LINK_PARTNER_10GXFD_CAPABLE: c_uint = 0x00800000;
pub const LINK_STATUS_LINK_PARTNER_20GXFD_CAPABLE: c_uint = 0x10000000;
pub const LINK_STATUS_PFC_ENABLED: c_uint = 0x20000000;
pub const LINK_STATUS_PHYSICAL_LINK_FLAG: c_uint = 0x40000000;
pub const LINK_STATUS_SFP_TX_FAULT: c_uint = 0x80000000;
    pub port_stx: u32,
    pub stat_nig_timer: u32,
// MCP firmware does not use this field
    pub ext_phy_fw_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_func_mb {
    pub drv_mb_header: u32,
pub const DRV_MSG_CODE_MASK: c_uint = 0xffff0000;
pub const DRV_MSG_CODE_LOAD_REQ: c_uint = 0x10000000;
pub const DRV_MSG_CODE_LOAD_DONE: c_uint = 0x11000000;
pub const DRV_MSG_CODE_UNLOAD_REQ_WOL_EN: c_uint = 0x20000000;
pub const DRV_MSG_CODE_UNLOAD_REQ_WOL_DIS: c_uint = 0x20010000;
pub const DRV_MSG_CODE_UNLOAD_REQ_WOL_MCP: c_uint = 0x20020000;
pub const DRV_MSG_CODE_UNLOAD_DONE: c_uint = 0x21000000;
pub const DRV_MSG_CODE_DCC_OK: c_uint = 0x30000000;
pub const DRV_MSG_CODE_DCC_FAILURE: c_uint = 0x31000000;
pub const DRV_MSG_CODE_DIAG_ENTER_REQ: c_uint = 0x50000000;
pub const DRV_MSG_CODE_DIAG_EXIT_REQ: c_uint = 0x60000000;
pub const DRV_MSG_CODE_VALIDATE_KEY: c_uint = 0x70000000;
pub const DRV_MSG_CODE_GET_CURR_KEY: c_uint = 0x80000000;
pub const DRV_MSG_CODE_GET_UPGRADE_KEY: c_uint = 0x81000000;
pub const DRV_MSG_CODE_GET_MANUF_KEY: c_uint = 0x82000000;
pub const DRV_MSG_CODE_LOAD_L2B_PRAM: c_uint = 0x90000000;
pub const DRV_MSG_CODE_OEM_OK: c_uint = 0x00010000;
pub const DRV_MSG_CODE_OEM_FAILURE: c_uint = 0x00020000;
pub const DRV_MSG_CODE_OEM_UPDATE_SVID_OK: c_uint = 0x00030000;
pub const DRV_MSG_CODE_OEM_UPDATE_SVID_FAILURE: c_uint = 0x00040000;
//
// The optic module verification command requires bootcode
// v5.0.6 or later, te specific optic module verification command
// requires bootcode v5.2.12 or later
//
pub const DRV_MSG_CODE_VRFY_FIRST_PHY_OPT_MDL: c_uint = 0xa0000000;
pub const REQ_BC_VER_4_VRFY_FIRST_PHY_OPT_MDL: c_uint = 0x00050006;
pub const DRV_MSG_CODE_VRFY_SPECIFIC_PHY_OPT_MDL: c_uint = 0xa1000000;
pub const REQ_BC_VER_4_VRFY_SPECIFIC_PHY_OPT_MDL: c_uint = 0x00050234;
pub const DRV_MSG_CODE_VRFY_AFEX_SUPPORTED: c_uint = 0xa2000000;
pub const REQ_BC_VER_4_VRFY_AFEX_SUPPORTED: c_uint = 0x00070002;
pub const REQ_BC_VER_4_SFP_TX_DISABLE_SUPPORTED: c_uint = 0x00070014;
pub const REQ_BC_VER_4_MT_SUPPORTED: c_uint = 0x00070201;
pub const REQ_BC_VER_4_PFC_STATS_SUPPORTED: c_uint = 0x00070201;
pub const REQ_BC_VER_4_FCOE_FEATURES: c_uint = 0x00070209;
pub const DRV_MSG_CODE_DCBX_ADMIN_PMF_MSG: c_uint = 0xb0000000;
pub const DRV_MSG_CODE_DCBX_PMF_DRV_OK: c_uint = 0xb2000000;
pub const REQ_BC_VER_4_DCBX_ADMIN_MSG_NON_PMF: c_uint = 0x00070401;
pub const DRV_MSG_CODE_VF_DISABLED_DONE: c_uint = 0xc0000000;
pub const DRV_MSG_CODE_AFEX_DRIVER_SETMAC: c_uint = 0xd0000000;
pub const DRV_MSG_CODE_AFEX_LISTGET_ACK: c_uint = 0xd1000000;
pub const DRV_MSG_CODE_AFEX_LISTSET_ACK: c_uint = 0xd2000000;
pub const DRV_MSG_CODE_AFEX_STATSGET_ACK: c_uint = 0xd3000000;
pub const DRV_MSG_CODE_AFEX_VIFSET_ACK: c_uint = 0xd4000000;
pub const DRV_MSG_CODE_DRV_INFO_ACK: c_uint = 0xd8000000;
pub const DRV_MSG_CODE_DRV_INFO_NACK: c_uint = 0xd9000000;
pub const DRV_MSG_CODE_EEE_RESULTS_ACK: c_uint = 0xda000000;
pub const DRV_MSG_CODE_RMMOD: c_uint = 0xdb000000;
pub const REQ_BC_VER_4_RMMOD_CMD: c_uint = 0x0007080f;
pub const DRV_MSG_CODE_SET_MF_BW: c_uint = 0xe0000000;
pub const REQ_BC_VER_4_SET_MF_BW: c_uint = 0x00060202;
pub const DRV_MSG_CODE_SET_MF_BW_ACK: c_uint = 0xe1000000;
pub const DRV_MSG_CODE_LINK_STATUS_CHANGED: c_uint = 0x01000000;
pub const DRV_MSG_CODE_INITIATE_FLR: c_uint = 0x02000000;
pub const REQ_BC_VER_4_INITIATE_FLR: c_uint = 0x00070213;
pub const BIOS_MSG_CODE_LIC_CHALLENGE: c_uint = 0xff010000;
pub const BIOS_MSG_CODE_LIC_RESPONSE: c_uint = 0xff020000;
pub const BIOS_MSG_CODE_VIRT_MAC_PRIM: c_uint = 0xff030000;
pub const BIOS_MSG_CODE_VIRT_MAC_ISCSI: c_uint = 0xff040000;
pub const DRV_MSG_SEQ_NUMBER_MASK: c_uint = 0x0000ffff;
    pub drv_mb_param: u32,
pub const DRV_MSG_CODE_SET_MF_BW_MIN_MASK: c_uint = 0x00ff0000;
pub const DRV_MSG_CODE_SET_MF_BW_MAX_MASK: c_uint = 0xff000000;
pub const DRV_MSG_CODE_UNLOAD_SKIP_LINK_RESET: c_uint = 0x00000002;
pub const DRV_MSG_CODE_LOAD_REQ_WITH_LFA: c_uint = 0x0000100a;
pub const DRV_MSG_CODE_LOAD_REQ_FORCE_LFA: c_uint = 0x00002000;
    pub fw_mb_header: u32,
pub const FW_MSG_CODE_MASK: c_uint = 0xffff0000;
pub const FW_MSG_CODE_DRV_LOAD_COMMON: c_uint = 0x10100000;
pub const FW_MSG_CODE_DRV_LOAD_PORT: c_uint = 0x10110000;
pub const FW_MSG_CODE_DRV_LOAD_FUNCTION: c_uint = 0x10120000;
// Load common chip is supported from bc 6.0.0
pub const REQ_BC_VER_4_DRV_LOAD_COMMON_CHIP: c_uint = 0x00060000;
pub const FW_MSG_CODE_DRV_LOAD_COMMON_CHIP: c_uint = 0x10130000;
pub const FW_MSG_CODE_DRV_LOAD_REFUSED: c_uint = 0x10200000;
pub const FW_MSG_CODE_DRV_LOAD_DONE: c_uint = 0x11100000;
pub const FW_MSG_CODE_DRV_UNLOAD_COMMON: c_uint = 0x20100000;
pub const FW_MSG_CODE_DRV_UNLOAD_PORT: c_uint = 0x20110000;
pub const FW_MSG_CODE_DRV_UNLOAD_FUNCTION: c_uint = 0x20120000;
pub const FW_MSG_CODE_DRV_UNLOAD_DONE: c_uint = 0x21100000;
pub const FW_MSG_CODE_DCC_DONE: c_uint = 0x30100000;
pub const FW_MSG_CODE_LLDP_DONE: c_uint = 0x40100000;
pub const FW_MSG_CODE_DIAG_ENTER_DONE: c_uint = 0x50100000;
pub const FW_MSG_CODE_DIAG_REFUSE: c_uint = 0x50200000;
pub const FW_MSG_CODE_DIAG_EXIT_DONE: c_uint = 0x60100000;
pub const FW_MSG_CODE_VALIDATE_KEY_SUCCESS: c_uint = 0x70100000;
pub const FW_MSG_CODE_VALIDATE_KEY_FAILURE: c_uint = 0x70200000;
pub const FW_MSG_CODE_GET_KEY_DONE: c_uint = 0x80100000;
pub const FW_MSG_CODE_NO_KEY: c_uint = 0x80f00000;
pub const FW_MSG_CODE_LIC_INFO_NOT_READY: c_uint = 0x80f80000;
pub const FW_MSG_CODE_L2B_PRAM_LOADED: c_uint = 0x90100000;
pub const FW_MSG_CODE_L2B_PRAM_T_LOAD_FAILURE: c_uint = 0x90210000;
pub const FW_MSG_CODE_L2B_PRAM_C_LOAD_FAILURE: c_uint = 0x90220000;
pub const FW_MSG_CODE_L2B_PRAM_X_LOAD_FAILURE: c_uint = 0x90230000;
pub const FW_MSG_CODE_L2B_PRAM_U_LOAD_FAILURE: c_uint = 0x90240000;
pub const FW_MSG_CODE_VRFY_OPT_MDL_SUCCESS: c_uint = 0xa0100000;
pub const FW_MSG_CODE_VRFY_OPT_MDL_INVLD_IMG: c_uint = 0xa0200000;
pub const FW_MSG_CODE_VRFY_OPT_MDL_UNAPPROVED: c_uint = 0xa0300000;
pub const FW_MSG_CODE_VF_DISABLED_DONE: c_uint = 0xb0000000;
pub const FW_MSG_CODE_HW_SET_INVALID_IMAGE: c_uint = 0xb0100000;
pub const FW_MSG_CODE_AFEX_DRIVER_SETMAC_DONE: c_uint = 0xd0100000;
pub const FW_MSG_CODE_AFEX_LISTGET_ACK: c_uint = 0xd1100000;
pub const FW_MSG_CODE_AFEX_LISTSET_ACK: c_uint = 0xd2100000;
pub const FW_MSG_CODE_AFEX_STATSGET_ACK: c_uint = 0xd3100000;
pub const FW_MSG_CODE_AFEX_VIFSET_ACK: c_uint = 0xd4100000;
pub const FW_MSG_CODE_DRV_INFO_ACK: c_uint = 0xd8100000;
pub const FW_MSG_CODE_DRV_INFO_NACK: c_uint = 0xd9100000;
pub const FW_MSG_CODE_EEE_RESULS_ACK: c_uint = 0xda100000;
pub const FW_MSG_CODE_RMMOD_ACK: c_uint = 0xdb100000;
pub const FW_MSG_CODE_SET_MF_BW_SENT: c_uint = 0xe0000000;
pub const FW_MSG_CODE_SET_MF_BW_DONE: c_uint = 0xe1000000;
pub const FW_MSG_CODE_LINK_CHANGED_ACK: c_uint = 0x01100000;
pub const FW_MSG_CODE_LIC_CHALLENGE: c_uint = 0xff010000;
pub const FW_MSG_CODE_LIC_RESPONSE: c_uint = 0xff020000;
pub const FW_MSG_CODE_VIRT_MAC_PRIM: c_uint = 0xff030000;
pub const FW_MSG_CODE_VIRT_MAC_ISCSI: c_uint = 0xff040000;
pub const FW_MSG_SEQ_NUMBER_MASK: c_uint = 0x0000ffff;
    pub fw_mb_param: u32,
    pub drv_pulse_mb: u32,
pub const DRV_PULSE_SEQ_MASK: c_uint = 0x00007fff;
pub const DRV_PULSE_SYSTEM_TIME_MASK: c_uint = 0xffff0000;
//
// The system time is in the format of
// (year-2001)*12*32 + month*32 + day.
//
pub const DRV_PULSE_ALWAYS_ALIVE: c_uint = 0x00008000;
//
// Indicate to the firmware not to go into the
// OS-absent when it is not getting driver pulse.
// This is used for debugging as well for PXE(MBA).
//
    pub mcp_pulse_mb: u32,
pub const MCP_PULSE_SEQ_MASK: c_uint = 0x00007fff;
pub const MCP_PULSE_ALWAYS_ALIVE: c_uint = 0x00008000;
// Indicates to the driver not to assert due to lack
// of MCP response
pub const MCP_EVENT_MASK: c_uint = 0xffff0000;
pub const MCP_EVENT_OTHER_DRIVER_RESET_REQ: c_uint = 0x00010000;
    pub iscsi_boot_signature: u32,
    pub iscsi_boot_block_offset: u32,
    pub drv_status: u32,
pub const DRV_STATUS_PMF: c_uint = 0x00000001;
pub const DRV_STATUS_VF_DISABLED: c_uint = 0x00000002;
pub const DRV_STATUS_SET_MF_BW: c_uint = 0x00000004;
pub const DRV_STATUS_LINK_EVENT: c_uint = 0x00000008;
pub const DRV_STATUS_OEM_EVENT_MASK: c_uint = 0x00000070;
pub const DRV_STATUS_OEM_DISABLE_ENABLE_PF: c_uint = 0x00000010;
pub const DRV_STATUS_OEM_BANDWIDTH_ALLOCATION: c_uint = 0x00000020;
pub const DRV_STATUS_OEM_UPDATE_SVID: c_uint = 0x00000080;
pub const DRV_STATUS_DCC_EVENT_MASK: c_uint = 0x0000ff00;
pub const DRV_STATUS_DCC_DISABLE_ENABLE_PF: c_uint = 0x00000100;
pub const DRV_STATUS_DCC_BANDWIDTH_ALLOCATION: c_uint = 0x00000200;
pub const DRV_STATUS_DCC_CHANGE_MAC_ADDRESS: c_uint = 0x00000400;
pub const DRV_STATUS_DCC_RESERVED1: c_uint = 0x00000800;
pub const DRV_STATUS_DCC_SET_PROTOCOL: c_uint = 0x00001000;
pub const DRV_STATUS_DCC_SET_PRIORITY: c_uint = 0x00002000;
pub const DRV_STATUS_DCBX_EVENT_MASK: c_uint = 0x000f0000;
pub const DRV_STATUS_DCBX_NEGOTIATION_RESULTS: c_uint = 0x00010000;
pub const DRV_STATUS_AFEX_EVENT_MASK: c_uint = 0x03f00000;
pub const DRV_STATUS_AFEX_LISTGET_REQ: c_uint = 0x00100000;
pub const DRV_STATUS_AFEX_LISTSET_REQ: c_uint = 0x00200000;
pub const DRV_STATUS_AFEX_STATSGET_REQ: c_uint = 0x00400000;
pub const DRV_STATUS_AFEX_VIFSET_REQ: c_uint = 0x00800000;
pub const DRV_STATUS_DRV_INFO_REQ: c_uint = 0x04000000;
pub const DRV_STATUS_EEE_NEGOTIATION_RESULTS: c_uint = 0x08000000;
    pub virt_mac_upper: u32,
pub const VIRT_MAC_SIGN_MASK: c_uint = 0xffff0000;
pub const VIRT_MAC_SIGNATURE: c_uint = 0x564d0000;
    pub virt_mac_lower: u32,
}

//
// Management firmware state
//
// Allocate 440 bytes for management firmware
pub const MGMTFW_STATE_WORD_SIZE: c_int = 110;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmtfw_state {
    pub opaque: [u32; MGMTFW_STATE_WORD_SIZE],
}

//
// Multi-Function configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_mf_cfg {
    pub clp_mb: u32,
pub const SHARED_MF_CLP_SET_DEFAULT: c_uint = 0x00000000;
// set by CLP
pub const SHARED_MF_CLP_EXIT: c_uint = 0x00000001;
// set by MCP
pub const SHARED_MF_CLP_EXIT_DONE: c_uint = 0x00010000;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_mf_cfg {
    pub /: *mut *mut u32 dynamic_cfg; / device control channel,
pub const PORT_MF_CFG_E1HOV_TAG_MASK: c_uint = 0x0000ffff;
pub const PORT_MF_CFG_E1HOV_TAG_SHIFT: c_int = 0;
    pub reserved: [u32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct func_mf_cfg {
    pub config: u32,
// E/R/I/D
// function 0 of each port cannot be hidden
pub const FUNC_MF_CFG_FUNC_HIDE: c_uint = 0x00000001;
pub const FUNC_MF_CFG_PROTOCOL_MASK: c_uint = 0x00000006;
pub const FUNC_MF_CFG_PROTOCOL_FCOE: c_uint = 0x00000000;
pub const FUNC_MF_CFG_PROTOCOL_ETHERNET: c_uint = 0x00000002;
pub const FUNC_MF_CFG_PROTOCOL_ETHERNET_WITH_RDMA: c_uint = 0x00000004;
pub const FUNC_MF_CFG_PROTOCOL_ISCSI: c_uint = 0x00000006;

pub const FUNC_MF_CFG_FUNC_DISABLED: c_uint = 0x00000008;
pub const FUNC_MF_CFG_FUNC_DELETED: c_uint = 0x00000010;
// PRI
// 0 - low priority, 3 - high priority
pub const FUNC_MF_CFG_TRANSMIT_PRIORITY_MASK: c_uint = 0x00000300;
pub const FUNC_MF_CFG_TRANSMIT_PRIORITY_SHIFT: c_int = 8;
pub const FUNC_MF_CFG_TRANSMIT_PRIORITY_DEFAULT: c_uint = 0x00000000;
// MINBW, MAXBW
// value range - 0..100, increments in 100Mbps
pub const FUNC_MF_CFG_MIN_BW_MASK: c_uint = 0x00ff0000;
pub const FUNC_MF_CFG_MIN_BW_SHIFT: c_int = 16;
pub const FUNC_MF_CFG_MIN_BW_DEFAULT: c_uint = 0x00000000;
pub const FUNC_MF_CFG_MAX_BW_MASK: c_uint = 0xff000000;
pub const FUNC_MF_CFG_MAX_BW_SHIFT: c_int = 24;
pub const FUNC_MF_CFG_MAX_BW_DEFAULT: c_uint = 0x64000000;
    pub /: *mut *mut u32 mac_upper; / MAC,
pub const FUNC_MF_CFG_UPPERMAC_MASK: c_uint = 0x0000ffff;
pub const FUNC_MF_CFG_UPPERMAC_SHIFT: c_int = 0;

    pub mac_lower: u32,
pub const FUNC_MF_CFG_LOWERMAC_DEFAULT: c_uint = 0xffffffff;
    pub /: *mut *mut u32 e1hov_tag; / VNI,
pub const FUNC_MF_CFG_E1HOV_TAG_MASK: c_uint = 0x0000ffff;
pub const FUNC_MF_CFG_E1HOV_TAG_SHIFT: c_int = 0;

// afex default VLAN ID - 12 bits
pub const FUNC_MF_CFG_AFEX_VLAN_MASK: c_uint = 0x0fff0000;
pub const FUNC_MF_CFG_AFEX_VLAN_SHIFT: c_int = 16;
    pub afex_config: u32,
pub const FUNC_MF_CFG_AFEX_COS_FILTER_MASK: c_uint = 0x000000ff;
pub const FUNC_MF_CFG_AFEX_COS_FILTER_SHIFT: c_int = 0;
pub const FUNC_MF_CFG_AFEX_MBA_ENABLED_MASK: c_uint = 0x0000ff00;
pub const FUNC_MF_CFG_AFEX_MBA_ENABLED_SHIFT: c_int = 8;
pub const FUNC_MF_CFG_AFEX_MBA_ENABLED_VAL: c_uint = 0x00000100;
pub const FUNC_MF_CFG_AFEX_VLAN_MODE_MASK: c_uint = 0x000f0000;
pub const FUNC_MF_CFG_AFEX_VLAN_MODE_SHIFT: c_int = 16;
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mf_cfg_afex_vlan_mode {
    FUNC_MF_CFG_AFEX_VLAN_TRUNK_MODE = 0,
    FUNC_MF_CFG_AFEX_VLAN_ACCESS_MODE,
    FUNC_MF_CFG_AFEX_VLAN_TRUNK_TAG_NATIVE_MODE
}

// This structure is not applicable and should not be accessed on 57711
#[repr(C)]
#[derive(Copy, Clone)]
pub struct func_ext_cfg {
    pub func_cfg: u32,
pub const MACP_FUNC_CFG_FLAGS_MASK: c_uint = 0x0000007F;
pub const MACP_FUNC_CFG_FLAGS_SHIFT: c_int = 0;
pub const MACP_FUNC_CFG_FLAGS_ENABLED: c_uint = 0x00000001;
pub const MACP_FUNC_CFG_FLAGS_ETHERNET: c_uint = 0x00000002;
pub const MACP_FUNC_CFG_FLAGS_ISCSI_OFFLOAD: c_uint = 0x00000004;
pub const MACP_FUNC_CFG_FLAGS_FCOE_OFFLOAD: c_uint = 0x00000008;
pub const MACP_FUNC_CFG_PAUSE_ON_HOST_RING: c_uint = 0x00000080;
    pub iscsi_mac_addr_upper: u32,
    pub iscsi_mac_addr_lower: u32,
    pub fcoe_mac_addr_upper: u32,
    pub fcoe_mac_addr_lower: u32,
    pub fcoe_wwn_port_name_upper: u32,
    pub fcoe_wwn_port_name_lower: u32,
    pub fcoe_wwn_node_name_upper: u32,
    pub fcoe_wwn_node_name_lower: u32,
    pub preserve_data: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mf_cfg {
    pub /: *mut *mut shared_mf_cfg shared_mf_config; / 0x4,
// 0x8*2*2=0x20
    pub port_mf_config: [port_mf_cfg; NVM_PATH_MAX][PORT_MAX],
// for all chips, there are 8 mf functions
    pub /: *mut *mut *mut func_mf_cfg func_mf_config[E1H_FUNC_MAX]; / 0x18  8 = 0xc0,
//
// Extended configuration per function  - this array does not exist and
// should not be accessed on 57711
//
    pub 0x140*/: *mut *mut *mut func_ext_cfg func_ext_config[E1H_FUNC_MAX]; / 0x28  8 =,
}

//
// Shared Memory Region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_region {
    pub /: *mut *mut *mut u32 validity_map[PORT_MAX]; / 0x0 (42 = 0x8),
pub const SHR_MEM_FORMAT_REV_MASK: c_uint = 0xff000000;

// validity bits
pub const SHR_MEM_VALIDITY_PCI_CFG: c_uint = 0x00100000;
pub const SHR_MEM_VALIDITY_MB: c_uint = 0x00200000;
pub const SHR_MEM_VALIDITY_DEV_INFO: c_uint = 0x00400000;
pub const SHR_MEM_VALIDITY_RESERVED: c_uint = 0x00000007;
// One licensing bit should be set
pub const SHR_MEM_VALIDITY_LIC_KEY_IN_EFFECT_MASK: c_uint = 0x00000038;
pub const SHR_MEM_VALIDITY_LIC_MANUF_KEY_IN_EFFECT: c_uint = 0x00000008;
pub const SHR_MEM_VALIDITY_LIC_UPGRADE_KEY_IN_EFFECT: c_uint = 0x00000010;
pub const SHR_MEM_VALIDITY_LIC_NO_KEY_IN_EFFECT: c_uint = 0x00000020;
// Active MFW
pub const SHR_MEM_VALIDITY_ACTIVE_MFW_UNKNOWN: c_uint = 0x00000000;
pub const SHR_MEM_VALIDITY_ACTIVE_MFW_MASK: c_uint = 0x000001c0;
pub const SHR_MEM_VALIDITY_ACTIVE_MFW_IPMI: c_uint = 0x00000040;
pub const SHR_MEM_VALIDITY_ACTIVE_MFW_UMP: c_uint = 0x00000080;
pub const SHR_MEM_VALIDITY_ACTIVE_MFW_NCSI: c_uint = 0x000000c0;
pub const SHR_MEM_VALIDITY_ACTIVE_MFW_NONE: c_uint = 0x000001c0;
    pub /: *mut *mut shm_dev_info dev_info; / 0x8 (0x438),
    pub /: *mut *mut *mut license_key drv_lic_key[PORT_MAX]; / 0x440 (522=0x68),
// FW information (for internal FW use)
    pub /: *mut *mut u32 fw_info_fio_offset; / 0x4a8 (0x4),
    pub /: *mut *mut mgmtfw_state mgmtfw_state; / 0x4ac (0x1b8),
    pub /: *mut *mut *mut drv_port_mb port_mb[PORT_MAX]; / 0x664 (162=0x20),

// This is a variable length array
// the number of function depends on the chip type
    pub /: *mut *mut *mut drv_func_mb func_mb[1]; / 0x684 (442/4/8=0x58/0xb0/0x160),

// the number of function depends on the chip type
    pub /: *mut *mut *mut drv_func_mb func_mb[]; / 0x684 (442/4/8=0x58/0xb0/0x160),

}

//
// Shared Memory 2 Region
//
// The fw_flr_ack is actually built in the following way:
// 8 bit:  PF ack
// 64 bit: VF ack
// 8 bit:  ios_dis_ack
// In order to maintain endianity in the mailbox hsi, we want to keep using
// u32. The fw must have the VF right after the PF since this is how it
// access arrays(it expects always the VF to reside after the PF, and that
// makes the calculation much easier for it. )
// In order to answer both limitations, and keep the struct small, the code
// will abuse the structure defined here to achieve the actual partition
// above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flr_ack {
    pub pf_ack: u32,
    pub vf_ack: [u32; 1],
    pub iov_dis_ack: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flr_mb {
    pub aggint: u32,
    pub opgen_addr: u32,
    pub ack: fw_flr_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eee_remote_vals {
    pub tx_tw: u32,
    pub rx_tw: u32,
}

// SUPPORT FOR SHMEM ARRRAYS
// The SHMEM HSI is aligned on 32 bit boundaries which makes it difficult to
// define arrays with storage types smaller then unsigned dwords.
// The macros below add generic support for SHMEM arrays with numeric elements
// that can span 2,4,8 or 16 bits. The array underlying type is a 32 bit dword
// array with individual bit-filed elements accessed using shifts and masks.
//
// eb is the bitwidth of a single element

// the bit-position macro allows the used to flip the order of the arrays
// elements on a per byte or word boundary.
//
// example: an array with 8 entries each 4 bit wide. This array will fit into
// a single dword. The diagrmas below show the array order of the nibbles.
//
// SHMEM_ARRAY_BITPOS(i, 4, 4) defines the stadard ordering:
//
// |                |                |               |
// 0    |   1   |   2    |   3   |   4    |   5   |   6   |   7   |
// |                |                |               |
//
// SHMEM_ARRAY_BITPOS(i, 4, 8) defines a flip ordering per byte:
//
// |                |                |               |
// 1   |   0    |   3    |   2   |   5    |   4   |   7   |   6   |
// |                |                |               |
//
// SHMEM_ARRAY_BITPOS(i, 4, 16) defines a flip ordering per word:
//
// |                |                |               |
// 3   |   2    |   1   |   0    |   7   |   6    |   5   |   4   |
// |                |                |               |
//

// START OF DCBX STRUCTURES DECLARATIONS
pub const DCBX_MAX_NUM_PRI_PG_ENTRIES: c_int = 8;
pub const DCBX_PRI_PG_BITWIDTH: c_int = 4;
pub const DCBX_PRI_PG_FBITS: c_int = 8;

pub const DCBX_MAX_NUM_PG_BW_ENTRIES: c_int = 8;
pub const DCBX_BW_PG_BITWIDTH: c_int = 8;

pub const DCBX_STRICT_PRI_PG: c_int = 15;
pub const DCBX_MAX_APP_PROTOCOL: c_int = 16;
pub const FCOE_APP_IDX: c_int = 0;
pub const ISCSI_APP_IDX: c_int = 1;
pub const PREDEFINED_APP_IDX_MAX: c_int = 2;
// Big/Little endian have the same representation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_ets_feature {
//
// For Admin MIB - is this feature supported by the
// driver | For Local MIB - should this feature be enabled.
//
    pub enabled: u32,
    pub pg_bw_tbl: [u32; 2],
    pub pri_pg_tbl: [u32; 1],
}

// Driver structure in LE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_pfc_feature {

    pub pri_en_bitmap: u8,
pub const DCBX_PFC_PRI_0: c_uint = 0x01;
pub const DCBX_PFC_PRI_1: c_uint = 0x02;
pub const DCBX_PFC_PRI_2: c_uint = 0x04;
pub const DCBX_PFC_PRI_3: c_uint = 0x08;
pub const DCBX_PFC_PRI_4: c_uint = 0x10;
pub const DCBX_PFC_PRI_5: c_uint = 0x20;
pub const DCBX_PFC_PRI_6: c_uint = 0x40;
pub const DCBX_PFC_PRI_7: c_uint = 0x80;
    pub pfc_caps: u8,
    pub reserved: u8,
    pub enabled: u8,

    pub enabled: u8,
    pub reserved: u8,
    pub pfc_caps: u8,
    pub pri_en_bitmap: u8,
pub const DCBX_PFC_PRI_0: c_uint = 0x01;
pub const DCBX_PFC_PRI_1: c_uint = 0x02;
pub const DCBX_PFC_PRI_2: c_uint = 0x04;
pub const DCBX_PFC_PRI_3: c_uint = 0x08;
pub const DCBX_PFC_PRI_4: c_uint = 0x10;
pub const DCBX_PFC_PRI_5: c_uint = 0x20;
pub const DCBX_PFC_PRI_6: c_uint = 0x40;
pub const DCBX_PFC_PRI_7: c_uint = 0x80;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_app_priority_entry {

    pub app_id: u16,
    pub pri_bitmap: u8,
    pub appBitfield: u8,
pub const DCBX_APP_ENTRY_VALID: c_uint = 0x01;
pub const DCBX_APP_ENTRY_SF_MASK: c_uint = 0xF0;
pub const DCBX_APP_ENTRY_SF_SHIFT: c_int = 4;
pub const DCBX_APP_SF_ETH_TYPE: c_uint = 0x10;
pub const DCBX_APP_SF_PORT: c_uint = 0x20;
pub const DCBX_APP_SF_UDP: c_uint = 0x40;
pub const DCBX_APP_SF_DEFAULT: c_uint = 0x80;

    pub appBitfield: u8,
pub const DCBX_APP_ENTRY_VALID: c_uint = 0x01;
pub const DCBX_APP_ENTRY_SF_MASK: c_uint = 0xF0;
pub const DCBX_APP_ENTRY_SF_SHIFT: c_int = 4;
pub const DCBX_APP_ENTRY_VALID: c_uint = 0x01;
pub const DCBX_APP_SF_ETH_TYPE: c_uint = 0x10;
pub const DCBX_APP_SF_PORT: c_uint = 0x20;
pub const DCBX_APP_SF_UDP: c_uint = 0x40;
pub const DCBX_APP_SF_DEFAULT: c_uint = 0x80;
    pub pri_bitmap: u8,
    pub app_id: u16,

}

// FW structure in BE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_app_priority_feature {

    pub reserved: u8,
    pub default_pri: u8,
    pub tc_supported: u8,
    pub enabled: u8,

    pub enabled: u8,
    pub tc_supported: u8,
    pub default_pri: u8,
    pub reserved: u8,
    pub app_pri_tbl: [dcbx_app_priority_entry; DCBX_MAX_APP_PROTOCOL],
}

// FW structure in BE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_features {
// PG feature
    pub ets: dcbx_ets_feature,
// PFC feature
    pub pfc: dcbx_pfc_feature,
// APP feature
    pub app: dcbx_app_priority_feature,
}

// LLDP protocol parameters
// FW structure in BE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_params {

    pub msg_fast_tx_interval: u8,
    pub msg_tx_hold: u8,
    pub msg_tx_interval: u8,
    pub admin_status: u8,
pub const LLDP_TX_ONLY: c_uint = 0x01;
pub const LLDP_RX_ONLY: c_uint = 0x02;
pub const LLDP_TX_RX: c_uint = 0x03;
pub const LLDP_DISABLED: c_uint = 0x04;
    pub reserved1: u8,
    pub tx_fast: u8,
    pub tx_crd_max: u8,
    pub tx_crd: u8,

    pub admin_status: u8,
pub const LLDP_TX_ONLY: c_uint = 0x01;
pub const LLDP_RX_ONLY: c_uint = 0x02;
pub const LLDP_TX_RX: c_uint = 0x03;
pub const LLDP_DISABLED: c_uint = 0x04;
    pub msg_tx_interval: u8,
    pub msg_tx_hold: u8,
    pub msg_fast_tx_interval: u8,
    pub tx_crd: u8,
    pub tx_crd_max: u8,
    pub tx_fast: u8,
    pub reserved1: u8,

pub const REM_CHASSIS_ID_STAT_LEN: c_int = 4;
pub const REM_PORT_ID_STAT_LEN: c_int = 4;
// Holds remote Chassis ID TLV header, subtype and 9B of payload.
    pub peer_chassis_id: [u32; REM_CHASSIS_ID_STAT_LEN],
// Holds remote Port ID TLV header, subtype and 9B of payload.
    pub peer_port_id: [u32; REM_PORT_ID_STAT_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_dcbx_stat {
pub const LOCAL_CHASSIS_ID_STAT_LEN: c_int = 2;
pub const LOCAL_PORT_ID_STAT_LEN: c_int = 2;
// Holds local Chassis ID 8B payload of constant subtype 4.
    pub local_chassis_id: [u32; LOCAL_CHASSIS_ID_STAT_LEN],
// Holds local Port ID 8B payload of constant subtype 3.
    pub local_port_id: [u32; LOCAL_PORT_ID_STAT_LEN],
// Number of DCBX frames transmitted.
    pub num_tx_dcbx_pkts: u32,
// Number of DCBX frames received.
    pub num_rx_dcbx_pkts: u32,
}

// ADMIN MIB - DCBX local machine default configuration.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_admin_mib {
    pub ver_cfg_flags: u32,
pub const DCBX_ETS_CONFIG_TX_ENABLED: c_uint = 0x00000001;
pub const DCBX_PFC_CONFIG_TX_ENABLED: c_uint = 0x00000002;
pub const DCBX_APP_CONFIG_TX_ENABLED: c_uint = 0x00000004;
pub const DCBX_ETS_RECO_TX_ENABLED: c_uint = 0x00000008;
pub const DCBX_ETS_RECO_VALID: c_uint = 0x00000010;
pub const DCBX_ETS_WILLING: c_uint = 0x00000020;
pub const DCBX_PFC_WILLING: c_uint = 0x00000040;
pub const DCBX_APP_WILLING: c_uint = 0x00000080;
pub const DCBX_VERSION_CEE: c_uint = 0x00000100;
pub const DCBX_VERSION_IEEE: c_uint = 0x00000200;
pub const DCBX_DCBX_ENABLED: c_uint = 0x00000400;
pub const DCBX_CEE_VERSION_MASK: c_uint = 0x0000f000;
pub const DCBX_CEE_VERSION_SHIFT: c_int = 12;
pub const DCBX_CEE_MAX_VERSION_MASK: c_uint = 0x000f0000;
pub const DCBX_CEE_MAX_VERSION_SHIFT: c_int = 16;
    pub features: dcbx_features,
}

// REMOTE MIB - remote machine DCBX configuration.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_remote_mib {
    pub prefix_seq_num: u32,
    pub flags: u32,
pub const DCBX_ETS_TLV_RX: c_uint = 0x00000001;
pub const DCBX_PFC_TLV_RX: c_uint = 0x00000002;
pub const DCBX_APP_TLV_RX: c_uint = 0x00000004;
pub const DCBX_ETS_RX_ERROR: c_uint = 0x00000010;
pub const DCBX_PFC_RX_ERROR: c_uint = 0x00000020;
pub const DCBX_APP_RX_ERROR: c_uint = 0x00000040;
pub const DCBX_ETS_REM_WILLING: c_uint = 0x00000100;
pub const DCBX_PFC_REM_WILLING: c_uint = 0x00000200;
pub const DCBX_APP_REM_WILLING: c_uint = 0x00000400;
pub const DCBX_REMOTE_ETS_RECO_VALID: c_uint = 0x00001000;
pub const DCBX_REMOTE_MIB_VALID: c_uint = 0x00002000;
    pub features: dcbx_features,
    pub suffix_seq_num: u32,
}

// LOCAL MIB - operational DCBX configuration - transmitted on Tx LLDPDU.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_local_mib {
    pub prefix_seq_num: u32,
// Indicates if there is mismatch with negotiation results.
    pub error: u32,
pub const DCBX_LOCAL_ETS_ERROR: c_uint = 0x00000001;
pub const DCBX_LOCAL_PFC_ERROR: c_uint = 0x00000002;
pub const DCBX_LOCAL_APP_ERROR: c_uint = 0x00000004;
pub const DCBX_LOCAL_PFC_MISMATCH: c_uint = 0x00000010;
pub const DCBX_LOCAL_APP_MISMATCH: c_uint = 0x00000020;
pub const DCBX_REMOTE_MIB_ERROR: c_uint = 0x00000040;
pub const DCBX_REMOTE_ETS_TLV_NOT_FOUND: c_uint = 0x00000080;
pub const DCBX_REMOTE_PFC_TLV_NOT_FOUND: c_uint = 0x00000100;
pub const DCBX_REMOTE_APP_TLV_NOT_FOUND: c_uint = 0x00000200;
    pub features: dcbx_features,
    pub suffix_seq_num: u32,
}

// END OF DCBX STRUCTURES DECLARATIONS
//
// Elink section
//
pub const SHMEM_LINK_CONFIG_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_lfa {
    pub req_duplex: u32,
pub const REQ_DUPLEX_PHY0_MASK: c_uint = 0x0000ffff;
pub const REQ_DUPLEX_PHY0_SHIFT: c_int = 0;
pub const REQ_DUPLEX_PHY1_MASK: c_uint = 0xffff0000;
pub const REQ_DUPLEX_PHY1_SHIFT: c_int = 16;
    pub req_flow_ctrl: u32,
pub const REQ_FLOW_CTRL_PHY0_MASK: c_uint = 0x0000ffff;
pub const REQ_FLOW_CTRL_PHY0_SHIFT: c_int = 0;
pub const REQ_FLOW_CTRL_PHY1_MASK: c_uint = 0xffff0000;
pub const REQ_FLOW_CTRL_PHY1_SHIFT: c_int = 16;
    pub /: *mut *mut u32 req_line_speed; / Also determine AutoNeg,
pub const REQ_LINE_SPD_PHY0_MASK: c_uint = 0x0000ffff;
pub const REQ_LINE_SPD_PHY0_SHIFT: c_int = 0;
pub const REQ_LINE_SPD_PHY1_MASK: c_uint = 0xffff0000;
pub const REQ_LINE_SPD_PHY1_SHIFT: c_int = 16;
    pub speed_cap_mask: [u32; SHMEM_LINK_CONFIG_SIZE],
    pub additional_config: u32,
pub const REQ_FC_AUTO_ADV_MASK: c_uint = 0x0000ffff;
pub const REQ_FC_AUTO_ADV0_SHIFT: c_int = 0;
pub const NO_LFA_DUE_TO_DCC_MASK: c_uint = 0x00010000;
    pub lfa_sts: u32,
pub const LFA_LINK_FLAP_REASON_OFFSET: c_int = 0;
pub const LFA_LINK_FLAP_REASON_MASK: c_uint = 0x000000ff;
pub const LFA_LINK_DOWN: c_uint = 0x1;
pub const LFA_LOOPBACK_ENABLED: c_uint = 0x2;
pub const LFA_DUPLEX_MISMATCH: c_uint = 0x3;
pub const LFA_MFW_IS_TOO_OLD: c_uint = 0x4;
pub const LFA_LINK_SPEED_MISMATCH: c_uint = 0x5;
pub const LFA_FLOW_CTRL_MISMATCH: c_uint = 0x6;
pub const LFA_SPEED_CAP_MISMATCH: c_uint = 0x7;
pub const LFA_DCC_LFA_DISABLED: c_uint = 0x8;
pub const LFA_EEE_MISMATCH: c_uint = 0x9;
pub const LINK_FLAP_AVOIDANCE_COUNT_OFFSET: c_int = 8;
pub const LINK_FLAP_AVOIDANCE_COUNT_MASK: c_uint = 0x0000ff00;
pub const LINK_FLAP_COUNT_OFFSET: c_int = 16;
pub const LINK_FLAP_COUNT_MASK: c_uint = 0x00ff0000;
pub const LFA_FLAGS_MASK: c_uint = 0xff000000;

}

// Used to support NSCI get OS driver version
// on driver load the version value will be set
// on driver unload driver value of 0x0 will be set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct os_drv_ver {
pub const DRV_VER_NOT_LOADED: c_int = 0;
// personalties order is important
pub const DRV_PERS_ETHERNET: c_int = 0;
pub const DRV_PERS_ISCSI: c_int = 1;
pub const DRV_PERS_FCOE: c_int = 2;
// shmem2 struct is constant can't add more personalties here
pub const MAX_DRV_PERS: c_int = 3;
    pub versions: [u32; MAX_DRV_PERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_oem_fcoe_features {
    pub fcoe_features1: u32,
pub const FCOE_FEATURES1_IOS_PER_CONNECTION_MASK: c_uint = 0x0000FFFF;
pub const FCOE_FEATURES1_IOS_PER_CONNECTION_OFFSET: c_int = 0;
pub const FCOE_FEATURES1_LOGINS_PER_PORT_MASK: c_uint = 0xFFFF0000;
pub const FCOE_FEATURES1_LOGINS_PER_PORT_OFFSET: c_int = 16;
    pub fcoe_features2: u32,
pub const FCOE_FEATURES2_EXCHANGES_MASK: c_uint = 0x0000FFFF;
pub const FCOE_FEATURES2_EXCHANGES_OFFSET: c_int = 0;
pub const FCOE_FEATURES2_NPIV_WWN_PER_PORT_MASK: c_uint = 0xFFFF0000;
pub const FCOE_FEATURES2_NPIV_WWN_PER_PORT_OFFSET: c_int = 16;
    pub fcoe_features3: u32,
pub const FCOE_FEATURES3_TARGETS_SUPPORTED_MASK: c_uint = 0x0000FFFF;
pub const FCOE_FEATURES3_TARGETS_SUPPORTED_OFFSET: c_int = 0;
pub const FCOE_FEATURES3_OUTSTANDING_COMMANDS_MASK: c_uint = 0xFFFF0000;
pub const FCOE_FEATURES3_OUTSTANDING_COMMANDS_OFFSET: c_int = 16;
    pub fcoe_features4: u32,
pub const FCOE_FEATURES4_FEATURE_SETTINGS_MASK: c_uint = 0x0000000F;
pub const FCOE_FEATURES4_FEATURE_SETTINGS_OFFSET: c_int = 0;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum curr_cfg_method_e {
    CURR_CFG_MET_NONE = 0,  /* default config */
    CURR_CFG_MET_OS = 1,
    CURR_CFG_MET_VENDOR_SPEC = 2,/* e.g. Option ROM, NPAR, O/S Cfg Utils */
}

pub const FC_NPIV_WWPN_SIZE: c_int = 8;
pub const FC_NPIV_WWNN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdn_npiv_settings {
    pub npiv_wwpn: [u8; FC_NPIV_WWPN_SIZE],
    pub npiv_wwnn: [u8; FC_NPIV_WWNN_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdn_fc_npiv_cfg {
// hdr used internally by the MFW
    pub hdr: u32,
    pub num_of_npiv: u32,
}

pub const MAX_NUMBER_NPIV: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdn_fc_npiv_tbl {
    pub fc_npiv_cfg: bdn_fc_npiv_cfg,
    pub settings: [bdn_npiv_settings; MAX_NUMBER_NPIV],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdump_driver_info {
    pub epoc: u32,
    pub drv_ver: u32,
    pub fw_ver: u32,
    pub valid_dump: u32,

    pub flags: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_oem_data {
    pub driver_version: [u32; 4],
    pub ncsi_oem_fcoe_features: ncsi_oem_fcoe_features,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem2_region {
    pub /: *mut *mut u32 size; / 0x0000,
    pub /: *mut *mut u32 dcc_support; / 0x0004,
pub const SHMEM_DCC_SUPPORT_NONE: c_uint = 0x00000000;
pub const SHMEM_DCC_SUPPORT_DISABLE_ENABLE_PF_TLV: c_uint = 0x00000001;
pub const SHMEM_DCC_SUPPORT_BANDWIDTH_ALLOCATION_TLV: c_uint = 0x00000004;
pub const SHMEM_DCC_SUPPORT_CHANGE_MAC_ADDRESS_TLV: c_uint = 0x00000008;
pub const SHMEM_DCC_SUPPORT_SET_PROTOCOL_TLV: c_uint = 0x00000040;
pub const SHMEM_DCC_SUPPORT_SET_PRIORITY_TLV: c_uint = 0x00000080;
    pub /: *mut *mut u32 ext_phy_fw_version2[PORT_MAX]; / 0x0008,
//
// For backwards compatibility, if the mf_cfg_addr does not exist
// (the size filed is smaller than 0xc) the mf_cfg resides at the
// end of struct shmem_region
//
    pub /: *mut *mut u32 mf_cfg_addr; / 0x0010,
pub const SHMEM_MF_CFG_ADDR_NONE: c_uint = 0x00000000;
    pub /: *mut *mut fw_flr_mb flr_mb; / 0x0014,
    pub /: *mut *mut u32 dcbx_lldp_params_offset; / 0x0028,
pub const SHMEM_LLDP_DCBX_PARAMS_NONE: c_uint = 0x00000000;
    pub /: *mut *mut u32 dcbx_neg_res_offset; / 0x002c,
pub const SHMEM_DCBX_NEG_RES_NONE: c_uint = 0x00000000;
    pub /: *mut *mut u32 dcbx_remote_mib_offset; / 0x0030,
pub const SHMEM_DCBX_REMOTE_MIB_NONE: c_uint = 0x00000000;
//
// The other shmemX_base_addr holds the other path's shmem address
// required for example in case of common phy init, or for path1 to know
// the address of mcp debug trace which is located in offset from shmem
// of path0
//
    pub /: *mut *mut u32 other_shmem_base_addr; / 0x0034,
    pub /: *mut *mut u32 other_shmem2_base_addr; / 0x0038,
//
// mcp_vf_disabled is set by the MCP to indicate the driver about VFs
// which were disabled/flred
//
    pub /: *mut *mut u32 mcp_vf_disabled[E2_VF_MAX / 32]; / 0x003c,
//
// drv_ack_vf_disabled is set by the PF driver to ack handled disabled
// VFs
//
    pub /: *mut *mut u32 drv_ack_vf_disabled[E2_FUNC_MAX][E2_VF_MAX / 32]; / 0x0044,
    pub /: *mut *mut u32 dcbx_lldp_dcbx_stat_offset; / 0x0064,
pub const SHMEM_LLDP_DCBX_STAT_NONE: c_uint = 0x00000000;
//
// edebug_driver_if field is used to transfer messages between edebug
// app to the driver through shmem2.
//
// message format:
// bits 0-2 -  function number / instance of driver to perform request
// bits 3-5 -  op code / is_ack?
// bits 6-63 - data
//
    pub /: *mut *mut u32 edebug_driver_if[2]; / 0x0068,
pub const EDEBUG_DRIVER_IF_OP_CODE_GET_PHYS_ADDR: c_int = 1;
pub const EDEBUG_DRIVER_IF_OP_CODE_GET_BUS_ADDR: c_int = 2;
pub const EDEBUG_DRIVER_IF_OP_CODE_DISABLE_STAT: c_int = 3;
    pub /: *mut *mut u32 nvm_retain_bitmap_addr; / 0x0070,
// afex support of that driver
    pub /: *mut *mut u32 afex_driver_support; / 0x0074,
pub const SHMEM_AFEX_VERSION_MASK: c_uint = 0x100f;
pub const SHMEM_AFEX_SUPPORTED_VERSION_ONE: c_uint = 0x1001;
pub const SHMEM_AFEX_REDUCED_DRV_LOADED: c_uint = 0x8000;
// driver receives addr in scratchpad to which it should respond
    pub afex_scratchpad_addr_to_write: [u32; E2_FUNC_MAX],
// generic params from MCP to driver (value depends on the msg sent
// to driver
//
    pub /: *mut *mut u32 afex_param1_to_driver[E2_FUNC_MAX]; / 0x0088,
    pub /: *mut *mut u32 afex_param2_to_driver[E2_FUNC_MAX]; / 0x0098,
    pub /: *mut *mut u32 swim_base_addr; / 0x0108,
    pub swim_funcs: u32,
    pub swim_main_cb: u32,
// bitmap notifying which VIF profiles stored in nvram are enabled by
// switch
//
    pub afex_profiles_enabled: [u32; 2],
// generic flags controlled by the driver
    pub drv_flags: u32,
pub const DRV_FLAGS_DCB_CONFIGURED: c_uint = 0x0;
pub const DRV_FLAGS_DCB_CONFIGURATION_ABORTED: c_uint = 0x1;
pub const DRV_FLAGS_DCB_MFW_CONFIGURED: c_uint = 0x2;

// pointer to extended dev_info shared data copied from nvm image
    pub extended_dev_info_shared_addr: u32,
    pub ncsi_oem_data_addr: u32,
    pub /: *mut *mut u32 ocsd_host_addr; / initialized by option ROM,
    pub /: *mut *mut u32 ocbb_host_addr; / initialized by option ROM,
    pub /: *mut *mut u32 ocsd_req_update_interval; / initialized by option ROM,
    pub temperature_in_half_celsius: u32,
    pub glob_struct_in_host: u32,
    pub dcbx_neg_res_ext_offset: u32,
pub const SHMEM_DCBX_NEG_RES_EXT_NONE: c_uint = 0x00000000;
    pub drv_capabilities_flag: [u32; E2_FUNC_MAX],
pub const DRV_FLAGS_CAPABILITIES_LOADED_SUPPORTED: c_uint = 0x00000001;
pub const DRV_FLAGS_CAPABILITIES_LOADED_L2: c_uint = 0x00000002;
pub const DRV_FLAGS_CAPABILITIES_LOADED_FCOE: c_uint = 0x00000004;
pub const DRV_FLAGS_CAPABILITIES_LOADED_ISCSI: c_uint = 0x00000008;
pub const DRV_FLAGS_MTU_MASK: c_uint = 0xffff0000;
pub const DRV_FLAGS_MTU_SHIFT: c_int = 16;
    pub extended_dev_info_shared_cfg_size: u32,
    pub dcbx_en: [u32; PORT_MAX],
// The offset points to the multi threaded meta structure
    pub multi_thread_data_offset: u32,
// address of DMAable host address holding values from the drivers
    pub drv_info_host_addr_lo: u32,
    pub drv_info_host_addr_hi: u32,
// general values written by the MFW (such as current version)
    pub drv_info_control: u32,
pub const DRV_INFO_CONTROL_VER_MASK: c_uint = 0x000000ff;
pub const DRV_INFO_CONTROL_VER_SHIFT: c_int = 0;
pub const DRV_INFO_CONTROL_OP_CODE_MASK: c_uint = 0x0000ff00;
pub const DRV_INFO_CONTROL_OP_CODE_SHIFT: c_int = 8;
    pub /: *mut *mut u32 ibft_host_addr; / initialized by option ROM,
    pub eee_remote_vals: [eee_remote_vals; PORT_MAX],
    pub reserved: [u32; E2_FUNC_MAX],
// the status of EEE auto-negotiation
// bits 15:0 the configured tx-lpi entry timer value. Depends on bit 31.
// bits 19:16 the supported modes for EEE.
// bits 23:20 the speeds advertised for EEE.
// bits 27:24 the speeds the Link partner advertised for EEE.
// The supported/adv. modes in bits 27:19 originate from the
// SHMEM_EEE_XXX_ADV definitions (where XXX is replaced by speed).
// bit 28 when 1'b1 EEE was requested.
// bit 29 when 1'b1 tx lpi was requested.
// bit 30 when 1'b1 EEE was negotiated. Tx lpi will be asserted iff
// 30:29 are 2'b11.
// bit 31 when 1'b0 bits 15:0 contain a PORT_FEAT_CFG_EEE_ define as
// value. When 1'b1 those bits contains a value times 16 microseconds.
//
    pub eee_status: [u32; PORT_MAX],
pub const SHMEM_EEE_TIMER_MASK: c_uint = 0x0000ffff;
pub const SHMEM_EEE_SUPPORTED_MASK: c_uint = 0x000f0000;
pub const SHMEM_EEE_SUPPORTED_SHIFT: c_int = 16;
pub const SHMEM_EEE_ADV_STATUS_MASK: c_uint = 0x00f00000;

pub const SHMEM_EEE_ADV_STATUS_SHIFT: c_int = 20;
pub const SHMEM_EEE_LP_ADV_STATUS_MASK: c_uint = 0x0f000000;
pub const SHMEM_EEE_LP_ADV_STATUS_SHIFT: c_int = 24;
pub const SHMEM_EEE_REQUESTED_BIT: c_uint = 0x10000000;
pub const SHMEM_EEE_LPI_REQUESTED_BIT: c_uint = 0x20000000;
pub const SHMEM_EEE_ACTIVE_BIT: c_uint = 0x40000000;
pub const SHMEM_EEE_TIME_OUTPUT_BIT: c_uint = 0x80000000;
    pub sizeof_port_stats: u32,
// Link Flap Avoidance
    pub lfa_host_addr: [u32; PORT_MAX],
    pub reserved1: u32,
    pub /: *mut *mut u32 reserved2; / Offset 0x148,
    pub /: *mut *mut u32 reserved3; / Offset 0x14C,
    pub /: *mut *mut u32 reserved4; / Offset 0x150,
    pub /: *mut *mut u32 link_attr_sync[PORT_MAX]; / Offset 0x154,
pub const LINK_ATTR_SYNC_KR2_ENABLE: c_uint = 0x00000001;
pub const LINK_ATTR_84858: c_uint = 0x00000002;
pub const LINK_SFP_EEPROM_COMP_CODE_MASK: c_uint = 0x0000ff00;
pub const LINK_SFP_EEPROM_COMP_CODE_SHIFT: c_int = 8;
pub const LINK_SFP_EEPROM_COMP_CODE_SR: c_uint = 0x00001000;
pub const LINK_SFP_EEPROM_COMP_CODE_LR: c_uint = 0x00002000;
pub const LINK_SFP_EEPROM_COMP_CODE_LRM: c_uint = 0x00004000;
    pub reserved5: [u32; 2],
    pub /: *mut *mut u32 link_change_count[PORT_MAX]; / Offset 0x160-0x164,
pub const LINK_CHANGE_COUNT_MASK: c_uint = 0xff     /* Offset 0x168 */;
// driver version for each personality
    pub /: *mut *mut os_drv_ver func_os_drv_ver[E2_FUNC_MAX]; / Offset 0x16c,
// Flag to the driver that PF's drv_info_host_addr buffer was read
    pub mfw_drv_indication: u32,
// We use indication for each PF (0..3)
    pub storage_boot_prog: [u8; E2_FUNC_MAX],
pub const STORAGE_BOOT_PROG_MASK: c_uint = 0x000000FF;
pub const STORAGE_BOOT_PROG_NONE: c_uint = 0x00000000;
pub const STORAGE_BOOT_PROG_ISCSI_IP_ACQUIRED: c_uint = 0x00000002;
pub const STORAGE_BOOT_PROG_FCOE_FABRIC_LOGIN_SUCCESS: c_uint = 0x00000002;
pub const STORAGE_BOOT_PROG_TARGET_FOUND: c_uint = 0x00000004;
pub const STORAGE_BOOT_PROG_ISCSI_CHAP_SUCCESS: c_uint = 0x00000008;
pub const STORAGE_BOOT_PROG_FCOE_LUN_FOUND: c_uint = 0x00000008;
pub const STORAGE_BOOT_PROG_LOGGED_INTO_TGT: c_uint = 0x00000010;
pub const STORAGE_BOOT_PROG_IMG_DOWNLOADED: c_uint = 0x00000020;
pub const STORAGE_BOOT_PROG_OS_HANDOFF: c_uint = 0x00000040;
pub const STORAGE_BOOT_PROG_COMPLETED: c_uint = 0x00000080;
    pub oem_i2c_data_addr: u32,
}

// 9 entires for the C2S PCP map for each inner VLAN PCP + 1 default
// For PCP values 0-3 use the map lower
// 0xFF000000 - PCP 0, 0x00FF0000 - PCP 1,
// 0x0000FF00 - PCP 2, 0x000000FF PCP 3
//
// For PCP values 4-7 use the map upper
// 0xFF000000 - PCP 4, 0x00FF0000 - PCP 5,
// 0x0000FF00 - PCP 6, 0x000000FF PCP 7
//
// For PCP default value get the MSB byte of the map default
// FC_NPIV table offset in NVRAM
// Shows last method that changed configuration of this device
// Storm FW version, shold be kept in the format 0xMMmmbbdd:
// MM - Major, mm - Minor, bb - Build ,dd - Drop
//
// Option ROM SMASH CLP version
pub const SRIOV_SWITCH_MODE_NONE: c_uint = 0x0;
pub const SRIOV_SWITCH_MODE_VEB: c_uint = 0x1;
pub const SRIOV_SWITCH_MODE_VEPA: c_uint = 0x2;

// mini dump driver info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_stats {
    pub rx_stat_ifhcinoctets: u32,
    pub rx_stat_ifhcinbadoctets: u32,
    pub rx_stat_etherstatsfragments: u32,
    pub rx_stat_ifhcinucastpkts: u32,
    pub rx_stat_ifhcinmulticastpkts: u32,
    pub rx_stat_ifhcinbroadcastpkts: u32,
    pub rx_stat_dot3statsfcserrors: u32,
    pub rx_stat_dot3statsalignmenterrors: u32,
    pub rx_stat_dot3statscarriersenseerrors: u32,
    pub rx_stat_xonpauseframesreceived: u32,
    pub rx_stat_xoffpauseframesreceived: u32,
    pub rx_stat_maccontrolframesreceived: u32,
    pub rx_stat_xoffstateentered: u32,
    pub rx_stat_dot3statsframestoolong: u32,
    pub rx_stat_etherstatsjabbers: u32,
    pub rx_stat_etherstatsundersizepkts: u32,
    pub rx_stat_etherstatspkts64octets: u32,
    pub rx_stat_etherstatspkts65octetsto127octets: u32,
    pub rx_stat_etherstatspkts128octetsto255octets: u32,
    pub rx_stat_etherstatspkts256octetsto511octets: u32,
    pub rx_stat_etherstatspkts512octetsto1023octets: u32,
    pub rx_stat_etherstatspkts1024octetsto1522octets: u32,
    pub rx_stat_etherstatspktsover1522octets: u32,
    pub rx_stat_falsecarriererrors: u32,
    pub tx_stat_ifhcoutoctets: u32,
    pub tx_stat_ifhcoutbadoctets: u32,
    pub tx_stat_etherstatscollisions: u32,
    pub tx_stat_outxonsent: u32,
    pub tx_stat_outxoffsent: u32,
    pub tx_stat_flowcontroldone: u32,
    pub tx_stat_dot3statssinglecollisionframes: u32,
    pub tx_stat_dot3statsmultiplecollisionframes: u32,
    pub tx_stat_dot3statsdeferredtransmissions: u32,
    pub tx_stat_dot3statsexcessivecollisions: u32,
    pub tx_stat_dot3statslatecollisions: u32,
    pub tx_stat_ifhcoutucastpkts: u32,
    pub tx_stat_ifhcoutmulticastpkts: u32,
    pub tx_stat_ifhcoutbroadcastpkts: u32,
    pub tx_stat_etherstatspkts64octets: u32,
    pub tx_stat_etherstatspkts65octetsto127octets: u32,
    pub tx_stat_etherstatspkts128octetsto255octets: u32,
    pub tx_stat_etherstatspkts256octetsto511octets: u32,
    pub tx_stat_etherstatspkts512octetsto1023octets: u32,
    pub tx_stat_etherstatspkts1024octetsto1522octets: u32,
    pub tx_stat_etherstatspktsover1522octets: u32,
    pub tx_stat_dot3statsinternalmactransmiterrors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmac1_stats {
    pub tx_stat_gtpkt_lo: u32,
    pub tx_stat_gtpkt_hi: u32,
    pub tx_stat_gtxpf_lo: u32,
    pub tx_stat_gtxpf_hi: u32,
    pub tx_stat_gtfcs_lo: u32,
    pub tx_stat_gtfcs_hi: u32,
    pub tx_stat_gtmca_lo: u32,
    pub tx_stat_gtmca_hi: u32,
    pub tx_stat_gtbca_lo: u32,
    pub tx_stat_gtbca_hi: u32,
    pub tx_stat_gtfrg_lo: u32,
    pub tx_stat_gtfrg_hi: u32,
    pub tx_stat_gtovr_lo: u32,
    pub tx_stat_gtovr_hi: u32,
    pub tx_stat_gt64_lo: u32,
    pub tx_stat_gt64_hi: u32,
    pub tx_stat_gt127_lo: u32,
    pub tx_stat_gt127_hi: u32,
    pub tx_stat_gt255_lo: u32,
    pub tx_stat_gt255_hi: u32,
    pub tx_stat_gt511_lo: u32,
    pub tx_stat_gt511_hi: u32,
    pub tx_stat_gt1023_lo: u32,
    pub tx_stat_gt1023_hi: u32,
    pub tx_stat_gt1518_lo: u32,
    pub tx_stat_gt1518_hi: u32,
    pub tx_stat_gt2047_lo: u32,
    pub tx_stat_gt2047_hi: u32,
    pub tx_stat_gt4095_lo: u32,
    pub tx_stat_gt4095_hi: u32,
    pub tx_stat_gt9216_lo: u32,
    pub tx_stat_gt9216_hi: u32,
    pub tx_stat_gt16383_lo: u32,
    pub tx_stat_gt16383_hi: u32,
    pub tx_stat_gtmax_lo: u32,
    pub tx_stat_gtmax_hi: u32,
    pub tx_stat_gtufl_lo: u32,
    pub tx_stat_gtufl_hi: u32,
    pub tx_stat_gterr_lo: u32,
    pub tx_stat_gterr_hi: u32,
    pub tx_stat_gtbyt_lo: u32,
    pub tx_stat_gtbyt_hi: u32,
    pub rx_stat_gr64_lo: u32,
    pub rx_stat_gr64_hi: u32,
    pub rx_stat_gr127_lo: u32,
    pub rx_stat_gr127_hi: u32,
    pub rx_stat_gr255_lo: u32,
    pub rx_stat_gr255_hi: u32,
    pub rx_stat_gr511_lo: u32,
    pub rx_stat_gr511_hi: u32,
    pub rx_stat_gr1023_lo: u32,
    pub rx_stat_gr1023_hi: u32,
    pub rx_stat_gr1518_lo: u32,
    pub rx_stat_gr1518_hi: u32,
    pub rx_stat_gr2047_lo: u32,
    pub rx_stat_gr2047_hi: u32,
    pub rx_stat_gr4095_lo: u32,
    pub rx_stat_gr4095_hi: u32,
    pub rx_stat_gr9216_lo: u32,
    pub rx_stat_gr9216_hi: u32,
    pub rx_stat_gr16383_lo: u32,
    pub rx_stat_gr16383_hi: u32,
    pub rx_stat_grmax_lo: u32,
    pub rx_stat_grmax_hi: u32,
    pub rx_stat_grpkt_lo: u32,
    pub rx_stat_grpkt_hi: u32,
    pub rx_stat_grfcs_lo: u32,
    pub rx_stat_grfcs_hi: u32,
    pub rx_stat_grmca_lo: u32,
    pub rx_stat_grmca_hi: u32,
    pub rx_stat_grbca_lo: u32,
    pub rx_stat_grbca_hi: u32,
    pub rx_stat_grxcf_lo: u32,
    pub rx_stat_grxcf_hi: u32,
    pub rx_stat_grxpf_lo: u32,
    pub rx_stat_grxpf_hi: u32,
    pub rx_stat_grxuo_lo: u32,
    pub rx_stat_grxuo_hi: u32,
    pub rx_stat_grjbr_lo: u32,
    pub rx_stat_grjbr_hi: u32,
    pub rx_stat_grovr_lo: u32,
    pub rx_stat_grovr_hi: u32,
    pub rx_stat_grflr_lo: u32,
    pub rx_stat_grflr_hi: u32,
    pub rx_stat_grmeg_lo: u32,
    pub rx_stat_grmeg_hi: u32,
    pub rx_stat_grmeb_lo: u32,
    pub rx_stat_grmeb_hi: u32,
    pub rx_stat_grbyt_lo: u32,
    pub rx_stat_grbyt_hi: u32,
    pub rx_stat_grund_lo: u32,
    pub rx_stat_grund_hi: u32,
    pub rx_stat_grfrg_lo: u32,
    pub rx_stat_grfrg_hi: u32,
    pub rx_stat_grerb_lo: u32,
    pub rx_stat_grerb_hi: u32,
    pub rx_stat_grfre_lo: u32,
    pub rx_stat_grfre_hi: u32,
    pub rx_stat_gripj_lo: u32,
    pub rx_stat_gripj_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmac2_stats {
    pub /: *mut *mut u32 tx_stat_gtpk_lo; / gtpok,
    pub /: *mut *mut u32 tx_stat_gtpk_hi; / gtpok,
    pub /: *mut *mut u32 tx_stat_gtxpf_lo; / gtpf,
    pub /: *mut *mut u32 tx_stat_gtxpf_hi; / gtpf,
    pub /: *mut *mut u32 tx_stat_gtpp_lo; / NEW BMAC2,
    pub /: *mut *mut u32 tx_stat_gtpp_hi; / NEW BMAC2,
    pub tx_stat_gtfcs_lo: u32,
    pub tx_stat_gtfcs_hi: u32,
    pub /: *mut *mut u32 tx_stat_gtuca_lo; / NEW BMAC2,
    pub /: *mut *mut u32 tx_stat_gtuca_hi; / NEW BMAC2,
    pub tx_stat_gtmca_lo: u32,
    pub tx_stat_gtmca_hi: u32,
    pub tx_stat_gtbca_lo: u32,
    pub tx_stat_gtbca_hi: u32,
    pub tx_stat_gtovr_lo: u32,
    pub tx_stat_gtovr_hi: u32,
    pub tx_stat_gtfrg_lo: u32,
    pub tx_stat_gtfrg_hi: u32,
    pub /: *mut *mut u32 tx_stat_gtpkt1_lo; / gtpkt,
    pub /: *mut *mut u32 tx_stat_gtpkt1_hi; / gtpkt,
    pub tx_stat_gt64_lo: u32,
    pub tx_stat_gt64_hi: u32,
    pub tx_stat_gt127_lo: u32,
    pub tx_stat_gt127_hi: u32,
    pub tx_stat_gt255_lo: u32,
    pub tx_stat_gt255_hi: u32,
    pub tx_stat_gt511_lo: u32,
    pub tx_stat_gt511_hi: u32,
    pub tx_stat_gt1023_lo: u32,
    pub tx_stat_gt1023_hi: u32,
    pub tx_stat_gt1518_lo: u32,
    pub tx_stat_gt1518_hi: u32,
    pub tx_stat_gt2047_lo: u32,
    pub tx_stat_gt2047_hi: u32,
    pub tx_stat_gt4095_lo: u32,
    pub tx_stat_gt4095_hi: u32,
    pub tx_stat_gt9216_lo: u32,
    pub tx_stat_gt9216_hi: u32,
    pub tx_stat_gt16383_lo: u32,
    pub tx_stat_gt16383_hi: u32,
    pub tx_stat_gtmax_lo: u32,
    pub tx_stat_gtmax_hi: u32,
    pub tx_stat_gtufl_lo: u32,
    pub tx_stat_gtufl_hi: u32,
    pub tx_stat_gterr_lo: u32,
    pub tx_stat_gterr_hi: u32,
    pub tx_stat_gtbyt_lo: u32,
    pub tx_stat_gtbyt_hi: u32,
    pub rx_stat_gr64_lo: u32,
    pub rx_stat_gr64_hi: u32,
    pub rx_stat_gr127_lo: u32,
    pub rx_stat_gr127_hi: u32,
    pub rx_stat_gr255_lo: u32,
    pub rx_stat_gr255_hi: u32,
    pub rx_stat_gr511_lo: u32,
    pub rx_stat_gr511_hi: u32,
    pub rx_stat_gr1023_lo: u32,
    pub rx_stat_gr1023_hi: u32,
    pub rx_stat_gr1518_lo: u32,
    pub rx_stat_gr1518_hi: u32,
    pub rx_stat_gr2047_lo: u32,
    pub rx_stat_gr2047_hi: u32,
    pub rx_stat_gr4095_lo: u32,
    pub rx_stat_gr4095_hi: u32,
    pub rx_stat_gr9216_lo: u32,
    pub rx_stat_gr9216_hi: u32,
    pub rx_stat_gr16383_lo: u32,
    pub rx_stat_gr16383_hi: u32,
    pub rx_stat_grmax_lo: u32,
    pub rx_stat_grmax_hi: u32,
    pub rx_stat_grpkt_lo: u32,
    pub rx_stat_grpkt_hi: u32,
    pub rx_stat_grfcs_lo: u32,
    pub rx_stat_grfcs_hi: u32,
    pub rx_stat_gruca_lo: u32,
    pub rx_stat_gruca_hi: u32,
    pub rx_stat_grmca_lo: u32,
    pub rx_stat_grmca_hi: u32,
    pub rx_stat_grbca_lo: u32,
    pub rx_stat_grbca_hi: u32,
    pub /: *mut *mut u32 rx_stat_grxpf_lo; / grpf,
    pub /: *mut *mut u32 rx_stat_grxpf_hi; / grpf,
    pub rx_stat_grpp_lo: u32,
    pub rx_stat_grpp_hi: u32,
    pub /: *mut *mut u32 rx_stat_grxuo_lo; / gruo,
    pub /: *mut *mut u32 rx_stat_grxuo_hi; / gruo,
    pub rx_stat_grjbr_lo: u32,
    pub rx_stat_grjbr_hi: u32,
    pub rx_stat_grovr_lo: u32,
    pub rx_stat_grovr_hi: u32,
    pub /: *mut *mut u32 rx_stat_grxcf_lo; / grcf,
    pub /: *mut *mut u32 rx_stat_grxcf_hi; / grcf,
    pub rx_stat_grflr_lo: u32,
    pub rx_stat_grflr_hi: u32,
    pub rx_stat_grpok_lo: u32,
    pub rx_stat_grpok_hi: u32,
    pub rx_stat_grmeg_lo: u32,
    pub rx_stat_grmeg_hi: u32,
    pub rx_stat_grmeb_lo: u32,
    pub rx_stat_grmeb_hi: u32,
    pub rx_stat_grbyt_lo: u32,
    pub rx_stat_grbyt_hi: u32,
    pub rx_stat_grund_lo: u32,
    pub rx_stat_grund_hi: u32,
    pub rx_stat_grfrg_lo: u32,
    pub rx_stat_grfrg_hi: u32,
    pub /: *mut *mut u32 rx_stat_grerb_lo; / grerrbyt,
    pub /: *mut *mut u32 rx_stat_grerb_hi; / grerrbyt,
    pub /: *mut *mut u32 rx_stat_grfre_lo; / grfrerr,
    pub /: *mut *mut u32 rx_stat_grfre_hi; / grfrerr,
    pub rx_stat_gripj_lo: u32,
    pub rx_stat_gripj_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstat_stats {
// OTE MSTAT on E3 has a bug where this register's contents are
// actually tx_gtxpok + tx_gtxpf + (possibly)tx_gtxpp
//
    pub tx_gtxpok_lo: u32,
    pub tx_gtxpok_hi: u32,
    pub tx_gtxpf_lo: u32,
    pub tx_gtxpf_hi: u32,
    pub tx_gtxpp_lo: u32,
    pub tx_gtxpp_hi: u32,
    pub tx_gtfcs_lo: u32,
    pub tx_gtfcs_hi: u32,
    pub tx_gtuca_lo: u32,
    pub tx_gtuca_hi: u32,
    pub tx_gtmca_lo: u32,
    pub tx_gtmca_hi: u32,
    pub tx_gtgca_lo: u32,
    pub tx_gtgca_hi: u32,
    pub tx_gtpkt_lo: u32,
    pub tx_gtpkt_hi: u32,
    pub tx_gt64_lo: u32,
    pub tx_gt64_hi: u32,
    pub tx_gt127_lo: u32,
    pub tx_gt127_hi: u32,
    pub tx_gt255_lo: u32,
    pub tx_gt255_hi: u32,
    pub tx_gt511_lo: u32,
    pub tx_gt511_hi: u32,
    pub tx_gt1023_lo: u32,
    pub tx_gt1023_hi: u32,
    pub tx_gt1518_lo: u32,
    pub tx_gt1518_hi: u32,
    pub tx_gt2047_lo: u32,
    pub tx_gt2047_hi: u32,
    pub tx_gt4095_lo: u32,
    pub tx_gt4095_hi: u32,
    pub tx_gt9216_lo: u32,
    pub tx_gt9216_hi: u32,
    pub tx_gt16383_lo: u32,
    pub tx_gt16383_hi: u32,
    pub tx_gtufl_lo: u32,
    pub tx_gtufl_hi: u32,
    pub tx_gterr_lo: u32,
    pub tx_gterr_hi: u32,
    pub tx_gtbyt_lo: u32,
    pub tx_gtbyt_hi: u32,
    pub tx_collisions_lo: u32,
    pub tx_collisions_hi: u32,
    pub tx_singlecollision_lo: u32,
    pub tx_singlecollision_hi: u32,
    pub tx_multiplecollisions_lo: u32,
    pub tx_multiplecollisions_hi: u32,
    pub tx_deferred_lo: u32,
    pub tx_deferred_hi: u32,
    pub tx_excessivecollisions_lo: u32,
    pub tx_excessivecollisions_hi: u32,
    pub tx_latecollisions_lo: u32,
    pub tx_latecollisions_hi: u32,
    pub stats_tx: },
    pub rx_gr64_lo: u32,
    pub rx_gr64_hi: u32,
    pub rx_gr127_lo: u32,
    pub rx_gr127_hi: u32,
    pub rx_gr255_lo: u32,
    pub rx_gr255_hi: u32,
    pub rx_gr511_lo: u32,
    pub rx_gr511_hi: u32,
    pub rx_gr1023_lo: u32,
    pub rx_gr1023_hi: u32,
    pub rx_gr1518_lo: u32,
    pub rx_gr1518_hi: u32,
    pub rx_gr2047_lo: u32,
    pub rx_gr2047_hi: u32,
    pub rx_gr4095_lo: u32,
    pub rx_gr4095_hi: u32,
    pub rx_gr9216_lo: u32,
    pub rx_gr9216_hi: u32,
    pub rx_gr16383_lo: u32,
    pub rx_gr16383_hi: u32,
    pub rx_grpkt_lo: u32,
    pub rx_grpkt_hi: u32,
    pub rx_grfcs_lo: u32,
    pub rx_grfcs_hi: u32,
    pub rx_gruca_lo: u32,
    pub rx_gruca_hi: u32,
    pub rx_grmca_lo: u32,
    pub rx_grmca_hi: u32,
    pub rx_grbca_lo: u32,
    pub rx_grbca_hi: u32,
    pub rx_grxpf_lo: u32,
    pub rx_grxpf_hi: u32,
    pub rx_grxpp_lo: u32,
    pub rx_grxpp_hi: u32,
    pub rx_grxuo_lo: u32,
    pub rx_grxuo_hi: u32,
    pub rx_grovr_lo: u32,
    pub rx_grovr_hi: u32,
    pub rx_grxcf_lo: u32,
    pub rx_grxcf_hi: u32,
    pub rx_grflr_lo: u32,
    pub rx_grflr_hi: u32,
    pub rx_grpok_lo: u32,
    pub rx_grpok_hi: u32,
    pub rx_grbyt_lo: u32,
    pub rx_grbyt_hi: u32,
    pub rx_grund_lo: u32,
    pub rx_grund_hi: u32,
    pub rx_grfrg_lo: u32,
    pub rx_grfrg_hi: u32,
    pub rx_grerb_lo: u32,
    pub rx_grerb_hi: u32,
    pub rx_grfre_lo: u32,
    pub rx_grfre_hi: u32,
    pub rx_alignmenterrors_lo: u32,
    pub rx_alignmenterrors_hi: u32,
    pub rx_falsecarrier_lo: u32,
    pub rx_falsecarrier_hi: u32,
    pub rx_llfcmsgcnt_lo: u32,
    pub rx_llfcmsgcnt_hi: u32,
    pub stats_rx: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mac_stats {
    pub emac_stats: emac_stats,
    pub bmac1_stats: bmac1_stats,
    pub bmac2_stats: bmac2_stats,
    pub mstat_stats: mstat_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_stx {
// in_bad_octets
    pub rx_stat_ifhcinbadoctets_hi: u32,
    pub rx_stat_ifhcinbadoctets_lo: u32,
// out_bad_octets
    pub tx_stat_ifhcoutbadoctets_hi: u32,
    pub tx_stat_ifhcoutbadoctets_lo: u32,
// crc_receive_errors
    pub rx_stat_dot3statsfcserrors_hi: u32,
    pub rx_stat_dot3statsfcserrors_lo: u32,
// alignment_errors
    pub rx_stat_dot3statsalignmenterrors_hi: u32,
    pub rx_stat_dot3statsalignmenterrors_lo: u32,
// carrier_sense_errors
    pub rx_stat_dot3statscarriersenseerrors_hi: u32,
    pub rx_stat_dot3statscarriersenseerrors_lo: u32,
// false_carrier_detections
    pub rx_stat_falsecarriererrors_hi: u32,
    pub rx_stat_falsecarriererrors_lo: u32,
// runt_packets_received
    pub rx_stat_etherstatsundersizepkts_hi: u32,
    pub rx_stat_etherstatsundersizepkts_lo: u32,
// jabber_packets_received
    pub rx_stat_dot3statsframestoolong_hi: u32,
    pub rx_stat_dot3statsframestoolong_lo: u32,
// error_runt_packets_received
    pub rx_stat_etherstatsfragments_hi: u32,
    pub rx_stat_etherstatsfragments_lo: u32,
// error_jabber_packets_received
    pub rx_stat_etherstatsjabbers_hi: u32,
    pub rx_stat_etherstatsjabbers_lo: u32,
// control_frames_received
    pub rx_stat_maccontrolframesreceived_hi: u32,
    pub rx_stat_maccontrolframesreceived_lo: u32,
    pub rx_stat_mac_xpf_hi: u32,
    pub rx_stat_mac_xpf_lo: u32,
    pub rx_stat_mac_xcf_hi: u32,
    pub rx_stat_mac_xcf_lo: u32,
// xoff_state_entered
    pub rx_stat_xoffstateentered_hi: u32,
    pub rx_stat_xoffstateentered_lo: u32,
// pause_xon_frames_received
    pub rx_stat_xonpauseframesreceived_hi: u32,
    pub rx_stat_xonpauseframesreceived_lo: u32,
// pause_xoff_frames_received
    pub rx_stat_xoffpauseframesreceived_hi: u32,
    pub rx_stat_xoffpauseframesreceived_lo: u32,
// pause_xon_frames_transmitted
    pub tx_stat_outxonsent_hi: u32,
    pub tx_stat_outxonsent_lo: u32,
// pause_xoff_frames_transmitted
    pub tx_stat_outxoffsent_hi: u32,
    pub tx_stat_outxoffsent_lo: u32,
// flow_control_done
    pub tx_stat_flowcontroldone_hi: u32,
    pub tx_stat_flowcontroldone_lo: u32,
// ether_stats_collisions
    pub tx_stat_etherstatscollisions_hi: u32,
    pub tx_stat_etherstatscollisions_lo: u32,
// single_collision_transmit_frames
    pub tx_stat_dot3statssinglecollisionframes_hi: u32,
    pub tx_stat_dot3statssinglecollisionframes_lo: u32,
// multiple_collision_transmit_frames
    pub tx_stat_dot3statsmultiplecollisionframes_hi: u32,
    pub tx_stat_dot3statsmultiplecollisionframes_lo: u32,
// deferred_transmissions
    pub tx_stat_dot3statsdeferredtransmissions_hi: u32,
    pub tx_stat_dot3statsdeferredtransmissions_lo: u32,
// excessive_collision_frames
    pub tx_stat_dot3statsexcessivecollisions_hi: u32,
    pub tx_stat_dot3statsexcessivecollisions_lo: u32,
// late_collision_frames
    pub tx_stat_dot3statslatecollisions_hi: u32,
    pub tx_stat_dot3statslatecollisions_lo: u32,
// frames_transmitted_64_bytes
    pub tx_stat_etherstatspkts64octets_hi: u32,
    pub tx_stat_etherstatspkts64octets_lo: u32,
// frames_transmitted_65_127_bytes
    pub tx_stat_etherstatspkts65octetsto127octets_hi: u32,
    pub tx_stat_etherstatspkts65octetsto127octets_lo: u32,
// frames_transmitted_128_255_bytes
    pub tx_stat_etherstatspkts128octetsto255octets_hi: u32,
    pub tx_stat_etherstatspkts128octetsto255octets_lo: u32,
// frames_transmitted_256_511_bytes
    pub tx_stat_etherstatspkts256octetsto511octets_hi: u32,
    pub tx_stat_etherstatspkts256octetsto511octets_lo: u32,
// frames_transmitted_512_1023_bytes
    pub tx_stat_etherstatspkts512octetsto1023octets_hi: u32,
    pub tx_stat_etherstatspkts512octetsto1023octets_lo: u32,
// frames_transmitted_1024_1522_bytes
    pub tx_stat_etherstatspkts1024octetsto1522octets_hi: u32,
    pub tx_stat_etherstatspkts1024octetsto1522octets_lo: u32,
// frames_transmitted_1523_9022_bytes
    pub tx_stat_etherstatspktsover1522octets_hi: u32,
    pub tx_stat_etherstatspktsover1522octets_lo: u32,
    pub tx_stat_mac_2047_hi: u32,
    pub tx_stat_mac_2047_lo: u32,
    pub tx_stat_mac_4095_hi: u32,
    pub tx_stat_mac_4095_lo: u32,
    pub tx_stat_mac_9216_hi: u32,
    pub tx_stat_mac_9216_lo: u32,
    pub tx_stat_mac_16383_hi: u32,
    pub tx_stat_mac_16383_lo: u32,
// internal_mac_transmit_errors
    pub tx_stat_dot3statsinternalmactransmiterrors_hi: u32,
    pub tx_stat_dot3statsinternalmactransmiterrors_lo: u32,
// if_out_discards
    pub tx_stat_mac_ufl_hi: u32,
    pub tx_stat_mac_ufl_lo: u32,
}

pub const MAC_STX_IDX_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_port_stats {
    pub host_port_stats_counter: u32,
    pub mac_stx: [mac_stx; MAC_STX_IDX_MAX],
    pub brb_drop_hi: u32,
    pub brb_drop_lo: u32,
    pub /: *mut *mut u32 not_used; / obsolete,
    pub pfc_frames_tx_hi: u32,
    pub pfc_frames_tx_lo: u32,
    pub pfc_frames_rx_hi: u32,
    pub pfc_frames_rx_lo: u32,
    pub eee_lpi_count_hi: u32,
    pub eee_lpi_count_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_func_stats {
    pub host_func_stats_start: u32,
    pub total_bytes_received_hi: u32,
    pub total_bytes_received_lo: u32,
    pub total_bytes_transmitted_hi: u32,
    pub total_bytes_transmitted_lo: u32,
    pub total_unicast_packets_received_hi: u32,
    pub total_unicast_packets_received_lo: u32,
    pub total_multicast_packets_received_hi: u32,
    pub total_multicast_packets_received_lo: u32,
    pub total_broadcast_packets_received_hi: u32,
    pub total_broadcast_packets_received_lo: u32,
    pub total_unicast_packets_transmitted_hi: u32,
    pub total_unicast_packets_transmitted_lo: u32,
    pub total_multicast_packets_transmitted_hi: u32,
    pub total_multicast_packets_transmitted_lo: u32,
    pub total_broadcast_packets_transmitted_hi: u32,
    pub total_broadcast_packets_transmitted_lo: u32,
    pub valid_bytes_received_hi: u32,
    pub valid_bytes_received_lo: u32,
    pub host_func_stats_end: u32,
}

// VIC definitions
pub const VICSTATST_UIF_INDEX: c_int = 2;
// stats collected for afex.
// NOTE: structure is exactly as expected to be received by the switch.
// order must remain exactly as is unless protocol changes !
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afex_stats {
    pub tx_unicast_frames_hi: u32,
    pub tx_unicast_frames_lo: u32,
    pub tx_unicast_bytes_hi: u32,
    pub tx_unicast_bytes_lo: u32,
    pub tx_multicast_frames_hi: u32,
    pub tx_multicast_frames_lo: u32,
    pub tx_multicast_bytes_hi: u32,
    pub tx_multicast_bytes_lo: u32,
    pub tx_broadcast_frames_hi: u32,
    pub tx_broadcast_frames_lo: u32,
    pub tx_broadcast_bytes_hi: u32,
    pub tx_broadcast_bytes_lo: u32,
    pub tx_frames_discarded_hi: u32,
    pub tx_frames_discarded_lo: u32,
    pub tx_frames_dropped_hi: u32,
    pub tx_frames_dropped_lo: u32,
    pub rx_unicast_frames_hi: u32,
    pub rx_unicast_frames_lo: u32,
    pub rx_unicast_bytes_hi: u32,
    pub rx_unicast_bytes_lo: u32,
    pub rx_multicast_frames_hi: u32,
    pub rx_multicast_frames_lo: u32,
    pub rx_multicast_bytes_hi: u32,
    pub rx_multicast_bytes_lo: u32,
    pub rx_broadcast_frames_hi: u32,
    pub rx_broadcast_frames_lo: u32,
    pub rx_broadcast_bytes_hi: u32,
    pub rx_broadcast_bytes_lo: u32,
    pub rx_frames_discarded_hi: u32,
    pub rx_frames_discarded_lo: u32,
    pub rx_frames_dropped_hi: u32,
    pub rx_frames_dropped_lo: u32,
}

pub const BCM_5710_FW_MAJOR_VERSION: c_int = 7;
pub const BCM_5710_FW_MINOR_VERSION: c_int = 13;
pub const BCM_5710_FW_REVISION_VERSION: c_int = 21;
pub const BCM_5710_FW_REVISION_VERSION_V15: c_int = 15;
pub const BCM_5710_FW_ENGINEERING_VERSION: c_int = 0;
pub const BCM_5710_FW_COMPILE_FLAGS: c_int = 1;
//
// attention bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atten_sp_status_block {
    pub attn_bits: __le32,
    pub attn_bits_ack: __le32,
    pub status_block_id: u8,
    pub reserved0: u8,
    pub attn_bits_index: __le16,
    pub reserved1: __le32,
}

//
// The eth aggregative context of Cstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstorm_eth_ag_context {
    pub __reserved0: [u32; 10],
}

//
// dmae command structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmae_command {
    pub opcode: u32,

pub const DMAE_COMMAND_SRC_SHIFT: c_int = 0;

pub const DMAE_COMMAND_DST_SHIFT: c_int = 1;

pub const DMAE_COMMAND_C_DST_SHIFT: c_int = 3;

pub const DMAE_COMMAND_C_TYPE_ENABLE_SHIFT: c_int = 4;

pub const DMAE_COMMAND_C_TYPE_CRC_ENABLE_SHIFT: c_int = 5;

pub const DMAE_COMMAND_C_TYPE_CRC_OFFSET_SHIFT: c_int = 6;

pub const DMAE_COMMAND_ENDIANITY_SHIFT: c_int = 9;

pub const DMAE_COMMAND_PORT_SHIFT: c_int = 11;

pub const DMAE_COMMAND_CRC_RESET_SHIFT: c_int = 12;

pub const DMAE_COMMAND_SRC_RESET_SHIFT: c_int = 13;

pub const DMAE_COMMAND_DST_RESET_SHIFT: c_int = 14;

pub const DMAE_COMMAND_E1HVN_SHIFT: c_int = 15;

pub const DMAE_COMMAND_DST_VN_SHIFT: c_int = 17;

pub const DMAE_COMMAND_C_FUNC_SHIFT: c_int = 19;

pub const DMAE_COMMAND_ERR_POLICY_SHIFT: c_int = 20;

pub const DMAE_COMMAND_RESERVED0_SHIFT: c_int = 22;
    pub src_addr_lo: u32,
    pub src_addr_hi: u32,
    pub dst_addr_lo: u32,
    pub dst_addr_hi: u32,

    pub opcode_iov: u16,

pub const DMAE_COMMAND_SRC_VFID_SHIFT: c_int = 0;

pub const DMAE_COMMAND_SRC_VFPF_SHIFT: c_int = 6;

pub const DMAE_COMMAND_RESERVED1_SHIFT: c_int = 7;

pub const DMAE_COMMAND_DST_VFID_SHIFT: c_int = 8;

pub const DMAE_COMMAND_DST_VFPF_SHIFT: c_int = 14;

pub const DMAE_COMMAND_RESERVED2_SHIFT: c_int = 15;
    pub len: u16,

    pub len: u16,
    pub opcode_iov: u16,

pub const DMAE_COMMAND_SRC_VFID_SHIFT: c_int = 0;

pub const DMAE_COMMAND_SRC_VFPF_SHIFT: c_int = 6;

pub const DMAE_COMMAND_RESERVED1_SHIFT: c_int = 7;

pub const DMAE_COMMAND_DST_VFID_SHIFT: c_int = 8;

pub const DMAE_COMMAND_DST_VFPF_SHIFT: c_int = 14;

pub const DMAE_COMMAND_RESERVED2_SHIFT: c_int = 15;

    pub comp_addr_lo: u32,
    pub comp_addr_hi: u32,
    pub comp_val: u32,
    pub crc32: u32,
    pub crc32_c: u32,

    pub crc16_c: u16,
    pub crc16: u16,

    pub crc16: u16,
    pub crc16_c: u16,

    pub reserved3: u16,
    pub crc_t10: u16,

    pub crc_t10: u16,
    pub reserved3: u16,

    pub xsum8: u16,
    pub xsum16: u16,

    pub xsum16: u16,
    pub xsum8: u16,

}

//
// common data for all protocols
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct doorbell_hdr {
    pub header: u8,

pub const DOORBELL_HDR_RX_SHIFT: c_int = 0;

pub const DOORBELL_HDR_DB_TYPE_SHIFT: c_int = 1;

pub const DOORBELL_HDR_DPM_SIZE_SHIFT: c_int = 2;

pub const DOORBELL_HDR_CONN_TYPE_SHIFT: c_int = 4;
}

//
// Ethernet doorbell
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_doorbell {

    pub npackets: u16,
    pub params: u8,

pub const ETH_TX_DOORBELL_NUM_BDS_SHIFT: c_int = 0;

pub const ETH_TX_DOORBELL_RESERVED_TX_FIN_FLAG_SHIFT: c_int = 6;

pub const ETH_TX_DOORBELL_SPARE_SHIFT: c_int = 7;
    pub hdr: doorbell_hdr,

    pub hdr: doorbell_hdr,
    pub params: u8,

pub const ETH_TX_DOORBELL_NUM_BDS_SHIFT: c_int = 0;

pub const ETH_TX_DOORBELL_RESERVED_TX_FIN_FLAG_SHIFT: c_int = 6;

pub const ETH_TX_DOORBELL_SPARE_SHIFT: c_int = 7;
    pub npackets: u16,

}

//
// 3 lines. status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_status_block_e1x {
    pub index_values: [__le16; HC_SB_MAX_INDICES_E1X],
    pub running_index: [__le16; HC_SB_MAX_SM],
    pub rsrv: [__le32; 11],
}

//
// host status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_hc_status_block_e1x {
    pub sb: hc_status_block_e1x,
}

//
// 3 lines. status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_status_block_e2 {
    pub index_values: [__le16; HC_SB_MAX_INDICES_E2],
    pub running_index: [__le16; HC_SB_MAX_SM],
    pub reserved: [__le32; 11],
}

//
// host status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_hc_status_block_e2 {
    pub sb: hc_status_block_e2,
}

//
// 5 lines. slow-path status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_sp_status_block {
    pub index_values: [__le16; HC_SP_SB_MAX_INDICES],
    pub running_index: __le16,
    pub rsrv: __le16,
    pub rsrv1: u32,
}

//
// host status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_sp_status_block {
    pub atten_status_block: atten_sp_status_block,
    pub sp_sb: hc_sp_status_block,
}

//
// IGU driver acknowledgment register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_ack_register {

    pub sb_id_and_flags: u16,

pub const IGU_ACK_REGISTER_STATUS_BLOCK_ID_SHIFT: c_int = 0;

pub const IGU_ACK_REGISTER_STORM_ID_SHIFT: c_int = 5;

pub const IGU_ACK_REGISTER_UPDATE_INDEX_SHIFT: c_int = 8;

pub const IGU_ACK_REGISTER_INTERRUPT_MODE_SHIFT: c_int = 9;

pub const IGU_ACK_REGISTER_RESERVED_SHIFT: c_int = 11;
    pub status_block_index: u16,

    pub status_block_index: u16,
    pub sb_id_and_flags: u16,

pub const IGU_ACK_REGISTER_STATUS_BLOCK_ID_SHIFT: c_int = 0;

pub const IGU_ACK_REGISTER_STORM_ID_SHIFT: c_int = 5;

pub const IGU_ACK_REGISTER_UPDATE_INDEX_SHIFT: c_int = 8;

pub const IGU_ACK_REGISTER_INTERRUPT_MODE_SHIFT: c_int = 9;

pub const IGU_ACK_REGISTER_RESERVED_SHIFT: c_int = 11;

}

//
// IGU driver acknowledgement register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_backward_compatible {
    pub sb_id_and_flags: u32,

pub const IGU_BACKWARD_COMPATIBLE_SB_INDEX_SHIFT: c_int = 0;

pub const IGU_BACKWARD_COMPATIBLE_SB_SELECT_SHIFT: c_int = 16;

pub const IGU_BACKWARD_COMPATIBLE_SEGMENT_ACCESS_SHIFT: c_int = 21;

pub const IGU_BACKWARD_COMPATIBLE_BUPDATE_SHIFT: c_int = 24;

pub const IGU_BACKWARD_COMPATIBLE_ENABLE_INT_SHIFT: c_int = 25;

pub const IGU_BACKWARD_COMPATIBLE_RESERVED_0_SHIFT: c_int = 27;
    pub reserved_2: u32,
}

//
// IGU driver acknowledgement register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_regular {
    pub sb_id_and_flags: u32,

pub const IGU_REGULAR_SB_INDEX_SHIFT: c_int = 0;

pub const IGU_REGULAR_RESERVED0_SHIFT: c_int = 20;

pub const IGU_REGULAR_SEGMENT_ACCESS_SHIFT: c_int = 21;

pub const IGU_REGULAR_BUPDATE_SHIFT: c_int = 24;

pub const IGU_REGULAR_ENABLE_INT_SHIFT: c_int = 25;

pub const IGU_REGULAR_RESERVED_1_SHIFT: c_int = 27;

pub const IGU_REGULAR_CLEANUP_TYPE_SHIFT: c_int = 28;

pub const IGU_REGULAR_CLEANUP_SET_SHIFT: c_int = 30;

pub const IGU_REGULAR_BCLEANUP_SHIFT: c_int = 31;
    pub reserved_2: u32,
}

//
// IGU driver acknowledgement register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union igu_consprod_reg {
    pub regular: igu_regular,
    pub backward_compatible: igu_backward_compatible,
}

//
// Igu control commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_ctrl_cmd {
    IGU_CTRL_CMD_TYPE_RD,
    IGU_CTRL_CMD_TYPE_WR,
    MAX_IGU_CTRL_CMD
}

//
// Control register for the IGU command register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_ctrl_reg {
    pub ctrl_data: u32,

pub const IGU_CTRL_REG_ADDRESS_SHIFT: c_int = 0;

pub const IGU_CTRL_REG_FID_SHIFT: c_int = 12;

pub const IGU_CTRL_REG_RESERVED_SHIFT: c_int = 19;

pub const IGU_CTRL_REG_TYPE_SHIFT: c_int = 20;

pub const IGU_CTRL_REG_UNUSED_SHIFT: c_int = 21;
}

//
// Igu interrupt command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_int_cmd {
    IGU_INT_ENABLE,
    IGU_INT_DISABLE,
    IGU_INT_NOP,
    IGU_INT_NOP2,
    MAX_IGU_INT_CMD
}

//
// Igu segments
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_seg_access {
    IGU_SEG_ACCESS_NORM,
    IGU_SEG_ACCESS_DEF,
    IGU_SEG_ACCESS_ATTN,
    MAX_IGU_SEG_ACCESS
}

//
// Parser parsing flags field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parsing_flags {
    pub flags: __le16,

pub const PARSING_FLAGS_ETHERNET_ADDRESS_TYPE_SHIFT: c_int = 0;

pub const PARSING_FLAGS_VLAN_SHIFT: c_int = 1;

pub const PARSING_FLAGS_EXTRA_VLAN_SHIFT: c_int = 2;

pub const PARSING_FLAGS_OVER_ETHERNET_PROTOCOL_SHIFT: c_int = 3;

pub const PARSING_FLAGS_IP_OPTIONS_SHIFT: c_int = 5;

pub const PARSING_FLAGS_FRAGMENTATION_STATUS_SHIFT: c_int = 6;

pub const PARSING_FLAGS_OVER_IP_PROTOCOL_SHIFT: c_int = 7;

pub const PARSING_FLAGS_PURE_ACK_INDICATION_SHIFT: c_int = 9;

pub const PARSING_FLAGS_TCP_OPTIONS_EXIST_SHIFT: c_int = 10;

pub const PARSING_FLAGS_TIME_STAMP_EXIST_FLAG_SHIFT: c_int = 11;

pub const PARSING_FLAGS_CONNECTION_MATCH_SHIFT: c_int = 12;

pub const PARSING_FLAGS_LLC_SNAP_SHIFT: c_int = 13;

pub const PARSING_FLAGS_RESERVED0_SHIFT: c_int = 14;
}

//
// Parsing flags for TCP ACK type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_flags_ack_type {
    PRS_FLAG_PUREACK_PIGGY,
    PRS_FLAG_PUREACK_PURE,
    MAX_PRS_FLAGS_ACK_TYPE
}

//
// Parsing flags for Ethernet address type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_flags_eth_addr_type {
    PRS_FLAG_ETHTYPE_NON_UNICAST,
    PRS_FLAG_ETHTYPE_UNICAST,
    MAX_PRS_FLAGS_ETH_ADDR_TYPE
}

//
// Parsing flags for over-ethernet protocol
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_flags_over_eth {
    PRS_FLAG_OVERETH_UNKNOWN,
    PRS_FLAG_OVERETH_IPV4,
    PRS_FLAG_OVERETH_IPV6,
    PRS_FLAG_OVERETH_LLCSNAP_UNKNOWN,
    MAX_PRS_FLAGS_OVER_ETH
}

//
// Parsing flags for over-IP protocol
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_flags_over_ip {
    PRS_FLAG_OVERIP_UNKNOWN,
    PRS_FLAG_OVERIP_TCP,
    PRS_FLAG_OVERIP_UDP,
    MAX_PRS_FLAGS_OVER_IP
}

//
// SDM operation gen command (generate aggregative interrupt)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdm_op_gen {
    pub command: __le32,

pub const SDM_OP_GEN_COMP_PARAM_SHIFT: c_int = 0;

pub const SDM_OP_GEN_COMP_TYPE_SHIFT: c_int = 5;

pub const SDM_OP_GEN_AGG_VECT_IDX_SHIFT: c_int = 8;

pub const SDM_OP_GEN_AGG_VECT_IDX_VALID_SHIFT: c_int = 16;

pub const SDM_OP_GEN_RESERVED_SHIFT: c_int = 17;
}

//
// Timers connection context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timers_block_context {
    pub __reserved_0: u32,
    pub __reserved_1: u32,
    pub __reserved_2: u32,
    pub flags: u32,

pub const __TIMERS_BLOCK_CONTEXT_NUM_OF_ACTIVE_TIMERS_SHIFT: c_int = 0;

pub const TIMERS_BLOCK_CONTEXT_CONN_VALID_FLG_SHIFT: c_int = 2;

pub const __TIMERS_BLOCK_CONTEXT_RESERVED0_SHIFT: c_int = 3;
}

//
// The eth aggregative context of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_ag_context {
    pub __reserved0: [u32; 14],
}

//
// The eth aggregative context of Ustorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_eth_ag_context {
    pub __reserved0: u32,

    pub cdu_usage: u8,
    pub __reserved2: u8,
    pub __reserved1: u16,

    pub __reserved1: u16,
    pub __reserved2: u8,
    pub cdu_usage: u8,
    pub __reserved3: [u32; 6],
}

//
// The eth aggregative context of Xstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_ag_context {
    pub reserved0: u32,

    pub cdu_reserved: u8,
    pub reserved2: u8,
    pub reserved1: u16,

    pub reserved1: u16,
    pub reserved2: u8,
    pub cdu_reserved: u8,
    pub reserved3: [u32; 30],
}

//
// doorbell message sent to the chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct doorbell {

    pub zero_fill2: u16,
    pub zero_fill1: u8,
    pub header: doorbell_hdr,

    pub header: doorbell_hdr,
    pub zero_fill1: u8,
    pub zero_fill2: u16,

}

//
// doorbell message sent to the chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct doorbell_set_prod {

    pub prod: u16,
    pub zero_fill1: u8,
    pub header: doorbell_hdr,

    pub header: doorbell_hdr,
    pub zero_fill1: u8,
    pub prod: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regpair {
    pub lo: __le32,
    pub hi: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regpair_native {
    pub lo: u32,
    pub hi: u32,
}

//
// Classify rule opcodes in E2/E3
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum classify_rule {
    CLASSIFY_RULE_OPCODE_MAC,
    CLASSIFY_RULE_OPCODE_VLAN,
    CLASSIFY_RULE_OPCODE_PAIR,
    CLASSIFY_RULE_OPCODE_IMAC_VNI,
    MAX_CLASSIFY_RULE
}

//
// Classify rule types in E2/E3
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum classify_rule_action_type {
    CLASSIFY_RULE_REMOVE,
    CLASSIFY_RULE_ADD,
    MAX_CLASSIFY_RULE_ACTION_TYPE
}

//
// client init ramrod data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_init_general_data {
    pub client_id: u8,
    pub statistics_counter_id: u8,
    pub statistics_en_flg: u8,
    pub is_fcoe_flg: u8,
    pub activate_flg: u8,
    pub sp_client_id: u8,
    pub mtu: __le16,
    pub statistics_zero_flg: u8,
    pub func_id: u8,
    pub cos: u8,
    pub traffic_type: u8,
    pub fp_hsi_ver: u8,
    pub reserved0: [u8; 3],
}

//
// client init rx data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_init_rx_data {
    pub tpa_en: u8,

pub const CLIENT_INIT_RX_DATA_TPA_EN_IPV4_SHIFT: c_int = 0;

pub const CLIENT_INIT_RX_DATA_TPA_EN_IPV6_SHIFT: c_int = 1;

pub const CLIENT_INIT_RX_DATA_TPA_MODE_SHIFT: c_int = 2;

pub const CLIENT_INIT_RX_DATA_TPA_OVER_VLAN_DISABLE_SHIFT: c_int = 3;

pub const CLIENT_INIT_RX_DATA_RESERVED5_SHIFT: c_int = 4;
    pub vmqueue_mode_en_flg: u8,
    pub extra_data_over_sgl_en_flg: u8,
    pub cache_line_alignment_log_size: u8,
    pub enable_dynamic_hc: u8,
    pub max_sges_for_packet: u8,
    pub client_qzone_id: u8,
    pub drop_ip_cs_err_flg: u8,
    pub drop_tcp_cs_err_flg: u8,
    pub drop_ttl0_flg: u8,
    pub drop_udp_cs_err_flg: u8,
    pub inner_vlan_removal_enable_flg: u8,
    pub outer_vlan_removal_enable_flg: u8,
    pub status_block_id: u8,
    pub rx_sb_index_number: u8,
    pub dont_verify_rings_pause_thr_flg: u8,
    pub max_tpa_queues: u8,
    pub silent_vlan_removal_flg: u8,
    pub max_bytes_on_bd: __le16,
    pub sge_buff_size: __le16,
    pub approx_mcast_engine_id: u8,
    pub rss_engine_id: u8,
    pub bd_page_base: regpair,
    pub sge_page_base: regpair,
    pub cqe_page_base: regpair,
    pub is_leading_rss: u8,
    pub is_approx_mcast: u8,
    pub max_agg_size: __le16,
    pub state: __le16,

pub const CLIENT_INIT_RX_DATA_UCAST_DROP_ALL_SHIFT: c_int = 0;

pub const CLIENT_INIT_RX_DATA_UCAST_ACCEPT_ALL_SHIFT: c_int = 1;

pub const CLIENT_INIT_RX_DATA_UCAST_ACCEPT_UNMATCHED_SHIFT: c_int = 2;

pub const CLIENT_INIT_RX_DATA_MCAST_DROP_ALL_SHIFT: c_int = 3;

pub const CLIENT_INIT_RX_DATA_MCAST_ACCEPT_ALL_SHIFT: c_int = 4;

pub const CLIENT_INIT_RX_DATA_BCAST_ACCEPT_ALL_SHIFT: c_int = 5;

pub const CLIENT_INIT_RX_DATA_ACCEPT_ANY_VLAN_SHIFT: c_int = 6;

pub const CLIENT_INIT_RX_DATA_RESERVED2_SHIFT: c_int = 7;
    pub cqe_pause_thr_low: __le16,
    pub cqe_pause_thr_high: __le16,
    pub bd_pause_thr_low: __le16,
    pub bd_pause_thr_high: __le16,
    pub sge_pause_thr_low: __le16,
    pub sge_pause_thr_high: __le16,
    pub rx_cos_mask: __le16,
    pub silent_vlan_value: __le16,
    pub silent_vlan_mask: __le16,
    pub handle_ptp_pkts_flg: u8,
    pub reserved6: [u8; 3],
    pub reserved7: __le32,
}

//
// client init tx data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_init_tx_data {
    pub enforce_security_flg: u8,
    pub tx_status_block_id: u8,
    pub tx_sb_index_number: u8,
    pub tss_leading_client_id: u8,
    pub tx_switching_flg: u8,
    pub anti_spoofing_flg: u8,
    pub default_vlan: __le16,
    pub tx_bd_page_base: regpair,
    pub state: __le16,

pub const CLIENT_INIT_TX_DATA_UCAST_ACCEPT_ALL_SHIFT: c_int = 0;

pub const CLIENT_INIT_TX_DATA_MCAST_ACCEPT_ALL_SHIFT: c_int = 1;

pub const CLIENT_INIT_TX_DATA_BCAST_ACCEPT_ALL_SHIFT: c_int = 2;

pub const CLIENT_INIT_TX_DATA_ACCEPT_ANY_VLAN_SHIFT: c_int = 3;

pub const CLIENT_INIT_TX_DATA_RESERVED0_SHIFT: c_int = 4;
    pub default_vlan_flg: u8,
    pub force_default_pri_flg: u8,
    pub tunnel_lso_inc_ip_id: u8,
    pub refuse_outband_vlan_flg: u8,
    pub tunnel_non_lso_pcsum_location: u8,
    pub tunnel_non_lso_outer_ip_csum_location: u8,
}

//
// client init ramrod data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_init_ramrod_data {
    pub general: client_init_general_data,
    pub rx: client_init_rx_data,
    pub tx: client_init_tx_data,
}

//
// client update ramrod data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_update_ramrod_data {
    pub client_id: u8,
    pub func_id: u8,
    pub inner_vlan_removal_enable_flg: u8,
    pub inner_vlan_removal_change_flg: u8,
    pub outer_vlan_removal_enable_flg: u8,
    pub outer_vlan_removal_change_flg: u8,
    pub anti_spoofing_enable_flg: u8,
    pub anti_spoofing_change_flg: u8,
    pub activate_flg: u8,
    pub activate_change_flg: u8,
    pub default_vlan: __le16,
    pub default_vlan_enable_flg: u8,
    pub default_vlan_change_flg: u8,
    pub silent_vlan_value: __le16,
    pub silent_vlan_mask: __le16,
    pub silent_vlan_removal_flg: u8,
    pub silent_vlan_change_flg: u8,
    pub refuse_outband_vlan_flg: u8,
    pub refuse_outband_vlan_change_flg: u8,
    pub tx_switching_flg: u8,
    pub tx_switching_change_flg: u8,
    pub handle_ptp_pkts_flg: u8,
    pub handle_ptp_pkts_change_flg: u8,
    pub reserved1: __le16,
    pub echo: __le32,
}

//
// The eth storm context of Cstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstorm_eth_st_context {
    pub __reserved0: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct double_regpair {
    pub regpair0_lo: u32,
    pub regpair0_hi: u32,
    pub regpair1_lo: u32,
    pub regpair1_hi: u32,
}

// 2nd parse bd type used in ethernet tx BDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_2nd_parse_bd_type {
    ETH_2ND_PARSE_BD_TYPE_LSO_TUNNEL,
    MAX_ETH_2ND_PARSE_BD_TYPE
}

//
// Ethernet address typesm used in ethernet tx BDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_addr_type {
    UNKNOWN_ADDRESS,
    UNICAST_ADDRESS,
    MULTICAST_ADDRESS,
    BROADCAST_ADDRESS,
    MAX_ETH_ADDR_TYPE
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_cmd_header {
    pub cmd_general_data: u8,

pub const ETH_CLASSIFY_CMD_HEADER_RX_CMD_SHIFT: c_int = 0;

pub const ETH_CLASSIFY_CMD_HEADER_TX_CMD_SHIFT: c_int = 1;

pub const ETH_CLASSIFY_CMD_HEADER_OPCODE_SHIFT: c_int = 2;

pub const ETH_CLASSIFY_CMD_HEADER_IS_ADD_SHIFT: c_int = 4;

pub const ETH_CLASSIFY_CMD_HEADER_RESERVED0_SHIFT: c_int = 5;
    pub func_id: u8,
    pub client_id: u8,
    pub reserved1: u8,
}

//
// header for eth classification config ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_header {
    pub rule_cnt: u8,
    pub warning_on_error: u8,
    pub reserved1: __le16,
    pub echo: __le32,
}

//
// Command for adding/removing a Inner-MAC/VNI classification rule
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_imac_vni_cmd {
    pub header: eth_classify_cmd_header,
    pub vni: __le32,
    pub imac_lsb: __le16,
    pub imac_mid: __le16,
    pub imac_msb: __le16,
    pub reserved1: __le16,
}

//
// Command for adding/removing a MAC classification rule
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_mac_cmd {
    pub header: eth_classify_cmd_header,
    pub reserved0: __le16,
    pub inner_mac: __le16,
    pub mac_lsb: __le16,
    pub mac_mid: __le16,
    pub mac_msb: __le16,
    pub reserved1: __le16,
}

//
// Command for adding/removing a MAC-VLAN pair classification rule
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_pair_cmd {
    pub header: eth_classify_cmd_header,
    pub reserved0: __le16,
    pub inner_mac: __le16,
    pub mac_lsb: __le16,
    pub mac_mid: __le16,
    pub mac_msb: __le16,
    pub vlan: __le16,
}

//
// Command for adding/removing a VLAN classification rule
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_vlan_cmd {
    pub header: eth_classify_cmd_header,
    pub reserved0: __le32,
    pub reserved1: __le32,
    pub reserved2: __le16,
    pub vlan: __le16,
}

//
// Command for adding/removing a VXLAN classification rule
//
// union for eth classification rule
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_classify_rule_cmd {
    pub mac: eth_classify_mac_cmd,
    pub vlan: eth_classify_vlan_cmd,
    pub pair: eth_classify_pair_cmd,
    pub imac_vni: eth_classify_imac_vni_cmd,
}

//
// parameters for eth classification configuration ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_classify_rules_ramrod_data {
    pub header: eth_classify_header,
    pub rules: [eth_classify_rule_cmd; CLASSIFY_RULES_COUNT],
}

//
// The data contain client ID need to the ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_common_ramrod_data {
    pub client_id: __le32,
    pub reserved1: __le32,
}

//
// The eth storm context of Ustorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_eth_st_context {
    pub reserved0: [u32; 52],
}

//
// The eth storm context of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_st_context {
    pub __reserved0: [u32; 28],
}

//
// The eth storm context of Xstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_st_context {
    pub reserved0: [u32; 60],
}

//
// Ethernet connection context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_context {
    pub ustorm_st_context: ustorm_eth_st_context,
    pub tstorm_st_context: tstorm_eth_st_context,
    pub xstorm_ag_context: xstorm_eth_ag_context,
    pub tstorm_ag_context: tstorm_eth_ag_context,
    pub cstorm_ag_context: cstorm_eth_ag_context,
    pub ustorm_ag_context: ustorm_eth_ag_context,
    pub timers_context: timers_block_context,
    pub xstorm_st_context: xstorm_eth_st_context,
    pub cstorm_st_context: cstorm_eth_st_context,
}

//
// union for sgl and raw data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_sgl_or_raw_data {
    pub sgl: [__le16; 8],
    pub raw_data: [u32; 4],
}

//
// eth FP end aggregation CQE parameters struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_end_agg_rx_cqe {
    pub type_error_flags: u8,

pub const ETH_END_AGG_RX_CQE_TYPE_SHIFT: c_int = 0;

pub const ETH_END_AGG_RX_CQE_SGL_RAW_SEL_SHIFT: c_int = 2;

pub const ETH_END_AGG_RX_CQE_RESERVED0_SHIFT: c_int = 3;
    pub reserved1: u8,
    pub queue_index: u8,
    pub reserved2: u8,
    pub timestamp_delta: __le32,
    pub num_of_coalesced_segs: __le16,
    pub pkt_len: __le16,
    pub pure_ack_count: u8,
    pub reserved3: u8,
    pub reserved4: __le16,
    pub sgl_or_raw_data: eth_sgl_or_raw_data,
    pub reserved5: [__le32; 8],
}

//
// regular eth FP CQE parameters struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_fast_path_rx_cqe {
    pub type_error_flags: u8,

pub const ETH_FAST_PATH_RX_CQE_TYPE_SHIFT: c_int = 0;

pub const ETH_FAST_PATH_RX_CQE_SGL_RAW_SEL_SHIFT: c_int = 2;

pub const ETH_FAST_PATH_RX_CQE_PHY_DECODE_ERR_FLG_SHIFT: c_int = 3;

pub const ETH_FAST_PATH_RX_CQE_IP_BAD_XSUM_FLG_SHIFT: c_int = 4;

pub const ETH_FAST_PATH_RX_CQE_L4_BAD_XSUM_FLG_SHIFT: c_int = 5;

pub const ETH_FAST_PATH_RX_CQE_PTP_PKT_SHIFT: c_int = 6;

pub const ETH_FAST_PATH_RX_CQE_RESERVED0_SHIFT: c_int = 7;
    pub status_flags: u8,

pub const ETH_FAST_PATH_RX_CQE_RSS_HASH_TYPE_SHIFT: c_int = 0;

pub const ETH_FAST_PATH_RX_CQE_RSS_HASH_FLG_SHIFT: c_int = 3;

pub const ETH_FAST_PATH_RX_CQE_BROADCAST_FLG_SHIFT: c_int = 4;

pub const ETH_FAST_PATH_RX_CQE_MAC_MATCH_FLG_SHIFT: c_int = 5;

pub const ETH_FAST_PATH_RX_CQE_IP_XSUM_NO_VALIDATION_FLG_SHIFT: c_int = 6;

pub const ETH_FAST_PATH_RX_CQE_L4_XSUM_NO_VALIDATION_FLG_SHIFT: c_int = 7;
    pub queue_index: u8,
    pub placement_offset: u8,
    pub rss_hash_result: __le32,
    pub vlan_tag: __le16,
    pub pkt_len_or_gro_seg_len: __le16,
    pub len_on_bd: __le16,
    pub pars_flags: parsing_flags,
    pub sgl_or_raw_data: eth_sgl_or_raw_data,
    pub tunn_type: u8,
    pub tunn_inner_hdrs_offset: u8,
    pub reserved1: __le16,
    pub tunn_tenant_id: __le32,
    pub padding: [__le32; 5],
    pub marker: u32,
}

//
// Command for setting classification flags for a client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_filter_rules_cmd {
    pub cmd_general_data: u8,

pub const ETH_FILTER_RULES_CMD_RX_CMD_SHIFT: c_int = 0;

pub const ETH_FILTER_RULES_CMD_TX_CMD_SHIFT: c_int = 1;

pub const ETH_FILTER_RULES_CMD_RESERVED0_SHIFT: c_int = 2;
    pub func_id: u8,
    pub client_id: u8,
    pub reserved1: u8,
    pub state: __le16,

pub const ETH_FILTER_RULES_CMD_UCAST_DROP_ALL_SHIFT: c_int = 0;

pub const ETH_FILTER_RULES_CMD_UCAST_ACCEPT_ALL_SHIFT: c_int = 1;

pub const ETH_FILTER_RULES_CMD_UCAST_ACCEPT_UNMATCHED_SHIFT: c_int = 2;

pub const ETH_FILTER_RULES_CMD_MCAST_DROP_ALL_SHIFT: c_int = 3;

pub const ETH_FILTER_RULES_CMD_MCAST_ACCEPT_ALL_SHIFT: c_int = 4;

pub const ETH_FILTER_RULES_CMD_BCAST_ACCEPT_ALL_SHIFT: c_int = 5;

pub const ETH_FILTER_RULES_CMD_ACCEPT_ANY_VLAN_SHIFT: c_int = 6;

pub const ETH_FILTER_RULES_CMD_RESERVED2_SHIFT: c_int = 7;
    pub reserved3: __le16,
    pub reserved4: regpair,
}

//
// parameters for eth classification filters ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_filter_rules_ramrod_data {
    pub header: eth_classify_header,
    pub rules: [eth_filter_rules_cmd; FILTER_RULES_COUNT],
}

// Hsi version
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_fp_hsi_ver {
    ETH_FP_HSI_VER_0,
    ETH_FP_HSI_VER_1,
    ETH_FP_HSI_VER_2,
    MAX_ETH_FP_HSI_VER
}

//
// parameters for eth classification configuration ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_general_rules_ramrod_data {
    pub header: eth_classify_header,
    pub rules: [eth_classify_rule_cmd; CLASSIFY_RULES_COUNT],
}

//
// The data for Halt ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_halt_ramrod_data {
    pub client_id: __le32,
    pub reserved0: __le32,
}

//
// destination and source mac address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_mac_addresses {

    pub dst_mid: __le16,
    pub dst_lo: __le16,

    pub dst_lo: __le16,
    pub dst_mid: __le16,

    pub src_lo: __le16,
    pub dst_hi: __le16,

    pub dst_hi: __le16,
    pub src_lo: __le16,

    pub src_hi: __le16,
    pub src_mid: __le16,

    pub src_mid: __le16,
    pub src_hi: __le16,

}

// tunneling related data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tunnel_data {
    pub dst_lo: __le16,
    pub dst_mid: __le16,
    pub dst_hi: __le16,
    pub fw_ip_hdr_csum: __le16,
    pub pseudo_csum: __le16,
    pub ip_hdr_start_inner_w: u8,
    pub flags: u8,

pub const ETH_TUNNEL_DATA_IPV6_OUTER_SHIFT: c_int = 0;

pub const ETH_TUNNEL_DATA_RESERVED_SHIFT: c_int = 1;
}

// union for mac addresses and for tunneling data.
// considered as tunneling data only if (tunnel_exist == 1).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_mac_addr_or_tunnel_data {
    pub mac_addr: eth_mac_addresses,
    pub tunnel_data: eth_tunnel_data,
}

// Command for setting multicast classification for a client
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_multicast_rules_cmd {
    pub cmd_general_data: u8,

pub const ETH_MULTICAST_RULES_CMD_RX_CMD_SHIFT: c_int = 0;

pub const ETH_MULTICAST_RULES_CMD_TX_CMD_SHIFT: c_int = 1;

pub const ETH_MULTICAST_RULES_CMD_IS_ADD_SHIFT: c_int = 2;

pub const ETH_MULTICAST_RULES_CMD_RESERVED0_SHIFT: c_int = 3;
    pub func_id: u8,
    pub bin_id: u8,
    pub engine_id: u8,
    pub reserved2: __le32,
    pub reserved3: regpair,
}

//
// parameters for multicast classification ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_multicast_rules_ramrod_data {
    pub header: eth_classify_header,
    pub rules: [eth_multicast_rules_cmd; MULTICAST_RULES_COUNT],
}

//
// Place holder for ramrods protocol specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ramrod_data {
    pub data_lo: __le32,
    pub data_hi: __le32,
}

//
// union for ramrod data for Ethernet protocol (CQE) (force size of 16 bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_ramrod_data {
    pub general: ramrod_data,
}

//
// RSS toeplitz hash type, as reported in CQE
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_rss_hash_type {
    DEFAULT_HASH_TYPE,
    IPV4_HASH_TYPE,
    TCP_IPV4_HASH_TYPE,
    IPV6_HASH_TYPE,
    TCP_IPV6_HASH_TYPE,
    VLAN_PRI_HASH_TYPE,
    E1HOV_PRI_HASH_TYPE,
    DSCP_HASH_TYPE,
    MAX_ETH_RSS_HASH_TYPE
}

//
// Ethernet RSS mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_rss_mode {
    ETH_RSS_MODE_DISABLED,
    ETH_RSS_MODE_REGULAR,
    ETH_RSS_MODE_VLAN_PRI,
    ETH_RSS_MODE_E1HOV_PRI,
    ETH_RSS_MODE_IP_DSCP,
    MAX_ETH_RSS_MODE
}

//
// parameters for RSS update ramrod (E2)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rss_update_ramrod_data {
    pub rss_engine_id: u8,
    pub rss_mode: u8,
    pub capabilities: __le16,

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV4_CAPABILITY_SHIFT: c_int = 0;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV4_TCP_CAPABILITY_SHIFT: c_int = 1;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV4_UDP_CAPABILITY_SHIFT: c_int = 2;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV4_VXLAN_CAPABILITY_SHIFT: c_int = 3;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV6_CAPABILITY_SHIFT: c_int = 4;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV6_TCP_CAPABILITY_SHIFT: c_int = 5;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV6_UDP_CAPABILITY_SHIFT: c_int = 6;

pub const ETH_RSS_UPDATE_RAMROD_DATA_IPV6_VXLAN_CAPABILITY_SHIFT: c_int = 7;

pub const ETH_RSS_UPDATE_RAMROD_DATA_TUNN_INNER_HDRS_CAPABILITY_SHIFT: c_int = 8;

pub const ETH_RSS_UPDATE_RAMROD_DATA_UPDATE_RSS_KEY_SHIFT: c_int = 9;

pub const ETH_RSS_UPDATE_RAMROD_DATA_RESERVED_SHIFT: c_int = 10;
    pub rss_result_mask: u8,
    pub reserved3: u8,
    pub reserved4: __le16,
    pub indirection_table: [u8; T_ETH_INDIRECTION_TABLE_SIZE],
    pub rss_key: [__le32; T_ETH_RSS_KEY],
    pub echo: __le32,
    pub reserved5: __le32,
}

//
// The eth Rx Buffer Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_bd {
    pub addr_lo: __le32,
    pub addr_hi: __le32,
}

//
// Eth Rx Cqe structure- general structure for ramrods
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_ramrod_eth_rx_cqe {
    pub ramrod_type: u8,

pub const COMMON_RAMROD_ETH_RX_CQE_TYPE_SHIFT: c_int = 0;

pub const COMMON_RAMROD_ETH_RX_CQE_ERROR_SHIFT: c_int = 2;

pub const COMMON_RAMROD_ETH_RX_CQE_RESERVED0_SHIFT: c_int = 3;
    pub conn_type: u8,
    pub reserved1: __le16,
    pub conn_and_cmd_data: __le32,

pub const COMMON_RAMROD_ETH_RX_CQE_CID_SHIFT: c_int = 0;

pub const COMMON_RAMROD_ETH_RX_CQE_CMD_ID_SHIFT: c_int = 24;
    pub protocol_data: ramrod_data,
    pub echo: __le32,
    pub reserved2: [__le32; 11],
}

//
// Rx Last CQE in page (in ETH)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_cqe_next_page {
    pub addr_lo: __le32,
    pub addr_hi: __le32,
    pub reserved: [__le32; 14],
}

//
// union for all eth rx cqe types (fix their sizes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_rx_cqe {
    pub fast_path_cqe: eth_fast_path_rx_cqe,
    pub ramrod_cqe: common_ramrod_eth_rx_cqe,
    pub next_page_cqe: eth_rx_cqe_next_page,
    pub end_agg_cqe: eth_end_agg_rx_cqe,
}

//
// Values for RX ETH CQE type field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_rx_cqe_type {
    RX_ETH_CQE_TYPE_ETH_FASTPATH,
    RX_ETH_CQE_TYPE_ETH_RAMROD,
    RX_ETH_CQE_TYPE_ETH_START_AGG,
    RX_ETH_CQE_TYPE_ETH_STOP_AGG,
    MAX_ETH_RX_CQE_TYPE
}

//
// Type of SGL/Raw field in ETH RX fast path CQE
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_rx_fp_sel {
    ETH_FP_CQE_REGULAR,
    ETH_FP_CQE_RAW,
    MAX_ETH_RX_FP_SEL
}

//
// The eth Rx SGE Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_sge {
    pub addr_lo: __le32,
    pub addr_hi: __le32,
}

//
// common data for all protocols
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spe_hdr {
    pub conn_and_cmd_data: __le32,

pub const SPE_HDR_CID_SHIFT: c_int = 0;

pub const SPE_HDR_CMD_ID_SHIFT: c_int = 24;
    pub type: __le16,

pub const SPE_HDR_CONN_TYPE_SHIFT: c_int = 0;

pub const SPE_HDR_FUNCTION_ID_SHIFT: c_int = 8;
    pub reserved1: __le16,
}

//
// specific data for ethernet slow path element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_specific_data {
    pub protocol_data: [u8; 8],
    pub client_update_ramrod_data: regpair,
    pub client_init_ramrod_init_data: regpair,
    pub halt_ramrod_data: eth_halt_ramrod_data,
    pub update_data_addr: regpair,
    pub common_ramrod_data: eth_common_ramrod_data,
    pub classify_cfg_addr: regpair,
    pub filter_cfg_addr: regpair,
    pub mcast_cfg_addr: regpair,
}

//
// Ethernet slow path element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_spe {
    pub hdr: spe_hdr,
    pub data: eth_specific_data,
}

//
// Ethernet command ID for slow path elements
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_spqe_cmd_id {
    RAMROD_CMD_ID_ETH_UNUSED,
    RAMROD_CMD_ID_ETH_CLIENT_SETUP,
    RAMROD_CMD_ID_ETH_HALT,
    RAMROD_CMD_ID_ETH_FORWARD_SETUP,
    RAMROD_CMD_ID_ETH_TX_QUEUE_SETUP,
    RAMROD_CMD_ID_ETH_CLIENT_UPDATE,
    RAMROD_CMD_ID_ETH_EMPTY,
    RAMROD_CMD_ID_ETH_TERMINATE,
    RAMROD_CMD_ID_ETH_TPA_UPDATE,
    RAMROD_CMD_ID_ETH_CLASSIFICATION_RULES,
    RAMROD_CMD_ID_ETH_FILTER_RULES,
    RAMROD_CMD_ID_ETH_MULTICAST_RULES,
    RAMROD_CMD_ID_ETH_RSS_UPDATE,
    RAMROD_CMD_ID_ETH_SET_MAC,
    MAX_ETH_SPQE_CMD_ID
}

//
// eth tpa update command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tpa_update_command {
    TPA_UPDATE_NONE_COMMAND,
    TPA_UPDATE_ENABLE_COMMAND,
    TPA_UPDATE_DISABLE_COMMAND,
    MAX_ETH_TPA_UPDATE_COMMAND
}

// In case of LSO over IPv4 tunnel, whether to increment
// IP ID on external IP header or internal IP header
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tunnel_lso_inc_ip_id {
    EXT_HEADER,
    INT_HEADER,
    MAX_ETH_TUNNEL_LSO_INC_IP_ID
}

// In case tunnel exist and L4 checksum offload,
// the pseudo checksum location, on packet or on BD.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tunnel_non_lso_csum_location {
    CSUM_ON_PKT,
    CSUM_ON_BD,
    MAX_ETH_TUNNEL_NON_LSO_CSUM_LOCATION
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tunn_type {
    TUNN_TYPE_NONE,
    TUNN_TYPE_VXLAN,
    TUNN_TYPE_L2_GRE,
    TUNN_TYPE_IPV4_GRE,
    TUNN_TYPE_IPV6_GRE,
    TUNN_TYPE_L2_GENEVE,
    TUNN_TYPE_IPV4_GENEVE,
    TUNN_TYPE_IPV6_GENEVE,
    MAX_ETH_TUNN_TYPE
}

//
// Tx regular BD structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_bd {
    pub addr_lo: __le32,
    pub addr_hi: __le32,
    pub total_pkt_bytes: __le16,
    pub nbytes: __le16,
    pub reserved: [u8; 4],
}

//
// structure for easy accessibility to assembler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_bd_flags {
    pub as_bitfield: u8,

pub const ETH_TX_BD_FLAGS_IP_CSUM_SHIFT: c_int = 0;

pub const ETH_TX_BD_FLAGS_L4_CSUM_SHIFT: c_int = 1;

pub const ETH_TX_BD_FLAGS_VLAN_MODE_SHIFT: c_int = 2;

pub const ETH_TX_BD_FLAGS_START_BD_SHIFT: c_int = 4;

pub const ETH_TX_BD_FLAGS_IS_UDP_SHIFT: c_int = 5;

pub const ETH_TX_BD_FLAGS_SW_LSO_SHIFT: c_int = 6;

pub const ETH_TX_BD_FLAGS_IPV6_SHIFT: c_int = 7;
}

//
// The eth Tx Buffer Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_start_bd {
    pub addr_lo: __le32,
    pub addr_hi: __le32,
    pub nbd: __le16,
    pub nbytes: __le16,
    pub vlan_or_ethertype: __le16,
    pub bd_flags: eth_tx_bd_flags,
    pub general_data: u8,

pub const ETH_TX_START_BD_HDR_NBDS_SHIFT: c_int = 0;

pub const ETH_TX_START_BD_NO_ADDED_TAGS_SHIFT: c_int = 3;

pub const ETH_TX_START_BD_FORCE_VLAN_MODE_SHIFT: c_int = 4;

pub const ETH_TX_START_BD_PARSE_NBDS_SHIFT: c_int = 5;

pub const ETH_TX_START_BD_TUNNEL_EXIST_SHIFT: c_int = 7;
}

//
// Tx parsing BD structure for ETH E1/E1h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_parse_bd_e1x {
    pub global_data: __le16,

pub const ETH_TX_PARSE_BD_E1X_IP_HDR_START_OFFSET_W_SHIFT: c_int = 0;

pub const ETH_TX_PARSE_BD_E1X_ETH_ADDR_TYPE_SHIFT: c_int = 4;

pub const ETH_TX_PARSE_BD_E1X_PSEUDO_CS_WITHOUT_LEN_SHIFT: c_int = 6;

pub const ETH_TX_PARSE_BD_E1X_LLC_SNAP_EN_SHIFT: c_int = 7;

pub const ETH_TX_PARSE_BD_E1X_NS_FLG_SHIFT: c_int = 8;

pub const ETH_TX_PARSE_BD_E1X_RESERVED0_SHIFT: c_int = 9;
    pub tcp_flags: u8,

pub const ETH_TX_PARSE_BD_E1X_FIN_FLG_SHIFT: c_int = 0;

pub const ETH_TX_PARSE_BD_E1X_SYN_FLG_SHIFT: c_int = 1;

pub const ETH_TX_PARSE_BD_E1X_RST_FLG_SHIFT: c_int = 2;

pub const ETH_TX_PARSE_BD_E1X_PSH_FLG_SHIFT: c_int = 3;

pub const ETH_TX_PARSE_BD_E1X_ACK_FLG_SHIFT: c_int = 4;

pub const ETH_TX_PARSE_BD_E1X_URG_FLG_SHIFT: c_int = 5;

pub const ETH_TX_PARSE_BD_E1X_ECE_FLG_SHIFT: c_int = 6;

pub const ETH_TX_PARSE_BD_E1X_CWR_FLG_SHIFT: c_int = 7;
    pub ip_hlen_w: u8,
    pub total_hlen_w: __le16,
    pub tcp_pseudo_csum: __le16,
    pub lso_mss: __le16,
    pub ip_id: __le16,
    pub tcp_send_seq: __le32,
}

//
// Tx parsing BD structure for ETH E2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_parse_bd_e2 {
    pub data: eth_mac_addr_or_tunnel_data,
    pub parsing_data: __le32,

pub const ETH_TX_PARSE_BD_E2_L4_HDR_START_OFFSET_W_SHIFT: c_int = 0;

pub const ETH_TX_PARSE_BD_E2_TCP_HDR_LENGTH_DW_SHIFT: c_int = 11;

pub const ETH_TX_PARSE_BD_E2_IPV6_WITH_EXT_HDR_SHIFT: c_int = 15;

pub const ETH_TX_PARSE_BD_E2_LSO_MSS_SHIFT: c_int = 16;

pub const ETH_TX_PARSE_BD_E2_ETH_ADDR_TYPE_SHIFT: c_int = 30;
}

//
// Tx 2nd parsing BD structure for ETH packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_parse_2nd_bd {
    pub global_data: __le16,

pub const ETH_TX_PARSE_2ND_BD_IP_HDR_START_OUTER_W_SHIFT: c_int = 0;

pub const ETH_TX_PARSE_2ND_BD_RESERVED0_SHIFT: c_int = 4;

pub const ETH_TX_PARSE_2ND_BD_LLC_SNAP_EN_SHIFT: c_int = 5;

pub const ETH_TX_PARSE_2ND_BD_NS_FLG_SHIFT: c_int = 6;

pub const ETH_TX_PARSE_2ND_BD_TUNNEL_UDP_EXIST_SHIFT: c_int = 7;

pub const ETH_TX_PARSE_2ND_BD_IP_HDR_LEN_OUTER_W_SHIFT: c_int = 8;

pub const ETH_TX_PARSE_2ND_BD_RESERVED1_SHIFT: c_int = 13;
    pub bd_type: u8,

pub const ETH_TX_PARSE_2ND_BD_TYPE_SHIFT: c_int = 0;

pub const ETH_TX_PARSE_2ND_BD_RESERVED2_SHIFT: c_int = 4;
    pub reserved3: u8,
    pub tcp_flags: u8,

pub const ETH_TX_PARSE_2ND_BD_FIN_FLG_SHIFT: c_int = 0;

pub const ETH_TX_PARSE_2ND_BD_SYN_FLG_SHIFT: c_int = 1;

pub const ETH_TX_PARSE_2ND_BD_RST_FLG_SHIFT: c_int = 2;

pub const ETH_TX_PARSE_2ND_BD_PSH_FLG_SHIFT: c_int = 3;

pub const ETH_TX_PARSE_2ND_BD_ACK_FLG_SHIFT: c_int = 4;

pub const ETH_TX_PARSE_2ND_BD_URG_FLG_SHIFT: c_int = 5;

pub const ETH_TX_PARSE_2ND_BD_ECE_FLG_SHIFT: c_int = 6;

pub const ETH_TX_PARSE_2ND_BD_CWR_FLG_SHIFT: c_int = 7;
    pub reserved4: u8,
    pub tunnel_udp_hdr_start_w: u8,
    pub fw_ip_hdr_to_payload_w: u8,
    pub fw_ip_csum_wo_len_flags_frag: __le16,
    pub hw_ip_id: __le16,
    pub tcp_send_seq: __le32,
}

// The last BD in the BD memory will hold a pointer to the next BD memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_next_bd {
    pub addr_lo: __le32,
    pub addr_hi: __le32,
    pub reserved: [u8; 8],
}

//
// union for 4 Bd types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_tx_bd_types {
    pub start_bd: eth_tx_start_bd,
    pub reg_bd: eth_tx_bd,
    pub parse_bd_e1x: eth_tx_parse_bd_e1x,
    pub parse_bd_e2: eth_tx_parse_bd_e2,
    pub parse_2nd_bd: eth_tx_parse_2nd_bd,
    pub next_bd: eth_tx_next_bd,
}

//
// array of 13 bds as appears in the eth xstorm context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_bds_array {
    pub bds: [eth_tx_bd_types; 13],
}

//
// VLAN mode on TX BDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tx_vlan_type {
    X_ETH_NO_VLAN,
    X_ETH_OUTBAND_VLAN,
    X_ETH_INBAND_VLAN,
    X_ETH_FW_ADDED_VLAN,
    MAX_ETH_TX_VLAN_TYPE
}

//
// Ethernet VLAN filtering mode in E1x
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_vlan_filter_mode {
    ETH_VLAN_FILTER_ANY_VLAN,
    ETH_VLAN_FILTER_SPECIFIC_VLAN,
    ETH_VLAN_FILTER_CLASSIFY,
    MAX_ETH_VLAN_FILTER_MODE
}

//
// MAC filtering configuration command header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_configuration_hdr {
    pub length: u8,
    pub offset: u8,
    pub client_id: __le16,
    pub echo: __le32,
}

//
// MAC address in list for ramrod
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_configuration_entry {
    pub lsb_mac_addr: __le16,
    pub middle_mac_addr: __le16,
    pub msb_mac_addr: __le16,
    pub vlan_id: __le16,
    pub pf_id: u8,
    pub flags: u8,

pub const MAC_CONFIGURATION_ENTRY_ACTION_TYPE_SHIFT: c_int = 0;

pub const MAC_CONFIGURATION_ENTRY_RDMA_MAC_SHIFT: c_int = 1;

pub const MAC_CONFIGURATION_ENTRY_VLAN_FILTERING_MODE_SHIFT: c_int = 2;

pub const MAC_CONFIGURATION_ENTRY_OVERRIDE_VLAN_REMOVAL_SHIFT: c_int = 4;

pub const MAC_CONFIGURATION_ENTRY_BROADCAST_SHIFT: c_int = 5;

pub const MAC_CONFIGURATION_ENTRY_RESERVED1_SHIFT: c_int = 6;
    pub reserved0: __le16,
    pub clients_bit_vector: __le32,
}

//
// MAC filtering configuration command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_configuration_cmd {
    pub hdr: mac_configuration_hdr,
    pub config_table: [mac_configuration_entry; 64],
}

//
// Set-MAC command type (in E1x)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum set_mac_action_type {
    T_ETH_MAC_COMMAND_INVALIDATE,
    T_ETH_MAC_COMMAND_SET,
    MAX_SET_MAC_ACTION_TYPE
}

//
// Ethernet TPA Modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpa_mode {
    TPA_LRO,
    TPA_GRO,
    MAX_TPA_MODE};


//
// tpa update ramrod data
//
    struct tpa_update_ramrod_data {
    u8 update_ipv4;
    u8 update_ipv6;
    u8 client_id;
    u8 max_tpa_queues;
    u8 max_sges_for_packet;
    u8 complete_on_both_clients;
    u8 dont_verify_rings_pause_thr_flg;
    u8 tpa_mode;
    __le16 sge_buff_size;
    __le16 max_agg_size;
    __le32 sge_page_base_lo;
    __le32 sge_page_base_hi;
    __le16 sge_pause_thr_low;
    __le16 sge_pause_thr_high;
    u8 tpa_over_vlan_disable;
    u8 reserved[7];
}

//
// approximate-match multicast filtering for E1H per function in Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_approximate_match_multicast_filtering {
    pub mcast_add_hash_bit_array: [u32; 8],
}

//
// Common configuration parameters per function in Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_function_common_config {
    pub config_flags: __le16,

pub const TSTORM_ETH_FUNCTION_COMMON_CONFIG_RSS_IPV4_CAPABILITY_SHIFT: c_int = 0;

pub const TSTORM_ETH_FUNCTION_COMMON_CONFIG_RSS_IPV4_TCP_CAPABILITY_SHIFT: c_int = 1;

pub const TSTORM_ETH_FUNCTION_COMMON_CONFIG_RSS_IPV6_CAPABILITY_SHIFT: c_int = 2;

pub const TSTORM_ETH_FUNCTION_COMMON_CONFIG_RSS_IPV6_TCP_CAPABILITY_SHIFT: c_int = 3;

pub const TSTORM_ETH_FUNCTION_COMMON_CONFIG_RSS_MODE_SHIFT: c_int = 4;

pub const TSTORM_ETH_FUNCTION_COMMON_CONFIG_VLAN_FILTERING_ENABLE_SHIFT: c_int = 7;

pub const __TSTORM_ETH_FUNCTION_COMMON_CONFIG_RESERVED0_SHIFT: c_int = 8;
    pub rss_result_mask: u8,
    pub reserved1: u8,
    pub vlan_id: [__le16; 2],
}

//
// MAC filtering configuration parameters per port in Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_mac_filter_config {
    pub ucast_drop_all: u32,
    pub ucast_accept_all: u32,
    pub mcast_drop_all: u32,
    pub mcast_accept_all: u32,
    pub bcast_accept_all: u32,
    pub vlan_filter: [u32; 2],
    pub unmatched_unicast: u32,
}

//
// tx only queue init ramrod data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_queue_init_ramrod_data {
    pub general: client_init_general_data,
    pub tx: client_init_tx_data,
}

//
// Three RX producers for ETH
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_eth_rx_producers {

    pub bd_prod: u16,
    pub cqe_prod: u16,

    pub cqe_prod: u16,
    pub bd_prod: u16,

    pub reserved: u16,
    pub sge_prod: u16,

    pub sge_prod: u16,
    pub reserved: u16,

}

//
// FCoE RX statistics parameters section#0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat_params_section0 {
    pub fcoe_rx_pkt_cnt: __le32,
    pub fcoe_rx_byte_cnt: __le32,
}

//
// FCoE RX statistics parameters section#1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat_params_section1 {
    pub fcoe_ver_cnt: __le32,
    pub fcoe_rx_drop_pkt_cnt: __le32,
}

//
// FCoE RX statistics parameters section#2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat_params_section2 {
    pub fc_crc_cnt: __le32,
    pub eofa_del_cnt: __le32,
    pub miss_frame_cnt: __le32,
    pub seq_timeout_cnt: __le32,
    pub drop_seq_cnt: __le32,
    pub fcoe_rx_drop_pkt_cnt: __le32,
    pub fcp_rx_pkt_cnt: __le32,
    pub reserved0: __le32,
}

//
// FCoE TX statistics parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tx_stat_params {
    pub fcoe_tx_pkt_cnt: __le32,
    pub fcoe_tx_byte_cnt: __le32,
    pub fcp_tx_pkt_cnt: __le32,
    pub reserved0: __le32,
}

//
// FCoE statistics parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_statistics_params {
    pub tx_stat: fcoe_tx_stat_params,
    pub rx_stat0: fcoe_rx_stat_params_section0,
    pub rx_stat1: fcoe_rx_stat_params_section1,
    pub rx_stat2: fcoe_rx_stat_params_section2,
}

//
// The data afex vif list ramrod need
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afex_vif_list_ramrod_data {
    pub afex_vif_list_command: u8,
    pub func_bit_map: u8,
    pub vif_list_index: __le16,
    pub func_to_clear: u8,
    pub echo: u8,
    pub reserved1: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2s_pri_trans_table_entry {
    pub val: [u8; MAX_VLAN_PRIORITIES],
}

//
// cfc delete event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfc_del_event_data {
    pub cid: __le32,
    pub reserved0: __le32,
    pub reserved1: __le32,
}

//
// per-port SAFC demo variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmng_flags_per_port {
    pub cmng_enables: u32,

pub const CMNG_FLAGS_PER_PORT_FAIRNESS_VN_SHIFT: c_int = 0;

pub const CMNG_FLAGS_PER_PORT_RATE_SHAPING_VN_SHIFT: c_int = 1;

pub const CMNG_FLAGS_PER_PORT_FAIRNESS_COS_SHIFT: c_int = 2;

pub const CMNG_FLAGS_PER_PORT_FAIRNESS_COS_MODE_SHIFT: c_int = 3;

pub const __CMNG_FLAGS_PER_PORT_RESERVED0_SHIFT: c_int = 4;
    pub __reserved1: u32,
}

//
// per-port rate shaping variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_shaping_vars_per_port {
    pub rs_periodic_timeout: u32,
    pub rs_threshold: u32,
}

//
// per-port fairness variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fairness_vars_per_port {
    pub upper_bound: u32,
    pub fair_threshold: u32,
    pub fairness_timeout: u32,
    pub size_thr: u32,
}

//
// per-port SAFC variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safc_struct_per_port {

    pub __reserved1: u16,
    pub __reserved0: u8,
    pub safc_timeout_usec: u8,

    pub safc_timeout_usec: u8,
    pub __reserved0: u8,
    pub __reserved1: u16,
    pub cos_to_traffic_types: [u8; MAX_COS_NUMBER],
    pub cos_to_pause_mask: [u16; NUM_OF_SAFC_BITS],
}

//
// Per-port congestion management variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmng_struct_per_port {
    pub rs_vars: rate_shaping_vars_per_port,
    pub fair_vars: fairness_vars_per_port,
    pub safc_vars: safc_struct_per_port,
    pub flags: cmng_flags_per_port,
}

//
// a single rate shaping counter. can be used as protocol or vnic counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_shaping_counter {
    pub quota: u32,

    pub __reserved0: u16,
    pub rate: u16,

    pub rate: u16,
    pub __reserved0: u16,

}

//
// per-vnic rate shaping variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_shaping_vars_per_vn {
    pub vn_counter: rate_shaping_counter,
}

//
// per-vnic fairness variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fairness_vars_per_vn {
    pub cos_credit_delta: [u32; MAX_COS_NUMBER],
    pub vn_credit_delta: u32,
    pub __reserved0: u32,
}

//
// cmng port init state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmng_vnic {
    pub vnic_max_rate: [rate_shaping_vars_per_vn; 4],
    pub vnic_min_rate: [fairness_vars_per_vn; 4],
}

//
// cmng port init state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmng_init {
    pub port: cmng_struct_per_port,
    pub vnic: cmng_vnic,
}

//
// driver parameters for congestion management init, all rates are in Mbps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmng_init_input {
    pub port_rate: u32,
    pub vnic_min_rate: [u16; 4],
    pub vnic_max_rate: [u16; 4],
    pub cos_min_rate: [u16; MAX_COS_NUMBER],
    pub cos_to_pause_mask: [u16; MAX_COS_NUMBER],
    pub flags: cmng_flags_per_port,
}

//
// Protocol-common command ID for slow path elements
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum common_spqe_cmd_id {
    RAMROD_CMD_ID_COMMON_UNUSED,
    RAMROD_CMD_ID_COMMON_FUNCTION_START,
    RAMROD_CMD_ID_COMMON_FUNCTION_STOP,
    RAMROD_CMD_ID_COMMON_FUNCTION_UPDATE,
    RAMROD_CMD_ID_COMMON_CFC_DEL,
    RAMROD_CMD_ID_COMMON_CFC_DEL_WB,
    RAMROD_CMD_ID_COMMON_STAT_QUERY,
    RAMROD_CMD_ID_COMMON_STOP_TRAFFIC,
    RAMROD_CMD_ID_COMMON_START_TRAFFIC,
    RAMROD_CMD_ID_COMMON_AFEX_VIF_LISTS,
    RAMROD_CMD_ID_COMMON_SET_TIMESYNC,
    MAX_COMMON_SPQE_CMD_ID
}

//
// Per-protocol connection types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connection_type {
    ETH_CONNECTION_TYPE,
    TOE_CONNECTION_TYPE,
    RDMA_CONNECTION_TYPE,
    ISCSI_CONNECTION_TYPE,
    FCOE_CONNECTION_TYPE,
    RESERVED_CONNECTION_TYPE_0,
    RESERVED_CONNECTION_TYPE_1,
    RESERVED_CONNECTION_TYPE_2,
    NONE_CONNECTION_TYPE,
    MAX_CONNECTION_TYPE
}

//
// Cos modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cos_mode {
    OVERRIDE_COS,
    STATIC_COS,
    FW_WRR,
    MAX_COS_MODE
}

//
// Dynamic HC counters set by the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_dynamic_drv_counter {
    pub val: [u32; HC_SB_MAX_DYNAMIC_INDICES],
}

//
// zone A per-queue data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstorm_queue_zone_data {
    pub hc_dyn_drv_cnt: hc_dynamic_drv_counter,
    pub reserved: [regpair; 2],
}

//
// Vf-PF channel data in cstorm ram (non-triggered zone)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_channel_zone_data {
    pub msg_addr_lo: u32,
    pub msg_addr_hi: u32,
}

//
// zone for VF non-triggered data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct non_trigger_vf_zone {
    pub vf_pf_channel: vf_pf_channel_zone_data,
}

//
// Vf-PF channel trigger zone in cstorm ram
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_channel_zone_trigger {
    pub addr_valid: u8,
}

//
// zone that triggers the in-bound interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trigger_vf_zone {
    pub vf_pf_channel: vf_pf_channel_zone_trigger,
    pub reserved0: u8,
    pub reserved1: u16,
    pub reserved2: u32,
}

//
// zone B per-VF data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstorm_vf_zone_data {
    pub non_trigger: non_trigger_vf_zone,
    pub trigger: trigger_vf_zone,
}

//
// Dynamic host coalescing init parameters, per state machine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_hc_sm_config {
    pub threshold: [u32; 3],
    pub shift_per_protocol: [u8; HC_SB_MAX_DYNAMIC_INDICES],
    pub hc_timeout0: [u8; HC_SB_MAX_DYNAMIC_INDICES],
    pub hc_timeout1: [u8; HC_SB_MAX_DYNAMIC_INDICES],
    pub hc_timeout2: [u8; HC_SB_MAX_DYNAMIC_INDICES],
    pub hc_timeout3: [u8; HC_SB_MAX_DYNAMIC_INDICES],
}

//
// Dynamic host coalescing init parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_hc_config {
    pub sm_config: [dynamic_hc_sm_config; HC_SB_MAX_SM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e2_integ_data {

    pub flags: u8,

pub const E2_INTEG_DATA_TESTING_EN_SHIFT: c_int = 0;

pub const E2_INTEG_DATA_LB_TX_SHIFT: c_int = 1;

pub const E2_INTEG_DATA_COS_TX_SHIFT: c_int = 2;

pub const E2_INTEG_DATA_OPPORTUNISTICQM_SHIFT: c_int = 3;

pub const E2_INTEG_DATA_DPMTESTRELEASEDQ_SHIFT: c_int = 4;

pub const E2_INTEG_DATA_RESERVED_SHIFT: c_int = 5;
    pub cos: u8,
    pub voq: u8,
    pub pbf_queue: u8,

    pub pbf_queue: u8,
    pub voq: u8,
    pub cos: u8,
    pub flags: u8,

pub const E2_INTEG_DATA_TESTING_EN_SHIFT: c_int = 0;

pub const E2_INTEG_DATA_LB_TX_SHIFT: c_int = 1;

pub const E2_INTEG_DATA_COS_TX_SHIFT: c_int = 2;

pub const E2_INTEG_DATA_OPPORTUNISTICQM_SHIFT: c_int = 3;

pub const E2_INTEG_DATA_DPMTESTRELEASEDQ_SHIFT: c_int = 4;

pub const E2_INTEG_DATA_RESERVED_SHIFT: c_int = 5;

    pub reserved3: u16,
    pub reserved2: u8,
    pub ramEn: u8,

    pub ramEn: u8,
    pub reserved2: u8,
    pub reserved3: u16,

}

//
// set mac event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_event_data {
    pub echo: __le32,
    pub reserved0: __le32,
    pub reserved1: __le32,
}

//
// pf-vf event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_event_data {
    pub vf_id: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
    pub msg_addr_lo: __le32,
    pub msg_addr_hi: __le32,
}

//
// VF FLR event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_flr_event_data {
    pub vf_id: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
    pub reserved2: __le32,
    pub reserved3: __le32,
}

//
// malicious VF event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct malicious_vf_event_data {
    pub vf_id: u8,
    pub err_id: u8,
    pub reserved1: __le16,
    pub reserved2: __le32,
    pub reserved3: __le32,
}

//
// vif list event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vif_list_event_data {
    pub func_bit_map: u8,
    pub echo: u8,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub reserved2: __le32,
}

// function update event data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct function_update_event_data {
    pub echo: u8,
    pub reserved: u8,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub reserved2: __le32,
}

// union for all event ring message types
#[repr(C)]
#[derive(Copy, Clone)]
pub union event_data {
    pub vf_pf_event: vf_pf_event_data,
    pub eth_event: eth_event_data,
    pub cfc_del_event: cfc_del_event_data,
    pub vf_flr_event: vf_flr_event_data,
    pub malicious_vf_event: malicious_vf_event_data,
    pub vif_list_event: vif_list_event_data,
    pub function_update_event: function_update_event_data,
}

//
// per PF event ring data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_ring_data {
    pub base_addr: regpair_native,

    pub index_id: u8,
    pub sb_id: u8,
    pub producer: u16,

    pub producer: u16,
    pub sb_id: u8,
    pub index_id: u8,

    pub reserved0: u32,
}

//
// event ring message element (each element is 128 bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_ring_msg {
    pub opcode: u8,
    pub error: u8,
    pub reserved1: u16,
    pub data: event_data,
}

//
// event ring next page element (128 bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_ring_next {
    pub addr: regpair,
    pub reserved: [u32; 2],
}

//
// union for event ring element types (each element is 128 bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union event_ring_elem {
    pub message: event_ring_msg,
    pub next_page: event_ring_next,
}

//
// Common event ring opcodes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_ring_opcode {
    EVENT_RING_OPCODE_VF_PF_CHANNEL,
    EVENT_RING_OPCODE_FUNCTION_START,
    EVENT_RING_OPCODE_FUNCTION_STOP,
    EVENT_RING_OPCODE_CFC_DEL,
    EVENT_RING_OPCODE_CFC_DEL_WB,
    EVENT_RING_OPCODE_STAT_QUERY,
    EVENT_RING_OPCODE_STOP_TRAFFIC,
    EVENT_RING_OPCODE_START_TRAFFIC,
    EVENT_RING_OPCODE_VF_FLR,
    EVENT_RING_OPCODE_MALICIOUS_VF,
    EVENT_RING_OPCODE_FORWARD_SETUP,
    EVENT_RING_OPCODE_RSS_UPDATE_RULES,
    EVENT_RING_OPCODE_FUNCTION_UPDATE,
    EVENT_RING_OPCODE_AFEX_VIF_LISTS,
    EVENT_RING_OPCODE_SET_MAC,
    EVENT_RING_OPCODE_CLASSIFICATION_RULES,
    EVENT_RING_OPCODE_FILTERS_RULES,
    EVENT_RING_OPCODE_MULTICAST_RULES,
    EVENT_RING_OPCODE_SET_TIMESYNC,
    MAX_EVENT_RING_OPCODE
}

//
// Modes for fairness algorithm
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fairness_mode {
    FAIRNESS_COS_WRR_MODE,
    FAIRNESS_COS_ETS_MODE,
    MAX_FAIRNESS_MODE
}

//
// Priority and cos
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct priority_cos {
    pub priority: u8,
    pub cos: u8,
    pub reserved1: __le16,
}

//
// The data for flow control configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_control_configuration {
    pub traffic_type_to_priority_cos: [priority_cos; MAX_TRAFFIC_TYPES],
    pub dcb_enabled: u8,
    pub dcb_version: u8,
    pub dont_add_pri_0_en: u8,
    pub reserved1: u8,
    pub reserved2: __le32,
    pub dcb_outer_pri: [u8; MAX_TRAFFIC_TYPES],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct function_start_data {
    pub function_mode: u8,
    pub allow_npar_tx_switching: u8,
    pub sd_vlan_tag: __le16,
    pub vif_id: __le16,
    pub path_id: u8,
    pub network_cos_mode: u8,
    pub dmae_cmd_id: u8,
    pub no_added_tags: u8,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub inner_clss_vxlan: u8,
    pub inner_clss_l2gre: u8,
    pub inner_clss_l2geneve: u8,
    pub inner_rss: u8,
    pub vxlan_dst_port: __le16,
    pub geneve_dst_port: __le16,
    pub sd_accept_mf_clss_fail: u8,
    pub sd_accept_mf_clss_fail_match_ethtype: u8,
    pub sd_accept_mf_clss_fail_ethtype: __le16,
    pub sd_vlan_eth_type: __le16,
    pub sd_vlan_force_pri_flg: u8,
    pub sd_vlan_force_pri_val: u8,
    pub c2s_pri_tt_valid: u8,
    pub c2s_pri_default: u8,
    pub tx_vlan_filtering_enable: u8,
    pub tx_vlan_filtering_use_pvid: u8,
    pub reserved2: [u8; 4],
    pub c2s_pri_trans_table: c2s_pri_trans_table_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct function_update_data {
    pub vif_id_change_flg: u8,
    pub afex_default_vlan_change_flg: u8,
    pub allowed_priorities_change_flg: u8,
    pub network_cos_mode_change_flg: u8,
    pub vif_id: __le16,
    pub afex_default_vlan: __le16,
    pub allowed_priorities: u8,
    pub network_cos_mode: u8,
    pub lb_mode_en_change_flg: u8,
    pub lb_mode_en: u8,
    pub tx_switch_suspend_change_flg: u8,
    pub tx_switch_suspend: u8,
    pub echo: u8,
    pub update_tunn_cfg_flg: u8,
    pub inner_clss_vxlan: u8,
    pub inner_clss_l2gre: u8,
    pub inner_clss_l2geneve: u8,
    pub inner_rss: u8,
    pub vxlan_dst_port: __le16,
    pub geneve_dst_port: __le16,
    pub sd_vlan_force_pri_change_flg: u8,
    pub sd_vlan_force_pri_flg: u8,
    pub sd_vlan_force_pri_val: u8,
    pub sd_vlan_tag_change_flg: u8,
    pub sd_vlan_eth_type_change_flg: u8,
    pub reserved1: u8,
    pub sd_vlan_tag: __le16,
    pub sd_vlan_eth_type: __le16,
    pub tx_vlan_filtering_pvid_change_flg: u8,
    pub reserved0: u8,
    pub reserved2: __le32,
}

//
// FW version stored in the Xstorm RAM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_version {

    pub engineering: u8,
    pub revision: u8,
    pub minor: u8,
    pub major: u8,

    pub major: u8,
    pub minor: u8,
    pub revision: u8,
    pub engineering: u8,

    pub flags: u32,

pub const FW_VERSION_OPTIMIZED_SHIFT: c_int = 0;

pub const FW_VERSION_BIG_ENDIEN_SHIFT: c_int = 1;

pub const FW_VERSION_CHIP_VERSION_SHIFT: c_int = 2;

pub const __FW_VERSION_RESERVED_SHIFT: c_int = 4;
}

//
// Dynamic Host-Coalescing - Driver(host) counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_dynamic_sb_drv_counters {
    pub dynamic_hc_drv_counter: [u32; HC_SB_MAX_DYNAMIC_INDICES],
}

//
// 2 bytes. configuration/state parameters for a single protocol index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_index_data {

    pub flags: u8,

pub const HC_INDEX_DATA_SM_ID_SHIFT: c_int = 0;

pub const HC_INDEX_DATA_HC_ENABLED_SHIFT: c_int = 1;

pub const HC_INDEX_DATA_DYNAMIC_HC_ENABLED_SHIFT: c_int = 2;

pub const HC_INDEX_DATA_RESERVE_SHIFT: c_int = 3;
    pub timeout: u8,

    pub timeout: u8,
    pub flags: u8,

pub const HC_INDEX_DATA_SM_ID_SHIFT: c_int = 0;

pub const HC_INDEX_DATA_HC_ENABLED_SHIFT: c_int = 1;

pub const HC_INDEX_DATA_DYNAMIC_HC_ENABLED_SHIFT: c_int = 2;

pub const HC_INDEX_DATA_RESERVE_SHIFT: c_int = 3;

}

//
// HC state-machine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_status_block_sm {

    pub igu_seg_id: u8,
    pub igu_sb_id: u8,
    pub timer_value: u8,
    pub __flags: u8,

    pub __flags: u8,
    pub timer_value: u8,
    pub igu_sb_id: u8,
    pub igu_seg_id: u8,

    pub time_to_expire: u32,
}

//
// hold PCI identification variables- used in various places in firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_entity {

    pub vf_valid: u8,
    pub vf_id: u8,
    pub vnic_id: u8,
    pub pf_id: u8,

    pub pf_id: u8,
    pub vnic_id: u8,
    pub vf_id: u8,
    pub vf_valid: u8,

}

//
// The fast-path status block meta-data, common to all chips
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_sb_data {
    pub host_sb_addr: regpair_native,
    pub state_machine: [hc_status_block_sm; HC_SB_MAX_SM],
    pub p_func: pci_entity,

    pub rsrv0: u8,
    pub state: u8,
    pub dhc_qzone_id: u8,
    pub same_igu_sb_1b: u8,

    pub same_igu_sb_1b: u8,
    pub dhc_qzone_id: u8,
    pub state: u8,
    pub rsrv0: u8,
    pub rsrv1: [regpair_native; 2],
}

//
// Segment types for host coaslescing
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hc_segment {
    HC_REGULAR_SEGMENT,
    HC_DEFAULT_SEGMENT,
    MAX_HC_SEGMENT
}

//
// The fast-path status block meta-data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_sp_status_block_data {
    pub host_sb_addr: regpair_native,

    pub rsrv1: u8,
    pub state: u8,
    pub igu_seg_id: u8,
    pub igu_sb_id: u8,

    pub igu_sb_id: u8,
    pub igu_seg_id: u8,
    pub state: u8,
    pub rsrv1: u8,

    pub p_func: pci_entity,
}

//
// The fast-path status block meta-data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_status_block_data_e1x {
    pub index_data: [hc_index_data; HC_SB_MAX_INDICES_E1X],
    pub common: hc_sb_data,
}

//
// The fast-path status block meta-data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_status_block_data_e2 {
    pub index_data: [hc_index_data; HC_SB_MAX_INDICES_E2],
    pub common: hc_sb_data,
}

//
// IGU block operartion modes (in Everest2)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_mode {
    HC_IGU_BC_MODE,
    HC_IGU_NBC_MODE,
    MAX_IGU_MODE
}

//
// Inner Headers Classification Type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inner_clss_type {
    INNER_CLSS_DISABLED,
    INNER_CLSS_USE_VLAN,
    INNER_CLSS_USE_VNI,
    MAX_INNER_CLSS_TYPE};

//
// IP versions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_ver {
    IP_V4,
    IP_V6,
    MAX_IP_VER
}

//
// Malicious VF error ID
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum malicious_vf_error_id {
    MALICIOUS_VF_NO_ERROR,
    VF_PF_CHANNEL_NOT_READY,
    ETH_ILLEGAL_BD_LENGTHS,
    ETH_PACKET_TOO_SHORT,
    ETH_PAYLOAD_TOO_BIG,
    ETH_ILLEGAL_ETH_TYPE,
    ETH_ILLEGAL_LSO_HDR_LEN,
    ETH_TOO_MANY_BDS,
    ETH_ZERO_HDR_NBDS,
    ETH_START_BD_NOT_SET,
    ETH_ILLEGAL_PARSE_NBDS,
    ETH_IPV6_AND_CHECKSUM,
    ETH_VLAN_FLG_INCORRECT,
    ETH_ILLEGAL_LSO_MSS,
    ETH_TUNNEL_NOT_SUPPORTED,
    MAX_MALICIOUS_VF_ERROR_ID
}

//
// Multi-function modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mf_mode {
    SINGLE_FUNCTION,
    MULTI_FUNCTION_SD,
    MULTI_FUNCTION_SI,
    MULTI_FUNCTION_AFEX,
    MAX_MF_MODE
}

//
// Protocol-common statistics collected by the Tstorm (per pf)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_per_pf_stats {
    pub rcv_error_bytes: regpair,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_pf_stats {
    pub tstorm_pf_statistics: tstorm_per_pf_stats,
}

//
// Protocol-common statistics collected by the Tstorm (per port)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_per_port_stats {
    pub mac_discard: __le32,
    pub mac_filter_discard: __le32,
    pub brb_truncate_discard: __le32,
    pub mf_tag_discard: __le32,
    pub packet_drop: __le32,
    pub reserved: __le32,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_port_stats {
    pub tstorm_port_statistics: tstorm_per_port_stats,
}

//
// Protocol-common statistics collected by the Tstorm (per client)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_per_queue_stats {
    pub rcv_ucast_bytes: regpair,
    pub rcv_ucast_pkts: __le32,
    pub checksum_discard: __le32,
    pub rcv_bcast_bytes: regpair,
    pub rcv_bcast_pkts: __le32,
    pub pkts_too_big_discard: __le32,
    pub rcv_mcast_bytes: regpair,
    pub rcv_mcast_pkts: __le32,
    pub ttl0_discard: __le32,
    pub no_buff_discard: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
}

//
// Protocol-common statistics collected by the Ustorm (per client)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_per_queue_stats {
    pub ucast_no_buff_bytes: regpair,
    pub mcast_no_buff_bytes: regpair,
    pub bcast_no_buff_bytes: regpair,
    pub ucast_no_buff_pkts: __le32,
    pub mcast_no_buff_pkts: __le32,
    pub bcast_no_buff_pkts: __le32,
    pub coalesced_pkts: __le32,
    pub coalesced_bytes: regpair,
    pub coalesced_events: __le32,
    pub coalesced_aborts: __le32,
}

//
// Protocol-common statistics collected by the Xstorm (per client)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_per_queue_stats {
    pub ucast_bytes_sent: regpair,
    pub mcast_bytes_sent: regpair,
    pub bcast_bytes_sent: regpair,
    pub ucast_pkts_sent: __le32,
    pub mcast_pkts_sent: __le32,
    pub bcast_pkts_sent: __le32,
    pub error_drop_pkts: __le32,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_queue_stats {
    pub tstorm_queue_statistics: tstorm_per_queue_stats,
    pub ustorm_queue_statistics: ustorm_per_queue_stats,
    pub xstorm_queue_statistics: xstorm_per_queue_stats,
}

//
// FW version stored in first line of pram
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pram_fw_version {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
    pub engineering: u8,
    pub flags: u8,

pub const PRAM_FW_VERSION_OPTIMIZED_SHIFT: c_int = 0;

pub const PRAM_FW_VERSION_STORM_ID_SHIFT: c_int = 1;

pub const PRAM_FW_VERSION_BIG_ENDIEN_SHIFT: c_int = 3;

pub const PRAM_FW_VERSION_CHIP_VERSION_SHIFT: c_int = 4;

pub const __PRAM_FW_VERSION_RESERVED0_SHIFT: c_int = 6;
}

//
// Ethernet slow path element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union protocol_common_specific_data {
    pub protocol_data: [u8; 8],
    pub phy_address: regpair,
    pub mac_config_addr: regpair,
    pub afex_vif_list_data: afex_vif_list_ramrod_data,
}

//
// The send queue element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protocol_common_spe {
    pub hdr: spe_hdr,
    pub data: protocol_common_specific_data,
}

// The data for the Set Timesync Ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_timesync_ramrod_data {
    pub drift_adjust_cmd: u8,
    pub offset_cmd: u8,
    pub add_sub_drift_adjust_value: u8,
    pub drift_adjust_value: u8,
    pub drift_adjust_period: u32,
    pub offset_delta: regpair,
}

//
// The send queue element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slow_path_element {
    pub hdr: spe_hdr,
    pub protocol_data: regpair,
}

//
// Protocol-common statistics counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_counter {
    pub xstats_counter: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub tstats_counter: __le16,
    pub reserved2: __le16,
    pub reserved3: __le32,
    pub ustats_counter: __le16,
    pub reserved4: __le16,
    pub reserved5: __le32,
    pub cstats_counter: __le16,
    pub reserved6: __le16,
    pub reserved7: __le32,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_query_entry {
    pub kind: u8,
    pub index: u8,
    pub funcID: __le16,
    pub reserved: __le32,
    pub address: regpair,
}

//
// statistic command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_query_cmd_group {
    pub query: [stats_query_entry; STATS_QUERY_CMD_COUNT],
}

//
// statistic command header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_query_header {
    pub cmd_num: u8,
    pub reserved0: u8,
    pub drv_stats_counter: __le16,
    pub reserved1: __le32,
    pub stats_counters_addrs: regpair,
}

//
// Types of statistcis query entry
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stats_query_type {
    STATS_TYPE_QUEUE,
    STATS_TYPE_PORT,
    STATS_TYPE_PF,
    STATS_TYPE_TOE,
    STATS_TYPE_FCOE,
    MAX_STATS_QUERY_TYPE
}

//
// Indicate of the function status block state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum status_block_state {
    SB_DISABLED,
    SB_ENABLED,
    SB_CLEANED,
    MAX_STATUS_BLOCK_STATE
}

//
// Storm IDs (including attentions for IGU related enums)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum storm_id {
    USTORM_ID,
    CSTORM_ID,
    XSTORM_ID,
    TSTORM_ID,
    ATTENTION_ID,
    MAX_STORM_ID
}

//
// Taffic types used in ETS and flow control algorithms
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum traffic_type {
    LLFC_TRAFFIC_TYPE_NW,
    LLFC_TRAFFIC_TYPE_FCOE,
    LLFC_TRAFFIC_TYPE_ISCSI,
    MAX_TRAFFIC_TYPE
}

//
// zone A per-queue data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_queue_zone_data {
    pub reserved: [regpair; 4],
}

//
// zone B per-VF data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_vf_zone_data {
    pub reserved: regpair,
}

// Add or Subtract Value for Set Timesync Ramrod
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ts_add_sub_value {
    TS_SUB_VALUE,
    TS_ADD_VALUE,
    MAX_TS_ADD_SUB_VALUE
}

// Drift-Adjust Commands for Set Timesync Ramrod
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ts_drift_adjust_cmd {
    TS_DRIFT_ADJUST_KEEP,
    TS_DRIFT_ADJUST_SET,
    TS_DRIFT_ADJUST_RESET,
    MAX_TS_DRIFT_ADJUST_CMD
}

// Offset Commands for Set Timesync Ramrod
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ts_offset_cmd {
    TS_OFFSET_KEEP,
    TS_OFFSET_INC,
    TS_OFFSET_DEC,
    MAX_TS_OFFSET_CMD
}

// zone A per-queue data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_queue_zone_data {
    pub eth_rx_producers: ustorm_eth_rx_producers,
    pub reserved: [regpair; 3],
}

//
// zone B per-VF data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_vf_zone_data {
    pub reserved: regpair,
}

//
// data per VF-PF channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_channel_data {

    pub reserved0: u16,
    pub valid: u8,
    pub state: u8,

    pub state: u8,
    pub valid: u8,
    pub reserved0: u16,

    pub reserved1: u32,
}

//
// State of VF-PF channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf_pf_channel_state {
    VF_PF_CHANNEL_STATE_READY,
    VF_PF_CHANNEL_STATE_WAITING_FOR_ACK,
    MAX_VF_PF_CHANNEL_STATE
}

//
// vif_list_rule_kind
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vif_list_rule_kind {
    VIF_LIST_RULE_SET,
    VIF_LIST_RULE_GET,
    VIF_LIST_RULE_CLEAR_ALL,
    VIF_LIST_RULE_CLEAR_FUNC,
    MAX_VIF_LIST_RULE_KIND
}

//
// zone A per-queue data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_queue_zone_data {
    pub reserved: [regpair; 4],
}

//
// zone B per-VF data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_vf_zone_data {
    pub reserved: regpair,
}
