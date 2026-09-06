//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/voice.c
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Lee Revell <rlrevell@joe-job.com>
// Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
// Creative Labs, Inc.
//
// Routines for control of EMU10K1 chips - voice manager
//

// Previously the voice allocator started at 0 every time.  The new voice
// allocator uses a round robin scheme.  The next free voice is tracked in
// the card record and each allocation begins where the last left off.  The
// hardware requires stereo interleaved voices be aligned to an even/odd
// boundary.
// --rlrevell
//
    static int voice_alloc(struct snd_emu10k1 *emu, int type, int number,
    struct snd_emu10k1_pcm *epcm, struct snd_emu10k1_voice **rvoice)
    {
    struct snd_emu10k1_voice *voice;
    int i, j, k, skip;
    for (i = emu.next_free_voice, j = 0; j < NUM_G; i = (i + skip) % NUM_G, j += skip) {
//
    dev_dbg(emu.card.dev, "i %d j %d next free %d!\n",
    i, j, emu.next_free_voice);
//
// stereo voices must be even/odd
    if ((number > 1) && (i % 2)) {
    skip = 1;
    continue;
    }
    for (k = 0; k < number; k++) {
    voice = &emu.voices[i + k];
    if (voice.use) {
    skip = k + 1;
    goto next;
    }
    }
    for (k = 0; k < number; k++) {
    voice = &emu.voices[i + k];
    voice.use = type;
    voice.epcm = epcm;
// dev_dbg(emu->card->dev, "allocated voice %d\n", i + k);
    }
    voice.last = 1;
// rvoice = &emu->voices[i];
    emu.next_free_voice = (i + number) % NUM_G;
    return 0;
    next: ;
    }
    return -ENOMEM;  // -EBUSY would have been better
    }
    static void voice_free(struct snd_emu10k1 *emu,
    struct snd_emu10k1_voice *pvoice)
    {
    if (pvoice.dirty)
    snd_emu10k1_voice_init(emu, pvoice.number);
    pvoice.interrupt = core::ptr::null_mut();
    pvoice.use = pvoice.dirty = pvoice.last = 0;
    pvoice.epcm = core::ptr::null_mut();
    }
    int snd_emu10k1_voice_alloc(struct snd_emu10k1 *emu, int type, int count, int channels,
    struct snd_emu10k1_pcm *epcm, struct snd_emu10k1_voice **rvoice)
    {
    int result;
    if (snd_BUG_ON(!rvoice))
    return -EINVAL;
    if (snd_BUG_ON(!count))
    return -EINVAL;
    if (snd_BUG_ON(!channels))
    return -EINVAL;
    guard(spinlock_irqsave)(&emu.voice_lock);
    for (int got = 0; got < channels; ) {
    result = voice_alloc(emu, type, count, epcm, &rvoice[got]);
    if (result == 0) {
    got++;
//
    dev_dbg(emu.card.dev, "voice alloc - %i, %i of %i\n",
    rvoice[got - 1].number, got, want);
//
    continue;
    }
    if (type != EMU10K1_SYNTH && emu.get_synth_voice) {
// free a voice from synth
    result = emu.get_synth_voice(emu);
    if (result >= 0) {
    voice_free(emu, &emu.voices[result]);
    continue;
    }
    }
    for (int i = 0; i < got; i++) {
    for (int j = 0; j < count; j++)
    voice_free(emu, rvoice[i] + j);
    rvoice[i] = core::ptr::null_mut();
    }
    break;
    }
    return result;
    }
    EXPORT_SYMBOL(snd_emu10k1_voice_alloc);
    int snd_emu10k1_voice_free(struct snd_emu10k1 *emu,
    struct snd_emu10k1_voice *pvoice)
    {
    int last;
    if (snd_BUG_ON(!pvoice))
    return -EINVAL;
    guard(spinlock_irqsave)(&emu.voice_lock);
    do {
    last = pvoice.last;
    voice_free(emu, pvoice++);
    } while (!last);
    return 0;
    }
    EXPORT_SYMBOL(snd_emu10k1_voice_free);
