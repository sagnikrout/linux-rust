//! Automatically rewritten from C to Rust
//! Source: mm/shrinker.c
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

    LIST_HEAD(shrinker_list);
    DEFINE_MUTEX(shrinker_mutex);

    static int shrinker_nr_max;
#[no_mangle]
pub unsafe extern "C" fn shrinker_unit_size(nr_items: c_int) -> c_int {
    static inline int shrinker_unit_size(int nr_items)
    {
    return (DIV_ROUND_UP(nr_items, SHRINKER_UNIT_BITS) * sizeof(struct shrinker_info_unit *));
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_unit_free(info: *mut shrinker_info, start: c_int) {
    static inline void shrinker_unit_free(struct shrinker_info *info, int start)
    {
    struct shrinker_info_unit **unit;
    int nr, i;
    if (!info)
    return;
    unit = info.unit;
    nr = DIV_ROUND_UP(info.map_nr_max, SHRINKER_UNIT_BITS);
    for (i = start; i < nr; i++) {
    if (!unit[i])
    break;
    kfree(unit[i]);
    unit[i] = core::ptr::null_mut();
    }
    }
    static inline int shrinker_unit_alloc(struct shrinker_info *new,
    struct shrinker_info *old, int nid)
    {
    struct shrinker_info_unit *unit;
    let mut nr: c_int = DIV_ROUND_UP(new.map_nr_max, SHRINKER_UNIT_BITS);
    let mut start: c_int = old ? DIV_ROUND_UP(old.map_nr_max, SHRINKER_UNIT_BITS) : 0;
    int i;
    for (i = start; i < nr; i++) {
    unit = kzalloc_node(sizeof(*unit), GFP_KERNEL, nid);
    if (!unit) {
    shrinker_unit_free(new, start);
    return -ENOMEM;
    }
    new.unit[i] = unit;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __free_shrinker_info(memcg: *mut mem_cgroup) {
    static void __free_shrinker_info(struct mem_cgroup *memcg)
    {
    struct mem_cgroup_per_node *pn;
    struct shrinker_info *info;
    int nid;
    lockdep_assert_held(&shrinker_mutex);
    for_each_node(nid) {
    pn = memcg.nodeinfo[nid];
    info = rcu_dereference_protected(pn.shrinker_info, true);
    shrinker_unit_free(info, 0);
    kvfree(info);
    rcu_assign_pointer(pn.shrinker_info, core::ptr::null_mut());
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_shrinker_info(memcg: *mut mem_cgroup) {
    void free_shrinker_info(struct mem_cgroup *memcg)
    {
    mutex_lock(&shrinker_mutex);
    __free_shrinker_info(memcg);
    mutex_unlock(&shrinker_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_shrinker_info(memcg: *mut mem_cgroup) -> c_int {
    int alloc_shrinker_info(struct mem_cgroup *memcg)
    {
    int nid, ret = 0;
    let mut array_size: c_int = 0;
    mutex_lock(&shrinker_mutex);
    array_size = shrinker_unit_size(shrinker_nr_max);
    for_each_node(nid) {
    struct shrinker_info *info = kvzalloc_node(sizeof(*info) + array_size,
    GFP_KERNEL, nid);
    if (!info)
    goto err;
    info.map_nr_max = shrinker_nr_max;
    if (shrinker_unit_alloc(info, core::ptr::null_mut(), nid)) {
    kvfree(info);
    goto err;
    }
    rcu_assign_pointer(memcg.nodeinfo[nid].shrinker_info, info);
    }
    mutex_unlock(&shrinker_mutex);
    return ret;
    err:
    __free_shrinker_info(memcg);
    mutex_unlock(&shrinker_mutex);
    return -ENOMEM;
    }
    static struct shrinker_info *shrinker_info_protected(struct mem_cgroup *memcg,
    int nid)
    {
    return rcu_dereference_protected(memcg.nodeinfo[nid].shrinker_info,
    lockdep_is_held(&shrinker_mutex));
    }
    static int expand_one_shrinker_info(struct mem_cgroup *memcg, int new_size,
    int old_size, int new_nr_max)
    {
    struct shrinker_info *new, *old;
    struct mem_cgroup_per_node *pn;
    int nid;
    for_each_node(nid) {
    pn = memcg.nodeinfo[nid];
    old = shrinker_info_protected(memcg, nid);
// Not yet online memcg
    if (!old)
    return 0;
// Already expanded this shrinker_info
    if (new_nr_max <= old.map_nr_max)
    continue;
    new = kvzalloc_node(sizeof(*new) + new_size, GFP_KERNEL, nid);
    if (!new)
    return -ENOMEM;
    new.map_nr_max = new_nr_max;
    memcpy(new.unit, old.unit, old_size);
    if (shrinker_unit_alloc(new, old, nid)) {
    kvfree(new);
    return -ENOMEM;
    }
    rcu_assign_pointer(pn.shrinker_info, new);
    kvfree_rcu(old, rcu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn expand_shrinker_info(new_id: c_int) -> c_int {
    static int expand_shrinker_info(int new_id)
    {
    let mut ret: c_int = 0;
    let mut new_nr_max: c_int = round_up(new_id + 1, SHRINKER_UNIT_BITS);
    int new_size, old_size = 0;
    struct mem_cgroup *memcg;
    if (!root_mem_cgroup)
    goto out;
    lockdep_assert_held(&shrinker_mutex);
    new_size = shrinker_unit_size(new_nr_max);
    old_size = shrinker_unit_size(shrinker_nr_max);
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    ret = expand_one_shrinker_info(memcg, new_size, old_size,
    new_nr_max);
    if (ret) {
    mem_cgroup_iter_break(core::ptr::null_mut(), memcg);
    goto out;
    }
    } while ((memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut())) != core::ptr::null_mut());
    out:
    if (!ret)
    shrinker_nr_max = new_nr_max;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_id_to_index(shrinker_id: c_int) -> c_int {
    static inline int shrinker_id_to_index(int shrinker_id)
    {
    return shrinker_id / SHRINKER_UNIT_BITS;
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_id_to_offset(shrinker_id: c_int) -> c_int {
    static inline int shrinker_id_to_offset(int shrinker_id)
    {
    return shrinker_id % SHRINKER_UNIT_BITS;
    }
#[no_mangle]
pub unsafe extern "C" fn calc_shrinker_id(index: c_int, offset: c_int) -> c_int {
    static inline int calc_shrinker_id(int index, int offset)
    {
    return index * SHRINKER_UNIT_BITS + offset;
    }
#[no_mangle]
pub unsafe extern "C" fn set_shrinker_bit(memcg: *mut mem_cgroup, nid: c_int, shrinker_id: c_int) {
    void set_shrinker_bit(struct mem_cgroup *memcg, int nid, int shrinker_id)
    {
    if (shrinker_id >= 0 && memcg && !mem_cgroup_is_root(memcg)) {
    struct shrinker_info *info;
    rcu_read_lock();
    info = rcu_dereference(memcg.nodeinfo[nid].shrinker_info);
    if (!WARN_ON_ONCE(shrinker_id >= info.map_nr_max)) {
    struct shrinker_info_unit *unit;
    unit = info.unit[shrinker_id_to_index(shrinker_id)];
// Pairs with smp mb in shrink_slab()
    smp_mb__before_atomic();
    set_bit(shrinker_id_to_offset(shrinker_id), unit.map);
    }
    rcu_read_unlock();
    }
    }
    static DEFINE_IDR(shrinker_idr);
#[no_mangle]
unsafe extern "C" fn shrinker_memcg_alloc(shrinker: *mut shrinker) -> c_int {
    static int shrinker_memcg_alloc(struct shrinker *shrinker)
    {
    int id;
    if (mem_cgroup_disabled())
    return -ENOSYS;
    if (mem_cgroup_kmem_disabled() && !(shrinker.flags & SHRINKER_NONSLAB))
    return -ENOSYS;
    guard(mutex)(&shrinker_mutex);
    id = idr_alloc(&shrinker_idr, shrinker, 0, 0, GFP_KERNEL);
    if (id < 0)
    return id;
    if (id >= shrinker_nr_max) {
    if (expand_shrinker_info(id)) {
    idr_remove(&shrinker_idr, id);
    return -ENOMEM;
    }
    }
    shrinker.id = id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shrinker_memcg_remove(shrinker: *mut shrinker) {
    static void shrinker_memcg_remove(struct shrinker *shrinker)
    {
    let mut id: c_int = shrinker.id;
    BUG_ON(id < 0);
    lockdep_assert_held(&shrinker_mutex);
    idr_remove(&shrinker_idr, id);
    }
    static long xchg_nr_deferred_memcg(int nid, struct shrinker *shrinker,
    struct mem_cgroup *memcg)
    {
    struct shrinker_info *info;
    struct shrinker_info_unit *unit;
    long nr_deferred;
    rcu_read_lock();
    info = rcu_dereference(memcg.nodeinfo[nid].shrinker_info);
    unit = info.unit[shrinker_id_to_index(shrinker.id)];
    nr_deferred = atomic_long_xchg(&unit.nr_deferred[shrinker_id_to_offset(shrinker.id)], 0);
    rcu_read_unlock();
    return nr_deferred;
    }
    static long add_nr_deferred_memcg(long nr, int nid, struct shrinker *shrinker,
    struct mem_cgroup *memcg)
    {
    struct shrinker_info *info;
    struct shrinker_info_unit *unit;
    long nr_deferred;
    rcu_read_lock();
    info = rcu_dereference(memcg.nodeinfo[nid].shrinker_info);
    unit = info.unit[shrinker_id_to_index(shrinker.id)];
    nr_deferred =
    atomic_long_add_return(nr, &unit.nr_deferred[shrinker_id_to_offset(shrinker.id)]);
    rcu_read_unlock();
    return nr_deferred;
    }
#[no_mangle]
pub unsafe extern "C" fn reparent_shrinker_deferred(memcg: *mut mem_cgroup) {
    void reparent_shrinker_deferred(struct mem_cgroup *memcg)
    {
    int nid, index, offset;
    long nr;
    struct mem_cgroup *parent = parent_mem_cgroup(memcg);
    struct shrinker_info *child_info, *parent_info;
    struct shrinker_info_unit *child_unit, *parent_unit;
// Prevent from concurrent shrinker_info expand
    mutex_lock(&shrinker_mutex);
    for_each_node(nid) {
    child_info = shrinker_info_protected(memcg, nid);
    parent_info = shrinker_info_protected(parent, nid);
    for (index = 0; index < shrinker_id_to_index(child_info.map_nr_max); index++) {
    child_unit = child_info.unit[index];
    parent_unit = parent_info.unit[index];
    for (offset = 0; offset < SHRINKER_UNIT_BITS; offset++) {
    nr = atomic_long_read(&child_unit.nr_deferred[offset]);
    atomic_long_add(nr, &parent_unit.nr_deferred[offset]);
    }
    }
    }
    mutex_unlock(&shrinker_mutex);
    }

#[no_mangle]
unsafe extern "C" fn shrinker_memcg_alloc(shrinker: *mut shrinker) -> c_int {
    static int shrinker_memcg_alloc(struct shrinker *shrinker)
    {
    return -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn shrinker_memcg_remove(shrinker: *mut shrinker) {
    static void shrinker_memcg_remove(struct shrinker *shrinker)
    {
    }
    static long xchg_nr_deferred_memcg(int nid, struct shrinker *shrinker,
    struct mem_cgroup *memcg)
    {
    return 0;
    }
    static long add_nr_deferred_memcg(long nr, int nid, struct shrinker *shrinker,
    struct mem_cgroup *memcg)
    {
    return 0;
    }

    static long xchg_nr_deferred(struct shrinker *shrinker,
    struct shrink_control *sc)
    {
    let mut nid: c_int = sc.nid;
    if (!(shrinker.flags & SHRINKER_NUMA_AWARE))
    nid = 0;
    if (sc.memcg &&
    (shrinker.flags & SHRINKER_MEMCG_AWARE))
    return xchg_nr_deferred_memcg(nid, shrinker,
    sc.memcg);
    return atomic_long_xchg(&shrinker.nr_deferred[nid], 0);
    }
    static long add_nr_deferred(long nr, struct shrinker *shrinker,
    struct shrink_control *sc)
    {
    let mut nid: c_int = sc.nid;
    if (!(shrinker.flags & SHRINKER_NUMA_AWARE))
    nid = 0;
    if (sc.memcg &&
    (shrinker.flags & SHRINKER_MEMCG_AWARE))
    return add_nr_deferred_memcg(nr, nid, shrinker,
    sc.memcg);
    return atomic_long_add_return(nr, &shrinker.nr_deferred[nid]);
    }
pub const SHRINK_BATCH: c_int = 128;
    static unsigned long do_shrink_slab(struct shrink_control *shrinkctl,
    struct shrinker *shrinker, int priority)
    {
    let mut freed: c_ulong = 0;
    unsigned long long delta;
    long total_scan;
    long freeable;
    long nr;
    long new_nr;
    long batch_size = shrinker.batch ? shrinker.batch
    : SHRINK_BATCH;
    let mut scanned: c_long = 0, next_deferred;
    freeable = shrinker.count_objects(shrinker, shrinkctl);
    if (freeable == 0 || freeable == SHRINK_EMPTY)
    return freeable;
//
// copy the current shrinker scan count into a local variable
// and zero it so that other concurrent shrinker invocations
// don't also do this scanning work.
//
    nr = xchg_nr_deferred(shrinker, shrinkctl);
    if (shrinker.seeks) {
    delta = freeable >> priority;
    delta *= 4;
    do_div(delta, shrinker.seeks);
    } else {
//
// These objects don't require any IO to create. Trim
// them aggressively under memory pressure to keep
// them from causing refetches in the IO caches.
//
    delta = freeable / 2;
    }
    total_scan = nr >> priority;
    total_scan += delta;
    total_scan = min(total_scan, (2 * freeable));
    trace_mm_shrink_slab_start(shrinker, shrinkctl, nr,
    freeable, delta, total_scan, priority,
    shrinkctl.memcg);
//
// Normally, we should not scan less than batch_size objects in one
// pass to avoid too frequent shrinker calls, but if the slab has less
// than batch_size objects in total and we are really tight on memory,
// we will try to reclaim all available objects, otherwise we can end
// up failing allocations although there are plenty of reclaimable
// objects spread over several slabs with usage less than the
// batch_size.
//
// We detect the "tight on memory" situations by looking at the total
// number of objects we want to scan (total_scan). If it is greater
// than the total number of objects on slab (freeable), we must be
// scanning at high prio and therefore should try to reclaim as much as
// possible.
//
    while (total_scan >= batch_size ||
    total_scan >= freeable) {
    unsigned long ret;
    let mut nr_to_scan: c_ulong = min(batch_size, total_scan);
    shrinkctl.nr_to_scan = nr_to_scan;
    shrinkctl.nr_scanned = nr_to_scan;
    ret = shrinker.scan_objects(shrinker, shrinkctl);
    if (ret == SHRINK_STOP)
    break;
    freed += ret;
    count_vm_events(SLABS_SCANNED, shrinkctl.nr_scanned);
    total_scan -= shrinkctl.nr_scanned;
    scanned += shrinkctl.nr_scanned;
    cond_resched();
    }
//
// The deferred work is increased by any new work (delta) that wasn't
// done, decreased by old deferred work that was done now.
//
// And it is capped to two times of the freeable items.
//
    next_deferred = max_t(long, (nr + delta - scanned), 0);
    next_deferred = min(next_deferred, (2 * freeable));
//
// move the unused scan count back into the shrinker in a
// manner that handles concurrent updates.
//
    new_nr = add_nr_deferred(next_deferred, shrinker, shrinkctl);
    trace_mm_shrink_slab_end(shrinker, shrinkctl.nid, freed, nr, new_nr, total_scan,
    shrinkctl.memcg);
    return freed;
    }

    static unsigned long shrink_slab_memcg(gfp_t gfp_mask, int nid,
    struct mem_cgroup *memcg, int priority)
    {
    struct shrinker_info *info;
    unsigned long ret, freed = 0;
    int offset, index = 0;
    if (!mem_cgroup_online(memcg))
    return 0;
//
// lockless algorithm of memcg shrink.
//
// The shrinker_info may be freed asynchronously via RCU in the
// expand_one_shrinker_info(), so the rcu_read_lock() needs to be used
// to ensure the existence of the shrinker_info.
//
// The shrinker_info_unit is never freed unless its corresponding memcg
// is destroyed. Here we already hold the refcount of memcg, so the
// memcg will not be destroyed, and of course shrinker_info_unit will
// not be freed.
//
// So in the memcg shrink:
// step 1: use rcu_read_lock() to guarantee existence of the
// shrinker_info.
// step 2: after getting shrinker_info_unit we can safely release the
// RCU lock.
// step 3: traverse the bitmap and calculate shrinker_id
// step 4: use rcu_read_lock() to guarantee existence of the shrinker.
// step 5: use shrinker_id to find the shrinker, then use
// shrinker_try_get() to guarantee existence of the shrinker,
// then we can release the RCU lock to do do_shrink_slab() that
// may sleep.
// step 6: do shrinker_put() paired with step 5 to put the refcount,
// if the refcount reaches 0, then wake up the waiter in
// shrinker_free() by calling complete().
// Note: here is different from the global shrink, we don't
// need to acquire the RCU lock to guarantee existence of
// the shrinker, because we don't need to use this
// shrinker to traverse the next shrinker in the bitmap.
// step 7: we have already exited the read-side of rcu critical section
// before calling do_shrink_slab(), the shrinker_info may be
// released in expand_one_shrinker_info(), so go back to step 1
// to reacquire the shrinker_info.
//
    again:
    rcu_read_lock();
    info = rcu_dereference(memcg.nodeinfo[nid].shrinker_info);
    if (unlikely(!info))
    goto unlock;
    if (index < shrinker_id_to_index(info.map_nr_max)) {
    struct shrinker_info_unit *unit;
    unit = info.unit[index];
    rcu_read_unlock();
    for_each_set_bit(offset, unit.map, SHRINKER_UNIT_BITS) {
    struct shrink_control sc = {
    .gfp_mask = gfp_mask,
    .nid = nid,
    .memcg = memcg,
    };
    struct shrinker *shrinker;
    let mut shrinker_id: c_int = calc_shrinker_id(index, offset);
    rcu_read_lock();
    shrinker = idr_find(&shrinker_idr, shrinker_id);
    if (unlikely(!shrinker || !shrinker_try_get(shrinker))) {
    clear_bit(offset, unit.map);
    rcu_read_unlock();
    continue;
    }
    rcu_read_unlock();
// Call non-slab shrinkers even though kmem is disabled
    if (!memcg_kmem_online() &&
    !(shrinker.flags & SHRINKER_NONSLAB)) {
    clear_bit(offset, unit.map);
    shrinker_put(shrinker);
    continue;
    }
    ret = do_shrink_slab(&sc, shrinker, priority);
    if (ret == SHRINK_EMPTY) {
    clear_bit(offset, unit.map);
//
// After the shrinker reported that it had no objects to
// free, but before we cleared the corresponding bit in
// the memcg shrinker map, a new object might have been
// added. To make sure, we have the bit set in this
// case, we invoke the shrinker one more time and reset
// the bit if it reports that it is not empty anymore.
// The memory barrier here pairs with the barrier in
// set_shrinker_bit():
//
// list_lru_add()     shrink_slab_memcg()
// list_add_tail()    clear_bit()
// <MB>               <MB>
// set_bit()          do_shrink_slab()
//
    smp_mb__after_atomic();
    ret = do_shrink_slab(&sc, shrinker, priority);
    if (ret == SHRINK_EMPTY)
    ret = 0;
    else
    set_shrinker_bit(memcg, nid, shrinker_id);
    }
    freed += ret;
    shrinker_put(shrinker);
    }
    index++;
    goto again;
    }
    unlock:
    rcu_read_unlock();
    return freed;
    }

    static unsigned long shrink_slab_memcg(gfp_t gfp_mask, int nid,
    struct mem_cgroup *memcg, int priority)
    {
    return 0;
    }

//
// shrink_slab - shrink slab caches
// @gfp_mask: allocation context
// @nid: node whose slab caches to target
// @memcg: memory cgroup whose slab caches to target
// @priority: the reclaim priority
//
// Call the shrink functions to age shrinkable caches.
//
// @nid is passed along to shrinkers with SHRINKER_NUMA_AWARE set,
// unaware shrinkers will receive a node id of 0 instead.
//
// @memcg specifies the memory cgroup to target. Unaware shrinkers
// are called only if it is the root cgroup.
//
// @priority is sc->priority, we take the number of objects and >> by priority
// in order to get the scan target.
//
// Returns the number of reclaimed slab objects.
//
    unsigned long shrink_slab(gfp_t gfp_mask, int nid, struct mem_cgroup *memcg,
    int priority)
    {
    unsigned long ret, freed = 0;
    struct shrinker *shrinker;
//
// The root memcg might be allocated even though memcg is disabled
// via "cgroup_disable=memory" boot parameter.  This could make
// mem_cgroup_is_root() return false, then just run memcg slab
// shrink, but skip global shrink.  This may result in premature
// oom.
//
    if (!mem_cgroup_disabled() && !mem_cgroup_is_root(memcg))
    return shrink_slab_memcg(gfp_mask, nid, memcg, priority);
//
// lockless algorithm of global shrink.
//
// In the unregistration setp, the shrinker will be freed asynchronously
// via RCU after its refcount reaches 0. So both rcu_read_lock() and
// shrinker_try_get() can be used to ensure the existence of the shrinker.
//
// So in the global shrink:
// step 1: use rcu_read_lock() to guarantee existence of the shrinker
// and the validity of the shrinker_list walk.
// step 2: use shrinker_try_get() to try get the refcount, if successful,
// then the existence of the shrinker can also be guaranteed,
// so we can release the RCU lock to do do_shrink_slab() that
// may sleep.
// step 3: *MUST* to reacquire the RCU lock before calling shrinker_put(),
// which ensures that neither this shrinker nor the next shrinker
// will be freed in the next traversal operation.
// step 4: do shrinker_put() paired with step 2 to put the refcount,
// if the refcount reaches 0, then wake up the waiter in
// shrinker_free() by calling complete().
//
    rcu_read_lock();
    list_for_each_entry_rcu(shrinker, &shrinker_list, list) {
    struct shrink_control sc = {
    .gfp_mask = gfp_mask,
    .nid = nid,
    .memcg = memcg,
    };
    if (!shrinker_try_get(shrinker))
    continue;
    rcu_read_unlock();
    ret = do_shrink_slab(&sc, shrinker, priority);
    if (ret == SHRINK_EMPTY)
    ret = 0;
    freed += ret;
    rcu_read_lock();
    shrinker_put(shrinker);
    }
    rcu_read_unlock();
    cond_resched();
    return freed;
    }
    struct shrinker *shrinker_alloc(unsigned int flags, const char *fmt, ...)
    {
    struct shrinker *shrinker;
    unsigned int size;
    va_list ap;
    int err;
    shrinker = kzalloc_obj(struct shrinker);
    if (!shrinker)
    return core::ptr::null_mut();
    va_start(ap, fmt);
    err = shrinker_debugfs_name_alloc(shrinker, fmt, ap);
    va_end(ap);
    if (err)
    goto err_name;
    shrinker.flags = flags | SHRINKER_ALLOCATED;
    shrinker.seeks = DEFAULT_SEEKS;
    if (flags & SHRINKER_MEMCG_AWARE) {
    err = shrinker_memcg_alloc(shrinker);
    if (err == -ENOSYS) {
// Memcg is not supported, fallback to non-memcg-aware shrinker.
    shrinker.flags &= ~SHRINKER_MEMCG_AWARE;
    goto non_memcg;
    }
    if (err)
    goto err_flags;
    return shrinker;
    }
    non_memcg:
//
// The nr_deferred is available on per memcg level for memcg aware
// shrinkers, so only allocate nr_deferred in the following cases:
// - non-memcg-aware shrinkers
// - !CONFIG_MEMCG
// - memcg is disabled by kernel command line
// - non-slab shrinkers: when memcg kmem is disabled
//
    size = sizeof(*shrinker.nr_deferred);
    if (flags & SHRINKER_NUMA_AWARE)
    size *= nr_node_ids;
    shrinker.nr_deferred = kzalloc(size, GFP_KERNEL);
    if (!shrinker.nr_deferred)
    goto err_flags;
    return shrinker;
    err_flags:
    shrinker_debugfs_name_free(shrinker);
    err_name:
    kfree(shrinker);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(shrinker_alloc);
#[no_mangle]
pub unsafe extern "C" fn shrinker_register(shrinker: *mut shrinker) {
    void shrinker_register(struct shrinker *shrinker)
    {
    if (unlikely(!(shrinker.flags & SHRINKER_ALLOCATED))) {
    pr_warn("Must use shrinker_alloc() to dynamically allocate the shrinker");
    return;
    }
    mutex_lock(&shrinker_mutex);
    list_add_tail_rcu(&shrinker.list, &shrinker_list);
    shrinker.flags |= SHRINKER_REGISTERED;
    shrinker_debugfs_add(shrinker);
    mutex_unlock(&shrinker_mutex);
    init_completion(&shrinker.done);
//
// Now the shrinker is fully set up, take the first reference to it to
// indicate that lookup operations are now allowed to use it via
// shrinker_try_get().
//
    refcount_set(&shrinker.refcount, 1);
    }
    EXPORT_SYMBOL_GPL(shrinker_register);
#[no_mangle]
unsafe extern "C" fn shrinker_free_rcu_cb(head: *mut rcu_head) {
    static void shrinker_free_rcu_cb(struct rcu_head *head)
    {
    struct shrinker *shrinker = container_of(head, struct shrinker, rcu);
    kfree(shrinker.nr_deferred);
    kfree(shrinker);
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_free(shrinker: *mut shrinker) {
    void shrinker_free(struct shrinker *shrinker)
    {
    struct dentry *debugfs_entry = core::ptr::null_mut();
    int debugfs_id;
    if (!shrinker)
    return;
    if (shrinker.flags & SHRINKER_REGISTERED) {
// drop the initial refcount
    shrinker_put(shrinker);
//
// Wait for all lookups of the shrinker to complete, after that,
// no shrinker is running or will run again, then we can safely
// free it asynchronously via RCU and safely free the structure
// where the shrinker is located, such as super_block etc.
//
    wait_for_completion(&shrinker.done);
    }
    mutex_lock(&shrinker_mutex);
    if (shrinker.flags & SHRINKER_REGISTERED) {
//
// Now we can safely remove it from the shrinker_list and then
// free it.
//
    list_del_rcu(&shrinker.list);
    debugfs_entry = shrinker_debugfs_detach(shrinker, &debugfs_id);
    shrinker.flags &= ~SHRINKER_REGISTERED;
    }
    shrinker_debugfs_name_free(shrinker);
    if (shrinker.flags & SHRINKER_MEMCG_AWARE)
    shrinker_memcg_remove(shrinker);
    mutex_unlock(&shrinker_mutex);
    if (debugfs_entry)
    shrinker_debugfs_remove(debugfs_entry, debugfs_id);
    call_rcu(&shrinker.rcu, shrinker_free_rcu_cb);
    }
    EXPORT_SYMBOL_GPL(shrinker_free);
