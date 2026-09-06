//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/syscall_wrapper.h
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
// syscall_wrapper.h - x86 specific wrappers to syscall definitions
//

extern "C" {
    pub fn __x64_sys_ni_syscall(regs: *const pt_regs) -> c_long;
}
extern "C" {
    pub fn __ia32_sys_ni_syscall(regs: *const pt_regs) -> c_long;
}
//
// Instead of the generic __SYSCALL_DEFINEx() definition, the x86 version takes
// struct pt_regs *regs as the only argument of the syscall stub(s) named as:
// __x64_sys_*()         - 64-bit native syscall
// __ia32_sys_*()        - 32-bit native syscall or common compat syscall
// __ia32_compat_sys_*() - 32-bit compat syscall
// __x64_compat_sys_*()  - 64-bit X32 compat syscall
//
// The registers are decoded according to the ABI:
// 64-bit: RDI, RSI, RDX, R10, R8, R9
// 32-bit: EBX, ECX, EDX, ESI, EDI, EBP
//
// The stub then passes the decoded arguments to the __se_sys_*() wrapper to
// perform sign-extension (omitted for zero-argument syscalls).  Finally the
// arguments are passed to the __do_sys_*() function which is the actual
// syscall.  These wrappers are marked as inline so the compiler can optimize
// the functions where appropriate.
//
// Example assembly (slightly re-ordered for better readability):
//
// <__x64_sys_recv>:		<-- syscall with 4 parameters
// callq	<__fentry__>
//
// mov	0x70(%rdi),%rdi	<-- decode regs->di
// mov	0x68(%rdi),%rsi	<-- decode regs->si
// mov	0x60(%rdi),%rdx	<-- decode regs->dx
// mov	0x38(%rdi),%rcx	<-- decode regs->r10
//
// xor	%r9d,%r9d	<-- clear %r9
// xor	%r8d,%r8d	<-- clear %r8
//
// callq	__sys_recvfrom	<-- do the actual work in __sys_recvfrom()
// which takes 6 arguments
//
// cltq			<-- extend return value to 64-bit
// retq			<-- return
//
// This approach avoids leaking random user-provided register content down
// the call chain.
//
// Mapping of registers to parameters for syscalls on x86-64 and x32

// SYSCALL_PT_ARGS is Adapted from s390x

// Mapping of registers to parameters for syscalls on i386

// Macro flag: #define __X64_SYS_STUB0(name)

// Macro flag: #define __X64_COND_SYSCALL(name)

// Macro flag: #define __IA32_SYS_STUB0(name)

// Macro flag: #define __IA32_COND_SYSCALL(name)

//
// For IA32 emulation, we need to handle "compat" syscalls *and* create
// additional wrappers (aptly named __ia32_sys_xyzzy) which decode the
// ia32 regs in the proper order for shared or "common" syscalls. As some
// syscalls may not be implemented, we need to expand COND_SYSCALL in
// kernel/sys_ni.c to cover this case as well.
//

// Macro flag: #define __IA32_COMPAT_SYS_STUB0(name)

// Macro flag: #define __IA32_COMPAT_COND_SYSCALL(name)

//
// For the x32 ABI, we need to create a stub for compat_sys_*() which is aware
// of the x86-64-style parameter ordering of x32 syscalls. The syscalls common
// with x86_64 obviously do not need such care.
//

// Macro flag: #define __X32_COMPAT_SYS_STUB0(name)

// Macro flag: #define __X32_COMPAT_COND_SYSCALL(name)

//
// Compat means IA32_EMULATION and/or X86_X32. As they use a different
// mapping of registers to parameters, we need to generate stubs for each
// of them.
//

//
// As some compat syscalls may not be implemented, we need to expand
// COND_SYSCALL_COMPAT in kernel/sys_ni.c to cover this case as well.
//

//
// As the generic SYSCALL_DEFINE0() macro does not decode any parameters for
// obvious reasons, and passing struct pt_regs *regs to it in %rdi does not
// hurt, we only need to re-define it here to keep the naming congruent to
// SYSCALL_DEFINEx() -- which is essential for the COND_SYSCALL() macro
// to work correctly.
//

//
// For VSYSCALLS, we need to declare these three syscalls with the new
// pt_regs-based calling convention for in-kernel use.
//
extern "C" {
    pub fn __x64_sys_getcpu(regs: *const pt_regs) -> c_long;
}
extern "C" {
    pub fn __x64_sys_gettimeofday(regs: *const pt_regs) -> c_long;
}
extern "C" {
    pub fn __x64_sys_time(regs: *const pt_regs) -> c_long;
}
