//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/landlock/audit.h
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
// Landlock audit helpers
//
// Copyright © 2024-2025 Microsoft Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_filter {
    pub record_type: __u32,
    pub exe_len: usize,
    pub exe: [c_char; PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_message {
    pub header: nlmsghdr,
    pub status: audit_status,
    pub features: audit_features,
    pub rule: audit_rule_data,
    pub err: nlmsgerr,
    pub 200]: char data[PATH_MAX +,
}

//
// Default socket timeout for audit_match_record() callers that expect a
// record to arrive.  Asynchronous kauditd delivery can exceed 1 usec
// under heavy debug configs (KASAN, lockdep), where kauditd_thread
// scheduling between audit_log_end() and netlink_unicast() takes longer
// than the previous 1 usec timeout. 1 second is a generous ceiling: on
// the happy path, kauditd delivers within dozens of usec.
//
// Fast timeout for paths that expect no record (audit_init() drain,
// audit_count_records(), probes).  Causes audit_recv() to return
// -EAGAIN once the socket buffer is empty, naturally terminating the
// read loop.
//
// Checks Netlink error or end of messages.
extern "C" {
    pub fn audit_request(_arg: audit_fd, _arg: &msg, _arg: NULL) -> return;
}
extern "C" {
    pub fn audit_request(_arg: audit_fd, _arg: &msg, _arg: NULL) -> return;
}
extern "C" {
    pub fn audit_request(_arg: fd, _arg: &msg, _arg: NULL) -> return;
}
//
// @domain_id: The domain ID extracted from the audit message (if the first part
// of @pattern is REGEX_LANDLOCK_PREFIX).  It is set to 0 if the domain ID is
// not found.
//
// Reads records until one matches both the expected type and the
// pattern.  Type-matching records with non-matching content are
// silently consumed, which handles stale domain deallocation records
// from a previous test emitted asynchronously by kworker threads.
//
// domain_id = 0;
// The maximal characters of a 2^64 hexadecimal number is 17.
// domain_id = strtoull(dom_id, NULL, 16);
//
// Matches a domain deallocation record.  When expected_domain_id is non-zero,
// the pattern includes the specific domain ID so that stale deallocation
// records from a previous test (with a different domain ID) are skipped by
// audit_match_record(), waiting for the asynchronous kworker deallocation with
// the default patient timeout.
//
// When expected_domain_id is zero, the caller is probing for any dealloc record
// that may or may not arrive.  Temporarily lowers the socket timeout to
// audit_tv_fast for this probe so it returns promptly when no record is
// pending; restores audit_tv_default after.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_records {
    pub access: usize,
    pub domain: usize,
}

//
// Counts remaining audit records by type, skipping domain deallocation records.
// Deallocation records are emitted asynchronously from kworker threads after a
// previous test's child has exited, so they can arrive after the drain in
// audit_init() and after the preceding audit_match_record() call.  Allocation
// records are emitted synchronously during landlock_log_denial() in the current
// test's syscall context, so only those are counted in records->domain.
//
// Temporarily lowers SO_RCVTIMEO to audit_tv_fast for the read loop: this is a
// "no record expected" path that should terminate on the first -EAGAIN.  The
// default patient timeout is restored on exit for subsequent
// audit_match_record() callers.
//
// Uses the fast timeout to drain stale records below.
//
// Drains stale audit records that accumulated in the kernel backlog
// while no audit daemon socket was open.  This happens when non-audit
// Landlock tests generate records while audit_enabled is non-zero (e.g.
// from boot configuration), or when domain deallocation records arrive
// asynchronously after a previous test's socket was closed.
//
// Restores the default timeout for audit_match_record() callers that
// expect a record to arrive.  Paths that expect no record restore the
// fast timeout locally (audit_count_records(), the expected_domain_id
// == 0 probe in matches_log_domain_deallocated()).
//
// It is assume that there is not already filtering rules.
// No need for the terminating NULL byte.
//
// Simulates audit_init_with_exe_filter() when called from
// FIXTURE_TEARDOWN_PARENT().
//
// Filters might not be in place.
