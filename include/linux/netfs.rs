//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfs.h
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
// Network filesystem support services.
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// See:
//
// Documentation/filesystems/netfs_library.rst
//
// for a description of the network filesystem interface declared here.
//

pub type mempool_t = mempool;
//
// folio_start_private_2 - Start an fscache write on a folio.  [DEPRECATED]
// @folio: The folio.
//
// Call this function before writing a folio to a local cache.  Starting a
// second write before the first one finishes is not allowed.
//
// Note that this should no longer be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_io_source {
    NETFS_SOURCE_UNKNOWN,
    NETFS_FILL_WITH_ZEROES,
    NETFS_DOWNLOAD_FROM_SERVER,
    NETFS_READ_FROM_CACHE,
    NETFS_INVALID_READ,
    NETFS_UPLOAD_TO_SERVER,
    NETFS_WRITE_TO_CACHE,
    } __mode(byte);

    typedef void (*netfs_io_terminated_t)(void *priv, ssize_t transferred_or_error);

//
// Per-inode context.  This wraps the VFS inode.
//
    struct netfs_inode {
    struct inode		inode;		/* The VFS inode */
    const struct netfs_request_ops *ops;

    struct fscache_cookie	*cache;

    struct list_head	wb_queue;	/* Queue of processes wanting to do writeback */
    loff_t			_remote_i_size;	/* Size of the remote file */
    loff_t			_zero_point;	/* Size after which we assume there's no data
// on the server
    spinlock_t		lock;		/* Lock covering wb_queue */
    atomic_t		io_count;	/* Number of outstanding reqs */
    unsigned long		flags;

}

//
// A netfs group - for instance a ceph snap.  This is marked on dirty pages and
// pages marked with a group must be flushed before they can be written under
// the domain of another group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_group {
    pub ref: refcount_t,
    pub netfs_group): *mut *mut void (free)(struct netfs_group,
}

//
// Information about a dirty page (attached only if necessary).
// folio->private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_folio {
    pub /: *mut *mut *mut netfs_group netfs_group; / Filesystem's grouping marker (or NULL).,
    pub /: *mut *mut unsigned int dirty_offset; / Write-streaming dirty data offset,
    pub /: *mut *mut unsigned int dirty_len; / Write-streaming dirty data length,
}

pub const NETFS_FOLIO_INFO: c_uint = 0x1UL	/* OR'd with folio->private. */;

extern "C" {
    pub fn __netfs_folio_info(_arg: folio_get_private(folio)) -> return;
}
//
// Stream of I/O subrequests going to a particular destination, such as the
// server or the local cache.  This is mainly intended for writing where we may
// have to write to multiple destinations concurrently.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_io_stream {
// Submission tracking
    pub /: *mut *mut *mut netfs_io_subrequest construct; / Op being constructed,
    pub /: *mut *mut size_t sreq_max_len; / Maximum size of a subrequest,
    pub /: *mut *mut unsigned int sreq_max_segs; / 0 or max number of segments in an iterator,
    pub /: *mut *mut unsigned int submit_off; / Folio offset we're submitting from,
    pub /: *mut *mut unsigned int submit_len; / Amount of data left to submit,
    pub /: *mut *mut unsigned int submit_extendable_to; / Amount I/O can be rounded up to,
    pub subreq): *mut *mut void (prepare_write)(struct netfs_io_subrequest,
    pub subreq): *mut *mut void (issue_write)(struct netfs_io_subrequest,
// Collection tracking
    pub /: *mut *mut list_head subrequests; / Contributory I/O operations,
    pub /: *mut *mut unsigned long long collected_to; / Position we've collected results to,
    pub /: *mut *mut size_t transferred; / The amount transferred from this stream,
    pub /: *mut *mut unsigned short error; / Aggregate error for the stream,
    pub /: *mut *mut netfs_io_source source; / Where to read from/write to,
    pub /: *mut *mut unsigned char stream_nr; / Index of stream in parent table,
    pub /: *mut *mut bool avail; / T if stream is available,
    pub /: *mut *mut bool active; / T if stream is active,
    pub /: *mut *mut bool need_retry; / T if this stream needs retrying,
    pub /: *mut *mut bool failed; / T if this stream failed,
    pub /: *mut *mut bool transferred_valid; / T is ->transferred is valid,
}

//
// Resources required to do operations on a cache.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_cache_resources {
    pub ops: *const netfs_cache_ops,
    pub cache_priv: *mut c_void,
    pub cache_priv2: *mut c_void,
    pub /: *mut *mut unsigned int debug_id; / Cookie debug ID,
    pub /: *mut *mut unsigned int inval_counter; / object->inval_counter at begin_op,
}

//
// Descriptor for a single component subrequest.  Each operation represents an
// individual read/write from/to a server, a cache, a journal, etc..
//
// The buffer iterator is persistent for the life of the subrequest struct and
// the pages it points to can be relied on to exist for the duration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_io_subrequest {
    pub /: *mut *mut *mut netfs_io_request rreq; / Supervising I/O request,
    pub work: work_struct,
    pub /: *mut *mut list_head rreq_link; / Link in rreq->subrequests,
    pub /: *mut *mut iov_iter io_iter; / Iterator for this subrequest,
    pub /: *mut *mut unsigned long long start; / Where to start the I/O,
    pub /: *mut *mut size_t len; / Size of the I/O,
    pub /: *mut *mut size_t transferred; / Amount of data transferred,
    pub ref: refcount_t,
    pub /: *mut *mut short error; / 0 or error that occurred,
    pub /: *mut *mut unsigned short debug_index; / Index in list (for debugging output),
    pub /: *mut *mut unsigned int nr_segs; / Number of segs in io_iter,
    pub /: *mut *mut u8 retry_count; / The number of retries (0 on initial pass),
    pub /: *mut *mut netfs_io_source source; / Where to read from/write to,
    pub /: *mut *mut unsigned char stream_nr; / I/O stream this belongs to,
    pub flags: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_io_origin {
    NETFS_READAHEAD,		/* This read was triggered by readahead */
    NETFS_READPAGE,			/* This read is a synchronous read */
    NETFS_READ_GAPS,		/* This read is a synchronous read to fill gaps */
    NETFS_READ_SINGLE,		/* This read should be treated as a single object */
    NETFS_READ_FOR_WRITE,		/* This read is to prepare a write */
    NETFS_UNBUFFERED_READ,		/* This is an unbuffered read */
    NETFS_DIO_READ,			/* This is a direct I/O read */
    NETFS_WRITEBACK,		/* This write was triggered by writepages */
    NETFS_WRITEBACK_SINGLE,		/* This monolithic write was triggered by writepages */
    NETFS_WRITETHROUGH,		/* This write was made by netfs_perform_write() */
    NETFS_UNBUFFERED_WRITE,		/* This is an unbuffered write */
    NETFS_DIO_WRITE,		/* This is a direct I/O write */
    NETFS_PGPRIV2_COPY_TO_CACHE,	/* [DEPRECATED] This is writing read data to the cache */
    nr__netfs_io_origin
    } __mode(byte);

//
// Descriptor for an I/O helper request.  This is used to make multiple I/O
// operations to a variety of data stores and then stitch the result together.
//
    struct netfs_io_request {
    union {
    struct work_struct cleanup_work; /* Deferred cleanup work */
    struct rcu_head rcu;
}

// write to cache on read
//
// Operations the network filesystem can/must provide to the helpers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_request_ops {
    pub request_pool: *mut mempool_t,
    pub subrequest_pool: *mut mempool_t,
    pub file): *mut *mut *mut int (init_request)(struct netfs_io_request rreq, struct file,
    pub rreq): *mut *mut void (free_request)(struct netfs_io_request,
    pub rreq): *mut *mut void (free_subrequest)(struct netfs_io_subrequest,
// Read request handling
    pub rreq): *mut *mut void (expand_readahead)(struct netfs_io_request,
    pub subreq): *mut *mut int (prepare_read)(struct netfs_io_subrequest,
    pub subreq): *mut *mut void (issue_read)(struct netfs_io_subrequest,
    pub rreq): *mut *mut bool (is_still_valid)(struct netfs_io_request,
    pub _fsdata): *mut *mut *mut folio foliop, void,
    pub rreq): *mut *mut void (done)(struct netfs_io_request,
// Modification handling
    pub i_size): *mut *mut *mut void (update_i_size)(struct inode inode, loff_t,
    pub inode): *mut *mut void (post_modify)(struct inode,
// Write request handling
    pub wreq): *mut *mut void (begin_writeback)(struct netfs_io_request,
    pub subreq): *mut *mut void (prepare_write)(struct netfs_io_subrequest,
    pub subreq): *mut *mut void (issue_write)(struct netfs_io_subrequest,
    pub stream): *mut *mut *mut void (retry_request)(struct netfs_io_request wreq, struct netfs_io_stream,
    pub wreq): *mut *mut void (invalidate_cache)(struct netfs_io_request,
}

//
// How to handle reading from a hole.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netfs_read_from_hole {
    NETFS_READ_HOLE_IGNORE,
    NETFS_READ_HOLE_FAIL,
}

//
// Table of operations for access to a cache.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netfs_cache_ops {
// End an operation
    pub cres): *mut *mut void (end_operation)(struct netfs_cache_resources,
// Read data from the cache
    pub term_func_priv): *mut c_void,
// Write data to the cache
    pub term_func_priv): *mut c_void,
// Write data to the cache from a netfs subrequest.
    pub subreq): *mut *mut void (issue_write)(struct netfs_io_subrequest,
// Expand readahead request
    pub i_size): c_ulonglong,
// Prepare a read operation, shortening it to a cached/uncached
// boundary as appropriate.
//
    pub i_size): c_ulonglong,
// Prepare a write subrequest, working out if we're allowed to do it
// and finding out the maximum amount of data to gather before
// attempting to submit.  If we're not permitted to do it, the
// subrequest should be marked failed.
//
    pub subreq): *mut *mut void (prepare_write_subreq)(struct netfs_io_subrequest,
// Prepare a write operation, working out what part of the write we can
// actually do.
//
    pub no_space_allocated_yet): loff_t i_size, bool,
// Query the occupancy of the cache in a region, returning where the
// next chunk of data starts and how long it is.
//
    pub _data_len): *mut *mut loff_t _data_start, size_t,
}

// High-level read API.
extern "C" {
    pub fn netfs_unbuffered_read_iter_locked(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn netfs_unbuffered_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn netfs_buffered_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn netfs_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
// High-level write API
extern "C" {
    pub fn netfs_unbuffered_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn netfs_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
}
// Single, monolithic object read/write API.
extern "C" {
    pub fn netfs_single_mark_inode_dirty(inode: *mut inode);
}
extern "C" {
    pub fn netfs_read_single(inode: *mut inode, file: *mut file, iter: *mut iov_iter) -> isize;
}
// Address operations API
extern "C" {
    pub fn netfs_readahead(: *mut readahead_control);
}
extern "C" {
    pub fn netfs_read_folio(: *mut file, : *mut folio) -> c_int;
}
extern "C" {
    pub fn netfs_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
}
extern "C" {
    pub fn netfs_unpin_writeback(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn netfs_clear_inode_writeback(inode: *mut inode, aux: *const c_void);
}
extern "C" {
    pub fn netfs_invalidate_folio(folio: *mut folio, offset: usize, length: usize);
}
extern "C" {
    pub fn netfs_release_folio(folio: *mut folio, gfp: gfp_t) -> bool;
}
// VMA operations API.
extern "C" {
    pub fn netfs_page_mkwrite(vmf: *mut vm_fault, netfs_group: *mut netfs_group) -> vm_fault_t;
}
// (Sub)request management API.
extern "C" {
    pub fn netfs_read_subreq_progress(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn netfs_read_subreq_terminated(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn netfs_prepare_write_failed(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn netfs_write_subrequest_terminated(_op: *mut c_void, transferred_or_error: isize);
}
extern "C" {
    pub fn netfs_start_io_read(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn netfs_end_io_read(inode: *mut inode);
}
extern "C" {
    pub fn netfs_start_io_write(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn netfs_end_io_write(inode: *mut inode);
}
extern "C" {
    pub fn netfs_start_io_direct(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn netfs_end_io_direct(inode: *mut inode);
}
// Miscellaneous APIs.
// Buffer wrangling helpers API.
extern "C" {
    pub fn netfs_free_folioq_buffer(fq: *mut folio_queue);
}
// Writeback exclusion API.
extern "C" {
    pub fn netfs_wb_begin(ictx: *mut netfs_inode, nowait: bool) -> bool;
}
extern "C" {
    pub fn netfs_wb_end(ictx: *mut netfs_inode);
}
//
// netfs_inode - Get the netfs inode context from the inode
// @inode: The inode to query
//
// Get the netfs lib inode context from the network filesystem's inode.  The
// context struct is expected to directly follow on from the VFS inode struct.
//
extern "C" {
    pub fn container_of(_arg: inode, netfs_inode: struct, _arg: inode) -> return;
}
//
// netfs_read_remote_i_size - Read remote_i_size safely
// @inode: The inode to access
//
// Read remote_i_size safely without the potential for tearing on 32-bit
// arches.
//
// NOTE: in a 32bit arch with a preemptable kernel and an UP compile the
// i_size_read/write must be atomic with respect to the local cpu (unlike with
// preempt disabled), but they don't need to be atomic with respect to other
// cpus like in true SMP (so they need either to either locally disable irq
// around the read or for example on x86 they can be still implemented as a
// cmpxchg8b without the need of the lock prefix).  For SMP compiles and 64bit
// archs it makes no difference if preempt is enabled or not.
//

// Pairs with smp_store_release() in netfs_write_remote_i_size()

//
// netfs_write_remote_i_size - Set remote_i_size safely
// @inode: The inode to access
// @remote_i_size: The new value for the size of the file on the server
//
// Set remote_i_size safely without the potential for tearing on 32-bit arches.
//
// Context: The caller must hold inode->i_lock.
//
// NOTE: unlike netfs_read_remote_i_size(), netfs_write_remote_i_size() does
// need locking around it (normally i_rwsem), otherwise on 32bit/SMP an update
// of i_size_seqcount can be lost, resulting in subsequent i_size_read() calls
// spinning forever.
//

//
// Pairs with smp_load_acquire() in netfs_read_remote_i_size() to
// ensure changes related to inode size (such as page contents) are
// visible before we see the changed inode size.
//

//
// netfs_read_zero_point - Read zero_point safely
// @inode: The inode to access
//
// Read zero_point safely without the potential for tearing on 32-bit
// arches.
//
// NOTE: in a 32bit arch with a preemptable kernel and an UP compile the
// i_size_read/write must be atomic with respect to the local cpu (unlike with
// preempt disabled), but they don't need to be atomic with respect to other
// cpus like in true SMP (so they need either to either locally disable irq
// around the read or for example on x86 they can be still implemented as a
// cmpxchg8b without the need of the lock prefix).  For SMP compiles and 64bit
// archs it makes no difference if preempt is enabled or not.
//

// Pairs with smp_store_release() in netfs_write_zero_point()

//
// netfs_write_zero_point - Set zero_point safely
// @inode: The inode to access
// @zero_point: The new value for the point beyond which the server has no data
//
// Set zero_point safely without the potential for tearing on 32-bit arches.
//
// Context: The caller must hold inode->i_lock.
//
// NOTE: unlike netfs_read_zero_point(), netfs_write_zero_point() does need
// locking around it (normally i_rwsem), otherwise on 32bit/SMP an update of
// i_size_seqcount can be lost, resulting in subsequent read calls spinning
// forever.
//

//
// Pairs with smp_load_acquire() in netfs_read_zero_point() to
// ensure changes related to inode size (such as page contents) are
// visible before we see the changed inode size.
//

//
// netfs_read_sizes - Read remote_i_size and zero_point safely
// @inode: The inode to access
// @i_size: Where to return the local file size.
// @remote_i_size: Where to return the size of the file on the server
// @zero_point: Where to return the the point beyond which the server has no data
//
// Read remote_i_size and zero_point safely without the potential for tearing
// on 32-bit arches.
//
// NOTE: in a 32bit arch with a preemptable kernel and an UP compile the
// i_size_read/write must be atomic with respect to the local cpu (unlike with
// preempt disabled), but they don't need to be atomic with respect to other
// cpus like in true SMP (so they need either to either locally disable irq
// around the read or for example on x86 they can be still implemented as a
// cmpxchg8b without the need of the lock prefix).  For SMP compiles and 64bit
// archs it makes no difference if preempt is enabled or not.
//

// i_size = inode->i_size;
// remote_i_size = ictx->_remote_i_size;
// zero_point = ictx->_zero_point;

// i_size = inode->i_size;
// remote_i_size = ictx->_remote_i_size;
// zero_point = ictx->_zero_point;

// Pairs with smp_store_release() in i_size_write()
// i_size = smp_load_acquire(&inode->i_size);
// Pairs with smp_store_release() in netfs_write_remote_i_size()
// remote_i_size = smp_load_acquire(&ictx->_remote_i_size);
// Pairs with smp_store_release() in netfs_write_zero_point()
// zero_point = smp_load_acquire(&ictx->_zero_point);

//
// netfs_write_sizes - Set i_size, remote_i_size and zero_point safely
// @inode: The inode to access
// @i_size: The new value for the local size of the file
// @remote_i_size: The new value for the size of the file on the server
// @zero_point: The new value for the point beyond which the server has no data
//
// Set both remote_i_size and zero_point safely without the potential for
// tearing on 32-bit arches.
//
// Context: The caller must hold inode->i_lock.
//
// NOTE: unlike netfs_read_zero_point(), netfs_write_zero_point() does need
// locking around it (normally i_rwsem), otherwise on 32bit/SMP an update of
// i_size_seqcount can be lost, resulting in subsequent read calls spinning
// forever.
//

//
// Pairs with smp_load_acquire() in i_size_read(),
// netfs_read_remote_i_size() and netfs_read_zero_point() to ensure
// changes related to inode size (such as page contents) are visible
// before we see the changed inode size.
//

//
// netfs_inode_init - Initialise a netfslib inode context
// @ctx: The netfs inode to initialise
// @ops: The netfs's operations list
// @use_zero_point: True to use the zero_point read optimisation
//
// Initialise the netfs library context struct.  This is expected to follow on
// directly from the VFS inode struct.
//

// ->releasepage() drives zero_point
//
// netfs_resize_file - Note that a file got resized
// @ictx: The netfs inode being resized
// @new_i_size: The new file size
// @changed_on_server: The change was applied to the server
//
// Inform the netfs lib that a file got resized so that it can adjust its state.
//

//
// Pairs with smp_load_acquire() in netfs_read_remote_i_size and
// netfs_read_zero_point() to ensure changes related to inode size
// (such as page contents) are visible before we see the changed inode
// size.
//

//
// netfs_i_cookie - Get the cache cookie from the inode
// @ctx: The netfs inode to query
//
// Get the caching cookie (if enabled) from the network filesystem's inode.
//

//
// netfs_wait_for_outstanding_io - Wait for outstanding I/O to complete
// @inode: The netfs inode to wait on
//
// Wait for outstanding I/O requests of any type to complete.  This is intended
// to be called from inode eviction routines.  This makes sure that any
// resources held by those requests are cleaned up before we let the inode get
// cleaned up.
//
