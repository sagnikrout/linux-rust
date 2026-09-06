//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-au1550.c
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
// i2c-au1550.c: SMBus (i2c) adapter for Alchemy PSC interface
// Copyright (C) 2004 Embedded Edge, LLC <dan@embeddededge.com>
//
// 2.6 port by Matt Porter <mporter@kernel.crashing.org>
//
// The documentation describes this as an SMBus controller, but it doesn't
// understand any of the SMBus protocol in hardware.  It's really an I2C
// controller that could emulate most of the SMBus in software.
//
// This is just a skeleton adapter to use with the Au1550 PSC
// algorithm.  It was developed for the Pb1550, but will work with
// any Au1550 board that has a similar PSC configuration.
//

pub const PSC_SEL: c_uint = 0x00;
pub const PSC_CTRL: c_uint = 0x04;
pub const PSC_SMBCFG: c_uint = 0x08;
pub const PSC_SMBMSK: c_uint = 0x0C;
pub const PSC_SMBPCR: c_uint = 0x10;
pub const PSC_SMBSTAT: c_uint = 0x14;
pub const PSC_SMBEVNT: c_uint = 0x18;
pub const PSC_SMBTXRX: c_uint = 0x1C;
pub const PSC_SMBTMR: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_au1550_data {
    pub psc_base: *mut void __iomem,
    pub xfer_timeout: c_int,
    pub adap: i2c_adapter,
}

#[no_mangle]
pub unsafe extern "C" fn WR(a: *mut i2c_au1550_data, r: c_int, v: c_ulong) {
    static inline void WR(struct i2c_au1550_data *a, int r, unsigned long v)
    {
    __raw_writel(v, a.psc_base + r);
    wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn RD(a: *mut i2c_au1550_data, r: c_int) -> c_ulong {
    static inline unsigned long RD(struct i2c_au1550_data *a, int r)
    {
    return __raw_readl(a.psc_base + r);
    }
#[no_mangle]
unsafe extern "C" fn wait_xfer_done(adap: *mut i2c_au1550_data) -> c_int {
    static int wait_xfer_done(struct i2c_au1550_data *adap)
    {
    int i;
// Wait for Tx Buffer Empty
    for (i = 0; i < adap.xfer_timeout; i++) {
    if (RD(adap, PSC_SMBSTAT) & PSC_SMBSTAT_TE)
    return 0;
    udelay(1);
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn wait_ack(adap: *mut i2c_au1550_data) -> c_int {
    static int wait_ack(struct i2c_au1550_data *adap)
    {
    unsigned long stat;
    if (wait_xfer_done(adap))
    return -ETIMEDOUT;
    stat = RD(adap, PSC_SMBEVNT);
    if ((stat & (PSC_SMBEVNT_DN | PSC_SMBEVNT_AN | PSC_SMBEVNT_AL)) != 0)
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wait_controller_done(adap: *mut i2c_au1550_data) -> c_int {
    static int wait_controller_done(struct i2c_au1550_data *adap)
    {
    int i;
    for (i = 0; i < 2 * adap.xfer_timeout; i++) {
    if ((RD(adap, PSC_SMBEVNT) & PSC_SMBEVNT_MD) != 0)
    return 0;
    udelay(1);
    }
    return -ETIMEDOUT;
    }
    static int
    do_address(struct i2c_au1550_data *adap, unsigned int addr, int rd, int q)
    {
    unsigned long stat;
// Reset the FIFOs, clear events.
    stat = RD(adap, PSC_SMBSTAT);
    WR(adap, PSC_SMBEVNT, PSC_SMBEVNT_ALLCLR);
    if (!(stat & PSC_SMBSTAT_TE) || !(stat & PSC_SMBSTAT_RE)) {
    WR(adap, PSC_SMBPCR, PSC_SMBPCR_DC);
    while ((RD(adap, PSC_SMBPCR) & PSC_SMBPCR_DC) != 0)
    cpu_relax();
    udelay(50);
    }
// Write out the i2c chip address and specify operation
    addr <<= 1;
    if (rd)
    addr |= 1;
// zero-byte xfers stop immediately
    if (q)
    addr |= PSC_SMBTXRX_STP;
// Put byte into fifo, start up controller
    WR(adap, PSC_SMBTXRX, addr);
    WR(adap, PSC_SMBPCR, PSC_SMBPCR_MS);
    if (wait_ack(adap))
    return -EIO;
    return (q) ? wait_controller_done(adap) : 0;
    }
#[no_mangle]
unsafe extern "C" fn wait_for_rx_byte(adap: *mut i2c_au1550_data, out: *mut c_uchar) -> c_int {
    static int wait_for_rx_byte(struct i2c_au1550_data *adap, unsigned char *out)
    {
    int j;
    if (wait_xfer_done(adap))
    return -EIO;
    j =  adap.xfer_timeout * 100;
    do {
    j--;
    if (j <= 0)
    return -EIO;
    if ((RD(adap, PSC_SMBSTAT) & PSC_SMBSTAT_RE) == 0)
    j = 0;
    else
    udelay(1);
    } while (j > 0);
// out = RD(adap, PSC_SMBTXRX);
    return 0;
    }
    static int i2c_read(struct i2c_au1550_data *adap, unsigned char *buf,
    unsigned int len)
    {
    int i;
    if (len == 0)
    return 0;
// A read is performed by stuffing the transmit fifo with
// zero bytes for timing, waiting for bytes to appear in the
// receive fifo, then reading the bytes.
//
    i = 0;
    while (i < (len - 1)) {
    WR(adap, PSC_SMBTXRX, 0);
    if (wait_for_rx_byte(adap, &buf[i]))
    return -EIO;
    i++;
    }
// The last byte has to indicate transfer done.
    WR(adap, PSC_SMBTXRX, PSC_SMBTXRX_STP);
    if (wait_controller_done(adap))
    return -EIO;
    buf[i] = (unsigned char)(RD(adap, PSC_SMBTXRX) & 0xff);
    return 0;
    }
    static int i2c_write(struct i2c_au1550_data *adap, unsigned char *buf,
    unsigned int len)
    {
    int i;
    unsigned long data;
    if (len == 0)
    return 0;
    i = 0;
    while (i < (len-1)) {
    data = buf[i];
    WR(adap, PSC_SMBTXRX, data);
    if (wait_ack(adap))
    return -EIO;
    i++;
    }
// The last byte has to indicate transfer done.
    data = buf[i];
    data |= PSC_SMBTXRX_STP;
    WR(adap, PSC_SMBTXRX, data);
    if (wait_controller_done(adap))
    return -EIO;
    return 0;
    }
    static int
    au1550_xfer(struct i2c_adapter *i2c_adap, struct i2c_msg *msgs, int num)
    {
    struct i2c_au1550_data *adap = i2c_adap.algo_data;
    struct i2c_msg *p;
    int i, err = 0;
    WR(adap, PSC_CTRL, PSC_CTRL_ENABLE);
    for (i = 0; !err && i < num; i++) {
    p = &msgs[i];
    err = do_address(adap, p.addr, p.flags & I2C_M_RD,
    (p.len == 0));
    if (err || !p.len)
    continue;
    if (p.flags & I2C_M_RD)
    err = i2c_read(adap, p.buf, p.len);
    else
    err = i2c_write(adap, p.buf, p.len);
    }
// Return the number of messages processed, or the error code.
//
    if (err == 0)
    err = num;
    WR(adap, PSC_CTRL, PSC_CTRL_SUSPEND);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn au1550_func(adap: *mut i2c_adapter) -> u32 {
    static u32 au1550_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm au1550_algo = {
    .xfer = au1550_xfer,
    .functionality = au1550_func,
    };
#[no_mangle]
unsafe extern "C" fn i2c_au1550_setup(priv: *mut i2c_au1550_data) {
    static void i2c_au1550_setup(struct i2c_au1550_data *priv)
    {
    unsigned long cfg;
    WR(priv, PSC_CTRL, PSC_CTRL_DISABLE);
    WR(priv, PSC_SEL, PSC_SEL_PS_SMBUSMODE);
    WR(priv, PSC_SMBCFG, 0);
    WR(priv, PSC_CTRL, PSC_CTRL_ENABLE);
    while ((RD(priv, PSC_SMBSTAT) & PSC_SMBSTAT_SR) == 0)
    cpu_relax();
    cfg = PSC_SMBCFG_RT_FIFO8 | PSC_SMBCFG_TT_FIFO8 | PSC_SMBCFG_DD_DISABLE;
    WR(priv, PSC_SMBCFG, cfg);
// Divide by 8 to get a 6.25 MHz clock.  The later protocol
// timings are based on this clock.
//
    cfg |= PSC_SMBCFG_SET_DIV(PSC_SMBCFG_DIV8);
    WR(priv, PSC_SMBCFG, cfg);
    WR(priv, PSC_SMBMSK, PSC_SMBMSK_ALLMASK);
// Set the protocol timer values.  See Table 71 in the
// Au1550 Data Book for standard timing values.
//
    WR(priv, PSC_SMBTMR, PSC_SMBTMR_SET_TH(0) | PSC_SMBTMR_SET_PS(20) | \
    PSC_SMBTMR_SET_PU(20) | PSC_SMBTMR_SET_SH(20) | \
    PSC_SMBTMR_SET_SU(20) | PSC_SMBTMR_SET_CL(20) | \
    PSC_SMBTMR_SET_CH(20));
    cfg |= PSC_SMBCFG_DE_ENABLE;
    WR(priv, PSC_SMBCFG, cfg);
    while ((RD(priv, PSC_SMBSTAT) & PSC_SMBSTAT_SR) == 0)
    cpu_relax();
    WR(priv, PSC_CTRL, PSC_CTRL_SUSPEND);
    }
#[no_mangle]
unsafe extern "C" fn i2c_au1550_disable(priv: *mut i2c_au1550_data) {
    static void i2c_au1550_disable(struct i2c_au1550_data *priv)
    {
    WR(priv, PSC_SMBCFG, 0);
    WR(priv, PSC_CTRL, PSC_CTRL_DISABLE);
    }
//
// registering functions to load algorithms at runtime
// Prior to calling us, the 50MHz clock frequency and routing
// must have been set up for the PSC indicated by the adapter.
//
    static int
    i2c_au1550_probe(struct platform_device *pdev)
    {
    struct i2c_au1550_data *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(struct i2c_au1550_data),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.psc_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(priv.psc_base))
    return PTR_ERR(priv.psc_base);
    priv.xfer_timeout = 200;
    priv.adap.nr = pdev.id;
    priv.adap.algo = &au1550_algo;
    priv.adap.algo_data = priv;
    priv.adap.dev.parent = &pdev.dev;
    strscpy(priv.adap.name, "Au1xxx PSC I2C", sizeof(priv.adap.name));
// Now, set up the PSC for SMBus PIO mode.
    i2c_au1550_setup(priv);
    ret = i2c_add_numbered_adapter(&priv.adap);
    if (ret) {
    i2c_au1550_disable(priv);
    return ret;
    }
    platform_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_au1550_remove(pdev: *mut platform_device) {
    static void i2c_au1550_remove(struct platform_device *pdev)
    {
    struct i2c_au1550_data *priv = platform_get_drvdata(pdev);
    i2c_del_adapter(&priv.adap);
    i2c_au1550_disable(priv);
    }
#[no_mangle]
unsafe extern "C" fn i2c_au1550_suspend(dev: *mut device) -> c_int {
    static int i2c_au1550_suspend(struct device *dev)
    {
    struct i2c_au1550_data *priv = dev_get_drvdata(dev);
    i2c_au1550_disable(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_au1550_resume(dev: *mut device) -> c_int {
    static int i2c_au1550_resume(struct device *dev)
    {
    struct i2c_au1550_data *priv = dev_get_drvdata(dev);
    i2c_au1550_setup(priv);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(i2c_au1550_pmops,
    i2c_au1550_suspend, i2c_au1550_resume);
    static struct platform_driver au1xpsc_smbus_driver = {
    .driver = {
    .name	= "au1xpsc_smbus",
    .pm	= pm_sleep_ptr(&i2c_au1550_pmops),
    },
    .probe		= i2c_au1550_probe,
    .remove		= i2c_au1550_remove,
    };
    module_platform_driver(au1xpsc_smbus_driver);
    MODULE_AUTHOR("Dan Malek, Embedded Edge, LLC.");
    MODULE_DESCRIPTION("SMBus adapter Alchemy pb1550");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:au1xpsc_smbus");
