//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/mantis/mantis_pcmcia.c
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

//
// If Slot state is already PLUG_IN event and we are called
// again, definitely it is jitter alone
//
#[no_mangle]
pub unsafe extern "C" fn mantis_event_cam_plugin(ca: *mut mantis_ca) {
    void mantis_event_cam_plugin(struct mantis_ca *ca)
    {
    struct mantis_pci *mantis = ca.ca_priv;
    u32 gpif_irqcfg;
    if (ca.slot_state == MODULE_XTRACTED) {
    dprintk(MANTIS_DEBUG, 1, "Event: CAM Plugged IN: Adapter(%d) Slot(0)", mantis.num);
    udelay(50);
    mmwrite(0xda000000, MANTIS_CARD_RESET);
    gpif_irqcfg  = mmread(MANTIS_GPIF_IRQCFG);
    gpif_irqcfg |= MANTIS_MASK_PLUGOUT;
    gpif_irqcfg &= ~MANTIS_MASK_PLUGIN;
    mmwrite(gpif_irqcfg, MANTIS_GPIF_IRQCFG);
    udelay(500);
    ca.slot_state = MODULE_INSERTED;
    }
    udelay(100);
    }
//
// If Slot state is already UN_PLUG event and we are called
// again, definitely it is jitter alone
//
#[no_mangle]
pub unsafe extern "C" fn mantis_event_cam_unplug(ca: *mut mantis_ca) {
    void mantis_event_cam_unplug(struct mantis_ca *ca)
    {
    struct mantis_pci *mantis = ca.ca_priv;
    u32 gpif_irqcfg;
    if (ca.slot_state == MODULE_INSERTED) {
    dprintk(MANTIS_DEBUG, 1, "Event: CAM Unplugged: Adapter(%d) Slot(0)", mantis.num);
    udelay(50);
    mmwrite(0x00da0000, MANTIS_CARD_RESET);
    gpif_irqcfg  = mmread(MANTIS_GPIF_IRQCFG);
    gpif_irqcfg |= MANTIS_MASK_PLUGIN;
    gpif_irqcfg &= ~MANTIS_MASK_PLUGOUT;
    mmwrite(gpif_irqcfg, MANTIS_GPIF_IRQCFG);
    udelay(500);
    ca.slot_state = MODULE_XTRACTED;
    }
    udelay(100);
    }
#[no_mangle]
pub unsafe extern "C" fn mantis_pcmcia_init(ca: *mut mantis_ca) -> c_int {
    int mantis_pcmcia_init(struct mantis_ca *ca)
    {
    struct mantis_pci *mantis = ca.ca_priv;
    u32 gpif_stat, card_stat;
    mantis_unmask_ints(mantis, MANTIS_INT_IRQ0);
    gpif_stat = mmread(MANTIS_GPIF_STATUS);
    card_stat = mmread(MANTIS_GPIF_IRQCFG);
    if (gpif_stat & MANTIS_GPIF_DETSTAT) {
    dprintk(MANTIS_DEBUG, 1, "CAM found on Adapter(%d) Slot(0)", mantis.num);
    mmwrite(card_stat | MANTIS_MASK_PLUGOUT, MANTIS_GPIF_IRQCFG);
    ca.slot_state = MODULE_INSERTED;
    dvb_ca_en50221_camchange_irq(&ca.en50221,
    0,
    DVB_CA_EN50221_CAMCHANGE_INSERTED);
    } else {
    dprintk(MANTIS_DEBUG, 1, "Empty Slot on Adapter(%d) Slot(0)", mantis.num);
    mmwrite(card_stat | MANTIS_MASK_PLUGIN, MANTIS_GPIF_IRQCFG);
    ca.slot_state = MODULE_XTRACTED;
    dvb_ca_en50221_camchange_irq(&ca.en50221,
    0,
    DVB_CA_EN50221_CAMCHANGE_REMOVED);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mantis_pcmcia_exit(ca: *mut mantis_ca) {
    void mantis_pcmcia_exit(struct mantis_ca *ca)
    {
    struct mantis_pci *mantis = ca.ca_priv;
    mmwrite(mmread(MANTIS_GPIF_STATUS) & (~MANTIS_CARD_PLUGOUT | ~MANTIS_CARD_PLUGIN), MANTIS_GPIF_STATUS);
    mantis_mask_ints(mantis, MANTIS_INT_IRQ0);
    }
