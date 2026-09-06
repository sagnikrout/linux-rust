//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/cx231xx/cx231xx-conf-reg.h
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
pub const BOARD_CFG_STAT: c_uint = 0x0;
pub const TS_MODE_REG: c_uint = 0x4;
pub const TS1_CFG_REG: c_uint = 0x8;
pub const TS1_LENGTH_REG: c_uint = 0xc;
pub const TS2_CFG_REG: c_uint = 0x10;
pub const TS2_LENGTH_REG: c_uint = 0x14;
pub const EP_MODE_SET: c_uint = 0x18;
pub const CIR_PWR_PTN1: c_uint = 0x1c;
pub const CIR_PWR_PTN2: c_uint = 0x20;
pub const CIR_PWR_PTN3: c_uint = 0x24;
pub const CIR_PWR_MASK0: c_uint = 0x28;
pub const CIR_PWR_MASK1: c_uint = 0x2c;
pub const CIR_PWR_MASK2: c_uint = 0x30;
pub const CIR_GAIN: c_uint = 0x34;
pub const CIR_CAR_REG: c_uint = 0x38;
pub const CIR_OT_CFG1: c_uint = 0x40;
pub const CIR_OT_CFG2: c_uint = 0x44;
pub const GBULK_BIT_EN: c_uint = 0x68;
pub const PWR_CTL_EN: c_uint = 0x74;
// Polaris Endpoints capture mask for register EP_MODE_SET
pub const ENABLE_EP1: c_uint = 0x01   /* Bit[0]=1 */;
pub const ENABLE_EP2: c_uint = 0x02   /* Bit[1]=1 */;
pub const ENABLE_EP3: c_uint = 0x04   /* Bit[2]=1 */;
pub const ENABLE_EP4: c_uint = 0x08   /* Bit[3]=1 */;
pub const ENABLE_EP5: c_uint = 0x10   /* Bit[4]=1 */;
pub const ENABLE_EP6: c_uint = 0x20   /* Bit[5]=1 */;
// Bit definition for register PWR_CTL_EN
pub const PWR_MODE_MASK: c_uint = 0x17f;
pub const PWR_AV_EN: c_uint = 0x08   /* bit3 */;
pub const PWR_ISO_EN: c_uint = 0x40   /* bit6 */;
pub const PWR_AV_MODE: c_uint = 0x30   /* bit4,5  */;
pub const PWR_TUNER_EN: c_uint = 0x04   /* bit2 */;
pub const PWR_DEMOD_EN: c_uint = 0x02   /* bit1 */;
pub const I2C_DEMOD_EN: c_uint = 0x01   /* bit0 */;
pub const PWR_RESETOUT_EN: c_uint = 0x100  /* bit8 */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AV_MODE {
    POLARIS_AVMODE_DEFAULT = 0,
    POLARIS_AVMODE_DIGITAL = 0x10,
    POLARIS_AVMODE_ANALOGT_TV = 0x20,
    POLARIS_AVMODE_ENXTERNAL_AV = 0x30,

}

// Colibri Registers
pub const SINGLE_ENDED: c_uint = 0x0;
pub const LOW_IF: c_uint = 0x4;
pub const EU_IF: c_uint = 0x9;
pub const US_IF: c_uint = 0xa;
pub const SUP_BLK_TUNE1: c_uint = 0x00;
pub const SUP_BLK_TUNE2: c_uint = 0x01;
pub const SUP_BLK_TUNE3: c_uint = 0x02;
pub const SUP_BLK_XTAL: c_uint = 0x03;
pub const SUP_BLK_PLL1: c_uint = 0x04;
pub const SUP_BLK_PLL2: c_uint = 0x05;
pub const SUP_BLK_PLL3: c_uint = 0x06;
pub const SUP_BLK_REF: c_uint = 0x07;
pub const SUP_BLK_PWRDN: c_uint = 0x08;
pub const SUP_BLK_TESTPAD: c_uint = 0x09;
pub const ADC_COM_INT5_STAB_REF: c_uint = 0x0a;
pub const ADC_COM_QUANT: c_uint = 0x0b;
pub const ADC_COM_BIAS1: c_uint = 0x0c;
pub const ADC_COM_BIAS2: c_uint = 0x0d;
pub const ADC_COM_BIAS3: c_uint = 0x0e;
pub const TESTBUS_CTRL: c_uint = 0x12;
pub const FLD_PWRDN_TUNING_BIAS: c_uint = 0x10;
pub const FLD_PWRDN_ENABLE_PLL: c_uint = 0x08;
pub const FLD_PWRDN_PD_BANDGAP: c_uint = 0x04;
pub const FLD_PWRDN_PD_BIAS: c_uint = 0x02;
pub const FLD_PWRDN_PD_TUNECK: c_uint = 0x01;
pub const ADC_STATUS_CH1: c_uint = 0x20;
pub const ADC_STATUS_CH2: c_uint = 0x40;
pub const ADC_STATUS_CH3: c_uint = 0x60;
pub const ADC_STATUS2_CH1: c_uint = 0x21;
pub const ADC_STATUS2_CH2: c_uint = 0x41;
pub const ADC_STATUS2_CH3: c_uint = 0x61;
pub const ADC_CAL_ATEST_CH1: c_uint = 0x22;
pub const ADC_CAL_ATEST_CH2: c_uint = 0x42;
pub const ADC_CAL_ATEST_CH3: c_uint = 0x62;
pub const ADC_PWRDN_CLAMP_CH1: c_uint = 0x23;
pub const ADC_PWRDN_CLAMP_CH2: c_uint = 0x43;
pub const ADC_PWRDN_CLAMP_CH3: c_uint = 0x63;
pub const ADC_CTRL_DAC23_CH1: c_uint = 0x24;
pub const ADC_CTRL_DAC23_CH2: c_uint = 0x44;
pub const ADC_CTRL_DAC23_CH3: c_uint = 0x64;
pub const ADC_CTRL_DAC1_CH1: c_uint = 0x25;
pub const ADC_CTRL_DAC1_CH2: c_uint = 0x45;
pub const ADC_CTRL_DAC1_CH3: c_uint = 0x65;
pub const ADC_DCSERVO_DEM_CH1: c_uint = 0x26;
pub const ADC_DCSERVO_DEM_CH2: c_uint = 0x46;
pub const ADC_DCSERVO_DEM_CH3: c_uint = 0x66;
pub const ADC_FB_FRCRST_CH1: c_uint = 0x27;
pub const ADC_FB_FRCRST_CH2: c_uint = 0x47;
pub const ADC_FB_FRCRST_CH3: c_uint = 0x67;
pub const ADC_INPUT_CH1: c_uint = 0x28;
pub const ADC_INPUT_CH2: c_uint = 0x48;
pub const ADC_INPUT_CH3: c_uint = 0x68;
pub const INPUT_SEL_MASK: c_uint = 0x30   /* [5:4] in_sel */;
pub const ADC_NTF_PRECLMP_EN_CH1: c_uint = 0x29;
pub const ADC_NTF_PRECLMP_EN_CH2: c_uint = 0x49;
pub const ADC_NTF_PRECLMP_EN_CH3: c_uint = 0x69;
pub const ADC_QGAIN_RES_TRM_CH1: c_uint = 0x2a;
pub const ADC_QGAIN_RES_TRM_CH2: c_uint = 0x4a;
pub const ADC_QGAIN_RES_TRM_CH3: c_uint = 0x6a;
pub const ADC_SOC_PRECLMP_TERM_CH1: c_uint = 0x2b;
pub const ADC_SOC_PRECLMP_TERM_CH2: c_uint = 0x4b;
pub const ADC_SOC_PRECLMP_TERM_CH3: c_uint = 0x6b;
pub const TESTBUS_CTRL_CH1: c_uint = 0x32;
pub const TESTBUS_CTRL_CH2: c_uint = 0x52;
pub const TESTBUS_CTRL_CH3: c_uint = 0x72;
//
// DIF registers
//
pub const DIRECT_IF_REVB_BASE: c_uint = 0x00300;
//

//
pub const FLD_DIF_PLL_LOCK: c_uint = 0x80000000;
// Reserved                                [30:29]
pub const FLD_DIF_PLL_FREE_RUN: c_uint = 0x10000000;
pub const FLD_DIF_PLL_FREQ: c_uint = 0x0fffffff;
//

//
pub const FLD_DIF_KD_PD: c_uint = 0xff000000;
// Reserved                             [23:20]
pub const FLD_DIF_KDS_PD: c_uint = 0x000f0000;
pub const FLD_DIF_KI_PD: c_uint = 0x0000ff00;
// Reserved                             [7:4]
pub const FLD_DIF_KIS_PD: c_uint = 0x0000000f;
//

//
pub const FLD_DIF_KD_FD: c_uint = 0xff000000;
// Reserved                             [23:20]
pub const FLD_DIF_KDS_FD: c_uint = 0x000f0000;
pub const FLD_DIF_KI_FD: c_uint = 0x0000ff00;
pub const FLD_DIF_SIG_PROP_SZ: c_uint = 0x000000f0;
pub const FLD_DIF_KIS_FD: c_uint = 0x0000000f;
//

//
pub const FLD_DIF_PLL_AGC_REF: c_uint = 0xfff00000;
pub const FLD_DIF_PLL_AGC_KI: c_uint = 0x000f0000;
// Reserved                             [15]
pub const FLD_DIF_FREQ_LIMIT: c_uint = 0x00007000;
pub const FLD_DIF_K_FD: c_uint = 0x00000f00;
pub const FLD_DIF_DOWNSMPL_FD: c_uint = 0x000000ff;
//

//
// Reserved                             [31:16]
pub const FLD_DIF_PLL_AGC_EN: c_uint = 0x00008000;
// Reserved                             [14:12]
pub const FLD_DIF_PLL_MAN_GAIN: c_uint = 0x00000fff;
//

//
pub const FLD_DIF_K_AGC_RF: c_uint = 0xf0000000;
pub const FLD_DIF_K_AGC_IF: c_uint = 0x0f000000;
pub const FLD_DIF_K_AGC_INT: c_uint = 0x00f00000;
// Reserved                             [19:12]
pub const FLD_DIF_IF_REF: c_uint = 0x00000fff;
//

//
pub const FLD_DIF_IF_MAX: c_uint = 0xff000000;
pub const FLD_DIF_IF_MIN: c_uint = 0x00ff0000;
pub const FLD_DIF_IF_AGC: c_uint = 0x0000ffff;
//

//
pub const FLD_DIF_INT_MAX: c_uint = 0xff000000;
pub const FLD_DIF_INT_MIN: c_uint = 0x00ff0000;
pub const FLD_DIF_INT_AGC: c_uint = 0x0000ffff;
//

//
pub const FLD_DIF_RF_MAX: c_uint = 0xff000000;
pub const FLD_DIF_RF_MIN: c_uint = 0x00ff0000;
pub const FLD_DIF_RF_AGC: c_uint = 0x0000ffff;
//

//
pub const FLD_DIF_IF_AGC_IN: c_uint = 0xffff0000;
pub const FLD_DIF_INT_AGC_IN: c_uint = 0x0000ffff;
//

//
// Reserved                            [31:16]
pub const FLD_DIF_RF_AGC_IN: c_uint = 0x0000ffff;
//

//
pub const FLD_DIF_AFD: c_uint = 0xc0000000;
pub const FLD_DIF_K_VID_AGC: c_uint = 0x30000000;
pub const FLD_DIF_LINE_LENGTH: c_uint = 0x0fff0000;
pub const FLD_DIF_AGC_GAIN: c_uint = 0x0000ffff;
//

//
pub const FLD_DIF_AUDIO_AGC_OVERRIDE: c_uint = 0x80000000;
// Reserved                             [30:30]
pub const FLD_DIF_AUDIO_MAN_GAIN: c_uint = 0x3f000000;
// Reserved                             [23:17]
pub const FLD_DIF_VID_AGC_OVERRIDE: c_uint = 0x00010000;
pub const FLD_DIF_VID_MAN_GAIN: c_uint = 0x0000ffff;
//

//
pub const FLD_DIF_LPF_FREQ: c_uint = 0xc0000000;
pub const FLD_DIF_AV_PHASE_INC: c_uint = 0x3f000000;
pub const FLD_DIF_AUDIO_FREQ: c_uint = 0x00ffffff;
//

//
// Reserved                            [31:24]
pub const FLD_DIF_IIR23_R2: c_uint = 0x00ff0000;
pub const FLD_DIF_IIR23_R1: c_uint = 0x0000ff00;
pub const FLD_DIF_IIR1_R1: c_uint = 0x000000ff;
//

//
pub const FLD_DIF_DIF_BYPASS: c_uint = 0x80000000;
pub const FLD_DIF_FM_NYQ_GAIN: c_uint = 0x40000000;
pub const FLD_DIF_RF_AGC_ENA: c_uint = 0x20000000;
pub const FLD_DIF_INT_AGC_ENA: c_uint = 0x10000000;
pub const FLD_DIF_IF_AGC_ENA: c_uint = 0x08000000;
pub const FLD_DIF_FORCE_RF_IF_LOCK: c_uint = 0x04000000;
pub const FLD_DIF_VIDEO_AGC_ENA: c_uint = 0x02000000;
pub const FLD_DIF_RF_AGC_INV: c_uint = 0x01000000;
pub const FLD_DIF_INT_AGC_INV: c_uint = 0x00800000;
pub const FLD_DIF_IF_AGC_INV: c_uint = 0x00400000;
pub const FLD_DIF_SPEC_INV: c_uint = 0x00200000;
pub const FLD_DIF_AUD_FULL_BW: c_uint = 0x00100000;
pub const FLD_DIF_AUD_SRC_SEL: c_uint = 0x00080000;
// Reserved                             [18]
pub const FLD_DIF_IF_FREQ: c_uint = 0x00030000;
// Reserved                             [15:14]
pub const FLD_DIF_TIP_OFFSET: c_uint = 0x00003f00;
// Reserved                             [7:5]
pub const FLD_DIF_DITHER_ENA: c_uint = 0x00000010;
// Reserved                             [3:1]
pub const FLD_DIF_RF_IF_LOCK: c_uint = 0x00000001;
//

//
// Reserved                             [31:29]
pub const FLD_DIF_PHASE_INC: c_uint = 0x1fffffff;
//

//
// Reserved                             [31:16]
pub const FLD_DIF_SRC_KI: c_uint = 0x0000ff00;
pub const FLD_DIF_SRC_KD: c_uint = 0x000000ff;
//

//
// Reserved                             [31:19]
pub const FLD_DIF_BPF_COEFF_0: c_uint = 0x00070000;
// Reserved                             [15:4]
pub const FLD_DIF_BPF_COEFF_1: c_uint = 0x0000000f;
//

//
// Reserved                             [31:22]
pub const FLD_DIF_BPF_COEFF_2: c_uint = 0x003f0000;
// Reserved                             [15:7]
pub const FLD_DIF_BPF_COEFF_3: c_uint = 0x0000007f;
//

//
// Reserved                             [31:24]
pub const FLD_DIF_BPF_COEFF_4: c_uint = 0x00ff0000;
// Reserved                             [15:8]
pub const FLD_DIF_BPF_COEFF_5: c_uint = 0x000000ff;
//

//
// Reserved                             [31:25]
pub const FLD_DIF_BPF_COEFF_6: c_uint = 0x01ff0000;
// Reserved                             [15:9]
pub const FLD_DIF_BPF_COEFF_7: c_uint = 0x000001ff;
//

//
// Reserved                             [31:26]
pub const FLD_DIF_BPF_COEFF_8: c_uint = 0x03ff0000;
// Reserved                             [15:10]
pub const FLD_DIF_BPF_COEFF_9: c_uint = 0x000003ff;
//

//
// Reserved                             [31:27]
pub const FLD_DIF_BPF_COEFF_10: c_uint = 0x07ff0000;
// Reserved                             [15:11]
pub const FLD_DIF_BPF_COEFF_11: c_uint = 0x000007ff;
//

//
// Reserved                             [31:27]
pub const FLD_DIF_BPF_COEFF_12: c_uint = 0x07ff0000;
// Reserved                             [15:12]
pub const FLD_DIF_BPF_COEFF_13: c_uint = 0x00000fff;
//

//
// Reserved                             [31:28]
pub const FLD_DIF_BPF_COEFF_14: c_uint = 0x0fff0000;
// Reserved                             [15:12]
pub const FLD_DIF_BPF_COEFF_15: c_uint = 0x00000fff;
//

//
// Reserved                             [31:29]
pub const FLD_DIF_BPF_COEFF_16: c_uint = 0x1fff0000;
// Reserved                             [15:13]
pub const FLD_DIF_BPF_COEFF_17: c_uint = 0x00001fff;
//

//
// Reserved                             [31:29]
pub const FLD_DIF_BPF_COEFF_18: c_uint = 0x1fff0000;
// Reserved                             [15:13]
pub const FLD_DIF_BPF_COEFF_19: c_uint = 0x00001fff;
//

//
// Reserved                             [31:29]
pub const FLD_DIF_BPF_COEFF_20: c_uint = 0x1fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_21: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_22: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_23: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_24: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_25: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_26: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_27: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_28: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_29: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_30: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_31: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_32: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_33: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_34: c_uint = 0x3fff0000;
// Reserved                             [15:14]
pub const FLD_DIF_BPF_COEFF_35: c_uint = 0x00003fff;
//

//
// Reserved                             [31:30]
pub const FLD_DIF_BPF_COEFF_36: c_uint = 0x3fff0000;
// Reserved                             [15:0]
//

//
// Reserved                             [31:20]
pub const FLD_DIF_RPT_VARIANCE: c_uint = 0x000fffff;
//

//
// Reserved                             [31:8]
pub const FLD_DIF_DIF_SOFT_RST: c_uint = 0x00000080;
pub const FLD_DIF_DIF_REG_RST_MSK: c_uint = 0x00000040;
pub const FLD_DIF_AGC_RST_MSK: c_uint = 0x00000020;
pub const FLD_DIF_CMP_RST_MSK: c_uint = 0x00000010;
pub const FLD_DIF_AVS_RST_MSK: c_uint = 0x00000008;
pub const FLD_DIF_NYQ_RST_MSK: c_uint = 0x00000004;
pub const FLD_DIF_DIF_SRC_RST_MSK: c_uint = 0x00000002;
pub const FLD_DIF_PLL_RST_MSK: c_uint = 0x00000001;
//

//
// Reserved                             [31:25]
pub const FLD_DIF_CTL_IP: c_uint = 0x01ffffff;
