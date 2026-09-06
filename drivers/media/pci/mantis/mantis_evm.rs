//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/mantis/mantis_evm.c
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
    Mantis PCI bridge driver
    Copyright (C) Manu Abraham (abraham.manu@gmail.com)
//

#[no_mangle]
unsafe extern "C" fn mantis_hifevm_work(work: *mut work_struct) {
    static void mantis_hifevm_work(struct work_struct *work)
    {
    struct mantis_ca *ca = container_of(work, struct mantis_ca, hif_evm_work);
    struct mantis_pci *mantis = ca.ca_priv;
    u32 gpif_stat;
    gpif_stat = mmread(MANTIS_GPIF_STATUS);
    if (gpif_stat & MANTIS_GPIF_DETSTAT) {
    if (gpif_stat & MANTIS_CARD_PLUGIN) {
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): CAM Plugin", mantis.num);
    mmwrite(0xdada0000, MANTIS_CARD_RESET);
    mantis_event_cam_plugin(ca);
    dvb_ca_en50221_camchange_irq(&ca.en50221,
    0,
    DVB_CA_EN50221_CAMCHANGE_INSERTED);
    }
    } else {
    if (gpif_stat & MANTIS_CARD_PLUGOUT) {
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): CAM Unplug", mantis.num);
    mmwrite(0xdada0000, MANTIS_CARD_RESET);
    mantis_event_cam_unplug(ca);
    dvb_ca_en50221_camchange_irq(&ca.en50221,
    0,
    DVB_CA_EN50221_CAMCHANGE_REMOVED);
    }
    }
    if (mantis.gpif_status & MANTIS_GPIF_EXTIRQ)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Ext IRQ", mantis.num);
    if (mantis.gpif_status & MANTIS_SBUF_WSTO)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Smart Buffer Timeout", mantis.num);
    if (mantis.gpif_status & MANTIS_GPIF_OTHERR)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Alignment Error", mantis.num);
    if (gpif_stat & MANTIS_SBUF_OVFLW)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Smart Buffer Overflow", mantis.num);
    if (gpif_stat & MANTIS_GPIF_BRRDY)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Smart Buffer Read Ready", mantis.num);
    if (gpif_stat & MANTIS_GPIF_INTSTAT)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): GPIF IRQ", mantis.num);
    if (gpif_stat & MANTIS_SBUF_EMPTY)
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Smart Buffer Empty", mantis.num);
    if (gpif_stat & MANTIS_SBUF_OPDONE) {
    dprintk(MANTIS_DEBUG, 1, "Event Mgr: Adapter(%d) Slot(0): Smart Buffer operation complete", mantis.num);
    ca.sbuf_status = MANTIS_SBUF_DATA_AVAIL;
    ca.hif_event = MANTIS_SBUF_OPDONE;
    wake_up(&ca.hif_opdone_wq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mantis_evmgr_init(ca: *mut mantis_ca) -> c_int {
    int mantis_evmgr_init(struct mantis_ca *ca)
    {
    struct mantis_pci *mantis = ca.ca_priv;
    dprintk(MANTIS_DEBUG, 1, "Initializing Mantis Host I/F Event manager");
    INIT_WORK(&ca.hif_evm_work, mantis_hifevm_work);
    mantis_pcmcia_init(ca);
    schedule_work(&ca.hif_evm_work);
    mantis_hif_init(ca);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mantis_evmgr_exit(ca: *mut mantis_ca) {
    void mantis_evmgr_exit(struct mantis_ca *ca)
    {
    struct mantis_pci *mantis = ca.ca_priv;
    dprintk(MANTIS_DEBUG, 1, "Mantis Host I/F Event manager exiting");
    flush_work(&ca.hif_evm_work);
    mantis_hif_exit(ca);
    mantis_pcmcia_exit(ca);
    }
