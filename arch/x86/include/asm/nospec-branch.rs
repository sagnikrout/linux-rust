//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/nospec-branch.h
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
// Call depth tracking for Intel SKL CPUs to address the RSB underflow
// issue in software.
//
// The tracking does not use a counter. It uses uses arithmetic shift
// right on call entry and logical shift left on return.
//
// The depth tracking variable is initialized to 0x8000.... when the call
// depth is zero. The arithmetic shift right sign extends the MSB and
// saturates after the 12th call. The shift count is 5 for both directions
// so the tracking covers 12 nested calls.
//
// Call
// 0: 0x8000000000000000	0x0000000000000000
// 1: 0xfc00000000000000	0xf000000000000000
// ...
// 11: 0xfffffffffffffff8	0xfffffffffffffc00
// 12: 0xffffffffffffffff	0xffffffffffffffe0
//
// After a return buffer fill the depth is credited 12 calls before the
// next stuffing has to take place.
//
// There is a inaccuracy for situations like this:
//
// 10 calls
// 5 returns
// 3 calls
// 4 returns
// 3 calls
// ....
//
// The shift count might cause this to be off by one in either direction,
// but there is still a cushion vs. the RSB depth. The algorithm does not
// claim to be perfect and it can be speculated around by the CPU, but it
// is considered that it obfuscates the problem enough to make exploitation
// extremely difficult.
//
pub const RET_DEPTH_SHIFT: c_int = 5;
pub const RSB_RET_STUFF_LOOPS: c_int = 16;
pub const RET_DEPTH_INIT: c_uint = 0x8000000000000000ULL;
pub const RET_DEPTH_INIT_FROM_CALL: c_uint = 0xfc00000000000000ULL;
pub const RET_DEPTH_CREDIT: c_uint = 0xffffffffffffffffULL;

extern "C" {
    pub fn PER_CPU_VAR(_arg: __x86_call_count) -> incq;
}

extern "C" {
    pub fn PER_CPU_VAR(_arg: __x86_ret_count) -> incq;
}

extern "C" {
    pub fn PER_CPU_VAR(_arg: __x86_stuffs_count) -> incq;
}

extern "C" {
    pub fn PER_CPU_VAR(_arg: __x86_ctxsw_count) -> incq;
}

// Macro flag: #define CREDIT_CALL_DEPTH
// Macro flag: #define RESET_CALL_DEPTH
// Macro flag: #define RESET_CALL_DEPTH_FROM_CALL
// Macro flag: #define INCREMENT_CALL_DEPTH

//
// Fill the CPU return stack buffer.
//
// Each entry in the RSB, if used for a speculative 'ret', contains an
// infinite 'pause; lfence; jmp' loop to capture speculative execution.
//
// This is required in various cases for retpoline and IBRS-based
// mitigations for the Spectre variant 2 vulnerability. Sometimes to
// eliminate potentially bogus entries from the RSB, and sometimes
// purely to ensure that it doesn't get empty, which on some CPUs would
// allow predictions from other (unwanted!) sources to be used.
//
// We define a CPP macro such that it can be used from both .S files and
// inline assembly. It's possible to do a .macro and then include that
// from C via asm(".include <asm/nospec-branch.h>") but let's not go there.
//
pub const RETPOLINE_THUNK_SIZE: c_int = 32;

//
// Common helper for __FILL_RETURN_BUFFER and __FILL_ONE_RETURN.
//

//
// Stuff the entire RSB.
//
// Google experimented with loop-unrolling and this turned out to be
// the optimal version - two calls, each with their own speculation
// trap should their return address end up getting used, in a loop.
//

// barrier for jnz misprediction */		\

//
// i386 doesn't unconditionally have LFENCE, as such it can't
// do a loop.
//

//
// Stuff a single RSB slot.
//
// To mitigate Post-Barrier RSB speculation, one CALL instruction must be
// forced to retire before letting a RET instruction execute.
//
// On PBRSB-vulnerable CPUs, it is not safe for a RET to be executed
// before this point.
//

//
// Helper for detecting if an interrupt occurred at an unsafe location within
// Safe-RET.  If Safe-RET is interrupted after the CALL or LEA the RSB may get
// poisoned by the interrupt handler.
//
// The Safe-RET sequence is:
//
// CALL
// LEA 8(%RSP), %RSP
// RET
//
// The two CMPs below check whether RIP points to after the CALL or after the
// LEA.
//
// The LFENCE below is to address this particular speculation case:
//
// 1. Userspace runs and poisons the BTB around the safe-RET routine
//
// 2. Userspace triggers some kind of exception
//
// 3. Kernel executes error_entry() and mis-speculates the branch into thinking
// it actually came from kernel space
//
// 4. The kernel then further mis-speculates that the exception occurred due
// to an interrupted safe-RET
//
// 5. The handle_interrupted_saferet() routine speculatively executes and
// speculatively does a safe-RET. But this is unsafe since it was never
// untrained.
//
// The LFENCE fixes this by ensuring step 5 is never reached speculatively.
// Note that this LFENCE only occurs if safe-RET was actually interrupted (so
// it's outside of the normal path).
//

//
// (ab)use RETPOLINE_SAFE on RET to annotate away 'bare' RET instructions
// vs RETBleed validation.
//

//
// Abuse ANNOTATE_RETPOLINE_SAFE on a NOP to indicate UNRET_END, should
// eventually turn into its own annotation.
//

//
// Emits a conditional CS prefix that is compatible with
// -mindirect-branch-cs-prefix.
//
// JMP_NOSPEC and CALL_NOSPEC macros can be used instead of a simple
// indirect jmp/call which may be susceptible to the Spectre variant 2
// attack.
//
// NOTE: these do not take kCFI into account and are thus not comparable to C
// indirect calls, take care when using. The target of these should be an ENDBR
// instruction irrespective of kCFI.
//

//
// A simpler FILL_RETURN_BUFFER macro. Don't make people use the CPP
// monstrosity above, manually.
//
// The CALL to srso_alias_untrain_ret() must be patched in directly at
// the spot where untraining must be done, ie., srso_alias_untrain_ret()
// must be the target of a CALL instruction instead of indirectly
// jumping to a wrapper which then calls it. Therefore, this macro is
// called outside of __UNTRAIN_RET below, for the time being, before the
// kernel can support nested alternatives with arbitrary nesting.
//

//
// Mitigate RETBleed for AMD/Hygon Zen uarch. Requires KERNEL CR3 because the
// return thunk isn't mapped into the userspace tables (then again, AMD
// typically has NO_MELTDOWN).
//
// While retbleed_untrain_ret() doesn't clobber anything but requires stack,
// write_ibpb() will clobber AX, CX, DX.
//
// As such, this must be placed after every *SWITCH_TO_KERNEL_CR3 at a point
// where we have a stack but before any RET instruction.
//

//
// Macro to execute VERW insns that mitigate transient data sampling
// attacks such as MDS or TSA. On affected systems a microcode update
// overloaded VERW insns to also clear the CPU buffers. VERW clobbers
// CFLAGS.ZF.
// Note: Only the memory operand variant of VERW clears the CPU buffers.
//

//
// In 32bit mode, the memory operand must be a %cs reference. The data segments
// may not be usable (vm86 mode), and the stack segment may not be flat (ESPFIX32).
//

//
// Provide a stringified VERW macro for simple usage, and a non-stringified
// VERW macro for use in more elaborate sequences, e.g. to encode a conditional
// VERW within an ALTERNATIVE.
//

// If necessary, emit VERW on exit-to-userspace to clear CPU buffers.

// Macro flag: #define CLEAR_BRANCH_HISTORY
// Macro flag: #define CLEAR_BRANCH_HISTORY_VMEXIT

pub const ITS_THUNK_SIZE: c_int = 64;

extern "C" {
    pub fn __x86_return_thunk();
}

extern "C" {
    pub fn retbleed_return_thunk();
}

extern "C" {
    pub fn srso_alias_untrain_ret();
}

extern "C" {
    pub fn srso_return_thunk();
}
extern "C" {
    pub fn srso_alias_return_thunk();
}

extern "C" {
    pub fn its_return_thunk();
}

extern "C" {
    pub fn retbleed_return_thunk();
}
extern "C" {
    pub fn srso_return_thunk();
}
extern "C" {
    pub fn srso_alias_return_thunk();
}
extern "C" {
    pub fn entry_untrain_ret();
}
extern "C" {
    pub fn write_ibpb();
}

extern "C" {
    pub fn bpf_arch_ibpb();
}

extern "C" {
    pub fn clear_bhb_loop();
}

extern "C" {
    pub fn void(_arg: *mut x86_return_thunk)(void) -> extern;
}
extern "C" {
    pub fn __warn_thunk();
}

extern "C" {
    pub fn call_depth_return_thunk();
}

//
// Emits a conditional CS prefix that is compatible with
// -mindirect-branch-cs-prefix.
//

//
// Inline asm uses the %V modifier which is only in newer GCC
// which is ensured when CONFIG_MITIGATION_RETPOLINE is defined.
//

//
// For i386 we use the original ret-equivalent retpoline, because
// otherwise we'll run out of registers. We don't care about CET
// here, anyway.
//

// The Spectre V2 mitigation variants
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spectre_v2_mitigation {
    SPECTRE_V2_NONE,
    SPECTRE_V2_RETPOLINE,
    SPECTRE_V2_LFENCE,
    SPECTRE_V2_EIBRS,
    SPECTRE_V2_EIBRS_RETPOLINE,
    SPECTRE_V2_EIBRS_LFENCE,
    SPECTRE_V2_IBRS,
}

// The indirect branch speculation control variants
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spectre_v2_user_mitigation {
    SPECTRE_V2_USER_NONE,
    SPECTRE_V2_USER_STRICT,
    SPECTRE_V2_USER_STRICT_PREFERRED,
    SPECTRE_V2_USER_PRCTL,
    SPECTRE_V2_USER_SECCOMP,
}

// The Speculative Store Bypass disable variants
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssb_mitigation {
    SPEC_STORE_BYPASS_NONE,
    SPEC_STORE_BYPASS_AUTO,
    SPEC_STORE_BYPASS_DISABLE,
    SPEC_STORE_BYPASS_PRCTL,
    SPEC_STORE_BYPASS_SECCOMP,
}

// The Intel SPEC CTRL MSR base value cache
extern "C" {
    pub fn update_spec_ctrl_cond(val: u64);
}
extern "C" {
    pub fn spec_ctrl_current() -> u64;
}
//
// With retpoline, we must use IBRS to restrict branch prediction
// before calling into firmware.
//
// (Implemented as CPP macros due to header hell.)
//

//
// x86_clear_cpu_buffers - Buffer clearing support for different x86 CPU vulns
//
// This uses the otherwise unused and obsolete VERW instruction in
// combination with microcode which triggers a CPU buffer flush when the
// instruction is executed.
//
// Has to be the memory-operand variant because only that
// guarantees the CPU buffer flush functionality according to
// documentation. The register-operand variant does not.
// Works with any segment selector, but a valid writable
// data segment is the fastest variant.
//
// "cc" clobber is required because VERW modifies ZF.
//
extern "C" {
    pub fn volatile("cc": "verw %[ds]" : : [ds] "m" (ds) :) -> asm;
}
//
// x86_idle_clear_cpu_buffers - Buffer clearing support in idle for the MDS
// and TSA vulnerabilities.
//
// Clear CPU buffers if the corresponding static key is enabled
//
extern "C" {
    pub fn srso_safe_ret();
}
extern "C" {
    pub fn srso_alias_safe_ret();
}
extern "C" {
    pub fn handle_interrupted_saferet(regs: *mut pt_regs);
}

