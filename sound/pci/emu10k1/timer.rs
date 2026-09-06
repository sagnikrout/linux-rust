//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/timer.c
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
// Copyright (c) by Lee Revell <rlrevell@joe-job.com>
// Clemens Ladisch <clemens@ladisch.de>
// Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
//
// Routines for control of EMU10K1 chips
//

#[no_mangle]
unsafe extern "C" fn snd_emu10k1_timer_start(timer: *mut snd_timer) -> c_int {
    static int snd_emu10k1_timer_start(struct snd_timer *timer)
    {
    struct snd_emu10k1 *emu;
    unsigned int delay;
    emu = snd_timer_chip(timer);
    delay = timer.sticks - 1;
    if (delay < 5 ) /* minimum time is 5 ticks */
    delay = 5;
    snd_emu10k1_intr_enable(emu, INTE_INTERVALTIMERENB);
    outw(delay & TIMER_RATE_MASK, emu.port + TIMER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_timer_stop(timer: *mut snd_timer) -> c_int {
    static int snd_emu10k1_timer_stop(struct snd_timer *timer)
    {
    struct snd_emu10k1 *emu;
    emu = snd_timer_chip(timer);
    snd_emu10k1_intr_disable(emu, INTE_INTERVALTIMERENB);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_timer_c_resolution(timer: *mut snd_timer) -> c_ulong {
    static unsigned long snd_emu10k1_timer_c_resolution(struct snd_timer *timer)
    {
    struct snd_emu10k1 *emu = snd_timer_chip(timer);
    if (emu.card_capabilities.emu_model &&
    emu.emu1010.word_clock == 44100)
    return 22676;  // 1 sample @ 44.1 kHz = 22.675736...us
    else
    return 20833;  // 1 sample @ 48 kHz = 20.833...us
    }
    static int snd_emu10k1_timer_precise_resolution(struct snd_timer *timer,
    unsigned long *num, unsigned long *den)
    {
    struct snd_emu10k1 *emu = snd_timer_chip(timer);
// num = 1;
    if (emu.card_capabilities.emu_model)
// den = emu->emu1010.word_clock;
    else
// den = 48000;
    return 0;
    }
    static const struct snd_timer_hardware snd_emu10k1_timer_hw = {
    .flags = SNDRV_TIMER_HW_AUTO,
    .ticks = 1024,
    .start = snd_emu10k1_timer_start,
    .stop = snd_emu10k1_timer_stop,
    .c_resolution = snd_emu10k1_timer_c_resolution,
    .precise_resolution = snd_emu10k1_timer_precise_resolution,
    };
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_timer(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    int snd_emu10k1_timer(struct snd_emu10k1 *emu, int device)
    {
    struct snd_timer *timer = core::ptr::null_mut();
    struct snd_timer_id tid;
    int err;
    tid.dev_class = SNDRV_TIMER_CLASS_CARD;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.card = emu.card.number;
    tid.device = device;
    tid.subdevice = 0;
    err = snd_timer_new(emu.card, "EMU10K1", &tid, &timer);
    if (err >= 0) {
    strscpy(timer.name, "EMU10K1 timer");
    timer.private_data = emu;
    timer.hw = snd_emu10k1_timer_hw;
    }
    emu.timer = timer;
    return err;
    }
