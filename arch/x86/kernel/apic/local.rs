//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/apic/local.h
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
// Historical copyright notices:
//
// Copyright 2004 James Cleverdon, IBM.
// (c) 1995 Alan Cox, Building #3 <alan@redhat.com>
// (c) 1998-99, 2000 Ingo Molnar <mingo@redhat.com>
// (c) 2002,2003 Andi Kleen, SuSE Labs.
//

// X2APIC
extern "C" {
    pub fn x2apic_get_apic_id(id: u32) -> u32;
}
extern "C" {
    pub fn x2apic_send_IPI_all(vector: c_int);
}
extern "C" {
    pub fn x2apic_send_IPI_allbutself(vector: c_int);
}
extern "C" {
    pub fn x2apic_send_IPI_self(vector: c_int);
}
// IPI

extern "C" {
    pub fn default_init_apic_ldr();
}
extern "C" {
    pub fn apic_mem_wait_icr_idle();
}
extern "C" {
    pub fn apic_mem_wait_icr_idle_timeout() -> u32;
}
//
// This is used to send an IPI with no shorthand notation (the destination is
// specified in bits 56 to 63 of the ICR).
//
extern "C" {
    pub fn __default_send_IPI_dest_field(mask: c_uint, vector: c_int, dest: c_uint);
}
extern "C" {
    pub fn default_send_IPI_single(cpu: c_int, vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_single_phys(cpu: c_int, vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_mask_sequence_phys(mask: *const cpumask, vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_mask_allbutself_phys(mask: *const cpumask, vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_allbutself(vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_all(vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_self(vector: c_int);
}

extern "C" {
    pub fn default_send_IPI_mask_sequence_logical(mask: *const cpumask, vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_mask_allbutself_logical(mask: *const cpumask, vector: c_int);
}
extern "C" {
    pub fn default_send_IPI_mask_logical(mask: *const cpumask, vector: c_int);
}
