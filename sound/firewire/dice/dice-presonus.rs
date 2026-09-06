//! Automatically rewritten from C to Rust
//! Source: sound/firewire/dice/dice-presonus.c
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
// dice-presonus.c - a part of driver for DICE based devices
//
// Copyright (c) 2019 Takashi Sakamoto

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dice_presonus_spec {
    pub tx_pcm_chs: [c_uint; MAX_STREAMS][SND_DICE_RATE_MODE_COUNT],
    pub rx_pcm_chs: [c_uint; MAX_STREAMS][SND_DICE_RATE_MODE_COUNT],
    pub has_midi: bool,
}

    static const struct dice_presonus_spec dice_presonus_firesutio = {
    .tx_pcm_chs = {{16, 16, 0}, {10, 2, 0} },
    .rx_pcm_chs = {{16, 16, 0}, {10, 2, 0} },
    .has_midi = true,
    };
#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_presonus_formats(dice: *mut snd_dice) -> c_int {
    int snd_dice_detect_presonus_formats(struct snd_dice *dice)
    {
    static const struct {
    u32 model_id;
    const struct dice_presonus_spec *spec;
    } *entry, entries[] = {
    {0x000008, &dice_presonus_firesutio},
    };
    struct fw_csr_iterator it;
    int key, val, model_id;
    int i;
    model_id = 0;
    fw_csr_iterator_init(&it, dice.unit.directory);
    while (fw_csr_iterator_next(&it, &key, &val)) {
    if (key == CSR_MODEL) {
    model_id = val;
    break;
    }
    }
    for (i = 0; i < ARRAY_SIZE(entries); ++i) {
    entry = entries + i;
    if (entry.model_id == model_id)
    break;
    }
    if (i == ARRAY_SIZE(entries))
    return -ENODEV;
    memcpy(dice.tx_pcm_chs, entry.spec.tx_pcm_chs,
    MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * sizeof(unsigned int));
    memcpy(dice.rx_pcm_chs, entry.spec.rx_pcm_chs,
    MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * sizeof(unsigned int));
    if (entry.spec.has_midi) {
    dice.tx_midi_ports[0] = 1;
    dice.rx_midi_ports[0] = 1;
    }
    return 0;
    }
