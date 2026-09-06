//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt712-sdca.h
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
// rt712-sdca.h -- RT712 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt712_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub dmic_component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub /: *mut *mut mutex calibrate_mutex; / for headset calibration,
    pub /: *mut *mut mutex disable_irq_lock; / SDCA irq lock protection,
    pub disable_irq: bool,
    pub jack_type: c_int,
    pub jd_src: c_int,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub hw_id: c_uint,
    pub version_id: c_uint,
    pub dmic_function_found: bool,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
    pub fu05_dapm_mute: bool,
    pub fu05_mixer_l_mute: bool,
    pub fu05_mixer_r_mute: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt712_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}

// SDCA (Channel)
pub const CH_01: c_uint = 0x01;
pub const CH_02: c_uint = 0x02;
pub const CH_03: c_uint = 0x03;
pub const CH_04: c_uint = 0x04;
// NID
pub const RT712_VENDOR_REG: c_uint = 0x20;
pub const RT712_EQ_CTRL: c_uint = 0x53;
pub const RT712_CHARGE_PUMP: c_uint = 0x57;
pub const RT712_VENDOR_CALI: c_uint = 0x58;
pub const RT712_ULTRA_SOUND_DET: c_uint = 0x59;
pub const RT712_VENDOR_IMS_DRE: c_uint = 0x5b;
pub const RT712_VENDOR_ANALOG_CTL: c_uint = 0x5f;
pub const RT712_VENDOR_HDA_CTL: c_uint = 0x61;
// Index (NID:20h)
pub const RT712_JD_PRODUCT_NUM: c_uint = 0x00;
pub const RT712_ANALOG_BIAS_CTL3: c_uint = 0x04;
pub const RT712_JD_CTL1: c_uint = 0x09;
pub const RT712_JD_CTL3: c_uint = 0x0b;
pub const RT712_IO_CTL: c_uint = 0x0c;
pub const RT712_LDO2_3_CTL1: c_uint = 0x0e;
pub const RT712_PARA_VERB_CTL: c_uint = 0x1a;
pub const RT712_CC_DET1: c_uint = 0x24;
pub const RT712_CLASSD_AMP_CTL1: c_uint = 0x37;
pub const RT712_CLASSD_AMP_CTL6: c_uint = 0x3c;
pub const RT712_COMBO_JACK_AUTO_CTL1: c_uint = 0x45;
pub const RT712_COMBO_JACK_AUTO_CTL2: c_uint = 0x46;
pub const RT712_COMBO_JACK_AUTO_CTL3: c_uint = 0x47;
pub const RT712_DIGITAL_MISC_CTRL4: c_uint = 0x4a;
pub const RT712_FSM_CTL: c_uint = 0x67;
pub const RT712_SW_CONFIG1: c_uint = 0x8a;
pub const RT712_SW_CONFIG2: c_uint = 0x8b;
// Index (NID:57h)
pub const RT712_HP_DET_CTL3: c_uint = 0x0c;
// Index (NID:58h)
pub const RT712_DAC_DC_CALI_CTL1: c_uint = 0x00;
pub const RT712_DAC_DC_CALI_CTL2: c_uint = 0x01;
// Index (NID:59h)
pub const RT712_ULTRA_SOUND_DETECTOR6: c_uint = 0x1e;
// Index (NID:5bh)
pub const RT712_IMS_DIGITAL_CTL1: c_uint = 0x00;
pub const RT712_IMS_DIGITAL_CTL5: c_uint = 0x05;
pub const RT712_SEL_VEE2_HP_CTL1: c_uint = 0x23;
pub const RT712_HP_DETECT_RLDET_CTL1: c_uint = 0x29;
pub const RT712_HP_DETECT_RLDET_CTL2: c_uint = 0x2a;
// Index (NID:5fh)
pub const RT712_MISC_POWER_CTL0: c_uint = 0x00;
pub const RT712_MISC_POWER_CTL7: c_uint = 0x08;
// Index (NID:61h)
pub const RT712_HDA_LEGACY_MUX_CTL0: c_uint = 0x00;
pub const RT712_HDA_LEGACY_CONFIG_CTL0: c_uint = 0x06;
pub const RT712_HDA_LEGACY_RESET_CTL: c_uint = 0x08;
pub const RT712_HDA_LEGACY_GPIO_WAKE_EN_CTL: c_uint = 0x0e;
pub const RT712_DMIC_ENT_FLOAT_CTL: c_uint = 0x10;
pub const RT712_DMIC_GAIN_ENT_FLOAT_CTL0: c_uint = 0x11;
pub const RT712_DMIC_GAIN_ENT_FLOAT_CTL2: c_uint = 0x13;
pub const RT712_ADC_ENT_FLOAT_CTL: c_uint = 0x15;
pub const RT712_ADC_VOL_CH_FLOAT_CTL2: c_uint = 0x18;
pub const RT712_DAC03_HP_PDE_FLOAT_CTL: c_uint = 0x22;
pub const RT712_MIC2_LINE2_PDE_FLOAT_CTL: c_uint = 0x23;
pub const RT712_ADC0A_08_PDE_FLOAT_CTL: c_uint = 0x26;
pub const RT712_ADC0B_11_PDE_FLOAT_CTL: c_uint = 0x27;
pub const RT712_DMIC1_2_PDE_FLOAT_CTL: c_uint = 0x2b;
pub const RT712_AMP_PDE_FLOAT_CTL: c_uint = 0x2c;
pub const RT712_I2S_IN_OUT_PDE_FLOAT_CTL: c_uint = 0x2f;
pub const RT712_GE_RELATED_CTL1: c_uint = 0x45;
pub const RT712_GE_RELATED_CTL2: c_uint = 0x46;
pub const RT712_MIXER_CTL0: c_uint = 0x52;
pub const RT712_MIXER_CTL1: c_uint = 0x53;
pub const RT712_EAPD_CTL: c_uint = 0x55;
pub const RT712_UMP_HID_CTL0: c_uint = 0x60;
pub const RT712_UMP_HID_CTL1: c_uint = 0x61;
pub const RT712_UMP_HID_CTL2: c_uint = 0x62;
pub const RT712_UMP_HID_CTL3: c_uint = 0x63;
pub const RT712_UMP_HID_CTL4: c_uint = 0x64;
pub const RT712_UMP_HID_CTL5: c_uint = 0x65;
pub const RT712_UMP_HID_CTL6: c_uint = 0x66;
pub const RT712_UMP_HID_CTL7: c_uint = 0x67;
pub const RT712_UMP_HID_CTL8: c_uint = 0x68;
pub const RT712_MISC_CTL_FOR_UAJ: c_uint = 0x72;
pub const RT712_ADC0A_CS_ADC0B_FU_FLOAT_CTL: c_uint = 0xa2;
pub const RT712_DMIC2_FU_IT_FLOAT_CTL: c_uint = 0xa6;
pub const RT712_ADC0B_FU_CH12_FLOAT_CTL: c_uint = 0xb0;
pub const RT712_DMIC2_FU_CH12_FLOAT_CTL: c_uint = 0xb1;
// Parameter & Verb control 01 (0x1a)(NID:20h)

// combo jack auto switch control 2 (0x46)(NID:20h)

// DAC DC offset calibration control-1 (0x00)(NID:58h)

pub const RT712_EAPD_HIGH: c_uint = 0x2;
pub const RT712_EAPD_LOW: c_uint = 0x0;
// RC Calibration register
pub const RT712_RC_CAL: c_uint = 0x3201;
// Buffer address for HID
pub const RT712_BUF_ADDR_HID1: c_uint = 0x44030000;
pub const RT712_BUF_ADDR_HID2: c_uint = 0x44030020;
// RT712 SDCA Control - function number
pub const FUNC_NUM_JACK_CODEC: c_uint = 0x01;
pub const FUNC_NUM_MIC_ARRAY: c_uint = 0x02;
pub const FUNC_NUM_HID: c_uint = 0x03;
pub const FUNC_NUM_AMP: c_uint = 0x04;
// RT712 SDCA entity
pub const RT712_SDCA_ENT0: c_uint = 0x00;
pub const RT712_SDCA_ENT_HID01: c_uint = 0x01;
pub const RT712_SDCA_ENT_GE49: c_uint = 0x49;
pub const RT712_SDCA_ENT_USER_FU05: c_uint = 0x05;
pub const RT712_SDCA_ENT_USER_FU06: c_uint = 0x06;
pub const RT712_SDCA_ENT_USER_FU0F: c_uint = 0x0f;
pub const RT712_SDCA_ENT_USER_FU10: c_uint = 0x19;
pub const RT712_SDCA_ENT_USER_FU1E: c_uint = 0x1e;
pub const RT712_SDCA_ENT_FU15: c_uint = 0x15;
pub const RT712_SDCA_ENT_PDE23: c_uint = 0x23;
pub const RT712_SDCA_ENT_PDE40: c_uint = 0x40;
pub const RT712_SDCA_ENT_PDE11: c_uint = 0x11;
pub const RT712_SDCA_ENT_PDE12: c_uint = 0x12;
pub const RT712_SDCA_ENT_CS01: c_uint = 0x01;
pub const RT712_SDCA_ENT_CS11: c_uint = 0x11;
pub const RT712_SDCA_ENT_CS1F: c_uint = 0x1f;
pub const RT712_SDCA_ENT_CS1C: c_uint = 0x1c;
pub const RT712_SDCA_ENT_CS31: c_uint = 0x31;
pub const RT712_SDCA_ENT_OT23: c_uint = 0x42;
pub const RT712_SDCA_ENT_IT11: c_uint = 0x26;
pub const RT712_SDCA_ENT_IT26: c_uint = 0x26;
pub const RT712_SDCA_ENT_IT09: c_uint = 0x09;
pub const RT712_SDCA_ENT_PLATFORM_FU15: c_uint = 0x15;
pub const RT712_SDCA_ENT_PLATFORM_FU44: c_uint = 0x44;
// RT712 SDCA control
pub const RT712_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT712_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT712_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT712_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const RT712_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: c_uint = 0x11;
pub const RT712_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint = 0x12;
pub const RT712_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0x13;
pub const RT712_SDCA_CTL_SELECTED_MODE: c_uint = 0x01;
pub const RT712_SDCA_CTL_DETECTED_MODE: c_uint = 0x02;
pub const RT712_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT712_SDCA_CTL_VENDOR_DEF: c_uint = 0x30;
pub const RT712_SDCA_CTL_FU_CH_GAIN: c_uint = 0x0b;
pub const RT712_SDCA_CTL_FUNC_STATUS: c_uint = 0x10;
// Function_Status

// sample frequency index
pub const RT712_SDCA_RATE_16000HZ: c_uint = 0x04;
pub const RT712_SDCA_RATE_32000HZ: c_uint = 0x07;
pub const RT712_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT712_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT712_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT712_SDCA_RATE_192000HZ: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt712_sdca_jd_src {
    RT712_JD_NULL,
    RT712_JD1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt712_sdca_hw_id {
    RT712_DEV_ID_712 = 0x7,
    RT712_DEV_ID_713 = 0x6,
    RT712_DEV_ID_716 = 0x5,
    RT712_DEV_ID_717 = 0x4,
}

pub const RT712_PART_ID_713: c_uint = 0x713;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt712_sdca_version {
    RT712_VA,
    RT712_VB,
}

extern "C" {
    pub fn rt712_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn rt712_sdca_jack_detect(rt712: *mut rt712_sdca_priv, hp: *mut bool, mic: *mut bool) -> c_int;
}
