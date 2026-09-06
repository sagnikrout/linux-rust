//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/wcd939x-usbss.c
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
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (C) 2023 Linaro Ltd.
//

pub const WCD_USBSS_PMP_OUT1: c_uint = 0x2;
pub const WCD_USBSS_DP_DN_MISC1: c_uint = 0x20;

pub const WCD_USBSS_MG1_EN: c_uint = 0x24;

pub const WCD_USBSS_MG1_BIAS: c_uint = 0x25;

pub const WCD_USBSS_MG1_MISC: c_uint = 0x27;

pub const WCD_USBSS_MG2_EN: c_uint = 0x28;

pub const WCD_USBSS_MG2_BIAS: c_uint = 0x29;

pub const WCD_USBSS_MG2_MISC: c_uint = 0x30;

pub const WCD_USBSS_DISP_AUXP_THRESH: c_uint = 0x80;

pub const WCD_USBSS_DISP_AUXP_CTL: c_uint = 0x81;

pub const WCD_USBSS_CPLDO_CTL2: c_uint = 0xa1;
pub const WCD_USBSS_SWITCH_SETTINGS_ENABLE: c_uint = 0x403;

pub const WCD_USBSS_SWITCH_SELECT0: c_uint = 0x404;

pub const WCD_USBSS_SWITCH_SELECT0_DNL_SWITCH_L: c_int = 0;
pub const WCD_USBSS_SWITCH_SELECT0_DNL_SWITCH_DN: c_int = 1;
pub const WCD_USBSS_SWITCH_SELECT0_DNL_SWITCH_DN2: c_int = 2;
pub const WCD_USBSS_SWITCH_SELECT0_DPR_SWITCH_R: c_int = 0;
pub const WCD_USBSS_SWITCH_SELECT0_DPR_SWITCH_DP: c_int = 1;
pub const WCD_USBSS_SWITCH_SELECT0_DPR_SWITCH_DR2: c_int = 2;
pub const WCD_USBSS_SWITCH_SELECT1: c_uint = 0x405;

pub const WCD_USBSS_DELAY_R_SW: c_uint = 0x40d;
pub const WCD_USBSS_DELAY_MIC_SW: c_uint = 0x40e;
pub const WCD_USBSS_DELAY_SENSE_SW: c_uint = 0x40f;
pub const WCD_USBSS_DELAY_GND_SW: c_uint = 0x410;
pub const WCD_USBSS_DELAY_L_SW: c_uint = 0x411;
pub const WCD_USBSS_FUNCTION_ENABLE: c_uint = 0x413;

pub const WCD_USBSS_FUNCTION_ENABLE_SOURCE_SELECT_MANUAL: c_int = 1;
pub const WCD_USBSS_FUNCTION_ENABLE_SOURCE_SELECT_AUDIO_FSM: c_int = 2;
pub const WCD_USBSS_EQUALIZER1: c_uint = 0x415;

pub const WCD_USBSS_USB_SS_CNTL: c_uint = 0x419;

pub const WCD_USBSS_USB_SS_CNTL_USB_SS_MODE_AATC: c_int = 2;
pub const WCD_USBSS_USB_SS_CNTL_USB_SS_MODE_USB: c_int = 5;
pub const WCD_USBSS_AUDIO_FSM_START: c_uint = 0x433;

pub const WCD_USBSS_RATIO_SPKR_REXT_L_LSB: c_uint = 0x461;
pub const WCD_USBSS_RATIO_SPKR_REXT_L_MSB: c_uint = 0x462;
pub const WCD_USBSS_RATIO_SPKR_REXT_R_LSB: c_uint = 0x463;
pub const WCD_USBSS_RATIO_SPKR_REXT_R_MSB: c_uint = 0x464;
pub const WCD_USBSS_AUD_COEF_L_K0_0: c_uint = 0x475;
pub const WCD_USBSS_AUD_COEF_L_K0_1: c_uint = 0x476;
pub const WCD_USBSS_AUD_COEF_L_K0_2: c_uint = 0x477;
pub const WCD_USBSS_AUD_COEF_L_K1_0: c_uint = 0x478;
pub const WCD_USBSS_AUD_COEF_L_K1_1: c_uint = 0x479;
pub const WCD_USBSS_AUD_COEF_L_K2_0: c_uint = 0x47a;
pub const WCD_USBSS_AUD_COEF_L_K2_1: c_uint = 0x47b;
pub const WCD_USBSS_AUD_COEF_L_K3_0: c_uint = 0x47c;
pub const WCD_USBSS_AUD_COEF_L_K3_1: c_uint = 0x47d;
pub const WCD_USBSS_AUD_COEF_L_K4_0: c_uint = 0x47e;
pub const WCD_USBSS_AUD_COEF_L_K4_1: c_uint = 0x47f;
pub const WCD_USBSS_AUD_COEF_L_K5_0: c_uint = 0x480;
pub const WCD_USBSS_AUD_COEF_L_K5_1: c_uint = 0x481;
pub const WCD_USBSS_AUD_COEF_R_K0_0: c_uint = 0x482;
pub const WCD_USBSS_AUD_COEF_R_K0_1: c_uint = 0x483;
pub const WCD_USBSS_AUD_COEF_R_K0_2: c_uint = 0x484;
pub const WCD_USBSS_AUD_COEF_R_K1_0: c_uint = 0x485;
pub const WCD_USBSS_AUD_COEF_R_K1_1: c_uint = 0x486;
pub const WCD_USBSS_AUD_COEF_R_K2_0: c_uint = 0x487;
pub const WCD_USBSS_AUD_COEF_R_K2_1: c_uint = 0x488;
pub const WCD_USBSS_AUD_COEF_R_K3_0: c_uint = 0x489;
pub const WCD_USBSS_AUD_COEF_R_K3_1: c_uint = 0x48a;
pub const WCD_USBSS_AUD_COEF_R_K4_0: c_uint = 0x48b;
pub const WCD_USBSS_AUD_COEF_R_K4_1: c_uint = 0x48c;
pub const WCD_USBSS_AUD_COEF_R_K5_0: c_uint = 0x48d;
pub const WCD_USBSS_AUD_COEF_R_K5_1: c_uint = 0x48e;
pub const WCD_USBSS_GND_COEF_L_K0_0: c_uint = 0x48f;
pub const WCD_USBSS_GND_COEF_L_K0_1: c_uint = 0x490;
pub const WCD_USBSS_GND_COEF_L_K0_2: c_uint = 0x491;
pub const WCD_USBSS_GND_COEF_L_K1_0: c_uint = 0x492;
pub const WCD_USBSS_GND_COEF_L_K1_1: c_uint = 0x493;
pub const WCD_USBSS_GND_COEF_L_K2_0: c_uint = 0x494;
pub const WCD_USBSS_GND_COEF_L_K2_1: c_uint = 0x495;
pub const WCD_USBSS_GND_COEF_L_K3_0: c_uint = 0x496;
pub const WCD_USBSS_GND_COEF_L_K3_1: c_uint = 0x497;
pub const WCD_USBSS_GND_COEF_L_K4_0: c_uint = 0x498;
pub const WCD_USBSS_GND_COEF_L_K4_1: c_uint = 0x499;
pub const WCD_USBSS_GND_COEF_L_K5_0: c_uint = 0x49a;
pub const WCD_USBSS_GND_COEF_L_K5_1: c_uint = 0x49b;
pub const WCD_USBSS_GND_COEF_R_K0_0: c_uint = 0x49c;
pub const WCD_USBSS_GND_COEF_R_K0_1: c_uint = 0x49d;
pub const WCD_USBSS_GND_COEF_R_K0_2: c_uint = 0x49e;
pub const WCD_USBSS_GND_COEF_R_K1_0: c_uint = 0x49f;
pub const WCD_USBSS_GND_COEF_R_K1_1: c_uint = 0x4a0;
pub const WCD_USBSS_GND_COEF_R_K2_0: c_uint = 0x4a1;
pub const WCD_USBSS_GND_COEF_R_K2_1: c_uint = 0x4a2;
pub const WCD_USBSS_GND_COEF_R_K3_0: c_uint = 0x4a3;
pub const WCD_USBSS_GND_COEF_R_K3_1: c_uint = 0x4a4;
pub const WCD_USBSS_GND_COEF_R_K4_0: c_uint = 0x4a5;
pub const WCD_USBSS_GND_COEF_R_K4_1: c_uint = 0x4a6;
pub const WCD_USBSS_GND_COEF_R_K5_0: c_uint = 0x4a7;
pub const WCD_USBSS_GND_COEF_R_K5_1: c_uint = 0x4a8;
pub const WCD_USBSS_MAX_REGISTER: c_uint = 0x4c1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd939x_usbss {
    pub client: *mut i2c_client,
    pub reset_gpio: *mut gpio_desc,
    pub vdd_supply: *mut regulator,
// used to serialize concurrent change requests
    pub lock: mutex,
    pub sw: *mut typec_switch_dev,
    pub mux: *mut typec_mux_dev,
    pub regmap: *mut regmap,
    pub codec: *mut typec_mux,
    pub codec_switch: *mut typec_switch,
    pub orientation: enum typec_orientation,
    pub mode: c_ulong,
    pub svid: c_uint,
}

    static const struct regmap_range_cfg wcd939x_usbss_ranges[] = {
    {
    .range_min = 0,
    .range_max = WCD_USBSS_MAX_REGISTER,
    .selector_reg = 0x0,
    .selector_mask = 0xff,
    .selector_shift = 0,
    .window_start = 0,
    .window_len = 0x100,
    },
    };
    static const struct regmap_config wcd939x_usbss_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = WCD_USBSS_MAX_REGISTER,
    .ranges = wcd939x_usbss_ranges,
    .num_ranges = ARRAY_SIZE(wcd939x_usbss_ranges),
    };
// Linearlizer coefficients for 32ohm load
    static const struct {
    unsigned int offset;
    unsigned int mask;
    unsigned int value;
    } wcd939x_usbss_coeff_init[] = {
    { WCD_USBSS_AUD_COEF_L_K5_0, GENMASK(7, 0), 0x39 },
    { WCD_USBSS_AUD_COEF_R_K5_0, GENMASK(7, 0), 0x39 },
    { WCD_USBSS_GND_COEF_L_K2_0, GENMASK(7, 0), 0xe8 },
    { WCD_USBSS_GND_COEF_L_K4_0, GENMASK(7, 0), 0x73 },
    { WCD_USBSS_GND_COEF_R_K2_0, GENMASK(7, 0), 0xe8 },
    { WCD_USBSS_GND_COEF_R_K4_0, GENMASK(7, 0), 0x73 },
    { WCD_USBSS_RATIO_SPKR_REXT_L_LSB, GENMASK(7, 0), 0x00 },
    { WCD_USBSS_RATIO_SPKR_REXT_L_MSB, GENMASK(6, 0), 0x04 },
    { WCD_USBSS_RATIO_SPKR_REXT_R_LSB, GENMASK(7, 0), 0x00 },
    { WCD_USBSS_RATIO_SPKR_REXT_R_MSB, GENMASK(6, 0), 0x04 },
    };
#[no_mangle]
unsafe extern "C" fn wcd939x_usbss_set(usbss: *mut wcd939x_usbss) -> c_int {
    static int wcd939x_usbss_set(struct wcd939x_usbss *usbss)
    {
    let mut reverse: bool = (usbss.orientation == TYPEC_ORIENTATION_REVERSE);
    let mut enable_audio: bool = false;
    let mut enable_usb: bool = false;
    let mut enable_dp: bool = false;
    int ret;
// USB Mode
    if (usbss.mode < TYPEC_STATE_MODAL ||
    (!usbss.svid && (usbss.mode == TYPEC_MODE_USB2 ||
    usbss.mode == TYPEC_MODE_USB3))) {
    enable_usb = true;
    } else if (usbss.svid) {
    switch (usbss.mode) {
// DP Only
    case TYPEC_DP_STATE_C:
    case TYPEC_DP_STATE_E:
    enable_dp = true;
    break;
// DP + USB
    case TYPEC_DP_STATE_D:
    case TYPEC_DP_STATE_F:
    enable_usb = true;
    enable_dp = true;
    break;
    default:
    return -EOPNOTSUPP;
    }
    } else if (usbss.mode == TYPEC_MODE_AUDIO) {
    enable_audio = true;
    } else {
    return -EOPNOTSUPP;
    }
// Disable all switches
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DP_AUXP_TO_MGX_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DP_AUXM_TO_MGX_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DPR_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DNL_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_SENSE_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_MIC_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_AGND_SWITCHES);
    if (ret)
    return ret;
// Clear switches
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_DP_AUXP_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DP_AUXM_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DPR_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DNL_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_SENSE_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_MIC_SWITCHES);
    if (ret)
    return ret;
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT1,
    WCD_USBSS_SWITCH_SELECT1_AGND_SWITCHES);
    if (ret)
    return ret;
// Enable OVP_MG1_BIAS PCOMP_DYN_BST_EN
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_MG1_BIAS,
    WCD_USBSS_MG1_BIAS_PCOMP_DYN_BST_EN);
    if (ret)
    return ret;
// Enable OVP_MG2_BIAS PCOMP_DYN_BST_EN
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_MG2_BIAS,
    WCD_USBSS_MG2_BIAS_PCOMP_DYN_BST_EN);
    if (ret)
    return ret;
// Disable Equalizer in safe mode
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_EQUALIZER1,
    WCD_USBSS_EQUALIZER1_EQ_EN);
    if (ret)
    return ret;
// Start FSM with all disabled, force write
    ret = regmap_write_bits(usbss.regmap, WCD_USBSS_AUDIO_FSM_START,
    WCD_USBSS_AUDIO_FSM_START_AUDIO_FSM_AUDIO_TRIG,
    WCD_USBSS_AUDIO_FSM_START_AUDIO_FSM_AUDIO_TRIG);
// 35us to allow the SBU switch to turn off
    usleep_range(35, 1000);
// Setup Audio Accessory mux/switch
    if (enable_audio) {
    int i;
//
// AATC switch configuration:
// "Normal":
// - R: DNR
// - L: DNL
// - Sense: GSBU2
// - Mic: MG1
// - AGND: MG2
// "Swapped":
// - R: DNR
// - L: DNL
// - Sense: GSBU1
// - Mic: MG2
// - AGND: MG1
// Swapped information is given by the codec MBHC logic
//
// Set AATC mode
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_USB_SS_CNTL,
    WCD_USBSS_USB_SS_CNTL_USB_SS_MODE,
    FIELD_PREP(WCD_USBSS_USB_SS_CNTL_USB_SS_MODE,
    WCD_USBSS_USB_SS_CNTL_USB_SS_MODE_AATC));
    if (ret)
    return ret;
// Select L for DNL_SWITCHES and R for DPR_SWITCHES
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_DPR_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DNL_SWITCHES,
    FIELD_PREP(WCD_USBSS_SWITCH_SELECT0_DNL_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_DNL_SWITCH_L) |
    FIELD_PREP(WCD_USBSS_SWITCH_SELECT0_DPR_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_DPR_SWITCH_R));
    if (ret)
    return ret;
    if (reverse)
// Select MG2 for MIC, SBU1 for Sense
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_MIC_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_MIC_SWITCHES);
    else
// Select MG1 for MIC, SBU2 for Sense
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_SENSE_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_SENSE_SWITCHES);
    if (ret)
    return ret;
    if (reverse)
// Disable OVP_MG1_BIAS PCOMP_DYN_BST_EN
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_MG1_BIAS,
    WCD_USBSS_MG1_BIAS_PCOMP_DYN_BST_EN);
    else
// Disable OVP_MG2_BIAS PCOMP_DYN_BST_EN
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_MG2_BIAS,
    WCD_USBSS_MG2_BIAS_PCOMP_DYN_BST_EN);
    if (ret)
    return ret;
// Enable SENSE, MIC switches
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_SENSE_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_MIC_SWITCHES);
    if (ret)
    return ret;
    if (reverse)
// Select MG1 for AGND_SWITCHES
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT1,
    WCD_USBSS_SWITCH_SELECT1_AGND_SWITCHES);
    else
// Select MG2 for AGND_SWITCHES
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT1,
    WCD_USBSS_SWITCH_SELECT1_AGND_SWITCHES);
    if (ret)
    return ret;
// Enable AGND switches
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_AGND_SWITCHES);
    if (ret)
    return ret;
// Enable DPR, DNL switches
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DNL_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DPR_SWITCHES);
    if (ret)
    return ret;
// Setup FSM delays
    ret = regmap_write(usbss.regmap, WCD_USBSS_DELAY_L_SW, 0x02);
    if (ret)
    return ret;
    ret = regmap_write(usbss.regmap, WCD_USBSS_DELAY_R_SW, 0x02);
    if (ret)
    return ret;
    ret = regmap_write(usbss.regmap, WCD_USBSS_DELAY_MIC_SW, 0x01);
    if (ret)
    return ret;
// Start FSM, force write
    ret = regmap_write_bits(usbss.regmap, WCD_USBSS_AUDIO_FSM_START,
    WCD_USBSS_AUDIO_FSM_START_AUDIO_FSM_AUDIO_TRIG,
    WCD_USBSS_AUDIO_FSM_START_AUDIO_FSM_AUDIO_TRIG);
    if (ret)
    return ret;
// Default Linearlizer coefficients
    for (i = 0; i < ARRAY_SIZE(wcd939x_usbss_coeff_init); ++i)
    regmap_update_bits(usbss.regmap,
    wcd939x_usbss_coeff_init[i].offset,
    wcd939x_usbss_coeff_init[i].mask,
    wcd939x_usbss_coeff_init[i].value);
    return 0;
    }
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_USB_SS_CNTL,
    WCD_USBSS_USB_SS_CNTL_USB_SS_MODE,
    FIELD_PREP(WCD_USBSS_USB_SS_CNTL_USB_SS_MODE,
    WCD_USBSS_USB_SS_CNTL_USB_SS_MODE_USB));
    if (ret)
    return ret;
// Enable USB muxes
    if (enable_usb) {
// Do not enable Equalizer in safe mode
    if (usbss.mode != TYPEC_STATE_SAFE) {
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_EQUALIZER1,
    WCD_USBSS_EQUALIZER1_EQ_EN);
    if (ret)
    return ret;
    }
// Select DN for DNL_SWITCHES and DP for DPR_SWITCHES
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_DPR_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DNL_SWITCHES,
    FIELD_PREP(WCD_USBSS_SWITCH_SELECT0_DNL_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_DNL_SWITCH_DN) |
    FIELD_PREP(WCD_USBSS_SWITCH_SELECT0_DPR_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_DPR_SWITCH_DP));
    if (ret)
    return ret;
// Enable DNL_SWITCHES and DPR_SWITCHES
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DPR_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DNL_SWITCHES);
    if (ret)
    return ret;
    }
// Enable DP AUX muxes
    if (enable_dp) {
// Update Leakage Canceller Coefficient for AUXP pins
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_DISP_AUXP_CTL,
    WCD_USBSS_DISP_AUXP_CTL_LK_CANCEL_TRK_COEFF,
    FIELD_PREP(WCD_USBSS_DISP_AUXP_CTL_LK_CANCEL_TRK_COEFF,
    5));
    if (ret)
    return ret;
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_DISP_AUXP_THRESH,
    WCD_USBSS_DISP_AUXP_THRESH_DISP_AUXP_OVPON_CM);
    if (ret)
    return ret;
    if (reverse)
// Select MG2 for AUXP and MG1 for AUXM
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_DP_AUXP_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DP_AUXM_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_DP_AUXP_SWITCHES);
    else
// Select MG1 for AUXP and MG2 for AUXM
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_SWITCH_SELECT0,
    WCD_USBSS_SWITCH_SELECT0_DP_AUXP_SWITCHES |
    WCD_USBSS_SWITCH_SELECT0_DP_AUXM_SWITCHES,
    WCD_USBSS_SWITCH_SELECT0_DP_AUXM_SWITCHES);
    if (ret)
    return ret;
// Enable DP_AUXP_TO_MGX and DP_AUXM_TO_MGX switches
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DP_AUXP_TO_MGX_SWITCHES |
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DP_AUXM_TO_MGX_SWITCHES);
// 15us to allow the SBU switch to turn on again
    usleep_range(15, 1000);
    }
    return 0;
    }
    static int wcd939x_usbss_switch_set(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
    struct wcd939x_usbss *usbss = typec_switch_get_drvdata(sw);
    let mut ret: c_int = 0;
    mutex_lock(&usbss.lock);
    if (usbss.orientation != orientation) {
    usbss.orientation = orientation;
    ret = wcd939x_usbss_set(usbss);
    }
    mutex_unlock(&usbss.lock);
    if (ret)
    return ret;
// Report orientation to codec after switch has been done
    return typec_switch_set(usbss.codec_switch, orientation);
    }
    static int wcd939x_usbss_mux_set(struct typec_mux_dev *mux,
    struct typec_mux_state *state)
    {
    struct wcd939x_usbss *usbss = typec_mux_get_drvdata(mux);
    let mut ret: c_int = 0;
    mutex_lock(&usbss.lock);
    if (usbss.mode != state.mode) {
    usbss.mode = state.mode;
    if (state.alt)
    usbss.svid = state.alt.svid;
    else
    usbss.svid = 0; // No SVID
    ret = wcd939x_usbss_set(usbss);
    }
    mutex_unlock(&usbss.lock);
    if (ret)
    return ret;
// Report event to codec after switch has been done
    return typec_mux_set(usbss.codec, state);
    }
#[no_mangle]
unsafe extern "C" fn wcd939x_usbss_probe(client: *mut i2c_client) -> c_int {
    static int wcd939x_usbss_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    let mut sw_desc: typec_switch_desc = { };
    let mut mux_desc: typec_mux_desc = { };
    struct wcd939x_usbss *usbss;
    int ret;
    usbss = devm_kzalloc(dev, sizeof(*usbss), GFP_KERNEL);
    if (!usbss)
    return -ENOMEM;
    usbss.client = client;
    mutex_init(&usbss.lock);
    usbss.regmap = devm_regmap_init_i2c(client, &wcd939x_usbss_regmap_config);
    if (IS_ERR(usbss.regmap))
    return dev_err_probe(dev, PTR_ERR(usbss.regmap), "failed to initialize regmap\n");
    usbss.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(usbss.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(usbss.reset_gpio),
    "unable to acquire reset gpio\n");
    usbss.vdd_supply = devm_regulator_get_optional(dev, "vdd");
    if (IS_ERR(usbss.vdd_supply))
    return PTR_ERR(usbss.vdd_supply);
// Get Codec's MUX & Switch devices
    usbss.codec = fwnode_typec_mux_get(dev.fwnode);
    if (IS_ERR(usbss.codec))
    return dev_err_probe(dev, PTR_ERR(usbss.codec),
    "failed to acquire codec mode-switch\n");
    usbss.codec_switch = fwnode_typec_switch_get(dev.fwnode);
    if (IS_ERR(usbss.codec_switch)) {
    ret = dev_err_probe(dev, PTR_ERR(usbss.codec_switch),
    "failed to acquire codec orientation-switch\n");
    goto err_mux_put;
    }
    usbss.mode = TYPEC_STATE_SAFE;
    usbss.orientation = TYPEC_ORIENTATION_NONE;
    gpiod_set_value(usbss.reset_gpio, 1);
    ret = regulator_enable(usbss.vdd_supply);
    if (ret) {
    dev_err(dev, "Failed to enable vdd: %d\n", ret);
    goto err_mux_switch;
    }
    msleep(20);
    gpiod_set_value(usbss.reset_gpio, 0);
    msleep(20);
// Disable standby
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_USB_SS_CNTL,
    WCD_USBSS_USB_SS_CNTL_STANDBY_STATE);
    if (ret)
    goto err_regulator_disable;
// Set manual mode by default
    ret = regmap_update_bits(usbss.regmap, WCD_USBSS_FUNCTION_ENABLE,
    WCD_USBSS_FUNCTION_ENABLE_SOURCE_SELECT,
    FIELD_PREP(WCD_USBSS_FUNCTION_ENABLE_SOURCE_SELECT,
    WCD_USBSS_FUNCTION_ENABLE_SOURCE_SELECT_MANUAL));
    if (ret)
    goto err_regulator_disable;
// Enable dynamic boosting for DP and DN
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_DP_DN_MISC1,
    WCD_USBSS_DP_DN_MISC1_DP_PCOMP_2X_DYN_BST_ON_EN |
    WCD_USBSS_DP_DN_MISC1_DN_PCOMP_2X_DYN_BST_ON_EN);
    if (ret)
    goto err_regulator_disable;
// Enable dynamic boosting for MG1 OVP
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_MG1_MISC,
    WCD_USBSS_MG1_MISC_PCOMP_2X_DYN_BST_ON_EN);
    if (ret)
    goto err_regulator_disable;
// Enable dynamic boosting for MG2 OVP
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_MG2_MISC,
    WCD_USBSS_MG2_MISC_PCOMP_2X_DYN_BST_ON_EN);
    if (ret)
    goto err_regulator_disable;
// Write 0xFF to WCD_USBSS_CPLDO_CTL2
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_CPLDO_CTL2, 0xff);
    if (ret)
    goto err_regulator_disable;
// Set RCO_EN: WCD_USBSS_USB_SS_CNTL Bit<3> --> 0x0 --> 0x1
    ret = regmap_clear_bits(usbss.regmap, WCD_USBSS_USB_SS_CNTL,
    WCD_USBSS_USB_SS_CNTL_RCO_EN);
    if (ret)
    goto err_regulator_disable;
    ret = regmap_set_bits(usbss.regmap, WCD_USBSS_USB_SS_CNTL,
    WCD_USBSS_USB_SS_CNTL_RCO_EN);
    if (ret)
    goto err_regulator_disable;
// Disable all switches but enable the mux
    ret = regmap_write(usbss.regmap, WCD_USBSS_SWITCH_SETTINGS_ENABLE,
    WCD_USBSS_SWITCH_SETTINGS_ENABLE_DEVICE_ENABLE);
    if (ret)
    goto err_regulator_disable;
// Setup in SAFE mode
    ret = wcd939x_usbss_set(usbss);
    if (ret)
    goto err_regulator_disable;
    sw_desc.drvdata = usbss;
    sw_desc.fwnode = dev_fwnode(dev);
    sw_desc.set = wcd939x_usbss_switch_set;
    usbss.sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(usbss.sw)) {
    ret = dev_err_probe(dev, PTR_ERR(usbss.sw), "failed to register typec switch\n");
    goto err_regulator_disable;
    }
    mux_desc.drvdata = usbss;
    mux_desc.fwnode = dev_fwnode(dev);
    mux_desc.set = wcd939x_usbss_mux_set;
    usbss.mux = typec_mux_register(dev, &mux_desc);
    if (IS_ERR(usbss.mux)) {
    ret = dev_err_probe(dev, PTR_ERR(usbss.mux), "failed to register typec mux\n");
    goto err_switch_unregister;
    }
    i2c_set_clientdata(client, usbss);
    return 0;
    err_switch_unregister:
    typec_switch_unregister(usbss.sw);
    err_regulator_disable:
    regulator_disable(usbss.vdd_supply);
    err_mux_switch:
    typec_switch_put(usbss.codec_switch);
    err_mux_put:
    typec_mux_put(usbss.codec);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wcd939x_usbss_remove(client: *mut i2c_client) {
    static void wcd939x_usbss_remove(struct i2c_client *client)
    {
    struct wcd939x_usbss *usbss = i2c_get_clientdata(client);
    typec_mux_unregister(usbss.mux);
    typec_switch_unregister(usbss.sw);
    regulator_disable(usbss.vdd_supply);
    typec_switch_put(usbss.codec_switch);
    typec_mux_put(usbss.codec);
    }
    static const struct i2c_device_id wcd939x_usbss_table[] = {
    { .name = "wcd9390-usbss" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, wcd939x_usbss_table);
    static const struct of_device_id wcd939x_usbss_of_table[] = {
    { .compatible = "qcom,wcd9390-usbss" },
    { }
    };
    MODULE_DEVICE_TABLE(of, wcd939x_usbss_of_table);
    static struct i2c_driver wcd939x_usbss_driver = {
    .driver = {
    .name = "wcd939x-usbss",
    .of_match_table = wcd939x_usbss_of_table,
    },
    .probe		= wcd939x_usbss_probe,
    .remove		= wcd939x_usbss_remove,
    .id_table	= wcd939x_usbss_table,
    };
    module_i2c_driver(wcd939x_usbss_driver);
    MODULE_DESCRIPTION("Qualcomm WCD939x USBSS driver");
    MODULE_LICENSE("GPL");
