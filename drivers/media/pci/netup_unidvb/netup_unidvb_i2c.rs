//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/netup_unidvb/netup_unidvb_i2c.c
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
// netup_unidvb_i2c.c
//
// Internal I2C bus driver for NetUP Universal Dual DVB-CI
//
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

pub const NETUP_I2C_BUS0_ADDR: c_uint = 0x4800;
pub const NETUP_I2C_BUS1_ADDR: c_uint = 0x4840;
pub const NETUP_I2C_TIMEOUT: c_int = 1000;
// twi_ctrl0_stat reg bits
pub const TWI_IRQEN_COMPL: c_uint = 0x1;
pub const TWI_IRQEN_ANACK: c_uint = 0x2;
pub const TWI_IRQEN_DNACK: c_uint = 0x4;

pub const TWI_IRQ_TX: c_uint = 0x800;
pub const TWI_IRQ_RX: c_uint = 0x1000;

// twi_addr_ctrl1 reg bits
pub const TWI_TRANSFER: c_uint = 0x100;
pub const TWI_NOSTOP: c_uint = 0x200;
pub const TWI_SOFT_RESET: c_uint = 0x2000;
// twi_clkdiv reg value
pub const TWI_CLKDIV: c_int = 156;
// fifo_stat_ctrl reg bits
pub const FIFO_IRQEN: c_uint = 0x8000;
pub const FIFO_RESET: c_uint = 0x4000;
// FIFO size
pub const FIFO_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_i2c_fifo_regs {
    union {
    pub data8: __u8,
    pub data16: __le16,
    pub data32: __le32,
}

    __u8		padding[4];
    __le16		stat_ctrl;
    } __packed __aligned(1);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_i2c_regs {
    pub clkdiv: __le16,
    pub twi_ctrl0_stat: __le16,
    pub twi_addr_ctrl1: __le16,
    pub length: __le16,
    pub padding1: [__u8; 8],
    pub tx_fifo: netup_i2c_fifo_regs,
    pub padding2: [__u8; 6],
    pub rx_fifo: netup_i2c_fifo_regs,
    pub __aligned(1): } __packed,
#[no_mangle]
pub unsafe extern "C" fn netup_i2c_interrupt(i2c: *mut netup_i2c) -> irqreturn_t {
    irqreturn_t netup_i2c_interrupt(struct netup_i2c *i2c)
    {
    pub tmp: u16 reg,,
    pub flags: c_ulong,
    pub IRQ_HANDLED: irqreturn_t iret =,
    pub flags): spin_lock_irqsave(&i2c->lock,,
    pub readw(&i2c->regs->twi_ctrl0_stat): reg =,
    pub &i2c->regs->twi_ctrl0_stat): writew(reg & ~TWI_IRQEN,,
    dev_dbg(i2c.adap.dev.parent,
    pub reg): "%s(): twi_ctrl0_state 0x%x\n", __func__,,
    if ((reg & TWI_IRQEN_COMPL) != 0 && (reg & TWI_IRQ_COMPL)) {
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): TWI_IRQEN_COMPL\n",,
    pub STATE_DONE: i2c->state =,
    pub irq_ok: goto,
    }
    if ((reg & TWI_IRQEN_ANACK) != 0 && (reg & TWI_IRQ_ANACK)) {
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): TWI_IRQEN_ANACK\n",,
    pub STATE_ERROR: i2c->state =,
    pub irq_ok: goto,
    }
    if ((reg & TWI_IRQEN_DNACK) != 0 && (reg & TWI_IRQ_DNACK)) {
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): TWI_IRQEN_DNACK\n",,
    pub STATE_ERROR: i2c->state =,
    pub irq_ok: goto,
    }
    if ((reg & TWI_IRQ_RX) != 0) {
    pub readw(&i2c->regs->rx_fifo.stat_ctrl): tmp =,
    pub &i2c->regs->rx_fifo.stat_ctrl): writew(tmp & ~FIFO_IRQEN,,
    pub STATE_WANT_READ: i2c->state =,
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): want read\n",,
    pub irq_ok: goto,
    }
    if ((reg & TWI_IRQ_TX) != 0) {
    pub readw(&i2c->regs->tx_fifo.stat_ctrl): tmp =,
    pub &i2c->regs->tx_fifo.stat_ctrl): writew(tmp & ~FIFO_IRQEN,,
    pub STATE_WANT_WRITE: i2c->state =,
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): want write\n",,
    pub irq_ok: goto,
    }
    pub __func__): dev_warn(&i2c->adap.dev, "%s(): not mine interrupt\n",,
    pub IRQ_NONE: iret =,
    irq_ok:
    pub flags): spin_unlock_irqrestore(&i2c->lock,,
    if (iret == IRQ_HANDLED)
    pub iret: return,
    }
#[no_mangle]
unsafe extern "C" fn netup_i2c_reset(i2c: *mut netup_i2c) {
    static void netup_i2c_reset(struct netup_i2c *i2c)
    {
    pub __func__): dev_dbg(i2c->adap.dev.parent, "%s()\n",,
    pub STATE_DONE: i2c->state =,
    pub &i2c->regs->twi_addr_ctrl1): writew(TWI_SOFT_RESET,,
    pub &i2c->regs->clkdiv): writew(TWI_CLKDIV,,
    pub &i2c->regs->tx_fifo.stat_ctrl): writew(FIFO_RESET,,
    pub &i2c->regs->rx_fifo.stat_ctrl): writew(FIFO_RESET,,
    pub &i2c->regs->tx_fifo.stat_ctrl): writew(0x800,,
    pub &i2c->regs->rx_fifo.stat_ctrl): writew(0x800,,
    }
#[no_mangle]
unsafe extern "C" fn netup_i2c_fifo_tx(i2c: *mut netup_i2c) {
    static void netup_i2c_fifo_tx(struct netup_i2c *i2c)
    {
    pub data: u8,
    u32 fifo_space = FIFO_SIZE -
    pub 0x3f): (readw(&i2c->regs->tx_fifo.stat_ctrl) &,
    pub i2c->xmit_size: u32 msg_length = i2c->msg->len -,
    pub fifo_space): msg_length = min(msg_length,,
    while (msg_length--) {
    pub i2c->msg->buf[i2c->xmit_size++]: data =,
    pub &i2c->regs->tx_fifo.data8): writeb(data,,
    dev_dbg(i2c.adap.dev.parent,
    pub data): "%s(): write 0x%02x\n", __func__,,
    }
    if (i2c.xmit_size < i2c.msg.len) {
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): TX IRQ enabled\n",,
    writew(readw(&i2c.regs.tx_fifo.stat_ctrl) | FIFO_IRQEN,
    }
    }
#[no_mangle]
unsafe extern "C" fn netup_i2c_fifo_rx(i2c: *mut netup_i2c) {
    static void netup_i2c_fifo_rx(struct netup_i2c *i2c)
    {
    pub data: u8,
    pub 0x3f: u32 fifo_size = readw(&i2c->regs->rx_fifo.stat_ctrl) &,
    dev_dbg(i2c.adap.dev.parent,
    pub fifo_size): "%s(): RX fifo size %d\n", __func__,,
    while (fifo_size--) {
    pub readb(&i2c->regs->rx_fifo.data8): data =,
    if ((i2c.msg.flags & I2C_M_RD) != 0 &&
    i2c.xmit_size < i2c.msg.len) {
    pub data: i2c->msg->buf[i2c->xmit_size++] =,
    dev_dbg(i2c.adap.dev.parent,
    pub data): "%s(): read 0x%02x\n", __func__,,
    }
    }
    if (i2c.xmit_size < i2c.msg.len) {
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): RX IRQ enabled\n",,
    writew(readw(&i2c.regs.rx_fifo.stat_ctrl) | FIFO_IRQEN,
    }
    }
#[no_mangle]
unsafe extern "C" fn netup_i2c_start_xfer(i2c: *mut netup_i2c) {
    static void netup_i2c_start_xfer(struct netup_i2c *i2c)
    {
    pub 0): u16 rdflag = ((i2c->msg->flags & I2C_M_RD) ? 1 :,
    pub readw(&i2c->regs->twi_ctrl0_stat): u16 reg =,
    pub &i2c->regs->twi_ctrl0_stat): writew(TWI_IRQEN | reg,,
    pub &i2c->regs->length): writew(i2c->msg->len,,
    writew(TWI_TRANSFER | (i2c.msg.addr << 1) | rdflag,
    dev_dbg(i2c.adap.dev.parent,
    "%s(): length %d twi_addr_ctrl1 0x%x twi_ctrl0_stat 0x%x\n",
    __func__, readw(&i2c.regs.length),
    readw(&i2c.regs.twi_addr_ctrl1),
    pub STATE_WAIT: i2c->state =,
    pub 0: i2c->xmit_size =,
    if (!rdflag)
    else
    writew(FIFO_IRQEN | readw(&i2c.regs.rx_fifo.stat_ctrl),
    }
    static int netup_i2c_xfer(struct i2c_adapter *adap,
    struct i2c_msg *msgs, int num)
    {
    pub flags: c_ulong,
    pub num: int i, trans_done, res =,
    pub i2c_get_adapdata(adap): *mut *mut netup_i2c i2c =,
    pub reg: u16,
    pub flags): spin_lock_irqsave(&i2c->lock,,
    if (i2c.state != STATE_DONE) {
    dev_dbg(i2c.adap.dev.parent,
    "%s(): i2c.state == %d, resetting I2C\n",
    pub i2c->state): __func__,,
    }
    pub num): dev_dbg(i2c->adap.dev.parent, "%s() num %d\n", __func__,,
    pub {: for (i = 0; i < num; i++),
    pub &msgs[i]: i2c->msg =,
    pub 0: trans_done =,
    while (!trans_done) {
    pub flags): spin_unlock_irqrestore(&i2c->lock,,
    if (wait_event_timeout(i2c.wq,
    i2c.state != STATE_WAIT,
    msecs_to_jiffies(NETUP_I2C_TIMEOUT))) {
    pub flags): spin_lock_irqsave(&i2c->lock,,
    switch (i2c.state) {
    case STATE_WANT_READ:
    case STATE_WANT_WRITE:
    case STATE_DONE:
    if ((i2c.msg.flags & I2C_M_RD) != 0 &&
    i2c.xmit_size != i2c.msg.len)
    dev_dbg(i2c.adap.dev.parent,
    "%s(): msg %d OK\n",
    pub i): __func__,,
    pub 1: trans_done =,
    case STATE_ERROR:
    pub -EIO: res =,
    dev_dbg(i2c.adap.dev.parent,
    "%s(): error state\n",
    pub done: goto,
    default:
    dev_dbg(i2c.adap.dev.parent,
    "%s(): invalid state %d\n",
    pub i2c->state): __func__,,
    pub -EINVAL: res =,
    pub done: goto,
    }
    if (!trans_done) {
    pub STATE_WAIT: i2c->state =,
    reg = readw(
    writew(TWI_IRQEN | reg,
    }
    pub flags): spin_unlock_irqrestore(&i2c->lock,,
    } else {
    pub flags): spin_lock_irqsave(&i2c->lock,,
    dev_dbg(i2c.adap.dev.parent,
    pub __func__): "%s(): wait timeout\n",,
    pub -ETIMEDOUT: res =,
    pub done: goto,
    }
    pub flags): spin_lock_irqsave(&i2c->lock,,
    }
    }
    done:
    pub flags): spin_unlock_irqrestore(&i2c->lock,,
    pub res): dev_dbg(i2c->adap.dev.parent, "%s(): result %d\n", __func__,,
    pub res: return,
    }
#[no_mangle]
unsafe extern "C" fn netup_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 netup_i2c_func(struct i2c_adapter *adap)
    {
    pub I2C_FUNC_SMBUS_EMUL: return I2C_FUNC_I2C |,
    }
    static const struct i2c_algorithm netup_i2c_algorithm = {
    .master_xfer	= netup_i2c_xfer,
    .functionality	= netup_i2c_func,
}

    static const struct i2c_adapter netup_i2c_adapter = {
    .owner		= THIS_MODULE,
    .name		= NETUP_UNIDVB_NAME,
    .class		= I2C_CLASS_HWMON,
    .algo		= &netup_i2c_algorithm,
    };
#[no_mangle]
unsafe extern "C" fn netup_i2c_init(ndev: *mut netup_unidvb_dev, bus_num: c_int) -> c_int {
    static int netup_i2c_init(struct netup_unidvb_dev *ndev, int bus_num)
    {
    int ret;
    struct netup_i2c *i2c;
    if (bus_num < 0 || bus_num > 1) {
    dev_err(&ndev.pci_dev.dev,
    "%s(): invalid bus_num %d\n", __func__, bus_num);
    return -EINVAL;
    }
    i2c = &ndev.i2c[bus_num];
    spin_lock_init(&i2c.lock);
    init_waitqueue_head(&i2c.wq);
    i2c.regs = (struct netup_i2c_regs __iomem *)(ndev.bmmio0 +
    (bus_num == 0 ? NETUP_I2C_BUS0_ADDR : NETUP_I2C_BUS1_ADDR));
    netup_i2c_reset(i2c);
    i2c.adap = netup_i2c_adapter;
    i2c.adap.dev.parent = &ndev.pci_dev.dev;
    i2c_set_adapdata(&i2c.adap, i2c);
    ret = i2c_add_adapter(&i2c.adap);
    if (ret)
    return ret;
    dev_info(&ndev.pci_dev.dev,
    "%s(): registered I2C bus %d at 0x%x\n",
    __func__,
    bus_num, (bus_num == 0 ?
    NETUP_I2C_BUS0_ADDR :
    NETUP_I2C_BUS1_ADDR));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netup_i2c_remove(ndev: *mut netup_unidvb_dev, bus_num: c_int) {
    static void netup_i2c_remove(struct netup_unidvb_dev *ndev, int bus_num)
    {
    struct netup_i2c *i2c;
    if (bus_num < 0 || bus_num > 1) {
    dev_err(&ndev.pci_dev.dev,
    "%s(): invalid bus number %d\n", __func__, bus_num);
    return;
    }
    i2c = &ndev.i2c[bus_num];
    netup_i2c_reset(i2c);
// remove adapter
    i2c_del_adapter(&i2c.adap);
    dev_info(&ndev.pci_dev.dev,
    "netup_i2c_remove: unregistered I2C bus %d\n", bus_num);
    }
#[no_mangle]
pub unsafe extern "C" fn netup_i2c_register(ndev: *mut netup_unidvb_dev) -> c_int {
    int netup_i2c_register(struct netup_unidvb_dev *ndev)
    {
    int ret;
    ret = netup_i2c_init(ndev, 0);
    if (ret)
    return ret;
    ret = netup_i2c_init(ndev, 1);
    if (ret) {
    netup_i2c_remove(ndev, 0);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn netup_i2c_unregister(ndev: *mut netup_unidvb_dev) {
    void netup_i2c_unregister(struct netup_unidvb_dev *ndev)
    {
    netup_i2c_remove(ndev, 0);
    netup_i2c_remove(ndev, 1);
    }
