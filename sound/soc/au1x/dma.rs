//! Automatically rewritten from C to Rust
//! Source: sound/soc/au1x/dma.c
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
// Au1000/Au1500/Au1100 Audio DMA support.
//
// (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
//
// copied almost verbatim from the old ALSA driver, written by
// Charles Eidsness <charles@cooper-street.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_period {
    pub start: u32,
    pub /: *mut *mut u32 relative_end; / relative to start of buffer,
    pub next: *mut pcm_period,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_stream {
    pub substream: *mut snd_pcm_substream,
    pub dma: c_int,
    pub buffer: *mut pcm_period,
    pub period_size: c_uint,
    pub periods: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alchemy_pcm_ctx {
    pub /: *mut *mut audio_stream stream[2]; / playback & capture,
}

#[no_mangle]
unsafe extern "C" fn au1000_release_dma_link(stream: *mut audio_stream) {
    static void au1000_release_dma_link(struct audio_stream *stream)
    {
    struct pcm_period *pointer;
    struct pcm_period *pointer_next;
    stream.period_size = 0;
    stream.periods = 0;
    pointer = stream.buffer;
    if (!pointer)
    return;
    do {
    pointer_next = pointer.next;
    kfree(pointer);
    pointer = pointer_next;
    } while (pointer != stream.buffer);
    stream.buffer = core::ptr::null_mut();
    }
    static int au1000_setup_dma_link(struct audio_stream *stream,
    unsigned int period_bytes,
    unsigned int periods)
    {
    struct snd_pcm_substream *substream = stream.substream;
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct pcm_period *pointer;
    unsigned long dma_start;
    int i;
    dma_start = virt_to_phys(runtime.dma_area);
    if (stream.period_size == period_bytes &&
    stream.periods == periods)
    return 0; /* not changed */
    au1000_release_dma_link(stream);
    stream.period_size = period_bytes;
    stream.periods = periods;
    stream.buffer = kmalloc_obj(struct pcm_period);
    if (!stream.buffer)
    return -ENOMEM;
    pointer = stream.buffer;
    for (i = 0; i < periods; i++) {
    pointer.start = (u32)(dma_start + (i * period_bytes));
    pointer.relative_end = (u32) (((i+1) * period_bytes) - 0x1);
    if (i < periods - 1) {
    pointer.next = kmalloc_obj(struct pcm_period);
    if (!pointer.next) {
    au1000_release_dma_link(stream);
    return -ENOMEM;
    }
    pointer = pointer.next;
    }
    }
    pointer.next = stream.buffer;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1000_dma_stop(stream: *mut audio_stream) {
    static void au1000_dma_stop(struct audio_stream *stream)
    {
    if (stream.buffer)
    disable_dma(stream.dma);
    }
#[no_mangle]
unsafe extern "C" fn au1000_dma_start(stream: *mut audio_stream) {
    static void au1000_dma_start(struct audio_stream *stream)
    {
    if (!stream.buffer)
    return;
    init_dma(stream.dma);
    if (get_dma_active_buffer(stream.dma) == 0) {
    clear_dma_done0(stream.dma);
    set_dma_addr0(stream.dma, stream.buffer.start);
    set_dma_count0(stream.dma, stream.period_size >> 1);
    set_dma_addr1(stream.dma, stream.buffer.next.start);
    set_dma_count1(stream.dma, stream.period_size >> 1);
    } else {
    clear_dma_done1(stream.dma);
    set_dma_addr1(stream.dma, stream.buffer.start);
    set_dma_count1(stream.dma, stream.period_size >> 1);
    set_dma_addr0(stream.dma, stream.buffer.next.start);
    set_dma_count0(stream.dma, stream.period_size >> 1);
    }
    enable_dma_buffers(stream.dma);
    start_dma(stream.dma);
    }
#[no_mangle]
unsafe extern "C" fn au1000_dma_interrupt(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t au1000_dma_interrupt(int irq, void *ptr)
    {
    struct audio_stream *stream = (struct audio_stream *)ptr;
    struct snd_pcm_substream *substream = stream.substream;
    switch (get_dma_buffer_done(stream.dma)) {
    case DMA_D0:
    stream.buffer = stream.buffer.next;
    clear_dma_done0(stream.dma);
    set_dma_addr0(stream.dma, stream.buffer.next.start);
    set_dma_count0(stream.dma, stream.period_size >> 1);
    enable_dma_buffer0(stream.dma);
    break;
    case DMA_D1:
    stream.buffer = stream.buffer.next;
    clear_dma_done1(stream.dma);
    set_dma_addr1(stream.dma, stream.buffer.next.start);
    set_dma_count1(stream.dma, stream.period_size >> 1);
    enable_dma_buffer1(stream.dma);
    break;
    case (DMA_D0 | DMA_D1):
    pr_debug("DMA %d missed interrupt.\n", stream.dma);
    au1000_dma_stop(stream);
    au1000_dma_start(stream);
    break;
    case (~DMA_D0 & ~DMA_D1):
    pr_debug("DMA %d empty irq.\n", stream.dma);
    }
    snd_pcm_period_elapsed(substream);
    return IRQ_HANDLED;
    }
    static const struct snd_pcm_hardware alchemy_pcm_hardware = {
    .info		  = SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BATCH,
    .period_bytes_min = 1024,
    .period_bytes_max = 16 * 1024 - 1,
    .periods_min	  = 4,
    .periods_max	  = 255,
    .buffer_bytes_max = 128 * 1024,
    .fifo_size	  = 16,
    };
    static inline struct alchemy_pcm_ctx *ss_to_ctx(struct snd_pcm_substream *ss,
    struct snd_soc_component *component)
    {
    return snd_soc_component_get_drvdata(component);
    }
    static inline struct audio_stream *ss_to_as(struct snd_pcm_substream *ss,
    struct snd_soc_component *component)
    {
    struct alchemy_pcm_ctx *ctx = ss_to_ctx(ss, component);
    return &(ctx.stream[ss.stream]);
    }
    static int alchemy_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct alchemy_pcm_ctx *ctx = ss_to_ctx(substream, component);
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    int *dmaids, s = substream.stream;
    char *name;
    dmaids = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    if (!dmaids)
    return -ENODEV;	/* whoa, has ordering changed? */
// DMA setup
    name = (s == SNDRV_PCM_STREAM_PLAYBACK) ? "audio-tx" : "audio-rx";
    ctx.stream[s].dma = request_au1000_dma(dmaids[s], name,
    au1000_dma_interrupt, 0,
    &ctx.stream[s]);
    set_dma_mode(ctx.stream[s].dma,
    get_dma_mode(ctx.stream[s].dma) & ~DMA_NC);
    ctx.stream[s].substream = substream;
    ctx.stream[s].buffer = core::ptr::null_mut();
    snd_soc_set_runtime_hwparams(substream, &alchemy_pcm_hardware);
    return 0;
    }
    static int alchemy_pcm_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct alchemy_pcm_ctx *ctx = ss_to_ctx(substream, component);
    let mut stype: c_int = substream.stream;
    ctx.stream[stype].substream = core::ptr::null_mut();
    free_au1000_dma(ctx.stream[stype].dma);
    return 0;
    }
    static int alchemy_pcm_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct audio_stream *stream = ss_to_as(substream, component);
    return au1000_setup_dma_link(stream,
    params_period_bytes(hw_params),
    params_periods(hw_params));
    }
    static int alchemy_pcm_hw_free(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct audio_stream *stream = ss_to_as(substream, component);
    au1000_release_dma_link(stream);
    return 0;
    }
    static int alchemy_pcm_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct audio_stream *stream = ss_to_as(substream, component);
    let mut err: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    au1000_dma_start(stream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    au1000_dma_stop(stream);
    break;
    default:
    err = -EINVAL;
    break;
    }
    return err;
    }
    static snd_pcm_uframes_t alchemy_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *ss)
    {
    struct audio_stream *stream = ss_to_as(ss, component);
    long location;
    location = get_dma_residue(stream.dma);
    location = stream.buffer.relative_end - location;
    if (location == -1)
    location = 0;
    return bytes_to_frames(ss.runtime, location);
    }
    static int alchemy_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_pcm *pcm = rtd.pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_CONTINUOUS,
    core::ptr::null_mut(), 65536, (4096 * 1024) - 1);
    return 0;
    }
    static const struct snd_soc_component_driver alchemy_pcm_soc_component = {
    .name		= DRV_NAME,
    .open		= alchemy_pcm_open,
    .close		= alchemy_pcm_close,
    .hw_params	= alchemy_pcm_hw_params,
    .hw_free	= alchemy_pcm_hw_free,
    .trigger	= alchemy_pcm_trigger,
    .pointer	= alchemy_pcm_pointer,
    .pcm_new	= alchemy_pcm_new,
    };
#[no_mangle]
unsafe extern "C" fn alchemy_pcm_drvprobe(pdev: *mut platform_device) -> c_int {
    static int alchemy_pcm_drvprobe(struct platform_device *pdev)
    {
    struct alchemy_pcm_ctx *ctx;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    platform_set_drvdata(pdev, ctx);
    return devm_snd_soc_register_component(&pdev.dev,
    &alchemy_pcm_soc_component, core::ptr::null_mut(), 0);
    }
    static struct platform_driver alchemy_pcmdma_driver = {
    .driver	= {
    .name	= "alchemy-pcm-dma",
    },
    .probe		= alchemy_pcm_drvprobe,
    };
    module_platform_driver(alchemy_pcmdma_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Au1000/Au1500/Au1100 Audio DMA driver");
    MODULE_AUTHOR("Manuel Lauss");
