//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/btf.h
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
// Copyright (c) 2018 Facebook
pub const _LINUX_BTF_H: c_int = 1;

// These need to be macros, as the expressions are used in assembler input

// Trusted arguments are those which are guaranteed to be valid when passed to
// the kfunc. It is used to enforce that pointers obtained from either acquire
// kfuncs, or from the main kernel on a tracepoint or struct_ops callback
// invocation, remain unmodified when being passed to helpers taking trusted
// args.
//
// Consider, for example, the following new task tracepoint:
//
// SEC("tp_btf/task_newtask")
// int BPF_PROG(new_task_tp, struct task_struct *task, u64 clone_flags)
// {
// ...
// }
//
// And the following kfunc:
//
// BTF_ID_FLAGS(func, bpf_task_acquire, KF_ACQUIRE)
//
// All invocations to the kfunc must pass the unmodified, unwalked task:
//
// bpf_task_acquire(task);		    // Allowed
// bpf_task_acquire(task->last_wakee); // Rejected, walked task
//
// Programs may also pass referenced tasks directly to the kfunc:
//
// struct task_struct *acquired;
//
// acquired = bpf_task_acquire(task);	// Allowed, same as above
// bpf_task_acquire(acquired);		// Allowed
// bpf_task_acquire(task);			// Allowed
// bpf_task_acquire(acquired->last_wakee); // Rejected, walked task
//
// Programs may _not_, however, pass a task from an arbitrary fentry/fexit, or
// kprobe/kretprobe to the kfunc, as BPF cannot guarantee that all of these
// pointers are guaranteed to be safe. For example, the following BPF program
// would be rejected:
//
// SEC("kretprobe/free_task")
// int BPF_PROG(free_task_probe, struct task_struct *tsk)
// {
// struct task_struct *acquired;
//
// acquired = bpf_task_acquire(acquired); // Rejected, not a trusted pointer
// bpf_task_release(acquired);
//
// return 0;
// }
//

// only one of KF_ITER_{NEW,NEXT,DESTROY} could be specified per kfunc

//
// Tag marking a kernel function as a kfunc. This is meant to minimize the
// amount of copy-paste that kfunc authors have to include for correctness so
// as to avoid issues such as the compiler inlining or eliding either a static
// kfunc, or a global kfunc in an LTO build.
//

//
// Return the name of the passed struct, if exists, or halt the build if for
// example the structure gets renamed. In this way, developers have to revisit
// the code using that structure name, and update it accordingly.
//

extern "C" {
    pub fn int(prog: *const *const btf_kfunc_filter_t)(struct bpf_prog, kfunc_id: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_kfunc_id_set {
    pub owner: *mut module,
    pub set: *mut btf_id_set8,
    pub filter: btf_kfunc_filter_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_id_dtor_kfunc {
    pub btf_id: u32,
    pub kfunc_btf_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_struct_meta {
    pub btf_id: u32,
    pub record: *mut btf_record,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_struct_metas {
    pub cnt: u32,
    pub types: [btf_struct_meta; ],
}

extern "C" {
    pub fn btf_get(btf: *mut btf);
}
extern "C" {
    pub fn btf_put(btf: *mut btf);
}
extern "C" {
    pub fn btf_new_fd(attr: *const bpf_attr, uattr: bpfptr_t, attr_log: *mut bpf_log_attr) -> c_int;
}
// Figure out the size of a type_id.  If type_id is a modifier
// (e.g. const), it will be resolved to find out the type with size.
//
// For example:
// In describing "const void *",  type_id is "const" and "const"
// refers to "void *".  The return type will be "void *".
//
// If type_id is a simple "int", then return type will be "int".
//
// @btf: struct btf object
// @type_id: Find out the size of type_id. The type_id of the return
// type is set to *type_id.
// @ret_size: It can be NULL.  If not NULL, the size of the return
// type is set to *ret_size.
// Return: The btf_type (resolved to another type with size info if needed).
// NULL is returned if type_id itself does not have size info
// (e.g. void) or it cannot be resolved to another type that
// has size info.
// *type_id and *ret_size will not be changed in the
// NULL return case.
//
// Options to control show behaviour.
// - BTF_SHOW_COMPACT: no formatting around type information
// - BTF_SHOW_NONAME: no struct/union member names/types
// - BTF_SHOW_PTR_RAW: show raw (unobfuscated) pointer values;
// equivalent to %px.
// - BTF_SHOW_ZERO: show zero-valued struct/union members; they
// are not displayed by default
// - BTF_SHOW_UNSAFE: skip use of bpf_probe_read() to safely read
// data before displaying it.
//

//
// Copy len bytes of string representation of obj of BTF type_id into buf.
//
// @btf: struct btf object
// @type_id: type id of type obj points to
// @obj: pointer to typed data
// @buf: buffer to write to
// @len: maximum length to write to buf
// @flags: show options (see above)
//
// Return: length that would have been/was copied as per snprintf, or
// negative error.
//
extern "C" {
    pub fn btf_type_name_to_buf(btf: *const btf, type_id: u32, buf: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn btf_get_fd_by_id(id: u32) -> c_int;
}
extern "C" {
    pub fn btf_obj_id(btf: *const btf) -> u32;
}
extern "C" {
    pub fn btf_is_kernel(btf: *const btf) -> bool;
}
extern "C" {
    pub fn btf_is_module(btf: *const btf) -> bool;
}
extern "C" {
    pub fn btf_is_vmlinux(btf: *const btf) -> bool;
}
extern "C" {
    pub fn btf_nr_types(btf: *const btf) -> u32;
}
extern "C" {
    pub fn btf_named_start_id(btf: *const btf, own: bool) -> u32;
}
extern "C" {
    pub fn btf_type_is_i32(t: *const btf_type) -> bool;
}
extern "C" {
    pub fn btf_type_is_i64(t: *const btf_type) -> bool;
}
extern "C" {
    pub fn btf_type_is_primitive(t: *const btf_type) -> bool;
}
extern "C" {
    pub fn btf_check_and_fixup_fields(btf: *const btf, rec: *mut btf_record) -> c_int;
}
extern "C" {
    pub fn btf_type_is_void(t: *const btf_type) -> bool;
}
extern "C" {
    pub fn btf_find_by_name_kind(btf: *const btf, name: *const c_char, kind: u8) -> i32;
}
extern "C" {
    pub fn bpf_find_btf_id(name: *const c_char, kind: u32, btf_p: *mut btf) -> i32;
}
extern "C" {
    pub fn btf_relocate_id(btf: *const btf, id: __u32) -> __u32;
}

extern "C" {
    pub fn btf_type_is_int(sizeof(u64: t) && t->size <=) -> return;
}
extern "C" {
    pub fn BTF_INT_ENCODING(1): *mut *mut *mut (u32 )(t +) -> return;
}
extern "C" {
    pub fn btf_type_is_int(BTF_INT_SIGNED: t) && (btf_int_encoding(t) &) -> return;
}
extern "C" {
    pub fn BTF_INFO_KIND(_arg: t->info) -> return;
}
extern "C" {
    pub fn BTF_INT_OFFSET(1): *mut *mut *mut (u32 )(t +) -> return;
}
extern "C" {
    pub fn BTF_INT_BITS(1): *mut *mut *mut (__u32 )(t +) -> return;
}
extern "C" {
    pub fn btf_type_is_int(btf_type_is_enum(t: t) ||) -> return;
}
// union is only a special case of struct:
// all its offsetof(member) == 0
//
extern "C" {
    pub fn BTF_INFO_VLEN(_arg: t->info) -> return;
}
extern "C" {
    pub fn btf_type_vlen(_arg: t) -> return;
}
extern "C" {
    pub fn BTF_INFO_VLEN(_arg: t->info) -> return;
}
extern "C" {
    pub fn BTF_INFO_KFLAG(_arg: t->info) -> return;
}
extern "C" {
    pub fn __btf_member_bit_offset(_arg: t, _arg: m) -> return;
}
extern "C" {
    pub fn __btf_member_bitfield_size(_arg: t, _arg: m) -> return;
}
extern "C" {
    pub fn bsearch(_arg: &id, _arg: set->pairs, _arg: set->cnt, _arg: sizeof(set->pairs[0]), _arg: btf_id_cmp_func) -> return;
}
extern "C" {
    pub fn btf_ctx_arg_idx(btf: *mut btf, func_proto: *const btf_type, off: c_int) -> u32;
}

extern "C" {
    pub fn __register_bpf_struct_ops(st_ops: *mut bpf_struct_ops) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_field_iter_kind {
    BTF_FIELD_ITER_IDS,
    BTF_FIELD_ITER_STRS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_desc {
// once-per-type offsets
    pub t_offs: [int t_off_cnt,; 2],
// member struct size, or zero, if no members
    pub m_sz: c_int,
// repeated per-member offsets
    pub m_offs: [int m_off_cnt,; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_iter {
    pub desc: btf_field_desc,
    pub p: *mut c_void,
    pub m_idx: c_int,
    pub off_idx: c_int,
    pub vlen: c_int,
}

extern "C" {
    pub fn btf_set_base_btf(btf: *mut btf, base_btf: *const btf);
}
extern "C" {
    pub fn btf_relocate(btf: *mut btf, base_btf: *const btf, map_ids: *mut __u32) -> c_int;
}
extern "C" {
    pub fn btf_kfunc_check_flag(btf: *const btf, kfunc_btf_id: u32, flag: u32) -> c_int;
}
extern "C" {
    pub fn btf_kfunc_is_allowed(btf: *const btf, kfunc_btf_id: u32, prog: *const bpf_prog) -> bool;
}
extern "C" {
    pub fn register_btf_fmodret_id_set(kset: *const btf_kfunc_id_set) -> c_int;
}
extern "C" {
    pub fn btf_find_dtor_kfunc(btf: *mut btf, btf_id: u32) -> i32;
}
extern "C" {
    pub fn btf_is_projection_of(pname: *const c_char, tname: *const c_char) -> bool;
}
extern "C" {
    pub fn get_kern_ctx_btf_id(log: *mut bpf_verifier_log, prog_type: bpf_prog_type) -> c_int;
}
extern "C" {
    pub fn btf_check_iter_arg(btf: *mut btf, func: *const btf_type, arg_idx: c_int) -> c_int;
}
extern "C" {
    pub fn btf_type_is_struct(_arg: t) -> return;
}

