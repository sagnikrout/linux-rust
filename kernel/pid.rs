//! Automatically rewritten from C to Rust
//! Source: kernel/pid.c
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
// Generic pidhash and scalable, time-bounded PID allocator
//
// (C) 2002-2003 Nadia Yvette Chambers, IBM
// (C) 2004 Nadia Yvette Chambers, Oracle
// (C) 2002-2004 Ingo Molnar, Red Hat
//
// pid-structures are backing objects for tasks sharing a given ID to chain
// against. There is very little to them aside from hashing them and
// parking tasks using given ID's on a list.
//
// The hash is always changed with the tasklist_lock write-acquired,
// and the hash is only accessed with the tasklist_lock at least
// read-acquired, so there's no additional SMP locking needed here.
//
// We have a list of bitmap pages, which bitmaps represent the PID space.
// Allocating and freeing PIDs is completely lockless. The worst-case
// allocation scenario when all but one out of 1 million PIDs possible are
// allocated already: the scanning of 32 list entries and at most PAGE_SIZE
// bytes. The typical fastpath is a single successful setbit. Freeing is O(1).
//
// Pid namespaces:
// (C) 2007 Pavel Emelyanov <xemul@openvz.org>, OpenVZ, SWsoft Inc.
// (C) 2007 Sukadev Bhattiprolu <sukadev@us.ibm.com>, IBM
// Many thanks to Oleg Nesterov for comments and help
//

    struct pid init_struct_pid = {
    .count		= REFCOUNT_INIT(1),
    .tasks		= {
    { .first = core::ptr::null_mut() },
    { .first = core::ptr::null_mut() },
    { .first = core::ptr::null_mut() },
    },
    .level		= 0,
    .numbers	= { {
    .nr		= 0,
    .ns		= &init_pid_ns,
    }, }
    };
    let mut pid_max_min: static int = RESERVED_PIDS + 1;
    let mut pid_max_max: static int = PID_MAX_LIMIT;
//
// PID-map pages start out as NULL, they get allocated upon
// first use and are never deallocated. This way a low pid_max
// value does not cause lots of bitmaps to be allocated, but
// the scheme scales to up to 4 million PIDs, runtime.
//
    struct pid_namespace init_pid_ns = {
    .ns = NS_COMMON_INIT(init_pid_ns),
    .idr = IDR_INIT(init_pid_ns.idr),
    .pid_allocated = PIDNS_ADDING,
    .level = 0,
    .child_reaper = &init_task,
    .user_ns = &init_user_ns,
    .pid_max = PID_MAX_DEFAULT,

    .memfd_noexec_scope = MEMFD_NOEXEC_SCOPE_EXEC,

    };
    EXPORT_SYMBOL_GPL(init_pid_ns);
    static  __cacheline_aligned_in_smp DEFINE_SPINLOCK(pidmap_lock);
#[no_mangle]
pub unsafe extern "C" fn put_pid(pid: *mut pid) {
    void put_pid(struct pid *pid)
    {
    struct pid_namespace *ns;
    if (!pid)
    return;
    ns = pid.numbers[pid.level].ns;
    if (refcount_dec_and_test(&pid.count)) {
    pidfs_free_pid(pid);
    kmem_cache_free(ns.pid_cachep, pid);
    put_pid_ns(ns);
    }
    }
    EXPORT_SYMBOL_GPL(put_pid);
#[no_mangle]
unsafe extern "C" fn delayed_put_pid(rhp: *mut rcu_head) {
    static void delayed_put_pid(struct rcu_head *rhp)
    {
    struct pid *pid = container_of(rhp, struct pid, rcu);
    put_pid(pid);
    }
#[no_mangle]
pub unsafe extern "C" fn free_pid(pid: *mut pid) {
    void free_pid(struct pid *pid)
    {
    int i;
    struct pid_namespace *active_ns;
    lockdep_assert_not_held(&tasklist_lock);
    active_ns = pid.numbers[pid.level].ns;
    ns_ref_active_put(active_ns);
    spin_lock(&pidmap_lock);
    for (i = 0; i <= pid.level; i++) {
    struct upid *upid = pid.numbers + i;
    struct pid_namespace *ns = upid.ns;
    switch (--ns.pid_allocated) {
    case 2:
    case 1:
// When all that is left in the pid namespace
// is the reaper wake up the reaper.  The reaper
// may be sleeping in zap_pid_ns_processes().
//
    wake_up_process(READ_ONCE(ns.child_reaper));
    break;
    case PIDNS_ADDING:
// Only possible if the 1st fork fails
    WARN_ON(READ_ONCE(ns.child_reaper));
    break;
    }
    idr_remove(&ns.idr, upid.nr);
    }
    spin_unlock(&pidmap_lock);
    pidfs_remove_pid(pid);
    call_rcu(&pid.rcu, delayed_put_pid);
    }
#[no_mangle]
pub unsafe extern "C" fn free_pids(pids: *mut pid) {
    void free_pids(struct pid **pids)
    {
    int tmp;
//
// This can batch pidmap_lock.
//
    for (tmp = PIDTYPE_MAX; --tmp >= 0; )
    if (pids[tmp])
    free_pid(pids[tmp]);
    }
    struct pid *alloc_pid(struct pid_namespace *ns, pid_t *arg_set_tid,
    size_t arg_set_tid_size)
    {
    int set_tid[MAX_PID_NS_LEVEL + 1] = {};
    int pid_max[MAX_PID_NS_LEVEL + 1] = {};
    struct pid *pid;
    enum pid_type type;
    int i, nr;
    struct pid_namespace *tmp;
    struct upid *upid;
    let mut retval: c_int = -ENOMEM;
    bool retried_preload;
//
// arg_set_tid_size contains the size of the arg_set_tid array. Starting at
// the most nested currently active PID namespace it tells alloc_pid()
// which PID to set for a process in that most nested PID namespace
// up to arg_set_tid_size PID namespaces. It does not have to set the PID
// for a process in all nested PID namespaces but arg_set_tid_size must
// never be greater than the current ns->level + 1.
//
    if (arg_set_tid_size > ns.level + 1)
    return ERR_PTR(-EINVAL);
//
// Prep before we take locks:
//
// 1. allocate and fill in pid struct
//
    pid = kmem_cache_alloc(ns.pid_cachep, GFP_KERNEL);
    if (!pid)
    return ERR_PTR(retval);
    get_pid_ns(ns);
    pid.level = ns.level;
    refcount_set(&pid.count, 1);
    spin_lock_init(&pid.lock);
    for (type = 0; type < PIDTYPE_MAX; ++type)
    INIT_HLIST_HEAD(&pid.tasks[type]);
    init_waitqueue_head(&pid.wait_pidfd);
    INIT_HLIST_HEAD(&pid.inodes);
    pidfs_prepare_pid(pid);
//
// 2. perm check checkpoint_restore_ns_capable()
//
// This stores found pid_max to make sure the used value is the same should
// later code need it.
//
    for (tmp = ns, i = ns.level; i >= 0; i--) {
    pid_max[ns.level - i] = READ_ONCE(tmp.pid_max);
    if (arg_set_tid_size) {
    let mut tid: c_int = set_tid[ns.level - i] = arg_set_tid[ns.level - i];
    retval = -EINVAL;
    if (tid < 1 || tid >= pid_max[ns.level - i])
    goto out_abort;
    retval = -EPERM;
    if (!checkpoint_restore_ns_capable(tmp.user_ns))
    goto out_abort;
    arg_set_tid_size--;
    }
    tmp = tmp.parent;
    }
//
// Prep is done, id allocation goes here:
//
    retried_preload = false;
    idr_preload(GFP_KERNEL);
    spin_lock(&pidmap_lock);
// For the case when the previous attempt to create init failed
    if (ns.pid_allocated == PIDNS_ADDING)
    idr_set_cursor(&ns.idr, 0);
    for (tmp = ns, i = ns.level; i >= 0;) {
    let mut tid: c_int = set_tid[ns.level - i];
    if (tid) {
    nr = idr_alloc(&tmp.idr, core::ptr::null_mut(), tid,
    tid + 1, GFP_ATOMIC);
//
// If ENOSPC is returned it means that the PID is
// alreay in use. Return EEXIST in that case.
//
    if (nr == -ENOSPC)
    nr = -EEXIST;
    } else {
    let mut pid_min: c_int = 1;
//
// init really needs pid 1, but after reaching the
// maximum wrap back to RESERVED_PIDS
//
    if (idr_get_cursor(&tmp.idr) > RESERVED_PIDS)
    pid_min = RESERVED_PIDS;
//
// Store a null pointer so find_pid_ns does not find
// a partially initialized PID (see below).
//
    nr = idr_alloc_cyclic(&tmp.idr, core::ptr::null_mut(), pid_min,
    pid_max[ns.level - i], GFP_ATOMIC);
    if (nr == -ENOSPC)
    nr = -EAGAIN;
    }
    if (unlikely(nr < 0)) {
//
// Preload more memory if idr_alloc{,cyclic} failed with -ENOMEM.
//
// The IDR API only allows us to preload memory for one call, while we may end
// up doing several under pidmap_lock with GFP_ATOMIC. The situation may be
// salvageable with GFP_KERNEL. But make sure to not loop indefinitely if preload
// did not help (the routine unfortunately returns void, so we have no idea
// if it got anywhere).
//
// The lock can be safely dropped and picked up as historically pid allocation
// for different namespaces was *not* atomic -- we try to hold on to it the
// entire time only for performance reasons.
//
    if (nr == -ENOMEM && !retried_preload) {
    spin_unlock(&pidmap_lock);
    idr_preload_end();
    retried_preload = true;
    idr_preload(GFP_KERNEL);
    spin_lock(&pidmap_lock);
    continue;
    }
    retval = nr;
    goto out_free;
    }
    pid.numbers[i].nr = nr;
    pid.numbers[i].ns = tmp;
    i--;
    retried_preload = false;
//
// PID 1 (init) must be created first.
//
    if (!READ_ONCE(tmp.child_reaper) && nr != 1) {
    retval = -EINVAL;
    goto out_free;
    }
    tmp = tmp.parent;
    }
//
// ENOMEM is not the most obvious choice especially for the case
// where the child subreaper has already exited and the pid
// namespace denies the creation of any new processes. But ENOMEM
// is what we have exposed to userspace for a long time and it is
// documented behavior for pid namespaces. So we can't easily
// change it even if there were an error code better suited.
//
// This can't be done earlier because we need to preserve other
// error conditions.
//
// We need this even if copy_process() does the same check. If two
// or more tasks from parent namespace try to inject a child into a
// dead namespace, one of free_pid() calls from the copy_process()
// error path may try to wakeup the possibly freed ns->child_reaper.
//
    retval = -ENOMEM;
    for (upid = pid.numbers + ns.level; upid >= pid.numbers; --upid)
    if (unlikely(!(upid.ns.pid_allocated & PIDNS_ADDING)))
    goto out_free;
    for (upid = pid.numbers + ns.level; upid >= pid.numbers; --upid) {
// Make the PID visible to find_pid_ns.
    idr_replace(&upid.ns.idr, pid, upid.nr);
    upid.ns.pid_allocated++;
    }
    spin_unlock(&pidmap_lock);
    idr_preload_end();
    ns_ref_active_get(ns);
    retval = pidfs_add_pid(pid);
    if (unlikely(retval)) {
    free_pid(pid);
    pid = ERR_PTR(-ENOMEM);
    }
    return pid;
    out_free:
    while (++i <= ns.level) {
    upid = pid.numbers + i;
    idr_remove(&upid.ns.idr, upid.nr);
    }
    spin_unlock(&pidmap_lock);
    idr_preload_end();
    out_abort:
    put_pid_ns(ns);
    kmem_cache_free(ns.pid_cachep, pid);
    return ERR_PTR(retval);
    }
#[no_mangle]
pub unsafe extern "C" fn disable_pid_allocation(ns: *mut pid_namespace) {
    void disable_pid_allocation(struct pid_namespace *ns)
    {
    spin_lock(&pidmap_lock);
    ns.pid_allocated &= ~PIDNS_ADDING;
    spin_unlock(&pidmap_lock);
    }
    struct pid *find_pid_ns(int nr, struct pid_namespace *ns)
    {
    return idr_find(&ns.idr, nr);
    }
    EXPORT_SYMBOL_GPL(find_pid_ns);
    struct pid *find_vpid(int nr)
    {
    return find_pid_ns(nr, task_active_pid_ns(current));
    }
    EXPORT_SYMBOL_GPL(find_vpid);
    static struct pid **task_pid_ptr(struct task_struct *task, enum pid_type type)
    {
    return (type == PIDTYPE_PID) ?
    &task.thread_pid :
    &task.signal.pids[type];
    }
//
// attach_pid() must be called with the tasklist_lock write-held.
//
#[no_mangle]
pub unsafe extern "C" fn attach_pid(task: *mut task_struct, type: enum pid_type) {
    void attach_pid(struct task_struct *task, enum pid_type type)
    {
    struct pid *pid;
    lockdep_assert_held_write(&tasklist_lock);
    pid = *task_pid_ptr(task, type);
    hlist_add_head_rcu(&task.pid_links[type], &pid.tasks[type]);
    }
    static void __change_pid(struct pid **pids, struct task_struct *task,
    enum pid_type type, struct pid *new)
    {
    struct pid **pid_ptr, *pid;
    int tmp;
    lockdep_assert_held_write(&tasklist_lock);
    pid_ptr = task_pid_ptr(task, type);
    pid = *pid_ptr;
    hlist_del_rcu(&task.pid_links[type]);
// pid_ptr = new;
    for (tmp = PIDTYPE_MAX; --tmp >= 0; )
    if (pid_has_task(pid, tmp))
    return;
    WARN_ON(pids[type]);
    pids[type] = pid;
    }
#[no_mangle]
pub unsafe extern "C" fn detach_pid(pids: *mut pid, task: *mut task_struct, type: enum pid_type) {
    void detach_pid(struct pid **pids, struct task_struct *task, enum pid_type type)
    {
    __change_pid(pids, task, type, core::ptr::null_mut());
    }
    void change_pid(struct pid **pids, struct task_struct *task, enum pid_type type,
    struct pid *pid)
    {
    __change_pid(pids, task, type, pid);
    attach_pid(task, type);
    }
#[no_mangle]
pub unsafe extern "C" fn exchange_tids(left: *mut task_struct, right: *mut task_struct) {
    void exchange_tids(struct task_struct *left, struct task_struct *right)
    {
    struct pid *pid1 = left.thread_pid;
    struct pid *pid2 = right.thread_pid;
    struct hlist_head *head1 = &pid1.tasks[PIDTYPE_PID];
    struct hlist_head *head2 = &pid2.tasks[PIDTYPE_PID];
    lockdep_assert_held_write(&tasklist_lock);
// Swap the single entry tid lists
    hlists_swap_heads_rcu(head1, head2);
// Swap the per task_struct pid
    rcu_assign_pointer(left.thread_pid, pid2);
    rcu_assign_pointer(right.thread_pid, pid1);
// Swap the cached value
    WRITE_ONCE(left.pid, pid_nr(pid2));
    WRITE_ONCE(right.pid, pid_nr(pid1));
    }
// transfer_pid is an optimization of attach_pid(new), detach_pid(old)
    void transfer_pid(struct task_struct *old, struct task_struct *new,
    enum pid_type type)
    {
    WARN_ON_ONCE(type == PIDTYPE_PID);
    lockdep_assert_held_write(&tasklist_lock);
    hlist_replace_rcu(&old.pid_links[type], &new.pid_links[type]);
    }
    struct task_struct *pid_task(struct pid *pid, enum pid_type type)
    {
    struct task_struct *result = core::ptr::null_mut();
    if (pid) {
    struct hlist_node *first;
    first = rcu_dereference_check(hlist_first_rcu(&pid.tasks[type]),
    lockdep_tasklist_lock_is_held());
    if (first)
    result = hlist_entry(first, struct task_struct, pid_links[(type)]);
    }
    return result;
    }
    EXPORT_SYMBOL(pid_task);
//
// Must be called under rcu_read_lock().
//
    struct task_struct *find_task_by_pid_ns(pid_t nr, struct pid_namespace *ns)
    {
    RCU_LOCKDEP_WARN(!rcu_read_lock_held(),
    "find_task_by_pid_ns() needs rcu_read_lock() protection");
    return pid_task(find_pid_ns(nr, ns), PIDTYPE_PID);
    }
    struct task_struct *find_task_by_vpid(pid_t vnr)
    {
    return find_task_by_pid_ns(vnr, task_active_pid_ns(current));
    }
    struct task_struct *find_get_task_by_vpid(pid_t nr)
    {
    struct task_struct *task;
    rcu_read_lock();
    task = find_task_by_vpid(nr);
    if (task)
    get_task_struct(task);
    rcu_read_unlock();
    return task;
    }
    struct pid *get_task_pid(struct task_struct *task, enum pid_type type)
    {
    struct pid *pid;
    rcu_read_lock();
    pid = get_pid(rcu_dereference(*task_pid_ptr(task, type)));
    rcu_read_unlock();
    return pid;
    }
    EXPORT_SYMBOL_GPL(get_task_pid);
    struct task_struct *get_pid_task(struct pid *pid, enum pid_type type)
    {
    struct task_struct *result;
    rcu_read_lock();
    result = pid_task(pid, type);
    if (result)
    get_task_struct(result);
    rcu_read_unlock();
    return result;
    }
    EXPORT_SYMBOL_GPL(get_pid_task);
    struct pid *find_get_pid(pid_t nr)
    {
    struct pid *pid;
    rcu_read_lock();
    pid = get_pid(find_vpid(nr));
    rcu_read_unlock();
    return pid;
    }
    EXPORT_SYMBOL_GPL(find_get_pid);
#[no_mangle]
pub unsafe extern "C" fn pid_nr_ns(pid: *mut pid, ns: *mut pid_namespace) -> pid_t {
    pid_t pid_nr_ns(struct pid *pid, struct pid_namespace *ns)
    {
    struct upid *upid;
    let mut nr: pid_t = 0;
    if (pid && ns && ns.level <= pid.level) {
    upid = &pid.numbers[ns.level];
    if (upid.ns == ns)
    nr = upid.nr;
    }
    return nr;
    }
    EXPORT_SYMBOL_GPL(pid_nr_ns);
#[no_mangle]
pub unsafe extern "C" fn pid_vnr(pid: *mut pid) -> pid_t {
    pid_t pid_vnr(struct pid *pid)
    {
    return pid_nr_ns(pid, task_active_pid_ns(current));
    }
    EXPORT_SYMBOL_GPL(pid_vnr);
    pid_t __task_pid_nr_ns(struct task_struct *task, enum pid_type type,
    struct pid_namespace *ns)
    {
    let mut nr: pid_t = 0;
    rcu_read_lock();
    if (!ns)
    ns = task_active_pid_ns(current);
    nr = pid_nr_ns(rcu_dereference(*task_pid_ptr(task, type)), ns);
    rcu_read_unlock();
    return nr;
    }
    EXPORT_SYMBOL(__task_pid_nr_ns);
    struct pid_namespace *task_active_pid_ns(struct task_struct *tsk)
    {
    return ns_of_pid(task_pid(tsk));
    }
    EXPORT_SYMBOL_GPL(task_active_pid_ns);
//
// Used by proc to find the first pid that is greater than or equal to nr.
//
// If there is a pid at nr this function is exactly the same as find_pid_ns.
//
    struct pid *find_ge_pid(int nr, struct pid_namespace *ns)
    {
    return idr_get_next(&ns.idr, &nr);
    }
    EXPORT_SYMBOL_GPL(find_ge_pid);
    struct pid *pidfd_get_pid(unsigned int fd, unsigned int *flags)
    {
    CLASS(fd, f)(fd);
    struct pid *pid;
    if (fd_empty(f))
    return ERR_PTR(-EBADF);
    pid = pidfd_pid(fd_file(f));
    if (!IS_ERR(pid)) {
    get_pid(pid);
// flags = fd_file(f)->f_flags;
    }
    return pid;
    }
//
// pidfd_get_task() - Get the task associated with a pidfd
//
// @pidfd: pidfd for which to get the task
// @flags: flags associated with this pidfd
//
// Return the task associated with @pidfd. The function takes a reference on
// the returned task. The caller is responsible for releasing that reference.
//
// Return: On success, the task_struct associated with the pidfd.
// On error, a negative errno number will be returned.
//
    struct task_struct *pidfd_get_task(int pidfd, unsigned int *flags)
    {
    let mut f_flags: c_uint = 0;
    struct pid *pid;
    struct task_struct *task;
    enum pid_type type;
    switch (pidfd) {
    case  PIDFD_SELF_THREAD:
    type = PIDTYPE_PID;
    pid = get_task_pid(current, type);
    break;
    case  PIDFD_SELF_THREAD_GROUP:
    type = PIDTYPE_TGID;
    pid = get_task_pid(current, type);
    break;
    default:
    pid = pidfd_get_pid(pidfd, &f_flags);
    if (IS_ERR(pid))
    return ERR_CAST(pid);
    type = PIDTYPE_TGID;
    break;
    }
    task = get_pid_task(pid, type);
    put_pid(pid);
    if (!task)
    return ERR_PTR(-ESRCH);
// flags = f_flags;
    return task;
    }
//
// pidfd_create() - Create a new pid file descriptor.
//
// @pid:   struct pid that the pidfd will reference
// @flags: flags to pass
//
// This creates a new pid file descriptor with the O_CLOEXEC flag set.
//
// Note, that this function can only be called after the fd table has
// been unshared to avoid leaking the pidfd to the new process.
//
// This symbol should not be explicitly exported to loadable modules.
//
// Return: On success, a cloexec pidfd is returned.
// On error, a negative errno number will be returned.
//
#[no_mangle]
unsafe extern "C" fn pidfd_create(pid: *mut pid, flags: c_uint) -> c_int {
    static int pidfd_create(struct pid *pid, unsigned int flags)
    {
    int pidfd;
    struct file *pidfd_file;
    pidfd = pidfd_prepare(pid, flags, &pidfd_file);
    if (pidfd < 0)
    return pidfd;
    fd_install(pidfd, pidfd_file);
    return pidfd;
    }
//
// sys_pidfd_open() - Open new pid file descriptor.
//
// @pid:   pid for which to retrieve a pidfd
// @flags: flags to pass
//
// This creates a new pid file descriptor with the O_CLOEXEC flag set for
// the task identified by @pid. Without PIDFD_THREAD flag the target task
// must be a thread-group leader.
//
// Return: On success, a cloexec pidfd is returned.
// On error, a negative errno number will be returned.
//
    SYSCALL_DEFINE2(pidfd_open, pid_t, pid, unsigned int, flags)
    {
    int fd;
    struct pid *p;
    if (flags & ~(PIDFD_NONBLOCK | PIDFD_THREAD))
    return -EINVAL;
    if (pid <= 0)
    return -EINVAL;
    p = find_get_pid(pid);
    if (!p)
    return -ESRCH;
    fd = pidfd_create(p, flags);
    put_pid(p);
    return fd;
    }

    static struct ctl_table_set *pid_table_root_lookup(struct ctl_table_root *root)
    {
    return &task_active_pid_ns(current).set;
    }
#[no_mangle]
unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    static int set_is_seen(struct ctl_table_set *set)
    {
    return &task_active_pid_ns(current).set == set;
    }
    static int pid_table_root_permissions(struct ctl_table_header *head,
    const struct ctl_table *table)
    {
    struct pid_namespace *pidns =
    container_of(head.set, struct pid_namespace, set);
    let mut mode: c_int = table.mode;
    if (ns_capable_noaudit(pidns.user_ns, CAP_SYS_ADMIN) ||
    uid_eq(current_euid(), make_kuid(pidns.user_ns, 0)))
    mode = (mode & S_IRWXU) >> 6;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: in_egroup_p(make_kgid(pidns->user_ns, _arg: 0))) -> else {
    else if (in_egroup_p(make_kgid(pidns.user_ns, 0)))
    mode = (mode & S_IRWXG) >> 3;
    else
    mode = mode & S_IROTH;
    return (mode << 6) | (mode << 3) | mode;
    }
    static void pid_table_root_set_ownership(struct ctl_table_header *head,
    kuid_t *uid, kgid_t *gid)
    {
    struct pid_namespace *pidns =
    container_of(head.set, struct pid_namespace, set);
    kuid_t ns_root_uid;
    kgid_t ns_root_gid;
    ns_root_uid = make_kuid(pidns.user_ns, 0);
    if (uid_valid(ns_root_uid))
// uid = ns_root_uid;
    ns_root_gid = make_kgid(pidns.user_ns, 0);
    if (gid_valid(ns_root_gid))
// gid = ns_root_gid;
    }
    static struct ctl_table_root pid_table_root = {
    .lookup		= pid_table_root_lookup,
    .permissions	= pid_table_root_permissions,
    .set_ownership	= pid_table_root_set_ownership,
    };
    static const struct ctl_table pid_table[] = {
    {
    .procname	= "pid_max",
    .data		= &init_pid_ns.pid_max,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &pid_max_min,
    .extra2		= &pid_max_max,
    },
    };

#[no_mangle]
pub unsafe extern "C" fn register_pidns_sysctls(pidns: *mut pid_namespace) -> c_int {
    int register_pidns_sysctls(struct pid_namespace *pidns)
    {

    struct ctl_table *tbl;
    setup_sysctl_set(&pidns.set, &pid_table_root, set_is_seen);
    tbl = kmemdup(pid_table, sizeof(pid_table), GFP_KERNEL);
    if (!tbl)
    return -ENOMEM;
    tbl.data = &pidns.pid_max;
    pidns.pid_max = min(pid_max_max, max_t(int, pidns.pid_max,
    PIDS_PER_CPU_DEFAULT * num_possible_cpus()));
    pidns.sysctls = __register_sysctl_table(&pidns.set, "kernel", tbl,
    ARRAY_SIZE(pid_table));
    if (!pidns.sysctls) {
    kfree(tbl);
    retire_sysctl_set(&pidns.set);
    return -ENOMEM;
    }

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_pidns_sysctls(pidns: *mut pid_namespace) {
    void unregister_pidns_sysctls(struct pid_namespace *pidns)
    {

    const struct ctl_table *tbl;
    tbl = pidns.sysctls.ctl_table_arg;
    unregister_sysctl_table(pidns.sysctls);
    retire_sysctl_set(&pidns.set);
    kfree(tbl);

    }
#[no_mangle]
pub unsafe extern "C" fn pid_idr_init() -> void __init {
    void __init pid_idr_init(void)
    {
// Verify no one has done anything silly:
    BUILD_BUG_ON(PID_MAX_LIMIT >= PIDNS_ADDING);
// bump default and minimum pid_max based on number of cpus
    init_pid_ns.pid_max = min(pid_max_max, max_t(int, init_pid_ns.pid_max,
    PIDS_PER_CPU_DEFAULT * num_possible_cpus()));
    pid_max_min = max_t(int, pid_max_min,
    PIDS_PER_CPU_MIN * num_possible_cpus());
    pr_info("pid_max: default: %u minimum: %u\n", init_pid_ns.pid_max, pid_max_min);
    idr_init(&init_pid_ns.idr);
    init_pid_ns.pid_cachep = kmem_cache_create("pid",
    struct_size_t(struct pid, numbers, 1),
    __alignof__(struct pid),
    SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT,
    core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pid_namespace_sysctl_init() -> __init int {
    static __init int pid_namespace_sysctl_init(void)
    {

// "kernel" directory will have already been initialized.
    BUG_ON(register_pidns_sysctls(&init_pid_ns));

    return 0;
    }
    subsys_initcall(pid_namespace_sysctl_init);
    static struct file *__pidfd_fget(struct task_struct *task, int fd)
    {
    struct file *file;
    int ret;
    ret = down_read_killable(&task.signal.exec_update_lock);
    if (ret)
    return ERR_PTR(ret);
    if (!ptrace_may_access(task, PTRACE_MODE_ATTACH_REALCREDS))
    file = ERR_PTR(-EPERM);
#[no_mangle]
pub unsafe extern "C" fn if(PF_EXITING: task->flags &) -> else {
    else if (task.flags & PF_EXITING)
    file = ERR_PTR(-ESRCH);
    else
    file = fget_task(task, fd);
    up_read(&task.signal.exec_update_lock);
    if (!file) {
//
// It is possible that the target thread is exiting; it can be
// either:
// 1. before exit_signals(), which gives a real fd
// 2. before exit_files() takes the task_lock() gives a real fd
// 3. after exit_files() releases task_lock(), ->files is NULL;
// this has PF_EXITING, since it was set in exit_signals(),
// __pidfd_fget() returns EBADF.
// In case 3 we get EBADF, but that really means ESRCH, since
// the task is currently exiting and has freed its files
// struct, so we fix it up.
//
    if (task.flags & PF_EXITING)
    file = ERR_PTR(-ESRCH);
    else
    file = ERR_PTR(-EBADF);
    }
    return file;
    }
#[no_mangle]
unsafe extern "C" fn pidfd_getfd(pid: *mut pid, fd: c_int) -> c_int {
    static int pidfd_getfd(struct pid *pid, int fd)
    {
    struct task_struct *task;
    struct file *file;
    int ret;
    task = get_pid_task(pid, PIDTYPE_PID);
    if (!task)
    return -ESRCH;
    file = __pidfd_fget(task, fd);
    put_task_struct(task);
    if (IS_ERR(file))
    return PTR_ERR(file);
    ret = receive_fd(file, core::ptr::null_mut(), O_CLOEXEC);
    fput(file);
    return ret;
    }
//
// sys_pidfd_getfd() - Get a file descriptor from another process
//
// @pidfd:	the pidfd file descriptor of the process
// @fd:		the file descriptor number to get
// @flags:	flags on how to get the fd (reserved)
//
// This syscall gets a copy of a file descriptor from another process
// based on the pidfd, and file descriptor number. It requires that
// the calling process has the ability to ptrace the process represented
// by the pidfd. The process which is having its file descriptor copied
// is otherwise unaffected.
//
// Return: On success, a cloexec file descriptor is returned.
// On error, a negative errno number will be returned.
//
    SYSCALL_DEFINE3(pidfd_getfd, int, pidfd, int, fd,
    unsigned int, flags)
    {
    struct pid *pid;
// flags is currently unused - make sure it's unset
    if (flags)
    return -EINVAL;
    CLASS(fd, f)(pidfd);
    if (fd_empty(f))
    return -EBADF;
    pid = pidfd_pid(fd_file(f));
    if (IS_ERR(pid))
    return PTR_ERR(pid);
    return pidfd_getfd(pid, fd);
    }
