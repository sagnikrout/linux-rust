//! Automatically rewritten from C to Rust
//! Source: sound/soc/atmel/atmel-pcm-pdc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// atmel-pcm.c  --  ALSA PCM interface for the Atmel atmel SoC.
//
// Copyright (C) 2005 SAN People
// Copyright (C) 2008 Atmel
//
// Authors: Sedji Gaouaou <sedji.gaouaou@atmel.com>
//
// Based on at91-pcm. by:
// Frank Mandarino <fmandarino@endrelia.com>
// Copyright 2006 Endrelia Technologies Inc.
//
// Based on pxa2xx-pcm.c by:
//
// Author:	Nicolas Pitre
// Created:	Nov 30, 2004
// Copyright:	(C) 2004 MontaVista Software, Inc.
//

    static int atmel_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_card *card = rtd.card.snd_card;
    int ret;
    ret = dma_coerce_mask_and_coherent(card.dev, DMA_BIT_MASK(32));
    if (ret)
    return ret;
    snd_pcm_set_managed_buffer_all(rtd.pcm, SNDRV_DMA_TYPE_DEV,
    card.dev, ATMEL_SSC_DMABUF_SIZE,
    ATMEL_SSC_DMABUF_SIZE);
    return 0;
    }
// --------------------------------------------------------------------------*\
// Hardware definition
    \*--------------------------------------------------------------------------*/
// TODO: These values were taken from the AT91 platform driver, check
// them against real values for AT32
//
    static const struct snd_pcm_hardware atmel_pcm_hardware = {
    .info			= SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE,
    .period_bytes_min	= 32,
    .period_bytes_max	= 8192,
    .periods_min		= 2,
    .periods_max		= 1024,
    .buffer_bytes_max	= ATMEL_SSC_DMABUF_SIZE,
    };
// --------------------------------------------------------------------------*\
// Data types
    \*--------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_runtime_data {
    pub params: *mut atmel_pcm_dma_params,
    pub /: *mut *mut dma_addr_t dma_buffer; / physical address of dma buffer,
    pub /: *mut *mut dma_addr_t dma_buffer_end; / first address beyond DMA buffer,
    pub period_size: usize,
    pub /: *mut *mut dma_addr_t period_ptr; / physical address of next period,
}

// --------------------------------------------------------------------------*\
// ISR
    \*--------------------------------------------------------------------------*/
    static void atmel_pcm_dma_irq(u32 ssc_sr,
    struct snd_pcm_substream *substream)
    {
    struct atmel_runtime_data *prtd = substream.runtime.private_data;
    struct atmel_pcm_dma_params *params = prtd.params;
    static int count;
    count++;
    if (ssc_sr & params.mask.ssc_endbuf) {
    pr_warn("atmel-pcm: buffer %s on %s (SSC_SR=%#x, count=%d)\n",
    substream.stream == SNDRV_PCM_STREAM_PLAYBACK
    ? "underrun" : "overrun",
    params.name, ssc_sr, count);
// re-start the PDC
    ssc_writex(params.ssc.regs, ATMEL_PDC_PTCR,
    params.mask.pdc_disable);
    prtd.period_ptr += prtd.period_size;
    if (prtd.period_ptr >= prtd.dma_buffer_end)
    prtd.period_ptr = prtd.dma_buffer;
    ssc_writex(params.ssc.regs, params.pdc.xpr,
    prtd.period_ptr);
    ssc_writex(params.ssc.regs, params.pdc.xcr,
    prtd.period_size / params.pdc_xfer_size);
    ssc_writex(params.ssc.regs, ATMEL_PDC_PTCR,
    params.mask.pdc_enable);
    }
    if (ssc_sr & params.mask.ssc_endx) {
// Load the PDC next pointer and counter registers
    prtd.period_ptr += prtd.period_size;
    if (prtd.period_ptr >= prtd.dma_buffer_end)
    prtd.period_ptr = prtd.dma_buffer;
    ssc_writex(params.ssc.regs, params.pdc.xnpr,
    prtd.period_ptr);
    ssc_writex(params.ssc.regs, params.pdc.xncr,
    prtd.period_size / params.pdc_xfer_size);
    }
    snd_pcm_period_elapsed(substream);
    }
// --------------------------------------------------------------------------*\
// PCM operations
    \*--------------------------------------------------------------------------*/
    static int atmel_pcm_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct atmel_runtime_data *prtd = runtime.private_data;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
// this may get called several times by oss emulation
// with different params
    prtd.params = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    prtd.params.dma_intr_handler = atmel_pcm_dma_irq;
    prtd.dma_buffer = runtime.dma_addr;
    prtd.dma_buffer_end = runtime.dma_addr + runtime.dma_bytes;
    prtd.period_size = params_period_bytes(params);
    pr_debug("atmel-pcm: "
    "hw_params: DMA for %s initialized "
    "(dma_bytes=%zu, period_size=%zu)\n",
    prtd.params.name,
    runtime.dma_bytes,
    prtd.period_size);
    return 0;
    }
    static int atmel_pcm_hw_free(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct atmel_runtime_data *prtd = substream.runtime.private_data;
    struct atmel_pcm_dma_params *params = prtd.params;
    if (params != core::ptr::null_mut()) {
    ssc_writex(params.ssc.regs, SSC_PDC_PTCR,
    params.mask.pdc_disable);
    prtd.params.dma_intr_handler = core::ptr::null_mut();
    }
    return 0;
    }
    static int atmel_pcm_prepare(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct atmel_runtime_data *prtd = substream.runtime.private_data;
    struct atmel_pcm_dma_params *params = prtd.params;
    ssc_writex(params.ssc.regs, SSC_IDR,
    params.mask.ssc_endx | params.mask.ssc_endbuf);
    ssc_writex(params.ssc.regs, ATMEL_PDC_PTCR,
    params.mask.pdc_disable);
    return 0;
    }
    static int atmel_pcm_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_pcm_runtime *rtd = substream.runtime;
    struct atmel_runtime_data *prtd = rtd.private_data;
    struct atmel_pcm_dma_params *params = prtd.params;
    let mut ret: c_int = 0;
    pr_debug("atmel-pcm:buffer_size = %ld,"
    "dma_area = %p, dma_bytes = %zu\n",
    rtd.buffer_size, rtd.dma_area, rtd.dma_bytes);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    prtd.period_ptr = prtd.dma_buffer;
    ssc_writex(params.ssc.regs, params.pdc.xpr,
    prtd.period_ptr);
    ssc_writex(params.ssc.regs, params.pdc.xcr,
    prtd.period_size / params.pdc_xfer_size);
    prtd.period_ptr += prtd.period_size;
    ssc_writex(params.ssc.regs, params.pdc.xnpr,
    prtd.period_ptr);
    ssc_writex(params.ssc.regs, params.pdc.xncr,
    prtd.period_size / params.pdc_xfer_size);
    pr_debug("atmel-pcm: trigger: "
    "period_ptr=%lx, xpr=%u, "
    "xcr=%u, xnpr=%u, xncr=%u\n",
    (unsigned long)prtd.period_ptr,
    ssc_readx(params.ssc.regs, params.pdc.xpr),
    ssc_readx(params.ssc.regs, params.pdc.xcr),
    ssc_readx(params.ssc.regs, params.pdc.xnpr),
    ssc_readx(params.ssc.regs, params.pdc.xncr));
    ssc_writex(params.ssc.regs, SSC_IER,
    params.mask.ssc_endx | params.mask.ssc_endbuf);
    ssc_writex(params.ssc.regs, SSC_PDC_PTCR,
    params.mask.pdc_enable);
    pr_debug("sr=%u imr=%u\n",
    ssc_readx(params.ssc.regs, SSC_SR),
    ssc_readx(params.ssc.regs, SSC_IER));
    break;		/* SNDRV_PCM_TRIGGER_START */
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    ssc_writex(params.ssc.regs, ATMEL_PDC_PTCR,
    params.mask.pdc_disable);
    break;
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    ssc_writex(params.ssc.regs, ATMEL_PDC_PTCR,
    params.mask.pdc_enable);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static snd_pcm_uframes_t atmel_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct atmel_runtime_data *prtd = runtime.private_data;
    struct atmel_pcm_dma_params *params = prtd.params;
    dma_addr_t ptr;
    snd_pcm_uframes_t x;
    ptr = (dma_addr_t) ssc_readx(params.ssc.regs, params.pdc.xpr);
    x = bytes_to_frames(runtime, ptr - prtd.dma_buffer);
    if (x == runtime.buffer_size)
    x = 0;
    return x;
    }
    static int atmel_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct atmel_runtime_data *prtd;
    let mut ret: c_int = 0;
    snd_soc_set_runtime_hwparams(substream, &atmel_pcm_hardware);
// ensure that buffer size is a multiple of period size
    ret = snd_pcm_hw_constraint_integer(runtime,
    SNDRV_PCM_HW_PARAM_PERIODS);
    if (ret < 0)
    goto out;
    prtd = kzalloc_obj(struct atmel_runtime_data);
    if (prtd == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto out;
    }
    runtime.private_data = prtd;
    out:
    return ret;
    }
    static int atmel_pcm_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct atmel_runtime_data *prtd = substream.runtime.private_data;
    kfree(prtd);
    return 0;
    }
    static const struct snd_soc_component_driver atmel_soc_platform = {
    .open		= atmel_pcm_open,
    .close		= atmel_pcm_close,
    .hw_params	= atmel_pcm_hw_params,
    .hw_free	= atmel_pcm_hw_free,
    .prepare	= atmel_pcm_prepare,
    .trigger	= atmel_pcm_trigger,
    .pointer	= atmel_pcm_pointer,
    .pcm_new	= atmel_pcm_new,
    };
#[no_mangle]
pub unsafe extern "C" fn atmel_pcm_pdc_platform_register(dev: *mut device) -> c_int {
    int atmel_pcm_pdc_platform_register(struct device *dev)
    {
    return devm_snd_soc_register_component(dev, &atmel_soc_platform,
    core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL(atmel_pcm_pdc_platform_register);
    MODULE_AUTHOR("Sedji Gaouaou <sedji.gaouaou@atmel.com>");
    MODULE_DESCRIPTION("Atmel PCM module");
    MODULE_LICENSE("GPL");
