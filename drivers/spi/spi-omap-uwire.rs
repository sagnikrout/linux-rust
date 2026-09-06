//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-omap-uwire.c
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
// MicroWire interface driver for OMAP
//
// Copyright 2003 MontaVista Software Inc. <source@mvista.com>
//
// Ported to 2.6 OMAP uwire interface.
// Copyright (C) 2004 Texas Instruments.
//
// Generalization patches by Juha Yrjola <juha.yrjola@nokia.com>
//
// Copyright (C) 2005 David Brownell (ported to 2.6 SPI interface)
// Copyright (C) 2006 Nokia
//
// Many updates by Imre Deak <imre.deak@nokia.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// THIS SOFTWARE IS PROVIDED "AS IS" AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
// USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// FIXME address is now a platform device resource,
// and irqs should show there too...
//
pub const UWIRE_BASE_PHYS: c_uint = 0xFFFB3000;
// uWire Registers:
pub const UWIRE_IO_SIZE: c_uint = 0x20;
pub const UWIRE_TDR: c_uint = 0x00;
pub const UWIRE_RDR: c_uint = 0x00;
pub const UWIRE_CSR: c_uint = 0x01;
pub const UWIRE_SR1: c_uint = 0x02;
pub const UWIRE_SR2: c_uint = 0x03;
pub const UWIRE_SR3: c_uint = 0x04;
pub const UWIRE_SR4: c_uint = 0x05;
pub const UWIRE_SR5: c_uint = 0x06;
// CSR bits

// SR1 or SR2 bits
pub const UWIRE_READ_FALLING_EDGE: c_uint = 0x0001;
pub const UWIRE_READ_RISING_EDGE: c_uint = 0x0000;
pub const UWIRE_WRITE_FALLING_EDGE: c_uint = 0x0000;
pub const UWIRE_WRITE_RISING_EDGE: c_uint = 0x0002;
pub const UWIRE_CS_ACTIVE_LOW: c_uint = 0x0000;
pub const UWIRE_CS_ACTIVE_HIGH: c_uint = 0x0004;
pub const UWIRE_FREQ_DIV_2: c_uint = 0x0000;
pub const UWIRE_FREQ_DIV_4: c_uint = 0x0008;
pub const UWIRE_FREQ_DIV_8: c_uint = 0x0010;
pub const UWIRE_CHK_READY: c_uint = 0x0020;
pub const UWIRE_CLK_INVERTED: c_uint = 0x0040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uwire_spi {
    pub bitbang: spi_bitbang,
    pub ck: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uwire_state {
    pub div1_idx: unsigned,
}

// REVISIT compile time constant for idx_shift?
//
// Or, put it in a structure which is used throughout the driver;
// that avoids having to issue two loads for each bit of static data.
//
    let mut uwire_idx_shift: static unsigned int = 2;
    static void __iomem *uwire_base;
#[no_mangle]
pub unsafe extern "C" fn uwire_write_reg(idx: c_int, val: u16) {
    static inline void uwire_write_reg(int idx, u16 val)
    {
    __raw_writew(val, uwire_base + (idx << uwire_idx_shift));
    }
#[no_mangle]
pub unsafe extern "C" fn uwire_read_reg(idx: c_int) -> u16 {
    static inline u16 uwire_read_reg(int idx)
    {
    return __raw_readw(uwire_base + (idx << uwire_idx_shift));
    }
#[no_mangle]
pub unsafe extern "C" fn omap_uwire_configure_mode(cs: u8, flags: c_ulong) {
    static inline void omap_uwire_configure_mode(u8 cs, unsigned long flags)
    {
    u16	w, val = 0;
    int	shift, reg;
    if (flags & UWIRE_CLK_INVERTED)
    val ^= 0x03;
    val = flags & 0x3f;
    if (cs & 1)
    shift = 6;
    else
    shift = 0;
    if (cs <= 1)
    reg = UWIRE_SR1;
    else
    reg = UWIRE_SR2;
    w = uwire_read_reg(reg);
    w &= ~(0x3f << shift);
    w |= val << shift;
    uwire_write_reg(reg, w);
    }
#[no_mangle]
unsafe extern "C" fn wait_uwire_csr_flag(mask: u16, val: u16, might_not_catch: c_int) -> c_int {
    static int wait_uwire_csr_flag(u16 mask, u16 val, int might_not_catch)
    {
    u16 w;
    let mut c: c_int = 0;
    let mut max_jiffies: c_ulong = jiffies + HZ;
    for (;;) {
    w = uwire_read_reg(UWIRE_CSR);
    if ((w & mask) == val)
    break;
    if (time_after(jiffies, max_jiffies)) {
    printk(KERN_ERR "%s: timeout. reg=%#06x "
    "mask=%#06x val=%#06x\n",
    __func__, w, mask, val);
    return -1;
    }
    c++;
    if (might_not_catch && c > 64)
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uwire_set_clk1_div(div1_idx: c_int) {
    static void uwire_set_clk1_div(int div1_idx)
    {
    u16 w;
    w = uwire_read_reg(UWIRE_SR3);
    w &= ~(0x03 << 1);
    w |= div1_idx << 1;
    uwire_write_reg(UWIRE_SR3, w);
    }
#[no_mangle]
unsafe extern "C" fn uwire_chipselect(spi: *mut spi_device, value: c_int) {
    static void uwire_chipselect(struct spi_device *spi, int value)
    {
    struct	uwire_state *ust = spi.controller_state;
    u16	w;
    int	old_cs;
    BUG_ON(wait_uwire_csr_flag(CSRB, 0, 0));
    w = uwire_read_reg(UWIRE_CSR);
    old_cs = (w >> 10) & 0x03;
    if (value == BITBANG_CS_INACTIVE || old_cs != spi_get_chipselect(spi, 0)) {
// Deselect this CS, or the previous CS
    w &= ~CS_CMD;
    uwire_write_reg(UWIRE_CSR, w);
    }
// activate specfied chipselect
    if (value == BITBANG_CS_ACTIVE) {
    uwire_set_clk1_div(ust.div1_idx);
// invert clock?
    if (spi.mode & SPI_CPOL)
    uwire_write_reg(UWIRE_SR4, 1);
    else
    uwire_write_reg(UWIRE_SR4, 0);
    w = spi_get_chipselect(spi, 0) << 10;
    w |= CS_CMD;
    uwire_write_reg(UWIRE_CSR, w);
    }
    }
#[no_mangle]
unsafe extern "C" fn uwire_txrx(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int uwire_txrx(struct spi_device *spi, struct spi_transfer *t)
    {
    let mut len: unsigned = t.len;
    let mut bits: unsigned = t.bits_per_word;
    unsigned	bytes;
    u16		val, w;
    let mut status: c_int = 0;
    if (!t.tx_buf && !t.rx_buf)
    return 0;
    w = spi_get_chipselect(spi, 0) << 10;
    w |= CS_CMD;
    if (t.tx_buf) {
    const u8	*buf = t.tx_buf;
// NOTE:  DMA could be used for TX transfers
// write one or two bytes at a time
    while (len >= 1) {
// tx bit 15 is first sent; we byteswap multibyte words
// (msb-first) on the way out from memory.
//
    val = *buf++;
    if (bits > 8) {
    bytes = 2;
    val |= *buf++ << 8;
    } else
    bytes = 1;
    val <<= 16 - bits;

    pr_debug("%s: write-%d =%04x\n",
    dev_name(&spi.dev), bits, val);

    if (wait_uwire_csr_flag(CSRB, 0, 0))
    goto eio;
    uwire_write_reg(UWIRE_TDR, val);
// start write
    val = START | w | (bits << 5);
    uwire_write_reg(UWIRE_CSR, val);
    len -= bytes;
// Wait till write actually starts.
// This is needed with MPU clock 60+ MHz.
// REVISIT: we may not have time to catch it...
//
    if (wait_uwire_csr_flag(CSRB, CSRB, 1))
    goto eio;
    status += bytes;
    }
// REVISIT:  save this for later to get more i/o overlap
    if (wait_uwire_csr_flag(CSRB, 0, 0))
    goto eio;
    } else if (t.rx_buf) {
    u8		*buf = t.rx_buf;
// read one or two bytes at a time
    while (len) {
    if (bits > 8) {
    bytes = 2;
    } else
    bytes = 1;
// start read
    val = START | w | (bits << 0);
    uwire_write_reg(UWIRE_CSR, val);
    len -= bytes;
// Wait till read actually starts
    (void) wait_uwire_csr_flag(CSRB, CSRB, 1);
    if (wait_uwire_csr_flag(RDRB | CSRB,
    RDRB, 0))
    goto eio;
// rx bit 0 is last received; multibyte words will
// be properly byteswapped on the way to memory.
//
    val = uwire_read_reg(UWIRE_RDR);
    val &= (1 << bits) - 1;
// buf++ = (u8) val;
    if (bytes == 2)
// buf++ = val >> 8;
    status += bytes;

    pr_debug("%s: read-%d =%04x\n",
    dev_name(&spi.dev), bits, val);

    }
    }
    return status;
    eio:
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn uwire_setup_transfer(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int uwire_setup_transfer(struct spi_device *spi, struct spi_transfer *t)
    {
    struct uwire_state	*ust = spi.controller_state;
    struct uwire_spi	*uwire;
    let mut flags: unsigned = 0;
    unsigned		hz;
    unsigned long		rate;
    int			div1_idx;
    int			div1;
    int			div2;
    int			status;
    uwire = spi_controller_get_devdata(spi.controller);
// mode 0..3, clock inverted separately;
// standard nCS signaling;
// don't treat DI=high as "not ready"
//
    if (spi.mode & SPI_CS_HIGH)
    flags |= UWIRE_CS_ACTIVE_HIGH;
    if (spi.mode & SPI_CPOL)
    flags |= UWIRE_CLK_INVERTED;
    switch (spi.mode & SPI_MODE_X_MASK) {
    case SPI_MODE_0:
    case SPI_MODE_3:
    flags |= UWIRE_WRITE_FALLING_EDGE | UWIRE_READ_RISING_EDGE;
    break;
    case SPI_MODE_1:
    case SPI_MODE_2:
    flags |= UWIRE_WRITE_RISING_EDGE | UWIRE_READ_FALLING_EDGE;
    break;
    }
// assume it's already enabled
    rate = clk_get_rate(uwire.ck);
    if (t != core::ptr::null_mut())
    hz = t.speed_hz;
    else
    hz = spi.max_speed_hz;
    if (!hz) {
    pr_debug("%s: zero speed?\n", dev_name(&spi.dev));
    status = -EINVAL;
    goto done;
    }
// F_INT = mpu_xor_clk / DIV1
    for (div1_idx = 0; div1_idx < 4; div1_idx++) {
    switch (div1_idx) {
    case 0:
    div1 = 2;
    break;
    case 1:
    div1 = 4;
    break;
    case 2:
    div1 = 7;
    break;
    default:
    case 3:
    div1 = 10;
    break;
    }
    div2 = (rate / div1 + hz - 1) / hz;
    if (div2 <= 8)
    break;
    }
    if (div1_idx == 4) {
    pr_debug("%s: lowest clock %ld, need %d\n",
    dev_name(&spi.dev), rate / 10 / 8, hz);
    status = -EDOM;
    goto done;
    }
// we have to cache this and reset in uwire_chipselect as this is a
// global parameter and another uwire device can change it under
// us
    ust.div1_idx = div1_idx;
    uwire_set_clk1_div(div1_idx);
    rate /= div1;
    switch (div2) {
    case 0:
    case 1:
    case 2:
    flags |= UWIRE_FREQ_DIV_2;
    rate /= 2;
    break;
    case 3:
    case 4:
    flags |= UWIRE_FREQ_DIV_4;
    rate /= 4;
    break;
    case 5:
    case 6:
    case 7:
    case 8:
    flags |= UWIRE_FREQ_DIV_8;
    rate /= 8;
    break;
    }
    omap_uwire_configure_mode(spi_get_chipselect(spi, 0), flags);
    pr_debug("%s: uwire flags %02x, armxor %lu KHz, SCK %lu KHz\n",
    __func__, flags,
    clk_get_rate(uwire.ck) / 1000,
    rate / 1000);
    status = 0;
    done:
    return status;
    }
#[no_mangle]
unsafe extern "C" fn uwire_setup(spi: *mut spi_device) -> c_int {
    static int uwire_setup(struct spi_device *spi)
    {
    struct uwire_state *ust = spi.controller_state;
    let mut initial_setup: bool = false;
    int status;
    if (ust == core::ptr::null_mut()) {
    ust = kzalloc_obj(*ust);
    if (ust == core::ptr::null_mut())
    return -ENOMEM;
    spi.controller_state = ust;
    initial_setup = true;
    }
    status = uwire_setup_transfer(spi, core::ptr::null_mut());
    if (status && initial_setup)
    kfree(ust);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn uwire_cleanup(spi: *mut spi_device) {
    static void uwire_cleanup(struct spi_device *spi)
    {
    kfree(spi.controller_state);
    }
#[no_mangle]
unsafe extern "C" fn uwire_off(uwire: *mut uwire_spi) {
    static void uwire_off(struct uwire_spi *uwire)
    {
    uwire_write_reg(UWIRE_SR3, 0);
    clk_disable_unprepare(uwire.ck);
    spi_controller_put(uwire.bitbang.ctlr);
    }
#[no_mangle]
unsafe extern "C" fn uwire_probe(pdev: *mut platform_device) -> c_int {
    static int uwire_probe(struct platform_device *pdev)
    {
    struct spi_controller	*host;
    struct uwire_spi	*uwire;
    int			status;
    host = spi_alloc_host(&pdev.dev, sizeof(*uwire));
    if (!host)
    return -ENODEV;
    uwire = spi_controller_get_devdata(host);
    uwire_base = devm_ioremap(&pdev.dev, UWIRE_BASE_PHYS, UWIRE_IO_SIZE);
    if (!uwire_base) {
    dev_dbg(&pdev.dev, "can't ioremap UWIRE\n");
    spi_controller_put(host);
    return -ENOMEM;
    }
    platform_set_drvdata(pdev, uwire);
    uwire.ck = devm_clk_get(&pdev.dev, "fck");
    if (IS_ERR(uwire.ck)) {
    status = PTR_ERR(uwire.ck);
    dev_dbg(&pdev.dev, "no functional clock?\n");
    spi_controller_put(host);
    return status;
    }
    clk_prepare_enable(uwire.ck);
    uwire_write_reg(UWIRE_SR3, 1);
// the spi->mode bits understood by this driver:
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(1, 16);
    host.flags = SPI_CONTROLLER_HALF_DUPLEX;
    host.bus_num = 2;	/* "official" */
    host.num_chipselect = 4;
    host.setup = uwire_setup;
    host.cleanup = uwire_cleanup;
    uwire.bitbang.ctlr = host;
    uwire.bitbang.chipselect = uwire_chipselect;
    uwire.bitbang.setup_transfer = uwire_setup_transfer;
    uwire.bitbang.txrx_bufs = uwire_txrx;
    status = spi_bitbang_start(&uwire.bitbang);
    if (status < 0) {
    uwire_off(uwire);
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn uwire_remove(pdev: *mut platform_device) {
    static void uwire_remove(struct platform_device *pdev)
    {
    struct uwire_spi	*uwire = platform_get_drvdata(pdev);
// FIXME remove all child devices, somewhere ...
    spi_bitbang_stop(&uwire.bitbang);
    uwire_off(uwire);
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:omap_uwire");
    static struct platform_driver uwire_driver = {
    .driver = {
    .name		= "omap_uwire",
    },
    .probe = uwire_probe,
    .remove = uwire_remove,
// suspend ... unuse ck
// resume ... use ck
    };
#[no_mangle]
unsafe extern "C" fn omap_uwire_init() -> int __init {
    static int __init omap_uwire_init(void)
    {
    return platform_driver_register(&uwire_driver);
    }
#[no_mangle]
unsafe extern "C" fn omap_uwire_exit() -> void __exit {
    static void __exit omap_uwire_exit(void)
    {
    platform_driver_unregister(&uwire_driver);
    }
    subsys_initcall(omap_uwire_init);
    module_exit(omap_uwire_exit);
    MODULE_DESCRIPTION("MicroWire interface driver for OMAP");
    MODULE_LICENSE("GPL");
