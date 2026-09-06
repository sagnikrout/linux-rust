//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/folio_queue.h
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
// Queue of folios definitions
//
// Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// See:
//
// Documentation/core-api/folio_queue.rst
//
// for a description of the API.
//

//
// Segment in a queue of running buffers.  Each segment can hold a number of
// folios and a portion of the queue can be referenced with the ITER_FOLIOQ
// iterator.  The possibility exists of inserting non-folio elements into the
// queue (such as gaps).
//
// Explicit prev and next pointers are used instead of a list_head to make it
// easier to add segments to tail and remove them from the head without the
// need for a lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio_queue {
    pub /: *mut *mut folio_batch vec; / Folios in the queue segment,
    pub /: *mut *mut u8 orders[FOLIO_BATCH_SIZE]; / Order of each folio,
    pub /: *mut *mut *mut folio_queue next; / Next queue segment or NULL,
    pub /: *mut *mut *mut folio_queue prev; / Previous queue segment of NULL,
    pub /: *mut *mut unsigned long marks; / 1-bit mark per folio,
    pub /: *mut *mut unsigned long marks2; / Second 1-bit mark per folio,

    pub rreq_id: c_uint,
    pub debug_id: c_uint,
}

//
// folioq_init - Initialise a folio queue segment
// @folioq: The segment to initialise
// @rreq_id: The request identifier to use in tracelines.
//
// Initialise a folio queue segment and set an identifier to be used in traces.
//
// Note that the folio pointers are left uninitialised.
//
// folioq_nr_slots: Query the capacity of a folio queue segment
// @folioq: The segment to query
//
// Query the number of folios that a particular folio queue segment might hold.
// [!] NOTE: This must not be assumed to be the same for every segment!
//
// folioq_count: Query the occupancy of a folio queue segment
// @folioq: The segment to query
//
// Query the number of folios that have been added to a folio queue segment.
// Note that this is not decreased as folios are removed from a segment.
//
extern "C" {
    pub fn folio_batch_count(_arg: &folioq->vec) -> return;
}
//
// folioq_full: Query if a folio queue segment is full
// @folioq: The segment to query
//
// Query if a folio queue segment is fully occupied.  Note that this does not
// change if folios are removed from a segment.
//
// return !folio_batch_space(&folioq->vec);
extern "C" {
    pub fn folioq_count(folioq_nr_slots(folioq: folioq) >=) -> return;
}
//
// folioq_is_marked: Check first folio mark in a folio queue segment
// @folioq: The segment to query
// @slot: The slot number of the folio to query
//
// Determine if the first mark is set for the folio in the specified slot in a
// folio queue segment.
//
extern "C" {
    pub fn test_bit(_arg: slot, _arg: &folioq->marks) -> return;
}
//
// folioq_mark: Set the first mark on a folio in a folio queue segment
// @folioq: The segment to modify
// @slot: The slot number of the folio to modify
//
// Set the first mark for the folio in the specified slot in a folio queue
// segment.
//
// folioq_unmark: Clear the first mark on a folio in a folio queue segment
// @folioq: The segment to modify
// @slot: The slot number of the folio to modify
//
// Clear the first mark for the folio in the specified slot in a folio queue
// segment.
//
// folioq_is_marked2: Check second folio mark in a folio queue segment
// @folioq: The segment to query
// @slot: The slot number of the folio to query
//
// Determine if the second mark is set for the folio in the specified slot in a
// folio queue segment.
//
extern "C" {
    pub fn test_bit(_arg: slot, _arg: &folioq->marks2) -> return;
}
//
// folioq_mark2: Set the second mark on a folio in a folio queue segment
// @folioq: The segment to modify
// @slot: The slot number of the folio to modify
//
// Set the second mark for the folio in the specified slot in a folio queue
// segment.
//
// folioq_unmark2: Clear the second mark on a folio in a folio queue segment
// @folioq: The segment to modify
// @slot: The slot number of the folio to modify
//
// Clear the second mark for the folio in the specified slot in a folio queue
// segment.
//
// folioq_append: Add a folio to a folio queue segment
// @folioq: The segment to add to
// @folio: The folio to add
//
// Add a folio to the tail of the sequence in a folio queue segment, increasing
// the occupancy count and returning the slot number for the folio just added.
// The folio size is extracted and stored in the queue and the marks are left
// unmodified.
//
// Note that it's left up to the caller to check that the segment capacity will
// not be exceeded and to extend the queue.
//
// folioq_append_mark: Add a folio to a folio queue segment
// @folioq: The segment to add to
// @folio: The folio to add
//
// Add a folio to the tail of the sequence in a folio queue segment, increasing
// the occupancy count and returning the slot number for the folio just added.
// The folio size is extracted and stored in the queue, the first mark is set
// and and the second and third marks are left unmodified.
//
// Note that it's left up to the caller to check that the segment capacity will
// not be exceeded and to extend the queue.
//
// folioq_folio: Get a folio from a folio queue segment
// @folioq: The segment to access
// @slot: The folio slot to access
//
// Retrieve the folio in the specified slot from a folio queue segment.  Note
// that no bounds check is made and if the slot hasn't been added into yet, the
// pointer will be undefined.  If the slot has been cleared, NULL will be
// returned.
//
// folioq_folio_order: Get the order of a folio from a folio queue segment
// @folioq: The segment to access
// @slot: The folio slot to access
//
// Retrieve the order of the folio in the specified slot from a folio queue
// segment.  Note that no bounds check is made and if the slot hasn't been
// added into yet, the order returned will be 0.
//
// folioq_folio_size: Get the size of a folio from a folio queue segment
// @folioq: The segment to access
// @slot: The folio slot to access
//
// Retrieve the size of the folio in the specified slot from a folio queue
// segment.  Note that no bounds check is made and if the slot hasn't been
// added into yet, the size returned will be PAGE_SIZE.
//
// folioq_clear: Clear a folio from a folio queue segment
// @folioq: The segment to clear
// @slot: The folio slot to clear
//
// Clear a folio from a sequence in a folio queue segment and clear its marks.
// The occupancy count is left unchanged.
//
