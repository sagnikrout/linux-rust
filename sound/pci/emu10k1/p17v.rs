//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/emu10k1/p17v.h
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
// Copyright (c) by James Courtier-Dutton <James@superbug.demon.co.uk>
// Driver p17v chips
//
// Audigy2Value Tina (P17V) pointer-offset register set,
// accessed through the PTR2 and DATA2 registers
//
// 00 - 07: Not used
pub const P17V_PLAYBACK_FIFO_PTR: c_uint = 0x08	/* Current playback fifo pointer;
// and number of sound samples in cache.
//
// 09 - 12: Not used
pub const P17V_CAPTURE_FIFO_PTR: c_uint = 0x13	/* Current capture fifo pointer;
// and number of sound samples in cache.
//
// 14 - 17: Not used
pub const P17V_PB_CHN_SEL: c_uint = 0x18	/* P17v playback channel select */;
pub const P17V_SE_SLOT_SEL_L: c_uint = 0x19	/* Sound Engine slot select low */;
pub const P17V_SE_SLOT_SEL_H: c_uint = 0x1a	/* Sound Engine slot select high */;
// 1b - 1f: Not used
// 20 - 2f: Not used
// 30 - 3b: Not used
pub const P17V_SPI: c_uint = 0x3c	/* SPI interface register */;
pub const P17V_I2C_ADDR: c_uint = 0x3d	/* I2C Address */;
pub const P17V_I2C_0: c_uint = 0x3e	/* I2C Data */;
pub const P17V_I2C_1: c_uint = 0x3f	/* I2C Data */;
// I2C values
pub const I2C_A_ADC_ADD_MASK: c_uint = 0x000000fe	/*The address is a 7 bit address */;
pub const I2C_A_ADC_RW_MASK: c_uint = 0x00000001	/*bit mask for R/W */;
pub const I2C_A_ADC_TRANS_MASK: c_uint = 0x00000010  	/*Bit mask for I2c address DAC value  */;
pub const I2C_A_ADC_ABORT_MASK: c_uint = 0x00000020	/*Bit mask for I2C transaction abort flag */;
pub const I2C_A_ADC_LAST_MASK: c_uint = 0x00000040	/*Bit mask for Last word transaction */;
pub const I2C_A_ADC_BYTE_MASK: c_uint = 0x00000080	/*Bit mask for Byte Mode */;
pub const I2C_A_ADC_ADD: c_uint = 0x00000034	/*This is the Device address for ADC  */;
pub const I2C_A_ADC_READ: c_uint = 0x00000001	/*To perform a read operation */;
pub const I2C_A_ADC_START: c_uint = 0x00000100	/*Start I2C transaction */;
pub const I2C_A_ADC_ABORT: c_uint = 0x00000200	/*I2C transaction abort */;
pub const I2C_A_ADC_LAST: c_uint = 0x00000400	/*I2C last transaction */;
pub const I2C_A_ADC_BYTE: c_uint = 0x00000800	/*I2C one byte mode */;
pub const I2C_D_ADC_REG_MASK: c_uint = 0xfe000000  	/*ADC address register */;
pub const I2C_D_ADC_DAT_MASK: c_uint = 0x01ff0000  	/*ADC data register */;
pub const ADC_TIMEOUT: c_uint = 0x00000007	/*ADC Timeout Clock Disable */;
pub const ADC_IFC_CTRL: c_uint = 0x0000000b	/*ADC Interface Control */;
pub const ADC_MASTER: c_uint = 0x0000000c	/*ADC Master Mode Control */;
pub const ADC_POWER: c_uint = 0x0000000d	/*ADC PowerDown Control */;
pub const ADC_ATTEN_ADCL: c_uint = 0x0000000e	/*ADC Attenuation ADCL */;
pub const ADC_ATTEN_ADCR: c_uint = 0x0000000f	/*ADC Attenuation ADCR */;
pub const ADC_ALC_CTRL1: c_uint = 0x00000010	/*ADC ALC Control 1 */;
pub const ADC_ALC_CTRL2: c_uint = 0x00000011	/*ADC ALC Control 2 */;
pub const ADC_ALC_CTRL3: c_uint = 0x00000012	/*ADC ALC Control 3 */;
pub const ADC_NOISE_CTRL: c_uint = 0x00000013	/*ADC Noise Gate Control */;
pub const ADC_LIMIT_CTRL: c_uint = 0x00000014	/*ADC Limiter Control */;
pub const ADC_MUX: c_uint = 0x00000015  	/*ADC Mux offset */;

// FIXME: Not tested yet.
pub const ADC_GAIN_MASK: c_uint = 0x000000ff	//Mask for ADC Gain;
pub const ADC_ZERODB: c_uint = 0x000000cf	//Value to set ADC to 0dB;
pub const ADC_MUTE_MASK: c_uint = 0x000000c0	//Mask for ADC mute;
pub const ADC_MUTE: c_uint = 0x000000c0	//Value to mute ADC;
pub const ADC_OSR: c_uint = 0x00000008	//Mask for ADC oversample rate select;
pub const ADC_TIMEOUT_DISABLE: c_uint = 0x00000008	//Value and mask to disable Timeout clock;
pub const ADC_HPF_DISABLE: c_uint = 0x00000100	//Value and mask to disable High pass filter;
pub const ADC_TRANWIN_MASK: c_uint = 0x00000070	//Mask for Length of Transient Window;

pub const ADC_MUX_MASK: c_uint = 0x0000000f	//Mask for ADC Mux;
pub const ADC_MUX_0: c_uint = 0x00000001	//Value to select Unknown at ADC Mux (Not used);
pub const ADC_MUX_1: c_uint = 0x00000002	//Value to select Unknown at ADC Mux (Not used);
pub const ADC_MUX_2: c_uint = 0x00000004	//Value to select Mic at ADC Mux;
pub const ADC_MUX_3: c_uint = 0x00000008	//Value to select Line-In at ADC Mux;
pub const P17V_START_AUDIO: c_uint = 0x40	/* Start Audio bit */;
// 41 - 47: Reserved
pub const P17V_START_CAPTURE: c_uint = 0x48	/* Start Capture bit */;
pub const P17V_CAPTURE_FIFO_BASE: c_uint = 0x49	/* Record FIFO base address */;
pub const P17V_CAPTURE_FIFO_SIZE: c_uint = 0x4a	/* Record FIFO buffer size */;
pub const P17V_CAPTURE_FIFO_INDEX: c_uint = 0x4b	/* Record FIFO capture index */;
pub const P17V_CAPTURE_VOL_H: c_uint = 0x4c	/* P17v capture volume control */;
pub const P17V_CAPTURE_VOL_L: c_uint = 0x4d	/* P17v capture volume control */;
// 4e - 4f: Not used
// 50 - 5f: Not used
pub const P17V_SRCSel: c_uint = 0x60	/* SRC48 and SRCMulti sample rate select;
// and output select
//
pub const P17V_MIXER_AC97_10K1_VOL_L: c_uint = 0x61	/* 10K to Mixer_AC97 input volume control */;
pub const P17V_MIXER_AC97_10K1_VOL_H: c_uint = 0x62	/* 10K to Mixer_AC97 input volume control */;
pub const P17V_MIXER_AC97_P17V_VOL_L: c_uint = 0x63	/* P17V to Mixer_AC97 input volume control */;
pub const P17V_MIXER_AC97_P17V_VOL_H: c_uint = 0x64	/* P17V to Mixer_AC97 input volume control */;
pub const P17V_MIXER_AC97_SRP_REC_VOL_L: c_uint = 0x65	/* SRP Record to Mixer_AC97 input volume control */;
pub const P17V_MIXER_AC97_SRP_REC_VOL_H: c_uint = 0x66	/* SRP Record to Mixer_AC97 input volume control */;
// 67 - 68: Reserved
pub const P17V_MIXER_Spdif_10K1_VOL_L: c_uint = 0x69	/* 10K to Mixer_Spdif input volume control */;
pub const P17V_MIXER_Spdif_10K1_VOL_H: c_uint = 0x6A	/* 10K to Mixer_Spdif input volume control */;
pub const P17V_MIXER_Spdif_P17V_VOL_L: c_uint = 0x6B	/* P17V to Mixer_Spdif input volume control */;
pub const P17V_MIXER_Spdif_P17V_VOL_H: c_uint = 0x6C	/* P17V to Mixer_Spdif input volume control */;
pub const P17V_MIXER_Spdif_SRP_REC_VOL_L: c_uint = 0x6D	/* SRP Record to Mixer_Spdif input volume control */;
pub const P17V_MIXER_Spdif_SRP_REC_VOL_H: c_uint = 0x6E	/* SRP Record to Mixer_Spdif input volume control */;
// 6f - 70: Reserved
pub const P17V_MIXER_I2S_10K1_VOL_L: c_uint = 0x71	/* 10K to Mixer_I2S input volume control */;
pub const P17V_MIXER_I2S_10K1_VOL_H: c_uint = 0x72	/* 10K to Mixer_I2S input volume control */;
pub const P17V_MIXER_I2S_P17V_VOL_L: c_uint = 0x73	/* P17V to Mixer_I2S input volume control */;
pub const P17V_MIXER_I2S_P17V_VOL_H: c_uint = 0x74	/* P17V to Mixer_I2S input volume control */;
pub const P17V_MIXER_I2S_SRP_REC_VOL_L: c_uint = 0x75	/* SRP Record to Mixer_I2S input volume control */;
pub const P17V_MIXER_I2S_SRP_REC_VOL_H: c_uint = 0x76	/* SRP Record to Mixer_I2S input volume control */;
// 77 - 78: Reserved
pub const P17V_MIXER_AC97_ENABLE: c_uint = 0x79	/* Mixer AC97 input audio enable */;
pub const P17V_MIXER_SPDIF_ENABLE: c_uint = 0x7A	/* Mixer SPDIF input audio enable */;
pub const P17V_MIXER_I2S_ENABLE: c_uint = 0x7B	/* Mixer I2S input audio enable */;
pub const P17V_AUDIO_OUT_ENABLE: c_uint = 0x7C	/* Audio out enable */;
pub const P17V_MIXER_ATT: c_uint = 0x7D	/* SRP Mixer Attenuation Select */;
pub const P17V_SRP_RECORD_SRR: c_uint = 0x7E	/* SRP Record channel source Select */;
pub const P17V_SOFT_RESET_SRP_MIXER: c_uint = 0x7F	/* SRP and mixer soft reset */;
pub const P17V_AC97_OUT_MASTER_VOL_L: c_uint = 0x80	/* AC97 Output master volume control */;
pub const P17V_AC97_OUT_MASTER_VOL_H: c_uint = 0x81	/* AC97 Output master volume control */;
pub const P17V_SPDIF_OUT_MASTER_VOL_L: c_uint = 0x82	/* SPDIF Output master volume control */;
pub const P17V_SPDIF_OUT_MASTER_VOL_H: c_uint = 0x83	/* SPDIF Output master volume control */;
pub const P17V_I2S_OUT_MASTER_VOL_L: c_uint = 0x84	/* I2S Output master volume control */;
pub const P17V_I2S_OUT_MASTER_VOL_H: c_uint = 0x85	/* I2S Output master volume control */;
// 86 - 87: Not used
pub const P17V_I2S_CHANNEL_SWAP_PHASE_INVERSE: c_uint = 0x88	/* I2S out mono channel swap;
// and phase inverse
pub const P17V_SPDIF_CHANNEL_SWAP_PHASE_INVERSE: c_uint = 0x89	/* SPDIF out mono channel swap;
// and phase inverse
// 8A: Not used
pub const P17V_SRP_P17V_ESR: c_uint = 0x8B	/* SRP_P17V estimated sample rate and rate lock */;
pub const P17V_SRP_REC_ESR: c_uint = 0x8C	/* SRP_REC estimated sample rate and rate lock */;
pub const P17V_SRP_BYPASS: c_uint = 0x8D	/* srps channel bypass and srps bypass */;
// 8E - 92: Not used
pub const P17V_I2S_SRC_SEL: c_uint = 0x93	/* I2SIN mode sel */;
