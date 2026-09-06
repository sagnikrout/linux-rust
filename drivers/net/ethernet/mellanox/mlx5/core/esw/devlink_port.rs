//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/devlink_port.c
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
// Copyright (c) 2020 Mellanox Technologies Ltd.

    static void
    mlx5_esw_get_port_parent_id(struct mlx5_core_dev *dev, struct netdev_phys_item_id *ppid)
    {
    mlx5_query_nic_sw_system_image_guid(dev, ppid.id, &ppid.id_len);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_esw_devlink_port_supported(esw: *mut mlx5_eswitch, vport_num: u16) -> bool {
    static bool mlx5_esw_devlink_port_supported(struct mlx5_eswitch *esw, u16 vport_num)
    {
    return (mlx5_core_is_ecpf(esw.dev) &&
    vport_num == MLX5_VPORT_HOST_PF) ||
    mlx5_eswitch_is_vf_vport(esw, vport_num) ||
    mlx5_core_is_ec_vf_vport(esw.dev, vport_num) ||
    mlx5_esw_is_spf_vport(esw, vport_num);
    }
    static void mlx5_esw_offloads_pf_vf_devlink_port_attrs_set(struct mlx5_eswitch *esw,
    u16 vport_num,
    struct devlink_port *dl_port)
    {
    struct mlx5_core_dev *dev = esw.dev;
    let mut ppid: netdev_phys_item_id = {};
    struct mlx5_vport *vport;
    let mut controller_num: u32 = 0;
    bool external;
    u16 pfnum;
    mlx5_esw_get_port_parent_id(dev, &ppid);
    pfnum = PCI_FUNC(dev.pdev.devfn);
    external = mlx5_core_is_ecpf_esw_manager(dev);
    if (external)
    controller_num = mlx5_esw_get_hpf_host_number(dev) + 1;
    if (vport_num == MLX5_VPORT_HOST_PF) {
    if (external)
    pfnum = mlx5_esw_get_hpf_pf_num(dev);
    memcpy(dl_port.attrs.switch_id.id, ppid.id, ppid.id_len);
    dl_port.attrs.switch_id.id_len = ppid.id_len;
    devlink_port_attrs_pci_pf_set(dl_port, controller_num, pfnum, external);
    } else if (mlx5_eswitch_is_vf_vport(esw, vport_num)) {
    let mut func_id: u16 = vport_num - 1;
    vport = mlx5_eswitch_get_vport(esw, vport_num);
    memcpy(dl_port.attrs.switch_id.id, ppid.id, ppid.id_len);
    dl_port.attrs.switch_id.id_len = ppid.id_len;
    if (vport.adjacent) {
    func_id = vport.adj_info.function_id;
    pfnum = vport.adj_info.parent_pci_devfn;
    } else if (external) {
    pfnum = mlx5_esw_get_hpf_pf_num(dev);
    }
    devlink_port_attrs_pci_vf_set(dl_port, controller_num, pfnum,
    func_id, external);
    }  else if (mlx5_core_is_ec_vf_vport(esw.dev, vport_num)) {
    let mut base_vport: u16 = mlx5_core_ec_vf_vport_base(dev);
    memcpy(dl_port.attrs.switch_id.id, ppid.id, ppid.id_len);
    dl_port.attrs.switch_id.id_len = ppid.id_len;
    devlink_port_attrs_pci_vf_set(dl_port, 0, pfnum,
    vport_num - base_vport, false);
    } else if (mlx5_esw_is_spf_vport(esw, vport_num)) {
    let mut spf_idx: c_int = mlx5_esw_spf_vport_to_idx(esw, vport_num);
    controller_num = esw.esw_funcs.spfs[spf_idx].host_number + 1;
    pfnum = esw.esw_funcs.spfs[spf_idx].pf_num;
    memcpy(dl_port.attrs.switch_id.id, ppid.id, ppid.id_len);
    dl_port.attrs.switch_id.id_len = ppid.id_len;
    devlink_port_attrs_pci_pf_set(dl_port, controller_num, pfnum,
    false);
    }
    }
    int mlx5_esw_offloads_pf_vf_devlink_port_init(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    struct mlx5_devlink_port *dl_port;
    let mut vport_num: u16 = vport.vport;
    if (!mlx5_esw_devlink_port_supported(esw, vport_num))
    return 0;
    dl_port = kzalloc_obj(*dl_port);
    if (!dl_port)
    return -ENOMEM;
    mlx5_esw_offloads_pf_vf_devlink_port_attrs_set(esw, vport_num,
    &dl_port.dl_port);
    vport.dl_port = dl_port;
    mlx5_devlink_port_init(dl_port, vport);
    return 0;
    }
    void mlx5_esw_offloads_pf_vf_devlink_port_cleanup(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    if (!vport.dl_port)
    return;
    kfree(vport.dl_port);
    vport.dl_port = core::ptr::null_mut();
    }
    static const struct devlink_port_ops mlx5_esw_pf_vf_dl_port_ops = {
    .port_fn_hw_addr_get = mlx5_devlink_port_fn_hw_addr_get,
    .port_fn_hw_addr_set = mlx5_devlink_port_fn_hw_addr_set,
    .port_fn_roce_get = mlx5_devlink_port_fn_roce_get,
    .port_fn_roce_set = mlx5_devlink_port_fn_roce_set,
    .port_fn_migratable_get = mlx5_devlink_port_fn_migratable_get,
    .port_fn_migratable_set = mlx5_devlink_port_fn_migratable_set,
    .port_fn_state_get = mlx5_devlink_pf_port_fn_state_get,
    .port_fn_state_set = mlx5_devlink_pf_port_fn_state_set,

    .port_fn_ipsec_crypto_get = mlx5_devlink_port_fn_ipsec_crypto_get,
    .port_fn_ipsec_crypto_set = mlx5_devlink_port_fn_ipsec_crypto_set,
    .port_fn_ipsec_packet_get = mlx5_devlink_port_fn_ipsec_packet_get,
    .port_fn_ipsec_packet_set = mlx5_devlink_port_fn_ipsec_packet_set,

    .port_fn_max_io_eqs_get = mlx5_devlink_port_fn_max_io_eqs_get,
    .port_fn_max_io_eqs_set = mlx5_devlink_port_fn_max_io_eqs_set,
    };
    static void mlx5_esw_offloads_sf_devlink_port_attrs_set(struct mlx5_eswitch *esw,
    struct devlink_port *dl_port,
    u32 controller, u32 sfnum)
    {
    struct mlx5_core_dev *dev = esw.dev;
    let mut ppid: netdev_phys_item_id = {};
    u32 hpf_ctrl;
    u16 pfnum;
    pfnum = mlx5_esw_sf_controller_to_pfnum(dev, controller);
    hpf_ctrl = mlx5_esw_get_hpf_host_number(dev) + 1;
    mlx5_esw_get_port_parent_id(dev, &ppid);
    memcpy(dl_port.attrs.switch_id.id, &ppid.id[0], ppid.id_len);
    dl_port.attrs.switch_id.id_len = ppid.id_len;
    devlink_port_attrs_pci_sf_set(dl_port, controller, pfnum, sfnum,
    controller == hpf_ctrl);
    }
    int mlx5_esw_offloads_sf_devlink_port_init(struct mlx5_eswitch *esw, struct mlx5_vport *vport,
    struct mlx5_devlink_port *dl_port,
    u32 controller, u32 sfnum)
    {
    mlx5_esw_offloads_sf_devlink_port_attrs_set(esw, &dl_port.dl_port, controller, sfnum);
    vport.dl_port = dl_port;
    mlx5_devlink_port_init(dl_port, vport);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_offloads_sf_devlink_port_cleanup(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) {
    void mlx5_esw_offloads_sf_devlink_port_cleanup(struct mlx5_eswitch *esw, struct mlx5_vport *vport)
    {
    vport.dl_port = core::ptr::null_mut();
    }
    static const struct devlink_port_ops mlx5_esw_dl_sf_port_ops = {

    .port_del = mlx5_devlink_sf_port_del,

    .port_fn_hw_addr_get = mlx5_devlink_port_fn_hw_addr_get,
    .port_fn_hw_addr_set = mlx5_devlink_port_fn_hw_addr_set,
    .port_fn_roce_get = mlx5_devlink_port_fn_roce_get,
    .port_fn_roce_set = mlx5_devlink_port_fn_roce_set,

    .port_fn_state_get = mlx5_devlink_sf_port_fn_state_get,
    .port_fn_state_set = mlx5_devlink_sf_port_fn_state_set,

    .port_fn_max_io_eqs_get = mlx5_devlink_port_fn_max_io_eqs_get,
    .port_fn_max_io_eqs_set = mlx5_devlink_port_fn_max_io_eqs_set,
    };
    static int mlx5_esw_devlink_port_res_register(struct mlx5_eswitch *esw,
    struct devlink_port *dl_port,
    u16 vport_num)
    {
    struct devlink_resource_size_params size_params;
    struct mlx5_core_dev *dev = esw.dev;
    u16 max_sfs, sf_base_id;
    int err;
    if (vport_num != MLX5_VPORT_HOST_PF &&
    !mlx5_esw_is_spf_vport(esw, vport_num))
    return 0;
    if (vport_num == MLX5_VPORT_HOST_PF) {
    err = mlx5_esw_sf_max_hpf_functions(dev, &max_sfs,
    &sf_base_id);
    } else {
    let mut spf_idx: c_int = mlx5_esw_spf_vport_to_idx(esw, vport_num);
    err = mlx5_esw_sf_max_spf_functions(dev, spf_idx, &max_sfs,
    &sf_base_id);
    }
    if (err)
    return err;
    devlink_resource_size_params_init(&size_params, max_sfs, max_sfs, 1,
    DEVLINK_RESOURCE_UNIT_ENTRY);
    return devl_port_resource_register(dl_port, "max_SFs", max_sfs,
    MLX5_DL_PORT_RES_MAX_SFS,
    DEVLINK_RESOURCE_ID_PARENT_TOP,
    &size_params);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_esw_devlink_port_res_unregister(dl_port: *mut devlink_port) {
    static void mlx5_esw_devlink_port_res_unregister(struct devlink_port *dl_port)
    {
    devl_port_resources_unregister(dl_port);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_offloads_devlink_port_register(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int {
    int mlx5_esw_offloads_devlink_port_register(struct mlx5_eswitch *esw, struct mlx5_vport *vport)
    {
    struct mlx5_core_dev *dev = esw.dev;
    const struct devlink_port_ops *ops;
    struct mlx5_devlink_port *dl_port;
    let mut vport_num: u16 = vport.vport;
    unsigned int dl_port_index;
    struct devlink *devlink;
    int err;
    dl_port = vport.dl_port;
    if (!dl_port)
    return 0;
    if (mlx5_esw_is_sf_vport(esw, vport_num))
    ops = &mlx5_esw_dl_sf_port_ops;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: mlx5_eswitch_is_pf_vf_vport(esw, _arg: vport_num)) -> else {
    else if (mlx5_eswitch_is_pf_vf_vport(esw, vport_num))
    ops = &mlx5_esw_pf_vf_dl_port_ops;
    else
    ops = core::ptr::null_mut();
    devlink = priv_to_devlink(dev);
    dl_port_index = mlx5_esw_vport_to_devlink_port_index(dev, vport_num);
    err = devl_port_register_with_ops(devlink, &dl_port.dl_port, dl_port_index, ops);
    if (err)
    return err;
    err = devl_rate_leaf_create(&dl_port.dl_port, vport, core::ptr::null_mut());
    if (err)
    goto rate_err;
    err = mlx5_esw_devlink_port_res_register(esw, &dl_port.dl_port,
    vport_num);
    if (err)
    mlx5_core_dbg(dev, "Failed to register port resources: %d\n",
    err);
    return 0;
    rate_err:
    devl_port_unregister(&dl_port.dl_port);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_offloads_devlink_port_unregister(vport: *mut mlx5_vport) {
    void mlx5_esw_offloads_devlink_port_unregister(struct mlx5_vport *vport)
    {
    struct mlx5_devlink_port *dl_port;
    if (!vport.dl_port)
    return;
    dl_port = vport.dl_port;
    mlx5_esw_devlink_port_res_unregister(&dl_port.dl_port);
    devl_rate_leaf_destroy(&dl_port.dl_port);
    devl_port_unregister(&dl_port.dl_port);
    }
    struct devlink_port *mlx5_esw_offloads_devlink_port(struct mlx5_eswitch *esw, u16 vport_num)
    {
    struct mlx5_vport *vport;
    vport = mlx5_eswitch_get_vport(esw, vport_num);
    if (IS_ERR(vport))
    return ERR_CAST(vport);
    if (!vport.dl_port)
    return ERR_PTR(-ENODEV);
    return &vport.dl_port.dl_port;
    }
