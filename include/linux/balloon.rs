//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/balloon.h
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
// Common interface for implementing a memory balloon, including support
// for migration of pages inflated in a memory balloon.
//
// Balloon page migration makes use of the general "movable_ops page migration"
// feature.
//
// page->private is used to reference the responsible balloon device.
// That these pages have movable_ops, and which movable_ops apply,
// is derived from the page type (PageOffline()) combined with the
// PG_movable_ops flag (PageMovableOps()).
//
// Once the page type and the PG_movable_ops are set, migration code
// can initiate page isolation by invoking the
// movable_operations()->isolate_page() callback
//
// As long as page->private is set, the page is either on the balloon list
// or isolated for migration. If page->private is not set, the page is
// either still getting inflated, or was deflated to be freed by the balloon
// driver soon. Isolation is impossible in both cases.
//
// As the page isolation scanning step a compaction thread does is a lockless
// procedure (from a page standpoint), it might bring some racy situations while
// performing balloon page migration. In order to sort out these racy scenarios
// and safely perform balloon's page migration we must, always, ensure following
// these simple rules:
//
// i. Inflation/deflation must set/clear page->private under the
// balloon_pages_lock
//
// ii. isolation or dequeueing procedure must remove the page from balloon
// device page list under balloon_pages_lock
//
// Copyright (C) 2012, Red Hat, Inc.  Rafael Aquini <aquini@redhat.com>
//

//
// Balloon device information descriptor.
// This struct is used to allow the common balloon page migration interface
// procedures to find the proper balloon device holding memory pages they'll
// have to cope for page migration, as well as it serves the balloon driver as
// a page book-keeper for its registered balloon devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct balloon_dev_info {
    pub /: *mut *mut unsigned long isolated_pages; / # of isolated pages for migration,
    pub /: *mut *mut list_head pages; / Pages enqueued & handled to Host,
    pub mode): *mut *mut page page, enum migrate_mode,
    pub adjust_managed_page_count: bool,
}
