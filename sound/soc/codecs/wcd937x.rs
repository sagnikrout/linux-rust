//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd937x.h
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
// Copyright (c) 2023-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const WCD937X_BASE_ADDRESS: c_uint = 0x3000;
pub const WCD937X_ANA_BIAS: c_uint = 0x3001;
pub const WCD937X_ANA_RX_SUPPLIES: c_uint = 0x3008;
pub const WCD937X_ANA_HPH: c_uint = 0x3009;
pub const WCD937X_ANA_EAR: c_uint = 0x300A;
pub const WCD937X_ANA_EAR_COMPANDER_CTL: c_uint = 0x300B;

pub const WCD937X_ANA_TX_CH1: c_uint = 0x300E;
pub const WCD937X_ANA_TX_CH2: c_uint = 0x300F;
pub const WCD937X_ANA_TX_CH3: c_uint = 0x3010;
pub const WCD937X_ANA_TX_CH3_HPF: c_uint = 0x3011;
pub const WCD937X_ANA_MICB1_MICB2_DSP_EN_LOGIC: c_uint = 0x3012;
pub const WCD937X_ANA_MICB3_DSP_EN_LOGIC: c_uint = 0x3013;
pub const WCD937X_ANA_MBHC_MECH: c_uint = 0x3014;

pub const WCD937X_MBHC_MECH_DETECT_TYPE_INS: c_int = 1;

pub const WCD937X_MBHC_HPHL_PLUG_TYPE_NO: c_int = 1;

pub const WCD937X_MBHC_GND_PLUG_TYPE_NO: c_int = 1;

pub const WCD937X_ANA_MBHC_ELECT: c_uint = 0x3015;

pub const WCD937X_ANA_MBHC_BD_ISRC_OFF: c_int = 0;

pub const WCD937X_ANA_MBHC_ZDET: c_uint = 0x3016;
pub const WCD937X_ANA_MBHC_RESULT_1: c_uint = 0x3017;
pub const WCD937X_ANA_MBHC_RESULT_2: c_uint = 0x3018;
pub const WCD937X_ANA_MBHC_RESULT_3: c_uint = 0x3019;

pub const WCD937X_ANA_MBHC_BTN0: c_uint = 0x301A;

pub const WCD937X_ANA_MBHC_BTN1: c_uint = 0x301B;
pub const WCD937X_ANA_MBHC_BTN2: c_uint = 0x301C;
pub const WCD937X_ANA_MBHC_BTN3: c_uint = 0x301D;
pub const WCD937X_ANA_MBHC_BTN4: c_uint = 0x301E;
pub const WCD937X_ANA_MBHC_BTN5: c_uint = 0x301F;

pub const WCD937X_ANA_MBHC_BTN6: c_uint = 0x3020;
pub const WCD937X_ANA_MBHC_BTN7: c_uint = 0x3021;
pub const WCD937X_ANA_MICB1: c_uint = 0x3022;

pub const WCD937X_MICB_DISABLE: c_int = 0;
pub const WCD937X_MICB_ENABLE: c_int = 1;
pub const WCD937X_MICB_PULL_UP: c_int = 2;
pub const WCD937X_MICB_PULL_DOWN: c_int = 3;
pub const WCD937X_ANA_MICB2: c_uint = 0x3023;

pub const WCD937X_ANA_MICB2_RAMP: c_uint = 0x3024;

pub const WCD937X_ANA_MICB3: c_uint = 0x3025;

pub const WCD937X_MICB_DISABLE: c_int = 0;
pub const WCD937X_MICB_ENABLE: c_int = 1;
pub const WCD937X_MICB_PULL_UP: c_int = 2;

pub const WCD937X_BIAS_CTL: c_uint = 0x3028;
pub const WCD937X_BIAS_VBG_FINE_ADJ: c_uint = 0x3029;
pub const WCD937X_LDOL_VDDCX_ADJUST: c_uint = 0x3040;
pub const WCD937X_LDOL_DISABLE_LDOL: c_uint = 0x3041;
pub const WCD937X_MBHC_CTL_CLK: c_uint = 0x3056;
pub const WCD937X_MBHC_CTL_ANA: c_uint = 0x3057;
pub const WCD937X_MBHC_CTL_SPARE_1: c_uint = 0x3058;
pub const WCD937X_MBHC_CTL_SPARE_2: c_uint = 0x3059;
pub const WCD937X_MBHC_CTL_BCS: c_uint = 0x305A;
pub const WCD937X_MBHC_MOISTURE_DET_FSM_STATUS: c_uint = 0x305B;
pub const WCD937X_MBHC_TEST_CTL: c_uint = 0x305C;
pub const WCD937X_LDOH_MODE: c_uint = 0x3067;
pub const WCD937X_LDOH_BIAS: c_uint = 0x3068;
pub const WCD937X_LDOH_STB_LOADS: c_uint = 0x3069;
pub const WCD937X_LDOH_SLOWRAMP: c_uint = 0x306A;
pub const WCD937X_MICB1_TEST_CTL_1: c_uint = 0x306B;
pub const WCD937X_MICB1_TEST_CTL_2: c_uint = 0x306C;
pub const WCD937X_MICB1_TEST_CTL_3: c_uint = 0x306D;
pub const WCD937X_MICB2_TEST_CTL_1: c_uint = 0x306E;
pub const WCD937X_MICB2_TEST_CTL_2: c_uint = 0x306F;
pub const WCD937X_MICB2_TEST_CTL_3: c_uint = 0x3070;
pub const WCD937X_MICB3_TEST_CTL_1: c_uint = 0x3071;
pub const WCD937X_MICB3_TEST_CTL_2: c_uint = 0x3072;
pub const WCD937X_MICB3_TEST_CTL_3: c_uint = 0x3073;
pub const WCD937X_TX_COM_ADC_VCM: c_uint = 0x3077;
pub const WCD937X_TX_COM_BIAS_ATEST: c_uint = 0x3078;
pub const WCD937X_TX_COM_ADC_INT1_IB: c_uint = 0x3079;
pub const WCD937X_TX_COM_ADC_INT2_IB: c_uint = 0x307A;
pub const WCD937X_TX_COM_TXFE_DIV_CTL: c_uint = 0x307B;
pub const WCD937X_TX_COM_TXFE_DIV_START: c_uint = 0x307C;
pub const WCD937X_TX_COM_TXFE_DIV_STOP_9P6M: c_uint = 0x307D;
pub const WCD937X_TX_COM_TXFE_DIV_STOP_12P288M: c_uint = 0x307E;
pub const WCD937X_TX_1_2_TEST_EN: c_uint = 0x307F;
pub const WCD937X_TX_1_2_ADC_IB: c_uint = 0x3080;
pub const WCD937X_TX_1_2_ATEST_REFCTL: c_uint = 0x3081;
pub const WCD937X_TX_1_2_TEST_CTL: c_uint = 0x3082;
pub const WCD937X_TX_1_2_TEST_BLK_EN: c_uint = 0x3083;
pub const WCD937X_TX_1_2_TXFE_CLKDIV: c_uint = 0x3084;
pub const WCD937X_TX_1_2_SAR2_ERR: c_uint = 0x3085;
pub const WCD937X_TX_1_2_SAR1_ERR: c_uint = 0x3086;
pub const WCD937X_TX_3_TEST_EN: c_uint = 0x3087;
pub const WCD937X_TX_3_ADC_IB: c_uint = 0x3088;
pub const WCD937X_TX_3_ATEST_REFCTL: c_uint = 0x3089;
pub const WCD937X_TX_3_TEST_CTL: c_uint = 0x308A;
pub const WCD937X_TX_3_TEST_BLK_EN: c_uint = 0x308B;
pub const WCD937X_TX_3_TXFE_CLKDIV: c_uint = 0x308C;
pub const WCD937X_TX_3_SPARE_MONO: c_uint = 0x308D;
pub const WCD937X_TX_3_SAR1_ERR: c_uint = 0x308E;
pub const WCD937X_CLASSH_MODE_1: c_uint = 0x3097;
pub const WCD937X_CLASSH_MODE_2: c_uint = 0x3098;
pub const WCD937X_CLASSH_MODE_3: c_uint = 0x3099;
pub const WCD937X_CLASSH_CTRL_VCL_1: c_uint = 0x309A;
pub const WCD937X_CLASSH_CTRL_VCL_2: c_uint = 0x309B;
pub const WCD937X_CLASSH_CTRL_CCL_1: c_uint = 0x309C;
pub const WCD937X_CLASSH_CTRL_CCL_2: c_uint = 0x309D;
pub const WCD937X_CLASSH_CTRL_CCL_3: c_uint = 0x309E;
pub const WCD937X_CLASSH_CTRL_CCL_4: c_uint = 0x309F;
pub const WCD937X_CLASSH_CTRL_CCL_5: c_uint = 0x30A0;
pub const WCD937X_CLASSH_BUCK_TMUX_A_D: c_uint = 0x30A1;
pub const WCD937X_CLASSH_BUCK_SW_DRV_CNTL: c_uint = 0x30A2;
pub const WCD937X_CLASSH_SPARE: c_uint = 0x30A3;
pub const WCD937X_FLYBACK_EN: c_uint = 0x30A4;
pub const WCD937X_FLYBACK_VNEG_CTRL_1: c_uint = 0x30A5;
pub const WCD937X_FLYBACK_VNEG_CTRL_2: c_uint = 0x30A6;
pub const WCD937X_FLYBACK_VNEG_CTRL_3: c_uint = 0x30A7;
pub const WCD937X_FLYBACK_VNEG_CTRL_4: c_uint = 0x30A8;
pub const WCD937X_FLYBACK_VNEG_CTRL_5: c_uint = 0x30A9;
pub const WCD937X_FLYBACK_VNEG_CTRL_6: c_uint = 0x30AA;
pub const WCD937X_FLYBACK_VNEG_CTRL_7: c_uint = 0x30AB;
pub const WCD937X_FLYBACK_VNEG_CTRL_8: c_uint = 0x30AC;
pub const WCD937X_FLYBACK_VNEG_CTRL_9: c_uint = 0x30AD;
pub const WCD937X_FLYBACK_VNEGDAC_CTRL_1: c_uint = 0x30AE;
pub const WCD937X_FLYBACK_VNEGDAC_CTRL_2: c_uint = 0x30AF;
pub const WCD937X_FLYBACK_VNEGDAC_CTRL_3: c_uint = 0x30B0;
pub const WCD937X_FLYBACK_CTRL_1: c_uint = 0x30B1;
pub const WCD937X_FLYBACK_TEST_CTL: c_uint = 0x30B2;
pub const WCD937X_RX_AUX_SW_CTL: c_uint = 0x30B3;
pub const WCD937X_RX_PA_AUX_IN_CONN: c_uint = 0x30B4;
pub const WCD937X_RX_TIMER_DIV: c_uint = 0x30B5;
pub const WCD937X_RX_OCP_CTL: c_uint = 0x30B6;
pub const WCD937X_RX_OCP_COUNT: c_uint = 0x30B7;
pub const WCD937X_RX_BIAS_EAR_DAC: c_uint = 0x30B8;
pub const WCD937X_RX_BIAS_EAR_AMP: c_uint = 0x30B9;
pub const WCD937X_RX_BIAS_HPH_LDO: c_uint = 0x30BA;
pub const WCD937X_RX_BIAS_HPH_PA: c_uint = 0x30BB;
pub const WCD937X_RX_BIAS_HPH_RDACBUFF_CNP2: c_uint = 0x30BC;
pub const WCD937X_RX_BIAS_HPH_RDAC_LDO: c_uint = 0x30BD;
pub const WCD937X_RX_BIAS_HPH_CNP1: c_uint = 0x30BE;
pub const WCD937X_RX_BIAS_HPH_LOWPOWER: c_uint = 0x30BF;
pub const WCD937X_RX_BIAS_AUX_DAC: c_uint = 0x30C0;
pub const WCD937X_RX_BIAS_AUX_AMP: c_uint = 0x30C1;
pub const WCD937X_RX_BIAS_VNEGDAC_BLEEDER: c_uint = 0x30C2;
pub const WCD937X_RX_BIAS_MISC: c_uint = 0x30C3;
pub const WCD937X_RX_BIAS_BUCK_RST: c_uint = 0x30C4;
pub const WCD937X_RX_BIAS_BUCK_VREF_ERRAMP: c_uint = 0x30C5;
pub const WCD937X_RX_BIAS_FLYB_ERRAMP: c_uint = 0x30C6;
pub const WCD937X_RX_BIAS_FLYB_BUFF: c_uint = 0x30C7;
pub const WCD937X_RX_BIAS_FLYB_MID_RST: c_uint = 0x30C8;
pub const WCD937X_HPH_L_STATUS: c_uint = 0x30C9;
pub const WCD937X_HPH_R_STATUS: c_uint = 0x30CA;
pub const WCD937X_HPH_CNP_EN: c_uint = 0x30CB;
pub const WCD937X_HPH_CNP_WG_CTL: c_uint = 0x30CC;
pub const WCD937X_HPH_CNP_WG_TIME: c_uint = 0x30CD;
pub const WCD937X_HPH_OCP_CTL: c_uint = 0x30CE;
pub const WCD937X_HPH_AUTO_CHOP: c_uint = 0x30CF;
pub const WCD937X_HPH_CHOP_CTL: c_uint = 0x30D0;
pub const WCD937X_HPH_PA_CTL1: c_uint = 0x30D1;
pub const WCD937X_HPH_PA_CTL2: c_uint = 0x30D2;

pub const WCD937X_HPH_L_EN: c_uint = 0x30D3;
pub const WCD937X_HPH_L_TEST: c_uint = 0x30D4;
pub const WCD937X_HPH_L_ATEST: c_uint = 0x30D5;
pub const WCD937X_HPH_R_EN: c_uint = 0x30D6;

pub const WCD937X_GAIN_SRC_SEL_REGISTER: c_int = 1;
pub const WCD937X_HPH_R_TEST: c_uint = 0x30D7;
pub const WCD937X_HPH_R_ATEST: c_uint = 0x30D8;
pub const WCD937X_HPH_RDAC_CLK_CTL1: c_uint = 0x30D9;

pub const WCD937X_HPH_RDAC_CLK_CTL2: c_uint = 0x30DA;
pub const WCD937X_HPH_RDAC_LDO_CTL: c_uint = 0x30DB;
pub const WCD937X_HPH_RDAC_CHOP_CLK_LP_CTL: c_uint = 0x30DC;
pub const WCD937X_HPH_REFBUFF_UHQA_CTL: c_uint = 0x30DD;
pub const WCD937X_HPH_REFBUFF_LP_CTL: c_uint = 0x30DE;

pub const WCD937X_HPH_L_DAC_CTL: c_uint = 0x30DF;
pub const WCD937X_HPH_R_DAC_CTL: c_uint = 0x30E0;
pub const WCD937X_HPH_SURGE_HPHLR_SURGE_COMP_SEL: c_uint = 0x30E1;
pub const WCD937X_HPH_SURGE_HPHLR_SURGE_EN: c_uint = 0x30E2;
pub const WCD937X_HPH_SURGE_HPHLR_SURGE_MISC1: c_uint = 0x30E3;
pub const WCD937X_HPH_SURGE_HPHLR_SURGE_STATUS: c_uint = 0x30E4;
pub const WCD937X_EAR_EAR_EN_REG: c_uint = 0x30E9;
pub const WCD937X_EAR_EAR_PA_CON: c_uint = 0x30EA;
pub const WCD937X_EAR_EAR_SP_CON: c_uint = 0x30EB;
pub const WCD937X_EAR_EAR_DAC_CON: c_uint = 0x30EC;
pub const WCD937X_EAR_EAR_CNP_FSM_CON: c_uint = 0x30ED;
pub const WCD937X_EAR_TEST_CTL: c_uint = 0x30EE;
pub const WCD937X_EAR_STATUS_REG_1: c_uint = 0x30EF;
pub const WCD937X_EAR_STATUS_REG_2: c_uint = 0x30F0;
pub const WCD937X_ANA_NEW_PAGE_REGISTER: c_uint = 0x3100;
pub const WCD937X_HPH_NEW_ANA_HPH2: c_uint = 0x3101;
pub const WCD937X_HPH_NEW_ANA_HPH3: c_uint = 0x3102;
pub const WCD937X_SLEEP_CTL: c_uint = 0x3103;
pub const WCD937X_SLEEP_WATCHDOG_CTL: c_uint = 0x3104;
pub const WCD937X_MBHC_NEW_ELECT_REM_CLAMP_CTL: c_uint = 0x311F;
pub const WCD937X_MBHC_NEW_CTL_1: c_uint = 0x3120;

pub const WCD937X_MBHC_BTN_DBNC_T_16_MS: c_uint = 0x2;
pub const WCD937X_MBHC_NEW_CTL_2: c_uint = 0x3121;
pub const WCD937X_MBHC_NEW_PLUG_DETECT_CTL: c_uint = 0x3122;
pub const WCD937X_MBHC_NEW_ZDET_ANA_CTL: c_uint = 0x3123;

pub const WCD937X_MBHC_HS_VREF_1P5_V: c_uint = 0x1;
pub const WCD937X_MBHC_DBNC_TIMER_INSREM_DBNC_T_96_MS: c_uint = 0x6;

pub const WCD937X_MBHC_NEW_ZDET_RAMP_CTL: c_uint = 0x3124;
pub const WCD937X_MBHC_NEW_FSM_STATUS: c_uint = 0x3125;
pub const WCD937X_MBHC_NEW_ADC_RESULT: c_uint = 0x3126;
pub const WCD937X_TX_NEW_TX_CH2_SEL: c_uint = 0x3127;
pub const WCD937X_AUX_AUXPA: c_uint = 0x3128;

pub const WCD937X_LDORXTX_MODE: c_uint = 0x3129;
pub const WCD937X_LDORXTX_CONFIG: c_uint = 0x312A;
pub const WCD937X_DIE_CRACK_DIE_CRK_DET_EN: c_uint = 0x312C;
pub const WCD937X_DIE_CRACK_DIE_CRK_DET_OUT: c_uint = 0x312D;
pub const WCD937X_HPH_NEW_INT_RDAC_GAIN_CTL: c_uint = 0x3132;
pub const WCD937X_HPH_NEW_INT_RDAC_HD2_CTL_L: c_uint = 0x3133;
pub const WCD937X_HPH_NEW_INT_RDAC_VREF_CTL: c_uint = 0x3134;
pub const WCD937X_HPH_NEW_INT_RDAC_OVERRIDE_CTL: c_uint = 0x3135;
pub const WCD937X_HPH_NEW_INT_RDAC_HD2_CTL_R: c_uint = 0x3136;
pub const WCD937X_HPH_NEW_INT_PA_MISC1: c_uint = 0x3137;
pub const WCD937X_HPH_NEW_INT_PA_MISC2: c_uint = 0x3138;
pub const WCD937X_HPH_NEW_INT_PA_RDAC_MISC: c_uint = 0x3139;
pub const WCD937X_HPH_NEW_INT_HPH_TIMER1: c_uint = 0x313A;
pub const WCD937X_HPH_NEW_INT_HPH_TIMER2: c_uint = 0x313B;
pub const WCD937X_HPH_NEW_INT_HPH_TIMER3: c_uint = 0x313C;
pub const WCD937X_HPH_NEW_INT_HPH_TIMER4: c_uint = 0x313D;
pub const WCD937X_HPH_NEW_INT_PA_RDAC_MISC2: c_uint = 0x313E;
pub const WCD937X_HPH_NEW_INT_PA_RDAC_MISC3: c_uint = 0x313F;
pub const WCD937X_RX_NEW_INT_HPH_RDAC_BIAS_LOHIFI: c_uint = 0x3145;
pub const WCD937X_RX_NEW_INT_HPH_RDAC_BIAS_ULP: c_uint = 0x3146;
pub const WCD937X_RX_NEW_INT_HPH_RDAC_LDO_LP: c_uint = 0x3147;
pub const WCD937X_MBHC_NEW_INT_MOISTURE_DET_DC_CTRL: c_uint = 0x31AF;
pub const WCD937X_MBHC_NEW_INT_MOISTURE_DET_POLLING_CTRL: c_uint = 0x31B0;

pub const WCD937X_MBHC_NEW_INT_MECH_DET_CURRENT: c_uint = 0x31B1;
pub const WCD937X_MBHC_NEW_INT_SPARE_2: c_uint = 0x31B2;
pub const WCD937X_EAR_INT_NEW_EAR_CHOPPER_CON: c_uint = 0x31B7;
pub const WCD937X_EAR_INT_NEW_CNP_VCM_CON1: c_uint = 0x31B8;
pub const WCD937X_EAR_INT_NEW_CNP_VCM_CON2: c_uint = 0x31B9;
pub const WCD937X_EAR_INT_NEW_EAR_DYNAMIC_BIAS: c_uint = 0x31BA;
pub const WCD937X_AUX_INT_EN_REG: c_uint = 0x31BD;
pub const WCD937X_AUX_INT_PA_CTRL: c_uint = 0x31BE;
pub const WCD937X_AUX_INT_SP_CTRL: c_uint = 0x31BF;
pub const WCD937X_AUX_INT_DAC_CTRL: c_uint = 0x31C0;
pub const WCD937X_AUX_INT_CLK_CTRL: c_uint = 0x31C1;
pub const WCD937X_AUX_INT_TEST_CTRL: c_uint = 0x31C2;
pub const WCD937X_AUX_INT_STATUS_REG: c_uint = 0x31C3;
pub const WCD937X_AUX_INT_MISC: c_uint = 0x31C4;
pub const WCD937X_LDORXTX_INT_BIAS: c_uint = 0x31C5;
pub const WCD937X_LDORXTX_INT_STB_LOADS_DTEST: c_uint = 0x31C6;
pub const WCD937X_LDORXTX_INT_TEST0: c_uint = 0x31C7;
pub const WCD937X_LDORXTX_INT_STARTUP_TIMER: c_uint = 0x31C8;
pub const WCD937X_LDORXTX_INT_TEST1: c_uint = 0x31C9;
pub const WCD937X_LDORXTX_INT_STATUS: c_uint = 0x31CA;
pub const WCD937X_SLEEP_INT_WATCHDOG_CTL_1: c_uint = 0x31D0;
pub const WCD937X_SLEEP_INT_WATCHDOG_CTL_2: c_uint = 0x31D1;
pub const WCD937X_DIE_CRACK_INT_DIE_CRK_DET_INT1: c_uint = 0x31D3;
pub const WCD937X_DIE_CRACK_INT_DIE_CRK_DET_INT2: c_uint = 0x31D4;
pub const WCD937X_DIGITAL_PAGE_REGISTER: c_uint = 0x3400;
pub const WCD937X_DIGITAL_CHIP_ID0: c_uint = 0x3401;
pub const WCD937X_DIGITAL_CHIP_ID1: c_uint = 0x3402;
pub const WCD937X_DIGITAL_CHIP_ID2: c_uint = 0x3403;
pub const WCD937X_DIGITAL_CHIP_ID3: c_uint = 0x3404;
pub const WCD937X_DIGITAL_CDC_RST_CTL: c_uint = 0x3406;
pub const WCD937X_DIGITAL_TOP_CLK_CFG: c_uint = 0x3407;
pub const WCD937X_DIGITAL_CDC_ANA_CLK_CTL: c_uint = 0x3408;
pub const WCD937X_DIGITAL_CDC_DIG_CLK_CTL: c_uint = 0x3409;
pub const WCD937X_DIGITAL_SWR_RST_EN: c_uint = 0x340A;
pub const WCD937X_DIGITAL_CDC_PATH_MODE: c_uint = 0x340B;
pub const WCD937X_DIGITAL_CDC_RX_RST: c_uint = 0x340C;
pub const WCD937X_DIGITAL_CDC_RX0_CTL: c_uint = 0x340D;
pub const WCD937X_DIGITAL_CDC_RX1_CTL: c_uint = 0x340E;
pub const WCD937X_DIGITAL_CDC_RX2_CTL: c_uint = 0x340F;
pub const WCD937X_DIGITAL_DEM_BYPASS_DATA0: c_uint = 0x3410;
pub const WCD937X_DIGITAL_DEM_BYPASS_DATA1: c_uint = 0x3411;
pub const WCD937X_DIGITAL_DEM_BYPASS_DATA2: c_uint = 0x3412;
pub const WCD937X_DIGITAL_DEM_BYPASS_DATA3: c_uint = 0x3413;
pub const WCD937X_DIGITAL_CDC_COMP_CTL_0: c_uint = 0x3414;
pub const WCD937X_DIGITAL_CDC_RX_DELAY_CTL: c_uint = 0x3417;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A1_0: c_uint = 0x3418;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A1_1: c_uint = 0x3419;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A2_0: c_uint = 0x341A;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A2_1: c_uint = 0x341B;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A3_0: c_uint = 0x341C;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A3_1: c_uint = 0x341D;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A4_0: c_uint = 0x341E;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A4_1: c_uint = 0x341F;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A5_0: c_uint = 0x3420;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A5_1: c_uint = 0x3421;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A6_0: c_uint = 0x3422;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_A7_0: c_uint = 0x3423;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_C_0: c_uint = 0x3424;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_C_1: c_uint = 0x3425;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_C_2: c_uint = 0x3426;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_C_3: c_uint = 0x3427;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R1: c_uint = 0x3428;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R2: c_uint = 0x3429;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R3: c_uint = 0x342A;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R4: c_uint = 0x342B;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R5: c_uint = 0x342C;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R6: c_uint = 0x342D;
pub const WCD937X_DIGITAL_CDC_HPH_DSM_R7: c_uint = 0x342E;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A1_0: c_uint = 0x342F;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A1_1: c_uint = 0x3430;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A2_0: c_uint = 0x3431;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A2_1: c_uint = 0x3432;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A3_0: c_uint = 0x3433;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A3_1: c_uint = 0x3434;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A4_0: c_uint = 0x3435;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A4_1: c_uint = 0x3436;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A5_0: c_uint = 0x3437;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A5_1: c_uint = 0x3438;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A6_0: c_uint = 0x3439;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_A7_0: c_uint = 0x343A;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_C_0: c_uint = 0x343B;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_C_1: c_uint = 0x343C;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_C_2: c_uint = 0x343D;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_C_3: c_uint = 0x343E;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R1: c_uint = 0x343F;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R2: c_uint = 0x3440;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R3: c_uint = 0x3441;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R4: c_uint = 0x3442;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R5: c_uint = 0x3443;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R6: c_uint = 0x3444;
pub const WCD937X_DIGITAL_CDC_AUX_DSM_R7: c_uint = 0x3445;
pub const WCD937X_DIGITAL_CDC_HPH_GAIN_RX_0: c_uint = 0x3446;
pub const WCD937X_DIGITAL_CDC_HPH_GAIN_RX_1: c_uint = 0x3447;
pub const WCD937X_DIGITAL_CDC_HPH_GAIN_DSD_0: c_uint = 0x3448;
pub const WCD937X_DIGITAL_CDC_HPH_GAIN_DSD_1: c_uint = 0x3449;
pub const WCD937X_DIGITAL_CDC_HPH_GAIN_DSD_2: c_uint = 0x344A;
pub const WCD937X_DIGITAL_CDC_AUX_GAIN_DSD_0: c_uint = 0x344B;
pub const WCD937X_DIGITAL_CDC_AUX_GAIN_DSD_1: c_uint = 0x344C;
pub const WCD937X_DIGITAL_CDC_AUX_GAIN_DSD_2: c_uint = 0x344D;
pub const WCD937X_DIGITAL_CDC_HPH_GAIN_CTL: c_uint = 0x344E;
pub const WCD937X_DIGITAL_CDC_AUX_GAIN_CTL: c_uint = 0x344F;
pub const WCD937X_DIGITAL_CDC_EAR_PATH_CTL: c_uint = 0x3450;
pub const WCD937X_DIGITAL_CDC_SWR_CLH: c_uint = 0x3451;
pub const WCD937X_DIGITAL_SWR_CLH_BYP: c_uint = 0x3452;
pub const WCD937X_DIGITAL_CDC_TX0_CTL: c_uint = 0x3453;
pub const WCD937X_DIGITAL_CDC_TX1_CTL: c_uint = 0x3454;
pub const WCD937X_DIGITAL_CDC_TX2_CTL: c_uint = 0x3455;
pub const WCD937X_DIGITAL_CDC_TX_RST: c_uint = 0x3456;
pub const WCD937X_DIGITAL_CDC_REQ_CTL: c_uint = 0x3457;
pub const WCD937X_DIGITAL_CDC_AMIC_CTL: c_uint = 0x345A;
pub const WCD937X_DIGITAL_CDC_DMIC_CTL: c_uint = 0x345B;
pub const WCD937X_DIGITAL_CDC_DMIC1_CTL: c_uint = 0x345C;
pub const WCD937X_DIGITAL_CDC_DMIC2_CTL: c_uint = 0x345D;
pub const WCD937X_DIGITAL_CDC_DMIC3_CTL: c_uint = 0x345E;
pub const WCD937X_DIGITAL_EFUSE_CTL: c_uint = 0x345F;
pub const WCD937X_DIGITAL_EFUSE_PRG_CTL: c_uint = 0x3460;
pub const WCD937X_DIGITAL_EFUSE_TEST_CTL_0: c_uint = 0x3461;
pub const WCD937X_DIGITAL_EFUSE_TEST_CTL_1: c_uint = 0x3462;
pub const WCD937X_DIGITAL_EFUSE_T_DATA_0: c_uint = 0x3463;
pub const WCD937X_DIGITAL_EFUSE_T_DATA_1: c_uint = 0x3464;
pub const WCD937X_DIGITAL_PDM_WD_CTL0: c_uint = 0x3465;
pub const WCD937X_DIGITAL_PDM_WD_CTL1: c_uint = 0x3466;
pub const WCD937X_DIGITAL_PDM_WD_CTL2: c_uint = 0x3467;

pub const WCD937X_DIGITAL_INTR_MODE: c_uint = 0x346A;
pub const WCD937X_DIGITAL_INTR_MASK_0: c_uint = 0x346B;
pub const WCD937X_DIGITAL_INTR_MASK_1: c_uint = 0x346C;
pub const WCD937X_DIGITAL_INTR_MASK_2: c_uint = 0x346D;
pub const WCD937X_DIGITAL_INTR_STATUS_0: c_uint = 0x346E;
pub const WCD937X_DIGITAL_INTR_STATUS_1: c_uint = 0x346F;
pub const WCD937X_DIGITAL_INTR_STATUS_2: c_uint = 0x3470;
pub const WCD937X_DIGITAL_INTR_CLEAR_0: c_uint = 0x3471;
pub const WCD937X_DIGITAL_INTR_CLEAR_1: c_uint = 0x3472;
pub const WCD937X_DIGITAL_INTR_CLEAR_2: c_uint = 0x3473;
pub const WCD937X_DIGITAL_INTR_LEVEL_0: c_uint = 0x3474;
pub const WCD937X_DIGITAL_INTR_LEVEL_1: c_uint = 0x3475;
pub const WCD937X_DIGITAL_INTR_LEVEL_2: c_uint = 0x3476;
pub const WCD937X_DIGITAL_INTR_SET_0: c_uint = 0x3477;
pub const WCD937X_DIGITAL_INTR_SET_1: c_uint = 0x3478;
pub const WCD937X_DIGITAL_INTR_SET_2: c_uint = 0x3479;
pub const WCD937X_DIGITAL_INTR_TEST_0: c_uint = 0x347A;
pub const WCD937X_DIGITAL_INTR_TEST_1: c_uint = 0x347B;
pub const WCD937X_DIGITAL_INTR_TEST_2: c_uint = 0x347C;
pub const WCD937X_DIGITAL_CDC_CONN_RX0_CTL: c_uint = 0x347F;
pub const WCD937X_DIGITAL_CDC_CONN_RX1_CTL: c_uint = 0x3480;
pub const WCD937X_DIGITAL_CDC_CONN_RX2_CTL: c_uint = 0x3481;
pub const WCD937X_DIGITAL_CDC_CONN_TX_CTL: c_uint = 0x3482;
pub const WCD937X_DIGITAL_LOOP_BACK_MODE: c_uint = 0x3483;
pub const WCD937X_DIGITAL_SWR_DAC_TEST: c_uint = 0x3484;
pub const WCD937X_DIGITAL_SWR_HM_TEST_RX_0: c_uint = 0x3485;
pub const WCD937X_DIGITAL_SWR_HM_TEST_TX_0: c_uint = 0x3491;
pub const WCD937X_DIGITAL_SWR_HM_TEST_RX_1: c_uint = 0x3492;
pub const WCD937X_DIGITAL_SWR_HM_TEST_TX_1: c_uint = 0x3493;
pub const WCD937X_DIGITAL_SWR_HM_TEST: c_uint = 0x3494;
pub const WCD937X_DIGITAL_PAD_CTL_PDM_RX0: c_uint = 0x3495;
pub const WCD937X_DIGITAL_PAD_CTL_PDM_RX1: c_uint = 0x3496;
pub const WCD937X_DIGITAL_PAD_CTL_PDM_TX0: c_uint = 0x3497;
pub const WCD937X_DIGITAL_PAD_CTL_PDM_TX1: c_uint = 0x3498;
pub const WCD937X_DIGITAL_PAD_INP_DIS_0: c_uint = 0x3499;
pub const WCD937X_DIGITAL_PAD_INP_DIS_1: c_uint = 0x349A;
pub const WCD937X_DIGITAL_DRIVE_STRENGTH_0: c_uint = 0x349B;
pub const WCD937X_DIGITAL_DRIVE_STRENGTH_1: c_uint = 0x349C;
pub const WCD937X_DIGITAL_DRIVE_STRENGTH_2: c_uint = 0x349D;
pub const WCD937X_DIGITAL_RX_DATA_EDGE_CTL: c_uint = 0x349E;
pub const WCD937X_DIGITAL_TX_DATA_EDGE_CTL: c_uint = 0x349F;
pub const WCD937X_DIGITAL_GPIO_MODE: c_uint = 0x34A0;
pub const WCD937X_DIGITAL_PIN_CTL_OE: c_uint = 0x34A1;
pub const WCD937X_DIGITAL_PIN_CTL_DATA_0: c_uint = 0x34A2;
pub const WCD937X_DIGITAL_PIN_CTL_DATA_1: c_uint = 0x34A3;
pub const WCD937X_DIGITAL_PIN_STATUS_0: c_uint = 0x34A4;
pub const WCD937X_DIGITAL_PIN_STATUS_1: c_uint = 0x34A5;
pub const WCD937X_DIGITAL_DIG_DEBUG_CTL: c_uint = 0x34A6;
pub const WCD937X_DIGITAL_DIG_DEBUG_EN: c_uint = 0x34A7;
pub const WCD937X_DIGITAL_ANA_CSR_DBG_ADD: c_uint = 0x34A8;
pub const WCD937X_DIGITAL_ANA_CSR_DBG_CTL: c_uint = 0x34A9;
pub const WCD937X_DIGITAL_SSP_DBG: c_uint = 0x34AA;
pub const WCD937X_DIGITAL_MODE_STATUS_0: c_uint = 0x34AB;
pub const WCD937X_DIGITAL_MODE_STATUS_1: c_uint = 0x34AC;
pub const WCD937X_DIGITAL_SPARE_0: c_uint = 0x34AD;
pub const WCD937X_DIGITAL_SPARE_1: c_uint = 0x34AE;
pub const WCD937X_DIGITAL_SPARE_2: c_uint = 0x34AF;
pub const WCD937X_DIGITAL_EFUSE_REG_0: c_uint = 0x34B0;
pub const WCD937X_DIGITAL_EFUSE_REG_1: c_uint = 0x34B1;
pub const WCD937X_DIGITAL_EFUSE_REG_2: c_uint = 0x34B2;
pub const WCD937X_DIGITAL_EFUSE_REG_3: c_uint = 0x34B3;
pub const WCD937X_DIGITAL_EFUSE_REG_4: c_uint = 0x34B4;
pub const WCD937X_DIGITAL_EFUSE_REG_5: c_uint = 0x34B5;
pub const WCD937X_DIGITAL_EFUSE_REG_6: c_uint = 0x34B6;
pub const WCD937X_DIGITAL_EFUSE_REG_7: c_uint = 0x34B7;
pub const WCD937X_DIGITAL_EFUSE_REG_8: c_uint = 0x34B8;
pub const WCD937X_DIGITAL_EFUSE_REG_9: c_uint = 0x34B9;
pub const WCD937X_DIGITAL_EFUSE_REG_10: c_uint = 0x34BA;
pub const WCD937X_DIGITAL_EFUSE_REG_11: c_uint = 0x34BB;
pub const WCD937X_DIGITAL_EFUSE_REG_12: c_uint = 0x34BC;
pub const WCD937X_DIGITAL_EFUSE_REG_13: c_uint = 0x34BD;
pub const WCD937X_DIGITAL_EFUSE_REG_14: c_uint = 0x34BE;
pub const WCD937X_DIGITAL_EFUSE_REG_15: c_uint = 0x34BF;
pub const WCD937X_DIGITAL_EFUSE_REG_16: c_uint = 0x34C0;
pub const WCD937X_DIGITAL_EFUSE_REG_17: c_uint = 0x34C1;
pub const WCD937X_DIGITAL_EFUSE_REG_18: c_uint = 0x34C2;
pub const WCD937X_DIGITAL_EFUSE_REG_19: c_uint = 0x34C3;
pub const WCD937X_DIGITAL_EFUSE_REG_20: c_uint = 0x34C4;
pub const WCD937X_DIGITAL_EFUSE_REG_21: c_uint = 0x34C5;
pub const WCD937X_DIGITAL_EFUSE_REG_22: c_uint = 0x34C6;
pub const WCD937X_DIGITAL_EFUSE_REG_23: c_uint = 0x34C7;
pub const WCD937X_DIGITAL_EFUSE_REG_24: c_uint = 0x34C8;
pub const WCD937X_DIGITAL_EFUSE_REG_25: c_uint = 0x34C9;
pub const WCD937X_DIGITAL_EFUSE_REG_26: c_uint = 0x34CA;
pub const WCD937X_DIGITAL_EFUSE_REG_27: c_uint = 0x34CB;
pub const WCD937X_DIGITAL_EFUSE_REG_28: c_uint = 0x34CC;
pub const WCD937X_DIGITAL_EFUSE_REG_29: c_uint = 0x34CD;
pub const WCD937X_DIGITAL_EFUSE_REG_30: c_uint = 0x34CE;
pub const WCD937X_DIGITAL_EFUSE_REG_31: c_uint = 0x34CF;

pub const WCD937X_MAX_MICBIAS: c_int = 3;
pub const WCD937X_MAX_SWR_CH_IDS: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd937x_tx_sdw_ports {
    WCD937X_ADC_1_PORT = 1,
    WCD937X_ADC_2_3_PORT,
    WCD937X_DMIC_0_3_MBHC_PORT,
    WCD937X_DMIC_4_6_PORT,
    WCD937X_MAX_TX_SWR_PORTS = WCD937X_DMIC_4_6_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd937x_rx_sdw_ports {
    WCD937X_HPH_PORT = 1,
    WCD937X_CLSH_PORT,
    WCD937X_COMP_PORT,
    WCD937X_LO_PORT,
    WCD937X_DSD_PORT,
    WCD937X_MAX_SWR_PORTS = WCD937X_DSD_PORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd937x_sdw_priv {
    pub sdev: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; WCD937X_MAX_SWR_PORTS],
    pub ch_info: *mut wcd_sdw_ch_info,
    pub port_enable: [bool; WCD937X_MAX_SWR_CH_IDS],
    pub master_channel_map: [c_uint; SDW_MAX_PORTS],
    pub active_ports: c_int,
    pub num_ports: c_int,
    pub is_tx: bool,
    pub wcd937x: *mut wcd937x_priv,
    pub slave_irq: *mut irq_domain,
    pub regmap: *mut regmap,
}

// INTR_CTRL_INT_MASK_0
// INTR_CTRL_INT_MASK_1
// INTR_CTRL_INT_MASK_2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd937x_tx_sdw_channels {
    WCD937X_ADC1,
    WCD937X_ADC2,
    WCD937X_ADC3,
    WCD937X_DMIC0,
    WCD937X_DMIC1,
    WCD937X_MBHC,
    WCD937X_DMIC2,
    WCD937X_DMIC3,
    WCD937X_DMIC4,
    WCD937X_DMIC5,
    WCD937X_DMIC6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd937x_rx_sdw_channels {
    WCD937X_HPH_L,
    WCD937X_HPH_R,
    WCD937X_CLSH,
    WCD937X_COMP_L,
    WCD937X_COMP_R,
    WCD937X_LO,
    WCD937X_DSD_R,
    WCD937X_DSD_L,
}
