//! Automatically rewritten from C to Rust
//! Source: kernel/jump_label.c
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
//
// jump label support
//
// Copyright (C) 2009 Jason Baron <jbaron@redhat.com>
// Copyright (C) 2011 Peter Zijlstra
//

// mutex to protect coming/going of the jump_label table
    static DEFINE_MUTEX(jump_label_mutex);
#[no_mangle]
pub unsafe extern "C" fn jump_label_lock() {
    void jump_label_lock(void)
    {
    mutex_lock(&jump_label_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn jump_label_unlock() {
    void jump_label_unlock(void)
    {
    mutex_unlock(&jump_label_mutex);
    }
#[no_mangle]
unsafe extern "C" fn jump_label_cmp(a: *const c_void, b: *const c_void) -> c_int {
    static int jump_label_cmp(const void *a, const void *b)
    {
    const struct jump_entry *jea = a;
    const struct jump_entry *jeb = b;
//
// Entrires are sorted by key.
//
    if (jump_entry_key(jea) < jump_entry_key(jeb))
    return -1;
    if (jump_entry_key(jea) > jump_entry_key(jeb))
    return 1;
//
// In the batching mode, entries should also be sorted by the code
// inside the already sorted list of entries, enabling a bsearch in
// the vector.
//
    if (jump_entry_code(jea) < jump_entry_code(jeb))
    return -1;
    if (jump_entry_code(jea) > jump_entry_code(jeb))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jump_label_swap(a: *mut c_void, b: *mut c_void, size: c_int) {
    static void jump_label_swap(void *a, void *b, int size)
    {
    let mut delta: c_long = (unsigned long)a - (unsigned long)b;
    struct jump_entry *jea = a;
    struct jump_entry *jeb = b;
    let mut tmp: jump_entry = *jea;
    jea.code	= jeb.code - delta;
    jea.target	= jeb.target - delta;
    jea.key	= jeb.key - delta;
    jeb.code	= tmp.code + delta;
    jeb.target	= tmp.target + delta;
    jeb.key	= tmp.key + delta;
    }
    static void
    jump_label_sort_entries(struct jump_entry *start, struct jump_entry *stop)
    {
    unsigned long size;
    void *swapfn = core::ptr::null_mut();
    if (IS_ENABLED(CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE))
    swapfn = jump_label_swap;
    size = (((unsigned long)stop - (unsigned long)start)
    / sizeof(struct jump_entry));
    sort(start, size, sizeof(struct jump_entry), jump_label_cmp, swapfn);
    }
    static void jump_label_update(struct static_key *key);
//
// There are similar definitions for the !CONFIG_JUMP_LABEL case in jump_label.h.
// The use of 'atomic_read()' requires atomic.h and its problematic for some
// kernel headers such as kernel.h and others. Since static_key_count() is not
// used in the branch statements as it is for the !CONFIG_JUMP_LABEL case its ok
// to have it be a function here. Similarly, for 'static_key_enable()' and
// 'static_key_disable()', which require bug.h. This should allow jump_label.h
// to be included from most/all places for CONFIG_JUMP_LABEL.
//
#[no_mangle]
pub unsafe extern "C" fn static_key_count(key: *mut static_key) -> c_int {
    int static_key_count(struct static_key *key)
    {
//
// -1 means the first static_key_slow_inc() is in progress.
// static_key_enabled() must return true, so return 1 here.
//
    let mut n: c_int = atomic_read(&key.enabled);
    return n >= 0 ? n : 1;
    }
    EXPORT_SYMBOL_GPL(static_key_count);
//
// static_key_fast_inc_not_disabled - adds a user for a static key
// @key: static key that must be already enabled
//
// The caller must make sure that the static key can't get disabled while
// in this function. It doesn't patch jump labels, only adds a user to
// an already enabled static key.
//
// Returns true if the increment was done. Unlike refcount_t the ref counter
// is not saturated, but will fail to increment on overflow.
//
#[no_mangle]
pub unsafe extern "C" fn static_key_fast_inc_not_disabled(key: *mut static_key) -> bool {
    bool static_key_fast_inc_not_disabled(struct static_key *key)
    {
    int v;
    STATIC_KEY_CHECK_USE(key);
//
// Negative key->enabled has a special meaning: it sends
// static_key_slow_inc/dec() down the slow path, and it is non-zero
// so it counts as "enabled" in jump_label_update().
//
// The INT_MAX overflow condition is either used by the networking
// code to reset or detected in the slow path of
// static_key_slow_inc_cpuslocked().
//
    v = atomic_read(&key.enabled);
    do {
    if (v <= 0 || v == INT_MAX)
    return false;
    } while (!likely(atomic_try_cmpxchg(&key.enabled, &v, v + 1)));
    return true;
    }
    EXPORT_SYMBOL_GPL(static_key_fast_inc_not_disabled);
#[no_mangle]
pub unsafe extern "C" fn static_key_slow_inc_cpuslocked(key: *mut static_key) -> bool {
    bool static_key_slow_inc_cpuslocked(struct static_key *key)
    {
    lockdep_assert_cpus_held();
//
// Careful if we get concurrent static_key_slow_inc/dec() calls;
// later calls must wait for the first one to _finish_ the
// jump_label_update() process.  At the same time, however,
// the jump_label_update() call below wants to see
// static_key_enabled(&key) for jumps to be updated properly.
//
    if (static_key_fast_inc_not_disabled(key))
    return true;
    guard(mutex)(&jump_label_mutex);
// Try to mark it as 'enabling in progress.
    if (!atomic_cmpxchg(&key.enabled, 0, -1)) {
    jump_label_update(key);
//
// Ensure that when static_key_fast_inc_not_disabled() or
// static_key_dec_not_one() observe the positive value,
// they must also observe all the text changes.
//
    atomic_set_release(&key.enabled, 1);
    } else {
//
// While holding the mutex this should never observe
// anything else than a value >= 1 and succeed
//
    if (WARN_ON_ONCE(!static_key_fast_inc_not_disabled(key)))
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_slow_inc(key: *mut static_key) -> bool {
    bool static_key_slow_inc(struct static_key *key)
    {
    bool ret;
    cpus_read_lock();
    ret = static_key_slow_inc_cpuslocked(key);
    cpus_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(static_key_slow_inc);
#[no_mangle]
pub unsafe extern "C" fn static_key_enable_cpuslocked(key: *mut static_key) {
    void static_key_enable_cpuslocked(struct static_key *key)
    {
    STATIC_KEY_CHECK_USE(key);
    lockdep_assert_cpus_held();
    if (atomic_read(&key.enabled) > 0) {
    WARN_ON_ONCE(atomic_read(&key.enabled) != 1);
    return;
    }
    jump_label_lock();
    if (atomic_read(&key.enabled) == 0) {
    atomic_set(&key.enabled, -1);
    jump_label_update(key);
//
// See static_key_slow_inc().
//
    atomic_set_release(&key.enabled, 1);
    }
    jump_label_unlock();
    }
    EXPORT_SYMBOL_GPL(static_key_enable_cpuslocked);
#[no_mangle]
pub unsafe extern "C" fn static_key_enable(key: *mut static_key) {
    void static_key_enable(struct static_key *key)
    {
    cpus_read_lock();
    static_key_enable_cpuslocked(key);
    cpus_read_unlock();
    }
    EXPORT_SYMBOL_GPL(static_key_enable);
#[no_mangle]
pub unsafe extern "C" fn static_key_disable_cpuslocked(key: *mut static_key) {
    void static_key_disable_cpuslocked(struct static_key *key)
    {
    STATIC_KEY_CHECK_USE(key);
    lockdep_assert_cpus_held();
    if (atomic_read(&key.enabled) != 1) {
    WARN_ON_ONCE(atomic_read(&key.enabled) != 0);
    return;
    }
    jump_label_lock();
    if (atomic_cmpxchg(&key.enabled, 1, 0) == 1)
    jump_label_update(key);
    jump_label_unlock();
    }
    EXPORT_SYMBOL_GPL(static_key_disable_cpuslocked);
#[no_mangle]
pub unsafe extern "C" fn static_key_disable(key: *mut static_key) {
    void static_key_disable(struct static_key *key)
    {
    cpus_read_lock();
    static_key_disable_cpuslocked(key);
    cpus_read_unlock();
    }
    EXPORT_SYMBOL_GPL(static_key_disable);
#[no_mangle]
unsafe extern "C" fn static_key_dec_not_one(key: *mut static_key) -> bool {
    static bool static_key_dec_not_one(struct static_key *key)
    {
    int v;
//
// Go into the slow path if key::enabled is less than or equal than
// one. One is valid to shut down the key, anything less than one
// is an imbalance, which is handled at the call site.
//
// That includes the special case of '-1' which is set in
// static_key_slow_inc_cpuslocked(), but that's harmless as it is
// fully serialized in the slow path below. By the time this task
// acquires the jump label lock the value is back to one and the
// retry under the lock must succeed.
//
    v = atomic_read(&key.enabled);
    do {
//
// Warn about the '-1' case though; since that means a
// decrement is concurrent with a first (0->1) increment. IOW
// people are trying to disable something that wasn't yet fully
// enabled. This suggests an ordering problem on the user side.
//
    WARN_ON_ONCE(v < 0);
//
// Warn about underflow, and lie about success in an attempt to
// not make things worse.
//
    if (WARN_ON_ONCE(v == 0))
    return true;
    if (v <= 1)
    return false;
    } while (!likely(atomic_try_cmpxchg(&key.enabled, &v, v - 1)));
    return true;
    }
#[no_mangle]
unsafe extern "C" fn __static_key_slow_dec_cpuslocked(key: *mut static_key) {
    static void __static_key_slow_dec_cpuslocked(struct static_key *key)
    {
    lockdep_assert_cpus_held();
    int val;
    if (static_key_dec_not_one(key))
    return;
    guard(mutex)(&jump_label_mutex);
    val = atomic_read(&key.enabled);
//
// It should be impossible to observe -1 with jump_label_mutex held,
// see static_key_slow_inc_cpuslocked().
//
    if (WARN_ON_ONCE(val == -1))
    return;
//
// Cannot already be 0, something went sideways.
//
    if (WARN_ON_ONCE(val == 0))
    return;
    if (atomic_dec_and_test(&key.enabled))
    jump_label_update(key);
    }
#[no_mangle]
unsafe extern "C" fn __static_key_slow_dec(key: *mut static_key) {
    static void __static_key_slow_dec(struct static_key *key)
    {
    cpus_read_lock();
    __static_key_slow_dec_cpuslocked(key);
    cpus_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn jump_label_update_timeout(work: *mut work_struct) {
    void jump_label_update_timeout(struct work_struct *work)
    {
    struct static_key_deferred *key =
    container_of(work, struct static_key_deferred, work.work);
    __static_key_slow_dec(&key.key);
    }
    EXPORT_SYMBOL_GPL(jump_label_update_timeout);
#[no_mangle]
pub unsafe extern "C" fn static_key_slow_dec(key: *mut static_key) {
    void static_key_slow_dec(struct static_key *key)
    {
    STATIC_KEY_CHECK_USE(key);
    __static_key_slow_dec(key);
    }
    EXPORT_SYMBOL_GPL(static_key_slow_dec);
#[no_mangle]
pub unsafe extern "C" fn static_key_slow_dec_cpuslocked(key: *mut static_key) {
    void static_key_slow_dec_cpuslocked(struct static_key *key)
    {
    STATIC_KEY_CHECK_USE(key);
    __static_key_slow_dec_cpuslocked(key);
    }
    void __static_key_slow_dec_deferred(struct static_key *key,
    struct delayed_work *work,
    unsigned long timeout)
    {
    STATIC_KEY_CHECK_USE(key);
    if (static_key_dec_not_one(key))
    return;
    schedule_delayed_work(work, timeout);
    }
    EXPORT_SYMBOL_GPL(__static_key_slow_dec_deferred);
#[no_mangle]
pub unsafe extern "C" fn __static_key_deferred_flush(key: *mut c_void, work: *mut delayed_work) {
    void __static_key_deferred_flush(void *key, struct delayed_work *work)
    {
    STATIC_KEY_CHECK_USE(key);
    flush_delayed_work(work);
    }
    EXPORT_SYMBOL_GPL(__static_key_deferred_flush);
    void jump_label_rate_limit(struct static_key_deferred *key,
    unsigned long rl)
    {
    STATIC_KEY_CHECK_USE(key);
    key.timeout = rl;
    INIT_DELAYED_WORK(&key.work, jump_label_update_timeout);
    }
    EXPORT_SYMBOL_GPL(jump_label_rate_limit);
#[no_mangle]
unsafe extern "C" fn addr_conflict(entry: *mut jump_entry, start: *mut c_void, end: *mut c_void) -> c_int {
    static int addr_conflict(struct jump_entry *entry, void *start, void *end)
    {
    if (jump_entry_code(entry) <= (unsigned long)end &&
    jump_entry_code(entry) + jump_entry_size(entry) > (unsigned long)start)
    return 1;
    return 0;
    }
    static int __jump_label_text_reserved(struct jump_entry *iter_start,
    struct jump_entry *iter_stop, void *start, void *end, bool init)
    {
    struct jump_entry *iter;
    iter = iter_start;
    while (iter < iter_stop) {
    if (init || !jump_entry_is_init(iter)) {
    if (addr_conflict(iter, start, end))
    return 1;
    }
    iter++;
    }
    return 0;
    }

    static void arch_jump_label_transform_static(struct jump_entry *entry,
    enum jump_label_type type)
    {
// nothing to do on most architectures
    }

    static inline struct jump_entry *static_key_entries(struct static_key *key)
    {
    WARN_ON_ONCE(key.type & JUMP_TYPE_LINKED);
    return (struct jump_entry *)(key.type & ~JUMP_TYPE_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_type(key: *mut static_key) -> bool {
    static inline bool static_key_type(struct static_key *key)
    {
    return key.type & JUMP_TYPE_TRUE;
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_linked(key: *mut static_key) -> bool {
    static inline bool static_key_linked(struct static_key *key)
    {
    return key.type & JUMP_TYPE_LINKED;
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_clear_linked(key: *mut static_key) {
    static inline void static_key_clear_linked(struct static_key *key)
    {
    key.type &= ~JUMP_TYPE_LINKED;
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_set_linked(key: *mut static_key) {
    static inline void static_key_set_linked(struct static_key *key)
    {
    key.type |= JUMP_TYPE_LINKED;
    }
//
// A 'struct static_key' uses a union such that it either points directly
// to a table of 'struct jump_entry' or to a linked list of modules which in
// turn point to 'struct jump_entry' tables.
//
// The two lower bits of the pointer are used to keep track of which pointer
// type is in use and to store the initial branch direction, we use an access
// function which preserves these bits.
//
    static void static_key_set_entries(struct static_key *key,
    struct jump_entry *entries)
    {
    unsigned long type;
    WARN_ON_ONCE((unsigned long)entries & JUMP_TYPE_MASK);
    type = key.type & JUMP_TYPE_MASK;
    key.entries = entries;
    key.type |= type;
    }
#[no_mangle]
unsafe extern "C" fn jump_label_type(entry: *mut jump_entry) -> enum jump_label_type {
    static enum jump_label_type jump_label_type(struct jump_entry *entry)
    {
    struct static_key *key = jump_entry_key(entry);
    let mut enabled: bool = static_key_enabled(key);
    let mut branch: bool = jump_entry_is_branch(entry);
// See the comment in linux/jump_label.h
    return enabled ^ branch;
    }
#[no_mangle]
unsafe extern "C" fn jump_label_can_update(entry: *mut jump_entry, init: bool) -> bool {
    static bool jump_label_can_update(struct jump_entry *entry, bool init)
    {
//
// Cannot update code that was in an init text area.
//
    if (!init && jump_entry_is_init(entry))
    return false;
    if (!kernel_text_address(jump_entry_code(entry))) {
//
// This skips patching built-in __exit, which
// is part of init_section_contains() but is
// not part of kernel_text_address().
//
// Skipping built-in __exit is fine since it
// will never be executed.
//
    WARN_ONCE(!jump_entry_is_init(entry),
    "can't patch jump_label at %pS",
    (void *)jump_entry_code(entry));
    return false;
    }
    return true;
    }

    static void __jump_label_update(struct static_key *key,
    struct jump_entry *entry,
    struct jump_entry *stop,
    bool init)
    {
    for (; (entry < stop) && (jump_entry_key(entry) == key); entry++) {
    if (jump_label_can_update(entry, init))
    arch_jump_label_transform(entry, jump_label_type(entry));
    }
    }

    static void __jump_label_update(struct static_key *key,
    struct jump_entry *entry,
    struct jump_entry *stop,
    bool init)
    {
    for (; (entry < stop) && (jump_entry_key(entry) == key); entry++) {
    if (!jump_label_can_update(entry, init))
    continue;
    if (!arch_jump_label_transform_queue(entry, jump_label_type(entry))) {
//
// Queue is full: Apply the current queue and try again.
//
    arch_jump_label_transform_apply();
    BUG_ON(!arch_jump_label_transform_queue(entry, jump_label_type(entry)));
    }
    }
    arch_jump_label_transform_apply();
    }

#[no_mangle]
pub unsafe extern "C" fn jump_label_init() -> void __init {
    void __init jump_label_init(void)
    {
    struct jump_entry *iter_start = __start___jump_table;
    struct jump_entry *iter_stop = __stop___jump_table;
    struct static_key *key = core::ptr::null_mut();
    struct jump_entry *iter;
    if (static_key_initialized)
    return;
    cpus_read_lock();
    jump_label_lock();
    jump_label_sort_entries(iter_start, iter_stop);
    for (iter = iter_start; iter < iter_stop; iter++) {
    struct static_key *iterk;
    bool in_init;
// rewrite NOPs
    if (jump_label_type(iter) == JUMP_LABEL_NOP)
    arch_jump_label_transform_static(iter, JUMP_LABEL_NOP);
    in_init = init_section_contains((void *)jump_entry_code(iter), 1);
    jump_entry_set_init(iter, in_init);
    iterk = jump_entry_key(iter);
    if (iterk == key)
    continue;
    key = iterk;
    static_key_set_entries(key, iter);
    }
    static_key_initialized = true;
    jump_label_unlock();
    cpus_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_sealed(key: *mut static_key) -> bool {
    static inline bool static_key_sealed(struct static_key *key)
    {
    return (key.type & JUMP_TYPE_LINKED) && !(key.type & ~JUMP_TYPE_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn static_key_seal(key: *mut static_key) {
    static inline void static_key_seal(struct static_key *key)
    {
    let mut type: c_ulong = key.type & JUMP_TYPE_TRUE;
    key.type = JUMP_TYPE_LINKED | type;
    }
#[no_mangle]
pub unsafe extern "C" fn jump_label_init_ro() {
    void jump_label_init_ro(void)
    {
    struct jump_entry *iter_start = __start___jump_table;
    struct jump_entry *iter_stop = __stop___jump_table;
    struct jump_entry *iter;
    if (WARN_ON_ONCE(!static_key_initialized))
    return;
    cpus_read_lock();
    jump_label_lock();
    for (iter = iter_start; iter < iter_stop; iter++) {
    struct static_key *iterk = jump_entry_key(iter);
    if (!is_kernel_ro_after_init((unsigned long)iterk))
    continue;
    if (static_key_sealed(iterk))
    continue;
    static_key_seal(iterk);
    }
    jump_label_unlock();
    cpus_read_unlock();
    }

#[no_mangle]
pub unsafe extern "C" fn jump_label_init_type(entry: *mut jump_entry) -> enum jump_label_type {
    enum jump_label_type jump_label_init_type(struct jump_entry *entry)
    {
    struct static_key *key = jump_entry_key(entry);
    let mut type: bool = static_key_type(key);
    let mut branch: bool = jump_entry_is_branch(entry);
// See the comment in linux/jump_label.h
    return type ^ branch;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_mod {
    pub next: *mut static_key_mod,
    pub entries: *mut jump_entry,
    pub mod: *mut module,
}

    static inline struct static_key_mod *static_key_mod(struct static_key *key)
    {
    WARN_ON_ONCE(!static_key_linked(key));
    return (struct static_key_mod *)(key.type & ~JUMP_TYPE_MASK);
    }
//
// key->type and key->next are the same via union.
// This sets key->next and preserves the type bits.
//
// See additional comments above static_key_set_entries().
//
    static void static_key_set_mod(struct static_key *key,
    struct static_key_mod *mod)
    {
    unsigned long type;
    WARN_ON_ONCE((unsigned long)mod & JUMP_TYPE_MASK);
    type = key.type & JUMP_TYPE_MASK;
    key.next = mod;
    key.type |= type;
    }
#[no_mangle]
unsafe extern "C" fn __jump_label_mod_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
    static int __jump_label_mod_text_reserved(void *start, void *end)
    {
    struct module *mod;
    int ret;
    scoped_guard(rcu) {
    mod = __module_text_address((unsigned long)start);
    WARN_ON_ONCE(__module_text_address((unsigned long)end) != mod);
    if (!try_module_get(mod))
    mod = core::ptr::null_mut();
    }
    if (!mod)
    return 0;
    ret = __jump_label_text_reserved(mod.jump_entries,
    mod.jump_entries + mod.num_jump_entries,
    start, end, mod.state == MODULE_STATE_COMING);
    module_put(mod);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __jump_label_mod_update(key: *mut static_key) {
    static void __jump_label_mod_update(struct static_key *key)
    {
    struct static_key_mod *mod;
    for (mod = static_key_mod(key); mod; mod = mod.next) {
    struct jump_entry *stop;
    struct module *m;
//
// NULL if the static_key is defined in a module
// that does not use it
//
    if (!mod.entries)
    continue;
    m = mod.mod;
    if (!m)
    stop = __stop___jump_table;
    else
    stop = m.jump_entries + m.num_jump_entries;
    __jump_label_update(key, mod.entries, stop,
    m && m.state == MODULE_STATE_COMING);
    }
    }
#[no_mangle]
unsafe extern "C" fn jump_label_add_module(mod: *mut module) -> c_int {
    static int jump_label_add_module(struct module *mod)
    {
    struct jump_entry *iter_start = mod.jump_entries;
    struct jump_entry *iter_stop = iter_start + mod.num_jump_entries;
    struct jump_entry *iter;
    struct static_key *key = core::ptr::null_mut();
    struct static_key_mod *jlm, *jlm2;
// if the module doesn't have jump label entries, just return
    if (iter_start == iter_stop)
    return 0;
    jump_label_sort_entries(iter_start, iter_stop);
    for (iter = iter_start; iter < iter_stop; iter++) {
    struct static_key *iterk;
    bool in_init;
    in_init = within_module_init(jump_entry_code(iter), mod);
    jump_entry_set_init(iter, in_init);
    iterk = jump_entry_key(iter);
    if (iterk == key)
    continue;
    key = iterk;
    if (within_module((unsigned long)key, mod)) {
    static_key_set_entries(key, iter);
    continue;
    }
//
// If the key was sealed at init, then there's no need to keep a
// reference to its module entries - just patch them now and be
// done with it.
//
    if (static_key_sealed(key))
    goto do_poke;
    jlm = kzalloc_obj(struct static_key_mod);
    if (!jlm)
    return -ENOMEM;
    if (!static_key_linked(key)) {
    jlm2 = kzalloc_obj(struct static_key_mod);
    if (!jlm2) {
    kfree(jlm);
    return -ENOMEM;
    }
    scoped_guard(rcu)
    jlm2.mod = __module_address((unsigned long)key);
    jlm2.entries = static_key_entries(key);
    jlm2.next = core::ptr::null_mut();
    static_key_set_mod(key, jlm2);
    static_key_set_linked(key);
    }
    jlm.mod = mod;
    jlm.entries = iter;
    jlm.next = static_key_mod(key);
    static_key_set_mod(key, jlm);
    static_key_set_linked(key);
// Only update if we've changed from our initial state
    do_poke:
    if (jump_label_type(iter) != jump_label_init_type(iter))
    __jump_label_update(key, iter, iter_stop, true);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jump_label_del_module(mod: *mut module) {
    static void jump_label_del_module(struct module *mod)
    {
    struct jump_entry *iter_start = mod.jump_entries;
    struct jump_entry *iter_stop = iter_start + mod.num_jump_entries;
    struct jump_entry *iter;
    struct static_key *key = core::ptr::null_mut();
    struct static_key_mod *jlm, **prev;
    for (iter = iter_start; iter < iter_stop; iter++) {
    if (jump_entry_key(iter) == key)
    continue;
    key = jump_entry_key(iter);
    if (within_module((unsigned long)key, mod))
    continue;
// No @jlm allocated because key was sealed at init.
    if (static_key_sealed(key))
    continue;
// No memory during module load
    if (WARN_ON(!static_key_linked(key)))
    continue;
    prev = &key.next;
    jlm = static_key_mod(key);
    while (jlm && jlm.mod != mod) {
    prev = &jlm.next;
    jlm = jlm.next;
    }
// No memory during module load
    if (WARN_ON(!jlm))
    continue;
    if (prev == &key.next)
    static_key_set_mod(key, jlm.next);
    else
// prev = jlm->next;
    kfree(jlm);
    jlm = static_key_mod(key);
// if only one etry is left, fold it back into the static_key
    if (jlm.next == core::ptr::null_mut()) {
    static_key_set_entries(key, jlm.entries);
    static_key_clear_linked(key);
    kfree(jlm);
    }
    }
    }
    static int
    jump_label_module_notify(struct notifier_block *self, unsigned long val,
    void *data)
    {
    struct module *mod = data;
    let mut ret: c_int = 0;
    cpus_read_lock();
    jump_label_lock();
    switch (val) {
    case MODULE_STATE_COMING:
    ret = jump_label_add_module(mod);
    if (ret) {
    WARN(1, "Failed to allocate memory: jump_label may not work properly.\n");
    jump_label_del_module(mod);
    }
    break;
    case MODULE_STATE_GOING:
    jump_label_del_module(mod);
    break;
    }
    jump_label_unlock();
    cpus_read_unlock();
    return notifier_from_errno(ret);
    }
    static struct notifier_block jump_label_module_nb = {
    .notifier_call = jump_label_module_notify,
    .priority = 1, /* higher than tracepoints */
    };
#[no_mangle]
unsafe extern "C" fn jump_label_init_module() -> __init int {
    static __init int jump_label_init_module(void)
    {
    return register_module_notifier(&jump_label_module_nb);
    }
    early_initcall(jump_label_init_module);

//
// jump_label_text_reserved - check if addr range is reserved
// @start: start text addr
// @end: end text addr
//
// checks if the text addr located between @start and @end
// overlaps with any of the jump label patch addresses. Code
// that wants to modify kernel text should first verify that
// it does not overlap with any of the jump label addresses.
// Caller must hold jump_label_mutex.
//
// returns 1 if there is an overlap, 0 otherwise
//
#[no_mangle]
pub unsafe extern "C" fn jump_label_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
    int jump_label_text_reserved(void *start, void *end)
    {
    let mut init: bool = system_state < SYSTEM_RUNNING;
    int ret = __jump_label_text_reserved(__start___jump_table,
    __stop___jump_table, start, end, init);
    if (ret)
    return ret;

    ret = __jump_label_mod_text_reserved(start, end);

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jump_label_update(key: *mut static_key) {
    static void jump_label_update(struct static_key *key)
    {
    struct jump_entry *stop = __stop___jump_table;
    let mut init: bool = system_state < SYSTEM_RUNNING;
    struct jump_entry *entry;

    struct module *mod;
    if (static_key_linked(key)) {
    __jump_label_mod_update(key);
    return;
    }
    scoped_guard(rcu) {
    mod = __module_address((unsigned long)key);
    if (mod) {
    stop = mod.jump_entries + mod.num_jump_entries;
    init = mod.state == MODULE_STATE_COMING;
    }
    }

    entry = static_key_entries(key);
// if there are no users, entry can be NULL
    if (entry)
    __jump_label_update(key, entry, stop, init);
    }

    static DEFINE_STATIC_KEY_TRUE(sk_true);
    static DEFINE_STATIC_KEY_FALSE(sk_false);
#[no_mangle]
unsafe extern "C" fn jump_label_test() -> __init int {
    static __init int jump_label_test(void)
    {
    int i;
    for (i = 0; i < 2; i++) {
    WARN_ON(static_key_enabled(&sk_true.key) != true);
    WARN_ON(static_key_enabled(&sk_false.key) != false);
    WARN_ON(!static_branch_likely(&sk_true));
    WARN_ON(!static_branch_unlikely(&sk_true));
    WARN_ON(static_branch_likely(&sk_false));
    WARN_ON(static_branch_unlikely(&sk_false));
    static_branch_disable(&sk_true);
    static_branch_enable(&sk_false);
    WARN_ON(static_key_enabled(&sk_true.key) == true);
    WARN_ON(static_key_enabled(&sk_false.key) == false);
    WARN_ON(static_branch_likely(&sk_true));
    WARN_ON(static_branch_unlikely(&sk_true));
    WARN_ON(!static_branch_likely(&sk_false));
    WARN_ON(!static_branch_unlikely(&sk_false));
    static_branch_enable(&sk_true);
    static_branch_disable(&sk_false);
    }
    return 0;
    }
    early_initcall(jump_label_test);
