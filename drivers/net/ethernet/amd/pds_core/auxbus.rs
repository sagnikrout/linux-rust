//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/amd/pds_core/auxbus.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2023 Advanced Micro Devices, Inc

//
// pds_client_register - Link the client to the firmware
// @pf:		ptr to the PF driver's private data struct
// @devname:	name that includes service into, e.g. pds_core.vDPA
//
// Return: positive client ID (ci) on success, or
// negative for error
//
#[no_mangle]
pub unsafe extern "C" fn pds_client_register(pf: *mut pdsc, devname: *mut c_char) -> c_int {
    int pds_client_register(struct pdsc *pf, char *devname)
    {
    let mut comp: union pds_core_adminq_comp = {};
    let mut cmd: union pds_core_adminq_cmd = {};
    int err;
    u16 ci;
    cmd.client_reg.opcode = PDS_AQ_CMD_CLIENT_REG;
    strscpy(cmd.client_reg.devname, devname,
    sizeof(cmd.client_reg.devname));
    err = pdsc_adminq_post(pf, &cmd, &comp, false);
    if (err) {
    dev_info(pf.dev, "register dev_name %s with DSC failed, status %d: %pe\n",
    devname, comp.status, ERR_PTR(err));
    return err;
    }
    ci = le16_to_cpu(comp.client_reg.client_id);
    if (!ci) {
    dev_err(pf.dev, "%s: device returned null client_id\n",
    __func__);
    return -EIO;
    }
    dev_dbg(pf.dev, "%s: device returned client_id %d for %s\n",
    __func__, ci, devname);
    return ci;
    }
    EXPORT_SYMBOL_GPL(pds_client_register);
//
// pds_client_unregister - Unlink the client from the firmware
// @pf:		ptr to the PF driver's private data struct
// @client_id:	id returned from pds_client_register()
//
// Return: 0 on success, or
// negative for error
//
#[no_mangle]
pub unsafe extern "C" fn pds_client_unregister(pf: *mut pdsc, client_id: u16) -> c_int {
    int pds_client_unregister(struct pdsc *pf, u16 client_id)
    {
    let mut comp: union pds_core_adminq_comp = {};
    let mut cmd: union pds_core_adminq_cmd = {};
    int err;
    cmd.client_unreg.opcode = PDS_AQ_CMD_CLIENT_UNREG;
    cmd.client_unreg.client_id = cpu_to_le16(client_id);
    err = pdsc_adminq_post(pf, &cmd, &comp, false);
    if (err)
    dev_info(pf.dev, "unregister client_id %d failed, status %d: %pe\n",
    client_id, comp.status, ERR_PTR(err));
    return err;
    }
    EXPORT_SYMBOL_GPL(pds_client_unregister);
//
// pds_client_adminq_cmd - Process an adminq request for the client
// @padev:   ptr to the client device
// @req:     ptr to buffer with request
// @req_len: length of actual struct used for request
// @resp:    ptr to buffer where answer is to be copied
// @flags:   optional flags from pds_core_adminq_flags
//
// Return: 0 on success, or
// negative for error
//
// Client sends pointers to request and response buffers
// Core copies request data into pds_core_client_request_cmd
// Core sets other fields as needed
// Core posts to AdminQ
// Core copies completion data into response buffer
//
    int pds_client_adminq_cmd(struct pds_auxiliary_dev *padev,
    union pds_core_adminq_cmd *req,
    size_t req_len,
    union pds_core_adminq_comp *resp,
    u64 flags)
    {
    let mut cmd: union pds_core_adminq_cmd = {};
    struct pci_dev *pf_pdev;
    struct pdsc *pf;
    size_t cp_len;
    int err;
    pf_pdev = pci_physfn(padev.vf_pdev);
    pf = pci_get_drvdata(pf_pdev);
    dev_dbg(pf.dev, "%s: %s opcode %d\n",
    __func__, dev_name(&padev.aux_dev.dev), req.opcode);
// Wrap the client's request
    cmd.client_request.opcode = PDS_AQ_CMD_CLIENT_CMD;
    cmd.client_request.client_id = cpu_to_le16(padev.client_id);
    cp_len = min_t(size_t, req_len, sizeof(cmd.client_request.client_cmd));
    memcpy(cmd.client_request.client_cmd, req, cp_len);
    err = pdsc_adminq_post(pf, &cmd, resp,
    !!(flags & PDS_AQ_FLAG_FASTPOLL));
    if (err && err != -EAGAIN)
    dev_info(pf.dev, "client admin cmd failed: %pe\n",
    ERR_PTR(err));
    return err;
    }
    EXPORT_SYMBOL_GPL(pds_client_adminq_cmd);
#[no_mangle]
unsafe extern "C" fn pdsc_auxbus_dev_release(dev: *mut device) {
    static void pdsc_auxbus_dev_release(struct device *dev)
    {
    struct pds_auxiliary_dev *padev =
    container_of(dev, struct pds_auxiliary_dev, aux_dev.dev);
    kfree(padev);
    }
    static struct pds_auxiliary_dev *pdsc_auxbus_dev_register(struct pdsc *cf,
    struct pdsc *pf,
    u16 client_id,
    char *name)
    {
    struct auxiliary_device *aux_dev;
    struct pds_auxiliary_dev *padev;
    int err;
    padev = kzalloc_obj(*padev);
    if (!padev)
    return ERR_PTR(-ENOMEM);
    padev.vf_pdev = cf.pdev;
    padev.client_id = client_id;
    aux_dev = &padev.aux_dev;
    aux_dev.name = name;
    aux_dev.id = cf.uid;
    aux_dev.dev.parent = cf.dev;
    aux_dev.dev.release = pdsc_auxbus_dev_release;
    err = auxiliary_device_init(aux_dev);
    if (err < 0) {
    dev_warn(cf.dev, "auxiliary_device_init of %s failed: %pe\n",
    name, ERR_PTR(err));
    kfree(padev);
    return ERR_PTR(err);
    }
    err = auxiliary_device_add(aux_dev);
    if (err) {
    dev_warn(cf.dev, "auxiliary_device_add of %s failed: %pe\n",
    name, ERR_PTR(err));
    auxiliary_device_uninit(aux_dev);
    return ERR_PTR(err);
    }
    return padev;
    }
    void pdsc_auxbus_dev_del(struct pdsc *cf, struct pdsc *pf,
    struct pds_auxiliary_dev **pd_ptr)
    {
    struct pds_auxiliary_dev *padev;
    mutex_lock(&pf.config_lock);
// A concurrent del may have already torn this device down and
// cleared it.
//
    padev = *pd_ptr;
    if (!padev)
    goto out_unlock;
    pds_client_unregister(pf, padev.client_id);
    auxiliary_device_delete(&padev.aux_dev);
    auxiliary_device_uninit(&padev.aux_dev);
// pd_ptr = NULL;
    out_unlock:
    mutex_unlock(&pf.config_lock);
    }
    int pdsc_auxbus_dev_add(struct pdsc *cf, struct pdsc *pf,
    enum pds_core_vif_types vt,
    struct pds_auxiliary_dev **pd_ptr)
    {
    struct pds_auxiliary_dev *padev;
    char devname[PDS_DEVNAME_LEN];
    unsigned long mask;
    u16 vt_support;
    int client_id;
    let mut err: c_int = 0;
    if (!cf)
    return -ENODEV;
    if (vt >= PDS_DEV_TYPE_MAX)
    return -EINVAL;
    mutex_lock(&pf.config_lock);
// Nothing to do if the aux device is already present.  This also
// guards against a second add overwriting *pd_ptr and leaking the
// first, symmetric with the check in pdsc_auxbus_dev_del().
//
    if (*pd_ptr)
    goto out_unlock;
    mask = BIT_ULL(PDSC_S_FW_DEAD) |
    BIT_ULL(PDSC_S_STOPPING_DRIVER);
    if (cf.state & mask) {
    dev_err(pf.dev, "%s: can't add dev, VF client in bad state %#lx\n",
    __func__, cf.state);
    err = -ENXIO;
    goto out_unlock;
    }
// Verify that the type is supported and enabled.  It is not
// an error if the firmware doesn't support the feature, the
// driver just won't set up an auxiliary_device for it.
//
    vt_support = !!le16_to_cpu(pf.dev_ident.vif_types[vt]);
    if (!(vt_support &&
    pf.viftype_status[vt].supported &&
    pf.viftype_status[vt].enabled))
    goto out_unlock;
// Need to register with FW and get the client_id before
// creating the aux device so that the aux client can run
// adminq commands as part its probe
//
    snprintf(devname, sizeof(devname), "%s.%s.%d",
    PDS_CORE_DRV_NAME, pf.viftype_status[vt].name, cf.uid);
    client_id = pds_client_register(pf, devname);
    if (client_id < 0) {
    err = client_id;
    goto out_unlock;
    }
    padev = pdsc_auxbus_dev_register(cf, pf, client_id,
    pf.viftype_status[vt].name);
    if (IS_ERR(padev)) {
    pds_client_unregister(pf, client_id);
    err = PTR_ERR(padev);
    goto out_unlock;
    }
// pd_ptr = padev;
    out_unlock:
    mutex_unlock(&pf.config_lock);
    return err;
    }
