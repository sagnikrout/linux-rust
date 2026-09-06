//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/irq_stack.h
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
// Macro to inline switching to an interrupt stack and invoking function
// calls from there. The following rules apply:
//
// - Ordering:
//
// 1. Write the stack pointer into the top most place of the irq
// stack. This ensures that the various unwinders can link back to the
// original stack.
//
// 2. Switch the stack pointer to the top of the irq stack.
//
// 3. Invoke whatever needs to be done (@asm_call argument)
//
// 4. Pop the original stack pointer from the top of the irq stack
// which brings it back to the original stack where it left off.
//
// - Function invocation:
//
// To allow flexible usage of the macro, the actual function code including
// the store of the arguments in the call ABI registers is handed in via
// the @asm_call argument.
//
// - Local variables:
//
// @tos:
// The @tos variable holds a pointer to the top of the irq stack and
// _must_ be allocated in a non-callee saved register as this is a
// restriction coming from objtool.
//
// Note, that (tos) is both in input and output constraints to ensure
// that the compiler does not assume that R11 is left untouched in
// case this macro is used in some place where the per cpu interrupt
// stack pointer is used again afterwards
//
// - Function arguments:
// The function argument(s), if any, have to be defined in register
// variables at the place where this is invoked. Storing the
// argument(s) in the proper register(s) is part of the @asm_call
//
// - Constraints:
//
// The constraints have to be done very carefully because the compiler
// does not know about the assembly call.
//
// output:
// As documented already above the @tos variable is required to be in
// the output constraints to make the compiler aware that R11 cannot be
// reused after the asm() statement.
//
// For builds with CONFIG_UNWINDER_FRAME_POINTER, ASM_CALL_CONSTRAINT is
// required as well as this prevents certain creative GCC variants from
// misplacing the ASM code.
//
// input:
// - func:
// Immediate, which tells the compiler that the function is referenced.
//
// - tos:
// Register. The actual register is defined by the variable declaration.
//
// - function arguments:
// The constraints are handed in via the 'argconstr' argument list. They
// describe the register arguments which are used in @asm_call.
//
// clobbers:
// Function calls can clobber anything except the callee-saved
// registers. Tell the compiler.
//

// Macros to assert type correctness for run_*_on_irqstack macros

//
// Macro to invoke system vector and device interrupt C handlers.
//

// \
// User mode entry and interrupt on the irq stack do not	\
// switch stacks. If from user mode the task stack is empty.	\
// \
// Mark the irq stack inuse _before_ and unmark _after_	\
// switching stacks. Interrupts are disabled in both	\
// places. Invoke the stack switch macro with the call	\
// sequence which matches the above direct invocation.	\
// \
//
// Function call sequence for __call_on_irqstack() for system vectors.
//
// Note that irq_enter_rcu() and irq_exit_rcu() do not use the input
// mechanism because these functions are global and cannot be optimized out
// when compiling a particular source file which uses one of these macros.
//
// The argument (regs) does not need to be pushed or stashed in a callee
// saved register to be safe vs. the irq_enter_rcu() call because the
// clobbers already prevent the compiler from storing it in a callee
// clobbered register. As the compiler has to preserve @regs for the final
// call to idtentry_exit() anyway, it's likely that it does not cause extra
// effort for this asm magic.
//

//
// As in ASM_CALL_SYSVEC above the clobbers force the compiler to store
// @regs and @vector in callee saved registers.
//

//
// Macro to invoke __do_softirq on the irq stack. This is only called from
// task context when bottom halves are about to be reenabled and soft
// interrupts are pending to be processed. The interrupt stack cannot be in
// use here.
//

// System vector handlers always run on the stack they interrupted.

// Switches to the irq stack within func()

