//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ieee802154/mcr20a.h
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
// Driver for NXP MCR20A 802.15.4 Wireless-PAN Networking controller
//
// Copyright (C) 2018 Xue Liu <liuxuenetmail@gmail.com>
//
// Direct Accress Register
pub const DAR_IRQ_STS1: c_uint = 0x00;
pub const DAR_IRQ_STS2: c_uint = 0x01;
pub const DAR_IRQ_STS3: c_uint = 0x02;
pub const DAR_PHY_CTRL1: c_uint = 0x03;
pub const DAR_PHY_CTRL2: c_uint = 0x04;
pub const DAR_PHY_CTRL3: c_uint = 0x05;
pub const DAR_RX_FRM_LEN: c_uint = 0x06;
pub const DAR_PHY_CTRL4: c_uint = 0x07;
pub const DAR_SRC_CTRL: c_uint = 0x08;
pub const DAR_SRC_ADDRS_SUM_LSB: c_uint = 0x09;
pub const DAR_SRC_ADDRS_SUM_MSB: c_uint = 0x0A;
pub const DAR_CCA1_ED_FNL: c_uint = 0x0B;
pub const DAR_EVENT_TMR_LSB: c_uint = 0x0C;
pub const DAR_EVENT_TMR_MSB: c_uint = 0x0D;
pub const DAR_EVENT_TMR_USB: c_uint = 0x0E;
pub const DAR_TIMESTAMP_LSB: c_uint = 0x0F;
pub const DAR_TIMESTAMP_MSB: c_uint = 0x10;
pub const DAR_TIMESTAMP_USB: c_uint = 0x11;
pub const DAR_T3CMP_LSB: c_uint = 0x12;
pub const DAR_T3CMP_MSB: c_uint = 0x13;
pub const DAR_T3CMP_USB: c_uint = 0x14;
pub const DAR_T2PRIMECMP_LSB: c_uint = 0x15;
pub const DAR_T2PRIMECMP_MSB: c_uint = 0x16;
pub const DAR_T1CMP_LSB: c_uint = 0x17;
pub const DAR_T1CMP_MSB: c_uint = 0x18;
pub const DAR_T1CMP_USB: c_uint = 0x19;
pub const DAR_T2CMP_LSB: c_uint = 0x1A;
pub const DAR_T2CMP_MSB: c_uint = 0x1B;
pub const DAR_T2CMP_USB: c_uint = 0x1C;
pub const DAR_T4CMP_LSB: c_uint = 0x1D;
pub const DAR_T4CMP_MSB: c_uint = 0x1E;
pub const DAR_T4CMP_USB: c_uint = 0x1F;
pub const DAR_PLL_INT0: c_uint = 0x20;
pub const DAR_PLL_FRAC0_LSB: c_uint = 0x21;
pub const DAR_PLL_FRAC0_MSB: c_uint = 0x22;
pub const DAR_PA_PWR: c_uint = 0x23;
pub const DAR_SEQ_STATE: c_uint = 0x24;
pub const DAR_LQI_VALUE: c_uint = 0x25;
pub const DAR_RSSI_CCA_CONT: c_uint = 0x26;
// ------------------            0x27
pub const DAR_ASM_CTRL1: c_uint = 0x28;
pub const DAR_ASM_CTRL2: c_uint = 0x29;
pub const DAR_ASM_DATA_0: c_uint = 0x2A;
pub const DAR_ASM_DATA_1: c_uint = 0x2B;
pub const DAR_ASM_DATA_2: c_uint = 0x2C;
pub const DAR_ASM_DATA_3: c_uint = 0x2D;
pub const DAR_ASM_DATA_4: c_uint = 0x2E;
pub const DAR_ASM_DATA_5: c_uint = 0x2F;
pub const DAR_ASM_DATA_6: c_uint = 0x30;
pub const DAR_ASM_DATA_7: c_uint = 0x31;
pub const DAR_ASM_DATA_8: c_uint = 0x32;
pub const DAR_ASM_DATA_9: c_uint = 0x33;
pub const DAR_ASM_DATA_A: c_uint = 0x34;
pub const DAR_ASM_DATA_B: c_uint = 0x35;
pub const DAR_ASM_DATA_C: c_uint = 0x36;
pub const DAR_ASM_DATA_D: c_uint = 0x37;
pub const DAR_ASM_DATA_E: c_uint = 0x38;
pub const DAR_ASM_DATA_F: c_uint = 0x39;
// -----------------------       0x3A
pub const DAR_OVERWRITE_VER: c_uint = 0x3B;
pub const DAR_CLK_OUT_CTRL: c_uint = 0x3C;
pub const DAR_PWR_MODES: c_uint = 0x3D;
pub const IAR_INDEX: c_uint = 0x3E;
pub const IAR_DATA: c_uint = 0x3F;
// Indirect Resgister Memory
pub const IAR_PART_ID: c_uint = 0x00;
pub const IAR_XTAL_TRIM: c_uint = 0x01;
pub const IAR_PMC_LP_TRIM: c_uint = 0x02;
pub const IAR_MACPANID0_LSB: c_uint = 0x03;
pub const IAR_MACPANID0_MSB: c_uint = 0x04;
pub const IAR_MACSHORTADDRS0_LSB: c_uint = 0x05;
pub const IAR_MACSHORTADDRS0_MSB: c_uint = 0x06;
pub const IAR_MACLONGADDRS0_0: c_uint = 0x07;
pub const IAR_MACLONGADDRS0_8: c_uint = 0x08;
pub const IAR_MACLONGADDRS0_16: c_uint = 0x09;
pub const IAR_MACLONGADDRS0_24: c_uint = 0x0A;
pub const IAR_MACLONGADDRS0_32: c_uint = 0x0B;
pub const IAR_MACLONGADDRS0_40: c_uint = 0x0C;
pub const IAR_MACLONGADDRS0_48: c_uint = 0x0D;
pub const IAR_MACLONGADDRS0_56: c_uint = 0x0E;
pub const IAR_RX_FRAME_FILTER: c_uint = 0x0F;
pub const IAR_PLL_INT1: c_uint = 0x10;
pub const IAR_PLL_FRAC1_LSB: c_uint = 0x11;
pub const IAR_PLL_FRAC1_MSB: c_uint = 0x12;
pub const IAR_MACPANID1_LSB: c_uint = 0x13;
pub const IAR_MACPANID1_MSB: c_uint = 0x14;
pub const IAR_MACSHORTADDRS1_LSB: c_uint = 0x15;
pub const IAR_MACSHORTADDRS1_MSB: c_uint = 0x16;
pub const IAR_MACLONGADDRS1_0: c_uint = 0x17;
pub const IAR_MACLONGADDRS1_8: c_uint = 0x18;
pub const IAR_MACLONGADDRS1_16: c_uint = 0x19;
pub const IAR_MACLONGADDRS1_24: c_uint = 0x1A;
pub const IAR_MACLONGADDRS1_32: c_uint = 0x1B;
pub const IAR_MACLONGADDRS1_40: c_uint = 0x1C;
pub const IAR_MACLONGADDRS1_48: c_uint = 0x1D;
pub const IAR_MACLONGADDRS1_56: c_uint = 0x1E;
pub const IAR_DUAL_PAN_CTRL: c_uint = 0x1F;
pub const IAR_DUAL_PAN_DWELL: c_uint = 0x20;
pub const IAR_DUAL_PAN_STS: c_uint = 0x21;
pub const IAR_CCA1_THRESH: c_uint = 0x22;
pub const IAR_CCA1_ED_OFFSET_COMP: c_uint = 0x23;
pub const IAR_LQI_OFFSET_COMP: c_uint = 0x24;
pub const IAR_CCA_CTRL: c_uint = 0x25;
pub const IAR_CCA2_CORR_PEAKS: c_uint = 0x26;
pub const IAR_CCA2_CORR_THRESH: c_uint = 0x27;
pub const IAR_TMR_PRESCALE: c_uint = 0x28;
// --------------------          0x29
pub const IAR_GPIO_DATA: c_uint = 0x2A;
pub const IAR_GPIO_DIR: c_uint = 0x2B;
pub const IAR_GPIO_PUL_EN: c_uint = 0x2C;
pub const IAR_GPIO_PUL_SEL: c_uint = 0x2D;
pub const IAR_GPIO_DS: c_uint = 0x2E;
// ------------------            0x2F
pub const IAR_ANT_PAD_CTRL: c_uint = 0x30;
pub const IAR_MISC_PAD_CTRL: c_uint = 0x31;
pub const IAR_BSM_CTRL: c_uint = 0x32;
// -------------------           0x33
pub const IAR_RNG: c_uint = 0x34;
pub const IAR_RX_BYTE_COUNT: c_uint = 0x35;
pub const IAR_RX_WTR_MARK: c_uint = 0x36;
pub const IAR_SOFT_RESET: c_uint = 0x37;
pub const IAR_TXDELAY: c_uint = 0x38;
pub const IAR_ACKDELAY: c_uint = 0x39;
pub const IAR_SEQ_MGR_CTRL: c_uint = 0x3A;
pub const IAR_SEQ_MGR_STS: c_uint = 0x3B;
pub const IAR_SEQ_T_STS: c_uint = 0x3C;
pub const IAR_ABORT_STS: c_uint = 0x3D;
pub const IAR_CCCA_BUSY_CNT: c_uint = 0x3E;
pub const IAR_SRC_ADDR_CHECKSUM1: c_uint = 0x3F;
pub const IAR_SRC_ADDR_CHECKSUM2: c_uint = 0x40;
pub const IAR_SRC_TBL_VALID1: c_uint = 0x41;
pub const IAR_SRC_TBL_VALID2: c_uint = 0x42;
pub const IAR_FILTERFAIL_CODE1: c_uint = 0x43;
pub const IAR_FILTERFAIL_CODE2: c_uint = 0x44;
pub const IAR_SLOT_PRELOAD: c_uint = 0x45;
// --------------------          0x46
pub const IAR_CORR_VT: c_uint = 0x47;
pub const IAR_SYNC_CTRL: c_uint = 0x48;
pub const IAR_PN_LSB_0: c_uint = 0x49;
pub const IAR_PN_LSB_1: c_uint = 0x4A;
pub const IAR_PN_MSB_0: c_uint = 0x4B;
pub const IAR_PN_MSB_1: c_uint = 0x4C;
pub const IAR_CORR_NVAL: c_uint = 0x4D;
pub const IAR_TX_MODE_CTRL: c_uint = 0x4E;
pub const IAR_SNF_THR: c_uint = 0x4F;
pub const IAR_FAD_THR: c_uint = 0x50;
pub const IAR_ANT_AGC_CTRL: c_uint = 0x51;
pub const IAR_AGC_THR1: c_uint = 0x52;
pub const IAR_AGC_THR2: c_uint = 0x53;
pub const IAR_AGC_HYS: c_uint = 0x54;
pub const IAR_AFC: c_uint = 0x55;
// -------------------           0x56
// -------------------           0x57
pub const IAR_PHY_STS: c_uint = 0x58;
pub const IAR_RX_MAX_CORR: c_uint = 0x59;
pub const IAR_RX_MAX_PREAMBLE: c_uint = 0x5A;
pub const IAR_RSSI: c_uint = 0x5B;
// -------------------           0x5C
// -------------------           0x5D
pub const IAR_PLL_DIG_CTRL: c_uint = 0x5E;
pub const IAR_VCO_CAL: c_uint = 0x5F;
pub const IAR_VCO_BEST_DIFF: c_uint = 0x60;
pub const IAR_VCO_BIAS: c_uint = 0x61;
pub const IAR_KMOD_CTRL: c_uint = 0x62;
pub const IAR_KMOD_CAL: c_uint = 0x63;
pub const IAR_PA_CAL: c_uint = 0x64;
pub const IAR_PA_PWRCAL: c_uint = 0x65;
pub const IAR_ATT_RSSI1: c_uint = 0x66;
pub const IAR_ATT_RSSI2: c_uint = 0x67;
pub const IAR_RSSI_OFFSET: c_uint = 0x68;
pub const IAR_RSSI_SLOPE: c_uint = 0x69;
pub const IAR_RSSI_CAL1: c_uint = 0x6A;
pub const IAR_RSSI_CAL2: c_uint = 0x6B;
// -------------------           0x6C
// -------------------           0x6D
pub const IAR_XTAL_CTRL: c_uint = 0x6E;
pub const IAR_XTAL_COMP_MIN: c_uint = 0x6F;
pub const IAR_XTAL_COMP_MAX: c_uint = 0x70;
pub const IAR_XTAL_GM: c_uint = 0x71;
// -------------------           0x72
// -------------------           0x73
pub const IAR_LNA_TUNE: c_uint = 0x74;
pub const IAR_LNA_AGCGAIN: c_uint = 0x75;
// -------------------           0x76
// -------------------           0x77
pub const IAR_CHF_PMA_GAIN: c_uint = 0x78;
pub const IAR_CHF_IBUF: c_uint = 0x79;
pub const IAR_CHF_QBUF: c_uint = 0x7A;
pub const IAR_CHF_IRIN: c_uint = 0x7B;
pub const IAR_CHF_QRIN: c_uint = 0x7C;
pub const IAR_CHF_IL: c_uint = 0x7D;
pub const IAR_CHF_QL: c_uint = 0x7E;
pub const IAR_CHF_CC1: c_uint = 0x7F;
pub const IAR_CHF_CCL: c_uint = 0x80;
pub const IAR_CHF_CC2: c_uint = 0x81;
pub const IAR_CHF_IROUT: c_uint = 0x82;
pub const IAR_CHF_QROUT: c_uint = 0x83;
// -------------------           0x84
// -------------------           0x85
pub const IAR_RSSI_CTRL: c_uint = 0x86;
// -------------------           0x87
// -------------------           0x88
pub const IAR_PA_BIAS: c_uint = 0x89;
pub const IAR_PA_TUNING: c_uint = 0x8A;
// -------------------           0x8B
// -------------------           0x8C
pub const IAR_PMC_HP_TRIM: c_uint = 0x8D;
pub const IAR_VREGA_TRIM: c_uint = 0x8E;
// -------------------           0x8F
// -------------------           0x90
pub const IAR_VCO_CTRL1: c_uint = 0x91;
pub const IAR_VCO_CTRL2: c_uint = 0x92;
// -------------------           0x93
// -------------------           0x94
pub const IAR_ANA_SPARE_OUT1: c_uint = 0x95;
pub const IAR_ANA_SPARE_OUT2: c_uint = 0x96;
pub const IAR_ANA_SPARE_IN: c_uint = 0x97;
pub const IAR_MISCELLANEOUS: c_uint = 0x98;
// -------------------           0x99
pub const IAR_SEQ_MGR_OVRD0: c_uint = 0x9A;
pub const IAR_SEQ_MGR_OVRD1: c_uint = 0x9B;
pub const IAR_SEQ_MGR_OVRD2: c_uint = 0x9C;
pub const IAR_SEQ_MGR_OVRD3: c_uint = 0x9D;
pub const IAR_SEQ_MGR_OVRD4: c_uint = 0x9E;
pub const IAR_SEQ_MGR_OVRD5: c_uint = 0x9F;
pub const IAR_SEQ_MGR_OVRD6: c_uint = 0xA0;
pub const IAR_SEQ_MGR_OVRD7: c_uint = 0xA1;
// -------------------           0xA2
pub const IAR_TESTMODE_CTRL: c_uint = 0xA3;
pub const IAR_DTM_CTRL1: c_uint = 0xA4;
pub const IAR_DTM_CTRL2: c_uint = 0xA5;
pub const IAR_ATM_CTRL1: c_uint = 0xA6;
pub const IAR_ATM_CTRL2: c_uint = 0xA7;
pub const IAR_ATM_CTRL3: c_uint = 0xA8;
// -------------------           0xA9
pub const IAR_LIM_FE_TEST_CTRL: c_uint = 0xAA;
pub const IAR_CHF_TEST_CTRL: c_uint = 0xAB;
pub const IAR_VCO_TEST_CTRL: c_uint = 0xAC;
pub const IAR_PLL_TEST_CTRL: c_uint = 0xAD;
pub const IAR_PA_TEST_CTRL: c_uint = 0xAE;
pub const IAR_PMC_TEST_CTRL: c_uint = 0xAF;
pub const IAR_SCAN_DTM_PROTECT_1: c_uint = 0xFE;
pub const IAR_SCAN_DTM_PROTECT_0: c_uint = 0xFF;
// IRQSTS1 bits

// IRQSTS2 bits

// IRQSTS3 bits

// PHY_CTRL1 bits

pub const DAR_PHY_CTRL1_CCABFRTX_SHIFT: c_int = 5;

pub const DAR_PHY_CTRL1_XCVSEQ_MASK: c_uint = 0x07;
// PHY_CTRL2 bits

// PHY_CTRL3 bits

// RX_FRM_LEN bits

// PHY_CTRL4 bits

// SRC_CTRL bits

// DAR_ASM_CTRL1 bits

// DAR_ASM_CTRL2 bits

// DAR_CLK_OUT_CTRL bits

// DAR_PWR_MODES bits

// RX_FRAME_FILTER bits

// DUAL_PAN_CTRL bits

// DUAL_PAN_STS bits

// CCA_CTRL bits

// ANT_PAD_CTRL bits

// MISC_PAD_CTRL bits

// ANT_AGC_CTRL bits

// BSM_CTRL bits

// SOFT_RESET bits

// SEQ_MGR_CTRL bits

// SEQ_MGR_STS bits

// ABORT_STS bits

// IAR_FILTERFAIL_CODE2 bits

// PHY_STS bits

// TESTMODE_CTRL bits

// DTM_CTRL1 bits

// TX_MODE_CTRL

