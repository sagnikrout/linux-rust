//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8824.h
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
// NAU88L24 ALSA SoC audio driver
//
// Copyright 2016 Nuvoton Technology Corp.
// Author: John Hsu <KCHSU0@nuvoton.com>
//
pub const NAU8824_REG_RESET: c_uint = 0x00;
pub const NAU8824_REG_ENA_CTRL: c_uint = 0x01;
pub const NAU8824_REG_CLK_GATING_ENA: c_uint = 0x02;
pub const NAU8824_REG_CLK_DIVIDER: c_uint = 0x03;
pub const NAU8824_REG_FLL1: c_uint = 0x04;
pub const NAU8824_REG_FLL2: c_uint = 0x05;
pub const NAU8824_REG_FLL3: c_uint = 0x06;
pub const NAU8824_REG_FLL4: c_uint = 0x07;
pub const NAU8824_REG_FLL5: c_uint = 0x08;
pub const NAU8824_REG_FLL6: c_uint = 0x09;
pub const NAU8824_REG_FLL_VCO_RSV: c_uint = 0x0A;
pub const NAU8824_REG_JACK_DET_CTRL: c_uint = 0x0D;
pub const NAU8824_REG_INTERRUPT_SETTING_1: c_uint = 0x0F;
pub const NAU8824_REG_IRQ: c_uint = 0x10;
pub const NAU8824_REG_CLEAR_INT_REG: c_uint = 0x11;
pub const NAU8824_REG_INTERRUPT_SETTING: c_uint = 0x12;
pub const NAU8824_REG_SAR_ADC: c_uint = 0x13;
pub const NAU8824_REG_VDET_COEFFICIENT: c_uint = 0x14;
pub const NAU8824_REG_VDET_THRESHOLD_1: c_uint = 0x15;
pub const NAU8824_REG_VDET_THRESHOLD_2: c_uint = 0x16;
pub const NAU8824_REG_VDET_THRESHOLD_3: c_uint = 0x17;
pub const NAU8824_REG_VDET_THRESHOLD_4: c_uint = 0x18;
pub const NAU8824_REG_GPIO_SEL: c_uint = 0x1A;
pub const NAU8824_REG_PORT0_I2S_PCM_CTRL_1: c_uint = 0x1C;
pub const NAU8824_REG_PORT0_I2S_PCM_CTRL_2: c_uint = 0x1D;
pub const NAU8824_REG_PORT0_LEFT_TIME_SLOT: c_uint = 0x1E;
pub const NAU8824_REG_PORT0_RIGHT_TIME_SLOT: c_uint = 0x1F;
pub const NAU8824_REG_TDM_CTRL: c_uint = 0x20;
pub const NAU8824_REG_ADC_HPF_FILTER: c_uint = 0x23;
pub const NAU8824_REG_ADC_FILTER_CTRL: c_uint = 0x24;
pub const NAU8824_REG_DAC_FILTER_CTRL_1: c_uint = 0x25;
pub const NAU8824_REG_DAC_FILTER_CTRL_2: c_uint = 0x26;
pub const NAU8824_REG_NOTCH_FILTER_1: c_uint = 0x27;
pub const NAU8824_REG_NOTCH_FILTER_2: c_uint = 0x28;
pub const NAU8824_REG_EQ1_LOW: c_uint = 0x29;
pub const NAU8824_REG_EQ2_EQ3: c_uint = 0x2A;
pub const NAU8824_REG_EQ4_EQ5: c_uint = 0x2B;
pub const NAU8824_REG_ADC_CH0_DGAIN_CTRL: c_uint = 0x2D;
pub const NAU8824_REG_ADC_CH1_DGAIN_CTRL: c_uint = 0x2E;
pub const NAU8824_REG_ADC_CH2_DGAIN_CTRL: c_uint = 0x2F;
pub const NAU8824_REG_ADC_CH3_DGAIN_CTRL: c_uint = 0x30;
pub const NAU8824_REG_DAC_MUTE_CTRL: c_uint = 0x31;
pub const NAU8824_REG_DAC_CH0_DGAIN_CTRL: c_uint = 0x32;
pub const NAU8824_REG_DAC_CH1_DGAIN_CTRL: c_uint = 0x33;
pub const NAU8824_REG_ADC_TO_DAC_ST: c_uint = 0x34;
pub const NAU8824_REG_DRC_KNEE_IP12_ADC_CH01: c_uint = 0x38;
pub const NAU8824_REG_DRC_KNEE_IP34_ADC_CH01: c_uint = 0x39;
pub const NAU8824_REG_DRC_SLOPE_ADC_CH01: c_uint = 0x3A;
pub const NAU8824_REG_DRC_ATKDCY_ADC_CH01: c_uint = 0x3B;
pub const NAU8824_REG_DRC_KNEE_IP12_ADC_CH23: c_uint = 0x3C;
pub const NAU8824_REG_DRC_KNEE_IP34_ADC_CH23: c_uint = 0x3D;
pub const NAU8824_REG_DRC_SLOPE_ADC_CH23: c_uint = 0x3E;
pub const NAU8824_REG_DRC_ATKDCY_ADC_CH23: c_uint = 0x3F;
pub const NAU8824_REG_DRC_GAINL_ADC0: c_uint = 0x40;
pub const NAU8824_REG_DRC_GAINL_ADC1: c_uint = 0x41;
pub const NAU8824_REG_DRC_GAINL_ADC2: c_uint = 0x42;
pub const NAU8824_REG_DRC_GAINL_ADC3: c_uint = 0x43;
pub const NAU8824_REG_DRC_KNEE_IP12_DAC: c_uint = 0x45;
pub const NAU8824_REG_DRC_KNEE_IP34_DAC: c_uint = 0x46;
pub const NAU8824_REG_DRC_SLOPE_DAC: c_uint = 0x47;
pub const NAU8824_REG_DRC_ATKDCY_DAC: c_uint = 0x48;
pub const NAU8824_REG_DRC_GAIN_DAC_CH0: c_uint = 0x49;
pub const NAU8824_REG_DRC_GAIN_DAC_CH1: c_uint = 0x4A;
pub const NAU8824_REG_MODE: c_uint = 0x4C;
pub const NAU8824_REG_MODE1: c_uint = 0x4D;
pub const NAU8824_REG_MODE2: c_uint = 0x4E;
pub const NAU8824_REG_CLASSG: c_uint = 0x50;
pub const NAU8824_REG_OTP_EFUSE: c_uint = 0x51;
pub const NAU8824_REG_OTPDOUT_1: c_uint = 0x53;
pub const NAU8824_REG_OTPDOUT_2: c_uint = 0x54;
pub const NAU8824_REG_MISC_CTRL: c_uint = 0x55;
pub const NAU8824_REG_I2C_TIMEOUT: c_uint = 0x56;
pub const NAU8824_REG_TEST_MODE: c_uint = 0x57;
pub const NAU8824_REG_I2C_DEVICE_ID: c_uint = 0x58;
pub const NAU8824_REG_SAR_ADC_DATA_OUT: c_uint = 0x59;
pub const NAU8824_REG_BIAS_ADJ: c_uint = 0x66;
pub const NAU8824_REG_PGA_GAIN: c_uint = 0x67;
pub const NAU8824_REG_TRIM_SETTINGS: c_uint = 0x68;
pub const NAU8824_REG_ANALOG_CONTROL_1: c_uint = 0x69;
pub const NAU8824_REG_ANALOG_CONTROL_2: c_uint = 0x6A;
pub const NAU8824_REG_ENABLE_LO: c_uint = 0x6B;
pub const NAU8824_REG_GAIN_LO: c_uint = 0x6C;
pub const NAU8824_REG_CLASSD_GAIN_1: c_uint = 0x6D;
pub const NAU8824_REG_CLASSD_GAIN_2: c_uint = 0x6E;
pub const NAU8824_REG_ANALOG_ADC_1: c_uint = 0x71;
pub const NAU8824_REG_ANALOG_ADC_2: c_uint = 0x72;
pub const NAU8824_REG_RDAC: c_uint = 0x73;
pub const NAU8824_REG_MIC_BIAS: c_uint = 0x74;
pub const NAU8824_REG_HS_VOLUME_CONTROL: c_uint = 0x75;
pub const NAU8824_REG_BOOST: c_uint = 0x76;
pub const NAU8824_REG_FEPGA: c_uint = 0x77;
pub const NAU8824_REG_FEPGA_II: c_uint = 0x78;
pub const NAU8824_REG_FEPGA_SE: c_uint = 0x79;
pub const NAU8824_REG_FEPGA_ATTENUATION: c_uint = 0x7A;
pub const NAU8824_REG_ATT_PORT0: c_uint = 0x7B;
pub const NAU8824_REG_ATT_PORT1: c_uint = 0x7C;
pub const NAU8824_REG_POWER_UP_CONTROL: c_uint = 0x7F;
pub const NAU8824_REG_CHARGE_PUMP_CONTROL: c_uint = 0x80;
pub const NAU8824_REG_CHARGE_PUMP_INPUT: c_uint = 0x81;

// 16-bit control register address, and 16-bits control register data
pub const NAU8824_REG_ADDR_LEN: c_int = 16;
pub const NAU8824_REG_DATA_LEN: c_int = 16;
// ENA_CTRL (0x1)

pub const NAU8824_ADC_CH3_DMIC_SFT: c_int = 9;

pub const NAU8824_ADC_CH2_DMIC_SFT: c_int = 8;

pub const NAU8824_ADC_CH1_DMIC_SFT: c_int = 7;

pub const NAU8824_ADC_CH0_DMIC_SFT: c_int = 6;

pub const NAU8824_ADC_CH0_EN: c_uint = 0x1;
// CLK_GATING_ENA (0x02)

// CLK_DIVIDER (0x3)
pub const NAU8824_CLK_SRC_SFT: c_int = 15;

pub const NAU8824_CLK_DMIC_SRC_SFT: c_int = 10;

pub const NAU8824_CLK_ADC_SRC_SFT: c_int = 6;

pub const NAU8824_CLK_DAC_SRC_SFT: c_int = 4;

// FLL1 (0x04)

// FLL3 (0x06)

pub const NAU8824_FLL_CLK_SRC_SFT: c_int = 10;

// FLL4 (0x07)
pub const NAU8824_FLL_REF_DIV_SFT: c_int = 10;

// FLL5 (0x08)

// FLL6 (0x9)

// IRQ (0x10)

pub const NAU8824_JACK_INSERTION_DETECTED: c_uint = 0x1;
// JACK_DET_CTRL (0x0D)
pub const NAU8824_JACK_EJECT_DT_SFT: c_int = 2;

// INTERRUPT_SETTING_1 (0x0F)

// INTERRUPT_SETTING (0x12)

pub const NAU8824_IRQ_INSERT_DIS: c_uint = 0x1;
// SAR_ADC (0x13)
pub const NAU8824_SAR_ADC_EN_SFT: c_int = 12;
pub const NAU8824_SAR_TRACKING_GAIN_SFT: c_int = 8;

pub const NAU8824_SAR_COMPARE_TIME_SFT: c_int = 2;

pub const NAU8824_SAR_SAMPLING_TIME_SFT: c_int = 0;

// VDET_COEFFICIENT (0x14)
pub const NAU8824_SHORTKEY_DEBOUNCE_SFT: c_int = 12;

pub const NAU8824_LEVELS_NR_SFT: c_int = 8;

pub const NAU8824_HYSTERESIS_SFT: c_int = 0;
pub const NAU8824_HYSTERESIS_MASK: c_uint = 0xf;
// PORT0_I2S_PCM_CTRL_1 (0x1C)
pub const NAU8824_I2S_BP_SFT: c_int = 7;

pub const NAU8824_I2S_PCMB_SFT: c_int = 6;

pub const NAU8824_I2S_DL_SFT: c_int = 2;

pub const NAU8824_I2S_DF_MASK: c_uint = 0x3;
pub const NAU8824_I2S_DF_RIGTH: c_int = 0;
pub const NAU8824_I2S_DF_LEFT: c_int = 1;
pub const NAU8824_I2S_DF_I2S: c_int = 2;
pub const NAU8824_I2S_DF_PCM_AB: c_int = 3;
// PORT0_I2S_PCM_CTRL_2 (0x1D)
pub const NAU8824_I2S_LRC_DIV_SFT: c_int = 12;

pub const NAU8824_I2S_MS_SFT: c_int = 3;

pub const NAU8824_I2S_BLK_DIV_MASK: c_uint = 0x7;
// PORT0_LEFT_TIME_SLOT (0x1E)
pub const NAU8824_TSLOT_L_MASK: c_uint = 0x3ff;
// TDM_CTRL (0x20)

pub const NAU8824_TDM_DACL_RX_SFT: c_int = 6;

pub const NAU8824_TDM_DACR_RX_SFT: c_int = 4;

pub const NAU8824_TDM_TX_MASK: c_uint = 0xf;
// ADC_FILTER_CTRL (0x24)
pub const NAU8824_ADC_SYNC_DOWN_MASK: c_uint = 0x3;
pub const NAU8824_ADC_SYNC_DOWN_32: c_int = 0;
pub const NAU8824_ADC_SYNC_DOWN_64: c_int = 1;
pub const NAU8824_ADC_SYNC_DOWN_128: c_int = 2;
pub const NAU8824_ADC_SYNC_DOWN_256: c_int = 3;
// DAC_FILTER_CTRL_1 (0x25)

pub const NAU8824_DAC_OVERSAMPLE_MASK: c_uint = 0x7;
pub const NAU8824_DAC_OVERSAMPLE_64: c_int = 0;
pub const NAU8824_DAC_OVERSAMPLE_256: c_int = 1;
pub const NAU8824_DAC_OVERSAMPLE_128: c_int = 2;
pub const NAU8824_DAC_OVERSAMPLE_32: c_int = 4;
// DAC_MUTE_CTRL (0x31)
pub const NAU8824_DAC_CH01_MIX: c_uint = 0x3;

// DAC_CH0_DGAIN_CTRL (0x32)
pub const NAU8824_DAC_CH0_SEL_SFT: c_int = 9;

pub const NAU8824_DAC_CH0_VOL_MASK: c_uint = 0x1ff;
// DAC_CH1_DGAIN_CTRL (0x33)
pub const NAU8824_DAC_CH1_SEL_SFT: c_int = 9;

pub const NAU8824_DAC_CH1_VOL_MASK: c_uint = 0x1ff;
// CLASSG (0x50)
pub const NAU8824_CLASSG_TIMER_SFT: c_int = 8;

pub const NAU8824_CLASSG_LDAC_EN_SFT: c_int = 2;
pub const NAU8824_CLASSG_RDAC_EN_SFT: c_int = 1;
pub const NAU8824_CLASSG_EN_SFT: c_int = 0;
// SAR_ADC_DATA_OUT (0x59)
pub const NAU8824_SAR_ADC_DATA_MASK: c_uint = 0xff;
// BIAS_ADJ (0x66)

pub const NAU8824_VMID_SEL_SFT: c_int = 4;

pub const NAU8824_DMIC2_EN_SFT: c_int = 3;
pub const NAU8824_DMIC1_EN_SFT: c_int = 2;
// TRIM_SETTINGS (0x68)

// ANALOG_CONTROL_1 (0x69)

// ANALOG_CONTROL_2 (0x6A)
pub const NAU8824_CLASSD_CLAMP_DIS_SFT: c_int = 3;

// ENABLE_LO (0x6B)
pub const NAU8824_TEST_DAC_SFT: c_int = 14;

pub const NAU8824_DACL_HPR_EN_SFT: c_int = 3;

pub const NAU8824_DACR_HPR_EN_SFT: c_int = 2;

pub const NAU8824_DACR_HPL_EN_SFT: c_int = 1;

pub const NAU8824_DACL_HPL_EN_SFT: c_int = 0;
pub const NAU8824_DACL_HPL_EN: c_uint = 0x1;
// CLASSD_GAIN_1 (0x6D)
pub const NAU8824_CLASSD_GAIN_1R_SFT: c_int = 8;

pub const NAU8824_CLASSD_EN_SFT: c_int = 7;

pub const NAU8824_CLASSD_GAIN_1L_MASK: c_uint = 0x1f;
// CLASSD_GAIN_2 (0x6E)
pub const NAU8824_CLASSD_GAIN_2R_SFT: c_int = 8;

pub const NAU8824_CLASSD_EN_SFT: c_int = 7;

pub const NAU8824_CLASSD_GAIN_2L_MASK: c_uint = 0x1f;
// ANALOG_ADC_2 (0x72)
pub const NAU8824_ADCR_EN_SFT: c_int = 7;
pub const NAU8824_ADCL_EN_SFT: c_int = 6;
// RDAC (0x73)
pub const NAU8824_DACR_EN_SFT: c_int = 13;
pub const NAU8824_DACL_EN_SFT: c_int = 12;
pub const NAU8824_DACR_CLK_SFT: c_int = 9;
pub const NAU8824_DACL_CLK_SFT: c_int = 8;
pub const NAU8824_RDAC_CLK_DELAY_SFT: c_int = 4;

pub const NAU8824_RDAC_VREF_SFT: c_int = 2;

// MIC_BIAS (0x74)

pub const NAU8824_MICBIAS_POWERUP_SFT: c_int = 8;
pub const NAU8824_MICBIAS_VOLTAGE_SFT: c_int = 0;
pub const NAU8824_MICBIAS_VOLTAGE_MASK: c_uint = 0x7;
// BOOST (0x76)

pub const NAU8824_HP_BOOST_DIS_SFT: c_int = 9;

pub const NAU8824_HP_BOOST_G_DIS_SFT: c_int = 8;

// FEPGA (0x77)
pub const NAU8824_FEPGA_MODER_SHORT_SFT: c_int = 7;

pub const NAU8824_FEPGA_MODER_MIC2_SFT: c_int = 5;

pub const NAU8824_FEPGA_MODER_HSMIC_SFT: c_int = 4;

pub const NAU8824_FEPGA_MODEL_SHORT_SFT: c_int = 3;

pub const NAU8824_FEPGA_MODEL_MIC1_SFT: c_int = 1;

pub const NAU8824_FEPGA_MODEL_HSMIC_SFT: c_int = 0;

// FEPGA_II (0x78)
pub const NAU8824_FEPGA_GAINR_SFT: c_int = 5;

pub const NAU8824_FEPGA_GAINL_SFT: c_int = 0;
pub const NAU8824_FEPGA_GAINL_MASK: c_uint = 0x1f;
// CHARGE_PUMP_CONTROL (0x80)

pub const NAU8824_CHARGE_PUMP_EN_SFT: c_int = 5;

// System Clock Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8824 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub jdet_work: work_struct,
    pub jd_sem: semaphore,
    pub mclk: *mut clk,
    pub fs: c_int,
    pub irq: c_int,
    pub resume_lock: c_int,
    pub micbias_voltage: c_int,
    pub vref_impedance: c_int,
    pub jkdet_polarity: c_int,
    pub sar_threshold_num: c_int,
    pub sar_threshold: [c_int; 8],
    pub sar_hysteresis: c_int,
    pub sar_voltage: c_int,
    pub sar_compare_time: c_int,
    pub sar_sampling_time: c_int,
    pub key_debounce: c_int,
    pub jack_eject_debounce: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8824_fll {
    pub mclk_src: c_int,
    pub ratio: c_int,
    pub fll_frac: c_int,
    pub fll_int: c_int,
    pub clk_ref_div: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8824_fll_attr {
    pub param: c_uint,
    pub val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8824_osr_attr {
    pub osr: c_uint,
    pub clk_src: c_uint,
}
