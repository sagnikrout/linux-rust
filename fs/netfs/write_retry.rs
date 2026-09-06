//! Automatically rewritten from C to Rust
//! Source: fs/netfs/write_retry.c
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
// Network filesystem write retrying.
//
// Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Perform retries on the streams that need it.
//
    static void netfs_retry_write_stream(struct netfs_io_request *wreq,
    struct netfs_io_stream *stream)
    {
    struct list_head *next;
    _enter("R=%x[%x:]", wreq.debug_id, stream.stream_nr);
    if (list_empty(&stream.subrequests))
    return;
    if (stream.source == NETFS_UPLOAD_TO_SERVER &&
    wreq.netfs_ops.retry_request)
    wreq.netfs_ops.retry_request(wreq, stream);
    if (unlikely(stream.failed))
    return;
// If there's no renegotiation to do, just resend each failed subreq.
    if (!stream.prepare_write) {
    struct netfs_io_subrequest *subreq;
    list_for_each_entry(subreq, &stream.subrequests, rreq_link) {
    if (test_bit(NETFS_SREQ_FAILED, &subreq.flags))
    break;
    if (__test_and_clear_bit(NETFS_SREQ_NEED_RETRY, &subreq.flags)) {
    struct iov_iter source;
    netfs_reset_iter(subreq);
    source = subreq.io_iter;
    netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
    netfs_reissue_write(stream, subreq, &source);
    }
    }
    return;
    }
    next = stream.subrequests.next;
    do {
    struct netfs_io_subrequest *subreq = core::ptr::null_mut(), *from, *to, *tmp;
    struct iov_iter source;
    unsigned long long start, len;
    size_t part;
    let mut boundary: bool = false;
// Go through the stream and find the next span of contiguous
// data that we then rejig (cifs, for example, needs the wsize
// renegotiating) and reissue.
//
    from = list_entry(next, struct netfs_io_subrequest, rreq_link);
    to = from;
    start = from.start + from.transferred;
    len   = from.len   - from.transferred;
    if (test_bit(NETFS_SREQ_FAILED, &from.flags) ||
    !test_bit(NETFS_SREQ_NEED_RETRY, &from.flags))
    return;
    for (;;) {
// Read pointer to subreq before reading subreq state.
    next = smp_load_acquire(&next.next);
    if (next == &stream.subrequests)
    break;
    subreq = list_entry(next, struct netfs_io_subrequest, rreq_link);
    if (subreq.start + subreq.transferred != start + len ||
    test_bit(NETFS_SREQ_BOUNDARY, &subreq.flags) ||
    !test_bit(NETFS_SREQ_NEED_RETRY, &subreq.flags))
    break;
    to = subreq;
    len += to.len;
    }
// Determine the set of buffers we're going to use.  Each
// subreq gets a subset of a single overall contiguous buffer.
//
    netfs_reset_iter(from);
    source = from.io_iter;
    source.count = len;
// Work through the sublist.
    subreq = from;
    list_for_each_entry_from(subreq, &stream.subrequests, rreq_link) {
    if (!len)
    break;
    subreq.start	= start;
    subreq.len	= len;
    __clear_bit(NETFS_SREQ_NEED_RETRY, &subreq.flags);
    trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
// Renegotiate max_len (wsize)
    stream.sreq_max_len = len;
    stream.prepare_write(subreq);
    part = umin(len, stream.sreq_max_len);
    if (unlikely(stream.sreq_max_segs))
    part = netfs_limit_iter(&source, 0, part, stream.sreq_max_segs);
    subreq.len = part;
    subreq.transferred = 0;
    len -= part;
    start += part;
    if (len && subreq == to &&
    __test_and_clear_bit(NETFS_SREQ_BOUNDARY, &to.flags))
    boundary = true;
    netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
    netfs_reissue_write(stream, subreq, &source);
    if (subreq == to)
    break;
    }
// If we managed to use fewer subreqs, we can discard the
// excess; if we used the same number, then we're done.
//
    if (!len) {
    if (subreq == to)
    continue;
    list_for_each_entry_safe_from(subreq, tmp,
    &stream.subrequests, rreq_link) {
    trace_netfs_sreq(subreq, netfs_sreq_trace_discard);
    spin_lock(&wreq.lock);
    list_del(&subreq.rreq_link);
    spin_unlock(&wreq.lock);
    netfs_put_subrequest(subreq, netfs_sreq_trace_put_done);
    if (subreq == to)
    break;
    }
    continue;
    }
// We ran out of subrequests, so we need to allocate some more
// and insert them after.
//
    do {
    subreq = netfs_alloc_subrequest(wreq);
    subreq.source		= to.source;
    subreq.start		= start;
    subreq.stream_nr	= to.stream_nr;
    subreq.retry_count	= 1;
    trace_netfs_sreq_ref(wreq.debug_id, subreq.debug_index,
    refcount_read(&subreq.ref),
    netfs_sreq_trace_new);
    trace_netfs_sreq(subreq, netfs_sreq_trace_split);
    spin_lock(&wreq.lock);
    list_add(&subreq.rreq_link, &to.rreq_link);
    spin_unlock(&wreq.lock);
    to = subreq;
    trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
    stream.sreq_max_len	= len;
    stream.sreq_max_segs	= INT_MAX;
    switch (stream.source) {
    case NETFS_UPLOAD_TO_SERVER:
    netfs_stat(&netfs_n_wh_upload);
    stream.sreq_max_len = umin(len, wreq.wsize);
    break;
    case NETFS_WRITE_TO_CACHE:
    netfs_stat(&netfs_n_wh_write);
    break;
    default:
    WARN_ON_ONCE(1);
    }
    stream.prepare_write(subreq);
    part = umin(len, stream.sreq_max_len);
    subreq.len = subreq.transferred + part;
    len -= part;
    start += part;
    if (!len && boundary) {
    __set_bit(NETFS_SREQ_BOUNDARY, &to.flags);
    boundary = false;
    }
    netfs_reissue_write(stream, subreq, &source);
    if (!len)
    break;
    } while (len);
    } while (!list_is_head(next, &stream.subrequests));
    }
//
// Perform retries on the streams that need it.  If we're doing content
// encryption and the server copy changed due to a third-party write, we may
// need to do an RMW cycle and also rewrite the data to the cache.
//
#[no_mangle]
pub unsafe extern "C" fn netfs_retry_writes(wreq: *mut netfs_io_request) {
    void netfs_retry_writes(struct netfs_io_request *wreq)
    {
    struct netfs_io_stream *stream;
    int s;
    netfs_stat(&netfs_n_wh_retry_write_req);
// Wait for all outstanding I/O to quiesce before performing retries as
// we may need to renegotiate the I/O sizes.
//
    set_bit(NETFS_RREQ_RETRYING, &wreq.flags);
    for (s = 0; s < NR_IO_STREAMS; s++) {
    stream = &wreq.io_streams[s];
    if (stream.active)
    netfs_wait_for_in_progress_stream(wreq, stream);
    }
    clear_bit(NETFS_RREQ_RETRYING, &wreq.flags);
// TODO: Enc: Fetch changed partial pages
// TODO: Enc: Reencrypt content if needed.
// TODO: Enc: Wind back transferred point.
// TODO: Enc: Mark cache pages for retry.
    for (s = 0; s < NR_IO_STREAMS; s++) {
    stream = &wreq.io_streams[s];
    if (stream.need_retry) {
    stream.need_retry = false;
    netfs_retry_write_stream(wreq, stream);
    }
    }
    }
