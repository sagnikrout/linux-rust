//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ath/ath6kl/usb.c
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


//
// Copyright (c) 2007-2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// constants
pub const TX_URB_COUNT: c_int = 32;
pub const RX_URB_COUNT: c_int = 32;
pub const ATH6KL_USB_RX_BUFFER_SIZE: c_int = 4096;
// tx/rx pipes for usb
    enum ATH6KL_USB_PIPE_ID {
    ATH6KL_USB_PIPE_TX_CTRL = 0,
    ATH6KL_USB_PIPE_TX_DATA_LP,
    ATH6KL_USB_PIPE_TX_DATA_MP,
    ATH6KL_USB_PIPE_TX_DATA_HP,
    ATH6KL_USB_PIPE_RX_CTRL,
    ATH6KL_USB_PIPE_RX_DATA,
    ATH6KL_USB_PIPE_RX_DATA2,
    ATH6KL_USB_PIPE_RX_INT,
    ATH6KL_USB_PIPE_MAX
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_usb_pipe {
    pub urb_list_head: list_head,
    pub urb_submitted: usb_anchor,
    pub urb_alloc: u32,
    pub urb_cnt: u32,
    pub urb_cnt_thresh: u32,
    pub usb_pipe_handle: c_uint,
    pub flags: u32,
    pub ep_address: u8,
    pub logical_pipe_num: u8,
    pub ar_usb: *mut ath6kl_usb,
    pub max_packet_size: u16,
    pub io_complete_work: work_struct,
    pub io_comp_queue: sk_buff_head,
    pub ep_desc: *mut usb_endpoint_descriptor,
}

// usb device object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_usb {
// protects pipe->urb_list_head and  pipe->urb_cnt
    pub cs_lock: spinlock_t,
    pub udev: *mut usb_device,
    pub interface: *mut usb_interface,
    pub pipes: [ath6kl_usb_pipe; ATH6KL_USB_PIPE_MAX],
    pub diag_cmd_buffer: *mut u8,
    pub diag_resp_buffer: *mut u8,
    pub ar: *mut ath6kl,
    pub wq: *mut workqueue_struct,
}

// usb urb object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_urb_context {
    pub link: list_head,
    pub pipe: *mut ath6kl_usb_pipe,
    pub skb: *mut sk_buff,
    pub ar: *mut ath6kl,
}

// USB endpoint definitions
pub const ATH6KL_USB_EP_ADDR_APP_CTRL_IN: c_uint = 0x81;
pub const ATH6KL_USB_EP_ADDR_APP_DATA_IN: c_uint = 0x82;
pub const ATH6KL_USB_EP_ADDR_APP_DATA2_IN: c_uint = 0x83;
pub const ATH6KL_USB_EP_ADDR_APP_INT_IN: c_uint = 0x84;
pub const ATH6KL_USB_EP_ADDR_APP_CTRL_OUT: c_uint = 0x01;
pub const ATH6KL_USB_EP_ADDR_APP_DATA_LP_OUT: c_uint = 0x02;
pub const ATH6KL_USB_EP_ADDR_APP_DATA_MP_OUT: c_uint = 0x03;
pub const ATH6KL_USB_EP_ADDR_APP_DATA_HP_OUT: c_uint = 0x04;
// diagnostic command definitions
pub const ATH6KL_USB_CONTROL_REQ_SEND_BMI_CMD: c_int = 1;
pub const ATH6KL_USB_CONTROL_REQ_RECV_BMI_RESP: c_int = 2;
pub const ATH6KL_USB_CONTROL_REQ_DIAG_CMD: c_int = 3;
pub const ATH6KL_USB_CONTROL_REQ_DIAG_RESP: c_int = 4;
pub const ATH6KL_USB_CTRL_DIAG_CC_READ: c_int = 0;
pub const ATH6KL_USB_CTRL_DIAG_CC_WRITE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_usb_ctrl_diag_cmd_write {
    pub cmd: __le32,
    pub address: __le32,
    pub value: __le32,
    pub _pad: [__le32; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_usb_ctrl_diag_cmd_read {
    pub cmd: __le32,
    pub address: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_usb_ctrl_diag_resp_read {
    pub value: __le32,
    pub __packed: },
// function declarations
    pub urb): *mut static void ath6kl_usb_recv_complete(struct urb,

// pipe/urb operations
    static struct ath6kl_urb_context *
    ath6kl_usb_alloc_urb_from_pipe(struct ath6kl_usb_pipe *pipe)
    {
    pub NULL: *mut *mut ath6kl_urb_context urb_context =,
    pub flags: c_ulong,
// bail if this pipe is not initialized
    if (!pipe.ar_usb)
    pub NULL: return,
    pub flags): spin_lock_irqsave(&pipe->ar_usb->cs_lock,,
    if (!list_empty(&pipe.urb_list_head)) {
    urb_context =
    list_first_entry(&pipe.urb_list_head,
    pub link): ath6kl_urb_context,,
    }
    pub flags): spin_unlock_irqrestore(&pipe->ar_usb->cs_lock,,
    pub urb_context: return,
    }
    static void ath6kl_usb_free_urb_to_pipe(struct ath6kl_usb_pipe *pipe,
    struct ath6kl_urb_context *urb_context)
    {
    pub flags: c_ulong,
// bail if this pipe is not initialized
    if (!pipe.ar_usb)
    pub flags): spin_lock_irqsave(&pipe->ar_usb->cs_lock,,
    pub &pipe->urb_list_head): list_add(&urb_context->link,,
    pub flags): spin_unlock_irqrestore(&pipe->ar_usb->cs_lock,,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_cleanup_recv_urb(urb_context: *mut ath6kl_urb_context) {
    static void ath6kl_usb_cleanup_recv_urb(struct ath6kl_urb_context *urb_context)
    {
    pub NULL: urb_context->skb =,
    pub urb_context): ath6kl_usb_free_urb_to_pipe(urb_context->pipe,,
    }
    static inline struct ath6kl_usb *ath6kl_usb_priv(struct ath6kl *ar)
    {
    pub ar->hif_priv: return,
    }
// pipe resource allocation/cleanup
    static int ath6kl_usb_alloc_pipe_resources(struct ath6kl_usb_pipe *pipe,
    int urb_cnt)
    {
    pub urb_context: *mut ath6kl_urb_context,
    pub i: int status = 0,,
    pub {: for (i = 0; i < urb_cnt; i++),
    pub ath6kl_urb_context): urb_context = kzalloc_obj(struct,
    if (urb_context == core::ptr::null_mut()) {
    pub -ENOMEM: status =,
    pub fail_alloc_pipe_resources: goto,
    }
    pub pipe: urb_context->pipe =,
//
// we are only allocate the urb contexts here, the actual URB
// is allocated from the kernel as needed to do a transaction
//
    pub urb_context): ath6kl_usb_free_urb_to_pipe(pipe,,
    }
    ath6kl_dbg(ATH6KL_DBG_USB,
    "ath6kl usb: alloc resources lpipe:%d hpipe:0x%X urbs:%d\n",
    pipe.logical_pipe_num, pipe.usb_pipe_handle,
    fail_alloc_pipe_resources:
    pub status: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_free_pipe_resources(pipe: *mut ath6kl_usb_pipe) {
    static void ath6kl_usb_free_pipe_resources(struct ath6kl_usb_pipe *pipe)
    {
    pub urb_context: *mut ath6kl_urb_context,
    if (pipe.ar_usb == core::ptr::null_mut()) {
// nothing allocated for this pipe
    }
    ath6kl_dbg(ATH6KL_DBG_USB,
    "ath6kl usb: free resources lpipe:%d"
    "hpipe:0x%X urbs:%d avail:%d\n",
    pipe.logical_pipe_num, pipe.usb_pipe_handle,
    pub pipe->urb_cnt): pipe->urb_alloc,,
    if (pipe.urb_alloc != pipe.urb_cnt) {
    ath6kl_dbg(ATH6KL_DBG_USB,
    "ath6kl usb: urb leak! lpipe:%d"
    "hpipe:0x%X urbs:%d avail:%d\n",
    pipe.logical_pipe_num, pipe.usb_pipe_handle,
    pub pipe->urb_cnt): pipe->urb_alloc,,
    }
    while (true) {
    pub ath6kl_usb_alloc_urb_from_pipe(pipe): urb_context =,
    if (urb_context == core::ptr::null_mut())
    }
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_cleanup_pipe_resources(ar_usb: *mut ath6kl_usb) {
    static void ath6kl_usb_cleanup_pipe_resources(struct ath6kl_usb *ar_usb)
    {
    pub i: c_int,
    pub i++): for (i = 0; i < ATH6KL_USB_PIPE_MAX;,
    }
    static u8 ath6kl_usb_get_logical_pipe_num(struct ath6kl_usb *ar_usb,
    u8 ep_address, int *urb_count)
    {
    pub ATH6KL_USB_PIPE_INVALID: u8 pipe_num =,
    switch (ep_address) {
    case ATH6KL_USB_EP_ADDR_APP_CTRL_IN:
    pub ATH6KL_USB_PIPE_RX_CTRL: pipe_num =,
// urb_count = RX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_DATA_IN:
    pub ATH6KL_USB_PIPE_RX_DATA: pipe_num =,
// urb_count = RX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_INT_IN:
    pub ATH6KL_USB_PIPE_RX_INT: pipe_num =,
// urb_count = RX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_DATA2_IN:
    pub ATH6KL_USB_PIPE_RX_DATA2: pipe_num =,
// urb_count = RX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_CTRL_OUT:
    pub ATH6KL_USB_PIPE_TX_CTRL: pipe_num =,
// urb_count = TX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_DATA_LP_OUT:
    pub ATH6KL_USB_PIPE_TX_DATA_LP: pipe_num =,
// urb_count = TX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_DATA_MP_OUT:
    pub ATH6KL_USB_PIPE_TX_DATA_MP: pipe_num =,
// urb_count = TX_URB_COUNT;
    case ATH6KL_USB_EP_ADDR_APP_DATA_HP_OUT:
    pub ATH6KL_USB_PIPE_TX_DATA_HP: pipe_num =,
// urb_count = TX_URB_COUNT;
    default:
// note: there may be endpoints not currently used
    }
    pub pipe_num: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_setup_pipe_resources(ar_usb: *mut ath6kl_usb) -> c_int {
    static int ath6kl_usb_setup_pipe_resources(struct ath6kl_usb *ar_usb)
    {
    pub ar_usb->interface: *mut *mut usb_interface interface =,
    pub interface->cur_altsetting: *mut *mut usb_host_interface iface_desc =,
    pub endpoint: *mut usb_endpoint_descriptor,
    pub pipe: *mut ath6kl_usb_pipe,
    pub 0: int i, urbcount, status =,
    pub pipe_num: u8,
    pub interface\n"): ath6kl_dbg(ATH6KL_DBG_USB, "setting up USB Pipes using,
// walk descriptors and setup pipes
    pub {: for (i = 0; i < iface_desc->desc.bNumEndpoints; ++i),
    pub &iface_desc->endpoint[i].desc: endpoint =,
    if (ATH6KL_USB_IS_BULK_EP(endpoint.bmAttributes)) {
    ath6kl_dbg(ATH6KL_DBG_USB,
    "%s Bulk Ep:0x%2.2X maxpktsz:%d\n",
    ATH6KL_USB_IS_DIR_IN
    (endpoint.bEndpointAddress) ?
    "RX" : "TX", endpoint.bEndpointAddress,
    } else if (ATH6KL_USB_IS_INT_EP(endpoint.bmAttributes)) {
    ath6kl_dbg(ATH6KL_DBG_USB,
    "%s Int Ep:0x%2.2X maxpktsz:%d interval:%d\n",
    ATH6KL_USB_IS_DIR_IN
    (endpoint.bEndpointAddress) ?
    "RX" : "TX", endpoint.bEndpointAddress,
    le16_to_cpu(endpoint.wMaxPacketSize),
    } else if (ATH6KL_USB_IS_ISOC_EP(endpoint.bmAttributes)) {
// TODO for ISO
    ath6kl_dbg(ATH6KL_DBG_USB,
    "%s ISOC Ep:0x%2.2X maxpktsz:%d interval:%d\n",
    ATH6KL_USB_IS_DIR_IN
    (endpoint.bEndpointAddress) ?
    "RX" : "TX", endpoint.bEndpointAddress,
    le16_to_cpu(endpoint.wMaxPacketSize),
    }
// Ignore broken descriptors.
    if (usb_endpoint_maxp(endpoint) == 0)
    pub 0: urbcount =,
    pipe_num =
    ath6kl_usb_get_logical_pipe_num(ar_usb,
    endpoint.bEndpointAddress,
    if (pipe_num == ATH6KL_USB_PIPE_INVALID)
    pub &ar_usb->pipes[pipe_num]: pipe =,
    if (pipe.ar_usb != core::ptr::null_mut()) {
// hmmm..pipe was already setup
    }
    pub ar_usb: pipe->ar_usb =,
    pub pipe_num: pipe->logical_pipe_num =,
    pub endpoint->bEndpointAddress: pipe->ep_address =,
    pub le16_to_cpu(endpoint->wMaxPacketSize): pipe->max_packet_size =,
    if (ATH6KL_USB_IS_BULK_EP(endpoint.bmAttributes)) {
    if (ATH6KL_USB_IS_DIR_IN(pipe.ep_address)) {
    pipe.usb_pipe_handle =
    usb_rcvbulkpipe(ar_usb.udev,
    } else {
    pipe.usb_pipe_handle =
    usb_sndbulkpipe(ar_usb.udev,
    }
    } else if (ATH6KL_USB_IS_INT_EP(endpoint.bmAttributes)) {
    if (ATH6KL_USB_IS_DIR_IN(pipe.ep_address)) {
    pipe.usb_pipe_handle =
    usb_rcvintpipe(ar_usb.udev,
    } else {
    pipe.usb_pipe_handle =
    usb_sndintpipe(ar_usb.udev,
    }
    } else if (ATH6KL_USB_IS_ISOC_EP(endpoint.bmAttributes)) {
// TODO for ISO
    if (ATH6KL_USB_IS_DIR_IN(pipe.ep_address)) {
    pipe.usb_pipe_handle =
    usb_rcvisocpipe(ar_usb.udev,
    } else {
    pipe.usb_pipe_handle =
    usb_sndisocpipe(ar_usb.udev,
    }
    }
    pub endpoint: pipe->ep_desc =,
    if (!ATH6KL_USB_IS_DIR_IN(pipe.ep_address))
    pub ATH6KL_USB_PIPE_FLAG_TX: pipe->flags |=,
    pub urbcount): status = ath6kl_usb_alloc_pipe_resources(pipe,,
    if (status != 0)
    }
    pub status: return,
    }
// pipe operations
    static void ath6kl_usb_post_recv_transfers(struct ath6kl_usb_pipe *recv_pipe,
    int buffer_length)
    {
    pub urb_context: *mut ath6kl_urb_context,
    pub urb: *mut urb,
    pub usb_status: c_int,
    while (true) {
    pub ath6kl_usb_alloc_urb_from_pipe(recv_pipe): urb_context =,
    if (urb_context == core::ptr::null_mut())
    pub dev_alloc_skb(buffer_length): urb_context->skb =,
    if (urb_context.skb == core::ptr::null_mut())
    pub err_cleanup_urb: goto,
    pub GFP_ATOMIC): urb = usb_alloc_urb(0,,
    if (urb == core::ptr::null_mut())
    pub err_cleanup_urb: goto,
    usb_fill_bulk_urb(urb,
    recv_pipe.ar_usb.udev,
    recv_pipe.usb_pipe_handle,
    urb_context.skb.data,
    buffer_length,
    pub urb_context): ath6kl_usb_recv_complete,,
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "ath6kl usb: bulk recv submit:%d, 0x%X (ep:0x%2.2X), %d bytes buf:0x%p\n",
    recv_pipe.logical_pipe_num,
    recv_pipe.usb_pipe_handle, recv_pipe.ep_address,
    pub urb_context->skb): buffer_length,,
    pub &recv_pipe->urb_submitted): usb_anchor_urb(urb,,
    pub GFP_ATOMIC): usb_status = usb_submit_urb(urb,,
    if (usb_status) {
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "ath6kl usb : usb bulk recv failed %d\n",
    pub err_cleanup_urb: goto,
    }
    }
    err_cleanup_urb:
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_flush_all(ar_usb: *mut ath6kl_usb) {
    static void ath6kl_usb_flush_all(struct ath6kl_usb *ar_usb)
    {
    pub i: c_int,
    pub {: for (i = 0; i < ATH6KL_USB_PIPE_MAX; i++),
    if (ar_usb.pipes[i].ar_usb != core::ptr::null_mut())
    }
//
// Flushing any pending I/O may schedule work this call will block
// until all scheduled work runs to completion.
//
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_start_recv_pipes(ar_usb: *mut ath6kl_usb) {
    static void ath6kl_usb_start_recv_pipes(struct ath6kl_usb *ar_usb)
    {
//
// note: control pipe is no longer used
// ar_usb->pipes[ATH6KL_USB_PIPE_RX_CTRL].urb_cnt_thresh =
// ar_usb->pipes[ATH6KL_USB_PIPE_RX_CTRL].urb_alloc/2;
// ath6kl_usb_post_recv_transfers(&ar_usb->
// pipes[ATH6KL_USB_PIPE_RX_CTRL],
// ATH6KL_USB_RX_BUFFER_SIZE);
//
    pub 1: ar_usb->pipes[ATH6KL_USB_PIPE_RX_DATA].urb_cnt_thresh =,
    ath6kl_usb_post_recv_transfers(&ar_usb.pipes[ATH6KL_USB_PIPE_RX_DATA],
    }
// hif usb rx/tx completion functions
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_recv_complete(urb: *mut urb) {
    static void ath6kl_usb_recv_complete(struct urb *urb)
    {
    pub urb->context: *mut *mut ath6kl_urb_context urb_context =,
    pub urb_context->pipe: *mut *mut ath6kl_usb_pipe pipe =,
    pub NULL: *mut *mut sk_buff skb =,
    pub 0: int status =,
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "%s: recv pipe: %d, stat:%d, len:%d urb:0x%p\n", __func__,
    pipe.logical_pipe_num, urb.status, urb.actual_length,
    if (urb.status != 0) {
    pub -EIO: status =,
    switch (urb.status) {
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
//
// no need to spew these errors when device
// removed or urb killed due to driver shutdown
//
    pub -ECANCELED: status =,
    default:
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "%s recv pipe: %d (ep:0x%2.2X), failed:%d\n",
    __func__, pipe.logical_pipe_num,
    pub urb->status): pipe->ep_address,,
    }
    pub cleanup_recv_urb: goto,
    }
    if (urb.actual_length == 0)
    pub cleanup_recv_urb: goto,
    pub urb_context->skb: skb =,
// we are going to pass it up
    pub NULL: urb_context->skb =,
    pub urb->actual_length): skb_put(skb,,
// note: queue implements a lock
    pub skb): skb_queue_tail(&pipe->io_comp_queue,,
    pub &pipe->io_complete_work): queue_work(pipe->ar_usb->wq,,
    cleanup_recv_urb:
    if (status == 0 &&
    pipe.urb_cnt >= pipe.urb_cnt_thresh) {
// our free urbs are piling up, post more transfers
    pub ATH6KL_USB_RX_BUFFER_SIZE): ath6kl_usb_post_recv_transfers(pipe,,
    }
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_usb_transmit_complete(urb: *mut urb) {
    static void ath6kl_usb_usb_transmit_complete(struct urb *urb)
    {
    pub urb->context: *mut *mut ath6kl_urb_context urb_context =,
    pub urb_context->pipe: *mut *mut ath6kl_usb_pipe pipe =,
    pub skb: *mut sk_buff,
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "%s: pipe: %d, stat:%d, len:%d\n",
    __func__, pipe.logical_pipe_num, urb.status,
    if (urb.status != 0) {
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "%s:  pipe: %d, failed:%d\n",
    pub urb->status): __func__, pipe->logical_pipe_num,,
    }
    pub urb_context->skb: skb =,
    pub NULL: urb_context->skb =,
    pub urb_context): ath6kl_usb_free_urb_to_pipe(urb_context->pipe,,
// note: queue implements a lock
    pub skb): skb_queue_tail(&pipe->io_comp_queue,,
    pub &pipe->io_complete_work): queue_work(pipe->ar_usb->wq,,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_io_comp_work(work: *mut work_struct) {
    static void ath6kl_usb_io_comp_work(struct work_struct *work)
    {
    struct ath6kl_usb_pipe *pipe = container_of(work,
    struct ath6kl_usb_pipe,
    pub ar_usb: *mut ath6kl_usb,
    pub skb: *mut sk_buff,
    pub pipe->ar_usb: ar_usb =,
    while ((skb = skb_dequeue(&pipe.io_comp_queue))) {
    if (pipe.flags & ATH6KL_USB_PIPE_FLAG_TX) {
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    pub skb): "ath6kl usb xmit callback buf:0x%p\n",,
    pub skb): ath6kl_core_tx_complete(ar_usb->ar,,
    } else {
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    pub skb): "ath6kl usb recv callback buf:0x%p\n",,
    ath6kl_core_rx_complete(ar_usb.ar, skb,
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn ath6kl_usb_destroy(ar_usb: *mut ath6kl_usb) {
    static void ath6kl_usb_destroy(struct ath6kl_usb *ar_usb)
    {
    pub NULL): usb_set_intfdata(ar_usb->interface,,
    }
    static struct ath6kl_usb *ath6kl_usb_create(struct usb_interface *interface)
    {
    pub interface_to_usbdev(interface): *mut *mut usb_device dev =,
    pub ar_usb: *mut ath6kl_usb,
    pub pipe: *mut ath6kl_usb_pipe,
    pub 0: int status =,
    pub i: c_int,
// ath6kl_usb_destroy() needs ar_usb != NULL && ar_usb->wq != NULL.
    pub ath6kl_usb): ar_usb = kzalloc_obj(struct,
    if (ar_usb == core::ptr::null_mut())
    pub NULL: return,
    pub 0): ar_usb->wq = alloc_workqueue("ath6kl_wq", WQ_PERCPU,,
    if (!ar_usb.wq) {
    pub NULL: return,
    }
    pub ar_usb): usb_set_intfdata(interface,,
    pub dev: ar_usb->udev =,
    pub interface: ar_usb->interface =,
    pub {: for (i = 0; i < ATH6KL_USB_PIPE_MAX; i++),
    pub &ar_usb->pipes[i]: pipe =,
    INIT_WORK(&pipe.io_complete_work,
    }
    pub GFP_KERNEL): ar_usb->diag_cmd_buffer = kzalloc(ATH6KL_USB_MAX_DIAG_CMD,,
    if (ar_usb.diag_cmd_buffer == core::ptr::null_mut()) {
    pub -ENOMEM: status =,
    pub fail_ath6kl_usb_create: goto,
    }
    ar_usb.diag_resp_buffer = kzalloc(ATH6KL_USB_MAX_DIAG_RESP,
    if (ar_usb.diag_resp_buffer == core::ptr::null_mut()) {
    pub -ENOMEM: status =,
    pub fail_ath6kl_usb_create: goto,
    }
    pub ath6kl_usb_setup_pipe_resources(ar_usb): status =,
    fail_ath6kl_usb_create:
    if (status != 0) {
    pub NULL: ar_usb =,
    }
    pub ar_usb: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_device_detached(interface: *mut usb_interface) {
    static void ath6kl_usb_device_detached(struct usb_interface *interface)
    {
    pub ar_usb: *mut ath6kl_usb,
    pub usb_get_intfdata(interface): ar_usb =,
    if (ar_usb == core::ptr::null_mut())
// Delay to wait for the target to reboot
    }
// exported hif usb APIs for htc pipe
#[no_mangle]
unsafe extern "C" fn hif_start(ar: *mut ath6kl) {
    static void hif_start(struct ath6kl *ar)
    {
    pub ath6kl_usb_priv(ar): *mut *mut ath6kl_usb device =,
    pub i: c_int,
// set the TX resource avail threshold for each TX pipe
    pub ATH6KL_USB_PIPE_TX_CTRL: for (i =,
    pub {: i <= ATH6KL_USB_PIPE_TX_DATA_HP; i++),
    device.pipes[i].urb_cnt_thresh =
    pub 2: device->pipes[i].urb_alloc /,
    }
    }
    static int ath6kl_usb_send(struct ath6kl *ar, u8 PipeID,
    struct sk_buff *hdr_skb, struct sk_buff *skb)
    {
    pub ath6kl_usb_priv(ar): *mut *mut ath6kl_usb device =,
    pub &device->pipes[PipeID]: *mut *mut ath6kl_usb_pipe pipe =,
    pub urb_context: *mut ath6kl_urb_context,
    pub 0: int usb_status, status =,
    pub urb: *mut urb,
    pub data: *mut u8,
    pub len: u32,
    ath6kl_dbg(ATH6KL_DBG_USB_BULK, "+%s pipe : %d, buf:0x%p\n",
    pub skb): __func__, PipeID,,
    pub ath6kl_usb_alloc_urb_from_pipe(pipe): urb_context =,
    if (urb_context == core::ptr::null_mut()) {
//
// TODO: it is possible to run out of urbs if
// 2 endpoints map to the same pipe ID
//
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "%s pipe:%d no urbs left. URB Cnt : %d\n",
    pub pipe->urb_cnt): __func__, PipeID,,
    pub -ENOMEM: status =,
    pub fail_hif_send: goto,
    }
    pub skb: urb_context->skb =,
    pub skb->data: data =,
    pub skb->len: len =,
    pub GFP_ATOMIC): urb = usb_alloc_urb(0,,
    if (urb == core::ptr::null_mut()) {
    pub -ENOMEM: status =,
    ath6kl_usb_free_urb_to_pipe(urb_context.pipe,
    pub fail_hif_send: goto,
    }
    usb_fill_bulk_urb(urb,
    device.udev,
    pipe.usb_pipe_handle,
    data,
    len,
    pub urb_context): ath6kl_usb_usb_transmit_complete,,
    if ((len % pipe.max_packet_size) == 0) {
// hit a max packet boundary on this pipe
    pub URB_ZERO_PACKET: urb->transfer_flags |=,
    }
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "athusb bulk send submit:%d, 0x%X (ep:0x%2.2X), %d bytes\n",
    pipe.logical_pipe_num, pipe.usb_pipe_handle,
    pub len): pipe->ep_address,,
    pub &pipe->urb_submitted): usb_anchor_urb(urb,,
    pub GFP_ATOMIC): usb_status = usb_submit_urb(urb,,
    if (usb_status) {
    ath6kl_dbg(ATH6KL_DBG_USB_BULK,
    "ath6kl usb : usb bulk transmit failed %d\n",
    ath6kl_usb_free_urb_to_pipe(urb_context.pipe,
    pub -EINVAL: status =,
    }
    fail_hif_send:
    pub status: return,
    }
#[no_mangle]
unsafe extern "C" fn hif_stop(ar: *mut ath6kl) {
    static void hif_stop(struct ath6kl *ar)
    {
    pub ath6kl_usb_priv(ar): *mut *mut ath6kl_usb device =,
    }
    static void ath6kl_usb_get_default_pipe(struct ath6kl *ar,
    u8 *ul_pipe, u8 *dl_pipe)
    {
// ul_pipe = ATH6KL_USB_PIPE_TX_CTRL;
// dl_pipe = ATH6KL_USB_PIPE_RX_CTRL;
    }
    static int ath6kl_usb_map_service_pipe(struct ath6kl *ar, u16 svc_id,
    u8 *ul_pipe, u8 *dl_pipe)
    {
    pub 0: int status =,
    switch (svc_id) {
    case HTC_CTRL_RSVD_SVC:
    case WMI_CONTROL_SVC:
// ul_pipe = ATH6KL_USB_PIPE_TX_CTRL;
// due to large control packets, shift to data pipe
// dl_pipe = ATH6KL_USB_PIPE_RX_DATA;
    case WMI_DATA_BE_SVC:
    case WMI_DATA_BK_SVC:
// ul_pipe = ATH6KL_USB_PIPE_TX_DATA_LP;
//
// Disable rxdata2 directly, it will be enabled
// if FW enable rxdata2
//
// dl_pipe = ATH6KL_USB_PIPE_RX_DATA;
    case WMI_DATA_VI_SVC:
    if (test_bit(ATH6KL_FW_CAPABILITY_MAP_LP_ENDPOINT,
    ar.fw_capabilities))
// ul_pipe = ATH6KL_USB_PIPE_TX_DATA_LP;
    else
// ul_pipe = ATH6KL_USB_PIPE_TX_DATA_MP;
//
// Disable rxdata2 directly, it will be enabled
// if FW enable rxdata2
//
// dl_pipe = ATH6KL_USB_PIPE_RX_DATA;
    case WMI_DATA_VO_SVC:
    if (test_bit(ATH6KL_FW_CAPABILITY_MAP_LP_ENDPOINT,
    ar.fw_capabilities))
// ul_pipe = ATH6KL_USB_PIPE_TX_DATA_LP;
    else
// ul_pipe = ATH6KL_USB_PIPE_TX_DATA_MP;
//
// Disable rxdata2 directly, it will be enabled
// if FW enable rxdata2
//
// dl_pipe = ATH6KL_USB_PIPE_RX_DATA;
    default:
    pub -EPERM: status =,
    }
    pub status: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_get_free_queue_number(ar: *mut ath6kl, pipe_id: u8) -> u16 {
    static u16 ath6kl_usb_get_free_queue_number(struct ath6kl *ar, u8 pipe_id)
    {
    pub ath6kl_usb_priv(ar): *mut *mut ath6kl_usb device =,
    pub device->pipes[pipe_id].urb_cnt: return,
    }
#[no_mangle]
unsafe extern "C" fn hif_detach_htc(ar: *mut ath6kl) {
    static void hif_detach_htc(struct ath6kl *ar)
    {
    pub ath6kl_usb_priv(ar): *mut *mut ath6kl_usb device =,
    }
    static int ath6kl_usb_submit_ctrl_out(struct ath6kl_usb *ar_usb,
    u8 req, u16 value, u16 index, void *data,
    u32 size)
    {
    pub NULL: *mut *mut u8 buf =,
    pub ret: c_int,
    if (size > 0) {
    pub GFP_KERNEL): buf = kmemdup(data, size,,
    if (buf == core::ptr::null_mut())
    pub -ENOMEM: return,
    }
// note: if successful returns number of bytes transferred
    ret = usb_control_msg(ar_usb.udev,
    usb_sndctrlpipe(ar_usb.udev, 0),
    req,
    USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE, value, index, buf,
    pub 1000): size,,
    if (ret < 0) {
    pub ret): ath6kl_warn("Failed to submit usb control message: %d\n",,
    pub ret: return,
    }
    pub 0: return,
    }
    static int ath6kl_usb_submit_ctrl_in(struct ath6kl_usb *ar_usb,
    u8 req, u16 value, u16 index, void *data,
    u32 size)
    {
    pub NULL: *mut *mut u8 buf =,
    pub ret: c_int,
    if (size > 0) {
    pub GFP_KERNEL): buf = kmalloc(size,,
    if (buf == core::ptr::null_mut())
    pub -ENOMEM: return,
    }
// note: if successful returns number of bytes transferred
    ret = usb_control_msg(ar_usb.udev,
    usb_rcvctrlpipe(ar_usb.udev, 0),
    req,
    USB_DIR_IN | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE, value, index, buf,
    pub 2000): size,,
    if (ret < 0) {
    pub ret): ath6kl_warn("Failed to read usb control message: %d\n",,
    pub ret: return,
    }
    pub size): *mut *mut memcpy((u8 ) data, buf,,
    pub 0: return,
    }
    static int ath6kl_usb_ctrl_msg_exchange(struct ath6kl_usb *ar_usb,
    u8 req_val, u8 *req_buf, u32 req_len,
    u8 resp_val, u8 *resp_buf, u32 *resp_len)
    {
    pub ret: c_int,
// send command
    ret = ath6kl_usb_submit_ctrl_out(ar_usb, req_val, 0, 0,
    pub req_len): req_buf,,
    if (ret != 0)
    pub ret: return,
    if (resp_buf == core::ptr::null_mut()) {
// no expected response
    pub ret: return,
    }
// get response
    ret = ath6kl_usb_submit_ctrl_in(ar_usb, resp_val, 0, 0,
    pub resp_len): *mut resp_buf,,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_diag_read32(ar: *mut ath6kl, address: u32, data: *mut u32) -> c_int {
    static int ath6kl_usb_diag_read32(struct ath6kl *ar, u32 address, u32 *data)
    {
    pub ar->hif_priv: *mut *mut ath6kl_usb ar_usb =,
    pub resp: *mut ath6kl_usb_ctrl_diag_resp_read,
    pub cmd: *mut ath6kl_usb_ctrl_diag_cmd_read,
    pub resp_len: u32,
    pub ret: c_int,
    pub ar_usb->diag_cmd_buffer: *mut *mut cmd = (struct ath6kl_usb_ctrl_diag_cmd_read ),
    pub sizeof(*cmd)): *mut memset(cmd, 0,,
    pub ATH6KL_USB_CTRL_DIAG_CC_READ: cmd->cmd =,
    pub cpu_to_le32(address): cmd->address =,
    pub sizeof(*resp): *mut resp_len =,
    ret = ath6kl_usb_ctrl_msg_exchange(ar_usb,
    ATH6KL_USB_CONTROL_REQ_DIAG_CMD,
    (u8 *) cmd,
    sizeof(struct ath6kl_usb_ctrl_diag_cmd_write),
    ATH6KL_USB_CONTROL_REQ_DIAG_RESP,
    pub &resp_len): ar_usb->diag_resp_buffer,,
    if (ret) {
    pub ret): ath6kl_warn("diag read32 failed: %d\n",,
    pub ret: return,
    }
    resp = (struct ath6kl_usb_ctrl_diag_resp_read *)
// data = le32_to_cpu(resp->value);
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_diag_write32(ar: *mut ath6kl, address: u32, data: __le32) -> c_int {
    static int ath6kl_usb_diag_write32(struct ath6kl *ar, u32 address, __le32 data)
    {
    pub ar->hif_priv: *mut *mut ath6kl_usb ar_usb =,
    pub cmd: *mut ath6kl_usb_ctrl_diag_cmd_write,
    pub ret: c_int,
    pub ar_usb->diag_cmd_buffer: *mut *mut cmd = (struct ath6kl_usb_ctrl_diag_cmd_write ),
    pub ath6kl_usb_ctrl_diag_cmd_write)): memset(cmd, 0, sizeof(struct,
    pub cpu_to_le32(ATH6KL_USB_CTRL_DIAG_CC_WRITE): cmd->cmd =,
    pub cpu_to_le32(address): cmd->address =,
    pub data: cmd->value =,
    ret = ath6kl_usb_ctrl_msg_exchange(ar_usb,
    ATH6KL_USB_CONTROL_REQ_DIAG_CMD,
    (u8 *) cmd,
    sizeof(*cmd),
    pub NULL): 0, NULL,,
    if (ret) {
    pub ret): ath6kl_warn("diag_write32 failed: %d\n",,
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_bmi_read(ar: *mut ath6kl, buf: *mut u8, len: u32) -> c_int {
    static int ath6kl_usb_bmi_read(struct ath6kl *ar, u8 *buf, u32 len)
    {
    pub ar->hif_priv: *mut *mut ath6kl_usb ar_usb =,
    pub ret: c_int,
// get response
    ret = ath6kl_usb_submit_ctrl_in(ar_usb,
    ATH6KL_USB_CONTROL_REQ_RECV_BMI_RESP,
    pub len): 0, 0, buf,,
    if (ret) {
    ath6kl_err("Unable to read the bmi data from the device: %d\n",
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_bmi_write(ar: *mut ath6kl, buf: *mut u8, len: u32) -> c_int {
    static int ath6kl_usb_bmi_write(struct ath6kl *ar, u8 *buf, u32 len)
    {
    pub ar->hif_priv: *mut *mut ath6kl_usb ar_usb =,
    pub ret: c_int,
// send command
    ret = ath6kl_usb_submit_ctrl_out(ar_usb,
    ATH6KL_USB_CONTROL_REQ_SEND_BMI_CMD,
    pub len): 0, 0, buf,,
    if (ret) {
    ath6kl_err("unable to send the bmi data to the device: %d\n",
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_power_on(ar: *mut ath6kl) -> c_int {
    static int ath6kl_usb_power_on(struct ath6kl *ar)
    {
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_power_off(ar: *mut ath6kl) -> c_int {
    static int ath6kl_usb_power_off(struct ath6kl *ar)
    {
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_stop(ar: *mut ath6kl) {
    static void ath6kl_usb_stop(struct ath6kl *ar)
    {
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_cleanup_scatter(ar: *mut ath6kl) {
    static void ath6kl_usb_cleanup_scatter(struct ath6kl *ar)
    {
//
// USB doesn't support it. Just return.
//
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_suspend(ar: *mut ath6kl, wow: *mut cfg80211_wowlan) -> c_int {
    static int ath6kl_usb_suspend(struct ath6kl *ar, struct cfg80211_wowlan *wow)
    {
//
// cfg80211 suspend/WOW currently not supported for USB.
//
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_resume(ar: *mut ath6kl) -> c_int {
    static int ath6kl_usb_resume(struct ath6kl *ar)
    {
//
// cfg80211 resume currently not supported for USB.
//
    pub 0: return,
    }
    static const struct ath6kl_hif_ops ath6kl_usb_ops = {
    .diag_read32 = ath6kl_usb_diag_read32,
    .diag_write32 = ath6kl_usb_diag_write32,
    .bmi_read = ath6kl_usb_bmi_read,
    .bmi_write = ath6kl_usb_bmi_write,
    .power_on = ath6kl_usb_power_on,
    .power_off = ath6kl_usb_power_off,
    .stop = ath6kl_usb_stop,
    .pipe_send = ath6kl_usb_send,
    .pipe_get_default = ath6kl_usb_get_default_pipe,
    .pipe_map_service = ath6kl_usb_map_service_pipe,
    .pipe_get_free_queue_number = ath6kl_usb_get_free_queue_number,
    .cleanup_scatter = ath6kl_usb_cleanup_scatter,
    .suspend = ath6kl_usb_suspend,
    .resume = ath6kl_usb_resume,
}

// ath6kl usb driver registered functions
    static int ath6kl_usb_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct usb_device *dev = interface_to_usbdev(interface);
    struct ath6kl *ar;
    struct ath6kl_usb *ar_usb = core::ptr::null_mut();
    int vendor_id, product_id;
    let mut ret: c_int = 0;
    vendor_id = le16_to_cpu(dev.descriptor.idVendor);
    product_id = le16_to_cpu(dev.descriptor.idProduct);
    ath6kl_dbg(ATH6KL_DBG_USB, "vendor_id = %04x\n", vendor_id);
    ath6kl_dbg(ATH6KL_DBG_USB, "product_id = %04x\n", product_id);
    if (interface.cur_altsetting)
    ath6kl_dbg(ATH6KL_DBG_USB, "USB Interface %d\n",
    interface.cur_altsetting.desc.bInterfaceNumber);
    if (dev.speed == USB_SPEED_HIGH)
    ath6kl_dbg(ATH6KL_DBG_USB, "USB 2.0 Host\n");
    else
    ath6kl_dbg(ATH6KL_DBG_USB, "USB 1.1 Host\n");
    ar_usb = ath6kl_usb_create(interface);
    if (ar_usb == core::ptr::null_mut())
    return -ENOMEM;
    ar = ath6kl_core_create(&ar_usb.udev.dev);
    if (ar == core::ptr::null_mut()) {
    ath6kl_err("Failed to alloc ath6kl core\n");
    ret = -ENOMEM;
    goto err_usb_destroy;
    }
    ar.hif_priv = ar_usb;
    ar.hif_type = ATH6KL_HIF_TYPE_USB;
    ar.hif_ops = &ath6kl_usb_ops;
    ar.mbox_info.block_size = 16;
    ar.bmi.max_data_size = 252;
    ar_usb.ar = ar;
    ret = ath6kl_core_init(ar, ATH6KL_HTC_TYPE_PIPE);
    if (ret) {
    ath6kl_err("Failed to init ath6kl core: %d\n", ret);
    goto err_core_free;
    }
    return ret;
    err_core_free:
    ath6kl_core_destroy(ar);
    err_usb_destroy:
    ath6kl_usb_destroy(ar_usb);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_disconnect(interface: *mut usb_interface) {
    static void ath6kl_usb_disconnect(struct usb_interface *interface)
    {
    ath6kl_usb_device_detached(interface);
    }

    static int ath6kl_usb_pm_suspend(struct usb_interface *interface,
    pm_message_t message)
    {
    struct ath6kl_usb *device;
    device = usb_get_intfdata(interface);
    ath6kl_usb_flush_all(device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ath6kl_usb_pm_resume(interface: *mut usb_interface) -> c_int {
    static int ath6kl_usb_pm_resume(struct usb_interface *interface)
    {
    struct ath6kl_usb *device;
    device = usb_get_intfdata(interface);
    ath6kl_usb_post_recv_transfers(&device.pipes[ATH6KL_USB_PIPE_RX_DATA],
    ATH6KL_USB_RX_BUFFER_SIZE);
    ath6kl_usb_post_recv_transfers(&device.pipes[ATH6KL_USB_PIPE_RX_DATA2],
    ATH6KL_USB_RX_BUFFER_SIZE);
    return 0;
    }

// table of devices that work with this driver
    static const struct usb_device_id ath6kl_usb_ids[] = {
    {USB_DEVICE(0x0cf3, 0x9375)},
    {USB_DEVICE(0x0cf3, 0x9374)},
    {USB_DEVICE(0x04da, 0x390d)},
    { /* Terminating entry */ },
    };
    MODULE_DEVICE_TABLE(usb, ath6kl_usb_ids);
    static struct usb_driver ath6kl_usb_driver = {
    .name = "ath6kl_usb",
    .probe = ath6kl_usb_probe,
    .suspend = ath6kl_usb_pm_suspend,
    .resume = ath6kl_usb_pm_resume,
    .disconnect = ath6kl_usb_disconnect,
    .id_table = ath6kl_usb_ids,
    .supports_autosuspend = true,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(ath6kl_usb_driver);
    MODULE_AUTHOR("Atheros Communications, Inc.");
    MODULE_DESCRIPTION("Driver support for Atheros AR600x USB devices");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_FIRMWARE(AR6004_HW_1_0_FIRMWARE_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_0_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_0_DEFAULT_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_1_FIRMWARE_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_1_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_1_DEFAULT_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_2_FIRMWARE_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_2_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_2_DEFAULT_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_3_FW_DIR "/" AR6004_HW_1_3_FIRMWARE_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_3_BOARD_DATA_FILE);
    MODULE_FIRMWARE(AR6004_HW_1_3_DEFAULT_BOARD_DATA_FILE);
