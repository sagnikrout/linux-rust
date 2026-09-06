//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/compress/zstd_cwksp.h
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// -
// Dependencies
//

// -
// Constants
//
// Since the workspace is effectively its own little malloc implementation
// arena, when we run under ASAN, we should similarly insert redzones between
// each internal element of the workspace, so ASAN will catch overruns that
// reach outside an object but that stay inside the workspace.
//
// This defines the size of that redzone.
//

pub const ZSTD_CWKSP_ASAN_REDZONE_SIZE: c_int = 128;

// Set our tables and aligneds to align by 64 bytes
pub const ZSTD_CWKSP_ALIGNMENT_BYTES: c_int = 64;
// -
// Structures
//
// Used to describe whether the workspace is statically allocated (and will not
// necessarily ever be freed), or if it's dynamically allocated and we can
// expect a well-formed caller to free this.
//
// Zstd fits all its internal datastructures into a single continuous buffer,
// so that it only needs to perform a single OS allocation (or so that a buffer
// can be provided to it and it can perform no allocations at all). This buffer
// is called the workspace.
//
// Several optimizations complicate that process of allocating memory ranges
// from this workspace for each internal datastructure:
//
// - These different internal datastructures have different setup requirements:
//
// - The static objects need to be cleared once and can then be trivially
// reused for each compression.
//
// - Various buffers don't need to be initialized at all--they are always
// written into before they're read.
//
// - The matchstate tables have a unique requirement that they don't need
// their memory to be totally cleared, but they do need the memory to have
// some bound, i.e., a guarantee that all values in the memory they've been
// allocated is less than some maximum value (which is the starting value
// for the indices that they will then use for compression). When this
// guarantee is provided to them, they can use the memory without any setup
// work. When it can't, they have to clear the area.
//
// - These buffers also have different alignment requirements.
//
// - We would like to reuse the objects in the workspace for multiple
// compressions without having to perform any expensive reallocation or
// reinitialization work.
//
// - We would like to be able to efficiently reuse the workspace across
// multiple compressions **even when the compression parameters change** and
// we need to resize some of the objects (where possible).
//
// To attempt to manage this buffer, given these constraints, the ZSTD_cwksp
// abstraction was created. It works as follows:
//
// Workspace Layout:
//
// [                        ... workspace ...                           ]
// [objects][tables ->] free space [<- buffers][<- aligned][<- init once]
//
// The various objects that live in the workspace are divided into the
// following categories, and are allocated separately:
//
// - Static objects: this is optionally the enclosing ZSTD_CCtx or ZSTD_CDict,
// so that literally everything fits in a single buffer. Note: if present,
// this must be the first object in the workspace, since ZSTD_customFree{CCtx,
// CDict}() rely on a pointer comparison to see whether one or two frees are
// required.
//
// - Fixed size objects: these are fixed-size, fixed-count objects that are
// nonetheless "dynamically" allocated in the workspace so that we can
// control how they're initialized separately from the broader ZSTD_CCtx.
// Examples:
// - Entropy Workspace
// - 2 x ZSTD_compressedBlockState_t
// - CDict dictionary contents
//
// - Tables: these are any of several different datastructures (hash tables,
// chain tables, binary trees) that all respect a common format: they are
// uint32_t arrays, all of whose values are between 0 and (nextSrc - base).
// Their sizes depend on the cparams. These tables are 64-byte aligned.
//
// - Init once: these buffers require to be initialized at least once before
// use. They should be used when we want to skip memory initialization
// while not triggering memory checkers (like Valgrind) when reading from
// from this memory without writing to it first.
// These buffers should be used carefully as they might contain data
// from previous compressions.
// Buffers are aligned to 64 bytes.
//
// - Aligned: these buffers don't require any initialization before they're
// used. The user of the buffer should make sure they write into a buffer
// location before reading from it.
// Buffers are aligned to 64 bytes.
//
// - Buffers: these buffers are used for various purposes that don't require
// any alignment or initialization before they're used. This means they can
// be moved around at no cost for a new compression.
//
// Allocating Memory:
//
// The various types of objects must be allocated in order, so they can be
// correctly packed into the workspace buffer. That order is:
//
// 1. Objects
// 2. Init once / Tables
// 3. Aligned / Tables
// 4. Buffers / Tables
//
// Attempts to reserve objects of different types out of order will fail.
//
// -
// Functions
//
extern "C" {
    pub fn ZSTD_cwksp_available_space(ws: *mut *mut ZSTD_cwksp) -> MEM_STATIC size_t;
}
extern "C" {
    pub fn ZSTD_cwksp_initialAllocStart(ws: *mut *mut ZSTD_cwksp) -> *mut MEM_STATIC void;
}
//
// Align must be a power of 2.
//
// Use this to determine how much space in the workspace we will consume to
// allocate this object. (Normally it should be exactly the size of the object,
// but under special conditions, like ASAN, where we pad each object, it might
// be larger.)
//
// Since tables aren't currently redzoned, you don't need to call through this
// to figure out how much space you need for the matchState tables. Everything
// else is though.
//
// Do not use for sizing aligned buffers. Instead, use ZSTD_cwksp_aligned64_alloc_size().
//
extern "C" {
    pub fn ZSTD_cwksp_alloc_size(_arg: ZSTD_cwksp_align(size, _arg: alignment)) -> return;
}
//
// Returns an adjusted alloc size that is the nearest larger multiple of 64 bytes.
// Used to determine the number of bytes required for a given "aligned".
//
extern "C" {
    pub fn ZSTD_cwksp_aligned_alloc_size(_arg: size, _arg: ZSTD_CWKSP_ALIGNMENT_BYTES) -> return;
}
//
// Returns the amount of additional space the cwksp must allocate
// for internal purposes (currently only alignment).
//
// For alignment, the wksp will always allocate an additional 2*ZSTD_CWKSP_ALIGNMENT_BYTES
// bytes to align the beginning of tables section and end of buffers;
//
// Return the number of additional bytes required to align a pointer to the given number of bytes.
// alignBytes must be a power of two.
//
// Returns the initial value for allocStart which is used to determine the position from
// which we can allocate from the end of the workspace.
//
// Internal function. Do not use directly.
// Reserves the given number of bytes within the aligned/buffer segment of the wksp,
// which counts from the end of the wksp (as opposed to the object/table segment).
//
// Returns a pointer to the beginning of that space.
//
// the area is reserved from the end of wksp.
// If it overlaps with tableValidEnd, it voids guarantees on values' range
//
// Moves the cwksp to the next phase, and does any necessary allocations.
// cwksp initialization must necessarily go through each phase in order.
// Returns a 0 on success, or zstd error
//
// Going from allocating objects to allocating initOnce / tables
//
// Returns whether this object/buffer/etc was allocated in this workspace.
//
// Internal function. Do not use directly.
//
// Reserves and returns unaligned memory.
//
// Reserves and returns memory sized on and aligned on ZSTD_CWKSP_ALIGNMENT_BYTES (64 bytes).
// This memory has been initialized at least once in the past.
// This doesn't mean it has been initialized this time, and it might contain data from previous
// operations.
// The main usage is for algorithms that might need read access into uninitialized memory.
// The algorithm must maintain safety under these conditions and must make sure it doesn't
// leak any of the past data (directly or in side channels).
//
// We assume the memory following the current allocation is either:
// 1. Not usable as initOnce memory (end of workspace)
// 2. Another initOnce buffer that has been allocated before (and so was previously memset)
// 3. An ASAN redzone, in which case we don't want to write on it
// For these reasons it should be fine to not explicitly zero every byte up to ws->initOnceStart.
// Note that we assume here that MSAN and ASAN cannot run in the same time.
//
// Reserves and returns memory sized on and aligned on ZSTD_CWKSP_ALIGNMENT_BYTES (64 bytes).
//
// Aligned on 64 bytes. These buffers have the special property that
// their values remain constrained, allowing us to reuse them without
// memset()-ing them.
//
// We can only start allocating tables after we are done reserving space for objects at the
// start of the workspace
//
// Aligned on sizeof(void*).
// Note : should happen only once, at workspace first initialization
//
// we must be in the first phase, no advance is possible
//
// with alignment control
// Note : should happen only once, at workspace first initialization
//
// Zero the part of the allocated tables not already marked clean.
//
// Invalidates table allocations.
// All other allocations remain valid.
//
// Invalidates all buffer, aligned, and table allocations.
// Object allocations remain valid.
//
// The provided workspace takes ownership of the buffer [start, start+size).
// Any existing values in the workspace are ignored (the previously managed
// buffer, if present, must be separately freed).
//
// Moves the management of a workspace from one cwksp to another. The src cwksp
// is left in an invalid state (src must be re-init()'ed before it's used again).
//
// dst = *src;
// -
// Functions Checking Free Space
//
// ZSTD_alignmentSpaceWithinBounds() :
// Returns if the estimated space needed for a wksp is within an acceptable limit of the
// actual amount of space used.
//
// We have an alignment space between objects and tables between tables and buffers, so we can have up to twice
// the alignment bytes difference between estimation and actual usage
