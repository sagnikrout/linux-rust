//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd9335.h
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
// WCD9335 register base can change according to the mode it works in.
// In slimbus mode the reg base starts from 0x800.
// In i2s/i2c mode the reg base is 0x0.
//

// Page-0 Registers

// Page-1 Registers

// Page-2 Registers

// Page-6 Registers

pub const WCD9335_ANA_BIAS_DISABLE: c_int = 0;

pub const WCD9335_ANA_BIAS_PRECHRG_DISABLE: c_int = 0;

pub const WCD9335_ANA_BIAS_PRECHRG_CTL_MODE_MANUAL: c_int = 0;

pub const WCD9335_ANA_CLK_MCLK_DISABLE: c_int = 0;

pub const WCD9335_ANA_CLK_MCLK_SRC_EXTERNAL: c_int = 0;

pub const WCD9335_ANA_CLK_EXT_CLKBUF_DISABLE: c_int = 0;

pub const WCD9335_ANA_BUCK_CTL_VOUT_D_IREF_INT: c_int = 0;

pub const WCD9335_ANA_BUCK_CTL_VOUT_D_VREF_INT: c_int = 0;

pub const WCD9335_ANA_BUCK_CTL_RAMP_START_DISABLE: c_int = 0;

pub const WCD9335_ANA_RX_BIAS_DISABLE: c_int = 0;

pub const WCD9335_MBHC_MECH_DETECT_TYPE_SHIFT: c_int = 5;

pub const WCD9335_ANA_MBHC_BD_ISRC_OFF: c_int = 0;

pub const WCD9335_SIDO_SIDO_CCL_10_ICHARG_PWR_SEL_C320FF: c_uint = 0x2;
// Comparator 1 and 2 Bias current at 1P0UA with start pulse width of C320FF
pub const WCD9335_SIDO_SIDO_CCL_DEF_VALUE: c_uint = 0x6e;

pub const WCD9335_MBHC_BTN_DBNC_T_16_MS: c_uint = 0x2;

pub const WCD9335_MBHC_HS_VREF_1P5_V: c_uint = 0x1;

pub const WCD9335_MBHC_HSDET_PULLUP_CTL_SHIFT: c_int = 6;
pub const WCD9335_MBHC_HSDET_PULLUP_CTL_1_2P0_UA: c_uint = 0x80;
pub const WCD9335_MBHC_DBNC_TIMER_INSREM_DBNC_T_96_MS: c_uint = 0x6;

pub const WCD9335_RX_BIAS_FLYB_I_0P0_UA: c_int = 0;

pub const WCD9335_HPH_CNP_WG_CTL_CURR_LDIV_RATIO_500: c_uint = 0x2;
pub const WCD9335_HPH_CNP_WG_CTL_CURR_LDIV_RATIO_1000: c_uint = 0x3;

pub const WCD9335_HPH_AUTO_CHOP_ENABLE_BY_CMPDR_GAIN: c_int = 0;

pub const WCD9335_HPH_PA_CTL2_FORCE_PSRREH_DISABLE: c_int = 0;

pub const WCD9335_HPH_PA_CTL2_FORCE_IQCTRL_DISABLE: c_int = 0;

pub const WCD9335_HPH_PA_CTL2_HPH_PSRR_DISABLE: c_int = 0;

pub const WCD9335_HPH_CONST_SEL_L_BYPASS: c_int = 0;
pub const WCD9335_HPH_CONST_SEL_L_LP_PATH: c_uint = 0x40;
pub const WCD9335_HPH_CONST_SEL_L_HQ_PATH: c_uint = 0x80;

pub const WCD9335_HPH_GAIN_SRC_SEL_COMPANDER: c_int = 0;

pub const WCD9335_HPH_RDAC_N1P65_LD_OUTCTL_V_N1P60: c_uint = 0x1;

pub const WCD9335_HPH_RDAC_1P65_LD_OUTCTL_V_N1P60: c_uint = 0x10;

pub const WCD9335_HPH_DAC_LDO_POWERMODE_LOWPOWER: c_int = 0;

pub const WCD9335_HPH_DAC_LDO_UHQA_OV_DISABLE: c_int = 0;

// Page-10 Registers

pub const WCD9335_CDC_TX_ADC_AMIC_SEL: c_int = 0;

// Page-11 Registers

pub const WCD9335_CDC_COMPANDER_CLK_DISABLE: c_int = 0;

pub const WCD9335_CDC_COMPANDER_SOFT_RST_DISABLE: c_int = 0;

pub const WCD9335_CDC_COMPANDER_NOHALT: c_int = 0;

pub const WCD9335_CDC_RX_PGA_MUTE_DISABLE: c_int = 0;

pub const WCD9335_CDC_RX_CLK_DISABLE: c_int = 0;

pub const WCD9335_CDC_RX_RESET_DISABLE: c_int = 0;

pub const WCD9335_CDC_RX_PATH_CFG_CMP_DISABLE: c_int = 0;

pub const WCD9335_CDC_RX_PATH_CFG_HD2_DISABLE: c_int = 0;

pub const WCD9335_CDC_RX_PATH_CFG0_DLY_ZN_DISABLE: c_int = 0;

pub const WCD9335_CDC_RX_PATH_SEC_HD2_SCALE_2: c_uint = 0x1;
pub const WCD9335_CDC_RX_PATH_SEC_HD2_SCALE_1: c_int = 0;

pub const WCD9335_CDC_RX_PATH_SEC_HD2_ALPHA_0P2500: c_uint = 0x10;
pub const WCD9335_CDC_RX_PATH_SEC_HD2_ALPHA_0P0000: c_int = 0;

// Page-12 Registers

// Page-13 Registers

pub const WCD9335_CDC_TX_INP_MUX_SEL_AMIC: c_uint = 0x1;
pub const WCD9335_CDC_TX_INP_MUX_SEL_DMIC: c_int = 0;

pub const WCD9335_CDC_CLK_RST_CTRL_MCLK_DISABLE: c_int = 0;

pub const WCD9335_CDC_CLK_RST_CTRL_FS_CNT_DISABLE: c_int = 0;

pub const WCD9335_MAX_REGISTER: c_uint = 0xffff;
pub const WCD9335_SEL_REGISTER: c_uint = 0x800;
// SLIMBUS Slave Registers

// ports range from 10-16

pub const WCD9335_IRQ_SLIMBUS: c_int = 0;
pub const WCD9335_IRQ_MBHC_SW_DET: c_int = 8;
pub const WCD9335_IRQ_MBHC_ELECT_INS_REM_DET: c_int = 9;
pub const WCD9335_IRQ_MBHC_BUTTON_PRESS_DET: c_int = 10;
pub const WCD9335_IRQ_MBHC_BUTTON_RELEASE_DET: c_int = 11;
pub const WCD9335_IRQ_MBHC_ELECT_INS_REM_LEG_DET: c_int = 12;
pub const SLIM_MANF_ID_QCOM: c_uint = 0x217;
pub const SLIM_PROD_CODE_WCD9335: c_uint = 0x1a0;
pub const WCD9335_VERSION_2_0: c_int = 2;
pub const WCD9335_MAX_SUPPLY: c_int = 5;
