//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-cpm.c
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
// Freescale CPM1/CPM2 I2C interface.
// Copyright (c) 1999 Dan Malek (dmalek@jlc.net).
//
// moved into proper i2c interface;
// Brad Parker (brad@heeltoe.com)
//
// Parts from dbox2_i2c.c (cvs.tuxbox.org)
// (C) 2000-2001 Felix Domke (tmbinc@gmx.net), Gillem (htoa@gmx.net)
//
// (C) 2007 Montavista Software, Inc.
// Vitaly Bordug <vitb@kernel.crashing.org>
//
// Converted to of_platform_device. Renamed to i2c-cpm.c.
// (C) 2007,2008 Jochen Friedrich <jochen@scram.de>
//

// Try to define this if you have an older CPU (earlier than rev D4)
// However, better use a GPIO based bitbang driver in this case :/

pub const CPM_MAX_READ: c_int = 513;
pub const CPM_MAXBD: c_int = 4;

// I2C parameter RAM.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_ram {
    pub /: *mut *mut ushort rbase; / Rx Buffer descriptor base address,
    pub /: *mut *mut ushort tbase; / Tx Buffer descriptor base address,
    pub /: *mut *mut u_char rfcr; / Rx function code,
    pub /: *mut *mut u_char tfcr; / Tx function code,
    pub /: *mut *mut ushort mrblr; / Max receive buffer length,
    pub /: *mut *mut uint rstate; / Internal,
    pub /: *mut *mut uint rdp; / Internal,
    pub /: *mut *mut ushort rbptr; / Rx Buffer descriptor pointer,
    pub /: *mut *mut ushort rbc; / Internal,
    pub /: *mut *mut uint rxtmp; / Internal,
    pub /: *mut *mut uint tstate; / Internal,
    pub /: *mut *mut uint tdp; / Internal,
    pub /: *mut *mut ushort tbptr; / Tx Buffer descriptor pointer,
    pub /: *mut *mut ushort tbc; / Internal,
    pub /: *mut *mut uint txtmp; / Internal,
    pub /: *mut *mut char res1[4]; / Reserved,
    pub /: *mut *mut ushort rpbase; / Relocation pointer,
    pub /: *mut *mut char res2[2]; / Reserved,
// The following elements are only for CPM2
    pub /: *mut *mut char res3[4]; / Reserved,
    pub /: *mut *mut uint sdmatmp; / Internal,
}

pub const I2COM_START: c_uint = 0x80;
pub const I2COM_MASTER: c_uint = 0x01;
pub const I2CER_TXE: c_uint = 0x10;
pub const I2CER_BUSY: c_uint = 0x04;
pub const I2CER_TXB: c_uint = 0x02;
pub const I2CER_RXB: c_uint = 0x01;
pub const I2MOD_EN: c_uint = 0x01;
// I2C Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_reg {
    pub i2mod: u8,
    pub res1: [u8; 3],
    pub i2add: u8,
    pub res2: [u8; 3],
    pub i2brg: u8,
    pub res3: [u8; 3],
    pub i2com: u8,
    pub res4: [u8; 3],
    pub i2cer: u8,
    pub res5: [u8; 3],
    pub i2cmr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_i2c {
    pub base: *mut c_char,
    pub ofdev: *mut platform_device,
    pub adap: i2c_adapter,
    pub dp_addr: c_uint,
    pub /: *mut *mut int version; / CPM1=1, CPM2=2,
    pub irq: c_int,
    pub cp_command: c_int,
    pub freq: c_int,
    pub i2c_reg: *mut i2c_reg __iomem,
    pub i2c_ram: *mut i2c_ram __iomem,
    pub i2c_addr: u16,
    pub i2c_wait: wait_queue_head_t,
    pub tbase: *mut cbd_t __iomem,
    pub rbase: *mut cbd_t __iomem,
    pub txbuf: [*mut u_char; CPM_MAXBD],
    pub rxbuf: [*mut u_char; CPM_MAXBD],
    pub txdma: [dma_addr_t; CPM_MAXBD],
    pub rxdma: [dma_addr_t; CPM_MAXBD],
}

#[no_mangle]
unsafe extern "C" fn cpm_i2c_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cpm_i2c_interrupt(int irq, void *dev_id)
    {
    struct cpm_i2c *cpm;
    struct i2c_reg __iomem *i2c_reg;
    struct i2c_adapter *adap = dev_id;
    int i;
    cpm = i2c_get_adapdata(dev_id);
    i2c_reg = cpm.i2c_reg;
// Clear interrupt.
    i = in_8(&i2c_reg.i2cer);
    out_8(&i2c_reg.i2cer, i);
    dev_dbg(&adap.dev, "Interrupt: %x\n", i);
    wake_up(&cpm.i2c_wait);
    return i ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn cpm_reset_i2c_params(cpm: *mut cpm_i2c) {
    static void cpm_reset_i2c_params(struct cpm_i2c *cpm)
    {
    struct i2c_ram __iomem *i2c_ram = cpm.i2c_ram;
// Set up the I2C parameters in the parameter ram.
    out_be16(&i2c_ram.tbase, (u8 __iomem *)cpm.tbase - DPRAM_BASE);
    out_be16(&i2c_ram.rbase, (u8 __iomem *)cpm.rbase - DPRAM_BASE);
    if (cpm.version == 1) {
    out_8(&i2c_ram.tfcr, I2C_EB);
    out_8(&i2c_ram.rfcr, I2C_EB);
    } else {
    out_8(&i2c_ram.tfcr, I2C_EB_CPM2);
    out_8(&i2c_ram.rfcr, I2C_EB_CPM2);
    }
    out_be16(&i2c_ram.mrblr, CPM_MAX_READ);
    out_be32(&i2c_ram.rstate, 0);
    out_be32(&i2c_ram.rdp, 0);
    out_be16(&i2c_ram.rbptr, 0);
    out_be16(&i2c_ram.rbc, 0);
    out_be32(&i2c_ram.rxtmp, 0);
    out_be32(&i2c_ram.tstate, 0);
    out_be32(&i2c_ram.tdp, 0);
    out_be16(&i2c_ram.tbptr, 0);
    out_be16(&i2c_ram.tbc, 0);
    out_be32(&i2c_ram.txtmp, 0);
    }
#[no_mangle]
unsafe extern "C" fn cpm_i2c_force_close(adap: *mut i2c_adapter) {
    static void cpm_i2c_force_close(struct i2c_adapter *adap)
    {
    struct cpm_i2c *cpm = i2c_get_adapdata(adap);
    struct i2c_reg __iomem *i2c_reg = cpm.i2c_reg;
    dev_dbg(&adap.dev, "cpm_i2c_force_close()\n");
    cpm_command(cpm.cp_command, CPM_CR_CLOSE_RX_BD);
    out_8(&i2c_reg.i2cmr, 0x00);	/* Disable all interrupts */
    out_8(&i2c_reg.i2cer, 0xff);
    }
    static void cpm_i2c_parse_message(struct i2c_adapter *adap,
    struct i2c_msg *pmsg, int num, int tx, int rx)
    {
    cbd_t __iomem *tbdf;
    cbd_t __iomem *rbdf;
    u_char addr;
    u_char *tb;
    u_char *rb;
    struct cpm_i2c *cpm = i2c_get_adapdata(adap);
    tbdf = cpm.tbase + tx;
    rbdf = cpm.rbase + rx;
    addr = i2c_8bit_addr_from_msg(pmsg);
    tb = cpm.txbuf[tx];
    rb = cpm.rxbuf[rx];
// Align read buffer
    rb = (u_char *) (((ulong) rb + 1) & ~1);
    tb[0] = addr;		/* Device address byte w/rw flag */
    out_be16(&tbdf.cbd_datlen, pmsg.len + 1);
    out_be16(&tbdf.cbd_sc, 0);
    if (!(pmsg.flags & I2C_M_NOSTART))
    setbits16(&tbdf.cbd_sc, BD_I2C_START);
    if (tx + 1 == num)
    setbits16(&tbdf.cbd_sc, BD_SC_LAST | BD_SC_WRAP);
    if (pmsg.flags & I2C_M_RD) {
//
// To read, we need an empty buffer of the proper length.
// All that is used is the first byte for address, the remainder
// is just used for timing (and doesn't really have to exist).
//
    dev_dbg(&adap.dev, "cpm_i2c_read(abyte=0x%x)\n", addr);
    out_be16(&rbdf.cbd_datlen, 0);
    out_be16(&rbdf.cbd_sc, BD_SC_EMPTY | BD_SC_INTRPT);
    if (rx + 1 == CPM_MAXBD)
    setbits16(&rbdf.cbd_sc, BD_SC_WRAP);
    eieio();
    setbits16(&tbdf.cbd_sc, BD_SC_READY);
    } else {
    dev_dbg(&adap.dev, "cpm_i2c_write(abyte=0x%x)\n", addr);
    memcpy(tb+1, pmsg.buf, pmsg.len);
    eieio();
    setbits16(&tbdf.cbd_sc, BD_SC_READY | BD_SC_INTRPT);
    }
    }
    static int cpm_i2c_check_message(struct i2c_adapter *adap,
    struct i2c_msg *pmsg, int tx, int rx)
    {
    cbd_t __iomem *tbdf;
    cbd_t __iomem *rbdf;
    u_char *tb;
    u_char *rb;
    struct cpm_i2c *cpm = i2c_get_adapdata(adap);
    tbdf = cpm.tbase + tx;
    rbdf = cpm.rbase + rx;
    tb = cpm.txbuf[tx];
    rb = cpm.rxbuf[rx];
// Align read buffer
    rb = (u_char *) (((uint) rb + 1) & ~1);
    eieio();
    if (pmsg.flags & I2C_M_RD) {
    dev_dbg(&adap.dev, "tx sc 0x%04x, rx sc 0x%04x\n",
    in_be16(&tbdf.cbd_sc), in_be16(&rbdf.cbd_sc));
    if (in_be16(&tbdf.cbd_sc) & BD_SC_NAK) {
    dev_dbg(&adap.dev, "I2C read; No ack\n");
    return -ENXIO;
    }
    if (in_be16(&rbdf.cbd_sc) & BD_SC_EMPTY) {
    dev_err(&adap.dev,
    "I2C read; complete but rbuf empty\n");
    return -EREMOTEIO;
    }
    if (in_be16(&rbdf.cbd_sc) & BD_SC_OV) {
    dev_err(&adap.dev, "I2C read; Overrun\n");
    return -EREMOTEIO;
    }
    memcpy(pmsg.buf, rb, pmsg.len);
    } else {
    dev_dbg(&adap.dev, "tx sc %d 0x%04x\n", tx,
    in_be16(&tbdf.cbd_sc));
    if (in_be16(&tbdf.cbd_sc) & BD_SC_NAK) {
    dev_dbg(&adap.dev, "I2C write; No ack\n");
    return -ENXIO;
    }
    if (in_be16(&tbdf.cbd_sc) & BD_SC_UN) {
    dev_err(&adap.dev, "I2C write; Underrun\n");
    return -EIO;
    }
    if (in_be16(&tbdf.cbd_sc) & BD_SC_CL) {
    dev_err(&adap.dev, "I2C write; Collision\n");
    return -EIO;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpm_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int cpm_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct cpm_i2c *cpm = i2c_get_adapdata(adap);
    struct i2c_reg __iomem *i2c_reg = cpm.i2c_reg;
    struct i2c_ram __iomem *i2c_ram = cpm.i2c_ram;
    struct i2c_msg *pmsg;
    int ret;
    int tptr;
    int rptr;
    cbd_t __iomem *tbdf;
    cbd_t __iomem *rbdf;
// Reset to use first buffer
    out_be16(&i2c_ram.rbptr, in_be16(&i2c_ram.rbase));
    out_be16(&i2c_ram.tbptr, in_be16(&i2c_ram.tbase));
    tbdf = cpm.tbase;
    rbdf = cpm.rbase;
    tptr = 0;
    rptr = 0;
//
// If there was a collision in the last i2c transaction,
// Set I2COM_MASTER as it was cleared during collision.
//
    if (in_be16(&tbdf.cbd_sc) & BD_SC_CL) {
    out_8(&cpm.i2c_reg.i2com, I2COM_MASTER);
    }
    while (tptr < num) {
    pmsg = &msgs[tptr];
    dev_dbg(&adap.dev, "R: %d T: %d\n", rptr, tptr);
    cpm_i2c_parse_message(adap, pmsg, num, tptr, rptr);
    if (pmsg.flags & I2C_M_RD)
    rptr++;
    tptr++;
    }
// Start transfer now
// Enable RX/TX/Error interupts
    out_8(&i2c_reg.i2cmr, I2CER_TXE | I2CER_TXB | I2CER_RXB);
    out_8(&i2c_reg.i2cer, 0xff);	/* Clear interrupt status */
// Chip bug, set enable here
    setbits8(&i2c_reg.i2mod, I2MOD_EN);	/* Enable */
// Begin transmission
    setbits8(&i2c_reg.i2com, I2COM_START);
    tptr = 0;
    rptr = 0;
    while (tptr < num) {
// Check for outstanding messages
    dev_dbg(&adap.dev, "test ready.\n");
    pmsg = &msgs[tptr];
    if (pmsg.flags & I2C_M_RD)
    ret = wait_event_timeout(cpm.i2c_wait,
    (in_be16(&tbdf[tptr].cbd_sc) & BD_SC_NAK) ||
    !(in_be16(&rbdf[rptr].cbd_sc) & BD_SC_EMPTY),
    1 * HZ);
    else
    ret = wait_event_timeout(cpm.i2c_wait,
    !(in_be16(&tbdf[tptr].cbd_sc) & BD_SC_READY),
    1 * HZ);
    if (ret == 0) {
    ret = -EREMOTEIO;
    dev_err(&adap.dev, "I2C transfer: timeout\n");
    goto out_err;
    }
    if (ret > 0) {
    dev_dbg(&adap.dev, "ready.\n");
    ret = cpm_i2c_check_message(adap, pmsg, tptr, rptr);
    tptr++;
    if (pmsg.flags & I2C_M_RD)
    rptr++;
    if (ret)
    goto out_err;
    }
    }

//
// Chip errata, clear enable. This is not needed on rev D4 CPUs.
// Disabling I2C too early may cause too short stop condition
//
    udelay(4);
    clrbits8(&i2c_reg.i2mod, I2MOD_EN);

    return (num);
    out_err:
    cpm_i2c_force_close(adap);

//
// Chip errata, clear enable. This is not needed on rev D4 CPUs.
//
    clrbits8(&i2c_reg.i2mod, I2MOD_EN);

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpm_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 cpm_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | (I2C_FUNC_SMBUS_EMUL & ~I2C_FUNC_SMBUS_QUICK);
    }
// -----exported algorithm data: -------------------------------------
    static const struct i2c_algorithm cpm_i2c_algo = {
    .xfer = cpm_i2c_xfer,
    .functionality = cpm_i2c_func,
    };
// CPM_MAX_READ is also limiting writes according to the code!
    static const struct i2c_adapter_quirks cpm_i2c_quirks = {
    .max_num_msgs = CPM_MAXBD,
    .max_read_len = CPM_MAX_READ,
    .max_write_len = CPM_MAX_READ,
    };
    static const struct i2c_adapter cpm_ops = {
    .owner		= THIS_MODULE,
    .name		= "i2c-cpm",
    .algo		= &cpm_i2c_algo,
    .quirks		= &cpm_i2c_quirks,
    };
#[no_mangle]
unsafe extern "C" fn cpm_i2c_setup(cpm: *mut cpm_i2c) -> c_int {
    static int cpm_i2c_setup(struct cpm_i2c *cpm)
    {
    struct platform_device *ofdev = cpm.ofdev;
    const u32 *data;
    int len, ret, i;
    void __iomem *i2c_base;
    cbd_t __iomem *tbdf;
    cbd_t __iomem *rbdf;
    unsigned char brg;
    dev_dbg(&cpm.ofdev.dev, "cpm_i2c_setup()\n");
    init_waitqueue_head(&cpm.i2c_wait);
    cpm.irq = irq_of_parse_and_map(ofdev.dev.of_node, 0);
    if (!cpm.irq)
    return -EINVAL;
// Install interrupt handler.
    ret = request_irq(cpm.irq, cpm_i2c_interrupt, 0, "cpm_i2c",
    &cpm.adap);
    if (ret)
    return ret;
// I2C parameter RAM
    i2c_base = of_iomap(ofdev.dev.of_node, 1);
    if (i2c_base == core::ptr::null_mut()) {
    ret = -EINVAL;
    goto out_irq;
    }
    if (of_device_is_compatible(ofdev.dev.of_node, "fsl,cpm1-i2c")) {
// Check for and use a microcode relocation patch.
    cpm.i2c_ram = i2c_base;
    cpm.i2c_addr = in_be16(&cpm.i2c_ram.rpbase);
//
// Maybe should use cpm_muram_alloc instead of hardcoding
// this in micropatch.c
//
    if (cpm.i2c_addr) {
    cpm.i2c_ram = cpm_muram_addr(cpm.i2c_addr);
    iounmap(i2c_base);
    }
    cpm.version = 1;
    } else if (of_device_is_compatible(ofdev.dev.of_node, "fsl,cpm2-i2c")) {
    cpm.i2c_addr = cpm_muram_alloc(sizeof(struct i2c_ram), 64);
    cpm.i2c_ram = cpm_muram_addr(cpm.i2c_addr);
    out_be16(i2c_base, cpm.i2c_addr);
    iounmap(i2c_base);
    cpm.version = 2;
    } else {
    iounmap(i2c_base);
    ret = -EINVAL;
    goto out_irq;
    }
// I2C control/status registers
    cpm.i2c_reg = of_iomap(ofdev.dev.of_node, 0);
    if (cpm.i2c_reg == core::ptr::null_mut()) {
    ret = -EINVAL;
    goto out_ram;
    }
    data = of_get_property(ofdev.dev.of_node, "fsl,cpm-command", &len);
    if (!data || len != 4) {
    ret = -EINVAL;
    goto out_reg;
    }
    cpm.cp_command = *data;
    data = of_get_property(ofdev.dev.of_node, "linux,i2c-class", &len);
    if (data && len == 4)
    cpm.adap.class = *data;
    data = of_get_property(ofdev.dev.of_node, "clock-frequency", &len);
    if (data && len == 4)
    cpm.freq = *data;
    else
    cpm.freq = 60000; /* use 60kHz i2c clock by default */
//
// Allocate space for CPM_MAXBD transmit and receive buffer
// descriptors in the DP ram.
//
    cpm.dp_addr = cpm_muram_alloc(sizeof(cbd_t) * 2 * CPM_MAXBD, 8);
    if (!cpm.dp_addr) {
    ret = -ENOMEM;
    goto out_reg;
    }
    cpm.tbase = cpm_muram_addr(cpm.dp_addr);
    cpm.rbase = cpm_muram_addr(cpm.dp_addr + sizeof(cbd_t) * CPM_MAXBD);
// Allocate TX and RX buffers
    tbdf = cpm.tbase;
    rbdf = cpm.rbase;
    for (i = 0; i < CPM_MAXBD; i++) {
    cpm.rxbuf[i] = dma_alloc_coherent(&cpm.ofdev.dev,
    CPM_MAX_READ + 1,
    &cpm.rxdma[i], GFP_KERNEL);
    if (!cpm.rxbuf[i]) {
    ret = -ENOMEM;
    goto out_muram;
    }
    out_be32(&rbdf[i].cbd_bufaddr, ((cpm.rxdma[i] + 1) & ~1));
    cpm.txbuf[i] = dma_alloc_coherent(&cpm.ofdev.dev,
    CPM_MAX_READ + 1,
    &cpm.txdma[i], GFP_KERNEL);
    if (!cpm.txbuf[i]) {
    ret = -ENOMEM;
    goto out_muram;
    }
    out_be32(&tbdf[i].cbd_bufaddr, cpm.txdma[i]);
    }
// Initialize Tx/Rx parameters.
    cpm_reset_i2c_params(cpm);
    dev_dbg(&cpm.ofdev.dev, "i2c_ram 0x%p, i2c_addr 0x%04x, freq %d\n",
    cpm.i2c_ram, cpm.i2c_addr, cpm.freq);
    dev_dbg(&cpm.ofdev.dev, "tbase 0x%04x, rbase 0x%04x\n",
    (u8 __iomem *)cpm.tbase - DPRAM_BASE,
    (u8 __iomem *)cpm.rbase - DPRAM_BASE);
    cpm_command(cpm.cp_command, CPM_CR_INIT_TRX);
//
// Select an invalid address. Just make sure we don't use loopback mode
//
    out_8(&cpm.i2c_reg.i2add, 0x7f << 1);
//
// PDIV is set to 00 in i2mod, so brgclk/32 is used as input to the
// i2c baud rate generator. This is divided by 2 x (DIV + 3) to get
// the actual i2c bus frequency.
//
    brg = get_brgfreq() / (32 * 2 * cpm.freq) - 3;
    out_8(&cpm.i2c_reg.i2brg, brg);
    out_8(&cpm.i2c_reg.i2mod, 0x00);
    out_8(&cpm.i2c_reg.i2com, I2COM_MASTER);
// Disable interrupts.
    out_8(&cpm.i2c_reg.i2cmr, 0);
    out_8(&cpm.i2c_reg.i2cer, 0xff);
    return 0;
    out_muram:
    for (i = 0; i < CPM_MAXBD; i++) {
    if (cpm.rxbuf[i])
    dma_free_coherent(&cpm.ofdev.dev, CPM_MAX_READ + 1,
    cpm.rxbuf[i], cpm.rxdma[i]);
    if (cpm.txbuf[i])
    dma_free_coherent(&cpm.ofdev.dev, CPM_MAX_READ + 1,
    cpm.txbuf[i], cpm.txdma[i]);
    }
    cpm_muram_free(cpm.dp_addr);
    out_reg:
    iounmap(cpm.i2c_reg);
    out_ram:
    if ((cpm.version == 1) && (!cpm.i2c_addr))
    iounmap(cpm.i2c_ram);
    if (cpm.version == 2)
    cpm_muram_free(cpm.i2c_addr);
    out_irq:
    free_irq(cpm.irq, &cpm.adap);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpm_i2c_shutdown(cpm: *mut cpm_i2c) {
    static void cpm_i2c_shutdown(struct cpm_i2c *cpm)
    {
    int i;
// Shut down I2C.
    clrbits8(&cpm.i2c_reg.i2mod, I2MOD_EN);
// Disable interrupts
    out_8(&cpm.i2c_reg.i2cmr, 0);
    out_8(&cpm.i2c_reg.i2cer, 0xff);
    free_irq(cpm.irq, &cpm.adap);
// Free all memory
    for (i = 0; i < CPM_MAXBD; i++) {
    dma_free_coherent(&cpm.ofdev.dev, CPM_MAX_READ + 1,
    cpm.rxbuf[i], cpm.rxdma[i]);
    dma_free_coherent(&cpm.ofdev.dev, CPM_MAX_READ + 1,
    cpm.txbuf[i], cpm.txdma[i]);
    }
    cpm_muram_free(cpm.dp_addr);
    iounmap(cpm.i2c_reg);
    if ((cpm.version == 1) && (!cpm.i2c_addr))
    iounmap(cpm.i2c_ram);
    if (cpm.version == 2)
    cpm_muram_free(cpm.i2c_addr);
    }
#[no_mangle]
unsafe extern "C" fn cpm_i2c_probe(ofdev: *mut platform_device) -> c_int {
    static int cpm_i2c_probe(struct platform_device *ofdev)
    {
    int result, len;
    struct cpm_i2c *cpm;
    const u32 *data;
    cpm = kzalloc_obj(struct cpm_i2c);
    if (!cpm)
    return -ENOMEM;
    cpm.ofdev = ofdev;
    platform_set_drvdata(ofdev, cpm);
    cpm.adap = cpm_ops;
    i2c_set_adapdata(&cpm.adap, cpm);
    cpm.adap.dev.parent = &ofdev.dev;
    cpm.adap.dev.of_node = of_node_get(ofdev.dev.of_node);
    result = cpm_i2c_setup(cpm);
    if (result) {
    dev_err(&ofdev.dev, "Unable to init hardware\n");
    goto out_free;
    }
// register new adapter to i2c module...
    data = of_get_property(ofdev.dev.of_node, "linux,i2c-index", &len);
    cpm.adap.nr = (data && len == 4) ? *data : -1;
    result = i2c_add_numbered_adapter(&cpm.adap);
    if (result < 0)
    goto out_shut;
    dev_dbg(&ofdev.dev, "hw routines for %s registered.\n",
    cpm.adap.name);
    return 0;
    out_shut:
    cpm_i2c_shutdown(cpm);
    out_free:
    kfree(cpm);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn cpm_i2c_remove(ofdev: *mut platform_device) {
    static void cpm_i2c_remove(struct platform_device *ofdev)
    {
    struct cpm_i2c *cpm = platform_get_drvdata(ofdev);
    i2c_del_adapter(&cpm.adap);
    cpm_i2c_shutdown(cpm);
    kfree(cpm);
    }
    static const struct of_device_id cpm_i2c_match[] = {
    {
    .compatible = "fsl,cpm1-i2c",
    },
    {
    .compatible = "fsl,cpm2-i2c",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpm_i2c_match);
    static struct platform_driver cpm_i2c_driver = {
    .probe		= cpm_i2c_probe,
    .remove		= cpm_i2c_remove,
    .driver = {
    .name = "fsl-i2c-cpm",
    .of_match_table = cpm_i2c_match,
    },
    };
    module_platform_driver(cpm_i2c_driver);
    MODULE_AUTHOR("Jochen Friedrich <jochen@scram.de>");
    MODULE_DESCRIPTION("I2C-Bus adapter routines for CPM boards");
    MODULE_LICENSE("GPL");
