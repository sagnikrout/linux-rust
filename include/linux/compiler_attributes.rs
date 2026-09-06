//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compiler_attributes.h
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
// The attributes in this file are unconditionally defined and they directly
// map to compiler attribute(s), unless one of the compilers does not support
// the attribute. In that case, __has_attribute is used to check for support
// and the reason is stated in its comment ("Optional: ...").
//
// Any other "attributes" (i.e. those that depend on a configuration option,
// on a compiler, on an architecture, on plugins, on other attributes...)
// should be defined elsewhere (e.g. compiler_types.h or compiler-*.h).
// The intention is to keep this file as simple as possible, as well as
// compiler- and version-agnostic (e.g. avoiding GCC_VERSION checks).
//
// This file is meant to be sorted (by actual attribute name,
// not by #define identifier). Use the __attribute__((__name__)) syntax
// (i.e. with underscores) to avoid future collisions with other macros.
// Provide links to the documentation of each supported compiler, if it exists.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-alias-function-attribute
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-aligned-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Type-Attributes.html#index-aligned-type-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-aligned-variable-attribute
//

//
// Note: do not use this directly. Instead, use __alloc_size() since it is conditionally
// available and includes other attributes. For GCC < 9.1, __alloc_size__ gets undefined
// in compiler-gcc.h, due to misbehaviors.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-alloc_005fsize-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#alloc-size
//

//
// Note: users of __always_inline currently do not write "inline" themselves,
// which seems to be required by gcc to apply the attribute according
// to its docs (and also "warning: always_inline function might not be
// inlinable [-Wattributes]" is emitted).
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-always_005finline-function-attribute
// clang: mentioned
//

//
// The second argument is optional (default 0), so we use a variadic macro
// to make the shorthand.
//
// Beware: Do not apply this to functions which may return
// ERR_PTRs. Also, it is probably unwise to apply it to functions
// returning extra information in the low bits (but in that case the
// compiler should see some alignment anyway, when the return value is
// massaged by 'flags = ptr & 3; ptr &= ~3;').
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-assume_005faligned-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#assume-aligned
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-cleanup-variable-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#cleanup
//

//
// Note the long name.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-const-function-attribute
//

//
// Optional: only supported since gcc >= 9
// Optional: not supported by clang
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-copy-function-attribute
//

//
// Optional: not supported by gcc
// Optional: only supported since clang >= 14.0
//
// clang: https://clang.llvm.org/docs/AttributeReference.html#diagnose_as_builtin
//

//
// Don't. Just don't. See commit 771c035372a0 ("deprecate the '__deprecated'
// attribute warnings entirely and for good") for more information.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-deprecated-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Type-Attributes.html#index-deprecated-type-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-deprecated-variable-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Enumerator-Attributes.html#index-deprecated-enumerator-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#deprecated
//
// Macro flag: #define __deprecated
//
// Optional: not supported by clang
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Type-Attributes.html#index-designated_005finit-type-attribute
//

//
// Optional: only supported since clang >= 14.0
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-error-function-attribute
//

//
// Optional: not supported by clang
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-externally_005fvisible-function-attribute
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-format-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#format
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-gnu_005finline-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#gnu-inline
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-malloc-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#malloc
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Type-Attributes.html#index-mode-type-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-mode-variable-attribute
//

//
// Optional: only supported since gcc >= 7
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-no_005fcaller_005fsaved_005fregisters-function-attribute_002c-x86
// clang: https://clang.llvm.org/docs/AttributeReference.html#no-caller-saved-registers
//

//
// Optional: not supported by clang
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-noclone-function-attribute
//

//
// Add the pseudo keyword 'fallthrough' so case statement blocks
// must end with any of these keywords:
// break;
// fallthrough;
// continue;
// goto <label>;
// return [expression];
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Statement-Attributes.html#Statement-Attributes
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#Common-Function-Attributes
// clang: https://clang.llvm.org/docs/AttributeReference.html#flatten
//

//
// Note the missing underscores.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-noinline-function-attribute
// clang: mentioned
//

//
// Note: deliberately not named '__nonnull', to avoid clashing with glibc's
// __nonnull() when kernel and userspace headers are combined.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Attributes.html#index-nonnull
// clang: https://clang.llvm.org/docs/AttributeReference.html#nonnull
//

//
// Optional: only supported since gcc >= 8
// Optional: not supported by clang
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-nonstring-variable-attribute
//

//
// Optional: only supported since GCC >= 7.1, clang >= 13.0.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-no_005fprofile_005finstrument_005ffunction-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#no-profile-instrument-function
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-noreturn-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#noreturn
// clang: https://clang.llvm.org/docs/AttributeReference.html#id1
//

//
// Optional: only supported since GCC >= 11.1, clang >= 7.0.
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-no_005fstack_005fprotector-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#no-stack-protector-safebuffers
//

//
// Optional: not supported by gcc.
//
// clang: https://clang.llvm.org/docs/AttributeReference.html#overloadable
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Type-Attributes.html#index-packed-type-attribute
// clang: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-packed-variable-attribute
//

//
// Note: the "type" argument should match any __builtin_object_size(p, type) usage.
//
// Optional: not supported by gcc.
//
// clang: https://clang.llvm.org/docs/AttributeReference.html#pass-object-size-pass-dynamic-object-size
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-pure-function-attribute
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-section-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-section-variable-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#section-declspec-allocate
//

//
// Optional: only supported since gcc >= 12
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-uninitialized-variable-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#uninitialized
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-unused-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Type-Attributes.html#index-unused-type-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-unused-variable-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Label-Attributes.html#index-unused-label-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#maybe-unused-unused
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-used-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-used-variable-attribute
//

//
// The __used attribute guarantees that the attributed variable will be
// always emitted by a compiler. It doesn't prevent the compiler from
// throwing 'unused' warnings when it can't detect how the variable is
// actually used. It's a compiler implementation details either emit
// the warning in that case or not.
//
// The combination of both 'used' and 'unused' attributes ensures that
// the variable would be emitted, and will not trigger 'unused' warnings.
// The attribute is applicable for functions, static and global variables.
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-warn_005funused_005fresult-function-attribute
// clang: https://clang.llvm.org/docs/AttributeReference.html#nodiscard-warn-unused-result
//

//
// Optional: only supported since clang >= 14.0
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-warning-function-attribute
//

//
// Optional: only supported since clang >= 14.0
//
// clang: https://clang.llvm.org/docs/AttributeReference.html#disable-sanitizer-instrumentation
//
// disable_sanitizer_instrumentation is not always similar to
// no_sanitize((<sanitizer-name>)): the latter may still let specific sanitizers
// insert code into functions to prevent false positives. Unlike that,
// disable_sanitizer_instrumentation prevents all kinds of instrumentation to
// functions with the attribute.
//

//
// Optional: not supported by clang
//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Attributes.html#index-noipa
//

//
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-weak-function-attribute
// gcc: https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-weak-variable-attribute
//

//
// Used by functions that use '__builtin_return_address'. These function
// don't want to be splited or made inline, which can make
// the '__builtin_return_address' get unexpected address.
//

