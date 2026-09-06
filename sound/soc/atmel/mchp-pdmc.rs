//! Automatically rewritten from C to Rust
//! Source: sound/soc/atmel/mchp-pdmc.c
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
// Driver for Microchip Pulse Density Microphone Controller (PDMC) interfaces
//
// Copyright (C) 2019-2022 Microchip Technology Inc. and its subsidiaries
//
// Author: Codrin Ciubotariu <codrin.ciubotariu@microchip.com>

//
// ---- PDMC Register map ----
//
pub const MCHP_PDMC_CR: c_uint = 0x00	/* Control Register */;
pub const MCHP_PDMC_MR: c_uint = 0x04	/* Mode Register */;
pub const MCHP_PDMC_CFGR: c_uint = 0x08	/* Configuration Register */;
pub const MCHP_PDMC_RHR: c_uint = 0x0C	/* Receive Holding Register */;
pub const MCHP_PDMC_IER: c_uint = 0x14	/* Interrupt Enable Register */;
pub const MCHP_PDMC_IDR: c_uint = 0x18	/* Interrupt Disable Register */;
pub const MCHP_PDMC_IMR: c_uint = 0x1C	/* Interrupt Mask Register */;
pub const MCHP_PDMC_ISR: c_uint = 0x20	/* Interrupt Status Register */;
pub const MCHP_PDMC_VER: c_uint = 0x50	/* Version Register */;
//
// ---- Control Register (Write-only) ----
//

//
// ---- Mode Register (Read/Write) ----
//

//
// ---- Configuration Register (Read/Write) ----
//

//
// ---- Interrupt Enable/Disable/Mask/Status Registers ----
//

//
// ---- Version Register (Read-only) ----
//

pub const MCHP_PDMC_MAX_CHANNELS: c_int = 4;
pub const MCHP_PDMC_DS_NO: c_int = 2;
pub const MCHP_PDMC_EDGE_NO: c_int = 2;
//
// ---- DMA chunk size allowed ----
//
pub const MCHP_PDMC_DMA_8_WORD_CHUNK: c_int = 8;
pub const MCHP_PDMC_DMA_4_WORD_CHUNK: c_int = 4;
pub const MCHP_PDMC_DMA_2_WORD_CHUNK: c_int = 2;
pub const MCHP_PDMC_DMA_1_WORD_CHUNK: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mic_map {
    pub ds_pos: c_int,
    pub clk_edge: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_pdmc_chmap {
    pub chmap: *mut snd_pcm_chmap_elem,
    pub dd: *mut mchp_pdmc,
    pub pcm: *mut snd_pcm,
    pub kctl: *mut snd_kcontrol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_pdmc {
    pub channel_mic_map: [mic_map; MCHP_PDMC_MAX_CHANNELS],
    pub dev: *mut device,
    pub addr: snd_dmaengine_dai_dma_data,
    pub regmap: *mut regmap,
    pub pclk: *mut clk,
    pub gclk: *mut clk,
    pub pdmcen: u32,
    pub suspend_irq: u32,
    pub startup_delay_us: u32,
    pub mic_no: c_int,
    pub sinc_order: c_int,
    pub audio_filter_en: bool,
    pub busy_stream: core::sync::atomic::AtomicI32,
}

    static const char *const mchp_pdmc_sinc_filter_order_text[] = {
    "1", "2", "3", "4", "5"
    };
    static const unsigned int mchp_pdmc_sinc_filter_order_values[] = {
    1, 2, 3, 4, 5,
    };
    static const struct soc_enum mchp_pdmc_sinc_filter_order_enum = {
    .items = ARRAY_SIZE(mchp_pdmc_sinc_filter_order_text),
    .texts = mchp_pdmc_sinc_filter_order_text,
    .values = mchp_pdmc_sinc_filter_order_values,
    };
    static int mchp_pdmc_sinc_order_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *uvalue)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct mchp_pdmc *dd = snd_soc_component_get_drvdata(component);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    unsigned int item;
    item = snd_soc_enum_val_to_item(e, dd.sinc_order);
    uvalue.value.enumerated.item[0] = item;
    return 0;
    }
    static int mchp_pdmc_sinc_order_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *uvalue)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct mchp_pdmc *dd = snd_soc_component_get_drvdata(component);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    unsigned int *item = uvalue.value.enumerated.item;
    unsigned int val;
    if (item[0] >= e.items)
    return -EINVAL;
    val = snd_soc_enum_item_to_val(e, item[0]) << e.shift_l;
    if (atomic_read(&dd.busy_stream))
    return -EBUSY;
    if (val == dd.sinc_order)
    return 0;
    dd.sinc_order = val;
    return 1;
    }
    static int mchp_pdmc_af_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *uvalue)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct mchp_pdmc *dd = snd_soc_component_get_drvdata(component);
    uvalue.value.integer.value[0] = !!dd.audio_filter_en;
    return 0;
    }
    static int mchp_pdmc_af_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *uvalue)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct mchp_pdmc *dd = snd_soc_component_get_drvdata(component);
    let mut af: bool = uvalue.value.integer.value[0] ? true : false;
    if (atomic_read(&dd.busy_stream))
    return -EBUSY;
    if (dd.audio_filter_en == af)
    return 0;
    dd.audio_filter_en = af;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_chmap_ctl_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static int mchp_pdmc_chmap_ctl_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
    {
    struct mchp_pdmc_chmap *info = snd_kcontrol_chip(kcontrol);
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = info.dd.mic_no;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = SNDRV_CHMAP_RR; /* maxmimum 4 channels */
    return 0;
    }
    static inline struct snd_pcm_substream *
    mchp_pdmc_chmap_substream(struct mchp_pdmc_chmap *info, unsigned int idx)
    {
    struct snd_pcm_substream *s;
    for (s = info.pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream; s; s = s.next)
    if (s.number == idx)
    return s;
    return core::ptr::null_mut();
    }
    static struct snd_pcm_chmap_elem *mchp_pdmc_chmap_get(struct snd_pcm_substream *substream,
    struct mchp_pdmc_chmap *ch_info)
    {
    struct snd_pcm_chmap_elem *map;
    for (map = ch_info.chmap; map.channels; map++) {
    if (map.channels == substream.runtime.channels)
    return map;
    }
    return core::ptr::null_mut();
    }
    static int mchp_pdmc_chmap_ctl_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct mchp_pdmc_chmap *info = snd_kcontrol_chip(kcontrol);
    struct mchp_pdmc *dd = info.dd;
    let mut idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &ucontrol.id);
    struct snd_pcm_substream *substream;
    const struct snd_pcm_chmap_elem *map;
    int i;
    let mut cfgr_val: u32 = 0;
    if (!info.chmap)
    return -EINVAL;
    substream = mchp_pdmc_chmap_substream(info, idx);
    if (!substream)
    return -ENODEV;
    memset(ucontrol.value.integer.value, 0, sizeof(long) * info.dd.mic_no);
    if (!substream.runtime)
    return 0; /* no channels set */
    map = mchp_pdmc_chmap_get(substream, info);
    if (!map)
    return -EINVAL;
    for (i = 0; i < map.channels; i++) {
    int map_idx = map.channels == 1 ? map.map[i] - SNDRV_CHMAP_MONO :
    map.map[i] - SNDRV_CHMAP_FL;
// make sure the reported channel map is the real one, so write the map
    if (dd.channel_mic_map[map_idx].ds_pos)
    cfgr_val |= MCHP_PDMC_CFGR_PDMSEL(i);
    if (dd.channel_mic_map[map_idx].clk_edge)
    cfgr_val |= MCHP_PDMC_CFGR_BSSEL(i);
    ucontrol.value.integer.value[i] = map.map[i];
    }
    regmap_write(dd.regmap, MCHP_PDMC_CFGR, cfgr_val);
    return 0;
    }
    static int mchp_pdmc_chmap_ctl_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct mchp_pdmc_chmap *info = snd_kcontrol_chip(kcontrol);
    struct mchp_pdmc *dd = info.dd;
    let mut idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &ucontrol.id);
    struct snd_pcm_substream *substream;
    struct snd_pcm_chmap_elem *map;
    let mut cfgr_val: u32 = 0;
    int i;
    if (!info.chmap)
    return -EINVAL;
    substream = mchp_pdmc_chmap_substream(info, idx);
    if (!substream)
    return -ENODEV;
    if (!substream.runtime)
    return 0; /* just for avoiding error from alsactl restore */
    map = mchp_pdmc_chmap_get(substream, info);
    if (!map)
    return -EINVAL;
    for (i = 0; i < map.channels; i++) {
    int map_idx;
    map.map[i] = ucontrol.value.integer.value[i];
    map_idx = map.channels == 1 ? map.map[i] - SNDRV_CHMAP_MONO :
    map.map[i] - SNDRV_CHMAP_FL;
// configure IP for the desired channel map
    if (dd.channel_mic_map[map_idx].ds_pos)
    cfgr_val |= MCHP_PDMC_CFGR_PDMSEL(i);
    if (dd.channel_mic_map[map_idx].clk_edge)
    cfgr_val |= MCHP_PDMC_CFGR_BSSEL(i);
    }
    regmap_write(dd.regmap, MCHP_PDMC_CFGR, cfgr_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_chmap_ctl_private_free(kcontrol: *mut snd_kcontrol) {
    static void mchp_pdmc_chmap_ctl_private_free(struct snd_kcontrol *kcontrol)
    {
    struct mchp_pdmc_chmap *info = snd_kcontrol_chip(kcontrol);
    info.pcm.streams[SNDRV_PCM_STREAM_CAPTURE].chmap_kctl = core::ptr::null_mut();
    kfree(info);
    }
    static int mchp_pdmc_chmap_ctl_tlv(struct snd_kcontrol *kcontrol, int op_flag,
    unsigned int size, unsigned int __user *tlv)
    {
    struct mchp_pdmc_chmap *info = snd_kcontrol_chip(kcontrol);
    const struct snd_pcm_chmap_elem *map;
    unsigned int __user *dst;
    int c, count = 0;
    if (!info.chmap)
    return -EINVAL;
    if (size < 8)
    return -ENOMEM;
    if (put_user(SNDRV_CTL_TLVT_CONTAINER, tlv))
    return -EFAULT;
    size -= 8;
    dst = tlv + 2;
    for (map = info.chmap; map.channels; map++) {
    let mut chs_bytes: c_int = map.channels * 4;
    if (size < 8)
    return -ENOMEM;
    if (put_user(SNDRV_CTL_TLVT_CHMAP_VAR, dst) ||
    put_user(chs_bytes, dst + 1))
    return -EFAULT;
    dst += 2;
    size -= 8;
    count += 8;
    if (size < chs_bytes)
    return -ENOMEM;
    size -= chs_bytes;
    count += chs_bytes;
    for (c = 0; c < map.channels; c++) {
    if (put_user(map.map[c], dst))
    return -EFAULT;
    dst++;
    }
    }
    if (put_user(count, tlv + 1))
    return -EFAULT;
    return 0;
    }
    static const struct snd_kcontrol_new mchp_pdmc_snd_controls[] = {
    SOC_SINGLE_BOOL_EXT("Audio Filter", 0, &mchp_pdmc_af_get, &mchp_pdmc_af_put),
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "SINC Filter Order",
    .info = snd_soc_info_enum_double,
    .get = mchp_pdmc_sinc_order_get,
    .put = mchp_pdmc_sinc_order_put,
    .private_value = (unsigned long)&mchp_pdmc_sinc_filter_order_enum,
    },
    };
    static const struct snd_soc_component_driver mchp_pdmc_dai_component = {
    .name = "mchp-pdmc",
    .controls = mchp_pdmc_snd_controls,
    .num_controls = ARRAY_SIZE(mchp_pdmc_snd_controls),
    };
    static const unsigned int mchp_pdmc_1mic[] = {1};
    static const unsigned int mchp_pdmc_2mic[] = {1, 2};
    static const unsigned int mchp_pdmc_3mic[] = {1, 2, 3};
    static const unsigned int mchp_pdmc_4mic[] = {1, 2, 3, 4};
    static const struct snd_pcm_hw_constraint_list mchp_pdmc_chan_constr[] = {
    {
    .list = mchp_pdmc_1mic,
    .count = ARRAY_SIZE(mchp_pdmc_1mic),
    },
    {
    .list = mchp_pdmc_2mic,
    .count = ARRAY_SIZE(mchp_pdmc_2mic),
    },
    {
    .list = mchp_pdmc_3mic,
    .count = ARRAY_SIZE(mchp_pdmc_3mic),
    },
    {
    .list = mchp_pdmc_4mic,
    .count = ARRAY_SIZE(mchp_pdmc_4mic),
    },
    };
    static int mchp_pdmc_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mchp_pdmc *dd = snd_soc_dai_get_drvdata(dai);
    regmap_write(dd.regmap, MCHP_PDMC_CR, MCHP_PDMC_CR_SWRST);
    snd_pcm_hw_constraint_list(substream.runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
    &mchp_pdmc_chan_constr[dd.mic_no - 1]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int mchp_pdmc_dai_probe(struct snd_soc_dai *dai)
    {
    struct mchp_pdmc *dd = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, core::ptr::null_mut(), &dd.addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int mchp_pdmc_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    let mut fmt_master: c_uint = fmt & SND_SOC_DAIFMT_MASTER_MASK;
    let mut fmt_format: c_uint = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
// IP needs to be bitclock master
    if (fmt_master != SND_SOC_DAIFMT_BP_FP &&
    fmt_master != SND_SOC_DAIFMT_BP_FC)
    return -EINVAL;
// IP supports only PDM interface
    if (fmt_format != SND_SOC_DAIFMT_PDM)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_mr_set_osr(audio_filter_en: c_int, osr: c_uint) -> u32 {
    static u32 mchp_pdmc_mr_set_osr(int audio_filter_en, unsigned int osr)
    {
    if (audio_filter_en) {
    switch (osr) {
    case 64:
    return MCHP_PDMC_MR_OSR64;
    case 128:
    return MCHP_PDMC_MR_OSR128;
    case 256:
    return MCHP_PDMC_MR_OSR256;
    }
    } else {
    switch (osr) {
    case 8:
    return MCHP_PDMC_MR_SINC_OSR_8;
    case 16:
    return MCHP_PDMC_MR_SINC_OSR_16;
    case 32:
    return MCHP_PDMC_MR_SINC_OSR_32;
    case 64:
    return MCHP_PDMC_MR_SINC_OSR_64;
    case 128:
    return MCHP_PDMC_MR_SINC_OSR_128;
    case 256:
    return MCHP_PDMC_MR_SINC_OSR_256;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mchp_pdmc_period_to_maxburst(period_size: c_int, sample_size: c_int) -> c_int {
    static inline int mchp_pdmc_period_to_maxburst(int period_size, int sample_size)
    {
    let mut p_size: c_int = period_size;
    let mut s_size: c_int = sample_size;
    if (DMA_BURST_ALIGNED(p_size, s_size, MCHP_PDMC_DMA_8_WORD_CHUNK))
    return MCHP_PDMC_DMA_8_WORD_CHUNK;
    if (DMA_BURST_ALIGNED(p_size, s_size, MCHP_PDMC_DMA_4_WORD_CHUNK))
    return MCHP_PDMC_DMA_4_WORD_CHUNK;
    if (DMA_BURST_ALIGNED(p_size, s_size, MCHP_PDMC_DMA_2_WORD_CHUNK))
    return MCHP_PDMC_DMA_2_WORD_CHUNK;
    return MCHP_PDMC_DMA_1_WORD_CHUNK;
    }
    static struct snd_pcm_chmap_elem mchp_pdmc_std_chmaps[] = {
    { .channels = 1,
    .map = { SNDRV_CHMAP_MONO } },
    { .channels = 2,
    .map = { SNDRV_CHMAP_FL, SNDRV_CHMAP_FR } },
    { .channels = 3,
    .map = { SNDRV_CHMAP_FL, SNDRV_CHMAP_FR,
    SNDRV_CHMAP_RL } },
    { .channels = 4,
    .map = { SNDRV_CHMAP_FL, SNDRV_CHMAP_FR,
    SNDRV_CHMAP_RL, SNDRV_CHMAP_RR } },
    { }
    };
    static int mchp_pdmc_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct mchp_pdmc *dd = snd_soc_dai_get_drvdata(dai);
    struct snd_soc_component *comp = dai.component;
    let mut gclk_rate: c_ulong = 0;
    let mut best_diff_rate: c_ulong = ~0UL;
    let mut channels: c_uint = params_channels(params);
    let mut osr: c_uint = 0, osr_start;
    let mut fs: c_uint = params_rate(params);
    let mut sample_bytes: c_int = params_physical_width(params) / 8;
    int period_bytes = params_period_size(params) *
    params_channels(params) * sample_bytes;
    int maxburst;
    let mut mr_val: u32 = 0;
    let mut cfgr_val: u32 = 0;
    int i;
    int ret;
    dev_dbg(comp.dev, "%s() rate=%u format=%#x width=%u channels=%u period_bytes=%d\n",
    __func__, params_rate(params), params_format(params),
    params_width(params), params_channels(params), period_bytes);
    if (channels > dd.mic_no) {
    dev_err(comp.dev, "more channels %u than microphones %d\n",
    channels, dd.mic_no);
    return -EINVAL;
    }
    dd.pdmcen = 0;
    for (i = 0; i < channels; i++) {
    dd.pdmcen |= MCHP_PDMC_MR_PDMCEN(i);
    if (dd.channel_mic_map[i].ds_pos)
    cfgr_val |= MCHP_PDMC_CFGR_PDMSEL(i);
    if (dd.channel_mic_map[i].clk_edge)
    cfgr_val |= MCHP_PDMC_CFGR_BSSEL(i);
    }
//
// from these point forward, we consider the controller busy, so the
// audio filter and SINC order can't be changed
//
    atomic_set(&dd.busy_stream, 1);
    for (osr_start = dd.audio_filter_en ? 64 : 8;
    osr_start <= 256 && best_diff_rate; osr_start *= 2) {
    long round_rate;
    unsigned long diff_rate;
    round_rate = clk_round_rate(dd.gclk,
    (unsigned long)fs * 16 * osr_start);
    if (round_rate < 0)
    continue;
    diff_rate = abs((fs * 16 * osr_start) - round_rate);
    if (diff_rate < best_diff_rate) {
    best_diff_rate = diff_rate;
    osr = osr_start;
    gclk_rate = fs * 16 * osr;
    }
    }
    if (!gclk_rate) {
    dev_err(comp.dev, "invalid sampling rate: %u\n", fs);
    return -EINVAL;
    }
// CLK is enabled by runtime PM.
    clk_disable_unprepare(dd.gclk);
// set the rate
    ret = clk_set_rate(dd.gclk, gclk_rate);
    clk_prepare_enable(dd.gclk);
    if (ret) {
    dev_err(comp.dev, "unable to set rate %lu to GCLK: %d\n",
    gclk_rate, ret);
    return ret;
    }
    mr_val |= mchp_pdmc_mr_set_osr(dd.audio_filter_en, osr);
    mr_val |= FIELD_PREP(MCHP_PDMC_MR_SINCORDER_MASK, dd.sinc_order);
    maxburst = mchp_pdmc_period_to_maxburst(period_bytes, sample_bytes);
    dd.addr.maxburst = maxburst;
    mr_val |= FIELD_PREP(MCHP_PDMC_MR_CHUNK_MASK, dd.addr.maxburst);
    dev_dbg(comp.dev, "maxburst set to %d\n", dd.addr.maxburst);
    snd_soc_component_update_bits(comp, MCHP_PDMC_MR,
    MCHP_PDMC_MR_OSR_MASK |
    MCHP_PDMC_MR_SINCORDER_MASK |
    MCHP_PDMC_MR_SINC_OSR_MASK |
    MCHP_PDMC_MR_CHUNK_MASK, mr_val);
    snd_soc_component_write(comp, MCHP_PDMC_CFGR, cfgr_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_noise_filter_workaround(dd: *mut mchp_pdmc) {
    static void mchp_pdmc_noise_filter_workaround(struct mchp_pdmc *dd)
    {
    u32 tmp, steps = 16;
//
// PDMC doesn't wait for microphones' startup time thus the acquisition
// may start before the microphones are ready leading to poc noises at
// the beginning of capture. To avoid this, we need to wait 50ms (in
// normal startup procedure) or 150 ms (worst case after resume from sleep
// states) after microphones are enabled and then clear the FIFOs (by
// reading the RHR 16 times) and possible interrupts before continuing.
// Also, for this to work the DMA needs to be started after interrupts
// are enabled.
//
    usleep_range(dd.startup_delay_us, dd.startup_delay_us + 5);
    while (steps--)
    regmap_read(dd.regmap, MCHP_PDMC_RHR, &tmp);
// Clear interrupts.
    regmap_read(dd.regmap, MCHP_PDMC_ISR, &tmp);
    }
    static int mchp_pdmc_trigger(struct snd_pcm_substream *substream,
    int cmd, struct snd_soc_dai *dai)
    {
    struct mchp_pdmc *dd = snd_soc_dai_get_drvdata(dai);
    struct snd_soc_component *cpu = dai.component;

    u32 val;

    switch (cmd) {
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    snd_soc_component_update_bits(cpu, MCHP_PDMC_MR,
    MCHP_PDMC_MR_PDMCEN_MASK,
    dd.pdmcen);
    mchp_pdmc_noise_filter_workaround(dd);
// Enable interrupts.
    regmap_write(dd.regmap, MCHP_PDMC_IER, dd.suspend_irq |
    MCHP_PDMC_IR_RXOVR | MCHP_PDMC_IR_RXUDR);
    dd.suspend_irq = 0;
    break;
    case SNDRV_PCM_TRIGGER_SUSPEND:
    regmap_read(dd.regmap, MCHP_PDMC_IMR, &dd.suspend_irq);
    fallthrough;
    case SNDRV_PCM_TRIGGER_STOP:
// Disable overrun and underrun error interrupts
    regmap_write(dd.regmap, MCHP_PDMC_IDR, dd.suspend_irq |
    MCHP_PDMC_IR_RXOVR | MCHP_PDMC_IR_RXUDR);
    fallthrough;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    snd_soc_component_update_bits(cpu, MCHP_PDMC_MR,
    MCHP_PDMC_MR_PDMCEN_MASK, 0);
    break;
    default:
    return -EINVAL;
    }

    regmap_read(dd.regmap, MCHP_PDMC_MR, &val);
    dev_dbg(dd.dev, "MR (0x%02x): 0x%08x\n", MCHP_PDMC_MR, val);
    regmap_read(dd.regmap, MCHP_PDMC_CFGR, &val);
    dev_dbg(dd.dev, "CFGR (0x%02x): 0x%08x\n", MCHP_PDMC_CFGR, val);
    regmap_read(dd.regmap, MCHP_PDMC_IMR, &val);
    dev_dbg(dd.dev, "IMR (0x%02x): 0x%08x\n", MCHP_PDMC_IMR, val);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_add_chmap_ctls(pcm: *mut snd_pcm, dd: *mut mchp_pdmc) -> c_int {
    static int mchp_pdmc_add_chmap_ctls(struct snd_pcm *pcm, struct mchp_pdmc *dd)
    {
    struct mchp_pdmc_chmap *info;
    struct snd_kcontrol_new knew = {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ |
    SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK,
    .info = mchp_pdmc_chmap_ctl_info,
    .get = mchp_pdmc_chmap_ctl_get,
    .put = mchp_pdmc_chmap_ctl_put,
    .tlv.c = mchp_pdmc_chmap_ctl_tlv,
    };
    int err;
    if (WARN_ON(pcm.streams[SNDRV_PCM_STREAM_CAPTURE].chmap_kctl))
    return -EBUSY;
    info = kzalloc_obj(*info);
    if (!info)
    return -ENOMEM;
    info.pcm = pcm;
    info.dd = dd;
    info.chmap = mchp_pdmc_std_chmaps;
    knew.name = "Capture Channel Map";
    knew.device = pcm.device;
    knew.count = pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream_count;
    info.kctl = snd_ctl_new1(&knew, info);
    if (!info.kctl) {
    kfree(info);
    return -ENOMEM;
    }
    info.kctl.private_free = mchp_pdmc_chmap_ctl_private_free;
    err = snd_ctl_add(pcm.card, info.kctl);
    if (err < 0)
    return err;
    pcm.streams[SNDRV_PCM_STREAM_CAPTURE].chmap_kctl = info.kctl;
    return 0;
    }
    static int mchp_pdmc_pcm_new(struct snd_soc_pcm_runtime *rtd,
    struct snd_soc_dai *dai)
    {
    struct mchp_pdmc *dd = snd_soc_dai_get_drvdata(dai);
    int ret;
    ret = mchp_pdmc_add_chmap_ctls(rtd.pcm, dd);
    if (ret < 0)
    dev_err(dd.dev, "failed to add channel map controls: %d\n", ret);
    return ret;
    }
    let mut mchp_selectable_formats: static u64 = SND_SOC_POSSIBLE_DAIFMT_PDM;
    static const struct snd_soc_dai_ops mchp_pdmc_dai_ops = {
    .probe		= mchp_pdmc_dai_probe,
    .set_fmt	= mchp_pdmc_set_fmt,
    .startup	= mchp_pdmc_startup,
    .hw_params	= mchp_pdmc_hw_params,
    .trigger	= mchp_pdmc_trigger,
    .pcm_new	= &mchp_pdmc_pcm_new,
    .auto_selectable_formats	= &mchp_selectable_formats,
    .num_auto_selectable_formats	= 1,
    };
    static struct snd_soc_dai_driver mchp_pdmc_dai = {
    .name	= "mchp-pdmc",
    .capture = {
    .stream_name	= "Capture",
    .channels_min	= 1,
    .channels_max	= 4,
    .rate_min	= 8000,
    .rate_max	= 192000,
    .rates		= SNDRV_PCM_RATE_KNOT,
    .formats	= SNDRV_PCM_FMTBIT_S24_LE,
    },
    .ops = &mchp_pdmc_dai_ops,
    };
// PDMC interrupt handler
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mchp_pdmc_interrupt(int irq, void *dev_id)
    {
    struct mchp_pdmc *dd = dev_id;
    u32 isr, msr, pending;
    let mut ret: irqreturn_t = IRQ_NONE;
    regmap_read(dd.regmap, MCHP_PDMC_ISR, &isr);
    regmap_read(dd.regmap, MCHP_PDMC_IMR, &msr);
    pending = isr & msr;
    dev_dbg(dd.dev, "ISR (0x%02x): 0x%08x, IMR (0x%02x): 0x%08x, pending: 0x%08x\n",
    MCHP_PDMC_ISR, isr, MCHP_PDMC_IMR, msr, pending);
    if (!pending)
    return IRQ_NONE;
    if (pending & MCHP_PDMC_IR_RXUDR) {
    dev_warn(dd.dev, "underrun detected\n");
    regmap_write(dd.regmap, MCHP_PDMC_IDR, MCHP_PDMC_IR_RXUDR);
    ret = IRQ_HANDLED;
    }
    if (pending & MCHP_PDMC_IR_RXOVR) {
    dev_warn(dd.dev, "overrun detected\n");
    regmap_write(dd.regmap, MCHP_PDMC_IDR, MCHP_PDMC_IR_RXOVR);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
// regmap configuration
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mchp_pdmc_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MCHP_PDMC_MR:
    case MCHP_PDMC_CFGR:
    case MCHP_PDMC_IMR:
    case MCHP_PDMC_ISR:
    case MCHP_PDMC_RHR:
    case MCHP_PDMC_VER:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mchp_pdmc_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MCHP_PDMC_CR:
    case MCHP_PDMC_MR:
    case MCHP_PDMC_CFGR:
    case MCHP_PDMC_IER:
    case MCHP_PDMC_IDR:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mchp_pdmc_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MCHP_PDMC_ISR:
    case MCHP_PDMC_RHR:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mchp_pdmc_precious_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MCHP_PDMC_RHR:
    case MCHP_PDMC_ISR:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config mchp_pdmc_regmap_config = {
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    .max_register	= MCHP_PDMC_VER,
    .readable_reg	= mchp_pdmc_readable_reg,
    .writeable_reg	= mchp_pdmc_writeable_reg,
    .precious_reg	= mchp_pdmc_precious_reg,
    .volatile_reg	= mchp_pdmc_volatile_reg,
    .cache_type	= REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_dt_init(dd: *mut mchp_pdmc) -> c_int {
    static int mchp_pdmc_dt_init(struct mchp_pdmc *dd)
    {
    struct device_node *np = dd.dev.of_node;
    bool mic_ch[MCHP_PDMC_DS_NO][MCHP_PDMC_EDGE_NO] = {0};
    int i;
    int ret;
    if (!np) {
    dev_err(dd.dev, "device node not found\n");
    return -EINVAL;
    }
    dd.mic_no = of_property_count_u32_elems(np, "microchip,mic-pos");
    if (dd.mic_no < 0) {
    dev_err(dd.dev, "failed to get microchip,mic-pos: %d",
    dd.mic_no);
    return dd.mic_no;
    }
    if (!dd.mic_no || dd.mic_no % 2 ||
    dd.mic_no / 2 > MCHP_PDMC_MAX_CHANNELS) {
    dev_err(dd.dev, "invalid array length for microchip,mic-pos: %d",
    dd.mic_no);
    return -EINVAL;
    }
    dd.mic_no /= 2;
    dev_info(dd.dev, "%d PDM microphones declared\n", dd.mic_no);
//
// by default, we consider the order of microphones in
// microchip,mic-pos to be the same with the channel mapping;
// 1st microphone channel 0, 2nd microphone channel 1, etc.
//
    for (i = 0; i < dd.mic_no; i++) {
    int ds;
    int edge;
    ret = of_property_read_u32_index(np, "microchip,mic-pos", i * 2,
    &ds);
    if (ret) {
    dev_err(dd.dev,
    "failed to get value no %d value from microchip,mic-pos: %d",
    i * 2, ret);
    return ret;
    }
    if (ds >= MCHP_PDMC_DS_NO) {
    dev_err(dd.dev,
    "invalid DS index in microchip,mic-pos array: %d",
    ds);
    return -EINVAL;
    }
    ret = of_property_read_u32_index(np, "microchip,mic-pos", i * 2 + 1,
    &edge);
    if (ret) {
    dev_err(dd.dev,
    "failed to get value no %d value from microchip,mic-pos: %d",
    i * 2 + 1, ret);
    return ret;
    }
    if (edge != MCHP_PDMC_CLK_POSITIVE &&
    edge != MCHP_PDMC_CLK_NEGATIVE) {
    dev_err(dd.dev,
    "invalid edge in microchip,mic-pos array: %d", edge);
    return -EINVAL;
    }
    if (mic_ch[ds][edge]) {
    dev_err(dd.dev,
    "duplicated mic (DS %d, edge %d) in microchip,mic-pos array",
    ds, edge);
    return -EINVAL;
    }
    mic_ch[ds][edge] = true;
    dd.channel_mic_map[i].ds_pos = ds;
    dd.channel_mic_map[i].clk_edge = edge;
    }
    dd.startup_delay_us = 150000;
    of_property_read_u32(np, "microchip,startup-delay-us", &dd.startup_delay_us);
    return 0;
    }
// used to clean the channel index found on RHR's MSB
    static int mchp_pdmc_process(struct snd_pcm_substream *substream,
    int channel, unsigned long hwoff,
    unsigned long bytes)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    u8 *dma_ptr = runtime.dma_area + hwoff +
    channel * (runtime.dma_bytes / runtime.channels);
    u8 *dma_ptr_end = dma_ptr + bytes;
    let mut sample_size: c_uint = samples_to_bytes(runtime, 1);
    for (; dma_ptr < dma_ptr_end; dma_ptr += sample_size)
// dma_ptr = 0;
    return 0;
    }
    static struct snd_dmaengine_pcm_config mchp_pdmc_config = {
    .process = mchp_pdmc_process,
    .prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config,
    };
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_runtime_suspend(dev: *mut device) -> c_int {
    static int mchp_pdmc_runtime_suspend(struct device *dev)
    {
    struct mchp_pdmc *dd = dev_get_drvdata(dev);
    regcache_cache_only(dd.regmap, true);
    clk_disable_unprepare(dd.gclk);
    clk_disable_unprepare(dd.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_runtime_resume(dev: *mut device) -> c_int {
    static int mchp_pdmc_runtime_resume(struct device *dev)
    {
    struct mchp_pdmc *dd = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(dd.pclk);
    if (ret) {
    dev_err(dd.dev,
    "failed to enable the peripheral clock: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(dd.gclk);
    if (ret) {
    dev_err(dd.dev,
    "failed to enable generic clock: %d\n", ret);
    goto disable_pclk;
    }
    regcache_cache_only(dd.regmap, false);
    regcache_mark_dirty(dd.regmap);
    ret = regcache_sync(dd.regmap);
    if (ret) {
    regcache_cache_only(dd.regmap, true);
    clk_disable_unprepare(dd.gclk);
    disable_pclk:
    clk_disable_unprepare(dd.pclk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_probe(pdev: *mut platform_device) -> c_int {
    static int mchp_pdmc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mchp_pdmc *dd;
    struct resource *res;
    void __iomem *io_base;
    u32 version;
    int irq;
    int ret;
    dd = devm_kzalloc(dev, sizeof(*dd), GFP_KERNEL);
    if (!dd)
    return -ENOMEM;
    dd.dev = &pdev.dev;
    ret = mchp_pdmc_dt_init(dd);
    if (ret < 0)
    return ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    dd.pclk = devm_clk_get(dev, "pclk");
    if (IS_ERR(dd.pclk)) {
    ret = PTR_ERR(dd.pclk);
    dev_err(dev, "failed to get peripheral clock: %d\n", ret);
    return ret;
    }
    dd.gclk = devm_clk_get(dev, "gclk");
    if (IS_ERR(dd.gclk)) {
    ret = PTR_ERR(dd.gclk);
    dev_err(dev, "failed to get GCK: %d\n", ret);
    return ret;
    }
    io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(io_base)) {
    ret = PTR_ERR(io_base);
    dev_err(dev, "failed to remap register memory: %d\n", ret);
    return ret;
    }
    dd.regmap = devm_regmap_init_mmio(dev, io_base,
    &mchp_pdmc_regmap_config);
    if (IS_ERR(dd.regmap)) {
    ret = PTR_ERR(dd.regmap);
    dev_err(dev, "failed to init register map: %d\n", ret);
    return ret;
    }
    ret = devm_request_irq(dev, irq, mchp_pdmc_interrupt, 0,
    dev_name(&pdev.dev), dd);
    if (ret < 0) {
    dev_err(dev, "can't register ISR for IRQ %u (ret=%i)\n",
    irq, ret);
    return ret;
    }
// by default audio filter is enabled and the SINC Filter order
// will be set to the recommended value, 3
//
    dd.audio_filter_en = true;
    dd.sinc_order = 3;
    dd.addr.addr = (dma_addr_t)res.start + MCHP_PDMC_RHR;
    platform_set_drvdata(pdev, dd);
    pm_runtime_enable(dd.dev);
    if (!pm_runtime_enabled(dd.dev)) {
    ret = mchp_pdmc_runtime_resume(dd.dev);
    if (ret)
    return ret;
    }
// register platform
    ret = devm_snd_dmaengine_pcm_register(dev, &mchp_pdmc_config, 0);
    if (ret) {
    dev_err(dev, "could not register platform: %d\n", ret);
    goto pm_runtime_suspend;
    }
    ret = devm_snd_soc_register_component(dev, &mchp_pdmc_dai_component,
    &mchp_pdmc_dai, 1);
    if (ret) {
    dev_err(dev, "could not register CPU DAI: %d\n", ret);
    goto pm_runtime_suspend;
    }
// print IP version
    regmap_read(dd.regmap, MCHP_PDMC_VER, &version);
    dev_info(dd.dev, "hw version: %#lx\n",
    version & MCHP_PDMC_VER_VERSION);
    return 0;
    pm_runtime_suspend:
    if (!pm_runtime_status_suspended(dd.dev))
    mchp_pdmc_runtime_suspend(dd.dev);
    pm_runtime_disable(dd.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pdmc_remove(pdev: *mut platform_device) {
    static void mchp_pdmc_remove(struct platform_device *pdev)
    {
    struct mchp_pdmc *dd = platform_get_drvdata(pdev);
    atomic_set(&dd.busy_stream, 0);
    if (!pm_runtime_status_suspended(dd.dev))
    mchp_pdmc_runtime_suspend(dd.dev);
    pm_runtime_disable(dd.dev);
    }
    static const struct of_device_id mchp_pdmc_of_match[] = {
    {
    .compatible = "microchip,sama7g5-pdmc",
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, mchp_pdmc_of_match);
    static const struct dev_pm_ops mchp_pdmc_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    RUNTIME_PM_OPS(mchp_pdmc_runtime_suspend, mchp_pdmc_runtime_resume,
    core::ptr::null_mut())
    };
    static struct platform_driver mchp_pdmc_driver = {
    .driver	= {
    .name		= "mchp-pdmc",
    .of_match_table	= of_match_ptr(mchp_pdmc_of_match),
    .pm		= pm_ptr(&mchp_pdmc_pm_ops),
    },
    .probe	= mchp_pdmc_probe,
    .remove = mchp_pdmc_remove,
    };
    module_platform_driver(mchp_pdmc_driver);
    MODULE_DESCRIPTION("Microchip PDMC driver under ALSA SoC architecture");
    MODULE_AUTHOR("Codrin Ciubotariu <codrin.ciubotariu@microchip.com>");
    MODULE_LICENSE("GPL v2");
