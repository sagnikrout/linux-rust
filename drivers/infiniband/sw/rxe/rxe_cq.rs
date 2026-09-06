//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/sw/rxe/rxe_cq.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//

    int rxe_cq_from_init(struct rxe_dev *rxe, struct rxe_cq *cq, int cqe,
    int comp_vector, struct ib_udata *udata,
    struct rxe_create_cq_resp __user *uresp)
    {
    int err;
    enum queue_type type;
    type = QUEUE_TYPE_TO_CLIENT;
    cq.queue = rxe_queue_init(rxe, &cqe,
    sizeof(struct rxe_cqe), type);
    if (!cq.queue) {
    rxe_dbg_dev(rxe, "unable to create cq\n");
    return -ENOMEM;
    }
    err = do_mmap_info(rxe, uresp ? &uresp.mi : core::ptr::null_mut(), udata,
    cq.queue.buf, cq.queue.buf_size, &cq.queue.ip);
    if (err)
    return err;
    cq.is_user = uresp;
    spin_lock_init(&cq.cq_lock);
    cq.ibcq.cqe = cqe;
    return 0;
    }
    int rxe_cq_resize_queue(struct rxe_cq *cq, int cqe,
    struct rxe_resize_cq_resp __user *uresp,
    struct ib_udata *udata)
    {
    int err;
    err = rxe_queue_resize(cq.queue, (unsigned int *)&cqe,
    sizeof(struct rxe_cqe), udata,
    uresp ? &uresp.mi : core::ptr::null_mut(), core::ptr::null_mut(), &cq.cq_lock);
    if (!err)
    cq.ibcq.cqe = cqe;
    return err;
    }
// caller holds reference to cq
#[no_mangle]
pub unsafe extern "C" fn rxe_cq_post(cq: *mut rxe_cq, cqe: *mut rxe_cqe, solicited: c_int) -> c_int {
    int rxe_cq_post(struct rxe_cq *cq, struct rxe_cqe *cqe, int solicited)
    {
    struct ib_event ev;
    int full;
    void *addr;
    unsigned long flags;
    spin_lock_irqsave(&cq.cq_lock, flags);
    full = queue_full(cq.queue, QUEUE_TYPE_TO_CLIENT);
    if (unlikely(full)) {
    rxe_err_cq(cq, "queue full\n");
    spin_unlock_irqrestore(&cq.cq_lock, flags);
    if (cq.ibcq.event_handler) {
    ev.device = cq.ibcq.device;
    ev.element.cq = &cq.ibcq;
    ev.event = IB_EVENT_CQ_ERR;
    cq.ibcq.event_handler(&ev, cq.ibcq.cq_context);
    }
    return -EBUSY;
    }
    addr = queue_producer_addr(cq.queue, QUEUE_TYPE_TO_CLIENT);
    memcpy(addr, cqe, sizeof(*cqe));
    queue_advance_producer(cq.queue, QUEUE_TYPE_TO_CLIENT);
    if ((cq.notify & IB_CQ_NEXT_COMP) ||
    (cq.notify & IB_CQ_SOLICITED && solicited)) {
    cq.notify = 0;
    cq.ibcq.comp_handler(&cq.ibcq, cq.ibcq.cq_context);
    }
    spin_unlock_irqrestore(&cq.cq_lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_cq_cleanup(elem: *mut rxe_pool_elem) {
    void rxe_cq_cleanup(struct rxe_pool_elem *elem)
    {
    struct rxe_cq *cq = container_of(elem, typeof(*cq), elem);
    if (cq.queue)
    rxe_queue_cleanup(cq.queue);
    }
