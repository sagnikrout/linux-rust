//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/posted_intr.h
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

pub const POSTED_INTR_ON: c_int = 0;
pub const POSTED_INTR_SN: c_int = 1;
pub const PID_TABLE_ENTRY_VALID: c_int = 1;
pub const NR_PIR_VECTORS: c_int = 256;

// Posted-Interrupt Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pi_desc {
    pub /: *mut *mut unsigned long pir[NR_PIR_WORDS]; / Posted interrupt requested,
    pub /: *mut *mut u16 notifications; / Suppress and outstanding bits,
    pub nv: u8,
    pub rsvd_2: u8,
    pub ndst: u32,
}

//
// De-multiplexing posted interrupts is on the performance path, the code
// below is written to optimize the cache performance based on the following
// considerations:
// 1.Posted interrupt descriptor (PID) fits in a cache line that is frequently
// accessed by both CPU and IOMMU.
// 2.During software processing of posted interrupts, the CPU needs to do
// natural width read and xchg for checking and clearing posted interrupt
// request (PIR), a 256 bit field within the PID.
// 3.On the other side, the IOMMU does atomic swaps of the entire PID cache
// line when posting interrupts and setting control bits.
// 4.The CPU can access the cache line a magnitude faster than the IOMMU.
// 5.Each time the IOMMU does interrupt posting to the PIR will evict the PID
// cache line. The cache line states after each operation are as follows,
// assuming a 64-bit kernel:
// CPU		IOMMU			PID Cache line state
// ---------------------------------------------------------------
// ...read64					exclusive
// ...lock xchg64				modified
// ...			post/atomic swap	invalid
// ...-------------------------------------------------------------
//
// To reduce L1 data cache miss, it is important to avoid contention with
// IOMMU's interrupt posting/atomic swap. Therefore, a copy of PIR is used
// when processing posted interrupts in software, e.g. to dispatch interrupt
// handlers for posted MSIs, or to move interrupts from the PIR to the vIRR
// in KVM.
//
// In addition, the code is trying to keep the cache line state consistent
// as much as possible. e.g. when making a copy and clearing the PIR
// (assuming non-zero PIR bits are present in the entire PIR), it does:
// read, read, read, read, xchg, xchg, xchg, xchg
// instead of:
// read, xchg, read, xchg, read, xchg, read, xchg
//
extern "C" {
    pub fn test_and_set_bit(_arg: POSTED_INTR_ON, )&pi_desc->control: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: POSTED_INTR_ON, )&pi_desc->control: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: POSTED_INTR_SN, )&pi_desc->control: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: vector, _arg: pi_desc->pir) -> return;
}
extern "C" {
    pub fn bitmap_empty(_arg: pi_desc->pir, _arg: NR_VECTORS) -> return;
}
extern "C" {
    pub fn test_bit(_arg: POSTED_INTR_ON, )&pi_desc->control: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_bit(_arg: POSTED_INTR_SN, )&pi_desc->control: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_bit(_arg: vector, )pi_desc->pir: *mut (unsigned long) -> return;
}
// Non-atomic helpers

//
// Not all external vectors are subject to interrupt remapping, e.g. IOMMU's
// own interrupts. Here we do not distinguish them since those vector bits in
// PIR will always be zero.
//
extern "C" {
    pub fn test_bit(_arg: vector, _arg: pid->pir) -> return;
}
extern "C" {
    pub fn intel_posted_msi_init();
}

