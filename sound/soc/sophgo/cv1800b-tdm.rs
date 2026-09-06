//! Automatically rewritten from C to Rust
//! Source: sound/soc/sophgo/cv1800b-tdm.c
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

pub const CV1800B_DEF_FREQ: c_int = 24576000;
pub const CV1800B_DEF_MCLK_FS_RATIO: c_int = 256;
// tdm registers
pub const CV1800B_BLK_MODE_SETTING: c_uint = 0x000;
pub const CV1800B_FRAME_SETTING: c_uint = 0x004;
pub const CV1800B_SLOT_SETTING1: c_uint = 0x008;
pub const CV1800B_SLOT_SETTING2: c_uint = 0x00C;
pub const CV1800B_DATA_FORMAT: c_uint = 0x010;
pub const CV1800B_BLK_CFG: c_uint = 0x014;
pub const CV1800B_I2S_ENABLE: c_uint = 0x018;
pub const CV1800B_I2S_RESET: c_uint = 0x01C;
pub const CV1800B_I2S_INT_EN: c_uint = 0x020;
pub const CV1800B_I2S_INT: c_uint = 0x024;
pub const CV1800B_FIFO_THRESHOLD: c_uint = 0x028;
pub const CV1800B_LRCK_MASTER: c_uint = 0x02C /* special clock only mode */;
pub const CV1800B_FIFO_RESET: c_uint = 0x030;
pub const CV1800B_RX_STATUS: c_uint = 0x040;
pub const CV1800B_TX_STATUS: c_uint = 0x048;
pub const CV1800B_CLK_CTRL0: c_uint = 0x060;
pub const CV1800B_CLK_CTRL1: c_uint = 0x064;
pub const CV1800B_PCM_SYNTH: c_uint = 0x068;
pub const CV1800B_RX_RD_PORT: c_uint = 0x080;
pub const CV1800B_TX_WR_PORT: c_uint = 0x0C0;
// CV1800B_BLK_MODE_SETTING (0x000)

// CV1800B_CLK_CTRL1 (0x064)

// CV1800B_CLK_CTRL0 (0x060)

// CV1800B_I2S_RESET (0x01C)

// CV1800B_FIFO_RESET (0x030)

// CV1800B_I2S_ENABLE (0x018)

// CV1800B_BLK_CFG (0x014)

// CV1800B_FRAME_SETTING (0x004)

// CV1800B_I2S_INT_EN (0x020)

// CV1800B_SLOT_SETTING2 (0x00C)

// CV1800B_LRCK_MASTER (0x02C)

// CV1800B_DATA_FORMAT (0x010)

// CV1800B_FIFO_THRESHOLD (0x028)

// CV1800B_SLOT_SETTING1 (0x008)

    enum cv1800b_tdm_word_length {
    CV1800B_WORD_LENGTH_8_BIT = 0,
    CV1800B_WORD_LENGTH_16_BIT = 1,
    CV1800B_WORD_LENGTH_32_BIT = 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800b_i2s {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub sysclk: *mut clk,
    pub dev: *mut device,
    pub playback_dma: snd_dmaengine_dai_dma_data,
    pub capture_dma: snd_dmaengine_dai_dma_data,
    pub mclk_rate: u32,
    pub bclk_ratio_fixed: bool,
    pub bclk_ratio: u32,
}

    static void cv1800b_setup_dma_struct(struct cv1800b_i2s *i2s,
    phys_addr_t phys_base)
    {
    i2s.playback_dma.addr = phys_base + CV1800B_TX_WR_PORT;
    i2s.playback_dma.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    i2s.playback_dma.fifo_size = TX_FIFO_SIZE;
    i2s.playback_dma.maxburst = TX_MAX_BURST;
    i2s.capture_dma.addr = phys_base + CV1800B_RX_RD_PORT;
    i2s.capture_dma.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    i2s.capture_dma.fifo_size = RX_FIFO_SIZE;
    i2s.capture_dma.maxburst = RX_MAX_BURST;
    }
    static const struct snd_dmaengine_pcm_config cv1800b_i2s_pcm_config = {
    .prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config,
    };
#[no_mangle]
unsafe extern "C" fn cv1800b_reset_fifo(i2s: *mut cv1800b_i2s) {
    static void cv1800b_reset_fifo(struct cv1800b_i2s *i2s)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_FIFO_RESET);
    val = u32_replace_bits(val, 1, FIFO_RX_RESET_MASK);
    val = u32_replace_bits(val, 1, FIFO_TX_RESET_MASK);
    writel(val, i2s.base + CV1800B_FIFO_RESET);
    usleep_range(10, 20);
    val = readl(i2s.base + CV1800B_FIFO_RESET);
    val = u32_replace_bits(val, 0, FIFO_RX_RESET_MASK);
    val = u32_replace_bits(val, 0, FIFO_TX_RESET_MASK);
    writel(val, i2s.base + CV1800B_FIFO_RESET);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_reset_i2s(i2s: *mut cv1800b_i2s) {
    static void cv1800b_reset_i2s(struct cv1800b_i2s *i2s)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_I2S_RESET);
    val = u32_replace_bits(val, 1, RST_I2S_RESET_RX_MASK);
    val = u32_replace_bits(val, 1, RST_I2S_RESET_TX_MASK);
    writel(val, i2s.base + CV1800B_I2S_RESET);
    usleep_range(10, 20);
    val = readl(i2s.base + CV1800B_I2S_RESET);
    val = u32_replace_bits(val, 0, RST_I2S_RESET_RX_MASK);
    val = u32_replace_bits(val, 0, RST_I2S_RESET_TX_MASK);
    writel(val, i2s.base + CV1800B_I2S_RESET);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_set_mclk_div(i2s: *mut cv1800b_i2s, mclk_div: u32) {
    static void cv1800b_set_mclk_div(struct cv1800b_i2s *i2s, u32 mclk_div)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_CLK_CTRL1);
    val = u32_replace_bits(val, mclk_div, CLK_MCLK_DIV_MASK);
    writel(val, i2s.base + CV1800B_CLK_CTRL1);
    dev_dbg(i2s.dev, "mclk_div is set to %u\n", mclk_div);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_set_tx_mode(i2s: *mut cv1800b_i2s, is_tx: bool) {
    static void cv1800b_set_tx_mode(struct cv1800b_i2s *i2s, bool is_tx)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_BLK_MODE_SETTING);
    val = u32_replace_bits(val, is_tx, BLK_TX_MODE_MASK);
    writel(val, i2s.base + CV1800B_BLK_MODE_SETTING);
    dev_dbg(i2s.dev, "tx_mode is set to %u\n", is_tx);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_set_bclk_div(i2s: *mut cv1800b_i2s, bclk_div: u32) -> c_int {
    static int cv1800b_set_bclk_div(struct cv1800b_i2s *i2s, u32 bclk_div)
    {
    u32 val;
    if (bclk_div == 0 || bclk_div > 0xFFFF)
    return -EINVAL;
    val = readl(i2s.base + CV1800B_CLK_CTRL1);
    val = u32_replace_bits(val, bclk_div, CLK_BCLK_DIV_MASK);
    writel(val, i2s.base + CV1800B_CLK_CTRL1);
    dev_dbg(i2s.dev, "bclk_div is set to %u\n", bclk_div);
    return 0;
    }
// set memory width of audio data , reg word_length
    static int cv1800b_set_word_length(struct cv1800b_i2s *i2s,
    unsigned int physical_width)
    {
    u8 word_length_val;
    u32 val;
    switch (physical_width) {
    case 8:
    word_length_val = CV1800B_WORD_LENGTH_8_BIT;
    break;
    case 16:
    word_length_val = CV1800B_WORD_LENGTH_16_BIT;
    break;
    case 32:
    word_length_val = CV1800B_WORD_LENGTH_32_BIT;
    break;
    default:
    dev_dbg(i2s.dev, "can't set word_length field\n");
    return -EINVAL;
    }
    val = readl(i2s.base + CV1800B_DATA_FORMAT);
    val = u32_replace_bits(val, word_length_val, DF_WORD_LENGTH_MASK);
    writel(val, i2s.base + CV1800B_DATA_FORMAT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_enable_clocks(i2s: *mut cv1800b_i2s, enabled: bool) {
    static void cv1800b_enable_clocks(struct cv1800b_i2s *i2s, bool enabled)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_CLK_CTRL0);
    val = u32_replace_bits(val, enabled, CLK_AUD_EN_MASK);
    writel(val, i2s.base + CV1800B_CLK_CTRL0);
    }
    static int cv1800b_set_slot_settings(struct cv1800b_i2s *i2s, u32 slots,
    u32 physical_width, u32 data_size)
    {
    u32 slot_num;
    u32 slot_size;
    u32 frame_length;
    u32 frame_active_length;
    u32 val;
    if (!slots || !physical_width || !data_size) {
    dev_err(i2s.dev, "frame or slot settings are not valid\n");
    return -EINVAL;
    }
    if (slots > 16 || physical_width > 64 || data_size > 32) {
    dev_err(i2s.dev, "frame or slot settings are not valid\n");
    return -EINVAL;
    }
    slot_num = slots - 1;
    slot_size = physical_width - 1;
    frame_length = (physical_width * slots) - 1;
    frame_active_length = physical_width - 1;
    if (frame_length > 511 || frame_active_length > 255) {
    dev_err(i2s.dev, "frame or slot settings are not valid\n");
    return -EINVAL;
    }
    val = readl(i2s.base + CV1800B_SLOT_SETTING1);
    val = u32_replace_bits(val, slot_size, SLOT_SIZE_MASK);
    val = u32_replace_bits(val, data_size - 1, DATA_SIZE_MASK);
    val = u32_replace_bits(val, slot_num, SLOT_NUM_MASK);
    writel(val, i2s.base + CV1800B_SLOT_SETTING1);
    val = readl(i2s.base + CV1800B_FRAME_SETTING);
    val = u32_replace_bits(val, frame_length, FRAME_LENGTH_MASK);
    val = u32_replace_bits(val, frame_active_length, FS_ACTIVE_LENGTH_MASK);
    writel(val, i2s.base + CV1800B_FRAME_SETTING);
    dev_dbg(i2s.dev, "slot settings num: %u width: %u\n", slots, physical_width);
    return 0;
    }
//
// calculate mclk_div.
// if requested value is bigger than optimal
// leave mclk_div as 1. cff clock is capable
// to handle it
//
#[no_mangle]
unsafe extern "C" fn cv1800b_calc_mclk_div(target_mclk: c_uint, mclk_div: *mut u32) -> c_int {
    static int cv1800b_calc_mclk_div(unsigned int target_mclk, u32 *mclk_div)
    {
// mclk_div = 1;
    if (target_mclk == 0)
    return -EINVAL;
// optimal parent frequency is close to CV1800B_DEF_FREQ
    if (target_mclk < CV1800B_DEF_FREQ) {
// mclk_div = DIV_ROUND_CLOSEST(CV1800B_DEF_FREQ, target_mclk);
    if (!*mclk_div || *mclk_div > 0xFFFF)
    return -EINVAL;
    }
    return 0;
    }
//
// set CCF clock and divider for this clock
// mclk_clock = ccf_clock / mclk_div
//
    static int cv1800b_i2s_set_rate_for_mclk(struct cv1800b_i2s *i2s,
    unsigned int target_mclk)
    {
    let mut mclk_div: u32 = 1;
    u64 tmp;
    int ret;
    unsigned long clk_rate;
    unsigned long actual;
    ret = cv1800b_calc_mclk_div(target_mclk, &mclk_div);
    if (ret) {
    dev_dbg(i2s.dev, "can't calc mclk_div for freq %u\n",
    target_mclk);
    return ret;
    }
    tmp = (u64)target_mclk * mclk_div;
    if (tmp > ULONG_MAX) {
    dev_err(i2s.dev, "clk_rate overflow: freq=%u div=%u\n",
    target_mclk, mclk_div);
    return -ERANGE;
    }
    clk_rate = (unsigned long)tmp;
    cv1800b_enable_clocks(i2s, false);
    ret = clk_set_rate(i2s.sysclk, clk_rate);
    if (ret)
    return ret;
    actual = clk_get_rate(i2s.sysclk);
    if (clk_rate != actual) {
    dev_err_ratelimited(i2s.dev,
    "clk_set_rate failed %lu, actual is %lu\n",
    clk_rate, actual);
    }
    cv1800b_set_mclk_div(i2s, mclk_div);
    cv1800b_enable_clocks(i2s, true);
    return 0;
    }
    static int cv1800b_i2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    let mut rate: c_uint = params_rate(params);
    let mut channels: c_uint = params_channels(params);
    let mut physical_width: c_uint = params_physical_width(params);
    let mut data_width: c_int = params_width(params);
    let mut tx_mode: bool = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) ? 1 : 0;
    int ret;
    u32 bclk_div;
    u32 bclk_ratio;
    u32 mclk_rate;
    u32 tmp;
    if (data_width < 0)
    return data_width;
    if (!channels || !rate || !physical_width)
    return -EINVAL;
    ret = cv1800b_set_slot_settings(i2s, channels, physical_width, data_width);
    if (ret)
    return ret;
    if (i2s.mclk_rate) {
    mclk_rate = i2s.mclk_rate;
    } else {
    dev_dbg(i2s.dev, "mclk is not set by machine driver\n");
    ret = cv1800b_i2s_set_rate_for_mclk(i2s,
    rate * CV1800B_DEF_MCLK_FS_RATIO);
    if (ret)
    return ret;
    mclk_rate = rate * CV1800B_DEF_MCLK_FS_RATIO;
    }
    bclk_ratio = (i2s.bclk_ratio_fixed) ? i2s.bclk_ratio :
    (physical_width * channels);
    if (check_mul_overflow(rate, bclk_ratio, &tmp))
    return -EOVERFLOW;
    if (!tmp)
    return -EINVAL;
    if (mclk_rate % tmp)
    dev_warn(i2s.dev, "mclk rate is not aligned to bclk or rate\n");
    bclk_div = DIV_ROUND_CLOSEST(mclk_rate, tmp);
    ret = cv1800b_set_bclk_div(i2s, bclk_div);
    if (ret)
    return ret;
    ret = cv1800b_set_word_length(i2s, physical_width);
    if (ret)
    return ret;
    cv1800b_set_tx_mode(i2s, tx_mode);
    cv1800b_reset_fifo(i2s);
    cv1800b_reset_i2s(i2s);
    return 0;
    }
    static int cv1800b_i2s_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    u32 val;
    val = readl(i2s.base + CV1800B_I2S_ENABLE);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    val = u32_replace_bits(val, 1, I2S_ENABLE_MASK);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    val = u32_replace_bits(val, 0, I2S_ENABLE_MASK);
    break;
    default:
    return -EINVAL;
    }
    writel(val, i2s.base + CV1800B_I2S_ENABLE);
    return 0;
    }
    static int cv1800b_i2s_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    struct snd_soc_dai_link *dai_link = rtd.dai_link;
    dev_dbg(i2s.dev, "%s: dai=%s substream=%d\n", __func__, dai.name,
    substream.stream);
//
// Ensure DMA is stopped before DAI
// shutdown (prevents DW AXI DMAC stop/busy on next open).
//
    dai_link.trigger_stop = SND_SOC_TRIGGER_ORDER_LDC;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int cv1800b_i2s_dai_probe(struct snd_soc_dai *dai)
    {
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    if (!i2s) {
    dev_err(dai.dev, "no drvdata in DAI probe\n");
    return -ENODEV;
    }
    snd_soc_dai_init_dma_data(dai, &i2s.playback_dma, &i2s.capture_dma);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_i2s_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int cv1800b_i2s_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    u32 val;
    u32 master;
// only i2s format is supported
    if ((fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S)
    return -EINVAL;
    switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    dev_dbg(i2s.dev, "set to master mode\n");
    master = 1;
    break;
    case SND_SOC_DAIFMT_CBC_CFC:
    dev_dbg(i2s.dev, "set to slave mode\n");
    master = 0;
    break;
    default:
    return -EINVAL;
    }
    val = readl(i2s.base + CV1800B_BLK_MODE_SETTING);
    val = u32_replace_bits(val, master, BLK_MASTER_MODE_MASK);
    writel(val, i2s.base + CV1800B_BLK_MODE_SETTING);
    return 0;
    }
    static int cv1800b_i2s_dai_set_bclk_ratio(struct snd_soc_dai *dai,
    unsigned int ratio)
    {
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    if (ratio == 0)
    return -EINVAL;
    i2s.bclk_ratio = ratio;
    i2s.bclk_ratio_fixed = true;
    return 0;
    }
    static int cv1800b_i2s_dai_set_sysclk(struct snd_soc_dai *dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct cv1800b_i2s *i2s = snd_soc_dai_get_drvdata(dai);
    int ret;
    u32 val;
    let mut output_enable: bool = (dir == SND_SOC_CLOCK_OUT) ? true : false;
    dev_dbg(i2s.dev, "%s called with %u\n", __func__, freq);
    ret = cv1800b_i2s_set_rate_for_mclk(i2s, freq);
    if (ret)
    return ret;
    val = readl(i2s.base + CV1800B_CLK_CTRL0);
    val = u32_replace_bits(val, output_enable, CLK_MCLK_OUT_EN_MASK);
    writel(val, i2s.base + CV1800B_CLK_CTRL0);
    i2s.mclk_rate = freq;
    return 0;
    }
    static const struct snd_soc_dai_ops cv1800b_i2s_dai_ops = {
    .probe = cv1800b_i2s_dai_probe,
    .startup = cv1800b_i2s_startup,
    .hw_params = cv1800b_i2s_hw_params,
    .trigger = cv1800b_i2s_trigger,
    .set_fmt = cv1800b_i2s_dai_set_fmt,
    .set_bclk_ratio = cv1800b_i2s_dai_set_bclk_ratio,
    .set_sysclk = cv1800b_i2s_dai_set_sysclk,
    };
    static struct snd_soc_dai_driver cv1800b_i2s_dai_template = {
    .name = "cv1800b-i2s",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
    },
    .ops = &cv1800b_i2s_dai_ops,
    };
    static const struct snd_soc_component_driver cv1800b_i2s_component = {
    .name = "cv1800b-i2s",
    };
#[no_mangle]
unsafe extern "C" fn cv1800b_i2s_hw_disable(i2s: *mut cv1800b_i2s) {
    static void cv1800b_i2s_hw_disable(struct cv1800b_i2s *i2s)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_I2S_ENABLE);
    val = u32_replace_bits(val, 0, I2S_ENABLE_MASK);
    writel(val, i2s.base + CV1800B_I2S_ENABLE);
    val = readl(i2s.base + CV1800B_CLK_CTRL0);
    val = u32_replace_bits(val, 0, CLK_AUD_EN_MASK);
    val = u32_replace_bits(val, 0, CLK_MCLK_OUT_EN_MASK);
    writel(val, i2s.base + CV1800B_CLK_CTRL0);
    val = readl(i2s.base + CV1800B_I2S_RESET);
    val = u32_replace_bits(val, 1, RST_I2S_RESET_RX_MASK);
    val = u32_replace_bits(val, 1, RST_I2S_RESET_TX_MASK);
    writel(val, i2s.base + CV1800B_I2S_RESET);
    val = readl(i2s.base + CV1800B_FIFO_RESET);
    val = u32_replace_bits(val, 1, FIFO_RX_RESET_MASK);
    val = u32_replace_bits(val, 1, FIFO_TX_RESET_MASK);
    writel(val, i2s.base + CV1800B_FIFO_RESET);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_i2s_setup_tdm(i2s: *mut cv1800b_i2s) {
    static void cv1800b_i2s_setup_tdm(struct cv1800b_i2s *i2s)
    {
    u32 val;
    val = readl(i2s.base + CV1800B_BLK_MODE_SETTING);
    val = u32_replace_bits(val, 1, BLK_DMA_MODE_MASK);
    writel(val, i2s.base + CV1800B_BLK_MODE_SETTING);
    val = readl(i2s.base + CV1800B_CLK_CTRL0);
    val = u32_replace_bits(val, 0, CLK_AUD_CLK_SEL_MASK);
    val = u32_replace_bits(val, 0, CLK_MCLK_OUT_EN_MASK);
    val = u32_replace_bits(val, 0, CLK_AUD_EN_MASK);
    writel(val, i2s.base + CV1800B_CLK_CTRL0);
    val = readl(i2s.base + CV1800B_FIFO_THRESHOLD);
    val = u32_replace_bits(val, 4, FIFO_RX_THRESHOLD_MASK);
    val = u32_replace_bits(val, 4, FIFO_TX_THRESHOLD_MASK);
    val = u32_replace_bits(val, 4, FIFO_TX_HIGH_THRESHOLD_MASK);
    writel(val, i2s.base + CV1800B_FIFO_THRESHOLD);
    val = readl(i2s.base + CV1800B_I2S_ENABLE);
    val = u32_replace_bits(val, 0, I2S_ENABLE_MASK);
    writel(val, i2s.base + CV1800B_I2S_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_i2s_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800b_i2s_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cv1800b_i2s *i2s;
    struct resource *res;
    void __iomem *regs;
    int ret;
    i2s = devm_kzalloc(dev, sizeof(*i2s), GFP_KERNEL);
    if (!i2s)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    i2s.dev = &pdev.dev;
    i2s.base = regs;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    cv1800b_setup_dma_struct(i2s, res.start);
    i2s.clk = devm_clk_get_enabled(dev, "i2s");
    if (IS_ERR(i2s.clk))
    return dev_err_probe(dev, PTR_ERR(i2s.clk),
    "failed to get+enable i2s\n");
    i2s.sysclk = devm_clk_get_enabled(dev, "mclk");
    if (IS_ERR(i2s.sysclk))
    return dev_err_probe(dev, PTR_ERR(i2s.sysclk),
    "failed to get+enable mclk\n");
    platform_set_drvdata(pdev, i2s);
    cv1800b_i2s_setup_tdm(i2s);
    ret = devm_snd_soc_register_component(dev, &cv1800b_i2s_component,
    &cv1800b_i2s_dai_template, 1);
    if (ret)
    return ret;
    ret = devm_snd_dmaengine_pcm_register(dev, &cv1800b_i2s_pcm_config, 0);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_i2s_remove(pdev: *mut platform_device) {
    static void cv1800b_i2s_remove(struct platform_device *pdev)
    {
    struct cv1800b_i2s *i2s = platform_get_drvdata(pdev);
    if (!i2s)
    return;
    cv1800b_i2s_hw_disable(i2s);
    }
    static const struct of_device_id cv1800b_i2s_of_match[] = {
    { .compatible = "sophgo,cv1800b-i2s" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, cv1800b_i2s_of_match);
    static struct platform_driver cv1800b_i2s_driver = {
    .probe = cv1800b_i2s_probe,
    .remove = cv1800b_i2s_remove,
    .driver = {
    .name = "cv1800b-i2s",
    .of_match_table = cv1800b_i2s_of_match,
    },
    };
    module_platform_driver(cv1800b_i2s_driver);
    MODULE_DESCRIPTION("Sophgo cv1800b I2S/TDM driver");
    MODULE_AUTHOR("Anton D. Stavinsky <stavinsky@gmail.com>");
    MODULE_LICENSE("GPL");
