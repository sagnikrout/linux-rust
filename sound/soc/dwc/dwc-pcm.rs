//! Automatically rewritten from C to Rust
//! Source: sound/soc/dwc/dwc-pcm.c
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
// ALSA SoC Synopsys PIO PCM for I2S driver
//
// sound/soc/dwc/designware_pcm.c
//
// Copyright (C) 2016 Synopsys
// Jose Abreu <joabreu@synopsys.com>
//

pub const PERIOD_BYTES_MIN: c_int = 4096;
pub const PERIODS_MIN: c_int = 2;

    static unsigned int dw_pcm_tx_##sample_bits(struct dw_i2s_dev *dev, \
    struct snd_pcm_runtime *runtime, unsigned int tx_ptr, \
    bool *period_elapsed) \
    { \
    const u##sample_bits (*p)[2] = (void *)runtime.dma_area; \
    unsigned int period_pos = tx_ptr % runtime.period_size; \
    int i; \
    \
    for (i = 0; i < dev.fifo_th; i++) { \
    iowrite32(p[tx_ptr][0], dev.i2s_base + dev.l_reg); \
    iowrite32(p[tx_ptr][1], dev.i2s_base + dev.r_reg); \
    period_pos++; \
    if (++tx_ptr >= runtime.buffer_size) \
    tx_ptr = 0; \
    } \
// period_elapsed = period_pos >= runtime->period_size; \
    return tx_ptr; \
    }

    static unsigned int dw_pcm_rx_##sample_bits(struct dw_i2s_dev *dev, \
    struct snd_pcm_runtime *runtime, unsigned int rx_ptr, \
    bool *period_elapsed) \
    { \
    u##sample_bits (*p)[2] = (void *)runtime.dma_area; \
    unsigned int period_pos = rx_ptr % runtime.period_size; \
    int i; \
    \
    for (i = 0; i < dev.fifo_th; i++) { \
    p[rx_ptr][0] = ioread32(dev.i2s_base + dev.l_reg); \
    p[rx_ptr][1] = ioread32(dev.i2s_base + dev.r_reg); \
    period_pos++; \
    if (++rx_ptr >= runtime.buffer_size) \
    rx_ptr = 0; \
    } \
// period_elapsed = period_pos >= runtime->period_size; \
    return rx_ptr; \
    }
    dw_pcm_tx_fn(16);
    dw_pcm_tx_fn(32);
    dw_pcm_rx_fn(16);
    dw_pcm_rx_fn(32);

    static const struct snd_pcm_hardware dw_pcm_hardware = {
    .info = SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_BLOCK_TRANSFER,
    .rates = SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000,
    .rate_min = 32000,
    .rate_max = 48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min = 2,
    .channels_max = 2,
    .buffer_bytes_max = BUFFER_BYTES_MAX,
    .period_bytes_min = PERIOD_BYTES_MIN,
    .period_bytes_max = BUFFER_BYTES_MAX / PERIODS_MIN,
    .periods_min = PERIODS_MIN,
    .periods_max = BUFFER_BYTES_MAX / PERIOD_BYTES_MIN,
    .fifo_size = 16,
    };
#[no_mangle]
unsafe extern "C" fn dw_pcm_transfer(dev: *mut dw_i2s_dev, push: bool) {
    static void dw_pcm_transfer(struct dw_i2s_dev *dev, bool push)
    {
    struct snd_pcm_substream *substream;
    bool active, period_elapsed;
    rcu_read_lock();
    if (push)
    substream = rcu_dereference(dev.tx_substream);
    else
    substream = rcu_dereference(dev.rx_substream);
    active = substream && snd_pcm_running(substream);
    if (active) {
    unsigned int ptr;
    unsigned int new_ptr;
    if (push) {
    ptr = READ_ONCE(dev.tx_ptr);
    new_ptr = dev.tx_fn(dev, substream.runtime, ptr,
    &period_elapsed);
    cmpxchg(&dev.tx_ptr, ptr, new_ptr);
    } else {
    ptr = READ_ONCE(dev.rx_ptr);
    new_ptr = dev.rx_fn(dev, substream.runtime, ptr,
    &period_elapsed);
    cmpxchg(&dev.rx_ptr, ptr, new_ptr);
    }
    if (period_elapsed)
    snd_pcm_period_elapsed(substream);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn dw_pcm_push_tx(dev: *mut dw_i2s_dev) {
    void dw_pcm_push_tx(struct dw_i2s_dev *dev)
    {
    dw_pcm_transfer(dev, true);
    }
#[no_mangle]
pub unsafe extern "C" fn dw_pcm_pop_rx(dev: *mut dw_i2s_dev) {
    void dw_pcm_pop_rx(struct dw_i2s_dev *dev)
    {
    dw_pcm_transfer(dev, false);
    }
    static int dw_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct dw_i2s_dev *dev = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0));
    snd_soc_set_runtime_hwparams(substream, &dw_pcm_hardware);
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    runtime.private_data = dev;
    return 0;
    }
    static int dw_pcm_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    synchronize_rcu();
    return 0;
    }
    static int dw_pcm_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct dw_i2s_dev *dev = runtime.private_data;
    switch (params_channels(hw_params)) {
    case 2:
    break;
    default:
    dev_err(dev.dev, "invalid channels number\n");
    return -EINVAL;
    }
    switch (params_format(hw_params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    dev.tx_fn = dw_pcm_tx_16;
    dev.rx_fn = dw_pcm_rx_16;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    case SNDRV_PCM_FORMAT_S32_LE:
    dev.tx_fn = dw_pcm_tx_32;
    dev.rx_fn = dw_pcm_rx_32;
    break;
    default:
    dev_err(dev.dev, "invalid format\n");
    return -EINVAL;
    }
    return 0;
    }
    static int dw_pcm_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct dw_i2s_dev *dev = runtime.private_data;
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    WRITE_ONCE(dev.tx_ptr, 0);
    rcu_assign_pointer(dev.tx_substream, substream);
    } else {
    WRITE_ONCE(dev.rx_ptr, 0);
    rcu_assign_pointer(dev.rx_substream, substream);
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    rcu_assign_pointer(dev.tx_substream, core::ptr::null_mut());
    else
    rcu_assign_pointer(dev.rx_substream, core::ptr::null_mut());
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static snd_pcm_uframes_t dw_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct dw_i2s_dev *dev = runtime.private_data;
    snd_pcm_uframes_t pos;
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    pos = READ_ONCE(dev.tx_ptr);
    else
    pos = READ_ONCE(dev.rx_ptr);
    return pos < runtime.buffer_size ? pos : 0;
    }
    static int dw_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    let mut size: usize = dw_pcm_hardware.buffer_bytes_max;
    snd_pcm_set_managed_buffer_all(rtd.pcm,
    SNDRV_DMA_TYPE_CONTINUOUS,
    core::ptr::null_mut(), size, size);
    return 0;
    }
    static const struct snd_soc_component_driver dw_pcm_component = {
    .open		= dw_pcm_open,
    .close		= dw_pcm_close,
    .hw_params	= dw_pcm_hw_params,
    .trigger	= dw_pcm_trigger,
    .pointer	= dw_pcm_pointer,
    .pcm_new	= dw_pcm_new,
    };
#[no_mangle]
pub unsafe extern "C" fn dw_pcm_register(pdev: *mut platform_device) -> c_int {
    int dw_pcm_register(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev, &dw_pcm_component,
    core::ptr::null_mut(), 0);
    }
