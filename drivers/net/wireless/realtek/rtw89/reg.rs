//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/reg.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2019-2020  Realtek Corporation
//
pub const R_AX_SYS_WL_EFUSE_CTRL: c_uint = 0x000A;

pub const R_AX_SYS_ISO_CTRL: c_uint = 0x0000;

pub const R_AX_SYS_FUNC_EN: c_uint = 0x0002;

pub const R_AX_SYS_PW_CTRL: c_uint = 0x0004;

pub const R_AX_SYS_CLK_CTRL: c_uint = 0x0008;

pub const R_AX_SYS_SWR_CTRL1: c_uint = 0x0010;

pub const R_AX_SYS_ADIE_PAD_PWR_CTRL: c_uint = 0x0018;

pub const R_AX_RSV_CTRL: c_uint = 0x001C;

pub const R_AX_AFE_LDO_CTRL: c_uint = 0x0020;

pub const R_AX_EFUSE_CTRL_1: c_uint = 0x0038;

pub const R_AX_EFUSE_CTRL: c_uint = 0x0030;

pub const R_AX_EFUSE_CTRL_1_V1: c_uint = 0x0038;

pub const R_AX_GPIO_MUXCFG: c_uint = 0x0040;

pub const MAC_AX_BT_MODE_0_3: c_int = 0;
pub const MAC_AX_BT_MODE_2: c_int = 2;
pub const MAC_AX_RTK_MODE: c_int = 0;
pub const MAC_AX_CSR_MODE: c_int = 1;

pub const R_AX_DBG_CTRL: c_uint = 0x0058;

pub const R_AX_GPIO_EXT_CTRL: c_uint = 0x0060;

pub const R_AX_SYS_SDIO_CTRL: c_uint = 0x0070;

pub const R_AX_HCI_OPT_CTRL: c_uint = 0x0074;

pub const R_AX_HCI_BG_CTRL: c_uint = 0x0078;

pub const R_AX_HCI_LDO_CTRL: c_uint = 0x007A;

pub const R_AX_PLATFORM_ENABLE: c_uint = 0x0088;

pub const R_AX_WLLPS_CTRL: c_uint = 0x0090;

pub const SW_LPS_OPTION: c_uint = 0x0001A0B2;
pub const R_AX_SCOREBOARD: c_uint = 0x00AC;

pub const MAC_AX_NOTIFY_TP_MAJOR: c_uint = 0x81;
pub const MAC_AX_NOTIFY_PWR_MAJOR: c_uint = 0x80;
pub const R_AX_DBG_PORT_SEL: c_uint = 0x00C0;

pub const R_AX_PAD_CTRL2: c_uint = 0x00C4;

pub const USB_MODE_U2: c_uint = 0x1;
pub const USB_MODE_U3: c_uint = 0x2;

pub const USB_SWITCH_DELAY: c_uint = 0xF;
pub const R_AX_PMC_DBG_CTRL2: c_uint = 0x00CC;

pub const R_AX_PCIE_MIO_INTF: c_uint = 0x00E4;

pub const MIO_WRITE_BYTE_ALL: c_uint = 0xF;

pub const R_AX_PCIE_MIO_INTD: c_uint = 0x00E8;

pub const R_AX_SYS_CFG1: c_uint = 0x00F0;

pub const R_AX_SYS_STATUS1: c_uint = 0x00F4;

pub const MAC_AX_HCI_SEL_SDIO_UART: c_int = 0;
pub const MAC_AX_HCI_SEL_MULTI_USB: c_int = 1;
pub const MAC_AX_HCI_SEL_PCIE_UART: c_int = 2;
pub const MAC_AX_HCI_SEL_PCIE_USB: c_int = 3;
pub const MAC_AX_HCI_SEL_MULTI_SDIO: c_int = 4;
pub const R_AX_GPIO_16_TO_18_EXT_CTRL: c_uint = 0x0150;
pub const R_AX_HALT_H2C_CTRL: c_uint = 0x0160;
pub const R_AX_HALT_H2C: c_uint = 0x0168;

pub const R_AX_HALT_C2H_CTRL: c_uint = 0x0164;
pub const R_AX_HALT_C2H: c_uint = 0x016C;
pub const R_AX_WCPU_FW_CTRL: c_uint = 0x01E0;

pub const R_AX_RPWM: c_uint = 0x01E4;
pub const R_AX_PCIE_HRPWM: c_uint = 0x10C0;

pub const PS_RPWM_STATE: c_uint = 0x7;
pub const RPWM_SEQ_NUM_MAX: c_int = 3;

pub const CPWM_SEQ_NUM_MAX: c_int = 3;
pub const R_AX_BOOT_REASON: c_uint = 0x01E6;

pub const R_AX_LDM: c_uint = 0x01E8;

pub const R_AX_UDM0: c_uint = 0x01F0;
pub const R_AX_UDM1: c_uint = 0x01F4;

pub const R_AX_UDM2: c_uint = 0x01F8;
pub const R_AX_UDM3: c_uint = 0x01FC;
pub const R_AX_SPS_DIG_ON_CTRL0: c_uint = 0x0200;

pub const R_AX_SPSLDO_ON_CTRL1: c_uint = 0x0204;

pub const R_AX_LDO_AON_CTRL0: c_uint = 0x0218;

pub const R_AX_SPSANA_ON_CTRL1: c_uint = 0x0224;
pub const R_AX_SPS_ANA_ON_CTRL2: c_uint = 0x0228;
pub const RTL8852B_RFE_05_SPS_ANA: c_uint = 0x4A82;
pub const R_AX_WLAN_XTAL_SI_CTRL: c_uint = 0x0270;

pub const XTAL_SI_NORMAL_WRITE: c_uint = 0x00;
pub const XTAL_SI_NORMAL_READ: c_uint = 0x01;

pub const R_AX_WLAN_XTAL_SI_CONFIG: c_uint = 0x0274;

pub const R_AX_XTAL_ON_CTRL0: c_uint = 0x0280;

pub const R_AX_XTAL_ON_CTRL3: c_uint = 0x028C;

pub const R_AX_GPIO0_7_FUNC_SEL: c_uint = 0x02D0;
pub const R_AX_GPIO8_15_FUNC_SEL: c_uint = 0x02D4;

pub const R_AX_EECS_EESK_FUNC_SEL: c_uint = 0x02D8;

pub const R_AX_GPIO16_23_FUNC_SEL: c_uint = 0x02D8;

pub const R_AX_LED1_FUNC_SEL: c_uint = 0x02DC;

pub const PINMUX_EESK_FUNC_SEL_BT_LOG: c_uint = 0x1;
pub const R_AX_GPIO0_15_EECS_EESK_LED1_PULL_LOW_EN: c_uint = 0x02E4;

pub const R_AX_GPIO0_16_EECS_EESK_LED1_PULL_LOW_EN: c_uint = 0x02E4;

pub const R_AX_WLRF_CTRL: c_uint = 0x02F0;

pub const R_AX_IC_PWR_STATE: c_uint = 0x03F0;

pub const MAC_AX_MAC_OFF: c_int = 0;
pub const MAC_AX_MAC_ON: c_int = 1;
pub const MAC_AX_MAC_LPS: c_int = 2;

pub const R_AX_SPS_DIG_OFF_CTRL0: c_uint = 0x0400;

pub const R_AX_AFE_OFF_CTRL1: c_uint = 0x0444;

pub const R_AX_DBG_WOW: c_uint = 0x0504;

pub const R_AX_SEC_CTRL: c_uint = 0x0C00;

pub const R_AX_FILTER_MODEL_ADDR: c_uint = 0x0C04;
pub const R_AX_HAXI_INIT_CFG1: c_uint = 0x1000;

pub const DMA_MOD_PCIE_1B: c_uint = 0x0;
pub const DMA_MOD_PCIE_4B: c_uint = 0x1;
pub const DMA_MOD_USB: c_uint = 0x2;
pub const DMA_MOD_SDIO: c_uint = 0x3;

pub const R_AX_HAXI_DMA_STOP1: c_uint = 0x1010;

pub const R_AX_HAXI_DMA_BUSY1: c_uint = 0x101C;

pub const R_AX_USB_ENDPOINT_0: c_uint = 0x1060;

pub const R_AX_USB_ENDPOINT_2: c_uint = 0x1068;
pub const NUMP: c_uint = 0x1;
pub const R_AX_USB_HOST_REQUEST_2: c_uint = 0x1078;

pub const R_AX_USB3_MAC_NPI_CONFIG_INTF_0: c_uint = 0x1114;

pub const R_AX_USB_WLAN0_1: c_uint = 0x1174;

pub const R_AX_PCIE_DBG_CTRL: c_uint = 0x11C0;

pub const R_AX_HAXI_DMA_STOP2: c_uint = 0x11C0;

pub const R_AX_HAXI_DMA_BUSY2: c_uint = 0x11C8;

pub const R_AX_HAXI_DMA_BUSY3: c_uint = 0x1208;

pub const R_AX_LTR_DEC_CTRL: c_uint = 0x1600;

pub const PCIE_LTR_IDX_IDLE: c_int = 3;

pub const R_AX_LTR_LATENCY_IDX0: c_uint = 0x1604;
pub const R_AX_LTR_LATENCY_IDX1: c_uint = 0x1608;
pub const R_AX_LTR_LATENCY_IDX2: c_uint = 0x160C;
pub const R_AX_LTR_LATENCY_IDX3: c_uint = 0x1610;
pub const R_AX_HCI_FC_CTRL_V1: c_uint = 0x1700;
pub const R_AX_CH_PAGE_CTRL_V1: c_uint = 0x1704;
pub const R_AX_ACH0_PAGE_CTRL_V1: c_uint = 0x1710;
pub const R_AX_ACH1_PAGE_CTRL_V1: c_uint = 0x1714;
pub const R_AX_ACH2_PAGE_CTRL_V1: c_uint = 0x1718;
pub const R_AX_ACH3_PAGE_CTRL_V1: c_uint = 0x171C;
pub const R_AX_ACH4_PAGE_CTRL_V1: c_uint = 0x1720;
pub const R_AX_ACH5_PAGE_CTRL_V1: c_uint = 0x1724;
pub const R_AX_ACH6_PAGE_CTRL_V1: c_uint = 0x1728;
pub const R_AX_ACH7_PAGE_CTRL_V1: c_uint = 0x172C;
pub const R_AX_CH8_PAGE_CTRL_V1: c_uint = 0x1730;
pub const R_AX_CH9_PAGE_CTRL_V1: c_uint = 0x1734;
pub const R_AX_CH10_PAGE_CTRL_V1: c_uint = 0x1738;
pub const R_AX_CH11_PAGE_CTRL_V1: c_uint = 0x173C;
pub const R_AX_ACH0_PAGE_INFO_V1: c_uint = 0x1750;
pub const R_AX_ACH1_PAGE_INFO_V1: c_uint = 0x1754;
pub const R_AX_ACH2_PAGE_INFO_V1: c_uint = 0x1758;
pub const R_AX_ACH3_PAGE_INFO_V1: c_uint = 0x175C;
pub const R_AX_ACH4_PAGE_INFO_V1: c_uint = 0x1760;
pub const R_AX_ACH5_PAGE_INFO_V1: c_uint = 0x1764;
pub const R_AX_ACH6_PAGE_INFO_V1: c_uint = 0x1768;
pub const R_AX_ACH7_PAGE_INFO_V1: c_uint = 0x176C;
pub const R_AX_CH8_PAGE_INFO_V1: c_uint = 0x1770;
pub const R_AX_CH9_PAGE_INFO_V1: c_uint = 0x1774;
pub const R_AX_CH10_PAGE_INFO_V1: c_uint = 0x1778;
pub const R_AX_CH11_PAGE_INFO_V1: c_uint = 0x177C;
pub const R_AX_CH12_PAGE_INFO_V1: c_uint = 0x1780;
pub const R_AX_PUB_PAGE_INFO3_V1: c_uint = 0x178C;
pub const R_AX_PUB_PAGE_CTRL1_V1: c_uint = 0x1790;
pub const R_AX_PUB_PAGE_CTRL2_V1: c_uint = 0x1794;
pub const R_AX_PUB_PAGE_INFO1_V1: c_uint = 0x1798;
pub const R_AX_PUB_PAGE_INFO2_V1: c_uint = 0x179C;
pub const R_AX_WP_PAGE_CTRL1_V1: c_uint = 0x17A0;
pub const R_AX_WP_PAGE_CTRL2_V1: c_uint = 0x17A4;
pub const R_AX_WP_PAGE_INFO1_V1: c_uint = 0x17A8;
pub const R_AX_USB_ENDPOINT_0_V1: c_uint = 0x5060;

pub const R_AX_USB_ENDPOINT_2_V1: c_uint = 0x5068;
pub const R_AX_USB_HOST_REQUEST_2_V1: c_uint = 0x5078;

pub const R_AX_USB3_MAC_NPI_CONFIG_INTF_0_V1: c_uint = 0x5114;

pub const R_AX_USB_WLAN0_1_V1: c_uint = 0x5174;

pub const R_AX_H2CREG_DATA0_V1: c_uint = 0x7140;
pub const R_AX_H2CREG_DATA1_V1: c_uint = 0x7144;
pub const R_AX_H2CREG_DATA2_V1: c_uint = 0x7148;
pub const R_AX_H2CREG_DATA3_V1: c_uint = 0x714C;
pub const R_AX_C2HREG_DATA0_V1: c_uint = 0x7150;
pub const R_AX_C2HREG_DATA1_V1: c_uint = 0x7154;
pub const R_AX_C2HREG_DATA2_V1: c_uint = 0x7158;
pub const R_AX_C2HREG_DATA3_V1: c_uint = 0x715C;
pub const R_AX_H2CREG_CTRL_V1: c_uint = 0x7160;
pub const R_AX_C2HREG_CTRL_V1: c_uint = 0x7164;
pub const R_AX_HCI_FUNC_EN_V1: c_uint = 0x7880;
pub const R_AX_PHYREG_SET: c_uint = 0x8040;
pub const PHYREG_SET_ALL_CYCLE: c_uint = 0x8;
pub const PHYREG_SET_XYN_CYCLE: c_uint = 0xE;
pub const R_AX_HD0IMR: c_uint = 0x8110;

pub const R_AX_HD0ISR: c_uint = 0x8114;

pub const R_AX_H2CREG_DATA0: c_uint = 0x8140;
pub const R_AX_H2CREG_DATA1: c_uint = 0x8144;
pub const R_AX_H2CREG_DATA2: c_uint = 0x8148;
pub const R_AX_H2CREG_DATA3: c_uint = 0x814C;
pub const R_AX_C2HREG_DATA0: c_uint = 0x8150;
pub const R_AX_C2HREG_DATA1: c_uint = 0x8154;
pub const R_AX_C2HREG_DATA2: c_uint = 0x8158;
pub const R_AX_C2HREG_DATA3: c_uint = 0x815C;
pub const R_AX_H2CREG_CTRL: c_uint = 0x8160;

pub const R_AX_C2HREG_CTRL: c_uint = 0x8164;

pub const R_AX_CPWM: c_uint = 0x8170;
pub const R_AX_HCI_FUNC_EN: c_uint = 0x8380;

pub const R_AX_BOOT_DBG: c_uint = 0x83F0;
pub const R_AX_DMAC_FUNC_EN: c_uint = 0x8400;

pub const R_AX_DMAC_CLK_EN: c_uint = 0x8404;

pub const PCI_LTR_IDLE_TIMER_1US: c_int = 0;
pub const PCI_LTR_IDLE_TIMER_10US: c_int = 1;
pub const PCI_LTR_IDLE_TIMER_100US: c_int = 2;
pub const PCI_LTR_IDLE_TIMER_200US: c_int = 3;
pub const PCI_LTR_IDLE_TIMER_400US: c_int = 4;
pub const PCI_LTR_IDLE_TIMER_800US: c_int = 5;
pub const PCI_LTR_IDLE_TIMER_1_6MS: c_int = 6;
pub const PCI_LTR_IDLE_TIMER_3_2MS: c_int = 7;
pub const PCI_LTR_IDLE_TIMER_R_ERR: c_uint = 0xFD;
pub const PCI_LTR_IDLE_TIMER_DEF: c_uint = 0xFE;
pub const PCI_LTR_IDLE_TIMER_IGNORE: c_uint = 0xFF;
pub const PCI_LTR_SPC_10US: c_int = 0;
pub const PCI_LTR_SPC_100US: c_int = 1;
pub const PCI_LTR_SPC_500US: c_int = 2;
pub const PCI_LTR_SPC_1MS: c_int = 3;
pub const PCI_LTR_SPC_R_ERR: c_uint = 0xFD;
pub const PCI_LTR_SPC_DEF: c_uint = 0xFE;
pub const PCI_LTR_SPC_IGNORE: c_uint = 0xFF;
pub const R_AX_LTR_CTRL_0: c_uint = 0x8410;

pub const R_AX_LTR_CTRL_1: c_uint = 0x8414;

pub const R_AX_LTR_IDLE_LATENCY: c_uint = 0x8418;
pub const R_AX_LTR_ACTIVE_LATENCY: c_uint = 0x841C;
pub const R_AX_SER_DBG_INFO: c_uint = 0x8424;

pub const R_AX_DLE_EMPTY0: c_uint = 0x8430;

pub const R_AX_DLE_EMPTY1: c_uint = 0x8434;

pub const R_AX_DMAC_ERR_IMR: c_uint = 0x8520;

pub const DMAC_ERR_IMR_DIS: c_int = 0;
pub const R_AX_DMAC_ERR_ISR: c_uint = 0x8524;

pub const R_AX_DISPATCHER_GLOBAL_SETTING_0: c_uint = 0x8800;

pub const R_AX_OTHER_DISPATCHER_ERR_ISR: c_uint = 0x8804;
pub const R_AX_HOST_DISPATCHER_ERR_ISR: c_uint = 0x8808;
pub const R_AX_CPU_DISPATCHER_ERR_ISR: c_uint = 0x880C;
pub const R_AX_TX_ADDRESS_INFO_MODE_SETTING: c_uint = 0x8810;

pub const R_AX_HOST_DISPATCHER_ERR_IMR: c_uint = 0x8850;

pub const R_AX_CPU_DISPATCHER_ERR_IMR: c_uint = 0x8854;

pub const R_AX_OTHER_DISPATCHER_ERR_IMR: c_uint = 0x8858;

pub const R_AX_DISPATCHER_DBG_PORT: c_uint = 0x8860;

pub const R_AX_RXDMA_SETTING: c_uint = 0x8908;

pub const USB11_BULKSIZE: c_uint = 0x2;
pub const USB2_BULKSIZE: c_uint = 0x1;
pub const USB3_BULKSIZE: c_uint = 0x0;
pub const R_AX_RX_FUNCTION_STOP: c_uint = 0x8920;

pub const R_AX_HCI_FC_CTRL: c_uint = 0x8A00;

pub const R_AX_CH_PAGE_CTRL: c_uint = 0x8A04;

pub const R_AX_ACH0_PAGE_CTRL: c_uint = 0x8A10;
pub const R_AX_ACH1_PAGE_CTRL: c_uint = 0x8A14;
pub const R_AX_ACH2_PAGE_CTRL: c_uint = 0x8A18;
pub const R_AX_ACH3_PAGE_CTRL: c_uint = 0x8A1C;
pub const R_AX_ACH4_PAGE_CTRL: c_uint = 0x8A20;
pub const R_AX_ACH5_PAGE_CTRL: c_uint = 0x8A24;
pub const R_AX_ACH6_PAGE_CTRL: c_uint = 0x8A28;
pub const R_AX_ACH7_PAGE_CTRL: c_uint = 0x8A2C;
pub const R_AX_CH8_PAGE_CTRL: c_uint = 0x8A30;
pub const R_AX_CH9_PAGE_CTRL: c_uint = 0x8A34;
pub const R_AX_CH10_PAGE_CTRL: c_uint = 0x8A38;
pub const R_AX_CH11_PAGE_CTRL: c_uint = 0x8A3C;

pub const R_AX_ACH0_PAGE_INFO: c_uint = 0x8A50;
pub const R_AX_ACH1_PAGE_INFO: c_uint = 0x8A54;
pub const R_AX_ACH2_PAGE_INFO: c_uint = 0x8A58;
pub const R_AX_ACH3_PAGE_INFO: c_uint = 0x8A5C;
pub const R_AX_ACH4_PAGE_INFO: c_uint = 0x8A60;
pub const R_AX_ACH5_PAGE_INFO: c_uint = 0x8A64;
pub const R_AX_ACH6_PAGE_INFO: c_uint = 0x8A68;
pub const R_AX_ACH7_PAGE_INFO: c_uint = 0x8A6C;
pub const R_AX_CH8_PAGE_INFO: c_uint = 0x8A70;
pub const R_AX_CH9_PAGE_INFO: c_uint = 0x8A74;
pub const R_AX_CH10_PAGE_INFO: c_uint = 0x8A78;
pub const R_AX_CH11_PAGE_INFO: c_uint = 0x8A7C;
pub const R_AX_CH12_PAGE_INFO: c_uint = 0x8A80;
pub const R_AX_PUB_PAGE_INFO3: c_uint = 0x8A8C;

pub const R_AX_PUB_PAGE_CTRL1: c_uint = 0x8A90;

pub const R_AX_PUB_PAGE_CTRL2: c_uint = 0x8A94;

pub const R_AX_PUB_PAGE_INFO1: c_uint = 0x8A98;

pub const R_AX_PUB_PAGE_INFO2: c_uint = 0x8A9C;

pub const R_AX_WP_PAGE_CTRL1: c_uint = 0x8AA0;

pub const R_AX_WP_PAGE_CTRL2: c_uint = 0x8AA4;

pub const R_AX_WP_PAGE_INFO1: c_uint = 0x8AA8;

pub const R_AX_WDE_PKTBUF_CFG: c_uint = 0x8C08;

pub const R_AX_WDE_ERRFLAG_MSG: c_uint = 0x8C30;

pub const R_AX_WDE_ERR_FLAG_CFG_NUM1: c_uint = 0x8C34;

pub const R_AX_WDE_ERR_IMR: c_uint = 0x8C38;

pub const R_AX_WDE_ERR_ISR: c_uint = 0x8C3C;

pub const R_AX_WDE_QTA0_CFG: c_uint = 0x8C40;
pub const R_AX_WDE_QTA1_CFG: c_uint = 0x8C44;
pub const R_AX_WDE_QTA2_CFG: c_uint = 0x8C48;
pub const R_AX_WDE_QTA3_CFG: c_uint = 0x8C4C;
pub const R_AX_WDE_QTA4_CFG: c_uint = 0x8C50;

pub const R_AX_WDE_INI_STATUS: c_uint = 0x8D00;

pub const R_AX_WDE_DBG_FUN_INTF_CTL: c_uint = 0x8D10;

pub const R_AX_WDE_DBG_FUN_INTF_DATA: c_uint = 0x8D14;

pub const R_AX_PLE_PKTBUF_CFG: c_uint = 0x9008;

pub const R_AX_PLE_DBGERR_LOCKEN: c_uint = 0x9020;

pub const R_AX_PLE_DBGERR_STS: c_uint = 0x9024;

pub const R_AX_PLE_ERR_FLAG_CFG_NUM1: c_uint = 0x9034;

pub const R_AX_PLE_ERRFLAG_MSG: c_uint = 0x9030;

pub const R_AX_PLE_ERR_IMR: c_uint = 0x9038;

pub const R_AX_PLE_ERR_FLAG_ISR: c_uint = 0x903C;

pub const R_AX_PLE_QTA0_CFG: c_uint = 0x9040;
pub const R_AX_PLE_QTA1_CFG: c_uint = 0x9044;
pub const R_AX_PLE_QTA2_CFG: c_uint = 0x9048;
pub const R_AX_PLE_QTA3_CFG: c_uint = 0x904C;
pub const R_AX_PLE_QTA4_CFG: c_uint = 0x9050;
pub const R_AX_PLE_QTA5_CFG: c_uint = 0x9054;
pub const R_AX_PLE_QTA6_CFG: c_uint = 0x9058;

pub const R_AX_PLE_QTA7_CFG: c_uint = 0x905C;

pub const R_AX_PLE_QTA8_CFG: c_uint = 0x9060;
pub const R_AX_PLE_QTA9_CFG: c_uint = 0x9064;
pub const R_AX_PLE_QTA10_CFG: c_uint = 0x9068;
pub const R_AX_PLE_QTA11_CFG: c_uint = 0x906C;
pub const R_AX_PLE_INI_STATUS: c_uint = 0x9100;

pub const R_AX_PLE_DBG_FUN_INTF_CTL: c_uint = 0x9110;

pub const R_AX_PLE_DBG_FUN_INTF_DATA: c_uint = 0x9114;

pub const R_AX_WDRLS_CFG: c_uint = 0x9408;

pub const R_AX_RLSRPT0_CFG0: c_uint = 0x9410;

pub const R_AX_RLSRPT0_CFG1: c_uint = 0x9414;

pub const R_AX_WDRLS_ERR_IMR: c_uint = 0x9430;

pub const R_AX_WDRLS_ERR_ISR: c_uint = 0x9434;
pub const R_AX_BBRPT_COM_ERR_IMR: c_uint = 0x9608;

pub const R_AX_BBRPT_COM_ERR_IMR_ISR: c_uint = 0x960C;

pub const R_AX_BBRPT_COM_ERR_ISR: c_uint = 0x960C;

pub const R_AX_BBRPT_CHINFO_ERR_ISR: c_uint = 0x962C;

pub const R_AX_BBRPT_CHINFO_ERR_IMR: c_uint = 0x9628;

pub const R_AX_BBRPT_CHINFO_ERR_IMR_ISR: c_uint = 0x962C;

pub const R_AX_BBRPT_DFS_ERR_IMR: c_uint = 0x9638;

pub const R_AX_BBRPT_DFS_ERR_IMR_ISR: c_uint = 0x963C;

pub const R_AX_BBRPT_DFS_ERR_ISR: c_uint = 0x963C;

pub const R_AX_LA_ERRFLAG: c_uint = 0x966C;

pub const R_AX_WD_BUF_REQ: c_uint = 0x9800;
pub const R_AX_PL_BUF_REQ: c_uint = 0x9820;

pub const R_AX_WD_BUF_STATUS: c_uint = 0x9804;
pub const R_AX_PL_BUF_STATUS: c_uint = 0x9824;

pub const R_AX_WD_CPUQ_OP_0: c_uint = 0x9810;
pub const R_AX_PL_CPUQ_OP_0: c_uint = 0x9830;

pub const R_AX_WD_CPUQ_OP_1: c_uint = 0x9814;
pub const R_AX_PL_CPUQ_OP_1: c_uint = 0x9834;

pub const R_AX_WD_CPUQ_OP_2: c_uint = 0x9818;
pub const R_AX_PL_CPUQ_OP_2: c_uint = 0x9838;

pub const R_AX_WD_CPUQ_OP_STATUS: c_uint = 0x981C;
pub const R_AX_PL_CPUQ_OP_STATUS: c_uint = 0x983C;

pub const R_AX_CPUIO_ERR_IMR: c_uint = 0x9840;

pub const R_AX_CPUIO_ERR_ISR: c_uint = 0x9844;
pub const R_AX_SEC_ERR_IMR_ISR: c_uint = 0x991C;
pub const R_AX_PKTIN_SETTING: c_uint = 0x9A00;

pub const R_AX_PKTIN_ERR_IMR: c_uint = 0x9A20;

pub const R_AX_PKTIN_ERR_ISR: c_uint = 0x9A24;
pub const R_AX_MPDU_TX_ERR_ISR: c_uint = 0x9BF0;
pub const R_AX_MPDU_TX_ERR_IMR: c_uint = 0x9BF4;

pub const R_AX_MPDU_PROC: c_uint = 0x9C00;

pub const R_AX_ACTION_FWD0: c_uint = 0x9C04;
pub const TRXCFG_MPDU_PROC_ACT_FRWD: c_uint = 0x02A95A95;
pub const R_AX_ACTION_FWD1: c_uint = 0x9C08;
pub const R_AX_TF_FWD: c_uint = 0x9C14;
pub const TRXCFG_MPDU_PROC_TF_FRWD: c_uint = 0x0000AA55;
pub const R_AX_HW_RPT_FWD: c_uint = 0x9C18;

pub const RTW89_PRPT_DEST_HOST: c_int = 1;
pub const RTW89_PRPT_DEST_WLCPU: c_int = 2;
pub const R_AX_CUT_AMSDU_CTRL: c_uint = 0x9C40;
pub const TRXCFG_MPDU_PROC_CUT_CTRL: c_uint = 0x010E05F0;
pub const R_AX_WOW_CTRL: c_uint = 0x9C50;

pub const R_AX_MPDU_RX_ERR_ISR: c_uint = 0x9CF0;
pub const R_AX_MPDU_RX_ERR_IMR: c_uint = 0x9CF4;

pub const R_AX_SEC_ENG_CTRL: c_uint = 0x9D00;

pub const R_AX_SEC_MPDU_PROC: c_uint = 0x9D04;

pub const R_AX_SEC_CAM_ACCESS: c_uint = 0x9D10;
pub const R_AX_SEC_CAM_RDATA: c_uint = 0x9D14;
pub const R_AX_SEC_CAM_WDATA: c_uint = 0x9D18;
pub const R_AX_SEC_DEBUG: c_uint = 0x9D1C;

pub const R_AX_SEC_DEBUG1: c_uint = 0x9D1C;

pub const AX_TX_TO_VAL: c_uint = 0x2;
pub const R_AX_SEC_TX_DEBUG: c_uint = 0x9D20;
pub const R_AX_SEC_RX_DEBUG: c_uint = 0x9D24;
pub const R_AX_SEC_TRX_PKT_CNT: c_uint = 0x9D28;
pub const R_AX_SEC_DEBUG2: c_uint = 0x9D28;
pub const B_AX_DBG_READ_SH: c_int = 2;
pub const B_AX_DBG_READ_MSK: c_uint = 0x3fffffff;
pub const R_AX_SEC_TRX_BLK_CNT: c_uint = 0x9D2C;
pub const R_AX_SEC_ERROR_FLAG_IMR: c_uint = 0x9D2C;

pub const R_AX_SEC_ERROR_FLAG: c_uint = 0x9D30;

pub const R_AX_SS_CTRL: c_uint = 0x9E10;

pub const R_AX_SS2FINFO_PATH: c_uint = 0x9E50;

pub const SS2F_PATH_WLCPU: c_uint = 0x0A;

pub const R_AX_SS_MACID_PAUSE_0: c_uint = 0x9EB0;
pub const B_AX_SS_MACID31_0_PAUSE_SH: c_int = 0;

pub const R_AX_SS_MACID_PAUSE_1: c_uint = 0x9EB4;
pub const B_AX_SS_MACID63_32_PAUSE_SH: c_int = 0;

pub const R_AX_SS_MACID_PAUSE_2: c_uint = 0x9EB8;
pub const B_AX_SS_MACID95_64_PAUSE_SH: c_int = 0;

pub const R_AX_SS_MACID_PAUSE_3: c_uint = 0x9EBC;
pub const B_AX_SS_MACID127_96_PAUSE_SH: c_int = 0;

pub const R_AX_STA_SCHEDULER_ERR_IMR: c_uint = 0x9EF0;

pub const R_AX_STA_SCHEDULER_ERR_ISR: c_uint = 0x9EF4;
pub const R_AX_TXPKTCTL_ERR_IMR_ISR: c_uint = 0x9F1C;

pub const R_AX_TXPKTCTL_ERR_IMR_ISR_B1: c_uint = 0x9F2C;

pub const R_AX_DBG_FUN_INTF_CTL: c_uint = 0x9F30;

pub const R_AX_DBG_FUN_INTF_DATA: c_uint = 0x9F34;

pub const R_AX_TXPKTCTL_B0_PRELD_CFG0: c_uint = 0x9F48;

pub const PRELD_B0_ENT_NUM: c_int = 10;
pub const PRELD_B01_ENT_NUM_8922D: c_int = 2;
pub const PRELD_AMSDU_SIZE: c_int = 52;
pub const PRELD_NEXT_MIN_SIZE: c_int = 255;

pub const R_AX_TXPKTCTL_B0_PRELD_CFG1: c_uint = 0x9F4C;

pub const PRELD_NEXT_WND: c_int = 1;

pub const R_AX_TXPKTCTL_B0_ERRFLAG_IMR: c_uint = 0x9F78;

pub const R_AX_TXPKTCTL_B0_ERRFLAG_ISR: c_uint = 0x9F7C;

pub const R_AX_TXPKTCTL_B1_PRELD_CFG0: c_uint = 0x9F88;

pub const PRELD_B1_ENT_NUM: c_int = 4;

pub const R_AX_TXPKTCTL_B1_PRELD_CFG1: c_uint = 0x9F8C;

pub const R_AX_TXPKTCTL_B1_ERRFLAG_IMR: c_uint = 0x9FB8;

pub const R_AX_TXPKTCTL_B1_ERRFLAG_ISR: c_uint = 0x9FBC;

pub const R_AX_AFE_CTRL1: c_uint = 0x0024;

pub const R_AX_SYS_ISO_CTRL_EXTEND: c_uint = 0x0080;

pub const R_AX_SYSON_FSM_MON: c_uint = 0x00A0;

pub const R_AX_CMAC_REG_START: c_uint = 0xC000;
pub const R_AX_CMAC_FUNC_EN: c_uint = 0xC000;
pub const R_AX_CMAC_FUNC_EN_C1: c_uint = 0xE000;

pub const R_AX_CK_EN: c_uint = 0xC004;
pub const R_AX_CK_EN_C1: c_uint = 0xE004;

pub const R_AX_WMAC_RFMOD: c_uint = 0xC010;
pub const R_AX_WMAC_RFMOD_C1: c_uint = 0xE010;

pub const AX_WMAC_RFMOD_20M: c_int = 0;
pub const AX_WMAC_RFMOD_40M: c_int = 1;
pub const AX_WMAC_RFMOD_80M: c_int = 2;
pub const AX_WMAC_RFMOD_160M: c_int = 3;
pub const R_AX_GID_POSITION0: c_uint = 0xC070;
pub const R_AX_GID_POSITION0_C1: c_uint = 0xE070;
pub const R_AX_GID_POSITION1: c_uint = 0xC074;
pub const R_AX_GID_POSITION1_C1: c_uint = 0xE074;
pub const R_AX_GID_POSITION2: c_uint = 0xC078;
pub const R_AX_GID_POSITION2_C1: c_uint = 0xE078;
pub const R_AX_GID_POSITION3: c_uint = 0xC07C;
pub const R_AX_GID_POSITION3_C1: c_uint = 0xE07C;
pub const R_AX_GID_POSITION_EN0: c_uint = 0xC080;
pub const R_AX_GID_POSITION_EN0_C1: c_uint = 0xE080;
pub const R_AX_GID_POSITION_EN1: c_uint = 0xC084;
pub const R_AX_GID_POSITION_EN1_C1: c_uint = 0xE084;
pub const R_AX_TX_SUB_CARRIER_VALUE: c_uint = 0xC088;
pub const R_AX_TX_SUB_CARRIER_VALUE_C1: c_uint = 0xE088;

pub const R_AX_PTCL_RRSR1: c_uint = 0xC090;
pub const R_AX_PTCL_RRSR1_C1: c_uint = 0xE090;

pub const RRSR_OFDM_CCK_EN: c_int = 3;

pub const R_AX_CMAC_ERR_IMR: c_uint = 0xC160;
pub const R_AX_CMAC_ERR_IMR_C1: c_uint = 0xE160;

pub const CMAC0_ERR_IMR_DIS: c_int = 0;
pub const CMAC1_ERR_IMR_DIS: c_int = 0;
pub const R_AX_CMAC_ERR_ISR: c_uint = 0xC164;
pub const R_AX_CMAC_ERR_ISR_C1: c_uint = 0xE164;

pub const R_AX_PORT0_TSF_SYNC: c_uint = 0xC2A0;
pub const R_AX_PORT0_TSF_SYNC_C1: c_uint = 0xE2A0;
pub const R_AX_PORT1_TSF_SYNC: c_uint = 0xC2A4;
pub const R_AX_PORT1_TSF_SYNC_C1: c_uint = 0xE2A4;
pub const R_AX_PORT2_TSF_SYNC: c_uint = 0xC2A8;
pub const R_AX_PORT2_TSF_SYNC_C1: c_uint = 0xE2A8;
pub const R_AX_PORT3_TSF_SYNC: c_uint = 0xC2AC;
pub const R_AX_PORT3_TSF_SYNC_C1: c_uint = 0xE2AC;
pub const R_AX_PORT4_TSF_SYNC: c_uint = 0xC2B0;
pub const R_AX_PORT4_TSF_SYNC_C1: c_uint = 0xE2B0;

pub const R_AX_MACID_SLEEP_0: c_uint = 0xC2C0;
pub const R_AX_MACID_SLEEP_0_C1: c_uint = 0xE2C0;
pub const B_AX_MACID31_0_SLEEP_SH: c_int = 0;

pub const R_AX_MACID_SLEEP_1: c_uint = 0xC2C4;
pub const R_AX_MACID_SLEEP_1_C1: c_uint = 0xE2C4;
pub const B_AX_MACID63_32_SLEEP_SH: c_int = 0;

pub const R_AX_MACID_SLEEP_2: c_uint = 0xC2C8;
pub const R_AX_MACID_SLEEP_2_C1: c_uint = 0xE2C8;
pub const B_AX_MACID95_64_SLEEP_SH: c_int = 0;

pub const R_AX_MACID_SLEEP_3: c_uint = 0xC2CC;
pub const R_AX_MACID_SLEEP_3_C1: c_uint = 0xE2CC;
pub const B_AX_MACID127_96_SLEEP_SH: c_int = 0;

pub const SCH_PREBKF_24US: c_uint = 0x18;
pub const R_AX_PREBKF_CFG_0: c_uint = 0xC338;
pub const R_AX_PREBKF_CFG_0_C1: c_uint = 0xE338;

pub const R_AX_PREBKF_CFG_1: c_uint = 0xC33C;
pub const R_AX_PREBKF_CFG_1_C1: c_uint = 0xE33C;

pub const SIFS_MACTXEN_T1: c_uint = 0x47;
pub const SIFS_MACTXEN_T1_V1: c_uint = 0x41;
pub const R_AX_CCA_CFG_0: c_uint = 0xC340;
pub const R_AX_CCA_CFG_0_C1: c_uint = 0xE340;

pub const R_AX_CTN_TXEN: c_uint = 0xC348;
pub const R_AX_CTN_TXEN_C1: c_uint = 0xE348;

pub const R_AX_MUEDCA_BE_PARAM_0: c_uint = 0xC350;
pub const R_AX_MUEDCA_BE_PARAM_0_C1: c_uint = 0xE350;

pub const R_AX_MUEDCA_BK_PARAM_0: c_uint = 0xC354;
pub const R_AX_MUEDCA_BK_PARAM_0_C1: c_uint = 0xE354;
pub const R_AX_MUEDCA_VI_PARAM_0: c_uint = 0xC358;
pub const R_AX_MUEDCA_VI_PARAM_0_C1: c_uint = 0xE358;
pub const R_AX_MUEDCA_VO_PARAM_0: c_uint = 0xC35C;
pub const R_AX_MUEDCA_VO_PARAM_0_C1: c_uint = 0xE35C;
pub const R_AX_MUEDCA_EN: c_uint = 0xC370;
pub const R_AX_MUEDCA_EN_C1: c_uint = 0xE370;

pub const R_AX_CCA_CONTROL: c_uint = 0xC390;
pub const R_AX_CCA_CONTROL_C1: c_uint = 0xE390;

pub const R_AX_CTN_DRV_TXEN: c_uint = 0xC398;
pub const R_AX_CTN_DRV_TXEN_C1: c_uint = 0xE398;

pub const R_AX_SCHEDULE_ERR_IMR: c_uint = 0xC3E8;
pub const R_AX_SCHEDULE_ERR_IMR_C1: c_uint = 0xE3E8;

pub const R_AX_SCHEDULE_ERR_ISR: c_uint = 0xC3EC;
pub const R_AX_SCHEDULE_ERR_ISR_C1: c_uint = 0xE3EC;
pub const R_AX_SCH_DBG_SEL: c_uint = 0xC3F4;
pub const R_AX_SCH_DBG_SEL_C1: c_uint = 0xE3F4;

pub const R_AX_SCH_DBG: c_uint = 0xC3F8;
pub const R_AX_SCH_DBG_C1: c_uint = 0xE3F8;

pub const R_AX_SCH_EXT_CTRL: c_uint = 0xC3FC;
pub const R_AX_SCH_EXT_CTRL_C1: c_uint = 0xE3FC;

pub const R_AX_PORT_CFG_P0: c_uint = 0xC400;
pub const R_AX_PORT_CFG_P1: c_uint = 0xC440;
pub const R_AX_PORT_CFG_P2: c_uint = 0xC480;
pub const R_AX_PORT_CFG_P3: c_uint = 0xC4C0;
pub const R_AX_PORT_CFG_P4: c_uint = 0xC500;

pub const R_AX_TBTT_PROHIB_P0: c_uint = 0xC404;
pub const R_AX_TBTT_PROHIB_P1: c_uint = 0xC444;
pub const R_AX_TBTT_PROHIB_P2: c_uint = 0xC484;
pub const R_AX_TBTT_PROHIB_P3: c_uint = 0xC4C4;
pub const R_AX_TBTT_PROHIB_P4: c_uint = 0xC504;

pub const R_AX_BCN_AREA_P0: c_uint = 0xC408;
pub const R_AX_BCN_AREA_P1: c_uint = 0xC448;
pub const R_AX_BCN_AREA_P2: c_uint = 0xC488;
pub const R_AX_BCN_AREA_P3: c_uint = 0xC4C8;
pub const R_AX_BCN_AREA_P4: c_uint = 0xC508;

pub const R_AX_BCNERLYINT_CFG_P0: c_uint = 0xC40C;
pub const R_AX_BCNERLYINT_CFG_P1: c_uint = 0xC44C;
pub const R_AX_BCNERLYINT_CFG_P2: c_uint = 0xC48C;
pub const R_AX_BCNERLYINT_CFG_P3: c_uint = 0xC4CC;
pub const R_AX_BCNERLYINT_CFG_P4: c_uint = 0xC50C;

pub const R_AX_TBTTERLYINT_CFG_P0: c_uint = 0xC40E;
pub const R_AX_TBTTERLYINT_CFG_P1: c_uint = 0xC44E;
pub const R_AX_TBTTERLYINT_CFG_P2: c_uint = 0xC48E;
pub const R_AX_TBTTERLYINT_CFG_P3: c_uint = 0xC4CE;
pub const R_AX_TBTTERLYINT_CFG_P4: c_uint = 0xC50E;

pub const R_AX_TBTT_AGG_P0: c_uint = 0xC412;
pub const R_AX_TBTT_AGG_P1: c_uint = 0xC452;
pub const R_AX_TBTT_AGG_P2: c_uint = 0xC492;
pub const R_AX_TBTT_AGG_P3: c_uint = 0xC4D2;
pub const R_AX_TBTT_AGG_P4: c_uint = 0xC512;

pub const R_AX_BCN_SPACE_CFG_P0: c_uint = 0xC414;
pub const R_AX_BCN_SPACE_CFG_P1: c_uint = 0xC454;
pub const R_AX_BCN_SPACE_CFG_P2: c_uint = 0xC494;
pub const R_AX_BCN_SPACE_CFG_P3: c_uint = 0xC4D4;
pub const R_AX_BCN_SPACE_CFG_P4: c_uint = 0xC514;

pub const R_AX_BCN_FORCETX_P0: c_uint = 0xC418;
pub const R_AX_BCN_FORCETX_P1: c_uint = 0xC458;
pub const R_AX_BCN_FORCETX_P2: c_uint = 0xC498;
pub const R_AX_BCN_FORCETX_P3: c_uint = 0xC4D8;
pub const R_AX_BCN_FORCETX_P4: c_uint = 0xC518;

pub const R_AX_BCN_ERR_CNT_P0: c_uint = 0xC420;
pub const R_AX_BCN_ERR_CNT_P1: c_uint = 0xC460;
pub const R_AX_BCN_ERR_CNT_P2: c_uint = 0xC4A0;
pub const R_AX_BCN_ERR_CNT_P3: c_uint = 0xC4E0;
pub const R_AX_BCN_ERR_CNT_P4: c_uint = 0xC520;

pub const R_AX_BCN_ERR_FLAG_P0: c_uint = 0xC424;
pub const R_AX_BCN_ERR_FLAG_P1: c_uint = 0xC464;
pub const R_AX_BCN_ERR_FLAG_P2: c_uint = 0xC4A4;
pub const R_AX_BCN_ERR_FLAG_P3: c_uint = 0xC4E4;
pub const R_AX_BCN_ERR_FLAG_P4: c_uint = 0xC524;

pub const R_AX_DTIM_CTRL_P0: c_uint = 0xC426;
pub const R_AX_DTIM_CTRL_P1: c_uint = 0xC466;
pub const R_AX_DTIM_CTRL_P2: c_uint = 0xC4A6;
pub const R_AX_DTIM_CTRL_P3: c_uint = 0xC4E6;
pub const R_AX_DTIM_CTRL_P4: c_uint = 0xC526;

pub const R_AX_TBTT_SHIFT_P0: c_uint = 0xC428;
pub const R_AX_TBTT_SHIFT_P1: c_uint = 0xC468;
pub const R_AX_TBTT_SHIFT_P2: c_uint = 0xC4A8;
pub const R_AX_TBTT_SHIFT_P3: c_uint = 0xC4E8;
pub const R_AX_TBTT_SHIFT_P4: c_uint = 0xC528;

pub const R_AX_BCN_CNT_TMR_P0: c_uint = 0xC434;
pub const R_AX_BCN_CNT_TMR_P1: c_uint = 0xC474;
pub const R_AX_BCN_CNT_TMR_P2: c_uint = 0xC4B4;
pub const R_AX_BCN_CNT_TMR_P3: c_uint = 0xC4F4;
pub const R_AX_BCN_CNT_TMR_P4: c_uint = 0xC534;

pub const R_AX_TSFTR_LOW_P0: c_uint = 0xC438;
pub const R_AX_TSFTR_LOW_P1: c_uint = 0xC478;
pub const R_AX_TSFTR_LOW_P2: c_uint = 0xC4B8;
pub const R_AX_TSFTR_LOW_P3: c_uint = 0xC4F8;
pub const R_AX_TSFTR_LOW_P4: c_uint = 0xC538;

pub const R_AX_TSFTR_HIGH_P0: c_uint = 0xC43C;
pub const R_AX_TSFTR_HIGH_P1: c_uint = 0xC47C;
pub const R_AX_TSFTR_HIGH_P2: c_uint = 0xC4BC;
pub const R_AX_TSFTR_HIGH_P3: c_uint = 0xC4FC;
pub const R_AX_TSFTR_HIGH_P4: c_uint = 0xC53C;

pub const R_AX_BCN_DROP_ALL0: c_uint = 0xC560;
pub const R_AX_BCN_DROP_ALL0_C1: c_uint = 0xE560;

pub const R_AX_MBSSID_CTRL: c_uint = 0xC568;
pub const R_AX_MBSSID_CTRL_C1: c_uint = 0xE568;

pub const R_AX_P0MB_HGQ_WINDOW_CFG_0: c_uint = 0xC590;
pub const R_AX_P0MB_HGQ_WINDOW_CFG_0_C1: c_uint = 0xE590;
pub const R_AX_PORT_HGQ_WINDOW_CFG: c_uint = 0xC5A0;
pub const R_AX_PORT_HGQ_WINDOW_CFG_C1: c_uint = 0xE5A0;
pub const R_AX_PTCL_COMMON_SETTING_0: c_uint = 0xC600;
pub const R_AX_PTCL_COMMON_SETTING_0_C1: c_uint = 0xE600;

pub const R_AX_AMPDU_AGG_LIMIT: c_uint = 0xC610;

pub const R_AX_AGG_LEN_HT_0: c_uint = 0xC614;
pub const R_AX_AGG_LEN_HT_0_C1: c_uint = 0xE614;

pub const R_AX_AGG_LEN_VHT_0: c_uint = 0xC618;
pub const R_AX_AGG_LEN_VHT_0_C1: c_uint = 0xE618;

pub const S_AX_CTS2S_TH_SEC_256B: c_int = 1;
pub const R_AX_SIFS_SETTING: c_uint = 0xC624;
pub const R_AX_SIFS_SETTING_C1: c_uint = 0xE624;

pub const B_AX_SPEC_SIFS_OFDM_PTCL_SH: c_int = 8;

pub const S_AX_CTS2S_TH_1K: c_int = 4;
pub const R_AX_TXRATE_CHK: c_uint = 0xC628;
pub const R_AX_TXRATE_CHK_C1: c_uint = 0xE628;

pub const R_AX_TXCNT: c_uint = 0xC62C;
pub const R_AX_TXCNT_C1: c_uint = 0xE62C;

pub const R_AX_MBSSID_DROP_0: c_uint = 0xC63C;
pub const R_AX_MBSSID_DROP_0_C1: c_uint = 0xE63C;

pub const R_AX_PTCLRPT_FULL_HDL: c_uint = 0xC660;
pub const R_AX_PTCLRPT_FULL_HDL_C1: c_uint = 0xE660;

pub const FWD_TO_WLCPU: c_int = 1;

pub const R_AX_BT_PLT: c_uint = 0xC67C;
pub const R_AX_BT_PLT_C1: c_uint = 0xE67C;

pub const R_AX_PTCL_BSS_COLOR_0: c_uint = 0xC6A0;
pub const R_AX_PTCL_BSS_COLOR_0_C1: c_uint = 0xE6A0;

pub const R_AX_PTCL_BSS_COLOR_1: c_uint = 0xC6A4;
pub const R_AX_PTCL_BSS_COLOR_1_C1: c_uint = 0xE6A4;

pub const R_AX_PTCL_IMR0: c_uint = 0xC6C0;
pub const R_AX_PTCL_IMR0_C1: c_uint = 0xE6C0;

pub const R_AX_PTCL_ISR0: c_uint = 0xC6C4;
pub const R_AX_PTCL_ISR0_C1: c_uint = 0xE6C4;
pub const S_AX_PTCL_TO_2MS: c_uint = 0x3F;
pub const R_AX_PTCL_FSM_MON: c_uint = 0xC6E8;
pub const R_AX_PTCL_FSM_MON_C1: c_uint = 0xE6E8;

pub const R_AX_PTCL_TX_CTN_SEL: c_uint = 0xC6EC;
pub const R_AX_PTCL_TX_CTN_SEL_C1: c_uint = 0xE6EC;

pub const R_AX_PTCL_DBG_INFO: c_uint = 0xC6F0;
pub const R_AX_PTCL_DBG_INFO_C1: c_uint = 0xE6F0;

pub const R_AX_PTCL_DBG: c_uint = 0xC6F4;
pub const R_AX_PTCL_DBG_C1: c_uint = 0xE6F4;

pub const AX_PTCL_DBG_BCNQ_NUM0: c_int = 8;
pub const AX_PTCL_DBG_BCNQ_NUM1: c_int = 9;
pub const R_AX_DLE_CTRL: c_uint = 0xC800;
pub const R_AX_DLE_CTRL_C1: c_uint = 0xE800;

pub const R_AX_RX_ERR_FLAG: c_uint = 0xC800;
pub const R_AX_RX_ERR_FLAG_C1: c_uint = 0xE800;

pub const R_AX_RXDMA_CTRL_0: c_uint = 0xC804;
pub const R_AX_RXDMA_CTRL_0_C1: c_uint = 0xE804;

pub const R_AX_RX_CTRL0: c_uint = 0xC808;
pub const R_AX_RX_CTRL0_C1: c_uint = 0xE808;

pub const R_AX_RX_CTRL1: c_uint = 0xC80C;
pub const R_AX_RX_CTRL1_C1: c_uint = 0xE80C;

pub const R_AX_RX_CTRL2: c_uint = 0xC810;
pub const R_AX_RX_CTRL2_C1: c_uint = 0xE810;

pub const R_AX_RXDMA_PKT_INFO_0: c_uint = 0xC814;
pub const R_AX_RXDMA_PKT_INFO_1: c_uint = 0xC818;
pub const R_AX_RXDMA_PKT_INFO_2: c_uint = 0xC81C;
pub const R_AX_RX_ERR_FLAG_IMR: c_uint = 0xC804;
pub const R_AX_RX_ERR_FLAG_IMR_C1: c_uint = 0xE804;

pub const R_AX_TX_ERR_FLAG_IMR: c_uint = 0xC870;
pub const R_AX_TX_ERR_FLAG_IMR_C1: c_uint = 0xE870;

pub const R_AX_TCR0: c_uint = 0xCA00;
pub const R_AX_TCR0_C1: c_uint = 0xEA00;

pub const TCR_UDF_THSD: c_uint = 0x6;

pub const R_AX_TCR1: c_uint = 0xCA04;
pub const R_AX_TCR1_C1: c_uint = 0xEA04;

pub const R_AX_MD_TSFT_STMP_CTL: c_uint = 0xCA08;
pub const R_AX_MD_TSFT_STMP_CTL_C1: c_uint = 0xEA08;

pub const R_AX_PPWRBIT_SETTING: c_uint = 0xCA0C;
pub const R_AX_PPWRBIT_SETTING_C1: c_uint = 0xEA0C;
pub const R_AX_TXD_FIFO_CTRL: c_uint = 0xCA1C;
pub const R_AX_TXD_FIFO_CTRL_C1: c_uint = 0xEA1C;

pub const TXDFIFO_HIGH_MCS_THRE: c_uint = 0x7;

pub const TXDFIFO_LOW_MCS_THRE: c_uint = 0x7;

pub const R_AX_MACTX_DBG_SEL_CNT: c_uint = 0xCA20;
pub const R_AX_MACTX_DBG_SEL_CNT_C1: c_uint = 0xEA20;

pub const R_AX_WMAC_TX_CTRL_DEBUG: c_uint = 0xCAE4;
pub const R_AX_WMAC_TX_CTRL_DEBUG_C1: c_uint = 0xEAE4;

pub const R_AX_WMAC_TX_INFO0_DEBUG: c_uint = 0xCAE8;
pub const R_AX_WMAC_TX_INFO0_DEBUG_C1: c_uint = 0xEAE8;

pub const R_AX_WMAC_TX_INFO1_DEBUG: c_uint = 0xCAEC;
pub const R_AX_WMAC_TX_INFO1_DEBUG_C1: c_uint = 0xEAEC;

pub const R_AX_RSP_CHK_SIG: c_uint = 0xCC00;
pub const R_AX_RSP_CHK_SIG_C1: c_uint = 0xEC00;

pub const R_AX_TRXPTCL_RESP_0: c_uint = 0xCC04;
pub const R_AX_TRXPTCL_RESP_0_C1: c_uint = 0xEC04;

pub const WMAC_SPEC_SIFS_OFDM_52A: c_uint = 0x15;
pub const WMAC_SPEC_SIFS_OFDM_52B: c_uint = 0x11;
pub const WMAC_SPEC_SIFS_OFDM_52C: c_uint = 0x11;
pub const WMAC_SPEC_SIFS_CCK: c_uint = 0xA;
pub const R_AX_TRXPTCL_RRSR_CTL_0: c_uint = 0xCC08;
pub const R_AX_TRXPTCL_RRSR_CTL_0_C1: c_uint = 0xEC08;

pub const R_AX_MAC_LOOPBACK: c_uint = 0xCC20;
pub const R_AX_MAC_LOOPBACK_C1: c_uint = 0xEC20;

pub const R_AX_WMAC_NAV_CTL: c_uint = 0xCC80;
pub const R_AX_WMAC_NAV_CTL_C1: c_uint = 0xEC80;

pub const NAV_12MS: c_uint = 0xBC;
pub const NAV_25MS: c_uint = 0xC4;

pub const R_AX_RXTRIG_TEST_USER_2: c_uint = 0xCCB0;
pub const R_AX_RXTRIG_TEST_USER_2_C1: c_uint = 0xECB0;

pub const R_AX_TRXPTCL_ERROR_INDICA_MASK: c_uint = 0xCCBC;
pub const R_AX_TRXPTCL_ERROR_INDICA_MASK_C1: c_uint = 0xECBC;

pub const R_AX_TRXPTCL_ERROR_INDICA: c_uint = 0xCCC0;
pub const R_AX_TRXPTCL_ERROR_INDICA_C1: c_uint = 0xECC0;

pub const R_AX_WMAC_TX_TF_INFO_0: c_uint = 0xCCD0;
pub const R_AX_WMAC_TX_TF_INFO_0_C1: c_uint = 0xECD0;

pub const R_AX_WMAC_TX_TF_INFO_1: c_uint = 0xCCD4;
pub const R_AX_WMAC_TX_TF_INFO_1_C1: c_uint = 0xECD4;

pub const R_AX_WMAC_TX_TF_INFO_2: c_uint = 0xCCD8;
pub const R_AX_WMAC_TX_TF_INFO_2_C1: c_uint = 0xECD8;

pub const R_AX_TMAC_ERR_IMR_ISR: c_uint = 0xCCEC;
pub const R_AX_TMAC_ERR_IMR_ISR_C1: c_uint = 0xECEC;

pub const R_AX_DBGSEL_TRXPTCL: c_uint = 0xCCF4;
pub const R_AX_DBGSEL_TRXPTCL_C1: c_uint = 0xECF4;

pub const R_AX_PHYINFO_ERR_IMR_V1: c_uint = 0xCCF8;
pub const R_AX_PHYINFO_ERR_IMR_V1_C1: c_uint = 0xECF8;

pub const R_AX_PHYINFO_ERR_IMR: c_uint = 0xCCFC;
pub const R_AX_PHYINFO_ERR_IMR_C1: c_uint = 0xECFC;

pub const R_AX_PHYINFO_ERR_ISR: c_uint = 0xCCFC;
pub const R_AX_PHYINFO_ERR_ISR_C1: c_uint = 0xECFC;
pub const R_AX_BFMER_CTRL_0: c_uint = 0xCD78;
pub const R_AX_BFMER_CTRL_0_C1: c_uint = 0xED78;

pub const R_AX_BFMEE_RESP_OPTION: c_uint = 0xCD80;
pub const R_AX_BFMEE_RESP_OPTION_C1: c_uint = 0xED80;

pub const BFRP_RX_STANDBY_TIMER_KEEP: c_uint = 0x0;
pub const BFRP_RX_STANDBY_TIMER_RELEASE: c_uint = 0x1;

pub const BFRP_RX_STANDBY_TIMER: c_uint = 0x0;
pub const NDP_RX_STANDBY_TIMER: c_uint = 0xFF;

pub const R_AX_TRXPTCL_RESP_CSI_CTRL_0: c_uint = 0xCD88;
pub const R_AX_TRXPTCL_RESP_CSI_CTRL_0_C1: c_uint = 0xED88;
pub const R_AX_TRXPTCL_RESP_CSI_CTRL_1: c_uint = 0xCD94;
pub const R_AX_TRXPTCL_RESP_CSI_CTRL_1_C1: c_uint = 0xED94;

pub const R_AX_TRXPTCL_RESP_CSI_RRSC: c_uint = 0xCD8C;
pub const R_AX_TRXPTCL_RESP_CSI_RRSC_C1: c_uint = 0xED8C;
pub const CSI_RRSC_BMAP: c_uint = 0x29292911;
pub const R_AX_TRXPTCL_RESP_CSI_RATE: c_uint = 0xCD90;
pub const R_AX_TRXPTCL_RESP_CSI_RATE_C1: c_uint = 0xED90;

pub const CSI_INIT_RATE_HE: c_uint = 0x3;
pub const CSI_INIT_RATE_VHT: c_uint = 0x3;
pub const CSI_INIT_RATE_HT: c_uint = 0x3;
pub const R_AX_RCR: c_uint = 0xCE00;
pub const R_AX_RCR_C1: c_uint = 0xEE00;

pub const R_AX_DLK_PROTECT_CTL: c_uint = 0xCE02;
pub const R_AX_DLK_PROTECT_CTL_C1: c_uint = 0xEE02;

pub const R_AX_PLCP_HDR_FLTR: c_uint = 0xCE04;
pub const R_AX_PLCP_HDR_FLTR_C1: c_uint = 0xEE04;

pub const R_AX_RX_FLTR_OPT: c_uint = 0xCE20;
pub const R_AX_RX_FLTR_OPT_C1: c_uint = 0xEE20;

pub const B_AX_UNSPT_FILTER_SH: c_int = 22;

pub const B_AX_RX_MPDU_MAX_LEN_SIZE: c_uint = 0x3f;

pub const R_AX_CTRL_FLTR: c_uint = 0xCE24;
pub const R_AX_CTRL_FLTR_C1: c_uint = 0xEE24;
pub const R_AX_MGNT_FLTR: c_uint = 0xCE28;
pub const R_AX_MGNT_FLTR_C1: c_uint = 0xEE28;
pub const R_AX_DATA_FLTR: c_uint = 0xCE2C;
pub const R_AX_DATA_FLTR_C1: c_uint = 0xEE2C;
pub const RX_FLTR_FRAME_DROP: c_uint = 0x00000000;
pub const RX_FLTR_FRAME_TO_HOST: c_uint = 0x55555555;
pub const RX_FLTR_FRAME_TO_WLCPU: c_uint = 0xAAAAAAAA;
pub const R_AX_ADDR_CAM_CTRL: c_uint = 0xCE34;
pub const R_AX_ADDR_CAM_CTRL_C1: c_uint = 0xEE34;

pub const R_AX_RESPBA_CAM_CTRL: c_uint = 0xCE3C;
pub const R_AX_RESPBA_CAM_CTRL_C1: c_uint = 0xEE3C;

pub const S_AX_BACAM_RST_ALL: c_int = 2;
pub const R_AX_PPDU_STAT: c_uint = 0xCE40;
pub const R_AX_PPDU_STAT_C1: c_uint = 0xEE40;

pub const R_AX_RX_SR_CTRL: c_uint = 0xCE4A;
pub const R_AX_RX_SR_CTRL_C1: c_uint = 0xEE4A;

pub const R_AX_BSSID_SRC_CTRL: c_uint = 0xCE4B;
pub const R_AX_BSSID_SRC_CTRL_C1: c_uint = 0xEE4B;

pub const R_AX_CSIRPT_OPTION: c_uint = 0xCE64;
pub const R_AX_CSIRPT_OPTION_C1: c_uint = 0xEE64;

pub const R_AX_BCN_PSR_RPT_P0: c_uint = 0xCE84;
pub const R_AX_BCN_PSR_RPT_P0_C1: c_uint = 0xEE84;

pub const R_AX_RX_STATE_MONITOR: c_uint = 0xCEF0;
pub const R_AX_RX_STATE_MONITOR_C1: c_uint = 0xEEF0;

pub const R_AX_RMAC_ERR_ISR: c_uint = 0xCEF4;
pub const R_AX_RMAC_ERR_ISR_C1: c_uint = 0xEEF4;

pub const R_AX_RX_ERR_IMR: c_uint = 0xCEF8;
pub const R_AX_RX_ERR_IMR_C1: c_uint = 0xEEF8;

pub const R_AX_RMAC_PLCP_MON: c_uint = 0xCEF8;
pub const R_AX_RMAC_PLCP_MON_C1: c_uint = 0xEEF8;

pub const R_AX_RX_DEBUG_SELECT: c_uint = 0xCEFC;
pub const R_AX_RX_DEBUG_SELECT_C1: c_uint = 0xEEFC;

pub const R_AX_PWR_RATE_CTRL: c_uint = 0xD200;
pub const R_AX_PWR_RATE_CTRL_C1: c_uint = 0xF200;

pub const R_AX_PWR_RATE_OFST_CTRL: c_uint = 0xD204;
pub const R_AX_PWR_COEXT_CTRL: c_uint = 0xD220;

pub const R_AX_PWR_SWING_OTHER_CTRL0: c_uint = 0xD230;
pub const R_AX_PWR_SWING_OTHER_CTRL0_C1: c_uint = 0xF230;

pub const R_AX_PWR_UL_CTRL0: c_uint = 0xD240;
pub const R_AX_PWR_UL_CTRL2: c_uint = 0xD248;

pub const B_AX_PWR_UL_CTRL2_MASK: c_uint = 0x07700007;
pub const R_AX_PWR_NORM_FORCE1: c_uint = 0xD260;
pub const R_AX_PWR_NORM_FORCE1_C1: c_uint = 0xF260;

pub const R_AX_PWR_UL_TB_CTRL: c_uint = 0xD288;

pub const R_AX_PWR_UL_TB_1T: c_uint = 0xD28C;

pub const R_AX_PWR_UL_TB_2T: c_uint = 0xD290;

pub const R_AX_PWR_BY_RATE_TABLE0: c_uint = 0xD2C0;
pub const R_AX_PWR_BY_RATE_TABLE6: c_uint = 0xD2D8;
pub const R_AX_PWR_BY_RATE_TABLE10: c_uint = 0xD2E8;

pub const R_AX_PWR_LMT_TABLE0: c_uint = 0xD2EC;
pub const R_AX_PWR_LMT_TABLE9: c_uint = 0xD310;
pub const R_AX_PWR_LMT_TABLE19: c_uint = 0xD338;

pub const R_AX_PWR_RU_LMT_TABLE0: c_uint = 0xD33C;
pub const R_AX_PWR_RU_LMT_TABLE5: c_uint = 0xD350;
pub const R_AX_PWR_RU_LMT_TABLE11: c_uint = 0xD368;

pub const R_AX_PWR_MACID_LMT_TABLE0: c_uint = 0xD36C;
pub const R_AX_PWR_MACID_LMT_TABLE127: c_uint = 0xD568;
pub const R_AX_PATH_COM0: c_uint = 0xD800;
pub const AX_PATH_COM0_DFVAL: c_uint = 0x00000000;
pub const AX_PATH_COM0_PATHA: c_uint = 0x08889880;
pub const AX_PATH_COM0_PATHB: c_uint = 0x11111900;
pub const AX_PATH_COM0_PATHAB: c_uint = 0x19999980;
pub const R_AX_PATH_COM1: c_uint = 0xD804;

pub const AX_PATH_COM1_DFVAL: c_uint = 0x00000000;
pub const AX_PATH_COM1_PATHA: c_uint = 0x13111111;
pub const AX_PATH_COM1_PATHB: c_uint = 0x23222222;
pub const AX_PATH_COM1_PATHAB: c_uint = 0x33333333;
pub const R_AX_PATH_COM2: c_uint = 0xD808;

pub const AX_PATH_COM2_DFVAL: c_uint = 0x00000000;
pub const AX_PATH_COM2_PATHA: c_uint = 0x01209313;
pub const AX_PATH_COM2_PATHB: c_uint = 0x01209323;
pub const AX_PATH_COM2_PATHAB: c_uint = 0x01209333;
pub const R_AX_PATH_COM3: c_uint = 0xD80C;
pub const AX_PATH_COM3_DFVAL: c_uint = 0x49249249;
pub const R_AX_PATH_COM4: c_uint = 0xD810;
pub const AX_PATH_COM4_DFVAL: c_uint = 0x1C9C9C49;
pub const R_AX_PATH_COM5: c_uint = 0xD814;
pub const AX_PATH_COM5_DFVAL: c_uint = 0x39393939;
pub const R_AX_PATH_COM6: c_uint = 0xD818;
pub const AX_PATH_COM6_DFVAL: c_uint = 0x39393939;
pub const R_AX_PATH_COM7: c_uint = 0xD81C;
pub const AX_PATH_COM7_DFVAL: c_uint = 0x39393939;
pub const AX_PATH_COM7_PATHA: c_uint = 0x39393939;
pub const AX_PATH_COM7_PATHB: c_uint = 0x39383939;
pub const AX_PATH_COM7_PATHAB: c_uint = 0x39393939;
pub const R_AX_PATH_COM8: c_uint = 0xD820;
pub const AX_PATH_COM8_DFVAL: c_uint = 0x00000000;
pub const AX_PATH_COM8_PATHA: c_uint = 0x00003939;
pub const AX_PATH_COM8_PATHB: c_uint = 0x00003938;
pub const AX_PATH_COM8_PATHAB: c_uint = 0x00003939;
pub const R_AX_PATH_COM9: c_uint = 0xD824;
pub const AX_PATH_COM9_DFVAL: c_uint = 0x000007C0;
pub const R_AX_PATH_COM10: c_uint = 0xD828;
pub const AX_PATH_COM10_DFVAL: c_uint = 0xE0000000;
pub const R_AX_PATH_COM11: c_uint = 0xD82C;
pub const AX_PATH_COM11_DFVAL: c_uint = 0x00000000;
pub const R_P80_AT_HIGH_FREQ_BB_WRP: c_uint = 0xD848;

pub const R_AX_TSSI_CTRL_HEAD: c_uint = 0xD908;
pub const R_AX_BANDEDGE_CFG: c_uint = 0xD94C;

pub const R_AX_TSSI_CTRL_TAIL: c_uint = 0xD95C;
pub const R_AX_TXPWR_IMR: c_uint = 0xD9E0;
pub const R_AX_TXPWR_IMR_C1: c_uint = 0xF9E0;
pub const R_AX_TXPWR_ISR: c_uint = 0xD9E4;
pub const R_AX_TXPWR_ISR_C1: c_uint = 0xF9E4;
pub const R_AX_BTC_CFG: c_uint = 0xDA00;

pub const R_AX_RTK_MODE_CFG_V1: c_uint = 0xDA04;
pub const R_AX_RTK_MODE_CFG_V1_C1: c_uint = 0xFA04;

pub const R_AX_WL_PRI_MSK: c_uint = 0xDA10;

pub const R_AX_BT_CNT_CFG: c_uint = 0xDA10;
pub const R_AX_BT_CNT_CFG_C1: c_uint = 0xFA10;

pub const R_BTC_BT_CNT_HIGH: c_uint = 0xDA14;
pub const R_BTC_BT_CNT_LOW: c_uint = 0xDA18;
pub const R_AX_BTC_FUNC_EN: c_uint = 0xDA20;
pub const R_AX_BTC_FUNC_EN_C1: c_uint = 0xFA20;

pub const R_BTC_COEX_WL_REQ: c_uint = 0xDA24;
pub const R_BTC_COEX_WL_REQ_BE: c_uint = 0xE324;

pub const R_BTC_BREAK_TABLE: c_uint = 0xDA2C;
pub const BTC_BREAK_PARAM: c_uint = 0xf0ffffff;
pub const R_BTC_BT_COEX_MSK_TABLE: c_uint = 0xDA30;

pub const R_AX_BT_COEX_CFG_2: c_uint = 0xDA34;
pub const R_AX_BT_COEX_CFG_2_C1: c_uint = 0xFA34;

pub const MAC_AX_CSR_RATE: c_int = 80;
pub const R_AX_CSR_MODE: c_uint = 0xDA40;
pub const R_AX_CSR_MODE_C1: c_uint = 0xFA40;

pub const MAC_AX_CSR_DELAY: c_int = 0;

pub const MAC_AX_CSR_TRX_TO: c_int = 4;

pub const MAC_AX_CSR_PRI_TO: c_int = 5;

pub const R_AX_BT_BREAK_TABLE: c_uint = 0xDA44;
pub const R_AX_BT_STAST_HIGH: c_uint = 0xDA44;

pub const R_AX_BT_STAST_LOW: c_uint = 0xDA48;

pub const R_AX_GNT_SW_CTRL: c_uint = 0xDA48;
pub const R_AX_GNT_SW_CTRL_C1: c_uint = 0xFA48;

pub const R_AX_GNT_VAL: c_uint = 0x0054;

pub const R_AX_GNT_VAL_V1: c_uint = 0xDA4C;

pub const R_AX_TDMA_MODE: c_uint = 0xDA4C;
pub const R_AX_TDMA_MODE_C1: c_uint = 0xFA4C;

pub const R_AX_BT_COEX_CFG_5: c_uint = 0xDA6C;
pub const R_AX_BT_COEX_CFG_5_C1: c_uint = 0xFA6C;

pub const MAC_AX_RTK_RATE: c_int = 5;
pub const R_AX_LTE_CTRL: c_uint = 0xDAF0;
pub const R_AX_LTE_WDATA: c_uint = 0xDAF4;
pub const R_AX_LTE_RDATA: c_uint = 0xDAF8;
pub const R_AX_MACID_ANT_TABLE: c_uint = 0xDC00;
pub const R_AX_MACID_ANT_TABLE_LAST: c_uint = 0xDDFC;
pub const CMAC1_START_ADDR_AX: c_uint = 0xE000;
pub const CMAC1_END_ADDR_AX: c_uint = 0xFFFF;
pub const R_AX_CMAC_REG_END: c_uint = 0xFFFF;
pub const R_AX_LTE_SW_CFG_1: c_uint = 0x0038;
pub const R_AX_LTE_SW_CFG_1_C1: c_uint = 0x2038;

pub const R_AX_LTE_SW_CFG_2: c_uint = 0x003C;
pub const R_AX_LTE_SW_CFG_2_C1: c_uint = 0x203C;

pub const R_BE_SYS_ISO_CTRL: c_uint = 0x0000;

pub const R_BE_SYS_PW_CTRL: c_uint = 0x0004;

pub const R_BE_SYS_CLK_CTRL: c_uint = 0x0008;

pub const R_BE_SYS_WL_EFUSE_CTRL: c_uint = 0x000A;

pub const R_BE_SYS_PAGE_CLK_GATED: c_uint = 0x000C;

pub const R_BE_ANAPAR_POW_MAC: c_uint = 0x0016;

pub const R_BE_SYS_ADIE_PAD_PWR_CTRL: c_uint = 0x0018;

pub const R_BE_RSV_CTRL: c_uint = 0x001C;

pub const R_BE_AFE_LDO_CTRL: c_uint = 0x0020;

pub const R_BE_AFE_CTRL1: c_uint = 0x0024;

pub const R_BE_EFUSE_CTRL: c_uint = 0x0030;

pub const R_BE_EFUSE_CTRL_1_V1: c_uint = 0x0034;

pub const R_BE_GPIO_MUXCFG: c_uint = 0x0040;

pub const R_BE_GPIO_EXT_CTRL: c_uint = 0x0060;

pub const R_BE_WL_BT_PWR_CTRL: c_uint = 0x0068;

pub const R_BE_SYS_SDIO_CTRL: c_uint = 0x0070;

pub const R_BE_HCI_OPT_CTRL: c_uint = 0x0074;

pub const R_BE_SYS_ISO_CTRL_EXTEND: c_uint = 0x0080;

pub const R_BE_FEN_RST_ENABLE: c_uint = 0x0084;

pub const R_BE_PLATFORM_ENABLE: c_uint = 0x0088;

pub const R_BE_WLLPS_CTRL: c_uint = 0x0090;

pub const R_BE_WLRESUME_CTRL: c_uint = 0x0094;

pub const R_BE_SYSON_FSM_MON: c_uint = 0x00A0;

pub const WLAN_FSM_MASK: c_uint = 0xFFFFFF;
pub const WLAN_FSM_SET: c_uint = 0x4000000;
pub const WLAN_FSM_STATE_MASK: c_uint = 0x1FF;
pub const WLAN_FSM_IDLE: c_int = 0;
pub const R_BE_EFUSE_CTRL_2_V1: c_uint = 0x00A4;

pub const R_BE_SCOREBOARD: c_uint = 0x00AC;

pub const R_BE_PAD_CTRL2: c_uint = 0x00C4;

pub const R_BE_PMC_DBG_CTRL2: c_uint = 0x00CC;

pub const R_BE_MEM_PWR_CTRL: c_uint = 0x00D0;

pub const R_BE_PCIE_MIO_INTF: c_uint = 0x00E4;

pub const R_BE_PCIE_MIO_INTD: c_uint = 0x00E8;

pub const R_BE_SYS_CHIPINFO: c_uint = 0x00FC;

pub const R_BE_SCOREBOARD_0: c_uint = 0x0110;

pub const R_BE_SCOREBOARD_0_BT_DATA: c_uint = 0x0114;

pub const R_BE_SCOREBOARD_1: c_uint = 0x0118;

pub const R_BE_SCOREBOARD_1_BT_DATA: c_uint = 0x011C;

pub const R_BE_HALT_H2C_CTRL: c_uint = 0x0160;

pub const R_BE_HALT_C2H_CTRL: c_uint = 0x0164;

pub const R_BE_HALT_H2C: c_uint = 0x0168;

pub const R_BE_HALT_C2H: c_uint = 0x016C;

pub const R_BE_SYS_CFG5: c_uint = 0x0170;

pub const R_BE_SECURE_BOOT_MALLOC_INFO: c_uint = 0x0184;
pub const R_BE_FW_AUTO_CAL_DELAY: c_uint = 0x0188;

pub const R_BE_FWS0IMR: c_uint = 0x0190;

pub const R_BE_FWS0ISR: c_uint = 0x0194;

pub const R_BE_FWS1IMR: c_uint = 0x0198;

pub const R_BE_FWS1ISR: c_uint = 0x019C;

pub const R_BE_HIMR0: c_uint = 0x01A0;

pub const R_BE_HISR0: c_uint = 0x01A4;

pub const R_BE_WCPU_FW_CTRL: c_uint = 0x01E0;

pub const R_BE_BOOT_REASON: c_uint = 0x01E6;

pub const R_BE_LDM: c_uint = 0x01E8;

pub const R_BE_UDM0: c_uint = 0x01F0;

pub const R_BE_UDM1: c_uint = 0x01F4;

pub const R_BE_UDM2: c_uint = 0x01F8;

pub const R_BE_SPS_DIG_ON_CTRL0: c_uint = 0x0200;

pub const R_BE_SPS_DIG_ON_CTRL1: c_uint = 0x0204;

pub const R_BE_SPS_ANA_ON_CTRL1: c_uint = 0x0224;

pub const R_BE_AFE_ON_CTRL0: c_uint = 0x0240;

pub const R_BE_AFE_ON_CTRL1: c_uint = 0x0244;

pub const R_BE_AFE_ON_CTRL3: c_uint = 0x024C;

pub const R_BE_GPIO8_15_FUNC_SEL: c_uint = 0x02D4;

pub const R_BE_WLAN_XTAL_SI_CTRL: c_uint = 0x0270;

pub const R_BE_PCIE_SER_DBG: c_uint = 0x02FC;

pub const R_BE_IC_PWR_STATE: c_uint = 0x03F0;

pub const MAC_AX_SYS_ACT: c_uint = 0x220;

pub const R_BE_WLCPU_PORT_PC: c_uint = 0x03FC;
pub const R_BE_DBG_WOW: c_uint = 0x0504;
pub const R_BE_DCPU_PLATFORM_ENABLE: c_uint = 0x0888;

pub const R_BE_PL_AXIDMA_IDCT_MSK: c_uint = 0x0910;

pub const R_BE_PL_AXIDMA_IDCT: c_uint = 0x0914;

pub const R_BE_FILTER_MODEL_ADDR: c_uint = 0x0C04;
pub const R_BE_WLAN_WDT: c_uint = 0x3050;

pub const R_BE_AXIDMA_WDT: c_uint = 0x305C;

pub const R_BE_AON_WDT: c_uint = 0x3068;

pub const R_BE_AON_WDT_TMR: c_uint = 0x306C;
pub const R_BE_MDIO_WDT_TMR: c_uint = 0x3090;
pub const R_BE_LA_MODE_WDT_TMR: c_uint = 0x309C;
pub const R_BE_WDT_AR_TMR: c_uint = 0x3144;
pub const R_BE_WDT_AW_TMR: c_uint = 0x3150;
pub const R_BE_WLAN_WDT_TMR: c_uint = 0x3054;
pub const R_BE_WDT_W_TMR: c_uint = 0x315C;
pub const R_BE_AXIDMA_WDT_TMR: c_uint = 0x3060;
pub const R_BE_WDT_B_TMR: c_uint = 0x3164;
pub const R_BE_WDT_R_TMR: c_uint = 0x316C;
pub const R_BE_LOCAL_WDT_TMR: c_uint = 0x3084;
pub const R_BE_LOCAL_WDT: c_uint = 0x3080;

pub const R_BE_MDIO_WDT: c_uint = 0x308C;

pub const R_BE_LA_MODE_WDT: c_uint = 0x3098;

pub const R_BE_WDT_AR: c_uint = 0x3140;

pub const R_BE_WDT_AW: c_uint = 0x314C;

pub const R_BE_WDT_W: c_uint = 0x3158;

pub const R_BE_WDT_B: c_uint = 0x3160;

pub const R_BE_WDT_R: c_uint = 0x3168;

pub const R_BE_LTR_DECISION_CTRL_V1: c_uint = 0x3610;

pub const R_BE_LTR_LATENCY_IDX0_V1: c_uint = 0x3614;
pub const R_BE_LTR_LATENCY_IDX1_V1: c_uint = 0x3618;
pub const R_BE_LTR_LATENCY_IDX2_V1: c_uint = 0x361C;
pub const R_BE_LTR_LATENCY_IDX3_V1: c_uint = 0x3620;
pub const R_BE_USB2_WLAN_TRX_OPT_PAR2: c_uint = 0x41BC;

pub const R_BE_HCI_BUF_IMR: c_uint = 0x6018;
pub const B_BE_HCI_BUF_IMR_CLR: c_uint = 0xC0000303;
pub const B_BE_HCI_BUF_IMR_SET: c_uint = 0xC0000301;
pub const R_BE_H2CREG_DATA0: c_uint = 0x7140;
pub const R_BE_H2CREG_DATA1: c_uint = 0x7144;
pub const R_BE_H2CREG_DATA2: c_uint = 0x7148;
pub const R_BE_H2CREG_DATA3: c_uint = 0x714C;
pub const R_BE_C2HREG_DATA0: c_uint = 0x7150;
pub const R_BE_C2HREG_DATA1: c_uint = 0x7154;
pub const R_BE_C2HREG_DATA2: c_uint = 0x7158;
pub const R_BE_C2HREG_DATA3: c_uint = 0x715C;
pub const R_BE_H2CREG_CTRL: c_uint = 0x7160;

pub const R_BE_C2HREG_CTRL: c_uint = 0x7164;

pub const R_BE_HCI_FUNC_EN: c_uint = 0x7880;

pub const R_BE_BOOT_DBG: c_uint = 0x78F0;

pub const R_BE_DBG_WOW_READY: c_uint = 0x815E;

pub const R_BE_DMAC_FUNC_EN: c_uint = 0x8400;

pub const R_BE_DMAC_CLK_EN: c_uint = 0x8404;

pub const R_BE_LTR_CTRL_0: c_uint = 0x8410;

pub const R_BE_LTR_CFG_0: c_uint = 0x8414;

pub const R_BE_LTR_CFG_1: c_uint = 0x8418;

pub const R_BE_NO_RX_ERR_CFG: c_uint = 0x841C;

pub const R_BE_DMAC_TABLE_CTRL: c_uint = 0x8420;

pub const R_BE_SER_DBG_INFO: c_uint = 0x8424;

pub const R_BE_DMAC_SYS_CR32B: c_uint = 0x842C;

pub const R_BE_DLE_EMPTY0: c_uint = 0x8430;

pub const R_BE_DLE_EMPTY1: c_uint = 0x8434;

pub const R_BE_SER_L1_DBG_CNT_0: c_uint = 0x8440;

pub const R_BE_SER_L1_DBG_CNT_1: c_uint = 0x8444;

pub const R_BE_SER_L1_DBG_CNT_2: c_uint = 0x8448;

pub const R_BE_SER_L1_DBG_CNT_3: c_uint = 0x844C;

pub const R_BE_SER_L1_DBG_CNT_4: c_uint = 0x8450;

pub const R_BE_SER_L1_DBG_CNT_5: c_uint = 0x8454;

pub const R_BE_SER_L1_DBG_CNT_6: c_uint = 0x8458;

pub const R_BE_SER_L1_DBG_CNT_7: c_uint = 0x845C;

pub const R_BE_FW_TRIGGER_IDCT_ISR: c_uint = 0x8508;

pub const R_BE_DMAC_ERR_IMR: c_uint = 0x8520;

pub const R_BE_DMAC_ERR_ISR: c_uint = 0x8524;

pub const R_BE_DISP_ERROR_ISR0: c_uint = 0x8804;

pub const R_BE_DISP_ERROR_ISR1: c_uint = 0x8808;

pub const R_BE_DISP_ERROR_ISR2: c_uint = 0x880C;

pub const R_BE_DISP_OTHER_IMR: c_uint = 0x8870;

pub const B_BE_DISP_OTHER_IMR_CLR_V1: c_uint = 0xFFFFFFFF;
pub const B_BE_DISP_OTHER_IMR_SET_V1: c_uint = 0x3F002000;
pub const R_BE_DISP_HOST_IMR: c_uint = 0x8874;

pub const B_BE_DISP_HOST_IMR_CLR_V1: c_uint = 0xFBFFFFFF;
pub const B_BE_DISP_HOST_IMR_SET_V1: c_uint = 0xC8B3E579;
pub const R_BE_DISP_CPU_IMR: c_uint = 0x8878;

pub const B_BE_DISP_CPU_IMR_CLR_V1: c_uint = 0x7DFFFFFD;
pub const B_BE_DISP_CPU_IMR_SET_V1: c_uint = 0x34F938FD;
pub const R_BE_RX_STOP: c_uint = 0x8914;

pub const R_BE_DISP_FWD_WLAN_0: c_uint = 0x8938;

pub const R_BE_WDE_PKTBUF_CFG: c_uint = 0x8C08;

pub const R_BE_WDE_BUFMGN_CTL: c_uint = 0x8C10;

pub const R_BE_WDE_ERR_IMR: c_uint = 0x8C38;

pub const R_BE_WDE_QTA0_CFG: c_uint = 0x8C40;

pub const R_BE_WDE_QTA1_CFG: c_uint = 0x8C44;

pub const R_BE_WDE_QTA2_CFG: c_uint = 0x8C48;

pub const R_BE_WDE_QTA3_CFG: c_uint = 0x8C4C;

pub const R_BE_WDE_QTA4_CFG: c_uint = 0x8C50;

pub const R_BE_WDE_ERR1_IMR: c_uint = 0x8CC0;

pub const R_BE_PLE_PKTBUF_CFG: c_uint = 0x9008;

pub const R_BE_PLE_BUFMGN_CTL: c_uint = 0x9010;

pub const R_BE_PLE_ERR_IMR: c_uint = 0x9038;

pub const R_BE_PLE_QTA0_CFG: c_uint = 0x9040;

pub const R_BE_PLE_QTA1_CFG: c_uint = 0x9044;

pub const R_BE_PLE_QTA2_CFG: c_uint = 0x9048;

pub const R_BE_PLE_QTA3_CFG: c_uint = 0x904C;

pub const R_BE_PLE_QTA4_CFG: c_uint = 0x9050;

pub const R_BE_PLE_QTA5_CFG: c_uint = 0x9054;

pub const R_BE_PLE_QTA6_CFG: c_uint = 0x9058;

pub const R_BE_PLE_QTA7_CFG: c_uint = 0x905C;

pub const R_BE_PLE_QTA8_CFG: c_uint = 0x9060;

pub const R_BE_PLE_QTA9_CFG: c_uint = 0x9064;

pub const R_BE_PLE_QTA10_CFG: c_uint = 0x9068;

pub const R_BE_PLE_QTA11_CFG: c_uint = 0x906C;

pub const R_BE_PLE_QTA12_CFG: c_uint = 0x9070;

pub const R_BE_PLE_QTA13_CFG: c_uint = 0x9074;

pub const R_BE_PLE_ERRFLAG1_IMR: c_uint = 0x90C0;

pub const R_BE_PLE_DBG_FUN_INTF_CTL: c_uint = 0x9110;

pub const R_BE_PLE_DBG_FUN_INTF_DATA: c_uint = 0x9114;

pub const R_BE_WDRLS_CFG: c_uint = 0x9408;

pub const R_BE_WDRLS_ERR_IMR: c_uint = 0x9430;

pub const R_BE_RLSRPT0_CFG0: c_uint = 0x9440;

pub const WDRLS_DEST_QID_POH: c_int = 1;
pub const WDRLS_DEST_QID_STF: c_int = 0;
pub const R_BE_RLSRPT0_CFG1: c_uint = 0x9444;

pub const S_BE_WDRLS_FLTR_TXOK: c_int = 1;
pub const S_BE_WDRLS_FLTR_RTYLMT: c_int = 2;
pub const S_BE_WDRLS_FLTR_LIFTIM: c_int = 4;
pub const S_BE_WDRLS_FLTR_MACID: c_int = 8;

pub const R_BE_BBRPT_COM_ERR_IMR: c_uint = 0x9608;

pub const R_BE_BBRPT_CHINFO_ERR_IMR: c_uint = 0x9628;

pub const R_BE_BBRPT_DFS_ERR_IMR: c_uint = 0x9638;

pub const R_BE_LA_ERRFLAG_IMR: c_uint = 0x9668;

pub const R_BE_LA_ERRFLAG_ISR: c_uint = 0x966C;

pub const R_BE_CH_INFO_DBGFLAG_IMR: c_uint = 0x9688;

pub const B_BE_CH_INFO_DBGFLAG_IMR_SET: c_int = 0;
pub const R_BE_WD_BUF_REQ: c_uint = 0x9800;

pub const R_BE_WD_BUF_STATUS: c_uint = 0x9804;

pub const R_BE_WD_CPUQ_OP_0: c_uint = 0x9810;

pub const R_BE_WD_CPUQ_OP_1: c_uint = 0x9814;

pub const R_BE_WD_CPUQ_OP_2: c_uint = 0x9818;

pub const R_BE_WD_CPUQ_OP_3: c_uint = 0x981C;

pub const R_BE_WD_CPUQ_OP_STATUS: c_uint = 0x9820;

pub const R_BE_PL_BUF_REQ: c_uint = 0x9840;

pub const R_BE_PL_BUF_STATUS: c_uint = 0x9844;

pub const R_BE_PL_CPUQ_OP_0: c_uint = 0x9850;

pub const R_BE_PL_CPUQ_OP_1: c_uint = 0x9854;

pub const R_BE_PL_CPUQ_OP_2: c_uint = 0x9858;

pub const R_BE_PL_CPUQ_OP_3: c_uint = 0x985C;

pub const R_BE_PL_CPUQ_OP_STATUS: c_uint = 0x9860;

pub const R_BE_CPUIO_ERR_IMR: c_uint = 0x9888;

pub const R_BE_PKTIN_ERR_IMR: c_uint = 0x9A20;

pub const R_BE_HDR_SHCUT_SETTING: c_uint = 0x9B00;

pub const R_BE_MPDU_TX_ERR_IMR: c_uint = 0x9BF4;

pub const B_BE_MPDU_TX_ERR_IMR_SET: c_int = 0;
pub const R_BE_MPDU_PROC: c_uint = 0x9C00;

pub const R_BE_FWD_ERR: c_uint = 0x9C10;
pub const R_BE_FWD_ACTN0: c_uint = 0x9C14;
pub const R_BE_FWD_ACTN1: c_uint = 0x9C18;
pub const R_BE_FWD_ACTN2: c_uint = 0x9C1C;
pub const R_BE_FWD_TF0: c_uint = 0x9C20;
pub const R_BE_FWD_TF1: c_uint = 0x9C24;
pub const R_BE_HW_PPDU_STATUS: c_uint = 0x9C30;

pub const R_BE_CUT_AMSDU_CTRL: c_uint = 0x9C94;

pub const R_BE_WOW_CTRL: c_uint = 0x9CB8;

pub const R_BE_RX_HDRTRNS: c_uint = 0x9CC0;

pub const TRXCFG_MPDU_PROC_RX_HDR_CONV: c_uint = 0x00000000;
pub const R_BE_MPDU_RX_ERR_IMR: c_uint = 0x9CF4;

pub const B_BE_MPDU_RX_ERR_IMR_SET: c_int = 0;
pub const R_BE_SEC_ENG_CTRL: c_uint = 0x9D00;

pub const R_BE_SEC_MPDU_PROC: c_uint = 0x9D04;

pub const R_BE_SEC_CAM_ACCESS: c_uint = 0x9D10;

pub const R_BE_SEC_CAM_RDATA: c_uint = 0x9D14;

pub const R_BE_SEC_DEBUG2: c_uint = 0x9D28;

pub const R_BE_SEC_ERROR_IMR: c_uint = 0x9D2C;

pub const R_BE_SEC_ERROR_FLAG: c_uint = 0x9D30;

pub const R_BE_TXPKTCTL_MPDUINFO_CFG: c_uint = 0x9F10;

pub const MPDU_INFO_B1_OFST: c_int = 18;
pub const MPDU_INFO_TBL_FACTOR: c_int = 3;
pub const R_BE_TXPKTCTL_B0_PRELD_CFG0: c_uint = 0x9F48;

pub const PRELD_MISCQ_ENT_NUM_8922A: c_int = 2;
pub const PRELD_MISCQ_ENT_NUM_8922D: c_int = 1;

pub const PRELD_B0_ACQ_ENT_NUM_8922A: c_int = 8;
pub const PRELD_B1_ACQ_ENT_NUM_8922A: c_int = 2;
pub const PRELD_ACQ_ENT_NUM_8922D: c_int = 1;
pub const R_BE_TXPKTCTL_B0_PRELD_CFG1: c_uint = 0x9F4C;

pub const R_BE_TXPKTCTL_B0_ERRFLAG_IMR: c_uint = 0x9F78;

pub const R_BE_TXPKTCTL_B1_PRELD_CFG0: c_uint = 0x9F88;

pub const R_BE_TXPKTCTL_B1_PRELD_CFG1: c_uint = 0x9F8C;

pub const R_BE_TXPKTCTL_B1_ERRFLAG_IMR: c_uint = 0x9FB8;

pub const R_BE_MLO_INIT_CTL: c_uint = 0xA114;

pub const R_BE_MLO_ERR_IDCT_IMR: c_uint = 0xA128;

pub const R_BE_MLO_ERR_IDCT_ISR: c_uint = 0xA12C;

pub const R_BE_PLRLS_ERR_IMR: c_uint = 0xA218;

pub const R_BE_PLRLS_ERR_ISR: c_uint = 0xA21C;

pub const R_BE_SS_CTRL: c_uint = 0xA310;
pub const R_BE_SS_CTRL_V1: c_uint = 0xA610;

pub const R_BE_INTERRUPT_MASK_REG: c_uint = 0xA3F0;

pub const R_BE_INTERRUPT_STS_REG: c_uint = 0xA3F4;

pub const R_BE_PLRLS_ERR_IMR_V1: c_uint = 0xA518;

pub const B_BE_PLRLS_ERR_IMR_V1_CLR: c_uint = 0x1;
pub const B_BE_PLRLS_ERR_IMR_V1_SET: c_uint = 0x1;
pub const R_BE_SS_LITE_TXL_MACID: c_uint = 0xA790;

pub const R_BE_HAXI_INIT_CFG1: c_uint = 0xB000;

pub const S_BE_DMA_MOD_PCIE_NO_DATA_CPU: c_uint = 0x0;
pub const S_BE_DMA_MOD_PCIE_DATA_CPU: c_uint = 0x1;
pub const S_BE_DMA_MOD_USB: c_uint = 0x4;
pub const S_BE_DMA_MOD_SDIO: c_uint = 0x6;

pub const R_BE_HAXI_DMA_STOP1: c_uint = 0xB010;

pub const R_BE_HAXI_MST_WDT_TIMEOUT_SEL_V1: c_uint = 0xB02C;

pub const R_BE_HAXI_IDCT_MSK: c_uint = 0xB0B8;

pub const R_BE_HAXI_IDCT: c_uint = 0xB0BC;

pub const R_BE_HCI_FC_CTRL: c_uint = 0xB700;

pub const R_BE_CH_PAGE_CTRL: c_uint = 0xB704;

pub const R_BE_CH0_PAGE_CTRL: c_uint = 0xB718;

pub const R_BE_CH0_PAGE_INFO: c_uint = 0xB750;

pub const R_BE_PUB_PAGE_INFO3: c_uint = 0xB78C;

pub const R_BE_PUB_PAGE_CTRL1: c_uint = 0xB790;

pub const R_BE_PUB_PAGE_CTRL2: c_uint = 0xB794;

pub const R_BE_PUB_PAGE_INFO1: c_uint = 0xB79C;

pub const R_BE_PUB_PAGE_INFO2: c_uint = 0xB7A0;

pub const R_BE_WP_PAGE_CTRL1: c_uint = 0xB7A4;

pub const R_BE_WP_PAGE_CTRL2: c_uint = 0xB7A8;

pub const R_BE_WP_PAGE_INFO1: c_uint = 0xB7AC;

pub const R_BE_LTPC_T0_PATH0: c_uint = 0xBA28;
pub const R_BE_LTPC_T0_PATH1: c_uint = 0xBB28;
pub const R_BE_CMAC_SHARE_FUNC_EN: c_uint = 0x0E000;

pub const R_BE_CMAC_SHARE_ACQCHK_CFG_0: c_uint = 0x0E010;

pub const R_BE_BTC_CFG: c_uint = 0x0E300;
pub const R_BE_BT_BREAK_TABLE: c_uint = 0x0E344;
pub const R_BE_GNT_SW_CTRL: c_uint = 0x0E348;

pub const R_BE_PTA_GNT_SW_CTRL: c_uint = 0x0E348;

pub const R_BE_PTA_GNT_VAL: c_uint = 0x0E34C;

pub const R_BE_PTA_GNT_ZL_SW_CTRL: c_uint = 0x0E350;

pub const R_BE_PWR_MACID_PATH_BASE: c_uint = 0x0E500;
pub const R_BE_PWR_MACID_PATH_BASE_V1: c_uint = 0x1C000;
pub const R_BE_PWR_MACID_LMT_BASE: c_uint = 0x0ED00;
pub const R_BE_PWR_MACID_LMT_BASE_V1: c_uint = 0x1C800;
pub const R_BE_CMAC_FUNC_EN: c_uint = 0x10000;
pub const R_BE_CMAC_FUNC_EN_C1: c_uint = 0x14000;

pub const R_BE_CK_EN: c_uint = 0x10004;
pub const R_BE_CK_EN_C1: c_uint = 0x14004;

pub const R_BE_WMAC_RFMOD: c_uint = 0x10010;
pub const R_BE_WMAC_RFMOD_C1: c_uint = 0x14010;

pub const BE_WMAC_RFMOD_20M: c_int = 0;
pub const BE_WMAC_RFMOD_40M: c_int = 1;
pub const BE_WMAC_RFMOD_80M: c_int = 2;
pub const BE_WMAC_RFMOD_160M: c_int = 3;
pub const BE_WMAC_RFMOD_320M: c_int = 4;
pub const R_BE_GID_POSITION0: c_uint = 0x10070;
pub const R_BE_GID_POSITION0_C1: c_uint = 0x14070;
pub const R_BE_GID_POSITION1: c_uint = 0x10074;
pub const R_BE_GID_POSITION1_C1: c_uint = 0x14074;
pub const R_BE_GID_POSITION2: c_uint = 0x10078;
pub const R_BE_GID_POSITION2_C1: c_uint = 0x14078;
pub const R_BE_GID_POSITION3: c_uint = 0x1007C;
pub const R_BE_GID_POSITION3_C1: c_uint = 0x1407C;
pub const R_BE_GID_POSITION_EN0: c_uint = 0x10080;
pub const R_BE_GID_POSITION_EN0_C1: c_uint = 0x14080;
pub const R_BE_GID_POSITION_EN1: c_uint = 0x10084;
pub const R_BE_GID_POSITION_EN1_C1: c_uint = 0x14084;
pub const R_BE_TX_SUB_BAND_VALUE: c_uint = 0x10088;
pub const R_BE_TX_SUB_BAND_VALUE_C1: c_uint = 0x14088;

pub const BE_PRI20_BITMAP_MAX: c_int = 15;

pub const S_BE_TXSB_160M_0: c_int = 0;
pub const S_BE_TXSB_160M_1: c_int = 1;

pub const S_BE_TXSB_80M_0: c_int = 0;
pub const S_BE_TXSB_80M_2: c_int = 2;
pub const S_BE_TXSB_80M_4: c_int = 4;

pub const S_BE_TXSB_40M_0: c_int = 0;
pub const S_BE_TXSB_40M_1: c_int = 1;
pub const S_BE_TXSB_40M_4: c_int = 4;

pub const S_BE_TXSB_20M_8: c_int = 8;
pub const S_BE_TXSB_20M_4: c_int = 4;
pub const S_BE_TXSB_20M_2: c_int = 2;
pub const R_BE_PTCL_RRSR0: c_uint = 0x1008C;
pub const R_BE_PTCL_RRSR0_C1: c_uint = 0x1408C;

pub const R_BE_PTCL_RRSR1: c_uint = 0x10090;
pub const R_BE_PTCL_RRSR1_C1: c_uint = 0x14090;

pub const R_BE_COMMON_PHYINTF_CTRL_0: c_uint = 0x100B8;
pub const R_BE_COMMON_PHYINTF_CTRL_0_C1: c_uint = 0x140B8;

pub const R_BE_CMAC_ERR_IMR: c_uint = 0x10160;
pub const R_BE_CMAC_ERR_IMR_C1: c_uint = 0x14160;

pub const R_BE_CMAC_ERR_ISR: c_uint = 0x10164;
pub const R_BE_CMAC_ERR_ISR_C1: c_uint = 0x14164;

pub const R_BE_CMAC_FW_TRIGGER_IDCT_ISR: c_uint = 0x10168;
pub const R_BE_CMAC_FW_TRIGGER_IDCT_ISR_C1: c_uint = 0x14168;

pub const R_BE_SER_L0_DBG_CNT: c_uint = 0x10170;
pub const R_BE_SER_L0_DBG_CNT_C1: c_uint = 0x14170;

pub const R_BE_SER_L0_DBG_CNT1: c_uint = 0x10174;
pub const R_BE_SER_L0_DBG_CNT1_C1: c_uint = 0x14174;

pub const R_BE_SER_L0_DBG_CNT2: c_uint = 0x10178;
pub const R_BE_SER_L0_DBG_CNT2_C1: c_uint = 0x14178;
pub const R_BE_SER_L0_DBG_CNT3: c_uint = 0x1017C;
pub const R_BE_SER_L0_DBG_CNT3_C1: c_uint = 0x1417C;

pub const R_BE_PORT_0_TSF_SYNC: c_uint = 0x102A0;
pub const R_BE_PORT_0_TSF_SYNC_C1: c_uint = 0x142A0;

pub const R_BE_SCH_EDCA_RST_CFG: c_uint = 0x102E4;
pub const R_BE_SCH_EDCA_RST_CFG_C1: c_uint = 0x142E4;

pub const R_BE_EDCA_BCNQ_PARAM: c_uint = 0x10324;
pub const R_BE_EDCA_BCNQ_PARAM_C1: c_uint = 0x14324;

pub const BCN_IFS_25US: c_uint = 0x19;

pub const R_BE_PREBKF_CFG_0: c_uint = 0x10338;
pub const R_BE_PREBKF_CFG_0_C1: c_uint = 0x14338;

pub const R_BE_PREBKF_CFG_1: c_uint = 0x1033C;
pub const R_BE_PREBKF_CFG_1_C1: c_uint = 0x1433C;

pub const R_BE_CCA_CFG_0: c_uint = 0x10340;
pub const R_BE_CCA_CFG_0_C1: c_uint = 0x14340;

pub const R_BE_CTN_CFG_0: c_uint = 0x1034C;
pub const R_BE_CTN_CFG_0_C1: c_uint = 0x1434C;

pub const R_BE_MUEDCA_BE_PARAM_0: c_uint = 0x10350;
pub const R_BE_MUEDCA_BK_PARAM_0: c_uint = 0x10354;
pub const R_BE_MUEDCA_VI_PARAM_0: c_uint = 0x10358;
pub const R_BE_MUEDCA_VO_PARAM_0: c_uint = 0x1035C;
pub const R_BE_MUEDCA_EN: c_uint = 0x10370;
pub const R_BE_MUEDCA_EN_C1: c_uint = 0x14370;

pub const R_BE_MISC_1: c_uint = 0x1037C;
pub const R_BE_MISC_1_C1: c_uint = 0x1437C;

pub const R_BE_CTN_DRV_TXEN: c_uint = 0x10398;
pub const R_BE_CTN_DRV_TXEN_C1: c_uint = 0x14398;

pub const R_BE_TB_CHK_CCA_NAV: c_uint = 0x103AC;
pub const R_BE_TB_CHK_CCA_NAV_C1: c_uint = 0x143AC;

pub const R_BE_HE_SIFS_CHK_CCA_NAV: c_uint = 0x103B4;
pub const R_BE_HE_SIFS_CHK_CCA_NAV_C1: c_uint = 0x143B4;

pub const R_BE_HE_CTN_CHK_CCA_NAV: c_uint = 0x103C4;
pub const R_BE_HE_CTN_CHK_CCA_NAV_C1: c_uint = 0x143C4;

pub const R_BE_SCHEDULE_ERR_IMR: c_uint = 0x103E8;
pub const R_BE_SCHEDULE_ERR_IMR_C1: c_uint = 0x143E8;

pub const R_BE_SCHEDULE_ERR_ISR: c_uint = 0x103EC;
pub const R_BE_SCHEDULE_ERR_ISR_C1: c_uint = 0x143EC;

pub const R_BE_PORT_CFG_P0: c_uint = 0x10400;
pub const R_BE_PORT_CFG_P0_C1: c_uint = 0x14400;

pub const R_BE_TBTT_PROHIB_P0: c_uint = 0x10404;
pub const R_BE_TBTT_PROHIB_P0_C1: c_uint = 0x14404;

pub const R_BE_BCN_AREA_P0: c_uint = 0x10408;
pub const R_BE_BCN_AREA_P0_C1: c_uint = 0x14408;
pub const B_BE_BCN_MSK_AREA_P0_MSK: c_uint = 0xfff;

pub const R_BE_BCNERLYINT_CFG_P0: c_uint = 0x1040C;
pub const R_BE_BCNERLYINT_CFG_P0_C1: c_uint = 0x1440C;

pub const R_BE_TBTTERLYINT_CFG_P0: c_uint = 0x1040E;
pub const R_BE_TBTTERLYINT_CFG_P0_C1: c_uint = 0x1440E;

pub const R_BE_TBTT_AGG_P0: c_uint = 0x10412;
pub const R_BE_TBTT_AGG_P0_C1: c_uint = 0x14412;

pub const R_BE_BCN_SPACE_CFG_P0: c_uint = 0x10414;
pub const R_BE_BCN_SPACE_CFG_P0_C1: c_uint = 0x14414;

pub const R_BE_BCN_FORCETX_P0: c_uint = 0x10418;
pub const R_BE_BCN_FORCETX_P0_C1: c_uint = 0x14418;

pub const R_BE_BCN_ERR_CNT_P0: c_uint = 0x10420;
pub const R_BE_BCN_ERR_CNT_P0_C1: c_uint = 0x14420;

pub const R_BE_BCN_ERR_FLAG_P0: c_uint = 0x10424;
pub const R_BE_BCN_ERR_FLAG_P0_C1: c_uint = 0x14424;

pub const R_BE_DTIM_CTRL_P0: c_uint = 0x10426;
pub const R_BE_DTIM_CTRL_P0_C1: c_uint = 0x14426;

pub const R_BE_TBTT_SHIFT_P0: c_uint = 0x10428;
pub const R_BE_TBTT_SHIFT_P0_C1: c_uint = 0x14428;
pub const B_BE_TBTT_SHIFT_OFST_P0_SH: c_int = 0;
pub const B_BE_TBTT_SHIFT_OFST_P0_MSK: c_uint = 0xfff;
pub const R_BE_BCN_CNT_TMR_P0: c_uint = 0x10434;
pub const R_BE_BCN_CNT_TMR_P0_C1: c_uint = 0x14434;

pub const R_BE_TSFTR_LOW_P0: c_uint = 0x10438;
pub const R_BE_TSFTR_LOW_P0_C1: c_uint = 0x14438;

pub const R_BE_TSFTR_HIGH_P0: c_uint = 0x1043C;
pub const R_BE_TSFTR_HIGH_P0_C1: c_uint = 0x1443C;

pub const R_BE_BCN_DROP_ALL0: c_uint = 0x10560;
pub const R_BE_MBSSID_CTRL: c_uint = 0x10568;
pub const R_BE_MBSSID_CTRL_C1: c_uint = 0x14568;

pub const R_BE_P0MB_HGQ_WINDOW_CFG_0: c_uint = 0x10590;
pub const R_BE_P0MB_HGQ_WINDOW_CFG_0_C1: c_uint = 0x14590;
pub const R_BE_PORT_HGQ_WINDOW_CFG: c_uint = 0x105A0;
pub const R_BE_PORT_HGQ_WINDOW_CFG_C1: c_uint = 0x145A0;
pub const R_BE_PTCL_COMMON_SETTING_0: c_uint = 0x10800;
pub const R_BE_PTCL_COMMON_SETTING_0_C1: c_uint = 0x14800;

pub const R_BE_AGG_BK_0: c_uint = 0x10804;
pub const R_BE_AGG_BK_0_C1: c_uint = 0x14804;

pub const R_BE_TB_PPDU_CTRL: c_uint = 0x1080C;
pub const R_BE_TB_PPDU_CTRL_C1: c_uint = 0x1480C;

pub const R_BE_AMPDU_AGG_LIMIT: c_uint = 0x10810;
pub const R_BE_AMPDU_AGG_LIMIT_C1: c_uint = 0x14810;

pub const AMPDU_MAX_TIME: c_uint = 0x9E;
pub const AMPDU_MAX_TIME_V1: c_uint = 0xA4;

pub const MAX_TX_AMPDU_NUM_V1: c_int = 128;
pub const R_BE_AGG_LEN_HT_0: c_uint = 0x10814;
pub const R_BE_AGG_LEN_HT_0_C1: c_uint = 0x14814;

pub const R_BE_SPECIAL_TX_SETTING: c_uint = 0x10820;
pub const R_BE_SPECIAL_TX_SETTING_C1: c_uint = 0x14820;

pub const R_BE_SIFS_SETTING: c_uint = 0x10824;
pub const R_BE_SIFS_SETTING_C1: c_uint = 0x14824;

pub const R_BE_TXRATE_CHK: c_uint = 0x10828;
pub const R_BE_TXRATE_CHK_C1: c_uint = 0x14828;

pub const R_BE_TXCNT: c_uint = 0x1082C;
pub const R_BE_TXCNT_C1: c_uint = 0x1482C;

pub const R_BE_MBSSID_DROP_0: c_uint = 0x1083C;
pub const R_BE_MBSSID_DROP_0_C1: c_uint = 0x1483C;

pub const R_BE_PTCL_PRELD_CTRL: c_uint = 0x10868;
pub const R_BE_PTCL_PRELD_CTRL_C1: c_uint = 0x14868;

pub const R_BE_BT_PLT: c_uint = 0x1087C;
pub const R_BE_BT_PLT_C1: c_uint = 0x1487C;

pub const R_BE_PTCL_BSS_COLOR_0: c_uint = 0x108A0;
pub const R_BE_PTCL_BSS_COLOR_0_C1: c_uint = 0x148A0;

pub const R_BE_PTCL_BSS_COLOR_1: c_uint = 0x108A4;
pub const R_BE_PTCL_BSS_COLOR_1_C1: c_uint = 0x148A4;

pub const R_BE_PTCL_IMR_2: c_uint = 0x108B8;
pub const R_BE_PTCL_IMR_2_C1: c_uint = 0x148B8;

pub const B_BE_PTCL_IMR_2_SET: c_int = 0;
pub const R_BE_PTCL_IMR0: c_uint = 0x108C0;
pub const R_BE_PTCL_IMR0_C1: c_uint = 0x148C0;

pub const R_BE_PTCL_ISR0: c_uint = 0x108C4;
pub const R_BE_PTCL_ISR0_C1: c_uint = 0x148C4;

pub const R_BE_PTCL_IMR1: c_uint = 0x108C8;
pub const R_BE_PTCL_IMR1_C1: c_uint = 0x148C8;

pub const R_BE_PTCL_ISR1: c_uint = 0x108CC;
pub const R_BE_PTCL_ISR1_C1: c_uint = 0x148CC;

pub const R_BE_PTCL_FSM_MON: c_uint = 0x108E8;
pub const R_BE_PTCL_FSM_MON_C1: c_uint = 0x148E8;

pub const R_BE_PTCL_TX_CTN_SEL: c_uint = 0x108EC;
pub const R_BE_PTCL_TX_CTN_SEL_C1: c_uint = 0x148EC;

pub const R_BE_PTCL_DBG_INFO: c_uint = 0x108F0;
pub const R_BE_PTCL_DBG: c_uint = 0x108F4;
pub const R_BE_RX_ERROR_FLAG: c_uint = 0x10C00;
pub const R_BE_RX_ERROR_FLAG_C1: c_uint = 0x14C00;

pub const R_BE_RX_ERROR_FLAG_IMR: c_uint = 0x10C04;
pub const R_BE_RX_ERROR_FLAG_IMR_C1: c_uint = 0x14C04;

pub const B_BE_RX_ERROR_FLAG_IMR_CLR_V1: c_uint = 0x7FFFFFF8;
pub const B_BE_RX_ERROR_FLAG_IMR_SET_V1: c_uint = 0x7FFFFF38;
pub const R_BE_RX_CTRL_1: c_uint = 0x10C0C;
pub const R_BE_RX_CTRL_1_C1: c_uint = 0x14C0C;

pub const WLCPU_RXCH2_QID: c_uint = 0xA;
pub const R_BE_TX_ERROR_FLAG: c_uint = 0x10C6C;
pub const R_BE_TX_ERROR_FLAG_C1: c_uint = 0x14C6C;

pub const R_BE_TX_ERROR_FLAG_IMR: c_uint = 0x10C70;
pub const R_BE_TX_ERROR_FLAG_IMR_C1: c_uint = 0x14C70;

pub const R_BE_RX_ERROR_FLAG_1: c_uint = 0x10C84;
pub const R_BE_RX_ERROR_FLAG_1_C1: c_uint = 0x14C84;

pub const R_BE_RX_ERROR_FLAG_IMR_1: c_uint = 0x10C88;
pub const R_BE_RX_ERROR_FLAG_IMR_1_C1: c_uint = 0x14C88;

pub const R_BE_WMTX_MOREDATA_TSFT_STMP_CTL: c_uint = 0x10E08;
pub const R_BE_WMTX_MOREDATA_TSFT_STMP_CTL_C1: c_uint = 0x14E08;

pub const R_BE_WMTX_POWER_BE_BIT_CTL: c_uint = 0x10E0C;
pub const R_BE_WMTX_POWER_BE_BIT_CTL_C1: c_uint = 0x14E0C;
pub const R_BE_WMTX_TCR_BE_4: c_uint = 0x10E2C;
pub const R_BE_WMTX_TCR_BE_4_C1: c_uint = 0x14E2C;

pub const R_BE_RSP_CHK_SIG: c_uint = 0x11000;
pub const R_BE_RSP_CHK_SIG_C1: c_uint = 0x15000;

pub const R_BE_TRXPTCL_RESP_0: c_uint = 0x11004;
pub const R_BE_TRXPTCL_RESP_0_C1: c_uint = 0x15004;

pub const WMAC_SPEC_SIFS_OFDM_1115E: c_uint = 0x11;

pub const R_BE_TRXPTCL_RESP_1: c_uint = 0x11008;
pub const R_BE_TRXPTCL_RESP_1_C1: c_uint = 0x15008;

pub const R_BE_MAC_LOOPBACK: c_uint = 0x11020;
pub const R_BE_MAC_LOOPBACK_C1: c_uint = 0x15020;

pub const S_BE_MACLBK_PLCP_DLY_DEF: c_uint = 0x28;

pub const R_BE_CLIENT_OM_CTRL: c_uint = 0x11040;
pub const R_BE_CLIENT_OM_CTRL_C1: c_uint = 0x15040;

pub const R_BE_WMAC_NAV_CTL: c_uint = 0x11080;
pub const R_BE_WMAC_NAV_CTL_C1: c_uint = 0x15080;

pub const NAV_25MS: c_uint = 0xC4;

pub const R_BE_RXTRIG_TEST_USER_2: c_uint = 0x110B0;
pub const R_BE_RXTRIG_TEST_USER_2_C1: c_uint = 0x150B0;

pub const R_BE_TRXPTCL_ERROR_INDICA_MASK: c_uint = 0x110BC;
pub const R_BE_TRXPTCL_ERROR_INDICA_MASK_C1: c_uint = 0x150BC;

pub const R_BE_TRXPTCL_ERROR_INDICA: c_uint = 0x110C0;
pub const R_BE_TRXPTCL_ERROR_INDICA_C1: c_uint = 0x150C0;

pub const R_BE_DBGSEL_TRXPTCL: c_uint = 0x110F4;
pub const R_BE_DBGSEL_TRXPTCL_C1: c_uint = 0x150F4;

pub const R_BE_PHYINFO_ERR_IMR_V1: c_uint = 0x110F8;
pub const R_BE_PHYINFO_ERR_IMR_V1_C1: c_uint = 0x150F8;

pub const B_BE_PHYINFO_ERR_IMR_V1_SET: c_int = 0;
pub const R_BE_PHYINFO_ERR_ISR: c_uint = 0x110FC;
pub const R_BE_PHYINFO_ERR_ISR_C1: c_uint = 0x150FC;

pub const R_BE_BFMEE_RESP_OPTION: c_uint = 0x11180;
pub const R_BE_BFMEE_RESP_OPTION_C1: c_uint = 0x15180;
pub const B_BE_BFMEE_CSI_SEC_TYPE_SH: c_int = 20;
pub const B_BE_BFMEE_CSI_SEC_TYPE_MSK: c_uint = 0xf;
pub const B_BE_BFMEE_BFRPT_SEG_SIZE_SH: c_int = 16;
pub const B_BE_BFMEE_BFRPT_SEG_SIZE_MSK: c_uint = 0x3;

pub const R_BE_TRXPTCL_RESP_CSI_CTRL_0: c_uint = 0x11188;
pub const R_BE_TRXPTCL_RESP_CSI_CTRL_0_C1: c_uint = 0x15188;

pub const CSI_RX_BW_CFG: c_uint = 0x1;
pub const R_BE_TRXPTCL_RESP_CSI_CTRL_1: c_uint = 0x11194;
pub const R_BE_TRXPTCL_RESP_CSI_CTRL_1_C1: c_uint = 0x15194;

pub const CSI_RRSC_BITMAP_CFG: c_uint = 0x2A;
pub const R_BE_TRXPTCL_RESP_CSI_RRSC: c_uint = 0x1118C;
pub const R_BE_TRXPTCL_RESP_CSI_RRSC_C1: c_uint = 0x1518C;
pub const CSI_RRSC_BMAP_BE: c_uint = 0x2A2AFF;
pub const R_BE_TRXPTCL_RESP_CSI_RATE: c_uint = 0x11190;
pub const R_BE_TRXPTCL_RESP_CSI_RATE_C1: c_uint = 0x15190;

pub const CSI_INIT_RATE_EHT: c_uint = 0x3;
pub const R_BE_WMAC_ACK_BA_RESP_LEGACY: c_uint = 0x11200;
pub const R_BE_WMAC_ACK_BA_RESP_LEGACY_C1: c_uint = 0x15200;

pub const R_BE_WMAC_ACK_BA_RESP_HE: c_uint = 0x11204;
pub const R_BE_WMAC_ACK_BA_RESP_HE_C1: c_uint = 0x15204;

pub const R_BE_WMAC_ACK_BA_RESP_EHT_LEG_PUNC: c_uint = 0x11208;
pub const R_BE_WMAC_ACK_BA_RESP_EHT_LEG_PUNC_C1: c_uint = 0x15208;

pub const R_BE_WMAC_RX_RTS_RESP_LEGACY: c_uint = 0x1120C;
pub const R_BE_WMAC_RX_RTS_RESP_LEGACY_C1: c_uint = 0x1520C;

pub const R_BE_WMAC_RX_RTS_RESP_LEGACY_PUNC: c_uint = 0x11210;
pub const R_BE_WMAC_RX_RTS_RESP_LEGACY_PUNC_C1: c_uint = 0x15210;

pub const R_BE_WMAC_RX_MURTS_RESP_LEGACY: c_uint = 0x11214;
pub const R_BE_WMAC_RX_MURTS_RESP_LEGACY_C1: c_uint = 0x15214;

pub const R_BE_WMAC_RX_MURTS_RESP_LEGACY_PUNC: c_uint = 0x11218;
pub const R_BE_WMAC_RX_MURTS_RESP_LEGACY_PUNC_C1: c_uint = 0x15218;

pub const R_BE_WMAC_OTHERS_RESP_LEGACY: c_uint = 0x1121C;
pub const R_BE_WMAC_OTHERS_RESP_LEGACY_C1: c_uint = 0x1521C;

pub const R_BE_WMAC_OTHERS_RESP_HE: c_uint = 0x11220;
pub const R_BE_WMAC_OTHERS_RESP_HE_C1: c_uint = 0x15220;

pub const R_BE_WMAC_OTHERS_RESP_EHT_LEG_PUNC: c_uint = 0x11224;
pub const R_BE_WMAC_OTHERS_RESP_EHT_LEG_PUNC_C1: c_uint = 0x15224;

pub const R_BE_RCR: c_uint = 0x11400;
pub const R_BE_RCR_C1: c_uint = 0x15400;

pub const R_BE_DLK_PROTECT_CTL: c_uint = 0x11402;
pub const R_BE_DLK_PROTECT_CTL_C1: c_uint = 0x15402;

pub const TRXCFG_RMAC_CCA_TO: c_int = 32;

pub const TRXCFG_RMAC_DATA_TO: c_int = 15;

pub const R_BE_PLCP_HDR_FLTR: c_uint = 0x11404;
pub const R_BE_PLCP_HDR_FLTR_C1: c_uint = 0x15404;

pub const R_BE_RXGCK_CTRL: c_uint = 0x11406;
pub const R_BE_RXGCK_CTRL_C1: c_uint = 0x15406;

pub const RX_GCK_LEGACY: c_int = 2;

pub const R_BE_RX_FLTR_OPT: c_uint = 0x11420;
pub const R_BE_RX_FLTR_OPT_C1: c_uint = 0x15420;

pub const R_BE_CTRL_FLTR: c_uint = 0x11424;
pub const R_BE_CTRL_FLTR_C1: c_uint = 0x15424;

pub const RX_FLTR_FRAME_DROP_BE: c_uint = 0x0000;
pub const RX_FLTR_FRAME_ACCEPT_BE: c_uint = 0xFFFF;
pub const R_BE_MGNT_FLTR: c_uint = 0x11428;
pub const R_BE_MGNT_FLTR_C1: c_uint = 0x15428;

pub const R_BE_DATA_FLTR: c_uint = 0x1142C;
pub const R_BE_DATA_FLTR_C1: c_uint = 0x1542C;

pub const R_BE_ADDR_CAM_CTRL: c_uint = 0x11434;
pub const R_BE_ADDR_CAM_CTRL_C1: c_uint = 0x15434;

pub const ADDR_CAM_SERCH_RANGE: c_uint = 0x7f;

pub const R_BE_RESPBA_CAM_CTRL: c_uint = 0x1143C;
pub const R_BE_RESPBA_CAM_CTRL_C1: c_uint = 0x1543C;

pub const S_BE_BACAM_RST_DONE: c_int = 0;
pub const S_BE_BACAM_RST_ENT: c_int = 1;
pub const S_BE_BACAM_RST_ALL: c_int = 2;
pub const R_BE_PPDU_STAT: c_uint = 0x11440;
pub const R_BE_PPDU_STAT_C1: c_uint = 0x15440;

pub const R_BE_RX_SR_CTRL: c_uint = 0x1144A;
pub const R_BE_RX_SR_CTRL_C1: c_uint = 0x1544A;

pub const R_BE_BSSID_SRC_CTRL: c_uint = 0x1144B;
pub const R_BE_BSSID_SRC_CTRL_C1: c_uint = 0x1544B;

pub const R_BE_CSIRPT_OPTION: c_uint = 0x11464;
pub const R_BE_CSIRPT_OPTION_C1: c_uint = 0x15464;

pub const R_BE_BSR_UPD_CTRL: c_uint = 0x11468;
pub const R_BE_BSR_UPD_CTRL_C1: c_uint = 0x15468;

pub const R_BE_DRV_INFO_OPTION: c_uint = 0x11470;
pub const R_BE_DRV_INFO_OPTION_C1: c_uint = 0x15470;

pub const R_BE_BCN_PSR_RPT_P0: c_uint = 0x11484;
pub const R_BE_BCN_PSR_RPT_P0_C1: c_uint = 0x15484;

pub const R_BE_RX_ERR_ISR: c_uint = 0x114F4;
pub const R_BE_RX_ERR_ISR_C1: c_uint = 0x154F4;

pub const R_BE_RX_ERR_IMR: c_uint = 0x114F8;
pub const R_BE_RX_ERR_IMR_C1: c_uint = 0x154F8;

pub const R_BE_RX_PLCP_EXT_OPTION_1: c_uint = 0x11514;
pub const R_BE_RX_PLCP_EXT_OPTION_1_C1: c_uint = 0x15514;

pub const R_BE_RX_PLCP_EXT_OPTION_2: c_uint = 0x11518;
pub const R_BE_RX_PLCP_EXT_OPTION_2_C1: c_uint = 0x15518;

pub const R_BE_RESP_CSI_RESERVED_PAGE: c_uint = 0x11810;
pub const R_BE_RESP_CSI_RESERVED_PAGE_C1: c_uint = 0x15810;

pub const R_BE_RESP_IMR1: c_uint = 0x11878;
pub const R_BE_RESP_IMR1_C1: c_uint = 0x15878;

pub const B_BE_RESP_IMR1_CLR: c_uint = 0x1FF;
pub const B_BE_RESP_IMR1_SET: c_uint = 0xFF;
pub const R_BE_RESP_IMR: c_uint = 0x11884;
pub const R_BE_RESP_IMR_C1: c_uint = 0x15884;

pub const B_BE_RESP_IMR_CLR_V1: c_uint = 0xFFFFFFFF;
pub const B_BE_RESP_IMR_SET_V1: c_uint = 0xFFFFFFFF;
pub const R_BE_PWR_MODULE: c_uint = 0x11900;
pub const R_BE_PWR_MODULE_C1: c_uint = 0x15900;
pub const R_BE_PWR_LISTEN_PATH: c_uint = 0x11988;

pub const R_BE_TXAGC_MAX_1TX_RU484_242_0: c_uint = 0x11990;
pub const R_BE_TXAGC_MAX_1TX_RU996_484_0: c_uint = 0x119A4;
pub const R_BE_TXAGC_MAX_1TX_RU996_484_242_0: c_uint = 0x119AC;
pub const R_BE_TXAGC_MAX_1TX_BF_RU484_242_0: c_uint = 0x119DC;
pub const R_BE_PWR_REF_CTRL: c_uint = 0x11A20;

pub const R_BE_PWR_OFST_LMTBF: c_uint = 0x11A24;

pub const R_BE_PWR_FORCE_LMT: c_uint = 0x11A28;

pub const R_BE_PWR_RATE_CTRL: c_uint = 0x11A2C;

pub const R_BE_PWR_RATE_OFST_CTRL: c_uint = 0x11A30;
pub const R_BE_PWR_RATE_OFST_END: c_uint = 0x11A38;
pub const R_BE_PWR_RULMT_START: c_uint = 0x12048;
pub const R_BE_PWR_RULMT_END: c_uint = 0x120e4;
pub const R_BE_PWR_BOOST: c_uint = 0x11A40;

pub const R_BE_PWR_OFST_RULMT: c_uint = 0x11A44;

pub const R_BE_PWR_FORCE_MACID: c_uint = 0x11A48;

pub const R_BE_TXAGC_MAX_1TX_BF_RU996_484_0: c_uint = 0x11A4C;
pub const R_BE_PWR_REG_CTRL: c_uint = 0x11A50;

pub const R_BE_PWR_COEX_CTRL: c_uint = 0x11A54;

pub const R_PWR_BOOST_BE4: c_uint = 0x11A64;

pub const R_BE_PWR_TH: c_uint = 0x11A78;
pub const R_BE_PWR_RSSI_TARGET_LMT: c_uint = 0x11A84;
pub const R_BE_TXAGC_MAX_1TX_BF_RU996_484_242_0: c_uint = 0x11ADC;
pub const R_BE_PWR_OFST_SW: c_uint = 0x11AE8;

pub const R_BE_PWR_FTM: c_uint = 0x11B00;
pub const R_BE_PWR_FTM_SS: c_uint = 0x11B04;

pub const R_BE_PWR_BY_RATE: c_uint = 0x11E00;
pub const R_BE_PWR_BY_RATE_MAX: c_uint = 0x11FA8;
pub const R_BE_PWR_LMT: c_uint = 0x11FAC;
pub const R_BE_PWR_LMT_MAX: c_uint = 0x12040;
pub const R_BE_PWR_BY_RATE_END: c_uint = 0x12044;
pub const R_BE_PWR_RU_LMT: c_uint = 0x12048;
pub const R_BE_PWR_RU_LMT_MAX: c_uint = 0x120E4;
pub const R_BE_C0_TXPWR_IMR: c_uint = 0x128E0;
pub const R_BE_C0_TXPWR_IMR_C1: c_uint = 0x168E0;

pub const R_BE_TXPWR_ERR_FLAG: c_uint = 0x128E4;
pub const R_BE_TXPWR_ERR_IMR: c_uint = 0x128E0;
pub const R_BE_TXPWR_ERR_FLAG_C1: c_uint = 0x158E4;
pub const R_BE_TXPWR_ERR_IMR_C1: c_uint = 0x158E0;
pub const R_BE_SCH_EXT_CTRL: c_uint = 0x103FC;
pub const R_BE_SCH_EXT_CTRL_C1: c_uint = 0x143FC;

pub const CMAC1_START_ADDR_BE: c_uint = 0x14000;
pub const CMAC1_END_ADDR_BE: c_uint = 0x17FFF;
pub const RR_MOD: c_uint = 0x00;
pub const RR_MOD_V1: c_uint = 0x10000;

pub const RR_MOD_V_DOWN: c_uint = 0x0;
pub const RR_MOD_V_STANDBY: c_uint = 0x1;
pub const RR_TXAGC: c_uint = 0x10001;
pub const RR_MOD_V_TX: c_uint = 0x2;
pub const RR_MOD_V_RX: c_uint = 0x3;
pub const RR_MOD_V_TXIQK: c_uint = 0x4;
pub const RR_MOD_V_DPK: c_uint = 0x5;
pub const RR_MOD_V_RXK1: c_uint = 0x6;
pub const RR_MOD_V_RXK2: c_uint = 0x7;

pub const RR_MODOPT: c_uint = 0x01;
pub const RR_MODOPT_V1: c_uint = 0x10001;

pub const RR_WLSEL: c_uint = 0x02;

pub const RR_RSV1: c_uint = 0x05;

pub const RR_BBDC: c_uint = 0x10005;

pub const RR_DTXLOK: c_uint = 0x08;
pub const RR_RSV2: c_uint = 0x09;
pub const RR_LOKVB: c_uint = 0x0a;

pub const RR_TXIG: c_uint = 0x11;

pub const RR_CHTR: c_uint = 0x17;

pub const RR_CFGCH: c_uint = 0x18;
pub const RR_CFGCH_V1: c_uint = 0x10018;

pub const CFGCH_BAND1_2G: c_int = 0;
pub const CFGCH_BAND1_5G: c_int = 1;
pub const CFGCH_BAND1_6G: c_int = 3;

pub const CFGCH_BAND0_2G: c_int = 0;
pub const CFGCH_BAND0_5G: c_int = 1;
pub const CFGCH_BAND0_6G: c_int = 0;

pub const CFGCH_BW_V2_20M: c_int = 0;
pub const CFGCH_BW_V2_40M: c_int = 1;
pub const CFGCH_BW_V2_80M: c_int = 2;
pub const CFGCH_BW_V2_160M: c_int = 3;
pub const CFGCH_BW_V2_320M: c_int = 4;

pub const CFGCH_BW_20M: c_int = 3;
pub const CFGCH_BW_40M: c_int = 2;
pub const CFGCH_BW_80M: c_int = 1;
pub const CFGCH_BW_160M: c_int = 0;
pub const RR_APK: c_uint = 0x19;

pub const RR_BTC: c_uint = 0x1a;

pub const RR_RCKC: c_uint = 0x1b;

pub const RR_RCKS: c_uint = 0x1c;
pub const RR_RCKO: c_uint = 0x1d;

pub const RR_RXKPLL: c_uint = 0x1e;

pub const RR_RSV4: c_uint = 0x1f;

pub const RR_RXK: c_uint = 0x20;

pub const RR_LUTWA: c_uint = 0x33;

pub const RR_LUTWD1: c_uint = 0x3e;
pub const RR_LUTWD0: c_uint = 0x3f;

pub const RR_TM: c_uint = 0x42;

pub const RR_TM2: c_uint = 0x43;

pub const RR_TXG1: c_uint = 0x51;

pub const RR_TXG2: c_uint = 0x52;

pub const RR_BSPAD: c_uint = 0x54;
pub const RR_TXGA: c_uint = 0x55;

pub const RR_TXGA_V1: c_uint = 0x10055;

pub const RR_GAINTX: c_uint = 0x56;

pub const RR_TXMO: c_uint = 0x58;

pub const RR_TXA: c_uint = 0x5d;

pub const RR_TXRSV: c_uint = 0x5c;

pub const RR_BIAS: c_uint = 0x5e;

pub const RR_TXAC: c_uint = 0x5f;

pub const RR_BIASA: c_uint = 0x60;

pub const RR_BIASA2: c_uint = 0x63;

pub const RR_TXATANK: c_uint = 0x64;

pub const RR_TXA2: c_uint = 0x65;

pub const RR_TRXIQ: c_uint = 0x66;
pub const RR_RSV6: c_uint = 0x6d;
pub const RR_TXVBUF: c_uint = 0x7c;

pub const RR_TXPOW: c_uint = 0x7f;

pub const RR_RXPOW: c_uint = 0x80;

pub const RR_RXBB: c_uint = 0x83;

pub const RR_RXG: c_uint = 0x84;

pub const RR_XGLNA2: c_uint = 0x85;

pub const RR_RXAE: c_uint = 0x89;

pub const RR_RXA: c_uint = 0x8a;

pub const RR_RXA_LNA: c_uint = 0x8b;
pub const RR_RXA2: c_uint = 0x8c;

pub const RR_RXIQGEN: c_uint = 0x8d;

pub const RR_RXBB2: c_uint = 0x8f;

pub const RR_XALNA2: c_uint = 0x90;

pub const RR_DCK: c_uint = 0x92;

pub const RR_DCK1: c_uint = 0x93;

pub const RR_DCK2: c_uint = 0x94;

pub const RR_DCKC: c_uint = 0x95;

pub const RR_IQGEN: c_uint = 0x97;

pub const RR_TXIQK: c_uint = 0x98;

pub const RR_TIA: c_uint = 0x9e;

pub const RR_MIXER: c_uint = 0x9f;

pub const RR_POW: c_uint = 0xa0;

pub const RR_LOGEN: c_uint = 0xa3;

pub const RR_SX: c_uint = 0xaf;
pub const RR_IBD: c_uint = 0xc9;

pub const RR_LDO: c_uint = 0xb1;

pub const RR_VCO: c_uint = 0xb2;

pub const RR_VCI: c_uint = 0xb3;

pub const RR_LPF: c_uint = 0xb7;

pub const RR_XTALX2: c_uint = 0xb8;
pub const RR_MALSEL: c_uint = 0xbe;
pub const RR_SYNFB: c_uint = 0xc5;

pub const RR_AACK: c_uint = 0xca;
pub const RR_LCKST: c_uint = 0xcf;

pub const RR_LCK_TRG: c_uint = 0xd3;

pub const RR_MMD: c_uint = 0xd5;

pub const RR_SMD: c_uint = 0xd6;

pub const RR_IQKPLL: c_uint = 0xdc;

pub const RR_SYNLUT: c_uint = 0xdd;

pub const RR_RCKD: c_uint = 0xde;

pub const RR_TXADBG: c_uint = 0xde;
pub const RR_LUTDBG: c_uint = 0xdf;

pub const RR_LUTPLL: c_uint = 0xec;

pub const RR_LUTWE2: c_uint = 0xee;

pub const RR_LUTWE: c_uint = 0xef;

pub const RR_RFC: c_uint = 0xf0;

pub const R_UPD_P0: c_uint = 0x0000;
pub const R_BBCLK: c_uint = 0x0000;

pub const R_RSTB_WATCH_DOG: c_uint = 0x000C;

pub const R_EMLSR: c_uint = 0x0044;

pub const R_CHK_LPS_STAT_BE4: c_uint = 0x3007C;
pub const R_CHK_LPS_STAT: c_uint = 0x0058;

pub const R_SPOOF_CG: c_uint = 0x00B4;

pub const R_CHINFO_SEG: c_uint = 0x00B4;

pub const R_DFS_FFT_CG: c_uint = 0x00B8;

pub const R_CHINFO_DATA: c_uint = 0x00C0;

pub const R_INTF_R_INTF_RPT_SEL: c_uint = 0x0200;
pub const R_INTF_R_INTF_RPT_SEL_BE4: c_uint = 0x20200;

pub const R_ANAPAR_PW15: c_uint = 0x030C;

pub const R_ANAPAR: c_uint = 0x032C;

pub const R_RFE_E_A2: c_uint = 0x0334;
pub const R_RFE_O_SEL_A2: c_uint = 0x0338;
pub const R_RFE_SEL0_A2: c_uint = 0x033C;

pub const R_RFE_SEL32_A2: c_uint = 0x0340;
pub const R_CIRST: c_uint = 0x035c;

pub const R_SWSI_DATA_V1: c_uint = 0x0370;

pub const R_SWSI_BIT_MASK_V1: c_uint = 0x0374;

pub const R_SWSI_READ_ADDR_V1: c_uint = 0x0378;

pub const R_BRK_R: c_uint = 0x0418;

pub const R_BRK_EHT: c_uint = 0x0474;

pub const R_BRK_RXEHT: c_uint = 0x0478;

pub const R_EN_SND_WO_NDP: c_uint = 0x047c;
pub const R_EN_SND_WO_NDP_C1: c_uint = 0x147c;

pub const R_BRK_HE: c_uint = 0x0480;

pub const R_RXCCA_BE1: c_uint = 0x0520;

pub const R_UPD_CLK_ADC: c_uint = 0x0700;

pub const R_RSTB_ASYNC: c_uint = 0x0704;

pub const R_P0_ANT_SW: c_uint = 0x0728;

pub const R_RST_ALL_CNT: c_uint = 0x0730;
pub const R_RST_ALL_CNT_BE4: c_uint = 0x20730;

pub const R_ENABLE_ALL_CNT: c_uint = 0x0730;
pub const R_ENABLE_ALL_CNT_BE4: c_uint = 0x20730;

pub const R_MAC_PIN_SEL: c_uint = 0x0734;
pub const R_MAC_PIN_SEL_BE4: c_uint = 0x20734;

pub const R_PLCP_HISTOGRAM: c_uint = 0x0738;
pub const R_PLCP_HISTOGRAM_BE_V1: c_uint = 0x20738;

pub const R_PHY_STS_BITMAP_ADDR_START_BE4: c_uint = 0x2073C;

pub const R_PHY_STS_BITMAP_SEARCH_FAIL: c_uint = 0x073C;
pub const B_PHY_STS_BITMAP_MSK_52A: c_uint = 0x337cff3f;
pub const R_PHY_STS_BITMAP_R2T: c_uint = 0x0740;
pub const R_PHY_STS_BITMAP_CCA_SPOOF: c_uint = 0x0744;
pub const R_PHY_STS_BITMAP_OFDM_BRK: c_uint = 0x0748;
pub const R_PHY_STS_BITMAP_CCK_BRK: c_uint = 0x074C;
pub const R_PHY_STS_BITMAP_DL_MU_SPOOF: c_uint = 0x0750;
pub const R_PHY_STS_BITMAP_HE_MU: c_uint = 0x0754;
pub const R_PHY_STS_BITMAP_VHT_MU: c_uint = 0x0758;
pub const R_PHY_STS_BITMAP_UL_TB_SPOOF: c_uint = 0x075C;
pub const R_PHY_STS_BITMAP_TRIGBASE: c_uint = 0x0760;
pub const R_PHY_STS_BITMAP_CCK: c_uint = 0x0764;
pub const R_PHY_STS_BITMAP_LEGACY: c_uint = 0x0768;
pub const R_PHY_STS_BITMAP_HT: c_uint = 0x076C;
pub const R_PHY_STS_BITMAP_VHT: c_uint = 0x0770;
pub const R_PHY_STS_BITMAP_HE: c_uint = 0x0774;
pub const R_PHY_STS_BITMAP_EHT: c_uint = 0x0788;
pub const R_PHY_STS_BITMAP_EHT_BE4: c_uint = 0x20788;
pub const R_EDCCA_RPTREG_SEL_BE: c_uint = 0x078C;
pub const R_EDCCA_RPTREG_SEL_BE4: c_uint = 0x2078C;

pub const R_PMAC_GNT: c_uint = 0x0980;

pub const R_PMAC_RX_CFG1: c_uint = 0x0988;

pub const R_PMAC_RXMOD: c_uint = 0x0994;

pub const R_MAC_SEL: c_uint = 0x09A4;

pub const R_PMAC_TX_CTRL: c_uint = 0x09C0;

pub const R_PMAC_TX_PRD: c_uint = 0x09C4;

pub const R_PMAC_TX_CNT: c_uint = 0x09C8;

pub const R_P80_AT_HIGH_FREQ: c_uint = 0x09D8;

pub const R_DBCC_80P80_SEL_EVM_RPT: c_uint = 0x0A10;

pub const R_CCX: c_uint = 0x0C00;
pub const R_CCX_BE4: c_uint = 0x20C00;

pub const R_NHM_CFG: c_uint = 0x0C08;

pub const R_NHM_TH1: c_uint = 0x0C0C;

pub const R_NHM_TH5: c_uint = 0x0C10;

pub const R_NHM_TH9: c_uint = 0x0C14;

pub const R_FAHM: c_uint = 0x0C1C;

pub const R_IFS_COUNTER: c_uint = 0x0C28;
pub const R_IFS_COUNTER_BE4: c_uint = 0x20C28;

pub const R_IFS_T1: c_uint = 0x0C2C;
pub const R_IFS_T1_BE4: c_uint = 0x20C2C;

pub const R_IFS_T2: c_uint = 0x0C30;
pub const R_IFS_T2_BE4: c_uint = 0x20C30;

pub const R_IFS_T3: c_uint = 0x0C34;
pub const R_IFS_T3_BE4: c_uint = 0x20C34;

pub const R_IFS_T4: c_uint = 0x0C38;
pub const R_IFS_T4_BE4: c_uint = 0x20C38;

pub const R_PD_CTRL: c_uint = 0x0C3C;

pub const R_IOQ_IQK_DPK: c_uint = 0x0C60;
pub const R_IOQ_IQK_DPK_BE4: c_uint = 0x20C60;

pub const R_GNT_BT_WGT_EN: c_uint = 0x0C6C;

pub const R_IQK_DPK_RST: c_uint = 0x0C6C;
pub const R_IQK_DPK_RST_BE4: c_uint = 0x20C6C;
pub const R_IQK_DPK_RST_C1: c_uint = 0x1C6C;

pub const R_TX_COLLISION_T2R_ST: c_uint = 0x0C70;

pub const R_TXGATING: c_uint = 0x0C74;

pub const R_TXRFC: c_uint = 0x0C7C;
pub const R_TXRFC_C1: c_uint = 0x1C7C;

pub const R_PD_ARBITER_OFF: c_uint = 0x0C80;

pub const R_SNDCCA_A1: c_uint = 0x0C9C;

pub const R_SNDCCA_A2: c_uint = 0x0CA0;

pub const R_UDP_COEEF: c_uint = 0x0CBC;

pub const R_TX_COLLISION_T2R_ST_BE: c_uint = 0x0CC8;
pub const R_TX_COLLISION_T2R_ST_BE4: c_uint = 0x20CC8;

pub const R_RXHT_MCS_LIMIT: c_uint = 0x0D18;

pub const R_RXVHT_MCS_LIMIT: c_uint = 0x0D18;

pub const R_BRK_OPT: c_uint = 0x0D44;

pub const R_P0_EN_SOUND_WO_NDP: c_uint = 0x0D7C;

pub const R_RXHE: c_uint = 0x0D80;

pub const R_SPOOF_ASYNC_RST: c_uint = 0x0D84;

pub const R_NDP_BRK0: c_uint = 0xDA0;
pub const R_NDP_BRK1: c_uint = 0xDA4;

pub const R_BRK_ASYNC_RST_EN_1: c_uint = 0x0DC0;
pub const R_BRK_ASYNC_RST_EN_2: c_uint = 0x0DC4;
pub const R_BRK_ASYNC_RST_EN_3: c_uint = 0x0DC8;
pub const R_NHM_BE: c_uint = 0x0EA4;

pub const R_CTLTOP: c_uint = 0x1008;

pub const R_CLK_GCK: c_uint = 0x1008;

pub const R_EDCCA_RPT_SEL_BE: c_uint = 0x10CC;
pub const R_ADC_FIFO_V1: c_uint = 0x10FC;

pub const R_S0_HW_SI_DIS: c_uint = 0x1200;

pub const R_P0_RXCK: c_uint = 0x12A0;

pub const R_P0_RFMODE: c_uint = 0x12AC;

pub const R_P0_RFMODE_ORI_RX: c_uint = 0x12AC;

pub const R_P0_RFMODE_FTM_RX: c_uint = 0x12B0;

pub const R_P0_NRBW: c_uint = 0x12B8;

pub const R_S0_RXDC: c_uint = 0x12D4;

pub const R_S0_RXDC2: c_uint = 0x12D8;

pub const R_CFO_COMP_SEG0_L: c_uint = 0x1384;
pub const R_CFO_COMP_SEG0_H: c_uint = 0x1388;
pub const R_CFO_COMP_SEG0_CTRL: c_uint = 0x138C;
pub const R_CNT_CCKTXEN: c_uint = 0x1700;
pub const R_CNT_CCKTXEN_V1: c_uint = 0x2E00;
pub const R_CNT_CCKTXEN_BE4: c_uint = 0x2CF00;

pub const R_CNT_CCKTXON: c_uint = 0x1704;
pub const R_CNT_CCKTXON_V1: c_uint = 0x2E04;
pub const R_CNT_CCKTXON_BE4: c_uint = 0x2CF04;

pub const R_CNT_CCK_CCA_P0: c_uint = 0x1710;
pub const R_CNT_CCK_CCA_P0_V1: c_uint = 0x2E10;

pub const R_CNT_CCK_CRC32_P0: c_uint = 0x1714;
pub const R_CNT_CCK_CRC32_P0_V1: c_uint = 0x2E14;

pub const R_DBG32_D: c_uint = 0x1730;
pub const R_EDCCA_RPT_A: c_uint = 0x1738;
pub const R_EDCCA_RPT_B: c_uint = 0x173c;

pub const R_EDCCA_RPT_P1_A: c_uint = 0x1740;
pub const R_EDCCA_RPT_P1_B: c_uint = 0x1744;
pub const R_SWSI_V1: c_uint = 0x174C;

pub const R_TX_INFO_0_0_COMB: c_uint = 0x1800;
pub const R_TX_INFO_0_0_COMB_V1: c_uint = 0x3E00;
pub const R_TX_INFO_0_1_COMB: c_uint = 0x1804;
pub const R_TX_INFO_0_1_COMB_V1: c_uint = 0x3E04;
pub const R_TX_INFO_1_0_COMB: c_uint = 0x1808;
pub const R_TX_INFO_1_0_COMB_V1: c_uint = 0x3E08;
pub const R_TX_INFO_1_1_COMB: c_uint = 0x180C;
pub const R_TX_INFO_1_1_COMB_V1: c_uint = 0x3E0C;
pub const R_TX_INFO_2_0_COMB_V1: c_uint = 0x3E10;
pub const R_TX_INFO_2_1_COMB_V1: c_uint = 0x3E14;
pub const R_TX_COMMON_CTRL_0_0_COMB: c_uint = 0x1810;
pub const R_TX_COMMON_CTRL_0_0_COMB_V1: c_uint = 0x3E20;
pub const R_TX_COMMON_CTRL_0_1_COMB: c_uint = 0x1814;
pub const R_TX_COMMON_CTRL_0_1_COMB_V1: c_uint = 0x3E24;
pub const R_CNT_LSIG_BRK_S_TH: c_uint = 0x1A00;
pub const R_CNT_LSIG_BRK_S_TH_V1: c_uint = 0x0E00;
pub const R_CNT_LSIG_BRK_S_TH_BE4: c_uint = 0x20E00;

pub const R_CNT_CCA_SPOOFING: c_uint = 0x1A00;
pub const R_CNT_CCA_SPOOFING_V1: c_uint = 0x0E00;
pub const R_CNT_CCA_SPOOFING_BE4: c_uint = 0x20E00;

pub const R_CNT_LSIG_BRK_L_TH: c_uint = 0x1A04;
pub const R_CNT_LSIG_BRK_L_TH_V1: c_uint = 0x0E04;
pub const R_CNT_LSIG_BRK_L_TH_BE4: c_uint = 0x20E04;

pub const R_CNT_BRK: c_uint = 0x1A08;
pub const R_CNT_BRK_V1: c_uint = 0x0E08;
pub const R_CNT_BRK_BE4: c_uint = 0x20E08;

pub const R_CNT_RXL_ERR_PARITY: c_uint = 0x1A0C;
pub const R_CNT_RXL_ERR_PARITY_V1: c_uint = 0x0E0C;
pub const R_CNT_RXL_ERR_PARITY_BE4: c_uint = 0x20E0C;

pub const R_CNT_RXL_ERR_RATE: c_uint = 0x1A10;
pub const R_CNT_RXL_ERR_RATE_V1: c_uint = 0x0E10;
pub const R_CNT_RXL_ERR_RATE_BE4: c_uint = 0x20E10;

pub const R_CNT_SEARCH_FAIL: c_uint = 0x1A20;
pub const R_CNT_SEARCH_FAIL_V1: c_uint = 0x0E20;
pub const R_CNT_SEARCH_FAIL_BE4: c_uint = 0x20E20;

pub const R_CNT_OFDM_CCA: c_uint = 0x1A24;
pub const R_CNT_OFDM_CCA_V1: c_uint = 0x0E24;
pub const R_CNT_OFDM_CCA_BE4: c_uint = 0x20E24;

pub const R_TX_COUNTER: c_uint = 0x1A40;
pub const R_CNT_OFDMTXON: c_uint = 0x1A40;
pub const R_CNT_OFDMTXON_V1: c_uint = 0x0E40;
pub const R_CNT_OFDMTXON_BE4: c_uint = 0x20E40;

pub const R_CNT_HE_CRC: c_uint = 0x1A58;
pub const R_CNT_HE_CRC_V1: c_uint = 0x0E58;
pub const R_CNT_HE_CRC_BE4: c_uint = 0x20E58;

pub const R_CNT_VHT_CRC: c_uint = 0x1A5C;
pub const R_CNT_VHT_CRC_V1: c_uint = 0x0E5C;
pub const R_CNT_VHT_CRC_BE4: c_uint = 0x20E5C;

pub const R_CNT_HT_CRC: c_uint = 0x1A60;
pub const R_CNT_HT_CRC_V1: c_uint = 0x0E60;
pub const R_CNT_HT_CRC_BE4: c_uint = 0x20E60;

pub const R_CNT_L_CRC: c_uint = 0x1A64;
pub const R_CNT_L_CRC_V1: c_uint = 0x0E64;
pub const R_CNT_L_CRC_BE4: c_uint = 0x20E64;

pub const R_CNT_AMPDU_MISS: c_uint = 0x1A7C;
pub const R_CNT_AMPDU_MISS_V1: c_uint = 0x0E7C;
pub const R_CNT_AMPDU_MISS_BE4: c_uint = 0x20E7C;

pub const R_CNT_AMPDU_RX_CRC32: c_uint = 0x1A80;
pub const R_CNT_AMPDU_RX_CRC32_V1: c_uint = 0x0E80;
pub const R_CNT_AMPDU_RX_CRC32_BE4: c_uint = 0x20E80;

pub const R_NHM_CNT0: c_uint = 0x1A88;

pub const R_NHM_CNT2: c_uint = 0x1A8C;

pub const R_NHM_CNT4: c_uint = 0x1A90;

pub const R_NHM_CNT6: c_uint = 0x1A94;

pub const R_NHM_CNT8: c_uint = 0x1A98;

pub const R_NHM_CNT10: c_uint = 0x1A9C;

pub const R_NHM_AX: c_uint = 0x1AA4;

pub const R_CLM_EDCCA_RESULT: c_uint = 0x1AC8;
pub const R_CLM_EDCCA_RESULT_V1: c_uint = 0x0EC8;
pub const R_CLM_EDCCA_RESULT_BE4: c_uint = 0x20EC8;

pub const R_CLM_EDCCA_RDY: c_uint = 0x1AC8;
pub const R_CLM_EDCCA_RDY_V1: c_uint = 0x0EC8;
pub const R_CLM_EDCCA_RDY_BE4: c_uint = 0x20EC8;

pub const R_IFS_CLM_TX_CNT: c_uint = 0x1ACC;
pub const R_IFS_CLM_TX_CNT_V1: c_uint = 0x0ECC;
pub const R_IFS_CLM_TX_CNT_BE4: c_uint = 0x20ECC;

pub const R_IFS_CLM_CCA: c_uint = 0x1AD0;
pub const R_IFS_CLM_CCA_V1: c_uint = 0x0ED0;
pub const R_IFS_CLM_CCA_BE4: c_uint = 0x20ED0;

pub const R_IFS_CLM_FA: c_uint = 0x1AD4;
pub const R_IFS_CLM_FA_V1: c_uint = 0x0ED4;
pub const R_IFS_CLM_FA_BE4: c_uint = 0x20ED4;

pub const R_IFS_HIS: c_uint = 0x1AD8;
pub const R_IFS_HIS_V1: c_uint = 0x0ED8;

pub const R_IFS_AVG_L: c_uint = 0x1ADC;
pub const R_IFS_AVG_L_V1: c_uint = 0x0EDC;

pub const R_IFS_AVG_H: c_uint = 0x1AE0;
pub const R_IFS_AVG_H_V1: c_uint = 0x0EE0;

pub const R_IFS_CCA_L: c_uint = 0x1AE4;
pub const R_IFS_CCA_L_V1: c_uint = 0x0EE4;

pub const R_IFS_CCA_H: c_uint = 0x1AE8;
pub const R_IFS_CCA_H_V1: c_uint = 0x0EE8;

pub const R_IFSCNT: c_uint = 0x1AEC;
pub const R_IFSCNT_V1: c_uint = 0x0EEC;

pub const R_TXAGC_TP: c_uint = 0x1C04;

pub const R_TSSI_THER: c_uint = 0x1C10;

pub const R_TSSI_CWRPT: c_uint = 0x1C18;

pub const R_TXAGC_BTP: c_uint = 0x1CA0;

pub const R_TXAGC_BB: c_uint = 0x1C60;

pub const R_PATH0_TXPWR: c_uint = 0x1C78;
pub const R_PATH0_TXPWR_V1: c_uint = 0xEE0C;
pub const R_PATH0_TXPWR_BE4: c_uint = 0x2F90C;

pub const R_S0_ADDCK: c_uint = 0x1E00;

pub const R_TXCKEN_FORCE: c_uint = 0x2008;

pub const R_EDCCA_RPT_SEL: c_uint = 0x20CC;

pub const R_ADC_FIFO: c_uint = 0x20fc;

pub const R_TXFIR0: c_uint = 0x2300;

pub const R_TXFIR2: c_uint = 0x2304;

pub const R_TXFIR4: c_uint = 0x2308;

pub const R_TXFIR6: c_uint = 0x230c;

pub const R_TXFIR8: c_uint = 0x2310;

pub const R_TXFIRA: c_uint = 0x2314;

pub const R_TXFIRC: c_uint = 0x2318;

pub const R_TXFIRE: c_uint = 0x231c;

pub const R_11B_RX_V1: c_uint = 0x2320;

pub const R_RPL_OFST: c_uint = 0x2340;

pub const R_RXCCA: c_uint = 0x2344;

pub const R_RXCCA_V1: c_uint = 0x2320;

pub const R_RXSC: c_uint = 0x237C;

pub const R_R1B_RR_SEL: c_uint = 0x2388;

pub const R_R1B_RX_RPT_RST: c_uint = 0x2388;
pub const R_R1B_RX_RPT_RST_V1: c_uint = 0x2340;
pub const R_R1B_RX_RPT_RST_BE: c_uint = 0x0540;
pub const R_R1B_RX_RPT_RST_BE4: c_uint = 0x20540;

pub const R_BRK_CNT: c_uint = 0x239C;
pub const R_BRK_CNT_V1: c_uint = 0x059C;
pub const R_BRK_CNT_BE4: c_uint = 0x2059C;

pub const R_RX_RPL_OFST: c_uint = 0x23AC;

pub const R_RXSCOBC: c_uint = 0x23B0;

pub const R_RXSCOCCK: c_uint = 0x23B4;

pub const R_SFD_GG_CNT: c_uint = 0x23E0;
pub const R_SFD_GG_CNT_V1: c_uint = 0x23A0;
pub const R_SFD_GG_CNT_V2: c_uint = 0x05A0;
pub const R_SFD_GG_CNT_BE4: c_uint = 0x205A0;

pub const R_SIG_GG_CNT: c_uint = 0x23E8;
pub const R_SIG_GG_CNT_V1: c_uint = 0x23AC;
pub const R_SIG_GG_CNT_V2: c_uint = 0x05AC;
pub const R_SIG_GG_CNT_BE4: c_uint = 0x205AC;

pub const R_SPOOF_CNT: c_uint = 0x23EC;
pub const R_SPOOF_CNT_V1: c_uint = 0x23A8;
pub const R_SPOOF_CNT_V2: c_uint = 0x05A8;
pub const R_SPOOF_CNT_BE4: c_uint = 0x205A8;

pub const R_P80_AT_HIGH_FREQ_RU_ALLOC: c_uint = 0x2410;

pub const R_DBCC_80P80_SEL_EVM_RPT2: c_uint = 0x2A10;

pub const R_AFEDAC0: c_uint = 0x2A5C;

pub const R_AFEDAC1: c_uint = 0x2A60;

pub const R_IQKDPK_HC: c_uint = 0x2AB8;

pub const R_HWSI_ADD0: c_uint = 0x2ADC;
pub const R_HWSI_ADD1: c_uint = 0x2BDC;

pub const R_HWSI_DATA: c_uint = 0x2AE0;

pub const R_HWSI_VAL0: c_uint = 0x2C24;
pub const R_HWSI_VAL1: c_uint = 0x2D24;

pub const R_P1_EN_SOUND_WO_NDP: c_uint = 0x2D7C;

pub const R_EDCCA_RPT_A_BE: c_uint = 0x2E38;
pub const R_EDCCA_RPT_A_BE4: c_uint = 0x2EE30;
pub const R_EDCCA_RPT_A_BE4_C1: c_uint = 0x2FE30;
pub const R_EDCCA_RPT_B_BE: c_uint = 0x2E3C;
pub const R_EDCCA_RPT_B_BE4: c_uint = 0x2EE34;
pub const R_EDCCA_RPT_B_BE4_C1: c_uint = 0x2FE34;
pub const R_EDCCA_RPT_P1_A_BE: c_uint = 0x2E40;
pub const R_EDCCA_RPT_P1_B_BE: c_uint = 0x2E44;
pub const R_CNT_EHT_CRC: c_uint = 0x2F00;
pub const R_CNT_EHT_CRC_BE4: c_uint = 0x22F00;

pub const R_S1_HW_SI_DIS: c_uint = 0x3200;

pub const R_P1_RXCK: c_uint = 0x32A0;

pub const R_P1_RFMODE: c_uint = 0x32AC;

pub const R_P1_RFMODE_ORI_RX: c_uint = 0x32AC;

pub const R_P1_RFMODE_FTM_RX: c_uint = 0x32B0;

pub const R_P1_DBGMOD: c_uint = 0x32B8;

pub const R_S1_RXDC: c_uint = 0x32D4;

pub const R_S1_RXDC2: c_uint = 0x32D8;

pub const R_TXAGC_BB_S1: c_uint = 0x3C60;

pub const R_PATH1_TXPWR: c_uint = 0x3C78;
pub const R_PATH1_TXPWR_V1: c_uint = 0xEF0C;
pub const R_PATH1_TXPWR_BE4: c_uint = 0x2FA0C;

pub const R_S1_ADDCK: c_uint = 0x3E00;

pub const R_OP1DB_A: c_uint = 0x40B0;

pub const R_OP1DB1_A: c_uint = 0x40BC;

pub const R_BKOFF_A: c_uint = 0x40E0;

pub const R_BACKOFF_A: c_uint = 0x40E4;

pub const R_RXBY_WBADC_A: c_uint = 0x40F4;

pub const R_MUIC: c_uint = 0x40F8;

pub const R_BT_RXBY_WBADC_A: c_uint = 0x4160;

pub const R_BT_SHARE_A: c_uint = 0x4164;

pub const R_FORCE_FIR_A: c_uint = 0x418C;

pub const R_DCFO: c_uint = 0x4264;

pub const R_SEG0CSI: c_uint = 0x42AC;
pub const R_SEG0CSI_V1: c_uint = 0x42B0;

pub const R_SEG0CSI_EN: c_uint = 0x42C4;
pub const R_SEG0CSI_EN_V1: c_uint = 0x42C8;

pub const R_BSS_CLR_MAP: c_uint = 0x43ac;
pub const R_BSS_CLR_MAP_V1: c_uint = 0x43B0;
pub const R_BSS_CLR_MAP_V2: c_uint = 0x4EB0;

pub const R_CFO_TRK0: c_uint = 0x4404;
pub const R_CFO_TRK1: c_uint = 0x440C;

pub const R_T2F_GI_COMB: c_uint = 0x4424;

pub const R_BT_DYN_DC_EST_EN: c_uint = 0x441C;
pub const R_BT_DYN_DC_EST_EN_V1: c_uint = 0x4420;

pub const R_ASSIGN_SBD_OPT_V1: c_uint = 0x4440;

pub const R_ASSIGN_SBD_OPT: c_uint = 0x4450;

pub const R_DCFO_COMP_S0: c_uint = 0x448C;

pub const R_DCFO_WEIGHT: c_uint = 0x4490;

pub const R_DCFO_OPT: c_uint = 0x4494;

pub const R_BANDEDGE: c_uint = 0x4498;

pub const R_DPD_BF: c_uint = 0x44a0;

pub const R_LNA_OP: c_uint = 0x44B0;

pub const R_LNA_TIA: c_uint = 0x44BC;

pub const R_BKOFF_B: c_uint = 0x44E0;

pub const R_BACKOFF_B: c_uint = 0x44E4;

pub const R_RXBY_WBADC_B: c_uint = 0x44F4;

pub const R_BT_RXBY_WBADC_B: c_uint = 0x4560;

pub const R_BT_SHARE_B: c_uint = 0x4564;

pub const R_TXPATH_SEL: c_uint = 0x458C;

pub const R_FORCE_FIR_B: c_uint = 0x458C;

pub const R_TXPWR: c_uint = 0x4594;

pub const R_TXNSS_MAP: c_uint = 0x45B4;

pub const R_PCOEFF0_V1: c_uint = 0x45BC;

pub const R_PCOEFF2_V1: c_uint = 0x45CC;

pub const R_PCOEFF4_V1: c_uint = 0x45D0;

pub const R_PCOEFF6_V1: c_uint = 0x45D4;

pub const R_PCOEFF8_V1: c_uint = 0x45D8;

pub const R_PCOEFFA_V1: c_uint = 0x45C0;

pub const R_PCOEFFC_V1: c_uint = 0x45C4;

pub const R_PCOEFFE_V1: c_uint = 0x45C8;

pub const R_PATH0_IB_PKPW: c_uint = 0x4628;

pub const R_PATH0_LNA_ERR1: c_uint = 0x462C;

pub const R_PATH0_LNA_ERR2: c_uint = 0x4630;

pub const R_PATH0_LNA_ERR3: c_uint = 0x4634;

pub const R_PATH0_LNA_ERR4: c_uint = 0x4638;

pub const R_PATH0_LNA_ERR5: c_uint = 0x463C;

pub const R_PATH0_TIA_ERR_G0: c_uint = 0x4640;

pub const R_PATH0_TIA_ERR_G1: c_uint = 0x4644;

pub const R_PATH0_IB_PBK: c_uint = 0x4650;

pub const R_PATH0_RXB_INIT: c_uint = 0x4658;

pub const R_PATH0_LNA_INIT: c_uint = 0x4668;
pub const R_PATH0_LNA_INIT_V1: c_uint = 0x472C;

pub const R_PATH0_BTG: c_uint = 0x466C;

pub const R_PATH0_TIA_INIT: c_uint = 0x4674;

pub const R_PATH0_P20_FOLLOW_BY_PAGCUGC: c_uint = 0x46A0;
pub const R_PATH0_P20_FOLLOW_BY_PAGCUGC_V1: c_uint = 0x4C24;
pub const R_PATH0_P20_FOLLOW_BY_PAGCUGC_V2: c_uint = 0x46E8;
pub const R_PATH0_P20_FOLLOW_BY_PAGCUGC_V3: c_uint = 0x41C8;
pub const R_PATH0_P20_FOLLOW_BY_PAGCUGC_BE4: c_uint = 0x241C8;

pub const R_PATH0_S20_FOLLOW_BY_PAGCUGC: c_uint = 0x46A4;
pub const R_PATH0_S20_FOLLOW_BY_PAGCUGC_V1: c_uint = 0x4C28;
pub const R_PATH0_S20_FOLLOW_BY_PAGCUGC_V2: c_uint = 0x46EC;
pub const R_PATH0_S20_FOLLOW_BY_PAGCUGC_V3: c_uint = 0x41CC;
pub const R_PATH0_S20_FOLLOW_BY_PAGCUGC_BE4: c_uint = 0x241CC;

pub const R_PATH0_RXB_INIT_V1: c_uint = 0x46A8;

pub const R_PATH0_G_LNA6_OP1DB_V1: c_uint = 0x4688;

pub const R_PATH0_G_TIA0_LNA6_OP1DB_V1: c_uint = 0x4694;

pub const R_PATH0_G_TIA1_LNA6_OP1DB_V1: c_uint = 0x4694;

pub const R_CDD_EVM_CHK_EN: c_uint = 0x46C0;

pub const R_PATH0_BAND_SEL_V1: c_uint = 0x4738;

pub const R_PATH0_BT_SHARE_V1: c_uint = 0x4738;

pub const R_PATH0_BTG_PATH_V1: c_uint = 0x4738;

pub const R_P0_NBIIDX: c_uint = 0x469C;

pub const R_P0_BACKOFF_IBADC_V1: c_uint = 0x469C;

pub const R_P1_MODE: c_uint = 0x4718;

pub const R_P0_AGC_CTL: c_uint = 0x4730;

pub const R_PATH1_LNA_INIT: c_uint = 0x473C;
pub const R_PATH1_LNA_INIT_V1: c_uint = 0x4A80;

pub const R_PATH0_TIA_INIT_V1: c_uint = 0x473C;

pub const R_PATH1_TIA_INIT: c_uint = 0x4748;

pub const R_PATH1_BTG: c_uint = 0x4740;

pub const R_PATH1_RXB_INIT: c_uint = 0x472C;

pub const R_PATH1_G_LNA6_OP1DB_V1: c_uint = 0x476C;

pub const R_PATH1_P20_FOLLOW_BY_PAGCUGC: c_uint = 0x4774;
pub const R_PATH1_P20_FOLLOW_BY_PAGCUGC_V1: c_uint = 0x4CE8;
pub const R_PATH1_P20_FOLLOW_BY_PAGCUGC_V2: c_uint = 0x47A8;
pub const R_PATH1_P20_FOLLOW_BY_PAGCUGC_V3: c_uint = 0x45C8;
pub const R_PATH1_P20_FOLLOW_BY_PAGCUGC_BE4: c_uint = 0x245C8;

pub const R_PATH1_S20_FOLLOW_BY_PAGCUGC: c_uint = 0x4778;
pub const R_PATH1_S20_FOLLOW_BY_PAGCUGC_V1: c_uint = 0x4CEC;
pub const R_PATH1_S20_FOLLOW_BY_PAGCUGC_V2: c_uint = 0x47AC;
pub const R_PATH1_S20_FOLLOW_BY_PAGCUGC_V3: c_uint = 0x45CC;
pub const R_PATH1_S20_FOLLOW_BY_PAGCUGC_BE4: c_uint = 0x245CC;

pub const R_PATH1_G_TIA0_LNA6_OP1DB_V1: c_uint = 0x4778;

pub const R_PATH1_G_TIA1_LNA6_OP1DB_V1: c_uint = 0x4778;

pub const R_PATH1_BAND_SEL_V1: c_uint = 0x4AA4;

pub const R_PATH1_BT_SHARE_V1: c_uint = 0x4AA4;

pub const R_PATH1_BTG_PATH_V1: c_uint = 0x4AA4;

pub const R_P1_NBIIDX: c_uint = 0x4770;

pub const R_PKT_CTRL: c_uint = 0x47D4;

pub const R_SEG0R_PD: c_uint = 0x481C;
pub const R_SEG0R_PD_V1: c_uint = 0x4860;
pub const R_SEG0R_PD_V2: c_uint = 0x6A74;
pub const R_SEG0R_PD_BE4: c_uint = 0x26210;
pub const R_SEG0R_EDCCA_LVL: c_uint = 0x4840;
pub const R_SEG0R_EDCCA_LVL_V1: c_uint = 0x4884;

pub const R_PWOFST: c_uint = 0x488C;

pub const R_2P4G_BAND: c_uint = 0x4970;

pub const R_FC0_BW: c_uint = 0x4974;
pub const R_FC0_BW_V1: c_uint = 0x49C0;

pub const R_Q_MATRIX_00: c_uint = 0x497C;

pub const R_CHBW_MOD: c_uint = 0x4978;
pub const R_CHBW_MOD_V1: c_uint = 0x49C4;

pub const R_Q_MATRIX_11: c_uint = 0x4988;

pub const R_CUSTOMIZE_Q_MATRIX: c_uint = 0x498C;

pub const R_P0_RPL1: c_uint = 0x49B0;

pub const B_P0_RPL1_SHIFT: c_int = 8;

pub const R_P0_RPL2: c_uint = 0x49B4;

pub const R_P0_RPL3: c_uint = 0x49B8;

pub const R_PD_BOOST_EN: c_uint = 0x49E8;

pub const R_P1_BACKOFF_IBADC_V1: c_uint = 0x49F0;

pub const R_P1_RPL1: c_uint = 0x4A00;
pub const R_P1_RPL2: c_uint = 0x4A04;
pub const R_P1_RPL3: c_uint = 0x4A08;
pub const R_BK_FC0_INV_V1: c_uint = 0x4A1C;

pub const R_CCK_FC0_INV_V1: c_uint = 0x4A20;

pub const R_PATH1_RXB_INIT_V1: c_uint = 0x4A5C;

pub const R_P1_AGC_CTL: c_uint = 0x4A9C;

pub const R_PATH1_TIA_INIT_V1: c_uint = 0x4AA8;

pub const R_P0_AGC_RSVD: c_uint = 0x4ACC;
pub const R_PATH0_RXBB_V1: c_uint = 0x4AD4;

pub const R_P1_AGC_RSVD: c_uint = 0x4AD8;
pub const R_PATH1_RXBB_V1: c_uint = 0x4AE0;

pub const R_PATH0_BT_BACKOFF_V1: c_uint = 0x4AE4;

pub const R_PATH1_BT_BACKOFF_V1: c_uint = 0x4AEC;

pub const R_DCFO_COMP_S0_V2: c_uint = 0x4B20;

pub const R_PATH0_TX_CFR: c_uint = 0x4B30;

pub const R_PATH0_TX_POLAR_CLIPPING: c_uint = 0x4B3C;

pub const R_PATH0_FRC_FIR_TYPE_V1: c_uint = 0x4C00;

pub const R_PATH0_NOTCH: c_uint = 0x4C14;

pub const R_PATH0_NOTCH2: c_uint = 0x4C20;

pub const R_PATH0_5MDET: c_uint = 0x4C4C;
pub const R_PATH0_5MDET_V1: c_uint = 0x46F8;

pub const R_PATH1_FRC_FIR_TYPE_V1: c_uint = 0x4CC4;

pub const R_PATH1_NOTCH: c_uint = 0x4CD8;

pub const R_PATH1_NOTCH2: c_uint = 0x4CE4;

pub const R_PATH1_5MDET: c_uint = 0x4D10;
pub const R_PATH1_5MDET_V1: c_uint = 0x47B8;

pub const R_S0S1_CSI_WGT: c_uint = 0x4D34;

pub const R_CHINFO_ELM_SRC: c_uint = 0x4D84;

pub const R_CHINFO_TYPE_SCAL: c_uint = 0x4D88;

pub const R_RPL_BIAS_COMP: c_uint = 0x4DF0;

pub const R_RPL_PATHAB: c_uint = 0x4E0C;

pub const R_RSSI_M_PATHAB: c_uint = 0x4E2C;

pub const R_FC0_V1: c_uint = 0x4E30;

pub const R_RX_BW40_2XFFT_EN_V1: c_uint = 0x4E30;

pub const R_DCFO_COMP_S0_V1: c_uint = 0x4A40;

pub const R_BMODE_PDTH_V1: c_uint = 0x4B64;
pub const R_BMODE_PDTH_V2: c_uint = 0x6708;
pub const R_BMODE_PDTH_BE4: c_uint = 0x26040;

pub const R_BMODE_PDTH_EN_V1: c_uint = 0x4B74;
pub const R_BMODE_PDTH_EN_V2: c_uint = 0x6718;
pub const R_BMODE_PDTH_EN_BE4: c_uint = 0x26050;

pub const R_BSS_CLR_VLD_V2: c_uint = 0x4EBC;

pub const R_CFO_COMP_SEG1_L: c_uint = 0x5384;
pub const R_CFO_COMP_SEG1_H: c_uint = 0x5388;
pub const R_CFO_COMP_SEG1_CTRL: c_uint = 0x538C;

pub const R_TSSI_PA_K1: c_uint = 0x5600;
pub const R_TSSI_PA_K2: c_uint = 0x5604;
pub const R_P0_TSSI_ALIM1: c_uint = 0x5630;

pub const R_P0_TSSI_ALIM3: c_uint = 0x5634;

pub const R_TSSI_PA_K5: c_uint = 0x5638;
pub const R_P0_TSSI_ALIM2: c_uint = 0x563c;

pub const R_P0_TSSI_ALIM4: c_uint = 0x5640;
pub const R_TSSI_PA_K8: c_uint = 0x5644;
pub const R_P0_TSSI_ADC_CLK: c_uint = 0x566c;

pub const R_UPD_CLK: c_uint = 0x5670;

pub const R_TXPWRB: c_uint = 0x56CC;
pub const R_P1_TXPWRB: c_uint = 0x76CC;

pub const R_DPD_OFT_EN: c_uint = 0x5800;

pub const R_P0_TSSIC: c_uint = 0x5814;

pub const R_DPD_OFT_ADDR: c_uint = 0x5804;

pub const R_TXPWRB_H: c_uint = 0x580c;

pub const R_P0_TMETER: c_uint = 0x5810;

pub const R_P0_ADCFF_EN: c_uint = 0x58C8;

pub const R_P1_TSSIC: c_uint = 0x7814;

pub const R_P0_TSSI_TRK: c_uint = 0x5818;

pub const R_P0_TSSI_SLOPE_CAL: c_uint = 0x581c;

pub const R_P0_TSSI_AVG: c_uint = 0x5820;

pub const R_P0_RFCTM: c_uint = 0x5864;

pub const R_P0_TRSW: c_uint = 0x5868;

pub const R_P0_ANTSEL: c_uint = 0x586C;

pub const R_RFSW_CTRL_ANT0_BASE: c_uint = 0x5870;

pub const R_RFE_SEL0_BASE: c_uint = 0x5880;

pub const R_RFE_SEL32_BASE: c_uint = 0x5884;
pub const RFE_SEL0_SRC_ANTSEL_0: c_int = 8;
pub const R_RFE_INV0: c_uint = 0x5890;
pub const R_P0_RFM: c_uint = 0x5894;

pub const R_P0_PATH_RST: c_uint = 0x58AC;

pub const R_P0_TXDPD: c_uint = 0x58D4;

pub const R_P0_TXPW_RSTB: c_uint = 0x58DC;

pub const R_P0_TSSI_MV_AVG: c_uint = 0x58E4;

pub const R_TXGAIN_SCALE: c_uint = 0x58F0;

pub const R_P0_DAC_COMP_POST_DPD_EN: c_uint = 0x58F8;

pub const R_P0_TSSI_BASE: c_uint = 0x5C00;
pub const R_S0_DACKI: c_uint = 0x5E00;

pub const R_S0_DACKI2: c_uint = 0x5E30;

pub const R_S0_DACKI7: c_uint = 0x5E44;

pub const R_S0_DACKI8: c_uint = 0x5E48;

pub const R_S0_DACKQ: c_uint = 0x5E50;

pub const R_S0_DACKQ2: c_uint = 0x5E80;

pub const R_S0_DACKQ7: c_uint = 0x5E94;

pub const R_S0_DACKQ8: c_uint = 0x5E98;

pub const R_DCFO_WEIGHT_BE: c_uint = 0x6244;
pub const R_DCFO_WEIGHT_BE_V1: c_uint = 0x24808;

pub const R_DAC_CLK: c_uint = 0x625C;

pub const R_DCFO_OPT_BE: c_uint = 0x6260;
pub const R_DCFO_OPT_BE_V1: c_uint = 0x24824;

pub const R_TXFCTR: c_uint = 0x627C;

pub const R_TXSCALE: c_uint = 0x6284;

pub const R_PCOEFF01: c_uint = 0x6684;

pub const R_PCOEFF23: c_uint = 0x6688;

pub const R_PCOEFF45: c_uint = 0x668c;

pub const R_PCOEFF67: c_uint = 0x6690;

pub const R_PCOEFF89: c_uint = 0x6694;

pub const R_PCOEFFAB: c_uint = 0x6698;

pub const R_PCOEFFCD: c_uint = 0x669c;

pub const R_PCOEFFEF: c_uint = 0x66a0;

pub const R_MGAIN_BIAS: c_uint = 0x672c;

pub const R_CCK_RPL_OFST: c_uint = 0x6750;

pub const R_BK_FC0INV: c_uint = 0x6758;

pub const R_CCK_FC0INV: c_uint = 0x675c;

pub const R_SEG0R_EDCCA_LVL_BE: c_uint = 0x69EC;
pub const R_SEG0R_EDCCA_LVL_BE4: c_uint = 0x2623C;
pub const R_SEG0R_PPDU_LVL_BE: c_uint = 0x69F0;
pub const R_SEG0R_PPDU_LVL_BE4: c_uint = 0x26240;
pub const R_SEGSND: c_uint = 0x6A14;

pub const R_DBCC: c_uint = 0x6B48;

pub const R_FC0: c_uint = 0x6B4C;

pub const R_FC0INV_SBW: c_uint = 0x6B50;

pub const R_ANT_CHBW: c_uint = 0x6B54;

pub const R_SLOPE: c_uint = 0x6B6C;

pub const R_SC_CORNER: c_uint = 0x6B70;

pub const R_MAG_A: c_uint = 0x6BF4;

pub const R_MAG_AB: c_uint = 0x6BF8;

pub const R_BEDGE: c_uint = 0x6BFC;

pub const R_BEDGE2: c_uint = 0x6C00;

pub const R_BEDGE3: c_uint = 0x6C04;

pub const R_SU_PUNC: c_uint = 0x6C08;

pub const R_BEDGE5: c_uint = 0x6C10;

pub const R_RPL_BIAS_COMP1: c_uint = 0x6DF0;

pub const R_DBCC_FA: c_uint = 0x703C;

pub const R_P1_TSSI_ALIM1: c_uint = 0x7630;

pub const R_P1_TSSI_ALIM3: c_uint = 0x7634;

pub const R_P1_TSSI_ALIM2: c_uint = 0x763c;

pub const R_P1_TSSI_ADC_CLK: c_uint = 0x766c;

pub const R_P1_TXAGC_TH: c_uint = 0x7800;

pub const R_P1_TXPW_FORCE: c_uint = 0x780C;

pub const R_P1_TSSIC: c_uint = 0x7814;

pub const R_P1_TMETER: c_uint = 0x7810;

pub const R_P1_TSSI_TRK: c_uint = 0x7818;

pub const R_P1_TSSI_AVG: c_uint = 0x7820;

pub const R_P1_RFCTM: c_uint = 0x7864;

pub const R_P1_PATH_RST: c_uint = 0x78AC;

pub const R_P1_ADCFF_EN: c_uint = 0x78C8;

pub const R_P1_TXPW_RSTB: c_uint = 0x78DC;

pub const R_P1_TSSI_MV_AVG: c_uint = 0x78E4;

pub const R_P1_DAC_COMP_POST_DPD_EN: c_uint = 0x78F8;

pub const R_TSSI_THOF: c_uint = 0x7C00;
pub const R_S1_DACKI: c_uint = 0x7E00;

pub const R_S1_DACKI2: c_uint = 0x7E30;

pub const R_S1_DACKI7: c_uint = 0x7E44;

pub const R_S1_DACKI8: c_uint = 0x7E48;

pub const R_S1_DACKQ: c_uint = 0x7E50;

pub const R_S1_DACKQ2: c_uint = 0x7E80;

pub const R_S1_DACKQ7: c_uint = 0x7E94;

pub const R_S1_DACKQ8: c_uint = 0x7E98;

pub const R_NCTL_CFG: c_uint = 0x8000;
pub const R_NCTL_CFG_BE4: c_uint = 0x38000;

pub const R_NCTL_RPT: c_uint = 0x8008;

pub const R_NCTL_N1: c_uint = 0x8010;

pub const R_NCTL_N2: c_uint = 0x8014;
pub const R_IQK_COM: c_uint = 0x8018;
pub const R_IQK_DIF: c_uint = 0x801C;

pub const R_IQK_DIF1: c_uint = 0x8020;

pub const R_IQK_DIF2: c_uint = 0x8024;

pub const R_IQK_DIF4: c_uint = 0x802C;

pub const IQK_DF4_TXT_8_25MHZ: c_uint = 0x021;
pub const R_IQK_CFG: c_uint = 0x8034;

pub const R_IQK_RXA: c_uint = 0x8044;

pub const R_TPG_SEL: c_uint = 0x8068;
pub const R_TPG_MOD: c_uint = 0x806C;

pub const R_MDPK_SYNC: c_uint = 0x8070;

pub const R_MDPK_RX_DCK: c_uint = 0x8074;

pub const R_KIP_MOD: c_uint = 0x8078;

pub const R_NCTL_RW: c_uint = 0x8080;
pub const R_NCTL_RW_BE4: c_uint = 0x38080;

pub const R_KIP_SYSCFG: c_uint = 0x8088;
pub const R_KIP_CLK: c_uint = 0x808C;
pub const R_DPK_IDL: c_uint = 0x809C;

pub const R_LDL_NORM: c_uint = 0x80A0;

pub const R_DPK_CTL: c_uint = 0x80B0;

pub const R_DPK_CFG: c_uint = 0x80B8;

pub const R_DPK_CFG2: c_uint = 0x80BC;

pub const R_DPK_CFG3: c_uint = 0x80C0;
pub const R_KPATH_CFG: c_uint = 0x80D0;

pub const R_KIP_RPT1: c_uint = 0x80D4;

pub const R_SRAM_IQRX: c_uint = 0x80D8;
pub const R_IDL_MPA: c_uint = 0x80DC;

pub const R_GAPK: c_uint = 0x80E0;

pub const R_SRAM_IQRX2: c_uint = 0x80E8;
pub const R_DPK_MPA: c_uint = 0x80EC;

pub const R_DPK_WR: c_uint = 0x80F4;

pub const R_DPK_TRK: c_uint = 0x80f0;

pub const R_RPT_COM: c_uint = 0x80FC;

pub const R_COEF_SEL: c_uint = 0x8104;
pub const R_COEF_SEL_C1: c_uint = 0x8204;

pub const R_CFIR_COEF: c_uint = 0x810c;
pub const R_CFIR_SYS: c_uint = 0x8120;
pub const R_IQK_RES: c_uint = 0x8124;

pub const R_TXIQC: c_uint = 0x8138;
pub const R_RXIQC: c_uint = 0x813c;

pub const R_KIP: c_uint = 0x8140;

pub const R_RFGAIN: c_uint = 0x8144;

pub const R_RFGAIN_BND: c_uint = 0x8148;

pub const R_CFIR_MAP: c_uint = 0x8150;
pub const R_CFIR_LUT: c_uint = 0x8154;
pub const R_CFIR_LUT_C1: c_uint = 0x8254;

pub const R_DPK_GN: c_uint = 0x819C;

pub const R_DPD_V1: c_uint = 0x81a0;

pub const R_DPD_CH0: c_uint = 0x81AC;
pub const R_DPD_BND: c_uint = 0x81B4;

pub const R_DPD_CH0A: c_uint = 0x81BC;

pub const R_TXAGC_RFK: c_uint = 0x81C4;

pub const R_DPD_COM: c_uint = 0x81C8;

pub const R_KIP_IQP: c_uint = 0x81CC;

pub const R_KIP_RPT: c_uint = 0x81D4;

pub const R_W_COEF: c_uint = 0x81D8;
pub const R_LOAD_COEF: c_uint = 0x81DC;

pub const R_DPK_GL: c_uint = 0x81F0;

pub const R_RPT_PER: c_uint = 0x81FC;

pub const R_IQRSN: c_uint = 0x8220;

pub const R_DPD_CH0B: c_uint = 0x82BC;
pub const R_RXCFIR_P0C0: c_uint = 0x8D40;
pub const R_RXCFIR_P0C1: c_uint = 0x8D84;
pub const R_RXCFIR_P0C2: c_uint = 0x8DC8;
pub const R_RXCFIR_P0C3: c_uint = 0x8E0C;
pub const R_TXCFIR_P0C0: c_uint = 0x8F50;
pub const R_TXCFIR_P0C1: c_uint = 0x8F84;
pub const R_TXCFIR_P0C2: c_uint = 0x8FB8;
pub const R_TXCFIR_P0C3: c_uint = 0x8FEC;
pub const R_RXCFIR_P1C0: c_uint = 0x9140;
pub const R_RXCFIR_P1C1: c_uint = 0x9184;
pub const R_RXCFIR_P1C2: c_uint = 0x91C8;
pub const R_RXCFIR_P1C3: c_uint = 0x920C;
pub const R_TXCFIR_P1C0: c_uint = 0x9350;
pub const R_TXCFIR_P1C1: c_uint = 0x9384;
pub const R_TXCFIR_P1C2: c_uint = 0x93B8;
pub const R_TXCFIR_P1C3: c_uint = 0x93EC;
pub const R_IQKINF: c_uint = 0x9FE0;

pub const R_IQKCH: c_uint = 0x9FE4;

pub const R_IQKINF2: c_uint = 0x9FE8;

pub const R_TXAGC_REF_DBM_RF1_P0: c_uint = 0xBC04;

pub const R_TSSI_K_RF1_P0: c_uint = 0xBC28;

pub const R_TXAGC_REF_DBM_RF1_P1: c_uint = 0xBD04;

pub const R_TSSI_K_RF1_P1: c_uint = 0xBD28;

pub const R_RFK_ST: c_uint = 0xBFF8;
pub const R_DCOF0: c_uint = 0xC000;

pub const R_DCOF1: c_uint = 0xC004;

pub const R_DCOF8: c_uint = 0xC020;

pub const R_DCOF9: c_uint = 0xC024;

pub const R_DACK_S0P0: c_uint = 0xC040;

pub const R_DACK_BIAS00: c_uint = 0xc048;

pub const R_DACK_S0P2: c_uint = 0xC05C;

pub const R_DACK_DADCK00: c_uint = 0xC060;

pub const R_DACK_S0P1: c_uint = 0xC064;

pub const R_DACK_BIAS01: c_uint = 0xC06C;

pub const R_DACK_S0P3: c_uint = 0xC080;

pub const R_DACK_DADCK01: c_uint = 0xC084;

pub const R_DRCK_FH: c_uint = 0xC094;

pub const R_DRCK: c_uint = 0xC0C4;

pub const R_DRCK_RES: c_uint = 0xC0C8;

pub const R_DRCK_V1: c_uint = 0xC0CC;

pub const R_DRCK_RS: c_uint = 0xC0D0;

pub const R_PATH0_SAMPL_DLY_T_V1: c_uint = 0xC0D4;

pub const R_P0_CFCH_BW0: c_uint = 0xC0D4;

pub const R_P0_CFCH_BW1: c_uint = 0xC0D8;

pub const R_WDADC: c_uint = 0xC0E4;

pub const R_ADCMOD: c_uint = 0xC0E8;

pub const R_DCIM: c_uint = 0xC0EC;

pub const R_ADDCK0D: c_uint = 0xC0F0;

pub const R_ADDCK0: c_uint = 0xC0F4;

pub const R_ADDCK0_RL: c_uint = 0xC0F8;

pub const R_ADDCKR0: c_uint = 0xC0FC;

pub const R_DACK10: c_uint = 0xC100;

pub const R_DACK1_K: c_uint = 0xc104;

pub const R_DACK11: c_uint = 0xC120;

pub const R_DACK2_K: c_uint = 0xC124;

pub const R_DACK_S1P0: c_uint = 0xC140;

pub const R_DACK_BIAS10: c_uint = 0xC148;

pub const R_DACK10S: c_uint = 0xC15C;

pub const R_DACK_S1P2: c_uint = 0xC15C;

pub const R_DACK_DADCK10: c_uint = 0xC160;

pub const R_DACK_S1P1: c_uint = 0xC164;

pub const R_DACK_BIAS11: c_uint = 0xC16C;

pub const R_DACK11S: c_uint = 0xC180;

pub const R_DACK_S1P3: c_uint = 0xC180;

pub const R_DACK_DADCK11: c_uint = 0xC184;

pub const R_PATH1_SAMPL_DLY_T_V1: c_uint = 0xC1D4;

pub const R_PATH0_BW_SEL_V1: c_uint = 0xC0D8;

pub const R_PATH1_BW_SEL_V1: c_uint = 0xC1D8;

pub const R_ADDCK1D: c_uint = 0xC1F0;

pub const R_ADDCK1: c_uint = 0xC1F4;

pub const R_ADDCK1_RL: c_uint = 0xC1F8;

pub const R_ADDCKR1: c_uint = 0xC1fC;

pub const R_DACKN0_CTL: c_uint = 0xC210;

pub const R_DACKN1_CTL: c_uint = 0xC224;

pub const R_DACKN2_CTL: c_uint = 0xC238;

pub const R_DACKN3_CTL: c_uint = 0xC24C;

pub const R_GAIN_MAP0: c_uint = 0xE44C;

pub const R_GAIN_MAP1: c_uint = 0xE54C;

pub const R_GOTX_IQKDPK_C0: c_uint = 0xE464;
pub const R_GOTX_IQKDPK_C0_BE4: c_uint = 0x2E464;
pub const R_GOTX_IQKDPK_C1: c_uint = 0xE564;
pub const R_GOTX_IQKDPK_C1_BE4: c_uint = 0x2E564;

pub const R_IQK_DPK_PRST: c_uint = 0xE4AC;
pub const R_IQK_DPK_PRST_BE4: c_uint = 0x2E4AC;
pub const R_IQK_DPK_PRST_C1: c_uint = 0xE5AC;
pub const R_IQK_DPK_PRST_C1_BE4: c_uint = 0x2E5AC;

pub const R_TXPWR_RSTA: c_uint = 0xE60C;

pub const R_TSSI_PWR_P0: c_uint = 0xE610;
pub const R_TSSI_PWR_P1: c_uint = 0xE710;

pub const R_P0_TXPWRB_BE: c_uint = 0xE61C;
pub const R_P1_TXPWRB_BE: c_uint = 0xE71C;
pub const R_P0_TXPWRB_BE4: c_uint = 0x2251C;
pub const R_P1_TXPWRB_BE4: c_uint = 0x2261C;

pub const R_TSSI_MAP_OFST_P0: c_uint = 0xE620;
pub const R_TSSI_MAP_OFST_P1: c_uint = 0xE720;

pub const R_TXAGC_REF_DBM_P0: c_uint = 0xE628;

pub const R_TSSI_K_P0: c_uint = 0xE6A0;

pub const R_TXPWR_RSTB: c_uint = 0xE70C;

pub const R_TXAGC_REF_DBM_P1: c_uint = 0xE728;

pub const R_TSSI_K_P1: c_uint = 0xE7A0;

pub const R_BBWRAP_ELMSR_BE4: c_uint = 0x11974;

pub const R_COMP_CIM3K_BE4: c_uint = 0x11998;

pub const R_DPD_CBW160_BE4: c_uint = 0x119B4;

pub const R_OOB_CBW20_BE4: c_uint = 0x119B4;

pub const R_OOB_CBW40_BE4: c_uint = 0x119B8;

pub const R_OOB_CBW80_BE4: c_uint = 0x119BC;

pub const R_DPD_DBW160_TH0_BE4: c_uint = 0x119BC;

pub const R_DPD_DBW160_TH1_BE4: c_uint = 0x119C0;

pub const R_DPD_CBW_TH0_BE4: c_uint = 0x119C0;

pub const R_DPD_CBW_TH1_BE4: c_uint = 0x119C4;

pub const R_DPD_CBW_TH2_BE4: c_uint = 0x119C8;

pub const R_QAM_TH0_BE4: c_uint = 0x119E4;

pub const R_QAM_TH1_BE4: c_uint = 0x119E8;

pub const R_QAM_TH2_BE4: c_uint = 0x119EC;

pub const R_RFSI_CT_DEF_BE4: c_uint = 0x119F0;

pub const R_FBTB_CT_DEF_BE4: c_uint = 0x119F4;

pub const R_CIM3K_SU_FORCE: c_uint = 0x119F8;

pub const R_QAM3_TH0_BE4: c_uint = 0x119FC;

pub const R_RFSI_CT_OPT_0_BE4: c_uint = 0x11A94;
pub const R_RFSI_CT_OPT_8_BE4: c_uint = 0x11A98;
pub const R_QAM_COMP_TH0_BE4: c_uint = 0x11A9C;

pub const R_QAM_COMP_TH1_BE4: c_uint = 0x11AA0;
pub const R_QAM_COMP_TH2_BE4: c_uint = 0x11AA4;
pub const R_QAM_COMP_TH3_BE4: c_uint = 0x11AA8;
pub const R_QAM_COMP_TH4_BE4: c_uint = 0x11ABC;

pub const R_QAM_COMP_TH5_BE4: c_uint = 0x11AC0;

pub const R_QAM_COMP_TH6_BE4: c_uint = 0x11AC4;

pub const R_OW_VAL_0_BE4: c_uint = 0x11AAC;

pub const R_OW_VAL_1_BE4: c_uint = 0x11AB0;
pub const R_OW_VAL_2_BE4: c_uint = 0x11AB4;
pub const R_OW_VAL_3_BE4: c_uint = 0x11AB8;
pub const R_BANDEDGE_DBWX_BE4: c_uint = 0x11ACC;

pub const R_BANDEDGE_DBWY_BE4: c_uint = 0x11AD0;

pub const R_QAM3_TH1_BE4: c_uint = 0x11BE8;

pub const R_QAM3_TH2_BE4: c_uint = 0x11BEC;

pub const R_QAM3_TH3_BE4: c_uint = 0x11BF4;

pub const R_QAM3_TH4_BE4: c_uint = 0x11BF8;

pub const R_QAM3_FLTR_BE4: c_uint = 0x11BFC;

pub const R_SYS_DBCC_BE4: c_uint = 0x20000;

pub const R_EMLSR_SWITCH_BE4: c_uint = 0x20044;

pub const R_CHINFO_SEG_BE4: c_uint = 0x200B4;

pub const R_SEL_GNT_BT_RX_BE4: c_uint = 0x2010C;

pub const R_SW_SI_WDATA_BE4: c_uint = 0x20370;

pub const R_SW_SI_READ_ADDR_BE4: c_uint = 0x20378;

pub const R_RXBW67_BE4: c_uint = 0x2040C;

pub const R_RXBW_BE4: c_uint = 0x20410;

pub const R_TXERRCT_EN_BE4: c_uint = 0x20518;

pub const R_TXERRCT1_EN_BE4: c_uint = 0x2051C;

pub const R_ENABLE_CCK0_BE4: c_uint = 0x20700;

pub const R_RSTB_ASYNC_BE4: c_uint = 0x20704;

pub const R_STS_HDR2_PARSING_BE4: c_uint = 0x2070C;

pub const R_EDCCA_RPT_SEL_BE4: c_uint = 0x20780;
pub const R_EDCCA_RPT_SEL_BE4_C1: c_uint = 0x21780;
pub const B_EDCCA_RPT_SEL_BE4_MSK: c_uint = 0xE0000;
pub const R_SEL_GNT_BT_RXPHY_BE4: c_uint = 0x2079C;

pub const R_IMR_TX_ERROR_BE4: c_uint = 0x20920;

pub const R_TXINFO_PATH_BE4: c_uint = 0x209A4;

pub const R_SHAPER_COEFF_BE4: c_uint = 0x20CBC;

pub const R_CL_MODE_CNT_BE4: c_uint = 0x20DE0;

pub const R_IFS_T1_AVG_BE4: c_uint = 0x20EDC;

pub const R_IFS_T3_AVG_BE4: c_uint = 0x20EE0;

pub const R_IFS_T1_CLM_BE4: c_uint = 0x20EE4;

pub const R_IFS_T3_CLM_BE4: c_uint = 0x20EE8;

pub const R_IFS_TOTAL_BE4: c_uint = 0x20EEC;

pub const R_IFS_T1_HIS_BE4: c_uint = 0x20F50;

pub const R_IFS_T3_HIS_BE4: c_uint = 0x20F54;

pub const R_CNT_CCK_CCA_BE4: c_uint = 0x20FE8;

pub const R_CNT_CCK_CRC32_BE4: c_uint = 0x20FEC;

pub const R_TX_ERROR_SEL_BE4: c_uint = 0x21254;

pub const R_TXPWR_RSTB0_BE4: c_uint = 0x2250C;

pub const R_TSSI_EN_P0_BE4: c_uint = 0x22510;

pub const R_TXAGC_REF_DBM_PATH0_TBL0_BE4: c_uint = 0x22528;

pub const R_USED_TSSI_TRK_ON_P0_BE4: c_uint = 0x22534;

pub const R_TSSI_K_OFDM_PATH0_TBL0_BE4: c_uint = 0x225A0;

pub const R_TSSI_DCK_MOV_AVG_LEN_P0_BE4: c_uint = 0x225CC;

pub const R_TXPWR_RSTB1_BE4: c_uint = 0x2260C;

pub const R_TXAGC_REF_DBM_PATH0_TBL1_BE4: c_uint = 0x23528;

pub const R_TSSI_K_OFDM_PATH0_TBL1_BE4: c_uint = 0x235A0;

pub const R_OFDM_OFST_P0_BE4: c_uint = 0x240C8;

pub const R_PATH0_RXIDX_INIT_BE4: c_uint = 0x24108;

pub const R_PATH0_LNA_INIT_BE4: c_uint = 0x24158;

pub const R_BAND_SEL0_BE4: c_uint = 0x24160;

pub const R_PATH0_TIA_INIT_BE4: c_uint = 0x24168;

pub const R_OFDM_RPL_BIAS_P0_BE4: c_uint = 0x2420C;

pub const R_OFDM_OFST_P1_BE4: c_uint = 0x244C8;

pub const R_PATH1_RXIDX_INIT_BE4: c_uint = 0x24508;

pub const R_PATH1_LNA_INIT_BE4: c_uint = 0x24558;

pub const R_BAND_SEL1_BE4: c_uint = 0x24560;

pub const R_PATH1_TIA_INIT_BE4: c_uint = 0x24568;

pub const R_OFDM_RPL_BIAS_P1_BE4: c_uint = 0x2460C;

pub const R_TX_CFR_MANUAL_EN_BE4: c_uint = 0x2483C;

pub const R_PCOEFF0_BE4: c_uint = 0x24880;

pub const R_PCOEFF2_BE4: c_uint = 0x24884;

pub const R_PCOEFF4_BE4: c_uint = 0x24888;

pub const R_PCOEFF6_BE4: c_uint = 0x2488C;

pub const R_PCOEFF8_BE4: c_uint = 0x24890;

pub const R_PCOEFF10_BE4: c_uint = 0x24894;

pub const R_PCOEFF12_BE4: c_uint = 0x24898;

pub const R_PCOEFF14_BE4: c_uint = 0x2489C;

pub const R_BW_BE4: c_uint = 0x24EE4;

pub const R_FC0_BE4: c_uint = 0x24EE8;

pub const R_ANT_RX_1RCCA_BE4: c_uint = 0x24EEC;

pub const R_ANT_RX_BE4: c_uint = 0x24EF0;

pub const R_FC0_INV_BE4: c_uint = 0x24EF4;

pub const R_CCK_RPL_OFST_BE4: c_uint = 0x26084;

pub const R_BK_FC0_INV_BE4: c_uint = 0x2608C;

pub const R_CCK_FC0_INV_BE4: c_uint = 0x26090;

pub const R_GAIN_BIAS_BE4: c_uint = 0x260A0;

pub const R_BF_SMO_PDP_LMT_EHT_BE4: c_uint = 0x26568;

pub const R_BF_SMO_PDP_LMT_HE_BE4: c_uint = 0x26568;

pub const R_BF_SMO_PDP_LMT_VHT_BE4: c_uint = 0x2656C;

pub const R_AWGN_DET_BE4: c_uint = 0x2668C;

pub const R_CSI_WGT_BE4: c_uint = 0x26770;

pub const R_CHINFO_OPT_BE4: c_uint = 0x267C8;

pub const R_CHINFO_NX_BE4: c_uint = 0x267D0;

pub const R_CHINFO_ALG_BE4: c_uint = 0x267C8;

pub const R_RX_AWGN02_BE4: c_uint = 0x2680C;

pub const R_RX_AWGN00_BE4: c_uint = 0x26814;

pub const R_RX_AWGN01_BE4: c_uint = 0x26818;

pub const R_RXCH_BCC0_BE4: c_uint = 0x26824;

pub const R_RXCH_BCC1_BE4: c_uint = 0x26828;

pub const R_RX_LDPC02_BE4: c_uint = 0x26834;

pub const R_RX_LDPC03_BE4: c_uint = 0x26838;

pub const R_RX_LDPC00_BE4: c_uint = 0x2683C;

pub const R_RX_LDPC01_BE4: c_uint = 0x26840;

pub const R_BSS_CLR_MAP_BE4: c_uint = 0x26914;
pub const R_BSS_CLR_VLD_BE4: c_uint = 0x26920;

pub const R_CL_MODE_TRIG_BE4: c_uint = 0x26F44;

pub const R_SELECTED_TONE_IDX_BE4: c_uint = 0x26F4C;

pub const R_OS_TRIG_BY_SW_BE4: c_uint = 0x26F50;

pub const R_OS_TRIG_SOURCE_BE4: c_uint = 0x26F6C;

pub const R_SW_SI_DATA_BE4: c_uint = 0x2CF4C;

pub const R_TX_INFO_0_0_COMB_BE4: c_uint = 0x2DF00;
pub const R_TX_INFO_0_1_COMB_BE4: c_uint = 0x2DF04;
pub const R_TX_INFO_1_0_COMB_BE4: c_uint = 0x2DF08;
pub const R_TX_INFO_1_1_COMB_BE4: c_uint = 0x2DF0C;
pub const R_TX_INFO_2_0_COMB_BE4: c_uint = 0x2DF10;
pub const R_TX_INFO_2_1_COMB_BE4: c_uint = 0x2DF14;
pub const R_TX_COMMON_CTRL_0_0_COMB_BE4: c_uint = 0x2DF20;
pub const R_TX_COMMON_CTRL_0_1_COMB_BE4: c_uint = 0x2DF24;
pub const R_RX_PATH0_TBL0_BE4: c_uint = 0x2E028;
pub const R_RX_PATH1_TBL0_BE4: c_uint = 0x2E128;
pub const R_KTBL0A_BE4: c_uint = 0x38104;
pub const R_KTBL0B_BE4: c_uint = 0x38204;

pub const R_CFIR_CTRL_A_BE4: c_uint = 0x38124;
pub const R_CFIR_CTRL_B_BE4: c_uint = 0x38224;

pub const R_TX_IQC_A_BE4: c_uint = 0x38138;
pub const R_TX_IQC_B_BE4: c_uint = 0x38238;

pub const R_KTBL1A_BE4: c_uint = 0x38154;
pub const R_KTBL1B_BE4: c_uint = 0x38254;

pub const R_TC_EN_BE4: c_uint = 0x3c200;

pub const R_TC_VAL_BE4: c_uint = 0x3c208;

// WiFi CPU local domain
pub const R_AX_WDT_CTRL: c_uint = 0x0040;

pub const WDT_CTRL_ALL_DIS: c_int = 0;
pub const R_AX_WDT_STATUS: c_uint = 0x0044;

