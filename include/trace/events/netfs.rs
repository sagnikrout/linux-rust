//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/netfs.h
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
// Network filesystem support module tracepoints
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Define enums for tracing information.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_read_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_write_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_rreq_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_sreq_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_failure {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_rreq_ref_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_sreq_ref_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_folio_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_collect_contig_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_donate_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_folioq_trace {

//
// Export enum symbols via userspace.
//

    netfs_read_traces;
    netfs_write_traces;
    netfs_rreq_origins;
    netfs_rreq_traces;
    netfs_sreq_sources;
    netfs_sreq_traces;
    netfs_failures;
    netfs_rreq_ref_traces;
    netfs_sreq_ref_traces;
    netfs_folio_traces;
    netfs_collect_contig_traces;
    netfs_donate_traces;
    netfs_folioq_traces;

//
// Now redefine the EM() and E_() macros to map the enums to the strings that
// will be printed in the output.
//

    TRACE_EVENT(netfs_read,
    TP_PROTO(struct netfs_io_request *rreq,
    loff_t start, size_t len,
    enum netfs_read_trace what),

    TP_ARGS(rreq, start, len, what),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned int,		cookie)
    __field(loff_t,			i_size)
    __field(loff_t,			start)
    __field(size_t,			len)
    __field(enum netfs_read_trace,	what)
    __field(u64,			netfs_inode)
    ),

    TP_fast_assign(
    __entry->rreq	= rreq->debug_id;
    __entry->cookie	= rreq->cache_resources.debug_id;
    __entry->i_size	= rreq->i_size;
    __entry->start	= start;
    __entry->len	= len;
    __entry->what	= what;
    __entry->netfs_inode = rreq->inode->i_ino;
    ),

    TP_printk("R=%08x %s c=%08x ni=%llx s=%llx l=%zx sz=%llx",
    __entry->rreq,
    __print_symbolic(__entry->what, netfs_read_traces),
    __entry->cookie,
    __entry->netfs_inode,
    __entry->start, __entry->len, __entry->i_size)
    );

    TRACE_EVENT(netfs_rreq,
    TP_PROTO(struct netfs_io_request *rreq,
    enum netfs_rreq_trace what),

    TP_ARGS(rreq, what),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned int,		flags)
    __field(enum netfs_io_origin,	origin)
    __field(enum netfs_rreq_trace,	what)
    ),

    TP_fast_assign(
    __entry->rreq	= rreq->debug_id;
    __entry->flags	= rreq->flags;
    __entry->origin	= rreq->origin;
    __entry->what	= what;
    ),

    TP_printk("R=%08x %s %s f=%02x",
    __entry->rreq,
    __print_symbolic(__entry->origin, netfs_rreq_origins),
    __print_symbolic(__entry->what, netfs_rreq_traces),
    __entry->flags)
    );

    TRACE_EVENT(netfs_sreq,
    TP_PROTO(struct netfs_io_subrequest *sreq,
    enum netfs_sreq_trace what),

    TP_ARGS(sreq, what),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned short,		index)
    __field(short,			error)
    __field(unsigned short,		flags)
    __field(enum netfs_io_source,	source)
    __field(enum netfs_sreq_trace,	what)
    __field(u8,				slot)
    __field(size_t,			len)
    __field(size_t,			transferred)
    __field(loff_t,			start)
    ),

    TP_fast_assign(
    __entry->rreq	= sreq->rreq->debug_id;
    __entry->index	= sreq->debug_index;
    __entry->error	= sreq->error;
    __entry->flags	= sreq->flags;
    __entry->source	= sreq->source;
    __entry->what	= what;
    __entry->len	= sreq->len;
    __entry->transferred = sreq->transferred;
    __entry->start	= sreq->start;
    __entry->slot	= sreq->io_iter.folioq_slot;
    ),

    TP_printk("R=%08x[%x] %s %s f=%03x s=%llx %zx/%zx s=%u e=%d",
    __entry->rreq, __entry->index,
    __print_symbolic(__entry->source, netfs_sreq_sources),
    __print_symbolic(__entry->what, netfs_sreq_traces),
    __entry->flags,
    __entry->start, __entry->transferred, __entry->len,
    __entry->slot, __entry->error)
    );

    TRACE_EVENT(netfs_failure,
    TP_PROTO(struct netfs_io_request *rreq,
    struct netfs_io_subrequest *sreq,
    int error, enum netfs_failure what),

    TP_ARGS(rreq, sreq, error, what),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(short,			index)
    __field(short,			error)
    __field(unsigned short,		flags)
    __field(enum netfs_io_source,	source)
    __field(enum netfs_failure,		what)
    __field(size_t,			len)
    __field(size_t,			transferred)
    __field(loff_t,			start)
    ),

    TP_fast_assign(
    __entry->rreq	= rreq->debug_id;
    __entry->index	= sreq ? sreq->debug_index : -1;
    __entry->error	= error;
    __entry->flags	= sreq ? sreq->flags : 0;
    __entry->source	= sreq ? sreq->source : NETFS_INVALID_READ;
    __entry->what	= what;
    __entry->len	= sreq ? sreq->len : rreq->len;
    __entry->transferred = sreq ? sreq->transferred : 0;
    __entry->start	= sreq ? sreq->start : 0;
    ),

    TP_printk("R=%08x[%x] %s f=%02x s=%llx %zx/%zx %s e=%d",
    __entry->rreq, __entry->index,
    __print_symbolic(__entry->source, netfs_sreq_sources),
    __entry->flags,
    __entry->start, __entry->transferred, __entry->len,
    __print_symbolic(__entry->what, netfs_failures),
    __entry->error)
    );

    TRACE_EVENT(netfs_rreq_ref,
    TP_PROTO(unsigned int rreq_debug_id, int ref,
    enum netfs_rreq_ref_trace what),

    TP_ARGS(rreq_debug_id, ref, what),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(int,			ref)
    __field(enum netfs_rreq_ref_trace,	what)
    ),

    TP_fast_assign(
    __entry->rreq	= rreq_debug_id;
    __entry->ref	= ref;
    __entry->what	= what;
    ),

    TP_printk("R=%08x %s r=%u",
    __entry->rreq,
    __print_symbolic(__entry->what, netfs_rreq_ref_traces),
    __entry->ref)
    );

    TRACE_EVENT(netfs_sreq_ref,
    TP_PROTO(unsigned int rreq_debug_id, unsigned int subreq_debug_index,
    int ref, enum netfs_sreq_ref_trace what),

    TP_ARGS(rreq_debug_id, subreq_debug_index, ref, what),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned int,		subreq)
    __field(int,			ref)
    __field(enum netfs_sreq_ref_trace,	what)
    ),

    TP_fast_assign(
    __entry->rreq	= rreq_debug_id;
    __entry->subreq	= subreq_debug_index;
    __entry->ref	= ref;
    __entry->what	= what;
    ),

    TP_printk("R=%08x[%x] %s r=%u",
    __entry->rreq,
    __entry->subreq,
    __print_symbolic(__entry->what, netfs_sreq_ref_traces),
    __entry->ref)
    );

    TRACE_EVENT(netfs_folio,
    TP_PROTO(struct folio *folio, enum netfs_folio_trace why),

    TP_ARGS(folio, why),

    TP_STRUCT__entry(
    __field(u64,			ino)
    __field(pgoff_t,			index)
    __field(unsigned int,		nr)
    __field(enum netfs_folio_trace,	why)
    ),

    TP_fast_assign(
    struct address_space *__m = READ_ONCE(folio->mapping);
    __entry->ino = __m ? __m->host->i_ino : 0;
    __entry->why = why;
    __entry->index = folio->index;
    __entry->nr = folio_nr_pages(folio);
    ),

    TP_printk("i=%05llx ix=%05lx-%05lx %s",
    __entry->ino, __entry->index, __entry->index + __entry->nr - 1,
    __print_symbolic(__entry->why, netfs_folio_traces))
    );

    TRACE_EVENT(netfs_write_iter,
    TP_PROTO(const struct kiocb *iocb, const struct iov_iter *from),

    TP_ARGS(iocb, from),

    TP_STRUCT__entry(
    __field(unsigned long long,		start)
    __field(size_t,			len)
    __field(unsigned int,		flags)
    __field(unsigned int,		ino)
    ),

    TP_fast_assign(
    __entry->start	= iocb->ki_pos;
    __entry->len	= iov_iter_count(from);
    __entry->ino	= iocb->ki_filp->f_inode->i_ino;
    __entry->flags	= iocb->ki_flags;
    ),

    TP_printk("WRITE-ITER i=%x s=%llx l=%zx f=%x",
    __entry->ino, __entry->start, __entry->len, __entry->flags)
    );

    TRACE_EVENT(netfs_write,
    TP_PROTO(const struct netfs_io_request *wreq,
    enum netfs_write_trace what),

    TP_ARGS(wreq, what),

    TP_STRUCT__entry(
    __field(unsigned int,		wreq)
    __field(unsigned int,		cookie)
    __field(unsigned int,		ino)
    __field(enum netfs_write_trace,	what)
    __field(unsigned long long,		start)
    __field(unsigned long long,		len)
    ),

    TP_fast_assign(
    struct netfs_inode *__ctx = netfs_inode(wreq->inode);
    struct fscache_cookie *__cookie = netfs_i_cookie(__ctx);
    __entry->wreq	= wreq->debug_id;
    __entry->cookie	= __cookie ? __cookie->debug_id : 0;
    __entry->ino	= wreq->inode->i_ino;
    __entry->what	= what;
    __entry->start	= wreq->start;
    __entry->len	= wreq->len;
    ),

    TP_printk("R=%08x %s c=%08x i=%x by=%llx-%llx",
    __entry->wreq,
    __print_symbolic(__entry->what, netfs_write_traces),
    __entry->cookie,
    __entry->ino,
    __entry->start, __entry->start + __entry->len - 1)
    );

    TRACE_EVENT(netfs_copy2cache,
    TP_PROTO(const struct netfs_io_request *rreq,
    const struct netfs_io_request *creq),

    TP_ARGS(rreq, creq),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned int,		creq)
    __field(unsigned int,		cookie)
    __field(unsigned int,		ino)
    ),

    TP_fast_assign(
    struct netfs_inode *__ctx = netfs_inode(rreq->inode);
    struct fscache_cookie *__cookie = netfs_i_cookie(__ctx);
    __entry->rreq	= rreq->debug_id;
    __entry->creq	= creq->debug_id;
    __entry->cookie	= __cookie ? __cookie->debug_id : 0;
    __entry->ino	= rreq->inode->i_ino;
    ),

    TP_printk("R=%08x CR=%08x c=%08x i=%x ",
    __entry->rreq,
    __entry->creq,
    __entry->cookie,
    __entry->ino)
    );

    TRACE_EVENT(netfs_collect,
    TP_PROTO(const struct netfs_io_request *wreq),

    TP_ARGS(wreq),

    TP_STRUCT__entry(
    __field(unsigned int,		wreq)
    __field(unsigned int,		len)
    __field(unsigned long long,		transferred)
    __field(unsigned long long,		start)
    ),

    TP_fast_assign(
    __entry->wreq	= wreq->debug_id;
    __entry->start	= wreq->start;
    __entry->len	= wreq->len;
    __entry->transferred = wreq->transferred;
    ),

    TP_printk("R=%08x s=%llx-%llx",
    __entry->wreq,
    __entry->start + __entry->transferred,
    __entry->start + __entry->len)
    );

    TRACE_EVENT(netfs_collect_sreq,
    TP_PROTO(const struct netfs_io_request *wreq,
    const struct netfs_io_subrequest *subreq),

    TP_ARGS(wreq, subreq),

    TP_STRUCT__entry(
    __field(unsigned int,		wreq)
    __field(unsigned int,		subreq)
    __field(unsigned int,		stream)
    __field(unsigned int,		len)
    __field(unsigned int,		transferred)
    __field(unsigned long long,		start)
    ),

    TP_fast_assign(
    __entry->wreq	= wreq->debug_id;
    __entry->subreq	= subreq->debug_index;
    __entry->stream	= subreq->stream_nr;
    __entry->start	= subreq->start;
    __entry->len	= subreq->len;
    __entry->transferred = subreq->transferred;
    ),

    TP_printk("R=%08x[%u:%02x] s=%llx t=%x/%x",
    __entry->wreq, __entry->stream, __entry->subreq,
    __entry->start, __entry->transferred, __entry->len)
    );

    TRACE_EVENT(netfs_collect_folio,
    TP_PROTO(const struct netfs_io_request *wreq,
    const struct folio *folio,
    unsigned long long fend,
    unsigned long long collected_to),

    TP_ARGS(wreq, folio, fend, collected_to),

    TP_STRUCT__entry(
    __field(unsigned int,	wreq)
    __field(unsigned long,	index)
    __field(unsigned long long,	fend)
    __field(unsigned long long,	cleaned_to)
    __field(unsigned long long,	collected_to)
    ),

    TP_fast_assign(
    __entry->wreq	= wreq->debug_id;
    __entry->index	= folio->index;
    __entry->fend	= fend;
    __entry->cleaned_to	= wreq->cleaned_to;
    __entry->collected_to = collected_to;
    ),

    TP_printk("R=%08x ix=%05lx r=%llx-%llx t=%llx/%llx",
    __entry->wreq, __entry->index,
    (unsigned long long)__entry->index * PAGE_SIZE, __entry->fend,
    __entry->cleaned_to, __entry->collected_to)
    );

    TRACE_EVENT(netfs_collect_state,
    TP_PROTO(const struct netfs_io_request *wreq,
    unsigned long long collected_to,
    unsigned int notes),

    TP_ARGS(wreq, collected_to, notes),

    TP_STRUCT__entry(
    __field(unsigned int,	wreq)
    __field(unsigned int,	notes)
    __field(unsigned long long,	collected_to)
    __field(unsigned long long,	cleaned_to)
    ),

    TP_fast_assign(
    __entry->wreq	= wreq->debug_id;
    __entry->notes	= notes;
    __entry->collected_to = collected_to;
    __entry->cleaned_to	= wreq->cleaned_to;
    ),

    TP_printk("R=%08x col=%llx cln=%llx n=%x",
    __entry->wreq, __entry->collected_to,
    __entry->cleaned_to,
    __entry->notes)
    );

    TRACE_EVENT(netfs_collect_gap,
    TP_PROTO(const struct netfs_io_request *wreq,
    const struct netfs_io_stream *stream,
    unsigned long long jump_to, char type),

    TP_ARGS(wreq, stream, jump_to, type),

    TP_STRUCT__entry(
    __field(unsigned int,	wreq)
    __field(unsigned char,	stream)
    __field(unsigned char,	type)
    __field(unsigned long long,	from)
    __field(unsigned long long,	to)
    ),

    TP_fast_assign(
    __entry->wreq	= wreq->debug_id;
    __entry->stream	= stream->stream_nr;
    __entry->from	= stream->collected_to;
    __entry->to		= jump_to;
    __entry->type	= type;
    ),

    TP_printk("R=%08x[%x:] %llx->%llx %c",
    __entry->wreq, __entry->stream,
    __entry->from, __entry->to, __entry->type)
    );

    TRACE_EVENT(netfs_collect_stream,
    TP_PROTO(const struct netfs_io_request *wreq,
    const struct netfs_io_stream *stream),

    TP_ARGS(wreq, stream),

    TP_STRUCT__entry(
    __field(unsigned int,	wreq)
    __field(unsigned char,	stream)
    __field(unsigned long long,	collected_to)
    __field(unsigned long long,	issued_to)
    ),

    TP_fast_assign(
    __entry->wreq	= wreq->debug_id;
    __entry->stream	= stream->stream_nr;
    __entry->collected_to = stream->collected_to;
    __entry->issued_to	= atomic64_read(&wreq->issued_to);
    ),

    TP_printk("R=%08x[%x:] cto=%llx ito=%llx",
    __entry->wreq, __entry->stream,
    __entry->collected_to, __entry->issued_to)
    );

    TRACE_EVENT(netfs_folioq,
    TP_PROTO(const struct folio_queue *fq,
    enum netfs_folioq_trace trace),

    TP_ARGS(fq, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned int,		id)
    __field(enum netfs_folioq_trace,	trace)
    ),

    TP_fast_assign(
    __entry->rreq	= fq ? fq->rreq_id : 0;
    __entry->id		= fq ? fq->debug_id : 0;
    __entry->trace	= trace;
    ),

    TP_printk("R=%08x fq=%x %s",
    __entry->rreq, __entry->id,
    __print_symbolic(__entry->trace, netfs_folioq_traces))
    );

// This part must be outside protection
