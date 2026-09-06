//! Automatically rewritten from C to Rust
//! Source: sound/soc/ti/j721e-evm.c
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
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
//

//
// Maximum number of configuration entries for prefixes:
// CPB: 2 (mcasp10 + codec)
// IVI: 3 (mcasp0 + 2x codec)
//
pub const J721E_CODEC_CONF_COUNT: c_int = 5;
    enum j721e_audio_domain_id {
    J721E_AUDIO_DOMAIN_CPB = 0,
    J721E_AUDIO_DOMAIN_IVI,
    J721E_AUDIO_DOMAIN_LAST,
    };
pub const J721E_CLK_PARENT_48000: c_int = 0;
pub const J721E_CLK_PARENT_44100: c_int = 1;
pub const J721E_MAX_CLK_HSDIV: c_int = 128;
pub const PCM1368A_MAX_SYSCLK: c_int = 36864000;

    SND_SOC_DAIFMT_NB_NF |   \
    SND_SOC_DAIFMT_CBC_CFC)
    enum j721e_board_type {
    J721E_BOARD_CPB = 1,
    J721E_BOARD_CPB_IVI,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct j721e_audio_match_data {
    pub board_type: enum j721e_board_type,
    pub num_links: c_int,
    pub pll_rates: [c_uint; 2],
}

    static unsigned int ratios_for_pcm3168a[] = {
    256,
    512,
    768,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct j721e_audio_clocks {
    pub target: *mut clk,
    pub parent: [*mut clk; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j721e_audio_domain {
    pub codec: j721e_audio_clocks,
    pub mcasp: j721e_audio_clocks,
    pub parent_clk_id: c_int,
    pub active: c_int,
    pub active_link: c_uint,
    pub rate: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j721e_priv {
    pub dev: *mut device,
    pub card: snd_soc_card,
    pub codec_conf: [snd_soc_codec_conf; J721E_CODEC_CONF_COUNT],
    pub rate_range: snd_interval,
    pub match_data: *const j721e_audio_match_data,
    pub pll_rates: [u32; 2],
    pub hsdiv_rates: [c_uint; 2],
    pub audio_domains: [j721e_audio_domain; J721E_AUDIO_DOMAIN_LAST],
    pub mutex: mutex,
    pub dai_links: [snd_soc_dai_link; ],
}

    static const struct snd_soc_dapm_widget j721e_cpb_dapm_widgets[] = {
    SND_SOC_DAPM_HP("CPB Stereo HP 1", core::ptr::null_mut()),
    SND_SOC_DAPM_HP("CPB Stereo HP 2", core::ptr::null_mut()),
    SND_SOC_DAPM_HP("CPB Stereo HP 3", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("CPB Line Out", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("CPB Stereo Mic 1", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("CPB Stereo Mic 2", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("CPB Line In", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route j721e_cpb_dapm_routes[] = {
    {"CPB Stereo HP 1", core::ptr::null_mut(), "codec-1 AOUT1L"},
    {"CPB Stereo HP 1", core::ptr::null_mut(), "codec-1 AOUT1R"},
    {"CPB Stereo HP 2", core::ptr::null_mut(), "codec-1 AOUT2L"},
    {"CPB Stereo HP 2", core::ptr::null_mut(), "codec-1 AOUT2R"},
    {"CPB Stereo HP 3", core::ptr::null_mut(), "codec-1 AOUT3L"},
    {"CPB Stereo HP 3", core::ptr::null_mut(), "codec-1 AOUT3R"},
    {"CPB Line Out", core::ptr::null_mut(), "codec-1 AOUT4L"},
    {"CPB Line Out", core::ptr::null_mut(), "codec-1 AOUT4R"},
    {"codec-1 AIN1L", core::ptr::null_mut(), "CPB Stereo Mic 1"},
    {"codec-1 AIN1R", core::ptr::null_mut(), "CPB Stereo Mic 1"},
    {"codec-1 AIN2L", core::ptr::null_mut(), "CPB Stereo Mic 2"},
    {"codec-1 AIN2R", core::ptr::null_mut(), "CPB Stereo Mic 2"},
    {"codec-1 AIN3L", core::ptr::null_mut(), "CPB Line In"},
    {"codec-1 AIN3R", core::ptr::null_mut(), "CPB Line In"},
    };
    static const struct snd_soc_dapm_widget j721e_ivi_codec_a_dapm_widgets[] = {
    SND_SOC_DAPM_LINE("IVI A Line Out 1", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI A Line Out 2", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI A Line Out 3", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI A Line Out 4", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("IVI A Stereo Mic 1", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("IVI A Stereo Mic 2", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI A Line In", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route j721e_codec_a_dapm_routes[] = {
    {"IVI A Line Out 1", core::ptr::null_mut(), "codec-a AOUT1L"},
    {"IVI A Line Out 1", core::ptr::null_mut(), "codec-a AOUT1R"},
    {"IVI A Line Out 2", core::ptr::null_mut(), "codec-a AOUT2L"},
    {"IVI A Line Out 2", core::ptr::null_mut(), "codec-a AOUT2R"},
    {"IVI A Line Out 3", core::ptr::null_mut(), "codec-a AOUT3L"},
    {"IVI A Line Out 3", core::ptr::null_mut(), "codec-a AOUT3R"},
    {"IVI A Line Out 4", core::ptr::null_mut(), "codec-a AOUT4L"},
    {"IVI A Line Out 4", core::ptr::null_mut(), "codec-a AOUT4R"},
    {"codec-a AIN1L", core::ptr::null_mut(), "IVI A Stereo Mic 1"},
    {"codec-a AIN1R", core::ptr::null_mut(), "IVI A Stereo Mic 1"},
    {"codec-a AIN2L", core::ptr::null_mut(), "IVI A Stereo Mic 2"},
    {"codec-a AIN2R", core::ptr::null_mut(), "IVI A Stereo Mic 2"},
    {"codec-a AIN3L", core::ptr::null_mut(), "IVI A Line In"},
    {"codec-a AIN3R", core::ptr::null_mut(), "IVI A Line In"},
    };
    static const struct snd_soc_dapm_widget j721e_ivi_codec_b_dapm_widgets[] = {
    SND_SOC_DAPM_LINE("IVI B Line Out 1", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI B Line Out 2", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI B Line Out 3", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI B Line Out 4", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("IVI B Stereo Mic 1", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("IVI B Stereo Mic 2", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("IVI B Line In", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route j721e_codec_b_dapm_routes[] = {
    {"IVI B Line Out 1", core::ptr::null_mut(), "codec-b AOUT1L"},
    {"IVI B Line Out 1", core::ptr::null_mut(), "codec-b AOUT1R"},
    {"IVI B Line Out 2", core::ptr::null_mut(), "codec-b AOUT2L"},
    {"IVI B Line Out 2", core::ptr::null_mut(), "codec-b AOUT2R"},
    {"IVI B Line Out 3", core::ptr::null_mut(), "codec-b AOUT3L"},
    {"IVI B Line Out 3", core::ptr::null_mut(), "codec-b AOUT3R"},
    {"IVI B Line Out 4", core::ptr::null_mut(), "codec-b AOUT4L"},
    {"IVI B Line Out 4", core::ptr::null_mut(), "codec-b AOUT4R"},
    {"codec-b AIN1L", core::ptr::null_mut(), "IVI B Stereo Mic 1"},
    {"codec-b AIN1R", core::ptr::null_mut(), "IVI B Stereo Mic 1"},
    {"codec-b AIN2L", core::ptr::null_mut(), "IVI B Stereo Mic 2"},
    {"codec-b AIN2R", core::ptr::null_mut(), "IVI B Stereo Mic 2"},
    {"codec-b AIN3L", core::ptr::null_mut(), "IVI B Line In"},
    {"codec-b AIN3R", core::ptr::null_mut(), "IVI B Line In"},
    };
    static int j721e_configure_refclk(struct j721e_priv *priv,
    unsigned int audio_domain, unsigned int rate)
    {
    struct j721e_audio_domain *domain = &priv.audio_domains[audio_domain];
    unsigned int scki;
    let mut ret: c_int = -EINVAL;
    int i, clk_id;
    if (!(rate % 8000) && priv.pll_rates[J721E_CLK_PARENT_48000])
    clk_id = J721E_CLK_PARENT_48000;
#[no_mangle]
pub unsafe extern "C" fn if(priv->pll_rates[J721E_CLK_PARENT_44100]: !(rate % 11025) &&) -> else {
    else if (!(rate % 11025) && priv.pll_rates[J721E_CLK_PARENT_44100])
    clk_id = J721E_CLK_PARENT_44100;
#[no_mangle]
pub unsafe extern "C" fn if(priv->pll_rates[J721E_CLK_PARENT_48000]: !(rate % 11025) &&) -> else {
    else if (!(rate % 11025) && priv.pll_rates[J721E_CLK_PARENT_48000])
    clk_id = J721E_CLK_PARENT_48000;
    else
    return ret;
    for (i = 0; i < ARRAY_SIZE(ratios_for_pcm3168a); i++) {
    scki = ratios_for_pcm3168a[i] * rate;
    if (priv.pll_rates[clk_id] / scki <= J721E_MAX_CLK_HSDIV) {
    ret = 0;
    break;
    }
    }
    if (ret) {
    dev_err(priv.dev, "No valid clock configuration for %u Hz\n",
    rate);
    return ret;
    }
    if (domain.parent_clk_id == -1 || priv.hsdiv_rates[domain.parent_clk_id] != scki) {
    dev_dbg(priv.dev,
    "domain%u configuration for %u Hz: %s, %dxFS (SCKI: %u Hz)\n",
    audio_domain, rate,
    clk_id == J721E_CLK_PARENT_48000 ? "PLL4" : "PLL15",
    ratios_for_pcm3168a[i], scki);
    if (domain.parent_clk_id != clk_id) {
    ret = clk_set_parent(domain.codec.target,
    domain.codec.parent[clk_id]);
    if (ret)
    return ret;
    ret = clk_set_parent(domain.mcasp.target,
    domain.mcasp.parent[clk_id]);
    if (ret)
    return ret;
    domain.parent_clk_id = clk_id;
    }
    ret = clk_set_rate(domain.codec.target, scki);
    if (ret) {
    dev_err(priv.dev, "codec set rate failed for %u Hz\n",
    scki);
    return ret;
    }
    ret = clk_set_rate(domain.mcasp.target, scki);
    if (!ret) {
    priv.hsdiv_rates[domain.parent_clk_id] = scki;
    } else {
    dev_err(priv.dev, "mcasp set rate failed for %u Hz\n",
    scki);
    return ret;
    }
    }
    return ret;
    }
    static int j721e_rule_rate(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct snd_interval *t = rule.private;
    return snd_interval_refine(hw_param_interval(params, rule.var), t);
    }
#[no_mangle]
unsafe extern "C" fn j721e_audio_startup(substream: *mut snd_pcm_substream) -> c_int {
    static int j721e_audio_startup(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct j721e_priv *priv = snd_soc_card_get_drvdata(rtd.card);
    let mut domain_id: c_uint = rtd.dai_link.id;
    struct j721e_audio_domain *domain = &priv.audio_domains[domain_id];
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_soc_dai *codec_dai;
    unsigned int active_rate;
    let mut ret: c_int = 0;
    int i;
    mutex_lock(&priv.mutex);
    domain.active++;
    for (i = 0; i < J721E_AUDIO_DOMAIN_LAST; i++) {
    active_rate = priv.audio_domains[i].rate;
    if (active_rate)
    break;
    }
    if (active_rate)
    ret = snd_pcm_hw_constraint_single(substream.runtime,
    SNDRV_PCM_HW_PARAM_RATE,
    active_rate);
    else
    ret = snd_pcm_hw_rule_add(substream.runtime, 0,
    SNDRV_PCM_HW_PARAM_RATE,
    j721e_rule_rate, &priv.rate_range,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (ret)
    goto out;
// Reset TDM slots to 32
    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, 32);
    if (ret && ret != -ENOTSUPP)
    goto out;
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 2, 32);
    if (ret && ret != -ENOTSUPP)
    goto out;
    }
    if (ret == -ENOTSUPP)
    ret = 0;
    out:
    if (ret)
    domain.active--;
    mutex_unlock(&priv.mutex);
    return ret;
    }
    static int j721e_audio_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_card *card = rtd.card;
    struct j721e_priv *priv = snd_soc_card_get_drvdata(card);
    let mut domain_id: c_uint = rtd.dai_link.id;
    struct j721e_audio_domain *domain = &priv.audio_domains[domain_id];
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_soc_dai *codec_dai;
    unsigned int sysclk_rate;
    let mut slot_width: c_int = 32;
    int ret;
    int i;
    guard(mutex)(&priv.mutex);
    if (domain.rate && domain.rate != params_rate(params))
    return -EINVAL;
    if (params_width(params) == 16)
    slot_width = 16;
    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, slot_width);
    if (ret && ret != -ENOTSUPP)
    return ret;
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 2,
    slot_width);
    if (ret && ret != -ENOTSUPP)
    return ret;
    }
    ret = j721e_configure_refclk(priv, domain_id, params_rate(params));
    if (ret)
    return ret;
    sysclk_rate = priv.hsdiv_rates[domain.parent_clk_id];
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, sysclk_rate,
    SND_SOC_CLOCK_IN);
    if (ret && ret != -ENOTSUPP) {
    dev_err(priv.dev,
    "codec set_sysclk failed for %u Hz\n",
    sysclk_rate);
    return ret;
    }
    }
    ret = snd_soc_dai_set_sysclk(cpu_dai, MCASP_CLK_HCLK_AUXCLK,
    sysclk_rate, SND_SOC_CLOCK_IN);
    if (ret && ret != -ENOTSUPP) {
    dev_err(priv.dev, "mcasp set_sysclk failed for %u Hz\n",
    sysclk_rate);
    } else {
    domain.rate = params_rate(params);
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn j721e_audio_shutdown(substream: *mut snd_pcm_substream) {
    static void j721e_audio_shutdown(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct j721e_priv *priv = snd_soc_card_get_drvdata(rtd.card);
    let mut domain_id: c_uint = rtd.dai_link.id;
    struct j721e_audio_domain *domain = &priv.audio_domains[domain_id];
    guard(mutex)(&priv.mutex);
    domain.active--;
    if (!domain.active) {
    domain.rate = 0;
    domain.active_link = 0;
    }
    }
    static const struct snd_soc_ops j721e_audio_ops = {
    .startup = j721e_audio_startup,
    .hw_params = j721e_audio_hw_params,
    .shutdown = j721e_audio_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn j721e_audio_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int j721e_audio_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct j721e_priv *priv = snd_soc_card_get_drvdata(rtd.card);
    let mut domain_id: c_uint = rtd.dai_link.id;
    struct j721e_audio_domain *domain = &priv.audio_domains[domain_id];
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_soc_dai *codec_dai;
    unsigned int sysclk_rate;
    int i, ret;
// Set up initial clock configuration
    ret = j721e_configure_refclk(priv, domain_id, 48000);
    if (ret)
    return ret;
    sysclk_rate = priv.hsdiv_rates[domain.parent_clk_id];
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, sysclk_rate,
    SND_SOC_CLOCK_IN);
    if (ret && ret != -ENOTSUPP)
    return ret;
    }
    ret = snd_soc_dai_set_sysclk(cpu_dai, MCASP_CLK_HCLK_AUXCLK,
    sysclk_rate, SND_SOC_CLOCK_IN);
    if (ret && ret != -ENOTSUPP)
    return ret;
// Set initial tdm slots
    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, 32);
    if (ret && ret != -ENOTSUPP)
    return ret;
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 2, 32);
    if (ret && ret != -ENOTSUPP)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn j721e_audio_init_ivi(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int j721e_audio_init_ivi(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(rtd.card);
    snd_soc_dapm_new_controls(dapm, j721e_ivi_codec_a_dapm_widgets,
    ARRAY_SIZE(j721e_ivi_codec_a_dapm_widgets));
    snd_soc_dapm_add_routes(dapm, j721e_codec_a_dapm_routes,
    ARRAY_SIZE(j721e_codec_a_dapm_routes));
    snd_soc_dapm_new_controls(dapm, j721e_ivi_codec_b_dapm_widgets,
    ARRAY_SIZE(j721e_ivi_codec_b_dapm_widgets));
    snd_soc_dapm_add_routes(dapm, j721e_codec_b_dapm_routes,
    ARRAY_SIZE(j721e_codec_b_dapm_routes));
    return j721e_audio_init(rtd);
    }
    static int j721e_get_clocks(struct device *dev,
    struct j721e_audio_clocks *clocks, char *prefix)
    {
    struct clk *parent;
    char *clk_name;
    int ret;
    clocks.target = devm_clk_get(dev, prefix);
    if (IS_ERR(clocks.target))
    return dev_err_probe(dev, PTR_ERR(clocks.target),
    "failed to acquire %s\n", prefix);
    clk_name = kasprintf(GFP_KERNEL, "%s-48000", prefix);
    if (clk_name) {
    parent = devm_clk_get(dev, clk_name);
    kfree(clk_name);
    if (IS_ERR(parent)) {
    ret = PTR_ERR(parent);
    if (ret == -EPROBE_DEFER)
    return ret;
    dev_dbg(dev, "no 48KHz parent for %s: %d\n", prefix, ret);
    parent = core::ptr::null_mut();
    }
    clocks.parent[J721E_CLK_PARENT_48000] = parent;
    } else {
    return -ENOMEM;
    }
    clk_name = kasprintf(GFP_KERNEL, "%s-44100", prefix);
    if (clk_name) {
    parent = devm_clk_get(dev, clk_name);
    kfree(clk_name);
    if (IS_ERR(parent)) {
    ret = PTR_ERR(parent);
    if (ret == -EPROBE_DEFER)
    return ret;
    dev_dbg(dev, "no 44.1KHz parent for %s: %d\n", prefix, ret);
    parent = core::ptr::null_mut();
    }
    clocks.parent[J721E_CLK_PARENT_44100] = parent;
    } else {
    return -ENOMEM;
    }
    if (!clocks.parent[J721E_CLK_PARENT_44100] &&
    !clocks.parent[J721E_CLK_PARENT_48000]) {
    dev_err(dev, "At least one parent clock is needed for %s\n",
    prefix);
    return -EINVAL;
    }
    return 0;
    }
    static const struct j721e_audio_match_data j721e_cpb_data = {
    .board_type = J721E_BOARD_CPB,
    .num_links = 2, /* CPB pcm3168a */
    .pll_rates = {
    [J721E_CLK_PARENT_44100] = 1083801600, /* PLL15 */
    [J721E_CLK_PARENT_48000] = 1179648000, /* PLL4 */
    },
    };
    static const struct j721e_audio_match_data j721e_cpb_ivi_data = {
    .board_type = J721E_BOARD_CPB_IVI,
    .num_links = 4, /* CPB pcm3168a + 2x pcm3168a on IVI */
    .pll_rates = {
    [J721E_CLK_PARENT_44100] = 1083801600, /* PLL15 */
    [J721E_CLK_PARENT_48000] = 1179648000, /* PLL4 */
    },
    };
    static const struct j721e_audio_match_data j7200_cpb_data = {
    .board_type = J721E_BOARD_CPB,
    .num_links = 2, /* CPB pcm3168a */
    .pll_rates = {
    [J721E_CLK_PARENT_48000] = 2359296000u, /* PLL4 */
    },
    };
    static const struct of_device_id j721e_audio_of_match[] = {
    {
    .compatible = "ti,j721e-cpb-audio",
    .data = &j721e_cpb_data,
    }, {
    .compatible = "ti,j721e-cpb-ivi-audio",
    .data = &j721e_cpb_ivi_data,
    }, {
    .compatible = "ti,j7200-cpb-audio",
    .data = &j7200_cpb_data,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, j721e_audio_of_match);
#[no_mangle]
unsafe extern "C" fn j721e_calculate_rate_range(priv: *mut j721e_priv) -> c_int {
    static int j721e_calculate_rate_range(struct j721e_priv *priv)
    {
    const struct j721e_audio_match_data *match_data = priv.match_data;
    struct j721e_audio_clocks *domain_clocks;
    unsigned int min_rate, max_rate, pll_rate;
    struct clk *pll;
    domain_clocks = &priv.audio_domains[J721E_AUDIO_DOMAIN_CPB].mcasp;
    pll = clk_get_parent(domain_clocks.parent[J721E_CLK_PARENT_44100]);
    if (IS_ERR_OR_NULL(pll)) {
    priv.pll_rates[J721E_CLK_PARENT_44100] =
    match_data.pll_rates[J721E_CLK_PARENT_44100];
    } else {
    priv.pll_rates[J721E_CLK_PARENT_44100] = clk_get_rate(pll);
    clk_put(pll);
    }
    pll = clk_get_parent(domain_clocks.parent[J721E_CLK_PARENT_48000]);
    if (IS_ERR_OR_NULL(pll)) {
    priv.pll_rates[J721E_CLK_PARENT_48000] =
    match_data.pll_rates[J721E_CLK_PARENT_48000];
    } else {
    priv.pll_rates[J721E_CLK_PARENT_48000] = clk_get_rate(pll);
    clk_put(pll);
    }
    if (!priv.pll_rates[J721E_CLK_PARENT_44100] &&
    !priv.pll_rates[J721E_CLK_PARENT_48000]) {
    dev_err(priv.dev, "At least one PLL is needed\n");
    return -EINVAL;
    }
    if (priv.pll_rates[J721E_CLK_PARENT_44100])
    pll_rate = priv.pll_rates[J721E_CLK_PARENT_44100];
    else
    pll_rate = priv.pll_rates[J721E_CLK_PARENT_48000];
    min_rate = pll_rate / J721E_MAX_CLK_HSDIV;
    min_rate /= ratios_for_pcm3168a[ARRAY_SIZE(ratios_for_pcm3168a) - 1];
    if (priv.pll_rates[J721E_CLK_PARENT_48000])
    pll_rate = priv.pll_rates[J721E_CLK_PARENT_48000];
    else
    pll_rate = priv.pll_rates[J721E_CLK_PARENT_44100];
    if (pll_rate > PCM1368A_MAX_SYSCLK)
    pll_rate = PCM1368A_MAX_SYSCLK;
    max_rate = pll_rate / ratios_for_pcm3168a[0];
    snd_interval_any(&priv.rate_range);
    priv.rate_range.min = min_rate;
    priv.rate_range.max = max_rate;
    return 0;
    }
    static int j721e_soc_probe_cpb(struct j721e_priv *priv, int *link_idx,
    int *conf_idx)
    {
    struct device_node *node = priv.dev.of_node;
    struct snd_soc_dai_link_component *compnent;
    struct device_node *dai_node, *codec_node;
    struct j721e_audio_domain *domain;
    int comp_count, comp_idx;
    int ret;
    dai_node = of_parse_phandle(node, "ti,cpb-mcasp", 0);
    if (!dai_node) {
    dev_err(priv.dev, "CPB McASP node is not provided\n");
    return -EINVAL;
    }
    codec_node = of_parse_phandle(node, "ti,cpb-codec", 0);
    if (!codec_node) {
    dev_err(priv.dev, "CPB codec node is not provided\n");
    ret = -EINVAL;
    goto put_dai_node;
    }
    domain = &priv.audio_domains[J721E_AUDIO_DOMAIN_CPB];
    ret = j721e_get_clocks(priv.dev, &domain.codec, "cpb-codec-scki");
    if (ret)
    goto put_codec_node;
    ret = j721e_get_clocks(priv.dev, &domain.mcasp, "cpb-mcasp-auxclk");
    if (ret)
    goto put_codec_node;
//
// Common Processor Board, two links
// Link 1: McASP10 -> pcm3168a_1 DAC
// Link 2: McASP10 <- pcm3168a_1 ADC
//
    comp_count = 6;
    compnent = devm_kcalloc(priv.dev, comp_count, sizeof(*compnent),
    GFP_KERNEL);
    if (!compnent) {
    ret = -ENOMEM;
    goto put_codec_node;
    }
    comp_idx = 0;
    priv.dai_links[*link_idx].cpus = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_cpus = 1;
    priv.dai_links[*link_idx].codecs = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_codecs = 1;
    priv.dai_links[*link_idx].platforms = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_platforms = 1;
    priv.dai_links[*link_idx].name = "CPB PCM3168A Playback";
    priv.dai_links[*link_idx].stream_name = "CPB PCM3168A Analog";
    priv.dai_links[*link_idx].cpus.of_node = dai_node;
    priv.dai_links[*link_idx].platforms.of_node = dai_node;
    priv.dai_links[*link_idx].codecs.of_node = codec_node;
    priv.dai_links[*link_idx].codecs.dai_name = "pcm3168a-dac";
    priv.dai_links[*link_idx].playback_only = 1;
    priv.dai_links[*link_idx].id = J721E_AUDIO_DOMAIN_CPB;
    priv.dai_links[*link_idx].dai_fmt = J721E_DAI_FMT;
    priv.dai_links[*link_idx].init = j721e_audio_init;
    priv.dai_links[*link_idx].ops = &j721e_audio_ops;
    (*link_idx)++;
    priv.dai_links[*link_idx].cpus = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_cpus = 1;
    priv.dai_links[*link_idx].codecs = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_codecs = 1;
    priv.dai_links[*link_idx].platforms = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_platforms = 1;
    priv.dai_links[*link_idx].name = "CPB PCM3168A Capture";
    priv.dai_links[*link_idx].stream_name = "CPB PCM3168A Analog";
    priv.dai_links[*link_idx].cpus.of_node = dai_node;
    priv.dai_links[*link_idx].platforms.of_node = dai_node;
    priv.dai_links[*link_idx].codecs.of_node = codec_node;
    priv.dai_links[*link_idx].codecs.dai_name = "pcm3168a-adc";
    priv.dai_links[*link_idx].capture_only = 1;
    priv.dai_links[*link_idx].id = J721E_AUDIO_DOMAIN_CPB;
    priv.dai_links[*link_idx].dai_fmt = J721E_DAI_FMT;
    priv.dai_links[*link_idx].init = j721e_audio_init;
    priv.dai_links[*link_idx].ops = &j721e_audio_ops;
    (*link_idx)++;
    priv.codec_conf[*conf_idx].dlc.of_node = codec_node;
    priv.codec_conf[*conf_idx].name_prefix = "codec-1";
    (*conf_idx)++;
    priv.codec_conf[*conf_idx].dlc.of_node = dai_node;
    priv.codec_conf[*conf_idx].name_prefix = "McASP10";
    (*conf_idx)++;
    return 0;
    put_codec_node:
    of_node_put(codec_node);
    put_dai_node:
    of_node_put(dai_node);
    return ret;
    }
    static int j721e_soc_probe_ivi(struct j721e_priv *priv, int *link_idx,
    int *conf_idx)
    {
    struct device_node *node = priv.dev.of_node;
    struct snd_soc_dai_link_component *compnent;
    struct device_node *dai_node, *codeca_node, *codecb_node;
    struct j721e_audio_domain *domain;
    int comp_count, comp_idx;
    int ret;
    if (priv.match_data.board_type != J721E_BOARD_CPB_IVI)
    return 0;
    dai_node = of_parse_phandle(node, "ti,ivi-mcasp", 0);
    if (!dai_node) {
    dev_err(priv.dev, "IVI McASP node is not provided\n");
    return -EINVAL;
    }
    codeca_node = of_parse_phandle(node, "ti,ivi-codec-a", 0);
    if (!codeca_node) {
    dev_err(priv.dev, "IVI codec-a node is not provided\n");
    ret = -EINVAL;
    goto put_dai_node;
    }
    codecb_node = of_parse_phandle(node, "ti,ivi-codec-b", 0);
    if (!codecb_node) {
    dev_warn(priv.dev, "IVI codec-b node is not provided\n");
    ret = 0;
    goto put_codeca_node;
    }
    domain = &priv.audio_domains[J721E_AUDIO_DOMAIN_IVI];
    ret = j721e_get_clocks(priv.dev, &domain.codec, "ivi-codec-scki");
    if (ret)
    goto put_codecb_node;
    ret = j721e_get_clocks(priv.dev, &domain.mcasp, "ivi-mcasp-auxclk");
    if (ret)
    goto put_codecb_node;
//
// IVI extension, two links
// Link 1: McASP0 -> pcm3168a_a DAC
// \> pcm3168a_b DAC
// Link 2: McASP0 <- pcm3168a_a ADC
// \ pcm3168a_b ADC
//
    comp_count = 8;
    compnent = devm_kcalloc(priv.dev, comp_count, sizeof(*compnent),
    GFP_KERNEL);
    if (!compnent) {
    ret = -ENOMEM;
    goto put_codecb_node;
    }
    comp_idx = 0;
    priv.dai_links[*link_idx].cpus = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_cpus = 1;
    priv.dai_links[*link_idx].platforms = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_platforms = 1;
    priv.dai_links[*link_idx].codecs = &compnent[comp_idx];
    priv.dai_links[*link_idx].num_codecs = 2;
    comp_idx += 2;
    priv.dai_links[*link_idx].name = "IVI 2xPCM3168A Playback";
    priv.dai_links[*link_idx].stream_name = "IVI 2xPCM3168A Analog";
    priv.dai_links[*link_idx].cpus.of_node = dai_node;
    priv.dai_links[*link_idx].platforms.of_node = dai_node;
    priv.dai_links[*link_idx].codecs[0].of_node = codeca_node;
    priv.dai_links[*link_idx].codecs[0].dai_name = "pcm3168a-dac";
    priv.dai_links[*link_idx].codecs[1].of_node = codecb_node;
    priv.dai_links[*link_idx].codecs[1].dai_name = "pcm3168a-dac";
    priv.dai_links[*link_idx].playback_only = 1;
    priv.dai_links[*link_idx].id = J721E_AUDIO_DOMAIN_IVI;
    priv.dai_links[*link_idx].dai_fmt = J721E_DAI_FMT;
    priv.dai_links[*link_idx].init = j721e_audio_init_ivi;
    priv.dai_links[*link_idx].ops = &j721e_audio_ops;
    (*link_idx)++;
    priv.dai_links[*link_idx].cpus = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_cpus = 1;
    priv.dai_links[*link_idx].platforms = &compnent[comp_idx++];
    priv.dai_links[*link_idx].num_platforms = 1;
    priv.dai_links[*link_idx].codecs = &compnent[comp_idx];
    priv.dai_links[*link_idx].num_codecs = 2;
    priv.dai_links[*link_idx].name = "IVI 2xPCM3168A Capture";
    priv.dai_links[*link_idx].stream_name = "IVI 2xPCM3168A Analog";
    priv.dai_links[*link_idx].cpus.of_node = dai_node;
    priv.dai_links[*link_idx].platforms.of_node = dai_node;
    priv.dai_links[*link_idx].codecs[0].of_node = codeca_node;
    priv.dai_links[*link_idx].codecs[0].dai_name = "pcm3168a-adc";
    priv.dai_links[*link_idx].codecs[1].of_node = codecb_node;
    priv.dai_links[*link_idx].codecs[1].dai_name = "pcm3168a-adc";
    priv.dai_links[*link_idx].capture_only = 1;
    priv.dai_links[*link_idx].id = J721E_AUDIO_DOMAIN_IVI;
    priv.dai_links[*link_idx].dai_fmt = J721E_DAI_FMT;
    priv.dai_links[*link_idx].init = j721e_audio_init;
    priv.dai_links[*link_idx].ops = &j721e_audio_ops;
    (*link_idx)++;
    priv.codec_conf[*conf_idx].dlc.of_node = codeca_node;
    priv.codec_conf[*conf_idx].name_prefix = "codec-a";
    (*conf_idx)++;
    priv.codec_conf[*conf_idx].dlc.of_node = codecb_node;
    priv.codec_conf[*conf_idx].name_prefix = "codec-b";
    (*conf_idx)++;
    priv.codec_conf[*conf_idx].dlc.of_node = dai_node;
    priv.codec_conf[*conf_idx].name_prefix = "McASP0";
    (*conf_idx)++;
    return 0;
    put_codecb_node:
    of_node_put(codecb_node);
    put_codeca_node:
    of_node_put(codeca_node);
    put_dai_node:
    of_node_put(dai_node);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn j721e_soc_probe(pdev: *mut platform_device) -> c_int {
    static int j721e_soc_probe(struct platform_device *pdev)
    {
    const struct j721e_audio_match_data *match;
    struct snd_soc_card *card;
    struct j721e_priv *priv;
    int link_cnt, conf_cnt, ret, i;
    match = of_device_get_match_data(&pdev.dev);
    if (!match) {
    dev_err(&pdev.dev, "No compatible match found\n");
    return -ENODEV;
    }
    priv = devm_kzalloc(&pdev.dev,
    struct_size(priv, dai_links, match.num_links), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.match_data = match;
    for (i = 0; i < J721E_AUDIO_DOMAIN_LAST; i++)
    priv.audio_domains[i].parent_clk_id = -1;
    priv.dev = &pdev.dev;
    card = &priv.card;
    card.dev = &pdev.dev;
    card.owner = THIS_MODULE;
    card.dapm_widgets = j721e_cpb_dapm_widgets;
    card.num_dapm_widgets = ARRAY_SIZE(j721e_cpb_dapm_widgets);
    card.dapm_routes = j721e_cpb_dapm_routes;
    card.num_dapm_routes = ARRAY_SIZE(j721e_cpb_dapm_routes);
    card.fully_routed = 1;
    ret = snd_soc_of_parse_card_name(card, "model");
    if (ret)
    return ret;
    link_cnt = 0;
    conf_cnt = 0;
    ret = j721e_soc_probe_cpb(priv, &link_cnt, &conf_cnt);
    if (ret)
    return ret;
    ret = j721e_soc_probe_ivi(priv, &link_cnt, &conf_cnt);
    if (ret)
    return ret;
    card.dai_link = priv.dai_links;
    card.num_links = link_cnt;
    card.codec_conf = priv.codec_conf;
    card.num_configs = conf_cnt;
    ret = j721e_calculate_rate_range(priv);
    if (ret)
    return ret;
    snd_soc_card_set_drvdata(card, priv);
    mutex_init(&priv.mutex);
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret)
    dev_err_probe(&pdev.dev, ret,
    "devm_snd_soc_register_card() failed: %d\n",
    ret);
    return ret;
    }
    static struct platform_driver j721e_soc_driver = {
    .driver = {
    .name = "j721e-audio",
    .pm = &snd_soc_pm_ops,
    .of_match_table = j721e_audio_of_match,
    },
    .probe = j721e_soc_probe,
    };
    module_platform_driver(j721e_soc_driver);
    MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>");
    MODULE_DESCRIPTION("ASoC machine driver for j721e Common Processor Board");
    MODULE_LICENSE("GPL v2");
