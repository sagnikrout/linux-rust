//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compiler.h
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
// Note: DISABLE_BRANCH_PROFILING can be used by special lowlevel code
// to disable branch tracing on a per file basis.
//

//
// Using __builtin_constant_p(x) to ignore cases where the return
// value is always the same.  This idea is taken from a similar patch
// written by Daniel Walker.
//

//
// "Define 'is'", Bill Clinton
// "Define 'if'", Steven Rostedt
//

// Optimization barrier

// The "volatile" is due to gcc bugs

//
// This version is i.e. to prevent dead stores elimination on @ptr
// where gcc and llvm may behave differently when otherwise using
// normal barrier(): while gcc behavior gets along with a normal
// barrier(), llvm needs an explicit input variable to be assumed
// clobbered. The issue is as follows: while the inline asm might
// access any memory it wants, the compiler could have fit all of
// @ptr into memory registers instead, and since @ptr never escaped
// from that, it proved that the inline asm wasn't touching any of
// it. This version works well with both compilers, i.e. we're telling
// the compiler that the inline asm absolutely may see the contents
// of @ptr. See also: https://llvm.org/bugs/show_bug.cgi?id=15495
//

// workaround for GCC PR82365 if needed

// Unreachable code

// Annotate a C jump table to allow objtool to follow the code flow

// Macro flag: #define __annotate_jump_table

//
// Mark a position in code as unreachable.  This can be used to
// suppress control flow warnings after asm blocks that transfer
// control elsewhere.
//

//
// KENTRY - kernel entry point
// This can be used to annotate symbols (functions or data) that are used
// without their linker symbol being referenced explicitly. For example,
// interrupt vector handlers, or functions in the kernel image that are found
// programatically.
//
// Not required for symbols exported with EXPORT_SYMBOL, or initcalls. Those
// are handled in their own way (with KEEP() in linker scripts).
//
// KENTRY can be avoided if the symbols in question are marked as KEEP() in the
// linker script. For example an architecture could KEEP() its entire
// boot/exception vector code rather than annotate each function and data.
//

// Make the optimizer believe the variable can be manipulated arbitrarily.

// Format: __UNIQUE_ID_<name>_<__COUNTER__>

//
// data_race - mark an expression as containing intentional data races
//
// This data_race() macro is useful for situations in which data races
// should be forgiven.  One example is diagnostic code that accesses
// shared variables but is not a part of the core synchronization design.
// For example, if accesses to a given variable are protected by a lock,
// except for diagnostic code, then the accesses under the lock should
// be plain C-language accesses and those in the diagnostic code should
// use data_race().  This way, KCSAN will complain if buggy lockless
// accesses to that variable are introduced, even if the buggy accesses
// are protected by READ_ONCE() or WRITE_ONCE().
//
// This macro *does not* affect normal code generation, but is a hint
// to tooling that data races here are to be ignored.  If the access must
// be atomic *and* KCSAN should ignore the access, use both data_race()
// and READ_ONCE(), for example, data_race(READ_ONCE(x)).
//

// &a[0] degrades to a pointer: a different type from an array

//
// If the "nonstring" attribute isn't available, we have to return true
// so the __must_*() checks pass when "nonstring" isn't supported.
//

// Require C Strings (i.e. NUL-terminated) lack the "nonstring" attribute.

//
// Define TYPEOF_UNQUAL() to use __typeof_unqual__() as typeof
// operator when available, to return an unqualified type of the exp.
//

//
// Force a reference to the external symbol so the compiler generates
// __kcfi_typid.
//

// Macro flag: #define KCFI_REFERENCE(sym)

//
// offset_to_ptr - convert a relative memory offset to an absolute pointer
// @off:	the address of the 32-bit offset value
//

//
// Force the compiler to emit 'sym' as a symbol, so that we can reference
// it from inline assembler. Necessary in case 'sym' could be inlined
// otherwise, or eliminated entirely due to lack of references that are
// visible to the compiler.
//

//
// This returns a constant expression while determining if an argument is
// a constant expression, most importantly without evaluating the argument.
// Glory to Martin Uecker <Martin.Uecker@med.uni-goettingen.de>
//
// Details:
// - sizeof() return an integer constant expression, and does not evaluate
// the value of its operand; it only examines the type of its operand.
// - The results of comparing two integer constant expressions is also
// an integer constant expression.
// - The first literal "8" isn't important. It could be any literal value.
// - The second literal "8" is to avoid warnings about unaligned pointers;
// this could otherwise just be "1".
// - (long)(x) is used to avoid warnings about 64-bit types on 32-bit
// architectures.
// - The C Standard defines "null pointer constant", "(void *)0", as
// distinct from other void pointers.
// - If (x) is an integer constant expression, then the "* 0l" resolves
// it into an integer constant expression of value 0. Since it is cast to
// "void *", this makes the second operand a null pointer constant.
// - If (x) is not an integer constant expression, then the second operand
// resolves to a void pointer (but not a null pointer constant: the value
// is not an integer constant 0).
// - The conditional operator's third operand, "(int *)8", is an object
// pointer (to type "int").
// - The behavior (including the return type) of the conditional operator
// ("operand1 ? operand2 : operand3") depends on the kind of expressions
// given for the second and third operands. This is the central mechanism
// of the macro:
// - When one operand is a null pointer constant (i.e. when x is an integer
// constant expression) and the other is an object pointer (i.e. our
// third operand), the conditional operator returns the type of the
// object pointer operand (i.e. "int *"). Here, within the sizeof(), we
// would then get:
// sizeof(*((int *)(...))  == sizeof(int)  == 4
// - When one operand is a void pointer (i.e. when x is not an integer
// constant expression) and the other is an object pointer (i.e. our
// third operand), the conditional operator returns a "void *" type.
// Here, within the sizeof(), we would then get:
// sizeof(*((void *)(...)) == sizeof(void) == 1
// - The equality comparison to "sizeof(int)" therefore depends on (x):
// sizeof(int) == sizeof(int)     (x) was a constant expression
// sizeof(int) != sizeof(void)    (x) was not a constant expression
//

//
// Whether 'type' is a signed type or an unsigned type. Supports scalar types,
// bool and also pointer types.
//

//
// Useful shorthand for "is this condition known at compile-time?"
//
// Note that the condition may involve non-constant values,
// but the compiler may know enough about the details of the
// values to determine that the condition is statically true.
//

//
// Similar to statically_true() but produces a constant expression
//
// To be used in conjunction with macros, such as BUILD_BUG_ON_ZERO(),
// which require their input to be a constant expression and for which
// statically_true() would otherwise fail.
//
// This is a trade-off: const_true() requires all its operands to be
// compile time constants. Else, it would always returns false even on
// the most trivial cases like:
//
// true || non_const_var
//
// On the opposite, statically_true() is able to fold more complex
// tautologies and will return true on expressions such as:
//
// !(non_const_var * 8 % 4)
//
// For the general case, statically_true() is better.
//

//
// This is needed in functions which generate the stack canary, see
// arch/x86/kernel/smpboot.c::start_secondary() for an example.
//

