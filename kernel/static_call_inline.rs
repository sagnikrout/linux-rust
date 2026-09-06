//! Automatically rewritten from C to Rust
//! Source: kernel/static_call_inline.c
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

    extern struct static_call_site __start_static_call_sites[],
    __stop_static_call_sites[];
    extern struct static_call_tramp_key __start_static_call_tramp_key[],
    __stop_static_call_tramp_key[];
    int static_call_initialized;
//
// Must be called before early_initcall() to be effective.
//
#[no_mangle]
pub unsafe extern "C" fn static_call_force_reinit() {
    void static_call_force_reinit(void)
    {
    if (WARN_ON_ONCE(!static_call_initialized))
    return;
    static_call_initialized++;
    }
// mutex to protect key modules/sites
    static DEFINE_MUTEX(static_call_mutex);
#[no_mangle]
unsafe extern "C" fn static_call_lock() {
    static void static_call_lock(void)
    {
    mutex_lock(&static_call_mutex);
    }
#[no_mangle]
unsafe extern "C" fn static_call_unlock() {
    static void static_call_unlock(void)
    {
    mutex_unlock(&static_call_mutex);
    }
    static inline void *static_call_addr(struct static_call_site *site)
    {
    return (void *)((long)site.addr + (long)&site.addr);
    }
#[no_mangle]
pub unsafe extern "C" fn __static_call_key(site: *const static_call_site) -> c_ulong {
    static inline unsigned long __static_call_key(const struct static_call_site *site)
    {
    return (long)site.key + (long)&site.key;
    }
    static inline struct static_call_key *static_call_key(const struct static_call_site *site)
    {
    return (void *)(__static_call_key(site) & ~STATIC_CALL_SITE_FLAGS);
    }
// These assume the key is word-aligned.
#[no_mangle]
pub unsafe extern "C" fn static_call_is_init(site: *mut static_call_site) -> bool {
    static inline bool static_call_is_init(struct static_call_site *site)
    {
    return __static_call_key(site) & STATIC_CALL_SITE_INIT;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_is_tail(site: *mut static_call_site) -> bool {
    static inline bool static_call_is_tail(struct static_call_site *site)
    {
    return __static_call_key(site) & STATIC_CALL_SITE_TAIL;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_set_init(site: *mut static_call_site) {
    static inline void static_call_set_init(struct static_call_site *site)
    {
    site.key = (__static_call_key(site) | STATIC_CALL_SITE_INIT) -
    (long)&site.key;
    }
#[no_mangle]
unsafe extern "C" fn static_call_site_cmp(_a: *const c_void, _b: *const c_void) -> c_int {
    static int static_call_site_cmp(const void *_a, const void *_b)
    {
    const struct static_call_site *a = _a;
    const struct static_call_site *b = _b;
    const struct static_call_key *key_a = static_call_key(a);
    const struct static_call_key *key_b = static_call_key(b);
    if (key_a < key_b)
    return -1;
    if (key_a > key_b)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn static_call_site_swap(_a: *mut c_void, _b: *mut c_void, size: c_int) {
    static void static_call_site_swap(void *_a, void *_b, int size)
    {
    let mut delta: c_long = (unsigned long)_a - (unsigned long)_b;
    struct static_call_site *a = _a;
    struct static_call_site *b = _b;
    let mut tmp: static_call_site = *a;
    a.addr = b.addr  - delta;
    a.key  = b.key   - delta;
    b.addr = tmp.addr + delta;
    b.key  = tmp.key  + delta;
    }
    static inline void static_call_sort_entries(struct static_call_site *start,
    struct static_call_site *stop)
    {
    sort(start, stop - start, sizeof(struct static_call_site),
    static_call_site_cmp, static_call_site_swap);
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_key_has_mods(key: *mut static_call_key) -> bool {
    static inline bool static_call_key_has_mods(struct static_call_key *key)
    {
    return !(key.type & 1);
    }
    static inline struct static_call_mod *static_call_key_next(struct static_call_key *key)
    {
    if (!static_call_key_has_mods(key))
    return core::ptr::null_mut();
    return key.mods;
    }
    static inline struct static_call_site *static_call_key_sites(struct static_call_key *key)
    {
    if (static_call_key_has_mods(key))
    return core::ptr::null_mut();
    return (struct static_call_site *)(key.type & ~1);
    }
#[no_mangle]
pub unsafe extern "C" fn __static_call_update(key: *mut static_call_key, tramp: *mut c_void, func: *mut c_void) {
    void __static_call_update(struct static_call_key *key, void *tramp, void *func)
    {
    struct static_call_site *site, *stop;
    struct static_call_mod *site_mod, first;
    cpus_read_lock();
    static_call_lock();
    if (key.func == func)
    goto done;
    key.func = func;
    arch_static_call_transform(core::ptr::null_mut(), tramp, func, false);
//
// If uninitialized, we'll not update the callsites, but they still
// point to the trampoline and we just patched that.
//
    if (WARN_ON_ONCE(!static_call_initialized))
    goto done;
    first = (struct static_call_mod){
    .next = static_call_key_next(key),
    .mod = core::ptr::null_mut(),
    .sites = static_call_key_sites(key),
    };
    for (site_mod = &first; site_mod; site_mod = site_mod.next) {
    let mut init: bool = system_state < SYSTEM_RUNNING;
    struct module *mod = site_mod.mod;
    if (!site_mod.sites) {
//
// This can happen if the static call key is defined in
// a module which doesn't use it.
//
// It also happens in the has_mods case, where the
// 'first' entry has no sites associated with it.
//
    continue;
    }
    stop = __stop_static_call_sites;
    if (mod) {

    stop = mod.static_call_sites +
    mod.num_static_call_sites;
    init = mod.state == MODULE_STATE_COMING;

    }
    for (site = site_mod.sites;
    site < stop && static_call_key(site) == key; site++) {
    void *site_addr = static_call_addr(site);
    if (!init && static_call_is_init(site))
    continue;
    if (!kernel_text_address((unsigned long)site_addr)) {
//
// This skips patching built-in __exit, which
// is part of init_section_contains() but is
// not part of kernel_text_address().
//
// Skipping built-in __exit is fine since it
// will never be executed.
//
    WARN_ONCE(!static_call_is_init(site),
    "can't patch static call site at %pS",
    site_addr);
    continue;
    }
    arch_static_call_transform(site_addr, tramp, func,
    static_call_is_tail(site));
    }
    }
    done:
    static_call_unlock();
    cpus_read_unlock();
    }
    EXPORT_SYMBOL_GPL(__static_call_update);
    static int __static_call_init(struct module *mod,
    struct static_call_site *start,
    struct static_call_site *stop)
    {
    struct static_call_site *site;
    struct static_call_key *key, *prev_key = core::ptr::null_mut();
    struct static_call_mod *site_mod;
    if (start == stop)
    return 0;
    static_call_sort_entries(start, stop);
    for (site = start; site < stop; site++) {
    void *site_addr = static_call_addr(site);
    if ((mod && within_module_init((unsigned long)site_addr, mod)) ||
    (!mod && init_section_contains(site_addr, 1)))
    static_call_set_init(site);
    key = static_call_key(site);
    if (key != prev_key) {
    prev_key = key;
//
// For vmlinux (!mod) avoid the allocation by storing
// the sites pointer in the key itself. Also see
// __static_call_update()'s @first.
//
// This allows architectures (eg. x86) to call
// static_call_init() before memory allocation works.
//
    if (!mod) {
    key.sites = site;
    key.type |= 1;
    goto do_transform;
    }
    site_mod = kzalloc_obj(*site_mod);
    if (!site_mod)
    return -ENOMEM;
//
// When the key has a direct sites pointer, extract
// that into an explicit struct static_call_mod, so we
// can have a list of modules.
//
    if (static_call_key_sites(key)) {
    site_mod.mod = core::ptr::null_mut();
    site_mod.next = core::ptr::null_mut();
    site_mod.sites = static_call_key_sites(key);
    key.mods = site_mod;
    site_mod = kzalloc_obj(*site_mod);
    if (!site_mod)
    return -ENOMEM;
    }
    site_mod.mod = mod;
    site_mod.sites = site;
    site_mod.next = static_call_key_next(key);
    key.mods = site_mod;
    }
    do_transform:
    arch_static_call_transform(site_addr, core::ptr::null_mut(), key.func,
    static_call_is_tail(site));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn addr_conflict(site: *mut static_call_site, start: *mut c_void, end: *mut c_void) -> c_int {
    static int addr_conflict(struct static_call_site *site, void *start, void *end)
    {
    let mut addr: c_ulong = (unsigned long)static_call_addr(site);
    if (addr <= (unsigned long)end &&
    addr + CALL_INSN_SIZE > (unsigned long)start)
    return 1;
    return 0;
    }
    static int __static_call_text_reserved(struct static_call_site *iter_start,
    struct static_call_site *iter_stop,
    void *start, void *end, bool init)
    {
    struct static_call_site *iter = iter_start;
    while (iter < iter_stop) {
    if (init || !static_call_is_init(iter)) {
    if (addr_conflict(iter, start, end))
    return 1;
    }
    iter++;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn __static_call_mod_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
    static int __static_call_mod_text_reserved(void *start, void *end)
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
    ret = __static_call_text_reserved(mod.static_call_sites,
    mod.static_call_sites + mod.num_static_call_sites,
    start, end, mod.state == MODULE_STATE_COMING);
    module_put(mod);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tramp_key_lookup(addr: c_ulong) -> c_ulong {
    static unsigned long tramp_key_lookup(unsigned long addr)
    {
    struct static_call_tramp_key *start = __start_static_call_tramp_key;
    struct static_call_tramp_key *stop = __stop_static_call_tramp_key;
    struct static_call_tramp_key *tramp_key;
    for (tramp_key = start; tramp_key != stop; tramp_key++) {
    unsigned long tramp;
    tramp = (long)tramp_key.tramp + (long)&tramp_key.tramp;
    if (tramp == addr)
    return (long)tramp_key.key + (long)&tramp_key.key;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn static_call_add_module(mod: *mut module) -> c_int {
    static int static_call_add_module(struct module *mod)
    {
    struct static_call_site *start = mod.static_call_sites;
    struct static_call_site *stop = start + mod.num_static_call_sites;
    struct static_call_site *site;
    for (site = start; site != stop; site++) {
    let mut s_key: c_ulong = __static_call_key(site);
    let mut addr: c_ulong = s_key & ~STATIC_CALL_SITE_FLAGS;
    unsigned long key;
//
// Is the key is exported, 'addr' points to the key, which
// means modules are allowed to call static_call_update() on
// it.
//
// Otherwise, the key isn't exported, and 'addr' points to the
// trampoline so we need to lookup the key.
//
// We go through this dance to prevent crazy modules from
// abusing sensitive static calls.
//
    if (!kernel_text_address(addr))
    continue;
    key = tramp_key_lookup(addr);
    if (!key) {
    pr_warn("Failed to fixup __raw_static_call() usage at: %ps\n",
    static_call_addr(site));
    return -EINVAL;
    }
    key |= s_key & STATIC_CALL_SITE_FLAGS;
    site.key = key - (long)&site.key;
    }
    return __static_call_init(mod, start, stop);
    }
#[no_mangle]
unsafe extern "C" fn static_call_del_module(mod: *mut module) {
    static void static_call_del_module(struct module *mod)
    {
    struct static_call_site *start = mod.static_call_sites;
    struct static_call_site *stop = mod.static_call_sites +
    mod.num_static_call_sites;
    struct static_call_key *key, *prev_key = core::ptr::null_mut();
    struct static_call_mod *site_mod, **prev;
    struct static_call_site *site;
    for (site = start; site < stop; site++) {
    key = static_call_key(site);
//
// If the key was not updated due to a memory allocation
// failure in __static_call_init() then treating key::sites
// as key::mods in the code below would cause random memory
// access and #GP. In that case all subsequent sites have
// not been touched either, so stop iterating.
//
    if (!static_call_key_has_mods(key))
    break;
    if (key == prev_key)
    continue;
    prev_key = key;
    for (prev = &key.mods, site_mod = key.mods;
    site_mod && site_mod.mod != mod;
    prev = &site_mod.next, site_mod = site_mod.next)
    ;
    if (!site_mod)
    continue;
// prev = site_mod->next;
    kfree(site_mod);
    }
    }
    static int static_call_module_notify(struct notifier_block *nb,
    unsigned long val, void *data)
    {
    struct module *mod = data;
    let mut ret: c_int = 0;
    cpus_read_lock();
    static_call_lock();
    switch (val) {
    case MODULE_STATE_COMING:
    ret = static_call_add_module(mod);
    if (ret) {
    pr_warn("Failed to allocate memory for static calls\n");
    static_call_del_module(mod);
    }
    break;
    case MODULE_STATE_GOING:
    static_call_del_module(mod);
    break;
    }
    static_call_unlock();
    cpus_read_unlock();
    return notifier_from_errno(ret);
    }
    static struct notifier_block static_call_module_nb = {
    .notifier_call = static_call_module_notify,
    };

#[no_mangle]
pub unsafe extern "C" fn __static_call_mod_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
    static inline int __static_call_mod_text_reserved(void *start, void *end)
    {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn static_call_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
    int static_call_text_reserved(void *start, void *end)
    {
    let mut init: bool = system_state < SYSTEM_RUNNING;
    int ret = __static_call_text_reserved(__start_static_call_sites,
    __stop_static_call_sites, start, end, init);
    if (ret)
    return ret;
    return __static_call_mod_text_reserved(start, end);
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_init() -> int __init {
    int __init static_call_init(void)
    {
    int ret;
// See static_call_force_reinit().
    if (static_call_initialized == 1)
    return 0;
    cpus_read_lock();
    static_call_lock();
    ret = __static_call_init(core::ptr::null_mut(), __start_static_call_sites,
    __stop_static_call_sites);
    static_call_unlock();
    cpus_read_unlock();
    if (ret) {
    pr_err("Failed to allocate memory for static_call!\n");
    BUG();
    }

    if (!static_call_initialized)
    register_module_notifier(&static_call_module_nb);

    static_call_initialized = 1;
    return 0;
    }
    early_initcall(static_call_init);

#[no_mangle]
unsafe extern "C" fn func_a(x: c_int) -> c_int {
    static int func_a(int x)
    {
    return x+1;
    }
#[no_mangle]
unsafe extern "C" fn func_b(x: c_int) -> c_int {
    static int func_b(int x)
    {
    return x+2;
    }
    DEFINE_STATIC_CALL(sc_selftest, func_a);
    static struct static_call_data {
    int (*func)(int);
    int val;
    int expect;
    } static_call_data [] __initdata = {
    { core::ptr::null_mut(),   2, 3 },
    { func_b, 2, 4 },
    { func_a, 2, 3 }
    };
#[no_mangle]
unsafe extern "C" fn test_static_call_init() -> int __init {
    static int __init test_static_call_init(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(static_call_data); i++ ) {
    struct static_call_data *scd = &static_call_data[i];
    if (scd.func)
    static_call_update(sc_selftest, scd.func);
    WARN_ON(static_call(sc_selftest)(scd.val) != scd.expect);
    }
    return 0;
    }
    early_initcall(test_static_call_init);
