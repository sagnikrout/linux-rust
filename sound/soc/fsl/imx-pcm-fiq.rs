//! Automatically rewritten from C to Rust
//! Source: sound/soc/fsl/imx-pcm-fiq.c
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
// imx-pcm-fiq.c  --  ALSA Soc Audio Layer
//
// Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
//
// This code is based on code copyrighted by Freescale,
// Liam Girdwood, Javier Martin and probably others.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pcm_runtime_data {
    pub period: c_uint,
    pub periods: c_int,
    pub offset: c_ulong,
    pub hrt: hrtimer,
    pub poll_time_ns: c_int,
    pub substream: *mut snd_pcm_substream,
    pub playing: core::sync::atomic::AtomicI32,
    pub capturing: core::sync::atomic::AtomicI32,
}

#[no_mangle]
unsafe extern "C" fn snd_hrtimer_callback(hrt: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart snd_hrtimer_callback(struct hrtimer *hrt)
    {
    struct imx_pcm_runtime_data *iprtd =
    container_of(hrt, struct imx_pcm_runtime_data, hrt);
    struct snd_pcm_substream *substream = iprtd.substream;
    struct pt_regs regs;
    if (!atomic_read(&iprtd.playing) && !atomic_read(&iprtd.capturing))
    return HRTIMER_NORESTART;
    get_fiq_regs(&regs);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    iprtd.offset = regs.ARM_r8 & 0xffff;
    else
    iprtd.offset = regs.ARM_r9 & 0xffff;
    snd_pcm_period_elapsed(substream);
    hrtimer_forward_now(hrt, ns_to_ktime(iprtd.poll_time_ns));
    return HRTIMER_RESTART;
    }
    static struct fiq_handler fh = {
    .name		= DRV_NAME,
    };
    static int snd_imx_pcm_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct imx_pcm_runtime_data *iprtd = runtime.private_data;
    iprtd.periods = params_periods(params);
    iprtd.period = params_period_bytes(params);
    iprtd.offset = 0;
    iprtd.poll_time_ns = 1000000000 / params_rate(params) *
    params_period_size(params);
    return 0;
    }
    static int snd_imx_pcm_prepare(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct imx_pcm_runtime_data *iprtd = runtime.private_data;
    struct pt_regs regs;
    get_fiq_regs(&regs);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    regs.ARM_r8 = (iprtd.period * iprtd.periods - 1) << 16;
    else
    regs.ARM_r9 = (iprtd.period * iprtd.periods - 1) << 16;
    set_fiq_regs(&regs);
    return 0;
    }
    static int imx_pcm_fiq;
    static int snd_imx_pcm_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct imx_pcm_runtime_data *iprtd = runtime.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    atomic_set(&iprtd.playing, 1);
    else
    atomic_set(&iprtd.capturing, 1);
    hrtimer_start(&iprtd.hrt, ns_to_ktime(iprtd.poll_time_ns),
    HRTIMER_MODE_REL);
    enable_fiq(imx_pcm_fiq);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    atomic_set(&iprtd.playing, 0);
    else
    atomic_set(&iprtd.capturing, 0);
    if (!atomic_read(&iprtd.playing) &&
    !atomic_read(&iprtd.capturing))
    disable_fiq(imx_pcm_fiq);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static snd_pcm_uframes_t
    snd_imx_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct imx_pcm_runtime_data *iprtd = runtime.private_data;
    return bytes_to_frames(substream.runtime, iprtd.offset);
    }
    static const struct snd_pcm_hardware snd_imx_hardware = {
    .info = SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    .buffer_bytes_max = IMX_SSI_DMABUF_SIZE,
    .period_bytes_min = 128,
    .period_bytes_max = 16 * 1024,
    .periods_min = 4,
    .periods_max = 255,
    .fifo_size = 0,
    };
    static int snd_imx_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct imx_pcm_runtime_data *iprtd;
    int ret;
    iprtd = kzalloc_obj(*iprtd);
    if (iprtd == core::ptr::null_mut())
    return -ENOMEM;
    runtime.private_data = iprtd;
    iprtd.substream = substream;
    atomic_set(&iprtd.playing, 0);
    atomic_set(&iprtd.capturing, 0);
    hrtimer_setup(&iprtd.hrt, snd_hrtimer_callback, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    ret = snd_pcm_hw_constraint_integer(substream.runtime,
    SNDRV_PCM_HW_PARAM_PERIODS);
    if (ret < 0) {
    kfree(iprtd);
    return ret;
    }
    snd_soc_set_runtime_hwparams(substream, &snd_imx_hardware);
    return 0;
    }
    static int snd_imx_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct imx_pcm_runtime_data *iprtd = runtime.private_data;
    hrtimer_cancel(&iprtd.hrt);
    kfree(iprtd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_pcm_new(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int imx_pcm_new(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_card *card = rtd.card.snd_card;
    struct snd_pcm *pcm = rtd.pcm;
    int ret;
    ret = dma_coerce_mask_and_coherent(card.dev, DMA_BIT_MASK(32));
    if (ret)
    return ret;
    return snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_WC,
    pcm.card.dev,
    IMX_SSI_DMABUF_SIZE);
    }
    static int ssi_irq;
    static int snd_imx_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_pcm *pcm = rtd.pcm;
    struct snd_pcm_substream *substream;
    int ret;
    ret = imx_pcm_new(rtd);
    if (ret)
    return ret;
    substream = pcm.streams[SNDRV_PCM_STREAM_PLAYBACK].substream;
    if (substream) {
    struct snd_dma_buffer *buf = &substream.dma_buffer;
    imx_ssi_fiq_tx_buffer = (unsigned long)buf.area;
    }
    substream = pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    if (substream) {
    struct snd_dma_buffer *buf = &substream.dma_buffer;
    imx_ssi_fiq_rx_buffer = (unsigned long)buf.area;
    }
    set_fiq_handler(&imx_ssi_fiq_start,
    &imx_ssi_fiq_end - &imx_ssi_fiq_start);
    return 0;
    }
    static void snd_imx_pcm_free(struct snd_soc_component *component,
    struct snd_pcm *pcm)
    {
    mxc_set_irq_fiq(ssi_irq, 0);
    release_fiq(&fh);
    }
    static const struct snd_soc_component_driver imx_soc_component_fiq = {
    .open		= snd_imx_open,
    .close		= snd_imx_close,
    .hw_params	= snd_imx_pcm_hw_params,
    .prepare	= snd_imx_pcm_prepare,
    .trigger	= snd_imx_pcm_trigger,
    .pointer	= snd_imx_pcm_pointer,
    .pcm_new	= snd_imx_pcm_new,
    .pcm_free	= snd_imx_pcm_free,
    };
    int imx_pcm_fiq_init(struct platform_device *pdev,
    struct imx_pcm_fiq_params *params)
    {
    int ret;
    ret = claim_fiq(&fh);
    if (ret) {
    dev_err(&pdev.dev, "failed to claim fiq: %d", ret);
    return ret;
    }
    mxc_set_irq_fiq(params.irq, 1);
    ssi_irq = params.irq;
    imx_pcm_fiq = params.irq;
    imx_ssi_fiq_base = (unsigned long)params.base;
    params.dma_params_tx.maxburst = 4;
    params.dma_params_rx.maxburst = 6;
    ret = devm_snd_soc_register_component(&pdev.dev, &imx_soc_component_fiq,
    core::ptr::null_mut(), 0);
    if (ret)
    goto failed_register;
    return 0;
    failed_register:
    mxc_set_irq_fiq(ssi_irq, 0);
    release_fiq(&fh);
    return ret;
    }
    EXPORT_SYMBOL_GPL(imx_pcm_fiq_init);
#[no_mangle]
pub unsafe extern "C" fn imx_pcm_fiq_exit(pdev: *mut platform_device) {
    void imx_pcm_fiq_exit(struct platform_device *pdev)
    {
    }
    EXPORT_SYMBOL_GPL(imx_pcm_fiq_exit);
    MODULE_DESCRIPTION("Freescale i.MX PCM FIQ handler");
    MODULE_LICENSE("GPL");
