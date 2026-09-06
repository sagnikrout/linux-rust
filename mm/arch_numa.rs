//! Automatically rewritten from C to Rust
//! Source: mm/arch_numa.c
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
// NUMA support, based on the x86 implementation.
//
// Copyright (C) 2015 Cavium Inc.
// Author: Ganapatrao Kulkarni <gkulkarni@cavium.com>
//

    static int cpu_to_node_map[NR_CPUS] = { [0 ... NR_CPUS-1] = NUMA_NO_NODE };
    let mut numa_off = 0;
#[no_mangle]
unsafe extern "C" fn numa_parse_early_param(opt: *mut c_char) -> __init int {
    if (!opt) {
    return -EINVAL;
    }
    if (str_has_prefix(opt, "off")) {
    numa_off = true;
    }
    if (!strncmp(opt, "fake=", 5)) {
    return numa_emu_cmdline(opt + 5);
    }
    return 0;
    }
    early_param!("numa", numa_parse_early_param);
    cpumask_var_t node_to_cpumask_map[MAX_NUMNODES];
    EXPORT_SYMBOL(node_to_cpumask_map);

//
// Returns a pointer to the bitmask of CPUs on Node 'node'.
//
    const struct cpumask *cpumask_of_node(int node)
    {
    if (node == NUMA_NO_NODE) {
    return cpu_all_mask;
    }
    if (WARN_ON!(node < 0 || node >= nr_node_ids)) {
    return cpu_none_mask;
    }
    if (WARN_ON!(node_to_cpumask_map[node] == core::ptr::null_mut())) {
    return cpu_online_mask;
    }
    return node_to_cpumask_map[node];
    }
    EXPORT_SYMBOL(cpumask_of_node);

#[no_mangle]
unsafe extern "C" fn numa_update_cpu(cpu: c_uint, remove: bool) {
pub static mut nid: c_int = 0;
    if (nid == NUMA_NO_NODE) {
    return;
    }
    if (remove) {
    cpumask_clear_cpu(cpu, node_to_cpumask_map[nid]);
    }
    else {
    cpumask_set_cpu(cpu, node_to_cpumask_map[nid]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn numa_add_cpu(cpu: c_uint) {
    numa_update_cpu(cpu, false);
    }
#[no_mangle]
pub unsafe extern "C" fn numa_remove_cpu(cpu: c_uint) {
    numa_update_cpu(cpu, true);
    }

#[no_mangle]
pub unsafe extern "C" fn numa_clear_node(cpu: c_uint) {
    numa_remove_cpu(cpu);
    set_cpu_numa_node(cpu, NUMA_NO_NODE);
    }
//
// Allocate node_to_cpumask_map based on number of available nodes
// Requires node_possible_map to be valid.
//
// Note: cpumask_of_node() is not valid until after this is done.
// (Use CONFIG_DEBUG_PER_CPU_MAPS to check this.)
//
#[no_mangle]
unsafe extern "C" fn setup_node_to_cpumask_map()  {
    let mut node = 0;
// setup nr_node_ids if not done yet
    if (nr_node_ids == MAX_NUMNODES) {
    setup_nr_node_ids();
    }
//
// This check should never be true but it makes it clear to compilers
// that node_to_cpumask_map is bound by nr_node_ids, avoiding false
// positive fortify warnings when accessing node_to_cpumask_map in the
// for loop below.
//
    if (unlikely(nr_node_ids > MAX_NUMNODES)) {
    pr_err!("nr_node_ids (%u) is larger than MAX_NUMNODES (%u)\n",
    nr_node_ids, MAX_NUMNODES);
    return;
    }
// allocate and clear the mapping
    while (node < nr_node_ids) {
    alloc_bootmem_cpumask_var(&node_to_cpumask_map[node]);
    cpumask_clear(node_to_cpumask_map[node]);
    }
// cpumask_of_node() will now work
    pr_debug!("Node to cpumask map for %u nodes\n", nr_node_ids);
    }
//
// Set the cpu to node and mem mapping
//
#[no_mangle]
pub unsafe extern "C" fn numa_store_cpu_info(cpu: c_uint) {
    set_cpu_numa_node(cpu, cpu_to_node_map[cpu]);
    }
#[no_mangle]
pub unsafe extern "C" fn early_map_cpu_to_node(cpu: c_uint, nid: c_int)  {
// fallback to node 0
    if (nid < 0 || nid >= MAX_NUMNODES || numa_off) {
    nid = 0;
    }
    cpu_to_node_map[cpu] = nid;
//
// We should set the numa node of cpu0 as soon as possible, because it
// has already been set up online before. cpu_to_node(0) will soon be
// called.
//
    if (!cpu) {
    set_cpu_numa_node(cpu, nid);
    }
    }

    unsigned long __per_cpu_offset[NR_CPUS] ;
    EXPORT_SYMBOL(__per_cpu_offset);
#[no_mangle]
pub unsafe extern "C" fn early_cpu_to_node(cpu: c_int) -> c_int {
    return cpu_to_node_map[cpu];
    }
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_distance(from: c_uint, to: c_uint) -> c_int {
    return node_distance(early_cpu_to_node(from), early_cpu_to_node(to));
    }
#[no_mangle]
pub unsafe extern "C" fn setup_per_cpu_areas()  {
    let mut delta = 0;
    let mut cpu = 0;
pub static mut rc: c_int = 0;
    if (pcpu_chosen_fc != PCPU_FC_PAGE) {
//
// Always reserve area for module percpu variables.  That's
// what the legacy allocator did.
//
    rc = pcpu_embed_first_chunk(PERCPU_MODULE_RESERVE,
    PERCPU_DYNAMIC_RESERVE, PAGE_SIZE,
    pcpu_cpu_distance,
    early_cpu_to_node);

    if (rc < 0) {
    pr_warn!("PERCPU: %s allocator failed (%d), falling back to page size\n",
    pcpu_fc_names[pcpu_chosen_fc], rc);
    }

    }

    if (rc < 0) {
    rc = pcpu_page_first_chunk(PERCPU_MODULE_RESERVE, early_cpu_to_node);
    }

    if (rc < 0) {
    panic("Failed to initialize percpu areas (err=%d).", rc);
    }
    delta = (unsigned long)pcpu_base_addr - (unsigned long)__per_cpu_start;
    for_each_possible_cpu(cpu) {
    __per_cpu_offset[cpu] = delta + pcpu_unit_offsets[cpu];
    }
    }

//
// Initialize NODE_DATA for a node on the local memory
//
#[no_mangle]
unsafe extern "C" fn setup_node_data(nid: c_int, start_pfn: u64, end_pfn: u64)  {
    if (start_pfn >= end_pfn) {
    pr_info!("Initmem setup node %d [<memory-less node>]\n", nid);
    }
    alloc_node_data(nid);
    NODE_DATA(nid).node_id = nid;
    NODE_DATA(nid).node_start_pfn = start_pfn;
    NODE_DATA(nid).node_spanned_pages = end_pfn - start_pfn;
    }
#[no_mangle]
unsafe extern "C" fn numa_register_nodes() -> c_int {
    let mut nid = 0;
// Check the validity of the memblock/node mapping
    if (!memblock_validate_numa_coverage(0)) {
    return -EINVAL;
    }
// Finally register nodes.
    for_each_node_mask(nid, numa_nodes_parsed) {
    unsigned long start_pfn, end_pfn;
    get_pfn_range_for_nid(nid, &start_pfn, &end_pfn);
    setup_node_data(nid, start_pfn, end_pfn);
    node_set_online(nid);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn numa_init((*init_func)(void): *mut c_int) -> c_int {
    static int __init numa_init(int (*init_func)(void))
    {
    let mut ret = 0;
    ret = numa_memblks_init(init_func, /* memblock_force_top_down */ false);
    if (ret < 0) {
// goto;
    }
    if (nodes_empty(numa_nodes_parsed)) {
    pr_info!("No NUMA configuration found\n");
    ret = -EINVAL;
// goto;
    }
    ret = numa_register_nodes();
    if (ret < 0) {
// goto;
    }
    setup_node_to_cpumask_map();
    return 0;
// label;
    numa_reset_distance();
    return ret;
    }
//
// dummy_numa_init() - Fallback dummy NUMA init
//
// Used if there's no underlying NUMA architecture, NUMA initialization
// fails, or NUMA is disabled on the command line.
//
// Must online at least one node (node 0) and add memory blocks that cover all
// allowed memory. It is unlikely that this function fails.
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
unsafe extern "C" fn dummy_numa_init() -> c_int {
pub static mut start: phys_addr_t = 0;
pub static mut end: phys_addr_t = 0;
    let mut ret = 0;
    if (numa_off) {
    pr_info!("NUMA disabled\n"); /* Forced off on command line. */
    }
    pr_info!("Faking a node at [mem %pap-%pap]\n", &start, &end);
    ret = numa_add_memblk(0, start, end + 1);
    if (ret) {
    pr_err!("NUMA init failed\n");
    return ret;
    }
    numa_off = true;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn arch_acpi_numa_init() -> c_int {
    let mut ret = 0;
    ret = acpi_numa_init();
    if (ret) {
    pr_debug!("Failed to initialise from firmware\n");
    return ret;
    }
    return srat_disabled() ? -EINVAL : 0;
    }

#[no_mangle]
unsafe extern "C" fn arch_acpi_numa_init() -> c_int {
    return -EOPNOTSUPP;
    }

//
// arch_numa_init() - Initialize NUMA
//
// Try each configured NUMA initialization method until one succeeds. The
// last fallback is dummy single node config encompassing whole memory.
//
#[no_mangle]
pub unsafe extern "C" fn arch_numa_init()  {
    if (!numa_off) {
    if (!acpi_disabled && !numa_init(arch_acpi_numa_init)) {
    return;
    }
    if (acpi_disabled && !numa_init(of_numa_init)) {
    return;
    }
    }
    numa_init(dummy_numa_init);
    }

    void __init numa_emu_update_cpu_to_node(int *emu_nid_to_phys,
    unsigned int nr_emu_nids)
    {
    let mut i = 0;
    let mut j = 0;
//
// Transform cpu_to_node_map table to use emulated nids by
// reverse-mapping phys_nid.  The maps should always exist but fall
// back to zero just in case.
//
    while (i < ARRAY_SIZE!(cpu_to_node_map)) {
    if (cpu_to_node_map[i] == NUMA_NO_NODE) {
    continue;
    }
    for (j = 0; j < nr_emu_nids; j++) {
    if (cpu_to_node_map[i] == emu_nid_to_phys[j])
    break;
    }
    cpu_to_node_map[i] = j < nr_emu_nids ? j : 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn numa_emu_dma_end() -> u64 __init {
    return memblock_start_of_DRAM() + SZ_4G;
    }
#[no_mangle]
pub unsafe extern "C" fn debug_cpumask_set_cpu(cpu: c_uint, node: c_int, enable: bool) {
pub static mut mask: *mut c_void = core::ptr::null_mut();
    if (node == NUMA_NO_NODE) {
    return;
    }
    mask = node_to_cpumask_map[node];
    if (!cpumask_available(mask)) {
    pr_err!("node_to_cpumask_map[%i] core::ptr::null_mut()\n", node);
    dump_stack();
    return;
    }
    if (enable) {
    cpumask_set_cpu(cpu, mask);
    }
    else {
    cpumask_clear_cpu(cpu, mask);
    }
    pr_debug!("%s cpu %d node %d: mask now %*pbl\n",
    enable ? "numa_add_cpu" : "numa_remove_cpu",
    cpu, node, cpumask_pr_args(mask));
    }
}
