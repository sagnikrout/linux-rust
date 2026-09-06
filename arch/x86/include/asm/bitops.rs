//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/bitops.h
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
// Copyright 1992, Linus Torvalds.
//
// Note: inlines with more than a single statement should be marked
// __always_inline to avoid problems with older gcc's inlining heuristics.
//

//
// These have to be done with inline assembly: that way the bit-setting
// is guaranteed to be atomic. All bit operations return 0 if the bit
// was cleared before the operation and != 0 if it was not.
//
// bit 0 is the LSB of addr; bit 32 is the LSB of (addr+1).
//

//
// We do the locked ops that don't return the old value as
// a mask operation on a byte.
//

extern "C" {
    pub fn volatile(%1: __ASM_SIZE(bts) ", ADDR: %0" : :, "memory": "Ir" (nr) :) -> asm;
}
extern "C" {
    pub fn volatile(%1: __ASM_SIZE(btr) ", ADDR: %0" : :, "memory": "Ir" (nr) :) -> asm;
}

extern "C" {
    pub fn volatile(%1: __ASM_SIZE(btc) ", ADDR: %0" : :, "memory": "Ir" (nr) :) -> asm;
}
extern "C" {
    pub fn GEN_BINARY_RMWcc(__ASM_SIZE(bts): LOCK_PREFIX, _arg: *mut addr, _arg: c, _arg: "Ir", _arg: nr) -> return;
}
extern "C" {
    pub fn arch_test_and_set_bit(_arg: nr, _arg: addr) -> return;
}
extern "C" {
    pub fn GEN_BINARY_RMWcc(__ASM_SIZE(btr): LOCK_PREFIX, _arg: *mut addr, _arg: c, _arg: "Ir", _arg: nr) -> return;
}
//
// Note: the operation is performed atomically with respect to
// the local CPU, but not other CPUs. Portable code should not
// rely on this behaviour.
// KVM relies on this behaviour on x86 for modifying memory that is also
// accessed from a hypervisor on the same CPU if running in a VM: don't change
// this without also updating arch/x86/kernel/kvm.c
//
extern "C" {
    pub fn GEN_BINARY_RMWcc(__ASM_SIZE(btc): LOCK_PREFIX, _arg: *mut addr, _arg: c, _arg: "Ir", _arg: nr) -> return;
}
//
// __ffs - find first set bit in word
// @word: The word to search
//
// Undefined if no bit exists, so code should check against 0 first.
//

extern "C" {
    pub fn variable__ffs(_arg: ~word) -> return;
}
//
// ffz - find first zero bit in word
// @word: The word to search
//
// Undefined if no zero exists, so code should check against ~0UL first.
//

//
// __fls: find last set bit in word
// @word: The word to search
//
// Undefined if no set bit exists, so code should check against 0 first.
//

//
// AMD64 says BSFL won't clobber the dest reg if x==0; Intel64 says the
// dest reg is undefined if x==0, but their CPU architect says its
// value is written to set it to the same as before, except that the
// top 32 bits will be cleared.
//
// We cannot do this on 32 bits because at the very least some
// 486 CPUs did not behave this way.
//

//
// ffs - find first set bit in word
// @x: the word to search
//
// This is defined the same way as the libc and compiler builtin ffs
// routines, therefore differs in spirit from the other bitops.
//
// ffs(value) returns 0 if value is 0 or the position of the first
// set bit if value is nonzero. The first (least significant) bit
// is at position 1.
//

//
// fls - find last set bit in word
// @x: the word to search
//
// This is defined in a similar way as the libc and compiler builtin
// ffs, but returns the position of the most significant set bit.
//
// fls(value) returns 0 if value is 0 or the position of the last
// set bit if value is nonzero. The last (most significant) bit is
// at position 32.
//

//
// AMD64 says BSRL won't clobber the dest reg if x==0; Intel64 says the
// dest reg is undefined if x==0, but their CPU architect says its
// value is written to set it to the same as before, except that the
// top 32 bits will be cleared.
//
// We cannot do this on 32 bits because at the very least some
// 486 CPUs did not behave this way.
//

//
// fls64 - find last set bit in a 64-bit word
// @x: the word to search
//
// This is defined in a similar way as the libc and compiler builtin
// ffsll, but returns the position of the most significant set bit.
//
// fls64(value) returns 0 if value is 0 or the position of the last
// set bit if value is nonzero. The last (most significant) bit is
// at position 64.
//

//
// AMD64 says BSRQ won't clobber the dest reg if x==0; Intel64 says the
// dest reg is undefined if x==0, but their CPU architect says its
// value is written to set it to the same as before.
//

