//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/tw686x/tw686x-audio.c
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
// Copyright (C) 2015 VanguardiaSur - www.vanguardiasur.com.ar
//
// Based on the audio support from the tw6869 driver:
// Copyright 2015 www.starterkit.ru <info@starterkit.ru>
//
// Based on:
// Driver for Intersil|Techwell TW6869 based DVR cards
// (c) 2011-12 liran <jli11@intersil.com> [Intersil|Techwell China]
//

pub const AUDIO_CHANNEL_OFFSET: c_int = 8;
    void tw686x_audio_irq(struct tw686x_dev *dev, unsigned long requests,
    unsigned int pb_status)
    {
    unsigned long flags;
    unsigned int ch, pb;
    for_each_set_bit(ch, &requests, max_channels(dev)) {
    struct tw686x_audio_channel *ac = &dev.audio_channels[ch];
    struct tw686x_audio_buf *done = core::ptr::null_mut();
    struct tw686x_audio_buf *next = core::ptr::null_mut();
    struct tw686x_dma_desc *desc;
    pb = !!(pb_status & BIT(AUDIO_CHANNEL_OFFSET + ch));
    spin_lock_irqsave(&ac.lock, flags);
// Sanity check
    if (!ac.ss || !ac.curr_bufs[0] || !ac.curr_bufs[1]) {
    spin_unlock_irqrestore(&ac.lock, flags);
    continue;
    }
    if (!list_empty(&ac.buf_list)) {
    next = list_first_entry(&ac.buf_list,
    struct tw686x_audio_buf, list);
    list_move_tail(&next.list, &ac.buf_list);
    done = ac.curr_bufs[!pb];
    ac.curr_bufs[pb] = next;
    }
    spin_unlock_irqrestore(&ac.lock, flags);
    if (!done)
    continue;
//
// Checking for a non-nil dma_desc[pb]->virt buffer is
// the same as checking for memcpy DMA mode.
//
    desc = &ac.dma_descs[pb];
    if (desc.virt) {
    memcpy(done.virt, desc.virt,
    dev.period_size);
    } else {
    let mut reg: u32 = pb ? ADMA_B_ADDR[ch] : ADMA_P_ADDR[ch];
    reg_write(dev, reg, next.dma);
    }
    ac.ptr = done.dma - ac.buf[0].dma;
    snd_pcm_period_elapsed(ac.ss);
    }
    }
//
// Audio parameters are global and shared among all
// capture channels. The driver prevents changes to
// the parameters if any audio channel is capturing.
//
    static const struct snd_pcm_hardware tw686x_capture_hw = {
    .info			= (SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats		= SNDRV_PCM_FMTBIT_S16_LE,
    .rates			= SNDRV_PCM_RATE_8000_48000,
    .rate_min		= 8000,
    .rate_max		= 48000,
    .channels_min		= 1,
    .channels_max		= 1,
    .buffer_bytes_max	= TW686X_AUDIO_PAGE_MAX * AUDIO_DMA_SIZE_MAX,
    .period_bytes_min	= AUDIO_DMA_SIZE_MIN,
    .period_bytes_max	= AUDIO_DMA_SIZE_MAX,
    .periods_min		= TW686X_AUDIO_PERIODS_MIN,
    .periods_max		= TW686X_AUDIO_PERIODS_MAX,
    };
#[no_mangle]
unsafe extern "C" fn tw686x_pcm_open(ss: *mut snd_pcm_substream) -> c_int {
    static int tw686x_pcm_open(struct snd_pcm_substream *ss)
    {
    struct tw686x_dev *dev = snd_pcm_substream_chip(ss);
    struct tw686x_audio_channel *ac = &dev.audio_channels[ss.number];
    struct snd_pcm_runtime *rt = ss.runtime;
    int err;
    ac.ss = ss;
    rt.hw = tw686x_capture_hw;
    err = snd_pcm_hw_constraint_integer(rt, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw686x_pcm_close(ss: *mut snd_pcm_substream) -> c_int {
    static int tw686x_pcm_close(struct snd_pcm_substream *ss)
    {
    struct tw686x_dev *dev = snd_pcm_substream_chip(ss);
    struct tw686x_audio_channel *ac = &dev.audio_channels[ss.number];
    ac.ss = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw686x_pcm_prepare(ss: *mut snd_pcm_substream) -> c_int {
    static int tw686x_pcm_prepare(struct snd_pcm_substream *ss)
    {
    struct tw686x_dev *dev = snd_pcm_substream_chip(ss);
    struct tw686x_audio_channel *ac = &dev.audio_channels[ss.number];
    struct snd_pcm_runtime *rt = ss.runtime;
    let mut period_size: c_uint = snd_pcm_lib_period_bytes(ss);
    struct tw686x_audio_buf *p_buf, *b_buf;
    unsigned long flags;
    int i;
    spin_lock_irqsave(&dev.lock, flags);
//
// Given the audio parameters are global (i.e. shared across
// DMA channels), we need to check new params are allowed.
//
    if (((dev.audio_rate != rt.rate) ||
    (dev.period_size != period_size)) && dev.audio_enabled)
    goto err_audio_busy;
    tw686x_disable_channel(dev, AUDIO_CHANNEL_OFFSET + ac.ch);
    spin_unlock_irqrestore(&dev.lock, flags);
    if (dev.audio_rate != rt.rate) {
    u32 reg;
    dev.audio_rate = rt.rate;
    reg = ((125000000 / rt.rate) << 16) +
    ((125000000 % rt.rate) << 16) / rt.rate;
    reg_write(dev, AUDIO_CONTROL2, reg);
    }
    if (dev.period_size != period_size) {
    u32 reg;
    dev.period_size = period_size;
    reg = reg_read(dev, AUDIO_CONTROL1);
    reg &= ~(AUDIO_DMA_SIZE_MASK << AUDIO_DMA_SIZE_SHIFT);
    reg |= period_size << AUDIO_DMA_SIZE_SHIFT;
    reg_write(dev, AUDIO_CONTROL1, reg);
    }
    if (rt.periods < TW686X_AUDIO_PERIODS_MIN ||
    rt.periods > TW686X_AUDIO_PERIODS_MAX)
    return -EINVAL;
    spin_lock_irqsave(&ac.lock, flags);
    INIT_LIST_HEAD(&ac.buf_list);
    for (i = 0; i < rt.periods; i++) {
    ac.buf[i].dma = rt.dma_addr + period_size * i;
    ac.buf[i].virt = rt.dma_area + period_size * i;
    INIT_LIST_HEAD(&ac.buf[i].list);
    list_add_tail(&ac.buf[i].list, &ac.buf_list);
    }
    p_buf =	list_first_entry(&ac.buf_list, struct tw686x_audio_buf, list);
    list_move_tail(&p_buf.list, &ac.buf_list);
    b_buf =	list_first_entry(&ac.buf_list, struct tw686x_audio_buf, list);
    list_move_tail(&b_buf.list, &ac.buf_list);
    ac.curr_bufs[0] = p_buf;
    ac.curr_bufs[1] = b_buf;
    ac.ptr = 0;
    if (dev.dma_mode != TW686X_DMA_MODE_MEMCPY) {
    reg_write(dev, ADMA_P_ADDR[ac.ch], p_buf.dma);
    reg_write(dev, ADMA_B_ADDR[ac.ch], b_buf.dma);
    }
    spin_unlock_irqrestore(&ac.lock, flags);
    return 0;
    err_audio_busy:
    spin_unlock_irqrestore(&dev.lock, flags);
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn tw686x_pcm_trigger(ss: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int tw686x_pcm_trigger(struct snd_pcm_substream *ss, int cmd)
    {
    struct tw686x_dev *dev = snd_pcm_substream_chip(ss);
    struct tw686x_audio_channel *ac = &dev.audio_channels[ss.number];
    unsigned long flags;
    let mut err: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    if (ac.curr_bufs[0] && ac.curr_bufs[1]) {
    spin_lock_irqsave(&dev.lock, flags);
    dev.audio_enabled = 1;
    tw686x_enable_channel(dev,
    AUDIO_CHANNEL_OFFSET + ac.ch);
    spin_unlock_irqrestore(&dev.lock, flags);
    mod_timer(&dev.dma_delay_timer,
    jiffies + msecs_to_jiffies(100));
    } else {
    err = -EIO;
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    spin_lock_irqsave(&dev.lock, flags);
    dev.audio_enabled = 0;
    tw686x_disable_channel(dev, AUDIO_CHANNEL_OFFSET + ac.ch);
    spin_unlock_irqrestore(&dev.lock, flags);
    spin_lock_irqsave(&ac.lock, flags);
    ac.curr_bufs[0] = core::ptr::null_mut();
    ac.curr_bufs[1] = core::ptr::null_mut();
    spin_unlock_irqrestore(&ac.lock, flags);
    break;
    default:
    err = -EINVAL;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tw686x_pcm_pointer(ss: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t tw686x_pcm_pointer(struct snd_pcm_substream *ss)
    {
    struct tw686x_dev *dev = snd_pcm_substream_chip(ss);
    struct tw686x_audio_channel *ac = &dev.audio_channels[ss.number];
    return bytes_to_frames(ss.runtime, ac.ptr);
    }
    static const struct snd_pcm_ops tw686x_pcm_ops = {
    .open = tw686x_pcm_open,
    .close = tw686x_pcm_close,
    .prepare = tw686x_pcm_prepare,
    .trigger = tw686x_pcm_trigger,
    .pointer = tw686x_pcm_pointer,
    };
#[no_mangle]
unsafe extern "C" fn tw686x_snd_pcm_init(dev: *mut tw686x_dev) -> c_int {
    static int tw686x_snd_pcm_init(struct tw686x_dev *dev)
    {
    struct snd_card *card = dev.snd_card;
    struct snd_pcm *pcm;
    struct snd_pcm_substream *ss;
    unsigned int i;
    int err;
    err = snd_pcm_new(card, card.driver, 0, 0, max_channels(dev), &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &tw686x_pcm_ops);
    snd_pcm_chip(pcm) = dev;
    pcm.info_flags = 0;
    strscpy(pcm.name, "tw686x PCM", sizeof(pcm.name));
    for (i = 0, ss = pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    ss; ss = ss.next, i++)
    snprintf(ss.name, sizeof(ss.name), "vch%u audio", i);
    snd_pcm_set_managed_buffer_all(pcm,
    SNDRV_DMA_TYPE_DEV,
    &dev.pci_dev.dev,
    TW686X_AUDIO_PAGE_MAX * AUDIO_DMA_SIZE_MAX,
    TW686X_AUDIO_PAGE_MAX * AUDIO_DMA_SIZE_MAX);
    return 0;
    }
    static void tw686x_audio_dma_free(struct tw686x_dev *dev,
    struct tw686x_audio_channel *ac)
    {
    int pb;
    for (pb = 0; pb < 2; pb++) {
    if (!ac.dma_descs[pb].virt)
    continue;
    dma_free_coherent(&dev.pci_dev.dev, ac.dma_descs[pb].size,
    ac.dma_descs[pb].virt,
    ac.dma_descs[pb].phys);
    ac.dma_descs[pb].virt = core::ptr::null_mut();
    }
    }
    static int tw686x_audio_dma_alloc(struct tw686x_dev *dev,
    struct tw686x_audio_channel *ac)
    {
    int pb;
//
// In the memcpy DMA mode we allocate a coherent buffer
// and use it for the DMA capture. Otherwise, DMA
// acts on the ALSA buffers as received in pcm_prepare.
//
    if (dev.dma_mode != TW686X_DMA_MODE_MEMCPY)
    return 0;
    for (pb = 0; pb < 2; pb++) {
    let mut reg: u32 = pb ? ADMA_B_ADDR[ac.ch] : ADMA_P_ADDR[ac.ch];
    void *virt;
    virt = dma_alloc_coherent(&dev.pci_dev.dev,
    AUDIO_DMA_SIZE_MAX,
    &ac.dma_descs[pb].phys, GFP_KERNEL);
    if (!virt) {
    dev_err(&dev.pci_dev.dev,
    "dma%d: unable to allocate audio DMA %s-buffer\n",
    ac.ch, pb ? "B" : "P");
    return -ENOMEM;
    }
    ac.dma_descs[pb].virt = virt;
    ac.dma_descs[pb].size = AUDIO_DMA_SIZE_MAX;
    reg_write(dev, reg, ac.dma_descs[pb].phys);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tw686x_audio_free(dev: *mut tw686x_dev) {
    void tw686x_audio_free(struct tw686x_dev *dev)
    {
    unsigned long flags;
    u32 dma_ch_mask;
    u32 dma_cmd;
    spin_lock_irqsave(&dev.lock, flags);
    dma_cmd = reg_read(dev, DMA_CMD);
    dma_ch_mask = reg_read(dev, DMA_CHANNEL_ENABLE);
    reg_write(dev, DMA_CMD, dma_cmd & ~0xff00);
    reg_write(dev, DMA_CHANNEL_ENABLE, dma_ch_mask & ~0xff00);
    spin_unlock_irqrestore(&dev.lock, flags);
    if (!dev.snd_card)
    return;
    snd_card_free(dev.snd_card);
    dev.snd_card = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn tw686x_audio_init(dev: *mut tw686x_dev) -> c_int {
    int tw686x_audio_init(struct tw686x_dev *dev)
    {
    struct pci_dev *pci_dev = dev.pci_dev;
    struct snd_card *card;
    int err, ch;
// Enable external audio
    reg_write(dev, AUDIO_CONTROL1, BIT(0));
    err = snd_card_new(&pci_dev.dev, SNDRV_DEFAULT_IDX1,
    SNDRV_DEFAULT_STR1,
    THIS_MODULE, 0, &card);
    if (err < 0)
    return err;
    dev.snd_card = card;
    strscpy(card.driver, "tw686x", sizeof(card.driver));
    strscpy(card.shortname, "tw686x", sizeof(card.shortname));
    strscpy(card.longname, pci_name(pci_dev), sizeof(card.longname));
    snd_card_set_dev(card, &pci_dev.dev);
    for (ch = 0; ch < max_channels(dev); ch++) {
    struct tw686x_audio_channel *ac;
    ac = &dev.audio_channels[ch];
    spin_lock_init(&ac.lock);
    ac.dev = dev;
    ac.ch = ch;
    err = tw686x_audio_dma_alloc(dev, ac);
    if (err < 0)
    goto err_cleanup;
    }
    err = tw686x_snd_pcm_init(dev);
    if (err < 0)
    goto err_cleanup;
    err = snd_card_register(card);
    if (!err)
    return 0;
    err_cleanup:
    for (ch = 0; ch < max_channels(dev); ch++) {
    if (!dev.audio_channels[ch].dev)
    continue;
    tw686x_audio_dma_free(dev, &dev.audio_channels[ch]);
    }
    snd_card_free(card);
    dev.snd_card = core::ptr::null_mut();
    return err;
    }
