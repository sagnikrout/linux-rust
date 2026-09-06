//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/bpf_core_read.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

//
// enum bpf_field_info_kind is passed as a second argument into
// __builtin_preserve_field_info() built-in to get a specific aspect of
// a field, captured as a first argument. __builtin_preserve_field_info(field,
// info_kind) returns __u32 integer and produces BTF field relocation, which
// is understood and processed by libbpf during BPF object loading. See
// selftests/bpf for examples.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_field_info_kind {
    BPF_FIELD_BYTE_OFFSET = 0,	/* field byte offset */
    BPF_FIELD_BYTE_SIZE = 1,
    BPF_FIELD_EXISTS = 2,		/* field existence in target kernel */
    BPF_FIELD_SIGNED = 3,
    BPF_FIELD_LSHIFT_U64 = 4,
    BPF_FIELD_RSHIFT_U64 = 5,
}

// second argument to __builtin_btf_type_id() built-in
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_type_id_kind {
    BPF_TYPE_ID_LOCAL = 0,		/* BTF type ID in local program */
    BPF_TYPE_ID_TARGET = 1,		/* BTF type ID in target kernel */
}

// second argument to __builtin_preserve_type_info() built-in
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_type_info_kind {
    BPF_TYPE_EXISTS = 0,		/* type existence in target kernel */
    BPF_TYPE_SIZE = 1,		/* type size in target kernel */
    BPF_TYPE_MATCHES = 2,		/* type match in target kernel */
}

// second argument to __builtin_preserve_enum_value() built-in
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_enum_value_kind {
    BPF_ENUMVAL_EXISTS = 0,		/* enum value existence in kernel */
    BPF_ENUMVAL_VALUE = 1,		/* enum value value relocation */
}

// semantics of LSHIFT_64 assumes loading values into low-ordered bytes, so
// for big-endian we need to adjust destination pointer accordingly, based on
// field byte size
//

//
// Extract bitfield, identified by s->field, and return its value as u64.
// All this is done in relocatable manner, so bitfield changes such as
// signedness, bit size, offset changes, this will be handled automatically.
// This version of macro is using bpf_probe_read_kernel() to read underlying
// integer storage. Macro functions as an expression and its return type is
// bpf_probe_read_kernel()'s return value: 0, on success, <0 on error.
//

//
// Extract bitfield, identified by s->field, and return its value as u64.
// This version of macro is using direct memory reads and should be used from
// BPF program types that support such functionality (e.g., typed raw
// tracepoints).
//

// This is a so-called barrier_var() operation that makes specified   \
// variable "a black box" for optimizing compiler.		      \
// It forces compiler to perform BYTE_OFFSET relocation on p and use  \
// its calculated value in the switch below, instead of applying      \
// the same relocation 4 times for each individual memory load.       \
// \
//
// Write to a bitfield, identified by s->field.
// This is the inverse of BPF_CORE_WRITE_BITFIELD().
//

// Differentiator between compilers builtin implementations. This is a
// requirement due to the compiler parsing differences where GCC optimizes
// early in parsing those constructs of type pointers to the builtin specific
// type, resulting in not being possible to collect the required type
// information in the builtin expansion.
//

//
// Convenience macro to check that field actually exists in target kernel's.
// Returns:
// 1, if matching field is present in target kernel;
// 0, if no matching field found.
//
// Supports two forms:
// - field reference through variable access:
// bpf_core_field_exists(p->my_field);
// - field reference through type and field names:
// bpf_core_field_exists(struct my_type, my_field).
//

//
// Convenience macro to get the byte size of a field. Works for integers,
// struct/unions, pointers, arrays, and enums.
//
// Supports two forms:
// - field reference through variable access:
// bpf_core_field_size(p->my_field);
// - field reference through type and field names:
// bpf_core_field_size(struct my_type, my_field).
//

//
// Convenience macro to get field's byte offset.
//
// Supports two forms:
// - field reference through variable access:
// bpf_core_field_offset(p->my_field);
// - field reference through type and field names:
// bpf_core_field_offset(struct my_type, my_field).
//

//
// Convenience macro to get BTF type ID of a specified type, using a local BTF
// information. Return 32-bit unsigned integer with type ID from program's own
// BTF. Always succeeds.
//

//
// Convenience macro to get BTF type ID of a target kernel's type that matches
// specified local type.
// Returns:
// - valid 32-bit unsigned type ID in kernel BTF;
// - 0, if no matching type was found in a target kernel BTF.
//

//
// Convenience macro to check that provided named type
// (struct/union/enum/typedef) exists in a target kernel.
// Returns:
// 1, if such type is present in target kernel's BTF;
// 0, if no matching type is found.
//

//
// Convenience macro to check that provided named type
// (struct/union/enum/typedef) "matches" that in a target kernel.
// Returns:
// 1, if the type matches in the target kernel's BTF;
// 0, if the type does not match any in the target kernel
//

//
// Convenience macro to get the byte size of a provided named type
// (struct/union/enum/typedef) in a target kernel.
// Returns:
// >= 0 size (in bytes), if type is present in target kernel's BTF;
// 0, if no matching type is found.
//

//
// Convenience macro to check that provided enumerator value is defined in
// a target kernel.
// Returns:
// 1, if specified enum type and its enumerator value are present in target
// kernel's BTF;
// 0, if no matching enum and/or enum value within that enum is found.
//

//
// Convenience macro to get the integer value of an enumerator value in
// a target kernel.
// Returns:
// 64-bit value, if specified enum type and its enumerator value are
// present in target kernel's BTF;
// 0, if no matching enum and/or enum value within that enum is found.
//

//
// bpf_core_read() abstracts away bpf_probe_read_kernel() call and captures
// offset relocation for source address using __builtin_preserve_access_index()
// built-in, provided by Clang.
//
// __builtin_preserve_access_index() takes as an argument an expression of
// taking an address of a field within struct/union. It makes compiler emit
// a relocation, which records BTF type ID describing root struct/union and an
// accessor string which describes exact embedded field that was used to take
// an address. See detailed description of this relocation format and
// semantics in comments to struct bpf_core_relo in include/uapi/linux/bpf.h.
//
// This relocation allows libbpf to adjust BPF instruction to use correct
// actual field offset, based on target kernel BTF type that matches original
// (local) BTF, used to record relocation.
//

// NOTE: see comments for BPF_CORE_READ_USER() about the proper types use.

//
// bpf_core_read_str() is a thin wrapper around bpf_probe_read_str()
// additionally emitting BPF CO-RE field relocation for specified source
// argument.
//

// NOTE: see comments for BPF_CORE_READ_USER() about the proper types use.

//
// Cast provided pointer *ptr* into a pointer to a specified *type* in such
// a way that BPF verifier will become aware of associated kernel-side BTF
// type. This allows to access members of kernel types directly without the
// need to use BPF_CORE_READ() macros.
//

//
// return number of provided arguments; used for switch-based variadic macro
// definitions (see ___last, ___arrow, etc below)
//

//
// return 0 if no arguments are passed, N - otherwise; used for
// recursively-defined macros to specify termination (0) case, and generic
// (N) case (e.g., ___read_ptrs, ___core_read)
//

// "recursively" read a sequence of inner pointers using local __t var

//
// BPF_CORE_READ_INTO() is a more performance-conscious variant of
// BPF_CORE_READ(), in which final field is read into user-provided storage.
// See BPF_CORE_READ() below for more details on general usage.
//

//
// Variant of BPF_CORE_READ_INTO() for reading from user-space memory.
//
// NOTE: see comments for BPF_CORE_READ_USER() about the proper types use.
//

// Non-CO-RE variant of BPF_CORE_READ_INTO()

// Non-CO-RE variant of BPF_CORE_READ_USER_INTO().
//
// As no CO-RE relocations are emitted, source types can be arbitrary and are
// not restricted to kernel types only.
//

//
// BPF_CORE_READ_STR_INTO() does same "pointer chasing" as
// BPF_CORE_READ() for intermediate pointers, but then executes (and returns
// corresponding error code) bpf_core_read_str() for final string read.
//

//
// Variant of BPF_CORE_READ_STR_INTO() for reading from user-space memory.
//
// NOTE: see comments for BPF_CORE_READ_USER() about the proper types use.
//

// Non-CO-RE variant of BPF_CORE_READ_STR_INTO()

//
// Non-CO-RE variant of BPF_CORE_READ_USER_STR_INTO().
//
// As no CO-RE relocations are emitted, source types can be arbitrary and are
// not restricted to kernel types only.
//

//
// BPF_CORE_READ() is used to simplify BPF CO-RE relocatable read, especially
// when there are few pointer chasing steps.
// E.g., what in non-BPF world (or in BPF w/ BCC) would be something like:
// int x = s->a.b.c->d.e->f->g;
// can be succinctly achieved using BPF_CORE_READ as:
// int x = BPF_CORE_READ(s, a.b.c, d.e, f, g);
//
// BPF_CORE_READ will decompose above statement into 4 bpf_core_read (BPF
// CO-RE relocatable bpf_probe_read_kernel() wrapper) calls, logically
// equivalent to:
// 1. const void *__t = s->a.b.c;
// 2. __t = __t->d.e;
// 3. __t = __t->f;
// 4. return __t->g;
//
// Equivalence is logical, because there is a heavy type casting/preservation
// involved, as well as all the reads are happening through
// bpf_probe_read_kernel() calls using __builtin_preserve_access_index() to
// emit CO-RE relocations.
//
// N.B. Only up to 9 "field accessors" are supported, which should be more
// than enough for any practical purpose.
//

//
// Variant of BPF_CORE_READ() for reading from user-space memory.
//
// NOTE: all the source types involved are still *kernel types* and need to
// exist in kernel (or kernel module) BTF, otherwise CO-RE relocation will
// fail. Custom user types are not relocatable with CO-RE.
// The typical situation in which BPF_CORE_READ_USER() might be used is to
// read kernel UAPI types from the user-space memory passed in as a syscall
// input argument.
//

// Non-CO-RE variant of BPF_CORE_READ()

//
// Non-CO-RE variant of BPF_CORE_READ_USER().
//
// As no CO-RE relocations are emitted, source types can be arbitrary and are
// not restricted to kernel types only.
//

