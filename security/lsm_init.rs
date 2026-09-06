//! Automatically rewritten from C to Rust
//! Source: security/lsm_init.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// LSM initialization functions
//

// LSM enabled constants.
    let mut lsm_enabled_true: static __initdata int = 1;
    let mut lsm_enabled_false: static __initdata int = 0;
// Pointers to LSM sections defined in include/asm-generic/vmlinux.lds.h
    extern struct lsm_info __start_lsm_info[], __end_lsm_info[];
    extern struct lsm_info __start_early_lsm_info[], __end_early_lsm_info[];
// Number of "early" LSMs
    static __initdata unsigned int lsm_count_early;
// Build and boot-time LSM ordering.
    let mut lsm_order_builtin: *const static __initchar const = CONFIG_LSM;
    static __initdata const char *lsm_order_cmdline;
    static __initdata const char *lsm_order_legacy;
// Ordered list of LSMs to initialize.
    static __initdata struct lsm_info *lsm_order[MAX_LSM_COUNT + 1];
    static __initdata struct lsm_info *lsm_exclusive;

    for ((iter) = lsm_order; *(iter); (iter)++)

    for ((iter) = __start_lsm_info;					\
    (iter) < __end_lsm_info; (iter)++)

    for ((iter) = __start_early_lsm_info;				\
    (iter) < __end_early_lsm_info; (iter)++)

    ({								\
    int _r, _rc = 0;					\
    struct lsm_info **_lp, *_l;				\
    lsm_order_for_each(_lp) {				\
    _l = *_lp;					\
    if (!_l.initcall_##level)			\
    continue;				\
    lsm_pr_dbg("running %s %s initcall",		\
    _l.id.name, #level);		\
    _r = _l.initcall_##level();			\
    if (_r) {					\
    pr_warn("failed LSM %s %s initcall with errno %d\n", \
    _l.id.name, #level, _r);	\
    if (!_rc)				\
    _rc = _r;			\
    }						\
    }							\
    _rc;							\
    })
//
// lsm_choose_security - Legacy "major" LSM selection
// @str: kernel command line parameter
//
#[no_mangle]
unsafe extern "C" fn lsm_choose_security(str: *mut c_char) -> int __init {
    static int __init lsm_choose_security(char *str)
    {
    lsm_order_legacy = str;
    return 1;
    }
    __setup("security=", lsm_choose_security);
//
// lsm_choose_lsm - Modern LSM selection
// @str: kernel command line parameter
//
#[no_mangle]
unsafe extern "C" fn lsm_choose_lsm(str: *mut c_char) -> int __init {
    static int __init lsm_choose_lsm(char *str)
    {
    lsm_order_cmdline = str;
    return 1;
    }
    __setup("lsm=", lsm_choose_lsm);
//
// lsm_debug_enable - Enable LSM framework debugging
// @str: kernel command line parameter
//
// Currently we only provide debug info during LSM initialization, but we may
// want to expand this in the future.
//
#[no_mangle]
unsafe extern "C" fn lsm_debug_enable(str: *mut c_char) -> int __init {
    static int __init lsm_debug_enable(char *str)
    {
    lsm_debug = true;
    return 1;
    }
    __setup("lsm.debug", lsm_debug_enable);
//
// lsm_enabled_set - Mark a LSM as enabled
// @lsm: LSM definition
// @enabled: enabled flag
//
#[no_mangle]
unsafe extern "C" fn lsm_enabled_set(lsm: *mut lsm_info, enabled: bool) -> void __init {
    static void __init lsm_enabled_set(struct lsm_info *lsm, bool enabled)
    {
//
// When an LSM hasn't configured an enable variable, we can use
// a hard-coded location for storing the default enabled state.
//
    if (!lsm.enabled ||
    lsm.enabled == &lsm_enabled_true ||
    lsm.enabled == &lsm_enabled_false) {
    lsm.enabled = enabled ? &lsm_enabled_true : &lsm_enabled_false;
    } else {
// lsm->enabled = enabled;
    }
    }
//
// lsm_is_enabled - Determine if a LSM is enabled
// @lsm: LSM definition
//
#[no_mangle]
pub unsafe extern "C" fn lsm_is_enabled(lsm: *mut lsm_info) -> bool {
    static inline bool lsm_is_enabled(struct lsm_info *lsm)
    {
    return (lsm.enabled ? *lsm.enabled : false);
    }
//
// lsm_order_exists - Determine if a LSM exists in the ordered list
// @lsm: LSM definition
//
#[no_mangle]
unsafe extern "C" fn lsm_order_exists(lsm: *mut lsm_info) -> bool __init {
    static bool __init lsm_order_exists(struct lsm_info *lsm)
    {
    struct lsm_info **check;
    lsm_order_for_each(check) {
    if (*check == lsm)
    return true;
    }
    return false;
    }
//
// lsm_order_append - Append a LSM to the ordered list
// @lsm: LSM definition
// @src: source of the addition
//
// Append @lsm to the enabled LSM array after ensuring that it hasn't been
// explicitly disabled, is a duplicate entry, or would run afoul of the
// LSM_FLAG_EXCLUSIVE logic.
//
#[no_mangle]
unsafe extern "C" fn lsm_order_append(lsm: *mut lsm_info, src: *const c_char) -> void __init {
    static void __init lsm_order_append(struct lsm_info *lsm, const char *src)
    {
// Ignore duplicate selections.
    if (lsm_order_exists(lsm))
    return;
// Skip explicitly disabled LSMs.
    if (lsm.enabled && !lsm_is_enabled(lsm)) {
    lsm_pr_dbg("skip previously disabled LSM %s:%s\n",
    src, lsm.id.name);
    return;
    }
    if (lsm_active_cnt == MAX_LSM_COUNT) {
    pr_warn("exceeded maximum LSM count on %s:%s\n",
    src, lsm.id.name);
    lsm_enabled_set(lsm, false);
    return;
    }
    if (lsm.flags & LSM_FLAG_EXCLUSIVE) {
    if (lsm_exclusive) {
    lsm_pr_dbg("skip exclusive LSM conflict %s:%s\n",
    src, lsm.id.name);
    lsm_enabled_set(lsm, false);
    return;
    } else {
    lsm_pr_dbg("select exclusive LSM %s:%s\n",
    src, lsm.id.name);
    lsm_exclusive = lsm;
    }
    }
    lsm_enabled_set(lsm, true);
    lsm_order[lsm_active_cnt] = lsm;
    lsm_idlist[lsm_active_cnt++] = lsm.id;
    lsm_pr_dbg("enabling LSM %s:%s\n", src, lsm.id.name);
    }
//
// lsm_order_parse - Parse the comma delimited LSM list
// @list: LSM list
// @src: source of the list
//
#[no_mangle]
unsafe extern "C" fn lsm_order_parse(list: *const c_char, src: *const c_char) -> void __init {
    static void __init lsm_order_parse(const char *list, const char *src)
    {
    struct lsm_info *lsm;
    char *sep, *name, *next;
// Handle any Legacy LSM exclusions if one was specified.
    if (lsm_order_legacy) {
//
// To match the original "security=" behavior, this explicitly
// does NOT fallback to another Legacy Major if the selected
// one was separately disabled: disable all non-matching
// Legacy Major LSMs.
//
    lsm_for_each_raw(lsm) {
    if ((lsm.flags & LSM_FLAG_LEGACY_MAJOR) &&
    strcmp(lsm.id.name, lsm_order_legacy)) {
    lsm_enabled_set(lsm, false);
    lsm_pr_dbg("skip legacy LSM conflict %s:%s\n",
    src, lsm.id.name);
    }
    }
    }
// LSM_ORDER_FIRST
    lsm_for_each_raw(lsm) {
    if (lsm.order == LSM_ORDER_FIRST)
    lsm_order_append(lsm, "first");
    }
// Normal or "mutable" LSMs
    sep = kstrdup(list, GFP_KERNEL);
    next = sep;
// Walk the list, looking for matching LSMs.
    while ((name = strsep(&next, ",")) != core::ptr::null_mut()) {
    lsm_for_each_raw(lsm) {
    if (!strcmp(lsm.id.name, name) &&
    lsm.order == LSM_ORDER_MUTABLE)
    lsm_order_append(lsm, src);
    }
    }
    kfree(sep);
// Legacy LSM if specified.
    if (lsm_order_legacy) {
    lsm_for_each_raw(lsm) {
    if (!strcmp(lsm.id.name, lsm_order_legacy))
    lsm_order_append(lsm, src);
    }
    }
// LSM_ORDER_LAST
    lsm_for_each_raw(lsm) {
    if (lsm.order == LSM_ORDER_LAST)
    lsm_order_append(lsm, "last");
    }
// Disable all LSMs not previously enabled.
    lsm_for_each_raw(lsm) {
    if (lsm_order_exists(lsm))
    continue;
    lsm_enabled_set(lsm, false);
    lsm_pr_dbg("skip disabled LSM %s:%s\n", src, lsm.id.name);
    }
    }
//
// lsm_blob_size_update - Update the LSM blob size and offset information
// @sz_req: the requested additional blob size
// @sz_cur: the existing blob size
//
    static void __init lsm_blob_size_update(unsigned int *sz_req,
    unsigned int *sz_cur)
    {
    unsigned int offset;
    if (*sz_req == 0)
    return;
    offset = ALIGN(*sz_cur, sizeof(void *));
// sz_cur = offset + *sz_req;
// sz_req = offset;
    }
//
// lsm_prepare - Prepare the LSM framework for a new LSM
// @lsm: LSM definition
//
#[no_mangle]
unsafe extern "C" fn lsm_prepare(lsm: *mut lsm_info) -> void __init {
    static void __init lsm_prepare(struct lsm_info *lsm)
    {
    struct lsm_blob_sizes *blobs = lsm.blobs;
    if (!blobs)
    return;
// Register the LSM blob sizes.
    blobs = lsm.blobs;
    lsm_blob_size_update(&blobs.lbs_cred, &blob_sizes.lbs_cred);
    lsm_blob_size_update(&blobs.lbs_file, &blob_sizes.lbs_file);
    lsm_blob_size_update(&blobs.lbs_backing_file,
    &blob_sizes.lbs_backing_file);
    lsm_blob_size_update(&blobs.lbs_ib, &blob_sizes.lbs_ib);
// inode blob gets an rcu_head in addition to LSM blobs.
    if (blobs.lbs_inode && blob_sizes.lbs_inode == 0)
    blob_sizes.lbs_inode = sizeof(struct rcu_head);
    lsm_blob_size_update(&blobs.lbs_inode, &blob_sizes.lbs_inode);
    lsm_blob_size_update(&blobs.lbs_ipc, &blob_sizes.lbs_ipc);
    lsm_blob_size_update(&blobs.lbs_key, &blob_sizes.lbs_key);
    lsm_blob_size_update(&blobs.lbs_msg_msg, &blob_sizes.lbs_msg_msg);
    lsm_blob_size_update(&blobs.lbs_perf_event,
    &blob_sizes.lbs_perf_event);
    lsm_blob_size_update(&blobs.lbs_sock, &blob_sizes.lbs_sock);
    lsm_blob_size_update(&blobs.lbs_superblock,
    &blob_sizes.lbs_superblock);
    lsm_blob_size_update(&blobs.lbs_task, &blob_sizes.lbs_task);
    lsm_blob_size_update(&blobs.lbs_tun_dev, &blob_sizes.lbs_tun_dev);
    lsm_blob_size_update(&blobs.lbs_xattr_count,
    &blob_sizes.lbs_xattr_count);
    lsm_blob_size_update(&blobs.lbs_bdev, &blob_sizes.lbs_bdev);
    lsm_blob_size_update(&blobs.lbs_bpf_map, &blob_sizes.lbs_bpf_map);
    lsm_blob_size_update(&blobs.lbs_bpf_prog, &blob_sizes.lbs_bpf_prog);
    lsm_blob_size_update(&blobs.lbs_bpf_token, &blob_sizes.lbs_bpf_token);
    }
//
// lsm_init_single - Initialize a given LSM
// @lsm: LSM definition
//
#[no_mangle]
unsafe extern "C" fn lsm_init_single(lsm: *mut lsm_info) -> void __init {
    static void __init lsm_init_single(struct lsm_info *lsm)
    {
    int ret;
    if (!lsm_is_enabled(lsm))
    return;
    lsm_pr_dbg("initializing %s\n", lsm.id.name);
    ret = lsm.init();
    WARN(ret, "%s failed to initialize: %d\n", lsm.id.name, ret);
    }
//
// lsm_static_call_init - Initialize a LSM's static calls
// @hl: LSM hook list
//
#[no_mangle]
unsafe extern "C" fn lsm_static_call_init(hl: *mut security_hook_list) -> int __init {
    static int __init lsm_static_call_init(struct security_hook_list *hl)
    {
    struct lsm_static_call *scall = hl.scalls;
    int i;
    for (i = 0; i < MAX_LSM_COUNT; i++) {
// Update the first static call that is not used yet
    if (!scall.hl) {
    __static_call_update(scall.key, scall.trampoline,
    hl.hook.lsm_func_addr);
    scall.hl = hl;
    static_branch_enable(scall.active);
    return 0;
    }
    scall++;
    }
    return -ENOSPC;
    }
//
// security_add_hooks - Add a LSM's hooks to the LSM framework's hook lists
// @hooks: LSM hooks to add
// @count: number of hooks to add
// @lsmid: identification information for the LSM
//
// Each LSM has to register its hooks with the LSM framework.
//
    void __init security_add_hooks(struct security_hook_list *hooks, int count,
    const struct lsm_id *lsmid)
    {
    int i;
    for (i = 0; i < count; i++) {
    hooks[i].lsmid = lsmid;
    if (lsm_static_call_init(&hooks[i]))
    panic("exhausted LSM callback slots with LSM %s\n",
    lsmid.name);
    }
    }
//
// early_security_init - Initialize the early LSMs
//
#[no_mangle]
pub unsafe extern "C" fn early_security_init() -> int __init {
    int __init early_security_init(void)
    {
    struct lsm_info *lsm;
// NOTE: lsm_pr_dbg() doesn't work here as lsm_debug is not yet set
    lsm_early_for_each_raw(lsm) {
    lsm_enabled_set(lsm, true);
    lsm_order_append(lsm, "early");
    lsm_prepare(lsm);
    lsm_init_single(lsm);
    lsm_count_early++;
    }
    return 0;
    }
//
// security_init - Initializes the LSM framework
//
// This should be called early in the kernel initialization sequence.
//
#[no_mangle]
pub unsafe extern "C" fn security_init() -> int __init {
    int __init security_init(void)
    {
    unsigned int cnt;
    struct lsm_info **lsm;
    if (lsm_debug) {
    struct lsm_info *i;
    cnt = 0;
    lsm_pr("available LSMs: ");
    lsm_early_for_each_raw(i)
    lsm_pr_cont("%s%s(E)", (cnt++ ? "," : ""), i.id.name);
    lsm_for_each_raw(i)
    lsm_pr_cont("%s%s", (cnt++ ? "," : ""), i.id.name);
    lsm_pr_cont("\n");
    lsm_pr("built-in LSM config: %s\n", lsm_order_builtin);
    lsm_pr("legacy LSM parameter: %s\n", lsm_order_legacy);
    lsm_pr("boot LSM parameter: %s\n", lsm_order_cmdline);
// see the note about lsm_pr_dbg() in early_security_init()
    lsm_early_for_each_raw(i)
    lsm_pr("enabled LSM early:%s\n", i.id.name);
    }
    if (lsm_order_cmdline) {
    if (lsm_order_legacy)
    lsm_order_legacy = core::ptr::null_mut();
    lsm_order_parse(lsm_order_cmdline, "cmdline");
    } else
    lsm_order_parse(lsm_order_builtin, "builtin");
    lsm_order_for_each(lsm)
    lsm_prepare(*lsm);
    if (lsm_debug) {
    lsm_pr("blob(cred) size %d\n", blob_sizes.lbs_cred);
    lsm_pr("blob(file) size %d\n", blob_sizes.lbs_file);
    lsm_pr("blob(backing_file) size %d\n",
    blob_sizes.lbs_backing_file);
    lsm_pr("blob(ib) size %d\n", blob_sizes.lbs_ib);
    lsm_pr("blob(inode) size %d\n", blob_sizes.lbs_inode);
    lsm_pr("blob(ipc) size %d\n", blob_sizes.lbs_ipc);
    lsm_pr("blob(key) size %d\n", blob_sizes.lbs_key);
    lsm_pr("blob(msg_msg)_size %d\n", blob_sizes.lbs_msg_msg);
    lsm_pr("blob(sock) size %d\n", blob_sizes.lbs_sock);
    lsm_pr("blob(superblock) size %d\n", blob_sizes.lbs_superblock);
    lsm_pr("blob(perf_event) size %d\n", blob_sizes.lbs_perf_event);
    lsm_pr("blob(task) size %d\n", blob_sizes.lbs_task);
    lsm_pr("blob(tun_dev) size %d\n", blob_sizes.lbs_tun_dev);
    lsm_pr("blob(xattr) count %d\n", blob_sizes.lbs_xattr_count);
    lsm_pr("blob(bdev) size %d\n", blob_sizes.lbs_bdev);
    lsm_pr("blob(bpf_map) size %d\n", blob_sizes.lbs_bpf_map);
    lsm_pr("blob(bpf_prog) size %d\n", blob_sizes.lbs_bpf_prog);
    lsm_pr("blob(bpf_token) size %d\n", blob_sizes.lbs_bpf_token);
    }
    if (blob_sizes.lbs_file)
    lsm_file_cache = kmem_cache_create("lsm_file_cache",
    blob_sizes.lbs_file, 0,
    SLAB_PANIC, core::ptr::null_mut());
    if (blob_sizes.lbs_backing_file)
    lsm_backing_file_cache = kmem_cache_create(
    "lsm_backing_file_cache",
    blob_sizes.lbs_backing_file,
    0, SLAB_PANIC, core::ptr::null_mut());
    if (blob_sizes.lbs_inode)
    lsm_inode_cache = kmem_cache_create("lsm_inode_cache",
    blob_sizes.lbs_inode, 0,
    SLAB_PANIC, core::ptr::null_mut());
    if (lsm_cred_alloc((struct cred *)unrcu_pointer(current.cred),
    GFP_KERNEL))
    panic("early LSM cred alloc failed\n");
    if (lsm_task_alloc(current))
    panic("early LSM task alloc failed\n");
    cnt = 0;
    lsm_order_for_each(lsm) {
// skip the "early" LSMs as they have already been setup
    if (cnt++ < lsm_count_early)
    continue;
    lsm_init_single(*lsm);
    }
    return 0;
    }
//
// security_initcall_pure - Run the LSM pure initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_pure() -> int __init {
    static int __init security_initcall_pure(void)
    {
    return lsm_initcall(pure);
    }
    pure_initcall(security_initcall_pure);
//
// security_initcall_early - Run the LSM early initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_early() -> int __init {
    static int __init security_initcall_early(void)
    {
    return lsm_initcall(early);
    }
    early_initcall(security_initcall_early);
//
// security_initcall_core - Run the LSM core initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_core() -> int __init {
    static int __init security_initcall_core(void)
    {
    int rc_sfs, rc_lsm;
    rc_sfs = securityfs_init();
    rc_lsm = lsm_initcall(core);
    return (rc_sfs ? rc_sfs : rc_lsm);
    }
    core_initcall(security_initcall_core);
//
// security_initcall_subsys - Run the LSM subsys initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_subsys() -> int __init {
    static int __init security_initcall_subsys(void)
    {
    return lsm_initcall(subsys);
    }
    subsys_initcall(security_initcall_subsys);
//
// security_initcall_fs - Run the LSM fs initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_fs() -> int __init {
    static int __init security_initcall_fs(void)
    {
    return lsm_initcall(fs);
    }
    fs_initcall(security_initcall_fs);
//
// security_initcall_device - Run the LSM device initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_device() -> int __init {
    static int __init security_initcall_device(void)
    {
    return lsm_initcall(device);
    }
    device_initcall(security_initcall_device);
//
// security_initcall_late - Run the LSM late initcalls
//
#[no_mangle]
unsafe extern "C" fn security_initcall_late() -> int __init {
    static int __init security_initcall_late(void)
    {
    return lsm_initcall(late);
    }
    late_initcall(security_initcall_late);
//
// security_initcall_late_sync - Run the LSM late initcalls sync
//
#[no_mangle]
unsafe extern "C" fn security_initcall_late_sync() -> int __init {
    static int __init security_initcall_late_sync(void)
    {
    int rc;
    rc = lsm_initcall(late_sync);
    lsm_pr_dbg("all enabled LSMs fully activated\n");
    call_blocking_lsm_notifier(LSM_STARTED_ALL, core::ptr::null_mut());
    return rc;
    }
    late_initcall_sync(security_initcall_late_sync);
