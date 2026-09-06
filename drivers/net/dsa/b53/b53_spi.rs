//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/b53/b53_spi.c
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


//
// B53 register access through SPI
//
// Copyright (C) 2011-2013 Jonas Gorski <jogo@openwrt.org>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const B53_SPI_DATA: c_uint = 0xf0;
pub const B53_SPI_STATUS: c_uint = 0xfe;

pub const B53_SPI_CMD_READ: c_uint = 0x00;
pub const B53_SPI_CMD_WRITE: c_uint = 0x01;
pub const B53_SPI_CMD_NORMAL: c_uint = 0x60;
pub const B53_SPI_CMD_FAST: c_uint = 0x10;
pub const B53_SPI_PAGE_SELECT: c_uint = 0xff;
    static inline int b53_spi_read_reg(struct spi_device *spi, u8 reg, u8 *val,
    unsigned int len)
    {
    u8 txbuf[2];
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_READ;
    txbuf[1] = reg;
    return spi_write_then_read(spi, txbuf, 2, val, len);
    }
#[no_mangle]
pub unsafe extern "C" fn b53_spi_clear_status(spi: *mut spi_device) -> c_int {
    static inline int b53_spi_clear_status(struct spi_device *spi)
    {
    unsigned int i;
    u8 rxbuf;
    int ret;
    for (i = 0; i < 10; i++) {
    ret = b53_spi_read_reg(spi, B53_SPI_STATUS, &rxbuf, 1);
    if (ret)
    return ret;
    if (!(rxbuf & B53_SPI_CMD_SPIF))
    break;
    mdelay(1);
    }
    if (i == 10)
    return -EIO;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn b53_spi_set_page(spi: *mut spi_device, page: u8) -> c_int {
    static inline int b53_spi_set_page(struct spi_device *spi, u8 page)
    {
    u8 txbuf[3];
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_WRITE;
    txbuf[1] = B53_SPI_PAGE_SELECT;
    txbuf[2] = page;
    return spi_write(spi, txbuf, sizeof(txbuf));
    }
#[no_mangle]
pub unsafe extern "C" fn b53_prepare_reg_access(spi: *mut spi_device, page: u8) -> c_int {
    static inline int b53_prepare_reg_access(struct spi_device *spi, u8 page)
    {
    let mut ret: c_int = b53_spi_clear_status(spi);
    if (ret)
    return ret;
    return b53_spi_set_page(spi, page);
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_prepare_reg_read(spi: *mut spi_device, reg: u8) -> c_int {
    static int b53_spi_prepare_reg_read(struct spi_device *spi, u8 reg)
    {
    u8 rxbuf;
    int retry_count;
    int ret;
    ret = b53_spi_read_reg(spi, reg, &rxbuf, 1);
    if (ret)
    return ret;
    for (retry_count = 0; retry_count < 10; retry_count++) {
    ret = b53_spi_read_reg(spi, B53_SPI_STATUS, &rxbuf, 1);
    if (ret)
    return ret;
    if (rxbuf & B53_SPI_CMD_RACK)
    break;
    mdelay(1);
    }
    if (retry_count == 10)
    return -EIO;
    return 0;
    }
    static int b53_spi_read(struct b53_device *dev, u8 page, u8 reg, u8 *data,
    unsigned int len)
    {
    struct spi_device *spi = dev.priv;
    int ret;
    ret = b53_prepare_reg_access(spi, page);
    if (ret)
    return ret;
    ret = b53_spi_prepare_reg_read(spi, reg);
    if (ret)
    return ret;
    return b53_spi_read_reg(spi, B53_SPI_DATA, data, len);
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_read8(dev: *mut b53_device, page: u8, reg: u8, val: *mut u8) -> c_int {
    static int b53_spi_read8(struct b53_device *dev, u8 page, u8 reg, u8 *val)
    {
    return b53_spi_read(dev, page, reg, val, 1);
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_read16(dev: *mut b53_device, page: u8, reg: u8, val: *mut u16) -> c_int {
    static int b53_spi_read16(struct b53_device *dev, u8 page, u8 reg, u16 *val)
    {
    __le16 value;
    int ret;
    ret = b53_spi_read(dev, page, reg, (u8 *)&value, 2);
    if (!ret)
// val = le16_to_cpu(value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_read32(dev: *mut b53_device, page: u8, reg: u8, val: *mut u32) -> c_int {
    static int b53_spi_read32(struct b53_device *dev, u8 page, u8 reg, u32 *val)
    {
    __le32 value;
    int ret;
    ret = b53_spi_read(dev, page, reg, (u8 *)&value, 4);
    if (!ret)
// val = le32_to_cpu(value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_read48(dev: *mut b53_device, page: u8, reg: u8, val: *mut u64) -> c_int {
    static int b53_spi_read48(struct b53_device *dev, u8 page, u8 reg, u64 *val)
    {
    __le64 value;
    int ret;
// val = 0;
    ret = b53_spi_read(dev, page, reg, (u8 *)&value, 6);
    if (!ret)
// val = le64_to_cpu(value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_read64(dev: *mut b53_device, page: u8, reg: u8, val: *mut u64) -> c_int {
    static int b53_spi_read64(struct b53_device *dev, u8 page, u8 reg, u64 *val)
    {
    __le64 value;
    int ret;
    ret = b53_spi_read(dev, page, reg, (u8 *)&value, 8);
    if (!ret)
// val = le64_to_cpu(value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_write8(dev: *mut b53_device, page: u8, reg: u8, value: u8) -> c_int {
    static int b53_spi_write8(struct b53_device *dev, u8 page, u8 reg, u8 value)
    {
    struct spi_device *spi = dev.priv;
    int ret;
    u8 txbuf[3];
    ret = b53_prepare_reg_access(spi, page);
    if (ret)
    return ret;
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_WRITE;
    txbuf[1] = reg;
    txbuf[2] = value;
    return spi_write(spi, txbuf, sizeof(txbuf));
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_write16(dev: *mut b53_device, page: u8, reg: u8, value: u16) -> c_int {
    static int b53_spi_write16(struct b53_device *dev, u8 page, u8 reg, u16 value)
    {
    struct spi_device *spi = dev.priv;
    int ret;
    u8 txbuf[4];
    ret = b53_prepare_reg_access(spi, page);
    if (ret)
    return ret;
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_WRITE;
    txbuf[1] = reg;
    put_unaligned_le16(value, &txbuf[2]);
    return spi_write(spi, txbuf, sizeof(txbuf));
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_write32(dev: *mut b53_device, page: u8, reg: u8, value: u32) -> c_int {
    static int b53_spi_write32(struct b53_device *dev, u8 page, u8 reg, u32 value)
    {
    struct spi_device *spi = dev.priv;
    int ret;
    u8 txbuf[6];
    ret = b53_prepare_reg_access(spi, page);
    if (ret)
    return ret;
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_WRITE;
    txbuf[1] = reg;
    put_unaligned_le32(value, &txbuf[2]);
    return spi_write(spi, txbuf, sizeof(txbuf));
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_write48(dev: *mut b53_device, page: u8, reg: u8, value: u64) -> c_int {
    static int b53_spi_write48(struct b53_device *dev, u8 page, u8 reg, u64 value)
    {
    struct spi_device *spi = dev.priv;
    int ret;
    u8 txbuf[10];
    ret = b53_prepare_reg_access(spi, page);
    if (ret)
    return ret;
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_WRITE;
    txbuf[1] = reg;
    put_unaligned_le64(value, &txbuf[2]);
    return spi_write(spi, txbuf, sizeof(txbuf) - 2);
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_write64(dev: *mut b53_device, page: u8, reg: u8, value: u64) -> c_int {
    static int b53_spi_write64(struct b53_device *dev, u8 page, u8 reg, u64 value)
    {
    struct spi_device *spi = dev.priv;
    int ret;
    u8 txbuf[10];
    ret = b53_prepare_reg_access(spi, page);
    if (ret)
    return ret;
    txbuf[0] = B53_SPI_CMD_NORMAL | B53_SPI_CMD_WRITE;
    txbuf[1] = reg;
    put_unaligned_le64(value, &txbuf[2]);
    return spi_write(spi, txbuf, sizeof(txbuf));
    }
    static const struct b53_io_ops b53_spi_ops = {
    .read8 = b53_spi_read8,
    .read16 = b53_spi_read16,
    .read32 = b53_spi_read32,
    .read48 = b53_spi_read48,
    .read64 = b53_spi_read64,
    .write8 = b53_spi_write8,
    .write16 = b53_spi_write16,
    .write32 = b53_spi_write32,
    .write48 = b53_spi_write48,
    .write64 = b53_spi_write64,
    };
#[no_mangle]
unsafe extern "C" fn b53_spi_probe(spi: *mut spi_device) -> c_int {
    static int b53_spi_probe(struct spi_device *spi)
    {
    struct b53_device *dev;
    int ret;
    dev = b53_switch_alloc(&spi.dev, &b53_spi_ops, spi);
    if (!dev)
    return -ENOMEM;
    if (spi.dev.platform_data)
    dev.pdata = spi.dev.platform_data;
    ret = b53_switch_register(dev);
    if (ret)
    return ret;
    spi_set_drvdata(spi, dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_remove(spi: *mut spi_device) {
    static void b53_spi_remove(struct spi_device *spi)
    {
    struct b53_device *dev = spi_get_drvdata(spi);
    if (dev)
    b53_switch_remove(dev);
    }
#[no_mangle]
unsafe extern "C" fn b53_spi_shutdown(spi: *mut spi_device) {
    static void b53_spi_shutdown(struct spi_device *spi)
    {
    struct b53_device *dev = spi_get_drvdata(spi);
    if (dev)
    b53_switch_shutdown(dev);
    spi_set_drvdata(spi, core::ptr::null_mut());
    }
    static const struct of_device_id b53_spi_of_match[] = {
    { .compatible = "brcm,bcm5325" },
    { .compatible = "brcm,bcm5365" },
    { .compatible = "brcm,bcm5395" },
    { .compatible = "brcm,bcm5397" },
    { .compatible = "brcm,bcm5398" },
    { .compatible = "brcm,bcm53115" },
    { .compatible = "brcm,bcm53125" },
    { .compatible = "brcm,bcm53128" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, b53_spi_of_match);
    static const struct spi_device_id b53_spi_ids[] = {
    { .name = "bcm5325" },
    { .name = "bcm5365" },
    { .name = "bcm5395" },
    { .name = "bcm5397" },
    { .name = "bcm5398" },
    { .name = "bcm53115" },
    { .name = "bcm53125" },
    { .name = "bcm53128" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, b53_spi_ids);
    static struct spi_driver b53_spi_driver = {
    .driver = {
    .name	= "b53-switch",
    .of_match_table = b53_spi_of_match,
    },
    .probe	= b53_spi_probe,
    .remove	= b53_spi_remove,
    .shutdown = b53_spi_shutdown,
    .id_table = b53_spi_ids,
    };
    module_spi_driver(b53_spi_driver);
    MODULE_AUTHOR("Jonas Gorski <jogo@openwrt.org>");
    MODULE_DESCRIPTION("B53 SPI access driver");
    MODULE_LICENSE("Dual BSD/GPL");
