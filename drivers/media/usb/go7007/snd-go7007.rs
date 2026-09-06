//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/go7007/snd-go7007.c
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
// Copyright (C) 2005-2006 Micronas USA Inc.
//

    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for the go7007 audio driver");
    MODULE_PARM_DESC(id, "ID string for the go7007 audio driver");
    MODULE_PARM_DESC(enable, "Enable for the go7007 audio driver");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct go7007_snd {
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub substream: *mut snd_pcm_substream,
    pub lock: spinlock_t,
    pub w_idx: c_int,
    pub hw_ptr: c_int,
    pub avail: c_int,
    pub capturing: c_int,
}

    static const struct snd_pcm_hardware go7007_snd_capture_hw = {
    .info			= (SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats		= SNDRV_PCM_FMTBIT_S16_LE,
    .rates			= SNDRV_PCM_RATE_48000,
    .rate_min		= 48000,
    .rate_max		= 48000,
    .channels_min		= 2,
    .channels_max		= 2,
    .buffer_bytes_max	= (128*1024),
    .period_bytes_min	= 4096,
    .period_bytes_max	= (128*1024),
    .periods_min		= 1,
    .periods_max		= 32,
    };
#[no_mangle]
unsafe extern "C" fn parse_audio_stream_data(go: *mut go7007, buf: *mut u8, length: c_int) {
    static void parse_audio_stream_data(struct go7007 *go, u8 *buf, int length)
    {
    struct go7007_snd *gosnd = go.snd_context;
    struct snd_pcm_runtime *runtime = gosnd.substream.runtime;
    let mut frames: c_int = bytes_to_frames(runtime, length);
    unsigned long flags;
    spin_lock_irqsave(&gosnd.lock, flags);
    gosnd.hw_ptr += frames;
    if (gosnd.hw_ptr >= runtime.buffer_size)
    gosnd.hw_ptr -= runtime.buffer_size;
    gosnd.avail += frames;
    spin_unlock_irqrestore(&gosnd.lock, flags);
    if (gosnd.w_idx + length > runtime.dma_bytes) {
    let mut cpy: c_int = runtime.dma_bytes - gosnd.w_idx;
    memcpy(runtime.dma_area + gosnd.w_idx, buf, cpy);
    length -= cpy;
    buf += cpy;
    gosnd.w_idx = 0;
    }
    memcpy(runtime.dma_area + gosnd.w_idx, buf, length);
    gosnd.w_idx += length;
    spin_lock_irqsave(&gosnd.lock, flags);
    if (gosnd.avail < runtime.period_size) {
    spin_unlock_irqrestore(&gosnd.lock, flags);
    return;
    }
    gosnd.avail -= runtime.period_size;
    spin_unlock_irqrestore(&gosnd.lock, flags);
    if (gosnd.capturing)
    snd_pcm_period_elapsed(gosnd.substream);
    }
    static int go7007_snd_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct go7007 *go = snd_pcm_substream_chip(substream);
    go.audio_deliver = parse_audio_stream_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn go7007_snd_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int go7007_snd_hw_free(struct snd_pcm_substream *substream)
    {
    struct go7007 *go = snd_pcm_substream_chip(substream);
    go.audio_deliver = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn go7007_snd_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int go7007_snd_capture_open(struct snd_pcm_substream *substream)
    {
    struct go7007 *go = snd_pcm_substream_chip(substream);
    struct go7007_snd *gosnd = go.snd_context;
    unsigned long flags;
    int r;
    spin_lock_irqsave(&gosnd.lock, flags);
    if (gosnd.substream == core::ptr::null_mut()) {
    gosnd.substream = substream;
    substream.runtime.hw = go7007_snd_capture_hw;
    r = 0;
    } else
    r = -EBUSY;
    spin_unlock_irqrestore(&gosnd.lock, flags);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn go7007_snd_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int go7007_snd_capture_close(struct snd_pcm_substream *substream)
    {
    struct go7007 *go = snd_pcm_substream_chip(substream);
    struct go7007_snd *gosnd = go.snd_context;
    gosnd.substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn go7007_snd_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int go7007_snd_pcm_prepare(struct snd_pcm_substream *substream)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn go7007_snd_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int go7007_snd_pcm_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct go7007 *go = snd_pcm_substream_chip(substream);
    struct go7007_snd *gosnd = go.snd_context;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
// Just set a flag to indicate we should signal ALSA when
// sound comes in
    gosnd.capturing = 1;
    return 0;
    case SNDRV_PCM_TRIGGER_STOP:
    gosnd.hw_ptr = gosnd.w_idx = gosnd.avail = 0;
    gosnd.capturing = 0;
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn go7007_snd_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t go7007_snd_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct go7007 *go = snd_pcm_substream_chip(substream);
    struct go7007_snd *gosnd = go.snd_context;
    return gosnd.hw_ptr;
    }
    static const struct snd_pcm_ops go7007_snd_capture_ops = {
    .open		= go7007_snd_capture_open,
    .close		= go7007_snd_capture_close,
    .hw_params	= go7007_snd_hw_params,
    .hw_free	= go7007_snd_hw_free,
    .prepare	= go7007_snd_pcm_prepare,
    .trigger	= go7007_snd_pcm_trigger,
    .pointer	= go7007_snd_pcm_pointer,
    };
#[no_mangle]
unsafe extern "C" fn go7007_snd_free(device: *mut snd_device) -> c_int {
    static int go7007_snd_free(struct snd_device *device)
    {
    struct go7007 *go = device.device_data;
    kfree(go.snd_context);
    go.snd_context = core::ptr::null_mut();
    return 0;
    }
    static const struct snd_device_ops go7007_snd_device_ops = {
    .dev_free	= go7007_snd_free,
    };
#[no_mangle]
unsafe extern "C" fn go7007_snd_card_free(card: *mut snd_card) {
    static void go7007_snd_card_free(struct snd_card *card)
    {
    struct go7007 *go = card.private_data;
    v4l2_device_put(&go.v4l2_dev);
    }
#[no_mangle]
pub unsafe extern "C" fn go7007_snd_init(go: *mut go7007) -> c_int {
    int go7007_snd_init(struct go7007 *go)
    {
    static int dev;
    struct go7007_snd *gosnd;
    int ret;
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (!enable[dev]) {
    dev++;
    return -ENOENT;
    }
    gosnd = kmalloc_obj(struct go7007_snd);
    if (gosnd == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&gosnd.lock);
    gosnd.hw_ptr = gosnd.w_idx = gosnd.avail = 0;
    gosnd.capturing = 0;
    ret = snd_card_new(go.dev, index[dev], id[dev], THIS_MODULE, 0,
    &gosnd.card);
    if (ret < 0)
    goto free_snd;
    ret = snd_device_new(gosnd.card, SNDRV_DEV_LOWLEVEL, go,
    &go7007_snd_device_ops);
    if (ret < 0)
    goto free_card;
    ret = snd_pcm_new(gosnd.card, "go7007", 0, 0, 1, &gosnd.pcm);
    if (ret < 0)
    goto free_card;
    strscpy(gosnd.card.driver, "go7007", sizeof(gosnd.card.driver));
    strscpy(gosnd.card.shortname, go.name, sizeof(gosnd.card.shortname));
    strscpy(gosnd.card.longname, gosnd.card.shortname,
    sizeof(gosnd.card.longname));
    gosnd.pcm.private_data = go;
    snd_pcm_set_ops(gosnd.pcm, SNDRV_PCM_STREAM_CAPTURE,
    &go7007_snd_capture_ops);
    snd_pcm_set_managed_buffer_all(gosnd.pcm, SNDRV_DMA_TYPE_VMALLOC,
    core::ptr::null_mut(), 0, 0);
    ret = snd_card_register(gosnd.card);
    if (ret < 0)
    goto free_card;
    gosnd.substream = core::ptr::null_mut();
    go.snd_context = gosnd;
    v4l2_device_get(&go.v4l2_dev);
    gosnd.card.private_data = go;
    gosnd.card.private_free = go7007_snd_card_free;
    ++dev;
    return 0;
    free_card:
    snd_card_free(gosnd.card);
    free_snd:
    kfree(gosnd);
    return ret;
    }
    EXPORT_SYMBOL(go7007_snd_init);
#[no_mangle]
pub unsafe extern "C" fn go7007_snd_remove(go: *mut go7007) -> c_int {
    int go7007_snd_remove(struct go7007 *go)
    {
    struct go7007_snd *gosnd = go.snd_context;
    snd_card_disconnect(gosnd.card);
    snd_card_free_when_closed(gosnd.card);
    return 0;
    }
    EXPORT_SYMBOL(go7007_snd_remove);
    MODULE_LICENSE("GPL v2");
