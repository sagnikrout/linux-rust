//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/percpu_alloc_fail.c
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_t {
    pub d: long b, c,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct val2_t {
    pub b: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_with_ptr_t {
    pub p: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_with_rb_root_t {
    pub lock: bpf_spin_lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_600b_t {
    pub b: [c_char; 600],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub sum: c_long,
    pub pc: *mut val_t __percpu_kptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } array SEC(".maps");
    long ret;
    SEC("?fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "store to referenced kptr) -> __failure {
    __failure __msg("store to referenced kptr disallowed")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_1) -> c_int {
    int BPF_PROG(test_array_map_1)
    {
    struct val_t __percpu_kptr *p;
    struct elem *e;
    let mut index: c_int = 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    p = bpf_percpu_obj_new(struct val_t);
    if (!p)
    return 0;
    p = bpf_kptr_xchg(&e.pc, p);
    if (p)
    bpf_percpu_obj_drop(p);
    e.pc = (struct val_t __percpu_kptr *)ret;
    return 0;
    }
    SEC("?fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(access: "invalid kptr, expected=ptr_val_t": R2 type=percpu_ptr_val2_t) -> __failure {
    __failure __msg("invalid kptr access, R2 type=percpu_ptr_val2_t expected=ptr_val_t")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_2) -> c_int {
    int BPF_PROG(test_array_map_2)
    {
    struct val2_t __percpu_kptr *p2;
    struct val_t __percpu_kptr *p;
    struct elem *e;
    let mut index: c_int = 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    p2 = bpf_percpu_obj_new(struct val2_t);
    if (!p2)
    return 0;
    p = bpf_kptr_xchg(&e.pc, p2);
    if (p)
    bpf_percpu_obj_drop(p);
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=percpu_ptr_: "R1 type=scalar, _arg: percpu_rcu_ptr_, _arg: percpu_trusted_ptr_") -> __failure {
    __failure __msg("R1 type=scalar expected=percpu_ptr_, percpu_rcu_ptr_, percpu_trusted_ptr_")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_3) -> c_int {
    int BPF_PROG(test_array_map_3)
    {
    struct val_t __percpu_kptr *p, *p1;
    struct val_t *v;
    struct elem *e;
    let mut index: c_int = 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    p = bpf_percpu_obj_new(struct val_t);
    if (!p)
    return 0;
    p1 = bpf_kptr_xchg(&e.pc, p);
    if (p1)
    bpf_percpu_obj_drop(p1);
    v = bpf_this_cpu_ptr(p);
    ret = v.b;
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_percpu_obj_drop()": "R1 expected for) -> __failure {
    __failure __msg("R1 expected for bpf_percpu_obj_drop()")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_4) -> c_int {
    int BPF_PROG(test_array_map_4)
    {
    struct val_t __percpu_kptr *p;
    p = bpf_percpu_obj_new(struct val_t);
    if (!p)
    return 0;
    bpf_obj_drop(p);
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_obj_drop()": "R1 expected for) -> __failure {
    __failure __msg("R1 expected for bpf_obj_drop()")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_5) -> c_int {
    int BPF_PROG(test_array_map_5)
    {
    struct val_t *p;
    p = bpf_obj_new(struct val_t);
    if (!p)
    return 0;
    bpf_percpu_obj_drop(p);
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalars": "bpf_percpu_obj_new type ID argument must be of a struct of) -> __failure {
    __failure __msg("bpf_percpu_obj_new type ID argument must be of a struct of scalars")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_6) -> c_int {
    int BPF_PROG(test_array_map_6)
    {
    struct val_with_ptr_t __percpu_kptr *p;
    p = bpf_percpu_obj_new(struct val_with_ptr_t);
    if (!p)
    return 0;
    bpf_percpu_obj_drop(p);
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(fields": "bpf_percpu_obj_new type ID argument must not contain special) -> __failure {
    __failure __msg("bpf_percpu_obj_new type ID argument must not contain special fields")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_7) -> c_int {
    int BPF_PROG(test_array_map_7)
    {
    struct val_with_rb_root_t __percpu_kptr *p;
    p = bpf_percpu_obj_new(struct val_with_rb_root_t);
    if (!p)
    return 0;
    bpf_percpu_obj_drop(p);
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(512": "bpf_percpu_obj_new type size (600) is greater than) -> __failure {
    __failure __msg("bpf_percpu_obj_new type size (600) is greater than 512")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_8) -> c_int {
    int BPF_PROG(test_array_map_8)
    {
    struct val_600b_t __percpu_kptr *p;
    p = bpf_percpu_obj_new(struct val_600b_t);
    if (!p)
    return 0;
    bpf_percpu_obj_drop(p);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
