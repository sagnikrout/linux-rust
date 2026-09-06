//! Automatically rewritten from C to Rust
//! Source: sound/last.c
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
// Advanced Linux Sound Architecture
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

#[no_mangle]
unsafe extern "C" fn alsa_sound_last_init() -> int __init {
    static int __init alsa_sound_last_init(void)
    {
    struct snd_card *card;
    int idx, ok = 0;
    printk(KERN_INFO "ALSA device list:\n");
    for (idx = 0; idx < SNDRV_CARDS; idx++) {
    card = snd_card_ref(idx);
    if (card) {
    printk(KERN_INFO "  #%i: %s\n", idx, card.longname);
    snd_card_unref(card);
    ok++;
    }
    }
    if (ok == 0)
    printk(KERN_INFO "  No soundcards found.\n");
    return 0;
    }
    late_initcall_sync(alsa_sound_last_init);
