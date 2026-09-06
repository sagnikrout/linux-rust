//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_type.h
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

// Device IDs
pub const IXGBE_DEV_ID_82598: c_uint = 0x10B6;
pub const IXGBE_DEV_ID_82598_BX: c_uint = 0x1508;
pub const IXGBE_DEV_ID_82598AF_DUAL_PORT: c_uint = 0x10C6;
pub const IXGBE_DEV_ID_82598AF_SINGLE_PORT: c_uint = 0x10C7;
pub const IXGBE_DEV_ID_82598EB_SFP_LOM: c_uint = 0x10DB;
pub const IXGBE_DEV_ID_82598AT: c_uint = 0x10C8;
pub const IXGBE_DEV_ID_82598AT2: c_uint = 0x150B;
pub const IXGBE_DEV_ID_82598EB_CX4: c_uint = 0x10DD;
pub const IXGBE_DEV_ID_82598_CX4_DUAL_PORT: c_uint = 0x10EC;
pub const IXGBE_DEV_ID_82598_DA_DUAL_PORT: c_uint = 0x10F1;
pub const IXGBE_DEV_ID_82598_SR_DUAL_PORT_EM: c_uint = 0x10E1;
pub const IXGBE_DEV_ID_82598EB_XF_LR: c_uint = 0x10F4;
pub const IXGBE_DEV_ID_82599_KX4: c_uint = 0x10F7;
pub const IXGBE_DEV_ID_82599_KX4_MEZZ: c_uint = 0x1514;
pub const IXGBE_DEV_ID_82599_KR: c_uint = 0x1517;
pub const IXGBE_DEV_ID_82599_T3_LOM: c_uint = 0x151C;
pub const IXGBE_DEV_ID_82599_CX4: c_uint = 0x10F9;
pub const IXGBE_DEV_ID_82599_SFP: c_uint = 0x10FB;
pub const IXGBE_DEV_ID_82599_BACKPLANE_FCOE: c_uint = 0x152a;
pub const IXGBE_DEV_ID_82599_SFP_FCOE: c_uint = 0x1529;
pub const IXGBE_SUBDEV_ID_82599_SFP: c_uint = 0x11A9;
pub const IXGBE_SUBDEV_ID_82599_SFP_WOL0: c_uint = 0x1071;
pub const IXGBE_SUBDEV_ID_82599_RNDC: c_uint = 0x1F72;
pub const IXGBE_SUBDEV_ID_82599_560FLR: c_uint = 0x17D0;
pub const IXGBE_SUBDEV_ID_82599_SP_560FLR: c_uint = 0x211B;
pub const IXGBE_SUBDEV_ID_82599_LOM_SNAP6: c_uint = 0x2159;
pub const IXGBE_SUBDEV_ID_82599_SFP_1OCP: c_uint = 0x000D;
pub const IXGBE_SUBDEV_ID_82599_SFP_2OCP: c_uint = 0x0008;
pub const IXGBE_SUBDEV_ID_82599_SFP_LOM_OEM1: c_uint = 0x8976;
pub const IXGBE_SUBDEV_ID_82599_SFP_LOM_OEM2: c_uint = 0x06EE;
pub const IXGBE_SUBDEV_ID_82599_ECNA_DP: c_uint = 0x0470;
pub const IXGBE_DEV_ID_82599_SFP_EM: c_uint = 0x1507;
pub const IXGBE_DEV_ID_82599_SFP_SF2: c_uint = 0x154D;
pub const IXGBE_DEV_ID_82599EN_SFP: c_uint = 0x1557;
pub const IXGBE_SUBDEV_ID_82599EN_SFP_OCP1: c_uint = 0x0001;
pub const IXGBE_DEV_ID_82599_XAUI_LOM: c_uint = 0x10FC;
pub const IXGBE_DEV_ID_82599_COMBO_BACKPLANE: c_uint = 0x10F8;
pub const IXGBE_SUBDEV_ID_82599_KX4_KR_MEZZ: c_uint = 0x000C;
pub const IXGBE_DEV_ID_82599_LS: c_uint = 0x154F;
pub const IXGBE_DEV_ID_X540T: c_uint = 0x1528;
pub const IXGBE_DEV_ID_82599_SFP_SF_QP: c_uint = 0x154A;
pub const IXGBE_DEV_ID_82599_QSFP_SF_QP: c_uint = 0x1558;
pub const IXGBE_DEV_ID_X540T1: c_uint = 0x1560;
pub const IXGBE_DEV_ID_X550T: c_uint = 0x1563;
pub const IXGBE_DEV_ID_X550T1: c_uint = 0x15D1;
pub const IXGBE_DEV_ID_X550EM_X_KX4: c_uint = 0x15AA;
pub const IXGBE_DEV_ID_X550EM_X_KR: c_uint = 0x15AB;
pub const IXGBE_DEV_ID_X550EM_X_SFP: c_uint = 0x15AC;
pub const IXGBE_DEV_ID_X550EM_X_10G_T: c_uint = 0x15AD;
pub const IXGBE_DEV_ID_X550EM_X_1G_T: c_uint = 0x15AE;
pub const IXGBE_DEV_ID_X550EM_X_XFI: c_uint = 0x15B0;
pub const IXGBE_DEV_ID_X550EM_A_KR: c_uint = 0x15C2;
pub const IXGBE_DEV_ID_X550EM_A_KR_L: c_uint = 0x15C3;
pub const IXGBE_DEV_ID_X550EM_A_SFP_N: c_uint = 0x15C4;
pub const IXGBE_DEV_ID_X550EM_A_SGMII: c_uint = 0x15C6;
pub const IXGBE_DEV_ID_X550EM_A_SGMII_L: c_uint = 0x15C7;
pub const IXGBE_DEV_ID_X550EM_A_10G_T: c_uint = 0x15C8;
pub const IXGBE_DEV_ID_X550EM_A_SFP: c_uint = 0x15CE;
pub const IXGBE_DEV_ID_X550EM_A_1G_T: c_uint = 0x15E4;
pub const IXGBE_DEV_ID_X550EM_A_1G_T_L: c_uint = 0x15E5;
pub const IXGBE_DEV_ID_E610_BACKPLANE: c_uint = 0x57AE;
pub const IXGBE_DEV_ID_E610_SFP: c_uint = 0x57AF;
pub const IXGBE_DEV_ID_E610_10G_T: c_uint = 0x57B0;
pub const IXGBE_DEV_ID_E610_2_5G_T: c_uint = 0x57B1;
pub const IXGBE_DEV_ID_E610_SGMII: c_uint = 0x57B2;
// VF Device IDs
pub const IXGBE_DEV_ID_82599_VF: c_uint = 0x10ED;
pub const IXGBE_DEV_ID_X540_VF: c_uint = 0x1515;
pub const IXGBE_DEV_ID_X550_VF: c_uint = 0x1565;
pub const IXGBE_DEV_ID_X550EM_X_VF: c_uint = 0x15A8;
pub const IXGBE_DEV_ID_X550EM_A_VF: c_uint = 0x15C5;
pub const IXGBE_DEV_ID_E610_VF: c_uint = 0x57AD;

// General Registers
pub const IXGBE_CTRL: c_uint = 0x00000;
pub const IXGBE_STATUS: c_uint = 0x00008;
pub const IXGBE_CTRL_EXT: c_uint = 0x00018;
pub const IXGBE_ESDP: c_uint = 0x00020;
pub const IXGBE_EODSDP: c_uint = 0x00028;
pub const IXGBE_I2CCTL_8259X: c_uint = 0x00028;

pub const IXGBE_I2CCTL_X550: c_uint = 0x15F5C;

pub const IXGBE_LEDCTL: c_uint = 0x00200;
pub const IXGBE_FRTIMER: c_uint = 0x00048;
pub const IXGBE_TCPTIMER: c_uint = 0x0004C;
pub const IXGBE_CORESPARE: c_uint = 0x00600;
pub const IXGBE_EXVET: c_uint = 0x05078;
// NVM Registers
pub const IXGBE_EEC_8259X: c_uint = 0x10010;

pub const IXGBE_EEC_X550EM_a: c_uint = 0x15FF8;

pub const IXGBE_EERD: c_uint = 0x10014;
pub const IXGBE_EEWR: c_uint = 0x10018;
pub const IXGBE_FLA_8259X: c_uint = 0x1001C;

pub const IXGBE_FLA_X550EM_a: c_uint = 0x15F68;

pub const IXGBE_EEMNGCTL: c_uint = 0x10110;
pub const IXGBE_EEMNGDATA: c_uint = 0x10114;
pub const IXGBE_FLMNGCTL: c_uint = 0x10118;
pub const IXGBE_FLMNGDATA: c_uint = 0x1011C;
pub const IXGBE_FLMNGCNT: c_uint = 0x10120;
pub const IXGBE_FLOP: c_uint = 0x1013C;
pub const IXGBE_GRC_8259X: c_uint = 0x10200;

pub const IXGBE_GRC_X550EM_a: c_uint = 0x15F64;

// General Receive Control
pub const IXGBE_GRC_MNG: c_uint = 0x00000001 /* Manageability Enable */;
pub const IXGBE_GRC_APME: c_uint = 0x00000002 /* APM enabled in EEPROM */;
pub const IXGBE_VPDDIAG0: c_uint = 0x10204;
pub const IXGBE_VPDDIAG1: c_uint = 0x10208;
// I2CCTL Bit Masks
pub const IXGBE_I2C_CLK_IN_8259X: c_uint = 0x00000001;

pub const IXGBE_I2C_CLK_IN_X550: c_uint = 0x00004000;

pub const IXGBE_I2C_CLK_OUT_8259X: c_uint = 0x00000002;

pub const IXGBE_I2C_CLK_OUT_X550: c_uint = 0x00000200;

pub const IXGBE_I2C_DATA_IN_8259X: c_uint = 0x00000004;

pub const IXGBE_I2C_DATA_IN_X550: c_uint = 0x00001000;

pub const IXGBE_I2C_DATA_OUT_8259X: c_uint = 0x00000008;

pub const IXGBE_I2C_DATA_OUT_X550: c_uint = 0x00000400;

pub const IXGBE_I2C_DATA_OE_N_EN_8259X: c_int = 0;

pub const IXGBE_I2C_DATA_OE_N_EN_X550: c_uint = 0x00000800;

pub const IXGBE_I2C_BB_EN_8259X: c_int = 0;

pub const IXGBE_I2C_BB_EN_X550: c_uint = 0x00000100;

pub const IXGBE_I2C_CLK_OE_N_EN_8259X: c_int = 0;

pub const IXGBE_I2C_CLK_OE_N_EN_X550: c_uint = 0x00002000;

pub const IXGBE_I2C_CLOCK_STRETCHING_TIMEOUT: c_int = 500;
pub const IXGBE_I2C_THERMAL_SENSOR_ADDR: c_uint = 0xF8;
pub const IXGBE_EMC_INTERNAL_DATA: c_uint = 0x00;
pub const IXGBE_EMC_INTERNAL_THERM_LIMIT: c_uint = 0x20;
pub const IXGBE_EMC_DIODE1_DATA: c_uint = 0x01;
pub const IXGBE_EMC_DIODE1_THERM_LIMIT: c_uint = 0x19;
pub const IXGBE_EMC_DIODE2_DATA: c_uint = 0x23;
pub const IXGBE_EMC_DIODE2_THERM_LIMIT: c_uint = 0x1A;
pub const IXGBE_MAX_SENSORS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_thermal_diode_data {
    pub location: u8,
    pub temp: u8,
    pub caution_thresh: u8,
    pub max_op_thresh: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_thermal_sensor_data {
    pub sensor: [ixgbe_thermal_diode_data; IXGBE_MAX_SENSORS],
}

pub const NVM_OROM_OFFSET: c_uint = 0x17;
pub const NVM_OROM_BLK_LOW: c_uint = 0x83;
pub const NVM_OROM_BLK_HI: c_uint = 0x84;
pub const NVM_OROM_PATCH_MASK: c_uint = 0xFF;
pub const NVM_OROM_SHIFT: c_int = 8;
pub const NVM_VER_MASK: c_uint = 0x00FF	/* version mask */;

pub const NVM_OEM_PROD_VER_PTR: c_uint = 0x1B /* OEM Product version block pointer */;
pub const NVM_OEM_PROD_VER_CAP_OFF: c_uint = 0x1 /* OEM Product version format offset */;
pub const NVM_OEM_PROD_VER_OFF_L: c_uint = 0x2  /* OEM Product version offset low */;
pub const NVM_OEM_PROD_VER_OFF_H: c_uint = 0x3  /* OEM Product version offset high */;
pub const NVM_OEM_PROD_VER_CAP_MASK: c_uint = 0xF /* OEM Product version cap mask */;
pub const NVM_OEM_PROD_VER_MOD_LEN: c_uint = 0x3 /* OEM Product version module length */;
pub const NVM_ETK_OFF_LOW: c_uint = 0x2D /* version low order word */;
pub const NVM_ETK_OFF_HI: c_uint = 0x2E /* version high order word */;

pub const NVM_VER_INVALID: c_uint = 0xFFFF;
pub const NVM_ETK_VALID: c_uint = 0x8000;
pub const NVM_INVALID_PTR: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_nvm_version {
    pub etk_id: u32,
    pub nvm_major: u8,
    pub nvm_minor: u16,
    pub nvm_id: u8,
    pub oem_valid: bool,
    pub oem_major: u8,
    pub oem_minor: u8,
    pub oem_release: u16,
    pub or_valid: bool,
    pub or_major: u8,
    pub or_build: u16,
    pub or_patch: u8,
}

// Interrupt Registers
pub const IXGBE_EICR: c_uint = 0x00800;
pub const IXGBE_EICS: c_uint = 0x00808;
pub const IXGBE_EIMS: c_uint = 0x00880;
pub const IXGBE_EIMC: c_uint = 0x00888;
pub const IXGBE_EIAC: c_uint = 0x00810;
pub const IXGBE_EIAM: c_uint = 0x00890;

//
// 82598 EITR is 16 bits but set the limits based on the max
// supported by all ixgbe hardware.  82599 EITR is only 12 bits,
// with the lower 3 always zero.
//
pub const IXGBE_MAX_INT_RATE: c_int = 488281;
pub const IXGBE_MIN_INT_RATE: c_int = 956;
pub const IXGBE_MAX_EITR: c_uint = 0x00000FF8;
pub const IXGBE_MIN_EITR: c_int = 8;

pub const IXGBE_EITR_ITR_INT_MASK: c_uint = 0x00000FF8;
pub const IXGBE_EITR_LLI_MOD: c_uint = 0x00008000;
pub const IXGBE_EITR_CNT_WDIS: c_uint = 0x80000000;

pub const IXGBE_IVAR_MISC: c_uint = 0x00A00 /* misc MSI-X interrupt causes */;
pub const IXGBE_EITRSEL: c_uint = 0x00894;
pub const IXGBE_MSIXT: c_uint = 0x00000 /* MSI-X Table. 0x0000 - 0x01C */;
pub const IXGBE_MSIXPBA: c_uint = 0x02000 /* MSI-X Pending bit array */;

pub const IXGBE_GPIE: c_uint = 0x00898;
// Flow Control Registers
pub const IXGBE_FCADBUL: c_uint = 0x03210;
pub const IXGBE_FCADBUH: c_uint = 0x03214;
pub const IXGBE_FCAMACL: c_uint = 0x04328;
pub const IXGBE_FCAMACH: c_uint = 0x0432C;

pub const IXGBE_PFCTOP: c_uint = 0x03008;

pub const IXGBE_FCRTV: c_uint = 0x032A0;
pub const IXGBE_FCCFG: c_uint = 0x03D00;
pub const IXGBE_TFCS: c_uint = 0x0CE00;
// Receive DMA Registers

pub const IXGBE_RSCDBU: c_uint = 0x03028;
pub const IXGBE_RDDCC: c_uint = 0x02F20;
pub const IXGBE_RXMEMWRAP: c_uint = 0x03190;
pub const IXGBE_STARCTRL: c_uint = 0x03024;
//
// Split and Replication Receive Control Registers
// 00-15 : 0x02100 + n*4
// 16-64 : 0x01014 + n*0x40
// 64-127: 0x0D014 + (n-64)*0x40
//

//
// Rx DCA Control Register:
// 00-15 : 0x02200 + n*4
// 16-64 : 0x0100C + n*0x40
// 64-127: 0x0D00C + (n-64)*0x40
//

pub const IXGBE_RDRXCTL: c_uint = 0x02F00;

// 8 of these 0x03C00 - 0x03C1C
pub const IXGBE_RXCTRL: c_uint = 0x03000;
pub const IXGBE_DROPEN: c_uint = 0x03D04;
pub const IXGBE_RXPBSIZE_SHIFT: c_int = 10;
// Receive Registers
pub const IXGBE_RXCSUM: c_uint = 0x05000;
pub const IXGBE_RFCTL: c_uint = 0x05008;
pub const IXGBE_DRECCCTL: c_uint = 0x02F08;
pub const IXGBE_DRECCCTL_DISABLE: c_int = 0;
// Multicast Table Array - 128 entries

// Packet split receive type

// array of 4096 1-bit vlan filters

// array of 4096 4-bit vlan vmdq indices

pub const IXGBE_FCTRL: c_uint = 0x05080;
pub const IXGBE_VLNCTRL: c_uint = 0x05088;
pub const IXGBE_MCSTCTRL: c_uint = 0x05090;
pub const IXGBE_MRQC: c_uint = 0x05818;

pub const IXGBE_SYNQF: c_uint = 0x0EC30 /* SYN Packet Queue Filter */;
pub const IXGBE_RQTC: c_uint = 0x0EC70;
pub const IXGBE_MTQC: c_uint = 0x08120;

pub const IXGBE_PFFLPL: c_uint = 0x050B0;
pub const IXGBE_PFFLPH: c_uint = 0x050B4;
pub const IXGBE_VT_CTL: c_uint = 0x051B0;

pub const IXGBE_QDE: c_uint = 0x2F04;

pub const IXGBE_LVMMC_RX: c_uint = 0x2FA8;
pub const IXGBE_LVMMC_TX: c_uint = 0x8108;

pub const IXGBE_RXFECCERR0: c_uint = 0x051B8;
pub const IXGBE_LLITHRESH: c_uint = 0x0EC90;

pub const IXGBE_IMIRVP: c_uint = 0x05AC0;
pub const IXGBE_VMD_CTL: c_uint = 0x0581C;

// Registers for setting up RSS on X550 with SRIOV
// _p - pool number (0..63)
// _i - index (0..10 for PFVFRSSRK, 0..15 for PFVFRETA)
//

// Flow Director registers
pub const IXGBE_FDIRCTRL: c_uint = 0x0EE00;
pub const IXGBE_FDIRHKEY: c_uint = 0x0EE68;
pub const IXGBE_FDIRSKEY: c_uint = 0x0EE6C;
pub const IXGBE_FDIRDIP4M: c_uint = 0x0EE3C;
pub const IXGBE_FDIRSIP4M: c_uint = 0x0EE40;
pub const IXGBE_FDIRTCPM: c_uint = 0x0EE44;
pub const IXGBE_FDIRUDPM: c_uint = 0x0EE48;
pub const IXGBE_FDIRSCTPM: c_uint = 0x0EE78;
pub const IXGBE_FDIRIP6M: c_uint = 0x0EE74;
pub const IXGBE_FDIRM: c_uint = 0x0EE70;
// Flow Director Stats registers
pub const IXGBE_FDIRFREE: c_uint = 0x0EE38;
pub const IXGBE_FDIRLEN: c_uint = 0x0EE4C;
pub const IXGBE_FDIRUSTAT: c_uint = 0x0EE50;
pub const IXGBE_FDIRFSTAT: c_uint = 0x0EE54;
pub const IXGBE_FDIRMATCH: c_uint = 0x0EE58;
pub const IXGBE_FDIRMISS: c_uint = 0x0EE5C;
// Flow Director Programming registers

pub const IXGBE_FDIRIPSA: c_uint = 0x0EE18;
pub const IXGBE_FDIRIPDA: c_uint = 0x0EE1C;
pub const IXGBE_FDIRPORT: c_uint = 0x0EE20;
pub const IXGBE_FDIRVLAN: c_uint = 0x0EE24;
pub const IXGBE_FDIRHASH: c_uint = 0x0EE28;
pub const IXGBE_FDIRCMD: c_uint = 0x0EE2C;
// Transmit DMA registers

pub const IXGBE_DTXCTL: c_uint = 0x07E00;
pub const IXGBE_DMATXCTL: c_uint = 0x04A80;

pub const IXGBE_PFDTXGSWC: c_uint = 0x08220;
pub const IXGBE_DTXMXSZRQ: c_uint = 0x08100;
pub const IXGBE_DTXTCPFLGL: c_uint = 0x04A88;
pub const IXGBE_DTXTCPFLGH: c_uint = 0x04A8C;
pub const IXGBE_LBDRPEN: c_uint = 0x0CA00;

pub const IXGBE_DMATXCTL_TE: c_uint = 0x1 /* Transmit Enable */;
pub const IXGBE_DMATXCTL_NS: c_uint = 0x2 /* No Snoop LSO hdr buffer */;
pub const IXGBE_DMATXCTL_GDV: c_uint = 0x8 /* Global Double VLAN */;
pub const IXGBE_DMATXCTL_MDP_EN: c_uint = 0x20 /* Bit 5 */;
pub const IXGBE_DMATXCTL_MBINTEN: c_uint = 0x40 /* Bit 6 */;

pub const IXGBE_PFDTXGSWC_VT_LBEN: c_uint = 0x1 /* Local L2 VT switch enable */;
// Anti-spoofing defines
pub const IXGBE_SPOOF_MACAS_MASK: c_uint = 0xFF;
pub const IXGBE_SPOOF_VLANAS_MASK: c_uint = 0xFF00;
pub const IXGBE_SPOOF_VLANAS_SHIFT: c_int = 8;
pub const IXGBE_SPOOF_ETHERTYPEAS: c_uint = 0xFF000000;
pub const IXGBE_SPOOF_ETHERTYPEAS_SHIFT: c_int = 16;
pub const IXGBE_PFVFSPOOF_REG_COUNT: c_int = 8;

// Tx DCA Control register : 128 of these (0-127)

pub const IXGBE_TIPG: c_uint = 0x0CB00;

pub const IXGBE_MNGTXMAP: c_uint = 0x0CD10;
pub const IXGBE_TIPG_FIBER_DEFAULT: c_int = 3;
pub const IXGBE_TXPBSIZE_SHIFT: c_int = 10;
// Wake up registers
pub const IXGBE_WUC: c_uint = 0x05800;
pub const IXGBE_WUFC: c_uint = 0x05808;
pub const IXGBE_WUS: c_uint = 0x05810;
pub const IXGBE_IPAV: c_uint = 0x05838;
pub const IXGBE_IP4AT: c_uint = 0x05840 /* IPv4 table 0x5840-0x5858 */;
pub const IXGBE_IP6AT: c_uint = 0x05880 /* IPv6 table 0x5880-0x588F */;
pub const IXGBE_WUPL: c_uint = 0x05900;
pub const IXGBE_WUPM: c_uint = 0x05A00 /* wake up pkt memory 0x5A00-0x5A7C */;
pub const IXGBE_VXLANCTRL: c_uint = 0x0000507C /* Rx filter VXLAN UDPPORT Register */;

// Filter Table
// masks for accessing VXLAN and GENEVE UDP ports
pub const IXGBE_VXLANCTRL_VXLAN_UDPPORT_MASK: c_uint = 0x0000ffff /* VXLAN port */;
pub const IXGBE_VXLANCTRL_GENEVE_UDPPORT_MASK: c_uint = 0xffff0000 /* GENEVE port */;
pub const IXGBE_VXLANCTRL_ALL_UDPPORT_MASK: c_uint = 0xffffffff /* GENEVE/VXLAN */;
pub const IXGBE_VXLANCTRL_GENEVE_UDPPORT_SHIFT: c_int = 16;
pub const IXGBE_FLEXIBLE_FILTER_COUNT_MAX: c_int = 4;
pub const IXGBE_EXT_FLEXIBLE_FILTER_COUNT_MAX: c_int = 2;
// Each Flexible Filter is at most 128 (0x80) bytes in length
pub const IXGBE_FLEXIBLE_FILTER_SIZE_MAX: c_int = 128;
pub const IXGBE_FHFT_LENGTH_OFFSET: c_uint = 0xFC  /* Length byte in FHFT */;
pub const IXGBE_FHFT_LENGTH_MASK: c_uint = 0x0FF /* Length in lower byte */;
// Definitions for power management and wakeup registers
// Wake Up Control
pub const IXGBE_WUC_PME_EN: c_uint = 0x00000002 /* PME Enable */;
pub const IXGBE_WUC_PME_STATUS: c_uint = 0x00000004 /* PME Status */;
pub const IXGBE_WUC_WKEN: c_uint = 0x00000010 /* Enable PE_WAKE_N pin assertion  */;
// Wake Up Filter Control
pub const IXGBE_WUFC_LNKC: c_uint = 0x00000001 /* Link Status Change Wakeup Enable */;
pub const IXGBE_WUFC_MAG: c_uint = 0x00000002 /* Magic Packet Wakeup Enable */;
pub const IXGBE_WUFC_EX: c_uint = 0x00000004 /* Directed Exact Wakeup Enable */;
pub const IXGBE_WUFC_MC: c_uint = 0x00000008 /* Directed Multicast Wakeup Enable */;
pub const IXGBE_WUFC_BC: c_uint = 0x00000010 /* Broadcast Wakeup Enable */;
pub const IXGBE_WUFC_ARP: c_uint = 0x00000020 /* ARP Request Packet Wakeup Enable */;
pub const IXGBE_WUFC_IPV4: c_uint = 0x00000040 /* Directed IPv4 Packet Wakeup Enable */;
pub const IXGBE_WUFC_IPV6: c_uint = 0x00000080 /* Directed IPv6 Packet Wakeup Enable */;
pub const IXGBE_WUFC_MNG: c_uint = 0x00000100 /* Directed Mgmt Packet Wakeup Enable */;
pub const IXGBE_WUFC_IGNORE_TCO: c_uint = 0x00008000 /* Ignore WakeOn TCO packets */;
pub const IXGBE_WUFC_FLX0: c_uint = 0x00010000 /* Flexible Filter 0 Enable */;
pub const IXGBE_WUFC_FLX1: c_uint = 0x00020000 /* Flexible Filter 1 Enable */;
pub const IXGBE_WUFC_FLX2: c_uint = 0x00040000 /* Flexible Filter 2 Enable */;
pub const IXGBE_WUFC_FLX3: c_uint = 0x00080000 /* Flexible Filter 3 Enable */;
pub const IXGBE_WUFC_FLX4: c_uint = 0x00100000 /* Flexible Filter 4 Enable */;
pub const IXGBE_WUFC_FLX5: c_uint = 0x00200000 /* Flexible Filter 5 Enable */;
pub const IXGBE_WUFC_FLX_FILTERS: c_uint = 0x000F0000 /* Mask for 4 flex filters */;
pub const IXGBE_WUFC_EXT_FLX_FILTERS: c_uint = 0x00300000 /* Mask for Ext. flex filters */;
pub const IXGBE_WUFC_ALL_FILTERS: c_uint = 0x003F00FF /* Mask for all wakeup filters */;

// Wake Up Status

// Wake Up Packet Length
pub const IXGBE_WUPL_LENGTH_MASK: c_uint = 0xFFFF;
// DCB registers
pub const MAX_TRAFFIC_CLASS: c_int = 8;
pub const X540_TRAFFIC_CLASS: c_int = 4;
pub const DEF_TRAFFIC_CLASS: c_int = 1;
pub const IXGBE_RMCS: c_uint = 0x03D00;
pub const IXGBE_DPMCS: c_uint = 0x07F40;
pub const IXGBE_PDPMCS: c_uint = 0x0CD00;
pub const IXGBE_RUPPBMR: c_uint = 0x050A0;

// Security Control Registers
pub const IXGBE_SECTXCTRL: c_uint = 0x08800;
pub const IXGBE_SECTXSTAT: c_uint = 0x08804;
pub const IXGBE_SECTXBUFFAF: c_uint = 0x08808;
pub const IXGBE_SECTXMINIFG: c_uint = 0x08810;
pub const IXGBE_SECRXCTRL: c_uint = 0x08D00;
pub const IXGBE_SECRXSTAT: c_uint = 0x08D04;
// Security Bit Fields and Masks
pub const IXGBE_SECTXCTRL_SECTX_DIS: c_uint = 0x00000001;
pub const IXGBE_SECTXCTRL_TX_DIS: c_uint = 0x00000002;
pub const IXGBE_SECTXCTRL_STORE_FORWARD: c_uint = 0x00000004;
pub const IXGBE_SECTXSTAT_SECTX_RDY: c_uint = 0x00000001;
pub const IXGBE_SECTXSTAT_SECTX_OFF_DIS: c_uint = 0x00000002;
pub const IXGBE_SECTXSTAT_ECC_TXERR: c_uint = 0x00000004;
pub const IXGBE_SECRXCTRL_SECRX_DIS: c_uint = 0x00000001;
pub const IXGBE_SECRXCTRL_RX_DIS: c_uint = 0x00000002;
pub const IXGBE_SECRXSTAT_SECRX_RDY: c_uint = 0x00000001;
pub const IXGBE_SECRXSTAT_SECRX_OFF_DIS: c_uint = 0x00000002;
pub const IXGBE_SECRXSTAT_ECC_RXERR: c_uint = 0x00000004;
// LinkSec (MacSec) Registers
pub const IXGBE_LSECTXCAP: c_uint = 0x08A00;
pub const IXGBE_LSECRXCAP: c_uint = 0x08F00;
pub const IXGBE_LSECTXCTRL: c_uint = 0x08A04;
pub const IXGBE_LSECTXSCL: c_uint = 0x08A08 /* SCI Low */;
pub const IXGBE_LSECTXSCH: c_uint = 0x08A0C /* SCI High */;
pub const IXGBE_LSECTXSA: c_uint = 0x08A10;
pub const IXGBE_LSECTXPN0: c_uint = 0x08A14;
pub const IXGBE_LSECTXPN1: c_uint = 0x08A18;

pub const IXGBE_LSECRXCTRL: c_uint = 0x08F04;
pub const IXGBE_LSECRXSCL: c_uint = 0x08F08;
pub const IXGBE_LSECRXSCH: c_uint = 0x08F0C;

pub const IXGBE_LSECTXUT: c_uint = 0x08A3C /* OutPktsUntagged */;
pub const IXGBE_LSECTXPKTE: c_uint = 0x08A40 /* OutPktsEncrypted */;
pub const IXGBE_LSECTXPKTP: c_uint = 0x08A44 /* OutPktsProtected */;
pub const IXGBE_LSECTXOCTE: c_uint = 0x08A48 /* OutOctetsEncrypted */;
pub const IXGBE_LSECTXOCTP: c_uint = 0x08A4C /* OutOctetsProtected */;
pub const IXGBE_LSECRXUT: c_uint = 0x08F40 /* InPktsUntagged/InPktsNoTag */;
pub const IXGBE_LSECRXOCTD: c_uint = 0x08F44 /* InOctetsDecrypted */;
pub const IXGBE_LSECRXOCTV: c_uint = 0x08F48 /* InOctetsValidated */;
pub const IXGBE_LSECRXBAD: c_uint = 0x08F4C /* InPktsBadTag */;
pub const IXGBE_LSECRXNOSCI: c_uint = 0x08F50 /* InPktsNoSci */;
pub const IXGBE_LSECRXUNSCI: c_uint = 0x08F54 /* InPktsUnknownSci */;
pub const IXGBE_LSECRXUNCH: c_uint = 0x08F58 /* InPktsUnchecked */;
pub const IXGBE_LSECRXDELAY: c_uint = 0x08F5C /* InPktsDelayed */;
pub const IXGBE_LSECRXLATE: c_uint = 0x08F60 /* InPktsLate */;

pub const IXGBE_LSECRXUNSA: c_uint = 0x08F7C /* InPktsUnusedSa */;
pub const IXGBE_LSECRXNUSA: c_uint = 0x08F80 /* InPktsNotUsingSa */;
// LinkSec (MacSec) Bit Fields and Masks
pub const IXGBE_LSECTXCAP_SUM_MASK: c_uint = 0x00FF0000;
pub const IXGBE_LSECTXCAP_SUM_SHIFT: c_int = 16;
pub const IXGBE_LSECRXCAP_SUM_MASK: c_uint = 0x00FF0000;
pub const IXGBE_LSECRXCAP_SUM_SHIFT: c_int = 16;
pub const IXGBE_LSECTXCTRL_EN_MASK: c_uint = 0x00000003;
pub const IXGBE_LSECTXCTRL_DISABLE: c_uint = 0x0;
pub const IXGBE_LSECTXCTRL_AUTH: c_uint = 0x1;
pub const IXGBE_LSECTXCTRL_AUTH_ENCRYPT: c_uint = 0x2;
pub const IXGBE_LSECTXCTRL_AISCI: c_uint = 0x00000020;
pub const IXGBE_LSECTXCTRL_PNTHRSH_MASK: c_uint = 0xFFFFFF00;
pub const IXGBE_LSECTXCTRL_RSV_MASK: c_uint = 0x000000D8;
pub const IXGBE_LSECRXCTRL_EN_MASK: c_uint = 0x0000000C;
pub const IXGBE_LSECRXCTRL_EN_SHIFT: c_int = 2;
pub const IXGBE_LSECRXCTRL_DISABLE: c_uint = 0x0;
pub const IXGBE_LSECRXCTRL_CHECK: c_uint = 0x1;
pub const IXGBE_LSECRXCTRL_STRICT: c_uint = 0x2;
pub const IXGBE_LSECRXCTRL_DROP: c_uint = 0x3;
pub const IXGBE_LSECRXCTRL_PLSH: c_uint = 0x00000040;
pub const IXGBE_LSECRXCTRL_RP: c_uint = 0x00000080;
pub const IXGBE_LSECRXCTRL_RSV_MASK: c_uint = 0xFFFFFF33;
// IpSec Registers
pub const IXGBE_IPSTXIDX: c_uint = 0x08900;
pub const IXGBE_IPSTXSALT: c_uint = 0x08904;

pub const IXGBE_IPSRXIDX: c_uint = 0x08E00;

pub const IXGBE_IPSRXSPI: c_uint = 0x08E14;
pub const IXGBE_IPSRXIPIDX: c_uint = 0x08E18;

pub const IXGBE_IPSRXSALT: c_uint = 0x08E2C;
pub const IXGBE_IPSRXMOD: c_uint = 0x08E30;
pub const IXGBE_SECTXCTRL_STORE_FORWARD_ENABLE: c_uint = 0x4;
// DCB registers
pub const IXGBE_RTRPCS: c_uint = 0x02430;
pub const IXGBE_RTTDCS: c_uint = 0x04900;
pub const IXGBE_RTTDCS_ARBDIS: c_uint = 0x00000040 /* DCB arbiter disable */;
pub const IXGBE_RTTPCS: c_uint = 0x0CD00;
pub const IXGBE_RTRUP2TC: c_uint = 0x03020;
pub const IXGBE_RTTUP2TC: c_uint = 0x0C800;

pub const IXGBE_RTTDQSEL: c_uint = 0x04904;
pub const IXGBE_RTTDT1C: c_uint = 0x04908;
pub const IXGBE_RTTDT1S: c_uint = 0x0490C;
pub const IXGBE_RTTQCNCR: c_uint = 0x08B00;
pub const IXGBE_RTTQCNTG: c_uint = 0x04A90;
pub const IXGBE_RTTBCNRD: c_uint = 0x0498C;
pub const IXGBE_RTTQCNRR: c_uint = 0x0498C;
pub const IXGBE_RTTDTECC: c_uint = 0x04990;
pub const IXGBE_RTTDTECC_NO_BCN: c_uint = 0x00000100;
pub const IXGBE_RTTBCNRC: c_uint = 0x04984;
pub const IXGBE_RTTBCNRC_RS_ENA: c_uint = 0x80000000;
pub const IXGBE_RTTBCNRC_RF_DEC_MASK: c_uint = 0x00003FFF;
pub const IXGBE_RTTBCNRC_RF_INT_SHIFT: c_int = 14;

pub const IXGBE_RTTBCNRM: c_uint = 0x04980;
pub const IXGBE_RTTQCNRM: c_uint = 0x04980;
// FCoE Direct DMA Context

// FCoE DMA Context Registers
pub const IXGBE_FCPTRL: c_uint = 0x02410 /* FC User Desc. PTR Low */;
pub const IXGBE_FCPTRH: c_uint = 0x02414 /* FC USer Desc. PTR High */;
pub const IXGBE_FCBUFF: c_uint = 0x02418 /* FC Buffer Control */;
pub const IXGBE_FCDMARW: c_uint = 0x02420 /* FC Receive DMA RW */;
pub const IXGBE_FCINVST0: c_uint = 0x03FC0 /* FC Invalid DMA Context Status Reg 0 */;

pub const IXGBE_FCBUFF_BUFFCNT: c_uint = 0x0000ff00 /* Number of User Buffers */;
pub const IXGBE_FCBUFF_OFFSET: c_uint = 0xffff0000 /* User Buffer Offset */;
pub const IXGBE_FCBUFF_BUFFSIZE_SHIFT: c_int = 3;
pub const IXGBE_FCBUFF_BUFFCNT_SHIFT: c_int = 8;
pub const IXGBE_FCBUFF_OFFSET_SHIFT: c_int = 16;

pub const IXGBE_FCDMARW_FCOESEL: c_uint = 0x000001ff  /* FC X_ID: 11 bits */;
pub const IXGBE_FCDMARW_LASTSIZE: c_uint = 0xffff0000  /* Last User Buffer Size */;
pub const IXGBE_FCDMARW_LASTSIZE_SHIFT: c_int = 16;
// FCoE SOF/EOF
pub const IXGBE_TEOFF: c_uint = 0x04A94 /* Tx FC EOF */;
pub const IXGBE_TSOFF: c_uint = 0x04A98 /* Tx FC SOF */;
pub const IXGBE_REOFF: c_uint = 0x05158 /* Rx FC EOF */;
pub const IXGBE_RSOFF: c_uint = 0x051F8 /* Rx FC SOF */;
// FCoE Direct Filter Context

// FCoE Filter Context Registers
pub const IXGBE_FCFLT: c_uint = 0x05108 /* FC FLT Context */;
pub const IXGBE_FCFLTRW: c_uint = 0x05110 /* FC Filter RW Control */;
pub const IXGBE_FCPARAM: c_uint = 0x051d8 /* FC Offset Parameter */;

pub const IXGBE_FCFLT_SEQID: c_uint = 0x00ff0000 /* Sequence ID */;
pub const IXGBE_FCFLT_SEQCNT: c_uint = 0xff000000 /* Sequence Count */;

// FCoE Receive Control
pub const IXGBE_FCRXCTRL: c_uint = 0x05100 /* FC Receive Control */;

pub const IXGBE_FCRXCTRL_FCOEVER: c_uint = 0x00000f00 /* FCoE Version: 4 bits */;
pub const IXGBE_FCRXCTRL_FCOEVER_SHIFT: c_int = 8;
// FCoE Redirection
pub const IXGBE_FCRECTL: c_uint = 0x0ED00 /* FC Redirection Control */;
pub const IXGBE_FCRETA0: c_uint = 0x0ED10 /* FC Redirection Table 0 */;

pub const IXGBE_FCRECTL_ENA: c_uint = 0x1        /* FCoE Redir Table Enable */;

pub const IXGBE_FCRETA_ENTRY_MASK: c_uint = 0x0000007f /* 7 bits for the queue index */;

// Higher 7 bits for the queue index
pub const IXGBE_FCRETA_ENTRY_HIGH_MASK: c_uint = 0x007F0000;
pub const IXGBE_FCRETA_ENTRY_HIGH_SHIFT: c_int = 16;
// Stats registers
pub const IXGBE_CRCERRS: c_uint = 0x04000;
pub const IXGBE_ILLERRC: c_uint = 0x04004;
pub const IXGBE_ERRBC: c_uint = 0x04008;
pub const IXGBE_MSPDC: c_uint = 0x04010;

pub const IXGBE_MLFC: c_uint = 0x04034;
pub const IXGBE_MRFC: c_uint = 0x04038;
pub const IXGBE_RLEC: c_uint = 0x04040;
pub const IXGBE_LXONTXC: c_uint = 0x03F60;
pub const IXGBE_LXONRXC: c_uint = 0x0CF60;
pub const IXGBE_LXOFFTXC: c_uint = 0x03F68;
pub const IXGBE_LXOFFRXC: c_uint = 0x0CF68;
pub const IXGBE_LXONRXCNT: c_uint = 0x041A4;
pub const IXGBE_LXOFFRXCNT: c_uint = 0x041A8;

pub const IXGBE_PRC64: c_uint = 0x0405C;
pub const IXGBE_PRC127: c_uint = 0x04060;
pub const IXGBE_PRC255: c_uint = 0x04064;
pub const IXGBE_PRC511: c_uint = 0x04068;
pub const IXGBE_PRC1023: c_uint = 0x0406C;
pub const IXGBE_PRC1522: c_uint = 0x04070;
pub const IXGBE_GPRC: c_uint = 0x04074;
pub const IXGBE_BPRC: c_uint = 0x04078;
pub const IXGBE_MPRC: c_uint = 0x0407C;
pub const IXGBE_GPTC: c_uint = 0x04080;
pub const IXGBE_GORCL: c_uint = 0x04088;
pub const IXGBE_GORCH: c_uint = 0x0408C;
pub const IXGBE_GOTCL: c_uint = 0x04090;
pub const IXGBE_GOTCH: c_uint = 0x04094;

pub const IXGBE_RUC: c_uint = 0x040A4;
pub const IXGBE_RFC: c_uint = 0x040A8;
pub const IXGBE_ROC: c_uint = 0x040AC;
pub const IXGBE_RJC: c_uint = 0x040B0;
pub const IXGBE_MNGPRC: c_uint = 0x040B4;
pub const IXGBE_MNGPDC: c_uint = 0x040B8;
pub const IXGBE_MNGPTC: c_uint = 0x0CF90;
pub const IXGBE_TORL: c_uint = 0x040C0;
pub const IXGBE_TORH: c_uint = 0x040C4;
pub const IXGBE_TPR: c_uint = 0x040D0;
pub const IXGBE_TPT: c_uint = 0x040D4;
pub const IXGBE_PTC64: c_uint = 0x040D8;
pub const IXGBE_PTC127: c_uint = 0x040DC;
pub const IXGBE_PTC255: c_uint = 0x040E0;
pub const IXGBE_PTC511: c_uint = 0x040E4;
pub const IXGBE_PTC1023: c_uint = 0x040E8;
pub const IXGBE_PTC1522: c_uint = 0x040EC;
pub const IXGBE_MPTC: c_uint = 0x040F0;
pub const IXGBE_BPTC: c_uint = 0x040F4;
pub const IXGBE_XEC: c_uint = 0x04120;
pub const IXGBE_SSVPC: c_uint = 0x08780;

pub const IXGBE_FCCRC: c_uint = 0x05118 /* Count of Good Eth CRC w/ Bad FC CRC */;
pub const IXGBE_FCOERPDC: c_uint = 0x0241C /* FCoE Rx Packets Dropped Count */;
pub const IXGBE_FCLAST: c_uint = 0x02424 /* FCoE Last Error Count */;
pub const IXGBE_FCOEPRC: c_uint = 0x02428 /* Number of FCoE Packets Received */;
pub const IXGBE_FCOEDWRC: c_uint = 0x0242C /* Number of FCoE DWords Received */;
pub const IXGBE_FCOEPTC: c_uint = 0x08784 /* Number of FCoE Packets Transmitted */;
pub const IXGBE_FCOEDWTC: c_uint = 0x08788 /* Number of FCoE DWords Transmitted */;
pub const IXGBE_O2BGPTC: c_uint = 0x041C4;
pub const IXGBE_O2BSPC: c_uint = 0x087B0;
pub const IXGBE_B2OSPC: c_uint = 0x041C0;
pub const IXGBE_B2OGPRC: c_uint = 0x02F90;
pub const IXGBE_PCRC8ECL: c_uint = 0x0E810;
pub const IXGBE_PCRC8ECH: c_uint = 0x0E811;
pub const IXGBE_PCRC8ECH_MASK: c_uint = 0x1F;
pub const IXGBE_LDPCECL: c_uint = 0x0E820;
pub const IXGBE_LDPCECH: c_uint = 0x0E821;
// MII clause 22/28 definitions
pub const IXGBE_MDIO_PHY_LOW_POWER_MODE: c_uint = 0x0800;
pub const IXGBE_MDIO_XENPAK_LASI_STATUS: c_uint = 0x9005 /* XENPAK LASI Status register */;
pub const IXGBE_XENPAK_LASI_LINK_STATUS_ALARM: c_uint = 0x1 /* Link Status Alarm change */;
pub const IXGBE_MDIO_AUTO_NEG_LINK_STATUS: c_uint = 0x4 /* Indicates if link is up */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_MASK: c_uint = 0x7 /* Speed/Duplex Mask */;
pub const IXGBE_MDIO_AUTO_NEG_VEN_STAT_SPEED_MASK: c_uint = 0x6 /* Speed Mask */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10M_HALF: c_uint = 0x0 /* 10Mb/s Half Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10M_FULL: c_uint = 0x1 /* 10Mb/s Full Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_100M_HALF: c_uint = 0x2 /* 100Mb/s H Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_100M_FULL: c_uint = 0x3 /* 100Mb/s F Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_1GB_HALF: c_uint = 0x4 /* 1Gb/s Half Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_1GB_FULL: c_uint = 0x5 /* 1Gb/s Full Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10GB_HALF: c_uint = 0x6 /* 10Gb/s Half Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10GB_FULL: c_uint = 0x7 /* 10Gb/s Full Duplex */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_1GB: c_uint = 0x4 /* 1Gb/s */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10GB: c_uint = 0x6 /* 10Gb/s */;
pub const IXGBE_MII_AUTONEG_VENDOR_PROVISION_1_REG: c_uint = 0xC400	/* 1G Provisioning 1 */;
pub const IXGBE_MII_AUTONEG_XNP_TX_REG: c_uint = 0x17	/* 1G XNP Transmit */;
pub const IXGBE_MII_1GBASE_T_ADVERTISE_XNP_TX: c_uint = 0x4000	/* full duplex, bit:14*/;
pub const IXGBE_MII_1GBASE_T_ADVERTISE: c_uint = 0x8000	/* full duplex, bit:15*/;
pub const IXGBE_MII_2_5GBASE_T_ADVERTISE: c_uint = 0x0400;
pub const IXGBE_MII_5GBASE_T_ADVERTISE: c_uint = 0x0800;
pub const IXGBE_MII_RESTART: c_uint = 0x200;
pub const IXGBE_MII_AUTONEG_LINK_UP: c_uint = 0x04;
pub const IXGBE_MII_AUTONEG_REG: c_uint = 0x0;
// Management

pub const IXGBE_MANC: c_uint = 0x05820;
pub const IXGBE_MFVAL: c_uint = 0x05824;
pub const IXGBE_MANC2H: c_uint = 0x05860;

pub const IXGBE_MIPAF: c_uint = 0x058B0;

pub const IXGBE_FTFT: c_uint = 0x09400 /* 0x9400-0x97FC */;

pub const IXGBE_LSWFW: c_uint = 0x15014;
// Management Bit Fields and Masks
pub const IXGBE_MANC_RCV_TCO_EN: c_uint = 0x00020000 /* Rcv TCO packet enable */;
// Firmware Semaphore Register
pub const IXGBE_FWSM_MODE_MASK: c_uint = 0xE;
pub const IXGBE_FWSM_FW_MODE_PT: c_uint = 0x4;

pub const IXGBE_FWSM_EXT_ERR_IND_MASK: c_uint = 0x01F80000;

// ARC Subsystem registers
pub const IXGBE_HICR: c_uint = 0x15F00;
pub const IXGBE_FWSTS: c_uint = 0x15F0C;
pub const IXGBE_HSMC0R: c_uint = 0x15F04;
pub const IXGBE_HSMC1R: c_uint = 0x15F08;
pub const IXGBE_SWSR: c_uint = 0x15F10;
pub const IXGBE_HFDR: c_uint = 0x15FE8;
pub const IXGBE_FLEX_MNG: c_uint = 0x15800 /* 0x15800 - 0x15EFC */;
pub const IXGBE_HICR_EN: c_uint = 0x01  /* Enable bit - RO */;
// Driver sets this bit when done to put command in RAM
pub const IXGBE_HICR_C: c_uint = 0x02;
pub const IXGBE_HICR_SV: c_uint = 0x04  /* Status Validity */;
pub const IXGBE_HICR_FW_RESET_ENABLE: c_uint = 0x40;
pub const IXGBE_HICR_FW_RESET: c_uint = 0x80;
// PCI-E registers
pub const IXGBE_GCR: c_uint = 0x11000;
pub const IXGBE_GTV: c_uint = 0x11004;
pub const IXGBE_FUNCTAG: c_uint = 0x11008;
pub const IXGBE_GLT: c_uint = 0x1100C;
pub const IXGBE_GSCL_1: c_uint = 0x11010;
pub const IXGBE_GSCL_2: c_uint = 0x11014;
pub const IXGBE_GSCL_3: c_uint = 0x11018;
pub const IXGBE_GSCL_4: c_uint = 0x1101C;
pub const IXGBE_GSCN_0: c_uint = 0x11020;
pub const IXGBE_GSCN_1: c_uint = 0x11024;
pub const IXGBE_GSCN_2: c_uint = 0x11028;
pub const IXGBE_GSCN_3: c_uint = 0x1102C;
pub const IXGBE_FACTPS_8259X: c_uint = 0x10150;

pub const IXGBE_FACTPS_X550EM_a: c_uint = 0x15FEC;

pub const IXGBE_PCIEANACTL: c_uint = 0x11040;
pub const IXGBE_SWSM_8259X: c_uint = 0x10140;

pub const IXGBE_SWSM_X550EM_a: c_uint = 0x15F70;

pub const IXGBE_FWSM_8259X: c_uint = 0x10148;

pub const IXGBE_FWSM_X550EM_a: c_uint = 0x15F74;

pub const IXGBE_GSSR: c_uint = 0x10160;
pub const IXGBE_MREVID: c_uint = 0x11064;
pub const IXGBE_DCA_ID: c_uint = 0x11070;
pub const IXGBE_DCA_CTRL: c_uint = 0x11074;

pub const IXGBE_SWFW_SYNC_X550EM_a: c_uint = 0x15F78;

// PCIe registers 82599-specific
pub const IXGBE_GCR_EXT: c_uint = 0x11050;
pub const IXGBE_GSCL_5_82599: c_uint = 0x11030;
pub const IXGBE_GSCL_6_82599: c_uint = 0x11034;
pub const IXGBE_GSCL_7_82599: c_uint = 0x11038;
pub const IXGBE_GSCL_8_82599: c_uint = 0x1103C;
pub const IXGBE_PHYADR_82599: c_uint = 0x11040;
pub const IXGBE_PHYDAT_82599: c_uint = 0x11044;
pub const IXGBE_PHYCTL_82599: c_uint = 0x11048;
pub const IXGBE_PBACLR_82599: c_uint = 0x11068;
pub const IXGBE_CIAA_8259X: c_uint = 0x11088;

pub const IXGBE_CIAA_X550: c_uint = 0x11508;

pub const IXGBE_CIAD_8259X: c_uint = 0x1108C;

pub const IXGBE_CIAD_X550: c_uint = 0x11510;

pub const IXGBE_PICAUSE: c_uint = 0x110B0;
pub const IXGBE_PIENA: c_uint = 0x110B8;
pub const IXGBE_CDQ_MBR_82599: c_uint = 0x110B4;
pub const IXGBE_PCIESPARE: c_uint = 0x110BC;
pub const IXGBE_MISC_REG_82599: c_uint = 0x110F0;
pub const IXGBE_ECC_CTRL_0_82599: c_uint = 0x11100;
pub const IXGBE_ECC_CTRL_1_82599: c_uint = 0x11104;
pub const IXGBE_ECC_STATUS_82599: c_uint = 0x110E0;
pub const IXGBE_BAR_CTRL_82599: c_uint = 0x110F4;
// PCI Express Control
pub const IXGBE_GCR_CMPL_TMOUT_MASK: c_uint = 0x0000F000;
pub const IXGBE_GCR_CMPL_TMOUT_10ms: c_uint = 0x00001000;
pub const IXGBE_GCR_CMPL_TMOUT_RESEND: c_uint = 0x00010000;
pub const IXGBE_GCR_CAP_VER2: c_uint = 0x00040000;
pub const IXGBE_GCR_EXT_MSIX_EN: c_uint = 0x80000000;
pub const IXGBE_GCR_EXT_BUFFERS_CLEAR: c_uint = 0x40000000;
pub const IXGBE_GCR_EXT_VT_MODE_16: c_uint = 0x00000001;
pub const IXGBE_GCR_EXT_VT_MODE_32: c_uint = 0x00000002;
pub const IXGBE_GCR_EXT_VT_MODE_64: c_uint = 0x00000003;
pub const IXGBE_GCR_EXT_VT_MODE_MASK: c_uint = 0x00000003;

// Time Sync Registers
pub const IXGBE_TSYNCRXCTL: c_uint = 0x05188 /* Rx Time Sync Control register - RW */;
pub const IXGBE_TSYNCTXCTL: c_uint = 0x08C00 /* Tx Time Sync Control register - RW */;
pub const IXGBE_RXSTMPL: c_uint = 0x051E8 /* Rx timestamp Low - RO */;
pub const IXGBE_RXSTMPH: c_uint = 0x051A4 /* Rx timestamp High - RO */;
pub const IXGBE_RXSATRL: c_uint = 0x051A0 /* Rx timestamp attribute low - RO */;
pub const IXGBE_RXSATRH: c_uint = 0x051A8 /* Rx timestamp attribute high - RO */;
pub const IXGBE_RXMTRL: c_uint = 0x05120 /* RX message type register low - RW */;
pub const IXGBE_TXSTMPL: c_uint = 0x08C04 /* Tx timestamp value Low - RO */;
pub const IXGBE_TXSTMPH: c_uint = 0x08C08 /* Tx timestamp value High - RO */;
pub const IXGBE_SYSTIML: c_uint = 0x08C0C /* System time register Low - RO */;
pub const IXGBE_SYSTIMH: c_uint = 0x08C10 /* System time register High - RO */;
pub const IXGBE_SYSTIMR: c_uint = 0x08C58 /* System time register Residue - RO */;
pub const IXGBE_TIMINCA: c_uint = 0x08C14 /* Increment attributes register - RW */;
pub const IXGBE_TIMADJL: c_uint = 0x08C18 /* Time Adjustment Offset register Low - RW */;
pub const IXGBE_TIMADJH: c_uint = 0x08C1C /* Time Adjustment Offset register High - RW */;
pub const IXGBE_TSAUXC: c_uint = 0x08C20 /* TimeSync Auxiliary Control register - RW */;
pub const IXGBE_TRGTTIML0: c_uint = 0x08C24 /* Target Time Register 0 Low - RW */;
pub const IXGBE_TRGTTIMH0: c_uint = 0x08C28 /* Target Time Register 0 High - RW */;
pub const IXGBE_TRGTTIML1: c_uint = 0x08C2C /* Target Time Register 1 Low - RW */;
pub const IXGBE_TRGTTIMH1: c_uint = 0x08C30 /* Target Time Register 1 High - RW */;
pub const IXGBE_CLKTIML: c_uint = 0x08C34 /* Clock Out Time Register Low - RW */;
pub const IXGBE_CLKTIMH: c_uint = 0x08C38 /* Clock Out Time Register High - RW */;
pub const IXGBE_FREQOUT0: c_uint = 0x08C34 /* Frequency Out 0 Control register - RW */;
pub const IXGBE_FREQOUT1: c_uint = 0x08C38 /* Frequency Out 1 Control register - RW */;
pub const IXGBE_AUXSTMPL0: c_uint = 0x08C3C /* Auxiliary Time Stamp 0 register Low - RO */;
pub const IXGBE_AUXSTMPH0: c_uint = 0x08C40 /* Auxiliary Time Stamp 0 register High - RO */;
pub const IXGBE_AUXSTMPL1: c_uint = 0x08C44 /* Auxiliary Time Stamp 1 register Low - RO */;
pub const IXGBE_AUXSTMPH1: c_uint = 0x08C48 /* Auxiliary Time Stamp 1 register High - RO */;
pub const IXGBE_TSIM: c_uint = 0x08C68 /* TimeSync Interrupt Mask Register - RW */;
pub const IXGBE_TSSDP: c_uint = 0x0003C /* TimeSync SDP Configuration Register - RW */;
// Diagnostic Registers
pub const IXGBE_RDSTATCTL: c_uint = 0x02C20;

pub const IXGBE_RDHMPN: c_uint = 0x02F08;

pub const IXGBE_RDPROBE: c_uint = 0x02F20;
pub const IXGBE_RDMAM: c_uint = 0x02F30;
pub const IXGBE_RDMAD: c_uint = 0x02F34;
pub const IXGBE_TDSTATCTL: c_uint = 0x07C20;

pub const IXGBE_TDHMPN: c_uint = 0x07F08;
pub const IXGBE_TDHMPN2: c_uint = 0x082FC;
pub const IXGBE_TXDESCIC: c_uint = 0x082CC;

pub const IXGBE_TDPROBE: c_uint = 0x07F20;
pub const IXGBE_TXBUFCTRL: c_uint = 0x0C600;

pub const IXGBE_RXBUFCTRL: c_uint = 0x03600;

pub const IXGBE_RFVAL: c_uint = 0x050A4;
pub const IXGBE_MDFTC1: c_uint = 0x042B8;
pub const IXGBE_MDFTC2: c_uint = 0x042C0;
pub const IXGBE_MDFTFIFO1: c_uint = 0x042C4;
pub const IXGBE_MDFTFIFO2: c_uint = 0x042C8;
pub const IXGBE_MDFTS: c_uint = 0x042CC;

pub const IXGBE_PCIEECCCTL: c_uint = 0x1106C;

pub const IXGBE_PCIEECCCTL0: c_uint = 0x11100;
pub const IXGBE_PCIEECCCTL1: c_uint = 0x11104;
pub const IXGBE_RXDBUECC: c_uint = 0x03F70;
pub const IXGBE_TXDBUECC: c_uint = 0x0CF70;
pub const IXGBE_RXDBUEST: c_uint = 0x03F74;
pub const IXGBE_TXDBUEST: c_uint = 0x0CF74;
pub const IXGBE_PBTXECC: c_uint = 0x0C300;
pub const IXGBE_PBRXECC: c_uint = 0x03300;
pub const IXGBE_GHECCR: c_uint = 0x110B0;
// MAC Registers
pub const IXGBE_PCS1GCFIG: c_uint = 0x04200;
pub const IXGBE_PCS1GLCTL: c_uint = 0x04208;
pub const IXGBE_PCS1GLSTA: c_uint = 0x0420C;
pub const IXGBE_PCS1GDBG0: c_uint = 0x04210;
pub const IXGBE_PCS1GDBG1: c_uint = 0x04214;
pub const IXGBE_PCS1GANA: c_uint = 0x04218;
pub const IXGBE_PCS1GANLP: c_uint = 0x0421C;
pub const IXGBE_PCS1GANNP: c_uint = 0x04220;
pub const IXGBE_PCS1GANLPNP: c_uint = 0x04224;
pub const IXGBE_HLREG0: c_uint = 0x04240;
pub const IXGBE_HLREG1: c_uint = 0x04244;
pub const IXGBE_PAP: c_uint = 0x04248;
pub const IXGBE_MACA: c_uint = 0x0424C;
pub const IXGBE_APAE: c_uint = 0x04250;
pub const IXGBE_ARD: c_uint = 0x04254;
pub const IXGBE_AIS: c_uint = 0x04258;
pub const IXGBE_MSCA: c_uint = 0x0425C;
pub const IXGBE_MSRWD: c_uint = 0x04260;
pub const IXGBE_MLADD: c_uint = 0x04264;
pub const IXGBE_MHADD: c_uint = 0x04268;
pub const IXGBE_MAXFRS: c_uint = 0x04268;
pub const IXGBE_TREG: c_uint = 0x0426C;
pub const IXGBE_PCSS1: c_uint = 0x04288;
pub const IXGBE_PCSS2: c_uint = 0x0428C;
pub const IXGBE_XPCSS: c_uint = 0x04290;
pub const IXGBE_MFLCN: c_uint = 0x04294;
pub const IXGBE_SERDESC: c_uint = 0x04298;
pub const IXGBE_MAC_SGMII_BUSY: c_uint = 0x04298;
pub const IXGBE_MACS: c_uint = 0x0429C;
pub const IXGBE_AUTOC: c_uint = 0x042A0;
pub const IXGBE_LINKS: c_uint = 0x042A4;
pub const IXGBE_LINKS2: c_uint = 0x04324;
pub const IXGBE_AUTOC2: c_uint = 0x042A8;
pub const IXGBE_AUTOC3: c_uint = 0x042AC;
pub const IXGBE_ANLP1: c_uint = 0x042B0;
pub const IXGBE_ANLP2: c_uint = 0x042B4;
pub const IXGBE_MACC: c_uint = 0x04330;
pub const IXGBE_ATLASCTL: c_uint = 0x04800;
pub const IXGBE_MMNGC: c_uint = 0x042D0;
pub const IXGBE_ANLPNP1: c_uint = 0x042D4;
pub const IXGBE_ANLPNP2: c_uint = 0x042D8;
pub const IXGBE_KRPCSFC: c_uint = 0x042E0;
pub const IXGBE_KRPCSS: c_uint = 0x042E4;
pub const IXGBE_FECS1: c_uint = 0x042E8;
pub const IXGBE_FECS2: c_uint = 0x042EC;
pub const IXGBE_SMADARCTL: c_uint = 0x14F10;
pub const IXGBE_MPVC: c_uint = 0x04318;
pub const IXGBE_SGMIIC: c_uint = 0x04314;
// Statistics Registers
pub const IXGBE_RXNFGPC: c_uint = 0x041B0;
pub const IXGBE_RXNFGBCL: c_uint = 0x041B4;
pub const IXGBE_RXNFGBCH: c_uint = 0x041B8;
pub const IXGBE_RXDGPC: c_uint = 0x02F50;
pub const IXGBE_RXDGBCL: c_uint = 0x02F54;
pub const IXGBE_RXDGBCH: c_uint = 0x02F58;
pub const IXGBE_RXDDGPC: c_uint = 0x02F5C;
pub const IXGBE_RXDDGBCL: c_uint = 0x02F60;
pub const IXGBE_RXDDGBCH: c_uint = 0x02F64;
pub const IXGBE_RXLPBKGPC: c_uint = 0x02F68;
pub const IXGBE_RXLPBKGBCL: c_uint = 0x02F6C;
pub const IXGBE_RXLPBKGBCH: c_uint = 0x02F70;
pub const IXGBE_RXDLPBKGPC: c_uint = 0x02F74;
pub const IXGBE_RXDLPBKGBCL: c_uint = 0x02F78;
pub const IXGBE_RXDLPBKGBCH: c_uint = 0x02F7C;
pub const IXGBE_TXDGPC: c_uint = 0x087A0;
pub const IXGBE_TXDGBCL: c_uint = 0x087A4;
pub const IXGBE_TXDGBCH: c_uint = 0x087A8;
pub const IXGBE_RXDSTATCTRL: c_uint = 0x02F40;
// Copper Pond 2 link timeout
pub const IXGBE_VALIDATE_LINK_READY_TIMEOUT: c_int = 50;
// Omer CORECTL
pub const IXGBE_CORECTL: c_uint = 0x014F00;
// BARCTRL
pub const IXGBE_BARCTRL: c_uint = 0x110F4;
pub const IXGBE_BARCTRL_FLSIZE: c_uint = 0x0700;
pub const IXGBE_BARCTRL_FLSIZE_SHIFT: c_int = 8;
pub const IXGBE_BARCTRL_CSRSIZE: c_uint = 0x2000;
// RSCCTL Bit Masks
pub const IXGBE_RSCCTL_RSCEN: c_uint = 0x01;
pub const IXGBE_RSCCTL_MAXDESC_1: c_uint = 0x00;
pub const IXGBE_RSCCTL_MAXDESC_4: c_uint = 0x04;
pub const IXGBE_RSCCTL_MAXDESC_8: c_uint = 0x08;
pub const IXGBE_RSCCTL_MAXDESC_16: c_uint = 0x0C;
// RSCDBU Bit Masks
pub const IXGBE_RSCDBU_RSCSMALDIS_MASK: c_uint = 0x0000007F;
pub const IXGBE_RSCDBU_RSCACKDIS: c_uint = 0x00000080;
// RDRXCTL Bit Masks
pub const IXGBE_RDRXCTL_RDMTS_1_2: c_uint = 0x00000000 /* Rx Desc Min Threshold Size */;
pub const IXGBE_RDRXCTL_CRCSTRIP: c_uint = 0x00000002 /* CRC Strip */;
pub const IXGBE_RDRXCTL_PSP: c_uint = 0x00000004 /* Pad small packet */;
pub const IXGBE_RDRXCTL_MVMEN: c_uint = 0x00000020;
pub const IXGBE_RDRXCTL_DMAIDONE: c_uint = 0x00000008 /* DMA init cycle done */;
pub const IXGBE_RDRXCTL_AGGDIS: c_uint = 0x00010000 /* Aggregation disable */;
pub const IXGBE_RDRXCTL_RSCFRSTSIZE: c_uint = 0x003E0000 /* RSC First packet size */;
pub const IXGBE_RDRXCTL_RSCLLIDIS: c_uint = 0x00800000 /* Disable RSC compl on LLI */;
pub const IXGBE_RDRXCTL_RSCACKC: c_uint = 0x02000000 /* must set 1 when RSC enabled */;
pub const IXGBE_RDRXCTL_FCOE_WRFIX: c_uint = 0x04000000 /* must set 1 when RSC enabled */;
pub const IXGBE_RDRXCTL_MBINTEN: c_uint = 0x10000000;
pub const IXGBE_RDRXCTL_MDP_EN: c_uint = 0x20000000;
// RQTC Bit Masks and Shifts

// PSRTYPE.RQPL Bit masks and shift
pub const IXGBE_PSRTYPE_RQPL_MASK: c_uint = 0x7;
pub const IXGBE_PSRTYPE_RQPL_SHIFT: c_int = 29;
// CTRL Bit Masks
pub const IXGBE_CTRL_GIO_DIS: c_uint = 0x00000004 /* Global IO Primary Disable bit */;
pub const IXGBE_CTRL_LNK_RST: c_uint = 0x00000008 /* Link Reset. Resets everything. */;
pub const IXGBE_CTRL_RST: c_uint = 0x04000000 /* Reset (SW) */;

// FACTPS
pub const IXGBE_FACTPS_MNGCG: c_uint = 0x20000000 /* Manageblility Clock Gated */;
pub const IXGBE_FACTPS_LFS: c_uint = 0x40000000 /* LAN Function Select */;
// MHADD Bit Masks
pub const IXGBE_MHADD_MFS_MASK: c_uint = 0xFFFF0000;
pub const IXGBE_MHADD_MFS_SHIFT: c_int = 16;
// Extended Device Control
pub const IXGBE_CTRL_EXT_PFRSTD: c_uint = 0x00004000 /* Physical Function Reset Done */;
pub const IXGBE_CTRL_EXT_NS_DIS: c_uint = 0x00010000 /* No Snoop disable */;
pub const IXGBE_CTRL_EXT_RO_DIS: c_uint = 0x00020000 /* Relaxed Ordering disable */;
pub const IXGBE_CTRL_EXT_DRV_LOAD: c_uint = 0x10000000 /* Driver loaded bit for FW */;
// Direct Cache Access (DCA) definitions
pub const IXGBE_DCA_CTRL_DCA_ENABLE: c_uint = 0x00000000 /* DCA Enable */;
pub const IXGBE_DCA_CTRL_DCA_DISABLE: c_uint = 0x00000001 /* DCA Disable */;
pub const IXGBE_DCA_CTRL_DCA_MODE_CB1: c_uint = 0x00 /* DCA Mode CB1 */;
pub const IXGBE_DCA_CTRL_DCA_MODE_CB2: c_uint = 0x02 /* DCA Mode CB2 */;
pub const IXGBE_DCA_RXCTRL_CPUID_MASK: c_uint = 0x0000001F /* Rx CPUID Mask */;
pub const IXGBE_DCA_RXCTRL_CPUID_MASK_82599: c_uint = 0xFF000000 /* Rx CPUID Mask */;

pub const IXGBE_DCA_TXCTRL_CPUID_MASK: c_uint = 0x0000001F /* Tx CPUID Mask */;
pub const IXGBE_DCA_TXCTRL_CPUID_MASK_82599: c_uint = 0xFF000000 /* Tx CPUID Mask */;

// MSCA Bit Masks
pub const IXGBE_MSCA_NP_ADDR_MASK: c_uint = 0x0000FFFF /* MDI Address (new protocol) */;
pub const IXGBE_MSCA_NP_ADDR_SHIFT: c_int = 0;
pub const IXGBE_MSCA_DEV_TYPE_MASK: c_uint = 0x001F0000 /* Device Type (new protocol) */;

pub const IXGBE_MSCA_PHY_ADDR_MASK: c_uint = 0x03E00000 /* PHY Address mask */;

pub const IXGBE_MSCA_OP_CODE_MASK: c_uint = 0x0C000000 /* OP CODE mask */;

pub const IXGBE_MSCA_ADDR_CYCLE: c_uint = 0x00000000 /* OP CODE 00 (addr cycle) */;
pub const IXGBE_MSCA_WRITE: c_uint = 0x04000000 /* OP CODE 01 (write) */;
pub const IXGBE_MSCA_READ: c_uint = 0x0C000000 /* OP CODE 11 (read) */;
pub const IXGBE_MSCA_READ_AUTOINC: c_uint = 0x08000000 /* OP CODE 10 (read, auto inc)*/;
pub const IXGBE_MSCA_ST_CODE_MASK: c_uint = 0x30000000 /* ST Code mask */;

pub const IXGBE_MSCA_NEW_PROTOCOL: c_uint = 0x00000000 /* ST CODE 00 (new protocol) */;
pub const IXGBE_MSCA_OLD_PROTOCOL: c_uint = 0x10000000 /* ST CODE 01 (old protocol) */;
pub const IXGBE_MSCA_MDI_COMMAND: c_uint = 0x40000000 /* Initiate MDI command */;
pub const IXGBE_MSCA_MDI_IN_PROG_EN: c_uint = 0x80000000 /* MDI in progress enable */;
// MSRWD bit masks
pub const IXGBE_MSRWD_WRITE_DATA_MASK: c_uint = 0x0000FFFF;
pub const IXGBE_MSRWD_WRITE_DATA_SHIFT: c_int = 0;
pub const IXGBE_MSRWD_READ_DATA_MASK: c_uint = 0xFFFF0000;
pub const IXGBE_MSRWD_READ_DATA_SHIFT: c_int = 16;
// Atlas registers
pub const IXGBE_ATLAS_PDN_LPBK: c_uint = 0x24;
pub const IXGBE_ATLAS_PDN_10G: c_uint = 0xB;
pub const IXGBE_ATLAS_PDN_1G: c_uint = 0xC;
pub const IXGBE_ATLAS_PDN_AN: c_uint = 0xD;
// Atlas bit masks
pub const IXGBE_ATLASCTL_WRITE_CMD: c_uint = 0x00010000;
pub const IXGBE_ATLAS_PDN_TX_REG_EN: c_uint = 0x10;
pub const IXGBE_ATLAS_PDN_TX_10G_QL_ALL: c_uint = 0xF0;
pub const IXGBE_ATLAS_PDN_TX_1G_QL_ALL: c_uint = 0xF0;
pub const IXGBE_ATLAS_PDN_TX_AN_QL_ALL: c_uint = 0xF0;
// Omer bit masks
pub const IXGBE_CORECTL_WRITE_CMD: c_uint = 0x00010000;
// MDIO definitions
pub const IXGBE_MDIO_ZERO_DEV_TYPE: c_uint = 0x0;
pub const IXGBE_MDIO_PCS_DEV_TYPE: c_uint = 0x3;
pub const IXGBE_TWINAX_DEV: c_int = 1;

pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_LINK_STATUS: c_uint = 0x0008 /* 1 = Link Up */;
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_SPEED_STATUS: c_uint = 0x0010 /* 0 - 10G, 1 - 1G */;
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_10G_SPEED: c_uint = 0x0018;
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_1G_SPEED: c_uint = 0x0010;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STAT: c_uint = 0xC800 /* AUTO_NEG Vendor Status Reg */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_TX_ALARM: c_uint = 0xCC00 /* AUTO_NEG Vendor TX Reg */;
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_TX_ALARM2: c_uint = 0xCC01 /* AUTO_NEG Vendor Tx Reg */;
pub const IXGBE_MDIO_AUTO_NEG_VEN_LSC: c_uint = 0x1 /* AUTO_NEG Vendor Tx LSC */;
pub const IXGBE_MDIO_AUTO_NEG_EEE_ADVT: c_uint = 0x3C /* AUTO_NEG EEE Advt Reg */;
pub const IXGBE_MDIO_PHY_SET_LOW_POWER_MODE: c_uint = 0x0800 /* Set low power mode */;
pub const IXGBE_AUTO_NEG_LP_STATUS: c_uint = 0xE820 /* AUTO NEG Rx LP Status Reg */;
pub const IXGBE_AUTO_NEG_LP_1000BASE_CAP: c_uint = 0x8000 /* AUTO NEG Rx LP 1000BaseT */;
pub const IXGBE_MDIO_TX_VENDOR_ALARMS_3: c_uint = 0xCC02 /* Vendor Alarms 3 Reg */;
pub const IXGBE_MDIO_TX_VENDOR_ALARMS_3_RST_MASK: c_uint = 0x3 /* PHY Reset Complete Mask */;
pub const IXGBE_MDIO_GLOBAL_RES_PR_10: c_uint = 0xC479 /* Global Resv Provisioning 10 Reg */;
pub const IXGBE_MDIO_POWER_UP_STALL: c_uint = 0x8000 /* Power Up Stall */;
pub const IXGBE_MDIO_GLOBAL_INT_CHIP_STD_MASK: c_uint = 0xFF00 /* int std mask */;
pub const IXGBE_MDIO_GLOBAL_CHIP_STD_INT_FLAG: c_uint = 0xFC00 /* chip std int flag */;
pub const IXGBE_MDIO_GLOBAL_INT_CHIP_VEN_MASK: c_uint = 0xFF01 /* int chip-wide mask */;
pub const IXGBE_MDIO_GLOBAL_INT_CHIP_VEN_FLAG: c_uint = 0xFC01 /* int chip-wide mask */;
pub const IXGBE_MDIO_GLOBAL_ALARM_1: c_uint = 0xCC00 /* Global alarm 1 */;
pub const IXGBE_MDIO_GLOBAL_ALM_1_DEV_FAULT: c_uint = 0x0010 /* device fault */;
pub const IXGBE_MDIO_GLOBAL_ALM_1_HI_TMP_FAIL: c_uint = 0x4000 /* high temp failure */;
pub const IXGBE_MDIO_GLOBAL_FAULT_MSG: c_uint = 0xC850 /* global fault msg */;
pub const IXGBE_MDIO_GLOBAL_FAULT_MSG_HI_TMP: c_uint = 0x8007 /* high temp failure */;
pub const IXGBE_MDIO_GLOBAL_INT_MASK: c_uint = 0xD400 /* Global int mask */;
// autoneg vendor alarm int enable
pub const IXGBE_MDIO_GLOBAL_AN_VEN_ALM_INT_EN: c_uint = 0x1000;
pub const IXGBE_MDIO_GLOBAL_ALARM_1_INT: c_uint = 0x4 /* int in Global alarm 1 */;
pub const IXGBE_MDIO_GLOBAL_VEN_ALM_INT_EN: c_uint = 0x1 /* vendor alarm int enable */;
pub const IXGBE_MDIO_GLOBAL_STD_ALM2_INT: c_uint = 0x200 /* vendor alarm2 int mask */;
pub const IXGBE_MDIO_GLOBAL_INT_HI_TEMP_EN: c_uint = 0x4000 /* int high temp enable */;
pub const IXGBE_MDIO_GLOBAL_INT_DEV_FAULT_EN: c_uint = 0x0010 /*int dev fault enable */;
pub const IXGBE_MDIO_PMA_PMD_SDA_SCL_ADDR: c_uint = 0xC30A /* PHY_XS SDA/SCL Addr Reg */;
pub const IXGBE_MDIO_PMA_PMD_SDA_SCL_DATA: c_uint = 0xC30B /* PHY_XS SDA/SCL Data Reg */;
pub const IXGBE_MDIO_PMA_PMD_SDA_SCL_STAT: c_uint = 0xC30C /* PHY_XS SDA/SCL Stat Reg */;
pub const IXGBE_MDIO_PMA_TX_VEN_LASI_INT_MASK: c_uint = 0xD401 /* PHY TX Vendor LASI */;
pub const IXGBE_MDIO_PMA_TX_VEN_LASI_INT_EN: c_uint = 0x1 /* PHY TX Vendor LASI enable */;
pub const IXGBE_MDIO_PMD_STD_TX_DISABLE_CNTR: c_uint = 0x9 /* Standard Tx Dis Reg */;
pub const IXGBE_MDIO_PMD_GLOBAL_TX_DISABLE: c_uint = 0x0001 /* PMD Global Tx Dis */;
// MII clause 22/28 definitions
pub const IXGBE_MII_AUTONEG_VENDOR_PROVISION_1_REG: c_uint = 0xC400 /* 1G Provisioning 1 */;
pub const IXGBE_MII_AUTONEG_XNP_TX_REG: c_uint = 0x17   /* 1G XNP Transmit */;
pub const IXGBE_MII_1GBASE_T_ADVERTISE_XNP_TX: c_uint = 0x4000 /* full duplex, bit:14*/;
pub const IXGBE_MII_1GBASE_T_ADVERTISE: c_uint = 0x8000 /* full duplex, bit:15*/;
pub const IXGBE_MII_AUTONEG_REG: c_uint = 0x0;
pub const IXGBE_PHY_REVISION_MASK: c_uint = 0xFFFFFFF0;
pub const IXGBE_MAX_PHY_ADDR: c_int = 32;
// PHY IDs
pub const TN1010_PHY_ID: c_uint = 0x00A19410;
pub const TNX_FW_REV: c_uint = 0xB;
pub const X540_PHY_ID: c_uint = 0x01540200;
pub const X550_PHY_ID2: c_uint = 0x01540223;
pub const X550_PHY_ID3: c_uint = 0x01540221;
pub const X557_PHY_ID: c_uint = 0x01540240;
pub const X557_PHY_ID2: c_uint = 0x01540250;
pub const QT2022_PHY_ID: c_uint = 0x0043A400;
pub const ATH_PHY_ID: c_uint = 0x03429050;
pub const AQ_FW_REV: c_uint = 0x20;
pub const BCM54616S_E_PHY_ID: c_uint = 0x03625D10;
// Special PHY Init Routine
pub const IXGBE_PHY_INIT_OFFSET_NL: c_uint = 0x002B;
pub const IXGBE_PHY_INIT_END_NL: c_uint = 0xFFFF;
pub const IXGBE_CONTROL_MASK_NL: c_uint = 0xF000;
pub const IXGBE_DATA_MASK_NL: c_uint = 0x0FFF;
pub const IXGBE_CONTROL_SHIFT_NL: c_int = 12;
pub const IXGBE_DELAY_NL: c_int = 0;
pub const IXGBE_DATA_NL: c_int = 1;
pub const IXGBE_CONTROL_NL: c_uint = 0x000F;
pub const IXGBE_CONTROL_EOL_NL: c_uint = 0x0FFF;
pub const IXGBE_CONTROL_SOL_NL: c_uint = 0x0000;
// General purpose Interrupt Enable
pub const IXGBE_SDP0_GPIEN_8259X: c_uint = 0x00000001 /* SDP0 */;
pub const IXGBE_SDP1_GPIEN_8259X: c_uint = 0x00000002 /* SDP1 */;
pub const IXGBE_SDP2_GPIEN_8259X: c_uint = 0x00000004 /* SDP2 */;
pub const IXGBE_SDP0_GPIEN_X540: c_uint = 0x00000002 /* SDP0 on X540 and X550 */;
pub const IXGBE_SDP1_GPIEN_X540: c_uint = 0x00000004 /* SDP1 on X540 and X550 */;
pub const IXGBE_SDP2_GPIEN_X540: c_uint = 0x00000008 /* SDP2 on X540 and X550 */;

pub const IXGBE_GPIE_MSIX_MODE: c_uint = 0x00000010 /* MSI-X mode */;
pub const IXGBE_GPIE_OCD: c_uint = 0x00000020 /* Other Clear Disable */;
pub const IXGBE_GPIE_EIMEN: c_uint = 0x00000040 /* Immediate Interrupt Enable */;
pub const IXGBE_GPIE_EIAME: c_uint = 0x40000000;
pub const IXGBE_GPIE_PBA_SUPPORT: c_uint = 0x80000000;
pub const IXGBE_GPIE_RSC_DELAY_SHIFT: c_int = 11;
pub const IXGBE_GPIE_VTMODE_MASK: c_uint = 0x0000C000 /* VT Mode Mask */;
pub const IXGBE_GPIE_VTMODE_16: c_uint = 0x00004000 /* 16 VFs 8 queues per VF */;
pub const IXGBE_GPIE_VTMODE_32: c_uint = 0x00008000 /* 32 VFs 4 queues per VF */;
pub const IXGBE_GPIE_VTMODE_64: c_uint = 0x0000C000 /* 64 VFs 2 queues per VF */;
// Packet Buffer Initialization
pub const IXGBE_TXPBSIZE_20KB: c_uint = 0x00005000 /* 20KB Packet Buffer */;
pub const IXGBE_TXPBSIZE_40KB: c_uint = 0x0000A000 /* 40KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_48KB: c_uint = 0x0000C000 /* 48KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_64KB: c_uint = 0x00010000 /* 64KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_80KB: c_uint = 0x00014000 /* 80KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_128KB: c_uint = 0x00020000 /* 128KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_MAX: c_uint = 0x00080000 /* 512KB Packet Buffer*/;
pub const IXGBE_TXPBSIZE_MAX: c_uint = 0x00028000 /* 160KB Packet Buffer*/;
pub const IXGBE_TXPKT_SIZE_MAX: c_uint = 0xA        /* Max Tx Packet size  */;
pub const IXGBE_MAX_PB: c_int = 8;
// Packet buffer allocation strategies

// Transmit Flow Control status
pub const IXGBE_TFCS_TXOFF: c_uint = 0x00000001;
pub const IXGBE_TFCS_TXOFF0: c_uint = 0x00000100;
pub const IXGBE_TFCS_TXOFF1: c_uint = 0x00000200;
pub const IXGBE_TFCS_TXOFF2: c_uint = 0x00000400;
pub const IXGBE_TFCS_TXOFF3: c_uint = 0x00000800;
pub const IXGBE_TFCS_TXOFF4: c_uint = 0x00001000;
pub const IXGBE_TFCS_TXOFF5: c_uint = 0x00002000;
pub const IXGBE_TFCS_TXOFF6: c_uint = 0x00004000;
pub const IXGBE_TFCS_TXOFF7: c_uint = 0x00008000;
// TCP Timer
pub const IXGBE_TCPTIMER_KS: c_uint = 0x00000100;
pub const IXGBE_TCPTIMER_COUNT_ENABLE: c_uint = 0x00000200;
pub const IXGBE_TCPTIMER_COUNT_FINISH: c_uint = 0x00000400;
pub const IXGBE_TCPTIMER_LOOP: c_uint = 0x00000800;
pub const IXGBE_TCPTIMER_DURATION_MASK: c_uint = 0x000000FF;
// HLREG0 Bit Masks
pub const IXGBE_HLREG0_TXCRCEN: c_uint = 0x00000001   /* bit  0 */;
pub const IXGBE_HLREG0_RXCRCSTRP: c_uint = 0x00000002   /* bit  1 */;
pub const IXGBE_HLREG0_JUMBOEN: c_uint = 0x00000004   /* bit  2 */;
pub const IXGBE_HLREG0_TXPADEN: c_uint = 0x00000400   /* bit 10 */;
pub const IXGBE_HLREG0_TXPAUSEEN: c_uint = 0x00001000   /* bit 12 */;
pub const IXGBE_HLREG0_RXPAUSEEN: c_uint = 0x00004000   /* bit 14 */;
pub const IXGBE_HLREG0_LPBK: c_uint = 0x00008000   /* bit 15 */;
pub const IXGBE_HLREG0_MDCSPD: c_uint = 0x00010000   /* bit 16 */;
pub const IXGBE_HLREG0_CONTMDC: c_uint = 0x00020000   /* bit 17 */;
pub const IXGBE_HLREG0_CTRLFLTR: c_uint = 0x00040000   /* bit 18 */;
pub const IXGBE_HLREG0_PREPEND: c_uint = 0x00F00000   /* bits 20-23 */;
pub const IXGBE_HLREG0_PRIPAUSEEN: c_uint = 0x01000000   /* bit 24 */;
pub const IXGBE_HLREG0_RXPAUSERECDA: c_uint = 0x06000000   /* bits 25-26 */;
pub const IXGBE_HLREG0_RXLNGTHERREN: c_uint = 0x08000000   /* bit 27 */;
pub const IXGBE_HLREG0_RXPADSTRIPEN: c_uint = 0x10000000   /* bit 28 */;
// VMD_CTL bitmasks
pub const IXGBE_VMD_CTL_VMDQ_EN: c_uint = 0x00000001;
pub const IXGBE_VMD_CTL_VMDQ_FILTER: c_uint = 0x00000002;
// VT_CTL bitmasks
pub const IXGBE_VT_CTL_DIS_DEFPL: c_uint = 0x20000000 /* disable default pool */;
pub const IXGBE_VT_CTL_REPLEN: c_uint = 0x40000000 /* replication enabled */;
pub const IXGBE_VT_CTL_VT_ENABLE: c_uint = 0x00000001  /* Enable VT Mode */;
pub const IXGBE_VT_CTL_POOL_SHIFT: c_int = 7;

// VMOLR bitmasks
pub const IXGBE_VMOLR_UPE: c_uint = 0x00400000 /* unicast promiscuous */;
pub const IXGBE_VMOLR_VPE: c_uint = 0x00800000 /* VLAN promiscuous */;
pub const IXGBE_VMOLR_AUPE: c_uint = 0x01000000 /* accept untagged packets */;
pub const IXGBE_VMOLR_ROMPE: c_uint = 0x02000000 /* accept packets in MTA tbl */;
pub const IXGBE_VMOLR_ROPE: c_uint = 0x04000000 /* accept packets in UC tbl */;
pub const IXGBE_VMOLR_BAM: c_uint = 0x08000000 /* accept broadcast packets */;
pub const IXGBE_VMOLR_MPE: c_uint = 0x10000000 /* multicast promiscuous */;
// VFRE bitmask
pub const IXGBE_VFRE_ENABLE_ALL: c_uint = 0xFFFFFFFF;

// RDHMPN and TDHMPN bitmasks
pub const IXGBE_RDHMPN_RDICADDR: c_uint = 0x007FF800;
pub const IXGBE_RDHMPN_RDICRDREQ: c_uint = 0x00800000;
pub const IXGBE_RDHMPN_RDICADDR_SHIFT: c_int = 11;
pub const IXGBE_TDHMPN_TDICADDR: c_uint = 0x003FF800;
pub const IXGBE_TDHMPN_TDICRDREQ: c_uint = 0x00800000;
pub const IXGBE_TDHMPN_TDICADDR_SHIFT: c_int = 11;
pub const IXGBE_RDMAM_MEM_SEL_SHIFT: c_int = 13;
pub const IXGBE_RDMAM_DWORD_SHIFT: c_int = 9;
pub const IXGBE_RDMAM_DESC_COMP_FIFO: c_int = 1;
pub const IXGBE_RDMAM_DFC_CMD_FIFO: c_int = 2;
pub const IXGBE_RDMAM_TCN_STATUS_RAM: c_int = 4;
pub const IXGBE_RDMAM_WB_COLL_FIFO: c_int = 5;
pub const IXGBE_RDMAM_QSC_CNT_RAM: c_int = 6;
pub const IXGBE_RDMAM_QSC_QUEUE_CNT: c_int = 8;
pub const IXGBE_RDMAM_QSC_QUEUE_RAM: c_uint = 0xA;
pub const IXGBE_RDMAM_DESC_COM_FIFO_RANGE: c_int = 135;
pub const IXGBE_RDMAM_DESC_COM_FIFO_COUNT: c_int = 4;
pub const IXGBE_RDMAM_DFC_CMD_FIFO_RANGE: c_int = 48;
pub const IXGBE_RDMAM_DFC_CMD_FIFO_COUNT: c_int = 7;
pub const IXGBE_RDMAM_TCN_STATUS_RAM_RANGE: c_int = 256;
pub const IXGBE_RDMAM_TCN_STATUS_RAM_COUNT: c_int = 9;
pub const IXGBE_RDMAM_WB_COLL_FIFO_RANGE: c_int = 8;
pub const IXGBE_RDMAM_WB_COLL_FIFO_COUNT: c_int = 4;
pub const IXGBE_RDMAM_QSC_CNT_RAM_RANGE: c_int = 64;
pub const IXGBE_RDMAM_QSC_CNT_RAM_COUNT: c_int = 4;
pub const IXGBE_RDMAM_QSC_QUEUE_CNT_RANGE: c_int = 32;
pub const IXGBE_RDMAM_QSC_QUEUE_CNT_COUNT: c_int = 4;
pub const IXGBE_RDMAM_QSC_QUEUE_RAM_RANGE: c_int = 128;
pub const IXGBE_RDMAM_QSC_QUEUE_RAM_COUNT: c_int = 8;
pub const IXGBE_TXDESCIC_READY: c_uint = 0x80000000;
// Receive Checksum Control
pub const IXGBE_RXCSUM_IPPCSE: c_uint = 0x00001000   /* IP payload checksum enable */;
pub const IXGBE_RXCSUM_PCSD: c_uint = 0x00002000   /* packet checksum disabled */;
// FCRTL Bit Masks
pub const IXGBE_FCRTL_XONE: c_uint = 0x80000000  /* XON enable */;
pub const IXGBE_FCRTH_FCEN: c_uint = 0x80000000  /* Packet buffer fc enable */;
// PAP bit masks
pub const IXGBE_PAP_TXPAUSECNT_MASK: c_uint = 0x0000FFFF /* Pause counter mask */;
// RMCS Bit Masks
pub const IXGBE_RMCS_RRM: c_uint = 0x00000002 /* Receive Recycle Mode enable */;
// Receive Arbitration Control: 0 Round Robin, 1 DFP
pub const IXGBE_RMCS_RAC: c_uint = 0x00000004;

pub const IXGBE_RMCS_TFCE_802_3X: c_uint = 0x00000008 /* Tx Priority FC ena */;
pub const IXGBE_RMCS_TFCE_PRIORITY: c_uint = 0x00000010 /* Tx Priority FC ena */;
pub const IXGBE_RMCS_ARBDIS: c_uint = 0x00000040 /* Arbitration disable bit */;
// FCCFG Bit Masks
pub const IXGBE_FCCFG_TFCE_802_3X: c_uint = 0x00000008 /* Tx link FC enable */;
pub const IXGBE_FCCFG_TFCE_PRIORITY: c_uint = 0x00000010 /* Tx priority FC enable */;
// Interrupt register bitmasks
// Extended Interrupt Cause Read
pub const IXGBE_EICR_RTX_QUEUE: c_uint = 0x0000FFFF /* RTx Queue Interrupt */;
pub const IXGBE_EICR_FLOW_DIR: c_uint = 0x00010000 /* FDir Exception */;
pub const IXGBE_EICR_RX_MISS: c_uint = 0x00020000 /* Packet Buffer Overrun */;
pub const IXGBE_EICR_PCI: c_uint = 0x00040000 /* PCI Exception */;
pub const IXGBE_EICR_MAILBOX: c_uint = 0x00080000 /* VF to PF Mailbox Interrupt */;
pub const IXGBE_EICR_LSC: c_uint = 0x00100000 /* Link Status Change */;
pub const IXGBE_EICR_FW_EVENT: c_uint = 0x00200000 /* Async FW event */;
pub const IXGBE_EICR_MNG: c_uint = 0x00400000 /* Manageability Event Interrupt */;
pub const IXGBE_EICR_TS: c_uint = 0x00800000 /* Thermal Sensor Event */;
pub const IXGBE_EICR_TIMESYNC: c_uint = 0x01000000 /* Timesync Event */;
pub const IXGBE_EICR_GPI_SDP0_8259X: c_uint = 0x01000000 /* Gen Purpose INT on SDP0 */;
pub const IXGBE_EICR_GPI_SDP1_8259X: c_uint = 0x02000000 /* Gen Purpose INT on SDP1 */;
pub const IXGBE_EICR_GPI_SDP2_8259X: c_uint = 0x04000000 /* Gen Purpose INT on SDP2 */;
pub const IXGBE_EICR_GPI_SDP0_X540: c_uint = 0x02000000;
pub const IXGBE_EICR_GPI_SDP1_X540: c_uint = 0x04000000;
pub const IXGBE_EICR_GPI_SDP2_X540: c_uint = 0x08000000;

pub const IXGBE_EICR_ECC: c_uint = 0x10000000 /* ECC Error */;
pub const IXGBE_EICR_PBUR: c_uint = 0x10000000 /* Packet Buffer Handler Error */;
pub const IXGBE_EICR_DHER: c_uint = 0x20000000 /* Descriptor Handler Error */;
pub const IXGBE_EICR_TCP_TIMER: c_uint = 0x40000000 /* TCP Timer */;
pub const IXGBE_EICR_OTHER: c_uint = 0x80000000 /* Interrupt Cause Active */;
// Extended Interrupt Cause Set

// Extended Interrupt Mask Set

// Extended Interrupt Mask Clear

// Immediate Interrupt Rx (A.K.A. Low Latency Interrupt)
pub const IXGBE_IMIR_PORT_IM_EN: c_uint = 0x00010000  /* TCP port enable */;
pub const IXGBE_IMIR_PORT_BP: c_uint = 0x00020000  /* TCP port check bypass */;
pub const IXGBE_IMIREXT_SIZE_BP: c_uint = 0x00001000  /* Packet size bypass */;
pub const IXGBE_IMIREXT_CTRL_URG: c_uint = 0x00002000  /* Check URG bit in header */;
pub const IXGBE_IMIREXT_CTRL_ACK: c_uint = 0x00004000  /* Check ACK bit in header */;
pub const IXGBE_IMIREXT_CTRL_PSH: c_uint = 0x00008000  /* Check PSH bit in header */;
pub const IXGBE_IMIREXT_CTRL_RST: c_uint = 0x00010000  /* Check RST bit in header */;
pub const IXGBE_IMIREXT_CTRL_SYN: c_uint = 0x00020000  /* Check SYN bit in header */;
pub const IXGBE_IMIREXT_CTRL_FIN: c_uint = 0x00040000  /* Check FIN bit in header */;
pub const IXGBE_IMIREXT_CTRL_BP: c_uint = 0x00080000  /* Bypass check of control bits */;
pub const IXGBE_IMIR_SIZE_BP_82599: c_uint = 0x00001000 /* Packet size bypass */;
pub const IXGBE_IMIR_CTRL_URG_82599: c_uint = 0x00002000 /* Check URG bit in header */;
pub const IXGBE_IMIR_CTRL_ACK_82599: c_uint = 0x00004000 /* Check ACK bit in header */;
pub const IXGBE_IMIR_CTRL_PSH_82599: c_uint = 0x00008000 /* Check PSH bit in header */;
pub const IXGBE_IMIR_CTRL_RST_82599: c_uint = 0x00010000 /* Check RST bit in header */;
pub const IXGBE_IMIR_CTRL_SYN_82599: c_uint = 0x00020000 /* Check SYN bit in header */;
pub const IXGBE_IMIR_CTRL_FIN_82599: c_uint = 0x00040000 /* Check FIN bit in header */;
pub const IXGBE_IMIR_CTRL_BP_82599: c_uint = 0x00080000 /* Bypass check of control bits */;
pub const IXGBE_IMIR_LLI_EN_82599: c_uint = 0x00100000 /* Enables low latency Int */;
pub const IXGBE_IMIR_RX_QUEUE_MASK_82599: c_uint = 0x0000007F /* Rx Queue Mask */;

pub const IXGBE_IMIRVP_PRIORITY_MASK: c_uint = 0x00000007 /* VLAN priority mask */;
pub const IXGBE_IMIRVP_PRIORITY_EN: c_uint = 0x00000008 /* VLAN priority enable */;
pub const IXGBE_MAX_FTQF_FILTERS: c_int = 128;
pub const IXGBE_FTQF_PROTOCOL_MASK: c_uint = 0x00000003;
pub const IXGBE_FTQF_PROTOCOL_TCP: c_uint = 0x00000000;
pub const IXGBE_FTQF_PROTOCOL_UDP: c_uint = 0x00000001;
pub const IXGBE_FTQF_PROTOCOL_SCTP: c_int = 2;
pub const IXGBE_FTQF_PRIORITY_MASK: c_uint = 0x00000007;
pub const IXGBE_FTQF_PRIORITY_SHIFT: c_int = 2;
pub const IXGBE_FTQF_POOL_MASK: c_uint = 0x0000003F;
pub const IXGBE_FTQF_POOL_SHIFT: c_int = 8;
pub const IXGBE_FTQF_5TUPLE_MASK_MASK: c_uint = 0x0000001F;
pub const IXGBE_FTQF_5TUPLE_MASK_SHIFT: c_int = 25;
pub const IXGBE_FTQF_SOURCE_ADDR_MASK: c_uint = 0x1E;
pub const IXGBE_FTQF_DEST_ADDR_MASK: c_uint = 0x1D;
pub const IXGBE_FTQF_SOURCE_PORT_MASK: c_uint = 0x1B;
pub const IXGBE_FTQF_DEST_PORT_MASK: c_uint = 0x17;
pub const IXGBE_FTQF_PROTOCOL_COMP_MASK: c_uint = 0x0F;
pub const IXGBE_FTQF_POOL_MASK_EN: c_uint = 0x40000000;
pub const IXGBE_FTQF_QUEUE_ENABLE: c_uint = 0x80000000;
// Interrupt clear mask
pub const IXGBE_IRQ_CLEAR_MASK: c_uint = 0xFFFFFFFF;
// Interrupt Vector Allocation Registers
pub const IXGBE_IVAR_REG_NUM: c_int = 25;
pub const IXGBE_IVAR_REG_NUM_82599: c_int = 64;
pub const IXGBE_IVAR_TXRX_ENTRY: c_int = 96;
pub const IXGBE_IVAR_RX_ENTRY: c_int = 64;

pub const IXGBE_IVAR_TX_ENTRY: c_int = 32;

pub const IXGBE_IVAR_ALLOC_VAL: c_uint = 0x80 /* Interrupt Allocation valid */;
// ETYPE Queue Filter/Select Bit Masks
pub const IXGBE_MAX_ETQF_FILTERS: c_int = 8;
pub const IXGBE_ETQF_FCOE: c_uint = 0x08000000 /* bit 27 */;
pub const IXGBE_ETQF_BCN: c_uint = 0x10000000 /* bit 28 */;
pub const IXGBE_ETQF_TX_ANTISPOOF: c_uint = 0x20000000 /* bit 29 */;
pub const IXGBE_ETQF_1588: c_uint = 0x40000000 /* bit 30 */;
pub const IXGBE_ETQF_FILTER_EN: c_uint = 0x80000000 /* bit 31 */;

pub const IXGBE_ETQF_POOL_SHIFT: c_int = 20;
pub const IXGBE_ETQS_RX_QUEUE: c_uint = 0x007F0000 /* bits 22:16 */;
pub const IXGBE_ETQS_RX_QUEUE_SHIFT: c_int = 16;
pub const IXGBE_ETQS_LLI: c_uint = 0x20000000 /* bit 29 */;
pub const IXGBE_ETQS_QUEUE_EN: c_uint = 0x80000000 /* bit 31 */;
//
// ETQF filter list: one static filter per filter consumer. This is
// to avoid filter collisions later. Add new filters
// here!!
//
// Current filters:
// EAPOL 802.1x (0x888e): Filter 0
// FCoE (0x8906):         Filter 2
// 1588 (0x88f7):         Filter 3
// FIP  (0x8914):         Filter 4
// LLDP (0x88CC):         Filter 5
// LACP (0x8809):         Filter 6
// FC   (0x8808):         Filter 7
//
pub const IXGBE_ETQF_FILTER_EAPOL: c_int = 0;
pub const IXGBE_ETQF_FILTER_FCOE: c_int = 2;
pub const IXGBE_ETQF_FILTER_1588: c_int = 3;
pub const IXGBE_ETQF_FILTER_FIP: c_int = 4;
pub const IXGBE_ETQF_FILTER_LLDP: c_int = 5;
pub const IXGBE_ETQF_FILTER_LACP: c_int = 6;
pub const IXGBE_ETQF_FILTER_FC: c_int = 7;
// VLAN Control Bit Masks
pub const IXGBE_VLNCTRL_VET: c_uint = 0x0000FFFF  /* bits 0-15 */;
pub const IXGBE_VLNCTRL_CFI: c_uint = 0x10000000  /* bit 28 */;
pub const IXGBE_VLNCTRL_CFIEN: c_uint = 0x20000000  /* bit 29 */;
pub const IXGBE_VLNCTRL_VFE: c_uint = 0x40000000  /* bit 30 */;
pub const IXGBE_VLNCTRL_VME: c_uint = 0x80000000  /* bit 31 */;
// VLAN pool filtering masks
pub const IXGBE_VLVF_VIEN: c_uint = 0x80000000  /* filter is valid */;
pub const IXGBE_VLVF_ENTRIES: c_int = 64;
pub const IXGBE_VLVF_VLANID_MASK: c_uint = 0x00000FFF;
// Per VF Port VLAN insertion rules
pub const IXGBE_VMVIR_VLANA_DEFAULT: c_uint = 0x40000000 /* Always use default VLAN */;
pub const IXGBE_VMVIR_VLANA_NEVER: c_uint = 0x80000000 /* Never insert VLAN tag */;
pub const IXGBE_ETHERNET_IEEE_VLAN_TYPE: c_uint = 0x8100  /* 802.1q protocol */;
// STATUS Bit Masks
pub const IXGBE_STATUS_LAN_ID: c_uint = 0x0000000C /* LAN ID */;

pub const IXGBE_STATUS_GIO: c_uint = 0x00080000 /* GIO Primary Enable Status */;
pub const IXGBE_STATUS_LAN_ID_0: c_uint = 0x00000000 /* LAN ID 0 */;
pub const IXGBE_STATUS_LAN_ID_1: c_uint = 0x00000004 /* LAN ID 1 */;
// ESDP Bit Masks
pub const IXGBE_ESDP_SDP0: c_uint = 0x00000001 /* SDP0 Data Value */;
pub const IXGBE_ESDP_SDP1: c_uint = 0x00000002 /* SDP1 Data Value */;
pub const IXGBE_ESDP_SDP2: c_uint = 0x00000004 /* SDP2 Data Value */;
pub const IXGBE_ESDP_SDP3: c_uint = 0x00000008 /* SDP3 Data Value */;
pub const IXGBE_ESDP_SDP4: c_uint = 0x00000010 /* SDP4 Data Value */;
pub const IXGBE_ESDP_SDP5: c_uint = 0x00000020 /* SDP5 Data Value */;
pub const IXGBE_ESDP_SDP6: c_uint = 0x00000040 /* SDP6 Data Value */;
pub const IXGBE_ESDP_SDP0_DIR: c_uint = 0x00000100 /* SDP0 IO direction */;
pub const IXGBE_ESDP_SDP1_DIR: c_uint = 0x00000200 /* SDP1 IO direction */;
pub const IXGBE_ESDP_SDP4_DIR: c_uint = 0x00000004 /* SDP4 IO direction */;
pub const IXGBE_ESDP_SDP5_DIR: c_uint = 0x00002000 /* SDP5 IO direction */;
pub const IXGBE_ESDP_SDP0_NATIVE: c_uint = 0x00010000 /* SDP0 Native Function */;
pub const IXGBE_ESDP_SDP1_NATIVE: c_uint = 0x00020000 /* SDP1 IO mode */;
// LEDCTL Bit Masks
pub const IXGBE_LED_IVRT_BASE: c_uint = 0x00000040;
pub const IXGBE_LED_BLINK_BASE: c_uint = 0x00000080;
pub const IXGBE_LED_MODE_MASK_BASE: c_uint = 0x0000000F;

pub const IXGBE_X557_MAX_LED_INDEX: c_int = 3;
pub const IXGBE_X557_LED_PROVISIONING: c_uint = 0xC430;
// LED modes
pub const IXGBE_LED_LINK_UP: c_uint = 0x0;
pub const IXGBE_LED_LINK_10G: c_uint = 0x1;
pub const IXGBE_LED_MAC: c_uint = 0x2;
pub const IXGBE_LED_FILTER: c_uint = 0x3;
pub const IXGBE_LED_LINK_ACTIVE: c_uint = 0x4;
pub const IXGBE_LED_LINK_1G: c_uint = 0x5;
pub const IXGBE_LED_ON: c_uint = 0xE;
pub const IXGBE_LED_OFF: c_uint = 0xF;
// AUTOC Bit Masks
pub const IXGBE_AUTOC_KX4_KX_SUPP_MASK: c_uint = 0xC0000000;
pub const IXGBE_AUTOC_KX4_SUPP: c_uint = 0x80000000;
pub const IXGBE_AUTOC_KX_SUPP: c_uint = 0x40000000;
pub const IXGBE_AUTOC_PAUSE: c_uint = 0x30000000;
pub const IXGBE_AUTOC_ASM_PAUSE: c_uint = 0x20000000;
pub const IXGBE_AUTOC_SYM_PAUSE: c_uint = 0x10000000;
pub const IXGBE_AUTOC_RF: c_uint = 0x08000000;
pub const IXGBE_AUTOC_PD_TMR: c_uint = 0x06000000;
pub const IXGBE_AUTOC_AN_RX_LOOSE: c_uint = 0x01000000;
pub const IXGBE_AUTOC_AN_RX_DRIFT: c_uint = 0x00800000;
pub const IXGBE_AUTOC_AN_RX_ALIGN: c_uint = 0x007C0000;
pub const IXGBE_AUTOC_FECA: c_uint = 0x00040000;
pub const IXGBE_AUTOC_FECR: c_uint = 0x00020000;
pub const IXGBE_AUTOC_KR_SUPP: c_uint = 0x00010000;
pub const IXGBE_AUTOC_AN_RESTART: c_uint = 0x00001000;
pub const IXGBE_AUTOC_FLU: c_uint = 0x00000001;
pub const IXGBE_AUTOC_LMS_SHIFT: c_int = 13;

pub const IXGBE_AUTOC_1G_PMA_PMD_MASK: c_uint = 0x00000200;
pub const IXGBE_AUTOC_1G_PMA_PMD_SHIFT: c_int = 9;
pub const IXGBE_AUTOC_10G_PMA_PMD_MASK: c_uint = 0x00000180;
pub const IXGBE_AUTOC_10G_PMA_PMD_SHIFT: c_int = 7;

pub const IXGBE_AUTOC2_UPPER_MASK: c_uint = 0xFFFF0000;
pub const IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_MASK: c_uint = 0x00030000;
pub const IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_SHIFT: c_int = 16;

pub const IXGBE_AUTOC2_LINK_DISABLE_ON_D3_MASK: c_uint = 0x50000000;
pub const IXGBE_AUTOC2_LINK_DISABLE_MASK: c_uint = 0x70000000;
pub const IXGBE_MACC_FLU: c_uint = 0x00000001;
pub const IXGBE_MACC_FSV_10G: c_uint = 0x00030000;
pub const IXGBE_MACC_FS: c_uint = 0x00040000;
pub const IXGBE_MAC_RX2TX_LPBK: c_uint = 0x00000002;
// Veto Bit definition
pub const IXGBE_MMNGC_MNG_VETO: c_uint = 0x00000001;
// LINKS Bit Masks
pub const IXGBE_LINKS_KX_AN_COMP: c_uint = 0x80000000;
pub const IXGBE_LINKS_UP: c_uint = 0x40000000;
pub const IXGBE_LINKS_SPEED: c_uint = 0x20000000;
pub const IXGBE_LINKS_MODE: c_uint = 0x18000000;
pub const IXGBE_LINKS_RX_MODE: c_uint = 0x06000000;
pub const IXGBE_LINKS_TX_MODE: c_uint = 0x01800000;
pub const IXGBE_LINKS_XGXS_EN: c_uint = 0x00400000;
pub const IXGBE_LINKS_SGMII_EN: c_uint = 0x02000000;
pub const IXGBE_LINKS_PCS_1G_EN: c_uint = 0x00200000;
pub const IXGBE_LINKS_1G_AN_EN: c_uint = 0x00100000;
pub const IXGBE_LINKS_KX_AN_IDLE: c_uint = 0x00080000;
pub const IXGBE_LINKS_1G_SYNC: c_uint = 0x00040000;
pub const IXGBE_LINKS_10G_ALIGN: c_uint = 0x00020000;
pub const IXGBE_LINKS_10G_LANE_SYNC: c_uint = 0x00017000;
pub const IXGBE_LINKS_TL_FAULT: c_uint = 0x00001000;
pub const IXGBE_LINKS_SIGNAL: c_uint = 0x00000F00;
pub const IXGBE_LINKS_SPEED_NON_STD: c_uint = 0x08000000;
pub const IXGBE_LINKS_SPEED_82599: c_uint = 0x30000000;
pub const IXGBE_LINKS_SPEED_10G_82599: c_uint = 0x30000000;
pub const IXGBE_LINKS_SPEED_1G_82599: c_uint = 0x20000000;
pub const IXGBE_LINKS_SPEED_100_82599: c_uint = 0x10000000;
pub const IXGBE_LINKS_SPEED_10_X550EM_A: c_int = 0;

pub const IXGBE_LINKS2_AN_SUPPORTED: c_uint = 0x00000040;
// PCS1GLSTA Bit Masks
pub const IXGBE_PCS1GLSTA_LINK_OK: c_int = 1;
pub const IXGBE_PCS1GLSTA_SYNK_OK: c_uint = 0x10;
pub const IXGBE_PCS1GLSTA_AN_COMPLETE: c_uint = 0x10000;
pub const IXGBE_PCS1GLSTA_AN_PAGE_RX: c_uint = 0x20000;
pub const IXGBE_PCS1GLSTA_AN_TIMED_OUT: c_uint = 0x40000;
pub const IXGBE_PCS1GLSTA_AN_REMOTE_FAULT: c_uint = 0x80000;
pub const IXGBE_PCS1GLSTA_AN_ERROR_RWS: c_uint = 0x100000;
pub const IXGBE_PCS1GANA_SYM_PAUSE: c_uint = 0x80;
pub const IXGBE_PCS1GANA_ASM_PAUSE: c_uint = 0x100;
// PCS1GLCTL Bit Masks
pub const IXGBE_PCS1GLCTL_AN_1G_TIMEOUT_EN: c_uint = 0x00040000 /* PCS 1G autoneg to en */;
pub const IXGBE_PCS1GLCTL_FLV_LINK_UP: c_int = 1;
pub const IXGBE_PCS1GLCTL_FORCE_LINK: c_uint = 0x20;
pub const IXGBE_PCS1GLCTL_LOW_LINK_LATCH: c_uint = 0x40;
pub const IXGBE_PCS1GLCTL_AN_ENABLE: c_uint = 0x10000;
pub const IXGBE_PCS1GLCTL_AN_RESTART: c_uint = 0x20000;
// ANLP1 Bit Masks
pub const IXGBE_ANLP1_PAUSE: c_uint = 0x0C00;
pub const IXGBE_ANLP1_SYM_PAUSE: c_uint = 0x0400;
pub const IXGBE_ANLP1_ASM_PAUSE: c_uint = 0x0800;
pub const IXGBE_ANLP1_AN_STATE_MASK: c_uint = 0x000f0000;
// SW Semaphore Register bitmasks
pub const IXGBE_SWSM_SMBI: c_uint = 0x00000001 /* Driver Semaphore bit */;
pub const IXGBE_SWSM_SWESMBI: c_uint = 0x00000002 /* FW Semaphore bit */;
pub const IXGBE_SWSM_WMNG: c_uint = 0x00000004 /* Wake MNG Clock */;
pub const IXGBE_SWFW_REGSMP: c_uint = 0x80000000 /* Register Semaphore bit 31 */;
// SW_FW_SYNC/GSSR definitions
pub const IXGBE_GSSR_EEP_SM: c_uint = 0x0001;
pub const IXGBE_GSSR_PHY0_SM: c_uint = 0x0002;
pub const IXGBE_GSSR_PHY1_SM: c_uint = 0x0004;
pub const IXGBE_GSSR_MAC_CSR_SM: c_uint = 0x0008;
pub const IXGBE_GSSR_FLASH_SM: c_uint = 0x0010;
pub const IXGBE_GSSR_NVM_UPDATE_SM: c_uint = 0x0200;
pub const IXGBE_GSSR_SW_MNG_SM: c_uint = 0x0400;
pub const IXGBE_GSSR_TOKEN_SM: c_uint = 0x40000000 /* SW bit for shared access */;
pub const IXGBE_GSSR_SHARED_I2C_SM: c_uint = 0x1806 /* Wait for both phys & I2Cs */;
pub const IXGBE_GSSR_I2C_MASK: c_uint = 0x1800;
pub const IXGBE_GSSR_NVM_PHY_MASK: c_uint = 0xF;
// FW Status register bitmask
pub const IXGBE_FWSTS_FWRI: c_uint = 0x00000200 /* Firmware Reset Indication */;
// EEC Register
pub const IXGBE_EEC_SK: c_uint = 0x00000001 /* EEPROM Clock */;
pub const IXGBE_EEC_CS: c_uint = 0x00000002 /* EEPROM Chip Select */;
pub const IXGBE_EEC_DI: c_uint = 0x00000004 /* EEPROM Data In */;
pub const IXGBE_EEC_DO: c_uint = 0x00000008 /* EEPROM Data Out */;
pub const IXGBE_EEC_FWE_MASK: c_uint = 0x00000030 /* FLASH Write Enable */;
pub const IXGBE_EEC_FWE_DIS: c_uint = 0x00000010 /* Disable FLASH writes */;
pub const IXGBE_EEC_FWE_EN: c_uint = 0x00000020 /* Enable FLASH writes */;
pub const IXGBE_EEC_FWE_SHIFT: c_int = 4;
pub const IXGBE_EEC_REQ: c_uint = 0x00000040 /* EEPROM Access Request */;
pub const IXGBE_EEC_GNT: c_uint = 0x00000080 /* EEPROM Access Grant */;
pub const IXGBE_EEC_PRES: c_uint = 0x00000100 /* EEPROM Present */;
pub const IXGBE_EEC_ARD: c_uint = 0x00000200 /* EEPROM Auto Read Done */;
pub const IXGBE_EEC_FLUP: c_uint = 0x00800000 /* Flash update command */;
pub const IXGBE_EEC_SEC1VAL: c_uint = 0x02000000 /* Sector 1 Valid */;
pub const IXGBE_EEC_FLUDONE: c_uint = 0x04000000 /* Flash update done */;
// EEPROM Addressing bits based on type (0-small, 1-large)
pub const IXGBE_EEC_ADDR_SIZE: c_uint = 0x00000400;
pub const IXGBE_EEC_SIZE: c_uint = 0x00007800 /* EEPROM Size */;
pub const IXGBE_EERD_MAX_ADDR: c_uint = 0x00003FFF /* EERD allows 14 bits for addr. */;
pub const IXGBE_EEC_SIZE_SHIFT: c_int = 11;
pub const IXGBE_EEPROM_WORD_SIZE_SHIFT: c_int = 6;
pub const IXGBE_EEPROM_OPCODE_BITS: c_int = 8;
// Part Number String Length
pub const IXGBE_PBANUM_LENGTH: c_int = 11;
// Checksum and EEPROM pointers
pub const IXGBE_PBANUM_PTR_GUARD: c_uint = 0xFAFA;
pub const IXGBE_EEPROM_CHECKSUM: c_uint = 0x3F;
pub const IXGBE_EEPROM_SUM: c_uint = 0xBABA;
pub const IXGBE_EEPROM_CTRL_4: c_uint = 0x45;
pub const IXGBE_EE_CTRL_4_INST_ID: c_uint = 0x10;
pub const IXGBE_EE_CTRL_4_INST_ID_SHIFT: c_int = 4;
pub const IXGBE_PCIE_ANALOG_PTR: c_uint = 0x03;
pub const IXGBE_ATLAS0_CONFIG_PTR: c_uint = 0x04;
pub const IXGBE_PHY_PTR: c_uint = 0x04;
pub const IXGBE_ATLAS1_CONFIG_PTR: c_uint = 0x05;
pub const IXGBE_OPTION_ROM_PTR: c_uint = 0x05;
pub const IXGBE_PCIE_GENERAL_PTR: c_uint = 0x06;
pub const IXGBE_PCIE_CONFIG0_PTR: c_uint = 0x07;
pub const IXGBE_PCIE_CONFIG1_PTR: c_uint = 0x08;
pub const IXGBE_CORE0_PTR: c_uint = 0x09;
pub const IXGBE_CORE1_PTR: c_uint = 0x0A;
pub const IXGBE_MAC0_PTR: c_uint = 0x0B;
pub const IXGBE_MAC1_PTR: c_uint = 0x0C;
pub const IXGBE_CSR0_CONFIG_PTR: c_uint = 0x0D;
pub const IXGBE_CSR1_CONFIG_PTR: c_uint = 0x0E;
pub const IXGBE_PCIE_ANALOG_PTR_X550: c_uint = 0x02;
pub const IXGBE_SHADOW_RAM_SIZE_X550: c_uint = 0x4000;
pub const IXGBE_IXGBE_PCIE_GENERAL_SIZE: c_uint = 0x24;
pub const IXGBE_PCIE_CONFIG_SIZE: c_uint = 0x08;
pub const IXGBE_EEPROM_LAST_WORD: c_uint = 0x41;
pub const IXGBE_FW_PTR: c_uint = 0x0F;
pub const IXGBE_PBANUM0_PTR: c_uint = 0x15;
pub const IXGBE_PBANUM1_PTR: c_uint = 0x16;

// External Thermal Sensor Config
pub const IXGBE_ETS_CFG: c_uint = 0x26;
pub const IXGBE_ETS_LTHRES_DELTA_MASK: c_uint = 0x07C0;
pub const IXGBE_ETS_LTHRES_DELTA_SHIFT: c_int = 6;
pub const IXGBE_ETS_TYPE_MASK: c_uint = 0x0038;
pub const IXGBE_ETS_TYPE_SHIFT: c_int = 3;
pub const IXGBE_ETS_TYPE_EMC: c_uint = 0x000;
pub const IXGBE_ETS_TYPE_EMC_SHIFTED: c_uint = 0x000;
pub const IXGBE_ETS_NUM_SENSORS_MASK: c_uint = 0x0007;
pub const IXGBE_ETS_DATA_LOC_MASK: c_uint = 0x3C00;
pub const IXGBE_ETS_DATA_LOC_SHIFT: c_int = 10;
pub const IXGBE_ETS_DATA_INDEX_MASK: c_uint = 0x0300;
pub const IXGBE_ETS_DATA_INDEX_SHIFT: c_int = 8;
pub const IXGBE_ETS_DATA_HTHRESH_MASK: c_uint = 0x00FF;
pub const IXGBE_SAN_MAC_ADDR_PTR: c_uint = 0x28;
pub const IXGBE_DEVICE_CAPS: c_uint = 0x2C;
pub const IXGBE_SERIAL_NUMBER_MAC_ADDR: c_uint = 0x11;
pub const IXGBE_PCIE_MSIX_E610_CAPS: c_uint = 0xB2;
pub const IXGBE_PCIE_MSIX_82599_CAPS: c_uint = 0x72;
pub const IXGBE_MAX_MSIX_VECTORS_82599: c_uint = 0x40;
pub const IXGBE_PCIE_MSIX_82598_CAPS: c_uint = 0x62;
pub const IXGBE_MAX_MSIX_VECTORS_82598: c_uint = 0x13;
// MSI-X capability fields masks
pub const IXGBE_PCIE_MSIX_TBL_SZ_MASK: c_uint = 0x7FF;
// Legacy EEPROM word offsets
pub const IXGBE_ISCSI_BOOT_CAPS: c_uint = 0x0033;
pub const IXGBE_ISCSI_SETUP_PORT_0: c_uint = 0x0030;
pub const IXGBE_ISCSI_SETUP_PORT_1: c_uint = 0x0034;
// EEPROM Commands - SPI

pub const IXGBE_EEPROM_STATUS_RDY_SPI: c_uint = 0x01;
pub const IXGBE_EEPROM_READ_OPCODE_SPI: c_uint = 0x03  /* EEPROM read opcode */;
pub const IXGBE_EEPROM_WRITE_OPCODE_SPI: c_uint = 0x02  /* EEPROM write opcode */;
pub const IXGBE_EEPROM_A8_OPCODE_SPI: c_uint = 0x08  /* opcode bit-3 = addr bit-8 */;
pub const IXGBE_EEPROM_WREN_OPCODE_SPI: c_uint = 0x06  /* EEPROM set Write Ena latch */;
// EEPROM reset Write Enable latch
pub const IXGBE_EEPROM_WRDI_OPCODE_SPI: c_uint = 0x04;
pub const IXGBE_EEPROM_RDSR_OPCODE_SPI: c_uint = 0x05  /* EEPROM read Status reg */;
pub const IXGBE_EEPROM_WRSR_OPCODE_SPI: c_uint = 0x01  /* EEPROM write Status reg */;
pub const IXGBE_EEPROM_ERASE4K_OPCODE_SPI: c_uint = 0x20  /* EEPROM ERASE 4KB */;
pub const IXGBE_EEPROM_ERASE64K_OPCODE_SPI: c_uint = 0xD8  /* EEPROM ERASE 64KB */;
pub const IXGBE_EEPROM_ERASE256_OPCODE_SPI: c_uint = 0xDB  /* EEPROM ERASE 256B */;
// EEPROM Read Register

pub const NVM_INIT_CTRL_3: c_uint = 0x38;
pub const NVM_INIT_CTRL_3_LPLU: c_uint = 0x8;
pub const NVM_INIT_CTRL_3_D10GMP_PORT0: c_uint = 0x40;
pub const NVM_INIT_CTRL_3_D10GMP_PORT1: c_uint = 0x100;
pub const IXGBE_EEPROM_PAGE_SIZE_MAX: c_int = 128;

// Number of 5 microseconds we wait for EERD read and
// EERW write to complete
pub const IXGBE_EERD_EEWR_ATTEMPTS: c_int = 100000;

// # attempts we wait for flush update to complete
pub const IXGBE_FLUDONE_ATTEMPTS: c_int = 20000;

pub const IXGBE_PCIE_CTRL2: c_uint = 0x5   /* PCIe Control 2 Offset */;
pub const IXGBE_PCIE_CTRL2_DUMMY_ENABLE: c_uint = 0x8   /* Dummy Function Enable */;
pub const IXGBE_PCIE_CTRL2_LAN_DISABLE: c_uint = 0x2   /* LAN PCI Disable */;
pub const IXGBE_PCIE_CTRL2_DISABLE_SELECT: c_uint = 0x1   /* LAN Disable Select */;
pub const IXGBE_SAN_MAC_ADDR_PORT0_OFFSET: c_uint = 0x0;
pub const IXGBE_SAN_MAC_ADDR_PORT1_OFFSET: c_uint = 0x3;
pub const IXGBE_DEVICE_CAPS_ALLOW_ANY_SFP: c_uint = 0x1;
pub const IXGBE_DEVICE_CAPS_FCOE_OFFLOADS: c_uint = 0x2;

pub const IXGBE_FW_LESM_PARAMETERS_PTR: c_uint = 0x2;
pub const IXGBE_FW_LESM_STATE_1: c_uint = 0x1;
pub const IXGBE_FW_LESM_STATE_ENABLED: c_uint = 0x8000 /* LESM Enable bit */;
pub const IXGBE_FW_PASSTHROUGH_PATCH_CONFIG_PTR: c_uint = 0x4;
pub const IXGBE_FW_PATCH_VERSION_4: c_uint = 0x7;
pub const IXGBE_FCOE_IBA_CAPS_BLK_PTR: c_uint = 0x33 /* iSCSI/FCOE block */;
pub const IXGBE_FCOE_IBA_CAPS_FCOE: c_uint = 0x20 /* FCOE flags */;
pub const IXGBE_ISCSI_FCOE_BLK_PTR: c_uint = 0x17 /* iSCSI/FCOE block */;
pub const IXGBE_ISCSI_FCOE_FLAGS_OFFSET: c_uint = 0x0  /* FCOE flags */;
pub const IXGBE_ISCSI_FCOE_FLAGS_ENABLE: c_uint = 0x1  /* FCOE flags enable bit */;
pub const IXGBE_ALT_SAN_MAC_ADDR_BLK_PTR: c_uint = 0x27 /* Alt. SAN MAC block */;
pub const IXGBE_ALT_SAN_MAC_ADDR_CAPS_OFFSET: c_uint = 0x0 /* Alt. SAN MAC capability */;
pub const IXGBE_ALT_SAN_MAC_ADDR_PORT0_OFFSET: c_uint = 0x1 /* Alt. SAN MAC 0 offset */;
pub const IXGBE_ALT_SAN_MAC_ADDR_PORT1_OFFSET: c_uint = 0x4 /* Alt. SAN MAC 1 offset */;
pub const IXGBE_ALT_SAN_MAC_ADDR_WWNN_OFFSET: c_uint = 0x7 /* Alt. WWNN prefix offset */;
pub const IXGBE_ALT_SAN_MAC_ADDR_WWPN_OFFSET: c_uint = 0x8 /* Alt. WWPN prefix offset */;
pub const IXGBE_ALT_SAN_MAC_ADDR_CAPS_SANMAC: c_uint = 0x0 /* Alt. SAN MAC exists */;
pub const IXGBE_ALT_SAN_MAC_ADDR_CAPS_ALTWWN: c_uint = 0x1 /* Alt. WWN base exists */;
pub const IXGBE_DEVICE_CAPS_WOL_PORT0_1: c_uint = 0x4 /* WoL supported on ports 0 & 1 */;
pub const IXGBE_DEVICE_CAPS_WOL_PORT0: c_uint = 0x8 /* WoL supported on port 0 */;
pub const IXGBE_DEVICE_CAPS_WOL_MASK: c_uint = 0xC /* Mask for WoL capabilities */;
// PCI Bus Info
pub const IXGBE_PCI_DEVICE_STATUS: c_uint = 0xAA;
pub const IXGBE_PCI_DEVICE_STATUS_TRANSACTION_PENDING: c_uint = 0x0020;
pub const IXGBE_PCI_LINK_STATUS: c_uint = 0xB2;
pub const IXGBE_PCI_LINK_STATUS_E610: c_uint = 0x82;
pub const IXGBE_PCI_DEVICE_CONTROL2: c_uint = 0xC8;
pub const IXGBE_PCI_LINK_WIDTH: c_uint = 0x3F0;
pub const IXGBE_PCI_LINK_WIDTH_1: c_uint = 0x10;
pub const IXGBE_PCI_LINK_WIDTH_2: c_uint = 0x20;
pub const IXGBE_PCI_LINK_WIDTH_4: c_uint = 0x40;
pub const IXGBE_PCI_LINK_WIDTH_8: c_uint = 0x80;
pub const IXGBE_PCI_LINK_SPEED: c_uint = 0xF;
pub const IXGBE_PCI_LINK_SPEED_2500: c_uint = 0x1;
pub const IXGBE_PCI_LINK_SPEED_5000: c_uint = 0x2;
pub const IXGBE_PCI_LINK_SPEED_8000: c_uint = 0x3;
pub const IXGBE_PCI_HEADER_TYPE_REGISTER: c_uint = 0x0E;
pub const IXGBE_PCI_DEVICE_CONTROL2_16ms: c_uint = 0x0005;
pub const IXGBE_PCIDEVCTRL2_TIMEO_MASK: c_uint = 0xf;
pub const IXGBE_PCIDEVCTRL2_16_32ms_def: c_uint = 0x0;
pub const IXGBE_PCIDEVCTRL2_50_100us: c_uint = 0x1;
pub const IXGBE_PCIDEVCTRL2_1_2ms: c_uint = 0x2;
pub const IXGBE_PCIDEVCTRL2_16_32ms: c_uint = 0x5;
pub const IXGBE_PCIDEVCTRL2_65_130ms: c_uint = 0x6;
pub const IXGBE_PCIDEVCTRL2_260_520ms: c_uint = 0x9;
pub const IXGBE_PCIDEVCTRL2_1_2s: c_uint = 0xa;
pub const IXGBE_PCIDEVCTRL2_4_8s: c_uint = 0xd;
pub const IXGBE_PCIDEVCTRL2_17_34s: c_uint = 0xe;
// Number of 100 microseconds we wait for PCI Express primary disable
pub const IXGBE_PCI_PRIMARY_DISABLE_TIMEOUT: c_int = 800;
// RAH
pub const IXGBE_RAH_VIND_MASK: c_uint = 0x003C0000;
pub const IXGBE_RAH_VIND_SHIFT: c_int = 18;
pub const IXGBE_RAH_AV: c_uint = 0x80000000;
pub const IXGBE_CLEAR_VMDQ_ALL: c_uint = 0xFFFFFFFF;
// Header split receive
pub const IXGBE_RFCTL_ISCSI_DIS: c_uint = 0x00000001;
pub const IXGBE_RFCTL_ISCSI_DWC_MASK: c_uint = 0x0000003E;
pub const IXGBE_RFCTL_ISCSI_DWC_SHIFT: c_int = 1;
pub const IXGBE_RFCTL_RSC_DIS: c_uint = 0x00000020;
pub const IXGBE_RFCTL_NFSW_DIS: c_uint = 0x00000040;
pub const IXGBE_RFCTL_NFSR_DIS: c_uint = 0x00000080;
pub const IXGBE_RFCTL_NFS_VER_MASK: c_uint = 0x00000300;
pub const IXGBE_RFCTL_NFS_VER_SHIFT: c_int = 8;
pub const IXGBE_RFCTL_NFS_VER_2: c_int = 0;
pub const IXGBE_RFCTL_NFS_VER_3: c_int = 1;
pub const IXGBE_RFCTL_NFS_VER_4: c_int = 2;
pub const IXGBE_RFCTL_IPV6_DIS: c_uint = 0x00000400;
pub const IXGBE_RFCTL_IPV6_XSUM_DIS: c_uint = 0x00000800;
pub const IXGBE_RFCTL_IPFRSP_DIS: c_uint = 0x00004000;
pub const IXGBE_RFCTL_IPV6_EX_DIS: c_uint = 0x00010000;
pub const IXGBE_RFCTL_NEW_IPV6_EXT_DIS: c_uint = 0x00020000;
// Transmit Config masks
pub const IXGBE_TXDCTL_ENABLE: c_uint = 0x02000000 /* Enable specific Tx Queue */;
pub const IXGBE_TXDCTL_SWFLSH: c_uint = 0x04000000 /* Tx Desc. write-back flushing */;

// Enable short packet padding to 64 bytes
pub const IXGBE_TX_PAD_ENABLE: c_uint = 0x00000400;
pub const IXGBE_JUMBO_FRAME_ENABLE: c_uint = 0x00000004  /* Allow jumbo frames */;
// This allows for 16K packets + 4k for vlan
pub const IXGBE_MAX_FRAME_SZ: c_uint = 0x40040000;
pub const IXGBE_TDWBAL_HEAD_WB_ENABLE: c_uint = 0x1      /* Tx head write-back enable */;
pub const IXGBE_TDWBAL_SEQNUM_WB_ENABLE: c_uint = 0x2      /* Tx seq# write-back enable */;
// Receive Config masks
pub const IXGBE_RXCTRL_RXEN: c_uint = 0x00000001  /* Enable Receiver */;
pub const IXGBE_RXCTRL_DMBYPS: c_uint = 0x00000002  /* Descriptor Monitor Bypass */;
pub const IXGBE_RXDCTL_ENABLE: c_uint = 0x02000000  /* Enable specific Rx Queue */;
pub const IXGBE_RXDCTL_SWFLSH: c_uint = 0x04000000  /* Rx Desc. write-back flushing */;
pub const IXGBE_RXDCTL_RLPMLMASK: c_uint = 0x00003FFF  /* Only supported on the X540 */;
pub const IXGBE_RXDCTL_RLPML_EN: c_uint = 0x00008000;
pub const IXGBE_RXDCTL_VME: c_uint = 0x40000000  /* VLAN mode enable */;
pub const IXGBE_TSAUXC_EN_CLK: c_uint = 0x00000004;
pub const IXGBE_TSAUXC_SYNCLK: c_uint = 0x00000008;
pub const IXGBE_TSAUXC_SDP0_INT: c_uint = 0x00000040;
pub const IXGBE_TSAUXC_EN_TT0: c_uint = 0x00000001;
pub const IXGBE_TSAUXC_EN_TT1: c_uint = 0x00000002;
pub const IXGBE_TSAUXC_ST0: c_uint = 0x00000010;
pub const IXGBE_TSAUXC_DISABLE_SYSTIME: c_uint = 0x80000000;
pub const IXGBE_TSSDP_TS_SDP0_SEL_MASK: c_uint = 0x000000C0;
pub const IXGBE_TSSDP_TS_SDP0_CLK0: c_uint = 0x00000080;
pub const IXGBE_TSSDP_TS_SDP0_EN: c_uint = 0x00000100;
pub const IXGBE_TSYNCTXCTL_VALID: c_uint = 0x00000001 /* Tx timestamp valid */;
pub const IXGBE_TSYNCTXCTL_ENABLED: c_uint = 0x00000010 /* Tx timestamping enabled */;
pub const IXGBE_TSYNCRXCTL_VALID: c_uint = 0x00000001 /* Rx timestamp valid */;
pub const IXGBE_TSYNCRXCTL_TYPE_MASK: c_uint = 0x0000000E /* Rx type mask */;
pub const IXGBE_TSYNCRXCTL_TYPE_L2_V2: c_uint = 0x00;
pub const IXGBE_TSYNCRXCTL_TYPE_L4_V1: c_uint = 0x02;
pub const IXGBE_TSYNCRXCTL_TYPE_L2_L4_V2: c_uint = 0x04;
pub const IXGBE_TSYNCRXCTL_TYPE_ALL: c_uint = 0x08;
pub const IXGBE_TSYNCRXCTL_TYPE_EVENT_V2: c_uint = 0x0A;
pub const IXGBE_TSYNCRXCTL_ENABLED: c_uint = 0x00000010 /* Rx Timestamping enabled */;
pub const IXGBE_TSYNCRXCTL_TSIP_UT_EN: c_uint = 0x00800000 /* Rx Timestamp in Packet */;
pub const IXGBE_TSIM_TXTS: c_uint = 0x00000002;
pub const IXGBE_RXMTRL_V1_CTRLT_MASK: c_uint = 0x000000FF;
pub const IXGBE_RXMTRL_V1_SYNC_MSG: c_uint = 0x00;
pub const IXGBE_RXMTRL_V1_DELAY_REQ_MSG: c_uint = 0x01;
pub const IXGBE_RXMTRL_V1_FOLLOWUP_MSG: c_uint = 0x02;
pub const IXGBE_RXMTRL_V1_DELAY_RESP_MSG: c_uint = 0x03;
pub const IXGBE_RXMTRL_V1_MGMT_MSG: c_uint = 0x04;
pub const IXGBE_RXMTRL_V2_MSGID_MASK: c_uint = 0x0000FF00;
pub const IXGBE_RXMTRL_V2_SYNC_MSG: c_uint = 0x0000;
pub const IXGBE_RXMTRL_V2_DELAY_REQ_MSG: c_uint = 0x0100;
pub const IXGBE_RXMTRL_V2_PDELAY_REQ_MSG: c_uint = 0x0200;
pub const IXGBE_RXMTRL_V2_PDELAY_RESP_MSG: c_uint = 0x0300;
pub const IXGBE_RXMTRL_V2_FOLLOWUP_MSG: c_uint = 0x0800;
pub const IXGBE_RXMTRL_V2_DELAY_RESP_MSG: c_uint = 0x0900;
pub const IXGBE_RXMTRL_V2_PDELAY_FOLLOWUP_MSG: c_uint = 0x0A00;
pub const IXGBE_RXMTRL_V2_ANNOUNCE_MSG: c_uint = 0x0B00;
pub const IXGBE_RXMTRL_V2_SIGNALING_MSG: c_uint = 0x0C00;
pub const IXGBE_RXMTRL_V2_MGMT_MSG: c_uint = 0x0D00;
pub const IXGBE_FCTRL_SBP: c_uint = 0x00000002 /* Store Bad Packet */;
pub const IXGBE_FCTRL_TPE: c_uint = 0x00000080 /* Tag Promiscuous Ena*/;
pub const IXGBE_FCTRL_MPE: c_uint = 0x00000100 /* Multicast Promiscuous Ena*/;
pub const IXGBE_FCTRL_UPE: c_uint = 0x00000200 /* Unicast Promiscuous Ena */;
pub const IXGBE_FCTRL_BAM: c_uint = 0x00000400 /* Broadcast Accept Mode */;
pub const IXGBE_FCTRL_PMCF: c_uint = 0x00001000 /* Pass MAC Control Frames */;
pub const IXGBE_FCTRL_DPF: c_uint = 0x00002000 /* Discard Pause Frame */;
// Receive Priority Flow Control Enable
pub const IXGBE_FCTRL_RPFCE: c_uint = 0x00004000;
pub const IXGBE_FCTRL_RFCE: c_uint = 0x00008000 /* Receive Flow Control Ena */;
pub const IXGBE_MFLCN_PMCF: c_uint = 0x00000001 /* Pass MAC Control Frames */;
pub const IXGBE_MFLCN_DPF: c_uint = 0x00000002 /* Discard Pause Frame */;
pub const IXGBE_MFLCN_RPFCE: c_uint = 0x00000004 /* Receive Priority FC Enable */;
pub const IXGBE_MFLCN_RFCE: c_uint = 0x00000008 /* Receive FC Enable */;
pub const IXGBE_MFLCN_RPFCE_MASK: c_uint = 0x00000FF4 /* Receive FC Mask */;
pub const IXGBE_MFLCN_RPFCE_SHIFT: c_int = 4;
// Multiple Receive Queue Control
pub const IXGBE_MRQC_RSSEN: c_uint = 0x00000001  /* RSS Enable */;
pub const IXGBE_MRQC_MRQE_MASK: c_uint = 0xF /* Bits 3:0 */;
pub const IXGBE_MRQC_RT8TCEN: c_uint = 0x00000002 /* 8 TC no RSS */;
pub const IXGBE_MRQC_RT4TCEN: c_uint = 0x00000003 /* 4 TC no RSS */;
pub const IXGBE_MRQC_RTRSS8TCEN: c_uint = 0x00000004 /* 8 TC w/ RSS */;
pub const IXGBE_MRQC_RTRSS4TCEN: c_uint = 0x00000005 /* 4 TC w/ RSS */;
pub const IXGBE_MRQC_VMDQEN: c_uint = 0x00000008 /* VMDq2 64 pools no RSS */;
pub const IXGBE_MRQC_VMDQRSS32EN: c_uint = 0x0000000A /* VMDq2 32 pools w/ RSS */;
pub const IXGBE_MRQC_VMDQRSS64EN: c_uint = 0x0000000B /* VMDq2 64 pools w/ RSS */;
pub const IXGBE_MRQC_VMDQRT8TCEN: c_uint = 0x0000000C /* VMDq2/RT 16 pool 8 TC */;
pub const IXGBE_MRQC_VMDQRT4TCEN: c_uint = 0x0000000D /* VMDq2/RT 32 pool 4 TC */;
pub const IXGBE_MRQC_RSS_FIELD_MASK: c_uint = 0xFFFF0000;
pub const IXGBE_MRQC_RSS_FIELD_IPV4_TCP: c_uint = 0x00010000;
pub const IXGBE_MRQC_RSS_FIELD_IPV4: c_uint = 0x00020000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_EX_TCP: c_uint = 0x00040000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_EX: c_uint = 0x00080000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6: c_uint = 0x00100000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_TCP: c_uint = 0x00200000;
pub const IXGBE_MRQC_RSS_FIELD_IPV4_UDP: c_uint = 0x00400000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_UDP: c_uint = 0x00800000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_EX_UDP: c_uint = 0x01000000;
pub const IXGBE_MRQC_MULTIPLE_RSS: c_uint = 0x00002000;
pub const IXGBE_MRQC_L3L4TXSWEN: c_uint = 0x00008000;
pub const IXGBE_FWSM_TS_ENABLED: c_uint = 0x1;
// Queue Drop Enable
pub const IXGBE_QDE_ENABLE: c_uint = 0x00000001;
pub const IXGBE_QDE_HIDE_VLAN: c_uint = 0x00000002;
pub const IXGBE_QDE_IDX_MASK: c_uint = 0x00007F00;
pub const IXGBE_QDE_IDX_SHIFT: c_int = 8;
pub const IXGBE_QDE_WRITE: c_uint = 0x00010000;
pub const IXGBE_TXD_POPTS_IXSM: c_uint = 0x01       /* Insert IP checksum */;
pub const IXGBE_TXD_POPTS_TXSM: c_uint = 0x02       /* Insert TCP/UDP checksum */;
pub const IXGBE_TXD_CMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const IXGBE_TXD_CMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const IXGBE_TXD_CMD_IC: c_uint = 0x04000000 /* Insert Checksum */;
pub const IXGBE_TXD_CMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const IXGBE_TXD_CMD_DEXT: c_uint = 0x20000000 /* Descriptor extension (0 = legacy) */;
pub const IXGBE_TXD_CMD_VLE: c_uint = 0x40000000 /* Add VLAN tag */;
pub const IXGBE_TXD_STAT_DD: c_uint = 0x00000001 /* Descriptor Done */;
// Multiple Transmit Queue Command Register
pub const IXGBE_MTQC_RT_ENA: c_uint = 0x1 /* DCB Enable */;
pub const IXGBE_MTQC_VT_ENA: c_uint = 0x2 /* VMDQ2 Enable */;
pub const IXGBE_MTQC_NUM_TC_OR_Q: c_uint = 0xC /* Number of TCs or TxQs per pool */;
pub const IXGBE_MTQC_64Q_1PB: c_uint = 0x0 /* 64 queues 1 pack buffer */;
pub const IXGBE_MTQC_32VF: c_uint = 0x8 /* 4 TX Queues per pool w/32VF's */;
pub const IXGBE_MTQC_64VF: c_uint = 0x4 /* 2 TX Queues per pool w/64VF's */;
pub const IXGBE_MTQC_8TC_8TQ: c_uint = 0xC /* 8 TC if RT_ENA or 8 TQ if VT_ENA */;
pub const IXGBE_MTQC_4TC_4TQ: c_uint = 0x8 /* 4 TC if RT_ENA or 4 TQ if VT_ENA */;
// Receive Descriptor bit definitions
pub const IXGBE_RXD_STAT_DD: c_uint = 0x01    /* Descriptor Done */;
pub const IXGBE_RXD_STAT_EOP: c_uint = 0x02    /* End of Packet */;
pub const IXGBE_RXD_STAT_FLM: c_uint = 0x04    /* FDir Match */;
pub const IXGBE_RXD_STAT_VP: c_uint = 0x08    /* IEEE VLAN Packet */;
pub const IXGBE_RXDADV_NEXTP_MASK: c_uint = 0x000FFFF0 /* Next Descriptor Index */;
pub const IXGBE_RXDADV_NEXTP_SHIFT: c_uint = 0x00000004;
pub const IXGBE_RXD_STAT_UDPCS: c_uint = 0x10    /* UDP xsum calculated */;
pub const IXGBE_RXD_STAT_L4CS: c_uint = 0x20    /* L4 xsum calculated */;
pub const IXGBE_RXD_STAT_IPCS: c_uint = 0x40    /* IP xsum calculated */;
pub const IXGBE_RXD_STAT_PIF: c_uint = 0x80    /* passed in-exact filter */;
pub const IXGBE_RXD_STAT_CRCV: c_uint = 0x100   /* Speculative CRC Valid */;
pub const IXGBE_RXD_STAT_OUTERIPCS: c_uint = 0x100 /* Cloud IP xsum calculated */;
pub const IXGBE_RXD_STAT_VEXT: c_uint = 0x200   /* 1st VLAN found */;
pub const IXGBE_RXD_STAT_UDPV: c_uint = 0x400   /* Valid UDP checksum */;
pub const IXGBE_RXD_STAT_DYNINT: c_uint = 0x800   /* Pkt caused INT via DYNINT */;
pub const IXGBE_RXD_STAT_LLINT: c_uint = 0x800   /* Pkt caused Low Latency Interrupt */;
pub const IXGBE_RXD_STAT_TSIP: c_uint = 0x08000 /* Time Stamp in packet buffer */;
pub const IXGBE_RXD_STAT_TS: c_uint = 0x10000 /* Time Stamp */;
pub const IXGBE_RXD_STAT_SECP: c_uint = 0x20000 /* Security Processing */;
pub const IXGBE_RXD_STAT_LB: c_uint = 0x40000 /* Loopback Status */;
pub const IXGBE_RXD_STAT_ACK: c_uint = 0x8000  /* ACK Packet indication */;
pub const IXGBE_RXD_ERR_CE: c_uint = 0x01    /* CRC Error */;
pub const IXGBE_RXD_ERR_LE: c_uint = 0x02    /* Length Error */;
pub const IXGBE_RXD_ERR_PE: c_uint = 0x08    /* Packet Error */;
pub const IXGBE_RXD_ERR_OSE: c_uint = 0x10    /* Oversize Error */;
pub const IXGBE_RXD_ERR_USE: c_uint = 0x20    /* Undersize Error */;
pub const IXGBE_RXD_ERR_TCPE: c_uint = 0x40    /* TCP/UDP Checksum Error */;
pub const IXGBE_RXD_ERR_IPE: c_uint = 0x80    /* IP Checksum Error */;
pub const IXGBE_RXDADV_ERR_MASK: c_uint = 0xfff00000 /* RDESC.ERRORS mask */;

pub const IXGBE_RXDADV_ERR_OUTERIPER: c_uint = 0x04000000 /* CRC IP Header error */;
pub const IXGBE_RXDADV_ERR_FCEOFE: c_uint = 0x80000000 /* FCoEFe/IPE */;
pub const IXGBE_RXDADV_ERR_FCERR: c_uint = 0x00700000 /* FCERR/FDIRERR */;
pub const IXGBE_RXDADV_ERR_FDIR_LEN: c_uint = 0x00100000 /* FDIR Length error */;
pub const IXGBE_RXDADV_ERR_FDIR_DROP: c_uint = 0x00200000 /* FDIR Drop error */;
pub const IXGBE_RXDADV_ERR_FDIR_COLL: c_uint = 0x00400000 /* FDIR Collision error */;
pub const IXGBE_RXDADV_ERR_HBO: c_uint = 0x00800000 /*Header Buffer Overflow */;
pub const IXGBE_RXDADV_ERR_CE: c_uint = 0x01000000 /* CRC Error */;
pub const IXGBE_RXDADV_ERR_LE: c_uint = 0x02000000 /* Length Error */;
pub const IXGBE_RXDADV_ERR_PE: c_uint = 0x08000000 /* Packet Error */;
pub const IXGBE_RXDADV_ERR_OSE: c_uint = 0x10000000 /* Oversize Error */;
pub const IXGBE_RXDADV_ERR_IPSEC_INV_PROTOCOL: c_uint = 0x08000000 /* overlap ERR_PE  */;
pub const IXGBE_RXDADV_ERR_IPSEC_INV_LENGTH: c_uint = 0x10000000 /* overlap ERR_OSE */;
pub const IXGBE_RXDADV_ERR_IPSEC_AUTH_FAILED: c_uint = 0x18000000;
pub const IXGBE_RXDADV_ERR_USE: c_uint = 0x20000000 /* Undersize Error */;
pub const IXGBE_RXDADV_ERR_TCPE: c_uint = 0x40000000 /* TCP/UDP Checksum Error */;
pub const IXGBE_RXDADV_ERR_IPE: c_uint = 0x80000000 /* IP Checksum Error */;
pub const IXGBE_RXD_VLAN_ID_MASK: c_uint = 0x0FFF  /* VLAN ID is in lower 12 bits */;
pub const IXGBE_RXD_PRI_MASK: c_uint = 0xE000  /* Priority is in upper 3 bits */;
pub const IXGBE_RXD_PRI_SHIFT: c_int = 13;
pub const IXGBE_RXD_CFI_MASK: c_uint = 0x1000  /* CFI is bit 12 */;
pub const IXGBE_RXD_CFI_SHIFT: c_int = 12;

pub const IXGBE_RXDADV_STAT_MASK: c_uint = 0x000fffff /* Stat/NEXTP: bit 0-19 */;
pub const IXGBE_RXDADV_STAT_FCEOFS: c_uint = 0x00000040 /* FCoE EOF/SOF Stat */;
pub const IXGBE_RXDADV_STAT_FCSTAT: c_uint = 0x00000030 /* FCoE Pkt Stat */;
pub const IXGBE_RXDADV_STAT_FCSTAT_NOMTCH: c_uint = 0x00000000 /* 00: No Ctxt Match */;
pub const IXGBE_RXDADV_STAT_FCSTAT_NODDP: c_uint = 0x00000010 /* 01: Ctxt w/o DDP */;
pub const IXGBE_RXDADV_STAT_FCSTAT_FCPRSP: c_uint = 0x00000020 /* 10: Recv. FCP_RSP */;
pub const IXGBE_RXDADV_STAT_FCSTAT_DDP: c_uint = 0x00000030 /* 11: Ctxt w/ DDP */;
pub const IXGBE_RXDADV_STAT_TS: c_uint = 0x00010000 /* IEEE 1588 Time Stamp */;
pub const IXGBE_RXDADV_STAT_SECP: c_uint = 0x00020000 /* IPsec/MACsec pkt found */;
// PSRTYPE bit definitions
pub const IXGBE_PSRTYPE_TCPHDR: c_uint = 0x00000010;
pub const IXGBE_PSRTYPE_UDPHDR: c_uint = 0x00000020;
pub const IXGBE_PSRTYPE_IPV4HDR: c_uint = 0x00000100;
pub const IXGBE_PSRTYPE_IPV6HDR: c_uint = 0x00000200;
pub const IXGBE_PSRTYPE_L2HDR: c_uint = 0x00001000;
// SRRCTL bit definitions

pub const IXGBE_SRRCTL_RDMTS_SHIFT: c_int = 22;
pub const IXGBE_SRRCTL_RDMTS_MASK: c_uint = 0x01C00000;
pub const IXGBE_SRRCTL_DROP_EN: c_uint = 0x10000000;
pub const IXGBE_SRRCTL_BSIZEPKT_MASK: c_uint = 0x0000007F;
pub const IXGBE_SRRCTL_BSIZEHDR_MASK: c_uint = 0x00003F00;
pub const IXGBE_SRRCTL_DESCTYPE_LEGACY: c_uint = 0x00000000;
pub const IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF: c_uint = 0x02000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_SPLIT: c_uint = 0x04000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_REPLICATION_LARGE_PKT: c_uint = 0x08000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_SPLIT_ALWAYS: c_uint = 0x0A000000;
pub const IXGBE_SRRCTL_DESCTYPE_MASK: c_uint = 0x0E000000;
pub const IXGBE_RXDPS_HDRSTAT_HDRSP: c_uint = 0x00008000;
pub const IXGBE_RXDPS_HDRSTAT_HDRLEN_MASK: c_uint = 0x000003FF;
pub const IXGBE_RXDADV_RSSTYPE_MASK: c_uint = 0x0000000F;
pub const IXGBE_RXDADV_PKTTYPE_MASK: c_uint = 0x0000FFF0;
pub const IXGBE_RXDADV_PKTTYPE_MASK_EX: c_uint = 0x0001FFF0;
pub const IXGBE_RXDADV_HDRBUFLEN_MASK: c_uint = 0x00007FE0;
pub const IXGBE_RXDADV_RSCCNT_MASK: c_uint = 0x001E0000;
pub const IXGBE_RXDADV_RSCCNT_SHIFT: c_int = 17;
pub const IXGBE_RXDADV_HDRBUFLEN_SHIFT: c_int = 5;
pub const IXGBE_RXDADV_SPLITHEADER_EN: c_uint = 0x00001000;
pub const IXGBE_RXDADV_SPH: c_uint = 0x8000;
// RSS Hash results
pub const IXGBE_RXDADV_RSSTYPE_NONE: c_uint = 0x00000000;
pub const IXGBE_RXDADV_RSSTYPE_IPV4_TCP: c_uint = 0x00000001;
pub const IXGBE_RXDADV_RSSTYPE_IPV4: c_uint = 0x00000002;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_TCP: c_uint = 0x00000003;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_EX: c_uint = 0x00000004;
pub const IXGBE_RXDADV_RSSTYPE_IPV6: c_uint = 0x00000005;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_TCP_EX: c_uint = 0x00000006;
pub const IXGBE_RXDADV_RSSTYPE_IPV4_UDP: c_uint = 0x00000007;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_UDP: c_uint = 0x00000008;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_UDP_EX: c_uint = 0x00000009;
// RSS Packet Types as indicated in the receive descriptor.
pub const IXGBE_RXDADV_PKTTYPE_NONE: c_uint = 0x00000000;
pub const IXGBE_RXDADV_PKTTYPE_IPV4: c_uint = 0x00000010 /* IPv4 hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_IPV4_EX: c_uint = 0x00000020 /* IPv4 hdr + extensions */;
pub const IXGBE_RXDADV_PKTTYPE_IPV6: c_uint = 0x00000040 /* IPv6 hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_IPV6_EX: c_uint = 0x00000080 /* IPv6 hdr + extensions */;
pub const IXGBE_RXDADV_PKTTYPE_TCP: c_uint = 0x00000100 /* TCP hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_UDP: c_uint = 0x00000200 /* UDP hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_SCTP: c_uint = 0x00000400 /* SCTP hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_NFS: c_uint = 0x00000800 /* NFS hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_VXLAN: c_uint = 0x00000800 /* VXLAN hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_TUNNEL: c_uint = 0x00010000 /* Tunnel type */;
pub const IXGBE_RXDADV_PKTTYPE_IPSEC_ESP: c_uint = 0x00001000 /* IPSec ESP */;
pub const IXGBE_RXDADV_PKTTYPE_IPSEC_AH: c_uint = 0x00002000 /* IPSec AH */;
pub const IXGBE_RXDADV_PKTTYPE_LINKSEC: c_uint = 0x00004000 /* LinkSec Encap */;
pub const IXGBE_RXDADV_PKTTYPE_ETQF: c_uint = 0x00008000 /* PKTTYPE is ETQF index */;
pub const IXGBE_RXDADV_PKTTYPE_ETQF_MASK: c_uint = 0x00000070 /* ETQF has 8 indices */;

// Masks to determine if packets should be dropped due to frame errors

// Multicast bit mask
pub const IXGBE_MCSTCTRL_MFE: c_uint = 0x4;
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const IXGBE_REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const IXGBE_REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const IXGBE_REQ_TX_BUFFER_GRANULARITY: c_int = 1024;
// Vlan-specific macros
pub const IXGBE_RX_DESC_SPECIAL_VLAN_MASK: c_uint = 0x0FFF /* VLAN ID in lower 12 bits */;
pub const IXGBE_RX_DESC_SPECIAL_PRI_MASK: c_uint = 0xE000 /* Priority in upper 3 bits */;
pub const IXGBE_RX_DESC_SPECIAL_PRI_SHIFT: c_uint = 0x000D /* Priority in upper 3 of 16 */;

// SR-IOV specific macros

// Translated register #defines

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_fdir_pballoc_type {
    IXGBE_FDIR_PBALLOC_NONE = 0,
    IXGBE_FDIR_PBALLOC_64K  = 1,
    IXGBE_FDIR_PBALLOC_128K = 2,
    IXGBE_FDIR_PBALLOC_256K = 3,
}

pub const IXGBE_FDIR_PBALLOC_SIZE_SHIFT: c_int = 16;
// Flow Director register values
pub const IXGBE_FDIRCTRL_PBALLOC_64K: c_uint = 0x00000001;
pub const IXGBE_FDIRCTRL_PBALLOC_128K: c_uint = 0x00000002;
pub const IXGBE_FDIRCTRL_PBALLOC_256K: c_uint = 0x00000003;
pub const IXGBE_FDIRCTRL_INIT_DONE: c_uint = 0x00000008;
pub const IXGBE_FDIRCTRL_PERFECT_MATCH: c_uint = 0x00000010;
pub const IXGBE_FDIRCTRL_REPORT_STATUS: c_uint = 0x00000020;
pub const IXGBE_FDIRCTRL_REPORT_STATUS_ALWAYS: c_uint = 0x00000080;
pub const IXGBE_FDIRCTRL_DROP_Q_SHIFT: c_int = 8;
pub const IXGBE_FDIRCTRL_FLEX_SHIFT: c_int = 16;
pub const IXGBE_FDIRCTRL_DROP_NO_MATCH: c_uint = 0x00008000;
pub const IXGBE_FDIRCTRL_FILTERMODE_SHIFT: c_int = 21;
pub const IXGBE_FDIRCTRL_FILTERMODE_MACVLAN: c_uint = 0x0001 /* bit 23:21, 001b */;
pub const IXGBE_FDIRCTRL_FILTERMODE_CLOUD: c_uint = 0x0002 /* bit 23:21, 010b */;
pub const IXGBE_FDIRCTRL_SEARCHLIM: c_uint = 0x00800000;
pub const IXGBE_FDIRCTRL_MAX_LENGTH_SHIFT: c_int = 24;
pub const IXGBE_FDIRCTRL_FULL_THRESH_MASK: c_uint = 0xF0000000;
pub const IXGBE_FDIRCTRL_FULL_THRESH_SHIFT: c_int = 28;
pub const IXGBE_FDIRTCPM_DPORTM_SHIFT: c_int = 16;
pub const IXGBE_FDIRUDPM_DPORTM_SHIFT: c_int = 16;
pub const IXGBE_FDIRIP6M_DIPM_SHIFT: c_int = 16;
pub const IXGBE_FDIRM_VLANID: c_uint = 0x00000001;
pub const IXGBE_FDIRM_VLANP: c_uint = 0x00000002;
pub const IXGBE_FDIRM_POOL: c_uint = 0x00000004;
pub const IXGBE_FDIRM_L4P: c_uint = 0x00000008;
pub const IXGBE_FDIRM_FLEX: c_uint = 0x00000010;
pub const IXGBE_FDIRM_DIPv6: c_uint = 0x00000020;
pub const IXGBE_FDIRFREE_FREE_MASK: c_uint = 0xFFFF;
pub const IXGBE_FDIRFREE_FREE_SHIFT: c_int = 0;
pub const IXGBE_FDIRFREE_COLL_MASK: c_uint = 0x7FFF0000;
pub const IXGBE_FDIRFREE_COLL_SHIFT: c_int = 16;
pub const IXGBE_FDIRLEN_MAXLEN_MASK: c_uint = 0x3F;
pub const IXGBE_FDIRLEN_MAXLEN_SHIFT: c_int = 0;
pub const IXGBE_FDIRLEN_MAXHASH_MASK: c_uint = 0x7FFF0000;
pub const IXGBE_FDIRLEN_MAXHASH_SHIFT: c_int = 16;
pub const IXGBE_FDIRUSTAT_ADD_MASK: c_uint = 0xFFFF;
pub const IXGBE_FDIRUSTAT_ADD_SHIFT: c_int = 0;
pub const IXGBE_FDIRUSTAT_REMOVE_MASK: c_uint = 0xFFFF0000;
pub const IXGBE_FDIRUSTAT_REMOVE_SHIFT: c_int = 16;
pub const IXGBE_FDIRFSTAT_FADD_MASK: c_uint = 0x00FF;
pub const IXGBE_FDIRFSTAT_FADD_SHIFT: c_int = 0;
pub const IXGBE_FDIRFSTAT_FREMOVE_MASK: c_uint = 0xFF00;
pub const IXGBE_FDIRFSTAT_FREMOVE_SHIFT: c_int = 8;
pub const IXGBE_FDIRPORT_DESTINATION_SHIFT: c_int = 16;
pub const IXGBE_FDIRVLAN_FLEX_SHIFT: c_int = 16;
pub const IXGBE_FDIRHASH_BUCKET_VALID_SHIFT: c_int = 15;
pub const IXGBE_FDIRHASH_SIG_SW_INDEX_SHIFT: c_int = 16;
pub const IXGBE_FDIRCMD_CMD_MASK: c_uint = 0x00000003;
pub const IXGBE_FDIRCMD_CMD_ADD_FLOW: c_uint = 0x00000001;
pub const IXGBE_FDIRCMD_CMD_REMOVE_FLOW: c_uint = 0x00000002;
pub const IXGBE_FDIRCMD_CMD_QUERY_REM_FILT: c_uint = 0x00000003;
pub const IXGBE_FDIRCMD_FILTER_VALID: c_uint = 0x00000004;
pub const IXGBE_FDIRCMD_FILTER_UPDATE: c_uint = 0x00000008;
pub const IXGBE_FDIRCMD_IPv6DMATCH: c_uint = 0x00000010;
pub const IXGBE_FDIRCMD_L4TYPE_UDP: c_uint = 0x00000020;
pub const IXGBE_FDIRCMD_L4TYPE_TCP: c_uint = 0x00000040;
pub const IXGBE_FDIRCMD_L4TYPE_SCTP: c_uint = 0x00000060;
pub const IXGBE_FDIRCMD_IPV6: c_uint = 0x00000080;
pub const IXGBE_FDIRCMD_CLEARHT: c_uint = 0x00000100;
pub const IXGBE_FDIRCMD_DROP: c_uint = 0x00000200;
pub const IXGBE_FDIRCMD_INT: c_uint = 0x00000400;
pub const IXGBE_FDIRCMD_LAST: c_uint = 0x00000800;
pub const IXGBE_FDIRCMD_COLLISION: c_uint = 0x00001000;
pub const IXGBE_FDIRCMD_QUEUE_EN: c_uint = 0x00008000;
pub const IXGBE_FDIRCMD_FLOW_TYPE_SHIFT: c_int = 5;
pub const IXGBE_FDIRCMD_RX_QUEUE_SHIFT: c_int = 16;
pub const IXGBE_FDIRCMD_RX_TUNNEL_FILTER_SHIFT: c_int = 23;
pub const IXGBE_FDIRCMD_VT_POOL_SHIFT: c_int = 24;
pub const IXGBE_FDIR_INIT_DONE_POLL: c_int = 10;
pub const IXGBE_FDIRCMD_CMD_POLL: c_int = 10;
pub const IXGBE_FDIRCMD_TUNNEL_FILTER: c_uint = 0x00800000;
pub const IXGBE_FDIR_DROP_QUEUE: c_int = 127;
// Manageablility Host Interface defines

// CEM Support
pub const FW_CEM_HDR_LEN: c_uint = 0x4;
pub const FW_CEM_CMD_DRIVER_INFO: c_uint = 0xDD;
pub const FW_CEM_CMD_DRIVER_INFO_LEN: c_uint = 0x5;
pub const FW_CEM_CMD_RESERVED: c_uint = 0x0;
pub const FW_CEM_UNUSED_VER: c_uint = 0x0;
pub const FW_CEM_MAX_RETRIES: c_int = 3;
pub const FW_CEM_RESP_STATUS_SUCCESS: c_uint = 0x1;

pub const FW_READ_SHADOW_RAM_CMD: c_uint = 0x31;
pub const FW_READ_SHADOW_RAM_LEN: c_uint = 0x6;
pub const FW_WRITE_SHADOW_RAM_CMD: c_uint = 0x33;
pub const FW_WRITE_SHADOW_RAM_LEN: c_uint = 0xA /* 8 plus 1 WORD to write */;
pub const FW_SHADOW_RAM_DUMP_CMD: c_uint = 0x36;
pub const FW_SHADOW_RAM_DUMP_LEN: c_int = 0;
pub const FW_DEFAULT_CHECKSUM: c_uint = 0xFF /* checksum always 0xFF */;
pub const FW_NVM_DATA_OFFSET: c_int = 3;
pub const FW_MAX_READ_BUFFER_SIZE: c_int = 1024;
pub const FW_DISABLE_RXEN_CMD: c_uint = 0xDE;
pub const FW_DISABLE_RXEN_LEN: c_uint = 0x1;
pub const FW_PHY_MGMT_REQ_CMD: c_uint = 0x20;
pub const FW_PHY_TOKEN_REQ_CMD: c_uint = 0x0A;
pub const FW_PHY_TOKEN_REQ_LEN: c_int = 2;
pub const FW_PHY_TOKEN_REQ: c_int = 0;
pub const FW_PHY_TOKEN_REL: c_int = 1;
pub const FW_PHY_TOKEN_OK: c_int = 1;
pub const FW_PHY_TOKEN_RETRY: c_uint = 0x80;

pub const FW_INT_PHY_REQ_CMD: c_uint = 0xB;
pub const FW_INT_PHY_REQ_LEN: c_int = 10;
pub const FW_INT_PHY_REQ_READ: c_int = 0;
pub const FW_INT_PHY_REQ_WRITE: c_int = 1;
pub const FW_PHY_ACT_REQ_CMD: c_int = 5;
pub const FW_PHY_ACT_DATA_COUNT: c_int = 4;

pub const FW_PHY_ACT_INIT_PHY: c_int = 1;
pub const FW_PHY_ACT_SETUP_LINK: c_int = 2;

pub const FW_PHY_ACT_SETUP_LINK_PAUSE_SHIFT: c_int = 16;

pub const FW_PHY_ACT_GET_LINK_INFO: c_int = 3;

pub const FW_PHY_ACT_FORCE_LINK_DOWN: c_int = 4;

pub const FW_PHY_ACT_PHY_SW_RESET: c_int = 5;
pub const FW_PHY_ACT_PHY_HW_RESET: c_int = 6;
pub const FW_PHY_ACT_GET_PHY_INFO: c_int = 7;
pub const FW_PHY_ACT_UD_2: c_uint = 0x1002;

pub const FW_PHY_ACT_RETRIES: c_int = 50;
pub const FW_PHY_INFO_SPEED_MASK: c_uint = 0xFFFu;
pub const FW_PHY_INFO_ID_HI_MASK: c_uint = 0xFFFF0000u;
pub const FW_PHY_INFO_ID_LO_MASK: c_uint = 0x0000FFFFu;
// There are only 3 options for VFs creation on this device:
// 16 VFs pool with 8 queues each
// 32 VFs pool with 4 queues each
// 64 VFs pool with 2 queues each
//
// That means reading some VF registers that map VF to queue depending on
// chosen option. Define values that help dealing with each scenario.
//
// Number of queues based on VFs pool
pub const IXGBE_16VFS_QUEUES: c_int = 8;
pub const IXGBE_32VFS_QUEUES: c_int = 4;
pub const IXGBE_64VFS_QUEUES: c_int = 2;
// Mask for getting queues bits based on VFs pool

// Convert queue index to register number.
// We have 4 registers with 32 queues in each.
//
pub const IXGBE_QUEUES_PER_REG: c_int = 32;
pub const IXGBE_QUEUES_REG_AMOUNT: c_int = 4;
// Host Interface Command Structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_hdr {
    pub cmd: u8,
    pub buf_len: u8,
    pub cmd_resv: u8,
    pub ret_status: u8,
    pub cmd_or_resp: },
    pub checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_hdr2_req {
    pub cmd: u8,
    pub buf_lenh: u8,
    pub buf_lenl: u8,
    pub checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_hdr2_rsp {
    pub cmd: u8,
    pub buf_lenl: u8,
    pub /: *mut *mut u8 buf_lenh_status; / 7-5: high bits of buf_len, 4-0: status,
    pub checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_hic_hdr2 {
    pub buf: [u32; 1],
    pub req: ixgbe_hic_hdr2_req,
    pub rsp: ixgbe_hic_hdr2_rsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_drv_info {
    pub hdr: ixgbe_hic_hdr,
    pub port_num: u8,
    pub ver_sub: u8,
    pub ver_build: u8,
    pub ver_min: u8,
    pub ver_maj: u8,
    pub /: *mut *mut u8 pad; / end spacing to ensure length is mult. of dword,
    pub /: *mut *mut u16 pad2; / end spacing to ensure length is mult. of dword2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_drv_info2 {
    pub hdr: ixgbe_hic_hdr,
    pub port_num: u8,
    pub ver_sub: u8,
    pub ver_build: u8,
    pub ver_min: u8,
    pub ver_maj: u8,
    pub driver_string: [c_char; FW_CEM_DRIVER_VERSION_SIZE],
}

// These need to be dword aligned
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_read_shadow_ram {
    pub hdr: ixgbe_hic_hdr2,
    pub address: u32,
    pub length: u16,
    pub pad2: u16,
    pub data: u16,
    pub pad3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_write_shadow_ram {
    pub hdr: ixgbe_hic_hdr2,
    pub address: __be32,
    pub length: __be16,
    pub pad2: u16,
    pub data: u16,
    pub pad3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_disable_rxen {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub pad2: u8,
    pub pad3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_phy_token_req {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub command_type: u8,
    pub pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_internal_phy_req {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub command_type: u8,
    pub address: __be16,
    pub rsv1: u16,
    pub write_data: __be32,
    pub pad: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_internal_phy_resp {
    pub hdr: ixgbe_hic_hdr,
    pub read_data: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_phy_activity_req {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub pad: u8,
    pub activity_id: __le16,
    pub data: [__be32; FW_PHY_ACT_DATA_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_phy_activity_resp {
    pub hdr: ixgbe_hic_hdr,
    pub data: [__be32; FW_PHY_ACT_DATA_COUNT],
}

// Transmit Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_len: __le32,
    pub olinfo_status: __le32,
    pub read: },
    pub /: *mut *mut __le64 rsvd; / Reserved,
    pub nxtseq_seed: __le32,
    pub status: __le32,
    pub wb: },
}

// Receive Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
    pub data: __le32,
    pub /: *mut *mut __le16 pkt_info; / RSS, Pkt type,
    pub /: *mut *mut __le16 hdr_info; / Splithdr, hdrlen,
    pub hs_rss: },
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub /: *mut *mut __le16 length; / Packet length,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub upper: },
    pub /: *mut *mut } wb; / writeback,
}

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_adv_tx_context_desc {
    pub vlan_macip_lens: __le32,
    pub fceof_saidx: __le32,
    pub type_tucmd_mlhl: __le32,
    pub mss_l4len_idx: __le32,
}

// Adv Transmit Descriptor Config Masks
pub const IXGBE_ADVTXD_DTALEN_MASK: c_uint = 0x0000FFFF /* Data buf length(bytes) */;
pub const IXGBE_ADVTXD_MAC_LINKSEC: c_uint = 0x00040000 /* Insert LinkSec */;
pub const IXGBE_ADVTXD_MAC_TSTAMP: c_uint = 0x00080000 /* IEEE 1588 Time Stamp */;
pub const IXGBE_ADVTXD_IPSEC_SA_INDEX_MASK: c_uint = 0x000003FF /* IPSec SA index */;
pub const IXGBE_ADVTXD_IPSEC_ESP_LEN_MASK: c_uint = 0x000001FF /* IPSec ESP length */;
pub const IXGBE_ADVTXD_DTYP_MASK: c_uint = 0x00F00000 /* DTYP mask */;
pub const IXGBE_ADVTXD_DTYP_CTXT: c_uint = 0x2 /* Advanced Context Desc */;
pub const IXGBE_ADVTXD_DTYP_DATA: c_uint = 0x00300000 /* Advanced Data Descriptor */;

pub const IXGBE_ADVTXD_DCMD_DDTYP_ISCSI: c_uint = 0x10000000    /* DDP hdr type or iSCSI */;

pub const IXGBE_ADVTXD_DCMD_TSE: c_uint = 0x80000000 /* TCP Seg enable */;

pub const IXGBE_ADVTXD_STAT_SN_CRC: c_uint = 0x00000002 /* NXTSEQ/SEED pres in WB */;
pub const IXGBE_ADVTXD_STAT_RSV: c_uint = 0x0000000C /* STA Reserved */;

pub const IXGBE_ADVTXD_CC: c_uint = 0x00000080 /* Check Context */;

pub const IXGBE_ADVTXD_POPTS_IPSEC: c_uint = 0x00000400 /* IPSec offload request */;
pub const IXGBE_ADVTXD_POPTS_ISCO_1ST: c_uint = 0x00000000 /* 1st TSO of iSCSI PDU */;
pub const IXGBE_ADVTXD_POPTS_ISCO_MDL: c_uint = 0x00000800 /* Middle TSO of iSCSI PDU */;
pub const IXGBE_ADVTXD_POPTS_ISCO_LAST: c_uint = 0x00001000 /* Last TSO of iSCSI PDU */;
pub const IXGBE_ADVTXD_POPTS_ISCO_FULL: c_uint = 0x00001800 /* 1st&Last TSO-full iSCSI PDU */;
pub const IXGBE_ADVTXD_POPTS_RSV: c_uint = 0x00002000 /* POPTS Reserved */;

pub const IXGBE_ADVTXD_TUCMD_IPV4: c_uint = 0x00000400  /* IP Packet Type: 1=IPv4 */;
pub const IXGBE_ADVTXD_TUCMD_IPV6: c_uint = 0x00000000  /* IP Packet Type: 0=IPv6 */;
pub const IXGBE_ADVTXD_TUCMD_L4T_UDP: c_uint = 0x00000000  /* L4 Packet TYPE of UDP */;
pub const IXGBE_ADVTXD_TUCMD_L4T_TCP: c_uint = 0x00000800  /* L4 Packet TYPE of TCP */;
pub const IXGBE_ADVTXD_TUCMD_L4T_SCTP: c_uint = 0x00001000  /* L4 Packet TYPE of SCTP */;
pub const IXGBE_ADVTXD_TUCMD_L4T_RSV: c_uint = 0x00001800 /* RSV L4 Packet TYPE */;
pub const IXGBE_ADVTXD_TUCMD_MKRREQ: c_uint = 0x00002000 /*Req requires Markers and CRC*/;
pub const IXGBE_ADVTXD_TUCMD_IPSEC_TYPE_ESP: c_uint = 0x00002000 /* IPSec Type ESP */;
pub const IXGBE_ADVTXD_TUCMD_IPSEC_ENCRYPT_EN: c_uint = 0x00004000/* ESP Encrypt Enable */;
pub const IXGBE_ADVTXT_TUCMD_FCOE: c_uint = 0x00008000       /* FCoE Frame Type */;

// Autonegotiation advertised speeds
pub type ixgbe_autoneg_advertised = u32;
// Link speed
pub type ixgbe_link_speed = u32;
pub const IXGBE_LINK_SPEED_UNKNOWN: c_int = 0;
pub const IXGBE_LINK_SPEED_10_FULL: c_uint = 0x0002;
pub const IXGBE_LINK_SPEED_100_FULL: c_uint = 0x0008;
pub const IXGBE_LINK_SPEED_1GB_FULL: c_uint = 0x0020;
pub const IXGBE_LINK_SPEED_2_5GB_FULL: c_uint = 0x0400;
pub const IXGBE_LINK_SPEED_5GB_FULL: c_uint = 0x0800;
pub const IXGBE_LINK_SPEED_10GB_FULL: c_uint = 0x0080;

// Physical layer type
pub type ixgbe_physical_layer = u64;
pub const IXGBE_PHYSICAL_LAYER_UNKNOWN: c_int = 0;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_T: c_uint = 0x00001;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_T: c_uint = 0x00002;
pub const IXGBE_PHYSICAL_LAYER_100BASE_TX: c_uint = 0x00004;
pub const IXGBE_PHYSICAL_LAYER_SFP_PLUS_CU: c_uint = 0x00008;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_LR: c_uint = 0x00010;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_LRM: c_uint = 0x00020;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_SR: c_uint = 0x00040;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_KX4: c_uint = 0x00080;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_CX4: c_uint = 0x00100;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_KX: c_uint = 0x00200;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_BX: c_uint = 0x00400;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_KR: c_uint = 0x00800;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_XAUI: c_uint = 0x01000;
pub const IXGBE_PHYSICAL_LAYER_SFP_ACTIVE_DA: c_uint = 0x02000;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_SX: c_uint = 0x04000;
pub const IXGBE_PHYSICAL_LAYER_10BASE_T: c_uint = 0x08000;
pub const IXGBE_PHYSICAL_LAYER_2500BASE_KX: c_uint = 0x10000;
pub const IXGBE_PHYSICAL_LAYER_2500BASE_T: c_uint = 0x20000;
pub const IXGBE_PHYSICAL_LAYER_5000BASE_T: c_uint = 0x40000;
// Flow Control Data Sheet defined values
// Calculation and defines taken from 802.1bb Annex O
//
// BitTimes (BT) conversion

// Calculate Delay to respond to PFC
pub const IXGBE_PFC_D: c_int = 672;
// Calculate Cable Delay

// Calculate Interface Delay X540

// Calculate Interface Delay 82598, 82599
pub const IXGBE_PHY_D: c_int = 12800;
pub const IXGBE_MAC_D: c_int = 4096;

// Calculate Delay incurred from higher layer
pub const IXGBE_HD: c_int = 6144;
// Calculate PCI Bus delay for low thresholds
pub const IXGBE_PCI_DELAY: c_int = 10000;
// Calculate X540 delay value in bit times

// Calculate 82599, 82598 delay value in bit times

// Calculate low threshold delay values

// Software ATR hash keys
pub const IXGBE_ATR_BUCKET_HASH_KEY: c_uint = 0x3DAD14E2;
pub const IXGBE_ATR_SIGNATURE_HASH_KEY: c_uint = 0x174D3614;
// Software ATR input stream values and masks
pub const IXGBE_ATR_HASH_MASK: c_uint = 0x7fff;
pub const IXGBE_ATR_L4TYPE_MASK: c_uint = 0x3;
pub const IXGBE_ATR_L4TYPE_UDP: c_uint = 0x1;
pub const IXGBE_ATR_L4TYPE_TCP: c_uint = 0x2;
pub const IXGBE_ATR_L4TYPE_SCTP: c_uint = 0x3;
pub const IXGBE_ATR_L4TYPE_IPV6_MASK: c_uint = 0x4;
pub const IXGBE_ATR_L4TYPE_TUNNEL_MASK: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_atr_flow_type {
    IXGBE_ATR_FLOW_TYPE_IPV4   = 0x0,
    IXGBE_ATR_FLOW_TYPE_UDPV4  = 0x1,
    IXGBE_ATR_FLOW_TYPE_TCPV4  = 0x2,
    IXGBE_ATR_FLOW_TYPE_SCTPV4 = 0x3,
    IXGBE_ATR_FLOW_TYPE_IPV6   = 0x4,
    IXGBE_ATR_FLOW_TYPE_UDPV6  = 0x5,
    IXGBE_ATR_FLOW_TYPE_TCPV6  = 0x6,
    IXGBE_ATR_FLOW_TYPE_SCTPV6 = 0x7,
}

// Flow Director ATR input struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_atr_input {
//
// Byte layout in order, all values with MSB first:
//
// vm_pool    - 1 byte
// flow_type  - 1 byte
// vlan_id    - 2 bytes
// src_ip     - 16 bytes
// dst_ip     - 16 bytes
// src_port   - 2 bytes
// dst_port   - 2 bytes
// flex_bytes - 2 bytes
// bkt_hash   - 2 bytes
//
    pub vm_pool: u8,
    pub flow_type: u8,
    pub vlan_id: __be16,
    pub dst_ip: [__be32; 4],
    pub src_ip: [__be32; 4],
    pub src_port: __be16,
    pub dst_port: __be16,
    pub flex_bytes: __be16,
    pub bkt_hash: __be16,
    pub formatted: },
    pub dword_stream: [__be32; 11],
}

// Flow Director compressed ATR hash input struct
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_atr_hash_dword {
    pub vm_pool: u8,
    pub flow_type: u8,
    pub vlan_id: __be16,
    pub formatted: },
    pub ip: __be32,
    pub src: __be16,
    pub dst: __be16,
    pub port: },
    pub flex_bytes: __be16,
    pub dword: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_mvals {
    IXGBE_MVALS_INIT(IDX),
    IXGBE_MVALS_IDX_LIMIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_eeprom_type {
    ixgbe_eeprom_uninitialized = 0,
    ixgbe_eeprom_spi,
    ixgbe_flash,
    ixgbe_eeprom_none /* No NVM support */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_mac_type {
    ixgbe_mac_unknown = 0,
    ixgbe_mac_82598EB,
    ixgbe_mac_82599EB,
    ixgbe_mac_X540,
    ixgbe_mac_X550,
    ixgbe_mac_X550EM_x,
    ixgbe_mac_x550em_a,
    ixgbe_mac_e610,
    ixgbe_mac_e610_vf,
    ixgbe_num_macs
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_phy_type {
    ixgbe_phy_unknown = 0,
    ixgbe_phy_none,
    ixgbe_phy_tn,
    ixgbe_phy_aq,
    ixgbe_phy_x550em_kr,
    ixgbe_phy_x550em_kx4,
    ixgbe_phy_x550em_xfi,
    ixgbe_phy_x550em_ext_t,
    ixgbe_phy_ext_1g_t,
    ixgbe_phy_cu_unknown,
    ixgbe_phy_qt,
    ixgbe_phy_xaui,
    ixgbe_phy_nl,
    ixgbe_phy_sfp_passive_tyco,
    ixgbe_phy_sfp_passive_unknown,
    ixgbe_phy_sfp_active_unknown,
    ixgbe_phy_sfp_avago,
    ixgbe_phy_sfp_ftl,
    ixgbe_phy_sfp_ftl_active,
    ixgbe_phy_sfp_unknown,
    ixgbe_phy_sfp_intel,
    ixgbe_phy_qsfp_passive_unknown,
    ixgbe_phy_qsfp_active_unknown,
    ixgbe_phy_qsfp_intel,
    ixgbe_phy_qsfp_unknown,
    ixgbe_phy_sfp_unsupported,
    ixgbe_phy_sgmii,
    ixgbe_phy_fw,
    ixgbe_phy_generic
}

//
// SFP+ module type IDs:
//
// ID   Module Type
// =============
// 0    SFP_DA_CU
// 1    SFP_SR
// 2    SFP_LR
// 3    SFP_DA_CU_CORE0 - 82599-specific
// 4    SFP_DA_CU_CORE1 - 82599-specific
// 5    SFP_SR/LR_CORE0 - 82599-specific
// 6    SFP_SR/LR_CORE1 - 82599-specific
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_sfp_type {
    ixgbe_sfp_type_da_cu = 0,
    ixgbe_sfp_type_sr = 1,
    ixgbe_sfp_type_lr = 2,
    ixgbe_sfp_type_da_cu_core0 = 3,
    ixgbe_sfp_type_da_cu_core1 = 4,
    ixgbe_sfp_type_srlr_core0 = 5,
    ixgbe_sfp_type_srlr_core1 = 6,
    ixgbe_sfp_type_da_act_lmt_core0 = 7,
    ixgbe_sfp_type_da_act_lmt_core1 = 8,
    ixgbe_sfp_type_1g_cu_core0 = 9,
    ixgbe_sfp_type_1g_cu_core1 = 10,
    ixgbe_sfp_type_1g_sx_core0 = 11,
    ixgbe_sfp_type_1g_sx_core1 = 12,
    ixgbe_sfp_type_1g_lx_core0 = 13,
    ixgbe_sfp_type_1g_lx_core1 = 14,
    ixgbe_sfp_type_1g_bx_core0 = 15,
    ixgbe_sfp_type_1g_bx_core1 = 16,
    ixgbe_sfp_type_10g_bx_core0 = 17,
    ixgbe_sfp_type_10g_bx_core1 = 18,

    ixgbe_sfp_type_not_present = 0xFFFE,
    ixgbe_sfp_type_unknown = 0xFFFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_media_type {
    ixgbe_media_type_unknown = 0,
    ixgbe_media_type_fiber,
    ixgbe_media_type_fiber_qsfp,
    ixgbe_media_type_fiber_lco,
    ixgbe_media_type_copper,
    ixgbe_media_type_backplane,
    ixgbe_media_type_cx4,
    ixgbe_media_type_virtual,
    ixgbe_media_type_da,
    ixgbe_media_type_aui,
}

// Flow Control Settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_fc_mode {
    ixgbe_fc_none = 0,
    ixgbe_fc_rx_pause,
    ixgbe_fc_tx_pause,
    ixgbe_fc_full,
    ixgbe_fc_default,
    ixgbe_fc_pfc,
}

// Smart Speed Settings
pub const IXGBE_SMARTSPEED_MAX_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_smart_speed {
    ixgbe_smart_speed_auto = 0,
    ixgbe_smart_speed_on,
    ixgbe_smart_speed_off
}

// PCI bus types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_bus_type {
    ixgbe_bus_type_unknown = 0,
    ixgbe_bus_type_pci_express,
    ixgbe_bus_type_internal,
    ixgbe_bus_type_reserved
}

// PCI bus speeds
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_bus_speed {
    ixgbe_bus_speed_unknown = 0,
    ixgbe_bus_speed_33      = 33,
    ixgbe_bus_speed_66      = 66,
    ixgbe_bus_speed_100     = 100,
    ixgbe_bus_speed_120     = 120,
    ixgbe_bus_speed_133     = 133,
    ixgbe_bus_speed_2500    = 2500,
    ixgbe_bus_speed_5000    = 5000,
    ixgbe_bus_speed_8000    = 8000,
    ixgbe_bus_speed_reserved
}

// PCI bus widths
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_bus_width {
    ixgbe_bus_width_unknown = 0,
    ixgbe_bus_width_pcie_x1 = 1,
    ixgbe_bus_width_pcie_x2 = 2,
    ixgbe_bus_width_pcie_x4 = 4,
    ixgbe_bus_width_pcie_x8 = 8,
    ixgbe_bus_width_32      = 32,
    ixgbe_bus_width_64      = 64,
    ixgbe_bus_width_reserved
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_addr_filter_info {
    pub num_mc_addrs: u32,
    pub rar_used_count: u32,
    pub mta_in_use: u32,
    pub overflow_promisc: u32,
    pub uc_set_promisc: bool,
    pub user_set_promisc: bool,
}

// Bus parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_bus_info {
    pub speed: ixgbe_bus_speed,
    pub width: ixgbe_bus_width,
    pub type: ixgbe_bus_type,
    pub func: u8,
    pub lan_id: u8,
    pub instance_id: u8,
}

// Flow control parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_fc_info {
    pub /: *mut *mut u32 high_water[MAX_TRAFFIC_CLASS]; / Flow Control High-water,
    pub /: *mut *mut u32 low_water[MAX_TRAFFIC_CLASS]; / Flow Control Low-water,
    pub /: *mut *mut u16 pause_time; / Flow Control Pause timer,
    pub /: *mut *mut bool send_xon; / Flow control send XON,
    pub /: *mut *mut bool strict_ieee; / Strict IEEE mode,
    pub /: *mut *mut bool disable_fc_autoneg; / Do not autonegotiate FC,
    pub /: *mut *mut bool fc_was_autonegged; / Is current_mode the result of autonegging?,
    pub /: *mut *mut ixgbe_fc_mode current_mode; / FC mode in effect,
    pub /: *mut *mut ixgbe_fc_mode requested_mode; / FC mode requested by caller,
}

// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hw_stats {
    pub crcerrs: u64,
    pub illerrc: u64,
    pub errbc: u64,
    pub mspdc: u64,
    pub mpctotal: u64,
    pub mpc: [u64; 8],
    pub mlfc: u64,
    pub mrfc: u64,
    pub rlec: u64,
    pub lxontxc: u64,
    pub lxonrxc: u64,
    pub lxofftxc: u64,
    pub lxoffrxc: u64,
    pub pxontxc: [u64; 8],
    pub pxonrxc: [u64; 8],
    pub pxofftxc: [u64; 8],
    pub pxoffrxc: [u64; 8],
    pub prc64: u64,
    pub prc127: u64,
    pub prc255: u64,
    pub prc511: u64,
    pub prc1023: u64,
    pub prc1522: u64,
    pub gprc: u64,
    pub bprc: u64,
    pub mprc: u64,
    pub gptc: u64,
    pub gorc: u64,
    pub gotc: u64,
    pub rnbc: [u64; 8],
    pub ruc: u64,
    pub rfc: u64,
    pub roc: u64,
    pub rjc: u64,
    pub mngprc: u64,
    pub mngpdc: u64,
    pub mngptc: u64,
    pub tor: u64,
    pub tpr: u64,
    pub tpt: u64,
    pub ptc64: u64,
    pub ptc127: u64,
    pub ptc255: u64,
    pub ptc511: u64,
    pub ptc1023: u64,
    pub ptc1522: u64,
    pub mptc: u64,
    pub bptc: u64,
    pub xec: u64,
    pub rqsmr: [u64; 16],
    pub tqsmr: [u64; 8],
    pub qprc: [u64; 16],
    pub qptc: [u64; 16],
    pub qbrc: [u64; 16],
    pub qbtc: [u64; 16],
    pub qprdc: [u64; 16],
    pub pxon2offc: [u64; 8],
    pub fdirustat_add: u64,
    pub fdirustat_remove: u64,
    pub fdirfstat_fadd: u64,
    pub fdirfstat_fremove: u64,
    pub fdirmatch: u64,
    pub fdirmiss: u64,
    pub fccrc: u64,
    pub fcoerpdc: u64,
    pub fcoeprc: u64,
    pub fcoeptc: u64,
    pub fcoedwrc: u64,
    pub fcoedwtc: u64,
    pub fcoe_noddp: u64,
    pub fcoe_noddp_ext_buff: u64,
    pub b2ospc: u64,
    pub b2ogprc: u64,
    pub o2bgptc: u64,
    pub o2bspc: u64,
}

// forward declaration
// Function pointer table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_eeprom_operations {
    pub ): *mut *mut int (init_params)(struct ixgbe_hw,
    pub ): *mut *mut *mut int (read)(struct ixgbe_hw , u16, u16,
    pub ): *mut *mut *mut int (read_buffer)(struct ixgbe_hw , u16, u16, u16,
    pub u16): *mut *mut *mut int (write)(struct ixgbe_hw , u16,,
    pub ): *mut *mut *mut int (write_buffer)(struct ixgbe_hw , u16, u16, u16,
    pub ): *mut *mut *mut int (validate_checksum)(struct ixgbe_hw , u16,
    pub ): *mut *mut int (update_checksum)(struct ixgbe_hw,
    pub ): *mut *mut int (calc_checksum)(struct ixgbe_hw,
    pub pba_num_size): u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mac_operations {
    pub ): *mut *mut int (init_hw)(struct ixgbe_hw,
    pub ): *mut *mut int (reset_hw)(struct ixgbe_hw,
    pub ): *mut *mut int (start_hw)(struct ixgbe_hw,
    pub ): *mut *mut int (clear_hw_cntrs)(struct ixgbe_hw,
    pub ): *mut *mut ixgbe_media_type (get_media_type)(struct ixgbe_hw,
    pub hw): *mut *mut int (get_fw_ver)(struct ixgbe_hw,
    pub ): *mut *mut *mut int (get_mac_addr)(struct ixgbe_hw , u8,
    pub ): *mut *mut *mut int (get_san_mac_addr)(struct ixgbe_hw , u8,
    pub ): *mut *mut *mut int (get_device_caps)(struct ixgbe_hw , u16,
    pub ): *mut *mut *mut *mut int (get_wwn_prefix)(struct ixgbe_hw , u16 , u16,
    pub ): *mut *mut int (stop_adapter)(struct ixgbe_hw,
    pub ): *mut *mut int (get_bus_info)(struct ixgbe_hw,
    pub ): *mut *mut void (set_lan_id)(struct ixgbe_hw,
    pub u8*): *mut *mut *mut int (read_analog_reg8)(struct ixgbe_hw, u32,,
    pub u8): *mut *mut *mut int (write_analog_reg8)(struct ixgbe_hw, u32,,
    pub ): *mut *mut int (setup_sfp)(struct ixgbe_hw,
    pub ): *mut *mut int (disable_rx_buff)(struct ixgbe_hw,
    pub ): *mut *mut int (enable_rx_buff)(struct ixgbe_hw,
    pub u32): *mut *mut *mut int (enable_rx_dma)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut int (acquire_swfw_sync)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut void (release_swfw_sync)(struct ixgbe_hw ,,
    pub ): *mut *mut void (init_swfw_sync)(struct ixgbe_hw,
    pub ): *mut *mut *mut *mut int (prot_autoc_read)(struct ixgbe_hw , bool , u32,
    pub bool): *mut *mut *mut int (prot_autoc_write)(struct ixgbe_hw , u32,,
// Link
    pub ): *mut *mut void (disable_tx_laser)(struct ixgbe_hw,
    pub ): *mut *mut void (enable_tx_laser)(struct ixgbe_hw,
    pub ): *mut *mut void (flap_tx_laser)(struct ixgbe_hw,
    pub ): *mut *mut void (stop_link_on_d3)(struct ixgbe_hw,
    pub bool): *mut *mut *mut int (setup_link)(struct ixgbe_hw , ixgbe_link_speed,,
    pub bool): *mut *mut *mut int (setup_mac_link)(struct ixgbe_hw , ixgbe_link_speed,,
    pub bool): *mut *mut *mut *mut *mut int (check_link)(struct ixgbe_hw , ixgbe_link_speed , bool ,,
    pub ): *mut bool,
    pub ixgbe_link_speed): *mut *mut *mut void (set_rate_select_speed)(struct ixgbe_hw ,,
    pub enable_eee): *mut *mut *mut int (setup_eee)(struct ixgbe_hw hw, bool,
// Packet Buffer Manipulation
    pub int): *mut *mut *mut void (set_rxpba)(struct ixgbe_hw , int, u32,,
// LED
    pub u32): *mut *mut *mut int (led_on)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut int (led_off)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut int (blink_led_start)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut int (blink_led_stop)(struct ixgbe_hw ,,
    pub ): *mut *mut int (init_led_link_act)(struct ixgbe_hw,
// RAR, Multicast, VLAN
    pub u32): *mut *mut *mut *mut int (set_rar)(struct ixgbe_hw , u32, u8 , u32,,
    pub u32): *mut *mut *mut int (clear_rar)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut int (set_vmdq)(struct ixgbe_hw , u32,,
    pub u32): *mut *mut *mut int (set_vmdq_san_mac)(struct ixgbe_hw ,,
    pub u32): *mut *mut *mut int (clear_vmdq)(struct ixgbe_hw , u32,,
    pub ): *mut *mut int (init_rx_addrs)(struct ixgbe_hw,
    pub ): *mut *mut *mut int (update_mc_addr_list)(struct ixgbe_hw , struct net_device,
    pub ): *mut *mut int (enable_mc)(struct ixgbe_hw,
    pub ): *mut *mut int (disable_mc)(struct ixgbe_hw,
    pub ): *mut *mut int (clear_vfta)(struct ixgbe_hw,
    pub bool): *mut *mut *mut int (set_vfta)(struct ixgbe_hw , u32, u32, bool,,
    pub ): *mut *mut int (init_uta_tables)(struct ixgbe_hw,
    pub int): *mut *mut *mut void (set_mac_anti_spoofing)(struct ixgbe_hw , bool,,
    pub int): *mut *mut *mut void (set_vlan_anti_spoofing)(struct ixgbe_hw , bool,,
// Flow Control
    pub ): *mut *mut int (fc_enable)(struct ixgbe_hw,
    pub ): *mut *mut int (setup_fc)(struct ixgbe_hw,
    pub ): *mut *mut void (fc_autoneg)(struct ixgbe_hw,
// Manageability interface
    pub ): *const c_char,
    pub ): *mut *mut int (get_thermal_sensor_data)(struct ixgbe_hw,
    pub hw): *mut *mut int (init_thermal_sensor_thresh)(struct ixgbe_hw,
    pub hw): *mut *mut bool (fw_recovery_mode)(struct ixgbe_hw,
    pub hw): *mut *mut bool (fw_rollback_mode)(struct ixgbe_hw,
    pub nvm): *mut *mut *mut int (get_nvm_ver)(struct ixgbe_hw hw, struct ixgbe_nvm_info,
    pub hw): *mut *mut void (disable_rx)(struct ixgbe_hw,
    pub hw): *mut *mut void (enable_rx)(struct ixgbe_hw,
    pub int): unsigned,
    pub int): *mut *mut *mut void (set_ethertype_anti_spoofing)(struct ixgbe_hw , bool,,
// DMA Coalescing
    pub hw): *mut *mut int (dmac_config)(struct ixgbe_hw,
    pub hw): *mut *mut int (dmac_update_tcs)(struct ixgbe_hw,
    pub hw): *mut *mut int (dmac_config_tcs)(struct ixgbe_hw,
    pub ): *mut *mut *mut int (read_iosf_sb_reg)(struct ixgbe_hw , u32, u32, u32,
    pub u32): *mut *mut *mut int (write_iosf_sb_reg)(struct ixgbe_hw , u32, u32,,
// MDD events
    pub hw): *mut *mut void (enable_mdd)(struct ixgbe_hw,
    pub hw): *mut *mut void (disable_mdd)(struct ixgbe_hw,
    pub vf): *mut *mut *mut void (restore_mdd_vf)(struct ixgbe_hw hw, u32,
    pub vf_bitmap): *mut *mut *mut void (handle_mdd)(struct ixgbe_hw hw, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_phy_operations {
    pub ): *mut *mut int (identify)(struct ixgbe_hw,
    pub ): *mut *mut int (identify_sfp)(struct ixgbe_hw,
    pub ): *mut *mut int (init)(struct ixgbe_hw,
    pub ): *mut *mut int (reset)(struct ixgbe_hw,
    pub ): *mut *mut *mut int (read_reg)(struct ixgbe_hw , u32, u32, u16,
    pub u16): *mut *mut *mut int (write_reg)(struct ixgbe_hw , u32, u32,,
    pub ): *mut *mut *mut int (read_reg_mdi)(struct ixgbe_hw , u32, u32, u16,
    pub u16): *mut *mut *mut int (write_reg_mdi)(struct ixgbe_hw , u32, u32,,
    pub ): *mut *mut int (setup_link)(struct ixgbe_hw,
    pub ): *mut *mut int (setup_internal_link)(struct ixgbe_hw,
    pub bool): *mut *mut *mut int (setup_link_speed)(struct ixgbe_hw , ixgbe_link_speed,,
    pub ): *mut *mut *mut *mut int (check_link)(struct ixgbe_hw , ixgbe_link_speed , bool,
    pub ): *mut *mut *mut int (read_i2c_byte)(struct ixgbe_hw , u8, u8, u8,
    pub u8): *mut *mut *mut int (write_i2c_byte)(struct ixgbe_hw , u8, u8,,
    pub ): *mut *mut *mut int (read_i2c_sff8472)(struct ixgbe_hw , u8, u8,
    pub ): *mut *mut *mut int (read_i2c_eeprom)(struct ixgbe_hw , u8, u8,
    pub u8): *mut *mut *mut int (write_i2c_eeprom)(struct ixgbe_hw , u8,,
    pub ): *mut *mut bool (check_overtemp)(struct ixgbe_hw,
    pub on): *mut *mut *mut int (set_phy_power)(struct ixgbe_hw , bool,
    pub ): *mut *mut int (enter_lplu)(struct ixgbe_hw,
    pub ): *mut *mut *mut int (handle_lasi)(struct ixgbe_hw hw, bool,
    pub value): *mut u8,
    pub value): u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_link_operations {
    pub val): *mut *mut *mut int (read_link)(struct ixgbe_hw , u8 addr, u16 reg, u16,
    pub val): *mut u16,
    pub val): *mut *mut *mut int (write_link)(struct ixgbe_hw , u8 addr, u16 reg, u16,
    pub val): u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_link_info {
    pub ops: ixgbe_link_operations,
    pub addr: u8,
    pub link_info: ixgbe_link_status,
    pub link_info_old: ixgbe_link_status,
    pub get_link_info: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_eeprom_info {
    pub ops: ixgbe_eeprom_operations,
    pub type: ixgbe_eeprom_type,
    pub semaphore_delay: u32,
    pub word_size: u16,
    pub address_bits: u16,
    pub word_page_size: u16,
    pub ctrl_word_3: u16,
}

pub const IXGBE_FLAGS_DOUBLE_RESET_REQUIRED: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mac_info {
    pub ops: ixgbe_mac_operations,
    pub type: ixgbe_mac_type,
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub san_addr: [u8; ETH_ALEN],
// prefix for World Wide Node Name (WWNN)
    pub wwnn_prefix: u16,
// prefix for World Wide Port Name (WWPN)
    pub wwpn_prefix: u16,
    pub max_msix_vectors: u16,
pub const IXGBE_MAX_MTA: c_int = 128;
    pub mta_shadow: [u32; IXGBE_MAX_MTA],
    pub mc_filter_type: i32,
    pub mcft_size: u32,
    pub vft_size: u32,
    pub num_rar_entries: u32,
    pub rar_highwater: u32,
    pub rx_pb_size: u32,
    pub max_tx_queues: u32,
    pub max_rx_queues: u32,
    pub orig_autoc: u32,
    pub orig_autoc2: u32,
    pub orig_link_settings_stored: bool,
    pub autotry_restart: bool,
    pub flags: u8,
    pub san_mac_rar_index: u8,
    pub thermal_sensor_data: ixgbe_thermal_sensor_data,
    pub set_lben: bool,
    pub max_link_up_time: u32,
    pub led_link_act: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_phy_info {
    pub ops: ixgbe_phy_operations,
    pub mdio: mdio_if_info,
    pub type: ixgbe_phy_type,
    pub id: u32,
    pub sfp_type: ixgbe_sfp_type,
    pub sfp_setup_needed: bool,
    pub revision: u32,
    pub media_type: ixgbe_media_type,
    pub phy_semaphore_mask: u32,
    pub reset_disable: bool,
    pub autoneg_advertised: ixgbe_autoneg_advertised,
    pub speeds_supported: ixgbe_link_speed,
    pub eee_speeds_supported: ixgbe_link_speed,
    pub eee_speeds_advertised: ixgbe_link_speed,
    pub smart_speed: ixgbe_smart_speed,
    pub smart_speed_active: bool,
    pub multispeed_fiber: bool,
    pub reset_if_overtemp: bool,
    pub qsfp_shared_i2c_bus: bool,
    pub nw_mng_if_sel: u32,
    pub phy_type_low: u64,
    pub phy_type_high: u64,
    pub curr_user_speed_req: u16,
    pub curr_user_phy_cfg: ixgbe_aci_cmd_set_phy_cfg_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mbx_stats {
    pub msgs_tx: u32,
    pub msgs_rx: u32,
    pub acks: u32,
    pub reqs: u32,
    pub rsts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mbx_info {
    pub ops: *const ixgbe_mbx_operations,
    pub stats: ixgbe_mbx_stats,
    pub timeout: u32,
    pub usec_delay: u32,
    pub v2p_mailbox: u32,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hw {
    pub hw_addr: *mut u8 __iomem,
    pub back: *mut c_void,
    pub mac: ixgbe_mac_info,
    pub addr_ctrl: ixgbe_addr_filter_info,
    pub fc: ixgbe_fc_info,
    pub phy: ixgbe_phy_info,
    pub link: ixgbe_link_info,
    pub eeprom: ixgbe_eeprom_info,
    pub bus: ixgbe_bus_info,
    pub mbx: ixgbe_mbx_info,
    pub mvals: *const u32,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub adapter_stopped: bool,
    pub force_full_reset: bool,
    pub allow_unsupported_sfp: bool,
    pub wol_enabled: bool,
    pub need_crosstalk_fix: bool,
    pub api_branch: u8,
    pub api_maj_ver: u8,
    pub api_min_ver: u8,
    pub api_patch: u8,
    pub fw_branch: u8,
    pub fw_maj_ver: u8,
    pub fw_min_ver: u8,
    pub fw_patch: u8,
    pub fw_build: u32,
    pub aci: ixgbe_aci_info,
    pub flash: ixgbe_flash_info,
    pub dev_caps: ixgbe_hw_dev_caps,
    pub func_caps: ixgbe_hw_func_caps,
    pub fwlog: libie_fwlog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_info {
    pub mac: ixgbe_mac_type,
    pub ): *mut *mut int (get_invariants)(struct ixgbe_hw,
    pub mac_ops: *const ixgbe_mac_operations,
    pub eeprom_ops: *const ixgbe_eeprom_operations,
    pub phy_ops: *const ixgbe_phy_operations,
    pub mbx_ops: *const ixgbe_mbx_operations,
    pub link_ops: *const ixgbe_link_operations,
    pub mvals: *const u32,
}

pub const IXGBE_SB_IOSF_INDIRECT_CTRL: c_uint = 0x00011144;
pub const IXGBE_SB_IOSF_INDIRECT_DATA: c_uint = 0x00011148;
pub const IXGBE_SB_IOSF_CTRL_ADDR_SHIFT: c_int = 0;
pub const IXGBE_SB_IOSF_CTRL_ADDR_MASK: c_uint = 0xFF;
pub const IXGBE_SB_IOSF_CTRL_RESP_STAT_SHIFT: c_int = 18;

pub const IXGBE_SB_IOSF_CTRL_CMPL_ERR_SHIFT: c_int = 20;

pub const IXGBE_SB_IOSF_CTRL_TARGET_SELECT_SHIFT: c_int = 28;
pub const IXGBE_SB_IOSF_CTRL_TARGET_SELECT_MASK: c_uint = 0x7;
pub const IXGBE_SB_IOSF_CTRL_BUSY_SHIFT: c_int = 31;

pub const IXGBE_SB_IOSF_TARGET_KR_PHY: c_int = 0;
pub const IXGBE_NW_MNG_IF_SEL: c_uint = 0x00011178;

pub const IXGBE_NW_MNG_IF_SEL_MDIO_PHY_ADD_SHIFT: c_int = 3;

