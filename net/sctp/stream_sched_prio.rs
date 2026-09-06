//! Automatically rewritten from C to Rust
//! Source: net/sctp/stream_sched_prio.c
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

// Priority handling
// RFC DRAFT ndata section 3.4
//
    static void sctp_sched_prio_unsched_all(struct sctp_stream *stream);
    static struct sctp_stream_priorities *sctp_sched_prio_head_get(struct sctp_stream_priorities *p)
    {
    p.users++;
    return p;
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_head_put(p: *mut sctp_stream_priorities) {
    static void sctp_sched_prio_head_put(struct sctp_stream_priorities *p)
    {
    if (p && --p.users == 0)
    kfree(p);
    }
    static struct sctp_stream_priorities *sctp_sched_prio_new_head(
    struct sctp_stream *stream, int prio, gfp_t gfp)
    {
    struct sctp_stream_priorities *p;
    p = kmalloc_obj(*p, gfp);
    if (!p)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&p.prio_sched);
    INIT_LIST_HEAD(&p.active);
    p.next = core::ptr::null_mut();
    p.prio = prio;
    p.users = 1;
    return p;
    }
    static struct sctp_stream_priorities *sctp_sched_prio_get_head(
    struct sctp_stream *stream, int prio, gfp_t gfp)
    {
    struct sctp_stream_priorities *p;
    int i;
// Look into scheduled priorities first, as they are sorted and
// we can find it fast IF it's scheduled.
//
    list_for_each_entry(p, &stream.prio_list, prio_sched) {
    if (p.prio == prio)
    return sctp_sched_prio_head_get(p);
    if (p.prio > prio)
    break;
    }
// No luck. So we search on all streams now.
    for (i = 0; i < stream.outcnt; i++) {
    if (!SCTP_SO(stream, i).ext)
    continue;
    p = SCTP_SO(stream, i).ext.prio_head;
    if (!p)
// Means all other streams won't be initialized
// as well.
//
    break;
    if (p.prio == prio)
    return sctp_sched_prio_head_get(p);
    }
// If not even there, allocate a new one.
    return sctp_sched_prio_new_head(stream, prio, gfp);
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_next_stream(p: *mut sctp_stream_priorities) {
    static void sctp_sched_prio_next_stream(struct sctp_stream_priorities *p)
    {
    struct list_head *pos;
    pos = p.next.prio_list.next;
    if (pos == &p.active)
    pos = pos.next;
    p.next = list_entry(pos, struct sctp_stream_out_ext, prio_list);
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_unsched(soute: *mut sctp_stream_out_ext) -> bool {
    static bool sctp_sched_prio_unsched(struct sctp_stream_out_ext *soute)
    {
    let mut scheduled: bool = false;
    if (!list_empty(&soute.prio_list)) {
    struct sctp_stream_priorities *prio_head = soute.prio_head;
// Scheduled
    scheduled = true;
    if (prio_head.next == soute)
// Try to move to the next stream
    sctp_sched_prio_next_stream(prio_head);
    list_del_init(&soute.prio_list);
// Also unsched the priority if this was the last stream
    if (list_empty(&prio_head.active)) {
    list_del_init(&prio_head.prio_sched);
// If there is no stream left, clear next
    prio_head.next = core::ptr::null_mut();
    }
    }
    return scheduled;
    }
    static void sctp_sched_prio_sched(struct sctp_stream *stream,
    struct sctp_stream_out_ext *soute)
    {
    struct sctp_stream_priorities *prio, *prio_head;
    prio_head = soute.prio_head;
// Nothing to do if already scheduled
    if (!list_empty(&soute.prio_list))
    return;
// Schedule the stream. If there is a next, we schedule the new
// one before it, so it's the last in round robin order.
// If there isn't, we also have to schedule the priority.
//
    if (prio_head.next) {
    list_add(&soute.prio_list, prio_head.next.prio_list.prev);
    return;
    }
    list_add(&soute.prio_list, &prio_head.active);
    prio_head.next = soute;
    list_for_each_entry(prio, &stream.prio_list, prio_sched) {
    if (prio.prio > prio_head.prio) {
    list_add(&prio_head.prio_sched, prio.prio_sched.prev);
    return;
    }
    }
    list_add_tail(&prio_head.prio_sched, &stream.prio_list);
    }
    static int sctp_sched_prio_set(struct sctp_stream *stream, __u16 sid,
    __u16 prio, gfp_t gfp)
    {
    struct sctp_stream_out *sout = SCTP_SO(stream, sid);
    struct sctp_stream_out_ext *soute = sout.ext;
    struct sctp_stream_priorities *prio_head, *old;
    let mut reschedule: bool = false;
    old = soute.prio_head;
    if (old && old.prio == prio)
    return 0;
    prio_head = sctp_sched_prio_get_head(stream, prio, gfp);
    if (!prio_head)
    return -ENOMEM;
    reschedule = sctp_sched_prio_unsched(soute);
    soute.prio_head = prio_head;
    if (reschedule)
    sctp_sched_prio_sched(stream, soute);
    sctp_sched_prio_head_put(old);
    return 0;
    }
    static int sctp_sched_prio_get(struct sctp_stream *stream, __u16 sid,
    __u16 *value)
    {
// value = SCTP_SO(stream, sid)->ext->prio_head->prio;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_init(stream: *mut sctp_stream) -> c_int {
    static int sctp_sched_prio_init(struct sctp_stream *stream)
    {
    INIT_LIST_HEAD(&stream.prio_list);
    return 0;
    }
    static int sctp_sched_prio_init_sid(struct sctp_stream *stream, __u16 sid,
    gfp_t gfp)
    {
    INIT_LIST_HEAD(&SCTP_SO(stream, sid).ext.prio_list);
    return sctp_sched_prio_set(stream, sid, 0, gfp);
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_free_sid(stream: *mut sctp_stream, sid: __u16) {
    static void sctp_sched_prio_free_sid(struct sctp_stream *stream, __u16 sid)
    {
    sctp_sched_prio_head_put(SCTP_SO(stream, sid).ext.prio_head);
    SCTP_SO(stream, sid).ext.prio_head = core::ptr::null_mut();
    }
    static void sctp_sched_prio_enqueue(struct sctp_outq *q,
    struct sctp_datamsg *msg)
    {
    struct sctp_stream *stream;
    struct sctp_chunk *ch;
    __u16 sid;
    ch = list_first_entry(&msg.chunks, struct sctp_chunk, frag_list);
    sid = sctp_chunk_stream_no(ch);
    stream = &q.asoc.stream;
    sctp_sched_prio_sched(stream, SCTP_SO(stream, sid).ext);
    }
    static struct sctp_chunk *sctp_sched_prio_dequeue(struct sctp_outq *q)
    {
    struct sctp_stream *stream = &q.asoc.stream;
    struct sctp_stream_priorities *prio;
    struct sctp_stream_out_ext *soute;
    struct sctp_chunk *ch = core::ptr::null_mut();
// Bail out quickly if queue is empty
    if (list_empty(&q.out_chunk_list))
    goto out;
// Find which chunk is next. It's easy, it's either the current
// one or the first chunk on the next active stream.
//
    if (stream.out_curr) {
    soute = stream.out_curr.ext;
    } else {
    prio = list_entry(stream.prio_list.next,
    struct sctp_stream_priorities, prio_sched);
    soute = prio.next;
    }
    ch = list_entry(soute.outq.next, struct sctp_chunk, stream_list);
    sctp_sched_dequeue_common(q, ch);
    out:
    return ch;
    }
    static void sctp_sched_prio_dequeue_done(struct sctp_outq *q,
    struct sctp_chunk *ch)
    {
    struct sctp_stream_priorities *prio;
    struct sctp_stream_out_ext *soute;
    __u16 sid;
// Last chunk on that msg, move to the next stream on
// this priority.
//
    sid = sctp_chunk_stream_no(ch);
    soute = SCTP_SO(&q.asoc.stream, sid).ext;
    prio = soute.prio_head;
    sctp_sched_prio_next_stream(prio);
    if (list_empty(&soute.outq))
    sctp_sched_prio_unsched(soute);
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_sched_all(stream: *mut sctp_stream) {
    static void sctp_sched_prio_sched_all(struct sctp_stream *stream)
    {
    struct sctp_association *asoc;
    struct sctp_stream_out *sout;
    struct sctp_chunk *ch;
    asoc = container_of(stream, struct sctp_association, stream);
    list_for_each_entry(ch, &asoc.outqueue.out_chunk_list, list) {
    __u16 sid;
    sid = sctp_chunk_stream_no(ch);
    sout = SCTP_SO(stream, sid);
    if (sout.ext)
    sctp_sched_prio_sched(stream, sout.ext);
    }
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_prio_unsched_all(stream: *mut sctp_stream) {
    static void sctp_sched_prio_unsched_all(struct sctp_stream *stream)
    {
    struct sctp_stream_priorities *p, *tmp;
    struct sctp_stream_out_ext *soute, *souttmp;
    list_for_each_entry_safe(p, tmp, &stream.prio_list, prio_sched)
    list_for_each_entry_safe(soute, souttmp, &p.active, prio_list)
    sctp_sched_prio_unsched(soute);
    }
    static const struct sctp_sched_ops sctp_sched_prio = {
    .set = sctp_sched_prio_set,
    .get = sctp_sched_prio_get,
    .init = sctp_sched_prio_init,
    .init_sid = sctp_sched_prio_init_sid,
    .free_sid = sctp_sched_prio_free_sid,
    .enqueue = sctp_sched_prio_enqueue,
    .dequeue = sctp_sched_prio_dequeue,
    .dequeue_done = sctp_sched_prio_dequeue_done,
    .sched_all = sctp_sched_prio_sched_all,
    .unsched_all = sctp_sched_prio_unsched_all,
    };
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_ops_prio_init() {
    void sctp_sched_ops_prio_init(void)
    {
    sctp_sched_ops_register(SCTP_SS_PRIO, &sctp_sched_prio);
    }
