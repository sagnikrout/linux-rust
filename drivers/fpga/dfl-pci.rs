//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-pci.c
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
// Driver for FPGA Device Feature List (DFL) PCIe device
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Zhang Yi <Yi.Z.Zhang@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Henry Mitchel <henry.mitchel@intel.com>
//

pub const PCI_VSEC_ID_INTEL_DFLS: c_uint = 0x43;
pub const PCI_VNDR_DFLS_CNT: c_uint = 0x8;
pub const PCI_VNDR_DFLS_RES: c_uint = 0xc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cci_drvdata {
    pub /: *mut *mut *mut dfl_fpga_cdev cdev; / container device,
}

#[no_mangle]
unsafe extern "C" fn cci_pci_alloc_irq(pcidev: *mut pci_dev) -> c_int {
    static int cci_pci_alloc_irq(struct pci_dev *pcidev)
    {
    int ret, nvec = pci_msix_vec_count(pcidev);
    if (nvec <= 0) {
    dev_dbg(&pcidev.dev, "fpga interrupt not supported\n");
    return 0;
    }
    ret = pci_alloc_irq_vectors(pcidev, nvec, nvec, PCI_IRQ_MSIX);
    if (ret < 0)
    return ret;
    return nvec;
    }
#[no_mangle]
unsafe extern "C" fn cci_pci_free_irq(pcidev: *mut pci_dev) {
    static void cci_pci_free_irq(struct pci_dev *pcidev)
    {
    pci_free_irq_vectors(pcidev);
    }
// PCI Device ID
pub const PCIE_DEVICE_ID_PF_INT_5_X: c_uint = 0xBCBD;
pub const PCIE_DEVICE_ID_PF_INT_6_X: c_uint = 0xBCC0;
pub const PCIE_DEVICE_ID_PF_DSC_1_X: c_uint = 0x09C4;
pub const PCIE_DEVICE_ID_INTEL_PAC_N3000: c_uint = 0x0B30;
pub const PCIE_DEVICE_ID_INTEL_PAC_D5005: c_uint = 0x0B2B;
pub const PCIE_DEVICE_ID_SILICOM_PAC_N5010: c_uint = 0x1000;
pub const PCIE_DEVICE_ID_SILICOM_PAC_N5011: c_uint = 0x1001;
pub const PCIE_DEVICE_ID_INTEL_DFL: c_uint = 0xbcce;
// PCI Subdevice ID for PCIE_DEVICE_ID_INTEL_DFL
pub const PCIE_SUBDEVICE_ID_INTEL_D5005: c_uint = 0x138d;
pub const PCIE_SUBDEVICE_ID_INTEL_N6000: c_uint = 0x1770;
pub const PCIE_SUBDEVICE_ID_INTEL_N6001: c_uint = 0x1771;
pub const PCIE_SUBDEVICE_ID_INTEL_C6100: c_uint = 0x17d4;
// VF Device
pub const PCIE_DEVICE_ID_VF_INT_5_X: c_uint = 0xBCBF;
pub const PCIE_DEVICE_ID_VF_INT_6_X: c_uint = 0xBCC1;
pub const PCIE_DEVICE_ID_VF_DSC_1_X: c_uint = 0x09C5;
pub const PCIE_DEVICE_ID_INTEL_PAC_D5005_VF: c_uint = 0x0B2C;
pub const PCIE_DEVICE_ID_INTEL_DFL_VF: c_uint = 0xbccf;
    static struct pci_device_id cci_pcie_id_tbl[] = {
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_PF_INT_5_X),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_VF_INT_5_X),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_PF_INT_6_X),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_VF_INT_6_X),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_PF_DSC_1_X),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_VF_DSC_1_X),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_PAC_N3000),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_PAC_D5005),},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_PAC_D5005_VF),},
    {PCI_DEVICE(PCI_VENDOR_ID_SILICOM_DENMARK, PCIE_DEVICE_ID_SILICOM_PAC_N5010),},
    {PCI_DEVICE(PCI_VENDOR_ID_SILICOM_DENMARK, PCIE_DEVICE_ID_SILICOM_PAC_N5011),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_D5005),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6000),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL_VF,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6000),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6001),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL_VF,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6001),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_C6100),},
    {PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL_VF,
    PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_C6100),},
    {0,}
    };
    MODULE_DEVICE_TABLE(pci, cci_pcie_id_tbl);
#[no_mangle]
unsafe extern "C" fn cci_init_drvdata(pcidev: *mut pci_dev) -> c_int {
    static int cci_init_drvdata(struct pci_dev *pcidev)
    {
    struct cci_drvdata *drvdata;
    drvdata = devm_kzalloc(&pcidev.dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    pci_set_drvdata(pcidev, drvdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cci_remove_feature_devs(pcidev: *mut pci_dev) {
    static void cci_remove_feature_devs(struct pci_dev *pcidev)
    {
    struct cci_drvdata *drvdata = pci_get_drvdata(pcidev);
// remove all children feature devices
    dfl_fpga_feature_devs_remove(drvdata.cdev);
    cci_pci_free_irq(pcidev);
    }
    static int *cci_pci_create_irq_table(struct pci_dev *pcidev, unsigned int nvec)
    {
    unsigned int i;
    int *table;
    table = kzalloc_objs(int, nvec);
    if (!table)
    return table;
    for (i = 0; i < nvec; i++)
    table[i] = pci_irq_vector(pcidev, i);
    return table;
    }
#[no_mangle]
unsafe extern "C" fn find_dfls_by_vsec(pcidev: *mut pci_dev, info: *mut dfl_fpga_enum_info) -> c_int {
    static int find_dfls_by_vsec(struct pci_dev *pcidev, struct dfl_fpga_enum_info *info)
    {
    u32 bir, offset, dfl_cnt, dfl_res;
    int dfl_res_off, i, bars, voff;
    resource_size_t start, len;
    voff = pci_find_vsec_capability(pcidev, PCI_VENDOR_ID_INTEL,
    PCI_VSEC_ID_INTEL_DFLS);
    if (!voff) {
    dev_dbg(&pcidev.dev, "%s no DFL VSEC found\n", __func__);
    return -ENODEV;
    }
    dfl_cnt = 0;
    pci_read_config_dword(pcidev, voff + PCI_VNDR_DFLS_CNT, &dfl_cnt);
    if (dfl_cnt > PCI_STD_NUM_BARS) {
    dev_err(&pcidev.dev, "%s too many DFLs %d > %d\n",
    __func__, dfl_cnt, PCI_STD_NUM_BARS);
    return -EINVAL;
    }
    dfl_res_off = voff + PCI_VNDR_DFLS_RES;
    if (dfl_res_off + (dfl_cnt * sizeof(u32)) > PCI_CFG_SPACE_EXP_SIZE) {
    dev_err(&pcidev.dev, "%s DFL VSEC too big for PCIe config space\n",
    __func__);
    return -EINVAL;
    }
    for (i = 0, bars = 0; i < dfl_cnt; i++, dfl_res_off += sizeof(u32)) {
    dfl_res = GENMASK(31, 0);
    pci_read_config_dword(pcidev, dfl_res_off, &dfl_res);
    bir = dfl_res & PCI_VNDR_DFLS_RES_BAR_MASK;
    if (bir >= PCI_STD_NUM_BARS) {
    dev_err(&pcidev.dev, "%s bad bir number %d\n",
    __func__, bir);
    return -EINVAL;
    }
    if (bars & BIT(bir)) {
    dev_err(&pcidev.dev, "%s DFL for BAR %d already specified\n",
    __func__, bir);
    return -EINVAL;
    }
    bars |= BIT(bir);
    len = pci_resource_len(pcidev, bir);
    offset = dfl_res & PCI_VNDR_DFLS_RES_OFF_MASK;
    if (offset >= len) {
    dev_err(&pcidev.dev, "%s bad offset %u >= %pa\n",
    __func__, offset, &len);
    return -EINVAL;
    }
    dev_dbg(&pcidev.dev, "%s BAR %d offset 0x%x\n", __func__, bir, offset);
    len -= offset;
    start = pci_resource_start(pcidev, bir) + offset;
    dfl_fpga_enum_info_add_dfl(info, start, len);
    }
    return 0;
    }
// default method of finding dfls starting at offset 0 of bar 0
    static int find_dfls_by_default(struct pci_dev *pcidev,
    struct dfl_fpga_enum_info *info)
    {
    int port_num, bar, i, ret = 0;
    resource_size_t start, len;
    void __iomem *base;
    u32 offset;
    u64 v;
// start to find Device Feature List from Bar 0
    base = pcim_iomap_region(pcidev, 0, DRV_NAME);
    if (IS_ERR(base))
    return PTR_ERR(base);
//
// PF device has FME and Ports/AFUs, and VF device only has one
// Port/AFU. Check them and add related "Device Feature List" info
// for the next step enumeration.
//
    if (dfl_feature_is_fme(base)) {
    start = pci_resource_start(pcidev, 0);
    len = pci_resource_len(pcidev, 0);
    dfl_fpga_enum_info_add_dfl(info, start, len);
//
// find more Device Feature Lists (e.g. Ports) per information
// indicated by FME module.
//
    v = readq(base + FME_HDR_CAP);
    port_num = FIELD_GET(FME_CAP_NUM_PORTS, v);
    WARN_ON(port_num > MAX_DFL_FPGA_PORT_NUM);
    for (i = 0; i < port_num; i++) {
    v = readq(base + FME_HDR_PORT_OFST(i));
// skip ports which are not implemented.
    if (!(v & FME_PORT_OFST_IMP))
    continue;
//
// add Port's Device Feature List information for next
// step enumeration.
//
    bar = FIELD_GET(FME_PORT_OFST_BAR_ID, v);
    offset = FIELD_GET(FME_PORT_OFST_DFH_OFST, v);
    if (bar == FME_PORT_OFST_BAR_SKIP) {
    continue;
    } else if (bar >= PCI_STD_NUM_BARS) {
    dev_err(&pcidev.dev, "bad BAR %d for port %d\n",
    bar, i);
    ret = -EINVAL;
    break;
    }
    start = pci_resource_start(pcidev, bar) + offset;
    len = pci_resource_len(pcidev, bar) - offset;
    dfl_fpga_enum_info_add_dfl(info, start, len);
    }
    } else if (dfl_feature_is_port(base)) {
    start = pci_resource_start(pcidev, 0);
    len = pci_resource_len(pcidev, 0);
    dfl_fpga_enum_info_add_dfl(info, start, len);
    } else {
    ret = -ENODEV;
    }
// release I/O mappings for next step enumeration
    pcim_iounmap_region(pcidev, 0);
    return ret;
    }
// enumerate feature devices under pci device
#[no_mangle]
unsafe extern "C" fn cci_enumerate_feature_devs(pcidev: *mut pci_dev) -> c_int {
    static int cci_enumerate_feature_devs(struct pci_dev *pcidev)
    {
    struct cci_drvdata *drvdata = pci_get_drvdata(pcidev);
    struct dfl_fpga_enum_info *info;
    struct dfl_fpga_cdev *cdev;
    int nvec, ret = 0;
    int *irq_table;
// allocate enumeration info via pci_dev
    info = dfl_fpga_enum_info_alloc(&pcidev.dev);
    if (!info)
    return -ENOMEM;
// add irq info for enumeration if the device support irq
    nvec = cci_pci_alloc_irq(pcidev);
    if (nvec < 0) {
    dev_err(&pcidev.dev, "Fail to alloc irq %d.\n", nvec);
    ret = nvec;
    goto enum_info_free_exit;
    } else if (nvec) {
    irq_table = cci_pci_create_irq_table(pcidev, nvec);
    if (!irq_table) {
    ret = -ENOMEM;
    goto irq_free_exit;
    }
    ret = dfl_fpga_enum_info_add_irq(info, nvec, irq_table);
    kfree(irq_table);
    if (ret)
    goto irq_free_exit;
    }
    ret = find_dfls_by_vsec(pcidev, info);
    if (ret == -ENODEV)
    ret = find_dfls_by_default(pcidev, info);
    if (ret)
    goto irq_free_exit;
// start enumeration with prepared enumeration information
    cdev = dfl_fpga_feature_devs_enumerate(info);
    if (IS_ERR(cdev)) {
    dev_err(&pcidev.dev, "Enumeration failure\n");
    ret = PTR_ERR(cdev);
    goto irq_free_exit;
    }
    drvdata.cdev = cdev;
    irq_free_exit:
    if (ret)
    cci_pci_free_irq(pcidev);
    enum_info_free_exit:
    dfl_fpga_enum_info_free(info);
    return ret;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn cci_pci_probe(pcidev: *mut pci_dev, pcidevid: *const pci_device_id) -> c_int {
    int cci_pci_probe(struct pci_dev *pcidev, const struct pci_device_id *pcidevid)
    {
    int ret;
    ret = pcim_enable_device(pcidev);
    if (ret < 0) {
    dev_err(&pcidev.dev, "Failed to enable device %d.\n", ret);
    return ret;
    }
    pci_set_master(pcidev);
    ret = dma_set_mask_and_coherent(&pcidev.dev, DMA_BIT_MASK(64));
    if (ret)
    ret = dma_set_mask_and_coherent(&pcidev.dev, DMA_BIT_MASK(32));
    if (ret) {
    dev_err(&pcidev.dev, "No suitable DMA support available.\n");
    return ret;
    }
    ret = cci_init_drvdata(pcidev);
    if (ret) {
    dev_err(&pcidev.dev, "Fail to init drvdata %d.\n", ret);
    return ret;
    }
    ret = cci_enumerate_feature_devs(pcidev);
    if (ret) {
    dev_err(&pcidev.dev, "enumeration failure %d.\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cci_pci_sriov_configure(pcidev: *mut pci_dev, num_vfs: c_int) -> c_int {
    static int cci_pci_sriov_configure(struct pci_dev *pcidev, int num_vfs)
    {
    struct cci_drvdata *drvdata = pci_get_drvdata(pcidev);
    struct dfl_fpga_cdev *cdev = drvdata.cdev;
    if (!num_vfs) {
//
// disable SRIOV and then put released ports back to default
// PF access mode.
//
    pci_disable_sriov(pcidev);
    dfl_fpga_cdev_config_ports_pf(cdev);
    } else {
    int ret;
//
// before enable SRIOV, put released ports into VF access mode
// first of all.
//
    ret = dfl_fpga_cdev_config_ports_vf(cdev, num_vfs);
    if (ret)
    return ret;
    ret = pci_enable_sriov(pcidev, num_vfs);
    if (ret) {
    dfl_fpga_cdev_config_ports_pf(cdev);
    return ret;
    }
    }
    return num_vfs;
    }
#[no_mangle]
unsafe extern "C" fn cci_pci_remove(pcidev: *mut pci_dev) {
    static void cci_pci_remove(struct pci_dev *pcidev)
    {
    if (dev_is_pf(&pcidev.dev))
    cci_pci_sriov_configure(pcidev, 0);
    cci_remove_feature_devs(pcidev);
    }
    static struct pci_driver cci_pci_driver = {
    .name = DRV_NAME,
    .id_table = cci_pcie_id_tbl,
    .probe = cci_pci_probe,
    .remove = cci_pci_remove,
    .sriov_configure = cci_pci_sriov_configure,
    };
    module_pci_driver(cci_pci_driver);
    MODULE_DESCRIPTION("FPGA DFL PCIe Device Driver");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
