//! Automatically rewritten from C to Rust
//! Source: sound/firewire/tascam/tascam-midi.c
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
// tascam-midi.c - a part of driver for TASCAM FireWire series
//
// Copyright (c) 2015 Takashi Sakamoto
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
    struct snd_tscm *tscm = substream.rmidi.private_data;
    snd_fw_async_midi_port_init(&tscm.out_ports[substream.number]);
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
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_drain(substream: *mut snd_rawmidi_substream) {
    static void midi_playback_drain(struct snd_rawmidi_substream *substream)
    {
    struct snd_tscm *tscm = substream.rmidi.private_data;
    snd_fw_async_midi_port_finish(&tscm.out_ports[substream.number]);
    }
#[no_mangle]
unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_capture_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_tscm *tscm = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&tscm.lock);
    if (up)
    tscm.tx_midi_substreams[substrm.number] = substrm;
    else
    tscm.tx_midi_substreams[substrm.number] = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    static void midi_playback_trigger(struct snd_rawmidi_substream *substrm, int up)
    {
    struct snd_tscm *tscm = substrm.rmidi.private_data;
    guard(spinlock_irqsave)(&tscm.lock);
    if (up)
    snd_fw_async_midi_port_run(&tscm.out_ports[substrm.number],
    substrm);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_create_midi_devices(tscm: *mut snd_tscm) -> c_int {
    int snd_tscm_create_midi_devices(struct snd_tscm *tscm)
    {
    static const struct snd_rawmidi_ops capture_ops = {
    .open		= midi_capture_open,
    .close		= midi_capture_close,
    .trigger	= midi_capture_trigger,
    };
    static const struct snd_rawmidi_ops playback_ops = {
    .open		= midi_playback_open,
    .close		= midi_playback_close,
    .drain		= midi_playback_drain,
    .trigger	= midi_playback_trigger,
    };
    struct snd_rawmidi *rmidi;
    struct snd_rawmidi_str *stream;
    struct snd_rawmidi_substream *subs;
    int err;
    err = snd_rawmidi_new(tscm.card, tscm.card.driver, 0,
    tscm.spec.midi_playback_ports,
    tscm.spec.midi_capture_ports,
    &rmidi);
    if (err < 0)
    return err;
    snprintf(rmidi.name, sizeof(rmidi.name),
    "%s MIDI", tscm.card.shortname);
    rmidi.private_data = tscm;
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &capture_ops);
    stream = &rmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT];
// Set port names for MIDI input.
    list_for_each_entry(subs, &stream.substreams, list) {
// TODO: support virtual MIDI ports.
    if (subs.number < tscm.spec.midi_capture_ports) {
// Hardware MIDI ports.
    scnprintf(subs.name, sizeof(subs.name),
    "%s MIDI %d",
    tscm.card.shortname, subs.number + 1);
    }
    }
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &playback_ops);
    stream = &rmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT];
// Set port names for MIDI ourput.
    list_for_each_entry(subs, &stream.substreams, list) {
    if (subs.number < tscm.spec.midi_playback_ports) {
// Hardware MIDI ports only.
    scnprintf(subs.name, sizeof(subs.name),
    "%s MIDI %d",
    tscm.card.shortname, subs.number + 1);
    }
    }
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    return 0;
    }
