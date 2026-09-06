//! Automatically rewritten from C to Rust
//! Source: mm/numa_memblks.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

    let mut numa_distance_cnt = 0;
pub static mut numa_distance: *mut c_void = core::ptr::null_mut();
    nodemask_t numa_nodes_parsed __initdata;
    static struct numa_meminfo numa_meminfo __initdata_or_meminfo;
    static struct numa_meminfo numa_reserved_meminfo __initdata_or_meminfo;
//
// numa_reset_distance - Reset NUMA distance table
//
// The current table is freed.  The next numa_set_distance() call will
// create a new one.
//
#[no_mangle]
pub unsafe extern "C" fn numa_reset_distance()  {
pub static mut size: usize = 0;
// numa_distance could be 1LU marking allocation failure, test cnt
    if (numa_distance_cnt) {
    memblock_free(numa_distance, size);
    }
    numa_distance_cnt = 0;
    numa_distance = core::ptr::null_mut();	/* enable table creation */
    }
#[no_mangle]
unsafe extern "C" fn numa_alloc_distance() -> c_int {
    let mut nodes_parsed;
    let mut size = 0;
    int i, j, cnt = 0;
// size the new table and allocate it
    nodes_parsed = numa_nodes_parsed;
    for_each_node_mask(i, nodes_parsed) {
    cnt = i;
    }
    cnt += 1;
    size = cnt * cnt * sizeof!(numa_distance[0]);
    numa_distance = memblock_alloc(size, PAGE_SIZE);
    if (!numa_distance) {
    pr_warn!("Warning: can't allocate distance table!\n");
// don't retry until explicitly reset
    numa_distance = 1LU;
    return -ENOMEM;
    }
    numa_distance_cnt = cnt;
// fill with the default distances
    for (i = 0; i < cnt; i++) {
    for (j = 0; j < cnt; j++)
    }
    numa_distance[i * cnt + j] = i == j ?
    LOCAL_DISTANCE : REMOTE_DISTANCE;
    pr_debug!("NUMA: Initialized distance table, cnt=%d\n", cnt);
    return 0;
    }
//
// numa_set_distance - Set NUMA distance from one NUMA to another
// @from: the 'from' node to set distance
// @to: the 'to'  node to set distance
// @distance: NUMA distance
//
// Set the distance from node @from to @to to @distance.  If distance table
// doesn't exist, one which is large enough to accommodate all the currently
// known nodes will be created.
//
// If such table cannot be allocated, a warning is printed and further
// calls are ignored until the distance table is reset with
// numa_reset_distance().
//
// If @from or @to is higher than the highest known node or lower than zero
// at the time of table creation or @distance doesn't make sense, the call
// is ignored.
// This is to allow simplification of specific NUMA config implementations.
//
#[no_mangle]
pub unsafe extern "C" fn numa_set_distance(from: c_int, to: c_int, distance: c_int)  {
    if (!numa_distance && numa_alloc_distance() < 0) {
    return;
    }
    if (from >= numa_distance_cnt || to >= numa_distance_cnt ||
    from < 0 || to < 0) {
    pr_warn_once("Warning: node ids are out of bound, from=%d to=%d distance=%d\n",
    from, to, distance);
    return;
    }
    if ((u8)distance != distance ||
    (from == to && distance != LOCAL_DISTANCE)) {
    pr_warn_once("Warning: invalid distance parameter, from=%d to=%d distance=%d\n",
    from, to, distance);
    return;
    }
    numa_distance[from * numa_distance_cnt + to] = distance;
    }
#[no_mangle]
pub unsafe extern "C" fn __node_distance(from: c_int, to: c_int) -> c_int {
    if (from >= numa_distance_cnt || to >= numa_distance_cnt) {
pub static mut from: return = 0;
    }
    return numa_distance[from * numa_distance_cnt + to];
    }
    EXPORT_SYMBOL(__node_distance);
    static int __init numa_add_memblk_to(int nid, u64 start, u64 end, numa_meminfo *mi)
    {
// whine about and ignore invalid nid
    if (nid < 0 || nid >= MAX_NUMNODES) {
    pr_warn!("Warning: invalid memblk node id %d [mem %#010Lx-%#010Lx]\n",
    nid, start, end - 1);
    return -EINVAL;
    }
// ignore zero length blks
    if (start == end) {
    return 0;
    }
// whine about and ignore invalid ranges
    if (start > end) {
    pr_warn!("Warning: invalid memblk range for node %d [mem %#010Lx-%#010Lx]\n",
    nid, start, end - 1);
    return 0;
    }
    if (mi.nr_blks >= NR_NODE_MEMBLKS) {
    pr_err!("too many memblk ranges\n");
    return -EINVAL;
    }
    mi.blk[mi.nr_blks].start = start;
    mi.blk[mi.nr_blks].end = end;
    mi.blk[mi.nr_blks].nid = nid;
    mi.nr_blks += 1;
    return 0;
    }
//
// numa_remove_memblk_from - Remove one numa_memblk from a numa_meminfo
// @idx: Index of memblk to remove
// @mi: numa_meminfo to remove memblk from
//
// Remove @idx'th numa_memblk from @mi by shifting @mi->blk[] and
// decrementing @mi->nr_blks.
//
#[no_mangle]
pub unsafe extern "C" fn numa_remove_memblk_from(idx: c_int, mi: *mut numa_meminfo)  {
    mi.nr_blks -= 1;
    memmove(&mi.blk[idx], &mi.blk[idx + 1],
    (mi.nr_blks - idx) * sizeof!(mi.blk[0]));
    }
//
// numa_move_tail_memblk - Move a numa_memblk from one numa_meminfo to another
// @dst: numa_meminfo to append block to
// @idx: Index of memblk to remove
// @src: numa_meminfo to remove memblk from
//
    static void __init numa_move_tail_memblk(numa_meminfo *dst, int idx, numa_meminfo *src)
    {
    dst.blk[dst.nr_blks++] = src.blk[idx];
    numa_remove_memblk_from(idx, src);
    }
//
// numa_add_memblk - Add one numa_memblk to numa_meminfo
// @nid: NUMA node ID of the new memblk
// @start: Start address of the new memblk
// @end: End address of the new memblk
//
// Add a new memblk to the default numa_meminfo.
// On success @nid is also set in numa_nodes_parsed.
//
// RETURNS:
// 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn numa_add_memblk(nid: c_int, start: u64, end: u64) -> c_int {
    let mut ret = 0;
    ret = numa_add_memblk_to(nid, start, end, &numa_meminfo);
    if (!ret) {
    node_set(nid, numa_nodes_parsed);
    }
    return ret;
    }
//
// numa_add_reserved_memblk - Add one numa_memblk to numa_reserved_meminfo
// @nid: NUMA node ID of the new memblk
// @start: Start address of the new memblk
// @end: End address of the new memblk
//
// Add a new memblk to the numa_reserved_meminfo.
//
// Usage Case: numa_cleanup_meminfo() reconciles all numa_memblk instances
// against memblock_type information and moves any that intersect reserved
// ranges to numa_reserved_meminfo. However, when that information is known
// ahead of time, we use numa_add_reserved_memblk() to add the numa_memblk
// to numa_reserved_meminfo directly.
//
// RETURNS:
// 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn numa_add_reserved_memblk(nid: c_int, start: u64, end: u64) -> c_int {
    return numa_add_memblk_to(nid, start, end, &numa_reserved_meminfo);
    }
//
// numa_cleanup_meminfo - Cleanup a numa_meminfo
// @mi: numa_meminfo to clean up
//
// Sanitize @mi by merging and removing unnecessary memblks.  Also check for
// conflicts and clear unused memblks.
//
// RETURNS:
// 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn numa_cleanup_meminfo(mi: *mut numa_meminfo) -> c_int {
pub static mut low: u64 = 0;
pub static mut high: u64 = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
// first, trim all entries
    while (i < mi.nr_blks) {
    let mut bi = &mi.blk[i];
// move / save reserved memory ranges
    if (!memblock_overlaps_region(&memblock.memory,
    bi.start, bi.end - bi.start)) {
    numa_move_tail_memblk(&numa_reserved_meminfo, i--, mi);
    continue;
    }
// make sure all non-reserved blocks are inside the limits
    bi.start = max(bi.start, low);
// preserve info for non-RAM areas above 'max_pfn':
    if (bi.end > high) {
    numa_add_reserved_memblk(bi.nid, high, bi.end);
    bi.end = high;
    }
// and there's no empty block
    if (bi.start >= bi.end) {
    numa_remove_memblk_from(i--, mi);
    }
    }
// merge neighboring / overlapping entries
    while (i < mi.nr_blks) {
    let mut bi = &mi.blk[i];
    while (j < mi.nr_blks) {
    let mut bj = &mi.blk[j];
    u64 start, end;
//
// See whether there are overlapping blocks.  Whine
// about but allow overlaps of the same nid.  They
// will be merged below.
//
    if (bi.end > bj.start && bi.start < bj.end) {
    if (bi.nid != bj.nid) {
    pr_err!("node %d [mem %#010Lx-%#010Lx] overlaps with node %d [mem %#010Lx-%#010Lx]\n",
    bi.nid, bi.start, bi.end - 1,
    bj.nid, bj.start, bj.end - 1);
    return -EINVAL;
    }
    pr_warn!("Warning: node %d [mem %#010Lx-%#010Lx] overlaps with itself [mem %#010Lx-%#010Lx]\n",
    bi.nid, bi.start, bi.end - 1,
    bj.start, bj.end - 1);
    }
//
// Join together blocks on the same node, holes
// between which don't overlap with memory on other
// nodes.
//
    if (bi.nid != bj.nid) {
    continue;
    }
    start = min(bi.start, bj.start);
    end = max(bi.end, bj.end);
    while (k < mi.nr_blks) {
    let mut bk = &mi.blk[k];
    if (bi.nid == bk.nid) {
    continue;
    }
    if (start < bk.end && end > bk.start) {
    break;
    }
    }
    if (k < mi.nr_blks) {
    continue;
    }
    pr_info!("NUMA: Node %d [mem %#010Lx-%#010Lx] + [mem %#010Lx-%#010Lx] . [mem %#010Lx-%#010Lx]\n",
    bi.nid, bi.start, bi.end - 1, bj.start,
    bj.end - 1, start, end - 1);
    bi.start = start;
    bi.end = end;
    numa_remove_memblk_from(j--, mi);
    }
    }
// clear unused ones
    while (i < ARRAY_SIZE!(mi.blk)) {
    mi.blk[i].start = mi.blk[i].end = 0;
    mi.blk[i].nid = NUMA_NO_NODE;
    }
    return 0;
    }
//
// Mark all currently memblock-reserved physical memory (which covers the
// kernel's own memory ranges) as hot-unswappable.
//
#[no_mangle]
unsafe extern "C" fn numa_clear_kernel_node_hotplug()  {
pub static mut reserved_nodemask: nodemask_t = 0;
pub static mut mb_region: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
//
// We have to do some preprocessing of memblock regions, to
// make them suitable for reservation.
//
// At this time, all memory regions reserved by memblock are
// used by the kernel, but those regions are not split up
// along node boundaries yet, and don't necessarily have their
// node ID set yet either.
//
// So iterate over all parsed memory blocks and use those ranges to
// set the nid in memblock.reserved.  This will split up the
// memblock regions along node boundaries and will set the node IDs
// as well.
//
    while (i < numa_meminfo.nr_blks) {
    let mut mb = numa_meminfo.blk + i;
    let mut ret = 0;
    ret = memblock_set_node(mb.start, mb.end - mb.start,
    &memblock.reserved, mb.nid);
    WARN_ON_ONCE!(ret);
    }
//
// Now go over all reserved memblock regions, to construct a
// node mask of all kernel reserved memory areas.
//
// [ Note, when booting with mem=nn[kMG] or in a kdump kernel,
// numa_meminfo might not include all memblock.reserved
// memory ranges, because quirks such as trim_snb_memory()
// reserve specific pages for Sandy Bridge graphics. ]
//
    for_each_reserved_mem_region(mb_region) {
pub static mut nid: c_int = 0;
    if (numa_valid_node(nid)) {
    node_set(nid, reserved_nodemask);
    }
    }
//
// Finally, clear the MEMBLOCK_HOTPLUG flag for all memory
// belonging to the reserved node mask.
//
// Note that this will include memory regions that reside
// on nodes that contain kernel memory - entire nodes
// become hot-unpluggable:
//
    while (i < numa_meminfo.nr_blks) {
    let mut mb = numa_meminfo.blk + i;
    if (!node_isset(mb.nid, reserved_nodemask)) {
    continue;
    }
    memblock_clear_hotplug(mb.start, mb.end - mb.start);
    }
    }
#[no_mangle]
unsafe extern "C" fn numa_register_meminfo(mi: *mut numa_meminfo) -> c_int {
    let mut i = 0;
// Account for nodes with cpus and no memory
    node_possible_map = numa_nodes_parsed;
    if (WARN_ON!(nodes_empty(node_possible_map))) {
    return -EINVAL;
    }
    while (i < mi.nr_blks) {
    let mut mb = &mi.blk[i];
    memblock_set_node(mb.start, mb.end - mb.start,
    &memblock.memory, mb.nid);
    }
//
// At very early time, the kernel have to use some memory such as
// loading the kernel image. We cannot prevent this anyway. So any
// node the kernel resides in should be un-hotpluggable.
//
// And when we come here, alloc node data won't fail.
//
    numa_clear_kernel_node_hotplug();
//
// If sections array is gonna be used for pfn -> nid mapping, check
// whether its granularity is fine enough.
//
    if (IS_ENABLED!(NODE_NOT_IN_PAGE_FLAGS)) {
pub static mut pfn_align: c_ulong = 0;
    if (pfn_align && pfn_align < PAGES_PER_SECTION) {
pub static mut node_align_mb: c_ulong = 0;
pub static mut sect_align_mb: c_ulong = 0;
    pr_warn!("Node alignment %luMB < min %luMB, rejecting NUMA config\n",
    node_align_mb, sect_align_mb);
    return -EINVAL;
    }
    }
    return 0;
    }
    int __init numa_memblks_init(int (*init_func)(void),
    bool memblock_force_top_down)
    {
pub static mut max_addr: phys_addr_t = 0;
    let mut ret = 0;
    nodes_clear(numa_nodes_parsed);
    nodes_clear(node_possible_map);
    nodes_clear(node_online_map);
    memset(&numa_meminfo, 0, sizeof!(numa_meminfo));
    WARN_ON!(memblock_set_node(0, max_addr, &memblock.memory, NUMA_NO_NODE));
    WARN_ON!(memblock_set_node(0, max_addr, &memblock.reserved,
    NUMA_NO_NODE));
// In case that parsing SRAT failed.
    WARN_ON!(memblock_clear_hotplug(0, max_addr));
    numa_reset_distance();
    ret = init_func();
    if (ret < 0) {
    return ret;
    }
//
// We reset memblock back to the top-down direction
// here because if we configured ACPI_NUMA, we have
// parsed SRAT in init_func(). It is ok to have the
// reset here even if we didn't configure ACPI_NUMA
// or acpi numa init fails and fallbacks to dummy
// numa init.
//
    if (memblock_force_top_down) {
    memblock_set_bottom_up(false);
    }
    ret = numa_cleanup_meminfo(&numa_meminfo);
    if (ret < 0) {
    return ret;
    }
    numa_emulation(&numa_meminfo, numa_distance_cnt);
    return numa_register_meminfo(&numa_meminfo);
    }
#[no_mangle]
unsafe extern "C" fn cmp_memblk(a: *const c_void, b: *const c_void) -> c_int {
    let mut ma = *a;
    let mut mb = *b;
    return (ma.start > mb.start) - (ma.start < mb.start);
    }
    static struct numa_memblk *numa_memblk_list[NR_NODE_MEMBLKS] __initdata;
//
// numa_fill_memblks - Fill gaps in numa_meminfo memblks
// @start: address to begin fill
// @end: address to end fill
//
// Find and extend numa_meminfo memblks to cover the physical
// address range @start-@end
//
// RETURNS:
// 0		  : Success
// NUMA_NO_MEMBLK : No memblks exist in address range @start-@end
//
#[no_mangle]
pub unsafe extern "C" fn numa_fill_memblks(start: u64, end: u64) -> c_int {
    let mut blk = &numa_memblk_list[0];
    let mut mi = &numa_meminfo;
pub static mut count: c_int = 0;
    let mut prev_end = 0;
//
// Create a list of pointers to numa_meminfo memblks that
// overlap start, end. The list is used to make in-place
// changes that fill out the numa_meminfo memblks.
//
    while (i < mi.nr_blks) {
    let mut bi = &mi.blk[i];
    if (memblock_addrs_overlap(start, end - start, bi.start,
    bi.end - bi.start)) {
    blk[count] = &mi.blk[i];
    count += 1;
    }
    }
    if (!count) {
    return NUMA_NO_MEMBLK;
    }
// Sort the list of pointers in memblk->start order
    sort(&blk[0], count, sizeof!(blk[0]), cmp_memblk, core::ptr::null_mut());
// Make sure the first/last memblks include start/end
    blk[0].start = min(blk[0].start, start);
    blk[count - 1].end = max(blk[count - 1].end, end);
//
// Fill any gaps by tracking the previous memblks
// end address and backfilling to it if needed.
//
    prev_end = blk[0].end;
    while (i < count) {
    let mut curr = blk[i];
    if (prev_end >= curr.start) {
    if (prev_end < curr.end) {
    prev_end = curr.end;
    }
    } else {
    curr.start = prev_end;
    prev_end = curr.end;
    }
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn meminfo_to_nid(mi: *mut numa_meminfo, start: u64) -> c_int {
    let mut i = 0;
    for (i = 0; i < mi.nr_blks; i++) {
    if (mi.blk[i].start <= start && mi.blk[i].end > start)
    return mi.blk[i].nid;
    }
    return NUMA_NO_NODE;
    }
#[no_mangle]
pub unsafe extern "C" fn phys_to_target_node(start: u64) -> c_int {
pub static mut nid: c_int = 0;
pub static mut reserved_nid: c_int = 0;
//
// Prefer online nodes unless the address is also described
// by reserved ranges, in which case use the reserved nid.
//
    if (nid != NUMA_NO_NODE && reserved_nid == NUMA_NO_NODE) {
    return nid;
    }
    return reserved_nid;
    }
    EXPORT_SYMBOL_GPL(phys_to_target_node);
#[no_mangle]
pub unsafe extern "C" fn memory_add_physaddr_to_nid(start: u64) -> c_int {
pub static mut nid: c_int = 0;
    if (nid == NUMA_NO_NODE) {
    nid = numa_meminfo.blk[0].nid;
    }
    return nid;
    }
    EXPORT_SYMBOL_GPL(memory_add_physaddr_to_nid);