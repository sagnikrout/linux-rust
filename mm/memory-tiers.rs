//! Automatically rewritten from C to Rust
//! Source: mm/memory-tiers.c
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_tier {
// hierarchy of memory tiers
    pub list: list_head,
// list of all memory types part of this tier
    pub memory_types: list_head,
//
// start value of abstract distance. memory tier maps
// an abstract distance  range,
// adistance_start .. adistance_start + MEMTIER_CHUNK_SIZE
//
    pub adistance_start: c_int,
    pub dev: device,
// All the nodes that are part of all the lower memory tiers.
    pub lower_tier_mask: nodemask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct demotion_nodes {
    pub preferred: nodemask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_memory_type_map {
    pub memtype: *mut memory_dev_type,
    pub map_count: c_int,
}

pub static mut memory_tier_lock: usize = 0;
pub static mut memory_tiers: usize = 0;
//
// The list is used to store all memory types that are not created
// by a device driver.
//
pub static mut default_memory_types: usize = 0;
    static struct node_memory_type_map node_memory_types[MAX_NUMNODES];
pub static mut default_dram_type: *mut c_void = core::ptr::null_mut();
pub static mut __initdata: nodemask_t default_dram_nodes = 0;
pub static mut bus_type: usize = 0;

//
// folio_use_access_time - check if a folio reuses cpupid for page access time
// @folio: folio to check
//
// folio's _last_cpupid field is repurposed by memory tiering. In memory
// tiering mode, cpupid of slow memory folio (not toptier memory) is used to
// record page access time.
//
// Return: the folio _last_cpupid is used to record page access time
//
#[no_mangle]
pub unsafe extern "C" fn folio_use_access_time(folio: *mut folio) -> bool {
    return (sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING) &&
    !node_is_toptier(folio_nid(folio));
    }

    static int top_tier_adistance;
//
// node_demotion[] examples:
//
// Example 1:
//
// Node 0 & 1 are CPU + DRAM nodes, node 2 & 3 are PMEM nodes.
//
// node distances:
// node   0    1    2    3
// 0  10   20   30   40
// 1  20   10   40   30
// 2  30   40   10   40
// 3  40   30   40   10
//
// memory_tiers0 = 0-1
// memory_tiers1 = 2-3
//
// node_demotion[0].preferred = 2
// node_demotion[1].preferred = 3
// node_demotion[2].preferred = <empty>
// node_demotion[3].preferred = <empty>
//
// Example 2:
//
// Node 0 & 1 are CPU + DRAM nodes, node 2 is memory-only DRAM node.
//
// node distances:
// node   0    1    2
// 0  10   20   30
// 1  20   10   30
// 2  30   30   10
//
// memory_tiers0 = 0-2
//
// node_demotion[0].preferred = <empty>
// node_demotion[1].preferred = <empty>
// node_demotion[2].preferred = <empty>
//
// Example 3:
//
// Node 0 is CPU + DRAM nodes, Node 1 is HBM node, node 2 is PMEM node.
//
// node distances:
// node   0    1    2
// 0  10   20   30
// 1  20   10   40
// 2  30   40   10
//
// memory_tiers0 = 1
// memory_tiers1 = 0
// memory_tiers2 = 2
//
// node_demotion[0].preferred = 2
// node_demotion[1].preferred = 0
// node_demotion[2].preferred = <empty>
//
pub static mut node_demotion: *mut c_void = core::ptr::null_mut();

    static BLOCKING_NOTIFIER_HEAD(mt_adistance_algorithms);
// The lock is used to protect `default_dram_perf*` info and nid.
pub static mut default_dram_perf_lock: usize = 0;
    static bool default_dram_perf_error;
pub static mut default_dram_perf: usize = 0;
pub static mut default_dram_perf_ref_nid: int = 0;
pub static mut default_dram_perf_ref_source: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn to_memory_tier(device: *mut device) -> *mut c_void {
    return container_of!(device, memory_tier, dev);
    }
#[no_mangle]
unsafe extern "C" fn get_memtier_nodemask(memtier: *mut memory_tier) -> __always_inline nodemask_t {
pub static mut nodes: nodemask_t = 0;
pub static mut memtype: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(memtype, &memtier.memory_types, tier_sibling) {
    nodes_or(nodes, nodes, memtype.nodes);
    }
    return nodes;
    }
#[no_mangle]
unsafe extern "C" fn memory_tier_device_release(dev: *mut device) {
    let mut tier = to_memory_tier(dev);
//
// synchronize_rcu in clear_node_memory_tier makes sure
// we don't have rcu access to this memory tier.
//
    kfree(tier);
    }
#[no_mangle]
pub unsafe extern "C" fn nodelist_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut ret = 0;
    let mut nmask;
    mutex_lock(&memory_tier_lock);
    nmask = get_memtier_nodemask(to_memory_tier(dev));
    ret = sysfs_emit(buf, "%*pbl\n", nodemask_pr_args(&nmask));
    mutex_unlock(&memory_tier_lock);
    return ret;
    }
    static DEVICE_ATTR_RO(nodelist);
    static struct attribute *memtier_dev_attrs[] = {
    &dev_attr_nodelist.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
    static const struct attribute_group *memtier_dev_groups[] = {
    &memtier_dev_group,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn find_create_memory_tier(memtype: *mut memory_dev_type) -> *mut c_void {
    let mut ret = 0;
pub static mut found_slot: bool = false;
    let mut memtier = core::ptr::null_mut();
    let mut new_memtier = core::ptr::null_mut();
pub static mut adistance: c_int = 0;
pub static mut memtier_adistance_chunk_size: c_uint = 0;
    lockdep_assert_held_once(&memory_tier_lock);
    adistance = round_down(adistance, memtier_adistance_chunk_size);
//
// If the memtype is already part of a memory tier,
// just return that.
//
    if (!list_empty(&memtype.tier_sibling)) {
    list_for_each_entry(memtier, &memory_tiers, list) {
    if (adistance == memtier.adistance_start) {
    return memtier;
    }
    }
    WARN_ON!(1);
    return ERR_PTR(-EINVAL);
    }
    list_for_each_entry(memtier, &memory_tiers, list) {
    if (adistance == memtier.adistance_start) {
// goto;
    } else if (adistance < memtier.adistance_start) {
    found_slot = true;
    break;
    }
    }
    new_memtier = kzalloc_obj(memory_tier);
    if (!new_memtier) {
    return ERR_PTR(-ENOMEM);
    }
    new_memtier.adistance_start = adistance;
    INIT_LIST_HEAD(&new_memtier.list);
    INIT_LIST_HEAD(&new_memtier.memory_types);
    if (found_slot) {
    list_add_tail(&new_memtier.list, &memtier.list);
    }
    else {
    list_add_tail(&new_memtier.list, &memory_tiers);
    }
    new_memtier.dev.id = adistance >> MEMTIER_CHUNK_BITS;
    new_memtier.dev.bus = &memory_tier_subsys;
    new_memtier.dev.release = memory_tier_device_release;
    new_memtier.dev.groups = memtier_dev_groups;
    ret = device_register(&new_memtier.dev);
    if (ret) {
    list_del(&new_memtier.list);
    put_device(&new_memtier.dev);
    return ERR_PTR(ret);
    }
    memtier = new_memtier;
// label;
    list_add(&memtype.tier_sibling, &memtier.memory_types);
    return memtier;
    }
#[no_mangle]
pub unsafe extern "C" fn __node_get_memory_tier(node: c_int) -> *mut c_void {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    pgdat = NODE_DATA(node);
    if (!pgdat) {
    return core::ptr::null_mut();
    }
//
// Since we hold memory_tier_lock, we can avoid
// RCU read locks when accessing the details. No
// parallel updates are possible here.
//
    return rcu_dereference_check(pgdat.memtier,
    lockdep_is_held(&memory_tier_lock));
    }

#[no_mangle]
pub unsafe extern "C" fn node_is_toptier(node: c_int) -> bool {
    let mut toptier = 0;
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut memtier: *mut c_void = core::ptr::null_mut();
    pgdat = NODE_DATA(node);
    if (!pgdat) {
    return false;
    }
    rcu_read_lock();
    memtier = rcu_dereference(pgdat.memtier);
    if (!memtier) {
    toptier = true;
// goto;
    }
    if (memtier.adistance_start <= top_tier_adistance) {
    toptier = true;
    }
    else {
    toptier = false;
    }
// label;
    rcu_read_unlock();
    return toptier;
    }
#[no_mangle]
pub unsafe extern "C" fn node_get_allowed_targets(pgdat: *mut pg_data_t, targets: *mut nodemask_t) {
pub static mut memtier: *mut c_void = core::ptr::null_mut();
//
// pg_data_t.memtier updates includes a synchronize_rcu()
// which ensures that we either find NULL or a valid memtier
// in NODE_DATA. protect the access via rcu_read_lock();
//
    rcu_read_lock();
    memtier = rcu_dereference(pgdat.memtier);
    if (memtier) {
// targets = memtier->lower_tier_mask;
    }
    else {
// targets = NODE_MASK_NONE;
    }
    rcu_read_unlock();
    }
//
// next_demotion_node() - Get the next node in the demotion path
// @node: The starting node to lookup the next node
// @allowed_mask: The pointer to allowed node mask
//
// Return: node id for next memory node in the demotion path hierarchy
// from @node; NUMA_NO_NODE if @node is terminal.  This does not keep
// @node online or guarantee that it *continues* to be the next demotion
// target.
//
#[no_mangle]
pub unsafe extern "C" fn next_demotion_node(node: c_int, allowed_mask: *const nodemask_t) -> c_int {
pub static mut nd: *mut c_void = core::ptr::null_mut();
    let mut mask;
    if (!node_demotion) {
    return NUMA_NO_NODE;
    }
    nd = &node_demotion[node];
//
// node_demotion[] is updated without excluding this
// function from running.
//
// Make sure to use RCU over entire code blocks if
// node_demotion[] reads need to be consistent.
//
    rcu_read_lock();
// Filter out nodes that are not in allowed_mask.
    nodes_and(mask, nd.preferred, *allowed_mask);
    rcu_read_unlock();
//
// If there are multiple target nodes, just select one
// target node randomly.
//
// In addition, we can also use round-robin to select
// target node, but we should introduce another variable
// for node_demotion[] to record last selected target node,
// that may cause cache ping-pong due to the changing of
// last target node. Or introducing per-cpu data to avoid
// caching issue, which seems more complicated. So selecting
// target node randomly seems better until now.
//
    if (!nodes_empty(mask)) {
    return node_random(&mask);
    }
//
// Preferred nodes are not in allowed_mask. Flip bits in
// allowed_mask as used node mask. Then, use it to get the
// closest demotion target.
//
    nodes_complement(mask, *allowed_mask);
    return find_next_best_node(node, &mask);
    }
#[no_mangle]
unsafe extern "C" fn disable_all_demotion_targets() {
pub static mut memtier: *mut c_void = core::ptr::null_mut();
    let mut node = 0;
    for_each_node_state(node, N_MEMORY) {
    node_demotion[node].preferred = NODE_MASK_NONE;
//
// We are holding memory_tier_lock, it is safe
// to access pgda->memtier.
//
    memtier = __node_get_memory_tier(node);
    if (memtier) {
    memtier.lower_tier_mask = NODE_MASK_NONE;
    }
    }
//
// Ensure that the "disable" is visible across the system.
// Readers will see either a combination of before+disable
// state or disable+after.  They will never see before and
// after state together.
//
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn dump_demotion_targets() {
    let mut node = 0;
    for_each_node_state(node, N_MEMORY) {
    let mut memtier = __node_get_memory_tier(node);
pub static mut preferred: nodemask_t = 0;
    if (!memtier) {
    continue;
    }
    if (nodes_empty(preferred)) {
    pr_info!("Demotion targets for Node %d: null\n", node);
    }
    else {
    pr_info!("Demotion targets for Node %d: preferred: %*pbl, fallback: %*pbl\n",
    node, nodemask_pr_args(&preferred),
    nodemask_pr_args(&memtier.lower_tier_mask));
    }
    }
    }
//
// Find an automatic demotion target for all memory
// nodes. Failing here is OK.  It might just indicate
// being at the end of a chain.
//
#[no_mangle]
unsafe extern "C" fn establish_demotion_targets() {
pub static mut memtier: *mut c_void = core::ptr::null_mut();
pub static mut nd: *mut c_void = core::ptr::null_mut();
pub static mut target: c_int = 0;
    let mut distance = 0;
    let mut best_distance = 0;
    nodemask_t tier_nodes, lower_tier;
    lockdep_assert_held_once(&memory_tier_lock);
    if (!node_demotion) {
    return;
    }
    disable_all_demotion_targets();
    for_each_node_state(node, N_MEMORY) {
    best_distance = -1;
    nd = &node_demotion[node];
    memtier = __node_get_memory_tier(node);
    if (!memtier || list_is_last(&memtier.list, &memory_tiers)) {
    continue;
    }
//
// Get the lower memtier to find the  demotion node list.
//
    memtier = list_next_entry(memtier, list);
    tier_nodes = get_memtier_nodemask(memtier);
//
// find_next_best_node, use 'used' nodemask as a skip list.
// Add all memory nodes except the selected memory tier
// nodelist to skip list so that we find the best node from the
// memtier nodelist.
//
    nodes_andnot(tier_nodes, node_states[N_MEMORY], tier_nodes);
//
// Find all the nodes in the memory tier node list of same best distance.
// add them to the preferred mask. We randomly select between nodes
// in the preferred mask when allocating pages during demotion.
//
    do {
    target = find_next_best_node(node, &tier_nodes);
    if (target == NUMA_NO_NODE) {
    break;
    }
    distance = node_distance(node, target);
    if (distance == best_distance || best_distance == -1) {
    best_distance = distance;
    node_set(target, nd.preferred);
    } else {
    break;
    }
    } while (1);
    }
//
// Promotion is allowed from a memory tier to higher
// memory tier only if the memory tier doesn't include
// compute. We want to skip promotion from a memory tier,
// if any node that is part of the memory tier have CPUs.
// Once we detect such a memory tier, we consider that tier
// as top tiper from which promotion is not allowed.
//
    list_for_each_entry_reverse(memtier, &memory_tiers, list) {
    tier_nodes = get_memtier_nodemask(memtier);
    if (nodes_and(tier_nodes, node_states[N_CPU], tier_nodes)) {
//
// abstract distance below the max value of this memtier
// is considered toptier.
//
    top_tier_adistance = memtier.adistance_start +
    MEMTIER_CHUNK_SIZE - 1;
    break;
    }
    }
//
// Now build the lower_tier mask for each node collecting node mask from
// all memory tier below it. This allows us to fallback demotion page
// allocation to a set of nodes that is closer the above selected
// preferred node.
//
    lower_tier = node_states[N_MEMORY];
    list_for_each_entry(memtier, &memory_tiers, list) {
//
// Keep removing current tier from lower_tier nodes,
// This will remove all nodes in current and above
// memory tier from the lower_tier mask.
//
    tier_nodes = get_memtier_nodemask(memtier);
    nodes_andnot(lower_tier, lower_tier, tier_nodes);
    memtier.lower_tier_mask = lower_tier;
    }
    dump_demotion_targets();
    }

#[no_mangle]
pub unsafe extern "C" fn establish_demotion_targets() {}

#[no_mangle]
pub unsafe extern "C" fn __init_node_memory_type(node: c_int, memtype: *mut memory_dev_type) {
    if (!node_memory_types[node].memtype) {
    node_memory_types[node].memtype = memtype;
    }
//
// for each device getting added in the same NUMA node
// with this specific memtype, bump the map count. We
// Only take memtype device reference once, so that
// changing a node memtype can be done by dropping the
// only reference count taken here.
//
    if (node_memory_types[node].memtype == memtype) {
    if (!node_memory_types[node].map_count++) {
    kref_get(&memtype.kref);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_node_memory_tier(node: c_int) -> *mut c_void {
pub static mut memtier: *mut c_void = core::ptr::null_mut();
    let mut memtype = default_dram_type;
pub static mut adist: c_int = 0;
    let mut pgdat = NODE_DATA(node);
    lockdep_assert_held_once(&memory_tier_lock);
    if (!node_state(node, N_MEMORY)) {
    return ERR_PTR(-EINVAL);
    }
    mt_calc_adistance(node, &adist);
    if (!node_memory_types[node].memtype) {
    memtype = mt_find_alloc_memory_type(adist, &default_memory_types);
    if (IS_ERR(memtype)) {
    memtype = default_dram_type;
    pr_info!("Failed to allocate a memory type. Fall back.\n");
    }
    }
    __init_node_memory_type(node, memtype);
    memtype = node_memory_types[node].memtype;
    node_set(node, memtype.nodes);
    memtier = find_create_memory_tier(memtype);
    if (!IS_ERR(memtier)) {
    rcu_assign_pointer(pgdat.memtier, memtier);
    }
    return memtier;
    }
#[no_mangle]
unsafe extern "C" fn destroy_memory_tier(memtier: *mut memory_tier) {
    list_del(&memtier.list);
    device_unregister(&memtier.dev);
    }
#[no_mangle]
unsafe extern "C" fn clear_node_memory_tier(node: c_int) -> bool {
pub static mut cleared: bool = false;
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut memtier: *mut c_void = core::ptr::null_mut();
    pgdat = NODE_DATA(node);
    if (!pgdat) {
    return false;
    }
//
// Make sure that anybody looking at NODE_DATA who finds
// a valid memtier finds memory_dev_types with nodes still
// linked to the memtier. We achieve this by waiting for
// rcu read section to finish using synchronize_rcu.
// This also enables us to free the destroyed memory tier
// with kfree instead of kfree_rcu
//
    memtier = __node_get_memory_tier(node);
    if (memtier) {
pub static mut memtype: *mut c_void = core::ptr::null_mut();
    rcu_assign_pointer(pgdat.memtier, core::ptr::null_mut());
    synchronize_rcu();
    memtype = node_memory_types[node].memtype;
    node_clear(node, memtype.nodes);
    if (nodes_empty(memtype.nodes)) {
    list_del_init(&memtype.tier_sibling);
    if (list_empty(&memtier.memory_types)) {
    destroy_memory_tier(memtier);
    }
    }
    cleared = true;
    }
    return cleared;
    }
#[no_mangle]
unsafe extern "C" fn release_memtype(kref: *mut kref) {
pub static mut memtype: *mut c_void = core::ptr::null_mut();
    memtype = container_of!(kref, memory_dev_type, kref);
    kfree(memtype);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_memory_type(adistance: c_int) -> *mut c_void {
pub static mut memtype: *mut c_void = core::ptr::null_mut();
    memtype = kmalloc_obj(*memtype);
    if (!memtype) {
    return ERR_PTR(-ENOMEM);
    }
    memtype.adistance = adistance;
    INIT_LIST_HEAD(&memtype.tier_sibling);
    memtype.nodes  = NODE_MASK_NONE;
    kref_init(&memtype.kref);
    return memtype;
    }
    EXPORT_SYMBOL_GPL(alloc_memory_type);
#[no_mangle]
pub unsafe extern "C" fn put_memory_type(memtype: *mut memory_dev_type) {
    kref_put(&memtype.kref, release_memtype);
    }
    EXPORT_SYMBOL_GPL(put_memory_type);
#[no_mangle]
pub unsafe extern "C" fn init_node_memory_type(node: c_int, memtype: *mut memory_dev_type) {
    mutex_lock(&memory_tier_lock);
    __init_node_memory_type(node, memtype);
    mutex_unlock(&memory_tier_lock);
    }
    EXPORT_SYMBOL_GPL(init_node_memory_type);
#[no_mangle]
pub unsafe extern "C" fn clear_node_memory_type(node: c_int, memtype: *mut memory_dev_type) {
    mutex_lock(&memory_tier_lock);
    if (node_memory_types[node].memtype == memtype || !memtype) {
    node_memory_types[node].map_count -= 1;
    }
//
// If we unmapped all the attached devices to this node,
// clear the node memory type.
//
    if (!node_memory_types[node].map_count) {
    memtype = node_memory_types[node].memtype;
    node_memory_types[node].memtype = core::ptr::null_mut();
    put_memory_type(memtype);
    }
    mutex_unlock(&memory_tier_lock);
    }
    EXPORT_SYMBOL_GPL(clear_node_memory_type);
#[no_mangle]
pub unsafe extern "C" fn mt_find_alloc_memory_type(adist: c_int, memory_types: *mut list_head) -> *mut c_void {
pub static mut mtype: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(mtype, memory_types, list) {
    if (mtype.adistance == adist)
    return mtype;
    }
    mtype = alloc_memory_type(adist);
    if (IS_ERR(mtype)) {
    return mtype;
    }
    list_add(&mtype.list, memory_types);
    return mtype;
    }
    EXPORT_SYMBOL_GPL(mt_find_alloc_memory_type);
#[no_mangle]
pub unsafe extern "C" fn mt_put_memory_types(memory_types: *mut list_head) {
    let mut mtype = core::ptr::null_mut();
    let mut mtn = core::ptr::null_mut();
    list_for_each_entry_safe(mtype, mtn, memory_types, list) {
    list_del(&mtype.list);
    put_memory_type(mtype);
    }
    }
    EXPORT_SYMBOL_GPL(mt_put_memory_types);
//
// This is invoked via `late_initcall!()` to initialize memory tiers for
// memory nodes, both with and without CPUs. After the initialization of
// firmware and devices, adistance algorithms are expected to be provided.
//
#[no_mangle]
unsafe extern "C" fn memory_tier_late_init() -> c_int {
    let mut nid = 0;
pub static mut memtier: *mut c_void = core::ptr::null_mut();
    get_online_mems();
    guard(mutex)(&memory_tier_lock);
// Assign each uninitialized N_MEMORY node to a memory tier.
    for_each_node_state(nid, N_MEMORY) {
//
// Some device drivers may have initialized
// memory tiers, potentially bringing memory nodes
// online and configuring memory tiers.
// Exclude them here.
//
    if (node_memory_types[nid].memtype) {
    continue;
    }
    memtier = set_node_memory_tier(nid);
    if (IS_ERR(memtier)) {
    continue;
    }
    }
    establish_demotion_targets();
    put_online_mems();
    return 0;
    }
    late_initcall!(memory_tier_late_init);
#[no_mangle]
unsafe extern "C" fn dump_hmem_attrs(coord: *mut access_coordinate, prefix: *const c_char) {
    pr_info!(
    "%sread_latency: %u, write_latency: %u, read_bandwidth: %u, write_bandwidth: %u\n",
    prefix, coord.read_latency, coord.write_latency,
    coord.read_bandwidth, coord.write_bandwidth);
    }
#[no_mangle]
pub unsafe extern "C" fn mt_set_default_dram_perf(nid: c_int, perf: *mut access_coordinate, source: *mut c_char) -> c_int {
    guard(mutex)(&default_dram_perf_lock);
    if (default_dram_perf_error) {
    return -EIO;
    }
    if (perf.read_latency + perf.write_latency == 0 ||
    perf.read_bandwidth + perf.write_bandwidth == 0) {
    return -EINVAL;
    }
    if (default_dram_perf_ref_nid == NUMA_NO_NODE) {
    default_dram_perf = *perf;
    default_dram_perf_ref_nid = nid;
    default_dram_perf_ref_source = kstrdup(source, GFP_KERNEL);
    return 0;
    }
//
// The performance of all default DRAM nodes is expected to be
// same (that is, the variation is less than 10%).  And it
// will be used as base to calculate the abstract distance of
// other memory nodes.
//
    if (abs(perf.read_latency - default_dram_perf.read_latency) * 10 >
    default_dram_perf.read_latency ||
    abs(perf.write_latency - default_dram_perf.write_latency) * 10 >
    default_dram_perf.write_latency ||
    abs(perf.read_bandwidth - default_dram_perf.read_bandwidth) * 10 >
    default_dram_perf.read_bandwidth ||
    abs(perf.write_bandwidth - default_dram_perf.write_bandwidth) * 10 >
    default_dram_perf.write_bandwidth) {
    pr_info!(
    "memory-tiers: the performance of DRAM node %d mismatches that of the reference\n"
    "DRAM node %d.\n", nid, default_dram_perf_ref_nid);
    pr_info!("  performance of reference DRAM node %d from %s:\n",
    default_dram_perf_ref_nid, default_dram_perf_ref_source);
    dump_hmem_attrs(&default_dram_perf, "    ");
    pr_info!("  performance of DRAM node %d from %s:\n", nid, source);
    dump_hmem_attrs(perf, "    ");
    pr_info!(
    "  disable default DRAM node performance based abstract distance algorithm.\n");
    default_dram_perf_error = true;
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt_perf_to_adistance(perf: *mut access_coordinate, adist: *mut c_int) -> c_int {
    guard(mutex)(&default_dram_perf_lock);
    if (default_dram_perf_error) {
    return -EIO;
    }
    if (perf.read_latency + perf.write_latency == 0 ||
    perf.read_bandwidth + perf.write_bandwidth == 0) {
    return -EINVAL;
    }
    if (default_dram_perf_ref_nid == NUMA_NO_NODE) {
    return -ENOENT;
    }
//
// The abstract distance of a memory node is in direct proportion to
// its memory latency (read + write) and inversely proportional to its
// memory bandwidth (read + write).  The abstract distance, memory
// latency, and memory bandwidth of the default DRAM nodes are used as
// the base.
//
// adist = MEMTIER_ADISTANCE_DRAM
    (perf.read_latency + perf.write_latency) /
    (default_dram_perf.read_latency + default_dram_perf.write_latency) *
    (default_dram_perf.read_bandwidth + default_dram_perf.write_bandwidth) /
    (perf.read_bandwidth + perf.write_bandwidth);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt_perf_to_adistance);
//
// register_mt_adistance_algorithm() - Register memory tiering abstract distance algorithm
// @nb: The notifier block which describe the algorithm
//
// Return: 0 on success, errno on error.
//
// Every memory tiering abstract distance algorithm provider needs to
// register the algorithm with register_mt_adistance_algorithm().  To
// calculate the abstract distance for a specified memory node, the
// notifier function will be called unless some high priority
// algorithm has provided result.  The prototype of the notifier
// function is as follows,
//
// int (*algorithm_notifier)(notifier_block *nb,
// unsigned long nid, void *data);
//
// Where "nid" specifies the memory node, "data" is the pointer to the
// returned abstract distance (that is, "int *adist").  If the
// algorithm provides the result, NOTIFY_STOP should be returned.
// Otherwise, return_value & %NOTIFY_STOP_MASK == 0 to allow the next
// algorithm in the chain to provide the result.
//
#[no_mangle]
pub unsafe extern "C" fn register_mt_adistance_algorithm(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_register(&mt_adistance_algorithms, nb);
    }
    EXPORT_SYMBOL_GPL(register_mt_adistance_algorithm);
//
// unregister_mt_adistance_algorithm() - Unregister memory tiering abstract distance algorithm
// @nb: the notifier block which describe the algorithm
//
// Return: 0 on success, errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_mt_adistance_algorithm(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_unregister(&mt_adistance_algorithms, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_mt_adistance_algorithm);
//
// mt_calc_adistance() - Calculate abstract distance with registered algorithms
// @node: the node to calculate abstract distance for
// @adist: the returned abstract distance
//
// Return: if return_value & %NOTIFY_STOP_MASK != 0, then some
// abstract distance algorithm provides the result, and return it via
// @adist.  Otherwise, no algorithm can provide the result and @adist
// will be kept as it is.
//
#[no_mangle]
pub unsafe extern "C" fn mt_calc_adistance(node: c_int, adist: *mut c_int) -> c_int {
    return blocking_notifier_call_chain(&mt_adistance_algorithms, node, adist);
    }
    EXPORT_SYMBOL_GPL(mt_calc_adistance);
    static int __meminit memtier_hotplug_callback(notifier_block *self,
    unsigned long action, void *_arg)
    {
pub static mut memtier: *mut c_void = core::ptr::null_mut();
    let mut nn = _arg;
    match (action) {
    NODE_REMOVED_LAST_MEMORY => {
    mutex_lock(&memory_tier_lock);
    if (clear_node_memory_tier(nn.nid)) {
    establish_demotion_targets();
    }
    mutex_unlock(&memory_tier_lock);
    // break;
    }
    NODE_ADDED_FIRST_MEMORY => {
    mutex_lock(&memory_tier_lock);
    memtier = set_node_memory_tier(nn.nid);
    if (!IS_ERR(memtier)) {
    establish_demotion_targets();
    }
    mutex_unlock(&memory_tier_lock);
    // break;
    }
    }
    return notifier_from_errno(0);
    }
#[no_mangle]
unsafe extern "C" fn memory_tier_init() -> c_int {
    let mut ret = 0;
    ret = subsys_virtual_register(&memory_tier_subsys, core::ptr::null_mut());
    if (ret) {
    panic("%s() failed to register memory tier subsystem\n", __func__);
    }

    node_demotion = kzalloc_objs(demotion_nodes, nr_node_ids);
    WARN_ON!(!node_demotion);

    mutex_lock(&memory_tier_lock);
//
// For now we can have 4 faster memory tiers with smaller adistance
// than default DRAM tier.
//
    default_dram_type = mt_find_alloc_memory_type(MEMTIER_ADISTANCE_DRAM,
    &default_memory_types);
    mutex_unlock(&memory_tier_lock);
    if (IS_ERR(default_dram_type)) {
    panic("%s() failed to allocate default DRAM tier\n", __func__);
    }
// Record nodes with memory and CPU to set default DRAM performance.
    nodes_and(default_dram_nodes, node_states[N_MEMORY],
    node_states[N_CPU]);
    hotplug_node_notifier(memtier_hotplug_callback, MEMTIER_HOTPLUG_PRI);
    return 0;
    }
    subsys_initcall!(memory_tier_init);
pub static mut numa_demotion_enabled: bool = false;

#[no_mangle]
pub unsafe extern "C" fn demotion_enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%s\n", str_true_false(numa_demotion_enabled));
    }
#[no_mangle]
pub unsafe extern "C" fn demotion_enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut ret = 0;
pub static mut before: bool = false;
    ret = kstrtobool(buf, &numa_demotion_enabled);
    if (ret) {
    return ret;
    }
//
// Reset kswapd_failures statistics. They may no longer be
// valid since the policy for kswapd has changed.
//
    if (before == false && numa_demotion_enabled == true) {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    for_each_online_pgdat(pgdat) {
    kswapd_clear_hopeless(pgdat, KSWAPD_CLEAR_HOPELESS_OTHER);
    }
    }
    return count;
    }
    static struct kobj_attribute numa_demotion_enabled_attr =
    __ATTR_RW(demotion_enabled);
    static struct attribute *numa_attrs[] = {
    &numa_demotion_enabled_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn numa_init_sysfs() -> c_int {
    let mut err = 0;
pub static mut numa_kobj: *mut c_void = core::ptr::null_mut();
    numa_kobj = kobject_create_and_add("numa", mm_kobj);
    if (!numa_kobj) {
    pr_err!("failed to create numa kobject\n");
    return -ENOMEM;
    }
    err = sysfs_create_group(numa_kobj, &numa_attr_group);
    if (err) {
    pr_err!("failed to register numa group\n");
// goto;
    }
    return 0;
// label;
    kobject_put(numa_kobj);
    return err;
    }
    subsys_initcall!(numa_init_sysfs);