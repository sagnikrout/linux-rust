//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ssm2602.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// File:         sound/soc/codecs/ssm2602.h
// Author:       Cliff Cai <Cliff.Cai@analog.com>
//
// Created:      Tue June 06 2008
//
// Modified:
// Copyright 2008 Analog Devices Inc.
//
// Bugs:         Enter bugs at http://blackfin.uclinux.org
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssm2602_type {
    SSM2602,
    SSM2604,
}

// SSM2602 Codec Register definitions
pub const SSM2602_LINVOL: c_uint = 0x00;
pub const SSM2602_RINVOL: c_uint = 0x01;
pub const SSM2602_LOUT1V: c_uint = 0x02;
pub const SSM2602_ROUT1V: c_uint = 0x03;
pub const SSM2602_APANA: c_uint = 0x04;
pub const SSM2602_APDIGI: c_uint = 0x05;
pub const SSM2602_PWR: c_uint = 0x06;
pub const SSM2602_IFACE: c_uint = 0x07;
pub const SSM2602_SRATE: c_uint = 0x08;
pub const SSM2602_ACTIVE: c_uint = 0x09;
pub const SSM2602_RESET: c_uint = 0x0f;
// SSM2602 Codec Register Field definitions
// (Mask value to extract the corresponding Register field)
//
// Left ADC Volume Control (SSM2602_REG_LEFT_ADC_VOL)
pub const LINVOL_LIN_VOL: c_uint = 0x01F   /* Left Channel PGA Volume control                      */;
pub const LINVOL_LIN_ENABLE_MUTE: c_uint = 0x080   /* Left Channel Input Mute                              */;
pub const LINVOL_LRIN_BOTH: c_uint = 0x100   /* Left Channel Line Input Volume update                */;
// Right ADC Volume Control (SSM2602_REG_RIGHT_ADC_VOL)
pub const RINVOL_RIN_VOL: c_uint = 0x01F   /* Right Channel PGA Volume control                     */;
pub const RINVOL_RIN_ENABLE_MUTE: c_uint = 0x080   /* Right Channel Input Mute                             */;
pub const RINVOL_RLIN_BOTH: c_uint = 0x100   /* Right Channel Line Input Volume update               */;
// Left DAC Volume Control (SSM2602_REG_LEFT_DAC_VOL)
pub const LOUT1V_LHP_VOL: c_uint = 0x07F   /* Left Channel Headphone volume control                */;
pub const LOUT1V_ENABLE_LZC: c_uint = 0x080   /* Left Channel Zero cross detect enable                */;
pub const LOUT1V_LRHP_BOTH: c_uint = 0x100   /* Left Channel Headphone volume update                 */;
// Right DAC Volume Control (SSM2602_REG_RIGHT_DAC_VOL)
pub const ROUT1V_RHP_VOL: c_uint = 0x07F   /* Right Channel Headphone volume control               */;
pub const ROUT1V_ENABLE_RZC: c_uint = 0x080   /* Right Channel Zero cross detect enable               */;
pub const ROUT1V_RLHP_BOTH: c_uint = 0x100   /* Right Channel Headphone volume update                */;
// Analogue Audio Path Control (SSM2602_REG_ANALOGUE_PATH)
pub const APANA_ENABLE_MIC_BOOST: c_uint = 0x001   /* Primary Microphone Amplifier gain booster control    */;
pub const APANA_ENABLE_MIC_MUTE: c_uint = 0x002   /* Microphone Mute Control                              */;
pub const APANA_ADC_IN_SELECT: c_uint = 0x004   /* Microphone/Line IN select to ADC (1=MIC, 0=Line In)  */;
pub const APANA_ENABLE_BYPASS: c_uint = 0x008   /* Line input bypass to line output                     */;
pub const APANA_SELECT_DAC: c_uint = 0x010   /* Select DAC (1=Select DAC, 0=Don't Select DAC)        */;
pub const APANA_ENABLE_SIDETONE: c_uint = 0x020   /* Enable/Disable Side Tone                             */;
pub const APANA_SIDETONE_ATTN: c_uint = 0x0C0   /* Side Tone Attenuation                                */;
pub const APANA_ENABLE_MIC_BOOST2: c_uint = 0x100   /* Secondary Microphone Amplifier gain booster control  */;
// Digital Audio Path Control (SSM2602_REG_DIGITAL_PATH)
pub const APDIGI_ENABLE_ADC_HPF: c_uint = 0x001   /* Enable/Disable ADC Highpass Filter                   */;
pub const APDIGI_DE_EMPHASIS: c_uint = 0x006   /* De-Emphasis Control                                  */;
pub const APDIGI_ENABLE_DAC_MUTE: c_uint = 0x008   /* DAC Mute Control                                     */;
pub const APDIGI_STORE_OFFSET: c_uint = 0x010   /* Store/Clear DC offset when HPF is disabled           */;
// Power Down Control (SSM2602_REG_POWER)
// (1=Enable PowerDown, 0=Disable PowerDown)
//
pub const PWR_LINE_IN_PDN: c_uint = 0x001   /* Line Input Power Down                                */;
pub const PWR_MIC_PDN: c_uint = 0x002   /* Microphone Input & Bias Power Down                   */;
pub const PWR_ADC_PDN: c_uint = 0x004   /* ADC Power Down                                       */;
pub const PWR_DAC_PDN: c_uint = 0x008   /* DAC Power Down                                       */;
pub const PWR_OUT_PDN: c_uint = 0x010   /* Outputs Power Down                                   */;
pub const PWR_OSC_PDN: c_uint = 0x020   /* Oscillator Power Down                                */;
pub const PWR_CLK_OUT_PDN: c_uint = 0x040   /* CLKOUT Power Down                                    */;
pub const PWR_POWER_OFF: c_uint = 0x080   /* POWEROFF Mode                                        */;
// Digital Audio Interface Format (SSM2602_REG_DIGITAL_IFACE)
pub const IFACE_IFACE_FORMAT: c_uint = 0x003   /* Digital Audio input format control                   */;
pub const IFACE_AUDIO_DATA_LEN: c_uint = 0x00C   /* Audio Data word length control                       */;
pub const IFACE_DAC_LR_POLARITY: c_uint = 0x010   /* Polarity Control for clocks in RJ,LJ and I2S modes   */;
pub const IFACE_DAC_LR_SWAP: c_uint = 0x020   /* Swap DAC data control                                */;
pub const IFACE_ENABLE_MASTER: c_uint = 0x040   /* Enable/Disable Master Mode                           */;
pub const IFACE_BCLK_INVERT: c_uint = 0x080   /* Bit Clock Inversion control                          */;
// Sampling Control (SSM2602_REG_SAMPLING_CTRL)
pub const SRATE_ENABLE_USB_MODE: c_uint = 0x001   /* Enable/Disable USB Mode                              */;
pub const SRATE_BOS_RATE: c_uint = 0x002   /* Base Over-Sampling rate                              */;
pub const SRATE_SAMPLE_RATE: c_uint = 0x03C   /* Clock setting condition (Sampling rate control)      */;
pub const SRATE_CORECLK_DIV2: c_uint = 0x040   /* Core Clock divider select                            */;
pub const SRATE_CLKOUT_DIV2: c_uint = 0x080   /* Clock Out divider select                             */;
// Active Control (SSM2602_REG_ACTIVE_CTRL)
pub const ACTIVE_ACTIVATE_CODEC: c_uint = 0x001   /* Activate Codec Digital Audio Interface               */;
//
pub const SSM2602_CACHEREGNUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssm2602_clk {
    SSM2602_SYSCLK,
    SSM2602_CLK_CLKOUT,
    SSM2602_CLK_XTO
}
