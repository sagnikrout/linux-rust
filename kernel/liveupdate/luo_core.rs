//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/luo_core.c
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
// DOC: Live Update Orchestrator (LUO)
//
// Live Update is a specialized, kexec-based reboot process that allows a
// running kernel to be updated from one version to another while preserving
// the state of selected resources and keeping designated hardware devices
// operational. For these devices, DMA activity may continue throughout the
// kernel transition.
//
// While the primary use case driving this work is supporting live updates of
// the Linux kernel when it is used as a hypervisor in cloud environments, the
// LUO framework itself is designed to be workload-agnostic. Live Update
// facilitates a full kernel version upgrade for any type of system.
//
// For example, a non-hypervisor system running an in-memory cache like
// memcached with many gigabytes of data can use LUO. The userspace service
// can place its cache into a memfd, have its state preserved by LUO, and
// restore it immediately after the kernel kexec.
//
// Whether the system is running virtual machines, containers, a
// high-performance database, or networking services, LUO's primary goal is to
// enable a full kernel update by preserving critical userspace state and
// keeping essential devices operational.
//
// The core of LUO is a mechanism that tracks the progress of a live update,
// along with a callback API that allows other kernel subsystems to participate
// in the process. Example subsystems that can hook into LUO include: kvm,
// iommu, interrupts, vfio, participating filesystems, and memory management.
//
// LUO uses Kexec Handover to transfer memory state from the current kernel to
// the next kernel. For more details see Documentation/core-api/kho/index.rst.
//
// .. note::
// To enable LUO, boot the kernel with the ``liveupdate=on`` command line
// parameter.
//

    static struct {
    let mut enabled = 0;
pub static mut luo_ser_out: *mut c_void = core::ptr::null_mut();
    let mut liveupdate_num = 0;
    } luo_global;
//
// luo_register_rwlock - Protects registration of file handlers and FLBs.
//
pub static mut luo_register_rwlock: usize = 0;
#[no_mangle]
unsafe extern "C" fn early_liveupdate_param(buf: *mut c_char) -> c_int {
    return kstrtobool(buf, &luo_global.enabled);
    }
    early_param!("liveupdate", early_liveupdate_param);
#[no_mangle]
unsafe extern "C" fn luo_early_startup() -> c_int {
    let mut luo_ser_phys;
pub static mut luo_ser: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    let mut err = 0;
    if (!kho_is_enabled()) {
    if (liveupdate_enabled()) {
    pr_warn!("Disabling liveupdate because KHO is disabled\n");
    }
    luo_global.enabled = false;
    return 0;
    }
// Retrieve LUO state from KHO.
    err = kho_retrieve_subtree(LUO_KHO_ENTRY_NAME, &luo_ser_phys, &len);
    if (err) {
    if (err != -ENOENT) {
    pr_err!("failed to retrieve LUO state '%s' from KHO: %pe\n",
    LUO_KHO_ENTRY_NAME, ERR_PTR(err));
    return err;
    }
    return 0;
    }
    if (len < sizeof!(*luo_ser)) {
    pr_err!("LUO state is too small (%zu < %zu)\n", len, sizeof!(*luo_ser));
    return -EINVAL;
    }
    luo_ser = phys_to_virt(luo_ser_phys);
    if (strncmp(luo_ser.compatible, LUO_ABI_COMPATIBLE, LUO_ABI_COMPAT_LEN)) {
    pr_err!("LUO state is incompatible with '%s'\n", LUO_ABI_COMPATIBLE);
    return -EINVAL;
    }
    luo_global.liveupdate_num = luo_ser.liveupdate_num;
    pr_info!("Retrieved live update data, liveupdate number: %lld\n",
    luo_global.liveupdate_num);
    err = luo_session_setup_incoming(luo_ser.sessions_pa);
    if (err) {
// goto;
    }
    luo_flb_setup_incoming(luo_ser.flbs_pa);
    err = 0;
// label;
    kho_restore_free(luo_ser);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn liveupdate_early_init() -> c_int {
    let mut err = 0;
    err = luo_early_startup();
    if (err) {
    luo_global.enabled = false;
    luo_restore_fail("The incoming tree failed to initialize properly [%pe], disabling live update\n",
    ERR_PTR(err));
    }
    return err;
    }
    early_initcall!(liveupdate_early_init);
// Called during boot to create outgoing LUO state
#[no_mangle]
unsafe extern "C" fn luo_state_setup() -> c_int {
pub static mut luo_ser: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    luo_ser = kho_alloc_preserve(sizeof!(*luo_ser));
    if (IS_ERR(luo_ser)) {
    pr_err!("failed to allocate/preserve LUO state memory\n");
    return PTR_ERR(luo_ser);
    }
    strscpy(luo_ser.compatible, LUO_ABI_COMPATIBLE, sizeof!(luo_ser.compatible));
    luo_ser.liveupdate_num = luo_global.liveupdate_num + 1;
    luo_session_setup_outgoing(&luo_ser.sessions_pa);
    err = luo_flb_setup_outgoing(&luo_ser.flbs_pa);
    if (err) {
// goto;
    }
    err = kho_add_subtree(LUO_KHO_ENTRY_NAME, luo_ser, sizeof!(*luo_ser));
    if (err) {
// goto;
    }
    luo_global.luo_ser_out = luo_ser;
    return 0;
// label;
    kho_unpreserve_free(luo_ser);
    pr_err!("failed to prepare LUO state: %d\n", err);
    return err;
    }
//
// late initcall because it initializes the outgoing tree that is needed only
// once userspace starts using /dev/liveupdate.
//
#[no_mangle]
unsafe extern "C" fn luo_late_startup() -> c_int {
    let mut err = 0;
    if (!liveupdate_enabled()) {
    return 0;
    }
    err = luo_state_setup();
    if (err) {
    luo_global.enabled = false;
    }
    return err;
    }
    late_initcall!(luo_late_startup);
// Public Functions
//
// liveupdate_reboot() - Kernel reboot notifier for live update final
// serialization.
//
// This function is invoked directly from the reboot() syscall pathway
// if kexec is in progress.
//
// If any callback fails, this function aborts KHO, undoes the freeze()
// callbacks, and returns an error.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_reboot() -> c_int {
    let mut err = 0;
    if (!liveupdate_enabled()) {
    return 0;
    }
    err = luo_session_serialize();
    if (err) {
    return err;
    }
    luo_flb_serialize();
    return 0;
    }
//
// liveupdate_enabled - Check if the live update feature is enabled.
//
// This function returns the state of the live update feature flag, which
// can be controlled via the ``liveupdate`` kernel command-line parameter.
//
// @return true if live update is enabled, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_enabled() -> bool {
    return luo_global.enabled;
    }
//
// DOC: LUO ioctl Interface
//
// The IOCTL user-space control interface for the LUO subsystem.
// It registers a character device, typically found at ``/dev/liveupdate``,
// which allows a userspace agent to manage the LUO state machine and its
// associated resources, such as preservable file descriptors.
//
// To ensure that the state machine is controlled by a single entity, access
// to this device is exclusive: only one process is permitted to have
// ``/dev/liveupdate`` open at any given time. Subsequent open attempts will
// fail with -EBUSY until the first process closes its file descriptor.
// This singleton model simplifies state management by preventing conflicting
// commands from multiple userspace agents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_device_state {
    pub miscdev: miscdevice,
    pub in_use: core::sync::atomic::AtomicI32,
}

#[no_mangle]
unsafe extern "C" fn luo_ioctl_create_session(ucmd: *mut luo_ucmd) -> c_int {
    let mut argp = ucmd.cmd;
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    argp.fd = get_unused_fd_flags(O_CLOEXEC);
    if (argp.fd < 0) {
    return argp.fd;
    }
    err = luo_session_create(argp.name, &file);
    if (err) {
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
unsafe extern "C" fn luo_ioctl_retrieve_session(ucmd: *mut luo_ucmd) -> c_int {
    let mut argp = ucmd.cmd;
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    argp.fd = get_unused_fd_flags(O_CLOEXEC);
    if (argp.fd < 0) {
    return argp.fd;
    }
    err = luo_session_retrieve(argp.name, &file);
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
unsafe extern "C" fn luo_open(inodep: *mut inode, filep: *mut file) -> c_int {
    let mut ldev = container_of!(filep.private_data, luo_device_state,
    miscdev);
    if (atomic_cmpxchg(&ldev.in_use, 0, 1)) {
    return -EBUSY;
    }
// Always return -EIO to user if deserialization fail
    if (luo_session_deserialize()) {
    atomic_set(&ldev.in_use, 0);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn luo_release(inodep: *mut inode, filep: *mut file) -> c_int {
    let mut ldev = container_of!(filep.private_data, luo_device_state,
    miscdev);
    atomic_set(&ldev.in_use, 0);
    return 0;
    }
    union ucmd_buffer {
pub static mut create: usize = 0;
pub static mut retrieve: usize = 0;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_ioctl_op {
    pub size: c_uint,
    pub min_size: c_uint,
    pub ioctl_num: c_uint,
    pub ucmd): *mut *mut int (execute)(luo_ucmd,
}

    [_IOC_NR(_ioctl) - LIVEUPDATE_CMD_BASE] = {                            
    .size = sizeof!(_struct) +                                      
    BUILD_BUG_ON_ZERO(sizeof!(union ucmd_buffer) <          
    sizeof!(_struct)),                    
    .min_size = offsetofend(_struct, _last),                       
    .ioctl_num = _ioctl,                                           
    .execute = _fn,                                                
    }
pub static mut luo_ioctl_op: usize = 0;
#[no_mangle]
unsafe extern "C" fn luo_ioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
pub static mut op: *mut c_void = core::ptr::null_mut();
pub static mut ucmd: luo_ucmd = 0;
    union ucmd_buffer buf;
    let mut nr = 0;
    let mut err = 0;
    nr = _IOC_NR(cmd);
    if (nr - LIVEUPDATE_CMD_BASE >= ARRAY_SIZE!(luo_ioctl_ops)) {
    return -EINVAL;
    }
    ucmd.ubuffer = arg;
    err = get_user(ucmd.user_size, ucmd.ubuffer);
    if (err) {
    return err;
    }
    op = &luo_ioctl_ops[nr - LIVEUPDATE_CMD_BASE];
    if (op.ioctl_num != cmd) {
    return -ENOIOCTLCMD;
    }
    if (ucmd.user_size < op.min_size) {
    return -EINVAL;
    }
    ucmd.cmd = &buf;
    err = copy_struct_from_user(ucmd.cmd, op.size, ucmd.ubuffer,
    ucmd.user_size);
    if (err) {
    return err;
    }
    return op.execute(&ucmd);
    }
pub static mut file_operations: usize = 0;
pub static mut luo_device_state: usize = 0;
#[no_mangle]
unsafe extern "C" fn liveupdate_ioctl_init() -> c_int {
    if (!liveupdate_enabled()) {
    return 0;
    }
    return misc_register(&luo_dev.miscdev);
    }
    late_initcall!(liveupdate_ioctl_init);