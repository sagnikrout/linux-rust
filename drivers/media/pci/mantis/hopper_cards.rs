//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/mantis/hopper_cards.c
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
    Hopper PCI bridge driver
    Copyright (C) Manu Abraham (abraham.manu@gmail.com)
//

    static unsigned int verbose;
    module_param(verbose, int, 0644);
    MODULE_PARM_DESC(verbose, "verbose startup messages, default is 0 (no)");

    static char *label[10] = {
    "DMA",
    "IRQ-0",
    "IRQ-1",
    "OCERR",
    "PABRT",
    "RIPRR",
    "PPERR",
    "FTRGT",
    "RISCI",
    "RACK"
    };
    static int devs;
#[no_mangle]
unsafe extern "C" fn hopper_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hopper_irq_handler(int irq, void *dev_id)
    {
    let mut stat: u32 = 0, mask = 0;
    let mut rst_stat: u32 = 0, rst_mask = 0;
    struct mantis_pci *mantis;
    struct mantis_ca *ca;
    mantis = (struct mantis_pci *) dev_id;
    if (unlikely(!mantis))
    return IRQ_NONE;
    ca = mantis.mantis_ca;
    stat = mmread(MANTIS_INT_STAT);
    mask = mmread(MANTIS_INT_MASK);
    if (!(stat & mask))
    return IRQ_NONE;
    rst_mask  = MANTIS_GPIF_WRACK  |
    MANTIS_GPIF_OTHERR |
    MANTIS_SBUF_WSTO   |
    MANTIS_GPIF_EXTIRQ;
    rst_stat  = mmread(MANTIS_GPIF_STATUS);
    rst_stat &= rst_mask;
    mmwrite(rst_stat, MANTIS_GPIF_STATUS);
    mantis.mantis_int_stat = stat;
    mantis.mantis_int_mask = mask;
    dprintk(MANTIS_DEBUG, 0, "\n-- Stat=<%02x> Mask=<%02x> --", stat, mask);
    if (stat & MANTIS_INT_RISCEN) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[0]);
    }
    if (stat & MANTIS_INT_IRQ0) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[1]);
    mantis.gpif_status = rst_stat;
    wake_up(&ca.hif_write_wq);
    schedule_work(&ca.hif_evm_work);
    }
    if (stat & MANTIS_INT_IRQ1) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[2]);
    spin_lock(&mantis.intmask_lock);
    mmwrite(mmread(MANTIS_INT_MASK) & ~MANTIS_INT_IRQ1,
    MANTIS_INT_MASK);
    spin_unlock(&mantis.intmask_lock);
    schedule_work(&mantis.uart_work);
    }
    if (stat & MANTIS_INT_OCERR) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[3]);
    }
    if (stat & MANTIS_INT_PABORT) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[4]);
    }
    if (stat & MANTIS_INT_RIPERR) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[5]);
    }
    if (stat & MANTIS_INT_PPERR) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[6]);
    }
    if (stat & MANTIS_INT_FTRGT) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[7]);
    }
    if (stat & MANTIS_INT_RISCI) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[8]);
    mantis.busy_block = (stat & MANTIS_INT_RISCSTAT) >> 28;
    queue_work(system_bh_wq, &mantis.bh_work);
    }
    if (stat & MANTIS_INT_I2CDONE) {
    dprintk(MANTIS_DEBUG, 0, "<%s>", label[9]);
    wake_up(&mantis.i2c_wq);
    }
    mmwrite(stat, MANTIS_INT_STAT);
    stat &= ~(MANTIS_INT_RISCEN   | MANTIS_INT_I2CDONE |
    MANTIS_INT_I2CRACK  | MANTIS_INT_PCMCIA7 |
    MANTIS_INT_PCMCIA6  | MANTIS_INT_PCMCIA5 |
    MANTIS_INT_PCMCIA4  | MANTIS_INT_PCMCIA3 |
    MANTIS_INT_PCMCIA2  | MANTIS_INT_PCMCIA1 |
    MANTIS_INT_PCMCIA0  | MANTIS_INT_IRQ1	   |
    MANTIS_INT_IRQ0     | MANTIS_INT_OCERR   |
    MANTIS_INT_PABORT   | MANTIS_INT_RIPERR  |
    MANTIS_INT_PPERR    | MANTIS_INT_FTRGT   |
    MANTIS_INT_RISCI);
    if (stat)
    dprintk(MANTIS_DEBUG, 0, "<Unknown> Stat=<%02x> Mask=<%02x>", stat, mask);
    dprintk(MANTIS_DEBUG, 0, "\n");
    return IRQ_HANDLED;
    }
    static int hopper_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *pci_id)
    {
    struct mantis_pci_drvdata *drvdata;
    struct mantis_pci *mantis;
    struct mantis_hwconfig *config;
    int err;
    mantis = kzalloc_obj(*mantis);
    if (!mantis) {
    err = -ENOMEM;
    goto fail0;
    }
    drvdata			= (void *)pci_id.driver_data;
    mantis.num		= devs;
    mantis.verbose		= verbose;
    mantis.pdev		= pdev;
    config			= drvdata.hwconfig;
    config.irq_handler	= &hopper_irq_handler;
    mantis.hwconfig	= config;
    mantis.rc_map_name	= drvdata.rc_map_name;
    spin_lock_init(&mantis.intmask_lock);
    err = mantis_pci_init(mantis);
    if (err) {
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis PCI initialization failed <%d>", err);
    goto fail1;
    }
    err = mantis_stream_control(mantis, STREAM_TO_HIF);
    if (err < 0) {
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis stream control failed <%d>", err);
    goto fail1;
    }
    err = mantis_i2c_init(mantis);
    if (err < 0) {
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis I2C initialization failed <%d>", err);
    goto fail2;
    }
    err = mantis_get_mac(mantis);
    if (err < 0) {
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis MAC address read failed <%d>", err);
    goto fail2;
    }
    err = mantis_dma_init(mantis);
    if (err < 0) {
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis DMA initialization failed <%d>", err);
    goto fail3;
    }
    err = mantis_dvb_init(mantis);
    if (err < 0) {
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis DVB initialization failed <%d>", err);
    goto fail4;
    }
    devs++;
    return err;
    fail4:
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis DMA exit! <%d>", err);
    mantis_dma_exit(mantis);
    fail3:
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis I2C exit! <%d>", err);
    mantis_i2c_exit(mantis);
    fail2:
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis PCI exit! <%d>", err);
    mantis_pci_exit(mantis);
    fail1:
    dprintk(MANTIS_ERROR, 1, "ERROR: Mantis free! <%d>", err);
    kfree(mantis);
    fail0:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hopper_pci_remove(pdev: *mut pci_dev) {
    static void hopper_pci_remove(struct pci_dev *pdev)
    {
    struct mantis_pci *mantis = pci_get_drvdata(pdev);
    if (mantis) {
    mantis_dvb_exit(mantis);
    mantis_dma_exit(mantis);
    mantis_i2c_exit(mantis);
    mantis_pci_exit(mantis);
    kfree(mantis);
    }
    return;
    }
    static const struct pci_device_id hopper_pci_table[] = {
    MAKE_ENTRY(TWINHAN_TECHNOLOGIES, MANTIS_VP_3028_DVB_T, &vp3028_config,
    core::ptr::null_mut()),
    { }
    };
    MODULE_DEVICE_TABLE(pci, hopper_pci_table);
    static struct pci_driver hopper_pci_driver = {
    .name		= DRIVER_NAME,
    .id_table	= hopper_pci_table,
    .probe		= hopper_pci_probe,
    .remove		= hopper_pci_remove,
    };
    module_pci_driver(hopper_pci_driver);
    MODULE_DESCRIPTION("HOPPER driver");
    MODULE_AUTHOR("Manu Abraham");
    MODULE_LICENSE("GPL");
