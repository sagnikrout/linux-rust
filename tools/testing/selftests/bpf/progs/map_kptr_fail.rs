//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/map_kptr_fail.c
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
pub struct map_value {
    pub buf: [c_char; 8],
    pub unref_ptr: *mut prog_test_ref_kfunc __kptr_untrusted,
    pub ref_ptr: *mut prog_test_ref_kfunc __kptr,
    pub ref_memb_ptr: *mut prog_test_member __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } array_map,
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(BPF_DW": "kptr access size must be) -> __failure {
    __failure __msg("kptr access size must be BPF_DW")
#[no_mangle]
pub unsafe extern "C" fn size_not_bpf_dw(ctx: *mut __sk_buff) -> c_int {
    int size_not_bpf_dw(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
// (u32 *)&v->unref_ptr = 0;
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset": "kptr access cannot have variable) -> __failure {
    __failure __msg("kptr access cannot have variable offset")
#[no_mangle]
pub unsafe extern "C" fn non_const_var_off(ctx: *mut __sk_buff) -> c_int {
    int non_const_var_off(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub id: int key = 0,,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub ctx->protocol: id =,
    if (id < 4 || id > 12)
    pub 0: return,
// (u64 *)((void *)v + id) = 0;
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(be": "R1 doesn't have constant offset. kptr has to) -> __failure {
    __failure __msg("R1 doesn't have constant offset. kptr has to be")
#[no_mangle]
pub unsafe extern "C" fn non_const_var_off_kptr_xchg(ctx: *mut __sk_buff) -> c_int {
    int non_const_var_off_kptr_xchg(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub id: int key = 0,,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub ctx->protocol: id =,
    if (id < 4 || id > 12)
    pub 0: return,
    pub NULL): *mut *mut bpf_kptr_xchg((void )v + id,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(off=7": "kptr access misaligned expected=8) -> __failure {
    __failure __msg("kptr access misaligned expected=8 off=7")
#[no_mangle]
pub unsafe extern "C" fn misaligned_access_write(ctx: *mut __sk_buff) -> c_int {
    int misaligned_access_write(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
// (void **)((void *)v + 7) = NULL;
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(off=1": "kptr access misaligned expected=8) -> __failure {
    __failure __msg("kptr access misaligned expected=8 off=1")
#[no_mangle]
pub unsafe extern "C" fn misaligned_access_read(ctx: *mut __sk_buff) -> c_int {
    int misaligned_access_read(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub 1): *mut *mut *mut *mut return (u64 )((void )v +,
    }
    SEC("?tc")
    pub 0x1e0)"): __failure __msg("variable untrusted_ptr_ access var_off=(0x0;,
#[no_mangle]
pub unsafe extern "C" fn reject_var_off_store(ctx: *mut __sk_buff) -> c_int {
    int reject_var_off_store(struct __sk_buff *ctx)
    {
    pub unref_ptr: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub id: int key = 0,,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub v->unref_ptr: unref_ptr =,
    if (!unref_ptr)
    pub 0: return,
    pub ctx->protocol: id =,
    if (id < 4 || id > 12)
    pub 0: return,
    pub id: unref_ptr +=,
    pub unref_ptr: v->unref_ptr =,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(access: "invalid kptr, type=untrusted_ptr_prog_test_ref_kfunc": R1) -> __failure {
    __failure __msg("invalid kptr access, R1 type=untrusted_ptr_prog_test_ref_kfunc")
#[no_mangle]
pub unsafe extern "C" fn reject_bad_type_match(ctx: *mut __sk_buff) -> c_int {
    int reject_bad_type_match(struct __sk_buff *ctx)
    {
    pub unref_ptr: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub v->unref_ptr: unref_ptr =,
    if (!unref_ptr)
    pub 0: return,
    pub 4: *mut *mut unref_ptr = (void )unref_ptr +,
    pub unref_ptr: v->unref_ptr =,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=percpu_ptr_": "R1 type=untrusted_ptr_or_null_) -> __failure {
    __failure __msg("R1 type=untrusted_ptr_or_null_ expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn marked_as_untrusted_or_null(ctx: *mut __sk_buff) -> c_int {
    int marked_as_untrusted_or_null(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(4": "access beyond struct prog_test_ref_kfunc at off 32 size) -> __failure {
    __failure __msg("access beyond struct prog_test_ref_kfunc at off 32 size 4")
#[no_mangle]
pub unsafe extern "C" fn correct_btf_id_check_size(ctx: *mut __sk_buff) -> c_int {
    int correct_btf_id_check_size(struct __sk_buff *ctx)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub v->unref_ptr: p =,
    if (!p)
    pub 0: return,
    pub prog_test_ref_kfunc)): *mut *mut *mut *mut return (int )((void )p + bpf_core_type_size(struct,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=percpu_ptr_": "R1 type=untrusted_ptr_) -> __failure {
    __failure __msg("R1 type=untrusted_ptr_ expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn inherit_untrusted_on_walk(ctx: *mut __sk_buff) -> c_int {
    int inherit_untrusted_on_walk(struct __sk_buff *ctx)
    {
    pub unref_ptr: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub v->unref_ptr: unref_ptr =,
    if (!unref_ptr)
    pub 0: return,
    pub unref_ptr->next: unref_ptr =,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(kptr": "off=8 kptr isn't referenced) -> __failure {
    __failure __msg("off=8 kptr isn't referenced kptr")
#[no_mangle]
pub unsafe extern "C" fn reject_kptr_xchg_on_unref(ctx: *mut __sk_buff) -> c_int {
    int reject_kptr_xchg_on_unref(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub NULL): bpf_kptr_xchg(&v->unref_ptr,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=percpu_ptr_": "R1 type=rcu_ptr_or_null_) -> __failure {
    __failure __msg("R1 type=rcu_ptr_or_null_ expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn mark_ref_as_untrusted_or_null(ctx: *mut __sk_buff) -> c_int {
    int mark_ref_as_untrusted_or_null(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "store to referenced kptr) -> __failure {
    __failure __msg("store to referenced kptr disallowed")
#[no_mangle]
pub unsafe extern "C" fn reject_untrusted_store_to_ref(ctx: *mut __sk_buff) -> c_int {
    int reject_untrusted_store_to_ref(struct __sk_buff *ctx)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub v->ref_ptr: p =,
    if (!p)
    pub 0: return,
// Checkmate, clang
// (struct prog_test_ref_kfunc * volatile *)&v->ref_ptr = p;
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "release helper bpf_kptr_xchg expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release helper bpf_kptr_xchg expects referenced PTR_TO_BTF_ID passed to R2")
#[no_mangle]
pub unsafe extern "C" fn reject_untrusted_xchg(ctx: *mut __sk_buff) -> c_int {
    int reject_untrusted_xchg(struct __sk_buff *ctx)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub v->ref_ptr: p =,
    if (!p)
    pub 0: return,
    pub p): bpf_kptr_xchg(&v->ref_ptr,,
    pub 0: return,
    }
    SEC("?tc")
    __failure
    __msg("invalid kptr access, R2 type=trusted_ptr_prog_test_ref_kfunc expected=ptr_prog_test_member")
#[no_mangle]
pub unsafe extern "C" fn reject_bad_type_xchg(ctx: *mut __sk_buff) -> c_int {
    int reject_bad_type_xchg(struct __sk_buff *ctx)
    {
    pub ref_ptr: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub long){0}): ref_ptr = bpf_kfunc_call_test_acquire(&(unsigned,
    if (!ref_ptr)
    pub 0: return,
    pub ref_ptr): bpf_kptr_xchg(&v->ref_memb_ptr,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(access: "invalid kptr, type=trusted_ptr_prog_test_ref_kfunc": R2) -> __failure {
    __failure __msg("invalid kptr access, R2 type=trusted_ptr_prog_test_ref_kfunc")
#[no_mangle]
pub unsafe extern "C" fn reject_member_of_ref_xchg(ctx: *mut __sk_buff) -> c_int {
    int reject_member_of_ref_xchg(struct __sk_buff *ctx)
    {
    pub ref_ptr: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub long){0}): ref_ptr = bpf_kfunc_call_test_acquire(&(unsigned,
    if (!ref_ptr)
    pub 0: return,
    pub &ref_ptr->memb): bpf_kptr_xchg(&v->ref_memb_ptr,,
    pub 0: return,
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(helper": "kptr cannot be accessed indirectly by) -> __failure {
    __failure __msg("kptr cannot be accessed indirectly by helper")
#[no_mangle]
pub unsafe extern "C" fn reject_indirect_helper_access(ctx: *mut __sk_buff) -> c_int {
    int reject_indirect_helper_access(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub 1): bpf_get_current_comm(v, sizeof(v->buf) +,
    pub 0: return,
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn write_func(p: *mut c_int) -> c_int {
    int write_func(int *p)
    {
    pub 0: *mut *mut return p ? p = 42 :,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(helper": "kptr cannot be accessed indirectly by) -> __failure {
    __failure __msg("kptr cannot be accessed indirectly by helper")
#[no_mangle]
pub unsafe extern "C" fn reject_indirect_global_func_access(ctx: *mut __sk_buff) -> c_int {
    int reject_indirect_global_func_access(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub 5): *mut *mut return write_func((void )v +,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(alloc_insn=": "Unreleased reference id=4) -> __failure {
    __failure __msg("Unreleased reference id=4 alloc_insn=")
#[no_mangle]
pub unsafe extern "C" fn kptr_xchg_ref_state(ctx: *mut __sk_buff) -> c_int {
    int kptr_xchg_ref_state(struct __sk_buff *ctx)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub long){0}): p = bpf_kfunc_call_test_acquire(&(unsigned,
    if (!p)
    pub 0: return,
    pub p): bpf_kptr_xchg(&v->ref_ptr,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "Possibly NULL pointer passed to helper) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to helper R2")
#[no_mangle]
pub unsafe extern "C" fn kptr_xchg_possibly_null(ctx: *mut __sk_buff) -> c_int {
    int kptr_xchg_possibly_null(struct __sk_buff *ctx)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
    pub long){0}): p = bpf_kfunc_call_test_acquire(&(unsigned,
// PTR_TO_BTF_ID | PTR_MAYBE_NULL passed to bpf_kptr_xchg()
    pub p): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (p)
    pub 0: return,
    }
    SEC("?tc")
//
// A compiler with BPF_ST folds the constant into a store-immediate, which the
// verifier rejects on a different path (and with a different message) than the
// BPF_STX form.
//

#[no_mangle]
pub unsafe extern "C" fn __msg(off=8": "BPF_ST imm must be 0 when storing to kptr at) -> __failure {
    __failure __msg("BPF_ST imm must be 0 when storing to kptr at off=8")

#[no_mangle]
pub unsafe extern "C" fn __msg(access: "invalid kptr, _arg: R") -> __failure {
    __failure __msg("invalid kptr access, R")

#[no_mangle]
pub unsafe extern "C" fn reject_scalar_store_to_kptr(ctx: *mut __sk_buff) -> c_int {
    int reject_scalar_store_to_kptr(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 0: return,
// (volatile u64 *)&v->unref_ptr = 0xBADC0DE;
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
