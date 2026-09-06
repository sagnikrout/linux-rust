//! Automatically rewritten from C to Rust
//! Source: drivers/media/dvb-core/dvb_ringbuffer.c
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
// dvb_ringbuffer.c: ring buffer implementation for the dvb driver
//
// Copyright (C) 2003 Oliver Endriss
// Copyright (C) 2004 Andrew de Quincey
//
// based on code originally found in av7110.c & dvb_ci.c:
// Copyright (C) 1999-2003 Ralph  Metzler
// & Marcus Metzler for convergence integrated media GmbH
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//

pub const PKT_READY: c_int = 0;
pub const PKT_DISPOSED: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_init(rbuf: *mut dvb_ringbuffer, data: *mut c_void, len: usize) {
    void dvb_ringbuffer_init(struct dvb_ringbuffer *rbuf, void *data, size_t len)
    {
    rbuf.pread = 0;
    rbuf.pwrite = 0;
    rbuf.data = data;
    rbuf.size = len;
    rbuf.error = 0;
    init_waitqueue_head(&rbuf.queue);
    spin_lock_init(&(rbuf.lock));
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_empty(rbuf: *mut dvb_ringbuffer) -> c_int {
    int dvb_ringbuffer_empty(struct dvb_ringbuffer *rbuf)
    {
// smp_load_acquire() to load write pointer on reader side
// this pairs with smp_store_release() in dvb_ringbuffer_write(),
// dvb_ringbuffer_write_user(), or dvb_ringbuffer_reset()
//
// for memory barriers also see Documentation/core-api/circular-buffers.rst
//
    return (rbuf.pread == smp_load_acquire(&rbuf.pwrite));
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_free(rbuf: *mut dvb_ringbuffer) -> isize {
    ssize_t dvb_ringbuffer_free(struct dvb_ringbuffer *rbuf)
    {
    ssize_t free;
// READ_ONCE() to load read pointer on writer side
// this pairs with smp_store_release() in dvb_ringbuffer_read(),
// dvb_ringbuffer_read_user(), dvb_ringbuffer_flush(),
// or dvb_ringbuffer_reset()
//
    free = READ_ONCE(rbuf.pread) - rbuf.pwrite;
    if (free <= 0)
    free += rbuf.size;
    return free-1;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_avail(rbuf: *mut dvb_ringbuffer) -> isize {
    ssize_t dvb_ringbuffer_avail(struct dvb_ringbuffer *rbuf)
    {
    ssize_t avail;
// smp_load_acquire() to load write pointer on reader side
// this pairs with smp_store_release() in dvb_ringbuffer_write(),
// dvb_ringbuffer_write_user(), or dvb_ringbuffer_reset()
//
    avail = smp_load_acquire(&rbuf.pwrite) - rbuf.pread;
    if (avail < 0)
    avail += rbuf.size;
    return avail;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_flush(rbuf: *mut dvb_ringbuffer) {
    void dvb_ringbuffer_flush(struct dvb_ringbuffer *rbuf)
    {
// dvb_ringbuffer_flush() counts as read operation
// smp_load_acquire() to load write pointer
// smp_store_release() to update read pointer, this ensures that the
// correct pointer is visible for subsequent dvb_ringbuffer_free()
// calls on other cpu cores
//
    smp_store_release(&rbuf.pread, smp_load_acquire(&rbuf.pwrite));
    rbuf.error = 0;
    }
    EXPORT_SYMBOL(dvb_ringbuffer_flush);
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_reset(rbuf: *mut dvb_ringbuffer) {
    void dvb_ringbuffer_reset(struct dvb_ringbuffer *rbuf)
    {
// dvb_ringbuffer_reset() counts as read and write operation
// smp_store_release() to update read pointer
//
    smp_store_release(&rbuf.pread, 0);
// smp_store_release() to update write pointer
    smp_store_release(&rbuf.pwrite, 0);
    rbuf.error = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_flush_spinlock_wakeup(rbuf: *mut dvb_ringbuffer) {
    void dvb_ringbuffer_flush_spinlock_wakeup(struct dvb_ringbuffer *rbuf)
    {
    unsigned long flags;
    spin_lock_irqsave(&rbuf.lock, flags);
    dvb_ringbuffer_flush(rbuf);
    spin_unlock_irqrestore(&rbuf.lock, flags);
    wake_up(&rbuf.queue);
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_read_user(rbuf: *mut dvb_ringbuffer, buf: *mut u8 __user, len: usize) -> isize {
    ssize_t dvb_ringbuffer_read_user(struct dvb_ringbuffer *rbuf, u8 __user *buf, size_t len)
    {
    let mut todo: usize = len;
    size_t split;
    split = (rbuf.pread + len > rbuf.size) ? rbuf.size - rbuf.pread : 0;
    if (split > 0) {
    if (copy_to_user(buf, rbuf.data+rbuf.pread, split))
    return -EFAULT;
    buf += split;
    todo -= split;
// smp_store_release() for read pointer update to ensure
// that buf is not overwritten until read is complete,
// this pairs with READ_ONCE() in dvb_ringbuffer_free()
//
    smp_store_release(&rbuf.pread, 0);
    }
    if (copy_to_user(buf, rbuf.data+rbuf.pread, todo))
    return -EFAULT;
// smp_store_release() to update read pointer, see above
    smp_store_release(&rbuf.pread, (rbuf.pread + todo) % rbuf.size);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_read(rbuf: *mut dvb_ringbuffer, buf: *mut u8, len: usize) {
    void dvb_ringbuffer_read(struct dvb_ringbuffer *rbuf, u8 *buf, size_t len)
    {
    let mut todo: usize = len;
    size_t split;
    split = (rbuf.pread + len > rbuf.size) ? rbuf.size - rbuf.pread : 0;
    if (split > 0) {
    memcpy(buf, rbuf.data+rbuf.pread, split);
    buf += split;
    todo -= split;
// smp_store_release() for read pointer update to ensure
// that buf is not overwritten until read is complete,
// this pairs with READ_ONCE() in dvb_ringbuffer_free()
//
    smp_store_release(&rbuf.pread, 0);
    }
    memcpy(buf, rbuf.data+rbuf.pread, todo);
// smp_store_release() to update read pointer, see above
    smp_store_release(&rbuf.pread, (rbuf.pread + todo) % rbuf.size);
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_write(rbuf: *mut dvb_ringbuffer, buf: *const u8, len: usize) -> isize {
    ssize_t dvb_ringbuffer_write(struct dvb_ringbuffer *rbuf, const u8 *buf, size_t len)
    {
    let mut todo: usize = len;
    size_t split;
    split = (rbuf.pwrite + len > rbuf.size) ? rbuf.size - rbuf.pwrite : 0;
    if (split > 0) {
    memcpy(rbuf.data+rbuf.pwrite, buf, split);
    buf += split;
    todo -= split;
// smp_store_release() for write pointer update to ensure that
// written data is visible on other cpu cores before the pointer
// update, this pairs with smp_load_acquire() in
// dvb_ringbuffer_empty() or dvb_ringbuffer_avail()
//
    smp_store_release(&rbuf.pwrite, 0);
    }
    memcpy(rbuf.data+rbuf.pwrite, buf, todo);
// smp_store_release() for write pointer update, see above
    smp_store_release(&rbuf.pwrite, (rbuf.pwrite + todo) % rbuf.size);
    return len;
    }
    ssize_t dvb_ringbuffer_write_user(struct dvb_ringbuffer *rbuf,
    const u8 __user *buf, size_t len)
    {
    int status;
    let mut todo: usize = len;
    size_t split;
    split = (rbuf.pwrite + len > rbuf.size) ? rbuf.size - rbuf.pwrite : 0;
    if (split > 0) {
    status = copy_from_user(rbuf.data+rbuf.pwrite, buf, split);
    if (status)
    return len - todo;
    buf += split;
    todo -= split;
// smp_store_release() for write pointer update to ensure that
// written data is visible on other cpu cores before the pointer
// update, this pairs with smp_load_acquire() in
// dvb_ringbuffer_empty() or dvb_ringbuffer_avail()
//
    smp_store_release(&rbuf.pwrite, 0);
    }
    status = copy_from_user(rbuf.data+rbuf.pwrite, buf, todo);
    if (status)
    return len - todo;
// smp_store_release() for write pointer update, see above
    smp_store_release(&rbuf.pwrite, (rbuf.pwrite + todo) % rbuf.size);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_pkt_write(rbuf: *mut dvb_ringbuffer, buf: *mut u8, len: usize) -> isize {
    ssize_t dvb_ringbuffer_pkt_write(struct dvb_ringbuffer *rbuf, u8 *buf, size_t len)
    {
    int status;
    let mut oldpwrite: isize = rbuf.pwrite;
    DVB_RINGBUFFER_WRITE_BYTE(rbuf, len >> 8);
    DVB_RINGBUFFER_WRITE_BYTE(rbuf, len & 0xff);
    DVB_RINGBUFFER_WRITE_BYTE(rbuf, PKT_READY);
    status = dvb_ringbuffer_write(rbuf, buf, len);
    if (status < 0)
    rbuf.pwrite = oldpwrite;
    return status;
    }
    ssize_t dvb_ringbuffer_pkt_read_user(struct dvb_ringbuffer *rbuf, size_t idx,
    int offset, u8 __user *buf, size_t len)
    {
    size_t todo;
    size_t split;
    size_t pktlen;
    pktlen = rbuf.data[idx] << 8;
    pktlen |= rbuf.data[(idx + 1) % rbuf.size];
    if (offset > pktlen)
    return -EINVAL;
    if ((offset + len) > pktlen)
    len = pktlen - offset;
    idx = (idx + DVB_RINGBUFFER_PKTHDRSIZE + offset) % rbuf.size;
    todo = len;
    split = ((idx + len) > rbuf.size) ? rbuf.size - idx : 0;
    if (split > 0) {
    if (copy_to_user(buf, rbuf.data+idx, split))
    return -EFAULT;
    buf += split;
    todo -= split;
    idx = 0;
    }
    if (copy_to_user(buf, rbuf.data+idx, todo))
    return -EFAULT;
    return len;
    }
    ssize_t dvb_ringbuffer_pkt_read(struct dvb_ringbuffer *rbuf, size_t idx,
    int offset, u8 *buf, size_t len)
    {
    size_t todo;
    size_t split;
    size_t pktlen;
    pktlen = rbuf.data[idx] << 8;
    pktlen |= rbuf.data[(idx + 1) % rbuf.size];
    if (offset > pktlen)
    return -EINVAL;
    if ((offset + len) > pktlen)
    len = pktlen - offset;
    idx = (idx + DVB_RINGBUFFER_PKTHDRSIZE + offset) % rbuf.size;
    todo = len;
    split = ((idx + len) > rbuf.size) ? rbuf.size - idx : 0;
    if (split > 0) {
    memcpy(buf, rbuf.data+idx, split);
    buf += split;
    todo -= split;
    idx = 0;
    }
    memcpy(buf, rbuf.data+idx, todo);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_pkt_dispose(rbuf: *mut dvb_ringbuffer, idx: usize) {
    void dvb_ringbuffer_pkt_dispose(struct dvb_ringbuffer *rbuf, size_t idx)
    {
    size_t pktlen;
    rbuf.data[(idx + 2) % rbuf.size] = PKT_DISPOSED;
// clean up disposed packets
    while (dvb_ringbuffer_avail(rbuf) > DVB_RINGBUFFER_PKTHDRSIZE) {
    if (DVB_RINGBUFFER_PEEK(rbuf, 2) == PKT_DISPOSED) {
    pktlen = DVB_RINGBUFFER_PEEK(rbuf, 0) << 8;
    pktlen |= DVB_RINGBUFFER_PEEK(rbuf, 1);
    DVB_RINGBUFFER_SKIP(rbuf, pktlen + DVB_RINGBUFFER_PKTHDRSIZE);
    } else {
// first packet is not disposed, so we stop cleaning now
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_ringbuffer_pkt_next(rbuf: *mut dvb_ringbuffer, idx: usize, pktlen: *mut usize) -> isize {
    ssize_t dvb_ringbuffer_pkt_next(struct dvb_ringbuffer *rbuf, size_t idx, size_t *pktlen)
    {
    int consumed;
    int curpktlen;
    int curpktstatus;
    if (idx == -1) {
    idx = rbuf.pread;
    } else {
    curpktlen = rbuf.data[idx] << 8;
    curpktlen |= rbuf.data[(idx + 1) % rbuf.size];
    idx = (idx + curpktlen + DVB_RINGBUFFER_PKTHDRSIZE) % rbuf.size;
    }
    consumed = (idx - rbuf.pread);
    if (consumed < 0)
    consumed += rbuf.size;
    while ((dvb_ringbuffer_avail(rbuf) - consumed) > DVB_RINGBUFFER_PKTHDRSIZE) {
    curpktlen = rbuf.data[idx] << 8;
    curpktlen |= rbuf.data[(idx + 1) % rbuf.size];
    curpktstatus = rbuf.data[(idx + 2) % rbuf.size];
    if (curpktstatus == PKT_READY) {
// pktlen = curpktlen;
    return idx;
    }
    consumed += curpktlen + DVB_RINGBUFFER_PKTHDRSIZE;
    idx = (idx + curpktlen + DVB_RINGBUFFER_PKTHDRSIZE) % rbuf.size;
    }
// no packets available
    return -1;
    }
    EXPORT_SYMBOL(dvb_ringbuffer_init);
    EXPORT_SYMBOL(dvb_ringbuffer_empty);
    EXPORT_SYMBOL(dvb_ringbuffer_free);
    EXPORT_SYMBOL(dvb_ringbuffer_avail);
    EXPORT_SYMBOL(dvb_ringbuffer_flush_spinlock_wakeup);
    EXPORT_SYMBOL(dvb_ringbuffer_read_user);
    EXPORT_SYMBOL(dvb_ringbuffer_read);
    EXPORT_SYMBOL(dvb_ringbuffer_write);
    EXPORT_SYMBOL(dvb_ringbuffer_write_user);
