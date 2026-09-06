//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-butterfly.c
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
// parport-to-butterfly adapter
//
// Copyright (C) 2005 David Brownell
//

//
// This uses SPI to talk with an "AVR Butterfly", which is a $US20 card
// with a battery powered AVR microcontroller and lots of goodies.  You
// can use GCC to develop firmware for this.
//
// See Documentation/spi/butterfly.rst for information about how to build
// and use this custom parallel port cable.
//
// DATA output bits (pins 2..9 == D0..D7)

// STATUS input bits

// CONTROL output bits

    static inline struct butterfly *spidev_to_pp(struct spi_device *spi)
    {
    return spi.controller_data;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct butterfly {
// REVISIT ... for now, this must be first
    pub bitbang: spi_bitbang,
    pub port: *mut parport,
    pub pd: *mut pardevice,
    pub lastbyte: u8,
    pub dataflash: *mut spi_device,
    pub butterfly: *mut spi_device,
    pub info: [spi_board_info; 2],
}

// ----------------------------------------------------------------------
    static inline void
    setsck(struct spi_device *spi, int is_on)
    {
    struct butterfly	*pp = spidev_to_pp(spi);
    u8			bit, byte = pp.lastbyte;
    bit = spi_sck_bit;
    if (is_on)
    byte |= bit;
    else
    byte &= ~bit;
    parport_write_data(pp.port, byte);
    pp.lastbyte = byte;
    }
    static inline void
    setmosi(struct spi_device *spi, int is_on)
    {
    struct butterfly	*pp = spidev_to_pp(spi);
    u8			bit, byte = pp.lastbyte;
    bit = spi_mosi_bit;
    if (is_on)
    byte |= bit;
    else
    byte &= ~bit;
    parport_write_data(pp.port, byte);
    pp.lastbyte = byte;
    }
#[no_mangle]
pub unsafe extern "C" fn getmiso(spi: *mut spi_device) -> c_int {
    static inline int getmiso(struct spi_device *spi)
    {
    struct butterfly	*pp = spidev_to_pp(spi);
    int			value;
    u8			bit;
    bit = spi_miso_bit;
// only STATUS_BUSY is NOT negated
    value = !(parport_read_status(pp.port) & bit);
    return (bit == PARPORT_STATUS_BUSY) ? value : !value;
    }
#[no_mangle]
unsafe extern "C" fn butterfly_chipselect(spi: *mut spi_device, value: c_int) {
    static void butterfly_chipselect(struct spi_device *spi, int value)
    {
    struct butterfly	*pp = spidev_to_pp(spi);
// set default clock polarity
    if (value != BITBANG_CS_INACTIVE)
    setsck(spi, spi.mode & SPI_CPOL);
// here, value == "activate or not";
// most PARPORT_CONTROL_* bits are negated, so we must
// morph it to value == "bit value to write in control register"
//
    if (spi_cs_bit == PARPORT_CONTROL_INIT)
    value = !value;
    parport_frob_control(pp.port, spi_cs_bit, value ? spi_cs_bit : 0);
    }
// we only needed to implement one mode here, and choose SPI_MODE_0

// #define spidelay	ndelay

    static u32
    butterfly_txrx_word_mode0(struct spi_device *spi, unsigned nsecs, u32 word,
    u8 bits, unsigned flags)
    {
    return bitbang_txrx_be_cpha0(spi, nsecs, 0, flags, word, bits);
    }
// ----------------------------------------------------------------------
// override default partitioning with cmdlinepart
    static struct mtd_partition partitions[] = { {
// JFFS2 wants partitions of 4*N blocks for this device,
// so sectors 0 and 1 can't be partitions by themselves.
//
// sector 0 = 8 pages * 264 bytes/page (1 block)
// sector 1 = 248 pages * 264 bytes/page
//
    .name		= "bookkeeping",	/* 66 KB */
    .offset		= 0,
    .size		= (8 + 248) * 264,
// .mask_flags	= MTD_WRITEABLE,
    }, {
// sector 2 = 256 pages * 264 bytes/page
// sectors 3-5 = 512 pages * 264 bytes/page
//
    .name		= "filesystem",		/* 462 KB */
    .offset		= MTDPART_OFS_APPEND,
    .size		= MTDPART_SIZ_FULL,
    } };
    static struct flash_platform_data flash = {
    .name		= "butterflash",
    .parts		= partitions,
    .nr_parts	= ARRAY_SIZE(partitions),
    };
// REVISIT remove this ugly global and its "only one" limitation
    static struct butterfly *butterfly;
#[no_mangle]
unsafe extern "C" fn butterfly_attach(p: *mut parport) {
    static void butterfly_attach(struct parport *p)
    {
    struct pardevice	*pd;
    int			status;
    struct butterfly	*pp;
    struct spi_controller	*host;
    struct device		*dev = p.physport.dev;
    struct pardev_cb	butterfly_cb;
    if (butterfly || !dev)
    return;
// REVISIT:  this just _assumes_ a butterfly is there ... no probe,
// and no way to be selective about what it binds to.
//
    host = spi_alloc_host(dev, sizeof(*pp));
    if (!host) {
    status = -ENOMEM;
    goto done;
    }
    pp = spi_controller_get_devdata(host);
//
// SPI and bitbang hookup
//
// use default setup(), cleanup(), and transfer() methods; and
// only bother implementing mode 0.  Start it later.
//
    host.bus_num = 42;
    host.num_chipselect = 2;
    pp.bitbang.ctlr = host;
    pp.bitbang.chipselect = butterfly_chipselect;
    pp.bitbang.txrx_word[SPI_MODE_0] = butterfly_txrx_word_mode0;
//
// parport hookup
//
    pp.port = p;
    memset(&butterfly_cb, 0, sizeof(butterfly_cb));
    butterfly_cb.private = pp;
    pd = parport_register_dev_model(p, "spi_butterfly", &butterfly_cb, 0);
    if (!pd) {
    status = -ENOMEM;
    goto clean0;
    }
    pp.pd = pd;
    status = parport_claim(pd);
    if (status < 0)
    goto clean1;
//
// Butterfly reset, powerup, run firmware
//
    pr_debug("%s: powerup/reset Butterfly\n", p.name);
// nCS for dataflash (this bit is inverted on output)
    parport_frob_control(pp.port, spi_cs_bit, 0);
// stabilize power with chip in reset (nRESET), and
// spi_sck_bit clear (CPOL=0)
//
    pp.lastbyte |= vcc_bits;
    parport_write_data(pp.port, pp.lastbyte);
    msleep(5);
// take it out of reset; assume long reset delay
    pp.lastbyte |= butterfly_nreset;
    parport_write_data(pp.port, pp.lastbyte);
    msleep(100);
//
// Start SPI ... for now, hide that we're two physical busses.
//
    status = spi_bitbang_start(&pp.bitbang);
    if (status < 0)
    goto clean2;
// Bus 1 lets us talk to at45db041b (firmware disables AVR SPI), AVR
// (firmware resets at45, acts as spi slave) or neither (we ignore
// both, AVR uses AT45).  Here we expect firmware for the first option.
//
    pp.info[0].max_speed_hz = 15 * 1000 * 1000;
    strcpy(pp.info[0].modalias, "mtd_dataflash");
    pp.info[0].platform_data = &flash;
    pp.info[0].chip_select = 1;
    pp.info[0].controller_data = pp;
    pp.dataflash = spi_new_device(pp.bitbang.ctlr, &pp.info[0]);
    if (pp.dataflash)
    pr_debug("%s: dataflash at %s\n", p.name,
    dev_name(&pp.dataflash.dev));
    pr_info("%s: AVR Butterfly\n", p.name);
    butterfly = pp;
    return;
    clean2:
// turn off VCC
    parport_write_data(pp.port, 0);
    parport_release(pp.pd);
    clean1:
    parport_unregister_device(pd);
    clean0:
    spi_controller_put(host);
    done:
    pr_debug("%s: butterfly probe, fail %d\n", p.name, status);
    }
#[no_mangle]
unsafe extern "C" fn butterfly_detach(p: *mut parport) {
    static void butterfly_detach(struct parport *p)
    {
    struct butterfly	*pp;
// FIXME this global is ugly ... but, how to quickly get from
// the parport to the "struct butterfly" associated with it?
// "old school" driver-internal device lists?
//
    if (!butterfly || butterfly.port != p)
    return;
    pp = butterfly;
    butterfly = core::ptr::null_mut();
// stop() unregisters child devices too
    spi_bitbang_stop(&pp.bitbang);
// turn off VCC
    parport_write_data(pp.port, 0);
    msleep(10);
    parport_release(pp.pd);
    parport_unregister_device(pp.pd);
    spi_controller_put(pp.bitbang.ctlr);
    }
    static struct parport_driver butterfly_driver = {
    .name =		"spi_butterfly",
    .match_port =	butterfly_attach,
    .detach =	butterfly_detach,
    };
    module_parport_driver(butterfly_driver);
    MODULE_DESCRIPTION("Parport Adapter driver for AVR Butterfly");
    MODULE_LICENSE("GPL");
