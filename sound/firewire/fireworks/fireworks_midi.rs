//! Automatically rewritten from C to Rust
//! Source: sound/firewire/fireworks/fireworks_midi.c
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
// fireworks_midi.c - a part of driver for Fireworks based devices
//
// Copyright (c) 2009-2010 Clemens Ladisch
// Copyright (c) 2013-2014 Takashi Sakamoto
//

#[no_mangle]
unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_efw *efw = substream.rmidi.private_data;
    int err;
    err = snd_efw_stream_lock_try(efw);
    if (err < 0)
    return err;
    scoped_guard(mutex, &efw.mutex) {
    err = snd_efw_stream_reserve_duplex(efw, 0, 0, 0);
    if (err >= 0) {
    ++efw.substreams_counter;
    err = snd_efw_stream_start_duplex(efw);
    if (err < 0)
    --efw.substreams_counter;
    }
    }
    if (err < 0)
    snd_efw_stream_lock_release(efw);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_efw *efw = substream.rmidi.private_data;
    scoped_guard(mutex, &efw.mutex) {
    --efw.substreams_counter;
    snd_efw_stream_stop_duplex(efw);
    }
    snd_efw_stream_lock_release(efw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_capture_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_efw *efw = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&efw.lock);
    if (up)
    amdtp_am824_midi_trigger(&efw.tx_stream,
    substrm.number, substrm);
    else
    amdtp_am824_midi_trigger(&efw.tx_stream,
    substrm.number, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_playback_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_efw *efw = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&efw.lock);
    if (up)
    amdtp_am824_midi_trigger(&efw.rx_stream,
    substrm.number, substrm);
    else
    amdtp_am824_midi_trigger(&efw.rx_stream,
    substrm.number, core::ptr::null_mut());
    }
    static void set_midi_substream_names(struct snd_efw *efw,
    struct snd_rawmidi_str *str)
    {
    struct snd_rawmidi_substream *subs;
    list_for_each_entry(subs, &str.substreams, list) {
    scnprintf(subs.name, sizeof(subs.name),
    "%s MIDI %d", efw.card.shortname, subs.number + 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snd_efw_create_midi_devices(efw: *mut snd_efw) -> c_int {
    int snd_efw_create_midi_devices(struct snd_efw *efw)
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
    err = snd_rawmidi_new(efw.card, efw.card.driver, 0,
    efw.midi_out_ports, efw.midi_in_ports,
    &rmidi);
    if (err < 0)
    return err;
    snprintf(rmidi.name, sizeof(rmidi.name),
    "%s MIDI", efw.card.shortname);
    rmidi.private_data = efw;
    if (efw.midi_in_ports > 0) {
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &capture_ops);
    str = &rmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT];
    set_midi_substream_names(efw, str);
    }
    if (efw.midi_out_ports > 0) {
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &playback_ops);
    str = &rmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT];
    set_midi_substream_names(efw, str);
    }
    if ((efw.midi_out_ports > 0) && (efw.midi_in_ports > 0))
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    return 0;
    }
