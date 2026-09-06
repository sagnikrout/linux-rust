//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iov_iter.h
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
// I/O iterator iteration building functions.
//
// Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Handle ITER_UBUF.
//
// Handle ITER_IOVEC.
//
// Handle ITER_KVEC.
//
// Handle ITER_BVEC.
//
// Handle ITER_FOLIOQ.
//
// The iterator may have been extended.
//
// Handle ITER_XARRAY.
//
// Handle ITER_DISCARD.
//
// iterate_and_advance2 - Iterate over an iterator
// @iter: The iterator to iterate over.
// @len: The amount to iterate over.
// @priv: Data for the step functions.
// @priv2: More data for the step functions.
// @ustep: Function for UBUF/IOVEC iterators; given __user addresses.
// @step: Function for other iterators; given kernel addresses.
//
// Iterate over the next part of an iterator, up to the specified length.  The
// buffer is presented in segments, which for kernel iteration are broken up by
// physical pages and mapped, with the mapped address being presented.
//
// Two step functions, @step and @ustep, must be provided, one for handling
// mapped kernel addresses and the other is given user addresses which have the
// potential to fault since no pinning is performed.
//
// The step functions are passed the address and length of the segment, @priv,
// @priv2 and the amount of data so far iterated over (which can, for example,
// be added to @priv to point to the right part of a second buffer).  The step
// functions should return the amount of the segment they didn't process (ie. 0
// indicates complete processsing).
//
// This function returns the amount of data processed (ie. 0 means nothing was
// processed and the value of @len means processes to completion).
//
extern "C" {
    pub fn iterate_ubuf(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: ustep) -> return;
}
extern "C" {
    pub fn iterate_iovec(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: ustep) -> return;
}
extern "C" {
    pub fn iterate_bvec(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_kvec(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_folioq(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_xarray(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_discard(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
//
// iterate_and_advance - Iterate over an iterator
// @iter: The iterator to iterate over.
// @len: The amount to iterate over.
// @priv: Data for the step functions.
// @ustep: Function for UBUF/IOVEC iterators; given __user addresses.
// @step: Function for other iterators; given kernel addresses.
//
// As iterate_and_advance2(), but priv2 is always NULL.
//
extern "C" {
    pub fn iterate_and_advance2(_arg: iter, _arg: len, _arg: priv, _arg: NULL, _arg: ustep, _arg: step) -> return;
}
//
// iterate_and_advance_kernel - Iterate over a kernel-internal iterator
// @iter: The iterator to iterate over.
// @len: The amount to iterate over.
// @priv: Data for the step functions.
// @priv2: More data for the step functions.
// @step: Function for other iterators; given kernel addresses.
//
// Iterate over the next part of an iterator, up to the specified length.  The
// buffer is presented in segments, which for kernel iteration are broken up by
// physical pages and mapped, with the mapped address being presented.
//
// [!] Note This will only handle BVEC, KVEC, FOLIOQ, XARRAY and DISCARD-type
// iterators; it will not handle UBUF or IOVEC-type iterators.
//
// A step functions, @step, must be provided, one for handling mapped kernel
// addresses and the other is given user addresses which have the potential to
// fault since no pinning is performed.
//
// The step functions are passed the address and length of the segment, @priv,
// @priv2 and the amount of data so far iterated over (which can, for example,
// be added to @priv to point to the right part of a second buffer).  The step
// functions should return the amount of the segment they didn't process (ie. 0
// indicates complete processsing).
//
// This function returns the amount of data processed (ie. 0 means nothing was
// processed and the value of @len means processes to completion).
//
extern "C" {
    pub fn iterate_bvec(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_kvec(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_folioq(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_xarray(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
extern "C" {
    pub fn iterate_discard(_arg: iter, _arg: len, _arg: priv, _arg: priv2, _arg: step) -> return;
}
