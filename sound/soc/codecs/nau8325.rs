//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8325.h
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
// nau8325.h -- Nuvoton NAU8325 audio codec driver
//
// Copyright 2023 Nuvoton Technology Crop.
// Author: Seven Lee <WTLI@nuvoton.com>
// David Lin <CTLIN0@nuvoton.com>
//
pub const NAU8325_R00_HARDWARE_RST: c_uint = 0x00;
pub const NAU8325_R01_SOFTWARE_RST: c_uint = 0x01;
pub const NAU8325_R02_DEVICE_ID: c_uint = 0x02;
pub const NAU8325_R03_CLK_CTRL: c_uint = 0x03;
pub const NAU8325_R04_ENA_CTRL: c_uint = 0x04;
pub const NAU8325_R05_INTERRUPT_CTRL: c_uint = 0x05;
pub const NAU8325_R06_INT_CLR_STATUS: c_uint = 0x06;
pub const NAU8325_R09_IRQOUT: c_uint = 0x09;
pub const NAU8325_R0A_IO_CTRL: c_uint = 0x0a;
pub const NAU8325_R0B_PDM_CTRL: c_uint = 0x0b;
pub const NAU8325_R0C_TDM_CTRL: c_uint = 0x0c;
pub const NAU8325_R0D_I2S_PCM_CTRL1: c_uint = 0x0d;
pub const NAU8325_R0E_I2S_PCM_CTRL2: c_uint = 0x0e;
pub const NAU8325_R0F_L_TIME_SLOT: c_uint = 0x0f;
pub const NAU8325_R10_R_TIME_SLOT: c_uint = 0x10;
pub const NAU8325_R11_HPF_CTRL: c_uint = 0x11;
pub const NAU8325_R12_MUTE_CTRL: c_uint = 0x12;
pub const NAU8325_R13_DAC_VOLUME: c_uint = 0x13;
pub const NAU8325_R1D_DEBUG_READ1: c_uint = 0x1d;
pub const NAU8325_R1F_DEBUG_READ2: c_uint = 0x1f;
pub const NAU8325_R22_DEBUG_READ3: c_uint = 0x22;
pub const NAU8325_R29_DAC_CTRL1: c_uint = 0x29;
pub const NAU8325_R2A_DAC_CTRL2: c_uint = 0x2a;
pub const NAU8325_R2C_ALC_CTRL1: c_uint = 0x2c;
pub const NAU8325_R2D_ALC_CTRL2: c_uint = 0x2d;
pub const NAU8325_R2E_ALC_CTRL3: c_uint = 0x2e;
pub const NAU8325_R2F_ALC_CTRL4: c_uint = 0x2f;
pub const NAU8325_R40_CLK_DET_CTRL: c_uint = 0x40;
pub const NAU8325_R49_TEST_STATUS: c_uint = 0x49;
pub const NAU8325_R4A_ANALOG_READ: c_uint = 0x4a;
pub const NAU8325_R50_MIXER_CTRL: c_uint = 0x50;
pub const NAU8325_R55_MISC_CTRL: c_uint = 0x55;
pub const NAU8325_R60_BIAS_ADJ: c_uint = 0x60;
pub const NAU8325_R61_ANALOG_CONTROL_1: c_uint = 0x61;
pub const NAU8325_R62_ANALOG_CONTROL_2: c_uint = 0x62;
pub const NAU8325_R63_ANALOG_CONTROL_3: c_uint = 0x63;
pub const NAU8325_R64_ANALOG_CONTROL_4: c_uint = 0x64;
pub const NAU8325_R65_ANALOG_CONTROL_5: c_uint = 0x65;
pub const NAU8325_R66_ANALOG_CONTROL_6: c_uint = 0x66;
pub const NAU8325_R69_CLIP_CTRL: c_uint = 0x69;
pub const NAU8325_R73_RDAC: c_uint = 0x73;

// 16-bit control register address, and 16-bits control register data
pub const NAU8325_REG_ADDR_LEN: c_int = 16;
pub const NAU8325_REG_DATA_LEN: c_int = 16;
// CLK_CTRL (0x03)
pub const NAU8325_CLK_DAC_SRC_SFT: c_int = 12;

pub const NAU8325_CLK_MUL_SRC_SFT: c_int = 6;

pub const NAU8325_MCLK_SEL_SFT: c_int = 3;

pub const NAU8325_MCLK_SRC_MASK: c_uint = 0x7;
// ENA_CTRL (0x04)
pub const NAU8325_DAC_LEFT_CH_EN_SFT: c_int = 3;

pub const NAU8325_DAC_RIGHT_CH_EN_SFT: c_int = 2;

// INTERRUPT_CTRL (0x05)
pub const NAU8325_ARP_DWN_INT_SFT: c_int = 12;

pub const NAU8325_CLIP_INT_SFT: c_int = 11;

pub const NAU8325_LVD_INT_SFT: c_int = 10;

pub const NAU8325_PWR_INT_DIS_SFT: c_int = 8;

pub const NAU8325_OCP_OTP_SHTDWN_INT_SFT: c_int = 4;

pub const NAU8325_CLIP_INT_DIS_SFT: c_int = 3;

pub const NAU8325_LVD_INT_DIS_SFT: c_int = 2;

pub const NAU8325_PWR_INT_MASK: c_uint = 0x1;
// INT_CLR_STATUS (0x06)
pub const NAU8325_INT_STATUS_MASK: c_uint = 0x7f;
// IRQOUT (0x9)
pub const NAU8325_IRQOUT_SEL_SEF: c_int = 12;

pub const NAU8325_DEM_DITH_SFT: c_int = 7;

pub const NAU8325_GAINZI3_SFT: c_int = 5;

pub const NAU8325_GAINZI2_MASK: c_uint = 0x1f;
// IO_CTRL (0x0a)
pub const NAU8325_IRQ_PL_SFT: c_int = 15;

pub const NAU8325_IRQ_PS_SFT: c_int = 14;

pub const NAU8325_IRQ_PE_SFT: c_int = 13;

pub const NAU8325_IRQ_DS_SFT: c_int = 12;

pub const NAU8325_IRQ_OUTPUT_SFT: c_int = 11;

pub const NAU8325_IRQ_PIN_DEBUG_SFT: c_int = 10;

// PDM_CTRL (0x0b)
pub const NAU8325_PDM_LCH_EDGE_SFT: c_int = 1;

pub const NAU8325_PDM_MODE_EN: c_uint = 0x1;
// TDM_CTRL (0x0c)
pub const NAU8325_TDM_SFT: c_int = 15;

pub const NAU8325_PCM_OFFSET_CTRL_SFT: c_int = 14;

pub const NAU8325_DAC_LEFT_SFT: c_int = 6;

pub const NAU8325_DAC_RIGHT_SFT: c_int = 3;

// I2S_PCM_CTRL1 (0x0d)
pub const NAU8325_DACCM_CTL_SFT: c_int = 14;

pub const NAU8325_CMB8_0_SFT: c_int = 10;

pub const NAU8325_UA_OFFSET_SFT: c_int = 9;

pub const NAU8325_I2S_BP_SFT: c_int = 7;

pub const NAU8325_I2S_PCMB_SFT: c_int = 6;

pub const NAU8325_I2S_DACPSHS0_SFT: c_int = 5;

pub const NAU8325_I2S_DL_SFT: c_int = 2;

pub const NAU8325_I2S_DF_MASK: c_uint = 0x3;
pub const NAU8325_I2S_DF_RIGTH: c_uint = 0x0;
pub const NAU8325_I2S_DF_LEFT: c_uint = 0x1;
pub const NAU8325_I2S_DF_I2S: c_uint = 0x2;
pub const NAU8325_I2S_DF_PCM_AB: c_uint = 0x3;
// I2S_PCM_CTRL2 (0x0e)
pub const NAU8325_PCM_TS_SFT: c_int = 10;

pub const NAU8325_PCM8BIT0_SFT: c_int = 8;

// L_TIME_SLOT (0x0f)
pub const NAU8325_SHORT_FS_DET_SFT: c_int = 13;

pub const NAU8325_TSLOT_L0_MASK: c_uint = 0x3ff;
// R_TIME_SLOT (0x10)
pub const NAU8325_TSLOT_R0_MASK: c_uint = 0x3ff;
// HPF_CTRL (0x11)
pub const NAU8325_DAC_HPF_SFT: c_int = 15;

pub const NAU8325_DAC_HPF_APP_SFT: c_int = 14;

pub const NAU8325_DAC_HPF_FCUT_SFT: c_int = 11;

// MUTE_CTRL (0x12)
pub const NAU8325_SOFT_MUTE_SFT: c_int = 15;

pub const NAU8325_DAC_ZC_SFT: c_int = 8;

pub const NAU8325_UNMUTE_CTL_SFT: c_int = 6;

pub const NAU8325_ANA_MUTE_SFT: c_int = 4;

pub const NAU8325_AUTO_MUTE_SFT: c_int = 3;

// DAC_VOLUME (0x13)
pub const NAU8325_DAC_VOLUME_L_SFT: c_int = 8;

pub const NAU8325_DAC_VOLUME_R_SFT: c_int = 0;

pub const NAU8325_DAC_VOL_MAX: c_uint = 0xff;
// DEBUG_READ1 (0x1d)

pub const NAU8325_POWERDOWN1B_D_MASK: c_uint = 0x1;
// DEBUG_READ2 (0x1f)
pub const NAU8325_R_CHANNEL_Vol_SFT: c_int = 8;

pub const NAU8325_L_CHANNEL_Vol_MASK: c_uint = 0xff;
// DEBUG_READ3(0x22)

pub const NAU8325_TMDET_MASK: c_uint = 0x1;
// DAC_CTRL1 (0x29)
pub const NAU8325_DAC_OVERSAMPLE_SFT: c_int = 0;
pub const NAU8325_DAC_OVERSAMPLE_MASK: c_uint = 0x7;
pub const NAU8325_DAC_OVERSAMPLE_256: c_int = 1;
pub const NAU8325_DAC_OVERSAMPLE_128: c_int = 2;
pub const NAU8325_DAC_OVERSAMPLE_64: c_int = 0;
pub const NAU8325_DAC_OVERSAMPLE_32: c_int = 4;
// ALC_CTRL1 (0x2c)
pub const NAU8325_ALC_MAXGAIN_SFT: c_int = 5;
pub const NAU8325_ALC_MAXGAIN_MAX: c_uint = 0x7;

pub const NAU8325_ALC_MINGAIN_MAX: c_int = 4;
pub const NAU8325_ALC_MINGAIN_SFT: c_int = 1;

// ALC_CTRL2 (0x2d)
pub const NAU8325_ALC_DCY_SFT: c_int = 12;
pub const NAU8325_ALC_DCY_MAX: c_uint = 0xb;

pub const NAU8325_ALC_ATK_SFT: c_int = 8;
pub const NAU8325_ALC_ATK_MAX: c_uint = 0xb;

pub const NAU8325_ALC_HLD_SFT: c_int = 4;
pub const NAU8325_ALC_HLD_MAX: c_uint = 0xa;

pub const NAU8325_ALC_LVL_SFT: c_int = 0;
pub const NAU8325_ALC_LVL_MAX: c_uint = 0xf;
pub const NAU8325_ALC_LVL_MASK: c_uint = 0xf;
// ALC_CTRL3 (0x2e)
pub const NAU8325_ALC_EN_SFT: c_int = 15;

// TEMP_COMP_CTRL (0x30)
pub const NAU8325_TEMP_COMP_ACT2_MASK: c_uint = 0xff;
// LPF_CTRL (0x33)
pub const NAU8325_LPF_IN1_EN_SFT: c_int = 15;

pub const NAU8325_LPF_IN1_TC_SFT: c_int = 11;

pub const NAU8325_LPF_IN2_EN_SFT: c_int = 10;

pub const NAU8325_LPF_IN2_TC_SFT: c_int = 6;

// CLK_DET_CTRL (0x40)
pub const NAU8325_APWRUP_SFT: c_int = 15;

pub const NAU8325_CLKPWRUP_SFT: c_int = 14;

pub const NAU8325_PWRUP_DFT_SFT: c_int = 13;

pub const NAU8325_REG_SRATE_SFT: c_int = 10;

pub const NAU8325_REG_ALT_SRATE_SFT: c_int = 9;

pub const NAU8325_REG_DIV_MAX: c_uint = 0x1;
// BIAS_ADJ (0x60)
pub const NAU8325_BIAS_VMID_SEL_SFT: c_int = 4;

// ANALOG_CONTROL_1 (0x61)
pub const NAU8325_VMDFSTENB_SFT: c_int = 14;

pub const NAU8325_CLASSDEN_SFT: c_int = 12;

pub const NAU8325_DACCLKEN_R_SFT: c_int = 10;

pub const NAU8325_DACEN_R_SFT: c_int = 8;

pub const NAU8325_DACCLKEN_SFT: c_int = 6;

pub const NAU8325_DACEN_SFT: c_int = 4;

pub const NAU8325_BIASEN_SFT: c_int = 2;

pub const NAU8325_VMIDEN_MASK: c_uint = 0x3;
// ANALOG_CONTROL_2 (0x62)
pub const NAU8325_PWMMOD_SFT: c_int = 14;

pub const NAU8325_DACTEST_SFT: c_int = 6;

pub const NAU8325_DACREFCAP_SFT: c_int = 4;

// ANALOG_CONTROL_3 (0x63)
pub const NAU8325_POWER_DOWN_L_SFT: c_int = 12;

pub const NAU8325_POWER_DOWN_R_SFT: c_int = 11;

pub const NAU8325_CLASSD_FINE_SFT: c_int = 5;

pub const NAU8325_CLASSD_COARSE_GAIN_MASK: c_uint = 0xf;
// ANALOG_CONTROL_4 (0x64)
pub const NAU8325_CLASSD_OCPN_SFT: c_int = 12;

pub const NAU8325_CLASSD_OCPP_SFT: c_int = 8;

pub const NAU8325_CLASSD_SLEWN_MASK: c_uint = 0xff;
// ANALOG_CONTROL_5 (0x65)
pub const NAU8325_MCLK_RANGE_SFT: c_int = 2;

pub const NAU8325_MCLK8XEN_SFT: c_int = 1;

pub const NAU8325_MCLK4XEN_EN: c_uint = 0x1;
// ANALOG_CONTROL_6 (0x66)
pub const NAU8325_VBATLOW_SFT: c_int = 4;

pub const NAU8325_VDDSPK_LIM_SFT: c_int = 3;

pub const NAU8325_VDDSPK_LIM_MASK: c_uint = 0x7;
// CLIP_CTRL (0x69)
pub const NAU8325_ANTI_CLIP_SFT: c_int = 4;

// RDAC (0x73)
pub const NAU8325_CLK_DAC_DELAY_SFT: c_int = 4;

pub const NAU8325_DACVREFSEL_SFT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8325 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mclk: c_int,
    pub fs: c_int,
    pub vref_impedance_ohms: c_int,
    pub dac_vref_microvolt: c_int,
    pub clock_detection: c_int,
    pub clock_det_data: c_int,
    pub alc_enable: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8325_src_attr {
    pub param: c_int,
    pub val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8325_srate_attr {
    pub fs: c_int,
    pub range: c_int,
    pub max: bool,
    pub mclk_src: [c_uint; NAU8325_MCLK_FS_RATIO_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8325_osr_attr {
    pub osr: c_uint,
    pub clk_src: c_uint,
}
