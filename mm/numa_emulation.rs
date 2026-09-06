//! Automatically rewritten from C to Rust
//! Source: mm/numa_emulation.c
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
//
// NUMA emulation
//

    int emu_nid_to_phys[MAX_NUMNODES];
pub static mut emu_cmdline: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn numa_emu_cmdline(str: *mut c_char) -> c_int {
    emu_cmdline = str;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn emu_find_memblk_by_nid(nid: c_int, mi: *const numa_meminfo) -> c_int {
    let mut i = 0;
    for (i = 0; i < mi.nr_blks; i++) {
    if (mi.blk[i].nid == nid)
    return i;
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn mem_hole_size(start: u64, end: u64) -> u64 __init {
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
    if (start_pfn < end_pfn) {
    return PFN_PHYS(absent_pages_in_range(start_pfn, end_pfn));
    }
    return 0;
    }
//
// Sets up nid to range from @start to @end.  The return value is -errno if
// something went wrong, 0 otherwise.
//
    static int __init emu_setup_memblk(numa_meminfo *ei, numa_meminfo *pi,
    int nid, int phys_blk, u64 size)
    {
    let mut eb = &ei.blk[ei.nr_blks];
    let mut pb = &pi.blk[phys_blk];
    if (ei.nr_blks >= NR_NODE_MEMBLKS) {
    pr_err!("NUMA: Too many emulated memblks, failing emulation\n");
    return -EINVAL;
    }
    ei.nr_blks += 1;
    eb.start = pb.start;
    eb.end = pb.start + size;
    eb.nid = nid;
    if (emu_nid_to_phys[nid] == NUMA_NO_NODE) {
    emu_nid_to_phys[nid] = pb.nid;
    }
    pb.start += size;
    if (pb.start >= pb.end) {
    WARN_ON_ONCE!(pb.start > pb.end);
    numa_remove_memblk_from(phys_blk, pi);
    }
    printk("Faking node %d at [mem %#018Lx-%#018Lx] (%LuMB)\n",
    nid, eb.start, eb.end - 1, (eb.end - eb.start) / SZ_1M);
    return 0;
    }
//
// Sets up nr_nodes fake nodes interleaved over physical nodes ranging from addr
// to max_addr.
//
// Returns zero on success or negative on error.
//
    static int __init split_nodes_interleave(numa_meminfo *ei, numa_meminfo *pi,
    u64 addr, u64 max_addr, int nr_nodes)
    {
pub static mut physnode_mask: nodemask_t = 0;
    let mut size = 0;
    let mut big = 0;
pub static mut nid: c_int = 0;
    let mut i = 0;
    let mut ret = 0;
    if (nr_nodes <= 0) {
    return -1;
    }
    if (nr_nodes > MAX_NUMNODES) {
    pr_info!("numa=fake=%d too large, reducing to %d\n",
    nr_nodes, MAX_NUMNODES);
    nr_nodes = MAX_NUMNODES;
    }
//
// Calculate target node size.  x86_32 freaks on __udivdi3() so do
// the division in ulong number of pages and convert back.
//
    size = max_addr - addr - mem_hole_size(addr, max_addr);
    size = PFN_PHYS((unsigned long)(size >> PAGE_SHIFT) / nr_nodes);
//
// Calculate the number of big nodes that can be allocated as a result
// of consolidating the remainder.
//
    big = ((size & ~FAKE_NODE_MIN_HASH_MASK) * nr_nodes) /
    FAKE_NODE_MIN_SIZE;
    size &= FAKE_NODE_MIN_HASH_MASK;
    if (!size) {
    pr_err!("Not enough memory for each node.  "
    "NUMA emulation disabled.\n");
    return -1;
    }
//
// Continue to fill physical nodes with fake nodes until there is no
// memory left on any of them.
//
    while (!nodes_empty(physnode_mask)) {
    for_each_node_mask(i, physnode_mask) {
pub static mut dma32_end: u64 = 0;
    u64 start, limit, end;
    let mut phys_blk = 0;
    phys_blk = emu_find_memblk_by_nid(i, pi);
    if (phys_blk < 0) {
    node_clear(i, physnode_mask);
    continue;
    }
    start = pi.blk[phys_blk].start;
    limit = pi.blk[phys_blk].end;
    end = start + size;
    if (nid < big) {
    end += FAKE_NODE_MIN_SIZE;
    }
//
// Continue to add memory to this fake node if its
// non-reserved memory is less than the per-node size.
//
    while (end - start - mem_hole_size(start, end) < size) {
    end += FAKE_NODE_MIN_SIZE;
    if (end > limit) {
    end = limit;
    break;
    }
    }
//
// If there won't be at least FAKE_NODE_MIN_SIZE of
// non-reserved memory in ZONE_DMA32 for the next node,
// this one must extend to the boundary.
//
    if (end < dma32_end && dma32_end - end -
    mem_hole_size(end, dma32_end) < FAKE_NODE_MIN_SIZE) {
    end = dma32_end;
    }
//
// If there won't be enough non-reserved memory for the
// next node, this one must extend to the end of the
// physical node.
//
    if (limit - end - mem_hole_size(end, limit) < size) {
    end = limit;
    }
    ret = emu_setup_memblk(ei, pi, nid++ % nr_nodes,
    phys_blk,
    min(end, limit) - start);
    if (ret < 0) {
    return ret;
    }
    }
    }
    return 0;
    }
//
// Returns the end address of a node so that there is at least `size' amount of
// non-reserved memory or `max_addr' is reached.
//
#[no_mangle]
unsafe extern "C" fn find_end_of_node(start: u64, max_addr: u64, size: u64) -> u64 __init {
pub static mut end: u64 = 0;
    while (end - start - mem_hole_size(start, end) < size) {
    end += FAKE_NODE_MIN_SIZE;
    if (end > max_addr) {
    end = max_addr;
    break;
    }
    }
    return end;
    }
#[no_mangle]
unsafe extern "C" fn uniform_size(max_addr: u64, base: u64, hole: u64, nr_nodes: c_int) -> u64 {
pub static mut max_pfn: c_ulong = 0;
pub static mut base_pfn: c_ulong = 0;
pub static mut hole_pfns: c_ulong = 0;
    return PFN_PHYS((max_pfn - base_pfn - hole_pfns) / nr_nodes);
    }
//
// Sets up fake nodes of `size' interleaved over physical nodes ranging from
// `addr' to `max_addr'.
//
// Returns node ID of the next node on success or negative error code.
//
    static int __init split_nodes_size_interleave_uniform(numa_meminfo *ei, numa_meminfo *pi,
    u64 addr, u64 max_addr, u64 size,
    int nr_nodes, numa_memblk *pblk,
    int nid)
    {
pub static mut physnode_mask: nodemask_t = 0;
    int i, ret, uniform = 0;
    let mut min_size = 0;
    if ((!size && !nr_nodes) || (nr_nodes && !pblk)) {
    return -1;
    }
//
// In the 'uniform' case split the passed in physical node by
// nr_nodes, in the non-uniform case, ignore the passed in
// physical block and try to create nodes of at least size
// @size.
//
// In the uniform case, split the nodes strictly by physical
// capacity, i.e. ignore holes. In the non-uniform case account
// for holes and treat @size as a minimum floor.
//
    if (!nr_nodes) {
    nr_nodes = MAX_NUMNODES;
    }
    else {
    nodes_clear(physnode_mask);
    node_set(pblk.nid, physnode_mask);
    uniform = 1;
    }
    if (uniform) {
    min_size = uniform_size(max_addr, addr, 0, nr_nodes);
    size = min_size;
    } else {
//
// The limit on emulated nodes is MAX_NUMNODES, so the
// size per node is increased accordingly if the
// requested size is too small.  This creates a uniform
// distribution of node sizes across the entire machine
// (but not necessarily over physical nodes).
//
    min_size = uniform_size(max_addr, addr,
    mem_hole_size(addr, max_addr), nr_nodes);
    }
    min_size = ALIGN(max(min_size, FAKE_NODE_MIN_SIZE), FAKE_NODE_MIN_SIZE);
    if (size < min_size) {
    pr_err!("Fake node size %LuMB too small, increasing to %LuMB\n",
    size / SZ_1M, min_size / SZ_1M);
    size = min_size;
    }
    size = ALIGN_DOWN(size, FAKE_NODE_MIN_SIZE);
//
// Fill physical nodes with fake nodes of size until there is no memory
// left on any of them.
//
    while (!nodes_empty(physnode_mask)) {
    for_each_node_mask(i, physnode_mask) {
pub static mut dma32_end: u64 = 0;
    u64 start, limit, end;
    let mut phys_blk = 0;
    phys_blk = emu_find_memblk_by_nid(i, pi);
    if (phys_blk < 0) {
    node_clear(i, physnode_mask);
    continue;
    }
    start = pi.blk[phys_blk].start;
    limit = pi.blk[phys_blk].end;
    if (uniform) {
    end = start + size;
    }
    else {
    end = find_end_of_node(start, limit, size);
    }
//
// If there won't be at least FAKE_NODE_MIN_SIZE of
// non-reserved memory in ZONE_DMA32 for the next node,
// this one must extend to the boundary.
//
    if (end < dma32_end && dma32_end - end -
    mem_hole_size(end, dma32_end) < FAKE_NODE_MIN_SIZE) {
    end = dma32_end;
    }
//
// If there won't be enough non-reserved memory for the
// next node, this one must extend to the end of the
// physical node.
//
    if ((limit - end - mem_hole_size(end, limit) < size)
    && !uniform) {
    end = limit;
    }
    ret = emu_setup_memblk(ei, pi, nid++ % MAX_NUMNODES,
    phys_blk,
    min(end, limit) - start);
    if (ret < 0) {
    return ret;
    }
    }
    }
    return nid;
    }
    static int __init split_nodes_size_interleave(numa_meminfo *ei, numa_meminfo *pi,
    u64 addr, u64 max_addr, u64 size)
    {
    return split_nodes_size_interleave_uniform(ei, pi, addr, max_addr, size,
    0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn setup_emu2phys_nid(dfl_phys_nid: *mut c_int) -> c_int {
    int i, max_emu_nid = 0;
// dfl_phys_nid = NUMA_NO_NODE;
    while (i < ARRAY_SIZE!(emu_nid_to_phys)) {
    if (emu_nid_to_phys[i] != NUMA_NO_NODE) {
    max_emu_nid = i;
    if (*dfl_phys_nid == NUMA_NO_NODE) {
// dfl_phys_nid = emu_nid_to_phys[i];
    }
    }
    }
    return max_emu_nid;
    }
//
// numa_emulation - Emulate NUMA nodes
// @numa_meminfo: NUMA configuration to massage
// @numa_dist_cnt: The size of the physical NUMA distance table
//
// Emulate NUMA nodes according to the numa=fake kernel parameter.
// @numa_meminfo contains the physical memory configuration and is modified
// to reflect the emulated configuration on success.  @numa_dist_cnt is
// used to determine the size of the physical distance table.
//
// On success, the following modifications are made.
//
// - @numa_meminfo is updated to reflect the emulated nodes.
//
// - __apicid_to_node[] is updated such that APIC IDs are mapped to the
// emulated nodes.
//
// - NUMA distance table is rebuilt to represent distances between emulated
// nodes.  The distances are determined considering how emulated nodes
// are mapped to physical nodes and match the actual distances.
//
// - emu_nid_to_phys[] reflects how emulated nodes are mapped to physical
// nodes.  This is used by numa_add_cpu() and numa_remove_cpu().
//
// If emulation is not enabled or fails, emu_nid_to_phys[] is filled with
// identity mapping and no other modification is made.
//
#[no_mangle]
pub unsafe extern "C" fn numa_emulation(numa_meminfo: *mut numa_meminfo, numa_dist_cnt: c_int)  {
    static struct numa_meminfo ei __initdata;
    static struct numa_meminfo pi __initdata;
pub static mut max_addr: u64 = 0;
    let mut phys_dist = core::ptr::null_mut();
pub static mut phys_size: usize = 0;
    let mut max_emu_nid = 0;
    let mut dfl_phys_nid = 0;
    let mut i = 0;
    let mut j = 0;
    let mut ret = 0;
pub static mut physnode_mask: nodemask_t = 0;
    if (!emu_cmdline) {
// goto;
    }
    memset(&ei, 0, sizeof!(ei));
    pi = *numa_meminfo;
    for (i = 0; i < MAX_NUMNODES; i++) {
    emu_nid_to_phys[i] = NUMA_NO_NODE;
    }
//
// If the numa=fake command-line contains a 'M' or 'G', it represents
// the fixed node size.  Otherwise, if it is just a single number N,
// split the system RAM into N fake nodes.
//
    if (strchr(emu_cmdline, 'U')) {
    let mut n = 0;
pub static mut nid: c_int = 0;
    n = simple_strtoul(emu_cmdline, &emu_cmdline, 0);
    ret = -1;
    for_each_node_mask(i, physnode_mask) {
//
// The reason we pass in blk[0] is due to
// numa_remove_memblk_from() called by
// emu_setup_memblk() will delete entry 0
// and then move everything else up in the pi.blk
// array. Therefore we should always be looking
// at blk[0].
//
    ret = split_nodes_size_interleave_uniform(&ei, &pi,
    pi.blk[0].start, pi.blk[0].end, 0,
    n, &pi.blk[0], nid);
    if (ret < 0) {
    break;
    }
//
// If no memory was found for this physical node,
// skip the under-allocation check.
//
    if (ret == nid) {
    continue;
    }
    nr_created = ret - nid;
    if (nr_created < n) {
    pr_info!("%s: phys: %d only got %d of %ld nodes, failing\n",
    __func__, i, nr_created, n);
    ret = -1;
    break;
    }
    nid = ret;
    }
    } else if (strchr(emu_cmdline, 'M') || strchr(emu_cmdline, 'G')) {
    let mut size = 0;
    size = memparse(emu_cmdline, &emu_cmdline);
    ret = split_nodes_size_interleave(&ei, &pi, 0, max_addr, size);
    } else {
    let mut n = 0;
    n = simple_strtoul(emu_cmdline, &emu_cmdline, 0);
    ret = split_nodes_interleave(&ei, &pi, 0, max_addr, n);
    }
    if (*emu_cmdline == ':') {
    emu_cmdline += 1;
    }
    if (ret < 0) {
// goto;
    }
    if (numa_cleanup_meminfo(&ei) < 0) {
    pr_warn!("NUMA: Warning: constructed meminfo invalid, disabling emulation\n");
// goto;
    }
// copy the physical distance table
    if (numa_dist_cnt) {
    phys_dist = memblock_alloc(phys_size, PAGE_SIZE);
    if (!phys_dist) {
    pr_warn!("NUMA: Warning: can't allocate copy of distance table, disabling emulation\n");
// goto;
    }
    for (i = 0; i < numa_dist_cnt; i++) {
    for (j = 0; j < numa_dist_cnt; j++)
    }
    phys_dist[i * numa_dist_cnt + j] =
    node_distance(i, j);
    }
//
// Determine the max emulated nid and the default phys nid to use
// for unmapped nodes.
//
    max_emu_nid = setup_emu2phys_nid(&dfl_phys_nid);
// Make sure numa_nodes_parsed only contains emulated nodes
    nodes_clear(numa_nodes_parsed);
    for (i = 0; i < ARRAY_SIZE!(ei.blk); i++) {
    if (ei.blk[i].start != ei.blk[i].end &&
    ei.blk[i].nid != NUMA_NO_NODE)
    node_set(ei.blk[i].nid, numa_nodes_parsed);
    }
// fix pxm_to_node_map[] and node_to_pxm_map[] to avoid collision
// with faked numa nodes, particularly during later memory hotplug
// handling, and also update numa_nodes_parsed accordingly.
//
    ret = fix_pxm_node_maps(max_emu_nid);
    if (ret < 0) {
// goto;
    }
// commit
// numa_meminfo = ei;
    numa_emu_update_cpu_to_node(emu_nid_to_phys, max_emu_nid + 1);
// make sure all emulated nodes are mapped to a physical node
    for (i = 0; i < max_emu_nid + 1; i++) {
    if (emu_nid_to_phys[i] == NUMA_NO_NODE)
    emu_nid_to_phys[i] = dfl_phys_nid;
    }
// transform distance table
    numa_reset_distance();
    while (i < max_emu_nid + 1) {
    while (j < max_emu_nid + 1) {
pub static mut physi: c_int = 0;
pub static mut physj: c_int = 0;
    let mut dist = 0;
    if (get_option(&emu_cmdline, &dist) == 2) {
    ;
    }

    else if (physi >= numa_dist_cnt || physj >= numa_dist_cnt) {
    dist = physi == physj ?
    LOCAL_DISTANCE : REMOTE_DISTANCE;
    }
    else {
    dist = phys_dist[physi * numa_dist_cnt + physj];
    }
    numa_set_distance(i, j, dist);
    }
    }
    while (i < numa_distance_cnt) {
    while (j < numa_distance_cnt) {
    let mut physi = 0;
    let mut physj = 0;
    let mut dist = 0;
// distance between fake nodes is already ok
    if (emu_nid_to_phys[i] != NUMA_NO_NODE &&
    emu_nid_to_phys[j] != NUMA_NO_NODE) {
    continue;
    }
    if (emu_nid_to_phys[i] != NUMA_NO_NODE) {
    physi = emu_nid_to_phys[i];
    }
    else {
    physi = i - max_emu_nid;
    }
    if (emu_nid_to_phys[j] != NUMA_NO_NODE) {
    physj = emu_nid_to_phys[j];
    }
    else {
    physj = j - max_emu_nid;
    }
    dist = phys_dist[physi * numa_dist_cnt + physj];
    numa_set_distance(i, j, dist);
    }
    }
// free the copied physical distance table
    memblock_free(phys_dist, phys_size);
    return;
// label;
    numa_nodes_parsed = physnode_mask;
// No emulation.  Build identity emu_nid_to_phys[] for numa_add_cpu()
    for (i = 0; i < ARRAY_SIZE!(emu_nid_to_phys); i++) {
    emu_nid_to_phys[i] = i;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn numa_add_cpu(cpu: c_uint) {
    let mut physnid = 0;
    let mut nid = 0;
    nid = early_cpu_to_node(cpu);
    BUG_ON!(nid == NUMA_NO_NODE || !node_online(nid));
    physnid = emu_nid_to_phys[nid];
//
// Map the cpu to each emulated node that is allocated on the physical
// node of the cpu's apic id.
//
    for_each_online_node(nid) {
    if (emu_nid_to_phys[nid] == physnid)
    cpumask_set_cpu(cpu, node_to_cpumask_map[nid]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn numa_remove_cpu(cpu: c_uint) {
    let mut i = 0;
    for_each_online_node(i) {
    cpumask_clear_cpu(cpu, node_to_cpumask_map[i]);
    }
    }

#[no_mangle]
unsafe extern "C" fn numa_set_cpumask(cpu: c_uint, enable: bool) {
    let mut nid = 0;
    let mut physnid = 0;
    nid = early_cpu_to_node(cpu);
    if (nid == NUMA_NO_NODE) {
// early_cpu_to_node() already emits a warning and trace
    return;
    }
    physnid = emu_nid_to_phys[nid];
    for_each_online_node(nid) {
    if (emu_nid_to_phys[nid] != physnid) {
    continue;
    }
    debug_cpumask_set_cpu(cpu, nid, enable);
    }
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: numa_add_cpu
pub unsafe extern "C" fn numa_add_cpu_dup(cpu: c_uint) {
    numa_set_cpumask(cpu, true);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: numa_remove_cpu
pub unsafe extern "C" fn numa_remove_cpu_dup(cpu: c_uint) {
    numa_set_cpumask(cpu, false);
    }