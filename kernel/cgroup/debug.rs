//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/debug.c
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
// Debug controller
//
// WARNING: This controller is for cgroup core debugging only.
// Its interfaces are unstable and subject to changes at any time.
//

#[no_mangle]
pub unsafe extern "C" fn debug_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut c_void {
    let mut css = kzalloc_obj(*css);
    if (!css) {
    return ERR_PTR(-ENOMEM);
    }
    return css;
    }
#[no_mangle]
unsafe extern "C" fn debug_css_free(css: *mut cgroup_subsys_state) {
    kfree(css);
    }
//
// debug_taskcount_read - return the number of tasks in a cgroup.
// @cgrp: the cgroup in question
//
#[no_mangle]
pub unsafe extern "C" fn debug_taskcount_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return cgroup_task_count(css.cgroup);
    }
#[no_mangle]
unsafe extern "C" fn current_css_set_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut of = seq.private;
pub static mut cset: *mut c_void = core::ptr::null_mut();
pub static mut ss: *mut c_void = core::ptr::null_mut();
pub static mut css: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut refcnt = 0;
    if (!cgroup_kn_lock_live(of.kn, false)) {
    return -ENODEV;
    }
    spin_lock_irq(&css_set_lock);
    cset = task_css_set(current);
    refcnt = refcount_read(&cset.refcount);
    seq_printf(seq, "css_set %pK %d", cset, refcnt);
    if (refcnt > cset.nr_tasks) {
    seq_printf(seq, " +%d", refcnt - cset.nr_tasks);
    }
    seq_puts(seq, "\n");
//
// Print the css'es stored in the current css_set.
//
    for_each_subsys(ss, i) {
    css = cset.subsys[ss.id];
    if (!css) {
    continue;
    }
    seq_printf(seq, "%2d: %-4s\t- %p[%d]\n", ss.id, ss.name,
    css, css.id);
    }
    spin_unlock_irq(&css_set_lock);
    cgroup_kn_unlock(of.kn);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn current_css_set_refcount_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    let mut count = 0;
    rcu_read_lock();
    count = refcount_read(&task_css_set(current).refcount);
    rcu_read_unlock();
    return count;
    }
#[no_mangle]
unsafe extern "C" fn current_css_set_cg_links_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut cset: *mut c_void = core::ptr::null_mut();
pub static mut name_buf: *mut c_void = core::ptr::null_mut();
    name_buf = kmalloc(NAME_MAX + 1, GFP_KERNEL);
    if (!name_buf) {
    return -ENOMEM;
    }
    spin_lock_irq(&css_set_lock);
    cset = task_css_set(current);
    list_for_each_entry(link, &cset.cgrp_links, cgrp_link) {
    let mut c = link.cgrp;
    cgroup_name(c, name_buf, NAME_MAX + 1);
    seq_printf(seq, "Root %d group %s\n",
    c.root.hierarchy_id, name_buf);
    }
    spin_unlock_irq(&css_set_lock);
    kfree(name_buf);
    return 0;
    }
pub const MAX_TASKS_SHOWN_PER_CSS: c_int = 25;
#[no_mangle]
unsafe extern "C" fn cgroup_css_links_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut css = seq_css(seq);
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut dead_cnt: c_int = 0;
    spin_lock_irq(&css_set_lock);
    list_for_each_entry(link, &css.cgroup.cset_links, cset_link) {
    let mut cset = link.cset;
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut count: c_int = 0;
pub static mut refcnt: c_int = 0;
//
// Print out the proc_cset and threaded_cset relationship
// and highlight difference between refcount and task_count.
//
    seq_printf(seq, "css_set %pK", cset);
    if (rcu_dereference_protected(cset.dom_cset, 1) != cset) {
    threaded_csets += 1;
    seq_printf(seq, "=>%pK", cset.dom_cset);
    }
    if (!list_empty(&cset.threaded_csets)) {
pub static mut tcset: *mut c_void = core::ptr::null_mut();
pub static mut idx: c_int = 0;
    list_for_each_entry(tcset, &cset.threaded_csets,
    threaded_csets_node) {
    seq_puts(seq, idx ? "," : "<=");
    seq_printf(seq, "%pK", tcset);
    idx += 1;
    }
    } else {
    seq_printf(seq, " %d", refcnt);
    if (refcnt - cset.nr_tasks > 0) {
pub static mut extra: c_int = 0;
    seq_printf(seq, " +%d", extra);
//
// Take out the one additional reference in
// init_css_set.
//
    if (cset == &init_css_set) {
    extra -= 1;
    }
    extra_refs += extra;
    }
    }
    seq_puts(seq, "\n");
    list_for_each_entry(task, &cset.tasks, cg_list) {
    if (count++ <= MAX_TASKS_SHOWN_PER_CSS) {
    seq_printf(seq, "  task %d\n",
    task_pid_vnr(task));
    }
    }
    list_for_each_entry(task, &cset.mg_tasks, cg_list) {
    if (count++ <= MAX_TASKS_SHOWN_PER_CSS) {
    seq_printf(seq, "  task %d\n",
    task_pid_vnr(task));
    }
    }
// show # of overflowed tasks
    if (count > MAX_TASKS_SHOWN_PER_CSS) {
    seq_printf(seq, "  ... (%d)\n",
    count - MAX_TASKS_SHOWN_PER_CSS);
    }
    if (cset.dead) {
    seq_puts(seq, "    [dead]\n");
    dead_cnt += 1;
    }
    WARN_ON!(count != cset.nr_tasks);
    }
    spin_unlock_irq(&css_set_lock);
    if (!dead_cnt && !extra_refs && !threaded_csets) {
    return 0;
    }
    seq_puts(seq, "\n");
    if (threaded_csets) {
    seq_printf(seq, "threaded css_sets = %d\n", threaded_csets);
    }
    if (extra_refs) {
    seq_printf(seq, "extra references = %d\n", extra_refs);
    }
    if (dead_cnt) {
    seq_printf(seq, "dead css_sets = %d\n", dead_cnt);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_subsys_states_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut of = seq.private;
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut ss: *mut c_void = core::ptr::null_mut();
pub static mut css: *mut c_void = core::ptr::null_mut();
    char pbuf[16];
    let mut i = 0;
    cgrp = cgroup_kn_lock_live(of.kn, false);
    if (!cgrp) {
    return -ENODEV;
    }
    for_each_subsys(ss, i) {
    css = rcu_dereference_check(cgrp.subsys[ss.id], true);
    if (!css) {
    continue;
    }
    pbuf[0] = '\0';
// Show the parent CSS if applicable
    if (css.parent) {
    snprintf(pbuf, sizeof!(pbuf) - 1, " P=%d",
    css.parent.id);
    }
    seq_printf(seq, "%2d: %-4s\t- %p[%d] %d%s\n", ss.id, ss.name,
    css, css.id,
    atomic_read(&css.online_cnt), pbuf);
    }
    cgroup_kn_unlock(of.kn);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_masks_read_one(seq: *mut seq_file, name: *mut c_char, mask: u32) {
pub static mut ss: *mut c_void = core::ptr::null_mut();
    let mut ssid = 0;
pub static mut first: bool = true;
    seq_printf(seq, "%-17s: ", name);
    for_each_subsys(ss, ssid) {
    if (!(mask & (1 << ssid))) {
    continue;
    }
    if (!first) {
    seq_puts(seq, ", ");
    }
    seq_puts(seq, ss.name);
    first = false;
    }
    seq_putc(seq, '\n');
    }
#[no_mangle]
unsafe extern "C" fn cgroup_masks_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut of = seq.private;
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    cgrp = cgroup_kn_lock_live(of.kn, false);
    if (!cgrp) {
    return -ENODEV;
    }
    cgroup_masks_read_one(seq, "subtree_control", cgrp.subtree_control);
    cgroup_masks_read_one(seq, "subtree_ss_mask", cgrp.subtree_ss_mask);
    cgroup_kn_unlock(of.kn);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn releasable_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return (!cgroup_is_populated(css.cgroup) &&
    !css_has_online_children(&css.cgroup.self));
    }
pub static mut cftype: usize = 0;
pub static mut cftype: usize = 0;
pub static mut cgroup_subsys: usize = 0;
//
// On v2, debug is an implicit controller enabled by "cgroup_debug" boot
// parameter.
//
#[no_mangle]
pub unsafe extern "C" fn enable_debug_cgroup()  {
    debug_cgrp_subsys.dfl_cftypes = debug_files;
    debug_cgrp_subsys.implicit_on_dfl = true;
    debug_cgrp_subsys.threaded = true;
    }