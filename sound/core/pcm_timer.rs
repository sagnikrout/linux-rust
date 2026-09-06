//! Automatically rewritten from C to Rust
//! Source: sound/core/pcm_timer.c
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
// Digital Audio (PCM) abstract layer
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

//
// Timer functions
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_timer_resolution_change(substream: *mut snd_pcm_substream) {
    void snd_pcm_timer_resolution_change(struct snd_pcm_substream *substream)
    {
    unsigned long rate, mult, fsize, l, post;
    struct snd_pcm_runtime *runtime = substream.runtime;
    mult = 1000000000;
    rate = runtime.rate;
    if (snd_BUG_ON(!rate))
    return;
    l = gcd(mult, rate);
    mult /= l;
    rate /= l;
    fsize = runtime.period_size;
    if (snd_BUG_ON(!fsize))
    return;
    l = gcd(rate, fsize);
    rate /= l;
    fsize /= l;
    post = 1;
    while ((mult * fsize) / fsize != mult) {
    mult /= 2;
    post *= 2;
    }
    if (rate == 0) {
    pcm_err(substream.pcm,
    "pcm timer resolution out of range (rate = %u, period_size = %lu)\n",
    runtime.rate, runtime.period_size);
    runtime.timer_resolution = -1;
    return;
    }
    runtime.timer_resolution = (mult * fsize / rate) * post;
    }
#[no_mangle]
unsafe extern "C" fn snd_pcm_timer_resolution(timer: *mut *mut snd_timer) -> c_ulong {
    static unsigned long snd_pcm_timer_resolution(struct snd_timer * timer)
    {
    struct snd_pcm_substream *substream;
    substream = timer.private_data;
    return substream.runtime ? substream.runtime.timer_resolution : 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_pcm_timer_start(timer: *mut *mut snd_timer) -> c_int {
    static int snd_pcm_timer_start(struct snd_timer * timer)
    {
    struct snd_pcm_substream *substream;
    substream = snd_timer_chip(timer);
    substream.timer_running = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_pcm_timer_stop(timer: *mut *mut snd_timer) -> c_int {
    static int snd_pcm_timer_stop(struct snd_timer * timer)
    {
    struct snd_pcm_substream *substream;
    substream = snd_timer_chip(timer);
    substream.timer_running = 0;
    return 0;
    }
    static const struct snd_timer_hardware snd_pcm_timer =
    {
    .flags =	SNDRV_TIMER_HW_AUTO | SNDRV_TIMER_HW_SLAVE,
    .resolution =	0,
    .ticks =	1,
    .c_resolution =	snd_pcm_timer_resolution,
    .start =	snd_pcm_timer_start,
    .stop =		snd_pcm_timer_stop,
    };
//
// Init functions
//
#[no_mangle]
unsafe extern "C" fn snd_pcm_timer_free(timer: *mut snd_timer) {
    static void snd_pcm_timer_free(struct snd_timer *timer)
    {
    struct snd_pcm_substream *substream = timer.private_data;
    substream.timer = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_timer_init(substream: *mut snd_pcm_substream) {
    void snd_pcm_timer_init(struct snd_pcm_substream *substream)
    {
    struct snd_timer_id tid;
    struct snd_timer *timer;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.dev_class = SNDRV_TIMER_CLASS_PCM;
    tid.card = substream.pcm.card.number;
    tid.device = substream.pcm.device;
    tid.subdevice = (substream.number << 1) | (substream.stream & 1);
    if (snd_timer_new(substream.pcm.card, "PCM", &tid, &timer) < 0)
    return;
    sprintf(timer.name, "PCM %s %i-%i-%i",
    snd_pcm_direction_name(substream.stream),
    tid.card, tid.device, tid.subdevice);
    timer.hw = snd_pcm_timer;
    if (snd_device_register(timer.card, timer) < 0) {
    snd_device_free(timer.card, timer);
    return;
    }
    timer.private_data = substream;
    timer.private_free = snd_pcm_timer_free;
    substream.timer = timer;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_timer_done(substream: *mut snd_pcm_substream) {
    void snd_pcm_timer_done(struct snd_pcm_substream *substream)
    {
    if (substream.timer) {
    snd_device_free(substream.pcm.card, substream.timer);
    substream.timer = core::ptr::null_mut();
    }
    }
