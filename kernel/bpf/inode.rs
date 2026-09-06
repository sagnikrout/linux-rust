//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/inode.c
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
//
// Minimal file system backend for holding eBPF maps and programs,
// used by bpf(2) object pinning.
//
// Authors:
//
// Daniel Borkmann <daniel@iogearbox.net>
//

    enum bpf_type {
    BPF_TYPE_UNSPEC	= 0,
    BPF_TYPE_PROG,
    BPF_TYPE_MAP,
    BPF_TYPE_LINK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fs_inode {
    pub xattrs: list_head,
    pub xlimits: simple_xattr_limits,
    pub vfs_inode: inode,
}

#[no_mangle]
pub unsafe extern "C" fn BPF_FS_I(inode: *mut inode) -> *mut c_void {
    return container_of!(inode, bpf_fs_inode, vfs_inode);
    }
pub static mut bpf_fs_inode_cachep: *mut c_void = core::ptr::null_mut();
// forward_decl: bpf_fs_initxattrs;
// forward_decl: bpf_fs_listxattr;
#[no_mangle]
pub unsafe extern "C" fn bpf_any_get(raw: *mut c_void, type: bpf_type) -> *mut c_void {
    match (type) {
    BPF_TYPE_PROG => {
    bpf_prog_inc(raw);
    // break;
    }
    BPF_TYPE_MAP => {
    bpf_map_inc_with_uref(raw);
    // break;
    }
    BPF_TYPE_LINK => {
    bpf_link_inc(raw);
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    // break;
    }
    }
    return raw;
    }
#[no_mangle]
unsafe extern "C" fn bpf_any_put(raw: *mut c_void, type: bpf_type) {
    match (type) {
    BPF_TYPE_PROG => {
    bpf_prog_put(raw);
    // break;
    }
    BPF_TYPE_MAP => {
    bpf_map_put_with_uref(raw);
    // break;
    }
    BPF_TYPE_LINK => {
    bpf_link_put(raw);
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_probe_obj(ufd: u32, type: *mut bpf_type) -> *mut c_void {
pub static mut raw: *mut c_void = core::ptr::null_mut();
    raw = bpf_map_get_with_uref(ufd);
    if (!IS_ERR(raw)) {
// type = BPF_TYPE_MAP;
    return raw;
    }
    raw = bpf_prog_get(ufd);
    if (!IS_ERR(raw)) {
// type = BPF_TYPE_PROG;
    return raw;
    }
    raw = bpf_link_get_from_fd(ufd);
    if (!IS_ERR(raw)) {
// type = BPF_TYPE_LINK;
    return raw;
    }
    return ERR_PTR(-EINVAL);
    }
pub static mut bpf_dir_iops: usize = 0;
pub static mut bpf_symlink_iops: usize = 0;
pub static mut inode_operations: usize = 0;
pub static mut inode_operations: usize = 0;
pub static mut inode_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_get_inode(sb: *mut super_block, dir: *mut inode, mode: umode_t) -> *mut c_void {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    match (mode & S_IFMT) {
    S_IFDIR => {
    }
    S_IFREG => {
    }
    S_IFLNK => {
    // break;
    }
    _ => {
    return ERR_PTR(-EINVAL);
    }
    }
    inode = new_inode(sb);
    if (!inode) {
    return ERR_PTR(-ENOSPC);
    }
    inode.i_ino = get_next_ino();
    simple_inode_init_ts(inode);
    inode_init_owner(&nop_mnt_idmap, inode, dir, mode);
    return inode;
    }
#[no_mangle]
unsafe extern "C" fn bpf_inode_type(inode: *const inode, type: *mut enum bpf_type) -> c_int {
// type = BPF_TYPE_UNSPEC;
    if (inode.i_op == &bpf_prog_iops) {
// type = BPF_TYPE_PROG;
    }

    else if (inode.i_op == &bpf_map_iops) {
// type = BPF_TYPE_MAP;
    }

    else if (inode.i_op == &bpf_link_iops) {
// type = BPF_TYPE_LINK;
    }
    else {
    return -EACCES;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dentry_finalize(dentry: *mut dentry, inode: *mut inode, dir: *mut inode) {
    d_make_persistent(dentry, inode);
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut c_void {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    inode = bpf_get_inode(dir.i_sb, dir, mode | S_IFDIR);
    if (IS_ERR(inode)) {
    return ERR_CAST(inode);
    }
    ret = security_inode_init_security(inode, dir, &dentry.d_name,
    bpf_fs_initxattrs, core::ptr::null_mut());
    if (ret && ret != -EOPNOTSUPP) {
    iput(inode);
    return ERR_PTR(ret);
    }
    inode.i_op = &bpf_dir_iops;
    inode.i_fop = &simple_dir_operations;
    inc_nlink(inode);
    inc_nlink(dir);
    bpf_dentry_finalize(dentry, inode, dir);
    return core::ptr::null_mut();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_iter {
    pub key: *mut c_void,
    pub done: bool,
}

#[no_mangle]
pub unsafe extern "C" fn map_iter(m: *mut seq_file) -> *mut c_void {
    return m.private;
    }
#[no_mangle]
pub unsafe extern "C" fn seq_file_to_map(m: *mut seq_file) -> *mut c_void {
    return file_inode(m.file).i_private;
    }
#[no_mangle]
unsafe extern "C" fn map_iter_free(iter: *mut map_iter) {
    if (iter) {
    kfree(iter.key);
    kfree(iter);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn map_iter_alloc(map: *mut bpf_map) -> *mut c_void {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    iter = kzalloc_obj(*iter, GFP_KERNEL | __GFP_NOWARN);
    if (!iter) {
// goto;
    }
    iter.key = kzalloc(map.key_size, GFP_KERNEL | __GFP_NOWARN);
    if (!iter.key) {
// goto;
    }
    return iter;
// label;
    map_iter_free(iter);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn map_seq_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut map = seq_file_to_map(m);
    let mut key = map_iter(m).key;
pub static mut prev_key: *mut c_void = core::ptr::null_mut();
    (*pos)++;
    if (map_iter(m).done) {
    return core::ptr::null_mut();
    }
    if (unlikely(v == SEQ_START_TOKEN)) {
    prev_key = core::ptr::null_mut();
    }
    else {
    prev_key = key;
    }
    rcu_read_lock();
    if (map.ops.map_get_next_key(map, prev_key, key)) {
    map_iter(m).done = true;
    key = core::ptr::null_mut();
    }
    rcu_read_unlock();
    return key;
    }
#[no_mangle]
pub unsafe extern "C" fn map_seq_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if (map_iter(m).done) {
    return core::ptr::null_mut();
    }
    return *pos ? map_iter(m).key : SEQ_START_TOKEN;
    }
#[no_mangle]
unsafe extern "C" fn map_seq_stop(m: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn map_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut map = seq_file_to_map(m);
    let mut key = map_iter(m).key;
    if (unlikely(v == SEQ_START_TOKEN)) {
    seq_puts(m, "# WARNING!! The output is for debug purpose only\n");
    seq_puts(m, "# WARNING!! The output format will change\n");
    } else {
    map.ops.map_seq_show_elem(map, key, m);
    }
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpffs_map_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut map = inode.i_private;
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut m: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    iter = map_iter_alloc(map);
    if (!iter) {
    return -ENOMEM;
    }
    err = seq_open(file, &bpffs_map_seq_ops);
    if (err) {
    map_iter_free(iter);
    return err;
    }
    m = file.private_data;
    m.private = iter;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpffs_map_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut m = file.private_data;
    map_iter_free(map_iter(m));
    return seq_release(inode, file);
    }
// bpffs_map_fops should only implement the basic
// read operation for a BPF map.  The purpose is to
// provide a simple user intuitive way to do
// "cat bpffs/pathto/a-pinned-map".
//
// Other operations (e.g. write, lookup...) should be realized by
// the userspace tools (e.g. bpftool) through the
// BPF_OBJ_GET_INFO_BY_FD and the map's lookup/update
// interface.
//
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpffs_obj_open(inode: *mut inode, file: *mut file) -> c_int {
    return -EIO;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_mkobj_ops(dentry: *mut dentry, mode: umode_t, raw: *mut c_void, iops: *mut inode_operations, fops: *mut file_operations) -> c_int {
    let mut dir = dentry.d_parent.d_inode;
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    inode = bpf_get_inode(dir.i_sb, dir, mode);
    if (IS_ERR(inode)) {
    return PTR_ERR(inode);
    }
    ret = security_inode_init_security(inode, dir, &dentry.d_name,
    bpf_fs_initxattrs, core::ptr::null_mut());
    if (ret && ret != -EOPNOTSUPP) {
    iput(inode);
    return ret;
    }
    inode.i_op = iops;
    inode.i_fop = fops;
    inode.i_private = raw;
    bpf_dentry_finalize(dentry, inode, dir);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_mkprog(dentry: *mut dentry, mode: umode_t, arg: *mut c_void) -> c_int {
    return bpf_mkobj_ops(dentry, mode, arg, &bpf_prog_iops,
    &bpffs_obj_fops);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mkmap(dentry: *mut dentry, mode: umode_t, arg: *mut c_void) -> c_int {
    let mut map = arg;
    return bpf_mkobj_ops(dentry, mode, arg, &bpf_map_iops,
    bpf_map_support_seq_show(map) ?
    &bpffs_map_fops : &bpffs_obj_fops);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mklink(dentry: *mut dentry, mode: umode_t, arg: *mut c_void) -> c_int {
    let mut link = arg;
    return bpf_mkobj_ops(dentry, mode, arg, &bpf_link_iops,
    bpf_link_is_iter(link) ?
    &bpf_iter_fops : &bpffs_obj_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_lookup(dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut c_void {
// Dots in names (e.g. "/sys/fs/bpf/foo.bar") are reserved for future
// extensions. That allows popoulate_bpffs() create special files.
//
    if ((dir.i_mode & S_IALLUGO) &&
    strchr(dentry.d_name.name, '.')) {
    return ERR_PTR(-EPERM);
    }
    return simple_lookup(dir, dentry, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, target: *mut c_char) -> c_int {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut link: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    link = kstrdup(target, GFP_KERNEL_ACCOUNT | __GFP_NOWARN);
    if (!link) {
    return -ENOMEM;
    }
    inode = bpf_get_inode(dir.i_sb, dir, S_IRWXUGO | S_IFLNK);
    if (IS_ERR(inode)) {
    kfree(link);
    return PTR_ERR(inode);
    }
    inode.i_op = &bpf_symlink_iops;
    inode.i_link = link;
    ret = security_inode_init_security(inode, dir, &dentry.d_name,
    bpf_fs_initxattrs, core::ptr::null_mut());
    if (ret && ret != -EOPNOTSUPP) {
    iput(inode);
    return ret;
    }
    bpf_dentry_finalize(dentry, inode, dir);
    return 0;
    }
pub static mut inode_operations: usize = 0;
pub static mut inode_operations: usize = 0;
// pin iterator link into bpffs
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_link_pin_kernel(parent: *mut dentry, name: *mut c_char, link: *mut bpf_link) -> c_int {
pub static mut mode: umode_t = 0;
pub static mut dentry: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    dentry = simple_start_creating(parent, name);
    if (IS_ERR(dentry)) {
    return PTR_ERR(dentry);
    }
    ret = bpf_mkobj_ops(dentry, mode, link, &bpf_link_iops,
    &bpf_iter_fops);
    simple_done_creating(dentry);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_do_pin(path_fd: c_int, pathname: *mut c_char, raw: *mut c_void, type: bpf_type) -> c_int {
pub static mut dentry: *mut c_void = core::ptr::null_mut();
pub static mut dir: *mut c_void = core::ptr::null_mut();
pub static mut path: usize = 0;
    let mut mode;
    let mut ret = 0;
    dentry = start_creating_user_path(path_fd, pathname, &path, 0);
    if (IS_ERR(dentry)) {
    return PTR_ERR(dentry);
    }
    dir = d_inode(path.dentry);
    if (dir.i_op != &bpf_dir_iops) {
    ret = -EPERM;
// goto;
    }
    mode = S_IFREG | ((S_IRUSR | S_IWUSR) & ~current_umask());
    ret = security_path_mknod(&path, dentry, mode, 0);
    if (ret) {
// goto;
    }
    match (type) {
    BPF_TYPE_PROG => {
    ret = vfs_mkobj(dentry, mode, bpf_mkprog, raw);
    // break;
    }
    BPF_TYPE_MAP => {
    ret = vfs_mkobj(dentry, mode, bpf_mkmap, raw);
    // break;
    }
    BPF_TYPE_LINK => {
    ret = vfs_mkobj(dentry, mode, bpf_mklink, raw);
    // break;
    }
    _ => {
    ret = -EPERM;
    }
    }
// label;
    end_creating_path(&path, dentry);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_pin_user(ufd: u32, path_fd: c_int, pathname: *const char ) -> c_int {
    enum bpf_type type;
pub static mut raw: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    raw = bpf_fd_probe_obj(ufd, &type);
    if (IS_ERR(raw)) {
    return PTR_ERR(raw);
    }
    ret = bpf_obj_do_pin(path_fd, pathname, raw, type);
    if (ret != 0) {
    bpf_any_put(raw, type);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_do_get(path_fd: c_int, pathname: *mut c_char, type: *mut bpf_type, flags: c_int) -> *mut c_void {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut path: usize = 0;
pub static mut raw: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = user_path_at(path_fd, pathname, LOOKUP_FOLLOW, &path);
    if (ret) {
    return ERR_PTR(ret);
    }
    inode = d_backing_inode(path.dentry);
    ret = path_permission(&path, ACC_MODE(flags));
    if (ret) {
// goto;
    }
    ret = bpf_inode_type(inode, type);
    if (ret) {
// goto;
    }
    raw = bpf_any_get(inode.i_private, *type);
    if (!IS_ERR(raw)) {
    touch_atime(&path);
    }
    path_put(&path);
    return raw;
// label;
    path_put(&path);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_get_user(path_fd: c_int, pathname: *const char , flags: c_int) -> c_int {
pub static mut type: bpf_type = 0;
    let mut f_flags = 0;
pub static mut raw: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    f_flags = bpf_get_file_flag(flags);
    if (f_flags < 0) {
    return f_flags;
    }
    raw = bpf_obj_do_get(path_fd, pathname, &type, f_flags);
    if (IS_ERR(raw)) {
    return PTR_ERR(raw);
    }
    if (type == BPF_TYPE_PROG) {
    ret = bpf_prog_new_fd(raw);
    }

    else if (type == BPF_TYPE_MAP) {
    ret = bpf_map_new_fd(raw, f_flags);
    }

    else if (type == BPF_TYPE_LINK) {
    ret = (f_flags != O_RDWR) ? -EINVAL : bpf_link_new_fd(raw);
    }
    else {
    return -ENOENT;
    }
    if (ret < 0) {
    bpf_any_put(raw, type);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __get_prog_inode(inode: *mut inode, type: bpf_prog_type) -> *mut c_void {
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (ret) {
    return ERR_PTR(ret);
    }
    if (inode.i_op == &bpf_map_iops) {
    return ERR_PTR(-EINVAL);
    }
    if (inode.i_op == &bpf_link_iops) {
    return ERR_PTR(-EINVAL);
    }
    if (inode.i_op != &bpf_prog_iops) {
    return ERR_PTR(-EACCES);
    }
    prog = inode.i_private;
    ret = security_bpf_prog(prog);
    if (ret < 0) {
    return ERR_PTR(ret);
    }
    if (!bpf_prog_get_ok(prog, &type, false)) {
    return ERR_PTR(-EINVAL);
    }
    bpf_prog_inc(prog);
    return prog;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_get_type_path(name: *mut c_char, type: bpf_prog_type) -> *mut c_void {
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut path: usize = 0;
pub static mut ret: c_int = 0;
    if (ret) {
    return ERR_PTR(ret);
    }
    prog = __get_prog_inode(d_backing_inode(path.dentry), type);
    if (!IS_ERR(prog)) {
    touch_atime(&path);
    }
    path_put(&path);
    return prog;
    }
    EXPORT_SYMBOL(bpf_prog_get_type_path);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpffs_btf_enums {
    pub btf: *const btf,
    pub cmd_t: *const btf_type,
    pub map_t: *const btf_type,
    pub prog_t: *const btf_type,
    pub attach_t: *const btf_type,
}

#[no_mangle]
unsafe extern "C" fn find_bpffs_btf_enums(info: *mut bpffs_btf_enums) -> c_int {
pub static mut btf_enums: usize = 0;
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut id = 0;
    memset(info, 0, sizeof!(*info));
    btf = bpf_get_btf_vmlinux();
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    if (!btf) {
    return -ENOENT;
    }
    info.btf = btf;
    while (i < ARRAY_SIZE!(btf_enums)) {
    id = btf_find_by_name_kind(btf, btf_enums[i].name,
    BTF_KIND_ENUM);
    if (id < 0) {
    return -ESRCH;
    }
// btf_enums[i].type = btf_type_by_id(btf, id);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn find_btf_enum_const(btf: *mut btf, enum_t: *mut btf_type, prefix: *mut c_char, str: *mut c_char, value: *mut c_int) -> bool {
pub static mut e: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    int i, n, pfx_len = strlen(prefix);
// value = 0;
    if (!btf || !enum_t) {
    return false;
    }
    while (i < n) {
    e = &btf_enum(enum_t)[i];
    name = btf_name_by_offset(btf, e.name_off);
    if (!name || strncasecmp(name, prefix, pfx_len) != 0) {
    continue;
    }
// match symbolic name case insensitive and ignoring prefix
    if (strcasecmp(name + pfx_len, str) == 0) {
// value = e->val;
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn seq_print_delegate_opts(m: *mut seq_file, opt_name: *mut c_char, btf: *mut btf, enum_t: *mut btf_type, prefix: *mut c_char, delegate_msk: u64, any_msk: u64) {
pub static mut e: *mut c_void = core::ptr::null_mut();
pub static mut first: bool = true;
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut msk = 0;
    int i, n, pfx_len = strlen(prefix);
    delegate_msk &= any_msk; /* clear unknown bits */
    if (delegate_msk == 0) {
    return;
    }
    seq_printf(m, ",%s", opt_name);
    if (delegate_msk == any_msk) {
    seq_printf(m, "=any");
    return;
    }
    if (btf && enum_t) {
    while (i < n) {
    e = &btf_enum(enum_t)[i];
    name = btf_name_by_offset(btf, e.name_off);
    if (!name || strncasecmp(name, prefix, pfx_len) != 0) {
    continue;
    }
    msk = 1ULL << e.val;
    if (delegate_msk & msk) {
// emit lower-case name without prefix
    seq_putc(m, first ? '=' : ':');
    name += pfx_len;
    while (*name) {
    seq_putc(m, tolower(*name));
    name += 1;
    }
    delegate_msk &= ~msk;
    first = false;
    }
    }
    }
    if (delegate_msk) {
    seq_printf(m, "%c0x%llx", first ? '=' : ':', delegate_msk);
    }
    }
//
// Display the mount options in /proc/mounts.
//
#[no_mangle]
unsafe extern "C" fn bpf_show_options(m: *mut seq_file, root: *mut dentry) -> c_int {
    let mut inode = d_inode(root);
pub static mut mode: umode_t = 0;
    let mut opts = root.d_sb.s_fs_info;
    let mut mask = 0;
    if (!uid_eq(inode.i_uid, GLOBAL_ROOT_UID)) {
    seq_printf(m, ",uid=%u",
    from_kuid_munged(&init_user_ns, inode.i_uid));
    }
    if (!gid_eq(inode.i_gid, GLOBAL_ROOT_GID)) {
    seq_printf(m, ",gid=%u",
    from_kgid_munged(&init_user_ns, inode.i_gid));
    }
    if (mode != S_IRWXUGO) {
    seq_printf(m, ",mode=%o", mode);
    }
    if (opts.delegate_cmds || opts.delegate_maps ||
    opts.delegate_progs || opts.delegate_attachs) {
pub static mut info: usize = 0;
// ignore errors, fallback to hex
    (void)find_bpffs_btf_enums(&info);
    mask = (1ULL << __MAX_BPF_CMD) - 1;
    seq_print_delegate_opts(m, "delegate_cmds",
    info.btf, info.cmd_t, "BPF_",
    opts.delegate_cmds, mask);
    mask = (1ULL << __MAX_BPF_MAP_TYPE) - 1;
    seq_print_delegate_opts(m, "delegate_maps",
    info.btf, info.map_t, "BPF_MAP_TYPE_",
    opts.delegate_maps, mask);
    mask = (1ULL << __MAX_BPF_PROG_TYPE) - 1;
    seq_print_delegate_opts(m, "delegate_progs",
    info.btf, info.prog_t, "BPF_PROG_TYPE_",
    opts.delegate_progs, mask);
    mask = (1ULL << __MAX_BPF_ATTACH_TYPE) - 1;
    seq_print_delegate_opts(m, "delegate_attachs",
    info.btf, info.attach_t, "BPF_",
    opts.delegate_attachs, mask);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fs_alloc_inode(sb: *mut super_block) -> *mut c_void {
pub static mut bi: *mut c_void = core::ptr::null_mut();
    bi = alloc_inode_sb(sb, bpf_fs_inode_cachep, GFP_KERNEL);
    if (!bi) {
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD_RCU(&bi.xattrs);
    simple_xattr_limits_init(&bi.xlimits);
    return &bi.vfs_inode;
    }
#[no_mangle]
unsafe extern "C" fn bpf_destroy_inode(inode: *mut inode) {
    let mut opts = inode.i_sb.s_fs_info;
    let mut bi = BPF_FS_I(inode);
    enum bpf_type type;
    if (!bpf_inode_type(inode, &type)) {
    bpf_any_put(inode.i_private, type);
    }
    simple_xattrs_free(&opts.xa_cache, &bi.xattrs, core::ptr::null_mut());
    }
//
// Called after RCU grace period - safe to free inode and anything
// that might be accessed by RCU pathwalk (inode fields, i_link).
//
#[no_mangle]
unsafe extern "C" fn bpf_free_inode(inode: *mut inode) {
    if (S_ISLNK(inode.i_mode)) {
    kfree(inode.i_link);
    }
    kmem_cache_free(bpf_fs_inode_cachep, BPF_FS_I(inode));
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fs_xattr_get(handler: *mut xattr_handler, unused: *mut dentry, inode: *mut inode, name: *mut c_char, value: *mut c_void, size: size_t) -> c_int {
    let mut opts = inode.i_sb.s_fs_info;
    let mut bi = BPF_FS_I(inode);
    name = xattr_full_name(handler, name);
    return simple_xattr_get(&opts.xa_cache, &bi.xattrs, name, value, size);
    }
    enum {
    BPF_FS_XATTR_UNSPEC,
    BPF_FS_XATTR_SECURITY,
    BPF_FS_XATTR_TRUSTED,
    };
#[no_mangle]
pub unsafe extern "C" fn bpf_fs_xattr_set(handler: *mut xattr_handler, idmap: *mut mnt_idmap, unused: *mut dentry, inode: *mut inode, name: *mut c_char, value: *mut c_void, size: size_t, flags: c_int) -> c_int {
    let mut opts = inode.i_sb.s_fs_info;
    let mut bi = BPF_FS_I(inode);
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    name = xattr_full_name(handler, name);
    match (handler.flags) {
    BPF_FS_XATTR_SECURITY => {
    err = simple_xattr_set_limited(&opts.xa_cache, &bi.xattrs,
    &bi.xlimits, name, value, size,
    flags);
    // break;
    }
    BPF_FS_XATTR_TRUSTED => {
    old = simple_xattr_set(&opts.xa_cache, &bi.xattrs, name,
    value, size, flags);
    err = IS_ERR(old) ? PTR_ERR(old) : 0;
    if (!err) {
    simple_xattr_free_rcu(old);
    }
    // break;
    }
    }
    if (err) {
    return err;
    }
    inode_set_ctime_current(inode);
    return 0;
    }
pub static mut xattr_handler: usize = 0;
pub static mut xattr_handler: usize = 0;
    static const struct xattr_handler * const bpf_fs_xattr_handlers[] = {
    &bpf_fs_trusted_xattr_handler,
    &bpf_fs_security_xattr_handler,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn bpf_fs_listxattr(dentry: *mut dentry, buf: *mut c_char, size: usize) -> isize {
    let mut inode = d_inode(dentry);
    return simple_xattr_list(inode, &BPF_FS_I(inode).xattrs, buf, size);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fs_initxattrs(inode: *mut inode, xattr_array: *mut xattr, fs_info: *mut c_void) -> c_int {
    let mut opts = inode.i_sb.s_fs_info;
    let mut bi = BPF_FS_I(inode);
pub static mut xattr: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    while (xattr.name != core::ptr::null_mut()) {
    CLASS(simple_xattr, new_xattr)(xattr.value, xattr.value_len);
    if (IS_ERR(new_xattr)) {
    return PTR_ERR(new_xattr);
    }
    new_xattr.name = kasprintf(GFP_KERNEL_ACCOUNT,
    XATTR_SECURITY_PREFIX "%s",
    xattr.name);
    if (!new_xattr.name) {
    return -ENOMEM;
    }
    err = simple_xattr_add_limited(&opts.xa_cache, &bi.xattrs,
    &bi.xlimits, new_xattr);
    if (err) {
    return err;
    }
    retain_and_null_ptr(new_xattr);
    }
    return 0;
    }
pub static mut super_operations: usize = 0;
    enum {
    OPT_UID,
    OPT_GID,
    OPT_MODE,
    OPT_DELEGATE_CMDS,
    OPT_DELEGATE_MAPS,
    OPT_DELEGATE_PROGS,
    OPT_DELEGATE_ATTACHS,
    };
pub static mut fs_parameter_spec: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let mut opts = fc.s_fs_info;
pub static mut result: usize = 0;
    let mut uid;
    let mut gid;
    let mut opt = 0;
    let mut err = 0;
    opt = fs_parse(fc, bpf_fs_parameters, param, &result);
    if (opt < 0) {
// We might like to report bad mount options here, but
// traditionally we've ignored all mount options, so we'd
// better continue to ignore non-existing options for bpf.
//
    if (opt == -ENOPARAM) {
    opt = vfs_parse_fs_param_source(fc, param);
    if (opt != -ENOPARAM) {
    return opt;
    }
    return 0;
    }
    if (opt < 0) {
    return opt;
    }
    }
    match (opt) {
    OPT_UID => {
    uid = make_kuid(current_user_ns(), result.uint_32);
    if (!uid_valid(uid)) {
// goto;
    }
//
// The requested uid must be representable in the
// filesystem's idmapping.
//
    if (!kuid_has_mapping(fc.user_ns, uid)) {
// goto;
    }
    opts.uid = uid;
    // break;
    }
    OPT_GID => {
    gid = make_kgid(current_user_ns(), result.uint_32);
    if (!gid_valid(gid)) {
// goto;
    }
//
// The requested gid must be representable in the
// filesystem's idmapping.
//
    if (!kgid_has_mapping(fc.user_ns, gid)) {
// goto;
    }
    opts.gid = gid;
    // break;
    }
    OPT_MODE => {
    opts.mode = result.uint_32 & S_IALLUGO;
    // break;
    }
    OPT_DELEGATE_CMDS => {
    }
    OPT_DELEGATE_MAPS => {
    }
    OPT_DELEGATE_PROGS => {
    }
    OPT_DELEGATE_ATTACHS => {
pub static mut info: usize = 0;
pub static mut enum_t: *mut c_void = core::ptr::null_mut();
pub static mut enum_pfx: *mut c_void = core::ptr::null_mut();
    u64 *delegate_msk, msk = 0;
    let mut p = core::ptr::null_mut();
    let mut str = core::ptr::null_mut();
    let mut val = 0;
// ignore errors, fallback to hex
    (void)find_bpffs_btf_enums(&info);
    match (opt) {
    OPT_DELEGATE_CMDS => {
    delegate_msk = &opts.delegate_cmds;
    enum_t = info.cmd_t;
    enum_pfx = "BPF_";
    // break;
    }
    OPT_DELEGATE_MAPS => {
    delegate_msk = &opts.delegate_maps;
    enum_t = info.map_t;
    enum_pfx = "BPF_MAP_TYPE_";
    // break;
    }
    OPT_DELEGATE_PROGS => {
    delegate_msk = &opts.delegate_progs;
    enum_t = info.prog_t;
    enum_pfx = "BPF_PROG_TYPE_";
    // break;
    }
    OPT_DELEGATE_ATTACHS => {
    delegate_msk = &opts.delegate_attachs;
    enum_t = info.attach_t;
    enum_pfx = "BPF_";
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    str = param.string;
    while ((p = strsep(&str, ":"))) {
    if (strcmp(p, "any") == 0) {
    msk |= ~0ULL;
    } else if (find_btf_enum_const(info.btf, enum_t, enum_pfx, p, &val)) {
    msk |= 1ULL << val;
    } else {
    err = kstrtou64(p, 0, &msk);
    if (err) {
    return err;
    }
    }
    }
// Setting delegation mount options requires privileges
    if (msk && !capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
// delegate_msk |= msk;
    break;
    }
// label;
// ignore unknown mount options
    break;
    }
    return 0;
// label;
    return invalfc(fc, "Bad value for '%s'", param.key);
    }
pub static mut bpf_preload_ops: *mut c_void = core::ptr::null_mut();
    EXPORT_SYMBOL_GPL(bpf_preload_ops);
#[no_mangle]
unsafe extern "C" fn bpf_preload_mod_get() -> bool {
// If bpf_preload.ko wasn't loaded earlier then load it now.
// When bpf_preload is built into vmlinux the module's __init
// function will populate it.
//
    if (!bpf_preload_ops) {
    request_module("bpf_preload");
    if (!bpf_preload_ops) {
    return false;
    }
    }
// And grab the reference, so the module doesn't disappear while the
// kernel is interacting with the kernel module and its UMD.
//
    if (!try_module_get(bpf_preload_ops.owner)) {
    pr_err!("bpf_preload module get failed.\n");
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn bpf_preload_mod_put() {
    if (bpf_preload_ops) {
// now user can "rmmod bpf_preload" if necessary
    module_put!(bpf_preload_ops.owner);
    }
    }
pub static mut bpf_preload_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn populate_bpffs(parent: *mut dentry) -> c_int {
pub static mut bpf_preload_info: usize = 0;
pub static mut err: c_int = 0;
// grab the mutex to make sure the kernel interactions with bpf_preload
// are serialized
//
    mutex_lock(&bpf_preload_lock);
// if bpf_preload.ko wasn't built into vmlinux then load it
    if (!bpf_preload_mod_get()) {
// goto;
    }
    err = bpf_preload_ops.preload(objs);
    if (err) {
// goto;
    }
    while (i < BPF_PRELOAD_LINKS) {
    bpf_link_inc(objs[i].link);
    err = bpf_iter_link_pin_kernel(parent,
    objs[i].link_name, objs[i].link);
    if (err) {
    bpf_link_put(objs[i].link);
// goto;
    }
    }
// label;
    bpf_preload_mod_put();
// label;
    mutex_unlock(&bpf_preload_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpf_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let mut opts = sb.s_fs_info;
pub static mut inode: *mut c_void = core::ptr::null_mut();
// Mounting an instance of BPF FS requires privileges
    if (fc.user_ns != &init_user_ns && !capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    sb.s_blocksize = PAGE_SIZE;
    sb.s_blocksize_bits = PAGE_SHIFT;
    sb.s_magic = BPF_FS_MAGIC;
    sb.s_op = &bpf_super_ops;
    sb.s_xattr = bpf_fs_xattr_handlers;
    sb.s_iflags |= SB_I_NOEXEC;
    sb.s_iflags |= SB_I_NODEV;
    sb.s_time_gran = 1;
    inode = bpf_get_inode(sb, core::ptr::null_mut(), S_IFDIR | 0777);
    if (IS_ERR(inode)) {
    return PTR_ERR(inode);
    }
    inode.i_ino = 1;
    inode.i_op = &bpf_dir_iops;
    inode.i_fop = &simple_dir_operations;
    set_nlink(inode, 2);
    sb.s_root = d_make_root(inode);
    if (!sb.s_root) {
    return -ENOMEM;
    }
    inode = d_inode(sb.s_root);
    inode.i_uid = opts.uid;
    inode.i_gid = opts.gid;
    inode.i_mode &= ~S_IALLUGO;
    populate_bpffs(sb.s_root);
    inode.i_mode |= S_ISVTX | opts.mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_get_tree(fc: *mut fs_context) -> c_int {
    return get_tree_nodev(fc, bpf_fill_super);
    }
#[no_mangle]
unsafe extern "C" fn bpf_free_fc(fc: *mut fs_context) {
    kfree(fc.s_fs_info);
    }
pub static mut fs_context_operations: usize = 0;
//
// Set up the filesystem mount context.
//
#[no_mangle]
unsafe extern "C" fn bpf_init_fs_context(fc: *mut fs_context) -> c_int {
pub static mut opts: *mut c_void = core::ptr::null_mut();
    opts = kzalloc_obj(bpf_mount_opts);
    if (!opts) {
    return -ENOMEM;
    }
    opts.mode = S_IRWXUGO;
    opts.uid = current_fsuid();
    opts.gid = current_fsgid();
// start out with no BPF token delegation enabled
    opts.delegate_cmds = 0;
    opts.delegate_maps = 0;
    opts.delegate_progs = 0;
    opts.delegate_attachs = 0;
    fc.s_fs_info = opts;
    fc.ops = &bpf_context_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_kill_super(sb: *mut super_block) {
    let mut opts = sb.s_fs_info;
    kill_anon_super(sb);
    simple_xattr_cache_cleanup(&opts.xa_cache);
    kfree(opts);
    }
pub static mut file_system_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_fs_inode_init_once(foo: *mut c_void) {
    let mut bi = foo;
    inode_init_once(&bi.vfs_inode);
    }
#[no_mangle]
unsafe extern "C" fn bpf_init() -> c_int {
    let mut ret = 0;
    bpf_fs_inode_cachep = kmem_cache_create("bpf_fs_inode_cache",
    sizeof!(bpf_fs_inode),
    0, SLAB_ACCOUNT,
    bpf_fs_inode_init_once);
    if (!bpf_fs_inode_cachep) {
    return -ENOMEM;
    }
    ret = sysfs_create_mount_point(fs_kobj, "bpf");
    if (ret) {
// goto;
    }
    ret = register_filesystem(&bpf_fs_type);
    if (ret) {
    sysfs_remove_mount_point(fs_kobj, "bpf");
// goto;
    }
    return 0;
// label;
    kmem_cache_destroy(bpf_fs_inode_cachep);
    return ret;
    }
    fs_initcall!(bpf_init);