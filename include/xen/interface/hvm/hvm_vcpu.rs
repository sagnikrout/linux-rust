//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/hvm/hvm_vcpu.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (c) 2015, Roger Pau Monne <roger.pau@citrix.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_hvm_x86_32 {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub eip: u32,
    pub eflags: u32,
    pub cr0: u32,
    pub cr3: u32,
    pub cr4: u32,
    pub pad1: u32,
//
// EFER should only be used to set the NXE bit (if required)
// when starting a vCPU in 32bit mode with paging enabled or
// to set the LME/LMA bits in order to start the vCPU in
// compatibility mode.
//
    pub efer: u64,
    pub cs_base: u32,
    pub ds_base: u32,
    pub ss_base: u32,
    pub es_base: u32,
    pub tr_base: u32,
    pub cs_limit: u32,
    pub ds_limit: u32,
    pub ss_limit: u32,
    pub es_limit: u32,
    pub tr_limit: u32,
    pub cs_ar: u16,
    pub ds_ar: u16,
    pub ss_ar: u16,
    pub es_ar: u16,
    pub tr_ar: u16,
    pub pad2: [u16; 3],
}

//
// The layout of the _ar fields of the segment registers is the
// following:
//
// Bits   [0,3]: type (bits 40-43).
// Bit        4: s    (descriptor type, bit 44).
// Bit    [5,6]: dpl  (descriptor privilege level, bits 45-46).
// Bit        7: p    (segment-present, bit 47).
// Bit        8: avl  (available for system software, bit 52).
// Bit        9: l    (64-bit code segment, bit 53).
// Bit       10: db   (meaning depends on the segment, bit 54).
// Bit       11: g    (granularity, bit 55)
// Bits [12,15]: unused, must be blank.
//
// A more complete description of the meaning of this fields can be
// obtained from the Intel SDM, Volume 3, section 3.4.5.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_hvm_x86_64 {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cr0: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub efer: u64,
//
// Using VCPU_HVM_MODE_64B implies that the vCPU is launched
// directly in long mode, so the cached parts of the segment
// registers get set to match that environment.
//
// If the user wants to launch the vCPU in compatibility mode
// the 32-bit structure should be used instead.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_hvm_context {

    pub mode: u32,
    pub pad: u32,
// CPU registers.
    pub x86_32: vcpu_hvm_x86_32,
    pub x86_64: vcpu_hvm_x86_64,
    pub cpu_regs: },
}

pub type vcpu_hvm_context_t = vcpu_hvm_context;
