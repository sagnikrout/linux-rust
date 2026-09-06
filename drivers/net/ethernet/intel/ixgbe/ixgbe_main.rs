//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_main.c
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
// Copyright(c) 1999 - 2024 Intel Corporation.

    char ixgbe_driver_name[] = "ixgbe";
    static const char ixgbe_driver_string[] =
    "Intel(R) 10 Gigabit PCI Express Network Driver";

    char ixgbe_default_device_descr[] =
    "Intel(R) 10 Gigabit Network Connection";

    static char ixgbe_default_device_descr[] =
    "Intel(R) 10 Gigabit Network Connection";

    static const char ixgbe_copyright[] =
    "Copyright (c) 1999-2016 Intel Corporation.";
    static const char ixgbe_overheat_msg[] = "Network adapter has been stopped because it has over heated. Restart the computer. If the problem persists, power off the system and replace the adapter";
    static const struct ixgbe_info *ixgbe_info_tbl[] = {
    [board_82598]		= &ixgbe_82598_info,
    [board_82599]		= &ixgbe_82599_info,
    [board_X540]		= &ixgbe_X540_info,
    [board_X550]		= &ixgbe_X550_info,
    [board_X550EM_x]	= &ixgbe_X550EM_x_info,
    [board_x550em_x_fw]	= &ixgbe_x550em_x_fw_info,
    [board_x550em_a]	= &ixgbe_x550em_a_info,
    [board_x550em_a_fw]	= &ixgbe_x550em_a_fw_info,
    [board_e610]		= &ixgbe_e610_info,
    };
// ixgbe_pci_tbl - PCI Device ID Table
//
// Wildcard entries (PCI_ANY_ID) should come last
// Last entry must be all 0s
//
// { Vendor ID, Device ID, SubVendor ID, SubDevice ID,
// Class, Class Mask, private data (not used) }
//
    static const struct pci_device_id ixgbe_pci_tbl[] = {
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598AF_DUAL_PORT), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598AF_SINGLE_PORT), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598AT), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598AT2), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598EB_CX4), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598_CX4_DUAL_PORT), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598_DA_DUAL_PORT), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598_SR_DUAL_PORT_EM), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598EB_XF_LR), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598EB_SFP_LOM), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82598_BX), .driver_data = board_82598 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_KX4), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_XAUI_LOM), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_KR), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_SFP), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_SFP_EM), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_KX4_MEZZ), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_CX4), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_BACKPLANE_FCOE), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_SFP_FCOE), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_T3_LOM), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_COMBO_BACKPLANE), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X540T), .driver_data = board_X540 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_SFP_SF2), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_LS), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_QSFP_SF_QP), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599EN_SFP), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_82599_SFP_SF_QP), .driver_data = board_82599 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X540T1), .driver_data = board_X540 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550T), .driver_data = board_X550 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550T1), .driver_data = board_X550 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_X_KX4), .driver_data = board_X550EM_x },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_X_XFI), .driver_data = board_X550EM_x },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_X_KR), .driver_data = board_X550EM_x },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_X_10G_T), .driver_data = board_X550EM_x },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_X_SFP), .driver_data = board_X550EM_x },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_X_1G_T), .driver_data = board_x550em_x_fw },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_KR), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_KR_L), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_SFP_N), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_SGMII), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_SGMII_L), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_10G_T), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_SFP), .driver_data = board_x550em_a },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_1G_T), .driver_data = board_x550em_a_fw },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_X550EM_A_1G_T_L), .driver_data = board_x550em_a_fw },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_E610_BACKPLANE), .driver_data = board_e610 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_E610_SFP), .driver_data = board_e610 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_E610_10G_T), .driver_data = board_e610 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_E610_2_5G_T), .driver_data = board_e610 },
    { PCI_VDEVICE(INTEL, IXGBE_DEV_ID_E610_SGMII), .driver_data = board_e610 },
// required last entry
    { }
    };
    MODULE_DEVICE_TABLE(pci, ixgbe_pci_tbl);

    static int ixgbe_notify_dca(struct notifier_block *, unsigned long event,
    void *p);
    static struct notifier_block dca_notifier = {
    .notifier_call = ixgbe_notify_dca,
    .next          = core::ptr::null_mut(),
    .priority      = 0
    };

    static unsigned int max_vfs;
    module_param(max_vfs, uint, 0);
    MODULE_PARM_DESC(max_vfs,
    "Maximum number of virtual functions to allocate per physical function - default is zero and maximum value is 63. (Deprecated)");

    static bool allow_unsupported_sfp;
    module_param(allow_unsupported_sfp, bool, 0444);
    MODULE_PARM_DESC(allow_unsupported_sfp,
    "Allow unsupported and untested SFP+ modules on 82599-based adapters");

    let mut debug: static int = -1;
    module_param(debug, int, 0);
    MODULE_PARM_DESC(debug, "Debug level (0=none,...,16=all)");
    MODULE_IMPORT_NS("LIBIE_FWLOG");
    MODULE_DESCRIPTION("Intel(R) 10 Gigabit PCI Express Network Driver");
    MODULE_LICENSE("GPL v2");
    DEFINE_STATIC_KEY_FALSE(ixgbe_xdp_locking_key);
    EXPORT_SYMBOL(ixgbe_xdp_locking_key);
    static struct workqueue_struct *ixgbe_wq;
    static bool ixgbe_check_cfg_remove(struct ixgbe_hw *hw, struct pci_dev *pdev);
    static void ixgbe_watchdog_link_is_down(struct ixgbe_adapter *);
    static void ixgbe_watchdog_link_is_up(struct ixgbe_adapter *);
    static void ixgbe_watchdog_update_link(struct ixgbe_adapter *);
    static const struct net_device_ops ixgbe_netdev_ops;
#[no_mangle]
unsafe extern "C" fn netif_is_ixgbe(dev: *mut net_device) -> bool {
    static bool netif_is_ixgbe(struct net_device *dev)
    {
    return dev && (dev.netdev_ops == &ixgbe_netdev_ops);
    }
    static int ixgbe_read_pci_cfg_word_parent(struct ixgbe_adapter *adapter,
    u32 reg, u16 *value)
    {
    struct pci_dev *parent_dev;
    struct pci_bus *parent_bus;
    parent_bus = adapter.pdev.bus.parent;
    if (!parent_bus)
    return -1;
    parent_dev = parent_bus.self;
    if (!parent_dev)
    return -1;
    if (!pci_is_pcie(parent_dev))
    return -1;
    pcie_capability_read_word(parent_dev, reg, value);
    if (*value == IXGBE_FAILED_READ_CFG_WORD &&
    ixgbe_check_cfg_remove(&adapter.hw, parent_dev))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_get_parent_bus_info(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_get_parent_bus_info(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut link_status: u16 = 0;
    int err;
    hw.bus.type = ixgbe_bus_type_pci_express;
// Get the negotiated link width and speed from PCI config space of the
// parent, as this device is behind a switch
//
    err = ixgbe_read_pci_cfg_word_parent(adapter, 18, &link_status);
// assume caller will handle error case
    if (err)
    return err;
    hw.bus.width = ixgbe_convert_bus_width(link_status);
    hw.bus.speed = ixgbe_convert_bus_speed(link_status);
    return 0;
    }
//
// ixgbe_pcie_from_parent - Determine whether PCIe info should come from parent
// @hw: hw specific details
//
// This function is used by probe to determine whether a device's PCI-Express
// bandwidth details should be gathered from the parent bus instead of from the
// device. Used to ensure that various locations all have the correct device ID
// checks.
//
// Return: true if information should be collected from the parent bus, false
// otherwise
//
#[no_mangle]
unsafe extern "C" fn ixgbe_pcie_from_parent(hw: *mut ixgbe_hw) -> bool {
    static bool ixgbe_pcie_from_parent(struct ixgbe_hw *hw)
    {
    switch (hw.device_id) {
    case IXGBE_DEV_ID_82599_SFP_SF_QP:
    case IXGBE_DEV_ID_82599_QSFP_SF_QP:
    return true;
    default:
    return false;
    }
    }
    static void ixgbe_check_minimum_link(struct ixgbe_adapter *adapter,
    int expected_gts)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct pci_dev *pdev;
// Some devices are not connected over PCIe and thus do not negotiate
// speed. These devices do not have valid bus info, and thus any report
// we generate may not be correct.
//
    if (hw.bus.type == ixgbe_bus_type_internal)
    return;
// determine whether to use the parent device
    if (ixgbe_pcie_from_parent(&adapter.hw))
    pdev = adapter.pdev.bus.parent.self;
    else
    pdev = adapter.pdev;
    pcie_print_link_status(pdev);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_service_event_schedule(adapter: *mut ixgbe_adapter) {
    static void ixgbe_service_event_schedule(struct ixgbe_adapter *adapter)
    {
    if (!test_bit(__IXGBE_DOWN, &adapter.state) &&
    !test_bit(__IXGBE_REMOVING, &adapter.state) &&
    !test_and_set_bit(__IXGBE_SERVICE_SCHED, &adapter.state))
    queue_work(ixgbe_wq, &adapter.service_task);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_remove_adapter(hw: *mut ixgbe_hw) {
    static void ixgbe_remove_adapter(struct ixgbe_hw *hw)
    {
    struct ixgbe_adapter *adapter = hw.back;
    if (!hw.hw_addr)
    return;
    hw.hw_addr = core::ptr::null_mut();
    e_dev_err("Adapter removed\n");
    if (test_bit(__IXGBE_SERVICE_INITED, &adapter.state))
    ixgbe_service_event_schedule(adapter);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_remove(hw: *mut ixgbe_hw, reg: u32) -> u32 {
    static u32 ixgbe_check_remove(struct ixgbe_hw *hw, u32 reg)
    {
    u8 __iomem *reg_addr;
    u32 value;
    int i;
    reg_addr = READ_ONCE(hw.hw_addr);
    if (ixgbe_removed(reg_addr))
    return IXGBE_FAILED_READ_REG;
// Register read of 0xFFFFFFF can indicate the adapter has been removed,
// so perform several status register reads to determine if the adapter
// has been removed.
//
    for (i = 0; i < IXGBE_FAILED_READ_RETRIES; i++) {
    value = readl(reg_addr + IXGBE_STATUS);
    if (value != IXGBE_FAILED_READ_REG)
    break;
    mdelay(3);
    }
    if (value == IXGBE_FAILED_READ_REG)
    ixgbe_remove_adapter(hw);
    else
    value = readl(reg_addr + reg);
    return value;
    }
//
// ixgbe_read_reg - Read from device register
// @hw: hw specific details
// @reg: offset of register to read
//
// Returns : value read or IXGBE_FAILED_READ_REG if removed
//
// This function is used to read device registers. It checks for device
// removal by confirming any read that returns all ones by checking the
// status register value for all ones. This function avoids reading from
// the hardware if a removal was previously detected in which case it
// returns IXGBE_FAILED_READ_REG (all ones).
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_read_reg(hw: *mut ixgbe_hw, reg: u32) -> u32 {
    u32 ixgbe_read_reg(struct ixgbe_hw *hw, u32 reg)
    {
    u8 __iomem *reg_addr = READ_ONCE(hw.hw_addr);
    u32 value;
    if (ixgbe_removed(reg_addr))
    return IXGBE_FAILED_READ_REG;
    if (unlikely(hw.phy.nw_mng_if_sel &
    IXGBE_NW_MNG_IF_SEL_SGMII_ENABLE)) {
    struct ixgbe_adapter *adapter;
    int i;
    for (i = 0; i < 200; ++i) {
    value = readl(reg_addr + IXGBE_MAC_SGMII_BUSY);
    if (likely(!value))
    goto writes_completed;
    if (value == IXGBE_FAILED_READ_REG) {
    ixgbe_remove_adapter(hw);
    return IXGBE_FAILED_READ_REG;
    }
    udelay(5);
    }
    adapter = hw.back;
    e_warn(hw, "register writes incomplete %08x\n", value);
    }
    writes_completed:
    value = readl(reg_addr + reg);
    if (unlikely(value == IXGBE_FAILED_READ_REG))
    value = ixgbe_check_remove(hw, reg);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_cfg_remove(hw: *mut ixgbe_hw, pdev: *mut pci_dev) -> bool {
    static bool ixgbe_check_cfg_remove(struct ixgbe_hw *hw, struct pci_dev *pdev)
    {
    u16 value;
    pci_read_config_word(pdev, PCI_VENDOR_ID, &value);
    if (value == IXGBE_FAILED_READ_CFG_WORD) {
    ixgbe_remove_adapter(hw);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_read_pci_cfg_word(hw: *mut ixgbe_hw, reg: u32) -> u16 {
    u16 ixgbe_read_pci_cfg_word(struct ixgbe_hw *hw, u32 reg)
    {
    struct ixgbe_adapter *adapter = hw.back;
    u16 value;
    if (ixgbe_removed(hw.hw_addr))
    return IXGBE_FAILED_READ_CFG_WORD;
    pci_read_config_word(adapter.pdev, reg, &value);
    if (value == IXGBE_FAILED_READ_CFG_WORD &&
    ixgbe_check_cfg_remove(hw, adapter.pdev))
    return IXGBE_FAILED_READ_CFG_WORD;
    return value;
    }

#[no_mangle]
unsafe extern "C" fn ixgbe_read_pci_cfg_dword(hw: *mut ixgbe_hw, reg: u32) -> u32 {
    static u32 ixgbe_read_pci_cfg_dword(struct ixgbe_hw *hw, u32 reg)
    {
    struct ixgbe_adapter *adapter = hw.back;
    u32 value;
    if (ixgbe_removed(hw.hw_addr))
    return IXGBE_FAILED_READ_CFG_DWORD;
    pci_read_config_dword(adapter.pdev, reg, &value);
    if (value == IXGBE_FAILED_READ_CFG_DWORD &&
    ixgbe_check_cfg_remove(hw, adapter.pdev))
    return IXGBE_FAILED_READ_CFG_DWORD;
    return value;
    }

#[no_mangle]
pub unsafe extern "C" fn ixgbe_write_pci_cfg_word(hw: *mut ixgbe_hw, reg: u32, value: u16) {
    void ixgbe_write_pci_cfg_word(struct ixgbe_hw *hw, u32 reg, u16 value)
    {
    struct ixgbe_adapter *adapter = hw.back;
    if (ixgbe_removed(hw.hw_addr))
    return;
    pci_write_config_word(adapter.pdev, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_service_event_complete(adapter: *mut ixgbe_adapter) {
    static void ixgbe_service_event_complete(struct ixgbe_adapter *adapter)
    {
    BUG_ON(!test_bit(__IXGBE_SERVICE_SCHED, &adapter.state));
// flush memory to make sure state is correct before next watchdog
    smp_mb__before_atomic();
    clear_bit(__IXGBE_SERVICE_SCHED, &adapter.state);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_reg_info {
    pub ofs: u32,
    pub name: *mut c_char,
}

    static const struct ixgbe_reg_info ixgbe_reg_info_tbl[] = {
// General Registers
    {IXGBE_CTRL, "CTRL"},
    {IXGBE_STATUS, "STATUS"},
    {IXGBE_CTRL_EXT, "CTRL_EXT"},
// Interrupt Registers
    {IXGBE_EICR, "EICR"},
// RX Registers
    {IXGBE_SRRCTL(0), "SRRCTL"},
    {IXGBE_DCA_RXCTRL(0), "DRXCTL"},
    {IXGBE_RDLEN(0), "RDLEN"},
    {IXGBE_RDH(0), "RDH"},
    {IXGBE_RDT(0), "RDT"},
    {IXGBE_RXDCTL(0), "RXDCTL"},
    {IXGBE_RDBAL(0), "RDBAL"},
    {IXGBE_RDBAH(0), "RDBAH"},
// TX Registers
    {IXGBE_TDBAL(0), "TDBAL"},
    {IXGBE_TDBAH(0), "TDBAH"},
    {IXGBE_TDLEN(0), "TDLEN"},
    {IXGBE_TDH(0), "TDH"},
    {IXGBE_TDT(0), "TDT"},
    {IXGBE_TXDCTL(0), "TXDCTL"},
// List Terminator
    { .name = core::ptr::null_mut() }
    };
//
// ixgbe_regdump - register printout routine
//
#[no_mangle]
unsafe extern "C" fn ixgbe_regdump(hw: *mut ixgbe_hw, reginfo: *mut ixgbe_reg_info) {
    static void ixgbe_regdump(struct ixgbe_hw *hw, struct ixgbe_reg_info *reginfo)
    {
    int i;
    char rname[16];
    u32 regs[64];
    switch (reginfo.ofs) {
    case IXGBE_SRRCTL(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_SRRCTL(i));
    break;
    case IXGBE_DCA_RXCTRL(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_DCA_RXCTRL(i));
    break;
    case IXGBE_RDLEN(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_RDLEN(i));
    break;
    case IXGBE_RDH(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_RDH(i));
    break;
    case IXGBE_RDT(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_RDT(i));
    break;
    case IXGBE_RXDCTL(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_RXDCTL(i));
    break;
    case IXGBE_RDBAL(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_RDBAL(i));
    break;
    case IXGBE_RDBAH(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_RDBAH(i));
    break;
    case IXGBE_TDBAL(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_TDBAL(i));
    break;
    case IXGBE_TDBAH(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_TDBAH(i));
    break;
    case IXGBE_TDLEN(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_TDLEN(i));
    break;
    case IXGBE_TDH(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_TDH(i));
    break;
    case IXGBE_TDT(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_TDT(i));
    break;
    case IXGBE_TXDCTL(0):
    for (i = 0; i < 64; i++)
    regs[i] = IXGBE_READ_REG(hw, IXGBE_TXDCTL(i));
    break;
    default:
    pr_info("%-15s %08x\n",
    reginfo.name, IXGBE_READ_REG(hw, reginfo.ofs));
    return;
    }
    i = 0;
    while (i < 64) {
    int j;
    char buf[9 * 8 + 1];
    char *p = buf;
    snprintf(rname, 16, "%s[%d-%d]", reginfo.name, i, i + 7);
    for (j = 0; j < 8; j++)
    p += sprintf(p, " %08x", regs[i++]);
    pr_err("%-15s%s\n", rname, buf);
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_print_buffer(ring: *mut ixgbe_ring, n: c_int) {
    static void ixgbe_print_buffer(struct ixgbe_ring *ring, int n)
    {
    struct ixgbe_tx_buffer *tx_buffer;
    tx_buffer = &ring.tx_buffer_info[ring.next_to_clean];
    pr_info(" %5d %5X %5X %016llX %08X %p %016llX\n",
    n, ring.next_to_use, ring.next_to_clean,
    (u64)dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
    tx_buffer.next_to_watch,
    (u64)tx_buffer.time_stamp);
    }
//
// ixgbe_dump - Print registers, tx-rings and rx-rings
//
#[no_mangle]
unsafe extern "C" fn ixgbe_dump(adapter: *mut ixgbe_adapter) {
    static void ixgbe_dump(struct ixgbe_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct ixgbe_hw *hw = &adapter.hw;
    struct ixgbe_reg_info *reginfo;
    let mut n: c_int = 0;
    struct ixgbe_ring *ring;
    struct ixgbe_tx_buffer *tx_buffer;
    union ixgbe_adv_tx_desc *tx_desc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_u0 {
    pub rx_ring: *mut ixgbe_ring,
    pub rx_desc: *mut union ixgbe_adv_rx_desc,
    pub rx_buffer_info: *mut ixgbe_rx_buffer,
    pub 0: int i =,
    if (!netif_msg_hw(adapter))
// Print netdevice Info
    if (netdev) {
    pub Info\n"): dev_info(&adapter->pdev->dev, "Net device,
    pr_info("Device Name     state            "
    pr_info("%-15s %016lX %016lX\n",
    netdev.name,
    netdev.state,
    }
// Print Registers
    pub Dump\n"): dev_info(&adapter->pdev->dev, "Register,
    pub Value\n"): pr_info(" Register Name,
    pub )ixgbe_reg_info_tbl: *mut for (reginfo = (struct ixgbe_reg_info,
    pub {: reginfo->name; reginfo++),
    pub reginfo): ixgbe_regdump(hw,,
    }
// Print TX Ring Summary
    if (!netdev || !netif_running(netdev))
    pub Summary\n"): dev_info(&adapter->pdev->dev, "TX Rings,
    pr_info(" %s     %s              %s        %s\n",
    "Queue [NTU] [NTC] [bi(ntc).dma  ]",
    pub "timestamp"): "leng", "ntw",,
    pub {: for (n = 0; n < adapter->num_tx_queues; n++),
    pub adapter->tx_ring[n]: ring =,
    pub n): ixgbe_print_buffer(ring,,
    }
    pub {: for (n = 0; n < adapter->num_xdp_queues; n++),
    pub adapter->xdp_ring[n]: ring =,
    pub n): ixgbe_print_buffer(ring,,
    }
// Print TX Rings
    if (!netif_msg_tx_done(adapter))
    pub rx_ring_summary: goto,
    pub Dump\n"): dev_info(&adapter->pdev->dev, "TX Rings,
// Transmit Descriptor Formats
//
// 82598 Advanced Transmit Descriptor
// +--------------------------------------------------------------+
// 0 |         Buffer Address [63:0]                                |
// +--------------------------------------------------------------+
// 8 |  PAYLEN  | POPTS  | IDX | STA | DCMD  |DTYP |  RSV |  DTALEN |
// +--------------------------------------------------------------+
// 63       46 45    40 39 36 35 32 31   24 23 20 19              0
//
// 82598 Advanced Transmit Descriptor (Write-Back Format)
// +--------------------------------------------------------------+
// 0 |                          RSV [63:0]                          |
// +--------------------------------------------------------------+
// 8 |            RSV           |  STA  |          NXTSEQ           |
// +--------------------------------------------------------------+
// 63                       36 35   32 31                         0
//
// 82599+ Advanced Transmit Descriptor
// +--------------------------------------------------------------+
// 0 |         Buffer Address [63:0]                                |
// +--------------------------------------------------------------+
// 8 |PAYLEN  |POPTS|CC|IDX  |STA  |DCMD  |DTYP |MAC  |RSV  |DTALEN |
// +--------------------------------------------------------------+
// 63     46 45 40 39 38 36 35 32 31  24 23 20 19 18 17 16 15     0
//
// 82599+ Advanced Transmit Descriptor (Write-Back Format)
// +--------------------------------------------------------------+
// 0 |                          RSV [63:0]                          |
// +--------------------------------------------------------------+
// 8 |            RSV           |  STA  |           RSV             |
// +--------------------------------------------------------------+
// 63                       36 35   32 31                         0
//
    pub {: for (n = 0; n < adapter->num_tx_queues; n++),
    pub adapter->tx_ring[n]: ring =,
    pub ring->queue_index): pr_info("TX QUEUE INDEX = %d\n",,
    pr_info("%s%s    %s              %s        %s          %s\n",
    "T [desc]     [address 63:0  ] ",
    "[PlPOIdStDDt Ln] [bi.dma       ] ",
    pub "bi->skb"): "leng", "ntw", "timestamp",,
    pub {: for (i = 0; ring->desc && (i < ring->count); i++),
    pub i): tx_desc = IXGBE_TX_DESC(ring,,
    pub &ring->tx_buffer_info[i]: tx_buffer =,
    pub )tx_desc: *mut u0 = (struct my_u0,
    if (dma_unmap_len(tx_buffer, len) > 0) {
    pub ring_desc: *const c_char,
    if (i == ring.next_to_use &&
    i == ring.next_to_clean)
    pub NTC/U": ring_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(ring->next_to_use: i ==) -> else {
    else if (i == ring.next_to_use)
    pub NTU": ring_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(ring->next_to_clean: i ==) -> else {
    else if (i == ring.next_to_clean)
    pub NTC": ring_desc = ",
    else
    pub "": ring_desc =,
    pr_info("T [0x%03X]    %016llX %016llX %016llX %08X %p %016llX %p%s",
    i,
    le64_to_cpu(( __le64)u0.a),
    le64_to_cpu(( __le64)u0.b),
    (u64)dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
    tx_buffer.next_to_watch,
    (u64)tx_buffer.time_stamp,
    tx_buffer.skb,
    if (netif_msg_pktdata(adapter) &&
    tx_buffer.skb)
    print_hex_dump(KERN_INFO, "",
    DUMP_PREFIX_ADDRESS, 16, 1,
    tx_buffer.skb.data,
    dma_unmap_len(tx_buffer, len),
    }
    }
    }
// Print RX Rings Summary
    rx_ring_summary:
    pub Summary\n"): dev_info(&adapter->pdev->dev, "RX Rings,
    pub [NTC]\n"): pr_info("Queue [NTU],
    pub {: for (n = 0; n < adapter->num_rx_queues; n++),
    pub adapter->rx_ring[n]: rx_ring =,
    pr_info("%5d %5X %5X\n",
    pub rx_ring->next_to_clean): n, rx_ring->next_to_use,,
    }
// Print RX Rings
    if (!netif_msg_rx_status(adapter))
    pub Dump\n"): dev_info(&adapter->pdev->dev, "RX Rings,
// Receive Descriptor Formats
//
// 82598 Advanced Receive Descriptor (Read) Format
// 63                                           1        0
// +-----------------------------------------------------+
// 0 |       Packet Buffer Address [63:1]           |A0/NSE|
// +----------------------------------------------+------+
// 8 |       Header Buffer Address [63:1]           |  DD  |
// +-----------------------------------------------------+
//
// 82598 Advanced Receive Descriptor (Write-Back) Format
//
// 63       48 47    32 31  30      21 20 16 15   4 3     0
// +------------------------------------------------------+
// 0 |       RSS Hash /  |SPH| HDR_LEN  | RSV |Packet|  RSS |
// | Packet   | IP     |   |          |     | Type | Type |
// | Checksum | Ident  |   |          |     |      |      |
// +------------------------------------------------------+
// 8 | VLAN Tag | Length | Extended Error | Extended Status |
// +------------------------------------------------------+
// 63       48 47    32 31            20 19               0
//
// 82599+ Advanced Receive Descriptor (Read) Format
// 63                                           1        0
// +-----------------------------------------------------+
// 0 |       Packet Buffer Address [63:1]           |A0/NSE|
// +----------------------------------------------+------+
// 8 |       Header Buffer Address [63:1]           |  DD  |
// +-----------------------------------------------------+
//
// 82599+ Advanced Receive Descriptor (Write-Back) Format
//
// 63       48 47    32 31  30      21 20 17 16   4 3     0
// +------------------------------------------------------+
// 0 |RSS / Frag Checksum|SPH| HDR_LEN  |RSC- |Packet|  RSS |
// |/ RTT / PCoE_PARAM |   |          | CNT | Type | Type |
// |/ Flow Dir Flt ID  |   |          |     |      |      |
// +------------------------------------------------------+
// 8 | VLAN Tag | Length |Extended Error| Xtnd Status/NEXTP |
// +------------------------------------------------------+
// 63       48 47    32 31          20 19                 0
//
    pub {: for (n = 0; n < adapter->num_rx_queues; n++),
    pub adapter->rx_ring[n]: rx_ring =,
    pub rx_ring->queue_index): pr_info("RX QUEUE INDEX = %d\n",,
    pr_info("%s%s%s\n",
    "R  [desc]      [ PktBuf     A0] ",
    "[  HeadBuf   DD] [bi.dma       ] [bi.skb       ] ",
    pub format"): "<-- Adv Rx Read,
    pr_info("%s%s%s\n",
    "RWB[desc]      [PcsmIpSHl PtRs] ",
    "[vl er S cks ln] ---------------- [bi.skb       ] ",
    pub format"): "<-- Adv Rx Write-Back,
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub ring_desc: *const c_char,
    if (i == rx_ring.next_to_use)
    pub NTU": ring_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(rx_ring->next_to_clean: i ==) -> else {
    else if (i == rx_ring.next_to_clean)
    pub NTC": ring_desc = ",
    else
    pub "": ring_desc =,
    pub &rx_ring->rx_buffer_info[i]: rx_buffer_info =,
    pub i): rx_desc = IXGBE_RX_DESC(rx_ring,,
    pub )rx_desc: *mut u0 = (struct my_u0,
    if (rx_desc.wb.upper.length) {
// Descriptor Done
    pr_info("RWB[0x%03X]     %016llX %016llX ---------------- %p%s\n",
    i,
    le64_to_cpu(( __le64)u0.a),
    le64_to_cpu(( __le64)u0.b),
    rx_buffer_info.skb,
    } else {
    pr_info("R  [0x%03X]     %016llX %016llX %016llX %p%s\n",
    i,
    le64_to_cpu(( __le64)u0.a),
    le64_to_cpu(( __le64)u0.b),
    (u64)rx_buffer_info.dma,
    rx_buffer_info.skb,
    if (netif_msg_pktdata(adapter) &&
    rx_buffer_info.dma) {
    print_hex_dump(KERN_INFO, "",
    DUMP_PREFIX_ADDRESS, 16, 1,
    page_address(rx_buffer_info.page) +
    rx_buffer_info.page_offset,
    pub true): ixgbe_rx_bufsz(rx_ring),,
    }
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_release_hw_control(adapter: *mut ixgbe_adapter) {
    static void ixgbe_release_hw_control(struct ixgbe_adapter *adapter)
    {
    pub ctrl_ext: u32,
// Let firmware take over control of h/w
    pub IXGBE_CTRL_EXT): ctrl_ext = IXGBE_READ_REG(&adapter->hw,,
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_CTRL_EXT,
    pub ~IXGBE_CTRL_EXT_DRV_LOAD): ctrl_ext &,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_get_hw_control(adapter: *mut ixgbe_adapter) {
    static void ixgbe_get_hw_control(struct ixgbe_adapter *adapter)
    {
    pub ctrl_ext: u32,
// Let firmware know the driver has taken over
    pub IXGBE_CTRL_EXT): ctrl_ext = IXGBE_READ_REG(&adapter->hw,,
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_CTRL_EXT,
    pub IXGBE_CTRL_EXT_DRV_LOAD): ctrl_ext |,
    }
//
// ixgbe_set_ivar - set the IVAR registers, mapping interrupt causes to vectors
// @adapter: pointer to adapter struct
// @direction: 0 for Rx, 1 for Tx, -1 for other causes
// @queue: queue to map the corresponding interrupt to
// @msix_vector: the vector to map to the corresponding queue
//
    static void ixgbe_set_ivar(struct ixgbe_adapter *adapter, s8 direction,
    u8 queue, u8 msix_vector)
    {
    pub index: u32 ivar,,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub IXGBE_IVAR_ALLOC_VAL: msix_vector |=,
    if (direction == -1)
    pub 0: direction =,
    pub 0x1F: *mut *mut index = (((direction  64) + queue) >> 2) &,
    pub IXGBE_IVAR(index)): ivar = IXGBE_READ_REG(hw,,
    pub 0x3))): *mut *mut ivar &= ~(0xFF << (8  (queue &,
    pub 0x3))): *mut *mut ivar |= (msix_vector << (8  (queue &,
    pub ivar): IXGBE_WRITE_REG(hw, IXGBE_IVAR(index),,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    if (direction == -1) {
// other causes
    pub IXGBE_IVAR_ALLOC_VAL: msix_vector |=,
    pub 8): *mut *mut index = ((queue & 1),
    pub IXGBE_IVAR_MISC): ivar = IXGBE_READ_REG(&adapter->hw,,
    pub index): ivar &= ~(0xFF <<,
    pub index): ivar |= (msix_vector <<,
    pub ivar): IXGBE_WRITE_REG(&adapter->hw, IXGBE_IVAR_MISC,,
    } else {
// tx or rx causes
    pub IXGBE_IVAR_ALLOC_VAL: msix_vector |=,
    pub direction)): *mut *mut *mut index = ((16  (queue & 1)) + (8,
    pub 1)): ivar = IXGBE_READ_REG(hw, IXGBE_IVAR(queue >>,
    pub index): ivar &= ~(0xFF <<,
    pub index): ivar |= (msix_vector <<,
    pub ivar): IXGBE_WRITE_REG(hw, IXGBE_IVAR(queue >> 1),,
    }
    default:
    }
    }
    void ixgbe_irq_rearm_queues(struct ixgbe_adapter *adapter,
    u64 qmask)
    {
    pub mask: u32,
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub qmask): mask = (IXGBE_EIMS_RTX_QUEUE &,
    pub mask): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EICS,,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pub 0xFFFFFFFF): mask = (qmask &,
    pub mask): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EICS_EX(0),,
    pub 32): mask = (qmask >>,
    pub mask): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EICS_EX(1),,
    default:
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_update_xoff_rx_lfc(adapter: *mut ixgbe_adapter) {
    static void ixgbe_update_xoff_rx_lfc(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub &adapter->stats: *mut *mut ixgbe_hw_stats hwstats =,
    pub i: c_int,
    pub data: u32,
    if ((hw.fc.current_mode != ixgbe_fc_full) &&
    (hw.fc.current_mode != ixgbe_fc_rx_pause))
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub IXGBE_LXOFFRXC): data = IXGBE_READ_REG(hw,,
    default:
    pub IXGBE_LXOFFRXCNT): data = IXGBE_READ_REG(hw,,
    }
    pub data: hwstats->lxoffrxc +=,
// refill credits (no tx hang) if we received xoff
    if (!data)
    pub i++): for (i = 0; i < adapter->num_tx_queues;,
    clear_bit(__IXGBE_HANG_CHECK_ARMED,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_update_xoff_received(adapter: *mut ixgbe_adapter) {
    static void ixgbe_update_xoff_received(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub &adapter->stats: *mut *mut ixgbe_hw_stats hwstats =,
    pub {0}: u32 xoff[8] =,
    pub tc: u8,
    pub i: c_int,
    pub adapter->dcb_cfg.pfc_mode_enable: bool pfc_en =,
    if (adapter.ixgbe_ieee_pfc)
    pub !!(adapter->ixgbe_ieee_pfc->pfc_en): pfc_en |=,
    if (!(adapter.flags & IXGBE_FLAG_DCB_ENABLED) || !pfc_en) {
    }
// update stats for each tc, only valid with PFC enabled
    pub {: for (i = 0; i < MAX_TX_PACKET_BUFFERS; i++),
    pub pxoffrxc: u32,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub IXGBE_PXOFFRXC(i)): pxoffrxc = IXGBE_READ_REG(hw,,
    default:
    pub IXGBE_PXOFFRXCNT(i)): pxoffrxc = IXGBE_READ_REG(hw,,
    }
    pub pxoffrxc: hwstats->pxoffrxc[i] +=,
// Get the TC for given UP
    pub i): tc = netdev_get_prio_tc_map(adapter->netdev,,
    pub pxoffrxc: xoff[tc] +=,
    }
// disarm tx queues that have received xoff frames
    pub {: for (i = 0; i < adapter->num_tx_queues; i++),
    pub adapter->tx_ring[i]: *mut *mut ixgbe_ring tx_ring =,
    pub tx_ring->dcb_tc: tc =,
    if (xoff[tc])
    pub tx_ring->state): clear_bit(__IXGBE_HANG_CHECK_ARMED,,
    }
    pub {: for (i = 0; i < adapter->num_xdp_queues; i++),
    pub adapter->xdp_ring[i]: *mut *mut ixgbe_ring xdp_ring =,
    pub xdp_ring->dcb_tc: tc =,
    if (xoff[tc])
    pub xdp_ring->state): clear_bit(__IXGBE_HANG_CHECK_ARMED,,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_get_tx_completed(ring: *mut ixgbe_ring) -> u64 {
    static u64 ixgbe_get_tx_completed(struct ixgbe_ring *ring)
    {
    pub ring->stats.packets: return,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_get_tx_pending(ring: *mut ixgbe_ring) -> u64 {
    static u64 ixgbe_get_tx_pending(struct ixgbe_ring *ring)
    {
    pub tail: unsigned int head,,
    pub ring->next_to_clean: head =,
    pub ring->next_to_use: tail =,
    pub head: return ((head <= tail) ? tail : tail + ring->count) -,
    }
//
// ixgbe_get_vf_idx - provide VF index number based on queue index
// @adapter: pointer to the adapter struct
// @queue: Tx queue identifier
// @vf: output VF index
//
// Provide VF index number associated to the input queue.
//
// Returns: 0 if VF provided or error number.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_get_vf_idx(adapter: *mut ixgbe_adapter, queue: u16, vf: *mut u16) -> c_int {
    static int ixgbe_get_vf_idx(struct ixgbe_adapter *adapter, u16 queue, u16 *vf)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub queue_count: u8,
    pub reg: u32,
    if (queue >= adapter.num_tx_queues)
    pub -EINVAL: return,
// Determine number of queues by checking
// number of virtual functions
//
    pub IXGBE_GCR_EXT): reg = IXGBE_READ_REG(hw,,
    switch (reg & IXGBE_GCR_EXT_VT_MODE_MASK) {
    case IXGBE_GCR_EXT_VT_MODE_64:
    pub IXGBE_64VFS_QUEUES: queue_count =,
    case IXGBE_GCR_EXT_VT_MODE_32:
    pub IXGBE_32VFS_QUEUES: queue_count =,
    case IXGBE_GCR_EXT_VT_MODE_16:
    pub IXGBE_16VFS_QUEUES: queue_count =,
    default:
    pub -EINVAL: return,
    }
// vf = queue / queue_count;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_tx_hang(tx_ring: *mut ixgbe_ring) -> bool {
    static bool ixgbe_check_tx_hang(struct ixgbe_ring *tx_ring)
    {
    pub ixgbe_get_tx_completed(tx_ring): u32 tx_done =,
    pub tx_ring->tx_stats.tx_done_old: u32 tx_done_old =,
    pub ixgbe_get_tx_pending(tx_ring): u32 tx_pending =,
//
// Check for a hung queue, but be thorough. This verifies
// that a transmit has been completed since the previous
// check AND there is at least one packet pending. The
// ARMED bit is set to indicate a potential hang. The
// bit is cleared if a pause frame is received to remove
// false hang detection due to PFC or 802.3x frames. By
// requiring this to fail twice we avoid races with
// pfc clearing the ARMED bit and conditions where we
// run the check_tx_hang logic with a transmit completion
// pending but without time to complete it yet.
//
    if (tx_done_old == tx_done && tx_pending)
// make sure it is true for two checks in a row
    return test_and_set_bit(__IXGBE_HANG_CHECK_ARMED,
// update completed stats and continue
    pub tx_done: tx_ring->tx_stats.tx_done_old =,
// reset the countdown
    pub tx_ring->state): clear_bit(__IXGBE_HANG_CHECK_ARMED,,
    pub false: return,
    }
//
// ixgbe_tx_timeout_reset - initiate reset due to Tx timeout
// @adapter: driver private struct
//
#[no_mangle]
unsafe extern "C" fn ixgbe_tx_timeout_reset(adapter: *mut ixgbe_adapter) {
    static void ixgbe_tx_timeout_reset(struct ixgbe_adapter *adapter)
    {
// Do the reset outside of interrupt context
    if (!test_bit(__IXGBE_DOWN, &adapter.state)) {
    pub &adapter->state): set_bit(__IXGBE_RESET_REQUESTED,,
    pub timeout\n"): e_warn(drv, "initiating reset due to tx,
    }
    }
//
// ixgbe_tx_maxrate - callback to set the maximum per-queue bitrate
// @netdev: network interface device structure
// @queue_index: Tx queue to set
// @maxrate: desired maximum transmit bitrate
//
    static int ixgbe_tx_maxrate(struct net_device *netdev,
    int queue_index, u32 maxrate)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ixgbe_link_mbps(adapter): u32 bcnrc_val =,
    if (!maxrate)
    pub 0: return,
// Calculate the rate factor values to set
    pub IXGBE_RTTBCNRC_RF_INT_SHIFT: bcnrc_val <<=,
    pub maxrate: bcnrc_val /=,
// clear everything but the rate factor
    bcnrc_val &= IXGBE_RTTBCNRC_RF_INT_MASK |
// enable the rate scheduler
    pub IXGBE_RTTBCNRC_RS_ENA: bcnrc_val |=,
    pub queue_index): IXGBE_WRITE_REG(hw, IXGBE_RTTDQSEL,,
    pub bcnrc_val): IXGBE_WRITE_REG(hw, IXGBE_RTTBCNRC,,
    pub 0: return,
    }
//
// ixgbe_update_tx_ring_stats - Update Tx ring specific counters
// @tx_ring: ring to update
// @q_vector: queue vector ring belongs to
// @pkts: number of processed packets
// @bytes: number of processed bytes
//
    void ixgbe_update_tx_ring_stats(struct ixgbe_ring *tx_ring,
    struct ixgbe_q_vector *q_vector, u64 pkts,
    u64 bytes)
    {
    pub bytes: tx_ring->stats.bytes +=,
    pub pkts: tx_ring->stats.packets +=,
    pub bytes: q_vector->tx.total_bytes +=,
    pub pkts: q_vector->tx.total_packets +=,
    }
//
// ixgbe_update_rx_ring_stats - Update Rx ring specific counters
// @rx_ring: ring to update
// @q_vector: queue vector ring belongs to
// @pkts: number of processed packets
// @bytes: number of processed bytes
//
    void ixgbe_update_rx_ring_stats(struct ixgbe_ring *rx_ring,
    struct ixgbe_q_vector *q_vector, u64 pkts,
    u64 bytes)
    {
    pub bytes: rx_ring->stats.bytes +=,
    pub pkts: rx_ring->stats.packets +=,
    pub bytes: q_vector->rx.total_bytes +=,
    pub pkts: q_vector->rx.total_packets +=,
    }
//
// ixgbe_pf_handle_tx_hang - handle Tx hang on PF
// @tx_ring: tx ring number
// @next: next ring
//
// Prints a message containing details about the tx hang.
//
    static void ixgbe_pf_handle_tx_hang(struct ixgbe_ring *tx_ring,
    unsigned int next)
    {
    pub netdev_priv(tx_ring->netdev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    e_err(drv, "Detected Tx Unit Hang\n"
    "  Tx Queue             <%d>\n"
    "  TDH, TDT             <%x>, <%x>\n"
    "  next_to_use          <%x>\n"
    "  next_to_clean        <%x>\n"
    "tx_buffer_info[next_to_clean]\n"
    "  time_stamp           <%lx>\n"
    "  jiffies              <%lx>\n",
    tx_ring.queue_index,
    IXGBE_READ_REG(hw, IXGBE_TDH(tx_ring.reg_idx)),
    IXGBE_READ_REG(hw, IXGBE_TDT(tx_ring.reg_idx)),
    tx_ring.next_to_use, next,
    pub jiffies): tx_ring->tx_buffer_info[next].time_stamp,,
    netif_stop_subqueue(tx_ring.netdev,
    }
//
// ixgbe_vf_handle_tx_hang - handle Tx hang on VF
// @adapter: structure containing ring specific data
// @vf: VF index
//
// Print a message containing details about malicious driver detection.
// Set malicious VF link down if the detection happened several times.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_vf_handle_tx_hang(adapter: *mut ixgbe_adapter, vf: u16) {
    static void ixgbe_vf_handle_tx_hang(struct ixgbe_adapter *adapter, u16 vf)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    if (adapter.hw.mac.type != ixgbe_mac_e610)
    e_warn(drv,
    "Malicious Driver Detection tx hang detected on PF %d VF %d MAC: %pM",
    pub adapter->vfinfo[vf].vf_mac_addresses): hw->bus.func, vf,,
    if (adapter.tx_hang_count[vf] == IXGBE_MAX_TX_VF_HANGS) {
    ixgbe_set_vf_link_state(adapter, vf,
    pub 0: adapter->tx_hang_count[vf] =,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_poll_tx_icache(hw: *mut ixgbe_hw, queue: u16, idx: u16) -> u32 {
    static u32 ixgbe_poll_tx_icache(struct ixgbe_hw *hw, u16 queue, u16 idx)
    {
    pub idx): *mut *mut IXGBE_WRITE_REG(hw, IXGBE_TXDESCIC, queue,
    pub IXGBE_TXDESCIC): return IXGBE_READ_REG(hw,,
    }
//
// ixgbe_check_illegal_queue - search for queue with illegal packet
// @adapter: structure containing ring specific data
// @queue: queue index
//
// Check if tx descriptor connected with input queue
// contains illegal packet.
//
// Returns: true if queue contain illegal packet.
//
    static bool ixgbe_check_illegal_queue(struct ixgbe_adapter *adapter,
    u16 queue)
    {
    pub type_reg: u32 hdr_len_reg, mss_len_reg,,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub reg: u32 mss_len, header_len,,
    pub {: for (u16 i = 0; i < IXGBE_MAX_TX_DESCRIPTORS; i++),
// HW will clear bit IXGBE_TXDESCIC_READY when address
// is written to address field. HW will set this bit
// when iCache read is done, and data is ready at TIC_DWx.
// Set descriptor address.
//
    read_poll_timeout(ixgbe_poll_tx_icache, reg,
    !(reg & IXGBE_TXDESCIC_READY), 0, 0, false,
    pub i): hw, queue,,
// read tx descriptor access registers
    pub IXGBE_TIC_DW2(IXGBE_VLAN_MACIP_LENS_REG)): hdr_len_reg = IXGBE_READ_REG(hw,,
    pub IXGBE_TIC_DW2(IXGBE_TYPE_TUCMD_MLHL)): type_reg = IXGBE_READ_REG(hw,,
    pub IXGBE_TIC_DW2(IXGBE_MSS_L4LEN_IDX)): mss_len_reg = IXGBE_READ_REG(hw,,
// check if Advanced Context Descriptor
    if (FIELD_GET(IXGBE_ADVTXD_DTYP_MASK, type_reg) !=
    IXGBE_ADVTXD_DTYP_CTXT)
// check for illegal MSS and Header length
    pub mss_len_reg): mss_len = FIELD_GET(IXGBE_ADVTXD_MSS_MASK,,
    header_len = FIELD_GET(IXGBE_ADVTXD_HEADER_LEN_MASK,
    if ((mss_len + header_len) > SZ_16K) {
    pub long\n"): e_warn(probe, "mss len + header len too,
    pub true: return,
    }
    }
    pub false: return,
    }
//
// ixgbe_handle_mdd_event - handle mdd event
// @adapter: structure containing ring specific data
// @tx_ring: tx descriptor ring to handle
//
// Reset VF driver if malicious vf detected or
// illegal packet in an any queue detected.
//
    static void ixgbe_handle_mdd_event(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *tx_ring)
    {
    pub q: u16 vf,,
    if (adapter.vfinfo && ixgbe_check_mdd_event(adapter)) {
// vf mdd info and malicious vf detected
    if (!ixgbe_get_vf_idx(adapter, tx_ring.queue_index, &vf))
    pub vf): ixgbe_vf_handle_tx_hang(adapter,,
    } else {
// malicious vf not detected
    pub {: for (q = 0; q < IXGBE_MAX_TX_QUEUES; q++),
    if (ixgbe_check_illegal_queue(adapter, q) &&
    !ixgbe_get_vf_idx(adapter, q, &vf))
// illegal queue detected
    pub vf): ixgbe_vf_handle_tx_hang(adapter,,
    }
    }
    }
//
// ixgbe_clean_tx_irq - Reclaim resources after transmit completes
// @q_vector: structure containing interrupt and ring information
// @tx_ring: tx ring to clean
// @napi_budget: Used to determine if we are in netpoll
//
    static bool ixgbe_clean_tx_irq(struct ixgbe_q_vector *q_vector,
    struct ixgbe_ring *tx_ring, int napi_budget)
    {
    pub q_vector->adapter: *mut *mut ixgbe_adapter adapter =,
    pub tx_buffer: *mut ixgbe_tx_buffer,
    pub tx_desc: *mut union ixgbe_adv_tx_desc,
    pub 0: unsigned int total_bytes = 0, total_packets = 0, total_ipsec =,
    pub q_vector->tx.work_limit: unsigned int budget =,
    pub tx_ring->next_to_clean: unsigned int i =,
    pub txq: *mut netdev_queue,
    if (test_bit(__IXGBE_DOWN, &adapter.state))
    pub true: return,
    pub &tx_ring->tx_buffer_info[i]: tx_buffer =,
    pub i): tx_desc = IXGBE_TX_DESC(tx_ring,,
    pub tx_ring->count: i -=,
    do {
    pub tx_buffer->next_to_watch: *mut *mut union ixgbe_adv_tx_desc eop_desc =,
// if next_to_watch is not set then there is no work pending
    if (!eop_desc)
// prevent any other reads prior to eop_desc
// if DD is not set pending work has not been completed
    if (!(eop_desc.wb.status & cpu_to_le32(IXGBE_TXD_STAT_DD)))
// clear next_to_watch to prevent false hangs
    pub NULL: tx_buffer->next_to_watch =,
// update the statistics for this packet
    pub tx_buffer->bytecount: total_bytes +=,
    pub tx_buffer->gso_segs: total_packets +=,
    if (tx_buffer.tx_flags & IXGBE_TX_FLAGS_IPSEC)
// free the skb
    if (ring_is_xdp(tx_ring))
    else
    pub napi_budget): napi_consume_skb(tx_buffer->skb,,
// unmap skb header data
    dma_unmap_single(tx_ring.dev,
    dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
// clear tx_buffer data
    pub 0): dma_unmap_len_set(tx_buffer, len,,
// unmap remaining buffers
    while (tx_desc != eop_desc) {
    if (unlikely(!i)) {
    pub tx_ring->count: i -=,
    pub tx_ring->tx_buffer_info: tx_buffer =,
    pub 0): tx_desc = IXGBE_TX_DESC(tx_ring,,
    }
// unmap any remaining paged data
    if (dma_unmap_len(tx_buffer, len)) {
    dma_unmap_page(tx_ring.dev,
    dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
    pub 0): dma_unmap_len_set(tx_buffer, len,,
    }
    }
// move us one more past the eop_desc for start of next pkt
    if (unlikely(!i)) {
    pub tx_ring->count: i -=,
    pub tx_ring->tx_buffer_info: tx_buffer =,
    pub 0): tx_desc = IXGBE_TX_DESC(tx_ring,,
    }
// issue prefetch for next Tx descriptor
// update budget accounting
    pub (likely(budget)): } while,
    pub tx_ring->count: i +=,
    pub i: tx_ring->next_to_clean =,
    ixgbe_update_tx_ring_stats(tx_ring, q_vector, total_packets,
    pub total_ipsec: adapter->tx_ipsec +=,
    if (ring_is_xdp(tx_ring))
    pub !!budget: return,
    if (check_for_tx_hang(tx_ring) && ixgbe_check_tx_hang(tx_ring)) {
    if (adapter.hw.mac.type == ixgbe_mac_e610)
    pub tx_ring): ixgbe_handle_mdd_event(adapter,,
    pub i): ixgbe_pf_handle_tx_hang(tx_ring,,
    e_info(probe,
    "tx hang %d detected on queue %d, resetting adapter\n",
    pub tx_ring->queue_index): adapter->tx_timeout_count + 1,,
// schedule immediate reset if we believe we hung
// the adapter is about to reset, no point in enabling stuff
    pub true: return,
    }

    pub tx_ring->queue_index): txq = netdev_get_tx_queue(tx_ring->netdev,,
    if (!__netif_txq_completed_wake(txq, total_packets, total_bytes,
    ixgbe_desc_unused(tx_ring),
    TX_WAKE_THRESHOLD,
    !netif_carrier_ok(tx_ring.netdev) ||
    test_bit(__IXGBE_DOWN, &adapter.state)))
    pub !!budget: return,
    }

    static void ixgbe_update_tx_dca(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *tx_ring,
    int cpu)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub 0: u32 txctrl =,
    pub reg_offset: u16,
    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED)
    pub cpu): txctrl = dca3_get_tag(tx_ring->dev,,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub IXGBE_DCA_TXCTRL(tx_ring->reg_idx): reg_offset =,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    pub IXGBE_DCA_TXCTRL_82599(tx_ring->reg_idx): reg_offset =,
    pub IXGBE_DCA_TXCTRL_CPUID_SHIFT_82599: txctrl <<=,
    default:
// for unknown hardware do not write register
    }
//
// We can enable relaxed ordering for reads, but not writes when
// DCA is enabled.  This is due to a known issue in some chipsets
// which will cause the DCA tag to be cleared.
//
    txctrl |= IXGBE_DCA_TXCTRL_DESC_RRO_EN |
    IXGBE_DCA_TXCTRL_DATA_RRO_EN |
    pub txctrl): IXGBE_WRITE_REG(hw, reg_offset,,
    }
    static void ixgbe_update_rx_dca(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *rx_ring,
    int cpu)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub 0: u32 rxctrl =,
    pub rx_ring->reg_idx: u8 reg_idx =,
    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED)
    pub cpu): rxctrl = dca3_get_tag(rx_ring->dev,,
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    pub IXGBE_DCA_RXCTRL_CPUID_SHIFT_82599: rxctrl <<=,
    default:
    }
//
// We can enable relaxed ordering for reads, but not writes when
// DCA is enabled.  This is due to a known issue in some chipsets
// which will cause the DCA tag to be cleared.
//
    rxctrl |= IXGBE_DCA_RXCTRL_DESC_RRO_EN |
    IXGBE_DCA_RXCTRL_DATA_DCA_EN |
    pub rxctrl): IXGBE_WRITE_REG(hw, IXGBE_DCA_RXCTRL(reg_idx),,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_update_dca(q_vector: *mut ixgbe_q_vector) {
    static void ixgbe_update_dca(struct ixgbe_q_vector *q_vector)
    {
    pub q_vector->adapter: *mut *mut ixgbe_adapter adapter =,
    pub ring: *mut ixgbe_ring,
    pub get_cpu(): int cpu =,
    if (q_vector.cpu == cpu)
    pub out_no_update: goto,
    ixgbe_for_each_ring(ring, q_vector.tx)
    pub cpu): ixgbe_update_tx_dca(adapter, ring,,
    ixgbe_for_each_ring(ring, q_vector.rx)
    pub cpu): ixgbe_update_rx_dca(adapter, ring,,
    pub cpu: q_vector->cpu =,
    out_no_update:
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_dca(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_dca(struct ixgbe_adapter *adapter)
    {
    pub i: c_int,
// always use CB2 mode, difference is masked in the CB driver
    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED)
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_DCA_CTRL,
    else
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_DCA_CTRL,
    pub {: for (i = 0; i < adapter->num_q_vectors; i++),
    pub -1: adapter->q_vector[i]->cpu =,
    }
    }
#[no_mangle]
unsafe extern "C" fn __ixgbe_notify_dca(dev: *mut device, data: *mut c_void) -> c_int {
    static int __ixgbe_notify_dca(struct device *dev, void *data)
    {
    pub dev_get_drvdata(dev): *mut *mut ixgbe_adapter adapter =,
    pub )data: *mut *mut unsigned long event = (unsigned long,
    if (!(adapter.flags & IXGBE_FLAG_DCA_CAPABLE))
    pub 0: return,
    switch (event) {
    case DCA_PROVIDER_ADD:
// if we're already enabled, don't do it again
    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED)
    if (dca_add_requester(dev) == 0) {
    pub IXGBE_FLAG_DCA_ENABLED: adapter->flags |=,
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_DCA_CTRL,
    }
    pub /: *mut *mut fallthrough; / DCA is disabled.,
    case DCA_PROVIDER_REMOVE:
    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED) {
    pub ~IXGBE_FLAG_DCA_ENABLED: adapter->flags &=,
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_DCA_CTRL,
    }
    }
    pub 0: return,
    }

    ((1ul << IXGBE_RXDADV_RSSTYPE_IPV4_TCP) | \
    (1ul << IXGBE_RXDADV_RSSTYPE_IPV4_UDP) | \
    (1ul << IXGBE_RXDADV_RSSTYPE_IPV6_TCP) | \
    (1ul << IXGBE_RXDADV_RSSTYPE_IPV6_UDP))
    static inline void ixgbe_rx_hash(struct ixgbe_ring *ring,
    union ixgbe_adv_rx_desc *rx_desc,
    struct sk_buff *skb)
    {
    pub rss_type: u16,
    if (!(ring.netdev.features & NETIF_F_RXHASH))
    rss_type = le16_to_cpu(rx_desc.wb.lower.lo_dword.hs_rss.pkt_info) &
    if (!rss_type)
    skb_set_hash(skb, le32_to_cpu(rx_desc.wb.lower.hi_dword.rss),
    (IXGBE_RSS_L4_TYPES_MASK & (1ul << rss_type)) ?
    pub PKT_HASH_TYPE_L3): PKT_HASH_TYPE_L4 :,
    }

//
// ixgbe_rx_is_fcoe - check the rx desc for incoming pkt type
// @ring: structure containing ring specific data
// @rx_desc: advanced rx descriptor
//
// Returns : true if it is FCoE pkt
//
    static inline bool ixgbe_rx_is_fcoe(struct ixgbe_ring *ring,
    union ixgbe_adv_rx_desc *rx_desc)
    {
    pub rx_desc->wb.lower.lo_dword.hs_rss.pkt_info: __le16 pkt_info =,
    return test_bit(__IXGBE_RX_FCOE, ring.state) &&
    ((pkt_info & cpu_to_le16(IXGBE_RXDADV_PKTTYPE_ETQF_MASK)) ==
    (cpu_to_le16(IXGBE_ETQF_FILTER_FCOE <<
    }

//
// ixgbe_rx_checksum - indicate in skb if hw indicated a good cksum
// @ring: structure containing ring specific data
// @rx_desc: current Rx descriptor being processed
// @skb: skb currently being received and modified
//
    static inline void ixgbe_rx_checksum(struct ixgbe_ring *ring,
    union ixgbe_adv_rx_desc *rx_desc,
    struct sk_buff *skb)
    {
    pub rx_desc->wb.lower.lo_dword.hs_rss.pkt_info: __le16 pkt_info =,
    pub false: bool encap_pkt =,
// Rx csum disabled
    if (!(ring.netdev.features & NETIF_F_RXCSUM))
// check for VXLAN and Geneve packets
    if (pkt_info & cpu_to_le16(IXGBE_RXDADV_PKTTYPE_VXLAN)) {
    pub true: encap_pkt =,
    pub 1: skb->encapsulation =,
    }
// if IP and error
    if (ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_IPCS) &&
    ixgbe_test_staterr(rx_desc, IXGBE_RXDADV_ERR_IPE)) {
    }
    if (!ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_L4CS))
    if (ixgbe_test_staterr(rx_desc, IXGBE_RXDADV_ERR_TCPE)) {
//
// 82599 errata, UDP frames with a 0 checksum can be marked as
// checksum errors.
//
    if ((pkt_info & cpu_to_le16(IXGBE_RXDADV_PKTTYPE_UDP)) &&
    test_bit(__IXGBE_RX_CSUM_UDP_ZERO_ERR, ring.state))
    }
// It must be a TCP or UDP packet with a valid checksum
    pub CHECKSUM_UNNECESSARY: skb->ip_summed =,
    if (encap_pkt) {
    if (!ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_OUTERIPCS))
    if (ixgbe_test_staterr(rx_desc, IXGBE_RXDADV_ERR_OUTERIPER)) {
    pub CHECKSUM_NONE: skb->ip_summed =,
    }
// If we checked the outer header let the stack know
    pub 1: skb->csum_level =,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_rx_offset(rx_ring: *mut ixgbe_ring) -> c_uint {
    static unsigned int ixgbe_rx_offset(struct ixgbe_ring *rx_ring)
    {
    pub 0: return ring_uses_build_skb(rx_ring) ? IXGBE_SKB_PAD :,
    }
    static bool ixgbe_alloc_mapped_page(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *bi)
    {
    pub bi->page: *mut *mut page page =,
    pub dma: dma_addr_t,
// since we are recycling buffers we should seldom need to alloc
    if (likely(page))
    pub true: return,
// alloc new page for storage
    pub dev_alloc_pages(ixgbe_rx_pg_order(rx_ring)): page =,
    if (unlikely(!page)) {
    pub false: return,
    }
// map page for use
    dma = dma_map_page_attrs(rx_ring.dev, page, 0,
    ixgbe_rx_pg_size(rx_ring),
    DMA_FROM_DEVICE,
//
// if mapping failed free memory back to system since
// there isn't much point in holding memory we can't use
//
    if (dma_mapping_error(rx_ring.dev, dma)) {
    pub ixgbe_rx_pg_order(rx_ring)): __free_pages(page,,
    pub false: return,
    }
    pub dma: bi->dma =,
    pub page: bi->page =,
    pub rx_ring->rx_offset: bi->page_offset =,
    pub 1): page_ref_add(page, USHRT_MAX -,
    pub USHRT_MAX: bi->pagecnt_bias =,
    pub true: return,
    }
//
// ixgbe_alloc_rx_buffers - Replace used receive buffers
// @rx_ring: ring to place buffers on
// @cleaned_count: number of buffers to replace
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_alloc_rx_buffers(rx_ring: *mut ixgbe_ring, cleaned_count: u16) {
    void ixgbe_alloc_rx_buffers(struct ixgbe_ring *rx_ring, u16 cleaned_count)
    {
    pub rx_desc: *mut union ixgbe_adv_rx_desc,
    pub bi: *mut ixgbe_rx_buffer,
    pub rx_ring->next_to_use: u16 i =,
    pub bufsz: u16,
// nothing to do
    if (!cleaned_count)
    pub i): rx_desc = IXGBE_RX_DESC(rx_ring,,
    pub &rx_ring->rx_buffer_info[i]: bi =,
    pub rx_ring->count: i -=,
    pub ixgbe_rx_bufsz(rx_ring): bufsz =,
    do {
    if (!ixgbe_alloc_mapped_page(rx_ring, bi))
// sync the buffer for use by the device
    dma_sync_single_range_for_device(rx_ring.dev, bi.dma,
    bi.page_offset, bufsz,
//
// Refresh the desc even if buffer_addrs didn't change
// because each write-back erases this info.
//
    pub bi->page_offset): rx_desc->read.pkt_addr = cpu_to_le64(bi->dma +,
    if (unlikely(!i)) {
    pub 0): rx_desc = IXGBE_RX_DESC(rx_ring,,
    pub rx_ring->rx_buffer_info: bi =,
    pub rx_ring->count: i -=,
    }
// clear the length for the next_to_use descriptor
    pub 0: rx_desc->wb.upper.length =,
    pub (cleaned_count): } while,
    pub rx_ring->count: i +=,
    if (rx_ring.next_to_use != i) {
    pub i: rx_ring->next_to_use =,
// update next to alloc since we have filled the ring
    pub i: rx_ring->next_to_alloc =,
// Force memory writes to complete before letting h/w
// know there are new descriptors to fetch.  (Only
// applicable for weak-ordered memory model archs,
// such as IA-64).
//
    pub rx_ring->tail): writel(i,,
    }
    }
    static void ixgbe_set_rsc_gso_size(struct ixgbe_ring *ring,
    struct sk_buff *skb)
    {
    pub skb_headlen(skb): u16 hdr_len =,
// set gso_size to avoid messing up TCP MSS
    skb_shinfo(skb).gso_size = DIV_ROUND_UP((skb.len - hdr_len),
    pub SKB_GSO_TCPV4: skb_shinfo(skb)->gso_type =,
    }
    static void ixgbe_update_rsc_stats(struct ixgbe_ring *rx_ring,
    struct sk_buff *skb)
    {
// if append_cnt is 0 then frame is not RSC
    if (!IXGBE_CB(skb).append_cnt)
    pub IXGBE_CB(skb)->append_cnt: rx_ring->rx_stats.rsc_count +=,
    pub skb): ixgbe_set_rsc_gso_size(rx_ring,,
// gso_size is computed using append_cnt so always clear it last
    pub 0: IXGBE_CB(skb)->append_cnt =,
    }
//
// ixgbe_process_skb_fields - Populate skb header fields from Rx descriptor
// @rx_ring: rx descriptor ring packet is being transacted on
// @rx_desc: pointer to the EOP Rx descriptor
// @skb: pointer to current skb being populated
//
// This function checks the ring, descriptor, and packet information in
// order to populate the hash, checksum, VLAN, timestamp, protocol, and
// other fields within the skb.
//
    void ixgbe_process_skb_fields(struct ixgbe_ring *rx_ring,
    union ixgbe_adv_rx_desc *rx_desc,
    struct sk_buff *skb)
    {
    pub rx_ring->netdev: *mut *mut net_device dev =,
    pub rx_ring->q_vector->adapter->flags: u32 flags =,
    pub skb): ixgbe_update_rsc_stats(rx_ring,,
    pub skb): ixgbe_rx_hash(rx_ring, rx_desc,,
    pub skb): ixgbe_rx_checksum(rx_ring, rx_desc,,
    if (unlikely(flags & IXGBE_FLAG_RX_HWTSTAMP_ENABLED))
    pub skb): ixgbe_ptp_rx_hwtstamp(rx_ring, rx_desc,,
    if ((dev.features & NETIF_F_HW_VLAN_CTAG_RX) &&
    ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_VP)) {
    pub le16_to_cpu(rx_desc->wb.upper.vlan): u16 vid =,
    pub vid): __vlan_hwaccel_put_tag(skb, htons(ETH_P_8021Q),,
    }
    if (ixgbe_test_staterr(rx_desc, IXGBE_RXDADV_STAT_SECP))
    pub skb): ixgbe_ipsec_rx(rx_ring, rx_desc,,
// record Rx queue, or update MACVLAN statistics
    if (netif_is_ixgbe(dev))
    pub rx_ring->queue_index): skb_record_rx_queue(skb,,
    else
    macvlan_count_rx(netdev_priv(dev), skb.len + ETH_HLEN, true,
    pub dev): skb->protocol = eth_type_trans(skb,,
    }
    void ixgbe_rx_skb(struct ixgbe_q_vector *q_vector,
    struct sk_buff *skb)
    {
    pub skb): napi_gro_receive(&q_vector->napi,,
    }
//
// ixgbe_is_non_eop - process handling of non-EOP buffers
// @rx_ring: Rx ring being processed
// @rx_desc: Rx descriptor for current buffer
// @skb: Current socket buffer containing buffer in progress
//
// This function updates next to clean.  If the buffer is an EOP buffer
// this function exits returning false, otherwise it will place the
// sk_buff in the next buffer to be chained and return true indicating
// that this is in fact a non-EOP buffer.
//
    static bool ixgbe_is_non_eop(struct ixgbe_ring *rx_ring,
    union ixgbe_adv_rx_desc *rx_desc,
    struct sk_buff *skb)
    {
    pub 1: u32 ntc = rx_ring->next_to_clean +,
// fetch, update, and store next to clean
    pub 0: ntc = (ntc < rx_ring->count) ? ntc :,
    pub ntc: rx_ring->next_to_clean =,
    pub ntc)): prefetch(IXGBE_RX_DESC(rx_ring,,
// update RSC append count if present
    if (ring_is_rsc_enabled(rx_ring)) {
    __le32 rsc_enabled = rx_desc.wb.lower.lo_dword.data &
    if (unlikely(rsc_enabled)) {
    pub le32_to_cpu(rsc_enabled): u32 rsc_cnt =,
    pub IXGBE_RXDADV_RSCCNT_SHIFT: rsc_cnt >>=,
    pub 1: IXGBE_CB(skb)->append_cnt += rsc_cnt -,
// update ntc based on RSC value
    pub le32_to_cpu(rx_desc->wb.upper.status_error): ntc =,
    pub IXGBE_RXDADV_NEXTP_MASK: ntc &=,
    pub IXGBE_RXDADV_NEXTP_SHIFT: ntc >>=,
    }
    }
// if we are the last buffer then there is nothing else to do
    if (likely(ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_EOP)))
    pub false: return,
// place skb in next buffer to be received
    pub skb: rx_ring->rx_buffer_info[ntc].skb =,
    pub true: return,
    }
//
// ixgbe_pull_tail - ixgbe specific version of skb_pull_tail
// @rx_ring: rx descriptor ring packet is being transacted on
// @skb: pointer to current skb being adjusted
//
// This function is an ixgbe specific version of __pskb_pull_tail.  The
// main difference between this version and the original function is that
// this function can make several assumptions about the state of things
// that allow for significant optimizations versus the standard function.
// As a result we can do things like drop a frag and maintain an accurate
// truesize for the skb.
//
    static void ixgbe_pull_tail(struct ixgbe_ring *rx_ring,
    struct sk_buff *skb)
    {
    pub &skb_shinfo(skb)->frags[0]: *mut *mut skb_frag_t frag =,
    pub va: *mut c_uchar,
    pub pull_len: c_uint,
//
// it is valid to use page_address instead of kmap since we are
// working with pages allocated out of the lomem pool per
// alloc_page(GFP_ATOMIC)
//
    pub skb_frag_address(frag): va =,
//
// we need the header to contain the greater of either ETH_HLEN or
// 60 bytes if the skb->len is less than 60 for skb_pad.
//
    pub IXGBE_RX_HDR_SIZE): pull_len = eth_get_headlen(skb->dev, va,,
// align pull length to size of long to optimize memcpy performance
    pub sizeof(long))): skb_copy_to_linear_data(skb, va, ALIGN(pull_len,,
// update all of the pointers
    pub pull_len): skb_frag_size_sub(frag,,
    pub pull_len): skb_frag_off_add(frag,,
    pub pull_len: skb->data_len -=,
    pub pull_len: skb->tail +=,
    }
//
// ixgbe_dma_sync_frag - perform DMA sync for first frag of SKB
// @rx_ring: rx descriptor ring packet is being transacted on
// @skb: pointer to current skb being updated
//
// This function provides a basic DMA sync up for the first fragment of an
// skb.  The reason for doing this is that the first fragment cannot be
// unmapped until we have reached the end of packet descriptor for a buffer
// chain.
//
    static void ixgbe_dma_sync_frag(struct ixgbe_ring *rx_ring,
    struct sk_buff *skb)
    {
    if (ring_uses_build_skb(rx_ring)) {
    pub 1: unsigned long mask = (unsigned long)ixgbe_rx_pg_size(rx_ring) -,
    pub mask: unsigned long offset = (unsigned long)(skb->data) &,
    dma_sync_single_range_for_cpu(rx_ring.dev,
    IXGBE_CB(skb).dma,
    offset,
    skb_headlen(skb),
    } else {
    pub &skb_shinfo(skb)->frags[0]: *mut *mut skb_frag_t frag =,
    dma_sync_single_range_for_cpu(rx_ring.dev,
    IXGBE_CB(skb).dma,
    skb_frag_off(frag),
    skb_frag_size(frag),
    }
// If the page was released, just unmap it.
    if (unlikely(IXGBE_CB(skb).page_released)) {
    dma_unmap_page_attrs(rx_ring.dev, IXGBE_CB(skb).dma,
    ixgbe_rx_pg_size(rx_ring),
    DMA_FROM_DEVICE,
    }
    }
//
// ixgbe_cleanup_headers - Correct corrupted or empty headers
// @rx_ring: rx descriptor ring packet is being transacted on
// @rx_desc: pointer to the EOP Rx descriptor
// @skb: pointer to current skb being fixed
//
// Check if the skb is valid in the XDP case it will be an error pointer.
// Return true in this case to abort processing and advance to next
// descriptor.
//
// Check for corrupted packet headers caused by senders on the local L2
// embedded NIC switch not setting up their Tx Descriptors right.  These
// should be very rare.
//
// Also address the case where we are pulling data in on pages only
// and as such no data is present in the skb header.
//
// In addition if skb is not at least 60 bytes we need to pad it so that
// it is large enough to qualify as a valid Ethernet frame.
//
// Returns true if an error was encountered and skb was freed.
//
    bool ixgbe_cleanup_headers(struct ixgbe_ring *rx_ring,
    union ixgbe_adv_rx_desc *rx_desc,
    struct sk_buff *skb)
    {
    pub rx_ring->netdev: *mut *mut net_device netdev =,
// Verify netdev is present, and that packet does not have any
// errors that would be unacceptable to the netdev.
//
    if (!netdev ||
    (unlikely(ixgbe_test_staterr(rx_desc,
    IXGBE_RXDADV_ERR_FRAME_ERR_MASK) &&
    !(netdev.features & NETIF_F_RXALL)))) {
    pub true: return,
    }
// place header in linear portion of buffer
    if (!skb_headlen(skb))
    pub skb): ixgbe_pull_tail(rx_ring,,

// do not attempt to pad FCoE Frames as this will disrupt DDP
    if (ixgbe_rx_is_fcoe(rx_ring, rx_desc))
    pub false: return,

// if eth_skb_pad returns an error the skb was freed
    if (eth_skb_pad(skb))
    pub true: return,
    pub false: return,
    }
//
// ixgbe_reuse_rx_page - page flip buffer and store it back on the ring
// @rx_ring: rx descriptor ring to store buffers on
// @old_buff: donor buffer to have page reused
//
// Synchronizes page for reuse by the adapter
//
    static void ixgbe_reuse_rx_page(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *old_buff)
    {
    pub new_buff: *mut ixgbe_rx_buffer,
    pub rx_ring->next_to_alloc: u16 nta =,
    pub &rx_ring->rx_buffer_info[nta]: new_buff =,
// update, and store next to alloc
    pub 0: rx_ring->next_to_alloc = (nta < rx_ring->count) ? nta :,
// Transfer page from old buffer to new buffer.
// Move each member individually to avoid possible store
// forwarding stalls and unnecessary copy of skb.
//
    pub old_buff->dma: new_buff->dma =,
    pub old_buff->page: new_buff->page =,
    pub old_buff->page_offset: new_buff->page_offset =,
    pub old_buff->pagecnt_bias: new_buff->pagecnt_bias =,
    }
    static bool ixgbe_can_reuse_rx_page(struct ixgbe_rx_buffer *rx_buffer,
    int rx_buffer_pgcnt)
    {
    pub rx_buffer->pagecnt_bias: unsigned int pagecnt_bias =,
    pub rx_buffer->page: *mut *mut page page =,
// avoid re-using remote and pfmemalloc pages
    if (!dev_page_is_reusable(page))
    pub false: return,

// if we are only owner of page we can reuse it
    if (unlikely((rx_buffer_pgcnt - pagecnt_bias) > 1))
    pub false: return,

// The last offset is a bit aggressive in that we assume the
// worst case of FCoE being enabled and using a 3K buffer.
// However this should have minimal impact as the 1K extra is
// still less than one buffer in size.
//

    (SKB_WITH_OVERHEAD(PAGE_SIZE) - IXGBE_RXBUFFER_3K)
    if (rx_buffer.page_offset > IXGBE_LAST_OFFSET)
    pub false: return,

// If we have drained the page fragment pool we need to update
// the pagecnt_bias and page count so that we fully restock the
// number of references the driver holds.
//
    if (unlikely(pagecnt_bias == 1)) {
    pub 1): page_ref_add(page, USHRT_MAX -,
    pub USHRT_MAX: rx_buffer->pagecnt_bias =,
    }
    pub true: return,
    }
//
// ixgbe_add_rx_frag - Add contents of Rx buffer to sk_buff
// @rx_ring: rx descriptor ring to transact packets on
// @rx_buffer: buffer containing page to add
// @skb: sk_buff to place the data into
// @size: size of data in rx_buffer
//
// This function will add the data contained in rx_buffer->page to the skb.
// This is done either through a direct copy if the data in the buffer is
// less than the skb header size, otherwise it will just attach the page as
// a frag to the skb.
//
// The function will then update the page offset if necessary and return
// true if the buffer can be reused by the adapter.
//
    static void ixgbe_add_rx_frag(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *rx_buffer,
    struct sk_buff *skb,
    unsigned int size)
    {

    pub 2: unsigned int truesize = ixgbe_rx_pg_size(rx_ring) /,

    unsigned int truesize = rx_ring.rx_offset ?
    SKB_DATA_ALIGN(rx_ring.rx_offset + size) :

    skb_add_rx_frag(skb, skb_shinfo(skb).nr_frags, rx_buffer.page,
    pub truesize): rx_buffer->page_offset, size,,

    pub truesize: rx_buffer->page_offset ^=,

    pub truesize: rx_buffer->page_offset +=,

    }
    static struct ixgbe_rx_buffer *ixgbe_get_rx_buffer(struct ixgbe_ring *rx_ring,
    union ixgbe_adv_rx_desc *rx_desc,
    struct sk_buff **skb,
    const unsigned int size,
    int *rx_buffer_pgcnt)
    {
    pub rx_buffer: *mut ixgbe_rx_buffer,
    pub &rx_ring->rx_buffer_info[rx_ring->next_to_clean]: rx_buffer =,
// rx_buffer_pgcnt =

// skb = rx_buffer->skb;
// Delay unmapping of the first packet. It carries the header
// information, HW may still access the header after the writeback.
// Only unmap it when EOP is reached
//
    if (!ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_EOP)) {
    if (!*skb)
    pub skip_sync: goto,
    } else {
    if (*skb)
    pub skb): *mut ixgbe_dma_sync_frag(rx_ring,,
    }
// we are reusing so sync this buffer for CPU use
    dma_sync_single_range_for_cpu(rx_ring.dev,
    rx_buffer.dma,
    rx_buffer.page_offset,
    size,
    skip_sync:
    pub rx_buffer: return,
    }
    static void ixgbe_put_rx_buffer(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *rx_buffer,
    struct sk_buff *skb,
    int rx_buffer_pgcnt)
    {
    if (ixgbe_can_reuse_rx_page(rx_buffer, rx_buffer_pgcnt)) {
// hand second half of page back to the ring
    pub rx_buffer): ixgbe_reuse_rx_page(rx_ring,,
    } else {
    if (skb && IXGBE_CB(skb).dma == rx_buffer.dma) {
// the page has been released from the ring
    pub true: IXGBE_CB(skb)->page_released =,
    } else {
// we are not reusing the buffer so unmap it
    dma_unmap_page_attrs(rx_ring.dev, rx_buffer.dma,
    ixgbe_rx_pg_size(rx_ring),
    DMA_FROM_DEVICE,
    }
    __page_frag_cache_drain(rx_buffer.page,
    }
// clear contents of rx_buffer
    pub NULL: rx_buffer->page =,
    pub NULL: rx_buffer->skb =,
    }
    static struct sk_buff *ixgbe_construct_skb(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *rx_buffer,
    struct xdp_buff *xdp,
    union ixgbe_adv_rx_desc *rx_desc)
    {
    pub xdp->data: unsigned int size = xdp->data_end -,

    pub 2: unsigned int truesize = ixgbe_rx_pg_size(rx_ring) /,

    unsigned int truesize = SKB_DATA_ALIGN(xdp.data_end -

    pub skb: *mut sk_buff,
// prefetch first cache line of first page
// Note, we get here by enabling legacy-rx via:
//
// ethtool --set-priv-flags <dev> legacy-rx on
//
// In this mode, we currently get 0 extra XDP headroom as
// opposed to having legacy-rx off, where we process XDP
// packets going to stack via ixgbe_build_skb(). The latter
// provides us currently with 192 bytes of headroom.
//
// For ixgbe_construct_skb() mode it means that the
// xdp->data_meta will always point to xdp->data, since
// the helper cannot expand the head. Should this ever
// change in future for legacy-rx mode on, then lets also
// add xdp->data_meta handling here.
//
// allocate a skb to store the frags
    pub IXGBE_RX_HDR_SIZE): skb = napi_alloc_skb(&rx_ring->q_vector->napi,,
    if (unlikely(!skb))
    pub NULL: return,
    if (size > IXGBE_RX_HDR_SIZE) {
    if (!ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_EOP))
    pub rx_buffer->dma: IXGBE_CB(skb)->dma =,
    skb_add_rx_frag(skb, 0, rx_buffer.page,
    xdp.data - page_address(rx_buffer.page),
    pub truesize): size,,

    pub truesize: rx_buffer->page_offset ^=,

    pub truesize: rx_buffer->page_offset +=,

    } else {
    memcpy(__skb_put(skb, size),
    pub sizeof(long))): xdp->data, ALIGN(size,,
    }
    pub skb: return,
    }
    static struct sk_buff *ixgbe_build_skb(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *rx_buffer,
    struct xdp_buff *xdp,
    union ixgbe_adv_rx_desc *rx_desc)
    {
    pub xdp->data_meta: unsigned int metasize = xdp->data -,

    pub 2: unsigned int truesize = ixgbe_rx_pg_size(rx_ring) /,

    unsigned int truesize = SKB_DATA_ALIGN(sizeof(struct skb_shared_info)) +
    SKB_DATA_ALIGN(xdp.data_end -

    pub skb: *mut sk_buff,
// Prefetch first cache line of first page. If xdp->data_meta
// is unused, this points exactly as xdp->data, otherwise we
// likely have a consumer accessing first few bytes of meta
// data, and then actual data.
//
// build an skb to around the page buffer
    pub truesize): skb = napi_build_skb(xdp->data_hard_start,,
    if (unlikely(!skb))
    pub NULL: return,
// update pointers within the skb to store the data
    pub xdp->data_hard_start): skb_reserve(skb, xdp->data -,
    pub xdp->data): __skb_put(skb, xdp->data_end -,
    if (metasize)
    pub metasize): skb_metadata_set(skb,,
// record DMA address if this is the start of a chain of buffers
    if (!ixgbe_test_staterr(rx_desc, IXGBE_RXD_STAT_EOP))
    pub rx_buffer->dma: IXGBE_CB(skb)->dma =,
// update buffer offset

    pub truesize: rx_buffer->page_offset ^=,

    pub truesize: rx_buffer->page_offset +=,

    pub skb: return,
    }
    static int ixgbe_run_xdp(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *rx_ring,
    struct xdp_buff *xdp)
    {
    pub IXGBE_XDP_PASS: int err, result =,
    pub xdp_prog: *mut bpf_prog,
    pub ring: *mut ixgbe_ring,
    pub xdpf: *mut xdp_frame,
    pub act: u32,
    pub READ_ONCE(rx_ring->xdp_prog): xdp_prog =,
    if (!xdp_prog)
    pub xdp_out: goto,
    pub /: *mut *mut prefetchw(xdp->data_hard_start); / xdp_frame write,
    pub xdp): act = bpf_prog_run_xdp(xdp_prog,,
    switch (act) {
    case XDP_PASS:
    case XDP_TX:
    pub xdp_convert_buff_to_frame(xdp): xdpf =,
    if (unlikely(!xdpf))
    pub out_failure: goto,
    pub ixgbe_determine_xdp_ring(adapter): ring =,
    if (static_branch_unlikely(&ixgbe_xdp_locking_key))
    pub xdpf): result = ixgbe_xmit_xdp_ring(ring,,
    if (static_branch_unlikely(&ixgbe_xdp_locking_key))
    if (result == IXGBE_XDP_CONSUMED)
    pub out_failure: goto,
    case XDP_REDIRECT:
    pub xdp_prog): err = xdp_do_redirect(adapter->netdev, xdp,,
    if (err)
    pub out_failure: goto,
    pub IXGBE_XDP_REDIR: result =,
    default:
    pub act): bpf_warn_invalid_xdp_action(rx_ring->netdev, xdp_prog,,
    case XDP_ABORTED:
    out_failure:
    pub act): trace_xdp_exception(rx_ring->netdev, xdp_prog,,
    pub /: *mut *mut fallthrough; / handle aborts by dropping packet,
    case XDP_DROP:
    pub IXGBE_XDP_CONSUMED: result =,
    }
    xdp_out:
    pub result: return,
    }
    static unsigned int ixgbe_rx_frame_truesize(struct ixgbe_ring *rx_ring,
    unsigned int size)
    {
    pub truesize: c_uint,

    pub /: *mut *mut truesize = ixgbe_rx_pg_size(rx_ring) / 2; / Must be power-of-2,

    truesize = rx_ring.rx_offset ?
    SKB_DATA_ALIGN(rx_ring.rx_offset + size) +
    SKB_DATA_ALIGN(sizeof(struct skb_shared_info)) :

    pub truesize: return,
    }
    static void ixgbe_rx_buffer_flip(struct ixgbe_ring *rx_ring,
    struct ixgbe_rx_buffer *rx_buffer,
    unsigned int size)
    {
    pub size): unsigned int truesize = ixgbe_rx_frame_truesize(rx_ring,,

    pub truesize: rx_buffer->page_offset ^=,

    pub truesize: rx_buffer->page_offset +=,

    }
//
// ixgbe_clean_rx_irq - Clean completed descriptors from Rx ring - bounce buf
// @q_vector: structure containing interrupt and ring information
// @rx_ring: rx descriptor ring to transact packets on
// @budget: Total limit on number of packets to process
//
// This function provides a "bounce buffer" approach to Rx interrupt
// processing.  The advantage to this is that on systems that have
// expensive overhead for IOMMU access this provides a means of avoiding
// it by maintaining the mapping of the page to the system.
//
// Returns amount of work completed
//
    static int ixgbe_clean_rx_irq(struct ixgbe_q_vector *q_vector,
    struct ixgbe_ring *rx_ring,
    const int budget)
    {
    pub 0: unsigned int total_rx_bytes = 0, total_rx_packets = 0, frame_sz =,
    pub q_vector->adapter: *mut *mut ixgbe_adapter adapter =,

    pub ddp_bytes: c_int,
    pub 0: unsigned int mss =,

    pub ixgbe_desc_unused(rx_ring): u16 cleaned_count =,
    pub rx_ring->rx_offset: unsigned int offset =,
    pub 0: unsigned int xdp_xmit =,
    pub xdp: xdp_buff,
    pub 0: int xdp_res =,
// Frame size depend on rx_ring setup when PAGE_SIZE=4K

    pub 0): frame_sz = ixgbe_rx_frame_truesize(rx_ring,,

    pub &rx_ring->xdp_rxq): xdp_init_buff(&xdp, frame_sz,,
    while (likely(total_rx_packets < budget)) {
    pub rx_desc: *mut union ixgbe_adv_rx_desc,
    pub rx_buffer: *mut ixgbe_rx_buffer,
    pub skb: *mut sk_buff,
    pub rx_buffer_pgcnt: c_int,
    pub size: c_uint,
// return some buffers to hardware, one at a time is too slow
    if (cleaned_count >= IXGBE_RX_BUFFER_WRITE) {
    pub cleaned_count): ixgbe_alloc_rx_buffers(rx_ring,,
    pub 0: cleaned_count =,
    }
    pub rx_ring->next_to_clean): rx_desc = IXGBE_RX_DESC(rx_ring,,
    pub le16_to_cpu(rx_desc->wb.upper.length): size =,
    if (!size)
// This memory barrier is needed to keep us from reading
// any other fields out of the rx_desc until we know the
// descriptor has been written back
//
    pub &rx_buffer_pgcnt): rx_buffer = ixgbe_get_rx_buffer(rx_ring, rx_desc, &skb, size,,
// retrieve a buffer from the ring
    if (!skb) {
    pub hard_start: *mut c_uchar,
    hard_start = page_address(rx_buffer.page) +
    pub offset: rx_buffer->page_offset -,
    pub true): xdp_prepare_buff(&xdp, hard_start, offset, size,,

// At larger PAGE_SIZE, frame_sz depend on len size
    pub size): xdp.frame_sz = ixgbe_rx_frame_truesize(rx_ring,,

    pub &xdp): xdp_res = ixgbe_run_xdp(adapter, rx_ring,,
    }
    if (xdp_res) {
    if (xdp_res & (IXGBE_XDP_TX | IXGBE_XDP_REDIR)) {
    pub xdp_res: xdp_xmit |=,
    pub size): ixgbe_rx_buffer_flip(rx_ring, rx_buffer,,
    } else {
    }
    pub size: total_rx_bytes +=,
    } else if (skb) {
    pub size): ixgbe_add_rx_frag(rx_ring, rx_buffer, skb,,
    } else if (ring_uses_build_skb(rx_ring)) {
    skb = ixgbe_build_skb(rx_ring, rx_buffer,
    pub rx_desc): &xdp,,
    } else {
    skb = ixgbe_construct_skb(rx_ring, rx_buffer,
    pub rx_desc): &xdp,,
    }
// exit if we failed to retrieve a buffer
    if (!xdp_res && !skb) {
    }
    pub rx_buffer_pgcnt): ixgbe_put_rx_buffer(rx_ring, rx_buffer, skb,,
// place incomplete frames back on ring for completion
    if (ixgbe_is_non_eop(rx_ring, rx_desc, skb))
// verify the packet layout is correct
    if (xdp_res || ixgbe_cleanup_headers(rx_ring, rx_desc, skb))
// probably a little skewed due to removing CRC
    pub skb->len: total_rx_bytes +=,
// populate checksum, timestamp, VLAN, and protocol
    pub skb): ixgbe_process_skb_fields(rx_ring, rx_desc,,

// if ddp, not passing to ULD unless for FCP_RSP or error
    if (ixgbe_rx_is_fcoe(rx_ring, rx_desc)) {
    pub skb): ddp_bytes = ixgbe_fcoe_ddp(adapter, rx_desc,,
// include DDPed FCoE data
    if (ddp_bytes > 0) {
    if (!mss) {
    mss = rx_ring.netdev.mtu -
    sizeof(struct fcoe_hdr) -
    sizeof(struct fc_frame_header) -
    pub fcoe_crc_eof): sizeof(struct,
    if (mss > 512)
    pub ~511: mss &=,
    }
    pub ddp_bytes: total_rx_bytes +=,
    total_rx_packets += DIV_ROUND_UP(ddp_bytes,
    }
    if (!ddp_bytes) {
    }
    }

    pub skb): ixgbe_rx_skb(q_vector,,
// update budget accounting
    }
    if (xdp_xmit & IXGBE_XDP_REDIR)
    if (xdp_xmit & IXGBE_XDP_TX) {
    pub ixgbe_determine_xdp_ring(adapter): *mut *mut ixgbe_ring ring =,
    }
    ixgbe_update_rx_ring_stats(rx_ring, q_vector, total_rx_packets,
    pub total_rx_packets: return,
    }
//
// ixgbe_configure_msix - Configure MSI-X hardware
// @adapter: board private structure
//
// ixgbe_configure_msix sets up the hardware to properly generate MSI-X
// interrupts.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_msix(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_msix(struct ixgbe_adapter *adapter)
    {
    pub q_vector: *mut ixgbe_q_vector,
    pub v_idx: c_int,
    pub mask: u32,
// Populate MSIX to EITR Select
    if (adapter.num_vfs > 32) {
    pub 1: u32 eitrsel = BIT(adapter->num_vfs - 32) -,
    pub eitrsel): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EITRSEL,,
    }
//
// Populate the IVAR table and set the ITR values to the
// corresponding register.
//
    pub {: for (v_idx = 0; v_idx < adapter->num_q_vectors; v_idx++),
    pub ring: *mut ixgbe_ring,
    pub adapter->q_vector[v_idx]: q_vector =,
    ixgbe_for_each_ring(ring, q_vector.rx)
    pub v_idx): ixgbe_set_ivar(adapter, 0, ring->reg_idx,,
    ixgbe_for_each_ring(ring, q_vector.tx)
    pub v_idx): ixgbe_set_ivar(adapter, 1, ring->reg_idx,,
    }
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82598EB:
    ixgbe_set_ivar(adapter, -1, IXGBE_IVAR_OTHER_CAUSES_INDEX,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pub v_idx): ixgbe_set_ivar(adapter, -1, 1,,
    default:
    }
    pub 1950): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EITR(v_idx),,
// set up to autoclear timer, and the vectors
    pub IXGBE_EIMS_ENABLE_MASK: mask =,
    mask &= ~(IXGBE_EIMS_OTHER |
    IXGBE_EIMS_MAILBOX |
    if (adapter.hw.mac.type == ixgbe_mac_e610)
    pub ~IXGBE_EIMS_FW_EVENT: mask &=,
    pub mask): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EIAC,,
    }
//
// ixgbe_update_itr - update the dynamic ITR value based on statistics
// @q_vector: structure containing interrupt and ring information
// @ring_container: structure containing ring performance data
//
// Stores a new ITR value based on packets and byte
// counts during the last interrupt.  The advantage of per interrupt
// computation is faster updates and more accurate ITR for the current
// traffic pattern.  Constants in this function were computed
// based on theoretical maximum wire speed and thresholds were set based
// on testing data as well as attempting to minimize response time
// while increasing bulk throughput.
//
    static void ixgbe_update_itr(struct ixgbe_q_vector *q_vector,
    struct ixgbe_ring_container *ring_container)
    {
    unsigned int itr = IXGBE_ITR_ADAPTIVE_MIN_USECS |
    pub bytes: unsigned int avg_wire_size, packets,,
    pub jiffies: unsigned long next_update =,
// If we don't have any rings just leave ourselves set for maximum
// possible latency so we take ourselves out of the equation.
//
    if (!ring_container.ring)
// If we didn't update within up to 1 - 2 jiffies we can assume
// that either packets are coming in so slow there hasn't been
// any work, or that there is so much work that NAPI is dealing
// with interrupt moderation and we don't need to do anything.
//
    if (time_after(next_update, ring_container.next_update))
    pub clear_counts: goto,
    pub ring_container->total_packets: packets =,
// We have no packets to actually measure against. This means
// either one of the other queues on this vector is active or
// we are a Tx queue doing TSO with too high of an interrupt rate.
//
// When this occurs just tick up our delay by the minimum value
// and hope that this extra delay will prevent us from being called
// without any work on our queue.
//
    if (!packets) {
    pub IXGBE_ITR_ADAPTIVE_MIN_INC: itr = (q_vector->itr >> 2) +,
    if (itr > IXGBE_ITR_ADAPTIVE_MAX_USECS)
    pub IXGBE_ITR_ADAPTIVE_MAX_USECS: itr =,
    pub IXGBE_ITR_ADAPTIVE_LATENCY: itr += ring_container->itr &,
    pub clear_counts: goto,
    }
    pub ring_container->total_bytes: bytes =,
// If packets are less than 4 or bytes are less than 9000 assume
// insufficient data to use bulk rate limiting approach. We are
// likely latency driven.
//
    if (packets < 4 && bytes < 9000) {
    pub IXGBE_ITR_ADAPTIVE_LATENCY: itr =,
    pub adjust_by_size: goto,
    }
// Between 4 and 48 we can assume that our current interrupt delay
// is only slightly too low. As such we should increase it by a small
// fixed amount.
//
    if (packets < 48) {
    pub IXGBE_ITR_ADAPTIVE_MIN_INC: itr = (q_vector->itr >> 2) +,
    if (itr > IXGBE_ITR_ADAPTIVE_MAX_USECS)
    pub IXGBE_ITR_ADAPTIVE_MAX_USECS: itr =,
    pub clear_counts: goto,
    }
// Between 48 and 96 is our "goldilocks" zone where we are working
// out "just right". Just report that our current ITR is good for us.
//
    if (packets < 96) {
    pub 2: itr = q_vector->itr >>,
    pub clear_counts: goto,
    }
// If packet count is 96 or greater we are likely looking at a slight
// overrun of the delay we want. Try halving our delay to see if that
// will cut the number of packets in half per interrupt.
//
    if (packets < 256) {
    pub 3: itr = q_vector->itr >>,
    if (itr < IXGBE_ITR_ADAPTIVE_MIN_USECS)
    pub IXGBE_ITR_ADAPTIVE_MIN_USECS: itr =,
    pub clear_counts: goto,
    }
// The paths below assume we are dealing with a bulk ITR since number
// of packets is 256 or greater. We are just going to have to compute
// a value and try to bring the count under control, though for smaller
// packet sizes there isn't much we can do as NAPI polling will likely
// be kicking in sooner rather than later.
//
    pub IXGBE_ITR_ADAPTIVE_BULK: itr =,
    adjust_by_size:
// If packet counts are 256 or greater we can assume we have a gross
// overestimation of what the rate should be. Instead of trying to fine
// tune it just use the formula below to try and dial in an exact value
// give the current packet size of the frame.
//
    pub packets: avg_wire_size = bytes /,
// The following is a crude approximation of:
// wmem_default / (size + overhead) = desired_pkts_per_int
// rate / bits_per_byte / (size + ethernet overhead) = pkt_rate
// (desired_pkt_rate / pkt_rate) * usecs_per_sec = ITR value
//
// Assuming wmem_default is 212992 and overhead is 640 bytes per
// packet, (256 skb, 64 headroom, 320 shared info), we can reduce the
// formula down to
//
// (170 * (size + 24)) / (size + 640) = ITR
//
// We first do some math on the packet size and then finally bitshift
// by 8 after rounding up. We also have to account for PCIe link speed
// difference as ITR scales based on this.
//
    if (avg_wire_size <= 60) {
// Start at 50k ints/sec
    pub 5120: avg_wire_size =,
    } else if (avg_wire_size <= 316) {
// 50K ints/sec to 16K ints/sec
    pub 40: *mut *mut avg_wire_size =,
    pub 2720: avg_wire_size +=,
    } else if (avg_wire_size <= 1084) {
// 16K ints/sec to 9.2K ints/sec
    pub 15: *mut *mut avg_wire_size =,
    pub 11452: avg_wire_size +=,
    } else if (avg_wire_size < 1968) {
// 9.2K ints/sec to 8K ints/sec
    pub 5: *mut *mut avg_wire_size =,
    pub 22420: avg_wire_size +=,
    } else {
// plateau at a limit of 8K ints/sec
    pub 32256: avg_wire_size =,
    }
// If we are in low latency mode half our delay which doubles the rate
// to somewhere between 100K to 16K ints/sec
//
    if (itr & IXGBE_ITR_ADAPTIVE_LATENCY)
    pub 1: avg_wire_size >>=,
// Resultant value is 256 times larger than it needs to be. This
// gives us room to adjust the value as needed to either increase
// or decrease the value based on link speeds of 10G, 2.5G, 1G, etc.
//
// Use addition as we have already recorded the new latency flag
// for the ITR value.
//
    switch (q_vector.adapter.link_speed) {
    case IXGBE_LINK_SPEED_10GB_FULL:
    case IXGBE_LINK_SPEED_100_FULL:
    default:
    itr += DIV_ROUND_UP(avg_wire_size,
    IXGBE_ITR_ADAPTIVE_MIN_INC * 256) *
    case IXGBE_LINK_SPEED_2_5GB_FULL:
    case IXGBE_LINK_SPEED_1GB_FULL:
    case IXGBE_LINK_SPEED_10_FULL:
    if (avg_wire_size > 8064)
    pub 8064: avg_wire_size =,
    itr += DIV_ROUND_UP(avg_wire_size,
    IXGBE_ITR_ADAPTIVE_MIN_INC * 64) *
    }
    clear_counts:
// write back value
    pub itr: ring_container->itr =,
// next update should occur within next jiffy
    pub 1: ring_container->next_update = next_update +,
    pub 0: ring_container->total_bytes =,
    pub 0: ring_container->total_packets =,
    }
//
// ixgbe_write_eitr - write EITR register in hardware specific way
// @q_vector: structure containing interrupt and ring information
//
// This function is made to be called by ethtool and by the driver
// when it needs to update EITR registers at runtime.  Hardware
// specific quirks/differences are taken care of here.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_write_eitr(q_vector: *mut ixgbe_q_vector) {
    void ixgbe_write_eitr(struct ixgbe_q_vector *q_vector)
    {
    pub q_vector->adapter: *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub q_vector->v_idx: int v_idx =,
    pub IXGBE_MAX_EITR: u32 itr_reg = q_vector->itr &,
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82598EB:
// must write high and low 16 bits to reset counter
    pub 16): itr_reg |= (itr_reg <<,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
//
// set the WDIS bit to not clear the timer bits and cause an
// immediate assertion of the interrupt
//
    pub IXGBE_EITR_CNT_WDIS: itr_reg |=,
    default:
    }
    pub itr_reg): IXGBE_WRITE_REG(hw, IXGBE_EITR(v_idx),,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_set_itr(q_vector: *mut ixgbe_q_vector) {
    static void ixgbe_set_itr(struct ixgbe_q_vector *q_vector)
    {
    pub new_itr: u32,
    pub &q_vector->tx): ixgbe_update_itr(q_vector,,
    pub &q_vector->rx): ixgbe_update_itr(q_vector,,
// use the smallest value of new ITR delay calculations
    pub q_vector->tx.itr): new_itr = min(q_vector->rx.itr,,
// Clear latency flag if set, shift into correct position
    pub ~IXGBE_ITR_ADAPTIVE_LATENCY: new_itr &=,
    pub 2: new_itr <<=,
    if (new_itr != q_vector.itr) {
// save the algorithm value here
    pub new_itr: q_vector->itr =,
    }
    }
//
// ixgbe_check_overtemp_subtask - check for over temperature
// @adapter: pointer to adapter
//
#[no_mangle]
unsafe extern "C" fn ixgbe_check_overtemp_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_check_overtemp_subtask(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub adapter->interrupt_event: u32 eicr =,
    if (test_bit(__IXGBE_DOWN, &adapter.state))
    if (!(adapter.flags2 & IXGBE_FLAG2_TEMP_SENSOR_EVENT))
    pub ~IXGBE_FLAG2_TEMP_SENSOR_EVENT: adapter->flags2 &=,
    switch (hw.device_id) {
    case IXGBE_DEV_ID_82599_T3_LOM:
//
// Since the warning interrupt is for both ports
// we don't have to check if:
// - This interrupt wasn't for our port.
// - We may have missed the interrupt so always have to
// check if we  got a LSC
//
    if (!(eicr & IXGBE_EICR_GPI_SDP0_8259X) &&
    !(eicr & IXGBE_EICR_LSC))
    if (!(eicr & IXGBE_EICR_LSC) && hw.mac.ops.check_link) {
    pub speed: u32,
    pub false: bool link_up =,
    pub false): hw->mac.ops.check_link(hw, &speed, &link_up,,
    if (link_up)
    }
// Check if this is not due to overtemp
    if (!hw.phy.ops.check_overtemp(hw))
    case IXGBE_DEV_ID_X550EM_A_1G_T:
    case IXGBE_DEV_ID_X550EM_A_1G_T_L:
    if (!hw.phy.ops.check_overtemp(hw))
    default:
    if (adapter.hw.mac.type >= ixgbe_mac_X540)
    if (!(eicr & IXGBE_EICR_GPI_SDP0(hw)))
    }
    pub ixgbe_overheat_msg): e_crit(drv, "%s\n",,
    pub 0: adapter->interrupt_event =,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_fan_failure(adapter: *mut ixgbe_adapter, eicr: u32) {
    static void ixgbe_check_fan_failure(struct ixgbe_adapter *adapter, u32 eicr)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    if ((adapter.flags & IXGBE_FLAG_FAN_FAIL_CAPABLE) &&
    (eicr & IXGBE_EICR_GPI_SDP1(hw))) {
    pub adapter\n"): e_crit(probe, "Fan has stopped, replace the,
// write to clear the interrupt
    pub IXGBE_EICR_GPI_SDP1(hw)): IXGBE_WRITE_REG(hw, IXGBE_EICR,,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_overtemp_event(adapter: *mut ixgbe_adapter, eicr: u32) {
    static void ixgbe_check_overtemp_event(struct ixgbe_adapter *adapter, u32 eicr)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    if (!(adapter.flags2 & IXGBE_FLAG2_TEMP_SENSOR_CAPABLE))
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82599EB:
//
// Need to check link state so complete overtemp check
// on service task
//
    if (((eicr & IXGBE_EICR_GPI_SDP0(hw)) ||
    (eicr & IXGBE_EICR_LSC)) &&
    (!test_bit(__IXGBE_DOWN, &adapter.state))) {
    pub eicr: adapter->interrupt_event =,
    pub IXGBE_FLAG2_TEMP_SENSOR_EVENT: adapter->flags2 |=,
    }
    case ixgbe_mac_x550em_a:
    if (eicr & IXGBE_EICR_GPI_SDP0_X550EM_a) {
    pub eicr: adapter->interrupt_event =,
    pub IXGBE_FLAG2_TEMP_SENSOR_EVENT: adapter->flags2 |=,
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_EIMC,
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_EICR,
    }
    case ixgbe_mac_X550:
    case ixgbe_mac_X540:
    if (!(eicr & IXGBE_EICR_TS))
    default:
    }
    pub ixgbe_overheat_msg): e_crit(drv, "%s\n",,
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_is_sfp(hw: *mut ixgbe_hw) -> bool {
    static inline bool ixgbe_is_sfp(struct ixgbe_hw *hw)
    {
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    if (hw.phy.type == ixgbe_phy_nl)
    pub true: return,
    pub false: return,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    switch (hw.mac.ops.get_media_type(hw)) {
    case ixgbe_media_type_fiber:
    case ixgbe_media_type_fiber_qsfp:
    pub true: return,
    default:
    pub false: return,
    }
    default:
    pub false: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_sfp_event(adapter: *mut ixgbe_adapter, eicr: u32) {
    static void ixgbe_check_sfp_event(struct ixgbe_adapter *adapter, u32 eicr)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub IXGBE_EICR_GPI_SDP2(hw): u32 eicr_mask =,
    if (!ixgbe_is_sfp(hw))
// Later MAC's use different SDP
    if (hw.mac.type >= ixgbe_mac_X540)
    pub IXGBE_EICR_GPI_SDP0_X540: eicr_mask =,
    if (eicr & eicr_mask) {
// Clear the interrupt
    pub eicr_mask): IXGBE_WRITE_REG(hw, IXGBE_EICR,,
    if (!test_bit(__IXGBE_DOWN, &adapter.state)) {
    pub IXGBE_FLAG2_SFP_NEEDS_RESET: adapter->flags2 |=,
    pub 0: adapter->sfp_poll_time =,
    }
    }
    if (adapter.hw.mac.type == ixgbe_mac_82599EB &&
    (eicr & IXGBE_EICR_GPI_SDP1(hw))) {
// Clear the interrupt
    pub IXGBE_EICR_GPI_SDP1(hw)): IXGBE_WRITE_REG(hw, IXGBE_EICR,,
    if (!test_bit(__IXGBE_DOWN, &adapter.state)) {
    pub IXGBE_FLAG_NEED_LINK_CONFIG: adapter->flags |=,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_lsc(adapter: *mut ixgbe_adapter) {
    static void ixgbe_check_lsc(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub IXGBE_FLAG_NEED_LINK_UPDATE: adapter->flags |=,
    pub jiffies: adapter->link_check_timeout =,
    if (!test_bit(__IXGBE_DOWN, &adapter.state)) {
    pub IXGBE_EIMC_LSC): IXGBE_WRITE_REG(hw, IXGBE_EIMC,,
    }
    }
//
// ixgbe_check_phy_fw_load - check if PHY FW load failed
// @adapter: pointer to adapter structure
// @link_cfg_err: bitmap from the link info structure
//
// Check if external PHY FW load failed and print an error message if it did.
//
    static void ixgbe_check_phy_fw_load(struct ixgbe_adapter *adapter,
    u8 link_cfg_err)
    {
    if (!(link_cfg_err & IXGBE_ACI_LINK_EXTERNAL_PHY_LOAD_FAILURE)) {
    pub ~IXGBE_FLAG2_PHY_FW_LOAD_FAILED: adapter->flags2 &=,
    }
    if (adapter.flags2 & IXGBE_FLAG2_PHY_FW_LOAD_FAILED)
    if (link_cfg_err & IXGBE_ACI_LINK_EXTERNAL_PHY_LOAD_FAILURE) {
    pub again\n"): netdev_err(adapter->netdev, "Device failed to load the FW for the external PHY. Please download and install the latest NVM for your device and try,
    pub IXGBE_FLAG2_PHY_FW_LOAD_FAILED: adapter->flags2 |=,
    }
    }
//
// ixgbe_check_module_power - check module power level
// @adapter: pointer to adapter structure
// @link_cfg_err: bitmap from the link info structure
//
// Check module power level returned by a previous call to aci_get_link_info
// and print error messages if module power level is not supported.
//
    static void ixgbe_check_module_power(struct ixgbe_adapter *adapter,
    u8 link_cfg_err)
    {
// If module power level is supported, clear the flag.
    if (!(link_cfg_err & (IXGBE_ACI_LINK_INVAL_MAX_POWER_LIMIT |
    IXGBE_ACI_LINK_MODULE_POWER_UNSUPPORTED))) {
    pub ~IXGBE_FLAG2_MOD_POWER_UNSUPPORTED: adapter->flags2 &=,
    }
// If IXGBE_FLAG2_MOD_POWER_UNSUPPORTED was previously set and the
// above block didn't clear this bit, there's nothing to do.
//
    if (adapter.flags2 & IXGBE_FLAG2_MOD_POWER_UNSUPPORTED)
    if (link_cfg_err & IXGBE_ACI_LINK_INVAL_MAX_POWER_LIMIT) {
    pub link.\n"): netdev_err(adapter->netdev, "The installed module is incompatible with the device's NVM image. Cannot start,
    pub IXGBE_FLAG2_MOD_POWER_UNSUPPORTED: adapter->flags2 |=,
    } else if (link_cfg_err & IXGBE_ACI_LINK_MODULE_POWER_UNSUPPORTED) {
    pub link.\n"): netdev_err(adapter->netdev, "The module's power requirements exceed the device's power supply. Cannot start,
    pub IXGBE_FLAG2_MOD_POWER_UNSUPPORTED: adapter->flags2 |=,
    }
    }
//
// ixgbe_check_link_cfg_err - check if link configuration failed
// @adapter: pointer to adapter structure
// @link_cfg_err: bitmap from the link info structure
//
// Print if any link configuration failure happens due to the value in the
// link_cfg_err parameter in the link info structure.
//
    static void ixgbe_check_link_cfg_err(struct ixgbe_adapter *adapter,
    u8 link_cfg_err)
    {
    pub link_cfg_err): ixgbe_check_module_power(adapter,,
    pub link_cfg_err): ixgbe_check_phy_fw_load(adapter,,
    }
//
// ixgbe_process_link_status_event - process the link event
// @adapter: pointer to adapter structure
// @link_up: true if the physical link is up and false if it is down
// @link_speed: current link speed received from the link event
//
// Return: 0 on success or negative value on failure.
//
    static int
    ixgbe_process_link_status_event(struct ixgbe_adapter *adapter, bool link_up,
    u16 link_speed)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub status: c_int,
// Update the link info structures and re-enable link events,
// don't bail on failure due to other book keeping needed.
//
    pub ixgbe_update_link_info(hw): status =,
    if (status)
    e_dev_err("Failed to update link status, err %d aq_err %d\n",
    pub hw->aci.last_status): status,,
    pub hw->link.link_info.link_cfg_err): ixgbe_check_link_cfg_err(adapter,,
// Check if the link state is up after updating link info, and treat
// this event as an UP event since the link is actually UP now.
//
    if (hw.link.link_info.link_info & IXGBE_ACI_LINK_UP)
    pub true: link_up =,
// Turn off PHY if media was removed.
    if (!(adapter.flags2 & IXGBE_FLAG2_NO_MEDIA) &&
    !(hw.link.link_info.link_info & IXGBE_ACI_MEDIA_AVAILABLE))
    pub IXGBE_FLAG2_NO_MEDIA: adapter->flags2 |=,
    if (link_up == adapter.link_up &&
    link_up == netif_carrier_ok(adapter.netdev) &&
    link_speed == adapter.link_speed)
    pub 0: return,
    pub IXGBE_FLAG_NEED_LINK_UPDATE: adapter->flags |=,
    pub jiffies: adapter->link_check_timeout =,
    if (link_up)
    else
    pub 0: return,
    }
//
// ixgbe_handle_link_status_event - handle link status event via ACI
// @adapter: pointer to adapter structure
// @e: event structure containing link status info
//
    static void
    ixgbe_handle_link_status_event(struct ixgbe_adapter *adapter,
    struct ixgbe_aci_event *e)
    {
    pub link_data: *mut ixgbe_aci_cmd_get_link_status_data,
    pub link_speed: u16,
    pub link_up: bool,
    pub )e->msg_buf: *mut link_data = (struct ixgbe_aci_cmd_get_link_status_data,
    pub IXGBE_ACI_LINK_UP): link_up = !!(link_data->link_info &,
    pub le16_to_cpu(link_data->link_speed): link_speed =,
    if (ixgbe_process_link_status_event(adapter, link_up, link_speed))
    pub event"): e_dev_warn("Could not process link status,
    }
//
// ixgbe_schedule_fw_event - schedule Firmware event
// @adapter: pointer to the adapter structure
//
// If the adapter is not in down, removing or resetting state,
// an event is scheduled.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_schedule_fw_event(adapter: *mut ixgbe_adapter) {
    static void ixgbe_schedule_fw_event(struct ixgbe_adapter *adapter)
    {
    if (!test_bit(__IXGBE_DOWN, &adapter.state) &&
    !test_bit(__IXGBE_REMOVING, &adapter.state) &&
    !test_bit(__IXGBE_RESETTING, &adapter.state)) {
    pub IXGBE_FLAG2_FW_ASYNC_EVENT: adapter->flags2 |=,
    }
    }
//
// ixgbe_aci_event_cleanup - release msg_buf memory
// @event: pointer to the event holding msg_buf to be released
//
// Clean memory allocated for event's msg_buf. Implements auto memory cleanup.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_aci_event_cleanup(event: *mut ixgbe_aci_event) {
    static void ixgbe_aci_event_cleanup(struct ixgbe_aci_event *event)
    {
    }
//
// ixgbe_handle_fw_event - handle Firmware event
// @adapter: pointer to the adapter structure
//
// Obtain an event from the ACI and then and then process it according to the
// type of the event and the opcode.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_handle_fw_event(adapter: *mut ixgbe_adapter) {
    static void ixgbe_handle_fw_event(struct ixgbe_adapter *adapter)
    {
    pub __cleanup(ixgbe_aci_event_cleanup): ixgbe_aci_event event,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub false: bool pending =,
    pub err: c_int,
    if (adapter.flags2 & IXGBE_FLAG2_FW_ASYNC_EVENT)
    pub ~IXGBE_FLAG2_FW_ASYNC_EVENT: adapter->flags2 &=,
    pub IXGBE_ACI_MAX_BUFFER_SIZE: event.buf_len =,
    pub GFP_KERNEL): event.msg_buf = kzalloc(event.buf_len,,
    if (!event.msg_buf)
    do {
    pub &pending): err = ixgbe_aci_get_event(hw, &event,,
    if (err)
    switch (le16_to_cpu(event.desc.opcode)) {
    case ixgbe_aci_opc_get_link_status:
    pub &event): ixgbe_handle_link_status_event(adapter,,
    case ixgbe_aci_opc_temp_tca_event:
    pub ixgbe_overheat_msg): e_crit(drv, "%s\n",,
    case libie_aqc_opc_fw_logs_event:
    libie_get_fwlog_data(&hw.fwlog, event.msg_buf,
    default:
    pub captured\n"): e_warn(hw, "unknown FW async event,
    }
    pub (pending): } while,
    }
    static inline void ixgbe_irq_enable_queues(struct ixgbe_adapter *adapter,
    u64 qmask)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub mask: u32,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub qmask): mask = (IXGBE_EIMS_RTX_QUEUE &,
    pub mask): IXGBE_WRITE_REG(hw, IXGBE_EIMS,,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pub 0xFFFFFFFF): mask = (qmask &,
    if (mask)
    pub mask): IXGBE_WRITE_REG(hw, IXGBE_EIMS_EX(0),,
    pub 32): mask = (qmask >>,
    if (mask)
    pub mask): IXGBE_WRITE_REG(hw, IXGBE_EIMS_EX(1),,
    default:
    }
// skip the flush
    }
//
// ixgbe_irq_enable - Enable default interrupt generation settings
// @adapter: board private structure
// @queues: enable irqs for queues
// @flush: flush register write
//
    static inline void ixgbe_irq_enable(struct ixgbe_adapter *adapter, bool queues,
    bool flush)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ~IXGBE_EIMS_RTX_QUEUE): u32 mask = (IXGBE_EIMS_ENABLE_MASK &,
// don't reenable LSC while waiting for link
    if (adapter.flags & IXGBE_FLAG_NEED_LINK_UPDATE)
    pub ~IXGBE_EIMS_LSC: mask &=,
    if (adapter.flags2 & IXGBE_FLAG2_TEMP_SENSOR_CAPABLE)
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82599EB:
    pub IXGBE_EIMS_GPI_SDP0(hw): mask |=,
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    pub IXGBE_EIMS_TS: mask |=,
    default:
    }
    if (adapter.flags & IXGBE_FLAG_FAN_FAIL_CAPABLE)
    pub IXGBE_EIMS_GPI_SDP1(hw): mask |=,
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82599EB:
    pub IXGBE_EIMS_GPI_SDP1(hw): mask |=,
    pub IXGBE_EIMS_GPI_SDP2(hw): mask |=,
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_e610:
    pub IXGBE_EIMS_FW_EVENT: mask |=,
    case ixgbe_mac_x550em_a:
    if (adapter.hw.device_id == IXGBE_DEV_ID_X550EM_X_SFP ||
    adapter.hw.device_id == IXGBE_DEV_ID_X550EM_A_SFP ||
    adapter.hw.device_id == IXGBE_DEV_ID_X550EM_A_SFP_N)
    pub IXGBE_EIMS_GPI_SDP0(&adapter->hw): mask |=,
    if (adapter.hw.phy.type == ixgbe_phy_x550em_ext_t)
    pub IXGBE_EICR_GPI_SDP0_X540: mask |=,
    pub IXGBE_EIMS_ECC: mask |=,
    pub IXGBE_EIMS_MAILBOX: mask |=,
    default:
    }
    if ((adapter.flags & IXGBE_FLAG_FDIR_HASH_CAPABLE) &&
    !(adapter.flags2 & IXGBE_FLAG2_FDIR_REQUIRES_REINIT))
    pub IXGBE_EIMS_FLOW_DIR: mask |=,
    pub mask): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EIMS,,
    if (queues)
    pub ~0): ixgbe_irq_enable_queues(adapter,,
    if (flush)
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_msix_other(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ixgbe_msix_other(int irq, void *data)
    {
    pub data: *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub eicr: u32,
//
// Workaround for Silicon errata.  Use clear-by-write instead
// of clear-by-read.  Reading with EICS will return the
// interrupt causes without clearing, which later be done
// with the write to EICR.
//
    pub IXGBE_EICS): eicr = IXGBE_READ_REG(hw,,
// The lower 16bits of the EICR register are for the queue interrupts
// which should be masked here in order to not accidentally clear them if
// the bits are high when ixgbe_msix_other is called. There is a race
// condition otherwise which results in possible performance loss
// especially if the ixgbe_msix_other interrupt is triggering
// consistently (as it would when PPS is turned on for the X540 device)
//
    pub 0xFFFF0000: eicr &=,
    pub eicr): IXGBE_WRITE_REG(hw, IXGBE_EICR,,
    if (eicr & IXGBE_EICR_LSC)
    if (eicr & IXGBE_EICR_MAILBOX)
    if (eicr & IXGBE_EICR_FW_EVENT)
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    if (hw.phy.type == ixgbe_phy_x550em_ext_t &&
    (eicr & IXGBE_EICR_GPI_SDP0_X540)) {
    pub IXGBE_FLAG2_PHY_INTERRUPT: adapter->flags2 |=,
    IXGBE_WRITE_REG(hw, IXGBE_EICR,
    }
    if (eicr & IXGBE_EICR_ECC) {
    pub reset\n"): e_info(link, "Received ECC Err, initiating,
    pub &adapter->state): set_bit(__IXGBE_RESET_REQUESTED,,
    pub IXGBE_EICR_ECC): IXGBE_WRITE_REG(hw, IXGBE_EICR,,
    }
// Handle Flow Director Full threshold interrupt
    if (eicr & IXGBE_EICR_FLOW_DIR) {
    pub 0: int reinit_count =,
    pub i: c_int,
    pub {: for (i = 0; i < adapter->num_tx_queues; i++),
    pub adapter->tx_ring[i]: *mut *mut ixgbe_ring ring =,
    if (test_and_clear_bit(__IXGBE_TX_FDIR_INIT_DONE,
    ring.state))
    }
    if (reinit_count) {
// no more flow director interrupts until after init
    pub IXGBE_EIMC_FLOW_DIR): IXGBE_WRITE_REG(hw, IXGBE_EIMC,,
    pub IXGBE_FLAG2_FDIR_REQUIRES_REINIT: adapter->flags2 |=,
    }
    }
    pub eicr): ixgbe_check_sfp_event(adapter,,
    pub eicr): ixgbe_check_overtemp_event(adapter,,
    default:
    }
    pub eicr): ixgbe_check_fan_failure(adapter,,
    if (unlikely(eicr & IXGBE_EICR_TIMESYNC))
// re-enable the original interrupt state, no lsc, no queues
    if (!test_bit(__IXGBE_DOWN, &adapter.state))
    pub false): ixgbe_irq_enable(adapter, false,,
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_msix_clean_rings(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ixgbe_msix_clean_rings(int irq, void *data)
    {
    pub data: *mut *mut ixgbe_q_vector q_vector =,
// EIAM disabled interrupts (on this vector) for us
    if (q_vector.rx.ring || q_vector.tx.ring)
    pub IRQ_HANDLED: return,
    }
//
// ixgbe_poll - NAPI Rx polling callback
// @napi: structure for representing this polling device
// @budget: how many packets driver is allowed to clean
//
// This function is used for legacy and MSI, NAPI mode
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    int ixgbe_poll(struct napi_struct *napi, int budget)
    {
    struct ixgbe_q_vector *q_vector =
    pub napi): container_of(napi, struct ixgbe_q_vector,,
    pub q_vector->adapter: *mut *mut ixgbe_adapter adapter =,
    pub ring: *mut ixgbe_ring,
    pub 0: int per_ring_budget, work_done =,
    pub true: bool clean_complete =,

    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED)

    ixgbe_for_each_ring(ring, q_vector.tx) {
    bool wd = ring.xsk_pool ?
    ixgbe_clean_xdp_tx_irq(q_vector, ring, budget) :
    pub budget): ixgbe_clean_tx_irq(q_vector, ring,,
    if (!wd)
    pub false: clean_complete =,
    }
// Exit if we are called by netpoll
    if (budget <= 0)
    pub budget: return,
// attempt to distribute budget to each queue fairly, but don't allow
// the budget to go below 1 because we'll exit polling
    if (q_vector.rx.count > 1)
    pub 1): per_ring_budget = max(budget/q_vector->rx.count,,
    else
    pub budget: per_ring_budget =,
    ixgbe_for_each_ring(ring, q_vector.rx) {
    int cleaned = ring.xsk_pool ?
    ixgbe_clean_rx_irq_zc(q_vector, ring,
    per_ring_budget) :
    ixgbe_clean_rx_irq(q_vector, ring,
    pub cleaned: work_done +=,
    if (cleaned >= per_ring_budget)
    pub false: clean_complete =,
    }
// If all work not completed, return budget and keep polling
    if (!clean_complete)
    pub budget: return,
// all work done, exit the polling mode
    if (likely(napi_complete_done(napi, work_done))) {
    if (adapter.rx_itr_setting & 1)
    if (!test_bit(__IXGBE_DOWN, &adapter.state))
    ixgbe_irq_enable_queues(adapter,
    }
    pub 1): return min(work_done, budget -,
    }
//
// ixgbe_request_msix_irqs - Initialize MSI-X interrupts
// @adapter: board private structure
//
// ixgbe_request_msix_irqs allocates MSI-X vectors and requests
// interrupts from the kernel.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_request_msix_irqs(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_request_msix_irqs(struct ixgbe_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub 0: unsigned int ri = 0, ti =,
    pub err: int vector,,
    pub {: for (vector = 0; vector < adapter->num_q_vectors; vector++),
    pub adapter->q_vector[vector]: *mut *mut ixgbe_q_vector q_vector =,
    pub &adapter->msix_entries[vector]: *mut *mut msix_entry entry =,
    if (q_vector.tx.ring && q_vector.rx.ring) {
    snprintf(q_vector.name, sizeof(q_vector.name),
    pub ri++): "%s-TxRx-%u", netdev->name,,
    } else if (q_vector.rx.ring) {
    snprintf(q_vector.name, sizeof(q_vector.name),
    pub ri++): "%s-rx-%u", netdev->name,,
    } else if (q_vector.tx.ring) {
    snprintf(q_vector.name, sizeof(q_vector.name),
    pub ti++): "%s-tx-%u", netdev->name,,
    } else {
// skip this unused q_vector
    }
    err = request_irq(entry.vector, &ixgbe_msix_clean_rings, 0,
    pub q_vector): q_vector->name,,
    if (err) {
    e_err(probe, "request_irq failed for MSIX interrupt "
    pub err): "Error: %d\n",,
    pub free_queue_irqs: goto,
    }
// If Flow Director is enabled, set interrupt affinity
    if (adapter.flags & IXGBE_FLAG_FDIR_HASH_CAPABLE) {
// assign the mask for this irq
    irq_update_affinity_hint(entry.vector,
    }
    }
    err = request_irq(adapter.msix_entries[vector].vector,
    pub adapter): ixgbe_msix_other, 0, netdev->name,,
    if (err) {
    pub err): e_err(probe, "request_irq for msix_other failed: %d\n",,
    pub free_queue_irqs: goto,
    }
    pub 0: return,
    free_queue_irqs:
    while (vector) {
    irq_update_affinity_hint(adapter.msix_entries[vector].vector,
    free_irq(adapter.msix_entries[vector].vector,
    }
    pub ~IXGBE_FLAG_MSIX_ENABLED: adapter->flags &=,
    pub NULL: adapter->msix_entries =,
    pub err: return,
    }
//
// ixgbe_intr - legacy mode Interrupt Handler
// @irq: interrupt number
// @data: pointer to a network interface device structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_intr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ixgbe_intr(int irq, void *data)
    {
    pub data: *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub adapter->q_vector[0]: *mut *mut ixgbe_q_vector q_vector =,
    pub eicr: u32,
//
// Workaround for silicon errata #26 on 82598.  Mask the interrupt
// before the read of EICR.
//
    pub IXGBE_IRQ_CLEAR_MASK): IXGBE_WRITE_REG(hw, IXGBE_EIMC,,
// for NAPI, using EIAM to auto-mask tx/rx interrupt bits on read
// therefore no explicit interrupt disable is necessary
    pub IXGBE_EICR): eicr = IXGBE_READ_REG(hw,,
    if (!eicr) {
//
// shared interrupt alert!
// make sure interrupts are enabled because the read will
// have disabled interrupts due to EIAM
// finish the workaround of silicon errata on 82598.  Unmask
// the interrupt that we masked before the EICR read.
//
    if (!test_bit(__IXGBE_DOWN, &adapter.state))
    pub true): ixgbe_irq_enable(adapter, true,,
    pub /: *mut *mut return IRQ_NONE; / Not our interrupt,
    }
    if (eicr & IXGBE_EICR_LSC)
    if (eicr & IXGBE_EICR_FW_EVENT)
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    pub eicr): ixgbe_check_sfp_event(adapter,,
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    if (eicr & IXGBE_EICR_ECC) {
    pub reset\n"): e_info(link, "Received ECC Err, initiating,
    pub &adapter->state): set_bit(__IXGBE_RESET_REQUESTED,,
    pub IXGBE_EICR_ECC): IXGBE_WRITE_REG(hw, IXGBE_EICR,,
    }
    pub eicr): ixgbe_check_overtemp_event(adapter,,
    default:
    }
    pub eicr): ixgbe_check_fan_failure(adapter,,
    if (unlikely(eicr & IXGBE_EICR_TIMESYNC))
// would disable interrupts here but EIAM disabled it
//
// re-enable link(maybe) and non-queue interrupts, no flush.
// ixgbe_poll will re-enable the queue interrupts
//
    if (!test_bit(__IXGBE_DOWN, &adapter.state))
    pub false): ixgbe_irq_enable(adapter, false,,
    pub IRQ_HANDLED: return,
    }
//
// ixgbe_request_irq - initialize interrupts
// @adapter: board private structure
//
// Attempts to configure interrupts using the best available
// capabilities of the hardware and kernel.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_request_irq(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_request_irq(struct ixgbe_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub err: c_int,
    if (adapter.flags & IXGBE_FLAG_MSIX_ENABLED)
    pub ixgbe_request_msix_irqs(adapter): err =,
#[no_mangle]
pub unsafe extern "C" fn if(IXGBE_FLAG_MSI_ENABLED: adapter->flags &) -> else {
    else if (adapter.flags & IXGBE_FLAG_MSI_ENABLED)
    err = request_irq(adapter.pdev.irq, ixgbe_intr, 0,
    pub adapter): netdev->name,,
    else
    err = request_irq(adapter.pdev.irq, ixgbe_intr, IRQF_SHARED,
    pub adapter): netdev->name,,
    if (err)
    pub err): e_err(probe, "request_irq failed, Error %d\n",,
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_free_irq(adapter: *mut ixgbe_adapter) {
    static void ixgbe_free_irq(struct ixgbe_adapter *adapter)
    {
    pub vector: c_int,
    if (!(adapter.flags & IXGBE_FLAG_MSIX_ENABLED)) {
    pub adapter): free_irq(adapter->pdev->irq,,
    }
    if (!adapter.msix_entries)
    pub {: for (vector = 0; vector < adapter->num_q_vectors; vector++),
    pub adapter->q_vector[vector]: *mut *mut ixgbe_q_vector q_vector =,
    pub &adapter->msix_entries[vector]: *mut *mut msix_entry entry =,
// free only the irqs that were actually requested
    if (!q_vector.rx.ring && !q_vector.tx.ring)
// clear the affinity_mask in the IRQ descriptor
    pub NULL): irq_update_affinity_hint(entry->vector,,
    pub q_vector): free_irq(entry->vector,,
    }
    pub adapter): free_irq(adapter->msix_entries[vector].vector,,
    }
//
// ixgbe_irq_disable - Mask off interrupt generation on the NIC
// @adapter: board private structure
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_irq_disable(adapter: *mut ixgbe_adapter) {
    static inline void ixgbe_irq_disable(struct ixgbe_adapter *adapter)
    {
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub ~0): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EIMC,,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pub 0xFFFF0000): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EIMC,,
    pub ~0): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EIMC_EX(0),,
    pub ~0): IXGBE_WRITE_REG(&adapter->hw, IXGBE_EIMC_EX(1),,
    default:
    }
    if (adapter.flags & IXGBE_FLAG_MSIX_ENABLED) {
    pub vector: c_int,
    pub vector++): for (vector = 0; vector < adapter->num_q_vectors;,
    } else {
    }
    }
//
// ixgbe_configure_msi_and_legacy - Initialize PIN (INTA...) and MSI interrupts
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_msi_and_legacy(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_msi_and_legacy(struct ixgbe_adapter *adapter)
    {
    pub adapter->q_vector[0]: *mut *mut ixgbe_q_vector q_vector =,
    pub 0): ixgbe_set_ivar(adapter, 0, 0,,
    pub 0): ixgbe_set_ivar(adapter, 1, 0,,
    pub done\n"): e_info(hw, "Legacy interrupt IVAR setup,
    }
//
// ixgbe_configure_tx_ring - Configure 8259x Tx ring after Reset
// @adapter: board private structure
// @ring: structure containing ring specific data
//
// Configure the Tx descriptor ring after a reset.
//
    void ixgbe_configure_tx_ring(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ring->dma: u64 tdba =,
    pub 10: int wait_loop =,
    pub IXGBE_TXDCTL_ENABLE: u32 txdctl =,
    pub ring->reg_idx: u8 reg_idx =,
    pub NULL: ring->xsk_pool =,
    if (ring_is_xdp(ring))
    pub ring): ring->xsk_pool = ixgbe_xsk_pool(adapter,,
// disable queue to avoid issues while updating state
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_TXDCTL(reg_idx),,
    IXGBE_WRITE_REG(hw, IXGBE_TDBAL(reg_idx),
    pub DMA_BIT_MASK(32))): (tdba &,
    pub 32)): IXGBE_WRITE_REG(hw, IXGBE_TDBAH(reg_idx), (tdba >>,
    IXGBE_WRITE_REG(hw, IXGBE_TDLEN(reg_idx),
    pub ixgbe_adv_tx_desc)): *mut *mut ring->count  sizeof(union,
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_TDH(reg_idx),,
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_TDT(reg_idx),,
    pub IXGBE_TDT(reg_idx): ring->tail = adapter->io_addr +,
//
// set WTHRESH to encourage burst writeback, it should not be set
// higher than 1 when:
// - ITR is 0 as it could cause false TX hangs
// - ITR is set to > 100k int/sec and BQL is enabled
//
// In order to avoid issues WTHRESH + PTHRESH should always be equal
// to or less than the number of on chip descriptors, which is
// currently 40.
//
    if (!ring.q_vector || (ring.q_vector.itr < IXGBE_100K_ITR))
    pub /: *mut *mut txdctl |= 1u << 16; / WTHRESH = 1,
    else
    pub /: *mut *mut txdctl |= 8u << 16; / WTHRESH = 8,
//
// Setting PTHRESH to 32 both improves performance
// and avoids a TX hang with DFP enabled
//
    txdctl |= (1u << 8) |	/* HTHRESH = 1 */
    pub /: *mut *mut 32; / PTHRESH = 32,
// reinitialize flowdirector state
    if (adapter.flags & IXGBE_FLAG_FDIR_HASH_CAPABLE) {
    pub adapter->atr_sample_rate: ring->atr_sample_rate =,
    pub 0: ring->atr_count =,
    pub ring->state): set_bit(__IXGBE_TX_FDIR_INIT_DONE,,
    } else {
    pub 0: ring->atr_sample_rate =,
    }
// initialize XPS
    if (!ring_is_xdp(ring) &&
    !test_and_set_bit(__IXGBE_TX_XPS_INIT_DONE, ring.state)) {
    pub ring->q_vector: *mut *mut ixgbe_q_vector q_vector =,
    if (q_vector)
    netif_set_xps_queue(ring.netdev,
    &q_vector.affinity_mask,
    }
    pub ring->state): clear_bit(__IXGBE_HANG_CHECK_ARMED,,
// reinitialize tx_buffer_info
    memset(ring.tx_buffer_info, 0,
    pub ring->count): *mut *mut sizeof(struct ixgbe_tx_buffer),
// enable queue
    pub txdctl): IXGBE_WRITE_REG(hw, IXGBE_TXDCTL(reg_idx),,
// TXDCTL.EN will return 0 on 82598 if link is down, so skip it
    if (hw.mac.type == ixgbe_mac_82598EB &&
    !(IXGBE_READ_REG(hw, IXGBE_LINKS) & IXGBE_LINKS_UP))
// poll to verify queue is enabled
    do {
    pub 2000): usleep_range(1000,,
    pub IXGBE_TXDCTL(reg_idx)): txdctl = IXGBE_READ_REG(hw,,
    pub IXGBE_TXDCTL_ENABLE)): } while (--wait_loop && !(txdctl &,
    if (!wait_loop)
    pub reg_idx): hw_dbg(hw, "Could not enable Tx Queue %d\n",,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_mtqc(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_mtqc(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub mtqc: u32 rttdcs,,
    pub adapter->hw_tcs: u8 tcs =,
    if (hw.mac.type == ixgbe_mac_82598EB)
// disable the arbiter while setting MTQC
    pub IXGBE_RTTDCS): rttdcs = IXGBE_READ_REG(hw,,
    pub IXGBE_RTTDCS_ARBDIS: rttdcs |=,
    pub rttdcs): IXGBE_WRITE_REG(hw, IXGBE_RTTDCS,,
// set transmit pool layout
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED) {
    pub IXGBE_MTQC_VT_ENA: mtqc =,
    if (tcs > 4)
    pub IXGBE_MTQC_8TC_8TQ: mtqc |= IXGBE_MTQC_RT_ENA |,
#[no_mangle]
pub unsafe extern "C" fn if(1: tcs >) -> else {
    else if (tcs > 1)
    pub IXGBE_MTQC_4TC_4TQ: mtqc |= IXGBE_MTQC_RT_ENA |,
    else if (adapter.ring_feature[RING_F_VMDQ].mask ==
    IXGBE_82599_VMDQ_4Q_MASK)
    pub IXGBE_MTQC_32VF: mtqc |=,
    else
    pub IXGBE_MTQC_64VF: mtqc |=,
    } else {
    if (tcs > 4) {
    pub IXGBE_MTQC_8TC_8TQ: mtqc = IXGBE_MTQC_RT_ENA |,
    } else if (tcs > 1) {
    pub IXGBE_MTQC_4TC_4TQ: mtqc = IXGBE_MTQC_RT_ENA |,
    } else {
    u8 max_txq = adapter.num_tx_queues +
    if (max_txq > 63)
    pub IXGBE_MTQC_4TC_4TQ: mtqc = IXGBE_MTQC_RT_ENA |,
    else
    pub IXGBE_MTQC_64Q_1PB: mtqc =,
    }
    }
    pub mtqc): IXGBE_WRITE_REG(hw, IXGBE_MTQC,,
// Enable Security TX Buffer IFG for multiple pb
    if (tcs) {
    pub IXGBE_SECTXMINIFG): u32 sectx = IXGBE_READ_REG(hw,,
    pub IXGBE_SECTX_DCB: sectx |=,
    pub sectx): IXGBE_WRITE_REG(hw, IXGBE_SECTXMINIFG,,
    }
// re-enable the arbiter
    pub ~IXGBE_RTTDCS_ARBDIS: rttdcs &=,
    pub rttdcs): IXGBE_WRITE_REG(hw, IXGBE_RTTDCS,,
    }
//
// ixgbe_configure_tx - Configure 8259x Transmit Unit after Reset
// @adapter: board private structure
//
// Configure the Tx unit of the MAC after a reset.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_tx(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_tx(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub dmatxctl: u32,
    pub i: u32,
    if (hw.mac.type != ixgbe_mac_82598EB) {
// DMATXCTL.EN must be before Tx queues are enabled
    pub IXGBE_DMATXCTL): dmatxctl = IXGBE_READ_REG(hw,,
    pub IXGBE_DMATXCTL_TE: dmatxctl |=,
    pub dmatxctl): IXGBE_WRITE_REG(hw, IXGBE_DMATXCTL,,
    }
// Setup the HW Tx Head and Tail descriptor pointers
    pub i++): for (i = 0; i < adapter->num_tx_queues;,
    pub adapter->tx_ring[i]): ixgbe_configure_tx_ring(adapter,,
    pub i++): for (i = 0; i < adapter->num_xdp_queues;,
    pub adapter->xdp_ring[i]): ixgbe_configure_tx_ring(adapter,,
    }
    static void ixgbe_enable_rx_drop(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ring->reg_idx: u8 reg_idx =,
    pub IXGBE_SRRCTL(reg_idx)): u32 srrctl = IXGBE_READ_REG(hw,,
    pub IXGBE_SRRCTL_DROP_EN: srrctl |=,
    pub srrctl): IXGBE_WRITE_REG(hw, IXGBE_SRRCTL(reg_idx),,
    }
    static void ixgbe_disable_rx_drop(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ring->reg_idx: u8 reg_idx =,
    pub IXGBE_SRRCTL(reg_idx)): u32 srrctl = IXGBE_READ_REG(hw,,
    pub ~IXGBE_SRRCTL_DROP_EN: srrctl &=,
    pub srrctl): IXGBE_WRITE_REG(hw, IXGBE_SRRCTL(reg_idx),,
    }

#[no_mangle]
pub unsafe extern "C" fn ixgbe_set_rx_drop_en(adapter: *mut ixgbe_adapter) {
    void ixgbe_set_rx_drop_en(struct ixgbe_adapter *adapter)

#[no_mangle]
unsafe extern "C" fn ixgbe_set_rx_drop_en(adapter: *mut ixgbe_adapter) {
    static void ixgbe_set_rx_drop_en(struct ixgbe_adapter *adapter)

    {
    pub adapter->dcb_cfg.pfc_mode_enable: bool pfc_en =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    if (hw.mac.ops.disable_mdd)
    if (adapter.ixgbe_ieee_pfc)
    pub !!(adapter->ixgbe_ieee_pfc->pfc_en): pfc_en |=,
//
// We should set the drop enable bit if:
// SR-IOV is enabled
// or
// Number of Rx queues > 1 and flow control is disabled
//
// This allows us to avoid head of line blocking for security
// and performance reasons.
//
    if (adapter.num_vfs || (adapter.num_rx_queues > 1 &&
    !(adapter.hw.fc.current_mode & ixgbe_fc_tx_pause) && !pfc_en)) {
    pub i++): for (i = 0; i < adapter->num_rx_queues;,
    pub adapter->rx_ring[i]): ixgbe_enable_rx_drop(adapter,,
    } else {
    pub i++): for (i = 0; i < adapter->num_rx_queues;,
    pub adapter->rx_ring[i]): ixgbe_disable_rx_drop(adapter,,
    }
    if (hw.mac.ops.enable_mdd)
    }
pub const IXGBE_SRRCTL_BSIZEHDRSIZE_SHIFT: c_int = 2;
    static void ixgbe_configure_srrctl(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *rx_ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub srrctl: u32,
    pub rx_ring->reg_idx: u8 reg_idx =,
    if (hw.mac.type == ixgbe_mac_82598EB) {
    pub adapter->ring_feature[RING_F_RSS].mask: u16 mask =,
//
// if VMDq is not active we must program one srrctl register
// per RSS queue since we have enabled RDRXCTL.MVMEN
//
    pub mask: reg_idx &=,
    }
// configure header buffer length, needed for RSC
    pub IXGBE_SRRCTL_BSIZEHDRSIZE_SHIFT: srrctl = IXGBE_RX_HDR_SIZE <<,
// configure the packet buffer length
    if (rx_ring.xsk_pool) {
    pub xsk_pool_get_rx_frame_size(rx_ring->xsk_pool): u32 xsk_buf_len =,
// If the MAC support setting RXDCTL.RLPML, the
// SRRCTL[n].BSIZEPKT is set to PAGE_SIZE and
// RXDCTL.RLPML is set to the actual UMEM buffer
// size. If not, then we are stuck with a 1k buffer
// size resolution. In this case frames larger than
// the UMEM buffer size viewed in a 1k resolution will
// be dropped.
//
    if (hw.mac.type != ixgbe_mac_82599EB)
    pub IXGBE_SRRCTL_BSIZEPKT_SHIFT: srrctl |= PAGE_SIZE >>,
    else
    pub IXGBE_SRRCTL_BSIZEPKT_SHIFT: srrctl |= xsk_buf_len >>,
    } else if (test_bit(__IXGBE_RX_3K_BUFFER, rx_ring.state)) {
    pub IXGBE_SRRCTL_BSIZEPKT_SHIFT: srrctl |= IXGBE_RXBUFFER_3K >>,
    } else {
    pub IXGBE_SRRCTL_BSIZEPKT_SHIFT: srrctl |= IXGBE_RXBUFFER_2K >>,
    }
// configure descriptor type
    pub IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF: srrctl |=,
    pub srrctl): IXGBE_WRITE_REG(hw, IXGBE_SRRCTL(reg_idx),,
    }
//
// ixgbe_rss_indir_tbl_entries - Return RSS indirection table entries
// @adapter: device handle
//
// - 82598/82599/X540:     128
// - X550(non-SRIOV mode): 512
// - X550(SRIOV mode):     64
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_rss_indir_tbl_entries(adapter: *mut ixgbe_adapter) -> u32 {
    u32 ixgbe_rss_indir_tbl_entries(struct ixgbe_adapter *adapter)
    {
    if (adapter.hw.mac.type < ixgbe_mac_X550)
    pub 128: return,
#[no_mangle]
pub unsafe extern "C" fn if(IXGBE_FLAG_SRIOV_ENABLED: adapter->flags &) -> else {
    else if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED)
    pub 64: return,
    else
    pub 512: return,
    }
//
// ixgbe_store_key - Write the RSS key to HW
// @adapter: device handle
//
// Write the RSS key stored in adapter.rss_key to HW.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_store_key(adapter: *mut ixgbe_adapter) {
    void ixgbe_store_key(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    pub i++): for (i = 0; i < 10;,
    pub adapter->rss_key[i]): IXGBE_WRITE_REG(hw, IXGBE_RSSRK(i),,
    }
//
// ixgbe_init_rss_key - Initialize adapter RSS key
// @adapter: device handle
//
// Allocates and initializes the RSS key if it is not allocated.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_init_rss_key(adapter: *mut ixgbe_adapter) -> c_int {
    static inline int ixgbe_init_rss_key(struct ixgbe_adapter *adapter)
    {
    pub rss_key: *mut u32,
    if (!adapter.rss_key) {
    pub GFP_KERNEL): rss_key = kzalloc(IXGBE_RSS_KEY_SIZE,,
    if (unlikely(!rss_key))
    pub -ENOMEM: return,
    pub IXGBE_RSS_KEY_SIZE): netdev_rss_key_fill(rss_key,,
    pub rss_key: adapter->rss_key =,
    }
    pub 0: return,
    }
//
// ixgbe_store_reta - Write the RETA table to HW
// @adapter: device handle
//
// Write the RSS redirection table stored in adapter.rss_indir_tbl[] to HW.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_store_reta(adapter: *mut ixgbe_adapter) {
    void ixgbe_store_reta(struct ixgbe_adapter *adapter)
    {
    pub ixgbe_rss_indir_tbl_entries(adapter): u32 i, reta_entries =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub 0: u32 reta =,
    pub indices_multi: u32,
    pub adapter->rss_indir_tbl: *mut *mut u8 indir_tbl =,
// Fill out the redirection table as follows:
// - 82598:      8 bit wide entries containing pair of 4 bit RSS
// indices.
// - 82599/X540: 8 bit wide entries containing 4 bit RSS index
// - X550:       8 bit wide entries containing 6 bit RSS index
//
    if (adapter.hw.mac.type == ixgbe_mac_82598EB)
    pub 0x11: indices_multi =,
    else
    pub 0x1: indices_multi =,
// Write redirection table to HW
    pub {: for (i = 0; i < reta_entries; i++),
    pub 8: *mut *mut *mut reta |= indices_multi  indir_tbl[i] << (i & 0x3),
    if ((i & 3) == 3) {
    if (i < 128)
    pub reta): IXGBE_WRITE_REG(hw, IXGBE_RETA(i >> 2),,
    else
    IXGBE_WRITE_REG(hw, IXGBE_ERETA((i >> 2) - 32),
    pub 0: reta =,
    }
    }
    }
//
// ixgbe_store_vfreta - Write the RETA table to HW (x550 devices in SRIOV mode)
// @adapter: device handle
//
// Write the RSS redirection table stored in adapter.rss_indir_tbl[] to HW.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_store_vfreta(adapter: *mut ixgbe_adapter) {
    static void ixgbe_store_vfreta(struct ixgbe_adapter *adapter)
    {
    pub ixgbe_rss_indir_tbl_entries(adapter): u32 i, reta_entries =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub 0: u32 vfreta =,
// Write redirection table to HW
    pub {: for (i = 0; i < reta_entries; i++),
    pub adapter->num_rx_pools: u16 pool =,
    pub 8: *mut *mut vfreta |= (u32)adapter->rss_indir_tbl[i] << (i & 0x3),
    if ((i & 3) != 3)
    while (pool--)
    IXGBE_WRITE_REG(hw,
    IXGBE_PFVFRETA(i >> 2, VMDQ_P(pool)),
    pub 0: vfreta =,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_reta(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_reta(struct ixgbe_adapter *adapter)
    {
    pub j: u32 i,,
    pub ixgbe_rss_indir_tbl_entries(adapter): u32 reta_entries =,
    pub adapter->ring_feature[RING_F_RSS].indices: u16 rss_i =,
// Program table for at least 4 queues w/ SR-IOV so that VFs can
// make full use of any rings they may have.  We will use the
// PSRTYPE register to control how many rings we use within the PF.
//
    if ((adapter.flags & IXGBE_FLAG_SRIOV_ENABLED) && (rss_i < 4))
    pub 4: rss_i =,
// Fill out hash function seeds
// Fill out redirection table
    pub sizeof(adapter->rss_indir_tbl)): memset(adapter->rss_indir_tbl, 0,,
    pub {: for (i = 0, j = 0; i < reta_entries; i++, j++),
    if (j == rss_i)
    pub 0: j =,
    pub j: adapter->rss_indir_tbl[i] =,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_vfreta(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_vfreta(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub adapter->ring_feature[RING_F_RSS].indices: u16 rss_i =,
    pub j: int i,,
// Fill out hash function seeds
    pub {: for (i = 0; i < 10; i++),
    pub adapter->num_rx_pools: u16 pool =,
    while (pool--)
    IXGBE_WRITE_REG(hw,
    IXGBE_PFVFRSSRK(i, VMDQ_P(pool)),
// (adapter->rss_key + i));
    }
// Fill out the redirection table
    pub {: for (i = 0, j = 0; i < 64; i++, j++),
    if (j == rss_i)
    pub 0: j =,
    pub j: adapter->rss_indir_tbl[i] =,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_mrqc(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_mrqc(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub 0: u32 mrqc = 0, rss_field = 0, vfmrqc =,
    pub rxcsum: u32,
// Disable indicating checksum in descriptor, enables RSS hash
    pub IXGBE_RXCSUM): rxcsum = IXGBE_READ_REG(hw,,
    pub IXGBE_RXCSUM_PCSD: rxcsum |=,
    pub rxcsum): IXGBE_WRITE_REG(hw, IXGBE_RXCSUM,,
    if (adapter.hw.mac.type == ixgbe_mac_82598EB) {
    if (adapter.ring_feature[RING_F_RSS].mask)
    pub IXGBE_MRQC_RSSEN: mrqc =,
    } else {
    pub adapter->hw_tcs: u8 tcs =,
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED) {
    if (tcs > 4)
    pub /: *mut *mut mrqc = IXGBE_MRQC_VMDQRT8TCEN; / 8 TCs,
#[no_mangle]
pub unsafe extern "C" fn if(1: tcs >) -> else {
    else if (tcs > 1)
    pub /: *mut *mut mrqc = IXGBE_MRQC_VMDQRT4TCEN; / 4 TCs,
    else if (adapter.ring_feature[RING_F_VMDQ].mask ==
    IXGBE_82599_VMDQ_4Q_MASK)
    pub IXGBE_MRQC_VMDQRSS32EN: mrqc =,
    else
    pub IXGBE_MRQC_VMDQRSS64EN: mrqc =,
// Enable L3/L4 for Tx Switched packets only for X550,
// older devices do not support this feature
//
    if (hw.mac.type >= ixgbe_mac_X550)
    pub IXGBE_MRQC_L3L4TXSWEN: mrqc |=,
    } else {
    if (tcs > 4)
    pub IXGBE_MRQC_RTRSS8TCEN: mrqc =,
#[no_mangle]
pub unsafe extern "C" fn if(1: tcs >) -> else {
    else if (tcs > 1)
    pub IXGBE_MRQC_RTRSS4TCEN: mrqc =,
    else
    pub IXGBE_MRQC_RSSEN: mrqc =,
    }
    }
// Perform hash on these packet types
    rss_field |= IXGBE_MRQC_RSS_FIELD_IPV4 |
    IXGBE_MRQC_RSS_FIELD_IPV4_TCP |
    IXGBE_MRQC_RSS_FIELD_IPV6 |
    if (adapter.flags2 & IXGBE_FLAG2_RSS_FIELD_IPV4_UDP)
    pub IXGBE_MRQC_RSS_FIELD_IPV4_UDP: rss_field |=,
    if (adapter.flags2 & IXGBE_FLAG2_RSS_FIELD_IPV6_UDP)
    pub IXGBE_MRQC_RSS_FIELD_IPV6_UDP: rss_field |=,
    if ((hw.mac.type >= ixgbe_mac_X550) &&
    (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED)) {
    pub adapter->num_rx_pools: u16 pool =,
// Enable VF RSS mode
    pub IXGBE_MRQC_MULTIPLE_RSS: mrqc |=,
    pub mrqc): IXGBE_WRITE_REG(hw, IXGBE_MRQC,,
// Setup RSS through the VF registers
    pub IXGBE_MRQC_RSSEN: vfmrqc =,
    pub rss_field: vfmrqc |=,
    while (pool--)
    IXGBE_WRITE_REG(hw,
    IXGBE_PFVFMRQC(VMDQ_P(pool)),
    } else {
    pub rss_field: mrqc |=,
    pub mrqc): IXGBE_WRITE_REG(hw, IXGBE_MRQC,,
    }
    }
//
// ixgbe_configure_rscctl - enable RSC for the indicated ring
// @adapter: address of board private structure
// @ring: structure containing ring specific data
//
    static void ixgbe_configure_rscctl(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub rscctrl: u32,
    pub ring->reg_idx: u8 reg_idx =,
    if (!ring_is_rsc_enabled(ring))
    pub IXGBE_RSCCTL(reg_idx)): rscctrl = IXGBE_READ_REG(hw,,
    pub IXGBE_RSCCTL_RSCEN: rscctrl |=,
//
// we must limit the number of descriptors so that the
// total size of max desc * buf_len is not greater
// than 65536
//
    pub IXGBE_RSCCTL_MAXDESC_16: rscctrl |=,
    pub rscctrl): IXGBE_WRITE_REG(hw, IXGBE_RSCCTL(reg_idx),,
    }
pub const IXGBE_MAX_RX_DESC_POLL: c_int = 10;
    static void ixgbe_rx_desc_queue_enable(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub IXGBE_MAX_RX_DESC_POLL: int wait_loop =,
    pub rxdctl: u32,
    pub ring->reg_idx: u8 reg_idx =,
    if (ixgbe_removed(hw.hw_addr))
// RXDCTL.EN will return 0 on 82598 if link is down, so skip it
    if (hw.mac.type == ixgbe_mac_82598EB &&
    !(IXGBE_READ_REG(hw, IXGBE_LINKS) & IXGBE_LINKS_UP))
    do {
    pub 2000): usleep_range(1000,,
    pub IXGBE_RXDCTL(reg_idx)): rxdctl = IXGBE_READ_REG(hw,,
    pub IXGBE_RXDCTL_ENABLE)): } while (--wait_loop && !(rxdctl &,
    if (!wait_loop) {
    e_err(drv, "RXDCTL.ENABLE on Rx queue %d not set within "
    pub reg_idx): "the polling period\n",,
    }
    }
    void ixgbe_configure_rx_ring(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *ring)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub rx_desc: *mut union ixgbe_adv_rx_desc,
    pub ring->dma: u64 rdba =,
    pub rxdctl: u32,
    pub ring->reg_idx: u8 reg_idx =,
    pub ring): ring->xsk_pool = ixgbe_xsk_pool(adapter,,
    if (ring.xsk_pool) {
    WARN_ON(xdp_rxq_info_reg_mem_model(&ring.xdp_rxq,
    MEM_TYPE_XSK_BUFF_POOL,
    pub &ring->xdp_rxq): xsk_pool_set_rxq_info(ring->xsk_pool,,
    } else {
    WARN_ON(xdp_rxq_info_reg_mem_model(&ring.xdp_rxq,
    pub NULL)): MEM_TYPE_PAGE_SHARED,,
    }
// disable queue to avoid use of these values while updating state
    pub IXGBE_RXDCTL(reg_idx)): rxdctl = IXGBE_READ_REG(hw,,
    pub ~IXGBE_RXDCTL_ENABLE: rxdctl &=,
// write value back with RXDCTL.ENABLE bit cleared
    pub rxdctl): IXGBE_WRITE_REG(hw, IXGBE_RXDCTL(reg_idx),,
    pub DMA_BIT_MASK(32))): IXGBE_WRITE_REG(hw, IXGBE_RDBAL(reg_idx), (rdba &,
    pub 32)): IXGBE_WRITE_REG(hw, IXGBE_RDBAH(reg_idx), (rdba >>,
    IXGBE_WRITE_REG(hw, IXGBE_RDLEN(reg_idx),
    pub ixgbe_adv_rx_desc)): *mut *mut ring->count  sizeof(union,
// Force flushing of IXGBE_RDLEN to prevent MDD
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_RDH(reg_idx),,
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_RDT(reg_idx),,
    pub IXGBE_RDT(reg_idx): ring->tail = adapter->io_addr +,
    pub ring): ixgbe_configure_srrctl(adapter,,
    pub ring): ixgbe_configure_rscctl(adapter,,
    if (hw.mac.type == ixgbe_mac_82598EB) {
//
// enable cache line friendly hardware writes:
// PTHRESH=32 descriptors (half the internal cache),
// this also removes ugly rx_no_buffer_count increment
// HTHRESH=4 descriptors (to minimize latency on fetch)
// WTHRESH=8 burst writeback up to two cache lines
//
    pub ~0x3FFFFF: rxdctl &=,
    pub 0x080420: rxdctl |=,

// RXDCTL.RLPML does not work on 82599
    } else if (hw.mac.type != ixgbe_mac_82599EB) {
    rxdctl &= ~(IXGBE_RXDCTL_RLPMLMASK |
// Limit the maximum frame size so we don't overrun the skb.
// This can happen in SRIOV mode when the MTU of the VF is
// higher than the MTU of the PF.
//
    if (ring_uses_build_skb(ring) &&
    !test_bit(__IXGBE_RX_3K_BUFFER, ring.state))
    rxdctl |= IXGBE_MAX_2K_FRAME_BUILD_SKB |

    }
    pub ixgbe_rx_offset(ring): ring->rx_offset =,
    if (ring.xsk_pool && hw.mac.type != ixgbe_mac_82599EB) {
    pub xsk_pool_get_rx_frame_size(ring->xsk_pool): u32 xsk_buf_len =,
    rxdctl &= ~(IXGBE_RXDCTL_RLPMLMASK |
    pub IXGBE_RXDCTL_RLPML_EN: rxdctl |= xsk_buf_len |,
    pub xsk_buf_len: ring->rx_buf_len =,
    }
// initialize rx_buffer_info
    memset(ring.rx_buffer_info, 0,
    pub ring->count): *mut *mut sizeof(struct ixgbe_rx_buffer),
// initialize Rx descriptor 0
    pub 0): rx_desc = IXGBE_RX_DESC(ring,,
    pub 0: rx_desc->wb.upper.length =,
// enable receive descriptor ring
    pub IXGBE_RXDCTL_ENABLE: rxdctl |=,
    pub rxdctl): IXGBE_WRITE_REG(hw, IXGBE_RXDCTL(reg_idx),,
    pub ring): ixgbe_rx_desc_queue_enable(adapter,,
    if (ring.xsk_pool)
    pub ixgbe_desc_unused(ring)): ixgbe_alloc_rx_buffers_zc(ring,,
    else
    pub ixgbe_desc_unused(ring)): ixgbe_alloc_rx_buffers(ring,,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_psrtype(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_psrtype(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub adapter->ring_feature[RING_F_RSS].indices: int rss_i =,
    pub adapter->num_rx_pools: u16 pool =,
// PSRTYPE must be initialized in non 82598 adapters
    u32 psrtype = IXGBE_PSRTYPE_TCPHDR |
    IXGBE_PSRTYPE_UDPHDR |
    IXGBE_PSRTYPE_IPV4HDR |
    IXGBE_PSRTYPE_L2HDR |
    if (hw.mac.type == ixgbe_mac_82598EB)
    if (rss_i > 3)
    pub 29: psrtype |= 2u <<,
#[no_mangle]
pub unsafe extern "C" fn if(1: rss_i >) -> else {
    else if (rss_i > 1)
    pub 29: psrtype |= 1u <<,
    while (pool--)
    pub psrtype): IXGBE_WRITE_REG(hw, IXGBE_PSRTYPE(VMDQ_P(pool)),,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_virtualization(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_virtualization(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub adapter->num_rx_pools: u16 pool =,
    pub vmolr: u32 reg_offset, vf_shift,,
    pub vmdctl: u32 gcr_ext,,
    pub i: c_int,
    if (!(adapter.flags & IXGBE_FLAG_SRIOV_ENABLED))
    pub IXGBE_VT_CTL): vmdctl = IXGBE_READ_REG(hw,,
    pub IXGBE_VMD_CTL_VMDQ_EN: vmdctl |=,
    pub ~IXGBE_VT_CTL_POOL_MASK: vmdctl &=,
    pub IXGBE_VT_CTL_POOL_SHIFT: vmdctl |= VMDQ_P(0) <<,
    pub IXGBE_VT_CTL_REPLEN: vmdctl |=,
    pub vmdctl): IXGBE_WRITE_REG(hw, IXGBE_VT_CTL,,
// accept untagged packets until a vlan tag is
// specifically set for the VMDQ queue/pool
//
    pub IXGBE_VMOLR_AUPE: vmolr =,
    while (pool--)
    pub vmolr): IXGBE_WRITE_REG(hw, IXGBE_VMOLR(VMDQ_P(pool)),,
    pub 32: vf_shift = VMDQ_P(0) %,
    pub 0: reg_offset = (VMDQ_P(0) >= 32) ? 1 :,
// Enable only the PF's pool for Tx/Rx
    pub vf_shift)): IXGBE_WRITE_REG(hw, IXGBE_VFRE(reg_offset), GENMASK(31,,
    pub 1): IXGBE_WRITE_REG(hw, IXGBE_VFRE(reg_offset ^ 1), reg_offset -,
    pub vf_shift)): IXGBE_WRITE_REG(hw, IXGBE_VFTE(reg_offset), GENMASK(31,,
    pub 1): IXGBE_WRITE_REG(hw, IXGBE_VFTE(reg_offset ^ 1), reg_offset -,
    if (adapter.bridge_mode == BRIDGE_MODE_VEB)
    pub IXGBE_PFDTXGSWC_VT_LBEN): IXGBE_WRITE_REG(hw, IXGBE_PFDTXGSWC,,
// Map PF MAC address in RAR Entry 0 to first pool following VFs
    pub VMDQ_P(0)): hw->mac.ops.set_vmdq(hw, 0,,
// clear VLAN promisc flag so VFTA will be updated if necessary
    pub ~IXGBE_FLAG2_VLAN_PROMISC: adapter->flags2 &=,
//
// Set up VF register offsets for selected VT Mode,
// i.e. 32 or 64 VFs for SR-IOV
//
    switch (adapter.ring_feature[RING_F_VMDQ].mask) {
    case IXGBE_82599_VMDQ_8Q_MASK:
    pub IXGBE_GCR_EXT_VT_MODE_16: gcr_ext =,
    case IXGBE_82599_VMDQ_4Q_MASK:
    pub IXGBE_GCR_EXT_VT_MODE_32: gcr_ext =,
    default:
    pub IXGBE_GCR_EXT_VT_MODE_64: gcr_ext =,
    }
    pub gcr_ext): IXGBE_WRITE_REG(hw, IXGBE_GCR_EXT,,
    pub {: for (i = 0; i < adapter->num_vfs; i++),
// configure spoof checking
    ixgbe_ndo_set_vf_spoofchk(adapter.netdev, i,
// Enable/Disable RSS query feature
    ixgbe_ndo_set_vf_rss_query_en(adapter.netdev, i,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_set_rx_buffer_len(adapter: *mut ixgbe_adapter) {
    static void ixgbe_set_rx_buffer_len(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub ETH_FCS_LEN: int max_frame = netdev->mtu + ETH_HLEN +,
    pub rx_ring: *mut ixgbe_ring,
    pub i: c_int,
    pub hlreg0: u32 mhadd,,

// adjust max frame to be able to do baby jumbo for FCoE
    if ((adapter.flags & IXGBE_FLAG_FCOE_ENABLED) &&
    (max_frame < IXGBE_FCOE_JUMBO_FRAME_SIZE))
    pub IXGBE_FCOE_JUMBO_FRAME_SIZE: max_frame =,

// adjust max frame to be at least the size of a standard frame
    if (max_frame < (ETH_FRAME_LEN + ETH_FCS_LEN))
    pub ETH_FCS_LEN): max_frame = (ETH_FRAME_LEN +,
    pub IXGBE_MHADD): mhadd = IXGBE_READ_REG(hw,,
    if (max_frame != (mhadd >> IXGBE_MHADD_MFS_SHIFT)) {
    pub ~IXGBE_MHADD_MFS_MASK: mhadd &=,
    pub IXGBE_MHADD_MFS_SHIFT: mhadd |= max_frame <<,
    pub mhadd): IXGBE_WRITE_REG(hw, IXGBE_MHADD,,
    }
    pub IXGBE_HLREG0): hlreg0 = IXGBE_READ_REG(hw,,
// set jumbo enable since MHADD.MFS is keeping size locked at max_frame
    pub IXGBE_HLREG0_JUMBOEN: hlreg0 |=,
    pub hlreg0): IXGBE_WRITE_REG(hw, IXGBE_HLREG0,,
//
// Setup the HW Rx Head and Tail Descriptor Pointers and
// the Base and Length of the Rx Descriptor Ring
//
    pub {: for (i = 0; i < adapter->num_rx_queues; i++),
    pub adapter->rx_ring[i]: rx_ring =,
    pub rx_ring->state): clear_bit(__IXGBE_RX_3K_BUFFER,,
    pub rx_ring->state): clear_bit(__IXGBE_RX_BUILD_SKB_ENABLED,,
    if (adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED)
    if (test_bit(__IXGBE_RX_FCOE, rx_ring.state))
    pub rx_ring->state): set_bit(__IXGBE_RX_3K_BUFFER,,
    if (adapter.flags2 & IXGBE_FLAG2_RX_LEGACY)
    pub rx_ring->state): set_bit(__IXGBE_RX_BUILD_SKB_ENABLED,,

    if (adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED)
    pub rx_ring->state): set_bit(__IXGBE_RX_3K_BUFFER,,
    if (IXGBE_2K_TOO_SMALL_WITH_PADDING ||
    (max_frame > (ETH_FRAME_LEN + ETH_FCS_LEN)))
    pub rx_ring->state): set_bit(__IXGBE_RX_3K_BUFFER,,

    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_rdrxctl(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_rdrxctl(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub IXGBE_RDRXCTL): u32 rdrxctl = IXGBE_READ_REG(hw,,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
//
// For VMDq support of different descriptor types or
// buffer sizes through the use of multiple SRRCTL
// registers, RDRXCTL.MVMEN must be set to 1
//
// also, the manual doesn't mention it clearly but DCA hints
// will only use queue 0's tags unless this bit is set.  Side
// effects of setting this bit are only that SRRCTL must be
// fully programmed [0..15]
//
    pub IXGBE_RDRXCTL_MVMEN: rdrxctl |=,
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    if (adapter.num_vfs)
    pub IXGBE_RDRXCTL_PSP: rdrxctl |=,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
// Disable RSC for ACK packets
    IXGBE_WRITE_REG(hw, IXGBE_RSCDBU,
    pub IXGBE_RSCDBU))): (IXGBE_RSCDBU_RSCACKDIS | IXGBE_READ_REG(hw,,
    pub ~IXGBE_RDRXCTL_RSCFRSTSIZE: rdrxctl &=,
// hardware requires some bits to be set by default
    pub IXGBE_RDRXCTL_FCOE_WRFIX): rdrxctl |= (IXGBE_RDRXCTL_RSCACKC |,
    pub IXGBE_RDRXCTL_CRCSTRIP: rdrxctl |=,
    default:
// We should do nothing since we don't know this hardware
    }
    pub rdrxctl): IXGBE_WRITE_REG(hw, IXGBE_RDRXCTL,,
    }
//
// ixgbe_configure_rx - Configure 8259x Receive Unit after Reset
// @adapter: board private structure
//
// Configure the Rx unit of the MAC after a reset.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_rx(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_rx(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    pub rfctl: u32 rxctrl,,
// disable receives while setting up the descriptors
// RSC Setup
    pub IXGBE_RFCTL): rfctl = IXGBE_READ_REG(hw,,
    pub ~IXGBE_RFCTL_RSC_DIS: rfctl &=,
    if (!(adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED))
    pub IXGBE_RFCTL_RSC_DIS: rfctl |=,
// disable NFS filtering
    pub IXGBE_RFCTL_NFSR_DIS): rfctl |= (IXGBE_RFCTL_NFSW_DIS |,
    pub rfctl): IXGBE_WRITE_REG(hw, IXGBE_RFCTL,,
// Program registers for the distribution of queues
// set_rx_buffer_len must be called before ring initialization
//
// Setup the HW Rx Head and Tail Descriptor Pointers and
// the Base and Length of the Rx Descriptor Ring
//
    pub i++): for (i = 0; i < adapter->num_rx_queues;,
    pub adapter->rx_ring[i]): ixgbe_configure_rx_ring(adapter,,
    pub IXGBE_RXCTRL): rxctrl = IXGBE_READ_REG(hw,,
// disable drop enable for 82598 parts
    if (hw.mac.type == ixgbe_mac_82598EB)
    pub IXGBE_RXCTRL_DMBYPS: rxctrl |=,
// enable all receives
    pub IXGBE_RXCTRL_RXEN: rxctrl |=,
    pub rxctrl): hw->mac.ops.enable_rx_dma(hw,,
    }
    static int ixgbe_vlan_rx_add_vid(struct net_device *netdev,
    __be16 proto, u16 vid)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
// add VID to filter table
    if (!vid || !(adapter.flags2 & IXGBE_FLAG2_VLAN_PROMISC))
    pub !!vid): hw->mac.ops.set_vfta(&adapter->hw, vid, VMDQ_P(0), true,,
    pub adapter->active_vlans): set_bit(vid,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_find_vlvf_entry(hw: *mut ixgbe_hw, vlan: u32) -> c_int {
    static int ixgbe_find_vlvf_entry(struct ixgbe_hw *hw, u32 vlan)
    {
    pub vlvf: u32,
    pub idx: c_int,
// short cut the special case
    if (vlan == 0)
    pub 0: return,
// Search for the vlan id in the VLVF entries
    pub {: for (idx = IXGBE_VLVF_ENTRIES; --idx;),
    pub IXGBE_VLVF(idx)): vlvf = IXGBE_READ_REG(hw,,
    if ((vlvf & VLAN_VID_MASK) == vlan)
    }
    pub idx: return,
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_update_pf_promisc_vlvf(adapter: *mut ixgbe_adapter, vid: u32) {
    void ixgbe_update_pf_promisc_vlvf(struct ixgbe_adapter *adapter, u32 vid)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub word: u32 bits,,
    pub idx: c_int,
    pub vid): idx = ixgbe_find_vlvf_entry(hw,,
    if (!idx)
// See if any other pools are set for this VLAN filter
// entry other than the PF.
//
    pub 32): *mut *mut word = idx  2 + (VMDQ_P(0) /,
    pub 32): bits = ~BIT(VMDQ_P(0) %,
    pub IXGBE_VLVFB(word)): bits &= IXGBE_READ_REG(hw,,
// Disable the filter so this falls into the default pool.
    if (!bits && !IXGBE_READ_REG(hw, IXGBE_VLVFB(word ^ 1))) {
    if (!(adapter.flags2 & IXGBE_FLAG2_VLAN_PROMISC))
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_VLVFB(word),,
    pub 0): IXGBE_WRITE_REG(hw, IXGBE_VLVF(idx),,
    }
    }
    static int ixgbe_vlan_rx_kill_vid(struct net_device *netdev,
    __be16 proto, u16 vid)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
// remove VID from filter table
    if (vid && !(adapter.flags2 & IXGBE_FLAG2_VLAN_PROMISC))
    pub true): hw->mac.ops.set_vfta(hw, vid, VMDQ_P(0), false,,
    pub adapter->active_vlans): clear_bit(vid,,
    pub 0: return,
    }
//
// ixgbe_vlan_strip_disable - helper to disable hw vlan stripping
// @adapter: driver data
//
#[no_mangle]
unsafe extern "C" fn ixgbe_vlan_strip_disable(adapter: *mut ixgbe_adapter) {
    static void ixgbe_vlan_strip_disable(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub vlnctrl: u32,
    pub j: int i,,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub IXGBE_VLNCTRL): vlnctrl = IXGBE_READ_REG(hw,,
    pub ~IXGBE_VLNCTRL_VME: vlnctrl &=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_VLNCTRL,,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pub {: for (i = 0; i < adapter->num_rx_queues; i++),
    pub adapter->rx_ring[i]: *mut *mut ixgbe_ring ring =,
    if (!netif_is_ixgbe(ring.netdev))
    pub ring->reg_idx: j =,
    pub IXGBE_RXDCTL(j)): vlnctrl = IXGBE_READ_REG(hw,,
    pub ~IXGBE_RXDCTL_VME: vlnctrl &=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_RXDCTL(j),,
    }
    default:
    }
    }
//
// ixgbe_vlan_strip_enable - helper to enable hw vlan stripping
// @adapter: driver data
//
#[no_mangle]
unsafe extern "C" fn ixgbe_vlan_strip_enable(adapter: *mut ixgbe_adapter) {
    static void ixgbe_vlan_strip_enable(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub vlnctrl: u32,
    pub j: int i,,
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pub IXGBE_VLNCTRL): vlnctrl = IXGBE_READ_REG(hw,,
    pub IXGBE_VLNCTRL_VME: vlnctrl |=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_VLNCTRL,,
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pub {: for (i = 0; i < adapter->num_rx_queues; i++),
    pub adapter->rx_ring[i]: *mut *mut ixgbe_ring ring =,
    if (!netif_is_ixgbe(ring.netdev))
    pub ring->reg_idx: j =,
    pub IXGBE_RXDCTL(j)): vlnctrl = IXGBE_READ_REG(hw,,
    pub IXGBE_RXDCTL_VME: vlnctrl |=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_RXDCTL(j),,
    }
    default:
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_vlan_promisc_enable(adapter: *mut ixgbe_adapter) {
    static void ixgbe_vlan_promisc_enable(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: u32 vlnctrl,,
    pub IXGBE_VLNCTRL): vlnctrl = IXGBE_READ_REG(hw,,
    if (adapter.flags & IXGBE_FLAG_VMDQ_ENABLED) {
// For VMDq and SR-IOV we must leave VLAN filtering enabled
    pub IXGBE_VLNCTRL_VFE: vlnctrl |=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_VLNCTRL,,
    } else {
    pub ~IXGBE_VLNCTRL_VFE: vlnctrl &=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_VLNCTRL,,
    }
// Nothing to do for 82598
    if (hw.mac.type == ixgbe_mac_82598EB)
// We are already in VLAN promisc, nothing to do
    if (adapter.flags2 & IXGBE_FLAG2_VLAN_PROMISC)
// Set flag so we don't redo unnecessary work
    pub IXGBE_FLAG2_VLAN_PROMISC: adapter->flags2 |=,
// Add PF to all active pools
    pub {: for (i = IXGBE_VLVF_ENTRIES; --i;),
    pub 32): *mut *mut u32 reg_offset = IXGBE_VLVFB(i  2 + VMDQ_P(0) /,
    pub reg_offset): u32 vlvfb = IXGBE_READ_REG(hw,,
    pub 32): vlvfb |= BIT(VMDQ_P(0) %,
    pub vlvfb): IXGBE_WRITE_REG(hw, reg_offset,,
    }
// Set all bits in the VLAN filter table array
    pub i--;): for (i = hw->mac.vft_size;,
    pub ~0U): IXGBE_WRITE_REG(hw, IXGBE_VFTA(i),,
    }
pub const VFTA_BLOCK_SIZE: c_int = 8;
#[no_mangle]
unsafe extern "C" fn ixgbe_scrub_vfta(adapter: *mut ixgbe_adapter, vfta_offset: u32) {
    static void ixgbe_scrub_vfta(struct ixgbe_adapter *adapter, u32 vfta_offset)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub }: u32 vfta[VFTA_BLOCK_SIZE] = { 0,
    pub 32: *mut *mut u32 vid_start = vfta_offset,
    pub 32): *mut *mut u32 vid_end = vid_start + (VFTA_BLOCK_SIZE,
    pub bits: u32 i, vid, word,,
    pub {: for (i = IXGBE_VLVF_ENTRIES; --i;),
    pub IXGBE_VLVF(i)): u32 vlvf = IXGBE_READ_REG(hw,,
// pull VLAN ID from VLVF
    pub VLAN_VID_MASK: vid = vlvf &,
// only concern ourselves with a certain range
    if (vid < vid_start || vid >= vid_end)
    if (vlvf) {
// record VLAN ID in VFTA
    pub 32): vfta[(vid - vid_start) / 32] |= BIT(vid %,
// if PF is part of this then continue
    if (test_bit(vid, adapter.active_vlans))
    }
// remove PF from the pool
    pub 32: *mut *mut word = i  2 + VMDQ_P(0) /,
    pub 32): bits = ~BIT(VMDQ_P(0) %,
    pub IXGBE_VLVFB(word)): bits &= IXGBE_READ_REG(hw,,
    pub bits): IXGBE_WRITE_REG(hw, IXGBE_VLVFB(word),,
    }
// extract values from active_vlans and write back to VFTA
    pub {: for (i = VFTA_BLOCK_SIZE; i--;),
    pub 32: *mut *mut vid = (vfta_offset + i),
    pub BITS_PER_LONG: word = vid /,
    pub BITS_PER_LONG: bits = vid %,
    pub bits: vfta[i] |= adapter->active_vlans[word] >>,
    pub vfta[i]): IXGBE_WRITE_REG(hw, IXGBE_VFTA(vfta_offset + i),,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_vlan_promisc_disable(adapter: *mut ixgbe_adapter) {
    static void ixgbe_vlan_promisc_disable(struct ixgbe_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: u32 vlnctrl,,
// Set VLAN filtering to enabled
    pub IXGBE_VLNCTRL): vlnctrl = IXGBE_READ_REG(hw,,
    pub IXGBE_VLNCTRL_VFE: vlnctrl |=,
    pub vlnctrl): IXGBE_WRITE_REG(hw, IXGBE_VLNCTRL,,
    if (!(adapter.flags & IXGBE_FLAG_VMDQ_ENABLED) ||
    hw.mac.type == ixgbe_mac_82598EB)
// We are not in VLAN promisc, nothing to do
    if (!(adapter.flags2 & IXGBE_FLAG2_VLAN_PROMISC))
// Set flag so we don't redo unnecessary work
    pub ~IXGBE_FLAG2_VLAN_PROMISC: adapter->flags2 &=,
    pub VFTA_BLOCK_SIZE): for (i = 0; i < hw->mac.vft_size; i +=,
    pub i): ixgbe_scrub_vfta(adapter,,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_restore_vlan(adapter: *mut ixgbe_adapter) {
    static void ixgbe_restore_vlan(struct ixgbe_adapter *adapter)
    {
    pub 1: u16 vid =,
    pub 0): ixgbe_vlan_rx_add_vid(adapter->netdev, htons(ETH_P_8021Q),,
    for_each_set_bit_from(vid, adapter.active_vlans, VLAN_N_VID)
    pub vid): ixgbe_vlan_rx_add_vid(adapter->netdev, htons(ETH_P_8021Q),,
    }
//
// ixgbe_write_mc_addr_list - write multicast addresses to MTA
// @netdev: network interface device structure
//
// Writes multicast address list to the MTA hash table.
// Returns: -ENOMEM on failure
// 0 on no addresses written
// X on writing X addresses to MTA
//
#[no_mangle]
unsafe extern "C" fn ixgbe_write_mc_addr_list(netdev: *mut net_device) -> c_int {
    static int ixgbe_write_mc_addr_list(struct net_device *netdev)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    if (!netif_running(netdev))
    pub 0: return,
    if (hw.mac.ops.update_mc_addr_list)
    pub netdev): hw->mac.ops.update_mc_addr_list(hw,,
    else
    pub -ENOMEM: return,

    pub netdev_mc_count(netdev): return,
    }

#[no_mangle]
pub unsafe extern "C" fn ixgbe_full_sync_mac_table(adapter: *mut ixgbe_adapter) {
    void ixgbe_full_sync_mac_table(struct ixgbe_adapter *adapter)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    pub {: for (i = 0; i < hw->mac.num_rar_entries; i++, mac_table++),
    pub ~IXGBE_MAC_STATE_MODIFIED: mac_table->state &=,
    if (mac_table.state & IXGBE_MAC_STATE_IN_USE)
    hw.mac.ops.set_rar(hw, i,
    mac_table.addr,
    mac_table.pool,
    else
    pub i): hw->mac.ops.clear_rar(hw,,
    }
    }

#[no_mangle]
unsafe extern "C" fn ixgbe_sync_mac_table(adapter: *mut ixgbe_adapter) {
    static void ixgbe_sync_mac_table(struct ixgbe_adapter *adapter)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    pub {: for (i = 0; i < hw->mac.num_rar_entries; i++, mac_table++),
    if (!(mac_table.state & IXGBE_MAC_STATE_MODIFIED))
    pub ~IXGBE_MAC_STATE_MODIFIED: mac_table->state &=,
    if (mac_table.state & IXGBE_MAC_STATE_IN_USE)
    hw.mac.ops.set_rar(hw, i,
    mac_table.addr,
    mac_table.pool,
    else
    pub i): hw->mac.ops.clear_rar(hw,,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_flush_sw_mac_table(adapter: *mut ixgbe_adapter) {
    static void ixgbe_flush_sw_mac_table(struct ixgbe_adapter *adapter)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    pub {: for (i = 0; i < hw->mac.num_rar_entries; i++, mac_table++),
    pub IXGBE_MAC_STATE_MODIFIED: mac_table->state |=,
    pub ~IXGBE_MAC_STATE_IN_USE: mac_table->state &=,
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_available_rars(adapter: *mut ixgbe_adapter, pool: u16) -> c_int {
    static int ixgbe_available_rars(struct ixgbe_adapter *adapter, u16 pool)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub 0: int i, count =,
    pub {: for (i = 0; i < hw->mac.num_rar_entries; i++, mac_table++),
// do not count default RAR as available
    if (mac_table.state & IXGBE_MAC_STATE_DEFAULT)
// only count unused and addresses that belong to us
    if (mac_table.state & IXGBE_MAC_STATE_IN_USE) {
    if (mac_table.pool != pool)
    }
    }
    pub count: return,
    }
// this function destroys the first RAR entry
#[no_mangle]
unsafe extern "C" fn ixgbe_mac_set_default_filter(adapter: *mut ixgbe_adapter) {
    static void ixgbe_mac_set_default_filter(struct ixgbe_adapter *adapter)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ETH_ALEN): memcpy(&mac_table->addr, hw->mac.addr,,
    pub VMDQ_P(0): mac_table->pool =,
    pub IXGBE_MAC_STATE_IN_USE: mac_table->state = IXGBE_MAC_STATE_DEFAULT |,
    hw.mac.ops.set_rar(hw, 0, mac_table.addr, mac_table.pool,
    }
    int ixgbe_add_mac_filter(struct ixgbe_adapter *adapter,
    const u8 *addr, u16 pool)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    if (is_zero_ether_addr(addr))
    pub -EINVAL: return,
    pub {: for (i = 0; i < hw->mac.num_rar_entries; i++, mac_table++),
    if (mac_table.state & IXGBE_MAC_STATE_IN_USE)
    pub addr): ether_addr_copy(mac_table->addr,,
    pub pool: mac_table->pool =,
    mac_table.state |= IXGBE_MAC_STATE_MODIFIED |
    pub i: return,
    }
    pub -ENOMEM: return,
    }
    int ixgbe_del_mac_filter(struct ixgbe_adapter *adapter,
    const u8 *addr, u16 pool)
    {
    pub &adapter->mac_table[0]: *mut *mut ixgbe_mac_addr mac_table =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub i: c_int,
    if (is_zero_ether_addr(addr))
    pub -EINVAL: return,
// search table for addr, if found clear IN_USE flag and sync
    pub {: for (i = 0; i < hw->mac.num_rar_entries; i++, mac_table++),
// we can only delete an entry if it is in use
    if (!(mac_table.state & IXGBE_MAC_STATE_IN_USE))
// we only care about entries that belong to the given pool
    if (mac_table.pool != pool)
// we only care about a specific MAC address
    if (!ether_addr_equal(addr, mac_table.addr))
    pub IXGBE_MAC_STATE_MODIFIED: mac_table->state |=,
    pub ~IXGBE_MAC_STATE_IN_USE: mac_table->state &=,
    pub 0: return,
    }
    pub -ENOMEM: return,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_uc_sync(netdev: *mut net_device, addr: *const c_uchar) -> c_int {
    static int ixgbe_uc_sync(struct net_device *netdev, const unsigned char *addr)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub ret: c_int,
    pub VMDQ_P(0)): ret = ixgbe_add_mac_filter(adapter, addr,,
    pub 0): return min_t(int, ret,,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_uc_unsync(netdev: *mut net_device, addr: *const c_uchar) -> c_int {
    static int ixgbe_uc_unsync(struct net_device *netdev, const unsigned char *addr)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub VMDQ_P(0)): ixgbe_del_mac_filter(adapter, addr,,
    pub 0: return,
    }
//
// ixgbe_set_rx_mode - Unicast, Multicast and Promiscuous mode set
// @netdev: network interface device structure
//
// The set_rx_method entry point is called whenever the unicast/multicast
// address list or the network interface flags are updated.  This routine is
// responsible for configuring the hardware for proper unicast, multicast and
// promiscuous mode.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_set_rx_mode(netdev: *mut net_device) {
    void ixgbe_set_rx_mode(struct net_device *netdev)
    {
    pub ixgbe_from_netdev(netdev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub IXGBE_VMOLR_AUPE: u32 fctrl, vmolr = IXGBE_VMOLR_BAM |,
    pub netdev->features: netdev_features_t features =,
    pub count: c_int,
// Check for Promiscuous and All Multicast modes
    pub IXGBE_FCTRL): fctrl = IXGBE_READ_REG(hw,,
// set all bits that we expect to always be set
    pub /: *mut *mut fctrl &= ~IXGBE_FCTRL_SBP; / disable store-bad-packets,
    pub IXGBE_FCTRL_BAM: fctrl |=,
    pub /: *mut *mut fctrl |= IXGBE_FCTRL_DPF; / discard pause frames when FC enabled,
    pub IXGBE_FCTRL_PMCF: fctrl |=,
// clear the bits we are changing the status of
    pub IXGBE_FCTRL_MPE): fctrl &= ~(IXGBE_FCTRL_UPE |,
    if (netdev.flags & IFF_PROMISC) {
    pub true: hw->addr_ctrl.user_set_promisc =,
    pub IXGBE_FCTRL_MPE): fctrl |= (IXGBE_FCTRL_UPE |,
    pub IXGBE_VMOLR_MPE: vmolr |=,
    pub ~NETIF_F_HW_VLAN_CTAG_FILTER: features &=,
    } else {
    if (netdev.flags & IFF_ALLMULTI) {
    pub IXGBE_FCTRL_MPE: fctrl |=,
    pub IXGBE_VMOLR_MPE: vmolr |=,
    }
    pub false: hw->addr_ctrl.user_set_promisc =,
    }
//
// Write addresses to available RAR registers, if there is not
// sufficient space to store all the addresses then enable
// unicast promiscuous mode
//
    if (__dev_uc_sync(netdev, ixgbe_uc_sync, ixgbe_uc_unsync)) {
    pub IXGBE_FCTRL_UPE: fctrl |=,
    pub IXGBE_VMOLR_ROPE: vmolr |=,
    }
// Write addresses to the MTA, if the attempt fails
// then we should just turn on promiscuous mode so
// that we can at least receive multicast traffic
//
    pub ixgbe_write_mc_addr_list(netdev): count =,
    if (count < 0) {
    pub IXGBE_FCTRL_MPE: fctrl |=,
    pub IXGBE_VMOLR_MPE: vmolr |=,
    } else if (count) {
    pub IXGBE_VMOLR_ROMPE: vmolr |=,
    }
    if (hw.mac.type != ixgbe_mac_82598EB) {
    vmolr |= IXGBE_READ_REG(hw, IXGBE_VMOLR(VMDQ_P(0))) &
    ~(IXGBE_VMOLR_MPE | IXGBE_VMOLR_ROMPE |
    pub vmolr): IXGBE_WRITE_REG(hw, IXGBE_VMOLR(VMDQ_P(0)),,
    }
// This is useful for sniffing bad packets.
    if (features & NETIF_F_RXALL) {
// UPE and MPE will be handled by normal PROMISC logic
// in e1000e_set_rx_mode
    fctrl |= (IXGBE_FCTRL_SBP | /* Receive bad packets */
    IXGBE_FCTRL_BAM | /* RX All Bcast Pkts */
    pub /: *mut *mut IXGBE_FCTRL_PMCF); / RX All MAC Ctrl Pkts,
    pub ~(IXGBE_FCTRL_DPF): fctrl &=,
// NOTE:  VLAN filtering is disabled by setting PROMISC
    }
    pub fctrl): IXGBE_WRITE_REG(hw, IXGBE_FCTRL,,
    if (features & NETIF_F_HW_VLAN_CTAG_RX)
    else
    if (features & NETIF_F_HW_VLAN_CTAG_FILTER)
    else
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_napi_enable_all(adapter: *mut ixgbe_adapter) {
    static void ixgbe_napi_enable_all(struct ixgbe_adapter *adapter)
    {
    pub q_idx: c_int,
    pub q_idx++): for (q_idx = 0; q_idx < adapter->num_q_vectors;,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_napi_disable_all(adapter: *mut ixgbe_adapter) {
    static void ixgbe_napi_disable_all(struct ixgbe_adapter *adapter)
    {
    pub q_idx: c_int,
    pub q_idx++): for (q_idx = 0; q_idx < adapter->num_q_vectors;,
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_udp_tunnel_sync(dev: *mut net_device, table: c_uint) -> c_int {
    static int ixgbe_udp_tunnel_sync(struct net_device *dev, unsigned int table)
    {
    pub ixgbe_from_netdev(dev): *mut *mut ixgbe_adapter adapter =,
    pub &adapter->hw: *mut *mut ixgbe_hw hw =,
    pub ti: udp_tunnel_info,
    pub &ti): udp_tunnel_nic_get_port(dev, table, 0,,
    if (ti.type == UDP_TUNNEL_TYPE_VXLAN)
    pub ti.port: adapter->vxlan_port =,
    else
    pub ti.port: adapter->geneve_port =,
    IXGBE_WRITE_REG(hw, IXGBE_VXLANCTRL,
    ntohs(adapter.vxlan_port) |
    ntohs(adapter.geneve_port) <<
    pub 0: return,
    }
    static const struct udp_tunnel_nic_info ixgbe_udp_tunnels_x550 = {
    .sync_table	= ixgbe_udp_tunnel_sync,
    .flags		= UDP_TUNNEL_NIC_INFO_IPV4_ONLY,
    .tables		= {
    { .n_entries = 1, .tunnel_types = UDP_TUNNEL_TYPE_VXLAN,  },
    },
}

    static const struct udp_tunnel_nic_info ixgbe_udp_tunnels_x550em_a = {
    .sync_table	= ixgbe_udp_tunnel_sync,
    .flags		= UDP_TUNNEL_NIC_INFO_IPV4_ONLY,
    .tables		= {
    { .n_entries = 1, .tunnel_types = UDP_TUNNEL_TYPE_VXLAN,  },
    { .n_entries = 1, .tunnel_types = UDP_TUNNEL_TYPE_GENEVE, },
    },
    };

//
// ixgbe_configure_dcb - Configure DCB hardware
// @adapter: ixgbe adapter struct
//
// This is called by the driver on open to configure the DCB hardware.
// This is also called by the gennetlink interface when reconfiguring
// the DCB state.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_dcb(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_dcb(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut max_frame: c_int = adapter.netdev.mtu + ETH_HLEN + ETH_FCS_LEN;
    if (!(adapter.flags & IXGBE_FLAG_DCB_ENABLED)) {
    if (hw.mac.type == ixgbe_mac_82598EB)
    netif_set_tso_max_size(adapter.netdev, 65536);
    return;
    }
    if (hw.mac.type == ixgbe_mac_82598EB)
    netif_set_tso_max_size(adapter.netdev, 32768);

    if (adapter.netdev.fcoe_mtu)
    max_frame = max(max_frame, IXGBE_FCOE_JUMBO_FRAME_SIZE);

// reconfigure the hardware
    if (adapter.dcbx_cap & DCB_CAP_DCBX_VER_CEE) {
    ixgbe_dcb_calculate_tc_credits(hw, &adapter.dcb_cfg, max_frame,
    DCB_TX_CONFIG);
    ixgbe_dcb_calculate_tc_credits(hw, &adapter.dcb_cfg, max_frame,
    DCB_RX_CONFIG);
    ixgbe_dcb_hw_config(hw, &adapter.dcb_cfg);
    } else if (adapter.ixgbe_ieee_ets && adapter.ixgbe_ieee_pfc) {
    ixgbe_dcb_hw_ets(&adapter.hw,
    adapter.ixgbe_ieee_ets,
    max_frame);
    ixgbe_dcb_hw_pfc_config(&adapter.hw,
    adapter.ixgbe_ieee_pfc.pfc_en,
    adapter.ixgbe_ieee_ets.prio_tc);
    }
// Enable RSS Hash per TC
    if (hw.mac.type != ixgbe_mac_82598EB) {
    let mut msb: u32 = 0;
    let mut rss_i: u16 = adapter.ring_feature[RING_F_RSS].indices - 1;
    while (rss_i) {
    msb++;
    rss_i >>= 1;
    }
// write msb to all 8 TCs in one write
    IXGBE_WRITE_REG(hw, IXGBE_RQTC, msb * 0x11111111);
    }
    }

// Additional bittime to account for IXGBE framing
pub const IXGBE_ETH_FRAMING: c_int = 20;
//
// ixgbe_hpbthresh - calculate high water mark for flow control
//
// @adapter: board private structure to calculate for
// @pb: packet buffer to calculate
//
#[no_mangle]
unsafe extern "C" fn ixgbe_hpbthresh(adapter: *mut ixgbe_adapter, pb: c_int) -> c_int {
    static int ixgbe_hpbthresh(struct ixgbe_adapter *adapter, int pb)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct net_device *dev = adapter.netdev;
    int link, tc, kb, marker;
    u32 dv_id, rx_pba;
// Calculate max LAN frame size
    tc = link = dev.mtu + ETH_HLEN + ETH_FCS_LEN + IXGBE_ETH_FRAMING;

// FCoE traffic class uses FCOE jumbo frames
    if (dev.fcoe_mtu && tc < IXGBE_FCOE_JUMBO_FRAME_SIZE &&
    (pb == ixgbe_fcoe_get_tc(adapter)))
    tc = IXGBE_FCOE_JUMBO_FRAME_SIZE;

// Calculate delay value for device
    switch (hw.mac.type) {
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    dv_id = IXGBE_DV_X540(link, tc);
    break;
    default:
    dv_id = IXGBE_DV(link, tc);
    break;
    }
// Loopback switch introduces additional latency
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED)
    dv_id += IXGBE_B2BT(tc);
// Delay value is calculated in bit times convert to KB
    kb = IXGBE_BT2KB(dv_id);
    rx_pba = IXGBE_READ_REG(hw, IXGBE_RXPBSIZE(pb)) >> 10;
    marker = rx_pba - kb;
// It is possible that the packet buffer is not large enough
// to provide required headroom. In this case throw an error
// to user and a do the best we can.
//
    if (marker < 0) {
    e_warn(drv, "Packet Buffer(%i) can not provide enough"
    "headroom to support flow control."
    "Decrease MTU or number of traffic classes\n", pb);
    marker = tc + 1;
    }
    return marker;
    }
//
// ixgbe_lpbthresh - calculate low water mark for flow control
//
// @adapter: board private structure to calculate for
// @pb: packet buffer to calculate
//
#[no_mangle]
unsafe extern "C" fn ixgbe_lpbthresh(adapter: *mut ixgbe_adapter, pb: c_int) -> c_int {
    static int ixgbe_lpbthresh(struct ixgbe_adapter *adapter, int pb)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct net_device *dev = adapter.netdev;
    int tc;
    u32 dv_id;
// Calculate max LAN frame size
    tc = dev.mtu + ETH_HLEN + ETH_FCS_LEN;

// FCoE traffic class uses FCOE jumbo frames
    if (dev.fcoe_mtu && tc < IXGBE_FCOE_JUMBO_FRAME_SIZE &&
    (pb == netdev_get_prio_tc_map(dev, adapter.fcoe.up)))
    tc = IXGBE_FCOE_JUMBO_FRAME_SIZE;

// Calculate delay value for device
    switch (hw.mac.type) {
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    dv_id = IXGBE_LOW_DV_X540(tc);
    break;
    default:
    dv_id = IXGBE_LOW_DV(tc);
    break;
    }
// Delay value is calculated in bit times convert to KB
    return IXGBE_BT2KB(dv_id);
    }
//
// ixgbe_pbthresh_setup - calculate and setup high low water marks
//
#[no_mangle]
unsafe extern "C" fn ixgbe_pbthresh_setup(adapter: *mut ixgbe_adapter) {
    static void ixgbe_pbthresh_setup(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut num_tc: c_int = adapter.hw_tcs;
    int i;
    if (!num_tc)
    num_tc = 1;
    for (i = 0; i < num_tc; i++) {
    hw.fc.high_water[i] = ixgbe_hpbthresh(adapter, i);
    hw.fc.low_water[i] = ixgbe_lpbthresh(adapter, i);
// Low water marks must not be larger than high water marks
    if (hw.fc.low_water[i] > hw.fc.high_water[i])
    hw.fc.low_water[i] = 0;
    }
    for (; i < MAX_TRAFFIC_CLASS; i++)
    hw.fc.high_water[i] = 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_pb(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_pb(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    int hdrm;
    let mut tc: u8 = adapter.hw_tcs;
    if (adapter.flags & IXGBE_FLAG_FDIR_HASH_CAPABLE ||
    adapter.flags & IXGBE_FLAG_FDIR_PERFECT_CAPABLE)
    hdrm = 32 << adapter.fdir_pballoc;
    else
    hdrm = 0;
    hw.mac.ops.set_rxpba(hw, tc, hdrm, PBA_STRATEGY_EQUAL);
    ixgbe_pbthresh_setup(adapter);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_fdir_filter_restore(adapter: *mut ixgbe_adapter) {
    static void ixgbe_fdir_filter_restore(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct hlist_node *node2;
    struct ixgbe_fdir_filter *filter;
    u8 queue;
    spin_lock(&adapter.fdir_perfect_lock);
    if (!hlist_empty(&adapter.fdir_filter_list))
    ixgbe_fdir_set_input_mask_82599(hw, &adapter.fdir_mask);
    hlist_for_each_entry_safe(filter, node2,
    &adapter.fdir_filter_list, fdir_node) {
    if (filter.action == IXGBE_FDIR_DROP_QUEUE) {
    queue = IXGBE_FDIR_DROP_QUEUE;
    } else {
    let mut ring: u32 = ethtool_get_flow_spec_ring(filter.action);
    let mut vf: u8 = ethtool_get_flow_spec_ring_vf(filter.action);
    if (!vf && (ring >= adapter.num_rx_queues)) {
    e_err(drv, "FDIR restore failed without VF, ring: %u\n",
    ring);
    continue;
    } else if (vf &&
    ((vf > adapter.num_vfs) ||
    ring >= adapter.num_rx_queues_per_pool)) {
    e_err(drv, "FDIR restore failed with VF, vf: %hhu, ring: %u\n",
    vf, ring);
    continue;
    }
// Map the ring onto the absolute queue index
    if (!vf)
    queue = adapter.rx_ring[ring].reg_idx;
    else
    queue = ((vf - 1) *
    adapter.num_rx_queues_per_pool) + ring;
    }
    ixgbe_fdir_write_perfect_filter_82599(hw,
    &filter.filter, filter.sw_idx, queue);
    }
    spin_unlock(&adapter.fdir_perfect_lock);
    }
//
// ixgbe_clean_rx_ring - Free Rx Buffers per Queue
// @rx_ring: ring to free buffers from
//
#[no_mangle]
unsafe extern "C" fn ixgbe_clean_rx_ring(rx_ring: *mut ixgbe_ring) {
    static void ixgbe_clean_rx_ring(struct ixgbe_ring *rx_ring)
    {
    let mut i: u16 = rx_ring.next_to_clean;
    struct ixgbe_rx_buffer *rx_buffer = &rx_ring.rx_buffer_info[i];
    if (rx_ring.xsk_pool) {
    ixgbe_xsk_clean_rx_ring(rx_ring);
    goto skip_free;
    }
// Free all the Rx ring sk_buffs
    while (i != rx_ring.next_to_alloc) {
    if (rx_buffer.skb) {
    struct sk_buff *skb = rx_buffer.skb;
    if (IXGBE_CB(skb).page_released)
    dma_unmap_page_attrs(rx_ring.dev,
    IXGBE_CB(skb).dma,
    ixgbe_rx_pg_size(rx_ring),
    DMA_FROM_DEVICE,
    IXGBE_RX_DMA_ATTR);
    dev_kfree_skb(skb);
    }
// Invalidate cache lines that may have been written to by
// device so that we avoid corrupting memory.
//
    dma_sync_single_range_for_cpu(rx_ring.dev,
    rx_buffer.dma,
    rx_buffer.page_offset,
    ixgbe_rx_bufsz(rx_ring),
    DMA_FROM_DEVICE);
// free resources associated with mapping
    dma_unmap_page_attrs(rx_ring.dev, rx_buffer.dma,
    ixgbe_rx_pg_size(rx_ring),
    DMA_FROM_DEVICE,
    IXGBE_RX_DMA_ATTR);
    __page_frag_cache_drain(rx_buffer.page,
    rx_buffer.pagecnt_bias);
    i++;
    rx_buffer++;
    if (i == rx_ring.count) {
    i = 0;
    rx_buffer = rx_ring.rx_buffer_info;
    }
    }
    skip_free:
    rx_ring.next_to_alloc = 0;
    rx_ring.next_to_clean = 0;
    rx_ring.next_to_use = 0;
    }
    static int ixgbe_fwd_ring_up(struct ixgbe_adapter *adapter,
    struct ixgbe_fwd_adapter *accel)
    {
    let mut rss_i: u16 = adapter.ring_feature[RING_F_RSS].indices;
    let mut num_tc: c_int = netdev_get_num_tc(adapter.netdev);
    struct net_device *vdev = accel.netdev;
    int i, baseq, err;
    baseq = accel.pool * adapter.num_rx_queues_per_pool;
    netdev_dbg(vdev, "pool %i:%i queues %i:%i\n",
    accel.pool, adapter.num_rx_pools,
    baseq, baseq + adapter.num_rx_queues_per_pool);
    accel.rx_base_queue = baseq;
    accel.tx_base_queue = baseq;
// record configuration for macvlan interface in vdev
    for (i = 0; i < num_tc; i++)
    netdev_bind_sb_channel_queue(adapter.netdev, vdev,
    i, rss_i, baseq + (rss_i * i));
    for (i = 0; i < adapter.num_rx_queues_per_pool; i++)
    adapter.rx_ring[baseq + i].netdev = vdev;
// Guarantee all rings are updated before we update the
// MAC address filter.
//
    wmb();
// ixgbe_add_mac_filter will return an index if it succeeds, so we
// need to only treat it as an error value if it is negative.
//
    err = ixgbe_add_mac_filter(adapter, vdev.dev_addr,
    VMDQ_P(accel.pool));
    if (err >= 0)
    return 0;
// if we cannot add the MAC rule then disable the offload
    macvlan_release_l2fw_offload(vdev);
    for (i = 0; i < adapter.num_rx_queues_per_pool; i++)
    adapter.rx_ring[baseq + i].netdev = core::ptr::null_mut();
    netdev_err(vdev, "L2FW offload disabled due to L2 filter error\n");
// unbind the queues and drop the subordinate channel config
    netdev_unbind_sb_channel(adapter.netdev, vdev);
    netdev_set_sb_channel(vdev, 0);
    clear_bit(accel.pool, adapter.fwd_bitmask);
    kfree(accel);
    return err;
    }
    static int ixgbe_macvlan_up(struct net_device *vdev,
    struct netdev_nested_priv *priv)
    {
    struct ixgbe_adapter *adapter = (struct ixgbe_adapter *)priv.data;
    struct ixgbe_fwd_adapter *accel;
    if (!netif_is_macvlan(vdev))
    return 0;
    accel = macvlan_accel_priv(vdev);
    if (!accel)
    return 0;
    ixgbe_fwd_ring_up(adapter, accel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_configure_dfwd(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure_dfwd(struct ixgbe_adapter *adapter)
    {
    struct netdev_nested_priv priv = {
    .data = (void *)adapter,
    };
    netdev_walk_all_upper_dev_rcu(adapter.netdev,
    ixgbe_macvlan_up, &priv);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_configure(adapter: *mut ixgbe_adapter) {
    static void ixgbe_configure(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    ixgbe_configure_pb(adapter);

    ixgbe_configure_dcb(adapter);

//
// We must restore virtualization before VLANs or else
// the VLVF registers will not be populated
//
    ixgbe_configure_virtualization(adapter);
    ixgbe_set_rx_mode(adapter.netdev);
    ixgbe_restore_vlan(adapter);
    ixgbe_ipsec_restore(adapter);
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    hw.mac.ops.disable_rx_buff(hw);
    break;
    default:
    break;
    }
    if (adapter.flags & IXGBE_FLAG_FDIR_HASH_CAPABLE) {
    ixgbe_init_fdir_signature_82599(&adapter.hw,
    adapter.fdir_pballoc);
    } else if (adapter.flags & IXGBE_FLAG_FDIR_PERFECT_CAPABLE) {
    ixgbe_init_fdir_perfect_82599(&adapter.hw,
    adapter.fdir_pballoc);
    ixgbe_fdir_filter_restore(adapter);
    }
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    hw.mac.ops.enable_rx_buff(hw);
    break;
    default:
    break;
    }

// configure DCA
    if (adapter.flags & IXGBE_FLAG_DCA_CAPABLE)
    ixgbe_setup_dca(adapter);

// configure FCoE L2 filters, redirection table, and Rx control
    ixgbe_configure_fcoe(adapter);

    ixgbe_configure_tx(adapter);
    ixgbe_configure_rx(adapter);
    ixgbe_configure_dfwd(adapter);
    }
//
// ixgbe_enable_link_status_events - enable link status events
// @adapter: pointer to the adapter structure
// @mask: event mask to be set
//
// Enables link status events by invoking ixgbe_configure_lse()
//
// Return: the exit code of the operation.
//
    static int ixgbe_enable_link_status_events(struct ixgbe_adapter *adapter,
    u16 mask)
    {
    int err;
    err = ixgbe_configure_lse(&adapter.hw, true, mask);
    if (err)
    return err;
    adapter.lse_mask = mask;
    return 0;
    }
//
// ixgbe_disable_link_status_events - disable link status events
// @adapter: pointer to the adapter structure
//
// Disables link status events by invoking ixgbe_configure_lse()
//
// Return: the exit code of the operation.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_disable_link_status_events(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_disable_link_status_events(struct ixgbe_adapter *adapter)
    {
    int err;
    err = ixgbe_configure_lse(&adapter.hw, false, adapter.lse_mask);
    if (err)
    return err;
    adapter.lse_mask = 0;
    return 0;
    }
//
// ixgbe_sfp_link_config - set up SFP+ link
// @adapter: pointer to private adapter struct
//
#[no_mangle]
unsafe extern "C" fn ixgbe_sfp_link_config(adapter: *mut ixgbe_adapter) {
    static void ixgbe_sfp_link_config(struct ixgbe_adapter *adapter)
    {
//
// We are assuming the worst case scenario here, and that
// is that an SFP was inserted/removed after the reset
// but before SFP detection was enabled.  As such the best
// solution is to just start searching as soon as we start
//
    if (adapter.hw.mac.type == ixgbe_mac_82598EB)
    adapter.flags2 |= IXGBE_FLAG2_SEARCH_FOR_SFP;
    adapter.flags2 |= IXGBE_FLAG2_SFP_NEEDS_RESET;
    adapter.sfp_poll_time = 0;
    }
//
// ixgbe_non_sfp_link_config - set up non-SFP+ link
// @hw: pointer to private hardware struct
//
// Configure non-SFP link.
//
// Return: 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_non_sfp_link_config(hw: *mut ixgbe_hw) -> c_int {
    static int ixgbe_non_sfp_link_config(struct ixgbe_hw *hw)
    {
    struct ixgbe_adapter *adapter = container_of(hw, struct ixgbe_adapter,
    hw);
    u16 mask = ~((u16)(IXGBE_ACI_LINK_EVENT_UPDOWN |
    IXGBE_ACI_LINK_EVENT_MEDIA_NA |
    IXGBE_ACI_LINK_EVENT_MODULE_QUAL_FAIL |
    IXGBE_ACI_LINK_EVENT_PHY_FW_LOAD_FAIL));
    bool autoneg, link_up = false;
    let mut ret: c_int = -EIO;
    u32 speed;
    if (hw.mac.ops.check_link)
    ret = hw.mac.ops.check_link(hw, &speed, &link_up, false);
    if (ret)
    return ret;
    speed = hw.phy.autoneg_advertised;
    if (!speed && hw.mac.ops.get_link_capabilities) {
    ret = hw.mac.ops.get_link_capabilities(hw, &speed,
    &autoneg);
// remove NBASE-T speeds from default autonegotiation
// to accommodate broken network switches in the field
// which cannot cope with advertised NBASE-T speeds
//
    speed &= ~(IXGBE_LINK_SPEED_5GB_FULL |
    IXGBE_LINK_SPEED_2_5GB_FULL);
    }
    if (ret)
    return ret;
    if (hw.mac.ops.setup_link) {
    if (adapter.hw.mac.type == ixgbe_mac_e610) {
    ret = ixgbe_enable_link_status_events(adapter, mask);
    if (ret)
    return ret;
    }
    ret = hw.mac.ops.setup_link(hw, speed, link_up);
    }
    return ret;
    }
//
// ixgbe_check_media_subtask - check for media
// @adapter: pointer to adapter structure
//
// If media is available then initialize PHY user configuration. Configure the
// PHY if the interface is up.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_check_media_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_check_media_subtask(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
// No need to check for media if it's already present
    if (!(adapter.flags2 & IXGBE_FLAG2_NO_MEDIA))
    return;
// Refresh link info and check if media is present
    if (ixgbe_update_link_info(hw))
    return;
    ixgbe_check_link_cfg_err(adapter, hw.link.link_info.link_cfg_err);
    if (hw.link.link_info.link_info & IXGBE_ACI_MEDIA_AVAILABLE) {
// PHY settings are reset on media insertion, reconfigure
// PHY to preserve settings.
//
    if (!(ixgbe_non_sfp_link_config(&adapter.hw)))
    adapter.flags2 &= ~IXGBE_FLAG2_NO_MEDIA;
// A Link Status Event will be generated; the event handler
// will complete bringing the interface up
//
    }
    }
//
// ixgbe_clear_vf_stats_counters - Clear out VF stats after reset
// @adapter: board private structure
//
// On a reset we need to clear out the VF stats or accounting gets
// messed up because they're not clear on read.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_clear_vf_stats_counters(adapter: *mut ixgbe_adapter) {
    static void ixgbe_clear_vf_stats_counters(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    int i;
    for (i = 0; i < adapter.num_vfs; i++) {
    adapter.vfinfo[i].last_vfstats.gprc =
    IXGBE_READ_REG(hw, IXGBE_PVFGPRC(i));
    adapter.vfinfo[i].saved_rst_vfstats.gprc +=
    adapter.vfinfo[i].vfstats.gprc;
    adapter.vfinfo[i].vfstats.gprc = 0;
    adapter.vfinfo[i].last_vfstats.gptc =
    IXGBE_READ_REG(hw, IXGBE_PVFGPTC(i));
    adapter.vfinfo[i].saved_rst_vfstats.gptc +=
    adapter.vfinfo[i].vfstats.gptc;
    adapter.vfinfo[i].vfstats.gptc = 0;
    adapter.vfinfo[i].last_vfstats.gorc =
    IXGBE_READ_REG(hw, IXGBE_PVFGORC_LSB(i));
    adapter.vfinfo[i].saved_rst_vfstats.gorc +=
    adapter.vfinfo[i].vfstats.gorc;
    adapter.vfinfo[i].vfstats.gorc = 0;
    adapter.vfinfo[i].last_vfstats.gotc =
    IXGBE_READ_REG(hw, IXGBE_PVFGOTC_LSB(i));
    adapter.vfinfo[i].saved_rst_vfstats.gotc +=
    adapter.vfinfo[i].vfstats.gotc;
    adapter.vfinfo[i].vfstats.gotc = 0;
    adapter.vfinfo[i].last_vfstats.mprc =
    IXGBE_READ_REG(hw, IXGBE_PVFMPRC(i));
    adapter.vfinfo[i].saved_rst_vfstats.mprc +=
    adapter.vfinfo[i].vfstats.mprc;
    adapter.vfinfo[i].vfstats.mprc = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_gpie(adapter: *mut ixgbe_adapter) {
    static void ixgbe_setup_gpie(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut gpie: u32 = 0;
    if (adapter.flags & IXGBE_FLAG_MSIX_ENABLED) {
    gpie = IXGBE_GPIE_MSIX_MODE | IXGBE_GPIE_PBA_SUPPORT |
    IXGBE_GPIE_OCD;
    gpie |= IXGBE_GPIE_EIAME;
//
// use EIAM to auto-mask when MSI-X interrupt is asserted
// this saves a register write for every interrupt
//
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    IXGBE_WRITE_REG(hw, IXGBE_EIAM, IXGBE_EICS_RTX_QUEUE);
    break;
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    default:
    IXGBE_WRITE_REG(hw, IXGBE_EIAM_EX(0), 0xFFFFFFFF);
    IXGBE_WRITE_REG(hw, IXGBE_EIAM_EX(1), 0xFFFFFFFF);
    break;
    }
    } else {
// legacy interrupts, use EIAM to auto-mask when reading EICR,
// specifically only auto mask tx and rx interrupts
    IXGBE_WRITE_REG(hw, IXGBE_EIAM, IXGBE_EICS_RTX_QUEUE);
    }
// XXX: to interrupt immediately for EICS writes, enable this
// gpie |= IXGBE_GPIE_EIMEN;
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED) {
    gpie &= ~IXGBE_GPIE_VTMODE_MASK;
    switch (adapter.ring_feature[RING_F_VMDQ].mask) {
    case IXGBE_82599_VMDQ_8Q_MASK:
    gpie |= IXGBE_GPIE_VTMODE_16;
    break;
    case IXGBE_82599_VMDQ_4Q_MASK:
    gpie |= IXGBE_GPIE_VTMODE_32;
    break;
    default:
    gpie |= IXGBE_GPIE_VTMODE_64;
    break;
    }
    }
// Enable Thermal over heat sensor interrupt
    if (adapter.flags2 & IXGBE_FLAG2_TEMP_SENSOR_CAPABLE) {
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82599EB:
    gpie |= IXGBE_SDP0_GPIEN_8259X;
    break;
    default:
    break;
    }
    }
// Enable fan failure interrupt
    if (adapter.flags & IXGBE_FLAG_FAN_FAIL_CAPABLE)
    gpie |= IXGBE_SDP1_GPIEN(hw);
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    gpie |= IXGBE_SDP1_GPIEN_8259X | IXGBE_SDP2_GPIEN_8259X;
    break;
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    gpie |= IXGBE_SDP0_GPIEN_X540;
    break;
    default:
    break;
    }
    IXGBE_WRITE_REG(hw, IXGBE_GPIE, gpie);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_up_complete(adapter: *mut ixgbe_adapter) {
    static void ixgbe_up_complete(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    int err;
    u32 ctrl_ext;
    ixgbe_get_hw_control(adapter);
    ixgbe_setup_gpie(adapter);
    if (adapter.flags & IXGBE_FLAG_MSIX_ENABLED)
    ixgbe_configure_msix(adapter);
    else
    ixgbe_configure_msi_and_legacy(adapter);
// enable the optics for 82599 SFP+ fiber
    if (hw.mac.ops.enable_tx_laser)
    hw.mac.ops.enable_tx_laser(hw);
    if (hw.phy.ops.set_phy_power)
    hw.phy.ops.set_phy_power(hw, true);
    smp_mb__before_atomic();
    clear_bit(__IXGBE_DOWN, &adapter.state);
    ixgbe_napi_enable_all(adapter);
    if (ixgbe_is_sfp(hw)) {
    ixgbe_sfp_link_config(adapter);
    } else {
    err = ixgbe_non_sfp_link_config(hw);
    if (err)
    e_err(probe, "link_config FAILED %d\n", err);
    }
// clear any pending interrupts, may auto mask
    IXGBE_READ_REG(hw, IXGBE_EICR);
    ixgbe_irq_enable(adapter, true, true);
//
// If this adapter has a fan, check to see if we had a failure
// before we enabled the interrupt.
//
    if (adapter.flags & IXGBE_FLAG_FAN_FAIL_CAPABLE) {
    let mut esdp: u32 = IXGBE_READ_REG(hw, IXGBE_ESDP);
    if (esdp & IXGBE_ESDP_SDP1)
    e_crit(drv, "Fan has stopped, replace the adapter\n");
    }
// bring the link up in the watchdog, this could race with our first
// link up interrupt but shouldn't be a problem
    adapter.flags |= IXGBE_FLAG_NEED_LINK_UPDATE;
    adapter.link_check_timeout = jiffies;
    mod_timer(&adapter.service_timer, jiffies);
    ixgbe_clear_vf_stats_counters(adapter);
// Set PF Reset Done bit so PF/VF Mail Ops can work
    ctrl_ext = IXGBE_READ_REG(hw, IXGBE_CTRL_EXT);
    ctrl_ext |= IXGBE_CTRL_EXT_PFRSTD;
    IXGBE_WRITE_REG(hw, IXGBE_CTRL_EXT, ctrl_ext);
// update setting rx tx for all active vfs
    ixgbe_set_all_vfs(adapter);
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_reinit_locked(adapter: *mut ixgbe_adapter) {
    void ixgbe_reinit_locked(struct ixgbe_adapter *adapter)
    {
// put off any impending NetWatchDogTimeout
    netif_trans_update(adapter.netdev);
    while (test_and_set_bit(__IXGBE_RESETTING, &adapter.state))
    usleep_range(1000, 2000);
    if (adapter.hw.phy.type == ixgbe_phy_fw)
    ixgbe_watchdog_link_is_down(adapter);
    ixgbe_down(adapter);
//
// If SR-IOV enabled then wait a bit before bringing the adapter
// back up to give the VFs time to respond to the reset.  The
// two second wait is based upon the watchdog timer cycle in
// the VF driver.
//
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED)
    msleep(2000);
    ixgbe_up(adapter);
// E610 has no FW event to notify all PFs of an EMPR reset, so
// refresh the FW version here to pick up any new FW version after
// a hardware reset (e.g. EMPR triggered by another PF's devlink
// reload).  ixgbe_refresh_fw_version() updates both hw->flash and
// adapter->eeprom_id so ethtool -i reports the correct string.
//
    if (adapter.hw.mac.type == ixgbe_mac_e610)
    (void)ixgbe_refresh_fw_version(adapter);
    clear_bit(__IXGBE_RESETTING, &adapter.state);
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_up(adapter: *mut ixgbe_adapter) {
    void ixgbe_up(struct ixgbe_adapter *adapter)
    {
// hardware has been reset, we need to reload some things
    ixgbe_configure(adapter);
    ixgbe_up_complete(adapter);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_get_completion_timeout(adapter: *mut ixgbe_adapter) -> c_ulong {
    static unsigned long ixgbe_get_completion_timeout(struct ixgbe_adapter *adapter)
    {
    u16 devctl2;
    pcie_capability_read_word(adapter.pdev, PCI_EXP_DEVCTL2, &devctl2);
    switch (devctl2 & IXGBE_PCIDEVCTRL2_TIMEO_MASK) {
    case IXGBE_PCIDEVCTRL2_17_34s:
    case IXGBE_PCIDEVCTRL2_4_8s:
// For now we cap the upper limit on delay to 2 seconds
// as we end up going up to 34 seconds of delay in worst
// case timeout value.
//
    case IXGBE_PCIDEVCTRL2_1_2s:
    return 2000000ul;	/* 2.0 s */
    case IXGBE_PCIDEVCTRL2_260_520ms:
    return 520000ul;	/* 520 ms */
    case IXGBE_PCIDEVCTRL2_65_130ms:
    return 130000ul;	/* 130 ms */
    case IXGBE_PCIDEVCTRL2_16_32ms:
    return 32000ul;		/* 32 ms */
    case IXGBE_PCIDEVCTRL2_1_2ms:
    return 2000ul;		/* 2 ms */
    case IXGBE_PCIDEVCTRL2_50_100us:
    return 100ul;		/* 100 us */
    case IXGBE_PCIDEVCTRL2_16_32ms_def:
    return 32000ul;		/* 32 ms */
    default:
    break;
    }
// We shouldn't need to hit this path, but just in case default as
// though completion timeout is not supported and support 32ms.
//
    return 32000ul;
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_disable_rx(adapter: *mut ixgbe_adapter) {
    void ixgbe_disable_rx(struct ixgbe_adapter *adapter)
    {
    unsigned long wait_delay, delay_interval;
    struct ixgbe_hw *hw = &adapter.hw;
    int i, wait_loop;
    u32 rxdctl;
// disable receives
    hw.mac.ops.disable_rx(hw);
    if (ixgbe_removed(hw.hw_addr))
    return;
// disable all enabled Rx queues
    for (i = 0; i < adapter.num_rx_queues; i++) {
    struct ixgbe_ring *ring = adapter.rx_ring[i];
    let mut reg_idx: u8 = ring.reg_idx;
    rxdctl = IXGBE_READ_REG(hw, IXGBE_RXDCTL(reg_idx));
    rxdctl &= ~IXGBE_RXDCTL_ENABLE;
    rxdctl |= IXGBE_RXDCTL_SWFLSH;
// write value back with RXDCTL.ENABLE bit cleared
    IXGBE_WRITE_REG(hw, IXGBE_RXDCTL(reg_idx), rxdctl);
    }
// RXDCTL.EN may not change on 82598 if link is down, so skip it
    if (hw.mac.type == ixgbe_mac_82598EB &&
    !(IXGBE_READ_REG(hw, IXGBE_LINKS) & IXGBE_LINKS_UP))
    return;
// Determine our minimum delay interval. We will increase this value
// with each subsequent test. This way if the device returns quickly
// we should spend as little time as possible waiting, however as
// the time increases we will wait for larger periods of time.
//
// The trick here is that we increase the interval using the
// following pattern: 1x 3x 5x 7x 9x 11x 13x 15x 17x 19x. The result
// of that wait is that it totals up to 100x whatever interval we
// choose. Since our minimum wait is 100us we can just divide the
// total timeout by 100 to get our minimum delay interval.
//
    delay_interval = ixgbe_get_completion_timeout(adapter) / 100;
    wait_loop = IXGBE_MAX_RX_DESC_POLL;
    wait_delay = delay_interval;
    while (wait_loop--) {
    usleep_range(wait_delay, wait_delay + 10);
    wait_delay += delay_interval * 2;
    rxdctl = 0;
// OR together the reading of all the active RXDCTL registers,
// and then test the result. We need the disable to complete
// before we start freeing the memory and invalidating the
// DMA mappings.
//
    for (i = 0; i < adapter.num_rx_queues; i++) {
    struct ixgbe_ring *ring = adapter.rx_ring[i];
    let mut reg_idx: u8 = ring.reg_idx;
    rxdctl |= IXGBE_READ_REG(hw, IXGBE_RXDCTL(reg_idx));
    }
    if (!(rxdctl & IXGBE_RXDCTL_ENABLE))
    return;
    }
    e_err(drv,
    "RXDCTL.ENABLE for one or more queues not cleared within the polling period\n");
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_disable_tx(adapter: *mut ixgbe_adapter) {
    void ixgbe_disable_tx(struct ixgbe_adapter *adapter)
    {
    unsigned long wait_delay, delay_interval;
    struct ixgbe_hw *hw = &adapter.hw;
    int i, wait_loop;
    u32 txdctl;
    if (ixgbe_removed(hw.hw_addr))
    return;
// disable all enabled Tx queues
    for (i = 0; i < adapter.num_tx_queues; i++) {
    struct ixgbe_ring *ring = adapter.tx_ring[i];
    let mut reg_idx: u8 = ring.reg_idx;
    IXGBE_WRITE_REG(hw, IXGBE_TXDCTL(reg_idx), IXGBE_TXDCTL_SWFLSH);
    }
// disable all enabled XDP Tx queues
    for (i = 0; i < adapter.num_xdp_queues; i++) {
    struct ixgbe_ring *ring = adapter.xdp_ring[i];
    let mut reg_idx: u8 = ring.reg_idx;
    IXGBE_WRITE_REG(hw, IXGBE_TXDCTL(reg_idx), IXGBE_TXDCTL_SWFLSH);
    }
// If the link is not up there shouldn't be much in the way of
// pending transactions. Those that are left will be flushed out
// when the reset logic goes through the flush sequence to clean out
// the pending Tx transactions.
//
    if (!(IXGBE_READ_REG(hw, IXGBE_LINKS) & IXGBE_LINKS_UP))
    goto dma_engine_disable;
// Determine our minimum delay interval. We will increase this value
// with each subsequent test. This way if the device returns quickly
// we should spend as little time as possible waiting, however as
// the time increases we will wait for larger periods of time.
//
// The trick here is that we increase the interval using the
// following pattern: 1x 3x 5x 7x 9x 11x 13x 15x 17x 19x. The result
// of that wait is that it totals up to 100x whatever interval we
// choose. Since our minimum wait is 100us we can just divide the
// total timeout by 100 to get our minimum delay interval.
//
    delay_interval = ixgbe_get_completion_timeout(adapter) / 100;
    wait_loop = IXGBE_MAX_RX_DESC_POLL;
    wait_delay = delay_interval;
    while (wait_loop--) {
    usleep_range(wait_delay, wait_delay + 10);
    wait_delay += delay_interval * 2;
    txdctl = 0;
// OR together the reading of all the active TXDCTL registers,
// and then test the result. We need the disable to complete
// before we start freeing the memory and invalidating the
// DMA mappings.
//
    for (i = 0; i < adapter.num_tx_queues; i++) {
    struct ixgbe_ring *ring = adapter.tx_ring[i];
    let mut reg_idx: u8 = ring.reg_idx;
    txdctl |= IXGBE_READ_REG(hw, IXGBE_TXDCTL(reg_idx));
    }
    for (i = 0; i < adapter.num_xdp_queues; i++) {
    struct ixgbe_ring *ring = adapter.xdp_ring[i];
    let mut reg_idx: u8 = ring.reg_idx;
    txdctl |= IXGBE_READ_REG(hw, IXGBE_TXDCTL(reg_idx));
    }
    if (!(txdctl & IXGBE_TXDCTL_ENABLE))
    goto dma_engine_disable;
    }
    e_err(drv,
    "TXDCTL.ENABLE for one or more queues not cleared within the polling period\n");
    dma_engine_disable:
// Disable the Tx DMA engine on 82599 and later MAC
    switch (hw.mac.type) {
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    IXGBE_WRITE_REG(hw, IXGBE_DMATXCTL,
    (IXGBE_READ_REG(hw, IXGBE_DMATXCTL) &
    ~IXGBE_DMATXCTL_TE));
    fallthrough;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_reset(adapter: *mut ixgbe_adapter) {
    void ixgbe_reset(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct net_device *netdev = adapter.netdev;
    int err;
    if (ixgbe_removed(hw.hw_addr))
    return;
// lock SFP init bit to prevent race conditions with the watchdog
    while (test_and_set_bit(__IXGBE_IN_SFP_INIT, &adapter.state))
    usleep_range(1000, 2000);
// clear all SFP and link config related flags while holding SFP_INIT
    adapter.flags2 &= ~(IXGBE_FLAG2_SEARCH_FOR_SFP |
    IXGBE_FLAG2_SFP_NEEDS_RESET);
    adapter.flags &= ~IXGBE_FLAG_NEED_LINK_CONFIG;
    err = hw.mac.ops.init_hw(hw);
    switch (err) {
    case 0:
    case -ENOENT:
    case -EOPNOTSUPP:
    break;
    case -EALREADY:
    e_dev_err("primary disable timed out\n");
    break;
    case -EACCES:
// We are running on a pre-production device, log a warning
    e_dev_warn("This device is a pre-production adapter/LOM. "
    "Please be aware there may be issues associated with "
    "your hardware.  If you are experiencing problems "
    "please contact your Intel or hardware "
    "representative who provided you with this "
    "hardware.\n");
    break;
    default:
    e_dev_err("Hardware Error: %d\n", err);
    }
    clear_bit(__IXGBE_IN_SFP_INIT, &adapter.state);
// flush entries out of MAC table
    ixgbe_flush_sw_mac_table(adapter);
    __dev_uc_unsync(netdev, core::ptr::null_mut());
// do not flush user set addresses
    ixgbe_mac_set_default_filter(adapter);
// update SAN MAC vmdq pool selection
    if (hw.mac.san_mac_rar_index)
    hw.mac.ops.set_vmdq_san_mac(hw, VMDQ_P(0));
    if (test_bit(__IXGBE_PTP_RUNNING, &adapter.state))
    ixgbe_ptp_reset(adapter);
    if (hw.phy.ops.set_phy_power) {
    if (!netif_running(adapter.netdev) && !adapter.wol)
    hw.phy.ops.set_phy_power(hw, false);
    else
    hw.phy.ops.set_phy_power(hw, true);
    }
    }
//
// ixgbe_clean_tx_ring - Free Tx Buffers
// @tx_ring: ring to be cleaned
//
#[no_mangle]
unsafe extern "C" fn ixgbe_clean_tx_ring(tx_ring: *mut ixgbe_ring) {
    static void ixgbe_clean_tx_ring(struct ixgbe_ring *tx_ring)
    {
    let mut i: u16 = tx_ring.next_to_clean;
    struct ixgbe_tx_buffer *tx_buffer = &tx_ring.tx_buffer_info[i];
    if (tx_ring.xsk_pool) {
    ixgbe_xsk_clean_tx_ring(tx_ring);
    goto out;
    }
    while (i != tx_ring.next_to_use) {
    union ixgbe_adv_tx_desc *eop_desc, *tx_desc;
// Free all the Tx ring sk_buffs
    if (ring_is_xdp(tx_ring))
    xdp_return_frame(tx_buffer.xdpf);
    else
    dev_kfree_skb_any(tx_buffer.skb);
// unmap skb header data
    dma_unmap_single(tx_ring.dev,
    dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
    DMA_TO_DEVICE);
// check for eop_desc to determine the end of the packet
    eop_desc = tx_buffer.next_to_watch;
    tx_desc = IXGBE_TX_DESC(tx_ring, i);
// unmap remaining buffers
    while (tx_desc != eop_desc) {
    tx_buffer++;
    tx_desc++;
    i++;
    if (unlikely(i == tx_ring.count)) {
    i = 0;
    tx_buffer = tx_ring.tx_buffer_info;
    tx_desc = IXGBE_TX_DESC(tx_ring, 0);
    }
// unmap any remaining paged data
    if (dma_unmap_len(tx_buffer, len))
    dma_unmap_page(tx_ring.dev,
    dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
    DMA_TO_DEVICE);
    }
// move us one more past the eop_desc for start of next pkt
    tx_buffer++;
    i++;
    if (unlikely(i == tx_ring.count)) {
    i = 0;
    tx_buffer = tx_ring.tx_buffer_info;
    }
    }
// reset BQL for queue
    if (!ring_is_xdp(tx_ring))
    netdev_tx_reset_queue(txring_txq(tx_ring));
    out:
// reset next_to_use and next_to_clean
    tx_ring.next_to_use = 0;
    tx_ring.next_to_clean = 0;
    }
//
// ixgbe_clean_all_rx_rings - Free Rx Buffers for all queues
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_clean_all_rx_rings(adapter: *mut ixgbe_adapter) {
    static void ixgbe_clean_all_rx_rings(struct ixgbe_adapter *adapter)
    {
    int i;
    for (i = 0; i < adapter.num_rx_queues; i++)
    ixgbe_clean_rx_ring(adapter.rx_ring[i]);
    }
//
// ixgbe_clean_all_tx_rings - Free Tx Buffers for all queues
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_clean_all_tx_rings(adapter: *mut ixgbe_adapter) {
    static void ixgbe_clean_all_tx_rings(struct ixgbe_adapter *adapter)
    {
    int i;
    for (i = 0; i < adapter.num_tx_queues; i++)
    ixgbe_clean_tx_ring(adapter.tx_ring[i]);
    for (i = 0; i < adapter.num_xdp_queues; i++)
    ixgbe_clean_tx_ring(adapter.xdp_ring[i]);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_fdir_filter_exit(adapter: *mut ixgbe_adapter) {
    static void ixgbe_fdir_filter_exit(struct ixgbe_adapter *adapter)
    {
    struct hlist_node *node2;
    struct ixgbe_fdir_filter *filter;
    spin_lock(&adapter.fdir_perfect_lock);
    hlist_for_each_entry_safe(filter, node2,
    &adapter.fdir_filter_list, fdir_node) {
    hlist_del(&filter.fdir_node);
    kfree(filter);
    }
    adapter.fdir_filter_count = 0;
    spin_unlock(&adapter.fdir_perfect_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_down(adapter: *mut ixgbe_adapter) {
    void ixgbe_down(struct ixgbe_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct ixgbe_hw *hw = &adapter.hw;
    int i;
// signal that we are down to the interrupt handler
    if (test_and_set_bit(__IXGBE_DOWN, &adapter.state))
    return; /* do nothing if already down */
// Shut off incoming Tx traffic
    netif_tx_stop_all_queues(netdev);
// call carrier off first to avoid false dev_watchdog timeouts
    netif_carrier_off(netdev);
    netif_tx_disable(netdev);
// Disable Rx
    ixgbe_disable_rx(adapter);
// synchronize_rcu() needed for pending XDP buffers to drain
    if (adapter.xdp_ring[0])
    synchronize_rcu();
    ixgbe_irq_disable(adapter);
    ixgbe_napi_disable_all(adapter);
    clear_bit(__IXGBE_RESET_REQUESTED, &adapter.state);
    adapter.flags2 &= ~IXGBE_FLAG2_FDIR_REQUIRES_REINIT;
    adapter.flags &= ~IXGBE_FLAG_NEED_LINK_UPDATE;
    timer_delete_sync(&adapter.service_timer);
    if (adapter.num_vfs) {
// Clear EITR Select mapping
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_EITRSEL, 0);
// Mark all the VFs as inactive
    for (i = 0 ; i < adapter.num_vfs; i++)
    adapter.vfinfo[i].clear_to_send = false;
// update setting rx tx for all active vfs
    ixgbe_set_all_vfs(adapter);
    }
// disable transmits in the hardware now that interrupts are off
    ixgbe_disable_tx(adapter);
    if (!pci_channel_offline(adapter.pdev))
    ixgbe_reset(adapter);
// power down the optics for 82599 SFP+ fiber
    if (hw.mac.ops.disable_tx_laser)
    hw.mac.ops.disable_tx_laser(hw);
    ixgbe_clean_all_tx_rings(adapter);
    ixgbe_clean_all_rx_rings(adapter);
    if (adapter.hw.mac.type == ixgbe_mac_e610)
    ixgbe_disable_link_status_events(adapter);
    }
//
// ixgbe_set_eee_capable - helper function to determine EEE support on X550
// and E610
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_set_eee_capable(adapter: *mut ixgbe_adapter) {
    static void ixgbe_set_eee_capable(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    switch (hw.device_id) {
    case IXGBE_DEV_ID_X550EM_A_1G_T:
    case IXGBE_DEV_ID_X550EM_A_1G_T_L:
    if (!hw.phy.eee_speeds_supported)
    break;
    adapter.flags2 |= IXGBE_FLAG2_EEE_CAPABLE;
    if (!hw.phy.eee_speeds_advertised)
    break;
    adapter.flags2 |= IXGBE_FLAG2_EEE_ENABLED;
    break;
    case IXGBE_DEV_ID_E610_BACKPLANE:
    case IXGBE_DEV_ID_E610_SFP:
    case IXGBE_DEV_ID_E610_10G_T:
    case IXGBE_DEV_ID_E610_2_5G_T:
    if (hw.dev_caps.common_cap.eee_support &&
    hw.phy.eee_speeds_supported) {
    adapter.flags2 |= IXGBE_FLAG2_EEE_CAPABLE;
// For E610 adapters EEE should be enabled by default
// if the feature is supported by FW.
//
    adapter.flags2 |= IXGBE_FLAG2_EEE_ENABLED;
    break;
    }
    fallthrough;
    default:
    adapter.flags2 &= ~IXGBE_FLAG2_EEE_CAPABLE;
    adapter.flags2 &= ~IXGBE_FLAG2_EEE_ENABLED;
    break;
    }
    }
//
// ixgbe_tx_timeout - Respond to a Tx Hang
// @netdev: network interface device structure
// @txqueue: queue number that timed out
//
#[no_mangle]
unsafe extern "C" fn ixgbe_tx_timeout(netdev: *mut net_device, txqueue: unsigned int __always_unused) {
    static void ixgbe_tx_timeout(struct net_device *netdev, unsigned int __always_unused txqueue)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
// Do the reset outside of interrupt context
    ixgbe_tx_timeout_reset(adapter);
    }

#[no_mangle]
unsafe extern "C" fn ixgbe_init_dcb(adapter: *mut ixgbe_adapter) {
    static void ixgbe_init_dcb(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct tc_configuration *tc;
    int j;
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    case ixgbe_mac_82599EB:
    adapter.dcb_cfg.num_tcs.pg_tcs = MAX_TRAFFIC_CLASS;
    adapter.dcb_cfg.num_tcs.pfc_tcs = MAX_TRAFFIC_CLASS;
    break;
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_e610:
    adapter.dcb_cfg.num_tcs.pg_tcs = X540_TRAFFIC_CLASS;
    adapter.dcb_cfg.num_tcs.pfc_tcs = X540_TRAFFIC_CLASS;
    break;
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    default:
    adapter.dcb_cfg.num_tcs.pg_tcs = DEF_TRAFFIC_CLASS;
    adapter.dcb_cfg.num_tcs.pfc_tcs = DEF_TRAFFIC_CLASS;
    break;
    }
// Configure DCB traffic classes
    for (j = 0; j < MAX_TRAFFIC_CLASS; j++) {
    tc = &adapter.dcb_cfg.tc_config[j];
    tc.path[DCB_TX_CONFIG].bwg_id = 0;
    tc.path[DCB_TX_CONFIG].bwg_percent = 12 + (j & 1);
    tc.path[DCB_RX_CONFIG].bwg_id = 0;
    tc.path[DCB_RX_CONFIG].bwg_percent = 12 + (j & 1);
    tc.dcb_pfc = pfc_disabled;
    }
// Initialize default user to priority mapping, UPx->TC0
    tc = &adapter.dcb_cfg.tc_config[0];
    tc.path[DCB_TX_CONFIG].up_to_tc_bitmap = 0xFF;
    tc.path[DCB_RX_CONFIG].up_to_tc_bitmap = 0xFF;
    adapter.dcb_cfg.bw_percentage[DCB_TX_CONFIG][0] = 100;
    adapter.dcb_cfg.bw_percentage[DCB_RX_CONFIG][0] = 100;
    adapter.dcb_cfg.pfc_mode_enable = false;
    adapter.dcb_set_bitmap = 0x00;
    if (adapter.flags & IXGBE_FLAG_DCB_CAPABLE)
    adapter.dcbx_cap = DCB_CAP_DCBX_HOST | DCB_CAP_DCBX_VER_CEE;
    memcpy(&adapter.temp_dcb_cfg, &adapter.dcb_cfg,
    sizeof(adapter.temp_dcb_cfg));
    }

//
// ixgbe_sw_init - Initialize general software structures (struct ixgbe_adapter)
// @adapter: board private structure to initialize
// @ii: pointer to ixgbe_info for device
//
// ixgbe_sw_init initializes the Adapter private data structure.
// Fields are initialized based on PCI device information and
// OS network device settings (MTU size).
//
    static int ixgbe_sw_init(struct ixgbe_adapter *adapter,
    const struct ixgbe_info *ii)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct pci_dev *pdev = adapter.pdev;
    unsigned int rss, fdir;
    u32 fwsm;
    int i;
// PCI config space info
    hw.vendor_id = pdev.vendor;
    hw.device_id = pdev.device;
    hw.revision_id = pdev.revision;
    hw.subsystem_vendor_id = pdev.subsystem_vendor;
    hw.subsystem_device_id = pdev.subsystem_device;
    hw.mac.max_link_up_time = IXGBE_LINK_UP_TIME;
// get_invariants needs the device IDs
    ii.get_invariants(hw);
// Set common capability flags and settings
    rss = min_t(int, ixgbe_max_rss_indices(adapter), num_online_cpus());
    adapter.ring_feature[RING_F_RSS].limit = rss;
    adapter.flags2 |= IXGBE_FLAG2_RSC_CAPABLE;
    adapter.max_q_vectors = MAX_Q_VECTORS_82599;
    adapter.atr_sample_rate = 20;
    fdir = min_t(int, IXGBE_MAX_FDIR_INDICES, num_online_cpus());
    adapter.ring_feature[RING_F_FDIR].limit = fdir;
    adapter.fdir_pballoc = IXGBE_FDIR_PBALLOC_64K;
    adapter.ring_feature[RING_F_VMDQ].limit = 1;

    adapter.flags |= IXGBE_FLAG_DCA_CAPABLE;

    adapter.flags |= IXGBE_FLAG_DCB_CAPABLE;
    adapter.flags &= ~IXGBE_FLAG_DCB_ENABLED;

    adapter.flags |= IXGBE_FLAG_FCOE_CAPABLE;
    adapter.flags &= ~IXGBE_FLAG_FCOE_ENABLED;

// Default traffic class to use for FCoE
    adapter.fcoe.up = IXGBE_FCOE_DEFTC;

// initialize static ixgbe jump table entries
    adapter.jump_tables[0] = kzalloc_obj(*adapter.jump_tables[0]);
    if (!adapter.jump_tables[0])
    return -ENOMEM;
    adapter.jump_tables[0].mat = ixgbe_ipv4_fields;
    for (i = 1; i < IXGBE_MAX_LINK_HANDLE; i++)
    adapter.jump_tables[i] = core::ptr::null_mut();
    adapter.mac_table = kzalloc_objs(struct ixgbe_mac_addr,
    hw.mac.num_rar_entries);
    if (!adapter.mac_table)
    return -ENOMEM;
    if (ixgbe_init_rss_key(adapter))
    return -ENOMEM;
    adapter.af_xdp_zc_qps = bitmap_zalloc(IXGBE_MAX_XDP_QS, GFP_KERNEL);
    if (!adapter.af_xdp_zc_qps)
    return -ENOMEM;
// Set MAC specific capability flags and exceptions
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    adapter.flags2 &= ~IXGBE_FLAG2_RSC_CAPABLE;
    if (hw.device_id == IXGBE_DEV_ID_82598AT)
    adapter.flags |= IXGBE_FLAG_FAN_FAIL_CAPABLE;
    adapter.max_q_vectors = MAX_Q_VECTORS_82598;
    adapter.ring_feature[RING_F_FDIR].limit = 0;
    adapter.atr_sample_rate = 0;
    adapter.fdir_pballoc = 0;

    adapter.flags &= ~IXGBE_FLAG_FCOE_CAPABLE;
    adapter.flags &= ~IXGBE_FLAG_FCOE_ENABLED;

    adapter.fcoe.up = 0;

    break;
    case ixgbe_mac_82599EB:
    if (hw.device_id == IXGBE_DEV_ID_82599_T3_LOM)
    adapter.flags2 |= IXGBE_FLAG2_TEMP_SENSOR_CAPABLE;
    break;
    case ixgbe_mac_X540:
    fwsm = IXGBE_READ_REG(hw, IXGBE_FWSM(hw));
    if (fwsm & IXGBE_FWSM_TS_ENABLED)
    adapter.flags2 |= IXGBE_FLAG2_TEMP_SENSOR_CAPABLE;
    break;
    case ixgbe_mac_x550em_a:
    switch (hw.device_id) {
    case IXGBE_DEV_ID_X550EM_A_1G_T:
    case IXGBE_DEV_ID_X550EM_A_1G_T_L:
    adapter.flags2 |= IXGBE_FLAG2_TEMP_SENSOR_CAPABLE;
    break;
    default:
    break;
    }
    fallthrough;
    case ixgbe_mac_X550EM_x:

    adapter.flags &= ~IXGBE_FLAG_DCB_CAPABLE;

    adapter.flags &= ~IXGBE_FLAG_FCOE_CAPABLE;

    adapter.fcoe.up = 0;

    fallthrough;
    case ixgbe_mac_X550:
    if (hw.mac.type == ixgbe_mac_X550)
    adapter.flags2 |= IXGBE_FLAG2_TEMP_SENSOR_CAPABLE;

    adapter.flags &= ~IXGBE_FLAG_DCA_CAPABLE;

    break;
    default:
    break;
    }
// Make sure the SWFW semaphore is in a valid state
    if (hw.mac.ops.init_swfw_sync)
    hw.mac.ops.init_swfw_sync(hw);
    if (hw.mac.type == ixgbe_mac_e610)
    mutex_init(&hw.aci.lock);

// FCoE support exists, always init the FCoE lock
    spin_lock_init(&adapter.fcoe.lock);

// n-tuple support exists, always init our spinlock
    spin_lock_init(&adapter.fdir_perfect_lock);
// init spinlock to avoid concurrency of VF resources
    spin_lock_init(&adapter.vfs_lock);

    ixgbe_init_dcb(adapter);

    ixgbe_init_ipsec_offload(adapter);
// default flow control settings
    hw.fc.requested_mode = ixgbe_fc_full;
    hw.fc.current_mode = ixgbe_fc_full;	/* init for ethtool output */
    ixgbe_pbthresh_setup(adapter);
    hw.fc.pause_time = IXGBE_DEFAULT_FCPAUSE;
    hw.fc.send_xon = true;
    hw.fc.disable_fc_autoneg = ixgbe_device_supports_autoneg_fc(hw);

    if (max_vfs > 0)
    e_dev_warn("Enabling SR-IOV VFs using the max_vfs module parameter is deprecated - please use the pci sysfs interface instead.\n");
// assign number of SR-IOV VFs
    if (hw.mac.type != ixgbe_mac_82598EB) {
    if (max_vfs > IXGBE_MAX_VFS_DRV_LIMIT) {
    max_vfs = 0;
    e_dev_warn("max_vfs parameter out of range. Not assigning any SR-IOV VFs\n");
    }
    }

// enable itr by default in dynamic mode
    adapter.rx_itr_setting = 1;
    adapter.tx_itr_setting = 1;
// set default ring sizes
    adapter.tx_ring_count = IXGBE_DEFAULT_TXD;
    adapter.rx_ring_count = IXGBE_DEFAULT_RXD;
// set default work limits
    adapter.tx_work_limit = IXGBE_DEFAULT_TX_WORK;
// initialize eeprom parameters
    if (hw.eeprom.ops.init_params(hw)) {
    e_dev_err("EEPROM initialization failed\n");
    return -EIO;
    }
// PF holds first pool slot
    set_bit(0, adapter.fwd_bitmask);
    set_bit(__IXGBE_DOWN, &adapter.state);
// enable locking for XDP_TX if we have more CPUs than queues
    if (nr_cpu_ids > IXGBE_MAX_XDP_QS)
    static_branch_enable(&ixgbe_xdp_locking_key);
    return 0;
    }
//
// ixgbe_setup_tx_resources - allocate Tx resources (Descriptors)
// @tx_ring:    tx descriptor ring (for a specific queue) to setup
//
// Return 0 on success, negative on failure
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_setup_tx_resources(tx_ring: *mut ixgbe_ring) -> c_int {
    int ixgbe_setup_tx_resources(struct ixgbe_ring *tx_ring)
    {
    struct device *dev = tx_ring.dev;
    let mut orig_node: c_int = dev_to_node(dev);
    let mut ring_node: c_int = NUMA_NO_NODE;
    int size;
    size = sizeof(struct ixgbe_tx_buffer) * tx_ring.count;
    if (tx_ring.q_vector)
    ring_node = tx_ring.q_vector.numa_node;
    tx_ring.tx_buffer_info = vmalloc_node(size, ring_node);
    if (!tx_ring.tx_buffer_info)
    tx_ring.tx_buffer_info = vmalloc(size);
    if (!tx_ring.tx_buffer_info)
    goto err;
// round up to nearest 4K
    tx_ring.size = tx_ring.count * sizeof(union ixgbe_adv_tx_desc);
    tx_ring.size = ALIGN(tx_ring.size, 4096);
    set_dev_node(dev, ring_node);
    tx_ring.desc = dma_alloc_coherent(dev,
    tx_ring.size,
    &tx_ring.dma,
    GFP_KERNEL);
    set_dev_node(dev, orig_node);
    if (!tx_ring.desc)
    tx_ring.desc = dma_alloc_coherent(dev, tx_ring.size,
    &tx_ring.dma, GFP_KERNEL);
    if (!tx_ring.desc)
    goto err;
    tx_ring.next_to_use = 0;
    tx_ring.next_to_clean = 0;
    return 0;
    err:
    vfree(tx_ring.tx_buffer_info);
    tx_ring.tx_buffer_info = core::ptr::null_mut();
    dev_err(dev, "Unable to allocate memory for the Tx descriptor ring\n");
    return -ENOMEM;
    }
//
// ixgbe_setup_all_tx_resources - allocate all queues Tx resources
// @adapter: board private structure
//
// If this function returns with an error, then it's possible one or
// more of the rings is populated (while the rest are not).  It is the
// callers duty to clean those orphaned rings.
//
// Return 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_all_tx_resources(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_setup_all_tx_resources(struct ixgbe_adapter *adapter)
    {
    int i, j = 0, err = 0;
    for (i = 0; i < adapter.num_tx_queues; i++) {
    err = ixgbe_setup_tx_resources(adapter.tx_ring[i]);
    if (!err)
    continue;
    e_err(probe, "Allocation for Tx Queue %u failed\n", i);
    goto err_setup_tx;
    }
    for (j = 0; j < adapter.num_xdp_queues; j++) {
    err = ixgbe_setup_tx_resources(adapter.xdp_ring[j]);
    if (!err)
    continue;
    e_err(probe, "Allocation for Tx Queue %u failed\n", j);
    goto err_setup_tx;
    }
    return 0;
    err_setup_tx:
// rewind the index freeing the rings as we go
    while (j--)
    ixgbe_free_tx_resources(adapter.xdp_ring[j]);
    while (i--)
    ixgbe_free_tx_resources(adapter.tx_ring[i]);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_rx_napi_id(rx_ring: *mut ixgbe_ring) -> c_int {
    static int ixgbe_rx_napi_id(struct ixgbe_ring *rx_ring)
    {
    struct ixgbe_q_vector *q_vector = rx_ring.q_vector;
    return q_vector ? q_vector.napi.napi_id : 0;
    }
//
// ixgbe_setup_rx_resources - allocate Rx resources (Descriptors)
// @adapter: pointer to ixgbe_adapter
// @rx_ring:    rx descriptor ring (for a specific queue) to setup
//
// Returns 0 on success, negative on failure
//
    int ixgbe_setup_rx_resources(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *rx_ring)
    {
    struct device *dev = rx_ring.dev;
    let mut orig_node: c_int = dev_to_node(dev);
    let mut ring_node: c_int = NUMA_NO_NODE;
    int size;
    size = sizeof(struct ixgbe_rx_buffer) * rx_ring.count;
    if (rx_ring.q_vector)
    ring_node = rx_ring.q_vector.numa_node;
    rx_ring.rx_buffer_info = vmalloc_node(size, ring_node);
    if (!rx_ring.rx_buffer_info)
    rx_ring.rx_buffer_info = vmalloc(size);
    if (!rx_ring.rx_buffer_info)
    goto err;
// Round up to nearest 4K
    rx_ring.size = rx_ring.count * sizeof(union ixgbe_adv_rx_desc);
    rx_ring.size = ALIGN(rx_ring.size, 4096);
    set_dev_node(dev, ring_node);
    rx_ring.desc = dma_alloc_coherent(dev,
    rx_ring.size,
    &rx_ring.dma,
    GFP_KERNEL);
    set_dev_node(dev, orig_node);
    if (!rx_ring.desc)
    rx_ring.desc = dma_alloc_coherent(dev, rx_ring.size,
    &rx_ring.dma, GFP_KERNEL);
    if (!rx_ring.desc)
    goto err;
    rx_ring.next_to_clean = 0;
    rx_ring.next_to_use = 0;
// XDP RX-queue info
    if (xdp_rxq_info_reg(&rx_ring.xdp_rxq, adapter.netdev,
    rx_ring.queue_index, ixgbe_rx_napi_id(rx_ring)) < 0)
    goto err;
    WRITE_ONCE(rx_ring.xdp_prog, adapter.xdp_prog);
    return 0;
    err:
    vfree(rx_ring.rx_buffer_info);
    rx_ring.rx_buffer_info = core::ptr::null_mut();
    dev_err(dev, "Unable to allocate memory for the Rx descriptor ring\n");
    return -ENOMEM;
    }
//
// ixgbe_setup_all_rx_resources - allocate all queues Rx resources
// @adapter: board private structure
//
// If this function returns with an error, then it's possible one or
// more of the rings is populated (while the rest are not).  It is the
// callers duty to clean those orphaned rings.
//
// Return 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_setup_all_rx_resources(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_setup_all_rx_resources(struct ixgbe_adapter *adapter)
    {
    int i, err = 0;
    for (i = 0; i < adapter.num_rx_queues; i++) {
    err = ixgbe_setup_rx_resources(adapter, adapter.rx_ring[i]);
    if (!err)
    continue;
    e_err(probe, "Allocation for Rx Queue %u failed\n", i);
    goto err_setup_rx;
    }

    err = ixgbe_setup_fcoe_ddp_resources(adapter);
    if (!err)

    return 0;
    err_setup_rx:
// rewind the index freeing the rings as we go
    while (i--)
    ixgbe_free_rx_resources(adapter.rx_ring[i]);
    return err;
    }
//
// ixgbe_free_tx_resources - Free Tx Resources per Queue
// @tx_ring: Tx descriptor ring for a specific queue
//
// Free all transmit software resources
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_free_tx_resources(tx_ring: *mut ixgbe_ring) {
    void ixgbe_free_tx_resources(struct ixgbe_ring *tx_ring)
    {
    ixgbe_clean_tx_ring(tx_ring);
    vfree(tx_ring.tx_buffer_info);
    tx_ring.tx_buffer_info = core::ptr::null_mut();
// if not set, then don't free
    if (!tx_ring.desc)
    return;
    dma_free_coherent(tx_ring.dev, tx_ring.size,
    tx_ring.desc, tx_ring.dma);
    tx_ring.desc = core::ptr::null_mut();
    }
//
// ixgbe_free_all_tx_resources - Free Tx Resources for All Queues
// @adapter: board private structure
//
// Free all transmit software resources
//
#[no_mangle]
unsafe extern "C" fn ixgbe_free_all_tx_resources(adapter: *mut ixgbe_adapter) {
    static void ixgbe_free_all_tx_resources(struct ixgbe_adapter *adapter)
    {
    int i;
    for (i = 0; i < adapter.num_tx_queues; i++)
    if (adapter.tx_ring[i].desc)
    ixgbe_free_tx_resources(adapter.tx_ring[i]);
    for (i = 0; i < adapter.num_xdp_queues; i++)
    if (adapter.xdp_ring[i].desc)
    ixgbe_free_tx_resources(adapter.xdp_ring[i]);
    }
//
// ixgbe_free_rx_resources - Free Rx Resources
// @rx_ring: ring to clean the resources from
//
// Free all receive software resources
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_free_rx_resources(rx_ring: *mut ixgbe_ring) {
    void ixgbe_free_rx_resources(struct ixgbe_ring *rx_ring)
    {
    ixgbe_clean_rx_ring(rx_ring);
    rx_ring.xdp_prog = core::ptr::null_mut();
    xdp_rxq_info_unreg(&rx_ring.xdp_rxq);
    vfree(rx_ring.rx_buffer_info);
    rx_ring.rx_buffer_info = core::ptr::null_mut();
// if not set, then don't free
    if (!rx_ring.desc)
    return;
    dma_free_coherent(rx_ring.dev, rx_ring.size,
    rx_ring.desc, rx_ring.dma);
    rx_ring.desc = core::ptr::null_mut();
    }
//
// ixgbe_free_all_rx_resources - Free Rx Resources for All Queues
// @adapter: board private structure
//
// Free all receive software resources
//
#[no_mangle]
unsafe extern "C" fn ixgbe_free_all_rx_resources(adapter: *mut ixgbe_adapter) {
    static void ixgbe_free_all_rx_resources(struct ixgbe_adapter *adapter)
    {
    int i;

    ixgbe_free_fcoe_ddp_resources(adapter);

    for (i = 0; i < adapter.num_rx_queues; i++)
    if (adapter.rx_ring[i].desc)
    ixgbe_free_rx_resources(adapter.rx_ring[i]);
    }
//
// ixgbe_max_xdp_frame_size - returns the maximum allowed frame size for XDP
// @adapter: device handle, pointer to adapter
//
#[no_mangle]
unsafe extern "C" fn ixgbe_max_xdp_frame_size(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_max_xdp_frame_size(struct ixgbe_adapter *adapter)
    {
    if (PAGE_SIZE >= 8192 || adapter.flags2 & IXGBE_FLAG2_RX_LEGACY)
    return IXGBE_RXBUFFER_2K;
    else
    return IXGBE_RXBUFFER_3K;
    }
//
// ixgbe_change_mtu - Change the Maximum Transfer Unit
// @netdev: network interface device structure
// @new_mtu: new value for maximum frame size
//
// Returns 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int {
    static int ixgbe_change_mtu(struct net_device *netdev, int new_mtu)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    if (ixgbe_enabled_xdp_adapter(adapter)) {
    let mut new_frame_size: c_int = new_mtu + IXGBE_PKT_HDR_PAD;
    if (new_frame_size > ixgbe_max_xdp_frame_size(adapter)) {
    e_warn(probe, "Requested MTU size is not supported with XDP\n");
    return -EINVAL;
    }
    }
//
// For 82599EB we cannot allow legacy VFs to enable their receive
// paths when MTU greater than 1500 is configured.  So display a
// warning that legacy VFs will be disabled.
//
    if ((adapter.flags & IXGBE_FLAG_SRIOV_ENABLED) &&
    (adapter.hw.mac.type == ixgbe_mac_82599EB) &&
    (new_mtu > ETH_DATA_LEN))
    e_warn(probe, "Setting MTU > 1500 will disable legacy VFs\n");
    netdev_dbg(netdev, "changing MTU from %d to %d\n",
    netdev.mtu, new_mtu);
// must set new MTU before calling down or up
    WRITE_ONCE(netdev.mtu, new_mtu);
    if (netif_running(netdev))
    ixgbe_reinit_locked(adapter);
    return 0;
    }
//
// ixgbe_open - Called when a network interface is made active
// @netdev: network interface device structure
//
// Returns 0 on success, negative value on failure
//
// The open entry point is called when a network interface is made
// active by the system (IFF_UP).  At this point all resources needed
// for transmit and receive operations are allocated, the interrupt
// handler is registered with the OS, the watchdog timer is started,
// and the stack is notified that the interface is ready.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_open(netdev: *mut net_device) -> c_int {
    int ixgbe_open(struct net_device *netdev)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    struct ixgbe_hw *hw = &adapter.hw;
    int err, queues;
// disallow open during test
    if (test_bit(__IXGBE_TESTING, &adapter.state))
    return -EBUSY;
    netif_carrier_off(netdev);
// allocate transmit descriptors
    err = ixgbe_setup_all_tx_resources(adapter);
    if (err)
    goto err_setup_tx;
// allocate receive descriptors
    err = ixgbe_setup_all_rx_resources(adapter);
    if (err)
    goto err_setup_rx;
    ixgbe_configure(adapter);
    err = ixgbe_request_irq(adapter);
    if (err)
    goto err_req_irq;
// Notify the stack of the actual queue counts.
    queues = adapter.num_tx_queues;
    err = netif_set_real_num_tx_queues(netdev, queues);
    if (err)
    goto err_set_queues;
    queues = adapter.num_rx_queues;
    err = netif_set_real_num_rx_queues(netdev, queues);
    if (err)
    goto err_set_queues;
    ixgbe_ptp_init(adapter);
    ixgbe_up_complete(adapter);
    udp_tunnel_nic_reset_ntf(netdev);
    if (adapter.hw.mac.type == ixgbe_mac_e610) {
    let mut err: c_int = ixgbe_update_link_info(&adapter.hw);
    if (err)
    e_dev_err("Failed to update link info, err %d.\n", err);
    ixgbe_check_link_cfg_err(adapter,
    adapter.hw.link.link_info.link_cfg_err);
    err = ixgbe_non_sfp_link_config(&adapter.hw);
    if (err)
    e_dev_err("Link setup failed, err %d.\n", err);
    }
    return 0;
    err_set_queues:
    ixgbe_free_irq(adapter);
    err_req_irq:
    ixgbe_free_all_rx_resources(adapter);
    if (hw.phy.ops.set_phy_power && !adapter.wol)
    hw.phy.ops.set_phy_power(&adapter.hw, false);
    err_setup_rx:
    ixgbe_free_all_tx_resources(adapter);
    err_setup_tx:
    ixgbe_reset(adapter);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_close_suspend(adapter: *mut ixgbe_adapter) {
    static void ixgbe_close_suspend(struct ixgbe_adapter *adapter)
    {
    ixgbe_ptp_suspend(adapter);
    if (adapter.hw.phy.ops.enter_lplu) {
    adapter.hw.phy.reset_disable = true;
    ixgbe_down(adapter);
    adapter.hw.phy.ops.enter_lplu(&adapter.hw);
    adapter.hw.phy.reset_disable = false;
    } else {
    ixgbe_down(adapter);
    }
    ixgbe_free_irq(adapter);
    ixgbe_free_all_tx_resources(adapter);
    ixgbe_free_all_rx_resources(adapter);
    }
//
// ixgbe_close - Disables a network interface
// @netdev: network interface device structure
//
// Returns 0, this is not allowed to fail
//
// The close entry point is called when an interface is de-activated
// by the OS.  The hardware is still under the drivers control, but
// needs to be disabled.  A global MAC reset is issued to stop the
// hardware, and all transmit and receive resources are freed.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_close(netdev: *mut net_device) -> c_int {
    int ixgbe_close(struct net_device *netdev)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    ixgbe_ptp_stop(adapter);
    if (netif_device_present(netdev))
    ixgbe_close_suspend(adapter);
    ixgbe_fdir_filter_exit(adapter);
    ixgbe_release_hw_control(adapter);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_resume(dev_d: *mut device) -> c_int {
    static int ixgbe_resume(struct device *dev_d)
    {
    struct pci_dev *pdev = to_pci_dev(dev_d);
    struct ixgbe_adapter *adapter = pci_get_drvdata(pdev);
    struct net_device *netdev = adapter.netdev;
    u32 err;
    adapter.hw.hw_addr = adapter.io_addr;
    err = pci_enable_device_mem(pdev);
    if (err) {
    e_dev_err("Cannot enable PCI device from suspend\n");
    return err;
    }
    smp_mb__before_atomic();
    clear_bit(__IXGBE_DISABLED, &adapter.state);
    pci_set_master(pdev);
    device_wakeup_disable(dev_d);
    ixgbe_reset(adapter);
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_WUS, ~0);
    rtnl_lock();
    err = ixgbe_init_interrupt_scheme(adapter);
    if (!err && netif_running(netdev))
    err = ixgbe_open(netdev);
    if (!err)
    netif_device_attach(netdev);
    rtnl_unlock();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __ixgbe_shutdown(pdev: *mut pci_dev, enable_wake: *mut bool) -> c_int {
    static int __ixgbe_shutdown(struct pci_dev *pdev, bool *enable_wake)
    {
    struct ixgbe_adapter *adapter = pci_get_drvdata(pdev);
    struct net_device *netdev = adapter.netdev;
    struct ixgbe_hw *hw = &adapter.hw;
    u32 ctrl;
    let mut wufc: u32 = adapter.wol;
    rtnl_lock();
    netif_device_detach(netdev);
    if (netif_running(netdev))
    ixgbe_close_suspend(adapter);
    ixgbe_clear_interrupt_scheme(adapter);
    rtnl_unlock();
    if (hw.mac.ops.stop_link_on_d3)
    hw.mac.ops.stop_link_on_d3(hw);
    if (wufc) {
    u32 fctrl;
    ixgbe_set_rx_mode(netdev);
// enable the optics for 82599 SFP+ fiber as we can WoL
    if (hw.mac.ops.enable_tx_laser)
    hw.mac.ops.enable_tx_laser(hw);
// enable the reception of multicast packets
    fctrl = IXGBE_READ_REG(hw, IXGBE_FCTRL);
    fctrl |= IXGBE_FCTRL_MPE;
    IXGBE_WRITE_REG(hw, IXGBE_FCTRL, fctrl);
    ctrl = IXGBE_READ_REG(hw, IXGBE_CTRL);
    ctrl |= IXGBE_CTRL_GIO_DIS;
    IXGBE_WRITE_REG(hw, IXGBE_CTRL, ctrl);
    IXGBE_WRITE_REG(hw, IXGBE_WUFC, wufc);
    } else {
    IXGBE_WRITE_REG(hw, IXGBE_WUC, 0);
    IXGBE_WRITE_REG(hw, IXGBE_WUFC, 0);
    }
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    pci_wake_from_d3(pdev, false);
    break;
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    pci_wake_from_d3(pdev, !!wufc);
    break;
    default:
    break;
    }
// enable_wake = !!wufc;
    if (hw.phy.ops.set_phy_power && !*enable_wake)
    hw.phy.ops.set_phy_power(hw, false);
    ixgbe_release_hw_control(adapter);
    if (!test_and_set_bit(__IXGBE_DISABLED, &adapter.state))
    pci_disable_device(pdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_suspend(dev_d: *mut device) -> c_int {
    static int ixgbe_suspend(struct device *dev_d)
    {
    struct pci_dev *pdev = to_pci_dev(dev_d);
    int retval;
    bool wake;
    retval = __ixgbe_shutdown(pdev, &wake);
    device_set_wakeup_enable(dev_d, wake);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_shutdown(pdev: *mut pci_dev) {
    static void ixgbe_shutdown(struct pci_dev *pdev)
    {
    bool wake;
    __ixgbe_shutdown(pdev, &wake);
    if (system_state == SYSTEM_POWER_OFF) {
    pci_wake_from_d3(pdev, wake);
    pci_set_power_state(pdev, PCI_D3hot);
    }
    }
//
// ixgbe_update_stats - Update the board statistics counters.
// @adapter: board private structure
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_update_stats(adapter: *mut ixgbe_adapter) {
    void ixgbe_update_stats(struct ixgbe_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct ixgbe_hw *hw = &adapter.hw;
    struct ixgbe_hw_stats *hwstats = &adapter.stats;
    let mut total_mpc: u64 = 0;
    u32 i, missed_rx = 0, mpc, bprc, lxon, lxoff, xon_off_tot;
    let mut non_eop_descs: u64 = 0, restart_queue = 0, tx_busy = 0;
    let mut alloc_rx_page_failed: u64 = 0, alloc_rx_buff_failed = 0;
    let mut alloc_rx_page: u64 = 0;
    let mut bytes: u64 = 0, packets = 0, hw_csum_rx_error = 0;
    if (test_bit(__IXGBE_DOWN, &adapter.state) ||
    test_bit(__IXGBE_RESETTING, &adapter.state))
    return;
    if (adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED) {
    let mut rsc_count: u64 = 0;
    let mut rsc_flush: u64 = 0;
    for (i = 0; i < adapter.num_rx_queues; i++) {
    rsc_count += adapter.rx_ring[i].rx_stats.rsc_count;
    rsc_flush += adapter.rx_ring[i].rx_stats.rsc_flush;
    }
    adapter.rsc_total_count = rsc_count;
    adapter.rsc_total_flush = rsc_flush;
    }
    for (i = 0; i < adapter.num_rx_queues; i++) {
    struct ixgbe_ring *rx_ring = READ_ONCE(adapter.rx_ring[i]);
    if (!rx_ring)
    continue;
    non_eop_descs += rx_ring.rx_stats.non_eop_descs;
    alloc_rx_page += rx_ring.rx_stats.alloc_rx_page;
    alloc_rx_page_failed += rx_ring.rx_stats.alloc_rx_page_failed;
    alloc_rx_buff_failed += rx_ring.rx_stats.alloc_rx_buff_failed;
    hw_csum_rx_error += rx_ring.rx_stats.csum_err;
    bytes += rx_ring.stats.bytes;
    packets += rx_ring.stats.packets;
    }
    adapter.non_eop_descs = non_eop_descs;
    adapter.alloc_rx_page = alloc_rx_page;
    adapter.alloc_rx_page_failed = alloc_rx_page_failed;
    adapter.alloc_rx_buff_failed = alloc_rx_buff_failed;
    adapter.hw_csum_rx_error = hw_csum_rx_error;
    netdev.stats.rx_bytes = bytes;
    netdev.stats.rx_packets = packets;
    bytes = 0;
    packets = 0;
// gather some stats to the adapter struct that are per queue
    for (i = 0; i < adapter.num_tx_queues; i++) {
    struct ixgbe_ring *tx_ring = READ_ONCE(adapter.tx_ring[i]);
    if (!tx_ring)
    continue;
    restart_queue += tx_ring.tx_stats.restart_queue;
    tx_busy += tx_ring.tx_stats.tx_busy;
    bytes += tx_ring.stats.bytes;
    packets += tx_ring.stats.packets;
    }
    for (i = 0; i < adapter.num_xdp_queues; i++) {
    struct ixgbe_ring *xdp_ring = READ_ONCE(adapter.xdp_ring[i]);
    if (!xdp_ring)
    continue;
    restart_queue += xdp_ring.tx_stats.restart_queue;
    tx_busy += xdp_ring.tx_stats.tx_busy;
    bytes += xdp_ring.stats.bytes;
    packets += xdp_ring.stats.packets;
    }
    adapter.restart_queue = restart_queue;
    adapter.tx_busy = tx_busy;
    netdev.stats.tx_bytes = bytes;
    netdev.stats.tx_packets = packets;
    hwstats.crcerrs += IXGBE_READ_REG(hw, IXGBE_CRCERRS);
// 8 register reads
    for (i = 0; i < 8; i++) {
// for packet buffers not used, the register should read 0
    mpc = IXGBE_READ_REG(hw, IXGBE_MPC(i));
    missed_rx += mpc;
    hwstats.mpc[i] += mpc;
    total_mpc += hwstats.mpc[i];
    hwstats.pxontxc[i] += IXGBE_READ_REG(hw, IXGBE_PXONTXC(i));
    hwstats.pxofftxc[i] += IXGBE_READ_REG(hw, IXGBE_PXOFFTXC(i));
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    hwstats.rnbc[i] += IXGBE_READ_REG(hw, IXGBE_RNBC(i));
    hwstats.qbtc[i] += IXGBE_READ_REG(hw, IXGBE_QBTC(i));
    hwstats.qbrc[i] += IXGBE_READ_REG(hw, IXGBE_QBRC(i));
    hwstats.pxonrxc[i] +=
    IXGBE_READ_REG(hw, IXGBE_PXONRXC(i));
    break;
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    hwstats.pxonrxc[i] +=
    IXGBE_READ_REG(hw, IXGBE_PXONRXCNT(i));
    break;
    default:
    break;
    }
    }
// 16 register reads
    for (i = 0; i < 16; i++) {
    hwstats.qptc[i] += IXGBE_READ_REG(hw, IXGBE_QPTC(i));
    hwstats.qprc[i] += IXGBE_READ_REG(hw, IXGBE_QPRC(i));
    if (hw.mac.type == ixgbe_mac_82599EB ||
    hw.mac.type == ixgbe_mac_X540 ||
    hw.mac.type == ixgbe_mac_X550 ||
    hw.mac.type == ixgbe_mac_X550EM_x ||
    hw.mac.type == ixgbe_mac_x550em_a ||
    hw.mac.type == ixgbe_mac_e610) {
    hwstats.qbtc[i] += IXGBE_READ_REG(hw, IXGBE_QBTC_L(i));
    IXGBE_READ_REG(hw, IXGBE_QBTC_H(i)); /* to clear */
    hwstats.qbrc[i] += IXGBE_READ_REG(hw, IXGBE_QBRC_L(i));
    IXGBE_READ_REG(hw, IXGBE_QBRC_H(i)); /* to clear */
    }
    }
    hwstats.gprc += IXGBE_READ_REG(hw, IXGBE_GPRC);
// work around hardware counting issue
    hwstats.gprc -= missed_rx;
    ixgbe_update_xoff_received(adapter);
// 82598 hardware only has a 32 bit counter in the high register
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    hwstats.lxonrxc += IXGBE_READ_REG(hw, IXGBE_LXONRXC);
    hwstats.gorc += IXGBE_READ_REG(hw, IXGBE_GORCH);
    hwstats.gotc += IXGBE_READ_REG(hw, IXGBE_GOTCH);
    hwstats.tor += IXGBE_READ_REG(hw, IXGBE_TORH);
    break;
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
// OS2BMC stats are X540 and later
    hwstats.o2bgptc += IXGBE_READ_REG(hw, IXGBE_O2BGPTC);
    hwstats.o2bspc += IXGBE_READ_REG(hw, IXGBE_O2BSPC);
    hwstats.b2ospc += IXGBE_READ_REG(hw, IXGBE_B2OSPC);
    hwstats.b2ogprc += IXGBE_READ_REG(hw, IXGBE_B2OGPRC);
    fallthrough;
    case ixgbe_mac_82599EB:
    for (i = 0; i < 16; i++)
    adapter.hw_rx_no_dma_resources +=
    IXGBE_READ_REG(hw, IXGBE_QPRDC(i));
    hwstats.gorc += IXGBE_READ_REG(hw, IXGBE_GORCL);
    IXGBE_READ_REG(hw, IXGBE_GORCH); /* to clear */
    hwstats.gotc += IXGBE_READ_REG(hw, IXGBE_GOTCL);
    IXGBE_READ_REG(hw, IXGBE_GOTCH); /* to clear */
    hwstats.tor += IXGBE_READ_REG(hw, IXGBE_TORL);
    IXGBE_READ_REG(hw, IXGBE_TORH); /* to clear */
    hwstats.lxonrxc += IXGBE_READ_REG(hw, IXGBE_LXONRXCNT);
    hwstats.fdirmatch += IXGBE_READ_REG(hw, IXGBE_FDIRMATCH);
    hwstats.fdirmiss += IXGBE_READ_REG(hw, IXGBE_FDIRMISS);

    hwstats.fccrc += IXGBE_READ_REG(hw, IXGBE_FCCRC);
    hwstats.fcoerpdc += IXGBE_READ_REG(hw, IXGBE_FCOERPDC);
    hwstats.fcoeprc += IXGBE_READ_REG(hw, IXGBE_FCOEPRC);
    hwstats.fcoeptc += IXGBE_READ_REG(hw, IXGBE_FCOEPTC);
    hwstats.fcoedwrc += IXGBE_READ_REG(hw, IXGBE_FCOEDWRC);
    hwstats.fcoedwtc += IXGBE_READ_REG(hw, IXGBE_FCOEDWTC);
// Add up per cpu counters for total ddp aloc fail
    if (adapter.fcoe.ddp_pool) {
    struct ixgbe_fcoe *fcoe = &adapter.fcoe;
    struct ixgbe_fcoe_ddp_pool *ddp_pool;
    unsigned int cpu;
    let mut noddp: u64 = 0, noddp_ext_buff = 0;
    for_each_possible_cpu(cpu) {
    ddp_pool = per_cpu_ptr(fcoe.ddp_pool, cpu);
    noddp += ddp_pool.noddp;
    noddp_ext_buff += ddp_pool.noddp_ext_buff;
    }
    hwstats.fcoe_noddp = noddp;
    hwstats.fcoe_noddp_ext_buff = noddp_ext_buff;
    }

    break;
    default:
    break;
    }
    bprc = IXGBE_READ_REG(hw, IXGBE_BPRC);
    hwstats.bprc += bprc;
    hwstats.mprc += IXGBE_READ_REG(hw, IXGBE_MPRC);
    if (hw.mac.type == ixgbe_mac_82598EB)
    hwstats.mprc -= bprc;
    hwstats.roc += IXGBE_READ_REG(hw, IXGBE_ROC);
    hwstats.prc64 += IXGBE_READ_REG(hw, IXGBE_PRC64);
    hwstats.prc127 += IXGBE_READ_REG(hw, IXGBE_PRC127);
    hwstats.prc255 += IXGBE_READ_REG(hw, IXGBE_PRC255);
    hwstats.prc511 += IXGBE_READ_REG(hw, IXGBE_PRC511);
    hwstats.prc1023 += IXGBE_READ_REG(hw, IXGBE_PRC1023);
    hwstats.prc1522 += IXGBE_READ_REG(hw, IXGBE_PRC1522);
    hwstats.rlec += IXGBE_READ_REG(hw, IXGBE_RLEC);
    lxon = IXGBE_READ_REG(hw, IXGBE_LXONTXC);
    hwstats.lxontxc += lxon;
    lxoff = IXGBE_READ_REG(hw, IXGBE_LXOFFTXC);
    hwstats.lxofftxc += lxoff;
    hwstats.gptc += IXGBE_READ_REG(hw, IXGBE_GPTC);
    hwstats.mptc += IXGBE_READ_REG(hw, IXGBE_MPTC);
//
// 82598 errata - tx of flow control packets is included in tx counters
//
    xon_off_tot = lxon + lxoff;
    hwstats.gptc -= xon_off_tot;
    hwstats.mptc -= xon_off_tot;
    hwstats.gotc -= (xon_off_tot * (ETH_ZLEN + ETH_FCS_LEN));
    hwstats.ruc += IXGBE_READ_REG(hw, IXGBE_RUC);
    hwstats.rfc += IXGBE_READ_REG(hw, IXGBE_RFC);
    hwstats.rjc += IXGBE_READ_REG(hw, IXGBE_RJC);
    hwstats.tpr += IXGBE_READ_REG(hw, IXGBE_TPR);
    hwstats.ptc64 += IXGBE_READ_REG(hw, IXGBE_PTC64);
    hwstats.ptc64 -= xon_off_tot;
    hwstats.ptc127 += IXGBE_READ_REG(hw, IXGBE_PTC127);
    hwstats.ptc255 += IXGBE_READ_REG(hw, IXGBE_PTC255);
    hwstats.ptc511 += IXGBE_READ_REG(hw, IXGBE_PTC511);
    hwstats.ptc1023 += IXGBE_READ_REG(hw, IXGBE_PTC1023);
    hwstats.ptc1522 += IXGBE_READ_REG(hw, IXGBE_PTC1522);
    hwstats.bptc += IXGBE_READ_REG(hw, IXGBE_BPTC);
// Fill out the OS statistics structure
    netdev.stats.multicast = hwstats.mprc;
// Rx Errors
    netdev.stats.rx_errors = hwstats.crcerrs + hwstats.rlec;
    netdev.stats.rx_dropped = 0;
    netdev.stats.rx_length_errors = hwstats.rlec;
    netdev.stats.rx_crc_errors = hwstats.crcerrs;
    netdev.stats.rx_missed_errors = total_mpc;
// VF Stats Collection - skip while resetting because these
// are not clear on read and otherwise you'll sometimes get
// crazy values.
//
    if (!test_bit(__IXGBE_RESETTING, &adapter.state)) {
    for (i = 0; i < adapter.num_vfs; i++) {
    UPDATE_VF_COUNTER_32bit(IXGBE_PVFGPRC(i),
    adapter.vfinfo[i].last_vfstats.gprc,
    adapter.vfinfo[i].vfstats.gprc);
    UPDATE_VF_COUNTER_32bit(IXGBE_PVFGPTC(i),
    adapter.vfinfo[i].last_vfstats.gptc,
    adapter.vfinfo[i].vfstats.gptc);
    UPDATE_VF_COUNTER_36bit(IXGBE_PVFGORC_LSB(i),
    IXGBE_PVFGORC_MSB(i),
    adapter.vfinfo[i].last_vfstats.gorc,
    adapter.vfinfo[i].vfstats.gorc);
    UPDATE_VF_COUNTER_36bit(IXGBE_PVFGOTC_LSB(i),
    IXGBE_PVFGOTC_MSB(i),
    adapter.vfinfo[i].last_vfstats.gotc,
    adapter.vfinfo[i].vfstats.gotc);
    UPDATE_VF_COUNTER_32bit(IXGBE_PVFMPRC(i),
    adapter.vfinfo[i].last_vfstats.mprc,
    adapter.vfinfo[i].vfstats.mprc);
    }
    }
    }
//
// ixgbe_fdir_reinit_subtask - worker thread to reinit FDIR filter table
// @adapter: pointer to the device adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_fdir_reinit_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_fdir_reinit_subtask(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    int i;
    if (!(adapter.flags2 & IXGBE_FLAG2_FDIR_REQUIRES_REINIT))
    return;
    adapter.flags2 &= ~IXGBE_FLAG2_FDIR_REQUIRES_REINIT;
// if interface is down do nothing
    if (test_bit(__IXGBE_DOWN, &adapter.state))
    return;
// do nothing if we are not using signature filters
    if (!(adapter.flags & IXGBE_FLAG_FDIR_HASH_CAPABLE))
    return;
    adapter.fdir_overflow++;
    if (ixgbe_reinit_fdir_tables_82599(hw) == 0) {
    for (i = 0; i < adapter.num_tx_queues; i++)
    set_bit(__IXGBE_TX_FDIR_INIT_DONE,
    adapter.tx_ring[i].state);
    for (i = 0; i < adapter.num_xdp_queues; i++)
    set_bit(__IXGBE_TX_FDIR_INIT_DONE,
    adapter.xdp_ring[i].state);
// re-enable flow director interrupts
    IXGBE_WRITE_REG(hw, IXGBE_EIMS, IXGBE_EIMS_FLOW_DIR);
    } else {
    e_err(probe, "failed to finish FDIR re-initialization, "
    "ignored adding FDIR ATR filters\n");
    }
    }
//
// ixgbe_check_hang_subtask - check for hung queues and dropped interrupts
// @adapter: pointer to the device adapter structure
//
// This function serves two purposes.  First it strobes the interrupt lines
// in order to make certain interrupts are occurring.  Secondly it sets the
// bits needed to check for TX hangs.  As a result we should immediately
// determine if a hang has occurred.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_check_hang_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_check_hang_subtask(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut eics: u64 = 0;
    int i;
// If we're down, removing or resetting, just bail
    if (test_bit(__IXGBE_DOWN, &adapter.state) ||
    test_bit(__IXGBE_REMOVING, &adapter.state) ||
    test_bit(__IXGBE_RESETTING, &adapter.state))
    return;
// Force detection of hung controller
    if (netif_carrier_ok(adapter.netdev))
    for (i = 0; i < adapter.num_tx_queues; i++)
    set_check_for_tx_hang(adapter.tx_ring[i]);
    if (!(adapter.flags & IXGBE_FLAG_MSIX_ENABLED)) {
//
// for legacy and MSI interrupts don't set any bits
// that are enabled for EIAM, because this operation
// would set *both* EIMS and EICS for any bit in EIAM
//
    IXGBE_WRITE_REG(hw, IXGBE_EICS,
    (IXGBE_EICS_TCP_TIMER | IXGBE_EICS_OTHER));
    } else {
// get one bit for every active tx/rx interrupt vector
    for (i = 0; i < adapter.num_q_vectors; i++) {
    struct ixgbe_q_vector *qv = adapter.q_vector[i];
    if (qv.rx.ring || qv.tx.ring)
    eics |= BIT_ULL(i);
    }
    }
// Cause software interrupt to ensure rings are cleaned
    ixgbe_irq_rearm_queues(adapter, eics);
    }
//
// ixgbe_watchdog_update_link - update the link status
// @adapter: pointer to the device adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_watchdog_update_link(adapter: *mut ixgbe_adapter) {
    static void ixgbe_watchdog_update_link(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut link_speed: u32 = adapter.link_speed;
    let mut link_up: bool = adapter.link_up;
    let mut pfc_en: bool = adapter.dcb_cfg.pfc_mode_enable;
    if (!(adapter.flags & IXGBE_FLAG_NEED_LINK_UPDATE))
    return;
    if (hw.mac.ops.check_link) {
    hw.mac.ops.check_link(hw, &link_speed, &link_up, false);
    } else {
// always assume link is up, if no check link function
    link_speed = IXGBE_LINK_SPEED_10GB_FULL;
    link_up = true;
    }
    if (adapter.ixgbe_ieee_pfc)
    pfc_en |= !!(adapter.ixgbe_ieee_pfc.pfc_en);
    if (link_up && !((adapter.flags & IXGBE_FLAG_DCB_ENABLED) && pfc_en)) {
    hw.mac.ops.fc_enable(hw);
    ixgbe_set_rx_drop_en(adapter);
    }
    if (link_up ||
    time_after(jiffies, (adapter.link_check_timeout +
    IXGBE_TRY_LINK_TIMEOUT))) {
    adapter.flags &= ~IXGBE_FLAG_NEED_LINK_UPDATE;
    IXGBE_WRITE_REG(hw, IXGBE_EIMS, IXGBE_EIMC_LSC);
    IXGBE_WRITE_FLUSH(hw);
    }
    adapter.link_up = link_up;
    adapter.link_speed = link_speed;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_update_default_up(adapter: *mut ixgbe_adapter) {
    static void ixgbe_update_default_up(struct ixgbe_adapter *adapter)
    {

    struct net_device *netdev = adapter.netdev;
    struct dcb_app app = {
    .selector = IEEE_8021QAZ_APP_SEL_ETHERTYPE,
    .protocol = 0,
    };
    let mut up: u8 = 0;
    if (adapter.dcbx_cap & DCB_CAP_DCBX_VER_IEEE)
    up = dcb_ieee_getapp_mask(netdev, &app);
    adapter.default_up = (up > 1) ? (ffs(up) - 1) : 0;

    }
//
// ixgbe_watchdog_link_is_up - update netif_carrier status and
// print link up message
// @adapter: pointer to the device adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_watchdog_link_is_up(adapter: *mut ixgbe_adapter) {
    static void ixgbe_watchdog_link_is_up(struct ixgbe_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct ixgbe_hw *hw = &adapter.hw;
    let mut link_speed: u32 = adapter.link_speed;
    let mut keee: ethtool_keee = {};
    const char *speed_str;
    bool flow_rx, flow_tx;
// only continue if link was previously down
    if (netif_carrier_ok(netdev))
    return;
    adapter.flags2 &= ~IXGBE_FLAG2_SEARCH_FOR_SFP;
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB: {
    let mut frctl: u32 = IXGBE_READ_REG(hw, IXGBE_FCTRL);
    let mut rmcs: u32 = IXGBE_READ_REG(hw, IXGBE_RMCS);
    flow_rx = !!(frctl & IXGBE_FCTRL_RFCE);
    flow_tx = !!(rmcs & IXGBE_RMCS_TFCE_802_3X);
    }
    break;
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    case ixgbe_mac_82599EB: {
    let mut mflcn: u32 = IXGBE_READ_REG(hw, IXGBE_MFLCN);
    let mut fccfg: u32 = IXGBE_READ_REG(hw, IXGBE_FCCFG);
    flow_rx = !!(mflcn & IXGBE_MFLCN_RFCE);
    flow_tx = !!(fccfg & IXGBE_FCCFG_TFCE_802_3X);
    }
    break;
    default:
    flow_tx = false;
    flow_rx = false;
    break;
    }
    adapter.last_rx_ptp_check = jiffies;
    if (test_bit(__IXGBE_PTP_RUNNING, &adapter.state))
    ixgbe_ptp_start_cyclecounter(adapter);
    netdev.ethtool_ops.get_eee(netdev, &keee);
    switch (link_speed) {
    case IXGBE_LINK_SPEED_10GB_FULL:
    speed_str = "10 Gbps";
    break;
    case IXGBE_LINK_SPEED_5GB_FULL:
    speed_str = "5 Gbps";
    break;
    case IXGBE_LINK_SPEED_2_5GB_FULL:
    speed_str = "2.5 Gbps";
    break;
    case IXGBE_LINK_SPEED_1GB_FULL:
    speed_str = "1 Gbps";
    break;
    case IXGBE_LINK_SPEED_100_FULL:
    speed_str = "100 Mbps";
    break;
    case IXGBE_LINK_SPEED_10_FULL:
    speed_str = "10 Mbps";
    break;
    default:
    speed_str = "unknown speed";
    break;
    }
    e_info(drv, "NIC Link is Up %s, Flow Control: %s, EEE: %s\n", speed_str,
    ((flow_rx && flow_tx) ? "RX/TX" :
    (flow_rx ? "RX" :
    (flow_tx ? "TX" : "None"))),
    str_on_off(keee.eee_enabled));
    netif_carrier_on(netdev);
    ixgbe_check_vf_rate_limit(adapter);
    if (adapter.num_vfs && hw.mac.ops.enable_mdd)
    hw.mac.ops.enable_mdd(hw);
// enable transmits
    netif_tx_wake_all_queues(adapter.netdev);
// update the default user priority for VFs
    ixgbe_update_default_up(adapter);
// ping all the active vfs to let them know link has changed
    ixgbe_ping_all_vfs(adapter);
    }
//
// ixgbe_watchdog_link_is_down - update netif_carrier status and
// print link down message
// @adapter: pointer to the adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_watchdog_link_is_down(adapter: *mut ixgbe_adapter) {
    static void ixgbe_watchdog_link_is_down(struct ixgbe_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct ixgbe_hw *hw = &adapter.hw;
    adapter.link_up = false;
    adapter.link_speed = 0;
// only continue if link was up previously
    if (!netif_carrier_ok(netdev))
    return;
    adapter.link_down_events++;
// poll for SFP+ cable when link is down
    if (ixgbe_is_sfp(hw) && hw.mac.type == ixgbe_mac_82598EB)
    adapter.flags2 |= IXGBE_FLAG2_SEARCH_FOR_SFP;
    if (test_bit(__IXGBE_PTP_RUNNING, &adapter.state))
    ixgbe_ptp_start_cyclecounter(adapter);
    e_info(drv, "NIC Link is Down\n");
    netif_carrier_off(netdev);
// ping all the active vfs to let them know link has changed
    ixgbe_ping_all_vfs(adapter);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_ring_tx_pending(adapter: *mut ixgbe_adapter) -> bool {
    static bool ixgbe_ring_tx_pending(struct ixgbe_adapter *adapter)
    {
    int i;
    for (i = 0; i < adapter.num_tx_queues; i++) {
    struct ixgbe_ring *tx_ring = adapter.tx_ring[i];
    if (tx_ring.next_to_use != tx_ring.next_to_clean)
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_vf_tx_pending(adapter: *mut ixgbe_adapter) -> bool {
    static bool ixgbe_vf_tx_pending(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct ixgbe_ring_feature *vmdq = &adapter.ring_feature[RING_F_VMDQ];
    let mut q_per_pool: u32 = __ALIGN_MASK(1, ~vmdq.mask);
    int i, j;
    if (!adapter.num_vfs)
    return false;
// resetting the PF is only needed for MAC before X550
    if (hw.mac.type >= ixgbe_mac_X550)
    return false;
    for (i = 0; i < adapter.num_vfs; i++) {
    for (j = 0; j < q_per_pool; j++) {
    u32 h, t;
    h = IXGBE_READ_REG(hw, IXGBE_PVFTDHN(q_per_pool, i, j));
    t = IXGBE_READ_REG(hw, IXGBE_PVFTDTN(q_per_pool, i, j));
    if (h != t)
    return true;
    }
    }
    return false;
    }
//
// ixgbe_watchdog_flush_tx - flush queues on link down
// @adapter: pointer to the device adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_watchdog_flush_tx(adapter: *mut ixgbe_adapter) {
    static void ixgbe_watchdog_flush_tx(struct ixgbe_adapter *adapter)
    {
    if (!netif_carrier_ok(adapter.netdev)) {
    if (ixgbe_ring_tx_pending(adapter) ||
    ixgbe_vf_tx_pending(adapter)) {
// We've lost link, so the controller stops DMA,
// but we've got queued Tx work that's never going
// to get done, so reset controller to flush Tx.
// (Do the reset outside of interrupt context).
//
    e_warn(drv, "initiating reset to clear Tx work after link loss\n");
    set_bit(__IXGBE_RESET_REQUESTED, &adapter.state);
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn ixgbe_bad_vf_abort(adapter: *mut ixgbe_adapter, vf: u32) {
    static void ixgbe_bad_vf_abort(struct ixgbe_adapter *adapter, u32 vf)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    if (adapter.hw.mac.type == ixgbe_mac_82599EB &&
    adapter.flags2 & IXGBE_FLAG2_AUTO_DISABLE_VF) {
    adapter.vfinfo[vf].primary_abort_count++;
    if (adapter.vfinfo[vf].primary_abort_count ==
    IXGBE_PRIMARY_ABORT_LIMIT) {
    ixgbe_set_vf_link_state(adapter, vf,
    IFLA_VF_LINK_STATE_DISABLE);
    adapter.vfinfo[vf].primary_abort_count = 0;
    e_info(drv,
    "Malicious Driver Detection event detected on PF %d VF %d MAC: %pM mdd-disable-vf=on",
    hw.bus.func, vf,
    adapter.vfinfo[vf].vf_mac_addresses);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_for_bad_vf(adapter: *mut ixgbe_adapter) {
    static void ixgbe_check_for_bad_vf(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct pci_dev *pdev = adapter.pdev;
    unsigned int vf;
    u32 gpc;
    if (!(netif_carrier_ok(adapter.netdev)))
    return;
    gpc = IXGBE_READ_REG(hw, IXGBE_TXDGPC);
    if (gpc) /* If incrementing then no need for the check below */
    return;
// Check to see if a bad DMA write target from an errant or
// malicious VF has caused a PCIe error.  If so then we can
// issue a VFLR to the offending VF(s) and then resume without
// requesting a full slot reset.
//
    if (!pdev)
    return;
// check status reg for all VFs owned by this PF
    for (vf = 0; vf < adapter.num_vfs; ++vf) {
    struct pci_dev *vfdev = adapter.vfinfo[vf].vfdev;
    u16 status_reg;
    if (!vfdev)
    continue;
    pci_read_config_word(vfdev, PCI_STATUS, &status_reg);
    if (status_reg != IXGBE_FAILED_READ_CFG_WORD &&
    status_reg & PCI_STATUS_REC_MASTER_ABORT) {
    ixgbe_bad_vf_abort(adapter, vf);
    pcie_flr(vfdev);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_spoof_check(adapter: *mut ixgbe_adapter) {
    static void ixgbe_spoof_check(struct ixgbe_adapter *adapter)
    {
    u32 ssvpc;
// Do not perform spoof check for 82598 or if not in IOV mode
    if (adapter.hw.mac.type == ixgbe_mac_82598EB ||
    adapter.num_vfs == 0)
    return;
    ssvpc = IXGBE_READ_REG(&adapter.hw, IXGBE_SSVPC);
//
// ssvpc register is cleared on read, if zero then no
// spoofed packets in the last interval.
//
    if (!ssvpc)
    return;
    e_warn(drv, "%u Spoofed packets detected\n", ssvpc);
    }

#[no_mangle]
unsafe extern "C" fn ixgbe_spoof_check(adapter: *mut ixgbe_adapter __always_unused) {
    static void ixgbe_spoof_check(struct ixgbe_adapter __always_unused *adapter)
    {
    }
    static void
    ixgbe_check_for_bad_vf(struct ixgbe_adapter __always_unused *adapter)
    {
    }

//
// ixgbe_watchdog_subtask - check and bring link up
// @adapter: pointer to the device adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_watchdog_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_watchdog_subtask(struct ixgbe_adapter *adapter)
    {
// if interface is down, removing or resetting, do nothing
    if (test_bit(__IXGBE_DOWN, &adapter.state) ||
    test_bit(__IXGBE_REMOVING, &adapter.state) ||
    test_bit(__IXGBE_RESETTING, &adapter.state))
    return;
    ixgbe_watchdog_update_link(adapter);
    if (adapter.link_up)
    ixgbe_watchdog_link_is_up(adapter);
    else
    ixgbe_watchdog_link_is_down(adapter);
    ixgbe_check_for_bad_vf(adapter);
    ixgbe_spoof_check(adapter);
    ixgbe_update_stats(adapter);
    ixgbe_watchdog_flush_tx(adapter);
    }
//
// ixgbe_sfp_detection_subtask - poll for SFP+ cable
// @adapter: the ixgbe adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_sfp_detection_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_sfp_detection_subtask(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    int err;
// not searching for SFP so there is nothing to do here
    if (!(adapter.flags2 & IXGBE_FLAG2_SEARCH_FOR_SFP) &&
    !(adapter.flags2 & IXGBE_FLAG2_SFP_NEEDS_RESET))
    return;
    if (adapter.sfp_poll_time &&
    time_after(adapter.sfp_poll_time, jiffies))
    return; /* If not yet time to poll for SFP */
// someone else is in init, wait until next service event
    if (test_and_set_bit(__IXGBE_IN_SFP_INIT, &adapter.state))
    return;
    adapter.sfp_poll_time = jiffies + IXGBE_SFP_POLL_JIFFIES - 1;
    err = hw.phy.ops.identify_sfp(hw);
    if (err == -EOPNOTSUPP)
    goto sfp_out;
    if (err == -ENOENT) {
// If no cable is present, then we need to reset
// the next time we find a good cable.
    adapter.flags2 |= IXGBE_FLAG2_SFP_NEEDS_RESET;
    }
// exit on error
    if (err)
    goto sfp_out;
// exit if reset not needed
    if (!(adapter.flags2 & IXGBE_FLAG2_SFP_NEEDS_RESET))
    goto sfp_out;
    adapter.flags2 &= ~IXGBE_FLAG2_SFP_NEEDS_RESET;
//
// A module may be identified correctly, but the EEPROM may not have
// support for that module.  setup_sfp() will fail in that case, so
// we should not allow that module to load.
//
    if (hw.mac.type == ixgbe_mac_82598EB)
    err = hw.phy.ops.reset(hw);
    else
    err = hw.mac.ops.setup_sfp(hw);
    if (err == -EOPNOTSUPP)
    goto sfp_out;
    adapter.flags |= IXGBE_FLAG_NEED_LINK_CONFIG;
    e_info(probe, "detected SFP+: %d\n", hw.phy.sfp_type);
    sfp_out:
    clear_bit(__IXGBE_IN_SFP_INIT, &adapter.state);
    if (err == -EOPNOTSUPP &&
    adapter.netdev.reg_state == NETREG_REGISTERED) {
    e_dev_err("failed to initialize because an unsupported "
    "SFP+ module type was detected.\n");
    e_dev_err("Reload the driver after installing a "
    "supported module.\n");
    unregister_netdev(adapter.netdev);
    }
    }
//
// ixgbe_sfp_link_config_subtask - set up link SFP after module install
// @adapter: the ixgbe adapter structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_sfp_link_config_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_sfp_link_config_subtask(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    u32 cap_speed;
    u32 speed;
    let mut autoneg: bool = false;
    if (!(adapter.flags & IXGBE_FLAG_NEED_LINK_CONFIG))
    return;
// someone else is in init, wait until next service event
    if (test_and_set_bit(__IXGBE_IN_SFP_INIT, &adapter.state))
    return;
    adapter.flags &= ~IXGBE_FLAG_NEED_LINK_CONFIG;
    hw.mac.ops.get_link_capabilities(hw, &cap_speed, &autoneg);
// advertise highest capable link speed
    if (!autoneg && (cap_speed & IXGBE_LINK_SPEED_10GB_FULL))
    speed = IXGBE_LINK_SPEED_10GB_FULL;
    else
    speed = cap_speed & (IXGBE_LINK_SPEED_10GB_FULL |
    IXGBE_LINK_SPEED_1GB_FULL);
    if (hw.mac.ops.setup_link)
    hw.mac.ops.setup_link(hw, speed, true);
    adapter.flags |= IXGBE_FLAG_NEED_LINK_UPDATE;
    adapter.link_check_timeout = jiffies;
    clear_bit(__IXGBE_IN_SFP_INIT, &adapter.state);
    }
//
// ixgbe_service_timer - Timer Call-back
// @t: pointer to timer_list structure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_service_timer(t: *mut timer_list) {
    static void ixgbe_service_timer(struct timer_list *t)
    {
    struct ixgbe_adapter *adapter = timer_container_of(adapter, t,
    service_timer);
    unsigned long next_event_offset;
// poll faster when waiting for link
    if (adapter.flags & IXGBE_FLAG_NEED_LINK_UPDATE)
    next_event_offset = HZ / 10;
    else
    next_event_offset = HZ * 2;
// Reset the timer
    mod_timer(&adapter.service_timer, next_event_offset + jiffies);
    ixgbe_service_event_schedule(adapter);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_phy_interrupt_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_phy_interrupt_subtask(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    bool overtemp;
    if (!(adapter.flags2 & IXGBE_FLAG2_PHY_INTERRUPT))
    return;
    adapter.flags2 &= ~IXGBE_FLAG2_PHY_INTERRUPT;
    if (!hw.phy.ops.handle_lasi)
    return;
    hw.phy.ops.handle_lasi(&adapter.hw, &overtemp);
    if (overtemp)
    e_crit(drv, "%s\n", ixgbe_overheat_msg);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_reset_subtask(adapter: *mut ixgbe_adapter) {
    static void ixgbe_reset_subtask(struct ixgbe_adapter *adapter)
    {
    if (!test_and_clear_bit(__IXGBE_RESET_REQUESTED, &adapter.state))
    return;
    rtnl_lock();
// If we're already down, removing or resetting, just bail
    if (test_bit(__IXGBE_DOWN, &adapter.state) ||
    test_bit(__IXGBE_REMOVING, &adapter.state) ||
    test_bit(__IXGBE_RESETTING, &adapter.state)) {
    rtnl_unlock();
    return;
    }
    ixgbe_dump(adapter);
    netdev_err(adapter.netdev, "Reset adapter\n");
    adapter.tx_timeout_count++;
    ixgbe_reinit_locked(adapter);
    rtnl_unlock();
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_check_fw_api_mismatch(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_check_fw_api_mismatch(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    if (hw.mac.type != ixgbe_mac_e610)
    return 0;
    if (hw.mac.ops.get_fw_ver && hw.mac.ops.get_fw_ver(hw))
    return 0;
    if (hw.api_maj_ver > IXGBE_FW_API_VER_MAJOR) {
    e_dev_err("The driver for the device stopped because the NVM image is newer than expected. You must install the most recent version of the network driver.\n");
    adapter.flags2 |= IXGBE_FLAG2_API_MISMATCH;
    return -EOPNOTSUPP;
    } else if (hw.api_maj_ver == IXGBE_FW_API_VER_MAJOR &&
    hw.api_min_ver > IXGBE_FW_API_VER_MINOR + IXGBE_FW_API_VER_DIFF_ALLOWED) {
    e_dev_info("The driver for the device detected a newer version of the NVM image than expected. Please install the most recent version of the network driver.\n");
    adapter.flags2 |= IXGBE_FLAG2_API_MISMATCH;
    } else if (hw.api_maj_ver < IXGBE_FW_API_VER_MAJOR ||
    hw.api_min_ver < IXGBE_FW_API_VER_MINOR - IXGBE_FW_API_VER_DIFF_ALLOWED) {
    e_dev_info("The driver for the device detected an older version of the NVM image than expected. Please update the NVM image.\n");
    adapter.flags2 |= IXGBE_FLAG2_API_MISMATCH;
    }
    return 0;
    }
//
// ixgbe_check_fw_error - Check firmware for errors
// @adapter: the adapter private structure
//
// Check firmware errors in register FWSM
//
#[no_mangle]
unsafe extern "C" fn ixgbe_check_fw_error(adapter: *mut ixgbe_adapter) -> bool {
    static bool ixgbe_check_fw_error(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    u32 fwsm;
    int err;
// read fwsm.ext_err_ind register and log errors
    fwsm = IXGBE_READ_REG(hw, IXGBE_FWSM(hw));
// skip if E610's FW is reloading, warning in that case may be misleading
    if (fwsm & IXGBE_FWSM_EXT_ERR_IND_MASK ||
    (!(fwsm & IXGBE_FWSM_FW_VAL_BIT) && !(hw.mac.type == ixgbe_mac_e610)))
    e_dev_warn("Warning firmware error detected FWSM: 0x%08X\n",
    fwsm);
    if (hw.mac.ops.fw_recovery_mode && hw.mac.ops.fw_recovery_mode(hw)) {
    e_dev_err("Firmware recovery mode detected. Limiting functionality. Refer to the Intel(R) Ethernet Adapters and Devices User Guide for details on firmware recovery mode.\n");
    return true;
    }
    if (!(adapter.flags2 & IXGBE_FLAG2_API_MISMATCH)) {
    err = ixgbe_check_fw_api_mismatch(adapter);
    if (err)
    return true;
    }
// return here if FW rollback mode has been already detected
    if (adapter.flags2 & IXGBE_FLAG2_FW_ROLLBACK)
    return false;
    if (hw.mac.ops.fw_rollback_mode && hw.mac.ops.fw_rollback_mode(hw)) {
    struct ixgbe_nvm_info *nvm_info = &adapter.hw.flash.nvm;
    char ver_buff[64] = "";
    if (hw.mac.ops.get_fw_ver && hw.mac.ops.get_fw_ver(hw))
    goto no_version;
    if (hw.mac.ops.get_nvm_ver &&
    hw.mac.ops.get_nvm_ver(hw, nvm_info))
    goto no_version;
    snprintf(ver_buff, sizeof(ver_buff),
    "Current version is NVM:%x.%x.%x, FW:%d.%d. ",
    nvm_info.major, nvm_info.minor, nvm_info.eetrack,
    hw.fw_maj_ver, hw.fw_maj_ver);
    no_version:
    e_dev_warn("Firmware rollback mode detected. %sDevice may exhibit limited functionality. Refer to the Intel(R) Ethernet Adapters and Devices User Guide for details on firmware rollback mode.",
    ver_buff);
    adapter.flags2 |= IXGBE_FLAG2_FW_ROLLBACK;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_recovery_service_task(work: *mut work_struct) {
    static void ixgbe_recovery_service_task(struct work_struct *work)
    {
    struct ixgbe_adapter *adapter = container_of(work,
    struct ixgbe_adapter,
    service_task);
    ixgbe_handle_fw_event(adapter);
    ixgbe_service_event_complete(adapter);
    mod_timer(&adapter.service_timer, jiffies + msecs_to_jiffies(100));
    }
//
// ixgbe_service_task - manages and runs subtasks
// @work: pointer to work_struct containing our data
//
#[no_mangle]
unsafe extern "C" fn ixgbe_service_task(work: *mut work_struct) {
    static void ixgbe_service_task(struct work_struct *work)
    {
    struct ixgbe_adapter *adapter = container_of(work,
    struct ixgbe_adapter,
    service_task);
    if (ixgbe_removed(adapter.hw.hw_addr)) {
    if (!test_bit(__IXGBE_DOWN, &adapter.state)) {
    rtnl_lock();
    ixgbe_down(adapter);
    rtnl_unlock();
    }
    ixgbe_service_event_complete(adapter);
    return;
    }
    if (ixgbe_check_fw_error(adapter)) {
    if (!test_bit(__IXGBE_DOWN, &adapter.state)) {
    if (adapter.mii_bus) {
    mdiobus_unregister(adapter.mii_bus);
    adapter.mii_bus = core::ptr::null_mut();
    }
    unregister_netdev(adapter.netdev);
    }
    ixgbe_service_event_complete(adapter);
    return;
    }
    if (adapter.hw.mac.type == ixgbe_mac_e610) {
    if (adapter.flags2 & IXGBE_FLAG2_FW_ASYNC_EVENT)
    ixgbe_handle_fw_event(adapter);
    ixgbe_check_media_subtask(adapter);
    }
    ixgbe_reset_subtask(adapter);
    ixgbe_phy_interrupt_subtask(adapter);
    ixgbe_sfp_detection_subtask(adapter);
    ixgbe_sfp_link_config_subtask(adapter);
    ixgbe_check_overtemp_subtask(adapter);
    ixgbe_watchdog_subtask(adapter);
    ixgbe_fdir_reinit_subtask(adapter);
    ixgbe_check_hang_subtask(adapter);
    if (test_bit(__IXGBE_PTP_RUNNING, &adapter.state)) {
    ixgbe_ptp_overflow_check(adapter);
    if (adapter.flags & IXGBE_FLAG_RX_HWTSTAMP_IN_REGISTER)
    ixgbe_ptp_rx_hang(adapter);
    ixgbe_ptp_tx_hang(adapter);
    }
    ixgbe_service_event_complete(adapter);
    }
    static int ixgbe_tso(struct ixgbe_ring *tx_ring,
    struct ixgbe_tx_buffer *first,
    u8 *hdr_len,
    struct ixgbe_ipsec_tx_data *itd)
    {
    u32 vlan_macip_lens, type_tucmd, mss_l4len_idx;
    struct sk_buff *skb = first.skb;
    union {
    struct iphdr *v4;
    struct ipv6hdr *v6;
    unsigned char *hdr;
    } ip;
    union {
    struct tcphdr *tcp;
    struct udphdr *udp;
    unsigned char *hdr;
    } l4;
    u32 paylen, l4_offset;
    let mut fceof_saidx: u32 = 0;
    int err;
    if (skb.ip_summed != CHECKSUM_PARTIAL)
    return 0;
    if (!skb_is_gso(skb))
    return 0;
    err = skb_cow_head(skb, 0);
    if (err < 0)
    return err;
    if (eth_p_mpls(first.protocol))
    ip.hdr = skb_inner_network_header(skb);
    else
    ip.hdr = skb_network_header(skb);
    l4.hdr = skb_checksum_start(skb);
// ADV DTYP TUCMD MKRLOC/ISCSIHEDLEN
    type_tucmd = (skb_shinfo(skb).gso_type & SKB_GSO_UDP_L4) ?
    IXGBE_ADVTXD_TUCMD_L4T_UDP : IXGBE_ADVTXD_TUCMD_L4T_TCP;
// initialize outer IP header fields
    if (ip.v4.version == 4) {
    unsigned char *csum_start = skb_checksum_start(skb);
    unsigned char *trans_start = ip.hdr + (ip.v4.ihl * 4);
    let mut len: c_int = csum_start - trans_start;
// IP header will have to cancel out any data that
// is not a part of the outer IP header, so set to
// a reverse csum if needed, else init check to 0.
//
    ip.v4.check = (skb_shinfo(skb).gso_type & SKB_GSO_PARTIAL) ?
    csum_fold(csum_partial(trans_start,
    len, 0)) : 0;
    type_tucmd |= IXGBE_ADVTXD_TUCMD_IPV4;
    ip.v4.tot_len = 0;
    first.tx_flags |= IXGBE_TX_FLAGS_TSO |
    IXGBE_TX_FLAGS_CSUM |
    IXGBE_TX_FLAGS_IPV4;
    } else {
    ip.v6.payload_len = 0;
    first.tx_flags |= IXGBE_TX_FLAGS_TSO |
    IXGBE_TX_FLAGS_CSUM;
    }
// determine offset of inner transport header
    l4_offset = l4.hdr - skb.data;
// remove payload length from inner checksum
    paylen = skb.len - l4_offset;
    if (type_tucmd & IXGBE_ADVTXD_TUCMD_L4T_TCP) {
// compute length of segmentation header
// hdr_len = (l4.tcp->doff * 4) + l4_offset;
    csum_replace_by_diff(&l4.tcp.check,
    ( __wsum)htonl(paylen));
    } else {
// compute length of segmentation header
// hdr_len = sizeof(*l4.udp) + l4_offset;
    csum_replace_by_diff(&l4.udp.check,
    ( __wsum)htonl(paylen));
    }
// update gso size and bytecount with header size
    first.gso_segs = skb_shinfo(skb).gso_segs;
    first.bytecount += (first.gso_segs - 1) * *hdr_len;
// mss_l4len_id: use 0 as index for TSO
    mss_l4len_idx = (*hdr_len - l4_offset) << IXGBE_ADVTXD_L4LEN_SHIFT;
    mss_l4len_idx |= skb_shinfo(skb).gso_size << IXGBE_ADVTXD_MSS_SHIFT;
    fceof_saidx |= itd.sa_idx;
    type_tucmd |= itd.flags | itd.trailer_len;
// vlan_macip_lens: HEADLEN, MACLEN, VLAN tag
    vlan_macip_lens = l4.hdr - ip.hdr;
    vlan_macip_lens |= (ip.hdr - skb.data) << IXGBE_ADVTXD_MACLEN_SHIFT;
    vlan_macip_lens |= first.tx_flags & IXGBE_TX_FLAGS_VLAN_MASK;
    ixgbe_tx_ctxtdesc(tx_ring, vlan_macip_lens, fceof_saidx, type_tucmd,
    mss_l4len_idx);
    return 1;
    }
    static void ixgbe_tx_csum(struct ixgbe_ring *tx_ring,
    struct ixgbe_tx_buffer *first,
    struct ixgbe_ipsec_tx_data *itd)
    {
    struct sk_buff *skb = first.skb;
    let mut vlan_macip_lens: u32 = 0;
    let mut fceof_saidx: u32 = 0;
    let mut type_tucmd: u32 = 0;
    if (skb.ip_summed != CHECKSUM_PARTIAL) {
    csum_failed:
    if (!(first.tx_flags & (IXGBE_TX_FLAGS_HW_VLAN |
    IXGBE_TX_FLAGS_CC)))
    return;
    goto no_csum;
    }
    switch (skb.csum_offset) {
    case offsetof(struct tcphdr, check):
    type_tucmd = IXGBE_ADVTXD_TUCMD_L4T_TCP;
    fallthrough;
    case offsetof(struct udphdr, check):
    break;
    case offsetof(struct sctphdr, checksum):
// validate that this is actually an SCTP request
    if (skb_csum_is_sctp(skb)) {
    type_tucmd = IXGBE_ADVTXD_TUCMD_L4T_SCTP;
    break;
    }
    fallthrough;
    default:
    skb_checksum_help(skb);
    goto csum_failed;
    }
// update TX checksum flag
    first.tx_flags |= IXGBE_TX_FLAGS_CSUM;
    vlan_macip_lens = skb_checksum_start_offset(skb) -
    skb_network_offset(skb);
    no_csum:
// vlan_macip_lens: MACLEN, VLAN tag
    vlan_macip_lens |= skb_network_offset(skb) << IXGBE_ADVTXD_MACLEN_SHIFT;
    vlan_macip_lens |= first.tx_flags & IXGBE_TX_FLAGS_VLAN_MASK;
    fceof_saidx |= itd.sa_idx;
    type_tucmd |= itd.flags | itd.trailer_len;
    ixgbe_tx_ctxtdesc(tx_ring, vlan_macip_lens, fceof_saidx, type_tucmd, 0);
    }

    ((_flag <= _result) ? \
    ((u32)(_input & _flag) * (_result / _flag)) : \
    ((u32)(_input & _flag) / (_flag / _result)))
#[no_mangle]
unsafe extern "C" fn ixgbe_tx_cmd_type(skb: *mut sk_buff, tx_flags: u32) -> u32 {
    static u32 ixgbe_tx_cmd_type(struct sk_buff *skb, u32 tx_flags)
    {
// set type for advanced descriptor with frame checksum insertion
    u32 cmd_type = IXGBE_ADVTXD_DTYP_DATA |
    IXGBE_ADVTXD_DCMD_DEXT |
    IXGBE_ADVTXD_DCMD_IFCS;
// set HW vlan bit if vlan is present
    cmd_type |= IXGBE_SET_FLAG(tx_flags, IXGBE_TX_FLAGS_HW_VLAN,
    IXGBE_ADVTXD_DCMD_VLE);
// set segmentation enable bits for TSO/FSO
    cmd_type |= IXGBE_SET_FLAG(tx_flags, IXGBE_TX_FLAGS_TSO,
    IXGBE_ADVTXD_DCMD_TSE);
// set timestamp bit if present
    cmd_type |= IXGBE_SET_FLAG(tx_flags, IXGBE_TX_FLAGS_TSTAMP,
    IXGBE_ADVTXD_MAC_TSTAMP);
// insert frame checksum
    cmd_type ^= IXGBE_SET_FLAG(skb.no_fcs, 1, IXGBE_ADVTXD_DCMD_IFCS);
    return cmd_type;
    }
    static void ixgbe_tx_olinfo_status(union ixgbe_adv_tx_desc *tx_desc,
    u32 tx_flags, unsigned int paylen)
    {
    let mut olinfo_status: u32 = paylen << IXGBE_ADVTXD_PAYLEN_SHIFT;
// enable L4 checksum for TSO and TX checksum offload
    olinfo_status |= IXGBE_SET_FLAG(tx_flags,
    IXGBE_TX_FLAGS_CSUM,
    IXGBE_ADVTXD_POPTS_TXSM);
// enable IPv4 checksum for TSO
    olinfo_status |= IXGBE_SET_FLAG(tx_flags,
    IXGBE_TX_FLAGS_IPV4,
    IXGBE_ADVTXD_POPTS_IXSM);
// enable IPsec
    olinfo_status |= IXGBE_SET_FLAG(tx_flags,
    IXGBE_TX_FLAGS_IPSEC,
    IXGBE_ADVTXD_POPTS_IPSEC);
//
// Check Context must be set if Tx switch is enabled, which it
// always is for case where virtual functions are running
//
    olinfo_status |= IXGBE_SET_FLAG(tx_flags,
    IXGBE_TX_FLAGS_CC,
    IXGBE_ADVTXD_CC);
    tx_desc.read.olinfo_status = cpu_to_le32(olinfo_status);
    }
#[no_mangle]
unsafe extern "C" fn __ixgbe_maybe_stop_tx(tx_ring: *mut ixgbe_ring, size: u16) -> c_int {
    static int __ixgbe_maybe_stop_tx(struct ixgbe_ring *tx_ring, u16 size)
    {
    if (!netif_subqueue_try_stop(tx_ring.netdev, tx_ring.queue_index,
    ixgbe_desc_unused(tx_ring), size))
    return -EBUSY;
    ++tx_ring.tx_stats.restart_queue;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_maybe_stop_tx(tx_ring: *mut ixgbe_ring, size: u16) -> c_int {
    static inline int ixgbe_maybe_stop_tx(struct ixgbe_ring *tx_ring, u16 size)
    {
    if (likely(ixgbe_desc_unused(tx_ring) >= size))
    return 0;
    return __ixgbe_maybe_stop_tx(tx_ring, size);
    }
    static int ixgbe_tx_map(struct ixgbe_ring *tx_ring,
    struct ixgbe_tx_buffer *first,
    const u8 hdr_len)
    {
    struct sk_buff *skb = first.skb;
    struct ixgbe_tx_buffer *tx_buffer;
    union ixgbe_adv_tx_desc *tx_desc;
    skb_frag_t *frag;
    dma_addr_t dma;
    unsigned int data_len, size;
    let mut tx_flags: u32 = first.tx_flags;
    let mut cmd_type: u32 = ixgbe_tx_cmd_type(skb, tx_flags);
    let mut i: u16 = tx_ring.next_to_use;
    tx_desc = IXGBE_TX_DESC(tx_ring, i);
    ixgbe_tx_olinfo_status(tx_desc, tx_flags, skb.len - hdr_len);
    size = skb_headlen(skb);
    data_len = skb.data_len;

    if (tx_flags & IXGBE_TX_FLAGS_FCOE) {
    if (data_len < sizeof(struct fcoe_crc_eof)) {
    size -= sizeof(struct fcoe_crc_eof) - data_len;
    data_len = 0;
    } else {
    data_len -= sizeof(struct fcoe_crc_eof);
    }
    }

    dma = dma_map_single(tx_ring.dev, skb.data, size, DMA_TO_DEVICE);
    tx_buffer = first;
    for (frag = &skb_shinfo(skb).frags[0];; frag++) {
    if (dma_mapping_error(tx_ring.dev, dma))
    goto dma_error;
// record length, and DMA address
    dma_unmap_len_set(tx_buffer, len, size);
    dma_unmap_addr_set(tx_buffer, dma, dma);
    tx_desc.read.buffer_addr = cpu_to_le64(dma);
    while (unlikely(size > IXGBE_MAX_DATA_PER_TXD)) {
    tx_desc.read.cmd_type_len =
    cpu_to_le32(cmd_type ^ IXGBE_MAX_DATA_PER_TXD);
    i++;
    tx_desc++;
    if (i == tx_ring.count) {
    tx_desc = IXGBE_TX_DESC(tx_ring, 0);
    i = 0;
    }
    tx_desc.read.olinfo_status = 0;
    dma += IXGBE_MAX_DATA_PER_TXD;
    size -= IXGBE_MAX_DATA_PER_TXD;
    tx_desc.read.buffer_addr = cpu_to_le64(dma);
    }
    if (likely(!data_len))
    break;
    tx_desc.read.cmd_type_len = cpu_to_le32(cmd_type ^ size);
    i++;
    tx_desc++;
    if (i == tx_ring.count) {
    tx_desc = IXGBE_TX_DESC(tx_ring, 0);
    i = 0;
    }
    tx_desc.read.olinfo_status = 0;

    size = min_t(unsigned int, data_len, skb_frag_size(frag));

    size = skb_frag_size(frag);

    data_len -= size;
    dma = skb_frag_dma_map(tx_ring.dev, frag, 0, size,
    DMA_TO_DEVICE);
    tx_buffer = &tx_ring.tx_buffer_info[i];
    }
// write last descriptor with RS and EOP bits
    cmd_type |= size | IXGBE_TXD_CMD;
    tx_desc.read.cmd_type_len = cpu_to_le32(cmd_type);
    netdev_tx_sent_queue(txring_txq(tx_ring), first.bytecount);
// set the timestamp
    first.time_stamp = jiffies;
    skb_tx_timestamp(skb);
//
// Force memory writes to complete before letting h/w know there
// are new descriptors to fetch.  (Only applicable for weak-ordered
// memory model archs, such as IA-64).
//
// We also need this memory barrier to make certain all of the
// status bits have been updated before next_to_watch is written.
//
    wmb();
// set next_to_watch value indicating a packet is present
    first.next_to_watch = tx_desc;
    i++;
    if (i == tx_ring.count)
    i = 0;
    tx_ring.next_to_use = i;
    ixgbe_maybe_stop_tx(tx_ring, DESC_NEEDED);
    if (netif_xmit_stopped(txring_txq(tx_ring)) || !netdev_xmit_more()) {
    writel(i, tx_ring.tail);
    }
    return 0;
    dma_error:
    dev_err(tx_ring.dev, "TX DMA map failed\n");
// clear dma mappings for failed tx_buffer_info map
    for (;;) {
    tx_buffer = &tx_ring.tx_buffer_info[i];
    if (dma_unmap_len(tx_buffer, len))
    dma_unmap_page(tx_ring.dev,
    dma_unmap_addr(tx_buffer, dma),
    dma_unmap_len(tx_buffer, len),
    DMA_TO_DEVICE);
    dma_unmap_len_set(tx_buffer, len, 0);
    if (tx_buffer == first)
    break;
    if (i == 0)
    i += tx_ring.count;
    i--;
    }
    dev_kfree_skb_any(first.skb);
    first.skb = core::ptr::null_mut();
    tx_ring.next_to_use = i;
    return -1;
    }
    static void ixgbe_atr(struct ixgbe_ring *ring,
    struct ixgbe_tx_buffer *first)
    {
    struct ixgbe_q_vector *q_vector = ring.q_vector;
    let mut input: union ixgbe_atr_hash_dword = { .dword = 0 };
    let mut common: union ixgbe_atr_hash_dword = { .dword = 0 };
    union {
    unsigned char *network;
    struct iphdr *ipv4;
    struct ipv6hdr *ipv6;
    } hdr;
    struct tcphdr *th;
    unsigned int hlen;
    struct sk_buff *skb;
    __be16 vlan_id;
    int l4_proto;
// if ring doesn't have a interrupt vector, cannot perform ATR
    if (!q_vector)
    return;
// do nothing if sampling is disabled
    if (!ring.atr_sample_rate)
    return;
    ring.atr_count++;
// currently only IPv4/IPv6 with TCP is supported
    if ((first.protocol != htons(ETH_P_IP)) &&
    (first.protocol != htons(ETH_P_IPV6)))
    return;
// snag network header to get L4 type and address
    skb = first.skb;
    hdr.network = skb_network_header(skb);
    if (unlikely(hdr.network <= skb.data))
    return;
    if (skb.encapsulation &&
    first.protocol == htons(ETH_P_IP) &&
    hdr.ipv4.protocol == IPPROTO_UDP) {
    struct ixgbe_adapter *adapter = q_vector.adapter;
    if (unlikely(skb_tail_pointer(skb) < hdr.network +
    vxlan_headroom(0)))
    return;
// verify the port is recognized as VXLAN
    if (adapter.vxlan_port &&
    udp_hdr(skb).dest == adapter.vxlan_port)
    hdr.network = skb_inner_network_header(skb);
    if (adapter.geneve_port &&
    udp_hdr(skb).dest == adapter.geneve_port)
    hdr.network = skb_inner_network_header(skb);
    }
// Make sure we have at least [minimum IPv4 header + TCP]
// or [IPv6 header] bytes
//
    if (unlikely(skb_tail_pointer(skb) < hdr.network + 40))
    return;
// Currently only IPv4/IPv6 with TCP is supported
    switch (hdr.ipv4.version) {
    case IPVERSION:
// access ihl as u8 to avoid unaligned access on ia64
    hlen = (hdr.network[0] & 0x0F) << 2;
    l4_proto = hdr.ipv4.protocol;
    break;
    case 6:
    hlen = hdr.network - skb.data;
    l4_proto = ipv6_find_hdr(skb, &hlen, IPPROTO_TCP, core::ptr::null_mut(), core::ptr::null_mut());
    hlen -= hdr.network - skb.data;
    break;
    default:
    return;
    }
    if (l4_proto != IPPROTO_TCP)
    return;
    if (unlikely(skb_tail_pointer(skb) < hdr.network +
    hlen + sizeof(struct tcphdr)))
    return;
    th = (struct tcphdr *)(hdr.network + hlen);
// skip this packet since the socket is closing
    if (th.fin)
    return;
// sample on all syn packets or once every atr sample count
    if (!th.syn && (ring.atr_count < ring.atr_sample_rate))
    return;
// reset sample count
    ring.atr_count = 0;
    vlan_id = htons(first.tx_flags >> IXGBE_TX_FLAGS_VLAN_SHIFT);
//
// src and dst are inverted, think how the receiver sees them
//
// The input is broken into two sections, a non-compressed section
// containing vm_pool, vlan_id, and flow_type.  The rest of the data
// is XORed together and stored in the compressed dword.
//
    input.formatted.vlan_id = vlan_id;
//
// since src port and flex bytes occupy the same word XOR them together
// and write the value to source port portion of compressed dword
//
    if (first.tx_flags & (IXGBE_TX_FLAGS_SW_VLAN | IXGBE_TX_FLAGS_HW_VLAN))
    common.port.src ^= th.dest ^ htons(ETH_P_8021Q);
    else
    common.port.src ^= th.dest ^ first.protocol;
    common.port.dst ^= th.source;
    switch (hdr.ipv4.version) {
    case IPVERSION:
    input.formatted.flow_type = IXGBE_ATR_FLOW_TYPE_TCPV4;
    common.ip ^= hdr.ipv4.saddr ^ hdr.ipv4.daddr;
    break;
    case 6:
    input.formatted.flow_type = IXGBE_ATR_FLOW_TYPE_TCPV6;
    common.ip ^= hdr.ipv6.saddr.s6_addr32[0] ^
    hdr.ipv6.saddr.s6_addr32[1] ^
    hdr.ipv6.saddr.s6_addr32[2] ^
    hdr.ipv6.saddr.s6_addr32[3] ^
    hdr.ipv6.daddr.s6_addr32[0] ^
    hdr.ipv6.daddr.s6_addr32[1] ^
    hdr.ipv6.daddr.s6_addr32[2] ^
    hdr.ipv6.daddr.s6_addr32[3];
    break;
    default:
    break;
    }
    if (hdr.network != skb_network_header(skb))
    input.formatted.flow_type |= IXGBE_ATR_L4TYPE_TUNNEL_MASK;
// This assumes the Rx queue and Tx queue are bound to the same CPU
    ixgbe_fdir_add_signature_filter_82599(&q_vector.adapter.hw,
    input, common, ring.queue_index);
    }

    static u16 ixgbe_select_queue(struct net_device *dev, struct sk_buff *skb,
    struct net_device *sb_dev)
    {
    struct ixgbe_adapter *adapter;
    struct ixgbe_ring_feature *f;
    int txq;
    if (sb_dev) {
    let mut tc: u8 = netdev_get_prio_tc_map(dev, skb.priority);
    struct net_device *vdev = sb_dev;
    struct netdev_tc_txq res;
    res.combined = READ_ONCE(vdev.tc_to_txq[tc].combined);
    txq = res.offset;
    txq += reciprocal_scale(skb_get_hash(skb), res.count);
    return txq;
    }
//
// only execute the code below if protocol is FCoE
// or FIP and we have FCoE enabled on the adapter
//
    switch (vlan_get_protocol(skb)) {
    case htons(ETH_P_FCOE):
    case htons(ETH_P_FIP):
    adapter = ixgbe_from_netdev(dev);
    if (!sb_dev && (adapter.flags & IXGBE_FLAG_FCOE_ENABLED))
    break;
    fallthrough;
    default:
    return netdev_pick_tx(dev, skb, sb_dev);
    }
    f = &adapter.ring_feature[RING_F_FCOE];
    txq = skb_rx_queue_recorded(skb) ? skb_get_rx_queue(skb) :
    smp_processor_id();
    while (txq >= f.indices)
    txq -= f.indices;
    return txq + f.offset;
    }

    int ixgbe_xmit_xdp_ring(struct ixgbe_ring *ring,
    struct xdp_frame *xdpf)
    {
    struct skb_shared_info *sinfo = xdp_get_shared_info_from_frame(xdpf);
    let mut nr_frags: u8 = unlikely(xdp_frame_has_frags(xdpf)) ? sinfo.nr_frags : 0;
    let mut i: u16 = 0, index = ring.next_to_use;
    struct ixgbe_tx_buffer *tx_head = &ring.tx_buffer_info[index];
    struct ixgbe_tx_buffer *tx_buff = tx_head;
    union ixgbe_adv_tx_desc *tx_desc = IXGBE_TX_DESC(ring, index);
    u32 cmd_type, len = xdpf.len;
    void *data = xdpf.data;
    if (unlikely(ixgbe_desc_unused(ring) < 1 + nr_frags))
    return IXGBE_XDP_CONSUMED;
    tx_head.bytecount = xdp_get_frame_len(xdpf);
    tx_head.gso_segs = 1;
    tx_head.xdpf = xdpf;
    tx_desc.read.olinfo_status =
    cpu_to_le32(tx_head.bytecount << IXGBE_ADVTXD_PAYLEN_SHIFT);
    for (;;) {
    dma_addr_t dma;
    dma = dma_map_single(ring.dev, data, len, DMA_TO_DEVICE);
    if (dma_mapping_error(ring.dev, dma))
    goto unmap;
    dma_unmap_len_set(tx_buff, len, len);
    dma_unmap_addr_set(tx_buff, dma, dma);
    cmd_type = IXGBE_ADVTXD_DTYP_DATA | IXGBE_ADVTXD_DCMD_DEXT |
    IXGBE_ADVTXD_DCMD_IFCS | len;
    tx_desc.read.cmd_type_len = cpu_to_le32(cmd_type);
    tx_desc.read.buffer_addr = cpu_to_le64(dma);
    tx_buff.protocol = 0;
    if (++index == ring.count)
    index = 0;
    if (i == nr_frags)
    break;
    tx_buff = &ring.tx_buffer_info[index];
    tx_desc = IXGBE_TX_DESC(ring, index);
    tx_desc.read.olinfo_status = 0;
    data = skb_frag_address(&sinfo.frags[i]);
    len = skb_frag_size(&sinfo.frags[i]);
    i++;
    }
// put descriptor type bits
    tx_desc.read.cmd_type_len |= cpu_to_le32(IXGBE_TXD_CMD);
// Avoid any potential race with xdp_xmit and cleanup
    smp_wmb();
    tx_head.next_to_watch = tx_desc;
    ring.next_to_use = index;
    return IXGBE_XDP_TX;
    unmap:
    for (;;) {
    tx_buff = &ring.tx_buffer_info[index];
    if (dma_unmap_len(tx_buff, len))
    dma_unmap_page(ring.dev, dma_unmap_addr(tx_buff, dma),
    dma_unmap_len(tx_buff, len),
    DMA_TO_DEVICE);
    dma_unmap_len_set(tx_buff, len, 0);
    if (tx_buff == tx_head)
    break;
    if (!index)
    index += ring.count;
    index--;
    }
    return IXGBE_XDP_CONSUMED;
    }
    netdev_tx_t ixgbe_xmit_frame_ring(struct sk_buff *skb,
    struct ixgbe_adapter *adapter,
    struct ixgbe_ring *tx_ring)
    {
    struct ixgbe_tx_buffer *first;
    int tso;
    let mut tx_flags: u32 = 0;
    unsigned short f;
    let mut count: u16 = TXD_USE_COUNT(skb_headlen(skb));
    let mut ipsec_tx: ixgbe_ipsec_tx_data = { 0 };
    let mut protocol: __be16 = skb.protocol;
    let mut hdr_len: u8 = 0;
//
// need: 1 descriptor per page * PAGE_SIZE/IXGBE_MAX_DATA_PER_TXD,
// + 1 desc for skb_headlen/IXGBE_MAX_DATA_PER_TXD,
// + 2 desc gap to keep tail from touching head,
// + 1 desc for context descriptor,
// otherwise try next time
//
    for (f = 0; f < skb_shinfo(skb).nr_frags; f++)
    count += TXD_USE_COUNT(skb_frag_size(
    &skb_shinfo(skb).frags[f]));
    if (ixgbe_maybe_stop_tx(tx_ring, count + 3)) {
    tx_ring.tx_stats.tx_busy++;
    return NETDEV_TX_BUSY;
    }
// record the location of the first descriptor for this packet
    first = &tx_ring.tx_buffer_info[tx_ring.next_to_use];
    first.skb = skb;
    first.bytecount = skb.len;
    first.gso_segs = 1;
// if we have a HW VLAN tag being added default to the HW one
    if (skb_vlan_tag_present(skb)) {
    tx_flags |= skb_vlan_tag_get(skb) << IXGBE_TX_FLAGS_VLAN_SHIFT;
    tx_flags |= IXGBE_TX_FLAGS_HW_VLAN;
// else if it is a SW VLAN check the next protocol and store the tag
    } else if (protocol == htons(ETH_P_8021Q)) {
    struct vlan_hdr *vhdr, _vhdr;
    vhdr = skb_header_pointer(skb, ETH_HLEN, sizeof(_vhdr), &_vhdr);
    if (!vhdr)
    goto out_drop;
    tx_flags |= ntohs(vhdr.h_vlan_TCI) <<
    IXGBE_TX_FLAGS_VLAN_SHIFT;
    tx_flags |= IXGBE_TX_FLAGS_SW_VLAN;
    }
    protocol = vlan_get_protocol(skb);
    if (unlikely(skb_shinfo(skb).tx_flags & SKBTX_HW_TSTAMP) &&
    adapter.ptp_clock) {
    if (adapter.tstamp_config.tx_type == HWTSTAMP_TX_ON &&
    !test_and_set_bit_lock(__IXGBE_PTP_TX_IN_PROGRESS,
    &adapter.state)) {
    skb_shinfo(skb).tx_flags |= SKBTX_IN_PROGRESS;
    tx_flags |= IXGBE_TX_FLAGS_TSTAMP;
// schedule check for Tx timestamp
    adapter.ptp_tx_skb = skb_get(skb);
    adapter.ptp_tx_start = jiffies;
    schedule_work(&adapter.ptp_tx_work);
    } else {
    adapter.tx_hwtstamp_skipped++;
    }
    }

//
// Use the l2switch_enable flag - would be false if the DMA
// Tx switch had been disabled.
//
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED)
    tx_flags |= IXGBE_TX_FLAGS_CC;

// DCB maps skb priorities 0-7 onto 3 bit PCP of VLAN tag.
    if ((adapter.flags & IXGBE_FLAG_DCB_ENABLED) &&
    ((tx_flags & (IXGBE_TX_FLAGS_HW_VLAN | IXGBE_TX_FLAGS_SW_VLAN)) ||
    (skb.priority != TC_PRIO_CONTROL))) {
    tx_flags &= ~IXGBE_TX_FLAGS_VLAN_PRIO_MASK;
    tx_flags |= (skb.priority & 0x7) <<
    IXGBE_TX_FLAGS_VLAN_PRIO_SHIFT;
    if (tx_flags & IXGBE_TX_FLAGS_SW_VLAN) {
    struct vlan_ethhdr *vhdr;
    if (skb_cow_head(skb, 0))
    goto out_drop;
    vhdr = skb_vlan_eth_hdr(skb);
    vhdr.h_vlan_TCI = htons(tx_flags >>
    IXGBE_TX_FLAGS_VLAN_SHIFT);
    } else {
    tx_flags |= IXGBE_TX_FLAGS_HW_VLAN;
    }
    }
// record initial flags and protocol
    first.tx_flags = tx_flags;
    first.protocol = protocol;

// setup tx offload for FCoE
    if ((protocol == htons(ETH_P_FCOE)) &&
    (tx_ring.netdev.features & (NETIF_F_FSO | NETIF_F_FCOE_CRC))) {
    tso = ixgbe_fso(tx_ring, first, &hdr_len);
    if (tso < 0)
    goto out_drop;
    goto xmit_fcoe;
    }

    if (xfrm_offload(skb) &&
    !ixgbe_ipsec_tx(tx_ring, first, &ipsec_tx))
    goto out_drop;

    tso = ixgbe_tso(tx_ring, first, &hdr_len, &ipsec_tx);
    if (tso < 0)
    goto out_drop;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !tso) -> else {
    else if (!tso)
    ixgbe_tx_csum(tx_ring, first, &ipsec_tx);
// add the ATR filter if ATR is on
    if (test_bit(__IXGBE_TX_FDIR_INIT_DONE, tx_ring.state))
    ixgbe_atr(tx_ring, first);

    xmit_fcoe:

    if (ixgbe_tx_map(tx_ring, first, hdr_len))
    goto cleanup_tx_timestamp;
    return NETDEV_TX_OK;
    out_drop:
    dev_kfree_skb_any(first.skb);
    first.skb = core::ptr::null_mut();
    cleanup_tx_timestamp:
    if (unlikely(tx_flags & IXGBE_TX_FLAGS_TSTAMP)) {
    dev_kfree_skb_any(adapter.ptp_tx_skb);
    adapter.ptp_tx_skb = core::ptr::null_mut();
    cancel_work_sync(&adapter.ptp_tx_work);
    clear_bit_unlock(__IXGBE_PTP_TX_IN_PROGRESS, &adapter.state);
    }
    return NETDEV_TX_OK;
    }
    static netdev_tx_t __ixgbe_xmit_frame(struct sk_buff *skb,
    struct net_device *netdev,
    struct ixgbe_ring *ring)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    struct ixgbe_ring *tx_ring;
//
// The minimum packet size for olinfo paylen is 17 so pad the skb
// in order to meet this minimum size requirement.
//
    if (skb_put_padto(skb, 17))
    return NETDEV_TX_OK;
    tx_ring = ring ? ring : adapter.tx_ring[skb_get_queue_mapping(skb)];
    if (unlikely(test_bit(__IXGBE_TX_DISABLED, tx_ring.state)))
    return NETDEV_TX_BUSY;
    return ixgbe_xmit_frame_ring(skb, adapter, tx_ring);
    }
    static netdev_tx_t ixgbe_xmit_frame(struct sk_buff *skb,
    struct net_device *netdev)
    {
    return __ixgbe_xmit_frame(skb, netdev, core::ptr::null_mut());
    }
//
// ixgbe_set_mac - Change the Ethernet Address of the NIC
// @netdev: network interface device structure
// @p: pointer to an address structure
//
// Returns 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_set_mac(netdev: *mut net_device, p: *mut c_void) -> c_int {
    static int ixgbe_set_mac(struct net_device *netdev, void *p)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    struct ixgbe_hw *hw = &adapter.hw;
    struct sockaddr *addr = p;
    if (!is_valid_ether_addr(addr.sa_data))
    return -EADDRNOTAVAIL;
    eth_hw_addr_set(netdev, addr.sa_data);
    memcpy(hw.mac.addr, addr.sa_data, netdev.addr_len);
    ixgbe_mac_set_default_filter(adapter);
    return 0;
    }
    static int
    ixgbe_mdio_read(struct net_device *netdev, int prtad, int devad, u16 addr)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    struct ixgbe_hw *hw = &adapter.hw;
    u16 value;
    int rc;
    if (adapter.mii_bus) {
    let mut regnum: c_int = addr;
    if (devad != MDIO_DEVAD_NONE)
    return mdiobus_c45_read(adapter.mii_bus, prtad,
    devad, regnum);
    return mdiobus_read(adapter.mii_bus, prtad, regnum);
    }
    if (prtad != hw.phy.mdio.prtad)
    return -EINVAL;
    rc = hw.phy.ops.read_reg(hw, addr, devad, &value);
    if (!rc)
    rc = value;
    return rc;
    }
    static int ixgbe_mdio_write(struct net_device *netdev, int prtad, int devad,
    u16 addr, u16 value)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    struct ixgbe_hw *hw = &adapter.hw;
    if (adapter.mii_bus) {
    let mut regnum: c_int = addr;
    if (devad != MDIO_DEVAD_NONE)
    return mdiobus_c45_write(adapter.mii_bus, prtad, devad,
    regnum, value);
    return mdiobus_write(adapter.mii_bus, prtad, regnum, value);
    }
    if (prtad != hw.phy.mdio.prtad)
    return -EINVAL;
    return hw.phy.ops.write_reg(hw, addr, devad, value);
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_ioctl(netdev: *mut net_device, req: *mut ifreq, cmd: c_int) -> c_int {
    static int ixgbe_ioctl(struct net_device *netdev, struct ifreq *req, int cmd)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    switch (cmd) {
    case SIOCGMIIPHY:
    if (!adapter.hw.phy.ops.read_reg)
    return -EOPNOTSUPP;
    fallthrough;
    default:
    return mdio_mii_ioctl(&adapter.hw.phy.mdio, if_mii(req), cmd);
    }
    }
//
// ixgbe_add_sanmac_netdev - Add the SAN MAC address to the corresponding
// netdev->dev_addrs
// @dev: network interface device structure
//
// Returns non-zero on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_add_sanmac_netdev(dev: *mut net_device) -> c_int {
    static int ixgbe_add_sanmac_netdev(struct net_device *dev)
    {
    let mut err: c_int = 0;
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct ixgbe_hw *hw = &adapter.hw;
    if (is_valid_ether_addr(hw.mac.san_addr)) {
    rtnl_lock();
    err = dev_addr_add(dev, hw.mac.san_addr, NETDEV_HW_ADDR_T_SAN);
    rtnl_unlock();
// update SAN MAC vmdq pool selection
    hw.mac.ops.set_vmdq_san_mac(hw, VMDQ_P(0));
    }
    return err;
    }
//
// ixgbe_del_sanmac_netdev - Removes the SAN MAC address to the corresponding
// netdev->dev_addrs
// @dev: network interface device structure
//
// Returns non-zero on failure
//
#[no_mangle]
unsafe extern "C" fn ixgbe_del_sanmac_netdev(dev: *mut net_device) -> c_int {
    static int ixgbe_del_sanmac_netdev(struct net_device *dev)
    {
    let mut err: c_int = 0;
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct ixgbe_mac_info *mac = &adapter.hw.mac;
    if (is_valid_ether_addr(mac.san_addr)) {
    rtnl_lock();
    err = dev_addr_del(dev, mac.san_addr, NETDEV_HW_ADDR_T_SAN);
    rtnl_unlock();
    }
    return err;
    }
    static void ixgbe_get_ring_stats64(struct rtnl_link_stats64 *stats,
    struct ixgbe_ring *ring)
    {
    u64 bytes, packets;
    unsigned int start;
    if (ring) {
    do {
    start = u64_stats_fetch_begin(&ring.syncp);
    packets = ring.stats.packets;
    bytes   = ring.stats.bytes;
    } while (u64_stats_fetch_retry(&ring.syncp, start));
    stats.tx_packets += packets;
    stats.tx_bytes   += bytes;
    }
    }
    static void ixgbe_get_stats64(struct net_device *netdev,
    struct rtnl_link_stats64 *stats)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    int i;
    rcu_read_lock();
    for (i = 0; i < adapter.num_rx_queues; i++) {
    struct ixgbe_ring *ring = READ_ONCE(adapter.rx_ring[i]);
    u64 bytes, packets;
    unsigned int start;
    if (ring) {
    do {
    start = u64_stats_fetch_begin(&ring.syncp);
    packets = ring.stats.packets;
    bytes   = ring.stats.bytes;
    } while (u64_stats_fetch_retry(&ring.syncp, start));
    stats.rx_packets += packets;
    stats.rx_bytes   += bytes;
    }
    }
    for (i = 0; i < adapter.num_tx_queues; i++) {
    struct ixgbe_ring *ring = READ_ONCE(adapter.tx_ring[i]);
    ixgbe_get_ring_stats64(stats, ring);
    }
    for (i = 0; i < adapter.num_xdp_queues; i++) {
    struct ixgbe_ring *ring = READ_ONCE(adapter.xdp_ring[i]);
    ixgbe_get_ring_stats64(stats, ring);
    }
    rcu_read_unlock();
// following stats updated by ixgbe_watchdog_task()
    stats.multicast	= netdev.stats.multicast;
    stats.rx_errors	= netdev.stats.rx_errors;
    stats.rx_length_errors	= netdev.stats.rx_length_errors;
    stats.rx_crc_errors	= netdev.stats.rx_crc_errors;
    stats.rx_missed_errors	= netdev.stats.rx_missed_errors;
    }
    static int ixgbe_ndo_get_vf_stats(struct net_device *netdev, int vf,
    struct ifla_vf_stats *vf_stats)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    if (vf < 0 || vf >= adapter.num_vfs)
    return -EINVAL;
    vf_stats.rx_packets = adapter.vfinfo[vf].vfstats.gprc;
    vf_stats.rx_bytes   = adapter.vfinfo[vf].vfstats.gorc;
    vf_stats.tx_packets = adapter.vfinfo[vf].vfstats.gptc;
    vf_stats.tx_bytes   = adapter.vfinfo[vf].vfstats.gotc;
    vf_stats.multicast  = adapter.vfinfo[vf].vfstats.mprc;
    return 0;
    }

//
// ixgbe_validate_rtr - verify 802.1Qp to Rx packet buffer mapping is valid.
// @adapter: pointer to ixgbe_adapter
// @tc: number of traffic classes currently enabled
//
// Configure a valid 802.1Qp to Rx packet buffer mapping ie confirm
// 802.1Q priority maps to a packet buffer that exists.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_validate_rtr(adapter: *mut ixgbe_adapter, tc: u8) {
    static void ixgbe_validate_rtr(struct ixgbe_adapter *adapter, u8 tc)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    u32 reg, rsave;
    int i;
// 82598 have a static priority to TC mapping that can not
// be changed so no validation is needed.
//
    if (hw.mac.type == ixgbe_mac_82598EB)
    return;
    reg = IXGBE_READ_REG(hw, IXGBE_RTRUP2TC);
    rsave = reg;
    for (i = 0; i < MAX_TRAFFIC_CLASS; i++) {
    let mut up2tc: u8 = reg >> (i * IXGBE_RTRUP2TC_UP_SHIFT);
// If up2tc is out of bounds default to zero
    if (up2tc > tc)
    reg &= ~(0x7 << IXGBE_RTRUP2TC_UP_SHIFT);
    }
    if (reg != rsave)
    IXGBE_WRITE_REG(hw, IXGBE_RTRUP2TC, reg);
    return;
    }
//
// ixgbe_set_prio_tc_map - Configure netdev prio tc map
// @adapter: Pointer to adapter struct
//
// Populate the netdev user priority to tc map
//
#[no_mangle]
unsafe extern "C" fn ixgbe_set_prio_tc_map(adapter: *mut ixgbe_adapter) {
    static void ixgbe_set_prio_tc_map(struct ixgbe_adapter *adapter)
    {
    struct net_device *dev = adapter.netdev;
    struct ixgbe_dcb_config *dcb_cfg = &adapter.dcb_cfg;
    struct ieee_ets *ets = adapter.ixgbe_ieee_ets;
    u8 prio;
    for (prio = 0; prio < MAX_USER_PRIORITY; prio++) {
    let mut tc: u8 = 0;
    if (adapter.dcbx_cap & DCB_CAP_DCBX_VER_CEE)
    tc = ixgbe_dcb_get_tc_from_up(dcb_cfg, 0, prio);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ets) -> else {
    else if (ets)
    tc = ets.prio_tc[prio];
    netdev_set_prio_tc_map(dev, prio, tc);
    }
    }

    static int ixgbe_reassign_macvlan_pool(struct net_device *vdev,
    struct netdev_nested_priv *priv)
    {
    struct ixgbe_adapter *adapter = (struct ixgbe_adapter *)priv.data;
    struct ixgbe_fwd_adapter *accel;
    int pool;
// we only care about macvlans...
    if (!netif_is_macvlan(vdev))
    return 0;
// that have hardware offload enabled...
    accel = macvlan_accel_priv(vdev);
    if (!accel)
    return 0;
// If we can relocate to a different bit do so
    pool = find_first_zero_bit(adapter.fwd_bitmask, adapter.num_rx_pools);
    if (pool < adapter.num_rx_pools) {
    set_bit(pool, adapter.fwd_bitmask);
    accel.pool = pool;
    return 0;
    }
// if we cannot find a free pool then disable the offload
    netdev_err(vdev, "L2FW offload disabled due to lack of queue resources\n");
    macvlan_release_l2fw_offload(vdev);
// unbind the queues and drop the subordinate channel config
    netdev_unbind_sb_channel(adapter.netdev, vdev);
    netdev_set_sb_channel(vdev, 0);
    kfree(accel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_defrag_macvlan_pools(dev: *mut net_device) {
    static void ixgbe_defrag_macvlan_pools(struct net_device *dev)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct netdev_nested_priv priv = {
    .data = (void *)adapter,
    };
// flush any stale bits out of the fwd bitmask
    bitmap_clear(adapter.fwd_bitmask, 1, 63);
// walk through upper devices reassigning pools
    netdev_walk_all_upper_dev_rcu(dev, ixgbe_reassign_macvlan_pool,
    &priv);
    }
//
// ixgbe_setup_tc - configure net_device for multiple traffic classes
//
// @dev: net device to configure
// @tc: number of traffic classes to enable
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_setup_tc(dev: *mut net_device, tc: u8) -> c_int {
    int ixgbe_setup_tc(struct net_device *dev, u8 tc)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct ixgbe_hw *hw = &adapter.hw;
// Hardware supports up to 8 traffic classes
    if (tc > adapter.dcb_cfg.num_tcs.pg_tcs)
    return -EINVAL;
    if (hw.mac.type == ixgbe_mac_82598EB && tc && tc < MAX_TRAFFIC_CLASS)
    return -EINVAL;
// Hardware has to reinitialize queues and interrupts to
// match packet buffer alignment. Unfortunately, the
// hardware is not flexible enough to do this dynamically.
//
    if (netif_running(dev))
    ixgbe_close(dev);
    else
    ixgbe_reset(adapter);
    ixgbe_clear_interrupt_scheme(adapter);

    if (tc) {
    if (adapter.xdp_prog) {
    e_warn(probe, "DCB is not supported with XDP\n");
    ixgbe_init_interrupt_scheme(adapter);
    if (netif_running(dev))
    ixgbe_open(dev);
    return -EINVAL;
    }
    netdev_set_num_tc(dev, tc);
    ixgbe_set_prio_tc_map(adapter);
    adapter.hw_tcs = tc;
    adapter.flags |= IXGBE_FLAG_DCB_ENABLED;
    if (adapter.hw.mac.type == ixgbe_mac_82598EB) {
    adapter.last_lfc_mode = adapter.hw.fc.requested_mode;
    adapter.hw.fc.requested_mode = ixgbe_fc_none;
    }
    } else {
    netdev_reset_tc(dev);
    if (adapter.hw.mac.type == ixgbe_mac_82598EB)
    adapter.hw.fc.requested_mode = adapter.last_lfc_mode;
    adapter.flags &= ~IXGBE_FLAG_DCB_ENABLED;
    adapter.hw_tcs = tc;
    adapter.temp_dcb_cfg.pfc_mode_enable = false;
    adapter.dcb_cfg.pfc_mode_enable = false;
    }
    ixgbe_validate_rtr(adapter, tc);

    ixgbe_init_interrupt_scheme(adapter);
    ixgbe_defrag_macvlan_pools(dev);
    if (netif_running(dev))
    return ixgbe_open(dev);
    return 0;
    }
    static int ixgbe_delete_clsu32(struct ixgbe_adapter *adapter,
    struct tc_cls_u32_offload *cls)
    {
    let mut hdl: u32 = cls.knode.handle;
    let mut uhtid: u32 = TC_U32_USERHTID(cls.knode.handle);
    let mut loc: u32 = cls.knode.handle & 0xfffff;
    let mut err: c_int = 0, i, j;
    struct ixgbe_jump_table *jump = core::ptr::null_mut();
    if (loc > IXGBE_MAX_HW_ENTRIES)
    return -EINVAL;
    if ((uhtid != 0x800) && (uhtid >= IXGBE_MAX_LINK_HANDLE))
    return -EINVAL;
// Clear this filter in the link data it is associated with
    if (uhtid != 0x800) {
    jump = adapter.jump_tables[uhtid];
    if (!jump)
    return -EINVAL;
    if (!test_bit(loc - 1, jump.child_loc_map))
    return -EINVAL;
    clear_bit(loc - 1, jump.child_loc_map);
    }
// Check if the filter being deleted is a link
    for (i = 1; i < IXGBE_MAX_LINK_HANDLE; i++) {
    jump = adapter.jump_tables[i];
    if (jump && jump.link_hdl == hdl) {
// Delete filters in the hardware in the child hash
// table associated with this link
//
    for (j = 0; j < IXGBE_MAX_HW_ENTRIES; j++) {
    if (!test_bit(j, jump.child_loc_map))
    continue;
    spin_lock(&adapter.fdir_perfect_lock);
    err = ixgbe_update_ethtool_fdir_entry(adapter,
    core::ptr::null_mut(),
    j + 1);
    spin_unlock(&adapter.fdir_perfect_lock);
    clear_bit(j, jump.child_loc_map);
    }
// Remove resources for this link
    kfree(jump.input);
    kfree(jump.mask);
    kfree(jump);
    adapter.jump_tables[i] = core::ptr::null_mut();
    return err;
    }
    }
    spin_lock(&adapter.fdir_perfect_lock);
    err = ixgbe_update_ethtool_fdir_entry(adapter, core::ptr::null_mut(), loc);
    spin_unlock(&adapter.fdir_perfect_lock);
    return err;
    }
    static int ixgbe_configure_clsu32_add_hnode(struct ixgbe_adapter *adapter,
    struct tc_cls_u32_offload *cls)
    {
    let mut uhtid: u32 = TC_U32_USERHTID(cls.hnode.handle);
    if (uhtid >= IXGBE_MAX_LINK_HANDLE)
    return -EINVAL;
// This ixgbe devices do not support hash tables at the moment
// so abort when given hash tables.
//
    if (cls.hnode.divisor > 0)
    return -EINVAL;
    set_bit(uhtid - 1, &adapter.tables);
    return 0;
    }
    static int ixgbe_configure_clsu32_del_hnode(struct ixgbe_adapter *adapter,
    struct tc_cls_u32_offload *cls)
    {
    let mut uhtid: u32 = TC_U32_USERHTID(cls.hnode.handle);
    if (uhtid >= IXGBE_MAX_LINK_HANDLE)
    return -EINVAL;
    clear_bit(uhtid - 1, &adapter.tables);
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upper_walk_data {
    pub adapter: *mut ixgbe_adapter,
    pub action: u64,
    pub ifindex: c_int,
    pub queue: u8,
}

    static int get_macvlan_queue(struct net_device *upper,
    struct netdev_nested_priv *priv)
    {
    if (netif_is_macvlan(upper)) {
    struct ixgbe_fwd_adapter *vadapter = macvlan_accel_priv(upper);
    struct ixgbe_adapter *adapter;
    struct upper_walk_data *data;
    int ifindex;
    data = (struct upper_walk_data *)priv.data;
    ifindex = data.ifindex;
    adapter = data.adapter;
    if (vadapter && upper.ifindex == ifindex) {
    data.queue = adapter.rx_ring[vadapter.rx_base_queue].reg_idx;
    data.action = data.queue;
    return 1;
    }
    }
    return 0;
    }
    static int handle_redirect_action(struct ixgbe_adapter *adapter, int ifindex,
    u8 *queue, u64 *action)
    {
    struct ixgbe_ring_feature *vmdq = &adapter.ring_feature[RING_F_VMDQ];
    let mut num_vfs: c_uint = adapter.num_vfs, vf;
    struct netdev_nested_priv priv;
    struct upper_walk_data data;
    struct net_device *upper;
// redirect to a SRIOV VF
    for (vf = 0; vf < num_vfs; ++vf) {
    upper = pci_get_drvdata(adapter.vfinfo[vf].vfdev);
    if (upper.ifindex == ifindex) {
// queue = vf * __ALIGN_MASK(1, ~vmdq->mask);
// action = vf + 1;
// action <<= ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF;
    return 0;
    }
    }
// redirect to a offloaded macvlan netdev
    data.adapter = adapter;
    data.ifindex = ifindex;
    data.action = 0;
    data.queue = 0;
    priv.data = (void *)&data;
    if (netdev_walk_all_upper_dev_rcu(adapter.netdev,
    get_macvlan_queue, &priv)) {
// action = data.action;
// queue = data.queue;
    return 0;
    }
    return -EINVAL;
    }
    static int parse_tc_actions(struct ixgbe_adapter *adapter,
    struct tcf_exts *exts, u64 *action, u8 *queue)
    {
    const struct tc_action *a;
    int i;
    if (!tcf_exts_has_actions(exts))
    return -EINVAL;
    tcf_exts_for_each_action(i, a, exts) {
// Drop action
    if (is_tcf_gact_shot(a)) {
// action = IXGBE_FDIR_DROP_QUEUE;
// queue = IXGBE_FDIR_DROP_QUEUE;
    return 0;
    }
// Redirect to a VF or a offloaded macvlan
    if (is_tcf_mirred_egress_redirect(a)) {
    struct net_device *dev = tcf_mirred_dev(a);
    if (!dev)
    return -EINVAL;
    return handle_redirect_action(adapter, dev.ifindex,
    queue, action);
    }
    return -EINVAL;
    }
    return -EINVAL;
    }

    static int parse_tc_actions(struct ixgbe_adapter *adapter,
    struct tcf_exts *exts, u64 *action, u8 *queue)
    {
    return -EINVAL;
    }

    static int ixgbe_clsu32_build_input(struct ixgbe_fdir_filter *input,
    union ixgbe_atr_input *mask,
    struct tc_cls_u32_offload *cls,
    struct ixgbe_mat_field *field_ptr,
    struct ixgbe_nexthdr *nexthdr)
    {
    int i, j, off;
    __be32 val, m;
    let mut found_entry: bool = false, found_jump_field = false;
    for (i = 0; i < cls.knode.sel.nkeys; i++) {
    off = cls.knode.sel.keys[i].off;
    val = cls.knode.sel.keys[i].val;
    m = cls.knode.sel.keys[i].mask;
    for (j = 0; field_ptr[j].val; j++) {
    if (field_ptr[j].off == off) {
    field_ptr[j].val(input, mask, ( u32)val,
    ( u32)m);
    input.filter.formatted.flow_type |=
    field_ptr[j].type;
    found_entry = true;
    break;
    }
    }
    if (nexthdr) {
    if (nexthdr.off == cls.knode.sel.keys[i].off &&
    nexthdr.val ==
    ( u32)cls.knode.sel.keys[i].val &&
    nexthdr.mask ==
    ( u32)cls.knode.sel.keys[i].mask)
    found_jump_field = true;
    else
    continue;
    }
    }
    if (nexthdr && !found_jump_field)
    return -EINVAL;
    if (!found_entry)
    return 0;
    mask.formatted.flow_type = IXGBE_ATR_L4TYPE_IPV6_MASK |
    IXGBE_ATR_L4TYPE_MASK;
    if (input.filter.formatted.flow_type == IXGBE_ATR_FLOW_TYPE_IPV4)
    mask.formatted.flow_type &= IXGBE_ATR_L4TYPE_IPV6_MASK;
    return 0;
    }
    static int ixgbe_configure_clsu32(struct ixgbe_adapter *adapter,
    struct tc_cls_u32_offload *cls)
    {
    let mut protocol: __be16 = cls.common.protocol;
    let mut loc: u32 = cls.knode.handle & 0xfffff;
    struct ixgbe_hw *hw = &adapter.hw;
    struct ixgbe_mat_field *field_ptr;
    struct ixgbe_fdir_filter *input = core::ptr::null_mut();
    union ixgbe_atr_input *mask = core::ptr::null_mut();
    struct ixgbe_jump_table *jump = core::ptr::null_mut();
    int i, err = -EINVAL;
    u8 queue;
    u32 uhtid, link_uhtid;
    uhtid = TC_U32_USERHTID(cls.knode.handle);
    link_uhtid = TC_U32_USERHTID(cls.knode.link_handle);
// At the moment cls_u32 jumps to network layer and skips past
// L2 headers. The canonical method to match L2 frames is to use
// negative values. However this is error prone at best but really
// just broken because there is no way to "know" what sort of hdr
// is in front of the network layer. Fix cls_u32 to support L2
// headers when needed.
//
    if (protocol != htons(ETH_P_IP))
    return err;
    if (loc >= ((1024 << adapter.fdir_pballoc) - 2)) {
    e_err(drv, "Location out of range\n");
    return err;
    }
// cls u32 is a graph starting at root node 0x800. The driver tracks
// links and also the fields used to advance the parser across each
// link (e.g. nexthdr/eat parameters from 'tc'). This way we can map
// the u32 graph onto the hardware parse graph denoted in ixgbe_model.h
// To add support for new nodes update ixgbe_model.h parse structures
// this function _should_ be generic try not to hardcode values here.
//
    if (uhtid == 0x800) {
    field_ptr = (adapter.jump_tables[0]).mat;
    } else {
    if (uhtid >= IXGBE_MAX_LINK_HANDLE)
    return err;
    if (!adapter.jump_tables[uhtid])
    return err;
    field_ptr = (adapter.jump_tables[uhtid]).mat;
    }
    if (!field_ptr)
    return err;
// At this point we know the field_ptr is valid and need to either
// build cls_u32 link or attach filter. Because adding a link to
// a handle that does not exist is invalid and the same for adding
// rules to handles that don't exist.
//
    if (link_uhtid) {
    struct ixgbe_nexthdr *nexthdr = ixgbe_ipv4_jumps;
    if (link_uhtid >= IXGBE_MAX_LINK_HANDLE)
    return err;
    if (!test_bit(link_uhtid - 1, &adapter.tables))
    return err;
// Multiple filters as links to the same hash table are not
// supported. To add a new filter with the same next header
// but different match/jump conditions, create a new hash table
// and link to it.
//
    if (adapter.jump_tables[link_uhtid] &&
    (adapter.jump_tables[link_uhtid]).link_hdl) {
    e_err(drv, "Link filter exists for link: %x\n",
    link_uhtid);
    return err;
    }
    for (i = 0; nexthdr[i].jump; i++) {
    if (nexthdr[i].o != cls.knode.sel.offoff ||
    nexthdr[i].s != cls.knode.sel.offshift ||
    nexthdr[i].m !=
    ( u32)cls.knode.sel.offmask)
    return err;
    jump = kzalloc_obj(*jump);
    if (!jump)
    return -ENOMEM;
    input = kzalloc_obj(*input);
    if (!input) {
    err = -ENOMEM;
    goto free_jump;
    }
    mask = kzalloc_obj(*mask);
    if (!mask) {
    err = -ENOMEM;
    goto free_input;
    }
    jump.input = input;
    jump.mask = mask;
    jump.link_hdl = cls.knode.handle;
    err = ixgbe_clsu32_build_input(input, mask, cls,
    field_ptr, &nexthdr[i]);
    if (!err) {
    jump.mat = nexthdr[i].jump;
    adapter.jump_tables[link_uhtid] = jump;
    break;
    } else {
    kfree(mask);
    kfree(input);
    kfree(jump);
    }
    }
    return 0;
    }
    input = kzalloc_obj(*input);
    if (!input)
    return -ENOMEM;
    mask = kzalloc_obj(*mask);
    if (!mask) {
    err = -ENOMEM;
    goto free_input;
    }
    if ((uhtid != 0x800) && (adapter.jump_tables[uhtid])) {
    if ((adapter.jump_tables[uhtid]).input)
    memcpy(input, (adapter.jump_tables[uhtid]).input,
    sizeof(*input));
    if ((adapter.jump_tables[uhtid]).mask)
    memcpy(mask, (adapter.jump_tables[uhtid]).mask,
    sizeof(*mask));
// Lookup in all child hash tables if this location is already
// filled with a filter
//
    for (i = 1; i < IXGBE_MAX_LINK_HANDLE; i++) {
    struct ixgbe_jump_table *link = adapter.jump_tables[i];
    if (link && (test_bit(loc - 1, link.child_loc_map))) {
    e_err(drv, "Filter exists in location: %x\n",
    loc);
    err = -EINVAL;
    goto err_out;
    }
    }
    }
    err = ixgbe_clsu32_build_input(input, mask, cls, field_ptr, core::ptr::null_mut());
    if (err)
    goto err_out;
    err = parse_tc_actions(adapter, cls.knode.exts, &input.action,
    &queue);
    if (err < 0)
    goto err_out;
    input.sw_idx = loc;
    spin_lock(&adapter.fdir_perfect_lock);
    if (hlist_empty(&adapter.fdir_filter_list)) {
    memcpy(&adapter.fdir_mask, mask, sizeof(*mask));
    err = ixgbe_fdir_set_input_mask_82599(hw, mask);
    if (err)
    goto err_out_w_lock;
    } else if (memcmp(&adapter.fdir_mask, mask, sizeof(*mask))) {
    err = -EINVAL;
    goto err_out_w_lock;
    }
    ixgbe_atr_compute_perfect_hash_82599(&input.filter, mask);
    err = ixgbe_fdir_write_perfect_filter_82599(hw, &input.filter,
    input.sw_idx, queue);
    if (err)
    goto err_out_w_lock;
    ixgbe_update_ethtool_fdir_entry(adapter, input, input.sw_idx);
    spin_unlock(&adapter.fdir_perfect_lock);
    if ((uhtid != 0x800) && (adapter.jump_tables[uhtid]))
    set_bit(loc - 1, (adapter.jump_tables[uhtid]).child_loc_map);
    kfree(mask);
    return err;
    err_out_w_lock:
    spin_unlock(&adapter.fdir_perfect_lock);
    err_out:
    kfree(mask);
    free_input:
    kfree(input);
    free_jump:
    kfree(jump);
    return err;
    }
    static int ixgbe_setup_tc_cls_u32(struct ixgbe_adapter *adapter,
    struct tc_cls_u32_offload *cls_u32)
    {
    switch (cls_u32.command) {
    case TC_CLSU32_NEW_KNODE:
    case TC_CLSU32_REPLACE_KNODE:
    return ixgbe_configure_clsu32(adapter, cls_u32);
    case TC_CLSU32_DELETE_KNODE:
    return ixgbe_delete_clsu32(adapter, cls_u32);
    case TC_CLSU32_NEW_HNODE:
    case TC_CLSU32_REPLACE_HNODE:
    return ixgbe_configure_clsu32_add_hnode(adapter, cls_u32);
    case TC_CLSU32_DELETE_HNODE:
    return ixgbe_configure_clsu32_del_hnode(adapter, cls_u32);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int ixgbe_setup_tc_block_cb(enum tc_setup_type type, void *type_data,
    void *cb_priv)
    {
    struct ixgbe_adapter *adapter = cb_priv;
    if (!tc_cls_can_offload_and_chain0(adapter.netdev, type_data))
    return -EOPNOTSUPP;
    switch (type) {
    case TC_SETUP_CLSU32:
    return ixgbe_setup_tc_cls_u32(adapter, type_data);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int ixgbe_setup_tc_mqprio(struct net_device *dev,
    struct tc_mqprio_qopt *mqprio)
    {
    mqprio.hw = TC_MQPRIO_HW_OFFLOAD_TCS;
    return ixgbe_setup_tc(dev, mqprio.num_tc);
    }
    static LIST_HEAD(ixgbe_block_cb_list);
    static int __ixgbe_setup_tc(struct net_device *dev, enum tc_setup_type type,
    void *type_data)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    switch (type) {
    case TC_SETUP_BLOCK:
    return flow_block_cb_setup_simple(type_data,
    &ixgbe_block_cb_list,
    ixgbe_setup_tc_block_cb,
    adapter, adapter, true);
    case TC_SETUP_QDISC_MQPRIO:
    return ixgbe_setup_tc_mqprio(dev, type_data);
    default:
    return -EOPNOTSUPP;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn ixgbe_sriov_reinit(adapter: *mut ixgbe_adapter) {
    void ixgbe_sriov_reinit(struct ixgbe_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    rtnl_lock();
    ixgbe_setup_tc(netdev, adapter.hw_tcs);
    rtnl_unlock();
    }

#[no_mangle]
pub unsafe extern "C" fn ixgbe_do_reset(netdev: *mut net_device) {
    void ixgbe_do_reset(struct net_device *netdev)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    if (netif_running(netdev))
    ixgbe_reinit_locked(adapter);
    else
    ixgbe_reset(adapter);
    }
    static netdev_features_t ixgbe_fix_features(struct net_device *netdev,
    netdev_features_t features)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
// If Rx checksum is disabled, then RSC/LRO should also be disabled
    if (!(features & NETIF_F_RXCSUM))
    features &= ~NETIF_F_LRO;
// Turn off LRO if not RSC capable
    if (!(adapter.flags2 & IXGBE_FLAG2_RSC_CAPABLE))
    features &= ~NETIF_F_LRO;
    if (adapter.xdp_prog && (features & NETIF_F_LRO)) {
    e_dev_err("LRO is not supported with XDP\n");
    features &= ~NETIF_F_LRO;
    }
    return features;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_reset_l2fw_offload(adapter: *mut ixgbe_adapter) {
    static void ixgbe_reset_l2fw_offload(struct ixgbe_adapter *adapter)
    {
    int rss = min_t(int, ixgbe_max_rss_indices(adapter),
    num_online_cpus());
// go back to full RSS if we're not running SR-IOV
    if (!adapter.ring_feature[RING_F_VMDQ].offset)
    adapter.flags &= ~(IXGBE_FLAG_VMDQ_ENABLED |
    IXGBE_FLAG_SRIOV_ENABLED);
    adapter.ring_feature[RING_F_RSS].limit = rss;
    adapter.ring_feature[RING_F_VMDQ].limit = 1;
    ixgbe_setup_tc(adapter.netdev, adapter.hw_tcs);
    }
    static int ixgbe_set_features(struct net_device *netdev,
    netdev_features_t features)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(netdev);
    let mut changed: netdev_features_t = netdev.features ^ features;
    let mut need_reset: bool = false;
// Make sure RSC matches LRO, reset if change
    if (!(features & NETIF_F_LRO)) {
    if (adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED)
    need_reset = true;
    adapter.flags2 &= ~IXGBE_FLAG2_RSC_ENABLED;
    } else if ((adapter.flags2 & IXGBE_FLAG2_RSC_CAPABLE) &&
    !(adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED)) {
    if (adapter.rx_itr_setting == 1 ||
    adapter.rx_itr_setting > IXGBE_MIN_RSC_ITR) {
    adapter.flags2 |= IXGBE_FLAG2_RSC_ENABLED;
    need_reset = true;
    } else if ((changed ^ features) & NETIF_F_LRO) {
    e_info(probe, "rx-usecs set too low, "
    "disabling RSC\n");
    }
    }
//
// Check if Flow Director n-tuple support or hw_tc support was
// enabled or disabled.  If the state changed, we need to reset.
//
    if ((features & NETIF_F_NTUPLE) || (features & NETIF_F_HW_TC)) {
// turn off ATR, enable perfect filters and reset
    if (!(adapter.flags & IXGBE_FLAG_FDIR_PERFECT_CAPABLE))
    need_reset = true;
    adapter.flags &= ~IXGBE_FLAG_FDIR_HASH_CAPABLE;
    adapter.flags |= IXGBE_FLAG_FDIR_PERFECT_CAPABLE;
    } else {
// turn off perfect filters, enable ATR and reset
    if (adapter.flags & IXGBE_FLAG_FDIR_PERFECT_CAPABLE)
    need_reset = true;
    adapter.flags &= ~IXGBE_FLAG_FDIR_PERFECT_CAPABLE;
// We cannot enable ATR if SR-IOV is enabled
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED ||
// We cannot enable ATR if we have 2 or more tcs
    (adapter.hw_tcs > 1) ||
// We cannot enable ATR if RSS is disabled
    (adapter.ring_feature[RING_F_RSS].limit <= 1) ||
// A sample rate of 0 indicates ATR disabled
    (!adapter.atr_sample_rate))
    ; /* do nothing not supported */
    else /* otherwise supported and set the flag */
    adapter.flags |= IXGBE_FLAG_FDIR_HASH_CAPABLE;
    }
    if (changed & NETIF_F_RXALL)
    need_reset = true;
    netdev.features = features;
    if ((changed & NETIF_F_HW_L2FW_DOFFLOAD) && adapter.num_rx_pools > 1)
    ixgbe_reset_l2fw_offload(adapter);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: need_reset) -> else {
    else if (need_reset)
    ixgbe_do_reset(netdev);
    else if (changed & (NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_HW_VLAN_CTAG_FILTER))
    ixgbe_set_rx_mode(netdev);
    return 1;
    }
    static int ixgbe_ndo_fdb_add(struct ndmsg *ndm, struct nlattr *tb[],
    struct net_device *dev,
    const unsigned char *addr, u16 vid,
    u16 flags, bool *notified,
    struct netlink_ext_ack *extack)
    {
// guarantee we can provide a unique filter for the unicast address
    if (is_unicast_ether_addr(addr) || is_link_local_ether_addr(addr)) {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    let mut pool: u16 = VMDQ_P(0);
    if (netdev_uc_count(dev) >= ixgbe_available_rars(adapter, pool))
    return -ENOMEM;
    }
    return ndo_dflt_fdb_add(ndm, tb, dev, addr, vid, flags);
    }
//
// ixgbe_configure_bridge_mode - set various bridge modes
// @adapter: the private structure
// @mode: requested bridge mode
//
// Configure some settings require for various bridge modes.
//
    static int ixgbe_configure_bridge_mode(struct ixgbe_adapter *adapter,
    __u16 mode)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    unsigned int p, num_pools;
    u32 vmdctl;
    switch (mode) {
    case BRIDGE_MODE_VEPA:
// disable Tx loopback, rely on switch hairpin mode
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_PFDTXGSWC, 0);
// must enable Rx switching replication to allow multicast
// packet reception on all VFs, and to enable source address
// pruning.
//
    vmdctl = IXGBE_READ_REG(hw, IXGBE_VMD_CTL);
    vmdctl |= IXGBE_VT_CTL_REPLEN;
    IXGBE_WRITE_REG(hw, IXGBE_VMD_CTL, vmdctl);
// enable Rx source address pruning. Note, this requires
// replication to be enabled or else it does nothing.
//
    num_pools = adapter.num_vfs + adapter.num_rx_pools;
    for (p = 0; p < num_pools; p++) {
    if (hw.mac.ops.set_source_address_pruning)
    hw.mac.ops.set_source_address_pruning(hw,
    true,
    p);
    }
    break;
    case BRIDGE_MODE_VEB:
// enable Tx loopback for internal VF/PF communication
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_PFDTXGSWC,
    IXGBE_PFDTXGSWC_VT_LBEN);
// disable Rx switching replication unless we have SR-IOV
// virtual functions
//
    vmdctl = IXGBE_READ_REG(hw, IXGBE_VMD_CTL);
    if (!adapter.num_vfs)
    vmdctl &= ~IXGBE_VT_CTL_REPLEN;
    IXGBE_WRITE_REG(hw, IXGBE_VMD_CTL, vmdctl);
// disable Rx source address pruning, since we don't expect to
// be receiving external loopback of our transmitted frames.
//
    num_pools = adapter.num_vfs + adapter.num_rx_pools;
    for (p = 0; p < num_pools; p++) {
    if (hw.mac.ops.set_source_address_pruning)
    hw.mac.ops.set_source_address_pruning(hw,
    false,
    p);
    }
    break;
    default:
    return -EINVAL;
    }
    adapter.bridge_mode = mode;
    e_info(drv, "enabling bridge mode: %s\n",
    mode == BRIDGE_MODE_VEPA ? "VEPA" : "VEB");
    return 0;
    }
    static int ixgbe_ndo_bridge_setlink(struct net_device *dev,
    struct nlmsghdr *nlh, u16 flags,
    struct netlink_ext_ack *extack)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct nlattr *attr, *br_spec;
    int rem;
    if (!(adapter.flags & IXGBE_FLAG_SRIOV_ENABLED))
    return -EOPNOTSUPP;
    br_spec = nlmsg_find_attr(nlh, sizeof(struct ifinfomsg), IFLA_AF_SPEC);
    if (!br_spec)
    return -EINVAL;
    nla_for_each_nested_type(attr, IFLA_BRIDGE_MODE, br_spec, rem) {
    let mut mode: __u16 = nla_get_u16(attr);
    let mut status: c_int = ixgbe_configure_bridge_mode(adapter, mode);
    if (status)
    return status;
    break;
    }
    return 0;
    }
    static int ixgbe_ndo_bridge_getlink(struct sk_buff *skb, u32 pid, u32 seq,
    struct net_device *dev,
    u32 filter_mask, int nlflags)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    if (!(adapter.flags & IXGBE_FLAG_SRIOV_ENABLED))
    return 0;
    return ndo_dflt_bridge_getlink(skb, pid, seq, dev,
    adapter.bridge_mode, 0, 0, nlflags,
    filter_mask, core::ptr::null_mut());
    }
    static void *ixgbe_fwd_add(struct net_device *pdev, struct net_device *vdev)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(pdev);
    struct ixgbe_fwd_adapter *accel;
    let mut tcs: c_int = adapter.hw_tcs ? : 1;
    int pool, err;
    if (adapter.xdp_prog) {
    e_warn(probe, "L2FW offload is not supported with XDP\n");
    return ERR_PTR(-EINVAL);
    }
// The hardware supported by ixgbe only filters on the destination MAC
// address. In order to avoid issues we only support offloading modes
// where the hardware can actually provide the functionality.
//
    if (!macvlan_supports_dest_filter(vdev))
    return ERR_PTR(-EMEDIUMTYPE);
// We need to lock down the macvlan to be a single queue device so that
// we can reuse the tc_to_txq field in the macvlan netdev to represent
// the queue mapping to our netdev.
//
    if (netif_is_multiqueue(vdev))
    return ERR_PTR(-ERANGE);
    pool = find_first_zero_bit(adapter.fwd_bitmask, adapter.num_rx_pools);
    if (pool == adapter.num_rx_pools) {
    let mut used_pools: u16 = adapter.num_vfs + adapter.num_rx_pools;
    u16 reserved_pools;
    if (((adapter.flags & IXGBE_FLAG_DCB_ENABLED) &&
    adapter.num_rx_pools >= (MAX_TX_QUEUES / tcs)) ||
    adapter.num_rx_pools > IXGBE_MAX_MACVLANS)
    return ERR_PTR(-EBUSY);
// Hardware has a limited number of available pools. Each VF,
// and the PF require a pool. Check to ensure we don't
// attempt to use more then the available number of pools.
//
    if (used_pools >= IXGBE_MAX_VF_FUNCTIONS)
    return ERR_PTR(-EBUSY);
// Enable VMDq flag so device will be set in VM mode
    adapter.flags |= IXGBE_FLAG_VMDQ_ENABLED |
    IXGBE_FLAG_SRIOV_ENABLED;
// Try to reserve as many queues per pool as possible,
// we start with the configurations that support 4 queues
// per pools, followed by 2, and then by just 1 per pool.
//
    if (used_pools < 32 && adapter.num_rx_pools < 16)
    reserved_pools = min_t(u16,
    32 - used_pools,
    16 - adapter.num_rx_pools);
#[no_mangle]
pub unsafe extern "C" fn if(32: adapter->num_rx_pools <) -> else {
    else if (adapter.num_rx_pools < 32)
    reserved_pools = min_t(u16,
    64 - used_pools,
    32 - adapter.num_rx_pools);
    else
    reserved_pools = 64 - used_pools;
    if (!reserved_pools)
    return ERR_PTR(-EBUSY);
    adapter.ring_feature[RING_F_VMDQ].limit += reserved_pools;
// Force reinit of ring allocation with VMDQ enabled
    err = ixgbe_setup_tc(pdev, adapter.hw_tcs);
    if (err)
    return ERR_PTR(err);
    if (pool >= adapter.num_rx_pools)
    return ERR_PTR(-ENOMEM);
    }
    accel = kzalloc_obj(*accel);
    if (!accel)
    return ERR_PTR(-ENOMEM);
    set_bit(pool, adapter.fwd_bitmask);
    netdev_set_sb_channel(vdev, pool);
    accel.pool = pool;
    accel.netdev = vdev;
    if (!netif_running(pdev))
    return accel;
    err = ixgbe_fwd_ring_up(adapter, accel);
    if (err)
    return ERR_PTR(err);
    return accel;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_fwd_del(pdev: *mut net_device, priv: *mut c_void) {
    static void ixgbe_fwd_del(struct net_device *pdev, void *priv)
    {
    struct ixgbe_fwd_adapter *accel = priv;
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(pdev);
    let mut rxbase: c_uint = accel.rx_base_queue;
    unsigned int i;
// delete unicast filter associated with offloaded interface
    ixgbe_del_mac_filter(adapter, accel.netdev.dev_addr,
    VMDQ_P(accel.pool));
// Allow remaining Rx packets to get flushed out of the
// Rx FIFO before we drop the netdev for the ring.
//
    usleep_range(10000, 20000);
    for (i = 0; i < adapter.num_rx_queues_per_pool; i++) {
    struct ixgbe_ring *ring = adapter.rx_ring[rxbase + i];
    struct ixgbe_q_vector *qv = ring.q_vector;
// Make sure we aren't processing any packets and clear
// netdev to shut down the ring.
//
    if (netif_running(adapter.netdev))
    napi_synchronize(&qv.napi);
    ring.netdev = core::ptr::null_mut();
    }
// unbind the queues and drop the subordinate channel config
    netdev_unbind_sb_channel(pdev, accel.netdev);
    netdev_set_sb_channel(accel.netdev, 0);
    clear_bit(accel.pool, adapter.fwd_bitmask);
    kfree(accel);
    }
pub const IXGBE_MAX_MAC_HDR_LEN: c_int = 127;
pub const IXGBE_MAX_NETWORK_HDR_LEN: c_int = 511;
    static netdev_features_t
    ixgbe_features_check(struct sk_buff *skb, struct net_device *dev,
    netdev_features_t features)
    {
    unsigned int network_hdr_len, mac_hdr_len;
// Make certain the headers can be described by a context descriptor
    mac_hdr_len = skb_network_offset(skb);
    if (unlikely(mac_hdr_len > IXGBE_MAX_MAC_HDR_LEN))
    return features & ~(NETIF_F_HW_CSUM |
    NETIF_F_SCTP_CRC |
    NETIF_F_GSO_UDP_L4 |
    NETIF_F_HW_VLAN_CTAG_TX |
    NETIF_F_TSO |
    NETIF_F_TSO6);
    network_hdr_len = skb_checksum_start(skb) - skb_network_header(skb);
    if (unlikely(network_hdr_len >  IXGBE_MAX_NETWORK_HDR_LEN))
    return features & ~(NETIF_F_HW_CSUM |
    NETIF_F_SCTP_CRC |
    NETIF_F_GSO_UDP_L4 |
    NETIF_F_TSO |
    NETIF_F_TSO6);
// We can only support IPV4 TSO in tunnels if we can mangle the
// inner IP ID field, so strip TSO if MANGLEID is not supported.
// IPsec offoad sets skb->encapsulation but still can handle
// the TSO, so it's the exception.
//
    if (skb.encapsulation && !(features & NETIF_F_TSO_MANGLEID)) {

    if (!secpath_exists(skb))

    features &= ~NETIF_F_TSO;
    }
    return features;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_xdp_setup(dev: *mut net_device, prog: *mut bpf_prog) -> c_int {
    static int ixgbe_xdp_setup(struct net_device *dev, struct bpf_prog *prog)
    {
    int i, frame_size = dev.mtu + ETH_HLEN + ETH_FCS_LEN + VLAN_HLEN;
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct bpf_prog *old_prog;
    bool need_reset;
    int num_queues;
    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED)
    return -EINVAL;
    if (adapter.flags & IXGBE_FLAG_DCB_ENABLED)
    return -EINVAL;
// verify ixgbe ring attributes are sufficient for XDP
    for (i = 0; i < adapter.num_rx_queues; i++) {
    struct ixgbe_ring *ring = adapter.rx_ring[i];
    if (ring_is_rsc_enabled(ring))
    return -EINVAL;
    if (frame_size > ixgbe_rx_bufsz(ring))
    return -EINVAL;
    }
// if the number of cpus is much larger than the maximum of queues,
// we should stop it and then return with ENOMEM like before.
//
    if (nr_cpu_ids > IXGBE_MAX_XDP_QS * 2)
    return -ENOMEM;
    old_prog = xchg(&adapter.xdp_prog, prog);
    need_reset = (!!prog != !!old_prog);
// If transitioning XDP modes reconfigure rings
    if (need_reset) {
    int err;
    if (!prog)
// Wait until ndo_xsk_wakeup completes.
    synchronize_rcu();
    err = ixgbe_setup_tc(dev, adapter.hw_tcs);
    if (err)
    return -EINVAL;
    if (!prog)
    xdp_features_clear_redirect_target(dev);
    } else {
    for (i = 0; i < adapter.num_rx_queues; i++) {
    WRITE_ONCE(adapter.rx_ring[i].xdp_prog,
    adapter.xdp_prog);
    }
    }
    if (old_prog)
    bpf_prog_put(old_prog);
// Kick start the NAPI context if there is an AF_XDP socket open
// on that queue id. This so that receiving will start.
//
    if (need_reset && prog) {
    num_queues = min_t(int, adapter.num_rx_queues,
    adapter.num_xdp_queues);
    for (i = 0; i < num_queues; i++)
    if (adapter.xdp_ring[i].xsk_pool)
    (void)ixgbe_xsk_wakeup(adapter.netdev, i,
    XDP_WAKEUP_RX);
    xdp_features_set_redirect_target(dev, true);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_xdp(dev: *mut net_device, xdp: *mut netdev_bpf) -> c_int {
    static int ixgbe_xdp(struct net_device *dev, struct netdev_bpf *xdp)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    switch (xdp.command) {
    case XDP_SETUP_PROG:
    return ixgbe_xdp_setup(dev, xdp.prog);
    case XDP_SETUP_XSK_POOL:
    return ixgbe_xsk_pool_setup(adapter, xdp.xsk.pool,
    xdp.xsk.queue_id);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_xdp_ring_update_tail(ring: *mut ixgbe_ring) {
    void ixgbe_xdp_ring_update_tail(struct ixgbe_ring *ring)
    {
// Force memory writes to complete before letting h/w know there
// are new descriptors to fetch.
//
    wmb();
    writel(ring.next_to_use, ring.tail);
    }
#[no_mangle]
pub unsafe extern "C" fn ixgbe_xdp_ring_update_tail_locked(ring: *mut ixgbe_ring) {
    void ixgbe_xdp_ring_update_tail_locked(struct ixgbe_ring *ring)
    {
    if (static_branch_unlikely(&ixgbe_xdp_locking_key))
    spin_lock(&ring.tx_lock);
    ixgbe_xdp_ring_update_tail(ring);
    if (static_branch_unlikely(&ixgbe_xdp_locking_key))
    spin_unlock(&ring.tx_lock);
    }
    static int ixgbe_xdp_xmit(struct net_device *dev, int n,
    struct xdp_frame **frames, u32 flags)
    {
    struct ixgbe_adapter *adapter = ixgbe_from_netdev(dev);
    struct ixgbe_ring *ring;
    let mut nxmit: c_int = 0;
    int i;
    if (unlikely(test_bit(__IXGBE_DOWN, &adapter.state)))
    return -ENETDOWN;
    if (!netif_carrier_ok(adapter.netdev) ||
    !netif_running(adapter.netdev))
    return -ENETDOWN;
    if (unlikely(flags & ~XDP_XMIT_FLAGS_MASK))
    return -EINVAL;
// During program transitions its possible adapter->xdp_prog is assigned
// but ring has not been configured yet. In this case simply abort xmit.
//
    ring = adapter.xdp_prog ? ixgbe_determine_xdp_ring(adapter) : core::ptr::null_mut();
    if (unlikely(!ring))
    return -ENXIO;
    if (unlikely(test_bit(__IXGBE_TX_DISABLED, ring.state)))
    return -ENXIO;
    if (static_branch_unlikely(&ixgbe_xdp_locking_key))
    spin_lock(&ring.tx_lock);
    for (i = 0; i < n; i++) {
    struct xdp_frame *xdpf = frames[i];
    int err;
    err = ixgbe_xmit_xdp_ring(ring, xdpf);
    if (err != IXGBE_XDP_TX)
    break;
    nxmit++;
    }
    if (unlikely(flags & XDP_XMIT_FLUSH))
    ixgbe_xdp_ring_update_tail(ring);
    if (static_branch_unlikely(&ixgbe_xdp_locking_key))
    spin_unlock(&ring.tx_lock);
    return nxmit;
    }
    static const struct net_device_ops ixgbe_netdev_ops = {
    .ndo_open		= ixgbe_open,
    .ndo_stop		= ixgbe_close,
    .ndo_start_xmit		= ixgbe_xmit_frame,
    .ndo_set_rx_mode	= ixgbe_set_rx_mode,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= ixgbe_set_mac,
    .ndo_change_mtu		= ixgbe_change_mtu,
    .ndo_tx_timeout		= ixgbe_tx_timeout,
    .ndo_set_tx_maxrate	= ixgbe_tx_maxrate,
    .ndo_vlan_rx_add_vid	= ixgbe_vlan_rx_add_vid,
    .ndo_vlan_rx_kill_vid	= ixgbe_vlan_rx_kill_vid,
    .ndo_eth_ioctl		= ixgbe_ioctl,
    .ndo_set_vf_mac		= ixgbe_ndo_set_vf_mac,
    .ndo_set_vf_vlan	= ixgbe_ndo_set_vf_vlan,
    .ndo_set_vf_rate	= ixgbe_ndo_set_vf_bw,
    .ndo_set_vf_spoofchk	= ixgbe_ndo_set_vf_spoofchk,
    .ndo_set_vf_link_state	= ixgbe_ndo_set_vf_link_state,
    .ndo_set_vf_rss_query_en = ixgbe_ndo_set_vf_rss_query_en,
    .ndo_set_vf_trust	= ixgbe_ndo_set_vf_trust,
    .ndo_get_vf_config	= ixgbe_ndo_get_vf_config,
    .ndo_get_vf_stats	= ixgbe_ndo_get_vf_stats,
    .ndo_get_stats64	= ixgbe_get_stats64,
    .ndo_setup_tc		= __ixgbe_setup_tc,

    .ndo_select_queue	= ixgbe_select_queue,
    .ndo_fcoe_ddp_setup = ixgbe_fcoe_ddp_get,
    .ndo_fcoe_ddp_target = ixgbe_fcoe_ddp_target,
    .ndo_fcoe_ddp_done = ixgbe_fcoe_ddp_put,
    .ndo_fcoe_enable = ixgbe_fcoe_enable,
    .ndo_fcoe_disable = ixgbe_fcoe_disable,
    .ndo_fcoe_get_wwn = ixgbe_fcoe_get_wwn,
    .ndo_fcoe_get_hbainfo = ixgbe_fcoe_get_hbainfo,

    .ndo_set_features = ixgbe_set_features,
    .ndo_fix_features = ixgbe_fix_features,
    .ndo_fdb_add		= ixgbe_ndo_fdb_add,
    .ndo_bridge_setlink	= ixgbe_ndo_bridge_setlink,
    .ndo_bridge_getlink	= ixgbe_ndo_bridge_getlink,
    .ndo_dfwd_add_station	= ixgbe_fwd_add,
    .ndo_dfwd_del_station	= ixgbe_fwd_del,
    .ndo_features_check	= ixgbe_features_check,
    .ndo_bpf		= ixgbe_xdp,
    .ndo_xdp_xmit		= ixgbe_xdp_xmit,
    .ndo_xsk_wakeup         = ixgbe_xsk_wakeup,
    .ndo_hwtstamp_get	= ixgbe_ptp_hwtstamp_get,
    .ndo_hwtstamp_set	= ixgbe_ptp_hwtstamp_set,
    };
    static void ixgbe_disable_txr_hw(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *tx_ring)
    {
    unsigned long wait_delay, delay_interval;
    struct ixgbe_hw *hw = &adapter.hw;
    let mut reg_idx: u8 = tx_ring.reg_idx;
    int wait_loop;
    u32 txdctl;
    IXGBE_WRITE_REG(hw, IXGBE_TXDCTL(reg_idx), IXGBE_TXDCTL_SWFLSH);
// delay mechanism from ixgbe_disable_tx
    delay_interval = ixgbe_get_completion_timeout(adapter) / 100;
    wait_loop = IXGBE_MAX_RX_DESC_POLL;
    wait_delay = delay_interval;
    while (wait_loop--) {
    usleep_range(wait_delay, wait_delay + 10);
    wait_delay += delay_interval * 2;
    txdctl = IXGBE_READ_REG(hw, IXGBE_TXDCTL(reg_idx));
    if (!(txdctl & IXGBE_TXDCTL_ENABLE))
    return;
    }
    e_err(drv, "TXDCTL.ENABLE not cleared within the polling period\n");
    }
    static void ixgbe_disable_txr(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *tx_ring)
    {
    set_bit(__IXGBE_TX_DISABLED, tx_ring.state);
    ixgbe_disable_txr_hw(adapter, tx_ring);
    }
    static void ixgbe_disable_rxr_hw(struct ixgbe_adapter *adapter,
    struct ixgbe_ring *rx_ring)
    {
    unsigned long wait_delay, delay_interval;
    struct ixgbe_hw *hw = &adapter.hw;
    let mut reg_idx: u8 = rx_ring.reg_idx;
    int wait_loop;
    u32 rxdctl;
    rxdctl = IXGBE_READ_REG(hw, IXGBE_RXDCTL(reg_idx));
    rxdctl &= ~IXGBE_RXDCTL_ENABLE;
    rxdctl |= IXGBE_RXDCTL_SWFLSH;
// write value back with RXDCTL.ENABLE bit cleared
    IXGBE_WRITE_REG(hw, IXGBE_RXDCTL(reg_idx), rxdctl);
// RXDCTL.EN may not change on 82598 if link is down, so skip it
    if (hw.mac.type == ixgbe_mac_82598EB &&
    !(IXGBE_READ_REG(hw, IXGBE_LINKS) & IXGBE_LINKS_UP))
    return;
// delay mechanism from ixgbe_disable_rx
    delay_interval = ixgbe_get_completion_timeout(adapter) / 100;
    wait_loop = IXGBE_MAX_RX_DESC_POLL;
    wait_delay = delay_interval;
    while (wait_loop--) {
    usleep_range(wait_delay, wait_delay + 10);
    wait_delay += delay_interval * 2;
    rxdctl = IXGBE_READ_REG(hw, IXGBE_RXDCTL(reg_idx));
    if (!(rxdctl & IXGBE_RXDCTL_ENABLE))
    return;
    }
    e_err(drv, "RXDCTL.ENABLE not cleared within the polling period\n");
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_reset_txr_stats(tx_ring: *mut ixgbe_ring) {
    static void ixgbe_reset_txr_stats(struct ixgbe_ring *tx_ring)
    {
    memset(&tx_ring.stats, 0, sizeof(tx_ring.stats));
    memset(&tx_ring.tx_stats, 0, sizeof(tx_ring.tx_stats));
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_reset_rxr_stats(rx_ring: *mut ixgbe_ring) {
    static void ixgbe_reset_rxr_stats(struct ixgbe_ring *rx_ring)
    {
    memset(&rx_ring.stats, 0, sizeof(rx_ring.stats));
    memset(&rx_ring.rx_stats, 0, sizeof(rx_ring.rx_stats));
    }
//
// ixgbe_irq_disable_single - Disable single IRQ vector
// @adapter: adapter structure
// @ring: ring index
//
#[no_mangle]
unsafe extern "C" fn ixgbe_irq_disable_single(adapter: *mut ixgbe_adapter, ring: u32) {
    static void ixgbe_irq_disable_single(struct ixgbe_adapter *adapter, u32 ring)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut qmask: u64 = BIT_ULL(ring);
    u32 mask;
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82598EB:
    mask = qmask & IXGBE_EIMC_RTX_QUEUE;
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_EIMC, mask);
    break;
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    mask = (qmask & 0xFFFFFFFF);
    if (mask)
    IXGBE_WRITE_REG(hw, IXGBE_EIMS_EX(0), mask);
    mask = (qmask >> 32);
    if (mask)
    IXGBE_WRITE_REG(hw, IXGBE_EIMS_EX(1), mask);
    break;
    default:
    break;
    }
    IXGBE_WRITE_FLUSH(&adapter.hw);
    if (adapter.flags & IXGBE_FLAG_MSIX_ENABLED)
    synchronize_irq(adapter.msix_entries[ring].vector);
    else
    synchronize_irq(adapter.pdev.irq);
    }
//
// ixgbe_txrx_ring_disable - Disable Rx/Tx/XDP Tx rings
// @adapter: adapter structure
// @ring: ring index
//
// This function disables a certain Rx/Tx/XDP Tx ring. The function
// assumes that the netdev is running.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_txrx_ring_disable(adapter: *mut ixgbe_adapter, ring: c_int) {
    void ixgbe_txrx_ring_disable(struct ixgbe_adapter *adapter, int ring)
    {
    struct ixgbe_ring *rx_ring, *tx_ring, *xdp_ring;
    rx_ring = adapter.rx_ring[ring];
    tx_ring = adapter.tx_ring[ring];
    xdp_ring = adapter.xdp_ring[ring];
    ixgbe_irq_disable_single(adapter, ring);
// Rx/Tx/XDP Tx share the same napi context.
    napi_disable(&rx_ring.q_vector.napi);
    ixgbe_disable_txr(adapter, tx_ring);
    if (xdp_ring)
    ixgbe_disable_txr(adapter, xdp_ring);
    ixgbe_disable_rxr_hw(adapter, rx_ring);
    if (xdp_ring)
    synchronize_rcu();
    ixgbe_clean_tx_ring(tx_ring);
    if (xdp_ring)
    ixgbe_clean_tx_ring(xdp_ring);
    ixgbe_clean_rx_ring(rx_ring);
    ixgbe_reset_txr_stats(tx_ring);
    if (xdp_ring)
    ixgbe_reset_txr_stats(xdp_ring);
    ixgbe_reset_rxr_stats(rx_ring);
    }
//
// ixgbe_txrx_ring_enable - Enable Rx/Tx/XDP Tx rings
// @adapter: adapter structure
// @ring: ring index
//
// This function enables a certain Rx/Tx/XDP Tx ring. The function
// assumes that the netdev is running.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_txrx_ring_enable(adapter: *mut ixgbe_adapter, ring: c_int) {
    void ixgbe_txrx_ring_enable(struct ixgbe_adapter *adapter, int ring)
    {
    struct ixgbe_ring *rx_ring, *tx_ring, *xdp_ring;
    rx_ring = adapter.rx_ring[ring];
    tx_ring = adapter.tx_ring[ring];
    xdp_ring = adapter.xdp_ring[ring];
    ixgbe_configure_tx_ring(adapter, tx_ring);
    if (xdp_ring)
    ixgbe_configure_tx_ring(adapter, xdp_ring);
    ixgbe_configure_rx_ring(adapter, rx_ring);
    clear_bit(__IXGBE_TX_DISABLED, tx_ring.state);
    if (xdp_ring)
    clear_bit(__IXGBE_TX_DISABLED, xdp_ring.state);
// Rx/Tx/XDP Tx share the same napi context.
    napi_enable(&rx_ring.q_vector.napi);
    ixgbe_irq_enable_queues(adapter, BIT_ULL(ring));
    IXGBE_WRITE_FLUSH(&adapter.hw);
    }
//
// ixgbe_enumerate_functions - Get the number of ports this device has
// @adapter: adapter structure
//
// This function enumerates the physical functions co-located on a single slot,
// in order to determine how many ports a device has. This is most useful in
// determining the required GT/s of PCIe bandwidth necessary for optimal
// performance.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_enumerate_functions(adapter: *mut ixgbe_adapter) -> c_int {
    static inline int ixgbe_enumerate_functions(struct ixgbe_adapter *adapter)
    {
    struct pci_dev *entry, *pdev = adapter.pdev;
    let mut physfns: c_int = 0;
// Some cards can not use the generic count PCIe functions method,
// because they are behind a parent switch, so we hardcode these with
// the correct number of functions.
//
    if (ixgbe_pcie_from_parent(&adapter.hw))
    physfns = 4;
    list_for_each_entry(entry, &adapter.pdev.bus.devices, bus_list) {
// don't count virtual functions
    if (entry.is_virtfn)
    continue;
// When the devices on the bus don't all match our device ID,
// we can't reliably determine the correct number of
// functions. This can occur if a function has been direct
// attached to a virtual machine using VT-d, for example. In
// this case, simply return -1 to indicate this.
//
    if ((entry.vendor != pdev.vendor) ||
    (entry.device != pdev.device))
    return -1;
    physfns++;
    }
    return physfns;
    }
//
// ixgbe_wol_supported - Check whether device supports WoL
// @adapter: the adapter private structure
// @device_id: the device ID
// @subdevice_id: the subsystem device ID
//
// This function is used by probe and ethtool to determine
// which devices have WoL support
//
    bool ixgbe_wol_supported(struct ixgbe_adapter *adapter, u16 device_id,
    u16 subdevice_id)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    let mut wol_cap: u16 = adapter.eeprom_cap & IXGBE_DEVICE_CAPS_WOL_MASK;
// WOL not supported on 82598
    if (hw.mac.type == ixgbe_mac_82598EB)
    return false;
// check eeprom to see if WOL is enabled for X540 and newer
    if (hw.mac.type >= ixgbe_mac_X540) {
    if ((wol_cap == IXGBE_DEVICE_CAPS_WOL_PORT0_1) ||
    ((wol_cap == IXGBE_DEVICE_CAPS_WOL_PORT0) &&
    (hw.bus.func == 0)))
    return true;
    }
// WOL is determined based on device IDs for 82599 MACs
    switch (device_id) {
    case IXGBE_DEV_ID_82599_SFP:
// Only these subdevices could supports WOL
    switch (subdevice_id) {
    case IXGBE_SUBDEV_ID_82599_560FLR:
    case IXGBE_SUBDEV_ID_82599_LOM_SNAP6:
    case IXGBE_SUBDEV_ID_82599_SFP_WOL0:
    case IXGBE_SUBDEV_ID_82599_SFP_2OCP:
// only support first port
    if (hw.bus.func != 0)
    break;
    fallthrough;
    case IXGBE_SUBDEV_ID_82599_SP_560FLR:
    case IXGBE_SUBDEV_ID_82599_SFP:
    case IXGBE_SUBDEV_ID_82599_RNDC:
    case IXGBE_SUBDEV_ID_82599_ECNA_DP:
    case IXGBE_SUBDEV_ID_82599_SFP_1OCP:
    case IXGBE_SUBDEV_ID_82599_SFP_LOM_OEM1:
    case IXGBE_SUBDEV_ID_82599_SFP_LOM_OEM2:
    return true;
    }
    break;
    case IXGBE_DEV_ID_82599EN_SFP:
// Only these subdevices support WOL
    switch (subdevice_id) {
    case IXGBE_SUBDEV_ID_82599EN_SFP_OCP1:
    return true;
    }
    break;
    case IXGBE_DEV_ID_82599_COMBO_BACKPLANE:
// All except this subdevice support WOL
    if (subdevice_id != IXGBE_SUBDEV_ID_82599_KX4_KR_MEZZ)
    return true;
    break;
    case IXGBE_DEV_ID_82599_KX4:
    return  true;
    default:
    break;
    }
    return false;
    }
//
// ixgbe_set_fw_version_e610 - Set FW version specifically on E610 adapters
// @adapter: the adapter private structure
//
// This function is used by probe and ethtool to determine the FW version to
// format to display. The FW version is taken from the EEPROM/NVM.
//
#[no_mangle]
pub unsafe extern "C" fn ixgbe_set_fw_version_e610(adapter: *mut ixgbe_adapter) {
    void ixgbe_set_fw_version_e610(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_orom_info *orom = &adapter.hw.flash.orom;
    struct ixgbe_nvm_info *nvm = &adapter.hw.flash.nvm;
    snprintf(adapter.eeprom_id, sizeof(adapter.eeprom_id),
    "%x.%02x 0x%x %d.%d.%d", nvm.major, nvm.minor,
    nvm.eetrack, orom.major, orom.build, orom.patch);
    }
//
// ixgbe_set_fw_version - Set FW version
// @adapter: the adapter private structure
//
// This function is used by probe and ethtool to determine the FW version to
// format to display. The FW version is taken from the EEPROM/NVM.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_set_fw_version(adapter: *mut ixgbe_adapter) {
    static void ixgbe_set_fw_version(struct ixgbe_adapter *adapter)
    {
    struct ixgbe_hw *hw = &adapter.hw;
    struct ixgbe_nvm_version nvm_ver;
    if (adapter.hw.mac.type == ixgbe_mac_e610) {
    ixgbe_set_fw_version_e610(adapter);
    return;
    }
    ixgbe_get_oem_prod_version(hw, &nvm_ver);
    if (nvm_ver.oem_valid) {
    snprintf(adapter.eeprom_id, sizeof(adapter.eeprom_id),
    "%x.%x.%x", nvm_ver.oem_major, nvm_ver.oem_minor,
    nvm_ver.oem_release);
    return;
    }
    ixgbe_get_etk_id(hw, &nvm_ver);
    ixgbe_get_orom_version(hw, &nvm_ver);
    if (nvm_ver.or_valid) {
    snprintf(adapter.eeprom_id, sizeof(adapter.eeprom_id),
    "0x%08x, %d.%d.%d", nvm_ver.etk_id, nvm_ver.or_major,
    nvm_ver.or_build, nvm_ver.or_patch);
    return;
    }
// Set ETrack ID format
    snprintf(adapter.eeprom_id, sizeof(adapter.eeprom_id),
    "0x%08x", nvm_ver.etk_id);
    }
//
// ixgbe_recovery_probe - Handle FW recovery mode during probe
// @adapter: the adapter private structure
//
// Perform limited driver initialization when FW error is detected.
//
// Return: 0 on successful probe for E610, -EIO if recovery mode is detected
// for non-E610 adapter, error status code on any other case.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_recovery_probe(adapter: *mut ixgbe_adapter) -> c_int {
    static int ixgbe_recovery_probe(struct ixgbe_adapter *adapter)
    {
    struct pci_dev *pdev = adapter.pdev;
    struct ixgbe_hw *hw = &adapter.hw;
    let mut err: c_int = -EIO;
    if (hw.mac.type != ixgbe_mac_e610)
    return err;
    ixgbe_get_hw_control(adapter);
    err = ixgbe_get_flash_data(&adapter.hw);
    if (err)
    goto err_release_hw_control;
    timer_setup(&adapter.service_timer, ixgbe_service_timer, 0);
    INIT_WORK(&adapter.service_task, ixgbe_recovery_service_task);
    set_bit(__IXGBE_SERVICE_INITED, &adapter.state);
    clear_bit(__IXGBE_SERVICE_SCHED, &adapter.state);
    if (hw.mac.ops.get_bus_info)
    hw.mac.ops.get_bus_info(hw);
    pci_set_drvdata(pdev, adapter);
// We are creating devlink interface so NIC can be managed,
// e.g. new NVM image loaded
//
    devl_lock(adapter.devlink);
    ixgbe_devlink_register_port(adapter);
    SET_NETDEV_DEVLINK_PORT(adapter.netdev,
    &adapter.devlink_port);
    ixgbe_devlink_init_regions(adapter);
    devl_register(adapter.devlink);
    devl_unlock(adapter.devlink);
    return 0;
    err_release_hw_control:
    ixgbe_release_hw_control(adapter);
    return err;
    }
//
// ixgbe_probe - Device Initialization Routine
// @pdev: PCI device information struct
// @ent: entry in ixgbe_pci_tbl
//
// Returns 0 on success, negative on failure
//
// ixgbe_probe initializes an adapter identified by a pci_dev structure.
// The OS initialization, configuring of the adapter private structure,
// and a hardware reset occur.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int ixgbe_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct net_device *netdev;
    struct ixgbe_netdevice_priv *netdev_priv_wrapper;
    struct ixgbe_adapter *adapter = core::ptr::null_mut();
    struct ixgbe_hw *hw;
    const struct ixgbe_info *ii = ixgbe_info_tbl[ent.driver_data];
    let mut indices: c_uint = MAX_TX_QUEUES;
    u8 part_str[IXGBE_PBANUM_LENGTH];
    int i, err, expected_gts;
    let mut disable_dev: bool = false;

    u16 device_caps;

    u32 eec;
// Catch broken hardware that put the wrong VF device ID in
// the PCIe SR-IOV capability.
//
    if (pdev.is_virtfn) {
    WARN(1, KERN_ERR "%s (%hx:%hx) should not be a VF!\n",
    pci_name(pdev), pdev.vendor, pdev.device);
    return -EINVAL;
    }
    err = pci_enable_device_mem(pdev);
    if (err)
    return err;
    err = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (err) {
    dev_err(&pdev.dev,
    "No usable DMA configuration, aborting\n");
    goto err_dma;
    }
    err = pci_request_mem_regions(pdev, ixgbe_driver_name);
    if (err) {
    dev_err(&pdev.dev,
    "pci_request_selected_regions failed 0x%x\n", err);
    goto err_pci_reg;
    }
    pci_set_master(pdev);
    pci_save_state(pdev);
    if (ii.mac == ixgbe_mac_82598EB) {

// 8 TC w/ 4 queues per TC
    indices = 4 * MAX_TRAFFIC_CLASS;

    indices = IXGBE_MAX_RSS_INDICES;

    } else if (ii.mac == ixgbe_mac_e610) {
    indices = IXGBE_MAX_RSS_INDICES_X550;
    }
    adapter = ixgbe_allocate_devlink(&pdev.dev);
    if (IS_ERR(adapter)) {
    err = PTR_ERR(adapter);
    goto err_devlink;
    }
    netdev = alloc_etherdev_mq(sizeof(*netdev_priv_wrapper), indices);
    if (!netdev) {
    err = -ENOMEM;
    goto err_alloc_etherdev;
    }
    SET_NETDEV_DEV(netdev, &pdev.dev);
    netdev_priv_wrapper = netdev_priv(netdev);
    netdev_priv_wrapper.adapter = adapter;
    adapter.netdev = netdev;
    adapter.pdev = pdev;
    hw = &adapter.hw;
    hw.back = adapter;
    adapter.msg_enable = netif_msg_init(debug, DEFAULT_MSG_ENABLE);
    hw.hw_addr = ioremap(pci_resource_start(pdev, 0),
    pci_resource_len(pdev, 0));
    adapter.io_addr = hw.hw_addr;
    if (!hw.hw_addr) {
    err = -EIO;
    goto err_ioremap;
    }
// Setup hw api
    hw.mac.ops   = *ii.mac_ops;
    hw.mac.type  = ii.mac;
    hw.mvals     = ii.mvals;
    if (ii.link_ops)
    hw.link.ops  = *ii.link_ops;
// EEPROM
    hw.eeprom.ops = *ii.eeprom_ops;
    eec = IXGBE_READ_REG(hw, IXGBE_EEC(hw));
    if (ixgbe_removed(hw.hw_addr)) {
    err = -EIO;
    goto err_ioremap;
    }
// If EEPROM is valid (bit 8 = 1), use default otherwise use bit bang
    if (!(eec & BIT(8)))
    hw.eeprom.ops.read = &ixgbe_read_eeprom_bit_bang_generic;
// PHY
    hw.phy.ops = *ii.phy_ops;
    hw.phy.sfp_type = ixgbe_sfp_type_unknown;
// ixgbe_identify_phy_generic will set prtad and mmds properly
    hw.phy.mdio.prtad = MDIO_PRTAD_NONE;
    hw.phy.mdio.mmds = 0;
    hw.phy.mdio.mode_support = MDIO_SUPPORTS_C45 | MDIO_EMULATE_C22;
    hw.phy.mdio.dev = netdev;
    hw.phy.mdio.mdio_read = ixgbe_mdio_read;
    hw.phy.mdio.mdio_write = ixgbe_mdio_write;
    netdev.netdev_ops = &ixgbe_netdev_ops;
    ixgbe_set_ethtool_ops(netdev);
    netdev.watchdog_timeo = 5 * HZ;
    strscpy(netdev.name, pci_name(pdev), sizeof(netdev.name));
// setup the private structure
    err = ixgbe_sw_init(adapter, ii);
    if (err)
    goto err_sw_init;
    if (ixgbe_check_fw_error(adapter)) {
    err = ixgbe_recovery_probe(adapter);
    if (err)
    goto err_sw_init;
    return 0;
    }
    if (adapter.hw.mac.type == ixgbe_mac_e610) {
    err = ixgbe_get_caps(&adapter.hw);
    if (err)
    dev_err(&pdev.dev, "ixgbe_get_caps failed %d\n", err);
    err = ixgbe_get_flash_data(&adapter.hw);
    if (err)
    goto err_sw_init;
    }
    if (adapter.hw.mac.type == ixgbe_mac_82599EB)
    adapter.flags2 |= IXGBE_FLAG2_AUTO_DISABLE_VF;
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_e610:
    netdev.udp_tunnel_nic_info = &ixgbe_udp_tunnels_x550;
    break;
    case ixgbe_mac_x550em_a:
    netdev.udp_tunnel_nic_info = &ixgbe_udp_tunnels_x550em_a;
    break;
    default:
    break;
    }
// Make it possible the adapter to be woken up via WOL
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82599EB:
    case ixgbe_mac_X540:
    case ixgbe_mac_X550:
    case ixgbe_mac_X550EM_x:
    case ixgbe_mac_x550em_a:
    case ixgbe_mac_e610:
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_WUS, ~0);
    break;
    default:
    break;
    }
//
// If there is a fan on this device and it has failed log the
// failure.
//
    if (adapter.flags & IXGBE_FLAG_FAN_FAIL_CAPABLE) {
    let mut esdp: u32 = IXGBE_READ_REG(hw, IXGBE_ESDP);
    if (esdp & IXGBE_ESDP_SDP1)
    e_crit(probe, "Fan has stopped, replace the adapter\n");
    }
    if (allow_unsupported_sfp)
    hw.allow_unsupported_sfp = allow_unsupported_sfp;
// reset_hw fills in the perm_addr as well
    hw.phy.reset_if_overtemp = true;
    err = hw.mac.ops.reset_hw(hw);
    hw.phy.reset_if_overtemp = false;
    ixgbe_set_eee_capable(adapter);
    if (err == -ENOENT) {
    err = 0;
    } else if (err == -EOPNOTSUPP) {
    e_dev_err("failed to load because an unsupported SFP+ or QSFP module type was detected.\n");
    e_dev_err("Reload the driver after installing a supported module.\n");
    goto err_sw_init;
    } else if (err) {
    e_dev_err("HW Init failed: %d\n", err);
    goto err_sw_init;
    }

// SR-IOV not supported on the 82598
    if (adapter.hw.mac.type == ixgbe_mac_82598EB)
    goto skip_sriov;
// Mailbox
    ixgbe_init_mbx_params_pf(hw);
    hw.mbx.ops = ii.mbx_ops;
    pci_sriov_set_totalvfs(pdev, IXGBE_MAX_VFS_DRV_LIMIT);
    ixgbe_enable_sriov(adapter, max_vfs);
    skip_sriov:

    netdev.features = NETIF_F_SG |
    NETIF_F_TSO |
    NETIF_F_TSO6 |
    NETIF_F_RXHASH |
    NETIF_F_RXCSUM |
    NETIF_F_HW_CSUM;

    NETIF_F_GSO_GRE_CSUM | \
    NETIF_F_GSO_IPXIP4 | \
    NETIF_F_GSO_IPXIP6 | \
    NETIF_F_GSO_UDP_TUNNEL | \
    NETIF_F_GSO_UDP_TUNNEL_CSUM)
    netdev.gso_partial_features = IXGBE_GSO_PARTIAL_FEATURES;
    netdev.features |= NETIF_F_GSO_PARTIAL |
    IXGBE_GSO_PARTIAL_FEATURES;
    if (hw.mac.type >= ixgbe_mac_82599EB)
    netdev.features |= NETIF_F_SCTP_CRC | NETIF_F_GSO_UDP_L4;

    NETIF_F_HW_ESP_TX_CSUM | \
    NETIF_F_GSO_ESP)
    if (adapter.ipsec)
    netdev.features |= IXGBE_ESP_FEATURES;

// copy netdev features into list of user selectable features
    netdev.hw_features |= netdev.features |
    NETIF_F_HW_VLAN_CTAG_FILTER |
    NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_HW_VLAN_CTAG_TX |
    NETIF_F_RXALL |
    NETIF_F_HW_L2FW_DOFFLOAD;
    if (hw.mac.type >= ixgbe_mac_82599EB)
    netdev.hw_features |= NETIF_F_NTUPLE |
    NETIF_F_HW_TC;
    netdev.features |= NETIF_F_HIGHDMA;
    netdev.vlan_features |= netdev.features | NETIF_F_TSO_MANGLEID;
    netdev.hw_enc_features |= netdev.vlan_features;
    netdev.mpls_features |= NETIF_F_SG |
    NETIF_F_TSO |
    NETIF_F_TSO6 |
    NETIF_F_HW_CSUM;
    netdev.mpls_features |= IXGBE_GSO_PARTIAL_FEATURES;
// set this bit last since it cannot be part of vlan_features
    netdev.features |= NETIF_F_HW_VLAN_CTAG_FILTER |
    NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_HW_VLAN_CTAG_TX;
    netdev.priv_flags |= IFF_UNICAST_FLT;
    netdev.priv_flags |= IFF_SUPP_NOFCS;
    netdev.xdp_features = NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT |
    NETDEV_XDP_ACT_XSK_ZEROCOPY;
// MTU range: 68 - 9710
    netdev.min_mtu = ETH_MIN_MTU;
    netdev.max_mtu = IXGBE_MAX_JUMBO_FRAME_SIZE - (ETH_HLEN + ETH_FCS_LEN);

    if (adapter.flags & IXGBE_FLAG_DCB_CAPABLE)
    netdev.dcbnl_ops = &ixgbe_dcbnl_ops;

    if (adapter.flags & IXGBE_FLAG_FCOE_CAPABLE) {
    unsigned int fcoe_l;
    if (hw.mac.ops.get_device_caps) {
    hw.mac.ops.get_device_caps(hw, &device_caps);
    if (device_caps & IXGBE_DEVICE_CAPS_FCOE_OFFLOADS)
    adapter.flags &= ~IXGBE_FLAG_FCOE_CAPABLE;
    }
    fcoe_l = min_t(int, IXGBE_FCRETA_SIZE, num_online_cpus());
    adapter.ring_feature[RING_F_FCOE].limit = fcoe_l;
    netdev.features |= NETIF_F_FSO |
    NETIF_F_FCOE_CRC;
    netdev.vlan_features |= NETIF_F_FSO |
    NETIF_F_FCOE_CRC;
    }

    if (adapter.flags2 & IXGBE_FLAG2_RSC_CAPABLE)
    netdev.hw_features |= NETIF_F_LRO;
    if (adapter.flags2 & IXGBE_FLAG2_RSC_ENABLED)
    netdev.features |= NETIF_F_LRO;
// make sure the EEPROM is good
    if (hw.eeprom.ops.validate_checksum(hw, core::ptr::null_mut()) < 0) {
    e_dev_err("The EEPROM Checksum Is Not Valid\n");
    err = -EIO;
    goto err_sw_init;
    }
    eth_platform_get_mac_address(&adapter.pdev.dev,
    adapter.hw.mac.perm_addr);
    eth_hw_addr_set(netdev, hw.mac.perm_addr);
    if (!is_valid_ether_addr(netdev.dev_addr)) {
    e_dev_err("invalid MAC address\n");
    err = -EIO;
    goto err_sw_init;
    }
// Set hw->mac.addr to permanent MAC address
    ether_addr_copy(hw.mac.addr, hw.mac.perm_addr);
    ixgbe_mac_set_default_filter(adapter);
    timer_setup(&adapter.service_timer, ixgbe_service_timer, 0);
    if (ixgbe_removed(hw.hw_addr)) {
    err = -EIO;
    goto err_sw_init;
    }
    INIT_WORK(&adapter.service_task, ixgbe_service_task);
    set_bit(__IXGBE_SERVICE_INITED, &adapter.state);
    clear_bit(__IXGBE_SERVICE_SCHED, &adapter.state);
    err = ixgbe_init_interrupt_scheme(adapter);
    if (err)
    goto err_sw_init;
    for (i = 0; i < adapter.num_rx_queues; i++)
    u64_stats_init(&adapter.rx_ring[i].syncp);
    for (i = 0; i < adapter.num_tx_queues; i++)
    u64_stats_init(&adapter.tx_ring[i].syncp);
    for (i = 0; i < adapter.num_xdp_queues; i++)
    u64_stats_init(&adapter.xdp_ring[i].syncp);
// WOL not supported for all devices
    adapter.wol = 0;
    hw.eeprom.ops.read(hw, 0x2c, &adapter.eeprom_cap);
    hw.wol_enabled = ixgbe_wol_supported(adapter, pdev.device,
    pdev.subsystem_device);
    if (hw.wol_enabled)
    adapter.wol = IXGBE_WUFC_MAG;
    device_set_wakeup_enable(&adapter.pdev.dev, adapter.wol);
// save off EEPROM version number
    ixgbe_set_fw_version(adapter);
// pick up the PCI bus settings for reporting later
    if (ixgbe_pcie_from_parent(hw))
    ixgbe_get_parent_bus_info(adapter);
    else
    hw.mac.ops.get_bus_info(hw);
// calculate the expected PCIe bandwidth required for optimal
// performance. Note that some older parts will never have enough
// bandwidth due to being older generation PCIe parts. We clamp these
// parts to ensure no warning is displayed if it can't be fixed.
//
    switch (hw.mac.type) {
    case ixgbe_mac_82598EB:
    expected_gts = min(ixgbe_enumerate_functions(adapter) * 10, 16);
    break;
    default:
    expected_gts = ixgbe_enumerate_functions(adapter) * 10;
    break;
    }
// don't check link if we failed to enumerate functions
    if (expected_gts > 0)
    ixgbe_check_minimum_link(adapter, expected_gts);
    err = hw.eeprom.ops.read_pba_string(hw, part_str, sizeof(part_str));
    if (err)
    strscpy(part_str, "Unknown", sizeof(part_str));
    if (ixgbe_is_sfp(hw) && hw.phy.sfp_type != ixgbe_sfp_type_not_present)
    e_dev_info("MAC: %d, PHY: %d, SFP+: %d, PBA No: %s\n",
    hw.mac.type, hw.phy.type, hw.phy.sfp_type,
    part_str);
    else
    e_dev_info("MAC: %d, PHY: %d, PBA No: %s\n",
    hw.mac.type, hw.phy.type, part_str);
    e_dev_info("%pM\n", netdev.dev_addr);
// reset the hardware with the new settings
    err = hw.mac.ops.start_hw(hw);
    if (err == -EACCES) {
// We are running on a pre-production device, log a warning
    e_dev_warn("This device is a pre-production adapter/LOM. "
    "Please be aware there may be issues associated "
    "with your hardware.  If you are experiencing "
    "problems please contact your Intel or hardware "
    "representative who provided you with this "
    "hardware.\n");
    }
    strcpy(netdev.name, "eth%d");
    pci_set_drvdata(pdev, adapter);
    devl_lock(adapter.devlink);
    ixgbe_devlink_register_port(adapter);
    SET_NETDEV_DEVLINK_PORT(adapter.netdev, &adapter.devlink_port);
    err = register_netdev(netdev);
    if (err)
    goto err_register;
// power down the optics for 82599 SFP+ fiber
    if (hw.mac.ops.disable_tx_laser)
    hw.mac.ops.disable_tx_laser(hw);
// carrier off reporting is important to ethtool even BEFORE open
    netif_carrier_off(netdev);

    if (dca_add_requester(&pdev.dev) == 0) {
    adapter.flags |= IXGBE_FLAG_DCA_ENABLED;
    ixgbe_setup_dca(adapter);
    }

    if (adapter.flags & IXGBE_FLAG_SRIOV_ENABLED) {
    e_info(probe, "IOV is enabled with %d VFs\n", adapter.num_vfs);
    for (i = 0; i < adapter.num_vfs; i++)
    ixgbe_vf_configuration(pdev, (i | 0x10000000));
    }
// firmware requires driver version to be 0xFFFFFFFF
// since os does not support feature
//
    if (hw.mac.ops.set_fw_drv_ver)
    hw.mac.ops.set_fw_drv_ver(hw, 0xFF, 0xFF, 0xFF, 0xFF,
    sizeof(UTS_RELEASE) - 1,
    UTS_RELEASE);
// add san mac addr to netdev
    ixgbe_add_sanmac_netdev(netdev);
    e_dev_info("%s\n", ixgbe_default_device_descr);

    if (ixgbe_sysfs_init(adapter))
    e_err(probe, "failed to allocate sysfs resources\n");

    ixgbe_dbg_adapter_init(adapter);
// setup link for SFP devices with MNG FW, else wait for IXGBE_UP
    if (ixgbe_mng_enabled(hw) && ixgbe_is_sfp(hw) && hw.mac.ops.setup_link)
    hw.mac.ops.setup_link(hw,
    IXGBE_LINK_SPEED_10GB_FULL | IXGBE_LINK_SPEED_1GB_FULL,
    true);
    err = ixgbe_mii_bus_init(hw);
    if (err)
    goto err_netdev;
    if (hw.mac.type == ixgbe_mac_e610 &&
    (adapter.flags2 & IXGBE_FLAG2_EEE_CAPABLE)) {
    let mut eee_enable: bool = adapter.flags2 & IXGBE_FLAG2_EEE_ENABLED;
    hw.mac.ops.setup_eee(hw, eee_enable);
    }
    ixgbe_devlink_init_regions(adapter);
    devl_register(adapter.devlink);
    devl_unlock(adapter.devlink);
    if (ixgbe_fwlog_init(hw))
    e_dev_info("Firmware logging not supported\n");
    return 0;
    err_netdev:
    unregister_netdev(netdev);
    err_register:
    devl_port_unregister(&adapter.devlink_port);
    devl_unlock(adapter.devlink);
    ixgbe_release_hw_control(adapter);
    ixgbe_clear_interrupt_scheme(adapter);
    err_sw_init:
    if (hw.mac.type == ixgbe_mac_e610)
    mutex_destroy(&adapter.hw.aci.lock);
    ixgbe_disable_sriov(adapter);
    adapter.flags2 &= ~IXGBE_FLAG2_SEARCH_FOR_SFP;
    iounmap(adapter.io_addr);
    kfree(adapter.jump_tables[0]);
    kfree(adapter.mac_table);
    kfree(adapter.rss_key);
    bitmap_free(adapter.af_xdp_zc_qps);
    err_ioremap:
    disable_dev = !test_and_set_bit(__IXGBE_DISABLED, &adapter.state);
    free_netdev(netdev);
    err_alloc_etherdev:
    devlink_free(adapter.devlink);
    pci_release_mem_regions(pdev);
    err_devlink:
    err_pci_reg:
    err_dma:
    if (!adapter || disable_dev)
    pci_disable_device(pdev);
    return err;
    }
//
// ixgbe_remove - Device Removal Routine
// @pdev: PCI device information struct
//
// ixgbe_remove is called by the PCI subsystem to alert the driver
// that it should release a PCI device.  This could be caused by a
// Hot-Plug event, or because the driver is going to be removed from
// memory.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_remove(pdev: *mut pci_dev) {
    static void ixgbe_remove(struct pci_dev *pdev)
    {
    struct ixgbe_adapter *adapter = pci_get_drvdata(pdev);
    struct net_device *netdev;
    bool disable_dev;
    int i;
// if !adapter then we already cleaned up in probe
    if (!adapter)
    return;
    netdev  = adapter.netdev;
    devl_lock(adapter.devlink);
    devl_unregister(adapter.devlink);
    ixgbe_devlink_destroy_regions(adapter);
    ixgbe_fwlog_deinit(&adapter.hw);
    ixgbe_dbg_adapter_exit(adapter);
    set_bit(__IXGBE_REMOVING, &adapter.state);
    cancel_work_sync(&adapter.service_task);
    if (adapter.hw.mac.type == ixgbe_mac_e610)
    ixgbe_disable_link_status_events(adapter);
    if (adapter.mii_bus)
    mdiobus_unregister(adapter.mii_bus);

    if (adapter.flags & IXGBE_FLAG_DCA_ENABLED) {
    adapter.flags &= ~IXGBE_FLAG_DCA_ENABLED;
    dca_remove_requester(&pdev.dev);
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_DCA_CTRL,
    IXGBE_DCA_CTRL_DCA_DISABLE);
    }

    ixgbe_sysfs_exit(adapter);

// remove the added san mac
    ixgbe_del_sanmac_netdev(netdev);

    ixgbe_disable_sriov(adapter);

    if (netdev.reg_state == NETREG_REGISTERED)
    unregister_netdev(netdev);
    devl_port_unregister(&adapter.devlink_port);
    devl_unlock(adapter.devlink);
    ixgbe_stop_ipsec_offload(adapter);
    ixgbe_clear_interrupt_scheme(adapter);
    ixgbe_release_hw_control(adapter);

    kfree(adapter.ixgbe_ieee_pfc);
    kfree(adapter.ixgbe_ieee_ets);

    iounmap(adapter.io_addr);
    pci_release_mem_regions(pdev);
    e_dev_info("complete\n");
    for (i = 0; i < IXGBE_MAX_LINK_HANDLE; i++) {
    if (adapter.jump_tables[i]) {
    kfree(adapter.jump_tables[i].input);
    kfree(adapter.jump_tables[i].mask);
    }
    kfree(adapter.jump_tables[i]);
    }
    kfree(adapter.mac_table);
    kfree(adapter.rss_key);
    bitmap_free(adapter.af_xdp_zc_qps);
    disable_dev = !test_and_set_bit(__IXGBE_DISABLED, &adapter.state);
    free_netdev(netdev);
    if (adapter.hw.mac.type == ixgbe_mac_e610)
    mutex_destroy(&adapter.hw.aci.lock);
    if (disable_dev)
    pci_disable_device(pdev);
    devlink_free(adapter.devlink);
    }
//
// ixgbe_io_error_detected - called when PCI error is detected
// @pdev: Pointer to PCI device
// @state: The current pci connection state
//
// This function is called after a PCI bus error affecting
// this device has been detected.
//
    static pci_ers_result_t ixgbe_io_error_detected(struct pci_dev *pdev,
    pci_channel_state_t state)
    {
    struct ixgbe_adapter *adapter = pci_get_drvdata(pdev);
    struct net_device *netdev = adapter.netdev;

    struct ixgbe_hw *hw = &adapter.hw;
    struct pci_dev *bdev, *vfdev;
    u32 dw0, dw1, dw2, dw3;
    int vf, pos;
    u16 req_id, pf_func;
    if (adapter.hw.mac.type == ixgbe_mac_82598EB ||
    adapter.num_vfs == 0)
    goto skip_bad_vf_detection;
    bdev = pdev.bus.self;
    while (bdev && (pci_pcie_type(bdev) != PCI_EXP_TYPE_ROOT_PORT))
    bdev = bdev.bus.self;
    if (!bdev)
    goto skip_bad_vf_detection;
    pos = pci_find_ext_capability(bdev, PCI_EXT_CAP_ID_ERR);
    if (!pos)
    goto skip_bad_vf_detection;
    dw0 = ixgbe_read_pci_cfg_dword(hw, pos + PCI_ERR_HEADER_LOG);
    dw1 = ixgbe_read_pci_cfg_dword(hw, pos + PCI_ERR_HEADER_LOG + 4);
    dw2 = ixgbe_read_pci_cfg_dword(hw, pos + PCI_ERR_HEADER_LOG + 8);
    dw3 = ixgbe_read_pci_cfg_dword(hw, pos + PCI_ERR_HEADER_LOG + 12);
    if (ixgbe_removed(hw.hw_addr))
    goto skip_bad_vf_detection;
    req_id = dw1 >> 16;
// On the 82599 if bit 7 of the requestor ID is set then it's a VF
    if (!(req_id & 0x0080))
    goto skip_bad_vf_detection;
    pf_func = req_id & 0x01;
    if ((pf_func & 1) == (pdev.devfn & 1)) {
    unsigned int device_id;
    vf = FIELD_GET(0x7F, req_id);
    e_dev_err("VF %d has caused a PCIe error\n", vf);
    e_dev_err("TLP: dw0: %8.8x\tdw1: %8.8x\tdw2: "
    "%8.8x\tdw3: %8.8x\n",
    dw0, dw1, dw2, dw3);
    switch (adapter.hw.mac.type) {
    case ixgbe_mac_82599EB:
    device_id = IXGBE_82599_VF_DEVICE_ID;
    break;
    case ixgbe_mac_X540:
    device_id = IXGBE_X540_VF_DEVICE_ID;
    break;
    case ixgbe_mac_X550:
    device_id = IXGBE_DEV_ID_X550_VF;
    break;
    case ixgbe_mac_X550EM_x:
    device_id = IXGBE_DEV_ID_X550EM_X_VF;
    break;
    case ixgbe_mac_x550em_a:
    device_id = IXGBE_DEV_ID_X550EM_A_VF;
    break;
    case ixgbe_mac_e610:
    device_id = IXGBE_DEV_ID_E610_VF;
    break;
    default:
    device_id = 0;
    break;
    }
// Find the pci device of the offending VF
    vfdev = pci_get_device(PCI_VENDOR_ID_INTEL, device_id, core::ptr::null_mut());
    while (vfdev) {
    if (vfdev.devfn == (req_id & 0xFF))
    break;
    vfdev = pci_get_device(PCI_VENDOR_ID_INTEL,
    device_id, vfdev);
    }
//
// There's a slim chance the VF could have been hot plugged,
// so if it is no longer present we don't need to issue the
// VFLR.  Just clean up the AER in that case.
//
    if (vfdev) {
    pcie_flr(vfdev);
// Free device reference count
    pci_dev_put(vfdev);
    }
    }
//
// Even though the error may have occurred on the other port
// we still need to increment the vf error reference count for
// both ports because the I/O resume function will be called
// for both of them.
//
    adapter.vferr_refcount++;
    return PCI_ERS_RESULT_RECOVERED;
    skip_bad_vf_detection:

    if (!test_bit(__IXGBE_SERVICE_INITED, &adapter.state))
    return PCI_ERS_RESULT_DISCONNECT;
    if (!netif_device_present(netdev))
    return PCI_ERS_RESULT_DISCONNECT;
    rtnl_lock();
    netif_device_detach(netdev);
    if (netif_running(netdev))
    ixgbe_close_suspend(adapter);
    if (state == pci_channel_io_perm_failure) {
    rtnl_unlock();
    return PCI_ERS_RESULT_DISCONNECT;
    }
    if (!test_and_set_bit(__IXGBE_DISABLED, &adapter.state))
    pci_disable_device(pdev);
    rtnl_unlock();
// Request a slot reset.
    return PCI_ERS_RESULT_NEED_RESET;
    }
//
// ixgbe_io_slot_reset - called after the pci bus has been reset.
// @pdev: Pointer to PCI device
//
// Restart the card from scratch, as if from a cold-boot.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_io_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t ixgbe_io_slot_reset(struct pci_dev *pdev)
    {
    struct ixgbe_adapter *adapter = pci_get_drvdata(pdev);
    pci_ers_result_t result;
    if (pci_enable_device_mem(pdev)) {
    e_err(probe, "Cannot re-enable PCI device after reset.\n");
    result = PCI_ERS_RESULT_DISCONNECT;
    } else {
    smp_mb__before_atomic();
    clear_bit(__IXGBE_DISABLED, &adapter.state);
    adapter.hw.hw_addr = adapter.io_addr;
    pci_set_master(pdev);
    pci_restore_state(pdev);
    pci_wake_from_d3(pdev, false);
    ixgbe_reset(adapter);
    IXGBE_WRITE_REG(&adapter.hw, IXGBE_WUS, ~0);
    result = PCI_ERS_RESULT_RECOVERED;
    }
    return result;
    }
//
// ixgbe_io_resume - called when traffic can start flowing again.
// @pdev: Pointer to PCI device
//
// This callback is called when the error recovery driver tells us that
// its OK to resume normal operation.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_io_resume(pdev: *mut pci_dev) {
    static void ixgbe_io_resume(struct pci_dev *pdev)
    {
    struct ixgbe_adapter *adapter = pci_get_drvdata(pdev);
    struct net_device *netdev = adapter.netdev;

    if (adapter.vferr_refcount) {
    e_info(drv, "Resuming after VF err\n");
    adapter.vferr_refcount--;
    return;
    }

    rtnl_lock();
    if (netif_running(netdev))
    ixgbe_open(netdev);
    netif_device_attach(netdev);
    rtnl_unlock();
    }
    static const struct pci_error_handlers ixgbe_err_handler = {
    .error_detected = ixgbe_io_error_detected,
    .slot_reset = ixgbe_io_slot_reset,
    .resume = ixgbe_io_resume,
    };
    static DEFINE_SIMPLE_DEV_PM_OPS(ixgbe_pm_ops, ixgbe_suspend, ixgbe_resume);
    static struct pci_driver ixgbe_driver = {
    .name      = ixgbe_driver_name,
    .id_table  = ixgbe_pci_tbl,
    .probe     = ixgbe_probe,
    .remove    = ixgbe_remove,
    .driver.pm = pm_sleep_ptr(&ixgbe_pm_ops),
    .shutdown  = ixgbe_shutdown,
    .sriov_configure = ixgbe_pci_sriov_configure,
    .err_handler = &ixgbe_err_handler
    };
//
// ixgbe_init_module - Driver Registration Routine
//
// ixgbe_init_module is the first routine called when the driver is
// loaded. All it does is register with the PCI subsystem.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_init_module() -> int __init {
    static int __init ixgbe_init_module(void)
    {
    int ret;
    pr_info("%s\n", ixgbe_driver_string);
    pr_info("%s\n", ixgbe_copyright);
    ixgbe_wq = create_singlethread_workqueue(ixgbe_driver_name);
    if (!ixgbe_wq) {
    pr_err("%s: Failed to create workqueue\n", ixgbe_driver_name);
    return -ENOMEM;
    }
    ixgbe_dbg_init();
    ret = pci_register_driver(&ixgbe_driver);
    if (ret) {
    destroy_workqueue(ixgbe_wq);
    ixgbe_dbg_exit();
    return ret;
    }

    dca_register_notify(&dca_notifier);

    return 0;
    }
    module_init(ixgbe_init_module);
//
// ixgbe_exit_module - Driver Exit Cleanup Routine
//
// ixgbe_exit_module is called just before the driver is removed
// from memory.
//
#[no_mangle]
unsafe extern "C" fn ixgbe_exit_module() -> void __exit {
    static void __exit ixgbe_exit_module(void)
    {

    dca_unregister_notify(&dca_notifier);

    pci_unregister_driver(&ixgbe_driver);
    ixgbe_dbg_exit();
    if (ixgbe_wq) {
    destroy_workqueue(ixgbe_wq);
    ixgbe_wq = core::ptr::null_mut();
    }
    }

    static int ixgbe_notify_dca(struct notifier_block *nb, unsigned long event,
    void *p)
    {
    int ret_val;
    ret_val = driver_for_each_device(&ixgbe_driver.driver, core::ptr::null_mut(), &event,
    __ixgbe_notify_dca);
    return ret_val ? NOTIFY_BAD : NOTIFY_DONE;
    }

    module_exit(ixgbe_exit_module);
// ixgbe_main.c
