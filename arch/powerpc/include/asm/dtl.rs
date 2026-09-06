//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/dtl.h
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
// Layout of entries in the hypervisor's dispatch trace log buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtl_entry {
    pub dispatch_reason: u8,
    pub preempt_reason: u8,
    pub processor_id: __be16,
    pub enqueue_to_dispatch_time: __be32,
    pub ready_to_enqueue_time: __be32,
    pub waiting_to_ready_time: __be32,
    pub timebase: __be64,
    pub fault_addr: __be64,
    pub srr0: __be64,
    pub srr1: __be64,
}

//
// Dispatch trace log event enable mask:
// 0x1: voluntary virtual processor waits
// 0x2: time-slice preempts
// 0x4: virtual partition memory page faults
//
pub const DTL_LOG_CEDE: c_uint = 0x1;
pub const DTL_LOG_PREEMPT: c_uint = 0x2;
pub const DTL_LOG_FAULT: c_uint = 0x4;

extern "C" {
    pub fn register_dtl_buffer(cpu: c_int);
}
extern "C" {
    pub fn alloc_dtl_buffers(time_limit: *mut c_ulong);
}
