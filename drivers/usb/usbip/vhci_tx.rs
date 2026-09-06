//! Automatically rewritten from C to Rust
//! Source: drivers/usb/usbip/vhci_tx.c
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
// Copyright (C) 2003-2008 Takahiro Hirofuchi
//

#[no_mangle]
unsafe extern "C" fn setup_cmd_submit_pdu(pdup: *mut usbip_header, urb: *mut urb) {
    static void setup_cmd_submit_pdu(struct usbip_header *pdup,  struct urb *urb)
    {
    struct vhci_priv *priv = ((struct vhci_priv *)urb.hcpriv);
    struct vhci_device *vdev = priv.vdev;
    usbip_dbg_vhci_tx("URB, local devnum %u, remote devid %u\n",
    usb_pipedevice(urb.pipe), vdev.devid);
    pdup.base.command   = USBIP_CMD_SUBMIT;
    pdup.base.seqnum    = priv.seqnum;
    pdup.base.devid     = vdev.devid;
    pdup.base.direction = usb_pipein(urb.pipe) ?
    USBIP_DIR_IN : USBIP_DIR_OUT;
    pdup.base.ep	     = usb_pipeendpoint(urb.pipe);
    usbip_pack_pdu(pdup, urb, USBIP_CMD_SUBMIT, 1);
    if (urb.setup_packet)
    memcpy(pdup.u.cmd_submit.setup, urb.setup_packet, 8);
    }
    static struct vhci_priv *dequeue_from_priv_tx(struct vhci_device *vdev)
    {
    struct vhci_priv *priv, *tmp;
    unsigned long flags;
    spin_lock_irqsave(&vdev.priv_lock, flags);
    list_for_each_entry_safe(priv, tmp, &vdev.priv_tx, list) {
    list_move_tail(&priv.list, &vdev.priv_rx);
    spin_unlock_irqrestore(&vdev.priv_lock, flags);
    return priv;
    }
    spin_unlock_irqrestore(&vdev.priv_lock, flags);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn vhci_send_cmd_submit(vdev: *mut vhci_device) -> c_int {
    static int vhci_send_cmd_submit(struct vhci_device *vdev)
    {
    struct usbip_iso_packet_descriptor *iso_buffer = core::ptr::null_mut();
    struct vhci_priv *priv = core::ptr::null_mut();
    struct scatterlist *sg;
    struct msghdr msg;
    struct kvec *iov;
    size_t txsize;
    let mut total_size: usize = 0;
    int iovnum;
    let mut err: c_int = -ENOMEM;
    int i;
    while ((priv = dequeue_from_priv_tx(vdev)) != core::ptr::null_mut()) {
    int ret;
    struct urb *urb = priv.urb;
    struct usbip_header pdu_header;
    txsize = 0;
    memset(&pdu_header, 0, sizeof(pdu_header));
    memset(&msg, 0, sizeof(msg));
    memset(&iov, 0, sizeof(iov));
    usbip_dbg_vhci_tx("setup txdata urb seqnum %lu\n",
    priv.seqnum);
    if (urb.num_sgs && usb_pipeout(urb.pipe))
    iovnum = 2 + urb.num_sgs;
    else
    iovnum = 3;
    iov = kzalloc_objs(*iov, iovnum);
    if (!iov) {
    usbip_event_add(&vdev.ud, SDEV_EVENT_ERROR_MALLOC);
    return -ENOMEM;
    }
    if (urb.num_sgs)
    urb.transfer_flags |= URB_DMA_MAP_SG;
// 1. setup usbip_header
    setup_cmd_submit_pdu(&pdu_header, urb);
    usbip_header_correct_endian(&pdu_header, 1);
    iovnum = 0;
    iov[iovnum].iov_base = &pdu_header;
    iov[iovnum].iov_len  = sizeof(pdu_header);
    txsize += sizeof(pdu_header);
    iovnum++;
// 2. setup transfer buffer
    if (!usb_pipein(urb.pipe) && urb.transfer_buffer_length > 0) {
    if (urb.num_sgs &&
    !usb_endpoint_xfer_isoc(&urb.ep.desc)) {
    for_each_sg(urb.sg, sg, urb.num_sgs, i) {
    iov[iovnum].iov_base = sg_virt(sg);
    iov[iovnum].iov_len = sg.length;
    iovnum++;
    }
    } else {
    iov[iovnum].iov_base = urb.transfer_buffer;
    iov[iovnum].iov_len  =
    urb.transfer_buffer_length;
    iovnum++;
    }
    txsize += urb.transfer_buffer_length;
    }
// 3. setup iso_packet_descriptor
    if (usb_pipetype(urb.pipe) == PIPE_ISOCHRONOUS) {
    let mut len: isize = 0;
    iso_buffer = usbip_alloc_iso_desc_pdu(urb, &len);
    if (!iso_buffer) {
    usbip_event_add(&vdev.ud,
    SDEV_EVENT_ERROR_MALLOC);
    goto err_iso_buffer;
    }
    iov[iovnum].iov_base = iso_buffer;
    iov[iovnum].iov_len  = len;
    iovnum++;
    txsize += len;
    }
    ret = kernel_sendmsg(vdev.ud.tcp_socket, &msg, iov, iovnum,
    txsize);
    if (ret != txsize) {
    pr_err("sendmsg failed!, ret=%d for %zd\n", ret,
    txsize);
    usbip_event_add(&vdev.ud, VDEV_EVENT_ERROR_TCP);
    err = -EPIPE;
    goto err_tx;
    }
    kfree(iov);
// This is only for isochronous case
    kfree(iso_buffer);
    iso_buffer = core::ptr::null_mut();
    usbip_dbg_vhci_tx("send txdata\n");
    total_size += txsize;
    }
    return total_size;
    err_tx:
    kfree(iso_buffer);
    err_iso_buffer:
    kfree(iov);
    return err;
    }
    static struct vhci_unlink *dequeue_from_unlink_tx(struct vhci_device *vdev)
    {
    struct vhci_unlink *unlink, *tmp;
    unsigned long flags;
    spin_lock_irqsave(&vdev.priv_lock, flags);
    list_for_each_entry_safe(unlink, tmp, &vdev.unlink_tx, list) {
    list_move_tail(&unlink.list, &vdev.unlink_rx);
    spin_unlock_irqrestore(&vdev.priv_lock, flags);
    return unlink;
    }
    spin_unlock_irqrestore(&vdev.priv_lock, flags);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn vhci_send_cmd_unlink(vdev: *mut vhci_device) -> c_int {
    static int vhci_send_cmd_unlink(struct vhci_device *vdev)
    {
    struct vhci_unlink *unlink = core::ptr::null_mut();
    struct msghdr msg;
    struct kvec iov;
    size_t txsize;
    let mut total_size: usize = 0;
    while ((unlink = dequeue_from_unlink_tx(vdev)) != core::ptr::null_mut()) {
    int ret;
    struct usbip_header pdu_header;
    memset(&pdu_header, 0, sizeof(pdu_header));
    memset(&msg, 0, sizeof(msg));
    memset(&iov, 0, sizeof(iov));
    usbip_dbg_vhci_tx("setup cmd unlink, %lu\n", unlink.seqnum);
// 1. setup usbip_header
    pdu_header.base.command = USBIP_CMD_UNLINK;
    pdu_header.base.seqnum  = unlink.seqnum;
    pdu_header.base.devid	= vdev.devid;
    pdu_header.base.ep	= 0;
    pdu_header.u.cmd_unlink.seqnum = unlink.unlink_seqnum;
    usbip_header_correct_endian(&pdu_header, 1);
    iov.iov_base = &pdu_header;
    iov.iov_len  = sizeof(pdu_header);
    txsize = sizeof(pdu_header);
    ret = kernel_sendmsg(vdev.ud.tcp_socket, &msg, &iov, 1, txsize);
    if (ret != txsize) {
    pr_err("sendmsg failed!, ret=%d for %zd\n", ret,
    txsize);
    usbip_event_add(&vdev.ud, VDEV_EVENT_ERROR_TCP);
    return -1;
    }
    usbip_dbg_vhci_tx("send txdata\n");
    total_size += txsize;
    }
    return total_size;
    }
#[no_mangle]
pub unsafe extern "C" fn vhci_tx_loop(data: *mut c_void) -> c_int {
    int vhci_tx_loop(void *data)
    {
    struct usbip_device *ud = data;
    struct vhci_device *vdev = container_of(ud, struct vhci_device, ud);
    while (!kthread_should_stop()) {
    if (vhci_send_cmd_submit(vdev) < 0)
    break;
    if (vhci_send_cmd_unlink(vdev) < 0)
    break;
    wait_event_interruptible(vdev.waitq_tx,
    (!list_empty(&vdev.priv_tx) ||
    !list_empty(&vdev.unlink_tx) ||
    kthread_should_stop()));
    usbip_dbg_vhci_tx("pending urbs ?, now wake up\n");
    }
    return 0;
    }
