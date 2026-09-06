//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/rz-ssi.c
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
// Renesas RZ/G2L ASoC Serial Sound Interface (SSIF-2) Driver
//
// Copyright (C) 2021 Renesas Electronics Corp.
// Copyright (C) 2019 Chris Brandt.
//

// REGISTER OFFSET
pub const SSICR: c_uint = 0x000;
pub const SSISR: c_uint = 0x004;
pub const SSIFCR: c_uint = 0x010;
pub const SSIFSR: c_uint = 0x014;
pub const SSIFTDR: c_uint = 0x018;
pub const SSIFRDR: c_uint = 0x01c;
pub const SSIOFR: c_uint = 0x020;
pub const SSISCR: c_uint = 0x024;
// SSI REGISTER BITS

pub const SSIFSR_TDC_MASK: c_uint = 0x3f;
pub const SSIFSR_TDC_SHIFT: c_int = 24;
pub const SSIFSR_RDC_MASK: c_uint = 0x3f;
pub const SSIFSR_RDC_SHIFT: c_int = 8;

// Pre allocated buffers sizes

    SNDRV_PCM_FMTBIT_S32_LE)
pub const SSI_CHAN_MIN: c_int = 2;
pub const SSI_CHAN_MAX: c_int = 2;
pub const SSI_FIFO_DEPTH: c_int = 32;
    struct rz_ssi_priv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_ssi_stream {
    pub priv: *mut rz_ssi_priv,
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut int fifo_sample_size; / sample capacity of SSI FIFO,
    pub /: *mut *mut int period_counter; / for keeping track of periods transferred,
    pub /: *mut *mut int buffer_pos; / current frame position in the buffer,
    pub /: *mut *mut int running; / 0=stopped, 1=running,
    pub uerr_num: c_int,
    pub oerr_num: c_int,
    pub strm): *mut *mut *mut int (transfer)(struct rz_ssi_priv ssi, struct rz_ssi_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_ssi_priv {
    pub base: *mut void __iomem,
    pub rstc: *mut reset_control,
    pub dev: *mut device,
    pub sfr_clk: *mut clk,
    pub clk: *mut clk,
    pub irq_int: c_int,
    pub irq_tx: c_int,
    pub irq_rx: c_int,
    pub irq_rt: c_int,
    pub lock: spinlock_t,
//
// The SSI supports full-duplex transmission and reception.
// However, if an error occurs, channel reset (both transmission
// and reception reset) is required.
// So it is better to use as half-duplex (playing and recording
// should be done on separate channels).
//
    pub playback: rz_ssi_stream,
    pub capture: rz_ssi_stream,
// clock
    pub audio_mck: c_ulong,
    pub audio_clk_1: c_ulong,
    pub audio_clk_2: c_ulong,
    pub /: *mut *mut bool lrckp_fsync_fall; / LR clock polarity (SSICR.LRCKP),
    pub /: *mut *mut bool bckp_rise; / Bit clock polarity (SSICR.BCKP),
    pub dma_rt: bool,
    struct {
    pub tx_active: bool,
    pub rx_active: bool,
    pub one_stream_triggered: bool,
    pub dup: },
// Full duplex communication support
    struct {
    pub rate: c_uint,
    pub channels: c_uint,
    pub sample_width: c_uint,
    pub sample_bits: c_uint,
    pub hw_params_cache: },
    pub 1]: snd_dmaengine_dai_dma_data dma_dais[SNDRV_PCM_STREAM_LAST +,
    pub 1]: *mut *mut dma_chan dmas[SNDRV_PCM_STREAM_LAST +,
}

#[no_mangle]
unsafe extern "C" fn rz_ssi_reg_writel(priv: *mut rz_ssi_priv, reg: c_uint, data: u32) {
    static void rz_ssi_reg_writel(struct rz_ssi_priv *priv, uint reg, u32 data)
    {
    writel(data, (priv.base + reg));
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_reg_readl(priv: *mut rz_ssi_priv, reg: c_uint) -> u32 {
    static u32 rz_ssi_reg_readl(struct rz_ssi_priv *priv, uint reg)
    {
    return readl(priv.base + reg);
    }
    static void rz_ssi_reg_mask_setl(struct rz_ssi_priv *priv, uint reg,
    u32 bclr, u32 bset)
    {
    u32 val;
    val = readl(priv.base + reg);
    val = (val & ~bclr) | bset;
    writel(val, (priv.base + reg));
    }
    static inline struct rz_ssi_stream *
    rz_ssi_stream_get(struct rz_ssi_priv *ssi, struct snd_pcm_substream *substream)
    {
    return (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) ? &ssi.playback : &ssi.capture;
    }
#[no_mangle]
pub unsafe extern "C" fn rz_ssi_is_dma_enabled(ssi: *mut rz_ssi_priv) -> bool {
    static inline bool rz_ssi_is_dma_enabled(struct rz_ssi_priv *ssi)
    {
    return !ssi.playback.transfer && !ssi.capture.transfer;
    }
    static void rz_ssi_set_substream(struct rz_ssi_stream *strm,
    struct snd_pcm_substream *substream)
    {
    struct rz_ssi_priv *ssi = strm.priv;
    guard(spinlock_irqsave)(&ssi.lock);
    strm.substream = substream;
    }
    static bool rz_ssi_stream_is_valid(struct rz_ssi_priv *ssi,
    struct rz_ssi_stream *strm)
    {
    guard(spinlock_irqsave)(&ssi.lock);
    return strm.substream && strm.substream.runtime;
    }
#[no_mangle]
pub unsafe extern "C" fn rz_ssi_is_stream_running(strm: *mut rz_ssi_stream) -> bool {
    static inline bool rz_ssi_is_stream_running(struct rz_ssi_stream *strm)
    {
    return strm.substream && strm.running;
    }
    static void rz_ssi_stream_init(struct rz_ssi_stream *strm,
    struct snd_pcm_substream *substream)
    {
    rz_ssi_set_substream(strm, substream);
    strm.period_counter = 0;
    strm.buffer_pos = 0;
    strm.oerr_num = 0;
    strm.uerr_num = 0;
    strm.running = 0;
// fifo init
    strm.fifo_sample_size = SSI_FIFO_DEPTH;
    }
    static void rz_ssi_stream_quit(struct rz_ssi_priv *ssi,
    struct rz_ssi_stream *strm)
    {
    struct device *dev = ssi.dev;
    rz_ssi_set_substream(strm, core::ptr::null_mut());
    if (strm.oerr_num > 0)
    dev_info(dev, "overrun = %d\n", strm.oerr_num);
    if (strm.uerr_num > 0)
    dev_info(dev, "underrun = %d\n", strm.uerr_num);
    }
    static int rz_ssi_clk_setup(struct rz_ssi_priv *ssi, struct snd_pcm_substream *substream,
    unsigned int rate, unsigned int channels)
    {
    static u8 ckdv[] = { 1,  2,  4,  8, 16, 32, 64, 128, 6, 12, 24, 48, 96 };
    unsigned int channel_bits = 32;	/* System Word Length */
    let mut bclk_rate: c_ulong = rate * channels * channel_bits;
    struct snd_dmaengine_dai_dma_data *dma_dai;
    unsigned int div;
    unsigned int i;
    let mut ssicr: u32 = 0;
    u32 clk_ckdv;
// Clear AUCKE so we can set MST
    rz_ssi_reg_writel(ssi, SSIFCR, 0);
// Continue to output LRCK pin even when idle
    rz_ssi_reg_writel(ssi, SSIOFR, SSIOFR_LRCONT);
    if (ssi.audio_clk_1 && ssi.audio_clk_2) {
    if (ssi.audio_clk_1 % bclk_rate)
    ssi.audio_mck = ssi.audio_clk_2;
    else
    ssi.audio_mck = ssi.audio_clk_1;
    }
// Clock setting
    ssicr |= SSICR_MST;
    if (ssi.audio_mck == ssi.audio_clk_1)
    ssicr |= SSICR_CKS;
    if (ssi.bckp_rise)
    ssicr |= SSICR_BCKP;
    if (ssi.lrckp_fsync_fall)
    ssicr |= SSICR_LRCKP;
// Determine the clock divider
    clk_ckdv = 0;
    div = ssi.audio_mck / bclk_rate;
// try to find an match
    for (i = 0; i < ARRAY_SIZE(ckdv); i++) {
    if (ckdv[i] == div) {
    clk_ckdv = i;
    break;
    }
    }
    if (i == ARRAY_SIZE(ckdv)) {
    dev_err(ssi.dev, "Rate not divisible by audio clock source\n");
    return -EINVAL;
    }
    dma_dai = &ssi.dma_dais[substream.stream];
//
// DWL: Data Word Length = {16, 24, 32} bits
// SWL: System Word Length = 32 bits
//
    ssicr |= SSICR_CKDV(clk_ckdv);
    switch (ssi.hw_params_cache.sample_width) {
    case 16:
    ssicr |= SSICR_DWL(1);
    dma_dai.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case 24:
    ssicr |= SSICR_DWL(5) | SSICR_PDTA;
    dma_dai.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    case 32:
    ssicr |= SSICR_DWL(6);
    dma_dai.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    default:
    dev_err(ssi.dev, "Not support %u data width",
    ssi.hw_params_cache.sample_width);
    return -EINVAL;
    }
    ssicr |= SSICR_SWL(3);
    rz_ssi_reg_writel(ssi, SSICR, ssicr);
    rz_ssi_reg_writel(ssi, SSIFCR, SSIFCR_AUCKE | SSIFCR_FIFO_RST);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_set_idle(ssi: *mut rz_ssi_priv) {
    static void rz_ssi_set_idle(struct rz_ssi_priv *ssi)
    {
    u32 tmp;
    int ret;
// Disable irqs
    rz_ssi_reg_mask_setl(ssi, SSICR, SSICR_TUIEN | SSICR_TOIEN |
    SSICR_RUIEN | SSICR_ROIEN, 0);
    rz_ssi_reg_mask_setl(ssi, SSIFCR, SSIFCR_TIE | SSIFCR_RIE, 0);
// Clear all error flags
    rz_ssi_reg_mask_setl(ssi, SSISR,
    (SSISR_TOIRQ | SSISR_TUIRQ | SSISR_ROIRQ |
    SSISR_RUIRQ), 0);
// Wait for idle
    ret = readl_poll_timeout_atomic(ssi.base + SSISR, tmp, (tmp & SSISR_IIRQ), 1, 100);
    if (ret)
    dev_warn_ratelimited(ssi.dev, "timeout waiting for SSI idle\n");
// Hold FIFOs in reset
    rz_ssi_reg_mask_setl(ssi, SSIFCR, 0, SSIFCR_FIFO_RST);
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_start(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    static int rz_ssi_start(struct rz_ssi_priv *ssi, struct rz_ssi_stream *strm)
    {
    let mut is_play: bool = strm.substream.stream == SNDRV_PCM_STREAM_PLAYBACK;
    bool is_full_duplex;
    u32 ssicr, ssifcr;
    is_full_duplex = ssi.dup.tx_active && ssi.dup.rx_active;
    ssicr = rz_ssi_reg_readl(ssi, SSICR);
    ssifcr = rz_ssi_reg_readl(ssi, SSIFCR);
    if (!is_full_duplex) {
    ssifcr &= ~0xF;
    } else if (ssi.dup.one_stream_triggered) {
    rz_ssi_reg_mask_setl(ssi, SSICR, SSICR_TEN | SSICR_REN, 0);
    rz_ssi_set_idle(ssi);
    ssifcr &= ~SSIFCR_FIFO_RST;
    }
// FIFO interrupt thresholds
    if (rz_ssi_is_dma_enabled(ssi))
    rz_ssi_reg_writel(ssi, SSISCR, 0);
    else
    rz_ssi_reg_writel(ssi, SSISCR,
    SSISCR_TDES(strm.fifo_sample_size / 2 - 1) |
    SSISCR_RDFS(0));
// enable IRQ
    if (is_play) {
    ssicr |= SSICR_TUIEN | SSICR_TOIEN;
    ssifcr |= SSIFCR_TIE;
    if (!is_full_duplex)
    ssifcr |= SSIFCR_RFRST;
    } else {
    ssicr |= SSICR_RUIEN | SSICR_ROIEN;
    ssifcr |= SSIFCR_RIE;
    if (!is_full_duplex)
    ssifcr |= SSIFCR_TFRST;
    }
    rz_ssi_reg_writel(ssi, SSICR, ssicr);
    rz_ssi_reg_writel(ssi, SSIFCR, ssifcr);
// Clear all error flags
    rz_ssi_reg_mask_setl(ssi, SSISR,
    (SSISR_TOIRQ | SSISR_TUIRQ | SSISR_ROIRQ |
    SSISR_RUIRQ), 0);
    strm.running = 1;
    if (!is_full_duplex) {
    ssicr |= is_play ? SSICR_TEN : SSICR_REN;
    rz_ssi_reg_writel(ssi, SSICR, ssicr);
    } else if (ssi.dup.one_stream_triggered) {
    ssicr |= SSICR_TEN | SSICR_REN;
    rz_ssi_reg_writel(ssi, SSICR, ssicr);
    ssi.dup.one_stream_triggered = false;
    } else {
    ssi.dup.one_stream_triggered = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_swreset(ssi: *mut rz_ssi_priv) -> c_int {
    static int rz_ssi_swreset(struct rz_ssi_priv *ssi)
    {
    u32 tmp;
    rz_ssi_reg_mask_setl(ssi, SSIFCR, 0, SSIFCR_SSIRST);
    rz_ssi_reg_mask_setl(ssi, SSIFCR, SSIFCR_SSIRST, 0);
    return readl_poll_timeout_atomic(ssi.base + SSIFCR, tmp, !(tmp & SSIFCR_SSIRST), 1, 5);
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_stop(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    static int rz_ssi_stop(struct rz_ssi_priv *ssi, struct rz_ssi_stream *strm)
    {
    strm.running = 0;
    if (rz_ssi_is_stream_running(&ssi.playback) ||
    rz_ssi_is_stream_running(&ssi.capture))
    return 0;
// Disable TX/RX
    rz_ssi_reg_mask_setl(ssi, SSICR, SSICR_TEN | SSICR_REN, 0);
    rz_ssi_set_idle(ssi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_pointer_update(strm: *mut rz_ssi_stream, frames: c_int) {
    static void rz_ssi_pointer_update(struct rz_ssi_stream *strm, int frames)
    {
    struct snd_pcm_substream *substream = strm.substream;
    struct snd_pcm_runtime *runtime;
    int current_period;
    if (!strm.running || !substream || !substream.runtime)
    return;
    runtime = substream.runtime;
    strm.buffer_pos += frames;
    WARN_ON(strm.buffer_pos > runtime.buffer_size);
// ring buffer
    if (strm.buffer_pos == runtime.buffer_size)
    strm.buffer_pos = 0;
    current_period = strm.buffer_pos / runtime.period_size;
    if (strm.period_counter != current_period) {
    snd_pcm_period_elapsed(strm.substream);
    strm.period_counter = current_period;
    }
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_pio_recv(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    static int rz_ssi_pio_recv(struct rz_ssi_priv *ssi, struct rz_ssi_stream *strm)
    {
    struct snd_pcm_substream *substream = strm.substream;
    struct snd_pcm_runtime *runtime;
    int fifo_samples;
    int frames_left;
    int samples;
    int i;
    if (!rz_ssi_stream_is_valid(ssi, strm))
    return -EINVAL;
    runtime = substream.runtime;
    do {
// frames left in this period
    frames_left = runtime.period_size -
    (strm.buffer_pos % runtime.period_size);
    if (!frames_left)
    frames_left = runtime.period_size;
// Samples in RX FIFO
    fifo_samples = (rz_ssi_reg_readl(ssi, SSIFSR) >>
    SSIFSR_RDC_SHIFT) & SSIFSR_RDC_MASK;
// Only read full frames at a time
    samples = 0;
    while (frames_left && (fifo_samples >= runtime.channels)) {
    samples += runtime.channels;
    fifo_samples -= runtime.channels;
    frames_left--;
    }
// not enough samples yet
    if (!samples)
    break;
// calculate new buffer index
    if (ssi.hw_params_cache.sample_width == 16) {
    u16 *buf;
    buf = (u16 *)runtime.dma_area;
    buf += strm.buffer_pos * runtime.channels;
    for (i = 0; i < samples; i++)
// buf++ = (u16)(rz_ssi_reg_readl(ssi, SSIFRDR) >> 16);
    } else {
    u32 *buf;
    buf = (u32 *)runtime.dma_area;
    buf += strm.buffer_pos * runtime.channels;
    for (i = 0; i < samples; i++)
// buf++ = rz_ssi_reg_readl(ssi, SSIFRDR);
    }
    rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_RDF, 0);
    rz_ssi_pointer_update(strm, samples / runtime.channels);
    } while (!frames_left && fifo_samples >= runtime.channels);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_pio_send(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    static int rz_ssi_pio_send(struct rz_ssi_priv *ssi, struct rz_ssi_stream *strm)
    {
    struct snd_pcm_substream *substream = strm.substream;
    struct snd_pcm_runtime *runtime = substream.runtime;
    int sample_space;
    let mut samples: c_int = 0;
    int frames_left;
    int i;
    u32 ssifsr;
    if (!rz_ssi_stream_is_valid(ssi, strm))
    return -EINVAL;
// frames left in this period
    frames_left = runtime.period_size - (strm.buffer_pos %
    runtime.period_size);
    if (frames_left == 0)
    frames_left = runtime.period_size;
    sample_space = strm.fifo_sample_size;
    ssifsr = rz_ssi_reg_readl(ssi, SSIFSR);
    sample_space -= (ssifsr >> SSIFSR_TDC_SHIFT) & SSIFSR_TDC_MASK;
    if (sample_space < 0)
    return -EINVAL;
// Only add full frames at a time
    while (frames_left && (sample_space >= runtime.channels)) {
    samples += runtime.channels;
    sample_space -= runtime.channels;
    frames_left--;
    }
// no space to send anything right now
    if (samples == 0)
    return 0;
// calculate new buffer index
    if (ssi.hw_params_cache.sample_width == 16) {
    u16 *buf;
    buf = (u16 *)(runtime.dma_area);
    buf += strm.buffer_pos * runtime.channels;
    for (i = 0; i < samples; i++)
    rz_ssi_reg_writel(ssi, SSIFTDR, ((u32)(*buf++) << 16));
    } else {
    u32 *buf;
    buf = (u32 *)(runtime.dma_area);
    buf += strm.buffer_pos * runtime.channels;
    for (i = 0; i < samples; i++)
    rz_ssi_reg_writel(ssi, SSIFTDR, *buf++);
    }
    rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_TDE, 0);
    rz_ssi_pointer_update(strm, samples / runtime.channels);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rz_ssi_interrupt(int irq, void *data)
    {
    struct rz_ssi_stream *strm_playback = core::ptr::null_mut();
    struct rz_ssi_stream *strm_capture = core::ptr::null_mut();
    struct rz_ssi_priv *ssi = data;
    let mut ssisr: u32 = rz_ssi_reg_readl(ssi, SSISR);
    if (ssi.playback.substream)
    strm_playback = &ssi.playback;
    if (ssi.capture.substream)
    strm_capture = &ssi.capture;
    if (!strm_playback && !strm_capture)
    return IRQ_HANDLED; /* Left over TX/RX interrupt */
    if (irq == ssi.irq_int) { /* error or idle */
    bool is_stopped = !!(ssisr & (SSISR_RUIRQ | SSISR_ROIRQ |
    SSISR_TUIRQ | SSISR_TOIRQ));
    if (ssi.capture.substream && is_stopped) {
    if (ssisr & SSISR_RUIRQ)
    strm_capture.uerr_num++;
    if (ssisr & SSISR_ROIRQ)
    strm_capture.oerr_num++;
    rz_ssi_stop(ssi, strm_capture);
    }
    if (ssi.playback.substream && is_stopped) {
    if (ssisr & SSISR_TUIRQ)
    strm_playback.uerr_num++;
    if (ssisr & SSISR_TOIRQ)
    strm_playback.oerr_num++;
    rz_ssi_stop(ssi, strm_playback);
    }
    if (!rz_ssi_is_stream_running(&ssi.playback) &&
    !rz_ssi_is_stream_running(&ssi.capture) &&
    rz_ssi_is_dma_enabled(ssi) && is_stopped) {
    if (ssi.playback.substream &&
    ssi.dmas[SNDRV_PCM_STREAM_PLAYBACK])
    dmaengine_pause(ssi.dmas[SNDRV_PCM_STREAM_PLAYBACK]);
    if (ssi.capture.substream &&
    ssi.dmas[SNDRV_PCM_STREAM_CAPTURE] &&
// Avoid calling pause twice in case of half duplex.
    ssi.dmas[SNDRV_PCM_STREAM_PLAYBACK] !=
    ssi.dmas[SNDRV_PCM_STREAM_CAPTURE])
    dmaengine_pause(ssi.dmas[SNDRV_PCM_STREAM_CAPTURE]);
    }
// Clear all flags
    rz_ssi_reg_mask_setl(ssi, SSISR, SSISR_TOIRQ | SSISR_TUIRQ |
    SSISR_ROIRQ | SSISR_RUIRQ, 0);
// Add/remove more data
    if (ssi.capture.substream && is_stopped) {
    if (rz_ssi_is_dma_enabled(ssi)) {
    if (ssi.dmas[SNDRV_PCM_STREAM_CAPTURE])
    dmaengine_resume(ssi.dmas[SNDRV_PCM_STREAM_CAPTURE]);
    } else {
    strm_capture.transfer(ssi, strm_capture);
    }
    }
    if (ssi.playback.substream && is_stopped) {
    if (rz_ssi_is_dma_enabled(ssi)) {
    if (ssi.dmas[SNDRV_PCM_STREAM_PLAYBACK])
    dmaengine_resume(ssi.dmas[SNDRV_PCM_STREAM_PLAYBACK]);
    } else {
    strm_playback.transfer(ssi, strm_playback);
    }
    }
// Resume
    if (ssi.playback.substream && is_stopped)
    rz_ssi_start(ssi, &ssi.playback);
    if (ssi.capture.substream && is_stopped)
    rz_ssi_start(ssi, &ssi.capture);
    }
    if (!rz_ssi_is_stream_running(&ssi.playback) &&
    !rz_ssi_is_stream_running(&ssi.capture))
    return IRQ_HANDLED;
// tx data empty
    if (irq == ssi.irq_tx && rz_ssi_is_stream_running(&ssi.playback))
    strm_playback.transfer(ssi, &ssi.playback);
// rx data full
    if (irq == ssi.irq_rx && rz_ssi_is_stream_running(&ssi.capture)) {
    strm_capture.transfer(ssi, &ssi.capture);
    rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_RDF, 0);
    }
    if (irq == ssi.irq_rt) {
    if (ssi.playback.substream) {
    strm_playback.transfer(ssi, &ssi.playback);
    } else {
    strm_capture.transfer(ssi, &ssi.capture);
    rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_RDF, 0);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_trigger_resume(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    static int rz_ssi_trigger_resume(struct rz_ssi_priv *ssi, struct rz_ssi_stream *strm)
    {
    struct snd_pcm_substream *substream = strm.substream;
    int ret;
    if (rz_ssi_is_stream_running(&ssi.playback) ||
    rz_ssi_is_stream_running(&ssi.capture))
    return 0;
    ret = rz_ssi_swreset(ssi);
    if (ret)
    return ret;
    return rz_ssi_clk_setup(ssi, substream, ssi.hw_params_cache.rate,
    ssi.hw_params_cache.channels);
    }
    static int rz_ssi_dai_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    struct rz_ssi_stream *strm = rz_ssi_stream_get(ssi, substream);
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    ret = rz_ssi_trigger_resume(ssi, strm);
    if (ret)
    return ret;
    fallthrough;
    case SNDRV_PCM_TRIGGER_START:
    if (cmd == SNDRV_PCM_TRIGGER_START)
    rz_ssi_stream_init(strm, substream);
    if (!rz_ssi_is_dma_enabled(ssi)) {
    ret = strm.transfer(ssi, strm);
    if (ret)
    return ret;
    }
    ret = rz_ssi_start(ssi, strm);
    break;
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    rz_ssi_stop(ssi, strm);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    rz_ssi_stop(ssi, strm);
    rz_ssi_stream_quit(ssi, strm);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int rz_ssi_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_BP_FP:
    break;
    default:
    dev_err(ssi.dev, "Codec should be clk and frame consumer\n");
    return -EINVAL;
    }
//
// set clock polarity
//
// "normal" BCLK = Signal is available at rising edge of BCLK
// "normal" FSYNC = (I2S) Left ch starts with falling FSYNC edge
//
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    ssi.bckp_rise = false;
    ssi.lrckp_fsync_fall = false;
    break;
    case SND_SOC_DAIFMT_NB_IF:
    ssi.bckp_rise = false;
    ssi.lrckp_fsync_fall = true;
    break;
    case SND_SOC_DAIFMT_IB_NF:
    ssi.bckp_rise = true;
    ssi.lrckp_fsync_fall = false;
    break;
    case SND_SOC_DAIFMT_IB_IF:
    ssi.bckp_rise = true;
    ssi.lrckp_fsync_fall = true;
    break;
    default:
    return -EINVAL;
    }
// only i2s support
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    break;
    default:
    dev_err(ssi.dev, "Only I2S mode is supported.\n");
    return -EINVAL;
    }
    return 0;
    }
    static int rz_ssi_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    ssi.dup.tx_active = true;
    else
    ssi.dup.rx_active = true;
    return 0;
    }
    static void rz_ssi_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    ssi.dup.tx_active = false;
    else
    ssi.dup.rx_active = false;
    ssi.dmas[substream.stream] = core::ptr::null_mut();
    }
    static bool rz_ssi_is_valid_hw_params(struct rz_ssi_priv *ssi, unsigned int rate,
    unsigned int channels,
    unsigned int sample_width,
    unsigned int sample_bits)
    {
    if (ssi.hw_params_cache.rate != rate ||
    ssi.hw_params_cache.channels != channels ||
    ssi.hw_params_cache.sample_width != sample_width ||
    ssi.hw_params_cache.sample_bits != sample_bits)
    return false;
    return true;
    }
    static void rz_ssi_cache_hw_params(struct rz_ssi_priv *ssi, unsigned int rate,
    unsigned int channels,
    unsigned int sample_width,
    unsigned int sample_bits)
    {
    ssi.hw_params_cache.rate = rate;
    ssi.hw_params_cache.channels = channels;
    ssi.hw_params_cache.sample_width = sample_width;
    ssi.hw_params_cache.sample_bits = sample_bits;
    }
    static int rz_ssi_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    unsigned int sample_bits = hw_param_interval(params,
    SNDRV_PCM_HW_PARAM_SAMPLE_BITS).min;
    let mut sample_width: c_uint = params_width(params);
    let mut channels: c_uint = params_channels(params);
    let mut rate: c_uint = params_rate(params);
    int ret;
    if (!(sample_bits == 16 || sample_bits == 24 || sample_bits == 32)) {
    dev_err(ssi.dev, "Unsupported sample width: %d\n",
    sample_bits);
    return -EINVAL;
    }
    if (channels != 2) {
    dev_err(ssi.dev, "Number of channels not matched: %d\n",
    channels);
    return -EINVAL;
    }
// Save the DMA channels for recovery.
    if (rz_ssi_is_dma_enabled(ssi))
    ssi.dmas[substream.stream] = snd_dmaengine_pcm_get_chan(substream);
    else
    ssi.dmas[substream.stream] = core::ptr::null_mut();
    if (rz_ssi_is_stream_running(&ssi.playback) ||
    rz_ssi_is_stream_running(&ssi.capture)) {
    if (rz_ssi_is_valid_hw_params(ssi, rate, channels, sample_width, sample_bits))
    return 0;
    dev_err(ssi.dev, "Full duplex needs same HW params\n");
    return -EINVAL;
    }
    rz_ssi_cache_hw_params(ssi, rate, channels, sample_width, sample_bits);
    ret = rz_ssi_swreset(ssi);
    if (ret)
    return ret;
    return rz_ssi_clk_setup(ssi, substream, rate, channels);
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int rz_ssi_dai_probe(struct snd_soc_dai *dai)
    {
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, &ssi.dma_dais[SNDRV_PCM_STREAM_PLAYBACK],
    &ssi.dma_dais[SNDRV_PCM_STREAM_CAPTURE]);
    return 0;
    }
    static const struct snd_soc_dai_ops rz_ssi_dai_ops = {
    .probe		= rz_ssi_dai_probe,
    .startup	= rz_ssi_startup,
    .shutdown	= rz_ssi_shutdown,
    .trigger	= rz_ssi_dai_trigger,
    .set_fmt	= rz_ssi_dai_set_fmt,
    .hw_params	= rz_ssi_dai_hw_params,
    };
    static const struct snd_pcm_hardware rz_ssi_pcm_hardware = {
    .info			= SNDRV_PCM_INFO_INTERLEAVED	|
    SNDRV_PCM_INFO_MMAP		|
    SNDRV_PCM_INFO_MMAP_VALID	|
    SNDRV_PCM_INFO_RESUME		|
    SNDRV_PCM_INFO_PAUSE,
    .buffer_bytes_max	= 192 * 1024,
    .period_bytes_min	= 32,
    .period_bytes_max	= 48 * 1024,
    .channels_min		= SSI_CHAN_MIN,
    .channels_max		= SSI_CHAN_MAX,
    .periods_min		= 1,
    .periods_max		= 32,
    .fifo_size		= 32 * 2,
    };
    static int rz_ssi_pcm_open_pio(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    snd_soc_set_runtime_hwparams(substream, &rz_ssi_pcm_hardware);
    return snd_pcm_hw_constraint_integer(substream.runtime,
    SNDRV_PCM_HW_PARAM_PERIODS);
    }
    static int rz_ssi_pcm_open_dma(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    return snd_pcm_hw_constraint_integer(substream.runtime,
    SNDRV_PCM_HW_PARAM_PERIODS);
    }
    static snd_pcm_uframes_t rz_ssi_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct rz_ssi_priv *ssi = snd_soc_dai_get_drvdata(dai);
    struct rz_ssi_stream *strm = rz_ssi_stream_get(ssi, substream);
    return strm.buffer_pos;
    }
    static int rz_ssi_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    snd_pcm_set_managed_buffer_all(rtd.pcm, SNDRV_DMA_TYPE_DEV,
    rtd.card.snd_card.dev,
    rz_ssi_pcm_hardware.buffer_bytes_max,
    rz_ssi_pcm_hardware.buffer_bytes_max);
    return 0;
    }
    static struct snd_soc_dai_driver rz_ssi_soc_dai[] = {
    {
    .name			= "rz-ssi-dai",
    .playback = {
    .rates		= SSI_RATES,
    .formats	= SSI_FMTS,
    .channels_min	= SSI_CHAN_MIN,
    .channels_max	= SSI_CHAN_MAX,
    },
    .capture = {
    .rates		= SSI_RATES,
    .formats	= SSI_FMTS,
    .channels_min	= SSI_CHAN_MIN,
    .channels_max	= SSI_CHAN_MAX,
    },
    .ops = &rz_ssi_dai_ops,
    },
    };
    static const struct snd_soc_component_driver rz_ssi_soc_component_pio = {
    .name			= "rz-ssi",
    .open			= rz_ssi_pcm_open_pio,
    .pointer		= rz_ssi_pcm_pointer,
    .pcm_new		= rz_ssi_pcm_new,
    .legacy_dai_naming	= 1,
    };
    static const struct snd_soc_component_driver rz_ssi_soc_component_dma = {
    .name			= "rz-ssi",
    .open			= rz_ssi_pcm_open_dma,
    .legacy_dai_naming	= 1,
    };
    static const struct snd_dmaengine_pcm_config rz_ssi_dmaengine_pcm_conf = {
    .pcm_hardware		= &rz_ssi_pcm_hardware,
    .prealloc_buffer_size	= 192 * 1024,
    .prepare_slave_config	= snd_dmaengine_pcm_prepare_slave_config,
    };
#[no_mangle]
unsafe extern "C" fn rz_ssi_probe(pdev: *mut platform_device) -> c_int {
    static int rz_ssi_probe(struct platform_device *pdev)
    {
    const struct snd_soc_component_driver *component_driver;
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct rz_ssi_priv *ssi;
    struct clk *audio_clk;
    struct resource *res;
    int ret;
    ssi = devm_kzalloc(dev, sizeof(*ssi), GFP_KERNEL);
    if (!ssi)
    return -ENOMEM;
    ssi.dev = dev;
    ssi.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(ssi.base))
    return PTR_ERR(ssi.base);
    ssi.clk = devm_clk_get(dev, "ssi");
    if (IS_ERR(ssi.clk))
    return PTR_ERR(ssi.clk);
    ssi.sfr_clk = devm_clk_get(dev, "ssi_sfr");
    if (IS_ERR(ssi.sfr_clk))
    return PTR_ERR(ssi.sfr_clk);
    audio_clk = devm_clk_get(dev, "audio_clk1");
    if (IS_ERR(audio_clk))
    return dev_err_probe(dev, PTR_ERR(audio_clk), "no audio clk1");
    ssi.audio_clk_1 = clk_get_rate(audio_clk);
    audio_clk = devm_clk_get(dev, "audio_clk2");
    if (IS_ERR(audio_clk))
    return dev_err_probe(dev, PTR_ERR(audio_clk), "no audio clk2");
    ssi.audio_clk_2 = clk_get_rate(audio_clk);
    if (!(ssi.audio_clk_1 || ssi.audio_clk_2))
    return dev_err_probe(dev, -EINVAL, "no audio clk1 or audio clk2");
    ssi.audio_mck = ssi.audio_clk_1 ? ssi.audio_clk_1 : ssi.audio_clk_2;
    ssi.dma_dais[SNDRV_PCM_STREAM_PLAYBACK].addr = (dma_addr_t)res.start + SSIFTDR;
    ssi.dma_dais[SNDRV_PCM_STREAM_CAPTURE].addr =  (dma_addr_t)res.start + SSIFRDR;
    if (of_property_present(np, "dma-names")) {
    struct snd_dmaengine_pcm_config *config;
    let mut flags: c_uint = 0;
    config = devm_kzalloc(dev, sizeof(*config), GFP_KERNEL);
    if (!config)
    return -ENOMEM;
    config.pcm_hardware = rz_ssi_dmaengine_pcm_conf.pcm_hardware;
    config.prealloc_buffer_size = rz_ssi_dmaengine_pcm_conf.prealloc_buffer_size;
    config.prepare_slave_config = rz_ssi_dmaengine_pcm_conf.prepare_slave_config;
    if (of_property_match_string(np, "dma-names", "rt") == 0) {
    flags = SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX;
    config.chan_names[SNDRV_PCM_STREAM_PLAYBACK] = "rt";
    } else {
    config.chan_names[SNDRV_PCM_STREAM_PLAYBACK] = "tx";
    config.chan_names[SNDRV_PCM_STREAM_CAPTURE] = "rx";
    }
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev, config, flags);
    } else {
    ret = -ENODEV;
    }
    if (ret == -EPROBE_DEFER) {
    return ret;
    } else if (ret) {
    dev_warn(dev, "DMA not available, using PIO\n");
    ssi.playback.transfer = rz_ssi_pio_send;
    ssi.capture.transfer = rz_ssi_pio_recv;
    component_driver = &rz_ssi_soc_component_pio;
    } else {
    dev_info(dev, "DMA enabled\n");
    component_driver = &rz_ssi_soc_component_dma;
    }
    ssi.playback.priv = ssi;
    ssi.capture.priv = ssi;
    spin_lock_init(&ssi.lock);
    dev_set_drvdata(dev, ssi);
// Error Interrupt
    ssi.irq_int = platform_get_irq_byname(pdev, "int_req");
    if (ssi.irq_int < 0)
    return ssi.irq_int;
    ret = devm_request_irq(dev, ssi.irq_int, rz_ssi_interrupt,
    0, dev_name(dev), ssi);
    if (ret < 0)
    return dev_err_probe(dev, ret, "irq request error (int_req)\n");
    if (!rz_ssi_is_dma_enabled(ssi)) {
// Tx and Rx interrupts (pio only)
    ssi.irq_tx = platform_get_irq_byname(pdev, "dma_tx");
    ssi.irq_rx = platform_get_irq_byname(pdev, "dma_rx");
    if (ssi.irq_tx == -ENXIO && ssi.irq_rx == -ENXIO) {
    ssi.irq_rt = platform_get_irq_byname(pdev, "dma_rt");
    if (ssi.irq_rt < 0)
    return ssi.irq_rt;
    ret = devm_request_irq(dev, ssi.irq_rt,
    rz_ssi_interrupt, 0,
    dev_name(dev), ssi);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "irq request error (dma_rt)\n");
    } else {
    if (ssi.irq_tx < 0)
    return ssi.irq_tx;
    if (ssi.irq_rx < 0)
    return ssi.irq_rx;
    ret = devm_request_irq(dev, ssi.irq_tx,
    rz_ssi_interrupt, 0,
    dev_name(dev), ssi);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "irq request error (dma_tx)\n");
    ret = devm_request_irq(dev, ssi.irq_rx,
    rz_ssi_interrupt, 0,
    dev_name(dev), ssi);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "irq request error (dma_rx)\n");
    }
    }
    ssi.rstc = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(ssi.rstc))
    return dev_err_probe(dev, PTR_ERR(ssi.rstc), "Failed to get reset\n");
// Default 0 for power saving. Can be overridden via sysfs.
    pm_runtime_set_autosuspend_delay(dev, 0);
    pm_runtime_use_autosuspend(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to enable runtime PM!\n");
    return devm_snd_soc_register_component(dev, component_driver,
    rz_ssi_soc_dai,
    ARRAY_SIZE(rz_ssi_soc_dai));
    }
    static const struct of_device_id rz_ssi_of_match[] = {
    { .compatible = "renesas,rz-ssi", },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rz_ssi_of_match);
#[no_mangle]
unsafe extern "C" fn rz_ssi_runtime_suspend(dev: *mut device) -> c_int {
    static int rz_ssi_runtime_suspend(struct device *dev)
    {
    struct rz_ssi_priv *ssi = dev_get_drvdata(dev);
    return reset_control_assert(ssi.rstc);
    }
#[no_mangle]
unsafe extern "C" fn rz_ssi_runtime_resume(dev: *mut device) -> c_int {
    static int rz_ssi_runtime_resume(struct device *dev)
    {
    struct rz_ssi_priv *ssi = dev_get_drvdata(dev);
    return reset_control_deassert(ssi.rstc);
    }
    static const struct dev_pm_ops rz_ssi_pm_ops = {
    RUNTIME_PM_OPS(rz_ssi_runtime_suspend, rz_ssi_runtime_resume, core::ptr::null_mut())
    NOIRQ_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    };
    static struct platform_driver rz_ssi_driver = {
    .driver	= {
    .name	= "rz-ssi-pcm-audio",
    .of_match_table = rz_ssi_of_match,
    .pm = pm_ptr(&rz_ssi_pm_ops),
    },
    .probe		= rz_ssi_probe,
    };
    module_platform_driver(rz_ssi_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Renesas RZ/G2L ASoC Serial Sound Interface Driver");
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
