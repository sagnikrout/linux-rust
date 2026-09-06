//! Automatically rewritten from C to Rust
//! Source: drivers/ras/amd/atl/system.c
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
// system.c : Functions to read and save system-wide data
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
//

    const guid_t norm_to_sys_guid = GUID_INIT(0xE7180659, 0xA65D, 0x451D,
    0x92, 0xCD, 0x2B, 0x56, 0xF1,
    0x2B, 0xEB, 0xA6);
#[no_mangle]
pub unsafe extern "C" fn determine_node_id(ctx: *mut addr_ctx, socket_id: u8, die_id: u8) -> c_int {
    int determine_node_id(struct addr_ctx *ctx, u8 socket_id, u8 die_id)
    {
    u16 socket_id_bits, die_id_bits;
    if (socket_id > 0 && df_cfg.socket_id_mask == 0) {
    atl_debug(ctx, "Invalid socket inputs: socket_id=%u socket_id_mask=0x%x",
    socket_id, df_cfg.socket_id_mask);
    return -EINVAL;
    }
// Do each step independently to avoid shift out-of-bounds issues.
    socket_id_bits =	socket_id;
    socket_id_bits <<=	df_cfg.socket_id_shift;
    socket_id_bits &=	df_cfg.socket_id_mask;
    if (die_id > 0 && df_cfg.die_id_mask == 0) {
    atl_debug(ctx, "Invalid die inputs: die_id=%u die_id_mask=0x%x",
    die_id, df_cfg.die_id_mask);
    return -EINVAL;
    }
// Do each step independently to avoid shift out-of-bounds issues.
    die_id_bits =		die_id;
    die_id_bits <<=		df_cfg.die_id_shift;
    die_id_bits &=		df_cfg.die_id_mask;
    ctx.node_id = (socket_id_bits | die_id_bits) >> df_cfg.node_id_shift;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn df2_get_masks_shifts(mask0: u32) {
    static void df2_get_masks_shifts(u32 mask0)
    {
    df_cfg.socket_id_shift		= FIELD_GET(DF2_SOCKET_ID_SHIFT, mask0);
    df_cfg.socket_id_mask		= FIELD_GET(DF2_SOCKET_ID_MASK, mask0);
    df_cfg.die_id_shift		= FIELD_GET(DF2_DIE_ID_SHIFT, mask0);
    df_cfg.die_id_mask		= FIELD_GET(DF2_DIE_ID_MASK, mask0);
    df_cfg.node_id_shift		= df_cfg.die_id_shift;
    df_cfg.node_id_mask		= df_cfg.socket_id_mask | df_cfg.die_id_mask;
    df_cfg.component_id_mask	= ~df_cfg.node_id_mask;
    }
#[no_mangle]
unsafe extern "C" fn df3_get_masks_shifts(mask0: u32, mask1: u32) {
    static void df3_get_masks_shifts(u32 mask0, u32 mask1)
    {
    df_cfg.component_id_mask	= FIELD_GET(DF3_COMPONENT_ID_MASK, mask0);
    df_cfg.node_id_mask		= FIELD_GET(DF3_NODE_ID_MASK, mask0);
    df_cfg.node_id_shift		= FIELD_GET(DF3_NODE_ID_SHIFT, mask1);
    df_cfg.socket_id_shift		= FIELD_GET(DF3_SOCKET_ID_SHIFT, mask1);
    df_cfg.socket_id_mask		= FIELD_GET(DF3_SOCKET_ID_MASK, mask1);
    df_cfg.die_id_mask		= FIELD_GET(DF3_DIE_ID_MASK, mask1);
    }
#[no_mangle]
unsafe extern "C" fn df3p5_get_masks_shifts(mask0: u32, mask1: u32, mask2: u32) {
    static void df3p5_get_masks_shifts(u32 mask0, u32 mask1, u32 mask2)
    {
    df_cfg.component_id_mask	= FIELD_GET(DF4_COMPONENT_ID_MASK, mask0);
    df_cfg.node_id_mask		= FIELD_GET(DF4_NODE_ID_MASK, mask0);
    df_cfg.node_id_shift		= FIELD_GET(DF3_NODE_ID_SHIFT, mask1);
    df_cfg.socket_id_shift		= FIELD_GET(DF4_SOCKET_ID_SHIFT, mask1);
    df_cfg.socket_id_mask		= FIELD_GET(DF4_SOCKET_ID_MASK, mask2);
    df_cfg.die_id_mask		= FIELD_GET(DF4_DIE_ID_MASK, mask2);
    }
#[no_mangle]
unsafe extern "C" fn df4_get_masks_shifts(mask0: u32, mask1: u32, mask2: u32) {
    static void df4_get_masks_shifts(u32 mask0, u32 mask1, u32 mask2)
    {
    df3p5_get_masks_shifts(mask0, mask1, mask2);
    if (!(df_cfg.flags.socket_id_shift_quirk && df_cfg.socket_id_shift == 1))
    return;
    df_cfg.socket_id_shift	= 0;
    df_cfg.socket_id_mask	= 1;
    df_cfg.die_id_shift	= 0;
    df_cfg.die_id_mask	= 0;
    df_cfg.node_id_shift	= 8;
    df_cfg.node_id_mask	= 0x100;
    }
#[no_mangle]
unsafe extern "C" fn df4_get_fabric_id_mask_registers() -> c_int {
    static int df4_get_fabric_id_mask_registers(void)
    {
    u32 mask0, mask1, mask2;
// Read D18F4x1B0 (SystemFabricIdMask0)
    if (df_indirect_read_broadcast(0, 4, 0x1B0, &mask0))
    return -EINVAL;
// Read D18F4x1B4 (SystemFabricIdMask1)
    if (df_indirect_read_broadcast(0, 4, 0x1B4, &mask1))
    return -EINVAL;
// Read D18F4x1B8 (SystemFabricIdMask2)
    if (df_indirect_read_broadcast(0, 4, 0x1B8, &mask2))
    return -EINVAL;
    df4_get_masks_shifts(mask0, mask1, mask2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn df4_determine_df_rev(reg: u32) -> c_int {
    static int df4_determine_df_rev(u32 reg)
    {
    df_cfg.rev = FIELD_GET(DF_MINOR_REVISION, reg) < 5 ? DF4 : DF4p5;
// Check for special cases or quirks based on Device/Vendor IDs.
// Read D18F0x000 (DeviceVendorId0)
    if (df_indirect_read_broadcast(0, 0, 0, &reg))
    return -EINVAL;
    if (reg == DF_FUNC0_ID_ZEN4_SERVER)
    df_cfg.flags.socket_id_shift_quirk = 1;
    if (reg == DF_FUNC0_ID_MI300) {
    df_cfg.flags.heterogeneous = 1;
    if (get_umc_info_mi300())
    return -EINVAL;
    }
    return df4_get_fabric_id_mask_registers();
    }
#[no_mangle]
unsafe extern "C" fn determine_df_rev_legacy() -> c_int {
    static int determine_df_rev_legacy(void)
    {
    u32 fabric_id_mask0, fabric_id_mask1, fabric_id_mask2;
//
// Check for DF3.5.
//
// Component ID Mask must be non-zero. Register D18F1x150 is
// reserved pre-DF3.5, so value will be Read-as-Zero.
//
// Read D18F1x150 (SystemFabricIdMask0).
    if (df_indirect_read_broadcast(0, 1, 0x150, &fabric_id_mask0))
    return -EINVAL;
    if (FIELD_GET(DF4_COMPONENT_ID_MASK, fabric_id_mask0)) {
    df_cfg.rev = DF3p5;
// Read D18F1x154 (SystemFabricIdMask1)
    if (df_indirect_read_broadcast(0, 1, 0x154, &fabric_id_mask1))
    return -EINVAL;
// Read D18F1x158 (SystemFabricIdMask2)
    if (df_indirect_read_broadcast(0, 1, 0x158, &fabric_id_mask2))
    return -EINVAL;
    df3p5_get_masks_shifts(fabric_id_mask0, fabric_id_mask1, fabric_id_mask2);
    return 0;
    }
//
// Check for DF3.
//
// Component ID Mask must be non-zero. Field is Read-as-Zero on DF2.
//
// Read D18F1x208 (SystemFabricIdMask).
    if (df_indirect_read_broadcast(0, 1, 0x208, &fabric_id_mask0))
    return -EINVAL;
    if (FIELD_GET(DF3_COMPONENT_ID_MASK, fabric_id_mask0)) {
    df_cfg.rev = DF3;
// Read D18F1x20C (SystemFabricIdMask1)
    if (df_indirect_read_broadcast(0, 1, 0x20C, &fabric_id_mask1))
    return -EINVAL;
    df3_get_masks_shifts(fabric_id_mask0, fabric_id_mask1);
    return 0;
    }
// Default to DF2.
    df_cfg.rev = DF2;
    df2_get_masks_shifts(fabric_id_mask0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn determine_df_rev() -> c_int {
    static int determine_df_rev(void)
    {
    u32 reg;
    u8 rev;
    if (df_cfg.rev != UNKNOWN)
    return 0;
// Read D18F0x40 (FabricBlockInstanceCount).
    if (df_indirect_read_broadcast(0, 0, 0x40, &reg))
    return -EINVAL;
//
// Revision fields added for DF4 and later.
//
// Major revision of '0' is found pre-DF4. Field is Read-as-Zero.
//
    rev = FIELD_GET(DF_MAJOR_REVISION, reg);
    if (!rev)
    return determine_df_rev_legacy();
    if (rev == 4)
    return df4_determine_df_rev(reg);
// All other systems should have PRM handlers.
    if (!acpi_prm_handler_available(&norm_to_sys_guid)) {
    pr_debug("PRM not available\n");
    return -ENODEV;
    }
    df_cfg.flags.prm_only = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_dram_hole_base() -> c_int {
    static int get_dram_hole_base(void)
    {
    let mut func: u8 = 0;
    if (df_cfg.rev >= DF4)
    func = 7;
    if (df_indirect_read_broadcast(0, func, 0x104, &df_cfg.dram_hole_base))
    return -EINVAL;
    df_cfg.dram_hole_base &= DF_DRAM_HOLE_BASE_MASK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_num_maps() {
    static void get_num_maps(void)
    {
    switch (df_cfg.rev) {
    case DF2:
    case DF3:
    case DF3p5:
    df_cfg.num_coh_st_maps	= 2;
    break;
    case DF4:
    case DF4p5:
    df_cfg.num_coh_st_maps	= 4;
    break;
    default:
    atl_debug_on_bad_df_rev();
    }
    }
#[no_mangle]
unsafe extern "C" fn apply_node_id_shift() {
    static void apply_node_id_shift(void)
    {
    if (df_cfg.rev == DF2)
    return;
    df_cfg.die_id_shift		= df_cfg.node_id_shift;
    df_cfg.die_id_mask		<<= df_cfg.node_id_shift;
    df_cfg.socket_id_mask		<<= df_cfg.node_id_shift;
    df_cfg.socket_id_shift		+= df_cfg.node_id_shift;
    }
#[no_mangle]
unsafe extern "C" fn dump_df_cfg() {
    static void dump_df_cfg(void)
    {
    pr_debug("rev=0x%x",				df_cfg.rev);
    pr_debug("component_id_mask=0x%x",		df_cfg.component_id_mask);
    pr_debug("die_id_mask=0x%x",			df_cfg.die_id_mask);
    pr_debug("node_id_mask=0x%x",			df_cfg.node_id_mask);
    pr_debug("socket_id_mask=0x%x",			df_cfg.socket_id_mask);
    pr_debug("die_id_shift=0x%x",			df_cfg.die_id_shift);
    pr_debug("node_id_shift=0x%x",			df_cfg.node_id_shift);
    pr_debug("socket_id_shift=0x%x",		df_cfg.socket_id_shift);
    pr_debug("num_coh_st_maps=%u",			df_cfg.num_coh_st_maps);
    pr_debug("dram_hole_base=0x%x",			df_cfg.dram_hole_base);
    pr_debug("flags.legacy_ficaa=%u",		df_cfg.flags.legacy_ficaa);
    pr_debug("flags.socket_id_shift_quirk=%u",	df_cfg.flags.socket_id_shift_quirk);
    }
#[no_mangle]
pub unsafe extern "C" fn get_df_system_info() -> c_int {
    int get_df_system_info(void)
    {
    int ret;
    ret = determine_df_rev();
    if (ret) {
    pr_warn("Failed to determine DF Revision");
    df_cfg.rev = UNKNOWN;
    return ret;
    }
    if (df_cfg.flags.prm_only)
    return 0;
    apply_node_id_shift();
    get_num_maps();
    if (get_dram_hole_base())
    pr_warn("Failed to read DRAM hole base");
    dump_df_cfg();
    return 0;
    }
