//! Automatically rewritten from C to Rust
//! Source: sound/soc/soc-utils.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// soc-util.c  --  ALSA SoC Audio Layer utility functions
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
// Liam Girdwood <lrg@slimlogic.co.uk>

#[no_mangle]
pub unsafe extern "C" fn snd_soc_ret(dev: *const device, ret: c_int, fmt: *const c_char, ...) -> c_int {
    int snd_soc_ret(const struct device *dev, int ret, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
// Positive, Zero values are not errors
    if (ret >= 0)
    return ret;
// Negative values might be errors
    switch (ret) {
    case -EPROBE_DEFER:
    case -ENOTSUPP:
    case -EOPNOTSUPP:
    break;
    default:
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    dev_err(dev, "ASoC error (%d): %pV", ret, &vaf);
    va_end(args);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(snd_soc_ret);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_calc_frame_size(sample_size: c_int, channels: c_int, tdm_slots: c_int) -> c_int {
    int snd_soc_calc_frame_size(int sample_size, int channels, int tdm_slots)
    {
    return sample_size * channels * tdm_slots;
    }
    EXPORT_SYMBOL_GPL(snd_soc_calc_frame_size);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_params_to_frame_size(params: *const snd_pcm_hw_params) -> c_int {
    int snd_soc_params_to_frame_size(const struct snd_pcm_hw_params *params)
    {
    int sample_size;
    sample_size = snd_pcm_format_width(params_format(params));
    if (sample_size < 0)
    return sample_size;
    return snd_soc_calc_frame_size(sample_size, params_channels(params),
    1);
    }
    EXPORT_SYMBOL_GPL(snd_soc_params_to_frame_size);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_calc_bclk(fs: c_int, sample_size: c_int, channels: c_int, tdm_slots: c_int) -> c_int {
    int snd_soc_calc_bclk(int fs, int sample_size, int channels, int tdm_slots)
    {
    return fs * snd_soc_calc_frame_size(sample_size, channels, tdm_slots);
    }
    EXPORT_SYMBOL_GPL(snd_soc_calc_bclk);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_params_to_bclk(params: *const snd_pcm_hw_params) -> c_int {
    int snd_soc_params_to_bclk(const struct snd_pcm_hw_params *params)
    {
    int ret;
    ret = snd_soc_params_to_frame_size(params);
    if (ret > 0)
    return ret * params_rate(params);
    else
    return ret;
    }
    EXPORT_SYMBOL_GPL(snd_soc_params_to_bclk);
//
// snd_soc_tdm_params_to_bclk - calculate bclk from params and tdm slot info.
//
// Calculate the bclk from the params sample rate, the tdm slot count and the
// tdm slot width. Optionally round-up the slot count to a given multiple.
// Either or both of tdm_width and tdm_slots can be 0.
//
// If tdm_width == 0:	use params_width() as the slot width.
// If tdm_slots == 0:	use params_channels() as the slot count.
//
// If slot_multiple > 1 the slot count (or params_channels() if tdm_slots == 0)
// will be rounded up to a multiple of slot_multiple. This is mainly useful for
// I2S mode, which has a left and right phase so the number of slots is always
// a multiple of 2.
//
// If tdm_width == 0 && tdm_slots == 0 && slot_multiple < 2, this is equivalent
// to calling snd_soc_params_to_bclk().
//
// @params:        Pointer to struct_pcm_hw_params.
// @tdm_width:     Width in bits of the tdm slots. Must be >= 0.
// @tdm_slots:     Number of tdm slots per frame. Must be >= 0.
// @slot_multiple: If >1 roundup slot count to a multiple of this value.
//
// Return: bclk frequency in Hz, else a negative error code if params format
// is invalid.
//
    int snd_soc_tdm_params_to_bclk(const struct snd_pcm_hw_params *params,
    int tdm_width, int tdm_slots, int slot_multiple)
    {
    if (!tdm_slots)
    tdm_slots = params_channels(params);
    if (slot_multiple > 1)
    tdm_slots = roundup(tdm_slots, slot_multiple);
    if (!tdm_width) {
    tdm_width = snd_pcm_format_width(params_format(params));
    if (tdm_width < 0)
    return tdm_width;
    }
    return snd_soc_calc_bclk(params_rate(params), tdm_width, 1, tdm_slots);
    }
    EXPORT_SYMBOL_GPL(snd_soc_tdm_params_to_bclk);
    static const struct snd_pcm_hardware dummy_dma_hardware = {
// Random values to keep userspace happy when checking constraints
    .info			= SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER,
    .buffer_bytes_max	= 128*1024,
    .period_bytes_min	= 4096,
    .period_bytes_max	= 4096*2,
    .periods_min		= 2,
    .periods_max		= 128,
    };
    static const struct snd_soc_component_driver dummy_platform;
    static int dummy_dma_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    int i;
//
// If there are other components associated with rtd, we shouldn't
// override their hwparams
//
    for_each_rtd_components(rtd, i, component) {
    if (component.driver == &dummy_platform)
    return 0;
    }
// BE's dont need dummy params
    if (!rtd.dai_link.no_pcm)
    snd_soc_set_runtime_hwparams(substream, &dummy_dma_hardware);
    return 0;
    }
    static const struct snd_soc_component_driver dummy_platform = {
    .open		= dummy_dma_open,
    };
    static const struct snd_soc_component_driver dummy_codec = {
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };

    SNDRV_PCM_FMTBIT_U8 | \
    SNDRV_PCM_FMTBIT_S16_LE | \
    SNDRV_PCM_FMTBIT_U16_LE | \
    SNDRV_PCM_FMTBIT_S24_LE | \
    SNDRV_PCM_FMTBIT_S24_3LE | \
    SNDRV_PCM_FMTBIT_U24_LE | \
    SNDRV_PCM_FMTBIT_S32_LE | \
    SNDRV_PCM_FMTBIT_U32_LE | \
    SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE)
    static const u64 dummy_dai_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_A	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_B	|
    SND_SOC_POSSIBLE_DAIFMT_AC97	|
    SND_SOC_POSSIBLE_DAIFMT_PDM	|
    SND_SOC_POSSIBLE_DAIFMT_GATED	|
    SND_SOC_POSSIBLE_DAIFMT_CONT	|
    SND_SOC_POSSIBLE_DAIFMT_NB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_NB_IF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_IF;
    static const struct snd_soc_dai_ops dummy_dai_ops = {
    .auto_selectable_formats	= &dummy_dai_formats,
    .num_auto_selectable_formats	= 1,
    };
//
// The dummy CODEC is only meant to be used in situations where there is no
// actual hardware.
//
// If there is actual hardware even if it does not have a control bus
// the hardware will still have constraints like supported samplerates, etc.
// which should be modelled. And the data flow graph also should be modelled
// using DAPM.
//
    static struct snd_soc_dai_driver dummy_dai = {
    .name = "snd-soc-dummy-dai",
    .playback = {
    .stream_name	= "Playback",
    .channels_min	= 1,
    .channels_max	= 384,
    .rates		= SNDRV_PCM_RATE_CONTINUOUS,
    .rate_min	= 5512,
    .rate_max	= 768000,
    .formats	= STUB_FORMATS,
    },
    .capture = {
    .stream_name	= "Capture",
    .channels_min	= 1,
    .channels_max	= 384,
    .rates = SNDRV_PCM_RATE_CONTINUOUS,
    .rate_min	= 5512,
    .rate_max	= 768000,
    .formats = STUB_FORMATS,
    },
    .ops = &dummy_dai_ops,
    };
#[no_mangle]
pub unsafe extern "C" fn snd_soc_dai_is_dummy(dai: *const snd_soc_dai) -> c_int {
    int snd_soc_dai_is_dummy(const struct snd_soc_dai *dai)
    {
    if (dai.driver == &dummy_dai)
    return 1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_soc_dai_is_dummy);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_is_dummy(component: *mut snd_soc_component) -> c_int {
    int snd_soc_component_is_dummy(struct snd_soc_component *component)
    {
    return ((component.driver == &dummy_platform) ||
    (component.driver == &dummy_codec));
    }
    struct snd_soc_dai_link_component snd_soc_dummy_dlc = {
    .of_node	= core::ptr::null_mut(),
    .dai_name	= "snd-soc-dummy-dai",
    .name		= "snd-soc-dummy",
    };
    EXPORT_SYMBOL_GPL(snd_soc_dummy_dlc);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_dlc_is_dummy(dlc: *mut snd_soc_dai_link_component) -> c_int {
    int snd_soc_dlc_is_dummy(struct snd_soc_dai_link_component *dlc)
    {
    if (dlc == &snd_soc_dummy_dlc)
    return true;
    if ((dlc.name     && strcmp(dlc.name,     snd_soc_dummy_dlc.name)     == 0) ||
    (dlc.dai_name && strcmp(dlc.dai_name, snd_soc_dummy_dlc.dai_name) == 0))
    return true;
    return false;
    }
    EXPORT_SYMBOL_GPL(snd_soc_dlc_is_dummy);
#[no_mangle]
unsafe extern "C" fn snd_soc_dummy_probe(fdev: *mut faux_device) -> c_int {
    static int snd_soc_dummy_probe(struct faux_device *fdev)
    {
    int ret;
    ret = devm_snd_soc_register_component(&fdev.dev,
    &dummy_codec, &dummy_dai, 1);
    if (ret < 0)
    return ret;
    ret = devm_snd_soc_register_component(&fdev.dev, &dummy_platform,
    core::ptr::null_mut(), 0);
    return ret;
    }
    static struct faux_device_ops soc_dummy_ops = {
    .probe = snd_soc_dummy_probe,
    };
    static struct faux_device *soc_dummy_dev;
#[no_mangle]
pub unsafe extern "C" fn snd_soc_util_init() -> int __init {
    int __init snd_soc_util_init(void)
    {
    soc_dummy_dev = faux_device_create("snd-soc-dummy", core::ptr::null_mut(),
    &soc_dummy_ops);
    if (!soc_dummy_dev)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_soc_util_exit() {
    void snd_soc_util_exit(void)
    {
    faux_device_destroy(soc_dummy_dev);
    }
