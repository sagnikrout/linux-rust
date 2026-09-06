//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/usb-urb.c
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


// SPDX-License-Identifier: GPL-2.0
// usb-urb.c is part of the DVB USB library.
//
// Copyright (C) 2004-6 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// This file keeps functions for initializing and handling the
// BULK and ISOC USB data transfers in a generic way.
// Can be used for DVB-only and also, that's the plan, for
// Hybrid USB devices (analog and DVB).
//

// URB stuff for streaming
#[no_mangle]
unsafe extern "C" fn usb_urb_complete(urb: *mut urb) {
    static void usb_urb_complete(struct urb *urb)
    {
    struct usb_data_stream *stream = urb.context;
    let mut ptype: c_int = usb_pipetype(urb.pipe);
    int i;
    u8 *b;
    deb_uxfer("'%s' urb completed. status: %d, length: %d/%d, pack_num: %d, errors: %d\n",
    ptype == PIPE_ISOCHRONOUS ? "isoc" : "bulk",
    urb.status,urb.actual_length,urb.transfer_buffer_length,
    urb.number_of_packets,urb.error_count);
    switch (urb.status) {
    case 0:         /* success */
    case -ETIMEDOUT:    /* NAK */
    break;
    case -ECONNRESET:   /* kill */
    case -ENOENT:
    case -ESHUTDOWN:
    return;
    default:        /* error */
    deb_ts("urb completion error %d.\n", urb.status);
    break;
    }
    b = (u8 *) urb.transfer_buffer;
    switch (ptype) {
    case PIPE_ISOCHRONOUS:
    for (i = 0; i < urb.number_of_packets; i++) {
    if (urb.iso_frame_desc[i].status != 0)
    deb_ts("iso frame descriptor has an error: %d\n",urb.iso_frame_desc[i].status);
#[no_mangle]
pub unsafe extern "C" fn if(0: urb->iso_frame_desc[i].actual_length >) -> else {
    else if (urb.iso_frame_desc[i].actual_length > 0)
    stream.complete(stream, b + urb.iso_frame_desc[i].offset, urb.iso_frame_desc[i].actual_length);
    urb.iso_frame_desc[i].status = 0;
    urb.iso_frame_desc[i].actual_length = 0;
    }
    debug_dump(b,20,deb_uxfer);
    break;
    case PIPE_BULK:
    if (urb.actual_length > 0)
    stream.complete(stream, b, urb.actual_length);
    break;
    default:
    err("unknown endpoint type in completion handler.");
    return;
    }
    usb_submit_urb(urb,GFP_ATOMIC);
    }
#[no_mangle]
pub unsafe extern "C" fn usb_urb_kill(stream: *mut usb_data_stream) -> c_int {
    int usb_urb_kill(struct usb_data_stream *stream)
    {
    int i;
    for (i = 0; i < stream.urbs_submitted; i++) {
    deb_ts("killing URB no. %d.\n",i);
// stop the URB
    usb_kill_urb(stream.urb_list[i]);
    }
    stream.urbs_submitted = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn usb_urb_submit(stream: *mut usb_data_stream) -> c_int {
    int usb_urb_submit(struct usb_data_stream *stream)
    {
    int i,ret;
    for (i = 0; i < stream.urbs_initialized; i++) {
    deb_ts("submitting URB no. %d\n",i);
    if ((ret = usb_submit_urb(stream.urb_list[i],GFP_ATOMIC))) {
    err("could not submit URB no. %d - get them all back",i);
    usb_urb_kill(stream);
    return ret;
    }
    stream.urbs_submitted++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_free_stream_buffers(stream: *mut usb_data_stream) -> c_int {
    static int usb_free_stream_buffers(struct usb_data_stream *stream)
    {
    if (stream.state & USB_STATE_URB_BUF) {
    while (stream.buf_num) {
    stream.buf_num--;
    deb_mem("freeing buffer %d\n",stream.buf_num);
    usb_free_coherent(stream.udev, stream.buf_size,
    stream.buf_list[stream.buf_num],
    stream.dma_addr[stream.buf_num]);
    }
    }
    stream.state &= ~USB_STATE_URB_BUF;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_allocate_stream_buffers(stream: *mut usb_data_stream, num: c_int, size: c_ulong) -> c_int {
    static int usb_allocate_stream_buffers(struct usb_data_stream *stream, int num, unsigned long size)
    {
    stream.buf_num = 0;
    stream.buf_size = size;
    deb_mem("all in all I will use %lu bytes for streaming\n",num*size);
    for (stream.buf_num = 0; stream.buf_num < num; stream.buf_num++) {
    deb_mem("allocating buffer %d\n",stream.buf_num);
    if (( stream.buf_list[stream.buf_num] =
    usb_alloc_coherent(stream.udev, size, GFP_KERNEL,
    &stream.dma_addr[stream.buf_num]) ) == core::ptr::null_mut()) {
    deb_mem("not enough memory for urb-buffer allocation.\n");
    usb_free_stream_buffers(stream);
    return -ENOMEM;
    }
    deb_mem("buffer %d: %p (dma: %Lu)\n",
    stream.buf_num,
    stream.buf_list[stream.buf_num], (long long)stream.dma_addr[stream.buf_num]);
    memset(stream.buf_list[stream.buf_num],0,size);
    stream.state |= USB_STATE_URB_BUF;
    }
    deb_mem("allocation successful\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_bulk_urb_init(stream: *mut usb_data_stream) -> c_int {
    static int usb_bulk_urb_init(struct usb_data_stream *stream)
    {
    int i, j;
    if ((i = usb_allocate_stream_buffers(stream,stream.props.count,
    stream.props.u.bulk.buffersize)) < 0)
    return i;
// allocate the URBs
    for (i = 0; i < stream.props.count; i++) {
    stream.urb_list[i] = usb_alloc_urb(0, GFP_KERNEL);
    if (!stream.urb_list[i]) {
    deb_mem("not enough memory for urb_alloc_urb!.\n");
    for (j = 0; j < i; j++)
    usb_free_urb(stream.urb_list[j]);
    return -ENOMEM;
    }
    usb_fill_bulk_urb( stream.urb_list[i], stream.udev,
    usb_rcvbulkpipe(stream.udev,stream.props.endpoint),
    stream.buf_list[i],
    stream.props.u.bulk.buffersize,
    usb_urb_complete, stream);
    stream.urb_list[i].transfer_flags = URB_NO_TRANSFER_DMA_MAP;
    stream.urb_list[i].transfer_dma = stream.dma_addr[i];
    stream.urbs_initialized++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_isoc_urb_init(stream: *mut usb_data_stream) -> c_int {
    static int usb_isoc_urb_init(struct usb_data_stream *stream)
    {
    int i,j;
    if ((i = usb_allocate_stream_buffers(stream,stream.props.count,
    stream.props.u.isoc.framesize*stream.props.u.isoc.framesperurb)) < 0)
    return i;
// allocate the URBs
    for (i = 0; i < stream.props.count; i++) {
    struct urb *urb;
    let mut frame_offset: c_int = 0;
    stream.urb_list[i] = usb_alloc_urb(stream.props.u.isoc.framesperurb, GFP_KERNEL);
    if (!stream.urb_list[i]) {
    deb_mem("not enough memory for urb_alloc_urb!\n");
    for (j = 0; j < i; j++)
    usb_free_urb(stream.urb_list[j]);
    return -ENOMEM;
    }
    urb = stream.urb_list[i];
    urb.dev = stream.udev;
    urb.context = stream;
    urb.complete = usb_urb_complete;
    urb.pipe = usb_rcvisocpipe(stream.udev,stream.props.endpoint);
    urb.transfer_flags = URB_ISO_ASAP | URB_NO_TRANSFER_DMA_MAP;
    urb.interval = stream.props.u.isoc.interval;
    urb.number_of_packets = stream.props.u.isoc.framesperurb;
    urb.transfer_buffer_length = stream.buf_size;
    urb.transfer_buffer = stream.buf_list[i];
    urb.transfer_dma = stream.dma_addr[i];
    for (j = 0; j < stream.props.u.isoc.framesperurb; j++) {
    urb.iso_frame_desc[j].offset = frame_offset;
    urb.iso_frame_desc[j].length = stream.props.u.isoc.framesize;
    frame_offset += stream.props.u.isoc.framesize;
    }
    stream.urbs_initialized++;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn usb_urb_init(stream: *mut usb_data_stream, props: *mut usb_data_stream_properties) -> c_int {
    int usb_urb_init(struct usb_data_stream *stream, struct usb_data_stream_properties *props)
    {
    if (stream == core::ptr::null_mut() || props == core::ptr::null_mut())
    return -EINVAL;
    memcpy(&stream.props, props, sizeof(*props));
    usb_clear_halt(stream.udev,usb_rcvbulkpipe(stream.udev,stream.props.endpoint));
    if (stream.complete == core::ptr::null_mut()) {
    err("there is no data callback - this doesn't make sense.");
    return -EINVAL;
    }
    switch (stream.props.type) {
    case USB_BULK:
    return usb_bulk_urb_init(stream);
    case USB_ISOC:
    return usb_isoc_urb_init(stream);
    default:
    err("unknown URB-type for data transfer.");
    return -EINVAL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn usb_urb_exit(stream: *mut usb_data_stream) -> c_int {
    int usb_urb_exit(struct usb_data_stream *stream)
    {
    int i;
    usb_urb_kill(stream);
    for (i = 0; i < stream.urbs_initialized; i++) {
    if (stream.urb_list[i] != core::ptr::null_mut()) {
    deb_mem("freeing URB no. %d.\n",i);
// free the URBs
    usb_free_urb(stream.urb_list[i]);
    }
    }
    stream.urbs_initialized = 0;
    usb_free_stream_buffers(stream);
    return 0;
    }
