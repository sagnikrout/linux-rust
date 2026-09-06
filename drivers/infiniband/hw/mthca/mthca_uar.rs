//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mthca/mthca_uar.c
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


//
// Copyright (c) 2005 Topspin Communications.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[no_mangle]
pub unsafe extern "C" fn mthca_uar_alloc(dev: *mut mthca_dev, uar: *mut mthca_uar) -> c_int {
    int mthca_uar_alloc(struct mthca_dev *dev, struct mthca_uar *uar)
    {
    uar.index = mthca_alloc(&dev.uar_table.alloc);
    if (uar.index == -1)
    return -ENOMEM;
    uar.pfn = (pci_resource_start(dev.pdev, 2) >> PAGE_SHIFT) + uar.index;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_uar_free(dev: *mut mthca_dev, uar: *mut mthca_uar) {
    void mthca_uar_free(struct mthca_dev *dev, struct mthca_uar *uar)
    {
    mthca_free(&dev.uar_table.alloc, uar.index);
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_init_uar_table(dev: *mut mthca_dev) -> c_int {
    int mthca_init_uar_table(struct mthca_dev *dev)
    {
    int ret;
    ret = mthca_alloc_init(&dev.uar_table.alloc,
    dev.limits.num_uars,
    dev.limits.num_uars - 1,
    dev.limits.reserved_uars + 1);
    if (ret)
    return ret;
    ret = mthca_init_db_tab(dev);
    if (ret)
    mthca_alloc_cleanup(&dev.uar_table.alloc);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_cleanup_uar_table(dev: *mut mthca_dev) {
    void mthca_cleanup_uar_table(struct mthca_dev *dev)
    {
    mthca_cleanup_db_tab(dev);
// XXX check if any UARs are still allocated?
    mthca_alloc_cleanup(&dev.uar_table.alloc);
    }
