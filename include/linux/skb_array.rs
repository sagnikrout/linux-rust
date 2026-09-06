//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/skb_array.h
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
//
// Definitions for the 'struct skb_array' datastructure.
//
// Author:
// Michael S. Tsirkin <mst@redhat.com>
//
// Copyright (C) 2016 Red Hat, Inc.
//
// Limited-size FIFO of skbs. Can be used more or less whenever
// sk_buff_head can be used, except you need to know the queue size in
// advance.
// Implemented as a type-safe wrapper around ptr_ring.
//
pub const _LINUX_SKB_ARRAY_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_array {
    pub ring: ptr_ring,
}

// Might be slightly faster than skb_array_full below, but callers invoking
// this in a loop must use a compiler barrier, for example cpu_relax().
//
extern "C" {
    pub fn __ptr_ring_full(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_full(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_produce(_arg: &a->ring, _arg: skb) -> return;
}
extern "C" {
    pub fn ptr_ring_produce_irq(_arg: &a->ring, _arg: skb) -> return;
}
extern "C" {
    pub fn ptr_ring_produce_bh(_arg: &a->ring, _arg: skb) -> return;
}
extern "C" {
    pub fn ptr_ring_produce_any(_arg: &a->ring, _arg: skb) -> return;
}
// Might be slightly faster than skb_array_empty below, but only safe if the
// array is never resized. Also, callers invoking this in a loop must take care
// to use a compiler barrier, for example cpu_relax().
//
extern "C" {
    pub fn __ptr_ring_empty(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn __ptr_ring_peek(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_empty(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_empty_bh(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_empty_irq(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_empty_any(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn __ptr_ring_consume(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_consume(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_batched(_arg: &a->ring, )array: *mut (void, _arg: n) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_irq(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_batched_irq(_arg: &a->ring, )array: *mut (void, _arg: n) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_any(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_batched_any(_arg: &a->ring, )array: *mut (void, _arg: n) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_bh(_arg: &a->ring) -> return;
}
extern "C" {
    pub fn ptr_ring_consume_batched_bh(_arg: &a->ring, )array: *mut (void, _arg: n) -> return;
}
extern "C" {
    pub fn PTR_RING_PEEK_CALL(_arg: &a->ring, _arg: __skb_array_len_with_tag) -> return;
}
extern "C" {
    pub fn PTR_RING_PEEK_CALL_IRQ(_arg: &a->ring, _arg: __skb_array_len_with_tag) -> return;
}
extern "C" {
    pub fn PTR_RING_PEEK_CALL_BH(_arg: &a->ring, _arg: __skb_array_len_with_tag) -> return;
}
extern "C" {
    pub fn PTR_RING_PEEK_CALL_ANY(_arg: &a->ring, _arg: __skb_array_len_with_tag) -> return;
}
extern "C" {
    pub fn ptr_ring_init_noprof(_arg: &a->ring, _arg: size, _arg: gfp) -> return;
}

extern "C" {
    pub fn ptr_ring_resize(_arg: &a->ring, _arg: size, _arg: gfp, _arg: __skb_array_destroy_skb) -> return;
}

