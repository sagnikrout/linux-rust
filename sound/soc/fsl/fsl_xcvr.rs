//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_xcvr.h
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
//
// NXP XCVR ALSA SoC Digital Audio Interface (DAI) driver
//
// Copyright 2019 NXP
//
pub const FSL_XCVR_MODE_SPDIF: c_int = 0;
pub const FSL_XCVR_MODE_ARC: c_int = 1;
pub const FSL_XCVR_MODE_EARC: c_int = 2;
// XCVR Registers
pub const FSL_XCVR_REG_OFFSET: c_uint = 0x800 /* regs offset */;
pub const FSL_XCVR_FIFO_SIZE: c_uint = 0x80  /* 128 */;

pub const FSL_XCVR_RX_FIFO_ADDR: c_uint = 0x0C00;
pub const FSL_XCVR_TX_FIFO_ADDR: c_uint = 0x0E00;
pub const FSL_XCVR_VERSION: c_uint = 0x00  /* Version */;
pub const FSL_XCVR_EXT_CTRL: c_uint = 0x10  /* Control */;
pub const FSL_XCVR_EXT_STATUS: c_uint = 0x20  /* Status */;
pub const FSL_XCVR_EXT_IER0: c_uint = 0x30  /* Interrupt en 0 */;
pub const FSL_XCVR_EXT_IER1: c_uint = 0x40  /* Interrupt en 1 */;
pub const FSL_XCVR_EXT_ISR: c_uint = 0x50  /* Interrupt status */;
pub const FSL_XCVR_EXT_ISR_SET: c_uint = 0x54  /* Interrupt status */;
pub const FSL_XCVR_EXT_ISR_CLR: c_uint = 0x58  /* Interrupt status */;
pub const FSL_XCVR_EXT_ISR_TOG: c_uint = 0x5C  /* Interrupt status */;
pub const FSL_XCVR_IER: c_uint = 0x70  /* Interrupt en for M0+ */;
pub const FSL_XCVR_ISR: c_uint = 0x80  /* Interrupt status */;
pub const FSL_XCVR_ISR_SET: c_uint = 0x84  /* Interrupt status set */;
pub const FSL_XCVR_ISR_CLR: c_uint = 0x88  /* Interrupt status clear */;
pub const FSL_XCVR_ISR_TOG: c_uint = 0x8C  /* Interrupt status toggle */;
pub const FSL_XCVR_PHY_AI_CTRL: c_uint = 0x90;
pub const FSL_XCVR_PHY_AI_CTRL_SET: c_uint = 0x94;
pub const FSL_XCVR_PHY_AI_CTRL_CLR: c_uint = 0x98;
pub const FSL_XCVR_PHY_AI_CTRL_TOG: c_uint = 0x9C;
pub const FSL_XCVR_PHY_AI_WDATA: c_uint = 0xA0;
pub const FSL_XCVR_PHY_AI_RDATA: c_uint = 0xA4;
pub const FSL_XCVR_CLK_CTRL: c_uint = 0xB0;
pub const FSL_XCVR_RX_DPTH_CTRL: c_uint = 0x180 /* RX datapath ctrl reg */;
pub const FSL_XCVR_RX_DPTH_CTRL_SET: c_uint = 0x184;
pub const FSL_XCVR_RX_DPTH_CTRL_CLR: c_uint = 0x188;
pub const FSL_XCVR_RX_DPTH_CTRL_TOG: c_uint = 0x18c;
pub const FSL_XCVR_RX_CS_DATA_0: c_uint = 0x190;
pub const FSL_XCVR_RX_CS_DATA_1: c_uint = 0x194;
pub const FSL_XCVR_RX_CS_DATA_2: c_uint = 0x198;
pub const FSL_XCVR_RX_CS_DATA_3: c_uint = 0x19C;
pub const FSL_XCVR_RX_CS_DATA_4: c_uint = 0x1A0;
pub const FSL_XCVR_RX_CS_DATA_5: c_uint = 0x1A4;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL: c_uint = 0x1C0;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_SET: c_uint = 0x1C4;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR: c_uint = 0x1C8;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG: c_uint = 0x1CC;
pub const FSL_XCVR_RX_DPTH_TSCR: c_uint = 0x1D0;
pub const FSL_XCVR_RX_DPTH_BCR: c_uint = 0x1D4;
pub const FSL_XCVR_RX_DPTH_BCTR: c_uint = 0x1D8;
pub const FSL_XCVR_RX_DPTH_BCRR: c_uint = 0x1DC;
pub const FSL_XCVR_TX_DPTH_CTRL: c_uint = 0x220 /* TX datapath ctrl reg */;
pub const FSL_XCVR_TX_DPTH_CTRL_SET: c_uint = 0x224;
pub const FSL_XCVR_TX_DPTH_CTRL_CLR: c_uint = 0x228;
pub const FSL_XCVR_TX_DPTH_CTRL_TOG: c_uint = 0x22C;
pub const FSL_XCVR_TX_CS_DATA_0: c_uint = 0x230 /* TX channel status bits regs */;
pub const FSL_XCVR_TX_CS_DATA_1: c_uint = 0x234;
pub const FSL_XCVR_TX_CS_DATA_2: c_uint = 0x238;
pub const FSL_XCVR_TX_CS_DATA_3: c_uint = 0x23C;
pub const FSL_XCVR_TX_CS_DATA_4: c_uint = 0x240;
pub const FSL_XCVR_TX_CS_DATA_5: c_uint = 0x244;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL: c_uint = 0x260;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_SET: c_uint = 0x264;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR: c_uint = 0x268;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG: c_uint = 0x26C;
pub const FSL_XCVR_TX_DPTH_TSCR: c_uint = 0x270;
pub const FSL_XCVR_TX_DPTH_BCR: c_uint = 0x274;
pub const FSL_XCVR_TX_DPTH_BCTR: c_uint = 0x278;
pub const FSL_XCVR_TX_DPTH_BCRR: c_uint = 0x27C;
pub const FSL_XCVR_DEBUG_REG_0: c_uint = 0x2E0;
pub const FSL_XCVR_DEBUG_REG_1: c_uint = 0x2F0;

pub const FSL_XCVR_EXT_CTRL_TX_FWM_SHFT: c_int = 0;

pub const FSL_XCVR_EXT_CTRL_RX_FWM_SHFT: c_int = 8;

pub const FSL_XCVR_EXT_CTRL_PAGE_SHFT: c_int = 16;

pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TSEN_SHIFT: c_int = 0;

pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TSINC_SHIFT: c_int = 1;

pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_RBC_SHIFT: c_int = 8;

pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_RTSC_SHIFT: c_int = 9;

pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TSEN_SHIFT: c_int = 0;

pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TSINC_SHIFT: c_int = 1;

pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_RBC_SHIFT: c_int = 8;

pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_RTSC_SHIFT: c_int = 9;

pub const FSL_XCVR_PLL_CTRL0: c_uint = 0x00;
pub const FSL_XCVR_PLL_CTRL0_SET: c_uint = 0x04;
pub const FSL_XCVR_PLL_CTRL0_CLR: c_uint = 0x08;
pub const FSL_XCVR_PLL_NUM: c_uint = 0x20;
pub const FSL_XCVR_PLL_DEN: c_uint = 0x30;
pub const FSL_XCVR_PLL_PDIV: c_uint = 0x40;
pub const FSL_XCVR_PLL_BANDGAP: c_uint = 0x50;
pub const FSL_XCVR_PLL_BANDGAP_SET: c_uint = 0x54;
pub const FSL_XCVR_PLL_STAT0: c_uint = 0x60;
pub const FSL_XCVR_PLL_STAT0_TOG: c_uint = 0x6c;
pub const FSL_XCVR_PHY_CTRL: c_uint = 0x00;
pub const FSL_XCVR_PHY_CTRL_SET: c_uint = 0x04;
pub const FSL_XCVR_PHY_CTRL_CLR: c_uint = 0x08;
pub const FSL_XCVR_PHY_CTRL_TOG: c_uint = 0x0c;
pub const FSL_XCVR_PHY_STATUS: c_uint = 0x10;
pub const FSL_XCVR_PHY_ANALOG_TRIM: c_uint = 0x20;
pub const FSL_XCVR_PHY_SLEW_RATE_TRIM: c_uint = 0x30;
pub const FSL_XCVR_PHY_DATA_TEST_DELAY: c_uint = 0x40;
pub const FSL_XCVR_PHY_TEST_CTRL: c_uint = 0x50;
pub const FSL_XCVR_PHY_DIFF_CDR_CTRL: c_uint = 0x60;
pub const FSL_XCVR_PHY_CTRL2: c_uint = 0x70;
pub const FSL_XCVR_PHY_CTRL2_SET: c_uint = 0x74;
pub const FSL_XCVR_PHY_CTRL2_CLR: c_uint = 0x78;
pub const FSL_XCVR_PHY_CTRL2_TOG: c_uint = 0x7c;

pub const FSL_XCVR_CS_DATA_0_FS_32000: c_uint = 0x3000000;
pub const FSL_XCVR_CS_DATA_0_FS_44100: c_uint = 0x0000000;
pub const FSL_XCVR_CS_DATA_0_FS_48000: c_uint = 0x2000000;
pub const FSL_XCVR_CS_DATA_0_FS_64000: c_uint = 0xB000000;
pub const FSL_XCVR_CS_DATA_0_FS_88200: c_uint = 0x8000000;
pub const FSL_XCVR_CS_DATA_0_FS_96000: c_uint = 0xA000000;
pub const FSL_XCVR_CS_DATA_0_FS_176400: c_uint = 0xC000000;
pub const FSL_XCVR_CS_DATA_0_FS_192000: c_uint = 0xE000000;
pub const FSL_XCVR_CS_DATA_0_CH_MASK: c_uint = 0x3A;
pub const FSL_XCVR_CS_DATA_0_CH_U2LPCM: c_uint = 0x00;
pub const FSL_XCVR_CS_DATA_0_CH_UMLPCM: c_uint = 0x20;
pub const FSL_XCVR_CS_DATA_0_CH_U1BAUD: c_uint = 0x30;
pub const FSL_XCVR_CS_DATA_1_CH_MASK: c_uint = 0xF000;
pub const FSL_XCVR_CS_DATA_1_CH_2: c_uint = 0x0000;
pub const FSL_XCVR_CS_DATA_1_CH_8: c_uint = 0x7000;
pub const FSL_XCVR_CS_DATA_1_CH_16: c_uint = 0xB000;
pub const FSL_XCVR_CS_DATA_1_CH_32: c_uint = 0x3000;
// Data memory structures
pub const FSL_XCVR_RX_CS_CTRL_0: c_uint = 0x20 /* First  RX CS control register */;
pub const FSL_XCVR_RX_CS_CTRL_1: c_uint = 0x24 /* Second RX CS control register */;
pub const FSL_XCVR_RX_CS_BUFF_0: c_uint = 0x80 /* First  RX CS buffer */;
pub const FSL_XCVR_RX_CS_BUFF_1: c_uint = 0xA0 /* Second RX CS buffer */;
pub const FSL_XCVR_CAP_DATA_STR: c_uint = 0x300 /* Capabilities data structure */;
// GP PLL Registers
pub const FSL_XCVR_GP_PLL_CTRL: c_uint = 0x00;
pub const FSL_XCVR_GP_PLL_CTRL_SET: c_uint = 0x04;
pub const FSL_XCVR_GP_PLL_CTRL_CLR: c_uint = 0x08;
pub const FSL_XCVR_GP_PLL_CTRL_TOG: c_uint = 0x0C;
pub const FSL_XCVR_GP_PLL_ANA_PRG: c_uint = 0x10;
pub const FSL_XCVR_GP_PLL_ANA_PRG_SET: c_uint = 0x14;
pub const FSL_XCVR_GP_PLL_ANA_PRG_CLR: c_uint = 0x18;
pub const FSL_XCVR_GP_PLL_ANA_PRG_TOG: c_uint = 0x1C;
pub const FSL_XCVR_GP_PLL_TEST: c_uint = 0x20;
pub const FSL_XCVR_GP_PLL_TEST_SET: c_uint = 0x24;
pub const FSL_XCVR_GP_PLL_TEST_CLR: c_uint = 0x28;
pub const FSL_XCVR_GP_PLL_TEST_TOG: c_uint = 0x2C;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM: c_uint = 0x30;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM_SET: c_uint = 0x34;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM_CLR: c_uint = 0x38;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM_TOG: c_uint = 0x3C;
pub const FSL_XCVR_GP_PLL_NUMERATOR: c_uint = 0x40;
pub const FSL_XCVR_GP_PLL_NUMERATOR_SET: c_uint = 0x44;
pub const FSL_XCVR_GP_PLL_NUMERATOR_CLR: c_uint = 0x48;
pub const FSL_XCVR_GP_PLL_NUMERATOR_TOG: c_uint = 0x4C;
pub const FSL_XCVR_GP_PLL_DENOMINATOR: c_uint = 0x50;
pub const FSL_XCVR_GP_PLL_DENOMINATOR_SET: c_uint = 0x54;
pub const FSL_XCVR_GP_PLL_DENOMINATOR_CLR: c_uint = 0x58;
pub const FSL_XCVR_GP_PLL_DENOMINATOR_TOG: c_uint = 0x5C;
pub const FSL_XCVR_GP_PLL_DIV: c_uint = 0x60;
pub const FSL_XCVR_GP_PLL_DIV_SET: c_uint = 0x64;
pub const FSL_XCVR_GP_PLL_DIV_CLR: c_uint = 0x68;
pub const FSL_XCVR_GP_PLL_DIV_TOG: c_uint = 0x6C;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0: c_uint = 0x70;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0_SET: c_uint = 0x74;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0_CLR: c_uint = 0x78;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0_TOG: c_uint = 0x7C;
pub const FSL_XCVR_GP_PLL_DFS_DIV0: c_uint = 0x80;
pub const FSL_XCVR_GP_PLL_DFS_DIV0_SET: c_uint = 0x84;
pub const FSL_XCVR_GP_PLL_DFS_DIV0_CLR: c_uint = 0x88;
pub const FSL_XCVR_GP_PLL_DFS_DIV0_TOG: c_uint = 0x8C;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1: c_uint = 0x90;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1_SET: c_uint = 0x94;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1_CLR: c_uint = 0x98;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1_TOG: c_uint = 0x9C;
pub const FSL_XCVR_GP_PLL_DFS_DIV1: c_uint = 0xA0;
pub const FSL_XCVR_GP_PLL_DFS_DIV1_SET: c_uint = 0xA4;
pub const FSL_XCVR_GP_PLL_DFS_DIV1_CLR: c_uint = 0xA8;
pub const FSL_XCVR_GP_PLL_DFS_DIV1_TOG: c_uint = 0xAC;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2: c_uint = 0xB0;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2_SET: c_uint = 0xB4;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2_CLR: c_uint = 0xB8;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2_TOG: c_uint = 0xBC;
pub const FSL_XCVR_GP_PLL_DFS_DIV2: c_uint = 0xC0;
pub const FSL_XCVR_GP_PLL_DFS_DIV2_SET: c_uint = 0xC4;
pub const FSL_XCVR_GP_PLL_DFS_DIV2_CLR: c_uint = 0xC8;
pub const FSL_XCVR_GP_PLL_DFS_DIV2_TOG: c_uint = 0xCC;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3: c_uint = 0xD0;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3_SET: c_uint = 0xD4;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3_CLR: c_uint = 0xD8;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3_TOG: c_uint = 0xDC;
pub const FSL_XCVR_GP_PLL_DFS_DIV3: c_uint = 0xE0;
pub const FSL_XCVR_GP_PLL_DFS_DIV3_SET: c_uint = 0xE4;
pub const FSL_XCVR_GP_PLL_DFS_DIV3_CLR: c_uint = 0xE8;
pub const FSL_XCVR_GP_PLL_DFS_DIV3_TOG: c_uint = 0xEC;
pub const FSL_XCVR_GP_PLL_STATUS: c_uint = 0xF0;
pub const FSL_XCVR_GP_PLL_STATUS_SET: c_uint = 0xF4;
pub const FSL_XCVR_GP_PLL_STATUS_CLR: c_uint = 0xF8;
pub const FSL_XCVR_GP_PLL_STATUS_TOG: c_uint = 0xFC;
// GP PLL Control Register

// GP PLL Numerator Register
pub const FSL_XCVR_GP_PLL_NUMERATOR_MFN_SHIFT: c_int = 2;

// GP PLL Denominator Register

// GP PLL Dividers Register
pub const FSL_XCVR_GP_PLL_DIV_MFI_SHIFT: c_int = 16;

