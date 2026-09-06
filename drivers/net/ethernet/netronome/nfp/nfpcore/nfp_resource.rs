//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_resource.c
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
// nfp_resource.c
// Author: Jakub Kicinski <jakub.kicinski@netronome.com>
// Jason McMullan <jason.mcmullan@netronome.com>
//

pub const NFP_RESOURCE_TBL_BASE: c_uint = 0x8100000000ULL;
// NFP Resource Table self-identifier

pub const NFP_RESOURCE_TBL_KEY: c_uint = 0x00000000 /* Special key for entry 0 */;
pub const NFP_RESOURCE_ENTRY_NAME_SZ: c_int = 8;
//
// struct nfp_resource_entry - Resource table entry
// @mutex:	NFP CPP Lock
// @mutex.owner:	NFP CPP Lock, interface owner
// @mutex.key:		NFP CPP Lock, posix_crc32(name, 8)
// @region:	Memory region descriptor
// @region.name:	ASCII, zero padded name
// @region.reserved:	padding
// @region.cpp_action:	CPP Action
// @region.cpp_token:	CPP Token
// @region.cpp_target:	CPP Target ID
// @region.page_offset:	256-byte page offset into target's CPP address
// @region.page_size:	size, in 256-byte pages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_resource_entry {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_resource_entry_mutex {
    pub owner: u32,
    pub key: u32,
    pub mutex: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_resource_entry_region {
    pub name: [u8; NFP_RESOURCE_ENTRY_NAME_SZ],
    pub reserved: [u8; 5],
    pub cpp_action: u8,
    pub cpp_token: u8,
    pub cpp_target: u8,
    pub page_offset: u32,
    pub page_size: u32,
    pub region: },
}

pub const NFP_RESOURCE_TBL_SIZE: c_int = 4096;

    sizeof(struct nfp_resource_entry))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_resource {
    pub 1]: char name[NFP_RESOURCE_ENTRY_NAME_SZ +,
    pub cpp_id: u32,
    pub addr: u64,
    pub size: u64,
    pub mutex: *mut nfp_cpp_mutex,
}

#[no_mangle]
unsafe extern "C" fn nfp_cpp_resource_find(cpp: *mut nfp_cpp, res: *mut nfp_resource) -> c_int {
    static int nfp_cpp_resource_find(struct nfp_cpp *cpp, struct nfp_resource *res)
    {
    struct nfp_resource_entry entry;
    u32 cpp_id, key;
    int ret, i;
    cpp_id = NFP_CPP_ID(NFP_RESOURCE_TBL_TARGET, 3, 0);  /* Atomic read */
// Search for a matching entry
    if (!strcmp(res.name, NFP_RESOURCE_TBL_NAME)) {
    nfp_err(cpp, "Grabbing device lock not supported\n");
    return -EOPNOTSUPP;
    }
    key = crc32_posix(res.name, NFP_RESOURCE_ENTRY_NAME_SZ);
    for (i = 0; i < NFP_RESOURCE_TBL_ENTRIES; i++) {
    u64 addr = NFP_RESOURCE_TBL_BASE +
    sizeof(struct nfp_resource_entry) * i;
    ret = nfp_cpp_read(cpp, cpp_id, addr, &entry, sizeof(entry));
    if (ret != sizeof(entry))
    return -EIO;
    if (entry.mutex.key != key)
    continue;
// Found key!
    res.mutex =
    nfp_cpp_mutex_alloc(cpp,
    NFP_RESOURCE_TBL_TARGET, addr, key);
    if (!res.mutex)
    return -ENOMEM;
    res.cpp_id = NFP_CPP_ID(entry.region.cpp_target,
    entry.region.cpp_action,
    entry.region.cpp_token);
    res.addr = (u64)entry.region.page_offset << 8;
    res.size = (u64)entry.region.page_size << 8;
    return 0;
    }
    return -ENOENT;
    }
    static int
    nfp_resource_try_acquire(struct nfp_cpp *cpp, struct nfp_resource *res,
    struct nfp_cpp_mutex *dev_mutex)
    {
    int err;
    if (nfp_cpp_mutex_lock(dev_mutex))
    return -EINVAL;
    err = nfp_cpp_resource_find(cpp, res);
    if (err)
    goto err_unlock_dev;
    err = nfp_cpp_mutex_trylock(res.mutex);
    if (err)
    goto err_res_mutex_free;
    nfp_cpp_mutex_unlock(dev_mutex);
    return 0;
    err_res_mutex_free:
    nfp_cpp_mutex_free(res.mutex);
    err_unlock_dev:
    nfp_cpp_mutex_unlock(dev_mutex);
    return err;
    }
//
// nfp_resource_acquire() - Acquire a resource handle
// @cpp:	NFP CPP handle
// @name:	Name of the resource
//
// NOTE: This function locks the acquired resource
//
// Return: NFP Resource handle, or ERR_PTR()
//
    struct nfp_resource *
    nfp_resource_acquire(struct nfp_cpp *cpp, const char *name)
    {
    let mut warn_at: c_ulong = jiffies + NFP_MUTEX_WAIT_FIRST_WARN * HZ;
    let mut err_at: c_ulong = jiffies + NFP_MUTEX_WAIT_ERROR * HZ;
    struct nfp_cpp_mutex *dev_mutex;
    struct nfp_resource *res;
    int err;
    res = kzalloc_obj(*res);
    if (!res)
    return ERR_PTR(-ENOMEM);
    strscpy(res.name, name, sizeof(res.name));
    dev_mutex = nfp_cpp_mutex_alloc(cpp, NFP_RESOURCE_TBL_TARGET,
    NFP_RESOURCE_TBL_BASE,
    NFP_RESOURCE_TBL_KEY);
    if (!dev_mutex) {
    kfree(res);
    return ERR_PTR(-ENOMEM);
    }
    for (;;) {
    err = nfp_resource_try_acquire(cpp, res, dev_mutex);
    if (!err)
    break;
    if (err != -EBUSY)
    goto err_free;
    err = msleep_interruptible(1);
    if (err != 0) {
    err = -ERESTARTSYS;
    goto err_free;
    }
    if (time_is_before_eq_jiffies(warn_at)) {
    warn_at = jiffies + NFP_MUTEX_WAIT_NEXT_WARN * HZ;
    nfp_warn(cpp, "Warning: waiting for NFP resource %s\n",
    name);
    }
    if (time_is_before_eq_jiffies(err_at)) {
    nfp_err(cpp, "Error: resource %s timed out\n", name);
    err = -EBUSY;
    goto err_free;
    }
    }
    nfp_cpp_mutex_free(dev_mutex);
    return res;
    err_free:
    nfp_cpp_mutex_free(dev_mutex);
    kfree(res);
    return ERR_PTR(err);
    }
//
// nfp_resource_release() - Release a NFP Resource handle
// @res:	NFP Resource handle
//
// NOTE: This function implictly unlocks the resource handle
//
#[no_mangle]
pub unsafe extern "C" fn nfp_resource_release(res: *mut nfp_resource) {
    void nfp_resource_release(struct nfp_resource *res)
    {
    nfp_cpp_mutex_unlock(res.mutex);
    nfp_cpp_mutex_free(res.mutex);
    kfree(res);
    }
//
// nfp_resource_wait() - Wait for resource to appear
// @cpp:	NFP CPP handle
// @name:	Name of the resource
// @secs:	Number of seconds to wait
//
// Wait for resource to appear in the resource table, grab and release
// its lock.  The wait is jiffies-based, don't expect fine granularity.
//
// Return: 0 on success, errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn nfp_resource_wait(cpp: *mut nfp_cpp, name: *const c_char, secs: c_uint) -> c_int {
    int nfp_resource_wait(struct nfp_cpp *cpp, const char *name, unsigned int secs)
    {
    let mut warn_at: c_ulong = jiffies + NFP_MUTEX_WAIT_FIRST_WARN * HZ;
    let mut err_at: c_ulong = jiffies + secs * HZ;
    struct nfp_resource *res;
    while (true) {
    res = nfp_resource_acquire(cpp, name);
    if (!IS_ERR(res)) {
    nfp_resource_release(res);
    return 0;
    }
    if (PTR_ERR(res) != -ENOENT) {
    nfp_err(cpp, "error waiting for resource %s: %ld\n",
    name, PTR_ERR(res));
    return PTR_ERR(res);
    }
    if (time_is_before_eq_jiffies(err_at)) {
    nfp_err(cpp, "timeout waiting for resource %s\n", name);
    return -ETIMEDOUT;
    }
    if (time_is_before_eq_jiffies(warn_at)) {
    warn_at = jiffies + NFP_MUTEX_WAIT_NEXT_WARN * HZ;
    nfp_info(cpp, "waiting for NFP resource %s\n", name);
    }
    if (msleep_interruptible(10)) {
    nfp_err(cpp, "wait for resource %s interrupted\n",
    name);
    return -ERESTARTSYS;
    }
    }
    }
//
// nfp_resource_cpp_id() - Return the cpp_id of a resource handle
// @res:	NFP Resource handle
//
// Return: NFP CPP ID
//
#[no_mangle]
pub unsafe extern "C" fn nfp_resource_cpp_id(res: *mut nfp_resource) -> u32 {
    u32 nfp_resource_cpp_id(struct nfp_resource *res)
    {
    return res.cpp_id;
    }
//
// nfp_resource_name() - Return the name of a resource handle
// @res:	NFP Resource handle
//
// Return: const char pointer to the name of the resource
//
    const char *nfp_resource_name(struct nfp_resource *res)
    {
    return res.name;
    }
//
// nfp_resource_address() - Return the address of a resource handle
// @res:	NFP Resource handle
//
// Return: Address of the resource
//
#[no_mangle]
pub unsafe extern "C" fn nfp_resource_address(res: *mut nfp_resource) -> u64 {
    u64 nfp_resource_address(struct nfp_resource *res)
    {
    return res.addr;
    }
//
// nfp_resource_size() - Return the size in bytes of a resource handle
// @res:	NFP Resource handle
//
// Return: Size of the resource in bytes
//
#[no_mangle]
pub unsafe extern "C" fn nfp_resource_size(res: *mut nfp_resource) -> u64 {
    u64 nfp_resource_size(struct nfp_resource *res)
    {
    return res.size;
    }
//
// nfp_resource_table_init() - Run initial checks on the resource table
// @cpp:	NFP CPP handle
//
// Start-of-day init procedure for resource table.  Must be called before
// any local resource table users may exist.
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
pub unsafe extern "C" fn nfp_resource_table_init(cpp: *mut nfp_cpp) -> c_int {
    int nfp_resource_table_init(struct nfp_cpp *cpp)
    {
    struct nfp_cpp_mutex *dev_mutex;
    int i, err;
    err = nfp_cpp_mutex_reclaim(cpp, NFP_RESOURCE_TBL_TARGET,
    NFP_RESOURCE_TBL_BASE);
    if (err < 0) {
    nfp_err(cpp, "Error: failed to reclaim resource table mutex\n");
    return err;
    }
    if (err)
    nfp_warn(cpp, "Warning: busted main resource table mutex\n");
    dev_mutex = nfp_cpp_mutex_alloc(cpp, NFP_RESOURCE_TBL_TARGET,
    NFP_RESOURCE_TBL_BASE,
    NFP_RESOURCE_TBL_KEY);
    if (!dev_mutex)
    return -ENOMEM;
    if (nfp_cpp_mutex_lock(dev_mutex)) {
    nfp_err(cpp, "Error: failed to claim resource table mutex\n");
    nfp_cpp_mutex_free(dev_mutex);
    return -EINVAL;
    }
// Resource 0 is the dev_mutex, start from 1
    for (i = 1; i < NFP_RESOURCE_TBL_ENTRIES; i++) {
    u64 addr = NFP_RESOURCE_TBL_BASE +
    sizeof(struct nfp_resource_entry) * i;
    err = nfp_cpp_mutex_reclaim(cpp, NFP_RESOURCE_TBL_TARGET, addr);
    if (err < 0) {
    nfp_err(cpp,
    "Error: failed to reclaim resource %d mutex\n",
    i);
    goto err_unlock;
    }
    if (err)
    nfp_warn(cpp, "Warning: busted resource %d mutex\n", i);
    }
    err = 0;
    err_unlock:
    nfp_cpp_mutex_unlock(dev_mutex);
    nfp_cpp_mutex_free(dev_mutex);
    return err;
    }
