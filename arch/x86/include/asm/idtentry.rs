//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/idtentry.h
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
// Interrupts/Exceptions

extern "C" {
    pub fn void(regs: *mut *mut idtentry_t)(struct pt_regs) -> typedef;
}
//
// DECLARE_IDTENTRY - Declare functions for simple IDT entry points
// No error code pushed by hardware
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Declares four functions:
// - The ASM entry point: asm_##func
// - The XEN PV trap entry point: xen_##func (maybe unused)
// - The C handler called from the FRED event dispatcher (maybe unused)
// - The C handler called from the ASM entry point
//
// Note: This is the C variant of DECLARE_IDTENTRY(). As the name says it
// declares the entry points for usage in C code. There is an ASM variant
// as well which is used to emit the entry stubs in entry_32/64.S.
//

//
// DEFINE_IDTENTRY - Emit code for simple IDT entry points
// @func:	Function name of the entry point
//
// @func is called from ASM entry code with interrupts disabled.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//
// irqentry_enter() contains common code which has to be invoked before
// arbitrary code in the body. irqentry_exit() contains common code
// which has to run before returning to the low level assembly code.
//

// Special case for 32bit IRET 'trap'

//
// DECLARE_IDTENTRY_ERRORCODE - Declare functions for simple IDT entry points
// Error code pushed by hardware
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Declares three functions:
// - The ASM entry point: asm_##func
// - The XEN PV trap entry point: xen_##func (maybe unused)
// - The C handler called from the ASM entry point
//
// Same as DECLARE_IDTENTRY, but has an extra error_code argument for the
// C-handler.
//

//
// DEFINE_IDTENTRY_ERRORCODE - Emit code for simple IDT entry points
// Error code pushed by hardware
// @func:	Function name of the entry point
//
// Same as DEFINE_IDTENTRY, but has an extra error_code argument
//

//
// DECLARE_IDTENTRY_RAW - Declare functions for raw IDT entry points
// No error code pushed by hardware
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Maps to DECLARE_IDTENTRY().
//

//
// DEFINE_IDTENTRY_RAW - Emit code for raw IDT entry points
// @func:	Function name of the entry point
//
// @func is called from ASM entry code with interrupts disabled.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//
// Contrary to DEFINE_IDTENTRY() this does not invoke the
// idtentry_enter/exit() helpers before and after the body invocation. This
// needs to be done in the body itself if applicable. Use if extra work
// is required before the enter/exit() helpers are invoked.
//

//
// DEFINE_FREDENTRY_RAW - Emit code for raw FRED entry points
// @func:	Function name of the entry point
//
// @func is called from the FRED event dispatcher with interrupts disabled.
//
// See @DEFINE_IDTENTRY_RAW for further details.
//

//
// DECLARE_IDTENTRY_RAW_ERRORCODE - Declare functions for raw IDT entry points
// Error code pushed by hardware
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Maps to DECLARE_IDTENTRY_ERRORCODE()
//

//
// DEFINE_IDTENTRY_RAW_ERRORCODE - Emit code for raw IDT entry points
// @func:	Function name of the entry point
//
// @func is called from ASM entry code with interrupts disabled.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//
// Contrary to DEFINE_IDTENTRY_ERRORCODE() this does not invoke the
// irqentry_enter/exit() helpers before and after the body invocation. This
// needs to be done in the body itself if applicable. Use if extra work
// is required before the enter/exit() helpers are invoked.
//

//
// DECLARE_IDTENTRY_IRQ - Declare functions for device interrupt IDT entry
// points (common/spurious)
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Maps to DECLARE_IDTENTRY_ERRORCODE()
//

//
// DEFINE_IDTENTRY_IRQ - Emit code for device interrupt IDT entry points
// @func:	Function name of the entry point
//
// The vector number is pushed by the low level entry stub and handed
// to the function as error_code argument which needs to be truncated
// to an u8 because the push is sign extending.
//
// irq_enter/exit_rcu() are invoked before the function body and the
// KVM L1D flush request is set. Stack switching to the interrupt stack
// has to be done in the function body if necessary.
//

//
// DECLARE_IDTENTRY_SYSVEC - Declare functions for system vector entry points
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Declares three functions:
// - The ASM entry point: asm_##func
// - The XEN PV trap entry point: xen_##func (maybe unused)
// - The C handler called from the ASM entry point
//
// Maps to DECLARE_IDTENTRY().
//

//
// DEFINE_IDTENTRY_SYSVEC - Emit code for system vector IDT entry points
// @func:	Function name of the entry point
//
// irqentry_enter/exit() and irq_enter/exit_rcu() are invoked before the
// function body. KVM L1D flush request is set.
//
// Runs the function on the interrupt stack if the entry hit kernel mode
//

//
// DEFINE_IDTENTRY_SYSVEC_SIMPLE - Emit code for simple system vector IDT
// entry points
// @func:	Function name of the entry point
//
// Runs the function on the interrupted stack. No switch to IRQ stack and
// only the minimal __irq_enter/exit() handling.
//
// Only use for 'empty' vectors like reschedule IPI and KVM posted
// interrupt vectors.
//

//
// DECLARE_IDTENTRY_XENCB - Declare functions for XEN HV callback entry point
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Declares three functions:
// - The ASM entry point: asm_##func
// - The XEN PV trap entry point: xen_##func (maybe unused)
// - The C handler called from the ASM entry point
//
// Maps to DECLARE_IDTENTRY(). Distinct entry point to handle the 32/64-bit
// difference
//

//
// DECLARE_IDTENTRY_IST - Declare functions for IST handling IDT entry points
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Maps to DECLARE_IDTENTRY_RAW, but declares also the NOIST C handler
// which is called from the ASM entry point on user mode entry
//

//
// DECLARE_IDTENTRY_VC - Declare a function for the VC entry point
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Maps to DECLARE_IDTENTRY_RAW_ERRORCODE.
//

//
// DEFINE_IDTENTRY_IST - Emit code for IST entry points
// @func:	Function name of the entry point
//
// Maps to DEFINE_IDTENTRY_RAW
//

//
// DEFINE_IDTENTRY_NOIST - Emit code for NOIST entry points which
// belong to a IST entry point (MCE, DB)
// @func:	Function name of the entry point. Must be the same as
// the function name of the corresponding IST variant
//
// Maps to DEFINE_IDTENTRY_RAW().
//

//
// DECLARE_IDTENTRY_DF - Declare functions for double fault
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Maps to DECLARE_IDTENTRY_RAW_ERRORCODE
//

//
// DEFINE_IDTENTRY_DF - Emit code for double fault
// @func:	Function name of the entry point
//
// Maps to DEFINE_IDTENTRY_RAW_ERRORCODE
//

//
// DECLARE_IDTENTRY_DF - Declare functions for double fault 32bit variant
// @vector:	Vector number (ignored for C)
// @func:	Function name of the entry point
//
// Declares two functions:
// - The ASM entry point: asm_##func
// - The C handler called from the C shim
//

//
// DEFINE_IDTENTRY_DF - Emit code for double fault on 32bit
// @func:	Function name of the entry point
//
// This is called through the doublefault shim which already provides
// cr2 in the address argument.
//

// C-Code mapping

extern "C" {
    pub fn idt_install_sysvec(n: c_uint, function: *const c_void);
}
extern "C" {
    pub fn fred_install_sysvec(vector: c_uint, function: idtentry_t);
}

//
// The ASM variants for DECLARE_IDTENTRY*() which emit the ASM entry stubs.
//

// Special case for 32bit IRET 'trap'. Do not emit ASM code

// Entries for common/spurious (device) interrupts

// System vector entries

// No ASM emitted for DF as this goes through a C shim

// No ASM emitted for XEN hypervisor callback

// No ASM code emitted for NMI

//
// ASM code to emit the common vector entry stubs where each stub is
// packed into IDT_ALIGN bytes.
//
// Note, that the 'pushq imm8' is emitted via '.byte 0x6a, vector' because
// GCC treats the local vector variable as unsigned int and would expand
// all vectors above 0x7F to a 5 byte push. The original code did an
// adjustment of the vector number to be in the signed byte range to avoid
// this. While clever it's mindboggling counterintuitive and requires the
// odd conversion back to a real vector number in the C entry points. Using
// .byte achieves the same thing and the only fixup needed in the C entry
// point is to mask off the bits above bit 7 because the push is sign
// extending.
//
// Ensure that the above is IDT_ALIGN bytes max

// Ensure that the above is IDT_ALIGN bytes max

//
// The actual entry points. Note that DECLARE_IDTENTRY*() serves two
// purposes:
// - provide the function declarations when included from C-Code
// - emit the ASM stubs when included from entry_32/64.S
//
// This avoids duplicate defines and ensures that everything is consistent.
//
// Dummy trap number so the low level ASM macro vector number checks do not
// match which results in emitting plain IDTENTRY stubs without bells and
// whistles.
//
pub const X86_TRAP_OTHER: c_uint = 0xFFFF;
// Simple exception entry points. No hardware error code
// 32bit software IRET trap. Do not emit ASM code
// Simple exception entries with error code pushed by hardware
// Raw exception entries which need extra work

// NMI

//
// Special entry point for VMX which invokes this on the kernel stack, even for
// 64-bit, i.e. without using an IST.  asm_exc_nmi() requires an IST to work
// correctly vs. the NMI 'executing' marker.  Used for 32-bit kernels as well
// to avoid more ifdeffery.
//

// #DB

// #DF

// #CP

// #VC

// Device interrupts common/spurious

// System vector entry points

