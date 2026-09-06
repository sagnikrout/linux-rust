//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-loongson-core.c
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


// SPDX-License-Identifier: GPL-2.0+
// Loongson SPI Support
// Copyright (C) 2023 Loongson Technology Corporation Limited

    static inline void loongson_spi_write_reg(struct loongson_spi *spi, unsigned char reg,
    unsigned char data)
    {
    writeb(data, spi.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn loongson_spi_read_reg(spi: *mut loongson_spi, reg: c_uchar) -> c_char {
    static inline char loongson_spi_read_reg(struct loongson_spi *spi, unsigned char reg)
    {
    return readb(spi.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_set_cs(spi: *mut spi_device, en: bool) {
    static void loongson_spi_set_cs(struct spi_device *spi, bool en)
    {
    int cs;
    let mut mask: c_uchar = (BIT(4) | BIT(0)) << spi_get_chipselect(spi, 0);
    let mut val: c_uchar = en ? mask :  (BIT(0) << spi_get_chipselect(spi, 0));
    struct loongson_spi *loongson_spi = spi_controller_get_devdata(spi.controller);
    cs = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SFCS_REG) & ~mask;
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SFCS_REG, val | cs);
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_set_clk(loongson_spi: *mut loongson_spi, hz: c_uint) {
    static void loongson_spi_set_clk(struct loongson_spi *loongson_spi, unsigned int hz)
    {
    unsigned char val;
    unsigned int div, div_tmp;
    static const char rdiv[12] = {0, 1, 4, 2, 3, 5, 6, 7, 8, 9, 10, 11};
    div = clamp_val(DIV_ROUND_UP_ULL(loongson_spi.clk_rate, hz), 2, 4096);
    div_tmp = rdiv[fls(div - 1)];
    loongson_spi.spcr = (div_tmp & GENMASK(1, 0)) >> 0;
    loongson_spi.sper = (div_tmp & GENMASK(3, 2)) >> 2;
    val = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SPCR_REG);
    val &= ~GENMASK(1, 0);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SPCR_REG, val |
    loongson_spi.spcr);
    val = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SPER_REG);
    val &= ~GENMASK(1, 0);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SPER_REG, val |
    loongson_spi.sper);
    loongson_spi.hz = hz;
    }
    static void loongson_spi_set_mode(struct loongson_spi *loongson_spi,
    struct spi_device *spi)
    {
    unsigned char val;
    val = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SPCR_REG);
    val &= ~(LOONGSON_SPI_SPCR_CPOL | LOONGSON_SPI_SPCR_CPHA);
    if (spi.mode & SPI_CPOL)
    val |= LOONGSON_SPI_SPCR_CPOL;
    if (spi.mode & SPI_CPHA)
    val |= LOONGSON_SPI_SPCR_CPHA;
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SPCR_REG, val);
    loongson_spi.mode |= spi.mode;
    }
    static int loongson_spi_update_state(struct loongson_spi *loongson_spi,
    struct spi_device *spi, struct spi_transfer *t)
    {
    if (t && loongson_spi.hz != t.speed_hz)
    loongson_spi_set_clk(loongson_spi, t.speed_hz);
    if ((spi.mode ^ loongson_spi.mode) & SPI_MODE_X_MASK)
    loongson_spi_set_mode(loongson_spi, spi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_setup(spi: *mut spi_device) -> c_int {
    static int loongson_spi_setup(struct spi_device *spi)
    {
    struct loongson_spi *loongson_spi;
    loongson_spi = spi_controller_get_devdata(spi.controller);
    if (spi.bits_per_word % 8)
    return -EINVAL;
    if (spi_get_chipselect(spi, 0) >= spi.controller.num_chipselect)
    return -EINVAL;
    loongson_spi.hz = 0;
    loongson_spi_set_cs(spi, true);
    return 0;
    }
    static int loongson_spi_write_read_8bit(struct spi_device *spi, const u8 **tx_buf,
    u8 **rx_buf, unsigned int num)
    {
    int ret;
    struct loongson_spi *loongson_spi = spi_controller_get_devdata(spi.controller);
    if (tx_buf && *tx_buf)
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_FIFO_REG, *((*tx_buf)++));
    else
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_FIFO_REG, 0);
    ret = readb_poll_timeout(loongson_spi.base + LOONGSON_SPI_SPSR_REG,
    loongson_spi.spsr, (loongson_spi.spsr &
    LOONGSON_SPI_SPSR_RFEMPTY) != LOONGSON_SPI_SPSR_RFEMPTY,
    1, USEC_PER_MSEC);
    if (rx_buf && *rx_buf)
// (*rx_buf)++ = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_FIFO_REG);
    else
    loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_FIFO_REG);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_write_read(spi: *mut spi_device, xfer: *mut spi_transfer) -> c_int {
    static int loongson_spi_write_read(struct spi_device *spi, struct spi_transfer *xfer)
    {
    int ret;
    unsigned int count;
    const u8 *tx = xfer.tx_buf;
    u8 *rx = xfer.rx_buf;
    count = xfer.len;
    do {
    ret = loongson_spi_write_read_8bit(spi, &tx, &rx, count);
    if (ret)
    break;
    } while (--count);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_prepare_message(ctlr: *mut spi_controller, m: *mut spi_message) -> c_int {
    static int loongson_spi_prepare_message(struct spi_controller *ctlr, struct spi_message *m)
    {
    struct loongson_spi *loongson_spi = spi_controller_get_devdata(ctlr);
    loongson_spi.para = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_PARA_REG);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_PARA_REG, loongson_spi.para &
    ~LOONGSON_SPI_PARA_MEM_EN);
    return 0;
    }
    static int loongson_spi_transfer_one(struct spi_controller *ctrl, struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct loongson_spi *loongson_spi = spi_controller_get_devdata(spi.controller);
    loongson_spi_update_state(loongson_spi, spi, xfer);
    if (xfer.len)
    return loongson_spi_write_read(spi, xfer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_unprepare_message(ctrl: *mut spi_controller, m: *mut spi_message) -> c_int {
    static int loongson_spi_unprepare_message(struct spi_controller *ctrl, struct spi_message *m)
    {
    struct loongson_spi *loongson_spi = spi_controller_get_devdata(ctrl);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_PARA_REG, loongson_spi.para);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_reginit(loongson_spi_dev: *mut loongson_spi) {
    static void loongson_spi_reginit(struct loongson_spi *loongson_spi_dev)
    {
    unsigned char val;
    val = loongson_spi_read_reg(loongson_spi_dev, LOONGSON_SPI_SPCR_REG);
    val &= ~LOONGSON_SPI_SPCR_SPE;
    loongson_spi_write_reg(loongson_spi_dev, LOONGSON_SPI_SPCR_REG, val);
    loongson_spi_write_reg(loongson_spi_dev, LOONGSON_SPI_SPSR_REG,
    (LOONGSON_SPI_SPSR_SPIF | LOONGSON_SPI_SPSR_WCOL));
    val = loongson_spi_read_reg(loongson_spi_dev, LOONGSON_SPI_SPCR_REG);
    val |= LOONGSON_SPI_SPCR_SPE;
    loongson_spi_write_reg(loongson_spi_dev, LOONGSON_SPI_SPCR_REG, val);
    }
#[no_mangle]
pub unsafe extern "C" fn loongson_spi_init_controller(dev: *mut device, regs: *mut void __iomem) -> c_int {
    int loongson_spi_init_controller(struct device *dev, void __iomem *regs)
    {
    struct spi_controller *controller;
    struct loongson_spi *spi;
    struct clk *clk;
    controller = devm_spi_alloc_host(dev, sizeof(struct loongson_spi));
    if (controller == core::ptr::null_mut())
    return -ENOMEM;
    controller.mode_bits = SPI_MODE_X_MASK | SPI_CS_HIGH;
    controller.setup = loongson_spi_setup;
    controller.prepare_message = loongson_spi_prepare_message;
    controller.transfer_one = loongson_spi_transfer_one;
    controller.unprepare_message = loongson_spi_unprepare_message;
    controller.set_cs = loongson_spi_set_cs;
    controller.num_chipselect = 4;
    dev_set_drvdata(dev, controller);
    spi = spi_controller_get_devdata(controller);
    spi.base = regs;
    spi.controller = controller;
    clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "unable to get clock\n");
    spi.clk_rate = clk_get_rate(clk);
    loongson_spi_reginit(spi);
    spi.mode = 0;
    return devm_spi_register_controller(dev, controller);
    }
    EXPORT_SYMBOL_NS_GPL(loongson_spi_init_controller, "SPI_LOONGSON_CORE");
#[no_mangle]
unsafe extern "C" fn loongson_spi_suspend(dev: *mut device) -> c_int {
    static int loongson_spi_suspend(struct device *dev)
    {
    struct loongson_spi *loongson_spi;
    struct spi_controller *controller;
    controller = dev_get_drvdata(dev);
    spi_controller_suspend(controller);
    loongson_spi = spi_controller_get_devdata(controller);
    loongson_spi.spcr = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SPCR_REG);
    loongson_spi.sper = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SPER_REG);
    loongson_spi.spsr = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SPSR_REG);
    loongson_spi.para = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_PARA_REG);
    loongson_spi.sfcs = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_SFCS_REG);
    loongson_spi.timi = loongson_spi_read_reg(loongson_spi, LOONGSON_SPI_TIMI_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_spi_resume(dev: *mut device) -> c_int {
    static int loongson_spi_resume(struct device *dev)
    {
    struct loongson_spi *loongson_spi;
    struct spi_controller *controller;
    controller = dev_get_drvdata(dev);
    loongson_spi = spi_controller_get_devdata(controller);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SPCR_REG, loongson_spi.spcr);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SPER_REG, loongson_spi.sper);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SPSR_REG, loongson_spi.spsr);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_PARA_REG, loongson_spi.para);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_SFCS_REG, loongson_spi.sfcs);
    loongson_spi_write_reg(loongson_spi, LOONGSON_SPI_TIMI_REG, loongson_spi.timi);
    spi_controller_resume(controller);
    return 0;
    }
    DEFINE_SIMPLE_DEV_PM_OPS(loongson_spi_dev_pm_ops, loongson_spi_suspend, loongson_spi_resume);
    EXPORT_SYMBOL_NS_GPL(loongson_spi_dev_pm_ops, "SPI_LOONGSON_CORE");
    MODULE_DESCRIPTION("Loongson SPI core driver");
    MODULE_LICENSE("GPL");
