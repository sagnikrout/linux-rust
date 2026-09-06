//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/fhci-dbg.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Freescale QUICC Engine USB Host Controller Driver
//
// Copyright (c) Freescale Semicondutor, Inc. 2006.
// Shlomi Gridish <gridish@freescale.com>
// Jerry Huang <Chang-Ming.Huang@freescale.com>
// Copyright (c) Logic Product Development, Inc. 2007
// Peter Barada <peterb@logicpd.com>
// Copyright (c) MontaVista Software, Inc. 2008.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[no_mangle]
pub unsafe extern "C" fn fhci_dbg_isr(fhci: *mut fhci_hcd, usb_er: c_int) {
    void fhci_dbg_isr(struct fhci_hcd *fhci, int usb_er)
    {
    int i;
    if (usb_er == -1) {
    fhci.usb_irq_stat[12]++;
    return;
    }
    for (i = 0; i < 12; ++i) {
    if (usb_er & (1 << i))
    fhci.usb_irq_stat[i]++;
    }
    }
#[no_mangle]
unsafe extern "C" fn fhci_dfs_regs_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int fhci_dfs_regs_show(struct seq_file *s, void *v)
    {
    struct fhci_hcd *fhci = s.private;
    struct qe_usb_ctlr __iomem *regs = fhci.regs;
    seq_printf(s,
    "mode: 0x%x\n" "addr: 0x%x\n"
    "command: 0x%x\n" "ep0: 0x%x\n"
    "event: 0x%x\n" "mask: 0x%x\n"
    "status: 0x%x\n" "SOF timer: %d\n"
    "frame number: %d\n"
    "lines status: 0x%x\n",
    in_8(&regs.usb_usmod), in_8(&regs.usb_usadr),
    in_8(&regs.usb_uscom), in_be16(&regs.usb_usep[0]),
    in_be16(&regs.usb_usber), in_be16(&regs.usb_usbmr),
    in_8(&regs.usb_usbs), in_be16(&regs.usb_ussft),
    in_be16(&regs.usb_usfrn),
    fhci_ioports_check_bus_state(fhci));
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(fhci_dfs_regs);
#[no_mangle]
unsafe extern "C" fn fhci_dfs_irq_stat_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int fhci_dfs_irq_stat_show(struct seq_file *s, void *v)
    {
    struct fhci_hcd *fhci = s.private;
    int *usb_irq_stat = fhci.usb_irq_stat;
    seq_printf(s,
    "RXB: %d\n" "TXB: %d\n" "BSY: %d\n"
    "SOF: %d\n" "TXE0: %d\n" "TXE1: %d\n"
    "TXE2: %d\n" "TXE3: %d\n" "IDLE: %d\n"
    "RESET: %d\n" "SFT: %d\n" "MSF: %d\n"
    "IDLE_ONLY: %d\n",
    usb_irq_stat[0], usb_irq_stat[1], usb_irq_stat[2],
    usb_irq_stat[3], usb_irq_stat[4], usb_irq_stat[5],
    usb_irq_stat[6], usb_irq_stat[7], usb_irq_stat[8],
    usb_irq_stat[9], usb_irq_stat[10], usb_irq_stat[11],
    usb_irq_stat[12]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(fhci_dfs_irq_stat);
#[no_mangle]
pub unsafe extern "C" fn fhci_dfs_create(fhci: *mut fhci_hcd) {
    void fhci_dfs_create(struct fhci_hcd *fhci)
    {
    struct device *dev = fhci_to_hcd(fhci).self.controller;
    fhci.dfs_root = debugfs_create_dir(dev_name(dev), usb_debug_root);
    debugfs_create_file("regs", S_IFREG | S_IRUGO, fhci.dfs_root, fhci,
    &fhci_dfs_regs_fops);
    debugfs_create_file("irq_stat", S_IFREG | S_IRUGO, fhci.dfs_root, fhci,
    &fhci_dfs_irq_stat_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_dfs_destroy(fhci: *mut fhci_hcd) {
    void fhci_dfs_destroy(struct fhci_hcd *fhci)
    {
    debugfs_remove_recursive(fhci.dfs_root);
    }
