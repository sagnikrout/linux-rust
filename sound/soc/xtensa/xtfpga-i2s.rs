//! Automatically rewritten from C to Rust
//! Source: sound/soc/xtensa/xtfpga-i2s.c
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
// Xtfpga I2S controller driver
//
// Copyright (c) 2014 Cadence Design Systems Inc.
//

pub const XTFPGA_I2S_VERSION: c_uint = 0x00;
pub const XTFPGA_I2S_CONFIG: c_uint = 0x04;
pub const XTFPGA_I2S_INT_MASK: c_uint = 0x08;
pub const XTFPGA_I2S_INT_STATUS: c_uint = 0x0c;
pub const XTFPGA_I2S_CHAN0_DATA: c_uint = 0x10;
pub const XTFPGA_I2S_CHAN1_DATA: c_uint = 0x14;
pub const XTFPGA_I2S_CHAN2_DATA: c_uint = 0x18;
pub const XTFPGA_I2S_CHAN3_DATA: c_uint = 0x1c;
pub const XTFPGA_I2S_CONFIG_TX_ENABLE: c_uint = 0x1;
pub const XTFPGA_I2S_CONFIG_INT_ENABLE: c_uint = 0x2;
pub const XTFPGA_I2S_CONFIG_LEFT: c_uint = 0x4;
pub const XTFPGA_I2S_CONFIG_RATIO_BASE: c_int = 8;
pub const XTFPGA_I2S_CONFIG_RATIO_MASK: c_uint = 0x0000ff00;
pub const XTFPGA_I2S_CONFIG_RES_BASE: c_int = 16;
pub const XTFPGA_I2S_CONFIG_RES_MASK: c_uint = 0x003f0000;
pub const XTFPGA_I2S_CONFIG_LEVEL_BASE: c_int = 24;
pub const XTFPGA_I2S_CONFIG_LEVEL_MASK: c_uint = 0x0f000000;
pub const XTFPGA_I2S_CONFIG_CHANNEL_BASE: c_int = 28;
pub const XTFPGA_I2S_INT_UNDERRUN: c_uint = 0x1;
pub const XTFPGA_I2S_INT_LEVEL: c_uint = 0x2;
pub const XTFPGA_I2S_INT_VALID: c_uint = 0x3;
pub const XTFPGA_I2S_FIFO_SIZE: c_int = 8192;
//
// I2S controller operation:
//
// Enabling TX: output 1 period of zeros (starting with left channel)
// and then queued data.
//
// Level status and interrupt: whenever FIFO level is below FIFO trigger,
// level status is 1 and an IRQ is asserted (if enabled).
//
// Underrun status and interrupt: whenever FIFO is empty, underrun status
// is 1 and an IRQ is asserted (if enabled).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtfpga_i2s {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub regs: *mut void __iomem,
// current playback substream. NULL if not playing.
//
// Access to that field is synchronized between the interrupt handler
// and userspace through RCU.
//
// Interrupt handler (threaded part) does PIO on substream data in RCU
// read-side critical section. Trigger callback sets and clears the
// pointer when the playback is started and stopped with
// rcu_assign_pointer. When userspace is about to free the playback
// stream in the pcm_close callback it synchronizes with the interrupt
// handler by means of synchronize_rcu call.
//
    pub tx_substream: *mut snd_pcm_substream __rcu,
    unsigned (*tx_fn)(struct xtfpga_i2s *i2s,
    struct snd_pcm_runtime *runtime,
    pub tx_ptr): unsigned,
    pub /: *mut *mut unsigned tx_ptr; / next frame index in the sample buffer,
// current fifo level estimate.
// Doesn't have to be perfectly accurate, but must be not less than
// the actual FIFO level in order to avoid stall on push attempt.
//
    pub tx_fifo_level: unsigned,
// FIFO level at which level interrupt occurs
    pub tx_fifo_low: unsigned,
// maximal FIFO level
    pub tx_fifo_high: unsigned,
}

#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_wr_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool xtfpga_i2s_wr_reg(struct device *dev, unsigned int reg)
    {
    return reg >= XTFPGA_I2S_CONFIG;
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool xtfpga_i2s_rd_reg(struct device *dev, unsigned int reg)
    {
    return reg < XTFPGA_I2S_CHAN0_DATA;
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool xtfpga_i2s_volatile_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = XTFPGA_I2S_INT_STATUS;
    }
    static const struct regmap_config xtfpga_i2s_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = XTFPGA_I2S_CHAN3_DATA,
    .writeable_reg = xtfpga_i2s_wr_reg,
    .readable_reg = xtfpga_i2s_rd_reg,
    .volatile_reg = xtfpga_i2s_volatile_reg,
    .cache_type = REGCACHE_FLAT,
    };
// Generate functions that do PIO from TX DMA area to FIFO for all supported
// stream formats.
// Functions will be called xtfpga_pcm_tx_<channels>x<sample bits>, e.g.
// xtfpga_pcm_tx_2x16 for 16-bit stereo.
//
// FIFO consists of 32-bit words, one word per channel, always 2 channels.
// If I2S interface is configured with smaller sample resolution, only
// the LSB of each word is used.
//

    static unsigned xtfpga_pcm_tx_##channels##x##sample_bits( \
    struct xtfpga_i2s *i2s, struct snd_pcm_runtime *runtime, \
    unsigned tx_ptr) \
    { \
    const u##sample_bits (*p)[channels] = \
    (void *)runtime.dma_area; \
    \
    for (; i2s.tx_fifo_level < i2s.tx_fifo_high; \
    i2s.tx_fifo_level += 2) { \
    iowrite32(p[tx_ptr][0], \
    i2s.regs + XTFPGA_I2S_CHAN0_DATA); \
    iowrite32(p[tx_ptr][channels - 1], \
    i2s.regs + XTFPGA_I2S_CHAN0_DATA); \
    if (++tx_ptr >= runtime.buffer_size) \
    tx_ptr = 0; \
    } \
    return tx_ptr; \
    }
    xtfpga_pcm_tx_fn(1, 16)
    xtfpga_pcm_tx_fn(2, 16)
    xtfpga_pcm_tx_fn(1, 32)
    xtfpga_pcm_tx_fn(2, 32)

#[no_mangle]
unsafe extern "C" fn xtfpga_pcm_push_tx(i2s: *mut xtfpga_i2s) -> bool {
    static bool xtfpga_pcm_push_tx(struct xtfpga_i2s *i2s)
    {
    struct snd_pcm_substream *tx_substream;
    bool tx_active;
    rcu_read_lock();
    tx_substream = rcu_dereference(i2s.tx_substream);
    tx_active = tx_substream && snd_pcm_running(tx_substream);
    if (tx_active) {
    let mut tx_ptr: unsigned = READ_ONCE(i2s.tx_ptr);
    unsigned new_tx_ptr = i2s.tx_fn(i2s, tx_substream.runtime,
    tx_ptr);
    cmpxchg(&i2s.tx_ptr, tx_ptr, new_tx_ptr);
    }
    rcu_read_unlock();
    return tx_active;
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_pcm_refill_fifo(i2s: *mut xtfpga_i2s) {
    static void xtfpga_pcm_refill_fifo(struct xtfpga_i2s *i2s)
    {
    unsigned int_status;
    unsigned i;
    regmap_read(i2s.regmap, XTFPGA_I2S_INT_STATUS,
    &int_status);
    for (i = 0; i < 2; ++i) {
    let mut tx_active: bool = xtfpga_pcm_push_tx(i2s);
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_STATUS,
    XTFPGA_I2S_INT_VALID);
    if (tx_active)
    regmap_read(i2s.regmap, XTFPGA_I2S_INT_STATUS,
    &int_status);
    if (!tx_active ||
    !(int_status & XTFPGA_I2S_INT_LEVEL))
    break;
// After the push the level IRQ is still asserted,
// means FIFO level is below tx_fifo_low. Estimate
// it as tx_fifo_low.
//
    i2s.tx_fifo_level = i2s.tx_fifo_low;
    }
    if (!(int_status & XTFPGA_I2S_INT_LEVEL))
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_MASK,
    XTFPGA_I2S_INT_VALID);
#[no_mangle]
pub unsafe extern "C" fn if(XTFPGA_I2S_INT_UNDERRUN): !(int_status &) -> else {
    else if (!(int_status & XTFPGA_I2S_INT_UNDERRUN))
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_MASK,
    XTFPGA_I2S_INT_UNDERRUN);
    if (!(int_status & XTFPGA_I2S_INT_UNDERRUN))
    regmap_update_bits(i2s.regmap, XTFPGA_I2S_CONFIG,
    XTFPGA_I2S_CONFIG_INT_ENABLE |
    XTFPGA_I2S_CONFIG_TX_ENABLE,
    XTFPGA_I2S_CONFIG_INT_ENABLE |
    XTFPGA_I2S_CONFIG_TX_ENABLE);
    else
    regmap_update_bits(i2s.regmap, XTFPGA_I2S_CONFIG,
    XTFPGA_I2S_CONFIG_INT_ENABLE |
    XTFPGA_I2S_CONFIG_TX_ENABLE, 0);
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_threaded_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xtfpga_i2s_threaded_irq_handler(int irq, void *dev_id)
    {
    struct xtfpga_i2s *i2s = dev_id;
    struct snd_pcm_substream *tx_substream;
    unsigned config, int_status, int_mask;
    regmap_read(i2s.regmap, XTFPGA_I2S_CONFIG, &config);
    regmap_read(i2s.regmap, XTFPGA_I2S_INT_MASK, &int_mask);
    regmap_read(i2s.regmap, XTFPGA_I2S_INT_STATUS, &int_status);
    if (!(config & XTFPGA_I2S_CONFIG_INT_ENABLE) ||
    !(int_status & int_mask & XTFPGA_I2S_INT_VALID))
    return IRQ_NONE;
// Update FIFO level estimate in accordance with interrupt status
// register.
//
    if (int_status & XTFPGA_I2S_INT_UNDERRUN) {
    i2s.tx_fifo_level = 0;
    regmap_update_bits(i2s.regmap, XTFPGA_I2S_CONFIG,
    XTFPGA_I2S_CONFIG_TX_ENABLE, 0);
    } else {
// The FIFO isn't empty, but is below tx_fifo_low. Estimate
// it as tx_fifo_low.
//
    i2s.tx_fifo_level = i2s.tx_fifo_low;
    }
    rcu_read_lock();
    tx_substream = rcu_dereference(i2s.tx_substream);
    if (tx_substream && snd_pcm_running(tx_substream)) {
    snd_pcm_period_elapsed(tx_substream);
    if (int_status & XTFPGA_I2S_INT_UNDERRUN)
    dev_dbg_ratelimited(i2s.dev, "%s: underrun\n",
    __func__);
    }
    rcu_read_unlock();
// Refill FIFO, update allowed IRQ reasons, enable IRQ if FIFO is
// not empty.
//
    xtfpga_pcm_refill_fifo(i2s);
    return IRQ_HANDLED;
    }
    static int xtfpga_i2s_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct xtfpga_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_set_dma_data(dai, substream, i2s);
    return 0;
    }
    static int xtfpga_i2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct xtfpga_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    let mut srate: unsigned = params_rate(params);
    let mut channels: unsigned = params_channels(params);
    let mut period_size: unsigned = params_period_size(params);
    let mut sample_size: unsigned = snd_pcm_format_width(params_format(params));
    unsigned freq, ratio, level;
    int err;
    regmap_update_bits(i2s.regmap, XTFPGA_I2S_CONFIG,
    XTFPGA_I2S_CONFIG_RES_MASK,
    sample_size << XTFPGA_I2S_CONFIG_RES_BASE);
    freq = 256 * srate;
    err = clk_set_rate(i2s.clk, freq);
    if (err < 0)
    return err;
// ratio field of the config register controls MCLK->I2S clock
// derivation: I2S clock = MCLK / (2 * (ratio + 2)).
//
// So with MCLK = 256 * sample rate ratio is 0 for 32 bit stereo
// and 2 for 16 bit stereo.
//
    ratio = (freq - (srate * sample_size * 8)) /
    (srate * sample_size * 4);
    regmap_update_bits(i2s.regmap, XTFPGA_I2S_CONFIG,
    XTFPGA_I2S_CONFIG_RATIO_MASK,
    ratio << XTFPGA_I2S_CONFIG_RATIO_BASE);
    i2s.tx_fifo_low = XTFPGA_I2S_FIFO_SIZE / 2;
// period_size * 2: FIFO always gets 2 samples per frame
    for (level = 1;
    i2s.tx_fifo_low / 2 >= period_size * 2 &&
    level < (XTFPGA_I2S_CONFIG_LEVEL_MASK >>
    XTFPGA_I2S_CONFIG_LEVEL_BASE); ++level)
    i2s.tx_fifo_low /= 2;
    i2s.tx_fifo_high = 2 * i2s.tx_fifo_low;
    regmap_update_bits(i2s.regmap, XTFPGA_I2S_CONFIG,
    XTFPGA_I2S_CONFIG_LEVEL_MASK,
    level << XTFPGA_I2S_CONFIG_LEVEL_BASE);
    dev_dbg(i2s.dev,
    "%s srate: %u, channels: %u, sample_size: %u, period_size: %u\n",
    __func__, srate, channels, sample_size, period_size);
    dev_dbg(i2s.dev, "%s freq: %u, ratio: %u, level: %u\n",
    __func__, freq, ratio, level);
    return 0;
    }
    static int xtfpga_i2s_set_fmt(struct snd_soc_dai *cpu_dai,
    unsigned int fmt)
    {
    if ((fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF)
    return -EINVAL;
    if ((fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_BP_FP)
    return -EINVAL;
    if ((fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S)
    return -EINVAL;
    return 0;
    }
// PCM
    static const struct snd_pcm_hardware xtfpga_pcm_hardware = {
    .info = SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_BLOCK_TRANSFER,
    .formats		= SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    .channels_min		= 1,
    .channels_max		= 2,
    .period_bytes_min	= 2,
    .period_bytes_max	= XTFPGA_I2S_FIFO_SIZE / 2 * 8,
    .periods_min		= 2,
    .periods_max		= XTFPGA_I2S_FIFO_SIZE * 8 / 2,
    .buffer_bytes_max	= XTFPGA_I2S_FIFO_SIZE * 8,
    .fifo_size		= 16,
    };
    static int xtfpga_pcm_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    void *p;
    snd_soc_set_runtime_hwparams(substream, &xtfpga_pcm_hardware);
    p = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    runtime.private_data = p;
    return 0;
    }
    static int xtfpga_pcm_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    synchronize_rcu();
    return 0;
    }
    static int xtfpga_pcm_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct xtfpga_i2s *i2s = runtime.private_data;
    let mut channels: unsigned = params_channels(hw_params);
    switch (channels) {
    case 1:
    case 2:
    break;
    default:
    return -EINVAL;
    }
    switch (params_format(hw_params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    i2s.tx_fn = (channels == 1) ?
    xtfpga_pcm_tx_1x16 :
    xtfpga_pcm_tx_2x16;
    break;
    case SNDRV_PCM_FORMAT_S32_LE:
    i2s.tx_fn = (channels == 1) ?
    xtfpga_pcm_tx_1x32 :
    xtfpga_pcm_tx_2x32;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int xtfpga_pcm_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    let mut ret: c_int = 0;
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct xtfpga_i2s *i2s = runtime.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    WRITE_ONCE(i2s.tx_ptr, 0);
    rcu_assign_pointer(i2s.tx_substream, substream);
    xtfpga_pcm_refill_fifo(i2s);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    rcu_assign_pointer(i2s.tx_substream, core::ptr::null_mut());
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static snd_pcm_uframes_t xtfpga_pcm_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct xtfpga_i2s *i2s = runtime.private_data;
    let mut pos: snd_pcm_uframes_t = READ_ONCE(i2s.tx_ptr);
    return pos < runtime.buffer_size ? pos : 0;
    }
    static int xtfpga_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_card *card = rtd.card.snd_card;
    let mut size: usize = xtfpga_pcm_hardware.buffer_bytes_max;
    snd_pcm_set_managed_buffer_all(rtd.pcm, SNDRV_DMA_TYPE_DEV,
    card.dev, size, size);
    return 0;
    }
    static const struct snd_soc_component_driver xtfpga_i2s_component = {
    .name			= DRV_NAME,
    .open			= xtfpga_pcm_open,
    .close			= xtfpga_pcm_close,
    .hw_params		= xtfpga_pcm_hw_params,
    .trigger		= xtfpga_pcm_trigger,
    .pointer		= xtfpga_pcm_pointer,
    .pcm_new		= xtfpga_pcm_new,
    .legacy_dai_naming	= 1,
    };
    static const struct snd_soc_dai_ops xtfpga_i2s_dai_ops = {
    .startup	= xtfpga_i2s_startup,
    .hw_params      = xtfpga_i2s_hw_params,
    .set_fmt	= xtfpga_i2s_set_fmt,
    };
    static struct snd_soc_dai_driver xtfpga_i2s_dai[] = {
    {
    .name = "xtfpga-i2s",
    .id = 0,
    .playback = {
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    },
    .ops = &xtfpga_i2s_dai_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_runtime_suspend(dev: *mut device) -> c_int {
    static int xtfpga_i2s_runtime_suspend(struct device *dev)
    {
    struct xtfpga_i2s *i2s = dev_get_drvdata(dev);
    clk_disable_unprepare(i2s.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_runtime_resume(dev: *mut device) -> c_int {
    static int xtfpga_i2s_runtime_resume(struct device *dev)
    {
    struct xtfpga_i2s *i2s = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(i2s.clk);
    if (ret) {
    dev_err(dev, "clk_prepare_enable failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_probe(pdev: *mut platform_device) -> c_int {
    static int xtfpga_i2s_probe(struct platform_device *pdev)
    {
    struct xtfpga_i2s *i2s;
    int err, irq;
    i2s = devm_kzalloc(&pdev.dev, sizeof(*i2s), GFP_KERNEL);
    if (!i2s)
    return -ENOMEM;
    platform_set_drvdata(pdev, i2s);
    i2s.dev = &pdev.dev;
    dev_dbg(&pdev.dev, "dev: %p, i2s: %p\n", &pdev.dev, i2s);
    i2s.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2s.regs))
    return PTR_ERR(i2s.regs);
    i2s.regmap = devm_regmap_init_mmio(&pdev.dev, i2s.regs,
    &xtfpga_i2s_regmap_config);
    if (IS_ERR(i2s.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(i2s.regmap), "regmap init failed\n");
    i2s.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(i2s.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(i2s.clk), "couldn't get clock\n");
    regmap_write(i2s.regmap, XTFPGA_I2S_CONFIG,
    (0x1 << XTFPGA_I2S_CONFIG_CHANNEL_BASE));
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_STATUS, XTFPGA_I2S_INT_VALID);
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_MASK, XTFPGA_I2S_INT_UNDERRUN);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    xtfpga_i2s_threaded_irq_handler,
    IRQF_SHARED | IRQF_ONESHOT,
    pdev.name, i2s);
    if (err < 0)
    return err;
    err = devm_snd_soc_register_component(&pdev.dev,
    &xtfpga_i2s_component,
    xtfpga_i2s_dai,
    ARRAY_SIZE(xtfpga_i2s_dai));
    if (err < 0)
    return err;
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    err = xtfpga_i2s_runtime_resume(&pdev.dev);
    if (err) {
    pm_runtime_disable(&pdev.dev);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xtfpga_i2s_remove(pdev: *mut platform_device) {
    static void xtfpga_i2s_remove(struct platform_device *pdev)
    {
    struct xtfpga_i2s *i2s = dev_get_drvdata(&pdev.dev);
    if (i2s.regmap && !IS_ERR(i2s.regmap)) {
    regmap_write(i2s.regmap, XTFPGA_I2S_CONFIG, 0);
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_MASK, 0);
    regmap_write(i2s.regmap, XTFPGA_I2S_INT_STATUS,
    XTFPGA_I2S_INT_VALID);
    }
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    xtfpga_i2s_runtime_suspend(&pdev.dev);
    }

    static const struct of_device_id xtfpga_i2s_of_match[] = {
    { .compatible = "cdns,xtfpga-i2s", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xtfpga_i2s_of_match);

    static const struct dev_pm_ops xtfpga_i2s_pm_ops = {
    RUNTIME_PM_OPS(xtfpga_i2s_runtime_suspend,
    xtfpga_i2s_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver xtfpga_i2s_driver = {
    .probe   = xtfpga_i2s_probe,
    .remove = xtfpga_i2s_remove,
    .driver  = {
    .name = "xtfpga-i2s",
    .of_match_table = of_match_ptr(xtfpga_i2s_of_match),
    .pm = pm_ptr(&xtfpga_i2s_pm_ops),
    },
    };
    module_platform_driver(xtfpga_i2s_driver);
    MODULE_AUTHOR("Max Filippov <jcmvbkbc@gmail.com>");
    MODULE_DESCRIPTION("xtfpga I2S controller driver");
    MODULE_LICENSE("GPL v2");
