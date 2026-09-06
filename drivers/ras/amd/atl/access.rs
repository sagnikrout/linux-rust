//! Automatically rewritten from C to Rust
//! Source: drivers/ras/amd/atl/access.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMD Address Translation Library
//
// access.c : DF Indirect Access functions
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
//

// Protect the PCI config register pairs used for DF indirect access.
    static DEFINE_MUTEX(df_indirect_mutex);
//
// Data Fabric Indirect Access uses FICAA/FICAD.
//
// Fabric Indirect Configuration Access Address (FICAA): constructed based
// on the device's Instance Id and the PCI function and register offset of
// the desired register.
//
// Fabric Indirect Configuration Access Data (FICAD): there are FICAD
// low and high registers but so far only the low register is needed.
//
// Use Instance Id 0xFF to indicate a broadcast read.
//
pub const DF_BROADCAST: c_uint = 0xFF;

#[no_mangle]
unsafe extern "C" fn get_accessible_node(node: u16) -> u16 {
    static u16 get_accessible_node(u16 node)
    {
//
// On heterogeneous systems, not all AMD Nodes are accessible
// through software-visible registers. The Node ID needs to be
// adjusted for register accesses. But its value should not be
// changed for the translation methods.
//
    if (df_cfg.flags.heterogeneous) {
// Only Node 0 is accessible on DF3.5 systems.
    if (df_cfg.rev == DF3p5)
    node = 0;
//
// Only the first Node in each Socket is accessible on
// DF4.5 systems, and this is visible to software as one
// Fabric per Socket.  The Socket ID can be derived from
// the Node ID and global shift values.
//
    if (df_cfg.rev == DF4p5)
    node >>= df_cfg.socket_id_shift - df_cfg.node_id_shift;
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn __df_indirect_read(node: u16, func: u8, reg: u16, instance_id: u8, lo: *mut u32) -> c_int {
    static int __df_indirect_read(u16 node, u8 func, u16 reg, u8 instance_id, u32 *lo)
    {
    let mut ficaa_addr: u32 = 0x8C, ficad_addr = 0xB8;
    struct pci_dev *F4;
    let mut err: c_int = -ENODEV;
    let mut ficaa: u32 = 0;
    node = get_accessible_node(node);
    if (node >= amd_nb_num()) {
    pr_debug("Node %u is out of bounds\n", node);
    goto out;
    }
    F4 = node_to_amd_nb(node).link;
    if (!F4) {
    pr_debug("DF function 4 not found\n");
    goto out;
    }
// Enable instance-specific access.
    if (instance_id != DF_BROADCAST) {
    ficaa |= FIELD_PREP(DF_FICAA_INST_EN, 1);
    ficaa |= FIELD_PREP(DF_FICAA_INST_ID, instance_id);
    }
//
// The two least-significant bits are masked when inputing the
// register offset to FICAA.
//
    reg >>= 2;
    if (df_cfg.flags.legacy_ficaa) {
    ficaa_addr = 0x5C;
    ficad_addr = 0x98;
    ficaa |= FIELD_PREP(DF_FICAA_REG_NUM_LEGACY, reg);
    } else {
    ficaa |= FIELD_PREP(DF_FICAA_REG_NUM, reg);
    }
    ficaa |= FIELD_PREP(DF_FICAA_FUNC_NUM, func);
    mutex_lock(&df_indirect_mutex);
    err = pci_write_config_dword(F4, ficaa_addr, ficaa);
    if (err) {
    pr_warn("Error writing DF Indirect FICAA, FICAA=0x%x\n", ficaa);
    goto out_unlock;
    }
    err = pci_read_config_dword(F4, ficad_addr, lo);
    if (err)
    pr_warn("Error reading DF Indirect FICAD LO, FICAA=0x%x.\n", ficaa);
    pr_debug("node=%u inst=0x%x func=0x%x reg=0x%x val=0x%x",
    node, instance_id, func, reg << 2, *lo);
    out_unlock:
    mutex_unlock(&df_indirect_mutex);
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn df_indirect_read_instance(node: u16, func: u8, reg: u16, instance_id: u8, lo: *mut u32) -> c_int {
    int df_indirect_read_instance(u16 node, u8 func, u16 reg, u8 instance_id, u32 *lo)
    {
    return __df_indirect_read(node, func, reg, instance_id, lo);
    }
#[no_mangle]
pub unsafe extern "C" fn df_indirect_read_broadcast(node: u16, func: u8, reg: u16, lo: *mut u32) -> c_int {
    int df_indirect_read_broadcast(u16 node, u8 func, u16 reg, u32 *lo)
    {
    return __df_indirect_read(node, func, reg, DF_BROADCAST, lo);
    }
