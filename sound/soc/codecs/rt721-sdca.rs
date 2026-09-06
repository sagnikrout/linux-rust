//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt721-sdca.h
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
// rt721-sdca.h -- RT721 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt721_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool,
// For Headset jack & Headphone
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub jack_type: c_int,
    pub jd_src: c_int,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
// For DMIC
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt721_sdca_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}

// NID
pub const RT721_ANA_POW_PART: c_uint = 0x01;
pub const RT721_DAC_CTRL: c_uint = 0x04;
pub const RT721_JD_CTRL: c_uint = 0x09;
pub const RT721_CBJ_CTRL: c_uint = 0x0a;
pub const RT721_CAP_PORT_CTRL: c_uint = 0x0c;
pub const RT721_CLASD_AMP_CTRL: c_uint = 0x0d;
pub const RT721_BOOST_CTRL: c_uint = 0x0f;
pub const RT721_VENDOR_REG: c_uint = 0x20;
pub const RT721_RC_CALIB_CTRL: c_uint = 0x40;
pub const RT721_VENDOR_EQ_L: c_uint = 0x53;
pub const RT721_VENDOR_EQ_R: c_uint = 0x54;
pub const RT721_VENDOR_HP_CALI: c_uint = 0x56;
pub const RT721_VENDOR_CHARGE_PUMP: c_uint = 0x57;
pub const RT721_VENDOR_CLASD_CALI: c_uint = 0x58;
pub const RT721_VENDOR_IMS_DRE: c_uint = 0x5b;
pub const RT721_VENDOR_SPK_EFUSE: c_uint = 0x5c;
pub const RT721_VENDOR_LEVEL_CTRL: c_uint = 0x5d;
pub const RT721_VENDOR_ANA_CTL: c_uint = 0x5f;
pub const RT721_HDA_SDCA_FLOAT: c_uint = 0x61;
// Index (NID:01h)
pub const RT721_MBIAS_LV_CTRL2: c_uint = 0x07;
pub const RT721_VREF1_HV_CTRL1: c_uint = 0x0a;
pub const RT721_VREF2_LV_CTRL1: c_uint = 0x0b;
// Index (NID:04h)
pub const RT721_DAC_2CH_CTRL3: c_uint = 0x02;
pub const RT721_DAC_2CH_CTRL4: c_uint = 0x03;
// Index (NID:09h)
pub const RT721_JD_1PIN_GAT_CTRL2: c_uint = 0x07;
// Index (NID:0ah)
pub const RT721_CBJ_A0_GAT_CTRL1: c_uint = 0x04;
pub const RT721_CBJ_A0_GAT_CTRL2: c_uint = 0x05;
// Index (NID:0Ch)
pub const RT721_HP_AMP_2CH_CAL1: c_uint = 0x05;
pub const RT721_HP_AMP_2CH_CAL4: c_uint = 0x08;
pub const RT721_HP_AMP_2CH_CAL18: c_uint = 0x1b;
// Index (NID:0dh)
pub const RT721_CLASD_AMP_2CH_CAL: c_uint = 0x14;
// Index (NID:0fh)
pub const RT721_BST_4CH_TOP_GATING_CTRL1: c_uint = 0x05;
// Index (NID:20h)
pub const RT721_JD_PRODUCT_NUM: c_uint = 0x00;
pub const RT721_ANALOG_BIAS_CTL3: c_uint = 0x04;
pub const RT721_JD_CTRL1: c_uint = 0x09;
pub const RT721_LDO2_3_CTL1: c_uint = 0x0e;
pub const RT721_GPIO_PAD_CTRL5: c_uint = 0x13;
pub const RT721_LDO1_CTL: c_uint = 0x1a;
pub const RT721_HP_JD_CTRL: c_uint = 0x24;
pub const RT721_VD_HIDDEN_CTRL: c_uint = 0x26;
pub const RT721_CLSD_CTRL6: c_uint = 0x3c;
pub const RT721_COMBO_JACK_AUTO_CTL1: c_uint = 0x45;
pub const RT721_COMBO_JACK_AUTO_CTL2: c_uint = 0x46;
pub const RT721_COMBO_JACK_AUTO_CTL3: c_uint = 0x47;
pub const RT721_DIGITAL_MISC_CTRL4: c_uint = 0x4a;
pub const RT721_VREFO_GAT: c_uint = 0x63;
pub const RT721_FSM_CTL: c_uint = 0x67;
pub const RT721_SDCA_INTR_REC: c_uint = 0x82;
pub const RT721_SW_CONFIG1: c_uint = 0x8a;
pub const RT721_SW_CONFIG2: c_uint = 0x8b;
// Index (NID:40h)
pub const RT721_RC_CALIB_CTRL0: c_uint = 0x00;
// Index (NID:58h)
pub const RT721_DAC_DC_CALI_CTL1: c_uint = 0x01;
pub const RT721_DAC_DC_CALI_CTL2: c_uint = 0x02;
pub const RT721_DAC_DC_CALI_CTL3: c_uint = 0x03;
// Index (NID:5fh)
pub const RT721_MISC_POWER_CTL0: c_uint = 0x00;
pub const RT721_MISC_POWER_CTL31: c_uint = 0x31;
pub const RT721_UAJ_TOP_TCON13: c_uint = 0x44;
pub const RT721_UAJ_TOP_TCON14: c_uint = 0x45;
pub const RT721_UAJ_TOP_TCON17: c_uint = 0x48;
// Index (NID:61h)
pub const RT721_HDA_LEGACY_MUX_CTL0: c_uint = 0x00;
pub const RT721_HDA_LEGACY_UAJ_CTL: c_uint = 0x02;
pub const RT721_HDA_LEGACY_CTL1: c_uint = 0x05;
pub const RT721_HDA_LEGACY_RESET_CTL: c_uint = 0x06;
pub const RT721_MISC_CTL: c_uint = 0x07;
pub const RT721_XU_REL_CTRL: c_uint = 0x0c;
pub const RT721_GE_REL_CTRL1: c_uint = 0x0d;
pub const RT721_HDA_LEGACY_GPIO_WAKE_EN_CTL: c_uint = 0x0e;
pub const RT721_GE_SDCA_RST_CTRL: c_uint = 0x10;
pub const RT721_INT_RST_EN_CTRL: c_uint = 0x11;
pub const RT721_XU_EVENT_EN: c_uint = 0x13;
pub const RT721_INLINE_CTL2: c_uint = 0x17;
pub const RT721_UMP_HID_CTRL1: c_uint = 0x18;
pub const RT721_UMP_HID_CTRL2: c_uint = 0x19;
pub const RT721_UMP_HID_CTRL3: c_uint = 0x1a;
pub const RT721_UMP_HID_CTRL4: c_uint = 0x1b;
pub const RT721_UMP_HID_CTRL5: c_uint = 0x1c;
pub const RT721_FUNC_FLOAT_CTL0: c_uint = 0x22;
pub const RT721_FUNC_FLOAT_CTL1: c_uint = 0x23;
pub const RT721_FUNC_FLOAT_CTL2: c_uint = 0x24;
pub const RT721_FUNC_FLOAT_CTL3: c_uint = 0x25;
pub const RT721_ENT_FLOAT_CTL0: c_uint = 0x29;
pub const RT721_ENT_FLOAT_CTL1: c_uint = 0x2c;
pub const RT721_ENT_FLOAT_CTL2: c_uint = 0x2d;
pub const RT721_ENT_FLOAT_CTL3: c_uint = 0x2e;
pub const RT721_ENT_FLOAT_CTL4: c_uint = 0x2f;
pub const RT721_CH_FLOAT_CTL1: c_uint = 0x45;
pub const RT721_CH_FLOAT_CTL2: c_uint = 0x46;
pub const RT721_ENT_FLOAT_CTL5: c_uint = 0x53;
pub const RT721_ENT_FLOAT_CTL6: c_uint = 0x54;
pub const RT721_ENT_FLOAT_CTL7: c_uint = 0x55;
pub const RT721_ENT_FLOAT_CTL8: c_uint = 0x57;
pub const RT721_ENT_FLOAT_CTL9: c_uint = 0x5a;
pub const RT721_ENT_FLOAT_CTL10: c_uint = 0x5b;
pub const RT721_CH_FLOAT_CTL3: c_uint = 0x6a;
pub const RT721_CH_FLOAT_CTL4: c_uint = 0x6d;
pub const RT721_CH_FLOAT_CTL5: c_uint = 0x70;
pub const RT721_CH_FLOAT_CTL6: c_uint = 0x92;
// Parameter & Verb control 01 (0x26)(NID:20h)

// Buffer address for HID
pub const RT721_BUF_ADDR_HID1: c_uint = 0x44030000;
pub const RT721_BUF_ADDR_HID2: c_uint = 0x44030020;
// RT721 SDCA Control - function number
pub const FUNC_NUM_JACK_CODEC: c_uint = 0x01;
pub const FUNC_NUM_MIC_ARRAY: c_uint = 0x02;
pub const FUNC_NUM_HID: c_uint = 0x03;
pub const FUNC_NUM_AMP: c_uint = 0x04;
// RT721 SDCA entity
pub const RT721_SDCA_ENT_HID01: c_uint = 0x01;
pub const RT721_SDCA_ENT_XUV: c_uint = 0x03;
pub const RT721_SDCA_ENT_GE49: c_uint = 0x49;
pub const RT721_SDCA_ENT_USER_FU05: c_uint = 0x05;
pub const RT721_SDCA_ENT_USER_FU06: c_uint = 0x06;
pub const RT721_SDCA_ENT_USER_FU0F: c_uint = 0x0f;
pub const RT721_SDCA_ENT_USER_FU10: c_uint = 0x19;
pub const RT721_SDCA_ENT_USER_FU1E: c_uint = 0x1e;
pub const RT721_SDCA_ENT_FU15: c_uint = 0x15;
pub const RT721_SDCA_ENT_PDE23: c_uint = 0x23;
pub const RT721_SDCA_ENT_PDE40: c_uint = 0x40;
pub const RT721_SDCA_ENT_PDE41: c_uint = 0x41;
pub const RT721_SDCA_ENT_PDE11: c_uint = 0x11;
pub const RT721_SDCA_ENT_PDE12: c_uint = 0x12;
pub const RT721_SDCA_ENT_PDE2A: c_uint = 0x2a;
pub const RT721_SDCA_ENT_CS01: c_uint = 0x01;
pub const RT721_SDCA_ENT_CS11: c_uint = 0x11;
pub const RT721_SDCA_ENT_CS1F: c_uint = 0x1f;
pub const RT721_SDCA_ENT_CS1C: c_uint = 0x1c;
pub const RT721_SDCA_ENT_CS31: c_uint = 0x31;
pub const RT721_SDCA_ENT_OT23: c_uint = 0x42;
pub const RT721_SDCA_ENT_IT26: c_uint = 0x26;
pub const RT721_SDCA_ENT_IT09: c_uint = 0x09;
pub const RT721_SDCA_ENT_PLATFORM_FU15: c_uint = 0x15;
pub const RT721_SDCA_ENT_PLATFORM_FU44: c_uint = 0x44;
pub const RT721_SDCA_ENT_XU03: c_uint = 0x03;
pub const RT721_SDCA_ENT_XU0D: c_uint = 0x0d;
pub const RT721_SDCA_ENT_FU55: c_uint = 0x55;
// RT721 SDCA control
pub const RT721_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT721_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT721_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT721_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const RT721_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: c_uint = 0x11;
pub const RT721_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint = 0x12;
pub const RT721_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0x13;
pub const RT721_SDCA_CTL_SELECTED_MODE: c_uint = 0x01;
pub const RT721_SDCA_CTL_DETECTED_MODE: c_uint = 0x02;
pub const RT721_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT721_SDCA_CTL_VENDOR_DEF: c_uint = 0x30;
pub const RT721_SDCA_CTL_XUV: c_uint = 0x34;
pub const RT721_SDCA_CTL_FU_CH_GAIN: c_uint = 0x0b;
// RT721 SDCA channel
pub const CH_L: c_uint = 0x01;
pub const CH_R: c_uint = 0x02;
pub const CH_01: c_uint = 0x01;
pub const CH_02: c_uint = 0x02;
pub const CH_03: c_uint = 0x03;
pub const CH_04: c_uint = 0x04;
pub const CH_08: c_uint = 0x08;
pub const CH_09: c_uint = 0x09;
pub const CH_0A: c_uint = 0x0a;
// sample frequency index
pub const RT721_SDCA_RATE_8000HZ: c_uint = 0x01;
pub const RT721_SDCA_RATE_11025HZ: c_uint = 0x02;
pub const RT721_SDCA_RATE_12000HZ: c_uint = 0x03;
pub const RT721_SDCA_RATE_16000HZ: c_uint = 0x04;
pub const RT721_SDCA_RATE_22050HZ: c_uint = 0x05;
pub const RT721_SDCA_RATE_24000HZ: c_uint = 0x06;
pub const RT721_SDCA_RATE_32000HZ: c_uint = 0x07;
pub const RT721_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT721_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT721_SDCA_RATE_88200HZ: c_uint = 0x0a;
pub const RT721_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT721_SDCA_RATE_176400HZ: c_uint = 0x0c;
pub const RT721_SDCA_RATE_192000HZ: c_uint = 0x0d;
pub const RT721_SDCA_RATE_384000HZ: c_uint = 0x0e;
pub const RT721_SDCA_RATE_768000HZ: c_uint = 0x0f;
// RT721 HID ID
pub const RT721_SDCA_HID_ID: c_uint = 0x11;
extern "C" {
    pub fn rt721_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
