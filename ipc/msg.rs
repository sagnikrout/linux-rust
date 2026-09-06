//! Automatically rewritten from C to Rust
//! Source: ipc/msg.c
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===



// SPDX-License-Identifier: GPL-2.0
//
// linux/ipc/msg.c
// Copyright (C) 1992 Krishna Balasubramanian
//
// Removed all the remaining kerneld mess
// Catch the -EFAULT stuff properly
// Use GFP_KERNEL for messages as in 1.2
// Fixed up the unchecked user space derefs
// Copyright (C) 1998 Alan Cox & Andi Kleen
//
// /proc/sysvipc/msg support (c) 1999 Dragos Acostachioaie <dragos@iname.com>
//
// mostly rewritten, threaded and wake-one semantics added
// MSGMAX limit removed, sysctl's added
// (c) 1999 Manfred Spraul <manfred@colorfullife.com>
//
// support for audit of ipc object properties and permission changes
// Dustin Kirkland <dustin.kirkland@us.ibm.com>
//
// namespaces support
// OpenVZ, SWsoft Inc.
// Pavel Emelianov <xemul@openvz.org>
//

// one msq_queue structure for each present queue on the system
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue {
    pub q_perm: kern_ipc_perm,
//     pub /: *mut *mut time64_t q_stime; / last msgsnd time,
//     pub /: *mut *mut time64_t q_rtime; / last msgrcv time,
//     pub /: *mut *mut time64_t q_ctime; / last change time,
//     pub /: *mut *mut unsigned long q_cbytes; / current number of bytes on queue,
//     pub /: *mut *mut unsigned long q_qnum; / number of messages in queue,
//     pub /: *mut *mut unsigned long q_qbytes; / max number of bytes on queue,
//     pub /: *mut *mut *mut pid q_lspid; / pid of last msgsnd,
//     pub /: *mut *mut *mut pid q_lrpid; / last receive pid,
    pub q_messages: list_head,
    pub q_receivers: list_head,
    pub q_senders: list_head,
}
//
// MSG_BARRIER Locking:
//
// Similar to the optimization used in ipc/mqueue.c, one syscall return path
// does not acquire any locks when it sees that a message exists in
// msg_receiver.r_msg. Therefore r_msg is set using smp_store_release()
// and accessed using READ_ONCE()+smp_acquire__after_ctrl_dep(). In addition,
// wake_q_add_safe() is used. See ipc/mqueue.c for more details
//
// one msg_receiver structure for each sleeping receiver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver {
    pub r_list: list_head,
    pub r_tsk: *mut task_struct,
    pub r_mode: c_int,
    pub r_msgtype: c_long,
    pub r_maxsize: c_long,
    pub r_msg: *mut msg_msg,
}

// one msg_sender for each sleeping sender
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender {
    pub list: list_head,
    pub tsk: *mut task_struct,
    pub msgsz: usize,
}

pub const SEARCH_ANY: c_int = 1;
pub const SEARCH_EQUAL: c_int = 2;
pub const SEARCH_NOTEQUAL: c_int = 3;
pub const SEARCH_LESSEQUAL: c_int = 4;
pub const SEARCH_NUMBER: c_int = 5;

#[no_mangle]
pub unsafe extern "C" fn msq_obtain_object(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
    let mut ipcp = ipc_obtain_object_idr(&msg_ids(ns), id);
    if (IS_ERR(ipcp)) {
    return ERR_CAST(ipcp);
    }
    return container_of!(ipcp, msg_queue, q_perm);
    }
#[no_mangle]
pub unsafe extern "C" fn msq_obtain_object_check(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
    let mut ipcp = ipc_obtain_object_check(&msg_ids(ns), id);
    if (IS_ERR(ipcp)) {
    return ERR_CAST(ipcp);
    }
    return container_of!(ipcp, msg_queue, q_perm);
    }
#[no_mangle]
pub unsafe extern "C" fn msg_rmid(ns: *mut ipc_namespace, s: *mut msg_queue) {
    ipc_rmid(&msg_ids(ns), &s.q_perm);
    }
#[no_mangle]
unsafe extern "C" fn msg_rcu_free(head: *mut rcu_head) {
    let mut p = container_of!(head, kern_ipc_perm, rcu);
    let mut msq = container_of!(p, msg_queue, q_perm);
    security_msg_queue_free(&msq.q_perm);
    kfree(msq);
    }
//
// newque - Create a new msg queue
// @ns: namespace
// @params: ptr to the structure that contains the key and msgflg
//
// Called with msg_ids.rwsem held (writer)
//
#[no_mangle]
unsafe extern "C" fn newque(ns: *mut ipc_namespace, params: *mut ipc_params) -> c_int {
pub static mut msq: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
pub static mut key: key_t = 0;
pub static mut msgflg: c_int = 0;
    msq = kmalloc_obj(*msq, GFP_KERNEL_ACCOUNT);
    if (unlikely(!msq)) {
    return -ENOMEM;
    }
    msq.q_perm.mode = msgflg & S_IRWXUGO;
    msq.q_perm.key = key;
    msq.q_perm.security = core::ptr::null_mut();
    retval = security_msg_queue_alloc(&msq.q_perm);
    if (retval) {
    kfree(msq);
    return retval;
    }
    msq.q_stime = msq.q_rtime = 0;
    msq.q_ctime = ktime_get_real_seconds();
    msq.q_cbytes = msq.q_qnum = 0;
    msq.q_qbytes = ns.msg_ctlmnb;
    msq.q_lspid = msq.q_lrpid = core::ptr::null_mut();
    INIT_LIST_HEAD(&msq.q_messages);
    INIT_LIST_HEAD(&msq.q_receivers);
    INIT_LIST_HEAD(&msq.q_senders);
// ipc_addid() locks msq upon success.
    retval = ipc_addid(&msg_ids(ns), &msq.q_perm, ns.msg_ctlmni);
    if (retval < 0) {
    ipc_rcu_putref(&msq.q_perm, msg_rcu_free);
    return retval;
    }
    ipc_unlock_object(&msq.q_perm);
    rcu_read_unlock();
    return msq.q_perm.id;
    }
#[no_mangle]
pub unsafe extern "C" fn msg_fits_inqueue(msq: *mut msg_queue, msgsz: usize) -> bool {
    return msgsz + msq.q_cbytes <= msq.q_qbytes &&
    1 + msq.q_qnum <= msq.q_qbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn ss_add(msq: *mut msg_queue, mss: *mut msg_sender, msgsz: size_t) {
    mss.tsk = current;
    mss.msgsz = msgsz;
//
// No memory barrier required: we did ipc_lock_object(),
// and the waker obtains that lock before calling wake_q_add().
//
    __set_current_state(TASK_INTERRUPTIBLE);
    list_add_tail(&mss.list, &msq.q_senders);
    }
#[no_mangle]
pub unsafe extern "C" fn ss_del(mss: *mut msg_sender) {
    if (mss.list.next) {
    list_del(&mss.list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ss_wakeup(msq: *mut msg_queue, wake_q: *mut wake_q_head, kill: bool) {
    let mut mss = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut stop_tsk = core::ptr::null_mut();
    let mut h = core::ptr::null_mut();
    if false {
    if (kill) {
    mss.list.next = core::ptr::null_mut();
    }
//
// Stop at the first task we don't wakeup,
// we've already iterated the original
// sender queue.
//

    else if (stop_tsk == mss.tsk) {
    break;
    }
//
// We are not in an EIDRM scenario here, therefore
// verify that we really need to wakeup the task.
// To maintain current semantics and wakeup order,
// move the sender to the tail on behalf of the
// blocked task.
//
if true {
    if (!stop_tsk) {
    stop_tsk = mss.tsk;
    }
    list_move_tail(&mss.list, &msq.q_senders);
    continue;
    }
    wake_q_add(wake_q, mss.tsk);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn expunge_all(msq: *mut msg_queue, res: c_int, wake_q: *mut wake_q_head) {
    let mut msr = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    if false {
pub static mut r_tsk: *mut c_void = core::ptr::null_mut();
    r_tsk = get_task_struct(msr.r_tsk);
// see MSG_BARRIER for purpose/pairing
    smp_store_release(&msr.r_msg, ERR_PTR(res));
    wake_q_add_safe(wake_q, r_tsk);
    }
    }
//
// freeque() wakes up waiters on the sender and receiver waiting queue,
// removes the message queue from message queue ID IDR, and cleans up all the
// messages associated with this queue.
//
// msg_ids.rwsem (writer) and the spinlock for this message queue are held
// before freeque() is called. msg_ids.rwsem remains locked on exit.
//
#[no_mangle]
unsafe extern "C" fn freeque(ns: *mut ipc_namespace, ipcp: *mut kern_ipc_perm) {
    let mut msg = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut msq = container_of!(ipcp, msg_queue, q_perm);
pub static mut wake_q: usize = 0;
    expunge_all(msq, -EIDRM, &wake_q);
    ss_wakeup(msq, &wake_q, true);
    msg_rmid(ns, msq);
    ipc_unlock_object(&msq.q_perm);
    wake_up_q(&wake_q);
    rcu_read_unlock();
    if false {
    percpu_counter_sub_local(&ns.percpu_msg_hdrs, 1);
    free_msg(msg);
    }
    percpu_counter_sub_local(&ns.percpu_msg_bytes, msq.q_cbytes);
    ipc_update_pid(&msq.q_lspid, core::ptr::null_mut());
    ipc_update_pid(&msq.q_lrpid, core::ptr::null_mut());
    ipc_rcu_putref(&msq.q_perm, msg_rcu_free);
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_msgget(key: key_t, msgflg: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut ipc_ops: usize = 0;
pub static mut msg_params: usize = 0;
    ns = current.nsproxy.ipc_ns;
    msg_params.key = key;
    msg_params.flg = msgflg;
    return ipcget(ns, &msg_ids(ns), &msg_ops, &msg_params);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_msgget(key: usize, msgflg: usize) -> c_long {
    return ksys_msgget(key, msgflg);
    }
#[no_mangle]
pub unsafe extern "C" fn copy_msqid_to_user(buf: *mut c_void, r#in: *mut msqid64_ds, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    return copy_to_user(buf, r#in, sizeof!(*r#in));
    }
    IPC_OLD => {
    {
pub static mut out: usize = 0;
    memset(&out, 0, sizeof!(out));
    ipc64_perm_to_ipc_perm(&r#in.msg_perm, &out.msg_perm);
    out.msg_stime		= r#in.msg_stime;
    out.msg_rtime		= r#in.msg_rtime;
    out.msg_ctime		= r#in.msg_ctime;
    if (r#in.msg_cbytes > USHRT_MAX) {
    out.msg_cbytes	= USHRT_MAX;
    }
    else {
    out.msg_cbytes	= r#in.msg_cbytes;
    }
    out.msg_lcbytes		= r#in.msg_cbytes;
    if (r#in.msg_qnum > USHRT_MAX) {
    out.msg_qnum	= USHRT_MAX;
    }
    else {
    out.msg_qnum	= r#in.msg_qnum;
    }
    if (r#in.msg_qbytes > USHRT_MAX) {
    out.msg_qbytes	= USHRT_MAX;
    }
    else {
    out.msg_qbytes	= r#in.msg_qbytes;
    }
    out.msg_lqbytes		= r#in.msg_qbytes;
    out.msg_lspid		= r#in.msg_lspid;
    out.msg_lrpid		= r#in.msg_lrpid;
    return copy_to_user(buf, &out, sizeof!(out));
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn copy_msqid_from_user(out: *mut msqid64_ds, buf: *mut c_void, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    if (copy_from_user(out, buf, sizeof!(*out))) {
    return -EFAULT;
    }
    return 0;
    }
    IPC_OLD => {
    {
pub static mut tbuf_old: usize = 0;
    if (copy_from_user(&tbuf_old, buf, sizeof!(tbuf_old))) {
    return -EFAULT;
    }
    out.msg_perm.uid	= tbuf_old.msg_perm.uid;
    out.msg_perm.gid	= tbuf_old.msg_perm.gid;
    out.msg_perm.mode	= tbuf_old.msg_perm.mode;
    if (tbuf_old.msg_qbytes == 0) {
    out.msg_qbytes	= tbuf_old.msg_lqbytes;
    }
    else {
    out.msg_qbytes	= tbuf_old.msg_qbytes;
    }
    return 0;
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
//
// This function handles some msgctl commands which require the rwsem
// to be held in write mode.
// NOTE: no locks must be held, the rwsem is taken inside this function.
//
#[no_mangle]
pub unsafe extern "C" fn msgctl_down(ns: *mut ipc_namespace, msqid: c_int, cmd: c_int, perm: *mut ipc64_perm, msg_qbytes: c_int) -> c_int {
pub static mut ipcp: *mut c_void = core::ptr::null_mut();
pub static mut msq: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    down_write(&msg_ids(ns).rwsem);
    rcu_read_lock();
    ipcp = ipcctl_obtain_check(ns, &msg_ids(ns), msqid, cmd,
    perm, msg_qbytes);
    if (IS_ERR(ipcp)) {
    err = PTR_ERR(ipcp);
// goto;
    }
    msq = container_of!(ipcp, msg_queue, q_perm);
    err = security_msg_queue_msgctl(&msq.q_perm, cmd);
    if (err) {
// goto;
    }
    match (cmd) {
    IPC_RMID => {
    ipc_lock_object(&msq.q_perm);
// freeque unlocks the ipc object and rcu
    freeque(ns, ipcp);
// goto;
    }
    IPC_SET => {
    {
pub static mut wake_q: usize = 0;
    if (msg_qbytes > ns.msg_ctlmnb &&
    !capable(CAP_SYS_RESOURCE)) {
    err = -EPERM;
// goto;
    }
    ipc_lock_object(&msq.q_perm);
    err = ipc_update_perm(perm, ipcp);
    if (err) {
// goto;
    }
    msq.q_qbytes = msg_qbytes;
    msq.q_ctime = ktime_get_real_seconds();
//
// Sleeping receivers might be excluded by
// stricter permissions.
//
    expunge_all(msq, -EAGAIN, &wake_q);
//
// Sleeping senders might be able to send
// due to a larger queue size.
//
    ss_wakeup(msq, &wake_q, false);
    ipc_unlock_object(&msq.q_perm);
    wake_up_q(&wake_q);
// goto;
    }
    }
    _ => {
    err = -EINVAL;
// goto;
    }
    }
    // label: out_unlock0
    ipc_unlock_object(&msq.q_perm);
    // label: out_unlock1
    rcu_read_unlock();
    // label: out_up
    up_write(&msg_ids(ns).rwsem);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn msgctl_info(ns: *mut ipc_namespace, msqid: c_int, cmd: c_int, msginfo: *mut msginfo) -> c_int {
    let mut err = 0;
    let mut max_idx = 0;
//
// We must not return kernel stack data.
// due to padding, it's not enough
// to set all member fields.
//
    err = security_msg_queue_msgctl(core::ptr::null_mut(), cmd);
    if (err) {
    return err;
    }
    memset(msginfo, 0, sizeof!(*msginfo));
    msginfo.msgmni = ns.msg_ctlmni;
    msginfo.msgmax = ns.msg_ctlmax;
    msginfo.msgmnb = ns.msg_ctlmnb;
    msginfo.msgssz = MSGSSZ;
    msginfo.msgseg = MSGSEG;
    down_read(&msg_ids(ns).rwsem);
    if (cmd == MSG_INFO) {
    msginfo.msgpool = msg_ids(ns).in_use;
    }
    max_idx = ipc_get_maxidx(&msg_ids(ns));
    up_read(&msg_ids(ns).rwsem);
    if (cmd == MSG_INFO) {
    msginfo.msgmap = min_t!(int,
    percpu_counter_sum(&ns.percpu_msg_hdrs),
    INT_MAX);
    msginfo.msgtql = min_t!(int,
    percpu_counter_sum(&ns.percpu_msg_bytes),
    INT_MAX);
    } else {
    msginfo.msgmap = MSGMAP;
    msginfo.msgpool = MSGPOOL;
    msginfo.msgtql = MSGTQL;
    }
    return if (max_idx < 0) { 0 } else { max_idx };
    }
#[no_mangle]
pub unsafe extern "C" fn msgctl_stat(ns: *mut ipc_namespace, msqid: c_int, cmd: c_int, p: *mut msqid64_ds) -> c_int {
pub static mut msq: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    memset(p, 0, sizeof!(*p));
    rcu_read_lock();
    if (cmd == MSG_STAT || cmd == MSG_STAT_ANY) {
    msq = msq_obtain_object(ns, msqid);
    if (IS_ERR(msq)) {
    err = PTR_ERR(msq);
// goto;
    }
    } else { /* IPC_STAT */
    msq = msq_obtain_object_check(ns, msqid);
    if (IS_ERR(msq)) {
    err = PTR_ERR(msq);
// goto;
    }
    }
// see comment for SHM_STAT_ANY
    if (cmd == MSG_STAT_ANY) {
    audit_ipc_obj(&msq.q_perm);
    }
    else {
    err = -EACCES;
    if (ipcperms(ns, &msq.q_perm, S_IRUGO)) {
// goto;
    }
    }
    err = security_msg_queue_msgctl(&msq.q_perm, cmd);
    if (err) {
// goto;
    }
    ipc_lock_object(&msq.q_perm);
    if (!ipc_valid_object(&msq.q_perm)) {
    ipc_unlock_object(&msq.q_perm);
    err = -EIDRM;
// goto;
    }
    kernel_to_ipc64_perm(&msq.q_perm, &p.msg_perm);
    p.msg_stime  = msq.q_stime;
    p.msg_rtime  = msq.q_rtime;
    p.msg_ctime  = msq.q_ctime;

    p.msg_stime_high = msq.q_stime >> 32;
    p.msg_rtime_high = msq.q_rtime >> 32;
    p.msg_ctime_high = msq.q_ctime >> 32;

    p.msg_cbytes = msq.q_cbytes;
    p.msg_qnum   = msq.q_qnum;
    p.msg_qbytes = msq.q_qbytes;
    p.msg_lspid  = pid_vnr(msq.q_lspid);
    p.msg_lrpid  = pid_vnr(msq.q_lrpid);
    if (cmd == IPC_STAT) {
//
// As defined in SUS:
// Return 0 on success
//
    err = 0;
    } else {
//
// MSG_STAT and MSG_STAT_ANY (both Linux specific)
// Return the full id, including the sequence number
//
    err = msq.q_perm.id;
    }
    ipc_unlock_object(&msq.q_perm);
    // label: out_unlock
    rcu_read_unlock();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ksys_msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds , version: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut msqid64: usize = 0;
    let mut err = 0;
    if (msqid < 0 || cmd < 0) {
    return -EINVAL;
    }
    ns = current.nsproxy.ipc_ns;
    match (cmd) {
     IPC_INFO | MSG_INFO => {
     {
pub static mut msginfo: usize = 0;
    err = msgctl_info(ns, msqid, cmd, &msginfo);
    if (err < 0) {
    return err;
    }
    if (copy_to_user(buf, &msginfo, sizeof!(msginfo))) {
    err = -EFAULT;
    }
    return err;
    }
    }
    MSG_STAT | MSG_STAT_ANY | IPC_STAT => {
    err = msgctl_stat(ns, msqid, cmd, &msqid64);
    if (err < 0) {
    return err;
    }
    if (copy_msqid_to_user(buf, &msqid64, version)) {
    err = -EFAULT;
    }
    return err;
    }
    IPC_SET => {
    if (copy_msqid_from_user(&msqid64, buf, version)) {
    return -EFAULT;
    }
    return msgctl_down(ns, msqid, cmd, &msqid64.msg_perm,
    msqid64.msg_qbytes);
    }
    IPC_RMID => {
    return msgctl_down(ns, msqid, cmd, core::ptr::null_mut(), 0);
    }
    _ => {
    return  -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sys_msgctl(msqid: usize, cmd: usize, buf: usize) -> c_long {
    return ksys_msgctl(msqid, cmd, buf, IPC_64);
    }

#[no_mangle]
pub unsafe extern "C" fn ksys_old_msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds ) -> c_long {
pub static mut version: c_int = 0;
    return ksys_msgctl(msqid, cmd, buf, version);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_old_msgctl(msqid: usize, cmd: usize, buf: usize) -> c_long {
    return ksys_old_msgctl(msqid, cmd, buf);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_msqid_ds {
    pub msg_perm: compat_ipc_perm,
    pub msg_first: compat_uptr_t,
    pub msg_last: compat_uptr_t,
    pub msg_stime: old_time32_t,
    pub msg_rtime: old_time32_t,
    pub msg_ctime: old_time32_t,
    pub msg_lcbytes: compat_ulong_t,
    pub msg_lqbytes: compat_ulong_t,
    pub msg_cbytes: c_ushort,
    pub msg_qnum: c_ushort,
    pub msg_qbytes: c_ushort,
    pub msg_lspid: compat_ipc_pid_t,
    pub msg_lrpid: compat_ipc_pid_t,
}

#[no_mangle]
pub unsafe extern "C" fn copy_compat_msqid_from_user(out: *mut msqid64_ds, buf: *mut c_void, version: c_int) -> c_int {
    memset(out, 0, sizeof!(*out));
    if (version == IPC_64) {
    let mut p = core::ptr::null_mut();
    if (get_compat_ipc64_perm(&out.msg_perm, &p.msg_perm)) {
    return -EFAULT;
    }
    if (get_user(out.msg_qbytes, &p.msg_qbytes)) {
    return -EFAULT;
    }
    } else {
    let mut p = core::ptr::null_mut();
    if (get_compat_ipc_perm(&out.msg_perm, &p.msg_perm)) {
    return -EFAULT;
    }
    if (get_user(out.msg_qbytes, &p.msg_qbytes)) {
    return -EFAULT;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn copy_compat_msqid_to_user(buf: *mut c_void, r#in: *mut msqid64_ds, version: c_int) -> c_int {
    if (version == IPC_64) {
pub static mut v: usize = 0;
    memset(&v, 0, sizeof!(v));
    to_compat_ipc64_perm(&v.msg_perm, &r#in.msg_perm);
    v.msg_stime	 = lower_32_bits(r#in.msg_stime);
    v.msg_stime_high = upper_32_bits(r#in.msg_stime);
    v.msg_rtime	 = lower_32_bits(r#in.msg_rtime);
    v.msg_rtime_high = upper_32_bits(r#in.msg_rtime);
    v.msg_ctime	 = lower_32_bits(r#in.msg_ctime);
    v.msg_ctime_high = upper_32_bits(r#in.msg_ctime);
    v.msg_cbytes = r#in.msg_cbytes;
    v.msg_qnum = r#in.msg_qnum;
    v.msg_qbytes = r#in.msg_qbytes;
    v.msg_lspid = r#in.msg_lspid;
    v.msg_lrpid = r#in.msg_lrpid;
    return copy_to_user(buf, &v, sizeof!(v));
    } else {
pub static mut v: usize = 0;
    memset(&v, 0, sizeof!(v));
    to_compat_ipc_perm(&v.msg_perm, &r#in.msg_perm);
    v.msg_stime = r#in.msg_stime;
    v.msg_rtime = r#in.msg_rtime;
    v.msg_ctime = r#in.msg_ctime;
    v.msg_cbytes = r#in.msg_cbytes;
    v.msg_qnum = r#in.msg_qnum;
    v.msg_qbytes = r#in.msg_qbytes;
    v.msg_lspid = r#in.msg_lspid;
    v.msg_lrpid = r#in.msg_lrpid;
    return copy_to_user(buf, &v, sizeof!(v));
    }
    }
#[no_mangle]
unsafe extern "C" fn compat_ksys_msgctl(msqid: c_int, cmd: c_int, uptr: *mut c_void , version: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
pub static mut msqid64: usize = 0;
    ns = current.nsproxy.ipc_ns;
    if (msqid < 0 || cmd < 0) {
    return -EINVAL;
    }
    match (cmd & (!IPC_64)) {
     IPC_INFO | MSG_INFO => {
     {
pub static mut msginfo: usize = 0;
    err = msgctl_info(ns, msqid, cmd, &msginfo);
    if (err < 0) {
    return err;
    }
    if (copy_to_user(uptr, &msginfo, sizeof!(msginfo))) {
    err = -EFAULT;
    }
    return err;
    }
    }
    IPC_STAT | MSG_STAT | MSG_STAT_ANY => {
    err = msgctl_stat(ns, msqid, cmd, &msqid64);
    if (err < 0) {
    return err;
    }
    if (copy_compat_msqid_to_user(uptr, &msqid64, version)) {
    err = -EFAULT;
    }
    return err;
    }
    IPC_SET => {
    if (copy_compat_msqid_from_user(&msqid64, uptr, version)) {
    return -EFAULT;
    }
    return msgctl_down(ns, msqid, cmd, &msqid64.msg_perm, msqid64.msg_qbytes);
    }
    IPC_RMID => {
    return msgctl_down(ns, msqid, cmd, core::ptr::null_mut(), 0);
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_msgctl
pub unsafe extern "C" fn sys_msgctl_dup(msqid: usize, cmd: usize, uptr: usize) -> c_long {
    return compat_ksys_msgctl(msqid, cmd, uptr, IPC_64);
    }

#[no_mangle]
pub unsafe extern "C" fn compat_ksys_old_msgctl(msqid: c_int, cmd: c_int, uptr: *mut c_void ) -> c_long {
pub static mut version: c_int = 0;
    return compat_ksys_msgctl(msqid, cmd, uptr, version);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_old_msgctl
pub unsafe extern "C" fn sys_old_msgctl_dup(msqid: usize, cmd: usize, uptr: usize) -> c_long {
    return compat_ksys_old_msgctl(msqid, cmd, uptr);
    }

#[no_mangle]
unsafe extern "C" fn testmsg(msg: *mut msg_msg, r#type: c_long, mode: c_int) -> c_int {
    match (mode) {
    SEARCH_ANY | SEARCH_NUMBER => {
    return 1;
    }
    SEARCH_LESSEQUAL => {
    if (msg.m_type <= r#type) {
    return 1;
    }
    // break;
    }
    SEARCH_EQUAL => {
    if (msg.m_type == r#type) {
    return 1;
    }
    // break;
    }
    SEARCH_NOTEQUAL => {
    if (msg.m_type != r#type) {
    return 1;
    }
    // break;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pipelined_send(msq: *mut msg_queue, msg: *mut msg_msg, wake_q: *mut wake_q_head) -> c_int {
    let mut msr = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    if false {
    if (testmsg(msg, msr.r_msgtype, msr.r_mode) &&
    !security_msg_queue_msgrcv(&msq.q_perm, msg, msr.r_tsk,
    msr.r_msgtype, msr.r_mode)) {
    list_del(&msr.r_list);
    if (msr.r_maxsize < msg.m_ts) {
    wake_q_add(wake_q, msr.r_tsk);
// See expunge_all regarding memory barrier
    smp_store_release(&msr.r_msg, ERR_PTR(-E2BIG));
    } else {
    ipc_update_pid(&msq.q_lrpid, task_pid(msr.r_tsk));
    msq.q_rtime = ktime_get_real_seconds();
    wake_q_add(wake_q, msr.r_tsk);
// See expunge_all regarding memory barrier
    smp_store_release(&msr.r_msg, msg);
    return 1;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn do_msgsnd(msqid: c_int, mtype: c_long, mtext: *mut c_void, msgsz: size_t, msgflg: c_int) -> c_long {
pub static mut msq: *mut c_void = core::ptr::null_mut();
pub static mut msg: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut wake_q: usize = 0;
    ns = current.nsproxy.ipc_ns;
    if (msgsz > ns.msg_ctlmax ||  msgsz < 0 || msqid < 0) {
    return -EINVAL;
    }
    if (mtype < 1) {
    return -EINVAL;
    }
    msg = load_msg(mtext, msgsz);
    if (IS_ERR(msg)) {
    return PTR_ERR(msg);
    }
    msg.m_type = mtype;
    msg.m_ts = msgsz;
    rcu_read_lock();
    msq = msq_obtain_object_check(ns, msqid);
    if (IS_ERR(msq)) {
    err = PTR_ERR(msq);
// goto;
    }
    ipc_lock_object(&msq.q_perm);
    loop {
pub static mut s: usize = 0;
    err = -EACCES;
    if (ipcperms(ns, &msq.q_perm, S_IWUGO)) {
// goto;
    }
// raced with RMID?
    if (!ipc_valid_object(&msq.q_perm)) {
    err = -EIDRM;
// goto;
    }
    err = security_msg_queue_msgsnd(&msq.q_perm, msg, msgflg);
    if (err) {
// goto;
    }
    if (msg_fits_inqueue(msq, msgsz)) {
    break;
    }
// queue full, wait:
    if (msgflg & IPC_NOWAIT) {
    err = -EAGAIN;
// goto;
    }
// enqueue the sender and prepare to block
    ss_add(msq, &s, msgsz);
    if (!ipc_rcu_getref(&msq.q_perm)) {
    err = -EIDRM;
// goto;
    }
    ipc_unlock_object(&msq.q_perm);
    rcu_read_unlock();
    schedule();
    rcu_read_lock();
    ipc_lock_object(&msq.q_perm);
    ipc_rcu_putref(&msq.q_perm, msg_rcu_free);
// raced with RMID?
    if (!ipc_valid_object(&msq.q_perm)) {
    err = -EIDRM;
// goto;
    }
    ss_del(&s);
    if (signal_pending(current)) {
    err = -ERESTARTNOHAND;
// goto;
    }
    }
    ipc_update_pid(&msq.q_lspid, task_tgid(current));
    msq.q_stime = ktime_get_real_seconds();
    if (!pipelined_send(msq, msg, &wake_q)) {
// no one is waiting for this message, enqueue it
    list_add_tail(&msg.m_list, &msq.q_messages);
    msq.q_cbytes += msgsz;
    msq.q_qnum += 1;
    percpu_counter_add_local(&ns.percpu_msg_bytes, msgsz);
    percpu_counter_add_local(&ns.percpu_msg_hdrs, 1);
    }
    err = 0;
    msg = core::ptr::null_mut();
    // label: out_unlock0
    ipc_unlock_object(&msq.q_perm);
    wake_up_q(&wake_q);
    // label: out_unlock1
    rcu_read_unlock();
    if (msg != core::ptr::null_mut()) {
    free_msg(msg);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_msgsnd(msqid: c_int, msgp: *mut msgbuf, msgsz: size_t, msgflg: c_int) -> c_long {
    let mut mtype = 0;
    if (get_user(mtype, &msgp.mtype)) {
    return -EFAULT;
    }
    return do_msgsnd(msqid, mtype, msgp.mtext, msgsz, msgflg);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_msgsnd(msqid: usize, msgp: usize, msgsz: usize, msgflg: usize) -> c_long {
    return ksys_msgsnd(msqid, msgp, msgsz, msgflg);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_msgbuf {
    pub mtype: compat_long_t,
    pub mtext: [c_char; 0],
}

#[no_mangle]
pub unsafe extern "C" fn compat_ksys_msgsnd(msqid: c_int, msgp: compat_uptr_t, msgsz: compat_ssize_t, msgflg: c_int) -> c_long {
    let mut up = core::ptr::null_mut();
    let mut mtype;
    if (get_user(mtype, &up.mtype)) {
    return -EFAULT;
    }
    return do_msgsnd(msqid, mtype, up.mtext, msgsz, msgflg);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_msgsnd
pub unsafe extern "C" fn sys_msgsnd_dup(msqid: usize, msgp: usize, msgsz: usize, msgflg: usize) -> c_long {
    return compat_ksys_msgsnd(msqid, msgp, msgsz, msgflg);
    }

#[no_mangle]
pub unsafe extern "C" fn convert_mode(msgtyp: *mut c_long, msgflg: c_int) -> c_int {
    if (msgflg & MSG_COPY) {
    return SEARCH_NUMBER;
    }
//
// find message of correct type.
// msgtyp = 0 => get first.
// msgtyp > 0 => get first message of matching type.
// msgtyp < 0 => get message with least type must be < abs(msgtype).
//
    if (*msgtyp == 0) {
    return SEARCH_ANY;
    }
    if (*msgtyp < 0) {
    if (*msgtyp == LONG_MIN) /* -LONG_MIN is undefined */ {
// msgtyp = LONG_MAX;
    }
    else {
// msgtyp = -*msgtyp;
    }
    return SEARCH_LESSEQUAL;
    }
    if (msgflg & MSG_EXCEPT) {
    return SEARCH_NOTEQUAL;
    }
    return SEARCH_EQUAL;
    }
#[no_mangle]
unsafe extern "C" fn do_msg_fill(dest: *mut c_void , msg: *mut msg_msg, bufsz: usize) -> c_long {
    let mut msgp = core::ptr::null_mut();
    let mut msgsz = 0;
    if (put_user(msg.m_type, &msgp.mtype)) {
    return -EFAULT;
    }
    msgsz = if (bufsz > msg.m_ts) { msg.m_ts } else { bufsz };
    if (store_msg(msgp.mtext, msg, msgsz)) {
    return -EFAULT;
    }
    return msgsz;
    }

//
// This function creates new kernel message structure, large enough to store
// bufsz message bytes.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_copy(buf: *mut c_void, bufsz: size_t) -> *mut c_void {
pub static mut copy: *mut c_void = core::ptr::null_mut();
//
// Create dummy message to copy real message to.
//
    copy = load_msg(buf, bufsz);
    if (!IS_ERR(copy)) {
    copy.m_ts = bufsz;
    }
    return copy;
    }
#[no_mangle]
pub unsafe extern "C" fn free_copy(copy: *mut msg_msg) {
    if (copy) {
    free_msg(copy);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: prepare_copy
pub unsafe extern "C" fn prepare_copy_dup(buf: *mut c_void, bufsz: size_t) -> *mut c_void {
    return ERR_PTR(-ENOSYS);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: free_copy
pub unsafe extern "C" fn free_copy_dup(copy: *mut msg_msg) {
    }

#[no_mangle]
pub unsafe extern "C" fn find_msg(msq: *mut msg_queue, msgtyp: *mut c_long, mode: c_int) -> *mut c_void {
    let mut msg = core::ptr::null_mut();
    let mut found = core::ptr::null_mut();
pub static mut count: c_long = 0;
    if false {
    if (testmsg(msg, *msgtyp, mode) &&
    !security_msg_queue_msgrcv(&msq.q_perm, msg, current,
msgtyp, mode)) {
    if (mode == SEARCH_LESSEQUAL && msg.m_type != 1) {
// msgtyp = msg->m_type - 1;
    found = msg;
    } else if (mode == SEARCH_NUMBER) {
    if (*msgtyp == count) {
    return msg;
    }
    } else {
    return msg;
    }
    count += 1;
    }
    }
return if found != core::ptr::null_mut() { found } else { ERR_PTR(-EAGAIN) };
    }
#[no_mangle]
pub unsafe extern "C" fn do_msgrcv(msqid: c_int, buf: *mut c_void, bufsz: size_t, msgtyp: c_long, msgflg: c_int, msg_handler: *mut c_void) -> c_long {
    let mut mode = 0;
pub static mut msq: *mut c_void = core::ptr::null_mut();
pub static mut ns: *mut c_void = core::ptr::null_mut();
    let mut msg = core::ptr::null_mut();
    let mut copy = core::ptr::null_mut();
pub static mut wake_q: usize = 0;
    ns = current.nsproxy.ipc_ns;
    if (msqid < 0 ||  bufsz < 0) {
    return -EINVAL;
    }
    if (msgflg & MSG_COPY) {
    if ((msgflg & MSG_EXCEPT) || !(msgflg & IPC_NOWAIT)) {
    return -EINVAL;
    }
    copy = prepare_copy(buf, min_t!(size_t, bufsz, ns.msg_ctlmax));
    if (IS_ERR(copy)) {
    return PTR_ERR(copy);
    }
    }
    mode = convert_mode(&msgtyp, msgflg);
    rcu_read_lock();
    msq = msq_obtain_object_check(ns, msqid);
    if (IS_ERR(msq)) {
    rcu_read_unlock();
    free_copy(copy);
    return PTR_ERR(msq);
    }
    loop {
pub static mut msr_d: usize = 0;
    msg = ERR_PTR(-EACCES);
    if (ipcperms(ns, &msq.q_perm, S_IRUGO)) {
// goto;
    }
    ipc_lock_object(&msq.q_perm);
// raced with RMID?
    if (!ipc_valid_object(&msq.q_perm)) {
    msg = ERR_PTR(-EIDRM);
// goto;
    }
    msg = find_msg(msq, &msgtyp, mode);
    if (!IS_ERR(msg)) {
//
// Found a suitable message.
// Unlink it from the queue.
//
    if ((bufsz < msg.m_ts) && !(msgflg & MSG_NOERROR)) {
    msg = ERR_PTR(-E2BIG);
// goto;
    }
//
// If we are copying, then do not unlink message and do
// not update queue parameters.
//
    if (msgflg & MSG_COPY) {
    msg = copy_msg(msg, copy);
// goto;
    }
    list_del(&msg.m_list);
    msq.q_qnum -= 1;
    msq.q_rtime = ktime_get_real_seconds();
    ipc_update_pid(&msq.q_lrpid, task_tgid(current));
    msq.q_cbytes -= msg.m_ts;
    percpu_counter_sub_local(&ns.percpu_msg_bytes, msg.m_ts);
    percpu_counter_sub_local(&ns.percpu_msg_hdrs, 1);
    ss_wakeup(msq, &wake_q, false);
// goto;
    }
// No message waiting. Wait for a message
    if (msgflg & IPC_NOWAIT) {
    msg = ERR_PTR(-ENOMSG);
// goto;
    }
    list_add_tail(&msr_d.r_list, &msq.q_receivers);
    msr_d.r_tsk = current;
    msr_d.r_msgtype = msgtyp;
    msr_d.r_mode = mode;
    if (msgflg & MSG_NOERROR) {
    msr_d.r_maxsize = INT_MAX;
    }
    else {
    msr_d.r_maxsize = bufsz;
    }
// memory barrier not require due to ipc_lock_object()
    WRITE_ONCE(msr_d.r_msg, ERR_PTR(-EAGAIN));
// memory barrier not required, we own ipc_lock_object()
    __set_current_state(TASK_INTERRUPTIBLE);
    ipc_unlock_object(&msq.q_perm);
    rcu_read_unlock();
    schedule();
//
// Lockless receive, part 1:
// We don't hold a reference to the queue and getting a
// reference would defeat the idea of a lockless operation,
// thus the code relies on rcu to guarantee the existence of
// msq:
// Prior to destruction, expunge_all(-EIRDM) changes r_msg.
// Thus if r_msg is -EAGAIN, then the queue not yet destroyed.
//
    rcu_read_lock();
//
// Lockless receive, part 2:
// The work in pipelined_send() and expunge_all():
// - Set pointer to message
// - Queue the receiver task for later wakeup
// - Wake up the process after the lock is dropped.
//
// Should the process wake up before this wakeup (due to a
// signal) it will either see the message and continue ...
//
    msg = READ_ONCE(msr_d.r_msg);
    if (msg != ERR_PTR(-EAGAIN)) {
// see MSG_BARRIER for purpose/pairing
    smp_acquire__after_ctrl_dep();
// goto;
    }
//
// ... or see -EAGAIN, acquire the lock to check the message
// again.
//
    ipc_lock_object(&msq.q_perm);
    msg = READ_ONCE(msr_d.r_msg);
    if (msg != ERR_PTR(-EAGAIN)) {
// goto;
    }
    list_del(&msr_d.r_list);
    if (signal_pending(current)) {
    msg = ERR_PTR(-ERESTARTNOHAND);
// goto;
    }
    ipc_unlock_object(&msq.q_perm);
    }
    // label: out_unlock0
    ipc_unlock_object(&msq.q_perm);
    wake_up_q(&wake_q);
    // label: out_unlock1
    rcu_read_unlock();
    if (IS_ERR(msg)) {
    free_copy(copy);
    return PTR_ERR(msg);
    }
    bufsz = msg_handler(buf, msg, bufsz);
    free_msg(msg);
    return bufsz;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_msgrcv(msqid: c_int, msgp: *mut msgbuf, msgsz: size_t, msgtyp: c_long, msgflg: c_int) -> c_long {
    return do_msgrcv(msqid, msgp, msgsz, msgtyp, msgflg, do_msg_fill);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_msgrcv(msqid: usize, msgp: usize, msgsz: usize, msgtyp: usize, msgflg: usize) -> c_long {
    return ksys_msgrcv(msqid, msgp, msgsz, msgtyp, msgflg);
    }

#[no_mangle]
unsafe extern "C" fn compat_do_msg_fill(dest: *mut c_void , msg: *mut msg_msg, bufsz: usize) -> c_long {
    let mut msgp = core::ptr::null_mut();
    let mut msgsz = 0;
    if (put_user(msg.m_type, &msgp.mtype)) {
    return -EFAULT;
    }
    msgsz = if (bufsz > msg.m_ts) { msg.m_ts } else { bufsz };
    if (store_msg(msgp.mtext, msg, msgsz)) {
    return -EFAULT;
    }
    return msgsz;
    }
#[no_mangle]
pub unsafe extern "C" fn compat_ksys_msgrcv(msqid: c_int, msgp: compat_uptr_t, msgsz: compat_ssize_t, msgtyp: compat_long_t, msgflg: c_int) -> c_long {
    return do_msgrcv(msqid, compat_ptr(msgp), msgsz, msgtyp,
    msgflg, compat_do_msg_fill);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_msgrcv
pub unsafe extern "C" fn sys_msgrcv_dup(msqid: usize, msgp: usize, msgsz: usize, msgtyp: usize, msgflg: usize) -> c_long {
    return compat_ksys_msgrcv(msqid, msgp, msgsz, msgtyp, msgflg);
    }

#[no_mangle]
pub unsafe extern "C" fn msg_init_ns(ns: *mut ipc_namespace) -> c_int {
    let mut ret = 0;
    ns.msg_ctlmax = MSGMAX;
    ns.msg_ctlmnb = MSGMNB;
    ns.msg_ctlmni = MSGMNI;
    ret = percpu_counter_init(&ns.percpu_msg_bytes, 0, GFP_KERNEL);
    if (ret) {
// goto;
    }
    ret = percpu_counter_init(&ns.percpu_msg_hdrs, 0, GFP_KERNEL);
    if (ret) {
// goto;
    }
    ipc_init_ids(&ns.ids[IPC_MSG_IDS]);
    return 0;
    // label: fail_msg_hdrs
    percpu_counter_destroy(&ns.percpu_msg_bytes);
    // label: fail_msg_bytes
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn msg_exit_ns(ns: *mut ipc_namespace) {
    free_ipcs(ns, &msg_ids(ns), freeque);
    idr_destroy(&ns.ids[IPC_MSG_IDS].ipcs_idr);
    rhashtable_destroy(&ns.ids[IPC_MSG_IDS].key_ht);
    percpu_counter_destroy(&ns.percpu_msg_bytes);
    percpu_counter_destroy(&ns.percpu_msg_hdrs);
    }

#[no_mangle]
unsafe extern "C" fn sysvipc_msg_proc_show(s: *mut seq_file, it: *mut c_void) -> c_int {
    let mut pid_ns = core::ptr::null_mut();
    let mut user_ns = core::ptr::null_mut();
    let mut ipcp = core::ptr::null_mut();
    let mut msq = container_of!(ipcp, msg_queue, q_perm);
    seq_printf(s,
    "%10d %10d  %4o  %10lu %10lu %5u %5u %5u %5u %5u %5u %10llu %10llu %10llu\n",
    msq.q_perm.key,
    msq.q_perm.id,
    msq.q_perm.mode,
    msq.q_cbytes,
    msq.q_qnum,
    pid_nr_ns(msq.q_lspid, pid_ns),
    pid_nr_ns(msq.q_lrpid, pid_ns),
    from_kuid_munged(user_ns, msq.q_perm.uid),
    from_kgid_munged(user_ns, msq.q_perm.gid),
    from_kuid_munged(user_ns, msq.q_perm.cuid),
    from_kgid_munged(user_ns, msq.q_perm.cgid),
    msq.q_stime,
    msq.q_rtime,
    msq.q_ctime);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn msg_init()  {
    msg_init_ns(&init_ipc_ns);
    ipc_init_proc_interface("sysvipc/msg",
    "       key      msqid perms      cbytes       qnum lspid lrpid   uid   gid  cuid  cgid      stime      rtime      ctime\n",
    IPC_MSG_IDS, sysvipc_msg_proc_show);
    }