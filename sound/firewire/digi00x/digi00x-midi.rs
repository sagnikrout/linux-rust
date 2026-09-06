//! Automatically rewritten from C to Rust
//! Source: sound/firewire/digi00x/digi00x-midi.c
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
// digi00x-midi.h - a part of driver for Digidesign Digi 002/003 family
//
// Copyright (c) 2014-2015 Takashi Sakamoto
//

#[no_mangle]
unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_dg00x *dg00x = substream.rmidi.private_data;
    int err;
    err = snd_dg00x_stream_lock_try(dg00x);
    if (err < 0)
    return err;
    scoped_guard(mutex, &dg00x.mutex) {
    err = snd_dg00x_stream_reserve_duplex(dg00x, 0, 0, 0);
    if (err >= 0) {
    ++dg00x.substreams_counter;
    err = snd_dg00x_stream_start_duplex(dg00x);
    if (err < 0)
    --dg00x.substreams_counter;
    }
    }
    if (err < 0)
    snd_dg00x_stream_lock_release(dg00x);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int midi_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_dg00x *dg00x = substream.rmidi.private_data;
    scoped_guard(mutex, &dg00x.mutex) {
    --dg00x.substreams_counter;
    snd_dg00x_stream_stop_duplex(dg00x);
    }
    snd_dg00x_stream_lock_release(dg00x);
    return 0;
    }
    static void midi_capture_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct snd_dg00x *dg00x = substream.rmidi.private_data;
    unsigned int port;
    if (substream.rmidi.device == 0)
    port = substream.number;
    else
    port = 2;
    guard(spinlock_irqsave)(&dg00x.lock);
    if (up)
    amdtp_dot_midi_trigger(&dg00x.tx_stream, port, substream);
    else
    amdtp_dot_midi_trigger(&dg00x.tx_stream, port, core::ptr::null_mut());
    }
    static void midi_playback_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct snd_dg00x *dg00x = substream.rmidi.private_data;
    unsigned int port;
    if (substream.rmidi.device == 0)
    port = substream.number;
    else
    port = 2;
    guard(spinlock_irqsave)(&dg00x.lock);
    if (up)
    amdtp_dot_midi_trigger(&dg00x.rx_stream, port, substream);
    else
    amdtp_dot_midi_trigger(&dg00x.rx_stream, port, core::ptr::null_mut());
    }
    static void set_substream_names(struct snd_dg00x *dg00x,
    struct snd_rawmidi *rmidi, bool is_console)
    {
    struct snd_rawmidi_substream *subs;
    struct snd_rawmidi_str *str;
    int i;
    for (i = 0; i < 2; ++i) {
    str = &rmidi.streams[i];
    list_for_each_entry(subs, &str.substreams, list) {
    if (!is_console) {
    scnprintf(subs.name, sizeof(subs.name),
    "%s MIDI %d",
    dg00x.card.shortname,
    subs.number + 1);
    } else {
    scnprintf(subs.name, sizeof(subs.name),
    "%s control",
    dg00x.card.shortname);
    }
    }
    }
    }
    static int add_substream_pair(struct snd_dg00x *dg00x, unsigned int out_ports,
    unsigned int in_ports, bool is_console)
    {
    static const struct snd_rawmidi_ops capture_ops = {
    .open = midi_open,
    .close = midi_close,
    .trigger = midi_capture_trigger,
    };
    static const struct snd_rawmidi_ops playback_ops = {
    .open = midi_open,
    .close = midi_close,
    .trigger = midi_playback_trigger,
    };
    const char *label;
    struct snd_rawmidi *rmidi;
    int err;
// Add physical midi ports.
    err = snd_rawmidi_new(dg00x.card, dg00x.card.driver, is_console,
    out_ports, in_ports, &rmidi);
    if (err < 0)
    return err;
    rmidi.private_data = dg00x;
    if (!is_console)
    label = "%s control";
    else
    label = "%s MIDI";
    snprintf(rmidi.name, sizeof(rmidi.name), label,
    dg00x.card.shortname);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &playback_ops);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &capture_ops);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_OUTPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    set_substream_names(dg00x, rmidi, is_console);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_dg00x_create_midi_devices(dg00x: *mut snd_dg00x) -> c_int {
    int snd_dg00x_create_midi_devices(struct snd_dg00x *dg00x)
    {
    int err;
// Add physical midi ports.
    err = add_substream_pair(dg00x, DOT_MIDI_OUT_PORTS, DOT_MIDI_IN_PORTS,
    false);
    if (err < 0)
    return err;
    if (dg00x.is_console)
    err = add_substream_pair(dg00x, 1, 1, true);
    return err;
    }
