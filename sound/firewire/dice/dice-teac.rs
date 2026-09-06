//! Automatically rewritten from C to Rust
//! Source: sound/firewire/dice/dice-teac.c
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
// dice-teac.c - a part of driver for DICE based devices
//
// Copyright (c) 2025 Takashi Sakamoto

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_teac_formats(dice: *mut snd_dice) -> c_int {
    int snd_dice_detect_teac_formats(struct snd_dice *dice)
    {
    __be32 reg;
    u32 data;
    int err;
    err = snd_dice_transaction_read_tx(dice, TX_NUMBER, &reg, sizeof(reg));
    if (err  < 0)
    return err;
    dice.tx_pcm_chs[0][SND_DICE_RATE_MODE_LOW] = 16;
    dice.tx_pcm_chs[0][SND_DICE_RATE_MODE_MIDDLE] = 16;
    dice.tx_midi_ports[0] = 1;
    data = be32_to_cpu(reg);
    if (data > 1) {
    dice.tx_pcm_chs[1][SND_DICE_RATE_MODE_LOW] = 16;
    dice.tx_pcm_chs[1][SND_DICE_RATE_MODE_MIDDLE] = 16;
    }
    err = snd_dice_transaction_read_rx(dice, RX_NUMBER, &reg, sizeof(reg));
    if (err  < 0)
    return err;
    dice.rx_pcm_chs[0][SND_DICE_RATE_MODE_LOW] = 16;
    dice.rx_pcm_chs[0][SND_DICE_RATE_MODE_MIDDLE] = 16;
    dice.rx_midi_ports[0] = 1;
    data = be32_to_cpu(reg);
    if (data > 1) {
    dice.rx_pcm_chs[1][SND_DICE_RATE_MODE_LOW] = 16;
    dice.rx_pcm_chs[1][SND_DICE_RATE_MODE_MIDDLE] = 16;
    }
    return 0;
    }
