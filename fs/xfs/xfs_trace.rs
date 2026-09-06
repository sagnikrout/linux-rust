//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_trace.h
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
//
// Copyright (c) 2009, Christoph Hellwig
// All Rights Reserved.
//
// NOTE: none of these tracepoints shall be considered a stable kernel ABI
// as they can change at any time.
//
// Current conventions for printing numbers measuring specific units:
//
// agno: allocation group number
//
// agino: per-AG inode number
// ino: filesystem inode number
//
// agbno: per-AG block number in fs blocks
// rgbno: per-rtgroup block number in fs blocks
// startblock: physical block number for file mappings.  This is either a
// segmented fsblock for data device mappings, or a rfsblock
// for realtime device mappings
// fsbcount: number of blocks in an extent, in fs blocks
//
// gbno: generic allocation group block number.  This is an agbno for
// space in a per-AG or a rgbno for space in a realtime group.
//
// daddr: physical block number in 512b blocks
// bbcount: number of blocks in a physical extent, in 512b blocks
//
// rtx: physical rt extent number for extent mappings
// rtxcount: number of rt extents in an extent mapping
//
// owner: reverse-mapping owner, usually inodes
//
// fileoff: file offset, in fs blocks
// pos: file offset, in bytes
// bytecount: number of bytes
//
// dablk: directory or xattr block offset, in filesystem blocks
//
// disize: ondisk file size, in bytes
// isize: incore file size, in bytes
//
// forkoff: inode fork offset, in bytes
//
// ireccount: number of inode records
//
// Numbers describing space allocations (blocks, extents, inodes) should be
// formatted in hexadecimal.
//

// not really buffer traces, but the buf provides useful information
// pass flags explicitly

//
// ftrace's __print_symbolic requires that all enum values be wrapped in the
// TRACE_DEFINE_ENUM macro so that the enum value can be encoded in the ftrace
// ring buffer.  Somehow this was only worth mentioning in the ftrace sample
// code.
//

// deferred ops

// rmap tracepoints

// btree cursor error/%ip tracepoint class

// deferred bmbt updates
//
// Legacy rt filesystems do not have allocation groups
// ondisk.  We emulate this incore with one gigantic
// rtgroup whose size can exceed a 32-bit block number.
// For this tracepoint, we report group 0 and a 64-bit
// group block number.
//

// per-AG reservation

// per-AG reservation tracepoints
// refcount tracepoint classes

// single-rcext tracepoint class

// single-rcext and an agbno tracepoint class

// double-rcext tracepoint class

// double-rcext and an agbno tracepoint class

// triple-rcext tracepoint class

// refcount btree tracepoints
// refcount adjustment tracepoints
// reflink helpers

// simple inode-based error/%ip tracepoint class

// reflink tracepoint classes
// two-file io tracepoint class

// inode/irec events

// inode iomap invalidation events

// refcount/reflink tracepoint definitions
// reflink tracepoints
// unshare tracepoints
// copy on write
// rmap swapext tracepoints
// fsmap traces

// exchmaps tracepoints

// file exchange-range tracepoint class

// metadata inode space reservations

