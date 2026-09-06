//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/hw_table.c
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
// Copyright (c) 2020 Mellanox Technologies Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sf_hw {
    pub usr_sfnum: u32,
    pub 1: u8 allocated:,
    pub 1: u8 pending_delete:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sf_hwc_table {
    pub sfs: *mut mlx5_sf_hw,
    pub max_fn: c_int,
    pub start_fn_id: u16,
    pub controller: u32,
}

    enum {
    MLX5_SF_HWC_LOCAL,
    MLX5_SF_HWC_EXT_HOST,
    MLX5_SF_HWC_FIRST_SPF,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sf_hw_table {
    pub /: *mut *mut mutex table_lock; / Serializes sf deletion and vhca state change handler.,
    pub hwc: *mut mlx5_sf_hwc_table,
    pub num_hwc: c_int,
}

    static struct mlx5_sf_hwc_table *
    mlx5_sf_controller_to_hwc(struct mlx5_core_dev *dev, u32 controller)
    {
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    int i;
    for (i = MLX5_SF_HWC_FIRST_SPF; i < table.num_hwc; i++) {
    if (table.hwc[i].controller == controller)
    return &table.hwc[i];
    }
    return &table.hwc[!!controller];
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_sw_to_hw_id(dev: *mut mlx5_core_dev, controller: u32, sw_id: u16) -> u16 {
    u16 mlx5_sf_sw_to_hw_id(struct mlx5_core_dev *dev, u32 controller, u16 sw_id)
    {
    struct mlx5_sf_hwc_table *hwc;
    hwc = mlx5_sf_controller_to_hwc(dev, controller);
    return hwc.start_fn_id + sw_id;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_hw_to_sw_id(hwc: *mut mlx5_sf_hwc_table, hw_id: u16) -> u16 {
    static u16 mlx5_sf_hw_to_sw_id(struct mlx5_sf_hwc_table *hwc, u16 hw_id)
    {
    return hw_id - hwc.start_fn_id;
    }
    static struct mlx5_sf_hwc_table *
    mlx5_sf_table_fn_to_hwc(struct mlx5_sf_hw_table *table, u16 fn_id)
    {
    int i;
    for (i = 0; i < table.num_hwc; i++) {
    if (table.hwc[i].max_fn &&
    fn_id >= table.hwc[i].start_fn_id &&
    fn_id < (table.hwc[i].start_fn_id + table.hwc[i].max_fn))
    return &table.hwc[i];
    }
    return core::ptr::null_mut();
    }
    static int mlx5_sf_hw_table_id_alloc(struct mlx5_core_dev *dev,
    struct mlx5_sf_hw_table *table,
    u32 controller,
    u32 usr_sfnum)
    {
    struct mlx5_sf_hwc_table *hwc;
    let mut free_idx: c_int = -1;
    int i;
    hwc = mlx5_sf_controller_to_hwc(dev, controller);
    if (!hwc.sfs)
    return -ENOSPC;
    for (i = 0; i < hwc.max_fn; i++) {
    if (!hwc.sfs[i].allocated && free_idx == -1) {
    free_idx = i;
    continue;
    }
    if (hwc.sfs[i].allocated && hwc.sfs[i].usr_sfnum == usr_sfnum)
    return -EEXIST;
    }
    if (free_idx == -1)
    return -ENOSPC;
    hwc.sfs[free_idx].usr_sfnum = usr_sfnum;
    hwc.sfs[free_idx].allocated = true;
    return free_idx;
    }
    static void mlx5_sf_hw_table_id_free(struct mlx5_core_dev *dev,
    struct mlx5_sf_hw_table *table,
    u32 controller, int id)
    {
    struct mlx5_sf_hwc_table *hwc;
    hwc = mlx5_sf_controller_to_hwc(dev, controller);
    hwc.sfs[id].allocated = false;
    hwc.sfs[id].pending_delete = false;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_sf_alloc(dev: *mut mlx5_core_dev, controller: u32, usr_sfnum: u32) -> c_int {
    int mlx5_sf_hw_table_sf_alloc(struct mlx5_core_dev *dev, u32 controller, u32 usr_sfnum)
    {
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    u16 hw_fn_id;
    int sw_id;
    int err;
    if (!table)
    return -EOPNOTSUPP;
    mutex_lock(&table.table_lock);
    sw_id = mlx5_sf_hw_table_id_alloc(dev, table, controller, usr_sfnum);
    if (sw_id < 0) {
    err = sw_id;
    goto exist_err;
    }
    hw_fn_id = mlx5_sf_sw_to_hw_id(dev, controller, sw_id);
    err = mlx5_cmd_alloc_sf(dev, hw_fn_id);
    if (err)
    goto err;
    err = mlx5_modify_vhca_sw_id(dev, hw_fn_id, usr_sfnum);
    if (err)
    goto vhca_err;
    if (controller) {
// If this SF is for external controller, SF manager
// needs to arm firmware to receive the events.
//
    err = mlx5_vhca_event_arm(dev, hw_fn_id);
    if (err)
    goto vhca_err;
    }
    trace_mlx5_sf_hwc_alloc(dev, controller, hw_fn_id, usr_sfnum);
    mutex_unlock(&table.table_lock);
    return sw_id;
    vhca_err:
    mlx5_cmd_dealloc_sf(dev, hw_fn_id);
    err:
    mlx5_sf_hw_table_id_free(dev, table, controller, sw_id);
    exist_err:
    mutex_unlock(&table.table_lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_sf_free(dev: *mut mlx5_core_dev, controller: u32, id: u16) {
    void mlx5_sf_hw_table_sf_free(struct mlx5_core_dev *dev, u32 controller, u16 id)
    {
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    u16 hw_fn_id;
    mutex_lock(&table.table_lock);
    hw_fn_id = mlx5_sf_sw_to_hw_id(dev, controller, id);
    mlx5_cmd_dealloc_sf(dev, hw_fn_id);
    mlx5_sf_hw_table_id_free(dev, table, controller, id);
    mutex_unlock(&table.table_lock);
    }
    static void mlx5_sf_hw_table_hwc_sf_free(struct mlx5_core_dev *dev,
    struct mlx5_sf_hwc_table *hwc, int idx)
    {
    mlx5_cmd_dealloc_sf(dev, hwc.start_fn_id + idx);
    hwc.sfs[idx].allocated = false;
    hwc.sfs[idx].pending_delete = false;
    trace_mlx5_sf_hwc_free(dev, hwc.start_fn_id + idx);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_sf_deferred_free(dev: *mut mlx5_core_dev, controller: u32, id: u16) {
    void mlx5_sf_hw_table_sf_deferred_free(struct mlx5_core_dev *dev, u32 controller, u16 id)
    {
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    u32 out[MLX5_ST_SZ_DW(query_vhca_state_out)] = {};
    struct mlx5_sf_hwc_table *hwc;
    u16 hw_fn_id;
    u8 state;
    int err;
    hw_fn_id = mlx5_sf_sw_to_hw_id(dev, controller, id);
    hwc = mlx5_sf_controller_to_hwc(dev, controller);
    mutex_lock(&table.table_lock);
    err = mlx5_cmd_query_vhca_state(dev, hw_fn_id, out, sizeof(out));
    if (err)
    goto err;
    state = MLX5_GET(query_vhca_state_out, out, vhca_state_context.vhca_state);
    if (state == MLX5_VHCA_STATE_ALLOCATED) {
    mlx5_cmd_dealloc_sf(dev, hw_fn_id);
    hwc.sfs[id].allocated = false;
    } else {
    hwc.sfs[id].pending_delete = true;
    trace_mlx5_sf_hwc_deferred_free(dev, hw_fn_id);
    }
    err:
    mutex_unlock(&table.table_lock);
    }
    static void mlx5_sf_hw_table_hwc_dealloc_all(struct mlx5_core_dev *dev,
    struct mlx5_sf_hwc_table *hwc)
    {
    int i;
    for (i = 0; i < hwc.max_fn; i++) {
    if (hwc.sfs[i].allocated)
    mlx5_sf_hw_table_hwc_sf_free(dev, hwc, i);
    }
    }
    static void mlx5_sf_hw_table_dealloc_all(struct mlx5_core_dev *dev,
    struct mlx5_sf_hw_table *table)
    {
    int i;
    for (i = 0; i < table.num_hwc; i++)
    mlx5_sf_hw_table_hwc_dealloc_all(dev, &table.hwc[i]);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_hw_table_hwc_init(hwc: *mut mlx5_sf_hwc_table, max_fn: u16, base_id: u16) -> c_int {
    static int mlx5_sf_hw_table_hwc_init(struct mlx5_sf_hwc_table *hwc, u16 max_fn, u16 base_id)
    {
    struct mlx5_sf_hw *sfs;
    if (!max_fn)
    return 0;
    sfs = kzalloc_objs(*sfs, max_fn);
    if (!sfs)
    return -ENOMEM;
    hwc.sfs = sfs;
    hwc.max_fn = max_fn;
    hwc.start_fn_id = base_id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_hw_table_hwc_cleanup(hwc: *mut mlx5_sf_hwc_table) {
    static void mlx5_sf_hw_table_hwc_cleanup(struct mlx5_sf_hwc_table *hwc)
    {
    kfree(hwc.sfs);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_hw_table_res_unregister(dev: *mut mlx5_core_dev) {
    static void mlx5_sf_hw_table_res_unregister(struct mlx5_core_dev *dev)
    {
    devl_resources_unregister(priv_to_devlink(dev));
    }
    static int mlx5_sf_hw_table_res_register(struct mlx5_core_dev *dev, u16 max_fn,
    u16 max_ext_fn)
    {
    struct devlink_resource_size_params size_params;
    struct devlink *devlink = priv_to_devlink(dev);
    int err;
    devlink_resource_size_params_init(&size_params, max_fn, max_fn, 1,
    DEVLINK_RESOURCE_UNIT_ENTRY);
    err = devl_resource_register(devlink, "max_local_SFs", max_fn, MLX5_DL_RES_MAX_LOCAL_SFS,
    DEVLINK_RESOURCE_ID_PARENT_TOP, &size_params);
    if (err)
    return err;
    devlink_resource_size_params_init(&size_params, max_ext_fn, max_ext_fn, 1,
    DEVLINK_RESOURCE_UNIT_ENTRY);
    return devl_resource_register(devlink, "max_external_SFs", max_ext_fn,
    MLX5_DL_RES_MAX_EXTERNAL_SFS, DEVLINK_RESOURCE_ID_PARENT_TOP,
    &size_params);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_init(dev: *mut mlx5_core_dev) -> c_int {
    int mlx5_sf_hw_table_init(struct mlx5_core_dev *dev)
    {
    struct mlx5_sf_hw_table *table;
    int num_spfs, num_hwc;
    let mut max_ext_fn: u16 = 0;
    let mut ext_base_id: u16 = 0;
    u16 base_id;
    u16 max_fn;
    int err;
    int i;
    if (!mlx5_vhca_event_supported(dev))
    return 0;
    max_fn = mlx5_sf_max_functions(dev);
    err = mlx5_esw_sf_max_hpf_functions(dev, &max_ext_fn, &ext_base_id);
    if (err)
    return err;
    if (mlx5_sf_hw_table_res_register(dev, max_fn, max_ext_fn))
    mlx5_core_dbg(dev, "failed to register max SFs resources");
    if (!max_fn && !max_ext_fn && !mlx5_esw_has_spf_sfs(dev))
    return 0;
    table = kzalloc_obj(*table);
    if (!table) {
    err = -ENOMEM;
    goto alloc_err;
    }
    num_spfs = mlx5_esw_get_num_spfs(dev);
    num_hwc = MLX5_SF_HWC_FIRST_SPF + num_spfs;
    table.hwc = kzalloc_objs(*table.hwc, num_hwc);
    if (!table.hwc) {
    err = -ENOMEM;
    goto hwc_alloc_err;
    }
    table.num_hwc = num_hwc;
    mutex_init(&table.table_lock);
    dev.priv.sf_hw_table = table;
    table.hwc[MLX5_SF_HWC_LOCAL].controller = 0;
    base_id = mlx5_sf_start_function_id(dev);
    err = mlx5_sf_hw_table_hwc_init(&table.hwc[MLX5_SF_HWC_LOCAL], max_fn, base_id);
    if (err)
    goto hwc_init_err;
    table.hwc[MLX5_SF_HWC_EXT_HOST].controller =
    mlx5_esw_get_hpf_host_number(dev) + 1;
    err = mlx5_sf_hw_table_hwc_init(&table.hwc[MLX5_SF_HWC_EXT_HOST],
    max_ext_fn, ext_base_id);
    if (err)
    goto hwc_init_err;
    for (i = 0; i < num_spfs; i++) {
    u16 spf_max_sfs, spf_base_id, host_number;
    let mut hwc_idx: c_int = MLX5_SF_HWC_FIRST_SPF + i;
    err = mlx5_esw_spf_get_host_number(dev, i, &host_number);
    if (err)
    goto hwc_init_err;
    err = mlx5_esw_sf_max_spf_functions(dev, i, &spf_max_sfs,
    &spf_base_id);
    if (err)
    goto hwc_init_err;
    table.hwc[hwc_idx].controller = host_number + 1;
    err = mlx5_sf_hw_table_hwc_init(&table.hwc[hwc_idx],
    spf_max_sfs, spf_base_id);
    if (err)
    goto hwc_init_err;
    }
    mlx5_core_dbg(dev, "SF HW table: max sfs = %d, ext sfs = %d, num spfs = %d\n",
    max_fn, max_ext_fn, num_spfs);
    return 0;
    hwc_init_err:
    dev.priv.sf_hw_table = core::ptr::null_mut();
    for (i = 0; i < num_hwc; i++)
    mlx5_sf_hw_table_hwc_cleanup(&table.hwc[i]);
    mutex_destroy(&table.table_lock);
    kfree(table.hwc);
    hwc_alloc_err:
    kfree(table);
    alloc_err:
    mlx5_sf_hw_table_res_unregister(dev);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_cleanup(dev: *mut mlx5_core_dev) {
    void mlx5_sf_hw_table_cleanup(struct mlx5_core_dev *dev)
    {
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    int i;
    if (!table)
    goto res_unregister;
    for (i = 0; i < table.num_hwc; i++)
    mlx5_sf_hw_table_hwc_cleanup(&table.hwc[i]);
    mutex_destroy(&table.table_lock);
    kfree(table.hwc);
    kfree(table);
    dev.priv.sf_hw_table = core::ptr::null_mut();
    res_unregister:
    mlx5_sf_hw_table_res_unregister(dev);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_hw_vhca_event(nb: *mut notifier_block, opcode: c_ulong, data: *mut c_void) -> c_int {
    static int mlx5_sf_hw_vhca_event(struct notifier_block *nb, unsigned long opcode, void *data)
    {
    struct mlx5_core_dev *dev = container_of(nb, struct mlx5_core_dev,
    priv.sf_hw_table_vhca_nb);
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    const struct mlx5_vhca_state_event *event = data;
    struct mlx5_sf_hwc_table *hwc;
    struct mlx5_sf_hw *sf_hw;
    u16 sw_id;
    if (!table || event.new_vhca_state != MLX5_VHCA_STATE_ALLOCATED)
    return 0;
    hwc = mlx5_sf_table_fn_to_hwc(table, event.function_id);
    if (!hwc)
    return 0;
    sw_id = mlx5_sf_hw_to_sw_id(hwc, event.function_id);
    sf_hw = &hwc.sfs[sw_id];
    mutex_lock(&table.table_lock);
// SF driver notified through firmware that SF is finally detached.
// Hence recycle the sf hardware id for reuse.
//
    if (sf_hw.allocated && sf_hw.pending_delete)
    mlx5_sf_hw_table_hwc_sf_free(dev, hwc, sw_id);
    mutex_unlock(&table.table_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_notifier_init(dev: *mut mlx5_core_dev) -> c_int {
    int mlx5_sf_hw_notifier_init(struct mlx5_core_dev *dev)
    {
    if (mlx5_core_is_sf(dev))
    return 0;
    dev.priv.sf_hw_table_vhca_nb.notifier_call = mlx5_sf_hw_vhca_event;
    return mlx5_vhca_event_notifier_register(dev,
    &dev.priv.sf_hw_table_vhca_nb);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_notifier_cleanup(dev: *mut mlx5_core_dev) {
    void mlx5_sf_hw_notifier_cleanup(struct mlx5_core_dev *dev)
    {
    if (mlx5_core_is_sf(dev))
    return;
    mlx5_vhca_event_notifier_unregister(dev,
    &dev.priv.sf_hw_table_vhca_nb);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_destroy(dev: *mut mlx5_core_dev) {
    void mlx5_sf_hw_table_destroy(struct mlx5_core_dev *dev)
    {
    struct mlx5_sf_hw_table *table = dev.priv.sf_hw_table;
    if (!table)
    return;
// Dealloc SFs whose firmware event has been missed.
    mlx5_sf_hw_table_dealloc_all(dev, table);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_supported(dev: *const mlx5_core_dev) -> bool {
    bool mlx5_sf_hw_table_supported(const struct mlx5_core_dev *dev)
    {
    return !!dev.priv.sf_hw_table;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_hw_table_esw_changed_event_handler(dev: *mut mlx5_core_dev) {
    void mlx5_sf_hw_table_esw_changed_event_handler(struct mlx5_core_dev *dev)
    {
    struct mlx5_sf_hw_table *table;
    struct mlx5_sf_hwc_table *hwc;
    int i;
    table = dev.priv.sf_hw_table;
    if (!table)
    return;
    mutex_lock(&table.table_lock);
    hwc = &table.hwc[MLX5_SF_HWC_EXT_HOST];
    for (i = 0; i < hwc.max_fn; i++) {
    struct mlx5_sf_hw *sf_hw;
    sf_hw = &hwc.sfs[i];
    if (sf_hw.allocated && sf_hw.pending_delete)
    mlx5_sf_hw_table_hwc_sf_free(dev, hwc, i);
    }
    mutex_unlock(&table.table_lock);
    }
