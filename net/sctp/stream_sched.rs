//! Automatically rewritten from C to Rust
//! Source: net/sctp/stream_sched.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// SCTP kernel implementation
// (C) Copyright Red Hat Inc. 2017
//
// This file is part of the SCTP kernel implementation
//
// These functions manipulate sctp stream queue/scheduling.
//
// Please send any bug reports or fixes you make to the
// email addresched(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Marcelo Ricardo Leitner <marcelo.leitner@gmail.com>
//

// First Come First Serve (a.k.a. FIFO)
// RFC DRAFT ndata Section 3.1
//
    static int sctp_sched_fcfs_set(struct sctp_stream *stream, __u16 sid,
    __u16 value, gfp_t gfp)
    {
    return 0;
    }
    static int sctp_sched_fcfs_get(struct sctp_stream *stream, __u16 sid,
    __u16 *value)
    {
// value = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fcfs_init(stream: *mut sctp_stream) -> c_int {
    static int sctp_sched_fcfs_init(struct sctp_stream *stream)
    {
    return 0;
    }
    static int sctp_sched_fcfs_init_sid(struct sctp_stream *stream, __u16 sid,
    gfp_t gfp)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fcfs_free_sid(stream: *mut sctp_stream, sid: __u16) {
    static void sctp_sched_fcfs_free_sid(struct sctp_stream *stream, __u16 sid)
    {
    }
    static void sctp_sched_fcfs_enqueue(struct sctp_outq *q,
    struct sctp_datamsg *msg)
    {
    }
    static struct sctp_chunk *sctp_sched_fcfs_dequeue(struct sctp_outq *q)
    {
    struct sctp_stream *stream = &q.asoc.stream;
    struct sctp_chunk *ch = core::ptr::null_mut();
    struct list_head *entry;
    if (list_empty(&q.out_chunk_list))
    goto out;
    if (stream.out_curr) {
    ch = list_entry(stream.out_curr.ext.outq.next,
    struct sctp_chunk, stream_list);
    } else {
    entry = q.out_chunk_list.next;
    ch = list_entry(entry, struct sctp_chunk, list);
    }
    sctp_sched_dequeue_common(q, ch);
    out:
    return ch;
    }
    static void sctp_sched_fcfs_dequeue_done(struct sctp_outq *q,
    struct sctp_chunk *chunk)
    {
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fcfs_sched_all(stream: *mut sctp_stream) {
    static void sctp_sched_fcfs_sched_all(struct sctp_stream *stream)
    {
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fcfs_unsched_all(stream: *mut sctp_stream) {
    static void sctp_sched_fcfs_unsched_all(struct sctp_stream *stream)
    {
    }
    static const struct sctp_sched_ops sctp_sched_fcfs = {
    .set = sctp_sched_fcfs_set,
    .get = sctp_sched_fcfs_get,
    .init = sctp_sched_fcfs_init,
    .init_sid = sctp_sched_fcfs_init_sid,
    .free_sid = sctp_sched_fcfs_free_sid,
    .enqueue = sctp_sched_fcfs_enqueue,
    .dequeue = sctp_sched_fcfs_dequeue,
    .dequeue_done = sctp_sched_fcfs_dequeue_done,
    .sched_all = sctp_sched_fcfs_sched_all,
    .unsched_all = sctp_sched_fcfs_unsched_all,
    };
#[no_mangle]
unsafe extern "C" fn sctp_sched_ops_fcfs_init() {
    static void sctp_sched_ops_fcfs_init(void)
    {
    sctp_sched_ops_register(SCTP_SS_FCFS, &sctp_sched_fcfs);
    }
// API to other parts of the stack
    static const struct sctp_sched_ops *sctp_sched_ops[SCTP_SS_MAX + 1];
    void sctp_sched_ops_register(enum sctp_sched_type sched,
    const struct sctp_sched_ops *sched_ops)
    {
    sctp_sched_ops[sched] = sched_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_ops_init() {
    void sctp_sched_ops_init(void)
    {
    sctp_sched_ops_fcfs_init();
    sctp_sched_ops_prio_init();
    sctp_sched_ops_rr_init();
    sctp_sched_ops_fc_init();
    sctp_sched_ops_wfq_init();
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_free_sched(stream: *mut sctp_stream) {
    static void sctp_sched_free_sched(struct sctp_stream *stream)
    {
    const struct sctp_sched_ops *sched = sctp_sched_ops_from_stream(stream);
    struct sctp_stream_out_ext *soute;
    int i;
    sched.unsched_all(stream);
    for (i = 0; i < stream.outcnt; i++) {
    soute = SCTP_SO(stream, i).ext;
    if (!soute)
    continue;
    sched.free_sid(stream, i);
// Give the next scheduler a clean slate.
    memset_after(soute, 0, outq);
    }
    }
    int sctp_sched_set_sched(struct sctp_association *asoc,
    enum sctp_sched_type sched)
    {
    const struct sctp_sched_ops *old = asoc.outqueue.sched;
    struct sctp_datamsg *msg = core::ptr::null_mut();
    const struct sctp_sched_ops *n;
    struct sctp_chunk *ch;
    int i, ret = 0;
    if (sched > SCTP_SS_MAX)
    return -EINVAL;
    n = sctp_sched_ops[sched];
    if (old == n)
    return ret;
    if (old)
    sctp_sched_free_sched(&asoc.stream);
    asoc.outqueue.sched = n;
    n.init(&asoc.stream);
    for (i = 0; i < asoc.stream.outcnt; i++) {
    if (!SCTP_SO(&asoc.stream, i).ext)
    continue;
    ret = n.init_sid(&asoc.stream, i, GFP_ATOMIC);
    if (ret)
    goto err;
    }
// We have to requeue all chunks already queued.
    list_for_each_entry(ch, &asoc.outqueue.out_chunk_list, list) {
    if (ch.msg == msg)
    continue;
    msg = ch.msg;
    n.enqueue(&asoc.outqueue, msg);
    }
    return ret;
    err:
    sctp_sched_free_sched(&asoc.stream);
    asoc.outqueue.sched = &sctp_sched_fcfs; /* Always safe */
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_get_sched(asoc: *mut sctp_association) -> c_int {
    int sctp_sched_get_sched(struct sctp_association *asoc)
    {
    int i;
    for (i = 0; i <= SCTP_SS_MAX; i++)
    if (asoc.outqueue.sched == sctp_sched_ops[i])
    return i;
    return 0;
    }
    int sctp_sched_set_value(struct sctp_association *asoc, __u16 sid,
    __u16 value, gfp_t gfp)
    {
    if (sid >= asoc.stream.outcnt)
    return -EINVAL;
    if (!SCTP_SO(&asoc.stream, sid).ext) {
    int ret;
    ret = sctp_stream_init_ext(&asoc.stream, sid);
    if (ret)
    return ret;
    }
    return asoc.outqueue.sched.set(&asoc.stream, sid, value, gfp);
    }
    int sctp_sched_get_value(struct sctp_association *asoc, __u16 sid,
    __u16 *value)
    {
    if (sid >= asoc.stream.outcnt)
    return -EINVAL;
    if (!SCTP_SO(&asoc.stream, sid).ext)
    return 0;
    return asoc.outqueue.sched.get(&asoc.stream, sid, value);
    }
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    void sctp_sched_dequeue_done(struct sctp_outq *q, struct sctp_chunk *ch)
    {
    if (!list_is_last(&ch.frag_list, &ch.msg.chunks) &&
    !q.asoc.peer.intl_capable) {
    struct sctp_stream_out *sout;
    __u16 sid;
// datamsg is not finish, so save it as current one,
// in case application switch scheduler or a higher
// priority stream comes in.
//
    sid = sctp_chunk_stream_no(ch);
    sout = SCTP_SO(&q.asoc.stream, sid);
    q.asoc.stream.out_curr = sout;
    return;
    }
    q.asoc.stream.out_curr = core::ptr::null_mut();
    q.sched.dequeue_done(q, ch);
    }
// Auxiliary functions for the schedulers
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_dequeue_common(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    void sctp_sched_dequeue_common(struct sctp_outq *q, struct sctp_chunk *ch)
    {
    list_del_init(&ch.list);
    list_del_init(&ch.stream_list);
    q.out_qlen -= ch.skb.len;
    }
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_init_sid(stream: *mut sctp_stream, sid: __u16, gfp: gfp_t) -> c_int {
    int sctp_sched_init_sid(struct sctp_stream *stream, __u16 sid, gfp_t gfp)
    {
    const struct sctp_sched_ops *sched = sctp_sched_ops_from_stream(stream);
    struct sctp_stream_out_ext *ext = SCTP_SO(stream, sid).ext;
    INIT_LIST_HEAD(&ext.outq);
    return sched.init_sid(stream, sid, gfp);
    }
    const struct sctp_sched_ops *sctp_sched_ops_from_stream(struct sctp_stream *stream)
    {
    struct sctp_association *asoc;
    asoc = container_of(stream, struct sctp_association, stream);
    return asoc.outqueue.sched;
    }
