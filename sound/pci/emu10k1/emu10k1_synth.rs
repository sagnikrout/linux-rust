//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/emu10k1_synth.c
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
// Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
//
// Routines for control of EMU10K1 WaveTable synth
//

    MODULE_AUTHOR("Takashi Iwai");
    MODULE_DESCRIPTION("Routines for control of EMU10K1 WaveTable synth");
    MODULE_LICENSE("GPL");
//
// create a new hardware dependent device for Emu10k1
//
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_synth_probe(dev: *mut snd_seq_device) -> c_int {
    static int snd_emu10k1_synth_probe(struct snd_seq_device *dev)
    {
    struct snd_emux *emux;
    struct snd_emu10k1 *hw;
    struct snd_emu10k1_synth_arg *arg;
    arg = SNDRV_SEQ_DEVICE_ARGPTR(dev);
    if (arg == core::ptr::null_mut())
    return -EINVAL;
    if (arg.seq_ports <= 0)
    return 0; /* nothing */
    if (arg.max_voices < 1)
    arg.max_voices = 1;
#[no_mangle]
pub unsafe extern "C" fn if(64: arg->max_voices >) -> else {
    else if (arg.max_voices > 64)
    arg.max_voices = 64;
    if (snd_emux_new(&emux) < 0)
    return -ENOMEM;
    snd_emu10k1_ops_setup(emux);
    hw = arg.hwptr;
    emux.hw = hw;
    emux.max_voices = arg.max_voices;
    emux.num_ports = arg.seq_ports;
    emux.memhdr = hw.memhdr;
// maximum two ports
    emux.midi_ports = arg.seq_ports < 2 ? arg.seq_ports : 2;
// audigy has two external midis
    emux.midi_devidx = hw.audigy ? 2 : 1;
    emux.linear_panning = 0;
    emux.hwdep_idx = 2; /* FIXED */
    if (snd_emux_register(emux, dev.card, arg.index, "Emu10k1") < 0) {
    snd_emux_free(emux);
    return -ENOMEM;
    }
    guard(spinlock_irq)(&hw.voice_lock);
    hw.synth = emux;
    hw.get_synth_voice = snd_emu10k1_synth_get_voice;
    dev.driver_data = emux;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_synth_remove(dev: *mut snd_seq_device) {
    static void snd_emu10k1_synth_remove(struct snd_seq_device *dev)
    {
    struct snd_emux *emux;
    struct snd_emu10k1 *hw;
    if (dev.driver_data == core::ptr::null_mut())
    return; /* not registered actually */
    emux = dev.driver_data;
    hw = emux.hw;
    scoped_guard(spinlock_irq, &hw.voice_lock) {
    hw.synth = core::ptr::null_mut();
    hw.get_synth_voice = core::ptr::null_mut();
    }
    snd_emux_free(emux);
    }
//
// INIT part
//
    static struct snd_seq_driver emu10k1_synth_driver = {
    .probe = snd_emu10k1_synth_probe,
    .remove = snd_emu10k1_synth_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    .id = SNDRV_SEQ_DEV_ID_EMU10K1_SYNTH,
    .argsize = sizeof(struct snd_emu10k1_synth_arg),
    };
    module_snd_seq_driver(emu10k1_synth_driver);
