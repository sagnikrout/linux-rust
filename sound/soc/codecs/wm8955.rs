//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8955.h
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
// wm8955.h  --  WM8904 ASoC driver
//
// Copyright 2009 Wolfson Microelectronics, plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
pub const WM8955_CLK_MCLK: c_int = 1;
//
// Register values.
//
pub const WM8955_LOUT1_VOLUME: c_uint = 0x02;
pub const WM8955_ROUT1_VOLUME: c_uint = 0x03;
pub const WM8955_DAC_CONTROL: c_uint = 0x05;
pub const WM8955_AUDIO_INTERFACE: c_uint = 0x07;
pub const WM8955_SAMPLE_RATE: c_uint = 0x08;
pub const WM8955_LEFT_DAC_VOLUME: c_uint = 0x0A;
pub const WM8955_RIGHT_DAC_VOLUME: c_uint = 0x0B;
pub const WM8955_BASS_CONTROL: c_uint = 0x0C;
pub const WM8955_TREBLE_CONTROL: c_uint = 0x0D;
pub const WM8955_RESET: c_uint = 0x0F;
pub const WM8955_ADDITIONAL_CONTROL_1: c_uint = 0x17;
pub const WM8955_ADDITIONAL_CONTROL_2: c_uint = 0x18;
pub const WM8955_POWER_MANAGEMENT_1: c_uint = 0x19;
pub const WM8955_POWER_MANAGEMENT_2: c_uint = 0x1A;
pub const WM8955_ADDITIONAL_CONTROL_3: c_uint = 0x1B;
pub const WM8955_LEFT_OUT_MIX_1: c_uint = 0x22;
pub const WM8955_LEFT_OUT_MIX_2: c_uint = 0x23;
pub const WM8955_RIGHT_OUT_MIX_1: c_uint = 0x24;
pub const WM8955_RIGHT_OUT_MIX_2: c_uint = 0x25;
pub const WM8955_MONO_OUT_MIX_1: c_uint = 0x26;
pub const WM8955_MONO_OUT_MIX_2: c_uint = 0x27;
pub const WM8955_LOUT2_VOLUME: c_uint = 0x28;
pub const WM8955_ROUT2_VOLUME: c_uint = 0x29;
pub const WM8955_MONOOUT_VOLUME: c_uint = 0x2A;
pub const WM8955_CLOCKING_PLL: c_uint = 0x2B;
pub const WM8955_PLL_CONTROL_1: c_uint = 0x2C;
pub const WM8955_PLL_CONTROL_2: c_uint = 0x2D;
pub const WM8955_PLL_CONTROL_3: c_uint = 0x2E;
pub const WM8955_PLL_CONTROL_4: c_uint = 0x3B;
pub const WM8955_REGISTER_COUNT: c_int = 29;
pub const WM8955_MAX_REGISTER: c_uint = 0x3B;
//
// Field Definitions.
//
// R2 (0x02) - LOUT1 volume
//
pub const WM8955_LO1VU: c_uint = 0x0100  /* LO1VU */;
pub const WM8955_LO1VU_MASK: c_uint = 0x0100  /* LO1VU */;

pub const WM8955_LO1ZC: c_uint = 0x0080  /* LO1ZC */;
pub const WM8955_LO1ZC_MASK: c_uint = 0x0080  /* LO1ZC */;

pub const WM8955_LOUTVOL_MASK: c_uint = 0x007F  /* LOUTVOL - [6:0] */;

//
// R3 (0x03) - ROUT1 volume
//
pub const WM8955_RO1VU: c_uint = 0x0100  /* RO1VU */;
pub const WM8955_RO1VU_MASK: c_uint = 0x0100  /* RO1VU */;

pub const WM8955_RO1ZC: c_uint = 0x0080  /* RO1ZC */;
pub const WM8955_RO1ZC_MASK: c_uint = 0x0080  /* RO1ZC */;

pub const WM8955_ROUTVOL_MASK: c_uint = 0x007F  /* ROUTVOL - [6:0] */;

//
// R5 (0x05) - DAC Control
//
pub const WM8955_DAT: c_uint = 0x0080  /* DAT */;
pub const WM8955_DAT_MASK: c_uint = 0x0080  /* DAT */;

pub const WM8955_DACMU: c_uint = 0x0008  /* DACMU */;
pub const WM8955_DACMU_MASK: c_uint = 0x0008  /* DACMU */;

pub const WM8955_DEEMPH_MASK: c_uint = 0x0006  /* DEEMPH - [2:1] */;

//
// R7 (0x07) - Audio Interface
//
pub const WM8955_BCLKINV: c_uint = 0x0080  /* BCLKINV */;
pub const WM8955_BCLKINV_MASK: c_uint = 0x0080  /* BCLKINV */;

pub const WM8955_MS: c_uint = 0x0040  /* MS */;
pub const WM8955_MS_MASK: c_uint = 0x0040  /* MS */;

pub const WM8955_LRSWAP: c_uint = 0x0020  /* LRSWAP */;
pub const WM8955_LRSWAP_MASK: c_uint = 0x0020  /* LRSWAP */;

pub const WM8955_LRP: c_uint = 0x0010  /* LRP */;
pub const WM8955_LRP_MASK: c_uint = 0x0010  /* LRP */;

pub const WM8955_WL_MASK: c_uint = 0x000C  /* WL - [3:2] */;

pub const WM8955_FORMAT_MASK: c_uint = 0x0003  /* FORMAT - [1:0] */;

//
// R8 (0x08) - Sample Rate
//
pub const WM8955_BCLKDIV2: c_uint = 0x0080  /* BCLKDIV2 */;
pub const WM8955_BCLKDIV2_MASK: c_uint = 0x0080  /* BCLKDIV2 */;

pub const WM8955_MCLKDIV2: c_uint = 0x0040  /* MCLKDIV2 */;
pub const WM8955_MCLKDIV2_MASK: c_uint = 0x0040  /* MCLKDIV2 */;

pub const WM8955_SR_MASK: c_uint = 0x003E  /* SR - [5:1] */;

pub const WM8955_USB: c_uint = 0x0001  /* USB */;
pub const WM8955_USB_MASK: c_uint = 0x0001  /* USB */;

//
// R10 (0x0A) - Left DAC volume
//
pub const WM8955_LDVU: c_uint = 0x0100  /* LDVU */;
pub const WM8955_LDVU_MASK: c_uint = 0x0100  /* LDVU */;

pub const WM8955_LDACVOL_MASK: c_uint = 0x00FF  /* LDACVOL - [7:0] */;

//
// R11 (0x0B) - Right DAC volume
//
pub const WM8955_RDVU: c_uint = 0x0100  /* RDVU */;
pub const WM8955_RDVU_MASK: c_uint = 0x0100  /* RDVU */;

pub const WM8955_RDACVOL_MASK: c_uint = 0x00FF  /* RDACVOL - [7:0] */;

//
// R12 (0x0C) - Bass control
//
pub const WM8955_BB: c_uint = 0x0080  /* BB */;
pub const WM8955_BB_MASK: c_uint = 0x0080  /* BB */;

pub const WM8955_BC: c_uint = 0x0040  /* BC */;
pub const WM8955_BC_MASK: c_uint = 0x0040  /* BC */;

pub const WM8955_BASS_MASK: c_uint = 0x000F  /* BASS - [3:0] */;

//
// R13 (0x0D) - Treble control
//
pub const WM8955_TC: c_uint = 0x0040  /* TC */;
pub const WM8955_TC_MASK: c_uint = 0x0040  /* TC */;

pub const WM8955_TRBL_MASK: c_uint = 0x000F  /* TRBL - [3:0] */;

//
// R15 (0x0F) - Reset
//
pub const WM8955_RESET_MASK: c_uint = 0x01FF  /* RESET - [8:0] */;

//
// R23 (0x17) - Additional control (1)
//
pub const WM8955_TSDEN: c_uint = 0x0100  /* TSDEN */;
pub const WM8955_TSDEN_MASK: c_uint = 0x0100  /* TSDEN */;

pub const WM8955_VSEL_MASK: c_uint = 0x00C0  /* VSEL - [7:6] */;

pub const WM8955_DMONOMIX_MASK: c_uint = 0x0030  /* DMONOMIX - [5:4] */;

pub const WM8955_DACINV: c_uint = 0x0002  /* DACINV */;
pub const WM8955_DACINV_MASK: c_uint = 0x0002  /* DACINV */;

pub const WM8955_TOEN: c_uint = 0x0001  /* TOEN */;
pub const WM8955_TOEN_MASK: c_uint = 0x0001  /* TOEN */;

//
// R24 (0x18) - Additional control (2)
//
pub const WM8955_OUT3SW_MASK: c_uint = 0x0180  /* OUT3SW - [8:7] */;

pub const WM8955_ROUT2INV: c_uint = 0x0010  /* ROUT2INV */;
pub const WM8955_ROUT2INV_MASK: c_uint = 0x0010  /* ROUT2INV */;

pub const WM8955_DACOSR: c_uint = 0x0001  /* DACOSR */;
pub const WM8955_DACOSR_MASK: c_uint = 0x0001  /* DACOSR */;

//
// R25 (0x19) - Power Management (1)
//
pub const WM8955_VMIDSEL_MASK: c_uint = 0x0180  /* VMIDSEL - [8:7] */;

pub const WM8955_VREF: c_uint = 0x0040  /* VREF */;
pub const WM8955_VREF_MASK: c_uint = 0x0040  /* VREF */;

pub const WM8955_DIGENB: c_uint = 0x0001  /* DIGENB */;
pub const WM8955_DIGENB_MASK: c_uint = 0x0001  /* DIGENB */;

//
// R26 (0x1A) - Power Management (2)
//
pub const WM8955_DACL: c_uint = 0x0100  /* DACL */;
pub const WM8955_DACL_MASK: c_uint = 0x0100  /* DACL */;

pub const WM8955_DACR: c_uint = 0x0080  /* DACR */;
pub const WM8955_DACR_MASK: c_uint = 0x0080  /* DACR */;

pub const WM8955_LOUT1: c_uint = 0x0040  /* LOUT1 */;
pub const WM8955_LOUT1_MASK: c_uint = 0x0040  /* LOUT1 */;

pub const WM8955_ROUT1: c_uint = 0x0020  /* ROUT1 */;
pub const WM8955_ROUT1_MASK: c_uint = 0x0020  /* ROUT1 */;

pub const WM8955_LOUT2: c_uint = 0x0010  /* LOUT2 */;
pub const WM8955_LOUT2_MASK: c_uint = 0x0010  /* LOUT2 */;

pub const WM8955_ROUT2: c_uint = 0x0008  /* ROUT2 */;
pub const WM8955_ROUT2_MASK: c_uint = 0x0008  /* ROUT2 */;

pub const WM8955_MONO: c_uint = 0x0004  /* MONO */;
pub const WM8955_MONO_MASK: c_uint = 0x0004  /* MONO */;

pub const WM8955_OUT3: c_uint = 0x0002  /* OUT3 */;
pub const WM8955_OUT3_MASK: c_uint = 0x0002  /* OUT3 */;

//
// R27 (0x1B) - Additional Control (3)
//
pub const WM8955_VROI: c_uint = 0x0040  /* VROI */;
pub const WM8955_VROI_MASK: c_uint = 0x0040  /* VROI */;

//
// R34 (0x22) - Left out Mix (1)
//
pub const WM8955_LD2LO: c_uint = 0x0100  /* LD2LO */;
pub const WM8955_LD2LO_MASK: c_uint = 0x0100  /* LD2LO */;

pub const WM8955_LI2LO: c_uint = 0x0080  /* LI2LO */;
pub const WM8955_LI2LO_MASK: c_uint = 0x0080  /* LI2LO */;

pub const WM8955_LI2LOVOL_MASK: c_uint = 0x0070  /* LI2LOVOL - [6:4] */;

//
// R35 (0x23) - Left out Mix (2)
//
pub const WM8955_RD2LO: c_uint = 0x0100  /* RD2LO */;
pub const WM8955_RD2LO_MASK: c_uint = 0x0100  /* RD2LO */;

pub const WM8955_RI2LO: c_uint = 0x0080  /* RI2LO */;
pub const WM8955_RI2LO_MASK: c_uint = 0x0080  /* RI2LO */;

pub const WM8955_RI2LOVOL_MASK: c_uint = 0x0070  /* RI2LOVOL - [6:4] */;

//
// R36 (0x24) - Right out Mix (1)
//
pub const WM8955_LD2RO: c_uint = 0x0100  /* LD2RO */;
pub const WM8955_LD2RO_MASK: c_uint = 0x0100  /* LD2RO */;

pub const WM8955_LI2RO: c_uint = 0x0080  /* LI2RO */;
pub const WM8955_LI2RO_MASK: c_uint = 0x0080  /* LI2RO */;

pub const WM8955_LI2ROVOL_MASK: c_uint = 0x0070  /* LI2ROVOL - [6:4] */;

//
// R37 (0x25) - Right Out Mix (2)
//
pub const WM8955_RD2RO: c_uint = 0x0100  /* RD2RO */;
pub const WM8955_RD2RO_MASK: c_uint = 0x0100  /* RD2RO */;

pub const WM8955_RI2RO: c_uint = 0x0080  /* RI2RO */;
pub const WM8955_RI2RO_MASK: c_uint = 0x0080  /* RI2RO */;

pub const WM8955_RI2ROVOL_MASK: c_uint = 0x0070  /* RI2ROVOL - [6:4] */;

//
// R38 (0x26) - Mono out Mix (1)
//
pub const WM8955_LD2MO: c_uint = 0x0100  /* LD2MO */;
pub const WM8955_LD2MO_MASK: c_uint = 0x0100  /* LD2MO */;

pub const WM8955_LI2MO: c_uint = 0x0080  /* LI2MO */;
pub const WM8955_LI2MO_MASK: c_uint = 0x0080  /* LI2MO */;

pub const WM8955_LI2MOVOL_MASK: c_uint = 0x0070  /* LI2MOVOL - [6:4] */;

pub const WM8955_DMEN: c_uint = 0x0001  /* DMEN */;
pub const WM8955_DMEN_MASK: c_uint = 0x0001  /* DMEN */;

//
// R39 (0x27) - Mono out Mix (2)
//
pub const WM8955_RD2MO: c_uint = 0x0100  /* RD2MO */;
pub const WM8955_RD2MO_MASK: c_uint = 0x0100  /* RD2MO */;

pub const WM8955_RI2MO: c_uint = 0x0080  /* RI2MO */;
pub const WM8955_RI2MO_MASK: c_uint = 0x0080  /* RI2MO */;

pub const WM8955_RI2MOVOL_MASK: c_uint = 0x0070  /* RI2MOVOL - [6:4] */;

//
// R40 (0x28) - LOUT2 volume
//
pub const WM8955_LO2VU: c_uint = 0x0100  /* LO2VU */;
pub const WM8955_LO2VU_MASK: c_uint = 0x0100  /* LO2VU */;

pub const WM8955_LO2ZC: c_uint = 0x0080  /* LO2ZC */;
pub const WM8955_LO2ZC_MASK: c_uint = 0x0080  /* LO2ZC */;

pub const WM8955_LOUT2VOL_MASK: c_uint = 0x007F  /* LOUT2VOL - [6:0] */;

//
// R41 (0x29) - ROUT2 volume
//
pub const WM8955_RO2VU: c_uint = 0x0100  /* RO2VU */;
pub const WM8955_RO2VU_MASK: c_uint = 0x0100  /* RO2VU */;

pub const WM8955_RO2ZC: c_uint = 0x0080  /* RO2ZC */;
pub const WM8955_RO2ZC_MASK: c_uint = 0x0080  /* RO2ZC */;

pub const WM8955_ROUT2VOL_MASK: c_uint = 0x007F  /* ROUT2VOL - [6:0] */;

//
// R42 (0x2A) - MONOOUT volume
//
pub const WM8955_MOZC: c_uint = 0x0080  /* MOZC */;
pub const WM8955_MOZC_MASK: c_uint = 0x0080  /* MOZC */;

pub const WM8955_MOUTVOL_MASK: c_uint = 0x007F  /* MOUTVOL - [6:0] */;

//
// R43 (0x2B) - Clocking / PLL
//
pub const WM8955_MCLKSEL: c_uint = 0x0100  /* MCLKSEL */;
pub const WM8955_MCLKSEL_MASK: c_uint = 0x0100  /* MCLKSEL */;

pub const WM8955_PLLOUTDIV2: c_uint = 0x0020  /* PLLOUTDIV2 */;
pub const WM8955_PLLOUTDIV2_MASK: c_uint = 0x0020  /* PLLOUTDIV2 */;

pub const WM8955_PLL_RB: c_uint = 0x0010  /* PLL_RB */;
pub const WM8955_PLL_RB_MASK: c_uint = 0x0010  /* PLL_RB */;

pub const WM8955_PLLEN: c_uint = 0x0008  /* PLLEN */;
pub const WM8955_PLLEN_MASK: c_uint = 0x0008  /* PLLEN */;

//
// R44 (0x2C) - PLL Control 1
//
pub const WM8955_N_MASK: c_uint = 0x01E0  /* N - [8:5] */;

pub const WM8955_K_21_18_MASK: c_uint = 0x000F  /* K(21:18) - [3:0] */;

//
// R45 (0x2D) - PLL Control 2
//
pub const WM8955_K_17_9_MASK: c_uint = 0x01FF  /* K(17:9) - [8:0] */;

//
// R46 (0x2E) - PLL Control 3
//
pub const WM8955_K_8_0_MASK: c_uint = 0x01FF  /* K(8:0) - [8:0] */;

//
// R59 (0x3B) - PLL Control 4
//
pub const WM8955_KEN: c_uint = 0x0080  /* KEN */;
pub const WM8955_KEN_MASK: c_uint = 0x0080  /* KEN */;

