//! Automatically rewritten from C to Rust
//! Source: sound/soc/bcm/cygnus-pcm.c
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
// Copyright (C) 2014-2015 Broadcom Corporation

// Register offset needed for ASoC PCM module
pub const INTH_R5F_STATUS_OFFSET: c_uint = 0x040;
pub const INTH_R5F_CLEAR_OFFSET: c_uint = 0x048;
pub const INTH_R5F_MASK_SET_OFFSET: c_uint = 0x050;
pub const INTH_R5F_MASK_CLEAR_OFFSET: c_uint = 0x054;
pub const BF_REARM_FREE_MARK_OFFSET: c_uint = 0x344;
pub const BF_REARM_FULL_MARK_OFFSET: c_uint = 0x348;
// Ring Buffer Ctrl Regs --- Start
// AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_RDADDR_REG_BASE
pub const SRC_RBUF_0_RDADDR_OFFSET: c_uint = 0x500;
pub const SRC_RBUF_1_RDADDR_OFFSET: c_uint = 0x518;
pub const SRC_RBUF_2_RDADDR_OFFSET: c_uint = 0x530;
pub const SRC_RBUF_3_RDADDR_OFFSET: c_uint = 0x548;
pub const SRC_RBUF_4_RDADDR_OFFSET: c_uint = 0x560;
pub const SRC_RBUF_5_RDADDR_OFFSET: c_uint = 0x578;
pub const SRC_RBUF_6_RDADDR_OFFSET: c_uint = 0x590;
// AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_WRADDR_REG_BASE
pub const SRC_RBUF_0_WRADDR_OFFSET: c_uint = 0x504;
pub const SRC_RBUF_1_WRADDR_OFFSET: c_uint = 0x51c;
pub const SRC_RBUF_2_WRADDR_OFFSET: c_uint = 0x534;
pub const SRC_RBUF_3_WRADDR_OFFSET: c_uint = 0x54c;
pub const SRC_RBUF_4_WRADDR_OFFSET: c_uint = 0x564;
pub const SRC_RBUF_5_WRADDR_OFFSET: c_uint = 0x57c;
pub const SRC_RBUF_6_WRADDR_OFFSET: c_uint = 0x594;
// AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_BASEADDR_REG_BASE
pub const SRC_RBUF_0_BASEADDR_OFFSET: c_uint = 0x508;
pub const SRC_RBUF_1_BASEADDR_OFFSET: c_uint = 0x520;
pub const SRC_RBUF_2_BASEADDR_OFFSET: c_uint = 0x538;
pub const SRC_RBUF_3_BASEADDR_OFFSET: c_uint = 0x550;
pub const SRC_RBUF_4_BASEADDR_OFFSET: c_uint = 0x568;
pub const SRC_RBUF_5_BASEADDR_OFFSET: c_uint = 0x580;
pub const SRC_RBUF_6_BASEADDR_OFFSET: c_uint = 0x598;
// AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_ENDADDR_REG_BASE
pub const SRC_RBUF_0_ENDADDR_OFFSET: c_uint = 0x50c;
pub const SRC_RBUF_1_ENDADDR_OFFSET: c_uint = 0x524;
pub const SRC_RBUF_2_ENDADDR_OFFSET: c_uint = 0x53c;
pub const SRC_RBUF_3_ENDADDR_OFFSET: c_uint = 0x554;
pub const SRC_RBUF_4_ENDADDR_OFFSET: c_uint = 0x56c;
pub const SRC_RBUF_5_ENDADDR_OFFSET: c_uint = 0x584;
pub const SRC_RBUF_6_ENDADDR_OFFSET: c_uint = 0x59c;
// AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_FREE_MARK_REG_BASE
pub const SRC_RBUF_0_FREE_MARK_OFFSET: c_uint = 0x510;
pub const SRC_RBUF_1_FREE_MARK_OFFSET: c_uint = 0x528;
pub const SRC_RBUF_2_FREE_MARK_OFFSET: c_uint = 0x540;
pub const SRC_RBUF_3_FREE_MARK_OFFSET: c_uint = 0x558;
pub const SRC_RBUF_4_FREE_MARK_OFFSET: c_uint = 0x570;
pub const SRC_RBUF_5_FREE_MARK_OFFSET: c_uint = 0x588;
pub const SRC_RBUF_6_FREE_MARK_OFFSET: c_uint = 0x5a0;
// AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_RDADDR_REG_BASE
pub const DST_RBUF_0_RDADDR_OFFSET: c_uint = 0x5c0;
pub const DST_RBUF_1_RDADDR_OFFSET: c_uint = 0x5d8;
pub const DST_RBUF_2_RDADDR_OFFSET: c_uint = 0x5f0;
pub const DST_RBUF_3_RDADDR_OFFSET: c_uint = 0x608;
pub const DST_RBUF_4_RDADDR_OFFSET: c_uint = 0x620;
pub const DST_RBUF_5_RDADDR_OFFSET: c_uint = 0x638;
// AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_WRADDR_REG_BASE
pub const DST_RBUF_0_WRADDR_OFFSET: c_uint = 0x5c4;
pub const DST_RBUF_1_WRADDR_OFFSET: c_uint = 0x5dc;
pub const DST_RBUF_2_WRADDR_OFFSET: c_uint = 0x5f4;
pub const DST_RBUF_3_WRADDR_OFFSET: c_uint = 0x60c;
pub const DST_RBUF_4_WRADDR_OFFSET: c_uint = 0x624;
pub const DST_RBUF_5_WRADDR_OFFSET: c_uint = 0x63c;
// AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_BASEADDR_REG_BASE
pub const DST_RBUF_0_BASEADDR_OFFSET: c_uint = 0x5c8;
pub const DST_RBUF_1_BASEADDR_OFFSET: c_uint = 0x5e0;
pub const DST_RBUF_2_BASEADDR_OFFSET: c_uint = 0x5f8;
pub const DST_RBUF_3_BASEADDR_OFFSET: c_uint = 0x610;
pub const DST_RBUF_4_BASEADDR_OFFSET: c_uint = 0x628;
pub const DST_RBUF_5_BASEADDR_OFFSET: c_uint = 0x640;
// AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_ENDADDR_REG_BASE
pub const DST_RBUF_0_ENDADDR_OFFSET: c_uint = 0x5cc;
pub const DST_RBUF_1_ENDADDR_OFFSET: c_uint = 0x5e4;
pub const DST_RBUF_2_ENDADDR_OFFSET: c_uint = 0x5fc;
pub const DST_RBUF_3_ENDADDR_OFFSET: c_uint = 0x614;
pub const DST_RBUF_4_ENDADDR_OFFSET: c_uint = 0x62c;
pub const DST_RBUF_5_ENDADDR_OFFSET: c_uint = 0x644;
// AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_FULL_MARK_REG_BASE
pub const DST_RBUF_0_FULL_MARK_OFFSET: c_uint = 0x5d0;
pub const DST_RBUF_1_FULL_MARK_OFFSET: c_uint = 0x5e8;
pub const DST_RBUF_2_FULL_MARK_OFFSET: c_uint = 0x600;
pub const DST_RBUF_3_FULL_MARK_OFFSET: c_uint = 0x618;
pub const DST_RBUF_4_FULL_MARK_OFFSET: c_uint = 0x630;
pub const DST_RBUF_5_FULL_MARK_OFFSET: c_uint = 0x648;
// Ring Buffer Ctrl Regs --- End
// Error Status Regs --- Start
// AUD_FMM_BF_ESR_ESRX_STATUS_REG_BASE
pub const ESR0_STATUS_OFFSET: c_uint = 0x900;
pub const ESR1_STATUS_OFFSET: c_uint = 0x918;
pub const ESR2_STATUS_OFFSET: c_uint = 0x930;
pub const ESR3_STATUS_OFFSET: c_uint = 0x948;
pub const ESR4_STATUS_OFFSET: c_uint = 0x960;
// AUD_FMM_BF_ESR_ESRX_STATUS_CLEAR_REG_BASE
pub const ESR0_STATUS_CLR_OFFSET: c_uint = 0x908;
pub const ESR1_STATUS_CLR_OFFSET: c_uint = 0x920;
pub const ESR2_STATUS_CLR_OFFSET: c_uint = 0x938;
pub const ESR3_STATUS_CLR_OFFSET: c_uint = 0x950;
pub const ESR4_STATUS_CLR_OFFSET: c_uint = 0x968;
// AUD_FMM_BF_ESR_ESRX_MASK_REG_BASE
pub const ESR0_MASK_STATUS_OFFSET: c_uint = 0x90c;
pub const ESR1_MASK_STATUS_OFFSET: c_uint = 0x924;
pub const ESR2_MASK_STATUS_OFFSET: c_uint = 0x93c;
pub const ESR3_MASK_STATUS_OFFSET: c_uint = 0x954;
pub const ESR4_MASK_STATUS_OFFSET: c_uint = 0x96c;
// AUD_FMM_BF_ESR_ESRX_MASK_SET_REG_BASE
pub const ESR0_MASK_SET_OFFSET: c_uint = 0x910;
pub const ESR1_MASK_SET_OFFSET: c_uint = 0x928;
pub const ESR2_MASK_SET_OFFSET: c_uint = 0x940;
pub const ESR3_MASK_SET_OFFSET: c_uint = 0x958;
pub const ESR4_MASK_SET_OFFSET: c_uint = 0x970;
// AUD_FMM_BF_ESR_ESRX_MASK_CLEAR_REG_BASE
pub const ESR0_MASK_CLR_OFFSET: c_uint = 0x914;
pub const ESR1_MASK_CLR_OFFSET: c_uint = 0x92c;
pub const ESR2_MASK_CLR_OFFSET: c_uint = 0x944;
pub const ESR3_MASK_CLR_OFFSET: c_uint = 0x95c;
pub const ESR4_MASK_CLR_OFFSET: c_uint = 0x974;
// Error Status Regs --- End

// Mask for R5F register.  Set all relevant interrupt for playback handler

    BIT(R5F_ESR1_SHIFT) | \
    BIT(R5F_ESR3_SHIFT))
// Mask for R5F register.  Set all relevant interrupt for capture handler

//
// PERIOD_BYTES_MIN is the number of bytes to at which the interrupt will tick.
// This number should be a multiple of 256. Minimum value is 256
//
pub const PERIOD_BYTES_MIN: c_uint = 0x100;
    static const struct snd_pcm_hardware cygnus_pcm_hw = {
    .info = SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
// A period is basically an interrupt
    .period_bytes_min = PERIOD_BYTES_MIN,
    .period_bytes_max = 0x10000,
// period_min/max gives range of approx interrupts per buffer
    .periods_min = 2,
    .periods_max = 8,
//
// maximum buffer size in bytes = period_bytes_max * periods_max
// We allocate this amount of data for each enabled channel
//
    .buffer_bytes_max = 4 * 0x8000,
    };
    let mut cygnus_dma_dmamask: static u64 = DMA_BIT_MASK(32);
    static struct cygnus_aio_port *cygnus_dai_get_dma_data(
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *soc_runtime = snd_soc_substream_to_rtd(substream);
    return snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(soc_runtime, 0), substream);
    }
    static void ringbuf_set_initial(void __iomem *audio_io,
    struct ringbuf_regs *p_rbuf,
    bool is_playback,
    u32 start,
    u32 periodsize,
    u32 bufsize)
    {
    u32 initial_rd;
    u32 initial_wr;
    u32 end;
    u32 fmark_val; /* free or full mark */
    p_rbuf.period_bytes = periodsize;
    p_rbuf.buf_size = bufsize;
    if (is_playback) {
// Set the pointers to indicate full (flip uppermost bit)
    initial_rd = start;
    initial_wr = initial_rd ^ BIT(31);
    } else {
// Set the pointers to indicate empty
    initial_wr = start;
    initial_rd = initial_wr;
    }
    end = start + bufsize - 1;
//
// The interrupt will fire when free/full mark is *exceeded
// The fmark value must be multiple of PERIOD_BYTES_MIN so set fmark
// to be PERIOD_BYTES_MIN less than the period size.
//
    fmark_val = periodsize - PERIOD_BYTES_MIN;
    writel(start, audio_io + p_rbuf.baseaddr);
    writel(end, audio_io + p_rbuf.endaddr);
    writel(fmark_val, audio_io + p_rbuf.fmark);
    writel(initial_rd, audio_io + p_rbuf.rdaddr);
    writel(initial_wr, audio_io + p_rbuf.wraddr);
    }
#[no_mangle]
unsafe extern "C" fn configure_ringbuf_regs(substream: *mut snd_pcm_substream) -> c_int {
    static int configure_ringbuf_regs(struct snd_pcm_substream *substream)
    {
    struct cygnus_aio_port *aio;
    struct ringbuf_regs *p_rbuf;
    let mut status: c_int = 0;
    aio = cygnus_dai_get_dma_data(substream);
// Map the ssp portnum to a set of ring buffers.
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    p_rbuf = &aio.play_rb_regs;
    switch (aio.portnum) {
    case 0:
// p_rbuf = RINGBUF_REG_PLAYBACK(0);
    break;
    case 1:
// p_rbuf = RINGBUF_REG_PLAYBACK(2);
    break;
    case 2:
// p_rbuf = RINGBUF_REG_PLAYBACK(4);
    break;
    case 3: /* SPDIF */
// p_rbuf = RINGBUF_REG_PLAYBACK(6);
    break;
    default:
    status = -EINVAL;
    }
    } else {
    p_rbuf = &aio.capture_rb_regs;
    switch (aio.portnum) {
    case 0:
// p_rbuf = RINGBUF_REG_CAPTURE(0);
    break;
    case 1:
// p_rbuf = RINGBUF_REG_CAPTURE(2);
    break;
    case 2:
// p_rbuf = RINGBUF_REG_CAPTURE(4);
    break;
    default:
    status = -EINVAL;
    }
    }
    return status;
    }
    static struct ringbuf_regs *get_ringbuf(struct snd_pcm_substream *substream)
    {
    struct cygnus_aio_port *aio;
    struct ringbuf_regs *p_rbuf = core::ptr::null_mut();
    aio = cygnus_dai_get_dma_data(substream);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    p_rbuf = &aio.play_rb_regs;
    else
    p_rbuf = &aio.capture_rb_regs;
    return p_rbuf;
    }
#[no_mangle]
unsafe extern "C" fn enable_intr(substream: *mut snd_pcm_substream) {
    static void enable_intr(struct snd_pcm_substream *substream)
    {
    struct cygnus_aio_port *aio;
    u32 clear_mask;
    aio = cygnus_dai_get_dma_data(substream);
// The port number maps to the bit position to be cleared
    clear_mask = BIT(aio.portnum);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
// Clear interrupt status before enabling them
    writel(clear_mask, aio.cygaud.audio + ESR0_STATUS_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR1_STATUS_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR3_STATUS_CLR_OFFSET);
// Unmask the interrupts of the given port
    writel(clear_mask, aio.cygaud.audio + ESR0_MASK_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR1_MASK_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR3_MASK_CLR_OFFSET);
    writel(ANY_PLAYBACK_IRQ,
    aio.cygaud.audio + INTH_R5F_MASK_CLEAR_OFFSET);
    } else {
    writel(clear_mask, aio.cygaud.audio + ESR2_STATUS_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR4_STATUS_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR2_MASK_CLR_OFFSET);
    writel(clear_mask, aio.cygaud.audio + ESR4_MASK_CLR_OFFSET);
    writel(ANY_CAPTURE_IRQ,
    aio.cygaud.audio + INTH_R5F_MASK_CLEAR_OFFSET);
    }
    }
#[no_mangle]
unsafe extern "C" fn disable_intr(substream: *mut snd_pcm_substream) {
    static void disable_intr(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct cygnus_aio_port *aio;
    u32 set_mask;
    aio = cygnus_dai_get_dma_data(substream);
    dev_dbg(snd_soc_rtd_to_cpu(rtd, 0).dev, "%s on port %d\n", __func__, aio.portnum);
// The port number maps to the bit position to be set
    set_mask = BIT(aio.portnum);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
// Mask the interrupts of the given port
    writel(set_mask, aio.cygaud.audio + ESR0_MASK_SET_OFFSET);
    writel(set_mask, aio.cygaud.audio + ESR1_MASK_SET_OFFSET);
    writel(set_mask, aio.cygaud.audio + ESR3_MASK_SET_OFFSET);
    } else {
    writel(set_mask, aio.cygaud.audio + ESR2_MASK_SET_OFFSET);
    writel(set_mask, aio.cygaud.audio + ESR4_MASK_SET_OFFSET);
    }
    }
    static int cygnus_pcm_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    enable_intr(substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    disable_intr(substream);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cygnus_pcm_period_elapsed(substream: *mut snd_pcm_substream) {
    static void cygnus_pcm_period_elapsed(struct snd_pcm_substream *substream)
    {
    struct cygnus_aio_port *aio;
    struct ringbuf_regs *p_rbuf = core::ptr::null_mut();
    u32 regval;
    aio = cygnus_dai_get_dma_data(substream);
    p_rbuf = get_ringbuf(substream);
//
// If free/full mark interrupt occurs, provide timestamp
// to ALSA and update appropriate idx by period_bytes
//
    snd_pcm_period_elapsed(substream);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
// Set the ring buffer to full
    regval = readl(aio.cygaud.audio + p_rbuf.rdaddr);
    regval = regval ^ BIT(31);
    writel(regval, aio.cygaud.audio + p_rbuf.wraddr);
    } else {
// Set the ring buffer to empty
    regval = readl(aio.cygaud.audio + p_rbuf.wraddr);
    writel(regval, aio.cygaud.audio + p_rbuf.rdaddr);
    }
    }
//
// ESR0/1/3 status  Description
// 0x1	I2S0_out port caused interrupt
// 0x2	I2S1_out port caused interrupt
// 0x4	I2S2_out port caused interrupt
// 0x8	SPDIF_out port caused interrupt
//
#[no_mangle]
unsafe extern "C" fn handle_playback_irq(cygaud: *mut cygnus_audio) {
    static void handle_playback_irq(struct cygnus_audio *cygaud)
    {
    void __iomem *audio_io;
    u32 port;
    u32 esr_status0, esr_status1, esr_status3;
    audio_io = cygaud.audio;
//
// ESR status gets updates with/without interrupts enabled.
// So, check the ESR mask, which provides interrupt enable
// disable status and use it to determine which ESR status
// should be serviced.
//
    esr_status0 = readl(audio_io + ESR0_STATUS_OFFSET);
    esr_status0 &= ~readl(audio_io + ESR0_MASK_STATUS_OFFSET);
    esr_status1 = readl(audio_io + ESR1_STATUS_OFFSET);
    esr_status1 &= ~readl(audio_io + ESR1_MASK_STATUS_OFFSET);
    esr_status3 = readl(audio_io + ESR3_STATUS_OFFSET);
    esr_status3 &= ~readl(audio_io + ESR3_MASK_STATUS_OFFSET);
    for (port = 0; port < CYGNUS_MAX_PLAYBACK_PORTS; port++) {
    let mut esrmask: u32 = BIT(port);
//
// Ringbuffer or FIFO underflow
// If we get this interrupt then, it is also true that we have
// not yet responded to the freemark interrupt.
// Log a debug message.  The freemark handler below will
// handle getting everything going again.
//
    if ((esrmask & esr_status1) || (esrmask & esr_status0)) {
    dev_dbg(cygaud.dev,
    "Underrun: esr0=0x%x, esr1=0x%x esr3=0x%x\n",
    esr_status0, esr_status1, esr_status3);
    }
//
// Freemark is hit. This is the normal interrupt.
// In typical operation the read and write regs will be equal
//
    if (esrmask & esr_status3) {
    struct snd_pcm_substream *playstr;
    playstr = cygaud.portinfo[port].play_stream;
    cygnus_pcm_period_elapsed(playstr);
    }
    }
// Clear ESR interrupt
    writel(esr_status0, audio_io + ESR0_STATUS_CLR_OFFSET);
    writel(esr_status1, audio_io + ESR1_STATUS_CLR_OFFSET);
    writel(esr_status3, audio_io + ESR3_STATUS_CLR_OFFSET);
// Rearm freemark logic by writing 1 to the correct bit
    writel(esr_status3, audio_io + BF_REARM_FREE_MARK_OFFSET);
    }
//
// ESR2/4 status  Description
// 0x1	I2S0_in port caused interrupt
// 0x2	I2S1_in port caused interrupt
// 0x4	I2S2_in port caused interrupt
//
#[no_mangle]
unsafe extern "C" fn handle_capture_irq(cygaud: *mut cygnus_audio) {
    static void handle_capture_irq(struct cygnus_audio *cygaud)
    {
    void __iomem *audio_io;
    u32 port;
    u32 esr_status2, esr_status4;
    audio_io = cygaud.audio;
//
// ESR status gets updates with/without interrupts enabled.
// So, check the ESR mask, which provides interrupt enable
// disable status and use it to determine which ESR status
// should be serviced.
//
    esr_status2 = readl(audio_io + ESR2_STATUS_OFFSET);
    esr_status2 &= ~readl(audio_io + ESR2_MASK_STATUS_OFFSET);
    esr_status4 = readl(audio_io + ESR4_STATUS_OFFSET);
    esr_status4 &= ~readl(audio_io + ESR4_MASK_STATUS_OFFSET);
    for (port = 0; port < CYGNUS_MAX_CAPTURE_PORTS; port++) {
    let mut esrmask: u32 = BIT(port);
//
// Ringbuffer or FIFO overflow
// If we get this interrupt then, it is also true that we have
// not yet responded to the fullmark interrupt.
// Log a debug message.  The fullmark handler below will
// handle getting everything going again.
//
    if (esrmask & esr_status2)
    dev_dbg(cygaud.dev,
    "Overflow: esr2=0x%x\n", esr_status2);
    if (esrmask & esr_status4) {
    struct snd_pcm_substream *capstr;
    capstr = cygaud.portinfo[port].capture_stream;
    cygnus_pcm_period_elapsed(capstr);
    }
    }
    writel(esr_status2, audio_io + ESR2_STATUS_CLR_OFFSET);
    writel(esr_status4, audio_io + ESR4_STATUS_CLR_OFFSET);
// Rearm fullmark logic by writing 1 to the correct bit
    writel(esr_status4, audio_io + BF_REARM_FULL_MARK_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn cygnus_dma_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cygnus_dma_irq(int irq, void *data)
    {
    u32 r5_status;
    struct cygnus_audio *cygaud = data;
//
// R5 status bits	Description
// 0		ESR0 (playback FIFO interrupt)
// 1		ESR1 (playback rbuf interrupt)
// 2		ESR2 (capture rbuf interrupt)
// 3		ESR3 (Freemark play. interrupt)
// 4		ESR4 (Fullmark capt. interrupt)
//
    r5_status = readl(cygaud.audio + INTH_R5F_STATUS_OFFSET);
    if (!(r5_status & (ANY_PLAYBACK_IRQ | ANY_CAPTURE_IRQ)))
    return IRQ_NONE;
// If playback interrupt happened
    if (ANY_PLAYBACK_IRQ & r5_status) {
    handle_playback_irq(cygaud);
    writel(ANY_PLAYBACK_IRQ & r5_status,
    cygaud.audio + INTH_R5F_CLEAR_OFFSET);
    }
// If  capture interrupt happened
    if (ANY_CAPTURE_IRQ & r5_status) {
    handle_capture_irq(cygaud);
    writel(ANY_CAPTURE_IRQ & r5_status,
    cygaud.audio + INTH_R5F_CLEAR_OFFSET);
    }
    return IRQ_HANDLED;
    }
    static int cygnus_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cygnus_aio_port *aio;
    int ret;
    aio = cygnus_dai_get_dma_data(substream);
    if (!aio)
    return -ENODEV;
    dev_dbg(snd_soc_rtd_to_cpu(rtd, 0).dev, "%s port %d\n", __func__, aio.portnum);
    snd_soc_set_runtime_hwparams(substream, &cygnus_pcm_hw);
    ret = snd_pcm_hw_constraint_step(runtime, 0,
    SNDRV_PCM_HW_PARAM_PERIOD_BYTES, PERIOD_BYTES_MIN);
    if (ret < 0)
    return ret;
    ret = snd_pcm_hw_constraint_step(runtime, 0,
    SNDRV_PCM_HW_PARAM_BUFFER_BYTES, PERIOD_BYTES_MIN);
    if (ret < 0)
    return ret;
//
// Keep track of which substream belongs to which port.
// This info is needed by snd_pcm_period_elapsed() in irq_handler
//
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    aio.play_stream = substream;
    else
    aio.capture_stream = substream;
    return 0;
    }
    static int cygnus_pcm_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct cygnus_aio_port *aio;
    aio = cygnus_dai_get_dma_data(substream);
    dev_dbg(snd_soc_rtd_to_cpu(rtd, 0).dev, "%s  port %d\n", __func__, aio.portnum);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    aio.play_stream = core::ptr::null_mut();
    else
    aio.capture_stream = core::ptr::null_mut();
    if (!aio.play_stream && !aio.capture_stream)
    dev_dbg(snd_soc_rtd_to_cpu(rtd, 0).dev, "freed  port %d\n", aio.portnum);
    return 0;
    }
    static int cygnus_pcm_prepare(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cygnus_aio_port *aio;
    unsigned long bufsize, periodsize;
    bool is_play;
    u32 start;
    struct ringbuf_regs *p_rbuf = core::ptr::null_mut();
    aio = cygnus_dai_get_dma_data(substream);
    dev_dbg(snd_soc_rtd_to_cpu(rtd, 0).dev, "%s port %d\n", __func__, aio.portnum);
    bufsize = snd_pcm_lib_buffer_bytes(substream);
    periodsize = snd_pcm_lib_period_bytes(substream);
    dev_dbg(snd_soc_rtd_to_cpu(rtd, 0).dev, "%s (buf_size %lu) (period_size %lu)\n",
    __func__, bufsize, periodsize);
    configure_ringbuf_regs(substream);
    p_rbuf = get_ringbuf(substream);
    start = runtime.dma_addr;
    is_play = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) ? 1 : 0;
    ringbuf_set_initial(aio.cygaud.audio, p_rbuf, is_play, start,
    periodsize, bufsize);
    return 0;
    }
    static snd_pcm_uframes_t cygnus_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct cygnus_aio_port *aio;
    let mut res: c_uint = 0, cur = 0, base = 0;
    struct ringbuf_regs *p_rbuf = core::ptr::null_mut();
    aio = cygnus_dai_get_dma_data(substream);
//
// Get the offset of the current read (for playack) or write
// index (for capture).  Report this value back to the asoc framework.
//
    p_rbuf = get_ringbuf(substream);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    cur = readl(aio.cygaud.audio + p_rbuf.rdaddr);
    else
    cur = readl(aio.cygaud.audio + p_rbuf.wraddr);
    base = readl(aio.cygaud.audio + p_rbuf.baseaddr);
//
// Mask off the MSB of the rdaddr,wraddr and baseaddr
// since MSB is not part of the address
//
    res = (cur & 0x7fffffff) - (base & 0x7fffffff);
    return bytes_to_frames(substream.runtime, res);
    }
    static int cygnus_dma_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    let mut size: usize = cygnus_pcm_hw.buffer_bytes_max;
    struct snd_card *card = rtd.card.snd_card;
    if (!card.dev.dma_mask)
    card.dev.dma_mask = &cygnus_dma_dmamask;
    if (!card.dev.coherent_dma_mask)
    card.dev.coherent_dma_mask = DMA_BIT_MASK(32);
    snd_pcm_set_managed_buffer_all(rtd.pcm, SNDRV_DMA_TYPE_DEV,
    card.dev, size, size);
    return 0;
    }
    static const struct snd_soc_component_driver cygnus_soc_platform = {
    .open		= cygnus_pcm_open,
    .close		= cygnus_pcm_close,
    .prepare	= cygnus_pcm_prepare,
    .trigger	= cygnus_pcm_trigger,
    .pointer	= cygnus_pcm_pointer,
    .pcm_new	= cygnus_dma_new,
    };
    int cygnus_soc_platform_register(struct device *dev,
    struct cygnus_audio *cygaud)
    {
    int rc;
    dev_dbg(dev, "%s Enter\n", __func__);
    rc = devm_request_irq(dev, cygaud.irq_num, cygnus_dma_irq,
    IRQF_SHARED, "cygnus-audio", cygaud);
    if (rc) {
    dev_err(dev, "%s request_irq error %d\n", __func__, rc);
    return rc;
    }
    rc = devm_snd_soc_register_component(dev, &cygnus_soc_platform,
    core::ptr::null_mut(), 0);
    if (rc) {
    dev_err(dev, "%s failed\n", __func__);
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cygnus_soc_platform_unregister(dev: *mut device) -> c_int {
    int cygnus_soc_platform_unregister(struct device *dev)
    {
    return 0;
    }
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("Cygnus ASoC PCM module");
