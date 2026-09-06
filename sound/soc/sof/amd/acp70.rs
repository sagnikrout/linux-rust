//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/amd/acp70.c
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
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2024 Advanced Micro Devices, Inc.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>
//
// Hardware interface for Audio DSP on ACP7.0 version based platform
//

pub const I2S_HS_INSTANCE: c_int = 0;
pub const I2S_BT_INSTANCE: c_int = 1;
pub const I2S_SP_INSTANCE: c_int = 2;
pub const PDM_DMIC_INSTANCE: c_int = 3;
pub const I2S_HS_VIRTUAL_INSTANCE: c_int = 4;
    static struct snd_soc_dai_driver acp70_sof_dai[] = {
    [I2S_HS_INSTANCE] = {
    .id = I2S_HS_INSTANCE,
    .name = "acp-sof-hs",
    .playback = {
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min = 2,
    .channels_max = 8,
    .rate_min = 8000,
    .rate_max = 96000,
    },
    .capture = {
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
// Supporting only stereo for I2S HS controller capture
    .channels_min = 2,
    .channels_max = 2,
    .rate_min = 8000,
    .rate_max = 48000,
    },
    },
    [I2S_BT_INSTANCE] = {
    .id = I2S_BT_INSTANCE,
    .name = "acp-sof-bt",
    .playback = {
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min = 2,
    .channels_max = 8,
    .rate_min = 8000,
    .rate_max = 96000,
    },
    .capture = {
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
// Supporting only stereo for I2S BT controller capture
    .channels_min = 2,
    .channels_max = 2,
    .rate_min = 8000,
    .rate_max = 48000,
    },
    },
    [I2S_SP_INSTANCE] = {
    .id = I2S_SP_INSTANCE,
    .name = "acp-sof-sp",
    .playback = {
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min = 2,
    .channels_max = 8,
    .rate_min = 8000,
    .rate_max = 96000,
    },
    .capture = {
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
// Supporting only stereo for I2S SP controller capture
    .channels_min = 2,
    .channels_max = 2,
    .rate_min = 8000,
    .rate_max = 48000,
    },
    },
    [PDM_DMIC_INSTANCE] = {
    .id = PDM_DMIC_INSTANCE,
    .name = "acp-sof-dmic",
    .capture = {
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min = 2,
    .channels_max = 4,
    .rate_min = 8000,
    .rate_max = 48000,
    },
    },
    [I2S_HS_VIRTUAL_INSTANCE] = {
    .id = I2S_HS_VIRTUAL_INSTANCE,
    .name = "acp-sof-hs-virtual",
    .playback = {
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min = 2,
    .channels_max = 8,
    .rate_min = 8000,
    .rate_max = 96000,
    },
    },
    };
// Phoenix ops
    struct snd_sof_dsp_ops sof_acp70_ops;
    EXPORT_SYMBOL_NS(sof_acp70_ops, "SND_SOC_SOF_AMD_COMMON");
#[no_mangle]
pub unsafe extern "C" fn sof_acp70_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    int sof_acp70_ops_init(struct snd_sof_dev *sdev)
    {
// common defaults
    memcpy(&sof_acp70_ops, &sof_acp_common_ops, sizeof(struct snd_sof_dsp_ops));
    sof_acp70_ops.drv = acp70_sof_dai;
    sof_acp70_ops.num_drv = ARRAY_SIZE(acp70_sof_dai);
    return 0;
    }
