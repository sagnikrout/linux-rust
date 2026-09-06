//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/acp/acp-sdw-sof-mach.c
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
// Copyright(c) 2024 Advanced Micro Devices, Inc.
//
// acp-sdw-sof-mach - ASoC Machine driver for AMD SoundWire platforms
//

    let mut sof_sdw_quirk: static unsigned long = RT711_JD1;
    let mut quirk_override: static int = -1;
    module_param_named(quirk, quirk_override, int, 0444);
    MODULE_PARM_DESC(quirk, "Board-specific quirk override");
#[no_mangle]
unsafe extern "C" fn log_quirks(dev: *mut device) {
    static void log_quirks(struct device *dev)
    {
    if (SOC_JACK_JDSRC(sof_sdw_quirk))
    dev_dbg(dev, "quirk realtek,jack-detect-source %ld\n",
    SOC_JACK_JDSRC(sof_sdw_quirk));
    if (sof_sdw_quirk & ASOC_SDW_ACP_DMIC)
    dev_dbg(dev, "quirk SOC_SDW_ACP_DMIC enabled\n");
    }
#[no_mangle]
unsafe extern "C" fn sof_sdw_quirk_cb(id: *const dmi_system_id) -> c_int {
    static int sof_sdw_quirk_cb(const struct dmi_system_id *id)
    {
    sof_sdw_quirk = (unsigned long)id.driver_data;
    return 1;
    }
    static const struct dmi_system_id sof_sdw_quirk_table[] = {
    {
    .callback = sof_sdw_quirk_cb,
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "AMD"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Birman-PHX"),
    },
    .driver_data = (void *)RT711_JD2,
    },
    {}
    };
    static struct snd_soc_dai_link_component platform_component[] = {
    {
// name might be overridden during probe
    .name = "0000:04:00.5",
    }
    };
    static const struct snd_soc_ops sdw_ops = {
    .startup = asoc_sdw_startup,
    .prepare = asoc_sdw_prepare,
    .trigger = asoc_sdw_trigger,
    .hw_params = asoc_sdw_hw_params,
    .hw_free = asoc_sdw_hw_free,
    .shutdown = asoc_sdw_shutdown,
    };
    static const char * const type_strings[] = {"SimpleJack", "SmartAmp", "SmartMic"};
    static int create_sdw_dailink(struct snd_soc_card *card,
    struct asoc_sdw_dailink *sof_dai,
    struct snd_soc_dai_link **dai_links,
    int *be_id, struct snd_soc_codec_conf **codec_conf)
    {
    struct device *dev = card.dev;
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct amd_mc_ctx *amd_ctx = (struct amd_mc_ctx *)ctx.private;
    struct asoc_sdw_endpoint *sof_end;
    int cpu_pin_id;
    int stream;
    int ret;
    list_for_each_entry(sof_end, &sof_dai.endpoints, list) {
    if (sof_end.name_prefix) {
    (*codec_conf).dlc.name = sof_end.codec_name;
    (*codec_conf).name_prefix = sof_end.name_prefix;
    (*codec_conf)++;
    }
    if (sof_end.include_sidecar) {
    ret = sof_end.codec_info.add_sidecar(card, dai_links, codec_conf);
    if (ret)
    return ret;
    }
    }
    for_each_pcm_streams(stream) {
    static const char * const sdw_stream_name[] = {
    "SDW%d-PIN%d-PLAYBACK",
    "SDW%d-PIN%d-CAPTURE",
    "SDW%d-PIN%d-PLAYBACK-%s",
    "SDW%d-PIN%d-CAPTURE-%s",
    };
    struct snd_soc_dai_link_ch_map *codec_maps;
    struct snd_soc_dai_link_component *codecs;
    struct snd_soc_dai_link_component *cpus;
    let mut num_cpus: c_int = hweight32(sof_dai.link_mask[stream]);
    let mut num_codecs: c_int = sof_dai.num_devs[stream];
    int playback, capture;
    let mut j: c_int = 0;
    char *name;
    if (!sof_dai.num_devs[stream])
    continue;
    sof_end = list_first_entry(&sof_dai.endpoints,
    struct asoc_sdw_endpoint, list);
// be_id = sof_end->dai_info->dailink[stream];
    if (*be_id < 0) {
    dev_err(dev, "Invalid dailink id %d\n", *be_id);
    return -EINVAL;
    }
    switch (amd_ctx.acp_rev) {
    case ACP63_PCI_REV:
    ret = get_acp63_cpu_pin_id(ffs(sof_end.link_mask - 1),
// be_id, &cpu_pin_id, dev);
    if (ret)
    return ret;
    break;
    case ACP70_PCI_REV:
    case ACP71_PCI_REV:
    case ACP72_PCI_REV:
    ret = get_acp70_cpu_pin_id(ffs(sof_end.link_mask - 1),
// be_id, &cpu_pin_id, dev);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
// create stream name according to first link id
    if (ctx.append_dai_type) {
    name = devm_kasprintf(dev, GFP_KERNEL,
    sdw_stream_name[stream + 2],
    ffs(sof_end.link_mask) - 1,
    cpu_pin_id,
    type_strings[sof_end.dai_info.dai_type]);
    } else {
    name = devm_kasprintf(dev, GFP_KERNEL,
    sdw_stream_name[stream],
    ffs(sof_end.link_mask) - 1,
    cpu_pin_id);
    }
    if (!name)
    return -ENOMEM;
    cpus = devm_kcalloc(dev, num_cpus, sizeof(*cpus), GFP_KERNEL);
    if (!cpus)
    return -ENOMEM;
    codecs = devm_kcalloc(dev, num_codecs, sizeof(*codecs), GFP_KERNEL);
    if (!codecs)
    return -ENOMEM;
    codec_maps = devm_kcalloc(dev, num_codecs, sizeof(*codec_maps), GFP_KERNEL);
    if (!codec_maps)
    return -ENOMEM;
    list_for_each_entry(sof_end, &sof_dai.endpoints, list) {
    if (!sof_end.dai_info.direction[stream])
    continue;
    let mut link_num: c_int = ffs(sof_end.link_mask) - 1;
    cpus.dai_name = devm_kasprintf(dev, GFP_KERNEL,
    "SDW%d Pin%d",
    link_num, cpu_pin_id);
    if (!cpus.dai_name)
    return -ENOMEM;
    dev_dbg(dev, "cpu.dai_name:%s\n", cpus.dai_name);
    codec_maps[j].cpu = 0;
    codec_maps[j].codec = j;
    codecs[j].name = sof_end.codec_name;
    codecs[j].dai_name = sof_end.dai_info.dai_name;
    j++;
    }
    WARN_ON(j != num_codecs);
    playback = (stream == SNDRV_PCM_STREAM_PLAYBACK);
    capture = (stream == SNDRV_PCM_STREAM_CAPTURE);
    asoc_sdw_init_dai_link(dev, *dai_links, be_id, name, playback, capture,
    cpus, num_cpus, platform_component,
    ARRAY_SIZE(platform_component), codecs, num_codecs,
    1, asoc_sdw_rtd_init, &sdw_ops);
//
// SoundWire DAILINKs use 'stream' functions and Bank Switch operations
// based on wait_for_completion(), tag them as 'nonatomic'.
//
    (*dai_links).nonatomic = true;
    (*dai_links).ch_maps = codec_maps;
    list_for_each_entry(sof_end, &sof_dai.endpoints, list) {
    if (sof_end.dai_info.init)
    sof_end.dai_info.init(card, *dai_links,
    sof_end.codec_info,
    playback);
    }
    (*dai_links)++;
    }
    return 0;
    }
    static int create_sdw_dailinks(struct snd_soc_card *card,
    struct snd_soc_dai_link **dai_links, int *be_id,
    struct asoc_sdw_dailink *sof_dais, int num_dais,
    struct snd_soc_codec_conf **codec_conf)
    {
    int i;
    int ret;
// generate DAI links by each sdw link
    for (i = 0; i < num_dais && sof_dais.initialised; i++) {
    let mut current_be_id: c_int = 0;
    ret = create_sdw_dailink(card, sof_dais, dai_links,
    &current_be_id, codec_conf);
    if (ret)
    return ret;
// Update the be_id to match the highest ID used for SDW link
    if (*be_id < current_be_id)
// be_id = current_be_id;
    sof_dais++;
    }
    return 0;
    }
    static int create_dmic_dailinks(struct snd_soc_card *card,
    struct snd_soc_dai_link **dai_links, int *be_id, int no_pcm)
    {
    struct device *dev = card.dev;
    int ret;
    ret = asoc_sdw_init_simple_dai_link(dev, *dai_links, be_id, "acp-dmic-codec",
    0, 1, // DMIC only supports capture
    "acp-sof-dmic", platform_component.name,
    "dmic-codec", "dmic-hifi", no_pcm,
    asoc_sdw_dmic_init, core::ptr::null_mut());
    if (ret)
    return ret;
    (*dai_links)++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sof_card_dai_links_create(card: *mut snd_soc_card) -> c_int {
    static int sof_card_dai_links_create(struct snd_soc_card *card)
    {
    struct device *dev = card.dev;
    struct snd_soc_acpi_mach *mach = dev_get_platdata(card.dev);
    let mut sdw_be_num: c_int = 0, dmic_num = 0;
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct snd_soc_acpi_mach_params *mach_params = &mach.mach_params;
    struct snd_soc_aux_dev *sof_aux;
    struct snd_soc_codec_conf *codec_conf;
    struct snd_soc_dai_link *dai_links;
    let mut num_devs: c_int = 0;
    let mut num_ends: c_int = 0;
    let mut num_aux: c_int = 0;
    int num_links;
    let mut be_id: c_int = 0;
    int ret;
    ret = asoc_sdw_count_sdw_endpoints(card, &num_devs, &num_ends, &num_aux);
    if (ret < 0) {
    dev_err(dev, "failed to count devices/endpoints: %d\n", ret);
    return ret;
    }
// One per DAI link, worst case is a DAI link for every endpoint
    struct asoc_sdw_dailink *sof_dais __free(kfree) =
    kzalloc_objs(*sof_dais, num_ends);
    if (!sof_dais)
    return -ENOMEM;
// One per endpoint, ie. each DAI on each codec/amp
    struct asoc_sdw_endpoint *sof_ends __free(kfree) =
    kzalloc_objs(*sof_ends, num_ends);
    if (!sof_ends)
    return -ENOMEM;
    sof_aux = devm_kcalloc(dev, num_aux, sizeof(*sof_aux), GFP_KERNEL);
    if (!sof_aux)
    return -ENOMEM;
    ret = asoc_sdw_parse_sdw_endpoints(dev, ctx, sof_aux, sof_dais, sof_ends, &num_devs);
    if (ret < 0)
    return ret;
    sdw_be_num = ret;
// enable dmic
    if (sof_sdw_quirk & ASOC_SDW_ACP_DMIC || mach_params.dmic_num)
    dmic_num = 1;
    dev_dbg(dev, "sdw %d, dmic %d", sdw_be_num, dmic_num);
    codec_conf = devm_kcalloc(dev, num_devs, sizeof(*codec_conf), GFP_KERNEL);
    if (!codec_conf)
    return -ENOMEM;
// allocate BE dailinks
    num_links = sdw_be_num + dmic_num;
    dai_links = devm_kcalloc(dev, num_links, sizeof(*dai_links), GFP_KERNEL);
    if (!dai_links)
    return -ENOMEM;
    card.codec_conf = codec_conf;
    card.num_configs = num_devs;
    card.dai_link = dai_links;
    card.num_links = num_links;
    card.aux_dev = sof_aux;
    card.num_aux_devs = num_aux;
// SDW
    if (sdw_be_num) {
    ret = create_sdw_dailinks(card, &dai_links, &be_id,
    sof_dais, num_ends, &codec_conf);
    if (ret)
    return ret;
    }
// dmic
    if (dmic_num > 0) {
    if (ctx.ignore_internal_dmic) {
    dev_warn(dev, "Ignoring ACP DMIC\n");
    } else {
    ret = create_dmic_dailinks(card, &dai_links, &be_id, 1);
    if (ret)
    return ret;
    }
    }
    WARN_ON(codec_conf != card.codec_conf + card.num_configs);
    WARN_ON(dai_links != card.dai_link + card.num_links);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc_probe(pdev: *mut platform_device) -> c_int {
    static int mc_probe(struct platform_device *pdev)
    {
    struct snd_soc_acpi_mach *mach = dev_get_platdata(&pdev.dev);
    struct snd_soc_card *card;
    struct amd_mc_ctx *amd_ctx;
    struct asoc_sdw_mc_private *ctx;
    let mut amp_num: c_int = 0, i;
    int ret;
    amd_ctx = devm_kzalloc(&pdev.dev, sizeof(*amd_ctx), GFP_KERNEL);
    if (!amd_ctx)
    return -ENOMEM;
    amd_ctx.acp_rev = mach.mach_params.subsystem_rev;
    amd_ctx.max_sdw_links = ACP63_SDW_MAX_LINKS;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.codec_info_list_count = asoc_sdw_get_codec_info_list_count();
    ctx.private = amd_ctx;
    card = &ctx.card;
    card.dev = &pdev.dev;
    card.name = "amd-soundwire";
    card.owner = THIS_MODULE;
    card.late_probe = asoc_sdw_card_late_probe;
    snd_soc_card_set_drvdata(card, ctx);
    dmi_check_system(sof_sdw_quirk_table);
    if (quirk_override != -1) {
    dev_info(&pdev.dev, "Overriding quirk 0x%lx => 0x%x\n",
    sof_sdw_quirk, quirk_override);
    sof_sdw_quirk = quirk_override;
    }
    log_quirks(&pdev.dev);
    ctx.mc_quirk = sof_sdw_quirk;
// reset amp_num to ensure amp_num++ starts from 0 in each probe
    for (i = 0; i < ctx.codec_info_list_count; i++)
    codec_info_list[i].amp_num = 0;
    ret = sof_card_dai_links_create(card);
    if (ret < 0)
    return ret;
//
// the default amp_num is zero for each codec and
// amp_num will only be increased for active amp
// codecs on used platform
//
    for (i = 0; i < ctx.codec_info_list_count; i++)
    amp_num += codec_info_list[i].amp_num;
    card.components = devm_kasprintf(&pdev.dev, GFP_KERNEL,
    " cfg-amp:%d", amp_num);
    if (!card.components)
    return -ENOMEM;
// Register the card
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret) {
    dev_err_probe(&pdev.dev, ret, "snd_soc_register_card failed %d\n", ret);
    asoc_sdw_mc_dailink_exit_loop(card);
    return ret;
    }
    platform_set_drvdata(pdev, card);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc_remove(pdev: *mut platform_device) {
    static void mc_remove(struct platform_device *pdev)
    {
    struct snd_soc_card *card = platform_get_drvdata(pdev);
    asoc_sdw_mc_dailink_exit_loop(card);
    }
    static const struct platform_device_id mc_id_table[] = {
    { .name = "amd_sof_sdw" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, mc_id_table);
    static struct platform_driver sof_sdw_driver = {
    .driver = {
    .name = "amd_sof_sdw",
    .pm = &snd_soc_pm_ops,
    },
    .probe = mc_probe,
    .remove = mc_remove,
    .id_table = mc_id_table,
    };
    module_platform_driver(sof_sdw_driver);
    MODULE_DESCRIPTION("ASoC AMD SoundWire Generic Machine driver");
    MODULE_AUTHOR("Vijendar Mukunda <Vijendar.Mukunda@amd.com");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("SND_SOC_SDW_UTILS");
    MODULE_IMPORT_NS("SND_SOC_AMD_SDW_MACH");
