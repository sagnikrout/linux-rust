//! Automatically rewritten from C to Rust
//! Source: drivers/ufs/host/cdns-pltfrm.c
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
//
// Platform UFS Host driver for Cadence controller
//
// Copyright (C) 2018 Cadence Design Systems, Inc.
//
// Authors:
// Jan Kotas <jank@cadence.com>
//

pub const CDNS_UFS_REG_HCLKDIV: c_uint = 0xFC;
pub const CDNS_UFS_REG_PHY_XCFGD1: c_uint = 0x113C;
pub const CDNS_UFS_MAX_L4_ATTRS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_ufs_host {
//
// cdns_ufs_dme_attr_val - for storing L4 attributes
//
    pub cdns_ufs_dme_attr_val: [u32; CDNS_UFS_MAX_L4_ATTRS],
}

//
// cdns_ufs_get_l4_attr - get L4 attributes on local side
// @hba: per adapter instance
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_get_l4_attr(hba: *mut ufs_hba) {
    static void cdns_ufs_get_l4_attr(struct ufs_hba *hba)
    {
    struct cdns_ufs_host *host = ufshcd_get_variant(hba);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_PEERDEVICEID),
    &host.cdns_ufs_dme_attr_val[0]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_PEERCPORTID),
    &host.cdns_ufs_dme_attr_val[1]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_TRAFFICCLASS),
    &host.cdns_ufs_dme_attr_val[2]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_PROTOCOLID),
    &host.cdns_ufs_dme_attr_val[3]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_CPORTFLAGS),
    &host.cdns_ufs_dme_attr_val[4]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_TXTOKENVALUE),
    &host.cdns_ufs_dme_attr_val[5]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_RXTOKENVALUE),
    &host.cdns_ufs_dme_attr_val[6]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_LOCALBUFFERSPACE),
    &host.cdns_ufs_dme_attr_val[7]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_PEERBUFFERSPACE),
    &host.cdns_ufs_dme_attr_val[8]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_CREDITSTOSEND),
    &host.cdns_ufs_dme_attr_val[9]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_CPORTMODE),
    &host.cdns_ufs_dme_attr_val[10]);
    ufshcd_dme_get(hba, UIC_ARG_MIB(T_CONNECTIONSTATE),
    &host.cdns_ufs_dme_attr_val[11]);
    }
//
// cdns_ufs_set_l4_attr - set L4 attributes on local side
// @hba: per adapter instance
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_set_l4_attr(hba: *mut ufs_hba) {
    static void cdns_ufs_set_l4_attr(struct ufs_hba *hba)
    {
    struct cdns_ufs_host *host = ufshcd_get_variant(hba);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_CONNECTIONSTATE), 0);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_PEERDEVICEID),
    host.cdns_ufs_dme_attr_val[0]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_PEERCPORTID),
    host.cdns_ufs_dme_attr_val[1]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_TRAFFICCLASS),
    host.cdns_ufs_dme_attr_val[2]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_PROTOCOLID),
    host.cdns_ufs_dme_attr_val[3]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_CPORTFLAGS),
    host.cdns_ufs_dme_attr_val[4]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_TXTOKENVALUE),
    host.cdns_ufs_dme_attr_val[5]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_RXTOKENVALUE),
    host.cdns_ufs_dme_attr_val[6]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_LOCALBUFFERSPACE),
    host.cdns_ufs_dme_attr_val[7]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_PEERBUFFERSPACE),
    host.cdns_ufs_dme_attr_val[8]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_CREDITSTOSEND),
    host.cdns_ufs_dme_attr_val[9]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_CPORTMODE),
    host.cdns_ufs_dme_attr_val[10]);
    ufshcd_dme_set(hba, UIC_ARG_MIB(T_CONNECTIONSTATE),
    host.cdns_ufs_dme_attr_val[11]);
    }
//
// cdns_ufs_set_hclkdiv() - set HCLKDIV register value based on the core_clk.
// @hba: host controller instance
//
// Return: zero for success and non-zero for failure.
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_set_hclkdiv(hba: *mut ufs_hba) -> c_int {
    static int cdns_ufs_set_hclkdiv(struct ufs_hba *hba)
    {
    struct ufs_clk_info *clki;
    struct list_head *head = &hba.clk_list_head;
    let mut core_clk_rate: c_ulong = 0;
    let mut core_clk_div: u32 = 0;
    if (list_empty(head))
    return 0;
    list_for_each_entry(clki, head, list) {
    if (IS_ERR_OR_NULL(clki.clk))
    continue;
    if (!strcmp(clki.name, "core_clk"))
    core_clk_rate = clk_get_rate(clki.clk);
    }
    if (!core_clk_rate) {
    dev_err(hba.dev, "%s: unable to find core_clk rate\n",
    __func__);
    return -EINVAL;
    }
    core_clk_div = core_clk_rate / USEC_PER_SEC;
    ufshcd_writel(hba, core_clk_div, CDNS_UFS_REG_HCLKDIV);
//
// Make sure the register was updated,
// UniPro layer will not work with an incorrect value.
//
    ufshcd_readl(hba, CDNS_UFS_REG_HCLKDIV);
    return 0;
    }
//
// cdns_ufs_hce_enable_notify() - set HCLKDIV register
// @hba: host controller instance
// @status: notify stage (pre, post change)
//
// Return: zero for success and non-zero for failure.
//
    static int cdns_ufs_hce_enable_notify(struct ufs_hba *hba,
    enum ufs_notify_change_status status)
    {
    if (status != PRE_CHANGE)
    return 0;
    return cdns_ufs_set_hclkdiv(hba);
    }
//
// cdns_ufs_hibern8_notify() - save and restore L4 attributes.
// @hba: host controller instance
// @cmd: UIC Command
// @status: notify stage (pre, post change)
//
    static void cdns_ufs_hibern8_notify(struct ufs_hba *hba, enum uic_cmd_dme cmd,
    enum ufs_notify_change_status status)
    {
    if (status == PRE_CHANGE && cmd == UIC_CMD_DME_HIBER_ENTER)
    cdns_ufs_get_l4_attr(hba);
    if (status == POST_CHANGE && cmd == UIC_CMD_DME_HIBER_EXIT)
    cdns_ufs_set_l4_attr(hba);
    }
//
// cdns_ufs_link_startup_notify() - handle link startup.
// @hba: host controller instance
// @status: notify stage (pre, post change)
//
// Return: zero for success and non-zero for failure.
//
    static int cdns_ufs_link_startup_notify(struct ufs_hba *hba,
    enum ufs_notify_change_status status)
    {
    if (status != PRE_CHANGE)
    return 0;
//
// Some UFS devices have issues if LCC is enabled.
// So we are setting PA_Local_TX_LCC_Enable to 0
// before link startup which will make sure that both host
// and device TX LCC are disabled once link startup is
// completed.
//
    ufshcd_disable_host_tx_lcc(hba);
//
// Disabling Autohibern8 feature in cadence UFS
// to mask unexpected interrupt trigger.
//
    hba.ahit = 0;
    return 0;
    }
//
// cdns_ufs_init - performs additional ufs initialization
// @hba: host controller instance
//
// Return: status of initialization.
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_init(hba: *mut ufs_hba) -> c_int {
    static int cdns_ufs_init(struct ufs_hba *hba)
    {
    let mut status: c_int = 0;
    struct cdns_ufs_host *host;
    struct device *dev = hba.dev;
    host = devm_kzalloc(dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    ufshcd_set_variant(hba, host);
    status = ufshcd_vops_phy_initialization(hba);
    return status;
    }
//
// cdns_ufs_m31_16nm_phy_initialization - performs m31 phy initialization
// @hba: host controller instance
//
// Return: 0 (success).
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_m31_16nm_phy_initialization(hba: *mut ufs_hba) -> c_int {
    static int cdns_ufs_m31_16nm_phy_initialization(struct ufs_hba *hba)
    {
    u32 data;
// Increase RX_Advanced_Min_ActivateTime_Capability
    data = ufshcd_readl(hba, CDNS_UFS_REG_PHY_XCFGD1);
    data |= BIT(24);
    ufshcd_writel(hba, data, CDNS_UFS_REG_PHY_XCFGD1);
    return 0;
    }
    static const struct ufs_hba_variant_ops cdns_ufs_pltfm_hba_vops = {
    .name = "cdns-ufs-pltfm",
    .init = cdns_ufs_init,
    .hce_enable_notify = cdns_ufs_hce_enable_notify,
    .link_startup_notify = cdns_ufs_link_startup_notify,
    .hibern8_notify = cdns_ufs_hibern8_notify,
    };
    static const struct ufs_hba_variant_ops cdns_ufs_m31_16nm_pltfm_hba_vops = {
    .name = "cdns-ufs-pltfm",
    .init = cdns_ufs_init,
    .hce_enable_notify = cdns_ufs_hce_enable_notify,
    .link_startup_notify = cdns_ufs_link_startup_notify,
    .phy_initialization = cdns_ufs_m31_16nm_phy_initialization,
    .hibern8_notify = cdns_ufs_hibern8_notify,
    };
    static const struct of_device_id cdns_ufs_of_match[] = {
    {
    .compatible = "cdns,ufshc",
    .data =  &cdns_ufs_pltfm_hba_vops,
    },
    {
    .compatible = "cdns,ufshc-m31-16nm",
    .data =  &cdns_ufs_m31_16nm_pltfm_hba_vops,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, cdns_ufs_of_match);
//
// cdns_ufs_pltfrm_probe - probe routine of the driver
// @pdev: pointer to platform device handle
//
// Return: zero for success and non-zero for failure.
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_pltfrm_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_ufs_pltfrm_probe(struct platform_device *pdev)
    {
    int err;
    const struct of_device_id *of_id;
    struct ufs_hba_variant_ops *vops;
    struct device *dev = &pdev.dev;
    of_id = of_match_node(cdns_ufs_of_match, dev.of_node);
    vops = (struct ufs_hba_variant_ops *)of_id.data;
// Perform generic probe
    err = ufshcd_pltfrm_init(pdev, vops);
    if (err)
    dev_err(dev, "ufshcd_pltfrm_init() failed %d\n", err);
    return err;
    }
//
// cdns_ufs_pltfrm_remove - removes the ufs driver
// @pdev: pointer to platform device handle
//
// Return: 0 (success).
//
#[no_mangle]
unsafe extern "C" fn cdns_ufs_pltfrm_remove(pdev: *mut platform_device) {
    static void cdns_ufs_pltfrm_remove(struct platform_device *pdev)
    {
    ufshcd_pltfrm_remove(pdev);
    }
    static const struct dev_pm_ops cdns_ufs_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(ufshcd_system_suspend, ufshcd_system_resume)
    SET_RUNTIME_PM_OPS(ufshcd_runtime_suspend, ufshcd_runtime_resume, core::ptr::null_mut())
    .prepare	 = ufshcd_suspend_prepare,
    .complete	 = ufshcd_resume_complete,
    };
    static struct platform_driver cdns_ufs_pltfrm_driver = {
    .probe	= cdns_ufs_pltfrm_probe,
    .remove = cdns_ufs_pltfrm_remove,
    .driver	= {
    .name   = "cdns-ufshcd",
    .pm     = &cdns_ufs_dev_pm_ops,
    .of_match_table = cdns_ufs_of_match,
    },
    };
    module_platform_driver(cdns_ufs_pltfrm_driver);
    MODULE_AUTHOR("Jan Kotas <jank@cadence.com>");
    MODULE_DESCRIPTION("Cadence UFS host controller platform driver");
    MODULE_LICENSE("GPL v2");
