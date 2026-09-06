//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/acp/acp-legacy-mach.c
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
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//
// Machine Driver Legacy Support for ACP HW block
//

    static struct acp_card_drvdata rt5682_rt1019_data = {
    .hs_cpu_id = I2S_SP,
    .amp_cpu_id = I2S_SP,
    .dmic_cpu_id = DMIC,
    .hs_codec_id = RT5682,
    .amp_codec_id = RT1019,
    .dmic_codec_id = DMIC,
    .tdm_mode = false,
    };
    static struct acp_card_drvdata rt5682s_max_data = {
    .hs_cpu_id = I2S_SP,
    .amp_cpu_id = I2S_SP,
    .dmic_cpu_id = DMIC,
    .hs_codec_id = RT5682S,
    .amp_codec_id = MAX98360A,
    .dmic_codec_id = DMIC,
    .tdm_mode = false,
    };
    static struct acp_card_drvdata rt5682s_rt1019_data = {
    .hs_cpu_id = I2S_SP,
    .amp_cpu_id = I2S_SP,
    .dmic_cpu_id = DMIC,
    .hs_codec_id = RT5682S,
    .amp_codec_id = RT1019,
    .dmic_codec_id = DMIC,
    .tdm_mode = false,
    };
    static struct acp_card_drvdata es83xx_rn_data = {
    .hs_cpu_id = I2S_SP,
    .dmic_cpu_id = DMIC,
    .hs_codec_id = ES83XX,
    .dmic_codec_id = DMIC,
    };
    static struct acp_card_drvdata max_nau8825_data = {
    .hs_cpu_id = I2S_HS,
    .amp_cpu_id = I2S_HS,
    .dmic_cpu_id = DMIC,
    .hs_codec_id = NAU8825,
    .amp_codec_id = MAX98360A,
    .dmic_codec_id = DMIC,
    .soc_mclk = true,
    .tdm_mode = false,
    };
    static struct acp_card_drvdata rt5682s_rt1019_rmb_data = {
    .hs_cpu_id = I2S_HS,
    .amp_cpu_id = I2S_HS,
    .dmic_cpu_id = DMIC,
    .hs_codec_id = RT5682S,
    .amp_codec_id = RT1019,
    .dmic_codec_id = DMIC,
    .soc_mclk = true,
    .tdm_mode = false,
    };
    static struct acp_card_drvdata acp_dmic_data = {
    .dmic_cpu_id = DMIC,
    .dmic_codec_id = DMIC,
    };
#[no_mangle]
unsafe extern "C" fn acp_asoc_init_ops(priv: *mut acp_card_drvdata) -> bool {
    static bool acp_asoc_init_ops(struct acp_card_drvdata *priv)
    {
    let mut has_ops: bool = false;
    if (priv.hs_codec_id == ES83XX) {
    has_ops = true;
    acp3x_es83xx_init_ops(&priv.ops);
    }
    return has_ops;
    }
#[no_mangle]
unsafe extern "C" fn acp_asoc_suspend_pre(card: *mut snd_soc_card) -> c_int {
    static int acp_asoc_suspend_pre(struct snd_soc_card *card)
    {
    int ret;
    ret = acp_ops_suspend_pre(card);
    if (ret == 1)
    return 0;
    else
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acp_asoc_resume_post(card: *mut snd_soc_card) -> c_int {
    static int acp_asoc_resume_post(struct snd_soc_card *card)
    {
    int ret;
    ret = acp_ops_resume_post(card);
    if (ret == 1)
    return 0;
    else
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acp_asoc_probe(pdev: *mut platform_device) -> c_int {
    static int acp_asoc_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = core::ptr::null_mut();
    struct device *dev = &pdev.dev;
    struct snd_soc_acpi_mach *mach = dev_get_platdata(&pdev.dev);
    const struct dmi_system_id *dmi_id;
    struct acp_card_drvdata *acp_card_drvdata;
    int ret;
    if (!pdev.id_entry) {
    ret = -EINVAL;
    goto out;
    }
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!card) {
    ret = -ENOMEM;
    goto out;
    }
    card.drvdata = (struct acp_card_drvdata *)pdev.id_entry.driver_data;
    acp_card_drvdata = card.drvdata;
    acp_card_drvdata.acpi_mach = (struct snd_soc_acpi_mach *)pdev.dev.platform_data;
    card.dev = dev;
    card.owner = THIS_MODULE;
    card.name = pdev.id_entry.name;
    acp_asoc_init_ops(card.drvdata);
// If widgets and controls are not set in specific callback,
// they will be added per-codec in acp-mach-common.c
//
    ret = acp_ops_configure_widgets(card);
    if (ret < 0) {
    dev_err(&pdev.dev,
    "Cannot configure widgets for card (%s): %d\n",
    card.name, ret);
    goto out;
    }
    card.suspend_pre = acp_asoc_suspend_pre;
    card.resume_post = acp_asoc_resume_post;
    ret = acp_ops_probe(card);
    if (ret < 0) {
    dev_err(&pdev.dev,
    "Cannot probe card (%s): %d\n",
    card.name, ret);
    goto out;
    }
    if (!strcmp(pdev.name, "acp-pdm-mach"))
    acp_card_drvdata.acp_rev =  *((int *)dev.platform_data);
    else
    acp_card_drvdata.acp_rev = mach.mach_params.subsystem_rev;
    dmi_id = dmi_first_match(acp_quirk_table);
    if (dmi_id && dmi_id.driver_data == (void *)QUIRK_TDM_MODE_ENABLE)
    acp_card_drvdata.tdm_mode = dmi_id.driver_data;
    ret = acp_legacy_dai_links_create(card);
    if (ret) {
    dev_err(&pdev.dev,
    "Cannot create dai links for card (%s): %d\n",
    card.name, ret);
    goto out;
    }
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret) {
    dev_err(&pdev.dev,
    "devm_snd_soc_register_card(%s) failed: %d\n",
    card.name, ret);
    goto out;
    }
    out:
    return ret;
    }
    static const struct platform_device_id board_ids[] = {
    {
    .name = "acp3xalc56821019",
    .driver_data = (kernel_ulong_t)&rt5682_rt1019_data,
    },
    {
    .name = "acp3xalc5682sm98360",
    .driver_data = (kernel_ulong_t)&rt5682s_max_data,
    },
    {
    .name = "acp3xalc5682s1019",
    .driver_data = (kernel_ulong_t)&rt5682s_rt1019_data,
    },
    {
    .name = "acp3x-es83xx",
    .driver_data = (kernel_ulong_t)&es83xx_rn_data,
    },
    {
    .name = "rmb-nau8825-max",
    .driver_data = (kernel_ulong_t)&max_nau8825_data,
    },
    {
    .name = "rmb-rt5682s-rt1019",
    .driver_data = (kernel_ulong_t)&rt5682s_rt1019_rmb_data,
    },
    {
    .name = "acp-pdm-mach",
    .driver_data = (kernel_ulong_t)&acp_dmic_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(platform, board_ids);
    static struct platform_driver acp_asoc_audio = {
    .driver = {
    .pm = &snd_soc_pm_ops,
    .name = "acp_mach",
    },
    .probe = acp_asoc_probe,
    .id_table = board_ids,
    };
    module_platform_driver(acp_asoc_audio);
    MODULE_IMPORT_NS("SND_SOC_AMD_MACH");
    MODULE_DESCRIPTION("ACP chrome audio support");
    MODULE_LICENSE("GPL v2");
