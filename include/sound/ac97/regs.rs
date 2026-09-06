//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ac97/regs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Universal interface for Audio Codec '97
//
// For more details look to AC '97 component specification revision 2.1
// by Intel Corporation (http://developer.intel.com).
//
// AC'97 codec registers
//
pub const AC97_RESET: c_uint = 0x00	/* Reset */;
pub const AC97_MASTER: c_uint = 0x02	/* Master Volume */;
pub const AC97_HEADPHONE: c_uint = 0x04	/* Headphone Volume (optional) */;
pub const AC97_MASTER_MONO: c_uint = 0x06	/* Master Volume Mono (optional) */;
pub const AC97_MASTER_TONE: c_uint = 0x08	/* Master Tone (Bass & Treble) (optional) */;
pub const AC97_PC_BEEP: c_uint = 0x0a	/* PC Beep Volume (optional) */;
pub const AC97_PHONE: c_uint = 0x0c	/* Phone Volume (optional) */;
pub const AC97_MIC: c_uint = 0x0e	/* MIC Volume */;
pub const AC97_LINE: c_uint = 0x10	/* Line In Volume */;
pub const AC97_CD: c_uint = 0x12	/* CD Volume */;
pub const AC97_VIDEO: c_uint = 0x14	/* Video Volume (optional) */;
pub const AC97_AUX: c_uint = 0x16	/* AUX Volume (optional) */;
pub const AC97_PCM: c_uint = 0x18	/* PCM Volume */;
pub const AC97_REC_SEL: c_uint = 0x1a	/* Record Select */;
pub const AC97_REC_GAIN: c_uint = 0x1c	/* Record Gain */;
pub const AC97_REC_GAIN_MIC: c_uint = 0x1e	/* Record Gain MIC (optional) */;
pub const AC97_GENERAL_PURPOSE: c_uint = 0x20	/* General Purpose (optional) */;
pub const AC97_3D_CONTROL: c_uint = 0x22	/* 3D Control (optional) */;
pub const AC97_INT_PAGING: c_uint = 0x24	/* Audio Interrupt & Paging (AC'97 2.3) */;
pub const AC97_POWERDOWN: c_uint = 0x26	/* Powerdown control / status */;
// range 0x28-0x3a - AUDIO AC'97 2.0 extensions
pub const AC97_EXTENDED_ID: c_uint = 0x28	/* Extended Audio ID */;
pub const AC97_EXTENDED_STATUS: c_uint = 0x2a	/* Extended Audio Status and Control */;
pub const AC97_PCM_FRONT_DAC_RATE: c_uint = 0x2c	/* PCM Front DAC Rate */;
pub const AC97_PCM_SURR_DAC_RATE: c_uint = 0x2e	/* PCM Surround DAC Rate */;
pub const AC97_PCM_LFE_DAC_RATE: c_uint = 0x30	/* PCM LFE DAC Rate */;
pub const AC97_PCM_LR_ADC_RATE: c_uint = 0x32	/* PCM LR ADC Rate */;
pub const AC97_PCM_MIC_ADC_RATE: c_uint = 0x34	/* PCM MIC ADC Rate */;
pub const AC97_CENTER_LFE_MASTER: c_uint = 0x36	/* Center + LFE Master Volume */;
pub const AC97_SURROUND_MASTER: c_uint = 0x38	/* Surround (Rear) Master Volume */;
pub const AC97_SPDIF: c_uint = 0x3a	/* S/PDIF control */;
// range 0x3c-0x58 - MODEM
pub const AC97_EXTENDED_MID: c_uint = 0x3c	/* Extended Modem ID */;
pub const AC97_EXTENDED_MSTATUS: c_uint = 0x3e	/* Extended Modem Status and Control */;
pub const AC97_LINE1_RATE: c_uint = 0x40	/* Line1 DAC/ADC Rate */;
pub const AC97_LINE2_RATE: c_uint = 0x42	/* Line2 DAC/ADC Rate */;
pub const AC97_HANDSET_RATE: c_uint = 0x44	/* Handset DAC/ADC Rate */;
pub const AC97_LINE1_LEVEL: c_uint = 0x46	/* Line1 DAC/ADC Level */;
pub const AC97_LINE2_LEVEL: c_uint = 0x48	/* Line2 DAC/ADC Level */;
pub const AC97_HANDSET_LEVEL: c_uint = 0x4a	/* Handset DAC/ADC Level */;
pub const AC97_GPIO_CFG: c_uint = 0x4c	/* GPIO Configuration */;
pub const AC97_GPIO_POLARITY: c_uint = 0x4e	/* GPIO Pin Polarity/Type, 0=low, 1=high active */;
pub const AC97_GPIO_STICKY: c_uint = 0x50	/* GPIO Pin Sticky, 0=not, 1=sticky */;
pub const AC97_GPIO_WAKEUP: c_uint = 0x52	/* GPIO Pin Wakeup, 0=no int, 1=yes int */;
pub const AC97_GPIO_STATUS: c_uint = 0x54	/* GPIO Pin Status, slot 12 */;
pub const AC97_MISC_AFE: c_uint = 0x56	/* Miscellaneous Modem AFE Status and Control */;
// range 0x5a-0x7b - Vendor Specific
pub const AC97_VENDOR_ID1: c_uint = 0x7c	/* Vendor ID1 */;
pub const AC97_VENDOR_ID2: c_uint = 0x7e	/* Vendor ID2 / revision */;
// range 0x60-0x6f (page 1) - extended codec registers
pub const AC97_CODEC_CLASS_REV: c_uint = 0x60	/* Codec Class/Revision */;
pub const AC97_PCI_SVID: c_uint = 0x62	/* PCI Subsystem Vendor ID */;
pub const AC97_PCI_SID: c_uint = 0x64	/* PCI Subsystem ID */;
pub const AC97_FUNC_SELECT: c_uint = 0x66	/* Function Select */;
pub const AC97_FUNC_INFO: c_uint = 0x68	/* Function Information */;
pub const AC97_SENSE_INFO: c_uint = 0x6a	/* Sense Details */;
// volume controls
pub const AC97_MUTE_MASK_MONO: c_uint = 0x8000;
pub const AC97_MUTE_MASK_STEREO: c_uint = 0x8080;
// slot allocation
pub const AC97_SLOT_TAG: c_int = 0;
pub const AC97_SLOT_CMD_ADDR: c_int = 1;
pub const AC97_SLOT_CMD_DATA: c_int = 2;
pub const AC97_SLOT_PCM_LEFT: c_int = 3;
pub const AC97_SLOT_PCM_RIGHT: c_int = 4;
pub const AC97_SLOT_MODEM_LINE1: c_int = 5;
pub const AC97_SLOT_PCM_CENTER: c_int = 6;

pub const AC97_SLOT_SPDIF_LEFT1: c_int = 6;

pub const AC97_SLOT_SPDIF_LEFT: c_int = 7;

pub const AC97_SLOT_SPDIF_RIGHT: c_int = 8;
pub const AC97_SLOT_LFE: c_int = 9;
pub const AC97_SLOT_SPDIF_RIGHT1: c_int = 9;
pub const AC97_SLOT_MODEM_LINE2: c_int = 10;

pub const AC97_SLOT_SPDIF_LEFT2: c_int = 10;

pub const AC97_SLOT_SPDIF_RIGHT2: c_int = 11;

// basic capabilities (reset register)
pub const AC97_BC_DEDICATED_MIC: c_uint = 0x0001	/* Dedicated Mic PCM In Channel */;
pub const AC97_BC_RESERVED1: c_uint = 0x0002	/* Reserved (was Modem Line Codec support) */;
pub const AC97_BC_BASS_TREBLE: c_uint = 0x0004	/* Bass & Treble Control */;
pub const AC97_BC_SIM_STEREO: c_uint = 0x0008	/* Simulated stereo */;
pub const AC97_BC_HEADPHONE: c_uint = 0x0010	/* Headphone Out Support */;
pub const AC97_BC_LOUDNESS: c_uint = 0x0020	/* Loudness (bass boost) Support */;
pub const AC97_BC_16BIT_DAC: c_uint = 0x0000	/* 16-bit DAC resolution */;
pub const AC97_BC_18BIT_DAC: c_uint = 0x0040	/* 18-bit DAC resolution */;
pub const AC97_BC_20BIT_DAC: c_uint = 0x0080	/* 20-bit DAC resolution */;
pub const AC97_BC_DAC_MASK: c_uint = 0x00c0;
pub const AC97_BC_16BIT_ADC: c_uint = 0x0000	/* 16-bit ADC resolution */;
pub const AC97_BC_18BIT_ADC: c_uint = 0x0100	/* 18-bit ADC resolution */;
pub const AC97_BC_20BIT_ADC: c_uint = 0x0200	/* 20-bit ADC resolution */;
pub const AC97_BC_ADC_MASK: c_uint = 0x0300;
pub const AC97_BC_3D_TECH_ID_MASK: c_uint = 0x7c00	/* Per-vendor ID of 3D enhancement */;
// general purpose
pub const AC97_GP_DRSS_MASK: c_uint = 0x0c00	/* double rate slot select */;
pub const AC97_GP_DRSS_1011: c_uint = 0x0000	/* LR(C) 10+11(+12) */;
pub const AC97_GP_DRSS_78: c_uint = 0x0400	/* LR 7+8 */;
// powerdown bits
pub const AC97_PD_ADC_STATUS: c_uint = 0x0001	/* ADC status (RO) */;
pub const AC97_PD_DAC_STATUS: c_uint = 0x0002	/* DAC status (RO) */;
pub const AC97_PD_MIXER_STATUS: c_uint = 0x0004	/* Analog mixer status (RO) */;
pub const AC97_PD_VREF_STATUS: c_uint = 0x0008	/* Vref status (RO) */;
pub const AC97_PD_PR0: c_uint = 0x0100	/* Power down PCM ADCs and input MUX */;
pub const AC97_PD_PR1: c_uint = 0x0200	/* Power down PCM front DAC */;
pub const AC97_PD_PR2: c_uint = 0x0400	/* Power down Mixer (Vref still on) */;
pub const AC97_PD_PR3: c_uint = 0x0800	/* Power down Mixer (Vref off) */;
pub const AC97_PD_PR4: c_uint = 0x1000	/* Power down AC-Link */;
pub const AC97_PD_PR5: c_uint = 0x2000	/* Disable internal clock usage */;
pub const AC97_PD_PR6: c_uint = 0x4000	/* Headphone amplifier */;
pub const AC97_PD_EAPD: c_uint = 0x8000	/* External Amplifer Power Down (EAPD) */;
// extended audio ID bit defines
pub const AC97_EI_VRA: c_uint = 0x0001	/* Variable bit rate supported */;
pub const AC97_EI_DRA: c_uint = 0x0002	/* Double rate supported */;
pub const AC97_EI_SPDIF: c_uint = 0x0004	/* S/PDIF out supported */;
pub const AC97_EI_VRM: c_uint = 0x0008	/* Variable bit rate supported for MIC */;
pub const AC97_EI_DACS_SLOT_MASK: c_uint = 0x0030	/* DACs slot assignment */;
pub const AC97_EI_DACS_SLOT_SHIFT: c_int = 4;
pub const AC97_EI_CDAC: c_uint = 0x0040	/* PCM Center DAC available */;
pub const AC97_EI_SDAC: c_uint = 0x0080	/* PCM Surround DACs available */;
pub const AC97_EI_LDAC: c_uint = 0x0100	/* PCM LFE DAC available */;
pub const AC97_EI_AMAP: c_uint = 0x0200	/* indicates optional slot/DAC mapping based on codec ID */;
pub const AC97_EI_REV_MASK: c_uint = 0x0c00	/* AC'97 revision mask */;
pub const AC97_EI_REV_22: c_uint = 0x0400	/* AC'97 revision 2.2 */;
pub const AC97_EI_REV_23: c_uint = 0x0800	/* AC'97 revision 2.3 */;
pub const AC97_EI_REV_SHIFT: c_int = 10;
pub const AC97_EI_ADDR_MASK: c_uint = 0xc000	/* physical codec ID (address) */;
pub const AC97_EI_ADDR_SHIFT: c_int = 14;
// extended audio status and control bit defines
pub const AC97_EA_VRA: c_uint = 0x0001	/* Variable bit rate enable bit */;
pub const AC97_EA_DRA: c_uint = 0x0002	/* Double-rate audio enable bit */;
pub const AC97_EA_SPDIF: c_uint = 0x0004	/* S/PDIF out enable bit */;
pub const AC97_EA_VRM: c_uint = 0x0008	/* Variable bit rate for MIC enable bit */;
pub const AC97_EA_SPSA_SLOT_MASK: c_uint = 0x0030	/* Mask for slot assignment bits */;
pub const AC97_EA_SPSA_SLOT_SHIFT: c_int = 4;
pub const AC97_EA_SPSA_3_4: c_uint = 0x0000	/* Slot assigned to 3 & 4 */;
pub const AC97_EA_SPSA_7_8: c_uint = 0x0010	/* Slot assigned to 7 & 8 */;
pub const AC97_EA_SPSA_6_9: c_uint = 0x0020	/* Slot assigned to 6 & 9 */;
pub const AC97_EA_SPSA_10_11: c_uint = 0x0030	/* Slot assigned to 10 & 11 */;
pub const AC97_EA_CDAC: c_uint = 0x0040	/* PCM Center DAC is ready (Read only) */;
pub const AC97_EA_SDAC: c_uint = 0x0080	/* PCM Surround DACs are ready (Read only) */;
pub const AC97_EA_LDAC: c_uint = 0x0100	/* PCM LFE DAC is ready (Read only) */;
pub const AC97_EA_MDAC: c_uint = 0x0200	/* MIC ADC is ready (Read only) */;
pub const AC97_EA_SPCV: c_uint = 0x0400	/* S/PDIF configuration valid (Read only) */;
pub const AC97_EA_PRI: c_uint = 0x0800	/* Turns the PCM Center DAC off */;
pub const AC97_EA_PRJ: c_uint = 0x1000	/* Turns the PCM Surround DACs off */;
pub const AC97_EA_PRK: c_uint = 0x2000	/* Turns the PCM LFE DAC off */;
pub const AC97_EA_PRL: c_uint = 0x4000	/* Turns the MIC ADC off */;
// S/PDIF control bit defines
pub const AC97_SC_PRO: c_uint = 0x0001	/* Professional status */;
pub const AC97_SC_NAUDIO: c_uint = 0x0002	/* Non audio stream */;
pub const AC97_SC_COPY: c_uint = 0x0004	/* Copyright status */;
pub const AC97_SC_PRE: c_uint = 0x0008	/* Preemphasis status */;
pub const AC97_SC_CC_MASK: c_uint = 0x07f0	/* Category Code mask */;
pub const AC97_SC_CC_SHIFT: c_int = 4;
pub const AC97_SC_L: c_uint = 0x0800	/* Generation Level status */;
pub const AC97_SC_SPSR_MASK: c_uint = 0x3000	/* S/PDIF Sample Rate bits */;
pub const AC97_SC_SPSR_SHIFT: c_int = 12;
pub const AC97_SC_SPSR_44K: c_uint = 0x0000	/* Use 44.1kHz Sample rate */;
pub const AC97_SC_SPSR_48K: c_uint = 0x2000	/* Use 48kHz Sample rate */;
pub const AC97_SC_SPSR_32K: c_uint = 0x3000	/* Use 32kHz Sample rate */;
pub const AC97_SC_DRS: c_uint = 0x4000	/* Double Rate S/PDIF */;
pub const AC97_SC_V: c_uint = 0x8000	/* Validity status */;
// Interrupt and Paging bit defines (AC'97 2.3)
pub const AC97_PAGE_MASK: c_uint = 0x000f	/* Page Selector */;

pub const AC97_INT_ENABLE: c_uint = 0x0800	/* Interrupt Enable */;
pub const AC97_INT_SENSE: c_uint = 0x1000	/* Sense Cycle */;
pub const AC97_INT_CAUSE_SENSE: c_uint = 0x2000	/* Sense Cycle Completed (RO) */;
pub const AC97_INT_CAUSE_GPIO: c_uint = 0x4000	/* GPIO bits changed (RO) */;
pub const AC97_INT_STATUS: c_uint = 0x8000	/* Interrupt Status */;
// extended modem ID bit defines
pub const AC97_MEI_LINE1: c_uint = 0x0001	/* Line1 present */;
pub const AC97_MEI_LINE2: c_uint = 0x0002	/* Line2 present */;
pub const AC97_MEI_HANDSET: c_uint = 0x0004	/* Handset present */;
pub const AC97_MEI_CID1: c_uint = 0x0008	/* caller ID decode for Line1 is supported */;
pub const AC97_MEI_CID2: c_uint = 0x0010	/* caller ID decode for Line2 is supported */;
pub const AC97_MEI_ADDR_MASK: c_uint = 0xc000	/* physical codec ID (address) */;
pub const AC97_MEI_ADDR_SHIFT: c_int = 14;
// extended modem status and control bit defines
pub const AC97_MEA_GPIO: c_uint = 0x0001	/* GPIO is ready (ro) */;
pub const AC97_MEA_MREF: c_uint = 0x0002	/* Vref is up to nominal level (ro) */;
pub const AC97_MEA_ADC1: c_uint = 0x0004	/* ADC1 operational (ro) */;
pub const AC97_MEA_DAC1: c_uint = 0x0008	/* DAC1 operational (ro) */;
pub const AC97_MEA_ADC2: c_uint = 0x0010	/* ADC2 operational (ro) */;
pub const AC97_MEA_DAC2: c_uint = 0x0020	/* DAC2 operational (ro) */;
pub const AC97_MEA_HADC: c_uint = 0x0040	/* HADC operational (ro) */;
pub const AC97_MEA_HDAC: c_uint = 0x0080	/* HDAC operational (ro) */;
pub const AC97_MEA_PRA: c_uint = 0x0100	/* GPIO power down (high) */;
pub const AC97_MEA_PRB: c_uint = 0x0200	/* reserved */;
pub const AC97_MEA_PRC: c_uint = 0x0400	/* ADC1 power down (high) */;
pub const AC97_MEA_PRD: c_uint = 0x0800	/* DAC1 power down (high) */;
pub const AC97_MEA_PRE: c_uint = 0x1000	/* ADC2 power down (high) */;
pub const AC97_MEA_PRF: c_uint = 0x2000	/* DAC2 power down (high) */;
pub const AC97_MEA_PRG: c_uint = 0x4000	/* HADC power down (high) */;
pub const AC97_MEA_PRH: c_uint = 0x8000	/* HDAC power down (high) */;
// modem gpio status defines
pub const AC97_GPIO_LINE1_OH: c_uint = 0x0001  /* Off Hook Line1 */;
pub const AC97_GPIO_LINE1_RI: c_uint = 0x0002  /* Ring Detect Line1 */;
pub const AC97_GPIO_LINE1_CID: c_uint = 0x0004  /* Caller ID path enable Line1 */;
pub const AC97_GPIO_LINE1_LCS: c_uint = 0x0008  /* Loop Current Sense Line1 */;
pub const AC97_GPIO_LINE1_PULSE: c_uint = 0x0010  /* Opt./ Pulse Dial Line1 (out) */;
pub const AC97_GPIO_LINE1_HL1R: c_uint = 0x0020  /* Opt./ Handset to Line1 relay control (out) */;
pub const AC97_GPIO_LINE1_HOHD: c_uint = 0x0040  /* Opt./ Handset off hook detect Line1 (in) */;
pub const AC97_GPIO_LINE12_AC: c_uint = 0x0080  /* Opt./ Int.bit 1 / Line1/2 AC (out) */;
pub const AC97_GPIO_LINE12_DC: c_uint = 0x0100  /* Opt./ Int.bit 2 / Line1/2 DC (out) */;
pub const AC97_GPIO_LINE12_RS: c_uint = 0x0200  /* Opt./ Int.bit 3 / Line1/2 RS (out) */;
pub const AC97_GPIO_LINE2_OH: c_uint = 0x0400  /* Off Hook Line2 */;
pub const AC97_GPIO_LINE2_RI: c_uint = 0x0800  /* Ring Detect Line2 */;
pub const AC97_GPIO_LINE2_CID: c_uint = 0x1000  /* Caller ID path enable Line2 */;
pub const AC97_GPIO_LINE2_LCS: c_uint = 0x2000  /* Loop Current Sense Line2 */;
pub const AC97_GPIO_LINE2_PULSE: c_uint = 0x4000  /* Opt./ Pulse Dial Line2 (out) */;
pub const AC97_GPIO_LINE2_HL1R: c_uint = 0x8000  /* Opt./ Handset to Line2 relay control (out) */;
