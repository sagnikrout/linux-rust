//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-sh-sci.c
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
// SH SCI SPI interface
//
// Copyright (c) 2008 Magnus Damm
//
// Based on S3C24XX GPIO based SPI driver, which is:
// Copyright (c) 2006 Ben Dooks
// Copyright (c) 2006 Simtec Electronics
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_sci_spi {
    pub bitbang: spi_bitbang,
    pub membase: *mut void __iomem,
    pub val: c_uchar,
    pub info: *mut sh_spi_info,
    pub dev: *mut platform_device,
}

#[no_mangle]
pub unsafe extern "C" fn setbits(sp: *mut sh_sci_spi, bits: c_int, on: c_int) {
    static inline void setbits(struct sh_sci_spi *sp, int bits, int on)
    {
//
// We are the only user of SCSPTR so no locking is required.
// Reading bit 2 and 0 in SCSPTR gives pin state as input.
// Writing the same bits sets the output value.
// This makes regular read-modify-write difficult so we
// use sp->val to keep track of the latest register value.
//
    if (on)
    sp.val |= bits;
    else
    sp.val &= ~bits;
    iowrite8(sp.val, SCSPTR(sp));
    }
#[no_mangle]
pub unsafe extern "C" fn setsck(dev: *mut spi_device, on: c_int) {
    static inline void setsck(struct spi_device *dev, int on)
    {
    setbits(spi_controller_get_devdata(dev.controller), PIN_SCK, on);
    }
#[no_mangle]
pub unsafe extern "C" fn setmosi(dev: *mut spi_device, on: c_int) {
    static inline void setmosi(struct spi_device *dev, int on)
    {
    setbits(spi_controller_get_devdata(dev.controller), PIN_TXD, on);
    }
#[no_mangle]
pub unsafe extern "C" fn getmiso(dev: *mut spi_device) -> u32 {
    static inline u32 getmiso(struct spi_device *dev)
    {
    struct sh_sci_spi *sp = spi_controller_get_devdata(dev.controller);
    return (ioread8(SCSPTR(sp)) & PIN_RXD) ? 1 : 0;
    }

    static u32 sh_sci_spi_txrx_mode0(struct spi_device *spi,
    unsigned nsecs, u32 word, u8 bits,
    unsigned flags)
    {
    return bitbang_txrx_be_cpha0(spi, nsecs, 0, flags, word, bits);
    }
    static u32 sh_sci_spi_txrx_mode1(struct spi_device *spi,
    unsigned nsecs, u32 word, u8 bits,
    unsigned flags)
    {
    return bitbang_txrx_be_cpha1(spi, nsecs, 0, flags, word, bits);
    }
    static u32 sh_sci_spi_txrx_mode2(struct spi_device *spi,
    unsigned nsecs, u32 word, u8 bits,
    unsigned flags)
    {
    return bitbang_txrx_be_cpha0(spi, nsecs, 1, flags, word, bits);
    }
    static u32 sh_sci_spi_txrx_mode3(struct spi_device *spi,
    unsigned nsecs, u32 word, u8 bits,
    unsigned flags)
    {
    return bitbang_txrx_be_cpha1(spi, nsecs, 1, flags, word, bits);
    }
#[no_mangle]
unsafe extern "C" fn sh_sci_spi_chipselect(dev: *mut spi_device, value: c_int) {
    static void sh_sci_spi_chipselect(struct spi_device *dev, int value)
    {
    struct sh_sci_spi *sp = spi_controller_get_devdata(dev.controller);
    if (sp.info.chip_select)
    (sp.info.chip_select)(sp.info, spi_get_chipselect(dev, 0), value);
    }
#[no_mangle]
unsafe extern "C" fn sh_sci_spi_probe(dev: *mut platform_device) -> c_int {
    static int sh_sci_spi_probe(struct platform_device *dev)
    {
    struct resource	*r;
    struct spi_controller *host;
    struct sh_sci_spi *sp;
    int ret;
    host = spi_alloc_host(&dev.dev, sizeof(struct sh_sci_spi));
    if (host == core::ptr::null_mut()) {
    dev_err(&dev.dev, "failed to allocate spi host\n");
    ret = -ENOMEM;
    goto err0;
    }
    sp = spi_controller_get_devdata(host);
    platform_set_drvdata(dev, sp);
    sp.info = dev_get_platdata(&dev.dev);
    if (!sp.info) {
    dev_err(&dev.dev, "platform data is missing\n");
    ret = -ENOENT;
    goto err1;
    }
// setup spi bitbang adaptor
    sp.bitbang.ctlr = host;
    sp.bitbang.ctlr.bus_num = sp.info.bus_num;
    sp.bitbang.ctlr.num_chipselect = sp.info.num_chipselect;
    sp.bitbang.chipselect = sh_sci_spi_chipselect;
    sp.bitbang.txrx_word[SPI_MODE_0] = sh_sci_spi_txrx_mode0;
    sp.bitbang.txrx_word[SPI_MODE_1] = sh_sci_spi_txrx_mode1;
    sp.bitbang.txrx_word[SPI_MODE_2] = sh_sci_spi_txrx_mode2;
    sp.bitbang.txrx_word[SPI_MODE_3] = sh_sci_spi_txrx_mode3;
    r = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (r == core::ptr::null_mut()) {
    ret = -ENOENT;
    goto err1;
    }
    sp.membase = ioremap(r.start, resource_size(r));
    if (!sp.membase) {
    ret = -ENXIO;
    goto err1;
    }
    sp.val = ioread8(SCSPTR(sp));
    setbits(sp, PIN_INIT, 1);
    ret = spi_bitbang_start(&sp.bitbang);
    if (!ret)
    return 0;
    setbits(sp, PIN_INIT, 0);
    iounmap(sp.membase);
    err1:
    spi_controller_put(sp.bitbang.ctlr);
    err0:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_sci_spi_remove(dev: *mut platform_device) {
    static void sh_sci_spi_remove(struct platform_device *dev)
    {
    struct sh_sci_spi *sp = platform_get_drvdata(dev);
    spi_bitbang_stop(&sp.bitbang);
    setbits(sp, PIN_INIT, 0);
    iounmap(sp.membase);
    spi_controller_put(sp.bitbang.ctlr);
    }
    static struct platform_driver sh_sci_spi_drv = {
    .probe		= sh_sci_spi_probe,
    .remove		= sh_sci_spi_remove,
    .driver		= {
    .name	= "spi_sh_sci",
    },
    };
    module_platform_driver(sh_sci_spi_drv);
    MODULE_DESCRIPTION("SH SCI SPI Driver");
    MODULE_AUTHOR("Magnus Damm <damm@opensource.se>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:spi_sh_sci");
