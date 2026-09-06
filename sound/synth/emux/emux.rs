//! Automatically rewritten from C to Rust
//! Source: sound/synth/emux/emux.c
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
// Routines for control of EMU WaveTable chip
//

    MODULE_AUTHOR("Takashi Iwai");
    MODULE_DESCRIPTION("Routines for control of EMU WaveTable chip");
    MODULE_LICENSE("GPL");
//
// create a new hardware dependent device for Emu8000/Emu10k1
//
#[no_mangle]
pub unsafe extern "C" fn snd_emux_new(remu: *mut snd_emux) -> c_int {
    int snd_emux_new(struct snd_emux **remu)
    {
    struct snd_emux *emu;
// remu = NULL;
    emu = kzalloc_obj(*emu);
    if (emu == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&emu.voice_lock);
    mutex_init(&emu.register_mutex);
    emu.client = -1;

    emu.oss_synth = core::ptr::null_mut();

    emu.max_voices = 0;
    emu.use_time = 0;
    timer_setup(&emu.tlist, snd_emux_timer_callback, 0);
    emu.timer_active = 0;
// remu = emu;
    return 0;
    }
    EXPORT_SYMBOL(snd_emux_new);
//
    static int sf_sample_new(void *private_data, struct snd_sf_sample *sp,
    struct snd_util_memhdr *hdr,
    const void __user *buf, long count)
    {
    struct snd_emux *emu = private_data;
    return emu.ops.sample_new(emu, sp, hdr, buf, count);
    }
    static int sf_sample_free(void *private_data, struct snd_sf_sample *sp,
    struct snd_util_memhdr *hdr)
    {
    struct snd_emux *emu = private_data;
    return emu.ops.sample_free(emu, sp, hdr);
    }
#[no_mangle]
unsafe extern "C" fn sf_sample_reset(private_data: *mut c_void) {
    static void sf_sample_reset(void *private_data)
    {
    struct snd_emux *emu = private_data;
    emu.ops.sample_reset(emu);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_emux_register(emu: *mut snd_emux, card: *mut snd_card, index: c_int, name: *mut c_char) -> c_int {
    int snd_emux_register(struct snd_emux *emu, struct snd_card *card, int index, char *name)
    {
    int err;
    struct snd_sf_callback sf_cb;
    if (snd_BUG_ON(!emu.hw || emu.max_voices <= 0))
    return -EINVAL;
    if (snd_BUG_ON(!card || !name))
    return -EINVAL;
    emu.card = card;
    emu.name = kstrdup_const(name, GFP_KERNEL);
    emu.voices = kzalloc_objs(struct snd_emux_voice, emu.max_voices);
    if (emu.name == core::ptr::null_mut() || emu.voices == core::ptr::null_mut())
    return -ENOMEM;
// create soundfont list
    memset(&sf_cb, 0, sizeof(sf_cb));
    sf_cb.private_data = emu;
    sf_cb.sample_new = sf_sample_new;
    sf_cb.sample_free = sf_sample_free;
    if (emu.ops.sample_reset)
    sf_cb.sample_reset = sf_sample_reset;
    emu.sflist = snd_sf_new(&sf_cb, emu.memhdr);
    if (emu.sflist == core::ptr::null_mut())
    return -ENOMEM;
    err = snd_emux_init_hwdep(emu);
    if (err < 0)
    return err;
    snd_emux_init_voices(emu);
    snd_emux_init_seq(emu, card, index);

    snd_emux_init_seq_oss(emu);

    snd_emux_init_virmidi(emu, card);
    snd_emux_proc_init(emu, card, index);
    return 0;
    }
    EXPORT_SYMBOL(snd_emux_register);
//
#[no_mangle]
pub unsafe extern "C" fn snd_emux_free(emu: *mut snd_emux) -> c_int {
    int snd_emux_free(struct snd_emux *emu)
    {
    if (! emu)
    return -EINVAL;
    timer_shutdown_sync(&emu.tlist);
    snd_emux_proc_free(emu);
    snd_emux_delete_virmidi(emu);

    snd_emux_detach_seq_oss(emu);

    snd_emux_detach_seq(emu);
    snd_emux_delete_hwdep(emu);
    snd_sf_free(emu.sflist);
    kfree(emu.voices);
    kfree_const(emu.name);
    kfree(emu);
    return 0;
    }
    EXPORT_SYMBOL(snd_emux_free);
