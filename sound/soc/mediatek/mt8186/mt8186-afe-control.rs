//! Automatically rewritten from C to Rust
//! Source: sound/soc/mediatek/mt8186/mt8186-afe-control.c
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


// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

    enum {
    MTK_AFE_RATE_8K = 0,
    MTK_AFE_RATE_11K,
    MTK_AFE_RATE_12K,
    MTK_AFE_RATE_384K,
    MTK_AFE_RATE_16K,
    MTK_AFE_RATE_22K,
    MTK_AFE_RATE_24K,
    MTK_AFE_RATE_352K,
    MTK_AFE_RATE_32K,
    MTK_AFE_RATE_44K,
    MTK_AFE_RATE_48K,
    MTK_AFE_RATE_88K,
    MTK_AFE_RATE_96K,
    MTK_AFE_RATE_176K,
    MTK_AFE_RATE_192K,
    MTK_AFE_RATE_260K,
    };
    enum {
    MTK_AFE_PCM_RATE_8K = 0,
    MTK_AFE_PCM_RATE_16K,
    MTK_AFE_PCM_RATE_32K,
    MTK_AFE_PCM_RATE_48K,
    };
    enum {
    MTK_AFE_TDM_RATE_8K = 0,
    MTK_AFE_TDM_RATE_12K,
    MTK_AFE_TDM_RATE_16K,
    MTK_AFE_TDM_RATE_24K,
    MTK_AFE_TDM_RATE_32K,
    MTK_AFE_TDM_RATE_48K,
    MTK_AFE_TDM_RATE_64K,
    MTK_AFE_TDM_RATE_96K,
    MTK_AFE_TDM_RATE_128K,
    MTK_AFE_TDM_RATE_192K,
    MTK_AFE_TDM_RATE_256K,
    MTK_AFE_TDM_RATE_384K,
    MTK_AFE_TDM_RATE_11K,
    MTK_AFE_TDM_RATE_22K,
    MTK_AFE_TDM_RATE_44K,
    MTK_AFE_TDM_RATE_88K,
    MTK_AFE_TDM_RATE_176K,
    MTK_AFE_TDM_RATE_352K,
    };
    enum {
    MTK_AFE_TDM_RELATCH_RATE_8K = 0,
    MTK_AFE_TDM_RELATCH_RATE_11K,
    MTK_AFE_TDM_RELATCH_RATE_12K,
    MTK_AFE_TDM_RELATCH_RATE_16K,
    MTK_AFE_TDM_RELATCH_RATE_22K,
    MTK_AFE_TDM_RELATCH_RATE_24K,
    MTK_AFE_TDM_RELATCH_RATE_32K,
    MTK_AFE_TDM_RELATCH_RATE_44K,
    MTK_AFE_TDM_RELATCH_RATE_48K,
    MTK_AFE_TDM_RELATCH_RATE_88K,
    MTK_AFE_TDM_RELATCH_RATE_96K,
    MTK_AFE_TDM_RELATCH_RATE_176K,
    MTK_AFE_TDM_RELATCH_RATE_192K,
    MTK_AFE_TDM_RELATCH_RATE_352K,
    MTK_AFE_TDM_RELATCH_RATE_384K,
    };
#[no_mangle]
pub unsafe extern "C" fn mt8186_general_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    unsigned int mt8186_general_rate_transform(struct device *dev, unsigned int rate)
    {
    switch (rate) {
    case 8000:
    return MTK_AFE_RATE_8K;
    case 11025:
    return MTK_AFE_RATE_11K;
    case 12000:
    return MTK_AFE_RATE_12K;
    case 16000:
    return MTK_AFE_RATE_16K;
    case 22050:
    return MTK_AFE_RATE_22K;
    case 24000:
    return MTK_AFE_RATE_24K;
    case 32000:
    return MTK_AFE_RATE_32K;
    case 44100:
    return MTK_AFE_RATE_44K;
    case 48000:
    return MTK_AFE_RATE_48K;
    case 88200:
    return MTK_AFE_RATE_88K;
    case 96000:
    return MTK_AFE_RATE_96K;
    case 176400:
    return MTK_AFE_RATE_176K;
    case 192000:
    return MTK_AFE_RATE_192K;
    case 260000:
    return MTK_AFE_RATE_260K;
    case 352800:
    return MTK_AFE_RATE_352K;
    case 384000:
    return MTK_AFE_RATE_384K;
    default:
    dev_err(dev, "%s(), rate %u invalid, use %d!!!\n",
    __func__, rate, MTK_AFE_RATE_48K);
    }
    return MTK_AFE_RATE_48K;
    }
#[no_mangle]
unsafe extern "C" fn tdm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    static unsigned int tdm_rate_transform(struct device *dev, unsigned int rate)
    {
    switch (rate) {
    case 8000:
    return MTK_AFE_TDM_RATE_8K;
    case 11025:
    return MTK_AFE_TDM_RATE_11K;
    case 12000:
    return MTK_AFE_TDM_RATE_12K;
    case 16000:
    return MTK_AFE_TDM_RATE_16K;
    case 22050:
    return MTK_AFE_TDM_RATE_22K;
    case 24000:
    return MTK_AFE_TDM_RATE_24K;
    case 32000:
    return MTK_AFE_TDM_RATE_32K;
    case 44100:
    return MTK_AFE_TDM_RATE_44K;
    case 48000:
    return MTK_AFE_TDM_RATE_48K;
    case 64000:
    return MTK_AFE_TDM_RATE_64K;
    case 88200:
    return MTK_AFE_TDM_RATE_88K;
    case 96000:
    return MTK_AFE_TDM_RATE_96K;
    case 128000:
    return MTK_AFE_TDM_RATE_128K;
    case 176400:
    return MTK_AFE_TDM_RATE_176K;
    case 192000:
    return MTK_AFE_TDM_RATE_192K;
    case 256000:
    return MTK_AFE_TDM_RATE_256K;
    case 352800:
    return MTK_AFE_TDM_RATE_352K;
    case 384000:
    return MTK_AFE_TDM_RATE_384K;
    default:
    dev_err(dev, "%s(), rate %u invalid, use %d!!!\n",
    __func__, rate, MTK_AFE_TDM_RATE_48K);
    }
    return MTK_AFE_TDM_RATE_48K;
    }
#[no_mangle]
unsafe extern "C" fn pcm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    static unsigned int pcm_rate_transform(struct device *dev, unsigned int rate)
    {
    switch (rate) {
    case 8000:
    return MTK_AFE_PCM_RATE_8K;
    case 16000:
    return MTK_AFE_PCM_RATE_16K;
    case 32000:
    return MTK_AFE_PCM_RATE_32K;
    case 48000:
    return MTK_AFE_PCM_RATE_48K;
    default:
    dev_err(dev, "%s(), rate %u invalid, use %d!!!\n",
    __func__, rate, MTK_AFE_PCM_RATE_48K);
    }
    return MTK_AFE_PCM_RATE_48K;
    }
#[no_mangle]
pub unsafe extern "C" fn mt8186_tdm_relatch_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    unsigned int mt8186_tdm_relatch_rate_transform(struct device *dev, unsigned int rate)
    {
    switch (rate) {
    case 8000:
    return MTK_AFE_TDM_RELATCH_RATE_8K;
    case 11025:
    return MTK_AFE_TDM_RELATCH_RATE_11K;
    case 12000:
    return MTK_AFE_TDM_RELATCH_RATE_12K;
    case 16000:
    return MTK_AFE_TDM_RELATCH_RATE_16K;
    case 22050:
    return MTK_AFE_TDM_RELATCH_RATE_22K;
    case 24000:
    return MTK_AFE_TDM_RELATCH_RATE_24K;
    case 32000:
    return MTK_AFE_TDM_RELATCH_RATE_32K;
    case 44100:
    return MTK_AFE_TDM_RELATCH_RATE_44K;
    case 48000:
    return MTK_AFE_TDM_RELATCH_RATE_48K;
    case 88200:
    return MTK_AFE_TDM_RELATCH_RATE_88K;
    case 96000:
    return MTK_AFE_TDM_RELATCH_RATE_96K;
    case 176400:
    return MTK_AFE_TDM_RELATCH_RATE_176K;
    case 192000:
    return MTK_AFE_TDM_RELATCH_RATE_192K;
    case 352800:
    return MTK_AFE_TDM_RELATCH_RATE_352K;
    case 384000:
    return MTK_AFE_TDM_RELATCH_RATE_384K;
    default:
    dev_err(dev, "%s(), rate %u invalid, use %d!!!\n",
    __func__, rate, MTK_AFE_TDM_RELATCH_RATE_48K);
    }
    return MTK_AFE_TDM_RELATCH_RATE_48K;
    }
#[no_mangle]
pub unsafe extern "C" fn mt8186_rate_transform(dev: *mut device, rate: c_uint, aud_blk: c_int) -> c_uint {
    unsigned int mt8186_rate_transform(struct device *dev, unsigned int rate, int aud_blk)
    {
    switch (aud_blk) {
    case MT8186_DAI_PCM:
    return pcm_rate_transform(dev, rate);
    case MT8186_DAI_TDM_IN:
    return tdm_rate_transform(dev, rate);
    default:
    return mt8186_general_rate_transform(dev, rate);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_set_priv(afe: *mut mtk_base_afe, id: c_int, priv_size: c_int, priv_data: *const c_void) -> c_int {
    int mt8186_dai_set_priv(struct mtk_base_afe *afe, int id, int priv_size, const void *priv_data)
    {
    struct mt8186_afe_private *afe_priv = afe.platform_priv;
    void *temp_data;
    temp_data = devm_kzalloc(afe.dev,
    priv_size,
    GFP_KERNEL);
    if (!temp_data)
    return -ENOMEM;
    if (priv_data)
    memcpy(temp_data, priv_data, priv_size);
    afe_priv.dai_priv[id] = temp_data;
    return 0;
    }
