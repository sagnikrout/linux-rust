//! Automatically rewritten from C to Rust
//! Source: kernel/irq/irqdomain.c
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

pub static mut irq_domain_list: usize = 0;
pub static mut irq_domain_mutex: usize = 0;
pub static mut irq_default_domain: *mut c_void = core::ptr::null_mut();
// forward_decl: irq_domain_alloc_irqs_locked;
// forward_decl: irq_domain_check_hierarchy;
// forward_decl: irq_domain_free_one_irq;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqchip_fwid {
    pub fwnode: fwnode_handle,
    pub parent: *mut fwnode_handle,
    pub type: c_uint,
    pub name: *mut c_char,
    pub pa: *mut phys_addr_t,
}

// forward_decl: debugfs_add_domain_dir;
// forward_decl: debugfs_remove_domain_dir;

#[no_mangle]
pub unsafe extern "C" fn debugfs_add_domain_dir(d: *mut irq_domain) { }
#[no_mangle]
pub unsafe extern "C" fn debugfs_remove_domain_dir(d: *mut irq_domain) { }

    static const char *irqchip_fwnode_get_name(const struct fwnode_handle *fwnode)
    {
    let mut fwid = container_of!(fwnode, irqchip_fwid, fwnode);
    return fwid.name;
    }
#[no_mangle]
pub unsafe extern "C" fn irqchip_fwnode_get_parent(fwnode: *mut fwnode_handle) -> *mut c_void {
    let mut fwid = container_of!(fwnode, irqchip_fwid, fwnode);
    return fwid.parent;
    }
pub static mut fwnode_operations: usize = 0;
    EXPORT_SYMBOL_GPL(irqchip_fwnode_ops);
//
// __irq_domain_alloc_fwnode - Allocate a fwnode_handle suitable for
// identifying an irq domain
// @type:	Type of irqchip_fwnode. See linux/irqdomain.h
// @id:		Optional user provided id if name != NULL
// @name:	Optional user provided domain name
// @pa:		Optional user-provided physical address
// @parent:	Optional parent fwnode_handle
//
// Allocate a struct irqchip_fwid, and return a pointer to the embedded
// fwnode_handle (or NULL on failure).
//
// Note: The types IRQCHIP_FWNODE_NAMED and IRQCHIP_FWNODE_NAMED_ID are
// solely to transport name information to irqdomain creation code. The
// node is not stored. For other types the pointer is kept in the irq
// domain struct.
//
#[no_mangle]
pub unsafe extern "C" fn __irq_domain_alloc_fwnode(type: c_uint, id: c_int, name: *mut c_char, pa: *mut phys_addr_t, parent: *mut fwnode_handle) -> *mut c_void {
pub static mut fwid: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
    fwid = kzalloc_obj(*fwid);
    match (type) {
    IRQCHIP_FWNODE_NAMED => {
    n = kasprintf(GFP_KERNEL, "%s", name);
    // break;
    }
    IRQCHIP_FWNODE_NAMED_ID => {
    n = kasprintf(GFP_KERNEL, "%s-%d", name, id);
    // break;
    }
    _ => {
    n = kasprintf(GFP_KERNEL, "irqchip@%pa", pa);
    // break;
    }
    }
    if (!fwid || !n) {
    kfree(fwid);
    kfree(n);
    return core::ptr::null_mut();
    }
    fwid.type = type;
    fwid.name = n;
    fwid.pa = pa;
    fwid.parent = parent;
    fwnode_init(&fwid.fwnode, &irqchip_fwnode_ops);
    return &fwid.fwnode;
    }
    EXPORT_SYMBOL_GPL(__irq_domain_alloc_fwnode);
//
// irq_domain_free_fwnode - Free a non-OF-backed fwnode_handle
// @fwnode: fwnode_handle to free
//
// Free a fwnode_handle allocated with irq_domain_alloc_fwnode.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_fwnode(fwnode: *mut fwnode_handle) {
pub static mut fwid: *mut c_void = core::ptr::null_mut();
    if (!fwnode || WARN_ON!(!is_fwnode_irqchip(fwnode))) {
    return;
    }
    fwid = container_of!(fwnode, irqchip_fwid, fwnode);
    kfree(fwid.name);
    kfree(fwid);
    }
    EXPORT_SYMBOL_GPL(irq_domain_free_fwnode);
#[no_mangle]
unsafe extern "C" fn alloc_name(domain: *mut irq_domain, base: *mut c_char, bus_token: irq_domain_bus_token) -> c_int {
    if (bus_token == DOMAIN_BUS_ANY) {
    domain.name = kasprintf(GFP_KERNEL, "%s", base);
    }
    else {
    domain.name = kasprintf(GFP_KERNEL, "%s-%d", base, bus_token);
    }
    if (!domain.name) {
    return -ENOMEM;
    }
    domain.flags |= IRQ_DOMAIN_NAME_ALLOCATED;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_fwnode_name(domain: *mut irq_domain, fwnode: *mut fwnode_handle, bus_token: irq_domain_bus_token, suffix: *mut c_char) -> c_int {
    let mut sep = suffix ? "-" : "";
    let mut suf = suffix ? : "";
pub static mut name: *mut c_void = core::ptr::null_mut();
    if (bus_token == DOMAIN_BUS_ANY) {
    name = kasprintf(GFP_KERNEL, "%pfw%s%s", fwnode, sep, suf);
    }
    else {
    name = kasprintf(GFP_KERNEL, "%pfw%s%s-%d", fwnode, sep, suf, bus_token);
    }
    if (!name) {
    return -ENOMEM;
    }
//
// fwnode paths contain '/', which debugfs is legitimately unhappy
// about. Replace them with ':', which does the trick and is not as
// offensive as '\'...
//
    domain.name = strreplace(name, '/', ':');
    domain.flags |= IRQ_DOMAIN_NAME_ALLOCATED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alloc_unknown_name(domain: *mut irq_domain, bus_token: irq_domain_bus_token) -> c_int {
    static atomic_t unknown_domains;
pub static mut id: c_int = 0;
    if (bus_token == DOMAIN_BUS_ANY) {
    domain.name = kasprintf(GFP_KERNEL, "unknown-%d", id);
    }
    else {
    domain.name = kasprintf(GFP_KERNEL, "unknown-%d-%d", id, bus_token);
    }
    if (!domain.name) {
    return -ENOMEM;
    }
    domain.flags |= IRQ_DOMAIN_NAME_ALLOCATED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_set_name(domain: *mut irq_domain, info: *const irq_domain_info) -> c_int {
pub static mut bus_token: irq_domain_bus_token = 0;
    let mut fwnode = info.fwnode;
    if (is_fwnode_irqchip(fwnode)) {
    let mut fwid = container_of!(fwnode, irqchip_fwid, fwnode);
//
// The name_suffix is only intended to be used to avoid a name
// collision when multiple domains are created for a single
// device and the name is picked using a real device node.
// (Typical use-case is regmap-IRQ controllers for devices
// providing more than one physical IRQ.) There should be no
// need to use name_suffix with irqchip-fwnode.
//
    if (info.name_suffix) {
    return -EINVAL;
    }
    match (fwid.type) {
    IRQCHIP_FWNODE_NAMED => {
    }
    IRQCHIP_FWNODE_NAMED_ID => {
    return alloc_name(domain, fwid.name, bus_token);
    }
    _ => {
    domain.name = fwid.name;
    if (bus_token != DOMAIN_BUS_ANY) {
    return alloc_name(domain, fwid.name, bus_token);
    }
    }
    }
    } else if (is_of_node(fwnode) || is_acpi_device_node(fwnode) || is_software_node(fwnode)) {
    return alloc_fwnode_name(domain, fwnode, bus_token, info.name_suffix);
    }
    if (domain.name) {
    return 0;
    }
    if (fwnode) {
    pr_err!("Invalid fwnode type for irqdomain\n");
    }
    return alloc_unknown_name(domain, bus_token);
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_domain_create(info: *mut irq_domain_info) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (WARN_ON!((info.size && info.direct_max) ||
    (!IS_ENABLED!(CONFIG_IRQ_DOMAIN_NOMAP) && info.direct_max) ||
    (info.direct_max && info.direct_max != info.hwirq_max))) {
    return ERR_PTR(-EINVAL);
    }
    domain = kzalloc_node(struct_size(domain, revmap, info.size),
    GFP_KERNEL, of_node_to_nid(to_of_node(info.fwnode)));
    if (!domain) {
    return ERR_PTR(-ENOMEM);
    }
    err = irq_domain_set_name(domain, info);
    if (err) {
    kfree(domain);
    return ERR_PTR(err);
    }
    domain.fwnode = fwnode_handle_get(info.fwnode);
    fwnode_dev_initialized(domain.fwnode, true);
// Fill structure
    INIT_RADIX_TREE(&domain.revmap_tree, GFP_KERNEL);
    domain.ops = info.ops;
    domain.host_data = info.host_data;
    domain.bus_token = info.bus_token;
    domain.hwirq_max = info.hwirq_max;
    if (info.direct_max) {
    domain.flags |= IRQ_DOMAIN_FLAG_NO_MAP;
    }
    domain.revmap_size = info.size;
//
// Hierarchical domains use the domain lock of the root domain
// (innermost domain).
//
// For non-hierarchical domains (as for root domains), the root
// pointer is set to the domain itself so that &domain->root->mutex
// always points to the right lock.
//
    mutex_init(&domain.mutex);
    domain.root = domain;
    irq_domain_check_hierarchy(domain);
    return domain;
    }
#[no_mangle]
unsafe extern "C" fn __irq_domain_publish(domain: *mut irq_domain) {
    mutex_lock(&irq_domain_mutex);
    debugfs_add_domain_dir(domain);
    list_add(&domain.link, &irq_domain_list);
    mutex_unlock(&irq_domain_mutex);
    pr_debug!("Added domain %s\n", domain.name);
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_free(domain: *mut irq_domain) {
    fwnode_dev_initialized(domain.fwnode, false);
    fwnode_handle_put(domain.fwnode);
    if (domain.flags & IRQ_DOMAIN_NAME_ALLOCATED) {
    kfree(domain.name);
    }
    kfree(domain);
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_instantiate_descs(info: *const irq_domain_info) {
    if (!IS_ENABLED!(CONFIG_SPARSE_IRQ)) {
    return;
    }
    if (irq_alloc_descs(info.virq_base, info.virq_base, info.size,
    of_node_to_nid(to_of_node(info.fwnode))) < 0) {
    pr_info!("Cannot allocate irq_descs @ IRQ%d, assuming pre-allocated\n",
    info.virq_base);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_domain_instantiate(info: *mut irq_domain_info, cond_alloc_descs: bool, force_associate: bool) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    domain = __irq_domain_create(info);
    if (IS_ERR(domain)) {
    return domain;
    }
    domain.flags |= info.domain_flags;
    domain.exit = info.exit;
    domain.dev = info.dev;

    if (info.parent) {
    domain.root = info.parent.root;
    domain.parent = info.parent;
    }

    if (info.dgc_info) {
    err = irq_domain_alloc_generic_chips(domain, info.dgc_info);
    if (err) {
// goto;
    }
    }
    if (info.init) {
    err = info.init(domain);
    if (err) {
// goto;
    }
    }
    __irq_domain_publish(domain);
    if (cond_alloc_descs && info.virq_base > 0) {
    irq_domain_instantiate_descs(info);
    }
//
// Legacy interrupt domains have a fixed Linux interrupt number
// associated. Other interrupt domains can request association by
// providing a Linux interrupt number > 0.
//
    if (force_associate || info.virq_base > 0) {
    irq_domain_associate_many(domain, info.virq_base, info.hwirq_base,
    info.size - info.hwirq_base);
    }
    return domain;
// label;
    if (info.dgc_info) {
    irq_domain_remove_generic_chips(domain);
    }
// label;
    irq_domain_free(domain);
    return ERR_PTR(err);
    }
//
// irq_domain_instantiate() - Instantiate a new irq domain data structure
// @info: Domain information pointer pointing to the information for this domain
//
// Return: A pointer to the instantiated irq domain or an ERR_PTR value.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_instantiate(info: *mut irq_domain_info) -> *mut c_void {
    return __irq_domain_instantiate(info, false, false);
    }
    EXPORT_SYMBOL_GPL(irq_domain_instantiate);
//
// irq_domain_remove() - Remove an irq domain.
// @domain: domain to remove
//
// This routine is used to remove an irq domain. The caller must ensure
// that all mappings within the domain have been disposed of prior to
// use, depending on the revmap type.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_remove(domain: *mut irq_domain) {
    if (domain.exit) {
    domain.exit(domain);
    }
    mutex_lock(&irq_domain_mutex);
    debugfs_remove_domain_dir(domain);
    WARN_ON!(!radix_tree_empty(&domain.revmap_tree));
    list_del(&domain.link);
//
// If the going away domain is the default one, reset it.
//
    if (unlikely(irq_default_domain == domain)) {
    irq_set_default_domain(core::ptr::null_mut());
    }
    mutex_unlock(&irq_domain_mutex);
    if (domain.flags & IRQ_DOMAIN_FLAG_DESTROY_GC) {
    irq_domain_remove_generic_chips(domain);
    }
    pr_debug!("Removed domain %s\n", domain.name);
    irq_domain_free(domain);
    }
    EXPORT_SYMBOL_GPL(irq_domain_remove);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_update_bus_token(domain: *mut irq_domain, bus_token: irq_domain_bus_token) {
pub static mut name: *mut c_void = core::ptr::null_mut();
    if (domain.bus_token == bus_token) {
    return;
    }
    mutex_lock(&irq_domain_mutex);
    domain.bus_token = bus_token;
    name = kasprintf(GFP_KERNEL, "%s-%d", domain.name, bus_token);
    if (!name) {
    mutex_unlock(&irq_domain_mutex);
    return;
    }
    debugfs_remove_domain_dir(domain);
    if (domain.flags & IRQ_DOMAIN_NAME_ALLOCATED) {
    kfree(domain.name);
    }
    else {
    domain.flags |= IRQ_DOMAIN_NAME_ALLOCATED;
    }
    domain.name = name;
    debugfs_add_domain_dir(domain);
    mutex_unlock(&irq_domain_mutex);
    }
    EXPORT_SYMBOL_GPL(irq_domain_update_bus_token);
//
// irq_domain_create_simple() - Register an irq_domain and optionally map a range of irqs
// @fwnode: firmware node for the interrupt controller
// @size: total number of irqs in mapping
// @first_irq: first number of irq block assigned to the domain,
// pass zero to assign irqs on-the-fly. If first_irq is non-zero, then
// pre-map all of the irqs in the domain to virqs starting at first_irq.
// @ops: domain callbacks
// @host_data: Controller private data pointer
//
// Allocates an irq_domain, and optionally if first_irq is positive then also
// allocate irq_descs and map all of the hwirqs to virqs starting at first_irq.
//
// This is intended to implement the expected behaviour for most
// interrupt controllers. If device tree is used, then first_irq will be 0 and
// irqs get mapped dynamically on the fly. However, if the controller requires
// static virq assignments (non-DT boot) then it will set that up correctly.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_create_simple(fwnode: *mut fwnode_handle, size: c_uint, first_irq: c_uint, ops: *mut irq_domain_ops, host_data: *mut c_void) -> *mut c_void {
pub static mut irq_domain_info: usize = 0;
    let mut domain = __irq_domain_instantiate(&info, true, false);
    return IS_ERR(domain) ? core::ptr::null_mut() : domain;
    }
    EXPORT_SYMBOL_GPL(irq_domain_create_simple);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_create_legacy(fwnode: *mut fwnode_handle, size: c_uint, first_irq: c_uint, first_hwirq: irq_hw_number_t, ops: *mut irq_domain_ops, host_data: *mut c_void) -> *mut c_void {
pub static mut irq_domain_info: usize = 0;
    let mut domain = __irq_domain_instantiate(&info, false, true);
    return IS_ERR(domain) ? core::ptr::null_mut() : domain;
    }
    EXPORT_SYMBOL_GPL(irq_domain_create_legacy);
//
// irq_find_matching_fwspec() - Locates a domain for a given fwspec
// @fwspec: FW specifier for an interrupt
// @bus_token: domain-specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_find_matching_fwspec(fwspec: *mut irq_fwspec, bus_token: irq_domain_bus_token) -> *mut c_void {
    struct irq_domain *h, *found = core::ptr::null_mut();
    let mut fwnode = fwspec.fwnode;
    let mut rc = 0;
//
// We might want to match the legacy controller last since
// it might potentially be set to match all interrupts in
// the absence of a device node. This isn't a problem so far
// yet though...
//
// bus_token == DOMAIN_BUS_ANY matches any domain, any other
// values must generate an exact match for the domain to be
// selected.
//
    mutex_lock(&irq_domain_mutex);
    list_for_each_entry(h, &irq_domain_list, link) {
    if (h.ops.select && bus_token != DOMAIN_BUS_ANY) {
    rc = h.ops.select(h, fwspec, bus_token);
    }

    else if (h.ops.match) {
    rc = h.ops.match(h, to_of_node(fwnode), bus_token);
    }
    else {
    rc = ((fwnode != core::ptr::null_mut()) && (h.fwnode == fwnode) &&
    ((bus_token == DOMAIN_BUS_ANY) ||
    (h.bus_token == bus_token)));
    }
    if (rc) {
    found = h;
    break;
    }
    }
    mutex_unlock(&irq_domain_mutex);
    return found;
    }
    EXPORT_SYMBOL_GPL(irq_find_matching_fwspec);
//
// irq_set_default_domain() - Set a "default" irq domain
// @domain: default domain pointer
//
// For convenience, it's possible to set a "default" domain that will be used
// whenever NULL is passed to irq_create_mapping(). It makes life easier for
// platforms that want to manipulate a few hard coded interrupt numbers that
// aren't properly represented in the device-tree.
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_default_domain(domain: *mut irq_domain) {
    pr_debug!("Default domain set to @0x%p\n", domain);
    irq_default_domain = domain;
    }
    EXPORT_SYMBOL_GPL(irq_set_default_domain);
//
// irq_get_default_domain() - Retrieve the "default" irq domain
//
// Returns: the default domain, if any.
//
// Modern code should never use this. This should only be used on
// systems that cannot implement a firmware->fwnode mapping (which
// both DT and ACPI provide).
//
#[no_mangle]
pub unsafe extern "C" fn irq_get_default_domain() -> *mut c_void {
    return irq_default_domain;
    }
    EXPORT_SYMBOL_GPL(irq_get_default_domain);
#[no_mangle]
unsafe extern "C" fn irq_domain_is_nomap(domain: *mut irq_domain) -> bool {
    return IS_ENABLED!(CONFIG_IRQ_DOMAIN_NOMAP) &&
    (domain.flags & IRQ_DOMAIN_FLAG_NO_MAP);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_clear_mapping(domain: *mut irq_domain, hwirq: irq_hw_number_t) {
    lockdep_assert_held(&domain.root.mutex);
    if (irq_domain_is_nomap(domain)) {
    return;
    }
    if (hwirq < domain.revmap_size) {
    rcu_assign_pointer(domain.revmap[hwirq], core::ptr::null_mut());
    }
    else {
    radix_tree_delete(&domain.revmap_tree, hwirq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_set_mapping(domain: *mut irq_domain, hwirq: irq_hw_number_t, irq_data: *mut irq_data) {
//
// This also makes sure that all domains point to the same root when
// called from irq_domain_insert_irq() for each domain in a hierarchy.
//
    lockdep_assert_held(&domain.root.mutex);
    if (irq_domain_is_nomap(domain)) {
    return;
    }
    if (hwirq < domain.revmap_size) {
    rcu_assign_pointer(domain.revmap[hwirq], irq_data);
    }
    else {
    radix_tree_insert(&domain.revmap_tree, hwirq, irq_data);
    }
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_disassociate(domain: *mut irq_domain, irq: c_uint) {
    let mut irq_data = irq_get_irq_data(irq);
    let mut hwirq;
    if (WARN(!irq_data || irq_data.domain != domain,
    "virq%i doesn't exist; cannot disassociate\n", irq)) {
    return;
    }
    hwirq = irq_data.hwirq;
    mutex_lock(&domain.root.mutex);
    irq_set_status_flags(irq, IRQ_NOREQUEST);
// remove chip and handler
    irq_set_chip_and_handler(irq, core::ptr::null_mut(), core::ptr::null_mut());
// Make sure it's completed
    synchronize_irq(irq);
// Tell the PIC about it
    if (domain.ops.unmap) {
    domain.ops.unmap(domain, irq);
    }
    smp_mb();
    irq_data.domain = core::ptr::null_mut();
    irq_data.hwirq = 0;
    domain.mapcount -= 1;
// Clear reverse map for this hwirq
    irq_domain_clear_mapping(domain, hwirq);
    mutex_unlock(&domain.root.mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_associate_locked(domain: *mut irq_domain, virq: c_uint, hwirq: irq_hw_number_t) -> c_int {
    let mut irq_data = irq_get_irq_data(virq);
    let mut ret = 0;
    if (WARN(hwirq >= domain.hwirq_max,
    "error: hwirq 0x%x is too large for %s\n", (int)hwirq, domain.name)) {
    return -EINVAL;
    }
    if (WARN(!irq_data, "error: virq%i is not allocated", virq)) {
    return -EINVAL;
    }
    if (WARN(irq_data.domain, "error: virq%i is already associated", virq)) {
    return -EINVAL;
    }
    irq_data.hwirq = hwirq;
    irq_data.domain = domain;
    if (domain.ops.map) {
    ret = domain.ops.map(domain, virq, hwirq);
    if (ret != 0) {
//
// If map() returns -EPERM, this interrupt is protected
// by the firmware or some other service and shall not
// be mapped. Don't bother telling the user about it.
//
    if (ret != -EPERM) {
    pr_info!("%s didn't like hwirq-0x%lx to VIRQ%i mapping (rc=%d)\n",
    domain.name, hwirq, virq, ret);
    }
    irq_data.domain = core::ptr::null_mut();
    irq_data.hwirq = 0;
    return ret;
    }
    }
    domain.mapcount += 1;
    irq_domain_set_mapping(domain, hwirq, irq_data);
    irq_clear_status_flags(virq, IRQ_NOREQUEST);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_associate(domain: *mut irq_domain, virq: c_uint, hwirq: irq_hw_number_t) -> c_int {
    let mut ret = 0;
    mutex_lock(&domain.root.mutex);
    ret = irq_domain_associate_locked(domain, virq, hwirq);
    mutex_unlock(&domain.root.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(irq_domain_associate);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_associate_many(domain: *mut irq_domain, irq_base: c_uint, hwirq_base: irq_hw_number_t, count: c_int) {
pub static mut of_node: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    of_node = irq_domain_get_of_node(domain);
    pr_debug!("%s(%s, irqbase=%i, hwbase=%i, count=%i)\n", __func__,
    of_node_full_name(of_node), irq_base, (int)hwirq_base, count);
    for (i = 0; i < count; i++) {
    irq_domain_associate(domain, irq_base + i, hwirq_base + i);
    }
    }
    EXPORT_SYMBOL_GPL(irq_domain_associate_many);

//
// irq_create_direct_mapping() - Allocate an irq for direct mapping
// @domain: domain to allocate the irq for or NULL for default domain
//
// This routine is used for irq controllers which can choose the hardware
// interrupt numbers they generate. In such a case it's simplest to use
// the linux irq as the hardware interrupt number. It still uses the linear
// or radix tree to store the mapping, but the irq controller can optimize
// the revmap path by using the hwirq directly.
//
#[no_mangle]
pub unsafe extern "C" fn irq_create_direct_mapping(domain: *mut irq_domain) -> c_uint {
pub static mut of_node: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    if (domain == core::ptr::null_mut()) {
    domain = irq_default_domain;
    }
    of_node = irq_domain_get_of_node(domain);
    virq = irq_alloc_desc_from(1, of_node_to_nid(of_node));
    if (!virq) {
    pr_debug!("create_direct virq allocation failed\n");
    return 0;
    }
    if (virq >= domain.hwirq_max) {
    pr_err!("ERROR: no free irqs available below %lu maximum\n",
    domain.hwirq_max);
    irq_free_desc(virq);
    return 0;
    }
    pr_debug!("create_direct obtained virq %d\n", virq);
    if (irq_domain_associate(domain, virq, virq)) {
    irq_free_desc(virq);
    return 0;
    }
    return virq;
    }
    EXPORT_SYMBOL_GPL(irq_create_direct_mapping);

#[no_mangle]
pub unsafe extern "C" fn irq_create_mapping_affinity_locked(domain: *mut irq_domain, hwirq: irq_hw_number_t, affinity: *mut irq_affinity_desc) -> c_uint {
    let mut of_node = irq_domain_get_of_node(domain);
    let mut virq = 0;
    pr_debug!("irq_create_mapping(0x%p, 0x%lx)\n", domain, hwirq);
// Allocate a virtual interrupt number
    virq = irq_domain_alloc_descs(-1, 1, hwirq, of_node_to_nid(of_node),
    affinity);
    if (virq <= 0) {
    pr_debug!(". virq allocation failed\n");
    return 0;
    }
    if (irq_domain_associate_locked(domain, virq, hwirq)) {
    irq_free_desc(virq);
    return 0;
    }
    pr_debug!("irq %lu on domain %s mapped to virtual irq %u\n",
    hwirq, of_node_full_name(of_node), virq);
    return virq;
    }
//
// irq_create_mapping_affinity() - Map a hardware interrupt into linux irq space
// @domain: domain owning this hardware interrupt or NULL for default domain
// @hwirq: hardware irq number in that domain space
// @affinity: irq affinity
//
// Only one mapping per hardware interrupt is permitted. Returns a linux
// irq number.
// If the sense/trigger is to be specified, set_irq_type() should be called
// on the number returned from that call.
//
#[no_mangle]
pub unsafe extern "C" fn irq_create_mapping_affinity(domain: *mut irq_domain, hwirq: irq_hw_number_t, affinity: *mut irq_affinity_desc) -> c_uint {
    let mut virq = 0;
// Look for default domain if necessary
    if (domain == core::ptr::null_mut()) {
    domain = irq_default_domain;
    }
    if (domain == core::ptr::null_mut()) {
    WARN(1, "%s(, %lx) called with core::ptr::null_mut() domain\n", __func__, hwirq);
    return 0;
    }
    mutex_lock(&domain.root.mutex);
// Check if mapping already exists
    virq = irq_find_mapping(domain, hwirq);
    if (virq) {
    pr_debug!("existing mapping on virq %d\n", virq);
// goto;
    }
    virq = irq_create_mapping_affinity_locked(domain, hwirq, affinity);
// label;
    mutex_unlock(&domain.root.mutex);
    return virq;
    }
    EXPORT_SYMBOL_GPL(irq_create_mapping_affinity);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_translate(d: *mut irq_domain, fwspec: *mut irq_fwspec, hwirq: *mut irq_hw_number_t, type: *mut c_uint) -> c_int {

    if (d.ops.translate) {
    return d.ops.translate(d, fwspec, hwirq, type);
    }

    if (d.ops.xlate) {
    return d.ops.xlate(d, to_of_node(fwspec.fwnode),
    fwspec.param, fwspec.param_count,
    hwirq, type);
    }
// If domain has no translation, then we assume interrupt line
// hwirq = fwspec->param[0];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn of_phandle_args_to_fwspec(np: *mut device_node, args: *mut u32, count: c_uint, fwspec: *mut irq_fwspec) {
    let mut i = 0;
    fwspec.fwnode = of_fwnode_handle(np);
    fwspec.param_count = count;
    for (i = 0; i < count; i++) {
    fwspec.param[i] = args[i];
    }
    }
    EXPORT_SYMBOL_GPL(of_phandle_args_to_fwspec);
#[no_mangle]
pub unsafe extern "C" fn fwspec_to_domain(fwspec: *mut irq_fwspec) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
    if (fwspec.fwnode) {
    domain = irq_find_matching_fwspec(fwspec, DOMAIN_BUS_WIRED);
    if (!domain) {
    domain = irq_find_matching_fwspec(fwspec, DOMAIN_BUS_ANY);
    }
    } else {
    domain = irq_default_domain;
    }
    return domain;
    }

#[no_mangle]
pub unsafe extern "C" fn irq_populate_fwspec_info(fwspec: *mut irq_fwspec, info: *mut irq_fwspec_info) -> c_int {
    let mut domain = fwspec_to_domain(fwspec);
    memset(info, 0, sizeof!(*info));
    if (!domain || !domain.ops.get_fwspec_info) {
    return 0;
    }
    return domain.ops.get_fwspec_info(fwspec, info);
    }

#[no_mangle]
pub unsafe extern "C" fn irq_create_fwspec_mapping(fwspec: *mut irq_fwspec) -> c_uint {
pub static mut type: c_uint = 0;
pub static mut domain: *mut c_void = core::ptr::null_mut();
pub static mut irq_data: *mut c_void = core::ptr::null_mut();
    let mut hwirq;
    let mut virq = 0;
    domain = fwspec_to_domain(fwspec);
    if (!domain) {
    pr_warn!("no irq domain found for %s !\n",
    of_node_full_name(to_of_node(fwspec.fwnode)));
    return 0;
    }
    if (irq_domain_translate(domain, fwspec, &hwirq, &type)) {
    return 0;
    }
//
// WARN if the irqchip returns a type with bits
// outside the sense mask set and clear these bits.
//
    if (WARN_ON!(type & ~IRQ_TYPE_SENSE_MASK)) {
    type &= IRQ_TYPE_SENSE_MASK;
    }
    mutex_lock(&domain.root.mutex);
//
// If we've already configured this interrupt,
// don't do it again, or hell will break loose.
//
    virq = irq_find_mapping(domain, hwirq);
    if (virq) {
//
// If the trigger type is not specified or matches the
// current trigger type then we are done so return the
// interrupt number.
//
    if (type == IRQ_TYPE_NONE || type == irq_get_trigger_type(virq)) {
// goto;
    }
//
// If the trigger type has not been set yet, then set
// it now and return the interrupt number.
//
    if (irq_get_trigger_type(virq) == IRQ_TYPE_NONE) {
    irq_data = irq_get_irq_data(virq);
    if (!irq_data) {
    virq = 0;
// goto;
    }
    irqd_set_trigger_type(irq_data, type);
// goto;
    }
    pr_warn!("type mismatch, failed to map hwirq-%lu for %s!\n",
    hwirq, of_node_full_name(to_of_node(fwspec.fwnode)));
    virq = 0;
// goto;
    }
    if (irq_domain_is_hierarchy(domain)) {
    if (irq_domain_is_msi_device(domain)) {
    mutex_unlock(&domain.root.mutex);
    virq = msi_device_domain_alloc_wired(domain, hwirq, type);
    mutex_lock(&domain.root.mutex);
    } else {
    virq = irq_domain_alloc_irqs_locked(domain, -1, 1, NUMA_NO_NODE,
    fwspec, false, core::ptr::null_mut());
    }
    if (virq <= 0) {
    virq = 0;
// goto;
    }
    } else {
// Create mapping
    virq = irq_create_mapping_affinity_locked(domain, hwirq, core::ptr::null_mut());
    if (!virq) {
// goto;
    }
    }
    irq_data = irq_get_irq_data(virq);
    if (WARN_ON!(!irq_data)) {
    virq = 0;
// goto;
    }
// Store trigger type
    irqd_set_trigger_type(irq_data, type);
// label;
    mutex_unlock(&domain.root.mutex);
    return virq;
    }
    EXPORT_SYMBOL_GPL(irq_create_fwspec_mapping);
#[no_mangle]
pub unsafe extern "C" fn irq_create_of_mapping(irq_data: *mut of_phandle_args) -> c_uint {
pub static mut fwspec: usize = 0;
    of_phandle_args_to_fwspec(irq_data.np, irq_data.args,
    irq_data.args_count, &fwspec);
    return irq_create_fwspec_mapping(&fwspec);
    }
    EXPORT_SYMBOL_GPL(irq_create_of_mapping);
//
// irq_dispose_mapping() - Unmap an interrupt
// @virq: linux irq number of the interrupt to unmap
//
#[no_mangle]
pub unsafe extern "C" fn irq_dispose_mapping(virq: c_uint) {
pub static mut irq_data: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    irq_data = virq ? irq_get_irq_data(virq) : core::ptr::null_mut();
    if (!irq_data) {
    return;
    }
    domain = irq_data.domain;
    if (WARN_ON!(domain == core::ptr::null_mut())) {
    return;
    }
    if (irq_domain_is_hierarchy(domain)) {
    irq_domain_free_one_irq(domain, virq);
    } else {
    irq_domain_disassociate(domain, virq);
    irq_free_desc(virq);
    }
    }
    EXPORT_SYMBOL_GPL(irq_dispose_mapping);
//
// __irq_resolve_mapping() - Find a linux irq from a hw irq number.
// @domain: domain owning this hardware interrupt
// @hwirq: hardware irq number in that domain space
// @irq: optional pointer to return the Linux irq if required
//
// Returns the interrupt descriptor.
//
#[no_mangle]
pub unsafe extern "C" fn __irq_resolve_mapping(domain: *mut irq_domain, hwirq: irq_hw_number_t, irq: *mut c_uint) -> *mut c_void {
    let mut desc = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
// Look for default domain if necessary
    if (domain == core::ptr::null_mut()) {
    domain = irq_default_domain;
    }
    if (domain == core::ptr::null_mut()) {
    return desc;
    }
    if (irq_domain_is_nomap(domain)) {
    if (hwirq < domain.hwirq_max) {
    data = irq_domain_get_irq_data(domain, hwirq);
    if (data && data.hwirq == hwirq) {
    desc = irq_data_to_desc(data);
    }
    if (irq && desc) {
// irq = hwirq;
    }
    }
    return desc;
    }
    rcu_read_lock();
// Check if the hwirq is in the linear revmap.
    if (hwirq < domain.revmap_size) {
    data = rcu_dereference(domain.revmap[hwirq]);
    }
    else {
    data = radix_tree_lookup(&domain.revmap_tree, hwirq);
    }
    if (likely(data)) {
    desc = irq_data_to_desc(data);
    if (irq) {
// irq = data->irq;
    }
    }
    rcu_read_unlock();
    return desc;
    }
    EXPORT_SYMBOL_GPL(__irq_resolve_mapping);
//
// irq_domain_xlate_onecell() - Generic xlate for direct one cell bindings
// @d:		Interrupt domain involved in the translation
// @ctrlr:	The device tree node for the device whose interrupt is translated
// @intspec:	The interrupt specifier data from the device tree
// @intsize:	The number of entries in @intspec
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
// Device Tree IRQ specifier translation function which works with one cell
// bindings where the cell value maps directly to the hwirq number.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_xlate_onecell(d: *mut irq_domain, ctrlr: *mut device_node, intspec: *mut u32, intsize: c_uint, out_hwirq: *mut c_ulong, out_type: *mut c_uint) -> c_int {
    if (WARN_ON!(intsize < 1)) {
    return -EINVAL;
    }
// out_hwirq = intspec[0];
// out_type = IRQ_TYPE_NONE;
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_xlate_onecell);
//
// irq_domain_xlate_twocell() - Generic xlate for direct two cell bindings
// @d:		Interrupt domain involved in the translation
// @ctrlr:	The device tree node for the device whose interrupt is translated
// @intspec:	The interrupt specifier data from the device tree
// @intsize:	The number of entries in @intspec
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
// Device Tree IRQ specifier translation function which works with two cell
// bindings where the cell values map directly to the hwirq number
// and linux irq flags.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_xlate_twocell(d: *mut irq_domain, ctrlr: *mut device_node, intspec: *mut u32, intsize: c_uint, out_hwirq: *mut irq_hw_number_t, out_type: *mut c_uint) -> c_int {
pub static mut fwspec: usize = 0;
    of_phandle_args_to_fwspec(ctrlr, intspec, intsize, &fwspec);
    return irq_domain_translate_twocell(d, &fwspec, out_hwirq, out_type);
    }
    EXPORT_SYMBOL_GPL(irq_domain_xlate_twocell);
//
// irq_domain_xlate_twothreecell() - Generic xlate for direct two or three cell bindings
// @d:		Interrupt domain involved in the translation
// @ctrlr:	The device tree node for the device whose interrupt is translated
// @intspec:	The interrupt specifier data from the device tree
// @intsize:	The number of entries in @intspec
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
// Device Tree interrupt specifier translation function for two or three
// cell bindings, where the cell values map directly to the hardware
// interrupt number and the type specifier.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_xlate_twothreecell(d: *mut irq_domain, ctrlr: *mut device_node, intspec: *mut u32, intsize: c_uint, out_hwirq: *mut irq_hw_number_t, out_type: *mut c_uint) -> c_int {
pub static mut fwspec: usize = 0;
    of_phandle_args_to_fwspec(ctrlr, intspec, intsize, &fwspec);
    return irq_domain_translate_twothreecell(d, &fwspec, out_hwirq, out_type);
    }
    EXPORT_SYMBOL_GPL(irq_domain_xlate_twothreecell);
//
// irq_domain_xlate_onetwocell() - Generic xlate for one or two cell bindings
// @d:		Interrupt domain involved in the translation
// @ctrlr:	The device tree node for the device whose interrupt is translated
// @intspec:	The interrupt specifier data from the device tree
// @intsize:	The number of entries in @intspec
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
// Device Tree IRQ specifier translation function which works with either one
// or two cell bindings where the cell values map directly to the hwirq number
// and linux irq flags.
//
// Note: don't use this function unless your interrupt controller explicitly
// supports both one and two cell bindings.  For the majority of controllers
// the _onecell() or _twocell() variants above should be used.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_xlate_onetwocell(d: *mut irq_domain, ctrlr: *mut device_node, intspec: *mut u32, intsize: c_uint, out_hwirq: *mut c_ulong, out_type: *mut c_uint) -> c_int {
    if (WARN_ON!(intsize < 1)) {
    return -EINVAL;
    }
// out_hwirq = intspec[0];
    if (intsize > 1) {
// out_type = intspec[1] & IRQ_TYPE_SENSE_MASK;
    }
    else {
// out_type = IRQ_TYPE_NONE;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_xlate_onetwocell);
pub static mut irq_domain_ops: usize = 0;
    EXPORT_SYMBOL_GPL(irq_domain_simple_ops);
//
// irq_domain_translate_onecell() - Generic translate for direct one cell
// bindings
// @d:		Interrupt domain involved in the translation
// @fwspec:	The firmware interrupt specifier to translate
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_translate_onecell(d: *mut irq_domain, fwspec: *mut irq_fwspec, out_hwirq: *mut c_ulong, out_type: *mut c_uint) -> c_int {
    if (WARN_ON!(fwspec.param_count < 1)) {
    return -EINVAL;
    }
// out_hwirq = fwspec->param[0];
// out_type = IRQ_TYPE_NONE;
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_translate_onecell);
//
// irq_domain_translate_twocell() - Generic translate for direct two cell
// bindings
// @d:		Interrupt domain involved in the translation
// @fwspec:	The firmware interrupt specifier to translate
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
// Device Tree IRQ specifier translation function which works with two cell
// bindings where the cell values map directly to the hwirq number
// and linux irq flags.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_translate_twocell(d: *mut irq_domain, fwspec: *mut irq_fwspec, out_hwirq: *mut c_ulong, out_type: *mut c_uint) -> c_int {
    if (WARN_ON!(fwspec.param_count < 2)) {
    return -EINVAL;
    }
// out_hwirq = fwspec->param[0];
// out_type = fwspec->param[1] & IRQ_TYPE_SENSE_MASK;
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_translate_twocell);
//
// irq_domain_translate_twothreecell() - Generic translate for direct two or three cell
// bindings
// @d:		Interrupt domain involved in the translation
// @fwspec:	The firmware interrupt specifier to translate
// @out_hwirq:	Pointer to storage for the hardware interrupt number
// @out_type:	Pointer to storage for the interrupt type
//
// Firmware interrupt specifier translation function for two or three cell
// specifications, where the parameter values map directly to the hardware
// interrupt number and the type specifier.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_translate_twothreecell(d: *mut irq_domain, fwspec: *mut irq_fwspec, out_hwirq: *mut c_ulong, out_type: *mut c_uint) -> c_int {
    if (fwspec.param_count == 2) {
// out_hwirq = fwspec->param[0];
// out_type = fwspec->param[1] & IRQ_TYPE_SENSE_MASK;
    return 0;
    }
    if (fwspec.param_count == 3) {
// out_hwirq = fwspec->param[1];
// out_type = fwspec->param[2] & IRQ_TYPE_SENSE_MASK;
    return 0;
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(irq_domain_translate_twothreecell);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_alloc_descs(virq: c_int, cnt: c_uint, hwirq: irq_hw_number_t, node: c_int, affinity: *mut irq_affinity_desc) -> c_int {
    let mut hint = 0;
    if (virq >= 0) {
    virq = __irq_alloc_descs(virq, virq, cnt, node, THIS_MODULE,
    affinity);
    } else {
    hint = hwirq % irq_get_nr_irqs();
    if (hint == 0) {
    hint += 1;
    }
    virq = __irq_alloc_descs(-1, hint, cnt, node, THIS_MODULE,
    affinity);
    if (virq <= 0 && hint > 1) {
    virq = __irq_alloc_descs(-1, 1, cnt, node, THIS_MODULE,
    affinity);
    }
    }
    return virq;
    }
//
// irq_domain_reset_irq_data - Clear hwirq, chip and chip_data in @irq_data
// @irq_data:	The pointer to irq_data
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_reset_irq_data(irq_data: *mut irq_data) {
    irq_data.hwirq = 0;
    irq_data.chip = &no_irq_chip;
    irq_data.chip_data = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(irq_domain_reset_irq_data);

#[no_mangle]
unsafe extern "C" fn irq_domain_insert_irq(virq: c_int) {
pub static mut data: *mut c_void = core::ptr::null_mut();
    while (data) {
    let mut domain = data.domain;
    domain.mapcount += 1;
    irq_domain_set_mapping(domain, data.hwirq, data);
    }
    irq_clear_status_flags(virq, IRQ_NOREQUEST);
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_remove_irq(virq: c_int) {
pub static mut data: *mut c_void = core::ptr::null_mut();
    irq_set_status_flags(virq, IRQ_NOREQUEST);
    irq_set_chip_and_handler(virq, core::ptr::null_mut(), core::ptr::null_mut());
    synchronize_irq(virq);
    smp_mb();
    while (data) {
    let mut domain = data.domain;
pub static mut hwirq: irq_hw_number_t = 0;
    domain.mapcount -= 1;
    irq_domain_clear_mapping(domain, hwirq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_insert_irq_data(domain: *mut irq_domain, child: *mut irq_data) -> *mut c_void {
pub static mut irq_data: *mut c_void = core::ptr::null_mut();
    irq_data = kzalloc_node(sizeof!(*irq_data), GFP_KERNEL,
    irq_data_get_node(child));
    if (irq_data) {
    child.parent_data = irq_data;
    irq_data.irq = child.irq;
    irq_data.common = child.common;
    irq_data.domain = domain;
    }
    return irq_data;
    }
#[no_mangle]
unsafe extern "C" fn __irq_domain_free_hierarchy(irq_data: *mut irq_data) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    while (irq_data) {
    tmp = irq_data;
    irq_data = irq_data.parent_data;
    kfree(tmp);
    }
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_free_irq_data(virq: c_uint, nr_irqs: c_uint) {
    let mut irq_data = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut i = 0;
    while (i < nr_irqs) {
    irq_data = irq_get_irq_data(virq + i);
    tmp = irq_data.parent_data;
    irq_data.parent_data = core::ptr::null_mut();
    irq_data.domain = core::ptr::null_mut();
    __irq_domain_free_hierarchy(tmp);
    }
    }
//
// irq_domain_disconnect_hierarchy - Mark the first unused level of a hierarchy
// @domain:	IRQ domain from which the hierarchy is to be disconnected
// @virq:	IRQ number where the hierarchy is to be trimmed
//
// Marks the @virq level belonging to @domain as disconnected.
// Returns -EINVAL if @virq doesn't have a valid irq_data pointing
// to @domain.
//
// Its only use is to be able to trim levels of hierarchy that do not
// have any real meaning for this interrupt, and that the driver marks
// as such from its .alloc() callback.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_disconnect_hierarchy(domain: *mut irq_domain, virq: c_uint) -> c_int {
pub static mut irqd: *mut c_void = core::ptr::null_mut();
    irqd = irq_domain_get_irq_data(domain, virq);
    if (!irqd) {
    return -EINVAL;
    }
    irqd.chip = ERR_PTR(-ENOTCONN);
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_disconnect_hierarchy);
#[no_mangle]
unsafe extern "C" fn irq_domain_trim_hierarchy(virq: c_uint) -> c_int {
    let mut tail = core::ptr::null_mut();
    let mut irqd = core::ptr::null_mut();
    let mut irq_data = core::ptr::null_mut();
    irq_data = irq_get_irq_data(virq);
    tail = core::ptr::null_mut();
// The first entry must have a valid irqchip
    if (IS_ERR_OR_NULL(irq_data.chip)) {
    return -EINVAL;
    }
//
// Validate that the irq_data chain is sane in the presence of
// a hierarchy trimming marker.
//
    while (irqd) {
// Can't have a valid irqchip after a trim marker
    if (irqd.chip && tail) {
    return -EINVAL;
    }
// Can't have an empty irqchip before a trim marker
    if (!irqd.chip && !tail) {
    return -EINVAL;
    }
    if (IS_ERR(irqd.chip)) {
// Only -ENOTCONN is a valid trim marker
    if (PTR_ERR(irqd.chip) != -ENOTCONN) {
    return -EINVAL;
    }
    tail = irq_data;
    }
    }
// No trim marker, nothing to do
    if (!tail) {
    return 0;
    }
    pr_info!("IRQ%d: trimming hierarchy from %s\n",
    virq, tail.parent_data.domain.name);
// Sever the inner part of the hierarchy...
    irqd = tail;
    tail = tail.parent_data;
    irqd.parent_data = core::ptr::null_mut();
    __irq_domain_free_hierarchy(tail);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_alloc_irq_data(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint) -> c_int {
pub static mut irq_data: *mut c_void = core::ptr::null_mut();
pub static mut parent: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// The outermost irq_data is embedded in struct irq_desc
    while (i < nr_irqs) {
    irq_data = irq_get_irq_data(virq + i);
    irq_data.domain = domain;
    while (parent) {
    irq_data = irq_domain_insert_irq_data(parent, irq_data);
    if (!irq_data) {
    irq_domain_free_irq_data(virq, i + 1);
    return -ENOMEM;
    }
    }
    }
    return 0;
    }
//
// irq_domain_get_irq_data - Get irq_data associated with @virq and @domain
// @domain:	domain to match
// @virq:	IRQ number to get irq_data
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_get_irq_data(domain: *mut irq_domain, virq: c_uint) -> *mut c_void {
pub static mut irq_data: *mut c_void = core::ptr::null_mut();
    for (irq_data = irq_get_irq_data(virq); irq_data;
    irq_data = irq_data.parent_data) {
    if (irq_data.domain == domain)
    return irq_data;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(irq_domain_get_irq_data);
//
// irq_domain_set_hwirq_and_chip - Set hwirq and irqchip of @virq at @domain
// @domain:	Interrupt domain to match
// @virq:	IRQ number
// @hwirq:	The hwirq number
// @chip:	The associated interrupt chip
// @chip_data:	The associated chip data
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_set_hwirq_and_chip(domain: *mut irq_domain, virq: c_uint, hwirq: irq_hw_number_t, chip: *mut irq_chip, chip_data: *mut c_void) -> c_int {
    let mut irq_data = irq_domain_get_irq_data(domain, virq);
    if (!irq_data) {
    return -ENOENT;
    }
    irq_data.hwirq = hwirq;
    irq_data.chip = (chip ? chip : &no_irq_chip);
    irq_data.chip_data = chip_data;
    irq_proc_update_chip(chip);
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_set_hwirq_and_chip);
//
// irq_domain_set_info - Set the complete data for a @virq in @domain
// @domain:		Interrupt domain to match
// @virq:		IRQ number
// @hwirq:		The hardware interrupt number
// @chip:		The associated interrupt chip
// @chip_data:		The associated interrupt chip data
// @handler:		The interrupt flow handler
// @handler_data:	The interrupt flow handler data
// @handler_name:	The interrupt handler name
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_set_info(domain: *mut irq_domain, virq: c_uint, hwirq: irq_hw_number_t, chip: *mut irq_chip, chip_data: *mut c_void, handler: irq_flow_handler_t, handler_data: *mut c_void, handler_name: *mut c_char) {
    irq_domain_set_hwirq_and_chip(domain, virq, hwirq, chip, chip_data);
    __irq_set_handler(virq, handler, 0, handler_name);
    irq_set_handler_data(virq, handler_data);
    }
    EXPORT_SYMBOL(irq_domain_set_info);
//
// irq_domain_free_irqs_common - Clear irq_data and free the parent
// @domain:	Interrupt domain to match
// @virq:	IRQ number to start with
// @nr_irqs:	The number of irqs to free
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_irqs_common(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint) {
pub static mut irq_data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < nr_irqs) {
    irq_data = irq_domain_get_irq_data(domain, virq + i);
    if (irq_data) {
    irq_domain_reset_irq_data(irq_data);
    }
    }
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    }
    EXPORT_SYMBOL_GPL(irq_domain_free_irqs_common);
//
// irq_domain_free_irqs_top - Clear handler and handler data, clear irqdata and free parent
// @domain:	Interrupt domain to match
// @virq:	IRQ number to start with
// @nr_irqs:	The number of irqs to free
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_irqs_top(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint) {
    let mut i = 0;
    while (i < nr_irqs) {
    irq_set_handler_data(virq + i, core::ptr::null_mut());
    irq_set_handler(virq + i, core::ptr::null_mut());
    }
    irq_domain_free_irqs_common(domain, virq, nr_irqs);
    }
    EXPORT_SYMBOL_GPL(irq_domain_free_irqs_top);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_irqs_hierarchy(domain: *mut irq_domain, irq_base: c_uint, nr_irqs: c_uint) {
    let mut i = 0;
    if (!domain.ops.free) {
    return;
    }
    while (i < nr_irqs) {
    if (irq_domain_get_irq_data(domain, irq_base + i)) {
    domain.ops.free(domain, irq_base + i, 1);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_alloc_irqs_hierarchy(domain: *mut irq_domain, irq_base: c_uint, nr_irqs: c_uint, arg: *mut c_void) -> c_int {
    if (!domain.ops.alloc) {
    pr_debug!("domain.ops.alloc() is core::ptr::null_mut()\n");
    return -ENOSYS;
    }
    return domain.ops.alloc(domain, irq_base, nr_irqs, arg);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_alloc_irqs_locked(domain: *mut irq_domain, irq_base: c_int, nr_irqs: c_uint, node: c_int, arg: *mut c_void, realloc: bool, affinity: *mut irq_affinity_desc) -> c_int {
    let mut i = 0;
    let mut ret = 0;
    let mut virq = 0;
    if (realloc && irq_base >= 0) {
    virq = irq_base;
    } else {
    virq = irq_domain_alloc_descs(irq_base, nr_irqs, 0, node,
    affinity);
    if (virq < 0) {
    pr_debug!("cannot allocate IRQ(base %d, count %d)\n",
    irq_base, nr_irqs);
    return virq;
    }
    }
    if (irq_domain_alloc_irq_data(domain, virq, nr_irqs)) {
    pr_debug!("cannot allocate memory for IRQ%d\n", virq);
    ret = -ENOMEM;
// goto;
    }
    ret = irq_domain_alloc_irqs_hierarchy(domain, virq, nr_irqs, arg);
    if (ret < 0) {
// goto;
    }
    while (i < nr_irqs) {
    ret = irq_domain_trim_hierarchy(virq + i);
    if (ret) {
// goto;
    }
    }
    for (i = 0; i < nr_irqs; i++) {
    irq_domain_insert_irq(virq + i);
    }
    return virq;
// label;
    irq_domain_free_irqs_hierarchy(domain, virq, nr_irqs);
// label;
    irq_domain_free_irq_data(virq, nr_irqs);
// label;
    irq_free_descs(virq, nr_irqs);
    return ret;
    }
//
// __irq_domain_alloc_irqs - Allocate IRQs from domain
// @domain:	domain to allocate from
// @irq_base:	allocate specified IRQ number if irq_base >= 0
// @nr_irqs:	number of IRQs to allocate
// @node:	NUMA node id for memory allocation
// @arg:	domain specific argument
// @realloc:	IRQ descriptors have already been allocated if true
// @affinity:	Optional irq affinity mask for multiqueue devices
//
// Allocate IRQ numbers and initialized all data structures to support
// hierarchy IRQ domains.
// Parameter @realloc is mainly to support legacy IRQs.
// Returns error code or allocated IRQ number
//
// The whole process to setup an IRQ has been split into two steps.
// The first step, __irq_domain_alloc_irqs(), is to allocate IRQ
// descriptor and required hardware resources. The second step,
// irq_domain_activate_irq(), is to program the hardware with preallocated
// resources. In this way, it's easier to rollback when failing to
// allocate resources.
//
#[no_mangle]
pub unsafe extern "C" fn __irq_domain_alloc_irqs(domain: *mut irq_domain, irq_base: c_int, nr_irqs: c_uint, node: c_int, arg: *mut c_void, realloc: bool, affinity: *mut irq_affinity_desc) -> c_int {
    let mut ret = 0;
    if (domain == core::ptr::null_mut()) {
    domain = irq_default_domain;
    if (WARN(!domain, "domain is core::ptr::null_mut(); cannot allocate IRQ\n")) {
    return -EINVAL;
    }
    }
    mutex_lock(&domain.root.mutex);
    ret = irq_domain_alloc_irqs_locked(domain, irq_base, nr_irqs, node, arg,
    realloc, affinity);
    mutex_unlock(&domain.root.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__irq_domain_alloc_irqs);
// The irq_data was moved, fix the revmap to refer to the new location
#[no_mangle]
unsafe extern "C" fn irq_domain_fix_revmap(d: *mut irq_data) {
    let mut slot = core::ptr::null_mut();
    lockdep_assert_held(&d.domain.root.mutex);
    if (irq_domain_is_nomap(d.domain)) {
    return;
    }
// Fix up the revmap.
    if (d.hwirq < d.domain.revmap_size) {
// Not using radix tree
    rcu_assign_pointer(d.domain.revmap[d.hwirq], d);
    } else {
    slot = radix_tree_lookup_slot(&d.domain.revmap_tree, d.hwirq);
    if (slot) {
    radix_tree_replace_slot(&d.domain.revmap_tree, slot, d);
    }
    }
    }
//
// irq_domain_push_irq() - Push a domain in to the top of a hierarchy.
// @domain:	Domain to push.
// @virq:	Irq to push the domain in to.
// @arg:	Passed to the irq_domain_ops alloc() function.
//
// For an already existing irqdomain hierarchy, as might be obtained
// via a call to pci_enable_msix(), add an additional domain to the
// head of the processing chain.  Must be called before request_irq()
// has been called.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_push_irq(domain: *mut irq_domain, virq: c_int, arg: *mut c_void) -> c_int {
    let mut irq_data = irq_get_irq_data(virq);
pub static mut parent_irq_data: *mut c_void = core::ptr::null_mut();
pub static mut desc: *mut c_void = core::ptr::null_mut();
pub static mut rv: c_int = 0;
//
// Check that no action has been set, which indicates the virq
// is in a state where this function doesn't have to deal with
// races between interrupt handling and maintaining the
// hierarchy.  This will catch gross misuse.  Attempting to
// make the check race free would require holding locks across
// calls to struct irq_domain_ops->alloc(), which could lead
// to deadlock, so we just do a simple check before starting.
//
    desc = irq_to_desc(virq);
    if (!desc) {
    return -EINVAL;
    }
    if (WARN_ON!(desc.action)) {
    return -EBUSY;
    }
    if (domain == core::ptr::null_mut()) {
    return -EINVAL;
    }
    if (WARN_ON!(!irq_domain_is_hierarchy(domain))) {
    return -EINVAL;
    }
    if (!irq_data) {
    return -EINVAL;
    }
    if (domain.parent != irq_data.domain) {
    return -EINVAL;
    }
    parent_irq_data = kzalloc_node(sizeof!(*parent_irq_data), GFP_KERNEL,
    irq_data_get_node(irq_data));
    if (!parent_irq_data) {
    return -ENOMEM;
    }
    mutex_lock(&domain.root.mutex);
// Copy the original irq_data.
// parent_irq_data = *irq_data;
//
// Overwrite the irq_data, which is embedded in struct irq_desc, with
// values for this domain.
//
    irq_data.parent_data = parent_irq_data;
    irq_data.domain = domain;
    irq_data.mask = 0;
    irq_data.hwirq = 0;
    irq_data.chip = core::ptr::null_mut();
    irq_data.chip_data = core::ptr::null_mut();
// May (probably does) set hwirq, chip, etc.
    rv = irq_domain_alloc_irqs_hierarchy(domain, virq, 1, arg);
    if (rv) {
// Restore the original irq_data.
// irq_data = *parent_irq_data;
    kfree(parent_irq_data);
// goto;
    }
    irq_domain_fix_revmap(parent_irq_data);
    irq_domain_set_mapping(domain, irq_data.hwirq, irq_data);
// label;
    mutex_unlock(&domain.root.mutex);
    return rv;
    }
    EXPORT_SYMBOL_GPL(irq_domain_push_irq);
//
// irq_domain_pop_irq() - Remove a domain from the top of a hierarchy.
// @domain:	Domain to remove.
// @virq:	Irq to remove the domain from.
//
// Undo the effects of a call to irq_domain_push_irq().  Must be
// called either before request_irq() or after free_irq().
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_pop_irq(domain: *mut irq_domain, virq: c_int) -> c_int {
    let mut irq_data = irq_get_irq_data(virq);
pub static mut parent_irq_data: *mut c_void = core::ptr::null_mut();
pub static mut tmp_irq_data: *mut c_void = core::ptr::null_mut();
pub static mut desc: *mut c_void = core::ptr::null_mut();
//
// Check that no action is set, which indicates the virq is in
// a state where this function doesn't have to deal with races
// between interrupt handling and maintaining the hierarchy.
// This will catch gross misuse.  Attempting to make the check
// race free would require holding locks across calls to
// struct irq_domain_ops->free(), which could lead to
// deadlock, so we just do a simple check before starting.
//
    desc = irq_to_desc(virq);
    if (!desc) {
    return -EINVAL;
    }
    if (WARN_ON!(desc.action)) {
    return -EBUSY;
    }
    if (domain == core::ptr::null_mut()) {
    return -EINVAL;
    }
    if (!irq_data) {
    return -EINVAL;
    }
    tmp_irq_data = irq_domain_get_irq_data(domain, virq);
// We can only "pop" if this domain is at the top of the list
    if (WARN_ON!(irq_data != tmp_irq_data)) {
    return -EINVAL;
    }
    if (WARN_ON!(irq_data.domain != domain)) {
    return -EINVAL;
    }
    parent_irq_data = irq_data.parent_data;
    if (WARN_ON!(!parent_irq_data)) {
    return -EINVAL;
    }
    mutex_lock(&domain.root.mutex);
    irq_data.parent_data = core::ptr::null_mut();
    irq_domain_clear_mapping(domain, irq_data.hwirq);
    irq_domain_free_irqs_hierarchy(domain, virq, 1);
// Restore the original irq_data.
// irq_data = *parent_irq_data;
    irq_domain_fix_revmap(irq_data);
    mutex_unlock(&domain.root.mutex);
    kfree(parent_irq_data);
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_domain_pop_irq);
//
// irq_domain_free_irqs - Free IRQ number and associated data structures
// @virq:	base IRQ number
// @nr_irqs:	number of IRQs to free
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_irqs(virq: c_uint, nr_irqs: c_uint) {
    let mut data = irq_get_irq_data(virq);
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (WARN(!data || !data.domain || !data.domain.ops.free,
    "core::ptr::null_mut() pointer, cannot free irq\n")) {
    return;
    }
    domain = data.domain;
    mutex_lock(&domain.root.mutex);
    for (i = 0; i < nr_irqs; i++) {
    irq_domain_remove_irq(virq + i);
    }
    irq_domain_free_irqs_hierarchy(domain, virq, nr_irqs);
    mutex_unlock(&domain.root.mutex);
    irq_domain_free_irq_data(virq, nr_irqs);
    irq_free_descs(virq, nr_irqs);
    }
    EXPORT_SYMBOL_GPL(irq_domain_free_irqs);
#[no_mangle]
unsafe extern "C" fn irq_domain_free_one_irq(domain: *mut irq_domain, virq: c_uint) {
    if (irq_domain_is_msi_device(domain)) {
    msi_device_domain_free_wired(domain, virq);
    }
    else {
    irq_domain_free_irqs(virq, 1);
    }
    }
//
// irq_domain_alloc_irqs_parent - Allocate interrupts from parent domain
// @domain:	Domain below which interrupts must be allocated
// @irq_base:	Base IRQ number
// @nr_irqs:	Number of IRQs to allocate
// @arg:	Allocation data (arch/domain specific)
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_alloc_irqs_parent(domain: *mut irq_domain, irq_base: c_uint, nr_irqs: c_uint, arg: *mut c_void) -> c_int {
    if (!domain.parent) {
    return -ENOSYS;
    }
    return irq_domain_alloc_irqs_hierarchy(domain.parent, irq_base,
    nr_irqs, arg);
    }
    EXPORT_SYMBOL_GPL(irq_domain_alloc_irqs_parent);
//
// irq_domain_free_irqs_parent - Free interrupts from parent domain
// @domain:	Domain below which interrupts must be freed
// @irq_base:	Base IRQ number
// @nr_irqs:	Number of IRQs to free
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_irqs_parent(domain: *mut irq_domain, irq_base: c_uint, nr_irqs: c_uint) {
    if (!domain.parent) {
    return;
    }
    irq_domain_free_irqs_hierarchy(domain.parent, irq_base, nr_irqs);
    }
    EXPORT_SYMBOL_GPL(irq_domain_free_irqs_parent);
#[no_mangle]
unsafe extern "C" fn __irq_domain_deactivate_irq(irq_data: *mut irq_data) {
    if (irq_data.domain) {
    let mut domain = irq_data.domain;
    if (domain.ops.deactivate) {
    domain.ops.deactivate(domain, irq_data);
    }
    if (irq_data.parent_data) {
    __irq_domain_deactivate_irq(irq_data.parent_data);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __irq_domain_activate_irq(irqd: *mut irq_data, reserve: bool) -> c_int {
pub static mut ret: c_int = 0;
    if (irqd.domain) {
    let mut domain = irqd.domain;
    if (irqd.parent_data) {
    ret = __irq_domain_activate_irq(irqd.parent_data,
    reserve);
    }
    if (!ret && domain.ops.activate) {
    ret = domain.ops.activate(domain, irqd, reserve);
// Rollback in case of error
    if (ret && irqd.parent_data) {
    __irq_domain_deactivate_irq(irqd.parent_data);
    }
    }
    }
    return ret;
    }
//
// irq_domain_activate_irq - Call domain_ops->activate recursively to activate
// interrupt
// @irq_data:	Outermost irq_data associated with interrupt
// @reserve:	If set only reserve an interrupt vector instead of assigning one
//
// This is the second step to call domain_ops->activate to program interrupt
// controllers, so the interrupt could actually get delivered.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_activate_irq(irq_data: *mut irq_data, reserve: bool) -> c_int {
pub static mut ret: c_int = 0;
    if (!irqd_is_activated(irq_data)) {
    ret = __irq_domain_activate_irq(irq_data, reserve);
    }
    if (!ret) {
    irqd_set_activated(irq_data);
    }
    return ret;
    }
//
// irq_domain_deactivate_irq - Call domain_ops->deactivate recursively to
// deactivate interrupt
// @irq_data: outermost irq_data associated with interrupt
//
// It calls domain_ops->deactivate to program interrupt controllers to disable
// interrupt delivery.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_deactivate_irq(irq_data: *mut irq_data) {
    if (irqd_is_activated(irq_data)) {
    __irq_domain_deactivate_irq(irq_data);
    irqd_clr_activated(irq_data);
    }
    }
#[no_mangle]
unsafe extern "C" fn irq_domain_check_hierarchy(domain: *mut irq_domain) {
// Hierarchy irq_domains must implement callback alloc()
    if (domain.ops.alloc) {
    domain.flags |= IRQ_DOMAIN_FLAG_HIERARCHY;
    }
    }

//
// irq_domain_get_irq_data - Get irq_data associated with @virq and @domain
// @domain:	domain to match
// @virq:	IRQ number to get irq_data
//
#[no_mangle]
#[no_mangle]
// duplicate fn: irq_domain_get_irq_data
pub unsafe extern "C" fn irq_domain_get_irq_data_dup(domain: *mut irq_domain, virq: c_uint) -> *mut c_void {
    let mut irq_data = irq_get_irq_data(virq);
    return (irq_data && irq_data.domain == domain) ? irq_data : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(irq_domain_get_irq_data);
//
// irq_domain_set_info - Set the complete data for a @virq in @domain
// @domain:		Interrupt domain to match
// @virq:		IRQ number
// @hwirq:		The hardware interrupt number
// @chip:		The associated interrupt chip
// @chip_data:		The associated interrupt chip data
// @handler:		The interrupt flow handler
// @handler_data:	The interrupt flow handler data
// @handler_name:	The interrupt handler name
//
#[no_mangle]
#[no_mangle]
// duplicate fn: irq_domain_set_info
pub unsafe extern "C" fn irq_domain_set_info_dup(domain: *mut irq_domain, virq: c_uint, hwirq: irq_hw_number_t, chip: *mut irq_chip, chip_data: *mut c_void, handler: irq_flow_handler_t, handler_data: *mut c_void, handler_name: *mut c_char) {
    irq_set_chip_and_handler_name(virq, chip, handler, handler_name);
    irq_set_chip_data(virq, chip_data);
    irq_set_handler_data(virq, handler_data);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: irq_domain_alloc_irqs_locked
pub unsafe extern "C" fn irq_domain_alloc_irqs_locked_dup(domain: *mut irq_domain, irq_base: c_int, nr_irqs: c_uint, node: c_int, arg: *mut c_void, realloc: bool, affinity: *mut irq_affinity_desc) -> c_int {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_check_hierarchy(domain: *mut irq_domain) { }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_one_irq(domain: *mut irq_domain, virq: c_uint) { }

pub static mut domain_dir: *mut c_void = core::ptr::null_mut();
pub static mut irq_bit_descr: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_domain_debug_show_one(m: *mut seq_file, d: *mut irq_domain, ind: c_int) {
    seq_printf(m, "%*sname:   %s\n", ind, "", d.name);
    seq_printf(m, "%*ssize:   %u\n", ind + 1, "", d.revmap_size);
    seq_printf(m, "%*smapped: %u\n", ind + 1, "", d.mapcount);
    seq_printf(m, "%*sflags:  0x%08x\n", ind +1 , "", d.flags);
    irq_debug_show_bits(m, ind, d.flags, irqdomain_flags, ARRAY_SIZE!(irqdomain_flags));
    if (d.ops && d.ops.debug_show) {
    d.ops.debug_show(m, d, core::ptr::null_mut(), ind + 1);
    }

    if (!d.parent) {
    return;
    }
    seq_printf(m, "%*sparent: %s\n", ind + 1, "", d.parent.name);
    irq_domain_debug_show_one(m, d.parent, ind + 4);

    }
#[no_mangle]
unsafe extern "C" fn irq_domain_debug_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    let mut d = m.private;
// Default domain? Might be NULL
    if (!d) {
    if (!irq_default_domain) {
    return 0;
    }
    d = irq_default_domain;
    }
    irq_domain_debug_show_one(m, d, 0);
    return 0;
    }
pub static mut irq_domain_debug: usize = 0;
#[no_mangle]
unsafe extern "C" fn debugfs_add_domain_dir(d: *mut irq_domain) {
    if (!d.name || !domain_dir) {
    return;
    }
    debugfs_create_file(d.name, 0444, domain_dir, d,
    &irq_domain_debug_fops);
    }
#[no_mangle]
unsafe extern "C" fn debugfs_remove_domain_dir(d: *mut irq_domain) {
    debugfs_lookup_and_remove(d.name, domain_dir);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_domain_debugfs_init(root: *mut dentry)  {
pub static mut d: *mut c_void = core::ptr::null_mut();
    domain_dir = debugfs_create_dir("domains", root);
    debugfs_create_file("default", 0444, domain_dir, core::ptr::null_mut(),
    &irq_domain_debug_fops);
    mutex_lock(&irq_domain_mutex);
    list_for_each_entry(d, &irq_domain_list, link) {
    debugfs_add_domain_dir(d);
    }
    mutex_unlock(&irq_domain_mutex);
    }