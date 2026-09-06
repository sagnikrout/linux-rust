//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/tests/hists_common.h
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
pub const FAKE_PID_PERF1: c_int = 100;
pub const FAKE_PID_PERF2: c_int = 200;
pub const FAKE_PID_BASH: c_int = 300;
pub const FAKE_MAP_PERF: c_uint = 0x400000;
pub const FAKE_MAP_BASH: c_uint = 0x400000;
pub const FAKE_MAP_LIBC: c_uint = 0x500000;
pub const FAKE_MAP_KERNEL: c_uint = 0xf00000;
pub const FAKE_MAP_LENGTH: c_uint = 0x100000;
pub const FAKE_SYM_OFFSET1: c_int = 700;
pub const FAKE_SYM_OFFSET2: c_int = 800;
pub const FAKE_SYM_OFFSET3: c_int = 900;
pub const FAKE_SYM_LENGTH: c_int = 100;

//
// The setup_fake_machine() provides a test environment which consists
// of 3 processes that have 3 mappings and in turn, have 3 symbols
// respectively.  See below table:
//
// Command:  Pid  Shared Object               Symbol
// .............  .............  ...................
// perf:  100           perf  main
// perf:  100           perf  run_command
// perf:  100           perf  cmd_record
// perf:  100           libc  malloc
// perf:  100           libc  free
// perf:  100           libc  realloc
// perf:  100       [kernel]  schedule
// perf:  100       [kernel]  page_fault
// perf:  100       [kernel]  sys_perf_event_open
// perf:  200           perf  main
// perf:  200           perf  run_command
// perf:  200           perf  cmd_record
// perf:  200           libc  malloc
// perf:  200           libc  free
// perf:  200           libc  realloc
// perf:  200       [kernel]  schedule
// perf:  200       [kernel]  page_fault
// perf:  200       [kernel]  sys_perf_event_open
// bash:  300           bash  main
// bash:  300           bash  xmalloc
// bash:  300           bash  xfree
// bash:  300           libc  malloc
// bash:  300           libc  free
// bash:  300           libc  realloc
// bash:  300       [kernel]  schedule
// bash:  300       [kernel]  page_fault
// bash:  300       [kernel]  sys_perf_event_open
//
extern "C" {
    pub fn print_hists_in(hists: *mut hists);
}
extern "C" {
    pub fn print_hists_out(hists: *mut hists);
}
