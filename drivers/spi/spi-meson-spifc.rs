//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-meson-spifc.c
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
//
// Driver for Amlogic Meson SPI flash controller (SPIFC)
//
// Copyright (C) 2014 Beniamino Galvani <b.galvani@gmail.com>
//

// register map
pub const REG_CMD: c_uint = 0x00;
pub const REG_ADDR: c_uint = 0x04;
pub const REG_CTRL: c_uint = 0x08;
pub const REG_CTRL1: c_uint = 0x0c;
pub const REG_STATUS: c_uint = 0x10;
pub const REG_CTRL2: c_uint = 0x14;
pub const REG_CLOCK: c_uint = 0x18;
pub const REG_USER: c_uint = 0x1c;
pub const REG_USER1: c_uint = 0x20;
pub const REG_USER2: c_uint = 0x24;
pub const REG_USER3: c_uint = 0x28;
pub const REG_USER4: c_uint = 0x2c;
pub const REG_SLAVE: c_uint = 0x30;
pub const REG_SLAVE1: c_uint = 0x34;
pub const REG_SLAVE2: c_uint = 0x38;
pub const REG_SLAVE3: c_uint = 0x3c;
pub const REG_C0: c_uint = 0x40;
pub const REG_B8: c_uint = 0x60;
pub const REG_MAX: c_uint = 0x7c;
// register fields

pub const CLOCK_DIV_SHIFT: c_int = 12;

pub const CLOCK_CNT_HIGH_SHIFT: c_int = 6;

pub const CLOCK_CNT_LOW_SHIFT: c_int = 0;

pub const USER1_BN_UC_DOUT_SHIFT: c_int = 17;

pub const USER1_BN_UC_DIN_SHIFT: c_int = 8;

pub const SPIFC_BUFFER_SIZE: c_int = 64;
//
// struct meson_spifc
// @host:	the SPI host
// @regmap:	regmap for device registers
// @clk:	input clock of the built-in baud rate generator
// @dev:	the device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_spifc {
    pub host: *mut spi_controller,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub dev: *mut device,
}

    static const struct regmap_config spifc_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = REG_MAX,
    };
//
// meson_spifc_wait_ready() - wait for the current operation to terminate
// @spifc:	the Meson SPI device
// Return:	0 on success, a negative value on error
//
#[no_mangle]
unsafe extern "C" fn meson_spifc_wait_ready(spifc: *mut meson_spifc) -> c_int {
    static int meson_spifc_wait_ready(struct meson_spifc *spifc)
    {
    let mut deadline: c_ulong = jiffies + msecs_to_jiffies(5);
    u32 data;
    do {
    regmap_read(spifc.regmap, REG_SLAVE, &data);
    if (data & SLAVE_TRST_DONE)
    return 0;
    cond_resched();
    } while (!time_after(jiffies, deadline));
    return -ETIMEDOUT;
    }
//
// meson_spifc_drain_buffer() - copy data from device buffer to memory
// @spifc:	the Meson SPI device
// @buf:	the destination buffer
// @len:	number of bytes to copy
//
    static void meson_spifc_drain_buffer(struct meson_spifc *spifc, u8 *buf,
    int len)
    {
    u32 data;
    let mut i: c_int = 0;
    while (i < len) {
    regmap_read(spifc.regmap, REG_C0 + i, &data);
    if (len - i >= 4) {
// ((u32 *)buf) = data;
    buf += 4;
    } else {
    memcpy(buf, &data, len - i);
    break;
    }
    i += 4;
    }
    }
//
// meson_spifc_fill_buffer() - copy data from memory to device buffer
// @spifc:	the Meson SPI device
// @buf:	the source buffer
// @len:	number of bytes to copy
//
    static void meson_spifc_fill_buffer(struct meson_spifc *spifc, const u8 *buf,
    int len)
    {
    u32 data;
    let mut i: c_int = 0;
    while (i < len) {
    if (len - i >= 4)
    data = *(u32 *)buf;
    else
    memcpy(&data, buf, len - i);
    regmap_write(spifc.regmap, REG_C0 + i, data);
    buf += 4;
    i += 4;
    }
    }
//
// meson_spifc_setup_speed() - program the clock divider
// @spifc:	the Meson SPI device
// @speed:	desired speed in Hz
//
#[no_mangle]
unsafe extern "C" fn meson_spifc_setup_speed(spifc: *mut meson_spifc, speed: u32) {
    static void meson_spifc_setup_speed(struct meson_spifc *spifc, u32 speed)
    {
    unsigned long parent, value;
    int n;
    parent = clk_get_rate(spifc.clk);
    n = max_t(int, parent / speed - 1, 1);
    dev_dbg(spifc.dev, "parent %lu, speed %u, n %d\n", parent,
    speed, n);
    value = (n << CLOCK_DIV_SHIFT) & CLOCK_DIV_MASK;
    value |= (n << CLOCK_CNT_LOW_SHIFT) & CLOCK_CNT_LOW_MASK;
    value |= (((n + 1) / 2 - 1) << CLOCK_CNT_HIGH_SHIFT) &
    CLOCK_CNT_HIGH_MASK;
    regmap_write(spifc.regmap, REG_CLOCK, value);
    }
//
// meson_spifc_txrx() - transfer a chunk of data
// @spifc:	the Meson SPI device
// @xfer:	the current SPI transfer
// @offset:	offset of the data to transfer
// @len:	length of the data to transfer
// @last_xfer:	whether this is the last transfer of the message
// @last_chunk:	whether this is the last chunk of the transfer
// Return:	0 on success, a negative value on error
//
    static int meson_spifc_txrx(struct meson_spifc *spifc,
    struct spi_transfer *xfer,
    int offset, int len, bool last_xfer,
    bool last_chunk)
    {
    let mut keep_cs: bool = true;
    int ret;
    if (xfer.tx_buf)
    meson_spifc_fill_buffer(spifc, xfer.tx_buf + offset, len);
// enable DOUT stage
    regmap_update_bits(spifc.regmap, REG_USER, USER_UC_MASK,
    USER_UC_DOUT_SEL);
    regmap_write(spifc.regmap, REG_USER1,
    (8 * len - 1) << USER1_BN_UC_DOUT_SHIFT);
// enable data input during DOUT
    regmap_update_bits(spifc.regmap, REG_USER, USER_DIN_EN_MS,
    USER_DIN_EN_MS);
    if (last_chunk) {
    if (last_xfer)
    keep_cs = xfer.cs_change;
    else
    keep_cs = !xfer.cs_change;
    }
    regmap_update_bits(spifc.regmap, REG_USER4, USER4_CS_ACT,
    keep_cs ? USER4_CS_ACT : 0);
// clear transition done bit
    regmap_update_bits(spifc.regmap, REG_SLAVE, SLAVE_TRST_DONE, 0);
// start transfer
    regmap_update_bits(spifc.regmap, REG_CMD, CMD_USER, CMD_USER);
    ret = meson_spifc_wait_ready(spifc);
    if (!ret && xfer.rx_buf)
    meson_spifc_drain_buffer(spifc, xfer.rx_buf + offset, len);
    return ret;
    }
//
// meson_spifc_transfer_one() - perform a single transfer
// @host:	the SPI host
// @spi:	the SPI device
// @xfer:	the current SPI transfer
// Return:	0 on success, a negative value on error
//
    static int meson_spifc_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct meson_spifc *spifc = spi_controller_get_devdata(host);
    int len, done = 0, ret = 0;
    meson_spifc_setup_speed(spifc, xfer.speed_hz);
    regmap_update_bits(spifc.regmap, REG_CTRL, CTRL_ENABLE_AHB, 0);
    while (done < xfer.len && !ret) {
    len = min_t(int, xfer.len - done, SPIFC_BUFFER_SIZE);
    ret = meson_spifc_txrx(spifc, xfer, done, len,
    spi_transfer_is_last(host, xfer),
    done + len >= xfer.len);
    done += len;
    }
    regmap_update_bits(spifc.regmap, REG_CTRL, CTRL_ENABLE_AHB,
    CTRL_ENABLE_AHB);
    return ret;
    }
//
// meson_spifc_hw_init() - reset and initialize the SPI controller
// @spifc:	the Meson SPI device
//
#[no_mangle]
unsafe extern "C" fn meson_spifc_hw_init(spifc: *mut meson_spifc) {
    static void meson_spifc_hw_init(struct meson_spifc *spifc)
    {
// reset device
    regmap_update_bits(spifc.regmap, REG_SLAVE, SLAVE_SW_RST,
    SLAVE_SW_RST);
// disable compatible mode
    regmap_update_bits(spifc.regmap, REG_USER, USER_CMP_MODE, 0);
// set master mode
    regmap_update_bits(spifc.regmap, REG_SLAVE, SLAVE_OP_MODE, 0);
    }
#[no_mangle]
unsafe extern "C" fn meson_spifc_probe(pdev: *mut platform_device) -> c_int {
    static int meson_spifc_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct meson_spifc *spifc;
    void __iomem *base;
    unsigned int rate;
    int ret;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct meson_spifc));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    spifc = spi_controller_get_devdata(host);
    spifc.dev = &pdev.dev;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    spifc.regmap = devm_regmap_init_mmio(spifc.dev, base,
    &spifc_regmap_config);
    if (IS_ERR(spifc.regmap))
    return PTR_ERR(spifc.regmap);
    spifc.clk = devm_clk_get_enabled(spifc.dev, core::ptr::null_mut());
    if (IS_ERR(spifc.clk)) {
    dev_err(spifc.dev, "missing clock\n");
    return PTR_ERR(spifc.clk);
    }
    rate = clk_get_rate(spifc.clk);
    host.num_chipselect = 1;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.auto_runtime_pm = true;
    host.transfer_one = meson_spifc_transfer_one;
    host.min_speed_hz = rate >> 6;
    host.max_speed_hz = rate >> 1;
    meson_spifc_hw_init(spifc);
    ret =  devm_pm_runtime_set_active_enabled(spifc.dev);
    if (ret)
    return dev_err_probe(spifc.dev, ret, "failed to set runtime PM\n");
    ret = devm_spi_register_controller(spifc.dev, host);
    if (ret)
    return dev_err_probe(spifc.dev, ret, "failed to register spi host\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_spifc_remove(pdev: *mut platform_device) {
    static void meson_spifc_remove(struct platform_device *pdev)
    {
    pm_runtime_get_sync(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn meson_spifc_suspend(dev: *mut device) -> c_int {
    static int meson_spifc_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct meson_spifc *spifc = spi_controller_get_devdata(host);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    if (!pm_runtime_suspended(dev))
    clk_disable_unprepare(spifc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_spifc_resume(dev: *mut device) -> c_int {
    static int meson_spifc_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct meson_spifc *spifc = spi_controller_get_devdata(host);
    int ret;
    if (!pm_runtime_suspended(dev)) {
    ret = clk_prepare_enable(spifc.clk);
    if (ret)
    return ret;
    }
    meson_spifc_hw_init(spifc);
    ret = spi_controller_resume(host);
    if (ret)
    clk_disable_unprepare(spifc.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_spifc_runtime_suspend(dev: *mut device) -> c_int {
    static int meson_spifc_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct meson_spifc *spifc = spi_controller_get_devdata(host);
    clk_disable_unprepare(spifc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_spifc_runtime_resume(dev: *mut device) -> c_int {
    static int meson_spifc_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct meson_spifc *spifc = spi_controller_get_devdata(host);
    return clk_prepare_enable(spifc.clk);
    }
    static const struct dev_pm_ops meson_spifc_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(meson_spifc_suspend, meson_spifc_resume)
    RUNTIME_PM_OPS(meson_spifc_runtime_suspend,
    meson_spifc_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id meson_spifc_dt_match[] = {
    { .compatible = "amlogic,meson6-spifc", },
    { .compatible = "amlogic,meson-gxbb-spifc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, meson_spifc_dt_match);
    static struct platform_driver meson_spifc_driver = {
    .probe	= meson_spifc_probe,
    .remove = meson_spifc_remove,
    .driver	= {
    .name		= "meson-spifc",
    .of_match_table	= of_match_ptr(meson_spifc_dt_match),
    .pm		= pm_ptr(&meson_spifc_pm_ops),
    },
    };
    module_platform_driver(meson_spifc_driver);
    MODULE_AUTHOR("Beniamino Galvani <b.galvani@gmail.com>");
    MODULE_DESCRIPTION("Amlogic Meson SPIFC driver");
    MODULE_LICENSE("GPL v2");
