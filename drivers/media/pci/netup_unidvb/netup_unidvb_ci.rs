//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/netup_unidvb/netup_unidvb_ci.c
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
// netup_unidvb_ci.c
//
// DVB CAM support for NetUP Universal Dual DVB-CI
//
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

// CI slot 0 base address
pub const CAM0_CONFIG: c_uint = 0x0;
pub const CAM0_IO: c_uint = 0x8000;
pub const CAM0_MEM: c_uint = 0x10000;
pub const CAM0_SZ: c_int = 32;
// CI slot 1 base address
pub const CAM1_CONFIG: c_uint = 0x20000;
pub const CAM1_IO: c_uint = 0x28000;
pub const CAM1_MEM: c_uint = 0x30000;
pub const CAM1_SZ: c_int = 32;
// ctrlstat registers
pub const CAM_CTRLSTAT_READ_SET: c_uint = 0x4980;
pub const CAM_CTRLSTAT_CLR: c_uint = 0x4982;
// register bits

// BIT_CAM_BYPASS bit shift for SLOT 1
pub const CAM1_SHIFT: c_int = 8;
#[no_mangle]
pub unsafe extern "C" fn netup_ci_interrupt(ndev: *mut netup_unidvb_dev) -> irqreturn_t {
    irqreturn_t netup_ci_interrupt(struct netup_unidvb_dev *ndev)
    {
    writew(0x101, ndev.bmmio0 + CAM_CTRLSTAT_CLR);
    return IRQ_HANDLED;
    }
    static int netup_unidvb_ci_slot_ts_ctl(struct dvb_ca_en50221 *en50221,
    int slot)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    let mut shift: u16 = (state.nr == 1) ? CAM1_SHIFT : 0;
    dev_dbg(&dev.pci_dev.dev, "%s(): CAM_CTRLSTAT=0x%x\n",
    __func__, readw(dev.bmmio0 + CAM_CTRLSTAT_READ_SET));
    if (slot != 0)
    return -EINVAL;
// pass data to CAM module
    writew(BIT_CAM_BYPASS << shift, dev.bmmio0 + CAM_CTRLSTAT_CLR);
    dev_dbg(&dev.pci_dev.dev, "%s(): CAM_CTRLSTAT=0x%x done\n",
    __func__, readw(dev.bmmio0 + CAM_CTRLSTAT_READ_SET));
    return 0;
    }
    static int netup_unidvb_ci_slot_shutdown(struct dvb_ca_en50221 *en50221,
    int slot)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    dev_dbg(&dev.pci_dev.dev, "%s()\n", __func__);
    return 0;
    }
    static int netup_unidvb_ci_slot_reset(struct dvb_ca_en50221 *en50221,
    int slot)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    let mut timeout: c_ulong = 0;
    let mut shift: u16 = (state.nr == 1) ? CAM1_SHIFT : 0;
    let mut ci_stat: u16 = 0;
    let mut reset_counter: c_int = 3;
    dev_dbg(&dev.pci_dev.dev, "%s(): CAM_CTRLSTAT_READ_SET=0x%x\n",
    __func__, readw(dev.bmmio0 + CAM_CTRLSTAT_READ_SET));
    reset:
    timeout = jiffies + msecs_to_jiffies(5000);
// start reset
    writew(BIT_CAM_RESET << shift, dev.bmmio0 + CAM_CTRLSTAT_READ_SET);
    dev_dbg(&dev.pci_dev.dev, "%s(): waiting for reset\n", __func__);
// wait until reset done
    while (time_before(jiffies, timeout)) {
    ci_stat = readw(dev.bmmio0 + CAM_CTRLSTAT_READ_SET);
    if (ci_stat & (BIT_CAM_READY << shift))
    break;
    udelay(1000);
    }
    if (!(ci_stat & (BIT_CAM_READY << shift)) && reset_counter > 0) {
    dev_dbg(&dev.pci_dev.dev,
    "%s(): CAMP reset timeout! Will try again..\n",
    __func__);
    reset_counter--;
    goto reset;
    }
    return 0;
    }
    static int netup_unidvb_poll_ci_slot_status(struct dvb_ca_en50221 *en50221,
    int slot, int open)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    let mut shift: u16 = (state.nr == 1) ? CAM1_SHIFT : 0;
    let mut ci_stat: u16 = 0;
    dev_dbg(&dev.pci_dev.dev, "%s(): CAM_CTRLSTAT_READ_SET=0x%x\n",
    __func__, readw(dev.bmmio0 + CAM_CTRLSTAT_READ_SET));
    ci_stat = readw(dev.bmmio0 + CAM_CTRLSTAT_READ_SET);
    if (ci_stat & (BIT_CAM_READY << shift)) {
    state.status = DVB_CA_EN50221_POLL_CAM_PRESENT |
    DVB_CA_EN50221_POLL_CAM_READY;
    } else if (ci_stat & (BIT_CAM_PRESENT << shift)) {
    state.status = DVB_CA_EN50221_POLL_CAM_PRESENT;
    } else {
    state.status = 0;
    }
    return state.status;
    }
    static int netup_unidvb_ci_read_attribute_mem(struct dvb_ca_en50221 *en50221,
    int slot, int addr)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    let mut val: u8 = *((u8  *)state.membase8_config + addr);
    dev_dbg(&dev.pci_dev.dev,
    "%s(): addr=0x%x val=0x%x\n", __func__, addr, val);
    return val;
    }
    static int netup_unidvb_ci_write_attribute_mem(struct dvb_ca_en50221 *en50221,
    int slot, int addr, u8 data)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    dev_dbg(&dev.pci_dev.dev,
    "%s(): addr=0x%x data=0x%x\n", __func__, addr, data);
// ((u8  *)state->membase8_config + addr) = data;
    return 0;
    }
    static int netup_unidvb_ci_read_cam_ctl(struct dvb_ca_en50221 *en50221,
    int slot, u8 addr)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    let mut val: u8 = *((u8  *)state.membase8_io + addr);
    dev_dbg(&dev.pci_dev.dev,
    "%s(): addr=0x%x val=0x%x\n", __func__, addr, val);
    return val;
    }
    static int netup_unidvb_ci_write_cam_ctl(struct dvb_ca_en50221 *en50221,
    int slot, u8 addr, u8 data)
    {
    struct netup_ci_state *state = en50221.data;
    struct netup_unidvb_dev *dev = state.dev;
    dev_dbg(&dev.pci_dev.dev,
    "%s(): addr=0x%x data=0x%x\n", __func__, addr, data);
// ((u8  *)state->membase8_io + addr) = data;
    return 0;
    }
    int netup_unidvb_ci_register(struct netup_unidvb_dev *dev,
    int num, struct pci_dev *pci_dev)
    {
    int result;
    struct netup_ci_state *state;
    if (num < 0 || num > 1) {
    dev_err(&pci_dev.dev, "%s(): invalid CI adapter %d\n",
    __func__, num);
    return -EINVAL;
    }
    state = &dev.ci[num];
    state.nr = num;
    state.membase8_config = dev.bmmio1 +
    ((num == 0) ? CAM0_CONFIG : CAM1_CONFIG);
    state.membase8_io = dev.bmmio1 +
    ((num == 0) ? CAM0_IO : CAM1_IO);
    state.dev = dev;
    state.ca.owner = THIS_MODULE;
    state.ca.read_attribute_mem = netup_unidvb_ci_read_attribute_mem;
    state.ca.write_attribute_mem = netup_unidvb_ci_write_attribute_mem;
    state.ca.read_cam_control = netup_unidvb_ci_read_cam_ctl;
    state.ca.write_cam_control = netup_unidvb_ci_write_cam_ctl;
    state.ca.slot_reset = netup_unidvb_ci_slot_reset;
    state.ca.slot_shutdown = netup_unidvb_ci_slot_shutdown;
    state.ca.slot_ts_enable = netup_unidvb_ci_slot_ts_ctl;
    state.ca.poll_slot_status = netup_unidvb_poll_ci_slot_status;
    state.ca.data = state;
    result = dvb_ca_en50221_init(&dev.frontends[num].adapter,
    &state.ca, 0, 1);
    if (result < 0) {
    dev_err(&pci_dev.dev,
    "%s(): dvb_ca_en50221_init result %d\n",
    __func__, result);
    return result;
    }
    writew(NETUP_UNIDVB_IRQ_CI, dev.bmmio0 + REG_IMASK_SET);
    dev_info(&pci_dev.dev,
    "%s(): CI adapter %d init done\n", __func__, num);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn netup_unidvb_ci_unregister(dev: *mut netup_unidvb_dev, num: c_int) {
    void netup_unidvb_ci_unregister(struct netup_unidvb_dev *dev, int num)
    {
    struct netup_ci_state *state;
    dev_dbg(&dev.pci_dev.dev, "%s()\n", __func__);
    if (num < 0 || num > 1) {
    dev_err(&dev.pci_dev.dev, "%s(): invalid CI adapter %d\n",
    __func__, num);
    return;
    }
    state = &dev.ci[num];
    dvb_ca_en50221_release(&state.ca);
    }
