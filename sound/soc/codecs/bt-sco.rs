//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/bt-sco.c
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
// Driver for generic Bluetooth SCO link
// Copyright 2011 Lars-Peter Clausen <lars@metafoo.de>
//

    static const struct snd_soc_dapm_widget bt_sco_widgets[] = {
    SND_SOC_DAPM_INPUT("RX"),
    SND_SOC_DAPM_OUTPUT("TX"),
    SND_SOC_DAPM_AIF_IN("BT_SCO_RX", "Playback", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("BT_SCO_TX", "Capture", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("BT_SCO_RX_WB", "WB Playback", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("BT_SCO_TX_WB", "WB Capture", 0,
    SND_SOC_NOPM, 0, 0),
    };
    static const struct snd_soc_dapm_route bt_sco_routes[] = {
    { "BT_SCO_TX", core::ptr::null_mut(), "RX" },
    { "TX", core::ptr::null_mut(), "BT_SCO_RX" },
    { "BT_SCO_TX_WB", core::ptr::null_mut(), "RX" },
    { "TX", core::ptr::null_mut(), "BT_SCO_RX_WB" },
    };
    static struct snd_soc_dai_driver bt_sco_dai[] = {
    {
    .name = "bt-sco-pcm",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    },
    {
    .name = "bt-sco-pcm-wb",
    .playback = {
    .stream_name = "WB Playback",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "WB Capture",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    }
    };
    static const struct snd_soc_component_driver soc_component_dev_bt_sco = {
    .dapm_widgets		= bt_sco_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(bt_sco_widgets),
    .dapm_routes		= bt_sco_routes,
    .num_dapm_routes	= ARRAY_SIZE(bt_sco_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn bt_sco_probe(pdev: *mut platform_device) -> c_int {
    static int bt_sco_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_bt_sco,
    bt_sco_dai, ARRAY_SIZE(bt_sco_dai));
    }
    static const struct platform_device_id bt_sco_driver_ids[] = {
    { .name = "dfbmcs320" },
    { .name = "bt-sco" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, bt_sco_driver_ids);

    static const struct of_device_id bt_sco_codec_of_match[] = {
    { .compatible = "delta,dfbmcs320", },
    { .compatible = "linux,bt-sco", },
    {},
    };
    MODULE_DEVICE_TABLE(of, bt_sco_codec_of_match);

    static struct platform_driver bt_sco_driver = {
    .driver = {
    .name = "bt-sco",
    .of_match_table = of_match_ptr(bt_sco_codec_of_match),
    },
    .probe = bt_sco_probe,
    .id_table = bt_sco_driver_ids,
    };
    module_platform_driver(bt_sco_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("ASoC generic bluetooth sco link driver");
    MODULE_LICENSE("GPL");
