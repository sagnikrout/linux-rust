//! Automatically rewritten from C to Rust
//! Source: block/bsg.c
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
// bsg.c - block layer implementation of the sg v4 interface
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_device {
    pub queue: *mut request_queue,
    pub device: device,
    pub cdev: cdev,
    pub max_queue: c_int,
    pub timeout: c_uint,
    pub reserved_size: c_uint,
    pub sg_io_fn: *mut bsg_sg_io_fn,
    pub uring_cmd_fn: *mut bsg_uring_cmd_fn,
}

#[no_mangle]
pub unsafe extern "C" fn to_bsg_device(inode: *mut inode) -> *mut c_void {
    return container_of!(inode.i_cdev, bsg_device, cdev);
    }
pub const BSG_DEFAULT_CMDS: c_int = 64;

pub static mut bsg_minor_ida: usize = 0;
pub static mut bsg_class: usize = 0;
    static int bsg_major;
#[no_mangle]
unsafe extern "C" fn bsg_timeout(bd: *mut bsg_device, hdr: *mut sg_io_v4) -> c_uint {
pub static mut timeout: c_uint = 0;
    if (hdr.timeout) {
    timeout = msecs_to_jiffies(hdr.timeout);
    }

    else if (bd.timeout) {
    timeout = bd.timeout;
    }
    return max_t(unsigned int, timeout, BLK_MIN_SG_TIMEOUT);
    }
#[no_mangle]
pub unsafe extern "C" fn bsg_sg_io(bd: *mut bsg_device, open_for_write: bool, uarg: *mut c_void) -> c_int {
pub static mut hdr: usize = 0;
    let mut ret = 0;
    if (copy_from_user(&hdr, uarg, sizeof!(hdr))) {
    return -EFAULT;
    }
    if (hdr.guard != 'Q') {
    return -EINVAL;
    }
    ret = bd.sg_io_fn(bd.queue, &hdr, open_for_write,
    bsg_timeout(bd, &hdr));
    if (!ret && copy_to_user(uarg, &hdr, sizeof!(hdr))) {
    return -EFAULT;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bsg_open(inode: *mut inode, file: *mut file) -> c_int {
    if (!blk_get_queue(to_bsg_device(inode).queue)) {
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_release(inode: *mut inode, file: *mut file) -> c_int {
    blk_put_queue(to_bsg_device(inode).queue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_get_command_q(bd: *mut bsg_device, uarg: *mut int ) -> c_int {
    return put_user(READ_ONCE(bd.max_queue), uarg);
    }
#[no_mangle]
unsafe extern "C" fn bsg_set_command_q(bd: *mut bsg_device, uarg: *mut int ) -> c_int {
    let mut max_queue = 0;
    if (get_user(max_queue, uarg)) {
    return -EFAULT;
    }
    if (max_queue < 1) {
    return -EINVAL;
    }
    WRITE_ONCE(bd.max_queue, max_queue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut bd = to_bsg_device(file_inode(file));
    let mut q = bd.queue;
    let mut uarg =  arg;
    let mut intp = uarg;
    let mut val = 0;
    match (cmd) {
//
// Our own ioctls
//
    SG_GET_COMMAND_Q => {
    return bsg_get_command_q(bd, uarg);
    }
    SG_SET_COMMAND_Q => {
    return bsg_set_command_q(bd, uarg);
//
// SCSI/sg ioctls
//
    }
    SG_GET_VERSION_NUM => {
    return put_user(30527, intp);
    }
    SCSI_IOCTL_GET_IDLUN => {
    return put_user(0, intp);
    }
    SCSI_IOCTL_GET_BUS_NUMBER => {
    return put_user(0, intp);
    }
    SG_SET_TIMEOUT => {
    if (get_user(val, intp)) {
    return -EFAULT;
    }
    bd.timeout = clock_t_to_jiffies(val);
    return 0;
    }
    SG_GET_TIMEOUT => {
    return jiffies_to_clock_t(bd.timeout);
    }
    SG_GET_RESERVED_SIZE => {
    return put_user(min(bd.reserved_size, queue_max_bytes(q)),
    intp);
    }
    SG_SET_RESERVED_SIZE => {
    if (get_user(val, intp)) {
    return -EFAULT;
    }
    if (val < 0) {
    return -EINVAL;
    }
    bd.reserved_size =
    min_t(unsigned int, val, queue_max_bytes(q));
    return 0;
    }
    SG_EMULATED_HOST => {
    return put_user(1, intp);
    }
    SG_IO => {
    return bsg_sg_io(bd, file.f_mode & FMODE_WRITE, uarg);
    }
    SCSI_IOCTL_SEND_COMMAND => {
    pr_warn_ratelimited("%s: calling unsupported SCSI_IOCTL_SEND_COMMAND\n",
    current.comm);
    return -EINVAL;
    }
    _ => {
    return -ENOTTY;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bsg_check_uring_features(issue_flags: c_uint) -> c_int {
// BSG passthrough requires big SQE/CQE support
    if ((issue_flags & (IO_URING_F_SQE128|IO_URING_F_CQE32)) !=
    (IO_URING_F_SQE128|IO_URING_F_CQE32)) {
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_uring_cmd(ioucmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int {
    let mut bd = to_bsg_device(file_inode(ioucmd.file));
pub static mut open_for_write: bool = false;
    let mut q = bd.queue;
    let mut ret = 0;
    ret = bsg_check_uring_features(issue_flags);
    if (ret) {
    return ret;
    }
    if (!bd.uring_cmd_fn) {
    return -EOPNOTSUPP;
    }
    return bd.uring_cmd_fn(q, ioucmd, issue_flags, open_for_write);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn bsg_device_release(dev: *mut device) {
    let mut bd = container_of!(dev, bsg_device, device);
    ida_free(&bsg_minor_ida, MINOR(bd.device.devt));
    kfree(bd);
    }
#[no_mangle]
pub unsafe extern "C" fn bsg_unregister_queue(bd: *mut bsg_device) {
    let mut disk = bd.queue.disk;
    if (disk && disk.queue_kobj.sd) {
    sysfs_remove_link(&disk.queue_kobj, "bsg");
    }
    cdev_device_del(&bd.cdev, &bd.device);
    put_device(&bd.device);
    }
    EXPORT_SYMBOL_GPL(bsg_unregister_queue);
#[no_mangle]
pub unsafe extern "C" fn bsg_register_queue(q: *mut request_queue, parent: *mut device, name: *mut c_char, sg_io_fn: *mut bsg_sg_io_fn, uring_cmd_fn: *mut bsg_uring_cmd_fn) -> *mut c_void {
pub static mut bd: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    bd = kzalloc_obj(*bd);
    if (!bd) {
    return ERR_PTR(-ENOMEM);
    }
    bd.max_queue = BSG_DEFAULT_CMDS;
    bd.reserved_size = INT_MAX;
    bd.queue = q;
    bd.sg_io_fn = sg_io_fn;
    bd.uring_cmd_fn = uring_cmd_fn;
    ret = ida_alloc_max(&bsg_minor_ida, BSG_MAX_DEVS - 1, GFP_KERNEL);
    if (ret < 0) {
    if (ret == -ENOSPC) {
    dev_err(parent, "bsg: too many bsg devices\n");
    }
    kfree(bd);
    return ERR_PTR(ret);
    }
    bd.device.devt = MKDEV(bsg_major, ret);
    bd.device.class = &bsg_class;
    bd.device.parent = parent;
    bd.device.release = bsg_device_release;
    dev_set_name(&bd.device, "%s", name);
    device_initialize(&bd.device);
    cdev_init(&bd.cdev, &bsg_fops);
    bd.cdev.owner = THIS_MODULE;
    ret = cdev_device_add(&bd.cdev, &bd.device);
    if (ret) {
// goto;
    }
    if (q.disk && q.disk.queue_kobj.sd) {
    ret = sysfs_create_link(&q.disk.queue_kobj, &bd.device.kobj,
    "bsg");
    if (ret) {
// goto;
    }
    }
    return bd;
// label;
    cdev_device_del(&bd.cdev, &bd.device);
// label;
    put_device(&bd.device);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(bsg_register_queue);
#[no_mangle]
pub unsafe extern "C" fn bsg_devnode(dev: *mut device, mode: *mut umode_t) -> *mut c_void {
    return kasprintf(GFP_KERNEL, "bsg/%s", dev_name(dev));
    }
pub static mut class: usize = 0;
#[no_mangle]
unsafe extern "C" fn bsg_init() -> c_int {
    let mut devid;
    let mut ret = 0;
    ret = class_register(&bsg_class);
    if (ret) {
    return ret;
    }
    ret = alloc_chrdev_region(&devid, 0, BSG_MAX_DEVS, "bsg");
    if (ret) {
// goto;
    }
    bsg_major = MAJOR(devid);
    printk(KERN_INFO BSG_DESCRIPTION " version " BSG_VERSION
    " loaded (major %d)\n", bsg_major);
    return 0;
// label;
    class_unregister(&bsg_class);
    return ret;
    }
    MODULE_AUTHOR("Jens Axboe");
    MODULE_DESCRIPTION(BSG_DESCRIPTION);
    MODULE_LICENSE("GPL");
    device_initcall!(bsg_init);