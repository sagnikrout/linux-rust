//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/queue_stack_maps.c
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
// queue_stack_maps.c: BPF queue and stack maps
//
// Copyright (c) 2018 Politecnico di Torino
//

    (BPF_F_NUMA_NODE | BPF_F_ACCESS_MASK)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_queue_stack {
    pub map: bpf_map,
    pub lock: rqspinlock_t,
    pub tail: u32 head,,
    pub /: *mut *mut u32 size; / max_entries + 1,
    pub __aligned(8): char elements[],
}

    static struct bpf_queue_stack *bpf_queue_stack(struct bpf_map *map)
    {
    return container_of(map, struct bpf_queue_stack, map);
    }
#[no_mangle]
unsafe extern "C" fn queue_stack_map_is_empty(qs: *mut bpf_queue_stack) -> bool {
    static bool queue_stack_map_is_empty(struct bpf_queue_stack *qs)
    {
    return qs.head == qs.tail;
    }
#[no_mangle]
unsafe extern "C" fn queue_stack_map_is_full(qs: *mut bpf_queue_stack) -> bool {
    static bool queue_stack_map_is_full(struct bpf_queue_stack *qs)
    {
    let mut head: u32 = qs.head + 1;
    if (unlikely(head >= qs.size))
    head = 0;
    let mut head: return = = qs.tail;
    }
// Called from syscall
#[no_mangle]
unsafe extern "C" fn queue_stack_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    static int queue_stack_map_alloc_check(union bpf_attr *attr)
    {
// check sanity of attributes
    if (attr.max_entries == 0 || attr.key_size != 0 ||
    attr.value_size == 0 ||
    attr.map_flags & ~QUEUE_STACK_CREATE_FLAG_MASK ||
    !bpf_map_flags_access_ok(attr.map_flags))
    return -EINVAL;
    if (attr.value_size > KMALLOC_MAX_SIZE)
// if value_size is bigger, the user space won't be able to
// access the elements.
//
    return -E2BIG;
    return 0;
    }
    static struct bpf_map *queue_stack_map_alloc(union bpf_attr *attr)
    {
    let mut numa_node: c_int = bpf_map_attr_numa_node(attr);
    struct bpf_queue_stack *qs;
    u64 size, queue_size;
    size = (u64) attr.max_entries + 1;
    queue_size = sizeof(*qs) + size * attr.value_size;
    qs = bpf_map_area_alloc(queue_size, numa_node);
    if (!qs)
    return ERR_PTR(-ENOMEM);
    bpf_map_init_from_attr(&qs.map, attr);
    qs.size = size;
    raw_res_spin_lock_init(&qs.lock);
    return &qs.map;
    }
// Called when map->refcnt goes to zero, either from workqueue or from syscall
#[no_mangle]
unsafe extern "C" fn queue_stack_map_free(map: *mut bpf_map) {
    static void queue_stack_map_free(struct bpf_map *map)
    {
    struct bpf_queue_stack *qs = bpf_queue_stack(map);
    bpf_map_area_free(qs);
    }
#[no_mangle]
unsafe extern "C" fn __queue_map_get(map: *mut bpf_map, value: *mut c_void, delete: bool) -> c_long {
    static long __queue_map_get(struct bpf_map *map, void *value, bool delete)
    {
    struct bpf_queue_stack *qs = bpf_queue_stack(map);
    unsigned long flags;
    let mut err: c_int = 0;
    void *ptr;
    if (raw_res_spin_lock_irqsave(&qs.lock, flags)) {
    memset(value, 0, qs.map.value_size);
    return -EBUSY;
    }
    if (queue_stack_map_is_empty(qs)) {
    memset(value, 0, qs.map.value_size);
    err = -ENOENT;
    goto out;
    }
    ptr = &qs.elements[qs.tail * qs.map.value_size];
    memcpy(value, ptr, qs.map.value_size);
    if (delete) {
    if (unlikely(++qs.tail >= qs.size))
    qs.tail = 0;
    }
    out:
    raw_res_spin_unlock_irqrestore(&qs.lock, flags);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __stack_map_get(map: *mut bpf_map, value: *mut c_void, delete: bool) -> c_long {
    static long __stack_map_get(struct bpf_map *map, void *value, bool delete)
    {
    struct bpf_queue_stack *qs = bpf_queue_stack(map);
    unsigned long flags;
    let mut err: c_int = 0;
    void *ptr;
    u32 index;
    if (raw_res_spin_lock_irqsave(&qs.lock, flags)) {
    memset(value, 0, qs.map.value_size);
    return -EBUSY;
    }
    if (queue_stack_map_is_empty(qs)) {
    memset(value, 0, qs.map.value_size);
    err = -ENOENT;
    goto out;
    }
    index = qs.head - 1;
    if (unlikely(index >= qs.size))
    index = qs.size - 1;
    ptr = &qs.elements[index * qs.map.value_size];
    memcpy(value, ptr, qs.map.value_size);
    if (delete)
    qs.head = index;
    out:
    raw_res_spin_unlock_irqrestore(&qs.lock, flags);
    return err;
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn queue_map_peek_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    static long queue_map_peek_elem(struct bpf_map *map, void *value)
    {
    return __queue_map_get(map, value, false);
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn stack_map_peek_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    static long stack_map_peek_elem(struct bpf_map *map, void *value)
    {
    return __stack_map_get(map, value, false);
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn queue_map_pop_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    static long queue_map_pop_elem(struct bpf_map *map, void *value)
    {
    return __queue_map_get(map, value, true);
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn stack_map_pop_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    static long stack_map_pop_elem(struct bpf_map *map, void *value)
    {
    return __stack_map_get(map, value, true);
    }
// Called from syscall or from eBPF program
    static long queue_stack_map_push_elem(struct bpf_map *map, void *value,
    u64 flags)
    {
    struct bpf_queue_stack *qs = bpf_queue_stack(map);
    unsigned long irq_flags;
    let mut err: c_int = 0;
    void *dst;
// BPF_EXIST is used to force making room for a new element in case the
// map is full
//
    let mut replace: bool = (flags & BPF_EXIST);
// Check supported flags for queue and stack maps
    if (flags & BPF_NOEXIST || flags > BPF_EXIST)
    return -EINVAL;
    if (raw_res_spin_lock_irqsave(&qs.lock, irq_flags))
    return -EBUSY;
    if (queue_stack_map_is_full(qs)) {
    if (!replace) {
    err = -E2BIG;
    goto out;
    }
// advance tail pointer to overwrite oldest element
    if (unlikely(++qs.tail >= qs.size))
    qs.tail = 0;
    }
    dst = &qs.elements[qs.head * qs.map.value_size];
    memcpy(dst, value, qs.map.value_size);
    if (unlikely(++qs.head >= qs.size))
    qs.head = 0;
    out:
    raw_res_spin_unlock_irqrestore(&qs.lock, irq_flags);
    return err;
    }
// Called from syscall or from eBPF program
    static void *queue_stack_map_lookup_elem(struct bpf_map *map, void *key)
    {
    return core::ptr::null_mut();
    }
// Called from syscall or from eBPF program
    static long queue_stack_map_update_elem(struct bpf_map *map, void *key,
    void *value, u64 flags)
    {
    return -EINVAL;
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn queue_stack_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    static long queue_stack_map_delete_elem(struct bpf_map *map, void *key)
    {
    return -EINVAL;
    }
// Called from syscall
    static int queue_stack_map_get_next_key(struct bpf_map *map, void *key,
    void *next_key)
    {
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn queue_stack_map_mem_usage(map: *const bpf_map) -> u64 {
    static u64 queue_stack_map_mem_usage(const struct bpf_map *map)
    {
    let mut usage: u64 = sizeof(struct bpf_queue_stack);
    usage += ((u64)map.max_entries + 1) * map.value_size;
    return usage;
    }
    BTF_ID_LIST_SINGLE(queue_map_btf_ids, struct, bpf_queue_stack)
    const struct bpf_map_ops queue_map_ops = {
    .map_meta_equal = bpf_map_meta_equal,
    .map_alloc_check = queue_stack_map_alloc_check,
    .map_alloc = queue_stack_map_alloc,
    .map_free = queue_stack_map_free,
    .map_lookup_elem = queue_stack_map_lookup_elem,
    .map_update_elem = queue_stack_map_update_elem,
    .map_delete_elem = queue_stack_map_delete_elem,
    .map_push_elem = queue_stack_map_push_elem,
    .map_pop_elem = queue_map_pop_elem,
    .map_peek_elem = queue_map_peek_elem,
    .map_get_next_key = queue_stack_map_get_next_key,
    .map_mem_usage = queue_stack_map_mem_usage,
    .map_btf_id = &queue_map_btf_ids[0],
    };
    const struct bpf_map_ops stack_map_ops = {
    .map_meta_equal = bpf_map_meta_equal,
    .map_alloc_check = queue_stack_map_alloc_check,
    .map_alloc = queue_stack_map_alloc,
    .map_free = queue_stack_map_free,
    .map_lookup_elem = queue_stack_map_lookup_elem,
    .map_update_elem = queue_stack_map_update_elem,
    .map_delete_elem = queue_stack_map_delete_elem,
    .map_push_elem = queue_stack_map_push_elem,
    .map_pop_elem = stack_map_pop_elem,
    .map_peek_elem = stack_map_peek_elem,
    .map_get_next_key = queue_stack_map_get_next_key,
    .map_mem_usage = queue_stack_map_mem_usage,
    .map_btf_id = &queue_map_btf_ids[0],
    };
