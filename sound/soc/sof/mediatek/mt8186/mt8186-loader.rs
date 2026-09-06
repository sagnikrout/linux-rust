//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/mediatek/mt8186/mt8186-loader.c
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
// Copyright (c) 2022 Mediatek Corporation. All rights reserved.
//
// Author: Allen-KH Cheng <allen-kh.cheng@mediatek.com>
// Tinghan Shen <tinghan.shen@mediatek.com>
//
// Hardware interface for mt8186 DSP code loader

#[no_mangle]
pub unsafe extern "C" fn mt8186_sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32) {
    void mt8186_sof_hifixdsp_boot_sequence(struct snd_sof_dev *sdev, u32 boot_addr)
    {
// set RUNSTALL to stop core
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_HIFI_IO_CONFIG,
    RUNSTALL, RUNSTALL);
// enable mbox 0 & 1 IRQ
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_MBOX_IRQ_EN,
    DSP_MBOX0_IRQ_EN | DSP_MBOX1_IRQ_EN,
    DSP_MBOX0_IRQ_EN | DSP_MBOX1_IRQ_EN);
// set core boot address
    snd_sof_dsp_write(sdev, DSP_SECREG_BAR, ADSP_ALTVEC_C0, boot_addr);
    snd_sof_dsp_write(sdev, DSP_SECREG_BAR, ADSP_ALTVECSEL, ADSP_ALTVECSEL_C0);
// assert core reset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_CFGREG_SW_RSTN,
    SW_RSTN_C0 | SW_DBG_RSTN_C0,
    SW_RSTN_C0 | SW_DBG_RSTN_C0);
// hardware requirement
    udelay(1);
// release core reset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_CFGREG_SW_RSTN,
    SW_RSTN_C0 | SW_DBG_RSTN_C0,
    0);
// clear RUNSTALL (bit31) to start core
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_HIFI_IO_CONFIG,
    RUNSTALL, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mt8186_sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev) {
    void mt8186_sof_hifixdsp_shutdown(struct snd_sof_dev *sdev)
    {
// set RUNSTALL to stop core
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_HIFI_IO_CONFIG,
    RUNSTALL, RUNSTALL);
// assert core reset
    snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_CFGREG_SW_RSTN,
    SW_RSTN_C0 | SW_DBG_RSTN_C0,
    SW_RSTN_C0 | SW_DBG_RSTN_C0);
    }
