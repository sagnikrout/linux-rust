//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/ich8lan.h
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
pub const ICH_FLASH_GFPREG: c_uint = 0x0000;
pub const ICH_FLASH_HSFSTS: c_uint = 0x0004;
pub const ICH_FLASH_HSFCTL: c_uint = 0x0006;
pub const ICH_FLASH_FADDR: c_uint = 0x0008;
pub const ICH_FLASH_FDATA0: c_uint = 0x0010;
pub const ICH_FLASH_PR0: c_uint = 0x0074;
// Requires up to 10 seconds when MNG might be accessing part.
pub const ICH_FLASH_READ_COMMAND_TIMEOUT: c_int = 10000000;
pub const ICH_FLASH_WRITE_COMMAND_TIMEOUT: c_int = 10000000;
pub const ICH_FLASH_ERASE_COMMAND_TIMEOUT: c_int = 10000000;
pub const ICH_FLASH_LINEAR_ADDR_MASK: c_uint = 0x00FFFFFF;
pub const ICH_FLASH_CYCLE_REPEAT_COUNT: c_int = 10;
pub const ICH_CYCLE_READ: c_int = 0;
pub const ICH_CYCLE_WRITE: c_int = 2;
pub const ICH_CYCLE_ERASE: c_int = 3;
pub const FLASH_GFPREG_BASE_MASK: c_uint = 0x1FFF;
pub const FLASH_SECTOR_ADDR_SHIFT: c_int = 12;
pub const ICH_FLASH_SEG_SIZE_256: c_int = 256;
pub const ICH_FLASH_SEG_SIZE_4K: c_int = 4096;
pub const ICH_FLASH_SEG_SIZE_8K: c_int = 8192;
pub const ICH_FLASH_SEG_SIZE_64K: c_int = 65536;
pub const E1000_ICH_FWSM_RSPCIPHY: c_uint = 0x00000040	/* Reset PHY on PCI Reset */;
// FW established a valid mode
pub const E1000_ICH_FWSM_FW_VALID: c_uint = 0x00008000;
pub const E1000_ICH_FWSM_PCIM2PCI: c_uint = 0x01000000	/* ME PCIm-to-PCI active */;
pub const E1000_ICH_FWSM_PCIM2PCI_COUNT: c_int = 2000;
pub const E1000_ICH_MNG_IAMT_MODE: c_uint = 0x2;
pub const E1000_FWSM_WLOCK_MAC_MASK: c_uint = 0x0380;
pub const E1000_FWSM_WLOCK_MAC_SHIFT: c_int = 7;
pub const E1000_FWSM_ULP_CFG_DONE: c_uint = 0x00000400	/* Low power cfg done */;
pub const E1000_EXFWSM_DPG_EXIT_DONE: c_uint = 0x00000001;
// Shared Receive Address Registers

pub const E1000_H2ME: c_uint = 0x05B50	/* Host to ME */;
pub const E1000_H2ME_START_DPG: c_uint = 0x00000001	/* indicate the ME of DPG */;
pub const E1000_H2ME_EXIT_DPG: c_uint = 0x00000002	/* indicate the ME exit DPG */;
pub const E1000_H2ME_ULP: c_uint = 0x00000800	/* ULP Indication Bit */;
pub const E1000_H2ME_ENFORCE_SETTINGS: c_uint = 0x00001000	/* Enforce Settings */;

pub const E1000_ICH_NVM_SIG_WORD: c_uint = 0x13u;
pub const E1000_ICH_NVM_SIG_MASK: c_uint = 0xC000u;
pub const E1000_ICH_NVM_VALID_SIG_MASK: c_uint = 0xC0u;
pub const E1000_ICH_NVM_SIG_VALUE: c_uint = 0x80u;
pub const E1000_ICH8_LAN_INIT_TIMEOUT: c_int = 1500;
// FEXT register bit definition
pub const E1000_FEXT_PHY_CABLE_DISCONNECTED: c_uint = 0x00000004;
pub const E1000_FEXTNVM_SW_CONFIG: c_int = 1;

pub const E1000_FEXTNVM3_PHY_CFG_COUNTER_MASK: c_uint = 0x0C000000;
pub const E1000_FEXTNVM3_PHY_CFG_COUNTER_50MSEC: c_uint = 0x08000000;
pub const E1000_FEXTNVM4_BEACON_DURATION_MASK: c_uint = 0x7;
pub const E1000_FEXTNVM4_BEACON_DURATION_8USEC: c_uint = 0x7;
pub const E1000_FEXTNVM4_BEACON_DURATION_16USEC: c_uint = 0x3;
pub const E1000_FEXTNVM6_REQ_PLL_CLK: c_uint = 0x00000100;
pub const E1000_FEXTNVM6_ENABLE_K1_ENTRY_CONDITION: c_uint = 0x00000200;
pub const E1000_FEXTNVM6_K1_OFF_ENABLE: c_uint = 0x80000000;
// bit for disabling packet buffer read
pub const E1000_FEXTNVM7_DISABLE_PB_READ: c_uint = 0x00040000;
pub const E1000_FEXTNVM7_SIDE_CLK_UNGATE: c_uint = 0x00000004;
pub const E1000_FEXTNVM7_DISABLE_SMB_PERST: c_uint = 0x00000020;
pub const E1000_FEXTNVM9_IOSFSB_CLKGATE_DIS: c_uint = 0x00000800;
pub const E1000_FEXTNVM9_IOSFSB_CLKREQ_DIS: c_uint = 0x00001000;
pub const E1000_FEXTNVM11_DISABLE_PB_READ: c_uint = 0x00000200;
pub const E1000_FEXTNVM11_DISABLE_MULR_FIX: c_uint = 0x00002000;
// bit24: RXDCTL thresholds granularity: 0 - cache lines, 1 - descriptors
pub const E1000_RXDCTL_THRESH_UNIT_DESC: c_uint = 0x01000000;
pub const K1_ENTRY_LATENCY: c_int = 0;
pub const K1_MIN_TIME: c_int = 1;

pub const E1000_FLASH_BASE_ADDR: c_uint = 0xE000	/*offset of NVM access regs */;
pub const E1000_CTRL_EXT_NVMVS: c_uint = 0x3	/*NVM valid sector */;
pub const E1000_TARC0_CB_MULTIQ_3_REQ: c_uint = 0x30000000;
pub const E1000_TARC0_CB_MULTIQ_2_REQ: c_uint = 0x20000000;

pub const E1000_ICH_RAR_ENTRIES: c_int = 7;

pub const PHY_PAGE_SHIFT: c_int = 5;

pub const IGP3_KMRN_DIAG_PCS_LOCK_LOSS: c_uint = 0x0002;
pub const IGP3_VR_CTRL_DEV_POWERDOWN_MODE_MASK: c_uint = 0x0300;
pub const IGP3_VR_CTRL_MODE_SHUTDOWN: c_uint = 0x0200;
// PHY Wakeup Registers and defines

pub const BM_RCTL_UPE: c_uint = 0x0001	/* Unicast Promiscuous Mode */;
pub const BM_RCTL_MPE: c_uint = 0x0002	/* Multicast Promiscuous Mode */;

pub const BM_RCTL_BAM: c_uint = 0x0020	/* Broadcast Accept Mode */;
pub const BM_RCTL_PMCF: c_uint = 0x0040	/* Pass MAC Control Frames */;
pub const BM_RCTL_RFCE: c_uint = 0x0080	/* Rx Flow Control Enable */;

pub const HV_MUX_DATA_CTRL_GEN_TO_MAC: c_uint = 0x0400;
pub const HV_MUX_DATA_CTRL_FORCE_SPEED: c_uint = 0x0004;
pub const HV_STATS_PAGE: c_int = 778;
// Half-duplex collision counts

pub const E1000_FCRTV_PCH: c_uint = 0x05F40	/* PCH Flow Control Refresh Timer Value */;
pub const E1000_NVM_K1_CONFIG: c_uint = 0x1B	/* NVM K1 Config Word */;
pub const E1000_NVM_K1_ENABLE: c_uint = 0x1	/* NVM Enable K1 bit */;
// SMBus Control Phy Register

pub const CV_SMB_CTRL_FORCE_SMBUS: c_uint = 0x0001;
// I218 Ultra Low Power Configuration 1 Register

pub const I218_ULP_CONFIG1_START: c_uint = 0x0001	/* Start auto ULP config */;
pub const I218_ULP_CONFIG1_IND: c_uint = 0x0004	/* Pwr up from ULP indication */;
pub const I218_ULP_CONFIG1_STICKY_ULP: c_uint = 0x0010	/* Set sticky ULP mode */;
pub const I218_ULP_CONFIG1_INBAND_EXIT: c_uint = 0x0020	/* Inband on ULP exit */;
pub const I218_ULP_CONFIG1_WOL_HOST: c_uint = 0x0040	/* WoL Host on ULP exit */;
pub const I218_ULP_CONFIG1_RESET_TO_SMBUS: c_uint = 0x0100	/* Reset to SMBus mode */;
// enable ULP even if when phy powered down via lanphypc
pub const I218_ULP_CONFIG1_EN_ULP_LANPHYPC: c_uint = 0x0400;
// disable clear of sticky ULP on PERST
pub const I218_ULP_CONFIG1_DIS_CLR_STICKY_ON_PERST: c_uint = 0x0800;
pub const I218_ULP_CONFIG1_DISABLE_SMB_PERST: c_uint = 0x1000	/* Disable on PERST# */;
// SMBus Address Phy Register

pub const HV_SMB_ADDR_MASK: c_uint = 0x007F;
pub const HV_SMB_ADDR_PEC_EN: c_uint = 0x0200;
pub const HV_SMB_ADDR_VALID: c_uint = 0x0080;
pub const HV_SMB_ADDR_FREQ_MASK: c_uint = 0x1100;
pub const HV_SMB_ADDR_FREQ_LOW_SHIFT: c_int = 8;
pub const HV_SMB_ADDR_FREQ_HIGH_SHIFT: c_int = 12;
// Strapping Option Register - RO
pub const E1000_STRAP: c_uint = 0x0000C;
pub const E1000_STRAP_SMBUS_ADDRESS_MASK: c_uint = 0x00FE0000;
pub const E1000_STRAP_SMBUS_ADDRESS_SHIFT: c_int = 17;
pub const E1000_STRAP_SMT_FREQ_MASK: c_uint = 0x00003000;
pub const E1000_STRAP_SMT_FREQ_SHIFT: c_int = 12;
// OEM Bits Phy Register

pub const HV_OEM_BITS_LPLU: c_uint = 0x0004	/* Low Power Link Up */;
pub const HV_OEM_BITS_GBE_DIS: c_uint = 0x0040	/* Gigabit Disable */;
pub const HV_OEM_BITS_RESTART_AN: c_uint = 0x0400	/* Restart Auto-negotiation */;
// KMRN Mode Control

pub const HV_KMRN_MDIO_SLOW: c_uint = 0x0400;
// KMRN FIFO Control and Status

pub const HV_KMRN_FIFO_CTRLSTA_PREAMBLE_MASK: c_uint = 0x7000;
pub const HV_KMRN_FIFO_CTRLSTA_PREAMBLE_SHIFT: c_int = 12;
// PHY Power Management Control

pub const HV_PM_CTRL_K1_CLK_REQ: c_uint = 0x200;
pub const HV_PM_CTRL_K1_ENABLE: c_uint = 0x4000;

pub const I217_PLL_CLOCK_GATE_MASK: c_uint = 0x07FF;
// PHY Timeouts

pub const I217_PHY_TIMEOUTS_K1_EXIT_TO_MASK: c_uint = 0x0FC0;

// Inband Control

pub const I217_INBAND_CTRL_LINK_STAT_TX_TIMEOUT_MASK: c_uint = 0x3F00;
pub const I217_INBAND_CTRL_LINK_STAT_TX_TIMEOUT_SHIFT: c_int = 8;
// Low Power Idle GPIO Control

pub const I217_LPI_GPIO_CTRL_AUTO_EN_LPI: c_uint = 0x0800;
// PHY Low Power Idle Control

pub const I82579_LPI_CTRL_100_ENABLE: c_uint = 0x2000;
pub const I82579_LPI_CTRL_1000_ENABLE: c_uint = 0x4000;
pub const I82579_LPI_CTRL_ENABLE_MASK: c_uint = 0x6000;
pub const I82579_LPI_CTRL_FORCE_PLL_LOCK_COUNT: c_uint = 0x80;
// Extended Management Interface (EMI) Registers
pub const I82579_EMI_ADDR: c_uint = 0x10;
pub const I82579_EMI_DATA: c_uint = 0x11;
pub const I82579_LPI_UPDATE_TIMER: c_uint = 0x4805	/* in 40ns units + 40 ns base value */;
pub const I82579_MSE_THRESHOLD: c_uint = 0x084F	/* 82579 Mean Square Error Threshold */;
pub const I82577_MSE_THRESHOLD: c_uint = 0x0887	/* 82577 Mean Square Error Threshold */;
pub const I82579_MSE_LINK_DOWN: c_uint = 0x2411	/* MSE count before dropping link */;
pub const I82579_RX_CONFIG: c_uint = 0x3412	/* Receive configuration */;
pub const I82579_LPI_PLL_SHUT: c_uint = 0x4412	/* LPI PLL Shut Enable */;
pub const I82579_EEE_PCS_STATUS: c_uint = 0x182E	/* IEEE MMD Register 3.1 >> 8 */;
pub const I82579_EEE_CAPABILITY: c_uint = 0x0410	/* IEEE MMD Register 3.20 */;
pub const I82579_EEE_ADVERTISEMENT: c_uint = 0x040E	/* IEEE MMD Register 7.60 */;
pub const I82579_EEE_LP_ABILITY: c_uint = 0x040F	/* IEEE MMD Register 7.61 */;

pub const I217_EEE_PCS_STATUS: c_uint = 0x9401	/* IEEE MMD Register 3.1 */;
pub const I217_EEE_CAPABILITY: c_uint = 0x8000	/* IEEE MMD Register 3.20 */;
pub const I217_EEE_ADVERTISEMENT: c_uint = 0x8001	/* IEEE MMD Register 7.60 */;
pub const I217_EEE_LP_ABILITY: c_uint = 0x8002	/* IEEE MMD Register 7.61 */;
pub const I217_RX_CONFIG: c_uint = 0xB20C	/* Receive configuration */;
pub const E1000_EEE_RX_LPI_RCVD: c_uint = 0x0400	/* Tx LP idle received */;
pub const E1000_EEE_TX_LPI_RCVD: c_uint = 0x0800	/* Rx LP idle received */;
// Intel Rapid Start Technology Support

pub const I217_PROXY_CTRL_AUTO_DISABLE: c_uint = 0x0080;

pub const I217_SxCTRL_ENABLE_LPI_RESET: c_uint = 0x1000;

pub const I217_CGFREG_ENABLE_MTA_RESET: c_uint = 0x0002;

pub const I217_MEMPWR_DISABLE_SMB_RELEASE: c_uint = 0x0010;
// Receive Address Initial CRC Calculation

// Latency Tolerance Reporting
pub const E1000_LTRV: c_uint = 0x000F8;
pub const E1000_LTRV_VALUE_MASK: c_uint = 0x000003FF;
pub const E1000_LTRV_SCALE_MAX: c_int = 5;
pub const E1000_LTRV_SCALE_FACTOR: c_int = 5;
pub const E1000_LTRV_SCALE_SHIFT: c_int = 10;
pub const E1000_LTRV_SCALE_MASK: c_uint = 0x00001C00;
pub const E1000_LTRV_REQ_SHIFT: c_int = 15;
pub const E1000_LTRV_NOSNOOP_SHIFT: c_int = 16;

// Proprietary Latency Tolerance Reporting PCI Capability
pub const E1000_PCI_LTR_CAP_LPT: c_uint = 0xA8;
// Don't gate wake DMA clock
pub const E1000_FFLT_DBG_DONT_GATE_WAKE_DMA_CLK: c_uint = 0x1000;
extern "C" {
    pub fn e1000e_write_protect_nvm_ich8lan(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_igp3_phy_powerdown_workaround_ich8lan(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_gig_downshift_workaround_ich8lan(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_suspend_workarounds_ich8lan(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_resume_workarounds_pchlan(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_configure_k1_ich8lan(hw: *mut e1000_hw, k1_enable: bool) -> i32;
}
extern "C" {
    pub fn e1000_copy_rx_addrs_to_phy_ich8lan(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_lv_jumbo_workaround_ich8lan(hw: *mut e1000_hw, enable: bool) -> i32;
}
extern "C" {
    pub fn e1000_read_emi_reg_locked(hw: *mut e1000_hw, addr: u16, data: *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_write_emi_reg_locked(hw: *mut e1000_hw, addr: u16, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_set_eee_pchlan(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_enable_ulp_lpt_lp(hw: *mut e1000_hw, to_sx: bool) -> i32;
}
