//! Automatically rewritten from C to Rust
//! Source: kernel/dma/mapping.c
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
// arch-independent dma-mapping routines
//
// Copyright (c) 2006  SUSE Linux Products GmbH
// Copyright (c) 2006  Tejun Heo <teheo@suse.de>
//

// Macro flag: #define CREATE_TRACE_POINTS

    defined(CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU) || 
    defined(CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU_ALL)
pub static mut dma_default_coherent: bool = false;

//
// Managed DMA API
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_devres {
    pub size: usize,
    pub vaddr: *mut c_void,
    pub dma_handle: dma_addr_t,
    pub attrs: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn dmam_release(dev: *mut device, res: *mut c_void) {
    let mut this = res;
    dma_free_attrs(dev, this.size, this.vaddr, this.dma_handle,
    this.attrs);
    }
#[no_mangle]
unsafe extern "C" fn dmam_match(dev: *mut device, res: *mut c_void, match_data: *mut c_void) -> c_int {
    let mut this = res, *match = match_data;
    if (this.vaddr == match.vaddr) {
    WARN_ON!(this.size != match.size ||
    this.dma_handle != match.dma_handle);
    return 1;
    }
    return 0;
    }
//
// dmam_free_coherent - Managed dma_free_coherent()
// @dev: Device to free coherent memory for
// @size: Size of allocation
// @vaddr: Virtual address of the memory to free
// @dma_handle: DMA handle of the memory to free
//
// Managed dma_free_coherent().
//
#[no_mangle]
pub unsafe extern "C" fn dmam_free_coherent(dev: *mut device, size: size_t, vaddr: *mut c_void, dma_handle: dma_addr_t) {
pub static mut match_data: dma_devres = 0;
    WARN_ON!(devres_destroy(dev, dmam_release, dmam_match, &match_data));
    dma_free_coherent(dev, size, vaddr, dma_handle);
    }
    EXPORT_SYMBOL(dmam_free_coherent);
//
// dmam_alloc_attrs - Managed dma_alloc_attrs()
// @dev: Device to allocate non_coherent memory for
// @size: Size of allocation
// @dma_handle: Out argument for allocated DMA handle
// @gfp: Allocation flags
// @attrs: Flags in the DMA_ATTR_* namespace.
//
// Managed dma_alloc_attrs().  Memory allocated using this function will be
// automatically released on driver detach.
//
// RETURNS:
// Pointer to allocated memory on success, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn dmam_alloc_attrs(dev: *mut device, size: size_t, dma_handle: *mut dma_addr_t, gfp: gfp_t, attrs: c_ulong) -> *mut c_void {
pub static mut dr: *mut c_void = core::ptr::null_mut();
pub static mut vaddr: *mut c_void = core::ptr::null_mut();
    dr = devres_alloc(dmam_release, sizeof!(*dr), gfp);
    if (!dr) {
    return core::ptr::null_mut();
    }
    vaddr = dma_alloc_attrs(dev, size, dma_handle, gfp, attrs);
    if (!vaddr) {
    devres_free(dr);
    return core::ptr::null_mut();
    }
    dr.vaddr = vaddr;
    dr.dma_handle = *dma_handle;
    dr.size = size;
    dr.attrs = attrs;
    devres_add(dev, dr);
    return vaddr;
    }
    EXPORT_SYMBOL(dmam_alloc_attrs);
#[no_mangle]
pub unsafe extern "C" fn dma_go_direct(dev: *mut device, mask: dma_addr_t, ops: *mut dma_map_ops) -> bool {
    if (use_dma_iommu(dev)) {
    return false;
    }
    if (likely(!ops)) {
    return true;
    }
    if (IS_ENABLED!(CONFIG_DMA_OPS_BYPASS) && dev_dma_ops_bypass(dev)) {
    return min_not_zero(mask, dev.bus_dma_limit) >=
    dma_direct_get_required_mask(dev);
    }
    return false;
    }
//
// Check if the devices uses a direct mapping for streaming DMA operations.
// This allows IOMMU drivers to set a bypass mode if the DMA mask is large
// enough.
//
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_direct(dev: *mut device, ops: *mut dma_map_ops) -> bool {
    return dma_go_direct(dev, dev.coherent_dma_mask, ops);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_map_direct(dev: *mut device, ops: *mut dma_map_ops) -> bool {
    return dma_go_direct(dev, *dev.dma_mask, ops);
    }
    dma_addr_t dma_map_phys(device *dev, phys_addr_t phys, size_t size,
    enum dma_data_direction dir, unsigned long attrs)
    {
    let mut ops = get_dma_ops(dev);
pub static mut is_mmio: bool = false;
pub static mut is_cc_shared: bool = false;
pub static mut addr: dma_addr_t = 0;
    BUG_ON!(!valid_dma_direction(dir));
    if (WARN_ON_ONCE!(!dev.dma_mask)) {
    return DMA_MAPPING_ERROR;
    }
    if (!dev_is_dma_coherent(dev) && (attrs & DMA_ATTR_REQUIRE_COHERENT)) {
    return DMA_MAPPING_ERROR;
    }
    if (dma_map_direct(dev, ops) ||
    (!is_mmio && !is_cc_shared &&
    arch_dma_map_phys_direct(dev, phys + size))) {
    addr = dma_direct_map_phys(dev, phys, size, dir, attrs, true);
    }

    else if (is_cc_shared) {
    return DMA_MAPPING_ERROR;
    }

    else if (use_dma_iommu(dev)) {
    addr = iommu_dma_map_phys(dev, phys, size, dir, attrs);
    }

    else if (ops.map_phys) {
    addr = ops.map_phys(dev, phys, size, dir, attrs);
    }
    if (!is_mmio) {
    kmsan_handle_dma(phys, size, dir);
    }
    trace_dma_map_phys(dev, phys, addr, size, dir, attrs);
    debug_dma_map_phys(dev, phys, size, dir, addr, attrs);
    return addr;
    }
    EXPORT_SYMBOL_GPL(dma_map_phys);
    dma_addr_t dma_map_page_attrs(device *dev, page *page,
    size_t offset, size_t size, enum dma_data_direction dir,
    unsigned long attrs)
    {
pub static mut phys: phys_addr_t = 0;
    if (unlikely(attrs & DMA_ATTR_MMIO)) {
    return DMA_MAPPING_ERROR;
    }
    if (IS_ENABLED!(CONFIG_DMA_API_DEBUG) &&
    WARN_ON_ONCE!(is_zone_device_page(page))) {
    return DMA_MAPPING_ERROR;
    }
    return dma_map_phys(dev, phys, size, dir, attrs);
    }
    EXPORT_SYMBOL(dma_map_page_attrs);
#[no_mangle]
pub unsafe extern "C" fn dma_unmap_phys(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction, attrs: c_ulong) {
    let mut ops = get_dma_ops(dev);
pub static mut is_mmio: bool = false;
pub static mut is_cc_shared: bool = false;
    BUG_ON!(!valid_dma_direction(dir));
    if (dma_map_direct(dev, ops) ||
    (!is_mmio && !is_cc_shared &&
    arch_dma_unmap_phys_direct(dev, addr + size))) {
    dma_direct_unmap_phys(dev, addr, size, dir, attrs, true);
    }

    else if (is_cc_shared) {
    return;
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_unmap_phys(dev, addr, size, dir, attrs);
    }

    else if (ops.unmap_phys) {
    ops.unmap_phys(dev, addr, size, dir, attrs);
    }
    trace_dma_unmap_phys(dev, addr, size, dir, attrs);
    debug_dma_unmap_phys(dev, addr, size, dir, attrs);
    }
    EXPORT_SYMBOL_GPL(dma_unmap_phys);
#[no_mangle]
pub unsafe extern "C" fn dma_unmap_page_attrs(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction, attrs: c_ulong) {
    if (unlikely(attrs & DMA_ATTR_MMIO)) {
    return;
    }
    dma_unmap_phys(dev, addr, size, dir, attrs);
    }
    EXPORT_SYMBOL(dma_unmap_page_attrs);
#[no_mangle]
pub unsafe extern "C" fn __dma_map_sg_attrs(dev: *mut device, sg: *mut scatterlist, nents: c_int, dir: dma_data_direction, attrs: c_ulong) -> c_int {
    let mut ops = get_dma_ops(dev);
    let mut ents = 0;
    BUG_ON!(!valid_dma_direction(dir));
    if (!dev_is_dma_coherent(dev) && (attrs & DMA_ATTR_REQUIRE_COHERENT)) {
    return -EOPNOTSUPP;
    }
    if (WARN_ON_ONCE!(!dev.dma_mask)) {
    return 0;
    }
    if (dma_map_direct(dev, ops) ||
    arch_dma_map_sg_direct(dev, sg, nents)) {
    ents = dma_direct_map_sg(dev, sg, nents, dir, attrs);
    }

    else if (use_dma_iommu(dev)) {
    ents = iommu_dma_map_sg(dev, sg, nents, dir, attrs);
    }
    else {
    ents = ops.map_sg(dev, sg, nents, dir, attrs);
    }
    if (ents > 0) {
    kmsan_handle_dma_sg(sg, nents, dir);
    trace_dma_map_sg(dev, sg, nents, ents, dir, attrs);
    debug_dma_map_sg(dev, sg, nents, ents, dir, attrs);
    } else if (WARN_ON_ONCE!(ents != -EINVAL && ents != -ENOMEM &&
    ents != -EIO && ents != -EREMOTEIO)) {
    trace_dma_map_sg_err(dev, sg, nents, ents, dir, attrs);
    return -EIO;
    }
    return ents;
    }
//
// dma_map_sg_attrs - Map the given buffer for DMA
// @dev:	The device for which to perform the DMA operation
// @sg:		The sg_table object describing the buffer
// @nents:	Number of entries to map
// @dir:	DMA direction
// @attrs:	Optional DMA attributes for the map operation
//
// Maps a buffer described by a scatterlist passed in the sg argument with
// nents segments for the @dir DMA operation by the @dev device.
//
// Returns the number of mapped entries (which can be less than nents)
// on success. Zero is returned for any error.
//
// dma_unmap_sg_attrs() should be used to unmap the buffer with the
// original sg and original nents (not the value returned by this funciton).
//
#[no_mangle]
pub unsafe extern "C" fn dma_map_sg_attrs(dev: *mut device, sg: *mut scatterlist, nents: c_int, dir: dma_data_direction, attrs: c_ulong) -> c_uint {
    let mut ret = 0;
    ret = __dma_map_sg_attrs(dev, sg, nents, dir, attrs);
    if (ret < 0) {
    return 0;
    }
    return ret;
    }
    EXPORT_SYMBOL(dma_map_sg_attrs);
//
// dma_map_sgtable - Map the given buffer for DMA
// @dev:	The device for which to perform the DMA operation
// @sgt:	The sg_table object describing the buffer
// @dir:	DMA direction
// @attrs:	Optional DMA attributes for the map operation
//
// Maps a buffer described by a scatterlist stored in the given sg_table
// object for the @dir DMA operation by the @dev device. After success, the
// ownership for the buffer is transferred to the DMA domain.  One has to
// call dma_sync_sgtable_for_cpu() or dma_unmap_sgtable() to move the
// ownership of the buffer back to the CPU domain before touching the
// buffer by the CPU.
//
// Returns 0 on success or a negative error code on error. The following
// error codes are supported with the given meaning:
//
// -EINVAL		An invalid argument, unaligned access or other error
// in usage. Will not succeed if retried.
// -ENOMEM		Insufficient resources (like memory or IOVA space) to
// complete the mapping. Should succeed if retried later.
// -EIO		Legacy error code with an unknown meaning. eg. this is
// returned if a lower level call returned
// DMA_MAPPING_ERROR.
// -EREMOTEIO		The DMA device cannot access P2PDMA memory specified
// in the sg_table. This will not succeed if retried.
//
#[no_mangle]
pub unsafe extern "C" fn dma_map_sgtable(dev: *mut device, sgt: *mut sg_table, dir: dma_data_direction, attrs: c_ulong) -> c_int {
    let mut nents = 0;
    nents = __dma_map_sg_attrs(dev, sgt.sgl, sgt.orig_nents, dir, attrs);
    if (nents < 0) {
    return nents;
    }
    sgt.nents = nents;
    return 0;
    }
    EXPORT_SYMBOL_GPL(dma_map_sgtable);
#[no_mangle]
pub unsafe extern "C" fn dma_unmap_sg_attrs(dev: *mut device, sg: *mut scatterlist, nents: c_int, dir: dma_data_direction, attrs: c_ulong) {
    let mut ops = get_dma_ops(dev);
    BUG_ON!(!valid_dma_direction(dir));
    trace_dma_unmap_sg(dev, sg, nents, dir, attrs);
    debug_dma_unmap_sg(dev, sg, nents, dir, attrs);
    if (dma_map_direct(dev, ops) ||
    arch_dma_unmap_sg_direct(dev, sg, nents)) {
    dma_direct_unmap_sg(dev, sg, nents, dir, attrs);
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_unmap_sg(dev, sg, nents, dir, attrs);
    }

    else if (ops.unmap_sg) {
    ops.unmap_sg(dev, sg, nents, dir, attrs);
    }
    }
    EXPORT_SYMBOL(dma_unmap_sg_attrs);
    dma_addr_t dma_map_resource(device *dev, phys_addr_t phys_addr,
    size_t size, enum dma_data_direction dir, unsigned long attrs)
    {
    return dma_map_phys(dev, phys_addr, size, dir, attrs | DMA_ATTR_MMIO);
    }
    EXPORT_SYMBOL(dma_map_resource);
#[no_mangle]
pub unsafe extern "C" fn dma_unmap_resource(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction, attrs: c_ulong) {
    dma_unmap_phys(dev, addr, size, dir, attrs | DMA_ATTR_MMIO);
    }
    EXPORT_SYMBOL(dma_unmap_resource);

#[no_mangle]
pub unsafe extern "C" fn __dma_sync_single_for_cpu(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction) {
    let mut ops = get_dma_ops(dev);
    BUG_ON!(!valid_dma_direction(dir));
    if (dma_map_direct(dev, ops)) {
    dma_direct_sync_single_for_cpu(dev, addr, size, dir, true);
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_sync_single_for_cpu(dev, addr, size, dir);
    }

    else if (ops.sync_single_for_cpu) {
    ops.sync_single_for_cpu(dev, addr, size, dir);
    }
    trace_dma_sync_single_for_cpu(dev, addr, size, dir);
    debug_dma_sync_single_for_cpu(dev, addr, size, dir);
    }
    EXPORT_SYMBOL(__dma_sync_single_for_cpu);
#[no_mangle]
pub unsafe extern "C" fn __dma_sync_single_for_device(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction) {
    let mut ops = get_dma_ops(dev);
    BUG_ON!(!valid_dma_direction(dir));
    if (dma_map_direct(dev, ops)) {
    dma_direct_sync_single_for_device(dev, addr, size, dir);
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_sync_single_for_device(dev, addr, size, dir);
    }

    else if (ops.sync_single_for_device) {
    ops.sync_single_for_device(dev, addr, size, dir);
    }
    trace_dma_sync_single_for_device(dev, addr, size, dir);
    debug_dma_sync_single_for_device(dev, addr, size, dir);
    }
    EXPORT_SYMBOL(__dma_sync_single_for_device);
#[no_mangle]
pub unsafe extern "C" fn __dma_sync_sg_for_cpu(dev: *mut device, sg: *mut scatterlist, nelems: c_int, dir: dma_data_direction) {
    let mut ops = get_dma_ops(dev);
    BUG_ON!(!valid_dma_direction(dir));
    if (dma_map_direct(dev, ops)) {
    dma_direct_sync_sg_for_cpu(dev, sg, nelems, dir);
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_sync_sg_for_cpu(dev, sg, nelems, dir);
    }

    else if (ops.sync_sg_for_cpu) {
    ops.sync_sg_for_cpu(dev, sg, nelems, dir);
    }
    trace_dma_sync_sg_for_cpu(dev, sg, nelems, dir);
    debug_dma_sync_sg_for_cpu(dev, sg, nelems, dir);
    }
    EXPORT_SYMBOL(__dma_sync_sg_for_cpu);
#[no_mangle]
pub unsafe extern "C" fn __dma_sync_sg_for_device(dev: *mut device, sg: *mut scatterlist, nelems: c_int, dir: dma_data_direction) {
    let mut ops = get_dma_ops(dev);
    BUG_ON!(!valid_dma_direction(dir));
    if (dma_map_direct(dev, ops)) {
    dma_direct_sync_sg_for_device(dev, sg, nelems, dir);
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_sync_sg_for_device(dev, sg, nelems, dir);
    }

    else if (ops.sync_sg_for_device) {
    ops.sync_sg_for_device(dev, sg, nelems, dir);
    }
    trace_dma_sync_sg_for_device(dev, sg, nelems, dir);
    debug_dma_sync_sg_for_device(dev, sg, nelems, dir);
    }
    EXPORT_SYMBOL(__dma_sync_sg_for_device);
#[no_mangle]
pub unsafe extern "C" fn __dma_need_sync(dev: *mut device, dma_addr: dma_addr_t) -> bool {
    let mut ops = get_dma_ops(dev);
    if (dma_map_direct(dev, ops)) {
//
// dma_skip_sync could've been reset on first SWIOTLB buffer
// mapping, but @dma_addr is not necessary an SWIOTLB buffer.
// In this case, fall back to more granular check.
//
    return dma_direct_need_sync(dev, dma_addr);
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(__dma_need_sync);
//
// dma_need_unmap - does this device need dma_unmap_* operations
// @dev: device to check
//
// If this function returns %false, drivers can skip calling dma_unmap_* after
// finishing an I/O.  This function must be called after all mappings that might
// need to be unmapped have been performed.
//
#[no_mangle]
pub unsafe extern "C" fn dma_need_unmap(dev: *mut device) -> bool {
    if (!dma_map_direct(dev, get_dma_ops(dev))) {
    return true;
    }
    if (!dev_dma_skip_sync(dev)) {
    return true;
    }
    return IS_ENABLED!(CONFIG_DMA_API_DEBUG);
    }
    EXPORT_SYMBOL_GPL(dma_need_unmap);
#[no_mangle]
unsafe extern "C" fn dma_setup_need_sync(dev: *mut device) {
    let mut ops = get_dma_ops(dev);
    if (dma_map_direct(dev, ops) || use_dma_iommu(dev)) {
//
// dma_skip_sync will be reset to %false on first SWIOTLB buffer
// mapping, if any. During the device initialization, it's
// enough to check only for the DMA coherence.
//
    dev_assign_dma_skip_sync(dev, dev_is_dma_coherent(dev));
    }
    else if (!ops.sync_single_for_device && !ops.sync_single_for_cpu &&
    !ops.sync_sg_for_device && !ops.sync_sg_for_cpu) {
//
// Synchronization is not possible when none of DMA sync ops
// is set.
//
    dev_set_dma_skip_sync(dev);
    }
    else {
    dev_clear_dma_skip_sync(dev);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn dma_setup_need_sync(dev: *mut device) { }

//
// The whole dma_get_sgtable() idea is fundamentally unsafe - it seems
// that the intention is to allow exporting memory allocated via the
// coherent DMA APIs through the dma_buf API, which only accepts a
// scattertable.  This presents a couple of problems:
// 1. Not all memory allocated via the coherent DMA APIs is backed by
// a struct page
// 2. Passing coherent DMA memory into the streaming APIs is not allowed
// as we will try to flush the memory through a different alias to that
// actually being used (and the flushes are redundant.)
//
#[no_mangle]
pub unsafe extern "C" fn dma_get_sgtable_attrs(dev: *mut device, sgt: *mut sg_table, cpu_addr: *mut c_void, dma_addr: dma_addr_t, size: size_t, attrs: c_ulong) -> c_int {
    let mut ops = get_dma_ops(dev);
    if (dma_alloc_direct(dev, ops)) {
    return dma_direct_get_sgtable(dev, sgt, cpu_addr, dma_addr,
    size, attrs);
    }
    if (use_dma_iommu(dev)) {
    return iommu_dma_get_sgtable(dev, sgt, cpu_addr, dma_addr,
    size, attrs);
    }
    if (!ops.get_sgtable) {
    return -ENXIO;
    }
    return ops.get_sgtable(dev, sgt, cpu_addr, dma_addr, size, attrs);
    }
    EXPORT_SYMBOL(dma_get_sgtable_attrs);

//
// Return the page attributes used for mapping dma_alloc_* memory, either in
// kernel space if remapping is needed, or to userspace through dma_mmap_*.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pgprot(dev: *mut device, prot: pgprot_t, attrs: c_ulong) -> pgprot_t {
    let mut dma_prot;
    if (dev_is_dma_coherent(dev)) {
    dma_prot = prot;
    }


    else if (attrs & DMA_ATTR_WRITE_COMBINE) {
    dma_prot = pgprot_writecombine(prot);
    }

    else {
    dma_prot = pgprot_dmacoherent(prot);
    }
    if (attrs & (DMA_ATTR_CC_SHARED | __DMA_ATTR_ALLOC_CC_SHARED)) {
    return pgprot_decrypted(dma_prot);
    }
    else {
    return pgprot_encrypted(dma_prot);
    }
    }

//
// dma_can_mmap - check if a given device supports dma_mmap_
// @dev: device to check
//
// Returns %true if @dev supports dma_mmap_coherent() and dma_mmap_attrs() to
// map DMA allocations to userspace.
//
#[no_mangle]
pub unsafe extern "C" fn dma_can_mmap(dev: *mut device) -> bool {
    let mut ops = get_dma_ops(dev);
    if (dma_alloc_direct(dev, ops)) {
    return dma_direct_can_mmap(dev);
    }
    if (use_dma_iommu(dev)) {
    return true;
    }
    return ops.mmap != core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(dma_can_mmap);
//
// dma_mmap_attrs - map a coherent DMA allocation into user space
// @dev: valid struct device pointer, or NULL for ISA and EISA-like devices
// @vma: vm_area_struct describing requested user mapping
// @cpu_addr: kernel CPU-view address returned from dma_alloc_attrs
// @dma_addr: device-view address returned from dma_alloc_attrs
// @size: size of memory originally requested in dma_alloc_attrs
// @attrs: attributes of mapping properties requested in dma_alloc_attrs
//
// Map a coherent DMA buffer previously allocated by dma_alloc_attrs into user
// space.  The coherent DMA buffer must not be freed by the driver until the
// user space mapping has been released.
//
#[no_mangle]
pub unsafe extern "C" fn dma_mmap_attrs(dev: *mut device, vma: *mut vm_area_struct, cpu_addr: *mut c_void, dma_addr: dma_addr_t, size: size_t, attrs: c_ulong) -> c_int {
    let mut ops = get_dma_ops(dev);
    if (dma_alloc_direct(dev, ops)) {
    return dma_direct_mmap(dev, vma, cpu_addr, dma_addr, size,
    attrs);
    }
    if (use_dma_iommu(dev)) {
    return iommu_dma_mmap(dev, vma, cpu_addr, dma_addr, size,
    attrs);
    }
    if (!ops.mmap) {
    return -ENXIO;
    }
    return ops.mmap(dev, vma, cpu_addr, dma_addr, size, attrs);
    }
    EXPORT_SYMBOL(dma_mmap_attrs);
#[no_mangle]
pub unsafe extern "C" fn dma_get_required_mask(dev: *mut device) -> u64 {
    let mut ops = get_dma_ops(dev);
    if (dma_alloc_direct(dev, ops)) {
    return dma_direct_get_required_mask(dev);
    }
    if (use_dma_iommu(dev)) {
    return DMA_BIT_MASK(32);
    }
    if (ops.get_required_mask) {
    return ops.get_required_mask(dev);
    }
//
// We require every DMA ops implementation to at least support a 32-bit
// DMA mask (and use bounce buffering if that isn't supported in
// hardware).  As the direct mapping code has its own routine to
// actually report an optimal mask we default to 32-bit here as that
// is the right thing for most IOMMUs, and at least not actively
// harmful in general.
//
    return DMA_BIT_MASK(32);
    }
    EXPORT_SYMBOL_GPL(dma_get_required_mask);
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_attrs(dev: *mut device, size: size_t, dma_handle: *mut dma_addr_t, flag: gfp_t, attrs: c_ulong) -> *mut c_void {
    let mut ops = get_dma_ops(dev);
pub static mut cpu_addr: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(!dev.coherent_dma_mask);
//
// DMA allocations can never be turned back into a page pointer, so
// requesting compound pages doesn't make sense (and can't even be
// supported at all by various backends).
//
    if (WARN_ON_ONCE!(flag & __GFP_COMP)) {
    return core::ptr::null_mut();
    }
    if (attrs & (DMA_ATTR_CC_SHARED | __DMA_ATTR_ALLOC_CC_SHARED)) {
    trace_dma_alloc(dev, core::ptr::null_mut(), 0, size, DMA_BIDIRECTIONAL, flag,
    attrs);
    return core::ptr::null_mut();
    }
    if (force_dma_unencrypted(dev)) {
    attrs |= __DMA_ATTR_ALLOC_CC_SHARED;
    }
    if (dma_alloc_from_dev_coherent(dev, size, dma_handle, &cpu_addr)) {
    trace_dma_alloc(dev, cpu_addr, *dma_handle, size,
    DMA_BIDIRECTIONAL, flag, attrs);
    return cpu_addr;
    }
// let the implementation decide on the zone to allocate from:
    flag &= ~(__GFP_DMA | __GFP_DMA32 | __GFP_HIGHMEM);
    if (dma_alloc_direct(dev, ops) || arch_dma_alloc_direct(dev)) {
    cpu_addr = dma_direct_alloc(dev, size, dma_handle, flag, attrs);
    } else if (use_dma_iommu(dev)) {
    cpu_addr = iommu_dma_alloc(dev, size, dma_handle, flag, attrs);
    } else if (ops.alloc) {
    cpu_addr = ops.alloc(dev, size, dma_handle, flag, attrs);
    } else {
    trace_dma_alloc(dev, core::ptr::null_mut(), 0, size, DMA_BIDIRECTIONAL, flag,
    attrs);
    return core::ptr::null_mut();
    }
    trace_dma_alloc(dev, cpu_addr, *dma_handle, size, DMA_BIDIRECTIONAL,
    flag, attrs);
    debug_dma_alloc_coherent(dev, size, *dma_handle, cpu_addr, attrs);
    return cpu_addr;
    }
    EXPORT_SYMBOL(dma_alloc_attrs);
#[no_mangle]
pub unsafe extern "C" fn dma_free_attrs(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_handle: dma_addr_t, attrs: c_ulong) {
    let mut ops = get_dma_ops(dev);
    if (dma_release_from_dev_coherent(dev, get_order(size), cpu_addr)) {
    return;
    }
//
// On non-coherent platforms which implement DMA-coherent buffers via
// non-cacheable remaps, ops->free() may call vunmap(). Thus getting
// this far in IRQ context is a) at risk of a BUG_ON!() or trying to
// sleep on some machines, and b) an indication that the driver is
// probably misusing the coherent API anyway.
//
    WARN_ON!(irqs_disabled());
    trace_dma_free(dev, cpu_addr, dma_handle, size, DMA_BIDIRECTIONAL,
    attrs);
    if (!cpu_addr) {
    return;
    }
    debug_dma_free_coherent(dev, size, cpu_addr, dma_handle, attrs);
    if (dma_alloc_direct(dev, ops) || arch_dma_free_direct(dev, dma_handle)) {
    dma_direct_free(dev, size, cpu_addr, dma_handle, attrs);
    }

    else if (use_dma_iommu(dev)) {
    iommu_dma_free(dev, size, cpu_addr, dma_handle, attrs);
    }

    else if (ops.free) {
    ops.free(dev, size, cpu_addr, dma_handle, attrs);
    }
    }
    EXPORT_SYMBOL(dma_free_attrs);
#[no_mangle]
pub unsafe extern "C" fn __dma_alloc_pages(dev: *mut device, size: size_t, dma_handle: *mut dma_addr_t, dir: dma_data_direction, gfp: gfp_t) -> *mut c_void {
    let mut ops = get_dma_ops(dev);
    if (WARN_ON_ONCE!(!dev.coherent_dma_mask)) {
    return core::ptr::null_mut();
    }
    if (WARN_ON_ONCE!(gfp & (__GFP_DMA | __GFP_DMA32 | __GFP_HIGHMEM))) {
    return core::ptr::null_mut();
    }
    if (WARN_ON_ONCE!(gfp & __GFP_COMP)) {
    return core::ptr::null_mut();
    }
    size = PAGE_ALIGN(size);
    if (dma_alloc_direct(dev, ops)) {
    return dma_direct_alloc_pages(dev, size, dma_handle, dir, gfp);
    }
    if (use_dma_iommu(dev)) {
    return dma_common_alloc_pages(dev, size, dma_handle, dir, gfp);
    }
    if (!ops.alloc_pages_op) {
    return core::ptr::null_mut();
    }
    return ops.alloc_pages_op(dev, size, dma_handle, dir, gfp);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_pages(dev: *mut device, size: size_t, dma_handle: *mut dma_addr_t, dir: dma_data_direction, gfp: gfp_t) -> *mut c_void {
    let mut page = __dma_alloc_pages(dev, size, dma_handle, dir, gfp);
    if (page) {
    trace_dma_alloc_pages(dev, page_to_virt(page), *dma_handle,
    size, dir, gfp, 0);
    debug_dma_alloc_pages(dev, page, size, dir, *dma_handle);
    } else {
    trace_dma_alloc_pages(dev, core::ptr::null_mut(), 0, size, dir, gfp, 0);
    }
    return page;
    }
    EXPORT_SYMBOL_GPL(dma_alloc_pages);
#[no_mangle]
pub unsafe extern "C" fn __dma_free_pages(dev: *mut device, size: size_t, page: *mut page, dma_handle: dma_addr_t, dir: dma_data_direction) {
    let mut ops = get_dma_ops(dev);
    size = PAGE_ALIGN(size);
    if (dma_alloc_direct(dev, ops)) {
    dma_direct_free_pages(dev, size, page, dma_handle, dir);
    }

    else if (use_dma_iommu(dev)) {
    dma_common_free_pages(dev, size, page, dma_handle, dir);
    }

    else if (ops.free_pages) {
    ops.free_pages(dev, size, page, dma_handle, dir);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dma_free_pages(dev: *mut device, size: size_t, page: *mut page, dma_handle: dma_addr_t, dir: dma_data_direction) {
    trace_dma_free_pages(dev, page_to_virt(page), dma_handle, size, dir, 0);
    debug_dma_free_pages(dev, page, size, dir, dma_handle);
    __dma_free_pages(dev, size, page, dma_handle, dir);
    }
    EXPORT_SYMBOL_GPL(dma_free_pages);
#[no_mangle]
pub unsafe extern "C" fn dma_mmap_pages(dev: *mut device, vma: *mut vm_area_struct, size: size_t, page: *mut page) -> c_int {
pub static mut pgoff_start: pgoff_t = 0;
pub static mut pgoff_end: pgoff_t = 0;
pub static mut count: c_ulong = 0;
    if (pgoff_start >= count || pgoff_end > count) {
    return -ENXIO;
    }
    return remap_pfn_range(vma, vma.vm_start,
    page_to_pfn(page) + pgoff_start,
    vma_pages(vma) << PAGE_SHIFT, vma.vm_page_prot);
    }
    EXPORT_SYMBOL_GPL(dma_mmap_pages);
#[no_mangle]
pub unsafe extern "C" fn alloc_single_sgt(dev: *mut device, size: size_t, dir: dma_data_direction, gfp: gfp_t) -> *mut c_void {
pub static mut sgt: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    sgt = kmalloc_obj(*sgt, gfp);
    if (!sgt) {
    return core::ptr::null_mut();
    }
    if (sg_alloc_table(sgt, 1, gfp)) {
// goto;
    }
    page = __dma_alloc_pages(dev, size, &sgt.sgl.dma_address, dir, gfp);
    if (!page) {
// goto;
    }
    sg_set_page(sgt.sgl, page, PAGE_ALIGN(size), 0);
    sg_dma_len(sgt.sgl) = sgt.sgl.length;
    return sgt;
// label;
    sg_free_table(sgt);
// label;
    kfree(sgt);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_noncontiguous(dev: *mut device, size: size_t, dir: dma_data_direction, gfp: gfp_t, attrs: c_ulong) -> *mut c_void {
pub static mut sgt: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(attrs & ~DMA_ATTR_ALLOC_SINGLE_PAGES)) {
    return core::ptr::null_mut();
    }
    if (WARN_ON_ONCE!(gfp & __GFP_COMP)) {
    return core::ptr::null_mut();
    }
    if (use_dma_iommu(dev)) {
    sgt = iommu_dma_alloc_noncontiguous(dev, size, dir, gfp, attrs);
    }
    else {
    sgt = alloc_single_sgt(dev, size, dir, gfp);
    }
    if (sgt) {
    sgt.nents = 1;
    trace_dma_alloc_sgt(dev, sgt, size, dir, gfp, attrs);
    debug_dma_map_sg(dev, sgt.sgl, sgt.orig_nents, 1, dir, attrs);
    } else {
    trace_dma_alloc_sgt_err(dev, core::ptr::null_mut(), 0, size, dir, gfp, attrs);
    }
    return sgt;
    }
    EXPORT_SYMBOL_GPL(dma_alloc_noncontiguous);
#[no_mangle]
pub unsafe extern "C" fn free_single_sgt(dev: *mut device, size: size_t, sgt: *mut sg_table, dir: dma_data_direction) {
    __dma_free_pages(dev, size, sg_page(sgt.sgl), sgt.sgl.dma_address,
    dir);
    sg_free_table(sgt);
    kfree(sgt);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_free_noncontiguous(dev: *mut device, size: size_t, sgt: *mut sg_table, dir: dma_data_direction) {
    trace_dma_free_sgt(dev, sgt, size, dir);
    debug_dma_unmap_sg(dev, sgt.sgl, sgt.orig_nents, dir, 0);
    if (use_dma_iommu(dev)) {
    iommu_dma_free_noncontiguous(dev, size, sgt, dir);
    }
    else {
    free_single_sgt(dev, size, sgt, dir);
    }
    }
    EXPORT_SYMBOL_GPL(dma_free_noncontiguous);
#[no_mangle]
pub unsafe extern "C" fn dma_vmap_noncontiguous(dev: *mut device, size: size_t, sgt: *mut sg_table) -> *mut c_void {
    if (use_dma_iommu(dev)) {
    return iommu_dma_vmap_noncontiguous(dev, size, sgt);
    }
    return page_address(sg_page(sgt.sgl));
    }
    EXPORT_SYMBOL_GPL(dma_vmap_noncontiguous);
#[no_mangle]
pub unsafe extern "C" fn dma_vunmap_noncontiguous(dev: *mut device, vaddr: *mut c_void) {
    if (use_dma_iommu(dev)) {
    iommu_dma_vunmap_noncontiguous(dev, vaddr);
    }
    }
    EXPORT_SYMBOL_GPL(dma_vunmap_noncontiguous);
#[no_mangle]
pub unsafe extern "C" fn dma_mmap_noncontiguous(dev: *mut device, vma: *mut vm_area_struct, size: size_t, sgt: *mut sg_table) -> c_int {
    if (use_dma_iommu(dev)) {
    return iommu_dma_mmap_noncontiguous(dev, vma, size, sgt);
    }
    return dma_mmap_pages(dev, vma, size, sg_page(sgt.sgl));
    }
    EXPORT_SYMBOL_GPL(dma_mmap_noncontiguous);
#[no_mangle]
unsafe extern "C" fn dma_supported(dev: *mut device, mask: u64) -> c_int {
    let mut ops = get_dma_ops(dev);
    if (use_dma_iommu(dev)) {
    if (WARN_ON!(ops)) {
    return false;
    }
    return true;
    }
//
// ->dma_supported sets and clears the bypass flag, so ignore it here
// and always call into the method if there is one.
//
    if (ops) {
    if (!ops.dma_supported) {
    return true;
    }
    return ops.dma_supported(dev, mask);
    }
    return dma_direct_supported(dev, mask);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_pci_p2pdma_supported(dev: *mut device) -> bool {
    let mut ops = get_dma_ops(dev);
//
// Note: dma_ops_bypass is not checked here because P2PDMA should
// not be used with dma mapping ops that do not have support even
// if the specific device is bypassing them.
//
// if ops is not set, dma direct and default IOMMU support P2PDMA
    return !ops;
    }
    EXPORT_SYMBOL_GPL(dma_pci_p2pdma_supported);
#[no_mangle]
pub unsafe extern "C" fn dma_set_mask(dev: *mut device, mask: u64) -> c_int {
//
// Truncate the mask to the actually supported dma_addr_t width to
// avoid generating unsupportable addresses.
//
    mask = (dma_addr_t)mask;
    if (!dev.dma_mask || !dma_supported(dev, mask)) {
    return -EIO;
    }
    arch_dma_set_mask(dev, mask);
// dev->dma_mask = mask;
    dma_setup_need_sync(dev);
    return 0;
    }
    EXPORT_SYMBOL(dma_set_mask);
#[no_mangle]
pub unsafe extern "C" fn dma_set_coherent_mask(dev: *mut device, mask: u64) -> c_int {
//
// Truncate the mask to the actually supported dma_addr_t width to
// avoid generating unsupportable addresses.
//
    mask = (dma_addr_t)mask;
    if (!dma_supported(dev, mask)) {
    return -EIO;
    }
    dev.coherent_dma_mask = mask;
    return 0;
    }
    EXPORT_SYMBOL(dma_set_coherent_mask);
#[no_mangle]
unsafe extern "C" fn __dma_addressing_limited(dev: *mut device) -> bool {
    let mut ops = get_dma_ops(dev);
    if (min_not_zero(dma_get_mask(dev), dev.bus_dma_limit) <
    dma_get_required_mask(dev)) {
    return true;
    }
    if (unlikely(ops) || use_dma_iommu(dev)) {
    return false;
    }
    return !dma_direct_all_ram_mapped(dev);
    }
//
// dma_addressing_limited - return if the device is addressing limited
// @dev:	device to check
//
// Return %true if the devices DMA mask is too small to address all memory in
// the system, else %false.  Lack of addressing bits is the prime reason for
// bounce buffering, but might not be the only one.
//
#[no_mangle]
pub unsafe extern "C" fn dma_addressing_limited(dev: *mut device) -> bool {
    if (!__dma_addressing_limited(dev)) {
    return false;
    }
    dev_dbg(dev, "device is DMA addressing limited\n");
    return true;
    }
    EXPORT_SYMBOL_GPL(dma_addressing_limited);
#[no_mangle]
pub unsafe extern "C" fn dma_max_mapping_size(dev: *mut device) -> usize {
    let mut ops = get_dma_ops(dev);
pub static mut size: usize = 0;
    if (!dev.dma_mask) {
    return 0;
    }
    if (dma_map_direct(dev, ops)) {
    size = dma_direct_max_mapping_size(dev);
    }

    else if (use_dma_iommu(dev)) {
    size = iommu_dma_max_mapping_size(dev);
    }

    else if (ops && ops.max_mapping_size) {
    size = ops.max_mapping_size(dev);
    }
    return size;
    }
    EXPORT_SYMBOL_GPL(dma_max_mapping_size);
#[no_mangle]
pub unsafe extern "C" fn dma_opt_mapping_size(dev: *mut device) -> usize {
    let mut ops = get_dma_ops(dev);
pub static mut size: usize = 0;
    if (use_dma_iommu(dev)) {
    size = iommu_dma_opt_mapping_size();
    }

    else if (ops && ops.opt_mapping_size) {
    size = ops.opt_mapping_size();
    }
    return min(dma_max_mapping_size(dev), size);
    }
    EXPORT_SYMBOL_GPL(dma_opt_mapping_size);
#[no_mangle]
pub unsafe extern "C" fn dma_get_merge_boundary(dev: *mut device) -> c_ulong {
    let mut ops = get_dma_ops(dev);
    if (use_dma_iommu(dev)) {
    return iommu_dma_get_merge_boundary(dev);
    }
    if (!ops || !ops.get_merge_boundary) {
    return 0;	/* can't merge */
    }
    return ops.get_merge_boundary(dev);
    }
    EXPORT_SYMBOL_GPL(dma_get_merge_boundary);