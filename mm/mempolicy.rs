//! Automatically rewritten from C to Rust
//! Source: mm/mempolicy.c
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
//
// Simple NUMA memory policy for the Linux kernel.
//
// Copyright 2003,2004 Andi Kleen, SuSE Labs.
// (C) Copyright 2005 Christoph Lameter, Silicon Graphics, Inc.
//
// NUMA policy allows the user to give hints in which node(s) memory should
// be allocated.
//
// Support six policies per VMA and per process:
//
// The VMA policy has priority over the process policy for a page fault.
//
// interleave     Allocate memory interleaved over a set of nodes,
// with normal fallback if it fails.
// For VMA based allocations this interleaves based on the
// offset into the backing object or offset into the mapping
// for anonymous memory. For process policy an process counter
// is used.
//
// weighted interleave
// Allocate memory interleaved over a set of nodes based on
// a set of weights (per-node), with normal fallback if it
// fails.  Otherwise operates the same as interleave.
// Example: nodeset(0,1) & weights (2,1) - 2 pages allocated
// on node 0 for every 1 page allocated on node 1.
//
// bind           Only allocate memory on a specific set of nodes,
// no fallback.
// FIXME: memory is allocated starting with the first node
// to the last. It would be better if bind would truly restrict
// the allocation to memory nodes instead
//
// preferred      Try a specific node first before normal fallback.
// As a special case NUMA_NO_NODE here means do the allocation
// on the local CPU. This is normally identical to default,
// but useful to set in a VMA when you have a non default
// process policy.
//
// preferred many Try a set of nodes first before normal fallback. This is
// similar to preferred without the special case.
//
// default        Allocate on the local node first, or when on a VMA
// use the process policy. This is what Linux always did
// in a NUMA aware kernel and still does by, ahem, default.
//
// The process policy is applied for most non interrupt memory allocations
// in that process' context. Interrupts ignore the policies and always
// try to allocate on the local CPU. The VMA policy is only applied for memory
// allocations for a VMA in the VM.
//
// Currently there are a few corner cases in swapping where the policy
// is not applied, but the majority should be handled. When process policy
// is used it is not remembered over swap outs/swap ins.
//
// Only the highest zone in the zone hierarchy gets policied. Allocations
// requesting a lower zone just use default policy. This implies that
// on systems with highmem kernel lowmem allocation don't get policied.
// Same with GFP_DMA allocations.
//
// For shmem/tmpfs shared memory the policy is shared between
// all users and remembered even when nobody has memory mapped.
//
// Notebook:
    fix mmap readahead to honour policy and enable policy for any page cache
    object
    statistics for bigpages
    global policy for page cache? currently it uses process policy. Requires
    first item above.
#[no_mangle]
pub unsafe extern "C" fn memory(policy: currently ignored for the) -> handle mremap for shared {
    handle mremap for shared memory (currently ignored for the policy)
    grows down?
    make bind policy root only? It can trigger oom much faster and the
    kernel is not always grateful with that.
//

// Internal flags

pub static mut policy_cache: *mut c_void = core::ptr::null_mut();
pub static mut sn_cache: *mut c_void = core::ptr::null_mut();
// Highest zone. An specific allocation for a zone below that is not
    policied. */
pub static mut policy_zone: zone_type = 0;
//
// run-time system-wide default policy => local allocation
//
pub static mut mempolicy: usize = 0;
    static struct mempolicy preferred_node_policy[MAX_NUMNODES];
//
// weightiness balances the tradeoff between small weights (cycles through nodes
// faster, more fair/even distribution) and large weights (smaller errors
// between actual bandwidth ratios and weight ratios). 32 is a number that has
// been found to perform at a reasonable compromise between the two goals.
//
pub static mut weightiness: int = 32;
//
// A null weighted_interleave_state is interpreted as having .mode="auto",
// and .iw_table is interpreted as an array of 1s with length nr_node_ids.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct weighted_interleave_state {
    pub mode_auto: bool,
    pub iw_table: [u8; 0],
}

    static struct weighted_interleave_state  *wi_state;
pub static mut node_bw_table: *mut c_void = core::ptr::null_mut();
//
// wi_state_lock protects both wi_state and node_bw_table.
// node_bw_table is only used by writers to update wi_state.
//
pub static mut wi_state_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn get_il_weight(node: c_int) -> u8 {
pub static mut state: *mut c_void = core::ptr::null_mut();
pub static mut weight: u8 = 1;
    rcu_read_lock();
    state = rcu_dereference(wi_state);
    if (state) {
    weight = state.iw_table[node];
    }
    rcu_read_unlock();
    return weight;
    }
//
// Convert bandwidth values into weighted interleave weights.
// Call with wi_state_lock.
//
#[no_mangle]
unsafe extern "C" fn reduce_interleave_weights(bw: *mut c_uint, new_iw: *mut u8) {
pub static mut sum_bw: u64 = 0;
    unsigned int cast_sum_bw, scaling_factor = 1, iw_gcd = 0;
    let mut nid = 0;
    for_each_node_state(nid, N_MEMORY) {
    sum_bw += bw[nid];
    }
// Scale bandwidths to whole numbers in the range [1, weightiness]
    for_each_node_state(nid, N_MEMORY) {
//
// Try not to perform 64-bit division.
// If sum_bw < scaling_factor, then sum_bw < U32_MAX.
// If sum_bw > scaling_factor, then round the weight up to 1.
//
    scaling_factor = weightiness * bw[nid];
    if (bw[nid] && sum_bw < scaling_factor) {
    cast_sum_bw = (unsigned int)sum_bw;
    new_iw[nid] = scaling_factor / cast_sum_bw;
    } else {
    new_iw[nid] = 1;
    }
    if (!iw_gcd) {
    iw_gcd = new_iw[nid];
    }
    iw_gcd = gcd(iw_gcd, new_iw[nid]);
    }
// 1:2 is strictly better than 16:32. Reduce by the weights' GCD.
    for_each_node_state(nid, N_MEMORY) {
    new_iw[nid] /= iw_gcd;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mempolicy_set_node_perf(node: c_uint, coords: *mut access_coordinate) -> c_int {
    struct weighted_interleave_state *new_wi_state, *old_wi_state = core::ptr::null_mut();
    unsigned int *old_bw, *new_bw;
    let mut bw_val = 0;
    let mut i = 0;
    bw_val = min(coords.read_bandwidth, coords.write_bandwidth);
    new_bw = kcalloc(nr_node_ids, sizeof!(unsigned int), GFP_KERNEL);
    if (!new_bw) {
    return -ENOMEM;
    }
    new_wi_state = kmalloc_flex(*new_wi_state, iw_table, nr_node_ids);
    if (!new_wi_state) {
    kfree(new_bw);
    return -ENOMEM;
    }
    new_wi_state.mode_auto = true;
    for (i = 0; i < nr_node_ids; i++) {
    new_wi_state.iw_table[i] = 1;
    }
//
// Update bandwidth info, even in manual mode. That way, when switching
// to auto mode in the future, iw_table can be overwritten using
// accurate bw data.
//
    mutex_lock(&wi_state_lock);
    old_bw = node_bw_table;
    if (old_bw) {
    memcpy(new_bw, old_bw, nr_node_ids * sizeof!(*old_bw));
    }
    new_bw[node] = bw_val;
    node_bw_table = new_bw;
    old_wi_state = rcu_dereference_protected(wi_state,
    lockdep_is_held(&wi_state_lock));
    if (old_wi_state && !old_wi_state.mode_auto) {
// Manual mode; skip reducing weights and updating wi_state
    mutex_unlock(&wi_state_lock);
    kfree(new_wi_state);
// goto;
    }
// NULL wi_state assumes auto=true; reduce weights and update wi_state
    reduce_interleave_weights(new_bw, new_wi_state.iw_table);
    rcu_assign_pointer(wi_state, new_wi_state);
    mutex_unlock(&wi_state_lock);
    if (old_wi_state) {
    synchronize_rcu();
    kfree(old_wi_state);
    }
// label;
    kfree(old_bw);
    return 0;
    }
//
// numa_nearest_node - Find nearest node by state
// @node: Node id to start the search
// @state: State to filter the search
//
// Lookup the closest node by distance if @nid is not in state.
//
// Return: this @node if it is in state, otherwise the closest node by distance
//
#[no_mangle]
pub unsafe extern "C" fn numa_nearest_node(node: c_int, state: c_uint) -> c_int {
pub static mut min_dist: c_int = 0;
    if (state >= NR_NODE_STATES) {
    return -EINVAL;
    }
    if (node == NUMA_NO_NODE || node_state(node, state)) {
    return node;
    }
    min_node = node;
    for_each_node_state(n, state) {
    dist = node_distance(node, n);
    if (dist < min_dist) {
    min_dist = dist;
    min_node = n;
    }
    }
    return min_node;
    }
    EXPORT_SYMBOL_GPL(numa_nearest_node);
//
// nearest_node_nodemask - Find the node in @mask at the nearest distance
// from @node.
//
// @node: a valid node ID to start the search from.
// @mask: a pointer to a nodemask representing the allowed nodes.
//
// This function iterates over all nodes in @mask and calculates the
// distance from the starting @node, then it returns the node ID that is
// the closest to @node, or MAX_NUMNODES if no node is found.
//
// Note that @node must be a valid node ID usable with node_distance(),
// providing an invalid node ID (e.g., NUMA_NO_NODE) may result in crashes
// or unexpected behavior.
//
#[no_mangle]
pub unsafe extern "C" fn nearest_node_nodemask(node: c_int, mask: *mut nodemask_t) -> c_int {
    int dist, n, min_dist = INT_MAX, min_node = MAX_NUMNODES;
    for_each_node_mask(n, *mask) {
    dist = node_distance(node, n);
    if (dist < min_dist) {
    min_dist = dist;
    min_node = n;
    }
    }
    return min_node;
    }
    EXPORT_SYMBOL_GPL(nearest_node_nodemask);
#[no_mangle]
pub unsafe extern "C" fn get_task_policy(p: *mut task_struct) -> *mut c_void {
    let mut pol = p.mempolicy;
    let mut node = 0;
    if (pol) {
    return pol;
    }
    node = numa_node_id();
    if (node != NUMA_NO_NODE) {
    pol = &preferred_node_policy[node];
// preferred_node_policy is not initialised early in boot
    if (pol.mode) {
    return pol;
    }
    }
    return &default_policy;
    }
    EXPORT_SYMBOL_FOR_MODULES(get_task_policy, "kvm");
    static const struct mempolicy_operations {
    int (*create)(mempolicy *pol, const nodemask_t *nodes);
    void (*rebind)(mempolicy *pol, const nodemask_t *nodes);
    } mpol_ops[MPOL_MAX];
#[no_mangle]
pub unsafe extern "C" fn mpol_store_user_nodemask(pol: *const mempolicy) -> c_int {
    return pol.flags & MPOL_USER_NODEMASK_FLAGS;
    }
#[no_mangle]
pub unsafe extern "C" fn mpol_relative_nodemask(ret: *mut nodemask_t, orig: *mut nodemask_t, rel: *mut nodemask_t) {
    let mut tmp;
    nodes_fold(tmp, *orig, nodes_weight(*rel));
    nodes_onto(*ret, tmp, *rel);
    }
#[no_mangle]
unsafe extern "C" fn mpol_new_nodemask(pol: *mut mempolicy, nodes: *const nodemask_t) -> c_int {
    if (nodes_empty(*nodes)) {
    return -EINVAL;
    }
    pol.nodes = *nodes;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpol_new_preferred(pol: *mut mempolicy, nodes: *const nodemask_t) -> c_int {
    if (nodes_empty(*nodes)) {
    return -EINVAL;
    }
    nodes_clear(pol.nodes);
    node_set(first_node(*nodes), pol.nodes);
    return 0;
    }
//
// mpol_set_nodemask is called after mpol_new() to set up the nodemask, if
// any, for the new policy.  mpol_new() has already validated the nodes
// parameter with respect to the policy mode and flags.
//
// Must be called holding task's alloc_lock to protect task's mems_allowed
// and mempolicy.  May also be called holding the mmap_lock for write.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_set_nodemask(pol: *mut mempolicy, nodes: *mut nodemask_t, nsc: *mut nodemask_scratch) -> c_int {
    let mut ret = 0;
//
// Default (pol==NULL) resp. local memory policies are not a
// subject of any remapping. They also do not need any special
// constructor.
//
    if (!pol || pol.mode == MPOL_LOCAL) {
    return 0;
    }
// Check N_MEMORY
    nodes_and(nsc.mask1,
    cpuset_current_mems_allowed, node_states[N_MEMORY]);
    VM_BUG_ON(!nodes);
    if (pol.flags & MPOL_F_RELATIVE_NODES) {
    mpol_relative_nodemask(&nsc.mask2, nodes, &nsc.mask1);
    }
    else {
    nodes_and(nsc.mask2, *nodes, nsc.mask1);
    }
    if (mpol_store_user_nodemask(pol)) {
    pol.w.user_nodemask = *nodes;
    }
    else {
    pol.w.cpuset_mems_allowed = cpuset_current_mems_allowed;
    }
    ret = mpol_ops[pol.mode].create(pol, &nsc.mask2);
    return ret;
    }
//
// This function just creates a new policy, does some check and simple
// initialization. You must invoke mpol_set_nodemask() to set nodes.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_new(mode: c_ushort, flags: c_ushort, nodes: *mut nodemask_t) -> *mut c_void {
pub static mut policy: *mut c_void = core::ptr::null_mut();
    if (mode == MPOL_DEFAULT) {
    if (nodes && !nodes_empty(*nodes)) {
    return ERR_PTR(-EINVAL);
    }
    return core::ptr::null_mut();
    }
    VM_BUG_ON(!nodes);
//
// MPOL_PREFERRED cannot be used with MPOL_F_STATIC_NODES or
// MPOL_F_RELATIVE_NODES if the nodemask is empty (local allocation).
// All other modes require a valid pointer to a non-empty nodemask.
//
    if (mode == MPOL_PREFERRED) {
    if (nodes_empty(*nodes)) {
    if (((flags & MPOL_F_STATIC_NODES) ||
    (flags & MPOL_F_RELATIVE_NODES))) {
    return ERR_PTR(-EINVAL);
    }
    mode = MPOL_LOCAL;
    }
    } else if (mode == MPOL_LOCAL) {
    if (!nodes_empty(*nodes) ||
    (flags & MPOL_F_STATIC_NODES) ||
    (flags & MPOL_F_RELATIVE_NODES)) {
    return ERR_PTR(-EINVAL);
    }
    } else if (nodes_empty(*nodes)) {
    return ERR_PTR(-EINVAL);
    }
    policy = kmem_cache_alloc(policy_cache, GFP_KERNEL);
    if (!policy) {
    return ERR_PTR(-ENOMEM);
    }
    atomic_set(&policy.refcnt, 1);
    policy.mode = mode;
    policy.flags = flags;
    policy.home_node = NUMA_NO_NODE;
    return policy;
    }
// Slow path of a mpol destructor.
#[no_mangle]
pub unsafe extern "C" fn __mpol_put(pol: *mut mempolicy) {
    if (!atomic_dec_and_test(&pol.refcnt)) {
    return;
    }
//
// Required to allow mmap_lock_speculative*() access, see for example
// futex_key_to_node_opt(). All accesses are serialized by mmap_lock,
// however the speculative lock section unbound by the normal lock
// boundaries, requiring RCU freeing.
//
    kfree_rcu(pol, rcu);
    }
    EXPORT_SYMBOL_FOR_MODULES(__mpol_put, "kvm");
#[no_mangle]
unsafe extern "C" fn mpol_rebind_default(pol: *mut mempolicy, nodes: *const nodemask_t) {
    }
#[no_mangle]
unsafe extern "C" fn mpol_rebind_nodemask(pol: *mut mempolicy, nodes: *const nodemask_t) {
    let mut tmp;
    if (pol.flags & MPOL_F_STATIC_NODES) {
    nodes_and(tmp, pol.w.user_nodemask, *nodes);
    }

    else if (pol.flags & MPOL_F_RELATIVE_NODES) {
    mpol_relative_nodemask(&tmp, &pol.w.user_nodemask, nodes);
    }
    else {
    nodes_remap(tmp, pol.nodes, pol.w.cpuset_mems_allowed,
// nodes);
    pol.w.cpuset_mems_allowed = *nodes;
    }
    if (nodes_empty(tmp)) {
    tmp = *nodes;
    }
    pol.nodes = tmp;
    }
#[no_mangle]
pub unsafe extern "C" fn mpol_rebind_preferred(pol: *mut mempolicy, nodes: *mut nodemask_t) {
    pol.w.cpuset_mems_allowed = *nodes;
    }
//
// mpol_rebind_policy - Migrate a policy to a different set of nodes
//
// Per-vma policies are protected by mmap_lock. Allocations using per-task
// policies are protected by task->mems_allowed_seq to prevent a premature
// OOM/allocation failure due to parallel nodemask modification.
//
#[no_mangle]
unsafe extern "C" fn mpol_rebind_policy(pol: *mut mempolicy, newmask: *const nodemask_t) {
    if (!pol || pol.mode == MPOL_LOCAL) {
    return;
    }
    if (!mpol_store_user_nodemask(pol) &&
    nodes_equal(pol.w.cpuset_mems_allowed, *newmask)) {
    return;
    }
    mpol_ops[pol.mode].rebind(pol, newmask);
    }
//
// Wrapper for mpol_rebind_policy() that just requires task
// pointer, and updates task mempolicy.
//
// Called with task's alloc_lock held.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_rebind_task(tsk: *mut task_struct, new: *const nodemask_t) {
    mpol_rebind_policy(tsk.mempolicy, new);
    }
//
// Rebind each vma in mm to new nodemask.
//
// Call holding a reference to mm.  Takes mm->mmap_lock during call.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_rebind_mm(mm: *mut mm_struct, new: *mut nodemask_t) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    VMA_ITERATOR(vmi, mm, 0);
    mmap_write_lock(mm);
    for_each_vma(vmi, vma) {
    vma_start_write(vma);
    mpol_rebind_policy(vma.vm_policy, new);
    }
    mmap_write_unlock(mm);
    }
pub static mut mempolicy_operations: usize = 0;
// forward_decl: migrate_folio_add;
    static nodemask_t *policy_nodemask(gfp_t gfp, mempolicy *pol,
    pgoff_t ilx, int *nid);
#[no_mangle]
unsafe extern "C" fn strictly_unmovable(flags: c_ulong) -> bool {
//
// STRICT without MOVE flags lets do_mbind() fail immediately with -EIO
// if any misplaced page is found.
//
    return (flags & (MPOL_MF_STRICT | MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) ==
    MPOL_MF_STRICT;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct migration_mpol {
    pub pol: *mut mempolicy,
    pub ilx: pgoff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_pages {
    pub pagelist: *mut list_head,
    pub flags: c_ulong,
    pub nmask: *mut nodemask_t,
    pub start: c_ulong,
    pub end: c_ulong,
    pub first: *mut vm_area_struct,
//     pub /: *mut *mut *mut folio large; / note last large folio encountered,
//     pub /: *mut *mut long nr_failed; / could not be isolated at this time,
}

//
// Check if the folio's nid is in qp->nmask.
//
// If MPOL_MF_INVERT is set in qp->flags, check if the nid is
// in the invert of qp->nmask.
//
#[no_mangle]
pub unsafe extern "C" fn queue_folio_required(folio: *mut folio, qp: *mut queue_pages) -> bool {
pub static mut nid: c_int = 0;
pub static mut flags: c_ulong = 0;
    return node_isset(nid, *qp.nmask) == !(flags & MPOL_MF_INVERT);
    }
#[no_mangle]
unsafe extern "C" fn queue_folios_pmd(pmd: *mut pmd_t, walk: *mut mm_walk) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut qp = walk.private;
pub static mut pmdval: pmd_t = 0;
    if (unlikely(!pmd_present(pmdval))) {
    if (pmd_is_migration_entry(pmdval)) {
    qp.nr_failed += 1;
    }
    return;
    }
    folio = pmd_folio(pmdval);
    if (is_huge_zero_folio(folio)) {
    walk.action = ACTION_CONTINUE;
    return;
    }
    if (!queue_folio_required(folio, qp)) {
    return;
    }
    if (!(qp.flags & (MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) ||
    !vma_migratable(walk.vma) ||
    !migrate_folio_add(folio, qp.pagelist, qp.flags)) {
    qp.nr_failed += 1;
    }
    }
//
// Scan through folios, checking if they satisfy the required conditions,
// moving them from LRU to local pagelist for migration if they do (or not).
//
// queue_folios_pte_range() has two possible return values:
// 0 - continue walking to scan for more, even if an existing folio on the
// wrong node could not be isolated and queued for migration.
// -EIO - only MPOL_MF_STRICT was specified, without MPOL_MF_MOVE or ..._ALL,
// and an existing folio was on a node that does not follow the policy.
//
#[no_mangle]
pub unsafe extern "C" fn queue_folios_pte_range(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut vma = walk.vma;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut qp = walk.private;
pub static mut flags: c_ulong = 0;
    let mut pte = core::ptr::null_mut();
    let mut mapped_pte = core::ptr::null_mut();
    let mut ptent;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut max_nr = 0;
    let mut nr = 0;
    ptl = pmd_trans_huge_lock(pmd, vma);
    if (ptl) {
    queue_folios_pmd(pmd, walk);
    spin_unlock(ptl);
// goto;
    }
    mapped_pte = pte = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (!pte) {
    walk.action = ACTION_AGAIN;
    return 0;
    }
    while (addr != end) {
    max_nr = (end - addr) >> PAGE_SHIFT;
    nr = 1;
    ptent = ptep_get(pte);
    if (pte_none(ptent)) {
    continue;
    }
    if (!pte_present(ptent)) {
pub static mut entry: softleaf_t = 0;
    if (softleaf_is_migration(entry)) {
    qp.nr_failed += 1;
    }
    continue;
    }
    folio = vm_normal_folio(vma, addr, ptent);
    if (!folio || folio_is_zone_device(folio)) {
    continue;
    }
    if (folio_test_large(folio) && max_nr != 1) {
    nr = folio_pte_batch(folio, pte, ptent, max_nr);
    }
//
// vm_normal_folio() filters out zero pages, but there might
// still be reserved folios to skip, perhaps in a VDSO.
//
    if (folio_test_reserved(folio)) {
    continue;
    }
    if (!queue_folio_required(folio, qp)) {
    continue;
    }
    if (folio_test_large(folio)) {
//
// A large folio can only be isolated from LRU once,
// but may be mapped by many PTEs (and Copy-On-Write may
// intersperse PTEs of other, order 0, folios).  This is
// a common case, so don't mistake it for failure (but
// there can be other cases of multi-mapped pages which
// this quick check does not help to filter out - and a
// search of the pagelist might grow to be prohibitive).
//
// migrate_pages(&pagelist) returns nr_failed folios, so
// check "large" now so that queue_pages_range() returns
// a comparable nr_failed folios.  This does imply that
// if folio could not be isolated for some racy reason
// at its first PTE, later PTEs will not give it another
// chance of isolation; but keeps the accounting simple.
//
    if (folio == qp.large) {
    continue;
    }
    qp.large = folio;
    }
    if (!(flags & (MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) ||
    !vma_migratable(vma) ||
    !migrate_folio_add(folio, qp.pagelist, flags)) {
    qp.nr_failed += nr;
    if (strictly_unmovable(flags)) {
    break;
    }
    }
    }
    pte_unmap_unlock(mapped_pte, ptl);
    cond_resched();
// label;
    if (qp.nr_failed && strictly_unmovable(flags)) {
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_folios_hugetlb(pte: *mut pte_t, hmask: c_ulong, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {

    let mut qp = walk.private;
pub static mut flags: c_ulong = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut ptep;
    ptl = huge_pte_lock(hstate_vma(walk.vma), walk.mm, pte);
    ptep = huge_ptep_get(walk.mm, addr, pte);
    if (!pte_present(ptep)) {
    if (!huge_pte_none(ptep)) {
pub static mut entry: softleaf_t = 0;
    if (unlikely(softleaf_is_migration(entry))) {
    qp.nr_failed += 1;
    }
    }
// goto;
    }
    folio = pfn_folio(pte_pfn(ptep));
    if (!queue_folio_required(folio, qp)) {
// goto;
    }
    if (!(flags & (MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) ||
    !vma_migratable(walk.vma)) {
    qp.nr_failed += 1;
// goto;
    }
//
// Unless MPOL_MF_MOVE_ALL, we try to avoid migrating a shared folio.
// Choosing not to migrate a shared folio is not counted as a failure.
//
// See folio_maybe_mapped_shared() on possible imprecision when we
// cannot easily detect if a folio is shared.
//
    if ((flags & MPOL_MF_MOVE_ALL) ||
    (!folio_maybe_mapped_shared(folio) && !hugetlb_pmd_shared(pte))) {
    if (!folio_isolate_hugetlb(folio, qp.pagelist))
    qp.nr_failed += 1;
    }
// label;
    spin_unlock(ptl);
    if (qp.nr_failed && strictly_unmovable(flags)) {
    return -EIO;
    }

    return 0;
    }

//
// folio_can_map_prot_numa() - check whether the folio can map prot numa
// @folio: The folio whose mapping considered for being made NUMA hintable
// @vma: The VMA that the folio belongs to.
// @is_private_single_threaded: Is this a single-threaded private VMA or not
//
// This function checks to see if the folio actually indicates that
// we need to make the mapping one which causes a NUMA hinting fault,
// as there are cases where it's simply unnecessary, and the folio's
// access time is adjusted for memory tiering if prot numa needed.
//
// Return: True if the mapping of the folio needs to be changed, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn folio_can_map_prot_numa(folio: *mut folio, vma: *mut vm_area_struct, is_private_single_threaded: bool) -> bool {
    let mut nid = 0;
    if (!folio || folio_is_zone_device(folio) || folio_test_ksm(folio)) {
    return false;
    }
// Also skip shared copy-on-write folios
    if (vma_is_cow_mapping(vma) && folio_maybe_mapped_shared(folio)) {
    return false;
    }
// Folios are pinned and can't be migrated
    if (folio_maybe_dma_pinned(folio)) {
    return false;
    }
//
// While migration can move some dirty folios,
// it cannot move them all from MIGRATE_ASYNC
// context.
//
    if (folio_is_file_lru(folio) && folio_test_dirty(folio)) {
    return false;
    }
//
// Don't mess with PTEs if folio is already on the node
// a single-threaded process is running on.
//
    nid = folio_nid(folio);
    if (is_private_single_threaded && (nid == numa_node_id())) {
    return false;
    }
//
// Skip scanning top tier node if normal numa
// balancing is disabled
//
    if (!(sysctl_numa_balancing_mode & NUMA_BALANCING_NORMAL) &&
    node_is_toptier(nid)) {
    return false;
    }
    if (folio_use_access_time(folio)) {
    folio_xchg_access_time(folio, jiffies_to_msecs(jiffies));
    }
    return true;
    }
//
// This is used to mark a range of virtual addresses to be inaccessible.
// These are later cleared by a NUMA hinting fault. Depending on these
// faults, pages may be migrated for better NUMA placement.
//
// This is assuming that NUMA faults are handled using PROT_NONE. If
// an architecture makes a different choice, it will need further
// changes to the core.
//
#[no_mangle]
pub unsafe extern "C" fn change_prot_numa(vma: *mut vm_area_struct, addr: c_ulong, end: c_ulong) -> c_ulong {
pub static mut tlb: usize = 0;
    let mut nr_updated = 0;
    tlb_gather_mmu(&tlb, vma.vm_mm);
    nr_updated = change_protection(&tlb, vma, addr, end, MM_CP_PROT_NUMA);
    if (nr_updated > 0) {
    count_vm_numa_events(NUMA_PTE_UPDATES, nr_updated);
    count_memcg_events_mm(vma.vm_mm, NUMA_PTE_UPDATES, nr_updated);
    }
    tlb_finish_mmu(&tlb);
    return nr_updated;
    }

#[no_mangle]
pub unsafe extern "C" fn queue_pages_test_walk(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    struct vm_area_struct *next, *vma = walk.vma;
    let mut qp = walk.private;
pub static mut flags: c_ulong = 0;
// range check first
    VM_BUG_ON_VMA(!range_in_vma(vma, start, end), vma);
    if (!qp.first) {
    qp.first = vma;
    if (!(flags & MPOL_MF_DISCONTIG_OK) &&
    (qp.start < vma.vm_start)) {
// hole at head side of range
    return -EFAULT;
    }
    }
    next = find_vma(vma.vm_mm, vma.vm_end);
    if (!(flags & MPOL_MF_DISCONTIG_OK) &&
    ((vma.vm_end < qp.end) &&
    (!next || vma.vm_end < next.vm_start))) {
// hole at middle or tail of range
    return -EFAULT;
    }
//
// Need check MPOL_MF_STRICT to return -EIO if possible
// regardless of vma_migratable
//
    if (!vma_migratable(vma) &&
    !(flags & MPOL_MF_STRICT)) {
    return 1;
    }
//
// Check page nodes, and queue pages to move, in the current vma.
// But if no moving, and no strict checking, the scan can be skipped.
//
    if (flags & (MPOL_MF_STRICT | MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) {
    return 0;
    }
    return 1;
    }
pub static mut mm_walk_ops: usize = 0;
pub static mut mm_walk_ops: usize = 0;
//
// Walk through page tables and collect pages to be migrated.
//
// If pages found in a given range are not on the required set of @nodes,
// and migration is allowed, they are isolated and queued to @pagelist.
//
// queue_pages_range() may return:
// 0 - all pages already on the right node, or successfully queued for moving
// (or neither strict checking nor moving requested: only range checking).
// >0 - this number of misplaced folios could not be queued for moving
// (a hugetlbfs page or a transparent huge page being counted as 1).
// -EIO - a misplaced page found, when MPOL_MF_STRICT specified without MOVEs.
// -EFAULT - a hole in the memory range, when MPOL_MF_DISCONTIG_OK unspecified.
//
#[no_mangle]
pub unsafe extern "C" fn queue_pages_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong, nodes: *mut nodemask_t, flags: c_ulong, pagelist: *mut list_head) -> c_long {
    let mut err = 0;
pub static mut queue_pages: usize = 0;
    let mut ops = (flags & MPOL_MF_WRLOCK) ?
    &queue_pages_lock_vma_walk_ops : &queue_pages_walk_ops;
    err = walk_page_range(mm, start, end, ops, &qp);
    if (!qp.first) {
// whole range in hole
    err = -EFAULT;
    }
    return err ? : qp.nr_failed;
    }
//
// Apply policy to a single VMA
// This must be called with the mmap_lock held for writing.
//
#[no_mangle]
pub unsafe extern "C" fn vma_replace_policy(vma: *mut vm_area_struct, pol: *mut mempolicy) -> c_int {
    let mut err = 0;
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    vma_assert_write_locked(vma);
    new = mpol_dup(pol);
    if (IS_ERR(new)) {
    return PTR_ERR(new);
    }
    if (vma.vm_ops && vma.vm_ops.set_policy) {
    err = vma.vm_ops.set_policy(vma, new);
    if (err) {
// goto;
    }
    }
    old = vma.vm_policy;
    WRITE_ONCE(vma.vm_policy, new); /* protected by mmap_lock */
    mpol_put(old);
    return 0;
// label;
    mpol_put(new);
    return err;
    }
// Split or merge the VMA (if required) and apply the new policy
#[no_mangle]
pub unsafe extern "C" fn mbind_range(vmi: *mut vma_iterator, vma: *mut vm_area_struct, prev: *mut *mut vm_area_struct, start: c_ulong, end: c_ulong, new_pol: *mut mempolicy) -> c_int {
    unsigned long vmstart, vmend;
    vmend = min(end, vma.vm_end);
    if (start > vma.vm_start) {
// prev = vma;
    vmstart = start;
    } else {
    vmstart = vma.vm_start;
    }
    if (mpol_equal(vma.vm_policy, new_pol)) {
// prev = vma;
    return 0;
    }
    vma =  vma_modify_policy(vmi, *prev, vma, vmstart, vmend, new_pol);
    if (IS_ERR(vma)) {
    return PTR_ERR(vma);
    }
// prev = vma;
    return vma_replace_policy(vma, new_pol);
    }
// Set the process memory policy
#[no_mangle]
pub unsafe extern "C" fn do_set_mempolicy(mode: c_ushort, flags: c_ushort, nodes: *mut nodemask_t) -> c_long {
    let mut new = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
    NODEMASK_SCRATCH(scratch);
    let mut ret = 0;
    if (!scratch) {
    return -ENOMEM;
    }
    new = mpol_new(mode, flags, nodes);
    if (IS_ERR(new)) {
    ret = PTR_ERR(new);
// goto;
    }
    task_lock(current);
    ret = mpol_set_nodemask(new, nodes, scratch);
    if (ret) {
    task_unlock(current);
    mpol_put(new);
// goto;
    }
    old = current.mempolicy;
    current.mempolicy = new;
    if (new && (new.mode == MPOL_INTERLEAVE ||
    new.mode == MPOL_WEIGHTED_INTERLEAVE)) {
    current.il_prev = MAX_NUMNODES-1;
    current.il_weight = 0;
    }
    task_unlock(current);
    mpol_put(old);
    ret = 0;
// label;
    NODEMASK_SCRATCH_FREE(scratch);
    return ret;
    }
//
// Return nodemask for policy for get_mempolicy() query
//
// Called with task's alloc_lock held
//
#[no_mangle]
unsafe extern "C" fn get_policy_nodemask(pol: *mut mempolicy, nodes: *mut nodemask_t) {
    nodes_clear(*nodes);
    if (pol == &default_policy) {
    return;
    }
    match (pol.mode) {
    MPOL_BIND => {
    }
    MPOL_INTERLEAVE => {
    }
    MPOL_PREFERRED => {
    }
    MPOL_PREFERRED_MANY => {
    }
    MPOL_WEIGHTED_INTERLEAVE => {
// nodes = pol->nodes;
    // break;
    }
    MPOL_LOCAL => {
// return empty node mask for local allocation
    // break;
    }
    _ => {
    BUG();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn lookup_node(mm: *mut mm_struct, addr: c_ulong) -> c_int {
    let mut p = core::ptr::null_mut();
    let mut ret = 0;
    ret = get_user_pages_fast(addr & PAGE_MASK, 1, 0, &p);
    if (ret > 0) {
    ret = page_to_nid(p);
    put_page(p);
    }
    return ret;
    }
// Retrieve NUMA policy
#[no_mangle]
pub unsafe extern "C" fn do_get_mempolicy(policy: *mut c_int, nmask: *mut nodemask_t, addr: c_ulong, flags: c_ulong) -> c_long {
    let mut err = 0;
    let mut mm = current.mm;
    let mut vma = core::ptr::null_mut();
    let mut pol = current.mempolicy, *pol_refcount = core::ptr::null_mut();
    if (flags &
    ~(unsigned long)(MPOL_F_NODE|MPOL_F_ADDR|MPOL_F_MEMS_ALLOWED)) {
    return -EINVAL;
    }
    if (flags & MPOL_F_MEMS_ALLOWED) {
    if (flags & (MPOL_F_NODE|MPOL_F_ADDR)) {
    return -EINVAL;
    }
// policy = 0;	// just so it's initialized
    task_lock(current);
// nmask  = cpuset_current_mems_allowed;
    task_unlock(current);
    return 0;
    }
    if (flags & MPOL_F_ADDR) {
    let mut ilx;		/* ignored here */
//
// Do NOT fall back to task policy if the
// vma/shared policy at addr is NULL.  We
// want to return MPOL_DEFAULT in this case.
//
    mmap_read_lock(mm);
    vma = vma_lookup(mm, addr);
    if (!vma) {
    mmap_read_unlock(mm);
    return -EFAULT;
    }
    pol = __get_vma_policy(vma, addr, &ilx);
    } else if (addr) {
    return -EINVAL;
    }
    if (!pol) {
    pol = &default_policy;	/* indicates default behavior */
    }
    if (flags & MPOL_F_NODE) {
    if (flags & MPOL_F_ADDR) {
//
// Take a refcount on the mpol, because we are about to
// drop the mmap_lock, after which only "pol" remains
// valid, "vma" is stale.
//
    pol_refcount = pol;
    vma = core::ptr::null_mut();
    mpol_get(pol);
    mmap_read_unlock(mm);
    err = lookup_node(mm, addr);
    if (err < 0) {
// goto;
    }
// policy = err;
    } else if (pol == current.mempolicy &&
    pol.mode == MPOL_INTERLEAVE) {
// policy = next_node_in(current->il_prev, pol->nodes);
    } else if (pol == current.mempolicy &&
    pol.mode == MPOL_WEIGHTED_INTERLEAVE) {
    if (current.il_weight) {
// policy = current->il_prev;
    }
    else {
// policy = next_node_in(current->il_prev,
    pol.nodes);
    }
    } else {
    err = -EINVAL;
// goto;
    }
    } else {
// policy = pol == &default_policy ? MPOL_DEFAULT :
    pol.mode;
//
// Internal mempolicy flags must be masked off before exposing
// the policy to userspace.
//
// policy |= (pol->flags & MPOL_MODE_FLAGS);
    }
    err = 0;
    if (nmask) {
    if (mpol_store_user_nodemask(pol)) {
// nmask = pol->w.user_nodemask;
    } else {
    task_lock(current);
    get_policy_nodemask(pol, nmask);
    task_unlock(current);
    }
    }
// label;
    mpol_cond_put(pol);
    if (vma) {
    mmap_read_unlock(mm);
    }
    if (pol_refcount) {
    mpol_put(pol_refcount);
    }
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn migrate_folio_add(folio: *mut folio, foliolist: *mut list_head, flags: c_ulong) -> bool {
//
// Unless MPOL_MF_MOVE_ALL, we try to avoid migrating a shared folio.
// Choosing not to migrate a shared folio is not counted as a failure.
//
// See folio_maybe_mapped_shared() on possible imprecision when we
// cannot easily detect if a folio is shared.
//
    if ((flags & MPOL_MF_MOVE_ALL) || !folio_maybe_mapped_shared(folio)) {
    if (folio_isolate_lru(folio)) {
    list_add_tail(&folio.lru, foliolist);
    node_stat_mod_folio(folio,
    NR_ISOLATED_ANON + folio_is_file_lru(folio),
    folio_nr_pages(folio));
    } else {
//
// Non-movable folio may reach here.  And, there may be
// temporary off LRU folios or non-LRU movable folios.
// Treat them as unmovable folios since they can't be
// isolated, so they can't be moved at the moment.
//
    return false;
    }
    }
    return true;
    }
//
// Migrate pages from one node to a target node.
// Returns error or the number of pages not migrated.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_to_node(mm: *mut mm_struct, source: c_int, dest: c_int, flags: c_int) -> c_long {
    let mut nmask;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut pagelist: usize = 0;
    let mut nr_failed = 0;
pub static mut err: c_long = 0;
pub static mut migration_target_control: usize = 0;
    nodes_clear(nmask);
    node_set(source, nmask);
    VM_BUG_ON(!(flags & (MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)));
    mmap_read_lock(mm);
    vma = find_vma(mm, 0);
    if (unlikely(!vma)) {
    mmap_read_unlock(mm);
    return 0;
    }
//
// This does not migrate the range, but isolates all pages that
// need migration.  Between passing in the full user address
// space range and MPOL_MF_DISCONTIG_OK, this call cannot fail,
// but passes back the count of pages which could not be isolated.
//
    nr_failed = queue_pages_range(mm, vma.vm_start, mm.task_size, &nmask,
    flags | MPOL_MF_DISCONTIG_OK, &pagelist);
    mmap_read_unlock(mm);
    if (!list_empty(&pagelist)) {
    err = migrate_pages(&pagelist, alloc_migration_target, core::ptr::null_mut(),
    (unsigned long)&mtc, MIGRATE_SYNC, MR_SYSCALL, core::ptr::null_mut());
    if (err) {
    putback_movable_pages(&pagelist);
    }
    }
    if (err >= 0) {
    err += nr_failed;
    }
    return err;
    }
//
// Move pages between the two nodesets so as to preserve the physical
// layout as much as possible.
//
// Returns the number of page that could not be moved.
//
#[no_mangle]
pub unsafe extern "C" fn do_migrate_pages(mm: *mut mm_struct, from: *mut nodemask_t, to: *mut nodemask_t, flags: c_int) -> c_int {
pub static mut nr_failed: c_long = 0;
pub static mut err: c_long = 0;
    let mut tmp;
    lru_cache_disable();
//
// Find a 'source' bit set in 'tmp' whose corresponding 'dest'
// bit in 'to' is not also set in 'tmp'.  Clear the found 'source'
// bit in 'tmp', and return that <source, dest> pair for migration.
// The pair of nodemasks 'to' and 'from' define the map.
//
// If no pair of bits is found that way, fallback to picking some
// pair of 'source' and 'dest' bits that are not the same.  If the
// 'source' and 'dest' bits are the same, this represents a node
// that will be migrating to itself, so no pages need move.
//
// If no bits are left in 'tmp', or if all remaining bits left
// in 'tmp' correspond to the same bit in 'to', return false
// (nothing left to migrate).
//
// This lets us pick a pair of nodes to migrate between, such that
// if possible the dest node is not already occupied by some other
// source node, minimizing the risk of overloading the memory on a
// node that would happen if we migrated incoming memory to a node
// before migrating outgoing memory source that same node.
//
// A single scan of tmp is sufficient.  As we go, we remember the
// most recent <s, d> pair that moved (s != d).  If we find a pair
// that not only moved, but what's better, moved to an empty slot
// (d is not set in tmp), then we break out then, with that pair.
// Otherwise when we finish scanning from_tmp, we at least have the
// most recent <s, d> pair that moved.  If we get all the way through
// the scan of tmp without finding any node that moved, much less
// moved to an empty node, then there is nothing left worth migrating.
//
    tmp = *from;
    while (!nodes_empty(tmp)) {
    let mut s = 0;
    let mut d = 0;
pub static mut source: c_int = 0;
pub static mut dest: c_int = 0;
    for_each_node_mask(s, tmp) {
//
// do_migrate_pages() tries to maintain the relative
// node relationship of the pages established between
// threads and memory areas.
//
// However if the number of source nodes is not equal to
// the number of destination nodes we can not preserve
// this node relative relationship.  In that case, skip
// copying memory from a node that is in the destination
// mask.
//
// Example: [2,3,4] -> [3,4,5] moves everything.
// [0-7] - > [3,4,5] moves only 0,1,2,6,7.
//
    if ((nodes_weight(*from) != nodes_weight(*to)) &&
    (node_isset(s, *to))) {
    continue;
    }
    d = node_remap(s, *from, *to);
    if (s == d) {
    continue;
    }
    source = s;	/* Node moved. Memorize */
    dest = d;
// dest not in remaining from nodes?
    if (!node_isset(dest, tmp)) {
    break;
    }
    }
    if (source == NUMA_NO_NODE) {
    break;
    }
    node_clear(source, tmp);
    err = migrate_to_node(mm, source, dest, flags);
    if (err > 0) {
    nr_failed += err;
    }
    if (err < 0) {
    break;
    }
    }
    lru_cache_enable();
    if (err < 0) {
    return err;
    }
    return (nr_failed < INT_MAX) ? nr_failed : INT_MAX;
    }
//
// Allocate a new folio for page migration, according to NUMA mempolicy.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_migration_target_by_mpol(src: *mut folio, private: c_ulong) -> *mut c_void {
    let mut mmpol = private;
    let mut pol = mmpol.pol;
pub static mut ilx: pgoff_t = 0;
    let mut order = 0;
pub static mut nid: c_int = 0;
    let mut gfp;
    order = folio_order(src);
    ilx += src.index >> order;
    if (folio_test_hugetlb(src)) {
pub static mut nodemask: *mut c_void = core::ptr::null_mut();
pub static mut h: *mut c_void = core::ptr::null_mut();
    h = folio_hstate(src);
    gfp = htlb_alloc_mask(h);
    nodemask = policy_nodemask(gfp, pol, ilx, &nid);
    return alloc_hugetlb_folio_nodemask(h, nid, nodemask, gfp,
    htlb_allow_alloc_fallback(MR_MEMPOLICY_MBIND));
    }
    if (folio_test_large(src)) {
    gfp = GFP_TRANSHUGE;
    }
    else {
    gfp = GFP_HIGHUSER_MOVABLE | __GFP_RETRY_MAYFAIL | __GFP_COMP;
    }
    return folio_alloc_mpol(gfp, order, pol, ilx, nid);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: migrate_folio_add
pub unsafe extern "C" fn migrate_folio_add_dup(folio: *mut folio, foliolist: *mut list_head, flags: c_ulong) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: do_migrate_pages
pub unsafe extern "C" fn do_migrate_pages_dup(mm: *mut mm_struct, from: *mut nodemask_t, to: *mut nodemask_t, flags: c_int) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_migration_target_by_mpol
pub unsafe extern "C" fn alloc_migration_target_by_mpol_dup(src: *mut folio, private: c_ulong) -> *mut c_void {
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn do_mbind(start: c_ulong, len: c_ulong, mode: c_ushort, mode_flags: c_ushort, nmask: *mut nodemask_t, flags: c_ulong) -> c_long {
    let mut mm = current.mm;
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
pub static mut vmi: usize = 0;
pub static mut mmpol: usize = 0;
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut end = 0;
    let mut err = 0;
    let mut nr_failed = 0;
pub static mut pagelist: usize = 0;
    if (flags & ~(unsigned long)MPOL_MF_VALID) {
    return -EINVAL;
    }
    if ((flags & MPOL_MF_MOVE_ALL) && !capable(CAP_SYS_NICE)) {
    return -EPERM;
    }
    if (start & ~PAGE_MASK) {
    return -EINVAL;
    }
    if (mode == MPOL_DEFAULT) {
    flags &= ~MPOL_MF_STRICT;
    }
    len = PAGE_ALIGN(len);
    end = start + len;
    if (end < start) {
    return -EINVAL;
    }
    if (end == start) {
    return 0;
    }
    new = mpol_new(mode, mode_flags, nmask);
    if (IS_ERR(new)) {
    return PTR_ERR(new);
    }
//
// If we are using the default policy then operation
// on discontinuous address spaces is okay after all
//
    if (!new) {
    flags |= MPOL_MF_DISCONTIG_OK;
    }
    if (flags & (MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) {
    lru_cache_disable();
    }
    {
    NODEMASK_SCRATCH(scratch);
    if (scratch) {
    mmap_write_lock(mm);
    err = mpol_set_nodemask(new, nmask, scratch);
    if (err) {
    mmap_write_unlock(mm);
    }
    } else {
    err = -ENOMEM;
    }
    NODEMASK_SCRATCH_FREE(scratch);
    }
    if (err) {
// goto;
    }
//
// Lock the VMAs before scanning for pages to migrate,
// to ensure we don't miss a concurrently inserted page.
//
    nr_failed = queue_pages_range(mm, start, end, nmask,
    flags | MPOL_MF_INVERT | MPOL_MF_WRLOCK, &pagelist);
    if (nr_failed < 0) {
    err = nr_failed;
    nr_failed = 0;
    } else {
    vma_iter_init(&vmi, mm, start);
    prev = vma_prev(&vmi);
    for_each_vma_range(vmi, vma, end) {
    err = mbind_range(&vmi, vma, &prev, start, end, new);
    if (err) {
    break;
    }
    }
    }
    if (!err && !list_empty(&pagelist)) {
// Convert MPOL_DEFAULT's NULL to task or default policy
    if (!new) {
    new = get_task_policy(current);
    mpol_get(new);
    }
    mmpol.pol = new;
    mmpol.ilx = 0;
//
// In the interleaved case, attempt to allocate on exactly the
// targeted nodes, for the first VMA to be migrated; for later
// VMAs, the nodes will still be interleaved from the targeted
// nodemask, but one by one may be selected differently.
//
    if (new.mode == MPOL_INTERLEAVE ||
    new.mode == MPOL_WEIGHTED_INTERLEAVE) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut order = 0;
pub static mut addr: c_ulong = 0;
    list_for_each_entry(folio, &pagelist, lru) {
    if (!folio_test_ksm(folio)) {
    break;
    }
    }
    if (!list_entry_is_head(folio, &pagelist, lru)) {
    vma_iter_init(&vmi, mm, start);
    for_each_vma_range(vmi, vma, end) {
    addr = page_address_in_vma(folio,
    folio_page(folio, 0), vma);
    if (addr != -EFAULT) {
    break;
    }
    }
    }
    if (addr != -EFAULT) {
    order = folio_order(folio);
// We already know the pol, but not the ilx
    mpol_cond_put(get_vma_policy(vma, addr, order,
    &mmpol.ilx));
// Set base from which to increment by index
    mmpol.ilx -= folio.index >> order;
    }
    }
    }
    mmap_write_unlock(mm);
    if (!err && !list_empty(&pagelist)) {
    nr_failed |= migrate_pages(&pagelist,
    alloc_migration_target_by_mpol, core::ptr::null_mut(),
    (unsigned long)&mmpol, MIGRATE_SYNC,
    MR_MEMPOLICY_MBIND, core::ptr::null_mut());
    }
    if (nr_failed && (flags & MPOL_MF_STRICT)) {
    err = -EIO;
    }
    if (!list_empty(&pagelist)) {
    putback_movable_pages(&pagelist);
    }
// label;
    mpol_put(new);
    if (flags & (MPOL_MF_MOVE | MPOL_MF_MOVE_ALL)) {
    lru_cache_enable();
    }
    return err;
    }
//
// User space interface with variable sized bitmaps for nodelists.
//
#[no_mangle]
pub unsafe extern "C" fn get_bitmap(mask: *mut c_ulong, nmask: *mut c_ulong, maxnode: c_ulong) -> c_int {
pub static mut nlongs: c_ulong = 0;
    let mut ret = 0;
    if (in_compat_syscall()) {
    ret = compat_get_bitmap(mask,
    nmask,
    maxnode);
    }
    else {
    ret = copy_from_user(mask, nmask,
    nlongs * sizeof!(unsigned long));
    }
    if (ret) {
    return -EFAULT;
    }
    if (maxnode % BITS_PER_LONG) {
    mask[nlongs - 1] &= (1UL << (maxnode % BITS_PER_LONG)) - 1;
    }
    return 0;
    }
// Copy a node mask from user space.
#[no_mangle]
pub unsafe extern "C" fn get_nodes(nodes: *mut nodemask_t, nmask: *mut c_ulong, maxnode: c_ulong) -> c_int {
    maxnode -= 1;
    nodes_clear(*nodes);
    if (maxnode == 0 || !nmask) {
    return 0;
    }
    if (maxnode > PAGE_SIZE*BITS_PER_BYTE) {
    return -EINVAL;
    }
//
// When the user specified more nodes than supported just check
// if the non supported part is all zero, one word at a time,
// starting at the end.
//
    while (maxnode > MAX_NUMNODES) {
pub static mut bits: c_ulong = 0;
    let mut t = 0;
    if (get_bitmap(&t, &nmask[(maxnode - 1) / BITS_PER_LONG], bits)) {
    return -EFAULT;
    }
    if (maxnode - bits >= MAX_NUMNODES) {
    maxnode -= bits;
    } else {
    maxnode = MAX_NUMNODES;
    t &= ~((1UL << (MAX_NUMNODES % BITS_PER_LONG)) - 1);
    }
    if (t) {
    return -EINVAL;
    }
    }
    return get_bitmap(nodes_addr(*nodes), nmask, maxnode);
    }
// Copy a kernel node mask to user space
#[no_mangle]
pub unsafe extern "C" fn copy_nodes_to_user(mask: *mut c_ulong, maxnode: c_ulong, nodes: *mut nodemask_t) -> c_int {
pub static mut copy: c_ulong = 0;
pub static mut nbytes: c_uint = 0;
pub static mut compat: bool = false;
    if (compat) {
    nbytes = BITS_TO_COMPAT_LONGS(nr_node_ids) * sizeof!(compat_long_t);
    }
    if (copy > nbytes) {
    if (copy > PAGE_SIZE) {
    return -EINVAL;
    }
    if (clear_user(mask + nbytes, copy - nbytes)) {
    return -EFAULT;
    }
    copy = nbytes;
    maxnode = nr_node_ids;
    }
    if (compat) {
    return compat_put_bitmap(mask,
    nodes_addr(*nodes), maxnode);
    }
    return copy_to_user(mask, nodes_addr(*nodes), copy) ? -EFAULT : 0;
    }
// Basic parameter sanity check used by both mbind() and set_mempolicy()
#[no_mangle]
pub unsafe extern "C" fn sanitize_mpol_flags(mode: *mut c_int, flags: *mut c_ushort) -> c_int {
// flags = *mode & MPOL_MODE_FLAGS;
// mode &= ~MPOL_MODE_FLAGS;
    if ((unsigned int)(*mode) >=  MPOL_MAX) {
    return -EINVAL;
    }
    if ((*flags & MPOL_F_STATIC_NODES) && (*flags & MPOL_F_RELATIVE_NODES)) {
    return -EINVAL;
    }
    if (*flags & MPOL_F_NUMA_BALANCING) {
    if (*mode == MPOL_BIND || *mode == MPOL_PREFERRED_MANY) {
// flags |= (MPOL_F_MOF | MPOL_F_MORON);
    }
    else {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_mbind(start: c_ulong, len: c_ulong, mode: c_ulong, nmask: *mut c_ulong, maxnode: c_ulong, flags: c_uint) -> c_long {
    let mut mode_flags = 0;
    let mut nodes;
pub static mut lmode: c_int = 0;
    let mut err = 0;
    start = untagged_addr(start);
    err = sanitize_mpol_flags(&lmode, &mode_flags);
    if (err) {
    return err;
    }
    err = get_nodes(&nodes, nmask, maxnode);
    if (err) {
    return err;
    }
    return do_mbind(start, len, lmode, mode_flags, &nodes, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_set_mempolicy_home_node(start: usize, len: usize, home_node: usize, flags: usize) -> c_long {
    let mut mm = current.mm;
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
    let mut end = 0;
pub static mut err: c_int = 0;
    VMA_ITERATOR(vmi, mm, start);
    start = untagged_addr(start);
    if (start & ~PAGE_MASK) {
    return -EINVAL;
    }
//
// flags is used for future extension if any.
//
    if (flags != 0) {
    return -EINVAL;
    }
//
// Check home_node is online to avoid accessing uninitialized
// NODE_DATA.
//
    if (home_node >= MAX_NUMNODES || !node_online(home_node)) {
    return -EINVAL;
    }
    len = PAGE_ALIGN(len);
    end = start + len;
    if (end < start) {
    return -EINVAL;
    }
    if (end == start) {
    return 0;
    }
    mmap_write_lock(mm);
    prev = vma_prev(&vmi);
    for_each_vma_range(vmi, vma, end) {
//
// If any vma in the range got policy other than MPOL_BIND
// or MPOL_PREFERRED_MANY we return error. We don't reset
// the home node for vmas we already updated before.
//
    old = vma_policy(vma);
    if (!old) {
    prev = vma;
    continue;
    }
    if (old.mode != MPOL_BIND && old.mode != MPOL_PREFERRED_MANY) {
    err = -EOPNOTSUPP;
    break;
    }
    new = mpol_dup(old);
    if (IS_ERR(new)) {
    err = PTR_ERR(new);
    break;
    }
    vma_start_write(vma);
    new.home_node = home_node;
    err = mbind_range(&vmi, vma, &prev, start, end, new);
    mpol_put(new);
    if (err) {
    break;
    }
    }
    mmap_write_unlock(mm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_mbind(start: usize, len: usize, mode: usize, nmask: usize, maxnode: usize, flags: usize) -> c_long {
    return kernel_mbind(start, len, mode, nmask, maxnode, flags);
    }
// Set the process memory policy
#[no_mangle]
pub unsafe extern "C" fn kernel_set_mempolicy(mode: c_int, nmask: *mut c_ulong, maxnode: c_ulong) -> c_long {
    let mut mode_flags = 0;
    let mut nodes;
pub static mut lmode: c_int = 0;
    let mut err = 0;
    err = sanitize_mpol_flags(&lmode, &mode_flags);
    if (err) {
    return err;
    }
    err = get_nodes(&nodes, nmask, maxnode);
    if (err) {
    return err;
    }
    return do_set_mempolicy(lmode, mode_flags, &nodes);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_set_mempolicy(mode: usize, nmask: usize, maxnode: usize) -> c_long {
    return kernel_set_mempolicy(mode, nmask, maxnode);
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_migrate_pages(pid: pid_t, maxnode: c_ulong, old_nodes: *mut c_ulong, new_nodes: *mut c_ulong) -> c_int {
    let mut mm = core::ptr::null_mut();
pub static mut task: *mut c_void = core::ptr::null_mut();
    let mut task_nodes;
    let mut err = 0;
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    NODEMASK_SCRATCH(scratch);
    if (!scratch) {
    return -ENOMEM;
    }
    old = &scratch.mask1;
    new = &scratch.mask2;
    err = get_nodes(old, old_nodes, maxnode);
    if (err) {
// goto;
    }
    err = get_nodes(new, new_nodes, maxnode);
    if (err) {
// goto;
    }
// Find the mm_struct
    rcu_read_lock();
    task = pid ? find_task_by_vpid(pid) : current;
    if (!task) {
    rcu_read_unlock();
    err = -ESRCH;
// goto;
    }
    get_task_struct(task);
    err = -EINVAL;
//
// Check if this process has the right to modify the specified process.
// Use the regular "ptrace_may_access()" checks.
//
    if (!ptrace_may_access(task, PTRACE_MODE_READ_REALCREDS)) {
    rcu_read_unlock();
    err = -EPERM;
// goto;
    }
    rcu_read_unlock();
    task_nodes = cpuset_mems_allowed(task);
// Is the user allowed to access the target nodes?
    if (!nodes_subset(*new, task_nodes) && !capable(CAP_SYS_NICE)) {
    err = -EPERM;
// goto;
    }
    task_nodes = cpuset_mems_allowed(current);
    if (!nodes_and(*new, *new, task_nodes)) {
// goto;
    }
    err = security_task_movememory(task);
    if (err) {
// goto;
    }
    mm = get_task_mm(task);
    put_task_struct(task);
    if (!mm) {
    err = -EINVAL;
// goto;
    }
    err = do_migrate_pages(mm, old, new,
    capable(CAP_SYS_NICE) ? MPOL_MF_MOVE_ALL : MPOL_MF_MOVE);
    mmput(mm);
// label;
    NODEMASK_SCRATCH_FREE(scratch);
    return err;
// label;
    put_task_struct(task);
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_migrate_pages(pid: usize, maxnode: usize, old_nodes: usize, new_nodes: usize) -> c_long {
    return kernel_migrate_pages(pid, maxnode, old_nodes, new_nodes);
    }
// Retrieve NUMA policy
#[no_mangle]
pub unsafe extern "C" fn kernel_get_mempolicy(policy: *mut c_int, nmask: *mut c_ulong, maxnode: c_ulong, addr: c_ulong, flags: c_ulong) -> c_int {
    let mut err = 0;
    let mut pval = 0;
    let mut nodes;
    if (nmask != core::ptr::null_mut() && maxnode < nr_node_ids) {
    return -EINVAL;
    }
    addr = untagged_addr(addr);
    err = do_get_mempolicy(&pval, &nodes, addr, flags);
    if (err) {
    return err;
    }
    if (policy && put_user(pval, policy)) {
    return -EFAULT;
    }
    if (nmask) {
    err = copy_nodes_to_user(nmask, maxnode, &nodes);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_get_mempolicy(policy: usize, nmask: usize, maxnode: usize, addr: usize, flags: usize) -> c_long {
    return kernel_get_mempolicy(policy, nmask, maxnode, addr, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vma_migratable(vma: *mut vm_area_struct) -> bool {
    if (vma.vm_flags & (VM_IO | VM_PFNMAP)) {
    return false;
    }
//
// DAX device mappings require predictable access latency, so avoid
// incurring periodic faults.
//
    if (vma_is_dax(vma)) {
    return false;
    }
    if (is_vm_hugetlb_page(vma) &&
    !hugepage_migration_supported(hstate_vma(vma))) {
    return false;
    }
//
// Migration allocates pages in the highest zone. If we cannot
// do so then migration (at least from node to node) is not
// possible.
//
    if (vma.vm_file &&
    gfp_zone(mapping_gfp_mask(vma.vm_file.f_mapping))
    < policy_zone) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __get_vma_policy(vma: *mut vm_area_struct, addr: c_ulong, ilx: *mut pgoff_t) -> *mut c_void {
// ilx = 0;
    return (vma.vm_ops && vma.vm_ops.get_policy) ?
    vma.vm_ops.get_policy(vma, addr, ilx) : vma.vm_policy;
    }
//
// get_vma_policy(@vma, @addr, @order, @ilx)
// @vma: virtual memory area whose policy is sought
// @addr: address in @vma for shared policy lookup
// @order: 0, or appropriate huge_page_order for interleaving
// @ilx: interleave index (output), for use only when MPOL_INTERLEAVE or
// MPOL_WEIGHTED_INTERLEAVE
//
// Returns effective policy for a VMA at specified address.
// Falls back to current->mempolicy or system default policy, as necessary.
// Shared policies [those marked as MPOL_F_SHARED] require an extra reference
// count--added by the get_policy() vm_op, as appropriate--to protect against
// freeing by another task.  It is the caller's responsibility to free the
// extra reference for shared policies.
//
#[no_mangle]
pub unsafe extern "C" fn get_vma_policy(vma: *mut vm_area_struct, addr: c_ulong, order: c_int, ilx: *mut pgoff_t) -> *mut c_void {
pub static mut pol: *mut c_void = core::ptr::null_mut();
    pol = __get_vma_policy(vma, addr, ilx);
    if (!pol) {
    pol = get_task_policy(current);
    }
    if (pol.mode == MPOL_INTERLEAVE ||
    pol.mode == MPOL_WEIGHTED_INTERLEAVE) {
// ilx += vma_start_pgoff(vma) >> order;
// ilx += linear_page_delta(vma, addr) >> order;
    }
    return pol;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_policy_mof(vma: *mut vm_area_struct) -> bool {
pub static mut pol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
    let mut mof = 0;
    pol = __get_vma_policy(vma, vma.vm_start, &ilx);
    if (!pol) {
    pol = get_task_policy(current);
    }
    mof = pol.flags & MPOL_F_MOF;
    mpol_cond_put(pol);
    return mof;
    }
#[no_mangle]
pub unsafe extern "C" fn apply_policy_zone(policy: *mut mempolicy, zone: zone_type) -> bool {
pub static mut dynamic_policy_zone: zone_type = 0;
    BUG_ON!(dynamic_policy_zone == ZONE_MOVABLE);
//
// if policy->nodes has movable memory only,
// we apply policy when gfp_zone(gfp) = ZONE_MOVABLE only.
//
// policy->nodes is intersect with node_states[N_MEMORY].
// so if the following test fails, it implies
// policy->nodes has movable memory only.
//
    if (!nodes_intersects(policy.nodes, node_states[N_HIGH_MEMORY])) {
    dynamic_policy_zone = ZONE_MOVABLE;
    }
    return zone >= dynamic_policy_zone;
    }
#[no_mangle]
unsafe extern "C" fn weighted_interleave_nodes(policy: *mut mempolicy) -> c_uint {
    let mut node = 0;
    let mut cpuset_mems_cookie = 0;
// label;
// to prevent miscount use tsk->mems_allowed_seq to detect rebind
    cpuset_mems_cookie = read_mems_allowed_begin();
    node = current.il_prev;
    if (!current.il_weight || !node_isset(node, policy.nodes)) {
    node = next_node_in(node, policy.nodes);
    if (read_mems_allowed_retry(cpuset_mems_cookie)) {
// goto;
    }
    if (node == MAX_NUMNODES) {
    return node;
    }
    current.il_prev = node;
    current.il_weight = get_il_weight(node);
    }
    current.il_weight -= 1;
    return node;
    }
// Do dynamic interleaving for a process
#[no_mangle]
unsafe extern "C" fn interleave_nodes(policy: *mut mempolicy) -> c_uint {
    let mut nid = 0;
    let mut cpuset_mems_cookie = 0;
// to prevent miscount, use tsk->mems_allowed_seq to detect rebind
    do {
    cpuset_mems_cookie = read_mems_allowed_begin();
    nid = next_node_in(current.il_prev, policy.nodes);
    } while (read_mems_allowed_retry(cpuset_mems_cookie));
    if (nid < MAX_NUMNODES) {
    current.il_prev = nid;
    }
    return nid;
    }
//
// Depending on the memory policy provide a node from which to allocate the
// next slab entry.
//
#[no_mangle]
pub unsafe extern "C" fn mempolicy_slab_node() -> c_uint {
pub static mut policy: *mut c_void = core::ptr::null_mut();
pub static mut node: c_int = 0;
    if (!in_task()) {
    return node;
    }
    policy = current.mempolicy;
    if (!policy) {
    return node;
    }
    match (policy.mode) {
    MPOL_PREFERRED => {
    return first_node(policy.nodes);
    }
    MPOL_INTERLEAVE => {
    return interleave_nodes(policy);
    }
    MPOL_WEIGHTED_INTERLEAVE => {
    return weighted_interleave_nodes(policy);
    }
    MPOL_BIND => {
    }
    MPOL_PREFERRED_MANY => {
    {
pub static mut z: *mut c_void = core::ptr::null_mut();
//
// Follow bind policy behavior and start allocation at the
// first node.
//
pub static mut zonelist: *mut c_void = core::ptr::null_mut();
pub static mut highest_zoneidx: zone_type = 0;
    zonelist = &NODE_DATA(node).node_zonelists[ZONELIST_FALLBACK];
    z = first_zones_zonelist(zonelist, highest_zoneidx,
    &policy.nodes);
    return zonelist_zone(z) ? zonelist_node_idx(z) : node;
    }
    }
    MPOL_LOCAL => {
    return node;
    }
    _ => {
    BUG();
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn read_once_policy_nodemask(pol: *mut mempolicy, mask: *mut nodemask_t) -> c_uint {
//
// barrier stabilizes the nodemask locally so that it can be iterated
// over safely without concern for changes. Allocators validate node
// selection does not violate mems_allowed, so this is safe.
//
    barrier();
    memcpy(mask, &pol.nodes, sizeof!(nodemask_t));
    barrier();
    return nodes_weight(*mask);
    }
#[no_mangle]
unsafe extern "C" fn weighted_interleave_nid(pol: *mut mempolicy, ilx: pgoff_t) -> c_uint {
pub static mut state: *mut c_void = core::ptr::null_mut();
    let mut nodemask;
    let mut target = 0;
    let mut nr_nodes = 0;
    let mut table = core::ptr::null_mut();
pub static mut weight_total: c_uint = 0;
    let mut weight = 0;
pub static mut nid: c_int = 0;
    nr_nodes = read_once_policy_nodemask(pol, &nodemask);
    if (!nr_nodes) {
    return numa_node_id();
    }
    rcu_read_lock();
    state = rcu_dereference(wi_state);
// Uninitialized wi_state means we should assume all weights are 1
    if (state) {
    table = state.iw_table;
    }
// calculate the total weight
    for_each_node_mask(nid, nodemask) {
    weight_total += table ? table[nid] : 1;
    }
// Calculate the node offset based on totals
    target = ilx % weight_total;
    nid = first_node(nodemask);
    while (target) {
// detect system default usage
    weight = table ? table[nid] : 1;
    if (target < weight) {
    break;
    }
    target -= weight;
    nid = next_node_in(nid, nodemask);
    }
    rcu_read_unlock();
    return nid;
    }
//
// Do static interleaving for interleave index @ilx.  Returns the ilx'th
// node in pol->nodes (starting from ilx=0), wrapping around if ilx
// exceeds the number of present nodes.
//
#[no_mangle]
unsafe extern "C" fn interleave_nid(pol: *mut mempolicy, ilx: pgoff_t) -> c_uint {
    let mut nodemask;
    let mut target = 0;
    let mut nnodes = 0;
    let mut i = 0;
    let mut nid = 0;
    nnodes = read_once_policy_nodemask(pol, &nodemask);
    if (!nnodes) {
    return numa_node_id();
    }
    target = ilx % nnodes;
    nid = first_node(nodemask);
    for (i = 0; i < target; i++) {
    nid = next_node(nid, nodemask);
    }
    return nid;
    }
//
// Return a nodemask representing a mempolicy for filtering nodes for
// page allocation, together with preferred node id (or the input node id).
//
    static nodemask_t *policy_nodemask(gfp_t gfp, mempolicy *pol,
    pgoff_t ilx, int *nid)
    {
    let mut nodemask = core::ptr::null_mut();
    match (pol.mode) {
    MPOL_PREFERRED => {
// Override input node id
// nid = first_node(pol->nodes);
    // break;
    }
    MPOL_PREFERRED_MANY => {
    nodemask = &pol.nodes;
    if (pol.home_node != NUMA_NO_NODE) {
// nid = pol->home_node;
    }
    // break;
    }
    MPOL_BIND => {
// Restrict to nodemask (but not on lower zones)
    if (apply_policy_zone(pol, gfp_zone(gfp)) &&
    cpuset_nodemask_valid_mems_allowed(&pol.nodes)) {
    nodemask = &pol.nodes;
    }
    if (pol.home_node != NUMA_NO_NODE) {
// nid = pol->home_node;
    }
//
// __GFP_THISNODE shouldn't even be used with the bind policy
// because we might easily break the expectation to stay on the
// requested node and not break the policy.
//
    WARN_ON_ONCE!(gfp & __GFP_THISNODE);
    // break;
    }
    MPOL_INTERLEAVE => {
// Override input node id
// nid = (ilx == NO_INTERLEAVE_INDEX) ?
// forward_decl: erleave_nodes;
    // break;
    }
    MPOL_WEIGHTED_INTERLEAVE => {
// nid = (ilx == NO_INTERLEAVE_INDEX) ?
    weighted_interleave_nodes(pol) :
    weighted_interleave_nid(pol, ilx);
    // break;
    }
    }
    return nodemask;
    }

//
// huge_node(@vma, @addr, @gfp_flags, @mpol)
// @vma: virtual memory area whose policy is sought
// @addr: address in @vma for shared policy lookup and interleave policy
// @gfp_flags: for requested zone
// @mpol: pointer to mempolicy pointer for reference counted mempolicy
// @nodemask: pointer to nodemask pointer for 'bind' and 'prefer-many' policy
//
// Returns a nid suitable for a huge page allocation and a pointer
// to the struct mempolicy for conditional unref after allocation.
// If the effective policy is 'bind' or 'prefer-many', returns a pointer
// to the mempolicy's @nodemask for filtering the zonelist.
//
#[no_mangle]
pub unsafe extern "C" fn huge_node(vma: *mut vm_area_struct, addr: c_ulong, gfp_flags: gfp_t, mpol: *mut *mut mempolicy, nodemask: *mut *mut nodemask_t) -> c_int {
    let mut ilx;
    let mut nid = 0;
    nid = numa_node_id();
// mpol = get_vma_policy(vma, addr, hstate_vma(vma)->order, &ilx);
// nodemask = policy_nodemask(gfp_flags, *mpol, ilx, &nid);
    return nid;
    }
//
// init_nodemask_of_mempolicy
//
// If the current task's mempolicy is "default" [NULL], return 'false'
// to indicate default policy.  Otherwise, extract the policy nodemask
// for 'bind' or 'interleave' policy into the argument nodemask, or
// initialize the argument nodemask to contain the single node for
// 'preferred' or 'local' policy and return 'true' to indicate presence
// of non-default mempolicy.
//
// We don't bother with reference counting the mempolicy [mpol_get/put]
// because the current task is examining it's own mempolicy and a task's
// mempolicy is only ever changed by the task itself.
//
// N.B., it is the caller's responsibility to free a returned nodemask.
//
#[no_mangle]
pub unsafe extern "C" fn init_nodemask_of_mempolicy(mask: *mut nodemask_t) -> bool {
pub static mut mempolicy: *mut c_void = core::ptr::null_mut();
    if (!(mask && current.mempolicy)) {
    return false;
    }
    task_lock(current);
    mempolicy = current.mempolicy;
    match (mempolicy.mode) {
    MPOL_PREFERRED => {
    }
    MPOL_PREFERRED_MANY => {
    }
    MPOL_BIND => {
    }
    MPOL_INTERLEAVE => {
    }
    MPOL_WEIGHTED_INTERLEAVE => {
// mask = mempolicy->nodes;
    // break;
    }
    MPOL_LOCAL => {
    init_nodemask_of_node(mask, numa_node_id());
    // break;
    }
    _ => {
    BUG();
    }
    }
    task_unlock(current);
    return true;
    }

//
// mempolicy_in_oom_domain
//
// If tsk's mempolicy is "bind", check for intersection between mask and
// the policy nodemask. Otherwise, return true for all other policies
// including "interleave", as a tsk with "interleave" policy may have
// memory allocated from all nodes in system.
//
// Takes task_lock(tsk) to prevent freeing of its mempolicy.
//
#[no_mangle]
pub unsafe extern "C" fn mempolicy_in_oom_domain(tsk: *mut task_struct, mask: *mut nodemask_t) -> bool {
pub static mut mempolicy: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = true;
    if (!mask) {
    return ret;
    }
    task_lock(tsk);
    mempolicy = tsk.mempolicy;
    if (mempolicy && mempolicy.mode == MPOL_BIND) {
    ret = nodes_intersects(mempolicy.nodes, *mask);
    }
    task_unlock(tsk);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_preferred_many(gfp: gfp_t, order: c_uint, nid: c_int, nodemask: *mut nodemask_t) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut preferred_gfp;
//
// This is a two pass approach. The first pass will only try the
// preferred nodes but skip the direct reclaim and allow the
// allocation to fail, while the second pass will try all the
// nodes in system.
//
    preferred_gfp = gfp | __GFP_NOWARN;
    preferred_gfp &= ~(__GFP_DIRECT_RECLAIM | __GFP_NOFAIL);
    page = __alloc_frozen_pages_noprof(preferred_gfp, order, nid, nodemask,
    ALLOC_DEFAULT);
    if (!page) {
    page = __alloc_frozen_pages_noprof(gfp, order, nid, core::ptr::null_mut(),
    ALLOC_DEFAULT);
    }
    return page;
    }
//
// alloc_pages_mpol - Allocate pages according to NUMA mempolicy.
// @gfp: GFP flags.
// @order: Order of the page allocation.
// @pol: Pointer to the NUMA mempolicy.
// @ilx: Index for interleave mempolicy (also distinguishes alloc_pages()).
// @nid: Preferred node (usually numa_node_id() but @mpol may override it).
//
// Return: The page on success or NULL if allocation fails.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_mpol(gfp: gfp_t, order: c_uint, pol: *mut mempolicy, ilx: pgoff_t, nid: c_int) -> *mut c_void {
pub static mut nodemask: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    nodemask = policy_nodemask(gfp, pol, ilx, &nid);
    if (pol.mode == MPOL_PREFERRED_MANY) {
    return alloc_pages_preferred_many(gfp, order, nid, nodemask);
    }
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
filter "hugepage" allocation, unless from alloc_pages()
    is_pmd_order(order) && ilx != NO_INTERLEAVE_INDEX) {
//
// For hugepage allocation and non-interleave policy which
// allows the current node (or other explicitly preferred
// node) we only try to allocate from the current/preferred
// node and don't fall back to other nodes, as the cost of
// remote accesses would likely offset THP benefits.
//
// If the policy is interleave or does not allow the current
// node in its nodemask, we allocate the standard way.
//
    if (pol.mode != MPOL_INTERLEAVE &&
    pol.mode != MPOL_WEIGHTED_INTERLEAVE &&
    (!nodemask || node_isset(nid, *nodemask))) {
//
// First, try to allocate THP only on local node, but
// don't reclaim unnecessarily, just compact.
//
    page = __alloc_frozen_pages_noprof(
    gfp | __GFP_THISNODE | __GFP_NORETRY, order,
    nid, core::ptr::null_mut(), ALLOC_DEFAULT);
    if (page || !(gfp & __GFP_DIRECT_RECLAIM)) {
    return page;
    }
//
// If hugepage allocations are configured to always
// synchronous compact or the vma has been madvised
// to prefer hugepage backing, retry allowing remote
// memory with both reclaim and compact as well.
//
    }
    }
    page = __alloc_frozen_pages_noprof(gfp, order, nid, nodemask, ALLOC_DEFAULT);
    if (unlikely(pol.mode == MPOL_INTERLEAVE ||
    pol.mode == MPOL_WEIGHTED_INTERLEAVE) && page) {
// skip NUMA_INTERLEAVE_HIT update if numa stats is disabled
    if (static_branch_likely(&vm_numa_stat_key) &&
    page_to_nid(page) == nid) {
    preempt_disable();
    __count_numa_event(page_zone(page), NUMA_INTERLEAVE_HIT);
    preempt_enable();
    }
    }
    return page;
    }
#[no_mangle]
pub unsafe extern "C" fn folio_alloc_mpol_noprof(gfp: gfp_t, order: c_uint, pol: *mut mempolicy, ilx: pgoff_t, nid: c_int) -> *mut c_void {
    let mut page = alloc_pages_mpol(gfp | __GFP_COMP, order, pol,
    ilx, nid);
    if (!page) {
    return core::ptr::null_mut();
    }
    set_page_refcounted(page);
    return page_rmappable_folio(page);
    }
//
// vma_alloc_folio - Allocate a folio for a VMA.
// @gfp: GFP flags.
// @order: Order of the folio.
// @vma: Pointer to VMA.
// @addr: Virtual address of the allocation.  Must be inside @vma.
//
// Allocate a folio for a specific address in @vma, using the appropriate
// NUMA policy.  The caller must hold the mmap_lock of the mm_struct of the
// VMA to prevent it from going away.  Should be used for all allocations
// for folios that will be mapped into user space, excepting hugetlbfs, and
// excepting where direct use of folio_alloc_mpol() is more appropriate.
//
// Return: The folio on success or NULL if allocation fails.
//
#[no_mangle]
pub unsafe extern "C" fn vma_alloc_folio_noprof(gfp: gfp_t, order: c_int, vma: *mut vm_area_struct, addr: c_ulong) -> *mut c_void {
pub static mut pol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (vma.vm_flags & VM_DROPPABLE) {
    gfp |= __GFP_NOWARN;
    }
    pol = get_vma_policy(vma, addr, order, &ilx);
    folio = folio_alloc_mpol_noprof(gfp, order, pol, ilx, numa_node_id());
    mpol_cond_put(pol);
    return folio;
    }
    EXPORT_SYMBOL(vma_alloc_folio_noprof);
#[no_mangle]
pub unsafe extern "C" fn alloc_frozen_pages_noprof(gfp: gfp_t, order: c_uint) -> *mut c_void {
    let mut pol = &default_policy;
//
// No reference counting needed for current->mempolicy
// nor system default_policy
//
    if (!in_interrupt() && !(gfp & __GFP_THISNODE)) {
    pol = get_task_policy(current);
    }
    return alloc_pages_mpol(gfp, order, pol, NO_INTERLEAVE_INDEX,
    numa_node_id());
    }
//
// alloc_pages - Allocate pages.
// @gfp: GFP flags.
// @order: Power of two of number of pages to allocate.
//
// Allocate 1 << @order contiguous pages.  The physical address of the
// first page is naturally aligned (eg an order-3 allocation will be aligned
// to a multiple of 8 * PAGE_SIZE bytes).  The NUMA policy of the current
// process is honoured when in process context.
//
// Context: Can be called from any context, providing the appropriate GFP
// flags are used.
// Return: The page on success or NULL if allocation fails.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_noprof(gfp: gfp_t, order: c_uint) -> *mut c_void {
    let mut page = alloc_frozen_pages_noprof(gfp, order);
    if (page) {
    set_page_refcounted(page);
    }
    return page;
    }
    EXPORT_SYMBOL(alloc_pages_noprof);
#[no_mangle]
pub unsafe extern "C" fn folio_alloc_noprof(gfp: gfp_t, order: c_uint) -> *mut c_void {
    return page_rmappable_folio(alloc_pages_noprof(gfp | __GFP_COMP, order));
    }
    EXPORT_SYMBOL(folio_alloc_noprof);
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_bulk_interleave(gfp: gfp_t, pol: *mut mempolicy, nr_pages: c_ulong, page_array: *mut *mut page) -> c_ulong {
    let mut nodes = 0;
    let mut nr_pages_per_node = 0;
    let mut delta = 0;
    let mut i = 0;
    let mut nr_allocated = 0;
pub static mut total_allocated: c_ulong = 0;
    nodes = nodes_weight(pol.nodes);
    nr_pages_per_node = nr_pages / nodes;
    delta = nr_pages - nodes * nr_pages_per_node;
    while (i < nodes) {
    if (delta) {
    nr_allocated = alloc_pages_bulk_noprof(gfp,
// forward_decl: erleave_nodes;
    delta -= 1;
    } else {
    nr_allocated = alloc_pages_bulk_noprof(gfp,
// forward_decl: erleave_nodes;
    }
    page_array += nr_allocated;
    total_allocated += nr_allocated;
    }
    return total_allocated;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_bulk_weighted_interleave(gfp: gfp_t, pol: *mut mempolicy, nr_pages: c_ulong, page_array: *mut *mut page) -> c_ulong {
pub static mut state: *mut c_void = core::ptr::null_mut();
    let mut me = current;
    let mut cpuset_mems_cookie = 0;
pub static mut total_allocated: c_ulong = 0;
pub static mut nr_allocated: c_ulong = 0;
    let mut rounds = 0;
    unsigned long node_pages, delta;
    u8 *weights, weight;
pub static mut weight_total: c_uint = 0;
pub static mut rem_pages: c_ulong = 0;
    let mut nodes;
    let mut nnodes = 0;
    let mut node = 0;
pub static mut resume_node: c_int = 0;
pub static mut resume_weight: u8 = 0;
    let mut prev_node = 0;
    let mut i = 0;
    if (!nr_pages) {
    return 0;
    }
// read the nodes onto the stack, retry if done during rebind
    do {
    cpuset_mems_cookie = read_mems_allowed_begin();
    nnodes = read_once_policy_nodemask(pol, &nodes);
    } while (read_mems_allowed_retry(cpuset_mems_cookie));
// if the nodemask has become invalid, we cannot do anything
    if (!nnodes) {
    return 0;
    }
// Continue allocating from most recent node and adjust the nr_pages
    node = me.il_prev;
    weight = me.il_weight;
    if (weight && node_isset(node, nodes)) {
    node_pages = min(rem_pages, weight);
    nr_allocated = __alloc_pages_bulk(gfp, node, core::ptr::null_mut(), node_pages,
    page_array);
    page_array += nr_allocated;
    total_allocated += nr_allocated;
// if that's all the pages, no need to interleave
    if (rem_pages <= weight) {
    me.il_weight -= rem_pages;
    return total_allocated;
    }
// Otherwise we adjust remaining pages, continue from there
    rem_pages -= weight;
    }
// clear active weight in case of an allocation failure
    me.il_weight = 0;
    prev_node = node;
// create a local copy of node weights to operate on outside rcu
    weights = kmalloc(nr_node_ids, gfp & GFP_RECLAIM_MASK);
    if (!weights) {
    return total_allocated;
    }
    rcu_read_lock();
    state = rcu_dereference(wi_state);
    if (state) {
    memcpy(weights, state.iw_table, nr_node_ids * sizeof!(u8));
    rcu_read_unlock();
    } else {
    rcu_read_unlock();
    for (i = 0; i < nr_node_ids; i++) {
    weights[i] = 1;
    }
    }
// calculate total, detect system default usage
    for_each_node_mask(node, nodes) {
    weight_total += weights[node];
    }
//
// Calculate rounds/partial rounds to minimize __alloc_pages_bulk calls.
// Track which node weighted interleave should resume from.
//
// if (rounds > 0) and (delta == 0), resume_node will always be
// the node following prev_node and its weight.
//
    rounds = rem_pages / weight_total;
    delta = rem_pages % weight_total;
    resume_node = next_node_in(prev_node, nodes);
    resume_weight = weights[resume_node];
    while (i < nnodes) {
    node = next_node_in(prev_node, nodes);
    weight = weights[node];
    node_pages = weight * rounds;
// If a delta exists, add this node's portion of the delta
    if (delta > weight) {
    node_pages += weight;
    delta -= weight;
    } else if (delta) {
// when delta is depleted, resume from that node
    node_pages += delta;
    resume_node = node;
    resume_weight = weight - delta;
    delta = 0;
    }
// node_pages can be 0 if an allocation fails and rounds == 0
    if (!node_pages) {
    break;
    }
    nr_allocated = __alloc_pages_bulk(gfp, node, core::ptr::null_mut(), node_pages,
    page_array);
    page_array += nr_allocated;
    total_allocated += nr_allocated;
    if (total_allocated == nr_pages) {
    break;
    }
    prev_node = node;
    }
    me.il_prev = resume_node;
    me.il_weight = resume_weight;
    kfree(weights);
    return total_allocated;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_bulk_preferred_many(gfp: gfp_t, nid: c_int, pol: *mut mempolicy, nr_pages: c_ulong, page_array: *mut *mut page) -> c_ulong {
    let mut preferred_gfp;
pub static mut nr_allocated: c_ulong = 0;
    preferred_gfp = gfp | __GFP_NOWARN;
    preferred_gfp &= ~(__GFP_DIRECT_RECLAIM | __GFP_NOFAIL);
    nr_allocated  = alloc_pages_bulk_noprof(preferred_gfp, nid, &pol.nodes,
    nr_pages, page_array);
    if (nr_allocated < nr_pages) {
    nr_allocated += alloc_pages_bulk_noprof(gfp, numa_node_id(), core::ptr::null_mut(),
    nr_pages - nr_allocated,
    page_array + nr_allocated);
    }
    return nr_allocated;
    }
// alloc pages bulk and mempolicy should be considered at the
// same time in some situation such as vmalloc.
//
// It can accelerate memory allocation especially interleaving
// allocate memory.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_bulk_mempolicy_noprof(gfp: gfp_t, nr_pages: c_ulong, page_array: *mut *mut page) -> c_ulong {
    let mut pol = &default_policy;
pub static mut nodemask: *mut c_void = core::ptr::null_mut();
    let mut nid = 0;
    if (!in_interrupt() && !(gfp & __GFP_THISNODE)) {
    pol = get_task_policy(current);
    }
    if (pol.mode == MPOL_INTERLEAVE) {
    return alloc_pages_bulk_interleave(gfp, pol,
    nr_pages, page_array);
    }
    if (pol.mode == MPOL_WEIGHTED_INTERLEAVE) {
    return alloc_pages_bulk_weighted_interleave(
    gfp, pol, nr_pages, page_array);
    }
    if (pol.mode == MPOL_PREFERRED_MANY) {
    return alloc_pages_bulk_preferred_many(gfp,
    numa_node_id(), pol, nr_pages, page_array);
    }
    nid = numa_node_id();
    nodemask = policy_nodemask(gfp, pol, NO_INTERLEAVE_INDEX, &nid);
    return alloc_pages_bulk_noprof(gfp, nid, nodemask,
    nr_pages, page_array);
    }
#[no_mangle]
pub unsafe extern "C" fn vma_dup_policy(src: *mut vm_area_struct, dst: *mut vm_area_struct) -> c_int {
    let mut pol = mpol_dup(src.vm_policy);
    if (IS_ERR(pol)) {
    return PTR_ERR(pol);
    }
    dst.vm_policy = pol;
    return 0;
    }
//
// If mpol_dup() sees current->cpuset == cpuset_being_rebound, then it
// rebinds the mempolicy its copying by calling mpol_rebind_policy()
// with the mems_allowed returned by cpuset_mems_allowed().  This
// keeps mempolicies cpuset relative after its cpuset moves.  See
// further kernel/cpuset.c update_nodemask().
//
// current's mempolicy may be rebinded by the other task(the task that changes
// cpuset's mems), so we needn't do rebind work for current task.
//
// Slow path of a mempolicy duplicate
#[no_mangle]
pub unsafe extern "C" fn __mpol_dup(old: *mut mempolicy) -> *mut c_void {
    let mut new = kmem_cache_alloc(policy_cache, GFP_KERNEL);
    if (!new) {
    return ERR_PTR(-ENOMEM);
    }
// task's mempolicy is protected by alloc_lock
    if (old == current.mempolicy) {
    task_lock(current);
// new = *old;
    task_unlock(current);
    } else {
// new = *old;
    }
    if (current_cpuset_is_being_rebound()) {
pub static mut mems: nodemask_t = 0;
    mpol_rebind_policy(new, &mems);
    }
    atomic_set(&new.refcnt, 1);
    return new;
    }
// Slow path of a mempolicy comparison
#[no_mangle]
pub unsafe extern "C" fn __mpol_equal(a: *mut mempolicy, b: *mut mempolicy) -> bool {
    if (!a || !b) {
    return false;
    }
    if (a.mode != b.mode) {
    return false;
    }
    if (a.flags != b.flags) {
    return false;
    }
    if (a.home_node != b.home_node) {
    return false;
    }
    if (mpol_store_user_nodemask(a)) {
    if (!nodes_equal(a.w.user_nodemask, b.w.user_nodemask))
    return false;
    }
    match (a.mode) {
    MPOL_BIND => {
    }
    MPOL_INTERLEAVE => {
    }
    MPOL_PREFERRED => {
    }
    MPOL_PREFERRED_MANY => {
    }
    MPOL_WEIGHTED_INTERLEAVE => {
    return nodes_equal(a.nodes, b.nodes);
    }
    MPOL_LOCAL => {
    return true;
    }
    _ => {
    BUG();
    return false;
    }
    }
    }
//
// Shared memory backing store policy support.
//
// Remember policies even when nobody has shared memory mapped.
// The policies are kept in Red-Black tree linked from the inode.
// They are protected by the sp->lock rwlock, which should be held
// for any accesses to the tree.
//
// lookup first element intersecting start-end.  Caller holds sp->lock for
// reading or for writing
//
#[no_mangle]
pub unsafe extern "C" fn sp_lookup(sp: *mut shared_policy, start: pgoff_t, end: pgoff_t) -> *mut c_void {
    let mut n = sp.root.rb_node;
    while (n) {
    let mut p = rb_entry(n, sp_node, nd);
    if (start >= p.end) {
    n = n.rb_right;
    }

    else if (end <= p.start) {
    n = n.rb_left;
    }
    else {
    break;
    }
    }
    if (!n) {
    return core::ptr::null_mut();
    }
    for (;;) {
    let mut w = core::ptr::null_mut();
    let mut prev = rb_prev(n);
    if (!prev) {
    break;
    }
    w = rb_entry(prev, sp_node, nd);
    if (w.end <= start) {
    break;
    }
    n = prev;
    }
    return rb_entry(n, sp_node, nd);
    }
//
// Insert a new shared policy into the list.  Caller holds sp->lock for
// writing.
//
#[no_mangle]
unsafe extern "C" fn sp_insert(sp: *mut shared_policy, new: *mut sp_node) {
    let mut p = &sp.root.rb_node;
    let mut parent = core::ptr::null_mut();
pub static mut nd: *mut c_void = core::ptr::null_mut();
    while (*p) {
    parent = *p;
    nd = rb_entry(parent, sp_node, nd);
    if (new.start < nd.start) {
    p = &(*p).rb_left;
    }

    else if (new.end > nd.end) {
    p = &(*p).rb_right;
    }
    else {
    BUG();
    }
    }
    rb_link_node(&new.nd, parent, p);
    rb_insert_color(&new.nd, &sp.root);
    }
// Find shared policy intersecting idx
#[no_mangle]
pub unsafe extern "C" fn mpol_shared_policy_lookup(sp: *mut shared_policy, idx: pgoff_t) -> *mut c_void {
    let mut pol = core::ptr::null_mut();
pub static mut sn: *mut c_void = core::ptr::null_mut();
    if (!sp.root.rb_node) {
    return core::ptr::null_mut();
    }
    read_lock(&sp.lock);
    sn = sp_lookup(sp, idx, idx+1);
    if (sn) {
    mpol_get(sn.policy);
    pol = sn.policy;
    }
    read_unlock(&sp.lock);
    return pol;
    }
    EXPORT_SYMBOL_FOR_MODULES(mpol_shared_policy_lookup, "kvm");
#[no_mangle]
unsafe extern "C" fn sp_free(n: *mut sp_node) {
    mpol_put(n.policy);
    kmem_cache_free(sn_cache, n);
    }
//
// mpol_misplaced - check whether current folio node is valid in policy
//
// @folio: folio to be checked
// @vmf: structure describing the fault
// @addr: virtual address in @vma for shared policy lookup and interleave policy
//
// Lookup current policy node id for vma,addr and "compare to" folio's
// node id.  Policy determination "mimics" alloc_page_vma().
// Called from fault path where we know the vma and faulting address.
//
// Return: NUMA_NO_NODE if the page is in a node that is valid for this
// policy, or a suitable node ID to allocate a replacement folio from.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_misplaced(folio: *mut folio, vmf: *mut vm_fault, addr: c_ulong) -> c_int {
pub static mut pol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
pub static mut z: *mut c_void = core::ptr::null_mut();
pub static mut curnid: c_int = 0;
    let mut vma = vmf.vma;
pub static mut thiscpu: c_int = 0;
pub static mut thisnid: c_int = 0;
pub static mut polnid: c_int = 0;
pub static mut ret: c_int = 0;
//
// Make sure ptl is held so that we don't preempt and we
// have a stable smp processor id
//
    lockdep_assert_held(vmf.ptl);
    pol = get_vma_policy(vma, addr, folio_order(folio), &ilx);
    if (!(pol.flags & MPOL_F_MOF)) {
// goto;
    }
    match (pol.mode) {
    MPOL_INTERLEAVE => {
    polnid = interleave_nid(pol, ilx);
    // break;
    }
    MPOL_WEIGHTED_INTERLEAVE => {
    polnid = weighted_interleave_nid(pol, ilx);
    // break;
    }
    MPOL_PREFERRED => {
    if (node_isset(curnid, pol.nodes)) {
// goto;
    }
    polnid = first_node(pol.nodes);
    // break;
    }
    MPOL_LOCAL => {
    polnid = numa_node_id();
    // break;
    }
    MPOL_BIND => {
    }
    MPOL_PREFERRED_MANY => {
//
// Even though MPOL_PREFERRED_MANY can allocate pages outside
// policy nodemask we don't allow numa migration to nodes
// outside policy nodemask for now. This is done so that if we
// want demotion to slow memory to happen, before allocating
// from some DRAM node say 'x', we will end up using a
// MPOL_PREFERRED_MANY mask excluding node 'x'. In such scenario
// we should not promote to node 'x' from slow memory node.
//
    if (pol.flags & MPOL_F_MORON) {
//
// Optimize placement among multiple nodes
// via NUMA balancing
//
    if (node_isset(thisnid, pol.nodes)) {
    // break;
    }
// goto;
    }
//
// use current page if in policy nodemask,
// else select nearest allowed node, if any.
// If no allowed nodes, use current [!misplaced].
//
    if (node_isset(curnid, pol.nodes)) {
// goto;
    }
    z = first_zones_zonelist(
    node_zonelist(thisnid, GFP_HIGHUSER),
    gfp_zone(GFP_HIGHUSER),
    &pol.nodes);
    polnid = zonelist_node_idx(z);
    // break;
    }
    _ => {
    BUG();
    }
    }
// Migrate the folio towards the node whose CPU is referencing it
    if (pol.flags & MPOL_F_MORON) {
    polnid = thisnid;
    if (!should_numa_migrate_memory(current, folio, curnid,
    thiscpu)) {
// goto;
    }
    }
    if (curnid != polnid) {
    ret = polnid;
    }
// label;
    mpol_cond_put(pol);
    return ret;
    }
//
// Drop the (possibly final) reference to task->mempolicy.  It needs to be
// dropped after task->mempolicy is set to NULL so that any allocation done as
// part of its kmem_cache_free(), such as by KASAN, doesn't reference a freed
// policy.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_put_task_policy(task: *mut task_struct) {
pub static mut pol: *mut c_void = core::ptr::null_mut();
    task_lock(task);
    pol = task.mempolicy;
    task.mempolicy = core::ptr::null_mut();
    task_unlock(task);
    mpol_put(pol);
    }
#[no_mangle]
unsafe extern "C" fn sp_delete(sp: *mut shared_policy, n: *mut sp_node) {
    rb_erase(&n.nd, &sp.root);
    sp_free(n);
    }
#[no_mangle]
pub unsafe extern "C" fn sp_node_init(node: *mut sp_node, start: c_ulong, end: c_ulong, pol: *mut mempolicy) {
    node.start = start;
    node.end = end;
    node.policy = pol;
    }
#[no_mangle]
pub unsafe extern "C" fn sp_alloc(start: c_ulong, end: c_ulong, pol: *mut mempolicy) -> *mut c_void {
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut newpol: *mut c_void = core::ptr::null_mut();
    n = kmem_cache_alloc(sn_cache, GFP_KERNEL);
    if (!n) {
    return core::ptr::null_mut();
    }
    newpol = mpol_dup(pol);
    if (IS_ERR(newpol)) {
    kmem_cache_free(sn_cache, n);
    return core::ptr::null_mut();
    }
    newpol.flags |= MPOL_F_SHARED;
    sp_node_init(n, start, end, newpol);
    return n;
    }
// Replace a policy range.
#[no_mangle]
pub unsafe extern "C" fn shared_policy_replace(sp: *mut shared_policy, start: pgoff_t, end: pgoff_t, new: *mut sp_node) -> c_int {
pub static mut n: *mut c_void = core::ptr::null_mut();
    let mut n_new = core::ptr::null_mut();
    let mut mpol_new = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// label;
    write_lock(&sp.lock);
    n = sp_lookup(sp, start, end);
// Take care of old policies in the same range.
    while (n && n.start < end) {
    let mut next = rb_next(&n.nd);
    if (n.start >= start) {
    if (n.end <= end) {
    sp_delete(sp, n);
    }
    else {
    n.start = end;
    }
    } else {
// Old policy spanning whole new range.
    if (n.end > end) {
    if (!n_new) {
// goto;
    }
// mpol_new = *n->policy;
    atomic_set(&mpol_new.refcnt, 1);
    sp_node_init(n_new, end, n.end, mpol_new);
    n.end = start;
    sp_insert(sp, n_new);
    n_new = core::ptr::null_mut();
    mpol_new = core::ptr::null_mut();
    break;
    } else {
    n.end = start;
    }
    }
    if (!next) {
    break;
    }
    n = rb_entry(next, sp_node, nd);
    }
    if (new) {
    sp_insert(sp, new);
    }
    write_unlock(&sp.lock);
    ret = 0;
// label;
    if (mpol_new) {
    mpol_put(mpol_new);
    }
    if (n_new) {
    kmem_cache_free(sn_cache, n_new);
    }
    return ret;
// label;
    write_unlock(&sp.lock);
    ret = -ENOMEM;
    n_new = kmem_cache_alloc(sn_cache, GFP_KERNEL);
    if (!n_new) {
// goto;
    }
    mpol_new = kmem_cache_alloc(policy_cache, GFP_KERNEL);
    if (!mpol_new) {
// goto;
    }
    atomic_set(&mpol_new.refcnt, 1);
// goto;
    }
//
// mpol_shared_policy_init - initialize shared policy for inode
// @sp: pointer to inode shared policy
// @mpol: mempolicy to install
//
// Install non-NULL @mpol in inode's shared policy rb-tree.
// On entry, the current task has a reference on a non-NULL @mpol.
// This must be released on exit.
// This is called at get_inode() calls and we can use GFP_KERNEL.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_shared_policy_init(sp: *mut shared_policy, mpol: *mut mempolicy) {
    let mut ret = 0;
    sp.root = RB_ROOT;		/* empty tree == default mempolicy */
    rwlock_init(&sp.lock);
    if (mpol) {
pub static mut sn: *mut c_void = core::ptr::null_mut();
pub static mut npol: *mut c_void = core::ptr::null_mut();
    NODEMASK_SCRATCH(scratch);
    if (!scratch) {
// goto;
    }
// contextualize the tmpfs mount point mempolicy to this file
    npol = mpol_new(mpol.mode, mpol.flags, &mpol.w.user_nodemask);
    if (IS_ERR(npol)) {
// goto; /* no valid nodemask intersection */
    }
    task_lock(current);
    ret = mpol_set_nodemask(npol, &mpol.w.user_nodemask, scratch);
    task_unlock(current);
    if (ret) {
// goto;
    }
// alloc node covering entire file; adds ref to file's npol
    sn = sp_alloc(0, MAX_LFS_FILESIZE >> PAGE_SHIFT, npol);
    if (sn) {
    sp_insert(sp, sn);
    }
// label;
    mpol_put(npol);	/* drop initial ref on file's npol */
// label;
    NODEMASK_SCRATCH_FREE(scratch);
// label;
    mpol_put(mpol);	/* drop our incoming ref on sb mpol */
    }
    }
    EXPORT_SYMBOL_FOR_MODULES(mpol_shared_policy_init, "kvm");
#[no_mangle]
pub unsafe extern "C" fn mpol_set_shared_policy(sp: *mut shared_policy, vma: *mut vm_area_struct, pol: *mut mempolicy) -> c_int {
pub static mut pgoff: pgoff_t = 0;
pub static mut pgoff_end: pgoff_t = 0;
    let mut new = core::ptr::null_mut();
    let mut err = 0;
    if (pol) {
    new = sp_alloc(pgoff, pgoff_end, pol);
    if (!new) {
    return -ENOMEM;
    }
    }
    err = shared_policy_replace(sp, pgoff, pgoff_end, new);
    if (err && new) {
    sp_free(new);
    }
    return err;
    }
    EXPORT_SYMBOL_FOR_MODULES(mpol_set_shared_policy, "kvm");
// Free a backing policy store on inode delete.
#[no_mangle]
pub unsafe extern "C" fn mpol_free_shared_policy(sp: *mut shared_policy) {
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
    if (!sp.root.rb_node) {
    return;
    }
    write_lock(&sp.lock);
    next = rb_first(&sp.root);
    while (next) {
    n = rb_entry(next, sp_node, nd);
    next = rb_next(&n.nd);
    sp_delete(sp, n);
    }
    write_unlock(&sp.lock);
    }
    EXPORT_SYMBOL_FOR_MODULES(mpol_free_shared_policy, "kvm");

    static int __initdata numabalancing_override;
#[no_mangle]
unsafe extern "C" fn check_numabalancing_enable()  {
pub static mut numabalancing_default: bool = false;
    if (IS_ENABLED!(CONFIG_NUMA_BALANCING_DEFAULT_ENABLED)) {
    numabalancing_default = true;
    }
// Parsed by setup_numabalancing. override == 1 enables, -1 disables
    if (numabalancing_override) {
    set_numabalancing_state(numabalancing_override == 1);
    }
    if (num_online_nodes() > 1 && !numabalancing_override) {
    pr_info!("%s automatic NUMA balancing. Configure with numa_balancing= or the kernel.numa_balancing sysctl\n",
    numabalancing_default ? "Enabling" : "Disabling");
    set_numabalancing_state(numabalancing_default);
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_numabalancing(str: *mut c_char) -> c_int {
pub static mut ret: c_int = 0;
    if (!str) {
// goto;
    }
    if (!strcmp(str, "enable")) {
    numabalancing_override = 1;
    ret = 1;
    } else if (!strcmp(str, "disable")) {
    numabalancing_override = -1;
    ret = 1;
    }
// label;
    if (!ret) {
    pr_warn!("Unable to parse numa_balancing=\n");
    }
    return ret;
    }
    __setup!("numa_balancing=", setup_numabalancing);

#[no_mangle]
pub unsafe extern "C" fn check_numabalancing_enable()  {
    }

#[no_mangle]
pub unsafe extern "C" fn numa_policy_init()  {
    let mut interleave_nodes;
pub static mut largest: c_ulong = 0;
    int nid, prefer = 0;
    policy_cache = kmem_cache_create("numa_policy",
    sizeof!(mempolicy),
    0, SLAB_PANIC, core::ptr::null_mut());
    sn_cache = kmem_cache_create("shared_policy_node",
    sizeof!(sp_node),
    0, SLAB_PANIC, core::ptr::null_mut());
    for_each_node(nid) {
    preferred_node_policy[nid] = (mempolicy) {
    .refcnt = ATOMIC_INIT(1),
    .mode = MPOL_PREFERRED,
    .flags = MPOL_F_MOF | MPOL_F_MORON,
    .nodes = nodemask_of_node(nid),
    };
    }
//
// Set interleaving policy for system init. Interleaving is only
// enabled across suitably sized nodes (default is >= 16MB), or
// fall back to the largest node if they're all smaller.
//
    nodes_clear(interleave_nodes);
    for_each_node_state(nid, N_MEMORY) {
pub static mut total_pages: c_ulong = 0;
// Preserve the largest node
    if (largest < total_pages) {
    largest = total_pages;
    prefer = nid;
    }
// Interleave this node?
    if ((total_pages << PAGE_SHIFT) >= (16 << 20)) {
    node_set(nid, interleave_nodes);
    }
    }
// All too small, use the largest
    if (unlikely(nodes_empty(interleave_nodes))) {
    node_set(prefer, interleave_nodes);
    }
    if (do_set_mempolicy(MPOL_INTERLEAVE, 0, &interleave_nodes)) {
    pr_err!("%s: interleaving failed\n", __func__);
    }
    check_numabalancing_enable();
    }
// Reset policy of current process to default
#[no_mangle]
pub unsafe extern "C" fn numa_default_policy() {
    do_set_mempolicy(MPOL_DEFAULT, 0, core::ptr::null_mut());
    }
//
// Parse and format mempolicy from/to strings
//
    static const char * const policy_modes[] =
    {
    [MPOL_DEFAULT]    = "default",
    [MPOL_PREFERRED]  = "prefer",
    [MPOL_BIND]       = "bind",
    [MPOL_INTERLEAVE] = "interleave",
    [MPOL_WEIGHTED_INTERLEAVE] = "weighted interleave",
    [MPOL_LOCAL]      = "local",
    [MPOL_PREFERRED_MANY]  = "prefer (many)",
    };

//
// mpol_parse_str - parse string to mempolicy, for tmpfs mpol mount option.
// @str:  string containing mempolicy to parse
// @mpol:  pointer to struct mempolicy pointer, returned on success.
//
// Format of input:
// <mode>[=<flags>][:<nodelist>]
//
// Return: %0 on success, else %1
//
#[no_mangle]
pub unsafe extern "C" fn mpol_parse_str(str: *mut c_char, mpol: *mut mempolicy) -> c_int {
    let mut new = core::ptr::null_mut();
    let mut mode_flags = 0;
    let mut nodes;
    let mut nodelist = strchr(str, ':');
    let mut flags = strchr(str, '=');
pub static mut err: c_int = 0;
    if (flags) {
// flags++ = '\0';	// terminate mode string
    }
    if (nodelist) {
// NUL-terminate mode or flags string
// nodelist++ = '\0';
    if (nodelist_parse(nodelist, nodes)) {
// goto;
    }
    if (!nodes_subset(nodes, node_states[N_MEMORY])) {
// goto;
    }
    } else {
    nodes_clear(nodes);
    }
    mode = match_string(policy_modes, MPOL_MAX, str);
    if (mode < 0) {
// goto;
    }
    match (mode) {
    MPOL_PREFERRED => {
//
// Insist on a nodelist of one node only, although later
// we use first_node(nodes) to grab a single node, so here
// nodelist (or nodes) cannot be empty.
//
    if (nodelist) {
    let mut rest = nodelist;
    while (isdigit(*rest)) {
    rest += 1;
    }
    if (*rest) {
// goto;
    }
    if (nodes_empty(nodes)) {
// goto;
    }
    }
    // break;
    }
    MPOL_INTERLEAVE => {
    }
    MPOL_WEIGHTED_INTERLEAVE => {
//
// Default to online nodes with memory if no nodelist
//
    if (!nodelist) {
    nodes = node_states[N_MEMORY];
    }
    // break;
    }
    MPOL_LOCAL => {
//
// Don't allow a nodelist;  mpol_new() checks flags
//
    if (nodelist) {
// goto;
    }
    // break;
    }
    MPOL_DEFAULT => {
//
// Insist on a empty nodelist
//
    if (!nodelist) {
    err = 0;
    }
// goto;
    }
    MPOL_PREFERRED_MANY => {
    }
    MPOL_BIND => {
//
// Insist on a nodelist
//
    if (!nodelist) {
// goto;
    }
    }
    }
    mode_flags = 0;
    if (flags) {
//
// Currently, we only support two mutually exclusive
// mode flags.
//
    if (!strcmp(flags, "static")) {
    mode_flags |= MPOL_F_STATIC_NODES;
    }

    else if (!strcmp(flags, "relative")) {
    mode_flags |= MPOL_F_RELATIVE_NODES;
    }
    else {
// goto;
    }
    }
    new = mpol_new(mode, mode_flags, &nodes);
    if (IS_ERR(new)) {
// goto;
    }
//
// Save nodes for mpol_to_str() to show the tmpfs mount options
// for /proc/mounts, /proc/pid/mounts and /proc/pid/mountinfo.
//
    if (mode != MPOL_PREFERRED) {
    new.nodes = nodes;
    } else if (nodelist) {
    nodes_clear(new.nodes);
    node_set(first_node(nodes), new.nodes);
    } else {
    new.mode = MPOL_LOCAL;
    }
//
// Save nodes for contextualization: this will be used to "clone"
// the mempolicy in a specific context [cpuset] at a later time.
//
    new.w.user_nodemask = nodes;
    err = 0;
// label;
// Restore string for error message
    if (nodelist) {
// --nodelist = ':';
    }
    if (flags) {
// --flags = '=';
    }
    if (!err) {
// mpol = new;
    }
    return err;
    }

//
// mpol_to_str - format a mempolicy structure for printing
// @buffer:  to contain formatted mempolicy string
// @maxlen:  length of @buffer
// @pol:  pointer to mempolicy to be formatted
//
// Convert @pol into a string.  If @buffer is too short, truncate the string.
// Recommend a @maxlen of at least 51 for the longest mode, "weighted
// interleave", plus the longest flag flags, "relative|balancing", and to
// display at least a few node ids.
//
#[no_mangle]
pub unsafe extern "C" fn mpol_to_str(buffer: *mut c_char, maxlen: c_int, pol: *mut mempolicy) {
    let mut p = buffer;
pub static mut nodes: nodemask_t = 0;
pub static mut mode: c_ushort = 0;
pub static mut flags: c_ushort = 0;
    if (pol &&
    pol != &default_policy &&
    !(pol >= &preferred_node_policy[0] &&
    pol <= &preferred_node_policy[ARRAY_SIZE!(preferred_node_policy) - 1])) {
    mode = pol.mode;
    flags = pol.flags;
    }
    match (mode) {
    MPOL_DEFAULT => {
    }
    MPOL_LOCAL => {
    // break;
    }
    MPOL_PREFERRED => {
    }
    MPOL_PREFERRED_MANY => {
    }
    MPOL_BIND => {
    }
    MPOL_INTERLEAVE => {
    }
    MPOL_WEIGHTED_INTERLEAVE => {
    nodes = pol.nodes;
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    snprintf(p, maxlen, "unknown");
    return;
    }
    }
    p += snprintf(p, maxlen, "%s", policy_modes[mode]);
    if (flags & MPOL_MODE_FLAGS) {
    p += snprintf(p, buffer + maxlen - p, "=");
//
// Static and relative are mutually exclusive.
//
    if (flags & MPOL_F_STATIC_NODES) {
    p += snprintf(p, buffer + maxlen - p, "static");
    }

    else if (flags & MPOL_F_RELATIVE_NODES) {
    p += snprintf(p, buffer + maxlen - p, "relative");
    }
    if (flags & MPOL_F_NUMA_BALANCING) {
    if (!is_power_of_2(flags & MPOL_MODE_FLAGS)) {
    p += snprintf(p, buffer + maxlen - p, "|");
    }
    p += snprintf(p, buffer + maxlen - p, "balancing");
    }
    }
    if (!nodes_empty(nodes)) {
    p += scnprintf(p, buffer + maxlen - p, ":%*pbl",
    nodemask_pr_args(&nodes));
    }
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_node_attr {
    pub kobj_attr: kobj_attribute,
    pub nid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysfs_wi_group {
    pub wi_kobj: kobject,
    pub kobj_lock: mutex,
    pub nattrs: [*mut iw_node_attr; ],
}

pub static mut wi_group: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn node_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut node_attr: *mut c_void = core::ptr::null_mut();
    let mut weight = 0;
    node_attr = container_of!(attr, iw_node_attr, kobj_attr);
    weight = get_il_weight(node_attr.nid);
    return sysfs_emit(buf, "%d\n", weight);
    }
#[no_mangle]
pub unsafe extern "C" fn node_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    struct weighted_interleave_state *new_wi_state, *old_wi_state = core::ptr::null_mut();
pub static mut node_attr: *mut c_void = core::ptr::null_mut();
pub static mut weight: u8 = 0;
    let mut i = 0;
    node_attr = container_of!(attr, iw_node_attr, kobj_attr);
    if (count == 0 || sysfs_streq(buf, "") ||
    kstrtou8(buf, 0, &weight) || weight == 0) {
    return -EINVAL;
    }
    new_wi_state = kzalloc_flex(*new_wi_state, iw_table, nr_node_ids);
    if (!new_wi_state) {
    return -ENOMEM;
    }
    mutex_lock(&wi_state_lock);
    old_wi_state = rcu_dereference_protected(wi_state,
    lockdep_is_held(&wi_state_lock));
    if (old_wi_state) {
    memcpy(new_wi_state.iw_table, old_wi_state.iw_table,
    nr_node_ids * sizeof!(u8));
    } else {
    for (i = 0; i < nr_node_ids; i++) {
    new_wi_state.iw_table[i] = 1;
    }
    }
    new_wi_state.iw_table[node_attr.nid] = weight;
    new_wi_state.mode_auto = false;
    rcu_assign_pointer(wi_state, new_wi_state);
    mutex_unlock(&wi_state_lock);
    if (old_wi_state) {
    synchronize_rcu();
    kfree(old_wi_state);
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn weighted_interleave_auto_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut state: *mut c_void = core::ptr::null_mut();
pub static mut wi_auto: bool = true;
    rcu_read_lock();
    state = rcu_dereference(wi_state);
    if (state) {
    wi_auto = state.mode_auto;
    }
    rcu_read_unlock();
    return sysfs_emit(buf, "%s\n", str_true_false(wi_auto));
    }
#[no_mangle]
pub unsafe extern "C" fn weighted_interleave_auto_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    struct weighted_interleave_state *new_wi_state, *old_wi_state = core::ptr::null_mut();
pub static mut bw: *mut c_void = core::ptr::null_mut();
    let mut input = 0;
    let mut i = 0;
    if (kstrtobool(buf, &input)) {
    return -EINVAL;
    }
    new_wi_state = kzalloc_flex(*new_wi_state, iw_table, nr_node_ids);
    if (!new_wi_state) {
    return -ENOMEM;
    }
    for (i = 0; i < nr_node_ids; i++) {
    new_wi_state.iw_table[i] = 1;
    }
    mutex_lock(&wi_state_lock);
    old_wi_state = rcu_dereference_protected(wi_state,
    lockdep_is_held(&wi_state_lock));
    if (old_wi_state && input == old_wi_state.mode_auto) {
    mutex_unlock(&wi_state_lock);
    kfree(new_wi_state);
    return count;
    }
    if (!input) {
    if (old_wi_state) {
    memcpy(new_wi_state.iw_table, old_wi_state.iw_table,
    nr_node_ids * sizeof!(u8));
    }
// goto;
    }
    bw = node_bw_table;
    if (!bw) {
    mutex_unlock(&wi_state_lock);
    kfree(new_wi_state);
    return -ENODEV;
    }
    new_wi_state.mode_auto = true;
    reduce_interleave_weights(bw, new_wi_state.iw_table);
// label;
    rcu_assign_pointer(wi_state, new_wi_state);
    mutex_unlock(&wi_state_lock);
    if (old_wi_state) {
    synchronize_rcu();
    kfree(old_wi_state);
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn sysfs_wi_node_delete(nid: c_int) {
pub static mut attr: *mut c_void = core::ptr::null_mut();
    if (nid < 0 || nid >= nr_node_ids) {
    return;
    }
    mutex_lock(&wi_group.kobj_lock);
    attr = wi_group.nattrs[nid];
    if (!attr) {
    mutex_unlock(&wi_group.kobj_lock);
    return;
    }
    wi_group.nattrs[nid] = core::ptr::null_mut();
    mutex_unlock(&wi_group.kobj_lock);
    sysfs_remove_file(&wi_group.wi_kobj, &attr.kobj_attr.attr);
    kfree(attr.kobj_attr.attr.name);
    kfree(attr);
    }
#[no_mangle]
unsafe extern "C" fn sysfs_wi_node_delete_all() {
    let mut nid = 0;
    for (nid = 0; nid < nr_node_ids; nid++) {
    sysfs_wi_node_delete(nid);
    }
    }
#[no_mangle]
unsafe extern "C" fn wi_state_free() {
pub static mut old_wi_state: *mut c_void = core::ptr::null_mut();
    mutex_lock(&wi_state_lock);
    old_wi_state = rcu_dereference_protected(wi_state,
    lockdep_is_held(&wi_state_lock));
    rcu_assign_pointer(wi_state, core::ptr::null_mut());
    mutex_unlock(&wi_state_lock);
    if (old_wi_state) {
    synchronize_rcu();
    kfree(old_wi_state);
    }
    }
pub static mut kobj_attribute: usize = 0;
#[no_mangle]
unsafe extern "C" fn wi_cleanup() {
    sysfs_remove_file(&wi_group.wi_kobj, &wi_auto_attr.attr);
    sysfs_wi_node_delete_all();
    wi_state_free();
    }
#[no_mangle]
unsafe extern "C" fn wi_kobj_release(wi_kobj: *mut kobject) {
    kfree(wi_group);
    }
pub static mut kobj_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn sysfs_wi_node_add(nid: c_int) -> c_int {
    let mut ret = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
pub static mut new_attr: *mut c_void = core::ptr::null_mut();
    if (nid < 0 || nid >= nr_node_ids) {
    pr_err!("invalid node id: %d\n", nid);
    return -EINVAL;
    }
    new_attr = kzalloc_obj(*new_attr);
    if (!new_attr) {
    return -ENOMEM;
    }
    name = kasprintf(GFP_KERNEL, "node%d", nid);
    if (!name) {
    kfree(new_attr);
    return -ENOMEM;
    }
    sysfs_attr_init(&new_attr.kobj_attr.attr);
    new_attr.kobj_attr.attr.name = name;
    new_attr.kobj_attr.attr.mode = 0644;
    new_attr.kobj_attr.show = node_show;
    new_attr.kobj_attr.store = node_store;
    new_attr.nid = nid;
    mutex_lock(&wi_group.kobj_lock);
    if (wi_group.nattrs[nid]) {
    mutex_unlock(&wi_group.kobj_lock);
    ret = -EEXIST;
// goto;
    }
    ret = sysfs_create_file(&wi_group.wi_kobj, &new_attr.kobj_attr.attr);
    if (ret) {
    mutex_unlock(&wi_group.kobj_lock);
// goto;
    }
    wi_group.nattrs[nid] = new_attr;
    mutex_unlock(&wi_group.kobj_lock);
    return 0;
// label;
    kfree(new_attr.kobj_attr.attr.name);
    kfree(new_attr);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn wi_node_notifier(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    let mut err = 0;
    let mut nn = data;
pub static mut nid: c_int = 0;
    match (action) {
    NODE_ADDED_FIRST_MEMORY => {
    err = sysfs_wi_node_add(nid);
    if (err) {
    pr_err!("failed to add sysfs for node%d during hotplug: %d\n",
    nid, err);
    }
    // break;
    }
    NODE_REMOVED_LAST_MEMORY => {
    sysfs_wi_node_delete(nid);
    // break;
    }
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn add_weighted_interleave_group(mempolicy_kobj: *mut kobject) -> c_int {
    let mut nid = 0;
    let mut err = 0;
    wi_group = kzalloc_flex(*wi_group, nattrs, nr_node_ids);
    if (!wi_group) {
    return -ENOMEM;
    }
    mutex_init(&wi_group.kobj_lock);
    err = kobject_init_and_add(&wi_group.wi_kobj, &wi_ktype, mempolicy_kobj,
    "weighted_interleave");
    if (err) {
// goto;
    }
    err = sysfs_create_file(&wi_group.wi_kobj, &wi_auto_attr.attr);
    if (err) {
// goto;
    }
    for_each_online_node(nid) {
    if (!node_state(nid, N_MEMORY)) {
    continue;
    }
    err = sysfs_wi_node_add(nid);
    if (err) {
    pr_err!("failed to add sysfs for node%d during init: %d\n",
    nid, err);
// goto;
    }
    }
    hotplug_node_notifier(wi_node_notifier, DEFAULT_CALLBACK_PRI);
    return 0;
// label;
    wi_cleanup();
    kobject_del(&wi_group.wi_kobj);
// label;
    kobject_put(&wi_group.wi_kobj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mempolicy_sysfs_init() -> c_int {
    let mut err = 0;
pub static mut mempolicy_kobj: *mut c_void = core::ptr::null_mut();
    mempolicy_kobj = kobject_create_and_add("mempolicy", mm_kobj);
    if (!mempolicy_kobj) {
    return -ENOMEM;
    }
    err = add_weighted_interleave_group(mempolicy_kobj);
    if (err) {
// goto;
    }
    return 0;
// label;
    kobject_del(mempolicy_kobj);
    kobject_put(mempolicy_kobj);
    return err;
    }
    late_initcall!(mempolicy_sysfs_init);
}
