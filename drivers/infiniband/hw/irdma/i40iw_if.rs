//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/irdma/i40iw_if.c
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
// Copyright (c) 2015 - 2021 Intel Corporation

    static struct i40e_client i40iw_client;
//
// i40iw_l2param_change - handle mss change
// @cdev_info: parent lan device information structure with data/ops
// @client: client for parameter change
// @params: new parameters from L2
//
    static void i40iw_l2param_change(struct i40e_info *cdev_info,
    struct i40e_client *client,
    struct i40e_params *params)
    {
    let mut l2params: irdma_l2params = {};
    struct irdma_device *iwdev;
    struct ib_device *ibdev;
    ibdev = ib_device_get_by_netdev(cdev_info.netdev, RDMA_DRIVER_IRDMA);
    if (!ibdev)
    return;
    iwdev = to_iwdev(ibdev);
    if (iwdev.vsi.mtu != params.mtu) {
    l2params.mtu_changed = true;
    l2params.mtu = params.mtu;
    }
    irdma_change_l2params(&iwdev.vsi, &l2params);
    ib_device_put(ibdev);
    }
//
// i40iw_close - client interface operation close for iwarp/uda device
// @cdev_info: parent lan device information structure with data/ops
// @client: client to close
// @reset: flag to indicate close on reset
//
// Called by the lan driver during the processing of client unregister
// Destroy and clean up the driver resources
//
    static void i40iw_close(struct i40e_info *cdev_info, struct i40e_client *client,
    bool reset)
    {
    struct irdma_device *iwdev;
    struct ib_device *ibdev;
    ibdev = ib_device_get_by_netdev(cdev_info.netdev, RDMA_DRIVER_IRDMA);
    if (WARN_ON(!ibdev))
    return;
    iwdev = to_iwdev(ibdev);
    if (reset)
    iwdev.rf.reset = true;
    iwdev.iw_status = 0;
    irdma_port_ibevent(iwdev);
    ib_unregister_device_and_put(ibdev);
    pr_debug("INIT: Gen1 PF[%d] close complete\n", PCI_FUNC(cdev_info.pcidev.devfn));
    }
#[no_mangle]
unsafe extern "C" fn i40iw_request_reset(rf: *mut irdma_pci_f) {
    static void i40iw_request_reset(struct irdma_pci_f *rf)
    {
    struct i40e_info *cdev_info = rf.cdev;
    cdev_info.ops.request_reset(cdev_info, &i40iw_client, 1);
    }
#[no_mangle]
unsafe extern "C" fn i40iw_fill_device_info(iwdev: *mut irdma_device, cdev_info: *mut i40e_info) {
    static void i40iw_fill_device_info(struct irdma_device *iwdev, struct i40e_info *cdev_info)
    {
    struct irdma_pci_f *rf = iwdev.rf;
    rf.rdma_ver = IRDMA_GEN_1;
    rf.sc_dev.hw = &rf.hw;
    rf.sc_dev.hw_attrs.uk_attrs.hw_rev = IRDMA_GEN_1;
    rf.sc_dev.privileged = true;
    rf.gen_ops.request_reset = i40iw_request_reset;
    rf.pcidev = cdev_info.pcidev;
    rf.pf_id = cdev_info.fid;
    rf.hw.hw_addr = cdev_info.hw_addr;
    rf.cdev = cdev_info;
    rf.msix_count = cdev_info.msix_count;
    rf.msix_entries = cdev_info.msix_entries;
    rf.limits_sel = 5;
    rf.protocol_used = IRDMA_IWARP_PROTOCOL_ONLY;
    rf.iwdev = iwdev;
    iwdev.init_state = INITIAL_STATE;
    iwdev.rcv_wnd = IRDMA_CM_DEFAULT_RCV_WND_SCALED;
    iwdev.rcv_wscale = IRDMA_CM_DEFAULT_RCV_WND_SCALE;
    iwdev.netdev = cdev_info.netdev;
    iwdev.vsi_num = 0;
    }
//
// i40iw_open - client interface operation open for iwarp/uda device
// @cdev_info: parent lan device information structure with data/ops
// @client: iwarp client information, provided during registration
//
// Called by the lan driver during the processing of client register
// Create device resources, set up queues, pble and hmc objects and
// register the device with the ib verbs interface
// Return 0 if successful, otherwise return error
//
#[no_mangle]
unsafe extern "C" fn i40iw_open(cdev_info: *mut i40e_info, client: *mut i40e_client) -> c_int {
    static int i40iw_open(struct i40e_info *cdev_info, struct i40e_client *client)
    {
    let mut l2params: irdma_l2params = {};
    struct irdma_device *iwdev;
    struct irdma_pci_f *rf;
    let mut err: c_int = -EIO;
    int i;
    u16 qset;
    let mut last_qset: u16 = IRDMA_NO_QSET;
    iwdev = ib_alloc_device(irdma_device, ibdev);
    if (!iwdev)
    return -ENOMEM;
    iwdev.rf = kzalloc_obj(*rf);
    if (!iwdev.rf) {
    ib_dealloc_device(&iwdev.ibdev);
    return -ENOMEM;
    }
    i40iw_fill_device_info(iwdev, cdev_info);
    rf = iwdev.rf;
    if (irdma_ctrl_init_hw(rf)) {
    err = -EIO;
    goto err_ctrl_init;
    }
    l2params.mtu = (cdev_info.params.mtu) ? cdev_info.params.mtu : IRDMA_DEFAULT_MTU;
    for (i = 0; i < I40E_CLIENT_MAX_USER_PRIORITY; i++) {
    qset = cdev_info.params.qos.prio_qos[i].qs_handle;
    l2params.up2tc[i] = cdev_info.params.qos.prio_qos[i].tc;
    l2params.qs_handle_list[i] = qset;
    if (last_qset == IRDMA_NO_QSET)
    last_qset = qset;
#[no_mangle]
pub unsafe extern "C" fn if(IRDMA_NO_QSET): (qset != last_qset) && (qset !=) -> else {
    else if ((qset != last_qset) && (qset != IRDMA_NO_QSET))
    iwdev.dcb_vlan_mode = true;
    }
    if (irdma_rt_init_hw(iwdev, &l2params)) {
    err = -EIO;
    goto err_rt_init;
    }
    err = irdma_ib_register_device(iwdev);
    if (err)
    goto err_ibreg;
    ibdev_dbg(&iwdev.ibdev, "INIT: Gen1 PF[%d] open success\n",
    PCI_FUNC(rf.pcidev.devfn));
    return 0;
    err_ibreg:
    irdma_rt_deinit_hw(iwdev);
    err_rt_init:
    irdma_ctrl_deinit_hw(rf);
    err_ctrl_init:
    kfree(iwdev.rf);
    ib_dealloc_device(&iwdev.ibdev);
    return err;
    }
// client interface functions
    static const struct i40e_client_ops i40e_ops = {
    .open = i40iw_open,
    .close = i40iw_close,
    .l2_param_change = i40iw_l2param_change
    };
    static struct i40e_client i40iw_client = {
    .ops = &i40e_ops,
    .type = I40E_CLIENT_IWARP,
    };
#[no_mangle]
unsafe extern "C" fn i40iw_probe(aux_dev: *mut auxiliary_device, id: *const auxiliary_device_id) -> c_int {
    static int i40iw_probe(struct auxiliary_device *aux_dev, const struct auxiliary_device_id *id)
    {
    struct i40e_auxiliary_device *i40e_adev = container_of(aux_dev,
    struct i40e_auxiliary_device,
    aux_dev);
    struct i40e_info *cdev_info = i40e_adev.ldev;
    strscpy_pad(i40iw_client.name, "irdma", I40E_CLIENT_STR_LENGTH);
    i40e_client_device_register(cdev_info, &i40iw_client);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i40iw_remove(aux_dev: *mut auxiliary_device) {
    static void i40iw_remove(struct auxiliary_device *aux_dev)
    {
    struct i40e_auxiliary_device *i40e_adev = container_of(aux_dev,
    struct i40e_auxiliary_device,
    aux_dev);
    struct i40e_info *cdev_info = i40e_adev.ldev;
    i40e_client_device_unregister(cdev_info);
    }
    static const struct auxiliary_device_id i40iw_auxiliary_id_table[] = {
    {.name = "i40e.iwarp", },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, i40iw_auxiliary_id_table);
    struct auxiliary_driver i40iw_auxiliary_drv = {
    .name = "gen_1",
    .id_table = i40iw_auxiliary_id_table,
    .probe = i40iw_probe,
    .remove = i40iw_remove,
    };
