//! Automatically rewritten from C to Rust
//! Source: kernel/ucount.c
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

    struct ucounts init_ucounts = {
    .ns    = &init_user_ns,
    .uid   = GLOBAL_ROOT_UID,
    .count = RCUREF_INIT(1),
    };
pub const UCOUNTS_HASHTABLE_BITS: c_int = 10;

    static struct hlist_nulls_head ucounts_hashtable[UCOUNTS_HASHTABLE_ENTRIES] = {
    [0 ... UCOUNTS_HASHTABLE_ENTRIES - 1] = HLIST_NULLS_HEAD_INIT(0)
    };
    static DEFINE_SPINLOCK(ucounts_lock);

    hash_long((unsigned long)__kuid_val(uid) + (unsigned long)(ns), \
    UCOUNTS_HASHTABLE_BITS)

    (ucounts_hashtable + ucounts_hashfn(ns, uid))

    static struct ctl_table_set *
    set_lookup(struct ctl_table_root *root)
    {
    return &current_user_ns().set;
    }
#[no_mangle]
unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    static int set_is_seen(struct ctl_table_set *set)
    {
    return &current_user_ns().set == set;
    }
    static int set_permissions(struct ctl_table_header *head,
    const struct ctl_table *table)
    {
    struct user_namespace *user_ns =
    container_of(head.set, struct user_namespace, set);
    int mode;
// Allow users with CAP_SYS_RESOURCE unrestrained access
    if (ns_capable_noaudit(user_ns, CAP_SYS_RESOURCE))
    mode = (table.mode & S_IRWXU) >> 6;
    else
// Allow all others at most read-only access
    mode = table.mode & S_IROTH;
    return (mode << 6) | (mode << 3) | mode;
    }
    static struct ctl_table_root set_root = {
    .lookup = set_lookup,
    .permissions = set_permissions,
    };
    let mut ue_zero: static long = 0;
    let mut ue_int_max: static long = INT_MAX;

    {							\
    .procname	= name,				\
    .maxlen		= sizeof(long),			\
    .mode		= 0644,				\
    .proc_handler	= proc_doulongvec_minmax,	\
    .extra1		= &ue_zero,			\
    .extra2		= &ue_int_max,			\
    }
    static const struct ctl_table user_table[] = {
    UCOUNT_ENTRY("max_user_namespaces"),
    UCOUNT_ENTRY("max_pid_namespaces"),
    UCOUNT_ENTRY("max_uts_namespaces"),
    UCOUNT_ENTRY("max_ipc_namespaces"),
    UCOUNT_ENTRY("max_net_namespaces"),
    UCOUNT_ENTRY("max_mnt_namespaces"),
    UCOUNT_ENTRY("max_cgroup_namespaces"),
    UCOUNT_ENTRY("max_time_namespaces"),

    UCOUNT_ENTRY("max_inotify_instances"),
    UCOUNT_ENTRY("max_inotify_watches"),

    UCOUNT_ENTRY("max_fanotify_groups"),
    UCOUNT_ENTRY("max_fanotify_marks"),

    UCOUNT_ENTRY("max_binfmt_misc_interpreters"),

    };

#[no_mangle]
pub unsafe extern "C" fn setup_userns_sysctls(ns: *mut user_namespace) -> bool {
    bool setup_userns_sysctls(struct user_namespace *ns)
    {

    struct ctl_table *tbl;
    BUILD_BUG_ON(ARRAY_SIZE(user_table) != UCOUNT_COUNTS);
    setup_sysctl_set(&ns.set, &set_root, set_is_seen);
    tbl = kmemdup(user_table, sizeof(user_table), GFP_KERNEL);
    if (tbl) {
    int i;
    for (i = 0; i < UCOUNT_COUNTS; i++) {
    tbl[i].data = &ns.ucount_max[i];
    }
    ns.sysctls = __register_sysctl_table(&ns.set, "user", tbl,
    ARRAY_SIZE(user_table));
    }
    if (!ns.sysctls) {
    kfree(tbl);
    retire_sysctl_set(&ns.set);
    return false;
    }

    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn retire_userns_sysctls(ns: *mut user_namespace) {
    void retire_userns_sysctls(struct user_namespace *ns)
    {

    const struct ctl_table *tbl;
    tbl = ns.sysctls.ctl_table_arg;
    unregister_sysctl_table(ns.sysctls);
    retire_sysctl_set(&ns.set);
    kfree(tbl);

    }
    static struct ucounts *find_ucounts(struct user_namespace *ns, kuid_t uid,
    struct hlist_nulls_head *hashent)
    {
    struct ucounts *ucounts;
    struct hlist_nulls_node *pos;
    guard(rcu)();
    hlist_nulls_for_each_entry_rcu(ucounts, pos, hashent, node) {
    if (uid_eq(ucounts.uid, uid) && (ucounts.ns == ns)) {
    if (rcuref_get(&ucounts.count))
    return ucounts;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn hlist_add_ucounts(ucounts: *mut ucounts) {
    static void hlist_add_ucounts(struct ucounts *ucounts)
    {
    struct hlist_nulls_head *hashent = ucounts_hashentry(ucounts.ns, ucounts.uid);
    spin_lock_irq(&ucounts_lock);
    hlist_nulls_add_head_rcu(&ucounts.node, hashent);
    spin_unlock_irq(&ucounts_lock);
    }
    struct ucounts *alloc_ucounts(struct user_namespace *ns, kuid_t uid)
    {
    struct hlist_nulls_head *hashent = ucounts_hashentry(ns, uid);
    struct ucounts *ucounts, *new;
    ucounts = find_ucounts(ns, uid, hashent);
    if (ucounts)
    return ucounts;
    new = kzalloc_obj(*new);
    if (!new)
    return core::ptr::null_mut();
    new.ns = ns;
    new.uid = uid;
    rcuref_init(&new.count, 1);
    spin_lock_irq(&ucounts_lock);
    ucounts = find_ucounts(ns, uid, hashent);
    if (ucounts) {
    spin_unlock_irq(&ucounts_lock);
    kfree(new);
    return ucounts;
    }
    hlist_nulls_add_head_rcu(&new.node, hashent);
    get_user_ns(new.ns);
    spin_unlock_irq(&ucounts_lock);
    return new;
    }
#[no_mangle]
pub unsafe extern "C" fn put_ucounts(ucounts: *mut ucounts) {
    void put_ucounts(struct ucounts *ucounts)
    {
    unsigned long flags;
    if (rcuref_put(&ucounts.count)) {
    spin_lock_irqsave(&ucounts_lock, flags);
    hlist_nulls_del_rcu(&ucounts.node);
    spin_unlock_irqrestore(&ucounts_lock, flags);
    put_user_ns(ucounts.ns);
    kfree_rcu(ucounts, rcu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn atomic_long_inc_below(v: *mut atomic_long_t, u: c_long) -> bool {
    static inline bool atomic_long_inc_below(atomic_long_t *v, long u)
    {
    let mut c: c_long = atomic_long_read(v);
    do {
    if (unlikely(c >= u))
    return false;
    } while (!atomic_long_try_cmpxchg(v, &c, c+1));
    return true;
    }
    struct ucounts *inc_ucount(struct user_namespace *ns, kuid_t uid,
    enum ucount_type type)
    {
    struct ucounts *ucounts, *iter, *bad;
    struct user_namespace *tns;
    ucounts = alloc_ucounts(ns, uid);
    for (iter = ucounts; iter; iter = tns.ucounts) {
    long max;
    tns = iter.ns;
    max = READ_ONCE(tns.ucount_max[type]);
    if (!atomic_long_inc_below(&iter.ucount[type], max))
    goto fail;
    }
    return ucounts;
    fail:
    bad = iter;
    for (iter = ucounts; iter != bad; iter = iter.ns.ucounts)
    atomic_long_dec(&iter.ucount[type]);
    put_ucounts(ucounts);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_FOR_MODULES(inc_ucount, "binfmt_misc");
#[no_mangle]
pub unsafe extern "C" fn dec_ucount(ucounts: *mut ucounts, type: enum ucount_type) {
    void dec_ucount(struct ucounts *ucounts, enum ucount_type type)
    {
    struct ucounts *iter;
    for (iter = ucounts; iter; iter = iter.ns.ucounts) {
    let mut dec: c_long = atomic_long_dec_if_positive(&iter.ucount[type]);
    WARN_ON_ONCE(dec < 0);
    }
    put_ucounts(ucounts);
    }
    EXPORT_SYMBOL_FOR_MODULES(dec_ucount, "binfmt_misc");
#[no_mangle]
pub unsafe extern "C" fn inc_rlimit_ucounts(ucounts: *mut ucounts, type: enum rlimit_type, v: c_long) -> c_long {
    long inc_rlimit_ucounts(struct ucounts *ucounts, enum rlimit_type type, long v)
    {
    struct ucounts *iter;
    let mut max: c_long = LONG_MAX;
    let mut ret: c_long = 0;
    for (iter = ucounts; iter; iter = iter.ns.ucounts) {
    let mut new: c_long = atomic_long_add_return(v, &iter.rlimit[type]);
    if (new < 0 || new > max)
    ret = LONG_MAX;
#[no_mangle]
pub unsafe extern "C" fn if(ucounts: iter ==) -> else {
    else if (iter == ucounts)
    ret = new;
    max = get_userns_rlimit_max(iter.ns, type);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rlimit_ucounts(ucounts: *mut ucounts, type: enum rlimit_type, v: c_long) -> bool {
    bool dec_rlimit_ucounts(struct ucounts *ucounts, enum rlimit_type type, long v)
    {
    struct ucounts *iter;
    long new = -1; /* Silence compiler warning */
    for (iter = ucounts; iter; iter = iter.ns.ucounts) {
    let mut dec: c_long = atomic_long_sub_return(v, &iter.rlimit[type]);
    WARN_ON_ONCE(dec < 0);
    if (iter == ucounts)
    new = dec;
    }
    return (new == 0);
    }
    static void do_dec_rlimit_put_ucounts(struct ucounts *ucounts,
    struct ucounts *last, enum rlimit_type type)
    {
    struct ucounts *iter, *next;
    for (iter = ucounts; iter != last; iter = next) {
    let mut dec: c_long = atomic_long_sub_return(1, &iter.rlimit[type]);
    WARN_ON_ONCE(dec < 0);
    next = iter.ns.ucounts;
    if (dec == 0)
    put_ucounts(iter);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rlimit_put_ucounts(ucounts: *mut ucounts, type: enum rlimit_type) {
    void dec_rlimit_put_ucounts(struct ucounts *ucounts, enum rlimit_type type)
    {
    do_dec_rlimit_put_ucounts(ucounts, core::ptr::null_mut(), type);
    }
    long inc_rlimit_get_ucounts(struct ucounts *ucounts, enum rlimit_type type,
    bool override_rlimit)
    {
// Caller must hold a reference to ucounts
    struct ucounts *iter;
    let mut max: c_long = LONG_MAX;
    long dec, ret = 0;
    for (iter = ucounts; iter; iter = iter.ns.ucounts) {
    let mut new: c_long = atomic_long_add_return(1, &iter.rlimit[type]);
    if (new < 0 || new > max)
    goto dec_unwind;
    if (iter == ucounts)
    ret = new;
    if (!override_rlimit)
    max = get_userns_rlimit_max(iter.ns, type);
//
// Grab an extra ucount reference for the caller when
// the rlimit count was previously 0.
//
    if (new != 1)
    continue;
    if (!get_ucounts(iter))
    goto dec_unwind;
    }
    return ret;
    dec_unwind:
    dec = atomic_long_sub_return(1, &iter.rlimit[type]);
    WARN_ON_ONCE(dec < 0);
    do_dec_rlimit_put_ucounts(ucounts, iter, type);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is_rlimit_overlimit(ucounts: *mut ucounts, type: enum rlimit_type, rlimit: c_ulong) -> bool {
    bool is_rlimit_overlimit(struct ucounts *ucounts, enum rlimit_type type, unsigned long rlimit)
    {
    struct ucounts *iter;
    let mut max: c_long = rlimit;
    if (rlimit > LONG_MAX)
    max = LONG_MAX;
    for (iter = ucounts; iter; iter = iter.ns.ucounts) {
    let mut val: c_long = get_rlimit_value(iter, type);
    if (val < 0 || val > max)
    return true;
    max = get_userns_rlimit_max(iter.ns, type);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn user_namespace_sysctl_init() -> __init int {
    static __init int user_namespace_sysctl_init(void)
    {

    static struct ctl_table_header *user_header;
    static struct ctl_table empty[1];
//
// It is necessary to register the user directory in the
// default set so that registrations in the child sets work
// properly.
//
    user_header = register_sysctl_sz("user", empty, 0);
    kmemleak_ignore(user_header);
    BUG_ON(!user_header);
    BUG_ON(!setup_userns_sysctls(&init_user_ns));

    hlist_add_ucounts(&init_ucounts);
    inc_rlimit_ucounts(&init_ucounts, UCOUNT_RLIMIT_NPROC, 1);
    return 0;
    }
    subsys_initcall(user_namespace_sysctl_init);
