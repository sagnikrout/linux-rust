//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/stackmap.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2016 Facebook
//

    (BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY |	\
    BPF_F_STACK_BUILD_ID)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_bucket {
    pub fnode: pcpu_freelist_node,
    pub hash: u32,
    pub nr: u32,
    pub data: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stack_map {
    pub map: bpf_map,
    pub elems: *mut c_void,
    pub freelist: pcpu_freelist,
    pub n_buckets: u32,
    pub __counted_by(n_buckets): *mut *mut stack_map_bucket buckets[],
}

#[no_mangle]
pub unsafe extern "C" fn stack_map_use_build_id(map: *mut bpf_map) -> bool {
    static inline bool stack_map_use_build_id(struct bpf_map *map)
    {
    return (map.map_flags & BPF_F_STACK_BUILD_ID);
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_data_size(map: *mut bpf_map) -> c_int {
    static inline int stack_map_data_size(struct bpf_map *map)
    {
    return stack_map_use_build_id(map) ?
    sizeof(struct bpf_stack_build_id) : sizeof(u64);
    }
//
// stack_map_calculate_max_depth - Calculate maximum allowed stack trace depth
// @size:  Size of the buffer/map value in bytes
// @elem_size:  Size of each stack trace element
// @flags:  BPF stack trace flags (BPF_F_USER_STACK, BPF_F_USER_BUILD_ID, ...)
//
// Return: Maximum number of stack trace entries that can be safely stored
//
#[no_mangle]
unsafe extern "C" fn stack_map_calculate_max_depth(size: u32, elem_size: u32, flags: u64) -> u32 {
    static u32 stack_map_calculate_max_depth(u32 size, u32 elem_size, u64 flags)
    {
    let mut skip: u32 = flags & BPF_F_SKIP_FIELD_MASK;
    u32 max_depth;
    let mut curr_sysctl_max_stack: u32 = READ_ONCE(sysctl_perf_event_max_stack);
    max_depth = size / elem_size;
    max_depth += skip;
    if (max_depth > curr_sysctl_max_stack)
    return curr_sysctl_max_stack;
    return max_depth;
    }
#[no_mangle]
unsafe extern "C" fn prealloc_elems_and_freelist(smap: *mut bpf_stack_map) -> c_int {
    static int prealloc_elems_and_freelist(struct bpf_stack_map *smap)
    {
    u64 elem_size = sizeof(struct stack_map_bucket) +
    (u64)smap.map.value_size;
    int err;
    smap.elems = bpf_map_area_alloc(elem_size * smap.map.max_entries,
    smap.map.numa_node);
    if (!smap.elems)
    return -ENOMEM;
    err = pcpu_freelist_init(&smap.freelist);
    if (err)
    goto free_elems;
    pcpu_freelist_populate(&smap.freelist, smap.elems, elem_size,
    smap.map.max_entries);
    return 0;
    free_elems:
    bpf_map_area_free(smap.elems);
    return err;
    }
// Called from syscall
    static struct bpf_map *stack_map_alloc(union bpf_attr *attr)
    {
    let mut value_size: u32 = attr.value_size;
    struct bpf_stack_map *smap;
    u64 cost, n_buckets;
    int err;
    if (attr.map_flags & ~STACK_CREATE_FLAG_MASK)
    return ERR_PTR(-EINVAL);
// check sanity of attributes
    if (attr.max_entries == 0 || attr.key_size != 4 ||
    value_size < 8 || value_size % 8)
    return ERR_PTR(-EINVAL);
    BUILD_BUG_ON(sizeof(struct bpf_stack_build_id) % sizeof(u64));
    if (attr.map_flags & BPF_F_STACK_BUILD_ID) {
    if (value_size % sizeof(struct bpf_stack_build_id) ||
    value_size / sizeof(struct bpf_stack_build_id)
    > sysctl_perf_event_max_stack)
    return ERR_PTR(-EINVAL);
    } else if (value_size / 8 > sysctl_perf_event_max_stack)
    return ERR_PTR(-EINVAL);
// hash table size must be power of 2; roundup_pow_of_two() can overflow
// into UB on 32-bit arches, so check that first
//
    if (attr.max_entries > 1UL << 31)
    return ERR_PTR(-E2BIG);
    n_buckets = roundup_pow_of_two(attr.max_entries);
    cost = n_buckets * sizeof(struct stack_map_bucket *) + sizeof(*smap);
    smap = bpf_map_area_alloc(cost, bpf_map_attr_numa_node(attr));
    if (!smap)
    return ERR_PTR(-ENOMEM);
    bpf_map_init_from_attr(&smap.map, attr);
    smap.n_buckets = n_buckets;
    err = get_callchain_buffers(sysctl_perf_event_max_stack);
    if (err)
    goto free_smap;
    err = prealloc_elems_and_freelist(smap);
    if (err)
    goto put_buffers;
    return &smap.map;
    put_buffers:
    put_callchain_buffers();
    free_smap:
    bpf_map_area_free(smap);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn fetch_build_id(vma: *mut vm_area_struct, build_id: *mut c_uchar, may_fault: bool) -> c_int {
    static int fetch_build_id(struct vm_area_struct *vma, unsigned char *build_id, bool may_fault)
    {
    return may_fault ? build_id_parse(vma, build_id, core::ptr::null_mut())
    : build_id_parse_nofault(vma, build_id, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_set_ip(id: *mut bpf_stack_build_id) {
    static inline void stack_map_build_id_set_ip(struct bpf_stack_build_id *id)
    {
    id.status = BPF_STACK_BUILD_ID_IP;
    memset(id.build_id, 0, BUILD_ID_SIZE_MAX);
    }
    static inline u64 stack_map_build_id_offset(unsigned long vm_pgoff,
    unsigned long vm_start, u64 ip)
    {
    return (vm_pgoff << PAGE_SHIFT) + ip - vm_start;
    }
    static inline void stack_map_build_id_set_valid(struct bpf_stack_build_id *id,
    u64 offset,
    const unsigned char *build_id)
    {
    id.status = BPF_STACK_BUILD_ID_VALID;
    id.offset = offset;
    if (id.build_id != build_id)
    memcpy(id.build_id, build_id, BUILD_ID_SIZE_MAX);
    }
//
// A cached VMA lookup result. The range [vm_start, vm_end) is always set.
// vm_pgoff, file, build_id are set only when the build ID was resolved.
// Zero vm_end marks the slot empty. build_id aliases the id_offs[] entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_cached_vma {
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
    pub vm_pgoff: c_ulong,
    pub /: *mut *mut *mut file file; / pinned in the sleepable path; NULL otherwise,
    pub build_id: *const c_uchar,
}

//
// Per stack_map_get_build_id_offset() call cache of the last VMA with a build ID
// resolved and the last VMA with no usable build ID. Adjacent stack frames tend
// to land in the same VMA or the same backing file, so caching the last result
// of each kind lets us skip unnecessary VMA lookups and build ID parse calls.
// Keeping the two slots independent means a build-ID-less VMA doesn't evict the
// last resolved build ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_build_id_cache {
    pub resolved: stack_map_cached_vma,
    pub unresolved: stack_map_cached_vma,
}

//
// Fill @id from a cached range covering @ip. On a hit this writes @id (resolved
// range -> build ID + offset, unresolved range -> raw ip) and returns 0; on a
// miss it leaves @id untouched and returns -ENOENT.
//
    static int stack_map_build_id_set_from_cache(struct stack_map_build_id_cache *cache,
    struct bpf_stack_build_id *id, u64 ip)
    {
    unsigned long vm_start, vm_end, vm_pgoff;
    u64 offset;
    vm_start = cache.resolved.vm_start;
    vm_end = cache.resolved.vm_end;
    if (vm_end && ip >= vm_start && ip < vm_end) {
    vm_pgoff = cache.resolved.vm_pgoff;
    offset = stack_map_build_id_offset(vm_pgoff, vm_start, ip);
    stack_map_build_id_set_valid(id, offset, cache.resolved.build_id);
    return 0;
    }
    vm_start = cache.unresolved.vm_start;
    vm_end = cache.unresolved.vm_end;
    if (vm_end && ip >= vm_start && ip < vm_end) {
    stack_map_build_id_set_ip(id);
    return 0;
    }
    return -ENOENT;
    }
//
// Record @vma's build ID as the last resolved one. @file is the pinned backing
// file in the sleepable path (released when evicted), or NULL otherwise.
//
    static void stack_map_build_id_cache_set_resolved(struct stack_map_build_id_cache *cache,
    struct file *file,
    const unsigned char *build_id,
    unsigned long vm_start,
    unsigned long vm_end,
    unsigned long vm_pgoff)
    {
    if (cache.resolved.file)
    fput(cache.resolved.file);
    cache.resolved = (struct stack_map_cached_vma){
    .vm_start = vm_start,
    .vm_end = vm_end,
    .vm_pgoff = vm_pgoff,
    .file = file,
    .build_id = build_id,
    };
    }
// Record [vm_start, vm_end) as a range with no usable build ID.
    static void stack_map_build_id_cache_set_unresolved(struct stack_map_build_id_cache *cache,
    unsigned long vm_start,
    unsigned long vm_end)
    {
    cache.unresolved = (struct stack_map_cached_vma){
    .vm_start = vm_start,
    .vm_end = vm_end,
    };
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_vma_lock {
    pub vma: *mut vm_area_struct,
    pub mm: *mut mm_struct,
}

//
// Acquire a stable read-side reference on the VMA covering @ip.
//
// With CONFIG_PER_VMA_LOCK=y this returns a VMA with its per-VMA read
// lock held and mmap_lock dropped, so the caller may sleep.
//
// With CONFIG_PER_VMA_LOCK=n it returns a VMA with mmap_lock still
// held; the caller must snapshot any fields it needs and pin vm_file
// with get_file() before stack_map_unlock_vma() drops mmap_lock, as
// the VMA may be split, merged, or freed after that.
//
// Returns NULL on failure, in which case no lock is held.
//
    static struct vm_area_struct *
    stack_map_lock_vma(struct stack_map_vma_lock *lock, unsigned long ip)
    {
    struct mm_struct *mm = lock.mm;
    struct vm_area_struct *vma;
// noop under !CONFIG_PER_VMA_LOCK
    vma = lock_vma_under_rcu(mm, ip);
    if (vma) {
    lock.vma = vma;
    return vma;
    }
//
// Taking mmap_read_lock() is unsafe here, because the caller BPF
// program might already hold it, causing a deadlock.
//
    if (!mmap_read_trylock(mm))
    return core::ptr::null_mut();
    vma = vma_lookup(mm, ip);
    if (!vma) {
    mmap_read_unlock(mm);
    return core::ptr::null_mut();
    }

    if (!vma_start_read_locked(vma)) {
    mmap_read_unlock(mm);
    return core::ptr::null_mut();
    }
    mmap_read_unlock(mm);

    lock.vma = vma;
    return vma;
    }
#[no_mangle]
unsafe extern "C" fn stack_map_unlock_vma(lock: *mut stack_map_vma_lock) {
    static void stack_map_unlock_vma(struct stack_map_vma_lock *lock)
    {

    vma_end_read(lock.vma);

    mmap_read_unlock(lock.mm);

    lock.vma = core::ptr::null_mut();
    }
    static void stack_map_get_build_id_offset_sleepable(struct bpf_stack_build_id *id_offs,
    u32 trace_nr)
    {
    let mut lock: stack_map_vma_lock = { .mm = current.mm };
    let mut cache: stack_map_build_id_cache = {};
    struct stack_map_cached_vma *res = &cache.resolved;
    unsigned long vm_pgoff, vm_start, vm_end;
    struct vm_area_struct *vma;
    struct file *file;
    u64 offset;
    u64 ip;
    for (u32 i = 0; i < trace_nr; i++) {
    ip = READ_ONCE(id_offs[i].ip);
    if (!stack_map_build_id_set_from_cache(&cache, &id_offs[i], ip))
    continue;
    vma = stack_map_lock_vma(&lock, ip);
    if (!vma) {
    stack_map_build_id_set_ip(&id_offs[i]);
    continue;
    }
    vm_pgoff = vma.vm_pgoff;
    vm_start = vma.vm_start;
    vm_end = vma.vm_end;
    if (vma_is_anonymous(vma) || !vma.vm_file) {
    stack_map_unlock_vma(&lock);
    stack_map_build_id_set_ip(&id_offs[i]);
    stack_map_build_id_cache_set_unresolved(&cache, vm_start, vm_end);
    continue;
    }
    file = vma.vm_file;
    offset = stack_map_build_id_offset(vm_pgoff, vm_start, ip);
//
// Same backing file as the last resolved VMA (another mapping
// of the same ELF binary): reuse its build_id without re-parsing.
//
    if (file == res.file) {
    stack_map_unlock_vma(&lock);
    stack_map_build_id_set_valid(&id_offs[i], offset, res.build_id);
    res.vm_start = vm_start;
    res.vm_end = vm_end;
    res.vm_pgoff = vm_pgoff;
    continue;
    }
    file = get_file(file);
    stack_map_unlock_vma(&lock);
// build_id_parse_file() may block on filesystem reads
    if (build_id_parse_file(file, id_offs[i].build_id, core::ptr::null_mut())) {
    stack_map_build_id_set_ip(&id_offs[i]);
    fput(file);
    stack_map_build_id_cache_set_unresolved(&cache, vm_start, vm_end);
    continue;
    }
    stack_map_build_id_set_valid(&id_offs[i], offset, id_offs[i].build_id);
    stack_map_build_id_cache_set_resolved(&cache, file, id_offs[i].build_id,
    vm_start, vm_end, vm_pgoff);
    }
    if (res.file)
    fput(res.file);
    }
//
// Expects all id_offs[i].ip values to be set to correct initial IPs.
// They will be subsequently:
// - either adjusted in place to a file offset, if build ID fetching
// succeeds; in this case id_offs[i].build_id is set to correct build ID,
// and id_offs[i].status is set to BPF_STACK_BUILD_ID_VALID;
// - or IP will be kept intact, if build ID fetching failed; in this case
// id_offs[i].build_id is zeroed out and id_offs[i].status is set to
// BPF_STACK_BUILD_ID_IP.
//
    static void stack_map_get_build_id_offset(struct bpf_stack_build_id *id_offs,
    u32 trace_nr, bool user, bool may_fault)
    {
    struct mmap_unlock_irq_work *work;
    let mut has_user_ctx: bool = user && current && current.mm;
    let mut cache: stack_map_build_id_cache = {};
    struct vm_area_struct *vma;
    int i;
    if (may_fault && has_user_ctx) {
    stack_map_get_build_id_offset_sleepable(id_offs, trace_nr);
    return;
    }
    if (!has_user_ctx)
    goto fallback;
    work = bpf_mmap_unlock_guard_get();
    if (IS_ERR(work))
    goto fallback;
    if (!mmap_read_trylock(current.mm)) {
    bpf_mmap_unlock_guard_put(work);
    goto fallback;
    }
    for (i = 0; i < trace_nr; i++) {
    let mut ip: u64 = READ_ONCE(id_offs[i].ip);
    if (!stack_map_build_id_set_from_cache(&cache, &id_offs[i], ip))
    continue;
    vma = find_vma(current.mm, ip);
    if (!vma || vma_is_anonymous(vma) ||
    fetch_build_id(vma, id_offs[i].build_id, may_fault)) {
// per entry fall back to ips; cache build-ID-less range
    stack_map_build_id_set_ip(&id_offs[i]);
    if (vma)
    stack_map_build_id_cache_set_unresolved(&cache,
    vma.vm_start, vma.vm_end);
    continue;
    }
//
// mmap_lock is held for the whole loop, so the cached VMA
// fields stay valid; no file pinning is needed here.
//
    stack_map_build_id_set_valid(&id_offs[i],
    stack_map_build_id_offset(vma.vm_pgoff, vma.vm_start, ip),
    id_offs[i].build_id);
    stack_map_build_id_cache_set_resolved(&cache, core::ptr::null_mut(), id_offs[i].build_id,
    vma.vm_start, vma.vm_end,
    vma.vm_pgoff);
    }
    bpf_mmap_unlock_mm(work, current.mm);
    return;
    fallback:
// cannot access current->mm, fall back to ips
    for (i = 0; i < trace_nr; i++)
    stack_map_build_id_set_ip(&id_offs[i]);
    }
    static struct perf_callchain_entry *
    get_callchain_entry_for_task(struct task_struct *task, u32 max_depth)
    {

    struct perf_callchain_entry *entry;
    int rctx;
    entry = get_callchain_entry(&rctx);
    if (!entry)
    return core::ptr::null_mut();
    entry.nr = stack_trace_save_tsk(task, (unsigned long *)entry.ip,
    max_depth, 0);
// stack_trace_save_tsk() works on unsigned long array, while
// perf_callchain_entry uses u64 array. For 32-bit systems, it is
// necessary to fix this mismatch.
//
    if (__BITS_PER_LONG != 64) {
    unsigned long *from = (unsigned long *) entry.ip;
    u64 *to = entry.ip;
    int i;
// copy data from the end to avoid using extra buffer
    for (i = entry.nr - 1; i >= 0; i--)
    to[i] = (u64)(from[i]);
    }
    put_callchain_entry(rctx);
    return entry;

    return core::ptr::null_mut();

    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stackid {
    pub bucket: *mut stack_map_bucket,
    pub ips: *const u64,
    pub nr: u32,
    pub len: u32,
    pub hash: u32,
    pub id: u32,
    pub hash_matches: bool,
}

    static int stackid_init(struct stackid *stackid, struct bpf_map *map,
    const struct perf_callchain_entry *trace, u32 trace_nr, u64 flags)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    let mut skip: u32 = flags & BPF_F_SKIP_FIELD_MASK;
    u32 max_depth;
    if (trace_nr <= skip)
// skipping more than usable stack trace
    return -EFAULT;
    max_depth = stack_map_calculate_max_depth(map.value_size, stack_map_data_size(map), flags);
    stackid.nr = min_t(u32, trace_nr - skip, max_depth - skip);
    stackid.len = stackid.nr * sizeof(u64);
    stackid.ips = trace.ip + skip;
    stackid.hash = jhash2((const u32 *)stackid.ips, stackid.len / sizeof(u32), 0);
    stackid.id = stackid.hash & (smap.n_buckets - 1);
    stackid.bucket = READ_ONCE(smap.buckets[stackid.id]);
    stackid.hash_matches = stackid.bucket && stackid.bucket.hash == stackid.hash;
    return 0;
    }
    static int stackid_fastpath(struct stackid *stackid, struct bpf_map *map,
    const struct perf_callchain_entry *trace, u32 trace_nr,
    u64 flags)
    {
    int err;
    err = stackid_init(stackid, map, trace, trace_nr, flags);
    if (err)
    return err;
// fast cmp
    if (stackid.hash_matches && flags & BPF_F_FAST_STACK_CMP)
    return stackid.id;
    if (stack_map_use_build_id(map))
    return -ENOENT;
    if (stackid.hash_matches && stackid.bucket.nr == stackid.nr &&
    memcmp(stackid.bucket.data, stackid.ips, stackid.len) == 0)
    return stackid.id;
    if (stackid.bucket && !(flags & BPF_F_REUSE_STACKID))
    return -EEXIST;
    return -ENOENT;
    }
    static struct stack_map_bucket *
    stackid_new_bucket(struct stackid *stackid, struct bpf_map *map)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    struct bpf_stack_build_id *id_offs;
    struct stack_map_bucket *bucket;
    u32 i;
    bucket = (struct stack_map_bucket *) pcpu_freelist_pop(&smap.freelist);
    if (unlikely(!bucket))
    return core::ptr::null_mut();
    if (stack_map_use_build_id(map)) {
    id_offs = (struct bpf_stack_build_id *)bucket.data;
    for (i = 0; i < stackid.nr; i++)
    id_offs[i].ip = stackid.ips[i];
    } else {
    memcpy(bucket.data, stackid.ips, stackid.len);
    }
    bucket.hash = stackid.hash;
    bucket.nr = stackid.nr;
    return bucket;
    }
    static long stackid_install(struct stackid *stackid, struct bpf_map *map,
    struct stack_map_bucket *new_bucket, u64 flags)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    let mut user: bool = flags & BPF_F_USER_STACK;
    struct stack_map_bucket *old_bucket;
    u32 trace_len;
    if (stack_map_use_build_id(map)) {
    struct bpf_stack_build_id *id_offs;
    id_offs = (struct bpf_stack_build_id *)new_bucket.data;
    stack_map_get_build_id_offset(id_offs, stackid.nr, user, false /* !may_fault */);
    trace_len = stackid.nr * sizeof(struct bpf_stack_build_id);
    if (stackid.hash_matches && stackid.bucket.nr == stackid.nr &&
    memcmp(stackid.bucket.data, new_bucket.data, trace_len) == 0) {
    pcpu_freelist_push(&smap.freelist, &new_bucket.fnode);
    return stackid.id;
    }
    if (stackid.bucket && !(flags & BPF_F_REUSE_STACKID)) {
    pcpu_freelist_push(&smap.freelist, &new_bucket.fnode);
    return -EEXIST;
    }
    }
    old_bucket = xchg(&smap.buckets[stackid.id], new_bucket);
    if (old_bucket)
    pcpu_freelist_push(&smap.freelist, &old_bucket.fnode);
    return stackid.id;
    }
    BPF_CALL_3(bpf_get_stackid, struct pt_regs *, regs, struct bpf_map *, map,
    u64, flags)
    {
    let mut elem_size: u32 = stack_map_data_size(map);
    let mut user: bool = flags & BPF_F_USER_STACK;
    struct stack_map_bucket *new_bucket;
    struct perf_callchain_entry *trace;
    struct stackid stackid;
    let mut kernel: bool = !user;
    u32 max_depth;
    int err;
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_FAST_STACK_CMP | BPF_F_REUSE_STACKID)))
    return -EINVAL;
    max_depth = stack_map_calculate_max_depth(map.value_size, elem_size, flags);
    scoped_guard(preempt) {
    trace = get_perf_callchain(regs, kernel, user, max_depth,
    false, false, 0);
    if (unlikely(!trace))
// couldn't fetch the stack trace
    return -EFAULT;
    err = stackid_fastpath(&stackid, map, trace, trace.nr, flags);
    if (err != -ENOENT)
    return err;
    new_bucket = stackid_new_bucket(&stackid, map);
    if (!new_bucket)
    return -ENOMEM;
    }
    return stackid_install(&stackid, map, new_bucket, flags);
    }
    const struct bpf_func_proto bpf_get_stackid_proto = {
    .func		= bpf_get_stackid,
    .gpl_only	= true,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_CTX,
    .arg2_type	= ARG_CONST_MAP_PTR,
    .arg3_type	= ARG_ANYTHING,
    };
#[no_mangle]
unsafe extern "C" fn count_kernel_ip(trace: *const perf_callchain_entry) -> __u64 {
    static __u64 count_kernel_ip(const struct perf_callchain_entry *trace)
    {
    let mut nr_kernel: __u64 = 0;
    while (nr_kernel < trace.nr) {
    if (trace.ip[nr_kernel] == PERF_CONTEXT_USER)
    break;
    nr_kernel++;
    }
    return nr_kernel;
    }
    BPF_CALL_3(bpf_get_stackid_pe, struct bpf_perf_event_data_kern *, ctx,
    struct bpf_map *, map, u64, flags)
    {
    const struct perf_callchain_entry *trace;
    struct perf_event *event = ctx.event;
    struct stack_map_bucket *new_bucket;
    struct stackid stackid;
    bool kernel, user;
    __u64 nr_kernel;
    u32 trace_nr;
    int ret;
// perf_sample_data doesn't have callchain, use bpf_get_stackid
    if (!(event.attr.sample_type & PERF_SAMPLE_CALLCHAIN))
    return bpf_get_stackid((unsigned long)(ctx.regs),
    (unsigned long) map, flags, 0, 0);
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_FAST_STACK_CMP | BPF_F_REUSE_STACKID)))
    return -EINVAL;
    user = flags & BPF_F_USER_STACK;
    kernel = !user;
    trace = ctx.data.callchain;
    if (unlikely(!trace))
    return -EFAULT;
    nr_kernel = count_kernel_ip(trace);
    if (kernel) {
    trace_nr = nr_kernel;
    } else { /* user */
    let mut skip: u64 = flags & BPF_F_SKIP_FIELD_MASK;
    trace_nr = trace.nr;
    skip += nr_kernel;
    if (skip > BPF_F_SKIP_FIELD_MASK)
    return -EFAULT;
    flags = (flags & ~BPF_F_SKIP_FIELD_MASK) | skip;
    }
    ret = stackid_fastpath(&stackid, map, trace, trace_nr, flags);
    if (ret != -ENOENT)
    return ret;
    new_bucket = stackid_new_bucket(&stackid, map);
    if (new_bucket)
    return stackid_install(&stackid, map, new_bucket, flags);
    return -ENOMEM;
    }
    const struct bpf_func_proto bpf_get_stackid_proto_pe = {
    .func		= bpf_get_stackid_pe,
    .gpl_only	= false,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_CTX,
    .arg2_type	= ARG_CONST_MAP_PTR,
    .arg3_type	= ARG_ANYTHING,
    };
    static u32 callchain_store(const struct perf_callchain_entry *trace, u32 trace_nr,
    void *buf, u32 elem_size, u64 flags)
    {
    let mut user_build_id: bool = flags & BPF_F_USER_BUILD_ID;
    let mut skip: u32 = flags & BPF_F_SKIP_FIELD_MASK;
    const u64 *ips;
    u32 copy_len;
    trace_nr = trace_nr - skip;
    copy_len = trace_nr * elem_size;
    ips = trace.ip + skip;
    if (user_build_id) {
    struct bpf_stack_build_id *id_offs = buf;
    for (u32 i = 0; i < trace_nr; i++)
    id_offs[i].ip = ips[i];
    } else {
    memcpy(buf, ips, copy_len);
    }
    return trace_nr;
    }
    static long callchain_finalize(void *buf, u32 size, u32 trace_nr, u32 elem_size,
    u64 flags, bool may_fault)
    {
    let mut user_build_id: bool = flags & BPF_F_USER_BUILD_ID;
    let mut user: bool = flags & BPF_F_USER_STACK;
    let mut copy_len: u32 = trace_nr * elem_size;
    if (user_build_id)
    stack_map_get_build_id_offset(buf, trace_nr, user, may_fault);
    if (size > copy_len)
    memset(buf + copy_len, 0, size - copy_len);
    return copy_len;
    }
    static long __bpf_get_stack(struct pt_regs *regs, struct task_struct *task,
    void *buf, u32 size, u64 flags, bool may_fault)
    {
    let mut user_build_id: bool = flags & BPF_F_USER_BUILD_ID;
    let mut crosstask: bool = task && task != current;
    let mut skip: u32 = flags & BPF_F_SKIP_FIELD_MASK;
    let mut user: bool = flags & BPF_F_USER_STACK;
    struct perf_callchain_entry *trace;
    u32 trace_nr, elem_size, max_depth;
    let mut kernel: bool = !user;
    let mut err: c_int = -EINVAL;
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_USER_BUILD_ID)))
    goto clear;
    if (kernel && user_build_id)
    goto clear;
    elem_size = user_build_id ? sizeof(struct bpf_stack_build_id) : sizeof(u64);
    if (unlikely(size % elem_size))
    goto clear;
// cannot get valid user stack for task without user_mode regs
    if (task && user && !user_mode(regs))
    goto err_fault;
// get_perf_callchain does not support crosstask user stack walking
// but returns an empty stack instead of NULL.
//
    if (crosstask && user) {
    err = -EOPNOTSUPP;
    goto clear;
    }
    max_depth = stack_map_calculate_max_depth(size, elem_size, flags);
    preempt_disable();
    if (may_fault)
    rcu_read_lock(); /* need RCU for perf's callchain below */
    if (kernel && task) {
    trace = get_callchain_entry_for_task(task, max_depth);
    } else {
    trace = get_perf_callchain(regs, kernel, user, max_depth,
    crosstask, false, 0);
    }
    if (unlikely(!trace) || trace.nr < skip) {
    if (may_fault)
    rcu_read_unlock();
    preempt_enable();
    goto err_fault;
    }
    trace_nr = callchain_store(trace, trace.nr, buf, elem_size, flags);
// trace should not be dereferenced after this point
    if (may_fault)
    rcu_read_unlock();
    preempt_enable();
    return callchain_finalize(buf, size, trace_nr, elem_size, flags, may_fault);
    err_fault:
    err = -EFAULT;
    clear:
    memset(buf, 0, size);
    return err;
    }
    BPF_CALL_4(bpf_get_stack, struct pt_regs *, regs, void *, buf, u32, size,
    u64, flags)
    {
    return __bpf_get_stack(regs, core::ptr::null_mut(), buf, size, flags, false /* !may_fault */);
    }
    const struct bpf_func_proto bpf_get_stack_proto = {
    .func		= bpf_get_stack,
    .gpl_only	= true,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_CTX,
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE_OR_ZERO,
    .arg4_type	= ARG_ANYTHING,
    };
    BPF_CALL_4(bpf_get_stack_sleepable, struct pt_regs *, regs, void *, buf, u32, size,
    u64, flags)
    {
    return __bpf_get_stack(regs, core::ptr::null_mut(), buf, size, flags, true /* may_fault */);
    }
    const struct bpf_func_proto bpf_get_stack_sleepable_proto = {
    .func		= bpf_get_stack_sleepable,
    .gpl_only	= true,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_CTX,
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE_OR_ZERO,
    .arg4_type	= ARG_ANYTHING,
    };
    static long __bpf_get_task_stack(struct task_struct *task, void *buf, u32 size,
    u64 flags, bool may_fault)
    {
    struct pt_regs *regs;
    let mut res: c_long = -EINVAL;
    if (!try_get_task_stack(task)) {
    memset(buf, 0, size);
    return -EFAULT;
    }
    regs = task_pt_regs(task);
    if (regs)
    res = __bpf_get_stack(regs, task, buf, size, flags, may_fault);
    else
    memset(buf, 0, size);
    put_task_stack(task);
    return res;
    }
    BPF_CALL_4(bpf_get_task_stack, struct task_struct *, task, void *, buf,
    u32, size, u64, flags)
    {
    return __bpf_get_task_stack(task, buf, size, flags, false /* !may_fault */);
    }
    const struct bpf_func_proto bpf_get_task_stack_proto = {
    .func		= bpf_get_task_stack,
    .gpl_only	= false,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_BTF_ID,
    .arg1_btf_id	= &btf_tracing_ids[BTF_TRACING_TYPE_TASK],
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE_OR_ZERO,
    .arg4_type	= ARG_ANYTHING,
    };
    BPF_CALL_4(bpf_get_task_stack_sleepable, struct task_struct *, task, void *, buf,
    u32, size, u64, flags)
    {
    return __bpf_get_task_stack(task, buf, size, flags, true /* !may_fault */);
    }
    const struct bpf_func_proto bpf_get_task_stack_sleepable_proto = {
    .func		= bpf_get_task_stack_sleepable,
    .gpl_only	= false,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_BTF_ID,
    .arg1_btf_id	= &btf_tracing_ids[BTF_TRACING_TYPE_TASK],
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE_OR_ZERO,
    .arg4_type	= ARG_ANYTHING,
    };
    static int __bpf_get_stack_pe(const struct perf_callchain_entry *trace, u32 trace_nr,
    void *buf, u32 size, u64 flags)
    {
    let mut user_build_id: bool = flags & BPF_F_USER_BUILD_ID;
    let mut skip: u64 = flags & BPF_F_SKIP_FIELD_MASK;
    let mut user: bool = flags & BPF_F_USER_STACK;
    u32 elem_size, max_depth, nr_trace;
    let mut kernel: bool = !user;
    if (kernel && user_build_id)
    return -EINVAL;
    elem_size = user_build_id ? sizeof(struct bpf_stack_build_id) : sizeof(u64);
    if (unlikely(size % elem_size))
    return -EINVAL;
    max_depth = stack_map_calculate_max_depth(size, elem_size, flags);
    trace_nr = min_t(u32, trace_nr, max_depth);
    if (trace_nr < skip)
    return -EFAULT;
    nr_trace = callchain_store(trace, trace_nr, buf, elem_size, flags);
    return callchain_finalize(buf, size, nr_trace, elem_size, flags, false /* !may_fault */);
    }
    BPF_CALL_4(bpf_get_stack_pe, struct bpf_perf_event_data_kern *, ctx,
    void *, buf, u32, size, u64, flags)
    {
    struct pt_regs *regs = (struct pt_regs *)(ctx.regs);
    const struct perf_callchain_entry *trace;
    struct perf_event *event = ctx.event;
    bool kernel, user;
    let mut err: c_int = -EINVAL;
    __u64 nr_kernel;
    if (!(event.attr.sample_type & PERF_SAMPLE_CALLCHAIN))
    return __bpf_get_stack(regs, core::ptr::null_mut(), buf, size, flags, false /* !may_fault */);
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_USER_BUILD_ID)))
    goto clear;
    user = flags & BPF_F_USER_STACK;
    kernel = !user;
    err = -EFAULT;
    trace = ctx.data.callchain;
    if (unlikely(!trace))
    goto clear;
    nr_kernel = count_kernel_ip(trace);
    if (kernel) {
    err = __bpf_get_stack_pe(trace, nr_kernel, buf, size, flags);
    } else { /* user */
    let mut skip: u64 = flags & BPF_F_SKIP_FIELD_MASK;
    skip += nr_kernel;
    if (skip > BPF_F_SKIP_FIELD_MASK)
    goto clear;
    flags = (flags & ~BPF_F_SKIP_FIELD_MASK) | skip;
    err = __bpf_get_stack_pe(trace, trace.nr, buf, size, flags);
    }
    clear:
    if (err < 0)
    memset(buf, 0, size);
    return err;
    }
    const struct bpf_func_proto bpf_get_stack_proto_pe = {
    .func		= bpf_get_stack_pe,
    .gpl_only	= true,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_CTX,
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE_OR_ZERO,
    .arg4_type	= ARG_ANYTHING,
    };
// Called from eBPF program
    static void *stack_map_lookup_elem(struct bpf_map *map, void *key)
    {
    return ERR_PTR(-EOPNOTSUPP);
    }
// Called from syscall
    static int stack_map_lookup_and_delete_elem(struct bpf_map *map, void *key,
    void *value, u64 flags)
    {
    return bpf_stackmap_extract(map, key, value, true);
    }
// Called from syscall
    int bpf_stackmap_extract(struct bpf_map *map, void *key, void *value,
    bool delete)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    struct stack_map_bucket *bucket, *old_bucket;
    let mut id: u32 = *(u32 *)key, trace_len;
    if (unlikely(id >= smap.n_buckets))
    return -ENOENT;
    bucket = xchg(&smap.buckets[id], core::ptr::null_mut());
    if (!bucket)
    return -ENOENT;
    trace_len = bucket.nr * stack_map_data_size(map);
    memcpy(value, bucket.data, trace_len);
    memset(value + trace_len, 0, map.value_size - trace_len);
    if (delete)
    old_bucket = bucket;
    else
    old_bucket = xchg(&smap.buckets[id], bucket);
    if (old_bucket)
    pcpu_freelist_push(&smap.freelist, &old_bucket.fnode);
    return 0;
    }
    static int stack_map_get_next_key(struct bpf_map *map, void *key,
    void *next_key)
    {
    struct bpf_stack_map *smap = container_of(map,
    struct bpf_stack_map, map);
    u32 id;
    WARN_ON_ONCE(!rcu_read_lock_held());
    if (!key) {
    id = 0;
    } else {
    id = *(u32 *)key;
    if (id >= smap.n_buckets || !smap.buckets[id])
    id = 0;
    else
    id++;
    }
    while (id < smap.n_buckets && !smap.buckets[id])
    id++;
    if (id >= smap.n_buckets)
    return -ENOENT;
// (u32 *)next_key = id;
    return 0;
    }
    static long stack_map_update_elem(struct bpf_map *map, void *key, void *value,
    u64 map_flags)
    {
    return -EINVAL;
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn stack_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    static long stack_map_delete_elem(struct bpf_map *map, void *key)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    struct stack_map_bucket *old_bucket;
    let mut id: u32 = *(u32 *)key;
    if (unlikely(id >= smap.n_buckets))
    return -E2BIG;
    old_bucket = xchg(&smap.buckets[id], core::ptr::null_mut());
    if (old_bucket) {
    pcpu_freelist_push(&smap.freelist, &old_bucket.fnode);
    return 0;
    } else {
    return -ENOENT;
    }
    }
// Called when map->refcnt goes to zero, either from workqueue or from syscall
#[no_mangle]
unsafe extern "C" fn stack_map_free(map: *mut bpf_map) {
    static void stack_map_free(struct bpf_map *map)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    bpf_map_area_free(smap.elems);
    pcpu_freelist_destroy(&smap.freelist);
    bpf_map_area_free(smap);
    put_callchain_buffers();
    }
#[no_mangle]
unsafe extern "C" fn stack_map_mem_usage(map: *const bpf_map) -> u64 {
    static u64 stack_map_mem_usage(const struct bpf_map *map)
    {
    struct bpf_stack_map *smap = container_of(map, struct bpf_stack_map, map);
    let mut value_size: u64 = map.value_size;
    let mut n_buckets: u64 = smap.n_buckets;
    let mut enties: u64 = map.max_entries;
    let mut usage: u64 = sizeof(*smap);
    usage += n_buckets * sizeof(struct stack_map_bucket *);
    usage += enties * (sizeof(struct stack_map_bucket) + value_size);
    return usage;
    }
    BTF_ID_LIST_SINGLE(stack_trace_map_btf_ids, struct, bpf_stack_map)
    const struct bpf_map_ops stack_trace_map_ops = {
    .map_meta_equal = bpf_map_meta_equal,
    .map_alloc = stack_map_alloc,
    .map_free = stack_map_free,
    .map_get_next_key = stack_map_get_next_key,
    .map_lookup_elem = stack_map_lookup_elem,
    .map_lookup_and_delete_elem = stack_map_lookup_and_delete_elem,
    .map_update_elem = stack_map_update_elem,
    .map_delete_elem = stack_map_delete_elem,
    .map_check_btf = map_check_no_btf,
    .map_mem_usage = stack_map_mem_usage,
    .map_btf_id = &stack_trace_map_btf_ids[0],
    };
