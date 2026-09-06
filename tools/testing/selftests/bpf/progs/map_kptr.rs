//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/map_kptr.c
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
pub struct map_uninit_value {
    pub unref_ptr: *mut prog_test_ref_kfunc __kptr_untrusted,
    pub data: __u32,
    pub __attribute__((packed)): },
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub map_uninit_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } pcpu_array,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub unref_ptr: *mut prog_test_ref_kfunc __kptr_untrusted,
    pub ref_ptr: *mut prog_test_ref_kfunc __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } array_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_array_map {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } pcpu_array_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_map {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } hash_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_hash_map {
    pub BPF_MAP_TYPE_PERCPU_HASH): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } pcpu_hash_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_malloc_map {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub SEC(".maps"): } hash_malloc_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_hash_malloc_map {
    pub BPF_MAP_TYPE_PERCPU_HASH): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub SEC(".maps"): } pcpu_hash_malloc_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_hash_map {
    pub BPF_MAP_TYPE_LRU_HASH): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } lru_hash_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_pcpu_hash_map {
    pub BPF_MAP_TYPE_LRU_PERCPU_HASH): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } lru_pcpu_hash_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgrp_ls_map {
    pub BPF_MAP_TYPE_CGRP_STORAGE): __uint(type,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub SEC(".maps"): } cgrp_ls_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_ls_map {
    pub BPF_MAP_TYPE_TASK_STORAGE): __uint(type,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub SEC(".maps"): } task_ls_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_ls_map {
    pub BPF_MAP_TYPE_INODE_STORAGE): __uint(type,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub SEC(".maps"): } inode_ls_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_ls_map {
    pub BPF_MAP_TYPE_SK_STORAGE): __uint(type,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub SEC(".maps"): } sk_ls_map,

    struct {                                                \
    pub \: __uint(type, map_type);,
    pub \: __uint(max_entries, 1);,
    pub \: __uint(key_size, sizeof(int));,
    pub \: __uint(value_size, sizeof(int));,
    pub \: __array(values, struct inner_map_type);,
    } name SEC(".maps") = {                                 \
    .values = { [0] = &inner_map_type },            \
    }
    pub array_of_array_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_ARRAY_OF_MAPS, array_map,,
    pub array_of_hash_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_ARRAY_OF_MAPS, hash_map,,
    pub array_of_hash_malloc_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_ARRAY_OF_MAPS, hash_malloc_map,,
    pub array_of_lru_hash_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_ARRAY_OF_MAPS, lru_hash_map,,
    pub array_of_pcpu_array_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_ARRAY_OF_MAPS, pcpu_array_map,,
    pub array_of_pcpu_hash_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_ARRAY_OF_MAPS, pcpu_hash_map,,
    pub hash_of_array_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_HASH_OF_MAPS, array_map,,
    pub hash_of_hash_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_HASH_OF_MAPS, hash_map,,
    pub hash_of_hash_malloc_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_HASH_OF_MAPS, hash_malloc_map,,
    pub hash_of_lru_hash_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_HASH_OF_MAPS, lru_hash_map,,
    pub hash_of_pcpu_array_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_HASH_OF_MAPS, pcpu_array_map,,
    pub hash_of_pcpu_hash_maps): DEFINE_MAP_OF_MAP(BPF_MAP_TYPE_HASH_OF_MAPS, pcpu_hash_map,,

#[no_mangle]
unsafe extern "C" fn test_kptr_unref(v: *mut map_value) {
    static void test_kptr_unref(struct map_value *v)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v->unref_ptr: p =,
// store untrusted_ptr_or_null_
    pub p): WRITE_ONCE(v->unref_ptr,,
    if (!p)
    if (p.a + p.b > 100)
// store untrusted_ptr_
    pub p): WRITE_ONCE(v->unref_ptr,,
// store NULL
    pub NULL): WRITE_ONCE(v->unref_ptr,,
    }
#[no_mangle]
unsafe extern "C" fn test_kptr_ref(v: *mut map_value) {
    static void test_kptr_ref(struct map_value *v)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub v->ref_ptr: p =,
// store ptr_or_null_
    pub p): WRITE_ONCE(v->unref_ptr,,
    if (!p)
//
// p is rcu_ptr_prog_test_ref_kfunc,
// because bpf prog is non-sleepable and runs in RCU CS.
// p can be passed to kfunc that requires KF_RCU.
//
    if (p.a + p.b > 100)
// store NULL
    pub NULL): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (!p)
//
// p is trusted_ptr_prog_test_ref_kfunc.
// p can be passed to kfunc that requires KF_RCU.
//
    if (p.a + p.b > 100) {
    }
// store ptr_
    pub p): WRITE_ONCE(v->unref_ptr,,
    pub long){0}): p = bpf_kfunc_call_test_acquire(&(unsigned,
    if (!p)
// store ptr_
    pub p): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (!p)
    if (p.a + p.b > 100) {
    }
    }
#[no_mangle]
unsafe extern "C" fn test_kptr(v: *mut map_value) {
    static void test_kptr(struct map_value *v)
    {
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_map_kptr(ctx: *mut __sk_buff) -> c_int {
    int test_map_kptr(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,

    pub \: v = bpf_map_lookup_elem(&map, &key);,
    if (!v)						\
    pub \: return 0;,
    test_kptr(v)

    pub 0: return,
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_cgrp_map_kptr, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(test_cgrp_map_kptr, struct cgroup *cgrp, const char *path)
    {
    pub v: *mut map_value,
    pub BPF_LOCAL_STORAGE_GET_F_CREATE): v = bpf_cgrp_storage_get(&cgrp_ls_map, cgrp, NULL,,
    if (v)
    pub 0: return,
    }
    SEC("lsm/inode_unlink")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_task_map_kptr, inode: *mut inode, victim: *mut dentry) -> c_int {
    int BPF_PROG(test_task_map_kptr, struct inode *inode, struct dentry *victim)
    {
    pub task: *mut task_struct,
    pub v: *mut map_value,
    pub bpf_get_current_task_btf(): task =,
    if (!task)
    pub 0: return,
    pub BPF_LOCAL_STORAGE_GET_F_CREATE): v = bpf_task_storage_get(&task_ls_map, task, NULL,,
    if (v)
    pub 0: return,
    }
    SEC("lsm/inode_unlink")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_inode_map_kptr, inode: *mut inode, victim: *mut dentry) -> c_int {
    int BPF_PROG(test_inode_map_kptr, struct inode *inode, struct dentry *victim)
    {
    pub v: *mut map_value,
    pub BPF_LOCAL_STORAGE_GET_F_CREATE): v = bpf_inode_storage_get(&inode_ls_map, inode, NULL,,
    if (v)
    pub 0: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_sk_map_kptr(ctx: *mut __sk_buff) -> c_int {
    int test_sk_map_kptr(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub sk: *mut bpf_sock,
    pub ctx->sk: sk =,
    if (!sk)
    pub 0: return,
    pub BPF_LOCAL_STORAGE_GET_F_CREATE): v = bpf_sk_storage_get(&sk_ls_map, sk, NULL,,
    if (v)
    pub 0: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_map_in_map_kptr(ctx: *mut __sk_buff) -> c_int {
    int test_map_in_map_kptr(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub 0: int key =,
    pub map: *mut c_void,

    pub \: map = bpf_map_lookup_elem(&map_in_map, &key);,
    if (!map)                                       \
    pub \: return 0;,
    pub \: v = bpf_map_lookup_elem(map, &key);,
    if (!v)						\
    pub \: return 0;,
    test_kptr(v)

    pub 0: return,
    }
    pub 1: int ref =,
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn test_map_kptr_ref_pre(v: *mut map_value) -> c_int {
    int test_map_kptr_ref_pre(struct map_value *v)
    {
    pub p_st: *mut *mut prog_test_ref_kfunc p,,
    pub 0: unsigned long arg =,
    pub ret: c_int,
    pub bpf_kfunc_call_test_acquire(&arg): p =,
    if (!p)
    pub 1: return,
    pub p->next: p_st =,
    if (p_st.cnt.refs.counter != ref) {
    pub 2: ret =,
    pub end: goto,
    }
    pub p): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (p) {
    pub 3: ret =,
    pub end: goto,
    }
    if (p_st.cnt.refs.counter != ref)
    pub 4: return,
    pub NULL): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (!p)
    pub 5: return,
    if (p_st.cnt.refs.counter != ref)
    pub 6: return,
    pub bpf_kfunc_call_test_acquire(&arg): p =,
    if (!p)
    pub 7: return,
    pub p): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (p) {
    pub 8: ret =,
    pub end: goto,
    }
    if (p_st.cnt.refs.counter != ref)
    pub 9: return,
// Leave in map
    pub 0: return,
    end:
    pub ret: return,
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn test_map_kptr_ref_post(v: *mut map_value) -> c_int {
    int test_map_kptr_ref_post(struct map_value *v)
    {
    pub p_st: *mut *mut prog_test_ref_kfunc p,,
    pub v->ref_ptr: p_st =,
    if (!p_st || p_st.cnt.refs.counter != ref)
    pub 1: return,
    pub NULL): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (!p)
    pub 2: return,
    if (p_st.cnt.refs.counter != ref) {
    pub 3: return,
    }
    pub p): p = bpf_kptr_xchg(&v->ref_ptr,,
    if (p) {
    pub 4: return,
    }
    if (p_st.cnt.refs.counter != ref)
    pub 5: return,
    pub 0: return,
    }

    pub \: v = bpf_map_lookup_elem(&map, &key);,
    if (!v)                              \
    pub \: return -1;,
    pub \: ret = test_map_kptr_ref_pre(v);,
    if (ret)                             \
    pub ret: return,

    pub \: v = bpf_map_lookup_percpu_elem(&map, &key, 0);,
    if (!v)                                        \
    pub \: return -1;,
    pub \: ret = test_map_kptr_ref_pre(v);,
    if (ret)                                       \
    pub ret: return,
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_map_kptr_ref1(ctx: *mut __sk_buff) -> c_int {
    int test_map_kptr_ref1(struct __sk_buff *ctx)
    {
    pub {}: *mut *mut map_value v, val =,
    pub ret: int key = 0,,
    pub 0): bpf_map_update_elem(&hash_map, &key, &val,,
    pub 0): bpf_map_update_elem(&hash_malloc_map, &key, &val,,
    pub 0): bpf_map_update_elem(&lru_hash_map, &key, &val,,
    pub 0): bpf_map_update_elem(&pcpu_hash_map, &key, &val,,
    pub 0): bpf_map_update_elem(&pcpu_hash_malloc_map, &key, &val,,
    pub 0): bpf_map_update_elem(&lru_pcpu_hash_map, &key, &val,,
    pub 0: return,
    }

    pub \: v = bpf_map_lookup_elem(&map, &key);,
    if (!v)                              \
    pub \: return -1;,
    pub \: ret = test_map_kptr_ref_post(v);,
    if (ret)                             \
    pub ret: return,

    pub \: v = bpf_map_lookup_percpu_elem(&map, &key, 0);,
    if (!v)                                        \
    pub \: return -1;,
    pub \: ret = test_map_kptr_ref_post(v);,
    if (ret)                                       \
    pub ret: return,
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_map_kptr_ref2(ctx: *mut __sk_buff) -> c_int {
    int test_map_kptr_ref2(struct __sk_buff *ctx)
    {
    pub v: *mut map_value,
    pub ret: int key = 0,,
    pub 0: return,
    }

    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_map_kptr_ref3(ctx: *mut __sk_buff) -> c_int {
    int test_map_kptr_ref3(struct __sk_buff *ctx)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub 0: unsigned long sp =,
    pub bpf_kfunc_call_test_acquire(&sp): p =,
    if (!p)
    pub 1: return,
    if (p.cnt.refs.counter != ref) {
    pub 2: return,
    }
    pub 0: return,
    }
    pub num_of_refs: c_int,
#[no_mangle]
unsafe extern "C" fn read_ref_count() -> __always_inline int {
    static __always_inline int read_ref_count(void)
    {
    pub p: *mut prog_test_ref_kfunc,
    pub 0: unsigned long arg =,
    pub bpf_kfunc_call_test_acquire(&arg): p =,
    if (!p)
    pub 1: return,
    pub p->cnt.refs.counter: num_of_refs =,
    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn count_ref(ctx: *mut c_void) -> c_int {
    int count_ref(void *ctx)
    {
    pub read_ref_count(): return,
    }
#[no_mangle]
unsafe extern "C" fn stash_ref_ptr(v: *mut map_value) -> __always_inline int {
    static __always_inline int stash_ref_ptr(struct map_value *v)
    {
    pub old: *mut *mut prog_test_ref_kfunc p,,
    pub 0: unsigned long arg =,
    pub bpf_kfunc_call_test_acquire(&arg): p =,
    if (!p)
    pub 1: return,
    pub p): old = bpf_kptr_xchg(&v->ref_ptr,,
    if (old) {
    pub NULL): old = bpf_kptr_xchg(&v->ref_ptr,,
    if (old)
    pub 2: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn check_refs(expected: c_int) -> __always_inline int {
    static __always_inline int check_refs(int expected)
    {
    pub ret: c_int,
    pub read_ref_count(): ret =,
    if (ret)
    pub ret: return,
    pub 3: return num_of_refs == expected ? 0 :,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_array_map_update_kptr(ctx: *mut c_void) -> c_int {
    int test_array_map_update_kptr(void *ctx)
    {
    pub v: *mut map_value init = {},,
    pub ret: int key = 0,,
    pub &key): v = bpf_map_lookup_elem(&array_map,,
    if (!v)
    pub 1: return,
    pub stash_ref_ptr(v): ret =,
    if (ret)
    pub ret: return,
    pub check_refs(3): ret =,
    if (ret)
    pub ret: return,
    pub BPF_EXIST): ret = bpf_map_update_elem(&array_map, &key, &init,,
    if (ret)
    pub 4: return,
    pub check_refs(3): return,
    }

    SEC("syscall")							\
    int name(void *ctx)						\
    {								\
    pub \: *mut *mut map_value init = {}, v;,
    pub \: int key = 0, ret;,
    \
    pub \: ret = bpf_map_update_elem(&map, &key, &init, BPF_NOEXIST);,
    if (ret)						\
    pub \: return 1;,
    pub \: v = bpf_map_lookup_elem(&map, &key);,
    if (!v)							\
    pub \: return 2;,
    pub \: ret = stash_ref_ptr(v);,
    if (ret)						\
    pub \: return ret;,
    pub \: ret = check_refs(3);,
    if (ret)						\
    pub \: return ret;,
    pub \: ret = bpf_map_update_elem(&map, &key, &init, BPF_EXIST);,
    if (ret)						\
    pub \: return 4;,
    pub \: return check_refs(3);,
    }
    DEFINE_HASH_UPDATE_KPTR_TEST(test_hash_map_update_kptr, hash_map)
    DEFINE_HASH_UPDATE_KPTR_TEST(test_hash_malloc_map_update_kptr, hash_malloc_map)
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_ls_map_kptr_ref1(ctx: *mut c_void) -> c_int {
    int test_ls_map_kptr_ref1(void *ctx)
    {
    pub current: *mut task_struct,
    pub v: *mut map_value,
    pub bpf_get_current_task_btf(): current =,
    if (!current)
    pub 100: return,
    pub 0): v = bpf_task_storage_get(&task_ls_map, current, NULL,,
    if (v)
    pub 150: return,
    pub BPF_LOCAL_STORAGE_GET_F_CREATE): v = bpf_task_storage_get(&task_ls_map, current, NULL,,
    if (!v)
    pub 200: return,
    pub test_map_kptr_ref_pre(v): return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_ls_map_kptr_ref2(ctx: *mut c_void) -> c_int {
    int test_ls_map_kptr_ref2(void *ctx)
    {
    pub current: *mut task_struct,
    pub v: *mut map_value,
    pub bpf_get_current_task_btf(): current =,
    if (!current)
    pub 100: return,
    pub 0): v = bpf_task_storage_get(&task_ls_map, current, NULL,,
    if (!v)
    pub 200: return,
    pub test_map_kptr_ref_post(v): return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_ls_map_kptr_ref_del(ctx: *mut c_void) -> c_int {
    int test_ls_map_kptr_ref_del(void *ctx)
    {
    pub current: *mut task_struct,
    pub v: *mut map_value,
    pub bpf_get_current_task_btf(): current =,
    if (!current)
    pub 100: return,
    pub 0): v = bpf_task_storage_get(&task_ls_map, current, NULL,,
    if (!v)
    pub 200: return,
    if (!v.ref_ptr)
    pub 300: return,
    pub current): return bpf_task_storage_delete(&task_ls_map,,
    }
    pub "GPL": char _license[] SEC("license") =,
