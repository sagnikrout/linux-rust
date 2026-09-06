//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/nxp-sar-adc.c
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
// NXP SAR-ADC driver (adapted from Freescale Vybrid vf610 ADC driver
// by Fugang Duan <B38611@freescale.com>)
//
// Copyright 2013 Freescale Semiconductor, Inc.
// Copyright 2017, 2020-2025 NXP
// Copyright 2025, Linaro Ltd
//

// SAR ADC registers.

// Main Configuration Register

// Main Status Register

// Interrupt Status Register

// Channel Pending Register

// Interrupt Mask Register

// Channel Interrupt Mask Register

// DMA Setting Register

// DMA Control register

// Conversion Timing Register

pub const NXP_SAR_ADC_CTR_INPSAMP_MIN: c_uint = 0x08;
pub const NXP_SAR_ADC_CTR_INPSAMP_MAX: c_uint = 0xff;
// Normal Conversion Mask Register

// Normal Conversion Mask Register field define

// Other field define

pub const NXP_SAR_ADC_RESOLUTION: c_int = 12;
// Duration of conversion phases
pub const NXP_SAR_ADC_TPT: c_int = 2;
pub const NXP_SAR_ADC_DP: c_int = 2;

pub const NXP_SAR_ADC_NR_CHANNELS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_sar_adc {
    pub regs: *mut void __iomem,
    pub regs_phys: phys_addr_t,
    pub current_channel: u8,
    pub channels_used: u8,
    pub value: u16,
    pub vref_mV: u32,
// Save and restore context.
    pub inpsamp: u32,
    pub pwdn: u32,
    pub clk: *mut clk,
    pub dma_chan: *mut dma_chan,
    pub completion: completion,
    pub dma_buf: circ_buf,
    pub rx_dma_buf: dma_addr_t,
    pub cookie: dma_cookie_t,
// Protect circular buffers access.
    pub lock: spinlock_t,
// Array of enabled channels.
    pub buffered_chan: [u16; NXP_SAR_ADC_NR_CHANNELS],
// Buffer to be filled by the DMA.
    pub NXP_SAR_ADC_NR_CHANNELS): IIO_DECLARE_BUFFER_WITH_TS(u16, buffer,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_sar_adc_data {
    pub vref_mV: u32,
    pub model: *const c_char,
}

    .type = (_chan_type),					\
    .indexed = 1,						\
    .channel = (_idx),					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),	\
    .scan_index = (_idx),					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = 12,					\
    .storagebits = 16,				\
    },							\
    }
    static const struct iio_chan_spec nxp_sar_adc_iio_channels[] = {
    ADC_CHAN(0, IIO_VOLTAGE),
    ADC_CHAN(1, IIO_VOLTAGE),
    ADC_CHAN(2, IIO_VOLTAGE),
    ADC_CHAN(3, IIO_VOLTAGE),
    ADC_CHAN(4, IIO_VOLTAGE),
    ADC_CHAN(5, IIO_VOLTAGE),
    ADC_CHAN(6, IIO_VOLTAGE),
    ADC_CHAN(7, IIO_VOLTAGE),
//
// The NXP SAR ADC documentation marks the channels 8 to 31 as
// "Reserved". Reflect the same in the driver in case new ADC
// variants comes with more channels.
//
    IIO_CHAN_SOFT_TIMESTAMP(32),
    };
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_irq_cfg(info: *mut nxp_sar_adc, enable: bool) {
    static void nxp_sar_adc_irq_cfg(struct nxp_sar_adc *info, bool enable)
    {
    if (enable)
    writel(NXP_SAR_ADC_ISR_ECH, NXP_SAR_ADC_IMR(info.regs));
    else
    writel(0, NXP_SAR_ADC_IMR(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_wait_for(info: *mut nxp_sar_adc, cycles: u64) {
    static void nxp_sar_adc_wait_for(struct nxp_sar_adc *info, u64 cycles)
    {
    u64 rate;
    rate = clk_get_rate(info.clk);
    if (rate)
    ndelay(div64_u64(NSEC_PER_SEC * cycles, rate));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_set_enabled(info: *mut nxp_sar_adc, enable: bool) -> bool {
    static bool nxp_sar_adc_set_enabled(struct nxp_sar_adc *info, bool enable)
    {
    u32 mcr;
    bool pwdn;
    mcr = readl(NXP_SAR_ADC_MCR(info.regs));
//
// Get the current state and return it later. This is used for
// suspend/resume to get the power state
//
    pwdn = FIELD_GET(NXP_SAR_ADC_MCR_PWDN, mcr);
// When the enabled flag is not set, we set the power down bit
    FIELD_MODIFY(NXP_SAR_ADC_MCR_PWDN, &mcr, !enable);
    writel(mcr, NXP_SAR_ADC_MCR(info.regs));
//
// Ensure there are at least three cycles between the
// configuration of NCMR and the setting of NSTART.
//
    if (enable)
    nxp_sar_adc_wait_for(info, 3);
    return pwdn;
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_sar_adc_enable(info: *mut nxp_sar_adc) -> bool {
    static inline bool nxp_sar_adc_enable(struct nxp_sar_adc *info)
    {
    return nxp_sar_adc_set_enabled(info, true);
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_sar_adc_disable(info: *mut nxp_sar_adc) -> bool {
    static inline bool nxp_sar_adc_disable(struct nxp_sar_adc *info)
    {
    return nxp_sar_adc_set_enabled(info, false);
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_sar_adc_calibration_start(base: *mut void __iomem) {
    static inline void nxp_sar_adc_calibration_start(void __iomem *base)
    {
    let mut mcr: u32 = readl(NXP_SAR_ADC_MCR(base));
    FIELD_MODIFY(NXP_SAR_ADC_MCR_CALSTART, &mcr, 0x1);
    writel(mcr, NXP_SAR_ADC_MCR(base));
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_sar_adc_calibration_wait(base: *mut void __iomem) -> c_int {
    static inline int nxp_sar_adc_calibration_wait(void __iomem *base)
    {
    u32 msr;
    int ret;
    ret = readl_poll_timeout(NXP_SAR_ADC_MSR(base), msr,
    !FIELD_GET(NXP_SAR_ADC_MSR_CALBUSY, msr),
    NXP_SAR_ADC_WAIT_US,
    NXP_SAR_ADC_CAL_TIMEOUT_US);
    if (ret)
    return ret;
    if (FIELD_GET(NXP_SAR_ADC_MSR_CALFAIL, msr)) {
//
// If the calibration fails, the status register bit must be
// cleared.
//
    FIELD_MODIFY(NXP_SAR_ADC_MSR_CALFAIL, &msr, 0x0);
    writel(msr, NXP_SAR_ADC_MSR(base));
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_calibration(info: *mut nxp_sar_adc) -> c_int {
    static int nxp_sar_adc_calibration(struct nxp_sar_adc *info)
    {
    int ret;
// Calibration works only if the ADC is powered up.
    nxp_sar_adc_enable(info);
// The calibration operation starts.
    nxp_sar_adc_calibration_start(info.regs);
    ret = nxp_sar_adc_calibration_wait(info.regs);
//
// Calibration works only if the ADC is powered up. However
// the calibration is called from the probe function where the
// iio is not enabled, so we disable after the calibration.
//
    nxp_sar_adc_disable(info);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_conversion_timing_set(info: *mut nxp_sar_adc, inpsamp: u32) {
    static void nxp_sar_adc_conversion_timing_set(struct nxp_sar_adc *info, u32 inpsamp)
    {
    inpsamp = clamp(inpsamp, NXP_SAR_ADC_CTR_INPSAMP_MIN, NXP_SAR_ADC_CTR_INPSAMP_MAX);
    writel(inpsamp, NXP_SAR_ADC_CTR0(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_conversion_timing_get(info: *mut nxp_sar_adc) -> u32 {
    static u32 nxp_sar_adc_conversion_timing_get(struct nxp_sar_adc *info)
    {
    return readl(NXP_SAR_ADC_CTR0(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_read_notify(info: *mut nxp_sar_adc) {
    static void nxp_sar_adc_read_notify(struct nxp_sar_adc *info)
    {
    writel(NXP_SAR_ADC_CH_MASK, NXP_SAR_ADC_CEOCFR0(info.regs));
    writel(NXP_SAR_ADC_CH_MASK, NXP_SAR_ADC_CEOCFR1(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_read_data(info: *mut nxp_sar_adc, chan: c_uint) -> c_int {
    static int nxp_sar_adc_read_data(struct nxp_sar_adc *info, unsigned int chan)
    {
    u32 ceocfr, cdr;
    ceocfr = readl(NXP_SAR_ADC_CEOCFR0(info.regs));
    if (!field_get(NXP_SAR_ADC_EOC_CH(chan), ceocfr))
    return -EIO;
    cdr = readl(NXP_SAR_ADC_CDR(info.regs, chan));
    if (!(FIELD_GET(NXP_SAR_ADC_CDR_VALID, cdr)))
    return -EIO;
    return FIELD_GET(NXP_SAR_ADC_CDR_CDATA_MASK, cdr);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_isr_buffer(indio_dev: *mut iio_dev) {
    static void nxp_sar_adc_isr_buffer(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    unsigned int i;
    int ret;
    for (i = 0; i < info.channels_used; i++) {
    ret = nxp_sar_adc_read_data(info, info.buffered_chan[i]);
    if (ret < 0) {
    nxp_sar_adc_read_notify(info);
    iio_trigger_notify_done(indio_dev.trig);
    return;
    }
    info.buffer[i] = ret;
    }
    nxp_sar_adc_read_notify(info);
    iio_push_to_buffers_with_ts(indio_dev, info.buffer, sizeof(info.buffer),
    iio_get_time_ns(indio_dev));
    iio_trigger_notify_done(indio_dev.trig);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_isr_read_raw(indio_dev: *mut iio_dev) {
    static void nxp_sar_adc_isr_read_raw(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    int ret;
    ret = nxp_sar_adc_read_data(info, info.current_channel);
    nxp_sar_adc_read_notify(info);
    if (ret < 0)
    return;
    info.value = ret;
    complete(&info.completion);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t nxp_sar_adc_isr(int irq, void *dev_id)
    {
    struct iio_dev *indio_dev = dev_id;
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    int isr;
    isr = readl(NXP_SAR_ADC_ISR(info.regs));
    if (!(FIELD_GET(NXP_SAR_ADC_ISR_ECH, isr)))
    return IRQ_NONE;
    if (iio_buffer_enabled(indio_dev))
    nxp_sar_adc_isr_buffer(indio_dev);
    else
    nxp_sar_adc_isr_read_raw(indio_dev);
    writel(NXP_SAR_ADC_ISR_ECH, NXP_SAR_ADC_ISR(info.regs));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_channels_disable(info: *mut nxp_sar_adc, mask: u32) {
    static void nxp_sar_adc_channels_disable(struct nxp_sar_adc *info, u32 mask)
    {
    u32 ncmr, cimr;
    ncmr = readl(NXP_SAR_ADC_NCMR0(info.regs));
    cimr = readl(NXP_SAR_ADC_CIMR0(info.regs));
// FIELD_MODIFY() can not be used because the mask is not constant
    ncmr &= ~mask;
    cimr &= ~mask;
    writel(ncmr, NXP_SAR_ADC_NCMR0(info.regs));
    writel(cimr, NXP_SAR_ADC_CIMR0(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_channels_enable(info: *mut nxp_sar_adc, mask: u32) {
    static void nxp_sar_adc_channels_enable(struct nxp_sar_adc *info, u32 mask)
    {
    u32 ncmr, cimr;
    ncmr = readl(NXP_SAR_ADC_NCMR0(info.regs));
    cimr = readl(NXP_SAR_ADC_CIMR0(info.regs));
    ncmr |= mask;
    cimr |= mask;
    writel(ncmr, NXP_SAR_ADC_NCMR0(info.regs));
    writel(cimr, NXP_SAR_ADC_CIMR0(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_dma_channels_enable(info: *mut nxp_sar_adc, mask: u32) {
    static void nxp_sar_adc_dma_channels_enable(struct nxp_sar_adc *info, u32 mask)
    {
    u32 dmar;
    dmar = readl(NXP_SAR_ADC_DMAR0(info.regs));
    dmar |= mask;
    writel(dmar, NXP_SAR_ADC_DMAR0(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_dma_channels_disable(info: *mut nxp_sar_adc, mask: u32) {
    static void nxp_sar_adc_dma_channels_disable(struct nxp_sar_adc *info, u32 mask)
    {
    u32 dmar;
    dmar = readl(NXP_SAR_ADC_DMAR0(info.regs));
    dmar &= ~mask;
    writel(dmar, NXP_SAR_ADC_DMAR0(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_dma_cfg(info: *mut nxp_sar_adc, enable: bool) {
    static void nxp_sar_adc_dma_cfg(struct nxp_sar_adc *info, bool enable)
    {
    u32 dmae;
    dmae = readl(NXP_SAR_ADC_DMAE(info.regs));
    FIELD_MODIFY(NXP_SAR_ADC_DMAE_DMAEN, &dmae, enable);
    writel(dmae, NXP_SAR_ADC_DMAE(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_stop_conversion(info: *mut nxp_sar_adc) {
    static void nxp_sar_adc_stop_conversion(struct nxp_sar_adc *info)
    {
    u32 mcr;
    mcr = readl(NXP_SAR_ADC_MCR(info.regs));
    FIELD_MODIFY(NXP_SAR_ADC_MCR_NSTART, &mcr, 0x0);
    writel(mcr, NXP_SAR_ADC_MCR(info.regs));
//
// On disable, we have to wait for the transaction to finish.
// ADC does not abort the transaction if a chain conversion is
// in progress. Wait for the worst case scenario - 80 ADC clk
// cycles. The clock rate is 80MHz, this routine is called
// only when the capture finishes. The delay will be very
// short, usec-ish, which is acceptable in the atomic context.
//
    nxp_sar_adc_wait_for(info, 80);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_start_conversion(info: *mut nxp_sar_adc, raw: bool) -> c_int {
    static int nxp_sar_adc_start_conversion(struct nxp_sar_adc *info, bool raw)
    {
    u32 mcr;
    mcr = readl(NXP_SAR_ADC_MCR(info.regs));
    FIELD_MODIFY(NXP_SAR_ADC_MCR_NSTART, &mcr, 0x1);
    FIELD_MODIFY(NXP_SAR_ADC_MCR_MODE, &mcr, raw ? 0 : 1);
    writel(mcr, NXP_SAR_ADC_MCR(info.regs));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_read_channel(info: *mut nxp_sar_adc, channel: c_int) -> c_int {
    static int nxp_sar_adc_read_channel(struct nxp_sar_adc *info, int channel)
    {
    int ret;
    info.current_channel = channel;
    nxp_sar_adc_channels_enable(info, BIT(channel));
    nxp_sar_adc_irq_cfg(info, true);
    nxp_sar_adc_enable(info);
    reinit_completion(&info.completion);
    ret = nxp_sar_adc_start_conversion(info, true);
    if (ret < 0)
    goto out_disable;
    if (!wait_for_completion_interruptible_timeout(&info.completion,
    NXP_SAR_ADC_CONV_TIMEOUT))
    ret = -ETIMEDOUT;
    nxp_sar_adc_stop_conversion(info);
    out_disable:
    nxp_sar_adc_channels_disable(info, BIT(channel));
    nxp_sar_adc_irq_cfg(info, false);
    nxp_sar_adc_disable(info);
    return ret;
    }
    static int nxp_sar_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    u32 inpsamp;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = nxp_sar_adc_read_channel(info, chan.channel);
    iio_device_release_direct(indio_dev);
    if (ret)
    return ret;
// val = info->value;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = info->vref_mV;
// val2 = NXP_SAR_ADC_RESOLUTION;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ:
    inpsamp = nxp_sar_adc_conversion_timing_get(info);
// val = clk_get_rate(info->clk) / (inpsamp + NXP_SAR_ADC_CONV_TIME);
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int nxp_sar_adc_write_raw(struct iio_dev *indio_dev, struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    u32 inpsamp;
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    if (val <= 0)
    return -EINVAL;
//
// Configures the sample period duration in terms of the SAR
// controller clock. The minimum acceptable value is 8.
// Configuring it to a value lower than 8 sets the sample period
// to 8 cycles.  We read the clock value and divide by the
// sampling timing which gives us the number of cycles expected.
// The value is 8-bit wide, consequently the max value is 0xFF.
//
    inpsamp = clk_get_rate(info.clk) / val;
    if (inpsamp < NXP_SAR_ADC_CONV_TIME)
    return -EINVAL;
    inpsamp -= NXP_SAR_ADC_CONV_TIME;
    nxp_sar_adc_conversion_timing_set(info, inpsamp);
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_dma_cb(data: *mut c_void) {
    static void nxp_sar_adc_dma_cb(void *data)
    {
    struct iio_dev *indio_dev = data;
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    struct dma_tx_state state;
    struct circ_buf *dma_buf;
    struct device *dev_dma;
    u32 *dma_samples;
    s64 timestamp;
    int idx, ret;
    guard(spinlock_irqsave)(&info.lock);
    dma_buf = &info.dma_buf;
    dma_samples = (u32 *)dma_buf.buf;
    dev_dma = info.dma_chan.device.dev;
//
// DMA in some corner cases might have already be charged for
// the next transfer. Potentially there can be a race where
// the residue changes while the dma engine updates the
// buffer. That could be handled by using the
// callback_result() instead of callback() because the residue
// will be passed as a parameter to the function. However this
// new callback is pretty new and the backend does not update
// the residue. So let's stick to the version other drivers do
// which has proven running well in production since several
// years.
//
    dmaengine_tx_status(info.dma_chan, info.cookie, &state);
    dma_sync_single_for_cpu(dev_dma, info.rx_dma_buf,
    NXP_SAR_ADC_DMA_BUFF_SZ, DMA_FROM_DEVICE);
// Current head position.
    dma_buf.head = (NXP_SAR_ADC_DMA_BUFF_SZ - state.residue) /
    NXP_SAR_ADC_DMA_SAMPLE_SZ;
// If everything was transferred, avoid an off by one error.
    if (!state.residue)
    dma_buf.head--;
// Something went wrong and nothing transferred.
    if (state.residue != NXP_SAR_ADC_DMA_BUFF_SZ) {
// Make sure that head is multiple of info->channels_used.
    dma_buf.head -= dma_buf.head % info.channels_used;
//
// dma_buf->tail != dma_buf->head condition will become false
// because dma_buf->tail will be incremented with 1.
//
    while (dma_buf.tail != dma_buf.head) {
    idx = dma_buf.tail % info.channels_used;
    info.buffer[idx] = dma_samples[dma_buf.tail];
    dma_buf.tail = (dma_buf.tail + 1) % NXP_SAR_ADC_DMA_SAMPLE_CNT;
    if (idx != info.channels_used - 1)
    continue;
//
// iio_push_to_buffers_with_ts() should not be
// called with dma_samples as parameter. The samples
// will be smashed if timestamp is enabled.
//
    timestamp = iio_get_time_ns(indio_dev);
    ret = iio_push_to_buffers_with_ts(indio_dev, info.buffer,
    sizeof(info.buffer),
    timestamp);
    if (ret < 0 && ret != -EBUSY)
    dev_err_ratelimited(&indio_dev.dev,
    "failed to push iio buffer: %d",
    ret);
    }
    dma_buf.tail = dma_buf.head;
    }
    dma_sync_single_for_device(dev_dma, info.rx_dma_buf,
    NXP_SAR_ADC_DMA_BUFF_SZ, DMA_FROM_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_start_cyclic_dma(indio_dev: *mut iio_dev) -> c_int {
    static int nxp_sar_adc_start_cyclic_dma(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    let mut config: dma_slave_config = { };
    struct dma_async_tx_descriptor *desc;
    int ret;
    info.dma_buf.head = 0;
    info.dma_buf.tail = 0;
    config.direction = DMA_DEV_TO_MEM;
    config.src_addr_width = NXP_SAR_ADC_DMA_SAMPLE_SZ;
    config.src_addr = NXP_SAR_ADC_CDR(info.regs_phys, info.buffered_chan[0]);
    config.src_port_window_size = info.channels_used;
    config.src_maxburst = info.channels_used;
    ret = dmaengine_slave_config(info.dma_chan, &config);
    if (ret < 0)
    return ret;
    desc = dmaengine_prep_dma_cyclic(info.dma_chan,
    info.rx_dma_buf,
    NXP_SAR_ADC_DMA_BUFF_SZ,
    NXP_SAR_ADC_DMA_BUFF_SZ / 2,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT);
    if (!desc)
    return -EINVAL;
    desc.callback = nxp_sar_adc_dma_cb;
    desc.callback_param = indio_dev;
    info.cookie = dmaengine_submit(desc);
    ret = dma_submit_error(info.cookie);
    if (ret) {
    dmaengine_terminate_async(info.dma_chan);
    return ret;
    }
    dma_async_issue_pending(info.dma_chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_buffer_software_do_predisable(indio_dev: *mut iio_dev) {
    static void nxp_sar_adc_buffer_software_do_predisable(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
//
// The ADC DMAEN bit should be cleared before DMA transaction
// is canceled.
//
    nxp_sar_adc_stop_conversion(info);
    dmaengine_terminate_sync(info.dma_chan);
    nxp_sar_adc_dma_cfg(info, false);
    nxp_sar_adc_dma_channels_disable(info, *indio_dev.active_scan_mask);
    dma_release_channel(info.dma_chan);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_buffer_software_do_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int nxp_sar_adc_buffer_software_do_postenable(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    int ret;
    info.dma_chan = dma_request_chan(indio_dev.dev.parent, "rx");
    if (IS_ERR(info.dma_chan))
    return PTR_ERR(info.dma_chan);
    nxp_sar_adc_dma_channels_enable(info, *indio_dev.active_scan_mask);
    nxp_sar_adc_dma_cfg(info, true);
    ret = nxp_sar_adc_start_cyclic_dma(indio_dev);
    if (ret)
    goto out_dma_channels_disable;
    ret = nxp_sar_adc_start_conversion(info, false);
    if (ret)
    goto out_stop_cyclic_dma;
    return 0;
    out_stop_cyclic_dma:
    dmaengine_terminate_sync(info.dma_chan);
    out_dma_channels_disable:
    nxp_sar_adc_dma_cfg(info, false);
    nxp_sar_adc_dma_channels_disable(info, *indio_dev.active_scan_mask);
    dma_release_channel(info.dma_chan);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_buffer_trigger_do_predisable(indio_dev: *mut iio_dev) {
    static void nxp_sar_adc_buffer_trigger_do_predisable(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    nxp_sar_adc_irq_cfg(info, false);
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_buffer_trigger_do_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int nxp_sar_adc_buffer_trigger_do_postenable(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    nxp_sar_adc_irq_cfg(info, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int nxp_sar_adc_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    let mut current_mode: c_int = iio_device_get_current_mode(indio_dev);
    unsigned long channel;
    int ret;
    info.channels_used = 0;
//
// The SAR-ADC has two groups of channels.
//
// - Group #0:
// * bit 0-7  : channel 0 -> channel 7
// * bit 8-31 : reserved
//
// - Group #32:
// * bit 0-7  : Internal
// * bit 8-31 : reserved
//
// The 8 channels from group #0 are used in this driver for
// ADC as described when declaring the IIO device and the
// mapping is the same. That means the active_scan_mask can be
// used directly to write the channel interrupt mask.
//
    nxp_sar_adc_channels_enable(info, *indio_dev.active_scan_mask);
    for_each_set_bit(channel, indio_dev.active_scan_mask, NXP_SAR_ADC_NR_CHANNELS)
    info.buffered_chan[info.channels_used++] = channel;
    nxp_sar_adc_enable(info);
    if (current_mode == INDIO_BUFFER_SOFTWARE)
    ret = nxp_sar_adc_buffer_software_do_postenable(indio_dev);
    else
    ret = nxp_sar_adc_buffer_trigger_do_postenable(indio_dev);
    if (ret)
    goto out_postenable;
    return 0;
    out_postenable:
    nxp_sar_adc_disable(info);
    nxp_sar_adc_channels_disable(info, *indio_dev.active_scan_mask);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int nxp_sar_adc_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    let mut currentmode: c_int = iio_device_get_current_mode(indio_dev);
    if (currentmode == INDIO_BUFFER_SOFTWARE)
    nxp_sar_adc_buffer_software_do_predisable(indio_dev);
    else
    nxp_sar_adc_buffer_trigger_do_predisable(indio_dev);
    nxp_sar_adc_disable(info);
    nxp_sar_adc_channels_disable(info, *indio_dev.active_scan_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t nxp_sar_adc_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct nxp_sar_adc *info = iio_priv(indio_dev);
    int ret;
    ret = nxp_sar_adc_start_conversion(info, true);
    if (ret < 0)
    dev_dbg(&indio_dev.dev, "Failed to start conversion\n");
    return IRQ_HANDLED;
    }
    static const struct iio_buffer_setup_ops iio_triggered_buffer_setup_ops = {
    .postenable = nxp_sar_adc_buffer_postenable,
    .predisable = nxp_sar_adc_buffer_predisable,
    };
    static const struct iio_info nxp_sar_adc_iio_info = {
    .read_raw  = nxp_sar_adc_read_raw,
    .write_raw = nxp_sar_adc_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_dma_probe(dev: *mut device, info: *mut nxp_sar_adc) -> c_int {
    static int nxp_sar_adc_dma_probe(struct device *dev, struct nxp_sar_adc *info)
    {
    u8 *rx_buf;
    rx_buf = dmam_alloc_coherent(dev, NXP_SAR_ADC_DMA_BUFF_SZ,
    &info.rx_dma_buf, GFP_KERNEL);
    if (!rx_buf)
    return -ENOMEM;
    info.dma_buf.buf = rx_buf;
    return 0;
    }
//
// The documentation describes the reset values for the registers.
// However some registers do not have these values after a reset. It
// is not a desirable situation. In some other SoC family
// documentation NXP recommends not assuming the default values are
// set and to initialize the registers conforming to the documentation
// reset information to prevent this situation. Assume the same rule
// applies here as there is a discrepancy between what is read from
// the registers at reset time and the documentation.
//
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_set_default_values(info: *mut nxp_sar_adc) {
    static void nxp_sar_adc_set_default_values(struct nxp_sar_adc *info)
    {
    writel(0x00003901, NXP_SAR_ADC_MCR(info.regs));
    writel(0x00000001, NXP_SAR_ADC_MSR(info.regs));
    writel(0x00000014, NXP_SAR_ADC_CTR0(info.regs));
    writel(0x00000014, NXP_SAR_ADC_CTR1(info.regs));
    writel(0x00000000, NXP_SAR_ADC_CIMR0(info.regs));
    writel(0x00000000, NXP_SAR_ADC_CIMR1(info.regs));
    writel(0x00000000, NXP_SAR_ADC_NCMR0(info.regs));
    writel(0x00000000, NXP_SAR_ADC_NCMR1(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_probe(pdev: *mut platform_device) -> c_int {
    static int nxp_sar_adc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct nxp_sar_adc_data *data = device_get_match_data(dev);
    struct nxp_sar_adc *info;
    struct iio_dev *indio_dev;
    struct resource *mem;
    int irq, ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*info));
    if (!indio_dev)
    return -ENOMEM;
    info = iio_priv(indio_dev);
    info.vref_mV = data.vref_mV;
    spin_lock_init(&info.lock);
    info.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mem);
    if (IS_ERR(info.regs))
    return dev_err_probe(dev, PTR_ERR(info.regs),
    "Failed to get and remap resource");
    info.regs_phys = mem.start;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, nxp_sar_adc_isr, 0, dev_name(dev),
    indio_dev);
    if (ret < 0)
    return ret;
    info.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(info.clk))
    return dev_err_probe(dev, PTR_ERR(info.clk),
    "Failed to get the clock\n");
    platform_set_drvdata(pdev, indio_dev);
    init_completion(&info.completion);
    indio_dev.name = data.model;
    indio_dev.info = &nxp_sar_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE | INDIO_BUFFER_SOFTWARE;
    indio_dev.channels = nxp_sar_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(nxp_sar_adc_iio_channels);
    nxp_sar_adc_set_default_values(info);
    ret = nxp_sar_adc_calibration(info);
    if (ret)
    dev_err_probe(dev, ret, "Calibration failed\n");
    ret = nxp_sar_adc_dma_probe(dev, info);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to initialize the DMA\n");
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    &iio_pollfunc_store_time,
    &nxp_sar_adc_trigger_handler,
    &iio_triggered_buffer_setup_ops);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Couldn't initialise the buffer\n");
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret)
    return dev_err_probe(dev, ret, "Couldn't register the device\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_suspend(dev: *mut device) -> c_int {
    static int nxp_sar_adc_suspend(struct device *dev)
    {
    struct nxp_sar_adc *info = iio_priv(dev_get_drvdata(dev));
    info.pwdn = nxp_sar_adc_disable(info);
    info.inpsamp = nxp_sar_adc_conversion_timing_get(info);
    clk_disable_unprepare(info.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_sar_adc_resume(dev: *mut device) -> c_int {
    static int nxp_sar_adc_resume(struct device *dev)
    {
    struct nxp_sar_adc *info = iio_priv(dev_get_drvdata(dev));
    int ret;
    ret = clk_prepare_enable(info.clk);
    if (ret)
    return ret;
    nxp_sar_adc_conversion_timing_set(info, info.inpsamp);
    if (!info.pwdn)
    nxp_sar_adc_enable(info);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(nxp_sar_adc_pm_ops, nxp_sar_adc_suspend,
    nxp_sar_adc_resume);
    static const struct nxp_sar_adc_data s32g2_sar_adc_data = {
    .vref_mV = 1800,
    .model = "s32g2-sar-adc",
    };
    static const struct of_device_id nxp_sar_adc_match[] = {
    { .compatible = "nxp,s32g2-sar-adc", .data = &s32g2_sar_adc_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, nxp_sar_adc_match);
    static struct platform_driver nxp_sar_adc_driver = {
    .probe = nxp_sar_adc_probe,
    .driver = {
    .name = "nxp-sar-adc",
    .of_match_table = nxp_sar_adc_match,
    .pm = pm_sleep_ptr(&nxp_sar_adc_pm_ops),
    },
    };
    module_platform_driver(nxp_sar_adc_driver);
    MODULE_AUTHOR("NXP");
    MODULE_DESCRIPTION("NXP SAR-ADC driver");
    MODULE_LICENSE("GPL");
