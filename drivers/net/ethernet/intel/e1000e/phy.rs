//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/phy.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
extern "C" {
    pub fn e1000e_check_downshift(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_check_polarity_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_check_polarity_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_check_polarity_ife(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_check_reset_block_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_copper_link_setup_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_copper_link_setup_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_phy_force_speed_duplex_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_phy_force_speed_duplex_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_phy_force_speed_duplex_ife(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_cable_length_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_cable_length_igp_2(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_cfg_done_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_phy_id(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_phy_info_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_phy_info_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_get_phy_info_ife(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_phy_sw_reset(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_phy_force_speed_duplex_setup(hw: *mut e1000_hw, phy_ctrl: *mut u16);
}
extern "C" {
    pub fn e1000e_phy_hw_reset_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_phy_reset_dsp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_read_kmrn_reg(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_read_kmrn_reg_locked(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_set_page_igp(hw: *mut e1000_hw, page: u16) -> i32;
}
extern "C" {
    pub fn e1000e_read_phy_reg_igp(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_read_phy_reg_igp_locked(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_read_phy_reg_m88(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_set_d3_lplu_state(hw: *mut e1000_hw, active: bool) -> i32;
}
extern "C" {
    pub fn e1000e_setup_copper_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_write_kmrn_reg(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000e_write_kmrn_reg_locked(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000e_write_phy_reg_igp(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000e_write_phy_reg_igp_locked(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000e_write_phy_reg_m88(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000e_phy_init_script_igp3(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_phy_type_from_id(phy_id: u32) -> e1000_phy_type;
}
extern "C" {
    pub fn e1000e_determine_phy_address(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_write_phy_reg_bm(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000e_read_phy_reg_bm(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_enable_phy_wakeup_reg_access_bm(hw: *mut e1000_hw, phy_reg: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_disable_phy_wakeup_reg_access_bm(hw: *mut e1000_hw, phy_reg: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_read_phy_reg_bm2(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_write_phy_reg_bm2(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_power_up_phy_copper(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_power_down_phy_copper(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_disable_phy_retry(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_enable_phy_retry(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_read_phy_reg_mdic(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000e_write_phy_reg_mdic(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_read_phy_reg_hv(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_read_phy_reg_hv_locked(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_read_phy_reg_page_hv(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_write_phy_reg_hv(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_write_phy_reg_hv_locked(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_write_phy_reg_page_hv(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_link_stall_workaround_hv(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_copper_link_setup_82577(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_check_polarity_82577(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_get_phy_info_82577(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_phy_force_speed_duplex_82577(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_get_cable_length_82577(hw: *mut e1000_hw) -> i32;
}
pub const E1000_MAX_PHY_ADDR: c_int = 8;
// IGP01E1000 Specific Registers
pub const IGP01E1000_PHY_PORT_CONFIG: c_uint = 0x10	/* Port Config */;
pub const IGP01E1000_PHY_PORT_STATUS: c_uint = 0x11	/* Status */;
pub const IGP01E1000_PHY_PORT_CTRL: c_uint = 0x12	/* Control */;
pub const IGP01E1000_PHY_LINK_HEALTH: c_uint = 0x13	/* PHY Link Health */;
pub const IGP02E1000_PHY_POWER_MGMT: c_uint = 0x19	/* Power Management */;
pub const IGP01E1000_PHY_PAGE_SELECT: c_uint = 0x1F	/* Page Select */;

pub const IGP_PAGE_SHIFT: c_int = 5;
pub const PHY_REG_MASK: c_uint = 0x1F;
// BM/HV Specific Registers
pub const BM_PORT_CTRL_PAGE: c_int = 769;
pub const BM_WUC_PAGE: c_int = 800;
pub const BM_WUC_ADDRESS_OPCODE: c_uint = 0x11;
pub const BM_WUC_DATA_OPCODE: c_uint = 0x12;

pub const BM_WUC_ENABLE_REG: c_int = 17;

pub const PHY_UPPER_SHIFT: c_int = 21;

pub const HV_INTC_FC_PAGE_START: c_int = 768;
pub const I82578_ADDR_REG: c_int = 29;
pub const I82577_ADDR_REG: c_int = 16;
pub const I82577_CFG_REG: c_int = 22;

pub const I82577_CTRL_REG: c_int = 23;
// 82577 specific PHY registers
pub const I82577_PHY_CTRL_2: c_int = 18;
pub const I82577_PHY_LBK_CTRL: c_int = 19;
pub const I82577_PHY_STATUS_2: c_int = 26;
pub const I82577_PHY_DIAG_STATUS: c_int = 31;
// I82577 PHY Status 2
pub const I82577_PHY_STATUS2_REV_POLARITY: c_uint = 0x0400;
pub const I82577_PHY_STATUS2_MDIX: c_uint = 0x0800;
pub const I82577_PHY_STATUS2_SPEED_MASK: c_uint = 0x0300;
pub const I82577_PHY_STATUS2_SPEED_1000MBPS: c_uint = 0x0200;
// I82577 PHY Control 2
pub const I82577_PHY_CTRL2_MANUAL_MDIX: c_uint = 0x0200;
pub const I82577_PHY_CTRL2_AUTO_MDI_MDIX: c_uint = 0x0400;
pub const I82577_PHY_CTRL2_MDIX_CFG_MASK: c_uint = 0x0600;
// I82577 PHY Diagnostics Status
pub const I82577_DSTATUS_CABLE_LENGTH: c_uint = 0x03FC;
pub const I82577_DSTATUS_CABLE_LENGTH_SHIFT: c_int = 2;
// BM PHY Copper Specific Control 1
pub const BM_CS_CTRL1: c_int = 16;
// BM PHY Copper Specific Status
pub const BM_CS_STATUS: c_int = 17;
pub const BM_CS_STATUS_LINK_UP: c_uint = 0x0400;
pub const BM_CS_STATUS_RESOLVED: c_uint = 0x0800;
pub const BM_CS_STATUS_SPEED_MASK: c_uint = 0xC000;
pub const BM_CS_STATUS_SPEED_1000: c_uint = 0x8000;
// 82577 Mobile Phy Status Register
pub const HV_M_STATUS: c_int = 26;
pub const HV_M_STATUS_AUTONEG_COMPLETE: c_uint = 0x1000;
pub const HV_M_STATUS_SPEED_MASK: c_uint = 0x0300;
pub const HV_M_STATUS_SPEED_1000: c_uint = 0x0200;
pub const HV_M_STATUS_SPEED_100: c_uint = 0x0100;
pub const HV_M_STATUS_LINK_UP: c_uint = 0x0040;
pub const IGP01E1000_PHY_PCS_INIT_REG: c_uint = 0x00B4;
pub const IGP01E1000_PHY_POLARITY_MASK: c_uint = 0x0078;
pub const IGP01E1000_PSCR_AUTO_MDIX: c_uint = 0x1000;
pub const IGP01E1000_PSCR_FORCE_MDI_MDIX: c_uint = 0x2000	/* 0=MDI, 1=MDIX */;
pub const IGP01E1000_PSCFR_SMART_SPEED: c_uint = 0x0080;
pub const IGP02E1000_PM_SPD: c_uint = 0x0001	/* Smart Power Down */;
pub const IGP02E1000_PM_D0_LPLU: c_uint = 0x0002	/* For D0a states */;
pub const IGP02E1000_PM_D3_LPLU: c_uint = 0x0004	/* For all other states */;
pub const IGP01E1000_PLHR_SS_DOWNGRADE: c_uint = 0x8000;
pub const IGP01E1000_PSSR_POLARITY_REVERSED: c_uint = 0x0002;
pub const IGP01E1000_PSSR_MDIX: c_uint = 0x0800;
pub const IGP01E1000_PSSR_SPEED_MASK: c_uint = 0xC000;
pub const IGP01E1000_PSSR_SPEED_1000MBPS: c_uint = 0xC000;
pub const IGP02E1000_PHY_CHANNEL_NUM: c_int = 4;
pub const IGP02E1000_PHY_AGC_A: c_uint = 0x11B1;
pub const IGP02E1000_PHY_AGC_B: c_uint = 0x12B1;
pub const IGP02E1000_PHY_AGC_C: c_uint = 0x14B1;
pub const IGP02E1000_PHY_AGC_D: c_uint = 0x18B1;

pub const IGP02E1000_AGC_LENGTH_MASK: c_uint = 0x7F;
pub const IGP02E1000_AGC_RANGE: c_int = 15;
pub const E1000_CABLE_LENGTH_UNDEFINED: c_uint = 0xFF;
pub const E1000_KMRNCTRLSTA_OFFSET: c_uint = 0x001F0000;
pub const E1000_KMRNCTRLSTA_OFFSET_SHIFT: c_int = 16;
pub const E1000_KMRNCTRLSTA_REN: c_uint = 0x00200000;
pub const E1000_KMRNCTRLSTA_CTRL_OFFSET: c_uint = 0x1	/* Kumeran Control */;
pub const E1000_KMRNCTRLSTA_DIAG_OFFSET: c_uint = 0x3	/* Kumeran Diagnostic */;
pub const E1000_KMRNCTRLSTA_TIMEOUTS: c_uint = 0x4	/* Kumeran Timeouts */;
pub const E1000_KMRNCTRLSTA_INBAND_PARAM: c_uint = 0x9	/* Kumeran InBand Parameters */;
pub const E1000_KMRNCTRLSTA_IBIST_DISABLE: c_uint = 0x0200	/* Kumeran IBIST Disable */;
pub const E1000_KMRNCTRLSTA_DIAG_NELPBK: c_uint = 0x1000	/* Nearend Loopback mode */;
pub const E1000_KMRNCTRLSTA_K1_CONFIG: c_uint = 0x7;
pub const E1000_KMRNCTRLSTA_K1_ENABLE: c_uint = 0x0002	/* enable K1 */;
pub const E1000_KMRNCTRLSTA_HD_CTRL: c_uint = 0x10	/* Kumeran HD Control */;
pub const IFE_PHY_EXTENDED_STATUS_CONTROL: c_uint = 0x10;
pub const IFE_PHY_SPECIAL_CONTROL: c_uint = 0x11	/* 100BaseTx PHY Special Ctrl */;
pub const IFE_PHY_SPECIAL_CONTROL_LED: c_uint = 0x1B	/* PHY Special and LED Ctrl */;
pub const IFE_PHY_MDIX_CONTROL: c_uint = 0x1C	/* MDI/MDI-X Control */;
// IFE PHY Extended Status Control
pub const IFE_PESC_POLARITY_REVERSED: c_uint = 0x0100;
// IFE PHY Special Control
pub const IFE_PSC_AUTO_POLARITY_DISABLE: c_uint = 0x0010;
pub const IFE_PSC_FORCE_POLARITY: c_uint = 0x0020;
// IFE PHY Special Control and LED Control
pub const IFE_PSCL_PROBE_MODE: c_uint = 0x0020;
pub const IFE_PSCL_PROBE_LEDS_OFF: c_uint = 0x0006	/* Force LEDs 0 and 2 off */;
pub const IFE_PSCL_PROBE_LEDS_ON: c_uint = 0x0007	/* Force LEDs 0 and 2 on */;
// IFE PHY MDIX Control
pub const IFE_PMC_MDIX_STATUS: c_uint = 0x0020	/* 1=MDI-X, 0=MDI */;
pub const IFE_PMC_FORCE_MDIX: c_uint = 0x0040	/* 1=force MDI-X, 0=force MDI */;
pub const IFE_PMC_AUTO_MDIX: c_uint = 0x0080	/* 1=enable auto, 0=disable */;
