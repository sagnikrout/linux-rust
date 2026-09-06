//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/ptrace.h
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
pub struct pt_regs {
//
// NB: 32-bit x86 CPUs are inconsistent as what happens in the
// following cases (where %seg represents a segment register):
//
// - pushl %seg: some do a 16-bit write and leave the high
// bits alone
// - movl %seg, [mem]: some do a 16-bit write despite the movl
// - IDT entry: some (e.g. 486) will leave the high bits of CS
// and (if applicable) SS undefined.
//
// Fortunately, x86-32 doesn't read the high bits on POP or IRET,
// so we can just treat all of the segment registers as 16-bit
// values.
//
    pub bx: c_ulong,
    pub cx: c_ulong,
    pub dx: c_ulong,
    pub si: c_ulong,
    pub di: c_ulong,
    pub bp: c_ulong,
    pub ax: c_ulong,
    pub ds: c_ushort,
    pub __dsh: c_ushort,
    pub es: c_ushort,
    pub __esh: c_ushort,
    pub fs: c_ushort,
    pub __fsh: c_ushort,
//
// On interrupt, gs and __gsh store the vector number.  They never
// store gs any more.
//
    pub gs: c_ushort,
    pub __gsh: c_ushort,
// On interrupt, this is the error code.
    pub orig_ax: c_ulong,
    pub ip: c_ulong,
    pub cs: c_ushort,
    pub __csh: c_ushort,
    pub flags: c_ulong,
    pub sp: c_ulong,
    pub ss: c_ushort,
    pub __ssh: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fred_cs {
// CS selector
// Stack level at event time
// IBT in WAIT_FOR_ENDBRANCH state
    pub 45: :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fred_ss {
// SS selector
// STI state
// Set if syscall, sysenter or INT n
// Event is NMI type
// Event vector
// Event type
// Event was incident to enclave execution
// CPU was in 64-bit mode
//
// Nested exception during FRED delivery, not set
// for #DF.
//
// The length of the instruction causing the event.
// Only set for INTO, INT1, INT3, INT n, SYSCALL
// and SYSENTER.  0 otherwise.
//
    pub 4: insnlen :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
//
// C ABI says these regs are callee-preserved. They aren't saved on
// kernel entry unless syscall needs a complete, fully filled
// "struct pt_regs".
//
    pub r15: c_ulong,
    pub r14: c_ulong,
    pub r13: c_ulong,
    pub r12: c_ulong,
    pub bp: c_ulong,
    pub bx: c_ulong,
// These regs are callee-clobbered. Always saved on kernel entry.
    pub r11: c_ulong,
    pub r10: c_ulong,
    pub r9: c_ulong,
    pub r8: c_ulong,
    pub ax: c_ulong,
    pub cx: c_ulong,
    pub dx: c_ulong,
    pub si: c_ulong,
    pub di: c_ulong,
//
// orig_ax is used on entry for:
// - the syscall number (syscall, sysenter, int80)
// - error_code stored by the CPU on traps and exceptions
// - the interrupt number for device interrupts
//
// A FRED stack frame starts here:
// 1) It _always_ includes an error code;
//
// 2) The return frame for ERET[US] starts here, but
// the content of orig_ax is ignored.
//
    pub orig_ax: c_ulong,
// The IRETQ return frame starts here
    pub ip: c_ulong,
// CS selector
    pub cs: u16,
// The extended 64-bit data slot containing CS
    pub csx: u64,
// The FRED CS extension
    pub fred_cs: fred_cs,
}

// SS selector
// The extended 64-bit data slot containing SS
// The FRED SS extension
//
// Top of stack on IDT systems, while FRED systems have extra fields
// defined above for storing exception related information, e.g. CR2 or
// DR6.
//

extern "C" {
    pub fn profile_pc(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn send_sigtrap(regs: *mut pt_regs, error_code: c_int, si_code: c_int);
}
//
// user_mode(regs) determines whether a register set came from user
// mode.  On x86_32, this is true if V8086 mode was enabled OR if the
// register set was from protected mode with RPL-3 CS value.  This
// tricky test checks that with one comparison.
//
// On x86_64, vm86 mode is mercifully nonexistent, and we don't need
// the extra check.
//

//
// On non-paravirt systems, this is the only long mode CPL 3
// selector.  We do not allow long mode selectors in the LDT.
//

// Headers are too twisted for this to go in paravirt.h.

//
// Determine whether the register set came from any context that is running in
// 64-bit mode.
//

// Query offset/name of register from its name/offset
extern "C" {
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
}

//
// regs_get_register() - get register value from its offset
// @regs:	pt_regs from which register value is gotten.
// @offset:	offset number of the register.
//
// regs_get_register returns the value of a register. The @offset is the
// offset of the register in struct pt_regs address which specified by @regs.
// If @offset is bigger than MAX_REG_OFFSET, this returns 0.
//

// The selector fields are 16-bit.

//
// regs_within_kernel_stack() - check the address in the stack
// @regs:	pt_regs which contains kernel stack pointer.
// @addr:	address which is checked.
//
// regs_within_kernel_stack() checks @addr is within the kernel stack page(s).
// If @addr is within the kernel stack, it returns true. If not, returns false.
//
// regs_get_kernel_stack_nth_addr() - get the address of the Nth entry on stack
// @regs:	pt_regs which contains kernel stack pointer.
// @n:		stack entry number.
//
// regs_get_kernel_stack_nth() returns the address of the @n th entry of the
// kernel stack which is specified by @regs. If the @n th entry is NOT in
// the kernel stack, this returns NULL.
//
// To avoid include hell, we can't include uaccess.h
extern "C" {
    pub fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_long;
}
//
// regs_get_kernel_stack_nth() - get Nth entry of the stack
// @regs:	pt_regs which contains kernel stack pointer.
// @n:		stack entry number.
//
// regs_get_kernel_stack_nth() returns @n th entry of the kernel stack which
// is specified by @regs. If the @n th entry is NOT in the kernel stack
// this returns 0.
//
// regs_get_kernel_argument() - get Nth function argument in kernel
// @regs:	pt_regs of that context
// @n:		function argument number (start from 0)
//
// regs_get_argument() returns @n th argument of the function call.
// Note that this chooses most probably assignment, in some case
// it can be incorrect.
// This is expected to be called from kprobes or ftrace with regs
// where the top of stack is the return address.
//

pub const NR_REG_ARGUMENTS: c_int = 3;

pub const NR_REG_ARGUMENTS: c_int = 6;

extern "C" {
    pub fn regs_get_kernel_stack_nth(_arg: regs, _arg: n) -> return;
}
extern "C" {
    pub fn regs_get_register(_arg: regs, _arg: argument_offs[n]) -> return;
}

// Macro flag: #define ARCH_HAS_USER_SINGLE_STEP_REPORT

