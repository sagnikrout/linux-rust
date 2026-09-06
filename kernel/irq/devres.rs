//! Automatically rewritten from C to Rust
//! Source: kernel/irq/devres.c
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
// Device resource management aware IRQ request/free implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_devres {
    pub irq: c_uint,
    pub dev_id: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn devm_irq_release(dev: *mut device, res: *mut c_void) {
    let mut this = res;
    free_irq(this.irq, this.dev_id);
    }
#[no_mangle]
unsafe extern "C" fn devm_irq_match(dev: *mut device, res: *mut c_void, data: *mut c_void) -> c_int {
    let mut this = res, *match = data;
    return this.irq == match.irq && this.dev_id == match.dev_id;
    }
#[no_mangle]
pub unsafe extern "C" fn devm_request_result(dev: *mut device, rc: c_int, irq: c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, devname: *mut c_char) -> c_int {
    if (rc >= 0) {
    return rc;
    }
    return dev_err_probe(dev, rc, "request_irq(%u) %ps %ps %s\n",
    irq, handler, thread_fn, devname ? : "");
    }
#[no_mangle]
pub unsafe extern "C" fn __devm_request_threaded_irq(dev: *mut device, irq: c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, irqflags: c_ulong, devname: *mut c_char, dev_id: *mut c_void) -> c_int {
pub static mut dr: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
    dr = devres_alloc(devm_irq_release, sizeof!(irq_devres),
    GFP_KERNEL);
    if (!dr) {
    return -ENOMEM;
    }
    if (!devname) {
    devname = dev_name(dev);
    }
    rc = request_threaded_irq(irq, handler, thread_fn, irqflags, devname,
    dev_id);
    if (rc) {
    devres_free(dr);
    return rc;
    }
    dr.irq = irq;
    dr.dev_id = dev_id;
    devres_add(dev, dr);
    return 0;
    }
//
// devm_request_threaded_irq - allocate an interrupt line for a managed device with error logging
// @dev:	Device to request interrupt for
// @irq:	Interrupt line to allocate
// @handler:	Function to be called when the interrupt occurs
// @thread_fn:	Function to be called in a threaded interrupt context. NULL
// for devices which handle everything in @handler
// @irqflags:	Interrupt type flags
// @devname:	An ascii name for the claiming device, dev_name(dev) if NULL
// @dev_id:	A cookie passed back to the handler function
//
// Except for the extra @dev argument, this function takes the same
// arguments and performs the same function as request_threaded_irq().
// Interrupts requested with this function will be automatically freed on
// driver detach.
//
// If an interrupt allocated with this function needs to be freed
// separately, devm_free_irq() must be used.
//
// When the request fails, an error message is printed with contextual
// information (device name, interrupt number, handler functions and
// error code). Don't add extra error messages at the call sites.
//
// Return: 0 on success or a negative error number.
//
#[no_mangle]
pub unsafe extern "C" fn devm_request_threaded_irq(dev: *mut device, irq: c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, irqflags: c_ulong, devname: *mut c_char, dev_id: *mut c_void) -> c_int {
    let mut rc = __devm_request_threaded_irq(dev, irq, handler, thread_fn,
    irqflags, devname, dev_id);
    return devm_request_result(dev, rc, irq, handler, thread_fn, devname);
    }
    EXPORT_SYMBOL(devm_request_threaded_irq);
#[no_mangle]
pub unsafe extern "C" fn __devm_request_any_context_irq(dev: *mut device, irq: c_uint, handler: irq_handler_t, irqflags: c_ulong, devname: *mut c_char, dev_id: *mut c_void) -> c_int {
pub static mut dr: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
    dr = devres_alloc(devm_irq_release, sizeof!(irq_devres),
    GFP_KERNEL);
    if (!dr) {
    return -ENOMEM;
    }
    if (!devname) {
    devname = dev_name(dev);
    }
    rc = request_any_context_irq(irq, handler, irqflags, devname, dev_id);
    if (rc < 0) {
    devres_free(dr);
    return rc;
    }
    dr.irq = irq;
    dr.dev_id = dev_id;
    devres_add(dev, dr);
    return rc;
    }
//
// devm_request_any_context_irq - allocate an interrupt line for a managed device with error logging
// @dev:	Device to request interrupt for
// @irq:	Interrupt line to allocate
// @handler:	Function to be called when the interrupt occurs
// @irqflags:	Interrupt type flags
// @devname:	An ascii name for the claiming device, dev_name(dev) if NULL
// @dev_id:	A cookie passed back to the handler function
//
// Except for the extra @dev argument, this function takes the same
// arguments and performs the same function as request_any_context_irq().
// Interrupts requested with this function will be automatically freed on
// driver detach.
//
// If an interrupt allocated with this function needs to be freed
// separately, devm_free_irq() must be used.
//
// When the request fails, an error message is printed with contextual
// information (device name, interrupt number, handler functions and
// error code). Don't add extra error messages at the call sites.
//
// Return: IRQC_IS_HARDIRQ or IRQC_IS_NESTED on success, or a negative error
// number.
//
#[no_mangle]
pub unsafe extern "C" fn devm_request_any_context_irq(dev: *mut device, irq: c_uint, handler: irq_handler_t, irqflags: c_ulong, devname: *mut c_char, dev_id: *mut c_void) -> c_int {
    let mut rc = __devm_request_any_context_irq(dev, irq, handler, irqflags,
    devname, dev_id);
    return devm_request_result(dev, rc, irq, handler, core::ptr::null_mut(), devname);
    }
    EXPORT_SYMBOL(devm_request_any_context_irq);
//
// devm_free_irq - free an interrupt
// @dev: device to free interrupt for
// @irq: Interrupt line to free
// @dev_id: Device identity to free
//
// Except for the extra @dev argument, this function takes the
// same arguments and performs the same function as free_irq().
// This function instead of free_irq() should be used to manually
// free IRQs allocated with devm_request_irq().
//
#[no_mangle]
pub unsafe extern "C" fn devm_free_irq(dev: *mut device, irq: c_uint, dev_id: *mut c_void) {
pub static mut match_data: irq_devres = 0;
    WARN_ON!(devres_release(dev, devm_irq_release, devm_irq_match,
    &match_data));
    }
    EXPORT_SYMBOL(devm_free_irq);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_desc_devres {
    pub from: c_uint,
    pub cnt: c_uint,
}

#[no_mangle]
unsafe extern "C" fn devm_irq_desc_release(dev: *mut device, res: *mut c_void) {
    let mut this = res;
    irq_free_descs(this.from, this.cnt);
    }
//
// __devm_irq_alloc_descs - Allocate and initialize a range of irq descriptors
// for a managed device
// @dev:	Device to allocate the descriptors for
// @irq:	Allocate for specific irq number if irq >= 0
// @from:	Start the search from this irq number
// @cnt:	Number of consecutive irqs to allocate
// @node:	Preferred node on which the irq descriptor should be allocated
// @owner:	Owning module (can be NULL)
// @affinity:	Optional pointer to an irq_affinity_desc array of size @cnt
// which hints where the irq descriptors should be allocated
// and which default affinities to use
//
// Returns the first irq number or error code.
//
// Note: Use the provided wrappers (devm_irq_alloc_desc*) for simplicity.
//
#[no_mangle]
pub unsafe extern "C" fn __devm_irq_alloc_descs(dev: *mut device, irq: c_int, from: c_uint, cnt: c_uint, node: c_int, owner: *mut module, affinity: *mut irq_affinity_desc) -> c_int {
pub static mut dr: *mut c_void = core::ptr::null_mut();
    let mut base = 0;
    dr = devres_alloc(devm_irq_desc_release, sizeof!(*dr), GFP_KERNEL);
    if (!dr) {
    return -ENOMEM;
    }
    base = __irq_alloc_descs(irq, from, cnt, node, owner, affinity);
    if (base < 0) {
    devres_free(dr);
    return base;
    }
    dr.from = base;
    dr.cnt = cnt;
    devres_add(dev, dr);
    return base;
    }
    EXPORT_SYMBOL_GPL(__devm_irq_alloc_descs);

//
// devm_irq_alloc_generic_chip - Allocate and initialize a generic chip
// for a managed device
// @dev:	Device to allocate the generic chip for
// @name:	Name of the irq chip
// @num_ct:	Number of irq_chip_type instances associated with this
// @irq_base:	Interrupt base nr for this chip
// @reg_base:	Register base address (virtual)
// @handler:	Default flow handler associated with this chip
//
// Returns an initialized irq_chip_generic structure. The chip defaults
// to the primary (index 0) irq_chip_type and @handler
//
#[no_mangle]
pub unsafe extern "C" fn devm_irq_alloc_generic_chip(dev: *mut device, name: *mut c_char, num_ct: c_int, irq_base: c_uint, reg_base: *mut c_void, handler: irq_flow_handler_t) -> *mut c_void {
pub static mut gc: *mut c_void = core::ptr::null_mut();
    gc = devm_kzalloc(dev, struct_size(gc, chip_types, num_ct), GFP_KERNEL);
    if (gc) {
    irq_init_generic_chip(gc, name, num_ct,
    irq_base, reg_base, handler);
    }
    return gc;
    }
    EXPORT_SYMBOL_GPL(devm_irq_alloc_generic_chip);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_generic_chip_devres {
    pub gc: *mut irq_chip_generic,
    pub msk: u32,
    pub clr: c_uint,
    pub set: c_uint,
}

#[no_mangle]
unsafe extern "C" fn devm_irq_remove_generic_chip(dev: *mut device, res: *mut c_void) {
    let mut this = res;
    irq_remove_generic_chip(this.gc, this.msk, this.clr, this.set);
    }
//
// devm_irq_setup_generic_chip - Setup a range of interrupts with a generic
// chip for a managed device
//
// @dev:	Device to setup the generic chip for
// @gc:		Generic irq chip holding all data
// @msk:	Bitmask holding the irqs to initialize relative to gc->irq_base
// @flags:	Flags for initialization
// @clr:	IRQ_* bits to clear
// @set:	IRQ_* bits to set
//
// Set up max. 32 interrupts starting from gc->irq_base. Note, this
// initializes all interrupts to the primary irq_chip_type and its
// associated handler.
//
#[no_mangle]
pub unsafe extern "C" fn devm_irq_setup_generic_chip(dev: *mut device, gc: *mut irq_chip_generic, msk: u32, flags: irq_gc_flags, clr: c_uint, set: c_uint) -> c_int {
pub static mut dr: *mut c_void = core::ptr::null_mut();
    dr = devres_alloc(devm_irq_remove_generic_chip,
    sizeof!(*dr), GFP_KERNEL);
    if (!dr) {
    return -ENOMEM;
    }
    irq_setup_generic_chip(gc, msk, flags, clr, set);
    dr.gc = gc;
    dr.msk = msk;
    dr.clr = clr;
    dr.set = set;
    devres_add(dev, dr);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_irq_setup_generic_chip);

#[no_mangle]
unsafe extern "C" fn devm_irq_domain_remove(dev: *mut device, res: *mut c_void) {
    let mut domain = res;
    irq_domain_remove(*domain);
    }
//
// devm_irq_domain_instantiate() - Instantiate a new irq domain data for a
// managed device.
// @dev:	Device to instantiate the domain for
// @info:	Domain information pointer pointing to the information for this
// domain
//
// Return: A pointer to the instantiated irq domain or an ERR_PTR value.
//
#[no_mangle]
pub unsafe extern "C" fn devm_irq_domain_instantiate(dev: *mut device, info: *mut irq_domain_info) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
pub static mut dr: *mut c_void = core::ptr::null_mut();
    dr = devres_alloc(devm_irq_domain_remove, sizeof!(*dr), GFP_KERNEL);
    if (!dr) {
    return ERR_PTR(-ENOMEM);
    }
    domain = irq_domain_instantiate(info);
    if (!IS_ERR(domain)) {
// dr = domain;
    devres_add(dev, dr);
    } else {
    devres_free(dr);
    }
    return domain;
    }
    EXPORT_SYMBOL_GPL(devm_irq_domain_instantiate);