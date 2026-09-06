//! Automatically rewritten from C to Rust
//! Source: sound/soc/ti/omap-hdmi.c
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
// omap-hdmi-audio.c -- OMAP4+ DSS HDMI audio support library
//
// Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
//
// Author: Jyri Sarha <jsarha@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_data {
    pub ops: *const omap_hdmi_audio_ops,
    pub dssdev: *mut device,
    pub dma_data: snd_dmaengine_dai_dma_data,
    pub dss_audio: omap_dss_audio,
    pub iec: snd_aes_iec958,
    pub cea: snd_cea_861_aud_if,
    pub current_stream_lock: mutex,
    pub current_stream: *mut snd_pcm_substream,
}

    static
    struct hdmi_audio_data *card_drvdata_substream(struct snd_pcm_substream *ss)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(ss);
    return snd_soc_card_get_drvdata(rtd.card);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_dai_abort(dev: *mut device) {
    static void hdmi_dai_abort(struct device *dev)
    {
    struct hdmi_audio_data *ad = dev_get_drvdata(dev);
    guard(mutex)(&ad.current_stream_lock);
    if (ad.current_stream && ad.current_stream.runtime &&
    snd_pcm_running(ad.current_stream)) {
    dev_err(dev, "HDMI display disabled, aborting playback\n");
    snd_pcm_stream_lock_irq(ad.current_stream);
    snd_pcm_stop(ad.current_stream, SNDRV_PCM_STATE_DISCONNECTED);
    snd_pcm_stream_unlock_irq(ad.current_stream);
    }
    }
    static int hdmi_dai_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct hdmi_audio_data *ad = card_drvdata_substream(substream);
    int ret;
//
// Make sure that the period bytes are multiple of the DMA packet size.
// Largest packet size we use is 32 32-bit words = 128 bytes
//
    ret = snd_pcm_hw_constraint_step(substream.runtime, 0,
    SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 128);
    if (ret < 0) {
    dev_err(dai.dev, "Could not apply period constraint: %d\n",
    ret);
    return ret;
    }
    ret = snd_pcm_hw_constraint_step(substream.runtime, 0,
    SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 128);
    if (ret < 0) {
    dev_err(dai.dev, "Could not apply buffer constraint: %d\n",
    ret);
    return ret;
    }
    snd_soc_dai_set_dma_data(dai, substream, &ad.dma_data);
    scoped_guard(mutex, &ad.current_stream_lock)
    ad.current_stream = substream;
    ret = ad.ops.audio_startup(ad.dssdev, hdmi_dai_abort);
    if (ret) {
    scoped_guard(mutex, &ad.current_stream_lock)
    ad.current_stream = core::ptr::null_mut();
    }
    return ret;
    }
    static int hdmi_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct hdmi_audio_data *ad = card_drvdata_substream(substream);
    struct snd_aes_iec958 *iec = &ad.iec;
    struct snd_cea_861_aud_if *cea = &ad.cea;
    WARN_ON(ad.current_stream != substream);
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    ad.dma_data.maxburst = 16;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    ad.dma_data.maxburst = 32;
    break;
    default:
    dev_err(dai.dev, "format not supported!\n");
    return -EINVAL;
    }
    ad.dss_audio.iec = iec;
    ad.dss_audio.cea = cea;
//
// fill the IEC-60958 channel status word
//
// initialize the word bytes
    memset(iec.status, 0, sizeof(iec.status));
// specify IEC-60958-3 (commercial use)
    iec.status[0] &= ~IEC958_AES0_PROFESSIONAL;
// specify that the audio is LPCM
    iec.status[0] &= ~IEC958_AES0_NONAUDIO;
    iec.status[0] |= IEC958_AES0_CON_NOT_COPYRIGHT;
    iec.status[0] |= IEC958_AES0_CON_EMPHASIS_NONE;
    iec.status[1] = IEC958_AES1_CON_GENERAL;
    iec.status[2] |= IEC958_AES2_CON_SOURCE_UNSPEC;
    iec.status[2] |= IEC958_AES2_CON_CHANNEL_UNSPEC;
    switch (params_rate(params)) {
    case 32000:
    iec.status[3] |= IEC958_AES3_CON_FS_32000;
    break;
    case 44100:
    iec.status[3] |= IEC958_AES3_CON_FS_44100;
    break;
    case 48000:
    iec.status[3] |= IEC958_AES3_CON_FS_48000;
    break;
    case 88200:
    iec.status[3] |= IEC958_AES3_CON_FS_88200;
    break;
    case 96000:
    iec.status[3] |= IEC958_AES3_CON_FS_96000;
    break;
    case 176400:
    iec.status[3] |= IEC958_AES3_CON_FS_176400;
    break;
    case 192000:
    iec.status[3] |= IEC958_AES3_CON_FS_192000;
    break;
    default:
    dev_err(dai.dev, "rate not supported!\n");
    return -EINVAL;
    }
// specify the clock accuracy
    iec.status[3] |= IEC958_AES3_CON_CLOCK_1000PPM;
//
// specify the word length. The same word length value can mean
// two different lengths. Hence, we need to specify the maximum
// word length as well.
//
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    iec.status[4] |= IEC958_AES4_CON_WORDLEN_20_16;
    iec.status[4] &= ~IEC958_AES4_CON_MAX_WORDLEN_24;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    iec.status[4] |= IEC958_AES4_CON_WORDLEN_24_20;
    iec.status[4] |= IEC958_AES4_CON_MAX_WORDLEN_24;
    break;
    default:
    dev_err(dai.dev, "format not supported!\n");
    return -EINVAL;
    }
//
// Fill the CEA-861 audio infoframe (see spec for details)
//
    cea.db1_ct_cc = (params_channels(params) - 1)
    & CEA861_AUDIO_INFOFRAME_DB1CC;
    cea.db1_ct_cc |= CEA861_AUDIO_INFOFRAME_DB1CT_FROM_STREAM;
    cea.db2_sf_ss = CEA861_AUDIO_INFOFRAME_DB2SF_FROM_STREAM;
    cea.db2_sf_ss |= CEA861_AUDIO_INFOFRAME_DB2SS_FROM_STREAM;
    cea.db3 = 0; /* not used, all zeros */
    if (params_channels(params) == 2)
    cea.db4_ca = 0x0;
#[no_mangle]
pub unsafe extern "C" fn if(6: params_channels(params) ==) -> else {
    else if (params_channels(params) == 6)
    cea.db4_ca = 0xb;
    else
    cea.db4_ca = 0x13;
    if (cea.db4_ca == 0x00)
    cea.db5_dminh_lsv = CEA861_AUDIO_INFOFRAME_DB5_DM_INH_PERMITTED;
    else
    cea.db5_dminh_lsv = CEA861_AUDIO_INFOFRAME_DB5_DM_INH_PROHIBITED;
// the expression is trivial but makes clear what we are doing
    cea.db5_dminh_lsv |= (0 & CEA861_AUDIO_INFOFRAME_DB5_LSV);
    return ad.ops.audio_config(ad.dssdev, &ad.dss_audio);
    }
    static int hdmi_dai_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct hdmi_audio_data *ad = card_drvdata_substream(substream);
    let mut err: c_int = 0;
    WARN_ON(ad.current_stream != substream);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    err = ad.ops.audio_start(ad.dssdev);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    ad.ops.audio_stop(ad.dssdev);
    break;
    default:
    err = -EINVAL;
    }
    return err;
    }
    static void hdmi_dai_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct hdmi_audio_data *ad = card_drvdata_substream(substream);
    WARN_ON(ad.current_stream != substream);
    ad.ops.audio_shutdown(ad.dssdev);
    scoped_guard(mutex, &ad.current_stream_lock)
    ad.current_stream = core::ptr::null_mut();
    }
    static const struct snd_soc_dai_ops hdmi_dai_ops = {
    .startup	= hdmi_dai_startup,
    .hw_params	= hdmi_dai_hw_params,
    .trigger	= hdmi_dai_trigger,
    .shutdown	= hdmi_dai_shutdown,
    };
    static const struct snd_soc_component_driver omap_hdmi_component = {
    .name = "omapdss_hdmi",
    .legacy_dai_naming = 1,
    };
    static struct snd_soc_dai_driver omap5_hdmi_dai = {
    .name = "omap5-hdmi-dai",
    .playback = {
    .channels_min = 2,
    .channels_max = 8,
    .rates = (SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 |
    SNDRV_PCM_RATE_192000),
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .ops = &hdmi_dai_ops,
    };
    static struct snd_soc_dai_driver omap4_hdmi_dai = {
    .name = "omap4-hdmi-dai",
    .playback = {
    .channels_min = 2,
    .channels_max = 8,
    .rates = (SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 |
    SNDRV_PCM_RATE_192000),
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    .ops = &hdmi_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn omap_hdmi_audio_probe(pdev: *mut platform_device) -> c_int {
    static int omap_hdmi_audio_probe(struct platform_device *pdev)
    {
    struct omap_hdmi_audio_pdata *ha = pdev.dev.platform_data;
    struct device *dev = &pdev.dev;
    struct hdmi_audio_data *ad;
    struct snd_soc_dai_driver *dai_drv;
    struct snd_soc_card *card;
    struct snd_soc_dai_link_component *compnent;
    int ret;
    if (!ha) {
    dev_err(dev, "No platform data\n");
    return -EINVAL;
    }
    ad = devm_kzalloc(dev, sizeof(*ad), GFP_KERNEL);
    if (!ad)
    return -ENOMEM;
    ad.dssdev = ha.dev;
    ad.ops = ha.ops;
    ad.dma_data.addr = ha.audio_dma_addr;
    ad.dma_data.filter_data = "audio_tx";
    ad.dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    mutex_init(&ad.current_stream_lock);
    switch (ha.version) {
    case 4:
    dai_drv = &omap4_hdmi_dai;
    break;
    case 5:
    dai_drv = &omap5_hdmi_dai;
    break;
    default:
    return -EINVAL;
    }
    ret = devm_snd_soc_register_component(ad.dssdev, &omap_hdmi_component,
    dai_drv, 1);
    if (ret)
    return ret;
    ret = sdma_pcm_platform_register(ad.dssdev, "audio_tx", core::ptr::null_mut());
    if (ret)
    return ret;
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
    card.name = "HDMI";
    card.owner = THIS_MODULE;
    card.dai_link =
    devm_kzalloc(dev, sizeof(*(card.dai_link)), GFP_KERNEL);
    if (!card.dai_link)
    return -ENOMEM;
    compnent = devm_kzalloc(dev, 2 * sizeof(*compnent), GFP_KERNEL);
    if (!compnent)
    return -ENOMEM;
    card.dai_link.cpus		= &compnent[0];
    card.dai_link.num_cpus	= 1;
    card.dai_link.codecs		= &snd_soc_dummy_dlc;
    card.dai_link.num_codecs	= 1;
    card.dai_link.platforms	= &compnent[1];
    card.dai_link.num_platforms	= 1;
    card.dai_link.name = card.name;
    card.dai_link.stream_name = card.name;
    card.dai_link.cpus.dai_name = dev_name(ad.dssdev);
    card.dai_link.platforms.name = dev_name(ad.dssdev);
    card.num_links = 1;
    card.dev = dev;
    ret = devm_snd_soc_register_card(dev, card);
    if (ret)
    return dev_err_probe(dev, ret, "snd_soc_register_card() failed\n");
    snd_soc_card_set_drvdata(card, ad);
    dev_set_drvdata(dev, ad);
    return 0;
    }
    static struct platform_driver hdmi_audio_driver = {
    .driver = {
    .name = DRV_NAME,
    },
    .probe = omap_hdmi_audio_probe,
    };
    module_platform_driver(hdmi_audio_driver);
    MODULE_AUTHOR("Jyri Sarha <jsarha@ti.com>");
    MODULE_DESCRIPTION("OMAP HDMI Audio Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
