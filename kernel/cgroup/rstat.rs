//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/rstat.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-only

pub static mut rstat_base_lock: usize = 0;
pub static mut struct llist_head: usize = 0;
// forward_decl: cgroup_base_stat_flush;
//
// Determines whether a given css can participate in rstat.
// css's that are cgroup::self use rstat for base stats.
// Other css's associated with a subsystem use rstat only when
// they define the ss->css_rstat_flush callback.
//
#[no_mangle]
pub unsafe extern "C" fn css_uses_rstat(css: *mut cgroup_subsys_state) -> bool {
    return css_is_self(css) || css.ss.css_rstat_flush != core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn css_rstat_cpu(css: *mut cgroup_subsys_state, cpu: c_int) -> *mut c_void {
    return per_cpu_ptr(css.rstat_cpu, cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_rstat_base_cpu(cgrp: *mut cgroup, cpu: c_int) -> *mut c_void {
    return per_cpu_ptr(cgrp.rstat_base_cpu, cpu);
    }
    static spinlock_t *ss_rstat_lock(cgroup_subsys *ss)
    {
    if (ss) {
    return &ss.rstat_ss_lock;
    }
    return &rstat_base_lock;
    }
#[no_mangle]
pub unsafe extern "C" fn ss_lhead_cpu(ss: *mut cgroup_subsys, cpu: c_int) -> *mut c_void {
    if (ss) {
    return per_cpu_ptr(ss.lhead, cpu);
    }
    return per_cpu_ptr(&rstat_backlog_list, cpu);
    }
//
// __css_rstat_updated - keep track of updated rstat_cpu
// @css: target cgroup subsystem state
// @cpu: cpu on which rstat_cpu was updated
//
// Atomically inserts the css in the ss's llist for the given cpu. This is
// reentrant safe i.e. safe against softirq, hardirq and nmi. The ss's llist
// will be processed at the flush time to create the update tree.
//
// NOTE: if the user needs the guarantee that the updater either add itself in
// the lockless list or the concurrent flusher flushes its updated stats, a
// memory barrier is needed before the call to __css_rstat_updated() i.e. a
// barrier after updating the per-cpu stats and before calling
// __css_rstat_updated().
//
#[no_mangle]
pub unsafe extern "C" fn __css_rstat_updated(css: *mut cgroup_subsys_state, cpu: c_int) {
pub static mut lhead: *mut c_void = core::ptr::null_mut();
pub static mut rstatc: *mut c_void = core::ptr::null_mut();
pub static mut self: *mut c_void = core::ptr::null_mut();
// Prevent access to uninitialized rstat pointers.
    if (!css_uses_rstat(css)) {
    return;
    }
    lockdep_assert_preemption_disabled();
//
// The lockless insertion below relies on NMI-safe cmpxchg;
// bail out in NMI on archs that don't provide it.
//
    if (!IS_ENABLED!(CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG) && in_nmi()) {
    return;
    }
    rstatc = css_rstat_cpu(css, cpu);
//
// If already on list return. This check is racy and smp_mb() is needed
// to pair it with the smp_mb() in css_process_update_tree() if the
// guarantee that the updated stats are visible to concurrent flusher is
// needed.
//
    if (llist_on_list(&rstatc.lnode)) {
    return;
    }
//
// This function can be renentered by irqs and nmis for the same cgroup
// and may try to insert the same per-cpu lnode into the llist. Note
// that llist_add() does not protect against such scenarios. In addition
// this same per-cpu lnode can be modified through init_llist_node()
// from css_rstat_flush() running on a different CPU.
//
// To protect against such stacked contexts of irqs/nmis, we use the
// fact that lnode points to itself when not on a list and then use
// try_cmpxchg() to atomically set to NULL to select the winner
// which will call llist_add(). The losers can assume the insertion is
// successful and the winner will eventually add the per-cpu lnode to
// the llist.
//
// Please note that we can not use this_cpu_cmpxchg() here as on some
// archs it is not safe against modifications from multiple CPUs.
//
    self = &rstatc.lnode;
    if (!try_cmpxchg(&rstatc.lnode.next, &self, core::ptr::null_mut())) {
    return;
    }
    lhead = ss_lhead_cpu(css.ss, cpu);
    llist_add(&rstatc.lnode, lhead);
    }
//
// BPF-facing wrapper for __css_rstat_updated(). Validate the caller-provided
// CPU before passing it to the internal rstat updater.
//
#[no_mangle]
pub unsafe extern "C" fn css_rstat_updated(css: *mut cgroup_subsys_state, cpu: c_int) -> __bpf_kfunc void {
    if (unlikely(cpu < 0 || cpu >= nr_cpu_ids || !cpu_possible(cpu))) {
    return;
    }
    __css_rstat_updated(css, cpu);
    }
#[no_mangle]
unsafe extern "C" fn __css_process_update_tree(css: *mut cgroup_subsys_state, cpu: c_int) {
// put @css and all ancestors on the corresponding updated lists
    while (true) {
    let mut rstatc = css_rstat_cpu(css, cpu);
    let mut parent = css.parent;
pub static mut prstatc: *mut c_void = core::ptr::null_mut();
//
// Both additions and removals are bottom-up.  If a cgroup
// is already in the tree, all ancestors are.
//
    if (rstatc.updated_next) {
    break;
    }
// Root has no parent to link it to, but mark it busy
    if (!parent) {
    rstatc.updated_next = css;
    break;
    }
    prstatc = css_rstat_cpu(parent, cpu);
    rstatc.updated_next = prstatc.updated_children;
    prstatc.updated_children = css;
    css = parent;
    }
    }
#[no_mangle]
unsafe extern "C" fn css_process_update_tree(ss: *mut cgroup_subsys, cpu: c_int) {
    let mut lhead = ss_lhead_cpu(ss, cpu);
pub static mut lnode: *mut c_void = core::ptr::null_mut();
    while ((lnode = llist_del_first_init(lhead))) {
pub static mut rstatc: *mut c_void = core::ptr::null_mut();
//
// smp_mb() is needed here (more specifically in between
// init_llist_node() and per-cpu stats flushing) if the
// guarantee is required by a rstat user where etiher the
// updater should add itself on the lockless list or the
// flusher flush the stats updated by the updater who have
// observed that they are already on the list. The
// corresponding barrier pair for this one should be before
// __css_rstat_updated() by the user.
//
// For now, there aren't any such user, so not adding the
// barrier here but if such a use-case arise, please add
// smp_mb() here.
//
    rstatc = container_of!(lnode, css_rstat_cpu, lnode);
    __css_process_update_tree(rstatc.owner, cpu);
    }
    }
//
// css_rstat_push_children - push children css's into the given list
// @head: current head of the list (= subtree root)
// @child: first child of the root
// @cpu: target cpu
// Return: A new singly linked list of css's to be flushed
//
// Iteratively traverse down the css_rstat_cpu updated tree level by
// level and push all the parents first before their next level children
// into a singly linked list via the rstat_flush_next pointer built from the
// tail backward like "pushing" css's into a stack. The root is pushed by
// the caller.
//
#[no_mangle]
pub unsafe extern "C" fn css_rstat_push_children(head: *mut cgroup_subsys_state, child: *mut cgroup_subsys_state, cpu: c_int) -> *mut c_void {
    let mut cnext = child;	/* Next head of child css level */
    let mut ghead = core::ptr::null_mut();	/* Head of grandchild css level */
    let mut parent = core::ptr::null_mut();
    let mut grandchild = core::ptr::null_mut();
pub static mut crstatc: *mut c_void = core::ptr::null_mut();
    child.rstat_flush_next = core::ptr::null_mut();
//
// The subsystem rstat lock must be held for the whole duration from
// here as the rstat_flush_next list is being constructed to when
// it is consumed later in css_rstat_flush().
//
    lockdep_assert_held(ss_rstat_lock(head.ss));
//
// Notation: -> updated_next pointer
// => rstat_flush_next pointer
//
// Assuming the following sample updated_children lists:
// P: C1 -> C2 -> P
// C1: G11 -> G12 -> C1
// C2: G21 -> G22 -> C2
//
// After 1st iteration:
// head => C2 => C1 => NULL
// ghead => G21 => G11 => NULL
//
// After 2nd iteration:
// head => G12 => G11 => G22 => G21 => C2 => C1 => NULL
//
// label;
    while (cnext) {
    child = cnext;
    cnext = child.rstat_flush_next;
    parent = child.parent;
// updated_next is parent cgroup terminated if !NULL
    while (child != parent) {
    child.rstat_flush_next = head;
    head = child;
    crstatc = css_rstat_cpu(child, cpu);
    grandchild = crstatc.updated_children;
    if (grandchild != child) {
// Push the grand child to the next level
    crstatc.updated_children = child;
    grandchild.rstat_flush_next = ghead;
    ghead = grandchild;
    }
    child = crstatc.updated_next;
    crstatc.updated_next = core::ptr::null_mut();
    }
    }
    if (ghead) {
    cnext = ghead;
    ghead = core::ptr::null_mut();
// goto;
    }
    return head;
    }
//
// css_rstat_updated_list - build a list of updated css's to be flushed
// @root: root of the css subtree to traverse
// @cpu: target cpu
// Return: A singly linked list of css's to be flushed
//
// Walks the updated rstat_cpu tree on @cpu from @root.  During traversal,
// each returned css is unlinked from the updated tree.
//
// The only ordering guarantee is that, for a parent and a child pair
// covered by a given traversal, the child is before its parent in
// the list.
//
// Note that updated_children is self terminated and points to a list of
// child css's if not empty. Whereas updated_next is like a sibling link
// within the children list and terminated by the parent css. An exception
// here is the css root whose updated_next can be self terminated.
//
#[no_mangle]
pub unsafe extern "C" fn css_rstat_updated_list(root: *mut cgroup_subsys_state, cpu: c_int) -> *mut c_void {
    let mut rstatc = css_rstat_cpu(root, cpu);
    let mut head = core::ptr::null_mut(), *parent, *child;
    css_process_update_tree(root.ss, cpu);
// Return NULL if this subtree is not on-list
    if (!rstatc.updated_next) {
    return core::ptr::null_mut();
    }
//
// Unlink @root from its parent. As the updated_children list is
// singly linked, we have to walk it to find the removal point.
//
    parent = root.parent;
    if (parent) {
pub static mut prstatc: *mut c_void = core::ptr::null_mut();
pub static mut nextp: *mut c_void = core::ptr::null_mut();
    prstatc = css_rstat_cpu(parent, cpu);
    nextp = &prstatc.updated_children;
    while (*nextp != root) {
pub static mut nrstatc: *mut c_void = core::ptr::null_mut();
    nrstatc = css_rstat_cpu(*nextp, cpu);
    WARN_ON_ONCE!(*nextp == parent);
    nextp = &nrstatc.updated_next;
    }
// nextp = rstatc->updated_next;
    }
    rstatc.updated_next = core::ptr::null_mut();
// Push @root to the list first before pushing the children
    head = root;
    root.rstat_flush_next = core::ptr::null_mut();
    child = rstatc.updated_children;
    rstatc.updated_children = root;
    if (child != root) {
    head = css_rstat_push_children(head, child, cpu);
    }
    return head;
    }
//
// A hook for bpf stat collectors to attach to and flush their stats.
// Together with providing bpf kfuncs for css_rstat_updated() and
// css_rstat_flush(), this enables a complete workflow where bpf progs that
// collect cgroup stats can integrate with rstat for efficient flushing.
//
// A static noinline declaration here could cause the compiler to optimize away
// the function. A global noinline declaration will keep the definition, but may
// optimize away the callsite. Therefore, __weak is needed to ensure that the
// call is still emitted, by telling the compiler that we don't know what the
// function might eventually be.
//
    __bpf_hook_start();
    __weak noinline void bpf_rstat_flush(cgroup *cgrp, cgroup *parent, int cpu)
    {
    }
    __bpf_hook_end();
//
// Helper functions for locking.
//
// This makes it easier to diagnose locking issues and contention in
// production environments.  The parameter @cpu_in_loop indicate lock
// was released and re-taken when collection data from the CPUs. The
// value -1 is used when obtaining the main lock else this is the CPU
// number processed last.
//
#[no_mangle]
pub unsafe extern "C" fn __css_rstat_lock(css: *mut cgroup_subsys_state) {
    let mut cgrp = css.cgroup;
pub static mut lock: *mut c_void = core::ptr::null_mut();
    let mut contended = 0;
    lock = ss_rstat_lock(css.ss);
    contended = !spin_trylock_irq(lock);
    if (contended) {
    trace_cgroup_rstat_lock_contended(cgrp, cpu_in_loop, contended);
    spin_lock_irq(lock);
    }
    trace_cgroup_rstat_locked(cgrp, cpu_in_loop, contended);
    }
#[no_mangle]
pub unsafe extern "C" fn __css_rstat_unlock(css: *mut cgroup_subsys_state) {
    let mut cgrp = css.cgroup;
pub static mut lock: *mut c_void = core::ptr::null_mut();
    lock = ss_rstat_lock(css.ss);
    trace_cgroup_rstat_unlock(cgrp, cpu_in_loop, false);
    spin_unlock_irq(lock);
    }
//
// css_rstat_flush - flush stats in @css's rstat subtree
// @css: target cgroup subsystem state
//
// Collect all per-cpu stats in @css's subtree into the global counters
// and propagate them upwards. After this function returns, all rstat
// nodes in the subtree have up-to-date ->stat.
//
// This also gets all rstat nodes in the subtree including @css off the
// ->updated_children lists.
//
// This function may block.
//
#[no_mangle]
pub unsafe extern "C" fn css_rstat_flush(css: *mut cgroup_subsys_state) -> __bpf_kfunc void {
    let mut cpu = 0;
pub static mut is_self: bool = false;
//
// Since bpf programs can call this function, prevent access to
// uninitialized rstat pointers.
//
    if (!css_uses_rstat(css)) {
    return;
    }
    might_sleep();
    for_each_possible_cpu(cpu) {
pub static mut pos: *mut c_void = core::ptr::null_mut();
// Reacquire for each CPU to avoid disabling IRQs too long
    __css_rstat_lock(css, cpu);
    pos = css_rstat_updated_list(css, cpu);
    while (pos) {
    if (is_self) {
    cgroup_base_stat_flush(pos.cgroup, cpu);
    bpf_rstat_flush(pos.cgroup,
    cgroup_parent(pos.cgroup), cpu);
    } else {
    pos.ss.css_rstat_flush(pos, cpu);
    }
    }
    __css_rstat_unlock(css, cpu);
    if (!cond_resched()) {
    cpu_relax();
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn css_rstat_init(css: *mut cgroup_subsys_state) -> c_int {
    let mut cgrp = css.cgroup;
    let mut cpu = 0;
pub static mut is_self: bool = false;
    if (is_self) {
// the root cgrp has rstat_base_cpu preallocated
    if (!cgrp.rstat_base_cpu) {
    cgrp.rstat_base_cpu = alloc_percpu(cgroup_rstat_base_cpu);
    if (!cgrp.rstat_base_cpu) {
    return -ENOMEM;
    }
    }
    } else if (css.ss.css_rstat_flush == core::ptr::null_mut()) {
    return 0;
    }
// the root cgrp's self css has rstat_cpu preallocated
    if (!css.rstat_cpu) {
    css.rstat_cpu = alloc_percpu(css_rstat_cpu);
    if (!css.rstat_cpu) {
    if (is_self) {
    free_percpu(cgrp.rstat_base_cpu);
    }
    return -ENOMEM;
    }
    }
// ->updated_children list is self terminated
    for_each_possible_cpu(cpu) {
    let mut rstatc = css_rstat_cpu(css, cpu);
    rstatc.owner = rstatc.updated_children = css;
    init_llist_node(&rstatc.lnode);
    if (is_self) {
pub static mut rstatbc: *mut c_void = core::ptr::null_mut();
    rstatbc = cgroup_rstat_base_cpu(cgrp, cpu);
// forward_decl: _stats_init;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn css_rstat_exit(css: *mut cgroup_subsys_state) {
    let mut cpu = 0;
    if (!css_uses_rstat(css)) {
    return;
    }
    if (!css.rstat_cpu) {
    return;
    }
    css_rstat_flush(css);
// sanity check
    for_each_possible_cpu(cpu) {
    let mut rstatc = css_rstat_cpu(css, cpu);
    if (WARN_ON_ONCE!(rstatc.updated_children != css) ||
    WARN_ON_ONCE!(rstatc.updated_next)) {
    return;
    }
    }
    if (css_is_self(css)) {
    let mut cgrp = css.cgroup;
    free_percpu(cgrp.rstat_base_cpu);
    cgrp.rstat_base_cpu = core::ptr::null_mut();
    }
    free_percpu(css.rstat_cpu);
    css.rstat_cpu = core::ptr::null_mut();
    }
//
// ss_rstat_init - subsystem-specific rstat initialization
// @ss: target subsystem
//
// If @ss is NULL, the static locks associated with the base stats
// are initialized. If @ss is non-NULL, the subsystem-specific locks
// are initialized.
//
#[no_mangle]
pub unsafe extern "C" fn ss_rstat_init(ss: *mut cgroup_subsys) -> c_int {
    let mut cpu = 0;
    if (ss) {
    ss.lhead = alloc_percpu(llist_head);
    if (!ss.lhead) {
    return -ENOMEM;
    }
    }
    spin_lock_init(ss_rstat_lock(ss));
    for_each_possible_cpu(cpu) {
    init_llist_head(ss_lhead_cpu(ss, cpu));
    }
    return 0;
    }
//
// Functions for cgroup basic resource statistics implemented on top of
// rstat.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_base_stat_add(dst_bstat: *mut cgroup_base_stat, src_bstat: *mut cgroup_base_stat) {
    dst_bstat.cputime.utime += src_bstat.cputime.utime;
    dst_bstat.cputime.stime += src_bstat.cputime.stime;
    dst_bstat.cputime.sum_exec_runtime += src_bstat.cputime.sum_exec_runtime;

    dst_bstat.forceidle_sum += src_bstat.forceidle_sum;

    dst_bstat.ntime += src_bstat.ntime;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_base_stat_sub(dst_bstat: *mut cgroup_base_stat, src_bstat: *mut cgroup_base_stat) {
    dst_bstat.cputime.utime -= src_bstat.cputime.utime;
    dst_bstat.cputime.stime -= src_bstat.cputime.stime;
    dst_bstat.cputime.sum_exec_runtime -= src_bstat.cputime.sum_exec_runtime;

    dst_bstat.forceidle_sum -= src_bstat.forceidle_sum;

    dst_bstat.ntime -= src_bstat.ntime;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_base_stat_flush(cgrp: *mut cgroup, cpu: c_int) {
    let mut rstatbc = cgroup_rstat_base_cpu(cgrp, cpu);
    let mut parent = cgroup_parent(cgrp);
pub static mut prstatbc: *mut c_void = core::ptr::null_mut();
pub static mut delta: usize = 0;
    let mut seq: c_uint = 0;
// Root-level stats are sourced from system-wide CPU stats
    if (!parent) {
    return;
    }
// fetch the current per-cpu values
    do {
    seq = __u64_stats_fetch_begin(&rstatbc.bsync);
    delta = rstatbc.bstat;
    } while (__u64_stats_fetch_retry(&rstatbc.bsync, seq));
// propagate per-cpu delta to cgroup and per-cpu global statistics
    cgroup_base_stat_sub(&delta, &rstatbc.last_bstat);
    cgroup_base_stat_add(&cgrp.bstat, &delta);
    cgroup_base_stat_add(&rstatbc.last_bstat, &delta);
    cgroup_base_stat_add(&rstatbc.subtree_bstat, &delta);
// propagate cgroup and per-cpu global delta to parent (unless that's root)
    if (cgroup_parent(parent)) {
    delta = cgrp.bstat;
    cgroup_base_stat_sub(&delta, &cgrp.last_bstat);
    cgroup_base_stat_add(&parent.bstat, &delta);
    cgroup_base_stat_add(&cgrp.last_bstat, &delta);
    delta = rstatbc.subtree_bstat;
    prstatbc = cgroup_rstat_base_cpu(parent, cpu);
    cgroup_base_stat_sub(&delta, &rstatbc.last_subtree_bstat);
    cgroup_base_stat_add(&prstatbc.subtree_bstat, &delta);
    cgroup_base_stat_add(&rstatbc.last_subtree_bstat, &delta);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_base_stat_cputime_account_begin(cgrp: *mut cgroup, flags: *mut c_ulong) -> *mut c_void {
pub static mut rstatbc: *mut c_void = core::ptr::null_mut();
    rstatbc = get_cpu_ptr(cgrp.rstat_base_cpu);
// flags = u64_stats_update_begin_irqsave(&rstatbc->bsync);
    return rstatbc;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_base_stat_cputime_account_end(cgrp: *mut cgroup, rstatbc: *mut cgroup_rstat_base_cpu, flags: c_ulong) {
// forward_decl: _stats_update_end_irqrestore;
    __css_rstat_updated(&cgrp.self, smp_processor_id());
    put_cpu_ptr(rstatbc);
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_account_cputime(cgrp: *mut cgroup, delta_exec: u64) {
pub static mut rstatbc: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    rstatbc = cgroup_base_stat_cputime_account_begin(cgrp, &flags);
    rstatbc.bstat.cputime.sum_exec_runtime += delta_exec;
    cgroup_base_stat_cputime_account_end(cgrp, rstatbc, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_account_cputime_field(cgrp: *mut cgroup, index: cpu_usage_stat, delta_exec: u64) {
pub static mut rstatbc: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    rstatbc = cgroup_base_stat_cputime_account_begin(cgrp, &flags);
    match (index) {
    CPUTIME_NICE => {
    rstatbc.bstat.ntime += delta_exec;
    fallthrough;
    }
    CPUTIME_USER => {
    rstatbc.bstat.cputime.utime += delta_exec;
    // break;
    }
    CPUTIME_SYSTEM => {
    }
    CPUTIME_IRQ => {
    }
    CPUTIME_SOFTIRQ => {
    rstatbc.bstat.cputime.stime += delta_exec;
    // break;

    }
    CPUTIME_FORCEIDLE => {
    rstatbc.bstat.forceidle_sum += delta_exec;
    // break;

    }
    _ => {
    // break;
    }
    }
    cgroup_base_stat_cputime_account_end(cgrp, rstatbc, flags);
    }
//
// compute the cputime for the root cgroup by getting the per cpu data
// at a global level, then categorizing the fields in a manner consistent
// with how it is done by __cgroup_account_cputime_field for each bit of
// cpu time attributed to a cgroup.
//
#[no_mangle]
unsafe extern "C" fn root_cgroup_cputime(bstat: *mut cgroup_base_stat) {
    let mut cputime = &bstat.cputime;
    let mut i = 0;
    memset(bstat, 0, sizeof!(*bstat));
    for_each_possible_cpu(i) {
pub static mut kcpustat: usize = 0;
    let mut cpustat = kcpustat.cpustat;
pub static mut user: u64 = 0;
pub static mut sys: u64 = 0;
    kcpustat_cpu_fetch(&kcpustat, i);
    user += cpustat[CPUTIME_USER];
    user += cpustat[CPUTIME_NICE];
    cputime.utime += user;
    sys += cpustat[CPUTIME_SYSTEM];
    sys += cpustat[CPUTIME_IRQ];
    sys += cpustat[CPUTIME_SOFTIRQ];
    cputime.stime += sys;
    cputime.sum_exec_runtime += user;
    cputime.sum_exec_runtime += sys;

    bstat.forceidle_sum += cpustat[CPUTIME_FORCEIDLE];

    bstat.ntime += cpustat[CPUTIME_NICE];
    }
    }
#[no_mangle]
unsafe extern "C" fn cgroup_force_idle_show(seq: *mut seq_file, bstat: *mut cgroup_base_stat) {

pub static mut forceidle_time: u64 = 0;
    do_div(forceidle_time, NSEC_PER_USEC);
    seq_printf(seq, "core_sched.force_idle_usec %llu\n", forceidle_time);

    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_base_stat_cputime_show(seq: *mut seq_file) {
    let mut cgrp = seq_css(seq).cgroup;
pub static mut bstat: usize = 0;
    if (cgroup_parent(cgrp)) {
    css_rstat_flush(&cgrp.self);
    __css_rstat_lock(&cgrp.self, -1);
    bstat = cgrp.bstat;
    cputime_adjust(&cgrp.bstat.cputime, &cgrp.prev_cputime,
    &bstat.cputime.utime, &bstat.cputime.stime);
    __css_rstat_unlock(&cgrp.self, -1);
    } else {
    root_cgroup_cputime(&bstat);
    }
    do_div(bstat.cputime.sum_exec_runtime, NSEC_PER_USEC);
    do_div(bstat.cputime.utime, NSEC_PER_USEC);
    do_div(bstat.cputime.stime, NSEC_PER_USEC);
    do_div(bstat.ntime, NSEC_PER_USEC);
    seq_printf(seq, "usage_usec %llu\n"
    "user_usec %llu\n"
    "system_usec %llu\n"
    "nice_usec %llu\n",
    bstat.cputime.sum_exec_runtime,
    bstat.cputime.utime,
    bstat.cputime.stime,
    bstat.ntime);
    cgroup_force_idle_show(seq, &bstat);
    }
// Add bpf kfuncs for css_rstat_updated() and css_rstat_flush()
    BTF_KFUNCS_START(bpf_rstat_kfunc_ids)
    BTF_ID_FLAGS(func, css_rstat_updated)
    BTF_ID_FLAGS(func, css_rstat_flush, KF_SLEEPABLE)
    BTF_KFUNCS_END(bpf_rstat_kfunc_ids)
pub static mut btf_kfunc_id_set: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_rstat_kfunc_init() -> c_int {
    return register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING,
    &bpf_rstat_kfunc_set);
    }
    late_initcall!(bpf_rstat_kfunc_init);