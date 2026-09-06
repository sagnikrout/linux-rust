//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/tlb.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// TLB Invalidate Flush
//
extern "C" {
    pub fn __volatile__(_arg: "tlbclr") -> __asm__;
}
extern "C" {
    pub fn __volatile__(_arg: "tlbflush") -> __asm__;
}
//
// TLB R/W operations.
//
extern "C" {
    pub fn __volatile__(_arg: "tlbsrch") -> __asm__;
}
extern "C" {
    pub fn __volatile__(_arg: "tlbrd") -> __asm__;
}
extern "C" {
    pub fn __volatile__(_arg: "tlbwr") -> __asm__;
}
extern "C" {
    pub fn __volatile__(_arg: "tlbfill") -> __asm__;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum invtlb_ops {
// Invalid all tlb
    INVTLB_ALL = 0x0,
// Invalid current tlb
    INVTLB_CURRENT_ALL = 0x1,
// Invalid all global=1 lines in current tlb
    INVTLB_CURRENT_GTRUE = 0x2,
// Invalid all global=0 lines in current tlb
    INVTLB_CURRENT_GFALSE = 0x3,
// Invalid global=0 and matched asid lines in current tlb
    INVTLB_GFALSE_AND_ASID = 0x4,
// Invalid addr with global=0 and matched asid in current tlb
    INVTLB_ADDR_GFALSE_AND_ASID = 0x5,
// Invalid addr with global=1 or matched asid in current tlb
    INVTLB_ADDR_GTRUE_OR_ASID = 0x6,
// Invalid matched gid in guest tlb
    INVGTLB_GID = 0x9,
// Invalid global=1, matched gid in guest tlb
    INVGTLB_GID_GTRUE = 0xa,
// Invalid global=0, matched gid in guest tlb
    INVGTLB_GID_GFALSE = 0xb,
// Invalid global=0, matched gid and asid in guest tlb
    INVGTLB_GID_GFALSE_ASID = 0xc,
// Invalid global=0 , matched gid, asid and addr in guest tlb
    INVGTLB_GID_GFALSE_ASID_ADDR = 0xd,
// Invalid global=1 , matched gid, asid and addr in guest tlb
    INVGTLB_GID_GTRUE_ASID_ADDR = 0xe,
// Invalid all gid gva-->gpa guest tlb
    INVGTLB_ALLGID_GVA_TO_GPA = 0x10,
// Invalid all gid gpa-->hpa tlb
    INVTLB_ALLGID_GPA_TO_HPA = 0x11,
// Invalid all gid tlb, including  gva-->gpa and gpa-->hpa
    INVTLB_ALLGID = 0x12,
// Invalid matched gid gva-->gpa guest tlb
    INVGTLB_GID_GVA_TO_GPA = 0x13,
// Invalid matched gid gpa-->hpa tlb
    INVTLB_GID_GPA_TO_HPA = 0x14,
// Invalid matched gid tlb,including gva-->gpa and gpa-->hpa
    INVTLB_GID_ALL = 0x15,
// Invalid matched gid and addr gpa-->hpa tlb
    INVTLB_GID_ADDR = 0x16,
}

extern "C" {
    pub fn tlb_flush(tlb: *mut mmu_gather) -> static void;
}

extern "C" {
    pub fn handle_tlb_load();
}
extern "C" {
    pub fn handle_tlb_store();
}
extern "C" {
    pub fn handle_tlb_modify();
}
extern "C" {
    pub fn handle_tlb_refill();
}
extern "C" {
    pub fn handle_tlb_protect();
}
extern "C" {
    pub fn handle_tlb_load_ptw();
}
extern "C" {
    pub fn handle_tlb_store_ptw();
}
extern "C" {
    pub fn handle_tlb_modify_ptw();
}
extern "C" {
    pub fn dump_tlb_all();
}
extern "C" {
    pub fn dump_tlb_regs();
}
