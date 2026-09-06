//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_flow.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2020 Mellanox Technologies. All rights reserved

    struct mlxsw_sp_flow_block *
    mlxsw_sp_flow_block_create(struct mlxsw_sp *mlxsw_sp, struct net *net)
    {
    struct mlxsw_sp_flow_block *block;
    block = kzalloc_obj(*block);
    if (!block)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&block.binding_list);
    INIT_LIST_HEAD(&block.mall.list);
    block.mlxsw_sp = mlxsw_sp;
    block.net = net;
    return block;
    }
#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_flow_block_destroy(block: *mut mlxsw_sp_flow_block) {
    void mlxsw_sp_flow_block_destroy(struct mlxsw_sp_flow_block *block)
    {
    WARN_ON(!list_empty(&block.binding_list));
    kfree(block);
    }
    static struct mlxsw_sp_flow_block_binding *
    mlxsw_sp_flow_block_lookup(struct mlxsw_sp_flow_block *block,
    struct mlxsw_sp_port *mlxsw_sp_port, bool ingress)
    {
    struct mlxsw_sp_flow_block_binding *binding;
    list_for_each_entry(binding, &block.binding_list, list)
    if (binding.mlxsw_sp_port == mlxsw_sp_port &&
    binding.ingress == ingress)
    return binding;
    return core::ptr::null_mut();
    }
    static bool
    mlxsw_sp_flow_block_ruleset_bound(const struct mlxsw_sp_flow_block *block)
    {
    return block.ruleset_zero;
    }
    static int mlxsw_sp_flow_block_bind(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp_flow_block *block,
    struct mlxsw_sp_port *mlxsw_sp_port,
    bool ingress,
    struct netlink_ext_ack *extack)
    {
    struct mlxsw_sp_flow_block_binding *binding;
    int err;
    if (WARN_ON(mlxsw_sp_flow_block_lookup(block, mlxsw_sp_port, ingress)))
    return -EEXIST;
    if (ingress && block.ingress_blocker_rule_count) {
    NL_SET_ERR_MSG_MOD(extack, "Block cannot be bound to ingress because it contains unsupported rules");
    return -EOPNOTSUPP;
    }
    if (!ingress && block.egress_blocker_rule_count) {
    NL_SET_ERR_MSG_MOD(extack, "Block cannot be bound to egress because it contains unsupported rules");
    return -EOPNOTSUPP;
    }
    err = mlxsw_sp_mall_port_bind(block, mlxsw_sp_port, extack);
    if (err)
    return err;
    binding = kzalloc_obj(*binding);
    if (!binding) {
    err = -ENOMEM;
    goto err_binding_alloc;
    }
    binding.mlxsw_sp_port = mlxsw_sp_port;
    binding.ingress = ingress;
    if (mlxsw_sp_flow_block_ruleset_bound(block)) {
    err = mlxsw_sp_acl_ruleset_bind(mlxsw_sp, block, binding);
    if (err)
    goto err_ruleset_bind;
    }
    if (ingress)
    block.ingress_binding_count++;
    else
    block.egress_binding_count++;
    list_add(&binding.list, &block.binding_list);
    return 0;
    err_ruleset_bind:
    kfree(binding);
    err_binding_alloc:
    mlxsw_sp_mall_port_unbind(block, mlxsw_sp_port);
    return err;
    }
    static int mlxsw_sp_flow_block_unbind(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp_flow_block *block,
    struct mlxsw_sp_port *mlxsw_sp_port,
    bool ingress)
    {
    struct mlxsw_sp_flow_block_binding *binding;
    binding = mlxsw_sp_flow_block_lookup(block, mlxsw_sp_port, ingress);
    if (!binding)
    return -ENOENT;
    list_del(&binding.list);
    if (ingress)
    block.ingress_binding_count--;
    else
    block.egress_binding_count--;
    if (mlxsw_sp_flow_block_ruleset_bound(block))
    mlxsw_sp_acl_ruleset_unbind(mlxsw_sp, block, binding);
    kfree(binding);
    mlxsw_sp_mall_port_unbind(block, mlxsw_sp_port);
    return 0;
    }
    static int mlxsw_sp_flow_block_mall_cb(struct mlxsw_sp_flow_block *flow_block,
    struct tc_cls_matchall_offload *f)
    {
    struct mlxsw_sp *mlxsw_sp = mlxsw_sp_flow_block_mlxsw_sp(flow_block);
    switch (f.command) {
    case TC_CLSMATCHALL_REPLACE:
    return mlxsw_sp_mall_replace(mlxsw_sp, flow_block, f);
    case TC_CLSMATCHALL_DESTROY:
    mlxsw_sp_mall_destroy(flow_block, f);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int mlxsw_sp_flow_block_flower_cb(struct mlxsw_sp_flow_block *flow_block,
    struct flow_cls_offload *f)
    {
    struct mlxsw_sp *mlxsw_sp = mlxsw_sp_flow_block_mlxsw_sp(flow_block);
    switch (f.command) {
    case FLOW_CLS_REPLACE:
    return mlxsw_sp_flower_replace(mlxsw_sp, flow_block, f);
    case FLOW_CLS_DESTROY:
    mlxsw_sp_flower_destroy(mlxsw_sp, flow_block, f);
    return 0;
    case FLOW_CLS_STATS:
    return mlxsw_sp_flower_stats(mlxsw_sp, flow_block, f);
    case FLOW_CLS_TMPLT_CREATE:
    return mlxsw_sp_flower_tmplt_create(mlxsw_sp, flow_block, f);
    case FLOW_CLS_TMPLT_DESTROY:
    mlxsw_sp_flower_tmplt_destroy(mlxsw_sp, flow_block, f);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int mlxsw_sp_flow_block_cb(enum tc_setup_type type,
    void *type_data, void *cb_priv)
    {
    struct mlxsw_sp_flow_block *flow_block = cb_priv;
    if (mlxsw_sp_flow_block_disabled(flow_block))
    return -EOPNOTSUPP;
    switch (type) {
    case TC_SETUP_CLSMATCHALL:
    return mlxsw_sp_flow_block_mall_cb(flow_block, type_data);
    case TC_SETUP_CLSFLOWER:
    return mlxsw_sp_flow_block_flower_cb(flow_block, type_data);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp_tc_block_release(cb_priv: *mut c_void) {
    static void mlxsw_sp_tc_block_release(void *cb_priv)
    {
    struct mlxsw_sp_flow_block *flow_block = cb_priv;
    mlxsw_sp_flow_block_destroy(flow_block);
    }
    static LIST_HEAD(mlxsw_sp_block_cb_list);
    static int mlxsw_sp_setup_tc_block_bind(struct mlxsw_sp_port *mlxsw_sp_port,
    struct flow_block_offload *f,
    bool ingress)
    {
    struct mlxsw_sp *mlxsw_sp = mlxsw_sp_port.mlxsw_sp;
    struct mlxsw_sp_flow_block *flow_block;
    struct flow_block_cb *block_cb;
    let mut register_block: bool = false;
    int err;
    block_cb = flow_block_cb_lookup(f.block, mlxsw_sp_flow_block_cb,
    mlxsw_sp);
    if (!block_cb) {
    flow_block = mlxsw_sp_flow_block_create(mlxsw_sp, f.net);
    if (!flow_block)
    return -ENOMEM;
    block_cb = flow_block_cb_alloc(mlxsw_sp_flow_block_cb,
    mlxsw_sp, flow_block,
    mlxsw_sp_tc_block_release);
    if (IS_ERR(block_cb)) {
    mlxsw_sp_flow_block_destroy(flow_block);
    return PTR_ERR(block_cb);
    }
    register_block = true;
    } else {
    flow_block = flow_block_cb_priv(block_cb);
    }
    flow_block_cb_incref(block_cb);
    err = mlxsw_sp_flow_block_bind(mlxsw_sp, flow_block,
    mlxsw_sp_port, ingress, f.extack);
    if (err)
    goto err_block_bind;
    if (ingress)
    mlxsw_sp_port.ing_flow_block = flow_block;
    else
    mlxsw_sp_port.eg_flow_block = flow_block;
    if (register_block) {
    flow_block_cb_add(block_cb, f);
    list_add_tail(&block_cb.driver_list, &mlxsw_sp_block_cb_list);
    }
    return 0;
    err_block_bind:
    if (!flow_block_cb_decref(block_cb))
    flow_block_cb_free(block_cb);
    return err;
    }
    static void mlxsw_sp_setup_tc_block_unbind(struct mlxsw_sp_port *mlxsw_sp_port,
    struct flow_block_offload *f,
    bool ingress)
    {
    struct mlxsw_sp *mlxsw_sp = mlxsw_sp_port.mlxsw_sp;
    struct mlxsw_sp_flow_block *flow_block;
    struct flow_block_cb *block_cb;
    int err;
    block_cb = flow_block_cb_lookup(f.block, mlxsw_sp_flow_block_cb,
    mlxsw_sp);
    if (!block_cb)
    return;
    if (ingress)
    mlxsw_sp_port.ing_flow_block = core::ptr::null_mut();
    else
    mlxsw_sp_port.eg_flow_block = core::ptr::null_mut();
    flow_block = flow_block_cb_priv(block_cb);
    err = mlxsw_sp_flow_block_unbind(mlxsw_sp, flow_block,
    mlxsw_sp_port, ingress);
    if (!err && !flow_block_cb_decref(block_cb)) {
    flow_block_cb_remove(block_cb, f);
    list_del(&block_cb.driver_list);
    }
    }
    int mlxsw_sp_setup_tc_block_clsact(struct mlxsw_sp_port *mlxsw_sp_port,
    struct flow_block_offload *f,
    bool ingress)
    {
    f.driver_block_list = &mlxsw_sp_block_cb_list;
    switch (f.command) {
    case FLOW_BLOCK_BIND:
    return mlxsw_sp_setup_tc_block_bind(mlxsw_sp_port, f, ingress);
    case FLOW_BLOCK_UNBIND:
    mlxsw_sp_setup_tc_block_unbind(mlxsw_sp_port, f, ingress);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
