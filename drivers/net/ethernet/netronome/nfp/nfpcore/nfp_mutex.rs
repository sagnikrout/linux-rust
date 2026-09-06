//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_mutex.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_mutex {
    pub cpp: *mut nfp_cpp,
    pub target: c_int,
    pub depth: u16,
    pub address: c_ulonglong,
    pub key: u32,
}

#[no_mangle]
unsafe extern "C" fn nfp_mutex_locked(interface: u16) -> u32 {
    static u32 nfp_mutex_locked(u16 interface)
    {
    return (u32)interface << 16 | 0x000f;
    }
#[no_mangle]
unsafe extern "C" fn nfp_mutex_unlocked(interface: u16) -> u32 {
    static u32 nfp_mutex_unlocked(u16 interface)
    {
    return (u32)interface << 16 | 0x0000;
    }
#[no_mangle]
unsafe extern "C" fn nfp_mutex_owner(val: u32) -> u32 {
    static u32 nfp_mutex_owner(u32 val)
    {
    return val >> 16;
    }
#[no_mangle]
unsafe extern "C" fn nfp_mutex_is_locked(val: u32) -> bool {
    static bool nfp_mutex_is_locked(u32 val)
    {
    return (val & 0xffff) == 0x000f;
    }
#[no_mangle]
unsafe extern "C" fn nfp_mutex_is_unlocked(val: u32) -> bool {
    static bool nfp_mutex_is_unlocked(u32 val)
    {
    return (val & 0xffff) == 0000;
    }
// If you need more than 65536 recursive locks, please rethink your code.
pub const NFP_MUTEX_DEPTH_MAX: c_uint = 0xffff;
    static int
    nfp_cpp_mutex_validate(u16 interface, int *target, unsigned long long address)
    {
// Not permitted on invalid interfaces
    if (NFP_CPP_INTERFACE_TYPE_of(interface) ==
    NFP_CPP_INTERFACE_TYPE_INVALID)
    return -EINVAL;
// Address must be 64-bit aligned
    if (address & 7)
    return -EINVAL;
    if (*target != NFP_CPP_TARGET_MU)
    return -EINVAL;
    return 0;
    }
//
// nfp_cpp_mutex_init() - Initialize a mutex location
// @cpp:	NFP CPP handle
// @target:	NFP CPP target ID (ie NFP_CPP_TARGET_CLS or NFP_CPP_TARGET_MU)
// @address:	Offset into the address space of the NFP CPP target ID
// @key:	Unique 32-bit value for this mutex
//
// The CPP target:address must point to a 64-bit aligned location, and
// will initialize 64 bits of data at the location.
//
// This creates the initial mutex state, as locked by this
// nfp_cpp_interface().
//
// This function should only be called when setting up
// the initial lock state upon boot-up of the system.
//
// Return: 0 on success, or -errno on failure
//
    int nfp_cpp_mutex_init(struct nfp_cpp *cpp,
    int target, unsigned long long address, u32 key)
    {
    const u32 muw = NFP_CPP_ID(target, 4, 0);    /* atomic_write */
    let mut interface: u16 = nfp_cpp_interface(cpp);
    int err;
    err = nfp_cpp_mutex_validate(interface, &target, address);
    if (err)
    return err;
    err = nfp_cpp_writel(cpp, muw, address + 4, key);
    if (err)
    return err;
    err = nfp_cpp_writel(cpp, muw, address, nfp_mutex_locked(interface));
    if (err)
    return err;
    return 0;
    }
//
// nfp_cpp_mutex_alloc() - Create a mutex handle
// @cpp:	NFP CPP handle
// @target:	NFP CPP target ID (ie NFP_CPP_TARGET_CLS or NFP_CPP_TARGET_MU)
// @address:	Offset into the address space of the NFP CPP target ID
// @key:	32-bit unique key (must match the key at this location)
//
// The CPP target:address must point to a 64-bit aligned location, and
// reserve 64 bits of data at the location for use by the handle.
//
// Only target/address pairs that point to entities that support the
// MU Atomic Engine's CmpAndSwap32 command are supported.
//
// Return:	A non-NULL struct nfp_cpp_mutex * on success, NULL on failure.
//
    struct nfp_cpp_mutex *nfp_cpp_mutex_alloc(struct nfp_cpp *cpp, int target,
    unsigned long long address, u32 key)
    {
    const u32 mur = NFP_CPP_ID(target, 3, 0);    /* atomic_read */
    let mut interface: u16 = nfp_cpp_interface(cpp);
    struct nfp_cpp_mutex *mutex;
    int err;
    u32 tmp;
    err = nfp_cpp_mutex_validate(interface, &target, address);
    if (err)
    return core::ptr::null_mut();
    err = nfp_cpp_readl(cpp, mur, address + 4, &tmp);
    if (err < 0)
    return core::ptr::null_mut();
    if (tmp != key)
    return core::ptr::null_mut();
    mutex = kzalloc_obj(*mutex);
    if (!mutex)
    return core::ptr::null_mut();
    mutex.cpp = cpp;
    mutex.target = target;
    mutex.address = address;
    mutex.key = key;
    mutex.depth = 0;
    return mutex;
    }
//
// nfp_cpp_mutex_free() - Free a mutex handle - does not alter the lock state
// @mutex:	NFP CPP Mutex handle
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_mutex_free(mutex: *mut nfp_cpp_mutex) {
    void nfp_cpp_mutex_free(struct nfp_cpp_mutex *mutex)
    {
    kfree(mutex);
    }
//
// nfp_cpp_mutex_lock() - Lock a mutex handle, using the NFP MU Atomic Engine
// @mutex:	NFP CPP Mutex handle
//
// Return: 0 on success, or -errno on failure
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_mutex_lock(mutex: *mut nfp_cpp_mutex) -> c_int {
    int nfp_cpp_mutex_lock(struct nfp_cpp_mutex *mutex)
    {
    let mut warn_at: c_ulong = jiffies + NFP_MUTEX_WAIT_FIRST_WARN * HZ;
    let mut err_at: c_ulong = jiffies + NFP_MUTEX_WAIT_ERROR * HZ;
    let mut timeout_ms: c_uint = 1;
    int err;
// We can't use a waitqueue here, because the unlocker
// might be on a separate CPU.
//
// So just wait for now.
//
    for (;;) {
    err = nfp_cpp_mutex_trylock(mutex);
    if (err != -EBUSY)
    break;
    err = msleep_interruptible(timeout_ms);
    if (err != 0) {
    nfp_info(mutex.cpp,
    "interrupted waiting for NFP mutex\n");
    return -ERESTARTSYS;
    }
    if (time_is_before_eq_jiffies(warn_at)) {
    warn_at = jiffies + NFP_MUTEX_WAIT_NEXT_WARN * HZ;
    nfp_warn(mutex.cpp,
    "Warning: waiting for NFP mutex [depth:%hd target:%d addr:%llx key:%08x]\n",
    mutex.depth,
    mutex.target, mutex.address, mutex.key);
    }
    if (time_is_before_eq_jiffies(err_at)) {
    nfp_err(mutex.cpp, "Error: mutex wait timed out\n");
    return -EBUSY;
    }
    }
    return err;
    }
//
// nfp_cpp_mutex_unlock() - Unlock a mutex handle, using the MU Atomic Engine
// @mutex:	NFP CPP Mutex handle
//
// Return: 0 on success, or -errno on failure
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_mutex_unlock(mutex: *mut nfp_cpp_mutex) -> c_int {
    int nfp_cpp_mutex_unlock(struct nfp_cpp_mutex *mutex)
    {
    const u32 muw = NFP_CPP_ID(mutex.target, 4, 0);    /* atomic_write */
    const u32 mur = NFP_CPP_ID(mutex.target, 3, 0);    /* atomic_read */
    struct nfp_cpp *cpp = mutex.cpp;
    u32 key, value;
    u16 interface;
    int err;
    interface = nfp_cpp_interface(cpp);
    if (mutex.depth > 1) {
    mutex.depth--;
    return 0;
    }
    err = nfp_cpp_readl(mutex.cpp, mur, mutex.address + 4, &key);
    if (err < 0)
    return err;
    if (key != mutex.key)
    return -EPERM;
    err = nfp_cpp_readl(mutex.cpp, mur, mutex.address, &value);
    if (err < 0)
    return err;
    if (value != nfp_mutex_locked(interface))
    return -EACCES;
    err = nfp_cpp_writel(cpp, muw, mutex.address,
    nfp_mutex_unlocked(interface));
    if (err < 0)
    return err;
    mutex.depth = 0;
    return 0;
    }
//
// nfp_cpp_mutex_trylock() - Attempt to lock a mutex handle
// @mutex:	NFP CPP Mutex handle
//
// Return:      0 if the lock succeeded, -errno on failure
//
#[no_mangle]
pub unsafe extern "C" fn nfp_cpp_mutex_trylock(mutex: *mut nfp_cpp_mutex) -> c_int {
    int nfp_cpp_mutex_trylock(struct nfp_cpp_mutex *mutex)
    {
    const u32 muw = NFP_CPP_ID(mutex.target, 4, 0);    /* atomic_write */
    const u32 mus = NFP_CPP_ID(mutex.target, 5, 3);    /* test_set_imm */
    const u32 mur = NFP_CPP_ID(mutex.target, 3, 0);    /* atomic_read */
    struct nfp_cpp *cpp = mutex.cpp;
    u32 key, value, tmp;
    int err;
    if (mutex.depth > 0) {
    if (mutex.depth == NFP_MUTEX_DEPTH_MAX)
    return -E2BIG;
    mutex.depth++;
    return 0;
    }
// Verify that the lock marker is not damaged
    err = nfp_cpp_readl(cpp, mur, mutex.address + 4, &key);
    if (err < 0)
    return err;
    if (key != mutex.key)
    return -EPERM;
// Compare against the unlocked state, and if true,
// write the interface id into the top 16 bits, and
// mark as locked.
//
    value = nfp_mutex_locked(nfp_cpp_interface(cpp));
// We use test_set_imm here, as it implies a read
// of the current state, and sets the bits in the
// bytemask of the command to 1s. Since the mutex
// is guaranteed to be 64-bit aligned, the bytemask
// of this 32-bit command is ensured to be 8'b00001111,
// which implies that the lower 4 bits will be set to
// ones regardless of the initial state.
//
// Since this is a 'Readback' operation, with no Pull
// data, we can treat this as a normal Push (read)
// atomic, which returns the original value.
//
    err = nfp_cpp_readl(cpp, mus, mutex.address, &tmp);
    if (err < 0)
    return err;
// Was it unlocked?
    if (nfp_mutex_is_unlocked(tmp)) {
// The read value can only be 0x....0000 in the unlocked state.
// If there was another contending for this lock, then
// the lock state would be 0x....000f
//
// Write our owner ID into the lock
// While not strictly necessary, this helps with
// debug and bookkeeping.
//
    err = nfp_cpp_writel(cpp, muw, mutex.address, value);
    if (err < 0)
    return err;
    mutex.depth = 1;
    return 0;
    }
    return nfp_mutex_is_locked(tmp) ? -EBUSY : -EINVAL;
    }
//
// nfp_cpp_mutex_reclaim() - Unlock mutex if held by local endpoint
// @cpp:	NFP CPP handle
// @target:	NFP CPP target ID (ie NFP_CPP_TARGET_CLS or NFP_CPP_TARGET_MU)
// @address:	Offset into the address space of the NFP CPP target ID
//
// Release lock if held by local system.  Extreme care is advised, call only
// when no local lock users can exist.
//
// Return:      0 if the lock was OK, 1 if locked by us, -errno on invalid mutex
//
    int nfp_cpp_mutex_reclaim(struct nfp_cpp *cpp, int target,
    unsigned long long address)
    {
    const u32 mur = NFP_CPP_ID(target, 3, 0);	/* atomic_read */
    const u32 muw = NFP_CPP_ID(target, 4, 0);	/* atomic_write */
    let mut interface: u16 = nfp_cpp_interface(cpp);
    int err;
    u32 tmp;
    err = nfp_cpp_mutex_validate(interface, &target, address);
    if (err)
    return err;
// Check lock
    err = nfp_cpp_readl(cpp, mur, address, &tmp);
    if (err < 0)
    return err;
    if (nfp_mutex_is_unlocked(tmp) || nfp_mutex_owner(tmp) != interface)
    return 0;
// Bust the lock
    err = nfp_cpp_writel(cpp, muw, address, nfp_mutex_unlocked(interface));
    if (err < 0)
    return err;
    return 1;
    }
