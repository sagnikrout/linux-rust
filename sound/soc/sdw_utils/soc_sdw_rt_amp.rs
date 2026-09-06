//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_rt_amp.c
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


// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2022 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
//
// soc_sdw_rt_amp - Helpers to handle RT1308/RT1316/RT1318 from generic machine driver
//

pub const CODEC_NAME_SIZE: c_int = 7;
// choose a larger value to resolve compatibility issues

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_amp_platform_data {
    pub bq_params: *const c_uchar,
    pub bq_params_cnt: c_uint,
}

    static const struct rt_amp_platform_data dell_0a5d_platform_data = {
    .bq_params = dell_0a5d_bq_params,
    .bq_params_cnt = ARRAY_SIZE(dell_0a5d_bq_params),
    };
    static const struct rt_amp_platform_data dell_0b00_platform_data = {
    .bq_params = dell_0b00_bq_params,
    .bq_params_cnt = ARRAY_SIZE(dell_0b00_bq_params),
    };
    static const struct dmi_system_id dmi_platform_data[] = {
// CometLake devices
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0990")
    },
    .driver_data = (void *)&dell_0a5d_platform_data,
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "098F")
    },
    .driver_data = (void *)&dell_0a5d_platform_data,
    },
// TigerLake devices
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0A5D")
    },
    .driver_data = (void *)&dell_0a5d_platform_data,
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0A5E")
    },
    .driver_data = (void *)&dell_0a5d_platform_data,
    },
// AlderLake devices
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0B00")
    },
    .driver_data = (void *)&dell_0b00_platform_data,
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0B01")
    },
    .driver_data = (void *)&dell_0b00_platform_data,
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0AFF")
    },
    .driver_data = (void *)&dell_0b00_platform_data,
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "0AFE")
    },
    .driver_data = (void *)&dell_0b00_platform_data,
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn rt_amp_add_device_props(sdw_dev: *mut device) -> c_int {
    static int rt_amp_add_device_props(struct device *sdw_dev)
    {
    struct property_entry props[3] = {};
    struct fwnode_handle *fwnode;
    const struct dmi_system_id *dmi_data;
    const struct rt_amp_platform_data *pdata;
    unsigned char params[RT_AMP_MAX_BQ_REG];
    int ret;
    dmi_data = dmi_first_match(dmi_platform_data);
    if (!dmi_data)
    return 0;
    pdata = dmi_data.driver_data;
    memcpy(&params, pdata.bq_params, sizeof(unsigned char) * pdata.bq_params_cnt);
    props[0] = PROPERTY_ENTRY_U8_ARRAY("realtek,bq-params", params);
    props[1] = PROPERTY_ENTRY_U32("realtek,bq-params-cnt", pdata.bq_params_cnt);
    fwnode = fwnode_create_software_node(props, core::ptr::null_mut());
    if (IS_ERR(fwnode))
    return PTR_ERR(fwnode);
    ret = device_add_software_node(sdw_dev, to_software_node(fwnode));
    fwnode_handle_put(fwnode);
    return ret;
    }
//
// dapm routes for rt1308/rt1316/rt1318 will be registered dynamically
// according to the number of rt1308/rt1316/rt1318 used. The first two
// entries will be registered for one codec case, and the last two entries
// are also registered if two 1308s/1316s/1318s are used.
//
    static const struct snd_soc_dapm_route rt1308_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt1308-1 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1308-1 SPOR" },
    { "Speaker", core::ptr::null_mut(), "rt1308-2 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1308-2 SPOR" },
    };
    static const struct snd_soc_dapm_route rt1316_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt1316-1 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1316-1 SPOR" },
    { "Speaker", core::ptr::null_mut(), "rt1316-2 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1316-2 SPOR" },
    };
    static const struct snd_soc_dapm_route rt1318_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt1318-1 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1318-1 SPOR" },
    { "Speaker", core::ptr::null_mut(), "rt1318-2 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1318-2 SPOR" },
    };
    static const struct snd_soc_dapm_route rt1320_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt1320-1 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1320-1 SPOR" },
    { "Speaker", core::ptr::null_mut(), "rt1320-2 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt1320-2 SPOR" },
    };
    static const struct snd_soc_dapm_route *get_codec_name_and_route(struct snd_soc_dai *dai,
    char *codec_name)
    {
// get the codec name
    snprintf(codec_name, CODEC_NAME_SIZE, "%s", dai.name);
// choose the right codec's map
    if (strcmp(codec_name, "rt1308") == 0)
    return rt1308_map;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(codec_name, 0: "rt1316") ==) -> else {
    else if (strcmp(codec_name, "rt1316") == 0)
    return rt1316_map;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(codec_name, 0: "rt1318") ==) -> else {
    else if (strcmp(codec_name, "rt1318") == 0)
    return rt1318_map;
    else
    return rt1320_map;
    }
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_amp_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_rt_amp_spk_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    const struct snd_soc_dapm_route *rt_amp_map;
    char codec_name[CODEC_NAME_SIZE];
    struct snd_soc_dai *codec_dai;
    let mut ret: c_int = -EINVAL;
    int i;
    rt_amp_map = get_codec_name_and_route(dai, codec_name);
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    if (strstr(codec_dai.component.name_prefix, "-1"))
    ret = snd_soc_dapm_add_routes(dapm, rt_amp_map, 2);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(codec_dai->component->name_prefix, _arg: "-2")) -> else {
    else if (strstr(codec_dai.component.name_prefix, "-2"))
    ret = snd_soc_dapm_add_routes(dapm, rt_amp_map + 2, 2);
    }
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt_amp_spk_rtd_init, "SND_SOC_SDW_UTILS");
    static int rt1308_i2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    int clk_id, clk_freq, pll_out;
    int err;
    clk_id = RT1308_PLL_S_MCLK;
    clk_freq = 38400000;
    pll_out = params_rate(params) * 512;
// Set rt1308 pll
    err = snd_soc_dai_set_pll(codec_dai, 0, clk_id, clk_freq, pll_out);
    if (err < 0) {
    dev_err(card.dev, "Failed to set RT1308 PLL: %d\n", err);
    return err;
    }
// Set rt1308 sysclk
    err = snd_soc_dai_set_sysclk(codec_dai, RT1308_FS_SYS_S_PLL, pll_out,
    SND_SOC_CLOCK_IN);
    if (err < 0) {
    dev_err(card.dev, "Failed to set RT1308 SYSCLK: %d\n", err);
    return err;
    }
    return 0;
    }
// machine stream operations
    const struct snd_soc_ops soc_sdw_rt1308_i2s_ops = {
    .hw_params = rt1308_i2s_hw_params,
    };
    EXPORT_SYMBOL_NS(soc_sdw_rt1308_i2s_ops, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_amp_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int {
    int asoc_sdw_rt_amp_exit(struct snd_soc_card *card, struct snd_soc_dai_link *dai_link)
    {
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    if (ctx.amp_dev1) {
    device_remove_software_node(ctx.amp_dev1);
    put_device(ctx.amp_dev1);
    ctx.amp_dev1 = core::ptr::null_mut();
    }
    if (ctx.amp_dev2) {
    device_remove_software_node(ctx.amp_dev2);
    put_device(ctx.amp_dev2);
    ctx.amp_dev2 = core::ptr::null_mut();
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt_amp_exit, "SND_SOC_SDW_UTILS");
    int asoc_sdw_rt_amp_init(struct snd_soc_card *card,
    struct snd_soc_dai_link *dai_links,
    struct asoc_sdw_codec_info *info,
    bool playback)
    {
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct device *sdw_dev1, *sdw_dev2;
    int ret;
// Count amp number and do init on playback link only.
    if (!playback)
    return 0;
    info.amp_num++;
    if (info.amp_num == 2) {
    sdw_dev1 = bus_find_device_by_name(&sdw_bus_type, core::ptr::null_mut(), dai_links.codecs[0].name);
    if (!sdw_dev1)
    return -EPROBE_DEFER;
    ret = rt_amp_add_device_props(sdw_dev1);
    if (ret < 0) {
    put_device(sdw_dev1);
    return ret;
    }
    ctx.amp_dev1 = sdw_dev1;
    sdw_dev2 = bus_find_device_by_name(&sdw_bus_type, core::ptr::null_mut(), dai_links.codecs[1].name);
    if (!sdw_dev2)
    return -EPROBE_DEFER;
    ret = rt_amp_add_device_props(sdw_dev2);
    if (ret < 0) {
    put_device(sdw_dev2);
    return ret;
    }
    ctx.amp_dev2 = sdw_dev2;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt_amp_init, "SND_SOC_SDW_UTILS");
