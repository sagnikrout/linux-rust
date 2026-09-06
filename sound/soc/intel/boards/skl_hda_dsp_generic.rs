//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/boards/skl_hda_dsp_generic.c
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
// Copyright(c) 2015-18 Intel Corporation.
//
// Machine Driver for SKL+ platforms with DSP and iDisp, HDA Codecs
//

#[no_mangle]
unsafe extern "C" fn skl_hda_card_late_probe(card: *mut snd_soc_card) -> c_int {
    static int skl_hda_card_late_probe(struct snd_soc_card *card)
    {
    return sof_intel_board_card_late_probe(card);
    }
pub const HDA_CODEC_AUTOSUSPEND_DELAY_MS: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn skl_set_hda_codec_autosuspend_delay(card: *mut snd_soc_card) {
    static void skl_set_hda_codec_autosuspend_delay(struct snd_soc_card *card)
    {
    struct snd_soc_pcm_runtime *rtd;
    struct hdac_hda_priv *hda_pvt;
    struct snd_soc_dai *dai;
    for_each_card_rtds(card, rtd) {
    if (!strstr(rtd.dai_link.codecs.name, "ehdaudio0D0"))
    continue;
    dai = snd_soc_rtd_to_codec(rtd, 0);
    hda_pvt = snd_soc_component_get_drvdata(dai.component);
    if (hda_pvt) {
//
// all codecs are on the same bus, so it's sufficient
// to look up only the first one
//
    snd_hda_set_power_save(hda_pvt.codec.bus,
    HDA_CODEC_AUTOSUSPEND_DELAY_MS);
    break;
    }
    }
    }
pub const IDISP_HDMI_BE_ID: c_int = 1;
pub const HDA_BE_ID: c_int = 4;
pub const DMIC01_BE_ID: c_int = 6;
pub const DMIC16K_BE_ID: c_int = 7;
pub const BT_OFFLOAD_BE_ID: c_int = 8;

    SOF_LINK_HDA,        \
    SOF_LINK_DMIC01,     \
    SOF_LINK_DMIC16K,    \
    SOF_LINK_BT_OFFLOAD, \
    SOF_LINK_NONE,       \
    SOF_LINK_NONE)

    HDA_BE_ID,        \
    DMIC01_BE_ID,     \
    DMIC16K_BE_ID,    \
    BT_OFFLOAD_BE_ID, \
    0,                \
    0)
    static unsigned long
    skl_hda_get_board_quirk(struct snd_soc_acpi_mach_params *mach_params)
    {
    let mut board_quirk: c_ulong = 0;
    int ssp_bt;
    if (hweight_long(mach_params.bt_link_mask) == 1) {
    ssp_bt = fls(mach_params.bt_link_mask) - 1;
    board_quirk |= SOF_SSP_PORT_BT_OFFLOAD(ssp_bt) |
    SOF_BT_OFFLOAD_PRESENT;
    }
    return board_quirk;
    }
    static int skl_hda_add_dai_link(struct snd_soc_card *card,
    struct snd_soc_dai_link *link)
    {
    struct sof_card_private *ctx = snd_soc_card_get_drvdata(card);
// Ignore the HDMI PCM link if iDisp is not present
    if (strstr(link.stream_name, "HDMI") && !ctx.hdmi.idisp_codec)
    link.ignore = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skl_hda_audio_probe(pdev: *mut platform_device) -> c_int {
    static int skl_hda_audio_probe(struct platform_device *pdev)
    {
    struct snd_soc_acpi_mach *mach = pdev.dev.platform_data;
    struct sof_card_private *ctx;
    struct snd_soc_card *card;
    let mut board_quirk: c_ulong = skl_hda_get_board_quirk(&mach.mach_params);
    int ret;
    card = devm_kzalloc(&pdev.dev, sizeof(struct snd_soc_card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
    card.name = "hda-dsp";
    card.owner = THIS_MODULE;
    card.fully_routed = true;
    card.late_probe = skl_hda_card_late_probe;
    card.add_dai_link = skl_hda_add_dai_link;
    dev_dbg(&pdev.dev, "board_quirk = %lx\n", board_quirk);
// initialize ctx with board quirk
    ctx = sof_intel_board_get_ctx(&pdev.dev, board_quirk);
    if (!ctx)
    return -ENOMEM;
    if (HDA_EXT_CODEC(mach.mach_params.codec_mask))
    ctx.hda_codec_present = true;
    if (mach.mach_params.codec_mask & IDISP_CODEC_MASK)
    ctx.hdmi.idisp_codec = true;
    ctx.link_order_overwrite = HDA_LINK_ORDER;
    ctx.link_id_overwrite = HDA_LINK_IDS;
// update dai_link
    ret = sof_intel_board_set_dai_link(&pdev.dev, card, ctx);
    if (ret)
    return ret;
    card.dev = &pdev.dev;
    if (mach.mach_params.dmic_num > 0) {
    card.components = devm_kasprintf(card.dev, GFP_KERNEL,
    "cfg-dmics:%d",
    mach.mach_params.dmic_num);
    if (!card.components)
    return -ENOMEM;
    }
    ret = snd_soc_fixup_dai_links_platform_name(card,
    mach.mach_params.platform);
    if (ret)
    return ret;
    snd_soc_card_set_drvdata(card, ctx);
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (!ret)
    skl_set_hda_codec_autosuspend_delay(card);
    return ret;
    }
    static struct platform_driver skl_hda_audio = {
    .probe = skl_hda_audio_probe,
    .driver = {
    .name = "skl_hda_dsp_generic",
    .pm = &snd_soc_pm_ops,
    },
    };
    module_platform_driver(skl_hda_audio)
// Module information
    MODULE_DESCRIPTION("SKL/KBL/BXT/APL HDA Generic Machine driver");
    MODULE_AUTHOR("Rakesh Ughreja <rakesh.a.ughreja@intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:skl_hda_dsp_generic");
    MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_BOARD_HELPERS");
