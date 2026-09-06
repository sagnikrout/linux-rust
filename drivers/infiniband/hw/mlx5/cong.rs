//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx5/cong.c
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
// Copyright (c) 2013-2017, Mellanox Technologies. All rights reserved.
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

    enum mlx5_ib_cong_node_type {
    MLX5_IB_RROCE_ECN_RP = 1,
    MLX5_IB_RROCE_ECN_NP = 2,
    MLX5_IB_RROCE_GENERAL = 3,
    };
    static const char * const mlx5_ib_dbg_cc_name[] = {
    "rp_clamp_tgt_rate",
    "rp_clamp_tgt_rate_ati",
    "rp_time_reset",
    "rp_byte_reset",
    "rp_threshold",
    "rp_ai_rate",
    "rp_max_rate",
    "rp_hai_rate",
    "rp_min_dec_fac",
    "rp_min_rate",
    "rp_rate_to_set_on_first_cnp",
    "rp_dce_tcp_g",
    "rp_dce_tcp_rtt",
    "rp_rate_reduce_monitor_period",
    "rp_initial_alpha_value",
    "rp_gd",
    "np_min_time_between_cnps",
    "np_cnp_dscp",
    "np_cnp_prio_mode",
    "np_cnp_prio",
    "rtt_resp_dscp_valid",
    "rtt_resp_dscp",
    };

    static enum mlx5_ib_cong_node_type
    mlx5_ib_param_to_node(enum mlx5_ib_dbg_cc_types param_offset)
    {
    if (param_offset <= MLX5_IB_DBG_CC_RP_GD)
    return MLX5_IB_RROCE_ECN_RP;
    if (param_offset <= MLX5_IB_DBG_CC_NP_CNP_PRIO)
    return MLX5_IB_RROCE_ECN_NP;
    return MLX5_IB_RROCE_GENERAL;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_get_cc_param_val(field: *mut c_void, offset: c_int) -> u32 {
    static u32 mlx5_get_cc_param_val(void *field, int offset)
    {
    switch (offset) {
    case MLX5_IB_DBG_CC_RP_CLAMP_TGT_RATE:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    clamp_tgt_rate);
    case MLX5_IB_DBG_CC_RP_CLAMP_TGT_RATE_ATI:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    clamp_tgt_rate_after_time_inc);
    case MLX5_IB_DBG_CC_RP_TIME_RESET:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_time_reset);
    case MLX5_IB_DBG_CC_RP_BYTE_RESET:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_byte_reset);
    case MLX5_IB_DBG_CC_RP_THRESHOLD:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_threshold);
    case MLX5_IB_DBG_CC_RP_AI_RATE:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_ai_rate);
    case MLX5_IB_DBG_CC_RP_MAX_RATE:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_max_rate);
    case MLX5_IB_DBG_CC_RP_HAI_RATE:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_hai_rate);
    case MLX5_IB_DBG_CC_RP_MIN_DEC_FAC:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_min_dec_fac);
    case MLX5_IB_DBG_CC_RP_MIN_RATE:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_min_rate);
    case MLX5_IB_DBG_CC_RP_RATE_TO_SET_ON_FIRST_CNP:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rate_to_set_on_first_cnp);
    case MLX5_IB_DBG_CC_RP_DCE_TCP_G:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    dce_tcp_g);
    case MLX5_IB_DBG_CC_RP_DCE_TCP_RTT:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    dce_tcp_rtt);
    case MLX5_IB_DBG_CC_RP_RATE_REDUCE_MONITOR_PERIOD:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rate_reduce_monitor_period);
    case MLX5_IB_DBG_CC_RP_INITIAL_ALPHA_VALUE:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    initial_alpha_value);
    case MLX5_IB_DBG_CC_RP_GD:
    return MLX5_GET(cong_control_r_roce_ecn_rp, field,
    rpg_gd);
    case MLX5_IB_DBG_CC_NP_MIN_TIME_BETWEEN_CNPS:
    return MLX5_GET(cong_control_r_roce_ecn_np, field,
    min_time_between_cnps);
    case MLX5_IB_DBG_CC_NP_CNP_DSCP:
    return MLX5_GET(cong_control_r_roce_ecn_np, field,
    cnp_dscp);
    case MLX5_IB_DBG_CC_NP_CNP_PRIO_MODE:
    return MLX5_GET(cong_control_r_roce_ecn_np, field,
    cnp_prio_mode);
    case MLX5_IB_DBG_CC_NP_CNP_PRIO:
    return MLX5_GET(cong_control_r_roce_ecn_np, field,
    cnp_802p_prio);
    case MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP_VALID:
    return MLX5_GET(cong_control_r_roce_general, field,
    rtt_resp_dscp_valid);
    case MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP:
    return MLX5_GET(cong_control_r_roce_general, field,
    rtt_resp_dscp);
    default:
    return 0;
    }
    }
    static void mlx5_ib_set_cc_param_mask_val(void *field, int offset,
    u32 var, u32 *attr_mask)
    {
    switch (offset) {
    case MLX5_IB_DBG_CC_RP_CLAMP_TGT_RATE:
// attr_mask |= MLX5_IB_RP_CLAMP_TGT_RATE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    clamp_tgt_rate, var);
    break;
    case MLX5_IB_DBG_CC_RP_CLAMP_TGT_RATE_ATI:
// attr_mask |= MLX5_IB_RP_CLAMP_TGT_RATE_ATI_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    clamp_tgt_rate_after_time_inc, var);
    break;
    case MLX5_IB_DBG_CC_RP_TIME_RESET:
// attr_mask |= MLX5_IB_RP_TIME_RESET_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_time_reset, var);
    break;
    case MLX5_IB_DBG_CC_RP_BYTE_RESET:
// attr_mask |= MLX5_IB_RP_BYTE_RESET_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_byte_reset, var);
    break;
    case MLX5_IB_DBG_CC_RP_THRESHOLD:
// attr_mask |= MLX5_IB_RP_THRESHOLD_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_threshold, var);
    break;
    case MLX5_IB_DBG_CC_RP_AI_RATE:
// attr_mask |= MLX5_IB_RP_AI_RATE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_ai_rate, var);
    break;
    case MLX5_IB_DBG_CC_RP_MAX_RATE:
// attr_mask |= MLX5_IB_RP_MAX_RATE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_max_rate, var);
    break;
    case MLX5_IB_DBG_CC_RP_HAI_RATE:
// attr_mask |= MLX5_IB_RP_HAI_RATE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_hai_rate, var);
    break;
    case MLX5_IB_DBG_CC_RP_MIN_DEC_FAC:
// attr_mask |= MLX5_IB_RP_MIN_DEC_FAC_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_min_dec_fac, var);
    break;
    case MLX5_IB_DBG_CC_RP_MIN_RATE:
// attr_mask |= MLX5_IB_RP_MIN_RATE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_min_rate, var);
    break;
    case MLX5_IB_DBG_CC_RP_RATE_TO_SET_ON_FIRST_CNP:
// attr_mask |= MLX5_IB_RP_RATE_TO_SET_ON_FIRST_CNP_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rate_to_set_on_first_cnp, var);
    break;
    case MLX5_IB_DBG_CC_RP_DCE_TCP_G:
// attr_mask |= MLX5_IB_RP_DCE_TCP_G_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    dce_tcp_g, var);
    break;
    case MLX5_IB_DBG_CC_RP_DCE_TCP_RTT:
// attr_mask |= MLX5_IB_RP_DCE_TCP_RTT_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    dce_tcp_rtt, var);
    break;
    case MLX5_IB_DBG_CC_RP_RATE_REDUCE_MONITOR_PERIOD:
// attr_mask |= MLX5_IB_RP_RATE_REDUCE_MONITOR_PERIOD_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rate_reduce_monitor_period, var);
    break;
    case MLX5_IB_DBG_CC_RP_INITIAL_ALPHA_VALUE:
// attr_mask |= MLX5_IB_RP_INITIAL_ALPHA_VALUE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    initial_alpha_value, var);
    break;
    case MLX5_IB_DBG_CC_RP_GD:
// attr_mask |= MLX5_IB_RP_GD_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_rp, field,
    rpg_gd, var);
    break;
    case MLX5_IB_DBG_CC_NP_MIN_TIME_BETWEEN_CNPS:
// attr_mask |= MLX5_IB_NP_MIN_TIME_BETWEEN_CNPS_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_np, field,
    min_time_between_cnps, var);
    break;
    case MLX5_IB_DBG_CC_NP_CNP_DSCP:
// attr_mask |= MLX5_IB_NP_CNP_DSCP_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_np, field, cnp_dscp, var);
    break;
    case MLX5_IB_DBG_CC_NP_CNP_PRIO_MODE:
// attr_mask |= MLX5_IB_NP_CNP_PRIO_MODE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_np, field, cnp_prio_mode, var);
    break;
    case MLX5_IB_DBG_CC_NP_CNP_PRIO:
// attr_mask |= MLX5_IB_NP_CNP_PRIO_MODE_ATTR;
    MLX5_SET(cong_control_r_roce_ecn_np, field, cnp_prio_mode, 0);
    MLX5_SET(cong_control_r_roce_ecn_np, field, cnp_802p_prio, var);
    break;
    case MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP_VALID:
// attr_mask |= MLX5_IB_GENERAL_RTT_RESP_DSCP_ATTR;
    MLX5_SET(cong_control_r_roce_general, field, rtt_resp_dscp_valid, var);
    break;
    case MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP:
// attr_mask |= MLX5_IB_GENERAL_RTT_RESP_DSCP_ATTR;
    MLX5_SET(cong_control_r_roce_general, field, rtt_resp_dscp_valid, 1);
    MLX5_SET(cong_control_r_roce_general, field, rtt_resp_dscp, var);
    break;
    }
    }
    static int mlx5_ib_get_cc_params(struct mlx5_ib_dev *dev, u32 port_num,
    int offset, u32 *var)
    {
    let mut outlen: c_int = MLX5_ST_SZ_BYTES(query_cong_params_out);
    void *out;
    void *field;
    int err;
    enum mlx5_ib_cong_node_type node;
    struct mlx5_core_dev *mdev;
// Takes a 1-based port number
    mdev = mlx5_ib_get_native_port_mdev(dev, port_num + 1, core::ptr::null_mut());
    if (!mdev)
    return -ENODEV;
    out = kvzalloc(outlen, GFP_KERNEL);
    if (!out) {
    err = -ENOMEM;
    goto alloc_err;
    }
    node = mlx5_ib_param_to_node(offset);
    err = mlx5_cmd_query_cong_params(mdev, node, out);
    if (err)
    goto free;
    field = MLX5_ADDR_OF(query_cong_params_out, out, congestion_parameters);
// var = mlx5_get_cc_param_val(field, offset);
    free:
    kvfree(out);
    alloc_err:
    mlx5_ib_put_native_port_mdev(dev, port_num + 1);
    return err;
    }
    static int mlx5_ib_set_cc_params(struct mlx5_ib_dev *dev, u32 port_num,
    int offset, u32 var)
    {
    let mut inlen: c_int = MLX5_ST_SZ_BYTES(modify_cong_params_in);
    void *in;
    void *field;
    enum mlx5_ib_cong_node_type node;
    struct mlx5_core_dev *mdev;
    let mut attr_mask: u32 = 0;
    int err;
// Takes a 1-based port number
    mdev = mlx5_ib_get_native_port_mdev(dev, port_num + 1, core::ptr::null_mut());
    if (!mdev)
    return -ENODEV;
    in = kvzalloc(inlen, GFP_KERNEL);
    if (!in) {
    err = -ENOMEM;
    goto alloc_err;
    }
    MLX5_SET(modify_cong_params_in, in, opcode,
    MLX5_CMD_OP_MODIFY_CONG_PARAMS);
    node = mlx5_ib_param_to_node(offset);
    MLX5_SET(modify_cong_params_in, in, cong_protocol, node);
    field = MLX5_ADDR_OF(modify_cong_params_in, in, congestion_parameters);
    mlx5_ib_set_cc_param_mask_val(field, offset, var, &attr_mask);
    field = MLX5_ADDR_OF(modify_cong_params_in, in, field_select);
    MLX5_SET(field_select_r_roce_rp, field, field_select_r_roce_rp,
    attr_mask);
    err = mlx5_cmd_exec_in(mdev, modify_cong_params, in);
    kvfree(in);
    alloc_err:
    mlx5_ib_put_native_port_mdev(dev, port_num + 1);
    return err;
    }
    static ssize_t set_param(struct file *filp, const char __user *buf,
    size_t count, loff_t *pos)
    {
    struct mlx5_ib_dbg_param *param = filp.private_data;
    let mut offset: c_int = param.offset;
    u32 var;
    int ret;
    ret = kstrtou32_from_user(buf, count, 0, &var);
    if (!ret)
    ret = mlx5_ib_set_cc_params(param.dev, param.port_num, offset, var);
    return ret ? ret : count;
    }
    static ssize_t get_param(struct file *filp, char __user *buf, size_t count,
    loff_t *pos)
    {
    struct mlx5_ib_dbg_param *param = filp.private_data;
    let mut offset: c_int = param.offset;
    let mut var: u32 = 0;
    int ret;
    char lbuf[12];
    ret = mlx5_ib_get_cc_params(param.dev, param.port_num, offset, &var);
    if (ret)
    return ret;
    ret = scnprintf(lbuf, sizeof(lbuf), "%u\n", var);
    return simple_read_from_buffer(buf, count, pos, lbuf, ret);
    }
    static const struct file_operations dbg_cc_fops = {
    .owner	= THIS_MODULE,
    .open	= simple_open,
    .write	= set_param,
    .read	= get_param,
    };
#[no_mangle]
pub unsafe extern "C" fn mlx5_ib_cleanup_cong_debugfs(dev: *mut mlx5_ib_dev, port_num: u32) {
    void mlx5_ib_cleanup_cong_debugfs(struct mlx5_ib_dev *dev, u32 port_num)
    {
    if (!mlx5_debugfs_root ||
    !dev.port[port_num].dbg_cc_params ||
    !dev.port[port_num].dbg_cc_params.root)
    return;
    debugfs_remove_recursive(dev.port[port_num].dbg_cc_params.root);
    kfree(dev.port[port_num].dbg_cc_params);
    dev.port[port_num].dbg_cc_params = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_ib_init_cong_debugfs(dev: *mut mlx5_ib_dev, port_num: u32) {
    void mlx5_ib_init_cong_debugfs(struct mlx5_ib_dev *dev, u32 port_num)
    {
    struct mlx5_ib_dbg_cc_params *dbg_cc_params;
    struct mlx5_core_dev *mdev;
    int i;
    if (!mlx5_debugfs_root)
    return;
// Takes a 1-based port number
    mdev = mlx5_ib_get_native_port_mdev(dev, port_num + 1, core::ptr::null_mut());
    if (!mdev)
    return;
    if (!MLX5_CAP_GEN(mdev, cc_query_allowed) ||
    !MLX5_CAP_GEN(mdev, cc_modify_allowed))
    goto put_mdev;
    dbg_cc_params = kzalloc_obj(*dbg_cc_params);
    if (!dbg_cc_params)
    goto err;
    dev.port[port_num].dbg_cc_params = dbg_cc_params;
    dbg_cc_params.root = debugfs_create_dir("cc_params", mlx5_debugfs_get_dev_root(mdev));
    for (i = 0; i < MLX5_IB_DBG_CC_MAX; i++) {
    if ((i == MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP_VALID ||
    i == MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP))
    if (!MLX5_CAP_GEN(mdev, roce) ||
    !MLX5_CAP_ROCE(mdev, roce_cc_general))
    continue;
    dbg_cc_params.params[i].offset = i;
    dbg_cc_params.params[i].dev = dev;
    dbg_cc_params.params[i].port_num = port_num;
    dbg_cc_params.params[i].dentry =
    debugfs_create_file(mlx5_ib_dbg_cc_name[i],
    0600, dbg_cc_params.root,
    &dbg_cc_params.params[i],
    &dbg_cc_fops);
    }
    put_mdev:
    mlx5_ib_put_native_port_mdev(dev, port_num + 1);
    return;
    err:
    mlx5_ib_warn(dev, "cong debugfs failure\n");
    mlx5_ib_cleanup_cong_debugfs(dev, port_num);
    mlx5_ib_put_native_port_mdev(dev, port_num + 1);
//
// We don't want to fail driver if debugfs failed to initialize,
// so we are not forwarding error to the user.
//
    return;
    }
