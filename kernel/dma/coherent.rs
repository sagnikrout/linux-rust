//! Automatically rewritten from C to Rust
//! Source: kernel/dma/coherent.c
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
// Coherent per-device memory handling.
// Borrowed from i386
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_coherent_mem {
    pub virt_base: *mut c_void,
    pub device_base: dma_addr_t,
    pub pfn_base: c_ulong,
    pub size: c_int,
    pub bitmap: *mut c_ulong,
    pub spinlock: spinlock_t,
    pub use_dev_dma_pfn_offset: bool,
}

#[no_mangle]
pub unsafe extern "C" fn dev_get_coherent_memory(dev: *mut device) -> *mut c_void {
    if (dev && dev.dma_mem) {
    return dev.dma_mem;
    }
    return core::ptr::null_mut();
    }
    static inline dma_addr_t dma_get_device_base(device *dev, dma_coherent_mem *mem)
    {
    if (mem.use_dev_dma_pfn_offset) {
    return phys_to_dma(dev, PFN_PHYS(mem.pfn_base));
    }
    return mem.device_base;
    }
#[no_mangle]
pub unsafe extern "C" fn dma_init_coherent_memory(phys_addr: phys_addr_t, device_addr: dma_addr_t, size: size_t, use_dma_pfn_offset: bool) -> *mut c_void {
pub static mut dma_mem: *mut c_void = core::ptr::null_mut();
pub static mut pages: c_int = 0;
pub static mut mem_base: *mut c_void = core::ptr::null_mut();
    if (!size) {
    return ERR_PTR(-EINVAL);
    }
    mem_base = memremap(phys_addr, size, MEMREMAP_WC);
    if (!mem_base) {
    return ERR_PTR(-EINVAL);
    }
    dma_mem = kzalloc_obj(dma_coherent_mem);
    if (!dma_mem) {
// goto;
    }
    dma_mem.bitmap = bitmap_zalloc(pages, GFP_KERNEL);
    if (!dma_mem.bitmap) {
// goto;
    }
    dma_mem.virt_base = mem_base;
    dma_mem.device_base = device_addr;
    dma_mem.pfn_base = PFN_DOWN(phys_addr);
    dma_mem.size = pages;
    dma_mem.use_dev_dma_pfn_offset = use_dma_pfn_offset;
    spin_lock_init(&dma_mem.spinlock);
    return dma_mem;
// label;
    kfree(dma_mem);
// label;
    memunmap(mem_base);
    pr_err!("Reserved memory: failed to init DMA memory pool at %pa, size %zu KiB\n",
    &phys_addr, size / SZ_1K);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn _dma_release_coherent_memory(mem: *mut dma_coherent_mem) {
    if (!mem) {
    return;
    }
    memunmap(mem.virt_base);
    bitmap_free(mem.bitmap);
    kfree(mem);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_assign_coherent_memory(dev: *mut device, mem: *mut dma_coherent_mem) -> c_int {
    if (!dev) {
    return -ENODEV;
    }
    if (dev.dma_mem) {
    return -EBUSY;
    }
    dev.dma_mem = mem;
    return 0;
    }
//
// Declare a region of memory to be handed out by dma_alloc_coherent() when it
// is asked for coherent memory for this device.  This shall only be used
// from platform code, usually based on the device tree description.
//
// phys_addr is the CPU physical address to which the memory is currently
// assigned (this will be ioremapped so the CPU can access the region).
//
// device_addr is the DMA address the device needs to be programmed with to
// actually address this memory (this will be handed out as the dma_addr_t in
// dma_alloc_coherent()).
//
// size is the size of the area (must be a multiple of PAGE_SIZE).
//
// As a simplification for the platforms, only *one* such region of memory may
// be declared per device.
//
#[no_mangle]
pub unsafe extern "C" fn dma_declare_coherent_memory(dev: *mut device, phys_addr: phys_addr_t, device_addr: dma_addr_t, size: size_t) -> c_int {
pub static mut mem: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    mem = dma_init_coherent_memory(phys_addr, device_addr, size, false);
    if (IS_ERR(mem)) {
    return PTR_ERR(mem);
    }
    ret = dma_assign_coherent_memory(dev, mem);
    if (ret) {
    _dma_release_coherent_memory(mem);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dma_release_coherent_memory(dev: *mut device) {
    if (dev) {
    _dma_release_coherent_memory(dev.dma_mem);
    dev.dma_mem = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __dma_alloc_from_coherent(dev: *mut device, mem: *mut dma_coherent_mem, size: ssize_t, dma_handle: *mut dma_addr_t) -> *mut c_void {
pub static mut order: c_int = 0;
    let mut flags = 0;
    let mut pageno = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
    spin_lock_irqsave(&mem.spinlock, flags);
    if (unlikely(size > ((dma_addr_t)mem.size << PAGE_SHIFT))) {
// goto;
    }
    pageno = bitmap_find_free_region(mem.bitmap, mem.size, order);
    if (unlikely(pageno < 0)) {
// goto;
    }
//
// Memory was found in the coherent area.
//
// dma_handle = dma_get_device_base(dev, mem) +
    ((dma_addr_t)pageno << PAGE_SHIFT);
    ret = mem.virt_base + ((dma_addr_t)pageno << PAGE_SHIFT);
    spin_unlock_irqrestore(&mem.spinlock, flags);
    memset(ret, 0, size);
    return ret;
// label;
    spin_unlock_irqrestore(&mem.spinlock, flags);
    return core::ptr::null_mut();
    }
//
// dma_alloc_from_dev_coherent() - allocate memory from device coherent pool
// @dev:	device from which we allocate memory
// @size:	size of requested memory area
// @dma_handle:	This will be filled with the correct dma handle
// @ret:	This pointer will be filled with the virtual address
// to allocated area.
//
// This function should be only called from per-arch dma_alloc_coherent()
// to support allocation from per-device coherent memory pools.
//
// Returns 0 if dma_alloc_coherent should continue with allocating from
// generic memory areas, or !0 if dma_alloc_coherent should return @ret.
//
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_from_dev_coherent(dev: *mut device, size: ssize_t, dma_handle: *mut dma_addr_t, ret: *mut *mut c_void) -> c_int {
    let mut mem = dev_get_coherent_memory(dev);
    if (!mem) {
    return 0;
    }
// ret = __dma_alloc_from_coherent(dev, mem, size, dma_handle);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __dma_release_from_coherent(mem: *mut dma_coherent_mem, order: c_int, vaddr: *mut c_void) -> c_int {
    if (mem && vaddr >= mem.virt_base && vaddr <
    (mem.virt_base + ((dma_addr_t)mem.size << PAGE_SHIFT))) {
pub static mut page: c_int = 0;
    let mut flags = 0;
    spin_lock_irqsave(&mem.spinlock, flags);
    bitmap_release_region(mem.bitmap, page, order);
    spin_unlock_irqrestore(&mem.spinlock, flags);
    return 1;
    }
    return 0;
    }
//
// dma_release_from_dev_coherent() - free memory to device coherent memory pool
// @dev:	device from which the memory was allocated
// @order:	the order of pages allocated
// @vaddr:	virtual address of allocated pages
//
// This checks whether the memory was allocated from the per-device
// coherent memory pool and if so, releases that memory.
//
// Returns 1 if we correctly released the memory, or 0 if the caller should
// proceed with releasing memory from generic pools.
//
#[no_mangle]
pub unsafe extern "C" fn dma_release_from_dev_coherent(dev: *mut device, order: c_int, vaddr: *mut c_void) -> c_int {
    let mut mem = dev_get_coherent_memory(dev);
    return __dma_release_from_coherent(mem, order, vaddr);
    }
#[no_mangle]
pub unsafe extern "C" fn __dma_mmap_from_coherent(mem: *mut dma_coherent_mem, vma: *mut vm_area_struct, vaddr: *mut c_void, size: size_t, ret: *mut c_int) -> c_int {
    if (mem && vaddr >= mem.virt_base && vaddr + size <=
    (mem.virt_base + ((dma_addr_t)mem.size << PAGE_SHIFT))) {
pub static mut pgoff_start: pgoff_t = 0;
pub static mut pgoff_end: pgoff_t = 0;
pub static mut start: c_int = 0;
pub static mut user_count: c_ulong = 0;
pub static mut count: c_int = 0;
// ret = -ENXIO;
    if (pgoff_start < count && pgoff_end <= count) {
pub static mut pfn: c_ulong = 0;
// ret = remap_pfn_range(vma, vma->vm_start, pfn,
    user_count << PAGE_SHIFT,
    vma.vm_page_prot);
    }
    return 1;
    }
    return 0;
    }
//
// dma_mmap_from_dev_coherent() - mmap memory from the device coherent pool
// @dev:	device from which the memory was allocated
// @vma:	vm_area for the userspace memory
// @vaddr:	cpu address returned by dma_alloc_from_dev_coherent
// @size:	size of the memory buffer allocated
// @ret:	result from remap_pfn_range()
//
// This checks whether the memory was allocated from the per-device
// coherent memory pool and if so, maps that memory to the provided vma.
//
// Returns 1 if @vaddr belongs to the device coherent pool and the caller
// should return @ret, or 0 if they should proceed with mapping memory from
// generic areas.
//
#[no_mangle]
pub unsafe extern "C" fn dma_mmap_from_dev_coherent(dev: *mut device, vma: *mut vm_area_struct, vaddr: *mut c_void, size: size_t, ret: *mut c_int) -> c_int {
    let mut mem = dev_get_coherent_memory(dev);
    return __dma_mmap_from_coherent(mem, vma, vaddr, size, ret);
    }

pub static mut dma_coherent_default_memory: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_from_global_coherent(dev: *mut device, size: ssize_t, dma_handle: *mut dma_addr_t) -> *mut c_void {
    if (!dma_coherent_default_memory) {
    return core::ptr::null_mut();
    }
    return __dma_alloc_from_coherent(dev, dma_coherent_default_memory, size,
    dma_handle);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_release_from_global_coherent(order: c_int, vaddr: *mut c_void) -> c_int {
    if (!dma_coherent_default_memory) {
    return 0;
    }
    return __dma_release_from_coherent(dma_coherent_default_memory, order,
    vaddr);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_mmap_from_global_coherent(vma: *mut vm_area_struct, vaddr: *mut c_void, size: size_t, ret: *mut c_int) -> c_int {
    if (!dma_coherent_default_memory) {
    return 0;
    }
    return __dma_mmap_from_coherent(dma_coherent_default_memory, vma,
    vaddr, size, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_init_global_coherent(phys_addr: phys_addr_t, size: usize) -> c_int {
pub static mut mem: *mut c_void = core::ptr::null_mut();
    mem = dma_init_coherent_memory(phys_addr, phys_addr, size, true);
    if (IS_ERR(mem)) {
    return PTR_ERR(mem);
    }
    dma_coherent_default_memory = mem;
    pr_info!("DMA: default coherent area is set\n");
    return 0;
    }

//
// Support for reserved memory regions defined in device tree
//

    static phys_addr_t dma_reserved_default_memory_base __initdata;
    static phys_addr_t dma_reserved_default_memory_size __initdata;

#[no_mangle]
unsafe extern "C" fn rmem_dma_device_init(rmem: *mut reserved_mem, dev: *mut device) -> c_int {
    let mut mem = rmem.priv;
    if (!mem) {
    mem = dma_init_coherent_memory(rmem.base, rmem.base,
    rmem.size, true);
    if (IS_ERR(mem)) {
    return PTR_ERR(mem);
    }
    rmem.priv = mem;
    }
// Warn if the device potentially can't use the reserved memory
    if (mem.device_base + rmem.size - 1 >
    min_not_zero(dev.coherent_dma_mask, dev.bus_dma_limit)) {
    dev_warn(dev, "reserved memory is beyond device's set DMA address range\n");
    }
    dma_assign_coherent_memory(dev, mem);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rmem_dma_device_release(rmem: *mut reserved_mem, dev: *mut device) {
    if (dev) {
    dev.dma_mem = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn rmem_dma_setup(node: c_ulong, rmem: *mut reserved_mem) -> c_int {
    if (of_get_flat_dt_prop(node, "reusable", core::ptr::null_mut())) {
    return -ENODEV;
    }

    if (!of_get_flat_dt_prop(node, "no-map", core::ptr::null_mut())) {
    pr_err!("Reserved memory: regions without no-map are not yet supported\n");
    return -EINVAL;
    }

    if (of_get_flat_dt_prop(node, "linux,dma-default", core::ptr::null_mut())) {
    WARN(dma_reserved_default_memory_size,
    "Reserved memory: region for default DMA coherent area is redefined\n");
    dma_reserved_default_memory_base = rmem.base;
    dma_reserved_default_memory_size = rmem.size;
    }

    pr_info!("Reserved memory: created DMA memory pool at %pa, size %llu KiB\n",
    &rmem.base, (unsigned long long)(rmem.size / SZ_1K));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dma_init_reserved_memory() -> c_int {
    if (!dma_reserved_default_memory_size) {
    return -ENOMEM;
    }
    return dma_init_global_coherent(dma_reserved_default_memory_base,
    dma_reserved_default_memory_size);
    }
    core_initcall!(dma_init_reserved_memory);

pub static mut reserved_mem_ops: usize = 0;
    RESERVEDMEM_OF_DECLARE(dma, "shared-dma-pool", &rmem_dma_ops);