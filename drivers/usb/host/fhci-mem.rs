//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/fhci-mem.c
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
unsafe extern "C" fn init_td(td: *mut td) {
    static void init_td(struct td *td)
    {
    memset(td, 0, sizeof(*td));
    INIT_LIST_HEAD(&td.node);
    INIT_LIST_HEAD(&td.frame_lh);
    }
#[no_mangle]
unsafe extern "C" fn init_ed(ed: *mut ed) {
    static void init_ed(struct ed *ed)
    {
    memset(ed, 0, sizeof(*ed));
    INIT_LIST_HEAD(&ed.td_list);
    INIT_LIST_HEAD(&ed.node);
    }
    static struct td *get_empty_td(struct fhci_hcd *fhci)
    {
    struct td *td;
    if (!list_empty(&fhci.empty_tds)) {
    td = list_entry(fhci.empty_tds.next, struct td, node);
    list_del(fhci.empty_tds.next);
    } else {
    td = kmalloc_obj(*td, GFP_ATOMIC);
    if (!td)
    fhci_err(fhci, "No memory to allocate to TD\n");
    else
    init_td(td);
    }
    return td;
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_recycle_empty_td(fhci: *mut fhci_hcd, td: *mut td) {
    void fhci_recycle_empty_td(struct fhci_hcd *fhci, struct td *td)
    {
    init_td(td);
    list_add(&td.node, &fhci.empty_tds);
    }
    struct ed *fhci_get_empty_ed(struct fhci_hcd *fhci)
    {
    struct ed *ed;
    if (!list_empty(&fhci.empty_eds)) {
    ed = list_entry(fhci.empty_eds.next, struct ed, node);
    list_del(fhci.empty_eds.next);
    } else {
    ed = kmalloc_obj(*ed, GFP_ATOMIC);
    if (!ed)
    fhci_err(fhci, "No memory to allocate to ED\n");
    else
    init_ed(ed);
    }
    return ed;
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_recycle_empty_ed(fhci: *mut fhci_hcd, ed: *mut ed) {
    void fhci_recycle_empty_ed(struct fhci_hcd *fhci, struct ed *ed)
    {
    init_ed(ed);
    list_add(&ed.node, &fhci.empty_eds);
    }
    struct td *fhci_td_fill(struct fhci_hcd *fhci, struct urb *urb,
    struct urb_priv *urb_priv, struct ed *ed, u16 index,
    enum fhci_ta_type type, int toggle, u8 *data, u32 len,
    u16 interval, u16 start_frame, bool ioc)
    {
    struct td *td = get_empty_td(fhci);
    if (!td)
    return core::ptr::null_mut();
    td.urb = urb;
    td.ed = ed;
    td.type = type;
    td.toggle = toggle;
    td.data = data;
    td.len = len;
    td.iso_index = index;
    td.interval = interval;
    td.start_frame = start_frame;
    td.ioc = ioc;
    td.status = USB_TD_OK;
    urb_priv.tds[index] = td;
    return td;
    }
