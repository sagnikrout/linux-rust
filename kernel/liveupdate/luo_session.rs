//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/luo_session.c
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// DOC: LUO Sessions
//
// LUO Sessions provide the core mechanism for grouping and managing `struct
// file *` instances that need to be preserved across a kexec-based live
// update. Each session acts as a named container for a set of file objects,
// allowing a userspace agent to manage the lifecycle of resources critical to a
// workload.
//
// Core Concepts:
//
// - Named Containers: Sessions are identified by a unique, user-provided name,
// which is used for both creation in the current kernel and retrieval in the
// next kernel.
//
// - Userspace Interface: Session management is driven from userspace via
// ioctls on /dev/liveupdate.
//
// - Serialization: Session metadata is preserved using the KHO framework. When
// a live update is triggered via kexec, session metadata is serialized into
// a chain of linked-blocks and placed in a preserved memory region. The
// physical address of the first block header is stored in the centralized
// `struct luo_ser` structure.
//
// Session Lifecycle:
//
// 1.  Creation: A userspace agent calls `luo_session_create()` to create a
// new, empty session and receives a file descriptor for it.
//
// 2.  Serialization: When the `reboot(LINUX_REBOOT_CMD_KEXEC)` syscall is
// made, `luo_session_serialize()` is called. It iterates through all
// active sessions and writes their metadata into a memory area preserved
// by KHO.
//
// 3.  Deserialization (in new kernel): After kexec, `luo_session_deserialize()`
// runs, reading the serialized data and creating a list of `struct
// luo_session` objects representing the preserved sessions.
//
// 4.  Retrieval: A userspace agent in the new kernel can then call
// `luo_session_retrieve()` with a session name to get a new file
// descriptor and access the preserved state.
//
// Locking:
//
// The LUO session subsystem uses a three-tier locking hierarchy to ensure thread
// safety and prevent deadlocks during concurrent session mutations and kexec
// serialization:
//
// 1. `luo_session_serialize_rwsem` (global rwsem):
// Protects session mutations (creation, retrieval, release, and ioctls)
// against the serialization process during reboot.
//
// - Readers: Taken by any path modifying or accessing session state (e.g.,
// `luo_session_create()`, `luo_session_retrieve()`, `luo_session_release()`,
// and `luo_session_ioctl()`).
// - Writer: Taken by the serialization process (`luo_session_serialize()`)
// during reboot. On success, the write lock is held indefinitely to freeze
// the subsystem. On failure, it is released to allow recovery.
//
// 2. `luo_session_header->rwsem` (per-list rwsem):
// Synchronizes list-level operations for the incoming and outgoing session headers.
//
// - Writer: Taken during list mutation operations (inserting or removing a
// session from the list).
// - Reader: Taken when traversing the list (e.g., retrieving a session by name).
//
// 3. `luo_session->mutex` (per-session mutex):
// Protects the internal state and file sets of an individual session. It is
// acquired during per-session operations such as preserving, retrieving,
// or freezing files.
//
// Lock Hierarchy:
// `luo_session_serialize_rwsem` -> `luo_session_header->rwsem` -> `luo_session->mutex`
//

pub static mut luo_session_serialize_rwsem: usize = 0;
//
// struct luo_session_header - Header struct for managing LUO sessions.
// @count:       The number of sessions currently tracked in the @list.
// @list:        The head of the linked list of `struct luo_session` instances.
// @rwsem:       A read-write semaphore providing synchronized access to the
// session list and other fields in this structure.
// @block_set:   The set of serialization blocks.
// @sessions_pa: Points to the location of sessions_pa within struct luo_ser.
// @active:      Set to true when first initialized. If previous kernel did not
// send session data, active stays false for incoming.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_session_header {
    pub count: c_long,
    pub list: list_head,
    pub rwsem: rw_semaphore,
    pub block_set: kho_block_set,
    pub sessions_pa: *mut u64,
    pub active: bool,
}

//
// struct luo_session_global - Global container for managing LUO sessions.
// @incoming:     The sessions passed from the previous kernel.
// @outgoing:     The sessions that are going to be passed to the next kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_session_global {
    pub incoming: luo_session_header,
    pub outgoing: luo_session_header,
}

pub static mut luo_session_global: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn luo_session_alloc(name: *mut c_char) -> *mut c_void {
    let mut session = kzalloc_obj(*session);
    if (!session) {
    return ERR_PTR(-ENOMEM);
    }
    strscpy(session.name, name, sizeof!(session.name));
    luo_file_set_init(&session.file_set);
    INIT_LIST_HEAD(&session.list);
    mutex_init(&session.mutex);
    return session;
    }
#[no_mangle]
unsafe extern "C" fn luo_session_free(session: *mut luo_session) {
    luo_file_set_destroy(&session.file_set);
    mutex_destroy(&session.mutex);
    kfree(session);
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_insert(sh: *mut luo_session_header, session: *mut luo_session) -> c_int {
pub static mut it: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    guard(rwsem_write)(&sh.rwsem);
//
// For outgoing we should make sure there is room in serialization array
// for new session.
//
    if (sh == &luo_session_global.outgoing) {
    err = kho_block_set_grow(&sh.block_set, sh.count + 1);
    if (err) {
    return err;
    }
    }
//
// For small number of sessions this loop won't hurt performance
// but if we ever start using a lot of sessions, this might
// become a bottle neck during deserialization time, as it would
// cause O(n*n) complexity.
//
    list_for_each_entry(it, &sh.list, list) {
    if (!strncmp(it.name, session.name, sizeof!(it.name))) {
    return -EEXIST;
    }
    }
    list_add_tail(&session.list, &sh.list);
    sh.count += 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_remove(sh: *mut luo_session_header, session: *mut luo_session) {
    guard(rwsem_write)(&sh.rwsem);
    list_del(&session.list);
    sh.count -= 1;
    if (sh == &luo_session_global.outgoing) {
    kho_block_set_shrink(&sh.block_set, sh.count);
    }
    }
#[no_mangle]
unsafe extern "C" fn luo_session_finish_one(session: *mut luo_session) -> c_int {
    guard(mutex)(&session.mutex);
    return luo_file_finish(&session.file_set);
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_unfreeze_one(session: *mut luo_session, ser: *mut luo_session_ser) {
    guard(mutex)(&session.mutex);
    luo_file_unfreeze(&session.file_set, &ser.file_set_ser);
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_freeze_one(session: *mut luo_session, ser: *mut luo_session_ser) -> c_int {
    guard(mutex)(&session.mutex);
    return luo_file_freeze(&session.file_set, &ser.file_set_ser);
    }
#[no_mangle]
unsafe extern "C" fn luo_session_release(inodep: *mut inode, filep: *mut file) -> c_int {
    let mut session = filep.private_data;
pub static mut sh: *mut c_void = core::ptr::null_mut();
    guard(rwsem_read)(&luo_session_serialize_rwsem);
// If retrieved is set, it means this session is from incoming list
    if (session.retrieved) {
pub static mut err: c_int = 0;
    if (err) {
    pr_warn!("Unable to finish session [%s] on release\n",
    session.name);
    return err;
    }
    sh = &luo_session_global.incoming;
    } else {
    scoped_guard(mutex, &session.mutex)
    luo_file_unpreserve_files(&session.file_set);
    sh = &luo_session_global.outgoing;
    }
    luo_session_remove(sh, session);
    luo_session_free(session);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_preserve_fd(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int {
    let mut argp = ucmd.cmd;
    let mut err = 0;
    guard(mutex)(&session.mutex);
    err = luo_preserve_file(&session.file_set, argp.token, argp.fd);
    if (err) {
    return err;
    }
    err = luo_ucmd_respond(ucmd, sizeof!(*argp));
    if (err) {
    pr_warn!("The file was successfully preserved, but response to user failed\n");
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_retrieve_fd(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int {
    let mut argp = ucmd.cmd;
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    argp.fd = get_unused_fd_flags(O_CLOEXEC);
    if (argp.fd < 0) {
    return argp.fd;
    }
    mutex_lock(&session.mutex);
    err = luo_retrieve_file(&session.file_set, argp.token, &file);
    mutex_unlock(&session.mutex);
    if (err < 0) {
// goto;
    }
    err = luo_ucmd_respond(ucmd, sizeof!(*argp));
    if (err) {
// goto;
    }
    fd_install(argp.fd, file);
    return 0;
// label;
    fput(file);
// label;
    put_unused_fd(argp.fd);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_finish(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int {
    let mut argp = ucmd.cmd;
    let mut err = 0;
    if (argp.reserved) {
    return -EINVAL;
    }
    err = luo_session_finish_one(session);
    if (err) {
    return err;
    }
    return luo_ucmd_respond(ucmd, sizeof!(*argp));
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_get_name(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int {
    let mut argp = ucmd.cmd;
    if (argp.reserved != 0) {
    return -EINVAL;
    }
    strscpy(argp.name, session.name, sizeof!(argp.name));
    return luo_ucmd_respond(ucmd, sizeof!(*argp));
    }
    union ucmd_buffer {
pub static mut finish: usize = 0;
pub static mut preserve: usize = 0;
pub static mut retrieve: usize = 0;
pub static mut get_name: usize = 0;
    };
// Type of sessions the ioctl applies to.
    enum luo_ioctl_type {
    LUO_IOCTL_INCOMING,
    LUO_IOCTL_OUTGOING,
    LUO_IOCTL_ALL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_ioctl_op {
    pub size: c_uint,
    pub min_size: c_uint,
    pub ioctl_num: c_uint,
    pub type: luo_ioctl_type,
    pub ucmd): *mut *mut *mut int (execute)(luo_session session, luo_ucmd,
}

    [_IOC_NR(_ioctl) - LIVEUPDATE_CMD_SESSION_BASE] = {                    
    .size = sizeof!(_struct) +                                      
    BUILD_BUG_ON_ZERO(sizeof!(union ucmd_buffer) <          
    sizeof!(_struct)),                    
    .min_size = offsetofend(_struct, _last),                       
    .ioctl_num = _ioctl,                                           
    .type = _type,                                                 
    .execute = _fn,                                                
    }
pub static mut luo_ioctl_op: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn luo_ioctl_type_valid(session: *mut luo_session, op: *mut luo_ioctl_op) -> bool {
    match (op.type) {
    LUO_IOCTL_INCOMING => {
// Retrieved is only set on incoming sessions
    return session.retrieved;
    }
    LUO_IOCTL_OUTGOING => {
    return !session.retrieved;
    }
    LUO_IOCTL_ALL => {
    return true;
    }
    }
// Catch-all.
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_ioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut session = filep.private_data;
pub static mut op: *mut c_void = core::ptr::null_mut();
pub static mut ucmd: luo_ucmd = 0;
    union ucmd_buffer buf;
    let mut nr = 0;
    let mut ret = 0;
    nr = _IOC_NR(cmd);
    if (nr < LIVEUPDATE_CMD_SESSION_BASE || (nr - LIVEUPDATE_CMD_SESSION_BASE) >=
    ARRAY_SIZE!(luo_session_ioctl_ops)) {
    return -EINVAL;
    }
    ucmd.ubuffer = arg;
    ret = get_user(ucmd.user_size, ucmd.ubuffer);
    if (ret) {
    return ret;
    }
    op = &luo_session_ioctl_ops[nr - LIVEUPDATE_CMD_SESSION_BASE];
    if (op.ioctl_num != cmd) {
    return -ENOIOCTLCMD;
    }
    if (!luo_ioctl_type_valid(session, op)) {
    return -EINVAL;
    }
    if (ucmd.user_size < op.min_size) {
    return -EINVAL;
    }
    ucmd.cmd = &buf;
    ret = copy_struct_from_user(ucmd.cmd, op.size, ucmd.ubuffer,
    ucmd.user_size);
    if (ret) {
    return ret;
    }
    guard(rwsem_read)(&luo_session_serialize_rwsem);
    return op.execute(session, &ucmd);
    }
pub static mut file_operations: usize = 0;
// Create a "struct file" for session
#[no_mangle]
unsafe extern "C" fn luo_session_getfile(session: *mut luo_session, filep: *mut file) -> c_int {
    char name_buf[128];
pub static mut file: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&session.mutex);
    snprintf(name_buf, sizeof!(name_buf), "[luo_session] %s", session.name);
    file = anon_inode_getfile(name_buf, &luo_session_fops, session, O_RDWR);
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
// filep = file;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_create(name: *const c_char, filep: *mut file) -> c_int {
pub static mut len: usize = 0;
pub static mut session: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (len == 0 || len > LIVEUPDATE_SESSION_NAME_LENGTH - 1) {
    return -EINVAL;
    }
    session = luo_session_alloc(name);
    if (IS_ERR(session)) {
    return PTR_ERR(session);
    }
    down_read(&luo_session_serialize_rwsem);
    err = luo_session_insert(&luo_session_global.outgoing, session);
    if (err) {
// goto;
    }
    mutex_lock(&session.mutex);
    err = luo_session_getfile(session, filep);
    mutex_unlock(&session.mutex);
    if (err) {
// goto;
    }
    up_read(&luo_session_serialize_rwsem);
    return 0;
// label;
    luo_session_remove(&luo_session_global.outgoing, session);
// label;
    luo_session_free(session);
    up_read(&luo_session_serialize_rwsem);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_retrieve(name: *const c_char, filep: *mut file) -> c_int {
    let mut sh = &luo_session_global.incoming;
    let mut session = core::ptr::null_mut();
pub static mut it: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    guard(rwsem_read)(&luo_session_serialize_rwsem);
    guard(rwsem_read)(&sh.rwsem);
    list_for_each_entry(it, &sh.list, list) {
    if (!strncmp(it.name, name, sizeof!(it.name))) {
    session = it;
    break;
    }
    }
    if (!session) {
    return -ENOENT;
    }
    guard(mutex)(&session.mutex);
    if (session.retrieved) {
    return -EINVAL;
    }
    err = luo_session_getfile(session, filep);
    if (!err) {
    session.retrieved = true;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_setup_outgoing(sessions_pa: *mut u64)  {
    luo_session_global.outgoing.sessions_pa = sessions_pa;
    luo_session_global.outgoing.active = true;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_setup_incoming(sessions_pa: u64) -> c_int {
    let mut sh = &luo_session_global.incoming;
    let mut err = 0;
    if (!sessions_pa) {
    return 0;
    }
    err = kho_block_set_restore(&sh.block_set, sessions_pa);
    if (err) {
    return err;
    }
    sh.active = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_deserialize_one(sh: *mut luo_session_header, ser: *mut luo_session_ser) -> c_int {
pub static mut session: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    session = luo_session_alloc(ser.name);
    if (IS_ERR(session)) {
    pr_warn!("Failed to allocate session [%.*s] during deserialization %pe\n",
    (int)sizeof!(ser.name), ser.name, session);
    return PTR_ERR(session);
    }
    err = luo_session_insert(sh, session);
    if (err) {
    pr_warn!("Failed to insert session [%s] %pe\n",
    session.name, ERR_PTR(err));
    luo_session_free(session);
    return err;
    }
    scoped_guard(mutex, &session.mutex) {
    err = luo_file_deserialize(&session.file_set,
    &ser.file_set_ser);
    }
    if (err) {
    pr_warn!("Failed to deserialize files for session [%s] %pe\n",
    session.name, ERR_PTR(err));
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_deserialize() -> c_int {
    let mut sh = &luo_session_global.incoming;
    static bool is_deserialized;
pub static mut ser: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
    static int saved_err;
    let mut err = 0;
// If has been deserialized, always return the same error code
    if (is_deserialized) {
    return saved_err;
    }
    is_deserialized = true;
    if (!sh.active) {
    return 0;
    }
//
// Note on error handling:
//
// If deserialization fails (e.g., allocation failure or corrupt data),
// we intentionally skip cleanup of sessions that were already restored.
//
// A partial failure leaves the preserved state inconsistent.
// Implementing a safe "undo" to unwind complex dependencies (sessions,
// files, hardware state) is error-prone and provides little value, as
// the system is effectively in a broken state.
//
// We treat these resources as leaked. The expected recovery path is for
// userspace to detect the failure and trigger a reboot, which will
// reliably reset devices and reclaim memory.
//
    kho_block_set_it_init(&it, &sh.block_set);
    while ((ser = kho_block_set_it_read_entry(&it))) {
    err = luo_session_deserialize_one(sh, ser);
    if (err) {
// goto;
    }
    }
    kho_block_set_destroy(&sh.block_set);
    return 0;
// label;
    kho_block_set_destroy(&sh.block_set);
    saved_err = err;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_session_serialize() -> c_int {
    let mut sh = &luo_session_global.outgoing;
pub static mut session: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
    let mut err = 0;
    down_write(&luo_session_serialize_rwsem);
    down_write(&sh.rwsem);
// sh->sessions_pa = 0;
    kho_block_set_it_init(&it, &sh.block_set);
    list_for_each_entry(session, &sh.list, list) {
    let mut ser = kho_block_set_it_reserve_entry(&it);
// This should not fail normally as blocks were pre-allocated
    if (WARN_ON_ONCE!(!ser)) {
    err = -ENOSPC;
// goto;
    }
    err = luo_session_freeze_one(session, ser);
    if (err) {
    kho_block_set_it_prev(&it);
// goto;
    }
    strscpy(ser.name, session.name, sizeof!(ser.name));
    }
    if (sh.count > 0) {
// sh->sessions_pa = kho_block_set_head_pa(&sh->block_set);
    }
    up_write(&sh.rwsem);
    return 0;
// label;
    list_for_each_entry_continue_reverse(session, &sh.list, list) {
    let mut ser = kho_block_set_it_prev(&it);
    luo_session_unfreeze_one(session, ser);
    memset(ser.name, 0, sizeof!(ser.name));
    }
    up_write(&sh.rwsem);
    up_write(&luo_session_serialize_rwsem);
    return err;
    }