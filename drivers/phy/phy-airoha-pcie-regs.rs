//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/phy-airoha-pcie-regs.h
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
//
// Copyright (c) 2024 AIROHA Inc
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
//
// CSR_2L
pub const REG_CSR_2L_CMN: c_uint = 0x0000;

pub const REG_CSR_2L_JCPLL_IB_EXT: c_uint = 0x0004;

pub const REG_CSR_2L_JCPLL_LPF_BR: c_uint = 0x0008;

pub const REG_CSR_2L_JCPLL_LPF_BWC: c_uint = 0x000c;

pub const REG_CSR_2L_JCPLL_KBAND_KFC: c_uint = 0x0010;

pub const REG_CSR_2L_JCPLL_MMD_PREDIV_MODE: c_uint = 0x0014;

pub const CSR_2L_PXP_JCPLL_MONCK: c_uint = 0x0018;

pub const REG_CSR_2L_JCPLL_RST_DLY: c_uint = 0x001c;

pub const REG_CSR_2L_JCPLL_SDM_IFM: c_uint = 0x0020;

pub const REG_CSR_2L_JCPLL_SDM_HREN: c_uint = 0x0024;

pub const REG_CSR_2L_JCPLL_TCL_CMP: c_uint = 0x0028;

pub const REG_CSR_2L_JCPLL_VCODIV: c_uint = 0x002c;

pub const REG_CSR_2L_JCPLL_VCO_TCLVAR: c_uint = 0x0030;

pub const REG_CSR_2L_JCPLL_SSC: c_uint = 0x0038;

pub const REG_CSR_2L_JCPLL_SSC_DELTA1: c_uint = 0x003c;

pub const REG_CSR_2L_JCPLL_SSC_PERIOD: c_uint = 0x0040;

pub const REG_CSR_2L_JCPLL_TCL_VTP_EN: c_uint = 0x004c;

pub const REG_CSR_2L_JCPLL_TCL_KBAND_VREF: c_uint = 0x0050;

pub const REG_CSR_2L_750M_SYS_CK: c_uint = 0x0054;

pub const REG_CSR_2L_TXPLL_CHP_IOFST: c_uint = 0x0058;

pub const REG_CSR_2L_TXPLL_LPF_BWR: c_uint = 0x005c;

pub const REG_CSR_2L_TXPLL_KBAND_DIV: c_uint = 0x0060;

pub const REG_CSR_2L_TXPLL_POSTDIV: c_uint = 0x0064;

pub const REG_CSR_2L_TXPLL_PHY_CK2: c_uint = 0x0068;

pub const REG_CSR_2L_TXPLL_REFIN_DIV: c_uint = 0x006c;

pub const REG_CSR_2L_TXPLL_SDM_DI_LS: c_uint = 0x0070;

pub const REG_CSR_2L_TXPLL_SDM_OUT: c_uint = 0x0074;

pub const REG_CSR_2L_TXPLL_TCL_AMP_VREF: c_uint = 0x0078;

pub const REG_CSR_2L_TXPLL_TCL_LPF_BW: c_uint = 0x007c;

pub const REG_CSR_2L_TXPLL_VCO_SCAPWR: c_uint = 0x0080;

pub const REG_CSR_2L_TXPLL_SSC: c_uint = 0x0084;

pub const REG_CSR_2L_TXPLL_SSC_DELTA1: c_uint = 0x0088;

pub const REG_CSR_2L_TXPLL_SSC_PERIOD: c_uint = 0x008c;

pub const REG_CSR_2L_TXPLL_VTP: c_uint = 0x0090;

pub const REG_CSR_2L_TXPLL_TCL_VTP: c_uint = 0x0098;

pub const REG_CSR_2L_TXPLL_TCL_KBAND_VREF: c_uint = 0x009c;

pub const REG_CSR_2L_TXPLL_POSTDIV_D256: c_uint = 0x00a0;

pub const REG_CSR_2L_CLKTX0_FORCE_OUT1: c_uint = 0x00a4;

pub const REG_CSR_2L_CLKTX1_OFFSET: c_uint = 0x00a8;

pub const REG_CSR_2L_CLKTX1_IMP_SEL: c_uint = 0x00ac;

pub const REG_CSR_2L_PLL_CMN_RESERVE0: c_uint = 0x00b0;

pub const REG_CSR_2L_TX0_CKLDO: c_uint = 0x00cc;

pub const REG_CSR_2L_TX1_CKLDO: c_uint = 0x00e8;

pub const REG_CSR_2L_TX1_MULTLANE: c_uint = 0x00ec;

pub const REG_CSR_2L_RX0_REV0: c_uint = 0x00fc;

pub const REG_CSR_2L_RX0_PHYCK_DIV: c_uint = 0x0100;

pub const REG_CSR_2L_CDR0_PD_PICAL_CKD8_INV: c_uint = 0x0104;

pub const REG_CSR_2L_CDR0_LPF_RATIO: c_uint = 0x0110;

pub const REG_CSR_2L_CDR0_PR_INJ_MODE: c_uint = 0x011c;

pub const REG_CSR_2L_CDR0_PR_BETA_DAC: c_uint = 0x0120;

pub const REG_CSR_2L_CDR0_PR_VREG_IBAND: c_uint = 0x0124;

pub const REG_CSR_2L_CDR0_PR_CKREF_DIV: c_uint = 0x0128;

pub const REG_CSR_2L_CDR0_PR_MONCK: c_uint = 0x012c;

pub const REG_CSR_2L_CDR0_PR_COR_HBW: c_uint = 0x0130;

pub const REG_CSR_2L_CDR0_PR_MONPI: c_uint = 0x0134;

pub const REG_CSR_2L_RX0_SIGDET_DCTEST: c_uint = 0x0140;

pub const REG_CSR_2L_RX0_SIGDET_VTH_SEL: c_uint = 0x0144;

pub const REG_CSR_2L_PXP_RX0_FE_VB_EQ2: c_uint = 0x0148;

pub const REG_CSR_2L_PXP_RX0_OSCAL_CTLE1IOS: c_uint = 0x0158;

pub const REG_CSR_2L_PXP_RX0_OSCA_VGA1VOS: c_uint = 0x015c;

pub const REG_CSR_2L_RX1_REV0: c_uint = 0x01b4;
pub const REG_CSR_2L_RX1_PHYCK_DIV: c_uint = 0x01b8;

pub const REG_CSR_2L_CDR1_PD_PICAL_CKD8_INV: c_uint = 0x01bc;

pub const REG_CSR_2L_CDR1_PR_BETA_DAC: c_uint = 0x01d8;

pub const REG_CSR_2L_CDR1_PR_MONCK: c_uint = 0x01e4;

pub const REG_CSR_2L_CDR1_LPF_RATIO: c_uint = 0x01c8;

pub const REG_CSR_2L_CDR1_PR_INJ_MODE: c_uint = 0x01d4;

pub const REG_CSR_2L_CDR1_PR_VREG_IBAND_VAL: c_uint = 0x01dc;

pub const REG_CSR_2L_CDR1_PR_CKREF_DIV: c_uint = 0x01e0;

pub const REG_CSR_2L_CDR1_PR_COR_HBW: c_uint = 0x01e8;

pub const REG_CSR_2L_CDR1_PR_MONPI: c_uint = 0x01ec;

pub const REG_CSR_2L_RX1_DAC_RANGE_EYE: c_uint = 0x01f4;

pub const REG_CSR_2L_RX1_SIGDET_NOVTH: c_uint = 0x01f8;

pub const REG_CSR_2L_RX1_FE_VB_EQ1: c_uint = 0x0200;

pub const REG_CSR_2L_RX1_OSCAL_VGA1IOS: c_uint = 0x0214;

// PMA
pub const REG_PCIE_PMA_SS_LCPLL_PWCTL_SETTING_1: c_uint = 0x0004;

pub const REG_PCIE_PMA_SEQUENCE_DISB_CTRL1: c_uint = 0x010c;

pub const REG_PCIE_PMA_CTRL_SEQUENCE_FORCE_CTRL1: c_uint = 0x0114;

pub const REG_PCIE_PMA_SS_RX_FREQ_DET1: c_uint = 0x014c;

pub const REG_PCIE_PMA_SS_RX_FREQ_DET2: c_uint = 0x0150;

pub const REG_PCIE_PMA_SS_RX_FREQ_DET3: c_uint = 0x0154;

pub const REG_PCIE_PMA_SS_RX_FREQ_DET4: c_uint = 0x0158;

pub const REG_PCIE_PMA_SS_RX_CAL1: c_uint = 0x0160;
pub const REG_PCIE_PMA_SS_RX_CAL2: c_uint = 0x0164;

pub const REG_PCIE_PMA_SS_RX_SIGDET0: c_uint = 0x0168;

pub const REG_PCIE_PMA_TX_RESET: c_uint = 0x0260;

pub const REG_PCIE_PMA_RX_FORCE_MODE0: c_uint = 0x0294;

pub const REG_PCIE_PMA_SS_DA_XPON_PWDB0: c_uint = 0x034c;

pub const REG_PCIE_PMA_SW_RESET: c_uint = 0x0460;

pub const REG_PCIE_PMA_RO_RX_FREQDET: c_uint = 0x0530;

pub const REG_PCIE_PMA_FORCE_DA_PXP_CDR_PR_IDAC: c_uint = 0x0794;

pub const REG_PCIE_PMA_FORCE_DA_PXP_TXPLL_SDM_PCW: c_uint = 0x0798;

pub const REG_PCIE_PMA_FORCE_DA_PXP_RX_FE_VOS: c_uint = 0x079c;

pub const REG_PCIE_PMA_FORCE_DA_PXP_JCPLL_SDM_PCW: c_uint = 0x0800;

pub const REG_PCIE_PMA_FORCE_DA_PXP_CDR_PD_PWDB: c_uint = 0x081c;

pub const REG_PCIE_PMA_FORCE_DA_PXP_CDR_PR_LPF_C: c_uint = 0x0820;

pub const REG_PCIE_PMA_FORCE_DA_PXP_CDR_PR_PIEYE_PWDB: c_uint = 0x0824;

pub const REG_PCIE_PMA_FORCE_PXP_JCPLL_CKOUT: c_uint = 0x0828;

pub const REG_PCIE_PMA_FORCE_DA_PXP_RX_SCAN_RST: c_uint = 0x0084c;

pub const REG_PCIE_PMA_FORCE_DA_PXP_TXPLL_CKOUT: c_uint = 0x0854;

pub const REG_PCIE_PMA_SCAN_MODE: c_uint = 0x0884;

pub const REG_PCIE_PMA_DIG_RESERVE_13: c_uint = 0x08bc;

pub const REG_PCIE_PMA_DIG_RESERVE_14: c_uint = 0x08c0;

pub const REG_PCIE_PMA_FORCE_DA_PXP_RX_FE_GAIN_CTRL: c_uint = 0x088c;

pub const REG_PCIE_PMA_FORCE_DA_PXP_RX_FE_PWDB: c_uint = 0x0894;

pub const REG_PCIE_PMA_DIG_RESERVE_12: c_uint = 0x08b8;

pub const REG_PCIE_PMA_DIG_RESERVE_17: c_uint = 0x08e0;
pub const REG_PCIE_PMA_DIG_RESERVE_18: c_uint = 0x08e4;

pub const REG_PCIE_PMA_DIG_RESERVE_19: c_uint = 0x08e8;

pub const REG_PCIE_PMA_DIG_RESERVE_20: c_uint = 0x08ec;

pub const REG_PCIE_PMA_DIG_RESERVE_21: c_uint = 0x08f0;
pub const REG_PCIE_PMA_DIG_RESERVE_22: c_uint = 0x08f4;
pub const REG_PCIE_PMA_DIG_RESERVE_27: c_uint = 0x0908;
pub const REG_PCIE_PMA_DIG_RESERVE_30: c_uint = 0x0914;
// DTIME
pub const REG_PCIE_PEXTP_DIG_GLB44: c_uint = 0x00;

// RX AEQ
pub const REG_PCIE_PEXTP_DIG_LN_RX30_P0: c_uint = 0x0000;

pub const REG_PCIE_PEXTP_DIG_LN_RX30_P1: c_uint = 0x0100;
