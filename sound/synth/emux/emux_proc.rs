//! Automatically rewritten from C to Rust
//! Source: sound/synth/emux/emux_proc.c
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
// Proc interface for Emu8k/Emu10k1 WaveTable synth
//

    static void
    snd_emux_proc_info_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buf)
    {
    struct snd_emux *emu;
    int i;
    emu = entry.private_data;
    guard(mutex)(&emu.register_mutex);
    if (emu.name)
    snd_iprintf(buf, "Device: %s\n", emu.name);
    snd_iprintf(buf, "Ports: %d\n", emu.num_ports);
    snd_iprintf(buf, "Addresses:");
    for (i = 0; i < emu.num_ports; i++)
    snd_iprintf(buf, " %d:%d", emu.client, emu.ports[i]);
    snd_iprintf(buf, "\n");
    snd_iprintf(buf, "Use Counter: %d\n", emu.used);
    snd_iprintf(buf, "Max Voices: %d\n", emu.max_voices);
    snd_iprintf(buf, "Allocated Voices: %d\n", emu.num_voices);
    if (emu.memhdr) {
    snd_iprintf(buf, "Memory Size: %d\n", emu.memhdr.size);
    snd_iprintf(buf, "Memory Available: %d\n", snd_util_mem_avail(emu.memhdr));
    snd_iprintf(buf, "Allocated Blocks: %d\n", emu.memhdr.nblocks);
    } else {
    snd_iprintf(buf, "Memory Size: 0\n");
    }
    if (emu.sflist) {
    guard(mutex)(&emu.sflist.presets_mutex);
    snd_iprintf(buf, "SoundFonts: %d\n", emu.sflist.fonts_size);
    snd_iprintf(buf, "Instruments: %d\n", emu.sflist.zone_counter);
    snd_iprintf(buf, "Samples: %d\n", emu.sflist.sample_counter);
    snd_iprintf(buf, "Locked Instruments: %d\n", emu.sflist.zone_locked);
    snd_iprintf(buf, "Locked Samples: %d\n", emu.sflist.sample_locked);
    }

    if (emu.voices[0].state != SNDRV_EMUX_ST_OFF && emu.voices[0].ch >= 0) {
    struct snd_emux_voice *vp = &emu.voices[0];
    snd_iprintf(buf, "voice 0: on\n");
    snd_iprintf(buf, "mod delay=%x, atkhld=%x, dcysus=%x, rel=%x\n",
    vp.reg.parm.moddelay,
    vp.reg.parm.modatkhld,
    vp.reg.parm.moddcysus,
    vp.reg.parm.modrelease);
    snd_iprintf(buf, "vol delay=%x, atkhld=%x, dcysus=%x, rel=%x\n",
    vp.reg.parm.voldelay,
    vp.reg.parm.volatkhld,
    vp.reg.parm.voldcysus,
    vp.reg.parm.volrelease);
    snd_iprintf(buf, "lfo1 delay=%x, lfo2 delay=%x, pefe=%x\n",
    vp.reg.parm.lfo1delay,
    vp.reg.parm.lfo2delay,
    vp.reg.parm.pefe);
    snd_iprintf(buf, "fmmod=%x, tremfrq=%x, fm2frq2=%x\n",
    vp.reg.parm.fmmod,
    vp.reg.parm.tremfrq,
    vp.reg.parm.fm2frq2);
    snd_iprintf(buf, "cutoff=%x, filterQ=%x, chorus=%x, reverb=%x\n",
    vp.reg.parm.cutoff,
    vp.reg.parm.filterQ,
    vp.reg.parm.chorus,
    vp.reg.parm.reverb);
    snd_iprintf(buf, "avol=%x, acutoff=%x, apitch=%x\n",
    vp.avol, vp.acutoff, vp.apitch);
    snd_iprintf(buf, "apan=%x, aaux=%x, ptarget=%x, vtarget=%x, ftarget=%x\n",
    vp.apan, vp.aaux,
    vp.ptarget,
    vp.vtarget,
    vp.ftarget);
    snd_iprintf(buf, "start=%x, end=%x, loopstart=%x, loopend=%x\n",
    vp.reg.start, vp.reg.end, vp.reg.loopstart, vp.reg.loopend);
    snd_iprintf(buf, "sample_mode=%x, rate=%x\n", vp.reg.sample_mode, vp.reg.rate_offset);
    }

    }
#[no_mangle]
pub unsafe extern "C" fn snd_emux_proc_init(emu: *mut snd_emux, card: *mut snd_card, device: c_int) {
    void snd_emux_proc_init(struct snd_emux *emu, struct snd_card *card, int device)
    {
    struct snd_info_entry *entry;
    char name[64];
    sprintf(name, "wavetableD%d", device);
    entry = snd_info_create_card_entry(card, name, card.proc_root);
    if (entry == core::ptr::null_mut())
    return;
    entry.content = SNDRV_INFO_CONTENT_TEXT;
    entry.private_data = emu;
    entry.c.text.read = snd_emux_proc_info_read;
    emu.proc = entry;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_emux_proc_free(emu: *mut snd_emux) {
    void snd_emux_proc_free(struct snd_emux *emu)
    {
    snd_info_free_entry(emu.proc);
    emu.proc = core::ptr::null_mut();
    }
