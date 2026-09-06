//! Automatically rewritten from C to Rust
//! Source: net/sctp/stream_sched_fc.c
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
// (C) Copyright Red Hat Inc. 2022
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
// Xin Long <lucien.xin@gmail.com>
//

// Fair Capacity and Weighted Fair Queueing handling
// RFC 8260 section 3.5 and 3.6
//
    static void sctp_sched_fc_unsched_all(struct sctp_stream *stream);
    static int sctp_sched_wfq_set(struct sctp_stream *stream, __u16 sid,
    __u16 weight, gfp_t gfp)
    {
    struct sctp_stream_out_ext *soute = SCTP_SO(stream, sid).ext;
    if (!weight)
    return -EINVAL;
    soute.fc_weight = weight;
    return 0;
    }
    static int sctp_sched_wfq_get(struct sctp_stream *stream, __u16 sid,
    __u16 *value)
    {
    struct sctp_stream_out_ext *soute = SCTP_SO(stream, sid).ext;
// value = soute->fc_weight;
    return 0;
    }
    static int sctp_sched_fc_set(struct sctp_stream *stream, __u16 sid,
    __u16 weight, gfp_t gfp)
    {
    return 0;
    }
    static int sctp_sched_fc_get(struct sctp_stream *stream, __u16 sid,
    __u16 *value)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fc_init(stream: *mut sctp_stream) -> c_int {
    static int sctp_sched_fc_init(struct sctp_stream *stream)
    {
    INIT_LIST_HEAD(&stream.fc_list);
    return 0;
    }
    static int sctp_sched_fc_init_sid(struct sctp_stream *stream, __u16 sid,
    gfp_t gfp)
    {
    struct sctp_stream_out_ext *soute = SCTP_SO(stream, sid).ext;
    INIT_LIST_HEAD(&soute.fc_list);
    soute.fc_length = 0;
    soute.fc_weight = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fc_free_sid(stream: *mut sctp_stream, sid: __u16) {
    static void sctp_sched_fc_free_sid(struct sctp_stream *stream, __u16 sid)
    {
    }
    static void sctp_sched_fc_sched(struct sctp_stream *stream,
    struct sctp_stream_out_ext *soute)
    {
    struct sctp_stream_out_ext *pos;
    if (!list_empty(&soute.fc_list))
    return;
    list_for_each_entry(pos, &stream.fc_list, fc_list)
    if ((__u64)pos.fc_length * soute.fc_weight >=
    (__u64)soute.fc_length * pos.fc_weight)
    break;
    list_add_tail(&soute.fc_list, &pos.fc_list);
    }
    static void sctp_sched_fc_enqueue(struct sctp_outq *q,
    struct sctp_datamsg *msg)
    {
    struct sctp_stream *stream;
    struct sctp_chunk *ch;
    __u16 sid;
    ch = list_first_entry(&msg.chunks, struct sctp_chunk, frag_list);
    sid = sctp_chunk_stream_no(ch);
    stream = &q.asoc.stream;
    sctp_sched_fc_sched(stream, SCTP_SO(stream, sid).ext);
    }
    static struct sctp_chunk *sctp_sched_fc_dequeue(struct sctp_outq *q)
    {
    struct sctp_stream *stream = &q.asoc.stream;
    struct sctp_stream_out_ext *soute;
    struct sctp_chunk *ch;
// Bail out quickly if queue is empty
    if (list_empty(&q.out_chunk_list))
    return core::ptr::null_mut();
// Find which chunk is next
    if (stream.out_curr)
    soute = stream.out_curr.ext;
    else
    soute = list_entry(stream.fc_list.next, struct sctp_stream_out_ext, fc_list);
    ch = list_entry(soute.outq.next, struct sctp_chunk, stream_list);
    sctp_sched_dequeue_common(q, ch);
    return ch;
    }
    static void sctp_sched_fc_dequeue_done(struct sctp_outq *q,
    struct sctp_chunk *ch)
    {
    struct sctp_stream *stream = &q.asoc.stream;
    struct sctp_stream_out_ext *soute, *pos;
    __u16 sid, i;
    sid = sctp_chunk_stream_no(ch);
    soute = SCTP_SO(stream, sid).ext;
// reduce all fc_lengths by U32_MAX / 4 if the current fc_length overflows.
    if (soute.fc_length > U32_MAX - ch.skb.len) {
    for (i = 0; i < stream.outcnt; i++) {
    pos = SCTP_SO(stream, i).ext;
    if (!pos)
    continue;
    if (pos.fc_length <= (U32_MAX >> 2)) {
    pos.fc_length = 0;
    continue;
    }
    pos.fc_length -= (U32_MAX >> 2);
    }
    }
    soute.fc_length += ch.skb.len;
    if (list_empty(&soute.outq)) {
    list_del_init(&soute.fc_list);
    return;
    }
    pos = soute;
    list_for_each_entry_continue(pos, &stream.fc_list, fc_list)
    if ((__u64)pos.fc_length * soute.fc_weight >=
    (__u64)soute.fc_length * pos.fc_weight)
    break;
    list_move_tail(&soute.fc_list, &pos.fc_list);
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fc_sched_all(stream: *mut sctp_stream) {
    static void sctp_sched_fc_sched_all(struct sctp_stream *stream)
    {
    struct sctp_association *asoc;
    struct sctp_chunk *ch;
    asoc = container_of(stream, struct sctp_association, stream);
    list_for_each_entry(ch, &asoc.outqueue.out_chunk_list, list) {
    let mut sid: __u16 = sctp_chunk_stream_no(ch);
    if (SCTP_SO(stream, sid).ext)
    sctp_sched_fc_sched(stream, SCTP_SO(stream, sid).ext);
    }
    }
#[no_mangle]
unsafe extern "C" fn sctp_sched_fc_unsched_all(stream: *mut sctp_stream) {
    static void sctp_sched_fc_unsched_all(struct sctp_stream *stream)
    {
    struct sctp_stream_out_ext *soute, *tmp;
    list_for_each_entry_safe(soute, tmp, &stream.fc_list, fc_list)
    list_del_init(&soute.fc_list);
    }
    static const struct sctp_sched_ops sctp_sched_fc = {
    .set = sctp_sched_fc_set,
    .get = sctp_sched_fc_get,
    .init = sctp_sched_fc_init,
    .init_sid = sctp_sched_fc_init_sid,
    .free_sid = sctp_sched_fc_free_sid,
    .enqueue = sctp_sched_fc_enqueue,
    .dequeue = sctp_sched_fc_dequeue,
    .dequeue_done = sctp_sched_fc_dequeue_done,
    .sched_all = sctp_sched_fc_sched_all,
    .unsched_all = sctp_sched_fc_unsched_all,
    };
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_ops_fc_init() {
    void sctp_sched_ops_fc_init(void)
    {
    sctp_sched_ops_register(SCTP_SS_FC, &sctp_sched_fc);
    }
    static const struct sctp_sched_ops sctp_sched_wfq = {
    .set = sctp_sched_wfq_set,
    .get = sctp_sched_wfq_get,
    .init = sctp_sched_fc_init,
    .init_sid = sctp_sched_fc_init_sid,
    .free_sid = sctp_sched_fc_free_sid,
    .enqueue = sctp_sched_fc_enqueue,
    .dequeue = sctp_sched_fc_dequeue,
    .dequeue_done = sctp_sched_fc_dequeue_done,
    .sched_all = sctp_sched_fc_sched_all,
    .unsched_all = sctp_sched_fc_unsched_all,
    };
#[no_mangle]
pub unsafe extern "C" fn sctp_sched_ops_wfq_init() {
    void sctp_sched_ops_wfq_init(void)
    {
    sctp_sched_ops_register(SCTP_SS_WFQ, &sctp_sched_wfq);
    }
