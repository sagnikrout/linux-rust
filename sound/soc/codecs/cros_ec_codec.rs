//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cros_ec_codec.c
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
// Copyright 2019 Google, Inc.
//
// ChromeOS Embedded Controller codec driver.
//
// This driver uses the cros-ec interface to communicate with the ChromeOS
// EC for audio function.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_codec_priv {
    pub dev: *mut device,
    pub ec_device: *mut cros_ec_device,
// common
    pub ec_capabilities: u32,
    pub ec_shm_addr: u64,
    pub ec_shm_len: u32,
    pub ap_shm_phys_addr: u64,
    pub ap_shm_len: u32,
    pub ap_shm_addr: u64,
    pub ap_shm_last_alloc: u64,
// DMIC
    pub dmic_probed: core::sync::atomic::AtomicI32,
// I2S_RX
    pub i2s_rx_bclk_ratio: u32,
// WoV
    pub wov_enabled: bool,
    pub wov_audio_shm_p: *mut u8,
    pub wov_audio_shm_len: u32,
    pub wov_audio_shm_type: u8,
    pub wov_lang_shm_p: *mut u8,
    pub wov_lang_shm_len: u32,
    pub wov_lang_shm_type: u8,
    pub wov_dma_lock: mutex,
    pub wov_buf: [u8; 64000],
    pub wov_wp: uint32_t wov_rp,,
    pub wov_dma_offset: usize,
    pub wov_burst_read: bool,
    pub wov_substream: *mut snd_pcm_substream,
    pub wov_copy_work: delayed_work,
    pub wov_notifier: notifier_block,
}

#[no_mangle]
unsafe extern "C" fn ec_codec_capable(priv: *mut cros_ec_codec_priv, cap: u8) -> c_int {
    static int ec_codec_capable(struct cros_ec_codec_priv *priv, uint8_t cap)
    {
    return priv.ec_capabilities & BIT(cap);
    }
    static int send_ec_host_command(struct cros_ec_device *ec_dev, uint32_t cmd,
    uint8_t *out, size_t outsize,
    uint8_t *in, size_t insize)
    {
    int ret;
    struct cros_ec_command *msg;
    msg = kmalloc(sizeof(*msg) + max(outsize, insize), GFP_KERNEL);
    if (!msg)
    return -ENOMEM;
    msg.version = 0;
    msg.command = cmd;
    msg.outsize = outsize;
    msg.insize = insize;
    if (outsize)
    memcpy(msg.data, out, outsize);
    ret = cros_ec_cmd_xfer_status(ec_dev, msg);
    if (ret < 0)
    goto error;
    if (in && insize)
    memcpy(in, msg.data, insize);
    ret = 0;
    error:
    kfree(msg);
    return ret;
    }
    static int dmic_get_gain(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    struct ec_param_ec_codec_dmic p;
    struct ec_response_ec_codec_dmic_get_gain_idx r;
    int ret;
    p.cmd = EC_CODEC_DMIC_GET_GAIN_IDX;
    p.get_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_0;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_DMIC,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret < 0)
    return ret;
    ucontrol.value.integer.value[0] = r.gain;
    p.cmd = EC_CODEC_DMIC_GET_GAIN_IDX;
    p.get_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_1;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_DMIC,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret < 0)
    return ret;
    ucontrol.value.integer.value[1] = r.gain;
    return 0;
    }
    static int dmic_put_gain(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    struct soc_mixer_control *control =
    (struct soc_mixer_control *)kcontrol.private_value;
    let mut max_dmic_gain: c_int = control.max;
    let mut left: c_int = ucontrol.value.integer.value[0];
    let mut right: c_int = ucontrol.value.integer.value[1];
    struct ec_param_ec_codec_dmic p;
    int ret;
    if (left > max_dmic_gain || right > max_dmic_gain)
    return -EINVAL;
    dev_dbg(component.dev, "set mic gain to %u, %u\n", left, right);
    p.cmd = EC_CODEC_DMIC_SET_GAIN_IDX;
    p.set_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_0;
    p.set_gain_idx_param.gain = left;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_DMIC,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
    p.cmd = EC_CODEC_DMIC_SET_GAIN_IDX;
    p.set_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_1;
    p.set_gain_idx_param.gain = right;
    return send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_DMIC,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    }
    static const DECLARE_TLV_DB_SCALE(dmic_gain_tlv, 0, 100, 0);
    enum {
    DMIC_CTL_GAIN = 0,
    };
    static struct snd_kcontrol_new dmic_controls[] = {
    [DMIC_CTL_GAIN] =
    SOC_DOUBLE_EXT_TLV("EC Mic Gain", SND_SOC_NOPM, SND_SOC_NOPM,
    0, 0, 0, dmic_get_gain, dmic_put_gain,
    dmic_gain_tlv),
    };
#[no_mangle]
unsafe extern "C" fn dmic_probe(component: *mut snd_soc_component) -> c_int {
    static int dmic_probe(struct snd_soc_component *component)
    {
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    struct device *dev = priv.dev;
    struct soc_mixer_control *control;
    struct ec_param_ec_codec_dmic p;
    struct ec_response_ec_codec_dmic_get_max_gain r;
    int ret;
    if (!atomic_add_unless(&priv.dmic_probed, 1, 1))
    return 0;
    p.cmd = EC_CODEC_DMIC_GET_MAX_GAIN;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_DMIC,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret < 0) {
    dev_warn(dev, "get_max_gain() unsupported\n");
    return 0;
    }
    dev_dbg(dev, "max gain = %d\n", r.max_gain);
    control = (struct soc_mixer_control *)
    dmic_controls[DMIC_CTL_GAIN].private_value;
    control.max = r.max_gain;
    control.platform_max = r.max_gain;
    return snd_soc_add_component_controls(component,
    &dmic_controls[DMIC_CTL_GAIN], 1);
    }
    static int i2s_rx_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    struct ec_param_ec_codec_i2s_rx p;
    enum ec_codec_i2s_rx_sample_depth depth;
    uint32_t bclk;
    int ret;
    if (params_rate(params) != 48000)
    return -EINVAL;
    switch (params_width(params)) {
    case 16:
    depth = EC_CODEC_I2S_RX_SAMPLE_DEPTH_16;
    break;
    case 24:
    depth = EC_CODEC_I2S_RX_SAMPLE_DEPTH_24;
    break;
    default:
    return -EINVAL;
    }
    dev_dbg(component.dev, "set depth to %u\n", depth);
    p.cmd = EC_CODEC_I2S_RX_SET_SAMPLE_DEPTH;
    p.set_sample_depth_param.depth = depth;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_I2S_RX,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
    if (priv.i2s_rx_bclk_ratio)
    bclk = params_rate(params) * priv.i2s_rx_bclk_ratio;
    else
    bclk = snd_soc_params_to_bclk(params);
    dev_dbg(component.dev, "set bclk to %u\n", bclk);
    p.cmd = EC_CODEC_I2S_RX_SET_BCLK;
    p.set_bclk_param.bclk = bclk;
    return send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_I2S_RX,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn i2s_rx_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    static int i2s_rx_set_bclk_ratio(struct snd_soc_dai *dai, unsigned int ratio)
    {
    struct snd_soc_component *component = dai.component;
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    priv.i2s_rx_bclk_ratio = ratio;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2s_rx_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int i2s_rx_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct snd_soc_component *component = dai.component;
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    struct ec_param_ec_codec_i2s_rx p;
    enum ec_codec_i2s_rx_daifmt daifmt;
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBC_CFC:
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    daifmt = EC_CODEC_I2S_RX_DAIFMT_I2S;
    break;
    case SND_SOC_DAIFMT_RIGHT_J:
    daifmt = EC_CODEC_I2S_RX_DAIFMT_RIGHT_J;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    daifmt = EC_CODEC_I2S_RX_DAIFMT_LEFT_J;
    break;
    default:
    return -EINVAL;
    }
    dev_dbg(component.dev, "set format to %u\n", daifmt);
    p.cmd = EC_CODEC_I2S_RX_SET_DAIFMT;
    p.set_daifmt_param.daifmt = daifmt;
    return send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_I2S_RX,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    }
    static const u64 i2s_rx_selectable_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
    SND_SOC_POSSIBLE_DAIFMT_NB_NF;
    static const struct snd_soc_dai_ops i2s_rx_dai_ops = {
    .hw_params = i2s_rx_hw_params,
    .set_fmt = i2s_rx_set_fmt,
    .set_bclk_ratio = i2s_rx_set_bclk_ratio,
    .auto_selectable_formats = &i2s_rx_selectable_formats,
    .num_auto_selectable_formats = 1,
    };
    static int i2s_rx_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component =
    snd_soc_dapm_to_component(w.dapm);
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    let mut p: ec_param_ec_codec_i2s_rx = {};
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    dev_dbg(component.dev, "enable I2S RX\n");
    p.cmd = EC_CODEC_I2S_RX_ENABLE;
    break;
    case SND_SOC_DAPM_PRE_PMD:
    dev_dbg(component.dev, "disable I2S RX\n");
    p.cmd = EC_CODEC_I2S_RX_DISABLE;
    break;
    default:
    return 0;
    }
    return send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_I2S_RX,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    }
    static struct snd_soc_dapm_widget i2s_rx_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("DMIC"),
    SND_SOC_DAPM_SUPPLY("I2S RX Enable", SND_SOC_NOPM, 0, 0, i2s_rx_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_AIF_OUT("I2S RX", "I2S Capture", 0, SND_SOC_NOPM, 0, 0),
    };
    static struct snd_soc_dapm_route i2s_rx_dapm_routes[] = {
    {"I2S RX", core::ptr::null_mut(), "DMIC"},
    {"I2S RX", core::ptr::null_mut(), "I2S RX Enable"},
    };
    static struct snd_soc_dai_driver i2s_rx_dai_driver = {
    .name = "EC Codec I2S RX",
    .capture = {
    .stream_name = "I2S Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE,
    },
    .ops = &i2s_rx_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn i2s_rx_probe(component: *mut snd_soc_component) -> c_int {
    static int i2s_rx_probe(struct snd_soc_component *component)
    {
    return dmic_probe(component);
    }
    static const struct snd_soc_component_driver i2s_rx_component_driver = {
    .probe			= i2s_rx_probe,
    .dapm_widgets		= i2s_rx_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(i2s_rx_dapm_widgets),
    .dapm_routes		= i2s_rx_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(i2s_rx_dapm_routes),
    .endianness		= 1,
    };
    static void *wov_map_shm(struct cros_ec_codec_priv *priv,
    uint8_t shm_id, uint32_t *len, uint8_t *type)
    {
    struct ec_param_ec_codec p;
    struct ec_response_ec_codec_get_shm_addr r;
    uint32_t req, offset;
    p.cmd = EC_CODEC_GET_SHM_ADDR;
    p.get_shm_addr_param.shm_id = shm_id;
    if (send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r)) < 0) {
    dev_err(priv.dev, "failed to EC_CODEC_GET_SHM_ADDR\n");
    return core::ptr::null_mut();
    }
    dev_dbg(priv.dev, "phys_addr=%#llx, len=%#x\n", r.phys_addr, r.len);
// len = r.len;
// type = r.type;
    switch (r.type) {
    case EC_CODEC_SHM_TYPE_EC_RAM:
    return (void  *)devm_ioremap_wc(priv.dev,
    r.phys_addr + priv.ec_shm_addr, r.len);
    case EC_CODEC_SHM_TYPE_SYSTEM_RAM:
    if (r.phys_addr) {
    dev_err(priv.dev, "unknown status\n");
    return core::ptr::null_mut();
    }
    req = round_up(r.len, PAGE_SIZE);
    dev_dbg(priv.dev, "round up from %u to %u\n", r.len, req);
    if (priv.ap_shm_last_alloc + req >
    priv.ap_shm_phys_addr + priv.ap_shm_len) {
    dev_err(priv.dev, "insufficient space for AP SHM\n");
    return core::ptr::null_mut();
    }
    dev_dbg(priv.dev, "alloc AP SHM addr=%#llx, len=%#x\n",
    priv.ap_shm_last_alloc, req);
    p.cmd = EC_CODEC_SET_SHM_ADDR;
    p.set_shm_addr_param.phys_addr = priv.ap_shm_last_alloc;
    p.set_shm_addr_param.len = req;
    p.set_shm_addr_param.shm_id = shm_id;
    if (send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC,
    (uint8_t *)&p, sizeof(p),
    core::ptr::null_mut(), 0) < 0) {
    dev_err(priv.dev, "failed to EC_CODEC_SET_SHM_ADDR\n");
    return core::ptr::null_mut();
    }
//
// Note: EC codec only requests for `r.len' but we allocate
// round up PAGE_SIZE `req'.
//
    offset = priv.ap_shm_last_alloc - priv.ap_shm_phys_addr;
    priv.ap_shm_last_alloc += req;
    return (void *)(uintptr_t)(priv.ap_shm_addr + offset);
    default:
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn wov_queue_full(priv: *mut cros_ec_codec_priv) -> bool {
    static bool wov_queue_full(struct cros_ec_codec_priv *priv)
    {
    return ((priv.wov_wp + 1) % sizeof(priv.wov_buf)) == priv.wov_rp;
    }
#[no_mangle]
unsafe extern "C" fn wov_queue_size(priv: *mut cros_ec_codec_priv) -> usize {
    static size_t wov_queue_size(struct cros_ec_codec_priv *priv)
    {
    if (priv.wov_wp >= priv.wov_rp)
    return priv.wov_wp - priv.wov_rp;
    else
    return sizeof(priv.wov_buf) - priv.wov_rp + priv.wov_wp;
    }
#[no_mangle]
unsafe extern "C" fn wov_queue_dequeue(priv: *mut cros_ec_codec_priv, len: usize) {
    static void wov_queue_dequeue(struct cros_ec_codec_priv *priv, size_t len)
    {
    struct snd_pcm_runtime *runtime = priv.wov_substream.runtime;
    size_t req;
    while (len) {
    req = min(len, runtime.dma_bytes - priv.wov_dma_offset);
    if (priv.wov_wp >= priv.wov_rp)
    req = min(req, (size_t)priv.wov_wp - priv.wov_rp);
    else
    req = min(req, sizeof(priv.wov_buf) - priv.wov_rp);
    memcpy(runtime.dma_area + priv.wov_dma_offset,
    priv.wov_buf + priv.wov_rp, req);
    priv.wov_dma_offset += req;
    if (priv.wov_dma_offset == runtime.dma_bytes)
    priv.wov_dma_offset = 0;
    priv.wov_rp += req;
    if (priv.wov_rp == sizeof(priv.wov_buf))
    priv.wov_rp = 0;
    len -= req;
    }
    snd_pcm_period_elapsed(priv.wov_substream);
    }
#[no_mangle]
unsafe extern "C" fn wov_queue_try_dequeue(priv: *mut cros_ec_codec_priv) {
    static void wov_queue_try_dequeue(struct cros_ec_codec_priv *priv)
    {
    let mut period_bytes: usize = snd_pcm_lib_period_bytes(priv.wov_substream);
    while (period_bytes && wov_queue_size(priv) >= period_bytes) {
    wov_queue_dequeue(priv, period_bytes);
    period_bytes = snd_pcm_lib_period_bytes(priv.wov_substream);
    }
    }
    static void wov_queue_enqueue(struct cros_ec_codec_priv *priv,
    uint8_t *addr, size_t len, bool iomem)
    {
    size_t req;
    while (len) {
    if (wov_queue_full(priv)) {
    wov_queue_try_dequeue(priv);
    if (wov_queue_full(priv)) {
    dev_err(priv.dev, "overrun detected\n");
    return;
    }
    }
    if (priv.wov_wp >= priv.wov_rp)
    req = sizeof(priv.wov_buf) - priv.wov_wp;
    else
// Note: waste 1-byte to differentiate full and empty
    req = priv.wov_rp - priv.wov_wp - 1;
    req = min(req, len);
    if (iomem)
    memcpy_fromio(priv.wov_buf + priv.wov_wp,
    (void  __iomem *)addr, req);
    else
    memcpy(priv.wov_buf + priv.wov_wp, addr, req);
    priv.wov_wp += req;
    if (priv.wov_wp == sizeof(priv.wov_buf))
    priv.wov_wp = 0;
    addr += req;
    len -= req;
    }
    wov_queue_try_dequeue(priv);
    }
#[no_mangle]
unsafe extern "C" fn wov_read_audio_shm(priv: *mut cros_ec_codec_priv) -> c_int {
    static int wov_read_audio_shm(struct cros_ec_codec_priv *priv)
    {
    struct ec_param_ec_codec_wov p;
    struct ec_response_ec_codec_wov_read_audio_shm r;
    int ret;
    p.cmd = EC_CODEC_WOV_READ_AUDIO_SHM;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_WOV,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret) {
    dev_err(priv.dev, "failed to EC_CODEC_WOV_READ_AUDIO_SHM\n");
    return ret;
    }
    if (!r.len)
    dev_dbg(priv.dev, "no data, sleep\n");
    else
    wov_queue_enqueue(priv, priv.wov_audio_shm_p + r.offset, r.len,
    priv.wov_audio_shm_type == EC_CODEC_SHM_TYPE_EC_RAM);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn wov_read_audio(priv: *mut cros_ec_codec_priv) -> c_int {
    static int wov_read_audio(struct cros_ec_codec_priv *priv)
    {
    struct ec_param_ec_codec_wov p;
    struct ec_response_ec_codec_wov_read_audio r;
    let mut remain: c_int = priv.wov_burst_read ? 16000 : 320;
    int ret;
    while (remain >= 0) {
    p.cmd = EC_CODEC_WOV_READ_AUDIO;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_WOV,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret) {
    dev_err(priv.dev,
    "failed to EC_CODEC_WOV_READ_AUDIO\n");
    return ret;
    }
    if (!r.len) {
    dev_dbg(priv.dev, "no data, sleep\n");
    priv.wov_burst_read = false;
    break;
    }
    wov_queue_enqueue(priv, r.buf, r.len, false);
    remain -= r.len;
    }
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn wov_copy_work(w: *mut work_struct) {
    static void wov_copy_work(struct work_struct *w)
    {
    struct cros_ec_codec_priv *priv =
    container_of(w, struct cros_ec_codec_priv, wov_copy_work.work);
    int ret;
    guard(mutex)(&priv.wov_dma_lock);
    if (!priv.wov_substream) {
    dev_warn(priv.dev, "no pcm substream\n");
    return;
    }
    if (ec_codec_capable(priv, EC_CODEC_CAP_WOV_AUDIO_SHM))
    ret = wov_read_audio_shm(priv);
    else
    ret = wov_read_audio(priv);
    if (ret == -EAGAIN)
    schedule_delayed_work(&priv.wov_copy_work,
    msecs_to_jiffies(10));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    dev_err(priv.dev, "failed to read audio data\n");
    }
    static int wov_enable_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *c = snd_kcontrol_chip(kcontrol);
    struct cros_ec_codec_priv *priv = snd_soc_component_get_drvdata(c);
    ucontrol.value.integer.value[0] = priv.wov_enabled;
    return 0;
    }
    static int wov_enable_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *c = snd_kcontrol_chip(kcontrol);
    struct cros_ec_codec_priv *priv = snd_soc_component_get_drvdata(c);
    let mut enabled: c_int = ucontrol.value.integer.value[0];
    struct ec_param_ec_codec_wov p;
    int ret;
    if (priv.wov_enabled != enabled) {
    if (enabled)
    p.cmd = EC_CODEC_WOV_ENABLE;
    else
    p.cmd = EC_CODEC_WOV_DISABLE;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_WOV,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(priv.dev, "failed to %s wov\n",
    str_enable_disable(enabled));
    return ret;
    }
    priv.wov_enabled = enabled;
    }
    return 0;
    }
    static int wov_set_lang_shm(struct cros_ec_codec_priv *priv,
    uint8_t *buf, size_t size, uint8_t *digest)
    {
    struct ec_param_ec_codec_wov p;
    struct ec_param_ec_codec_wov_set_lang_shm *pp = &p.set_lang_shm_param;
    int ret;
    if (size > priv.wov_lang_shm_len) {
    dev_err(priv.dev, "no enough SHM size: %d\n",
    priv.wov_lang_shm_len);
    return -EIO;
    }
    switch (priv.wov_lang_shm_type) {
    case EC_CODEC_SHM_TYPE_EC_RAM:
    memcpy_toio((void  __iomem *)priv.wov_lang_shm_p,
    buf, size);
    memset_io((void  __iomem *)priv.wov_lang_shm_p + size,
    0, priv.wov_lang_shm_len - size);
    break;
    case EC_CODEC_SHM_TYPE_SYSTEM_RAM:
    memcpy(priv.wov_lang_shm_p, buf, size);
    memset(priv.wov_lang_shm_p + size, 0,
    priv.wov_lang_shm_len - size);
// make sure write to memory before calling host command
    wmb();
    break;
    }
    p.cmd = EC_CODEC_WOV_SET_LANG_SHM;
    memcpy(pp.hash, digest, SHA256_DIGEST_SIZE);
    pp.total_len = size;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_WOV,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(priv.dev, "failed to EC_CODEC_WOV_SET_LANG_SHM\n");
    return ret;
    }
    return 0;
    }
    static int wov_set_lang(struct cros_ec_codec_priv *priv,
    uint8_t *buf, size_t size, uint8_t *digest)
    {
    struct ec_param_ec_codec_wov p;
    struct ec_param_ec_codec_wov_set_lang *pp = &p.set_lang_param;
    size_t i, req;
    int ret;
    for (i = 0; i < size; i += req) {
    req = min(size - i, ARRAY_SIZE(pp.buf));
    p.cmd = EC_CODEC_WOV_SET_LANG;
    memcpy(pp.hash, digest, SHA256_DIGEST_SIZE);
    pp.total_len = size;
    pp.offset = i;
    memcpy(pp.buf, buf + i, req);
    pp.len = req;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_WOV,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(priv.dev, "failed to EC_CODEC_WOV_SET_LANG\n");
    return ret;
    }
    }
    return 0;
    }
    static int wov_hotword_model_put(struct snd_kcontrol *kcontrol,
    const unsigned int __user *bytes,
    unsigned int size)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    struct ec_param_ec_codec_wov p;
    struct ec_response_ec_codec_wov_get_lang r;
    uint8_t digest[SHA256_DIGEST_SIZE];
    uint8_t *buf;
    int ret;
// Skips the TLV header.
    bytes += 2;
    size -= 8;
    dev_dbg(priv.dev, "%s: size=%d\n", __func__, size);
    buf = memdup_user(bytes, size);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    sha256(buf, size, digest);
    dev_dbg(priv.dev, "hash=%*phN\n", SHA256_DIGEST_SIZE, digest);
    p.cmd = EC_CODEC_WOV_GET_LANG;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_WOV,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret)
    goto leave;
    if (memcmp(digest, r.hash, SHA256_DIGEST_SIZE) == 0) {
    dev_dbg(priv.dev, "not updated");
    goto leave;
    }
    if (ec_codec_capable(priv, EC_CODEC_CAP_WOV_LANG_SHM))
    ret = wov_set_lang_shm(priv, buf, size, digest);
    else
    ret = wov_set_lang(priv, buf, size, digest);
    leave:
    kfree(buf);
    return ret;
    }
    static struct snd_kcontrol_new wov_controls[] = {
    SOC_SINGLE_BOOL_EXT("Wake-on-Voice Switch", 0,
    wov_enable_get, wov_enable_put),
    SND_SOC_BYTES_TLV("Hotword Model", 0x11000, core::ptr::null_mut(),
    wov_hotword_model_put),
    };
    static struct snd_soc_dai_driver wov_dai_driver = {
    .name = "Wake on Voice",
    .capture = {
    .stream_name = "WoV Capture",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_16000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    };
    static int wov_host_event(struct notifier_block *nb,
    unsigned long queued_during_suspend, void *notify)
    {
    struct cros_ec_codec_priv *priv =
    container_of(nb, struct cros_ec_codec_priv, wov_notifier);
    u32 host_event;
    dev_dbg(priv.dev, "%s\n", __func__);
    host_event = cros_ec_get_host_event(priv.ec_device);
    if (host_event & EC_HOST_EVENT_MASK(EC_HOST_EVENT_WOV)) {
    schedule_delayed_work(&priv.wov_copy_work, 0);
    return NOTIFY_OK;
    } else {
    return NOTIFY_DONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn wov_probe(component: *mut snd_soc_component) -> c_int {
    static int wov_probe(struct snd_soc_component *component)
    {
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    int ret;
    mutex_init(&priv.wov_dma_lock);
    INIT_DELAYED_WORK(&priv.wov_copy_work, wov_copy_work);
    priv.wov_notifier.notifier_call = wov_host_event;
    ret = blocking_notifier_chain_register(
    &priv.ec_device.event_notifier, &priv.wov_notifier);
    if (ret)
    return ret;
    if (ec_codec_capable(priv, EC_CODEC_CAP_WOV_LANG_SHM)) {
    priv.wov_lang_shm_p = wov_map_shm(priv,
    EC_CODEC_SHM_ID_WOV_LANG,
    &priv.wov_lang_shm_len,
    &priv.wov_lang_shm_type);
    if (!priv.wov_lang_shm_p)
    return -EFAULT;
    }
    if (ec_codec_capable(priv, EC_CODEC_CAP_WOV_AUDIO_SHM)) {
    priv.wov_audio_shm_p = wov_map_shm(priv,
    EC_CODEC_SHM_ID_WOV_AUDIO,
    &priv.wov_audio_shm_len,
    &priv.wov_audio_shm_type);
    if (!priv.wov_audio_shm_p)
    return -EFAULT;
    }
    return dmic_probe(component);
    }
#[no_mangle]
unsafe extern "C" fn wov_remove(component: *mut snd_soc_component) {
    static void wov_remove(struct snd_soc_component *component)
    {
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    blocking_notifier_chain_unregister(
    &priv.ec_device.event_notifier, &priv.wov_notifier);
    }
    static int wov_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    static const struct snd_pcm_hardware hw_param = {
    .info = SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    .rates = SNDRV_PCM_RATE_16000,
    .channels_min = 1,
    .channels_max = 1,
    .period_bytes_min = PAGE_SIZE,
    .period_bytes_max = 0x20000 / 8,
    .periods_min = 8,
    .periods_max = 8,
    .buffer_bytes_max = 0x20000,
    };
    return snd_soc_set_runtime_hwparams(substream, &hw_param);
    }
    static int wov_pcm_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    guard(mutex)(&priv.wov_dma_lock);
    priv.wov_substream = substream;
    priv.wov_rp = priv.wov_wp = 0;
    priv.wov_dma_offset = 0;
    priv.wov_burst_read = true;
    return 0;
    }
    static int wov_pcm_hw_free(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    scoped_guard(mutex, &priv.wov_dma_lock) {
    wov_queue_dequeue(priv, wov_queue_size(priv));
    priv.wov_substream = core::ptr::null_mut();
    }
    cancel_delayed_work_sync(&priv.wov_copy_work);
    return 0;
    }
    static snd_pcm_uframes_t wov_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cros_ec_codec_priv *priv =
    snd_soc_component_get_drvdata(component);
    return bytes_to_frames(runtime, priv.wov_dma_offset);
    }
    static int wov_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    snd_pcm_set_managed_buffer_all(rtd.pcm, SNDRV_DMA_TYPE_VMALLOC,
    core::ptr::null_mut(), 0, 0);
    return 0;
    }
    static const struct snd_soc_component_driver wov_component_driver = {
    .probe		= wov_probe,
    .remove		= wov_remove,
    .controls	= wov_controls,
    .num_controls	= ARRAY_SIZE(wov_controls),
    .open		= wov_pcm_open,
    .hw_params	= wov_pcm_hw_params,
    .hw_free	= wov_pcm_hw_free,
    .pointer	= wov_pcm_pointer,
    .pcm_new	= wov_pcm_new,
    };
#[no_mangle]
unsafe extern "C" fn cros_ec_codec_platform_probe(pdev: *mut platform_device) -> c_int {
    static int cros_ec_codec_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cros_ec_device *ec_device = dev_get_drvdata(pdev.dev.parent);
    struct cros_ec_codec_priv *priv;
    struct ec_param_ec_codec p;
    struct ec_response_ec_codec_get_capabilities r;
    int ret;

    struct resource res;
    u64 ec_shm_size;
    const __be32 *regaddr_p;

    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;

    regaddr_p = of_get_address(dev.of_node, 0, &ec_shm_size, core::ptr::null_mut());
    if (regaddr_p) {
    priv.ec_shm_addr = of_read_number(regaddr_p, 2);
    priv.ec_shm_len = ec_shm_size;
    dev_dbg(dev, "ec_shm_addr=%#llx len=%#x\n",
    priv.ec_shm_addr, priv.ec_shm_len);
    }
    ret = of_reserved_mem_region_to_resource(dev.of_node, 0, &res);
    if (!ret) {
    priv.ap_shm_phys_addr = res.start;
    priv.ap_shm_len = resource_size(&res);
    priv.ap_shm_addr =
    (uint64_t)(uintptr_t)devm_ioremap_wc(
    dev, priv.ap_shm_phys_addr,
    priv.ap_shm_len);
    priv.ap_shm_last_alloc = priv.ap_shm_phys_addr;
    dev_dbg(dev, "ap_shm_phys_addr=%#llx len=%#x\n",
    priv.ap_shm_phys_addr, priv.ap_shm_len);
    }

    priv.dev = dev;
    priv.ec_device = ec_device;
    atomic_set(&priv.dmic_probed, 0);
    p.cmd = EC_CODEC_GET_CAPABILITIES;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC,
    (uint8_t *)&p, sizeof(p),
    (uint8_t *)&r, sizeof(r));
    if (ret) {
    dev_err(dev, "failed to EC_CODEC_GET_CAPABILITIES\n");
    return ret;
    }
    priv.ec_capabilities = r.capabilities;
// Reset EC codec i2s rx.
    p.cmd = EC_CODEC_I2S_RX_RESET;
    ret = send_ec_host_command(priv.ec_device, EC_CMD_EC_CODEC_I2S_RX,
    (uint8_t *)&p, sizeof(p), core::ptr::null_mut(), 0);
    if (ret == -ENOPROTOOPT) {
    dev_info(dev,
    "Missing reset command. Please update EC firmware.\n");
    } else if (ret) {
    dev_err(dev, "failed to EC_CODEC_I2S_RESET: %d\n", ret);
    return ret;
    }
    platform_set_drvdata(pdev, priv);
    ret = devm_snd_soc_register_component(dev, &i2s_rx_component_driver,
    &i2s_rx_dai_driver, 1);
    if (ret)
    return ret;
    return devm_snd_soc_register_component(dev, &wov_component_driver,
    &wov_dai_driver, 1);
    }

    static const struct of_device_id cros_ec_codec_of_match[] = {
    { .compatible = "google,cros-ec-codec" },
    {},
    };
    MODULE_DEVICE_TABLE(of, cros_ec_codec_of_match);

    static const struct acpi_device_id cros_ec_codec_acpi_id[] = {
    { "GOOG0013", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, cros_ec_codec_acpi_id);

    static struct platform_driver cros_ec_codec_platform_driver = {
    .driver = {
    .name = "cros-ec-codec",
    .of_match_table = of_match_ptr(cros_ec_codec_of_match),
    .acpi_match_table = ACPI_PTR(cros_ec_codec_acpi_id),
    },
    .probe = cros_ec_codec_platform_probe,
    };
    module_platform_driver(cros_ec_codec_platform_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ChromeOS EC codec driver");
    MODULE_AUTHOR("Cheng-Yi Chiang <cychiang@chromium.org>");
    MODULE_ALIAS("platform:cros-ec-codec");
