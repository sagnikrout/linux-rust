//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/synopsys/dw-hdmi-ahb-audio.c
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
// DesignWare HDMI audio driver
//
// Written and tested against the Designware HDMI Tx found in iMX6.
//

// Provide some bits rather than bit offsets
    enum {
    HDMI_AHB_DMA_CONF0_SW_FIFO_RST = BIT(7),
    HDMI_AHB_DMA_CONF0_EN_HLOCK = BIT(3),
    HDMI_AHB_DMA_START_START = BIT(0),
    HDMI_AHB_DMA_STOP_STOP = BIT(0),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_ERROR = BIT(5),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_LOST = BIT(4),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_RETRY = BIT(3),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_DONE = BIT(2),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_BUFFFULL = BIT(1),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_BUFFEMPTY = BIT(0),
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_ALL =
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_ERROR |
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_LOST |
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_RETRY |
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_DONE |
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_BUFFFULL |
    HDMI_IH_MUTE_AHBDMAAUD_STAT0_BUFFEMPTY,
    HDMI_IH_AHBDMAAUD_STAT0_ERROR = BIT(5),
    HDMI_IH_AHBDMAAUD_STAT0_LOST = BIT(4),
    HDMI_IH_AHBDMAAUD_STAT0_RETRY = BIT(3),
    HDMI_IH_AHBDMAAUD_STAT0_DONE = BIT(2),
    HDMI_IH_AHBDMAAUD_STAT0_BUFFFULL = BIT(1),
    HDMI_IH_AHBDMAAUD_STAT0_BUFFEMPTY = BIT(0),
    HDMI_IH_AHBDMAAUD_STAT0_ALL =
    HDMI_IH_AHBDMAAUD_STAT0_ERROR |
    HDMI_IH_AHBDMAAUD_STAT0_LOST |
    HDMI_IH_AHBDMAAUD_STAT0_RETRY |
    HDMI_IH_AHBDMAAUD_STAT0_DONE |
    HDMI_IH_AHBDMAAUD_STAT0_BUFFFULL |
    HDMI_IH_AHBDMAAUD_STAT0_BUFFEMPTY,
    HDMI_AHB_DMA_CONF0_INCR16 = 2 << 1,
    HDMI_AHB_DMA_CONF0_INCR8 = 1 << 1,
    HDMI_AHB_DMA_CONF0_INCR4 = 0,
    HDMI_AHB_DMA_CONF0_BURST_MODE = BIT(0),
    HDMI_AHB_DMA_MASK_DONE = BIT(7),
    HDMI_REVISION_ID = 0x0001,
    HDMI_IH_AHBDMAAUD_STAT0 = 0x0109,
    HDMI_IH_MUTE_AHBDMAAUD_STAT0 = 0x0189,
    HDMI_AHB_DMA_CONF0 = 0x3600,
    HDMI_AHB_DMA_START = 0x3601,
    HDMI_AHB_DMA_STOP = 0x3602,
    HDMI_AHB_DMA_THRSLD = 0x3603,
    HDMI_AHB_DMA_STRADDR0 = 0x3604,
    HDMI_AHB_DMA_STPADDR0 = 0x3608,
    HDMI_AHB_DMA_MASK = 0x3614,
    HDMI_AHB_DMA_POL = 0x3615,
    HDMI_AHB_DMA_CONF1 = 0x3616,
    HDMI_AHB_DMA_BUFFPOL = 0x361a,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_channel_conf {
    pub conf1: u8,
    pub ca: u8,
}

//
// The default mapping of ALSA channels to HDMI channels and speaker
// allocation bits.  Note that we can't do channel remapping here -
// channels must be in the same order.
//
// Mappings for alsa-lib pcm/surround*.conf files:
//
// Front	Sur4.0	Sur4.1	Sur5.0	Sur5.1	Sur7.1
// Channels	2	4	6	6	6	8
//
// Our mapping from ALSA channel to CEA686D speaker name and HDMI channel:
//
// Number of ALSA channels
// ALSA Channel	2	3	4	5	6	7	8
// 0		FL:0	=	=	=	=	=	=
// 1		FR:1	=	=	=	=	=	=
// 2			FC:3	RL:4	LFE:2	=	=	=
// 3				RR:5	RL:4	FC:3	=	=
// 4					RR:5	RL:4	=	=
// 5						RR:5	=	=
// 6							RC:6	=
// 7							RLC/FRC	RLC/FRC
//
    static struct dw_hdmi_channel_conf default_hdmi_channel_config[7] = {
    { 0x03, 0x00 },	/* FL,FR */
    { 0x0b, 0x02 },	/* FL,FR,FC */
    { 0x33, 0x08 },	/* FL,FR,RL,RR */
    { 0x37, 0x09 },	/* FL,FR,LFE,RL,RR */
    { 0x3f, 0x0b },	/* FL,FR,LFE,FC,RL,RR */
    { 0x7f, 0x0f },	/* FL,FR,LFE,FC,RL,RR,RC */
    { 0xff, 0x13 },	/* FL,FR,LFE,FC,RL,RR,[FR]RC,[FR]LC */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dw_hdmi {
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub lock: spinlock_t,
    pub data: dw_hdmi_audio_data,
    pub substream: *mut snd_pcm_substream,
    pub size_t): *mut *mut *mut void (reformat)(struct snd_dw_hdmi , size_t,,
    pub buf_src: *mut c_void,
    pub buf_dst: *mut c_void,
    pub buf_addr: dma_addr_t,
    pub buf_offset: unsigned,
    pub buf_period: unsigned,
    pub buf_size: unsigned,
    pub channels: unsigned,
    pub revision: u8,
    pub iec_offset: u8,
    pub cs: [u8; 192][8],
}

#[no_mangle]
unsafe extern "C" fn dw_hdmi_writel(val: u32, ptr: *mut void __iomem) {
    static void dw_hdmi_writel(u32 val, void __iomem *ptr)
    {
    writeb_relaxed(val, ptr);
    writeb_relaxed(val >> 8, ptr + 1);
    writeb_relaxed(val >> 16, ptr + 2);
    writeb_relaxed(val >> 24, ptr + 3);
    }
//
// Convert to hardware format: The userspace buffer contains IEC958 samples,
// with the PCUV bits in bits 31..28 and audio samples in bits 27..4.  We
// need these to be in bits 27..24, with the IEC B bit in bit 28, and audio
// samples in 23..0.
//
// Default preamble in bits 3..0: 8 = block start, 4 = even 2 = odd
//
// Ideally, we could do with having the data properly formatted in userspace.
//
    static void dw_hdmi_reformat_iec958(struct snd_dw_hdmi *dw,
    size_t offset, size_t bytes)
    {
    u32 *src = dw.buf_src + offset;
    u32 *dst = dw.buf_dst + offset;
    u32 *end = dw.buf_src + offset + bytes;
    do {
    u32 b, sample = *src++;
    b = (sample & 8) << (28 - 3);
    sample >>= 4;
// dst++ = sample | b;
    } while (src < end);
    }
#[no_mangle]
unsafe extern "C" fn parity(sample: u32) -> u32 {
    static u32 parity(u32 sample)
    {
    sample ^= sample >> 16;
    sample ^= sample >> 8;
    sample ^= sample >> 4;
    sample ^= sample >> 2;
    sample ^= sample >> 1;
    return (sample & 1) << 27;
    }
    static void dw_hdmi_reformat_s24(struct snd_dw_hdmi *dw,
    size_t offset, size_t bytes)
    {
    u32 *src = dw.buf_src + offset;
    u32 *dst = dw.buf_dst + offset;
    u32 *end = dw.buf_src + offset + bytes;
    do {
    unsigned i;
    u8 *cs;
    cs = dw.cs[dw.iec_offset++];
    if (dw.iec_offset >= 192)
    dw.iec_offset = 0;
    i = dw.channels;
    do {
    let mut sample: u32 = *src++;
    sample &= ~0xff000000;
    sample |= *cs++ << 24;
    sample |= parity(sample & ~0xf8000000);
// dst++ = sample;
    } while (--i);
    } while (src < end);
    }
    static void dw_hdmi_create_cs(struct snd_dw_hdmi *dw,
    struct snd_pcm_runtime *runtime)
    {
    u8 cs[4];
    unsigned ch, i, j;
    snd_pcm_create_iec958_consumer(runtime, cs, sizeof(cs));
    memset(dw.cs, 0, sizeof(dw.cs));
    for (ch = 0; ch < 8; ch++) {
    cs[2] &= ~IEC958_AES2_CON_CHANNEL;
    cs[2] |= (ch + 1) << 4;
    for (i = 0; i < ARRAY_SIZE(cs); i++) {
    let mut c: unsigned = cs[i];
    for (j = 0; j < 8; j++, c >>= 1)
    dw.cs[i * 8 + j][ch] = (c & 1) << 2;
    }
    }
    dw.cs[0][0] |= BIT(4);
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_start_dma(dw: *mut snd_dw_hdmi) {
    static void dw_hdmi_start_dma(struct snd_dw_hdmi *dw)
    {
    void __iomem *base = dw.data.base;
    let mut offset: unsigned = dw.buf_offset;
    let mut period: unsigned = dw.buf_period;
    u32 start, stop;
    dw.reformat(dw, offset, period);
// Clear all irqs before enabling irqs and starting DMA
    writeb_relaxed(HDMI_IH_AHBDMAAUD_STAT0_ALL,
    base + HDMI_IH_AHBDMAAUD_STAT0);
    start = dw.buf_addr + offset;
    stop = start + period - 1;
// Setup the hardware start/stop addresses
    dw_hdmi_writel(start, base + HDMI_AHB_DMA_STRADDR0);
    dw_hdmi_writel(stop, base + HDMI_AHB_DMA_STPADDR0);
    writeb_relaxed((u8)~HDMI_AHB_DMA_MASK_DONE, base + HDMI_AHB_DMA_MASK);
    writeb(HDMI_AHB_DMA_START_START, base + HDMI_AHB_DMA_START);
    offset += period;
    if (offset >= dw.buf_size)
    offset = 0;
    dw.buf_offset = offset;
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_stop_dma(dw: *mut snd_dw_hdmi) {
    static void dw_hdmi_stop_dma(struct snd_dw_hdmi *dw)
    {
// Disable interrupts before disabling DMA
    writeb_relaxed(~0, dw.data.base + HDMI_AHB_DMA_MASK);
    writeb_relaxed(HDMI_AHB_DMA_STOP_STOP, dw.data.base + HDMI_AHB_DMA_STOP);
    }
#[no_mangle]
unsafe extern "C" fn snd_dw_hdmi_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_dw_hdmi_irq(int irq, void *data)
    {
    struct snd_dw_hdmi *dw = data;
    struct snd_pcm_substream *substream;
    unsigned stat;
    stat = readb_relaxed(dw.data.base + HDMI_IH_AHBDMAAUD_STAT0);
    if (!stat)
    return IRQ_NONE;
    writeb_relaxed(stat, dw.data.base + HDMI_IH_AHBDMAAUD_STAT0);
    substream = dw.substream;
    if (stat & HDMI_IH_AHBDMAAUD_STAT0_DONE && substream) {
    snd_pcm_period_elapsed(substream);
    spin_lock(&dw.lock);
    if (dw.substream)
    dw_hdmi_start_dma(dw);
    spin_unlock(&dw.lock);
    }
    return IRQ_HANDLED;
    }
    static const struct snd_pcm_hardware dw_hdmi_hw = {
    .info = SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID,
    .formats = SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE |
    SNDRV_PCM_FMTBIT_S24_LE,
    .rates = SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_176400 |
    SNDRV_PCM_RATE_192000,
    .channels_min = 2,
    .channels_max = 8,
    .buffer_bytes_max = 1024 * 1024,
    .period_bytes_min = 256,
    .period_bytes_max = 8192,	/* ERR004323: must limit to 8k */
    .periods_min = 2,
    .periods_max = 16,
    .fifo_size = 0,
    };
#[no_mangle]
unsafe extern "C" fn dw_hdmi_open(substream: *mut snd_pcm_substream) -> c_int {
    static int dw_hdmi_open(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_dw_hdmi *dw = substream.private_data;
    void __iomem *base = dw.data.base;
    u8 *eld;
    int ret;
    runtime.hw = dw_hdmi_hw;
    eld = dw.data.get_eld(dw.data.hdmi);
    if (eld) {
    ret = snd_pcm_hw_constraint_eld(runtime, eld);
    if (ret < 0)
    return ret;
    }
    ret = snd_pcm_limit_hw_rates(runtime);
    if (ret < 0)
    return ret;
    ret = snd_pcm_hw_constraint_integer(runtime,
    SNDRV_PCM_HW_PARAM_PERIODS);
    if (ret < 0)
    return ret;
// Limit the buffer size to the size of the preallocated buffer
    ret = snd_pcm_hw_constraint_minmax(runtime,
    SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
    0, substream.dma_buffer.bytes);
    if (ret < 0)
    return ret;
// Clear FIFO
    writeb_relaxed(HDMI_AHB_DMA_CONF0_SW_FIFO_RST,
    base + HDMI_AHB_DMA_CONF0);
// Configure interrupt polarities
    writeb_relaxed(~0, base + HDMI_AHB_DMA_POL);
    writeb_relaxed(~0, base + HDMI_AHB_DMA_BUFFPOL);
// Keep interrupts masked, and clear any pending
    writeb_relaxed(~0, base + HDMI_AHB_DMA_MASK);
    writeb_relaxed(~0, base + HDMI_IH_AHBDMAAUD_STAT0);
    ret = request_irq(dw.data.irq, snd_dw_hdmi_irq, IRQF_SHARED,
    "dw-hdmi-audio", dw);
    if (ret)
    return ret;
// Un-mute done interrupt
    writeb_relaxed(HDMI_IH_MUTE_AHBDMAAUD_STAT0_ALL &
    ~HDMI_IH_MUTE_AHBDMAAUD_STAT0_DONE,
    base + HDMI_IH_MUTE_AHBDMAAUD_STAT0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_close(substream: *mut snd_pcm_substream) -> c_int {
    static int dw_hdmi_close(struct snd_pcm_substream *substream)
    {
    struct snd_dw_hdmi *dw = substream.private_data;
// Mute all interrupts
    writeb_relaxed(HDMI_IH_MUTE_AHBDMAAUD_STAT0_ALL,
    dw.data.base + HDMI_IH_MUTE_AHBDMAAUD_STAT0);
    free_irq(dw.data.irq, dw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int dw_hdmi_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    vfree(runtime.dma_area);
    runtime.dma_area = core::ptr::null_mut();
    return 0;
    }
    static int dw_hdmi_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut size: usize = params_buffer_bytes(params);
// Allocate the PCM runtime buffer, which is exposed to userspace.
    if (runtime.dma_area) {
    if (runtime.dma_bytes >= size)
    return 0; /* already large enough */
    vfree(runtime.dma_area);
    }
    runtime.dma_area = vzalloc(size);
    if (!runtime.dma_area)
    return -ENOMEM;
    runtime.dma_bytes = size;
    return 1;
    }
    static struct page *dw_hdmi_get_page(struct snd_pcm_substream *substream,
    unsigned long offset)
    {
    return vmalloc_to_page(substream.runtime.dma_area + offset);
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int dw_hdmi_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_dw_hdmi *dw = substream.private_data;
    u8 threshold, conf0, conf1, ca;
// Setup as per 3.0.5 FSL 4.1.0 BSP
    switch (dw.revision) {
    case 0x0a:
    conf0 = HDMI_AHB_DMA_CONF0_BURST_MODE |
    HDMI_AHB_DMA_CONF0_INCR4;
    if (runtime.channels == 2)
    threshold = 126;
    else
    threshold = 124;
    break;
    case 0x1a:
    conf0 = HDMI_AHB_DMA_CONF0_BURST_MODE |
    HDMI_AHB_DMA_CONF0_INCR8;
    threshold = 128;
    break;
    default:
// NOTREACHED
    return -EINVAL;
    }
    dw_hdmi_set_sample_rate(dw.data.hdmi, runtime.rate);
// Minimum number of bytes in the fifo.
    runtime.hw.fifo_size = threshold * 32;
    conf0 |= HDMI_AHB_DMA_CONF0_EN_HLOCK;
    conf1 = default_hdmi_channel_config[runtime.channels - 2].conf1;
    ca = default_hdmi_channel_config[runtime.channels - 2].ca;
    writeb_relaxed(threshold, dw.data.base + HDMI_AHB_DMA_THRSLD);
    writeb_relaxed(conf0, dw.data.base + HDMI_AHB_DMA_CONF0);
    writeb_relaxed(conf1, dw.data.base + HDMI_AHB_DMA_CONF1);
    dw_hdmi_set_channel_count(dw.data.hdmi, runtime.channels);
    dw_hdmi_set_channel_allocation(dw.data.hdmi, ca);
    switch (runtime.format) {
    case SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE:
    dw.reformat = dw_hdmi_reformat_iec958;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    dw_hdmi_create_cs(dw, runtime);
    dw.reformat = dw_hdmi_reformat_s24;
    break;
    }
    dw.iec_offset = 0;
    dw.channels = runtime.channels;
    dw.buf_src  = runtime.dma_area;
    dw.buf_dst  = substream.dma_buffer.area;
    dw.buf_addr = substream.dma_buffer.addr;
    dw.buf_period = snd_pcm_lib_period_bytes(substream);
    dw.buf_size = snd_pcm_lib_buffer_bytes(substream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int dw_hdmi_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_dw_hdmi *dw = substream.private_data;
    unsigned long flags;
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    spin_lock_irqsave(&dw.lock, flags);
    dw.buf_offset = 0;
    dw.substream = substream;
    dw_hdmi_start_dma(dw);
    dw_hdmi_audio_enable(dw.data.hdmi);
    spin_unlock_irqrestore(&dw.lock, flags);
    substream.runtime.delay = substream.runtime.period_size;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    spin_lock_irqsave(&dw.lock, flags);
    dw.substream = core::ptr::null_mut();
    dw_hdmi_stop_dma(dw);
    dw_hdmi_audio_disable(dw.data.hdmi);
    spin_unlock_irqrestore(&dw.lock, flags);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_hdmi_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t dw_hdmi_pointer(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_dw_hdmi *dw = substream.private_data;
//
// We are unable to report the exact hardware position as
// reading the 32-bit DMA position using 8-bit reads is racy.
//
    return bytes_to_frames(runtime, dw.buf_offset);
    }
    static const struct snd_pcm_ops snd_dw_hdmi_ops = {
    .open = dw_hdmi_open,
    .close = dw_hdmi_close,
    .ioctl = snd_pcm_lib_ioctl,
    .hw_params = dw_hdmi_hw_params,
    .hw_free = dw_hdmi_hw_free,
    .prepare = dw_hdmi_prepare,
    .trigger = dw_hdmi_trigger,
    .pointer = dw_hdmi_pointer,
    .page = dw_hdmi_get_page,
    };
#[no_mangle]
unsafe extern "C" fn snd_dw_hdmi_probe(pdev: *mut platform_device) -> c_int {
    static int snd_dw_hdmi_probe(struct platform_device *pdev)
    {
    const struct dw_hdmi_audio_data *data = pdev.dev.platform_data;
    struct device *dev = pdev.dev.parent;
    struct snd_dw_hdmi *dw;
    struct snd_card *card;
    struct snd_pcm *pcm;
    unsigned revision;
    int ret;
    writeb_relaxed(HDMI_IH_MUTE_AHBDMAAUD_STAT0_ALL,
    data.base + HDMI_IH_MUTE_AHBDMAAUD_STAT0);
    revision = readb_relaxed(data.base + HDMI_REVISION_ID);
    if (revision != 0x0a && revision != 0x1a) {
    dev_err(dev, "dw-hdmi-audio: unknown revision 0x%02x\n",
    revision);
    return -ENXIO;
    }
    ret = snd_card_new(dev, SNDRV_DEFAULT_IDX1, SNDRV_DEFAULT_STR1,
    THIS_MODULE, sizeof(struct snd_dw_hdmi), &card);
    if (ret < 0)
    return ret;
    strscpy(card.driver, DRIVER_NAME, sizeof(card.driver));
    strscpy(card.shortname, "DW-HDMI", sizeof(card.shortname));
    snprintf(card.longname, sizeof(card.longname),
    "%s rev 0x%02x, irq %d", card.shortname, revision,
    data.irq);
    dw = card.private_data;
    dw.card = card;
    dw.data = *data;
    dw.revision = revision;
    spin_lock_init(&dw.lock);
    ret = snd_pcm_new(card, "DW HDMI", 0, 1, 0, &pcm);
    if (ret < 0)
    goto err;
    dw.pcm = pcm;
    pcm.private_data = dw;
    strscpy(pcm.name, DRIVER_NAME, sizeof(pcm.name));
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_dw_hdmi_ops);
//
// To support 8-channel 96kHz audio reliably, we need 512k
// to satisfy alsa with our restricted period (ERR004323).
//
    snd_pcm_lib_preallocate_pages_for_all(pcm, SNDRV_DMA_TYPE_DEV,
    dev, 128 * 1024, 1024 * 1024);
    ret = snd_card_register(card);
    if (ret < 0)
    goto err;
    platform_set_drvdata(pdev, dw);
    return 0;
    err:
    snd_card_free(card);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_dw_hdmi_remove(pdev: *mut platform_device) {
    static void snd_dw_hdmi_remove(struct platform_device *pdev)
    {
    struct snd_dw_hdmi *dw = platform_get_drvdata(pdev);
    snd_card_free(dw.card);
    }

//
// This code is fine, but requires implementation in the dw_hdmi_trigger()
// method which is currently missing as I have no way to test this.
//
#[no_mangle]
unsafe extern "C" fn snd_dw_hdmi_suspend(dev: *mut device) -> c_int {
    static int snd_dw_hdmi_suspend(struct device *dev)
    {
    struct snd_dw_hdmi *dw = dev_get_drvdata(dev);
    snd_power_change_state(dw.card, SNDRV_CTL_POWER_D3cold);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_dw_hdmi_resume(dev: *mut device) -> c_int {
    static int snd_dw_hdmi_resume(struct device *dev)
    {
    struct snd_dw_hdmi *dw = dev_get_drvdata(dev);
    snd_power_change_state(dw.card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(snd_dw_hdmi_pm, snd_dw_hdmi_suspend,
    snd_dw_hdmi_resume);

    static struct platform_driver snd_dw_hdmi_driver = {
    .probe	= snd_dw_hdmi_probe,
    .remove = snd_dw_hdmi_remove,
    .driver	= {
    .name = DRIVER_NAME,
    .pm = PM_OPS,
    },
    };
    module_platform_driver(snd_dw_hdmi_driver);
    MODULE_AUTHOR("Russell King <rmk+kernel@armlinux.org.uk>");
    MODULE_DESCRIPTION("Synopsis Designware HDMI AHB ALSA interface");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);
