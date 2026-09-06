//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/cluster/masklog.h
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
// Copyright (C) 2005 Oracle.  All rights reserved.
//
// For now this is a trivial wrapper around printk() that gives the critical
// ability to enable sets of debugging output at run-time.  In the future this
// will almost certainly be redirected to relayfs so that it can pay a
// substantially lower heisenberg tax.
//
// Callers associate the message with a bitmask and a global bitmask is
// maintained with help from /proc.  If any of the bits match the message is
// output.
//
// We must have efficient bit tests on i386 and it seems gcc still emits crazy
// code for the 64bit compare.  It emits very good code for the dual unsigned
// long tests, though, completely avoiding tests that can never pass if the
// caller gives a constant bitmask that fills one of the longs with all 0s.  So
// the desire is to have almost all of the calls decided on by comparing just
// one of the longs.  This leads to having infrequently given bits that are
// frequently matched in the high bits.
//
// _ERROR and _NOTICE are used for messages that always go to the console and
// have appropriate KERN_ prefixes.  We wrap these in our function instead of
// just calling printk() so that this can eventually make its way through
// relayfs along with the debugging messages.  Everything else gets KERN_DEBUG.
// The inline tests and macro dance give GCC the opportunity to quite cleverly
// only emit the appropriate printk() when the caller passes in a constant
// mask, as is almost always the case.
//
// All this bitmask nonsense is managed from the files under
// /sys/fs/o2cb/logmask/.  Reading the files gives a straightforward
// indication of which bits are allowed (allow) or denied (off/deny).
// ENTRY deny
// EXIT deny
// TCP off
// MSG off
// SOCKET off
// ERROR allow
// NOTICE allow
//
// Writing changes the state of a given bit and requires a strictly formatted
// single write() call:
//
// write(fd, "allow", 5);
//
// Echoing allow/deny/off string into the logmask files can flip the bits
// on or off as expected; here is the bash script for example:
//
// log_mask="/sys/fs/o2cb/log_mask"
// for node in ENTRY EXIT TCP MSG SOCKET ERROR NOTICE; do
// echo allow >"$log_mask"/"$node"
// done
//
// The debugfs.ocfs2 tool can also flip the bits with the -l option:
//
// debugfs.ocfs2 -l TCP allow
//
// for task_struct

// bits that are frequently given and infrequently matched in the low word
// NOTE: If you add a flag, you need to also update masklog.c!
pub const ML_TCP: c_uint = 0x0000000000000001ULL /* net cluster/tcp.c */;
pub const ML_MSG: c_uint = 0x0000000000000002ULL /* net network messages */;
pub const ML_SOCKET: c_uint = 0x0000000000000004ULL /* net socket lifetime */;
pub const ML_HEARTBEAT: c_uint = 0x0000000000000008ULL /* hb all heartbeat tracking */;
pub const ML_HB_BIO: c_uint = 0x0000000000000010ULL /* hb io tracing */;
pub const ML_DLMFS: c_uint = 0x0000000000000020ULL /* dlm user dlmfs */;
pub const ML_DLM: c_uint = 0x0000000000000040ULL /* dlm general debugging */;
pub const ML_DLM_DOMAIN: c_uint = 0x0000000000000080ULL /* dlm domain debugging */;
pub const ML_DLM_THREAD: c_uint = 0x0000000000000100ULL /* dlm domain thread */;
pub const ML_DLM_MASTER: c_uint = 0x0000000000000200ULL /* dlm master functions */;
pub const ML_DLM_RECOVERY: c_uint = 0x0000000000000400ULL /* dlm master functions */;
pub const ML_DLM_GLUE: c_uint = 0x0000000000000800ULL /* ocfs2 dlm glue layer */;
pub const ML_VOTE: c_uint = 0x0000000000001000ULL /* ocfs2 node messaging  */;
pub const ML_CONN: c_uint = 0x0000000000002000ULL /* net connection management */;
pub const ML_QUORUM: c_uint = 0x0000000000004000ULL /* net connection quorum */;
pub const ML_BASTS: c_uint = 0x0000000000008000ULL /* dlmglue asts and basts */;
pub const ML_CLUSTER: c_uint = 0x0000000000010000ULL /* cluster stack */;
// bits that are infrequently given and frequently matched in the high word
pub const ML_ERROR: c_uint = 0x1000000000000000ULL /* sent to KERN_ERR */;
pub const ML_NOTICE: c_uint = 0x2000000000000000ULL /* setn to KERN_NOTICE */;
pub const ML_KTHREAD: c_uint = 0x4000000000000000ULL /* kernel thread activity */;

pub const MLOG_MASK_PREFIX: c_int = 0;

//
// When logging is disabled, force the bit test to 0 for anything other
// than errors and notices, allowing gcc to remove the code completely.
// When enabled, allow all masks.
//

pub const MLOG_MAX_BITS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlog_bits {
    pub BITS_PER_LONG]: unsigned long words[MLOG_MAX_BITS /,
}

//
// Testing before the __mlog_printk call lets the compiler eliminate the
// call completely when (m & ML_ALLOWED_BITS) is 0.
//

extern "C" {
    pub fn mlog_sys_init(o2cb_subsys: *mut kset) -> c_int;
}
extern "C" {
    pub fn mlog_sys_shutdown();
}
