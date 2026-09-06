//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_spin_lock_fail.c
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
pub struct foo {
    pub lock: bpf_spin_lock,
    pub data: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub foo): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } array_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub array_map): __array(values, struct,
    } map_of_maps SEC(".maps") = {
    .values = {
    [0] = &array_map,
    },
}

    static struct bpf_spin_lock lockA SEC(".data.A");
    static struct bpf_spin_lock lockB SEC(".data.B");
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_kptr_preserve(ctx: *mut c_void) -> c_int {
    int lock_id_kptr_preserve(void *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    bpf_this_cpu_ptr(f);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_global_zero(ctx: *mut c_void) -> c_int {
    int lock_id_global_zero(void *ctx)
    {
    bpf_this_cpu_ptr(&lockA);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_mapval_preserve(ctx: *mut c_void) -> c_int {
    int lock_id_mapval_preserve(void *ctx)
    {
    struct foo *f;
    let mut key: c_int = 0;
    f = bpf_map_lookup_elem(&array_map, &key);
    if (!f)
    return 0;
    bpf_this_cpu_ptr(f);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_innermapval_preserve(ctx: *mut c_void) -> c_int {
    int lock_id_innermapval_preserve(void *ctx)
    {
    struct foo *f;
    let mut key: c_int = 0;
    void *map;
    map = bpf_map_lookup_elem(&map_of_maps, &key);
    if (!map)
    return 0;
    f = bpf_map_lookup_elem(map, &key);
    if (!f)
    return 0;
    bpf_this_cpu_ptr(f);
    return 0;
    }

    SEC("?tc")                                             \
    int lock_id_mismatch_##test(void *ctx)                 \
    {                                                      \
    struct foo *f1, *f2, *v, *iv;                  \
    int key = 0;                                   \
    void *map;                                     \
    \
    map = bpf_map_lookup_elem(&map_of_maps, &key); \
    if (!map)                                      \
    return 0;                              \
    iv = bpf_map_lookup_elem(map, &key);           \
    if (!iv)                                       \
    return 0;                              \
    v = bpf_map_lookup_elem(&array_map, &key);     \
    if (!v)                                        \
    return 0;                              \
    f1 = bpf_obj_new(typeof(*f1));                 \
    if (!f1)                                       \
    return 0;                              \
    f2 = bpf_obj_new(typeof(*f2));                 \
    if (!f2) {                                     \
    bpf_obj_drop(f1);                      \
    return 0;                              \
    }                                              \
    bpf_spin_lock(A);                              \
    bpf_spin_unlock(B);                            \
    return 0;                                      \
    }
    CHECK(kptr_kptr, &f1.lock, &f2.lock);
    CHECK(kptr_global, &f1.lock, &lockA);
    CHECK(kptr_mapval, &f1.lock, &v.lock);
    CHECK(kptr_innermapval, &f1.lock, &iv.lock);
    CHECK(global_global, &lockA, &lockB);
    CHECK(global_kptr, &lockA, &f1.lock);
    CHECK(global_mapval, &lockA, &v.lock);
    CHECK(global_innermapval, &lockA, &iv.lock);
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_mismatch_mapval_mapval(ctx: *mut c_void) -> c_int {
    int lock_id_mismatch_mapval_mapval(void *ctx)
    {
    struct foo *f1, *f2;
    let mut key: c_int = 0;
    f1 = bpf_map_lookup_elem(&array_map, &key);
    if (!f1)
    return 0;
    f2 = bpf_map_lookup_elem(&array_map, &key);
    if (!f2)
    return 0;
    bpf_spin_lock(&f1.lock);
    f1.data = 42;
    bpf_spin_unlock(&f2.lock);
    return 0;
    }
    CHECK(mapval_kptr, &v.lock, &f1.lock);
    CHECK(mapval_global, &v.lock, &lockB);
    CHECK(mapval_innermapval, &v.lock, &iv.lock);
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_mismatch_innermapval_innermapval1(ctx: *mut c_void) -> c_int {
    int lock_id_mismatch_innermapval_innermapval1(void *ctx)
    {
    struct foo *f1, *f2;
    let mut key: c_int = 0;
    void *map;
    map = bpf_map_lookup_elem(&map_of_maps, &key);
    if (!map)
    return 0;
    f1 = bpf_map_lookup_elem(map, &key);
    if (!f1)
    return 0;
    f2 = bpf_map_lookup_elem(map, &key);
    if (!f2)
    return 0;
    bpf_spin_lock(&f1.lock);
    f1.data = 42;
    bpf_spin_unlock(&f2.lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_id_mismatch_innermapval_innermapval2(ctx: *mut c_void) -> c_int {
    int lock_id_mismatch_innermapval_innermapval2(void *ctx)
    {
    struct foo *f1, *f2;
    let mut key: c_int = 0;
    void *map;
    map = bpf_map_lookup_elem(&map_of_maps, &key);
    if (!map)
    return 0;
    f1 = bpf_map_lookup_elem(map, &key);
    if (!f1)
    return 0;
    map = bpf_map_lookup_elem(&map_of_maps, &key);
    if (!map)
    return 0;
    f2 = bpf_map_lookup_elem(map, &key);
    if (!f2)
    return 0;
    bpf_spin_lock(&f1.lock);
    f1.data = 42;
    bpf_spin_unlock(&f2.lock);
    return 0;
    }
    CHECK(innermapval_kptr, &iv.lock, &f1.lock);
    CHECK(innermapval_global, &iv.lock, &lockA);
    CHECK(innermapval_mapval, &iv.lock, &v.lock);

    __noinline
#[no_mangle]
pub unsafe extern "C" fn global_subprog(ctx: *mut __sk_buff) -> c_int {
    int global_subprog(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    if (ctx.protocol)
    ret += ctx.protocol;
    return ret + ctx.mark;
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog_call_global(ctx: *mut __sk_buff) -> c_int {
    static int static_subprog_call_global(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    if (ctx.protocol)
    return ret;
    return ret + ctx.len + global_subprog(ctx);
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_global_subprog_call1(ctx: *mut __sk_buff) -> c_int {
    int lock_global_subprog_call1(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    if (ctx.mark == 42)
    ret = global_subprog(ctx);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lock_global_subprog_call2(ctx: *mut __sk_buff) -> c_int {
    int lock_global_subprog_call2(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    if (ctx.mark == 42)
    ret = static_subprog_call_global(ctx);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    int __noinline
    global_subprog_int(int i)
    {
    if (i)
    bpf_printk("%p", &i);
    return i;
    }
    int __noinline
    global_sleepable_helper_subprog(int i)
    {
    if (i)
    bpf_copy_from_user(&i, sizeof(i), core::ptr::null_mut());
    return i;
    }
    int __noinline
    global_sleepable_kfunc_subprog(int i)
    {
    if (i)
    bpf_copy_from_user_str(&i, sizeof(i), core::ptr::null_mut(), 0);
    global_subprog_int(i);
    return i;
    }
    int __noinline
    global_subprog_calling_sleepable_global(int i)
    {
    if (!i)
    global_sleepable_kfunc_subprog(i);
    return i;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn lock_global_sleepable_helper_subprog(ctx: *mut __sk_buff) -> c_int {
    int lock_global_sleepable_helper_subprog(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    if (ctx.mark == 42)
    ret = global_sleepable_helper_subprog(ctx.mark);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn lock_global_sleepable_kfunc_subprog(ctx: *mut __sk_buff) -> c_int {
    int lock_global_sleepable_kfunc_subprog(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    if (ctx.mark == 42)
    ret = global_sleepable_kfunc_subprog(ctx.mark);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn lock_global_sleepable_subprog_indirect(ctx: *mut __sk_buff) -> c_int {
    int lock_global_sleepable_subprog_indirect(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    if (ctx.mark == 42)
    ret = global_subprog_calling_sleepable_global(ctx.mark);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    char _license[] SEC("license") = "GPL";
