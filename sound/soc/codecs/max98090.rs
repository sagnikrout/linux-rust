//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98090.h
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
// max98090.h -- MAX98090 ALSA SoC Audio driver
//
// Copyright 2011-2012 Maxim Integrated Products
//
// The default operating frequency for a DMIC attached to the codec.
// This can be overridden by a device tree property.
//
pub const MAX98090_DEFAULT_DMIC_FREQ: c_int = 2500000;
//
// MAX98090 Register Definitions
//
pub const M98090_REG_SOFTWARE_RESET: c_uint = 0x00;
pub const M98090_REG_DEVICE_STATUS: c_uint = 0x01;
pub const M98090_REG_JACK_STATUS: c_uint = 0x02;
pub const M98090_REG_INTERRUPT_S: c_uint = 0x03;
pub const M98090_REG_QUICK_SYSTEM_CLOCK: c_uint = 0x04;
pub const M98090_REG_QUICK_SAMPLE_RATE: c_uint = 0x05;
pub const M98090_REG_DAI_INTERFACE: c_uint = 0x06;
pub const M98090_REG_DAC_PATH: c_uint = 0x07;
pub const M98090_REG_MIC_DIRECT_TO_ADC: c_uint = 0x08;
pub const M98090_REG_LINE_TO_ADC: c_uint = 0x09;
pub const M98090_REG_ANALOG_MIC_LOOP: c_uint = 0x0A;
pub const M98090_REG_ANALOG_LINE_LOOP: c_uint = 0x0B;
pub const M98090_REG_RESERVED: c_uint = 0x0C;
pub const M98090_REG_LINE_INPUT_CONFIG: c_uint = 0x0D;
pub const M98090_REG_LINE_INPUT_LEVEL: c_uint = 0x0E;
pub const M98090_REG_INPUT_MODE: c_uint = 0x0F;
pub const M98090_REG_MIC1_INPUT_LEVEL: c_uint = 0x10;
pub const M98090_REG_MIC2_INPUT_LEVEL: c_uint = 0x11;
pub const M98090_REG_MIC_BIAS_VOLTAGE: c_uint = 0x12;
pub const M98090_REG_DIGITAL_MIC_ENABLE: c_uint = 0x13;
pub const M98090_REG_DIGITAL_MIC_CONFIG: c_uint = 0x14;
pub const M98090_REG_LEFT_ADC_MIXER: c_uint = 0x15;
pub const M98090_REG_RIGHT_ADC_MIXER: c_uint = 0x16;
pub const M98090_REG_LEFT_ADC_LEVEL: c_uint = 0x17;
pub const M98090_REG_RIGHT_ADC_LEVEL: c_uint = 0x18;
pub const M98090_REG_ADC_BIQUAD_LEVEL: c_uint = 0x19;
pub const M98090_REG_ADC_SIDETONE: c_uint = 0x1A;
pub const M98090_REG_SYSTEM_CLOCK: c_uint = 0x1B;
pub const M98090_REG_CLOCK_MODE: c_uint = 0x1C;
pub const M98090_REG_CLOCK_RATIO_NI_MSB: c_uint = 0x1D;
pub const M98090_REG_CLOCK_RATIO_NI_LSB: c_uint = 0x1E;
pub const M98090_REG_CLOCK_RATIO_MI_MSB: c_uint = 0x1F;
pub const M98090_REG_CLOCK_RATIO_MI_LSB: c_uint = 0x20;
pub const M98090_REG_MASTER_MODE: c_uint = 0x21;
pub const M98090_REG_INTERFACE_FORMAT: c_uint = 0x22;
pub const M98090_REG_TDM_CONTROL: c_uint = 0x23;
pub const M98090_REG_TDM_FORMAT: c_uint = 0x24;
pub const M98090_REG_IO_CONFIGURATION: c_uint = 0x25;
pub const M98090_REG_FILTER_CONFIG: c_uint = 0x26;
pub const M98090_REG_DAI_PLAYBACK_LEVEL: c_uint = 0x27;
pub const M98090_REG_DAI_PLAYBACK_LEVEL_EQ: c_uint = 0x28;
pub const M98090_REG_LEFT_HP_MIXER: c_uint = 0x29;
pub const M98090_REG_RIGHT_HP_MIXER: c_uint = 0x2A;
pub const M98090_REG_HP_CONTROL: c_uint = 0x2B;
pub const M98090_REG_LEFT_HP_VOLUME: c_uint = 0x2C;
pub const M98090_REG_RIGHT_HP_VOLUME: c_uint = 0x2D;
pub const M98090_REG_LEFT_SPK_MIXER: c_uint = 0x2E;
pub const M98090_REG_RIGHT_SPK_MIXER: c_uint = 0x2F;
pub const M98090_REG_SPK_CONTROL: c_uint = 0x30;
pub const M98090_REG_LEFT_SPK_VOLUME: c_uint = 0x31;
pub const M98090_REG_RIGHT_SPK_VOLUME: c_uint = 0x32;
pub const M98090_REG_DRC_TIMING: c_uint = 0x33;
pub const M98090_REG_DRC_COMPRESSOR: c_uint = 0x34;
pub const M98090_REG_DRC_EXPANDER: c_uint = 0x35;
pub const M98090_REG_DRC_GAIN: c_uint = 0x36;
pub const M98090_REG_RCV_LOUTL_MIXER: c_uint = 0x37;
pub const M98090_REG_RCV_LOUTL_CONTROL: c_uint = 0x38;
pub const M98090_REG_RCV_LOUTL_VOLUME: c_uint = 0x39;
pub const M98090_REG_LOUTR_MIXER: c_uint = 0x3A;
pub const M98090_REG_LOUTR_CONTROL: c_uint = 0x3B;
pub const M98090_REG_LOUTR_VOLUME: c_uint = 0x3C;
pub const M98090_REG_JACK_DETECT: c_uint = 0x3D;
pub const M98090_REG_INPUT_ENABLE: c_uint = 0x3E;
pub const M98090_REG_OUTPUT_ENABLE: c_uint = 0x3F;
pub const M98090_REG_LEVEL_CONTROL: c_uint = 0x40;
pub const M98090_REG_DSP_FILTER_ENABLE: c_uint = 0x41;
pub const M98090_REG_BIAS_CONTROL: c_uint = 0x42;
pub const M98090_REG_DAC_CONTROL: c_uint = 0x43;
pub const M98090_REG_ADC_CONTROL: c_uint = 0x44;
pub const M98090_REG_DEVICE_SHUTDOWN: c_uint = 0x45;
pub const M98090_REG_EQUALIZER_BASE: c_uint = 0x46;
pub const M98090_REG_RECORD_BIQUAD_BASE: c_uint = 0xAF;
pub const M98090_REG_DMIC3_VOLUME: c_uint = 0xBE;
pub const M98090_REG_DMIC4_VOLUME: c_uint = 0xBF;
pub const M98090_REG_DMIC34_BQ_PREATTEN: c_uint = 0xC0;
pub const M98090_REG_RECORD_TDM_SLOT: c_uint = 0xC1;
pub const M98090_REG_SAMPLE_RATE: c_uint = 0xC2;
pub const M98090_REG_DMIC34_BIQUAD_BASE: c_uint = 0xC3;
pub const M98090_REG_REVISION_ID: c_uint = 0xFF;

pub const MAX98090_MAX_REGISTER: c_uint = 0xFF;
// MAX98090 Register Bit Fields
//
// M98090_REG_SOFTWARE_RESET
//

pub const M98090_SWRESET_SHIFT: c_int = 7;
pub const M98090_SWRESET_WIDTH: c_int = 1;
//
// M98090_REG_DEVICE_STATUS
//

pub const M98090_CLD_SHIFT: c_int = 7;
pub const M98090_CLD_WIDTH: c_int = 1;

pub const M98090_SLD_SHIFT: c_int = 6;
pub const M98090_SLD_WIDTH: c_int = 1;

pub const M98090_ULK_SHIFT: c_int = 5;
pub const M98090_ULK_WIDTH: c_int = 1;

pub const M98090_JDET_SHIFT: c_int = 2;
pub const M98090_JDET_WIDTH: c_int = 1;

pub const M98090_DRCACT_SHIFT: c_int = 1;
pub const M98090_DRCACT_WIDTH: c_int = 1;

pub const M98090_DRCCLP_SHIFT: c_int = 0;
pub const M98090_DRCCLP_WIDTH: c_int = 1;
//
// M98090_REG_JACK_STATUS
//

pub const M98090_LSNS_SHIFT: c_int = 2;
pub const M98090_LSNS_WIDTH: c_int = 1;

pub const M98090_JKSNS_SHIFT: c_int = 1;
pub const M98090_JKSNS_WIDTH: c_int = 1;
//
// M98090_REG_INTERRUPT_S
//

pub const M98090_ICLD_SHIFT: c_int = 7;
pub const M98090_ICLD_WIDTH: c_int = 1;

pub const M98090_ISLD_SHIFT: c_int = 6;
pub const M98090_ISLD_WIDTH: c_int = 1;

pub const M98090_IULK_SHIFT: c_int = 5;
pub const M98090_IULK_WIDTH: c_int = 1;

pub const M98090_IJDET_SHIFT: c_int = 2;
pub const M98090_IJDET_WIDTH: c_int = 1;

pub const M98090_IDRCACT_SHIFT: c_int = 1;
pub const M98090_IDRCACT_WIDTH: c_int = 1;

pub const M98090_IDRCCLP_SHIFT: c_int = 0;
pub const M98090_IDRCCLP_WIDTH: c_int = 1;
//
// M98090_REG_QUICK_SYSTEM_CLOCK
//

pub const M98090_26M_SHIFT: c_int = 7;
pub const M98090_26M_WIDTH: c_int = 1;

pub const M98090_19P2M_SHIFT: c_int = 6;
pub const M98090_19P2M_WIDTH: c_int = 1;

pub const M98090_13M_SHIFT: c_int = 5;
pub const M98090_13M_WIDTH: c_int = 1;

pub const M98090_12P288M_SHIFT: c_int = 4;
pub const M98090_12P288M_WIDTH: c_int = 1;

pub const M98090_12M_SHIFT: c_int = 3;
pub const M98090_12M_WIDTH: c_int = 1;

pub const M98090_11P2896M_SHIFT: c_int = 2;
pub const M98090_11P2896M_WIDTH: c_int = 1;

pub const M98090_256FS_SHIFT: c_int = 0;
pub const M98090_256FS_WIDTH: c_int = 1;
pub const M98090_CLK_ALL_SHIFT: c_int = 0;
pub const M98090_CLK_ALL_WIDTH: c_int = 8;

//
// M98090_REG_QUICK_SAMPLE_RATE
//

pub const M98090_SR_96K_SHIFT: c_int = 5;
pub const M98090_SR_96K_WIDTH: c_int = 1;

pub const M98090_SR_32K_SHIFT: c_int = 4;
pub const M98090_SR_32K_WIDTH: c_int = 1;

pub const M98090_SR_48K_SHIFT: c_int = 3;
pub const M98090_SR_48K_WIDTH: c_int = 1;

pub const M98090_SR_44K1_SHIFT: c_int = 2;
pub const M98090_SR_44K1_WIDTH: c_int = 1;

pub const M98090_SR_16K_SHIFT: c_int = 1;
pub const M98090_SR_16K_WIDTH: c_int = 1;

pub const M98090_SR_8K_SHIFT: c_int = 0;
pub const M98090_SR_8K_WIDTH: c_int = 1;
pub const M98090_SR_MASK: c_uint = 0x3F;
pub const M98090_SR_ALL_SHIFT: c_int = 0;
pub const M98090_SR_ALL_WIDTH: c_int = 8;

//
// M98090_REG_DAI_INTERFACE
//

pub const M98090_RJ_M_SHIFT: c_int = 5;
pub const M98090_RJ_M_WIDTH: c_int = 1;

pub const M98090_RJ_S_SHIFT: c_int = 4;
pub const M98090_RJ_S_WIDTH: c_int = 1;

pub const M98090_LJ_M_SHIFT: c_int = 3;
pub const M98090_LJ_M_WIDTH: c_int = 1;

pub const M98090_LJ_S_SHIFT: c_int = 2;
pub const M98090_LJ_S_WIDTH: c_int = 1;

pub const M98090_I2S_M_SHIFT: c_int = 1;
pub const M98090_I2S_M_WIDTH: c_int = 1;

pub const M98090_I2S_S_SHIFT: c_int = 0;
pub const M98090_I2S_S_WIDTH: c_int = 1;
pub const M98090_DAI_ALL_SHIFT: c_int = 0;
pub const M98090_DAI_ALL_WIDTH: c_int = 8;

//
// M98090_REG_DAC_PATH
//

pub const M98090_DIG2_HP_SHIFT: c_int = 7;
pub const M98090_DIG2_HP_WIDTH: c_int = 1;

pub const M98090_DIG2_EAR_SHIFT: c_int = 6;
pub const M98090_DIG2_EAR_WIDTH: c_int = 1;

pub const M98090_DIG2_SPK_SHIFT: c_int = 5;
pub const M98090_DIG2_SPK_WIDTH: c_int = 1;

pub const M98090_DIG2_LOUT_SHIFT: c_int = 4;
pub const M98090_DIG2_LOUT_WIDTH: c_int = 1;
pub const M98090_DIG2_ALL_SHIFT: c_int = 0;
pub const M98090_DIG2_ALL_WIDTH: c_int = 8;

//
// M98090_REG_MIC_DIRECT_TO_ADC
//

pub const M98090_IN12_MIC1_SHIFT: c_int = 7;
pub const M98090_IN12_MIC1_WIDTH: c_int = 1;

pub const M98090_IN34_MIC2_SHIFT: c_int = 6;
pub const M98090_IN34_MIC2_WIDTH: c_int = 1;

pub const M98090_IN56_MIC1_SHIFT: c_int = 5;
pub const M98090_IN56_MIC1_WIDTH: c_int = 1;

pub const M98090_IN56_MIC2_SHIFT: c_int = 4;
pub const M98090_IN56_MIC2_WIDTH: c_int = 1;

pub const M98090_IN12_DADC_SHIFT: c_int = 3;
pub const M98090_IN12_DADC_WIDTH: c_int = 1;

pub const M98090_IN34_DADC_SHIFT: c_int = 2;
pub const M98090_IN34_DADC_WIDTH: c_int = 1;

pub const M98090_IN56_DADC_SHIFT: c_int = 1;
pub const M98090_IN56_DADC_WIDTH: c_int = 1;
pub const M98090_MIC_ALL_SHIFT: c_int = 0;
pub const M98090_MIC_ALL_WIDTH: c_int = 8;

//
// M98090_REG_LINE_TO_ADC
//

pub const M98090_IN12S_AB_SHIFT: c_int = 7;
pub const M98090_IN12S_AB_WIDTH: c_int = 1;

pub const M98090_IN34S_AB_SHIFT: c_int = 6;
pub const M98090_IN34S_AB_WIDTH: c_int = 1;

pub const M98090_IN56S_AB_SHIFT: c_int = 5;
pub const M98090_IN56S_AB_WIDTH: c_int = 1;

pub const M98090_IN34D_A_SHIFT: c_int = 4;
pub const M98090_IN34D_A_WIDTH: c_int = 1;

pub const M98090_IN56D_B_SHIFT: c_int = 3;
pub const M98090_IN56D_B_WIDTH: c_int = 1;
pub const M98090_LINE_ALL_SHIFT: c_int = 0;
pub const M98090_LINE_ALL_WIDTH: c_int = 8;

//
// M98090_REG_ANALOG_MIC_LOOP
//

pub const M98090_IN12_M1HPL_SHIFT: c_int = 7;
pub const M98090_IN12_M1HPL_WIDTH: c_int = 1;

pub const M98090_IN12_M1SPKL_SHIFT: c_int = 6;
pub const M98090_IN12_M1SPKL_WIDTH: c_int = 1;

pub const M98090_IN12_M1EAR_SHIFT: c_int = 5;
pub const M98090_IN12_M1EAR_WIDTH: c_int = 1;

pub const M98090_IN12_M1LOUTL_SHIFT: c_int = 4;
pub const M98090_IN12_M1LOUTL_WIDTH: c_int = 1;

pub const M98090_IN34_M2HPR_SHIFT: c_int = 3;
pub const M98090_IN34_M2HPR_WIDTH: c_int = 1;

pub const M98090_IN34_M2SPKR_SHIFT: c_int = 2;
pub const M98090_IN34_M2SPKR_WIDTH: c_int = 1;

pub const M98090_IN34_M2EAR_SHIFT: c_int = 1;
pub const M98090_IN34_M2EAR_WIDTH: c_int = 1;

pub const M98090_IN34_M2LOUTR_SHIFT: c_int = 0;
pub const M98090_IN34_M2LOUTR_WIDTH: c_int = 1;
pub const M98090_AMIC_ALL_SHIFT: c_int = 0;
pub const M98090_AMIC_ALL_WIDTH: c_int = 8;

//
// M98090_REG_ANALOG_LINE_LOOP
//

pub const M98090_IN12S_ABHP_SHIFT: c_int = 7;
pub const M98090_IN12S_ABHP_WIDTH: c_int = 1;

pub const M98090_IN34D_ASPKL_SHIFT: c_int = 6;
pub const M98090_IN34D_ASPKL_WIDTH: c_int = 1;

pub const M98090_IN34D_AEAR_SHIFT: c_int = 5;
pub const M98090_IN34D_AEAR_WIDTH: c_int = 1;

pub const M98090_IN12S_ABLOUT_SHIFT: c_int = 4;
pub const M98090_IN12S_ABLOUT_WIDTH: c_int = 1;

pub const M98090_IN34S_ABHP_SHIFT: c_int = 3;
pub const M98090_IN34S_ABHP_WIDTH: c_int = 1;

pub const M98090_IN56D_BSPKR_SHIFT: c_int = 2;
pub const M98090_IN56D_BSPKR_WIDTH: c_int = 1;

pub const M98090_IN56D_BEAR_SHIFT: c_int = 1;
pub const M98090_IN56D_BEAR_WIDTH: c_int = 1;

pub const M98090_IN34S_ABLOUT_SHIFT: c_int = 0;
pub const M98090_IN34S_ABLOUT_WIDTH: c_int = 1;
pub const M98090_ALIN_ALL_SHIFT: c_int = 0;
pub const M98090_ALIN_ALL_WIDTH: c_int = 8;

//
// M98090_REG_RESERVED
//
// M98090_REG_LINE_INPUT_CONFIG
//

pub const M98090_IN34DIFF_SHIFT: c_int = 7;
pub const M98090_IN34DIFF_WIDTH: c_int = 1;

pub const M98090_IN56DIFF_SHIFT: c_int = 6;
pub const M98090_IN56DIFF_WIDTH: c_int = 1;

pub const M98090_IN1SEEN_SHIFT: c_int = 5;
pub const M98090_IN1SEEN_WIDTH: c_int = 1;

pub const M98090_IN2SEEN_SHIFT: c_int = 4;
pub const M98090_IN2SEEN_WIDTH: c_int = 1;

pub const M98090_IN3SEEN_SHIFT: c_int = 3;
pub const M98090_IN3SEEN_WIDTH: c_int = 1;

pub const M98090_IN4SEEN_SHIFT: c_int = 2;
pub const M98090_IN4SEEN_WIDTH: c_int = 1;

pub const M98090_IN5SEEN_SHIFT: c_int = 1;
pub const M98090_IN5SEEN_WIDTH: c_int = 1;

pub const M98090_IN6SEEN_SHIFT: c_int = 0;
pub const M98090_IN6SEEN_WIDTH: c_int = 1;
//
// M98090_REG_LINE_INPUT_LEVEL
//

pub const M98090_MIXG135_SHIFT: c_int = 7;
pub const M98090_MIXG135_WIDTH: c_int = 1;

pub const M98090_MIXG246_SHIFT: c_int = 6;
pub const M98090_MIXG246_WIDTH: c_int = 1;

pub const M98090_LINAPGA_SHIFT: c_int = 3;
pub const M98090_LINAPGA_WIDTH: c_int = 3;
pub const M98090_LINAPGA_NUM: c_int = 6;

pub const M98090_LINBPGA_SHIFT: c_int = 0;
pub const M98090_LINBPGA_WIDTH: c_int = 3;
pub const M98090_LINBPGA_NUM: c_int = 6;
//
// M98090_REG_INPUT_MODE
//

pub const M98090_EXTBUFA_SHIFT: c_int = 7;
pub const M98090_EXTBUFA_WIDTH: c_int = 1;

pub const M98090_EXTBUFB_SHIFT: c_int = 6;
pub const M98090_EXTBUFB_WIDTH: c_int = 1;

pub const M98090_EXTMIC_SHIFT: c_int = 0;
pub const M98090_EXTMIC1_SHIFT: c_int = 0;
pub const M98090_EXTMIC2_SHIFT: c_int = 1;
pub const M98090_EXTMIC_WIDTH: c_int = 2;

//
// M98090_REG_MIC1_INPUT_LEVEL
//

pub const M98090_MIC_PA1EN_SHIFT: c_int = 5;
pub const M98090_MIC_PA1EN_WIDTH: c_int = 2;
pub const M98090_MIC_PA1EN_NUM: c_int = 3;

pub const M98090_MIC_PGAM1_SHIFT: c_int = 0;
pub const M98090_MIC_PGAM1_WIDTH: c_int = 5;
pub const M98090_MIC_PGAM1_NUM: c_int = 21;
//
// M98090_REG_MIC2_INPUT_LEVEL
//

pub const M98090_MIC_PA2EN_SHIFT: c_int = 5;
pub const M98090_MIC_PA2EN_WIDTH: c_int = 2;
pub const M98090_MIC_PA2EN_NUM: c_int = 3;

pub const M98090_MIC_PGAM2_SHIFT: c_int = 0;
pub const M98090_MIC_PGAM2_WIDTH: c_int = 5;
pub const M98090_MIC_PGAM2_NUM: c_int = 21;
//
// M98090_REG_MIC_BIAS_VOLTAGE
//

pub const M98090_MBVSEL_SHIFT: c_int = 0;
pub const M98090_MBVSEL_WIDTH: c_int = 2;

//
// M98090_REG_DIGITAL_MIC_ENABLE
//

pub const M98090_MICCLK_SHIFT: c_int = 4;
pub const M98090_MICCLK_WIDTH: c_int = 3;

pub const M98090_DIGMIC4_SHIFT: c_int = 3;
pub const M98090_DIGMIC4_WIDTH: c_int = 1;

pub const M98090_DIGMIC3_SHIFT: c_int = 2;
pub const M98090_DIGMIC3_WIDTH: c_int = 1;

pub const M98090_DIGMICR_SHIFT: c_int = 1;
pub const M98090_DIGMICR_WIDTH: c_int = 1;

pub const M98090_DIGMICL_SHIFT: c_int = 0;
pub const M98090_DIGMICL_WIDTH: c_int = 1;

//
// M98090_REG_DIGITAL_MIC_CONFIG
//

pub const M98090_DMIC_COMP_SHIFT: c_int = 4;
pub const M98090_DMIC_COMP_WIDTH: c_int = 4;

pub const M98090_DMIC_FREQ_SHIFT: c_int = 0;
pub const M98090_DMIC_FREQ_WIDTH: c_int = 2;
//
// M98090_REG_LEFT_ADC_MIXER
//

pub const M98090_MIXADL_MIC2_SHIFT: c_int = 6;
pub const M98090_MIXADL_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXADL_MIC1_SHIFT: c_int = 5;
pub const M98090_MIXADL_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXADL_LINEB_SHIFT: c_int = 4;
pub const M98090_MIXADL_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXADL_LINEA_SHIFT: c_int = 3;
pub const M98090_MIXADL_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXADL_IN65DIFF_SHIFT: c_int = 2;
pub const M98090_MIXADL_IN65DIFF_WIDTH: c_int = 1;

pub const M98090_MIXADL_IN34DIFF_SHIFT: c_int = 1;
pub const M98090_MIXADL_IN34DIFF_WIDTH: c_int = 1;

pub const M98090_MIXADL_IN12DIFF_SHIFT: c_int = 0;
pub const M98090_MIXADL_IN12DIFF_WIDTH: c_int = 1;

pub const M98090_MIXADL_SHIFT: c_int = 0;
pub const M98090_MIXADL_WIDTH: c_int = 8;
//
// M98090_REG_RIGHT_ADC_MIXER
//

pub const M98090_MIXADR_MIC2_SHIFT: c_int = 6;
pub const M98090_MIXADR_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXADR_MIC1_SHIFT: c_int = 5;
pub const M98090_MIXADR_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXADR_LINEB_SHIFT: c_int = 4;
pub const M98090_MIXADR_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXADR_LINEA_SHIFT: c_int = 3;
pub const M98090_MIXADR_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXADR_IN65DIFF_SHIFT: c_int = 2;
pub const M98090_MIXADR_IN65DIFF_WIDTH: c_int = 1;

pub const M98090_MIXADR_IN34DIFF_SHIFT: c_int = 1;
pub const M98090_MIXADR_IN34DIFF_WIDTH: c_int = 1;

pub const M98090_MIXADR_IN12DIFF_SHIFT: c_int = 0;
pub const M98090_MIXADR_IN12DIFF_WIDTH: c_int = 1;

pub const M98090_MIXADR_SHIFT: c_int = 0;
pub const M98090_MIXADR_WIDTH: c_int = 8;
//
// M98090_REG_LEFT_ADC_LEVEL
//

pub const M98090_AVLG_SHIFT: c_int = 4;
pub const M98090_AVLG_WIDTH: c_int = 3;

pub const M98090_AVL_SHIFT: c_int = 0;
pub const M98090_AVL_WIDTH: c_int = 4;

//
// M98090_REG_RIGHT_ADC_LEVEL
//

pub const M98090_AVRG_SHIFT: c_int = 4;
pub const M98090_AVRG_WIDTH: c_int = 3;

pub const M98090_AVR_SHIFT: c_int = 0;
pub const M98090_AVR_WIDTH: c_int = 4;

//
// M98090_REG_ADC_BIQUAD_LEVEL
//

pub const M98090_AVBQ_SHIFT: c_int = 0;
pub const M98090_AVBQ_WIDTH: c_int = 4;

//
// M98090_REG_ADC_SIDETONE
//

pub const M98090_DSTSR_SHIFT: c_int = 7;
pub const M98090_DSTSR_WIDTH: c_int = 1;

pub const M98090_DSTSL_SHIFT: c_int = 6;
pub const M98090_DSTSL_WIDTH: c_int = 1;

pub const M98090_DVST_SHIFT: c_int = 0;
pub const M98090_DVST_WIDTH: c_int = 5;
pub const M98090_DVST_NUM: c_int = 31;
//
// M98090_REG_SYSTEM_CLOCK
//

pub const M98090_PSCLK_SHIFT: c_int = 4;
pub const M98090_PSCLK_WIDTH: c_int = 2;

//
// M98090_REG_CLOCK_MODE
//

pub const M98090_FREQ_SHIFT: c_int = 4;
pub const M98090_FREQ_WIDTH: c_int = 4;

pub const M98090_USE_M1_SHIFT: c_int = 0;
pub const M98090_USE_M1_WIDTH: c_int = 1;

//
// M98090_REG_CLOCK_RATIO_NI_MSB
//

pub const M98090_NI_HI_SHIFT: c_int = 0;
pub const M98090_NI_HI_WIDTH: c_int = 7;

//
// M98090_REG_CLOCK_RATIO_NI_LSB
//

pub const M98090_NI_LO_SHIFT: c_int = 0;
pub const M98090_NI_LO_WIDTH: c_int = 8;

//
// M98090_REG_CLOCK_RATIO_MI_MSB
//

pub const M98090_MI_HI_SHIFT: c_int = 0;
pub const M98090_MI_HI_WIDTH: c_int = 8;

//
// M98090_REG_CLOCK_RATIO_MI_LSB
//

pub const M98090_MI_LO_SHIFT: c_int = 0;
pub const M98090_MI_LO_WIDTH: c_int = 8;

//
// M98090_REG_MASTER_MODE
//

pub const M98090_MAS_SHIFT: c_int = 7;
pub const M98090_MAS_WIDTH: c_int = 1;

pub const M98090_BSEL_SHIFT: c_int = 0;
pub const M98090_BSEL_WIDTH: c_int = 1;

//
// M98090_REG_INTERFACE_FORMAT
//

pub const M98090_RJ_SHIFT: c_int = 5;
pub const M98090_RJ_WIDTH: c_int = 1;

pub const M98090_WCI_SHIFT: c_int = 4;
pub const M98090_WCI_WIDTH: c_int = 1;

pub const M98090_BCI_SHIFT: c_int = 3;
pub const M98090_BCI_WIDTH: c_int = 1;

pub const M98090_DLY_SHIFT: c_int = 2;
pub const M98090_DLY_WIDTH: c_int = 1;

pub const M98090_WS_SHIFT: c_int = 0;
pub const M98090_WS_WIDTH: c_int = 2;

//
// M98090_REG_TDM_CONTROL
//

pub const M98090_FSW_SHIFT: c_int = 1;
pub const M98090_FSW_WIDTH: c_int = 1;

pub const M98090_TDM_SHIFT: c_int = 0;
pub const M98090_TDM_WIDTH: c_int = 1;

//
// M98090_REG_TDM_FORMAT
//

pub const M98090_TDM_SLOTL_SHIFT: c_int = 6;
pub const M98090_TDM_SLOTL_WIDTH: c_int = 2;

pub const M98090_TDM_SLOTR_SHIFT: c_int = 4;
pub const M98090_TDM_SLOTR_WIDTH: c_int = 2;

pub const M98090_TDM_SLOTDLY_SHIFT: c_int = 0;
pub const M98090_TDM_SLOTDLY_WIDTH: c_int = 4;

//
// M98090_REG_IO_CONFIGURATION
//

pub const M98090_LTEN_SHIFT: c_int = 5;
pub const M98090_LTEN_WIDTH: c_int = 1;

pub const M98090_LBEN_SHIFT: c_int = 4;
pub const M98090_LBEN_WIDTH: c_int = 1;

pub const M98090_DMONO_SHIFT: c_int = 3;
pub const M98090_DMONO_WIDTH: c_int = 1;

pub const M98090_HIZOFF_SHIFT: c_int = 2;
pub const M98090_HIZOFF_WIDTH: c_int = 1;

pub const M98090_SDOEN_SHIFT: c_int = 1;
pub const M98090_SDOEN_WIDTH: c_int = 1;

pub const M98090_SDIEN_SHIFT: c_int = 0;
pub const M98090_SDIEN_WIDTH: c_int = 1;

//
// M98090_REG_FILTER_CONFIG
//

pub const M98090_MODE_SHIFT: c_int = 7;
pub const M98090_MODE_WIDTH: c_int = 1;

pub const M98090_AHPF_SHIFT: c_int = 6;
pub const M98090_AHPF_WIDTH: c_int = 1;

pub const M98090_DHPF_SHIFT: c_int = 5;
pub const M98090_DHPF_WIDTH: c_int = 1;

pub const M98090_DHF_SHIFT: c_int = 4;
pub const M98090_DHF_WIDTH: c_int = 1;

pub const M98090_FLT_DMIC34MODE_SHIFT: c_int = 3;
pub const M98090_FLT_DMIC34MODE_WIDTH: c_int = 1;

pub const M98090_FLT_DMIC34HPF_SHIFT: c_int = 2;
pub const M98090_FLT_DMIC34HPF_WIDTH: c_int = 1;

//
// M98090_REG_DAI_PLAYBACK_LEVEL
//

pub const M98090_DVM_SHIFT: c_int = 7;
pub const M98090_DVM_WIDTH: c_int = 1;

pub const M98090_DVG_SHIFT: c_int = 4;
pub const M98090_DVG_WIDTH: c_int = 2;

pub const M98090_DV_SHIFT: c_int = 0;
pub const M98090_DV_WIDTH: c_int = 4;

//
// M98090_REG_DAI_PLAYBACK_LEVEL_EQ
//

pub const M98090_EQCLPN_SHIFT: c_int = 4;
pub const M98090_EQCLPN_WIDTH: c_int = 1;

pub const M98090_DVEQ_SHIFT: c_int = 0;
pub const M98090_DVEQ_WIDTH: c_int = 4;

//
// M98090_REG_LEFT_HP_MIXER
//

pub const M98090_MIXHPL_MIC2_SHIFT: c_int = 5;
pub const M98090_MIXHPL_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXHPL_MIC1_SHIFT: c_int = 4;
pub const M98090_MIXHPL_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXHPL_LINEB_SHIFT: c_int = 3;
pub const M98090_MIXHPL_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXHPL_LINEA_SHIFT: c_int = 2;
pub const M98090_MIXHPL_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXHPL_DACR_SHIFT: c_int = 1;
pub const M98090_MIXHPL_DACR_WIDTH: c_int = 1;

pub const M98090_MIXHPL_DACL_SHIFT: c_int = 0;
pub const M98090_MIXHPL_DACL_WIDTH: c_int = 1;

pub const M98090_MIXHPL_SHIFT: c_int = 0;
pub const M98090_MIXHPL_WIDTH: c_int = 6;
//
// M98090_REG_RIGHT_HP_MIXER
//

pub const M98090_MIXHPR_MIC2_SHIFT: c_int = 5;
pub const M98090_MIXHPR_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXHPR_MIC1_SHIFT: c_int = 4;
pub const M98090_MIXHPR_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXHPR_LINEB_SHIFT: c_int = 3;
pub const M98090_MIXHPR_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXHPR_LINEA_SHIFT: c_int = 2;
pub const M98090_MIXHPR_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXHPR_DACR_SHIFT: c_int = 1;
pub const M98090_MIXHPR_DACR_WIDTH: c_int = 1;

pub const M98090_MIXHPR_DACL_SHIFT: c_int = 0;
pub const M98090_MIXHPR_DACL_WIDTH: c_int = 1;

pub const M98090_MIXHPR_SHIFT: c_int = 0;
pub const M98090_MIXHPR_WIDTH: c_int = 6;
//
// M98090_REG_HP_CONTROL
//

pub const M98090_MIXHPRSEL_SHIFT: c_int = 5;
pub const M98090_MIXHPRSEL_WIDTH: c_int = 1;

pub const M98090_MIXHPLSEL_SHIFT: c_int = 4;
pub const M98090_MIXHPLSEL_WIDTH: c_int = 1;

pub const M98090_MIXHPRG_SHIFT: c_int = 2;
pub const M98090_MIXHPRG_WIDTH: c_int = 2;

pub const M98090_MIXHPLG_SHIFT: c_int = 0;
pub const M98090_MIXHPLG_WIDTH: c_int = 2;

//
// M98090_REG_LEFT_HP_VOLUME
//

pub const M98090_HPLM_SHIFT: c_int = 7;
pub const M98090_HPLM_WIDTH: c_int = 1;

pub const M98090_HPVOLL_SHIFT: c_int = 0;
pub const M98090_HPVOLL_WIDTH: c_int = 5;

//
// M98090_REG_RIGHT_HP_VOLUME
//

pub const M98090_HPRM_SHIFT: c_int = 7;
pub const M98090_HPRM_WIDTH: c_int = 1;

pub const M98090_HPVOLR_SHIFT: c_int = 0;
pub const M98090_HPVOLR_WIDTH: c_int = 5;

//
// M98090_REG_LEFT_SPK_MIXER
//

pub const M98090_MIXSPL_MIC2_SHIFT: c_int = 5;
pub const M98090_MIXSPL_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXSPL_MIC1_SHIFT: c_int = 4;
pub const M98090_MIXSPL_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXSPL_LINEB_SHIFT: c_int = 3;
pub const M98090_MIXSPL_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXSPL_LINEA_SHIFT: c_int = 2;
pub const M98090_MIXSPL_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXSPL_DACR_SHIFT: c_int = 1;
pub const M98090_MIXSPL_DACR_WIDTH: c_int = 1;

pub const M98090_MIXSPL_DACL_SHIFT: c_int = 0;
pub const M98090_MIXSPL_DACL_WIDTH: c_int = 1;

pub const M98090_MIXSPL_SHIFT: c_int = 0;
pub const M98090_MIXSPL_WIDTH: c_int = 6;

pub const M98090_MIXSPR_DACR_SHIFT: c_int = 1;
pub const M98090_MIXSPR_DACR_WIDTH: c_int = 1;
//
// M98090_REG_RIGHT_SPK_MIXER
//

pub const M98090_SPK_SLAVE_SHIFT: c_int = 6;
pub const M98090_SPK_SLAVE_WIDTH: c_int = 1;

pub const M98090_MIXSPR_MIC2_SHIFT: c_int = 5;
pub const M98090_MIXSPR_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXSPR_MIC1_SHIFT: c_int = 4;
pub const M98090_MIXSPR_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXSPR_LINEB_SHIFT: c_int = 3;
pub const M98090_MIXSPR_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXSPR_LINEA_SHIFT: c_int = 2;
pub const M98090_MIXSPR_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXSPR_DACR_SHIFT: c_int = 1;
pub const M98090_MIXSPR_DACR_WIDTH: c_int = 1;

pub const M98090_MIXSPR_DACL_SHIFT: c_int = 0;
pub const M98090_MIXSPR_DACL_WIDTH: c_int = 1;

pub const M98090_MIXSPR_SHIFT: c_int = 0;
pub const M98090_MIXSPR_WIDTH: c_int = 6;
//
// M98090_REG_SPK_CONTROL
//

pub const M98090_MIXSPRG_SHIFT: c_int = 2;
pub const M98090_MIXSPRG_WIDTH: c_int = 2;

pub const M98090_MIXSPLG_SHIFT: c_int = 0;
pub const M98090_MIXSPLG_WIDTH: c_int = 2;

//
// M98090_REG_LEFT_SPK_VOLUME
//

pub const M98090_SPLM_SHIFT: c_int = 7;
pub const M98090_SPLM_WIDTH: c_int = 1;

pub const M98090_SPVOLL_SHIFT: c_int = 0;
pub const M98090_SPVOLL_WIDTH: c_int = 6;
pub const M98090_SPVOLL_NUM: c_int = 40;
//
// M98090_REG_RIGHT_SPK_VOLUME
//

pub const M98090_SPRM_SHIFT: c_int = 7;
pub const M98090_SPRM_WIDTH: c_int = 1;

pub const M98090_SPVOLR_SHIFT: c_int = 0;
pub const M98090_SPVOLR_WIDTH: c_int = 6;
pub const M98090_SPVOLR_NUM: c_int = 40;
//
// M98090_REG_DRC_TIMING
//

pub const M98090_DRCEN_SHIFT: c_int = 7;
pub const M98090_DRCEN_WIDTH: c_int = 1;

pub const M98090_DRCRLS_SHIFT: c_int = 4;
pub const M98090_DRCRLS_WIDTH: c_int = 3;

pub const M98090_DRCATK_SHIFT: c_int = 0;
pub const M98090_DRCATK_WIDTH: c_int = 3;
//
// M98090_REG_DRC_COMPRESSOR
//

pub const M98090_DRCCMP_SHIFT: c_int = 5;
pub const M98090_DRCCMP_WIDTH: c_int = 3;

pub const M98090_DRCTHC_SHIFT: c_int = 0;
pub const M98090_DRCTHC_WIDTH: c_int = 5;

//
// M98090_REG_DRC_EXPANDER
//

pub const M98090_DRCEXP_SHIFT: c_int = 5;
pub const M98090_DRCEXP_WIDTH: c_int = 3;

pub const M98090_DRCTHE_SHIFT: c_int = 0;
pub const M98090_DRCTHE_WIDTH: c_int = 5;

//
// M98090_REG_DRC_GAIN
//

pub const M98090_DRCG_SHIFT: c_int = 0;
pub const M98090_DRCG_WIDTH: c_int = 5;
pub const M98090_DRCG_NUM: c_int = 13;
//
// M98090_REG_RCV_LOUTL_MIXER
//

pub const M98090_MIXRCVL_MIC2_SHIFT: c_int = 5;
pub const M98090_MIXRCVL_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXRCVL_MIC1_SHIFT: c_int = 4;
pub const M98090_MIXRCVL_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXRCVL_LINEB_SHIFT: c_int = 3;
pub const M98090_MIXRCVL_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXRCVL_LINEA_SHIFT: c_int = 2;
pub const M98090_MIXRCVL_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXRCVL_DACR_SHIFT: c_int = 1;
pub const M98090_MIXRCVL_DACR_WIDTH: c_int = 1;

pub const M98090_MIXRCVL_DACL_SHIFT: c_int = 0;
pub const M98090_MIXRCVL_DACL_WIDTH: c_int = 1;

pub const M98090_MIXRCVL_SHIFT: c_int = 0;
pub const M98090_MIXRCVL_WIDTH: c_int = 6;
//
// M98090_REG_RCV_LOUTL_CONTROL
//

pub const M98090_MIXRCVLG_SHIFT: c_int = 0;
pub const M98090_MIXRCVLG_WIDTH: c_int = 2;

//
// M98090_REG_RCV_LOUTL_VOLUME
//

pub const M98090_RCVLM_SHIFT: c_int = 7;
pub const M98090_RCVLM_WIDTH: c_int = 1;

pub const M98090_RCVLVOL_SHIFT: c_int = 0;
pub const M98090_RCVLVOL_WIDTH: c_int = 5;

//
// M98090_REG_LOUTR_MIXER
//

pub const M98090_LINMOD_SHIFT: c_int = 7;
pub const M98090_LINMOD_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_MIC2_SHIFT: c_int = 5;
pub const M98090_MIXRCVR_MIC2_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_MIC1_SHIFT: c_int = 4;
pub const M98090_MIXRCVR_MIC1_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_LINEB_SHIFT: c_int = 3;
pub const M98090_MIXRCVR_LINEB_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_LINEA_SHIFT: c_int = 2;
pub const M98090_MIXRCVR_LINEA_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_DACR_SHIFT: c_int = 1;
pub const M98090_MIXRCVR_DACR_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_DACL_SHIFT: c_int = 0;
pub const M98090_MIXRCVR_DACL_WIDTH: c_int = 1;

pub const M98090_MIXRCVR_SHIFT: c_int = 0;
pub const M98090_MIXRCVR_WIDTH: c_int = 6;
//
// M98090_REG_LOUTR_CONTROL
//

pub const M98090_MIXRCVRG_SHIFT: c_int = 0;
pub const M98090_MIXRCVRG_WIDTH: c_int = 2;

//
// M98090_REG_LOUTR_VOLUME
//

pub const M98090_RCVRM_SHIFT: c_int = 7;
pub const M98090_RCVRM_WIDTH: c_int = 1;

pub const M98090_RCVRVOL_SHIFT: c_int = 0;
pub const M98090_RCVRVOL_WIDTH: c_int = 5;

//
// M98090_REG_JACK_DETECT
//

pub const M98090_JDETEN_SHIFT: c_int = 7;
pub const M98090_JDETEN_WIDTH: c_int = 1;

pub const M98090_JDWK_SHIFT: c_int = 6;
pub const M98090_JDWK_WIDTH: c_int = 1;

pub const M98090_JDEB_SHIFT: c_int = 0;
pub const M98090_JDEB_WIDTH: c_int = 2;

//
// M98090_REG_INPUT_ENABLE
//

pub const M98090_MBEN_SHIFT: c_int = 4;
pub const M98090_MBEN_WIDTH: c_int = 1;

pub const M98090_LINEAEN_SHIFT: c_int = 3;
pub const M98090_LINEAEN_WIDTH: c_int = 1;

pub const M98090_LINEBEN_SHIFT: c_int = 2;
pub const M98090_LINEBEN_WIDTH: c_int = 1;

pub const M98090_ADREN_SHIFT: c_int = 1;
pub const M98090_ADREN_WIDTH: c_int = 1;

pub const M98090_ADLEN_SHIFT: c_int = 0;
pub const M98090_ADLEN_WIDTH: c_int = 1;
//
// M98090_REG_OUTPUT_ENABLE
//

pub const M98090_HPREN_SHIFT: c_int = 7;
pub const M98090_HPREN_WIDTH: c_int = 1;

pub const M98090_HPLEN_SHIFT: c_int = 6;
pub const M98090_HPLEN_WIDTH: c_int = 1;

pub const M98090_SPREN_SHIFT: c_int = 5;
pub const M98090_SPREN_WIDTH: c_int = 1;

pub const M98090_SPLEN_SHIFT: c_int = 4;
pub const M98090_SPLEN_WIDTH: c_int = 1;

pub const M98090_RCVLEN_SHIFT: c_int = 3;
pub const M98090_RCVLEN_WIDTH: c_int = 1;

pub const M98090_RCVREN_SHIFT: c_int = 2;
pub const M98090_RCVREN_WIDTH: c_int = 1;

pub const M98090_DAREN_SHIFT: c_int = 1;
pub const M98090_DAREN_WIDTH: c_int = 1;

pub const M98090_DALEN_SHIFT: c_int = 0;
pub const M98090_DALEN_WIDTH: c_int = 1;
//
// M98090_REG_LEVEL_CONTROL
//

pub const M98090_ZDENN_SHIFT: c_int = 2;
pub const M98090_ZDENN_WIDTH: c_int = 1;

pub const M98090_VS2ENN_SHIFT: c_int = 1;
pub const M98090_VS2ENN_WIDTH: c_int = 1;

pub const M98090_VSENN_SHIFT: c_int = 0;
pub const M98090_VSENN_WIDTH: c_int = 1;

//
// M98090_REG_DSP_FILTER_ENABLE
//

pub const M98090_DMIC34BQEN_SHIFT: c_int = 4;
pub const M98090_DMIC34BQEN_WIDTH: c_int = 1;

pub const M98090_ADCBQEN_SHIFT: c_int = 3;
pub const M98090_ADCBQEN_WIDTH: c_int = 1;

pub const M98090_EQ3BANDEN_SHIFT: c_int = 2;
pub const M98090_EQ3BANDEN_WIDTH: c_int = 1;

pub const M98090_EQ5BANDEN_SHIFT: c_int = 1;
pub const M98090_EQ5BANDEN_WIDTH: c_int = 1;

pub const M98090_EQ7BANDEN_SHIFT: c_int = 0;
pub const M98090_EQ7BANDEN_WIDTH: c_int = 1;

//
// M98090_REG_BIAS_CONTROL
//

pub const M98090_VCM_MODE_SHIFT: c_int = 0;
pub const M98090_VCM_MODE_WIDTH: c_int = 1;

//
// M98090_REG_DAC_CONTROL
//

pub const M98090_PERFMODE_SHIFT: c_int = 1;
pub const M98090_PERFMODE_WIDTH: c_int = 1;

pub const M98090_DACHP_SHIFT: c_int = 0;
pub const M98090_DACHP_WIDTH: c_int = 1;

//
// M98090_REG_ADC_CONTROL
//

pub const M98090_OSR128_SHIFT: c_int = 2;
pub const M98090_OSR128_WIDTH: c_int = 1;

pub const M98090_ADCDITHER_SHIFT: c_int = 1;
pub const M98090_ADCDITHER_WIDTH: c_int = 1;

pub const M98090_ADCHP_SHIFT: c_int = 0;
pub const M98090_ADCHP_WIDTH: c_int = 1;

//
// M98090_REG_DEVICE_SHUTDOWN
//

pub const M98090_SHDNN_SHIFT: c_int = 7;
pub const M98090_SHDNN_WIDTH: c_int = 1;
//
// M98090_REG_EQUALIZER_BASE
//

pub const M98090_B0_1_HI_SHIFT: c_int = 0;
pub const M98090_B0_1_HI_WIDTH: c_int = 8;

pub const M98090_B0_1_MID_SHIFT: c_int = 0;
pub const M98090_B0_1_MID_WIDTH: c_int = 8;

pub const M98090_B0_1_LO_SHIFT: c_int = 0;
pub const M98090_B0_1_LO_WIDTH: c_int = 8;

pub const M98090_B1_1_HI_SHIFT: c_int = 0;
pub const M98090_B1_1_HI_WIDTH: c_int = 8;

pub const M98090_B1_1_MID_SHIFT: c_int = 0;
pub const M98090_B1_1_MID_WIDTH: c_int = 8;

pub const M98090_B1_1_LO_SHIFT: c_int = 0;
pub const M98090_B1_1_LO_WIDTH: c_int = 8;

pub const M98090_B2_1_HI_SHIFT: c_int = 0;
pub const M98090_B2_1_HI_WIDTH: c_int = 8;

pub const M98090_B2_1_MID_SHIFT: c_int = 0;
pub const M98090_B2_1_MID_WIDTH: c_int = 8;

pub const M98090_B2_1_LO_SHIFT: c_int = 0;
pub const M98090_B2_1_LO_WIDTH: c_int = 8;

pub const M98090_A1_1_HI_SHIFT: c_int = 0;
pub const M98090_A1_1_HI_WIDTH: c_int = 8;

pub const M98090_A1_1_MID_SHIFT: c_int = 0;
pub const M98090_A1_1_MID_WIDTH: c_int = 8;

pub const M98090_A1_1_LO_SHIFT: c_int = 0;
pub const M98090_A1_1_LO_WIDTH: c_int = 8;

pub const M98090_A2_1_HI_SHIFT: c_int = 0;
pub const M98090_A2_1_HI_WIDTH: c_int = 8;

pub const M98090_A2_1_MID_SHIFT: c_int = 0;
pub const M98090_A2_1_MID_WIDTH: c_int = 8;

pub const M98090_A2_1_LO_SHIFT: c_int = 0;
pub const M98090_A2_1_LO_WIDTH: c_int = 8;
pub const M98090_COEFS_PER_BAND: c_int = 5;

//
// M98090_REG_RECORD_BIQUAD_BASE
//

pub const M98090_REC_B0_HI_SHIFT: c_int = 0;
pub const M98090_REC_B0_HI_WIDTH: c_int = 8;

pub const M98090_REC_B0_MID_SHIFT: c_int = 0;
pub const M98090_REC_B0_MID_WIDTH: c_int = 8;

pub const M98090_REC_B0_LO_SHIFT: c_int = 0;
pub const M98090_REC_B0_LO_WIDTH: c_int = 8;

pub const M98090_REC_B1_HI_SHIFT: c_int = 0;
pub const M98090_REC_B1_HI_WIDTH: c_int = 8;

pub const M98090_REC_B1_MID_SHIFT: c_int = 0;
pub const M98090_REC_B1_MID_WIDTH: c_int = 8;

pub const M98090_REC_B1_LO_SHIFT: c_int = 0;
pub const M98090_REC_B1_LO_WIDTH: c_int = 8;

pub const M98090_REC_B2_HI_SHIFT: c_int = 0;
pub const M98090_REC_B2_HI_WIDTH: c_int = 8;

pub const M98090_REC_B2_MID_SHIFT: c_int = 0;
pub const M98090_REC_B2_MID_WIDTH: c_int = 8;

pub const M98090_REC_B2_LO_SHIFT: c_int = 0;
pub const M98090_REC_B2_LO_WIDTH: c_int = 8;

pub const M98090_REC_A1_HI_SHIFT: c_int = 0;
pub const M98090_REC_A1_HI_WIDTH: c_int = 8;

pub const M98090_REC_A1_MID_SHIFT: c_int = 0;
pub const M98090_REC_A1_MID_WIDTH: c_int = 8;

pub const M98090_REC_A1_LO_SHIFT: c_int = 0;
pub const M98090_REC_A1_LO_WIDTH: c_int = 8;

pub const M98090_REC_A2_HI_SHIFT: c_int = 0;
pub const M98090_REC_A2_HI_WIDTH: c_int = 8;

pub const M98090_REC_A2_MID_SHIFT: c_int = 0;
pub const M98090_REC_A2_MID_WIDTH: c_int = 8;

pub const M98090_REC_A2_LO_SHIFT: c_int = 0;
pub const M98090_REC_A2_LO_WIDTH: c_int = 8;
//
// M98090_REG_DMIC3_VOLUME
//

pub const M98090_DMIC_AV3G_SHIFT: c_int = 4;
pub const M98090_DMIC_AV3G_WIDTH: c_int = 3;

pub const M98090_DMIC_AV3_SHIFT: c_int = 0;
pub const M98090_DMIC_AV3_WIDTH: c_int = 4;

//
// M98090_REG_DMIC4_VOLUME
//

pub const M98090_DMIC_AV4G_SHIFT: c_int = 4;
pub const M98090_DMIC_AV4G_WIDTH: c_int = 3;

pub const M98090_DMIC_AV4_SHIFT: c_int = 0;
pub const M98090_DMIC_AV4_WIDTH: c_int = 4;

//
// M98090_REG_DMIC34_BQ_PREATTEN
//

pub const M98090_AV34BQ_SHIFT: c_int = 0;
pub const M98090_AV34BQ_WIDTH: c_int = 4;

//
// M98090_REG_RECORD_TDM_SLOT
//

pub const M98090_TDM_SLOTADCL_SHIFT: c_int = 6;
pub const M98090_TDM_SLOTADCL_WIDTH: c_int = 2;

pub const M98090_TDM_SLOTADCR_SHIFT: c_int = 4;
pub const M98090_TDM_SLOTADCR_WIDTH: c_int = 2;

pub const M98090_TDM_SLOTDMIC3_SHIFT: c_int = 2;
pub const M98090_TDM_SLOTDMIC3_WIDTH: c_int = 2;

pub const M98090_TDM_SLOTDMIC4_SHIFT: c_int = 0;
pub const M98090_TDM_SLOTDMIC4_WIDTH: c_int = 2;

//
// M98090_REG_SAMPLE_RATE
//

pub const M98090_DMIC34_ZEROPAD_SHIFT: c_int = 4;
pub const M98090_DMIC34_ZEROPAD_WIDTH: c_int = 1;

pub const M98090_DMIC34_SRDIV_SHIFT: c_int = 0;
pub const M98090_DMIC34_SRDIV_WIDTH: c_int = 3;
//
// M98090_REG_DMIC34_BIQUAD_BASE
//

pub const M98090_DMIC34_B0_HI_SHIFT: c_int = 0;
pub const M98090_DMIC34_B0_HI_WIDTH: c_int = 8;

pub const M98090_DMIC34_B0_MID_SHIFT: c_int = 0;
pub const M98090_DMIC34_B0_MID_WIDTH: c_int = 8;

pub const M98090_DMIC34_B0_LO_SHIFT: c_int = 0;
pub const M98090_DMIC34_B0_LO_WIDTH: c_int = 8;

pub const M98090_DMIC34_B1_HI_SHIFT: c_int = 0;
pub const M98090_DMIC34_B1_HI_WIDTH: c_int = 8;

pub const M98090_DMIC34_B1_MID_SHIFT: c_int = 0;
pub const M98090_DMIC34_B1_MID_WIDTH: c_int = 8;

pub const M98090_DMIC34_B1_LO_SHIFT: c_int = 0;
pub const M98090_DMIC34_B1_LO_WIDTH: c_int = 8;

pub const M98090_DMIC34_B2_HI_SHIFT: c_int = 0;
pub const M98090_DMIC34_B2_HI_WIDTH: c_int = 8;

pub const M98090_DMIC34_B2_MID_SHIFT: c_int = 0;
pub const M98090_DMIC34_B2_MID_WIDTH: c_int = 8;

pub const M98090_DMIC34_B2_LO_SHIFT: c_int = 0;
pub const M98090_DMIC34_B2_LO_WIDTH: c_int = 8;

pub const M98090_DMIC34_A1_HI_SHIFT: c_int = 0;
pub const M98090_DMIC34_A1_HI_WIDTH: c_int = 8;

pub const M98090_DMIC34_A1_MID_SHIFT: c_int = 0;
pub const M98090_DMIC34_A1_MID_WIDTH: c_int = 8;

pub const M98090_DMIC34_A1_LO_SHIFT: c_int = 0;
pub const M98090_DMIC34_A1_LO_WIDTH: c_int = 8;

pub const M98090_DMIC34_A2_HI_SHIFT: c_int = 0;
pub const M98090_DMIC34_A2_HI_WIDTH: c_int = 8;

pub const M98090_DMIC34_A2_MID_SHIFT: c_int = 0;
pub const M98090_DMIC34_A2_MID_WIDTH: c_int = 8;

pub const M98090_DMIC34_A2_LO_SHIFT: c_int = 0;
pub const M98090_DMIC34_A2_LO_WIDTH: c_int = 8;
pub const M98090_JACK_STATE_NO_HEADSET: c_int = 0;
pub const M98090_JACK_STATE_NO_HEADSET_2: c_int = 1;
pub const M98090_JACK_STATE_HEADPHONE: c_int = 2;
pub const M98090_JACK_STATE_HEADSET: c_int = 3;
//
// M98090_REG_REVISION_ID
//

pub const M98090_REVID_SHIFT: c_int = 0;
pub const M98090_REVID_WIDTH: c_int = 8;

// Silicon revision number
pub const M98090_REVA: c_uint = 0x40;
pub const M98091_REVA: c_uint = 0x50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max98090_type {
    MAX98090,
    MAX98091,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98090_cdata {
    pub rate: c_uint,
    pub fmt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98090_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub devtype: max98090_type,
    pub pdata: *mut max98090_pdata,
    pub mclk: *mut clk,
    pub sysclk: c_uint,
    pub pclk: c_uint,
    pub bclk: c_uint,
    pub lrclk: c_uint,
    pub dmic_freq: u32,
    pub dai: [max98090_cdata; 1],
    pub jack_state: c_int,
    pub jack_work: delayed_work,
    pub pll_det_enable_work: delayed_work,
    pub pll_det_disable_work: work_struct,
    pub jack: *mut snd_soc_jack,
    pub dai_fmt: c_uint,
    pub tdm_slots: c_int,
    pub tdm_lslot: c_int,
    pub tdm_rslot: c_int,
    pub lin_state: u8,
    pub pa1en: c_uint,
    pub pa2en: c_uint,
    pub sidetone: c_uint,
    pub master: bool,
    pub shdn_pending: bool,
}
