//! Automatically rewritten from C to Rust
//! Source: sound/firewire/bebob/bebob_terratec.c
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
// bebob_terratec.c - a part of driver for BeBoB based devices
//
// Copyright (c) 2013-2014 Takashi Sakamoto
//

    static const enum snd_bebob_clock_type phase88_rack_clk_src_types[] = {
    SND_BEBOB_CLOCK_TYPE_INTERNAL,
    SND_BEBOB_CLOCK_TYPE_EXTERNAL,	/* S/PDIF */
    SND_BEBOB_CLOCK_TYPE_EXTERNAL,	/* Word Clock */
    };
    static int
    phase88_rack_clk_src_get(struct snd_bebob *bebob, unsigned int *id)
    {
    unsigned int enable_ext, enable_word;
    int err;
    err = avc_audio_get_selector(bebob.unit, 0, 9, &enable_ext);
    if (err < 0)
    goto end;
    err = avc_audio_get_selector(bebob.unit, 0, 8, &enable_word);
    if (err < 0)
    goto end;
    if (enable_ext == 0)
// id = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: enable_word ==) -> else {
    else if (enable_word == 0)
// id = 1;
    else
// id = 2;
    end:
    return err;
    }
    static const struct snd_bebob_rate_spec phase_series_rate_spec = {
    .get	= &snd_bebob_stream_get_rate,
    .set	= &snd_bebob_stream_set_rate,
    };
// PHASE 88 Rack FW
    static const struct snd_bebob_clock_spec phase88_rack_clk = {
    .num	= ARRAY_SIZE(phase88_rack_clk_src_types),
    .types	= phase88_rack_clk_src_types,
    .get	= &phase88_rack_clk_src_get,
    };
    const struct snd_bebob_spec phase88_rack_spec = {
    .clock	= &phase88_rack_clk,
    .rate	= &phase_series_rate_spec,
    .meter	= core::ptr::null_mut()
    };
