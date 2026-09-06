//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/sis7019.h
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

// Macro flag: #define __sis7019_h__
//
// Definitions for SiS7019 Audio Accelerator
//
// Copyright (C) 2004-2007, David Dillow
// Written by David Dillow <dave@thedillows.org>
// Inspired by the Trident 4D-WaveDX/NX driver.
//
// All rights reserved.
//
// General Control Register
pub const SIS_GCR: c_uint = 0x00;
pub const SIS_GCR_MACRO_POWER_DOWN: c_uint = 0x80000000;
pub const SIS_GCR_MODEM_ENABLE: c_uint = 0x00010000;
pub const SIS_GCR_SOFTWARE_RESET: c_uint = 0x00000001;
// General Interrupt Enable Register
pub const SIS_GIER: c_uint = 0x04;
pub const SIS_GIER_MODEM_TIMER_IRQ_ENABLE: c_uint = 0x00100000;
pub const SIS_GIER_MODEM_RX_DMA_IRQ_ENABLE: c_uint = 0x00080000;
pub const SIS_GIER_MODEM_TX_DMA_IRQ_ENABLE: c_uint = 0x00040000;
pub const SIS_GIER_AC97_GPIO1_IRQ_ENABLE: c_uint = 0x00020000;
pub const SIS_GIER_AC97_GPIO0_IRQ_ENABLE: c_uint = 0x00010000;
pub const SIS_GIER_AC97_SAMPLE_TIMER_IRQ_ENABLE: c_uint = 0x00000010;
pub const SIS_GIER_AUDIO_GLOBAL_TIMER_IRQ_ENABLE: c_uint = 0x00000008;
pub const SIS_GIER_AUDIO_RECORD_DMA_IRQ_ENABLE: c_uint = 0x00000004;
pub const SIS_GIER_AUDIO_PLAY_DMA_IRQ_ENABLE: c_uint = 0x00000002;
pub const SIS_GIER_AUDIO_WAVE_ENGINE_IRQ_ENABLE: c_uint = 0x00000001;
// General Interrupt Status Register
pub const SIS_GISR: c_uint = 0x08;
pub const SIS_GISR_MODEM_TIMER_IRQ_STATUS: c_uint = 0x00100000;
pub const SIS_GISR_MODEM_RX_DMA_IRQ_STATUS: c_uint = 0x00080000;
pub const SIS_GISR_MODEM_TX_DMA_IRQ_STATUS: c_uint = 0x00040000;
pub const SIS_GISR_AC97_GPIO1_IRQ_STATUS: c_uint = 0x00020000;
pub const SIS_GISR_AC97_GPIO0_IRQ_STATUS: c_uint = 0x00010000;
pub const SIS_GISR_AC97_SAMPLE_TIMER_IRQ_STATUS: c_uint = 0x00000010;
pub const SIS_GISR_AUDIO_GLOBAL_TIMER_IRQ_STATUS: c_uint = 0x00000008;
pub const SIS_GISR_AUDIO_RECORD_DMA_IRQ_STATUS: c_uint = 0x00000004;
pub const SIS_GISR_AUDIO_PLAY_DMA_IRQ_STATUS: c_uint = 0x00000002;
pub const SIS_GISR_AUDIO_WAVE_ENGINE_IRQ_STATUS: c_uint = 0x00000001;
// DMA Control Register
pub const SIS_DMA_CSR: c_uint = 0x10;
pub const SIS_DMA_CSR_PCI_SETTINGS: c_uint = 0x0000001d;
pub const SIS_DMA_CSR_CONCURRENT_ENABLE: c_uint = 0x00000200;
pub const SIS_DMA_CSR_PIPELINE_ENABLE: c_uint = 0x00000100;
pub const SIS_DMA_CSR_RX_DRAIN_ENABLE: c_uint = 0x00000010;
pub const SIS_DMA_CSR_RX_FILL_ENABLE: c_uint = 0x00000008;
pub const SIS_DMA_CSR_TX_DRAIN_ENABLE: c_uint = 0x00000004;
pub const SIS_DMA_CSR_TX_LOWPRI_FILL_ENABLE: c_uint = 0x00000002;
pub const SIS_DMA_CSR_TX_HIPRI_FILL_ENABLE: c_uint = 0x00000001;
// Playback Channel Start Registers
pub const SIS_PLAY_START_A_REG: c_uint = 0x14;
pub const SIS_PLAY_START_B_REG: c_uint = 0x18;
// Playback Channel Stop Registers
pub const SIS_PLAY_STOP_A_REG: c_uint = 0x1c;
pub const SIS_PLAY_STOP_B_REG: c_uint = 0x20;
// Recording Channel Start Register
pub const SIS_RECORD_START_REG: c_uint = 0x24;
// Recording Channel Stop Register
pub const SIS_RECORD_STOP_REG: c_uint = 0x28;
// Playback Interrupt Status Registers
pub const SIS_PISR_A: c_uint = 0x2c;
pub const SIS_PISR_B: c_uint = 0x30;
// Recording Interrupt Status Register
pub const SIS_RISR: c_uint = 0x34;
// AC97 AC-link Playback Source Register
pub const SIS_AC97_PSR: c_uint = 0x40;
pub const SIS_AC97_PSR_MODEM_HEADSET_SRC_MIXER: c_uint = 0x0f000000;
pub const SIS_AC97_PSR_MODEM_LINE2_SRC_MIXER: c_uint = 0x00f00000;
pub const SIS_AC97_PSR_MODEM_LINE1_SRC_MIXER: c_uint = 0x000f0000;
pub const SIS_AC97_PSR_PCM_LFR_SRC_MIXER: c_uint = 0x0000f000;
pub const SIS_AC97_PSR_PCM_SURROUND_SRC_MIXER: c_uint = 0x00000f00;
pub const SIS_AC97_PSR_PCM_CENTER_SRC_MIXER: c_uint = 0x000000f0;
pub const SIS_AC97_PSR_PCM_LR_SRC_MIXER: c_uint = 0x0000000f;
// AC97 AC-link Command Register
pub const SIS_AC97_CMD: c_uint = 0x50;
pub const SIS_AC97_CMD_DATA_MASK: c_uint = 0xffff0000;
pub const SIS_AC97_CMD_REG_MASK: c_uint = 0x0000ff00;
pub const SIS_AC97_CMD_CODEC3_READ: c_uint = 0x0000000d;
pub const SIS_AC97_CMD_CODEC3_WRITE: c_uint = 0x0000000c;
pub const SIS_AC97_CMD_CODEC2_READ: c_uint = 0x0000000b;
pub const SIS_AC97_CMD_CODEC2_WRITE: c_uint = 0x0000000a;
pub const SIS_AC97_CMD_CODEC_READ: c_uint = 0x00000009;
pub const SIS_AC97_CMD_CODEC_WRITE: c_uint = 0x00000008;
pub const SIS_AC97_CMD_CODEC_WARM_RESET: c_uint = 0x00000005;
pub const SIS_AC97_CMD_CODEC_COLD_RESET: c_uint = 0x00000004;
pub const SIS_AC97_CMD_DONE: c_uint = 0x00000000;
// AC97 AC-link Semaphore Register
pub const SIS_AC97_SEMA: c_uint = 0x54;
pub const SIS_AC97_SEMA_BUSY: c_uint = 0x00000001;
pub const SIS_AC97_SEMA_RELEASE: c_uint = 0x00000000;
// AC97 AC-link Status Register
pub const SIS_AC97_STATUS: c_uint = 0x58;
pub const SIS_AC97_STATUS_AUDIO_D2_INACT_SECS: c_uint = 0x03f00000;
pub const SIS_AC97_STATUS_MODEM_ALIVE: c_uint = 0x00002000;
pub const SIS_AC97_STATUS_AUDIO_ALIVE: c_uint = 0x00001000;
pub const SIS_AC97_STATUS_CODEC3_READY: c_uint = 0x00000400;
pub const SIS_AC97_STATUS_CODEC2_READY: c_uint = 0x00000200;
pub const SIS_AC97_STATUS_CODEC_READY: c_uint = 0x00000100;
pub const SIS_AC97_STATUS_WARM_RESET: c_uint = 0x00000080;
pub const SIS_AC97_STATUS_COLD_RESET: c_uint = 0x00000040;
pub const SIS_AC97_STATUS_POWERED_DOWN: c_uint = 0x00000020;
pub const SIS_AC97_STATUS_NORMAL: c_uint = 0x00000010;
pub const SIS_AC97_STATUS_READ_EXPIRED: c_uint = 0x00000004;
pub const SIS_AC97_STATUS_SEMAPHORE: c_uint = 0x00000002;
pub const SIS_AC97_STATUS_BUSY: c_uint = 0x00000001;
// AC97 AC-link Audio Configuration Register
pub const SIS_AC97_CONF: c_uint = 0x5c;
pub const SIS_AC97_CONF_AUDIO_ALIVE: c_uint = 0x80000000;
pub const SIS_AC97_CONF_WARM_RESET_ENABLE: c_uint = 0x40000000;
pub const SIS_AC97_CONF_PR6_ENABLE: c_uint = 0x20000000;
pub const SIS_AC97_CONF_PR5_ENABLE: c_uint = 0x10000000;
pub const SIS_AC97_CONF_PR4_ENABLE: c_uint = 0x08000000;
pub const SIS_AC97_CONF_PR3_ENABLE: c_uint = 0x04000000;
pub const SIS_AC97_CONF_PR2_PR7_ENABLE: c_uint = 0x02000000;
pub const SIS_AC97_CONF_PR0_PR1_ENABLE: c_uint = 0x01000000;
pub const SIS_AC97_CONF_AUTO_PM_ENABLE: c_uint = 0x00800000;
pub const SIS_AC97_CONF_PCM_LFE_ENABLE: c_uint = 0x00080000;
pub const SIS_AC97_CONF_PCM_SURROUND_ENABLE: c_uint = 0x00040000;
pub const SIS_AC97_CONF_PCM_CENTER_ENABLE: c_uint = 0x00020000;
pub const SIS_AC97_CONF_PCM_LR_ENABLE: c_uint = 0x00010000;
pub const SIS_AC97_CONF_PCM_CAP_MIC_ENABLE: c_uint = 0x00002000;
pub const SIS_AC97_CONF_PCM_CAP_LR_ENABLE: c_uint = 0x00001000;
pub const SIS_AC97_CONF_PCM_CAP_MIC_FROM_CODEC3: c_uint = 0x00000200;
pub const SIS_AC97_CONF_PCM_CAP_LR_FROM_CODEC3: c_uint = 0x00000100;
pub const SIS_AC97_CONF_CODEC3_PM_VRM: c_uint = 0x00000080;
pub const SIS_AC97_CONF_CODEC_PM_VRM: c_uint = 0x00000040;
pub const SIS_AC97_CONF_CODEC3_VRA_ENABLE: c_uint = 0x00000020;
pub const SIS_AC97_CONF_CODEC_VRA_ENABLE: c_uint = 0x00000010;
pub const SIS_AC97_CONF_CODEC3_PM_EAC: c_uint = 0x00000008;
pub const SIS_AC97_CONF_CODEC_PM_EAC: c_uint = 0x00000004;
pub const SIS_AC97_CONF_CODEC3_EXISTS: c_uint = 0x00000002;
pub const SIS_AC97_CONF_CODEC_EXISTS: c_uint = 0x00000001;
// Playback Channel Sync Group registers
pub const SIS_PLAY_SYNC_GROUP_A: c_uint = 0x80;
pub const SIS_PLAY_SYNC_GROUP_B: c_uint = 0x84;
pub const SIS_PLAY_SYNC_GROUP_C: c_uint = 0x88;
pub const SIS_PLAY_SYNC_GROUP_D: c_uint = 0x8c;
pub const SIS_MIXER_SYNC_GROUP: c_uint = 0x90;
// Wave Engine Config and Control Register
pub const SIS_WECCR: c_uint = 0xa0;
pub const SIS_WECCR_TESTMODE_MASK: c_uint = 0x00300000;
pub const SIS_WECCR_TESTMODE_NORMAL: c_uint = 0x00000000;
pub const SIS_WECCR_TESTMODE_BYPASS_NSO_ALPHA: c_uint = 0x00100000;
pub const SIS_WECCR_TESTMODE_BYPASS_FC: c_uint = 0x00200000;
pub const SIS_WECCR_TESTMODE_BYPASS_WOL: c_uint = 0x00300000;
pub const SIS_WECCR_RESONANCE_DELAY_MASK: c_uint = 0x00060000;
pub const SIS_WECCR_RESONANCE_DELAY_NONE: c_uint = 0x00000000;
pub const SIS_WECCR_RESONANCE_DELAY_FC_1F00: c_uint = 0x00020000;
pub const SIS_WECCR_RESONANCE_DELAY_FC_1E00: c_uint = 0x00040000;
pub const SIS_WECCR_RESONANCE_DELAY_FC_1C00: c_uint = 0x00060000;
pub const SIS_WECCR_IGNORE_CHANNEL_PARMS: c_uint = 0x00010000;
pub const SIS_WECCR_COMMAND_CHANNEL_ID_MASK: c_uint = 0x0003ff00;
pub const SIS_WECCR_COMMAND_MASK: c_uint = 0x00000007;
pub const SIS_WECCR_COMMAND_NONE: c_uint = 0x00000000;
pub const SIS_WECCR_COMMAND_DONE: c_uint = 0x00000000;
pub const SIS_WECCR_COMMAND_PAUSE: c_uint = 0x00000001;
pub const SIS_WECCR_COMMAND_TOGGLE_VEG: c_uint = 0x00000002;
pub const SIS_WECCR_COMMAND_TOGGLE_MEG: c_uint = 0x00000003;
pub const SIS_WECCR_COMMAND_TOGGLE_VEG_MEG: c_uint = 0x00000004;
// Wave Engine Volume Control Register
pub const SIS_WEVCR: c_uint = 0xa4;
pub const SIS_WEVCR_LEFT_MUSIC_ATTENUATION_MASK: c_uint = 0xff000000;
pub const SIS_WEVCR_RIGHT_MUSIC_ATTENUATION_MASK: c_uint = 0x00ff0000;
pub const SIS_WEVCR_LEFT_WAVE_ATTENUATION_MASK: c_uint = 0x0000ff00;
pub const SIS_WEVCR_RIGHT_WAVE_ATTENUATION_MASK: c_uint = 0x000000ff;
// Wave Engine Interrupt Status Registers
pub const SIS_WEISR_A: c_uint = 0xa8;
pub const SIS_WEISR_B: c_uint = 0xac;
// Playback DMA parameters (parameter RAM)
pub const SIS_PLAY_DMA_OFFSET: c_uint = 0x0000;
pub const SIS_PLAY_DMA_SIZE: c_uint = 0x10;

pub const SIS_PLAY_DMA_FORMAT_CSO: c_uint = 0x00;
pub const SIS_PLAY_DMA_FORMAT_UNSIGNED: c_uint = 0x00080000;
pub const SIS_PLAY_DMA_FORMAT_8BIT: c_uint = 0x00040000;
pub const SIS_PLAY_DMA_FORMAT_MONO: c_uint = 0x00020000;
pub const SIS_PLAY_DMA_CSO_MASK: c_uint = 0x0000ffff;
pub const SIS_PLAY_DMA_BASE: c_uint = 0x04;
pub const SIS_PLAY_DMA_CONTROL: c_uint = 0x08;
pub const SIS_PLAY_DMA_STOP_AT_SSO: c_uint = 0x04000000;
pub const SIS_PLAY_DMA_RELEASE: c_uint = 0x02000000;
pub const SIS_PLAY_DMA_LOOP: c_uint = 0x01000000;
pub const SIS_PLAY_DMA_INTR_AT_SSO: c_uint = 0x00080000;
pub const SIS_PLAY_DMA_INTR_AT_ESO: c_uint = 0x00040000;
pub const SIS_PLAY_DMA_INTR_AT_LEO: c_uint = 0x00020000;
pub const SIS_PLAY_DMA_INTR_AT_MLP: c_uint = 0x00010000;
pub const SIS_PLAY_DMA_LEO_MASK: c_uint = 0x0000ffff;
pub const SIS_PLAY_DMA_SSO_ESO: c_uint = 0x0c;
pub const SIS_PLAY_DMA_SSO_MASK: c_uint = 0xffff0000;
pub const SIS_PLAY_DMA_ESO_MASK: c_uint = 0x0000ffff;
// Capture DMA parameters (parameter RAM)
pub const SIS_CAPTURE_DMA_OFFSET: c_uint = 0x0800;
pub const SIS_CAPTURE_DMA_SIZE: c_uint = 0x10;

pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_0: c_int = 0;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_1: c_int = 1;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_2: c_int = 2;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_3: c_int = 3;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_4: c_int = 4;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_5: c_int = 5;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_6: c_int = 6;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_7: c_int = 7;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_8: c_int = 8;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_9: c_int = 9;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_10: c_int = 10;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_11: c_int = 11;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_12: c_int = 12;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_13: c_int = 13;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_14: c_int = 14;
pub const SIS_CAPTURE_CHAN_MIXER_ROUTE_BACK_15: c_int = 15;
pub const SIS_CAPTURE_CHAN_AC97_PCM_IN: c_int = 16;
pub const SIS_CAPTURE_CHAN_AC97_MIC_IN: c_int = 17;
pub const SIS_CAPTURE_CHAN_AC97_LINE1_IN: c_int = 18;
pub const SIS_CAPTURE_CHAN_AC97_LINE2_IN: c_int = 19;
pub const SIS_CAPTURE_CHAN_AC97_HANDSE_IN: c_int = 20;
pub const SIS_CAPTURE_DMA_FORMAT_CSO: c_uint = 0x00;
pub const SIS_CAPTURE_DMA_MONO_MODE_MASK: c_uint = 0xc0000000;
pub const SIS_CAPTURE_DMA_MONO_MODE_AVG: c_uint = 0x00000000;
pub const SIS_CAPTURE_DMA_MONO_MODE_LEFT: c_uint = 0x40000000;
pub const SIS_CAPTURE_DMA_MONO_MODE_RIGHT: c_uint = 0x80000000;
pub const SIS_CAPTURE_DMA_FORMAT_UNSIGNED: c_uint = 0x00080000;
pub const SIS_CAPTURE_DMA_FORMAT_8BIT: c_uint = 0x00040000;
pub const SIS_CAPTURE_DMA_FORMAT_MONO: c_uint = 0x00020000;
pub const SIS_CAPTURE_DMA_CSO_MASK: c_uint = 0x0000ffff;
pub const SIS_CAPTURE_DMA_BASE: c_uint = 0x04;
pub const SIS_CAPTURE_DMA_CONTROL: c_uint = 0x08;
pub const SIS_CAPTURE_DMA_STOP_AT_SSO: c_uint = 0x04000000;
pub const SIS_CAPTURE_DMA_RELEASE: c_uint = 0x02000000;
pub const SIS_CAPTURE_DMA_LOOP: c_uint = 0x01000000;
pub const SIS_CAPTURE_DMA_INTR_AT_LEO: c_uint = 0x00020000;
pub const SIS_CAPTURE_DMA_INTR_AT_MLP: c_uint = 0x00010000;
pub const SIS_CAPTURE_DMA_LEO_MASK: c_uint = 0x0000ffff;
pub const SIS_CAPTURE_DMA_RESERVED: c_uint = 0x0c;
// Mixer routing list start pointer (parameter RAM)
pub const SIS_MIXER_START_OFFSET: c_uint = 0x1000;
pub const SIS_MIXER_START_SIZE: c_uint = 0x04;

pub const SIS_MIXER_START_MASK: c_uint = 0x0000007f;
// Mixer routing table (parameter RAM)
pub const SIS_MIXER_OFFSET: c_uint = 0x1400;
pub const SIS_MIXER_SIZE: c_uint = 0x04;

pub const SIS_MIXER_RIGHT_ATTENUTATION_MASK: c_uint = 0xff000000;
pub const SIS_MIXER_RIGHT_NO_ATTEN: c_uint = 0xff000000;
pub const SIS_MIXER_LEFT_ATTENUTATION_MASK: c_uint = 0x00ff0000;
pub const SIS_MIXER_LEFT_NO_ATTEN: c_uint = 0x00ff0000;
pub const SIS_MIXER_NEXT_ENTRY_MASK: c_uint = 0x00007f00;
pub const SIS_MIXER_NEXT_ENTRY_NONE: c_uint = 0x00000000;
pub const SIS_MIXER_DEST_MASK: c_uint = 0x0000007f;
pub const SIS_MIXER_DEST_0: c_uint = 0x00000020;
pub const SIS_MIXER_DEST_1: c_uint = 0x00000021;
pub const SIS_MIXER_DEST_2: c_uint = 0x00000022;
pub const SIS_MIXER_DEST_3: c_uint = 0x00000023;
pub const SIS_MIXER_DEST_4: c_uint = 0x00000024;
pub const SIS_MIXER_DEST_5: c_uint = 0x00000025;
pub const SIS_MIXER_DEST_6: c_uint = 0x00000026;
pub const SIS_MIXER_DEST_7: c_uint = 0x00000027;
pub const SIS_MIXER_DEST_8: c_uint = 0x00000028;
pub const SIS_MIXER_DEST_9: c_uint = 0x00000029;
pub const SIS_MIXER_DEST_10: c_uint = 0x0000002a;
pub const SIS_MIXER_DEST_11: c_uint = 0x0000002b;
pub const SIS_MIXER_DEST_12: c_uint = 0x0000002c;
pub const SIS_MIXER_DEST_13: c_uint = 0x0000002d;
pub const SIS_MIXER_DEST_14: c_uint = 0x0000002e;
pub const SIS_MIXER_DEST_15: c_uint = 0x0000002f;
// Wave Engine Control Parameters (parameter RAM)
pub const SIS_WAVE_OFFSET: c_uint = 0x2000;
pub const SIS_WAVE_SIZE: c_uint = 0x40;

pub const SIS_WAVE_GENERAL: c_uint = 0x00;
pub const SIS_WAVE_GENERAL_WAVE_VOLUME: c_uint = 0x80000000;
pub const SIS_WAVE_GENERAL_MUSIC_VOLUME: c_uint = 0x00000000;
pub const SIS_WAVE_GENERAL_VOLUME_MASK: c_uint = 0x7f000000;
pub const SIS_WAVE_GENERAL_ARTICULATION: c_uint = 0x04;
pub const SIS_WAVE_GENERAL_ARTICULATION_DELTA_MASK: c_uint = 0x3fff0000;
pub const SIS_WAVE_ARTICULATION: c_uint = 0x08;
pub const SIS_WAVE_TIMER: c_uint = 0x0c;
pub const SIS_WAVE_GENERATOR: c_uint = 0x10;
pub const SIS_WAVE_CHANNEL_CONTROL: c_uint = 0x14;
pub const SIS_WAVE_CHANNEL_CONTROL_FIRST_SAMPLE: c_uint = 0x80000000;
pub const SIS_WAVE_CHANNEL_CONTROL_AMP_ENABLE: c_uint = 0x40000000;
pub const SIS_WAVE_CHANNEL_CONTROL_FILTER_ENABLE: c_uint = 0x20000000;
pub const SIS_WAVE_CHANNEL_CONTROL_INTERPOLATE_ENABLE: c_uint = 0x10000000;
pub const SIS_WAVE_LFO_EG_CONTROL: c_uint = 0x18;
pub const SIS_WAVE_LFO_EG_CONTROL_2: c_uint = 0x1c;
pub const SIS_WAVE_LFO_EG_CONTROL_3: c_uint = 0x20;
pub const SIS_WAVE_LFO_EG_CONTROL_4: c_uint = 0x24;
