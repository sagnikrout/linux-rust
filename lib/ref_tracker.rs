//! Automatically rewritten from C to Rust
//! Source: lib/ref_tracker.c
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

pub const REF_TRACKER_STACK_ENTRIES: c_int = 16;
pub const STACK_BUF_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_tracker {
    pub /: *mut *mut list_head head; / anchor into dir->list or dir->quarantine,
    pub dead: bool,
    pub alloc_stack_handle: depot_stack_handle_t,
    pub free_stack_handle: depot_stack_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_tracker_dir_stats {
    pub total: c_int,
    pub count: c_int,
    struct {
    pub stack_handle: depot_stack_handle_t,
    pub count: c_uint,
    pub stacks: [}; ],
}

//
// ref_tracker_dir_init() is usually called in allocation-safe contexts, but
// the same is not true of ref_tracker_dir_exit() which can be called from
// anywhere an object is freed. Removing debugfs dentries is a blocking
// operation, so we defer that work to the debugfs_reap_worker.
//
// Each dentry is tracked in the appropriate xarray.  When
// ref_tracker_dir_exit() is called, its entries in the xarrays are marked and
// the workqueue job is scheduled. The worker then runs and deletes any marked
// dentries asynchronously.
//
    static struct xarray		debugfs_dentries;
    static struct xarray		debugfs_symlinks;
    static struct work_struct	debugfs_reap_worker;

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_debugfs_mark(dir: *mut ref_tracker_dir) {
    static inline void ref_tracker_debugfs_mark(struct ref_tracker_dir *dir)
    {
    unsigned long flags;
    xa_lock_irqsave(&debugfs_dentries, flags);
    __xa_set_mark(&debugfs_dentries, (unsigned long)dir, REF_TRACKER_DIR_DEAD);
    xa_unlock_irqrestore(&debugfs_dentries, flags);
    xa_lock_irqsave(&debugfs_symlinks, flags);
    __xa_set_mark(&debugfs_symlinks, (unsigned long)dir, REF_TRACKER_DIR_DEAD);
    xa_unlock_irqrestore(&debugfs_symlinks, flags);
    schedule_work(&debugfs_reap_worker);
    }

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_debugfs_mark(dir: *mut ref_tracker_dir) {
    static inline void ref_tracker_debugfs_mark(struct ref_tracker_dir *dir)
    {
    }

    static struct ref_tracker_dir_stats *
    ref_tracker_get_stats(struct ref_tracker_dir *dir, unsigned int limit)
    {
    struct ref_tracker_dir_stats *stats;
    struct ref_tracker *tracker;
    stats = kmalloc_flex(*stats, stacks, limit, GFP_NOWAIT);
    if (!stats)
    return ERR_PTR(-ENOMEM);
    stats.total = 0;
    stats.count = 0;
    list_for_each_entry(tracker, &dir.list, head) {
    let mut stack: depot_stack_handle_t = tracker.alloc_stack_handle;
    int i;
    ++stats.total;
    for (i = 0; i < stats.count; ++i)
    if (stats.stacks[i].stack_handle == stack)
    break;
    if (i >= limit)
    continue;
    if (i >= stats.count) {
    stats.stacks[i].stack_handle = stack;
    stats.stacks[i].count = 0;
    ++stats.count;
    }
    ++stats.stacks[i].count;
    }
    return stats;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ostream {
    pub ...): *mut *mut *mut *mut void __ostream_printf (func)(struct ostream stream, char fmt,,
    pub prefix: *mut c_char,
    pub buf: *mut c_char,
    pub seq: *mut seq_file,
    pub used: int size,,
}

#[no_mangle]
unsafe extern "C" fn pr_ostream_log(stream: *mut ostream, fmt: *mut c_char, ...) -> void __ostream_printf {
    static void __ostream_printf pr_ostream_log(struct ostream *stream, char *fmt, ...)
    {
    va_list args;
    va_start(args, fmt);
    vprintk(fmt, args);
    va_end(args);
    }
#[no_mangle]
unsafe extern "C" fn pr_ostream_buf(stream: *mut ostream, fmt: *mut c_char, ...) -> void __ostream_printf {
    static void __ostream_printf pr_ostream_buf(struct ostream *stream, char *fmt, ...)
    {
    int ret, len = stream.size - stream.used;
    va_list args;
    va_start(args, fmt);
    ret = vsnprintf(stream.buf + stream.used, len, fmt, args);
    va_end(args);
    if (ret > 0)
    stream.used += min(ret, len);
    }

    ({ \
    struct ostream *_s = (stream); \
    \
    _s.func(_s, fmt, ##args); \
    })
    static void
    __ref_tracker_dir_pr_ostream(struct ref_tracker_dir *dir,
    unsigned int display_limit, struct ostream *s)
    {
    struct ref_tracker_dir_stats *stats;
    let mut i: c_uint = 0, skipped;
    depot_stack_handle_t stack;
    char *sbuf;
    lockdep_assert_held(&dir.lock);
    if (list_empty(&dir.list))
    return;
    stats = ref_tracker_get_stats(dir, display_limit);
    if (IS_ERR(stats)) {
    pr_ostream(s, "%s%s@%p: couldn't get stats, error %pe\n",
    s.prefix, dir.class, dir, stats);
    return;
    }
    sbuf = kmalloc(STACK_BUF_SIZE, GFP_NOWAIT);
    for (i = 0, skipped = stats.total; i < stats.count; ++i) {
    stack = stats.stacks[i].stack_handle;
    if (sbuf && !stack_depot_snprint(stack, sbuf, STACK_BUF_SIZE, 4))
    sbuf[0] = 0;
    pr_ostream(s, "%s%s@%p has %d/%d users at\n%s\n", s.prefix,
    dir.class, dir, stats.stacks[i].count,
    stats.total, sbuf);
    skipped -= stats.stacks[i].count;
    }
    if (skipped)
    pr_ostream(s, "%s%s@%p skipped reports about %d/%d users.\n",
    s.prefix, dir.class, dir, skipped, stats.total);
    kfree(sbuf);
    kfree(stats);
    }
    void ref_tracker_dir_print_locked(struct ref_tracker_dir *dir,
    unsigned int display_limit)
    {
    struct ostream os = { .func = pr_ostream_log,
    .prefix = "ref_tracker: " };
    __ref_tracker_dir_pr_ostream(dir, display_limit, &os);
    }
    EXPORT_SYMBOL(ref_tracker_dir_print_locked);
    void ref_tracker_dir_print(struct ref_tracker_dir *dir,
    unsigned int display_limit)
    {
    unsigned long flags;
    spin_lock_irqsave(&dir.lock, flags);
    ref_tracker_dir_print_locked(dir, display_limit);
    spin_unlock_irqrestore(&dir.lock, flags);
    }
    EXPORT_SYMBOL(ref_tracker_dir_print);
#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_snprint(dir: *mut ref_tracker_dir, buf: *mut c_char, size: usize) -> c_int {
    int ref_tracker_dir_snprint(struct ref_tracker_dir *dir, char *buf, size_t size)
    {
    struct ostream os = { .func = pr_ostream_buf,
    .prefix = "ref_tracker: ",
    .buf = buf,
    .size = size };
    unsigned long flags;
    spin_lock_irqsave(&dir.lock, flags);
    __ref_tracker_dir_pr_ostream(dir, 16, &os);
    spin_unlock_irqrestore(&dir.lock, flags);
    return os.used;
    }
    EXPORT_SYMBOL(ref_tracker_dir_snprint);
#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_exit(dir: *mut ref_tracker_dir) {
    void ref_tracker_dir_exit(struct ref_tracker_dir *dir)
    {
    struct ref_tracker *tracker, *n;
    unsigned long flags;
    let mut leak: bool = false;
    dir.dead = true;
//
// The xarray entries must be marked before the dir->lock is taken to
// protect simultaneous debugfs readers.
//
    ref_tracker_debugfs_mark(dir);
    spin_lock_irqsave(&dir.lock, flags);
    list_for_each_entry_safe(tracker, n, &dir.quarantine, head) {
    list_del(&tracker.head);
    kfree(tracker);
    dir.quarantine_avail++;
    }
    if (!list_empty(&dir.list)) {
    ref_tracker_dir_print_locked(dir, 16);
    leak = true;
    list_for_each_entry_safe(tracker, n, &dir.list, head) {
    list_del(&tracker.head);
    kfree(tracker);
    }
    }
    spin_unlock_irqrestore(&dir.lock, flags);
    WARN_ON_ONCE(leak);
    WARN_ON_ONCE(refcount_read(&dir.untracked) != 1);
    WARN_ON_ONCE(refcount_read(&dir.no_tracker) != 1);
    }
    EXPORT_SYMBOL(ref_tracker_dir_exit);
    int ref_tracker_alloc(struct ref_tracker_dir *dir,
    struct ref_tracker **trackerp,
    gfp_t gfp)
    {
    unsigned long entries[REF_TRACKER_STACK_ENTRIES];
    struct ref_tracker *tracker;
    unsigned int nr_entries;
    let mut gfp_mask: gfp_t = gfp | __GFP_NOWARN;
    unsigned long flags;
    WARN_ON_ONCE(dir.dead);
    if (!trackerp) {
    refcount_inc(&dir.no_tracker);
    return 0;
    }
    if (gfp & __GFP_DIRECT_RECLAIM)
    gfp_mask |= __GFP_NOFAIL;
// trackerp = tracker = kzalloc_obj(*tracker, gfp_mask);
    if (unlikely(!tracker)) {
    pr_err_once("memory allocation failure, unreliable refcount tracker.\n");
    refcount_inc(&dir.untracked);
    return -ENOMEM;
    }
    nr_entries = stack_trace_save(entries, ARRAY_SIZE(entries), 1);
    tracker.alloc_stack_handle = stack_depot_save(entries, nr_entries, gfp);
    spin_lock_irqsave(&dir.lock, flags);
    list_add(&tracker.head, &dir.list);
    spin_unlock_irqrestore(&dir.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ref_tracker_alloc);
    int ref_tracker_free(struct ref_tracker_dir *dir,
    struct ref_tracker **trackerp)
    {
    unsigned long entries[REF_TRACKER_STACK_ENTRIES];
    depot_stack_handle_t stack_handle;
    struct ref_tracker *tracker;
    unsigned int nr_entries;
    unsigned long flags;
    WARN_ON_ONCE(dir.dead);
    if (!trackerp) {
    refcount_dec(&dir.no_tracker);
    return 0;
    }
    tracker = *trackerp;
    if (!tracker) {
    refcount_dec(&dir.untracked);
    return -EEXIST;
    }
    nr_entries = stack_trace_save(entries, ARRAY_SIZE(entries), 1);
    stack_handle = stack_depot_save(entries, nr_entries,
    GFP_NOWAIT);
    spin_lock_irqsave(&dir.lock, flags);
    if (tracker.dead) {
    pr_err("reference already released.\n");
    if (tracker.alloc_stack_handle) {
    pr_err("allocated in:\n");
    stack_depot_print(tracker.alloc_stack_handle);
    }
    if (tracker.free_stack_handle) {
    pr_err("freed in:\n");
    stack_depot_print(tracker.free_stack_handle);
    }
    spin_unlock_irqrestore(&dir.lock, flags);
    WARN_ON_ONCE(1);
    return -EINVAL;
    }
    tracker.dead = true;
    tracker.free_stack_handle = stack_handle;
    list_move_tail(&tracker.head, &dir.quarantine);
    if (!dir.quarantine_avail) {
    tracker = list_first_entry(&dir.quarantine, struct ref_tracker, head);
    list_del(&tracker.head);
    } else {
    dir.quarantine_avail--;
    tracker = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(&dir.lock, flags);
    kfree(tracker);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ref_tracker_free);

    static struct dentry *ref_tracker_debug_dir = (struct dentry *)-ENOENT;
#[no_mangle]
unsafe extern "C" fn pr_ostream_seq(stream: *mut ostream, fmt: *mut c_char, ...) -> void __ostream_printf {
    static void __ostream_printf pr_ostream_seq(struct ostream *stream, char *fmt, ...)
    {
    va_list args;
    va_start(args, fmt);
    seq_vprintf(stream.seq, fmt, args);
    va_end(args);
    }
#[no_mangle]
unsafe extern "C" fn ref_tracker_dir_seq_print(dir: *mut ref_tracker_dir, seq: *mut seq_file) -> c_int {
    static int ref_tracker_dir_seq_print(struct ref_tracker_dir *dir, struct seq_file *seq)
    {
    struct ostream os = { .func = pr_ostream_seq,
    .prefix = "",
    .seq = seq };
    __ref_tracker_dir_pr_ostream(dir, 16, &os);
    return os.used;
    }
#[no_mangle]
unsafe extern "C" fn ref_tracker_debugfs_show(f: *mut seq_file, v: *mut c_void) -> c_int {
    static int ref_tracker_debugfs_show(struct seq_file *f, void *v)
    {
    struct ref_tracker_dir *dir = f.private;
    let mut index: c_ulong = (unsigned long)dir;
    unsigned long flags;
    int ret;
//
// "dir" may not exist at this point if ref_tracker_dir_exit() has
// already been called. Take care not to dereference it until its
// legitimacy is established.
//
// The xa_lock is necessary to ensure that "dir" doesn't disappear
// before its lock can be taken. If it's in the hash and not marked
// dead, then it's safe to take dir->lock which prevents
// ref_tracker_dir_exit() from completing. Once the dir->lock is
// acquired, the xa_lock can be released. All of this must be IRQ-safe.
//
    xa_lock_irqsave(&debugfs_dentries, flags);
    if (!xa_load(&debugfs_dentries, index) ||
    xa_get_mark(&debugfs_dentries, index, REF_TRACKER_DIR_DEAD)) {
    xa_unlock_irqrestore(&debugfs_dentries, flags);
    return -ENODATA;
    }
    spin_lock(&dir.lock);
    xa_unlock(&debugfs_dentries);
    ret = ref_tracker_dir_seq_print(dir, f);
    spin_unlock_irqrestore(&dir.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ref_tracker_debugfs_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int ref_tracker_debugfs_open(struct inode *inode, struct file *filp)
    {
    struct ref_tracker_dir *dir = inode.i_private;
    return single_open(filp, ref_tracker_debugfs_show, dir);
    }
    static const struct file_operations ref_tracker_debugfs_fops = {
    .owner		= THIS_MODULE,
    .open		= ref_tracker_debugfs_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= single_release,
    };
//
// ref_tracker_dir_debugfs - create debugfs file for ref_tracker_dir
// @dir: ref_tracker_dir to be associated with debugfs file
//
// In most cases, a debugfs file will be created automatically for every
// ref_tracker_dir. If the object was created before debugfs is brought up
// then that may fail. In those cases, it is safe to call this at a later
// time to create the file.
//
#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_debugfs(dir: *mut ref_tracker_dir) {
    void ref_tracker_dir_debugfs(struct ref_tracker_dir *dir)
    {
    char name[NAME_MAX + 1];
    struct dentry *dentry;
    int ret;
// No-op if already created
    dentry = xa_load(&debugfs_dentries, (unsigned long)dir);
    if (dentry && !xa_is_err(dentry))
    return;
    ret = snprintf(name, sizeof(name), "%s@%p", dir.class, dir);
    name[sizeof(name) - 1] = '\0';
    if (ret < sizeof(name)) {
    dentry = debugfs_create_file(name, S_IFREG | 0400,
    ref_tracker_debug_dir, dir,
    &ref_tracker_debugfs_fops);
    if (!IS_ERR(dentry)) {
    void *old;
    old = xa_store_irq(&debugfs_dentries, (unsigned long)dir,
    dentry, GFP_KERNEL);
    if (xa_is_err(old))
    debugfs_remove(dentry);
    else
    WARN_ON_ONCE(old);
    }
    }
    }
    EXPORT_SYMBOL(ref_tracker_dir_debugfs);
#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_symlink(dir: *mut ref_tracker_dir, fmt: *const c_char, ...) -> void __ostream_printf {
    void __ostream_printf ref_tracker_dir_symlink(struct ref_tracker_dir *dir, const char *fmt, ...)
    {
    char name[NAME_MAX + 1];
    struct dentry *symlink, *dentry;
    va_list args;
    int ret;
    symlink = xa_load(&debugfs_symlinks, (unsigned long)dir);
    dentry = xa_load(&debugfs_dentries, (unsigned long)dir);
// Already created?
    if (symlink && !xa_is_err(symlink))
    return;
    if (!dentry || xa_is_err(dentry))
    return;
    va_start(args, fmt);
    ret = vsnprintf(name, sizeof(name), fmt, args);
    va_end(args);
    name[sizeof(name) - 1] = '\0';
    if (ret < sizeof(name)) {
    symlink = debugfs_create_symlink(name, ref_tracker_debug_dir,
    dentry.d_name.name);
    if (!IS_ERR(symlink)) {
    void *old;
    old = xa_store_irq(&debugfs_symlinks, (unsigned long)dir,
    symlink, GFP_KERNEL);
    if (xa_is_err(old))
    debugfs_remove(symlink);
    else
    WARN_ON_ONCE(old);
    }
    }
    }
    EXPORT_SYMBOL(ref_tracker_dir_symlink);
#[no_mangle]
unsafe extern "C" fn debugfs_reap_work(work: *mut work_struct) {
    static void debugfs_reap_work(struct work_struct *work)
    {
    struct dentry *dentry;
    unsigned long index;
    bool reaped;
    do {
    reaped = false;
    xa_for_each_marked(&debugfs_symlinks, index, dentry, REF_TRACKER_DIR_DEAD) {
    xa_erase_irq(&debugfs_symlinks, index);
    debugfs_remove(dentry);
    reaped = true;
    }
    xa_for_each_marked(&debugfs_dentries, index, dentry, REF_TRACKER_DIR_DEAD) {
    xa_erase_irq(&debugfs_dentries, index);
    debugfs_remove(dentry);
    reaped = true;
    }
    } while (reaped);
    }
#[no_mangle]
unsafe extern "C" fn ref_tracker_debugfs_postcore_init() -> int __init {
    static int __init ref_tracker_debugfs_postcore_init(void)
    {
    INIT_WORK(&debugfs_reap_worker, debugfs_reap_work);
    xa_init_flags(&debugfs_dentries, XA_FLAGS_LOCK_IRQ);
    xa_init_flags(&debugfs_symlinks, XA_FLAGS_LOCK_IRQ);
    return 0;
    }
    postcore_initcall(ref_tracker_debugfs_postcore_init);
#[no_mangle]
unsafe extern "C" fn ref_tracker_debugfs_late_init() -> int __init {
    static int __init ref_tracker_debugfs_late_init(void)
    {
    ref_tracker_debug_dir = debugfs_create_dir("ref_tracker", core::ptr::null_mut());
    return 0;
    }
    late_initcall(ref_tracker_debugfs_late_init);
