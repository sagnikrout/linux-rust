//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/netup_unidvb/netup_unidvb_spi.c
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
// netup_unidvb_spi.c
//
// Internal SPI driver for NetUP Universal Dual DVB-CI
//
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

pub const NETUP_SPI_CTRL_IRQ: c_uint = 0x1000;
pub const NETUP_SPI_CTRL_IMASK: c_uint = 0x2000;
pub const NETUP_SPI_CTRL_START: c_uint = 0x8000;
pub const NETUP_SPI_CTRL_LAST_CS: c_uint = 0x4000;
pub const NETUP_SPI_TIMEOUT: c_int = 6000;
    enum netup_spi_state {
    SPI_STATE_START,
    SPI_STATE_DONE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_spi_regs {
    pub data: [__u8; 1024],
    pub control_stat: __le16,
    pub clock_divider: __le16,
    pub __aligned(1): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_spi {
    pub dev: *mut device,
    pub ctlr: *mut spi_controller,
    pub regs: *mut netup_spi_regs __iomem,
    pub mmio: *mut u8 __iomem,
    pub lock: spinlock_t,
    pub waitq: wait_queue_head_t,
    pub state: enum netup_spi_state,
}

    static char netup_spi_name[64] = "fpga";
    static struct mtd_partition netup_spi_flash_partitions = {
    .name = netup_spi_name,
    .size = 0x1000000, /* 16MB */
    .offset = 0,
    .mask_flags = MTD_CAP_ROM
    };
    static struct flash_platform_data spi_flash_data = {
    .name = "netup0_m25p128",
    .parts = &netup_spi_flash_partitions,
    .nr_parts = 1,
    };
    static struct spi_board_info netup_spi_board = {
    .modalias = "m25p128",
    .max_speed_hz = 11000000,
    .chip_select = 0,
    .mode = SPI_MODE_0,
    .platform_data = &spi_flash_data,
    };
#[no_mangle]
pub unsafe extern "C" fn netup_spi_interrupt(spi: *mut netup_spi) -> irqreturn_t {
    irqreturn_t netup_spi_interrupt(struct netup_spi *spi)
    {
    u16 reg;
    unsigned long flags;
    if (!spi)
    return IRQ_NONE;
    spin_lock_irqsave(&spi.lock, flags);
    reg = readw(&spi.regs.control_stat);
    if (!(reg & NETUP_SPI_CTRL_IRQ)) {
    spin_unlock_irqrestore(&spi.lock, flags);
    dev_dbg(&spi.ctlr.dev,
    "%s(): not mine interrupt\n", __func__);
    return IRQ_NONE;
    }
    writew(reg | NETUP_SPI_CTRL_IRQ, &spi.regs.control_stat);
    reg = readw(&spi.regs.control_stat);
    writew(reg & ~NETUP_SPI_CTRL_IMASK, &spi.regs.control_stat);
    spi.state = SPI_STATE_DONE;
    wake_up(&spi.waitq);
    spin_unlock_irqrestore(&spi.lock, flags);
    dev_dbg(&spi.ctlr.dev,
    "%s(): SPI interrupt handled\n", __func__);
    return IRQ_HANDLED;
    }
    static int netup_spi_transfer(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct netup_spi *spi = spi_controller_get_devdata(ctlr);
    struct spi_transfer *t;
    let mut result: c_int = 0;
    u32 tr_size;
// reset CS
    writew(NETUP_SPI_CTRL_LAST_CS, &spi.regs.control_stat);
    writew(0, &spi.regs.control_stat);
    list_for_each_entry(t, &msg.transfers, transfer_list) {
    tr_size = t.len;
    while (tr_size) {
    let mut frag_offset: u32 = t.len - tr_size;
    u32 frag_size = (tr_size > sizeof(spi.regs.data)) ?
    sizeof(spi.regs.data) : tr_size;
    let mut frag_last: c_int = 0;
    if (list_is_last(&t.transfer_list,
    &msg.transfers) &&
    frag_offset + frag_size == t.len) {
    frag_last = 1;
    }
    if (t.tx_buf) {
    memcpy_toio(spi.regs.data,
    t.tx_buf + frag_offset,
    frag_size);
    } else {
    memset_io(spi.regs.data,
    0, frag_size);
    }
    spi.state = SPI_STATE_START;
    writew((frag_size & 0x3ff) |
    NETUP_SPI_CTRL_IMASK |
    NETUP_SPI_CTRL_START |
    (frag_last ? NETUP_SPI_CTRL_LAST_CS : 0),
    &spi.regs.control_stat);
    dev_dbg(&spi.ctlr.dev,
    "%s(): control_stat 0x%04x\n",
    __func__, readw(&spi.regs.control_stat));
    wait_event_timeout(spi.waitq,
    spi.state != SPI_STATE_START,
    msecs_to_jiffies(NETUP_SPI_TIMEOUT));
    if (spi.state == SPI_STATE_DONE) {
    if (t.rx_buf) {
    memcpy_fromio(t.rx_buf + frag_offset,
    spi.regs.data, frag_size);
    }
    } else {
    if (spi.state == SPI_STATE_START) {
    dev_dbg(&spi.ctlr.dev,
    "%s(): transfer timeout\n",
    __func__);
    } else {
    dev_dbg(&spi.ctlr.dev,
    "%s(): invalid state %d\n",
    __func__, spi.state);
    }
    result = -EIO;
    goto done;
    }
    tr_size -= frag_size;
    msg.actual_length += frag_size;
    }
    }
    done:
    msg.status = result;
    spi_finalize_current_message(ctlr);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn netup_spi_setup(spi: *mut spi_device) -> c_int {
    static int netup_spi_setup(struct spi_device *spi)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn netup_spi_init(ndev: *mut netup_unidvb_dev) -> c_int {
    int netup_spi_init(struct netup_unidvb_dev *ndev)
    {
    struct spi_controller *ctlr;
    struct netup_spi *nspi;
    ctlr = devm_spi_alloc_host(&ndev.pci_dev.dev,
    sizeof(struct netup_spi));
    if (!ctlr) {
    dev_err(&ndev.pci_dev.dev,
    "%s(): unable to alloc SPI host\n", __func__);
    return -EINVAL;
    }
    nspi = spi_controller_get_devdata(ctlr);
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LSB_FIRST;
    ctlr.bus_num = -1;
    ctlr.num_chipselect = 1;
    ctlr.transfer_one_message = netup_spi_transfer;
    ctlr.setup = netup_spi_setup;
    spin_lock_init(&nspi.lock);
    init_waitqueue_head(&nspi.waitq);
    nspi.ctlr = ctlr;
    nspi.regs = (struct netup_spi_regs __iomem *)(ndev.bmmio0 + 0x4000);
    writew(2, &nspi.regs.clock_divider);
    writew(NETUP_UNIDVB_IRQ_SPI, ndev.bmmio0 + REG_IMASK_SET);
    ndev.spi = nspi;
    if (spi_register_controller(ctlr)) {
    ndev.spi = core::ptr::null_mut();
    dev_err(&ndev.pci_dev.dev,
    "%s(): unable to register SPI bus\n", __func__);
    return -EINVAL;
    }
    snprintf(netup_spi_name,
    sizeof(netup_spi_name),
    "fpga_%02x:%02x.%01x",
    ndev.pci_bus,
    ndev.pci_slot,
    ndev.pci_func);
    if (!spi_new_device(ctlr, &netup_spi_board)) {
    spi_unregister_controller(ctlr);
    ndev.spi = core::ptr::null_mut();
    dev_err(&ndev.pci_dev.dev,
    "%s(): unable to create SPI device\n", __func__);
    return -EINVAL;
    }
    dev_dbg(&ndev.pci_dev.dev, "%s(): SPI init OK\n", __func__);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn netup_spi_release(ndev: *mut netup_unidvb_dev) {
    void netup_spi_release(struct netup_unidvb_dev *ndev)
    {
    u16 reg;
    unsigned long flags;
    struct netup_spi *spi = ndev.spi;
    if (!spi)
    return;
    spi_unregister_controller(spi.ctlr);
    spin_lock_irqsave(&spi.lock, flags);
    reg = readw(&spi.regs.control_stat);
    writew(reg | NETUP_SPI_CTRL_IRQ, &spi.regs.control_stat);
    reg = readw(&spi.regs.control_stat);
    writew(reg & ~NETUP_SPI_CTRL_IMASK, &spi.regs.control_stat);
    spin_unlock_irqrestore(&spi.lock, flags);
    ndev.spi = core::ptr::null_mut();
    }
