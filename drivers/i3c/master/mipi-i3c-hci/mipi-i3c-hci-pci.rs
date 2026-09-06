//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/mipi-i3c-hci/mipi-i3c-hci-pci.c
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
// PCI glue code for MIPI I3C HCI driver
//
// Copyright (C) 2024 Intel Corporation
//
// Author: Jarkko Nikula <jarkko.nikula@linux.intel.com>
//

//
// There can up to 15 instances, but implementations have at most 2 at this
// time.
//
pub const INST_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_i3c_hci_pci_instance {
    pub dev: *mut device,
    pub operational: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_i3c_hci_pci {
    pub pci: *mut pci_dev,
    pub base: *mut void __iomem,
    pub info: *const mipi_i3c_hci_pci_info,
    pub instance: [mipi_i3c_hci_pci_instance; INST_MAX],
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_i3c_hci_pci_info {
    pub hci): *mut *mut int (init)(struct mipi_i3c_hci_pci,
    pub hci): *mut *mut void (exit)(struct mipi_i3c_hci_pci,
    pub name: *const c_char,
    pub id: [c_int; INST_MAX],
    pub instance_offset: [u32; INST_MAX],
    pub instance_count: c_int,
    pub control_instance_pm: bool,
}

pub const INTEL_PRIV_OFFSET: c_uint = 0x2b0;
pub const INTEL_RESETS: c_uint = 0x04;

pub const INTEL_ACTIVELTR: c_uint = 0x0c;
pub const INTEL_IDLELTR: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_host {
    pub priv: *mut void __iomem,
    pub active_ltr: u32,
    pub idle_ltr: u32,
    pub debugfs_root: *mut dentry,
}

#[no_mangle]
unsafe extern "C" fn intel_cache_ltr(host: *mut intel_host) {
    static void intel_cache_ltr(struct intel_host *host)
    {
    host.active_ltr = readl(host.priv + INTEL_ACTIVELTR);
    host.idle_ltr = readl(host.priv + INTEL_IDLELTR);
    }
#[no_mangle]
unsafe extern "C" fn intel_ltr_set(dev: *mut device, val: i32) {
    static void intel_ltr_set(struct device *dev, s32 val)
    {
    struct mipi_i3c_hci_pci *hci = dev_get_drvdata(dev);
    struct intel_host *host = hci.private;
    u32 ltr;
//
// Program latency tolerance (LTR) accordingly what has been asked
// by the PM QoS layer or disable it in case we were passed
// negative value or PM_QOS_LATENCY_ANY.
//
    ltr = readl(host.priv + INTEL_ACTIVELTR);
    if (val == PM_QOS_LATENCY_ANY || val < 0) {
    ltr &= ~INTEL_LTR_REQ;
    } else {
    ltr |= INTEL_LTR_REQ;
    ltr &= ~INTEL_LTR_SCALE_MASK;
    ltr &= ~INTEL_LTR_VALUE_MASK;
    if (val > INTEL_LTR_VALUE_MASK) {
    val >>= 5;
    if (val > INTEL_LTR_VALUE_MASK)
    val = INTEL_LTR_VALUE_MASK;
    ltr |= INTEL_LTR_SCALE_32US | val;
    } else {
    ltr |= INTEL_LTR_SCALE_1US | val;
    }
    }
    if (ltr == host.active_ltr)
    return;
    writel(ltr, host.priv + INTEL_ACTIVELTR);
    writel(ltr, host.priv + INTEL_IDLELTR);
// Cache the values into intel_host structure
    intel_cache_ltr(host);
    }
#[no_mangle]
unsafe extern "C" fn intel_ltr_expose(dev: *mut device) {
    static void intel_ltr_expose(struct device *dev)
    {
    dev.power.set_latency_tolerance = intel_ltr_set;
    dev_pm_qos_expose_latency_tolerance(dev);
    }
#[no_mangle]
unsafe extern "C" fn intel_ltr_hide(dev: *mut device) {
    static void intel_ltr_hide(struct device *dev)
    {
    dev_pm_qos_hide_latency_tolerance(dev);
    dev.power.set_latency_tolerance = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn intel_add_debugfs(hci: *mut mipi_i3c_hci_pci) {
    static void intel_add_debugfs(struct mipi_i3c_hci_pci *hci)
    {
    struct dentry *dir = debugfs_create_dir(dev_name(&hci.pci.dev), core::ptr::null_mut());
    struct intel_host *host = hci.private;
    intel_cache_ltr(host);
    host.debugfs_root = dir;
    debugfs_create_x32("active_ltr", 0444, dir, &host.active_ltr);
    debugfs_create_x32("idle_ltr", 0444, dir, &host.idle_ltr);
    }
#[no_mangle]
unsafe extern "C" fn intel_remove_debugfs(hci: *mut mipi_i3c_hci_pci) {
    static void intel_remove_debugfs(struct mipi_i3c_hci_pci *hci)
    {
    struct intel_host *host = hci.private;
    debugfs_remove_recursive(host.debugfs_root);
    }
#[no_mangle]
unsafe extern "C" fn intel_reset(priv: *mut void __iomem) {
    static void intel_reset(void __iomem *priv)
    {
    u32 reg;
// Assert reset, wait for completion and release reset
    writel(0, priv + INTEL_RESETS);
    readl_poll_timeout(priv + INTEL_RESETS, reg,
    reg & INTEL_RESETS_RESET_DONE, 0,
    INTEL_RESETS_TIMEOUT_US);
    writel(INTEL_RESETS_RESET, priv + INTEL_RESETS);
    }
#[no_mangle]
unsafe extern "C" fn intel_i3c_init(hci: *mut mipi_i3c_hci_pci) -> c_int {
    static int intel_i3c_init(struct mipi_i3c_hci_pci *hci)
    {
    struct intel_host *host = devm_kzalloc(&hci.pci.dev, sizeof(*host), GFP_KERNEL);
    void __iomem *priv = hci.base + INTEL_PRIV_OFFSET;
    if (!host)
    return -ENOMEM;
    dma_set_mask_and_coherent(&hci.pci.dev, DMA_BIT_MASK(64));
    hci.pci.d3cold_delay = 0;
    hci.pci.d3hot_delay = 0;
    hci.private = host;
    host.priv = priv;
    intel_reset(priv);
    intel_ltr_expose(&hci.pci.dev);
    intel_add_debugfs(hci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_i3c_exit(hci: *mut mipi_i3c_hci_pci) {
    static void intel_i3c_exit(struct mipi_i3c_hci_pci *hci)
    {
    intel_remove_debugfs(hci);
    intel_ltr_hide(&hci.pci.dev);
    }
    static const struct mipi_i3c_hci_pci_info intel_mi_1_info = {
    .init = intel_i3c_init,
    .exit = intel_i3c_exit,
    .name = "intel-lpss-i3c",
    .id = {0, 1},
    .instance_offset = {0, 0x400},
    .instance_count = 2,
    .control_instance_pm = true,
    };
    static const struct mipi_i3c_hci_pci_info intel_mi_2_info = {
    .init = intel_i3c_init,
    .exit = intel_i3c_exit,
    .name = "intel-lpss-i3c",
    .id = {2, 3},
    .instance_offset = {0, 0x400},
    .instance_count = 2,
    .control_instance_pm = true,
    };
    static const struct mipi_i3c_hci_pci_info intel_si_2_info = {
    .init = intel_i3c_init,
    .exit = intel_i3c_exit,
    .name = "intel-lpss-i3c",
    .id = {2},
    .instance_offset = {0},
    .instance_count = 1,
    .control_instance_pm = true,
    };
    static const struct mipi_i3c_hci_pci_info amd_pt_info = {
    .name = "amd-pt-i3c-hci",
    .id = {0},
    .instance_offset = {0},
    .instance_count = 1,
    };
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_find_instance(hci: *mut mipi_i3c_hci_pci, dev: *mut device) -> c_int {
    static int mipi_i3c_hci_pci_find_instance(struct mipi_i3c_hci_pci *hci, struct device *dev)
    {
    for (int i = 0; i < INST_MAX; i++) {
    if (!hci.instance[i].dev)
    hci.instance[i].dev = dev;
    if (hci.instance[i].dev == dev)
    return i;
    }
    return -1;
    }
pub const HC_CONTROL: c_uint = 0x04;

#[no_mangle]
unsafe extern "C" fn __mipi_i3c_hci_pci_is_operational(dev: *mut device) -> bool {
    static bool __mipi_i3c_hci_pci_is_operational(struct device *dev)
    {
    const struct mipi_i3c_hci_platform_data *pdata = dev.platform_data;
    let mut hc_control: u32 = readl(pdata.base_regs + HC_CONTROL);
    return hc_control & HC_CONTROL_BUS_ENABLE;
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_is_operational(dev: *mut device, update: bool) -> bool {
    static bool mipi_i3c_hci_pci_is_operational(struct device *dev, bool update)
    {
    struct mipi_i3c_hci_pci *hci = dev_get_drvdata(dev.parent);
    let mut pos: c_int = mipi_i3c_hci_pci_find_instance(hci, dev);
    if (pos < 0) {
    dev_err(dev, "%s: I3C instance not found\n", __func__);
    return false;
    }
    if (update)
    hci.instance[pos].operational = __mipi_i3c_hci_pci_is_operational(dev);
    return hci.instance[pos].operational;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_i3c_hci_pci_pm_data {
    pub dev: [*mut device; INST_MAX],
    pub dev_cnt: c_int,
    pub can_wakeup: bool,
    pub may_wakeup: bool,
}

#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_is_mfd(dev: *mut device) -> bool {
    static bool mipi_i3c_hci_pci_is_mfd(struct device *dev)
    {
    return dev_is_platform(dev) && mfd_get_cell(to_platform_device(dev));
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_any_wakeup_enabled(dev: *mut device) -> bool {
    static bool mipi_i3c_hci_pci_any_wakeup_enabled(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    return i3c_master_has_wakeup_enabled_devs(&hci.master);
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_suspend_instance(dev: *mut device, data: *mut c_void) -> c_int {
    static int mipi_i3c_hci_pci_suspend_instance(struct device *dev, void *data)
    {
    struct mipi_i3c_hci_pci_pm_data *pm_data = data;
    int ret;
    if (!mipi_i3c_hci_pci_is_mfd(dev) ||
    !mipi_i3c_hci_pci_is_operational(dev, true))
    return 0;
    ret = i3c_hci_rpm_suspend(dev);
    if (ret)
    return ret;
    pm_data.dev[pm_data.dev_cnt++] = dev;
    if (pm_data.can_wakeup && mipi_i3c_hci_pci_any_wakeup_enabled(dev))
    pm_data.may_wakeup = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_resume_instance(dev: *mut device, data: *mut c_void) -> c_int {
    static int mipi_i3c_hci_pci_resume_instance(struct device *dev, void *data)
    {
    struct mipi_i3c_hci_pci_pm_data *pm_data = data;
    int ret;
    if (!mipi_i3c_hci_pci_is_mfd(dev) ||
    !mipi_i3c_hci_pci_is_operational(dev, false))
    return 0;
    ret = i3c_hci_rpm_resume(dev);
    if (ret)
    return ret;
    pm_data.dev[pm_data.dev_cnt++] = dev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_suspend(dev: *mut device) -> c_int {
    static int mipi_i3c_hci_pci_suspend(struct device *dev)
    {
    struct mipi_i3c_hci_pci *hci = dev_get_drvdata(dev);
    let mut pm_data: mipi_i3c_hci_pci_pm_data = {};
    int ret;
    if (!hci.info.control_instance_pm)
    return 0;
    pm_data.can_wakeup = device_can_wakeup(dev);
    ret = device_for_each_child_reverse(dev, &pm_data, mipi_i3c_hci_pci_suspend_instance);
    if (ret) {
    for (int i = 0; i < pm_data.dev_cnt; i++)
    i3c_hci_rpm_resume(pm_data.dev[i]);
    return ret;
    }
    if (device_may_wakeup(dev) != pm_data.may_wakeup)
    device_set_wakeup_enable(dev, pm_data.may_wakeup);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_resume(dev: *mut device) -> c_int {
    static int mipi_i3c_hci_pci_resume(struct device *dev)
    {
    struct mipi_i3c_hci_pci *hci = dev_get_drvdata(dev);
    let mut pm_data: mipi_i3c_hci_pci_pm_data = {};
    int ret;
    if (!hci.info.control_instance_pm)
    return 0;
    ret = device_for_each_child(dev, &pm_data, mipi_i3c_hci_pci_resume_instance);
    if (ret)
    for (int i = 0; i < pm_data.dev_cnt; i++)
    i3c_hci_rpm_suspend(pm_data.dev[i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_rpm_allow(dev: *mut device) {
    static void mipi_i3c_hci_pci_rpm_allow(struct device *dev)
    {
    pm_runtime_put(dev);
    pm_runtime_allow(dev);
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_rpm_forbid(dev: *mut device) {
    static void mipi_i3c_hci_pci_rpm_forbid(struct device *dev)
    {
    pm_runtime_forbid(dev);
    pm_runtime_get_sync(dev);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_i3c_hci_pci_cell_data {
    pub pdata: mipi_i3c_hci_platform_data,
    pub res: resource,
}

    static void mipi_i3c_hci_pci_setup_cell(struct mipi_i3c_hci_pci *hci, int idx,
    struct mipi_i3c_hci_pci_cell_data *data,
    struct mfd_cell *cell)
    {
    data.pdata.base_regs = hci.base + hci.info.instance_offset[idx];
    data.res = DEFINE_RES_IRQ(0);
    cell.name = hci.info.name;
    cell.id = hci.info.id[idx];
    cell.platform_data = &data.pdata;
    cell.pdata_size = sizeof(data.pdata);
    cell.num_resources = 1;
    cell.resources = &data.res;
    }

#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_add_instances(hci: *mut mipi_i3c_hci_pci) -> c_int {
    static int mipi_i3c_hci_pci_add_instances(struct mipi_i3c_hci_pci *hci)
    {
    struct mipi_i3c_hci_pci_cell_data *data __free(kfree) = mipi_i3c_hci_pci_alloc(hci, data);
    struct mfd_cell *cells __free(kfree) = mipi_i3c_hci_pci_alloc(hci, cells);
    let mut irq: c_int = pci_irq_vector(hci.pci, 0);
    let mut nr: c_int = hci.info.instance_count;
    if (!cells || !data)
    return -ENOMEM;
    for (int i = 0; i < nr; i++)
    mipi_i3c_hci_pci_setup_cell(hci, i, data + i, cells + i);
    return mfd_add_devices(&hci.pci.dev, 0, cells, nr, core::ptr::null_mut(), irq, core::ptr::null_mut());
    }
    static int mipi_i3c_hci_pci_probe(struct pci_dev *pci,
    const struct pci_device_id *id)
    {
    struct mipi_i3c_hci_pci *hci;
    int ret;
    hci = devm_kzalloc(&pci.dev, sizeof(*hci), GFP_KERNEL);
    if (!hci)
    return -ENOMEM;
    hci.pci = pci;
    ret = pcim_enable_device(pci);
    if (ret)
    return ret;
    pci_set_master(pci);
    hci.base = pcim_iomap_region(pci, 0, pci_name(pci));
    if (IS_ERR(hci.base))
    return PTR_ERR(hci.base);
    ret = pci_alloc_irq_vectors(pci, 1, 1, PCI_IRQ_ALL_TYPES);
    if (ret < 0)
    return ret;
    hci.info = (const struct mipi_i3c_hci_pci_info *)id.driver_data;
    ret = hci.info.init ? hci.info.init(hci) : 0;
    if (ret)
    return ret;
    ret = mipi_i3c_hci_pci_add_instances(hci);
    if (ret)
    goto err_exit;
    pci_set_drvdata(pci, hci);
    mipi_i3c_hci_pci_rpm_allow(&pci.dev);
    return 0;
    err_exit:
    if (hci.info.exit)
    hci.info.exit(hci);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mipi_i3c_hci_pci_remove(pci: *mut pci_dev) {
    static void mipi_i3c_hci_pci_remove(struct pci_dev *pci)
    {
    struct mipi_i3c_hci_pci *hci = pci_get_drvdata(pci);
    if (hci.info.exit)
    hci.info.exit(hci);
    mipi_i3c_hci_pci_rpm_forbid(&pci.dev);
    mfd_remove_devices(&pci.dev);
    }
// PM ops must exist for PCI to put a device to a low power state
    static const struct dev_pm_ops mipi_i3c_hci_pci_pm_ops = {
    RUNTIME_PM_OPS(mipi_i3c_hci_pci_suspend, mipi_i3c_hci_pci_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(mipi_i3c_hci_pci_suspend, mipi_i3c_hci_pci_resume)
    };
    static const struct pci_device_id mipi_i3c_hci_pci_devices[] = {
// Wildcat Lake-U
    { PCI_VDEVICE(INTEL, 0x4d7c), .driver_data = (kernel_ulong_t)&intel_mi_1_info },
    { PCI_VDEVICE(INTEL, 0x4d6f), .driver_data = (kernel_ulong_t)&intel_si_2_info },
// Panther Lake-H
    { PCI_VDEVICE(INTEL, 0xe37c), .driver_data = (kernel_ulong_t)&intel_mi_1_info },
    { PCI_VDEVICE(INTEL, 0xe36f), .driver_data = (kernel_ulong_t)&intel_si_2_info },
// Panther Lake-P
    { PCI_VDEVICE(INTEL, 0xe47c), .driver_data = (kernel_ulong_t)&intel_mi_1_info },
    { PCI_VDEVICE(INTEL, 0xe46f), .driver_data = (kernel_ulong_t)&intel_si_2_info },
// Nova Lake-S
    { PCI_VDEVICE(INTEL, 0x6e2c), .driver_data = (kernel_ulong_t)&intel_mi_1_info },
    { PCI_VDEVICE(INTEL, 0x6e2d), .driver_data = (kernel_ulong_t)&intel_mi_2_info },
// Nova Lake-H
    { PCI_VDEVICE(INTEL, 0xd37c), .driver_data = (kernel_ulong_t)&intel_mi_1_info },
    { PCI_VDEVICE(INTEL, 0xd36f), .driver_data = (kernel_ulong_t)&intel_mi_2_info },
// AMD_PT
    { PCI_VDEVICE(AMD, 0x444c), .driver_data = (kernel_ulong_t)&amd_pt_info },
    { }
    };
    MODULE_DEVICE_TABLE(pci, mipi_i3c_hci_pci_devices);
    static struct pci_driver mipi_i3c_hci_pci_driver = {
    .name = "mipi_i3c_hci_pci",
    .id_table = mipi_i3c_hci_pci_devices,
    .probe = mipi_i3c_hci_pci_probe,
    .remove = mipi_i3c_hci_pci_remove,
    .driver = {
    .pm = pm_ptr(&mipi_i3c_hci_pci_pm_ops)
    },
    };
    module_pci_driver(mipi_i3c_hci_pci_driver);
    MODULE_AUTHOR("Jarkko Nikula <jarkko.nikula@intel.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MIPI I3C HCI driver on PCI bus");
