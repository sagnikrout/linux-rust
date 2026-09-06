//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_cppcore.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
//
// nfp_cppcore.c
// Provides low-level access to the NFP's internal CPP bus
// Authors: Jakub Kicinski <jakub.kicinski@netronome.com>
// Jason McMullan <jason.mcmullan@netronome.com>
// Rolf Neugebauer <rolf.neugebauer@netronome.com>
//

pub const NFP_ARM_GCSR_SOFTMODEL2: c_uint = 0x0000014c;
pub const NFP_ARM_GCSR_SOFTMODEL3: c_uint = 0x00000150;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_resource {
    pub list: list_head,
    pub name: *const c_char,
    pub cpp_id: u32,
    pub start: u64,
    pub end: u64,
}

//
// struct nfp_cpp - main nfpcore device structure
// Following fields are read-only after probe() exits or netdevs are spawned.
// @dev:		embedded device structure
// @op:			low-level implementation ops
// @priv:		private data of the low-level implementation
// @model:		chip model
// @interface:		chip interface id we are using to reach it
// @serial:		chip serial number
// @imb_cat_table:	CPP Mapping Table
// @mu_locality_lsb:	MU access type bit offset
//
// Following fields use explicit locking:
// @resource_list:	NFP CPP resource list
// @resource_lock:	protects @resource_list
//
// @area_cache_list:	cached areas for cpp/xpb read/write speed up
// @area_cache_mutex:	protects @area_cache_list
//
// @waitq:		area wait queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp {
    pub dev: device,
    pub priv: *mut c_void,
    pub model: u32,
    pub interface: u16,
    pub serial: [u8; NFP_SERIAL_LEN],
    pub op: *const nfp_cpp_operations,
    pub resource_list: list_head,
    pub resource_lock: rwlock_t,
    pub waitq: wait_queue_head_t,
    pub imb_cat_table: [u32; 16],
    pub mu_locality_lsb: c_uint,
    pub area_cache_mutex: mutex,
    pub area_cache_list: list_head,
}

// Element of the area_cache_list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_area_cache {
    pub entry: list_head,
    pub id: u32,
    pub addr: u64,
    pub size: u32,
    pub area: *mut nfp_cpp_area,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_area {
    pub cpp: *mut nfp_cpp,
    pub kref: kref,
    pub refcount: core::sync::atomic::AtomicI32,
    pub /: *mut *mut mutex mutex; / Lock for the area's refcount,
    pub offset: c_ulonglong,
    pub size: c_ulong,
    pub resource: nfp_cpp_resource,
    pub iomem: *mut void __iomem,
// Here follows the 'priv' part of nfp_cpp_area.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_explicit {
    pub cpp: *mut nfp_cpp,
    pub cmd: nfp_cpp_explicit_command,
// Here follows the 'priv' part of nfp_cpp_area.
}

#[no_mangle]
unsafe extern "C" fn __resource_add(head: *mut list_head, res: *mut nfp_cpp_resource) {
    static void __resource_add(struct list_head *head, struct nfp_cpp_resource *res)
    {
    struct nfp_cpp_resource *tmp;
    struct list_head *pos;
    list_for_each(pos, head) {
    tmp = container_of(pos, struct nfp_cpp_resource, list);
    if (tmp.cpp_id > res.cpp_id)
    break;
    if (tmp.cpp_id == res.cpp_id && tmp.start > res.start)
    break;
    }
    list_add_tail(&res.list, pos);
    }
#[no_mangle]
unsafe extern "C" fn __resource_del(res: *mut nfp_cpp_resource) {
    static void __resource_del(struct nfp_cpp_resource *res)
    {
    list_del_init(&res.list);
    }
#[no_mangle]
unsafe extern "C" fn __release_cpp_area(kref: *mut kref) {
    static void __release_cpp_area(struct kref *kref)
    {
    struct nfp_cpp_area *area =
    container_of(kref, struct nfp_cpp_area, kref);
    struct nfp_cpp *cpp = nfp_cpp_area_cpp(area);
    if (area.cpp.op.area_cleanup)
    area.cpp.op.area_cleanup(area);
    write_lock(&cpp.resource_lock);
    __resource_del(&area.resource);
    write_unlock(&cpp.resource_lock);
    kfree(area);
    }
#[no_mangle]
unsafe extern "C" fn nfp_cpp_area_put(area: *mut nfp_cpp_area) {
    static void nfp_cpp_area_put(struct nfp_cpp_area *area)
    {
    kref_put(&area.kref, __release_cpp_area);
    }
    static struct nfp_cpp_area *nfp_cpp_area_get(struct nfp_cpp_area *area)
    {
    kref_get(&area.kref);
    return area;
    }
//
// nfp_cpp_free() - free the CPP handle
// @cpp:	CPP handle
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_free(cpp: *mut nfp_cpp) {
    void nfp_cpp_free(struct nfp_cpp *cpp)
    {
    struct nfp_cpp_area_cache *cache, *ctmp;
    struct nfp_cpp_resource *res, *rtmp;
// Remove all caches
    list_for_each_entry_safe(cache, ctmp, &cpp.area_cache_list, entry) {
    list_del(&cache.entry);
    if (cache.id)
    nfp_cpp_area_release(cache.area);
    nfp_cpp_area_free(cache.area);
    kfree(cache);
    }
// There should be no dangling areas at this point
    WARN_ON(!list_empty(&cpp.resource_list));
// .. but if they weren't, try to clean up.
    list_for_each_entry_safe(res, rtmp, &cpp.resource_list, list) {
    struct nfp_cpp_area *area = container_of(res,
    struct nfp_cpp_area,
    resource);
    dev_err(cpp.dev.parent, "Dangling area: %d:%d:%d:0x%0llx-0x%0llx%s%s\n",
    NFP_CPP_ID_TARGET_of(res.cpp_id),
    NFP_CPP_ID_ACTION_of(res.cpp_id),
    NFP_CPP_ID_TOKEN_of(res.cpp_id),
    res.start, res.end,
    res.name ? " " : "",
    res.name ? res.name : "");
    if (area.cpp.op.area_release)
    area.cpp.op.area_release(area);
    __release_cpp_area(&area.kref);
    }
    if (cpp.op.free)
    cpp.op.free(cpp);
    device_unregister(&cpp.dev);
    kfree(cpp);
    }
//
// nfp_cpp_model() - Retrieve the Model ID of the NFP
// @cpp:	NFP CPP handle
//
// Return: NFP CPP Model ID
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_model(cpp: *mut nfp_cpp) -> u32 {
    u32 nfp_cpp_model(struct nfp_cpp *cpp)
    {
    return cpp.model;
    }
//
// nfp_cpp_interface() - Retrieve the Interface ID of the NFP
// @cpp:	NFP CPP handle
//
// Return: NFP CPP Interface ID
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_interface(cpp: *mut nfp_cpp) -> u16 {
    u16 nfp_cpp_interface(struct nfp_cpp *cpp)
    {
    return cpp.interface;
    }
//
// nfp_cpp_serial() - Retrieve the Serial ID of the NFP
// @cpp:	NFP CPP handle
// @serial:	Pointer to NFP serial number
//
// Return:  Length of NFP serial number
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_serial(cpp: *mut nfp_cpp, serial: *const u8) -> c_int {
    int nfp_cpp_serial(struct nfp_cpp *cpp, const u8 **serial)
    {
// serial = &cpp->serial[0];
    return sizeof(cpp.serial);
    }

pub const NFP_IMB_TGTADDRESSMODECFG_ADDRMODE_32_BIT: c_int = 0;

#[no_mangle]
unsafe extern "C" fn nfp_cpp_set_mu_locality_lsb(cpp: *mut nfp_cpp) -> c_int {
    static int nfp_cpp_set_mu_locality_lsb(struct nfp_cpp *cpp)
    {
    unsigned int mode, addr40;
    u32 imbcppat;
    int res;
    imbcppat = cpp.imb_cat_table[NFP_CPP_TARGET_MU];
    mode = NFP_IMB_TGTADDRESSMODECFG_MODE_of(imbcppat);
    addr40 = !!(imbcppat & NFP_IMB_TGTADDRESSMODECFG_ADDRMODE);
    res = nfp_cppat_mu_locality_lsb(mode, addr40);
    if (res < 0)
    return res;
    cpp.mu_locality_lsb = res;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_mu_locality_lsb(cpp: *mut nfp_cpp) -> c_uint {
    unsigned int nfp_cpp_mu_locality_lsb(struct nfp_cpp *cpp)
    {
    return cpp.mu_locality_lsb;
    }
//
// nfp_cpp_area_alloc_with_name() - allocate a new CPP area
// @cpp:	CPP device handle
// @dest:	NFP CPP ID
// @name:	Name of region
// @address:	Address of region
// @size:	Size of region
//
// Allocate and initialize a CPP area structure.  The area must later
// be locked down with an 'acquire' before it can be safely accessed.
//
// NOTE: @address and @size must be 32-bit aligned values.
//
// Return: NFP CPP area handle, or NULL
//
    struct nfp_cpp_area *
    nfp_cpp_area_alloc_with_name(struct nfp_cpp *cpp, u32 dest, const char *name,
    unsigned long long address, unsigned long size)
    {
    struct nfp_cpp_area *area;
    let mut tmp64: u64 = address;
    int err, name_len;
// Remap from cpp_island to cpp_target
    err = nfp_target_cpp(dest, tmp64, &dest, &tmp64, cpp.imb_cat_table);
    if (err < 0)
    return core::ptr::null_mut();
    address = tmp64;
    if (!name)
    name = "(reserved)";
    name_len = strlen(name) + 1;
    area = kzalloc(sizeof(*area) + cpp.op.area_priv_size + name_len,
    GFP_KERNEL);
    if (!area)
    return core::ptr::null_mut();
    area.cpp = cpp;
    area.resource.name = (void *)area + sizeof(*area) +
    cpp.op.area_priv_size;
    memcpy((char *)area.resource.name, name, name_len);
    area.resource.cpp_id = dest;
    area.resource.start = address;
    area.resource.end = area.resource.start + size - 1;
    INIT_LIST_HEAD(&area.resource.list);
    atomic_set(&area.refcount, 0);
    kref_init(&area.kref);
    mutex_init(&area.mutex);
    if (cpp.op.area_init) {
    int err;
    err = cpp.op.area_init(area, dest, address, size);
    if (err < 0) {
    kfree(area);
    return core::ptr::null_mut();
    }
    }
    write_lock(&cpp.resource_lock);
    __resource_add(&cpp.resource_list, &area.resource);
    write_unlock(&cpp.resource_lock);
    area.offset = address;
    area.size = size;
    return area;
    }
//
// nfp_cpp_area_alloc() - allocate a new CPP area
// @cpp:	CPP handle
// @dest:	CPP id
// @address:	Start address on CPP target
// @size:	Size of area in bytes
//
// Allocate and initialize a CPP area structure.  The area must later
// be locked down with an 'acquire' before it can be safely accessed.
//
// NOTE: @address and @size must be 32-bit aligned values.
//
// Return: NFP CPP Area handle, or NULL
//
    struct nfp_cpp_area *
    nfp_cpp_area_alloc(struct nfp_cpp *cpp, u32 dest,
    unsigned long long address, unsigned long size)
    {
    return nfp_cpp_area_alloc_with_name(cpp, dest, core::ptr::null_mut(), address, size);
    }
//
// nfp_cpp_area_alloc_acquire() - allocate a new CPP area and lock it down
// @cpp:	CPP handle
// @name:	Name of region
// @dest:	CPP id
// @address:	Start address on CPP target
// @size:	Size of area
//
// Allocate and initialize a CPP area structure, and lock it down so
// that it can be accessed directly.
//
// NOTE: @address and @size must be 32-bit aligned values.
// The area must also be 'released' when the structure is freed.
//
// Return: NFP CPP Area handle, or NULL
//
    struct nfp_cpp_area *
    nfp_cpp_area_alloc_acquire(struct nfp_cpp *cpp, const char *name, u32 dest,
    unsigned long long address, unsigned long size)
    {
    struct nfp_cpp_area *area;
    area = nfp_cpp_area_alloc_with_name(cpp, dest, name, address, size);
    if (!area)
    return core::ptr::null_mut();
    if (nfp_cpp_area_acquire(area)) {
    nfp_cpp_area_free(area);
    return core::ptr::null_mut();
    }
    return area;
    }
//
// nfp_cpp_area_free() - free up the CPP area
// @area:	CPP area handle
//
// Frees up memory resources held by the CPP area.
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_free(area: *mut nfp_cpp_area) {
    void nfp_cpp_area_free(struct nfp_cpp_area *area)
    {
    if (atomic_read(&area.refcount))
    nfp_warn(area.cpp, "Warning: freeing busy area\n");
    nfp_cpp_area_put(area);
    }
#[no_mangle]
unsafe extern "C" fn nfp_cpp_area_acquire_try(area: *mut nfp_cpp_area, status: *mut c_int) -> bool {
    static bool nfp_cpp_area_acquire_try(struct nfp_cpp_area *area, int *status)
    {
// status = area->cpp->op->area_acquire(area);
    return *status != -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn __nfp_cpp_area_acquire(area: *mut nfp_cpp_area) -> c_int {
    static int __nfp_cpp_area_acquire(struct nfp_cpp_area *area)
    {
    int err, status;
    if (atomic_inc_return(&area.refcount) > 1)
    return 0;
    if (!area.cpp.op.area_acquire)
    return 0;
    err = wait_event_interruptible(area.cpp.waitq,
    nfp_cpp_area_acquire_try(area, &status));
    if (!err)
    err = status;
    if (err) {
    nfp_warn(area.cpp, "Warning: area wait failed: %d\n", err);
    atomic_dec(&area.refcount);
    return err;
    }
    nfp_cpp_area_get(area);
    return 0;
    }
//
// nfp_cpp_area_acquire() - lock down a CPP area for access
// @area:	CPP area handle
//
// Locks down the CPP area for a potential long term activity.  Area
// must always be locked down before being accessed.
//
// Return: 0, or -ERRNO
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_acquire(area: *mut nfp_cpp_area) -> c_int {
    int nfp_cpp_area_acquire(struct nfp_cpp_area *area)
    {
    int ret;
    mutex_lock(&area.mutex);
    ret = __nfp_cpp_area_acquire(area);
    mutex_unlock(&area.mutex);
    return ret;
    }
//
// nfp_cpp_area_acquire_nonblocking() - lock down a CPP area for access
// @area:	CPP area handle
//
// Locks down the CPP area for a potential long term activity.  Area
// must always be locked down before being accessed.
//
// NOTE: Returns -EAGAIN is no area is available
//
// Return: 0, or -ERRNO
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_acquire_nonblocking(area: *mut nfp_cpp_area) -> c_int {
    int nfp_cpp_area_acquire_nonblocking(struct nfp_cpp_area *area)
    {
    mutex_lock(&area.mutex);
    if (atomic_inc_return(&area.refcount) == 1) {
    if (area.cpp.op.area_acquire) {
    int err;
    err = area.cpp.op.area_acquire(area);
    if (err < 0) {
    atomic_dec(&area.refcount);
    mutex_unlock(&area.mutex);
    return err;
    }
    }
    }
    mutex_unlock(&area.mutex);
    nfp_cpp_area_get(area);
    return 0;
    }
//
// nfp_cpp_area_release() - release a locked down CPP area
// @area:	CPP area handle
//
// Releases a previously locked down CPP area.
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_release(area: *mut nfp_cpp_area) {
    void nfp_cpp_area_release(struct nfp_cpp_area *area)
    {
    mutex_lock(&area.mutex);
// Only call the release on refcount == 0
    if (atomic_dec_and_test(&area.refcount)) {
    if (area.cpp.op.area_release) {
    area.cpp.op.area_release(area);
// Let anyone waiting for a BAR try to get one..
    wake_up_interruptible_all(&area.cpp.waitq);
    }
    }
    mutex_unlock(&area.mutex);
    nfp_cpp_area_put(area);
    }
//
// nfp_cpp_area_release_free() - release CPP area and free it
// @area:	CPP area handle
//
// Releases CPP area and frees up memory resources held by the it.
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_release_free(area: *mut nfp_cpp_area) {
    void nfp_cpp_area_release_free(struct nfp_cpp_area *area)
    {
    nfp_cpp_area_release(area);
    nfp_cpp_area_free(area);
    }
//
// nfp_cpp_area_read() - read data from CPP area
// @area:	  CPP area handle
// @offset:	  offset into CPP area
// @kernel_vaddr: kernel address to put data into
// @length:	  number of bytes to read
//
// Read data from indicated CPP region.
//
// NOTE: @offset and @length must be 32-bit aligned values.
// Area must have been locked down with an 'acquire'.
//
// Return: length of io, or -ERRNO
//
    int nfp_cpp_area_read(struct nfp_cpp_area *area,
    unsigned long offset, void *kernel_vaddr,
    size_t length)
    {
    return area.cpp.op.area_read(area, kernel_vaddr, offset, length);
    }
//
// nfp_cpp_area_write() - write data to CPP area
// @area:	CPP area handle
// @offset:	offset into CPP area
// @kernel_vaddr: kernel address to read data from
// @length:	number of bytes to write
//
// Write data to indicated CPP region.
//
// NOTE: @offset and @length must be 32-bit aligned values.
// Area must have been locked down with an 'acquire'.
//
// Return: length of io, or -ERRNO
//
    int nfp_cpp_area_write(struct nfp_cpp_area *area,
    unsigned long offset, const void *kernel_vaddr,
    size_t length)
    {
    return area.cpp.op.area_write(area, kernel_vaddr, offset, length);
    }
//
// nfp_cpp_area_size() - return size of a CPP area
// @cpp_area:	CPP area handle
//
// Return: Size of the area
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_size(cpp_area: *mut nfp_cpp_area) -> usize {
    size_t nfp_cpp_area_size(struct nfp_cpp_area *cpp_area)
    {
    return cpp_area.size;
    }
//
// nfp_cpp_area_name() - return name of a CPP area
// @cpp_area:	CPP area handle
//
// Return: Name of the area, or NULL
//
    const char *nfp_cpp_area_name(struct nfp_cpp_area *cpp_area)
    {
    return cpp_area.resource.name;
    }
//
// nfp_cpp_area_priv() - return private struct for CPP area
// @cpp_area:	CPP area handle
//
// Return: Private data for the CPP area
//
    void *nfp_cpp_area_priv(struct nfp_cpp_area *cpp_area)
    {
    return &cpp_area[1];
    }
//
// nfp_cpp_area_cpp() - return CPP handle for CPP area
// @cpp_area:	CPP area handle
//
// Return: NFP CPP handle
//
    struct nfp_cpp *nfp_cpp_area_cpp(struct nfp_cpp_area *cpp_area)
    {
    return cpp_area.cpp;
    }
//
// nfp_cpp_area_resource() - get resource
// @area:	CPP area handle
//
// NOTE: Area must have been locked down with an 'acquire'.
//
// Return: struct resource pointer, or NULL
//
    struct resource *nfp_cpp_area_resource(struct nfp_cpp_area *area)
    {
    struct resource *res = core::ptr::null_mut();
    if (area.cpp.op.area_resource)
    res = area.cpp.op.area_resource(area);
    return res;
    }
//
// nfp_cpp_area_phys() - get physical address of CPP area
// @area:	CPP area handle
//
// NOTE: Area must have been locked down with an 'acquire'.
//
// Return: phy_addr_t of the area, or NULL
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_phys(area: *mut nfp_cpp_area) -> phys_addr_t {
    phys_addr_t nfp_cpp_area_phys(struct nfp_cpp_area *area)
    {
    let mut addr: phys_addr_t = ~0;
    if (area.cpp.op.area_phys)
    addr = area.cpp.op.area_phys(area);
    return addr;
    }
//
// nfp_cpp_area_iomem() - get IOMEM region for CPP area
// @area:	CPP area handle
//
// Returns an iomem pointer for use with readl()/writel() style
// operations.
//
// NOTE: Area must have been locked down with an 'acquire'.
//
// Return: __iomem pointer to the area, or NULL
//
    void __iomem *nfp_cpp_area_iomem(struct nfp_cpp_area *area)
    {
    void __iomem *iomem = core::ptr::null_mut();
    if (area.cpp.op.area_iomem)
    iomem = area.cpp.op.area_iomem(area);
    return iomem;
    }
//
// nfp_cpp_area_readl() - Read a u32 word from an area
// @area:	CPP Area handle
// @offset:	Offset into area
// @value:	Pointer to read buffer
//
// Return: 0 on success, or -ERRNO
//
    int nfp_cpp_area_readl(struct nfp_cpp_area *area,
    unsigned long offset, u32 *value)
    {
    u8 tmp[4];
    int n;
    n = nfp_cpp_area_read(area, offset, &tmp, sizeof(tmp));
    if (n != sizeof(tmp))
    return n < 0 ? n : -EIO;
// value = get_unaligned_le32(tmp);
    return 0;
    }
//
// nfp_cpp_area_writel() - Write a u32 word to an area
// @area:	CPP Area handle
// @offset:	Offset into area
// @value:	Value to write
//
// Return: 0 on success, or -ERRNO
//
    int nfp_cpp_area_writel(struct nfp_cpp_area *area,
    unsigned long offset, u32 value)
    {
    u8 tmp[4];
    int n;
    put_unaligned_le32(value, tmp);
    n = nfp_cpp_area_write(area, offset, &tmp, sizeof(tmp));
    let mut n: return = = sizeof(tmp) ? 0 : n < 0 ? n : -EIO;
    }
//
// nfp_cpp_area_readq() - Read a u64 word from an area
// @area:	CPP Area handle
// @offset:	Offset into area
// @value:	Pointer to read buffer
//
// Return: 0 on success, or -ERRNO
//
    int nfp_cpp_area_readq(struct nfp_cpp_area *area,
    unsigned long offset, u64 *value)
    {
    u8 tmp[8];
    int n;
    n = nfp_cpp_area_read(area, offset, &tmp, sizeof(tmp));
    if (n != sizeof(tmp))
    return n < 0 ? n : -EIO;
// value = get_unaligned_le64(tmp);
    return 0;
    }
//
// nfp_cpp_area_writeq() - Write a u64 word to an area
// @area:	CPP Area handle
// @offset:	Offset into area
// @value:	Value to write
//
// Return: 0 on success, or -ERRNO
//
    int nfp_cpp_area_writeq(struct nfp_cpp_area *area,
    unsigned long offset, u64 value)
    {
    u8 tmp[8];
    int n;
    put_unaligned_le64(value, tmp);
    n = nfp_cpp_area_write(area, offset, &tmp, sizeof(tmp));
    let mut n: return = = sizeof(tmp) ? 0 : n < 0 ? n : -EIO;
    }
//
// nfp_cpp_area_fill() - fill a CPP area with a value
// @area:	CPP area
// @offset:	offset into CPP area
// @value:	value to fill with
// @length:	length of area to fill
//
// Fill indicated area with given value.
//
// Return: length of io, or -ERRNO
//
    int nfp_cpp_area_fill(struct nfp_cpp_area *area,
    unsigned long offset, u32 value, size_t length)
    {
    u8 tmp[4];
    size_t i;
    int k;
    put_unaligned_le32(value, tmp);
    if (offset % sizeof(tmp) || length % sizeof(tmp))
    return -EINVAL;
    for (i = 0; i < length; i += sizeof(tmp)) {
    k = nfp_cpp_area_write(area, offset + i, &tmp, sizeof(tmp));
    if (k < 0)
    return k;
    }
    return i;
    }
//
// nfp_cpp_area_cache_add() - Permanently reserve and area for the hot cache
// @cpp:	NFP CPP handle
// @size:	Size of the area - MUST BE A POWER OF 2.
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_area_cache_add(cpp: *mut nfp_cpp, size: usize) -> c_int {
    int nfp_cpp_area_cache_add(struct nfp_cpp *cpp, size_t size)
    {
    struct nfp_cpp_area_cache *cache;
    struct nfp_cpp_area *area;
// Allocate an area - we use the MU target's base as a placeholder,
// as all supported chips have a MU.
//
    area = nfp_cpp_area_alloc(cpp, NFP_CPP_ID(7, NFP_CPP_ACTION_RW, 0),
    0, size);
    if (!area)
    return -ENOMEM;
    cache = kzalloc_obj(*cache);
    if (!cache) {
    nfp_cpp_area_free(area);
    return -ENOMEM;
    }
    cache.id = 0;
    cache.addr = 0;
    cache.size = size;
    cache.area = area;
    mutex_lock(&cpp.area_cache_mutex);
    list_add_tail(&cache.entry, &cpp.area_cache_list);
    mutex_unlock(&cpp.area_cache_mutex);
    return 0;
    }
    static struct nfp_cpp_area_cache *
    area_cache_get(struct nfp_cpp *cpp, u32 id,
    u64 addr, unsigned long *offset, size_t length)
    {
    struct nfp_cpp_area_cache *cache;
    int err;
// Early exit when length == 0, which prevents
// the need for special case code below when
// checking against available cache size.
//
    if (length == 0 || id == 0)
    return core::ptr::null_mut();
// Remap from cpp_island to cpp_target
    err = nfp_target_cpp(id, addr, &id, &addr, cpp.imb_cat_table);
    if (err < 0)
    return core::ptr::null_mut();
    mutex_lock(&cpp.area_cache_mutex);
    if (list_empty(&cpp.area_cache_list)) {
    mutex_unlock(&cpp.area_cache_mutex);
    return core::ptr::null_mut();
    }
    addr += *offset;
// See if we have a match
    list_for_each_entry(cache, &cpp.area_cache_list, entry) {
    if (id == cache.id &&
    addr >= cache.addr &&
    addr + length <= cache.addr + cache.size)
    goto exit;
    }
// No matches - inspect the tail of the LRU
    cache = list_entry(cpp.area_cache_list.prev,
    struct nfp_cpp_area_cache, entry);
// Can we fit in the cache entry?
    if (round_down(addr + length - 1, cache.size) !=
    round_down(addr, cache.size)) {
    mutex_unlock(&cpp.area_cache_mutex);
    return core::ptr::null_mut();
    }
// If id != 0, we will need to release it
    if (cache.id) {
    nfp_cpp_area_release(cache.area);
    cache.id = 0;
    cache.addr = 0;
    }
// Adjust the start address to be cache size aligned
    cache.addr = addr & ~(u64)(cache.size - 1);
// Re-init to the new ID and address
    if (cpp.op.area_init) {
    err = cpp.op.area_init(cache.area,
    id, cache.addr, cache.size);
    if (err < 0) {
    mutex_unlock(&cpp.area_cache_mutex);
    return core::ptr::null_mut();
    }
    }
// Attempt to acquire
    err = nfp_cpp_area_acquire(cache.area);
    if (err < 0) {
    mutex_unlock(&cpp.area_cache_mutex);
    return core::ptr::null_mut();
    }
    cache.id = id;
    exit:
// Adjust offset
// offset = addr - cache->addr;
    return cache;
    }
    static void
    area_cache_put(struct nfp_cpp *cpp, struct nfp_cpp_area_cache *cache)
    {
    if (!cache)
    return;
// Move to front of LRU
    list_move(&cache.entry, &cpp.area_cache_list);
    mutex_unlock(&cpp.area_cache_mutex);
    }
    static int __nfp_cpp_read(struct nfp_cpp *cpp, u32 destination,
    unsigned long long address, void *kernel_vaddr,
    size_t length)
    {
    struct nfp_cpp_area_cache *cache;
    struct nfp_cpp_area *area;
    let mut offset: c_ulong = 0;
    int err;
    cache = area_cache_get(cpp, destination, address, &offset, length);
    if (cache) {
    area = cache.area;
    } else {
    area = nfp_cpp_area_alloc(cpp, destination, address, length);
    if (!area)
    return -ENOMEM;
    err = nfp_cpp_area_acquire(area);
    if (err) {
    nfp_cpp_area_free(area);
    return err;
    }
    }
    err = nfp_cpp_area_read(area, offset, kernel_vaddr, length);
    if (cache)
    area_cache_put(cpp, cache);
    else
    nfp_cpp_area_release_free(area);
    return err;
    }
//
// nfp_cpp_read() - read from CPP target
// @cpp:		CPP handle
// @destination:	CPP id
// @address:		offset into CPP target
// @kernel_vaddr:	kernel buffer for result
// @length:		number of bytes to read
//
// Return: length of io, or -ERRNO
//
    int nfp_cpp_read(struct nfp_cpp *cpp, u32 destination,
    unsigned long long address, void *kernel_vaddr,
    size_t length)
    {
    size_t n, offset;
    int ret;
    for (offset = 0; offset < length; offset += n) {
    let mut r_addr: c_ulonglong = address + offset;
// make first read smaller to align to safe window
    n = min_t(size_t, length - offset,
    ALIGN(r_addr + 1, NFP_CPP_SAFE_AREA_SIZE) - r_addr);
    ret = __nfp_cpp_read(cpp, destination, address + offset,
    kernel_vaddr + offset, n);
    if (ret < 0)
    return ret;
    if (ret != n)
    return offset + n;
    }
    return length;
    }
    static int __nfp_cpp_write(struct nfp_cpp *cpp, u32 destination,
    unsigned long long address,
    const void *kernel_vaddr, size_t length)
    {
    struct nfp_cpp_area_cache *cache;
    struct nfp_cpp_area *area;
    let mut offset: c_ulong = 0;
    int err;
    cache = area_cache_get(cpp, destination, address, &offset, length);
    if (cache) {
    area = cache.area;
    } else {
    area = nfp_cpp_area_alloc(cpp, destination, address, length);
    if (!area)
    return -ENOMEM;
    err = nfp_cpp_area_acquire(area);
    if (err) {
    nfp_cpp_area_free(area);
    return err;
    }
    }
    err = nfp_cpp_area_write(area, offset, kernel_vaddr, length);
    if (cache)
    area_cache_put(cpp, cache);
    else
    nfp_cpp_area_release_free(area);
    return err;
    }
//
// nfp_cpp_write() - write to CPP target
// @cpp:		CPP handle
// @destination:	CPP id
// @address:		offset into CPP target
// @kernel_vaddr:	kernel buffer to read from
// @length:		number of bytes to write
//
// Return: length of io, or -ERRNO
//
    int nfp_cpp_write(struct nfp_cpp *cpp, u32 destination,
    unsigned long long address,
    const void *kernel_vaddr, size_t length)
    {
    size_t n, offset;
    int ret;
    for (offset = 0; offset < length; offset += n) {
    let mut w_addr: c_ulonglong = address + offset;
// make first write smaller to align to safe window
    n = min_t(size_t, length - offset,
    ALIGN(w_addr + 1, NFP_CPP_SAFE_AREA_SIZE) - w_addr);
    ret = __nfp_cpp_write(cpp, destination, address + offset,
    kernel_vaddr + offset, n);
    if (ret < 0)
    return ret;
    if (ret != n)
    return offset + n;
    }
    return length;
    }
// Return the correct CPP address, and fixup xpb_addr as needed.
#[no_mangle]
unsafe extern "C" fn nfp_xpb_to_cpp(cpp: *mut nfp_cpp, xpb_addr: *mut u32) -> u32 {
    static u32 nfp_xpb_to_cpp(struct nfp_cpp *cpp, u32 *xpb_addr)
    {
    int island;
    u32 xpb;
    xpb = NFP_CPP_ID(14, NFP_CPP_ACTION_RW, 0);
// Ensure that non-local XPB accesses go
// out through the global XPBM bus.
//
    island = (*xpb_addr >> 24) & 0x3f;
    if (!island)
    return xpb;
    if (island != 1) {
// xpb_addr |= 1 << 30;
    return xpb;
    }
// Accesses to the ARM Island overlay uses Island 0 / Global Bit
// xpb_addr &= ~0x7f000000;
    if (*xpb_addr < 0x60000) {
// xpb_addr |= 1 << 30;
    } else {
// And only non-ARM interfaces use the island id = 1
    if (NFP_CPP_INTERFACE_TYPE_of(nfp_cpp_interface(cpp))
    != NFP_CPP_INTERFACE_TYPE_ARM)
// xpb_addr |= 1 << 24;
    }
    return xpb;
    }
//
// nfp_xpb_readl() - Read a u32 word from a XPB location
// @cpp:	CPP device handle
// @xpb_addr:	Address for operation
// @value:	Pointer to read buffer
//
// Return: 0 on success, or -ERRNO
//
#[no_mangle]
pub unsafe extern "C" fn nfp_xpb_readl(cpp: *mut nfp_cpp, xpb_addr: u32, value: *mut u32) -> c_int {
    int nfp_xpb_readl(struct nfp_cpp *cpp, u32 xpb_addr, u32 *value)
    {
    let mut cpp_dest: u32 = nfp_xpb_to_cpp(cpp, &xpb_addr);
    return nfp_cpp_readl(cpp, cpp_dest, xpb_addr, value);
    }
//
// nfp_xpb_writel() - Write a u32 word to a XPB location
// @cpp:	CPP device handle
// @xpb_addr:	Address for operation
// @value:	Value to write
//
// Return: 0 on success, or -ERRNO
//
#[no_mangle]
pub unsafe extern "C" fn nfp_xpb_writel(cpp: *mut nfp_cpp, xpb_addr: u32, value: u32) -> c_int {
    int nfp_xpb_writel(struct nfp_cpp *cpp, u32 xpb_addr, u32 value)
    {
    let mut cpp_dest: u32 = nfp_xpb_to_cpp(cpp, &xpb_addr);
    return nfp_cpp_writel(cpp, cpp_dest, xpb_addr, value);
    }
//
// nfp_xpb_writelm() - Modify bits of a 32-bit value from the XPB bus
// @cpp:	NFP CPP device handle
// @xpb_tgt:	XPB target and address
// @mask:	mask of bits to alter
// @value:	value to modify
//
// KERNEL: This operation is safe to call in interrupt or softirq context.
//
// Return: 0 on success, or -ERRNO
//
    int nfp_xpb_writelm(struct nfp_cpp *cpp, u32 xpb_tgt,
    u32 mask, u32 value)
    {
    int err;
    u32 tmp;
    err = nfp_xpb_readl(cpp, xpb_tgt, &tmp);
    if (err < 0)
    return err;
    tmp &= ~mask;
    tmp |= mask & value;
    return nfp_xpb_writel(cpp, xpb_tgt, tmp);
    }
// Lockdep markers
    static struct lock_class_key nfp_cpp_resource_lock_key;
#[no_mangle]
unsafe extern "C" fn nfp_cpp_dev_release(dev: *mut device) {
    static void nfp_cpp_dev_release(struct device *dev)
    {
// Nothing to do here - it just makes the kernel happy
    }
//
// nfp_cpp_from_operations() - Create a NFP CPP handle
// from an operations structure
// @ops:	NFP CPP operations structure
// @parent:	Parent device
// @priv:	Private data of low-level implementation
//
// NOTE: On failure, cpp_ops->free will be called!
//
// Return: NFP CPP handle on success, ERR_PTR on failure
//
    struct nfp_cpp *
    nfp_cpp_from_operations(const struct nfp_cpp_operations *ops,
    struct device *parent, void *priv)
    {
    let mut arm: u32 = NFP_CPP_ID(NFP_CPP_TARGET_ARM, NFP_CPP_ACTION_RW, 0);
    struct nfp_cpp *cpp;
    int ifc, err;
    u32 mask[2];
    u32 xpbaddr;
    size_t tgt;
    cpp = kzalloc_obj(*cpp);
    if (!cpp) {
    err = -ENOMEM;
    goto err_malloc;
    }
    cpp.op = ops;
    cpp.priv = priv;
    ifc = ops.get_interface(parent);
    if (ifc < 0) {
    err = ifc;
    goto err_free_cpp;
    }
    cpp.interface = ifc;
    if (ops.read_serial) {
    err = ops.read_serial(parent, cpp.serial);
    if (err)
    goto err_free_cpp;
    }
    rwlock_init(&cpp.resource_lock);
    init_waitqueue_head(&cpp.waitq);
    lockdep_set_class(&cpp.resource_lock, &nfp_cpp_resource_lock_key);
    INIT_LIST_HEAD(&cpp.resource_list);
    INIT_LIST_HEAD(&cpp.area_cache_list);
    mutex_init(&cpp.area_cache_mutex);
    cpp.dev.init_name = "cpp";
    cpp.dev.parent = parent;
    cpp.dev.release = nfp_cpp_dev_release;
    err = device_register(&cpp.dev);
    if (err < 0) {
    put_device(&cpp.dev);
    goto err_free_cpp;
    }
    dev_set_drvdata(&cpp.dev, cpp);
// NOTE: cpp_lock is NOT locked for op->init,
// since it may call NFP CPP API operations
//
    if (cpp.op.init) {
    err = cpp.op.init(cpp);
    if (err < 0) {
    dev_err(parent,
    "NFP interface initialization failed\n");
    goto err_out;
    }
    }
    err = nfp_cpp_model_autodetect(cpp, &cpp.model);
    if (err < 0) {
    dev_err(parent, "NFP model detection failed\n");
    goto err_out;
    }
    for (tgt = 0; tgt < ARRAY_SIZE(cpp.imb_cat_table); tgt++) {
// Hardcoded XPB IMB Base, island 0
    xpbaddr = 0x000a0000 + (tgt * 4);
    err = nfp_xpb_readl(cpp, xpbaddr,
    &cpp.imb_cat_table[tgt]);
    if (err < 0) {
    dev_err(parent,
    "Can't read CPP mapping from device\n");
    goto err_out;
    }
    }
    nfp_cpp_readl(cpp, arm, NFP_ARM_GCSR + NFP_ARM_GCSR_SOFTMODEL2,
    &mask[0]);
    nfp_cpp_readl(cpp, arm, NFP_ARM_GCSR + NFP_ARM_GCSR_SOFTMODEL3,
    &mask[1]);
    err = nfp_cpp_set_mu_locality_lsb(cpp);
    if (err < 0) {
    dev_err(parent,	"Can't calculate MU locality bit offset\n");
    goto err_out;
    }
    dev_info(cpp.dev.parent, "Model: 0x%08x, SN: %pM, Ifc: 0x%04x\n",
    nfp_cpp_model(cpp), cpp.serial, nfp_cpp_interface(cpp));
    return cpp;
    err_out:
    device_unregister(&cpp.dev);
    err_free_cpp:
    kfree(cpp);
    err_malloc:
    return ERR_PTR(err);
    }
//
// nfp_cpp_priv() - Get the operations private data of a CPP handle
// @cpp:	CPP handle
//
// Return: Private data for the NFP CPP handle
//
    void *nfp_cpp_priv(struct nfp_cpp *cpp)
    {
    return cpp.priv;
    }
//
// nfp_cpp_device() - Get the Linux device handle of a CPP handle
// @cpp:	CPP handle
//
// Return: Device for the NFP CPP bus
//
    struct device *nfp_cpp_device(struct nfp_cpp *cpp)
    {
    return &cpp.dev;
    }

    ({							  \
    struct nfp_cpp *cpp = nfp_cpp_explicit_cpp(expl); \
    int err = -ENODEV;				  \
    \
    if (cpp.op.func)				  \
    err = cpp.op.func(expl, ##args);	  \
    err;						  \
    })

    ({							  \
    struct nfp_cpp *cpp = nfp_cpp_explicit_cpp(expl); \
    \
    if (cpp.op.func)				  \
    cpp.op.func(expl, ##args);		  \
    \
    })
//
// nfp_cpp_explicit_acquire() - Acquire explicit access handle
// @cpp:	NFP CPP handle
//
// The 'data_ref' and 'signal_ref' values are useful when
// constructing the NFP_EXPL_CSR1 and NFP_EXPL_POST values.
//
// Return: NFP CPP explicit handle
//
    struct nfp_cpp_explicit *nfp_cpp_explicit_acquire(struct nfp_cpp *cpp)
    {
    struct nfp_cpp_explicit *expl;
    int err;
    expl = kzalloc(sizeof(*expl) + cpp.op.explicit_priv_size, GFP_KERNEL);
    if (!expl)
    return core::ptr::null_mut();
    expl.cpp = cpp;
    err = NFP_EXPL_OP(explicit_acquire, expl);
    if (err < 0) {
    kfree(expl);
    return core::ptr::null_mut();
    }
    return expl;
    }
//
// nfp_cpp_explicit_set_target() - Set target fields for explicit
// @expl:	Explicit handle
// @cpp_id:	CPP ID field
// @len:	CPP Length field
// @mask:	CPP Mask field
//
// Return: 0, or -ERRNO
//
    int nfp_cpp_explicit_set_target(struct nfp_cpp_explicit *expl,
    u32 cpp_id, u8 len, u8 mask)
    {
    expl.cmd.cpp_id = cpp_id;
    expl.cmd.len = len;
    expl.cmd.byte_mask = mask;
    return 0;
    }
//
// nfp_cpp_explicit_set_data() - Set data fields for explicit
// @expl:	Explicit handle
// @data_master: CPP Data Master field
// @data_ref:	CPP Data Ref field
//
// Return: 0, or -ERRNO
//
    int nfp_cpp_explicit_set_data(struct nfp_cpp_explicit *expl,
    u8 data_master, u16 data_ref)
    {
    expl.cmd.data_master = data_master;
    expl.cmd.data_ref = data_ref;
    return 0;
    }
//
// nfp_cpp_explicit_set_signal() - Set signal fields for explicit
// @expl:	Explicit handle
// @signal_master: CPP Signal Master field
// @signal_ref:	CPP Signal Ref field
//
// Return: 0, or -ERRNO
//
    int nfp_cpp_explicit_set_signal(struct nfp_cpp_explicit *expl,
    u8 signal_master, u8 signal_ref)
    {
    expl.cmd.signal_master = signal_master;
    expl.cmd.signal_ref = signal_ref;
    return 0;
    }
//
// nfp_cpp_explicit_set_posted() - Set completion fields for explicit
// @expl:	Explicit handle
// @posted:	True for signaled completion, false otherwise
// @siga:	CPP Signal A field
// @siga_mode:	CPP Signal A Mode field
// @sigb:	CPP Signal B field
// @sigb_mode:	CPP Signal B Mode field
//
// Return: 0, or -ERRNO
//
    int nfp_cpp_explicit_set_posted(struct nfp_cpp_explicit *expl, int posted,
    u8 siga,
    enum nfp_cpp_explicit_signal_mode siga_mode,
    u8 sigb,
    enum nfp_cpp_explicit_signal_mode sigb_mode)
    {
    expl.cmd.posted = posted;
    expl.cmd.siga = siga;
    expl.cmd.sigb = sigb;
    expl.cmd.siga_mode = siga_mode;
    expl.cmd.sigb_mode = sigb_mode;
    return 0;
    }
//
// nfp_cpp_explicit_put() - Set up the write (pull) data for a explicit access
// @expl:	NFP CPP Explicit handle
// @buff:	Data to have the target pull in the transaction
// @len:	Length of data, in bytes
//
// The 'len' parameter must be less than or equal to 128 bytes.
//
// If this function is called before the configuration
// registers are set, it will return -EINVAL.
//
// Return: 0, or -ERRNO
//
    int nfp_cpp_explicit_put(struct nfp_cpp_explicit *expl,
    const void *buff, size_t len)
    {
    return NFP_EXPL_OP(explicit_put, expl, buff, len);
    }
//
// nfp_cpp_explicit_do() - Execute a transaction, and wait for it to complete
// @expl:	NFP CPP Explicit handle
// @address:	Address to send in the explicit transaction
//
// If this function is called before the configuration
// registers are set, it will return -1, with an errno of EINVAL.
//
// Return: 0, or -ERRNO
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_explicit_do(expl: *mut nfp_cpp_explicit, address: u64) -> c_int {
    int nfp_cpp_explicit_do(struct nfp_cpp_explicit *expl, u64 address)
    {
    return NFP_EXPL_OP(explicit_do, expl, &expl.cmd, address);
    }
//
// nfp_cpp_explicit_get() - Get the 'push' (read) data from a explicit access
// @expl:	NFP CPP Explicit handle
// @buff:	Data that the target pushed in the transaction
// @len:	Length of data, in bytes
//
// The 'len' parameter must be less than or equal to 128 bytes.
//
// If this function is called before all three configuration
// registers are set, it will return -1, with an errno of EINVAL.
//
// If this function is called before nfp_cpp_explicit_do()
// has completed, it will return -1, with an errno of EBUSY.
//
// Return: 0, or -ERRNO
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_explicit_get(expl: *mut nfp_cpp_explicit, buff: *mut c_void, len: usize) -> c_int {
    int nfp_cpp_explicit_get(struct nfp_cpp_explicit *expl, void *buff, size_t len)
    {
    return NFP_EXPL_OP(explicit_get, expl, buff, len);
    }
//
// nfp_cpp_explicit_release() - Release explicit access handle
// @expl:	NFP CPP Explicit handle
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_explicit_release(expl: *mut nfp_cpp_explicit) {
    void nfp_cpp_explicit_release(struct nfp_cpp_explicit *expl)
    {
    NFP_EXPL_OP_NR(explicit_release, expl);
    kfree(expl);
    }
//
// nfp_cpp_explicit_cpp() - return CPP handle for CPP explicit
// @cpp_explicit:	CPP explicit handle
//
// Return: NFP CPP handle of the explicit
//
    struct nfp_cpp *nfp_cpp_explicit_cpp(struct nfp_cpp_explicit *cpp_explicit)
    {
    return cpp_explicit.cpp;
    }
//
// nfp_cpp_explicit_priv() - return private struct for CPP explicit
// @cpp_explicit:	CPP explicit handle
//
// Return: private data of the explicit, or NULL
//
    void *nfp_cpp_explicit_priv(struct nfp_cpp_explicit *cpp_explicit)
    {
    return &cpp_explicit[1];
    }
