//! Automatically rewritten from C to Rust
//! Source: sound/firewire/dice/dice-mytek.c
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
// dice-mytek.c - a part of driver for DICE based devices
//
// Copyright (c) 2018 Melvin Vermeeren
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dice_mytek_spec {
    pub tx_pcm_chs: [c_uint; MAX_STREAMS][SND_DICE_RATE_MODE_COUNT],
    pub rx_pcm_chs: [c_uint; MAX_STREAMS][SND_DICE_RATE_MODE_COUNT],
}

    static const struct dice_mytek_spec stereo_192_dsd_dac = {
// AES, TOSLINK, SPDIF, ADAT inputs on device
    .tx_pcm_chs = {{8, 8, 8}, {0, 0, 0} },
// PCM 44.1-192, native DSD64/DSD128 to device
    .rx_pcm_chs = {{4, 4, 4}, {0, 0, 0} }
    };
//
// Mytek has a few other firewire-capable devices, though newer models appear
// to lack the port more often than not. As I don't have access to any of them
// they are missing here. An example is the Mytek 8x192 ADDA, which is DICE.
//
#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_mytek_formats(dice: *mut snd_dice) -> c_int {
    int snd_dice_detect_mytek_formats(struct snd_dice *dice)
    {
    int i;
    const struct dice_mytek_spec *dev;
    dev = &stereo_192_dsd_dac;
    memcpy(dice.tx_pcm_chs, dev.tx_pcm_chs,
    MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * sizeof(unsigned int));
    memcpy(dice.rx_pcm_chs, dev.rx_pcm_chs,
    MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * sizeof(unsigned int));
    for (i = 0; i < MAX_STREAMS; ++i) {
    dice.tx_midi_ports[i] = 0;
    dice.rx_midi_ports[i] = 0;
    }
    return 0;
    }
