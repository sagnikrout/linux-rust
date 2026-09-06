//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wcd934x/registers.h
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
pub const WCD934X_CODEC_RPM_CLK_GATE: c_uint = 0x0002;

pub const WCD934X_CODEC_RPM_CLK_MCLK_CFG: c_uint = 0x0003;

pub const WCD934X_CODEC_RPM_RST_CTL: c_uint = 0x0009;
pub const WCD934X_CODEC_RPM_PWR_CDC_DIG_HM_CTL: c_uint = 0x0011;
pub const WCD934X_CHIP_TIER_CTRL_CHIP_ID_BYTE0: c_uint = 0x0021;
pub const WCD934X_CHIP_TIER_CTRL_CHIP_ID_BYTE2: c_uint = 0x0023;
pub const WCD934X_CHIP_TIER_CTRL_EFUSE_CTL: c_uint = 0x0025;

pub const WCD934X_EFUSE_SENSE_STATE_DEF: c_uint = 0x10;

pub const WCD934X_CHIP_TIER_CTRL_EFUSE_VAL_OUT1: c_uint = 0x002a;
pub const WCD934X_CHIP_TIER_CTRL_EFUSE_VAL_OUT2: c_uint = 0x002b;
pub const WCD934X_CHIP_TIER_CTRL_EFUSE_VAL_OUT14: c_uint = 0x0037;
pub const WCD934X_CHIP_TIER_CTRL_EFUSE_VAL_OUT15: c_uint = 0x0038;
pub const WCD934X_CHIP_TIER_CTRL_EFUSE_STATUS: c_uint = 0x0039;
pub const WCD934X_DATA_HUB_SB_TX10_INP_CFG: c_uint = 0x006b;
pub const WCD934X_DATA_HUB_SB_TX11_INP_CFG: c_uint = 0x006c;
pub const WCD934X_DATA_HUB_SB_TX13_INP_CFG: c_uint = 0x006e;
pub const WCD934X_CPE_FLL_CONFIG_CTL_2: c_uint = 0x0111;
pub const WCD934X_CPE_SS_CPARMAD_BUFRDY_INT_PERIOD: c_uint = 0x0213;
pub const WCD934X_CPE_SS_SVA_CFG: c_uint = 0x0214;
pub const WCD934X_CPE_SS_DMIC0_CTL: c_uint = 0x0218;
pub const WCD934X_CPE_SS_DMIC1_CTL: c_uint = 0x0219;

pub const WCD934X_CPE_SS_DMIC2_CTL: c_uint = 0x021a;
pub const WCD934X_CPE_SS_DMIC_CFG: c_uint = 0x021b;
pub const WCD934X_CPE_SS_DMIC_CFG: c_uint = 0x021b;
pub const WCD934X_CPE_SS_CPAR_CFG: c_uint = 0x021c;
pub const WCD934X_INTR_PIN1_MASK0: c_uint = 0x0409;
pub const WCD934X_INTR_PIN1_STATUS0: c_uint = 0x0411;
pub const WCD934X_INTR_PIN1_CLEAR0: c_uint = 0x0419;
pub const WCD934X_INTR_PIN2_CLEAR3: c_uint = 0x0434;
pub const WCD934X_INTR_LEVEL0: c_uint = 0x0461;
// INTR_REG 0
pub const WCD934X_IRQ_SLIMBUS: c_int = 0;
pub const WCD934X_IRQ_MISC: c_int = 1;
pub const WCD934X_IRQ_HPH_PA_OCPL_FAULT: c_int = 2;
pub const WCD934X_IRQ_HPH_PA_OCPR_FAULT: c_int = 3;
pub const WCD934X_IRQ_EAR_PA_OCP_FAULT: c_int = 4;
pub const WCD934X_IRQ_HPH_PA_CNPL_COMPLETE: c_int = 5;
pub const WCD934X_IRQ_HPH_PA_CNPR_COMPLETE: c_int = 6;
pub const WCD934X_IRQ_EAR_PA_CNP_COMPLETE: c_int = 7;
// INTR_REG 1
pub const WCD934X_IRQ_MBHC_SW_DET: c_int = 8;
pub const WCD934X_IRQ_MBHC_ELECT_INS_REM_DET: c_int = 9;
pub const WCD934X_IRQ_MBHC_BUTTON_PRESS_DET: c_int = 10;
pub const WCD934X_IRQ_MBHC_BUTTON_RELEASE_DET: c_int = 11;
pub const WCD934X_IRQ_MBHC_ELECT_INS_REM_LEG_DET: c_int = 12;
pub const WCD934X_IRQ_RESERVED_0: c_int = 13;
pub const WCD934X_IRQ_RESERVED_1: c_int = 14;
pub const WCD934X_IRQ_RESERVED_2: c_int = 15;
// INTR_REG 2
pub const WCD934X_IRQ_LINE_PA1_CNP_COMPLETE: c_int = 16;
pub const WCD934X_IRQ_LINE_PA2_CNP_COMPLETE: c_int = 17;
pub const WCD934X_IRQ_SLNQ_ANALOG_ERROR: c_int = 18;
pub const WCD934X_IRQ_RESERVED_3: c_int = 19;
pub const WCD934X_IRQ_SOUNDWIRE: c_int = 20;
pub const WCD934X_IRQ_VDD_DIG_RAMP_COMPLETE: c_int = 21;
pub const WCD934X_IRQ_RCO_ERROR: c_int = 22;
pub const WCD934X_IRQ_CPE_ERROR: c_int = 23;
// INTR_REG 3
pub const WCD934X_IRQ_MAD_AUDIO: c_int = 24;
pub const WCD934X_IRQ_MAD_BEACON: c_int = 25;
pub const WCD934X_IRQ_MAD_ULTRASOUND: c_int = 26;
pub const WCD934X_IRQ_VBAT_ATTACK: c_int = 27;
pub const WCD934X_IRQ_VBAT_RESTORE: c_int = 28;
pub const WCD934X_IRQ_CPE1_INTR: c_int = 29;
pub const WCD934X_IRQ_RESERVED_4: c_int = 30;
pub const WCD934X_IRQ_SLNQ_DIGITAL: c_int = 31;
pub const WCD934X_NUM_IRQS: c_int = 32;
pub const WCD934X_ANA_BIAS: c_uint = 0x0601;

pub const WCD934X_ANA_RCO: c_uint = 0x0603;

pub const WCD934X_ANA_BUCK_CTL: c_uint = 0x0606;

pub const WCD934X_ANA_RX_SUPPLIES: c_uint = 0x0608;
pub const WCD934X_ANA_HPH: c_uint = 0x0609;
pub const WCD934X_ANA_EAR: c_uint = 0x060a;
pub const WCD934X_ANA_LO_1_2: c_uint = 0x060b;
pub const WCD934X_ANA_AMIC1: c_uint = 0x060e;
pub const WCD934X_ANA_AMIC2: c_uint = 0x060f;
pub const WCD934X_ANA_AMIC3: c_uint = 0x0610;
pub const WCD934X_ANA_AMIC4: c_uint = 0x0611;
pub const WCD934X_ANA_MBHC_MECH: c_uint = 0x0614;

pub const WCD934X_MBHC_MECH_DETECT_TYPE_INS: c_int = 1;

pub const WCD934X_MBHC_HPHL_PLUG_TYPE_NO: c_int = 1;

pub const WCD934X_MBHC_GND_PLUG_TYPE_NO: c_int = 1;

pub const WCD934X_ANA_MBHC_ELECT: c_uint = 0x0615;

pub const WCD934X_ANA_MBHC_ZDET: c_uint = 0x0616;
pub const WCD934X_ANA_MBHC_RESULT_1: c_uint = 0x0617;
pub const WCD934X_ANA_MBHC_RESULT_2: c_uint = 0x0618;
pub const WCD934X_ANA_MBHC_RESULT_3: c_uint = 0x0619;
pub const WCD934X_ANA_MBHC_BTN0: c_uint = 0x061a;

pub const WCD934X_ANA_MBHC_BTN1: c_uint = 0x061b;
pub const WCD934X_ANA_MBHC_BTN2: c_uint = 0x061c;
pub const WCD934X_ANA_MBHC_BTN3: c_uint = 0x061d;
pub const WCD934X_ANA_MBHC_BTN4: c_uint = 0x061e;
pub const WCD934X_ANA_MBHC_BTN5: c_uint = 0x061f;
pub const WCD934X_ANA_MBHC_BTN6: c_uint = 0x0620;
pub const WCD934X_ANA_MBHC_BTN7: c_uint = 0x0621;

pub const WCD934X_ANA_MICB1: c_uint = 0x0622;

pub const WCD934X_MICB_DISABLE: c_int = 0;
pub const WCD934X_MICB_ENABLE: c_int = 1;
pub const WCD934X_MICB_PULL_UP: c_int = 2;
pub const WCD934X_MICB_PULL_DOWN: c_int = 3;
pub const WCD934X_ANA_MICB_PULL_UP: c_uint = 0x80;
pub const WCD934X_ANA_MICB_ENABLE: c_uint = 0x40;
pub const WCD934X_ANA_MICB_DISABLE: c_uint = 0x0;
pub const WCD934X_ANA_MICB2: c_uint = 0x0623;

pub const WCD934X_ANA_MICB2_RAMP: c_uint = 0x0624;

pub const WCD934X_ANA_MICB3: c_uint = 0x0625;
pub const WCD934X_ANA_MICB4: c_uint = 0x0626;
pub const WCD934X_BIAS_VBG_FINE_ADJ: c_uint = 0x0629;
pub const WCD934X_MBHC_CTL_CLK: c_uint = 0x0656;
pub const WCD934X_MBHC_CTL_BCS: c_uint = 0x065a;
pub const WCD934X_MBHC_STATUS_SPARE_1: c_uint = 0x065b;
pub const WCD934X_MICB1_TEST_CTL_1: c_uint = 0x066b;
pub const WCD934X_MICB1_TEST_CTL_2: c_uint = 0x066c;
pub const WCD934X_MICB2_TEST_CTL_1: c_uint = 0x066e;
pub const WCD934X_MICB3_TEST_CTL_1: c_uint = 0x0671;
pub const WCD934X_MICB4_TEST_CTL_1: c_uint = 0x0674;
pub const WCD934X_CLASSH_MODE_1: c_uint = 0x0697;
pub const WCD934X_CLASSH_MODE_2: c_uint = 0x0698;
pub const WCD934X_CLASSH_MODE_3: c_uint = 0x0699;
pub const WCD934X_CLASSH_CTRL_VCL_1: c_uint = 0x069a;
pub const WCD934X_CLASSH_CTRL_VCL_2: c_uint = 0x069b;
pub const WCD934X_CLASSH_CTRL_CCL_1: c_uint = 0x069c;
pub const WCD934X_CLASSH_CTRL_CCL_2: c_uint = 0x069d;
pub const WCD934X_CLASSH_CTRL_CCL_3: c_uint = 0x069e;
pub const WCD934X_CLASSH_CTRL_CCL_4: c_uint = 0x069f;
pub const WCD934X_CLASSH_CTRL_CCL_5: c_uint = 0x06a0;
pub const WCD934X_CLASSH_BUCK_TMUX_A_D: c_uint = 0x06a1;
pub const WCD934X_CLASSH_BUCK_SW_DRV_CNTL: c_uint = 0x06a2;
pub const WCD934X_RX_OCP_CTL: c_uint = 0x06b6;
pub const WCD934X_RX_OCP_COUNT: c_uint = 0x06b7;
pub const WCD934X_HPH_CNP_EN: c_uint = 0x06cb;
pub const WCD934X_HPH_CNP_WG_CTL: c_uint = 0x06cc;

pub const WCD934X_HPH_CNP_WG_TIME: c_uint = 0x06cd;
pub const WCD934X_HPH_OCP_CTL: c_uint = 0x06ce;
pub const WCD934X_HPH_PA_CTL2: c_uint = 0x06d2;

pub const WCD934X_HPH_L_EN: c_uint = 0x06d3;

pub const WCD934X_HPH_GAIN_SRC_SEL_COMPANDER: c_int = 0;

pub const WCD934X_HPH_L_TEST: c_uint = 0x06d4;
pub const WCD934X_HPH_R_EN: c_uint = 0x06d6;
pub const WCD934X_HPH_R_TEST: c_uint = 0x06d7;

pub const WCD934X_HPH_OCP_DET_DISABLE: c_int = 0;
pub const WCD934X_HPH_R_ATEST: c_uint = 0x06d8;

pub const WCD934X_DIFF_LO_LO2_COMPANDER: c_uint = 0x06ea;
pub const WCD934X_DIFF_LO_LO1_COMPANDER: c_uint = 0x06eb;
pub const WCD934X_CLK_SYS_MCLK_PRG: c_uint = 0x0711;

pub const WCD934X_EXT_CLK_DIV_BY_2: c_uint = 0x10;

pub const WCD934X_MCLK_SRC_EXT_CLK: c_int = 0;

pub const WCD934X_CLK_SYS_MCLK2_PRG1: c_uint = 0x0712;
pub const WCD934X_CLK_SYS_MCLK2_PRG2: c_uint = 0x0713;
pub const WCD934X_SIDO_NEW_VOUT_A_STARTUP: c_uint = 0x071b;
pub const WCD934X_SIDO_NEW_VOUT_D_STARTUP: c_uint = 0x071c;
pub const WCD934X_SIDO_NEW_VOUT_D_FREQ1: c_uint = 0x071d;
pub const WCD934X_SIDO_NEW_VOUT_D_FREQ2: c_uint = 0x071e;

pub const WCD934X_MBHC_NEW_CTL_1: c_uint = 0x0720;

pub const WCD934X_MBHC_NEW_CTL_2: c_uint = 0x0721;

pub const WCD934X_MBHC_NEW_PLUG_DETECT_CTL: c_uint = 0x0722;

pub const WCD934X_MBHC_NEW_ZDET_ANA_CTL: c_uint = 0x0723;

pub const WCD934X_MBHC_NEW_ZDET_RAMP_CTL: c_uint = 0x0724;
pub const WCD934X_MBHC_NEW_FSM_STATUS: c_uint = 0x0725;
pub const WCD934X_MBHC_NEW_ADC_RESULT: c_uint = 0x0726;
pub const WCD934X_TX_NEW_AMIC_4_5_SEL: c_uint = 0x0727;
pub const WCD934X_HPH_NEW_INT_RDAC_HD2_CTL_L: c_uint = 0x0733;
pub const WCD934X_HPH_NEW_INT_RDAC_OVERRIDE_CTL: c_uint = 0x0735;
pub const WCD934X_HPH_NEW_INT_RDAC_HD2_CTL_R: c_uint = 0x0736;
pub const WCD934X_HPH_NEW_INT_HPH_TIMER1: c_uint = 0x073a;

pub const WCD934X_CDC_TX0_TX_PATH_CTL: c_uint = 0x0a31;

pub const WCD934X_CDC_TX0_TX_PATH_CFG0: c_uint = 0x0a32;
pub const WCD934X_CDC_TX0_TX_PATH_CFG1: c_uint = 0x0a33;
pub const WCD934X_CDC_TX0_TX_VOL_CTL: c_uint = 0x0a34;
pub const WCD934X_CDC_TX0_TX_PATH_192_CTL: c_uint = 0x0a35;
pub const WCD934X_CDC_TX0_TX_PATH_192_CFG: c_uint = 0x0a36;
pub const WCD934X_CDC_TX0_TX_PATH_SEC2: c_uint = 0x0a39;

pub const WCD934X_CDC_TX1_TX_PATH_CTL: c_uint = 0x0a41;
pub const WCD934X_CDC_TX1_TX_PATH_CFG0: c_uint = 0x0a42;
pub const WCD934X_CDC_TX1_TX_PATH_CFG1: c_uint = 0x0a43;
pub const WCD934X_CDC_TX1_TX_VOL_CTL: c_uint = 0x0a44;
pub const WCD934X_CDC_TX2_TX_PATH_CTL: c_uint = 0x0a51;
pub const WCD934X_CDC_TX2_TX_PATH_CFG0: c_uint = 0x0a52;
pub const WCD934X_CDC_TX2_TX_PATH_CFG1: c_uint = 0x0a53;
pub const WCD934X_CDC_TX2_TX_VOL_CTL: c_uint = 0x0a54;
pub const WCD934X_CDC_TX3_TX_PATH_CTL: c_uint = 0x0a61;
pub const WCD934X_CDC_TX3_TX_PATH_CFG0: c_uint = 0x0a62;
pub const WCD934X_CDC_TX3_TX_PATH_CFG1: c_uint = 0x0a63;
pub const WCD934X_CDC_TX3_TX_VOL_CTL: c_uint = 0x0a64;
pub const WCD934X_CDC_TX3_TX_PATH_192_CTL: c_uint = 0x0a65;
pub const WCD934X_CDC_TX3_TX_PATH_192_CFG: c_uint = 0x0a66;
pub const WCD934X_CDC_TX4_TX_PATH_CTL: c_uint = 0x0a71;
pub const WCD934X_CDC_TX4_TX_PATH_CFG0: c_uint = 0x0a72;
pub const WCD934X_CDC_TX4_TX_PATH_CFG1: c_uint = 0x0a73;
pub const WCD934X_CDC_TX4_TX_VOL_CTL: c_uint = 0x0a74;
pub const WCD934X_CDC_TX4_TX_PATH_192_CTL: c_uint = 0x0a75;
pub const WCD934X_CDC_TX4_TX_PATH_192_CFG: c_uint = 0x0a76;
pub const WCD934X_CDC_TX5_TX_PATH_CTL: c_uint = 0x0a81;
pub const WCD934X_CDC_TX5_TX_PATH_CFG0: c_uint = 0x0a82;
pub const WCD934X_CDC_TX5_TX_PATH_CFG1: c_uint = 0x0a83;
pub const WCD934X_CDC_TX5_TX_VOL_CTL: c_uint = 0x0a84;
pub const WCD934X_CDC_TX5_TX_PATH_192_CTL: c_uint = 0x0a85;
pub const WCD934X_CDC_TX5_TX_PATH_192_CFG: c_uint = 0x0a86;
pub const WCD934X_CDC_TX6_TX_PATH_CTL: c_uint = 0x0a91;
pub const WCD934X_CDC_TX6_TX_PATH_CFG0: c_uint = 0x0a92;
pub const WCD934X_CDC_TX6_TX_PATH_CFG1: c_uint = 0x0a93;
pub const WCD934X_CDC_TX6_TX_VOL_CTL: c_uint = 0x0a94;
pub const WCD934X_CDC_TX6_TX_PATH_192_CTL: c_uint = 0x0a95;
pub const WCD934X_CDC_TX6_TX_PATH_192_CFG: c_uint = 0x0a96;
pub const WCD934X_CDC_TX7_TX_PATH_CTL: c_uint = 0x0aa1;
pub const WCD934X_CDC_TX7_TX_PATH_CFG0: c_uint = 0x0aa2;
pub const WCD934X_CDC_TX7_TX_PATH_CFG1: c_uint = 0x0aa3;
pub const WCD934X_CDC_TX7_TX_VOL_CTL: c_uint = 0x0aa4;
pub const WCD934X_CDC_TX7_TX_PATH_192_CTL: c_uint = 0x0aa5;
pub const WCD934X_CDC_TX7_TX_PATH_192_CFG: c_uint = 0x0aa6;
pub const WCD934X_CDC_TX8_TX_PATH_CTL: c_uint = 0x0ab1;
pub const WCD934X_CDC_TX8_TX_PATH_CFG0: c_uint = 0x0ab2;
pub const WCD934X_CDC_TX8_TX_PATH_CFG1: c_uint = 0x0ab3;
pub const WCD934X_CDC_TX8_TX_VOL_CTL: c_uint = 0x0ab4;
pub const WCD934X_CDC_TX8_TX_PATH_192_CTL: c_uint = 0x0ab5;
pub const WCD934X_CDC_TX8_TX_PATH_192_CFG: c_uint = 0x0ab6;
pub const WCD934X_CDC_TX9_SPKR_PROT_PATH_CFG0: c_uint = 0x0ac3;
pub const WCD934X_CDC_TX10_SPKR_PROT_PATH_CFG0: c_uint = 0x0ac7;
pub const WCD934X_CDC_TX11_SPKR_PROT_PATH_CFG0: c_uint = 0x0acb;
pub const WCD934X_CDC_TX12_SPKR_PROT_PATH_CFG0: c_uint = 0x0acf;
pub const WCD934X_CDC_COMPANDER1_CTL0: c_uint = 0x0b01;

pub const WCD934X_COMP_SOFT_RST_DISABLE: c_int = 0;
pub const WCD934X_CDC_COMPANDER1_CTL7: c_uint = 0x0b08;

pub const WCD934X_CDC_COMPANDER2_CTL7: c_uint = 0x0b10;
pub const WCD934X_CDC_COMPANDER7_CTL3: c_uint = 0x0b34;
pub const WCD934X_CDC_COMPANDER7_CTL7: c_uint = 0x0b38;
pub const WCD934X_CDC_COMPANDER8_CTL3: c_uint = 0x0b3c;
pub const WCD934X_CDC_COMPANDER8_CTL7: c_uint = 0x0b40;
pub const WCD934X_CDC_RX0_RX_PATH_CTL: c_uint = 0x0b41;

pub const WCD934X_CDC_RX_PGA_MUTE_DISABLE: c_int = 0;

pub const WCD934X_RX_RESET_DISABLE: c_int = 0;

pub const WCD934X_RX_PCM_RATE_F_48K: c_uint = 0x04;

pub const WCD934X_CDC_RX0_RX_PATH_CFG0: c_uint = 0x0b42;

pub const WCD934X_RX_DLY_ZN_DISABLE: c_int = 0;
pub const WCD934X_CDC_RX0_RX_PATH_CFG1: c_uint = 0x0b43;
pub const WCD934X_CDC_RX0_RX_PATH_CFG2: c_uint = 0x0b44;
pub const WCD934X_CDC_RX0_RX_VOL_CTL: c_uint = 0x0b45;
pub const WCD934X_CDC_RX0_RX_PATH_MIX_CTL: c_uint = 0x0b46;

pub const WCD934X_CDC_RX0_RX_PATH_MIX_CFG: c_uint = 0x0b47;
pub const WCD934X_CDC_RX0_RX_VOL_MIX_CTL: c_uint = 0x0b48;
pub const WCD934X_CDC_RX0_RX_PATH_SEC0: c_uint = 0x0b49;
pub const WCD934X_CDC_RX0_RX_PATH_DSMDEM_CTL: c_uint = 0x0b53;
pub const WCD934X_CDC_RX1_RX_PATH_CTL: c_uint = 0x0b55;

pub const WCD934X_CDC_RX_PATH_PGA_MUTE_DISABLE: c_int = 0;

pub const WCD934X_CDC_RX_PATH_CLK_DISABLE: c_int = 0;
pub const WCD934X_CDC_RX1_RX_PATH_CFG0: c_uint = 0x0b56;

pub const WCD934X_HPH_CMP_DISABLE: c_int = 0;
pub const WCD934X_CDC_RX1_RX_PATH_CFG2: c_uint = 0x0b58;
pub const WCD934X_CDC_RX1_RX_VOL_CTL: c_uint = 0x0b59;
pub const WCD934X_CDC_RX1_RX_PATH_MIX_CTL: c_uint = 0x0b5a;
pub const WCD934X_CDC_RX1_RX_PATH_MIX_CFG: c_uint = 0x0b5b;
pub const WCD934X_CDC_RX1_RX_VOL_MIX_CTL: c_uint = 0x0b5c;
pub const WCD934X_CDC_RX1_RX_PATH_SEC0: c_uint = 0x0b5d;
pub const WCD934X_CDC_RX1_RX_PATH_SEC3: c_uint = 0x0b60;

pub const WCD934X_CDC_RX_PATH_SEC_HD2_ALPHA_0P3125: c_uint = 0x14;
pub const WCD934X_CDC_RX_PATH_SEC_HD2_ALPHA_0P0000: c_int = 0;
pub const WCD934X_CDC_RX1_RX_PATH_DSMDEM_CTL: c_uint = 0x0b67;
pub const WCD934X_CDC_RX2_RX_PATH_CTL: c_uint = 0x0b69;
pub const WCD934X_CDC_RX2_RX_PATH_CFG0: c_uint = 0x0b6a;

pub const WCD934X_CDC_RX_PATH_CFG_HD2_DISABLE: c_int = 0;
pub const WCD934X_CDC_RX2_RX_PATH_CFG2: c_uint = 0x0b6c;
pub const WCD934X_CDC_RX2_RX_VOL_CTL: c_uint = 0x0b6d;
pub const WCD934X_CDC_RX2_RX_PATH_MIX_CTL: c_uint = 0x0b6e;
pub const WCD934X_CDC_RX2_RX_PATH_MIX_CFG: c_uint = 0x0b6f;
pub const WCD934X_CDC_RX2_RX_VOL_MIX_CTL: c_uint = 0x0b70;
pub const WCD934X_CDC_RX2_RX_PATH_SEC0: c_uint = 0x0b71;
pub const WCD934X_CDC_RX2_RX_PATH_SEC3: c_uint = 0x0b74;
pub const WCD934X_CDC_RX2_RX_PATH_DSMDEM_CTL: c_uint = 0x0b7b;
pub const WCD934X_CDC_RX3_RX_PATH_CTL: c_uint = 0x0b7d;
pub const WCD934X_CDC_RX3_RX_PATH_CFG0: c_uint = 0x0b6e;
pub const WCD934X_CDC_RX3_RX_PATH_CFG2: c_uint = 0x0b80;
pub const WCD934X_CDC_RX3_RX_VOL_CTL: c_uint = 0x0b81;
pub const WCD934X_CDC_RX3_RX_PATH_MIX_CTL: c_uint = 0x0b82;
pub const WCD934X_CDC_RX3_RX_PATH_MIX_CFG: c_uint = 0x0b83;
pub const WCD934X_CDC_RX3_RX_VOL_MIX_CTL: c_uint = 0x0b84;
pub const WCD934X_CDC_RX3_RX_PATH_SEC0: c_uint = 0x0b85;
pub const WCD934X_CDC_RX3_RX_PATH_DSMDEM_CTL: c_uint = 0x0b8f;
pub const WCD934X_CDC_RX4_RX_PATH_CTL: c_uint = 0x0b91;
pub const WCD934X_CDC_RX4_RX_PATH_CFG0: c_uint = 0x0b92;
pub const WCD934X_CDC_RX4_RX_PATH_CFG2: c_uint = 0x0b94;
pub const WCD934X_CDC_RX4_RX_VOL_CTL: c_uint = 0x0b95;
pub const WCD934X_CDC_RX4_RX_PATH_MIX_CTL: c_uint = 0x0b96;
pub const WCD934X_CDC_RX4_RX_PATH_MIX_CFG: c_uint = 0x0b97;
pub const WCD934X_CDC_RX4_RX_VOL_MIX_CTL: c_uint = 0x0b98;
pub const WCD934X_CDC_RX4_RX_PATH_SEC0: c_uint = 0x0b99;
pub const WCD934X_CDC_RX4_RX_PATH_DSMDEM_CTL: c_uint = 0x0ba3;
pub const WCD934X_CDC_RX7_RX_PATH_CTL: c_uint = 0x0bcd;
pub const WCD934X_CDC_RX7_RX_PATH_CFG0: c_uint = 0x0bce;
pub const WCD934X_CDC_RX7_RX_PATH_CFG1: c_uint = 0x0bcf;
pub const WCD934X_CDC_RX7_RX_PATH_CFG2: c_uint = 0x0bd0;
pub const WCD934X_CDC_RX7_RX_VOL_CTL: c_uint = 0x0bd1;
pub const WCD934X_CDC_RX7_RX_PATH_MIX_CTL: c_uint = 0x0bd2;
pub const WCD934X_CDC_RX7_RX_PATH_MIX_CFG: c_uint = 0x0bd3;
pub const WCD934X_CDC_RX7_RX_VOL_MIX_CTL: c_uint = 0x0bd4;
pub const WCD934X_CDC_RX7_RX_PATH_SEC1: c_uint = 0x0bd6;
pub const WCD934X_CDC_RX7_RX_PATH_MIX_SEC0: c_uint = 0x0bdd;
pub const WCD934X_CDC_RX7_RX_PATH_DSMDEM_CTL: c_uint = 0x0bdf;
pub const WCD934X_CDC_RX8_RX_PATH_CTL: c_uint = 0x0be1;
pub const WCD934X_CDC_RX8_RX_PATH_CFG0: c_uint = 0x0be2;
pub const WCD934X_CDC_RX8_RX_PATH_CFG1: c_uint = 0x0be3;

pub const WCD934X_RX_SMART_BOOST_DISABLE: c_int = 0;
pub const WCD934X_CDC_RX8_RX_PATH_CFG2: c_uint = 0x0be4;
pub const WCD934X_CDC_RX8_RX_VOL_CTL: c_uint = 0x0be5;
pub const WCD934X_CDC_RX8_RX_PATH_MIX_CTL: c_uint = 0x0be6;
pub const WCD934X_CDC_RX8_RX_PATH_MIX_CFG: c_uint = 0x0be7;
pub const WCD934X_CDC_RX8_RX_VOL_MIX_CTL: c_uint = 0x0be8;
pub const WCD934X_CDC_RX8_RX_PATH_SEC1: c_uint = 0x0bea;
pub const WCD934X_CDC_RX8_RX_PATH_MIX_SEC0: c_uint = 0x0bf1;
pub const WCD934X_CDC_RX8_RX_PATH_DSMDEM_CTL: c_uint = 0x0bf3;
pub const WCD934X_CDC_CLSH_DECAY_CTRL: c_uint = 0x0c03;
pub const WCD934X_CDC_CLSH_K2_MSB: c_uint = 0x0c0a;
pub const WCD934X_CDC_CLSH_K2_LSB: c_uint = 0x0c0b;
pub const WCD934X_CDC_CLSH_TEST0: c_uint = 0x0c0f;
pub const WCD934X_CDC_BOOST0_BOOST_PATH_CTL: c_uint = 0x0c19;

pub const WCD934X_BOOST_PATH_CLK_DISABLE: c_int = 0;
pub const WCD934X_CDC_BOOST0_BOOST_CTL: c_uint = 0x0c1a;
pub const WCD934X_CDC_BOOST0_BOOST_CFG1: c_uint = 0x0c1b;
pub const WCD934X_CDC_BOOST0_BOOST_CFG2: c_uint = 0x0c1c;
pub const WCD934X_CDC_BOOST1_BOOST_PATH_CTL: c_uint = 0x0c21;
pub const WCD934X_CDC_BOOST1_BOOST_CTL: c_uint = 0x0c22;
pub const WCD934X_CDC_BOOST1_BOOST_CFG1: c_uint = 0x0c23;
pub const WCD934X_CDC_BOOST1_BOOST_CFG2: c_uint = 0x0c24;
pub const WCD934X_SWR_AHB_BRIDGE_RD_DATA_0: c_uint = 0x0c91;
pub const WCD934X_SWR_AHB_BRIDGE_RD_DATA_1: c_uint = 0x0c92;
pub const WCD934X_SWR_AHB_BRIDGE_RD_DATA_2: c_uint = 0x0c93;
pub const WCD934X_SWR_AHB_BRIDGE_RD_DATA_3: c_uint = 0x0c94;
pub const WCD934X_SWR_AHB_BRIDGE_ACCESS_STATUS: c_uint = 0x0c96;
pub const WCD934X_CDC_SIDETONE_SRC0_ST_SRC_PATH_CTL: c_uint = 0x0cb5;
pub const WCD934X_CDC_SIDETONE_SRC1_ST_SRC_PATH_CTL: c_uint = 0x0cb9;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT0_CFG0: c_uint = 0x0d01;

pub const WCD934X_CDC_RX_INP_MUX_RX_INT0_CFG1: c_uint = 0x0d02;

pub const WCD934X_CDC_RX_INP_MUX_RX_INT1_CFG0: c_uint = 0x0d03;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT1_CFG1: c_uint = 0x0d04;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT2_CFG0: c_uint = 0x0d05;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT2_CFG1: c_uint = 0x0d06;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT3_CFG0: c_uint = 0x0d07;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT3_CFG1: c_uint = 0x0d08;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT4_CFG0: c_uint = 0x0d09;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT4_CFG1: c_uint = 0x0d0a;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT7_CFG0: c_uint = 0x0d0f;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT7_CFG1: c_uint = 0x0d10;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT8_CFG0: c_uint = 0x0d11;
pub const WCD934X_CDC_RX_INP_MUX_RX_INT8_CFG1: c_uint = 0x0d12;
pub const WCD934X_CDC_RX_INP_MUX_RX_MIX_CFG0: c_uint = 0x0d13;
pub const WCD934X_CDC_RX_INP_MUX_RX_MIX_CFG1: c_uint = 0x0d14;
pub const WCD934X_CDC_RX_INP_MUX_RX_MIX_CFG2: c_uint = 0x0d15;
pub const WCD934X_CDC_RX_INP_MUX_RX_MIX_CFG3: c_uint = 0x0d16;
pub const WCD934X_CDC_RX_INP_MUX_RX_MIX_CFG4: c_uint = 0x0d17;
pub const WCD934X_CDC_RX_INP_MUX_SIDETONE_SRC_CFG0: c_uint = 0x0d18;
pub const WCD934X_CDC_RX_INP_MUX_SIDETONE_SRC_CFG1: c_uint = 0x0d19;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX0_CFG0: c_uint = 0x0d1d;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX0_CFG1: c_uint = 0x0d1e;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX1_CFG0: c_uint = 0x0d1f;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX1_CFG1: c_uint = 0x0d20;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX2_CFG0: c_uint = 0x0d21;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX2_CFG1: c_uint = 0x0d22;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX3_CFG0: c_uint = 0x0d23;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX3_CFG1: c_uint = 0x0d25;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX4_CFG0: c_uint = 0x0d26;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX5_CFG0: c_uint = 0x0d27;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX6_CFG0: c_uint = 0x0d28;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX7_CFG0: c_uint = 0x0d29;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX8_CFG0: c_uint = 0x0d2a;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX10_CFG0: c_uint = 0x0d2b;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX11_CFG0: c_uint = 0x0d2c;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX12_CFG0: c_uint = 0x0d2d;
pub const WCD934X_CDC_TX_INP_MUX_ADC_MUX13_CFG0: c_uint = 0x0d2e;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR0_MIX_CFG0: c_uint = 0x0d31;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR0_MIX_CFG1: c_uint = 0x0d32;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR0_MIX_CFG2: c_uint = 0x0d33;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR0_MIX_CFG3: c_uint = 0x0d34;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR1_MIX_CFG0: c_uint = 0x0d35;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR1_MIX_CFG1: c_uint = 0x0d36;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR1_MIX_CFG2: c_uint = 0x0d37;
pub const WCD934X_CDC_SIDETONE_IIR_INP_MUX_IIR1_MIX_CFG3: c_uint = 0x0d38;
pub const WCD934X_CDC_IF_ROUTER_TX_MUX_CFG0: c_uint = 0x0d3a;
pub const WCD934X_CDC_IF_ROUTER_TX_MUX_CFG1: c_uint = 0x0d3b;
pub const WCD934X_CDC_IF_ROUTER_TX_MUX_CFG2: c_uint = 0x0d3c;
pub const WCD934X_CDC_IF_ROUTER_TX_MUX_CFG3: c_uint = 0x0d3d;
pub const WCD934X_CDC_CLK_RST_CTRL_MCLK_CONTROL: c_uint = 0x0d41;

pub const WCD934X_CDC_CLK_RST_CTRL_FS_CNT_CONTROL: c_uint = 0x0d42;

pub const WCD934X_CDC_CLK_RST_CTRL_SWR_CONTROL: c_uint = 0x0d43;

pub const WCD934X_CDC_CLK_RST_CTRL_DSD_CONTROL: c_uint = 0x0d44;
pub const WCD934X_CDC_CLK_RST_CTRL_ASRC_SHARE_CONTROL: c_uint = 0x0d45;
pub const WCD934X_CDC_CLK_RST_CTRL_GFM_CONTROL: c_uint = 0x0d46;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_PATH_CTL: c_uint = 0x0d55;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B1_CTL: c_uint = 0x0d56;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B2_CTL: c_uint = 0x0d57;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B3_CTL: c_uint = 0x0d58;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B4_CTL: c_uint = 0x0d59;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B5_CTL: c_uint = 0x0d5a;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B6_CTL: c_uint = 0x0d5b;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B7_CTL: c_uint = 0x0d5c;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_B8_CTL: c_uint = 0x0d5d;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_CTL: c_uint = 0x0d5e;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_GAIN_TIMER_CTL: c_uint = 0x0d5f;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_COEF_B1_CTL: c_uint = 0x0d60;
pub const WCD934X_CDC_SIDETONE_IIR0_IIR_COEF_B2_CTL: c_uint = 0x0d61;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_PATH_CTL: c_uint = 0x0d65;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B1_CTL: c_uint = 0x0d66;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B2_CTL: c_uint = 0x0d67;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B3_CTL: c_uint = 0x0d68;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B4_CTL: c_uint = 0x0d69;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B5_CTL: c_uint = 0x0d6a;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B6_CTL: c_uint = 0x0d6b;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B7_CTL: c_uint = 0x0d6c;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_B8_CTL: c_uint = 0x0d6d;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_CTL: c_uint = 0x0d6e;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_GAIN_TIMER_CTL: c_uint = 0x0d6f;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_COEF_B1_CTL: c_uint = 0x0d70;
pub const WCD934X_CDC_SIDETONE_IIR1_IIR_COEF_B2_CTL: c_uint = 0x0d71;
pub const WCD934X_CDC_TOP_TOP_CFG1: c_uint = 0x0d82;
pub const WCD934X_CDC_TOP_TOP_CFG7: c_uint = 0x0d88;
pub const WCD934X_CDC_TOP_HPHL_COMP_LUT: c_uint = 0x0d8b;
pub const WCD934X_CDC_TOP_HPHR_COMP_LUT: c_uint = 0x0d90;

pub const WCD934X_HPH_LUT_BYPASS_DISABLE: c_int = 0;
pub const WCD934X_CODEC_CPR_WR_DATA_0: c_uint = 0x5001;
pub const WCD934X_CODEC_CPR_WR_ADDR_0: c_uint = 0x5005;
pub const WCD934X_CODEC_CPR_SVS_CX_VDD: c_uint = 0x5022;
pub const WCD934X_CODEC_CPR_SVS2_CX_VDD: c_uint = 0x5023;
pub const WCD934X_CODEC_CPR_SVS2_MIN_CX_VDD: c_uint = 0x5027;
pub const WCD934X_TLMM_DMIC1_CLK_PINCFG: c_uint = 0x8015;
pub const WCD934X_TLMM_DMIC1_DATA_PINCFG: c_uint = 0x8016;
pub const WCD934X_TLMM_DMIC2_CLK_PINCFG: c_uint = 0x8017;
pub const WCD934X_TLMM_DMIC2_DATA_PINCFG: c_uint = 0x8018;
pub const WCD934X_TLMM_DMIC3_CLK_PINCFG: c_uint = 0x8019;
pub const WCD934X_TLMM_DMIC3_DATA_PINCFG: c_uint = 0x801a;
pub const WCD934X_TEST_DEBUG_PAD_DRVCTL_0: c_uint = 0x803b;
pub const WCD934X_TEST_DEBUG_NPL_DLY_TEST_1: c_uint = 0x803e;
pub const WCD934X_MAX_REGISTER: c_uint = 0xffff;
pub const WCD934X_SEL_REGISTER: c_uint = 0x800;
pub const WCD934X_SEL_MASK: c_uint = 0xff;
pub const WCD934X_SEL_SHIFT: c_uint = 0x0;
pub const WCD934X_WINDOW_START: c_uint = 0x800;
pub const WCD934X_WINDOW_LENGTH: c_uint = 0x100;
// SLIMBUS Slave Registers
pub const WCD934X_SLIM_PGD_PORT_INT_EN0: c_uint = 0x30;
pub const WCD934X_SLIM_PGD_PORT_INT_STATUS_RX_0: c_uint = 0x34;
pub const WCD934X_SLIM_PGD_PORT_INT_STATUS_RX_1: c_uint = 0x35;
pub const WCD934X_SLIM_PGD_PORT_INT_STATUS_TX_0: c_uint = 0x36;
pub const WCD934X_SLIM_PGD_PORT_INT_STATUS_TX_1: c_uint = 0x37;
pub const WCD934X_SLIM_PGD_PORT_INT_CLR_RX_0: c_uint = 0x38;
pub const WCD934X_SLIM_PGD_PORT_INT_CLR_RX_1: c_uint = 0x39;
pub const WCD934X_SLIM_PGD_PORT_INT_CLR_TX_0: c_uint = 0x3A;
pub const WCD934X_SLIM_PGD_PORT_INT_CLR_TX_1: c_uint = 0x3B;
pub const WCD934X_SLIM_PGD_PORT_INT_RX_SOURCE0: c_uint = 0x60;
pub const WCD934X_SLIM_PGD_PORT_INT_TX_SOURCE0: c_uint = 0x70;

// ports range from 10-16

pub const SLIM_MANF_ID_QCOM: c_uint = 0x217;
pub const SLIM_PROD_CODE_WCD9340: c_uint = 0x250;
pub const SLIM_DEV_IDX_WCD9340: c_uint = 0x1;
pub const SLIM_DEV_INSTANCE_ID_WCD9340: c_int = 0;
