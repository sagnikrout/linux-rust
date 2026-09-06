//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/global2.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Marvell 88E6xxx Switch Global 2 Registers support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2016-2017 Savoir-faire Linux Inc.
// Vivien Didelot <vivien.didelot@savoirfairelinux.com>
//

// Offset 0x00: Interrupt Source Register
pub const MV88E6XXX_G2_INT_SRC: c_uint = 0x00;
pub const MV88E6XXX_G2_INT_SRC_WDOG: c_uint = 0x8000;
pub const MV88E6XXX_G2_INT_SRC_JAM_LIMIT: c_uint = 0x4000;
pub const MV88E6XXX_G2_INT_SRC_DUPLEX_MISMATCH: c_uint = 0x2000;
pub const MV88E6XXX_G2_INT_SRC_WAKE_EVENT: c_uint = 0x1000;
pub const MV88E6352_G2_INT_SRC_SERDES: c_uint = 0x0800;
pub const MV88E6352_G2_INT_SRC_PHY: c_uint = 0x001f;
pub const MV88E6390_G2_INT_SRC_PHY: c_uint = 0x07fe;
pub const MV88E6XXX_G2_INT_SOURCE_WATCHDOG: c_int = 15;
// Offset 0x01: Interrupt Mask Register
pub const MV88E6XXX_G2_INT_MASK: c_uint = 0x01;
pub const MV88E6XXX_G2_INT_MASK_WDOG: c_uint = 0x8000;
pub const MV88E6XXX_G2_INT_MASK_JAM_LIMIT: c_uint = 0x4000;
pub const MV88E6XXX_G2_INT_MASK_DUPLEX_MISMATCH: c_uint = 0x2000;
pub const MV88E6XXX_G2_INT_MASK_WAKE_EVENT: c_uint = 0x1000;
pub const MV88E6352_G2_INT_MASK_SERDES: c_uint = 0x0800;
pub const MV88E6352_G2_INT_MASK_PHY: c_uint = 0x001f;
pub const MV88E6390_G2_INT_MASK_PHY: c_uint = 0x07fe;
// Offset 0x02: MGMT Enable Register 2x
pub const MV88E6XXX_G2_MGMT_EN_2X: c_uint = 0x02;
// Offset 0x02: MAC LINK change IRQ Register for MV88E6393X
pub const MV88E6393X_G2_MACLINK_INT_SRC: c_uint = 0x02;
// Offset 0x03: MGMT Enable Register 0x
pub const MV88E6XXX_G2_MGMT_EN_0X: c_uint = 0x03;
// Offset 0x03: MAC LINK change IRQ Mask Register for MV88E6393X
pub const MV88E6393X_G2_MACLINK_INT_MASK: c_uint = 0x03;
// Offset 0x04: Flow Control Delay Register
pub const MV88E6XXX_G2_FLOW_CTL: c_uint = 0x04;
// Offset 0x05: Switch Management Register
pub const MV88E6XXX_G2_SWITCH_MGMT: c_uint = 0x05;
pub const MV88E6XXX_G2_SWITCH_MGMT_USE_DOUBLE_TAG_DATA: c_uint = 0x8000;
pub const MV88E6XXX_G2_SWITCH_MGMT_PREVENT_LOOPS: c_uint = 0x4000;
pub const MV88E6XXX_G2_SWITCH_MGMT_FLOW_CTL_MSG: c_uint = 0x2000;
pub const MV88E6XXX_G2_SWITCH_MGMT_FORCE_FLOW_CTL_PRI: c_uint = 0x0080;
pub const MV88E6XXX_G2_SWITCH_MGMT_RSVD2CPU: c_uint = 0x0008;
pub const MV88E6393X_G2_EGRESS_MONITOR_DEST: c_uint = 0x05;
// Offset 0x06: Device Mapping Table Register
pub const MV88E6XXX_G2_DEVICE_MAPPING: c_uint = 0x06;
pub const MV88E6XXX_G2_DEVICE_MAPPING_UPDATE: c_uint = 0x8000;
pub const MV88E6XXX_G2_DEVICE_MAPPING_DEV_MASK: c_uint = 0x1f00;
pub const MV88E6352_G2_DEVICE_MAPPING_PORT_MASK: c_uint = 0x000f;
pub const MV88E6390_G2_DEVICE_MAPPING_PORT_MASK: c_uint = 0x001f;
// Offset 0x07: Trunk Mask Table Register
pub const MV88E6XXX_G2_TRUNK_MASK: c_uint = 0x07;
pub const MV88E6XXX_G2_TRUNK_MASK_UPDATE: c_uint = 0x8000;
pub const MV88E6XXX_G2_TRUNK_MASK_NUM_MASK: c_uint = 0x7000;
pub const MV88E6XXX_G2_TRUNK_MASK_HASH: c_uint = 0x0800;
// Offset 0x08: Trunk Mapping Table Register
pub const MV88E6XXX_G2_TRUNK_MAPPING: c_uint = 0x08;
pub const MV88E6XXX_G2_TRUNK_MAPPING_UPDATE: c_uint = 0x8000;
pub const MV88E6XXX_G2_TRUNK_MAPPING_ID_MASK: c_uint = 0x7800;
// Offset 0x09: Ingress Rate Command Register
pub const MV88E6XXX_G2_IRL_CMD: c_uint = 0x09;
pub const MV88E6XXX_G2_IRL_CMD_BUSY: c_uint = 0x8000;
pub const MV88E6352_G2_IRL_CMD_OP_MASK: c_uint = 0x7000;
pub const MV88E6352_G2_IRL_CMD_OP_NOOP: c_uint = 0x0000;
pub const MV88E6352_G2_IRL_CMD_OP_INIT_ALL: c_uint = 0x1000;
pub const MV88E6352_G2_IRL_CMD_OP_INIT_RES: c_uint = 0x2000;
pub const MV88E6352_G2_IRL_CMD_OP_WRITE_REG: c_uint = 0x3000;
pub const MV88E6352_G2_IRL_CMD_OP_READ_REG: c_uint = 0x4000;
pub const MV88E6390_G2_IRL_CMD_OP_MASK: c_uint = 0x6000;
pub const MV88E6390_G2_IRL_CMD_OP_READ_REG: c_uint = 0x0000;
pub const MV88E6390_G2_IRL_CMD_OP_INIT_ALL: c_uint = 0x2000;
pub const MV88E6390_G2_IRL_CMD_OP_INIT_RES: c_uint = 0x4000;
pub const MV88E6390_G2_IRL_CMD_OP_WRITE_REG: c_uint = 0x6000;
pub const MV88E6352_G2_IRL_CMD_PORT_MASK: c_uint = 0x0f00;
pub const MV88E6390_G2_IRL_CMD_PORT_MASK: c_uint = 0x1f00;
pub const MV88E6XXX_G2_IRL_CMD_RES_MASK: c_uint = 0x00e0;
pub const MV88E6XXX_G2_IRL_CMD_REG_MASK: c_uint = 0x000f;
// Offset 0x0A: Ingress Rate Data Register
pub const MV88E6XXX_G2_IRL_DATA: c_uint = 0x0a;
pub const MV88E6XXX_G2_IRL_DATA_MASK: c_uint = 0xffff;
// Offset 0x0B: Cross-chip Port VLAN Register
pub const MV88E6XXX_G2_PVT_ADDR: c_uint = 0x0b;
pub const MV88E6XXX_G2_PVT_ADDR_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_G2_PVT_ADDR_OP_MASK: c_uint = 0x7000;
pub const MV88E6XXX_G2_PVT_ADDR_OP_INIT_ONES: c_uint = 0x1000;
pub const MV88E6XXX_G2_PVT_ADDR_OP_WRITE_PVLAN: c_uint = 0x3000;
pub const MV88E6XXX_G2_PVT_ADDR_OP_READ: c_uint = 0x4000;
pub const MV88E6XXX_G2_PVT_ADDR_PTR_MASK: c_uint = 0x01ff;
pub const MV88E6XXX_G2_PVT_ADDR_DEV_TRUNK: c_uint = 0x1f;
// Offset 0x0C: Cross-chip Port VLAN Data Register
pub const MV88E6XXX_G2_PVT_DATA: c_uint = 0x0c;
pub const MV88E6XXX_G2_PVT_DATA_MASK: c_uint = 0x7f;
// Offset 0x0D: Switch MAC/WoL/WoF Register
pub const MV88E6XXX_G2_SWITCH_MAC: c_uint = 0x0d;
pub const MV88E6XXX_G2_SWITCH_MAC_UPDATE: c_uint = 0x8000;
pub const MV88E6XXX_G2_SWITCH_MAC_PTR_MASK: c_uint = 0x1f00;
pub const MV88E6XXX_G2_SWITCH_MAC_DATA_MASK: c_uint = 0x00ff;
// Offset 0x0E: ATU Stats Register
pub const MV88E6XXX_G2_ATU_STATS: c_uint = 0x0e;

pub const MV88E6XXX_G2_ATU_STATS_MASK: c_uint = 0x0fff;
// Offset 0x0F: Priority Override Table
pub const MV88E6XXX_G2_PRIO_OVERRIDE: c_uint = 0x0f;
pub const MV88E6XXX_G2_PRIO_OVERRIDE_UPDATE: c_uint = 0x8000;
pub const MV88E6XXX_G2_PRIO_OVERRIDE_FPRISET: c_uint = 0x1000;
pub const MV88E6XXX_G2_PRIO_OVERRIDE_PTR_MASK: c_uint = 0x0f00;
pub const MV88E6352_G2_PRIO_OVERRIDE_QPRIAVBEN: c_uint = 0x0080;
pub const MV88E6352_G2_PRIO_OVERRIDE_DATAAVB_MASK: c_uint = 0x0030;
pub const MV88E6XXX_G2_PRIO_OVERRIDE_QFPRIEN: c_uint = 0x0008;
pub const MV88E6XXX_G2_PRIO_OVERRIDE_DATA_MASK: c_uint = 0x0007;
// Offset 0x14: EEPROM Command
pub const MV88E6XXX_G2_EEPROM_CMD: c_uint = 0x14;
pub const MV88E6XXX_G2_EEPROM_CMD_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_G2_EEPROM_CMD_OP_MASK: c_uint = 0x7000;
pub const MV88E6XXX_G2_EEPROM_CMD_OP_WRITE: c_uint = 0x3000;
pub const MV88E6XXX_G2_EEPROM_CMD_OP_READ: c_uint = 0x4000;
pub const MV88E6XXX_G2_EEPROM_CMD_OP_LOAD: c_uint = 0x6000;
pub const MV88E6XXX_G2_EEPROM_CMD_RUNNING: c_uint = 0x0800;
pub const MV88E6XXX_G2_EEPROM_CMD_WRITE_EN: c_uint = 0x0400;
pub const MV88E6352_G2_EEPROM_CMD_ADDR_MASK: c_uint = 0x00ff;
pub const MV88E6390_G2_EEPROM_CMD_DATA_MASK: c_uint = 0x00ff;
// Offset 0x15: EEPROM Data
pub const MV88E6352_G2_EEPROM_DATA: c_uint = 0x15;
pub const MV88E6352_G2_EEPROM_DATA_MASK: c_uint = 0xffff;
// Offset 0x15: EEPROM Addr
pub const MV88E6390_G2_EEPROM_ADDR: c_uint = 0x15;
pub const MV88E6390_G2_EEPROM_ADDR_MASK: c_uint = 0xffff;
// Offset 0x16: AVB Command Register
pub const MV88E6352_G2_AVB_CMD: c_uint = 0x16;
pub const MV88E6352_G2_AVB_CMD_BUSY: c_uint = 0x8000;
pub const MV88E6352_G2_AVB_CMD_OP_READ: c_uint = 0x4000;
pub const MV88E6352_G2_AVB_CMD_OP_READ_INCR: c_uint = 0x6000;
pub const MV88E6352_G2_AVB_CMD_OP_WRITE: c_uint = 0x3000;
pub const MV88E6390_G2_AVB_CMD_OP_READ: c_uint = 0x0000;
pub const MV88E6390_G2_AVB_CMD_OP_READ_INCR: c_uint = 0x4000;
pub const MV88E6390_G2_AVB_CMD_OP_WRITE: c_uint = 0x6000;
pub const MV88E6352_G2_AVB_CMD_PORT_MASK: c_uint = 0x0f00;
pub const MV88E6352_G2_AVB_CMD_PORT_TAIGLOBAL: c_uint = 0xe;
pub const MV88E6165_G2_AVB_CMD_PORT_PTPGLOBAL: c_uint = 0xf;
pub const MV88E6352_G2_AVB_CMD_PORT_PTPGLOBAL: c_uint = 0xf;
pub const MV88E6390_G2_AVB_CMD_PORT_MASK: c_uint = 0x1f00;
pub const MV88E6390_G2_AVB_CMD_PORT_TAIGLOBAL: c_uint = 0x1e;
pub const MV88E6390_G2_AVB_CMD_PORT_PTPGLOBAL: c_uint = 0x1f;
pub const MV88E6352_G2_AVB_CMD_BLOCK_PTP: c_int = 0;
pub const MV88E6352_G2_AVB_CMD_BLOCK_AVB: c_int = 1;
pub const MV88E6352_G2_AVB_CMD_BLOCK_QAV: c_int = 2;
pub const MV88E6352_G2_AVB_CMD_BLOCK_QVB: c_int = 3;
pub const MV88E6352_G2_AVB_CMD_BLOCK_MASK: c_uint = 0x00e0;
pub const MV88E6352_G2_AVB_CMD_ADDR_MASK: c_uint = 0x001f;
// Offset 0x17: AVB Data Register
pub const MV88E6352_G2_AVB_DATA: c_uint = 0x17;
// Offset 0x18: SMI PHY Command Register
pub const MV88E6XXX_G2_SMI_PHY_CMD: c_uint = 0x18;
pub const MV88E6XXX_G2_SMI_PHY_CMD_BUSY: c_uint = 0x8000;
pub const MV88E6390_G2_SMI_PHY_CMD_FUNC_MASK: c_uint = 0x6000;
pub const MV88E6390_G2_SMI_PHY_CMD_FUNC_INTERNAL: c_uint = 0x0000;
pub const MV88E6390_G2_SMI_PHY_CMD_FUNC_EXTERNAL: c_uint = 0x2000;
pub const MV88E6390_G2_SMI_PHY_CMD_FUNC_SETUP: c_uint = 0x4000;
pub const MV88E6XXX_G2_SMI_PHY_CMD_MODE_MASK: c_uint = 0x1000;
pub const MV88E6XXX_G2_SMI_PHY_CMD_MODE_45: c_uint = 0x0000;
pub const MV88E6XXX_G2_SMI_PHY_CMD_MODE_22: c_uint = 0x1000;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_MASK: c_uint = 0x0c00;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_22_WRITE_DATA: c_uint = 0x0400;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_22_READ_DATA: c_uint = 0x0800;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_45_WRITE_ADDR: c_uint = 0x0000;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_45_WRITE_DATA: c_uint = 0x0400;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_45_READ_DATA_INC: c_uint = 0x0800;
pub const MV88E6XXX_G2_SMI_PHY_CMD_OP_45_READ_DATA: c_uint = 0x0c00;
pub const MV88E6XXX_G2_SMI_PHY_CMD_DEV_ADDR_MASK: c_uint = 0x03e0;
pub const MV88E6XXX_G2_SMI_PHY_CMD_REG_ADDR_MASK: c_uint = 0x001f;
pub const MV88E6XXX_G2_SMI_PHY_CMD_SETUP_PTR_MASK: c_uint = 0x03ff;
// Offset 0x19: SMI PHY Data Register
pub const MV88E6XXX_G2_SMI_PHY_DATA: c_uint = 0x19;
// Offset 0x1A: Scratch and Misc. Register
pub const MV88E6XXX_G2_SCRATCH_MISC_MISC: c_uint = 0x1a;
pub const MV88E6XXX_G2_SCRATCH_MISC_UPDATE: c_uint = 0x8000;
pub const MV88E6XXX_G2_SCRATCH_MISC_PTR_MASK: c_uint = 0x7f00;
pub const MV88E6XXX_G2_SCRATCH_MISC_DATA_MASK: c_uint = 0x00ff;
// Offset 0x1B: Watch Dog Control Register
pub const MV88E6250_G2_WDOG_CTL: c_uint = 0x1b;
pub const MV88E6250_G2_WDOG_CTL_QC_HISTORY: c_uint = 0x0100;
pub const MV88E6250_G2_WDOG_CTL_QC_EVENT: c_uint = 0x0080;
pub const MV88E6250_G2_WDOG_CTL_QC_ENABLE: c_uint = 0x0040;
pub const MV88E6250_G2_WDOG_CTL_EGRESS_HISTORY: c_uint = 0x0020;
pub const MV88E6250_G2_WDOG_CTL_EGRESS_EVENT: c_uint = 0x0010;
pub const MV88E6250_G2_WDOG_CTL_EGRESS_ENABLE: c_uint = 0x0008;
pub const MV88E6250_G2_WDOG_CTL_FORCE_IRQ: c_uint = 0x0004;
pub const MV88E6250_G2_WDOG_CTL_HISTORY: c_uint = 0x0002;
pub const MV88E6250_G2_WDOG_CTL_SWRESET: c_uint = 0x0001;
// Offset 0x1B: Watch Dog Control Register
pub const MV88E6352_G2_WDOG_CTL: c_uint = 0x1b;
pub const MV88E6352_G2_WDOG_CTL_EGRESS_EVENT: c_uint = 0x0080;
pub const MV88E6352_G2_WDOG_CTL_RMU_TIMEOUT: c_uint = 0x0040;
pub const MV88E6352_G2_WDOG_CTL_QC_ENABLE: c_uint = 0x0020;
pub const MV88E6352_G2_WDOG_CTL_EGRESS_HISTORY: c_uint = 0x0010;
pub const MV88E6352_G2_WDOG_CTL_EGRESS_ENABLE: c_uint = 0x0008;
pub const MV88E6352_G2_WDOG_CTL_FORCE_IRQ: c_uint = 0x0004;
pub const MV88E6352_G2_WDOG_CTL_HISTORY: c_uint = 0x0002;
pub const MV88E6352_G2_WDOG_CTL_SWRESET: c_uint = 0x0001;
// Offset 0x1B: Watch Dog Control Register
pub const MV88E6390_G2_WDOG_CTL: c_uint = 0x1b;
pub const MV88E6390_G2_WDOG_CTL_UPDATE: c_uint = 0x8000;
pub const MV88E6390_G2_WDOG_CTL_PTR_MASK: c_uint = 0x7f00;
pub const MV88E6390_G2_WDOG_CTL_PTR_INT_SOURCE: c_uint = 0x0000;
pub const MV88E6390_G2_WDOG_CTL_PTR_INT_STS: c_uint = 0x1000;
pub const MV88E6390_G2_WDOG_CTL_PTR_INT_ENABLE: c_uint = 0x1100;
pub const MV88E6390_G2_WDOG_CTL_PTR_EVENT: c_uint = 0x1200;
pub const MV88E6390_G2_WDOG_CTL_PTR_HISTORY: c_uint = 0x1300;
pub const MV88E6390_G2_WDOG_CTL_DATA_MASK: c_uint = 0x00ff;
pub const MV88E6390_G2_WDOG_CTL_CUT_THROUGH: c_uint = 0x0008;
pub const MV88E6390_G2_WDOG_CTL_QUEUE_CONTROLLER: c_uint = 0x0004;
pub const MV88E6390_G2_WDOG_CTL_EGRESS: c_uint = 0x0002;
pub const MV88E6390_G2_WDOG_CTL_FORCE_IRQ: c_uint = 0x0001;
// Offset 0x1C: QoS Weights Register
pub const MV88E6XXX_G2_QOS_WEIGHTS: c_uint = 0x1c;
pub const MV88E6XXX_G2_QOS_WEIGHTS_UPDATE: c_uint = 0x8000;
pub const MV88E6352_G2_QOS_WEIGHTS_PTR_MASK: c_uint = 0x3f00;
pub const MV88E6390_G2_QOS_WEIGHTS_PTR_MASK: c_uint = 0x7f00;
pub const MV88E6XXX_G2_QOS_WEIGHTS_DATA_MASK: c_uint = 0x00ff;
// Offset 0x1D: Misc Register
pub const MV88E6XXX_G2_MISC: c_uint = 0x1d;
pub const MV88E6XXX_G2_MISC_5_BIT_PORT: c_uint = 0x4000;
pub const MV88E6352_G2_NOEGR_POLICY: c_uint = 0x2000;
pub const MV88E6390_G2_LAG_ID_4: c_uint = 0x2000;
// Scratch/Misc registers accessed through MV88E6XXX_G2_SCRATCH_MISC
// Offset 0x02: Misc Configuration
pub const MV88E6352_G2_SCRATCH_MISC_CFG: c_uint = 0x02;
pub const MV88E6352_G2_SCRATCH_MISC_CFG_NORMALSMI: c_uint = 0x80;
// Offset 0x60-0x61: GPIO Configuration
pub const MV88E6352_G2_SCRATCH_GPIO_CFG0: c_uint = 0x60;
pub const MV88E6352_G2_SCRATCH_GPIO_CFG1: c_uint = 0x61;
// Offset 0x62-0x63: GPIO Direction
pub const MV88E6352_G2_SCRATCH_GPIO_DIR0: c_uint = 0x62;
pub const MV88E6352_G2_SCRATCH_GPIO_DIR1: c_uint = 0x63;
pub const MV88E6352_G2_SCRATCH_GPIO_DIR_OUT: c_int = 0;
pub const MV88E6352_G2_SCRATCH_GPIO_DIR_IN: c_int = 1;
// Offset 0x64-0x65: GPIO Data
pub const MV88E6352_G2_SCRATCH_GPIO_DATA0: c_uint = 0x64;
pub const MV88E6352_G2_SCRATCH_GPIO_DATA1: c_uint = 0x65;
// Offset 0x68-0x6F: GPIO Pin Control
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL0: c_uint = 0x68;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL1: c_uint = 0x69;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL2: c_uint = 0x6A;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL3: c_uint = 0x6B;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL4: c_uint = 0x6C;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL5: c_uint = 0x6D;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL6: c_uint = 0x6E;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL7: c_uint = 0x6F;
pub const MV88E6352_G2_SCRATCH_CONFIG_DATA0: c_uint = 0x70;
pub const MV88E6352_G2_SCRATCH_CONFIG_DATA1: c_uint = 0x71;

pub const MV88E6352_G2_SCRATCH_CONFIG_DATA2: c_uint = 0x72;
pub const MV88E6352_G2_SCRATCH_CONFIG_DATA2_P0_MODE_MASK: c_uint = 0xf;
pub const MV88E6352_G2_SCRATCH_CONFIG_DATA3: c_uint = 0x73;

pub const MV88E6352_G2_SCRATCH_GPIO_PCTL_GPIO: c_int = 0;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL_TRIG: c_int = 1;
pub const MV88E6352_G2_SCRATCH_GPIO_PCTL_EVREQ: c_int = 2;
extern "C" {
    pub fn mv88e6xxx_g2_read(chip: *mut mv88e6xxx_chip, reg: c_int, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_write(chip: *mut mv88e6xxx_chip, reg: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn mv88e6352_g2_irl_init_all(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g2_irl_init_all(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_set_switch_mac(chip: *mut mv88e6xxx_chip, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_misc_4_bit_port(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_irq_setup(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_irq_free(chip: *mut mv88e6xxx_chip);
}
extern "C" {
    pub fn mv88e6185_g2_mgmt_rsvd2cpu(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6352_g2_mgmt_rsvd2cpu(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_pot_clear(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_trunk_clear(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_eeprom_wait(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6352_g2_cache_global_scratch_config3(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6352_g2_scratch_port_has_serdes(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_atu_stats_set(chip: *mut mv88e6xxx_chip, kind: u16, bin: u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g2_atu_stats_get(chip: *mut mv88e6xxx_chip, stats: *mut u16) -> c_int;
}
