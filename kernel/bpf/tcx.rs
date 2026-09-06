//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/tcx.c
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
// Copyright (c) 2023 Isovalent

#[no_mangle]
pub unsafe extern "C" fn tcx_prog_attach(attr: *const union bpf_attr, prog: *mut bpf_prog) -> c_int {
    bool created, ingress = attr.attach_type == BPF_TCX_INGRESS;
    let mut net = current.nsproxy.net_ns;
    let mut entry = core::ptr::null_mut();
    let mut entry_new = core::ptr::null_mut();
    let mut replace_prog = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    rtnl_lock();
    dev = __dev_get_by_index(net, attr.target_ifindex);
    if (!dev) {
    ret = -ENODEV;
// goto;
    }
    if (attr.attach_flags & BPF_F_REPLACE) {
    replace_prog = bpf_prog_get_type(attr.replace_bpf_fd,
    prog.type);
    if (IS_ERR(replace_prog)) {
    ret = PTR_ERR(replace_prog);
    replace_prog = core::ptr::null_mut();
// goto;
    }
    }
    entry = tcx_entry_fetch_or_create(dev, ingress, &created);
    if (!entry) {
    ret = -ENOMEM;
// goto;
    }
    ret = bpf_mprog_attach(entry, &entry_new, prog, core::ptr::null_mut(), replace_prog,
    attr.attach_flags, attr.relative_fd,
    attr.expected_revision);
    if (!ret) {
    if (entry != entry_new) {
    tcx_entry_update(dev, entry_new, ingress);
    tcx_entry_sync();
    tcx_skeys_inc(ingress);
    }
    bpf_mprog_commit(entry);
    } else if (created) {
    tcx_entry_free(entry);
    }
// label;
    if (replace_prog) {
    bpf_prog_put(replace_prog);
    }
    rtnl_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_prog_detach(attr: *const union bpf_attr, prog: *mut bpf_prog) -> c_int {
pub static mut ingress: bool = false;
    let mut net = current.nsproxy.net_ns;
    let mut entry = core::ptr::null_mut();
    let mut entry_new = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    rtnl_lock();
    dev = __dev_get_by_index(net, attr.target_ifindex);
    if (!dev) {
    ret = -ENODEV;
// goto;
    }
    entry = tcx_entry_fetch(dev, ingress);
    if (!entry) {
    ret = -ENOENT;
// goto;
    }
    ret = bpf_mprog_detach(entry, &entry_new, prog, core::ptr::null_mut(), attr.attach_flags,
    attr.relative_fd, attr.expected_revision);
    if (!ret) {
    if (!tcx_entry_is_active(entry_new)) {
    entry_new = core::ptr::null_mut();
    }
    tcx_entry_update(dev, entry_new, ingress);
    tcx_entry_sync();
    tcx_skeys_dec(ingress);
    bpf_mprog_commit(entry);
    if (!entry_new) {
    tcx_entry_free(entry);
    }
    }
// label;
    rtnl_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_uninstall(dev: *mut net_device, ingress: bool) {
    struct bpf_mprog_entry *entry, *entry_new = core::ptr::null_mut();
pub static mut tuple: bpf_tuple = 0;
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
    let mut active = 0;
    entry = tcx_entry_fetch(dev, ingress);
    if (!entry) {
    return;
    }
    active = tcx_entry(entry).miniq_active;
    if (active) {
    bpf_mprog_clear_all(entry, &entry_new);
    }
    tcx_entry_update(dev, entry_new, ingress);
    tcx_entry_sync();
    bpf_mprog_foreach_tuple(entry, fp, cp, tuple) {
    if (tuple.link) {
    tcx_link(tuple.link).dev = core::ptr::null_mut();
    }
    else {
    bpf_prog_put(tuple.prog);
    }
    tcx_skeys_dec(ingress);
    }
    if (!active) {
    tcx_entry_free(entry);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_prog_query(attr: *const union bpf_attr, uattr: *mut union bpf_attr ) -> c_int {
pub static mut ingress: bool = false;
    let mut net = current.nsproxy.net_ns;
pub static mut dev: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    rtnl_lock();
    dev = __dev_get_by_index(net, attr.query.target_ifindex);
    if (!dev) {
    ret = -ENODEV;
// goto;
    }
    ret = bpf_mprog_query(attr, uattr, tcx_entry_fetch(dev, ingress));
// label;
    rtnl_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_link_prog_attach(link: *mut bpf_link, flags: u32, id_or_fd: u32, revision: u64) -> c_int {
    let mut tcx = tcx_link(link);
    bool created, ingress = link.attach_type == BPF_TCX_INGRESS;
    let mut entry = core::ptr::null_mut();
    let mut entry_new = core::ptr::null_mut();
    let mut dev = tcx.dev;
    let mut ret = 0;
    ASSERT_RTNL();
    entry = tcx_entry_fetch_or_create(dev, ingress, &created);
    if (!entry) {
    return -ENOMEM;
    }
    ret = bpf_mprog_attach(entry, &entry_new, link.prog, link, core::ptr::null_mut(), flags,
    id_or_fd, revision);
    if (!ret) {
    if (entry != entry_new) {
    tcx_entry_update(dev, entry_new, ingress);
    tcx_entry_sync();
    tcx_skeys_inc(ingress);
    }
    bpf_mprog_commit(entry);
    } else if (created) {
    tcx_entry_free(entry);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tcx_link_release(link: *mut bpf_link) {
    let mut tcx = tcx_link(link);
pub static mut ingress: bool = false;
    let mut entry = core::ptr::null_mut();
    let mut entry_new = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    rtnl_lock();
    dev = tcx.dev;
    if (!dev) {
// goto;
    }
    entry = tcx_entry_fetch(dev, ingress);
    if (!entry) {
    ret = -ENOENT;
// goto;
    }
    ret = bpf_mprog_detach(entry, &entry_new, link.prog, link, 0, 0, 0);
    if (!ret) {
    if (!tcx_entry_is_active(entry_new)) {
    entry_new = core::ptr::null_mut();
    }
    tcx_entry_update(dev, entry_new, ingress);
    tcx_entry_sync();
    tcx_skeys_dec(ingress);
    bpf_mprog_commit(entry);
    if (!entry_new) {
    tcx_entry_free(entry);
    }
    tcx.dev = core::ptr::null_mut();
    }
// label;
    WARN_ON_ONCE!(ret);
    rtnl_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_link_update(link: *mut bpf_link, nprog: *mut bpf_prog, oprog: *mut bpf_prog) -> c_int {
    let mut tcx = tcx_link(link);
pub static mut ingress: bool = false;
    let mut entry = core::ptr::null_mut();
    let mut entry_new = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    rtnl_lock();
    dev = tcx.dev;
    if (!dev) {
    ret = -ENOLINK;
// goto;
    }
    if (oprog && link.prog != oprog) {
    ret = -EPERM;
// goto;
    }
    oprog = link.prog;
    if (oprog == nprog) {
    bpf_prog_put(nprog);
// goto;
    }
    entry = tcx_entry_fetch(dev, ingress);
    if (!entry) {
    ret = -ENOENT;
// goto;
    }
    ret = bpf_mprog_attach(entry, &entry_new, nprog, link, oprog,
    BPF_F_REPLACE | BPF_F_ID,
    link.prog.aux.id, 0);
    if (!ret) {
    WARN_ON_ONCE!(entry != entry_new);
    oprog = xchg(&link.prog, nprog);
    bpf_prog_put(oprog);
    bpf_mprog_commit(entry);
    }
// label;
    rtnl_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tcx_link_dealloc(link: *mut bpf_link) {
    kfree(tcx_link(link));
    }
#[no_mangle]
unsafe extern "C" fn tcx_link_fdinfo(link: *const bpf_link, seq: *mut seq_file) {
    let mut tcx = tcx_link(link);
pub static mut ifindex: u32 = 0;
    rtnl_lock();
    if (tcx.dev) {
    ifindex = tcx.dev.ifindex;
    }
    rtnl_unlock();
    seq_printf(seq, "ifindex:\t%u\n", ifindex);
    seq_printf(seq, "attach_type:\t%u (%s)\n",
    link.attach_type,
    link.attach_type == BPF_TCX_INGRESS ? "ingress" : "egress");
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_link_fill_info(link: *mut bpf_link, info: *mut bpf_link_info) -> c_int {
    let mut tcx = tcx_link(link);
pub static mut ifindex: u32 = 0;
    rtnl_lock();
    if (tcx.dev) {
    ifindex = tcx.dev.ifindex;
    }
    rtnl_unlock();
    info.tcx.ifindex = ifindex;
    info.tcx.attach_type = link.attach_type;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tcx_link_detach(link: *mut bpf_link) -> c_int {
    tcx_link_release(link);
    return 0;
    }
pub static mut bpf_link_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn tcx_link_init(tcx: *mut tcx_link, link_primer: *mut bpf_link_primer, attr: *mut union bpf_attr, dev: *mut net_device, prog: *mut bpf_prog) -> c_int {
    bpf_link_init(&tcx.link, BPF_LINK_TYPE_TCX, &tcx_link_lops, prog,
    attr.link_create.attach_type);
    tcx.dev = dev;
    return bpf_link_prime(&tcx.link, link_primer);
    }
#[no_mangle]
pub unsafe extern "C" fn tcx_link_attach(attr: *const union bpf_attr, prog: *mut bpf_prog) -> c_int {
    let mut net = current.nsproxy.net_ns;
pub static mut link_primer: usize = 0;
pub static mut dev: *mut c_void = core::ptr::null_mut();
pub static mut tcx: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    rtnl_lock();
    dev = __dev_get_by_index(net, attr.link_create.target_ifindex);
    if (!dev) {
    ret = -ENODEV;
// goto;
    }
    tcx = kzalloc_obj(*tcx, GFP_USER);
    if (!tcx) {
    ret = -ENOMEM;
// goto;
    }
    ret = tcx_link_init(tcx, &link_primer, attr, dev, prog);
    if (ret) {
    kfree(tcx);
// goto;
    }
    ret = tcx_link_prog_attach(&tcx.link, attr.link_create.flags,
    attr.link_create.tcx.relative_fd,
    attr.link_create.tcx.expected_revision);
    if (ret) {
    tcx.dev = core::ptr::null_mut();
    bpf_link_cleanup(&link_primer);
// goto;
    }
    ret = bpf_link_settle(&link_primer);
// label;
    rtnl_unlock();
    return ret;
    }