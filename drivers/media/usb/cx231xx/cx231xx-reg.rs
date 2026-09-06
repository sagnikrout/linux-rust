//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/cx231xx/cx231xx-reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// VBI codes
//
pub const SAV_ACTIVE_VIDEO_FIELD1: c_uint = 0x80;
pub const EAV_ACTIVE_VIDEO_FIELD1: c_uint = 0x90;
pub const SAV_ACTIVE_VIDEO_FIELD2: c_uint = 0xc0;
pub const EAV_ACTIVE_VIDEO_FIELD2: c_uint = 0xd0;
pub const SAV_VBLANK_FIELD1: c_uint = 0xa0;
pub const EAV_VBLANK_FIELD1: c_uint = 0xb0;
pub const SAV_VBLANK_FIELD2: c_uint = 0xe0;
pub const EAV_VBLANK_FIELD2: c_uint = 0xf0;
pub const SAV_VBI_FIELD1: c_uint = 0x20;
pub const EAV_VBI_FIELD1: c_uint = 0x30;
pub const SAV_VBI_FIELD2: c_uint = 0x60;
pub const EAV_VBI_FIELD2: c_uint = 0x70;
//
// Audio ADC Registers
pub const CH_PWR_CTRL1: c_uint = 0x0000000e;
pub const CH_PWR_CTRL2: c_uint = 0x0000000f;
//
pub const HOST_REG1: c_uint = 0x000;
pub const FLD_FORCE_CHIP_SEL: c_uint = 0x80;
pub const FLD_AUTO_INC_DIS: c_uint = 0x20;
pub const FLD_PREFETCH_EN: c_uint = 0x10;
// Reserved [2:3]
pub const FLD_DIGITAL_PWR_DN: c_uint = 0x02;
pub const FLD_SLEEP: c_uint = 0x01;
//
pub const HOST_REG2: c_uint = 0x001;
//
pub const HOST_REG3: c_uint = 0x002;
//
// added for polaris
pub const GPIO_PIN_CTL0: c_uint = 0x3;
pub const GPIO_PIN_CTL1: c_uint = 0x4;
pub const GPIO_PIN_CTL2: c_uint = 0x5;
pub const GPIO_PIN_CTL3: c_uint = 0x6;
pub const TS1_PIN_CTL0: c_uint = 0x7;
pub const TS1_PIN_CTL1: c_uint = 0x8;
//
pub const FLD_CLK_IN_EN: c_uint = 0x80;
pub const FLD_XTAL_CTRL: c_uint = 0x70;
pub const FLD_BB_CLK_MODE: c_uint = 0x0C;
pub const FLD_REF_DIV_PLL: c_uint = 0x02;
pub const FLD_REF_SEL_PLL1: c_uint = 0x01;
//
pub const CHIP_CTRL: c_uint = 0x100;
// Reserved [27]
// Reserved [31:21]
pub const FLD_CHIP_ACFG_DIS: c_uint = 0x00100000;
// Reserved [19]
pub const FLD_DUAL_MODE_ADC2: c_uint = 0x00040000;
pub const FLD_SIF_EN: c_uint = 0x00020000;
pub const FLD_SOFT_RST: c_uint = 0x00010000;
pub const FLD_DEVICE_ID: c_uint = 0x0000ffff;
//
pub const AFE_CTRL: c_uint = 0x104;
pub const AFE_CTRL_C2HH_SRC_CTRL: c_uint = 0x104;
pub const FLD_DIF_OUT_SEL: c_uint = 0xc0000000;
pub const FLD_AUX_PLL_CLK_ALT_SEL: c_uint = 0x3c000000;
pub const FLD_UV_ORDER_MODE: c_uint = 0x02000000;
pub const FLD_FUNC_MODE: c_uint = 0x01800000;
pub const FLD_ROT1_PHASE_CTL: c_uint = 0x007f8000;
pub const FLD_AUD_IN_SEL: c_uint = 0x00004000;
pub const FLD_LUMA_IN_SEL: c_uint = 0x00002000;
pub const FLD_CHROMA_IN_SEL: c_uint = 0x00001000;
// reserve [11:10]
pub const FLD_INV_SPEC_DIS: c_uint = 0x00000200;
pub const FLD_VGA_SEL_CH3: c_uint = 0x00000100;
pub const FLD_VGA_SEL_CH2: c_uint = 0x00000080;
pub const FLD_VGA_SEL_CH1: c_uint = 0x00000040;
pub const FLD_DCR_BYP_CH1: c_uint = 0x00000020;
pub const FLD_DCR_BYP_CH2: c_uint = 0x00000010;
pub const FLD_DCR_BYP_CH3: c_uint = 0x00000008;
pub const FLD_EN_12DB_CH3: c_uint = 0x00000004;
pub const FLD_EN_12DB_CH2: c_uint = 0x00000002;
pub const FLD_EN_12DB_CH1: c_uint = 0x00000001;
// redefine in Cx231xx
//
pub const DC_CTRL1: c_uint = 0x108;
// reserve [31:30]
pub const FLD_CLAMP_LVL_CH1: c_uint = 0x3fff8000;
pub const FLD_CLAMP_LVL_CH2: c_uint = 0x00007fff;
//
pub const DC_CTRL2: c_uint = 0x10c;
// reserve [31:28]
pub const FLD_CLAMP_LVL_CH3: c_uint = 0x00fffe00;
pub const FLD_CLAMP_WIND_LENTH: c_uint = 0x000001e0;
pub const FLD_C2HH_SAT_MIN: c_uint = 0x0000001e;
pub const FLD_FLT_BYP_SEL: c_uint = 0x00000001;
//
pub const DC_CTRL3: c_uint = 0x110;
// reserve [31:16]
pub const FLD_ERR_GAIN_CTL: c_uint = 0x00070000;
pub const FLD_LPF_MIN: c_uint = 0x0000ffff;
//
pub const DC_CTRL4: c_uint = 0x114;
// reserve [31:31]
pub const FLD_INTG_CH1: c_uint = 0x7fffffff;
//
pub const DC_CTRL5: c_uint = 0x118;
// reserve [31:31]
pub const FLD_INTG_CH2: c_uint = 0x7fffffff;
//
pub const DC_CTRL6: c_uint = 0x11c;
// reserve [31:31]
pub const FLD_INTG_CH3: c_uint = 0x7fffffff;
//
pub const PIN_CTRL: c_uint = 0x120;
pub const FLD_OEF_AGC_RF: c_uint = 0x00000001;
pub const FLD_OEF_AGC_IFVGA: c_uint = 0x00000002;
pub const FLD_OEF_AGC_IF: c_uint = 0x00000004;
pub const FLD_REG_BO_PUD: c_uint = 0x80000000;
pub const FLD_IR_IRQ_STAT: c_uint = 0x40000000;
pub const FLD_AUD_IRQ_STAT: c_uint = 0x20000000;
pub const FLD_VID_IRQ_STAT: c_uint = 0x10000000;
// Reserved [27:26]
pub const FLD_IRQ_N_OUT_EN: c_uint = 0x02000000;
pub const FLD_IRQ_N_POLAR: c_uint = 0x01000000;
// Reserved [23:6]
pub const FLD_OE_AUX_PLL_CLK: c_uint = 0x00000020;
pub const FLD_OE_I2S_BCLK: c_uint = 0x00000010;
pub const FLD_OE_I2S_WCLK: c_uint = 0x00000008;
pub const FLD_OE_AGC_IF: c_uint = 0x00000004;
pub const FLD_OE_AGC_IFVGA: c_uint = 0x00000002;
pub const FLD_OE_AGC_RF: c_uint = 0x00000001;
//
pub const AUD_IO_CTRL: c_uint = 0x124;
// Reserved [31:8]
pub const FLD_I2S_PORT_DIR: c_uint = 0x00000080;
pub const FLD_I2S_OUT_SRC: c_uint = 0x00000040;
pub const FLD_AUD_CHAN3_SRC: c_uint = 0x00000030;
pub const FLD_AUD_CHAN2_SRC: c_uint = 0x0000000c;
pub const FLD_AUD_CHAN1_SRC: c_uint = 0x00000003;
//
pub const AUD_LOCK1: c_uint = 0x128;
pub const FLD_AUD_LOCK_KI_SHIFT: c_uint = 0xc0000000;
pub const FLD_AUD_LOCK_KD_SHIFT: c_uint = 0x30000000;
// Reserved [27:25]
pub const FLD_EN_AV_LOCK: c_uint = 0x01000000;
pub const FLD_VID_COUNT: c_uint = 0x00ffffff;
//
pub const AUD_LOCK2: c_uint = 0x12c;
pub const FLD_AUD_LOCK_KI_MULT: c_uint = 0xf0000000;
pub const FLD_AUD_LOCK_KD_MULT: c_uint = 0x0F000000;
// Reserved [23:22]
pub const FLD_AUD_LOCK_FREQ_SHIFT: c_uint = 0x00300000;
pub const FLD_AUD_COUNT: c_uint = 0x000fffff;
//
pub const AFE_DIAG_CTRL1: c_uint = 0x134;
// Reserved [31:16]
pub const FLD_CUV_DLY_LENGTH: c_uint = 0x0000ff00;
pub const FLD_YC_DLY_LENGTH: c_uint = 0x000000ff;
//
// Poalris redefine
pub const AFE_DIAG_CTRL3: c_uint = 0x138;
// Reserved [31:26]
pub const FLD_AUD_DUAL_FLAG_POL: c_uint = 0x02000000;
pub const FLD_VID_DUAL_FLAG_POL: c_uint = 0x01000000;
// Reserved [23:23]
pub const FLD_COL_CLAMP_DIS_CH1: c_uint = 0x00400000;
pub const FLD_COL_CLAMP_DIS_CH2: c_uint = 0x00200000;
pub const FLD_COL_CLAMP_DIS_CH3: c_uint = 0x00100000;
pub const TEST_CTRL1: c_uint = 0x144;
// Reserved [31:29]
pub const FLD_LBIST_EN: c_uint = 0x10000000;
// Reserved [27:10]
pub const FLD_FI_BIST_INTR_R: c_uint = 0x0000200;
pub const FLD_FI_BIST_INTR_L: c_uint = 0x0000100;
pub const FLD_BIST_FAIL_AUD_PLL: c_uint = 0x0000080;
pub const FLD_BIST_INTR_AUD_PLL: c_uint = 0x0000040;
pub const FLD_BIST_FAIL_VID_PLL: c_uint = 0x0000020;
pub const FLD_BIST_INTR_VID_PLL: c_uint = 0x0000010;
// Reserved [3:1]
pub const FLD_CIR_TEST_DIS: c_uint = 0x00000001;
//
pub const TEST_CTRL2: c_uint = 0x148;
pub const FLD_TSXCLK_POL_CTL: c_uint = 0x80000000;
pub const FLD_ISO_CTL_SEL: c_uint = 0x40000000;
pub const FLD_ISO_CTL_EN: c_uint = 0x20000000;
pub const FLD_BIST_DEBUGZ: c_uint = 0x10000000;
pub const FLD_AUD_BIST_TEST_H: c_uint = 0x0f000000;
// Reserved [23:22]
pub const FLD_FLTRN_BIST_TEST_H: c_uint = 0x00020000;
pub const FLD_VID_BIST_TEST_H: c_uint = 0x00010000;
// Reserved [19:17]
pub const FLD_BIST_TEST_H: c_uint = 0x00010000;
// Reserved [15:13]
pub const FLD_TAB_EN: c_uint = 0x00001000;
// Reserved [11:0]
//
pub const BIST_STAT: c_uint = 0x14c;
pub const FLD_AUD_BIST_FAIL_H: c_uint = 0xfff00000;
pub const FLD_FLTRN_BIST_FAIL_H: c_uint = 0x00180000;
pub const FLD_VID_BIST_FAIL_H: c_uint = 0x00070000;
pub const FLD_AUD_BIST_TST_DONE: c_uint = 0x0000fff0;
pub const FLD_FLTRN_BIST_TST_DONE: c_uint = 0x00000008;
pub const FLD_VID_BIST_TST_DONE: c_uint = 0x00000007;
//
// DirectIF registers definition have been moved to DIF_reg.h
//
pub const MODE_CTRL: c_uint = 0x400;
pub const FLD_AFD_PAL60_DIS: c_uint = 0x20000000;
pub const FLD_AFD_FORCE_SECAM: c_uint = 0x10000000;
pub const FLD_AFD_FORCE_PALNC: c_uint = 0x08000000;
pub const FLD_AFD_FORCE_PAL: c_uint = 0x04000000;
pub const FLD_AFD_PALM_SEL: c_uint = 0x03000000;
pub const FLD_CKILL_MODE: c_uint = 0x00300000;
pub const FLD_COMB_NOTCH_MODE: c_uint = 0x00c00000       /* bit[19:18] */;
pub const FLD_CLR_LOCK_STAT: c_uint = 0x00020000;
pub const FLD_FAST_LOCK_MD: c_uint = 0x00010000;
pub const FLD_WCEN: c_uint = 0x00008000;
pub const FLD_CAGCEN: c_uint = 0x00004000;
pub const FLD_CKILLEN: c_uint = 0x00002000;
pub const FLD_AUTO_SC_LOCK: c_uint = 0x00001000;
pub const FLD_MAN_SC_FAST_LOCK: c_uint = 0x00000800;
pub const FLD_INPUT_MODE: c_uint = 0x00000600;
pub const FLD_AFD_ACQUIRE: c_uint = 0x00000100;
pub const FLD_AFD_NTSC_SEL: c_uint = 0x00000080;
pub const FLD_AFD_PAL_SEL: c_uint = 0x00000040;
pub const FLD_ACFG_DIS: c_uint = 0x00000020;
pub const FLD_SQ_PIXEL: c_uint = 0x00000010;
pub const FLD_VID_FMT_SEL: c_uint = 0x0000000f;
//
pub const OUT_CTRL1: c_uint = 0x404;
pub const FLD_POLAR: c_uint = 0x7f000000;
// Reserved [23]
pub const FLD_RND_MODE: c_uint = 0x00600000;
pub const FLD_VIPCLAMP_EN: c_uint = 0x00100000;
pub const FLD_VIPBLANK_EN: c_uint = 0x00080000;
pub const FLD_VIP_OPT_AL: c_uint = 0x00040000;
pub const FLD_IDID0_SOURCE: c_uint = 0x00020000;
pub const FLD_DCMODE: c_uint = 0x00010000;
pub const FLD_CLK_GATING: c_uint = 0x0000c000;
pub const FLD_CLK_INVERT: c_uint = 0x00002000;
pub const FLD_HSFMT: c_uint = 0x00001000;
pub const FLD_VALIDFMT: c_uint = 0x00000800;
pub const FLD_ACTFMT: c_uint = 0x00000400;
pub const FLD_SWAPRAW: c_uint = 0x00000200;
pub const FLD_CLAMPRAW_EN: c_uint = 0x00000100;
pub const FLD_BLUE_FIELD_EN: c_uint = 0x00000080;
pub const FLD_BLUE_FIELD_ACT: c_uint = 0x00000040;
pub const FLD_TASKBIT_VAL: c_uint = 0x00000020;
pub const FLD_ANC_DATA_EN: c_uint = 0x00000010;
pub const FLD_VBIHACTRAW_EN: c_uint = 0x00000008;
pub const FLD_MODE10B: c_uint = 0x00000004;
pub const FLD_OUT_MODE: c_uint = 0x00000003;
//
pub const OUT_CTRL2: c_uint = 0x408;
pub const FLD_AUD_GRP: c_uint = 0xc0000000;
pub const FLD_SAMPLE_RATE: c_uint = 0x30000000;
pub const FLD_AUD_ANC_EN: c_uint = 0x08000000;
pub const FLD_EN_C: c_uint = 0x04000000;
pub const FLD_EN_B: c_uint = 0x02000000;
pub const FLD_EN_A: c_uint = 0x01000000;
// Reserved [23:20]
pub const FLD_IDID1_LSB: c_uint = 0x000c0000;
pub const FLD_IDID0_LSB: c_uint = 0x00030000;
pub const FLD_IDID1_MSB: c_uint = 0x0000ff00;
pub const FLD_IDID0_MSB: c_uint = 0x000000ff;
//
pub const GEN_STAT: c_uint = 0x40c;
pub const FLD_VCR_DETECT: c_uint = 0x00800000;
pub const FLD_SPECIAL_PLAY_N: c_uint = 0x00400000;
pub const FLD_VPRES: c_uint = 0x00200000;
pub const FLD_AGC_LOCK: c_uint = 0x00100000;
pub const FLD_CSC_LOCK: c_uint = 0x00080000;
pub const FLD_VLOCK: c_uint = 0x00040000;
pub const FLD_SRC_LOCK: c_uint = 0x00020000;
pub const FLD_HLOCK: c_uint = 0x00010000;
pub const FLD_VSYNC_N: c_uint = 0x00008000;
pub const FLD_SRC_FIFO_UFLOW: c_uint = 0x00004000;
pub const FLD_SRC_FIFO_OFLOW: c_uint = 0x00002000;
pub const FLD_FIELD: c_uint = 0x00001000;
pub const FLD_AFD_FMT_STAT: c_uint = 0x00000f00;
pub const FLD_MV_TYPE2_PAIR: c_uint = 0x00000080;
pub const FLD_MV_T3CS: c_uint = 0x00000040;
pub const FLD_MV_CS: c_uint = 0x00000020;
pub const FLD_MV_PSP: c_uint = 0x00000010;
// Reserved [3]
pub const FLD_MV_CDAT: c_uint = 0x00000003;
//
pub const INT_STAT_MASK: c_uint = 0x410;
pub const FLD_COMB_3D_FIFO_MSK: c_uint = 0x80000000;
pub const FLD_WSS_DAT_AVAIL_MSK: c_uint = 0x40000000;
pub const FLD_GS2_DAT_AVAIL_MSK: c_uint = 0x20000000;
pub const FLD_GS1_DAT_AVAIL_MSK: c_uint = 0x10000000;
pub const FLD_CC_DAT_AVAIL_MSK: c_uint = 0x08000000;
pub const FLD_VPRES_CHANGE_MSK: c_uint = 0x04000000;
pub const FLD_MV_CHANGE_MSK: c_uint = 0x02000000;
pub const FLD_END_VBI_EVEN_MSK: c_uint = 0x01000000;
pub const FLD_END_VBI_ODD_MSK: c_uint = 0x00800000;
pub const FLD_FMT_CHANGE_MSK: c_uint = 0x00400000;
pub const FLD_VSYNC_TRAIL_MSK: c_uint = 0x00200000;
pub const FLD_HLOCK_CHANGE_MSK: c_uint = 0x00100000;
pub const FLD_VLOCK_CHANGE_MSK: c_uint = 0x00080000;
pub const FLD_CSC_LOCK_CHANGE_MSK: c_uint = 0x00040000;
pub const FLD_SRC_FIFO_UFLOW_MSK: c_uint = 0x00020000;
pub const FLD_SRC_FIFO_OFLOW_MSK: c_uint = 0x00010000;
pub const FLD_COMB_3D_FIFO_STAT: c_uint = 0x00008000;
pub const FLD_WSS_DAT_AVAIL_STAT: c_uint = 0x00004000;
pub const FLD_GS2_DAT_AVAIL_STAT: c_uint = 0x00002000;
pub const FLD_GS1_DAT_AVAIL_STAT: c_uint = 0x00001000;
pub const FLD_CC_DAT_AVAIL_STAT: c_uint = 0x00000800;
pub const FLD_VPRES_CHANGE_STAT: c_uint = 0x00000400;
pub const FLD_MV_CHANGE_STAT: c_uint = 0x00000200;
pub const FLD_END_VBI_EVEN_STAT: c_uint = 0x00000100;
pub const FLD_END_VBI_ODD_STAT: c_uint = 0x00000080;
pub const FLD_FMT_CHANGE_STAT: c_uint = 0x00000040;
pub const FLD_VSYNC_TRAIL_STAT: c_uint = 0x00000020;
pub const FLD_HLOCK_CHANGE_STAT: c_uint = 0x00000010;
pub const FLD_VLOCK_CHANGE_STAT: c_uint = 0x00000008;
pub const FLD_CSC_LOCK_CHANGE_STAT: c_uint = 0x00000004;
pub const FLD_SRC_FIFO_UFLOW_STAT: c_uint = 0x00000002;
pub const FLD_SRC_FIFO_OFLOW_STAT: c_uint = 0x00000001;
//
pub const LUMA_CTRL: c_uint = 0x414;
pub const BRIGHTNESS_CTRL_BYTE: c_uint = 0x414;
pub const CONTRAST_CTRL_BYTE: c_uint = 0x415;
pub const LUMA_CTRL_BYTE_3: c_uint = 0x416;
pub const FLD_LUMA_CORE_SEL: c_uint = 0x00c00000;
pub const FLD_RANGE: c_uint = 0x00300000;
// Reserved [19]
pub const FLD_PEAK_EN: c_uint = 0x00040000;
pub const FLD_PEAK_SEL: c_uint = 0x00030000;
pub const FLD_CNTRST: c_uint = 0x0000ff00;
pub const FLD_BRITE: c_uint = 0x000000ff;
//
pub const HSCALE_CTRL: c_uint = 0x418;
pub const FLD_HFILT: c_uint = 0x03000000;
pub const FLD_HSCALE: c_uint = 0x00ffffff;
//
pub const VSCALE_CTRL: c_uint = 0x41c;
pub const FLD_LINE_AVG_DIS: c_uint = 0x01000000;
// Reserved [23:20]
pub const FLD_VS_INTRLACE: c_uint = 0x00080000;
pub const FLD_VFILT: c_uint = 0x00070000;
// Reserved [15:13]
pub const FLD_VSCALE: c_uint = 0x00001fff;
//
pub const CHROMA_CTRL: c_uint = 0x420;
pub const USAT_CTRL_BYTE: c_uint = 0x420;
pub const VSAT_CTRL_BYTE: c_uint = 0x421;
pub const HUE_CTRL_BYTE: c_uint = 0x422;
pub const FLD_C_LPF_EN: c_uint = 0x20000000;
pub const FLD_CHR_DELAY: c_uint = 0x1c000000;
pub const FLD_C_CORE_SEL: c_uint = 0x03000000;
pub const FLD_HUE: c_uint = 0x00ff0000;
pub const FLD_VSAT: c_uint = 0x0000ff00;
pub const FLD_USAT: c_uint = 0x000000ff;
//
pub const VBI_LINE_CTRL1: c_uint = 0x424;
pub const FLD_VBI_MD_LINE4: c_uint = 0xff000000;
pub const FLD_VBI_MD_LINE3: c_uint = 0x00ff0000;
pub const FLD_VBI_MD_LINE2: c_uint = 0x0000ff00;
pub const FLD_VBI_MD_LINE1: c_uint = 0x000000ff;
//
pub const VBI_LINE_CTRL2: c_uint = 0x428;
pub const FLD_VBI_MD_LINE8: c_uint = 0xff000000;
pub const FLD_VBI_MD_LINE7: c_uint = 0x00ff0000;
pub const FLD_VBI_MD_LINE6: c_uint = 0x0000ff00;
pub const FLD_VBI_MD_LINE5: c_uint = 0x000000ff;
//
pub const VBI_LINE_CTRL3: c_uint = 0x42c;
pub const FLD_VBI_MD_LINE12: c_uint = 0xff000000;
pub const FLD_VBI_MD_LINE11: c_uint = 0x00ff0000;
pub const FLD_VBI_MD_LINE10: c_uint = 0x0000ff00;
pub const FLD_VBI_MD_LINE9: c_uint = 0x000000ff;
//
pub const VBI_LINE_CTRL4: c_uint = 0x430;
pub const FLD_VBI_MD_LINE16: c_uint = 0xff000000;
pub const FLD_VBI_MD_LINE15: c_uint = 0x00ff0000;
pub const FLD_VBI_MD_LINE14: c_uint = 0x0000ff00;
pub const FLD_VBI_MD_LINE13: c_uint = 0x000000ff;
//
pub const VBI_LINE_CTRL5: c_uint = 0x434;
pub const FLD_VBI_MD_LINE17: c_uint = 0x000000ff;
//
pub const VBI_FC_CFG: c_uint = 0x438;
pub const FLD_FC_ALT2: c_uint = 0xff000000;
pub const FLD_FC_ALT1: c_uint = 0x00ff0000;
pub const FLD_FC_ALT2_TYPE: c_uint = 0x0000f000;
pub const FLD_FC_ALT1_TYPE: c_uint = 0x00000f00;
// Reserved [7:1]
pub const FLD_FC_SEARCH_MODE: c_uint = 0x00000001;
//
pub const VBI_MISC_CFG1: c_uint = 0x43c;
pub const FLD_TTX_PKTADRU: c_uint = 0xfff00000;
pub const FLD_TTX_PKTADRL: c_uint = 0x000fff00;
// Reserved [7:6]
pub const FLD_MOJI_PACK_DIS: c_uint = 0x00000020;
pub const FLD_VPS_DEC_DIS: c_uint = 0x00000010;
pub const FLD_CRI_MARG_SCALE: c_uint = 0x0000000c;
pub const FLD_EDGE_RESYNC_EN: c_uint = 0x00000002;
pub const FLD_ADAPT_SLICE_DIS: c_uint = 0x00000001;
//
pub const VBI_MISC_CFG2: c_uint = 0x440;
pub const FLD_HAMMING_TYPE: c_uint = 0x0f000000;
// Reserved [23:20]
pub const FLD_WSS_FIFO_RST: c_uint = 0x00080000;
pub const FLD_GS2_FIFO_RST: c_uint = 0x00040000;
pub const FLD_GS1_FIFO_RST: c_uint = 0x00020000;
pub const FLD_CC_FIFO_RST: c_uint = 0x00010000;
// Reserved [15:12]
pub const FLD_VBI3_SDID: c_uint = 0x00000f00;
pub const FLD_VBI2_SDID: c_uint = 0x000000f0;
pub const FLD_VBI1_SDID: c_uint = 0x0000000f;
//
pub const VBI_PAY1: c_uint = 0x444;
pub const FLD_GS1_FIFO_DAT: c_uint = 0xFF000000;
pub const FLD_GS1_STAT: c_uint = 0x00FF0000;
pub const FLD_CC_FIFO_DAT: c_uint = 0x0000FF00;
pub const FLD_CC_STAT: c_uint = 0x000000FF;
//
pub const VBI_PAY2: c_uint = 0x448;
pub const FLD_WSS_FIFO_DAT: c_uint = 0xff000000;
pub const FLD_WSS_STAT: c_uint = 0x00ff0000;
pub const FLD_GS2_FIFO_DAT: c_uint = 0x0000ff00;
pub const FLD_GS2_STAT: c_uint = 0x000000ff;
//
pub const VBI_CUST1_CFG1: c_uint = 0x44c;
// Reserved [31]
pub const FLD_VBI1_CRIWIN: c_uint = 0x7f000000;
pub const FLD_VBI1_SLICE_DIST: c_uint = 0x00f00000;
pub const FLD_VBI1_BITINC: c_uint = 0x000fff00;
pub const FLD_VBI1_HDELAY: c_uint = 0x000000ff;
//
pub const VBI_CUST1_CFG2: c_uint = 0x450;
pub const FLD_VBI1_FC_LENGTH: c_uint = 0x1f000000;
pub const FLD_VBI1_FRAME_CODE: c_uint = 0x00ffffff;
//
pub const VBI_CUST1_CFG3: c_uint = 0x454;
pub const FLD_VBI1_HAM_EN: c_uint = 0x80000000;
pub const FLD_VBI1_FIFO_MODE: c_uint = 0x70000000;
pub const FLD_VBI1_FORMAT_TYPE: c_uint = 0x0f000000;
pub const FLD_VBI1_PAYLD_LENGTH: c_uint = 0x00ff0000;
pub const FLD_VBI1_CRI_LENGTH: c_uint = 0x0000f000;
pub const FLD_VBI1_CRI_MARGIN: c_uint = 0x00000f00;
pub const FLD_VBI1_CRI_TIME: c_uint = 0x000000ff;
//
pub const VBI_CUST2_CFG1: c_uint = 0x458;
// Reserved [31]
pub const FLD_VBI2_CRIWIN: c_uint = 0x7f000000;
pub const FLD_VBI2_SLICE_DIST: c_uint = 0x00f00000;
pub const FLD_VBI2_BITINC: c_uint = 0x000fff00;
pub const FLD_VBI2_HDELAY: c_uint = 0x000000ff;
//
pub const VBI_CUST2_CFG2: c_uint = 0x45c;
pub const FLD_VBI2_FC_LENGTH: c_uint = 0x1f000000;
pub const FLD_VBI2_FRAME_CODE: c_uint = 0x00ffffff;
//
pub const VBI_CUST2_CFG3: c_uint = 0x460;
pub const FLD_VBI2_HAM_EN: c_uint = 0x80000000;
pub const FLD_VBI2_FIFO_MODE: c_uint = 0x70000000;
pub const FLD_VBI2_FORMAT_TYPE: c_uint = 0x0f000000;
pub const FLD_VBI2_PAYLD_LENGTH: c_uint = 0x00ff0000;
pub const FLD_VBI2_CRI_LENGTH: c_uint = 0x0000f000;
pub const FLD_VBI2_CRI_MARGIN: c_uint = 0x00000f00;
pub const FLD_VBI2_CRI_TIME: c_uint = 0x000000ff;
//
pub const VBI_CUST3_CFG1: c_uint = 0x464;
// Reserved [31]
pub const FLD_VBI3_CRIWIN: c_uint = 0x7f000000;
pub const FLD_VBI3_SLICE_DIST: c_uint = 0x00f00000;
pub const FLD_VBI3_BITINC: c_uint = 0x000fff00;
pub const FLD_VBI3_HDELAY: c_uint = 0x000000ff;
//
pub const VBI_CUST3_CFG2: c_uint = 0x468;
pub const FLD_VBI3_FC_LENGTH: c_uint = 0x1f000000;
pub const FLD_VBI3_FRAME_CODE: c_uint = 0x00ffffff;
//
pub const VBI_CUST3_CFG3: c_uint = 0x46c;
pub const FLD_VBI3_HAM_EN: c_uint = 0x80000000;
pub const FLD_VBI3_FIFO_MODE: c_uint = 0x70000000;
pub const FLD_VBI3_FORMAT_TYPE: c_uint = 0x0f000000;
pub const FLD_VBI3_PAYLD_LENGTH: c_uint = 0x00ff0000;
pub const FLD_VBI3_CRI_LENGTH: c_uint = 0x0000f000;
pub const FLD_VBI3_CRI_MARGIN: c_uint = 0x00000f00;
pub const FLD_VBI3_CRI_TIME: c_uint = 0x000000ff;
//
pub const HORIZ_TIM_CTRL: c_uint = 0x470;
pub const FLD_BGDEL_CNT: c_uint = 0xff000000;
// Reserved [23:22]
pub const FLD_HACTIVE_CNT: c_uint = 0x003ff000;
// Reserved [11:10]
pub const FLD_HBLANK_CNT: c_uint = 0x000003ff;
//
pub const VERT_TIM_CTRL: c_uint = 0x474;
pub const FLD_V656BLANK_CNT: c_uint = 0xff000000;
// Reserved [23:22]
pub const FLD_VACTIVE_CNT: c_uint = 0x003ff000;
// Reserved [11:10]
pub const FLD_VBLANK_CNT: c_uint = 0x000003ff;
//
pub const SRC_COMB_CFG: c_uint = 0x478;
pub const FLD_CCOMB_2LN_CHECK: c_uint = 0x80000000;
pub const FLD_CCOMB_3LN_EN: c_uint = 0x40000000;
pub const FLD_CCOMB_2LN_EN: c_uint = 0x20000000;
pub const FLD_CCOMB_3D_EN: c_uint = 0x10000000;
// Reserved [27]
pub const FLD_LCOMB_3LN_EN: c_uint = 0x04000000;
pub const FLD_LCOMB_2LN_EN: c_uint = 0x02000000;
pub const FLD_LCOMB_3D_EN: c_uint = 0x01000000;
pub const FLD_LUMA_LPF_SEL: c_uint = 0x00c00000;
pub const FLD_UV_LPF_SEL: c_uint = 0x00300000;
pub const FLD_BLEND_SLOPE: c_uint = 0x000f0000;
pub const FLD_CCOMB_REDUCE_EN: c_uint = 0x00008000;
// Reserved [14:10]
pub const FLD_SRC_DECIM_RATIO: c_uint = 0x000003ff;
//
pub const CHROMA_VBIOFF_CFG: c_uint = 0x47c;
pub const FLD_VBI_VOFFSET: c_uint = 0x1f000000;
// Reserved [23:20]
pub const FLD_SC_STEP: c_uint = 0x000fffff;
//
pub const FIELD_COUNT: c_uint = 0x480;
pub const FLD_FIELD_COUNT_FLD: c_uint = 0x000003ff;
//
pub const MISC_TIM_CTRL: c_uint = 0x484;
pub const FLD_DEBOUNCE_COUNT: c_uint = 0xc0000000;
pub const FLD_VT_LINE_CNT_HYST: c_uint = 0x30000000;
// Reserved [27]
pub const FLD_AFD_STAT: c_uint = 0x07ff0000;
pub const FLD_VPRES_VERT_EN: c_uint = 0x00008000;
// Reserved [14:12]
pub const FLD_HR32: c_uint = 0x00000800;
pub const FLD_TDALGN: c_uint = 0x00000400;
pub const FLD_TDFIELD: c_uint = 0x00000200;
// Reserved [8:6]
pub const FLD_TEMPDEC: c_uint = 0x0000003f;
//
pub const DFE_CTRL1: c_uint = 0x488;
pub const FLD_CLAMP_AUTO_EN: c_uint = 0x80000000;
pub const FLD_AGC_AUTO_EN: c_uint = 0x40000000;
pub const FLD_VGA_CRUSH_EN: c_uint = 0x20000000;
pub const FLD_VGA_AUTO_EN: c_uint = 0x10000000;
pub const FLD_VBI_GATE_EN: c_uint = 0x08000000;
pub const FLD_CLAMP_LEVEL: c_uint = 0x07000000;
// Reserved [23:22]
pub const FLD_CLAMP_SKIP_CNT: c_uint = 0x00300000;
pub const FLD_AGC_GAIN: c_uint = 0x000fff00;
// Reserved [7:6]
pub const FLD_VGA_GAIN: c_uint = 0x0000003f;
//
pub const DFE_CTRL2: c_uint = 0x48c;
pub const FLD_VGA_ACQUIRE_RANGE: c_uint = 0x00ff0000;
pub const FLD_VGA_TRACK_RANGE: c_uint = 0x0000ff00;
pub const FLD_VGA_SYNC: c_uint = 0x000000ff;
//
pub const DFE_CTRL3: c_uint = 0x490;
pub const FLD_BP_PERCENT: c_uint = 0xff000000;
pub const FLD_DFT_THRESHOLD: c_uint = 0x00ff0000;
// Reserved [15:12]
pub const FLD_SYNC_WIDTH_SEL: c_uint = 0x00000600;
pub const FLD_BP_LOOP_GAIN: c_uint = 0x00000300;
pub const FLD_SYNC_LOOP_GAIN: c_uint = 0x000000c0;
// Reserved [5:4]
pub const FLD_AGC_LOOP_GAIN: c_uint = 0x0000000c;
pub const FLD_DCC_LOOP_GAIN: c_uint = 0x00000003;
//
pub const PLL_CTRL: c_uint = 0x494;
pub const FLD_PLL_KD: c_uint = 0xff000000;
pub const FLD_PLL_KI: c_uint = 0x00ff0000;
pub const FLD_PLL_MAX_OFFSET: c_uint = 0x0000ffff;
//
pub const HTL_CTRL: c_uint = 0x498;
// Reserved [31:24]
pub const FLD_AUTO_LOCK_SPD: c_uint = 0x00080000;
pub const FLD_MAN_FAST_LOCK: c_uint = 0x00040000;
pub const FLD_HTL_15K_EN: c_uint = 0x00020000;
pub const FLD_HTL_500K_EN: c_uint = 0x00010000;
pub const FLD_HTL_KD: c_uint = 0x0000ff00;
pub const FLD_HTL_KI: c_uint = 0x000000ff;
//
pub const COMB_CTRL: c_uint = 0x49c;
pub const FLD_COMB_PHASE_LIMIT: c_uint = 0xff000000;
pub const FLD_CCOMB_ERR_LIMIT: c_uint = 0x00ff0000;
pub const FLD_LUMA_THRESHOLD: c_uint = 0x0000ff00;
pub const FLD_LCOMB_ERR_LIMIT: c_uint = 0x000000ff;
//
pub const CRUSH_CTRL: c_uint = 0x4a0;
pub const FLD_WTW_EN: c_uint = 0x00400000;
pub const FLD_CRUSH_FREQ: c_uint = 0x00200000;
pub const FLD_MAJ_SEL_EN: c_uint = 0x00100000;
pub const FLD_MAJ_SEL: c_uint = 0x000c0000;
// Reserved [17:15]
pub const FLD_SYNC_TIP_REDUCE: c_uint = 0x00007e00;
// Reserved [8:6]
pub const FLD_SYNC_TIP_INC: c_uint = 0x0000003f;
//
pub const SOFT_RST_CTRL: c_uint = 0x4a4;
pub const FLD_VD_SOFT_RST: c_uint = 0x00008000;
// Reserved [14:12]
pub const FLD_REG_RST_MSK: c_uint = 0x00000800;
pub const FLD_VOF_RST_MSK: c_uint = 0x00000400;
pub const FLD_MVDET_RST_MSK: c_uint = 0x00000200;
pub const FLD_VBI_RST_MSK: c_uint = 0x00000100;
pub const FLD_SCALE_RST_MSK: c_uint = 0x00000080;
pub const FLD_CHROMA_RST_MSK: c_uint = 0x00000040;
pub const FLD_LUMA_RST_MSK: c_uint = 0x00000020;
pub const FLD_VTG_RST_MSK: c_uint = 0x00000010;
pub const FLD_YCSEP_RST_MSK: c_uint = 0x00000008;
pub const FLD_SRC_RST_MSK: c_uint = 0x00000004;
pub const FLD_DFE_RST_MSK: c_uint = 0x00000002;
// Reserved [0]
//
pub const MV_DT_CTRL1: c_uint = 0x4a8;
// Reserved [31:29]
pub const FLD_PSP_STOP_LINE: c_uint = 0x1f000000;
// Reserved [23:21]
pub const FLD_PSP_STRT_LINE: c_uint = 0x001f0000;
// Reserved [15]
pub const FLD_PSP_LLIMW: c_uint = 0x00007f00;
// Reserved [7]
pub const FLD_PSP_ULIMW: c_uint = 0x0000007f;
//
pub const MV_DT_CTRL2: c_uint = 0x4aC;
pub const FLD_CS_STOPWIN: c_uint = 0xff000000;
pub const FLD_CS_STRTWIN: c_uint = 0x00ff0000;
pub const FLD_CS_WIDTH: c_uint = 0x0000ff00;
pub const FLD_PSP_SPEC_VAL: c_uint = 0x000000ff;
//
pub const MV_DT_CTRL3: c_uint = 0x4B0;
pub const FLD_AUTO_RATE_DIS: c_uint = 0x80000000;
pub const FLD_HLOCK_DIS: c_uint = 0x40000000;
pub const FLD_SEL_FIELD_CNT: c_uint = 0x20000000;
pub const FLD_CS_TYPE2_SEL: c_uint = 0x10000000;
pub const FLD_CS_LINE_THRSH_SEL: c_uint = 0x08000000;
pub const FLD_CS_ATHRESH_SEL: c_uint = 0x04000000;
pub const FLD_PSP_SPEC_SEL: c_uint = 0x02000000;
pub const FLD_PSP_LINES_SEL: c_uint = 0x01000000;
pub const FLD_FIELD_CNT: c_uint = 0x00f00000;
pub const FLD_CS_TYPE2_CNT: c_uint = 0x000fc000;
pub const FLD_CS_LINE_CNT: c_uint = 0x00003f00;
pub const FLD_CS_ATHRESH_LEV: c_uint = 0x000000ff;
//
pub const CHIP_VERSION: c_uint = 0x4b4;
// Cx231xx redefine
pub const VERSION: c_uint = 0x4b4;
pub const FLD_REV_ID: c_uint = 0x000000ff;
//
pub const MISC_DIAG_CTRL: c_uint = 0x4b8;
// Reserved [31:24]
pub const FLD_SC_CONVERGE_THRESH: c_uint = 0x00ff0000;
pub const FLD_CCOMB_ERR_LIMIT_3D: c_uint = 0x0000ff00;
pub const FLD_LCOMB_ERR_LIMIT_3D: c_uint = 0x000000ff;
//
pub const VBI_PASS_CTRL: c_uint = 0x4bc;
pub const FLD_VBI_PASS_MD: c_uint = 0x00200000;
pub const FLD_VBI_SETUP_DIS: c_uint = 0x00100000;
pub const FLD_PASS_LINE_CTRL: c_uint = 0x000fffff;
//
// Cx231xx redefine
pub const VCR_DET_CTRL: c_uint = 0x4c0;
pub const FLD_EN_FIELD_PHASE_DET: c_uint = 0x80000000;
pub const FLD_EN_HEAD_SW_DET: c_uint = 0x40000000;
pub const FLD_FIELD_PHASE_LENGTH: c_uint = 0x01ff0000;
// Reserved [29:25]
pub const FLD_FIELD_PHASE_DELAY: c_uint = 0x0000ff00;
pub const FLD_FIELD_PHASE_LIMIT: c_uint = 0x000000f0;
pub const FLD_HEAD_SW_DET_LIMIT: c_uint = 0x0000000f;
//
pub const DL_CTL: c_uint = 0x800;
pub const DL_CTL_ADDRESS_LOW: c_uint = 0x800    /* Byte 1 in DL_CTL */;
pub const DL_CTL_ADDRESS_HIGH: c_uint = 0x801    /* Byte 2 in DL_CTL */;
pub const DL_CTL_DATA: c_uint = 0x802    /* Byte 3 in DL_CTL */;
pub const DL_CTL_CONTROL: c_uint = 0x803    /* Byte 4 in DL_CTL */;
// Reserved [31:5]
pub const FLD_START_8051: c_uint = 0x10000000;
pub const FLD_DL_ENABLE: c_uint = 0x08000000;
pub const FLD_DL_AUTO_INC: c_uint = 0x04000000;
pub const FLD_DL_MAP: c_uint = 0x03000000;
//
pub const STD_DET_STATUS: c_uint = 0x804;
pub const FLD_SPARE_STATUS1: c_uint = 0xff000000;
pub const FLD_SPARE_STATUS0: c_uint = 0x00ff0000;
pub const FLD_MOD_DET_STATUS1: c_uint = 0x0000ff00;
pub const FLD_MOD_DET_STATUS0: c_uint = 0x000000ff;
//
pub const AUD_BUILD_NUM: c_uint = 0x806;
pub const AUD_VER_NUM: c_uint = 0x807;
pub const STD_DET_CTL: c_uint = 0x808;
pub const STD_DET_CTL_AUD_CTL: c_uint = 0x808    /* Byte 1 in STD_DET_CTL */;
pub const STD_DET_CTL_PREF_MODE: c_uint = 0x809    /* Byte 2 in STD_DET_CTL */;
pub const FLD_SPARE_CTL0: c_uint = 0xff000000;
pub const FLD_DIS_DBX: c_uint = 0x00800000;
pub const FLD_DIS_BTSC: c_uint = 0x00400000;
pub const FLD_DIS_NICAM_A2: c_uint = 0x00200000;
pub const FLD_VIDEO_PRESENT: c_uint = 0x00100000;
pub const FLD_DW8051_VIDEO_FORMAT: c_uint = 0x000f0000;
pub const FLD_PREF_DEC_MODE: c_uint = 0x0000ff00;
pub const FLD_AUD_CONFIG: c_uint = 0x000000ff;
//
pub const DW8051_INT: c_uint = 0x80c;
pub const FLD_VIDEO_PRESENT_CHANGE: c_uint = 0x80000000;
pub const FLD_VIDEO_CHANGE: c_uint = 0x40000000;
pub const FLD_RDS_READY: c_uint = 0x20000000;
pub const FLD_AC97_INT: c_uint = 0x10000000;
pub const FLD_NICAM_BIT_ERROR_TOO_HIGH: c_uint = 0x08000000;
pub const FLD_NICAM_LOCK: c_uint = 0x04000000;
pub const FLD_NICAM_UNLOCK: c_uint = 0x02000000;
pub const FLD_DFT4_TH_CMP: c_uint = 0x01000000;
// Reserved [23:22]
pub const FLD_LOCK_IND_INT: c_uint = 0x00200000;
pub const FLD_DFT3_TH_CMP: c_uint = 0x00100000;
pub const FLD_DFT2_TH_CMP: c_uint = 0x00080000;
pub const FLD_DFT1_TH_CMP: c_uint = 0x00040000;
pub const FLD_FM2_DFT_TH_CMP: c_uint = 0x00020000;
pub const FLD_FM1_DFT_TH_CMP: c_uint = 0x00010000;
pub const FLD_VIDEO_PRESENT_EN: c_uint = 0x00008000;
pub const FLD_VIDEO_CHANGE_EN: c_uint = 0x00004000;
pub const FLD_RDS_READY_EN: c_uint = 0x00002000;
pub const FLD_AC97_INT_EN: c_uint = 0x00001000;
pub const FLD_NICAM_BIT_ERROR_TOO_HIGH_EN: c_uint = 0x00000800;
pub const FLD_NICAM_LOCK_EN: c_uint = 0x00000400;
pub const FLD_NICAM_UNLOCK_EN: c_uint = 0x00000200;
pub const FLD_DFT4_TH_CMP_EN: c_uint = 0x00000100;
// Reserved [7]
pub const FLD_DW8051_INT6_CTL1: c_uint = 0x00000040;
pub const FLD_DW8051_INT5_CTL1: c_uint = 0x00000020;
pub const FLD_DW8051_INT4_CTL1: c_uint = 0x00000010;
pub const FLD_DW8051_INT3_CTL1: c_uint = 0x00000008;
pub const FLD_DW8051_INT2_CTL1: c_uint = 0x00000004;
pub const FLD_DW8051_INT1_CTL1: c_uint = 0x00000002;
pub const FLD_DW8051_INT0_CTL1: c_uint = 0x00000001;
//
pub const GENERAL_CTL: c_uint = 0x810;
pub const FLD_RDS_INT: c_uint = 0x80000000;
pub const FLD_NBER_INT: c_uint = 0x40000000;
pub const FLD_NLL_INT: c_uint = 0x20000000;
pub const FLD_IFL_INT: c_uint = 0x10000000;
pub const FLD_FDL_INT: c_uint = 0x08000000;
pub const FLD_AFC_INT: c_uint = 0x04000000;
pub const FLD_AMC_INT: c_uint = 0x02000000;
pub const FLD_AC97_INT_CTL: c_uint = 0x01000000;
pub const FLD_RDS_INT_DIS: c_uint = 0x00800000;
pub const FLD_NBER_INT_DIS: c_uint = 0x00400000;
pub const FLD_NLL_INT_DIS: c_uint = 0x00200000;
pub const FLD_IFL_INT_DIS: c_uint = 0x00100000;
pub const FLD_FDL_INT_DIS: c_uint = 0x00080000;
pub const FLD_FC_INT_DIS: c_uint = 0x00040000;
pub const FLD_AMC_INT_DIS: c_uint = 0x00020000;
pub const FLD_AC97_INT_DIS: c_uint = 0x00010000;
pub const FLD_REV_NUM: c_uint = 0x0000ff00;
// Reserved [7:5]
pub const FLD_DBX_SOFT_RESET_REG: c_uint = 0x00000010;
pub const FLD_AD_SOFT_RESET_REG: c_uint = 0x00000008;
pub const FLD_SRC_SOFT_RESET_REG: c_uint = 0x00000004;
pub const FLD_CDMOD_SOFT_RESET: c_uint = 0x00000002;
pub const FLD_8051_SOFT_RESET: c_uint = 0x00000001;
//
pub const AAGC_CTL: c_uint = 0x814;
pub const FLD_AFE_12DB_EN: c_uint = 0x80000000;
pub const FLD_AAGC_DEFAULT_EN: c_uint = 0x40000000;
pub const FLD_AAGC_DEFAULT: c_uint = 0x3f000000;
// Reserved [23]
pub const FLD_AAGC_GAIN: c_uint = 0x00600000;
pub const FLD_AAGC_TH: c_uint = 0x001f0000;
// Reserved [15:14]
pub const FLD_AAGC_HYST2: c_uint = 0x00003f00;
// Reserved [7:6]
pub const FLD_AAGC_HYST1: c_uint = 0x0000003f;
//
pub const IF_SRC_CTL: c_uint = 0x818;
pub const FLD_DBX_BYPASS: c_uint = 0x80000000;
// Reserved [30:25]
pub const FLD_IF_SRC_MODE: c_uint = 0x01000000;
// Reserved [23:18]
pub const FLD_IF_SRC_PHASE_INC: c_uint = 0x0001ffff;
//
pub const ANALOG_DEMOD_CTL: c_uint = 0x81c;
pub const FLD_ROT1_PHACC_PROG: c_uint = 0xffff0000;
// Reserved [15]
pub const FLD_FM1_DELAY_FIX: c_uint = 0x00007000;
pub const FLD_PDF4_SHIFT: c_uint = 0x00000c00;
pub const FLD_PDF3_SHIFT: c_uint = 0x00000300;
pub const FLD_PDF2_SHIFT: c_uint = 0x000000c0;
pub const FLD_PDF1_SHIFT: c_uint = 0x00000030;
pub const FLD_FMBYPASS_MODE2: c_uint = 0x00000008;
pub const FLD_FMBYPASS_MODE1: c_uint = 0x00000004;
pub const FLD_NICAM_MODE: c_uint = 0x00000002;
pub const FLD_BTSC_FMRADIO_MODE: c_uint = 0x00000001;
//
pub const ROT_FREQ_CTL: c_uint = 0x820;
pub const FLD_ROT3_PHACC_PROG: c_uint = 0xffff0000;
pub const FLD_ROT2_PHACC_PROG: c_uint = 0x0000ffff;
//
pub const FM_CTL: c_uint = 0x824;
pub const FLD_FM2_DC_FB_SHIFT: c_uint = 0xf0000000;
pub const FLD_FM2_DC_INT_SHIFT: c_uint = 0x0f000000;
pub const FLD_FM2_AFC_RESET: c_uint = 0x00800000;
pub const FLD_FM2_DC_PASS_IN: c_uint = 0x00400000;
pub const FLD_FM2_DAGC_SHIFT: c_uint = 0x00380000;
pub const FLD_FM2_CORDIC_SHIFT: c_uint = 0x00070000;
pub const FLD_FM1_DC_FB_SHIFT: c_uint = 0x0000f000;
pub const FLD_FM1_DC_INT_SHIFT: c_uint = 0x00000f00;
pub const FLD_FM1_AFC_RESET: c_uint = 0x00000080;
pub const FLD_FM1_DC_PASS_IN: c_uint = 0x00000040;
pub const FLD_FM1_DAGC_SHIFT: c_uint = 0x00000038;
pub const FLD_FM1_CORDIC_SHIFT: c_uint = 0x00000007;
//
pub const LPF_PDF_CTL: c_uint = 0x828;
// Reserved [31:30]
pub const FLD_LPF32_SHIFT1: c_uint = 0x30000000;
pub const FLD_LPF32_SHIFT2: c_uint = 0x0c000000;
pub const FLD_LPF160_SHIFTA: c_uint = 0x03000000;
pub const FLD_LPF160_SHIFTB: c_uint = 0x00c00000;
pub const FLD_LPF160_SHIFTC: c_uint = 0x00300000;
pub const FLD_LPF32_COEF_SEL2: c_uint = 0x000c0000;
pub const FLD_LPF32_COEF_SEL1: c_uint = 0x00030000;
pub const FLD_LPF160_COEF_SELC: c_uint = 0x0000c000;
pub const FLD_LPF160_COEF_SELB: c_uint = 0x00003000;
pub const FLD_LPF160_COEF_SELA: c_uint = 0x00000c00;
pub const FLD_LPF160_IN_EN_REG: c_uint = 0x00000300;
pub const FLD_PDF4_PDF_SEL: c_uint = 0x000000c0;
pub const FLD_PDF3_PDF_SEL: c_uint = 0x00000030;
pub const FLD_PDF2_PDF_SEL: c_uint = 0x0000000c;
pub const FLD_PDF1_PDF_SEL: c_uint = 0x00000003;
//
pub const DFT1_CTL1: c_uint = 0x82c;
pub const FLD_DFT1_DWELL: c_uint = 0xffff0000;
pub const FLD_DFT1_FREQ: c_uint = 0x0000ffff;
//
pub const DFT1_CTL2: c_uint = 0x830;
pub const FLD_DFT1_THRESHOLD: c_uint = 0xffffff00;
pub const FLD_DFT1_CMP_CTL: c_uint = 0x00000080;
pub const FLD_DFT1_AVG: c_uint = 0x00000070;
// Reserved [3:1]
pub const FLD_DFT1_START: c_uint = 0x00000001;
//
pub const DFT1_STATUS: c_uint = 0x834;
pub const FLD_DFT1_DONE: c_uint = 0x80000000;
pub const FLD_DFT1_TH_CMP_STAT: c_uint = 0x40000000;
pub const FLD_DFT1_RESULT: c_uint = 0x3fffffff;
//
pub const DFT2_CTL1: c_uint = 0x838;
pub const FLD_DFT2_DWELL: c_uint = 0xffff0000;
pub const FLD_DFT2_FREQ: c_uint = 0x0000ffff;
//
pub const DFT2_CTL2: c_uint = 0x83C;
pub const FLD_DFT2_THRESHOLD: c_uint = 0xffffff00;
pub const FLD_DFT2_CMP_CTL: c_uint = 0x00000080;
pub const FLD_DFT2_AVG: c_uint = 0x00000070;
// Reserved [3:1]
pub const FLD_DFT2_START: c_uint = 0x00000001;
//
pub const DFT2_STATUS: c_uint = 0x840;
pub const FLD_DFT2_DONE: c_uint = 0x80000000;
pub const FLD_DFT2_TH_CMP_STAT: c_uint = 0x40000000;
pub const FLD_DFT2_RESULT: c_uint = 0x3fffffff;
//
pub const DFT3_CTL1: c_uint = 0x844;
pub const FLD_DFT3_DWELL: c_uint = 0xffff0000;
pub const FLD_DFT3_FREQ: c_uint = 0x0000ffff;
//
pub const DFT3_CTL2: c_uint = 0x848;
pub const FLD_DFT3_THRESHOLD: c_uint = 0xffffff00;
pub const FLD_DFT3_CMP_CTL: c_uint = 0x00000080;
pub const FLD_DFT3_AVG: c_uint = 0x00000070;
// Reserved [3:1]
pub const FLD_DFT3_START: c_uint = 0x00000001;
//
pub const DFT3_STATUS: c_uint = 0x84c;
pub const FLD_DFT3_DONE: c_uint = 0x80000000;
pub const FLD_DFT3_TH_CMP_STAT: c_uint = 0x40000000;
pub const FLD_DFT3_RESULT: c_uint = 0x3fffffff;
//
pub const DFT4_CTL1: c_uint = 0x850;
pub const FLD_DFT4_DWELL: c_uint = 0xffff0000;
pub const FLD_DFT4_FREQ: c_uint = 0x0000ffff;
//
pub const DFT4_CTL2: c_uint = 0x854;
pub const FLD_DFT4_THRESHOLD: c_uint = 0xffffff00;
pub const FLD_DFT4_CMP_CTL: c_uint = 0x00000080;
pub const FLD_DFT4_AVG: c_uint = 0x00000070;
// Reserved [3:1]
pub const FLD_DFT4_START: c_uint = 0x00000001;
//
pub const DFT4_STATUS: c_uint = 0x858;
pub const FLD_DFT4_DONE: c_uint = 0x80000000;
pub const FLD_DFT4_TH_CMP_STAT: c_uint = 0x40000000;
pub const FLD_DFT4_RESULT: c_uint = 0x3fffffff;
//
pub const AM_MTS_DET: c_uint = 0x85c;
pub const FLD_AM_MTS_MODE: c_uint = 0x80000000;
// Reserved [30:26]
pub const FLD_AM_SUB: c_uint = 0x02000000;
pub const FLD_AM_GAIN_EN: c_uint = 0x01000000;
// Reserved [23:16]
pub const FLD_AMMTS_GAIN_SCALE: c_uint = 0x0000e000;
pub const FLD_MTS_PDF_SHIFT: c_uint = 0x00001800;
pub const FLD_AM_REG_GAIN: c_uint = 0x00000700;
pub const FLD_AGC_REF: c_uint = 0x000000ff;
//
pub const ANALOG_MUX_CTL: c_uint = 0x860;
// Reserved [31:29]
pub const FLD_MUX21_SEL: c_uint = 0x10000000;
pub const FLD_MUX20_SEL: c_uint = 0x08000000;
pub const FLD_MUX19_SEL: c_uint = 0x04000000;
pub const FLD_MUX18_SEL: c_uint = 0x02000000;
pub const FLD_MUX17_SEL: c_uint = 0x01000000;
pub const FLD_MUX16_SEL: c_uint = 0x00800000;
pub const FLD_MUX15_SEL: c_uint = 0x00400000;
pub const FLD_MUX14_SEL: c_uint = 0x00300000;
pub const FLD_MUX13_SEL: c_uint = 0x000C0000;
pub const FLD_MUX12_SEL: c_uint = 0x00020000;
pub const FLD_MUX11_SEL: c_uint = 0x00018000;
pub const FLD_MUX10_SEL: c_uint = 0x00004000;
pub const FLD_MUX9_SEL: c_uint = 0x00002000;
pub const FLD_MUX8_SEL: c_uint = 0x00001000;
pub const FLD_MUX7_SEL: c_uint = 0x00000800;
pub const FLD_MUX6_SEL: c_uint = 0x00000600;
pub const FLD_MUX5_SEL: c_uint = 0x00000100;
pub const FLD_MUX4_SEL: c_uint = 0x000000c0;
pub const FLD_MUX3_SEL: c_uint = 0x00000030;
pub const FLD_MUX2_SEL: c_uint = 0x0000000c;
pub const FLD_MUX1_SEL: c_uint = 0x00000003;
//
// Cx231xx redefine
pub const DPLL_CTRL1: c_uint = 0x864;
pub const DIG_PLL_CTL1: c_uint = 0x864;
pub const FLD_PLL_STATUS: c_uint = 0x07000000;
pub const FLD_BANDWIDTH_SELECT: c_uint = 0x00030000;
pub const FLD_PLL_SHIFT_REG: c_uint = 0x00007000;
pub const FLD_PHASE_SHIFT: c_uint = 0x000007ff;
//
// Cx231xx redefine
pub const DPLL_CTRL2: c_uint = 0x868;
pub const DIG_PLL_CTL2: c_uint = 0x868;
pub const FLD_PLL_UNLOCK_THR: c_uint = 0xff000000;
pub const FLD_PLL_LOCK_THR: c_uint = 0x00ff0000;
// Reserved [15:8]
pub const FLD_AM_PDF_SEL2: c_uint = 0x000000c0;
pub const FLD_AM_PDF_SEL1: c_uint = 0x00000030;
pub const FLD_DPLL_FSM_CTRL: c_uint = 0x0000000c;
// Reserved [1]
pub const FLD_PLL_PILOT_DET: c_uint = 0x00000001;
//
// Cx231xx redefine
pub const DPLL_CTRL3: c_uint = 0x86c;
pub const DIG_PLL_CTL3: c_uint = 0x86c;
pub const FLD_DISABLE_LOOP: c_uint = 0x01000000;
pub const FLD_A1_DS1_SEL: c_uint = 0x000c0000;
pub const FLD_A1_DS2_SEL: c_uint = 0x00030000;
pub const FLD_A1_KI: c_uint = 0x0000ff00;
pub const FLD_A1_KD: c_uint = 0x000000ff;
//
// Cx231xx redefine
pub const DPLL_CTRL4: c_uint = 0x870;
pub const DIG_PLL_CTL4: c_uint = 0x870;
pub const FLD_A2_DS1_SEL: c_uint = 0x000c0000;
pub const FLD_A2_DS2_SEL: c_uint = 0x00030000;
pub const FLD_A2_KI: c_uint = 0x0000ff00;
pub const FLD_A2_KD: c_uint = 0x000000ff;
//
// Cx231xx redefine
pub const DPLL_CTRL5: c_uint = 0x874;
pub const DIG_PLL_CTL5: c_uint = 0x874;
pub const FLD_TRK_DS1_SEL: c_uint = 0x000c0000;
pub const FLD_TRK_DS2_SEL: c_uint = 0x00030000;
pub const FLD_TRK_KI: c_uint = 0x0000ff00;
pub const FLD_TRK_KD: c_uint = 0x000000ff;
//
pub const DEEMPH_GAIN_CTL: c_uint = 0x878;
pub const FLD_DEEMPH2_GAIN: c_uint = 0xFFFF0000;
pub const FLD_DEEMPH1_GAIN: c_uint = 0x0000FFFF;
//
// Cx231xx redefine
pub const DEEMPH_COEFF1: c_uint = 0x87c;
pub const DEEMPH_COEF1: c_uint = 0x87c;
pub const FLD_DEEMPH_B0: c_uint = 0xffff0000;
pub const FLD_DEEMPH_A0: c_uint = 0x0000ffff;
//
// Cx231xx redefine
pub const DEEMPH_COEFF2: c_uint = 0x880;
pub const DEEMPH_COEF2: c_uint = 0x880;
pub const FLD_DEEMPH_B1: c_uint = 0xFFFF0000;
pub const FLD_DEEMPH_A1: c_uint = 0x0000FFFF;
//
pub const DBX1_CTL1: c_uint = 0x884;
pub const FLD_DBX1_WBE_GAIN: c_uint = 0xffff0000;
pub const FLD_DBX1_IN_GAIN: c_uint = 0x0000ffff;
//
pub const DBX1_CTL2: c_uint = 0x888;
pub const FLD_DBX1_SE_BYPASS: c_uint = 0xffff0000;
pub const FLD_DBX1_SE_GAIN: c_uint = 0x0000ffff;
//
pub const DBX1_RMS_SE: c_uint = 0x88C;
pub const FLD_DBX1_RMS_WBE: c_uint = 0xffff0000;
pub const FLD_DBX1_RMS_SE_FLD: c_uint = 0x0000ffff;
//
pub const DBX2_CTL1: c_uint = 0x890;
pub const FLD_DBX2_WBE_GAIN: c_uint = 0xffff0000;
pub const FLD_DBX2_IN_GAIN: c_uint = 0x0000ffff;
//
pub const DBX2_CTL2: c_uint = 0x894;
pub const FLD_DBX2_SE_BYPASS: c_uint = 0xffff0000;
pub const FLD_DBX2_SE_GAIN: c_uint = 0x0000ffff;
//
pub const DBX2_RMS_SE: c_uint = 0x898;
pub const FLD_DBX2_RMS_WBE: c_uint = 0xffff0000;
pub const FLD_DBX2_RMS_SE_FLD: c_uint = 0x0000ffff;
//
pub const AM_FM_DIFF: c_uint = 0x89c;
// Reserved [31]
pub const FLD_FM_DIFF_OUT: c_uint = 0x7fff0000;
// Reserved [15]
pub const FLD_AM_DIFF_OUT: c_uint = 0x00007fff;
//
pub const NICAM_FAW: c_uint = 0x8a0;
pub const FLD_FAWDETWINEND: c_uint = 0xFc000000;
pub const FLD_FAWDETWINSTR: c_uint = 0x03ff0000;
// Reserved [15:12]
pub const FLD_FAWDETTHRSHLD3: c_uint = 0x00000f00;
pub const FLD_FAWDETTHRSHLD2: c_uint = 0x000000f0;
pub const FLD_FAWDETTHRSHLD1: c_uint = 0x0000000f;
//
// Cx231xx redefine
pub const DEEMPH_GAIN: c_uint = 0x8a4;
pub const NICAM_DEEMPHGAIN: c_uint = 0x8a4;
// Reserved [31:18]
pub const FLD_DEEMPHGAIN: c_uint = 0x0003ffff;
//
// Cx231xx redefine
pub const DEEMPH_NUMER1: c_uint = 0x8a8;
pub const NICAM_DEEMPHNUMER1: c_uint = 0x8a8;
// Reserved [31:18]
pub const FLD_DEEMPHNUMER1: c_uint = 0x0003ffff;
//
// Cx231xx redefine
pub const DEEMPH_NUMER2: c_uint = 0x8ac;
pub const NICAM_DEEMPHNUMER2: c_uint = 0x8ac;
// Reserved [31:18]
pub const FLD_DEEMPHNUMER2: c_uint = 0x0003ffff;
//
// Cx231xx redefine
pub const DEEMPH_DENOM1: c_uint = 0x8b0;
pub const NICAM_DEEMPHDENOM1: c_uint = 0x8b0;
// Reserved [31:18]
pub const FLD_DEEMPHDENOM1: c_uint = 0x0003ffff;
//
// Cx231xx redefine
pub const DEEMPH_DENOM2: c_uint = 0x8b4;
pub const NICAM_DEEMPHDENOM2: c_uint = 0x8b4;
// Reserved [31:18]
pub const FLD_DEEMPHDENOM2: c_uint = 0x0003ffff;
//
pub const NICAM_ERRLOG_CTL1: c_uint = 0x8B8;
// Reserved [31:28]
pub const FLD_ERRINTRPTTHSHLD1: c_uint = 0x0fff0000;
// Reserved [15:12]
pub const FLD_ERRLOGPERIOD: c_uint = 0x00000fff;
//
pub const NICAM_ERRLOG_CTL2: c_uint = 0x8bc;
// Reserved [31:28]
pub const FLD_ERRINTRPTTHSHLD3: c_uint = 0x0fff0000;
// Reserved [15:12]
pub const FLD_ERRINTRPTTHSHLD2: c_uint = 0x00000fff;
//
pub const NICAM_ERRLOG_STS1: c_uint = 0x8c0;
// Reserved [31:28]
pub const FLD_ERRLOG2: c_uint = 0x0fff0000;
// Reserved [15:12]
pub const FLD_ERRLOG1: c_uint = 0x00000fff;
//
pub const NICAM_ERRLOG_STS2: c_uint = 0x8c4;
// Reserved [31:12]
pub const FLD_ERRLOG3: c_uint = 0x00000fff;
//
pub const NICAM_STATUS: c_uint = 0x8c8;
// Reserved [31:20]
pub const FLD_NICAM_CIB: c_uint = 0x000c0000;
pub const FLD_NICAM_LOCK_STAT: c_uint = 0x00020000;
pub const FLD_NICAM_MUTE: c_uint = 0x00010000;
pub const FLD_NICAMADDIT_DATA: c_uint = 0x0000ffe0;
pub const FLD_NICAMCNTRL: c_uint = 0x0000001f;
//
pub const DEMATRIX_CTL: c_uint = 0x8cc;
pub const FLD_AC97_IN_SHIFT: c_uint = 0xf0000000;
pub const FLD_I2S_IN_SHIFT: c_uint = 0x0f000000;
pub const FLD_DEMATRIX_SEL_CTL: c_uint = 0x00ff0000;
// Reserved [15:11]
pub const FLD_DMTRX_BYPASS: c_uint = 0x00000400;
pub const FLD_DEMATRIX_MODE: c_uint = 0x00000300;
// Reserved [7:6]
pub const FLD_PH_DBX_SEL: c_uint = 0x00000020;
pub const FLD_PH_CH_SEL: c_uint = 0x00000010;
pub const FLD_PHASE_FIX: c_uint = 0x0000000f;
//
pub const PATH1_CTL1: c_uint = 0x8d0;
// Reserved [31:29]
pub const FLD_PATH1_MUTE_CTL: c_uint = 0x1f000000;
// Reserved [23:22]
pub const FLD_PATH1_AVC_CG: c_uint = 0x00300000;
pub const FLD_PATH1_AVC_RT: c_uint = 0x000f0000;
pub const FLD_PATH1_AVC_AT: c_uint = 0x0000f000;
pub const FLD_PATH1_AVC_STEREO: c_uint = 0x00000800;
pub const FLD_PATH1_AVC_CR: c_uint = 0x00000700;
pub const FLD_PATH1_AVC_RMS_CON: c_uint = 0x000000f0;
pub const FLD_PATH1_SEL_CTL: c_uint = 0x0000000f;
//
pub const PATH1_VOL_CTL: c_uint = 0x8d4;
pub const FLD_PATH1_AVC_THRESHOLD: c_uint = 0x7fff0000;
pub const FLD_PATH1_BAL_LEFT: c_uint = 0x00008000;
pub const FLD_PATH1_BAL_LEVEL: c_uint = 0x00007f00;
pub const FLD_PATH1_VOLUME: c_uint = 0x000000ff;
//
pub const PATH1_EQ_CTL: c_uint = 0x8d8;
// Reserved [31:30]
pub const FLD_PATH1_EQ_TREBLE_VOL: c_uint = 0x3f000000;
// Reserved [23:22]
pub const FLD_PATH1_EQ_MID_VOL: c_uint = 0x003f0000;
// Reserved [15:14]
pub const FLD_PATH1_EQ_BASS_VOL: c_uint = 0x00003f00;
// Reserved [7:1]
pub const FLD_PATH1_EQ_BAND_SEL: c_uint = 0x00000001;
//
pub const PATH1_SC_CTL: c_uint = 0x8dc;
pub const FLD_PATH1_SC_THRESHOLD: c_uint = 0x7fff0000;
pub const FLD_PATH1_SC_RT: c_uint = 0x0000f000;
pub const FLD_PATH1_SC_AT: c_uint = 0x00000f00;
pub const FLD_PATH1_SC_STEREO: c_uint = 0x00000080;
pub const FLD_PATH1_SC_CR: c_uint = 0x00000070;
pub const FLD_PATH1_SC_RMS_CON: c_uint = 0x0000000f;
//
pub const PATH2_CTL1: c_uint = 0x8e0;
// Reserved [31:26]
pub const FLD_PATH2_MUTE_CTL: c_uint = 0x03000000;
// Reserved [23:22]
pub const FLD_PATH2_AVC_CG: c_uint = 0x00300000;
pub const FLD_PATH2_AVC_RT: c_uint = 0x000f0000;
pub const FLD_PATH2_AVC_AT: c_uint = 0x0000f000;
pub const FLD_PATH2_AVC_STEREO: c_uint = 0x00000800;
pub const FLD_PATH2_AVC_CR: c_uint = 0x00000700;
pub const FLD_PATH2_AVC_RMS_CON: c_uint = 0x000000f0;
pub const FLD_PATH2_SEL_CTL: c_uint = 0x0000000f;
//
pub const PATH2_VOL_CTL: c_uint = 0x8e4;
pub const FLD_PATH2_AVC_THRESHOLD: c_uint = 0xffff0000;
pub const FLD_PATH2_BAL_LEFT: c_uint = 0x00008000;
pub const FLD_PATH2_BAL_LEVEL: c_uint = 0x00007f00;
pub const FLD_PATH2_VOLUME: c_uint = 0x000000ff;
//
pub const PATH2_EQ_CTL: c_uint = 0x8e8;
// Reserved [31:30]
pub const FLD_PATH2_EQ_TREBLE_VOL: c_uint = 0x3f000000;
// Reserved [23:22]
pub const FLD_PATH2_EQ_MID_VOL: c_uint = 0x003f0000;
// Reserved [15:14]
pub const FLD_PATH2_EQ_BASS_VOL: c_uint = 0x00003f00;
// Reserved [7:1]
pub const FLD_PATH2_EQ_BAND_SEL: c_uint = 0x00000001;
//
pub const PATH2_SC_CTL: c_uint = 0x8eC;
pub const FLD_PATH2_SC_THRESHOLD: c_uint = 0xffff0000;
pub const FLD_PATH2_SC_RT: c_uint = 0x0000f000;
pub const FLD_PATH2_SC_AT: c_uint = 0x00000f00;
pub const FLD_PATH2_SC_STEREO: c_uint = 0x00000080;
pub const FLD_PATH2_SC_CR: c_uint = 0x00000070;
pub const FLD_PATH2_SC_RMS_CON: c_uint = 0x0000000f;
//
pub const SRC_CTL: c_uint = 0x8f0;
pub const FLD_SRC_STATUS: c_uint = 0xffffff00;
pub const FLD_FIFO_LF_EN: c_uint = 0x000000fc;
pub const FLD_BYPASS_LI: c_uint = 0x00000002;
pub const FLD_BYPASS_PF: c_uint = 0x00000001;
//
pub const SRC_LF_COEF: c_uint = 0x8f4;
pub const FLD_LOOP_FILTER_COEF2: c_uint = 0xffff0000;
pub const FLD_LOOP_FILTER_COEF1: c_uint = 0x0000ffff;
//
pub const SRC1_CTL: c_uint = 0x8f8;
// Reserved [31:28]
pub const FLD_SRC1_FIFO_RD_TH: c_uint = 0x0f000000;
// Reserved [23:18]
pub const FLD_SRC1_PHASE_INC: c_uint = 0x0003ffff;
//
pub const SRC2_CTL: c_uint = 0x8fc;
// Reserved [31:28]
pub const FLD_SRC2_FIFO_RD_TH: c_uint = 0x0f000000;
// Reserved [23:18]
pub const FLD_SRC2_PHASE_INC: c_uint = 0x0003ffff;
//
pub const SRC3_CTL: c_uint = 0x900;
// Reserved [31:28]
pub const FLD_SRC3_FIFO_RD_TH: c_uint = 0x0f000000;
// Reserved [23:18]
pub const FLD_SRC3_PHASE_INC: c_uint = 0x0003ffff;
//
pub const SRC4_CTL: c_uint = 0x904;
// Reserved [31:28]
pub const FLD_SRC4_FIFO_RD_TH: c_uint = 0x0f000000;
// Reserved [23:18]
pub const FLD_SRC4_PHASE_INC: c_uint = 0x0003ffff;
//
pub const SRC5_CTL: c_uint = 0x908;
// Reserved [31:28]
pub const FLD_SRC5_FIFO_RD_TH: c_uint = 0x0f000000;
// Reserved [23:18]
pub const FLD_SRC5_PHASE_INC: c_uint = 0x0003ffff;
//
pub const SRC6_CTL: c_uint = 0x90c;
// Reserved [31:28]
pub const FLD_SRC6_FIFO_RD_TH: c_uint = 0x0f000000;
// Reserved [23:18]
pub const FLD_SRC6_PHASE_INC: c_uint = 0x0003ffff;
//
pub const BAND_OUT_SEL: c_uint = 0x910;
pub const FLD_SRC6_IN_SEL: c_uint = 0xc0000000;
pub const FLD_SRC6_CLK_SEL: c_uint = 0x30000000;
pub const FLD_SRC5_IN_SEL: c_uint = 0x0c000000;
pub const FLD_SRC5_CLK_SEL: c_uint = 0x03000000;
pub const FLD_SRC4_IN_SEL: c_uint = 0x00c00000;
pub const FLD_SRC4_CLK_SEL: c_uint = 0x00300000;
pub const FLD_SRC3_IN_SEL: c_uint = 0x000c0000;
pub const FLD_SRC3_CLK_SEL: c_uint = 0x00030000;
pub const FLD_BASEBAND_BYPASS_CTL: c_uint = 0x0000ff00;
pub const FLD_AC97_SRC_SEL: c_uint = 0x000000c0;
pub const FLD_I2S_SRC_SEL: c_uint = 0x00000030;
pub const FLD_PARALLEL2_SRC_SEL: c_uint = 0x0000000c;
pub const FLD_PARALLEL1_SRC_SEL: c_uint = 0x00000003;
//
pub const I2S_IN_CTL: c_uint = 0x914;
// Reserved [31:11]
pub const FLD_I2S_UP2X_BW20K: c_uint = 0x00000400;
pub const FLD_I2S_UP2X_BYPASS: c_uint = 0x00000200;
pub const FLD_I2S_IN_MASTER_MODE: c_uint = 0x00000100;
pub const FLD_I2S_IN_SONY_MODE: c_uint = 0x00000080;
pub const FLD_I2S_IN_RIGHT_JUST: c_uint = 0x00000040;
pub const FLD_I2S_IN_WS_SEL: c_uint = 0x00000020;
pub const FLD_I2S_IN_BCN_DEL: c_uint = 0x0000001f;
//
pub const I2S_OUT_CTL: c_uint = 0x918;
// Reserved [31:17]
pub const FLD_I2S_OUT_SOFT_RESET_EN: c_uint = 0x00010000;
// Reserved [15:9]
pub const FLD_I2S_OUT_MASTER_MODE: c_uint = 0x00000100;
pub const FLD_I2S_OUT_SONY_MODE: c_uint = 0x00000080;
pub const FLD_I2S_OUT_RIGHT_JUST: c_uint = 0x00000040;
pub const FLD_I2S_OUT_WS_SEL: c_uint = 0x00000020;
pub const FLD_I2S_OUT_BCN_DEL: c_uint = 0x0000001f;
//
pub const AC97_CTL: c_uint = 0x91c;
// Reserved [31:26]
pub const FLD_AC97_UP2X_BW20K: c_uint = 0x02000000;
pub const FLD_AC97_UP2X_BYPASS: c_uint = 0x01000000;
// Reserved [23:17]
pub const FLD_AC97_RST_ACL: c_uint = 0x00010000;
// Reserved [15:9]
pub const FLD_AC97_WAKE_UP_SYNC: c_uint = 0x00000100;
// Reserved [7:1]
pub const FLD_AC97_SHUTDOWN: c_uint = 0x00000001;
// Cx231xx redefine
pub const QPSK_IAGC_CTL1: c_uint = 0x94c;
pub const QPSK_IAGC_CTL2: c_uint = 0x950;
pub const QPSK_FEPR_FREQ: c_uint = 0x954;
pub const QPSK_BTL_CTL1: c_uint = 0x958;
pub const QPSK_BTL_CTL2: c_uint = 0x95c;
pub const QPSK_CTL_CTL1: c_uint = 0x960;
pub const QPSK_CTL_CTL2: c_uint = 0x964;
pub const QPSK_MF_FAGC_CTL: c_uint = 0x968;
pub const QPSK_EQ_CTL: c_uint = 0x96c;
pub const QPSK_LOCK_CTL: c_uint = 0x970;
//
pub const FM1_DFT_CTL: c_uint = 0x9a8;
pub const FLD_FM1_DFT_THRESHOLD: c_uint = 0xffff0000;
// Reserved [15:8]
pub const FLD_FM1_DFT_CMP_CTL: c_uint = 0x00000080;
pub const FLD_FM1_DFT_AVG: c_uint = 0x00000070;
// Reserved [3:1]
pub const FLD_FM1_DFT_START: c_uint = 0x00000001;
//
pub const FM1_DFT_STATUS: c_uint = 0x9ac;
pub const FLD_FM1_DFT_DONE: c_uint = 0x80000000;
// Reserved [30:19]
pub const FLD_FM_DFT_TH_CMP: c_uint = 0x00040000;
pub const FLD_FM1_DFT: c_uint = 0x0003ffff;
//
pub const FM2_DFT_CTL: c_uint = 0x9b0;
pub const FLD_FM2_DFT_THRESHOLD: c_uint = 0xffff0000;
// Reserved [15:8]
pub const FLD_FM2_DFT_CMP_CTL: c_uint = 0x00000080;
pub const FLD_FM2_DFT_AVG: c_uint = 0x00000070;
// Reserved [3:1]
pub const FLD_FM2_DFT_START: c_uint = 0x00000001;
//
pub const FM2_DFT_STATUS: c_uint = 0x9b4;
pub const FLD_FM2_DFT_DONE: c_uint = 0x80000000;
// Reserved [30:19]
pub const FLD_FM2_DFT_TH_CMP_STAT: c_uint = 0x00040000;
pub const FLD_FM2_DFT: c_uint = 0x0003ffff;
//
// Cx231xx redefine
pub const AAGC_STATUS_REG: c_uint = 0x9b8;
pub const AAGC_STATUS: c_uint = 0x9b8;
// Reserved [31:27]
pub const FLD_FM2_DAGC_OUT: c_uint = 0x07000000;
// Reserved [23:19]
pub const FLD_FM1_DAGC_OUT: c_uint = 0x00070000;
// Reserved [15:6]
pub const FLD_AFE_VGA_OUT: c_uint = 0x0000003f;
//
pub const MTS_GAIN_STATUS: c_uint = 0x9bc;
// Reserved [31:14]
pub const FLD_MTS_GAIN: c_uint = 0x00003fff;
pub const RDS_OUT: c_uint = 0x9c0;
pub const FLD_RDS_Q: c_uint = 0xffff0000;
pub const FLD_RDS_I: c_uint = 0x0000ffff;
//
pub const AUTOCONFIG_REG: c_uint = 0x9c4;
// Reserved [31:4]
pub const FLD_AUTOCONFIG_MODE: c_uint = 0x0000000f;
pub const FM_AFC: c_uint = 0x9c8;
pub const FLD_FM2_AFC: c_uint = 0xffff0000;
pub const FLD_FM1_AFC: c_uint = 0x0000ffff;
//
// Cx231xx redefine
pub const NEW_SPARE: c_uint = 0x9cc;
pub const NEW_SPARE_REG: c_uint = 0x9cc;
//
pub const DBX_ADJ: c_uint = 0x9d0;
// Reserved [31:28]
pub const FLD_DBX2_ADJ: c_uint = 0x0fff0000;
// Reserved [15:12]
pub const FLD_DBX1_ADJ: c_uint = 0x00000fff;
pub const VID_FMT_AUTO: c_int = 0;
pub const VID_FMT_NTSC_M: c_int = 1;
pub const VID_FMT_NTSC_J: c_int = 2;
pub const VID_FMT_NTSC_443: c_int = 3;
pub const VID_FMT_PAL_BDGHI: c_int = 4;
pub const VID_FMT_PAL_M: c_int = 5;
pub const VID_FMT_PAL_N: c_int = 6;
pub const VID_FMT_PAL_NC: c_int = 7;
pub const VID_FMT_PAL_60: c_int = 8;
pub const VID_FMT_SECAM: c_int = 12;
pub const VID_FMT_SECAM_60: c_int = 13;

pub const TWO_TAP_FILT: c_int = 0;
pub const THREE_TAP_FILT: c_int = 1;
pub const FOUR_TAP_FILT: c_int = 2;
pub const FIVE_TAP_FILT: c_int = 3;
pub const AUD_CHAN_SRC_PARALLEL: c_int = 0;
pub const AUD_CHAN_SRC_I2S_INPUT: c_int = 1;
pub const AUD_CHAN_SRC_FLATIRON: c_int = 2;
pub const AUD_CHAN_SRC_PARALLEL3: c_int = 3;
pub const OUT_MODE_601: c_int = 0;
pub const OUT_MODE_656: c_int = 1;
pub const OUT_MODE_VIP11: c_int = 2;
pub const OUT_MODE_VIP20: c_int = 3;
pub const PHASE_INC_49MHZ: c_uint = 0x0df22;
pub const PHASE_INC_56MHZ: c_uint = 0x0fa5b;
pub const PHASE_INC_28MHZ: c_uint = 0x010000;
