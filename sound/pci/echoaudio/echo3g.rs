//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/echo3g.c
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
// ALSA driver for Echoaudio soundcards.
// Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
//
// Macro flag: #define ECHO3G_FAMILY
// Macro flag: #define ECHOCARD_ECHO3G

// Macro flag: #define ECHOCARD_HAS_MONITOR
// Macro flag: #define ECHOCARD_HAS_ASIC
// Macro flag: #define ECHOCARD_HAS_INPUT_NOMINAL_LEVEL
// Macro flag: #define ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL
// Macro flag: #define ECHOCARD_HAS_SUPER_INTERLEAVE
// Macro flag: #define ECHOCARD_HAS_DIGITAL_IO
// Macro flag: #define ECHOCARD_HAS_DIGITAL_MODE_SWITCH
pub const ECHOCARD_HAS_ADAT: c_int = 6;
// Macro flag: #define ECHOCARD_HAS_EXTERNAL_CLOCK
// Macro flag: #define ECHOCARD_HAS_STEREO_BIG_ENDIAN32
// Macro flag: #define ECHOCARD_HAS_MIDI
// Macro flag: #define ECHOCARD_HAS_PHANTOM_POWER
// Pipe indexes
pub const PX_ANALOG_OUT: c_int = 0;

// Bus indexes
pub const BX_ANALOG_OUT: c_int = 0;

    MODULE_FIRMWARE("ea/loader_dsp.fw");
    MODULE_FIRMWARE("ea/echo3g_dsp.fw");
    MODULE_FIRMWARE("ea/3g_asic.fw");
pub const FW_361_LOADER: c_int = 0;
pub const FW_ECHO3G_DSP: c_int = 1;
pub const FW_3G_ASIC: c_int = 2;
    static const struct firmware card_fw[] = {
    {0, "loader_dsp.fw"},
    {0, "echo3g_dsp.fw"},
    {0, "3g_asic.fw"}
    };
    static const struct pci_device_id snd_echo_ids[] = {
    { PCI_DEVICE_SUB(0x1057, 0x3410, 0xECC0, 0x0100) },	/* Echo 3G */
    { }
    };
    static const struct snd_pcm_hardware pcm_hardware_skel = {
    .info = SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_SYNC_START,
    .formats =	SNDRV_PCM_FMTBIT_U8 |
    SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_3LE |
    SNDRV_PCM_FMTBIT_S32_LE |
    SNDRV_PCM_FMTBIT_S32_BE,
    .rates = 	SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_CONTINUOUS,
    .rate_min = 32000,
    .rate_max = 100000,
    .channels_min = 1,
    .channels_max = 8,
    .buffer_bytes_max = 262144,
    .period_bytes_min = 32,
    .period_bytes_max = 131072,
    .periods_min = 2,
    .periods_max = 220,
    };

