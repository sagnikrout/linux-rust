//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/intel_ds.h
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


pub const PEBS_BUFFER_SHIFT: c_int = 4;

//
// The largest PEBS record could consume a page, ensure
// a record at least can be written after triggering PMI.
//

pub const ARCH_PEBS_THRESH_SINGLE: c_int = 1;
// The maximal number of PEBS events:
pub const MAX_PEBS_EVENTS_FMT4: c_int = 8;
pub const MAX_PEBS_EVENTS: c_int = 32;

pub const MAX_FIXED_PEBS_EVENTS: c_int = 16;
//
// A debug store configuration.
//
// We only support architectures that use 64bit fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_store {
    pub bts_buffer_base: u64,
    pub bts_index: u64,
    pub bts_absolute_maximum: u64,
    pub bts_interrupt_threshold: u64,
    pub pebs_buffer_base: u64,
    pub pebs_index: u64,
    pub pebs_absolute_maximum: u64,
    pub pebs_interrupt_threshold: u64,
    pub MAX_FIXED_PEBS_EVENTS]: u64 pebs_event_reset[MAX_PEBS_EVENTS +,
    pub __aligned(PAGE_SIZE): },
    pub cpu_debug_store): DECLARE_PER_CPU_PAGE_ALIGNED(struct debug_store,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_store_buffers {
    pub bts_buffer: [c_char; BTS_BUFFER_SIZE],
    pub pebs_buffer: [c_char; PEBS_BUFFER_SIZE],
}
