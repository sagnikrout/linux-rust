//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-clps711x.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// CLPS711X SPI bus driver
//
// Copyright (C) 2012-2016 Alexander Shiyan <shc_work@mail.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_clps711x_data {
    pub syncio: *mut void __iomem,
    pub syscon: *mut regmap,
    pub spi_clk: *mut clk,
    pub tx_buf: *mut u8,
    pub rx_buf: *mut u8,
    pub bpw: c_uint,
    pub len: c_int,
}

    static int spi_clps711x_prepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct spi_clps711x_data *hw = spi_controller_get_devdata(host);
    struct spi_device *spi = msg.spi;
// Setup mode for transfer
    return regmap_update_bits(hw.syscon, SYSCON_OFFSET, SYSCON3_ADCCKNSEN,
    (spi.mode & SPI_CPHA) ?
    SYSCON3_ADCCKNSEN : 0);
    }
    static int spi_clps711x_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct spi_clps711x_data *hw = spi_controller_get_devdata(host);
    u8 data;
    clk_set_rate(hw.spi_clk, xfer.speed_hz ? : spi.max_speed_hz);
    hw.len = xfer.len;
    hw.bpw = xfer.bits_per_word;
    hw.tx_buf = (u8 *)xfer.tx_buf;
    hw.rx_buf = (u8 *)xfer.rx_buf;
// Initiate transfer
    data = hw.tx_buf ? *hw.tx_buf++ : 0;
    writel(data | SYNCIO_FRMLEN(hw.bpw) | SYNCIO_TXFRMEN, hw.syncio);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn spi_clps711x_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t spi_clps711x_isr(int irq, void *dev_id)
    {
    struct spi_controller *host = dev_id;
    struct spi_clps711x_data *hw = spi_controller_get_devdata(host);
    u8 data;
// Handle RX
    data = readb(hw.syncio);
    if (hw.rx_buf)
// hw->rx_buf++ = data;
// Handle TX
    if (--hw.len > 0) {
    data = hw.tx_buf ? *hw.tx_buf++ : 0;
    writel(data | SYNCIO_FRMLEN(hw.bpw) | SYNCIO_TXFRMEN,
    hw.syncio);
    } else
    spi_finalize_current_transfer(host);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spi_clps711x_probe(pdev: *mut platform_device) -> c_int {
    static int spi_clps711x_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct spi_clps711x_data *hw;
    struct spi_controller *host;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*hw));
    if (!host)
    return -ENOMEM;
    host.use_gpio_descriptors = true;
    host.bus_num = -1;
    host.mode_bits = SPI_CPHA | SPI_CS_HIGH;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(1, 8);
    host.prepare_message = spi_clps711x_prepare_message;
    host.transfer_one = spi_clps711x_transfer_one;
    hw = spi_controller_get_devdata(host);
    hw.spi_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(hw.spi_clk))
    return PTR_ERR(hw.spi_clk);
    hw.syscon = syscon_regmap_lookup_by_phandle(np, "syscon");
    if (IS_ERR(hw.syscon))
    return PTR_ERR(hw.syscon);
    hw.syncio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hw.syncio))
    return PTR_ERR(hw.syncio);
// Disable extended mode due hardware problems
    regmap_update_bits(hw.syscon, SYSCON_OFFSET, SYSCON3_ADCCON, 0);
// Clear possible pending interrupt
    readl(hw.syncio);
    ret = devm_request_irq(&pdev.dev, irq, spi_clps711x_isr, 0,
    dev_name(&pdev.dev), host);
    if (ret)
    return ret;
    return devm_spi_register_controller(&pdev.dev, host);
    }
    static const struct of_device_id clps711x_spi_dt_ids[] = {
    { .compatible = "cirrus,ep7209-spi", },
    { }
    };
    MODULE_DEVICE_TABLE(of, clps711x_spi_dt_ids);
    static struct platform_driver clps711x_spi_driver = {
    .driver	= {
    .name	= DRIVER_NAME,
    .of_match_table = clps711x_spi_dt_ids,
    },
    .probe	= spi_clps711x_probe,
    };
    module_platform_driver(clps711x_spi_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
    MODULE_DESCRIPTION("CLPS711X SPI bus driver");
    MODULE_ALIAS("platform:" DRIVER_NAME);
