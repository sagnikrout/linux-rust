//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-midi.c
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
// motu-midi.h - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

#[no_mangle]
unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_motu *motu = substream.rmidi.private_data;
    int err;
    err = snd_motu_stream_lock_try(motu);
    if (err < 0)
    return err;
    scoped_guard(mutex, &motu.mutex) {
    err = snd_motu_stream_reserve_duplex(motu, 0, 0, 0);
    if (err >= 0) {
    ++motu.substreams_counter;
    err = snd_motu_stream_start_duplex(motu);
    if (err < 0)
    --motu.substreams_counter;
    }
    }
    if (err < 0)
    snd_motu_stream_lock_release(motu);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_motu *motu = substream.rmidi.private_data;
    scoped_guard(mutex, &motu.mutex) {
    --motu.substreams_counter;
    snd_motu_stream_stop_duplex(motu);
    }
    snd_motu_stream_lock_release(motu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_capture_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_motu *motu = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&motu.lock);
    if (up)
    amdtp_motu_midi_trigger(&motu.tx_stream, substrm.number,
    substrm);
    else
    amdtp_motu_midi_trigger(&motu.tx_stream, substrm.number,
    core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_playback_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_motu *motu = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&motu.lock);
    if (up)
    amdtp_motu_midi_trigger(&motu.rx_stream, substrm.number,
    substrm);
    else
    amdtp_motu_midi_trigger(&motu.rx_stream, substrm.number,
    core::ptr::null_mut());
    }
    static void set_midi_substream_names(struct snd_motu *motu,
    struct snd_rawmidi_str *str)
    {
    struct snd_rawmidi_substream *subs;
    list_for_each_entry(subs, &str.substreams, list) {
    scnprintf(subs.name, sizeof(subs.name),
    "%s MIDI %d", motu.card.shortname, subs.number + 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_create_midi_devices(motu: *mut snd_motu) -> c_int {
    int snd_motu_create_midi_devices(struct snd_motu *motu)
    {
    static const struct snd_rawmidi_ops capture_ops = {
    .open		= midi_open,
    .close		= midi_close,
    .trigger	= midi_capture_trigger,
    };
    static const struct snd_rawmidi_ops playback_ops = {
    .open		= midi_open,
    .close		= midi_close,
    .trigger	= midi_playback_trigger,
    };
    struct snd_rawmidi *rmidi;
    struct snd_rawmidi_str *str;
    int err;
// create midi ports
    err = snd_rawmidi_new(motu.card, motu.card.driver, 0, 1, 1, &rmidi);
    if (err < 0)
    return err;
    snprintf(rmidi.name, sizeof(rmidi.name),
    "%s MIDI", motu.card.shortname);
    rmidi.private_data = motu;
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_OUTPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &capture_ops);
    str = &rmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT];
    set_midi_substream_names(motu, str);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &playback_ops);
    str = &rmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT];
    set_midi_substream_names(motu, str);
    return 0;
    }
