//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/wangxun/ngbevf/ngbevf_main.c
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
// Copyright (c) 2015 - 2025 Beijing WangXun Technology Co., Ltd.

// ngbevf_pci_tbl - PCI Device ID Table
//
// Wildcard entries (PCI_ANY_ID) should come last
// Last entry must be all 0s
//
// { Vendor ID, Device ID, SubVendor ID, SubDevice ID,
// Class, Class Mask, private data (not used) }
//
    static const struct pci_device_id ngbevf_pci_tbl[] = {
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860AL_W) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860A2) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860A2S) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860A4) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860A4S) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860AL2) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860AL2S) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860AL4) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860AL4S) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860NCSI) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860A1) },
    { PCI_VDEVICE(WANGXUN, NGBEVF_DEV_ID_EM_WX1860AL1) },
// required last entry
    { }
    };
    static const struct net_device_ops ngbevf_netdev_ops = {
    .ndo_open               = wxvf_open,
    .ndo_stop               = wxvf_close,
    .ndo_start_xmit         = wx_xmit_frame,
    .ndo_validate_addr      = eth_validate_addr,
    .ndo_set_mac_address    = wx_set_mac_vf,
    };
#[no_mangle]
unsafe extern "C" fn ngbevf_set_num_queues(wx: *mut wx) {
    static void ngbevf_set_num_queues(struct wx *wx)
    {
// Start with base case
    wx.num_rx_queues = 1;
    wx.num_tx_queues = 1;
    }
#[no_mangle]
unsafe extern "C" fn ngbevf_sw_init(wx: *mut wx) -> c_int {
    static int ngbevf_sw_init(struct wx *wx)
    {
    struct net_device *netdev = wx.netdev;
    struct pci_dev *pdev = wx.pdev;
    int err;
// Initialize pcie info and common capability flags
    err = wx_sw_init(wx);
    if (err < 0)
    goto err_wx_sw_init;
// Initialize the mailbox
    err = wx_init_mbx_params_vf(wx);
    if (err)
    goto err_init_mbx_params;
// Initialize the device type
    wx.mac.type = wx_mac_em;
    wx.mac.max_msix_vectors = NGBEVF_MAX_MSIX_VECTORS;
// lock to protect mailbox accesses
    spin_lock_init(&wx.mbx.mbx_lock);
    err = wx_reset_hw_vf(wx);
    if (err) {
    wx_err(wx, "PF still in reset state. Is the PF interface up?\n");
    goto err_reset_hw;
    }
    wx_init_hw_vf(wx);
    wx_negotiate_api_vf(wx);
    if (is_zero_ether_addr(wx.mac.addr))
    dev_info(&pdev.dev,
    "MAC address not assigned by administrator.\n");
    eth_hw_addr_set(netdev, wx.mac.addr);
    if (!is_valid_ether_addr(netdev.dev_addr)) {
    dev_info(&pdev.dev, "Assigning random MAC address\n");
    eth_hw_addr_random(netdev);
    ether_addr_copy(wx.mac.addr, netdev.dev_addr);
    ether_addr_copy(wx.mac.perm_addr, netdev.dev_addr);
    }
    wx.mac.max_tx_queues = NGBEVF_MAX_TX_QUEUES;
    wx.mac.max_rx_queues = NGBEVF_MAX_RX_QUEUES;
// Enable dynamic interrupt throttling rates
    wx.adaptive_itr = true;
    wx.rx_itr_setting = 1;
    wx.tx_itr_setting = 1;
// set default ring sizes
    wx.tx_ring_count = NGBEVF_DEFAULT_TXD;
    wx.rx_ring_count = NGBEVF_DEFAULT_RXD;
// set default work limits
    wx.tx_work_limit = NGBEVF_DEFAULT_TX_WORK;
    wx.rx_work_limit = NGBEVF_DEFAULT_RX_WORK;
    wx.set_num_queues = ngbevf_set_num_queues;
    return 0;
    err_reset_hw:
    kfree(wx.vfinfo);
    err_init_mbx_params:
    kfree(wx.rss_key);
    kfree(wx.mac_table);
    err_wx_sw_init:
    return err;
    }
//
// ngbevf_probe - Device Initialization Routine
// @pdev: PCI device information struct
// @ent: entry in ngbevf_pci_tbl
//
// Return: return 0 on success, negative on failure
//
// ngbevf_probe initializes an adapter identified by a pci_dev structure.
// The OS initialization, configuring of the adapter private structure,
// and a hardware reset occur.
//
    static int ngbevf_probe(struct pci_dev *pdev,
    const struct pci_device_id __always_unused *ent)
    {
    struct net_device *netdev;
    struct wx *wx = core::ptr::null_mut();
    int err;
    err = pci_enable_device_mem(pdev);
    if (err)
    return err;
    err = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (err) {
    dev_err(&pdev.dev,
    "No usable DMA configuration, aborting\n");
    goto err_pci_disable_dev;
    }
    err = pci_request_selected_regions(pdev,
    pci_select_bars(pdev, IORESOURCE_MEM),
    dev_driver_string(&pdev.dev));
    if (err) {
    dev_err(&pdev.dev,
    "pci_request_selected_regions failed 0x%x\n", err);
    goto err_pci_disable_dev;
    }
    pci_set_master(pdev);
    netdev = devm_alloc_etherdev_mqs(&pdev.dev,
    sizeof(struct wx),
    NGBEVF_MAX_TX_QUEUES,
    NGBEVF_MAX_RX_QUEUES);
    if (!netdev) {
    err = -ENOMEM;
    goto err_pci_release_regions;
    }
    SET_NETDEV_DEV(netdev, &pdev.dev);
    wx = netdev_priv(netdev);
    wx.netdev = netdev;
    wx.pdev = pdev;
    wx.msg_enable = netif_msg_init(-1, NETIF_MSG_DRV |
    NETIF_MSG_PROBE | NETIF_MSG_LINK);
    wx.hw_addr = devm_ioremap(&pdev.dev,
    pci_resource_start(pdev, 0),
    pci_resource_len(pdev, 0));
    if (!wx.hw_addr) {
    err = -EIO;
    goto err_pci_release_regions;
    }
    wx.driver_name = KBUILD_MODNAME;
    wx_set_ethtool_ops_vf(netdev);
    netdev.netdev_ops = &ngbevf_netdev_ops;
// setup the private structure
    err = ngbevf_sw_init(wx);
    if (err)
    goto err_pci_release_regions;
    netdev.features |= NETIF_F_HIGHDMA;
    eth_hw_addr_set(netdev, wx.mac.perm_addr);
    ether_addr_copy(netdev.perm_addr, wx.mac.addr);
    wxvf_init_service(wx);
    err = wx_init_interrupt_scheme(wx);
    if (err)
    goto err_free_sw_init;
    wx_get_fw_version_vf(wx);
    err = register_netdev(netdev);
    if (err)
    goto err_register;
    pci_set_drvdata(pdev, wx);
    netif_tx_stop_all_queues(netdev);
    return 0;
    err_register:
    wx_clear_interrupt_scheme(wx);
    err_free_sw_init:
    timer_delete_sync(&wx.service_timer);
    cancel_work_sync(&wx.service_task);
    kfree(wx.vfinfo);
    kfree(wx.rss_key);
    kfree(wx.mac_table);
    err_pci_release_regions:
    pci_release_selected_regions(pdev,
    pci_select_bars(pdev, IORESOURCE_MEM));
    err_pci_disable_dev:
    pci_disable_device(pdev);
    return err;
    }
//
// ngbevf_remove - Device Removal Routine
// @pdev: PCI device information struct
//
// ngbevf_remove is called by the PCI subsystem to alert the driver
// that it should release a PCI device.  The could be caused by a
// Hot-Plug event, or because the driver is going to be removed from
// memory.
//
#[no_mangle]
unsafe extern "C" fn ngbevf_remove(pdev: *mut pci_dev) {
    static void ngbevf_remove(struct pci_dev *pdev)
    {
    wxvf_remove(pdev);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ngbevf_pm_ops, wxvf_suspend, wxvf_resume);
    static struct pci_driver ngbevf_driver = {
    .name     = KBUILD_MODNAME,
    .id_table = ngbevf_pci_tbl,
    .probe    = ngbevf_probe,
    .remove   = ngbevf_remove,
    .shutdown = wxvf_shutdown,
// Power Management Hooks
    .driver.pm	= pm_sleep_ptr(&ngbevf_pm_ops)
    };
    module_pci_driver(ngbevf_driver);
    MODULE_DEVICE_TABLE(pci, ngbevf_pci_tbl);
    MODULE_AUTHOR("Beijing WangXun Technology Co., Ltd, <software@trustnetic.com>");
    MODULE_DESCRIPTION("WangXun(R) Gigabit PCI Express Network Driver");
    MODULE_LICENSE("GPL");
