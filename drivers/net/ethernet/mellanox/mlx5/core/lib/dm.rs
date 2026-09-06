//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/dm.c
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
// Copyright (c) 2019 Mellanox Technologies

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_dm {
// protect access to icm bitmask
    pub lock: spinlock_t,
    pub steering_sw_icm_alloc_blocks: *mut c_ulong,
    pub header_modify_sw_icm_alloc_blocks: *mut c_ulong,
    pub header_modify_pattern_sw_icm_alloc_blocks: *mut c_ulong,
    pub header_encap_sw_icm_alloc_blocks: *mut c_ulong,
}

    struct mlx5_dm *mlx5_dm_create(struct mlx5_core_dev *dev)
    {
    let mut header_modify_pattern_icm_blocks: u64 = 0;
    let mut header_sw_encap_icm_blocks: u64 = 0;
    let mut header_modify_icm_blocks: u64 = 0;
    let mut steering_icm_blocks: u64 = 0;
    struct mlx5_dm *dm;
    bool support_v2;
    if (!(MLX5_CAP_GEN_64(dev, general_obj_types) & MLX5_GENERAL_OBJ_TYPES_CAP_SW_ICM))
    return core::ptr::null_mut();
    dm = kzalloc_obj(*dm);
    if (!dm)
    return core::ptr::null_mut();
    spin_lock_init(&dm.lock);
    if (MLX5_CAP64_DEV_MEM(dev, steering_sw_icm_start_address)) {
    steering_icm_blocks =
    BIT(MLX5_CAP_DEV_MEM(dev, log_steering_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev));
    dm.steering_sw_icm_alloc_blocks =
    bitmap_zalloc(steering_icm_blocks, GFP_KERNEL);
    if (!dm.steering_sw_icm_alloc_blocks)
    goto err_steering;
    }
    if (MLX5_CAP64_DEV_MEM(dev, header_modify_sw_icm_start_address)) {
    header_modify_icm_blocks =
    BIT(MLX5_CAP_DEV_MEM(dev, log_header_modify_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev));
    dm.header_modify_sw_icm_alloc_blocks =
    bitmap_zalloc(header_modify_icm_blocks, GFP_KERNEL);
    if (!dm.header_modify_sw_icm_alloc_blocks)
    goto err_modify_hdr;
    }
    if (MLX5_CAP_DEV_MEM(dev, log_indirect_encap_sw_icm_size)) {
    header_sw_encap_icm_blocks =
    BIT(MLX5_CAP_DEV_MEM(dev, log_indirect_encap_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev));
    dm.header_encap_sw_icm_alloc_blocks =
    bitmap_zalloc(header_sw_encap_icm_blocks, GFP_KERNEL);
    if (!dm.header_encap_sw_icm_alloc_blocks)
    goto err_pattern;
    }
    support_v2 = MLX5_CAP_FLOWTABLE_NIC_RX(dev, sw_owner_v2) &&
    MLX5_CAP_FLOWTABLE_NIC_TX(dev, sw_owner_v2) &&
    MLX5_CAP64_DEV_MEM(dev, header_modify_pattern_sw_icm_start_address);
    if (support_v2) {
    header_modify_pattern_icm_blocks =
    BIT(MLX5_CAP_DEV_MEM(dev, log_header_modify_pattern_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev));
    dm.header_modify_pattern_sw_icm_alloc_blocks =
    bitmap_zalloc(header_modify_pattern_icm_blocks, GFP_KERNEL);
    if (!dm.header_modify_pattern_sw_icm_alloc_blocks)
    goto err_sw_encap;
    }
    return dm;
    err_sw_encap:
    bitmap_free(dm.header_encap_sw_icm_alloc_blocks);
    err_pattern:
    bitmap_free(dm.header_modify_sw_icm_alloc_blocks);
    err_modify_hdr:
    bitmap_free(dm.steering_sw_icm_alloc_blocks);
    err_steering:
    kfree(dm);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_dm_cleanup(dev: *mut mlx5_core_dev) {
    void mlx5_dm_cleanup(struct mlx5_core_dev *dev)
    {
    struct mlx5_dm *dm = dev.dm;
    if (!dev.dm)
    return;
    if (dm.steering_sw_icm_alloc_blocks) {
    WARN_ON(!bitmap_empty(dm.steering_sw_icm_alloc_blocks,
    BIT(MLX5_CAP_DEV_MEM(dev, log_steering_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev))));
    bitmap_free(dm.steering_sw_icm_alloc_blocks);
    }
    if (dm.header_modify_sw_icm_alloc_blocks) {
    WARN_ON(!bitmap_empty(dm.header_modify_sw_icm_alloc_blocks,
    BIT(MLX5_CAP_DEV_MEM(dev,
    log_header_modify_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev))));
    bitmap_free(dm.header_modify_sw_icm_alloc_blocks);
    }
    if (dm.header_encap_sw_icm_alloc_blocks) {
    WARN_ON(!bitmap_empty(dm.header_encap_sw_icm_alloc_blocks,
    BIT(MLX5_CAP_DEV_MEM(dev,
    log_indirect_encap_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev))));
    bitmap_free(dm.header_encap_sw_icm_alloc_blocks);
    }
    if (dm.header_modify_pattern_sw_icm_alloc_blocks) {
    WARN_ON(!bitmap_empty(dm.header_modify_pattern_sw_icm_alloc_blocks,
    BIT(MLX5_CAP_DEV_MEM(dev,
    log_header_modify_pattern_sw_icm_size) -
    MLX5_LOG_SW_ICM_BLOCK_SIZE(dev))));
    bitmap_free(dm.header_modify_pattern_sw_icm_alloc_blocks);
    }
    kfree(dm);
    }
    int mlx5_dm_sw_icm_alloc(struct mlx5_core_dev *dev, enum mlx5_sw_icm_type type,
    u64 length, u32 log_alignment, u16 uid,
    phys_addr_t *addr, u32 *obj_id)
    {
    let mut num_blocks: u32 = DIV_ROUND_UP_ULL(length, MLX5_SW_ICM_BLOCK_SIZE(dev));
    u32 out[MLX5_ST_SZ_DW(general_obj_out_cmd_hdr)] = {};
    u32 in[MLX5_ST_SZ_DW(create_sw_icm_in)] = {};
    struct mlx5_dm *dm = dev.dm;
    unsigned long *block_map;
    u64 icm_start_addr;
    u32 log_icm_size;
    u64 align_mask;
    u32 max_blocks;
    u64 block_idx;
    void *sw_icm;
    int ret;
    if (!dev.dm)
    return -EOPNOTSUPP;
    if (!length || (length & (length - 1)) ||
    length & (MLX5_SW_ICM_BLOCK_SIZE(dev) - 1))
    return -EINVAL;
    MLX5_SET(general_obj_in_cmd_hdr, in, opcode,
    MLX5_CMD_OP_CREATE_GENERAL_OBJECT);
    MLX5_SET(general_obj_in_cmd_hdr, in, obj_type, MLX5_OBJ_TYPE_SW_ICM);
    MLX5_SET(general_obj_in_cmd_hdr, in, uid, uid);
    switch (type) {
    case MLX5_SW_ICM_TYPE_STEERING:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev, steering_sw_icm_start_address);
    log_icm_size = MLX5_CAP_DEV_MEM(dev, log_steering_sw_icm_size);
    block_map = dm.steering_sw_icm_alloc_blocks;
    break;
    case MLX5_SW_ICM_TYPE_HEADER_MODIFY:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev, header_modify_sw_icm_start_address);
    log_icm_size = MLX5_CAP_DEV_MEM(dev,
    log_header_modify_sw_icm_size);
    block_map = dm.header_modify_sw_icm_alloc_blocks;
    break;
    case MLX5_SW_ICM_TYPE_HEADER_MODIFY_PATTERN:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev,
    header_modify_pattern_sw_icm_start_address);
    log_icm_size = MLX5_CAP_DEV_MEM(dev,
    log_header_modify_pattern_sw_icm_size);
    block_map = dm.header_modify_pattern_sw_icm_alloc_blocks;
    break;
    case MLX5_SW_ICM_TYPE_SW_ENCAP:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev,
    indirect_encap_sw_icm_start_address);
    log_icm_size = MLX5_CAP_DEV_MEM(dev,
    log_indirect_encap_sw_icm_size);
    block_map = dm.header_encap_sw_icm_alloc_blocks;
    break;
    default:
    return -EINVAL;
    }
    if (!block_map)
    return -EOPNOTSUPP;
    max_blocks = BIT(log_icm_size - MLX5_LOG_SW_ICM_BLOCK_SIZE(dev));
    if (log_alignment < MLX5_LOG_SW_ICM_BLOCK_SIZE(dev))
    log_alignment = MLX5_LOG_SW_ICM_BLOCK_SIZE(dev);
    align_mask = BIT(log_alignment - MLX5_LOG_SW_ICM_BLOCK_SIZE(dev)) - 1;
    spin_lock(&dm.lock);
    block_idx = bitmap_find_next_zero_area(block_map, max_blocks, 0,
    num_blocks, align_mask);
    if (block_idx < max_blocks)
    bitmap_set(block_map,
    block_idx, num_blocks);
    spin_unlock(&dm.lock);
    if (block_idx >= max_blocks)
    return -ENOMEM;
    sw_icm = MLX5_ADDR_OF(create_sw_icm_in, in, sw_icm);
    icm_start_addr += block_idx << MLX5_LOG_SW_ICM_BLOCK_SIZE(dev);
    MLX5_SET64(sw_icm, sw_icm, sw_icm_start_addr,
    icm_start_addr);
    MLX5_SET(sw_icm, sw_icm, log_sw_icm_size, ilog2(length));
    ret = mlx5_cmd_exec(dev, in, sizeof(in), out, sizeof(out));
    if (ret) {
    spin_lock(&dm.lock);
    bitmap_clear(block_map,
    block_idx, num_blocks);
    spin_unlock(&dm.lock);
    return ret;
    }
// addr = icm_start_addr;
// obj_id = MLX5_GET(general_obj_out_cmd_hdr, out, obj_id);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mlx5_dm_sw_icm_alloc);
    int mlx5_dm_sw_icm_dealloc(struct mlx5_core_dev *dev, enum mlx5_sw_icm_type type,
    u64 length, u16 uid, phys_addr_t addr, u32 obj_id)
    {
    let mut num_blocks: u32 = DIV_ROUND_UP_ULL(length, MLX5_SW_ICM_BLOCK_SIZE(dev));
    u32 out[MLX5_ST_SZ_DW(general_obj_out_cmd_hdr)] = {};
    u32 in[MLX5_ST_SZ_DW(general_obj_in_cmd_hdr)] = {};
    struct mlx5_dm *dm = dev.dm;
    unsigned long *block_map;
    u64 icm_start_addr;
    u64 start_idx;
    int err;
    if (!dev.dm)
    return -EOPNOTSUPP;
    switch (type) {
    case MLX5_SW_ICM_TYPE_STEERING:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev, steering_sw_icm_start_address);
    block_map = dm.steering_sw_icm_alloc_blocks;
    break;
    case MLX5_SW_ICM_TYPE_HEADER_MODIFY:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev, header_modify_sw_icm_start_address);
    block_map = dm.header_modify_sw_icm_alloc_blocks;
    break;
    case MLX5_SW_ICM_TYPE_HEADER_MODIFY_PATTERN:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev,
    header_modify_pattern_sw_icm_start_address);
    block_map = dm.header_modify_pattern_sw_icm_alloc_blocks;
    break;
    case MLX5_SW_ICM_TYPE_SW_ENCAP:
    icm_start_addr = MLX5_CAP64_DEV_MEM(dev,
    indirect_encap_sw_icm_start_address);
    block_map = dm.header_encap_sw_icm_alloc_blocks;
    break;
    default:
    return -EINVAL;
    }
    MLX5_SET(general_obj_in_cmd_hdr, in, opcode,
    MLX5_CMD_OP_DESTROY_GENERAL_OBJECT);
    MLX5_SET(general_obj_in_cmd_hdr, in, obj_type, MLX5_OBJ_TYPE_SW_ICM);
    MLX5_SET(general_obj_in_cmd_hdr, in, obj_id, obj_id);
    MLX5_SET(general_obj_in_cmd_hdr, in, uid, uid);
    err =  mlx5_cmd_exec(dev, in, sizeof(in), out, sizeof(out));
    if (err)
    return err;
    start_idx = (addr - icm_start_addr) >> MLX5_LOG_SW_ICM_BLOCK_SIZE(dev);
    spin_lock(&dm.lock);
    bitmap_clear(block_map,
    start_idx, num_blocks);
    spin_unlock(&dm.lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mlx5_dm_sw_icm_dealloc);
