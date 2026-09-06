//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/sja1000/plx_pci.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2008-2010 Pavel Cheblakov <P.B.Cheblakov@inp.nsk.su>
//
// Derived from the ems_pci.c driver:
// Copyright (C) 2007 Wolfgang Grandegger <wg@grandegger.com>
// Copyright (C) 2008 Markus Plessing <plessing@ems-wuensche.com>
// Copyright (C) 2008 Sebastian Haas <haas@ems-wuensche.com>
//

    MODULE_AUTHOR("Pavel Cheblakov <P.B.Cheblakov@inp.nsk.su>");
    MODULE_DESCRIPTION("Socket-CAN driver for PLX90xx PCI-bridge cards with "
    "the SJA1000 chips");
    MODULE_LICENSE("GPL v2");
pub const PLX_PCI_MAX_CHAN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_pci_card {
    pub /: *mut *mut int channels; / detected channels count,
    pub net_dev: [*mut net_device; PLX_PCI_MAX_CHAN],
    pub conf_addr: *mut void __iomem,
// Pointer to device-dependent reset function
    pub pdev): *mut *mut void (reset_func)(struct pci_dev,
}

// PLX9030/9050/9052 registers
pub const PLX_INTCSR: c_uint = 0x4c		/* Interrupt Control/Status */;
pub const PLX_CNTRL: c_uint = 0x50		/* User I/O, Direct Slave Response,;
// Serial EEPROM, and Initialization
// Control register
//
pub const PLX_LINT1_EN: c_uint = 0x1		/* Local interrupt 1 enable */;

// PLX9056 registers
pub const PLX9056_INTCSR: c_uint = 0x68		/* Interrupt Control/Status */;
pub const PLX9056_CNTRL: c_uint = 0x6c		/* Control / Software Reset */;

//
// The board configuration is probably following:
// RX1 is connected to ground.
// TX1 is not connected.
// CLKO is not connected.
// Setting the OCR register to 0xDA is a good idea.
// This means normal output mode, push-pull and the correct polarity.
//

// OCR setting for ASEM Dual CAN raw
pub const ASEM_PCI_OCR: c_uint = 0xfe;
//
// In the CDR register, you should set CBP to 1.
// You will probably also want to set the clock divider value to 7
// (meaning direct oscillator output) because the second SJA1000 chip
// is driven by the first one CLKOUT output.
//

// SJA1000 Control Register in the BasicCAN Mode
pub const REG_CR: c_uint = 0x00;
// States of some SJA1000 registers after hardware reset in the BasicCAN mode
pub const REG_CR_BASICCAN_INITIAL: c_uint = 0x21;
pub const REG_CR_BASICCAN_INITIAL_MASK: c_uint = 0xa1;
pub const REG_SR_BASICCAN_INITIAL: c_uint = 0x0c;
pub const REG_IR_BASICCAN_INITIAL: c_uint = 0xe0;
// States of some SJA1000 registers after hardware reset in the PeliCAN mode
pub const REG_MOD_PELICAN_INITIAL: c_uint = 0x01;
pub const REG_SR_PELICAN_INITIAL: c_uint = 0x3c;
pub const REG_IR_PELICAN_INITIAL: c_uint = 0x00;
pub const ADLINK_PCI_VENDOR_ID: c_uint = 0x144A;
pub const ADLINK_PCI_DEVICE_ID: c_uint = 0x7841;
pub const ESD_PCI_SUB_SYS_ID_PCI200: c_uint = 0x0004;
pub const ESD_PCI_SUB_SYS_ID_PCI266: c_uint = 0x0009;
pub const ESD_PCI_SUB_SYS_ID_PMC266: c_uint = 0x000e;
pub const ESD_PCI_SUB_SYS_ID_CPCI200: c_uint = 0x010b;
pub const ESD_PCI_SUB_SYS_ID_PCIE2000: c_uint = 0x0200;
pub const ESD_PCI_SUB_SYS_ID_PCI104200: c_uint = 0x0501;
pub const CAN200PCI_DEVICE_ID: c_uint = 0x9030;
pub const CAN200PCI_VENDOR_ID: c_uint = 0x10b5;
pub const CAN200PCI_SUB_DEVICE_ID: c_uint = 0x0301;
pub const CAN200PCI_SUB_VENDOR_ID: c_uint = 0xe1c5;
pub const IXXAT_PCI_VENDOR_ID: c_uint = 0x10b5;
pub const IXXAT_PCI_DEVICE_ID: c_uint = 0x9050;
pub const IXXAT_PCI_SUB_SYS_ID: c_uint = 0x2540;
pub const MARATHON_PCI_DEVICE_ID: c_uint = 0x2715;
pub const MARATHON_PCIE_DEVICE_ID: c_uint = 0x3432;
pub const TEWS_PCI_VENDOR_ID: c_uint = 0x1498;
pub const TEWS_PCI_DEVICE_ID_TMPC810: c_uint = 0x032A;
pub const CTI_PCI_DEVICE_ID_CRG001: c_uint = 0x0900;
pub const MOXA_PCI_VENDOR_ID: c_uint = 0x1393;
pub const MOXA_PCI_DEVICE_ID: c_uint = 0x0100;
pub const ASEM_RAW_CAN_VENDOR_ID: c_uint = 0x10b5;
pub const ASEM_RAW_CAN_DEVICE_ID: c_uint = 0x9030;
pub const ASEM_RAW_CAN_SUB_VENDOR_ID: c_uint = 0x3000;
pub const ASEM_RAW_CAN_SUB_DEVICE_ID: c_uint = 0x1001;
pub const ASEM_RAW_CAN_SUB_DEVICE_ID_BIS: c_uint = 0x1002;
pub const ASEM_RAW_CAN_RST_REGISTER: c_uint = 0x54;
pub const ASEM_RAW_CAN_RST_MASK_CAN1: c_uint = 0x20;
pub const ASEM_RAW_CAN_RST_MASK_CAN2: c_uint = 0x04;
    static void plx_pci_reset_common(struct pci_dev *pdev);
    static void plx9056_pci_reset_common(struct pci_dev *pdev);
    static void plx_pci_reset_marathon_pci(struct pci_dev *pdev);
    static void plx_pci_reset_marathon_pcie(struct pci_dev *pdev);
    static void plx_pci_reset_asem_dual_can_raw(struct pci_dev *pdev);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_pci_channel_map {
    pub bar: u32,
    pub offset: u32,
    pub /: *mut *mut u32 size; / 0x00 - auto, e.g. length of entire bar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_pci_card_info {
    pub name: *const c_char,
    pub channel_count: c_int,
    pub can_clock: u32,
    pub /: *mut *mut u8 ocr; / output control register,
    pub /: *mut *mut u8 cdr; / clock divider register,
// Parameters for mapping local configuration space
    pub conf_map: plx_pci_channel_map,
// Parameters for mapping the SJA1000 chips
    pub chan_map_tbl: [plx_pci_channel_map; PLX_PCI_MAX_CHAN],
// Pointer to device-dependent reset function
    pub pdev): *mut *mut void (reset_func)(struct pci_dev,
}

    static struct plx_pci_card_info plx_pci_card_info_adlink = {
    "Adlink PCI-7841/cPCI-7841", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {1, 0x00, 0x00}, { {2, 0x00, 0x80}, {2, 0x80, 0x80} },
    &plx_pci_reset_common
// based on PLX9052
    };
    static struct plx_pci_card_info plx_pci_card_info_adlink_se = {
    "Adlink PCI-7841/cPCI-7841 SE", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x80}, {2, 0x80, 0x80} },
    &plx_pci_reset_common
// based on PLX9052
    };
    static struct plx_pci_card_info plx_pci_card_info_esd200 = {
    "esd CAN-PCI/CPCI/PCI104/200", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x80}, {2, 0x100, 0x80} },
    &plx_pci_reset_common
// based on PLX9030/9050
    };
    static struct plx_pci_card_info plx_pci_card_info_esd266 = {
    "esd CAN-PCI/PMC/266", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x80}, {2, 0x100, 0x80} },
    &plx9056_pci_reset_common
// based on PLX9056
    };
    static struct plx_pci_card_info plx_pci_card_info_esd2000 = {
    "esd CAN-PCIe/2000", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x80}, {2, 0x100, 0x80} },
    &plx9056_pci_reset_common
// based on PEX8311
    };
    static struct plx_pci_card_info plx_pci_card_info_ixxat = {
    "IXXAT PC-I 04/PCI", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x80}, {2, 0x200, 0x80} },
    &plx_pci_reset_common
// based on PLX9050
    };
    static struct plx_pci_card_info plx_pci_card_info_marathon_pci = {
    "Marathon CAN-bus-PCI", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x00}, {4, 0x00, 0x00} },
    &plx_pci_reset_marathon_pci
// based on PLX9052
    };
    static struct plx_pci_card_info plx_pci_card_info_marathon_pcie = {
    "Marathon CAN-bus-PCIe", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x00}, {3, 0x80, 0x00} },
    &plx_pci_reset_marathon_pcie
// based on PEX8311
    };
    static struct plx_pci_card_info plx_pci_card_info_tews = {
    "TEWS TECHNOLOGIES TPMC810", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x000, 0x80}, {2, 0x100, 0x80} },
    &plx_pci_reset_common
// based on PLX9030
    };
    static struct plx_pci_card_info plx_pci_card_info_cti = {
    "Connect Tech Inc. CANpro/104-Plus Opto (CRG001)", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x000, 0x80}, {2, 0x100, 0x80} },
    &plx_pci_reset_common
// based on PLX9030
    };
    static struct plx_pci_card_info plx_pci_card_info_elcus = {
    "Eclus CAN-200-PCI", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {1, 0x00, 0x00}, { {2, 0x00, 0x80}, {3, 0x00, 0x80} },
    &plx_pci_reset_common
// based on PLX9030
    };
    static struct plx_pci_card_info plx_pci_card_info_moxa = {
    "MOXA", 2,
    PLX_PCI_CAN_CLOCK, PLX_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {0, 0x00, 0x80}, {1, 0x00, 0x80} },
    &plx_pci_reset_common
// based on PLX9052
    };
    static struct plx_pci_card_info plx_pci_card_info_asem_dual_can = {
    "ASEM Dual CAN raw PCI", 2,
    PLX_PCI_CAN_CLOCK, ASEM_PCI_OCR, PLX_PCI_CDR,
    {0, 0x00, 0x00}, { {2, 0x00, 0x00}, {4, 0x00, 0x00} },
    &plx_pci_reset_asem_dual_can_raw
// based on PLX9030
    };
    static const struct pci_device_id plx_pci_tbl[] = {
    {
// Adlink PCI-7841/cPCI-7841
    PCI_DEVICE(ADLINK_PCI_VENDOR_ID, ADLINK_PCI_DEVICE_ID),
    .class = PCI_CLASS_NETWORK_OTHER << 8,
    .class_mask = ~0,
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_adlink,
    }, {
// Adlink PCI-7841/cPCI-7841 SE
    PCI_DEVICE(ADLINK_PCI_VENDOR_ID, ADLINK_PCI_DEVICE_ID),
    .class = PCI_CLASS_COMMUNICATION_OTHER << 8,
    .class_mask = ~0,
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_adlink_se,
    }, {
// esd CAN-PCI/200
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9050,
    PCI_VENDOR_ID_ESDGMBH, ESD_PCI_SUB_SYS_ID_PCI200),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_esd200,
    }, {
// esd CAN-CPCI/200
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9030,
    PCI_VENDOR_ID_ESDGMBH, ESD_PCI_SUB_SYS_ID_CPCI200),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_esd200,
    }, {
// esd CAN-PCI104/200
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9030,
    PCI_VENDOR_ID_ESDGMBH, ESD_PCI_SUB_SYS_ID_PCI104200),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_esd200,
    }, {
// esd CAN-PCI/266
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9056,
    PCI_VENDOR_ID_ESDGMBH, ESD_PCI_SUB_SYS_ID_PCI266),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_esd266,
    }, {
// esd CAN-PMC/266
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9056,
    PCI_VENDOR_ID_ESDGMBH, ESD_PCI_SUB_SYS_ID_PMC266),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_esd266,
    }, {
// esd CAN-PCIE/2000
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9056,
    PCI_VENDOR_ID_ESDGMBH, ESD_PCI_SUB_SYS_ID_PCIE2000),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_esd2000,
    }, {
// IXXAT PC-I 04/PCI card
    PCI_DEVICE_SUB(IXXAT_PCI_VENDOR_ID, IXXAT_PCI_DEVICE_ID,
    PCI_ANY_ID, IXXAT_PCI_SUB_SYS_ID),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_ixxat,
    }, {
// Marathon CAN-bus-PCI card
    PCI_VDEVICE(PLX, MARATHON_PCI_DEVICE_ID),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_marathon_pci,
    }, {
// Marathon CAN-bus-PCIe card
    PCI_VDEVICE(PLX, MARATHON_PCIE_DEVICE_ID),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_marathon_pcie,
    }, {
// TEWS TECHNOLOGIES TPMC810 card
    PCI_DEVICE(TEWS_PCI_VENDOR_ID, TEWS_PCI_DEVICE_ID_TMPC810),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_tews,
    }, {
// Connect Tech Inc. CANpro/104-Plus Opto (CRG001) card
    PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9030,
    PCI_SUBVENDOR_ID_CONNECT_TECH, CTI_PCI_DEVICE_ID_CRG001),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_cti,
    }, {
// Elcus CAN-200-PCI
    PCI_DEVICE_SUB(CAN200PCI_VENDOR_ID, CAN200PCI_DEVICE_ID,
    CAN200PCI_SUB_VENDOR_ID, CAN200PCI_SUB_DEVICE_ID),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_elcus,
    }, {
// moxa
    PCI_DEVICE(MOXA_PCI_VENDOR_ID, MOXA_PCI_DEVICE_ID),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_moxa,
    }, {
// ASEM Dual CAN raw
    PCI_DEVICE_SUB(ASEM_RAW_CAN_VENDOR_ID, ASEM_RAW_CAN_DEVICE_ID,
    ASEM_RAW_CAN_SUB_VENDOR_ID, ASEM_RAW_CAN_SUB_DEVICE_ID),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_asem_dual_can,
    }, {
// ASEM Dual CAN raw -new model
    PCI_DEVICE_SUB(ASEM_RAW_CAN_VENDOR_ID, ASEM_RAW_CAN_DEVICE_ID,
    ASEM_RAW_CAN_SUB_VENDOR_ID, ASEM_RAW_CAN_SUB_DEVICE_ID_BIS),
    .driver_data = (kernel_ulong_t)&plx_pci_card_info_asem_dual_can,
    },
    { }
    };
    MODULE_DEVICE_TABLE(pci, plx_pci_tbl);
#[no_mangle]
unsafe extern "C" fn plx_pci_read_reg(priv: *const sja1000_priv, port: c_int) -> u8 {
    static u8 plx_pci_read_reg(const struct sja1000_priv *priv, int port)
    {
    return ioread8(priv.reg_base + port);
    }
#[no_mangle]
unsafe extern "C" fn plx_pci_write_reg(priv: *const sja1000_priv, port: c_int, val: u8) {
    static void plx_pci_write_reg(const struct sja1000_priv *priv, int port, u8 val)
    {
    iowrite8(val, priv.reg_base + port);
    }
//
// Check if a CAN controller is present at the specified location
// by trying to switch 'em from the Basic mode into the PeliCAN mode.
// Also check states of some registers in reset mode.
//
#[no_mangle]
pub unsafe extern "C" fn plx_pci_check_sja1000(priv: *const sja1000_priv) -> c_int {
    static inline int plx_pci_check_sja1000(const struct sja1000_priv *priv)
    {
    let mut flag: c_int = 0;
//
// Check registers after hardware reset (the Basic mode)
// See states on p. 10 of the Datasheet.
//
    if ((priv.read_reg(priv, REG_CR) & REG_CR_BASICCAN_INITIAL_MASK) ==
    REG_CR_BASICCAN_INITIAL &&
    (priv.read_reg(priv, SJA1000_SR) == REG_SR_BASICCAN_INITIAL) &&
    (priv.read_reg(priv, SJA1000_IR) == REG_IR_BASICCAN_INITIAL))
    flag = 1;
// Bring the SJA1000 into the PeliCAN mode
    priv.write_reg(priv, SJA1000_CDR, CDR_PELICAN);
//
// Check registers after reset in the PeliCAN mode.
// See states on p. 23 of the Datasheet.
//
    if (priv.read_reg(priv, SJA1000_MOD) == REG_MOD_PELICAN_INITIAL &&
    priv.read_reg(priv, SJA1000_SR) == REG_SR_PELICAN_INITIAL &&
    priv.read_reg(priv, SJA1000_IR) == REG_IR_PELICAN_INITIAL)
    return flag;
    return 0;
    }
//
// PLX9030/50/52 software reset
// Also LRESET# asserts and brings to reset device on the Local Bus (if wired).
// For most cards it's enough for reset the SJA1000 chips.
//
#[no_mangle]
unsafe extern "C" fn plx_pci_reset_common(pdev: *mut pci_dev) {
    static void plx_pci_reset_common(struct pci_dev *pdev)
    {
    struct plx_pci_card *card = pci_get_drvdata(pdev);
    u32 cntrl;
    cntrl = ioread32(card.conf_addr + PLX_CNTRL);
    cntrl |= PLX_PCI_RESET;
    iowrite32(cntrl, card.conf_addr + PLX_CNTRL);
    udelay(100);
    cntrl ^= PLX_PCI_RESET;
    iowrite32(cntrl, card.conf_addr + PLX_CNTRL);
    };
//
// PLX9056 software reset
// Assert LRESET# and reset device(s) on the Local Bus (if wired).
//
#[no_mangle]
unsafe extern "C" fn plx9056_pci_reset_common(pdev: *mut pci_dev) {
    static void plx9056_pci_reset_common(struct pci_dev *pdev)
    {
    struct plx_pci_card *card = pci_get_drvdata(pdev);
    u32 cntrl;
// issue a local bus reset
    cntrl = ioread32(card.conf_addr + PLX9056_CNTRL);
    cntrl |= PLX_PCI_RESET;
    iowrite32(cntrl, card.conf_addr + PLX9056_CNTRL);
    udelay(100);
    cntrl ^= PLX_PCI_RESET;
    iowrite32(cntrl, card.conf_addr + PLX9056_CNTRL);
// reload local configuration from EEPROM
    cntrl |= PLX9056_PCI_RCR;
    iowrite32(cntrl, card.conf_addr + PLX9056_CNTRL);
//
// There is no safe way to poll for the end
// of reconfiguration process. Waiting for 10ms
// is safe.
//
    mdelay(10);
    cntrl ^= PLX9056_PCI_RCR;
    iowrite32(cntrl, card.conf_addr + PLX9056_CNTRL);
    };
// Special reset function for Marathon CAN-bus-PCI card
#[no_mangle]
unsafe extern "C" fn plx_pci_reset_marathon_pci(pdev: *mut pci_dev) {
    static void plx_pci_reset_marathon_pci(struct pci_dev *pdev)
    {
    void __iomem *reset_addr;
    int i;
    static const int reset_bar[2] = {3, 5};
    plx_pci_reset_common(pdev);
    for (i = 0; i < 2; i++) {
    reset_addr = pci_iomap(pdev, reset_bar[i], 0);
    if (!reset_addr) {
    dev_err(&pdev.dev, "Failed to remap reset "
    "space %d (BAR%d)\n", i, reset_bar[i]);
    } else {
// reset the SJA1000 chip
    iowrite8(0x1, reset_addr);
    udelay(100);
    pci_iounmap(pdev, reset_addr);
    }
    }
    }
// Special reset function for Marathon CAN-bus-PCIe card
#[no_mangle]
unsafe extern "C" fn plx_pci_reset_marathon_pcie(pdev: *mut pci_dev) {
    static void plx_pci_reset_marathon_pcie(struct pci_dev *pdev)
    {
    void __iomem *addr;
    void __iomem *reset_addr;
    int i;
    plx9056_pci_reset_common(pdev);
    for (i = 0; i < 2; i++) {
    struct plx_pci_channel_map *chan_map =
    &plx_pci_card_info_marathon_pcie.chan_map_tbl[i];
    addr = pci_iomap(pdev, chan_map.bar, chan_map.size);
    if (!addr) {
    dev_err(&pdev.dev, "Failed to remap reset "
    "space %d (BAR%d)\n", i, chan_map.bar);
    } else {
// reset the SJA1000 chip
pub const MARATHON_PCIE_RESET_OFFSET: c_int = 32;
    reset_addr = addr + chan_map.offset +
    MARATHON_PCIE_RESET_OFFSET;
    iowrite8(0x1, reset_addr);
    udelay(100);
    pci_iounmap(pdev, addr);
    }
    }
    }
// Special reset function for ASEM Dual CAN raw card
#[no_mangle]
unsafe extern "C" fn plx_pci_reset_asem_dual_can_raw(pdev: *mut pci_dev) {
    static void plx_pci_reset_asem_dual_can_raw(struct pci_dev *pdev)
    {
    void __iomem *bar0_addr;
    u8 tmpval;
    plx_pci_reset_common(pdev);
    bar0_addr = pci_iomap(pdev, 0, 0);
    if (!bar0_addr) {
    dev_err(&pdev.dev, "Failed to remap reset space 0 (BAR0)\n");
    return;
    }
// reset the two SJA1000 chips
    tmpval = ioread8(bar0_addr + ASEM_RAW_CAN_RST_REGISTER);
    tmpval &= ~(ASEM_RAW_CAN_RST_MASK_CAN1 | ASEM_RAW_CAN_RST_MASK_CAN2);
    iowrite8(tmpval, bar0_addr + ASEM_RAW_CAN_RST_REGISTER);
    usleep_range(300, 400);
    tmpval |= ASEM_RAW_CAN_RST_MASK_CAN1 | ASEM_RAW_CAN_RST_MASK_CAN2;
    iowrite8(tmpval, bar0_addr + ASEM_RAW_CAN_RST_REGISTER);
    usleep_range(300, 400);
    pci_iounmap(pdev, bar0_addr);
    }
#[no_mangle]
unsafe extern "C" fn plx_pci_del_card(pdev: *mut pci_dev) {
    static void plx_pci_del_card(struct pci_dev *pdev)
    {
    struct plx_pci_card *card = pci_get_drvdata(pdev);
    struct net_device *dev;
    struct sja1000_priv *priv;
    let mut i: c_int = 0;
    for (i = 0; i < PLX_PCI_MAX_CHAN; i++) {
    dev = card.net_dev[i];
    if (!dev)
    continue;
    dev_info(&pdev.dev, "Removing %s\n", dev.name);
    unregister_sja1000dev(dev);
    priv = netdev_priv(dev);
    if (priv.reg_base)
    pci_iounmap(pdev, priv.reg_base);
    free_sja1000dev(dev);
    }
    card.reset_func(pdev);
//
// Disable interrupts from PCI-card and disable local
// interrupts
//
    if (pdev.device != PCI_DEVICE_ID_PLX_9056 &&
    pdev.device != MARATHON_PCIE_DEVICE_ID)
    iowrite32(0x0, card.conf_addr + PLX_INTCSR);
    else
    iowrite32(0x0, card.conf_addr + PLX9056_INTCSR);
    if (card.conf_addr)
    pci_iounmap(pdev, card.conf_addr);
    kfree(card);
    pci_disable_device(pdev);
    }
//
// Probe PLX90xx based device for the SJA1000 chips and register each
// available CAN channel to SJA1000 Socket-CAN subsystem.
//
    static int plx_pci_add_card(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct sja1000_priv *priv;
    struct net_device *dev;
    struct plx_pci_card *card;
    struct plx_pci_card_info *ci;
    int err, i;
    u32 val;
    void __iomem *addr;
    ci = (struct plx_pci_card_info *)ent.driver_data;
    if (pci_enable_device(pdev) < 0) {
    dev_err(&pdev.dev, "Failed to enable PCI device\n");
    return -ENODEV;
    }
    dev_info(&pdev.dev, "Detected \"%s\" card at slot #%i\n",
    ci.name, PCI_SLOT(pdev.devfn));
// Allocate card structures to hold addresses, ...
    card = kzalloc_obj(*card);
    if (!card) {
    pci_disable_device(pdev);
    return -ENOMEM;
    }
    pci_set_drvdata(pdev, card);
    card.channels = 0;
// Remap PLX90xx configuration space
    addr = pci_iomap(pdev, ci.conf_map.bar, ci.conf_map.size);
    if (!addr) {
    err = -ENOMEM;
    dev_err(&pdev.dev, "Failed to remap configuration space "
    "(BAR%d)\n", ci.conf_map.bar);
    goto failure_cleanup;
    }
    card.conf_addr = addr + ci.conf_map.offset;
    ci.reset_func(pdev);
    card.reset_func = ci.reset_func;
// Detect available channels
    for (i = 0; i < ci.channel_count; i++) {
    struct plx_pci_channel_map *cm = &ci.chan_map_tbl[i];
    dev = alloc_sja1000dev(0);
    if (!dev) {
    err = -ENOMEM;
    goto failure_cleanup;
    }
    card.net_dev[i] = dev;
    priv = netdev_priv(dev);
    priv.priv = card;
    priv.irq_flags = IRQF_SHARED;
    dev.irq = pdev.irq;
//
// Remap IO space of the SJA1000 chips
// This is device-dependent mapping
//
    addr = pci_iomap(pdev, cm.bar, cm.size);
    if (!addr) {
    err = -ENOMEM;
    dev_err(&pdev.dev, "Failed to remap BAR%d\n", cm.bar);
    goto failure_cleanup;
    }
    priv.reg_base = addr + cm.offset;
    priv.read_reg = plx_pci_read_reg;
    priv.write_reg = plx_pci_write_reg;
// Check if channel is present
    if (plx_pci_check_sja1000(priv)) {
    priv.can.clock.freq = ci.can_clock;
    priv.ocr = ci.ocr;
    priv.cdr = ci.cdr;
    SET_NETDEV_DEV(dev, &pdev.dev);
    dev.dev_id = i;
// Register SJA1000 device
    err = register_sja1000dev(dev);
    if (err) {
    dev_err(&pdev.dev, "Registering device failed "
    "(err=%d)\n", err);
    goto failure_cleanup;
    }
    card.channels++;
    dev_info(&pdev.dev, "Channel #%d at 0x%p, irq %d "
    "registered as %s\n", i + 1, priv.reg_base,
    dev.irq, dev.name);
    } else {
    dev_err(&pdev.dev, "Channel #%d not detected\n",
    i + 1);
    free_sja1000dev(dev);
    card.net_dev[i] = core::ptr::null_mut();
    }
    }
    if (!card.channels) {
    err = -ENODEV;
    goto failure_cleanup;
    }
//
// Enable interrupts from PCI-card (PLX90xx) and enable Local_1,
// Local_2 interrupts from the SJA1000 chips
//
    if (pdev.device != PCI_DEVICE_ID_PLX_9056 &&
    pdev.device != MARATHON_PCIE_DEVICE_ID) {
    val = ioread32(card.conf_addr + PLX_INTCSR);
    if (pdev.subsystem_vendor == PCI_VENDOR_ID_ESDGMBH)
    val |= PLX_LINT1_EN | PLX_PCI_INT_EN;
    else
    val |= PLX_LINT1_EN | PLX_LINT2_EN | PLX_PCI_INT_EN;
    iowrite32(val, card.conf_addr + PLX_INTCSR);
    } else {
    iowrite32(PLX9056_LINTI | PLX9056_PCI_INT_EN,
    card.conf_addr + PLX9056_INTCSR);
    }
    return 0;
    failure_cleanup:
    dev_err(&pdev.dev, "Error: %d. Cleaning Up.\n", err);
    plx_pci_del_card(pdev);
    return err;
    }
    static struct pci_driver plx_pci_driver = {
    .name = DRV_NAME,
    .id_table = plx_pci_tbl,
    .probe = plx_pci_add_card,
    .remove = plx_pci_del_card,
    };
    module_pci_driver(plx_pci_driver);
