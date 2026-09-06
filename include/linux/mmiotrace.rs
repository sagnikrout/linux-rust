//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmiotrace.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmmio_probe {
// kmmio internal list:
    pub list: list_head,
// start location of the probe point:
    pub addr: c_ulong,
// length of the probe region:
    pub len: c_ulong,
// Called before addr is executed:
    pub pre_handler: kmmio_pre_handler_t,
// Called after addr is executed:
    pub post_handler: kmmio_post_handler_t,
    pub private: *mut c_void,
}

extern "C" {
    pub fn register_kmmio_probe(p: *mut kmmio_probe) -> c_int;
}
extern "C" {
    pub fn unregister_kmmio_probe(p: *mut kmmio_probe);
}
extern "C" {
    pub fn kmmio_init() -> c_int;
}
extern "C" {
    pub fn kmmio_cleanup();
}

// kmmio is active by some kmmio_probes?
// Called from page fault handler.
extern "C" {
    pub fn kmmio_handler(regs: *mut pt_regs, addr: c_ulong) -> c_int;
}
// Called from ioremap.c
extern "C" {
    pub fn mmiotrace_iounmap(addr: *mut volatile void __iomem);
}
// For anyone to insert markers. Remember trailing newline.
extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 2) int mmiotrace_printk(char, ...) -> extern;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm_io_opcode {
    MMIO_READ	= 0x1,	/* struct mmiotrace_rw */
    MMIO_WRITE	= 0x2,	/* struct mmiotrace_rw */
    MMIO_PROBE	= 0x3,	/* struct mmiotrace_map */
    MMIO_UNPROBE	= 0x4,	/* struct mmiotrace_map */
    MMIO_UNKNOWN_OP = 0x5,	/* struct mmiotrace_rw */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmiotrace_rw {
    pub /: *mut *mut resource_size_t phys; / PCI address of register,
    pub value: c_ulong,
    pub /: *mut *mut unsigned long pc; / optional program counter,
    pub map_id: c_int,
    pub /: *mut *mut unsigned char opcode; / one of MMIO_{READ,WRITE,UNKNOWN_OP},
    pub /: *mut *mut unsigned char width; / size of register access in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmiotrace_map {
    pub /: *mut *mut resource_size_t phys; / base address in PCI space,
    pub /: *mut *mut unsigned long virt; / base virtual address,
    pub /: *mut *mut unsigned long len; / mapping size,
    pub map_id: c_int,
    pub /: *mut *mut unsigned char opcode; / MMIO_PROBE or MMIO_UNPROBE,
}

// in kernel/trace/trace_mmiotrace.c
extern "C" {
    pub fn enable_mmiotrace();
}
extern "C" {
    pub fn disable_mmiotrace();
}
extern "C" {
    pub fn mmio_trace_rw(rw: *mut mmiotrace_rw);
}
extern "C" {
    pub fn mmio_trace_mapping(map: *mut mmiotrace_map);
}
extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 0) int mmio_trace_printk(char, args: va_list) -> extern;
}
