//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/watch_queue.h
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
// User-mappable watch queue
//
// Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// See Documentation/core-api/watch_queue.rst
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_type_filter {
    pub type: watch_notification_type,
    pub /: *mut *mut __u32 subtype_filter[1]; / Bitmask of subtypes to filter on,
    pub /: *mut *mut __u32 info_filter; / Filter on watch_notification::info,
    pub /: *mut *mut __u32 info_mask; / Mask of relevant bits in info_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_filter {
    pub rcu: rcu_head,
// Bitmask of accepted types
    pub WATCH_TYPE__NR): DECLARE_BITMAP(type_filter,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_queue {
    pub rcu: rcu_head,
    pub filter: *mut watch_filter __rcu,
    pub /: *mut *mut *mut pipe_inode_info pipe; / Pipe we use as a buffer, NULL if queue closed,
    pub /: *mut *mut hlist_head watches; / Contributory watches,
    pub /: *mut *mut *mut *mut page notes; / Preallocated notifications,
    pub /: *mut *mut *mut unsigned long notes_bitmap; / Allocation bitmap for notes,
    pub /: *mut *mut kref usage; / Object usage count,
    pub lock: spinlock_t,
    pub /: *mut *mut unsigned int nr_notes; / Number of notes,
    pub /: *mut *mut unsigned int nr_pages; / Number of pages in notes[],
}

//
// Representation of a watch on an object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch {
    pub rcu: rcu_head,
    pub /: *mut *mut u32 info_id; / ID to be OR'd in to info field,
}

//
// List of watches on an object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_list {
    pub rcu: rcu_head,
    pub watchers: hlist_head,
    pub ): *mut *mut void (release_watch)(struct watch,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn put_watch_queue(: *mut watch_queue);
}
extern "C" {
    pub fn init_watch(: *mut watch, : *mut watch_queue);
}
extern "C" {
    pub fn add_watch_to_object(: *mut watch, : *mut watch_list) -> c_int;
}
extern "C" {
    pub fn remove_watch_from_object(: *mut watch_list, : *mut watch_queue, _arg: u64, _arg: bool) -> c_int;
}
extern "C" {
    pub fn watch_queue_set_size(: *mut pipe_inode_info, int: unsigned) -> c_long;
}
extern "C" {
    pub fn watch_queue_init(: *mut pipe_inode_info) -> c_int;
}
extern "C" {
    pub fn watch_queue_clear(: *mut watch_queue);
}
//
// watch_sizeof - Calculate the information part of the size of a watch record,
// given the structure size.
//

