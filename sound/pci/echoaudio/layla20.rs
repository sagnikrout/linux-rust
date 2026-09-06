//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/layla20.c
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
// Macro flag: #define ECHOGALS_FAMILY
// Macro flag: #define ECHOCARD_LAYLA20

// Macro flag: #define ECHOCARD_HAS_MONITOR
// Macro flag: #define ECHOCARD_HAS_ASIC
// Macro flag: #define ECHOCARD_HAS_INPUT_GAIN
// Macro flag: #define ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL
// Macro flag: #define ECHOCARD_HAS_SUPER_INTERLEAVE
// Macro flag: #define ECHOCARD_HAS_DIGITAL_IO
// Macro flag: #define ECHOCARD_HAS_EXTERNAL_CLOCK

// Macro flag: #define ECHOCARD_HAS_OUTPUT_CLOCK_SWITCH
// Macro flag: #define ECHOCARD_HAS_MIDI
// Pipe indexes

pub const PX_NUM: c_int = 22;
// Bus indexes

pub const BX_NUM: c_int = 22;

    MODULE_FIRMWARE("ea/layla20_dsp.fw");
    MODULE_FIRMWARE("ea/layla20_asic.fw");
pub const FW_LAYLA20_DSP: c_int = 0;
pub const FW_LAYLA20_ASIC: c_int = 1;
    static const struct firmware card_fw[] = {
    {0, "layla20_dsp.fw"},
    {0, "layla20_asic.fw"}
    };
    static const struct pci_device_id snd_echo_ids[] = {
    { PCI_DEVICE_SUB(0x1057, 0x1801, 0xECC0, 0x0030) },	/* DSP 56301 Layla20 rev.0 */
    { PCI_DEVICE_SUB(0x1057, 0x1801, 0xECC0, 0x0031) },	/* DSP 56301 Layla20 rev.1 */
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
    .rates = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_CONTINUOUS,
    .rate_min = 8000,
    .rate_max = 50000,
    .channels_min = 1,
    .channels_max = 10,
    .buffer_bytes_max = 262144,
    .period_bytes_min = 32,
    .period_bytes_max = 131072,
    .periods_min = 2,
    .periods_max = 220,
// One page (4k) contains 512 instructions. I don't know if the hw
    supports lists longer than this. In this case periods_max=220 is a
    safe limit to make sure the list never exceeds 512 instructions. */
    };

