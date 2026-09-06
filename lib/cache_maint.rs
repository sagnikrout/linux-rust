//! Automatically rewritten from C to Rust
//! Source: lib/cache_maint.c
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
// Generic support for Memory System Cache Maintenance operations.
//
// Coherency maintenance drivers register with this simple framework that will
// iterate over each registered instance to first kick off invalidation and
// then to wait until it is complete.
//
// If no implementations are registered yet cpu_cache_has_invalidate_memregion()
// will return false. If this runs concurrently with unregistration then a
// race exists but this is no worse than the case where the operations instance
// responsible for a given memory region has not yet registered.
//

    static LIST_HEAD(cache_ops_instance_list);
    static DECLARE_RWSEM(cache_ops_instance_list_lock);
#[no_mangle]
unsafe extern "C" fn __cache_coherency_ops_instance_free(kref: *mut kref) {
    static void __cache_coherency_ops_instance_free(struct kref *kref)
    {
    struct cache_coherency_ops_inst *cci =
    container_of(kref, struct cache_coherency_ops_inst, kref);
    kfree(cci);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_coherency_ops_instance_put(cci: *mut cache_coherency_ops_inst) {
    void cache_coherency_ops_instance_put(struct cache_coherency_ops_inst *cci)
    {
    kref_put(&cci.kref, __cache_coherency_ops_instance_free);
    }
    EXPORT_SYMBOL_GPL(cache_coherency_ops_instance_put);
#[no_mangle]
unsafe extern "C" fn cache_inval_one(cci: *mut cache_coherency_ops_inst, data: *mut c_void) -> c_int {
    static int cache_inval_one(struct cache_coherency_ops_inst *cci, void *data)
    {
    if (!cci.ops)
    return -EINVAL;
    return cci.ops.wbinv(cci, data);
    }
#[no_mangle]
unsafe extern "C" fn cache_inval_done_one(cci: *mut cache_coherency_ops_inst) -> c_int {
    static int cache_inval_done_one(struct cache_coherency_ops_inst *cci)
    {
    if (!cci.ops)
    return -EINVAL;
    if (!cci.ops.done)
    return 0;
    return cci.ops.done(cci);
    }
#[no_mangle]
unsafe extern "C" fn cache_invalidate_memregion(addr: phys_addr_t, size: usize) -> c_int {
    static int cache_invalidate_memregion(phys_addr_t addr, size_t size)
    {
    int ret;
    struct cache_coherency_ops_inst *cci;
    struct cc_inval_params params = {
    .addr = addr,
    .size = size,
    };
    guard(rwsem_read)(&cache_ops_instance_list_lock);
    list_for_each_entry(cci, &cache_ops_instance_list, node) {
    ret = cache_inval_one(cci, &params);
    if (ret)
    return ret;
    }
    list_for_each_entry(cci, &cache_ops_instance_list, node) {
    ret = cache_inval_done_one(cci);
    if (ret)
    return ret;
    }
    return 0;
    }
    struct cache_coherency_ops_inst *
    _cache_coherency_ops_instance_alloc(const struct cache_coherency_ops *ops,
    size_t size)
    {
    struct cache_coherency_ops_inst *cci;
    if (!ops || !ops.wbinv)
    return core::ptr::null_mut();
    cci = kzalloc(size, GFP_KERNEL);
    if (!cci)
    return core::ptr::null_mut();
    cci.ops = ops;
    INIT_LIST_HEAD(&cci.node);
    kref_init(&cci.kref);
    return cci;
    }
    EXPORT_SYMBOL_NS_GPL(_cache_coherency_ops_instance_alloc, "CACHE_COHERENCY");
#[no_mangle]
pub unsafe extern "C" fn cache_coherency_ops_instance_register(cci: *mut cache_coherency_ops_inst) -> c_int {
    int cache_coherency_ops_instance_register(struct cache_coherency_ops_inst *cci)
    {
    guard(rwsem_write)(&cache_ops_instance_list_lock);
    list_add(&cci.node, &cache_ops_instance_list);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(cache_coherency_ops_instance_register, "CACHE_COHERENCY");
#[no_mangle]
pub unsafe extern "C" fn cache_coherency_ops_instance_unregister(cci: *mut cache_coherency_ops_inst) {
    void cache_coherency_ops_instance_unregister(struct cache_coherency_ops_inst *cci)
    {
    guard(rwsem_write)(&cache_ops_instance_list_lock);
    list_del(&cci.node);
    }
    EXPORT_SYMBOL_NS_GPL(cache_coherency_ops_instance_unregister, "CACHE_COHERENCY");
#[no_mangle]
pub unsafe extern "C" fn cpu_cache_invalidate_memregion(start: phys_addr_t, len: usize) -> c_int {
    int cpu_cache_invalidate_memregion(phys_addr_t start, size_t len)
    {
    return cache_invalidate_memregion(start, len);
    }
    EXPORT_SYMBOL_NS_GPL(cpu_cache_invalidate_memregion, "DEVMEM");
//
// Used for optimization / debug purposes only as removal can race
//
// Machines that do not support invalidation, e.g. VMs, will not have any
// operations instance to register and so this will always return false.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_cache_has_invalidate_memregion() -> bool {
    bool cpu_cache_has_invalidate_memregion(void)
    {
    guard(rwsem_read)(&cache_ops_instance_list_lock);
    return !list_empty(&cache_ops_instance_list);
    }
    EXPORT_SYMBOL_NS_GPL(cpu_cache_has_invalidate_memregion, "DEVMEM");
