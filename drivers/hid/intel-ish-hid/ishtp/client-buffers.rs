//! Automatically rewritten from C to Rust
//! Source: drivers/hid/intel-ish-hid/ishtp/client-buffers.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ISHTP Ring Buffers
//
// Copyright (c) 2003-2016, Intel Corporation.
//

//
// ishtp_cl_alloc_rx_ring() - Allocate RX ring buffers
// @cl: client device instance
//
// Allocate and initialize RX ring buffers
//
// Return: 0 on success else -ENOMEM
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_cl_alloc_rx_ring(cl: *mut ishtp_cl) -> c_int {
    int ishtp_cl_alloc_rx_ring(struct ishtp_cl *cl)
    {
    let mut len: usize = cl.device.fw_client.props.max_msg_length;
    int	j;
    struct ishtp_cl_rb *rb;
    let mut ret: c_int = 0;
    unsigned long	flags;
    for (j = 0; j < cl.rx_ring_size; ++j) {
    rb = ishtp_io_rb_init(cl);
    if (!rb) {
    ret = -ENOMEM;
    goto out;
    }
    ret = ishtp_io_rb_alloc_buf(rb, len);
    if (ret)
    goto out;
    spin_lock_irqsave(&cl.free_list_spinlock, flags);
    list_add_tail(&rb.list, &cl.free_rb_list.list);
    spin_unlock_irqrestore(&cl.free_list_spinlock, flags);
    }
    return	0;
    out:
    dev_err(&cl.device.dev, "error in allocating Rx buffers\n");
    ishtp_cl_free_rx_ring(cl);
    return	ret;
    }
//
// ishtp_cl_alloc_tx_ring() - Allocate TX ring buffers
// @cl: client device instance
//
// Allocate and initialize TX ring buffers
//
// Return: 0 on success else -ENOMEM
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_cl_alloc_tx_ring(cl: *mut ishtp_cl) -> c_int {
    int ishtp_cl_alloc_tx_ring(struct ishtp_cl *cl)
    {
    let mut len: usize = cl.device.fw_client.props.max_msg_length;
    int	j;
    unsigned long	flags;
    cl.tx_ring_free_size = 0;
// Allocate pool to free Tx bufs
    for (j = 0; j < cl.tx_ring_size; ++j) {
    struct ishtp_cl_tx_ring	*tx_buf;
    tx_buf = kzalloc_obj(struct ishtp_cl_tx_ring);
    if (!tx_buf)
    goto	out;
    tx_buf.send_buf.data = kmalloc(len, GFP_KERNEL);
    if (!tx_buf.send_buf.data) {
    kfree(tx_buf);
    goto	out;
    }
    spin_lock_irqsave(&cl.tx_free_list_spinlock, flags);
    list_add_tail(&tx_buf.list, &cl.tx_free_list.list);
    ++cl.tx_ring_free_size;
    spin_unlock_irqrestore(&cl.tx_free_list_spinlock, flags);
    }
    return	0;
    out:
    dev_err(&cl.device.dev, "error in allocating Tx pool\n");
    ishtp_cl_free_tx_ring(cl);
    return	-ENOMEM;
    }
//
// ishtp_cl_free_rx_ring() - Free RX ring buffers
// @cl: client device instance
//
// Free RX ring buffers
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_cl_free_rx_ring(cl: *mut ishtp_cl) {
    void ishtp_cl_free_rx_ring(struct ishtp_cl *cl)
    {
    struct ishtp_cl_rb *rb;
    unsigned long	flags;
// release allocated memory - pass over free_rb_list
    spin_lock_irqsave(&cl.free_list_spinlock, flags);
    while (!list_empty(&cl.free_rb_list.list)) {
    rb = list_entry(cl.free_rb_list.list.next, struct ishtp_cl_rb,
    list);
    list_del(&rb.list);
    kfree(rb.buffer.data);
    kfree(rb);
    }
    spin_unlock_irqrestore(&cl.free_list_spinlock, flags);
// release allocated memory - pass over in_process_list
    spin_lock_irqsave(&cl.in_process_spinlock, flags);
    while (!list_empty(&cl.in_process_list.list)) {
    rb = list_entry(cl.in_process_list.list.next,
    struct ishtp_cl_rb, list);
    list_del(&rb.list);
    kfree(rb.buffer.data);
    kfree(rb);
    }
    spin_unlock_irqrestore(&cl.in_process_spinlock, flags);
    }
//
// ishtp_cl_free_tx_ring() - Free TX ring buffers
// @cl: client device instance
//
// Free TX ring buffers
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_cl_free_tx_ring(cl: *mut ishtp_cl) {
    void ishtp_cl_free_tx_ring(struct ishtp_cl *cl)
    {
    struct ishtp_cl_tx_ring	*tx_buf;
    unsigned long	flags;
    spin_lock_irqsave(&cl.tx_free_list_spinlock, flags);
// release allocated memory - pass over tx_free_list
    while (!list_empty(&cl.tx_free_list.list)) {
    tx_buf = list_entry(cl.tx_free_list.list.next,
    struct ishtp_cl_tx_ring, list);
    list_del(&tx_buf.list);
    --cl.tx_ring_free_size;
    kfree(tx_buf.send_buf.data);
    kfree(tx_buf);
    }
    spin_unlock_irqrestore(&cl.tx_free_list_spinlock, flags);
    spin_lock_irqsave(&cl.tx_list_spinlock, flags);
// release allocated memory - pass over tx_list
    while (!list_empty(&cl.tx_list.list)) {
    tx_buf = list_entry(cl.tx_list.list.next,
    struct ishtp_cl_tx_ring, list);
    list_del(&tx_buf.list);
    kfree(tx_buf.send_buf.data);
    kfree(tx_buf);
    }
    spin_unlock_irqrestore(&cl.tx_list_spinlock, flags);
    }
//
// ishtp_io_rb_free() - Free IO request block
// @rb: IO request block
//
// Free io request block memory
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_io_rb_free(rb: *mut ishtp_cl_rb) {
    void ishtp_io_rb_free(struct ishtp_cl_rb *rb)
    {
    if (rb == core::ptr::null_mut())
    return;
    kfree(rb.buffer.data);
    kfree(rb);
    }
//
// ishtp_io_rb_init() - Allocate and init IO request block
// @cl: client device instance
//
// Allocate and initialize request block
//
// Return: Allocted IO request block pointer
//
    struct ishtp_cl_rb *ishtp_io_rb_init(struct ishtp_cl *cl)
    {
    struct ishtp_cl_rb *rb;
    rb = kzalloc_obj(struct ishtp_cl_rb);
    if (!rb)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&rb.list);
    rb.cl = cl;
    rb.buf_idx = 0;
    return rb;
    }
//
// ishtp_io_rb_alloc_buf() - Allocate and init response buffer
// @rb: IO request block
// @length: length of response buffer
//
// Allocate respose buffer
//
// Return: 0 on success else -ENOMEM
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_io_rb_alloc_buf(rb: *mut ishtp_cl_rb, length: usize) -> c_int {
    int ishtp_io_rb_alloc_buf(struct ishtp_cl_rb *rb, size_t length)
    {
    if (!rb)
    return -EINVAL;
    if (length == 0)
    return 0;
    rb.buffer.data = kmalloc(length, GFP_KERNEL);
    if (!rb.buffer.data)
    return -ENOMEM;
    rb.buffer.size = length;
    return 0;
    }
//
// ishtp_cl_io_rb_recycle() - Recycle IO request blocks
// @rb: IO request block
//
// Re-append rb to its client's free list and send flow control if needed
//
// Return: 0 on success else -EFAULT
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_cl_io_rb_recycle(rb: *mut ishtp_cl_rb) -> c_int {
    int ishtp_cl_io_rb_recycle(struct ishtp_cl_rb *rb)
    {
    struct ishtp_cl *cl;
    let mut rets: c_int = 0;
    unsigned long	flags;
    if (!rb || !rb.cl)
    return	-EFAULT;
    cl = rb.cl;
    spin_lock_irqsave(&cl.free_list_spinlock, flags);
    list_add_tail(&rb.list, &cl.free_rb_list.list);
    spin_unlock_irqrestore(&cl.free_list_spinlock, flags);
//
// If we returned the first buffer to empty 'free' list,
// send flow control
//
    if (!cl.out_flow_ctrl_creds)
    rets = ishtp_cl_read_start(cl);
    return	rets;
    }
    EXPORT_SYMBOL(ishtp_cl_io_rb_recycle);
//
// ishtp_cl_rx_get_rb() -Get a rb from client device rx buffer list
// @cl: Pointer to client device instance
//
// Check client device in-processing buffer list and get a rb from it.
//
// Return: rb pointer if buffer list isn't empty else NULL
//
    struct ishtp_cl_rb *ishtp_cl_rx_get_rb(struct ishtp_cl *cl)
    {
    unsigned long rx_flags;
    struct ishtp_cl_rb *rb;
    spin_lock_irqsave(&cl.in_process_spinlock, rx_flags);
    rb = list_first_entry_or_null(&cl.in_process_list.list,
    struct ishtp_cl_rb, list);
    if (rb)
    list_del_init(&rb.list);
    spin_unlock_irqrestore(&cl.in_process_spinlock, rx_flags);
    return rb;
    }
    EXPORT_SYMBOL(ishtp_cl_rx_get_rb);
