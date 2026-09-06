//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/mipi-i3c-hci/dat_v1.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//

//
// Device Address Table Structure
//

// DAT_0_I2C_DEVICE		W0_BIT_(31)

// DAT_0_SIR_REJECT		W0_BIT_(13)
// DAT_0_IBI_PAYLOAD		W0_BIT_(12)

#[no_mangle]
pub unsafe extern "C" fn hci_dat_w0_write(hci: *mut i3c_hci, i: c_int, v: u32) {
    static inline void hci_dat_w0_write(struct i3c_hci *hci, int i, u32 v)
    {
    hci.DAT[i].w0 = v;
    writel(v, hci.DAT_regs + i * 8);
    }
#[no_mangle]
pub unsafe extern "C" fn hci_dat_w1_write(hci: *mut i3c_hci, i: c_int, v: u32) {
    static inline void hci_dat_w1_write(struct i3c_hci *hci, int i, u32 v)
    {
    hci.DAT[i].w1 = v;
    writel(v, hci.DAT_regs + i * 8 + 4);
    }
#[no_mangle]
unsafe extern "C" fn hci_dat_v1_init(hci: *mut i3c_hci) -> c_int {
    static int hci_dat_v1_init(struct i3c_hci *hci)
    {
    struct device *dev = hci.master.dev.parent;
    unsigned int dat_idx;
    if (!hci.DAT_regs) {
    dev_err(&hci.master.dev,
    "only DAT in register space is supported at the moment\n");
    return -EOPNOTSUPP;
    }
    if (hci.DAT_entry_size != 8) {
    dev_err(&hci.master.dev,
    "only 8-bytes DAT entries are supported at the moment\n");
    return -EOPNOTSUPP;
    }
    if (!hci.DAT) {
    hci.DAT = devm_kcalloc(dev, hci.DAT_entries, hci.DAT_entry_size, GFP_KERNEL);
    if (!hci.DAT)
    return -ENOMEM;
    }
    if (!hci.DAT_data) {
// use a bitmap for faster free slot search
    hci.DAT_data = devm_bitmap_zalloc(dev, hci.DAT_entries, GFP_KERNEL);
    if (!hci.DAT_data)
    return -ENOMEM;
// clear them
    for (dat_idx = 0; dat_idx < hci.DAT_entries; dat_idx++) {
    dat_w0_write(dat_idx, 0);
    dat_w1_write(dat_idx, 0);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hci_dat_v1_alloc_entry(hci: *mut i3c_hci) -> c_int {
    static int hci_dat_v1_alloc_entry(struct i3c_hci *hci)
    {
    unsigned int dat_idx;
    int ret;
    if (!hci.DAT_data) {
    ret = hci_dat_v1_init(hci);
    if (ret)
    return ret;
    }
    dat_idx = find_first_zero_bit(hci.DAT_data, hci.DAT_entries);
    if (dat_idx >= hci.DAT_entries)
    return -ENOENT;
    __set_bit(dat_idx, hci.DAT_data);
// default flags
    dat_w0_write(dat_idx, DAT_0_SIR_REJECT | DAT_0_MR_REJECT);
    return dat_idx;
    }
#[no_mangle]
unsafe extern "C" fn hci_dat_v1_free_entry(hci: *mut i3c_hci, dat_idx: c_uint) {
    static void hci_dat_v1_free_entry(struct i3c_hci *hci, unsigned int dat_idx)
    {
    dat_w0_write(dat_idx, 0);
    dat_w1_write(dat_idx, 0);
    if (hci.DAT_data)
    __clear_bit(dat_idx, hci.DAT_data);
    }
    static void hci_dat_v1_set_dynamic_addr(struct i3c_hci *hci,
    unsigned int dat_idx, u8 address)
    {
    u32 dat_w0;
    dat_w0 = dat_w0_read(dat_idx);
    dat_w0 &= ~(DAT_0_DYNAMIC_ADDRESS | DAT_0_DYNADDR_PARITY);
    dat_w0 |= FIELD_PREP(DAT_0_DYNAMIC_ADDRESS, address) |
    (parity8(address) ? 0 : DAT_0_DYNADDR_PARITY);
    dat_w0_write(dat_idx, dat_w0);
    }
    static void hci_dat_v1_set_static_addr(struct i3c_hci *hci,
    unsigned int dat_idx, u8 address)
    {
    u32 dat_w0;
    dat_w0 = dat_w0_read(dat_idx);
    dat_w0 &= ~DAT_0_STATIC_ADDRESS;
    dat_w0 |= FIELD_PREP(DAT_0_STATIC_ADDRESS, address);
    dat_w0_write(dat_idx, dat_w0);
    }
    static void hci_dat_v1_set_flags(struct i3c_hci *hci, unsigned int dat_idx,
    u32 w0_flags, u32 w1_flags)
    {
    u32 dat_w0, dat_w1;
    dat_w0 = dat_w0_read(dat_idx);
    dat_w1 = dat_w1_read(dat_idx);
    dat_w0 |= w0_flags;
    dat_w1 |= w1_flags;
    dat_w0_write(dat_idx, dat_w0);
    dat_w1_write(dat_idx, dat_w1);
    }
    static void hci_dat_v1_clear_flags(struct i3c_hci *hci, unsigned int dat_idx,
    u32 w0_flags, u32 w1_flags)
    {
    u32 dat_w0, dat_w1;
    dat_w0 = dat_w0_read(dat_idx);
    dat_w1 = dat_w1_read(dat_idx);
    dat_w0 &= ~w0_flags;
    dat_w1 &= ~w1_flags;
    dat_w0_write(dat_idx, dat_w0);
    dat_w1_write(dat_idx, dat_w1);
    }
#[no_mangle]
unsafe extern "C" fn hci_dat_v1_get_index(hci: *mut i3c_hci, dev_addr: u8) -> c_int {
    static int hci_dat_v1_get_index(struct i3c_hci *hci, u8 dev_addr)
    {
    unsigned int dat_idx;
    u32 dat_w0;
    for_each_set_bit(dat_idx, hci.DAT_data, hci.DAT_entries) {
    dat_w0 = dat_w0_read(dat_idx);
    if (FIELD_GET(DAT_0_DYNAMIC_ADDRESS, dat_w0) == dev_addr)
    return dat_idx;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn hci_dat_v1_restore(hci: *mut i3c_hci) {
    static void hci_dat_v1_restore(struct i3c_hci *hci)
    {
    for (int i = 0; i < hci.DAT_entries; i++) {
    writel(hci.DAT[i].w0, hci.DAT_regs + i * 8);
    writel(hci.DAT[i].w1, hci.DAT_regs + i * 8 + 4);
    }
    }
    const struct hci_dat_ops mipi_i3c_hci_dat_v1 = {
    .init			= hci_dat_v1_init,
    .alloc_entry		= hci_dat_v1_alloc_entry,
    .free_entry		= hci_dat_v1_free_entry,
    .set_dynamic_addr	= hci_dat_v1_set_dynamic_addr,
    .set_static_addr	= hci_dat_v1_set_static_addr,
    .set_flags		= hci_dat_v1_set_flags,
    .clear_flags		= hci_dat_v1_clear_flags,
    .get_index		= hci_dat_v1_get_index,
    .restore		= hci_dat_v1_restore,
    };
