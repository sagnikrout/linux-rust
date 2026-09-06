//! Automatically rewritten from C to Rust
//! Source: sound/firewire/dice/dice-focusrite.c
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
// dice-focusrite.c - a part of driver for DICE based devices
//
// Copyright (c) 2022 Takashi Sakamoto

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_focusrite_pro40_tcd3070_formats(dice: *mut snd_dice) -> c_int {
    int snd_dice_detect_focusrite_pro40_tcd3070_formats(struct snd_dice *dice)
    {
// Focusrite shipped several variants of Saffire Pro 40. One of them is based on TCD3070-CH
// apart from the others with TCD2220. It doesn't support TCAT protocol extension.
    dice.tx_pcm_chs[0][0] = 20;
    dice.tx_midi_ports[0] = 1;
    dice.rx_pcm_chs[0][0] = 20;
    dice.rx_midi_ports[0] = 1;
    dice.tx_pcm_chs[0][1] = 16;
    dice.tx_midi_ports[1] = 1;
    dice.rx_pcm_chs[0][1] = 16;
    dice.rx_midi_ports[1] = 1;
    return 0;
    }
