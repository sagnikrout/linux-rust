//! Automatically rewritten from C to Rust
//! Source: drivers/usb/usbip/vudc_tx.c
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
// Copyright (C) 2015 Karol Kosik <karo9@interia.eu>
// Copyright (C) 2015-2016 Samsung Electronics
// Igor Kotrasinski <i.kotrasinsk@samsung.com>
//

    static inline void setup_base_pdu(struct usbip_header_basic *base,
    __u32 command, __u32 seqnum)
    {
    base.command	= command;
    base.seqnum	= seqnum;
    base.devid	= 0;
    base.ep	= 0;
    base.direction = 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_ret_submit_pdu(rpdu: *mut usbip_header, urb_p: *mut urbp) {
    static void setup_ret_submit_pdu(struct usbip_header *rpdu, struct urbp *urb_p)
    {
    setup_base_pdu(&rpdu.base, USBIP_RET_SUBMIT, urb_p.seqnum);
    usbip_pack_pdu(rpdu, urb_p.urb, USBIP_RET_SUBMIT, 1);
    }
    static void setup_ret_unlink_pdu(struct usbip_header *rpdu,
    struct v_unlink *unlink)
    {
    setup_base_pdu(&rpdu.base, USBIP_RET_UNLINK, unlink.seqnum);
    rpdu.u.ret_unlink.status = unlink.status;
    }
#[no_mangle]
unsafe extern "C" fn v_send_ret_unlink(udc: *mut vudc, unlink: *mut v_unlink) -> c_int {
    static int v_send_ret_unlink(struct vudc *udc, struct v_unlink *unlink)
    {
    struct msghdr msg;
    struct kvec iov[1];
    size_t txsize;
    int ret;
    struct usbip_header pdu_header;
    txsize = 0;
    memset(&pdu_header, 0, sizeof(pdu_header));
    memset(&msg, 0, sizeof(msg));
    memset(&iov, 0, sizeof(iov));
// 1. setup usbip_header
    setup_ret_unlink_pdu(&pdu_header, unlink);
    usbip_header_correct_endian(&pdu_header, 1);
    iov[0].iov_base = &pdu_header;
    iov[0].iov_len  = sizeof(pdu_header);
    txsize += sizeof(pdu_header);
    ret = kernel_sendmsg(udc.ud.tcp_socket, &msg, iov,
    1, txsize);
    if (ret != txsize) {
    usbip_event_add(&udc.ud, VUDC_EVENT_ERROR_TCP);
    if (ret >= 0)
    return -EPIPE;
    return ret;
    }
    kfree(unlink);
    return txsize;
    }
#[no_mangle]
unsafe extern "C" fn v_send_ret_submit(udc: *mut vudc, urb_p: *mut urbp) -> c_int {
    static int v_send_ret_submit(struct vudc *udc, struct urbp *urb_p)
    {
    struct urb *urb = urb_p.urb;
    struct usbip_header pdu_header;
    struct usbip_iso_packet_descriptor *iso_buffer = core::ptr::null_mut();
    struct kvec *iov = core::ptr::null_mut();
    let mut iovnum: c_int = 0;
    let mut ret: c_int = 0;
    size_t txsize;
    struct msghdr msg;
    txsize = 0;
    memset(&pdu_header, 0, sizeof(pdu_header));
    memset(&msg, 0, sizeof(msg));
    if (urb.actual_length > 0 && !urb.transfer_buffer) {
    dev_err(&udc.gadget.dev,
    "urb: actual_length %d transfer_buffer null\n",
    urb.actual_length);
    return -1;
    }
    if (urb_p.type == USB_ENDPOINT_XFER_ISOC)
    iovnum = 2 + urb.number_of_packets;
    else
    iovnum = 2;
    iov = kzalloc_objs(*iov, iovnum);
    if (!iov) {
    usbip_event_add(&udc.ud, VUDC_EVENT_ERROR_MALLOC);
    ret = -ENOMEM;
    goto out;
    }
    iovnum = 0;
// 1. setup usbip_header
    setup_ret_submit_pdu(&pdu_header, urb_p);
    usbip_dbg_stub_tx("setup txdata seqnum: %u\n",
    pdu_header.base.seqnum);
    usbip_header_correct_endian(&pdu_header, 1);
    iov[iovnum].iov_base = &pdu_header;
    iov[iovnum].iov_len  = sizeof(pdu_header);
    iovnum++;
    txsize += sizeof(pdu_header);
// 2. setup transfer buffer
    if (urb_p.type != USB_ENDPOINT_XFER_ISOC &&
    usb_pipein(urb.pipe) && urb.actual_length > 0) {
    iov[iovnum].iov_base = urb.transfer_buffer;
    iov[iovnum].iov_len  = urb.actual_length;
    iovnum++;
    txsize += urb.actual_length;
    } else if (urb_p.type == USB_ENDPOINT_XFER_ISOC &&
    usb_pipein(urb.pipe)) {
// FIXME - copypasted from stub_tx, refactor
    int i;
    for (i = 0; i < urb.number_of_packets; i++) {
    iov[iovnum].iov_base = urb.transfer_buffer +
    urb.iso_frame_desc[i].offset;
    iov[iovnum].iov_len =
    urb.iso_frame_desc[i].actual_length;
    iovnum++;
    txsize += urb.iso_frame_desc[i].actual_length;
    }
    if (txsize != sizeof(pdu_header) + urb.actual_length) {
    usbip_event_add(&udc.ud, VUDC_EVENT_ERROR_TCP);
    ret = -EPIPE;
    goto out;
    }
    }
// else - no buffer to send
// 3. setup iso_packet_descriptor
    if (urb_p.type == USB_ENDPOINT_XFER_ISOC) {
    let mut len: isize = 0;
    iso_buffer = usbip_alloc_iso_desc_pdu(urb, &len);
    if (!iso_buffer) {
    usbip_event_add(&udc.ud,
    VUDC_EVENT_ERROR_MALLOC);
    ret = -ENOMEM;
    goto out;
    }
    iov[iovnum].iov_base = iso_buffer;
    iov[iovnum].iov_len  = len;
    txsize += len;
    iovnum++;
    }
    ret = kernel_sendmsg(udc.ud.tcp_socket, &msg,
    iov,  iovnum, txsize);
    if (ret != txsize) {
    usbip_event_add(&udc.ud, VUDC_EVENT_ERROR_TCP);
    if (ret >= 0)
    ret = -EPIPE;
    goto out;
    }
    out:
    kfree(iov);
    kfree(iso_buffer);
    free_urbp_and_urb(urb_p);
    if (ret < 0)
    return ret;
    return txsize;
    }
#[no_mangle]
unsafe extern "C" fn v_send_ret(udc: *mut vudc) -> c_int {
    static int v_send_ret(struct vudc *udc)
    {
    unsigned long flags;
    struct tx_item *txi;
    let mut total_size: usize = 0;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&udc.lock_tx, flags);
    while (!list_empty(&udc.tx_queue)) {
    txi = list_first_entry(&udc.tx_queue, struct tx_item,
    tx_entry);
    list_del(&txi.tx_entry);
    spin_unlock_irqrestore(&udc.lock_tx, flags);
    switch (txi.type) {
    case TX_SUBMIT:
    ret = v_send_ret_submit(udc, txi.s);
    break;
    case TX_UNLINK:
    ret = v_send_ret_unlink(udc, txi.u);
    break;
    }
    kfree(txi);
    if (ret < 0)
    return ret;
    total_size += ret;
    spin_lock_irqsave(&udc.lock_tx, flags);
    }
    spin_unlock_irqrestore(&udc.lock_tx, flags);
    return total_size;
    }
#[no_mangle]
pub unsafe extern "C" fn v_tx_loop(data: *mut c_void) -> c_int {
    int v_tx_loop(void *data)
    {
    struct usbip_device *ud = (struct usbip_device *) data;
    struct vudc *udc = container_of(ud, struct vudc, ud);
    int ret;
    while (!kthread_should_stop()) {
    if (usbip_event_happened(&udc.ud))
    break;
    ret = v_send_ret(udc);
    if (ret < 0) {
    pr_warn("v_tx exit with error %d", ret);
    break;
    }
    wait_event_interruptible(udc.tx_waitq,
    (!list_empty(&udc.tx_queue) ||
    kthread_should_stop()));
    }
    return 0;
    }
// called with spinlocks held
#[no_mangle]
pub unsafe extern "C" fn v_enqueue_ret_unlink(udc: *mut vudc, seqnum: __u32, status: __u32) {
    void v_enqueue_ret_unlink(struct vudc *udc, __u32 seqnum, __u32 status)
    {
    struct tx_item *txi;
    struct v_unlink *unlink;
    txi = kzalloc_obj(*txi, GFP_ATOMIC);
    if (!txi) {
    usbip_event_add(&udc.ud, VDEV_EVENT_ERROR_MALLOC);
    return;
    }
    unlink = kzalloc_obj(*unlink, GFP_ATOMIC);
    if (!unlink) {
    kfree(txi);
    usbip_event_add(&udc.ud, VDEV_EVENT_ERROR_MALLOC);
    return;
    }
    unlink.seqnum = seqnum;
    unlink.status = status;
    txi.type = TX_UNLINK;
    txi.u = unlink;
    list_add_tail(&txi.tx_entry, &udc.tx_queue);
    }
// called with spinlocks held
#[no_mangle]
pub unsafe extern "C" fn v_enqueue_ret_submit(udc: *mut vudc, urb_p: *mut urbp) {
    void v_enqueue_ret_submit(struct vudc *udc, struct urbp *urb_p)
    {
    struct tx_item *txi;
    txi = kzalloc_obj(*txi, GFP_ATOMIC);
    if (!txi) {
    usbip_event_add(&udc.ud, VDEV_EVENT_ERROR_MALLOC);
    return;
    }
    txi.type = TX_SUBMIT;
    txi.s = urb_p;
    list_add_tail(&txi.tx_entry, &udc.tx_queue);
    }
