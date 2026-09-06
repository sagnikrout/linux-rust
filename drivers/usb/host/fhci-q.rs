//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/fhci-q.c
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

// maps the hardware error code to the USB error code
#[no_mangle]
unsafe extern "C" fn status_to_error(status: u32) -> c_int {
    static int status_to_error(u32 status)
    {
    if (status == USB_TD_OK)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_ER_CRC: status &) -> else {
    else if (status & USB_TD_RX_ER_CRC)
    return -EILSEQ;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_ER_NONOCT: status &) -> else {
    else if (status & USB_TD_RX_ER_NONOCT)
    return -EPROTO;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_ER_OVERUN: status &) -> else {
    else if (status & USB_TD_RX_ER_OVERUN)
    return -ECOMM;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_ER_BITSTUFF: status &) -> else {
    else if (status & USB_TD_RX_ER_BITSTUFF)
    return -EPROTO;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_ER_PID: status &) -> else {
    else if (status & USB_TD_RX_ER_PID)
    return -EILSEQ;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_TX_ER_TIMEOUT): status & (USB_TD_TX_ER_NAK |) -> else {
    else if (status & (USB_TD_TX_ER_NAK | USB_TD_TX_ER_TIMEOUT))
    return -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_TX_ER_STALL: status &) -> else {
    else if (status & USB_TD_TX_ER_STALL)
    return -EPIPE;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_TX_ER_UNDERUN: status &) -> else {
    else if (status & USB_TD_TX_ER_UNDERUN)
    return -ENOSR;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_DATA_UNDERUN: status &) -> else {
    else if (status & USB_TD_RX_DATA_UNDERUN)
    return -EREMOTEIO;
#[no_mangle]
pub unsafe extern "C" fn if(USB_TD_RX_DATA_OVERUN: status &) -> else {
    else if (status & USB_TD_RX_DATA_OVERUN)
    return -EOVERFLOW;
    else
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_add_td_to_frame(frame: *mut fhci_time_frame, td: *mut td) {
    void fhci_add_td_to_frame(struct fhci_time_frame *frame, struct td *td)
    {
    list_add_tail(&td.frame_lh, &frame.tds_list);
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_add_tds_to_ed(ed: *mut ed, td_list: *mut td, number: c_int) {
    void fhci_add_tds_to_ed(struct ed *ed, struct td **td_list, int number)
    {
    int i;
    for (i = 0; i < number; i++) {
    struct td *td = td_list[i];
    list_add_tail(&td.node, &ed.td_list);
    }
    if (ed.td_head == core::ptr::null_mut())
    ed.td_head = td_list[0];
    }
    static struct td *peek_td_from_ed(struct ed *ed)
    {
    struct td *td;
    if (!list_empty(&ed.td_list))
    td = list_entry(ed.td_list.next, struct td, node);
    else
    td = core::ptr::null_mut();
    return td;
    }
    struct td *fhci_remove_td_from_frame(struct fhci_time_frame *frame)
    {
    struct td *td;
    if (!list_empty(&frame.tds_list)) {
    td = list_entry(frame.tds_list.next, struct td, frame_lh);
    list_del_init(frame.tds_list.next);
    } else
    td = core::ptr::null_mut();
    return td;
    }
    struct td *fhci_peek_td_from_frame(struct fhci_time_frame *frame)
    {
    struct td *td;
    if (!list_empty(&frame.tds_list))
    td = list_entry(frame.tds_list.next, struct td, frame_lh);
    else
    td = core::ptr::null_mut();
    return td;
    }
    struct td *fhci_remove_td_from_ed(struct ed *ed)
    {
    struct td *td;
    if (!list_empty(&ed.td_list)) {
    td = list_entry(ed.td_list.next, struct td, node);
    list_del_init(ed.td_list.next);
// if this TD was the ED's head, find next TD
    if (!list_empty(&ed.td_list))
    ed.td_head = list_entry(ed.td_list.next, struct td,
    node);
    else
    ed.td_head = core::ptr::null_mut();
    } else
    td = core::ptr::null_mut();
    return td;
    }
    struct td *fhci_remove_td_from_done_list(struct fhci_controller_list *p_list)
    {
    struct td *td;
    if (!list_empty(&p_list.done_list)) {
    td = list_entry(p_list.done_list.next, struct td, node);
    list_del_init(p_list.done_list.next);
    } else
    td = core::ptr::null_mut();
    return td;
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_move_td_from_ed_to_done_list(usb: *mut fhci_usb, ed: *mut ed) {
    void fhci_move_td_from_ed_to_done_list(struct fhci_usb *usb, struct ed *ed)
    {
    struct td *td;
    td = ed.td_head;
    list_del_init(&td.node);
// If this TD was the ED's head,find next TD
    if (!list_empty(&ed.td_list))
    ed.td_head = list_entry(ed.td_list.next, struct td, node);
    else {
    ed.td_head = core::ptr::null_mut();
    ed.state = FHCI_ED_SKIP;
    }
    ed.toggle_carry = td.toggle;
    list_add_tail(&td.node, &usb.hc_list.done_list);
    if (td.ioc)
    usb.transfer_confirm(usb.fhci);
    }
// free done FHCI URB resource such as ED and TD
#[no_mangle]
unsafe extern "C" fn free_urb_priv(fhci: *mut fhci_hcd, urb: *mut urb) {
    static void free_urb_priv(struct fhci_hcd *fhci, struct urb *urb)
    {
    int i;
    struct urb_priv *urb_priv = urb.hcpriv;
    struct ed *ed = urb_priv.ed;
    for (i = 0; i < urb_priv.num_of_tds; i++) {
    list_del_init(&urb_priv.tds[i].node);
    fhci_recycle_empty_td(fhci, urb_priv.tds[i]);
    }
// if this TD was the ED's head,find the next TD
    if (!list_empty(&ed.td_list))
    ed.td_head = list_entry(ed.td_list.next, struct td, node);
    else
    ed.td_head = core::ptr::null_mut();
    kfree(urb_priv.tds);
    kfree(urb_priv);
    urb.hcpriv = core::ptr::null_mut();
// if this TD was the ED's head,find next TD
    if (ed.td_head == core::ptr::null_mut())
    list_del_init(&ed.node);
    fhci.active_urbs--;
    }
// this routine called to complete and free done URB
#[no_mangle]
pub unsafe extern "C" fn fhci_urb_complete_free(fhci: *mut fhci_hcd, urb: *mut urb) {
    void fhci_urb_complete_free(struct fhci_hcd *fhci, struct urb *urb)
    {
    free_urb_priv(fhci, urb);
    if (urb.status == -EINPROGRESS) {
    if (urb.actual_length != urb.transfer_buffer_length &&
    urb.transfer_flags & URB_SHORT_NOT_OK)
    urb.status = -EREMOTEIO;
    else
    urb.status = 0;
    }
    usb_hcd_unlink_urb_from_ep(fhci_to_hcd(fhci), urb);
    spin_unlock(&fhci.lock);
    usb_hcd_giveback_urb(fhci_to_hcd(fhci), urb, urb.status);
    spin_lock(&fhci.lock);
    }
//
// caculate transfer length/stats and update the urb
// Precondition: irqsafe(only for urb-?status locking)
//
#[no_mangle]
pub unsafe extern "C" fn fhci_done_td(urb: *mut urb, td: *mut td) {
    void fhci_done_td(struct urb *urb, struct td *td)
    {
    struct ed *ed = td.ed;
    let mut cc: u32 = td.status;
// ISO...drivers see per-TD length/status
    if (ed.mode == FHCI_TF_ISO) {
    u32 len;
    if (!(urb.transfer_flags & URB_SHORT_NOT_OK &&
    cc == USB_TD_RX_DATA_UNDERUN))
    cc = USB_TD_OK;
    if (usb_pipeout(urb.pipe))
    len = urb.iso_frame_desc[td.iso_index].length;
    else
    len = td.actual_len;
    urb.actual_length += len;
    urb.iso_frame_desc[td.iso_index].actual_length = len;
    urb.iso_frame_desc[td.iso_index].status =
    status_to_error(cc);
    }
// BULK,INT,CONTROL... drivers see aggregate length/status,
// except that "setup" bytes aren't counted and "short" transfers
// might not be reported as errors.
//
    else {
    if (td.error_cnt >= 3)
    urb.error_count = 3;
// control endpoint only have soft stalls
// update packet status if needed(short may be ok)
    if (!(urb.transfer_flags & URB_SHORT_NOT_OK) &&
    cc == USB_TD_RX_DATA_UNDERUN) {
    ed.state = FHCI_ED_OPER;
    cc = USB_TD_OK;
    }
    if (cc != USB_TD_OK) {
    if (urb.status == -EINPROGRESS)
    urb.status = status_to_error(cc);
    }
// count all non-empty packets except control SETUP packet
    if (td.type != FHCI_TA_SETUP || td.iso_index != 0)
    urb.actual_length += td.actual_len;
    }
    }
// there are some pedning request to unlink
#[no_mangle]
pub unsafe extern "C" fn fhci_del_ed_list(fhci: *mut fhci_hcd, ed: *mut ed) {
    void fhci_del_ed_list(struct fhci_hcd *fhci, struct ed *ed)
    {
    struct td *td = peek_td_from_ed(ed);
    struct urb *urb = td.urb;
    struct urb_priv *urb_priv = urb.hcpriv;
    if (urb_priv.state == URB_DEL) {
    td = fhci_remove_td_from_ed(ed);
// HC may have partly processed this TD
    if (td.status != USB_TD_INPROGRESS)
    fhci_done_td(urb, td);
// URB is done;clean up
    if (++(urb_priv.tds_cnt) == urb_priv.num_of_tds)
    fhci_urb_complete_free(fhci, urb);
    }
    }
