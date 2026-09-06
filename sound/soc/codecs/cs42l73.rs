//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l73.h
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
// ALSA SoC CS42L73 codec driver
//
// Copyright 2011 Cirrus Logic, Inc.
//
// Author: Georgi Vlaev <joe@nucleusys.com>
// Brian Austin <brian.austin@cirrus.com>
//
// I2C Registers
// I2C Address: 1001010[R/W] - 10010100 = 0x94(Write); 10010101 = 0x95(Read)
pub const CS42L73_CHIP_ID: c_uint = 0x4a;
pub const CS42L73_DEVID_AB: c_uint = 0x01	/* Device ID A & B [RO]. */;
pub const CS42L73_DEVID_CD: c_uint = 0x02    /* Device ID C & D [RO]. */;
pub const CS42L73_DEVID_E: c_uint = 0x03    /* Device ID E [RO]. */;
pub const CS42L73_REVID: c_uint = 0x05    /* Revision ID [RO]. */;
pub const CS42L73_PWRCTL1: c_uint = 0x06    /* Power Control 1. */;
pub const CS42L73_PWRCTL2: c_uint = 0x07    /* Power Control 2. */;
pub const CS42L73_PWRCTL3: c_uint = 0x08    /* Power Control 3. */;
pub const CS42L73_CPFCHC: c_uint = 0x09    /* Charge Pump Freq. Class H Ctl. */;
pub const CS42L73_OLMBMSDC: c_uint = 0x0A    /* Output Load, MIC Bias, MIC2 SDT */;
pub const CS42L73_DMMCC: c_uint = 0x0B    /* Digital MIC & Master Clock Ctl. */;
pub const CS42L73_XSPC: c_uint = 0x0C    /* Auxiliary Serial Port (XSP) Ctl. */;
pub const CS42L73_XSPMMCC: c_uint = 0x0D    /* XSP Master Mode Clocking Control. */;
pub const CS42L73_ASPC: c_uint = 0x0E    /* Audio Serial Port (ASP) Control. */;
pub const CS42L73_ASPMMCC: c_uint = 0x0F    /* ASP Master Mode Clocking Control. */;
pub const CS42L73_VSPC: c_uint = 0x10    /* Voice Serial Port (VSP) Control. */;
pub const CS42L73_VSPMMCC: c_uint = 0x11    /* VSP Master Mode Clocking Control. */;
pub const CS42L73_VXSPFS: c_uint = 0x12    /* VSP & XSP Sample Rate. */;
pub const CS42L73_MIOPC: c_uint = 0x13    /* Misc. Input & Output Path Control. */;
pub const CS42L73_ADCIPC: c_uint = 0x14	/* ADC/IP Control. */;
pub const CS42L73_MICAPREPGAAVOL: c_uint = 0x15	/* MIC 1 [A] PreAmp, PGAA Vol. */;
pub const CS42L73_MICBPREPGABVOL: c_uint = 0x16	/* MIC 2 [B] PreAmp, PGAB Vol. */;
pub const CS42L73_IPADVOL: c_uint = 0x17	/* Input Pat7h A Digital Volume. */;
pub const CS42L73_IPBDVOL: c_uint = 0x18	/* Input Path B Digital Volume. */;
pub const CS42L73_PBDC: c_uint = 0x19	/* Playback Digital Control. */;
pub const CS42L73_HLADVOL: c_uint = 0x1A	/* HP/Line A Out Digital Vol. */;
pub const CS42L73_HLBDVOL: c_uint = 0x1B	/* HP/Line B Out Digital Vol. */;
pub const CS42L73_SPKDVOL: c_uint = 0x1C	/* Spkphone Out [A] Digital Vol. */;
pub const CS42L73_ESLDVOL: c_uint = 0x1D	/* Ear/Spkphone LO [B] Digital */;
pub const CS42L73_HPAAVOL: c_uint = 0x1E	/* HP A Analog Volume. */;
pub const CS42L73_HPBAVOL: c_uint = 0x1F	/* HP B Analog Volume. */;
pub const CS42L73_LOAAVOL: c_uint = 0x20	/* Line Out A Analog Volume. */;
pub const CS42L73_LOBAVOL: c_uint = 0x21	/* Line Out B Analog Volume. */;
pub const CS42L73_STRINV: c_uint = 0x22	/* Stereo Input Path Adv. Vol. */;
pub const CS42L73_XSPINV: c_uint = 0x23	/* Auxiliary Port Input Advisory Vol. */;
pub const CS42L73_ASPINV: c_uint = 0x24	/* Audio Port Input Advisory Vol. */;
pub const CS42L73_VSPINV: c_uint = 0x25	/* Voice Port Input Advisory Vol. */;
pub const CS42L73_LIMARATEHL: c_uint = 0x26	/* Lmtr Attack Rate HP/Line. */;
pub const CS42L73_LIMRRATEHL: c_uint = 0x27	/* Lmtr Ctl, Rel.Rate HP/Line. */;
pub const CS42L73_LMAXHL: c_uint = 0x28	/* Lmtr Thresholds HP/Line. */;
pub const CS42L73_LIMARATESPK: c_uint = 0x29	/* Lmtr Attack Rate Spkphone [A]. */;
pub const CS42L73_LIMRRATESPK: c_uint = 0x2A	/* Lmtr Ctl,Release Rate Spk. [A]. */;
pub const CS42L73_LMAXSPK: c_uint = 0x2B	/* Lmtr Thresholds Spkphone [A]. */;
pub const CS42L73_LIMARATEESL: c_uint = 0x2C	/* Lmtr Attack Rate  */;
pub const CS42L73_LIMRRATEESL: c_uint = 0x2D	/* Lmtr Ctl,Release Rate */;
pub const CS42L73_LMAXESL: c_uint = 0x2E	/* Lmtr Thresholds */;
pub const CS42L73_ALCARATE: c_uint = 0x2F	/* ALC Enable, Attack Rate AB. */;
pub const CS42L73_ALCRRATE: c_uint = 0x30	/* ALC Release Rate AB.  */;
pub const CS42L73_ALCMINMAX: c_uint = 0x31	/* ALC Thresholds AB. */;
pub const CS42L73_NGCAB: c_uint = 0x32	/* Noise Gate Ctl AB. */;
pub const CS42L73_ALCNGMC: c_uint = 0x33	/* ALC & Noise Gate Misc Ctl. */;
pub const CS42L73_MIXERCTL: c_uint = 0x34	/* Mixer Control. */;
pub const CS42L73_HLAIPAA: c_uint = 0x35	/* HP/LO Left Mixer: L. */;
pub const CS42L73_HLBIPBA: c_uint = 0x36	/* HP/LO Right Mixer: R.  */;
pub const CS42L73_HLAXSPAA: c_uint = 0x37	/* HP/LO Left Mixer: XSP L */;
pub const CS42L73_HLBXSPBA: c_uint = 0x38	/* HP/LO Right Mixer: XSP R */;
pub const CS42L73_HLAASPAA: c_uint = 0x39	/* HP/LO Left Mixer: ASP L */;
pub const CS42L73_HLBASPBA: c_uint = 0x3A	/* HP/LO Right Mixer: ASP R */;
pub const CS42L73_HLAVSPMA: c_uint = 0x3B	/* HP/LO Left Mixer: VSP. */;
pub const CS42L73_HLBVSPMA: c_uint = 0x3C	/* HP/LO Right Mixer: VSP */;
pub const CS42L73_XSPAIPAA: c_uint = 0x3D	/* XSP Left Mixer: Left */;
pub const CS42L73_XSPBIPBA: c_uint = 0x3E	/* XSP Rt. Mixer: Right */;
pub const CS42L73_XSPAXSPAA: c_uint = 0x3F	/* XSP Left Mixer: XSP L */;
pub const CS42L73_XSPBXSPBA: c_uint = 0x40	/* XSP Rt. Mixer: XSP R */;
pub const CS42L73_XSPAASPAA: c_uint = 0x41	/* XSP Left Mixer: ASP L */;
pub const CS42L73_XSPAASPBA: c_uint = 0x42	/* XSP Rt. Mixer: ASP R */;
pub const CS42L73_XSPAVSPMA: c_uint = 0x43	/* XSP Left Mixer: VSP */;
pub const CS42L73_XSPBVSPMA: c_uint = 0x44	/* XSP Rt. Mixer: VSP */;
pub const CS42L73_ASPAIPAA: c_uint = 0x45	/* ASP Left Mixer: Left */;
pub const CS42L73_ASPBIPBA: c_uint = 0x46	/* ASP Rt. Mixer: Right */;
pub const CS42L73_ASPAXSPAA: c_uint = 0x47	/* ASP Left Mixer: XSP L */;
pub const CS42L73_ASPBXSPBA: c_uint = 0x48	/* ASP Rt. Mixer: XSP R */;
pub const CS42L73_ASPAASPAA: c_uint = 0x49	/* ASP Left Mixer: ASP L */;
pub const CS42L73_ASPBASPBA: c_uint = 0x4A	/* ASP Rt. Mixer: ASP R */;
pub const CS42L73_ASPAVSPMA: c_uint = 0x4B	/* ASP Left Mixer: VSP */;
pub const CS42L73_ASPBVSPMA: c_uint = 0x4C	/* ASP Rt. Mixer: VSP */;
pub const CS42L73_VSPAIPAA: c_uint = 0x4D	/* VSP Left Mixer: Left */;
pub const CS42L73_VSPBIPBA: c_uint = 0x4E	/* VSP Rt. Mixer: Right */;
pub const CS42L73_VSPAXSPAA: c_uint = 0x4F	/* VSP Left Mixer: XSP L */;
pub const CS42L73_VSPBXSPBA: c_uint = 0x50	/* VSP Rt. Mixer: XSP R */;
pub const CS42L73_VSPAASPAA: c_uint = 0x51	/* VSP Left Mixer: ASP Left */;
pub const CS42L73_VSPBASPBA: c_uint = 0x52	/* VSP Rt. Mixer: ASP Right */;
pub const CS42L73_VSPAVSPMA: c_uint = 0x53	/* VSP Left Mixer: VSP */;
pub const CS42L73_VSPBVSPMA: c_uint = 0x54	/* VSP Rt. Mixer: VSP */;
pub const CS42L73_MMIXCTL: c_uint = 0x55	/* Mono Mixer Controls. */;
pub const CS42L73_SPKMIPMA: c_uint = 0x56	/* SPK Mono Mixer: In. Path */;
pub const CS42L73_SPKMXSPA: c_uint = 0x57	/* SPK Mono Mixer: XSP Mono/L/R Att. */;
pub const CS42L73_SPKMASPA: c_uint = 0x58	/* SPK Mono Mixer: ASP Mono/L/R Att. */;
pub const CS42L73_SPKMVSPMA: c_uint = 0x59	/* SPK Mono Mixer: VSP Mono Atten. */;
pub const CS42L73_ESLMIPMA: c_uint = 0x5A	/* Ear/SpLO Mono Mixer: */;
pub const CS42L73_ESLMXSPA: c_uint = 0x5B	/* Ear/SpLO Mono Mixer: XSP */;
pub const CS42L73_ESLMASPA: c_uint = 0x5C	/* Ear/SpLO Mono Mixer: ASP */;
pub const CS42L73_ESLMVSPMA: c_uint = 0x5D	/* Ear/SpLO Mono Mixer: VSP */;
pub const CS42L73_IM1: c_uint = 0x5E	/* Interrupt Mask 1.  */;
pub const CS42L73_IM2: c_uint = 0x5F	/* Interrupt Mask 2. */;
pub const CS42L73_IS1: c_uint = 0x60	/* Interrupt Status 1 [RO]. */;
pub const CS42L73_IS2: c_uint = 0x61	/* Interrupt Status 2 [RO]. */;
pub const CS42L73_MAX_REGISTER: c_uint = 0x61	/* Total Registers */;
// Bitfield Definitions
// CS42L73_PWRCTL1

// CS42L73_PWRCTL2

// CS42L73_PWRCTL3

// Thermal Overload Detect. Requires interrupt ...
pub const CS42L73_THMOVLD_150C: c_int = 0;
pub const CS42L73_THMOVLD_132C: c_int = 1;
pub const CS42L73_THMOVLD_115C: c_int = 2;
pub const CS42L73_THMOVLD_098C: c_int = 3;

// CS42L73_ASPC, CS42L73_XSPC, CS42L73_VSPC

// CS42L73_xSPMMCC

// CS42L73_DMMCC

// CS42L73 MCLK derived from MCLK1 or MCLK2
pub const CS42L73_CLKID_MCLK1: c_int = 0;
pub const CS42L73_CLKID_MCLK2: c_int = 1;
pub const CS42L73_MCLKXDIV: c_int = 0;
pub const CS42L73_MMCCDIV: c_int = 1;
pub const CS42L73_XSP: c_int = 0;
pub const CS42L73_ASP: c_int = 1;
pub const CS42L73_VSP: c_int = 2;
// IS1, IM1

// Analog Softramp

// HP A/B Analog Mute

// LO A/B Analog Mute

// Digital Mute

// Misc defines for codec
pub const CS42L73_DEVID: c_uint = 0x00042A73;
pub const CS42L73_MCLKX_MIN: c_int = 5644800;
pub const CS42L73_MCLKX_MAX: c_int = 38400000;

