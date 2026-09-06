//! Automatically rewritten from C to Rust
//! Source: block/disk-events.c
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
// Disk events - monitor disk events like media change and eject request.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_events {
//     pub /: *mut *mut list_head node; / all disk_event's,
//     pub /: *mut *mut *mut gendisk disk; / the associated disk,
    pub lock: spinlock_t,
//     pub /: *mut *mut mutex block_mutex; / protects blocking,
//     pub /: *mut *mut int block; / event blocking depth,
//     pub /: *mut *mut unsigned int pending; / events already sent out,
//     pub /: *mut *mut unsigned int clearing; / events being cleared,
//     pub /: *mut *mut long poll_msecs; / interval, -1 for default,
    pub dwork: delayed_work,
}

    static const char *disk_events_strs[] = {
    [ilog2(DISK_EVENT_MEDIA_CHANGE)]	= "media_change",
    [ilog2(DISK_EVENT_EJECT_REQUEST)]	= "eject_request",
    };
    static char *disk_uevents[] = {
    [ilog2(DISK_EVENT_MEDIA_CHANGE)]	= "DISK_MEDIA_CHANGE=1",
    [ilog2(DISK_EVENT_EJECT_REQUEST)]	= "DISK_EJECT_REQUEST=1",
    };
// list of all disk_events
pub static mut disk_events_mutex: usize = 0;
pub static mut disk_events: usize = 0;
// disable in-kernel polling by default
    static unsigned long disk_events_dfl_poll_msecs;
#[no_mangle]
unsafe extern "C" fn disk_events_poll_jiffies(disk: *mut gendisk) -> c_ulong {
    let mut ev = disk.ev;
pub static mut intv_msecs: c_long = 0;
//
// If device-specific poll interval is set, always use it.  If
// the default is being used, poll if the POLL flag is set.
//
    if (ev.poll_msecs >= 0) {
    intv_msecs = ev.poll_msecs;
    }

    else if (disk.event_flags & DISK_EVENT_FLAG_POLL) {
    intv_msecs = disk_events_dfl_poll_msecs;
    }
    return msecs_to_jiffies(intv_msecs);
    }
//
// disk_block_events - block and flush disk event checking
// @disk: disk to block events for
//
// On return from this function, it is guaranteed that event checking
// isn't in progress and won't happen until unblocked by
// disk_unblock_events().  Events blocking is counted and the actual
// unblocking happens after the matching number of unblocks are done.
//
// Note that this intentionally does not block event checking from
// disk_clear_events().
//
// CONTEXT:
// Might sleep.
//
#[no_mangle]
pub unsafe extern "C" fn disk_block_events(disk: *mut gendisk) {
    let mut ev = disk.ev;
    let mut flags = 0;
    let mut cancel = 0;
    if (!ev) {
    return;
    }
//
// Outer mutex ensures that the first blocker completes canceling
// the event work before further blockers are allowed to finish.
//
    mutex_lock(&ev.block_mutex);
    spin_lock_irqsave(&ev.lock, flags);
    cancel = !ev.block += 1;
    spin_unlock_irqrestore(&ev.lock, flags);
    if (cancel) {
    cancel_delayed_work_sync(&disk.ev.dwork);
    }
    mutex_unlock(&ev.block_mutex);
    }
#[no_mangle]
unsafe extern "C" fn __disk_unblock_events(disk: *mut gendisk, check_now: bool) {
    let mut ev = disk.ev;
    let mut intv = 0;
    let mut flags = 0;
    spin_lock_irqsave(&ev.lock, flags);
    if (WARN_ON_ONCE!(ev.block <= 0)) {
// goto;
    }
    if (--ev.block) {
// goto;
    }
    intv = disk_events_poll_jiffies(disk);
    if (check_now) {
    queue_delayed_work(system_freezable_power_efficient_wq,
    &ev.dwork, 0);
    }

    else if (intv) {
    queue_delayed_work(system_freezable_power_efficient_wq,
    &ev.dwork, intv);
    }
// label;
    spin_unlock_irqrestore(&ev.lock, flags);
    }
//
// disk_unblock_events - unblock disk event checking
// @disk: disk to unblock events for
//
// Undo disk_block_events().  When the block count reaches zero, it
// starts events polling if configured.
//
// CONTEXT:
// Don't care.  Safe to call from irq context.
//
#[no_mangle]
pub unsafe extern "C" fn disk_unblock_events(disk: *mut gendisk) {
    if (disk.ev) {
    __disk_unblock_events(disk, false);
    }
    }
//
// disk_flush_events - schedule immediate event checking and flushing
// @disk: disk to check and flush events for
// @mask: events to flush
//
// Schedule immediate event checking on @disk if not blocked.  Events in
// @mask are scheduled to be cleared from the driver.  Note that this
// doesn't clear the events from @disk->ev.
//
// CONTEXT:
// If @mask is non-zero must be called with disk->open_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn disk_flush_events(disk: *mut gendisk, mask: c_uint) {
    let mut ev = disk.ev;
    if (!ev) {
    return;
    }
    spin_lock_irq(&ev.lock);
    ev.clearing |= mask;
    if (!ev.block) {
    mod_delayed_work(system_freezable_power_efficient_wq,
    &ev.dwork, 0);
    }
    spin_unlock_irq(&ev.lock);
    }
//
// Tell userland about new events.  Only the events listed in @disk->events are
// reported, and only if DISK_EVENT_FLAG_UEVENT is set.  Otherwise, events are
// processed internally but never get reported to userland.
//
#[no_mangle]
unsafe extern "C" fn disk_event_uevent(disk: *mut gendisk, events: c_uint) {
    char *envp[ARRAY_SIZE!(disk_uevents) + 1] = { };
pub static mut nr_events: c_int = 0;
    for (i = 0; i < ARRAY_SIZE!(disk_uevents); i++) {
    if (events & disk.events & (1 << i))
    envp[nr_events++] = disk_uevents[i];
    }
    if (nr_events) {
    kobject_uevent_env(&disk_to_dev(disk).kobj, KOBJ_CHANGE, envp);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn disk_check_events(ev: *mut disk_events, clearing_ptr: *mut c_uint) {
    let mut disk = ev.disk;
pub static mut clearing: c_uint = 0;
    let mut events = 0;
    let mut intv = 0;
// check events
    events = disk.fops.check_events(disk, clearing);
// accumulate pending events and schedule next poll if necessary
    spin_lock_irq(&ev.lock);
    events &= ~ev.pending;
    ev.pending |= events;
// clearing_ptr &= ~clearing;
    intv = disk_events_poll_jiffies(disk);
    if (!ev.block && intv) {
    queue_delayed_work(system_freezable_power_efficient_wq,
    &ev.dwork, intv);
    }
    spin_unlock_irq(&ev.lock);
    if (events & DISK_EVENT_MEDIA_CHANGE) {
    inc_diskseq(disk);
    }
    if (disk.event_flags & DISK_EVENT_FLAG_UEVENT) {
    disk_event_uevent(disk, events);
    }
    }
//
// disk_clear_events - synchronously check, clear and return pending events
// @disk: disk to fetch and clear events from
// @mask: mask of events to be fetched and cleared
//
// Disk events are synchronously checked and pending events in @mask
// are cleared and returned.  This ignores the block count.
//
// CONTEXT:
// Might sleep.
//
#[no_mangle]
unsafe extern "C" fn disk_clear_events(disk: *mut gendisk, mask: c_uint) -> c_uint {
    let mut ev = disk.ev;
    let mut pending = 0;
pub static mut clearing: c_uint = 0;
    if (!ev) {
    return 0;
    }
    disk_block_events(disk);
//
// store the union of mask and ev->clearing on the stack so that the
// race with disk_flush_events does not cause ambiguity (ev->clearing
// can still be modified even if events are blocked).
//
    spin_lock_irq(&ev.lock);
    clearing |= ev.clearing;
    ev.clearing = 0;
    spin_unlock_irq(&ev.lock);
    disk_check_events(ev, &clearing);
//
// if ev->clearing is not 0, the disk_flush_events got called in the
// middle of this function, so we want to run the workfn without delay.
//
    __disk_unblock_events(disk, ev.clearing ? true : false);
// then, fetch and clear pending events
    spin_lock_irq(&ev.lock);
    pending = ev.pending & mask;
    ev.pending &= ~mask;
    spin_unlock_irq(&ev.lock);
    WARN_ON_ONCE!(clearing & mask);
    return pending;
    }
//
// disk_check_media_change - check if a removable media has been changed
// @disk: gendisk to check
//
// Returns %true and marks the disk for a partition rescan whether a removable
// media has been changed, and %false if the media did not change.
//
#[no_mangle]
pub unsafe extern "C" fn disk_check_media_change(disk: *mut gendisk) -> bool {
    let mut events = 0;
    events = disk_clear_events(disk, DISK_EVENT_MEDIA_CHANGE |
    DISK_EVENT_EJECT_REQUEST);
    if (events & DISK_EVENT_MEDIA_CHANGE) {
    set_bit(GD_NEED_PART_SCAN, &disk.state);
    return true;
    }
    return false;
    }
    EXPORT_SYMBOL(disk_check_media_change);
//
// disk_force_media_change - force a media change event
// @disk: the disk which will raise the event
//
// Should be called when the media changes for @disk.  Generates a uevent
// and attempts to free all dentries and inodes and invalidates all block
// device page cache entries in that case.
//
// Callers that need a partition re-scan should arrange for one explicitly.
//
#[no_mangle]
pub unsafe extern "C" fn disk_force_media_change(disk: *mut gendisk) {
    disk_event_uevent(disk, DISK_EVENT_MEDIA_CHANGE);
    inc_diskseq(disk);
    bdev_mark_dead(disk.part0, true);
    }
    EXPORT_SYMBOL_GPL(disk_force_media_change);
//
// Separate this part out so that a different pointer for clearing_ptr can be
// passed in for disk_clear_events.
//
#[no_mangle]
unsafe extern "C" fn disk_events_workfn(work: *mut work_struct) {
    let mut dwork = to_delayed_work(work);
    let mut ev = container_of!(dwork, disk_events, dwork);
    disk_check_events(ev, &ev.clearing);
    }
//
// A disk events enabled device has the following sysfs nodes under
// its /sys/block/X/ directory.
//
// events		: list of all supported events
// events_async		: list of events which can be detected w/o polling
// (always empty, only for backwards compatibility)
// events_poll_msecs	: polling interval, 0: disable, -1: system default
//
#[no_mangle]
unsafe extern "C" fn __disk_events_show(events: c_uint, buf: *mut c_char) -> isize {
    let mut delim = "";
pub static mut pos: isize = 0;
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE!(disk_events_strs); i++) {
    if (events & (1 << i)) {
    }
    pos += sprintf(buf + pos, "%s%s",
    delim, disk_events_strs[i]);
    delim = " ";
    }
    if (pos) {
    pos += sprintf(buf + pos, "\n");
    }
    return pos;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_events_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    if (!(disk.event_flags & DISK_EVENT_FLAG_UEVENT)) {
    return 0;
    }
    return __disk_events_show(disk.events, buf);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_events_async_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_events_poll_msecs_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    if (!disk.ev) {
    return sprintf(buf, "-1\n");
    }
    return sprintf(buf, "%ld\n", disk.ev.poll_msecs);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_events_poll_msecs_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    let mut intv = 0;
    if (!count || !sscanf(buf, "%ld", &intv)) {
    return -EINVAL;
    }
    if (intv < 0 && intv != -1) {
    return -EINVAL;
    }
    if (!disk.ev) {
    return -ENODEV;
    }
    disk_block_events(disk);
    disk.ev.poll_msecs = intv;
    __disk_unblock_events(disk, true);
    return count;
    }
    DEVICE_ATTR(events, 0444, disk_events_show, core::ptr::null_mut());
    DEVICE_ATTR(events_async, 0444, disk_events_async_show, core::ptr::null_mut());
    DEVICE_ATTR(events_poll_msecs, 0644, disk_events_poll_msecs_show,
    disk_events_poll_msecs_store);
//
// The default polling interval can be specified by the kernel
// parameter block.events_dfl_poll_msecs which defaults to 0
// (disable).  This can also be modified runtime by writing to
// /sys/module/block/parameters/events_dfl_poll_msecs.
//
#[no_mangle]
pub unsafe extern "C" fn disk_events_set_dfl_poll_msecs(val: *mut c_char, kp: *mut kernel_param) -> c_int {
pub static mut ev: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = param_set_ulong(val, kp);
    if (ret < 0) {
    return ret;
    }
    mutex_lock(&disk_events_mutex);
    list_for_each_entry(ev, &disk_events, node) {
    disk_flush_events(ev.disk, 0);
    }
    mutex_unlock(&disk_events_mutex);
    return 0;
    }
pub static mut kernel_param_ops: usize = 0;

    module_param_cb!(events_dfl_poll_msecs, &disk_events_dfl_poll_msecs_param_ops,
    &disk_events_dfl_poll_msecs, 0644);
//
// disk_{alloc|add|del|release}_events - initialize and destroy disk_events.
//
#[no_mangle]
pub unsafe extern "C" fn disk_alloc_events(disk: *mut gendisk) -> c_int {
pub static mut ev: *mut c_void = core::ptr::null_mut();
    if (!disk.fops.check_events || !disk.events) {
    return 0;
    }
    ev = kzalloc_obj(*ev);
    if (!ev) {
    pr_warn!("%s: failed to initialize events\n", disk.disk_name);
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&ev.node);
    ev.disk = disk;
    spin_lock_init(&ev.lock);
    mutex_init(&ev.block_mutex);
    ev.block = 1;
    ev.poll_msecs = -1;
    INIT_DELAYED_WORK(&ev.dwork, disk_events_workfn);
    disk.ev = ev;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_add_events(disk: *mut gendisk) {
    if (!disk.ev) {
    return;
    }
    mutex_lock(&disk_events_mutex);
    list_add_tail(&disk.ev.node, &disk_events);
    mutex_unlock(&disk_events_mutex);
//
// Block count is initialized to 1 and the following initial
// unblock kicks it into action.
//
    __disk_unblock_events(disk, true);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_del_events(disk: *mut gendisk) {
    if (disk.ev) {
    disk_block_events(disk);
    mutex_lock(&disk_events_mutex);
    list_del_init(&disk.ev.node);
    mutex_unlock(&disk_events_mutex);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn disk_release_events(disk: *mut gendisk) {
// the block count should be 1 from disk_del_events()
    WARN_ON_ONCE!(disk.ev && disk.ev.block != 1);
    kfree(disk.ev);
    }