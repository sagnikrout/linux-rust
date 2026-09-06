//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/mediatek/mt8195/mt8195-loader.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright (c) 2021 Mediatek Corporation. All rights reserved.
//
// Author: YC Hung <yc.hung@mediatek.com>
//
// Hardware interface for mt8195 DSP code loader

#[no_mangle]
pub unsafe extern "C" fn sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32) {
    void sof_hifixdsp_boot_sequence(struct snd_sof_dev *sdev, u32 boot_addr)
    {
// ADSP bootup base
    snd_sof_dsp_write(sdev, DSP_REG_BAR, DSP_ALTRESETVEC, boot_addr);
// pull high RunStall (set bit3 to 1)
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    ADSP_RUNSTALL, ADSP_RUNSTALL);
// pull high StatVectorSel to use AltResetVec (set bit4 to 1)
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    STATVECTOR_SEL, STATVECTOR_SEL);
// toggle  DReset & BReset
// pull high DReset & BReset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    ADSP_BRESET_SW | ADSP_DRESET_SW,
    ADSP_BRESET_SW | ADSP_DRESET_SW);
// delay 10 DSP cycles at 26M about 1us by IP vendor's suggestion
    udelay(1);
// pull low DReset & BReset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    ADSP_BRESET_SW | ADSP_DRESET_SW,
    0);
// Enable PDebug
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_PDEBUGBUS0,
    PDEBUG_ENABLE,
    PDEBUG_ENABLE);
// release RunStall (set bit3 to 0)
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    ADSP_RUNSTALL, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev) {
    void sof_hifixdsp_shutdown(struct snd_sof_dev *sdev)
    {
// RUN_STALL pull high again to reset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    ADSP_RUNSTALL, ADSP_RUNSTALL);
// pull high DReset & BReset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW,
    ADSP_BRESET_SW | ADSP_DRESET_SW,
    ADSP_BRESET_SW | ADSP_DRESET_SW);
    }
