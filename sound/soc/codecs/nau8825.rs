//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8825.h
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
// NAU8825 ALSA SoC audio driver
//
// Copyright 2015 Google Inc.
// Author: Anatol Pomozov <anatol.pomozov@chrominium.org>
//
pub const NAU8825_REG_RESET: c_uint = 0x00;
pub const NAU8825_REG_ENA_CTRL: c_uint = 0x01;
pub const NAU8825_REG_IIC_ADDR_SET: c_uint = 0x02;
pub const NAU8825_REG_CLK_DIVIDER: c_uint = 0x03;
pub const NAU8825_REG_FLL1: c_uint = 0x04;
pub const NAU8825_REG_FLL2: c_uint = 0x05;
pub const NAU8825_REG_FLL3: c_uint = 0x06;
pub const NAU8825_REG_FLL4: c_uint = 0x07;
pub const NAU8825_REG_FLL5: c_uint = 0x08;
pub const NAU8825_REG_FLL6: c_uint = 0x09;
pub const NAU8825_REG_FLL_VCO_RSV: c_uint = 0x0a;
pub const NAU8825_REG_HSD_CTRL: c_uint = 0x0c;
pub const NAU8825_REG_JACK_DET_CTRL: c_uint = 0x0d;
pub const NAU8825_REG_INTERRUPT_MASK: c_uint = 0x0f;
pub const NAU8825_REG_IRQ_STATUS: c_uint = 0x10;
pub const NAU8825_REG_INT_CLR_KEY_STATUS: c_uint = 0x11;
pub const NAU8825_REG_INTERRUPT_DIS_CTRL: c_uint = 0x12;
pub const NAU8825_REG_SAR_CTRL: c_uint = 0x13;
pub const NAU8825_REG_KEYDET_CTRL: c_uint = 0x14;
pub const NAU8825_REG_VDET_THRESHOLD_1: c_uint = 0x15;
pub const NAU8825_REG_VDET_THRESHOLD_2: c_uint = 0x16;
pub const NAU8825_REG_VDET_THRESHOLD_3: c_uint = 0x17;
pub const NAU8825_REG_VDET_THRESHOLD_4: c_uint = 0x18;
pub const NAU8825_REG_GPIO34_CTRL: c_uint = 0x19;
pub const NAU8825_REG_GPIO12_CTRL: c_uint = 0x1a;
pub const NAU8825_REG_TDM_CTRL: c_uint = 0x1b;
pub const NAU8825_REG_I2S_PCM_CTRL1: c_uint = 0x1c;
pub const NAU8825_REG_I2S_PCM_CTRL2: c_uint = 0x1d;
pub const NAU8825_REG_LEFT_TIME_SLOT: c_uint = 0x1e;
pub const NAU8825_REG_RIGHT_TIME_SLOT: c_uint = 0x1f;
pub const NAU8825_REG_BIQ_CTRL: c_uint = 0x20;
pub const NAU8825_REG_BIQ_COF1: c_uint = 0x21;
pub const NAU8825_REG_BIQ_COF2: c_uint = 0x22;
pub const NAU8825_REG_BIQ_COF3: c_uint = 0x23;
pub const NAU8825_REG_BIQ_COF4: c_uint = 0x24;
pub const NAU8825_REG_BIQ_COF5: c_uint = 0x25;
pub const NAU8825_REG_BIQ_COF6: c_uint = 0x26;
pub const NAU8825_REG_BIQ_COF7: c_uint = 0x27;
pub const NAU8825_REG_BIQ_COF8: c_uint = 0x28;
pub const NAU8825_REG_BIQ_COF9: c_uint = 0x29;
pub const NAU8825_REG_BIQ_COF10: c_uint = 0x2a;
pub const NAU8825_REG_ADC_RATE: c_uint = 0x2b;
pub const NAU8825_REG_DAC_CTRL1: c_uint = 0x2c;
pub const NAU8825_REG_DAC_CTRL2: c_uint = 0x2d;
pub const NAU8825_REG_DAC_DGAIN_CTRL: c_uint = 0x2f;
pub const NAU8825_REG_ADC_DGAIN_CTRL: c_uint = 0x30;
pub const NAU8825_REG_MUTE_CTRL: c_uint = 0x31;
pub const NAU8825_REG_HSVOL_CTRL: c_uint = 0x32;
pub const NAU8825_REG_DACL_CTRL: c_uint = 0x33;
pub const NAU8825_REG_DACR_CTRL: c_uint = 0x34;
pub const NAU8825_REG_ADC_DRC_KNEE_IP12: c_uint = 0x38;
pub const NAU8825_REG_ADC_DRC_KNEE_IP34: c_uint = 0x39;
pub const NAU8825_REG_ADC_DRC_SLOPES: c_uint = 0x3a;
pub const NAU8825_REG_ADC_DRC_ATKDCY: c_uint = 0x3b;
pub const NAU8825_REG_DAC_DRC_KNEE_IP12: c_uint = 0x45;
pub const NAU8825_REG_DAC_DRC_KNEE_IP34: c_uint = 0x46;
pub const NAU8825_REG_DAC_DRC_SLOPES: c_uint = 0x47;
pub const NAU8825_REG_DAC_DRC_ATKDCY: c_uint = 0x48;
pub const NAU8825_REG_IMM_MODE_CTRL: c_uint = 0x4c;
pub const NAU8825_REG_IMM_RMS_L: c_uint = 0x4d;
pub const NAU8825_REG_IMM_RMS_R: c_uint = 0x4e;
pub const NAU8825_REG_CLASSG_CTRL: c_uint = 0x50;
pub const NAU8825_REG_OPT_EFUSE_CTRL: c_uint = 0x51;
pub const NAU8825_REG_MISC_CTRL: c_uint = 0x55;
pub const NAU8825_REG_I2C_DEVICE_ID: c_uint = 0x58;
pub const NAU8825_REG_SARDOUT_RAM_STATUS: c_uint = 0x59;
pub const NAU8825_REG_FLL2_LOWER: c_uint = 0x5a;
pub const NAU8825_REG_FLL2_UPPER: c_uint = 0x5b;
pub const NAU8825_REG_BIAS_ADJ: c_uint = 0x66;
pub const NAU8825_REG_TRIM_SETTINGS: c_uint = 0x68;
pub const NAU8825_REG_ANALOG_CONTROL_1: c_uint = 0x69;
pub const NAU8825_REG_ANALOG_CONTROL_2: c_uint = 0x6a;
pub const NAU8825_REG_ANALOG_ADC_1: c_uint = 0x71;
pub const NAU8825_REG_ANALOG_ADC_2: c_uint = 0x72;
pub const NAU8825_REG_RDAC: c_uint = 0x73;
pub const NAU8825_REG_MIC_BIAS: c_uint = 0x74;
pub const NAU8825_REG_BOOST: c_uint = 0x76;
pub const NAU8825_REG_FEPGA: c_uint = 0x77;
pub const NAU8825_REG_POWER_UP_CONTROL: c_uint = 0x7f;
pub const NAU8825_REG_CHARGE_PUMP: c_uint = 0x80;
pub const NAU8825_REG_CHARGE_PUMP_INPUT_READ: c_uint = 0x81;
pub const NAU8825_REG_GENERAL_STATUS: c_uint = 0x82;

// 16-bit control register address, and 16-bits control register data
pub const NAU8825_REG_ADDR_LEN: c_int = 16;
pub const NAU8825_REG_DATA_LEN: c_int = 16;
// ENA_CTRL (0x1)
pub const NAU8825_ENABLE_DACR_SFT: c_int = 10;

pub const NAU8825_ENABLE_DACL_SFT: c_int = 9;

pub const NAU8825_ENABLE_ADC_SFT: c_int = 8;

pub const NAU8825_ENABLE_ADC_CLK_SFT: c_int = 7;

pub const NAU8825_ENABLE_DAC_CLK_SFT: c_int = 6;

pub const NAU8825_ENABLE_SAR_SFT: c_int = 1;
// CLK_DIVIDER (0x3)
pub const NAU8825_CLK_SRC_SFT: c_int = 15;

pub const NAU8825_CLK_ADC_SRC_SFT: c_int = 6;

pub const NAU8825_CLK_DAC_SRC_SFT: c_int = 4;

// FLL1 (0x04)
pub const NAU8825_ICTRL_LATCH_SFT: c_int = 10;

// FLL3 (0x06)
pub const NAU8825_GAIN_ERR_SFT: c_int = 12;

pub const NAU8825_FLL_CLK_SRC_SFT: c_int = 10;

// FLL4 (0x07)
pub const NAU8825_FLL_REF_DIV_SFT: c_int = 10;

// FLL5 (0x08)

// FLL6 (0x9)

// HSD_CTRL (0xc)

// 0 - open, 1 - short to GND

// JACK_DET_CTRL (0xd)

pub const NAU8825_JACK_INSERT_DEBOUNCE_SFT: c_int = 5;

pub const NAU8825_JACK_EJECT_DEBOUNCE_SFT: c_int = 2;

// INTERRUPT_MASK (0xf)

// IRQ_STATUS (0x10)

// INTERRUPT_DIS_CTRL (0x12)

// SAR_CTRL (0x13)
pub const NAU8825_SAR_ADC_EN_SFT: c_int = 12;

pub const NAU8825_SAR_TRACKING_GAIN_SFT: c_int = 8;

pub const NAU8825_SAR_HV_SEL_SFT: c_int = 7;

pub const NAU8825_SAR_RES_SEL_SFT: c_int = 4;

pub const NAU8825_SAR_COMPARE_TIME_SFT: c_int = 2;

pub const NAU8825_SAR_SAMPLING_TIME_SFT: c_int = 0;

// KEYDET_CTRL (0x14)
pub const NAU8825_KEYDET_SHORTKEY_DEBOUNCE_SFT: c_int = 12;

pub const NAU8825_KEYDET_LEVELS_NR_SFT: c_int = 8;

pub const NAU8825_KEYDET_HYSTERESIS_SFT: c_int = 0;
pub const NAU8825_KEYDET_HYSTERESIS_MASK: c_uint = 0xf;
// GPIO12_CTRL (0x1a)

// TDM_CTRL (0x1b)

pub const NAU8825_TDM_DACL_RX_SFT: c_int = 6;

pub const NAU8825_TDM_DACR_RX_SFT: c_int = 4;

pub const NAU8825_TDM_TX_MASK: c_uint = 0x3;
// I2S_PCM_CTRL1 (0x1c)
pub const NAU8825_I2S_BP_SFT: c_int = 7;

pub const NAU8825_I2S_PCMB_SFT: c_int = 6;

pub const NAU8825_I2S_DL_SFT: c_int = 2;

pub const NAU8825_I2S_DF_SFT: c_int = 0;

// I2S_PCM_CTRL2 (0x1d)

pub const NAU8825_I2S_LRC_DIV_SFT: c_int = 12;

pub const NAU8825_I2S_PCM_TS_EN_SFT: c_int = 10;

pub const NAU8825_I2S_MS_SFT: c_int = 3;

pub const NAU8825_I2S_BLK_DIV_MASK: c_uint = 0x7;
// LEFT_TIME_SLOT (0x1e)
pub const NAU8825_FS_ERR_CMP_SEL_SFT: c_int = 14;

pub const NAU8825_TSLOT_L0_MASK: c_uint = 0x3ff;
pub const NAU8825_TSLOT_R0_MASK: c_uint = 0x3ff;
// BIQ_CTRL (0x20)
pub const NAU8825_BIQ_WRT_SFT: c_int = 4;

pub const NAU8825_BIQ_PATH_SFT: c_int = 0;

// ADC_RATE (0x2b)
pub const NAU8825_ADC_SINC4_SFT: c_int = 4;

pub const NAU8825_ADC_SYNC_DOWN_SFT: c_int = 0;
pub const NAU8825_ADC_SYNC_DOWN_MASK: c_uint = 0x3;
pub const NAU8825_ADC_SYNC_DOWN_32: c_int = 0;
pub const NAU8825_ADC_SYNC_DOWN_64: c_int = 1;
pub const NAU8825_ADC_SYNC_DOWN_128: c_int = 2;
pub const NAU8825_ADC_SYNC_DOWN_256: c_int = 3;
// DAC_CTRL1 (0x2c)

pub const NAU8825_DAC_OVERSAMPLE_SFT: c_int = 0;
pub const NAU8825_DAC_OVERSAMPLE_MASK: c_uint = 0x7;
pub const NAU8825_DAC_OVERSAMPLE_64: c_int = 0;
pub const NAU8825_DAC_OVERSAMPLE_256: c_int = 1;
pub const NAU8825_DAC_OVERSAMPLE_128: c_int = 2;
pub const NAU8825_DAC_OVERSAMPLE_32: c_int = 4;
// ADC_DGAIN_CTRL (0x30)
pub const NAU8825_ADC_DIG_VOL_MASK: c_uint = 0xff;
// MUTE_CTRL (0x31)

// HSVOL_CTRL (0x32)

pub const NAU8825_HPL_VOL_SFT: c_int = 6;

pub const NAU8825_HPR_VOL_SFT: c_int = 0;

pub const NAU8825_HP_VOL_MIN: c_uint = 0x36;
// DACL_CTRL (0x33)
pub const NAU8825_DACL_CH_SEL_SFT: c_int = 9;

pub const NAU8825_DACL_CH_VOL_MASK: c_uint = 0xff;
// DACR_CTRL (0x34)
pub const NAU8825_DACR_CH_SEL_SFT: c_int = 9;

pub const NAU8825_DACR_CH_VOL_MASK: c_uint = 0xff;
// IMM_MODE_CTRL (0x4C)
pub const NAU8825_IMM_THD_SFT: c_int = 8;

pub const NAU8825_IMM_GEN_VOL_SFT: c_int = 6;

pub const NAU8825_IMM_CYC_SFT: c_int = 4;

pub const NAU8825_IMM_DAC_SRC_MASK: c_uint = 0x7;
pub const NAU8825_IMM_DAC_SRC_BIQ: c_uint = 0x0;
pub const NAU8825_IMM_DAC_SRC_DRC: c_uint = 0x1;
pub const NAU8825_IMM_DAC_SRC_MIX: c_uint = 0x2;
pub const NAU8825_IMM_DAC_SRC_SIN: c_uint = 0x3;
// CLASSG_CTRL (0x50)
pub const NAU8825_CLASSG_TIMER_SFT: c_int = 8;

// I2C_DEVICE_ID (0x58)

pub const NAU8825_SOFTWARE_ID_MASK: c_uint = 0x3;
pub const NAU8825_SOFTWARE_ID_NAU8825: c_uint = 0x0;
pub const NAU8825_SOFTWARE_ID_NAU8825C: c_uint = 0x1;
// BIAS_ADJ (0x66)

pub const NAU8825_BIAS_TESTDAC_SFT: c_int = 8;

pub const NAU8825_BIAS_VMID_SEL_SFT: c_int = 4;

// ANALOG_CONTROL_1 (0x69)
pub const NAU8825_TESTDACIN_SFT: c_int = 14;

// ANALOG_CONTROL_2 (0x6a)

// ANALOG_ADC_2 (0x72)

// RDAC (0x73)

pub const NAU8825_RDAC_EN_SFT: c_int = 12;

pub const NAU8825_RDAC_CLK_EN_SFT: c_int = 8;

pub const NAU8825_RDAC_CLK_DELAY_SFT: c_int = 4;

pub const NAU8825_RDAC_VREF_SFT: c_int = 2;

// MIC_BIAS (0x74)

pub const NAU8825_MICBIAS_LOWNOISE_SFT: c_int = 10;

pub const NAU8825_MICBIAS_POWERUP_SFT: c_int = 8;
pub const NAU8825_MICBIAS_VOLTAGE_SFT: c_int = 0;
pub const NAU8825_MICBIAS_VOLTAGE_MASK: c_uint = 0x7;
// BOOST (0x76)

// FEPGA (0x77)
pub const NAU8825_ACDC_CTRL_SFT: c_int = 14;

// POWER_UP_CONTROL (0x7f)

// CHARGE_PUMP (0x80)
pub const NAU8825_ADCOUT_DS_SFT: c_int = 12;

// System Clock Source
// Cross talk detection state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8825 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub mclk: *mut clk,
    pub xtalk_work: work_struct,
    pub xtalk_sem: semaphore,
    pub sw_id: c_int,
    pub irq: c_int,
    pub /: *mut *mut int mclk_freq; / 0 - mclk is disabled,
    pub button_pressed: c_int,
    pub micbias_voltage: c_int,
    pub vref_impedance: c_int,
    pub jkdet_enable: bool,
    pub jkdet_pull_enable: bool,
    pub jkdet_pull_up: bool,
    pub jkdet_polarity: c_int,
    pub sar_threshold_num: c_int,
    pub sar_threshold: [c_int; 8],
    pub sar_hysteresis: c_int,
    pub sar_voltage: c_int,
    pub sar_compare_time: c_int,
    pub sar_sampling_time: c_int,
    pub key_debounce: c_int,
    pub jack_insert_debounce: c_int,
    pub jack_eject_debounce: c_int,
    pub high_imped: c_int,
    pub xtalk_state: c_int,
    pub xtalk_event: c_int,
    pub xtalk_event_mask: c_int,
    pub xtalk_protect: bool,
    pub imp_rms: [c_int; NAU8825_XTALK_IMM],
    pub xtalk_enable: c_int,
    pub /: *mut *mut bool xtalk_baktab_initialized; / True if initialized.,
    pub adcout_ds: bool,
    pub adc_delay: c_int,
}
