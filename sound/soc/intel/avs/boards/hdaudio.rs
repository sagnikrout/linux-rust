//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/boards/hdaudio.c
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
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

    static int avs_create_dai_links(struct device *dev, struct hda_codec *codec, int pcm_count,
    struct snd_soc_dai_link **links)
    {
    struct snd_soc_dai_link_component *platform;
    struct snd_soc_dai_link *dl;
    struct hda_pcm *pcm;
    const char *cname = dev_name(&codec.core.dev);
    int i;
    dl = devm_kcalloc(dev, pcm_count, sizeof(*dl), GFP_KERNEL);
    platform = devm_kzalloc(dev, sizeof(*platform), GFP_KERNEL);
    if (!dl || !platform)
    return -ENOMEM;
    platform.name = dev_name(dev);
    pcm = list_first_entry(&codec.pcm_list_head, struct hda_pcm, list);
    for (i = 0; i < pcm_count; i++, pcm = list_next_entry(pcm, list)) {
    dl[i].name = devm_kasprintf(dev, GFP_KERNEL, "%s link%d", cname, i);
    if (!dl[i].name)
    return -ENOMEM;
    dl[i].id = i;
    dl[i].nonatomic = 1;
    dl[i].no_pcm = 1;
    dl[i].platforms = platform;
    dl[i].num_platforms = 1;
    dl[i].ignore_pmdown_time = 1;
    dl[i].codecs = devm_kzalloc(dev, sizeof(*dl.codecs), GFP_KERNEL);
    dl[i].cpus = devm_kzalloc(dev, sizeof(*dl.cpus), GFP_KERNEL);
    if (!dl[i].codecs || !dl[i].cpus)
    return -ENOMEM;
    dl[i].cpus.dai_name = devm_kasprintf(dev, GFP_KERNEL, "%s-cpu%d", cname, i);
    if (!dl[i].cpus.dai_name)
    return -ENOMEM;
    dl[i].codecs.name = devm_kstrdup_const(dev, cname, GFP_KERNEL);
    if (!dl[i].codecs.name)
    return -ENOMEM;
    dl[i].codecs.dai_name = pcm.name;
    dl[i].num_codecs = 1;
    dl[i].num_cpus = 1;
    }
// links = dl;
    return 0;
    }
// Should be aligned with SectionPCM's name from topology

    static struct snd_pcm *
    avs_card_hdmi_pcm_at(struct snd_soc_card *card, int hdmi_idx)
    {
    struct snd_soc_pcm_runtime *rtd;
    let mut dir: c_int = SNDRV_PCM_STREAM_PLAYBACK;
    for_each_card_rtds(card, rtd) {
    struct snd_pcm *spcm;
    int ret, n;
    spcm = rtd.pcm ? rtd.pcm.streams[dir].pcm : core::ptr::null_mut();
    if (!spcm || !strstr(spcm.id, FEDAI_NAME_PREFIX))
    continue;
    ret = sscanf(spcm.id, FEDAI_NAME_PREFIX "%d", &n);
    if (ret != 1)
    continue;
    if (n == hdmi_idx)
    return rtd.pcm;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn avs_card_late_probe(card: *mut snd_soc_card) -> c_int {
    static int avs_card_late_probe(struct snd_soc_card *card)
    {
    struct snd_soc_acpi_mach *mach = dev_get_platdata(card.dev);
    struct avs_mach_pdata *pdata = mach.pdata;
    struct hda_codec *codec = pdata.codec;
    struct hda_pcm *hpcm;
// Topology pcm indexing is 1-based
    let mut i: c_int = 1;
    list_for_each_entry(hpcm, &codec.pcm_list_head, list) {
    struct snd_pcm *spcm;
    spcm = avs_card_hdmi_pcm_at(card, i);
    if (spcm) {
    hpcm.pcm = spcm;
    hpcm.device = spcm.device;
    dev_info(card.dev, "%s: mapping HDMI converter %d to PCM %d (%p)\n",
    __func__, i, hpcm.device, spcm);
    } else {
    hpcm.pcm = core::ptr::null_mut();
    hpcm.device = SNDRV_PCM_INVALID_DEVICE;
    dev_warn(card.dev, "%s: no PCM in topology for HDMI converter %d\n",
    __func__, i);
    }
    i++;
    }
    return hda_codec_probe_complete(codec);
    }
#[no_mangle]
unsafe extern "C" fn avs_probing_link_init(rtm: *mut snd_soc_pcm_runtime) -> c_int {
    static int avs_probing_link_init(struct snd_soc_pcm_runtime *rtm)
    {
    struct snd_soc_acpi_mach *mach;
    struct avs_mach_pdata *pdata;
    struct snd_soc_dai_link *links = core::ptr::null_mut();
    struct snd_soc_card *card = rtm.card;
    struct hda_codec *codec;
    struct hda_pcm *pcm;
    int ret, pcm_count = 0;
    mach = dev_get_platdata(card.dev);
    pdata = mach.pdata;
    codec = pdata.codec;
    if (list_empty(&codec.pcm_list_head))
    return -EINVAL;
    list_for_each_entry(pcm, &codec.pcm_list_head, list)
    pcm_count++;
    ret = avs_create_dai_links(card.dev, codec, pcm_count, &links);
    if (ret < 0) {
    dev_err(card.dev, "create links failed: %d\n", ret);
    return ret;
    }
    ret = snd_soc_add_pcm_runtimes(card, links, pcm_count);
    if (ret < 0) {
    dev_err(card.dev, "add links failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_dai_link probing_link = {
    .name = "probing-LINK",
    .id = -1,
    .nonatomic = 1,
    .no_pcm = 1,
    .cpus = &snd_soc_dummy_dlc,
    .num_cpus = 1,
    .init = avs_probing_link_init,
    };
#[no_mangle]
unsafe extern "C" fn avs_hdaudio_probe(pdev: *mut platform_device) -> c_int {
    static int avs_hdaudio_probe(struct platform_device *pdev)
    {
    struct snd_soc_dai_link *binder;
    struct snd_soc_acpi_mach *mach;
    struct avs_mach_pdata *pdata;
    struct snd_soc_card *card;
    struct device *dev = &pdev.dev;
    struct hda_codec *codec;
    mach = dev_get_platdata(dev);
    pdata = mach.pdata;
    codec = pdata.codec;
// codec may be unloaded before card's probe() fires
    if (!device_is_registered(&codec.core.dev))
    return -ENODEV;
    binder = devm_kmemdup(dev, &probing_link, sizeof(probing_link), GFP_KERNEL);
    if (!binder)
    return -ENOMEM;
    binder.platforms = devm_kzalloc(dev, sizeof(*binder.platforms), GFP_KERNEL);
    binder.codecs = devm_kzalloc(dev, sizeof(*binder.codecs), GFP_KERNEL);
    if (!binder.platforms || !binder.codecs)
    return -ENOMEM;
    binder.codecs.name = devm_kstrdup_const(dev, dev_name(&codec.core.dev), GFP_KERNEL);
    if (!binder.codecs.name)
    return -ENOMEM;
    binder.platforms.name = dev_name(dev);
    binder.num_platforms = 1;
    binder.codecs.dai_name = "codec-probing-DAI";
    binder.num_codecs = 1;
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
    if (pdata.obsolete_card_names) {
    card.name = devm_kasprintf(dev, GFP_KERNEL, "hdaudioB%dD%d", codec.bus.core.idx,
    codec.core.addr);
    if (!card.name)
    return -ENOMEM;
    } else {
    card.driver_name = "avs_hdaudio";
    if (hda_codec_is_display(codec))
    card.long_name = card.name = "AVS HDMI";
    else
    card.long_name = card.name = "AVS HD-Audio";
    }
    card.dev = dev;
    card.owner = THIS_MODULE;
    card.dai_link = binder;
    card.num_links = 1;
    card.fully_routed = true;
    if (hda_codec_is_display(codec))
    card.late_probe = avs_card_late_probe;
    return devm_snd_soc_register_deferrable_card(dev, card);
    }
    static const struct platform_device_id avs_hdaudio_driver_ids[] = {
    { .name = "avs_hdaudio" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, avs_hdaudio_driver_ids);
    static struct platform_driver avs_hdaudio_driver = {
    .probe = avs_hdaudio_probe,
    .driver = {
    .name = "avs_hdaudio",
    .pm = &snd_soc_pm_ops,
    },
    .id_table = avs_hdaudio_driver_ids,
    };
    module_platform_driver(avs_hdaudio_driver)
    MODULE_DESCRIPTION("Intel HD-Audio machine driver");
    MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
    MODULE_LICENSE("GPL");
