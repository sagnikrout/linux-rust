//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/i3c-master-cdns.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 Cadence Design Systems Inc.
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

pub const DEV_ID: c_uint = 0x0;
pub const DEV_ID_I3C_MASTER: c_uint = 0x5034;
pub const CONF_STATUS0: c_uint = 0x4;

pub const CONF_STATUS1: c_uint = 0x8;

pub const REV_ID: c_uint = 0xc;

pub const CTRL: c_uint = 0x10;

pub const CTRL_PURE_BUS_MODE: c_int = 0;
pub const CTRL_MIXED_FAST_BUS_MODE: c_int = 2;
pub const CTRL_MIXED_SLOW_BUS_MODE: c_int = 3;

pub const THD_DELAY_MAX: c_int = 3;
pub const PRESCL_CTRL0: c_uint = 0x14;

pub const PRESCL_CTRL1: c_uint = 0x18;

pub const MST_IER: c_uint = 0x20;
pub const MST_IDR: c_uint = 0x24;
pub const MST_IMR: c_uint = 0x28;
pub const MST_ICR: c_uint = 0x2c;
pub const MST_ISR: c_uint = 0x30;

pub const MST_STATUS0: c_uint = 0x34;

pub const CMDR: c_uint = 0x38;
pub const CMDR_NO_ERROR: c_int = 0;
pub const CMDR_DDR_PREAMBLE_ERROR: c_int = 1;
pub const CMDR_DDR_PARITY_ERROR: c_int = 2;
pub const CMDR_DDR_RX_FIFO_OVF: c_int = 3;
pub const CMDR_DDR_TX_FIFO_UNF: c_int = 4;
pub const CMDR_M0_ERROR: c_int = 5;
pub const CMDR_M1_ERROR: c_int = 6;
pub const CMDR_M2_ERROR: c_int = 7;
pub const CMDR_MST_ABORT: c_int = 8;
pub const CMDR_NACK_RESP: c_int = 9;
pub const CMDR_INVALID_DA: c_int = 10;
pub const CMDR_DDR_DROPPED: c_int = 11;

pub const CMDR_CMDID_HJACK_DISEC: c_uint = 0xfe;
pub const CMDR_CMDID_HJACK_ENTDAA: c_uint = 0xff;

pub const IBIR: c_uint = 0x3c;

pub const IBIR_TYPE_IBI: c_int = 0;
pub const IBIR_TYPE_HJ: c_int = 1;
pub const IBIR_TYPE_MR: c_int = 2;

pub const SLV_IER: c_uint = 0x40;
pub const SLV_IDR: c_uint = 0x44;
pub const SLV_IMR: c_uint = 0x48;
pub const SLV_ICR: c_uint = 0x4c;
pub const SLV_ISR: c_uint = 0x50;

pub const SLV_STATUS0: c_uint = 0x54;

pub const SLV_STATUS1: c_uint = 0x58;

pub const CMD0_FIFO: c_uint = 0x60;

pub const XMIT_BURST_STATIC_SUBADDR: c_int = 0;
pub const XMIT_SINGLE_INC_SUBADDR: c_int = 1;
pub const XMIT_SINGLE_STATIC_SUBADDR: c_int = 2;
pub const XMIT_BURST_WITHOUT_SUBADDR: c_int = 3;

pub const CMD0_FIFO_PL_LEN_MAX: c_int = 4095;

pub const CMD1_FIFO: c_uint = 0x64;

pub const TX_FIFO: c_uint = 0x68;
pub const IMD_CMD0: c_uint = 0x70;

pub const IMD_CMD1: c_uint = 0x74;

pub const IMD_DATA: c_uint = 0x78;
pub const RX_FIFO: c_uint = 0x80;
pub const IBI_DATA_FIFO: c_uint = 0x84;
pub const SLV_DDR_TX_FIFO: c_uint = 0x88;
pub const SLV_DDR_RX_FIFO: c_uint = 0x8c;
pub const CMD_IBI_THR_CTRL: c_uint = 0x90;

pub const TX_RX_THR_CTRL: c_uint = 0x94;

pub const SLV_DDR_TX_RX_THR_CTRL: c_uint = 0x98;

pub const FLUSH_CTRL: c_uint = 0x9c;

pub const TTO_PRESCL_CTRL0: c_uint = 0xb0;

pub const TTO_PRESCL_CTRL1: c_uint = 0xb4;

pub const DEVS_CTRL: c_uint = 0xb8;
pub const DEVS_CTRL_DEV_CLR_SHIFT: c_int = 16;

pub const MAX_DEVS: c_int = 16;

    (((a) & GENMASK(9, 7)) << 6))

    (((x) >> 6) & GENMASK(9, 7)))

pub const DEV_ROLE_SLAVE: c_int = 0;
pub const DEV_ROLE_MASTER: c_int = 1;

    (((val) >> (((id) % 4) * 8)) & GENMASK(7, 0))

    (((val) >> (((id) % 4) * 8)) & GENMASK(7, 0))
pub const ASF_INT_STATUS: c_uint = 0x300;
pub const ASF_INT_RAW_STATUS: c_uint = 0x304;
pub const ASF_INT_MASK: c_uint = 0x308;
pub const ASF_INT_TEST: c_uint = 0x30c;
pub const ASF_INT_FATAL_SELECT: c_uint = 0x310;

pub const ASF_SRAM_CORR_FAULT_STATUS: c_uint = 0x320;
pub const ASF_SRAM_UNCORR_FAULT_STATUS: c_uint = 0x324;

pub const ASF_SRAM_FAULT_STATS: c_uint = 0x328;

pub const ASF_TRANS_TOUT_CTRL: c_uint = 0x330;

pub const ASF_TRANS_TOUT_FAULT_MASK: c_uint = 0x334;
pub const ASF_TRANS_TOUT_FAULT_STATUS: c_uint = 0x338;

pub const ASF_PROTO_FAULT_MASK: c_uint = 0x340;
pub const ASF_PROTO_FAULT_STATUS: c_uint = 0x344;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i3c_master_caps {
    pub cmdfifodepth: u32,
    pub cmdrfifodepth: u32,
    pub txfifodepth: u32,
    pub rxfifodepth: u32,
    pub ibirfifodepth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i3c_cmd {
    pub cmd0: u32,
    pub cmd1: u32,
    pub tx_len: u32,
    pub tx_buf: *const c_void,
    pub rx_len: u32,
    pub rx_buf: *mut c_void,
    pub error: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i3c_xfer {
    pub node: list_head,
    pub comp: completion,
    pub ret: c_int,
    pub ncmds: c_uint,
    pub __counted_by(ncmds): cdns_i3c_cmd cmds[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i3c_data {
    pub thd_delay_ns: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i3c_master {
    pub base: i3c_master_controller,
    pub free_rr_slots: u32,
    pub maxdevs: c_uint,
    struct {
    pub num_slots: c_uint,
    pub slots: *mut i3c_dev_desc,
    pub lock: spinlock_t,
    pub ibi: },
    struct {
    pub list: list_head,
    pub cur: *mut cdns_i3c_xfer,
    pub lock: spinlock_t,
    pub xferqueue: },
    pub regs: *mut void __iomem,
    pub sysclk: *mut clk,
    pub caps: cdns_i3c_master_caps,
    pub i3c_scl_lim: c_ulong,
    pub devdata: *const cdns_i3c_data,
}

    static inline struct cdns_i3c_master *
    to_cdns_i3c_master(struct i3c_master_controller *master)
    {
    return container_of(master, struct cdns_i3c_master, base);
    }
    static void cdns_i3c_master_wr_to_tx_fifo(struct cdns_i3c_master *master,
    const u8 *bytes, int nbytes)
    {
    i3c_writel_fifo(master.regs + TX_FIFO, bytes, nbytes);
    }
    static void cdns_i3c_master_rd_from_rx_fifo(struct cdns_i3c_master *master,
    u8 *bytes, int nbytes)
    {
    i3c_readl_fifo(master.regs + RX_FIFO, bytes, nbytes);
    }
    static bool cdns_i3c_master_supports_ccc_cmd(struct i3c_master_controller *m,
    const struct i3c_ccc_cmd *cmd)
    {
    if (cmd.ndests > 1)
    return false;
    switch (cmd.id) {
    case I3C_CCC_ENEC(true):
    case I3C_CCC_ENEC(false):
    case I3C_CCC_DISEC(true):
    case I3C_CCC_DISEC(false):
    case I3C_CCC_ENTAS(0, true):
    case I3C_CCC_ENTAS(0, false):
    case I3C_CCC_RSTDAA(true):
    case I3C_CCC_RSTDAA(false):
    case I3C_CCC_ENTDAA:
    case I3C_CCC_SETMWL(true):
    case I3C_CCC_SETMWL(false):
    case I3C_CCC_SETMRL(true):
    case I3C_CCC_SETMRL(false):
    case I3C_CCC_DEFSLVS:
    case I3C_CCC_ENTHDR(0):
    case I3C_CCC_SETDASA:
    case I3C_CCC_SETNEWDA:
    case I3C_CCC_GETMWL:
    case I3C_CCC_GETMRL:
    case I3C_CCC_GETPID:
    case I3C_CCC_GETBCR:
    case I3C_CCC_GETDCR:
    case I3C_CCC_GETSTATUS:
    case I3C_CCC_GETACCMST:
    case I3C_CCC_GETMXDS:
    case I3C_CCC_GETHDRCAP:
    return true;
    default:
    break;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_disable(master: *mut cdns_i3c_master) -> c_int {
    static int cdns_i3c_master_disable(struct cdns_i3c_master *master)
    {
    u32 status;
    writel(readl(master.regs + CTRL) & ~CTRL_DEV_EN, master.regs + CTRL);
    return readl_poll_timeout(master.regs + MST_STATUS0, status,
    status & MST_STATUS0_IDLE, 10, 1000000);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_enable(master: *mut cdns_i3c_master) {
    static void cdns_i3c_master_enable(struct cdns_i3c_master *master)
    {
    writel(readl(master.regs + CTRL) | CTRL_DEV_EN, master.regs + CTRL);
    }
    static struct cdns_i3c_xfer *
    cdns_i3c_master_alloc_xfer(struct cdns_i3c_master *master, unsigned int ncmds)
    {
    struct cdns_i3c_xfer *xfer;
    xfer = kzalloc_flex(*xfer, cmds, ncmds);
    if (!xfer)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&xfer.node);
    xfer.ncmds = ncmds;
    xfer.ret = -ETIMEDOUT;
    return xfer;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_free_xfer(xfer: *mut cdns_i3c_xfer) {
    static void cdns_i3c_master_free_xfer(struct cdns_i3c_xfer *xfer)
    {
    kfree(xfer);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_start_xfer_locked(master: *mut cdns_i3c_master) {
    static void cdns_i3c_master_start_xfer_locked(struct cdns_i3c_master *master)
    {
    struct cdns_i3c_xfer *xfer = master.xferqueue.cur;
    unsigned int i;
    if (!xfer)
    return;
    writel(MST_INT_CMDD_EMP, master.regs + MST_ICR);
    for (i = 0; i < xfer.ncmds; i++) {
    struct cdns_i3c_cmd *cmd = &xfer.cmds[i];
    cdns_i3c_master_wr_to_tx_fifo(master, cmd.tx_buf,
    cmd.tx_len);
    }
    for (i = 0; i < xfer.ncmds; i++) {
    struct cdns_i3c_cmd *cmd = &xfer.cmds[i];
    writel(cmd.cmd1 | CMD1_FIFO_CMDID(i),
    master.regs + CMD1_FIFO);
    writel(cmd.cmd0, master.regs + CMD0_FIFO);
    }
    writel(readl(master.regs + CTRL) | CTRL_MCS,
    master.regs + CTRL);
    writel(MST_INT_CMDD_EMP, master.regs + MST_IER);
    }
    static void cdns_i3c_master_end_xfer_locked(struct cdns_i3c_master *master,
    u32 isr)
    {
    struct cdns_i3c_xfer *xfer = master.xferqueue.cur;
    int i, ret = 0;
    u32 status0;
    if (!xfer)
    return;
    if (!(isr & MST_INT_CMDD_EMP))
    return;
    writel(MST_INT_CMDD_EMP, master.regs + MST_IDR);
    for (status0 = readl(master.regs + MST_STATUS0);
    !(status0 & MST_STATUS0_CMDR_EMP);
    status0 = readl(master.regs + MST_STATUS0)) {
    struct cdns_i3c_cmd *cmd;
    u32 cmdr, rx_len, id;
    cmdr = readl(master.regs + CMDR);
    id = CMDR_CMDID(cmdr);
    if (id == CMDR_CMDID_HJACK_DISEC ||
    id == CMDR_CMDID_HJACK_ENTDAA ||
    WARN_ON(id >= xfer.ncmds))
    continue;
    cmd = &xfer.cmds[CMDR_CMDID(cmdr)];
    rx_len = min_t(u32, CMDR_XFER_BYTES(cmdr), cmd.rx_len);
    cdns_i3c_master_rd_from_rx_fifo(master, cmd.rx_buf, rx_len);
    cmd.rx_len = rx_len;
    cmd.error = CMDR_ERROR(cmdr);
    }
    for (i = 0; i < xfer.ncmds; i++) {
    switch (xfer.cmds[i].error) {
    case CMDR_NO_ERROR:
    break;
    case CMDR_DDR_PREAMBLE_ERROR:
    case CMDR_DDR_PARITY_ERROR:
    case CMDR_M0_ERROR:
    case CMDR_M1_ERROR:
    case CMDR_M2_ERROR:
    case CMDR_MST_ABORT:
    case CMDR_NACK_RESP:
    case CMDR_DDR_DROPPED:
    ret = -EIO;
    break;
    case CMDR_DDR_RX_FIFO_OVF:
    case CMDR_DDR_TX_FIFO_UNF:
    ret = -ENOSPC;
    break;
    case CMDR_INVALID_DA:
    default:
    ret = -EINVAL;
    break;
    }
    }
    xfer.ret = ret;
    complete(&xfer.comp);
    xfer = list_first_entry_or_null(&master.xferqueue.list,
    struct cdns_i3c_xfer, node);
    if (xfer)
    list_del_init(&xfer.node);
    master.xferqueue.cur = xfer;
    cdns_i3c_master_start_xfer_locked(master);
    }
    static void cdns_i3c_master_queue_xfer(struct cdns_i3c_master *master,
    struct cdns_i3c_xfer *xfer)
    {
    unsigned long flags;
    init_completion(&xfer.comp);
    spin_lock_irqsave(&master.xferqueue.lock, flags);
    if (master.xferqueue.cur) {
    list_add_tail(&xfer.node, &master.xferqueue.list);
    } else {
    master.xferqueue.cur = xfer;
    cdns_i3c_master_start_xfer_locked(master);
    }
    spin_unlock_irqrestore(&master.xferqueue.lock, flags);
    }
    static void cdns_i3c_master_unqueue_xfer(struct cdns_i3c_master *master,
    struct cdns_i3c_xfer *xfer)
    {
    unsigned long flags;
    spin_lock_irqsave(&master.xferqueue.lock, flags);
    if (master.xferqueue.cur == xfer) {
    u32 status;
    writel(readl(master.regs + CTRL) & ~CTRL_DEV_EN,
    master.regs + CTRL);
    readl_poll_timeout_atomic(master.regs + MST_STATUS0, status,
    status & MST_STATUS0_IDLE, 10,
    1000000);
    master.xferqueue.cur = core::ptr::null_mut();
    writel(FLUSH_RX_FIFO | FLUSH_TX_FIFO | FLUSH_CMD_FIFO |
    FLUSH_CMD_RESP,
    master.regs + FLUSH_CTRL);
    writel(MST_INT_CMDD_EMP, master.regs + MST_IDR);
    writel(readl(master.regs + CTRL) | CTRL_DEV_EN,
    master.regs + CTRL);
    } else {
    list_del_init(&xfer.node);
    }
    spin_unlock_irqrestore(&master.xferqueue.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_cmd_get_err(cmd: *mut cdns_i3c_cmd) -> enum i3c_error_code {
    static enum i3c_error_code cdns_i3c_cmd_get_err(struct cdns_i3c_cmd *cmd)
    {
    switch (cmd.error) {
    case CMDR_M0_ERROR:
    return I3C_ERROR_M0;
    case CMDR_M1_ERROR:
    return I3C_ERROR_M1;
    case CMDR_M2_ERROR:
    case CMDR_NACK_RESP:
    return I3C_ERROR_M2;
    default:
    break;
    }
    return I3C_ERROR_UNKNOWN;
    }
    static int cdns_i3c_master_send_ccc_cmd(struct i3c_master_controller *m,
    struct i3c_ccc_cmd *cmd)
    {
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_xfer *xfer;
    struct cdns_i3c_cmd *ccmd;
    int ret;
    xfer = cdns_i3c_master_alloc_xfer(master, 1);
    if (!xfer)
    return -ENOMEM;
    ccmd = xfer.cmds;
    ccmd.cmd1 = CMD1_FIFO_CCC(cmd.id);
    ccmd.cmd0 = CMD0_FIFO_IS_CCC |
    CMD0_FIFO_PL_LEN(cmd.dests[0].payload.len);
    if (cmd.id & I3C_CCC_DIRECT)
    ccmd.cmd0 |= CMD0_FIFO_DEV_ADDR(cmd.dests[0].addr);
    if (cmd.rnw) {
    ccmd.cmd0 |= CMD0_FIFO_RNW;
    ccmd.rx_buf = cmd.dests[0].payload.data;
    ccmd.rx_len = cmd.dests[0].payload.len;
    } else {
    ccmd.tx_buf = cmd.dests[0].payload.data;
    ccmd.tx_len = cmd.dests[0].payload.len;
    }
    cdns_i3c_master_queue_xfer(master, xfer);
    if (!wait_for_completion_timeout(&xfer.comp, msecs_to_jiffies(1000)))
    cdns_i3c_master_unqueue_xfer(master, xfer);
    ret = xfer.ret;
    cmd.err = cdns_i3c_cmd_get_err(&xfer.cmds[0]);
    if (!ret && cmd.rnw)
    cmd.dests[0].payload.actual_len = ccmd.rx_len;
    cdns_i3c_master_free_xfer(xfer);
    return ret;
    }
    static int cdns_i3c_master_i3c_xfers(struct i3c_dev_desc *dev,
    struct i3c_xfer *xfers,
    int nxfers, enum i3c_xfer_mode mode)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    let mut txslots: c_int = 0, rxslots = 0, i, ret;
    struct cdns_i3c_xfer *cdns_xfer;
    for (i = 0; i < nxfers; i++) {
    if (xfers[i].len > CMD0_FIFO_PL_LEN_MAX)
    return -EOPNOTSUPP;
    }
    if (!nxfers)
    return 0;
    if (nxfers > master.caps.cmdfifodepth ||
    nxfers > master.caps.cmdrfifodepth)
    return -EOPNOTSUPP;
//
// First make sure that all transactions (block of transfers separated
// by a STOP marker) fit in the FIFOs.
//
    for (i = 0; i < nxfers; i++) {
    if (xfers[i].rnw)
    rxslots += DIV_ROUND_UP(xfers[i].len, 4);
    else
    txslots += DIV_ROUND_UP(xfers[i].len, 4);
    }
    if (rxslots > master.caps.rxfifodepth ||
    txslots > master.caps.txfifodepth)
    return -EOPNOTSUPP;
    cdns_xfer = cdns_i3c_master_alloc_xfer(master, nxfers);
    if (!cdns_xfer)
    return -ENOMEM;
    for (i = 0; i < nxfers; i++) {
    struct cdns_i3c_cmd *ccmd = &cdns_xfer.cmds[i];
    let mut pl_len: u32 = xfers[i].len;
    ccmd.cmd0 = CMD0_FIFO_DEV_ADDR(dev.info.dyn_addr) |
    CMD0_FIFO_PRIV_XMIT_MODE(XMIT_BURST_WITHOUT_SUBADDR);
    if (xfers[i].rnw) {
    ccmd.cmd0 |= CMD0_FIFO_RNW;
    ccmd.rx_buf = xfers[i].data.in;
    ccmd.rx_len = xfers[i].len;
    pl_len++;
    } else {
    ccmd.tx_buf = xfers[i].data.out;
    ccmd.tx_len = xfers[i].len;
    }
    ccmd.cmd0 |= CMD0_FIFO_PL_LEN(pl_len);
    if (i < nxfers - 1)
    ccmd.cmd0 |= CMD0_FIFO_RSBC;
    if (!i)
    ccmd.cmd0 |= CMD0_FIFO_BCH;
    }
    cdns_i3c_master_queue_xfer(master, cdns_xfer);
    if (!wait_for_completion_timeout(&cdns_xfer.comp,
    msecs_to_jiffies(1000)))
    cdns_i3c_master_unqueue_xfer(master, cdns_xfer);
    ret = cdns_xfer.ret;
    for (i = 0; i < nxfers; i++)
    xfers[i].err = cdns_i3c_cmd_get_err(&cdns_xfer.cmds[i]);
    cdns_i3c_master_free_xfer(cdns_xfer);
    return ret;
    }
    static int cdns_i3c_master_i2c_xfers(struct i2c_dev_desc *dev,
    struct i2c_msg *xfers, int nxfers)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    let mut nrxwords: c_uint = 0, ntxwords = 0;
    struct cdns_i3c_xfer *xfer;
    int i, ret = 0;
    if (nxfers > master.caps.cmdfifodepth)
    return -EOPNOTSUPP;
    for (i = 0; i < nxfers; i++) {
    if (xfers[i].len > CMD0_FIFO_PL_LEN_MAX)
    return -EOPNOTSUPP;
    if (xfers[i].flags & I2C_M_RD)
    nrxwords += DIV_ROUND_UP(xfers[i].len, 4);
    else
    ntxwords += DIV_ROUND_UP(xfers[i].len, 4);
    }
    if (ntxwords > master.caps.txfifodepth ||
    nrxwords > master.caps.rxfifodepth)
    return -EOPNOTSUPP;
    xfer = cdns_i3c_master_alloc_xfer(master, nxfers);
    if (!xfer)
    return -ENOMEM;
    for (i = 0; i < nxfers; i++) {
    struct cdns_i3c_cmd *ccmd = &xfer.cmds[i];
    ccmd.cmd0 = CMD0_FIFO_DEV_ADDR(xfers[i].addr) |
    CMD0_FIFO_PL_LEN(xfers[i].len) |
    CMD0_FIFO_PRIV_XMIT_MODE(XMIT_BURST_WITHOUT_SUBADDR);
    if (xfers[i].flags & I2C_M_TEN)
    ccmd.cmd0 |= CMD0_FIFO_IS_10B;
    if (xfers[i].flags & I2C_M_RD) {
    ccmd.cmd0 |= CMD0_FIFO_RNW;
    ccmd.rx_buf = xfers[i].buf;
    ccmd.rx_len = xfers[i].len;
    } else {
    ccmd.tx_buf = xfers[i].buf;
    ccmd.tx_len = xfers[i].len;
    }
    }
    cdns_i3c_master_queue_xfer(master, xfer);
    if (!wait_for_completion_timeout(&xfer.comp, m.i2c.timeout))
    cdns_i3c_master_unqueue_xfer(master, xfer);
    ret = xfer.ret;
    cdns_i3c_master_free_xfer(xfer);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i3c_i2c_dev_data {
    pub id: u16,
    pub ibi: i16,
    pub ibi_pool: *mut i3c_generic_ibi_pool,
}

#[no_mangle]
unsafe extern "C" fn prepare_rr0_dev_address(addr: u32) -> u32 {
    static u32 prepare_rr0_dev_address(u32 addr)
    {
    let mut ret: u32 = (addr << 1) & 0xff;
// RR0[7:1] = addr[6:0]
    ret |= (addr & GENMASK(6, 0)) << 1;
// RR0[15:13] = addr[9:7]
    ret |= (addr & GENMASK(9, 7)) << 6;
// RR0[0] = ~XOR(addr[6:0])
    ret |= parity8(addr & 0x7f) ? 0 : BIT(0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_upd_i3c_addr(dev: *mut i3c_dev_desc) {
    static void cdns_i3c_master_upd_i3c_addr(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    u32 rr;
    rr = prepare_rr0_dev_address(dev.info.dyn_addr ?
    dev.info.dyn_addr :
    dev.info.static_addr);
    writel(DEV_ID_RR0_IS_I3C | rr, master.regs + DEV_ID_RR0(data.id));
    }
    static int cdns_i3c_master_get_rr_slot(struct cdns_i3c_master *master,
    u8 dyn_addr)
    {
    unsigned long activedevs;
    u32 rr;
    int i;
    if (!dyn_addr) {
    if (!master.free_rr_slots)
    return -ENOSPC;
    return ffs(master.free_rr_slots) - 1;
    }
    activedevs = readl(master.regs + DEVS_CTRL) & DEVS_CTRL_DEVS_ACTIVE_MASK;
    activedevs &= ~BIT(0);
    for_each_set_bit(i, &activedevs, master.maxdevs + 1) {
    rr = readl(master.regs + DEV_ID_RR0(i));
    if (!(rr & DEV_ID_RR0_IS_I3C) ||
    DEV_ID_RR0_GET_DEV_ADDR(rr) != dyn_addr)
    continue;
    return i;
    }
    return -EINVAL;
    }
    static int cdns_i3c_master_reattach_i3c_dev(struct i3c_dev_desc *dev,
    u8 old_dyn_addr)
    {
    cdns_i3c_master_upd_i3c_addr(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_attach_i3c_dev(dev: *mut i3c_dev_desc) -> c_int {
    static int cdns_i3c_master_attach_i3c_dev(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data;
    int slot;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    slot = cdns_i3c_master_get_rr_slot(master, dev.info.dyn_addr);
    if (slot < 0) {
    kfree(data);
    return slot;
    }
    data.ibi = -1;
    data.id = slot;
    i3c_dev_set_master_data(dev, data);
    master.free_rr_slots &= ~BIT(slot);
    if (!dev.info.dyn_addr) {
    cdns_i3c_master_upd_i3c_addr(dev);
    writel(readl(master.regs + DEVS_CTRL) |
    DEVS_CTRL_DEV_ACTIVE(data.id),
    master.regs + DEVS_CTRL);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_detach_i3c_dev(dev: *mut i3c_dev_desc) {
    static void cdns_i3c_master_detach_i3c_dev(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    writel(readl(master.regs + DEVS_CTRL) |
    DEVS_CTRL_DEV_CLR(data.id),
    master.regs + DEVS_CTRL);
    i3c_dev_set_master_data(dev, core::ptr::null_mut());
    master.free_rr_slots |= BIT(data.id);
    kfree(data);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_attach_i2c_dev(dev: *mut i2c_dev_desc) -> c_int {
    static int cdns_i3c_master_attach_i2c_dev(struct i2c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data;
    int slot;
    slot = cdns_i3c_master_get_rr_slot(master, 0);
    if (slot < 0)
    return slot;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    data.id = slot;
    master.free_rr_slots &= ~BIT(slot);
    i2c_dev_set_master_data(dev, data);
    writel(prepare_rr0_dev_address(dev.addr),
    master.regs + DEV_ID_RR0(data.id));
    writel(dev.lvr, master.regs + DEV_ID_RR2(data.id));
    writel(readl(master.regs + DEVS_CTRL) |
    DEVS_CTRL_DEV_ACTIVE(data.id),
    master.regs + DEVS_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_detach_i2c_dev(dev: *mut i2c_dev_desc) {
    static void cdns_i3c_master_detach_i2c_dev(struct i2c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i2c_dev_get_master_data(dev);
    writel(readl(master.regs + DEVS_CTRL) |
    DEVS_CTRL_DEV_CLR(data.id),
    master.regs + DEVS_CTRL);
    master.free_rr_slots |= BIT(data.id);
    i2c_dev_set_master_data(dev, core::ptr::null_mut());
    kfree(data);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_bus_cleanup(m: *mut i3c_master_controller) {
    static void cdns_i3c_master_bus_cleanup(struct i3c_master_controller *m)
    {
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    cdns_i3c_master_disable(master);
    }
    static void cdns_i3c_master_dev_rr_to_info(struct cdns_i3c_master *master,
    unsigned int slot,
    struct i3c_device_info *info)
    {
    u32 rr;
    memset(info, 0, sizeof(*info));
    rr = readl(master.regs + DEV_ID_RR0(slot));
    info.dyn_addr = DEV_ID_RR0_GET_DEV_ADDR(rr);
    rr = readl(master.regs + DEV_ID_RR2(slot));
    info.dcr = rr;
    info.bcr = rr >> 8;
    info.pid = rr >> 16;
    info.pid |= (u64)readl(master.regs + DEV_ID_RR1(slot)) << 16;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_upd_i3c_scl_lim(master: *mut cdns_i3c_master) {
    static void cdns_i3c_master_upd_i3c_scl_lim(struct cdns_i3c_master *master)
    {
    struct i3c_master_controller *m = &master.base;
    unsigned long i3c_lim_period, pres_step, ncycles;
    struct i3c_bus *bus = i3c_master_get_bus(m);
    let mut new_i3c_scl_lim: c_ulong = 0;
    struct i3c_dev_desc *dev;
    u32 prescl1, ctrl;
    i3c_bus_for_each_i3cdev(bus, dev) {
    unsigned long max_fscl;
    max_fscl = max(I3C_CCC_MAX_SDR_FSCL(dev.info.max_read_ds),
    I3C_CCC_MAX_SDR_FSCL(dev.info.max_write_ds));
    switch (max_fscl) {
    case I3C_SDR1_FSCL_8MHZ:
    max_fscl = 8000000;
    break;
    case I3C_SDR2_FSCL_6MHZ:
    max_fscl = 6000000;
    break;
    case I3C_SDR3_FSCL_4MHZ:
    max_fscl = 4000000;
    break;
    case I3C_SDR4_FSCL_2MHZ:
    max_fscl = 2000000;
    break;
    case I3C_SDR0_FSCL_MAX:
    default:
    max_fscl = 0;
    break;
    }
    if (max_fscl &&
    (new_i3c_scl_lim > max_fscl || !new_i3c_scl_lim))
    new_i3c_scl_lim = max_fscl;
    }
// Only update PRESCL_CTRL1 if the I3C SCL limitation has changed.
    if (new_i3c_scl_lim == master.i3c_scl_lim)
    return;
    master.i3c_scl_lim = new_i3c_scl_lim;
    if (!new_i3c_scl_lim)
    return;
    pres_step = 1000000000UL / (bus.scl_rate.i3c * 4);
// Configure PP_LOW to meet I3C slave limitations.
    prescl1 = readl(master.regs + PRESCL_CTRL1) &
    ~PRESCL_CTRL1_PP_LOW_MASK;
    ctrl = readl(master.regs + CTRL);
    i3c_lim_period = DIV_ROUND_UP(1000000000, master.i3c_scl_lim);
    ncycles = DIV_ROUND_UP(i3c_lim_period, pres_step);
    if (ncycles < 4)
    ncycles = 0;
    else
    ncycles -= 4;
    prescl1 |= PRESCL_CTRL1_PP_LOW(ncycles);
// Disable I3C master before updating PRESCL_CTRL1.
    if (ctrl & CTRL_DEV_EN)
    cdns_i3c_master_disable(master);
    writel(prescl1, master.regs + PRESCL_CTRL1);
    if (ctrl & CTRL_DEV_EN)
    cdns_i3c_master_enable(master);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_do_daa(m: *mut i3c_master_controller) -> c_int {
    static int cdns_i3c_master_do_daa(struct i3c_master_controller *m)
    {
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    unsigned long olddevs, newdevs;
    int ret, slot;
    u8 addrs[MAX_DEVS] = { };
    let mut last_addr: u8 = 0;
    olddevs = readl(master.regs + DEVS_CTRL) & DEVS_CTRL_DEVS_ACTIVE_MASK;
    olddevs |= BIT(0);
// Prepare RR slots before launching DAA.
    for_each_clear_bit(slot, &olddevs, master.maxdevs + 1) {
    ret = i3c_master_get_free_addr(m, last_addr + 1);
    if (ret < 0)
    return -ENOSPC;
    last_addr = ret;
    addrs[slot] = last_addr;
    writel(prepare_rr0_dev_address(last_addr) | DEV_ID_RR0_IS_I3C,
    master.regs + DEV_ID_RR0(slot));
    writel(0, master.regs + DEV_ID_RR1(slot));
    writel(0, master.regs + DEV_ID_RR2(slot));
    }
    ret = i3c_master_entdaa_locked(&master.base);
    if (ret)
    return ret;
    newdevs = readl(master.regs + DEVS_CTRL) & DEVS_CTRL_DEVS_ACTIVE_MASK;
    newdevs &= ~olddevs;
//
// Clear all retaining registers filled during DAA. We already
// have the addressed assigned to them in the addrs array.
//
    for_each_set_bit(slot, &newdevs, master.maxdevs + 1)
    i3c_master_add_i3c_dev_locked(m, addrs[slot]);
//
// Clear slots that ended up not being used. Can be caused by I3C
// device creation failure or when the I3C device was already known
// by the system but with a different address (in this case the device
// already has a slot and does not need a new one).
//
    writel(readl(master.regs + DEVS_CTRL) |
    master.free_rr_slots << DEVS_CTRL_DEV_CLR_SHIFT,
    master.regs + DEVS_CTRL);
    i3c_master_defslvs_locked(&master.base);
    cdns_i3c_master_upd_i3c_scl_lim(master);
// Unmask Hot-Join and Mastership request interrupts.
    i3c_master_enec_locked(m, I3C_BROADCAST_ADDR,
    I3C_CCC_EVENT_HJ | I3C_CCC_EVENT_MR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_calculate_thd_delay(master: *mut cdns_i3c_master) -> u8 {
    static u8 cdns_i3c_master_calculate_thd_delay(struct cdns_i3c_master *master)
    {
    let mut sysclk_rate: c_ulong = clk_get_rate(master.sysclk);
    u8 thd_delay = DIV_ROUND_UP(master.devdata.thd_delay_ns,
    (NSEC_PER_SEC / sysclk_rate));
// Every value greater than 3 is not valid.
    if (thd_delay > THD_DELAY_MAX)
    thd_delay = THD_DELAY_MAX;
// CTLR_THD_DEL value is encoded.
    return (THD_DELAY_MAX - thd_delay);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_bus_init(m: *mut i3c_master_controller) -> c_int {
    static int cdns_i3c_master_bus_init(struct i3c_master_controller *m)
    {
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    unsigned long pres_step, sysclk_rate, max_i2cfreq;
    struct i3c_bus *bus = i3c_master_get_bus(m);
    u32 ctrl, prescl0, prescl1, pres, low;
    let mut info: i3c_device_info = { };
    int ret, ncycles;
    switch (bus.mode) {
    case I3C_BUS_MODE_PURE:
    ctrl = CTRL_PURE_BUS_MODE;
    break;
    case I3C_BUS_MODE_MIXED_FAST:
    ctrl = CTRL_MIXED_FAST_BUS_MODE;
    break;
    case I3C_BUS_MODE_MIXED_SLOW:
    ctrl = CTRL_MIXED_SLOW_BUS_MODE;
    break;
    default:
    return -EINVAL;
    }
    sysclk_rate = clk_get_rate(master.sysclk);
    if (!sysclk_rate)
    return -EINVAL;
    pres = DIV_ROUND_UP(sysclk_rate, (bus.scl_rate.i3c * 4)) - 1;
    if (pres > PRESCL_CTRL0_I3C_MAX)
    return -ERANGE;
    bus.scl_rate.i3c = sysclk_rate / ((pres + 1) * 4);
    prescl0 = PRESCL_CTRL0_I3C(pres);
    low = ((I3C_BUS_TLOW_OD_MIN_NS * sysclk_rate) / (pres + 1)) - 2;
    prescl1 = PRESCL_CTRL1_OD_LOW(low);
    max_i2cfreq = bus.scl_rate.i2c;
    pres = (sysclk_rate / (max_i2cfreq * 5)) - 1;
    if (pres > PRESCL_CTRL0_I2C_MAX)
    return -ERANGE;
    bus.scl_rate.i2c = sysclk_rate / ((pres + 1) * 5);
    prescl0 |= PRESCL_CTRL0_I2C(pres);
    writel(prescl0, master.regs + PRESCL_CTRL0);
// Calculate OD and PP low.
    pres_step = 1000000000 / (bus.scl_rate.i3c * 4);
    ncycles = DIV_ROUND_UP(I3C_BUS_TLOW_OD_MIN_NS, pres_step) - 2;
    if (ncycles < 0)
    ncycles = 0;
    prescl1 = PRESCL_CTRL1_OD_LOW(ncycles);
    writel(prescl1, master.regs + PRESCL_CTRL1);
// Get an address for the master.
    ret = i3c_master_get_free_addr(m, 0);
    if (ret < 0)
    return ret;
    writel(prepare_rr0_dev_address(ret) | DEV_ID_RR0_IS_I3C,
    master.regs + DEV_ID_RR0(0));
    cdns_i3c_master_dev_rr_to_info(master, 0, &info);
    if (info.bcr & I3C_BCR_HDR_CAP)
    info.hdr_cap = I3C_CCC_HDR_MODE(I3C_HDR_DDR);
    ret = i3c_master_set_info(&master.base, &info);
    if (ret)
    return ret;
//
// Enable Hot-Join, and, when a Hot-Join request happens, disable all
// events coming from this device.
//
// We will issue ENTDAA afterwards from the threaded IRQ handler.
//
    ctrl |= CTRL_HJ_ACK | CTRL_HJ_DISEC | CTRL_HALT_EN | CTRL_MCS_EN;
//
// Configure data hold delay based on device-specific data.
//
// MIPI I3C Specification 1.0 defines non-zero minimal tHD_PP timing on
// master output. This setting allows to meet this timing on master's
// SoC outputs, regardless of PCB balancing.
//
    ctrl |= CTRL_THD_DELAY(cdns_i3c_master_calculate_thd_delay(master));
    writel(ctrl, master.regs + CTRL);
    cdns_i3c_master_enable(master);
    return 0;
    }
    static void cdns_i3c_master_handle_ibi(struct cdns_i3c_master *master,
    u32 ibir)
    {
    struct cdns_i3c_i2c_dev_data *data;
    let mut data_consumed: bool = false;
    struct i3c_ibi_slot *slot;
    let mut id: u32 = IBIR_SLVID(ibir);
    struct i3c_dev_desc *dev;
    size_t nbytes;
    u8 *buf;
//
// FIXME: maybe we should report the FIFO OVF errors to the upper
// layer.
//
    if (id >= master.ibi.num_slots || (ibir & IBIR_ERROR))
    goto out;
    dev = master.ibi.slots[id];
    spin_lock(&master.ibi.lock);
    data = i3c_dev_get_master_data(dev);
    slot = i3c_generic_ibi_get_free_slot(data.ibi_pool);
    if (!slot)
    goto out_unlock;
    buf = slot.data;
    nbytes = IBIR_XFER_BYTES(ibir);
    i3c_readl_fifo(master.regs + IBI_DATA_FIFO, buf, nbytes);
    slot.len = min_t(unsigned int, IBIR_XFER_BYTES(ibir),
    dev.ibi.max_payload_len);
    i3c_master_queue_ibi(dev, slot);
    data_consumed = true;
    out_unlock:
    spin_unlock(&master.ibi.lock);
    out:
// Consume data from the FIFO if it's not been done already.
    if (!data_consumed) {
    int i;
    for (i = 0; i < IBIR_XFER_BYTES(ibir); i += 4)
    readl(master.regs + IBI_DATA_FIFO);
    }
    }
#[no_mangle]
unsafe extern "C" fn cnds_i3c_master_demux_ibis(master: *mut cdns_i3c_master) {
    static void cnds_i3c_master_demux_ibis(struct cdns_i3c_master *master)
    {
    u32 status0;
    writel(MST_INT_IBIR_THR, master.regs + MST_ICR);
    for (status0 = readl(master.regs + MST_STATUS0);
    !(status0 & MST_STATUS0_IBIR_EMP);
    status0 = readl(master.regs + MST_STATUS0)) {
    let mut ibir: u32 = readl(master.regs + IBIR);
    switch (IBIR_TYPE(ibir)) {
    case IBIR_TYPE_IBI:
    cdns_i3c_master_handle_ibi(master, ibir);
    break;
    case IBIR_TYPE_HJ:
    WARN_ON(IBIR_XFER_BYTES(ibir) || (ibir & IBIR_ERROR));
    i3c_master_queue_hotjoin(&master.base);
    break;
    case IBIR_TYPE_MR:
    WARN_ON(IBIR_XFER_BYTES(ibir) || (ibir & IBIR_ERROR));
    break;
    default:
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_i3c_master_interrupt(int irq, void *data)
    {
    struct cdns_i3c_master *master = data;
    u32 status;
    status = readl(master.regs + MST_ISR);
    if (!(status & readl(master.regs + MST_IMR)))
    return IRQ_NONE;
    spin_lock(&master.xferqueue.lock);
    cdns_i3c_master_end_xfer_locked(master, status);
    spin_unlock(&master.xferqueue.lock);
    if (status & MST_INT_IBIR_THR)
    cnds_i3c_master_demux_ibis(master);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_disable_ibi(dev: *mut i3c_dev_desc) -> c_int {
    static int cdns_i3c_master_disable_ibi(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    unsigned long flags;
    u32 sirmap;
    int ret;
    ret = i3c_master_disec_locked(m, dev.info.dyn_addr,
    I3C_CCC_EVENT_SIR);
    if (ret)
    return ret;
    spin_lock_irqsave(&master.ibi.lock, flags);
    sirmap = readl(master.regs + SIR_MAP_DEV_REG(data.ibi));
    sirmap &= ~SIR_MAP_DEV_CONF_MASK(data.ibi);
    sirmap |= SIR_MAP_DEV_CONF(data.ibi,
    SIR_MAP_DEV_DA(I3C_BROADCAST_ADDR));
    writel(sirmap, master.regs + SIR_MAP_DEV_REG(data.ibi));
    spin_unlock_irqrestore(&master.ibi.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_enable_ibi(dev: *mut i3c_dev_desc) -> c_int {
    static int cdns_i3c_master_enable_ibi(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    unsigned long flags;
    u32 sircfg, sirmap;
    int ret;
    spin_lock_irqsave(&master.ibi.lock, flags);
    sirmap = readl(master.regs + SIR_MAP_DEV_REG(data.ibi));
    sirmap &= ~SIR_MAP_DEV_CONF_MASK(data.ibi);
    sircfg = SIR_MAP_DEV_ROLE(dev.info.bcr >> 6) |
    SIR_MAP_DEV_DA(dev.info.dyn_addr) |
    SIR_MAP_DEV_PL(dev.info.max_ibi_len) |
    SIR_MAP_DEV_ACK;
    if (dev.info.bcr & I3C_BCR_MAX_DATA_SPEED_LIM)
    sircfg |= SIR_MAP_DEV_SLOW;
    sirmap |= SIR_MAP_DEV_CONF(data.ibi, sircfg);
    writel(sirmap, master.regs + SIR_MAP_DEV_REG(data.ibi));
    spin_unlock_irqrestore(&master.ibi.lock, flags);
    ret = i3c_master_enec_locked(m, dev.info.dyn_addr,
    I3C_CCC_EVENT_SIR);
    if (ret) {
    spin_lock_irqsave(&master.ibi.lock, flags);
    sirmap = readl(master.regs + SIR_MAP_DEV_REG(data.ibi));
    sirmap &= ~SIR_MAP_DEV_CONF_MASK(data.ibi);
    sirmap |= SIR_MAP_DEV_CONF(data.ibi,
    SIR_MAP_DEV_DA(I3C_BROADCAST_ADDR));
    writel(sirmap, master.regs + SIR_MAP_DEV_REG(data.ibi));
    spin_unlock_irqrestore(&master.ibi.lock, flags);
    }
    return ret;
    }
    static int cdns_i3c_master_request_ibi(struct i3c_dev_desc *dev,
    const struct i3c_ibi_setup *req)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    unsigned long flags;
    unsigned int i;
    data.ibi_pool = i3c_generic_ibi_alloc_pool(dev, req);
    if (IS_ERR(data.ibi_pool))
    return PTR_ERR(data.ibi_pool);
    spin_lock_irqsave(&master.ibi.lock, flags);
    for (i = 0; i < master.ibi.num_slots; i++) {
    if (!master.ibi.slots[i]) {
    data.ibi = i;
    master.ibi.slots[i] = dev;
    break;
    }
    }
    spin_unlock_irqrestore(&master.ibi.lock, flags);
    if (i < master.ibi.num_slots)
    return 0;
    i3c_generic_ibi_free_pool(data.ibi_pool);
    data.ibi_pool = core::ptr::null_mut();
    return -ENOSPC;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_free_ibi(dev: *mut i3c_dev_desc) {
    static void cdns_i3c_master_free_ibi(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct cdns_i3c_master *master = to_cdns_i3c_master(m);
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    unsigned long flags;
    spin_lock_irqsave(&master.ibi.lock, flags);
    master.ibi.slots[data.ibi] = core::ptr::null_mut();
    data.ibi = -1;
    spin_unlock_irqrestore(&master.ibi.lock, flags);
    i3c_generic_ibi_free_pool(data.ibi_pool);
    }
    static void cdns_i3c_master_recycle_ibi_slot(struct i3c_dev_desc *dev,
    struct i3c_ibi_slot *slot)
    {
    struct cdns_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    i3c_generic_ibi_recycle_slot(data.ibi_pool, slot);
    }
    static const struct i3c_master_controller_ops cdns_i3c_master_ops = {
    .bus_init = cdns_i3c_master_bus_init,
    .bus_cleanup = cdns_i3c_master_bus_cleanup,
    .do_daa = cdns_i3c_master_do_daa,
    .attach_i3c_dev = cdns_i3c_master_attach_i3c_dev,
    .reattach_i3c_dev = cdns_i3c_master_reattach_i3c_dev,
    .detach_i3c_dev = cdns_i3c_master_detach_i3c_dev,
    .attach_i2c_dev = cdns_i3c_master_attach_i2c_dev,
    .detach_i2c_dev = cdns_i3c_master_detach_i2c_dev,
    .supports_ccc_cmd = cdns_i3c_master_supports_ccc_cmd,
    .send_ccc_cmd = cdns_i3c_master_send_ccc_cmd,
    .i3c_xfers = cdns_i3c_master_i3c_xfers,
    .i2c_xfers = cdns_i3c_master_i2c_xfers,
    .enable_ibi = cdns_i3c_master_enable_ibi,
    .disable_ibi = cdns_i3c_master_disable_ibi,
    .request_ibi = cdns_i3c_master_request_ibi,
    .free_ibi = cdns_i3c_master_free_ibi,
    .recycle_ibi_slot = cdns_i3c_master_recycle_ibi_slot,
    };
    static struct cdns_i3c_data cdns_i3c_devdata = {
    .thd_delay_ns = 10,
    };
    static const struct of_device_id cdns_i3c_master_of_ids[] = {
    { .compatible = "cdns,i3c-master", .data = &cdns_i3c_devdata },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, cdns_i3c_master_of_ids);
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_i3c_master_probe(struct platform_device *pdev)
    {
    struct cdns_i3c_master *master;
    struct clk *pclk;
    int ret, irq;
    u32 val;
    master = devm_kzalloc(&pdev.dev, sizeof(*master), GFP_KERNEL);
    if (!master)
    return -ENOMEM;
    master.devdata = of_device_get_match_data(&pdev.dev);
    if (!master.devdata)
    return -EINVAL;
    master.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(master.regs))
    return PTR_ERR(master.regs);
    pclk = devm_clk_get_enabled(&pdev.dev, "pclk");
    if (IS_ERR(pclk))
    return PTR_ERR(pclk);
    master.sysclk = devm_clk_get_enabled(&pdev.dev, "sysclk");
    if (IS_ERR(master.sysclk))
    return PTR_ERR(master.sysclk);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (readl(master.regs + DEV_ID) != DEV_ID_I3C_MASTER)
    return -EINVAL;
    spin_lock_init(&master.xferqueue.lock);
    INIT_LIST_HEAD(&master.xferqueue.list);
    writel(0xffffffff, master.regs + MST_IDR);
    writel(0xffffffff, master.regs + SLV_IDR);
    ret = devm_request_irq(&pdev.dev, irq, cdns_i3c_master_interrupt, 0,
    dev_name(&pdev.dev), master);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, master);
    val = readl(master.regs + CONF_STATUS0);
// Device ID0 is reserved to describe this master.
    master.maxdevs = CONF_STATUS0_DEVS_NUM(val);
    master.free_rr_slots = GENMASK(master.maxdevs, 1);
    master.caps.ibirfifodepth = CONF_STATUS0_IBIR_DEPTH(val);
    master.caps.cmdrfifodepth = CONF_STATUS0_CMDR_DEPTH(val);
    val = readl(master.regs + CONF_STATUS1);
    master.caps.cmdfifodepth = CONF_STATUS1_CMD_DEPTH(val);
    master.caps.rxfifodepth = CONF_STATUS1_RX_DEPTH(val);
    master.caps.txfifodepth = CONF_STATUS1_TX_DEPTH(val);
    spin_lock_init(&master.ibi.lock);
    master.ibi.num_slots = CONF_STATUS1_IBI_HW_RES(val);
    master.ibi.slots = devm_kcalloc(&pdev.dev, master.ibi.num_slots,
    sizeof(*master.ibi.slots),
    GFP_KERNEL);
    if (!master.ibi.slots)
    return -ENOMEM;
    writel(IBIR_THR(1), master.regs + CMD_IBI_THR_CTRL);
    writel(MST_INT_IBIR_THR, master.regs + MST_IER);
    writel(DEVS_CTRL_DEV_CLR_ALL, master.regs + DEVS_CTRL);
    return i3c_master_register(&master.base, &pdev.dev,
    &cdns_i3c_master_ops, false);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i3c_master_remove(pdev: *mut platform_device) {
    static void cdns_i3c_master_remove(struct platform_device *pdev)
    {
    struct cdns_i3c_master *master = platform_get_drvdata(pdev);
    i3c_master_unregister(&master.base);
    }
    static struct platform_driver cdns_i3c_master = {
    .probe = cdns_i3c_master_probe,
    .remove = cdns_i3c_master_remove,
    .driver = {
    .name = "cdns-i3c-master",
    .of_match_table = cdns_i3c_master_of_ids,
    },
    };
    module_platform_driver(cdns_i3c_master);
    MODULE_AUTHOR("Boris Brezillon <boris.brezillon@bootlin.com>");
    MODULE_DESCRIPTION("Cadence I3C master driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:cdns-i3c-master");
