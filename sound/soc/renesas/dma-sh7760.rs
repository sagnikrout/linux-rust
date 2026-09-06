//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/dma-sh7760.c
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
// SH7760 ("camelot") DMABRG audio DMA unit support
//
// Copyright (C) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// The SH7760 DMABRG provides 4 dma channels (2x rec, 2x play), which
// trigger an interrupt when one half of the programmed transfer size
// has been xmitted.
//
// FIXME: little-endian only for now

// registers and bits
pub const BRGATXSAR: c_uint = 0x00;
pub const BRGARXDAR: c_uint = 0x04;
pub const BRGATXTCR: c_uint = 0x08;
pub const BRGARXTCR: c_uint = 0x0C;
pub const BRGACR: c_uint = 0x10;
pub const BRGATXTCNT: c_uint = 0x14;
pub const BRGARXTCNT: c_uint = 0x18;

// receiver/transmitter data alignment

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camelot_pcm {
    pub /: *mut *mut unsigned long mmio; / DMABRG audio channel control reg MMIO,
    pub /: *mut *mut unsigned int txid; / ID of first DMABRG IRQ for this unit,
    pub tx_ss: *mut snd_pcm_substream,
    pub tx_period_size: c_ulong,
    pub tx_period: c_uint,
    pub rx_ss: *mut snd_pcm_substream,
    pub rx_period_size: c_ulong,
    pub rx_period: c_uint,
}

    static struct camelot_pcm cam_pcm_data[2] = {
    {
    .mmio	=	0xFE3C0040,
    .txid	=	DMABRGIRQ_A0TXF,
    },
    {
    .mmio	=	0xFE3C0060,
    .txid	=	DMABRGIRQ_A1TXF,
    },
    };

//
// set a minimum of 16kb per period, to avoid interrupt-"storm" and
// resulting skipping. In general, the bigger the minimum size, the
// better for overall system performance. (The SH7760 is a puny CPU
// with a slow SDRAM interface and poor internal bus bandwidth,
// *especially* when the LCDC is active).  The minimum for the DMAC
// is 8 bytes; 16kbytes are enough to get skip-free playback of a
// 44kHz/16bit/stereo MP3 on a lightly loaded system, and maintain
// reasonable responsiveness in MPlayer.
//

pub const DMABRG_PERIOD_MAX: c_uint = 0x03fffffc;

    static const struct snd_pcm_hardware camelot_pcm_hardware = {
    .info = (SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_BATCH),
    .buffer_bytes_max =	DMABRG_PERIOD_MAX,
    .period_bytes_min =	DMABRG_PERIOD_MIN,
    .period_bytes_max =	DMABRG_PERIOD_MAX / 2,
    .periods_min =		2,
    .periods_max =		2,
    .fifo_size =		128,
    };
#[no_mangle]
unsafe extern "C" fn camelot_txdma(data: *mut c_void) {
    static void camelot_txdma(void *data)
    {
    struct camelot_pcm *cam = data;
    cam.tx_period ^= 1;
    snd_pcm_period_elapsed(cam.tx_ss);
    }
#[no_mangle]
unsafe extern "C" fn camelot_rxdma(data: *mut c_void) {
    static void camelot_rxdma(void *data)
    {
    struct camelot_pcm *cam = data;
    cam.rx_period ^= 1;
    snd_pcm_period_elapsed(cam.rx_ss);
    }
    static int camelot_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct camelot_pcm *cam = &cam_pcm_data[snd_soc_rtd_to_cpu(rtd, 0).id];
    let mut recv: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK ? 0:1;
    int ret, dmairq;
    snd_soc_set_runtime_hwparams(substream, &camelot_pcm_hardware);
// DMABRG buffer half/full events
    dmairq = (recv) ? cam.txid + 2 : cam.txid;
    if (recv) {
    cam.rx_ss = substream;
    ret = dmabrg_request_irq(dmairq, camelot_rxdma, cam);
    if (unlikely(ret)) {
    pr_debug("audio unit %d irqs already taken!\n",
    snd_soc_rtd_to_cpu(rtd, 0).id);
    return -EBUSY;
    }
    (void)dmabrg_request_irq(dmairq + 1,camelot_rxdma, cam);
    } else {
    cam.tx_ss = substream;
    ret = dmabrg_request_irq(dmairq, camelot_txdma, cam);
    if (unlikely(ret)) {
    pr_debug("audio unit %d irqs already taken!\n",
    snd_soc_rtd_to_cpu(rtd, 0).id);
    return -EBUSY;
    }
    (void)dmabrg_request_irq(dmairq + 1, camelot_txdma, cam);
    }
    return 0;
    }
    static int camelot_pcm_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct camelot_pcm *cam = &cam_pcm_data[snd_soc_rtd_to_cpu(rtd, 0).id];
    let mut recv: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK ? 0:1;
    int dmairq;
    dmairq = (recv) ? cam.txid + 2 : cam.txid;
    if (recv)
    cam.rx_ss = core::ptr::null_mut();
    else
    cam.tx_ss = core::ptr::null_mut();
    dmabrg_free_irq(dmairq + 1);
    dmabrg_free_irq(dmairq);
    return 0;
    }
    static int camelot_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct camelot_pcm *cam = &cam_pcm_data[snd_soc_rtd_to_cpu(rtd, 0).id];
    let mut recv: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK ? 0:1;
    if (recv) {
    cam.rx_period_size = params_period_bytes(hw_params);
    cam.rx_period = 0;
    } else {
    cam.tx_period_size = params_period_bytes(hw_params);
    cam.tx_period = 0;
    }
    return 0;
    }
    static int camelot_prepare(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct camelot_pcm *cam = &cam_pcm_data[snd_soc_rtd_to_cpu(rtd, 0).id];
    pr_debug("PCM data: addr %pad len %zu\n", &runtime.dma_addr,
    runtime.dma_bytes);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    BRGREG(BRGATXSAR) = (unsigned long)runtime.dma_area;
    BRGREG(BRGATXTCR) = runtime.dma_bytes;
    } else {
    BRGREG(BRGARXDAR) = (unsigned long)runtime.dma_area;
    BRGREG(BRGARXTCR) = runtime.dma_bytes;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dmabrg_play_dma_start(cam: *mut camelot_pcm) {
    static inline void dmabrg_play_dma_start(struct camelot_pcm *cam)
    {
    let mut acr: c_ulong = BRGREG(BRGACR) & ~(ACR_TDS | ACR_RDS);
// start DMABRG engine: XFER start, auto-addr-reload
    BRGREG(BRGACR) = acr | ACR_TDE | ACR_TAR | ACR_TAM_2WORD;
    }
#[no_mangle]
pub unsafe extern "C" fn dmabrg_play_dma_stop(cam: *mut camelot_pcm) {
    static inline void dmabrg_play_dma_stop(struct camelot_pcm *cam)
    {
    let mut acr: c_ulong = BRGREG(BRGACR) & ~(ACR_TDS | ACR_RDS);
// forcibly terminate data transmission
    BRGREG(BRGACR) = acr | ACR_TDS;
    }
#[no_mangle]
pub unsafe extern "C" fn dmabrg_rec_dma_start(cam: *mut camelot_pcm) {
    static inline void dmabrg_rec_dma_start(struct camelot_pcm *cam)
    {
    let mut acr: c_ulong = BRGREG(BRGACR) & ~(ACR_TDS | ACR_RDS);
// start DMABRG engine: recv start, auto-reload
    BRGREG(BRGACR) = acr | ACR_RDE | ACR_RAR | ACR_RAM_2WORD;
    }
#[no_mangle]
pub unsafe extern "C" fn dmabrg_rec_dma_stop(cam: *mut camelot_pcm) {
    static inline void dmabrg_rec_dma_stop(struct camelot_pcm *cam)
    {
    let mut acr: c_ulong = BRGREG(BRGACR) & ~(ACR_TDS | ACR_RDS);
// forcibly terminate data receiver
    BRGREG(BRGACR) = acr | ACR_RDS;
    }
    static int camelot_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct camelot_pcm *cam = &cam_pcm_data[snd_soc_rtd_to_cpu(rtd, 0).id];
    let mut recv: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK ? 0:1;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    if (recv)
    dmabrg_rec_dma_start(cam);
    else
    dmabrg_play_dma_start(cam);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    if (recv)
    dmabrg_rec_dma_stop(cam);
    else
    dmabrg_play_dma_stop(cam);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static snd_pcm_uframes_t camelot_pos(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct camelot_pcm *cam = &cam_pcm_data[snd_soc_rtd_to_cpu(rtd, 0).id];
    let mut recv: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK ? 0:1;
    unsigned long pos;
// cannot use the DMABRG pointer register: under load, by the
// time ALSA comes around to read the register, it is already
// far ahead (or worse, already done with the fragment) of the
// position at the time the IRQ was triggered, which results in
// fast-playback sound in my test application (ScummVM)
//
    if (recv)
    pos = cam.rx_period ? cam.rx_period_size : 0;
    else
    pos = cam.tx_period ? cam.tx_period_size : 0;
    return bytes_to_frames(runtime, pos);
    }
    static int camelot_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_pcm *pcm = rtd.pcm;
// dont use SNDRV_DMA_TYPE_DEV, since it will oops the SH kernel
// in MMAP mode (i.e. aplay -M)
//
    snd_pcm_set_managed_buffer_all(pcm,
    SNDRV_DMA_TYPE_CONTINUOUS,
    core::ptr::null_mut(),
    DMABRG_PREALLOC_BUFFER,	DMABRG_PREALLOC_BUFFER_MAX);
    return 0;
    }
    static const struct snd_soc_component_driver sh7760_soc_component = {
    .open		= camelot_pcm_open,
    .close		= camelot_pcm_close,
    .hw_params	= camelot_hw_params,
    .prepare	= camelot_prepare,
    .trigger	= camelot_trigger,
    .pointer	= camelot_pos,
    .pcm_new	= camelot_pcm_new,
    };
#[no_mangle]
unsafe extern "C" fn sh7760_soc_platform_probe(pdev: *mut platform_device) -> c_int {
    static int sh7760_soc_platform_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev, &sh7760_soc_component,
    core::ptr::null_mut(), 0);
    }
    static struct platform_driver sh7760_pcm_driver = {
    .driver = {
    .name = "sh7760-pcm-audio",
    },
    .probe = sh7760_soc_platform_probe,
    };
    module_platform_driver(sh7760_pcm_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("SH7760 Audio DMA (DMABRG) driver");
    MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>");
