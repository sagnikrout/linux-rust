//! Automatically rewritten from C to Rust
//! Source: mm/slab_common.c
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
// Slab allocator functions that are independent of the allocator strategy
//
// (C) 2012 Christoph Lameter <cl@gentwo.org>
//

// Macro flag: #define CREATE_TRACE_POINTS

    enum slab_state slab_state;
    LIST_HEAD(slab_caches);
    DEFINE_MUTEX(slab_mutex);
    struct kmem_cache *kmem_cache;
//
// Set of flags that will prevent slab merging.
// Any flag that adds per-object metadata should be included,
// since slab merging can update s->inuse that affects the metadata layout.
//

    SLAB_NOLEAKTRACE | SLAB_FAILSLAB | SLAB_NO_MERGE | \
    SLAB_OBJ_EXT_IN_OBJ)

    SLAB_CACHE_DMA32 | SLAB_ACCOUNT | SLAB_MAY_ACCOUNT)
//
// Merge control. If this is set then no merging of slab caches will occur.
//
    let mut slab_nomerge: static bool = !IS_ENABLED(CONFIG_SLAB_MERGE_DEFAULT);
#[no_mangle]
unsafe extern "C" fn setup_slab_nomerge(str: *mut c_char) -> int __init {
    static int __init setup_slab_nomerge(char *str)
    {
    slab_nomerge = true;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn setup_slab_merge(str: *mut c_char) -> int __init {
    static int __init setup_slab_merge(char *str)
    {
    slab_nomerge = false;
    return 1;
    }
    __setup_param("slub_nomerge", slub_nomerge, setup_slab_nomerge, 0);
    __setup_param("slub_merge", slub_merge, setup_slab_merge, 0);
    __setup("slab_nomerge", setup_slab_nomerge);
    __setup("slab_merge", setup_slab_merge);
//
// Determine the size of a slab object
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_size(s: *mut kmem_cache) -> c_uint {
    unsigned int kmem_cache_size(struct kmem_cache *s)
    {
    return s.object_size;
    }
    EXPORT_SYMBOL(kmem_cache_size);

#[no_mangle]
unsafe extern "C" fn kmem_cache_is_duplicate_name(name: *const c_char) -> bool {
    static bool kmem_cache_is_duplicate_name(const char *name)
    {
    struct kmem_cache *s;
    list_for_each_entry(s, &slab_caches, list) {
    if (!strcmp(s.name, name))
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_sanity_check(name: *const c_char, size: c_uint) -> c_int {
    static int kmem_cache_sanity_check(const char *name, unsigned int size)
    {
    if (!name || in_interrupt() || size > KMALLOC_MAX_SIZE) {
    pr_err("kmem_cache_create(%s) integrity check failed\n", name);
    return -EINVAL;
    }
// Duplicate names will confuse slabtop, et al
    WARN(kmem_cache_is_duplicate_name(name),
    "kmem_cache of name '%s' already exists\n", name);
    WARN_ON(strchr(name, ' '));	/* It confuses parsers */
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn kmem_cache_sanity_check(name: *const c_char, size: c_uint) -> c_int {
    static inline int kmem_cache_sanity_check(const char *name, unsigned int size)
    {
    return 0;
    }

//
// Figure out what the alignment of the objects will be given a set of
// flags, a user specified alignment and the size of the objects.
//
    static unsigned int calculate_alignment(slab_flags_t flags,
    unsigned int align, unsigned int size)
    {
//
// If the user wants hardware cache aligned objects then follow that
// suggestion if the object is sufficiently large.
//
// The hardware cache alignment cannot override the specified
// alignment though. If that is greater then use it.
//
    if (flags & SLAB_HWCACHE_ALIGN) {
    unsigned int ralign;
    ralign = cache_line_size();
    while (size <= ralign / 2)
    ralign /= 2;
    align = max(align, ralign);
    }
    align = max(align, arch_slab_minalign());
    return ALIGN(align, sizeof(void *));
    }
//
// Find a mergeable slab cache
//
#[no_mangle]
pub unsafe extern "C" fn slab_unmergeable(s: *mut kmem_cache) -> c_int {
    int slab_unmergeable(struct kmem_cache *s)
    {
    if (slab_nomerge || (s.flags & SLAB_NEVER_MERGE))
    return 1;
    if (s.ctor)
    return 1;

    if (s.usersize)
    return 1;

//
// We may have set a slab to be unmergeable during bootstrap.
//
    if (s.refcount < 0)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn slab_args_unmergeable(args: *mut kmem_cache_args, flags: slab_flags_t) -> bool {
    bool slab_args_unmergeable(struct kmem_cache_args *args, slab_flags_t flags)
    {
    if (slab_nomerge)
    return true;
    if (args.ctor)
    return true;
    if (IS_ENABLED(CONFIG_HARDENED_USERCOPY) && args.usersize)
    return true;
    if (flags & SLAB_NEVER_MERGE)
    return true;
    return false;
    }
    static struct kmem_cache *find_mergeable(unsigned int size, slab_flags_t flags,
    const char *name, struct kmem_cache_args *args)
    {
    struct kmem_cache *s;
    unsigned int align;
    flags = kmem_cache_flags(flags, name);
    if (slab_args_unmergeable(args, flags))
    return core::ptr::null_mut();
    size = ALIGN(size, sizeof(void *));
    align = calculate_alignment(flags, args.align, size);
    size = ALIGN(size, align);
    list_for_each_entry_reverse(s, &slab_caches, list) {
    if (slab_unmergeable(s))
    continue;
    if (size > s.size)
    continue;
    if ((flags & SLAB_MERGE_SAME) != (s.flags & SLAB_MERGE_SAME))
    continue;
//
// Check if alignment is compatible.
// Courtesy of Adrian Drzewiecki
//
    if ((s.size & ~(align - 1)) != s.size)
    continue;
    if (s.size - size >= sizeof(void *))
    continue;
    return s;
    }
    return core::ptr::null_mut();
    }
    static struct kmem_cache *create_cache(const char *name,
    unsigned int object_size,
    struct kmem_cache_args *args,
    slab_flags_t flags)
    {
    struct kmem_cache *s;
    int err;
// If a custom freelist pointer is requested make sure it's sane.
    err = -EINVAL;
    if (args.use_freeptr_offset &&
    (args.freeptr_offset >= object_size ||
    (!(flags & SLAB_TYPESAFE_BY_RCU) && !args.ctor) ||
    !IS_ALIGNED(args.freeptr_offset, __alignof__(freeptr_t))))
    goto out;
    err = -ENOMEM;
    s = kmem_cache_zalloc(kmem_cache, GFP_KERNEL);
    if (!s)
    goto out;
    err = do_kmem_cache_create(s, name, object_size, args, flags);
    if (err)
    goto out_free_cache;
    s.refcount = 1;
    list_add(&s.list, &slab_caches);
    return s;
    out_free_cache:
    kmem_cache_free(kmem_cache, s);
    out:
    return ERR_PTR(err);
    }
    static struct kmem_cache *
    __kmem_cache_alias(const char *name, unsigned int size, slab_flags_t flags,
    struct kmem_cache_args *args)
    {
    struct kmem_cache *s;
    s = find_mergeable(size, flags, name, args);
    if (s) {
    if (sysfs_slab_alias(s, name))
    pr_err("SLUB: Unable to add cache alias %s to sysfs\n",
    name);
    s.refcount++;
//
// Adjust the object sizes so that we clear
// the complete object on kzalloc.
//
    s.object_size = max(s.object_size, size);
    s.inuse = max(s.inuse, ALIGN(size, sizeof(void *)));
    }
    return s;
    }
//
// __kmem_cache_create_args - Create a kmem cache.
// @name: A string which is used in /proc/slabinfo to identify this cache.
// @object_size: The size of objects to be created in this cache.
// @args: Additional arguments for the cache creation (see
// &struct kmem_cache_args).
// @flags: See the descriptions of individual flags. The common ones are listed
// in the description below.
//
// Not to be called directly, use the kmem_cache_create() wrapper with the same
// parameters.
//
// Commonly used @flags:
//
// &SLAB_ACCOUNT - Account allocations to memcg.
//
// &SLAB_HWCACHE_ALIGN - Align objects on cache line boundaries.
//
// &SLAB_RECLAIM_ACCOUNT - Objects are reclaimable.
//
// &SLAB_TYPESAFE_BY_RCU - Slab page (not individual objects) freeing delayed
// by a grace period - see the full description before using.
//
// Context: Cannot be called within a interrupt, but can be interrupted.
//
// Return: a pointer to the cache on success, NULL on failure.
//
    struct kmem_cache *__kmem_cache_create_args(const char *name,
    unsigned int object_size,
    struct kmem_cache_args *args,
    slab_flags_t flags)
    {
    struct kmem_cache *s = core::ptr::null_mut();
    const char *cache_name;
    int err;

//
// If no slab_debug was enabled globally, the static key is not yet
// enabled by setup_slub_debug(). Enable it if the cache is being
// created with any of the debugging flags passed explicitly.
// It's also possible that this is the first cache created with
// SLAB_STORE_USER and we should init stack_depot for it.
//
    if (flags & SLAB_DEBUG_FLAGS)
    static_branch_enable(&slub_debug_enabled);
    if (flags & SLAB_STORE_USER)
    stack_depot_init();

    flags &= ~SLAB_DEBUG_FLAGS;

//
// Caches with specific capacity are special enough. It's simpler to
// make them unmergeable.
//
    if (args.sheaf_capacity)
    flags |= SLAB_NO_MERGE;
    mutex_lock(&slab_mutex);
    err = kmem_cache_sanity_check(name, object_size);
    if (err) {
    goto out_unlock;
    }
    if (flags & ~SLAB_FLAGS_PERMITTED) {
    err = -EINVAL;
    goto out_unlock;
    }
//
// For now we assume any cache can be used with __GFP_ACCOUNT and thus
// may need to store objcg pointers for objects
//
    if (!mem_cgroup_kmem_disabled())
    flags |= SLAB_MAY_ACCOUNT;
// Fail closed on bad usersize of useroffset values.
    if (!IS_ENABLED(CONFIG_HARDENED_USERCOPY) ||
    WARN_ON(!args.usersize && args.useroffset) ||
    WARN_ON(object_size < args.usersize ||
    object_size - args.usersize < args.useroffset))
    args.usersize = args.useroffset = 0;
    s = __kmem_cache_alias(name, object_size, flags, args);
    if (s)
    goto out_unlock;
    cache_name = kstrdup_const(name, GFP_KERNEL);
    if (!cache_name) {
    err = -ENOMEM;
    goto out_unlock;
    }
    args.align = calculate_alignment(flags, args.align, object_size);
    s = create_cache(cache_name, object_size, args, flags);
    if (IS_ERR(s)) {
    err = PTR_ERR(s);
    kfree_const(cache_name);
    }
    out_unlock:
    mutex_unlock(&slab_mutex);
    if (err) {
    if (flags & SLAB_PANIC)
    panic("%s: Failed to create slab '%s'. Error %d\n",
    __func__, name, err);
    else {
    pr_warn("%s(%s) failed with error %d\n",
    __func__, name, err);
    dump_stack();
    }
    return core::ptr::null_mut();
    }
    return s;
    }
    EXPORT_SYMBOL(__kmem_cache_create_args);
    static struct kmem_cache *kmem_buckets_cache __ro_after_init;
//
// kmem_buckets_create - Create a set of caches that handle dynamic sized
// allocations via kmem_buckets_alloc()
// @name: A prefix string which is used in /proc/slabinfo to identify this
// cache. The individual caches with have their sizes as the suffix.
// @flags: SLAB flags (see kmem_cache_create() for details).
// @useroffset: Starting offset within an allocation that may be copied
// to/from userspace.
// @usersize: How many bytes, starting at @useroffset, may be copied
// to/from userspace.
// @ctor: A constructor for the objects, run when new allocations are made.
//
// Cannot be called within an interrupt, but can be interrupted.
//
// Return: a pointer to the cache on success, NULL on failure. When
// CONFIG_SLAB_BUCKETS is not enabled, ZERO_SIZE_PTR is returned, and
// subsequent calls to kmem_buckets_alloc() will fall back to kmalloc().
// (i.e. callers only need to check for NULL on failure.)
//
    kmem_buckets *kmem_buckets_create(const char *name, slab_flags_t flags,
    unsigned int useroffset,
    unsigned int usersize,
    void (*ctor)(void *))
    {
    let mut mask: c_ulong = 0;
    unsigned int idx;
    kmem_buckets *b;
    BUILD_BUG_ON(ARRAY_SIZE(kmalloc_caches[KMALLOC_NORMAL]) > BITS_PER_LONG);
//
// When the separate buckets API is not built in, just return
// a non-NULL value for the kmem_buckets pointer, which will be
// unused when performing allocations.
//
    if (!IS_ENABLED(CONFIG_SLAB_BUCKETS))
    return ZERO_SIZE_PTR;
    if (WARN_ON(!kmem_buckets_cache))
    return core::ptr::null_mut();
    b = kmem_cache_alloc(kmem_buckets_cache, GFP_KERNEL|__GFP_ZERO);
    if (WARN_ON(!b))
    return core::ptr::null_mut();
    flags |= SLAB_NO_MERGE;
    for (idx = 0; idx < ARRAY_SIZE(kmalloc_caches[KMALLOC_NORMAL]); idx++) {
    char *short_size, *cache_name;
    unsigned int cache_useroffset, cache_usersize;
    unsigned int size, aligned_idx;
    if (!kmalloc_caches[KMALLOC_NORMAL][idx])
    continue;
    size = kmalloc_caches[KMALLOC_NORMAL][idx].object_size;
    if (!size)
    continue;
    short_size = strchr(kmalloc_caches[KMALLOC_NORMAL][idx].name, '-');
    if (WARN_ON(!short_size))
    goto fail;
    if (useroffset >= size) {
    cache_useroffset = 0;
    cache_usersize = 0;
    } else {
    cache_useroffset = useroffset;
    cache_usersize = min(size - cache_useroffset, usersize);
    }
    aligned_idx = __kmalloc_index(size, false);
    if (!(*b)[aligned_idx]) {
    cache_name = kasprintf(GFP_KERNEL, "%s-%s", name, short_size + 1);
    if (WARN_ON(!cache_name))
    goto fail;
    (*b)[aligned_idx] = kmem_cache_create_usercopy(cache_name, size,
    0, flags, cache_useroffset,
    cache_usersize, ctor);
    kfree(cache_name);
    if (WARN_ON(!(*b)[aligned_idx]))
    goto fail;
    set_bit(aligned_idx, &mask);
    }
    if (idx != aligned_idx)
    (*b)[idx] = (*b)[aligned_idx];
    }
    return b;
    fail:
    for_each_set_bit(idx, &mask, ARRAY_SIZE(kmalloc_caches[KMALLOC_NORMAL]))
    kmem_cache_destroy((*b)[idx]);
    kmem_cache_free(kmem_buckets_cache, b);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(kmem_buckets_create);
//
// For a given kmem_cache, kmem_cache_destroy() should only be called
// once or there will be a use-after-free problem. The actual deletion
// and release of the kobject does not need slab_mutex or cpu_hotplug_lock
// protection. So they are now done without holding those locks.
//
#[no_mangle]
unsafe extern "C" fn kmem_cache_release(s: *mut kmem_cache) {
    static void kmem_cache_release(struct kmem_cache *s)
    {
    kfence_shutdown_cache(s);
    if (__is_defined(SLAB_SUPPORTS_SYSFS) && slab_state >= FULL)
    sysfs_slab_release(s);
    else
    slab_kmem_cache_release(s);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_kmem_cache_release(s: *mut kmem_cache) {
    void slab_kmem_cache_release(struct kmem_cache *s)
    {
    __kmem_cache_release(s);
    kfree_const(s.name);
    kmem_cache_free(kmem_cache, s);
    }
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_destroy(s: *mut kmem_cache) {
    void kmem_cache_destroy(struct kmem_cache *s)
    {
    int err;
    if (unlikely(!s) || !kasan_check_byte(s))
    return;
// in-flight kfree_rcu()'s may include objects from our cache
    kvfree_rcu_barrier_on_cache(s);
    if (IS_ENABLED(CONFIG_SLUB_RCU_DEBUG) &&
    (s.flags & SLAB_TYPESAFE_BY_RCU)) {
//
// Under CONFIG_SLUB_RCU_DEBUG, when objects in a
// SLAB_TYPESAFE_BY_RCU slab are freed, SLUB will internally
// defer their freeing with call_rcu().
// Wait for such call_rcu() invocations here before actually
// destroying the cache.
//
// It doesn't matter that we haven't looked at the slab refcount
// yet - slabs with SLAB_TYPESAFE_BY_RCU can't be merged, so
// the refcount should be 1 here.
//
    rcu_barrier();
    }
// Wait for deferred work from kmalloc/kfree_nolock()
    deferred_work_barrier();
    cpus_read_lock();
    mutex_lock(&slab_mutex);
    s.refcount--;
    if (s.refcount) {
    mutex_unlock(&slab_mutex);
    cpus_read_unlock();
    return;
    }
// free asan quarantined objects
    kasan_cache_shutdown(s);
    err = __kmem_cache_shutdown(s);
    if (!slab_in_kunit_test())
    WARN(err, "%s %s: Slab cache still has objects when called from %pS",
    __func__, s.name, (void *)_RET_IP_);
    list_del(&s.list);
    mutex_unlock(&slab_mutex);
    cpus_read_unlock();
    if (slab_state >= FULL)
    sysfs_slab_unlink(s);
    debugfs_slab_release(s);
    if (err)
    return;
    if (s.flags & SLAB_TYPESAFE_BY_RCU)
    rcu_barrier();
    kmem_cache_release(s);
    }
    EXPORT_SYMBOL(kmem_cache_destroy);
//
// kmem_cache_shrink - Shrink a cache.
// @cachep: The cache to shrink.
//
// Releases as many slabs as possible for a cache.
// To help debugging, a zero exit status indicates all slabs were released.
//
// Return: %0 if all slabs were released, non-zero otherwise
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_shrink(cachep: *mut kmem_cache) -> c_int {
    int kmem_cache_shrink(struct kmem_cache *cachep)
    {
    kasan_cache_shrink(cachep);
    return __kmem_cache_shrink(cachep);
    }
    EXPORT_SYMBOL(kmem_cache_shrink);
#[no_mangle]
pub unsafe extern "C" fn slab_is_available() -> bool {
    bool slab_is_available(void)
    {
    return slab_state >= UP;
    }

#[no_mangle]
unsafe extern "C" fn kmem_obj_info(kpp: *mut kmem_obj_info, object: *mut c_void, slab: *mut slab) {
    static void kmem_obj_info(struct kmem_obj_info *kpp, void *object, struct slab *slab)
    {
    if (__kfence_obj_info(kpp, object, slab))
    return;
    __kmem_obj_info(kpp, object, slab);
    }
//
// kmem_dump_obj - Print available slab provenance information
// @object: slab object for which to find provenance information.
//
// This function uses pr_cont(), so that the caller is expected to have
// printed out whatever preamble is appropriate.  The provenance information
// depends on the type of object and on how much debugging is enabled.
// For a slab-cache object, the fact that it is a slab object is printed,
// and, if available, the slab name, return address, and stack trace from
// the allocation and last free path of that object.
//
// Return: %true if the pointer is to a not-yet-freed object from
// kmalloc() or kmem_cache_alloc(), either %true or %false if the pointer
// is to an already-freed object, and %false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn kmem_dump_obj(object: *mut c_void) -> bool {
    bool kmem_dump_obj(void *object)
    {
    char *cp = IS_ENABLED(CONFIG_MMU) ? "" : "/vmalloc";
    int i;
    struct slab *slab;
    unsigned long ptroffset;
    let mut kp: kmem_obj_info = { };
// Some arches consider ZERO_SIZE_PTR to be a valid address.
    if (object < (void *)PAGE_SIZE || !virt_addr_valid(object))
    return false;
    slab = virt_to_slab(object);
    if (!slab)
    return false;
    kmem_obj_info(&kp, object, slab);
    if (kp.kp_slab_cache)
    pr_cont(" slab%s %s", cp, kp.kp_slab_cache.name);
    else
    pr_cont(" slab%s", cp);
    if (is_kfence_address(object))
    pr_cont(" (kfence)");
    if (kp.kp_objp)
    pr_cont(" start %px", kp.kp_objp);
    if (kp.kp_data_offset)
    pr_cont(" data offset %lu", kp.kp_data_offset);
    if (kp.kp_objp) {
    ptroffset = ((char *)object - (char *)kp.kp_objp) - kp.kp_data_offset;
    pr_cont(" pointer offset %lu", ptroffset);
    }
    if (kp.kp_slab_cache && kp.kp_slab_cache.object_size)
    pr_cont(" size %u", kp.kp_slab_cache.object_size);
    if (kp.kp_ret)
    pr_cont(" allocated at %pS\n", kp.kp_ret);
    else
    pr_cont("\n");
    for (i = 0; i < ARRAY_SIZE(kp.kp_stack); i++) {
    if (!kp.kp_stack[i])
    break;
    pr_info("    %pS\n", kp.kp_stack[i]);
    }
    if (kp.kp_free_stack[0])
    pr_cont(" Free path:\n");
    for (i = 0; i < ARRAY_SIZE(kp.kp_free_stack); i++) {
    if (!kp.kp_free_stack[i])
    break;
    pr_info("    %pS\n", kp.kp_free_stack[i]);
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(kmem_dump_obj);

// Create a cache during boot when no slab services are available yet
    void __init create_boot_cache(struct kmem_cache *s, const char *name,
    unsigned int size, slab_flags_t flags,
    unsigned int useroffset, unsigned int usersize)
    {
    int err;
    let mut align: c_uint = ARCH_KMALLOC_MINALIGN;
    let mut kmem_args: kmem_cache_args = {};
//
// kmalloc caches guarantee alignment of at least the largest
// power-of-two divisor of the size. For power-of-two sizes,
// it is the size itself.
//
    if (flags & SLAB_KMALLOC)
    align = max(align, 1U << (ffs(size) - 1));
    kmem_args.align = calculate_alignment(flags, align, size);

    kmem_args.useroffset = useroffset;
    kmem_args.usersize = usersize;

    err = do_kmem_cache_create(s, name, size, &kmem_args, flags);
    if (err)
    panic("Creation of kmalloc slab %s size=%u failed. Reason %d\n",
    name, size, err);
    s.refcount = -1;	/* Exempt from merging for now */
    }
    static struct kmem_cache *__init create_kmalloc_cache(const char *name,
    unsigned int size,
    slab_flags_t flags)
    {
    struct kmem_cache *s = kmem_cache_zalloc(kmem_cache, GFP_NOWAIT);
    if (!s)
    panic("Out of memory when creating slab %s\n", name);
    create_boot_cache(s, name, size, flags | SLAB_KMALLOC, 0, size);
    list_add(&s.list, &slab_caches);
    s.refcount = 1;
    return s;
    }
    kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES] __ro_after_init =
    { /* initialization for https://llvm.org/pr42570 */ };
    EXPORT_SYMBOL(kmalloc_caches);

    unsigned long random_kmalloc_seed __ro_after_init;
    EXPORT_SYMBOL(random_kmalloc_seed);

//
// Conversion table for small slabs sizes / 8 to the index in the
// kmalloc array. This is necessary for slabs < 192 since we have non power
// of two cache sizes there. The size of larger slabs can be determined using
// fls.
//
    u8 kmalloc_size_index[24] __ro_after_init = {
    3,	/* 8 */
    4,	/* 16 */
    5,	/* 24 */
    5,	/* 32 */
    6,	/* 40 */
    6,	/* 48 */
    6,	/* 56 */
    6,	/* 64 */
    1,	/* 72 */
    1,	/* 80 */
    1,	/* 88 */
    1,	/* 96 */
    7,	/* 104 */
    7,	/* 112 */
    7,	/* 120 */
    7,	/* 128 */
    2,	/* 136 */
    2,	/* 144 */
    2,	/* 152 */
    2,	/* 160 */
    2,	/* 168 */
    2,	/* 176 */
    2,	/* 184 */
    2	/* 192 */
    };
#[no_mangle]
pub unsafe extern "C" fn kmalloc_size_roundup(size: usize) -> usize {
    size_t kmalloc_size_roundup(size_t size)
    {
    if (size && size <= KMALLOC_MAX_CACHE_SIZE) {
    struct kmem_cache *s;
//
// The flags don't matter since size_index is common to all.
// Neither does the caller for just getting ->object_size.
//
    s = kmalloc_slab(size, core::ptr::null_mut(), GFP_KERNEL, __kmalloc_token(0),
    SLAB_ALLOC_DEFAULT);
    return s.object_size;
    }
// Above the smaller buckets, size is a multiple of page size.
    if (size && size <= KMALLOC_MAX_SIZE)
    return PAGE_SIZE << get_order(size);
//
// Return 'size' for 0 - kmalloc() returns ZERO_SIZE_PTR
// and very large size - kmalloc() may fail.
//
    return size;
    }
    EXPORT_SYMBOL(kmalloc_size_roundup);

// Macro flag: #define KMALLOC_DMA_NAME(sz)

// Macro flag: #define KMALLOC_CGROUP_NAME(sz)

// Macro flag: #define KMALLOC_RCL_NAME(sz)

// Macro flag: #define KMALLOC_NO_OBJ_EXT_NAME(sz)

    {								\
    .name[KMALLOC_NORMAL]  = "kmalloc-" #__short_size,	\
    KMALLOC_RCL_NAME(__short_size)				\
    KMALLOC_CGROUP_NAME(__short_size)			\
    KMALLOC_DMA_NAME(__short_size)				\
    KMALLOC_PARTITION_NAME(KMALLOC_PARTITION_CACHES_NR, __short_size)	\
    KMALLOC_NO_OBJ_EXT_NAME(__short_size)			\
    .size = __size,						\
    }
//
// kmalloc_info[] is to make slab_debug=,kmalloc-xx option work at boot time.
// kmalloc_index() supports up to 2^21=2MB, so the final entry of the table is
// kmalloc-2M.
//
    const struct kmalloc_info_struct kmalloc_info[] __initconst = {
    INIT_KMALLOC_INFO(0, 0),
    INIT_KMALLOC_INFO(96, 96),
    INIT_KMALLOC_INFO(192, 192),
    INIT_KMALLOC_INFO(8, 8),
    INIT_KMALLOC_INFO(16, 16),
    INIT_KMALLOC_INFO(32, 32),
    INIT_KMALLOC_INFO(64, 64),
    INIT_KMALLOC_INFO(128, 128),
    INIT_KMALLOC_INFO(256, 256),
    INIT_KMALLOC_INFO(512, 512),
    INIT_KMALLOC_INFO(1024, 1k),
    INIT_KMALLOC_INFO(2048, 2k),
    INIT_KMALLOC_INFO(4096, 4k),
    INIT_KMALLOC_INFO(8192, 8k),
    INIT_KMALLOC_INFO(16384, 16k),
    INIT_KMALLOC_INFO(32768, 32k),
    INIT_KMALLOC_INFO(65536, 64k),
    INIT_KMALLOC_INFO(131072, 128k),
    INIT_KMALLOC_INFO(262144, 256k),
    INIT_KMALLOC_INFO(524288, 512k),
    INIT_KMALLOC_INFO(1048576, 1M),
    INIT_KMALLOC_INFO(2097152, 2M)
    };
//
// Patch up the size_index table if we have strange large alignment
// requirements for the kmalloc array. This is only the case for
// MIPS it seems. The standard arches will not generate any code here.
//
// Largest permitted alignment is 256 bytes due to the way we
// handle the index determination for the smaller caches.
//
// Make sure that nothing crazy happens if someone starts tinkering
// around with ARCH_KMALLOC_MINALIGN
//
#[no_mangle]
pub unsafe extern "C" fn setup_kmalloc_cache_index_table() -> void __init {
    void __init setup_kmalloc_cache_index_table(void)
    {
    unsigned int i;
    BUILD_BUG_ON(KMALLOC_MIN_SIZE > 256 ||
    !is_power_of_2(KMALLOC_MIN_SIZE));
    for (i = 8; i < KMALLOC_MIN_SIZE; i += 8) {
    let mut elem: c_uint = size_index_elem(i);
    if (elem >= ARRAY_SIZE(kmalloc_size_index))
    break;
    kmalloc_size_index[elem] = KMALLOC_SHIFT_LOW;
    }
    if (KMALLOC_MIN_SIZE >= 64) {
//
// The 96 byte sized cache is not used if the alignment
// is 64 byte.
//
    for (i = 64 + 8; i <= 96; i += 8)
    kmalloc_size_index[size_index_elem(i)] = 7;
    }
    if (KMALLOC_MIN_SIZE >= 128) {
//
// The 192 byte sized cache is not used if the alignment
// is 128 byte. Redirect kmalloc to use the 256 byte cache
// instead.
//
    for (i = 128 + 8; i <= 192; i += 8)
    kmalloc_size_index[size_index_elem(i)] = 8;
    }
    }
#[no_mangle]
unsafe extern "C" fn __kmalloc_minalign() -> c_uint {
    static unsigned int __kmalloc_minalign(void)
    {
    let mut minalign: c_uint = dma_get_cache_alignment();
    if (IS_ENABLED(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) &&
    is_swiotlb_allocated())
    minalign = ARCH_KMALLOC_MINALIGN;
    return max(minalign, arch_slab_minalign());
    }
    static void __init
    new_kmalloc_cache(int idx, enum kmalloc_cache_type type)
    {
    let mut flags: slab_flags_t = 0;
    let mut minalign: c_uint = __kmalloc_minalign();
    let mut aligned_size: c_uint = kmalloc_info[idx].size;
    let mut aligned_idx: c_int = idx;
    if ((KMALLOC_RECLAIM != KMALLOC_NORMAL) && (type == KMALLOC_RECLAIM)) {
    flags |= SLAB_RECLAIM_ACCOUNT;
    } else if (IS_ENABLED(CONFIG_MEMCG) && (type == KMALLOC_CGROUP)) {
    if (mem_cgroup_kmem_disabled()) {
    kmalloc_caches[type][idx] = kmalloc_caches[KMALLOC_NORMAL][idx];
    return;
    }
    flags |= SLAB_ACCOUNT;
    } else if (IS_ENABLED(CONFIG_SLAB_OBJ_EXT) && type == KMALLOC_NO_OBJ_EXT) {
    if (!need_kmalloc_no_objext()) {
    kmalloc_caches[type][idx] = kmalloc_caches[KMALLOC_NORMAL][idx];
    return;
    }
    flags |= SLAB_NO_OBJ_EXT | SLAB_NO_MERGE;
    } else if (IS_ENABLED(CONFIG_ZONE_DMA) && (type == KMALLOC_DMA)) {
    flags |= SLAB_CACHE_DMA;
    }

    if (type >= KMALLOC_PARTITION_START && type <= KMALLOC_PARTITION_END)
    flags |= SLAB_NO_MERGE;

//
// If memcg_kmem is enabled and this is a KMALLOC_NORMAL cache and not
// aliased with any other type, make sure it's never merged with any other
// cache.
//
// In other cases the kmalloc cache may end up being used for a
// __GFP_ACCOUNT allocation so mark it as such. The exception is a
// KMALLOC_NO_OBJ_EXT cache.
//
    if (!mem_cgroup_kmem_disabled()) {
    if (type == KMALLOC_NORMAL && KMALLOC_RECLAIM != KMALLOC_NORMAL)
    flags |= SLAB_NO_MERGE;
#[no_mangle]
pub unsafe extern "C" fn if(SLAB_NO_OBJ_EXT): !(flags &) -> else {
    else if (!(flags & SLAB_NO_OBJ_EXT))
    flags |= SLAB_MAY_ACCOUNT;
    }
    if (minalign > ARCH_KMALLOC_MINALIGN) {
    aligned_size = ALIGN(aligned_size, minalign);
    aligned_idx = __kmalloc_index(aligned_size, false);
    }
    if (!kmalloc_caches[type][aligned_idx])
    kmalloc_caches[type][aligned_idx] = create_kmalloc_cache(
    kmalloc_info[aligned_idx].name[type],
    aligned_size, flags);
    if (idx != aligned_idx)
    kmalloc_caches[type][idx] = kmalloc_caches[type][aligned_idx];
    }
//
// Create the kmalloc array. Some of the regular kmalloc arrays
// may already have been created because they were needed to
// enable allocations for slab creation.
//
#[no_mangle]
pub unsafe extern "C" fn create_kmalloc_caches() -> void __init {
    void __init create_kmalloc_caches(void)
    {
    int i;
    enum kmalloc_cache_type type;
//
// Including KMALLOC_CGROUP if CONFIG_MEMCG defined
//
    for (type = KMALLOC_NORMAL; type < NR_KMALLOC_TYPES; type++) {
// Caches that are NOT of the two-to-the-power-of size.
    if (KMALLOC_MIN_SIZE <= 32)
    new_kmalloc_cache(1, type);
    if (KMALLOC_MIN_SIZE <= 64)
    new_kmalloc_cache(2, type);
// Caches that are of the two-to-the-power-of size.
    for (i = KMALLOC_SHIFT_LOW; i <= KMALLOC_SHIFT_HIGH; i++)
    new_kmalloc_cache(i, type);
    }

    random_kmalloc_seed = get_random_u64();

// Kmalloc array is now usable
    slab_state = UP;
    if (IS_ENABLED(CONFIG_SLAB_BUCKETS))
    kmem_buckets_cache = kmem_cache_create("kmalloc_buckets",
    sizeof(kmem_buckets),
    0, SLAB_NO_MERGE, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn kmalloc_fix_flags(flags: gfp_t) -> gfp_t {
    gfp_t kmalloc_fix_flags(gfp_t flags)
    {
    let mut invalid_mask: gfp_t = flags & GFP_SLAB_BUG_MASK;
    flags &= ~GFP_SLAB_BUG_MASK;
    pr_warn("Unexpected gfp: %#x (%pGg). Fixing up to gfp: %#x (%pGg). Fix your code!\n",
    invalid_mask, &invalid_mask, flags, &flags);
    dump_stack();
    return flags;
    }

// Randomize a generic freelist
    static void freelist_randomize(unsigned int *list,
    unsigned int count)
    {
    unsigned int rand;
    unsigned int i;
    for (i = 0; i < count; i++)
    list[i] = i;
// Fisher-Yates shuffle
    for (i = count - 1; i > 0; i--) {
    rand = get_random_u32_below(i + 1);
    swap(list[i], list[rand]);
    }
    }
// Create a random sequence per cache
    int cache_random_seq_create(struct kmem_cache *cachep, unsigned int count,
    gfp_t gfp)
    {
    if (count < 2 || cachep.random_seq)
    return 0;
    cachep.random_seq = kcalloc(count, sizeof(unsigned int), gfp);
    if (!cachep.random_seq)
    return -ENOMEM;
    freelist_randomize(cachep.random_seq, count);
    return 0;
    }
// Destroy the per-cache random freelist sequence
#[no_mangle]
pub unsafe extern "C" fn cache_random_seq_destroy(cachep: *mut kmem_cache) {
    void cache_random_seq_destroy(struct kmem_cache *cachep)
    {
    kfree(cachep.random_seq);
    cachep.random_seq = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn print_slabinfo_header(m: *mut seq_file) {
    static void print_slabinfo_header(struct seq_file *m)
    {
//
// Output format version, so at least we can change it
// without _too_ many complaints.
//
    seq_puts(m, "slabinfo - version: 2.1\n");
    seq_puts(m, "# name            <active_objs> <num_objs> <objsize> <objperslab> <pagesperslab>");
    seq_puts(m, " : tunables <limit> <batchcount> <sharedfactor>");
    seq_puts(m, " : slabdata <active_slabs> <num_slabs> <sharedavail>");
    seq_putc(m, '\n');
    }
    static void *slab_start(struct seq_file *m, loff_t *pos)
    {
    mutex_lock(&slab_mutex);
    return seq_list_start(&slab_caches, *pos);
    }
    static void *slab_next(struct seq_file *m, void *p, loff_t *pos)
    {
    return seq_list_next(p, &slab_caches, pos);
    }
#[no_mangle]
unsafe extern "C" fn slab_stop(m: *mut seq_file, p: *mut c_void) {
    static void slab_stop(struct seq_file *m, void *p)
    {
    mutex_unlock(&slab_mutex);
    }
#[no_mangle]
unsafe extern "C" fn cache_show(s: *mut kmem_cache, m: *mut seq_file) {
    static void cache_show(struct kmem_cache *s, struct seq_file *m)
    {
    struct slabinfo sinfo;
    memset(&sinfo, 0, sizeof(sinfo));
    get_slabinfo(s, &sinfo);
    seq_printf(m, "%-17s %6lu %6lu %6u %4u %4d",
    s.name, sinfo.active_objs, sinfo.num_objs, s.size,
    sinfo.objects_per_slab, (1 << sinfo.cache_order));
    seq_printf(m, " : tunables %4u %4u %4u",
    sinfo.limit, sinfo.batchcount, sinfo.shared);
    seq_printf(m, " : slabdata %6lu %6lu %6lu",
    sinfo.active_slabs, sinfo.num_slabs, sinfo.shared_avail);
    seq_putc(m, '\n');
    }
#[no_mangle]
unsafe extern "C" fn slab_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int slab_show(struct seq_file *m, void *p)
    {
    struct kmem_cache *s = list_entry(p, struct kmem_cache, list);
    if (p == slab_caches.next)
    print_slabinfo_header(m);
    cache_show(s, m);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dump_unreclaimable_slab() {
    void dump_unreclaimable_slab(void)
    {
    struct kmem_cache *s;
    struct slabinfo sinfo;
//
// Here acquiring slab_mutex is risky since we don't prefer to get
// sleep in oom path. But, without mutex hold, it may introduce a
// risk of crash.
// Use mutex_trylock to protect the list traverse, dump nothing
// without acquiring the mutex.
//
    if (!mutex_trylock(&slab_mutex)) {
    pr_warn("excessive unreclaimable slab but cannot dump stats\n");
    return;
    }
    pr_info("Unreclaimable slab info:\n");
    pr_info("Name                      Used          Total\n");
    list_for_each_entry(s, &slab_caches, list) {
    if (s.flags & SLAB_RECLAIM_ACCOUNT)
    continue;
    get_slabinfo(s, &sinfo);
    if (sinfo.num_objs > 0)
    pr_info("%-17s %10luKB %10luKB\n", s.name,
    (sinfo.active_objs * s.size) / 1024,
    (sinfo.num_objs * s.size) / 1024);
    }
    mutex_unlock(&slab_mutex);
    }
//
// slabinfo_op - iterator that generates /proc/slabinfo
//
// Output layout:
// cache-name
// num-active-objs
// total-objs
// object size
// num-active-slabs
// total-slabs
// num-pages-per-slab
// + further values on SMP and with statistics enabled
//
    static const struct seq_operations slabinfo_op = {
    .start = slab_start,
    .next = slab_next,
    .stop = slab_stop,
    .show = slab_show,
    };
#[no_mangle]
unsafe extern "C" fn slabinfo_open(inode: *mut inode, file: *mut file) -> c_int {
    static int slabinfo_open(struct inode *inode, struct file *file)
    {
    return seq_open(file, &slabinfo_op);
    }
    static const struct proc_ops slabinfo_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_open	= slabinfo_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_release	= seq_release,
    };
#[no_mangle]
unsafe extern "C" fn slab_proc_init() -> int __init {
    static int __init slab_proc_init(void)
    {
    proc_create("slabinfo", SLABINFO_RIGHTS, core::ptr::null_mut(), &slabinfo_proc_ops);
    return 0;
    }
    module_init(slab_proc_init);

//
// kfree_sensitive - Clear sensitive information in memory before freeing
// @p: object to free memory of
//
// The memory of the object @p points to is zeroed before freed.
// If @p is %NULL, kfree_sensitive() does nothing.
//
// Note: this function zeroes the whole allocated buffer which can be a good
// deal bigger than the requested buffer size passed to kmalloc(). So be
// careful when using this function in performance sensitive code.
//
#[no_mangle]
pub unsafe extern "C" fn kfree_sensitive(p: *const c_void) {
    void kfree_sensitive(const void *p)
    {
    size_t ks;
    void *mem = (void *)p;
    ks = ksize(mem);
    if (ks) {
    kasan_unpoison_range(mem, ks);
    memzero_explicit(mem, ks);
    }
    kfree(mem);
    }
    EXPORT_SYMBOL(kfree_sensitive);

    __bpf_kfunc_start_defs();
    __bpf_kfunc struct kmem_cache *bpf_get_kmem_cache(u64 addr)
    {
    struct slab *slab;
    if (!virt_addr_valid((void *)(long)addr))
    return core::ptr::null_mut();
    slab = virt_to_slab((void *)(long)addr);
    return slab ? slab.slab_cache : core::ptr::null_mut();
    }
    __bpf_kfunc_end_defs();

// Tracepoints definitions.
    EXPORT_TRACEPOINT_SYMBOL(kmalloc);
    EXPORT_TRACEPOINT_SYMBOL(kmem_cache_alloc);
    EXPORT_TRACEPOINT_SYMBOL(kfree);
    EXPORT_TRACEPOINT_SYMBOL(kmem_cache_free);
#[no_mangle]
pub unsafe extern "C" fn kfree_call_rcu_nolock(head: *mut kvfree_rcu_head, ptr: *mut c_void) {
    void kfree_call_rcu_nolock(struct kvfree_rcu_head *head, void *ptr)
    {
    struct slab *slab;
    if (!IS_ENABLED(CONFIG_KVFREE_RCU_BATCHED))
    goto fallback;
    if (unlikely(is_vmalloc_addr(ptr)))
    goto fallback;
    slab = virt_to_slab(ptr);
    if (unlikely(!slab))
    goto fallback;
    if (unlikely(IS_ENABLED(CONFIG_NUMA) && slab_nid(slab) != numa_mem_id()))
    goto fallback;
    if (unlikely(!__kfree_rcu_sheaf(slab.slab_cache, ptr, SLAB_FREE_NOLOCK)))
    goto fallback;
    return;
    fallback:
    defer_kfree_rcu(head);
    }
    EXPORT_SYMBOL_GPL(kfree_call_rcu_nolock);

#[no_mangle]
pub unsafe extern "C" fn kvfree_call_rcu(head: *mut kvfree_rcu_head, ptr: *mut c_void) {
    void kvfree_call_rcu(struct kvfree_rcu_head *head, void *ptr)
    {
    if (head) {
    kasan_record_aux_stack(ptr);
    call_rcu(&head.head, kvfree_rcu_cb);
    return;
    }
// kvfree_rcu(one_arg) call.
    might_sleep();
    synchronize_rcu();
    kvfree(ptr);
    }
    EXPORT_SYMBOL_GPL(kvfree_call_rcu);
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_barrier() {
    void kvfree_rcu_barrier(void)
    {
    deferred_work_barrier();
    rcu_barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_barrier_on_cache(s: *mut kmem_cache) {
    void kvfree_rcu_barrier_on_cache(struct kmem_cache *s)
    {
    deferred_work_barrier();
    rcu_barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_init() -> void __init {
    void __init kvfree_rcu_init(void)
    {
    }

//
// This rcu parameter is runtime-read-only. It reflects
// a minimum allowed number of objects which can be cached
// per-CPU. Object size is equal to one page. This value
// can be changed at boot time.
//
    let mut rcu_min_cached_objs: static int = 5;
    module_param(rcu_min_cached_objs, int, 0444);
// A page shrinker can ask for pages to be freed to make them
// available for other parts of the system. This usually happens
// under low memory conditions, and in that case we should also
// defer page-cache filling for a short time period.
//
// The default value is 5 seconds, which is long enough to reduce
// interference with the shrinker while it asks other systems to
// drain their caches.
    let mut rcu_delay_page_cache_fill_msec: static int = 5000;
    module_param(rcu_delay_page_cache_fill_msec, int, 0444);
    static struct workqueue_struct *rcu_reclaim_wq;
// Maximum number of jiffies to wait before draining a batch.

pub const KFREE_N_BATCHES: c_int = 2;
pub const FREE_N_CHANNELS: c_int = 2;
//
// struct kvfree_rcu_bulk_data - single block to store kvfree_rcu() pointers
// @list: List node. All blocks are linked between each other
// @gp_snap: Snapshot of RCU state for objects placed to this bulk
// @nr_records: Number of active pointers in the array
// @records: Array of the kvfree_rcu() pointers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvfree_rcu_bulk_data {
    pub list: list_head,
    pub gp_snap: rcu_gp_seq,
    pub nr_records: c_ulong,
    pub __counted_by(nr_records): *mut *mut void records[],
}

//
// This macro defines how many entries the "records" array
// will contain. It is based on the fact that the size of
// kvfree_rcu_bulk_data structure becomes exactly one page.
//

    ((PAGE_SIZE - sizeof(struct kvfree_rcu_bulk_data)) / sizeof(void *))
//
// struct kfree_rcu_cpu_work - single batch of kfree_rcu() requests
// @rcu_work: Let queue_rcu_work() invoke workqueue handler after grace period
// @head_free: List of kfree_rcu() objects waiting for a grace period
// @head_free_gp_snap: Grace-period snapshot to check for attempted premature frees.
// @bulk_head_free: Bulk-List of kvfree_rcu() objects waiting for a grace period
// @krcp: Pointer to @kfree_rcu_cpu structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfree_rcu_cpu_work {
    pub rcu_work: rcu_work,
    pub head_free: *mut kvfree_rcu_head,
    pub head_free_gp_snap: rcu_gp_seq,
    pub bulk_head_free: [list_head; FREE_N_CHANNELS],
    pub krcp: *mut kfree_rcu_cpu,
}

//
// struct kfree_rcu_cpu - batch up kfree_rcu() requests for RCU grace period
// @head: List of kfree_rcu() objects not yet waiting for a grace period
// @head_gp_snap: Snapshot of RCU state for objects placed to "@head"
// @bulk_head: Bulk-List of kvfree_rcu() objects not yet waiting for a grace period
// @krw_arr: Array of batches of kfree_rcu() objects waiting for a grace period
// @lock: Synchronize access to this structure
// @monitor_work: Promote @head to @head_free after KFREE_DRAIN_JIFFIES
// @initialized: The @rcu_work fields have been initialized
// @head_count: Number of objects in rcu_head singular list
// @bulk_count: Number of objects in bulk-list
// @bkvcache:
// A simple cache list that contains objects for reuse purpose.
// In order to save some per-cpu space the list is singular.
// Even though it is lockless an access has to be protected by the
// per-cpu lock.
// @page_cache_work: A work to refill the cache when it is empty
// @backoff_page_cache_fill: Delay cache refills
// @work_in_progress: Indicates that page_cache_work is running
// @hrtimer: A hrtimer for scheduling a page_cache_work
// @nr_bkv_objs: number of allocated objects at @bkvcache.
//
// This is a per-CPU structure.  The reason that it is not included in
// the rcu_data structure is to permit this code to be extracted from
// the RCU files.  Such extraction could allow further optimization of
// the interactions with the slab allocators.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfree_rcu_cpu {
// Objects queued on a linked list
// through their rcu_head structures.
    pub head: *mut kvfree_rcu_head,
    pub head_gp_snap: c_ulong,
    pub head_count: core::sync::atomic::AtomicI32,
// Objects queued on a bulk-list.
    pub bulk_head: [list_head; FREE_N_CHANNELS],
    pub bulk_count: [core::sync::atomic::AtomicI32; FREE_N_CHANNELS],
    pub krw_arr: [kfree_rcu_cpu_work; KFREE_N_BATCHES],
    pub lock: raw_spinlock_t,
    pub monitor_work: delayed_work,
    pub initialized: bool,
    pub page_cache_work: delayed_work,
    pub backoff_page_cache_fill: core::sync::atomic::AtomicI32,
    pub work_in_progress: core::sync::atomic::AtomicI32,
    pub hrtimer: hrtimer,
    pub bkvcache: llist_head,
    pub nr_bkv_objs: c_int,
}

    static DEFINE_PER_CPU(struct kfree_rcu_cpu, krc) = {
    .lock = __RAW_SPIN_LOCK_UNLOCKED(krc.lock),
    };
    static __always_inline void
    debug_rcu_bhead_unqueue(struct kvfree_rcu_bulk_data *bhead)
    {

    int i;
    for (i = 0; i < bhead.nr_records; i++)
    debug_rcu_head_unqueue((struct rcu_head *)(bhead.records[i]));

    }
    static inline struct kfree_rcu_cpu *
    krc_this_cpu_lock(unsigned long *flags)
    {
    struct kfree_rcu_cpu *krcp;
    local_irq_save(*flags);	// For safely calling this_cpu_ptr().
    krcp = this_cpu_ptr(&krc);
    raw_spin_lock(&krcp.lock);
    return krcp;
    }
    static inline void
    krc_this_cpu_unlock(struct kfree_rcu_cpu *krcp, unsigned long flags)
    {
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    }
    static inline struct kvfree_rcu_bulk_data *
    get_cached_bnode(struct kfree_rcu_cpu *krcp)
    {
    if (!krcp.nr_bkv_objs)
    return core::ptr::null_mut();
    WRITE_ONCE(krcp.nr_bkv_objs, krcp.nr_bkv_objs - 1);
    return (struct kvfree_rcu_bulk_data *)
    llist_del_first(&krcp.bkvcache);
    }
    static inline bool
    put_cached_bnode(struct kfree_rcu_cpu *krcp,
    struct kvfree_rcu_bulk_data *bnode)
    {
// Check the limit.
    if (krcp.nr_bkv_objs >= rcu_min_cached_objs)
    return false;
    llist_add((struct llist_node *) bnode, &krcp.bkvcache);
    WRITE_ONCE(krcp.nr_bkv_objs, krcp.nr_bkv_objs + 1);
    return true;
    }
    static int
    drain_page_cache(struct kfree_rcu_cpu *krcp)
    {
    unsigned long flags;
    struct llist_node *page_list, *pos, *n;
    let mut freed: c_int = 0;
    if (!rcu_min_cached_objs)
    return 0;
    raw_spin_lock_irqsave(&krcp.lock, flags);
    page_list = llist_del_all(&krcp.bkvcache);
    WRITE_ONCE(krcp.nr_bkv_objs, 0);
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    llist_for_each_safe(pos, n, page_list) {
    free_page((unsigned long)pos);
    freed++;
    }
    return freed;
    }
    static void
    kvfree_rcu_bulk(struct kfree_rcu_cpu *krcp,
    struct kvfree_rcu_bulk_data *bnode, int idx)
    {
    unsigned long flags;
    int i;
    if (!WARN_ON_ONCE(!poll_state_synchronize_rcu_full(&bnode.gp_snap))) {
    debug_rcu_bhead_unqueue(bnode);
    rcu_lock_acquire(&rcu_callback_map);
    if (idx == 0) { // kmalloc() / kfree().
    trace_rcu_invoke_kfree_bulk_callback(
    "slab", bnode.nr_records,
    bnode.records);
    kfree_bulk(bnode.nr_records, bnode.records);
    } else { // vmalloc() / vfree().
    for (i = 0; i < bnode.nr_records; i++) {
    trace_rcu_invoke_kvfree_callback(
    "slab", bnode.records[i], 0);
    vfree(bnode.records[i]);
    }
    }
    rcu_lock_release(&rcu_callback_map);
    }
    raw_spin_lock_irqsave(&krcp.lock, flags);
    if (put_cached_bnode(krcp, bnode))
    bnode = core::ptr::null_mut();
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    if (bnode)
    free_page((unsigned long) bnode);
    cond_resched_tasks_rcu_qs();
    }
    static void
    kvfree_rcu_list(struct kvfree_rcu_head *head)
    {
    struct kvfree_rcu_head *next;
    for (; head; head = next) {
    void *ptr = kvmalloc_obj_start_addr(head);
    let mut offset: c_ulong = (void *) head - ptr;
    next = head.next;
    debug_rcu_head_unqueue((struct rcu_head *)ptr);
    rcu_lock_acquire(&rcu_callback_map);
    trace_rcu_invoke_kvfree_callback("slab", head, offset);
    kvfree(ptr);
    rcu_lock_release(&rcu_callback_map);
    cond_resched_tasks_rcu_qs();
    }
    }
//
// This function is invoked in workqueue context after a grace period.
// It frees all the objects queued on ->bulk_head_free or ->head_free.
//
#[no_mangle]
unsafe extern "C" fn kfree_rcu_work(work: *mut work_struct) {
    static void kfree_rcu_work(struct work_struct *work)
    {
    unsigned long flags;
    struct kvfree_rcu_bulk_data *bnode, *n;
    struct list_head bulk_head[FREE_N_CHANNELS];
    struct kvfree_rcu_head *head;
    struct kfree_rcu_cpu *krcp;
    struct kfree_rcu_cpu_work *krwp;
    struct rcu_gp_seq head_gp_snap;
    int i;
    krwp = container_of(to_rcu_work(work),
    struct kfree_rcu_cpu_work, rcu_work);
    krcp = krwp.krcp;
    raw_spin_lock_irqsave(&krcp.lock, flags);
// Channels 1 and 2.
    for (i = 0; i < FREE_N_CHANNELS; i++)
    list_replace_init(&krwp.bulk_head_free[i], &bulk_head[i]);
// Channel 3.
    head = krwp.head_free;
    krwp.head_free = core::ptr::null_mut();
    head_gp_snap = krwp.head_free_gp_snap;
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
// Handle the first two channels.
    for (i = 0; i < FREE_N_CHANNELS; i++) {
// Start from the tail page, so a GP is likely passed for it.
    list_for_each_entry_safe(bnode, n, &bulk_head[i], list)
    kvfree_rcu_bulk(krcp, bnode, i);
    }
//
// This is used when the "bulk" path can not be used for the
// double-argument of kvfree_rcu().  This happens when the
// page-cache is empty, which means that objects are instead
// queued on a linked list through their rcu_head structures.
// This list is named "Channel 3".
//
    if (head && !WARN_ON_ONCE(!poll_state_synchronize_rcu_full(&head_gp_snap)))
    kvfree_rcu_list(head);
    }
#[no_mangle]
unsafe extern "C" fn kfree_rcu_sheaf(obj: *mut c_void) -> bool {
    static bool kfree_rcu_sheaf(void *obj)
    {
    struct kmem_cache *s;
    struct slab *slab;
    let mut free_flags: c_uint = SLAB_FREE_DEFAULT;
//
// It is not safe to spin on PREEMPT_RT because the kernel might be
// holding a raw spinlock and slab acquires sleeping locks.
//
    if (IS_ENABLED(CONFIG_PREEMPT_RT))
    free_flags = SLAB_FREE_NOLOCK;
    if (is_vmalloc_addr(obj))
    return false;
    slab = virt_to_slab(obj);
    if (unlikely(!slab))
    return false;
    s = slab.slab_cache;
    if (likely(!IS_ENABLED(CONFIG_NUMA) || slab_nid(slab) == numa_mem_id()))
    return __kfree_rcu_sheaf(s, obj, free_flags);
    return false;
    }
    static bool
    need_offload_krc(struct kfree_rcu_cpu *krcp)
    {
    int i;
    for (i = 0; i < FREE_N_CHANNELS; i++)
    if (!list_empty(&krcp.bulk_head[i]))
    return true;
    return !!READ_ONCE(krcp.head);
    }
    static bool
    need_wait_for_krwp_work(struct kfree_rcu_cpu_work *krwp)
    {
    int i;
    for (i = 0; i < FREE_N_CHANNELS; i++)
    if (!list_empty(&krwp.bulk_head_free[i]))
    return true;
    return !!krwp.head_free;
    }
#[no_mangle]
unsafe extern "C" fn krc_count(krcp: *mut kfree_rcu_cpu) -> c_int {
    static int krc_count(struct kfree_rcu_cpu *krcp)
    {
    let mut sum: c_int = atomic_read(&krcp.head_count);
    int i;
    for (i = 0; i < FREE_N_CHANNELS; i++)
    sum += atomic_read(&krcp.bulk_count[i]);
    return sum;
    }
    static void
    __schedule_delayed_monitor_work(struct kfree_rcu_cpu *krcp)
    {
    long delay, delay_left;
    delay = krc_count(krcp) >= KVFREE_BULK_MAX_ENTR ? 1:KFREE_DRAIN_JIFFIES;
    if (delayed_work_pending(&krcp.monitor_work)) {
    delay_left = krcp.monitor_work.timer.expires - jiffies;
    if (delay < delay_left)
    mod_delayed_work(rcu_reclaim_wq, &krcp.monitor_work, delay);
    return;
    }
    queue_delayed_work(rcu_reclaim_wq, &krcp.monitor_work, delay);
    }
    static void
    schedule_delayed_monitor_work(struct kfree_rcu_cpu *krcp)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&krcp.lock, flags);
    __schedule_delayed_monitor_work(krcp);
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    }
    static void
    kvfree_rcu_drain_ready(struct kfree_rcu_cpu *krcp)
    {
    struct list_head bulk_ready[FREE_N_CHANNELS];
    struct kvfree_rcu_bulk_data *bnode, *n;
    struct kvfree_rcu_head *head_ready = core::ptr::null_mut();
    unsigned long flags;
    int i;
    raw_spin_lock_irqsave(&krcp.lock, flags);
    for (i = 0; i < FREE_N_CHANNELS; i++) {
    INIT_LIST_HEAD(&bulk_ready[i]);
    list_for_each_entry_safe_reverse(bnode, n, &krcp.bulk_head[i], list) {
    if (!poll_state_synchronize_rcu_full(&bnode.gp_snap))
    break;
    atomic_sub(bnode.nr_records, &krcp.bulk_count[i]);
    list_move(&bnode.list, &bulk_ready[i]);
    }
    }
    if (krcp.head && poll_state_synchronize_rcu(krcp.head_gp_snap)) {
    head_ready = krcp.head;
    atomic_set(&krcp.head_count, 0);
    WRITE_ONCE(krcp.head, core::ptr::null_mut());
    }
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    for (i = 0; i < FREE_N_CHANNELS; i++) {
    list_for_each_entry_safe(bnode, n, &bulk_ready[i], list)
    kvfree_rcu_bulk(krcp, bnode, i);
    }
    if (head_ready)
    kvfree_rcu_list(head_ready);
    }
//
// Return: %true if a work is queued, %false otherwise.
//
    static bool
    kvfree_rcu_queue_batch(struct kfree_rcu_cpu *krcp)
    {
    unsigned long flags;
    let mut queued: bool = false;
    int i, j;
    raw_spin_lock_irqsave(&krcp.lock, flags);
// Attempt to start a new batch.
    for (i = 0; i < KFREE_N_BATCHES; i++) {
    struct kfree_rcu_cpu_work *krwp = &(krcp.krw_arr[i]);
// Try to detach bulk_head or head and attach it, only when
// all channels are free.  Any channel is not free means at krwp
// there is on-going rcu work to handle krwp's free business.
    if (need_wait_for_krwp_work(krwp))
    continue;
// kvfree_rcu_drain_ready() might handle this krcp, if so give up.
    if (need_offload_krc(krcp)) {
// Channel 1 corresponds to the SLAB-pointer bulk path.
// Channel 2 corresponds to vmalloc-pointer bulk path.
    for (j = 0; j < FREE_N_CHANNELS; j++) {
    if (list_empty(&krwp.bulk_head_free[j])) {
    atomic_set(&krcp.bulk_count[j], 0);
    list_replace_init(&krcp.bulk_head[j],
    &krwp.bulk_head_free[j]);
    }
    }
// Channel 3 corresponds to both SLAB and vmalloc
// objects queued on the linked list.
    if (!krwp.head_free) {
    krwp.head_free = krcp.head;
    get_state_synchronize_rcu_full(&krwp.head_free_gp_snap);
    atomic_set(&krcp.head_count, 0);
    WRITE_ONCE(krcp.head, core::ptr::null_mut());
    }
// One work is per one batch, so there are three
// "free channels", the batch can handle. Break
// the loop since it is done with this CPU thus
// queuing an RCU work is _always_ success here.
    queued = queue_rcu_work(rcu_reclaim_wq, &krwp.rcu_work);
    WARN_ON_ONCE(!queued);
    break;
    }
    }
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    return queued;
    }
//
// This function is invoked after the KFREE_DRAIN_JIFFIES timeout.
//
#[no_mangle]
unsafe extern "C" fn kfree_rcu_monitor(work: *mut work_struct) {
    static void kfree_rcu_monitor(struct work_struct *work)
    {
    struct kfree_rcu_cpu *krcp = container_of(work,
    struct kfree_rcu_cpu, monitor_work.work);
// Drain ready for reclaim.
    kvfree_rcu_drain_ready(krcp);
// Queue a batch for a rest.
    kvfree_rcu_queue_batch(krcp);
// If there is nothing to detach, it means that our job is
// successfully done here. In case of having at least one
// of the channels that is still busy we should rearm the
// work to repeat an attempt. Because previous batches are
// still in progress.
    if (need_offload_krc(krcp))
    schedule_delayed_monitor_work(krcp);
    }
#[no_mangle]
unsafe extern "C" fn fill_page_cache_func(work: *mut work_struct) {
    static void fill_page_cache_func(struct work_struct *work)
    {
    struct kvfree_rcu_bulk_data *bnode;
    struct kfree_rcu_cpu *krcp =
    container_of(work, struct kfree_rcu_cpu,
    page_cache_work.work);
    unsigned long flags;
    int nr_pages;
    bool pushed;
    int i;
    nr_pages = atomic_read(&krcp.backoff_page_cache_fill) ?
    1 : rcu_min_cached_objs;
    for (i = READ_ONCE(krcp.nr_bkv_objs); i < nr_pages; i++) {
    bnode = (struct kvfree_rcu_bulk_data *)
    __get_free_page(GFP_KERNEL | __GFP_NORETRY | __GFP_NOMEMALLOC | __GFP_NOWARN);
    if (!bnode)
    break;
    raw_spin_lock_irqsave(&krcp.lock, flags);
    pushed = put_cached_bnode(krcp, bnode);
    raw_spin_unlock_irqrestore(&krcp.lock, flags);
    if (!pushed) {
    free_page((unsigned long) bnode);
    break;
    }
    }
    atomic_set(&krcp.work_in_progress, 0);
    atomic_set(&krcp.backoff_page_cache_fill, 0);
    }
// Record ptr in a page managed by krcp, with the pre-krc_this_cpu_lock()
// state specified by flags.  If can_alloc is true, the caller must
// be schedulable and not be holding any locks or mutexes that might be
// acquired by the memory allocator or anything that it might invoke.
// Returns true if ptr was successfully recorded, else the caller must
// use a fallback.
    static inline bool
    add_ptr_to_bulk_krc_lock(struct kfree_rcu_cpu **krcp,
    unsigned long *flags, void *ptr, bool can_alloc)
    {
    struct kvfree_rcu_bulk_data *bnode;
    int idx;
// krcp = krc_this_cpu_lock(flags);
    if (unlikely(!(*krcp).initialized))
    return false;
    idx = !!is_vmalloc_addr(ptr);
    bnode = list_first_entry_or_null(&(*krcp).bulk_head[idx],
    struct kvfree_rcu_bulk_data, list);
// Check if a new block is required.
    if (!bnode || bnode.nr_records == KVFREE_BULK_MAX_ENTR) {
    bnode = get_cached_bnode(*krcp);
    if (!bnode && can_alloc) {
    krc_this_cpu_unlock(*krcp, *flags);
// __GFP_NORETRY - allows a light-weight direct reclaim
// what is OK from minimizing of fallback hitting point of
// view. Apart of that it forbids any OOM invoking what is
// also beneficial since we are about to release memory soon.
//
// __GFP_NOMEMALLOC - prevents from consuming of all the
// memory reserves. Please note we have a fallback path.
//
// __GFP_NOWARN - it is supposed that an allocation can
// be failed under low memory or high memory pressure
// scenarios.
    bnode = (struct kvfree_rcu_bulk_data *)
    __get_free_page(GFP_KERNEL | __GFP_NORETRY | __GFP_NOMEMALLOC | __GFP_NOWARN);
    raw_spin_lock_irqsave(&(*krcp).lock, *flags);
    }
    if (!bnode)
    return false;
// Initialize the new block and attach it.
    bnode.nr_records = 0;
    list_add(&bnode.list, &(*krcp).bulk_head[idx]);
    }
// Finally insert and update the GP for this page.
    bnode.nr_records++;
    bnode.records[bnode.nr_records - 1] = ptr;
    get_state_synchronize_rcu_full(&bnode.gp_snap);
    atomic_inc(&(*krcp).bulk_count[idx]);
    return true;
    }
    static enum hrtimer_restart
    schedule_page_work_fn(struct hrtimer *t)
    {
    struct kfree_rcu_cpu *krcp =
    container_of(t, struct kfree_rcu_cpu, hrtimer);
    queue_delayed_work(system_highpri_wq, &krcp.page_cache_work, 0);
    return HRTIMER_NORESTART;
    }
    static void
    run_page_cache_worker(struct kfree_rcu_cpu *krcp)
    {
// If cache disabled, bail out.
    if (!rcu_min_cached_objs)
    return;
    if (rcu_scheduler_active == RCU_SCHEDULER_RUNNING &&
    !atomic_xchg(&krcp.work_in_progress, 1)) {
    if (atomic_read(&krcp.backoff_page_cache_fill)) {
    queue_delayed_work(rcu_reclaim_wq,
    &krcp.page_cache_work,
    msecs_to_jiffies(rcu_delay_page_cache_fill_msec));
    } else {
    hrtimer_setup(&krcp.hrtimer, schedule_page_work_fn, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
    hrtimer_start(&krcp.hrtimer, 0, HRTIMER_MODE_REL);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kfree_rcu_scheduler_running() -> void __init {
    void __init kfree_rcu_scheduler_running(void)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    struct kfree_rcu_cpu *krcp = per_cpu_ptr(&krc, cpu);
    if (need_offload_krc(krcp))
    schedule_delayed_monitor_work(krcp);
    }
    }
//
// Queue a request for lazy invocation of the appropriate free routine
// after a grace period.  Please note that three paths are maintained,
// two for the common case using arrays of pointers and a third one that
// is used only when the main paths cannot be used, for example, due to
// memory pressure.
//
// Each kvfree_call_rcu() request is added to a batch. The batch will be drained
// every KFREE_DRAIN_JIFFIES number of jiffies. All the objects in the batch will
// be free'd in workqueue context. This allows us to: batch requests together to
// reduce the number of grace periods during heavy kfree_rcu()/kvfree_rcu() load.
//
#[no_mangle]
pub unsafe extern "C" fn kvfree_call_rcu(head: *mut kvfree_rcu_head, ptr: *mut c_void) {
    void kvfree_call_rcu(struct kvfree_rcu_head *head, void *ptr)
    {
    unsigned long flags;
    struct kfree_rcu_cpu *krcp;
    bool success;
//
// Please note there is a limitation for the head-less
// variant, that is why there is a clear rule for such
// objects: it can be used from might_sleep() context
// only. For other places please embed an rcu_head to
// your data.
//
    if (!head)
    might_sleep();
    if (kfree_rcu_sheaf(ptr))
    return;
// Queue the object but don't yet schedule the batch.
    if (debug_rcu_head_queue(ptr)) {
// Probable double kfree_rcu(), just leak.
    WARN_ONCE(1, "%s(): Double-freed call. rcu_head %p\n",
    __func__, head);
// Mark as success and leave.
    return;
    }
    kasan_record_aux_stack(ptr);
    success = add_ptr_to_bulk_krc_lock(&krcp, &flags, ptr, !head);
    if (!success) {
    run_page_cache_worker(krcp);
    if (head == core::ptr::null_mut())
// Inline if kvfree_rcu(one_arg) call.
    goto unlock_return;
    head.next = krcp.head;
    WRITE_ONCE(krcp.head, head);
    atomic_inc(&krcp.head_count);
// Take a snapshot for this krcp.
    krcp.head_gp_snap = get_state_synchronize_rcu();
    success = true;
    }
//
// The kvfree_rcu() caller considers the pointer freed at this point
// and likely removes any references to it. Since the actual slab
// freeing (and kmemleak_free()) is deferred, tell kmemleak to ignore
// this object (no scanning or false positives reporting).
//
    kmemleak_ignore(ptr);
// Set timer to drain after KFREE_DRAIN_JIFFIES.
    if (rcu_scheduler_active == RCU_SCHEDULER_RUNNING)
    __schedule_delayed_monitor_work(krcp);
    unlock_return:
    krc_this_cpu_unlock(krcp, flags);
//
// Inline kvfree() after synchronize_rcu(). We can do
// it from might_sleep() context only, so the current
// CPU can pass the QS state.
//
    if (!success) {
    debug_rcu_head_unqueue((struct rcu_head *) ptr);
    synchronize_rcu();
    kvfree(ptr);
    }
    }
    EXPORT_SYMBOL_GPL(kvfree_call_rcu);
#[no_mangle]
pub unsafe extern "C" fn __kvfree_rcu_barrier() {
    static inline void __kvfree_rcu_barrier(void)
    {
    struct kfree_rcu_cpu_work *krwp;
    struct kfree_rcu_cpu *krcp;
    bool queued;
    int i, cpu;
//
// Firstly we detach objects and queue them over an RCU-batch
// for all CPUs. Finally queued works are flushed for each CPU.
//
// Please note. If there are outstanding batches for a particular
// CPU, those have to be finished first following by queuing a new.
//
    for_each_possible_cpu(cpu) {
    krcp = per_cpu_ptr(&krc, cpu);
//
// Check if this CPU has any objects which have been queued for a
// new GP completion. If not(means nothing to detach), we are done
// with it. If any batch is pending/running for this "krcp", below
// per-cpu flush_rcu_work() waits its completion(see last step).
//
    if (!need_offload_krc(krcp))
    continue;
    while (1) {
//
// If we are not able to queue a new RCU work it means:
// - batches for this CPU are still in flight which should
// be flushed first and then repeat;
// - no objects to detach, because of concurrency.
//
    queued = kvfree_rcu_queue_batch(krcp);
//
// Bail out, if there is no need to offload this "krcp"
// anymore. As noted earlier it can run concurrently.
//
    if (queued || !need_offload_krc(krcp))
    break;
// There are ongoing batches.
    for (i = 0; i < KFREE_N_BATCHES; i++) {
    krwp = &(krcp.krw_arr[i]);
    flush_rcu_work(&krwp.rcu_work);
    }
    }
    }
//
// Now we guarantee that all objects are flushed.
//
    for_each_possible_cpu(cpu) {
    krcp = per_cpu_ptr(&krc, cpu);
//
// A monitor work can drain ready to reclaim objects
// directly. Wait its completion if running or pending.
//
    cancel_delayed_work_sync(&krcp.monitor_work);
    for (i = 0; i < KFREE_N_BATCHES; i++) {
    krwp = &(krcp.krw_arr[i]);
    flush_rcu_work(&krwp.rcu_work);
    }
    }
    }
//
// kvfree_rcu_barrier - Wait until all in-flight kvfree_rcu() complete.
//
// Note that a single argument of kvfree_rcu() call has a slow path that
// triggers synchronize_rcu() following by freeing a pointer. It is done
// before the return from the function. Therefore for any single-argument
// call that will result in a kfree() to a cache that is to be destroyed
// during module exit, it is developer's responsibility to ensure that all
// such calls have returned before the call to kmem_cache_destroy().
//
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_barrier() {
    void kvfree_rcu_barrier(void)
    {
    flush_all_rcu_sheaves();
    __kvfree_rcu_barrier();
    }
//
// kvfree_rcu_barrier_on_cache - Wait for in-flight kvfree_rcu() calls on a
// specific slab cache.
// @s: slab cache to wait for
//
// See the description of kvfree_rcu_barrier() for details.
//
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_barrier_on_cache(s: *mut kmem_cache) {
    void kvfree_rcu_barrier_on_cache(struct kmem_cache *s)
    {
// kfree_rcu_nolock() might have deferred frees even without sheaves
    deferred_work_barrier();
    if (cache_has_sheaves(s)) {
    cpus_read_lock();
    flush_rcu_sheaves_on_cache(s);
    cpus_read_unlock();
    }
    rcu_barrier();
    __kvfree_rcu_barrier();
    }
    static unsigned long
    kfree_rcu_shrink_count(struct shrinker *shrink, struct shrink_control *sc)
    {
    int cpu;
    let mut count: c_ulong = 0;
// Snapshot count of all CPUs
    for_each_possible_cpu(cpu) {
    struct kfree_rcu_cpu *krcp = per_cpu_ptr(&krc, cpu);
    count += krc_count(krcp);
    count += READ_ONCE(krcp.nr_bkv_objs);
    atomic_set(&krcp.backoff_page_cache_fill, 1);
    }
    let mut count: return = = 0 ? SHRINK_EMPTY : count;
    }
    static unsigned long
    kfree_rcu_shrink_scan(struct shrinker *shrink, struct shrink_control *sc)
    {
    int cpu, freed = 0;
    for_each_possible_cpu(cpu) {
    int count;
    struct kfree_rcu_cpu *krcp = per_cpu_ptr(&krc, cpu);
    count = krc_count(krcp);
    count += drain_page_cache(krcp);
    kfree_rcu_monitor(&krcp.monitor_work.work);
    sc.nr_to_scan -= count;
    freed += count;
    if (sc.nr_to_scan <= 0)
    break;
    }
    let mut freed: return = = 0 ? SHRINK_STOP : freed;
    }
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_init() -> void __init {
    void __init kvfree_rcu_init(void)
    {
    int cpu;
    int i, j;
    struct shrinker *kfree_rcu_shrinker;
    rcu_reclaim_wq = alloc_workqueue("kvfree_rcu_reclaim",
    WQ_UNBOUND | WQ_MEM_RECLAIM, 0);
    WARN_ON(!rcu_reclaim_wq);
// Clamp it to [0:100] seconds interval.
    if (rcu_delay_page_cache_fill_msec < 0 ||
    rcu_delay_page_cache_fill_msec > 100 * MSEC_PER_SEC) {
    rcu_delay_page_cache_fill_msec =
    clamp(rcu_delay_page_cache_fill_msec, 0,
    (int) (100 * MSEC_PER_SEC));
    pr_info("Adjusting rcutree.rcu_delay_page_cache_fill_msec to %d ms.\n",
    rcu_delay_page_cache_fill_msec);
    }
    for_each_possible_cpu(cpu) {
    struct kfree_rcu_cpu *krcp = per_cpu_ptr(&krc, cpu);
    for (i = 0; i < KFREE_N_BATCHES; i++) {
    INIT_RCU_WORK(&krcp.krw_arr[i].rcu_work, kfree_rcu_work);
    krcp.krw_arr[i].krcp = krcp;
    for (j = 0; j < FREE_N_CHANNELS; j++)
    INIT_LIST_HEAD(&krcp.krw_arr[i].bulk_head_free[j]);
    }
    for (i = 0; i < FREE_N_CHANNELS; i++)
    INIT_LIST_HEAD(&krcp.bulk_head[i]);
    INIT_DELAYED_WORK(&krcp.monitor_work, kfree_rcu_monitor);
    INIT_DELAYED_WORK(&krcp.page_cache_work, fill_page_cache_func);
    krcp.initialized = true;
    }
    kfree_rcu_shrinker = shrinker_alloc(0, "slab-kvfree-rcu");
    if (!kfree_rcu_shrinker) {
    pr_err("Failed to allocate kfree_rcu() shrinker!\n");
    return;
    }
    kfree_rcu_shrinker.count_objects = kfree_rcu_shrink_count;
    kfree_rcu_shrinker.scan_objects = kfree_rcu_shrink_scan;
    shrinker_register(kfree_rcu_shrinker);
    }
