//! Automatically rewritten from C to Rust
//! Source: mm/shrinker_debug.c
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

// defined in vmscan.c
extern "C" { pub static mut shrinker_mutex: usize; }
extern "C" { pub static mut shrinker_list: usize; }
pub static mut shrinker_debugfs_ida: usize = 0;
pub static mut shrinker_debugfs_root: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn shrinker_count_objects(shrinker: *mut shrinker, memcg: *mut mem_cgroup, count_per_node: *mut c_ulong) -> c_ulong {
    unsigned long nr, total = 0;
    let mut nid = 0;
    for_each_node(nid) {
    if (nid == 0 || (shrinker.flags & SHRINKER_NUMA_AWARE)) {
pub static mut shrink_control: usize = 0;
    nr = shrinker.count_objects(shrinker, &sc);
    if (nr == SHRINK_EMPTY) {
    nr = 0;
    }
    } else {
    nr = 0;
    }
    count_per_node[nid] = nr;
    total += nr;
    }
    return total;
    }
#[no_mangle]
unsafe extern "C" fn shrinker_debugfs_count_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut shrinker = m.private;
pub static mut count_per_node: *mut c_void = core::ptr::null_mut();
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut total = 0;
    let mut memcg_aware = 0;
pub static mut ret: c_int = 0;
    count_per_node = kcalloc(nr_node_ids, sizeof!(unsigned long), GFP_KERNEL);
    if (!count_per_node) {
    return -ENOMEM;
    }
    memcg_aware = shrinker.flags & SHRINKER_MEMCG_AWARE;
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    if (memcg && !mem_cgroup_online(memcg)) {
    continue;
    }
    total = shrinker_count_objects(shrinker,
    memcg_aware ? memcg : core::ptr::null_mut(),
    count_per_node);
    if (total) {
    seq_printf(m, "%llu", mem_cgroup_id(memcg));
    for_each_node(nid) {
    seq_printf(m, " %lu", count_per_node[nid]);
    }
    seq_putc(m, '\n');
    }
    if (!memcg_aware) {
    mem_cgroup_iter_break(core::ptr::null_mut(), memcg);
    break;
    }
    if (signal_pending(current)) {
    mem_cgroup_iter_break(core::ptr::null_mut(), memcg);
    ret = -EINTR;
    break;
    }
    } while ((memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut())) != core::ptr::null_mut());
    kfree(count_per_node);
    return ret;
    }
pub static mut shrinker_debugfs_count: usize = 0;
#[no_mangle]
unsafe extern "C" fn shrinker_debugfs_scan_open(inode: *mut inode, file: *mut file) -> c_int {
    file.private_data = inode.i_private;
    return nonseekable_open(inode, file);
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_debugfs_scan_write(file: *mut file, buf: *mut c_char, size: size_t, pos: *mut loff_t) -> ssize_t {
    let mut shrinker = file.private_data;
pub static mut nr_to_scan: c_ulong = 0;
    let mut id = 0;
pub static mut shrink_control: usize = 0;
    let mut memcg = core::ptr::null_mut();
    let mut nid = 0;
    char kbuf[72];
    read_len = min(size, sizeof!(kbuf) - 1);
    if (copy_from_user(kbuf, buf, read_len)) {
    return -EFAULT;
    }
    kbuf[read_len] = '\0';
    if (sscanf(kbuf, "%llu %d %lu", &id, &nid, &nr_to_scan) != 3) {
    return -EINVAL;
    }
    if (nid < 0 || nid >= nr_node_ids) {
    return -EINVAL;
    }
    if (nr_to_scan == 0) {
    return size;
    }
    if (shrinker.flags & SHRINKER_MEMCG_AWARE) {
    memcg = mem_cgroup_get_from_id(id);
    if (!memcg) {
    return -ENOENT;
    }
    if (!mem_cgroup_online(memcg)) {
    mem_cgroup_put(memcg);
    return -ENOENT;
    }
    } else if (id != 0) {
    return -EINVAL;
    }
    sc.nid = nid;
    sc.memcg = memcg;
    sc.nr_to_scan = nr_to_scan;
    sc.nr_scanned = nr_to_scan;
    shrinker.scan_objects(shrinker, &sc);
    mem_cgroup_put(memcg);
    return size;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn shrinker_debugfs_add(shrinker: *mut shrinker) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    char buf[128];
    let mut id = 0;
    lockdep_assert_held(&shrinker_mutex);
// debugfs isn't initialized yet, add debugfs entries later.
    if (!shrinker_debugfs_root) {
    return 0;
    }
    id = ida_alloc(&shrinker_debugfs_ida, GFP_KERNEL);
    if (id < 0) {
    return id;
    }
    shrinker.debugfs_id = id;
    snprintf(buf, sizeof!(buf), "%s-%d", shrinker.name, id);
// create debugfs entry
    entry = debugfs_create_dir(buf, shrinker_debugfs_root);
    if (IS_ERR(entry)) {
    ida_free(&shrinker_debugfs_ida, id);
    return PTR_ERR(entry);
    }
    shrinker.debugfs_entry = entry;
    if (shrinker.count_objects) {
    debugfs_create_file("count", 0440, entry, shrinker,
    &shrinker_debugfs_count_fops);
    }
    if (shrinker.scan_objects) {
    debugfs_create_file("scan", 0220, entry, shrinker,
    &shrinker_debugfs_scan_fops);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_debugfs_rename(shrinker: *mut shrinker, fmt: *const c_char, ...) -> c_int {
    let mut new = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
    let mut ap;
pub static mut ret: c_int = 0;
    va_start(ap, fmt);
    new = kvasprintf_const(GFP_KERNEL, fmt, ap);
    va_end(ap);
    if (!new) {
    return -ENOMEM;
    }
    mutex_lock(&shrinker_mutex);
    old = shrinker.name;
    shrinker.name = new;
    ret = debugfs_change_name(shrinker.debugfs_entry, "%s-%d",
    shrinker.name, shrinker.debugfs_id);
    if (ret) {
    shrinker.name = old;
    kfree_const(new);
    } else {
    kfree_const(old);
    }
    mutex_unlock(&shrinker_mutex);
    return ret;
    }
    EXPORT_SYMBOL(shrinker_debugfs_rename);
#[no_mangle]
pub unsafe extern "C" fn shrinker_debugfs_detach(shrinker: *mut shrinker, debugfs_id: *mut c_int) -> *mut c_void {
    let mut entry = shrinker.debugfs_entry;
    lockdep_assert_held(&shrinker_mutex);
// debugfs_id = entry ? shrinker->debugfs_id : -1;
    shrinker.debugfs_entry = core::ptr::null_mut();
    return entry;
    }
#[no_mangle]
pub unsafe extern "C" fn shrinker_debugfs_remove(debugfs_entry: *mut dentry, debugfs_id: c_int) {
    debugfs_remove_recursive(debugfs_entry);
    ida_free(&shrinker_debugfs_ida, debugfs_id);
    }
#[no_mangle]
unsafe extern "C" fn shrinker_debugfs_init() -> c_int {
pub static mut shrinker: *mut c_void = core::ptr::null_mut();
pub static mut dentry: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    dentry = debugfs_create_dir("shrinker", core::ptr::null_mut());
    if (IS_ERR(dentry)) {
    return PTR_ERR(dentry);
    }
    shrinker_debugfs_root = dentry;
// Create debugfs entries for shrinkers registered at boot
    mutex_lock(&shrinker_mutex);
    list_for_each_entry(shrinker, &shrinker_list, list) {
    if (!shrinker.debugfs_entry) {
    }
    ret = shrinker_debugfs_add(shrinker);
    if (ret) {
    break;
    }
    }
    mutex_unlock(&shrinker_mutex);
    return ret;
    }
    late_initcall!(shrinker_debugfs_init);