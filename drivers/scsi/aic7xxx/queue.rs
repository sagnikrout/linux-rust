//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/queue.h
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


//
// Copyright (c) 1991, 1993
// The Regents of the University of California.  All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//
// @(#)queue.h	8.5 (Berkeley) 8/20/94
// $FreeBSD: src/sys/sys/queue.h,v 1.38 2000/05/26 02:06:56 jake Exp $
//
// This file defines five types of data structures: singly-linked lists,
// singly-linked tail queues, lists, tail queues, and circular queues.
//
// A singly-linked list is headed by a single forward pointer. The elements
// are singly linked for minimum space and pointer manipulation overhead at
// the expense of O(n) removal for arbitrary elements. New elements can be
// added to the list after an existing element or at the head of the list.
// Elements being removed from the head of the list should use the explicit
// macro for this purpose for optimum efficiency. A singly-linked list may
// only be traversed in the forward direction.  Singly-linked lists are ideal
// for applications with large datasets and few or no removals or for
// implementing a LIFO queue.
//
// A singly-linked tail queue is headed by a pair of pointers, one to the
// head of the list and the other to the tail of the list. The elements are
// singly linked for minimum space and pointer manipulation overhead at the
// expense of O(n) removal for arbitrary elements. New elements can be added
// to the list after an existing element, at the head of the list, or at the
// end of the list. Elements being removed from the head of the tail queue
// should use the explicit macro for this purpose for optimum efficiency.
// A singly-linked tail queue may only be traversed in the forward direction.
// Singly-linked tail queues are ideal for applications with large datasets
// and few or no removals or for implementing a FIFO queue.
//
// A list is headed by a single forward pointer (or an array of forward
// pointers for a hash table header). The elements are doubly linked
// so that an arbitrary element can be removed without a need to
// traverse the list. New elements can be added to the list before
// or after an existing element or at the head of the list. A list
// may only be traversed in the forward direction.
//
// A tail queue is headed by a pair of pointers, one to the head of the
// list and the other to the tail of the list. The elements are doubly
// linked so that an arbitrary element can be removed without a need to
// traverse the list. New elements can be added to the list before or
// after an existing element, at the head of the list, or at the end of
// the list. A tail queue may be traversed in either direction.
//
// A circle queue is headed by a pair of pointers, one to the head of the
// list and the other to the tail of the list. The elements are doubly
// linked so that an arbitrary element can be removed without a need to
// traverse the list. New elements can be added to the list before or after
// an existing element, at the head of the list, or at the end of the list.
// A circle queue may be traversed in either direction, but has a more
// complex end of list detection.
//
// For details on the use of these macros, see the queue(3) manual page.
//
// SLIST	LIST	STAILQ	TAILQ	CIRCLEQ
// _HEAD		+	+	+	+	+
// _HEAD_INITIALIZER	+	+	+	+	+
// _ENTRY		+	+	+	+	+
// _INIT		+	+	+	+	+
// _EMPTY		+	+	+	+	+
// _FIRST		+	+	+	+	+
// _NEXT		+	+	+	+	+
// _PREV		-	-	-	+	+
// _LAST		-	-	+	+	+
// _FOREACH		+	+	+	+	+
// _FOREACH_REVERSE	-	-	-	+	+
// _INSERT_HEAD		+	+	+	+	+
// _INSERT_BEFORE	-	+	-	+	+
// _INSERT_AFTER	+	+	+	+	+
// _INSERT_TAIL		-	-	+	+	+
// _REMOVE_HEAD		+	-	+	-	-
// _REMOVE		+	+	+	+	+
//
// Singly-linked List declarations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name {
    pub \: *mut *mut *mut *mut type slh_first; / first element /,

    pub \: *mut *mut *mut *mut type sle_next; / next element /,
//
// Singly-linked List functions.
//

    pub \: for ((var) = SLIST_FIRST((head));,
    pub \: (var);,

    pub \: SLIST_FIRST((head)) = NULL;,

    pub \: SLIST_NEXT((elm), field) = SLIST_NEXT((slistelm), field);,
    pub \: SLIST_NEXT((slistelm), field) = (elm);,

    pub \: SLIST_NEXT((elm), field) = SLIST_FIRST((head));,
    pub \: SLIST_FIRST((head)) = (elm);,

    pub \: SLIST_REMOVE_HEAD((head), field);,
    pub \: *mut *mut type curelm = SLIST_FIRST((head));,
    pub \: curelm = SLIST_NEXT(curelm, field);,
    pub \: SLIST_NEXT(SLIST_NEXT(curelm, field), field);,

    pub \: SLIST_FIRST((head)) = SLIST_NEXT(SLIST_FIRST((head)), field);,
//
// Singly-linked Tail queue declarations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name {
    pub \: *mut *mut *mut *mut type stqh_first;/ first element /,
    pub \: *mut *mut *mut *mut *mut type stqh_last;/ addr of last next element /,

    pub \: *mut *mut *mut *mut type stqe_next; / next element /,
//
// Singly-linked Tail queue functions.
//

    pub \: for((var) = STAILQ_FIRST((head));,
    pub \: (var);,

    pub \: STAILQ_FIRST((head)) = NULL;,
    pub \: (head)->stqh_last = &STAILQ_FIRST((head));,

    pub \: (head)->stqh_last = &STAILQ_NEXT((elm), field);,
    pub \: STAILQ_NEXT((tqelm), field) = (elm);,

    pub \: (head)->stqh_last = &STAILQ_NEXT((elm), field);,
    pub \: STAILQ_FIRST((head)) = (elm);,

    pub \: STAILQ_NEXT((elm), field) = NULL;,
    pub \: STAILQ_LAST((head)) = (elm);,
    pub \: (head)->stqh_last = &STAILQ_NEXT((elm), field);,

    pub \: STAILQ_REMOVE_HEAD(head, field);,
    pub \: *mut *mut type curelm = STAILQ_FIRST((head));,
    pub \: curelm = STAILQ_NEXT(curelm, field);,
    pub field);\: (head)->stqh_last = &STAILQ_NEXT((curelm),,

    pub \: (head)->stqh_last = &STAILQ_FIRST((head));,

    pub \: (head)->stqh_last = &STAILQ_FIRST((head));,
//
// List declarations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name {
    pub \: *mut *mut *mut *mut type lh_first; / first element /,

    pub \: *mut *mut *mut *mut type le_next; / next element /,
    pub \: *mut *mut *mut *mut *mut type le_prev; / address of previous next element /,
//
// List functions.
//

    pub \: for ((var) = LIST_FIRST((head));,
    pub \: (var);,

    pub \: LIST_FIRST((head)) = NULL;,

    pub \: &LIST_NEXT((elm), field);,
    pub \: LIST_NEXT((listelm), field) = (elm);,
    pub \: (elm)->field.le_prev = &LIST_NEXT((listelm), field);,

    pub \: (elm)->field.le_prev = (listelm)->field.le_prev;,
    pub \: LIST_NEXT((elm), field) = (listelm);,
// (listelm)->field.le_prev = (elm);				\
    pub \: (listelm)->field.le_prev = &LIST_NEXT((elm), field);,

    pub field);\: LIST_FIRST((head))->field.le_prev = &LIST_NEXT((elm),,
    pub \: LIST_FIRST((head)) = (elm);,
    pub \: (elm)->field.le_prev = &LIST_FIRST((head));,

    pub \: (elm)->field.le_prev;,
// (elm)->field.le_prev = LIST_NEXT((elm), field);		\
//
// Tail queue declarations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name {
    pub \: *mut *mut *mut *mut type tqh_first; / first element /,
    pub \: *mut *mut *mut *mut *mut type tqh_last; / addr of last next element /,

    pub \: *mut *mut *mut *mut type tqe_next; / next element /,
    pub \: *mut *mut *mut *mut *mut type tqe_prev; / address of previous next element /,
//
// Tail queue functions.
//

    pub \: for ((var) = TAILQ_FIRST((head));,
    pub \: (var);,

    pub \: for ((var) = TAILQ_LAST((head), headname);,
    pub \: (var);,

    pub \: TAILQ_FIRST((head)) = NULL;,
    pub \: (head)->tqh_last = &TAILQ_FIRST((head));,

    pub \: &TAILQ_NEXT((elm), field);,
    pub \: (head)->tqh_last = &TAILQ_NEXT((elm), field);,
    pub \: TAILQ_NEXT((listelm), field) = (elm);,
    pub \: (elm)->field.tqe_prev = &TAILQ_NEXT((listelm), field);,

    pub \: (elm)->field.tqe_prev = (listelm)->field.tqe_prev;,
    pub \: TAILQ_NEXT((elm), field) = (listelm);,
// (listelm)->field.tqe_prev = (elm);				\
    pub \: (listelm)->field.tqe_prev = &TAILQ_NEXT((elm), field);,

    pub \: &TAILQ_NEXT((elm), field);,
    pub \: (head)->tqh_last = &TAILQ_NEXT((elm), field);,
    pub \: TAILQ_FIRST((head)) = (elm);,
    pub \: (elm)->field.tqe_prev = &TAILQ_FIRST((head));,

    pub \: TAILQ_NEXT((elm), field) = NULL;,
    pub \: (elm)->field.tqe_prev = (head)->tqh_last;,
// (head)->tqh_last = (elm);					\
    pub \: (head)->tqh_last = &TAILQ_NEXT((elm), field);,

    pub \: (elm)->field.tqe_prev;,
    pub \: (head)->tqh_last = (elm)->field.tqe_prev;,
// (elm)->field.tqe_prev = TAILQ_NEXT((elm), field);		\
//
// Circular queue declarations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name {
    pub \: *mut *mut *mut *mut type cqh_first; / first element /,
    pub \: *mut *mut *mut *mut type cqh_last; / last element /,

    pub \: *mut *mut *mut *mut type cqe_next; / next element /,
    pub \: *mut *mut *mut *mut type cqe_prev; / previous element /,
//
// Circular queue functions.
//

    pub \: for ((var) = CIRCLEQ_FIRST((head));,
    pub \: *mut *mut (var) != (void )(head);,

    pub \: for ((var) = CIRCLEQ_LAST((head));,
    pub \: *mut *mut (var) != (void )(head);,

    pub \: *mut *mut CIRCLEQ_FIRST((head)) = (void )(head);,
    pub \: *mut *mut CIRCLEQ_LAST((head)) = (void )(head);,

    pub \: CIRCLEQ_NEXT((elm), field) = CIRCLEQ_NEXT((listelm), field);,
    pub \: CIRCLEQ_PREV((elm), field) = (listelm);,
    pub \: CIRCLEQ_LAST((head)) = (elm);,
    pub (elm);\: CIRCLEQ_PREV(CIRCLEQ_NEXT((listelm), field), field) =,
    pub \: CIRCLEQ_NEXT((listelm), field) = (elm);,

    pub \: CIRCLEQ_NEXT((elm), field) = (listelm);,
    pub \: CIRCLEQ_PREV((elm), field) = CIRCLEQ_PREV((listelm), field);,
    pub \: CIRCLEQ_FIRST((head)) = (elm);,
    pub (elm);\: CIRCLEQ_NEXT(CIRCLEQ_PREV((listelm), field), field) =,
    pub \: CIRCLEQ_PREV((listelm), field) = (elm);,

    pub \: CIRCLEQ_NEXT((elm), field) = CIRCLEQ_FIRST((head));,
    pub \: *mut *mut CIRCLEQ_PREV((elm), field) = (void )(head);,
    pub \: CIRCLEQ_LAST((head)) = (elm);,
    pub \: CIRCLEQ_PREV(CIRCLEQ_FIRST((head)), field) = (elm);,
    pub \: CIRCLEQ_FIRST((head)) = (elm);,

    pub \: *mut *mut CIRCLEQ_NEXT((elm), field) = (void )(head);,
    pub \: CIRCLEQ_PREV((elm), field) = CIRCLEQ_LAST((head));,
    pub \: CIRCLEQ_FIRST((head)) = (elm);,
    pub \: CIRCLEQ_NEXT(CIRCLEQ_LAST((head)), field) = (elm);,
    pub \: CIRCLEQ_LAST((head)) = (elm);,

    pub \: CIRCLEQ_LAST((head)) = CIRCLEQ_PREV((elm), field);,
    pub \: CIRCLEQ_PREV((elm), field);,
    pub \: CIRCLEQ_FIRST((head)) = CIRCLEQ_NEXT((elm), field);,
    pub \: CIRCLEQ_NEXT((elm), field);,
