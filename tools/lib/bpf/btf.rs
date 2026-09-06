//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/btf.h
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
// Copyright (c) 2018 Facebook
// ! \file

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_endianness {
    BTF_LITTLE_ENDIAN = 0,
    BTF_BIG_ENDIAN = 1,
}

//
// @brief **btf__free()** frees all data of a BTF object
// @param btf BTF object to free
//
extern "C" {
    pub fn btf__free(btf: *mut btf) -> LIBBPF_API void;
}
//
// @brief **btf__new()** creates a new instance of a BTF object from the raw
// bytes of an ELF's BTF section
// @param data raw bytes
// @param size number of bytes passed in `data`
// @return new BTF object instance which has to be eventually freed with
// **btf__free()
//
// On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
// error code from such a pointer `libbpf_get_error()` should be used. If
// `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
// returned on error instead. In both cases thread-local `errno` variable is
// always set to error code as well.
//
// @brief **btf__new_split()** create a new instance of a BTF object from the
// provided raw data bytes. It takes another BTF instance, **base_btf**, which
// serves as a base BTF, which is extended by types in a newly created BTF
// instance
// @param data raw bytes
// @param size length of raw bytes
// @param base_btf the base BTF object
// @return new BTF object instance which has to be eventually freed with
// **btf__free()
//
// If *base_btf* is NULL, `btf__new_split()` is equivalent to `btf__new()` and
// creates non-split BTF.
//
// On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
// error code from such a pointer `libbpf_get_error()` should be used. If
// `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
// returned on error instead. In both cases thread-local `errno` variable is
// always set to error code as well.
//
// @brief **btf__new_empty()** creates an empty BTF object.  Use
// `btf__add_*()` to populate such BTF object.
// @return new BTF object instance which has to be eventually freed with
// **btf__free()
//
// On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
// error code from such a pointer `libbpf_get_error()` should be used. If
// `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
// returned on error instead. In both cases thread-local `errno` variable is
// always set to error code as well.
//
// @brief **btf__new_empty_split()** creates an unpopulated BTF object from an
// ELF BTF section except with a base BTF on top of which split BTF should be
// based
// @param base_btf base BTF object
// @return new BTF object instance which has to be eventually freed with
// **btf__free()
//
// If *base_btf* is NULL, `btf__new_empty_split()` is equivalent to
// `btf__new_empty()` and creates non-split BTF.
//
// On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
// error code from such a pointer `libbpf_get_error()` should be used. If
// `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
// returned on error instead. In both cases thread-local `errno` variable is
// always set to error code as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_new_opts {
    pub sz: usize,
    pub /: *mut *mut *mut btf base_btf; / optional base BTF,
    pub /: *mut *mut bool add_layout; / add BTF layout information,
}

//
// @brief **btf__new_empty_opts()** creates an unpopulated BTF object with
// optional *base_btf* and BTF kind layout description if *add_layout
// is set
// @return new BTF object instance which has to be eventually freed with
// **btf__free()
//
// On error, NULL is returned and the thread-local `errno` variable is
// set to the error code.
//
// @brief **btf__distill_base()** creates new versions of the split BTF
// *src_btf* and its base BTF. The new base BTF will only contain the types
// needed to improve robustness of the split BTF to small changes in base BTF.
// When that split BTF is loaded against a (possibly changed) base, this
// distilled base BTF will help update references to that (possibly changed)
// base BTF.
// @param src_btf source split BTF object
// @param new_base_btf pointer to where the new base BTF object pointer will be stored
// @param new_split_btf pointer to where the new split BTF object pointer will be stored
// @return 0 on success; negative error code, otherwise
//
// Both the new split and its associated new base BTF must be freed by
// the caller.
//
// If successful, 0 is returned and **new_base_btf** and **new_split_btf
// will point at new base/split BTF. Both the new split and its associated
// new base BTF must be freed by the caller.
//
// A negative value is returned on error and the thread-local `errno` variable
// is set to the error code as well.
//
extern "C" {
    pub fn btf__load_into_kernel(btf: *mut btf) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__type_cnt(btf: *const btf) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn btf__pointer_size(btf: *const btf) -> LIBBPF_API size_t;
}
extern "C" {
    pub fn btf__set_pointer_size(btf: *mut btf, ptr_sz: usize) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__endianness(btf: *const btf) -> LIBBPF_API enum btf_endianness;
}
extern "C" {
    pub fn btf__set_endianness(btf: *mut btf, endian: btf_endianness) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__resolve_size(btf: *const btf, type_id: __u32) -> LIBBPF_API __s64;
}
extern "C" {
    pub fn btf__resolve_type(btf: *const btf, type_id: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__align_of(btf: *const btf, id: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__fd(btf: *const btf) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__set_fd(btf: *mut btf, fd: c_int) -> LIBBPF_API void;
}
extern "C" {
    pub fn btf_ext__free(btf_ext: *mut btf_ext) -> LIBBPF_API void;
}
extern "C" {
    pub fn btf_ext__endianness(btf_ext: *const btf_ext) -> LIBBPF_API enum btf_endianness;
}
extern "C" {
    pub fn btf__find_str(btf: *mut btf, s: *const c_char) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_str(btf: *mut btf, s: *const c_char) -> LIBBPF_API int;
}
//
// @brief **btf__add_btf()** appends all the BTF types from *src_btf* into *btf
// @param btf BTF object which all the BTF types and strings are added to
// @param src_btf BTF object which all BTF types and referenced strings are copied from
// @return BTF type ID of the first appended BTF type, or negative error code
//
// **btf__add_btf()** can be used to simply and efficiently append the entire
// contents of one BTF object to another one. All the BTF type data is copied
// over, all referenced type IDs are adjusted by adding a necessary ID offset.
// Only strings referenced from BTF types are copied over and deduplicated, so
// if there were some unused strings in *src_btf*, those won't be copied over,
// which is consistent with the general string deduplication semantics of BTF
// writing APIs.
//
// If any error is encountered during this process, the contents of *btf* is
// left intact, which means that **btf__add_btf()** follows the transactional
// semantics and the operation as a whole is all-or-nothing.
//
// *src_btf* has to be non-split BTF, as of now copying types from split BTF
// is not supported and will result in -ENOTSUP error code returned.
//
extern "C" {
    pub fn btf__add_btf(btf: *mut btf, src_btf: *const btf) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: usize, encoding: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_float(btf: *mut btf, name: *const c_char, byte_sz: usize) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_ptr(btf: *mut btf, ref_type_id: c_int) -> LIBBPF_API int;
}
// struct/union construction APIs
extern "C" {
    pub fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_union(btf: *mut btf, name: *const c_char, sz: __u32) -> LIBBPF_API int;
}
// enum construction APIs
extern "C" {
    pub fn btf__add_enum(btf: *mut btf, name: *const c_char, bytes_sz: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_enum_value(btf: *mut btf, name: *const c_char, value: __s64) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_enum64(btf: *mut btf, name: *const c_char, bytes_sz: __u32, is_signed: bool) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_enum64_value(btf: *mut btf, name: *const c_char, value: __u64) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_fwd_kind {
    BTF_FWD_STRUCT = 0,
    BTF_FWD_UNION = 1,
    BTF_FWD_ENUM = 2,
}

extern "C" {
    pub fn btf__add_fwd(btf: *mut btf, name: *const c_char, fwd_kind: btf_fwd_kind) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_typedef(btf: *mut btf, name: *const c_char, ref_type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_volatile(btf: *mut btf, ref_type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_const(btf: *mut btf, ref_type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_restrict(btf: *mut btf, ref_type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_type_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_type_attr(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> LIBBPF_API int;
}
// func and func_proto construction APIs
extern "C" {
    pub fn btf__add_func_proto(btf: *mut btf, ret_type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: c_int) -> LIBBPF_API int;
}
// var & datasec construction APIs
extern "C" {
    pub fn btf__add_var(btf: *mut btf, name: *const c_char, linkage: c_int, type_id: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn btf__add_datasec(btf: *mut btf, name: *const c_char, byte_sz: __u32) -> LIBBPF_API int;
}
// tag construction API
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_dedup_opts {
    pub sz: usize,
// optional .BTF.ext info to dedup along the main BTF info
    pub btf_ext: *mut btf_ext,
// force hash collisions (used for testing)
    pub force_collisions: bool,
    pub :0: usize,
}

extern "C" {
    pub fn btf__dedup(btf: *mut btf, opts: *const btf_dedup_opts) -> LIBBPF_API int;
}
//
// @brief **btf__relocate()** will check the split BTF *btf* for references
// to base BTF kinds, and verify those references are compatible with
// *base_btf*; if they are, *btf* is adjusted such that is re-parented to
// *base_btf* and type ids and strings are adjusted to accommodate this.
// @param btf split BTF object to relocate
// @param base_btf base BTF object
// @return 0 on success; negative error code, otherwise
//
// If successful, 0 is returned and **btf** now has **base_btf** as its
// base.
//
// A negative value is returned on error and the thread-local `errno` variable
// is set to the error code as well.
//
extern "C" {
    pub fn btf__relocate(btf: *mut btf, base_btf: *const btf) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_permute_opts {
    pub sz: usize,
// optional .BTF.ext info along the main BTF info
    pub btf_ext: *mut btf_ext,
    pub :0: usize,
}

//
// @brief **btf__permute()** rearranges BTF types in-place according to a specified ID mapping
// @param btf BTF object to permute
// @param id_map Array mapping original type IDs to new IDs
// @param id_map_cnt Number of elements in @id_map
// @param opts Optional parameters, including BTF extension data for reference updates
// @return 0 on success, negative error code on failure
//
// **btf__permute()** reorders BTF types based on the provided @id_map array,
// updating all internal type references to maintain consistency. The function
// operates in-place, modifying the BTF object directly.
//
// For **base BTF**:
// - @id_map must include all types from ID 0 to `btf__type_cnt(btf) - 1`
// - @id_map_cnt must be `btf__type_cnt(btf)`
// - Mapping is defined as `id_map[original_id] = new_id`
// - `id_map[0]` must be 0 (void type cannot be moved)
//
// For **split BTF**:
// - @id_map must include only split types (types added on top of the base BTF)
// - @id_map_cnt must be `btf__type_cnt(btf) - btf__type_cnt(btf__base_btf(btf))`
// - Mapping is defined as `id_map[original_id - start_id] = new_id`
// - `start_id` equals `btf__type_cnt(btf__base_btf(btf))`
//
// After permutation, all type references within the BTF data and optional
// BTF extension (if provided via @opts) are updated automatically.
//
// On error, returns a negative error code and sets errno:
// - `-EINVAL`: Invalid parameters or invalid ID mapping
// - `-ENOMEM`: Memory allocation failure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_dump_opts {
    pub sz: usize,
}

extern "C" {
    pub fn void(ctx: *mut *mut btf_dump_printf_fn_t)(void, fmt: *const c_char, args: va_list) -> typedef;
}
extern "C" {
    pub fn btf_dump__free(d: *mut btf_dump) -> LIBBPF_API void;
}
extern "C" {
    pub fn btf_dump__dump_type(d: *mut btf_dump, id: __u32) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_dump_emit_type_decl_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// optional field name for type declaration, e.g.:
// - struct my_struct <FNAME>
// - void (*<FNAME>)(int)
// - char (*<FNAME>)[123]
//
    pub field_name: *const c_char,
// extra indentation level (in number of tabs) to emit for multi-line
// type declarations (e.g., anonymous struct); applies for lines
// starting from the second one (first line is assumed to have
// necessary indentation already
//
    pub indent_level: c_int,
// strip all the const/volatile/restrict mods
    pub strip_mods: bool,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_dump_type_data_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
    pub indent_str: *const c_char,
    pub indent_level: c_int,
// below match "show" flags for bpf_show_snprintf()
    pub /: *mut *mut bool compact; / no newlines/indentation,
    pub /: *mut *mut bool skip_names; / skip member/type names,
    pub /: *mut *mut bool emit_zeroes; / show 0-valued fields,
    pub /: *mut *mut bool emit_strings; / print char arrays as strings,
    pub :0: usize,
}

//
// A set of helpers for easier BTF types handling.
//
// The inline functions below rely on constants from the kernel headers which
// may not be available for applications including this header file. To avoid
// compilation errors, we define all the constants here that were added after
// the initial introduction of the BTF_KIND* constants.
//

// The kernel header switched to enums, so the following were never #defined

extern "C" {
    pub fn BTF_INFO_KIND(_arg: t->info) -> return;
}
extern "C" {
    pub fn BTF_INFO_VLEN(_arg: t->info) -> return;
}
extern "C" {
    pub fn BTF_INFO_KFLAG(_arg: t->info) -> return;
}
extern "C" {
    pub fn btf_is_enum(btf_is_enum64(t: t) ||) -> return;
}
extern "C" {
    pub fn BTF_INT_ENCODING(1): *mut *mut *mut (__u32 )(t +) -> return;
}
extern "C" {
    pub fn BTF_INT_OFFSET(1): *mut *mut *mut (__u32 )(t +) -> return;
}
extern "C" {
    pub fn BTF_INT_BITS(1): *mut *mut *mut (__u32 )(t +) -> return;
}
// struct btf_enum64 is introduced in Linux 6.0, which is very
// bleeding-edge. Here we are avoiding relying on struct btf_enum64
// definition coming from kernel UAPI headers to support wider range
// of system-wide kernel headers.
//
// Given this header can be also included from C++ applications, that
// further restricts C tricks we can use (like using compatible
// anonymous struct). So just treat struct btf_enum64 as
// a three-element array of u32 and access second (lo32) and third
// (hi32) elements directly.
//
// For reference, here is a struct btf_enum64 definition:
//
// const struct btf_enum64 {
// __u32	name_off;
// __u32	val_lo32;
// __u32	val_hi32;
// };
//
// Get bit offset of a member with specified index.
//
// Get bitfield size of a member, assuming t is BTF_KIND_STRUCT or
// BTF_KIND_UNION. If member is not a bitfield, zero is returned.
//

