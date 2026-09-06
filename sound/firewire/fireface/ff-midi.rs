//! Automatically rewritten from C to Rust
//! Source: sound/firewire/fireface/ff-midi.c
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
// ff-midi.c - a part of driver for RME Fireface series
//
// Copyright (c) 2015-2017 Takashi Sakamoto
//

#[no_mangle]
unsafe extern "C" fn midi_capture_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_capture_open(struct snd_rawmidi_substream *substream)
    {
// Do nothing.
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_playback_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_ff *ff = substream.rmidi.private_data;
// Initialize internal status.
    ff.on_sysex[substream.number] = 0;
    ff.rx_midi_error[substream.number] = false;
    WRITE_ONCE(ff.rx_midi_substreams[substream.number], substream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_capture_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_capture_close(struct snd_rawmidi_substream *substream)
    {
// Do nothing.
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_playback_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_ff *ff = substream.rmidi.private_data;
    cancel_work_sync(&ff.rx_midi_work[substream.number]);
    WRITE_ONCE(ff.rx_midi_substreams[substream.number], core::ptr::null_mut());
    return 0;
    }
    static void midi_capture_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct snd_ff *ff = substream.rmidi.private_data;
    guard(spinlock_irqsave)(&ff.lock);
    if (up)
    WRITE_ONCE(ff.tx_midi_substreams[substream.number],
    substream);
    else
    WRITE_ONCE(ff.tx_midi_substreams[substream.number], core::ptr::null_mut());
    }
    static void midi_playback_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct snd_ff *ff = substream.rmidi.private_data;
    guard(spinlock_irqsave)(&ff.lock);
    if (up || !ff.rx_midi_error[substream.number])
    schedule_work(&ff.rx_midi_work[substream.number]);
    }
    static void set_midi_substream_names(struct snd_rawmidi_str *stream,
    const char *const name)
    {
    struct snd_rawmidi_substream *substream;
    list_for_each_entry(substream, &stream.substreams, list) {
    scnprintf(substream.name, sizeof(substream.name),
    "%s MIDI %d", name, substream.number + 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snd_ff_create_midi_devices(ff: *mut snd_ff) -> c_int {
    int snd_ff_create_midi_devices(struct snd_ff *ff)
    {
    static const struct snd_rawmidi_ops midi_capture_ops = {
    .open		= midi_capture_open,
    .close		= midi_capture_close,
    .trigger	= midi_capture_trigger,
    };
    static const struct snd_rawmidi_ops midi_playback_ops = {
    .open		= midi_playback_open,
    .close		= midi_playback_close,
    .trigger	= midi_playback_trigger,
    };
    struct snd_rawmidi *rmidi;
    struct snd_rawmidi_str *stream;
    int err;
    err = snd_rawmidi_new(ff.card, ff.card.driver, 0,
    ff.spec.midi_out_ports, ff.spec.midi_in_ports,
    &rmidi);
    if (err < 0)
    return err;
    snprintf(rmidi.name, sizeof(rmidi.name),
    "%s MIDI", ff.card.shortname);
    rmidi.private_data = ff;
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &midi_capture_ops);
    stream = &rmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT];
    set_midi_substream_names(stream, ff.card.shortname);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &midi_playback_ops);
    stream = &rmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT];
    set_midi_substream_names(stream, ff.card.shortname);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    return 0;
    }
