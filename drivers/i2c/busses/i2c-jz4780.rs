//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-jz4780.c
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
// Ingenic JZ4780 I2C bus driver
//
// Copyright (C) 2006 - 2009 Ingenic Semiconductor Inc.
// Copyright (C) 2015 Imagination Technologies
// Copyright (C) 2019 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

pub const JZ4780_I2C_CTRL: c_uint = 0x00;
pub const JZ4780_I2C_TAR: c_uint = 0x04;
pub const JZ4780_I2C_SAR: c_uint = 0x08;
pub const JZ4780_I2C_DC: c_uint = 0x10;
pub const JZ4780_I2C_SHCNT: c_uint = 0x14;
pub const JZ4780_I2C_SLCNT: c_uint = 0x18;
pub const JZ4780_I2C_FHCNT: c_uint = 0x1C;
pub const JZ4780_I2C_FLCNT: c_uint = 0x20;
pub const JZ4780_I2C_INTST: c_uint = 0x2C;
pub const JZ4780_I2C_INTM: c_uint = 0x30;
pub const JZ4780_I2C_RXTL: c_uint = 0x38;
pub const JZ4780_I2C_TXTL: c_uint = 0x3C;
pub const JZ4780_I2C_CINTR: c_uint = 0x40;
pub const JZ4780_I2C_CRXUF: c_uint = 0x44;
pub const JZ4780_I2C_CRXOF: c_uint = 0x48;
pub const JZ4780_I2C_CTXOF: c_uint = 0x4C;
pub const JZ4780_I2C_CRXREQ: c_uint = 0x50;
pub const JZ4780_I2C_CTXABRT: c_uint = 0x54;
pub const JZ4780_I2C_CRXDONE: c_uint = 0x58;
pub const JZ4780_I2C_CACT: c_uint = 0x5C;
pub const JZ4780_I2C_CSTP: c_uint = 0x60;
pub const JZ4780_I2C_CSTT: c_uint = 0x64;
pub const JZ4780_I2C_CGC: c_uint = 0x68;
pub const JZ4780_I2C_ENB: c_uint = 0x6C;
pub const JZ4780_I2C_STA: c_uint = 0x70;
pub const JZ4780_I2C_TXABRT: c_uint = 0x80;
pub const JZ4780_I2C_DMACR: c_uint = 0x88;
pub const JZ4780_I2C_DMATDLR: c_uint = 0x8C;
pub const JZ4780_I2C_DMARDLR: c_uint = 0x90;
pub const JZ4780_I2C_SDASU: c_uint = 0x94;
pub const JZ4780_I2C_ACKGC: c_uint = 0x98;
pub const JZ4780_I2C_ENSTA: c_uint = 0x9C;
pub const JZ4780_I2C_SDAHD: c_uint = 0xD0;
pub const X1000_I2C_SDAHD: c_uint = 0x7C;

pub const JZ4780_I2C_FIFO_LEN: c_int = 16;
pub const X1000_I2C_FIFO_LEN: c_int = 64;
pub const JZ4780_I2C_TIMEOUT: c_int = 300;
pub const BUFSIZE: c_int = 200;
    enum ingenic_i2c_version {
    ID_JZ4780,
    ID_X1000,
    };
// ingenic_i2c_config: SoC specific config data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_i2c_config {
    pub version: enum ingenic_i2c_version,
    pub fifosize: c_int,
    pub tx_level: c_int,
    pub rx_level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz4780_i2c {
    pub iomem: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
    pub clk_rate_khz: c_ulong,
    pub adap: i2c_adapter,
    pub cdata: *const ingenic_i2c_config,
// lock to protect rbuf and wbuf between xfer_rd/wr and irq handler
    pub lock: spinlock_t,
// beginning of lock scope
    pub rbuf: *mut c_uchar,
    pub rd_total_len: c_int,
    pub rd_data_xfered: c_int,
    pub rd_cmd_xfered: c_int,
    pub wbuf: *mut c_uchar,
    pub wt_len: c_int,
    pub is_write: c_int,
    pub stop_hold: c_int,
    pub speed: c_int,
    pub data_buf: [c_int; BUFSIZE],
    pub cmd_buf: [c_int; BUFSIZE],
    pub cmd: c_int,
// end of lock scope
    pub trans_waitq: completion,
}

    static inline unsigned short jz4780_i2c_readw(struct jz4780_i2c *i2c,
    unsigned long offset)
    {
    return readw(i2c.iomem + offset);
    }
    static inline void jz4780_i2c_writew(struct jz4780_i2c *i2c,
    unsigned long offset, unsigned short val)
    {
    writew(val, i2c.iomem + offset);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_disable(i2c: *mut jz4780_i2c) -> c_int {
    static int jz4780_i2c_disable(struct jz4780_i2c *i2c)
    {
    unsigned short regval;
    let mut loops: c_ulong = 5;
    jz4780_i2c_writew(i2c, JZ4780_I2C_ENB, 0);
    do {
    regval = jz4780_i2c_readw(i2c, JZ4780_I2C_ENSTA);
    if (!(regval & JZ4780_I2C_ENB_I2C))
    return 0;
    usleep_range(5000, 15000);
    } while (--loops);
    dev_err(&i2c.adap.dev, "disable failed: ENSTA=0x%04x\n", regval);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_enable(i2c: *mut jz4780_i2c) -> c_int {
    static int jz4780_i2c_enable(struct jz4780_i2c *i2c)
    {
    unsigned short regval;
    let mut loops: c_ulong = 5;
    jz4780_i2c_writew(i2c, JZ4780_I2C_ENB, 1);
    do {
    regval = jz4780_i2c_readw(i2c, JZ4780_I2C_ENSTA);
    if (regval & JZ4780_I2C_ENB_I2C)
    return 0;
    usleep_range(5000, 15000);
    } while (--loops);
    dev_err(&i2c.adap.dev, "enable failed: ENSTA=0x%04x\n", regval);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_set_target(i2c: *mut jz4780_i2c, address: c_uchar) -> c_int {
    static int jz4780_i2c_set_target(struct jz4780_i2c *i2c, unsigned char address)
    {
    unsigned short regval;
    let mut loops: c_ulong = 5;
    do {
    regval = jz4780_i2c_readw(i2c, JZ4780_I2C_STA);
    if ((regval & JZ4780_I2C_STA_TFE) &&
    !(regval & JZ4780_I2C_STA_MSTACT))
    break;
    usleep_range(5000, 15000);
    } while (--loops);
    if (loops) {
    jz4780_i2c_writew(i2c, JZ4780_I2C_TAR, address);
    return 0;
    }
    dev_err(&i2c.adap.dev,
    "set device to address 0x%02x failed, STA=0x%04x\n",
    address, regval);
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_set_speed(i2c: *mut jz4780_i2c) -> c_int {
    static int jz4780_i2c_set_speed(struct jz4780_i2c *i2c)
    {
    let mut dev_clk_khz: c_int = i2c.clk_rate_khz;
    int cnt_high = 0;	/* HIGH period count of the SCL clock */
    int cnt_low = 0;	/* LOW period count of the SCL clock */
    int cnt_period = 0;	/* period count of the SCL clock */
    let mut setup_time: c_int = 0;
    let mut hold_time: c_int = 0;
    let mut tmp: c_ushort = 0;
    let mut i2c_clk: c_int = i2c.speed;
    if (jz4780_i2c_disable(i2c))
    dev_dbg(&i2c.adap.dev, "i2c not disabled\n");
//
// 1 JZ4780_I2C cycle equals to cnt_period PCLK(i2c_clk)
// standard mode, min LOW and HIGH period are 4700 ns and 4000 ns
// fast mode, min LOW and HIGH period are 1300 ns and 600 ns
//
    cnt_period = dev_clk_khz / i2c_clk;
    if (i2c_clk <= 100)
    cnt_high = (cnt_period * 4000) / (4700 + 4000);
    else
    cnt_high = (cnt_period * 600) / (1300 + 600);
    cnt_low = cnt_period - cnt_high;
//
// NOTE: JZ4780_I2C_CTRL_REST can't set when i2c enabled, because
// normal read are 2 messages, we cannot disable i2c controller
// between these two messages, this means that we must always set
// JZ4780_I2C_CTRL_REST when init JZ4780_I2C_CTRL
//
    if (i2c_clk <= 100) {
    tmp = JZ4780_I2C_CTRL_SPDS | JZ4780_I2C_CTRL_REST
    | JZ4780_I2C_CTRL_SLVDIS | JZ4780_I2C_CTRL_MD;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    jz4780_i2c_writew(i2c, JZ4780_I2C_SHCNT,
    JZ4780_I2CSHCNT_ADJUST(cnt_high));
    jz4780_i2c_writew(i2c, JZ4780_I2C_SLCNT,
    JZ4780_I2CSLCNT_ADJUST(cnt_low));
    } else {
    tmp = JZ4780_I2C_CTRL_SPDF | JZ4780_I2C_CTRL_REST
    | JZ4780_I2C_CTRL_SLVDIS | JZ4780_I2C_CTRL_MD;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    jz4780_i2c_writew(i2c, JZ4780_I2C_FHCNT,
    JZ4780_I2CFHCNT_ADJUST(cnt_high));
    jz4780_i2c_writew(i2c, JZ4780_I2C_FLCNT,
    JZ4780_I2CFLCNT_ADJUST(cnt_low));
    }
//
// a i2c device must internally provide a hold time at least 300ns
// tHD:DAT
// Standard Mode: min=300ns, max=3450ns
// Fast Mode: min=0ns, max=900ns
// tSU:DAT
// Standard Mode: min=250ns, max=infinite
// Fast Mode: min=100(250ns is recommended), max=infinite
//
// 1i2c_clk = 10^6 / dev_clk_khz
// on FPGA, dev_clk_khz = 12000, so 1i2c_clk = 1000/12 = 83ns
// on Pisces(1008M), dev_clk_khz=126000, so 1i2c_clk = 1000 / 126 = 8ns
//
// The actual hold time is (SDAHD + 1) * (i2c_clk period).
//
// Length of setup time calculated using (SDASU - 1) * (ic_clk_period)
//
    if (i2c_clk <= 100) { /* standard mode */
    setup_time = 300;
    hold_time = 400;
    } else {
    setup_time = 450;
    hold_time = 450;
    }
    hold_time = ((hold_time * dev_clk_khz) / 1000000) - 1;
    setup_time = ((setup_time * dev_clk_khz) / 1000000)  + 1;
    if (setup_time > 255)
    setup_time = 255;
    if (setup_time <= 0)
    setup_time = 1;
    jz4780_i2c_writew(i2c, JZ4780_I2C_SDASU, setup_time);
    if (hold_time > 255)
    hold_time = 255;
    if (hold_time >= 0) {
// i2c hold time enable
    if (i2c.cdata.version >= ID_X1000) {
    jz4780_i2c_writew(i2c, X1000_I2C_SDAHD, hold_time);
    } else {
    hold_time |= JZ4780_I2C_SDAHD_HDENB;
    jz4780_i2c_writew(i2c, JZ4780_I2C_SDAHD, hold_time);
    }
    } else {
// disable hold time
    if (i2c.cdata.version >= ID_X1000)
    jz4780_i2c_writew(i2c, X1000_I2C_SDAHD, 0);
    else
    jz4780_i2c_writew(i2c, JZ4780_I2C_SDAHD, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_cleanup(i2c: *mut jz4780_i2c) -> c_int {
    static int jz4780_i2c_cleanup(struct jz4780_i2c *i2c)
    {
    int ret;
    unsigned long flags;
    unsigned short tmp;
    spin_lock_irqsave(&i2c.lock, flags);
// can send stop now if need
    if (i2c.cdata.version < ID_X1000) {
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_CTRL);
    tmp &= ~JZ4780_I2C_CTRL_STPHLD;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    }
// disable all interrupts first
    jz4780_i2c_writew(i2c, JZ4780_I2C_INTM, 0);
// then clear all interrupts
    jz4780_i2c_readw(i2c, JZ4780_I2C_CTXABRT);
    jz4780_i2c_readw(i2c, JZ4780_I2C_CINTR);
// then disable the controller
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_CTRL);
    tmp &= ~JZ4780_I2C_ENB_I2C;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    udelay(10);
    tmp |= JZ4780_I2C_ENB_I2C;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    spin_unlock_irqrestore(&i2c.lock, flags);
    ret = jz4780_i2c_disable(i2c);
    if (ret)
    dev_err(&i2c.adap.dev,
    "unable to disable device during cleanup!\n");
    if (unlikely(jz4780_i2c_readw(i2c, JZ4780_I2C_INTM)
    & jz4780_i2c_readw(i2c, JZ4780_I2C_INTST)))
    dev_err(&i2c.adap.dev,
    "device has interrupts after a complete cleanup!\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_prepare(i2c: *mut jz4780_i2c) -> c_int {
    static int jz4780_i2c_prepare(struct jz4780_i2c *i2c)
    {
    jz4780_i2c_set_speed(i2c);
    return jz4780_i2c_enable(i2c);
    }
    static void jz4780_i2c_send_rcmd(struct jz4780_i2c *i2c,
    int cmd_count,
    int cmd_left)
    {
    int i;
    for (i = 0; i < cmd_count - 1; i++)
    jz4780_i2c_writew(i2c, JZ4780_I2C_DC, JZ4780_I2C_DC_READ);
    if ((cmd_left == 0) && (i2c.cdata.version >= ID_X1000))
    jz4780_i2c_writew(i2c, JZ4780_I2C_DC,
    JZ4780_I2C_DC_READ | X1000_I2C_DC_STOP);
    else
    jz4780_i2c_writew(i2c, JZ4780_I2C_DC, JZ4780_I2C_DC_READ);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_trans_done(i2c: *mut jz4780_i2c) {
    static void jz4780_i2c_trans_done(struct jz4780_i2c *i2c)
    {
    jz4780_i2c_writew(i2c, JZ4780_I2C_INTM, 0);
    complete(&i2c.trans_waitq);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_irq(irqno: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t jz4780_i2c_irq(int irqno, void *dev_id)
    {
    unsigned short tmp;
    unsigned short intst;
    unsigned short intmsk;
    struct jz4780_i2c *i2c = dev_id;
    spin_lock(&i2c.lock);
    intmsk = jz4780_i2c_readw(i2c, JZ4780_I2C_INTM);
    intst = jz4780_i2c_readw(i2c, JZ4780_I2C_INTST);
    intst &= intmsk;
    if (intst & JZ4780_I2C_INTST_TXABT) {
    jz4780_i2c_trans_done(i2c);
    goto done;
    }
    if (intst & JZ4780_I2C_INTST_RXOF) {
    dev_dbg(&i2c.adap.dev, "received fifo overflow!\n");
    jz4780_i2c_trans_done(i2c);
    goto done;
    }
//
// When reading, always drain RX FIFO before we send more Read
// Commands to avoid fifo overrun
//
    if (i2c.is_write == 0) {
    int rd_left;
    while ((jz4780_i2c_readw(i2c, JZ4780_I2C_STA)
    & JZ4780_I2C_STA_RFNE)) {
// (i2c->rbuf++) = jz4780_i2c_readw(i2c, JZ4780_I2C_DC)
    & 0xff;
    i2c.rd_data_xfered++;
    if (i2c.rd_data_xfered == i2c.rd_total_len) {
    jz4780_i2c_trans_done(i2c);
    goto done;
    }
    }
    rd_left = i2c.rd_total_len - i2c.rd_data_xfered;
    if (rd_left <= i2c.cdata.fifosize)
    jz4780_i2c_writew(i2c, JZ4780_I2C_RXTL, rd_left - 1);
    }
    if (intst & JZ4780_I2C_INTST_TXEMP) {
    if (i2c.is_write == 0) {
    let mut cmd_left: c_int = i2c.rd_total_len - i2c.rd_cmd_xfered;
    int max_send = (i2c.cdata.fifosize - 1)
    - (i2c.rd_cmd_xfered
    - i2c.rd_data_xfered);
    let mut cmd_to_send: c_int = min(cmd_left, max_send);
    if (i2c.rd_cmd_xfered != 0)
    cmd_to_send = min(cmd_to_send,
    i2c.cdata.fifosize
    - i2c.cdata.tx_level - 1);
    if (cmd_to_send) {
    i2c.rd_cmd_xfered += cmd_to_send;
    cmd_left = i2c.rd_total_len -
    i2c.rd_cmd_xfered;
    jz4780_i2c_send_rcmd(i2c,
    cmd_to_send, cmd_left);
    }
    if (cmd_left == 0) {
    intmsk = jz4780_i2c_readw(i2c, JZ4780_I2C_INTM);
    intmsk &= ~JZ4780_I2C_INTM_MTXEMP;
    jz4780_i2c_writew(i2c, JZ4780_I2C_INTM, intmsk);
    if (i2c.cdata.version < ID_X1000) {
    tmp = jz4780_i2c_readw(i2c,
    JZ4780_I2C_CTRL);
    tmp &= ~JZ4780_I2C_CTRL_STPHLD;
    jz4780_i2c_writew(i2c,
    JZ4780_I2C_CTRL, tmp);
    }
    }
    } else {
    unsigned short data;
    unsigned short i2c_sta;
    i2c_sta = jz4780_i2c_readw(i2c, JZ4780_I2C_STA);
    while ((i2c_sta & JZ4780_I2C_STA_TFNF) &&
    (i2c.wt_len > 0)) {
    i2c_sta = jz4780_i2c_readw(i2c, JZ4780_I2C_STA);
    data = *i2c.wbuf;
    data &= ~JZ4780_I2C_DC_READ;
    if ((i2c.wt_len == 1) && (!i2c.stop_hold) &&
    (i2c.cdata.version >= ID_X1000))
    data |= X1000_I2C_DC_STOP;
    jz4780_i2c_writew(i2c, JZ4780_I2C_DC, data);
    i2c.wbuf++;
    i2c.wt_len--;
    }
    if (i2c.wt_len == 0) {
    if ((!i2c.stop_hold) && (i2c.cdata.version <
    ID_X1000)) {
    tmp = jz4780_i2c_readw(i2c,
    JZ4780_I2C_CTRL);
    tmp &= ~JZ4780_I2C_CTRL_STPHLD;
    jz4780_i2c_writew(i2c,
    JZ4780_I2C_CTRL, tmp);
    }
    jz4780_i2c_trans_done(i2c);
    goto done;
    }
    }
    }
    done:
    spin_unlock(&i2c.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_txabrt(i2c: *mut jz4780_i2c, src: c_int) {
    static void jz4780_i2c_txabrt(struct jz4780_i2c *i2c, int src)
    {
    dev_dbg(&i2c.adap.dev, "txabrt: 0x%08x, cmd: %d, send: %d, recv: %d\n",
    src, i2c.cmd, i2c.cmd_buf[i2c.cmd], i2c.data_buf[i2c.cmd]);
    }
    static inline int jz4780_i2c_xfer_read(struct jz4780_i2c *i2c,
    unsigned char *buf, int len, int cnt,
    int idx)
    {
    let mut ret: c_int = 0;
    unsigned long time_left;
    let mut wait_time: c_int = JZ4780_I2C_TIMEOUT * (len + 5);
    unsigned short tmp;
    unsigned long flags;
    memset(buf, 0, len);
    spin_lock_irqsave(&i2c.lock, flags);
    i2c.stop_hold = 0;
    i2c.is_write = 0;
    i2c.rbuf = buf;
    i2c.rd_total_len = len;
    i2c.rd_data_xfered = 0;
    i2c.rd_cmd_xfered = 0;
    if (len <= i2c.cdata.fifosize)
    jz4780_i2c_writew(i2c, JZ4780_I2C_RXTL, len - 1);
    else
    jz4780_i2c_writew(i2c, JZ4780_I2C_RXTL, i2c.cdata.rx_level);
    jz4780_i2c_writew(i2c, JZ4780_I2C_TXTL, i2c.cdata.tx_level);
    jz4780_i2c_writew(i2c, JZ4780_I2C_INTM,
    JZ4780_I2C_INTM_MRXFL | JZ4780_I2C_INTM_MTXEMP
    | JZ4780_I2C_INTM_MTXABT | JZ4780_I2C_INTM_MRXOF);
    if (i2c.cdata.version < ID_X1000) {
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_CTRL);
    tmp |= JZ4780_I2C_CTRL_STPHLD;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    }
    spin_unlock_irqrestore(&i2c.lock, flags);
    time_left = wait_for_completion_timeout(&i2c.trans_waitq,
    msecs_to_jiffies(wait_time));
    if (!time_left) {
    dev_err(&i2c.adap.dev, "irq read timeout\n");
    dev_dbg(&i2c.adap.dev, "send cmd count:%d  %d\n",
    i2c.cmd, i2c.cmd_buf[i2c.cmd]);
    dev_dbg(&i2c.adap.dev, "receive data count:%d  %d\n",
    i2c.cmd, i2c.data_buf[i2c.cmd]);
    ret = -EIO;
    }
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_TXABRT);
    if (tmp) {
    jz4780_i2c_txabrt(i2c, tmp);
    ret = -EIO;
    }
    return ret;
    }
    static inline int jz4780_i2c_xfer_write(struct jz4780_i2c *i2c,
    unsigned char *buf, int len,
    int cnt, int idx)
    {
    let mut ret: c_int = 0;
    let mut wait_time: c_int = JZ4780_I2C_TIMEOUT * (len + 5);
    unsigned long time_left;
    unsigned short tmp;
    unsigned long flags;
    spin_lock_irqsave(&i2c.lock, flags);
    if (idx < (cnt - 1))
    i2c.stop_hold = 1;
    else
    i2c.stop_hold = 0;
    i2c.is_write = 1;
    i2c.wbuf = buf;
    i2c.wt_len = len;
    jz4780_i2c_writew(i2c, JZ4780_I2C_TXTL, i2c.cdata.tx_level);
    jz4780_i2c_writew(i2c, JZ4780_I2C_INTM, JZ4780_I2C_INTM_MTXEMP
    | JZ4780_I2C_INTM_MTXABT);
    if (i2c.cdata.version < ID_X1000) {
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_CTRL);
    tmp |= JZ4780_I2C_CTRL_STPHLD;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    }
    spin_unlock_irqrestore(&i2c.lock, flags);
    time_left = wait_for_completion_timeout(&i2c.trans_waitq,
    msecs_to_jiffies(wait_time));
    if (time_left && !i2c.stop_hold) {
    unsigned short i2c_sta;
    int write_in_process;
    time_left = JZ4780_I2C_TIMEOUT * 100;
    for (; time_left > 0; time_left--) {
    i2c_sta = jz4780_i2c_readw(i2c, JZ4780_I2C_STA);
    write_in_process = (i2c_sta & JZ4780_I2C_STA_MSTACT) ||
    !(i2c_sta & JZ4780_I2C_STA_TFE);
    if (!write_in_process)
    break;
    udelay(10);
    }
    }
    if (!time_left) {
    dev_err(&i2c.adap.dev, "write wait timeout\n");
    ret = -EIO;
    }
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_TXABRT);
    if (tmp) {
    jz4780_i2c_txabrt(i2c, tmp);
    ret = -EIO;
    }
    return ret;
    }
    static int jz4780_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msg,
    int count)
    {
    let mut i: c_int = -EIO;
    let mut ret: c_int = 0;
    struct jz4780_i2c *i2c = adap.algo_data;
    ret = jz4780_i2c_prepare(i2c);
    if (ret) {
    dev_err(&i2c.adap.dev, "I2C prepare failed\n");
    goto out;
    }
    if (msg.addr != jz4780_i2c_readw(i2c, JZ4780_I2C_TAR)) {
    ret = jz4780_i2c_set_target(i2c, msg.addr);
    if (ret)
    goto out;
    }
    for (i = 0; i < count; i++, msg++) {
    if (msg.flags & I2C_M_RD)
    ret = jz4780_i2c_xfer_read(i2c, msg.buf, msg.len,
    count, i);
    else
    ret = jz4780_i2c_xfer_write(i2c, msg.buf, msg.len,
    count, i);
    if (ret)
    goto out;
    }
    ret = i;
    out:
    jz4780_i2c_cleanup(i2c);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 jz4780_i2c_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm jz4780_i2c_algorithm = {
    .xfer = jz4780_i2c_xfer,
    .functionality = jz4780_i2c_functionality,
    };
    static const struct ingenic_i2c_config jz4780_i2c_config = {
    .version = ID_JZ4780,
    .fifosize = JZ4780_I2C_FIFO_LEN,
    .tx_level = JZ4780_I2C_FIFO_LEN / 2,
    .rx_level = JZ4780_I2C_FIFO_LEN / 2 - 1,
    };
    static const struct ingenic_i2c_config x1000_i2c_config = {
    .version = ID_X1000,
    .fifosize = X1000_I2C_FIFO_LEN,
    .tx_level = X1000_I2C_FIFO_LEN / 2,
    .rx_level = X1000_I2C_FIFO_LEN / 2 - 1,
    };
    static const struct of_device_id jz4780_i2c_of_matches[] = {
    { .compatible = "ingenic,jz4770-i2c", .data = &jz4780_i2c_config },
    { .compatible = "ingenic,jz4780-i2c", .data = &jz4780_i2c_config },
    { .compatible = "ingenic,x1000-i2c", .data = &x1000_i2c_config },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, jz4780_i2c_of_matches);
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int jz4780_i2c_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    let mut clk_freq: c_uint = 0;
    unsigned short tmp;
    struct jz4780_i2c *i2c;
    i2c = devm_kzalloc(&pdev.dev, sizeof(struct jz4780_i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    i2c.cdata = device_get_match_data(&pdev.dev);
    if (!i2c.cdata) {
    dev_err(&pdev.dev, "Error: No device match found\n");
    return -ENODEV;
    }
    i2c.adap.owner		= THIS_MODULE;
    i2c.adap.algo		= &jz4780_i2c_algorithm;
    i2c.adap.algo_data	= i2c;
    i2c.adap.retries	= 5;
    i2c.adap.dev.parent	= &pdev.dev;
    i2c.adap.dev.of_node	= pdev.dev.of_node;
    sprintf(i2c.adap.name, "%s", pdev.name);
    init_completion(&i2c.trans_waitq);
    spin_lock_init(&i2c.lock);
    i2c.iomem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c.iomem))
    return PTR_ERR(i2c.iomem);
    platform_set_drvdata(pdev, i2c);
    i2c.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(i2c.clk))
    return PTR_ERR(i2c.clk);
    i2c.clk_rate_khz = clk_get_rate(i2c.clk) / 1000;
    ret = of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &clk_freq);
    if (ret) {
    dev_err(&pdev.dev, "clock-frequency not specified in DT\n");
    return ret;
    }
    i2c.speed = clk_freq / 1000;
    if (i2c.speed == 0) {
    ret = -EINVAL;
    dev_err(&pdev.dev, "clock-frequency minimum is 1000\n");
    return ret;
    }
    jz4780_i2c_set_speed(i2c);
    dev_info(&pdev.dev, "Bus frequency is %d KHz\n", i2c.speed);
    if (i2c.cdata.version < ID_X1000) {
    tmp = jz4780_i2c_readw(i2c, JZ4780_I2C_CTRL);
    tmp &= ~JZ4780_I2C_CTRL_STPHLD;
    jz4780_i2c_writew(i2c, JZ4780_I2C_CTRL, tmp);
    }
    jz4780_i2c_writew(i2c, JZ4780_I2C_INTM, 0x0);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    i2c.irq = ret;
    ret = devm_request_irq(&pdev.dev, i2c.irq, jz4780_i2c_irq, 0,
    dev_name(&pdev.dev), i2c);
    if (ret)
    return ret;
    ret = i2c_add_adapter(&i2c.adap);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_i2c_remove(pdev: *mut platform_device) {
    static void jz4780_i2c_remove(struct platform_device *pdev)
    {
    struct jz4780_i2c *i2c = platform_get_drvdata(pdev);
    i2c_del_adapter(&i2c.adap);
    }
    static struct platform_driver jz4780_i2c_driver = {
    .probe		= jz4780_i2c_probe,
    .remove		= jz4780_i2c_remove,
    .driver		= {
    .name	= "jz4780-i2c",
    .of_match_table = jz4780_i2c_of_matches,
    },
    };
    module_platform_driver(jz4780_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("ztyan<ztyan@ingenic.cn>");
    MODULE_DESCRIPTION("i2c driver for JZ4780 SoCs");
