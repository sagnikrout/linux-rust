//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/map_ptr_kern.c
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
// Copyright (c) 2020 Facebook

pub const LOOP_BOUND: c_uint = 0xf;
pub const MAX_ENTRIES: c_int = 8;

    _Static_assert(MAX_ENTRIES < LOOP_BOUND, "MAX_ENTRIES must be < LOOP_BOUND");
    let mut g_map_type: enum bpf_map_type = BPF_MAP_TYPE_UNSPEC;
    let mut g_line: __u32 = 0;
    int page_size = 0; /* userspace should set it */

    g_map_type = type;		\
    if (!func())			\
    return 0;		\
    })

    g_line = __LINE__;	\
    if (!(expr))		\
    return 0;	\
    })
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map {
    pub map_type: enum bpf_map_type,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub id: __u32,
    pub __attribute__((preserve_access_index)): },
    static inline int check_bpf_map_fields(struct bpf_map *map, __u32 key_size,
    __u32 value_size, __u32 max_entries)
    {
    pub g_map_type): VERIFY(map->map_type ==,
    pub key_size): VERIFY(map->key_size ==,
    pub value_size): VERIFY(map->value_size ==,
    pub max_entries): VERIFY(map->max_entries ==,
    pub 0): VERIFY(map->id >,
    pub 1: return,
    }
    static inline int check_bpf_map_ptr(struct bpf_map *indirect,
    struct bpf_map *direct)
    {
    pub direct->map_type): VERIFY(indirect->map_type ==,
    pub direct->key_size): VERIFY(indirect->key_size ==,
    pub direct->value_size): VERIFY(indirect->value_size ==,
    pub direct->max_entries): VERIFY(indirect->max_entries ==,
    pub direct->id): VERIFY(indirect->id ==,
    pub 1: return,
    }
    static inline int check(struct bpf_map *indirect, struct bpf_map *direct,
    __u32 key_size, __u32 value_size, __u32 max_entries)
    {
    pub direct)): VERIFY(check_bpf_map_ptr(indirect,,
    VERIFY(check_bpf_map_fields(indirect, key_size, value_size,
    pub 1: return,
    }
    static inline int check_default(struct bpf_map *indirect,
    struct bpf_map *direct)
    {
    VERIFY(check(indirect, direct, sizeof(__u32), sizeof(__u32),
    pub 1: return,
    }
    static __noinline int
    check_default_noinline(struct bpf_map *indirect, struct bpf_map *direct)
    {
    VERIFY(check(indirect, direct, sizeof(__u32), sizeof(__u32),
    pub 1: return,
    }
    typedef struct {
    pub counter: c_int,
    pub atomic_t: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_htab {
    pub map: bpf_map,
    pub count: core::sync::atomic::AtomicI32,
    pub n_buckets: __u32,
    pub elem_size: __u32,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub /: *mut *mut __uint(map_flags, BPF_F_NO_PREALLOC); / to test bpf_htab.count,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_hash,
    pub __ksym: *mut *mut __s64 bpf_map_sum_elem_count(struct bpf_map map),
#[no_mangle]
pub unsafe extern "C" fn check_hash() -> c_int {
    static inline int check_hash(void)
    {
    pub )&m_hash: *mut *mut bpf_htab hash = (bpf_htab,
    pub )&m_hash: *mut *mut bpf_map map = (bpf_map,
    pub i: c_int,
    pub map)): VERIFY(check_default_noinline(&hash->map,,
    pub MAX_ENTRIES): VERIFY(hash->n_buckets ==,
    pub 64): VERIFY(hash->elem_size ==,
    pub 0): VERIFY(hash->count.counter ==,
    pub 0): VERIFY(bpf_map_sum_elem_count(map) ==,
    pub {: for (i = 0; i < HALF_ENTRIES; ++i),
    pub i: __u32 key =,
    pub 1: __u32 val =,
    if (bpf_map_update_elem(hash, &key, &val, 0))
    pub 0: return,
    }
    pub HALF_ENTRIES): VERIFY(hash->count.counter ==,
    pub HALF_ENTRIES): VERIFY(bpf_map_sum_elem_count(map) ==,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_array {
    pub map: bpf_map,
    pub elem_size: __u32,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_array,
#[no_mangle]
pub unsafe extern "C" fn check_array() -> c_int {
    static inline int check_array(void)
    {
    pub )&m_array: *mut *mut bpf_array array = (bpf_array,
    pub )&m_array: *mut *mut bpf_map map = (bpf_map,
    pub 0: int i, n_lookups = 0, n_keys =,
    pub map)): VERIFY(check_default(&array->map,,
    pub 8): VERIFY(array->elem_size ==,
    pub {: for (i = 0; i < array->map.max_entries && i < LOOP_BOUND; ++i),
    pub i: __u32 key =,
    pub &key): *mut *mut __u32 val = bpf_map_lookup_elem(array,,
    if (val)
    }
    pub MAX_ENTRIES): VERIFY(n_lookups ==,
    pub MAX_ENTRIES): VERIFY(n_keys ==,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_PROG_ARRAY): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_prog_array,
#[no_mangle]
pub unsafe extern "C" fn check_prog_array() -> c_int {
    static inline int check_prog_array(void)
    {
    pub )&m_prog_array: *mut *mut bpf_array prog_array = (bpf_array,
    pub )&m_prog_array: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&prog_array->map,,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_PERF_EVENT_ARRAY): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_perf_event_array,
#[no_mangle]
pub unsafe extern "C" fn check_perf_event_array() -> c_int {
    static inline int check_perf_event_array(void)
    {
    pub )&m_perf_event_array: *mut *mut bpf_array perf_event_array = (bpf_array,
    pub )&m_perf_event_array: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&perf_event_array->map,,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_PERCPU_HASH): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_percpu_hash,
#[no_mangle]
pub unsafe extern "C" fn check_percpu_hash() -> c_int {
    static inline int check_percpu_hash(void)
    {
    pub )&m_percpu_hash: *mut *mut bpf_htab percpu_hash = (bpf_htab,
    pub )&m_percpu_hash: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&percpu_hash->map,,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_percpu_array,
#[no_mangle]
pub unsafe extern "C" fn check_percpu_array() -> c_int {
    static inline int check_percpu_array(void)
    {
    pub )&m_percpu_array: *mut *mut bpf_array percpu_array = (bpf_array,
    pub )&m_percpu_array: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&percpu_array->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stack_map {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_STACK_TRACE): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u64): __type(value,,
    pub SEC(".maps"): } m_stack_trace,
#[no_mangle]
pub unsafe extern "C" fn check_stack_trace() -> c_int {
    static inline int check_stack_trace(void)
    {
    struct bpf_stack_map *stack_trace =
    pub )&m_stack_trace: *mut (struct bpf_stack_map,
    pub )&m_stack_trace: *mut *mut bpf_map map = (bpf_map,
    VERIFY(check(&stack_trace.map, map, sizeof(__u32), sizeof(__u64),
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_CGROUP_ARRAY): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_cgroup_array,
#[no_mangle]
pub unsafe extern "C" fn check_cgroup_array() -> c_int {
    static inline int check_cgroup_array(void)
    {
    pub )&m_cgroup_array: *mut *mut bpf_array cgroup_array = (bpf_array,
    pub )&m_cgroup_array: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&cgroup_array->map,,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_LRU_HASH): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_lru_hash,
#[no_mangle]
pub unsafe extern "C" fn check_lru_hash() -> c_int {
    static inline int check_lru_hash(void)
    {
    pub )&m_lru_hash: *mut *mut bpf_htab lru_hash = (bpf_htab,
    pub )&m_lru_hash: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&lru_hash->map,,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_LRU_PERCPU_HASH): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_lru_percpu_hash,
#[no_mangle]
pub unsafe extern "C" fn check_lru_percpu_hash() -> c_int {
    static inline int check_lru_percpu_hash(void)
    {
    pub )&m_lru_percpu_hash: *mut *mut bpf_htab lru_percpu_hash = (bpf_htab,
    pub )&m_lru_percpu_hash: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&lru_percpu_hash->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpm_trie {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpm_key {
    pub trie_key: bpf_lpm_trie_key_hdr,
    pub data: __u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_LPM_TRIE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, MAX_ENTRIES);
    __type(key, struct lpm_key);
    __type(value, __u32);
    } m_lpm_trie SEC(".maps");
#[no_mangle]
pub unsafe extern "C" fn check_lpm_trie() -> c_int {
    static inline int check_lpm_trie(void)
    {
    struct lpm_trie *lpm_trie = (struct lpm_trie *)&m_lpm_trie;
    struct bpf_map *map = (struct bpf_map *)&m_lpm_trie;
    VERIFY(check(&lpm_trie.map, map, sizeof(struct lpm_key), sizeof(__u32),
    MAX_ENTRIES));
    return 1;
    }
pub const INNER_MAX_ENTRIES: c_int = 1234;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub INNER_MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } inner_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    __array(values, struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub INNER_MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    } m_array_of_maps SEC(".maps") = {
    .values = { (void *)&inner_map, 0, 0, 0, 0, 0, 0, 0, 0 },
}

#[no_mangle]
pub unsafe extern "C" fn check_array_of_maps() -> c_int {
    static inline int check_array_of_maps(void)
    {
    struct bpf_array *array_of_maps = (struct bpf_array *)&m_array_of_maps;
    struct bpf_map *map = (struct bpf_map *)&m_array_of_maps;
    struct bpf_array *inner_map;
    let mut key: c_int = 0;
    VERIFY(check_default(&array_of_maps.map, map));
    inner_map = bpf_map_lookup_elem(array_of_maps, &key);
    VERIFY(inner_map != core::ptr::null_mut());
    VERIFY(inner_map.map.max_entries == INNER_MAX_ENTRIES);
    return 1;
    }
    struct {
    __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
    __uint(max_entries, MAX_ENTRIES);
    __type(key, __u32);
    __type(value, __u32);
    __array(values, struct inner_map);
    } m_hash_of_maps SEC(".maps") = {
    .values = {
    [2] = &inner_map,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn check_hash_of_maps() -> c_int {
    static inline int check_hash_of_maps(void)
    {
    struct bpf_htab *hash_of_maps = (struct bpf_htab *)&m_hash_of_maps;
    struct bpf_map *map = (struct bpf_map *)&m_hash_of_maps;
    struct bpf_htab *inner_map;
    let mut key: c_int = 2;
    VERIFY(check_default(&hash_of_maps.map, map));
    inner_map = bpf_map_lookup_elem(hash_of_maps, &key);
    VERIFY(inner_map != core::ptr::null_mut());
    VERIFY(inner_map.map.max_entries == INNER_MAX_ENTRIES);
    return 1;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dtab {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_DEVMAP): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_devmap,
#[no_mangle]
pub unsafe extern "C" fn check_devmap() -> c_int {
    static inline int check_devmap(void)
    {
    pub )&m_devmap: *mut *mut bpf_dtab devmap = (bpf_dtab,
    pub )&m_devmap: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&devmap->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stab {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_SOCKMAP): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_sockmap,
#[no_mangle]
pub unsafe extern "C" fn check_sockmap() -> c_int {
    static inline int check_sockmap(void)
    {
    pub )&m_sockmap: *mut *mut bpf_stab sockmap = (bpf_stab,
    pub )&m_sockmap: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&sockmap->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cpu_map {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_CPUMAP): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_cpumap,
#[no_mangle]
pub unsafe extern "C" fn check_cpumap() -> c_int {
    static inline int check_cpumap(void)
    {
    pub )&m_cpumap: *mut *mut bpf_cpu_map cpumap = (bpf_cpu_map,
    pub )&m_cpumap: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&cpumap->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_map {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_XSKMAP): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_xskmap,
#[no_mangle]
pub unsafe extern "C" fn check_xskmap() -> c_int {
    static inline int check_xskmap(void)
    {
    pub )&m_xskmap: *mut *mut xsk_map xskmap = (xsk_map,
    pub )&m_xskmap: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&xskmap->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_shtab {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_SOCKHASH): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_sockhash,
#[no_mangle]
pub unsafe extern "C" fn check_sockhash() -> c_int {
    static inline int check_sockhash(void)
    {
    pub )&m_sockhash: *mut *mut bpf_shtab sockhash = (bpf_shtab,
    pub )&m_sockhash: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&sockhash->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cgroup_storage_map {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_CGROUP_STORAGE): __uint(type,,
    pub bpf_cgroup_storage_key): __type(key, struct,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_cgroup_storage,
#[no_mangle]
pub unsafe extern "C" fn check_cgroup_storage() -> c_int {
    static inline int check_cgroup_storage(void)
    {
    struct bpf_cgroup_storage_map *cgroup_storage =
    pub )&m_cgroup_storage: *mut (struct bpf_cgroup_storage_map,
    pub )&m_cgroup_storage: *mut *mut bpf_map map = (bpf_map,
    VERIFY(check(&cgroup_storage.map, map,
    pub 0)): sizeof(struct bpf_cgroup_storage_key), sizeof(__u32),,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reuseport_array {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_REUSEPORT_SOCKARRAY): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_reuseport_sockarray,
#[no_mangle]
pub unsafe extern "C" fn check_reuseport_sockarray() -> c_int {
    static inline int check_reuseport_sockarray(void)
    {
    struct reuseport_array *reuseport_sockarray =
    pub )&m_reuseport_sockarray: *mut (struct reuseport_array,
    pub )&m_reuseport_sockarray: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&reuseport_sockarray->map,,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE): __uint(type,,
    pub bpf_cgroup_storage_key): __type(key, struct,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_percpu_cgroup_storage,
#[no_mangle]
pub unsafe extern "C" fn check_percpu_cgroup_storage() -> c_int {
    static inline int check_percpu_cgroup_storage(void)
    {
    struct bpf_cgroup_storage_map *percpu_cgroup_storage =
    pub )&m_percpu_cgroup_storage: *mut (struct bpf_cgroup_storage_map,
    pub )&m_percpu_cgroup_storage: *mut *mut bpf_map map = (bpf_map,
    VERIFY(check(&percpu_cgroup_storage.map, map,
    pub 0)): sizeof(struct bpf_cgroup_storage_key), sizeof(__u32),,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_queue_stack {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_QUEUE): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_queue,
#[no_mangle]
pub unsafe extern "C" fn check_queue() -> c_int {
    static inline int check_queue(void)
    {
    pub )&m_queue: *mut *mut bpf_queue_stack queue = (bpf_queue_stack,
    pub )&m_queue: *mut *mut bpf_map map = (bpf_map,
    pub MAX_ENTRIES)): VERIFY(check(&queue->map, map, 0, sizeof(__u32),,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_STACK): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_stack,
#[no_mangle]
pub unsafe extern "C" fn check_stack() -> c_int {
    static inline int check_stack(void)
    {
    pub )&m_stack: *mut *mut bpf_queue_stack stack = (bpf_queue_stack,
    pub )&m_stack: *mut *mut bpf_map map = (bpf_map,
    pub MAX_ENTRIES)): VERIFY(check(&stack->map, map, 0, sizeof(__u32),,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage_map {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_SK_STORAGE): __uint(type,,
    pub BPF_F_NO_PREALLOC): __uint(map_flags,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_sk_storage,
#[no_mangle]
pub unsafe extern "C" fn check_sk_storage() -> c_int {
    static inline int check_sk_storage(void)
    {
    struct bpf_local_storage_map *sk_storage =
    pub )&m_sk_storage: *mut (struct bpf_local_storage_map,
    pub )&m_sk_storage: *mut *mut bpf_map map = (bpf_map,
    pub 0)): VERIFY(check(&sk_storage->map, map, sizeof(__u32), sizeof(__u32),,
    pub 1: return,
    }
    struct {
    pub BPF_MAP_TYPE_DEVMAP_HASH): __uint(type,,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } m_devmap_hash,
#[no_mangle]
pub unsafe extern "C" fn check_devmap_hash() -> c_int {
    static inline int check_devmap_hash(void)
    {
    pub )&m_devmap_hash: *mut *mut bpf_dtab devmap_hash = (bpf_dtab,
    pub )&m_devmap_hash: *mut *mut bpf_map map = (bpf_map,
    pub map)): VERIFY(check_default(&devmap_hash->map,,
    pub 1: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ringbuf {
    pub consumer_pos: c_ulong,
    pub producer_pos: c_ulong,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ringbuf_map {
    pub map: bpf_map,
    pub rb: *mut bpf_ringbuf,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_RINGBUF): __uint(type,,
    pub SEC(".maps"): } m_ringbuf,
#[no_mangle]
pub unsafe extern "C" fn check_ringbuf() -> c_int {
    static inline int check_ringbuf(void)
    {
    pub )&m_ringbuf: *mut *mut bpf_ringbuf_map ringbuf = (bpf_ringbuf_map,
    pub )&m_ringbuf: *mut *mut bpf_map map = (bpf_map,
    pub rb: *mut bpf_ringbuf,
    pub ptr: *mut c_void,
    pub page_size)): VERIFY(check(&ringbuf->map, map, 0, 0,,
    pub 0): ptr = bpf_ringbuf_reserve(&m_ringbuf, 128,,
    pub 0): bpf_ringbuf_discard(ptr,,
    pub ringbuf->rb: rb =,
    pub 0): VERIFY(rb->consumer_pos ==,
    pub BPF_RINGBUF_HDR_SZ): VERIFY(rb->producer_pos == 128 +,
    pub 1: return,
    }
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn cg_skb(ctx: *mut c_void) -> c_int {
    int cg_skb(void *ctx)
    {
    pub check_hash): VERIFY_TYPE(BPF_MAP_TYPE_HASH,,
    pub check_array): VERIFY_TYPE(BPF_MAP_TYPE_ARRAY,,
    pub check_prog_array): VERIFY_TYPE(BPF_MAP_TYPE_PROG_ARRAY,,
    pub check_perf_event_array): VERIFY_TYPE(BPF_MAP_TYPE_PERF_EVENT_ARRAY,,
    pub check_percpu_hash): VERIFY_TYPE(BPF_MAP_TYPE_PERCPU_HASH,,
    pub check_percpu_array): VERIFY_TYPE(BPF_MAP_TYPE_PERCPU_ARRAY,,
    pub check_stack_trace): VERIFY_TYPE(BPF_MAP_TYPE_STACK_TRACE,,
    pub check_cgroup_array): VERIFY_TYPE(BPF_MAP_TYPE_CGROUP_ARRAY,,
    pub check_lru_hash): VERIFY_TYPE(BPF_MAP_TYPE_LRU_HASH,,
    pub check_lru_percpu_hash): VERIFY_TYPE(BPF_MAP_TYPE_LRU_PERCPU_HASH,,
    pub check_lpm_trie): VERIFY_TYPE(BPF_MAP_TYPE_LPM_TRIE,,
    pub check_array_of_maps): VERIFY_TYPE(BPF_MAP_TYPE_ARRAY_OF_MAPS,,
    pub check_hash_of_maps): VERIFY_TYPE(BPF_MAP_TYPE_HASH_OF_MAPS,,
    pub check_devmap): VERIFY_TYPE(BPF_MAP_TYPE_DEVMAP,,
    pub check_sockmap): VERIFY_TYPE(BPF_MAP_TYPE_SOCKMAP,,
    pub check_cpumap): VERIFY_TYPE(BPF_MAP_TYPE_CPUMAP,,
    pub check_xskmap): VERIFY_TYPE(BPF_MAP_TYPE_XSKMAP,,
    pub check_sockhash): VERIFY_TYPE(BPF_MAP_TYPE_SOCKHASH,,
    pub check_cgroup_storage): VERIFY_TYPE(BPF_MAP_TYPE_CGROUP_STORAGE,,
    VERIFY_TYPE(BPF_MAP_TYPE_REUSEPORT_SOCKARRAY,
    VERIFY_TYPE(BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE,
    pub check_queue): VERIFY_TYPE(BPF_MAP_TYPE_QUEUE,,
    pub check_stack): VERIFY_TYPE(BPF_MAP_TYPE_STACK,,
    pub check_sk_storage): VERIFY_TYPE(BPF_MAP_TYPE_SK_STORAGE,,
    pub check_devmap_hash): VERIFY_TYPE(BPF_MAP_TYPE_DEVMAP_HASH,,
    pub check_ringbuf): VERIFY_TYPE(BPF_MAP_TYPE_RINGBUF,,
    pub 1: return,
    }
    pub "GPL": char _license[] SEC("license") =,
