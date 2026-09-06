//! Automatically rewritten from C to Rust
//! Source: fs/nfs_common/nfs_ssc.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Helper for knfsd's SSC to access ops in NFS client modules
//
// Author: Dai Ngo <dai.ngo@oracle.com>
//
// Copyright (c) 2020, Oracle and/or its affiliates.
//

    struct nfs_ssc_client_ops_tbl nfs_ssc_client_tbl;
    EXPORT_SYMBOL_GPL(nfs_ssc_client_tbl);

//
// nfs42_ssc_register - install the NFS_V4 client ops in the nfs_ssc_client_tbl
// @ops: NFS_V4 ops to be installed
//
// Return values:
// None
//
#[no_mangle]
pub unsafe extern "C" fn nfs42_ssc_register(ops: *const nfs4_ssc_client_ops) {
    void nfs42_ssc_register(const struct nfs4_ssc_client_ops *ops)
    {
    nfs_ssc_client_tbl.ssc_nfs4_ops = ops;
    }
    EXPORT_SYMBOL_GPL(nfs42_ssc_register);
//
// nfs42_ssc_unregister - uninstall the NFS_V4 client ops from
// the nfs_ssc_client_tbl
// @ops: ops to be uninstalled
//
// Return values:
// None
//
#[no_mangle]
pub unsafe extern "C" fn nfs42_ssc_unregister(ops: *const nfs4_ssc_client_ops) {
    void nfs42_ssc_unregister(const struct nfs4_ssc_client_ops *ops)
    {
    if (nfs_ssc_client_tbl.ssc_nfs4_ops != ops)
    return;
    nfs_ssc_client_tbl.ssc_nfs4_ops = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(nfs42_ssc_unregister);

//
// nfs_ssc_register - install the NFS_FS client ops in the nfs_ssc_client_tbl
// @ops: NFS_FS ops to be installed
//
// Return values:
// None
//
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_register(ops: *const nfs_ssc_client_ops) {
    void nfs_ssc_register(const struct nfs_ssc_client_ops *ops)
    {
    nfs_ssc_client_tbl.ssc_nfs_ops = ops;
    }
    EXPORT_SYMBOL_GPL(nfs_ssc_register);
//
// nfs_ssc_unregister - uninstall the NFS_FS client ops from
// the nfs_ssc_client_tbl
// @ops: ops to be uninstalled
//
// Return values:
// None
//
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_unregister(ops: *const nfs_ssc_client_ops) {
    void nfs_ssc_unregister(const struct nfs_ssc_client_ops *ops)
    {
    if (nfs_ssc_client_tbl.ssc_nfs_ops != ops)
    return;
    nfs_ssc_client_tbl.ssc_nfs_ops = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(nfs_ssc_unregister);

#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_register(ops: *const nfs_ssc_client_ops) {
    void nfs_ssc_register(const struct nfs_ssc_client_ops *ops)
    {
    }
    EXPORT_SYMBOL_GPL(nfs_ssc_register);
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_unregister(ops: *const nfs_ssc_client_ops) {
    void nfs_ssc_unregister(const struct nfs_ssc_client_ops *ops)
    {
    }
    EXPORT_SYMBOL_GPL(nfs_ssc_unregister);
