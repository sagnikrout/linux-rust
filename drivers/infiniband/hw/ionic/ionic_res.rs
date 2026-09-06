//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ionic/ionic_res.h
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
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

//
// struct ionic_resid_bits - Number allocator based on IDA
//
// @inuse:      IDA handle
// @inuse_size: Highest ID limit for IDA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_resid_bits {
    pub inuse: ida,
    pub inuse_size: c_uint,
}

//
// ionic_resid_init() - Initialize a resid allocator
// @resid:  Uninitialized resid allocator
// @size:   Capacity of the allocator
//
// Return: Zero on success, or negative error number
//
// ionic_resid_destroy() - Destroy a resid allocator
// @resid:  Resid allocator
//
// ionic_resid_get_shared() - Allocate an available shared resource id
// @resid:   Resid allocator
// @min:     Smallest valid resource id
// @size:    One after largest valid resource id
//
// Return: Resource id, or negative error number
//
extern "C" {
    pub fn ida_alloc_range(_arg: &resid->inuse, _arg: min, 1: size -, _arg: GFP_KERNEL) -> return;
}
//
// ionic_resid_get() - Allocate an available resource id
// @resid: Resid allocator
//
// Return: Resource id, or negative error number
//
extern "C" {
    pub fn ionic_resid_get_shared(_arg: resid, _arg: 0, _arg: resid->inuse_size) -> return;
}
//
// ionic_resid_put() - Free a resource id
// @resid:  Resid allocator
// @id:     Resource id
//
// ionic_bitid_to_qid() - Transform a resource bit index into a queue id
// @bitid:           Bit index
// @qgrp_shift:      Log2 number of queues per queue group
// @half_qid_shift:  Log2 of half the total number of queues
//
// Return: Queue id
//
// Udma-constrained queues (QPs and CQs) are associated with their udma by
// queue group. Even queue groups are associated with udma0, and odd queue
// groups with udma1.
//
// For allocating queue ids, we want to arrange the bits into two halves,
// with the even queue groups of udma0 in the lower half of the bitset,
// and the odd queue groups of udma1 in the upper half of the bitset.
// Then, one or two calls of find_next_zero_bit can examine all the bits
// for queues of an entire udma.
//
// For example, assuming eight queue groups with qgrp qids per group:
//
// bitid 0*qgrp..1*qgrp-1 : qid 0*qgrp..1*qgrp-1
// bitid 1*qgrp..2*qgrp-1 : qid 2*qgrp..3*qgrp-1
// bitid 2*qgrp..3*qgrp-1 : qid 4*qgrp..5*qgrp-1
// bitid 3*qgrp..4*qgrp-1 : qid 6*qgrp..7*qgrp-1
// bitid 4*qgrp..5*qgrp-1 : qid 1*qgrp..2*qgrp-1
// bitid 5*qgrp..6*qgrp-1 : qid 3*qgrp..4*qgrp-1
// bitid 6*qgrp..7*qgrp-1 : qid 5*qgrp..6*qgrp-1
// bitid 7*qgrp..8*qgrp-1 : qid 7*qgrp..8*qgrp-1
//
// There are three important ranges of bits in the qid.  There is the udma
// bit "U" at qgrp_shift, which is the least significant bit of the group
// index, and determines which udma a queue is associated with.
// The bits of lesser significance we can call the idx bits "I", which are
// the index of the queue within the group.  The bits of greater significance
// we can call the grp bits "G", which are other bits of the group index that
// do not determine the udma.  Those bits are just rearranged in the bit index
// in the bitset.  A bitid has the udma bit in the most significant place,
// then the grp bits, then the idx bits.
//
// bitid: 00000000000000 U GGG IIIIII
// qid:   00000000000000 GGG U IIIIII
//
// Transforming from bit index to qid, or from qid to bit index, can be
// accomplished by rearranging the bits by masking and shifting.
//
// ionic_qid_to_bitid() - Transform a queue id into a resource bit index
// @qid:            queue index
// @qgrp_shift:     Log2 number of queues per queue group
// @half_qid_shift: Log2 of half the total number of queues
//
// Return: Resource bit index
//
// This is the inverse of ionic_bitid_to_qid().
//
