//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compiler_types.h
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
// __has_builtin is supported on gcc >= 10, clang >= 3 and icc >= 21.
// In the meantime, to support gcc < 10, we implement __has_builtin
// by hand.
//

// Indirect macros required for expanded argument pasting, eg. __LINE__.

//
// C23 introduces "auto" as a standard way to define type-inferred
// variables, but "auto" has been a (useless) keyword even since K&R C,
// so it has always been "namespace reserved."
//
// Until at some future time we require C23 support, we need the gcc
// extension __auto_type, but there is no reason to put that elsewhere
// in the source code.
//

//
// Skipped when running bindgen due to a libclang issue;
// see https://github.com/rust-lang/rust-bindgen/issues/2244.
//

// sparse defines __CHECKER__; see Documentation/dev-tools/sparse.rst

// address spaces

// other

// address spaces

// other

// Attributes

// Macro flag: #define __function_aligned

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-cold-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Label-Attributes.html#index-cold-label-attribute
//
// When -falign-functions=N is in use, we must avoid the cold attribute as
// GCC drops the alignment for cold functions. Worse, GCC can implicitly mark
// callees of cold functions as cold themselves, so it's not sufficient to add
// __function_aligned here as that will not ensure that callees are correctly
// aligned.
//
// See:
//
// https://lore.kernel.org/lkml/Y77%2FqVgvaJidFpYt@FVFF77S0Q05N
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=88345#c9
//

// Macro flag: #define __cold

//
// On x86-64 and arm64 targets, __preserve_most changes the calling convention
// of a function to make the code in the caller as unintrusive as possible. This
// convention behaves identically to the C calling convention on how arguments
// and return values are passed, but uses a different set of caller- and callee-
// saved registers.
//
// The purpose is to alleviates the burden of saving and recovering a large
// register set before and after the call in the caller.  This is beneficial for
// rarely taken slow paths, such as error-reporting functions that may be called
// from hot paths.
//
// Note: This may conflict with instrumentation inserted on function entry which
// does not use __preserve_most or equivalent convention (if in assembly). Since
// function tracing assumes the normal C calling convention, where the attribute
// is supported, __preserve_most implies notrace.  It is recommended to restrict
// use of the attribute to functions that should or already disable tracing.
//
// Optional: not supported by gcc.
//
// clang: https://clang.llvm.org/docs/AttributeReference.html#preserve-most
//

//
// Annotating a function/variable with __retain tells the compiler to place
// the object in its own section and set the flag SHF_GNU_RETAIN. This flag
// instructs the linker to retain the object during garbage-cleanup or LTO
// phases.
//
// Note that the __used macro is also used to prevent functions or data
// being optimized out, but operates at the compiler/IR-level and may still
// allow unintended removal of objects during linking.
//
// Optional: only supported since gcc >= 11, clang >= 13
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-retain-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#retain
//

// Compiler specific macros.

// The above compilers also define __GNUC__, so order is important here.

//
// Some architectures need to provide custom definitions of macros provided
// by linux/compiler-*.h, and can do so using asm/compiler.h. We include that
// conditionally rather than using an asm-generic wrapper in order to avoid
// build failures if any C compilation, which will include this file via an
// -include argument in c_flags, occurs prior to the asm-generic wrappers being
// generated.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_branch_data {
    pub func: *const c_char,
    pub file: *const c_char,
    pub line: unsigned,
    pub correct: c_ulong,
    pub incorrect: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_likely_data {
    pub data: ftrace_branch_data,
    pub constant: c_ulong,
}

//
// it doesn't make sense on ARM (currently the only user of __naked)
// to trace naked functions because then mcount is called without
// stack and frame pointer being set up and there is no chance to
// restore the lr register to the value before mcount was called.
//

//
// Prefer gnu_inline, so that extern inline functions do not emit an
// externally visible function. This makes extern inline behave as per gnu89
// semantics rather than c99. This prevents multiple symbol definition errors
// of extern inline functions at link time.
// A lot of inline functions can cause havoc with function tracing.
//

//
// gcc provides both __inline__ and __inline as alternate spellings of
// the inline keyword, though the latter is undocumented. New kernel
// code should only use the inline spelling, but some existing code
// uses __inline__. Since we #define inline above, to ensure
// __inline__ has the same semantics, we need this #define.
//
// However, the spelling __inline is strictly reserved for referring
// to the bare keyword.
//

//
// GCC does not warn about unused static inline functions for -Wunused-function.
// Suppress the warning in clang as well by using __maybe_unused, but enable it
// for W=2 build. This will allow clang to find unused functions.
//

// Macro flag: #define __inline_maybe_unused

//
// Rather then using noinline to prevent stack consumption, use
// noinline_for_stack instead.  For documentation reasons.
//

//
// Use noinline_for_tracing for functions that should not be inlined.
// For tracing reasons.
//

//
// Sanitizer helper attributes: Because using __always_inline and
// __no_sanitize_* conflict, provide helper attributes that will either expand
// to __no_sanitize_* in compilation units where instrumentation is enabled
// (__SANITIZE_*__), or __always_inline in compilation units without
// instrumentation (__SANITIZE_*__ undefined).
//

//
// We can't declare function 'inline' because __no_sanitize_address conflicts
// with inlining. Attempt to inline it may cause a build failure.
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=67368
// '__maybe_unused' allows us to avoid defined-but-not-used warnings.
//

//
// Type qualifier to mark variables where all data-racy accesses should be
// ignored by KCSAN. Note, the implementation simply marks these variables as
// volatile, since KCSAN will treat such accesses as "marked".
//
// Defined here because defining __data_racy as volatile for KCSAN objects only
// causes problems in BPF Type Format (BTF) generation since struct members
// of core kernel data structs will be volatile in some objects and not in
// others.  Instead define it globally for KCSAN kernels.
//

//
// Clang still emits instrumentation for __tsan_func_{entry,exit}() and builtin
// atomics even with __no_sanitize_thread (to avoid false positives in userspace
// ThreadSanitizer). The kernel's requirements are stricter and we really do not
// want any instrumentation with __no_kcsan.
//
// Therefore we add __disable_sanitizer_instrumentation where available to
// disable all instrumentation. See Kconfig.kcsan where this is mandatory.
//

//
// Similarly to KASAN and KCSAN, KMSAN loses function attributes of inlined
// functions, therefore disabling KMSAN checks also requires disabling inlining.
//
// __no_sanitize_or_inline effectively prevents KMSAN from reporting errors
// within the function and marks all its outputs as initialized.
//

//
// The assume attribute is used to indicate that a certain condition is
// assumed to be true. If this condition is violated at runtime, the behavior
// is undefined. Compilers may or may not use this indication to generate
// optimized code.
//
// Note that the clang documentation states that optimizers may react
// differently to this attribute, and this may even have a negative
// performance impact. Therefore this attribute should be used with care.
//
// Optional: only supported since gcc >= 13
// Optional: only supported since clang >= 19
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Statement-Attributes.html#index-assume-statement-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#id13
//

//
// Optional: only supported since gcc >= 15
// Optional: only supported since clang >= 18
//
// gcc: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108896
// clang: https://clang.llvm.org/docs/AttributeReference.html#counted-by-counted-by-or-null-sized-by-sized-by-or-null
//
// __bdos on clang < 19.1.2 can erroneously return 0:
// https://github.com/llvm/llvm-project/pull/110497
//
// __bdos on clang < 19.1.3 can be off by 4:
// https://github.com/llvm/llvm-project/pull/112636
//

//
// Runtime track number of objects pointed to by a pointer member for use by
// CONFIG_FORTIFY_SOURCE and CONFIG_UBSAN_BOUNDS.
//
// Optional: only supported since gcc >= 16
// Optional: only supported since clang >= 22
//
// gcc: https://gcc.gnu.org/pipermail/gcc-patches/2025-April/681727.html
// clang: https://clang.llvm.org/docs/AttributeReference.html#counted-by-counted-by-or-null-sized-by-sized-by-or-null
//

// Macro flag: #define __counted_by_ptr(member)

//
// Optional: only supported since gcc >= 15
// Optional: not supported by Clang
//
// gcc: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=117178
//

//
// Apply __counted_by() when the Endianness matches to increase test coverage.
//

// Macro flag: #define __counted_by_be(member)

// Macro flag: #define __counted_by_le(member)

//
// This designates the minimum number of elements a passed array parameter must
// have. For example:
//
// void some_function(u8 param[at_least 7]);
//
// If a caller passes an array with fewer than 7 elements, the compiler will
// emit a warning.
//

// Macro flag: #define at_least

// Section for code which can't be instrumented at all

//
// The __cpuidle section is used twofold:
//
// 1) the original use -- identifying if a CPU is 'stuck' in idle state based
// on it's instruction pointer. See cpu_in_idle().
//
// 2) supressing instrumentation around where cpuidle disables RCU; where the
// function isn't strictly required for #1, this is interchangeable with
// noinstr.
//

//
// The below symbols may be defined for one or more, but not ALL, of the above
// compilers. We don't consider that to be an error, so set them to nothing.
// For example, some of them are for compiler specific plugins.
//

// This anon struct can add padding, so only enable it under randstruct.

//
// Any place that could be marked with the "alloc_size" attribute is also
// a place to be marked with the "malloc" attribute, except those that may
// be performing a _reallocation_, as that may alias the existing pointer.
// For these, use __realloc_size().
//

//
// When the size of an allocated object is needed, use the best available
// mechanism to find it. (For cases where sizeof() cannot be used.)
//
// Optional: only supported since gcc >= 12
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Object-Size-Checking.html
// clang: https://clang.llvm.org/docs/LanguageExtensions.html#evaluating-object-size
//

//
// Determine if an attribute has been applied to a variable.
// Using __annotated needs to check for __annotated being available,
// or negative tests may fail when annotation cannot be checked. For
// example, see the definition of __is_cstr().
//

//
// Optional: only supported since gcc >= 15, clang >= 19
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Other-Builtins.html#index-_005f_005fbuiltin_005fcounted_005fby_005fref
// clang: https://clang.llvm.org/docs/LanguageExtensions.html#builtin-counted-by-ref
//

//
// __flex_counter() - Get pointer to counter member for the given
// flexible array, if it was annotated with __counted_by()
// @FAM: Pointer to flexible array member of an addressable struct instance
//
// For example, with:
//
// struct foo {
// int counter;
// short array[] __counted_by(counter);
// } *p;
//
// __flex_counter(p->array) will resolve to &p->counter.
//
// Note that Clang may not allow this to be assigned to a separate
// variable; it must be used directly.
//
// If p->array is unannotated, this returns (void *)NULL.
//

//
// Some versions of gcc do not mark 'asm goto' volatile:
//
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=103979
//
// We do it here by hand, because it doesn't hurt.
//

//
// Clang has trouble with constraints with multiple
// alternative behaviors ("g" , "rm" and "=rm").
//

//
// Use __typeof_unqual__() when available.
//

// Are two types/vars the same type (ignoring qualifiers)?

//
// __unqual_scalar_typeof(x) - Declare an unqualified scalar type, leaving
// non-scalar types unchanged.
//

//
// Prefer C11 _Generic for better compile-times and simpler code. Note: 'char'
// is not type-compatible with 'signed char', and we define a separate case.
//

//
// __signed_scalar_typeof(x) - Declare a signed scalar type, leaving
// non-scalar types unchanged.
//

// Is this type a native word size -- useful for atomic operations

//
// #ifdef __OPTIMIZE__ is only a good approximation; for instance "make
// CFLAGS_foo.o=-Og" defines __OPTIMIZE__, does not elide the conditional code
// and can break compilation with wrong error message(s). Combine with
// -U__OPTIMIZE__ when needed.
//

// \
// __noreturn is needed to give the compiler enough	\
// information to avoid certain possibly-uninitialized	\
// warnings (regardless of the build failing).		\
// \

//
// compiletime_assert - break build and emit msg if condition is false
// @condition: a compile-time constant condition to check
// @msg:       a message to emit if condition is false
//
// In tradition of POSIX assert, this macro will break the build if the
// supplied condition is *false*, emitting the supplied error message if the
// compiler has support to do so.
//

// Helpers for emitting diagnostics in pragmas.

// Macro flag: #define __diag(string)

