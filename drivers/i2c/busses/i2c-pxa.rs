//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-pxa.c
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
// i2c_adap_pxa.c
//
// I2C adapter for the PXA I2C bus access.
//
// Copyright (C) 2002 Intrinsyc Software Inc.
// Copyright (C) 2004-2005 Deep Blue Solutions Ltd.
//
// History:
// Apr 2002: Initial version [CS]
// Jun 2002: Properly separated algo/adap [FB]
// Jan 2003: Fixed several bugs concerning interrupt handling [Kai-Uwe Bloem]
// Jan 2003: added limited signal handling [Kai-Uwe Bloem]
// Sep 2004: Major rework to ensure efficient bus handling [RMK]
// Dec 2004: Added support for PXA27x and slave device probing [Liam Girdwood]
// Feb 2005: Rework slave mode handling [RMK]
//

// I2C register field definitions

pub const ILCR_SLV_SHIFT: c_int = 0;

pub const ILCR_FLV_SHIFT: c_int = 9;

pub const ILCR_HLVL_SHIFT: c_int = 18;

pub const ILCR_HLVH_SHIFT: c_int = 27;

pub const IWCR_CNT_SHIFT: c_int = 0;

pub const IWCR_HS_CNT1_SHIFT: c_int = 5;

pub const IWCR_HS_CNT2_SHIFT: c_int = 10;

// need a longer timeout if we're dealing with the fact we may well be
// looking at a multi-master environment
//
pub const DEF_TIMEOUT: c_int = 32;

// ICR initialize bit values
//
// 15 FM     0 (100 kHz operation)
// 14 UR     0 (No unit reset)
// 13 SADIE  0 (Disables the unit from interrupting on slave addresses
// matching its slave address)
// 12 ALDIE  0 (Disables the unit from interrupt when it loses arbitration
// in master mode)
// 11 SSDIE  0 (Disables interrupts from a slave stop detected, in slave mode)
// 10 BEIE   1 (Enable interrupts from detected bus errors, no ACK sent)
// 9 IRFIE  1 (Enable interrupts from full buffer received)
// 8 ITEIE  1 (Enables the I2C unit to interrupt when transmit buffer empty)
// 7 GCD    1 (Disables i2c unit response to general call messages as a slave)
// 6 IUE    0 (Disable unit until we change settings)
// 5 SCLE   1 (Enables the i2c clock output for master mode (drives SCL)
// 4 MA     0 (Only send stop with the ICR stop bit)
// 3 TB     0 (We are not transmitting a byte initially)
// 2 ACKNAK 0 (Send an ACK after the unit receives a byte)
// 1 STOP   0 (Do not send a STOP)
// 0 START  0 (Do not send a START)
//

// I2C status register init values
//
// 10 BED    1 (Clear bus error detected)
// 9 SAD    1 (Clear slave address detected)
// 7 IRF    1 (Clear IDBR Receive Full)
// 6 ITE    1 (Clear IDBR Transmit Empty)
// 5 ALD    1 (Clear Arbitration Loss Detected)
// 4 SSD    1 (Clear Slave Stop Detected)
//
pub const I2C_ISR_INIT: c_uint = 0x7FF  /* status register init */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_reg_layout {
    pub ibmr: u32,
    pub idbr: u32,
    pub icr: u32,
    pub isr: u32,
    pub isar: u32,
    pub ilcr: u32,
    pub iwcr: u32,
    pub fm: u32,
    pub hs: u32,
}

    enum pxa_i2c_types {
    REGS_PXA2XX,
    REGS_PXA3XX,
    REGS_CE4100,
    REGS_PXA910,
    REGS_A3700,
    };
// I2C register layout definitions
    static struct pxa_reg_layout pxa_reg_layout[] = {
    [REGS_PXA2XX] = {
    .ibmr =	0x00,
    .idbr =	0x08,
    .icr =	0x10,
    .isr =	0x18,
    .isar =	0x20,
    .fm = ICR_FM,
    .hs = ICR_HS,
    },
    [REGS_PXA3XX] = {
    .ibmr =	0x00,
    .idbr =	0x04,
    .icr =	0x08,
    .isr =	0x0c,
    .isar =	0x10,
    .fm = ICR_FM,
    .hs = ICR_HS,
    },
    [REGS_CE4100] = {
    .ibmr =	0x14,
    .idbr =	0x0c,
    .icr =	0x00,
    .isr =	0x04,
// no isar register
    .fm = ICR_FM,
    .hs = ICR_HS,
    },
    [REGS_PXA910] = {
    .ibmr = 0x00,
    .idbr = 0x08,
    .icr =	0x10,
    .isr =	0x18,
    .isar = 0x20,
    .ilcr = 0x28,
    .iwcr = 0x30,
    .fm = ICR_FM,
    .hs = ICR_HS,
    },
    [REGS_A3700] = {
    .ibmr =	0x00,
    .idbr =	0x04,
    .icr =	0x08,
    .isr =	0x0c,
    .isar =	0x10,
    .fm = ICR_A3700_FM,
    .hs = ICR_A3700_HS,
    },
    };
    static const struct of_device_id i2c_pxa_dt_ids[] = {
    { .compatible = "mrvl,pxa-i2c", .data = (void *)REGS_PXA2XX },
    { .compatible = "mrvl,pwri2c", .data = (void *)REGS_PXA3XX },
    { .compatible = "mrvl,mmp-twsi", .data = (void *)REGS_PXA910 },
    { .compatible = "marvell,armada-3700-i2c", .data = (void *)REGS_A3700 },
    {}
    };
    MODULE_DEVICE_TABLE(of, i2c_pxa_dt_ids);
    static const struct platform_device_id i2c_pxa_id_table[] = {
    { .name = "pxa2xx-i2c",		.driver_data = REGS_PXA2XX },
    { .name = "pxa3xx-pwri2c",	.driver_data = REGS_PXA3XX },
    { .name = "ce4100-i2c",		.driver_data = REGS_CE4100 },
    { .name = "pxa910-i2c",		.driver_data = REGS_PXA910 },
    { .name = "armada-3700-i2c",	.driver_data = REGS_A3700 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, i2c_pxa_id_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_i2c {
    pub lock: spinlock_t,
    pub wait: wait_queue_head_t,
    pub msg: *mut i2c_msg,
    pub msg_num: c_uint,
    pub msg_idx: c_uint,
    pub msg_ptr: c_uint,
    pub slave_addr: c_uint,
    pub req_slave_addr: c_uint,
    pub adap: i2c_adapter,
    pub clk: *mut clk,

    pub slave: *mut i2c_client,

    pub irqlogidx: c_uint,
    pub isrlog: [u32; 32],
    pub icrlog: [u32; 32],
    pub reg_base: *mut void __iomem,
    pub reg_ibmr: *mut void __iomem,
    pub reg_idbr: *mut void __iomem,
    pub reg_icr: *mut void __iomem,
    pub reg_isr: *mut void __iomem,
    pub reg_isar: *mut void __iomem,
    pub reg_ilcr: *mut void __iomem,
    pub reg_iwcr: *mut void __iomem,
    pub iobase: c_ulong,
    pub iosize: c_ulong,
    pub irq: c_int,
    pub :1: unsigned int use_pio,
    pub :1: unsigned int fast_mode,
    pub high_mode:1: c_uint,
    pub master_code: c_uchar,
    pub rate: c_ulong,
    pub highmode_enter: bool,
    pub fm_mask: u32,
    pub hs_mask: u32,
    pub busy_mask: u32,
    pub recovery: i2c_bus_recovery_info,
    pub pinctrl: *mut pinctrl,
    pub pinctrl_default: *mut pinctrl_state,
    pub pinctrl_recovery: *mut pinctrl_state,
    pub reset_before_xfer: bool,
}

//
// I2C Slave mode address
//
pub const I2C_PXA_SLAVE_ADDR: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bits {
    pub mask: u32,
    pub set: *const c_char,
    pub unset: *const c_char,
}

    static inline void
    decode_bits(const char *prefix, const struct bits *bits, int num, u32 val)
    {
    printk("%s %08x:", prefix, val);
    while (num--) {
    const char *str = val & bits.mask ? bits.set : bits.unset;
    if (str)
    pr_cont(" %s", str);
    bits++;
    }
    pr_cont("\n");
    }
    static const struct bits isr_bits[] = {
    PXA_BIT(ISR_RWM,	"RX",		"TX"),
    PXA_BIT(ISR_ACKNAK,	"NAK",		"ACK"),
    PXA_BIT(ISR_UB,		"Bsy",		"Rdy"),
    PXA_BIT(ISR_IBB,	"BusBsy",	"BusRdy"),
    PXA_BIT(ISR_SSD,	"SlaveStop",	core::ptr::null_mut()),
    PXA_BIT(ISR_ALD,	"ALD",		core::ptr::null_mut()),
    PXA_BIT(ISR_ITE,	"TxEmpty",	core::ptr::null_mut()),
    PXA_BIT(ISR_IRF,	"RxFull",	core::ptr::null_mut()),
    PXA_BIT(ISR_GCAD,	"GenCall",	core::ptr::null_mut()),
    PXA_BIT(ISR_SAD,	"SlaveAddr",	core::ptr::null_mut()),
    PXA_BIT(ISR_BED,	"BusErr",	core::ptr::null_mut()),
    };
#[no_mangle]
unsafe extern "C" fn decode_ISR(val: c_uint) {
    static void decode_ISR(unsigned int val)
    {
    decode_bits(KERN_DEBUG "ISR", isr_bits, ARRAY_SIZE(isr_bits), val);
    }

    static const struct bits icr_bits[] = {
    PXA_BIT(ICR_START,  "START",	core::ptr::null_mut()),
    PXA_BIT(ICR_STOP,   "STOP",	core::ptr::null_mut()),
    PXA_BIT(ICR_ACKNAK, "ACKNAK",	core::ptr::null_mut()),
    PXA_BIT(ICR_TB,     "TB",	core::ptr::null_mut()),
    PXA_BIT(ICR_MA,     "MA",	core::ptr::null_mut()),
    PXA_BIT(ICR_SCLE,   "SCLE",	"scle"),
    PXA_BIT(ICR_IUE,    "IUE",	"iue"),
    PXA_BIT(ICR_GCD,    "GCD",	core::ptr::null_mut()),
    PXA_BIT(ICR_ITEIE,  "ITEIE",	core::ptr::null_mut()),
    PXA_BIT(ICR_IRFIE,  "IRFIE",	core::ptr::null_mut()),
    PXA_BIT(ICR_BEIE,   "BEIE",	core::ptr::null_mut()),
    PXA_BIT(ICR_SSDIE,  "SSDIE",	core::ptr::null_mut()),
    PXA_BIT(ICR_ALDIE,  "ALDIE",	core::ptr::null_mut()),
    PXA_BIT(ICR_SADIE,  "SADIE",	core::ptr::null_mut()),
    PXA_BIT(ICR_UR,     "UR",		"ur"),
    };
#[no_mangle]
unsafe extern "C" fn decode_ICR(val: c_uint) {
    static void decode_ICR(unsigned int val)
    {
    decode_bits(KERN_DEBUG "ICR", icr_bits, ARRAY_SIZE(icr_bits), val);
    }

    let mut i2c_debug: static unsigned int = DEBUG;
#[no_mangle]
unsafe extern "C" fn i2c_pxa_show_state(i2c: *mut pxa_i2c, lno: c_int, fname: *const c_char) {
    static void i2c_pxa_show_state(struct pxa_i2c *i2c, int lno, const char *fname)
    {
    dev_dbg(&i2c.adap.dev, "state:%s:%d: ISR=%08x, ICR=%08x, IBMR=%02x\n", fname, lno,
    readl(_ISR(i2c)), readl(_ICR(i2c)), readl(_IBMR(i2c)));
    }

#[no_mangle]
unsafe extern "C" fn i2c_pxa_scream_blue_murder(i2c: *mut pxa_i2c, why: *const c_char) {
    static void i2c_pxa_scream_blue_murder(struct pxa_i2c *i2c, const char *why)
    {
    unsigned int i;
    struct device *dev = &i2c.adap.dev;
    dev_err(dev, "slave_0x%x error: %s\n",
    i2c.req_slave_addr >> 1, why);
    dev_err(dev, "msg_num: %d msg_idx: %d msg_ptr: %d\n",
    i2c.msg_num, i2c.msg_idx, i2c.msg_ptr);
    dev_err(dev, "IBMR: %08x IDBR: %08x ICR: %08x ISR: %08x\n",
    readl(_IBMR(i2c)), readl(_IDBR(i2c)), readl(_ICR(i2c)),
    readl(_ISR(i2c)));
    dev_err(dev, "log:");
    for (i = 0; i < i2c.irqlogidx; i++)
    pr_cont(" [%03x:%05x]", i2c.isrlog[i], i2c.icrlog[i]);
    pr_cont("\n");
    }

pub const i2c_debug: c_int = 0;

    static void i2c_pxa_master_complete(struct pxa_i2c *i2c, int ret);
#[no_mangle]
pub unsafe extern "C" fn i2c_pxa_is_slavemode(i2c: *mut pxa_i2c) -> c_int {
    static inline int i2c_pxa_is_slavemode(struct pxa_i2c *i2c)
    {
    return !(readl(_ICR(i2c)) & ICR_SCLE);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_abort(i2c: *mut pxa_i2c) {
    static void i2c_pxa_abort(struct pxa_i2c *i2c)
    {
    let mut i: c_int = 250;
    if (i2c_pxa_is_slavemode(i2c)) {
    dev_dbg(&i2c.adap.dev, "%s: called in slave mode\n", __func__);
    return;
    }
    while ((i > 0) && (readl(_IBMR(i2c)) & IBMR_SDAS) == 0) {
    let mut icr: c_ulong = readl(_ICR(i2c));
    icr &= ~ICR_START;
    icr |= ICR_ACKNAK | ICR_STOP | ICR_TB;
    writel(icr, _ICR(i2c));
    show_state(i2c);
    mdelay(1);
    i --;
    }
    writel(readl(_ICR(i2c)) & ~(ICR_MA | ICR_START | ICR_STOP),
    _ICR(i2c));
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_wait_bus_not_busy(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_wait_bus_not_busy(struct pxa_i2c *i2c)
    {
    let mut timeout: c_int = DEF_TIMEOUT;
    u32 isr;
    while (1) {
    isr = readl(_ISR(i2c));
    if (!(isr & i2c.busy_mask))
    return 0;
    if (isr & ISR_SAD)
    timeout += 4;
    if (!timeout--)
    break;
    msleep(2);
    show_state(i2c);
    }
    show_state(i2c);
    return I2C_RETRY;
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_wait_master(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_wait_master(struct pxa_i2c *i2c)
    {
    let mut timeout: c_ulong = jiffies + HZ*4;
    while (time_before(jiffies, timeout)) {
    if (i2c_debug > 1)
    dev_dbg(&i2c.adap.dev, "%s: %ld: ISR=%08x, ICR=%08x, IBMR=%02x\n",
    __func__, (long)jiffies, readl(_ISR(i2c)), readl(_ICR(i2c)), readl(_IBMR(i2c)));
    if (readl(_ISR(i2c)) & ISR_SAD) {
    if (i2c_debug > 0)
    dev_dbg(&i2c.adap.dev, "%s: Slave detected\n", __func__);
    goto out;
    }
// wait for unit and bus being not busy, and we also do a
// quick check of the i2c lines themselves to ensure they've
// gone high...
//
    if ((readl(_ISR(i2c)) & i2c.busy_mask) == 0 &&
    readl(_IBMR(i2c)) == (IBMR_SCLS | IBMR_SDAS)) {
    if (i2c_debug > 0)
    dev_dbg(&i2c.adap.dev, "%s: done\n", __func__);
    return 1;
    }
    msleep(1);
    }
    if (i2c_debug > 0)
    dev_dbg(&i2c.adap.dev, "%s: did not free\n", __func__);
    out:
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_set_master(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_set_master(struct pxa_i2c *i2c)
    {
    if (i2c_debug)
    dev_dbg(&i2c.adap.dev, "setting to bus master\n");
    if ((readl(_ISR(i2c)) & i2c.busy_mask) != 0) {
    dev_dbg(&i2c.adap.dev, "%s: unit is busy\n", __func__);
    if (!i2c_pxa_wait_master(i2c)) {
    dev_dbg(&i2c.adap.dev, "%s: error: unit busy\n", __func__);
    return I2C_RETRY;
    }
    }
    writel(readl(_ICR(i2c)) | ICR_SCLE, _ICR(i2c));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn i2c_pxa_wait_slave(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_wait_slave(struct pxa_i2c *i2c)
    {
    let mut timeout: c_ulong = jiffies + HZ*1;
// wait for stop
    show_state(i2c);
    while (time_before(jiffies, timeout)) {
    if (i2c_debug > 1)
    dev_dbg(&i2c.adap.dev, "%s: %ld: ISR=%08x, ICR=%08x, IBMR=%02x\n",
    __func__, (long)jiffies, readl(_ISR(i2c)), readl(_ICR(i2c)), readl(_IBMR(i2c)));
    if ((readl(_ISR(i2c)) & i2c.busy_mask) == 0 ||
    (readl(_ISR(i2c)) & ISR_SAD) != 0 ||
    (readl(_ICR(i2c)) & ICR_SCLE) == 0) {
    if (i2c_debug > 1)
    dev_dbg(&i2c.adap.dev, "%s: done\n", __func__);
    return 1;
    }
    msleep(1);
    }
    if (i2c_debug > 0)
    dev_dbg(&i2c.adap.dev, "%s: did not free\n", __func__);
    return 0;
    }
//
// clear the hold on the bus, and take of anything else
// that has been configured
//
#[no_mangle]
unsafe extern "C" fn i2c_pxa_set_slave(i2c: *mut pxa_i2c, errcode: c_int) {
    static void i2c_pxa_set_slave(struct pxa_i2c *i2c, int errcode)
    {
    show_state(i2c);
    if (errcode < 0) {
    udelay(100);   /* simple delay */
    } else {
// we need to wait for the stop condition to end
// if we where in stop, then clear...
    if (readl(_ICR(i2c)) & ICR_STOP) {
    udelay(100);
    writel(readl(_ICR(i2c)) & ~ICR_STOP, _ICR(i2c));
    }
    if (!i2c_pxa_wait_slave(i2c)) {
    dev_err(&i2c.adap.dev, "%s: wait timedout\n",
    __func__);
    return;
    }
    }
    writel(readl(_ICR(i2c)) & ~(ICR_STOP|ICR_ACKNAK|ICR_MA), _ICR(i2c));
    writel(readl(_ICR(i2c)) & ~ICR_SCLE, _ICR(i2c));
    if (i2c_debug) {
    dev_dbg(&i2c.adap.dev, "ICR now %08x, ISR %08x\n", readl(_ICR(i2c)), readl(_ISR(i2c)));
    decode_ICR(readl(_ICR(i2c)));
    }
    }

#[no_mangle]
unsafe extern "C" fn i2c_pxa_do_reset(i2c: *mut pxa_i2c) {
    static void i2c_pxa_do_reset(struct pxa_i2c *i2c)
    {
// reset according to 9.8
    writel(ICR_UR, _ICR(i2c));
    writel(I2C_ISR_INIT, _ISR(i2c));
    writel(readl(_ICR(i2c)) & ~ICR_UR, _ICR(i2c));
    if (i2c.reg_isar && IS_ENABLED(CONFIG_I2C_PXA_SLAVE))
    writel(i2c.slave_addr, _ISAR(i2c));
// set control register values
    writel(I2C_ICR_INIT | (i2c.fast_mode ? i2c.fm_mask : 0), _ICR(i2c));
    writel(readl(_ICR(i2c)) | (i2c.high_mode ? i2c.hs_mask : 0), _ICR(i2c));

    dev_info(&i2c.adap.dev, "Enabling slave mode\n");
    writel(readl(_ICR(i2c)) | ICR_SADIE | ICR_ALDIE | ICR_SSDIE, _ICR(i2c));

    i2c_pxa_set_slave(i2c, 0);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_enable(i2c: *mut pxa_i2c) {
    static void i2c_pxa_enable(struct pxa_i2c *i2c)
    {
// enable unit
    writel(readl(_ICR(i2c)) | ICR_IUE, _ICR(i2c));
    udelay(100);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_reset(i2c: *mut pxa_i2c) {
    static void i2c_pxa_reset(struct pxa_i2c *i2c)
    {
    pr_debug("Resetting I2C Controller Unit\n");
// abort any transfer currently under way
    i2c_pxa_abort(i2c);
    i2c_pxa_do_reset(i2c);
    i2c_pxa_enable(i2c);
    }

//
// PXA I2C Slave mode
//
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_txempty(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_slave_txempty(struct pxa_i2c *i2c, u32 isr)
    {
    if (isr & ISR_BED) {
// what should we do here?
    } else {
    let mut byte: u8 = 0;
    if (i2c.slave != core::ptr::null_mut())
    i2c_slave_event(i2c.slave, I2C_SLAVE_READ_PROCESSED,
    &byte);
    writel(byte, _IDBR(i2c));
    writel(readl(_ICR(i2c)) | ICR_TB, _ICR(i2c));   /* allow next byte */
    }
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_rxfull(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_slave_rxfull(struct pxa_i2c *i2c, u32 isr)
    {
    let mut byte: u8 = readl(_IDBR(i2c));
    if (i2c.slave != core::ptr::null_mut())
    i2c_slave_event(i2c.slave, I2C_SLAVE_WRITE_RECEIVED, &byte);
    writel(readl(_ICR(i2c)) | ICR_TB, _ICR(i2c));
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_start(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_slave_start(struct pxa_i2c *i2c, u32 isr)
    {
    int timeout;
    if (i2c_debug > 0)
    dev_dbg(&i2c.adap.dev, "SAD, mode is slave-%cx\n",
    (isr & ISR_RWM) ? 'r' : 't');
    if (i2c.slave != core::ptr::null_mut()) {
    if (isr & ISR_RWM) {
    let mut byte: u8 = 0;
    i2c_slave_event(i2c.slave, I2C_SLAVE_READ_REQUESTED,
    &byte);
    writel(byte, _IDBR(i2c));
    } else {
    i2c_slave_event(i2c.slave, I2C_SLAVE_WRITE_REQUESTED,
    core::ptr::null_mut());
    }
    }
//
// slave could interrupt in the middle of us generating a
// start condition... if this happens, we'd better back off
// and stop holding the poor thing up
//
    writel(readl(_ICR(i2c)) & ~(ICR_START|ICR_STOP), _ICR(i2c));
    writel(readl(_ICR(i2c)) | ICR_TB, _ICR(i2c));
    timeout = 0x10000;
    while (1) {
    if ((readl(_IBMR(i2c)) & IBMR_SCLS) == IBMR_SCLS)
    break;
    timeout--;
    if (timeout <= 0) {
    dev_err(&i2c.adap.dev, "timeout waiting for SCL high\n");
    break;
    }
    }
    writel(readl(_ICR(i2c)) & ~ICR_SCLE, _ICR(i2c));
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_stop(i2c: *mut pxa_i2c) {
    static void i2c_pxa_slave_stop(struct pxa_i2c *i2c)
    {
    if (i2c_debug > 2)
    dev_dbg(&i2c.adap.dev, "ISR: SSD (Slave Stop)\n");
    if (i2c.slave != core::ptr::null_mut())
    i2c_slave_event(i2c.slave, I2C_SLAVE_STOP, core::ptr::null_mut());
    if (i2c_debug > 2)
    dev_dbg(&i2c.adap.dev, "ISR: SSD (Slave Stop) acked\n");
//
// If we have a master-mode message waiting,
// kick it off now that the slave has completed.
//
    if (i2c.msg)
    i2c_pxa_master_complete(i2c, I2C_RETRY);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_reg(slave: *mut i2c_client) -> c_int {
    static int i2c_pxa_slave_reg(struct i2c_client *slave)
    {
    struct pxa_i2c *i2c = slave.adapter.algo_data;
    if (i2c.slave)
    return -EBUSY;
    if (!i2c.reg_isar)
    return -EAFNOSUPPORT;
    i2c.slave = slave;
    i2c.slave_addr = slave.addr;
    writel(i2c.slave_addr, _ISAR(i2c));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_unreg(slave: *mut i2c_client) -> c_int {
    static int i2c_pxa_slave_unreg(struct i2c_client *slave)
    {
    struct pxa_i2c *i2c = slave.adapter.algo_data;
    WARN_ON(!i2c.slave);
    i2c.slave_addr = I2C_PXA_SLAVE_ADDR;
    writel(i2c.slave_addr, _ISAR(i2c));
    i2c.slave = core::ptr::null_mut();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_txempty(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_slave_txempty(struct pxa_i2c *i2c, u32 isr)
    {
    if (isr & ISR_BED) {
// what should we do here?
    } else {
    writel(0, _IDBR(i2c));
    writel(readl(_ICR(i2c)) | ICR_TB, _ICR(i2c));
    }
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_rxfull(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_slave_rxfull(struct pxa_i2c *i2c, u32 isr)
    {
    writel(readl(_ICR(i2c)) | ICR_TB | ICR_ACKNAK, _ICR(i2c));
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_start(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_slave_start(struct pxa_i2c *i2c, u32 isr)
    {
    int timeout;
//
// slave could interrupt in the middle of us generating a
// start condition... if this happens, we'd better back off
// and stop holding the poor thing up
//
    writel(readl(_ICR(i2c)) & ~(ICR_START|ICR_STOP), _ICR(i2c));
    writel(readl(_ICR(i2c)) | ICR_TB | ICR_ACKNAK, _ICR(i2c));
    timeout = 0x10000;
    while (1) {
    if ((readl(_IBMR(i2c)) & IBMR_SCLS) == IBMR_SCLS)
    break;
    timeout--;
    if (timeout <= 0) {
    dev_err(&i2c.adap.dev, "timeout waiting for SCL high\n");
    break;
    }
    }
    writel(readl(_ICR(i2c)) & ~ICR_SCLE, _ICR(i2c));
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_slave_stop(i2c: *mut pxa_i2c) {
    static void i2c_pxa_slave_stop(struct pxa_i2c *i2c)
    {
    if (i2c.msg)
    i2c_pxa_master_complete(i2c, I2C_RETRY);
    }

//
// PXA I2C Master mode
//
#[no_mangle]
pub unsafe extern "C" fn i2c_pxa_start_message(i2c: *mut pxa_i2c) {
    static inline void i2c_pxa_start_message(struct pxa_i2c *i2c)
    {
    u32 icr;
//
// Step 1: target slave address into IDBR
//
    i2c.req_slave_addr = i2c_8bit_addr_from_msg(i2c.msg);
    writel(i2c.req_slave_addr, _IDBR(i2c));
//
// Step 2: initiate the write.
//
    icr = readl(_ICR(i2c)) & ~(ICR_STOP | ICR_ALDIE);
    writel(icr | ICR_START | ICR_TB, _ICR(i2c));
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_pxa_stop_message(i2c: *mut pxa_i2c) {
    static inline void i2c_pxa_stop_message(struct pxa_i2c *i2c)
    {
    u32 icr;
// Clear the START, STOP, ACK, TB and MA flags
    icr = readl(_ICR(i2c));
    icr &= ~(ICR_START | ICR_STOP | ICR_ACKNAK | ICR_TB | ICR_MA);
    writel(icr, _ICR(i2c));
    }
//
// PXA I2C send master code
// 1. Load master code to IDBR and send it.
// Note for HS mode, set ICR [GPIOEN].
// 2. Wait until win arbitration.
//
#[no_mangle]
unsafe extern "C" fn i2c_pxa_send_mastercode(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_send_mastercode(struct pxa_i2c *i2c)
    {
    u32 icr;
    long time_left;
    spin_lock_irq(&i2c.lock);
    i2c.highmode_enter = true;
    writel(i2c.master_code, _IDBR(i2c));
    icr = readl(_ICR(i2c)) & ~(ICR_STOP | ICR_ALDIE);
    icr |= ICR_GPIOEN | ICR_START | ICR_TB | ICR_ITEIE;
    writel(icr, _ICR(i2c));
    spin_unlock_irq(&i2c.lock);
    time_left = wait_event_timeout(i2c.wait,
    i2c.highmode_enter == false, HZ * 1);
    i2c.highmode_enter = false;
    return (time_left == 0) ? I2C_RETRY : 0;
    }
//
// i2c_pxa_master_complete - complete the message and wake up.
//
#[no_mangle]
unsafe extern "C" fn i2c_pxa_master_complete(i2c: *mut pxa_i2c, ret: c_int) {
    static void i2c_pxa_master_complete(struct pxa_i2c *i2c, int ret)
    {
    i2c.msg_ptr = 0;
    i2c.msg = core::ptr::null_mut();
    i2c.msg_idx ++;
    i2c.msg_num = 0;
    if (ret)
    i2c.msg_idx = ret;
    if (!i2c.use_pio)
    wake_up(&i2c.wait);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_irq_txempty(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_irq_txempty(struct pxa_i2c *i2c, u32 isr)
    {
    let mut icr: u32 = readl(_ICR(i2c)) & ~(ICR_START|ICR_STOP|ICR_ACKNAK|ICR_TB);
    again:
//
// If ISR_ALD is set, we lost arbitration.
//
    if (isr & ISR_ALD) {
//
// Do we need to do anything here?  The PXA docs
// are vague about what happens.
//
    i2c_pxa_scream_blue_murder(i2c, "ALD set");
//
// We ignore this error.  We seem to see spurious ALDs
// for seemingly no reason.  If we handle them as I think
// they should, we end up causing an I2C error, which
// is painful for some systems.
//
    return; /* ignore */
    }
    if ((isr & ISR_BED) &&
    (!((i2c.msg.flags & I2C_M_IGNORE_NAK) &&
    (isr & ISR_ACKNAK)))) {
    let mut ret: c_int = BUS_ERROR;
//
// I2C bus error - either the device NAK'd us, or
// something more serious happened.  If we were NAK'd
// on the initial address phase, we can retry.
//
    if (isr & ISR_ACKNAK) {
    if (i2c.msg_ptr == 0 && i2c.msg_idx == 0)
    ret = NO_SLAVE;
    else
    ret = XFER_NAKED;
    }
    i2c_pxa_master_complete(i2c, ret);
    } else if (isr & ISR_RWM) {
//
// Read mode.  We have just sent the address byte, and
// now we must initiate the transfer.
//
    if (i2c.msg_ptr == i2c.msg.len - 1 &&
    i2c.msg_idx == i2c.msg_num - 1)
    icr |= ICR_STOP | ICR_ACKNAK;
    icr |= ICR_ALDIE | ICR_TB;
    } else if (i2c.msg_ptr < i2c.msg.len) {
//
// Write mode.  Write the next data byte.
//
    writel(i2c.msg.buf[i2c.msg_ptr++], _IDBR(i2c));
    icr |= ICR_ALDIE | ICR_TB;
//
// If this is the last byte of the last message or last byte
// of any message with I2C_M_STOP (e.g. SCCB), send a STOP.
//
    if ((i2c.msg_ptr == i2c.msg.len) &&
    ((i2c.msg.flags & I2C_M_STOP) ||
    (i2c.msg_idx == i2c.msg_num - 1)))
    icr |= ICR_STOP;
    } else if (i2c.msg_idx < i2c.msg_num - 1) {
//
// Next segment of the message.
//
    i2c.msg_ptr = 0;
    i2c.msg_idx ++;
    i2c.msg++;
//
// If we aren't doing a repeated start and address,
// go back and try to send the next byte.  Note that
// we do not support switching the R/W direction here.
//
    if (i2c.msg.flags & I2C_M_NOSTART)
    goto again;
//
// Write the next address.
//
    i2c.req_slave_addr = i2c_8bit_addr_from_msg(i2c.msg);
    writel(i2c.req_slave_addr, _IDBR(i2c));
//
// And trigger a repeated start, and send the byte.
//
    icr &= ~ICR_ALDIE;
    icr |= ICR_START | ICR_TB;
    } else {
    if (i2c.msg.len == 0)
    icr |= ICR_MA;
    i2c_pxa_master_complete(i2c, 0);
    }
    i2c.icrlog[i2c.irqlogidx-1] = icr;
    writel(icr, _ICR(i2c));
    show_state(i2c);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_irq_rxfull(i2c: *mut pxa_i2c, isr: u32) {
    static void i2c_pxa_irq_rxfull(struct pxa_i2c *i2c, u32 isr)
    {
    let mut icr: u32 = readl(_ICR(i2c)) & ~(ICR_START|ICR_STOP|ICR_ACKNAK|ICR_TB);
//
// Read the byte.
//
    i2c.msg.buf[i2c.msg_ptr++] = readl(_IDBR(i2c));
    if (i2c.msg_ptr < i2c.msg.len) {
//
// If this is the last byte of the last
// message, send a STOP.
//
    if (i2c.msg_ptr == i2c.msg.len - 1)
    icr |= ICR_STOP | ICR_ACKNAK;
    icr |= ICR_ALDIE | ICR_TB;
    } else {
    i2c_pxa_master_complete(i2c, 0);
    }
    i2c.icrlog[i2c.irqlogidx-1] = icr;
    writel(icr, _ICR(i2c));
    }

    ISR_SAD | ISR_BED)
#[no_mangle]
unsafe extern "C" fn i2c_pxa_handler(this_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t i2c_pxa_handler(int this_irq, void *dev_id)
    {
    struct pxa_i2c *i2c = dev_id;
    let mut isr: u32 = readl(_ISR(i2c));
    if (!(isr & VALID_INT_SOURCE))
    return IRQ_NONE;
    if (i2c_debug > 2 && 0) {
    dev_dbg(&i2c.adap.dev, "%s: ISR=%08x, ICR=%08x, IBMR=%02x\n",
    __func__, isr, readl(_ICR(i2c)), readl(_IBMR(i2c)));
    decode_ISR(isr);
    }
    if (i2c.irqlogidx < ARRAY_SIZE(i2c.isrlog))
    i2c.isrlog[i2c.irqlogidx++] = isr;
    show_state(i2c);
//
// Always clear all pending IRQs.
//
    writel(isr & VALID_INT_SOURCE, _ISR(i2c));
    if (isr & ISR_SAD)
    i2c_pxa_slave_start(i2c, isr);
    if (isr & ISR_SSD)
    i2c_pxa_slave_stop(i2c);
    if (i2c_pxa_is_slavemode(i2c)) {
    if (isr & ISR_ITE)
    i2c_pxa_slave_txempty(i2c, isr);
    if (isr & ISR_IRF)
    i2c_pxa_slave_rxfull(i2c, isr);
    } else if (i2c.msg && (!i2c.highmode_enter)) {
    if (isr & ISR_ITE)
    i2c_pxa_irq_txempty(i2c, isr);
    if (isr & ISR_IRF)
    i2c_pxa_irq_rxfull(i2c, isr);
    } else if ((isr & ISR_ITE) && i2c.highmode_enter) {
    i2c.highmode_enter = false;
    wake_up(&i2c.wait);
    } else {
    i2c_pxa_scream_blue_murder(i2c, "spurious irq");
    }
    return IRQ_HANDLED;
    }
//
// We are protected by the adapter bus mutex.
//
#[no_mangle]
unsafe extern "C" fn i2c_pxa_do_xfer(i2c: *mut pxa_i2c, msg: *mut i2c_msg, num: c_int) -> c_int {
    static int i2c_pxa_do_xfer(struct pxa_i2c *i2c, struct i2c_msg *msg, int num)
    {
    long time_left;
    int ret;
//
// Wait for the bus to become free.
//
    ret = i2c_pxa_wait_bus_not_busy(i2c);
    if (ret) {
    dev_err(&i2c.adap.dev, "i2c_pxa: timeout waiting for bus free\n");
    i2c_recover_bus(&i2c.adap);
    goto out;
    }
//
// Set master mode.
//
    ret = i2c_pxa_set_master(i2c);
    if (ret) {
    dev_err(&i2c.adap.dev, "i2c_pxa_set_master: error %d\n", ret);
    goto out;
    }
    if (i2c.high_mode) {
    ret = i2c_pxa_send_mastercode(i2c);
    if (ret) {
    dev_err(&i2c.adap.dev, "i2c_pxa_send_mastercode timeout\n");
    goto out;
    }
    }
    spin_lock_irq(&i2c.lock);
    i2c.msg = msg;
    i2c.msg_num = num;
    i2c.msg_idx = 0;
    i2c.msg_ptr = 0;
    i2c.irqlogidx = 0;
    i2c_pxa_start_message(i2c);
    spin_unlock_irq(&i2c.lock);
//
// The rest of the processing occurs in the interrupt handler.
//
    time_left = wait_event_timeout(i2c.wait, i2c.msg_num == 0, HZ * 5);
    i2c_pxa_stop_message(i2c);
//
// We place the return code in i2c->msg_idx.
//
    ret = i2c.msg_idx;
    if (!time_left && i2c.msg_num) {
    i2c_pxa_scream_blue_murder(i2c, "timeout with active message");
    i2c_recover_bus(&i2c.adap);
    ret = I2C_RETRY;
    }
    out:
    return ret;
    }
    static int i2c_pxa_internal_xfer(struct pxa_i2c *i2c,
    struct i2c_msg *msgs, int num,
    int (*xfer)(struct pxa_i2c *,
    struct i2c_msg *, int num))
    {
    int ret, i;
    for (i = 0; ; ) {
    ret = xfer(i2c, msgs, num);
    if (ret != I2C_RETRY && ret != NO_SLAVE)
    goto out;
    if (++i >= i2c.adap.retries)
    break;
    if (i2c_debug)
    dev_dbg(&i2c.adap.dev, "Retrying transmission\n");
    udelay(100);
    }
    if (ret != NO_SLAVE)
    i2c_pxa_scream_blue_murder(i2c, "exhausted retries");
    ret = -EREMOTEIO;
    out:
    i2c_pxa_set_slave(i2c, ret);
    return ret;
    }
    static int i2c_pxa_xfer(struct i2c_adapter *adap,
    struct i2c_msg msgs[], int num)
    {
    struct pxa_i2c *i2c = adap.algo_data;
    if (i2c.reset_before_xfer) {
    i2c_pxa_reset(i2c);
    i2c.reset_before_xfer = false;
    }
    return i2c_pxa_internal_xfer(i2c, msgs, num, i2c_pxa_do_xfer);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 i2c_pxa_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL |
    I2C_FUNC_PROTOCOL_MANGLING | I2C_FUNC_NOSTART;
    }
    static const struct i2c_algorithm i2c_pxa_algorithm = {
    .xfer = i2c_pxa_xfer,
    .functionality = i2c_pxa_functionality,

    .reg_slave = i2c_pxa_slave_reg,
    .unreg_slave = i2c_pxa_slave_unreg,

    };
// Non-interrupt mode support
#[no_mangle]
unsafe extern "C" fn i2c_pxa_pio_set_master(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_pio_set_master(struct pxa_i2c *i2c)
    {
// make timeout the same as for interrupt based functions
    let mut timeout: c_long = 2 * DEF_TIMEOUT;
//
// Wait for the bus to become free.
//
    while (timeout-- && readl(_ISR(i2c)) & i2c.busy_mask)
    udelay(1000);
    if (timeout < 0) {
    show_state(i2c);
    dev_err(&i2c.adap.dev,
    "i2c_pxa: timeout waiting for bus free (set_master)\n");
    return I2C_RETRY;
    }
//
// Set master mode.
//
    writel(readl(_ICR(i2c)) | ICR_SCLE, _ICR(i2c));
    return 0;
    }
    static int i2c_pxa_do_pio_xfer(struct pxa_i2c *i2c,
    struct i2c_msg *msg, int num)
    {
    unsigned long timeout = 500000; /* 5 seconds */
    let mut ret: c_int = 0;
    ret = i2c_pxa_pio_set_master(i2c);
    if (ret)
    goto out;
    i2c.msg = msg;
    i2c.msg_num = num;
    i2c.msg_idx = 0;
    i2c.msg_ptr = 0;
    i2c.irqlogidx = 0;
    i2c_pxa_start_message(i2c);
    while (i2c.msg_num > 0 && --timeout) {
    i2c_pxa_handler(0, i2c);
    udelay(10);
    }
    i2c_pxa_stop_message(i2c);
//
// We place the return code in i2c->msg_idx.
//
    ret = i2c.msg_idx;
    out:
    if (timeout == 0) {
    i2c_pxa_scream_blue_murder(i2c, "timeout (do_pio_xfer)");
    ret = I2C_RETRY;
    }
    return ret;
    }
    static int i2c_pxa_pio_xfer(struct i2c_adapter *adap,
    struct i2c_msg msgs[], int num)
    {
    struct pxa_i2c *i2c = adap.algo_data;
// If the I2C controller is disabled we need to reset it
    (probably due to a suspend/resume destroying state). We do
    this here as we can then avoid worrying about resuming the
    controller before its users. */
    if (!(readl(_ICR(i2c)) & ICR_IUE))
    i2c_pxa_reset(i2c);
    return i2c_pxa_internal_xfer(i2c, msgs, num, i2c_pxa_do_pio_xfer);
    }
    static const struct i2c_algorithm i2c_pxa_pio_algorithm = {
    .xfer = i2c_pxa_pio_xfer,
    .functionality = i2c_pxa_functionality,

    .reg_slave = i2c_pxa_slave_reg,
    .unreg_slave = i2c_pxa_slave_unreg,

    };
    static int i2c_pxa_probe_dt(struct platform_device *pdev, struct pxa_i2c *i2c,
    enum pxa_i2c_types *i2c_types)
    {
    struct device_node *np = pdev.dev.of_node;
    if (!pdev.dev.of_node)
    return 1;
// For device tree we always use the dynamic or alias-assigned ID
    i2c.adap.nr = -1;
    i2c.use_pio = of_property_read_bool(np, "mrvl,i2c-polling");
    i2c.fast_mode = of_property_read_bool(np, "mrvl,i2c-fast-mode");
// i2c_types = (kernel_ulong_t)device_get_match_data(&pdev->dev);
    return 0;
    }
    static int i2c_pxa_probe_pdata(struct platform_device *pdev,
    struct pxa_i2c *i2c,
    enum pxa_i2c_types *i2c_types)
    {
    struct i2c_pxa_platform_data *plat = dev_get_platdata(&pdev.dev);
    const struct platform_device_id *id = platform_get_device_id(pdev);
// i2c_types = id->driver_data;
    if (plat) {
    i2c.use_pio = plat.use_pio;
    i2c.fast_mode = plat.fast_mode;
    i2c.high_mode = plat.high_mode;
    i2c.master_code = plat.master_code;
    if (!i2c.master_code)
    i2c.master_code = 0xe;
    i2c.rate = plat.rate;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_prepare_recovery(adap: *mut i2c_adapter) {
    static void i2c_pxa_prepare_recovery(struct i2c_adapter *adap)
    {
    struct pxa_i2c *i2c = adap.algo_data;
    let mut ibmr: u32 = readl(_IBMR(i2c));
//
// Program the GPIOs to reflect the current I2C bus state while
// we transition to recovery; this avoids glitching the bus.
//
    gpiod_set_value(i2c.recovery.scl_gpiod, ibmr & IBMR_SCLS);
    gpiod_set_value(i2c.recovery.sda_gpiod, ibmr & IBMR_SDAS);
    WARN_ON(pinctrl_select_state(i2c.pinctrl, i2c.pinctrl_recovery));
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_unprepare_recovery(adap: *mut i2c_adapter) {
    static void i2c_pxa_unprepare_recovery(struct i2c_adapter *adap)
    {
    struct pxa_i2c *i2c = adap.algo_data;
    u32 isr;
//
// The bus should now be free. Clear up the I2C controller before
// handing control of the bus back to avoid the bus changing state.
//
    isr = readl(_ISR(i2c));
    if (isr & i2c.busy_mask) {
    dev_dbg(&i2c.adap.dev,
    "recovery: resetting controller, ISR=0x%08x\n", isr);
    i2c_pxa_do_reset(i2c);
    }
    WARN_ON(pinctrl_select_state(i2c.pinctrl, i2c.pinctrl_default));
    dev_dbg(&i2c.adap.dev, "recovery: IBMR 0x%08x ISR 0x%08x\n",
    readl(_IBMR(i2c)), readl(_ISR(i2c)));
    i2c_pxa_enable(i2c);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_init_recovery(i2c: *mut pxa_i2c) -> c_int {
    static int i2c_pxa_init_recovery(struct pxa_i2c *i2c)
    {
    struct i2c_bus_recovery_info *bri = &i2c.recovery;
    struct device *dev = i2c.adap.dev.parent;
//
// When slave mode is enabled, we are not the only master on the bus.
// Bus recovery can only be performed when we are the master, which
// we can't be certain of. Therefore, when slave mode is enabled, do
// not configure bus recovery.
//
    if (IS_ENABLED(CONFIG_I2C_PXA_SLAVE))
    return 0;
    i2c.pinctrl = devm_pinctrl_get(dev);
    if (PTR_ERR(i2c.pinctrl) == -ENODEV)
    i2c.pinctrl = core::ptr::null_mut();
    if (IS_ERR(i2c.pinctrl))
    return PTR_ERR(i2c.pinctrl);
    if (!i2c.pinctrl)
    return 0;
    i2c.pinctrl_default = pinctrl_lookup_state(i2c.pinctrl,
    PINCTRL_STATE_DEFAULT);
    i2c.pinctrl_recovery = pinctrl_lookup_state(i2c.pinctrl, "recovery");
    if (IS_ERR(i2c.pinctrl_default) || IS_ERR(i2c.pinctrl_recovery)) {
    dev_info(dev, "missing pinmux recovery information: %ld %ld\n",
    PTR_ERR(i2c.pinctrl_default),
    PTR_ERR(i2c.pinctrl_recovery));
    return 0;
    }
//
// Claiming GPIOs can influence the pinmux state, and may glitch the
// I2C bus. Do this carefully.
//
    bri.scl_gpiod = devm_gpiod_get(dev, "scl", GPIOD_OUT_HIGH_OPEN_DRAIN);
    if (bri.scl_gpiod == ERR_PTR(-EPROBE_DEFER))
    return -EPROBE_DEFER;
    if (IS_ERR(bri.scl_gpiod)) {
    dev_info(dev, "missing scl gpio recovery information: %pe\n",
    bri.scl_gpiod);
    return 0;
    }
//
// We have SCL. Pull SCL low and wait a bit so that SDA glitches
// have no effect.
//
    gpiod_direction_output(bri.scl_gpiod, 0);
    udelay(10);
    bri.sda_gpiod = devm_gpiod_get(dev, "sda", GPIOD_OUT_HIGH_OPEN_DRAIN);
// Wait a bit in case of a SDA glitch, and then release SCL.
    udelay(10);
    gpiod_direction_output(bri.scl_gpiod, 1);
    if (bri.sda_gpiod == ERR_PTR(-EPROBE_DEFER))
    return -EPROBE_DEFER;
    if (IS_ERR(bri.sda_gpiod)) {
    dev_info(dev, "missing sda gpio recovery information: %pe\n",
    bri.sda_gpiod);
    return 0;
    }
    bri.prepare_recovery = i2c_pxa_prepare_recovery;
    bri.unprepare_recovery = i2c_pxa_unprepare_recovery;
    bri.recover_bus = i2c_generic_scl_recovery;
    i2c.adap.bus_recovery_info = bri;
//
// Claiming GPIOs can change the pinmux state, which confuses the
// pinctrl since pinctrl's idea of the current setting is unaffected
// by the pinmux change caused by claiming the GPIO. Work around that
// by switching pinctrl to the GPIO state here. We do it this way to
// avoid glitching the I2C bus.
//
    pinctrl_select_state(i2c.pinctrl, i2c.pinctrl_recovery);
    return pinctrl_select_state(i2c.pinctrl, i2c.pinctrl_default);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_probe(dev: *mut platform_device) -> c_int {
    static int i2c_pxa_probe(struct platform_device *dev)
    {
    struct i2c_pxa_platform_data *plat = dev_get_platdata(&dev.dev);
    enum pxa_i2c_types i2c_type;
    struct pxa_i2c *i2c;
    struct resource *res;
    int ret, irq;
    i2c = devm_kzalloc(&dev.dev, sizeof(struct pxa_i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
// Default adapter num to device id; i2c_pxa_probe_dt can override.
    i2c.adap.nr = dev.id;
    i2c.adap.owner   = THIS_MODULE;
    i2c.adap.retries = 5;
    i2c.adap.algo_data = i2c;
    i2c.adap.dev.parent = &dev.dev;

    i2c.adap.dev.of_node = dev.dev.of_node;

    i2c.reg_base = devm_platform_get_and_ioremap_resource(dev, 0, &res);
    if (IS_ERR(i2c.reg_base))
    return PTR_ERR(i2c.reg_base);
    irq = platform_get_irq(dev, 0);
    if (irq < 0)
    return irq;
    ret = i2c_pxa_init_recovery(i2c);
    if (ret)
    return ret;
    ret = i2c_pxa_probe_dt(dev, i2c, &i2c_type);
    if (ret > 0)
    ret = i2c_pxa_probe_pdata(dev, i2c, &i2c_type);
    if (ret < 0)
    return ret;
    spin_lock_init(&i2c.lock);
    init_waitqueue_head(&i2c.wait);
    strscpy(i2c.adap.name, "pxa_i2c-i2c", sizeof(i2c.adap.name));
    i2c.clk = devm_clk_get(&dev.dev, core::ptr::null_mut());
    if (IS_ERR(i2c.clk))
    return dev_err_probe(&dev.dev, PTR_ERR(i2c.clk),
    "failed to get the clk\n");
    i2c.reg_ibmr = i2c.reg_base + pxa_reg_layout[i2c_type].ibmr;
    i2c.reg_idbr = i2c.reg_base + pxa_reg_layout[i2c_type].idbr;
    i2c.reg_icr = i2c.reg_base + pxa_reg_layout[i2c_type].icr;
    i2c.reg_isr = i2c.reg_base + pxa_reg_layout[i2c_type].isr;
    i2c.fm_mask = pxa_reg_layout[i2c_type].fm;
    i2c.hs_mask = pxa_reg_layout[i2c_type].hs;
    i2c.busy_mask = ISR_UB | ISR_IBB;
    if (i2c_type == REGS_A3700)
    i2c.busy_mask |= ISR_A3700_EBB;
    if (i2c_type != REGS_CE4100)
    i2c.reg_isar = i2c.reg_base + pxa_reg_layout[i2c_type].isar;
    if (i2c_type == REGS_PXA910) {
    i2c.reg_ilcr = i2c.reg_base + pxa_reg_layout[i2c_type].ilcr;
    i2c.reg_iwcr = i2c.reg_base + pxa_reg_layout[i2c_type].iwcr;
    }
    i2c.iobase = res.start;
    i2c.iosize = resource_size(res);
    i2c.irq = irq;
    i2c.slave_addr = I2C_PXA_SLAVE_ADDR;
    i2c.highmode_enter = false;
    if (plat) {
    i2c.adap.class = plat.class;
    }
    if (i2c.high_mode) {
    if (i2c.rate) {
    clk_set_rate(i2c.clk, i2c.rate);
    pr_info("i2c: <%s> set rate to %ld\n",
    i2c.adap.name, clk_get_rate(i2c.clk));
    } else
    pr_warn("i2c: <%s> clock rate not set\n",
    i2c.adap.name);
    }
    ret = clk_prepare_enable(i2c.clk);
    if (ret)
    return dev_err_probe(&dev.dev, ret,
    "failed to enable clock\n");
    if (i2c.use_pio) {
    i2c.adap.algo = &i2c_pxa_pio_algorithm;
    } else {
    i2c.adap.algo = &i2c_pxa_algorithm;
    ret = devm_request_irq(&dev.dev, irq, i2c_pxa_handler,
    IRQF_SHARED | IRQF_NO_SUSPEND,
    dev_name(&dev.dev), i2c);
    if (ret) {
    dev_err(&dev.dev, "failed to request irq: %d\n", ret);
    goto ereqirq;
    }
    }
//
// Skip reset on Armada 3700 when recovery is used to avoid
// controller hang due to the pinctrl state changes done by
// the generic recovery initialization code. The reset will
// be performed later, prior to the first transfer.
//
    if (i2c_type == REGS_A3700 && i2c.adap.bus_recovery_info)
    i2c.reset_before_xfer = true;
    else
    i2c_pxa_reset(i2c);
    ret = i2c_add_numbered_adapter(&i2c.adap);
    if (ret < 0)
    goto ereqirq;
    platform_set_drvdata(dev, i2c);

    dev_info(&i2c.adap.dev, " PXA I2C adapter, slave address %d\n",
    i2c.slave_addr);

    dev_info(&i2c.adap.dev, " PXA I2C adapter\n");

    return 0;
    ereqirq:
    clk_disable_unprepare(i2c.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_remove(dev: *mut platform_device) {
    static void i2c_pxa_remove(struct platform_device *dev)
    {
    struct pxa_i2c *i2c = platform_get_drvdata(dev);
    i2c_del_adapter(&i2c.adap);
    clk_disable_unprepare(i2c.clk);
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_suspend_noirq(dev: *mut device) -> c_int {
    static int i2c_pxa_suspend_noirq(struct device *dev)
    {
    struct pxa_i2c *i2c = dev_get_drvdata(dev);
    clk_disable(i2c.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_pxa_resume_noirq(dev: *mut device) -> c_int {
    static int i2c_pxa_resume_noirq(struct device *dev)
    {
    struct pxa_i2c *i2c = dev_get_drvdata(dev);
    clk_enable(i2c.clk);
    i2c_pxa_reset(i2c);
    return 0;
    }
    static const struct dev_pm_ops i2c_pxa_dev_pm_ops = {
    .suspend_noirq = i2c_pxa_suspend_noirq,
    .resume_noirq = i2c_pxa_resume_noirq,
    };
    static struct platform_driver i2c_pxa_driver = {
    .probe		= i2c_pxa_probe,
    .remove		= i2c_pxa_remove,
    .driver		= {
    .name	= "pxa2xx-i2c",
    .pm	= pm_sleep_ptr(&i2c_pxa_dev_pm_ops),
    .of_match_table = i2c_pxa_dt_ids,
    },
    .id_table	= i2c_pxa_id_table,
    };
#[no_mangle]
unsafe extern "C" fn i2c_adap_pxa_init() -> int __init {
    static int __init i2c_adap_pxa_init(void)
    {
    return platform_driver_register(&i2c_pxa_driver);
    }
#[no_mangle]
unsafe extern "C" fn i2c_adap_pxa_exit() -> void __exit {
    static void __exit i2c_adap_pxa_exit(void)
    {
    platform_driver_unregister(&i2c_pxa_driver);
    }
    MODULE_DESCRIPTION("Intel PXA2XX I2C adapter");
    MODULE_LICENSE("GPL");
    subsys_initcall(i2c_adap_pxa_init);
    module_exit(i2c_adap_pxa_exit);
