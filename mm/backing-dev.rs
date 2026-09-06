//! Automatically rewritten from C to Rust
//! Source: mm/backing-dev.c
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

pub static mut noop_backing_dev_info: usize = 0;
    EXPORT_SYMBOL_GPL(noop_backing_dev_info);
    static const char *bdi_unknown_name = "(unknown)";
//
// bdi_lock protects bdi_tree and updates to bdi_list. bdi_list has RCU
// reader side locking.
//
pub static mut bdi_lock: usize = 0;
    static u64 bdi_id_cursor;
pub static mut bdi_tree: rb_root = 0;
pub static mut bdi_list: usize = 0;
// bdi_wq serves all asynchronous writeback tasks
pub static mut bdi_wq: *mut c_void = core::ptr::null_mut();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wb_stats {
    pub nr_dirty: c_ulong,
    pub nr_io: c_ulong,
    pub nr_more_io: c_ulong,
    pub nr_dirty_time: c_ulong,
    pub nr_writeback: c_ulong,
    pub nr_reclaimable: c_ulong,
    pub nr_dirtied: c_ulong,
    pub nr_written: c_ulong,
    pub dirty_thresh: c_ulong,
    pub wb_thresh: c_ulong,
}

pub static mut bdi_debug_root: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn bdi_debug_init() {
    bdi_debug_root = debugfs_create_dir("bdi", core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn collect_wb_stats(stats: *mut wb_stats, wb: *mut bdi_writeback) {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    spin_lock(&wb.list_lock);
    list_for_each_entry(inode, &wb.b_dirty, i_io_list) {
    stats.nr_dirty += 1;
    }
    list_for_each_entry(inode, &wb.b_io, i_io_list) {
    stats.nr_io += 1;
    }
    list_for_each_entry(inode, &wb.b_more_io, i_io_list) {
    stats.nr_more_io += 1;
    }
    list_for_each_entry(inode, &wb.b_dirty_time, i_io_list) {
    if (inode_state_read_once(inode) & I_DIRTY_TIME)
    stats.nr_dirty_time += 1;
    }
    spin_unlock(&wb.list_lock);
    stats.nr_writeback += wb_stat(wb, WB_WRITEBACK);
    stats.nr_reclaimable += wb_stat(wb, WB_RECLAIMABLE);
    stats.nr_dirtied += wb_stat(wb, WB_DIRTIED);
    stats.nr_written += wb_stat(wb, WB_WRITTEN);
    stats.wb_thresh += wb_calc_thresh(wb, stats.dirty_thresh);
    }

#[no_mangle]
pub unsafe extern "C" fn bdi_collect_stats(bdi: *mut backing_dev_info, stats: *mut wb_stats) {
pub static mut wb: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(wb, &bdi.wb_list, bdi_node) {
    if (!wb_tryget(wb)) {
    continue;
    }
    collect_wb_stats(stats, wb);
    wb_put(wb);
    }
    rcu_read_unlock();
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: bdi_collect_stats
pub unsafe extern "C" fn bdi_collect_stats_dup(bdi: *mut backing_dev_info, stats: *mut wb_stats) {
    collect_wb_stats(stats, &bdi.wb);
    }

#[no_mangle]
unsafe extern "C" fn bdi_debug_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut bdi = m.private;
    let mut background_thresh = 0;
    let mut dirty_thresh = 0;
pub static mut stats: usize = 0;
    let mut tot_bw = 0;
    global_dirty_limits(&background_thresh, &dirty_thresh);
    memset(&stats, 0, sizeof!(stats));
    stats.dirty_thresh = dirty_thresh;
    bdi_collect_stats(bdi, &stats);
    tot_bw = atomic_long_read(&bdi.tot_write_bandwidth);
    seq_printf(m,
    "BdiWriteback:       %10lu kB\n"
    "BdiReclaimable:     %10lu kB\n"
    "BdiDirtyThresh:     %10lu kB\n"
    "DirtyThresh:        %10lu kB\n"
    "BackgroundThresh:   %10lu kB\n"
    "BdiDirtied:         %10lu kB\n"
    "BdiWritten:         %10lu kB\n"
    "BdiWriteBandwidth:  %10lu kBps\n"
    "b_dirty:            %10lu\n"
    "b_io:               %10lu\n"
    "b_more_io:          %10lu\n"
    "b_dirty_time:       %10lu\n"
    "bdi_list:           %10u\n"
    "state:              %10lx\n",
    K(stats.nr_writeback),
    K(stats.nr_reclaimable),
    K(stats.wb_thresh),
    K(dirty_thresh),
    K(background_thresh),
    K(stats.nr_dirtied),
    K(stats.nr_written),
    K(tot_bw),
    stats.nr_dirty,
    stats.nr_io,
    stats.nr_more_io,
    stats.nr_dirty_time,
    !list_empty(&bdi.bdi_list), bdi.wb.state);
    return 0;
    }
pub static mut bdi_debug_stats: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn wb_stats_show(m: *mut seq_file, wb: *mut bdi_writeback, stats: *mut wb_stats) {
    seq_printf(m,
    "WbCgIno:           %10lu\n"
    "WbWriteback:       %10lu kB\n"
    "WbReclaimable:     %10lu kB\n"
    "WbDirtyThresh:     %10lu kB\n"
    "WbDirtied:         %10lu kB\n"
    "WbWritten:         %10lu kB\n"
    "WbWriteBandwidth:  %10lu kBps\n"
    "b_dirty:           %10lu\n"
    "b_io:              %10lu\n"
    "b_more_io:         %10lu\n"
    "b_dirty_time:      %10lu\n"
    "state:             %10lx\n\n",

    cgroup_ino(wb.memcg_css.cgroup),

    1ul,

    K(stats.nr_writeback),
    K(stats.nr_reclaimable),
    K(stats.wb_thresh),
    K(stats.nr_dirtied),
    K(stats.nr_written),
    K(wb.avg_write_bandwidth),
    stats.nr_dirty,
    stats.nr_io,
    stats.nr_more_io,
    stats.nr_dirty_time,
    wb.state);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_debug_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut bdi = m.private;
    let mut background_thresh = 0;
    let mut dirty_thresh = 0;
pub static mut wb: *mut c_void = core::ptr::null_mut();
    global_dirty_limits(&background_thresh, &dirty_thresh);
    rcu_read_lock();
    list_for_each_entry_rcu(wb, &bdi.wb_list, bdi_node) {
pub static mut stats: wb_stats = 0;
    if (!wb_tryget(wb)) {
    continue;
    }
    collect_wb_stats(&stats, wb);
//
// Calculate thresh of wb in writeback cgroup which is min of
// thresh in global domain and thresh in cgroup domain. Drop
// rcu lock because cgwb_calc_thresh may sleep in
// cgroup_rstat_flush. We can do so here because we have a ref.
//
    if (mem_cgroup_wb_domain(wb)) {
    rcu_read_unlock();
    stats.wb_thresh = min(stats.wb_thresh, cgwb_calc_thresh(wb));
    rcu_read_lock();
    }
    wb_stats_show(m, wb, &stats);
    wb_put(wb);
    }
    rcu_read_unlock();
    return 0;
    }
pub static mut cgwb_debug_stats: usize = 0;
#[no_mangle]
unsafe extern "C" fn bdi_debug_register(bdi: *mut backing_dev_info, name: *const c_char) {
    bdi.debug_dir = debugfs_create_dir(name, bdi_debug_root);
    debugfs_create_file("stats", 0444, bdi.debug_dir, bdi,
    &bdi_debug_stats_fops);
    debugfs_create_file("wb_stats", 0444, bdi.debug_dir, bdi,
    &cgwb_debug_stats_fops);
    }
#[no_mangle]
unsafe extern "C" fn bdi_debug_unregister(bdi: *mut backing_dev_info) {
    debugfs_remove_recursive(bdi.debug_dir);
    }

#[no_mangle]
pub unsafe extern "C" fn bdi_debug_init() {
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_debug_register(bdi: *mut backing_dev_info, name: *mut c_char) {
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_debug_unregister(bdi: *mut backing_dev_info) {
    }

#[no_mangle]
pub unsafe extern "C" fn read_ahead_kb_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut read_ahead_kb = 0;
    let mut ret = 0;
    ret = kstrtoul(buf, 10, &read_ahead_kb);
    if (ret < 0) {
    return ret;
    }
    bdi.ra_pages = read_ahead_kb >> (PAGE_SHIFT - 10);
    return count;
    }

    static ssize_t name##_show(device *dev, device_attribute *attr, char *buf)	
    {									
    let mut bdi = dev_get_drvdata(dev);		
    
    return sysfs_emit(buf, "%lld\n", (long long)expr);		
    }									
    static DEVICE_ATTR_RW(name);
    BDI_SHOW(read_ahead_kb, K(bdi.ra_pages))
#[no_mangle]
pub unsafe extern "C" fn min_ratio_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut ratio = 0;
    let mut ret = 0;
    ret = kstrtouint(buf, 10, &ratio);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_min_ratio(bdi, ratio);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
    BDI_SHOW(min_ratio, bdi.min_ratio / BDI_RATIO_SCALE)
#[no_mangle]
pub unsafe extern "C" fn min_ratio_fine_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut ratio = 0;
    let mut ret = 0;
    ret = kstrtouint(buf, 10, &ratio);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_min_ratio_no_scale(bdi, ratio);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
    BDI_SHOW(min_ratio_fine, bdi.min_ratio)
#[no_mangle]
pub unsafe extern "C" fn max_ratio_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut ratio = 0;
    let mut ret = 0;
    ret = kstrtouint(buf, 10, &ratio);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_max_ratio(bdi, ratio);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
    BDI_SHOW(max_ratio, bdi.max_ratio / BDI_RATIO_SCALE)
#[no_mangle]
pub unsafe extern "C" fn max_ratio_fine_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut ratio = 0;
    let mut ret = 0;
    ret = kstrtouint(buf, 10, &ratio);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_max_ratio_no_scale(bdi, ratio);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
    BDI_SHOW(max_ratio_fine, bdi.max_ratio)
#[no_mangle]
pub unsafe extern "C" fn min_bytes_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%llu\n", bdi_get_min_bytes(bdi));
    }
#[no_mangle]
pub unsafe extern "C" fn min_bytes_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut bytes = 0;
    let mut ret = 0;
    ret = kstrtoull(buf, 10, &bytes);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_min_bytes(bdi, bytes);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
    static DEVICE_ATTR_RW(min_bytes);
#[no_mangle]
pub unsafe extern "C" fn max_bytes_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%llu\n", bdi_get_max_bytes(bdi));
    }
#[no_mangle]
pub unsafe extern "C" fn max_bytes_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut bytes = 0;
    let mut ret = 0;
    ret = kstrtoull(buf, 10, &bytes);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_max_bytes(bdi, bytes);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
    static DEVICE_ATTR_RW(max_bytes);
#[no_mangle]
pub unsafe extern "C" fn stable_pages_required_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    dev_warn_once(dev,
    "the stable_pages_required attribute has been removed. Use the stable_writes queue attribute instead.\n");
    return sysfs_emit(buf, "%d\n", 0);
    }
    static DEVICE_ATTR_RO(stable_pages_required);
#[no_mangle]
pub unsafe extern "C" fn strict_limit_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    let mut strict_limit = 0;
    let mut ret = 0;
    ret = kstrtouint(buf, 10, &strict_limit);
    if (ret < 0) {
    return ret;
    }
    ret = bdi_set_strict_limit(bdi, strict_limit);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn strict_limit_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut bdi = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n",
    !!(bdi.capabilities & BDI_CAP_STRICTLIMIT));
    }
    static DEVICE_ATTR_RW(strict_limit);
    static struct attribute *bdi_dev_attrs[] = {
    &dev_attr_read_ahead_kb.attr,
    &dev_attr_min_ratio.attr,
    &dev_attr_min_ratio_fine.attr,
    &dev_attr_max_ratio.attr,
    &dev_attr_max_ratio_fine.attr,
    &dev_attr_min_bytes.attr,
    &dev_attr_max_bytes.attr,
    &dev_attr_stable_pages_required.attr,
    &dev_attr_strict_limit.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(bdi_dev);
pub static mut class: usize = 0;
#[no_mangle]
unsafe extern "C" fn bdi_class_init() -> __init int {
    let mut ret = 0;
    ret = class_register(&bdi_class);
    if (ret) {
    return ret;
    }
    bdi_debug_init();
    return 0;
    }
    postcore_initcall!(bdi_class_init);
#[no_mangle]
unsafe extern "C" fn default_bdi_init() -> c_int {
    bdi_wq = alloc_workqueue("writeback", WQ_MEM_RECLAIM | WQ_UNBOUND |
    WQ_SYSFS, 0);
    if (!bdi_wq) {
    return -ENOMEM;
    }
    return 0;
    }
    subsys_initcall!(default_bdi_init);
#[no_mangle]
unsafe extern "C" fn wb_update_bandwidth_workfn(work: *mut work_struct) {
    let mut wb = container_of!(to_delayed_work(work), bdi_writeback, bw_dwork);
    wb_update_bandwidth(wb);
    }
//
// Initial write bandwidth: 100 MB/s
//

#[no_mangle]
pub unsafe extern "C" fn wb_init(wb: *mut bdi_writeback, bdi: *mut backing_dev_info, gfp: gfp_t) -> c_int {
    let mut err = 0;
    memset(wb, 0, sizeof!(*wb));
    wb.bdi = bdi;
    wb.last_old_flush = jiffies;
    INIT_LIST_HEAD(&wb.b_dirty);
    INIT_LIST_HEAD(&wb.b_io);
    INIT_LIST_HEAD(&wb.b_more_io);
    INIT_LIST_HEAD(&wb.b_dirty_time);
    spin_lock_init(&wb.list_lock);
    atomic_set(&wb.writeback_inodes, 0);
    wb.bw_time_stamp = jiffies;
    wb.balanced_dirty_ratelimit = INIT_BW;
    wb.dirty_ratelimit = INIT_BW;
    wb.write_bandwidth = INIT_BW;
    wb.avg_write_bandwidth = INIT_BW;
    spin_lock_init(&wb.work_lock);
    INIT_LIST_HEAD(&wb.work_list);
    INIT_DELAYED_WORK(&wb.dwork, wb_workfn);
    INIT_DELAYED_WORK(&wb.bw_dwork, wb_update_bandwidth_workfn);
    err = fprop_local_init_percpu(&wb.completions, gfp);
    if (err) {
    return err;
    }
    err = percpu_counter_init_many(wb.stat, 0, gfp, NR_WB_STAT_ITEMS);
    if (err) {
    fprop_local_destroy_percpu(&wb.completions);
    }
    return err;
    }
// forward_decl: cgwb_remove_from_bdi_list;
//
// Remove bdi from the global list and shutdown any threads we have running
//
#[no_mangle]
unsafe extern "C" fn wb_shutdown(wb: *mut bdi_writeback) {
// Make sure nobody queues further work
    spin_lock_irq(&wb.work_lock);
    if (!test_and_clear_bit(WB_registered, &wb.state)) {
    spin_unlock_irq(&wb.work_lock);
    return;
    }
    spin_unlock_irq(&wb.work_lock);
    cgwb_remove_from_bdi_list(wb);
//
// Drain work list and shutdown the delayed_work.  !WB_registered
// tells wb_workfn() that @wb is dying and its work_list needs to
// be drained no matter what.
//
    mod_delayed_work(bdi_wq, &wb.dwork, 0);
    flush_delayed_work(&wb.dwork);
    WARN_ON!(!list_empty(&wb.work_list));
    flush_delayed_work(&wb.bw_dwork);
    }
#[no_mangle]
unsafe extern "C" fn wb_exit(wb: *mut bdi_writeback) {
    WARN_ON!(delayed_work_pending(&wb.dwork));
    percpu_counter_destroy_many(wb.stat, NR_WB_STAT_ITEMS);
    fprop_local_destroy_percpu(&wb.completions);
    }

//
// cgwb_lock protects bdi->cgwb_tree, blkcg->cgwb_list, offline_cgwbs and
// memcg->cgwb_list.  bdi->cgwb_tree is also RCU protected.
//
pub static mut cgwb_lock: usize = 0;
pub static mut cgwb_release_wq: *mut c_void = core::ptr::null_mut();
pub static mut offline_cgwbs: usize = 0;
// forward_decl: cleanup_offline_cgwbs_workfn;
pub static mut cleanup_offline_cgwbs_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn cgwb_free_rcu(rcu_head: *mut rcu_head) {
    let mut wb = container_of!(rcu_head, bdi_writeback, rcu);
    percpu_ref_exit(&wb.refcnt);
    kfree(wb);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_release_workfn(work: *mut work_struct) {
    let mut wb = container_of!(work, bdi_writeback,
    release_work);
    let mut bdi = wb.bdi;
    mutex_lock(&wb.bdi.cgwb_release_mutex);
    wb_shutdown(wb);
    css_put(wb.memcg_css);
// triggers blkg destruction if no online users left
    blkcg_unpin_online(wb.blkcg_css);
    css_put(wb.blkcg_css);
    mutex_unlock(&wb.bdi.cgwb_release_mutex);
    fprop_local_destroy_percpu(&wb.memcg_completions);
    spin_lock_irq(&cgwb_lock);
    list_del(&wb.offline_node);
    spin_unlock_irq(&cgwb_lock);
    wb_exit(wb);
    bdi_put(bdi);
    WARN_ON_ONCE!(!list_empty(&wb.b_attached));
    WARN_ON_ONCE!(work_pending(&wb.switch_work));
    call_rcu(&wb.rcu, cgwb_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_release(refcnt: *mut percpu_ref) {
    let mut wb = container_of!(refcnt, bdi_writeback,
    refcnt);
    queue_work(cgwb_release_wq, &wb.release_work);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_kill(wb: *mut bdi_writeback) {
    lockdep_assert_held(&cgwb_lock);
    WARN_ON!(!radix_tree_delete(&wb.bdi.cgwb_tree, wb.memcg_css.id));
    list_del(&wb.memcg_node);
    list_del(&wb.blkcg_node);
    list_add(&wb.offline_node, &offline_cgwbs);
    percpu_ref_kill(&wb.refcnt);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_remove_from_bdi_list(wb: *mut bdi_writeback) {
    spin_lock_irq(&cgwb_lock);
    list_del_rcu(&wb.bdi_node);
    spin_unlock_irq(&cgwb_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn cgwb_create(bdi: *mut backing_dev_info, memcg_css: *mut cgroup_subsys_state, gfp: gfp_t) -> c_int {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut blkcg_css: *mut c_void = core::ptr::null_mut();
    let mut memcg_cgwb_list = core::ptr::null_mut();
    let mut blkcg_cgwb_list = core::ptr::null_mut();
pub static mut wb: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
    memcg = mem_cgroup_from_css(memcg_css);
    blkcg_css = cgroup_get_e_css(memcg_css.cgroup, &io_cgrp_subsys);
    memcg_cgwb_list = &memcg.cgwb_list;
    blkcg_cgwb_list = blkcg_get_cgwb_list(blkcg_css);
// look up again under lock and discard on blkcg mismatch
    spin_lock_irqsave(&cgwb_lock, flags);
    wb = radix_tree_lookup(&bdi.cgwb_tree, memcg_css.id);
    if (wb && wb.blkcg_css != blkcg_css) {
    cgwb_kill(wb);
    wb = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(&cgwb_lock, flags);
    if (wb) {
// goto;
    }
// need to create a new one
    wb = kmalloc_obj(*wb, gfp);
    if (!wb) {
    ret = -ENOMEM;
// goto;
    }
    ret = wb_init(wb, bdi, gfp);
    if (ret) {
// goto;
    }
    ret = percpu_ref_init(&wb.refcnt, cgwb_release, 0, gfp);
    if (ret) {
// goto;
    }
    ret = fprop_local_init_percpu(&wb.memcg_completions, gfp);
    if (ret) {
// goto;
    }
    wb.memcg_css = memcg_css;
    wb.blkcg_css = blkcg_css;
    INIT_LIST_HEAD(&wb.b_attached);
    INIT_WORK(&wb.switch_work, inode_switch_wbs_work_fn);
    init_llist_head(&wb.switch_wbs_ctxs);
    INIT_WORK(&wb.release_work, cgwb_release_workfn);
    set_bit(WB_registered, &wb.state);
    bdi_get(bdi);
//
// The root wb determines the registered state of the whole bdi and
// memcg_cgwb_list and blkcg_cgwb_list's next pointers indicate
// whether they're still online.  Don't link @wb if any is dead.
// See wb_memcg_offline() and wb_blkcg_offline().
//
    ret = -ENODEV;
    spin_lock_irqsave(&cgwb_lock, flags);
    if (test_bit(WB_registered, &bdi.wb.state) &&
    blkcg_cgwb_list.next && memcg_cgwb_list.next) {
// we might have raced another instance of this function
    ret = radix_tree_insert(&bdi.cgwb_tree, memcg_css.id, wb);
    if (!ret) {
    list_add_tail_rcu(&wb.bdi_node, &bdi.wb_list);
    list_add(&wb.memcg_node, memcg_cgwb_list);
    list_add(&wb.blkcg_node, blkcg_cgwb_list);
    blkcg_pin_online(blkcg_css);
    css_get(memcg_css);
    css_get(blkcg_css);
    }
    }
    spin_unlock_irqrestore(&cgwb_lock, flags);
    if (ret) {
    if (ret == -EEXIST) {
    ret = 0;
    }
// goto;
    }
// goto;
// label;
    bdi_put(bdi);
    fprop_local_destroy_percpu(&wb.memcg_completions);
// label;
    percpu_ref_exit(&wb.refcnt);
// label;
    wb_exit(wb);
// label;
    kfree(wb);
// label;
    css_put(blkcg_css);
    return ret;
    }
//
// wb_get_lookup - get wb for a given memcg
// @bdi: target bdi
// @memcg_css: cgroup_subsys_state of the target memcg (must have positive ref)
//
// Try to get the wb for @memcg_css on @bdi.  The returned wb has its
// refcount incremented.
//
// This function uses css_get() on @memcg_css and thus expects its refcnt
// to be positive on invocation.  IOW, rcu_read_lock() protection on
// @memcg_css isn't enough.  try_get it before calling this function.
//
// A wb is keyed by its associated memcg.  As blkcg implicitly enables
// memcg on the default hierarchy, memcg association is guaranteed to be
// more specific (equal or descendant to the associated blkcg) and thus can
// identify both the memcg and blkcg associations.
//
// Because the blkcg associated with a memcg may change as blkcg is enabled
// and disabled closer to root in the hierarchy, each wb keeps track of
// both the memcg and blkcg associated with it and verifies the blkcg on
// each lookup.  On mismatch, the existing wb is discarded and a new one is
// created.
//
#[no_mangle]
pub unsafe extern "C" fn wb_get_lookup(bdi: *mut backing_dev_info, memcg_css: *mut cgroup_subsys_state) -> *mut c_void {
pub static mut wb: *mut c_void = core::ptr::null_mut();
    if (!memcg_css.parent) {
    return &bdi.wb;
    }
    rcu_read_lock();
    wb = radix_tree_lookup(&bdi.cgwb_tree, memcg_css.id);
    if (wb) {
pub static mut blkcg_css: *mut c_void = core::ptr::null_mut();
// see whether the blkcg association has changed
    blkcg_css = cgroup_get_e_css(memcg_css.cgroup, &io_cgrp_subsys);
    if (unlikely(wb.blkcg_css != blkcg_css || !wb_tryget(wb))) {
    wb = core::ptr::null_mut();
    }
    css_put(blkcg_css);
    }
    rcu_read_unlock();
    return wb;
    }
//
// wb_get_create - get wb for a given memcg, create if necessary
// @bdi: target bdi
// @memcg_css: cgroup_subsys_state of the target memcg (must have positive ref)
// @gfp: allocation mask to use
//
// Try to get the wb for @memcg_css on @bdi.  If it doesn't exist, try to
// create one.  See wb_get_lookup() for more details.
//
#[no_mangle]
pub unsafe extern "C" fn wb_get_create(bdi: *mut backing_dev_info, memcg_css: *mut cgroup_subsys_state, gfp: gfp_t) -> *mut c_void {
pub static mut wb: *mut c_void = core::ptr::null_mut();
    might_alloc(gfp);
    do {
    wb = wb_get_lookup(bdi, memcg_css);
    } while (!wb && !cgwb_create(bdi, memcg_css, gfp));
    return wb;
    }
#[no_mangle]
unsafe extern "C" fn cgwb_bdi_init(bdi: *mut backing_dev_info) -> c_int {
    let mut ret = 0;
    INIT_RADIX_TREE(&bdi.cgwb_tree, GFP_ATOMIC);
    mutex_init(&bdi.cgwb_release_mutex);
    init_rwsem(&bdi.wb_switch_rwsem);
    ret = wb_init(&bdi.wb, bdi, GFP_KERNEL);
    if (!ret) {
    bdi.wb.memcg_css = &root_mem_cgroup.css;
    bdi.wb.blkcg_css = blkcg_root_css;
    INIT_WORK(&bdi.wb.switch_work, inode_switch_wbs_work_fn);
    init_llist_head(&bdi.wb.switch_wbs_ctxs);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cgwb_bdi_unregister(bdi: *mut backing_dev_info) {
pub static mut iter: usize = 0;
pub static mut slot: *mut c_void = core::ptr::null_mut();
pub static mut wb: *mut c_void = core::ptr::null_mut();
    WARN_ON!(test_bit(WB_registered, &bdi.wb.state));
    spin_lock_irq(&cgwb_lock);
    radix_tree_for_each_slot(slot, &bdi.cgwb_tree, &iter, 0)
    cgwb_kill(*slot);
    spin_unlock_irq(&cgwb_lock);
    mutex_lock(&bdi.cgwb_release_mutex);
    spin_lock_irq(&cgwb_lock);
    while (!list_empty(&bdi.wb_list)) {
    wb = list_first_entry(&bdi.wb_list, bdi_writeback,
    bdi_node);
    spin_unlock_irq(&cgwb_lock);
    wb_shutdown(wb);
    spin_lock_irq(&cgwb_lock);
    }
    spin_unlock_irq(&cgwb_lock);
    mutex_unlock(&bdi.cgwb_release_mutex);
    }
//
// cleanup_offline_cgwbs_workfn - try to release dying cgwbs
//
// Try to release dying cgwbs by switching attached inodes to the nearest
// living ancestor's writeback. Processed wbs are placed at the end
// of the list to guarantee the forward progress.
//
#[no_mangle]
unsafe extern "C" fn cleanup_offline_cgwbs_workfn(work: *mut work_struct) {
pub static mut wb: *mut c_void = core::ptr::null_mut();
pub static mut processed: usize = 0;
    spin_lock_irq(&cgwb_lock);
    while (!list_empty(&offline_cgwbs)) {
    wb = list_first_entry(&offline_cgwbs, bdi_writeback,
    offline_node);
    list_move(&wb.offline_node, &processed);
//
// If wb is dirty, cleaning up the writeback by switching
// attached inodes will result in an effective removal of any
// bandwidth restrictions, which isn't the goal.  Instead,
// it can be postponed until the next time, when all io
// will be likely completed.  If in the meantime some inodes
// will get re-dirtied, they should be eventually switched to
// a new cgwb.
//
    if (wb_has_dirty_io(wb)) {
    continue;
    }
    if (!wb_tryget(wb)) {
    continue;
    }
    spin_unlock_irq(&cgwb_lock);
    while (cleanup_offline_cgwb(wb)) {
    cond_resched();
    }
    spin_lock_irq(&cgwb_lock);
    wb_put(wb);
    }
    if (!list_empty(&processed)) {
    list_splice_tail(&processed, &offline_cgwbs);
    }
    spin_unlock_irq(&cgwb_lock);
    }
//
// wb_memcg_offline - kill all wb's associated with a memcg being offlined
// @memcg: memcg being offlined
//
// Also prevents creation of any new wb's associated with @memcg.
//
#[no_mangle]
pub unsafe extern "C" fn wb_memcg_offline(memcg: *mut mem_cgroup) {
    let mut memcg_cgwb_list = &memcg.cgwb_list;
    let mut wb = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    spin_lock_irq(&cgwb_lock);
    list_for_each_entry_safe(wb, next, memcg_cgwb_list, memcg_node) {
    cgwb_kill(wb);
    }
    memcg_cgwb_list.next = core::ptr::null_mut();	/* prevent new wb's */
    spin_unlock_irq(&cgwb_lock);
    queue_work(system_dfl_wq, &cleanup_offline_cgwbs_work);
    }
//
// wb_blkcg_offline - kill all wb's associated with a blkcg being offlined
// @css: blkcg being offlined
//
// Also prevents creation of any new wb's associated with @blkcg.
//
#[no_mangle]
pub unsafe extern "C" fn wb_blkcg_offline(css: *mut cgroup_subsys_state) {
    let mut wb = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut list = blkcg_get_cgwb_list(css);
    spin_lock_irq(&cgwb_lock);
    list_for_each_entry_safe(wb, next, list, blkcg_node) {
    cgwb_kill(wb);
    }
    list.next = core::ptr::null_mut();	/* prevent new wb's */
    spin_unlock_irq(&cgwb_lock);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_bdi_register(bdi: *mut backing_dev_info) {
    spin_lock_irq(&cgwb_lock);
    list_add_tail_rcu(&bdi.wb.bdi_node, &bdi.wb_list);
    spin_unlock_irq(&cgwb_lock);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_init() -> c_int {
//
// There can be many concurrent release work items overwhelming
// system_percpu_wq.  Put them in a separate wq and limit concurrency.
// There's no point in executing many of these in parallel.
//
    cgwb_release_wq = alloc_workqueue("cgwb_release", WQ_PERCPU, 1);
    if (!cgwb_release_wq) {
    return -ENOMEM;
    }
    return 0;
    }
    subsys_initcall!(cgwb_init);

#[no_mangle]
unsafe extern "C" fn cgwb_bdi_init(bdi: *mut backing_dev_info) -> c_int {
    return wb_init(&bdi.wb, bdi, GFP_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn cgwb_bdi_unregister(bdi: *mut backing_dev_info) { }
#[no_mangle]
unsafe extern "C" fn cgwb_bdi_register(bdi: *mut backing_dev_info) {
    list_add_tail_rcu(&bdi.wb.bdi_node, &bdi.wb_list);
    }
#[no_mangle]
unsafe extern "C" fn cgwb_remove_from_bdi_list(wb: *mut bdi_writeback) {
    list_del_rcu(&wb.bdi_node);
    }

#[no_mangle]
pub unsafe extern "C" fn bdi_init(bdi: *mut backing_dev_info) -> c_int {
    bdi.dev = core::ptr::null_mut();
    kref_init(&bdi.refcnt);
    bdi.min_ratio = 0;
    bdi.max_ratio = 100 * BDI_RATIO_SCALE;
    bdi.max_prop_frac = FPROP_FRAC_BASE;
    INIT_LIST_HEAD(&bdi.bdi_list);
    INIT_LIST_HEAD(&bdi.wb_list);
    init_waitqueue_head(&bdi.wb_waitq);
    bdi.last_bdp_sleep = jiffies;
    return cgwb_bdi_init(bdi);
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_alloc(node_id: c_int) -> *mut c_void {
pub static mut bdi: *mut c_void = core::ptr::null_mut();
    bdi = kzalloc_node(sizeof!(*bdi), GFP_KERNEL, node_id);
    if (!bdi) {
    return core::ptr::null_mut();
    }
    if (bdi_init(bdi)) {
    kfree(bdi);
    return core::ptr::null_mut();
    }
    bdi.capabilities = BDI_CAP_WRITEBACK;
    bdi.ra_pages = VM_READAHEAD_PAGES;
    bdi.io_pages = VM_READAHEAD_PAGES;
    return bdi;
    }
    EXPORT_SYMBOL(bdi_alloc);
    static struct rb_node **bdi_lookup_rb_node(u64 id, rb_node **parentp)
    {
    let mut p = &bdi_tree.rb_node;
    let mut parent = core::ptr::null_mut();
pub static mut bdi: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&bdi_lock);
    while (*p) {
    parent = *p;
    bdi = rb_entry(parent, backing_dev_info, rb_node);
    if (bdi.id > id) {
    p = &(*p).rb_left;
    }

    else if (bdi.id < id) {
    p = &(*p).rb_right;
    }
    else {
    break;
    }
    }
    if (parentp) {
// parentp = parent;
    }
    return p;
    }
//
// bdi_get_by_id - lookup and get bdi from its id
// @id: bdi id to lookup
//
// Find bdi matching @id and get it.  Returns NULL if the matching bdi
// doesn't exist or is already unregistered.
//
#[no_mangle]
pub unsafe extern "C" fn bdi_get_by_id(id: u64) -> *mut c_void {
    let mut bdi = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    spin_lock_bh(&bdi_lock);
    p = bdi_lookup_rb_node(id, core::ptr::null_mut());
    if (*p) {
    bdi = rb_entry(*p, backing_dev_info, rb_node);
    bdi_get(bdi);
    }
    spin_unlock_bh(&bdi_lock);
    return bdi;
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_register_va(bdi: *mut backing_dev_info, fmt: *const c_char, args: va_list) -> c_int {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    if (bdi.dev)	/* The driver needs to use separate queues per device */ {
    return 0;
    }
    vsnprintf(bdi.dev_name, sizeof!(bdi.dev_name), fmt, args);
    dev = device_create(&bdi_class, core::ptr::null_mut(), MKDEV(0, 0), bdi, bdi.dev_name);
    if (IS_ERR(dev)) {
    return PTR_ERR(dev);
    }
    cgwb_bdi_register(bdi);
    bdi.dev = dev;
    bdi_debug_register(bdi, dev_name(dev));
    set_bit(WB_registered, &bdi.wb.state);
    spin_lock_bh(&bdi_lock);
    bdi.id = bdi_id_cursor += 1;
    p = bdi_lookup_rb_node(bdi.id, &parent);
    rb_link_node(&bdi.rb_node, parent, p);
    rb_insert_color(&bdi.rb_node, &bdi_tree);
    list_add_tail_rcu(&bdi.bdi_list, &bdi_list);
    spin_unlock_bh(&bdi_lock);
    trace_writeback_bdi_register(bdi);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_register(bdi: *mut backing_dev_info, fmt: *const c_char, ...) -> c_int {
    let mut args;
    let mut ret = 0;
    va_start(args, fmt);
    ret = bdi_register_va(bdi, fmt, args);
    va_end(args);
    return ret;
    }
    EXPORT_SYMBOL(bdi_register);
#[no_mangle]
pub unsafe extern "C" fn bdi_set_owner(bdi: *mut backing_dev_info, owner: *mut device) {
    WARN_ON_ONCE!(bdi.owner);
    bdi.owner = owner;
    get_device(owner);
    }
//
// Remove bdi from bdi_list, and ensure that it is no longer visible
//
#[no_mangle]
unsafe extern "C" fn bdi_remove_from_list(bdi: *mut backing_dev_info) {
    spin_lock_bh(&bdi_lock);
    rb_erase(&bdi.rb_node, &bdi_tree);
    list_del_rcu(&bdi.bdi_list);
    spin_unlock_bh(&bdi_lock);
    synchronize_rcu_expedited();
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_unregister(bdi: *mut backing_dev_info) {
// make sure nobody finds us on the bdi_list anymore
    bdi_remove_from_list(bdi);
    wb_shutdown(&bdi.wb);
    cgwb_bdi_unregister(bdi);
//
// If this BDI's min ratio has been set, use bdi_set_min_ratio() to
// update the global bdi_min_ratio.
//
    if (bdi.min_ratio) {
    bdi_set_min_ratio(bdi, 0);
    }
    if (bdi.dev) {
    bdi_debug_unregister(bdi);
    device_unregister(bdi.dev);
    bdi.dev = core::ptr::null_mut();
    }
    if (bdi.owner) {
    put_device(bdi.owner);
    bdi.owner = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL(bdi_unregister);
#[no_mangle]
unsafe extern "C" fn release_bdi(ref: *mut kref) {
    let mut bdi = container_of!(ref, backing_dev_info, refcnt);
    WARN_ON_ONCE!(test_bit(WB_registered, &bdi.wb.state));
    WARN_ON_ONCE!(bdi.dev);
    wb_exit(&bdi.wb);
    kfree(bdi);
    }
#[no_mangle]
pub unsafe extern "C" fn bdi_put(bdi: *mut backing_dev_info) {
    kref_put(&bdi.refcnt, release_bdi);
    }
    EXPORT_SYMBOL(bdi_put);
#[no_mangle]
pub unsafe extern "C" fn inode_to_bdi(inode: *mut inode) -> *mut c_void {
pub static mut sb: *mut c_void = core::ptr::null_mut();
    if (!inode) {
    return &noop_backing_dev_info;
    }
    sb = inode.i_sb;

    if (sb_is_blkdev_sb(sb)) {
    return I_BDEV(inode).bd_disk.bdi;
    }

    return sb.s_bdi;
    }
    EXPORT_SYMBOL(inode_to_bdi);
    const char *bdi_dev_name(backing_dev_info *bdi)
    {
    if (!bdi || !bdi.dev) {
    return bdi_unknown_name;
    }
    return bdi.dev_name;
    }
    EXPORT_SYMBOL_GPL(bdi_dev_name);