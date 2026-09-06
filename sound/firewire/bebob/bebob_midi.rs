//! Automatically rewritten from C to Rust
//! Source: sound/firewire/bebob/bebob_midi.c
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
// bebob_midi.c - a part of driver for BeBoB based devices
//
// Copyright (c) 2013-2014 Takashi Sakamoto
//

#[no_mangle]
unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_bebob *bebob = substream.rmidi.private_data;
    int err;
    err = snd_bebob_stream_lock_try(bebob);
    if (err < 0)
    return err;
    scoped_guard(mutex, &bebob.mutex) {
    err = snd_bebob_stream_reserve_duplex(bebob, 0, 0, 0);
    if (err >= 0) {
    ++bebob.substreams_counter;
    err = snd_bebob_stream_start_duplex(bebob);
    if (err < 0)
    --bebob.substreams_counter;
    }
    }
    if (err < 0)
    snd_bebob_stream_lock_release(bebob);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_bebob *bebob = substream.rmidi.private_data;
    scoped_guard(mutex, &bebob.mutex) {
    bebob.substreams_counter--;
    snd_bebob_stream_stop_duplex(bebob);
    }
    snd_bebob_stream_lock_release(bebob);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_capture_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_bebob *bebob = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&bebob.lock);
    if (up)
    amdtp_am824_midi_trigger(&bebob.tx_stream,
    substrm.number, substrm);
    else
    amdtp_am824_midi_trigger(&bebob.tx_stream,
    substrm.number, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_playback_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_bebob *bebob = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&bebob.lock);
    if (up)
    amdtp_am824_midi_trigger(&bebob.rx_stream,
    substrm.number, substrm);
    else
    amdtp_am824_midi_trigger(&bebob.rx_stream,
    substrm.number, core::ptr::null_mut());
    }
    static void set_midi_substream_names(struct snd_bebob *bebob,
    struct snd_rawmidi_str *str)
    {
    struct snd_rawmidi_substream *subs;
    list_for_each_entry(subs, &str.substreams, list) {
    scnprintf(subs.name, sizeof(subs.name),
    "%s MIDI %d",
    bebob.card.shortname, subs.number + 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snd_bebob_create_midi_devices(bebob: *mut snd_bebob) -> c_int {
    int snd_bebob_create_midi_devices(struct snd_bebob *bebob)
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
    err = snd_rawmidi_new(bebob.card, bebob.card.driver, 0,
    bebob.midi_output_ports, bebob.midi_input_ports,
    &rmidi);
    if (err < 0)
    return err;
    snprintf(rmidi.name, sizeof(rmidi.name),
    "%s MIDI", bebob.card.shortname);
    rmidi.private_data = bebob;
    if (bebob.midi_input_ports > 0) {
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &capture_ops);
    str = &rmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT];
    set_midi_substream_names(bebob, str);
    }
    if (bebob.midi_output_ports > 0) {
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &playback_ops);
    str = &rmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT];
    set_midi_substream_names(bebob, str);
    }
    if ((bebob.midi_output_ports > 0) && (bebob.midi_input_ports > 0))
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    return 0;
    }
