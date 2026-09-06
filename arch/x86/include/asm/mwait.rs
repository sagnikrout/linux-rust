//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mwait.h
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

pub const MWAIT_SUBSTATE_MASK: c_uint = 0xf;
pub const MWAIT_CSTATE_MASK: c_uint = 0xf;
pub const MWAIT_SUBSTATE_SIZE: c_int = 4;

pub const MWAIT_C1_SUBSTATE_MASK: c_uint = 0xf0;
pub const CPUID5_ECX_EXTENSIONS_SUPPORTED: c_uint = 0x1;
pub const CPUID5_ECX_INTERRUPT_BREAK: c_uint = 0x2;
pub const MWAIT_ECX_INTERRUPT_BREAK: c_uint = 0x1;

pub const MWAITX_DISABLE_CSTATES: c_uint = 0xf0;
pub const TPAUSE_C01_STATE: c_int = 1;
pub const TPAUSE_C02_STATE: c_int = 0;
//
// Use the instruction mnemonic with implicit operands, as the LLVM
// assembler fails to assemble the mnemonic with explicit operands:
//
extern "C" {
    pub fn volatile((eax): "monitor" :: "a", (ecx): "c", (edx): "d") -> asm;
}
extern "C" {
    pub fn volatile((eax): "monitorx" :: "a", (ecx): "c", _arg: "d"(edx)) -> asm;
}
//
// Use the instruction mnemonic with implicit operands, as the LLVM
// assembler fails to assemble the mnemonic with explicit operands:
//
extern "C" {
    pub fn volatile((eax): "mwait" :: "a", (ecx): "c") -> asm;
}
//
// MWAITX allows for a timer expiration to get the core out a wait state in
// addition to the default MWAIT exit condition of a store appearing at a
// monitored virtual address.
//
// Registers:
//
// MWAITX ECX[1]: enable timer if set
// MWAITX EBX[31:0]: max wait time expressed in SW P0 clocks. The software P0
// frequency is the same as the TSC frequency.
//
// Below is a comparison between MWAIT and MWAITX on AMD processors:
//
// MWAIT                           MWAITX
// opcode          0f 01 c9           |            0f 01 fb
// ECX[0]                  value of RFLAGS.IF seen by instruction
// ECX[1]          unused/#GP if set  |            enable timer if set
// ECX[31:2]                     unused/#GP if set
// EAX                           unused (reserve for hint)
// EBX[31:0]       unused             |            max wait time (P0 clocks)
//
// MONITOR                         MONITORX
// opcode          0f 01 c8           |            0f 01 fa
// EAX                     (logical) address to monitor
// ECX                     #GP if not zero
//
// No need for TSA buffer clearing on AMD
extern "C" {
    pub fn volatile((eax): "mwaitx" :: "a", (ebx): "b", (ecx): "c") -> asm;
}
//
// Re-enable interrupts right upon calling mwait in such a way that
// no interrupt can fire _before_ the execution of mwait, ie: no
// instruction must be placed between "sti" and "mwait".
//
// This is necessary because if an interrupt queues a timer before
// executing mwait, it would otherwise go unnoticed and the next tick
// would not be reprogrammed accordingly before mwait ever wakes up.
//
extern "C" {
    pub fn volatile((eax): "sti; mwait" :: "a", (ecx): "c") -> asm;
}
//
// This uses new MONITOR/MWAIT instructions on P4 processors with PNI,
// which can obviate IPI to trigger checking of need_resched.
// We execute MONITOR against need_resched and enter optimized wait state
// through MWAIT. Whenever someone changes need_resched, we would be woken
// up from MWAIT (without an IPI).
//
// New with Core Duo processors, MWAIT can take some hints based on CPU
// capability.
//
// Caller can specify whether to enter C0.1 (low latency, less
// power saving) or C0.2 state (saves more power, but longer wakeup
// latency). This may be overridden by the IA32_UMWAIT_CONTROL MSR
// which can force requests for C0.2 to be downgraded to C0.1.
//
// "tpause %ecx"
