//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pci-aardvark.c
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
// Driver for the Aardvark PCIe controller, used on Marvell Armada
// 3700.
//
// Copyright (C) 2016 Marvell
//
// Author: Hezi Shahmoon <hezi.shahmoon@marvell.com>
//

// PCIe core registers
pub const PCIE_CORE_DEV_ID_REG: c_uint = 0x0;
pub const PCIE_CORE_CMD_STATUS_REG: c_uint = 0x4;
pub const PCIE_CORE_DEV_REV_REG: c_uint = 0x8;
pub const PCIE_CORE_SSDEV_ID_REG: c_uint = 0x2c;
pub const PCIE_CORE_PCIEXP_CAP: c_uint = 0xc0;
pub const PCIE_CORE_PCIERR_CAP: c_uint = 0x100;
pub const PCIE_CORE_ERR_CAPCTL_REG: c_uint = 0x118;

// PIO registers base address and register offsets
pub const PIO_BASE_ADDR: c_uint = 0x4000;

pub const PIO_COMPLETION_STATUS_SHIFT: c_int = 7;

pub const PIO_COMPLETION_STATUS_OK: c_int = 0;
pub const PIO_COMPLETION_STATUS_UR: c_int = 1;
pub const PIO_COMPLETION_STATUS_RRS: c_int = 2;
pub const PIO_COMPLETION_STATUS_CA: c_int = 4;

// Aardvark Control registers
pub const CONTROL_BASE_ADDR: c_uint = 0x4800;

pub const PCIE_GEN_SEL_MSK: c_uint = 0x3;
pub const PCIE_GEN_SEL_SHIFT: c_uint = 0x0;
pub const SPEED_GEN_1: c_int = 0;
pub const SPEED_GEN_2: c_int = 1;
pub const SPEED_GEN_3: c_int = 2;
pub const IS_RC_MSK: c_int = 1;
pub const IS_RC_SHIFT: c_int = 2;
pub const LANE_CNT_MSK: c_uint = 0x18;
pub const LANE_CNT_SHIFT: c_uint = 0x3;

pub const PCIE_CORE_CTRL2_RESERVED: c_uint = 0x7;

// PCIe window configuration
pub const OB_WIN_BASE_ADDR: c_uint = 0x4c00;
pub const OB_WIN_BLOCK_SIZE: c_uint = 0x20;
pub const OB_WIN_COUNT: c_int = 8;

    OB_WIN_BLOCK_SIZE * (win) + \
    (offset))

pub const OB_WIN_FUNC_NUM_SHIFT: c_int = 24;

pub const OB_WIN_BUS_NUM_BITS_SHIFT: c_int = 20;

pub const OB_WIN_MSG_CODE_SHIFT: c_int = 14;

pub const OB_WIN_ATTR_TC_SHIFT: c_int = 8;

pub const OB_WIN_TYPE_SHIFT: c_int = 0;
pub const OB_WIN_TYPE_MEM: c_uint = 0x0;
pub const OB_WIN_TYPE_IO: c_uint = 0x4;
pub const OB_WIN_TYPE_CONFIG_TYPE0: c_uint = 0x8;
pub const OB_WIN_TYPE_CONFIG_TYPE1: c_uint = 0x9;
pub const OB_WIN_TYPE_MSG: c_uint = 0xc;
// LMI registers base address and register offsets
pub const LMI_BASE_ADDR: c_uint = 0x6000;

pub const LTSSM_SHIFT: c_int = 24;
pub const LTSSM_MASK: c_uint = 0x3f;
pub const RC_BAR_CONFIG: c_uint = 0x300;
// LTSSM values in CFG_REG
    enum {
    LTSSM_DETECT_QUIET			= 0x0,
    LTSSM_DETECT_ACTIVE			= 0x1,
    LTSSM_POLLING_ACTIVE			= 0x2,
    LTSSM_POLLING_COMPLIANCE		= 0x3,
    LTSSM_POLLING_CONFIGURATION		= 0x4,
    LTSSM_CONFIG_LINKWIDTH_START		= 0x5,
    LTSSM_CONFIG_LINKWIDTH_ACCEPT		= 0x6,
    LTSSM_CONFIG_LANENUM_ACCEPT		= 0x7,
    LTSSM_CONFIG_LANENUM_WAIT		= 0x8,
    LTSSM_CONFIG_COMPLETE			= 0x9,
    LTSSM_CONFIG_IDLE			= 0xa,
    LTSSM_RECOVERY_RCVR_LOCK		= 0xb,
    LTSSM_RECOVERY_SPEED			= 0xc,
    LTSSM_RECOVERY_RCVR_CFG			= 0xd,
    LTSSM_RECOVERY_IDLE			= 0xe,
    LTSSM_L0				= 0x10,
    LTSSM_RX_L0S_ENTRY			= 0x11,
    LTSSM_RX_L0S_IDLE			= 0x12,
    LTSSM_RX_L0S_FTS			= 0x13,
    LTSSM_TX_L0S_ENTRY			= 0x14,
    LTSSM_TX_L0S_IDLE			= 0x15,
    LTSSM_TX_L0S_FTS			= 0x16,
    LTSSM_L1_ENTRY				= 0x17,
    LTSSM_L1_IDLE				= 0x18,
    LTSSM_L2_IDLE				= 0x19,
    LTSSM_L2_TRANSMIT_WAKE			= 0x1a,
    LTSSM_DISABLED				= 0x20,
    LTSSM_LOOPBACK_ENTRY_MASTER		= 0x21,
    LTSSM_LOOPBACK_ACTIVE_MASTER		= 0x22,
    LTSSM_LOOPBACK_EXIT_MASTER		= 0x23,
    LTSSM_LOOPBACK_ENTRY_SLAVE		= 0x24,
    LTSSM_LOOPBACK_ACTIVE_SLAVE		= 0x25,
    LTSSM_LOOPBACK_EXIT_SLAVE		= 0x26,
    LTSSM_HOT_RESET				= 0x27,
    LTSSM_RECOVERY_EQUALIZATION_PHASE0	= 0x28,
    LTSSM_RECOVERY_EQUALIZATION_PHASE1	= 0x29,
    LTSSM_RECOVERY_EQUALIZATION_PHASE2	= 0x2a,
    LTSSM_RECOVERY_EQUALIZATION_PHASE3	= 0x2b,
    };

// PCIe core controller registers
pub const CTRL_CORE_BASE_ADDR: c_uint = 0x18000;

pub const CTRL_MODE_SHIFT: c_uint = 0x0;
pub const CTRL_MODE_MASK: c_uint = 0x1;
pub const PCIE_CORE_MODE_DIRECT: c_uint = 0x0;
pub const PCIE_CORE_MODE_COMMAND: c_uint = 0x1;
// PCIe Central Interrupts Registers
pub const CENTRAL_INT_BASE_ADDR: c_uint = 0x1b000;

// Transaction types
pub const PCIE_CONFIG_RD_TYPE0: c_uint = 0x8;
pub const PCIE_CONFIG_RD_TYPE1: c_uint = 0x9;
pub const PCIE_CONFIG_WR_TYPE0: c_uint = 0xa;
pub const PCIE_CONFIG_WR_TYPE1: c_uint = 0xb;

pub const RETRAIN_WAIT_MAX_RETRIES: c_int = 10;
pub const RETRAIN_WAIT_USLEEP_US: c_int = 2000;
pub const MSI_IRQ_NUM: c_int = 32;
pub const CFG_RD_RRS_VAL: c_uint = 0xffff0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct advk_pcie {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    struct {
    pub match: phys_addr_t,
    pub remap: phys_addr_t,
    pub mask: phys_addr_t,
    pub actions: u32,
    pub wins: [}; OB_WIN_COUNT],
    pub wins_count: u8,
    pub rp_irq_domain: *mut irq_domain,
    pub irq_domain: *mut irq_domain,
    pub irq_chip: irq_chip,
    pub irq_lock: raw_spinlock_t,
    pub msi_inner_domain: *mut irq_domain,
    pub msi_irq_lock: raw_spinlock_t,
    pub MSI_IRQ_NUM): DECLARE_BITMAP(msi_used,,
    pub msi_used_lock: mutex,
    pub link_gen: c_int,
    pub bridge: pci_bridge_emul,
    pub reset_gpio: *mut gpio_desc,
    pub phy: *mut phy,
}

#[no_mangle]
pub unsafe extern "C" fn advk_writel(pcie: *mut advk_pcie, val: u32, reg: u64) {
    static inline void advk_writel(struct advk_pcie *pcie, u32 val, u64 reg)
    {
    writel(val, pcie.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn advk_readl(pcie: *mut advk_pcie, reg: u64) -> u32 {
    static inline u32 advk_readl(struct advk_pcie *pcie, u64 reg)
    {
    return readl(pcie.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_ltssm_state(pcie: *mut advk_pcie) -> u8 {
    static u8 advk_pcie_ltssm_state(struct advk_pcie *pcie)
    {
    u32 val;
    u8 ltssm_state;
    val = advk_readl(pcie, CFG_REG);
    ltssm_state = (val >> LTSSM_SHIFT) & LTSSM_MASK;
    return ltssm_state;
    }
#[no_mangle]
pub unsafe extern "C" fn advk_pcie_link_up(pcie: *mut advk_pcie) -> bool {
    static inline bool advk_pcie_link_up(struct advk_pcie *pcie)
    {
// check if LTSSM is in normal operation - some L* state
    let mut ltssm_state: u8 = advk_pcie_ltssm_state(pcie);
    return ltssm_state >= LTSSM_L0 && ltssm_state < LTSSM_DISABLED;
    }
#[no_mangle]
pub unsafe extern "C" fn advk_pcie_link_active(pcie: *mut advk_pcie) -> bool {
    static inline bool advk_pcie_link_active(struct advk_pcie *pcie)
    {
//
// According to PCIe Base specification 3.0, Table 4-14: Link
// Status Mapped to the LTSSM, and 4.2.6.3.6 Configuration.Idle
// is Link Up mapped to LTSSM Configuration.Idle, Recovery, L0,
// L0s, L1 and L2 states. And according to 3.2.1. Data Link
// Control and Management State Machine Rules is DL Up status
// reported in DL Active state.
//
    let mut ltssm_state: u8 = advk_pcie_ltssm_state(pcie);
    return ltssm_state >= LTSSM_CONFIG_IDLE && ltssm_state < LTSSM_DISABLED;
    }
#[no_mangle]
pub unsafe extern "C" fn advk_pcie_link_training(pcie: *mut advk_pcie) -> bool {
    static inline bool advk_pcie_link_training(struct advk_pcie *pcie)
    {
//
// According to PCIe Base specification 3.0, Table 4-14: Link
// Status Mapped to the LTSSM is Link Training mapped to LTSSM
// Configuration and Recovery states.
//
    let mut ltssm_state: u8 = advk_pcie_ltssm_state(pcie);
    return ((ltssm_state >= LTSSM_CONFIG_LINKWIDTH_START &&
    ltssm_state < LTSSM_L0) ||
    (ltssm_state >= LTSSM_RECOVERY_EQUALIZATION_PHASE0 &&
    ltssm_state <= LTSSM_RECOVERY_EQUALIZATION_PHASE3));
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_wait_for_link(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_wait_for_link(struct advk_pcie *pcie)
    {
    int retries;
// check if the link is up or not
    for (retries = 0; retries < PCIE_LINK_WAIT_MAX_RETRIES; retries++) {
    if (advk_pcie_link_up(pcie)) {
    pci_host_common_link_train_delay(pcie.link_gen);
    return 0;
    }
    msleep(PCIE_LINK_WAIT_SLEEP_MS);
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_wait_for_retrain(pcie: *mut advk_pcie) {
    static void advk_pcie_wait_for_retrain(struct advk_pcie *pcie)
    {
    size_t retries;
    for (retries = 0; retries < RETRAIN_WAIT_MAX_RETRIES; ++retries) {
    if (advk_pcie_link_training(pcie))
    break;
    udelay(RETRAIN_WAIT_USLEEP_US);
    }
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_issue_perst(pcie: *mut advk_pcie) {
    static void advk_pcie_issue_perst(struct advk_pcie *pcie)
    {
    if (!pcie.reset_gpio)
    return;
// 10ms delay is needed for some cards
    dev_info(&pcie.pdev.dev, "issuing PERST via reset GPIO for 10ms\n");
    gpiod_set_value_cansleep(pcie.reset_gpio, 1);
    usleep_range(10000, 11000);
    gpiod_set_value_cansleep(pcie.reset_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_train_link(pcie: *mut advk_pcie) {
    static void advk_pcie_train_link(struct advk_pcie *pcie)
    {
    struct device *dev = &pcie.pdev.dev;
    u32 reg;
    int ret;
//
// Setup PCIe rev / gen compliance based on device tree property
// 'max-link-speed' which also forces maximal link speed.
//
    reg = advk_readl(pcie, PCIE_CORE_CTRL0_REG);
    reg &= ~PCIE_GEN_SEL_MSK;
    if (pcie.link_gen == 3)
    reg |= SPEED_GEN_3;
#[no_mangle]
pub unsafe extern "C" fn if(2: pcie->link_gen ==) -> else {
    else if (pcie.link_gen == 2)
    reg |= SPEED_GEN_2;
    else
    reg |= SPEED_GEN_1;
    advk_writel(pcie, reg, PCIE_CORE_CTRL0_REG);
//
// Set maximal link speed value also into PCIe Link Control 2 register.
// Armada 3700 Functional Specification says that default value is based
// on SPEED_GEN but tests showed that default value is always 8.0 GT/s.
//
    reg = advk_readl(pcie, PCIE_CORE_PCIEXP_CAP + PCI_EXP_LNKCTL2);
    reg &= ~PCI_EXP_LNKCTL2_TLS;
    if (pcie.link_gen == 3)
    reg |= PCI_EXP_LNKCTL2_TLS_8_0GT;
#[no_mangle]
pub unsafe extern "C" fn if(2: pcie->link_gen ==) -> else {
    else if (pcie.link_gen == 2)
    reg |= PCI_EXP_LNKCTL2_TLS_5_0GT;
    else
    reg |= PCI_EXP_LNKCTL2_TLS_2_5GT;
    advk_writel(pcie, reg, PCIE_CORE_PCIEXP_CAP + PCI_EXP_LNKCTL2);
// Enable link training after selecting PCIe generation
    reg = advk_readl(pcie, PCIE_CORE_CTRL0_REG);
    reg |= LINK_TRAINING_EN;
    advk_writel(pcie, reg, PCIE_CORE_CTRL0_REG);
//
// Reset PCIe card via PERST# signal. Some cards are not detected
// during link training when they are in some non-initial state.
//
    advk_pcie_issue_perst(pcie);
//
// PERST# signal could have been asserted by pinctrl subsystem before
// probe() callback has been called or issued explicitly by reset gpio
// function advk_pcie_issue_perst(), making the endpoint going into
// fundamental reset. As required by PCI Express spec (PCI Express
// Base Specification, REV. 4.0 PCI Express, February 19 2014, 6.6.1
// Conventional Reset) a delay for at least 100ms after such a reset
// before sending a Configuration Request to the device is needed.
// So wait until PCIe link is up. Function advk_pcie_wait_for_link()
// waits for link at least 900ms.
//
    ret = advk_pcie_wait_for_link(pcie);
    if (ret < 0)
    dev_err(dev, "link never came up\n");
    else
    dev_info(dev, "link up\n");
    }
//
// Set PCIe address window register which could be used for memory
// mapping.
//
    static void advk_pcie_set_ob_win(struct advk_pcie *pcie, u8 win_num,
    phys_addr_t match, phys_addr_t remap,
    phys_addr_t mask, u32 actions)
    {
    advk_writel(pcie, OB_WIN_ENABLE |
    lower_32_bits(match), OB_WIN_MATCH_LS(win_num));
    advk_writel(pcie, upper_32_bits(match), OB_WIN_MATCH_MS(win_num));
    advk_writel(pcie, lower_32_bits(remap), OB_WIN_REMAP_LS(win_num));
    advk_writel(pcie, upper_32_bits(remap), OB_WIN_REMAP_MS(win_num));
    advk_writel(pcie, lower_32_bits(mask), OB_WIN_MASK_LS(win_num));
    advk_writel(pcie, upper_32_bits(mask), OB_WIN_MASK_MS(win_num));
    advk_writel(pcie, actions, OB_WIN_ACTIONS(win_num));
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_disable_ob_win(pcie: *mut advk_pcie, win_num: u8) {
    static void advk_pcie_disable_ob_win(struct advk_pcie *pcie, u8 win_num)
    {
    advk_writel(pcie, 0, OB_WIN_MATCH_LS(win_num));
    advk_writel(pcie, 0, OB_WIN_MATCH_MS(win_num));
    advk_writel(pcie, 0, OB_WIN_REMAP_LS(win_num));
    advk_writel(pcie, 0, OB_WIN_REMAP_MS(win_num));
    advk_writel(pcie, 0, OB_WIN_MASK_LS(win_num));
    advk_writel(pcie, 0, OB_WIN_MASK_MS(win_num));
    advk_writel(pcie, 0, OB_WIN_ACTIONS(win_num));
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_setup_hw(pcie: *mut advk_pcie) {
    static void advk_pcie_setup_hw(struct advk_pcie *pcie)
    {
    phys_addr_t msi_addr;
    u32 reg;
    int i;
//
// Configure PCIe Reference clock. Direction is from the PCIe
// controller to the endpoint card, so enable transmitting of
// Reference clock differential signal off-chip and disable
// receiving off-chip differential signal.
//
    reg = advk_readl(pcie, PCIE_CORE_REF_CLK_REG);
    reg |= PCIE_CORE_REF_CLK_TX_ENABLE;
    reg &= ~PCIE_CORE_REF_CLK_RX_ENABLE;
    advk_writel(pcie, reg, PCIE_CORE_REF_CLK_REG);
// Set to Direct mode
    reg = advk_readl(pcie, CTRL_CONFIG_REG);
    reg &= ~(CTRL_MODE_MASK << CTRL_MODE_SHIFT);
    reg |= ((PCIE_CORE_MODE_DIRECT & CTRL_MODE_MASK) << CTRL_MODE_SHIFT);
    advk_writel(pcie, reg, CTRL_CONFIG_REG);
// Set PCI global control register to RC mode
    reg = advk_readl(pcie, PCIE_CORE_CTRL0_REG);
    reg |= (IS_RC_MSK << IS_RC_SHIFT);
    advk_writel(pcie, reg, PCIE_CORE_CTRL0_REG);
//
// Replace incorrect PCI vendor id value 0x1b4b by correct value 0x11ab.
// VENDOR_ID_REG contains vendor id in low 16 bits and subsystem vendor
// id in high 16 bits. Updating this register changes readback value of
// read-only vendor id bits in PCIE_CORE_DEV_ID_REG register. Workaround
// for erratum 4.1: "The value of device and vendor ID is incorrect".
//
    reg = (PCI_VENDOR_ID_MARVELL << 16) | PCI_VENDOR_ID_MARVELL;
    advk_writel(pcie, reg, VENDOR_ID_REG);
//
// Change Class Code of PCI Bridge device to PCI Bridge (0x600400),
// because the default value is Mass storage controller (0x010400).
//
// Note that this Aardvark PCI Bridge does not have compliant Type 1
// Configuration Space and it even cannot be accessed via Aardvark's
// PCI config space access method. Something like config space is
// available in internal Aardvark registers starting at offset 0x0
// and is reported as Type 0. In range 0x10 - 0x34 it has totally
// different registers.
//
// Therefore driver uses emulation of PCI Bridge which emulates
// access to configuration space via internal Aardvark registers or
// emulated configuration buffer.
//
    reg = advk_readl(pcie, PCIE_CORE_DEV_REV_REG);
    reg &= ~0xffffff00;
    reg |= PCI_CLASS_BRIDGE_PCI_NORMAL << 8;
    advk_writel(pcie, reg, PCIE_CORE_DEV_REV_REG);
// Disable Root Bridge I/O space, memory space and bus mastering
    reg = advk_readl(pcie, PCIE_CORE_CMD_STATUS_REG);
    reg &= ~(PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
    advk_writel(pcie, reg, PCIE_CORE_CMD_STATUS_REG);
// Set Advanced Error Capabilities and Control PF0 register
    reg = PCIE_CORE_ERR_CAPCTL_ECRC_CHK_TX |
    PCIE_CORE_ERR_CAPCTL_ECRC_CHK_TX_EN |
    PCIE_CORE_ERR_CAPCTL_ECRC_CHCK |
    PCIE_CORE_ERR_CAPCTL_ECRC_CHCK_RCV;
    advk_writel(pcie, reg, PCIE_CORE_ERR_CAPCTL_REG);
// Set PCIe Device Control register
    reg = advk_readl(pcie, PCIE_CORE_PCIEXP_CAP + PCI_EXP_DEVCTL);
    reg &= ~PCI_EXP_DEVCTL_RELAX_EN;
    reg &= ~PCI_EXP_DEVCTL_NOSNOOP_EN;
    reg &= ~PCI_EXP_DEVCTL_PAYLOAD;
    reg &= ~PCI_EXP_DEVCTL_READRQ;
    reg |= PCI_EXP_DEVCTL_PAYLOAD_512B;
    reg |= PCI_EXP_DEVCTL_READRQ_512B;
    advk_writel(pcie, reg, PCIE_CORE_PCIEXP_CAP + PCI_EXP_DEVCTL);
// Program PCIe Control 2 to disable strict ordering
    reg = PCIE_CORE_CTRL2_RESERVED |
    PCIE_CORE_CTRL2_TD_ENABLE;
    advk_writel(pcie, reg, PCIE_CORE_CTRL2_REG);
// Set lane X1
    reg = advk_readl(pcie, PCIE_CORE_CTRL0_REG);
    reg &= ~LANE_CNT_MSK;
    reg |= LANE_COUNT_1;
    advk_writel(pcie, reg, PCIE_CORE_CTRL0_REG);
// Set MSI address
    msi_addr = virt_to_phys(pcie);
    advk_writel(pcie, lower_32_bits(msi_addr), PCIE_MSI_ADDR_LOW_REG);
    advk_writel(pcie, upper_32_bits(msi_addr), PCIE_MSI_ADDR_HIGH_REG);
// Enable MSI
    reg = advk_readl(pcie, PCIE_CORE_CTRL2_REG);
    reg |= PCIE_CORE_CTRL2_MSI_ENABLE;
    advk_writel(pcie, reg, PCIE_CORE_CTRL2_REG);
// Clear all interrupts
    advk_writel(pcie, PCIE_MSI_ALL_MASK, PCIE_MSI_STATUS_REG);
    advk_writel(pcie, PCIE_ISR0_ALL_MASK, PCIE_ISR0_REG);
    advk_writel(pcie, PCIE_ISR1_ALL_MASK, PCIE_ISR1_REG);
    advk_writel(pcie, PCIE_IRQ_ALL_MASK, HOST_CTRL_INT_STATUS_REG);
// Disable All ISR0/1 and MSI Sources
    advk_writel(pcie, PCIE_ISR0_ALL_MASK, PCIE_ISR0_MASK_REG);
    advk_writel(pcie, PCIE_ISR1_ALL_MASK, PCIE_ISR1_MASK_REG);
    advk_writel(pcie, PCIE_MSI_ALL_MASK, PCIE_MSI_MASK_REG);
// Unmask summary MSI interrupt
    reg = advk_readl(pcie, PCIE_ISR0_MASK_REG);
    reg &= ~PCIE_ISR0_MSI_INT_PENDING;
    advk_writel(pcie, reg, PCIE_ISR0_MASK_REG);
// Unmask PME interrupt for processing of PME requester
    reg = advk_readl(pcie, PCIE_ISR0_MASK_REG);
    reg &= ~PCIE_MSG_PM_PME_MASK;
    advk_writel(pcie, reg, PCIE_ISR0_MASK_REG);
// Enable summary interrupt for GIC SPI source
    reg = PCIE_IRQ_ALL_MASK & (~PCIE_IRQ_ENABLE_INTS_MASK);
    advk_writel(pcie, reg, HOST_CTRL_INT_MASK_REG);
//
// Enable AXI address window location generation:
// When it is enabled, the default outbound window
// configurations (Default User Field: 0xD0074CFC)
// are used to transparent address translation for
// the outbound transactions. Thus, PCIe address
// windows are not required for transparent memory
// access when default outbound window configuration
// is set for memory access.
//
    reg = advk_readl(pcie, PCIE_CORE_CTRL2_REG);
    reg |= PCIE_CORE_CTRL2_OB_WIN_ENABLE;
    advk_writel(pcie, reg, PCIE_CORE_CTRL2_REG);
//
// Set memory access in Default User Field so it
// is not required to configure PCIe address for
// transparent memory access.
//
    advk_writel(pcie, OB_WIN_TYPE_MEM, OB_WIN_DEFAULT_ACTIONS);
//
// Bypass the address window mapping for PIO:
// Since PIO access already contains all required
// info over AXI interface by PIO registers, the
// address window is not required.
//
    reg = advk_readl(pcie, PIO_CTRL);
    reg |= PIO_CTRL_ADDR_WIN_DISABLE;
    advk_writel(pcie, reg, PIO_CTRL);
//
// Configure PCIe address windows for non-memory or
// non-transparent access as by default PCIe uses
// transparent memory access.
//
    for (i = 0; i < pcie.wins_count; i++)
    advk_pcie_set_ob_win(pcie, i,
    pcie.wins[i].match, pcie.wins[i].remap,
    pcie.wins[i].mask, pcie.wins[i].actions);
// Disable remaining PCIe outbound windows
    for (i = pcie.wins_count; i < OB_WIN_COUNT; i++)
    advk_pcie_disable_ob_win(pcie, i);
    advk_pcie_train_link(pcie);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_check_pio_status(pcie: *mut advk_pcie, allow_rrs: bool, val: *mut u32) -> c_int {
    static int advk_pcie_check_pio_status(struct advk_pcie *pcie, bool allow_rrs, u32 *val)
    {
    struct device *dev = &pcie.pdev.dev;
    u32 reg;
    unsigned int status;
    char *strcomp_status, *str_posted;
    int ret;
    reg = advk_readl(pcie, PIO_STAT);
    status = (reg & PIO_COMPLETION_STATUS_MASK) >>
    PIO_COMPLETION_STATUS_SHIFT;
//
// According to HW spec, the PIO status check sequence as below:
// 1) even if COMPLETION_STATUS(bit9:7) indicates successful,
// it still needs to check Error Status(bit11), only when this bit
// indicates no error happen, the operation is successful.
// 2) value Unsupported Request(1) of COMPLETION_STATUS(bit9:7) only
// means a PIO write error, and for PIO read it is successful with
// a read value of 0xFFFFFFFF.
// 3) value Config Request Retry Status(RRS) of COMPLETION_STATUS(bit9:7)
// only means a PIO write error, and for PIO read it is successful
// with a read value of 0xFFFF0001.
// 4) value Completer Abort (CA) of COMPLETION_STATUS(bit9:7) means
// error for both PIO read and PIO write operation.
// 5) other errors are indicated as 'unknown'.
//
    switch (status) {
    case PIO_COMPLETION_STATUS_OK:
    if (reg & PIO_ERR_STATUS) {
    strcomp_status = "COMP_ERR";
    ret = -EFAULT;
    break;
    }
// Get the read result
    if (val)
// val = advk_readl(pcie, PIO_RD_DATA);
// No error
    strcomp_status = core::ptr::null_mut();
    ret = 0;
    break;
    case PIO_COMPLETION_STATUS_UR:
    strcomp_status = "UR";
    ret = -EOPNOTSUPP;
    break;
    case PIO_COMPLETION_STATUS_RRS:
    if (allow_rrs && val) {
// PCIe r6.0, sec 2.3.2, says:
// If Configuration RRS Software Visibility is enabled:
// For a Configuration Read Request that includes both
// bytes of the Vendor ID field of a device Function's
// Configuration Space Header, the Root Complex must
// complete the Request to the host by returning a
// read-data value of 0001h for the Vendor ID field and
// all '1's for any additional bytes included in the
// request.
//
// So RRS in this case is not an error status.
//
// val = CFG_RD_RRS_VAL;
    strcomp_status = core::ptr::null_mut();
    ret = 0;
    break;
    }
// PCIe r6.0, sec 2.3.2, says:
// If RRS Software Visibility is not enabled, the Root Complex
// must re-issue the Configuration Request as a new Request.
// If RRS Software Visibility is enabled: For a Configuration
// Write Request or for any other Configuration Read Request,
// the Root Complex must re-issue the Configuration Request as
// a new Request.
// A Root Complex implementation may choose to limit the number
// of Configuration Request/RRS Completion Status loops before
// determining that something is wrong with the target of the
// Request and taking appropriate action, e.g., complete the
// Request to the host as a failed transaction.
//
// So return -EAGAIN and caller (pci-aardvark.c driver) will
// re-issue request again up to the PIO_RETRY_CNT retries.
//
    strcomp_status = "RRS";
    ret = -EAGAIN;
    break;
    case PIO_COMPLETION_STATUS_CA:
    strcomp_status = "CA";
    ret = -ECANCELED;
    break;
    default:
    strcomp_status = "Unknown";
    ret = -EINVAL;
    break;
    }
    if (!strcomp_status)
    return ret;
    if (reg & PIO_NON_POSTED_REQ)
    str_posted = "Non-posted";
    else
    str_posted = "Posted";
    dev_dbg(dev, "%s PIO Response Status: %s, %#x @ %#x\n",
    str_posted, strcomp_status, reg, advk_readl(pcie, PIO_ADDR_LS));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_wait_pio(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_wait_pio(struct advk_pcie *pcie)
    {
    struct device *dev = &pcie.pdev.dev;
    int i;
    for (i = 1; i <= PIO_RETRY_CNT; i++) {
    u32 start, isr;
    start = advk_readl(pcie, PIO_START);
    isr = advk_readl(pcie, PIO_ISR);
    if (!start && isr)
    return i;
    udelay(PIO_RETRY_DELAY);
    }
    dev_err(dev, "PIO read/write transfer time out\n");
    return -ETIMEDOUT;
    }
    static pci_bridge_emul_read_status_t
    advk_pci_bridge_emul_base_conf_read(struct pci_bridge_emul *bridge,
    int reg, u32 *value)
    {
    struct advk_pcie *pcie = bridge.data;
    switch (reg) {
    case PCI_COMMAND:
// value = advk_readl(pcie, PCIE_CORE_CMD_STATUS_REG);
    return PCI_BRIDGE_EMUL_HANDLED;
    case PCI_INTERRUPT_LINE: {
//
// From the whole 32bit register we support reading from HW only
// two bits: PCI_BRIDGE_CTL_BUS_RESET and PCI_BRIDGE_CTL_SERR.
// Other bits are retrieved only from emulated config buffer.
//
    __le32 *cfgspace = (__le32 *)&bridge.conf;
    let mut val: u32 = le32_to_cpu(cfgspace[PCI_INTERRUPT_LINE / 4]);
    if (advk_readl(pcie, PCIE_ISR0_MASK_REG) & PCIE_ISR0_ERR_MASK)
    val &= ~(PCI_BRIDGE_CTL_SERR << 16);
    else
    val |= PCI_BRIDGE_CTL_SERR << 16;
    if (advk_readl(pcie, PCIE_CORE_CTRL1_REG) & HOT_RESET_GEN)
    val |= PCI_BRIDGE_CTL_BUS_RESET << 16;
    else
    val &= ~(PCI_BRIDGE_CTL_BUS_RESET << 16);
// value = val;
    return PCI_BRIDGE_EMUL_HANDLED;
    }
    default:
    return PCI_BRIDGE_EMUL_NOT_HANDLED;
    }
    }
    static void
    advk_pci_bridge_emul_base_conf_write(struct pci_bridge_emul *bridge,
    int reg, u32 old, u32 new, u32 mask)
    {
    struct advk_pcie *pcie = bridge.data;
    switch (reg) {
    case PCI_COMMAND:
    advk_writel(pcie, new, PCIE_CORE_CMD_STATUS_REG);
    break;
    case PCI_INTERRUPT_LINE:
//
// According to Figure 6-3: Pseudo Logic Diagram for Error
// Message Controls in PCIe base specification, SERR# Enable bit
// in Bridge Control register enable receiving of ERR_* messages
//
    if (mask & (PCI_BRIDGE_CTL_SERR << 16)) {
    let mut val: u32 = advk_readl(pcie, PCIE_ISR0_MASK_REG);
    if (new & (PCI_BRIDGE_CTL_SERR << 16))
    val &= ~PCIE_ISR0_ERR_MASK;
    else
    val |= PCIE_ISR0_ERR_MASK;
    advk_writel(pcie, val, PCIE_ISR0_MASK_REG);
    }
    if (mask & (PCI_BRIDGE_CTL_BUS_RESET << 16)) {
    let mut val: u32 = advk_readl(pcie, PCIE_CORE_CTRL1_REG);
    if (new & (PCI_BRIDGE_CTL_BUS_RESET << 16))
    val |= HOT_RESET_GEN;
    else
    val &= ~HOT_RESET_GEN;
    advk_writel(pcie, val, PCIE_CORE_CTRL1_REG);
    }
    break;
    default:
    break;
    }
    }
    static pci_bridge_emul_read_status_t
    advk_pci_bridge_emul_pcie_conf_read(struct pci_bridge_emul *bridge,
    int reg, u32 *value)
    {
    struct advk_pcie *pcie = bridge.data;
    switch (reg) {
//
// PCI_EXP_SLTCAP, PCI_EXP_SLTCTL, PCI_EXP_RTCTL and PCI_EXP_RTSTA are
// also supported, but do not need to be handled here, because their
// values are stored in emulated config space buffer, and we read them
// from there when needed.
//
    case PCI_EXP_LNKCAP: {
    let mut val: u32 = advk_readl(pcie, PCIE_CORE_PCIEXP_CAP + reg);
//
// PCI_EXP_LNKCAP_DLLLARC bit is hardwired in aardvark HW to 0.
// But support for PCI_EXP_LNKSTA_DLLLA is emulated via ltssm
// state so explicitly enable PCI_EXP_LNKCAP_DLLLARC flag.
//
    val |= PCI_EXP_LNKCAP_DLLLARC;
// value = val;
    return PCI_BRIDGE_EMUL_HANDLED;
    }
    case PCI_EXP_LNKCTL: {
// u32 contains both PCI_EXP_LNKCTL and PCI_EXP_LNKSTA
    u32 val = advk_readl(pcie, PCIE_CORE_PCIEXP_CAP + reg) &
    ~(PCI_EXP_LNKSTA_LT << 16);
    if (advk_pcie_link_training(pcie))
    val |= (PCI_EXP_LNKSTA_LT << 16);
    if (advk_pcie_link_active(pcie))
    val |= (PCI_EXP_LNKSTA_DLLLA << 16);
// value = val;
    return PCI_BRIDGE_EMUL_HANDLED;
    }
    case PCI_EXP_DEVCAP:
    case PCI_EXP_DEVCTL:
    case PCI_EXP_DEVCAP2:
    case PCI_EXP_DEVCTL2:
    case PCI_EXP_LNKCAP2:
    case PCI_EXP_LNKCTL2:
// value = advk_readl(pcie, PCIE_CORE_PCIEXP_CAP + reg);
    return PCI_BRIDGE_EMUL_HANDLED;
    default:
    return PCI_BRIDGE_EMUL_NOT_HANDLED;
    }
    }
    static void
    advk_pci_bridge_emul_pcie_conf_write(struct pci_bridge_emul *bridge,
    int reg, u32 old, u32 new, u32 mask)
    {
    struct advk_pcie *pcie = bridge.data;
    switch (reg) {
    case PCI_EXP_LNKCTL:
    advk_writel(pcie, new, PCIE_CORE_PCIEXP_CAP + reg);
    if (new & PCI_EXP_LNKCTL_RL)
    advk_pcie_wait_for_retrain(pcie);
    break;
    case PCI_EXP_RTCTL: {
    let mut rootctl: u16 = le16_to_cpu(bridge.pcie_conf.rootctl);
// Only emulation of PMEIE and RRS_SVE bits is provided
    rootctl &= PCI_EXP_RTCTL_PMEIE | PCI_EXP_RTCTL_RRS_SVE;
    bridge.pcie_conf.rootctl = cpu_to_le16(rootctl);
    break;
    }
//
// PCI_EXP_RTSTA is also supported, but does not need to be handled
// here, because its value is stored in emulated config space buffer,
// and we write it there when needed.
//
    case PCI_EXP_DEVCTL:
    case PCI_EXP_DEVCTL2:
    case PCI_EXP_LNKCTL2:
    advk_writel(pcie, new, PCIE_CORE_PCIEXP_CAP + reg);
    break;
    default:
    break;
    }
    }
    static pci_bridge_emul_read_status_t
    advk_pci_bridge_emul_ext_conf_read(struct pci_bridge_emul *bridge,
    int reg, u32 *value)
    {
    struct advk_pcie *pcie = bridge.data;
    switch (reg) {
    case 0:
// value = advk_readl(pcie, PCIE_CORE_PCIERR_CAP + reg);
//
// PCI_EXT_CAP_NEXT bits are set to offset 0x150, but Armada
// 3700 Functional Specification does not document registers
// at those addresses.
//
// Thus we clear PCI_EXT_CAP_NEXT bits to make Advanced Error
// Reporting Capability header the last Extended Capability.
// If we obtain documentation for those registers in the
// future, this can be changed.
//
// value &= 0x000fffff;
    return PCI_BRIDGE_EMUL_HANDLED;
    case PCI_ERR_UNCOR_STATUS:
    case PCI_ERR_UNCOR_MASK:
    case PCI_ERR_UNCOR_SEVER:
    case PCI_ERR_COR_STATUS:
    case PCI_ERR_COR_MASK:
    case PCI_ERR_CAP:
    case PCI_ERR_HEADER_LOG + 0:
    case PCI_ERR_HEADER_LOG + 4:
    case PCI_ERR_HEADER_LOG + 8:
    case PCI_ERR_HEADER_LOG + 12:
    case PCI_ERR_ROOT_COMMAND:
    case PCI_ERR_ROOT_STATUS:
    case PCI_ERR_ROOT_ERR_SRC:
// value = advk_readl(pcie, PCIE_CORE_PCIERR_CAP + reg);
    return PCI_BRIDGE_EMUL_HANDLED;
    default:
    return PCI_BRIDGE_EMUL_NOT_HANDLED;
    }
    }
    static void
    advk_pci_bridge_emul_ext_conf_write(struct pci_bridge_emul *bridge,
    int reg, u32 old, u32 new, u32 mask)
    {
    struct advk_pcie *pcie = bridge.data;
    switch (reg) {
// These are W1C registers, so clear other bits
    case PCI_ERR_UNCOR_STATUS:
    case PCI_ERR_COR_STATUS:
    case PCI_ERR_ROOT_STATUS:
    new &= mask;
    fallthrough;
    case PCI_ERR_UNCOR_MASK:
    case PCI_ERR_UNCOR_SEVER:
    case PCI_ERR_COR_MASK:
    case PCI_ERR_CAP:
    case PCI_ERR_HEADER_LOG + 0:
    case PCI_ERR_HEADER_LOG + 4:
    case PCI_ERR_HEADER_LOG + 8:
    case PCI_ERR_HEADER_LOG + 12:
    case PCI_ERR_ROOT_COMMAND:
    case PCI_ERR_ROOT_ERR_SRC:
    advk_writel(pcie, new, PCIE_CORE_PCIERR_CAP + reg);
    break;
    default:
    break;
    }
    }
    static const struct pci_bridge_emul_ops advk_pci_bridge_emul_ops = {
    .read_base = advk_pci_bridge_emul_base_conf_read,
    .write_base = advk_pci_bridge_emul_base_conf_write,
    .read_pcie = advk_pci_bridge_emul_pcie_conf_read,
    .write_pcie = advk_pci_bridge_emul_pcie_conf_write,
    .read_ext = advk_pci_bridge_emul_ext_conf_read,
    .write_ext = advk_pci_bridge_emul_ext_conf_write,
    };
//
// Initialize the configuration space of the PCI-to-PCI bridge
// associated with the given PCIe interface.
//
#[no_mangle]
unsafe extern "C" fn advk_sw_pci_bridge_init(pcie: *mut advk_pcie) -> c_int {
    static int advk_sw_pci_bridge_init(struct advk_pcie *pcie)
    {
    struct pci_bridge_emul *bridge = &pcie.bridge;
    bridge.conf.vendor =
    cpu_to_le16(advk_readl(pcie, PCIE_CORE_DEV_ID_REG) & 0xffff);
    bridge.conf.device =
    cpu_to_le16(advk_readl(pcie, PCIE_CORE_DEV_ID_REG) >> 16);
    bridge.conf.class_revision =
    cpu_to_le32(advk_readl(pcie, PCIE_CORE_DEV_REV_REG) & 0xff);
// Support 32 bits I/O addressing
    bridge.conf.iobase = PCI_IO_RANGE_TYPE_32;
    bridge.conf.iolimit = PCI_IO_RANGE_TYPE_32;
// Support 64 bits memory pref
    bridge.conf.pref_mem_base = cpu_to_le16(PCI_PREF_RANGE_TYPE_64);
    bridge.conf.pref_mem_limit = cpu_to_le16(PCI_PREF_RANGE_TYPE_64);
// Support interrupt A for MSI feature
    bridge.conf.intpin = PCI_INTERRUPT_INTA;
//
// Aardvark HW provides PCIe Capability structure in version 2 and
// indicate slot support, which is emulated.
//
    bridge.pcie_conf.cap = cpu_to_le16(2 | PCI_EXP_FLAGS_SLOT);
//
// Set Presence Detect State bit permanently since there is no support
// for unplugging the card nor detecting whether it is plugged. (If a
// platform exists in the future that supports it, via a GPIO for
// example, it should be implemented via this bit.)
//
// Set physical slot number to 1 since there is only one port and zero
// value is reserved for ports within the same silicon as Root Port
// which is not our case.
//
    bridge.pcie_conf.slotcap = cpu_to_le32(FIELD_PREP(PCI_EXP_SLTCAP_PSN,
    1));
    bridge.pcie_conf.slotsta = cpu_to_le16(PCI_EXP_SLTSTA_PDS);
// Indicates supports for Completion Retry Status
    bridge.pcie_conf.rootcap = cpu_to_le16(PCI_EXP_RTCAP_RRS_SV);
    bridge.subsystem_vendor_id = advk_readl(pcie, PCIE_CORE_SSDEV_ID_REG) & 0xffff;
    bridge.subsystem_id = advk_readl(pcie, PCIE_CORE_SSDEV_ID_REG) >> 16;
    bridge.has_pcie = true;
    bridge.pcie_start = PCIE_CORE_PCIEXP_CAP;
    bridge.data = pcie;
    bridge.ops = &advk_pci_bridge_emul_ops;
    return pci_bridge_emul_init(bridge, 0);
    }
    static bool advk_pcie_valid_device(struct advk_pcie *pcie, struct pci_bus *bus,
    int devfn)
    {
    if (pci_is_root_bus(bus) && PCI_SLOT(devfn) != 0)
    return false;
//
// If the link goes down after we check for link-up, we have a problem:
// if a PIO request is executed while link-down, the whole controller
// gets stuck in a non-functional state, and even after link comes up
// again, PIO requests won't work anymore, and a reset of the whole PCIe
// controller is needed. Therefore we need to prevent sending PIO
// requests while the link is down.
//
    if (!pci_is_root_bus(bus) && !advk_pcie_link_up(pcie))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_pio_is_running(pcie: *mut advk_pcie) -> bool {
    static bool advk_pcie_pio_is_running(struct advk_pcie *pcie)
    {
    struct device *dev = &pcie.pdev.dev;
//
// Trying to start a new PIO transfer when previous has not completed
// cause External Abort on CPU which results in kernel panic:
//
// SError Interrupt on CPU0, code 0xbf000002 -- SError
// Kernel panic - not syncing: Asynchronous SError Interrupt
//
// Functions advk_pcie_rd_conf() and advk_pcie_wr_conf() are protected
// by raw_spin_lock_irqsave() at pci_lock_config() level to prevent
// concurrent calls at the same time. But because PIO transfer may take
// about 1.5s when link is down or card is disconnected, it means that
// advk_pcie_wait_pio() does not always have to wait for completion.
//
// Some versions of ARM Trusted Firmware handles this External Abort at
// EL3 level and mask it to prevent kernel panic. Relevant TF-A commit:
// https://git.trustedfirmware.org/TF-A/trusted-firmware-a.git/commit/?id=3c7dcdac5c50
//
    if (advk_readl(pcie, PIO_START)) {
    dev_err(dev, "Previous PIO read/write transfer is still running\n");
    return true;
    }
    return false;
    }
    static int advk_pcie_rd_conf(struct pci_bus *bus, u32 devfn,
    int where, int size, u32 *val)
    {
    struct advk_pcie *pcie = bus.sysdata;
    int retry_count;
    bool allow_rrs;
    u32 reg;
    int ret;
    if (!advk_pcie_valid_device(pcie, bus, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    if (pci_is_root_bus(bus))
    return pci_bridge_emul_conf_read(&pcie.bridge, where,
    size, val);
//
// Configuration Request Retry Status (RRS) is possible to return
// only when reading both bytes from PCI_VENDOR_ID at once and
// RRS_SVE flag on Root Port is enabled.
//
    allow_rrs = (where == PCI_VENDOR_ID) && (size >= 2) &&
    (le16_to_cpu(pcie.bridge.pcie_conf.rootctl) &
    PCI_EXP_RTCTL_RRS_SVE);
    if (advk_pcie_pio_is_running(pcie))
    goto try_rrs;
// Program the control register
    reg = advk_readl(pcie, PIO_CTRL);
    reg &= ~PIO_CTRL_TYPE_MASK;
    if (pci_is_root_bus(bus.parent))
    reg |= PCIE_CONFIG_RD_TYPE0;
    else
    reg |= PCIE_CONFIG_RD_TYPE1;
    advk_writel(pcie, reg, PIO_CTRL);
// Program the address registers
    reg = ALIGN_DOWN(PCIE_ECAM_OFFSET(bus.number, devfn, where), 4);
    advk_writel(pcie, reg, PIO_ADDR_LS);
    advk_writel(pcie, 0, PIO_ADDR_MS);
// Program the data strobe
    advk_writel(pcie, 0xf, PIO_WR_DATA_STRB);
    retry_count = 0;
    do {
// Clear PIO DONE ISR and start the transfer
    advk_writel(pcie, 1, PIO_ISR);
    advk_writel(pcie, 1, PIO_START);
    ret = advk_pcie_wait_pio(pcie);
    if (ret < 0)
    goto try_rrs;
    retry_count += ret;
// Check PIO status and get the read result
    ret = advk_pcie_check_pio_status(pcie, allow_rrs, val);
    } while (ret == -EAGAIN && retry_count < PIO_RETRY_CNT);
    if (ret < 0)
    goto fail;
    if (size == 1)
// val = (*val >> (8 * (where & 3))) & 0xff;
#[no_mangle]
pub unsafe extern "C" fn if(2: size ==) -> else {
    else if (size == 2)
// val = (*val >> (8 * (where & 3))) & 0xffff;
    return PCIBIOS_SUCCESSFUL;
    try_rrs:
//
// If it is possible, return Configuration Request Retry Status so
// that caller tries to issue the request again instead of failing.
//
    if (allow_rrs) {
// val = CFG_RD_RRS_VAL;
    return PCIBIOS_SUCCESSFUL;
    }
    fail:
// val = 0xffffffff;
    return PCIBIOS_SET_FAILED;
    }
    static int advk_pcie_wr_conf(struct pci_bus *bus, u32 devfn,
    int where, int size, u32 val)
    {
    struct advk_pcie *pcie = bus.sysdata;
    u32 reg;
    let mut data_strobe: u32 = 0x0;
    int retry_count;
    int offset;
    int ret;
    if (!advk_pcie_valid_device(pcie, bus, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    if (pci_is_root_bus(bus))
    return pci_bridge_emul_conf_write(&pcie.bridge, where,
    size, val);
    if (where % size)
    return PCIBIOS_SET_FAILED;
    if (advk_pcie_pio_is_running(pcie))
    return PCIBIOS_SET_FAILED;
// Program the control register
    reg = advk_readl(pcie, PIO_CTRL);
    reg &= ~PIO_CTRL_TYPE_MASK;
    if (pci_is_root_bus(bus.parent))
    reg |= PCIE_CONFIG_WR_TYPE0;
    else
    reg |= PCIE_CONFIG_WR_TYPE1;
    advk_writel(pcie, reg, PIO_CTRL);
// Program the address registers
    reg = ALIGN_DOWN(PCIE_ECAM_OFFSET(bus.number, devfn, where), 4);
    advk_writel(pcie, reg, PIO_ADDR_LS);
    advk_writel(pcie, 0, PIO_ADDR_MS);
// Calculate the write strobe
    offset      = where & 0x3;
    reg         = val << (8 * offset);
    data_strobe = GENMASK(size - 1, 0) << offset;
// Program the data register
    advk_writel(pcie, reg, PIO_WR_DATA);
// Program the data strobe
    advk_writel(pcie, data_strobe, PIO_WR_DATA_STRB);
    retry_count = 0;
    do {
// Clear PIO DONE ISR and start the transfer
    advk_writel(pcie, 1, PIO_ISR);
    advk_writel(pcie, 1, PIO_START);
    ret = advk_pcie_wait_pio(pcie);
    if (ret < 0)
    return PCIBIOS_SET_FAILED;
    retry_count += ret;
    ret = advk_pcie_check_pio_status(pcie, false, core::ptr::null_mut());
    } while (ret == -EAGAIN && retry_count < PIO_RETRY_CNT);
    return ret < 0 ? PCIBIOS_SET_FAILED : PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops advk_pcie_ops = {
    .read = advk_pcie_rd_conf,
    .write = advk_pcie_wr_conf,
    };
    static void advk_msi_irq_compose_msi_msg(struct irq_data *data,
    struct msi_msg *msg)
    {
    struct advk_pcie *pcie = irq_data_get_irq_chip_data(data);
    let mut msi_addr: phys_addr_t = virt_to_phys(pcie);
    msg.address_lo = lower_32_bits(msi_addr);
    msg.address_hi = upper_32_bits(msi_addr);
    msg.data = data.hwirq;
    }
#[no_mangle]
unsafe extern "C" fn advk_msi_irq_mask(d: *mut irq_data) {
    static void advk_msi_irq_mask(struct irq_data *d)
    {
    struct advk_pcie *pcie = d.domain.host_data;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    u32 mask;
    raw_spin_lock_irqsave(&pcie.msi_irq_lock, flags);
    mask = advk_readl(pcie, PCIE_MSI_MASK_REG);
    mask |= BIT(hwirq);
    advk_writel(pcie, mask, PCIE_MSI_MASK_REG);
    raw_spin_unlock_irqrestore(&pcie.msi_irq_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn advk_msi_irq_unmask(d: *mut irq_data) {
    static void advk_msi_irq_unmask(struct irq_data *d)
    {
    struct advk_pcie *pcie = d.domain.host_data;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    u32 mask;
    raw_spin_lock_irqsave(&pcie.msi_irq_lock, flags);
    mask = advk_readl(pcie, PCIE_MSI_MASK_REG);
    mask &= ~BIT(hwirq);
    advk_writel(pcie, mask, PCIE_MSI_MASK_REG);
    raw_spin_unlock_irqrestore(&pcie.msi_irq_lock, flags);
    }
    static struct irq_chip advk_msi_bottom_irq_chip = {
    .name			= "MSI",
    .irq_compose_msi_msg	= advk_msi_irq_compose_msi_msg,
    .irq_mask		= advk_msi_irq_mask,
    .irq_unmask		= advk_msi_irq_unmask,
    };
    static int advk_msi_irq_domain_alloc(struct irq_domain *domain,
    unsigned int virq,
    unsigned int nr_irqs, void *args)
    {
    struct advk_pcie *pcie = domain.host_data;
    int hwirq, i;
    mutex_lock(&pcie.msi_used_lock);
    hwirq = bitmap_find_free_region(pcie.msi_used, MSI_IRQ_NUM,
    order_base_2(nr_irqs));
    mutex_unlock(&pcie.msi_used_lock);
    if (hwirq < 0)
    return -ENOSPC;
    for (i = 0; i < nr_irqs; i++)
    irq_domain_set_info(domain, virq + i, hwirq + i,
    &advk_msi_bottom_irq_chip,
    domain.host_data, handle_simple_irq,
    core::ptr::null_mut(), core::ptr::null_mut());
    return 0;
    }
    static void advk_msi_irq_domain_free(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs)
    {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq);
    struct advk_pcie *pcie = domain.host_data;
    mutex_lock(&pcie.msi_used_lock);
    bitmap_release_region(pcie.msi_used, d.hwirq, order_base_2(nr_irqs));
    mutex_unlock(&pcie.msi_used_lock);
    }
    static const struct irq_domain_ops advk_msi_domain_ops = {
    .alloc = advk_msi_irq_domain_alloc,
    .free = advk_msi_irq_domain_free,
    };
#[no_mangle]
unsafe extern "C" fn advk_pcie_irq_mask(d: *mut irq_data) {
    static void advk_pcie_irq_mask(struct irq_data *d)
    {
    struct advk_pcie *pcie = d.domain.host_data;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    u32 mask;
    raw_spin_lock_irqsave(&pcie.irq_lock, flags);
    mask = advk_readl(pcie, PCIE_ISR1_MASK_REG);
    mask |= PCIE_ISR1_INTX_ASSERT(hwirq);
    advk_writel(pcie, mask, PCIE_ISR1_MASK_REG);
    raw_spin_unlock_irqrestore(&pcie.irq_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_irq_unmask(d: *mut irq_data) {
    static void advk_pcie_irq_unmask(struct irq_data *d)
    {
    struct advk_pcie *pcie = d.domain.host_data;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    u32 mask;
    raw_spin_lock_irqsave(&pcie.irq_lock, flags);
    mask = advk_readl(pcie, PCIE_ISR1_MASK_REG);
    mask &= ~PCIE_ISR1_INTX_ASSERT(hwirq);
    advk_writel(pcie, mask, PCIE_ISR1_MASK_REG);
    raw_spin_unlock_irqrestore(&pcie.irq_lock, flags);
    }
    static int advk_pcie_irq_map(struct irq_domain *h,
    unsigned int virq, irq_hw_number_t hwirq)
    {
    struct advk_pcie *pcie = h.host_data;
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &pcie.irq_chip,
    handle_level_irq);
    irq_set_chip_data(virq, pcie);
    return 0;
    }
    static const struct irq_domain_ops advk_pcie_irq_domain_ops = {
    .map = advk_pcie_irq_map,
    .xlate = irq_domain_xlate_onecell,
    };

    MSI_FLAG_USE_DEF_CHIP_OPS	| \
    MSI_FLAG_PCI_MSI_MASK_PARENT	| \
    MSI_FLAG_NO_AFFINITY)

    MSI_FLAG_PCI_MSIX		| \
    MSI_FLAG_MULTI_PCI_MSI)
    static const struct msi_parent_ops advk_msi_parent_ops = {
    .required_flags		= ADVK_MSI_FLAGS_REQUIRED,
    .supported_flags	= ADVK_MSI_FLAGS_SUPPORTED,
    .bus_select_token	= DOMAIN_BUS_PCI_MSI,
    .prefix			= "advk-",
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };
#[no_mangle]
unsafe extern "C" fn advk_pcie_init_msi_irq_domain(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_init_msi_irq_domain(struct advk_pcie *pcie)
    {
    struct device *dev = &pcie.pdev.dev;
    raw_spin_lock_init(&pcie.msi_irq_lock);
    mutex_init(&pcie.msi_used_lock);
    struct irq_domain_info info = {
    .fwnode		= dev_fwnode(dev),
    .ops		= &advk_msi_domain_ops,
    .host_data	= pcie,
    .size		= MSI_IRQ_NUM,
    };
    pcie.msi_inner_domain = msi_create_parent_irq_domain(&info, &advk_msi_parent_ops);
    if (!pcie.msi_inner_domain)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_remove_msi_irq_domain(pcie: *mut advk_pcie) {
    static void advk_pcie_remove_msi_irq_domain(struct advk_pcie *pcie)
    {
    irq_domain_remove(pcie.msi_inner_domain);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_init_irq_domain(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_init_irq_domain(struct advk_pcie *pcie)
    {
    struct device *dev = &pcie.pdev.dev;
    struct device_node *node = dev.of_node;
    struct device_node *pcie_intc_node;
    struct irq_chip *irq_chip;
    let mut ret: c_int = 0;
    raw_spin_lock_init(&pcie.irq_lock);
    pcie_intc_node =  of_get_next_child(node, core::ptr::null_mut());
    if (!pcie_intc_node) {
    dev_err(dev, "No PCIe Intc node found\n");
    return -ENODEV;
    }
    irq_chip = &pcie.irq_chip;
    irq_chip.name = devm_kasprintf(dev, GFP_KERNEL, "%s-irq",
    dev_name(dev));
    if (!irq_chip.name) {
    ret = -ENOMEM;
    goto out_put_node;
    }
    irq_chip.irq_mask = advk_pcie_irq_mask;
    irq_chip.irq_unmask = advk_pcie_irq_unmask;
    pcie.irq_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node), PCI_NUM_INTX,
    &advk_pcie_irq_domain_ops, pcie);
    if (!pcie.irq_domain) {
    dev_err(dev, "Failed to get a INTx IRQ domain\n");
    ret = -ENOMEM;
    goto out_put_node;
    }
    out_put_node:
    of_node_put(pcie_intc_node);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_remove_irq_domain(pcie: *mut advk_pcie) {
    static void advk_pcie_remove_irq_domain(struct advk_pcie *pcie)
    {
    irq_domain_remove(pcie.irq_domain);
    }
    static struct irq_chip advk_rp_irq_chip = {
    .name = "advk-RP",
    };
    static int advk_pcie_rp_irq_map(struct irq_domain *h,
    unsigned int virq, irq_hw_number_t hwirq)
    {
    struct advk_pcie *pcie = h.host_data;
    irq_set_chip_and_handler(virq, &advk_rp_irq_chip, handle_simple_irq);
    irq_set_chip_data(virq, pcie);
    return 0;
    }
    static const struct irq_domain_ops advk_pcie_rp_irq_domain_ops = {
    .map = advk_pcie_rp_irq_map,
    .xlate = irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn advk_pcie_init_rp_irq_domain(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_init_rp_irq_domain(struct advk_pcie *pcie)
    {
    pcie.rp_irq_domain = irq_domain_create_linear(core::ptr::null_mut(), 1, &advk_pcie_rp_irq_domain_ops, pcie);
    if (!pcie.rp_irq_domain) {
    dev_err(&pcie.pdev.dev, "Failed to add Root Port IRQ domain\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_remove_rp_irq_domain(pcie: *mut advk_pcie) {
    static void advk_pcie_remove_rp_irq_domain(struct advk_pcie *pcie)
    {
    irq_domain_remove(pcie.rp_irq_domain);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_handle_pme(pcie: *mut advk_pcie) {
    static void advk_pcie_handle_pme(struct advk_pcie *pcie)
    {
    let mut requester: u32 = advk_readl(pcie, PCIE_MSG_LOG_REG) >> 16;
    advk_writel(pcie, PCIE_MSG_PM_PME_MASK, PCIE_ISR0_REG);
//
// PCIE_MSG_LOG_REG contains the last inbound message, so store
// the requester ID only when PME was not asserted yet.
// Also do not trigger PME interrupt when PME is still asserted.
//
    if (!(le32_to_cpu(pcie.bridge.pcie_conf.rootsta) & PCI_EXP_RTSTA_PME)) {
    pcie.bridge.pcie_conf.rootsta = cpu_to_le32(requester | PCI_EXP_RTSTA_PME);
//
// Trigger PME interrupt only if PMEIE bit in Root Control is set.
// Aardvark HW returns zero for PCI_EXP_FLAGS_IRQ, so use PCIe interrupt 0.
//
    if (!(le16_to_cpu(pcie.bridge.pcie_conf.rootctl) & PCI_EXP_RTCTL_PMEIE))
    return;
    if (generic_handle_domain_irq(pcie.rp_irq_domain, 0) == -EINVAL)
    dev_err_ratelimited(&pcie.pdev.dev, "unhandled PME IRQ\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_handle_msi(pcie: *mut advk_pcie) {
    static void advk_pcie_handle_msi(struct advk_pcie *pcie)
    {
    u32 msi_val, msi_mask, msi_status, msi_idx;
    msi_mask = advk_readl(pcie, PCIE_MSI_MASK_REG);
    msi_val = advk_readl(pcie, PCIE_MSI_STATUS_REG);
    msi_status = msi_val & ((~msi_mask) & PCIE_MSI_ALL_MASK);
    for (msi_idx = 0; msi_idx < MSI_IRQ_NUM; msi_idx++) {
    if (!(BIT(msi_idx) & msi_status))
    continue;
    advk_writel(pcie, BIT(msi_idx), PCIE_MSI_STATUS_REG);
    if (generic_handle_domain_irq(pcie.msi_inner_domain, msi_idx) == -EINVAL)
    dev_err_ratelimited(&pcie.pdev.dev, "unexpected MSI 0x%02x\n", msi_idx);
    }
    advk_writel(pcie, PCIE_ISR0_MSI_INT_PENDING,
    PCIE_ISR0_REG);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_handle_int(pcie: *mut advk_pcie) {
    static void advk_pcie_handle_int(struct advk_pcie *pcie)
    {
    u32 isr0_val, isr0_mask, isr0_status;
    u32 isr1_val, isr1_mask, isr1_status;
    int i;
    isr0_val = advk_readl(pcie, PCIE_ISR0_REG);
    isr0_mask = advk_readl(pcie, PCIE_ISR0_MASK_REG);
    isr0_status = isr0_val & ((~isr0_mask) & PCIE_ISR0_ALL_MASK);
    isr1_val = advk_readl(pcie, PCIE_ISR1_REG);
    isr1_mask = advk_readl(pcie, PCIE_ISR1_MASK_REG);
    isr1_status = isr1_val & ((~isr1_mask) & PCIE_ISR1_ALL_MASK);
// Process PME interrupt as the first one to do not miss PME requester id
    if (isr0_status & PCIE_MSG_PM_PME_MASK)
    advk_pcie_handle_pme(pcie);
// Process ERR interrupt
    if (isr0_status & PCIE_ISR0_ERR_MASK) {
    advk_writel(pcie, PCIE_ISR0_ERR_MASK, PCIE_ISR0_REG);
//
// Aardvark HW returns zero for PCI_ERR_ROOT_AER_IRQ, so use
// PCIe interrupt 0
//
    if (generic_handle_domain_irq(pcie.rp_irq_domain, 0) == -EINVAL)
    dev_err_ratelimited(&pcie.pdev.dev, "unhandled ERR IRQ\n");
    }
// Process MSI interrupts
    if (isr0_status & PCIE_ISR0_MSI_INT_PENDING)
    advk_pcie_handle_msi(pcie);
// Process legacy interrupts
    for (i = 0; i < PCI_NUM_INTX; i++) {
    if (!(isr1_status & PCIE_ISR1_INTX_ASSERT(i)))
    continue;
    advk_writel(pcie, PCIE_ISR1_INTX_ASSERT(i),
    PCIE_ISR1_REG);
    if (generic_handle_domain_irq(pcie.irq_domain, i) == -EINVAL)
    dev_err_ratelimited(&pcie.pdev.dev, "unexpected INT%c IRQ\n",
    (char)i + 'A');
    }
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t advk_pcie_irq_handler(int irq, void *arg)
    {
    struct advk_pcie *pcie = arg;
    u32 status;
    status = advk_readl(pcie, HOST_CTRL_INT_STATUS_REG);
    if (!(status & PCIE_IRQ_CORE_INT))
    return IRQ_NONE;
    advk_pcie_handle_int(pcie);
// Clear interrupt
    advk_writel(pcie, PCIE_IRQ_CORE_INT, HOST_CTRL_INT_STATUS_REG);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int {
    static int advk_pcie_map_irq(const struct pci_dev *dev, u8 slot, u8 pin)
    {
    struct advk_pcie *pcie = dev.bus.sysdata;
//
// Emulated root bridge has its own emulated irq chip and irq domain.
// Argument pin is the INTx pin (1=INTA, 2=INTB, 3=INTC, 4=INTD) and
// hwirq for irq_create_mapping() is indexed from zero.
//
    if (pci_is_root_bus(dev.bus))
    return irq_create_mapping(pcie.rp_irq_domain, pin - 1);
    else
    return of_irq_parse_and_map_pci(dev, slot, pin);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_disable_phy(pcie: *mut advk_pcie) {
    static void advk_pcie_disable_phy(struct advk_pcie *pcie)
    {
    phy_power_off(pcie.phy);
    phy_exit(pcie.phy);
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_enable_phy(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_enable_phy(struct advk_pcie *pcie)
    {
    int ret;
    if (!pcie.phy)
    return 0;
    ret = phy_init(pcie.phy);
    if (ret)
    return ret;
    ret = phy_set_mode(pcie.phy, PHY_MODE_PCIE);
    if (ret) {
    phy_exit(pcie.phy);
    return ret;
    }
    ret = phy_power_on(pcie.phy);
    if (ret) {
    phy_exit(pcie.phy);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_setup_phy(pcie: *mut advk_pcie) -> c_int {
    static int advk_pcie_setup_phy(struct advk_pcie *pcie)
    {
    struct device *dev = &pcie.pdev.dev;
    struct device_node *node = dev.of_node;
    let mut ret: c_int = 0;
    pcie.phy = devm_of_phy_get(dev, node, core::ptr::null_mut());
    if (IS_ERR(pcie.phy) && (PTR_ERR(pcie.phy) == -EPROBE_DEFER))
    return PTR_ERR(pcie.phy);
// Old bindings miss the PHY handle
    if (IS_ERR(pcie.phy)) {
    dev_warn(dev, "PHY unavailable (%pe)\n", pcie.phy);
    pcie.phy = core::ptr::null_mut();
    return 0;
    }
    ret = advk_pcie_enable_phy(pcie);
    if (ret)
    dev_err(dev, "Failed to initialize PHY (%d)\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int advk_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct advk_pcie *pcie;
    struct pci_host_bridge *bridge;
    struct resource_entry *entry;
    int ret, irq;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(struct advk_pcie));
    if (!bridge)
    return -ENOMEM;
    pcie = pci_host_bridge_priv(bridge);
    pcie.pdev = pdev;
    platform_set_drvdata(pdev, pcie);
    resource_list_for_each_entry(entry, &bridge.windows) {
    let mut start: resource_size_t = entry.res.start;
    let mut size: resource_size_t = resource_size(entry.res);
    let mut type: c_ulong = resource_type(entry.res);
    u64 win_size;
//
// Aardvark hardware allows to configure also PCIe window
// for config type 0 and type 1 mapping, but driver uses
// only PIO for issuing configuration transfers which does
// not use PCIe window configuration.
//
    if (type != IORESOURCE_MEM && type != IORESOURCE_IO)
    continue;
//
// Skip transparent memory resources. Default outbound access
// configuration is set to transparent memory access so it
// does not need window configuration.
//
    if (type == IORESOURCE_MEM && entry.offset == 0)
    continue;
//
// The n-th PCIe window is configured by tuple (match, remap, mask)
// and an access to address A uses this window if A matches the
// match with given mask.
// So every PCIe window size must be a power of two and every start
// address must be aligned to window size. Minimal size is 64 KiB
// because lower 16 bits of mask must be zero. Remapped address
// may have set only bits from the mask.
//
    while (pcie.wins_count < OB_WIN_COUNT && size > 0) {
// Calculate the largest aligned window size
    win_size = (1ULL << (fls64(size)-1)) |
    (start ? (1ULL << __ffs64(start)) : 0);
    win_size = 1ULL << __ffs64(win_size);
    if (win_size < 0x10000)
    break;
    dev_dbg(dev,
    "Configuring PCIe window %d: [0x%llx-0x%llx] as %lu\n",
    pcie.wins_count, (unsigned long long)start,
    (unsigned long long)start + win_size, type);
    if (type == IORESOURCE_IO) {
    pcie.wins[pcie.wins_count].actions = OB_WIN_TYPE_IO;
    pcie.wins[pcie.wins_count].match = pci_pio_to_address(start);
    } else {
    pcie.wins[pcie.wins_count].actions = OB_WIN_TYPE_MEM;
    pcie.wins[pcie.wins_count].match = start;
    }
    pcie.wins[pcie.wins_count].remap = start - entry.offset;
    pcie.wins[pcie.wins_count].mask = ~(win_size - 1);
    if (pcie.wins[pcie.wins_count].remap & (win_size - 1))
    break;
    start += win_size;
    size -= win_size;
    pcie.wins_count++;
    }
    if (size > 0) {
    dev_err(&pcie.pdev.dev,
    "Invalid PCIe region [0x%llx-0x%llx]\n",
    (unsigned long long)entry.res.start,
    (unsigned long long)entry.res.end + 1);
    return -EINVAL;
    }
    }
    pcie.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pcie.base))
    return PTR_ERR(pcie.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, advk_pcie_irq_handler,
    IRQF_SHARED | IRQF_NO_THREAD, "advk-pcie",
    pcie);
    if (ret) {
    dev_err(dev, "Failed to register interrupt\n");
    return ret;
    }
    pcie.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    ret = PTR_ERR_OR_ZERO(pcie.reset_gpio);
    if (ret) {
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Failed to get reset-gpio: %i\n", ret);
    return ret;
    }
    ret = gpiod_set_consumer_name(pcie.reset_gpio, "pcie1-reset");
    if (ret) {
    dev_err(dev, "Failed to set reset gpio name: %d\n", ret);
    return ret;
    }
    ret = of_pci_get_max_link_speed(dev.of_node);
    if (ret <= 0 || ret > 3)
    pcie.link_gen = 3;
    else
    pcie.link_gen = ret;
    ret = advk_pcie_setup_phy(pcie);
    if (ret)
    return ret;
    advk_pcie_setup_hw(pcie);
    ret = advk_sw_pci_bridge_init(pcie);
    if (ret) {
    dev_err(dev, "Failed to register emulated root PCI bridge\n");
    return ret;
    }
    ret = advk_pcie_init_irq_domain(pcie);
    if (ret) {
    dev_err(dev, "Failed to initialize irq\n");
    return ret;
    }
    ret = advk_pcie_init_msi_irq_domain(pcie);
    if (ret) {
    dev_err(dev, "Failed to initialize irq\n");
    advk_pcie_remove_irq_domain(pcie);
    return ret;
    }
    ret = advk_pcie_init_rp_irq_domain(pcie);
    if (ret) {
    dev_err(dev, "Failed to initialize irq\n");
    advk_pcie_remove_msi_irq_domain(pcie);
    advk_pcie_remove_irq_domain(pcie);
    return ret;
    }
    bridge.sysdata = pcie;
    bridge.ops = &advk_pcie_ops;
    bridge.map_irq = advk_pcie_map_irq;
    ret = pci_host_probe(bridge);
    if (ret < 0) {
    advk_pcie_remove_rp_irq_domain(pcie);
    advk_pcie_remove_msi_irq_domain(pcie);
    advk_pcie_remove_irq_domain(pcie);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn advk_pcie_remove(pdev: *mut platform_device) {
    static void advk_pcie_remove(struct platform_device *pdev)
    {
    struct advk_pcie *pcie = platform_get_drvdata(pdev);
    struct pci_host_bridge *bridge = pci_host_bridge_from_priv(pcie);
    u32 val;
    int i;
// Remove PCI bus with all devices
    pci_lock_rescan_remove();
    pci_stop_root_bus(bridge.bus);
    pci_remove_root_bus(bridge.bus);
    pci_unlock_rescan_remove();
// Disable Root Bridge I/O space, memory space and bus mastering
    val = advk_readl(pcie, PCIE_CORE_CMD_STATUS_REG);
    val &= ~(PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
    advk_writel(pcie, val, PCIE_CORE_CMD_STATUS_REG);
// Disable MSI
    val = advk_readl(pcie, PCIE_CORE_CTRL2_REG);
    val &= ~PCIE_CORE_CTRL2_MSI_ENABLE;
    advk_writel(pcie, val, PCIE_CORE_CTRL2_REG);
// Clear MSI address
    advk_writel(pcie, 0, PCIE_MSI_ADDR_LOW_REG);
    advk_writel(pcie, 0, PCIE_MSI_ADDR_HIGH_REG);
// Mask all interrupts
    advk_writel(pcie, PCIE_MSI_ALL_MASK, PCIE_MSI_MASK_REG);
    advk_writel(pcie, PCIE_ISR0_ALL_MASK, PCIE_ISR0_MASK_REG);
    advk_writel(pcie, PCIE_ISR1_ALL_MASK, PCIE_ISR1_MASK_REG);
    advk_writel(pcie, PCIE_IRQ_ALL_MASK, HOST_CTRL_INT_MASK_REG);
// Clear all interrupts
    advk_writel(pcie, PCIE_MSI_ALL_MASK, PCIE_MSI_STATUS_REG);
    advk_writel(pcie, PCIE_ISR0_ALL_MASK, PCIE_ISR0_REG);
    advk_writel(pcie, PCIE_ISR1_ALL_MASK, PCIE_ISR1_REG);
    advk_writel(pcie, PCIE_IRQ_ALL_MASK, HOST_CTRL_INT_STATUS_REG);
// Remove IRQ domains
    advk_pcie_remove_rp_irq_domain(pcie);
    advk_pcie_remove_msi_irq_domain(pcie);
    advk_pcie_remove_irq_domain(pcie);
// Free config space for emulated root bridge
    pci_bridge_emul_cleanup(&pcie.bridge);
// Assert PERST# signal which prepares PCIe card for power down
    if (pcie.reset_gpio)
    gpiod_set_value_cansleep(pcie.reset_gpio, 1);
// Disable link training
    val = advk_readl(pcie, PCIE_CORE_CTRL0_REG);
    val &= ~LINK_TRAINING_EN;
    advk_writel(pcie, val, PCIE_CORE_CTRL0_REG);
// Disable outbound address windows mapping
    for (i = 0; i < OB_WIN_COUNT; i++)
    advk_pcie_disable_ob_win(pcie, i);
// Disable phy
    advk_pcie_disable_phy(pcie);
    }
    static const struct of_device_id advk_pcie_of_match_table[] = {
    { .compatible = "marvell,armada-3700-pcie", },
    {},
    };
    MODULE_DEVICE_TABLE(of, advk_pcie_of_match_table);
    static struct platform_driver advk_pcie_driver = {
    .driver = {
    .name = "advk-pcie",
    .of_match_table = advk_pcie_of_match_table,
    },
    .probe = advk_pcie_probe,
    .remove = advk_pcie_remove,
    };
    module_platform_driver(advk_pcie_driver);
    MODULE_DESCRIPTION("Aardvark PCIe controller");
    MODULE_LICENSE("GPL v2");
