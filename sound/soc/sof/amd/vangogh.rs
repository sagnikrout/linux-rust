//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/amd/vangogh.c
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
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: Venkata Prasad Potturu <venkataprasad.potturu@amd.com>
//
// Hardware interface for Audio DSP on Vangogh platform
//

pub const I2S_HS_INSTANCE: c_int = 0;
pub const I2S_BT_INSTANCE: c_int = 1;
pub const I2S_SP_INSTANCE: c_int = 2;
pub const PDM_DMIC_INSTANCE: c_int = 3;
pub const I2S_HS_VIRTUAL_INSTANCE: c_int = 4;
    static struct snd_soc_dai_driver vangogh_sof_dai[] = {
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
    .capture = {
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
// Supporting only stereo for I2S HS-Virtual controller capture
    .channels_min = 2,
    .channels_max = 2,
    .rate_min = 8000,
    .rate_max = 48000,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn sof_vangogh_post_fw_run_delay(sdev: *mut snd_sof_dev) -> c_int {
    static int sof_vangogh_post_fw_run_delay(struct snd_sof_dev *sdev)
    {
//
// Resuming from suspend in some cases my cause the DSP firmware
// to enter an unrecoverable faulty state.  Delaying a bit any host
// to DSP transmission right after firmware boot completion seems
// to resolve the issue.
//
    if (!sdev.first_boot)
    usleep_range(100, 150);
    return 0;
    }
// Vangogh ops
    struct snd_sof_dsp_ops sof_vangogh_ops;
    EXPORT_SYMBOL_NS(sof_vangogh_ops, "SND_SOC_SOF_AMD_COMMON");
#[no_mangle]
pub unsafe extern "C" fn sof_vangogh_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    int sof_vangogh_ops_init(struct snd_sof_dev *sdev)
    {
    const struct dmi_system_id *dmi_id;
    struct acp_quirk_entry *quirks;
// common defaults
    memcpy(&sof_vangogh_ops, &sof_acp_common_ops, sizeof(struct snd_sof_dsp_ops));
    sof_vangogh_ops.drv = vangogh_sof_dai;
    sof_vangogh_ops.num_drv = ARRAY_SIZE(vangogh_sof_dai);
    dmi_id = dmi_first_match(acp_sof_quirk_table);
    if (dmi_id) {
    quirks = dmi_id.driver_data;
    if (quirks.signed_fw_image)
    sof_vangogh_ops.load_firmware = acp_sof_load_signed_firmware;
    if (quirks.post_fw_run_delay)
    sof_vangogh_ops.post_fw_run = sof_vangogh_post_fw_run_delay;
    }
    return 0;
    }
