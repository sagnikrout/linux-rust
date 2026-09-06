//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_fw.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2019 Mellanox Technologies.

    struct mlx5dr_fw_recalc_cs_ft *
    mlx5dr_fw_create_recalc_cs_ft(struct mlx5dr_domain *dmn, u16 vport_num)
    {
    let mut ft_attr: mlx5dr_cmd_create_flow_table_attr = {};
    struct mlx5dr_fw_recalc_cs_ft *recalc_cs_ft;
    u32 table_id, group_id, modify_hdr_id;
    u64 rx_icm_addr, modify_ttl_action;
    int ret;
    recalc_cs_ft = kzalloc_obj(*recalc_cs_ft);
    if (!recalc_cs_ft)
    return core::ptr::null_mut();
    ft_attr.table_type = MLX5_FLOW_TABLE_TYPE_FDB;
    ft_attr.level = dmn.info.caps.max_ft_level - 1;
    ft_attr.term_tbl = true;
    ret = mlx5dr_cmd_create_flow_table(dmn.mdev,
    &ft_attr,
    &rx_icm_addr,
    &table_id);
    if (ret) {
    mlx5dr_err(dmn, "Failed creating TTL W/A FW flow table %d\n", ret);
    goto free_ttl_tbl;
    }
    ret = mlx5dr_cmd_create_empty_flow_group(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
    table_id, &group_id);
    if (ret) {
    mlx5dr_err(dmn, "Failed creating TTL W/A FW flow group %d\n", ret);
    goto destroy_flow_table;
    }
// Modify TTL action by adding zero to trigger CS recalculation
    modify_ttl_action = 0;
    MLX5_SET(set_action_in, &modify_ttl_action, action_type, MLX5_ACTION_TYPE_ADD);
    MLX5_SET(set_action_in, &modify_ttl_action, field, MLX5_ACTION_IN_FIELD_OUT_IP_TTL);
    ret = mlx5dr_cmd_alloc_modify_header(dmn.mdev, MLX5_FLOW_TABLE_TYPE_FDB, 1,
    &modify_ttl_action,
    &modify_hdr_id);
    if (ret) {
    mlx5dr_err(dmn, "Failed modify header TTL %d\n", ret);
    goto destroy_flow_group;
    }
    ret = mlx5dr_cmd_set_fte_modify_and_vport(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
    table_id, group_id, modify_hdr_id,
    vport_num);
    if (ret) {
    mlx5dr_err(dmn, "Failed setting TTL W/A flow table entry %d\n", ret);
    goto dealloc_modify_header;
    }
    recalc_cs_ft.modify_hdr_id = modify_hdr_id;
    recalc_cs_ft.rx_icm_addr = rx_icm_addr;
    recalc_cs_ft.table_id = table_id;
    recalc_cs_ft.group_id = group_id;
    return recalc_cs_ft;
    dealloc_modify_header:
    mlx5dr_cmd_dealloc_modify_header(dmn.mdev, modify_hdr_id);
    destroy_flow_group:
    mlx5dr_cmd_destroy_flow_group(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
    table_id, group_id);
    destroy_flow_table:
    mlx5dr_cmd_destroy_flow_table(dmn.mdev, table_id, MLX5_FLOW_TABLE_TYPE_FDB);
    free_ttl_tbl:
    kfree(recalc_cs_ft);
    return core::ptr::null_mut();
    }
    void mlx5dr_fw_destroy_recalc_cs_ft(struct mlx5dr_domain *dmn,
    struct mlx5dr_fw_recalc_cs_ft *recalc_cs_ft)
    {
    mlx5dr_cmd_del_flow_table_entry(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
    recalc_cs_ft.table_id);
    mlx5dr_cmd_dealloc_modify_header(dmn.mdev, recalc_cs_ft.modify_hdr_id);
    mlx5dr_cmd_destroy_flow_group(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
    recalc_cs_ft.table_id,
    recalc_cs_ft.group_id);
    mlx5dr_cmd_destroy_flow_table(dmn.mdev,
    recalc_cs_ft.table_id,
    MLX5_FLOW_TABLE_TYPE_FDB);
    kfree(recalc_cs_ft);
    }
    int mlx5dr_fw_create_md_tbl(struct mlx5dr_domain *dmn,
    struct mlx5dr_cmd_flow_destination_hw_info *dest,
    int num_dest,
    bool reformat_req,
    u32 *tbl_id,
    u32 *group_id,
    bool ignore_flow_level,
    u32 flow_source)
    {
    let mut ft_attr: mlx5dr_cmd_create_flow_table_attr = {};
    let mut fte_info: mlx5dr_cmd_fte_info = {};
    u32 val[MLX5_ST_SZ_DW_MATCH_PARAM] = {};
    let mut ft_info: mlx5dr_cmd_ft_info = {};
    int ret;
    ft_attr.table_type = MLX5_FLOW_TABLE_TYPE_FDB;
    ft_attr.level = min_t(int, dmn.info.caps.max_ft_level - 2,
    MLX5_FT_MAX_MULTIPATH_LEVEL);
    ft_attr.reformat_en = reformat_req;
    ft_attr.decap_en = reformat_req;
    ret = mlx5dr_cmd_create_flow_table(dmn.mdev, &ft_attr, core::ptr::null_mut(), tbl_id);
    if (ret) {
    mlx5dr_err(dmn, "Failed creating multi dest FW flow table %d\n", ret);
    return ret;
    }
    ret = mlx5dr_cmd_create_empty_flow_group(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
// tbl_id, group_id);
    if (ret) {
    mlx5dr_err(dmn, "Failed creating multi dest FW flow group %d\n", ret);
    goto free_flow_table;
    }
    ft_info.id = *tbl_id;
    ft_info.type = FS_FT_FDB;
    fte_info.action.action = MLX5_FLOW_CONTEXT_ACTION_FWD_DEST;
    fte_info.dests_size = num_dest;
    fte_info.val = val;
    fte_info.dest_arr = dest;
    fte_info.ignore_flow_level = ignore_flow_level;
    fte_info.flow_context.flow_source = flow_source;
    ret = mlx5dr_cmd_set_fte(dmn.mdev, 0, 0, &ft_info, *group_id, &fte_info);
    if (ret) {
    mlx5dr_err(dmn, "Failed setting fte into table %d\n", ret);
    goto free_flow_group;
    }
    return 0;
    free_flow_group:
    mlx5dr_cmd_destroy_flow_group(dmn.mdev, MLX5_FLOW_TABLE_TYPE_FDB,
// tbl_id, *group_id);
    free_flow_table:
    mlx5dr_cmd_destroy_flow_table(dmn.mdev, *tbl_id,
    MLX5_FLOW_TABLE_TYPE_FDB);
    return ret;
    }
    void mlx5dr_fw_destroy_md_tbl(struct mlx5dr_domain *dmn,
    u32 tbl_id, u32 group_id)
    {
    mlx5dr_cmd_del_flow_table_entry(dmn.mdev, FS_FT_FDB, tbl_id);
    mlx5dr_cmd_destroy_flow_group(dmn.mdev,
    MLX5_FLOW_TABLE_TYPE_FDB,
    tbl_id, group_id);
    mlx5dr_cmd_destroy_flow_table(dmn.mdev, tbl_id,
    MLX5_FLOW_TABLE_TYPE_FDB);
    }
