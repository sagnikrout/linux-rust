//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_mip.c
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
// Copyright (C) 2015-2017 Netronome Systems, Inc.
//
// nfp_mip.c
// Authors: Jakub Kicinski <jakub.kicinski@netronome.com>
// Jason McMullan <jason.mcmullan@netronome.com>
// Espen Skoglund <espen.skoglund@netronome.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_mip {
    pub signature: __le32,
    pub mip_version: __le32,
    pub mip_size: __le32,
    pub first_entry: __le32,
    pub version: __le32,
    pub buildnum: __le32,
    pub buildtime: __le32,
    pub loadtime: __le32,
    pub symtab_addr: __le32,
    pub symtab_size: __le32,
    pub strtab_addr: __le32,
    pub strtab_size: __le32,
    pub name: [c_char; 16],
    pub toolchain: [c_char; 32],
}

// Read memory and check if it could be a valid MIP
    static int
    nfp_mip_try_read(struct nfp_cpp *cpp, u32 cpp_id, u64 addr, struct nfp_mip *mip)
    {
    int ret;
    ret = nfp_cpp_read(cpp, cpp_id, addr, mip, sizeof(*mip));
    if (ret != sizeof(*mip)) {
    nfp_err(cpp, "Failed to read MIP data (%d, %zu)\n",
    ret, sizeof(*mip));
    return -EIO;
    }
    if (mip.signature != NFP_MIP_SIGNATURE) {
    nfp_warn(cpp, "Incorrect MIP signature (0x%08x)\n",
    le32_to_cpu(mip.signature));
    return -EINVAL;
    }
    if (mip.mip_version != NFP_MIP_VERSION) {
    nfp_warn(cpp, "Unsupported MIP version (%d)\n",
    le32_to_cpu(mip.mip_version));
    return -EINVAL;
    }
    return 0;
    }
// Try to locate MIP using the resource table
#[no_mangle]
unsafe extern "C" fn nfp_mip_read_resource(cpp: *mut nfp_cpp, mip: *mut nfp_mip) -> c_int {
    static int nfp_mip_read_resource(struct nfp_cpp *cpp, struct nfp_mip *mip)
    {
    struct nfp_nffw_info *nffw_info;
    u32 cpp_id;
    u64 addr;
    int err;
    nffw_info = nfp_nffw_info_open(cpp);
    if (IS_ERR(nffw_info))
    return PTR_ERR(nffw_info);
    err = nfp_nffw_info_mip_first(nffw_info, &cpp_id, &addr);
    if (err)
    goto exit_close_nffw;
    err = nfp_mip_try_read(cpp, cpp_id, addr, mip);
    exit_close_nffw:
    nfp_nffw_info_close(nffw_info);
    return err;
    }
//
// nfp_mip_open() - Get device MIP structure
// @cpp:	NFP CPP Handle
//
// Copy MIP structure from NFP device and return it.  The returned
// structure is handled internally by the library and should be
// freed by calling nfp_mip_close().
//
// Return: pointer to mip, NULL on failure.
//
    const struct nfp_mip *nfp_mip_open(struct nfp_cpp *cpp)
    {
    struct nfp_mip *mip;
    int err;
    mip = kmalloc_obj(*mip);
    if (!mip)
    return core::ptr::null_mut();
    err = nfp_mip_read_resource(cpp, mip);
    if (err) {
    kfree(mip);
    return core::ptr::null_mut();
    }
    mip.name[sizeof(mip.name) - 1] = 0;
    return mip;
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_mip_close(mip: *const nfp_mip) {
    void nfp_mip_close(const struct nfp_mip *mip)
    {
    kfree(mip);
    }
    const char *nfp_mip_name(const struct nfp_mip *mip)
    {
    return mip.name;
    }
//
// nfp_mip_symtab() - Get the address and size of the MIP symbol table
// @mip:	MIP handle
// @addr:	Location for NFP DDR address of MIP symbol table
// @size:	Location for size of MIP symbol table
//
#[no_mangle]
pub unsafe extern "C" fn nfp_mip_symtab(mip: *const nfp_mip, addr: *mut u32, size: *mut u32) {
    void nfp_mip_symtab(const struct nfp_mip *mip, u32 *addr, u32 *size)
    {
// addr = le32_to_cpu(mip->symtab_addr);
// size = le32_to_cpu(mip->symtab_size);
    }
//
// nfp_mip_strtab() - Get the address and size of the MIP symbol name table
// @mip:	MIP handle
// @addr:	Location for NFP DDR address of MIP symbol name table
// @size:	Location for size of MIP symbol name table
//
#[no_mangle]
pub unsafe extern "C" fn nfp_mip_strtab(mip: *const nfp_mip, addr: *mut u32, size: *mut u32) {
    void nfp_mip_strtab(const struct nfp_mip *mip, u32 *addr, u32 *size)
    {
// addr = le32_to_cpu(mip->strtab_addr);
// size = le32_to_cpu(mip->strtab_size);
    }
