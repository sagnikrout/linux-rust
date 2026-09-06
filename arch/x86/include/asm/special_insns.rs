//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/special_insns.h
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

extern "C" {
    pub fn native_write_cr0(val: c_ulong);
}
extern "C" {
    pub fn volatile(%%cr0: "mov, (val): %0" : "=r") -> asm;
}
extern "C" {
    pub fn volatile(%%cr2: "mov, (val): %0" : "=r") -> asm;
}
extern "C" {
    pub fn volatile(%0: "mov, "memory": %%cr2": : "r" (val) :) -> asm;
}
extern "C" {
    pub fn volatile(%%cr3: "mov, (val): %0" : "=r") -> asm;
}
extern "C" {
    pub fn volatile(%0: "mov, "memory": %%cr3": : "r" (val) :) -> asm;
}

//
// This could fault if CR4 does not exist.  Non-existent CR4
// is functionally equivalent to CR4 == 0.  Keep it simple and pretend
// that CR4 == 0 on CPUs that don't have CR4.
//

// CR4 always exists on x86_64.
extern "C" {
    pub fn volatile(%%cr4: "mov, (val): %0" : "=r") -> asm;
}

extern "C" {
    pub fn native_write_cr4(val: c_ulong);
}

//
// "rdpkru" instruction.  Places PKRU contents in to EAX,
// clears EDX and requires that ecx=0.
//
extern "C" {
    pub fn volatile((pkru): "rdpkru" : "=a", (ecx): "=d" (edx) : "c") -> asm;
}
//
// "wrpkru" instruction.  Loads contents in EAX to PKRU,
// requires that ecx = edx = 0.
//
extern "C" {
    pub fn volatile((pkru): "wrpkru" : : "a", _arg: "c"(ecx), _arg: "d"(edx)) -> asm;
}

//
// Write back all modified lines in all levels of cache associated with this
// logical processor to main memory, and then invalidate all caches.  Depending
// on the micro-architecture, WBINVD (and WBNOINVD below) may or may not affect
// lower level caches associated with another logical processor that shares any
// level of this processor's cache hierarchy.
//
extern "C" {
    pub fn volatile("memory": "wbinvd" : : :) -> asm;
}
// Instruction encoding provided for binutils backwards compatibility.

//
// Write back all modified lines in all levels of cache associated with this
// logical processor to main memory, but do NOT explicitly invalidate caches,
// i.e. leave all/most cache lines in the hierarchy in non-modified state.
//
// Explicitly encode WBINVD if X86_FEATURE_WBNOINVD is unavailable even
// though WBNOINVD is backwards compatible (it's simply WBINVD with an
// ignored REP prefix), to guarantee that WBNOINVD isn't used if it
// needs to be avoided for any reason.  For all supported usage in the
// kernel, WBINVD is functionally a superset of WBNOINVD.
//
extern "C" {
    pub fn native_read_cr4() -> return;
}

extern "C" {
    pub fn native_read_cr0() -> return;
}
extern "C" {
    pub fn native_read_cr2() -> return;
}
//
// Careful!  CR3 contains more than just an address.  You probably want
// read_cr3_pa() instead.
//
extern "C" {
    pub fn __native_read_cr3() -> return;
}

extern "C" {
    pub fn volatile()__p): *mut *mut "clflush %0" : "+m" ((volatile char ) -> asm;
}

// Instruction opcode for SERIALIZE; supported in binutils >= 2.35.
extern "C" {
    pub fn volatile(0xf: ".byte, _arg: 0x1, "memory": 0xe8" :::) -> asm;
}
// The dst parameter must be 64-bytes aligned
//
// MOVDIR64B %(rdx), rax.
//
// Both __src and __dst must be memory constraints in order to tell the
// compiler that no other memory accesses should be reordered around
// this one.
//
// Also, both must be supplied as lvalues because this tells
// the compiler what the object is (its size) the instruction accesses.
// I.e., not the pointers but what they point to, thus the deref'ing '*'.
//
// enqcmds - Enqueue a command in supervisor (CPL0) mode
// @dst: destination, in MMIO space (must be 512-bit aligned)
// @src: 512 bits memory operand
//
// The ENQCMDS instruction allows software to write a 512-bit command to
// a 512-bit-aligned special MMIO region that supports the instruction.
// A return status is loaded into the ZF flag in the RFLAGS register.
// ZF = 0 equates to success, and ZF = 1 indicates retry or error.
//
// This function issues the ENQCMDS instruction to submit data from
// kernel space to MMIO space, in a unit of 512 bits. Order of data access
// is not guaranteed, nor is a memory barrier performed afterwards. It
// returns 0 on success and -EAGAIN on failure.
//
// Warning: Do not use this helper unless your driver has checked that the
// ENQCMDS instruction is supported on the platform and the device accepts
// ENQCMDS.
//
// ENQCMDS %(rdx), rax
//
// See movdir64b()'s comment on operand specification.
//
// Submission failure is indicated via EFLAGS.ZF=1
//
// Instruction opcode for TILERELEASE; supported in binutils
// version >= 2.36.
//
extern "C" {
    pub fn volatile(0xc4: ".byte, _arg: 0xe2, _arg: 0x78, _arg: 0x49, _arg: 0xc0") -> asm;
}

