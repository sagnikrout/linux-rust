//! Automatically rewritten from C to Rust
//! Source: sound/pci/cs5535audio/cs5535audio_pm.c
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
// Power management for audio on multifunction CS5535 companion device
// Copyright (C) Jaya Kumar
//

#[no_mangle]
unsafe extern "C" fn snd_cs5535audio_stop_hardware(cs5535au: *mut cs5535audio) {
    static void snd_cs5535audio_stop_hardware(struct cs5535audio *cs5535au)
    {
//
    we depend on snd_ac97_suspend to tell the
    AC97 codec to shutdown. the amd spec suggests
    that the LNK_SHUTDOWN be done at the same time
    that the codec power-down is issued. instead,
    we do it just after rather than at the same
    time. excluding codec specific build_ops.suspend
    ac97 powerdown hits:
    0x8000 EAPD
    0x4000 Headphone amplifier
    0x0300 ADC & DAC
#[no_mangle]
pub unsafe extern "C" fn powerdown(on: Vref) -> 0x0400 Analog Mixer {
    0x0400 Analog Mixer powerdown (Vref on)
    I am not sure if this is the best that we can do.
    The remainder to be investigated are:
    - analog mixer (vref off) 0x0800
    - AC-link powerdown 0x1000
    - codec internal clock 0x2000
//
// set LNK_SHUTDOWN to shutdown AC link
    cs_writel(cs5535au, ACC_CODEC_CNTL, ACC_CODEC_CNTL_LNK_SHUTDOWN);
    }
#[no_mangle]
unsafe extern "C" fn snd_cs5535audio_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_cs5535audio_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct cs5535audio *cs5535au = card.private_data;
    int i;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend(cs5535au.ac97);
    for (i = 0; i < NUM_CS5535AUDIO_DMAS; i++) {
    struct cs5535audio_dma *dma = &cs5535au.dmas[i];
    if (dma && dma.substream)
    dma.saved_prd = dma.ops.read_prd(cs5535au);
    }
// save important regs, then disable aclink in hw
    snd_cs5535audio_stop_hardware(cs5535au);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs5535audio_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_cs5535audio_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct cs5535audio *cs5535au = card.private_data;
    u32 tmp;
    int timeout;
    int i;
// set LNK_WRM_RST to reset AC link
    cs_writel(cs5535au, ACC_CODEC_CNTL, ACC_CODEC_CNTL_LNK_WRM_RST);
    timeout = 50;
    do {
    tmp = cs_readl(cs5535au, ACC_CODEC_STATUS);
    if (tmp & PRM_RDY_STS)
    break;
    udelay(1);
    } while (--timeout);
    if (!timeout)
    dev_err(cs5535au.card.dev, "Failure getting AC Link ready\n");
// set up rate regs, dma. actual initiation is done in trig
    for (i = 0; i < NUM_CS5535AUDIO_DMAS; i++) {
    struct cs5535audio_dma *dma = &cs5535au.dmas[i];
    if (dma && dma.substream) {
    dma.substream.ops.prepare(dma.substream);
    dma.ops.setup_prd(cs5535au, dma.saved_prd);
    }
    }
// we depend on ac97 to perform the codec power up
    snd_ac97_resume(cs5535au.ac97);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    SIMPLE_DEV_PM_OPS(snd_cs5535audio_pm, snd_cs5535audio_suspend, snd_cs5535audio_resume);
