//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/tg3.h
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
// $Id: tg3.h,v 1.37.2.32 2002/03/11 12:18:18 davem Exp $
// tg3.h: Definitions for Broadcom Tigon3 ethernet driver.
//
// Copyright (C) 2001, 2002, 2003, 2004 David S. Miller (davem@redhat.com)
// Copyright (C) 2001 Jeff Garzik (jgarzik@pobox.com)
// Copyright (C) 2004 Sun Microsystems Inc.
// Copyright (C) 2007-2016 Broadcom Corporation.
// Copyright (C) 2016-2017 Broadcom Limited.
// Copyright (C) 2018 Broadcom. All Rights Reserved. The term "Broadcom"
// refers to Broadcom Inc. and/or its subsidiaries.
//
pub const TG3_64BIT_REG_HIGH: c_uint = 0x00UL;
pub const TG3_64BIT_REG_LOW: c_uint = 0x04UL;
// Descriptor block info.
pub const TG3_BDINFO_HOST_ADDR: c_uint = 0x0UL /* 64-bit */;
pub const TG3_BDINFO_MAXLEN_FLAGS: c_uint = 0x8UL /* 32-bit */;
pub const BDINFO_FLAGS_USE_EXT_RECV: c_uint = 0x00000001 /* ext rx_buffer_desc */;
pub const BDINFO_FLAGS_DISABLED: c_uint = 0x00000002;
pub const BDINFO_FLAGS_MAXLEN_MASK: c_uint = 0xffff0000;
pub const BDINFO_FLAGS_MAXLEN_SHIFT: c_int = 16;
pub const TG3_BDINFO_NIC_ADDR: c_uint = 0xcUL /* 32-bit */;
pub const TG3_BDINFO_SIZE: c_uint = 0x10UL;
pub const TG3_RX_STD_MAX_SIZE_5700: c_int = 512;
pub const TG3_RX_STD_MAX_SIZE_5717: c_int = 2048;
pub const TG3_RX_JMB_MAX_SIZE_5700: c_int = 256;
pub const TG3_RX_JMB_MAX_SIZE_5717: c_int = 1024;
pub const TG3_RX_RET_MAX_SIZE_5700: c_int = 1024;
pub const TG3_RX_RET_MAX_SIZE_5705: c_int = 512;
pub const TG3_RX_RET_MAX_SIZE_5717: c_int = 4096;
pub const TG3_RSS_INDIR_TBL_SIZE: c_int = 128;
// First 256 bytes are a mirror of PCI config space.
pub const TG3PCI_VENDOR: c_uint = 0x00000000;
pub const TG3PCI_VENDOR_BROADCOM: c_uint = 0x14e4;
pub const TG3PCI_DEVICE: c_uint = 0x00000002;
pub const TG3PCI_DEVICE_TIGON3_1: c_uint = 0x1644 /* BCM5700 */;
pub const TG3PCI_DEVICE_TIGON3_2: c_uint = 0x1645 /* BCM5701 */;
pub const TG3PCI_DEVICE_TIGON3_3: c_uint = 0x1646 /* BCM5702 */;
pub const TG3PCI_DEVICE_TIGON3_4: c_uint = 0x1647 /* BCM5703 */;
pub const TG3PCI_DEVICE_TIGON3_5761S: c_uint = 0x1688;
pub const TG3PCI_DEVICE_TIGON3_5761SE: c_uint = 0x1689;
pub const TG3PCI_DEVICE_TIGON3_57780: c_uint = 0x1692;
pub const TG3PCI_DEVICE_TIGON3_5787M: c_uint = 0x1693;
pub const TG3PCI_DEVICE_TIGON3_57760: c_uint = 0x1690;
pub const TG3PCI_DEVICE_TIGON3_57790: c_uint = 0x1694;
pub const TG3PCI_DEVICE_TIGON3_57788: c_uint = 0x1691;
pub const TG3PCI_DEVICE_TIGON3_5785_G: c_uint = 0x1699 /* GPHY */;
pub const TG3PCI_DEVICE_TIGON3_5785_F: c_uint = 0x16a0 /* 10/100 only */;
pub const TG3PCI_DEVICE_TIGON3_5717: c_uint = 0x1655;
pub const TG3PCI_DEVICE_TIGON3_5717_C: c_uint = 0x1665;
pub const TG3PCI_DEVICE_TIGON3_5718: c_uint = 0x1656;
pub const TG3PCI_DEVICE_TIGON3_57781: c_uint = 0x16b1;
pub const TG3PCI_DEVICE_TIGON3_57785: c_uint = 0x16b5;
pub const TG3PCI_DEVICE_TIGON3_57761: c_uint = 0x16b0;
pub const TG3PCI_DEVICE_TIGON3_57765: c_uint = 0x16b4;
pub const TG3PCI_DEVICE_TIGON3_57791: c_uint = 0x16b2;
pub const TG3PCI_DEVICE_TIGON3_57795: c_uint = 0x16b6;
pub const TG3PCI_DEVICE_TIGON3_5719: c_uint = 0x1657;
pub const TG3PCI_DEVICE_TIGON3_5720: c_uint = 0x165f;
pub const TG3PCI_DEVICE_TIGON3_57762: c_uint = 0x1682;
pub const TG3PCI_DEVICE_TIGON3_57766: c_uint = 0x1686;
pub const TG3PCI_DEVICE_TIGON3_57786: c_uint = 0x16b3;
pub const TG3PCI_DEVICE_TIGON3_57782: c_uint = 0x16b7;
pub const TG3PCI_DEVICE_TIGON3_5762: c_uint = 0x1687;
pub const TG3PCI_DEVICE_TIGON3_5725: c_uint = 0x1643;
pub const TG3PCI_DEVICE_TIGON3_5727: c_uint = 0x16f3;
pub const TG3PCI_DEVICE_TIGON3_57764: c_uint = 0x1642;
pub const TG3PCI_DEVICE_TIGON3_57767: c_uint = 0x1683;
pub const TG3PCI_DEVICE_TIGON3_57787: c_uint = 0x1641;
// 0x04 --> 0x2c unused

pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95700A6: c_uint = 0x1644;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95701A5: c_uint = 0x0001;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95700T6: c_uint = 0x0002;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95700A9: c_uint = 0x0003;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95701T1: c_uint = 0x0005;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95701T8: c_uint = 0x0006;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95701A7: c_uint = 0x0007;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95701A10: c_uint = 0x0008;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95701A12: c_uint = 0x8008;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95703AX1: c_uint = 0x0009;
pub const TG3PCI_SUBDEVICE_ID_BROADCOM_95703AX2: c_uint = 0x8009;

pub const TG3PCI_SUBDEVICE_ID_3COM_3C996T: c_uint = 0x1000;
pub const TG3PCI_SUBDEVICE_ID_3COM_3C996BT: c_uint = 0x1006;
pub const TG3PCI_SUBDEVICE_ID_3COM_3C996SX: c_uint = 0x1004;
pub const TG3PCI_SUBDEVICE_ID_3COM_3C1000T: c_uint = 0x1007;
pub const TG3PCI_SUBDEVICE_ID_3COM_3C940BR01: c_uint = 0x1008;

pub const TG3PCI_SUBDEVICE_ID_DELL_VIPER: c_uint = 0x00d1;
pub const TG3PCI_SUBDEVICE_ID_DELL_JAGUAR: c_uint = 0x0106;
pub const TG3PCI_SUBDEVICE_ID_DELL_MERLOT: c_uint = 0x0109;
pub const TG3PCI_SUBDEVICE_ID_DELL_SLIM_MERLOT: c_uint = 0x010a;
pub const TG3PCI_SUBDEVICE_ID_DELL_5762: c_uint = 0x07f0;

pub const TG3PCI_SUBDEVICE_ID_COMPAQ_BANSHEE: c_uint = 0x007c;
pub const TG3PCI_SUBDEVICE_ID_COMPAQ_BANSHEE_2: c_uint = 0x009a;
pub const TG3PCI_SUBDEVICE_ID_COMPAQ_CHANGELING: c_uint = 0x007d;
pub const TG3PCI_SUBDEVICE_ID_COMPAQ_NC7780: c_uint = 0x0085;
pub const TG3PCI_SUBDEVICE_ID_COMPAQ_NC7780_2: c_uint = 0x0099;

pub const TG3PCI_SUBDEVICE_ID_IBM_5703SAX2: c_uint = 0x0281;
pub const TG3PCI_SUBDEVICE_ID_ACER_57780_A: c_uint = 0x0601;
pub const TG3PCI_SUBDEVICE_ID_ACER_57780_B: c_uint = 0x0612;
pub const TG3PCI_SUBDEVICE_ID_LENOVO_5787M: c_uint = 0x3056;
// 0x30 --> 0x64 unused
pub const TG3PCI_MSI_DATA: c_uint = 0x00000064;
// 0x66 --> 0x68 unused
pub const TG3PCI_MISC_HOST_CTRL: c_uint = 0x00000068;
pub const MISC_HOST_CTRL_CLEAR_INT: c_uint = 0x00000001;
pub const MISC_HOST_CTRL_MASK_PCI_INT: c_uint = 0x00000002;
pub const MISC_HOST_CTRL_BYTE_SWAP: c_uint = 0x00000004;
pub const MISC_HOST_CTRL_WORD_SWAP: c_uint = 0x00000008;
pub const MISC_HOST_CTRL_PCISTATE_RW: c_uint = 0x00000010;
pub const MISC_HOST_CTRL_CLKREG_RW: c_uint = 0x00000020;
pub const MISC_HOST_CTRL_REGWORD_SWAP: c_uint = 0x00000040;
pub const MISC_HOST_CTRL_INDIR_ACCESS: c_uint = 0x00000080;
pub const MISC_HOST_CTRL_IRQ_MASK_MODE: c_uint = 0x00000100;
pub const MISC_HOST_CTRL_TAGGED_STATUS: c_uint = 0x00000200;
pub const MISC_HOST_CTRL_CHIPREV: c_uint = 0xffff0000;
pub const MISC_HOST_CTRL_CHIPREV_SHIFT: c_int = 16;
pub const CHIPREV_ID_5700_A0: c_uint = 0x7000;
pub const CHIPREV_ID_5700_A1: c_uint = 0x7001;
pub const CHIPREV_ID_5700_B0: c_uint = 0x7100;
pub const CHIPREV_ID_5700_B1: c_uint = 0x7101;
pub const CHIPREV_ID_5700_B3: c_uint = 0x7102;
pub const CHIPREV_ID_5700_ALTIMA: c_uint = 0x7104;
pub const CHIPREV_ID_5700_C0: c_uint = 0x7200;
pub const CHIPREV_ID_5701_A0: c_uint = 0x0000;
pub const CHIPREV_ID_5701_B0: c_uint = 0x0100;
pub const CHIPREV_ID_5701_B2: c_uint = 0x0102;
pub const CHIPREV_ID_5701_B5: c_uint = 0x0105;
pub const CHIPREV_ID_5703_A0: c_uint = 0x1000;
pub const CHIPREV_ID_5703_A1: c_uint = 0x1001;
pub const CHIPREV_ID_5703_A2: c_uint = 0x1002;
pub const CHIPREV_ID_5703_A3: c_uint = 0x1003;
pub const CHIPREV_ID_5704_A0: c_uint = 0x2000;
pub const CHIPREV_ID_5704_A1: c_uint = 0x2001;
pub const CHIPREV_ID_5704_A2: c_uint = 0x2002;
pub const CHIPREV_ID_5704_A3: c_uint = 0x2003;
pub const CHIPREV_ID_5705_A0: c_uint = 0x3000;
pub const CHIPREV_ID_5705_A1: c_uint = 0x3001;
pub const CHIPREV_ID_5705_A2: c_uint = 0x3002;
pub const CHIPREV_ID_5705_A3: c_uint = 0x3003;
pub const CHIPREV_ID_5750_A0: c_uint = 0x4000;
pub const CHIPREV_ID_5750_A1: c_uint = 0x4001;
pub const CHIPREV_ID_5750_A3: c_uint = 0x4003;
pub const CHIPREV_ID_5750_C2: c_uint = 0x4202;
pub const CHIPREV_ID_5752_A0_HW: c_uint = 0x5000;
pub const CHIPREV_ID_5752_A0: c_uint = 0x6000;
pub const CHIPREV_ID_5752_A1: c_uint = 0x6001;
pub const CHIPREV_ID_5714_A2: c_uint = 0x9002;
pub const CHIPREV_ID_5906_A1: c_uint = 0xc001;
pub const CHIPREV_ID_57780_A0: c_uint = 0x57780000;
pub const CHIPREV_ID_57780_A1: c_uint = 0x57780001;
pub const CHIPREV_ID_5717_A0: c_uint = 0x05717000;
pub const CHIPREV_ID_5717_C0: c_uint = 0x05717200;
pub const CHIPREV_ID_57765_A0: c_uint = 0x57785000;
pub const CHIPREV_ID_5719_A0: c_uint = 0x05719000;
pub const CHIPREV_ID_5720_A0: c_uint = 0x05720000;
pub const CHIPREV_ID_5762_A0: c_uint = 0x05762000;
pub const ASIC_REV_5700: c_uint = 0x07;
pub const ASIC_REV_5701: c_uint = 0x00;
pub const ASIC_REV_5703: c_uint = 0x01;
pub const ASIC_REV_5704: c_uint = 0x02;
pub const ASIC_REV_5705: c_uint = 0x03;
pub const ASIC_REV_5750: c_uint = 0x04;
pub const ASIC_REV_5752: c_uint = 0x06;
pub const ASIC_REV_5780: c_uint = 0x08;
pub const ASIC_REV_5714: c_uint = 0x09;
pub const ASIC_REV_5755: c_uint = 0x0a;
pub const ASIC_REV_5787: c_uint = 0x0b;
pub const ASIC_REV_5906: c_uint = 0x0c;
pub const ASIC_REV_USE_PROD_ID_REG: c_uint = 0x0f;
pub const ASIC_REV_5784: c_uint = 0x5784;
pub const ASIC_REV_5761: c_uint = 0x5761;
pub const ASIC_REV_5785: c_uint = 0x5785;
pub const ASIC_REV_57780: c_uint = 0x57780;
pub const ASIC_REV_5717: c_uint = 0x5717;
pub const ASIC_REV_57765: c_uint = 0x57785;
pub const ASIC_REV_5719: c_uint = 0x5719;
pub const ASIC_REV_5720: c_uint = 0x5720;
pub const ASIC_REV_57766: c_uint = 0x57766;
pub const ASIC_REV_5762: c_uint = 0x5762;
pub const CHIPREV_5700_AX: c_uint = 0x70;
pub const CHIPREV_5700_BX: c_uint = 0x71;
pub const CHIPREV_5700_CX: c_uint = 0x72;
pub const CHIPREV_5701_AX: c_uint = 0x00;
pub const CHIPREV_5703_AX: c_uint = 0x10;
pub const CHIPREV_5704_AX: c_uint = 0x20;
pub const CHIPREV_5704_BX: c_uint = 0x21;
pub const CHIPREV_5750_AX: c_uint = 0x40;
pub const CHIPREV_5750_BX: c_uint = 0x41;
pub const CHIPREV_5784_AX: c_uint = 0x57840;
pub const CHIPREV_5761_AX: c_uint = 0x57610;
pub const CHIPREV_57765_AX: c_uint = 0x577650;
pub const METAL_REV_A0: c_uint = 0x00;
pub const METAL_REV_A1: c_uint = 0x01;
pub const METAL_REV_B0: c_uint = 0x00;
pub const METAL_REV_B1: c_uint = 0x01;
pub const METAL_REV_B2: c_uint = 0x02;
pub const TG3PCI_DMA_RW_CTRL: c_uint = 0x0000006c;
pub const DMA_RWCTRL_DIS_CACHE_ALIGNMENT: c_uint = 0x00000001;
pub const DMA_RWCTRL_TAGGED_STAT_WA: c_uint = 0x00000080;
pub const DMA_RWCTRL_CRDRDR_RDMA_MRRS_MSK: c_uint = 0x00000380;
pub const DMA_RWCTRL_READ_BNDRY_MASK: c_uint = 0x00000700;
pub const DMA_RWCTRL_READ_BNDRY_DISAB: c_uint = 0x00000000;
pub const DMA_RWCTRL_READ_BNDRY_16: c_uint = 0x00000100;
pub const DMA_RWCTRL_READ_BNDRY_128_PCIX: c_uint = 0x00000100;
pub const DMA_RWCTRL_READ_BNDRY_32: c_uint = 0x00000200;
pub const DMA_RWCTRL_READ_BNDRY_256_PCIX: c_uint = 0x00000200;
pub const DMA_RWCTRL_READ_BNDRY_64: c_uint = 0x00000300;
pub const DMA_RWCTRL_READ_BNDRY_384_PCIX: c_uint = 0x00000300;
pub const DMA_RWCTRL_READ_BNDRY_128: c_uint = 0x00000400;
pub const DMA_RWCTRL_READ_BNDRY_256: c_uint = 0x00000500;
pub const DMA_RWCTRL_READ_BNDRY_512: c_uint = 0x00000600;
pub const DMA_RWCTRL_READ_BNDRY_1024: c_uint = 0x00000700;
pub const DMA_RWCTRL_WRITE_BNDRY_MASK: c_uint = 0x00003800;
pub const DMA_RWCTRL_WRITE_BNDRY_DISAB: c_uint = 0x00000000;
pub const DMA_RWCTRL_WRITE_BNDRY_16: c_uint = 0x00000800;
pub const DMA_RWCTRL_WRITE_BNDRY_128_PCIX: c_uint = 0x00000800;
pub const DMA_RWCTRL_WRITE_BNDRY_32: c_uint = 0x00001000;
pub const DMA_RWCTRL_WRITE_BNDRY_256_PCIX: c_uint = 0x00001000;
pub const DMA_RWCTRL_WRITE_BNDRY_64: c_uint = 0x00001800;
pub const DMA_RWCTRL_WRITE_BNDRY_384_PCIX: c_uint = 0x00001800;
pub const DMA_RWCTRL_WRITE_BNDRY_128: c_uint = 0x00002000;
pub const DMA_RWCTRL_WRITE_BNDRY_256: c_uint = 0x00002800;
pub const DMA_RWCTRL_WRITE_BNDRY_512: c_uint = 0x00003000;
pub const DMA_RWCTRL_WRITE_BNDRY_1024: c_uint = 0x00003800;
pub const DMA_RWCTRL_ONE_DMA: c_uint = 0x00004000;
pub const DMA_RWCTRL_READ_WATER: c_uint = 0x00070000;
pub const DMA_RWCTRL_READ_WATER_SHIFT: c_int = 16;
pub const DMA_RWCTRL_WRITE_WATER: c_uint = 0x00380000;
pub const DMA_RWCTRL_WRITE_WATER_SHIFT: c_int = 19;
pub const DMA_RWCTRL_USE_MEM_READ_MULT: c_uint = 0x00400000;
pub const DMA_RWCTRL_ASSERT_ALL_BE: c_uint = 0x00800000;
pub const DMA_RWCTRL_PCI_READ_CMD: c_uint = 0x0f000000;
pub const DMA_RWCTRL_PCI_READ_CMD_SHIFT: c_int = 24;
pub const DMA_RWCTRL_PCI_WRITE_CMD: c_uint = 0xf0000000;
pub const DMA_RWCTRL_PCI_WRITE_CMD_SHIFT: c_int = 28;
pub const DMA_RWCTRL_WRITE_BNDRY_64_PCIE: c_uint = 0x10000000;
pub const DMA_RWCTRL_WRITE_BNDRY_128_PCIE: c_uint = 0x30000000;
pub const DMA_RWCTRL_WRITE_BNDRY_DISAB_PCIE: c_uint = 0x70000000;
pub const TG3PCI_PCISTATE: c_uint = 0x00000070;
pub const PCISTATE_FORCE_RESET: c_uint = 0x00000001;
pub const PCISTATE_INT_NOT_ACTIVE: c_uint = 0x00000002;
pub const PCISTATE_CONV_PCI_MODE: c_uint = 0x00000004;
pub const PCISTATE_BUS_SPEED_HIGH: c_uint = 0x00000008;
pub const PCISTATE_BUS_32BIT: c_uint = 0x00000010;
pub const PCISTATE_ROM_ENABLE: c_uint = 0x00000020;
pub const PCISTATE_ROM_RETRY_ENABLE: c_uint = 0x00000040;
pub const PCISTATE_FLAT_VIEW: c_uint = 0x00000100;
pub const PCISTATE_RETRY_SAME_DMA: c_uint = 0x00002000;
pub const PCISTATE_ALLOW_APE_CTLSPC_WR: c_uint = 0x00010000;
pub const PCISTATE_ALLOW_APE_SHMEM_WR: c_uint = 0x00020000;
pub const PCISTATE_ALLOW_APE_PSPACE_WR: c_uint = 0x00040000;
pub const TG3PCI_CLOCK_CTRL: c_uint = 0x00000074;
pub const CLOCK_CTRL_CORECLK_DISABLE: c_uint = 0x00000200;
pub const CLOCK_CTRL_RXCLK_DISABLE: c_uint = 0x00000400;
pub const CLOCK_CTRL_TXCLK_DISABLE: c_uint = 0x00000800;
pub const CLOCK_CTRL_ALTCLK: c_uint = 0x00001000;
pub const CLOCK_CTRL_PWRDOWN_PLL133: c_uint = 0x00008000;
pub const CLOCK_CTRL_44MHZ_CORE: c_uint = 0x00040000;
pub const CLOCK_CTRL_625_CORE: c_uint = 0x00100000;
pub const CLOCK_CTRL_FORCE_CLKRUN: c_uint = 0x00200000;
pub const CLOCK_CTRL_CLKRUN_OENABLE: c_uint = 0x00400000;
pub const CLOCK_CTRL_DELAY_PCI_GRANT: c_uint = 0x80000000;
pub const TG3PCI_REG_BASE_ADDR: c_uint = 0x00000078;
pub const TG3PCI_MEM_WIN_BASE_ADDR: c_uint = 0x0000007c;
pub const TG3PCI_REG_DATA: c_uint = 0x00000080;
pub const TG3PCI_MEM_WIN_DATA: c_uint = 0x00000084;
pub const TG3PCI_MISC_LOCAL_CTRL: c_uint = 0x00000090;
// 0x94 --> 0x98 unused
pub const TG3PCI_STD_RING_PROD_IDX: c_uint = 0x00000098 /* 64-bit */;
pub const TG3PCI_RCV_RET_RING_CON_IDX: c_uint = 0x000000a0 /* 64-bit */;
// 0xa8 --> 0xb8 unused
pub const TG3PCI_DEV_STATUS_CTRL: c_uint = 0x000000b4;
pub const MAX_READ_REQ_SIZE_2048: c_uint = 0x00004000;
pub const MAX_READ_REQ_MASK: c_uint = 0x00007000;
pub const TG3PCI_DUAL_MAC_CTRL: c_uint = 0x000000b8;
pub const DUAL_MAC_CTRL_CH_MASK: c_uint = 0x00000003;
pub const DUAL_MAC_CTRL_ID: c_uint = 0x00000004;
pub const TG3PCI_PRODID_ASICREV: c_uint = 0x000000bc;
pub const PROD_ID_ASIC_REV_MASK: c_uint = 0x0fffffff;
// 0xc0 --> 0xf4 unused
pub const TG3PCI_GEN2_PRODID_ASICREV: c_uint = 0x000000f4;
pub const TG3PCI_GEN15_PRODID_ASICREV: c_uint = 0x000000fc;
// 0xf8 --> 0x200 unused
pub const TG3_CORR_ERR_STAT: c_uint = 0x00000110;
pub const TG3_CORR_ERR_STAT_CLEAR: c_uint = 0xffffffff;
// 0x114 --> 0x200 unused
// Mailbox registers
pub const MAILBOX_INTERRUPT_0: c_uint = 0x00000200 /* 64-bit */;
pub const MAILBOX_INTERRUPT_1: c_uint = 0x00000208 /* 64-bit */;
pub const MAILBOX_INTERRUPT_2: c_uint = 0x00000210 /* 64-bit */;
pub const MAILBOX_INTERRUPT_3: c_uint = 0x00000218 /* 64-bit */;
pub const MAILBOX_GENERAL_0: c_uint = 0x00000220 /* 64-bit */;
pub const MAILBOX_GENERAL_1: c_uint = 0x00000228 /* 64-bit */;
pub const MAILBOX_GENERAL_2: c_uint = 0x00000230 /* 64-bit */;
pub const MAILBOX_GENERAL_3: c_uint = 0x00000238 /* 64-bit */;
pub const MAILBOX_GENERAL_4: c_uint = 0x00000240 /* 64-bit */;
pub const MAILBOX_GENERAL_5: c_uint = 0x00000248 /* 64-bit */;
pub const MAILBOX_GENERAL_6: c_uint = 0x00000250 /* 64-bit */;
pub const MAILBOX_GENERAL_7: c_uint = 0x00000258 /* 64-bit */;
pub const MAILBOX_RELOAD_STAT: c_uint = 0x00000260 /* 64-bit */;
pub const MAILBOX_RCV_STD_PROD_IDX: c_uint = 0x00000268 /* 64-bit */;

pub const MAILBOX_RCV_JUMBO_PROD_IDX: c_uint = 0x00000270 /* 64-bit */;

pub const MAILBOX_RCV_MINI_PROD_IDX: c_uint = 0x00000278 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_0: c_uint = 0x00000280 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_1: c_uint = 0x00000288 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_2: c_uint = 0x00000290 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_3: c_uint = 0x00000298 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_4: c_uint = 0x000002a0 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_5: c_uint = 0x000002a8 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_6: c_uint = 0x000002b0 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_7: c_uint = 0x000002b8 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_8: c_uint = 0x000002c0 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_9: c_uint = 0x000002c8 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_10: c_uint = 0x000002d0 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_11: c_uint = 0x000002d8 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_12: c_uint = 0x000002e0 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_13: c_uint = 0x000002e8 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_14: c_uint = 0x000002f0 /* 64-bit */;
pub const MAILBOX_RCVRET_CON_IDX_15: c_uint = 0x000002f8 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_0: c_uint = 0x00000300 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_1: c_uint = 0x00000308 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_2: c_uint = 0x00000310 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_3: c_uint = 0x00000318 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_4: c_uint = 0x00000320 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_5: c_uint = 0x00000328 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_6: c_uint = 0x00000330 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_7: c_uint = 0x00000338 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_8: c_uint = 0x00000340 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_9: c_uint = 0x00000348 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_10: c_uint = 0x00000350 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_11: c_uint = 0x00000358 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_12: c_uint = 0x00000360 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_13: c_uint = 0x00000368 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_14: c_uint = 0x00000370 /* 64-bit */;
pub const MAILBOX_SNDHOST_PROD_IDX_15: c_uint = 0x00000378 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_0: c_uint = 0x00000380 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_1: c_uint = 0x00000388 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_2: c_uint = 0x00000390 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_3: c_uint = 0x00000398 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_4: c_uint = 0x000003a0 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_5: c_uint = 0x000003a8 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_6: c_uint = 0x000003b0 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_7: c_uint = 0x000003b8 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_8: c_uint = 0x000003c0 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_9: c_uint = 0x000003c8 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_10: c_uint = 0x000003d0 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_11: c_uint = 0x000003d8 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_12: c_uint = 0x000003e0 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_13: c_uint = 0x000003e8 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_14: c_uint = 0x000003f0 /* 64-bit */;
pub const MAILBOX_SNDNIC_PROD_IDX_15: c_uint = 0x000003f8 /* 64-bit */;
// MAC control registers
pub const MAC_MODE: c_uint = 0x00000400;
pub const MAC_MODE_RESET: c_uint = 0x00000001;
pub const MAC_MODE_HALF_DUPLEX: c_uint = 0x00000002;
pub const MAC_MODE_PORT_MODE_MASK: c_uint = 0x0000000c;
pub const MAC_MODE_PORT_MODE_TBI: c_uint = 0x0000000c;
pub const MAC_MODE_PORT_MODE_GMII: c_uint = 0x00000008;
pub const MAC_MODE_PORT_MODE_MII: c_uint = 0x00000004;
pub const MAC_MODE_PORT_MODE_NONE: c_uint = 0x00000000;
pub const MAC_MODE_PORT_INT_LPBACK: c_uint = 0x00000010;
pub const MAC_MODE_TAGGED_MAC_CTRL: c_uint = 0x00000080;
pub const MAC_MODE_TX_BURSTING: c_uint = 0x00000100;
pub const MAC_MODE_MAX_DEFER: c_uint = 0x00000200;
pub const MAC_MODE_LINK_POLARITY: c_uint = 0x00000400;
pub const MAC_MODE_RXSTAT_ENABLE: c_uint = 0x00000800;
pub const MAC_MODE_RXSTAT_CLEAR: c_uint = 0x00001000;
pub const MAC_MODE_RXSTAT_FLUSH: c_uint = 0x00002000;
pub const MAC_MODE_TXSTAT_ENABLE: c_uint = 0x00004000;
pub const MAC_MODE_TXSTAT_CLEAR: c_uint = 0x00008000;
pub const MAC_MODE_TXSTAT_FLUSH: c_uint = 0x00010000;
pub const MAC_MODE_SEND_CONFIGS: c_uint = 0x00020000;
pub const MAC_MODE_MAGIC_PKT_ENABLE: c_uint = 0x00040000;
pub const MAC_MODE_ACPI_ENABLE: c_uint = 0x00080000;
pub const MAC_MODE_MIP_ENABLE: c_uint = 0x00100000;
pub const MAC_MODE_TDE_ENABLE: c_uint = 0x00200000;
pub const MAC_MODE_RDE_ENABLE: c_uint = 0x00400000;
pub const MAC_MODE_FHDE_ENABLE: c_uint = 0x00800000;
pub const MAC_MODE_KEEP_FRAME_IN_WOL: c_uint = 0x01000000;
pub const MAC_MODE_APE_RX_EN: c_uint = 0x08000000;
pub const MAC_MODE_APE_TX_EN: c_uint = 0x10000000;
pub const MAC_STATUS: c_uint = 0x00000404;
pub const MAC_STATUS_PCS_SYNCED: c_uint = 0x00000001;
pub const MAC_STATUS_SIGNAL_DET: c_uint = 0x00000002;
pub const MAC_STATUS_RCVD_CFG: c_uint = 0x00000004;
pub const MAC_STATUS_CFG_CHANGED: c_uint = 0x00000008;
pub const MAC_STATUS_SYNC_CHANGED: c_uint = 0x00000010;
pub const MAC_STATUS_PORT_DEC_ERR: c_uint = 0x00000400;
pub const MAC_STATUS_LNKSTATE_CHANGED: c_uint = 0x00001000;
pub const MAC_STATUS_MI_COMPLETION: c_uint = 0x00400000;
pub const MAC_STATUS_MI_INTERRUPT: c_uint = 0x00800000;
pub const MAC_STATUS_AP_ERROR: c_uint = 0x01000000;
pub const MAC_STATUS_ODI_ERROR: c_uint = 0x02000000;
pub const MAC_STATUS_RXSTAT_OVERRUN: c_uint = 0x04000000;
pub const MAC_STATUS_TXSTAT_OVERRUN: c_uint = 0x08000000;
pub const MAC_EVENT: c_uint = 0x00000408;
pub const MAC_EVENT_PORT_DECODE_ERR: c_uint = 0x00000400;
pub const MAC_EVENT_LNKSTATE_CHANGED: c_uint = 0x00001000;
pub const MAC_EVENT_MI_COMPLETION: c_uint = 0x00400000;
pub const MAC_EVENT_MI_INTERRUPT: c_uint = 0x00800000;
pub const MAC_EVENT_AP_ERROR: c_uint = 0x01000000;
pub const MAC_EVENT_ODI_ERROR: c_uint = 0x02000000;
pub const MAC_EVENT_RXSTAT_OVERRUN: c_uint = 0x04000000;
pub const MAC_EVENT_TXSTAT_OVERRUN: c_uint = 0x08000000;
pub const MAC_LED_CTRL: c_uint = 0x0000040c;
pub const LED_CTRL_LNKLED_OVERRIDE: c_uint = 0x00000001;
pub const LED_CTRL_1000MBPS_ON: c_uint = 0x00000002;
pub const LED_CTRL_100MBPS_ON: c_uint = 0x00000004;
pub const LED_CTRL_10MBPS_ON: c_uint = 0x00000008;
pub const LED_CTRL_TRAFFIC_OVERRIDE: c_uint = 0x00000010;
pub const LED_CTRL_TRAFFIC_BLINK: c_uint = 0x00000020;
pub const LED_CTRL_TRAFFIC_LED: c_uint = 0x00000040;
pub const LED_CTRL_1000MBPS_STATUS: c_uint = 0x00000080;
pub const LED_CTRL_100MBPS_STATUS: c_uint = 0x00000100;
pub const LED_CTRL_10MBPS_STATUS: c_uint = 0x00000200;
pub const LED_CTRL_TRAFFIC_STATUS: c_uint = 0x00000400;
pub const LED_CTRL_MODE_MAC: c_uint = 0x00000000;
pub const LED_CTRL_MODE_PHY_1: c_uint = 0x00000800;
pub const LED_CTRL_MODE_PHY_2: c_uint = 0x00001000;
pub const LED_CTRL_MODE_SHASTA_MAC: c_uint = 0x00002000;
pub const LED_CTRL_MODE_SHARED: c_uint = 0x00004000;
pub const LED_CTRL_MODE_COMBO: c_uint = 0x00008000;
pub const LED_CTRL_BLINK_RATE_MASK: c_uint = 0x7ff80000;
pub const LED_CTRL_BLINK_RATE_SHIFT: c_int = 19;
pub const LED_CTRL_BLINK_PER_OVERRIDE: c_uint = 0x00080000;
pub const LED_CTRL_BLINK_RATE_OVERRIDE: c_uint = 0x80000000;
pub const MAC_ADDR_0_HIGH: c_uint = 0x00000410 /* upper 2 bytes */;
pub const MAC_ADDR_0_LOW: c_uint = 0x00000414 /* lower 4 bytes */;
pub const MAC_ADDR_1_HIGH: c_uint = 0x00000418 /* upper 2 bytes */;
pub const MAC_ADDR_1_LOW: c_uint = 0x0000041c /* lower 4 bytes */;
pub const MAC_ADDR_2_HIGH: c_uint = 0x00000420 /* upper 2 bytes */;
pub const MAC_ADDR_2_LOW: c_uint = 0x00000424 /* lower 4 bytes */;
pub const MAC_ADDR_3_HIGH: c_uint = 0x00000428 /* upper 2 bytes */;
pub const MAC_ADDR_3_LOW: c_uint = 0x0000042c /* lower 4 bytes */;
pub const MAC_ACPI_MBUF_PTR: c_uint = 0x00000430;
pub const MAC_ACPI_LEN_OFFSET: c_uint = 0x00000434;
pub const ACPI_LENOFF_LEN_MASK: c_uint = 0x0000ffff;
pub const ACPI_LENOFF_LEN_SHIFT: c_int = 0;
pub const ACPI_LENOFF_OFF_MASK: c_uint = 0x0fff0000;
pub const ACPI_LENOFF_OFF_SHIFT: c_int = 16;
pub const MAC_TX_BACKOFF_SEED: c_uint = 0x00000438;
pub const TX_BACKOFF_SEED_MASK: c_uint = 0x000003ff;
pub const MAC_RX_MTU_SIZE: c_uint = 0x0000043c;
pub const RX_MTU_SIZE_MASK: c_uint = 0x0000ffff;
pub const MAC_PCS_TEST: c_uint = 0x00000440;
pub const PCS_TEST_PATTERN_MASK: c_uint = 0x000fffff;
pub const PCS_TEST_PATTERN_SHIFT: c_int = 0;
pub const PCS_TEST_ENABLE: c_uint = 0x00100000;
pub const MAC_TX_AUTO_NEG: c_uint = 0x00000444;
pub const TX_AUTO_NEG_MASK: c_uint = 0x0000ffff;
pub const TX_AUTO_NEG_SHIFT: c_int = 0;
pub const MAC_RX_AUTO_NEG: c_uint = 0x00000448;
pub const RX_AUTO_NEG_MASK: c_uint = 0x0000ffff;
pub const RX_AUTO_NEG_SHIFT: c_int = 0;
pub const MAC_MI_COM: c_uint = 0x0000044c;
pub const MI_COM_CMD_MASK: c_uint = 0x0c000000;
pub const MI_COM_CMD_WRITE: c_uint = 0x04000000;
pub const MI_COM_CMD_READ: c_uint = 0x08000000;
pub const MI_COM_READ_FAILED: c_uint = 0x10000000;
pub const MI_COM_START: c_uint = 0x20000000;
pub const MI_COM_BUSY: c_uint = 0x20000000;
pub const MI_COM_PHY_ADDR_MASK: c_uint = 0x03e00000;
pub const MI_COM_PHY_ADDR_SHIFT: c_int = 21;
pub const MI_COM_REG_ADDR_MASK: c_uint = 0x001f0000;
pub const MI_COM_REG_ADDR_SHIFT: c_int = 16;
pub const MI_COM_DATA_MASK: c_uint = 0x0000ffff;
pub const MAC_MI_STAT: c_uint = 0x00000450;
pub const MAC_MI_STAT_LNKSTAT_ATTN_ENAB: c_uint = 0x00000001;
pub const MAC_MI_STAT_10MBPS_MODE: c_uint = 0x00000002;
pub const MAC_MI_MODE: c_uint = 0x00000454;
pub const MAC_MI_MODE_CLK_10MHZ: c_uint = 0x00000001;
pub const MAC_MI_MODE_SHORT_PREAMBLE: c_uint = 0x00000002;
pub const MAC_MI_MODE_AUTO_POLL: c_uint = 0x00000010;
pub const MAC_MI_MODE_500KHZ_CONST: c_uint = 0x00008000;
pub const MAC_MI_MODE_BASE: c_uint = 0x000c0000 /* XXX magic values XXX */;
pub const MAC_AUTO_POLL_STATUS: c_uint = 0x00000458;
pub const MAC_AUTO_POLL_ERROR: c_uint = 0x00000001;
pub const MAC_TX_MODE: c_uint = 0x0000045c;
pub const TX_MODE_RESET: c_uint = 0x00000001;
pub const TX_MODE_ENABLE: c_uint = 0x00000002;
pub const TX_MODE_FLOW_CTRL_ENABLE: c_uint = 0x00000010;
pub const TX_MODE_BIG_BCKOFF_ENABLE: c_uint = 0x00000020;
pub const TX_MODE_LONG_PAUSE_ENABLE: c_uint = 0x00000040;
pub const TX_MODE_MBUF_LOCKUP_FIX: c_uint = 0x00000100;
pub const TX_MODE_JMB_FRM_LEN: c_uint = 0x00400000;
pub const TX_MODE_CNT_DN_MODE: c_uint = 0x00800000;
pub const MAC_TX_STATUS: c_uint = 0x00000460;
pub const TX_STATUS_XOFFED: c_uint = 0x00000001;
pub const TX_STATUS_SENT_XOFF: c_uint = 0x00000002;
pub const TX_STATUS_SENT_XON: c_uint = 0x00000004;
pub const TX_STATUS_LINK_UP: c_uint = 0x00000008;
pub const TX_STATUS_ODI_UNDERRUN: c_uint = 0x00000010;
pub const TX_STATUS_ODI_OVERRUN: c_uint = 0x00000020;
pub const MAC_TX_LENGTHS: c_uint = 0x00000464;
pub const TX_LENGTHS_SLOT_TIME_MASK: c_uint = 0x000000ff;
pub const TX_LENGTHS_SLOT_TIME_SHIFT: c_int = 0;
pub const TX_LENGTHS_IPG_MASK: c_uint = 0x00000f00;
pub const TX_LENGTHS_IPG_SHIFT: c_int = 8;
pub const TX_LENGTHS_IPG_CRS_MASK: c_uint = 0x00003000;
pub const TX_LENGTHS_IPG_CRS_SHIFT: c_int = 12;
pub const TX_LENGTHS_JMB_FRM_LEN_MSK: c_uint = 0x00ff0000;
pub const TX_LENGTHS_CNT_DWN_VAL_MSK: c_uint = 0xff000000;
pub const MAC_RX_MODE: c_uint = 0x00000468;
pub const RX_MODE_RESET: c_uint = 0x00000001;
pub const RX_MODE_ENABLE: c_uint = 0x00000002;
pub const RX_MODE_FLOW_CTRL_ENABLE: c_uint = 0x00000004;
pub const RX_MODE_KEEP_MAC_CTRL: c_uint = 0x00000008;
pub const RX_MODE_KEEP_PAUSE: c_uint = 0x00000010;
pub const RX_MODE_ACCEPT_OVERSIZED: c_uint = 0x00000020;
pub const RX_MODE_ACCEPT_RUNTS: c_uint = 0x00000040;
pub const RX_MODE_LEN_CHECK: c_uint = 0x00000080;
pub const RX_MODE_PROMISC: c_uint = 0x00000100;
pub const RX_MODE_NO_CRC_CHECK: c_uint = 0x00000200;
pub const RX_MODE_KEEP_VLAN_TAG: c_uint = 0x00000400;
pub const RX_MODE_RSS_IPV4_HASH_EN: c_uint = 0x00010000;
pub const RX_MODE_RSS_TCP_IPV4_HASH_EN: c_uint = 0x00020000;
pub const RX_MODE_RSS_IPV6_HASH_EN: c_uint = 0x00040000;
pub const RX_MODE_RSS_TCP_IPV6_HASH_EN: c_uint = 0x00080000;
pub const RX_MODE_RSS_ITBL_HASH_BITS_7: c_uint = 0x00700000;
pub const RX_MODE_RSS_ENABLE: c_uint = 0x00800000;
pub const RX_MODE_IPV6_CSUM_ENABLE: c_uint = 0x01000000;
pub const RX_MODE_IPV4_FRAG_FIX: c_uint = 0x02000000;
pub const MAC_RX_STATUS: c_uint = 0x0000046c;
pub const RX_STATUS_REMOTE_TX_XOFFED: c_uint = 0x00000001;
pub const RX_STATUS_XOFF_RCVD: c_uint = 0x00000002;
pub const RX_STATUS_XON_RCVD: c_uint = 0x00000004;
pub const MAC_HASH_REG_0: c_uint = 0x00000470;
pub const MAC_HASH_REG_1: c_uint = 0x00000474;
pub const MAC_HASH_REG_2: c_uint = 0x00000478;
pub const MAC_HASH_REG_3: c_uint = 0x0000047c;
pub const MAC_RCV_RULE_0: c_uint = 0x00000480;
pub const MAC_RCV_VALUE_0: c_uint = 0x00000484;
pub const MAC_RCV_RULE_1: c_uint = 0x00000488;
pub const MAC_RCV_VALUE_1: c_uint = 0x0000048c;
pub const MAC_RCV_RULE_2: c_uint = 0x00000490;
pub const MAC_RCV_VALUE_2: c_uint = 0x00000494;
pub const MAC_RCV_RULE_3: c_uint = 0x00000498;
pub const MAC_RCV_VALUE_3: c_uint = 0x0000049c;
pub const MAC_RCV_RULE_4: c_uint = 0x000004a0;
pub const MAC_RCV_VALUE_4: c_uint = 0x000004a4;
pub const MAC_RCV_RULE_5: c_uint = 0x000004a8;
pub const MAC_RCV_VALUE_5: c_uint = 0x000004ac;
pub const MAC_RCV_RULE_6: c_uint = 0x000004b0;
pub const MAC_RCV_VALUE_6: c_uint = 0x000004b4;
pub const MAC_RCV_RULE_7: c_uint = 0x000004b8;
pub const MAC_RCV_VALUE_7: c_uint = 0x000004bc;
pub const MAC_RCV_RULE_8: c_uint = 0x000004c0;
pub const MAC_RCV_VALUE_8: c_uint = 0x000004c4;
pub const MAC_RCV_RULE_9: c_uint = 0x000004c8;
pub const MAC_RCV_VALUE_9: c_uint = 0x000004cc;
pub const MAC_RCV_RULE_10: c_uint = 0x000004d0;
pub const MAC_RCV_VALUE_10: c_uint = 0x000004d4;
pub const MAC_RCV_RULE_11: c_uint = 0x000004d8;
pub const MAC_RCV_VALUE_11: c_uint = 0x000004dc;
pub const MAC_RCV_RULE_12: c_uint = 0x000004e0;
pub const MAC_RCV_VALUE_12: c_uint = 0x000004e4;
pub const MAC_RCV_RULE_13: c_uint = 0x000004e8;
pub const MAC_RCV_VALUE_13: c_uint = 0x000004ec;
pub const MAC_RCV_RULE_14: c_uint = 0x000004f0;
pub const MAC_RCV_VALUE_14: c_uint = 0x000004f4;
pub const MAC_RCV_RULE_15: c_uint = 0x000004f8;
pub const MAC_RCV_VALUE_15: c_uint = 0x000004fc;
pub const RCV_RULE_DISABLE_MASK: c_uint = 0x7fffffff;
pub const MAC_RCV_RULE_CFG: c_uint = 0x00000500;
pub const RCV_RULE_CFG_DEFAULT_CLASS: c_uint = 0x00000008;
pub const MAC_LOW_WMARK_MAX_RX_FRAME: c_uint = 0x00000504;
// 0x508 --> 0x520 unused
pub const MAC_HASHREGU_0: c_uint = 0x00000520;
pub const MAC_HASHREGU_1: c_uint = 0x00000524;
pub const MAC_HASHREGU_2: c_uint = 0x00000528;
pub const MAC_HASHREGU_3: c_uint = 0x0000052c;
pub const MAC_EXTADDR_0_HIGH: c_uint = 0x00000530;
pub const MAC_EXTADDR_0_LOW: c_uint = 0x00000534;
pub const MAC_EXTADDR_1_HIGH: c_uint = 0x00000538;
pub const MAC_EXTADDR_1_LOW: c_uint = 0x0000053c;
pub const MAC_EXTADDR_2_HIGH: c_uint = 0x00000540;
pub const MAC_EXTADDR_2_LOW: c_uint = 0x00000544;
pub const MAC_EXTADDR_3_HIGH: c_uint = 0x00000548;
pub const MAC_EXTADDR_3_LOW: c_uint = 0x0000054c;
pub const MAC_EXTADDR_4_HIGH: c_uint = 0x00000550;
pub const MAC_EXTADDR_4_LOW: c_uint = 0x00000554;
pub const MAC_EXTADDR_5_HIGH: c_uint = 0x00000558;
pub const MAC_EXTADDR_5_LOW: c_uint = 0x0000055c;
pub const MAC_EXTADDR_6_HIGH: c_uint = 0x00000560;
pub const MAC_EXTADDR_6_LOW: c_uint = 0x00000564;
pub const MAC_EXTADDR_7_HIGH: c_uint = 0x00000568;
pub const MAC_EXTADDR_7_LOW: c_uint = 0x0000056c;
pub const MAC_EXTADDR_8_HIGH: c_uint = 0x00000570;
pub const MAC_EXTADDR_8_LOW: c_uint = 0x00000574;
pub const MAC_EXTADDR_9_HIGH: c_uint = 0x00000578;
pub const MAC_EXTADDR_9_LOW: c_uint = 0x0000057c;
pub const MAC_EXTADDR_10_HIGH: c_uint = 0x00000580;
pub const MAC_EXTADDR_10_LOW: c_uint = 0x00000584;
pub const MAC_EXTADDR_11_HIGH: c_uint = 0x00000588;
pub const MAC_EXTADDR_11_LOW: c_uint = 0x0000058c;
pub const MAC_SERDES_CFG: c_uint = 0x00000590;
pub const MAC_SERDES_CFG_EDGE_SELECT: c_uint = 0x00001000;
pub const MAC_SERDES_STAT: c_uint = 0x00000594;
// 0x598 --> 0x5a0 unused
pub const MAC_PHYCFG1: c_uint = 0x000005a0;
pub const MAC_PHYCFG1_RGMII_INT: c_uint = 0x00000001;
pub const MAC_PHYCFG1_RXCLK_TO_MASK: c_uint = 0x00001ff0;
pub const MAC_PHYCFG1_RXCLK_TIMEOUT: c_uint = 0x00001000;
pub const MAC_PHYCFG1_TXCLK_TO_MASK: c_uint = 0x01ff0000;
pub const MAC_PHYCFG1_TXCLK_TIMEOUT: c_uint = 0x01000000;
pub const MAC_PHYCFG1_RGMII_EXT_RX_DEC: c_uint = 0x02000000;
pub const MAC_PHYCFG1_RGMII_SND_STAT_EN: c_uint = 0x04000000;
pub const MAC_PHYCFG1_TXC_DRV: c_uint = 0x20000000;
pub const MAC_PHYCFG2: c_uint = 0x000005a4;
pub const MAC_PHYCFG2_INBAND_ENABLE: c_uint = 0x00000001;
pub const MAC_PHYCFG2_EMODE_MASK_MASK: c_uint = 0x000001c0;
pub const MAC_PHYCFG2_EMODE_MASK_AC131: c_uint = 0x000000c0;
pub const MAC_PHYCFG2_EMODE_MASK_50610: c_uint = 0x00000100;
pub const MAC_PHYCFG2_EMODE_MASK_RT8211: c_uint = 0x00000000;
pub const MAC_PHYCFG2_EMODE_MASK_RT8201: c_uint = 0x000001c0;
pub const MAC_PHYCFG2_EMODE_COMP_MASK: c_uint = 0x00000e00;
pub const MAC_PHYCFG2_EMODE_COMP_AC131: c_uint = 0x00000600;
pub const MAC_PHYCFG2_EMODE_COMP_50610: c_uint = 0x00000400;
pub const MAC_PHYCFG2_EMODE_COMP_RT8211: c_uint = 0x00000800;
pub const MAC_PHYCFG2_EMODE_COMP_RT8201: c_uint = 0x00000000;
pub const MAC_PHYCFG2_FMODE_MASK_MASK: c_uint = 0x00007000;
pub const MAC_PHYCFG2_FMODE_MASK_AC131: c_uint = 0x00006000;
pub const MAC_PHYCFG2_FMODE_MASK_50610: c_uint = 0x00004000;
pub const MAC_PHYCFG2_FMODE_MASK_RT8211: c_uint = 0x00000000;
pub const MAC_PHYCFG2_FMODE_MASK_RT8201: c_uint = 0x00007000;
pub const MAC_PHYCFG2_FMODE_COMP_MASK: c_uint = 0x00038000;
pub const MAC_PHYCFG2_FMODE_COMP_AC131: c_uint = 0x00030000;
pub const MAC_PHYCFG2_FMODE_COMP_50610: c_uint = 0x00008000;
pub const MAC_PHYCFG2_FMODE_COMP_RT8211: c_uint = 0x00038000;
pub const MAC_PHYCFG2_FMODE_COMP_RT8201: c_uint = 0x00000000;
pub const MAC_PHYCFG2_GMODE_MASK_MASK: c_uint = 0x001c0000;
pub const MAC_PHYCFG2_GMODE_MASK_AC131: c_uint = 0x001c0000;
pub const MAC_PHYCFG2_GMODE_MASK_50610: c_uint = 0x00100000;
pub const MAC_PHYCFG2_GMODE_MASK_RT8211: c_uint = 0x00000000;
pub const MAC_PHYCFG2_GMODE_MASK_RT8201: c_uint = 0x001c0000;
pub const MAC_PHYCFG2_GMODE_COMP_MASK: c_uint = 0x00e00000;
pub const MAC_PHYCFG2_GMODE_COMP_AC131: c_uint = 0x00e00000;
pub const MAC_PHYCFG2_GMODE_COMP_50610: c_uint = 0x00000000;
pub const MAC_PHYCFG2_GMODE_COMP_RT8211: c_uint = 0x00200000;
pub const MAC_PHYCFG2_GMODE_COMP_RT8201: c_uint = 0x00000000;
pub const MAC_PHYCFG2_ACT_MASK_MASK: c_uint = 0x03000000;
pub const MAC_PHYCFG2_ACT_MASK_AC131: c_uint = 0x03000000;
pub const MAC_PHYCFG2_ACT_MASK_50610: c_uint = 0x01000000;
pub const MAC_PHYCFG2_ACT_MASK_RT8211: c_uint = 0x03000000;
pub const MAC_PHYCFG2_ACT_MASK_RT8201: c_uint = 0x01000000;
pub const MAC_PHYCFG2_ACT_COMP_MASK: c_uint = 0x0c000000;
pub const MAC_PHYCFG2_ACT_COMP_AC131: c_uint = 0x00000000;
pub const MAC_PHYCFG2_ACT_COMP_50610: c_uint = 0x00000000;
pub const MAC_PHYCFG2_ACT_COMP_RT8211: c_uint = 0x00000000;
pub const MAC_PHYCFG2_ACT_COMP_RT8201: c_uint = 0x08000000;
pub const MAC_PHYCFG2_QUAL_MASK_MASK: c_uint = 0x30000000;
pub const MAC_PHYCFG2_QUAL_MASK_AC131: c_uint = 0x30000000;
pub const MAC_PHYCFG2_QUAL_MASK_50610: c_uint = 0x30000000;
pub const MAC_PHYCFG2_QUAL_MASK_RT8211: c_uint = 0x30000000;
pub const MAC_PHYCFG2_QUAL_MASK_RT8201: c_uint = 0x30000000;
pub const MAC_PHYCFG2_QUAL_COMP_MASK: c_uint = 0xc0000000;
pub const MAC_PHYCFG2_QUAL_COMP_AC131: c_uint = 0x00000000;
pub const MAC_PHYCFG2_QUAL_COMP_50610: c_uint = 0x00000000;
pub const MAC_PHYCFG2_QUAL_COMP_RT8211: c_uint = 0x00000000;
pub const MAC_PHYCFG2_QUAL_COMP_RT8201: c_uint = 0x00000000;

pub const MAC_EXT_RGMII_MODE: c_uint = 0x000005a8;
pub const MAC_RGMII_MODE_TX_ENABLE: c_uint = 0x00000001;
pub const MAC_RGMII_MODE_TX_LOWPWR: c_uint = 0x00000002;
pub const MAC_RGMII_MODE_TX_RESET: c_uint = 0x00000004;
pub const MAC_RGMII_MODE_RX_INT_B: c_uint = 0x00000100;
pub const MAC_RGMII_MODE_RX_QUALITY: c_uint = 0x00000200;
pub const MAC_RGMII_MODE_RX_ACTIVITY: c_uint = 0x00000400;
pub const MAC_RGMII_MODE_RX_ENG_DET: c_uint = 0x00000800;
// 0x5ac --> 0x5b0 unused
pub const SERDES_RX_CTRL: c_uint = 0x000005b0	/* 5780/5714 only */;
pub const SERDES_RX_SIG_DETECT: c_uint = 0x00000400;
pub const SG_DIG_CTRL: c_uint = 0x000005b0;
pub const SG_DIG_USING_HW_AUTONEG: c_uint = 0x80000000;
pub const SG_DIG_SOFT_RESET: c_uint = 0x40000000;
pub const SG_DIG_DISABLE_LINKRDY: c_uint = 0x20000000;
pub const SG_DIG_CRC16_CLEAR_N: c_uint = 0x01000000;
pub const SG_DIG_EN10B: c_uint = 0x00800000;
pub const SG_DIG_CLEAR_STATUS: c_uint = 0x00400000;
pub const SG_DIG_LOCAL_DUPLEX_STATUS: c_uint = 0x00200000;
pub const SG_DIG_LOCAL_LINK_STATUS: c_uint = 0x00100000;
pub const SG_DIG_SPEED_STATUS_MASK: c_uint = 0x000c0000;
pub const SG_DIG_SPEED_STATUS_SHIFT: c_int = 18;
pub const SG_DIG_JUMBO_PACKET_DISABLE: c_uint = 0x00020000;
pub const SG_DIG_RESTART_AUTONEG: c_uint = 0x00010000;
pub const SG_DIG_FIBER_MODE: c_uint = 0x00008000;
pub const SG_DIG_REMOTE_FAULT_MASK: c_uint = 0x00006000;
pub const SG_DIG_PAUSE_MASK: c_uint = 0x00001800;
pub const SG_DIG_PAUSE_CAP: c_uint = 0x00000800;
pub const SG_DIG_ASYM_PAUSE: c_uint = 0x00001000;
pub const SG_DIG_GBIC_ENABLE: c_uint = 0x00000400;
pub const SG_DIG_CHECK_END_ENABLE: c_uint = 0x00000200;
pub const SG_DIG_SGMII_AUTONEG_TIMER: c_uint = 0x00000100;
pub const SG_DIG_CLOCK_PHASE_SELECT: c_uint = 0x00000080;
pub const SG_DIG_GMII_INPUT_SELECT: c_uint = 0x00000040;
pub const SG_DIG_MRADV_CRC16_SELECT: c_uint = 0x00000020;
pub const SG_DIG_COMMA_DETECT_ENABLE: c_uint = 0x00000010;
pub const SG_DIG_AUTONEG_TIMER_REDUCE: c_uint = 0x00000008;
pub const SG_DIG_AUTONEG_LOW_ENABLE: c_uint = 0x00000004;
pub const SG_DIG_REMOTE_LOOPBACK: c_uint = 0x00000002;
pub const SG_DIG_LOOPBACK: c_uint = 0x00000001;

pub const SG_DIG_STATUS: c_uint = 0x000005b4;
pub const SG_DIG_CRC16_BUS_MASK: c_uint = 0xffff0000;
pub const SG_DIG_PARTNER_FAULT_MASK: c_uint = 0x00600000 /* If !MRADV_CRC16_SELECT */;
pub const SG_DIG_PARTNER_ASYM_PAUSE: c_uint = 0x00100000 /* If !MRADV_CRC16_SELECT */;
pub const SG_DIG_PARTNER_PAUSE_CAPABLE: c_uint = 0x00080000 /* If !MRADV_CRC16_SELECT */;
pub const SG_DIG_PARTNER_HALF_DUPLEX: c_uint = 0x00040000 /* If !MRADV_CRC16_SELECT */;
pub const SG_DIG_PARTNER_FULL_DUPLEX: c_uint = 0x00020000 /* If !MRADV_CRC16_SELECT */;
pub const SG_DIG_PARTNER_NEXT_PAGE: c_uint = 0x00010000 /* If !MRADV_CRC16_SELECT */;
pub const SG_DIG_AUTONEG_STATE_MASK: c_uint = 0x00000ff0;
pub const SG_DIG_IS_SERDES: c_uint = 0x00000100;
pub const SG_DIG_COMMA_DETECTOR: c_uint = 0x00000008;
pub const SG_DIG_MAC_ACK_STATUS: c_uint = 0x00000004;
pub const SG_DIG_AUTONEG_COMPLETE: c_uint = 0x00000002;
pub const SG_DIG_AUTONEG_ERROR: c_uint = 0x00000001;
pub const TG3_TX_TSTAMP_LSB: c_uint = 0x000005c0;
pub const TG3_TX_TSTAMP_MSB: c_uint = 0x000005c4;
pub const TG3_TSTAMP_MASK: c_uint = 0x7fffffffffffffffLL;
// 0x5c8 --> 0x600 unused
pub const MAC_TX_MAC_STATE_BASE: c_uint = 0x00000600 /* 16 bytes */;
pub const MAC_RX_MAC_STATE_BASE: c_uint = 0x00000610 /* 20 bytes */;
// 0x624 --> 0x670 unused
pub const MAC_RSS_INDIR_TBL_0: c_uint = 0x00000630;
pub const MAC_RSS_HASH_KEY_0: c_uint = 0x00000670;
pub const MAC_RSS_HASH_KEY_1: c_uint = 0x00000674;
pub const MAC_RSS_HASH_KEY_2: c_uint = 0x00000678;
pub const MAC_RSS_HASH_KEY_3: c_uint = 0x0000067c;
pub const MAC_RSS_HASH_KEY_4: c_uint = 0x00000680;
pub const MAC_RSS_HASH_KEY_5: c_uint = 0x00000684;
pub const MAC_RSS_HASH_KEY_6: c_uint = 0x00000688;
pub const MAC_RSS_HASH_KEY_7: c_uint = 0x0000068c;
pub const MAC_RSS_HASH_KEY_8: c_uint = 0x00000690;
pub const MAC_RSS_HASH_KEY_9: c_uint = 0x00000694;
// 0x698 --> 0x6b0 unused
pub const TG3_RX_TSTAMP_LSB: c_uint = 0x000006b0;
pub const TG3_RX_TSTAMP_MSB: c_uint = 0x000006b4;
// 0x6b8 --> 0x6c8 unused
pub const TG3_RX_PTP_CTL: c_uint = 0x000006c8;
pub const TG3_RX_PTP_CTL_SYNC_EVNT: c_uint = 0x00000001;
pub const TG3_RX_PTP_CTL_DELAY_REQ: c_uint = 0x00000002;
pub const TG3_RX_PTP_CTL_PDLAY_REQ: c_uint = 0x00000004;
pub const TG3_RX_PTP_CTL_PDLAY_RES: c_uint = 0x00000008;

pub const TG3_RX_PTP_CTL_FOLLOW_UP: c_uint = 0x00000100;
pub const TG3_RX_PTP_CTL_DELAY_RES: c_uint = 0x00000200;
pub const TG3_RX_PTP_CTL_PDRES_FLW_UP: c_uint = 0x00000400;
pub const TG3_RX_PTP_CTL_ANNOUNCE: c_uint = 0x00000800;
pub const TG3_RX_PTP_CTL_SIGNALING: c_uint = 0x00001000;
pub const TG3_RX_PTP_CTL_MANAGEMENT: c_uint = 0x00002000;
pub const TG3_RX_PTP_CTL_RX_PTP_V2_L2_EN: c_uint = 0x00800000;
pub const TG3_RX_PTP_CTL_RX_PTP_V2_L4_EN: c_uint = 0x01000000;

pub const TG3_RX_PTP_CTL_RX_PTP_V1_EN: c_uint = 0x02000000;
pub const TG3_RX_PTP_CTL_HWTS_INTERLOCK: c_uint = 0x04000000;
// 0x6cc --> 0x800 unused
pub const MAC_TX_STATS_OCTETS: c_uint = 0x00000800;
pub const MAC_TX_STATS_RESV1: c_uint = 0x00000804;
pub const MAC_TX_STATS_COLLISIONS: c_uint = 0x00000808;
pub const MAC_TX_STATS_XON_SENT: c_uint = 0x0000080c;
pub const MAC_TX_STATS_XOFF_SENT: c_uint = 0x00000810;
pub const MAC_TX_STATS_RESV2: c_uint = 0x00000814;
pub const MAC_TX_STATS_MAC_ERRORS: c_uint = 0x00000818;
pub const MAC_TX_STATS_SINGLE_COLLISIONS: c_uint = 0x0000081c;
pub const MAC_TX_STATS_MULT_COLLISIONS: c_uint = 0x00000820;
pub const MAC_TX_STATS_DEFERRED: c_uint = 0x00000824;
pub const MAC_TX_STATS_RESV3: c_uint = 0x00000828;
pub const MAC_TX_STATS_EXCESSIVE_COL: c_uint = 0x0000082c;
pub const MAC_TX_STATS_LATE_COL: c_uint = 0x00000830;
pub const MAC_TX_STATS_RESV4_1: c_uint = 0x00000834;
pub const MAC_TX_STATS_RESV4_2: c_uint = 0x00000838;
pub const MAC_TX_STATS_RESV4_3: c_uint = 0x0000083c;
pub const MAC_TX_STATS_RESV4_4: c_uint = 0x00000840;
pub const MAC_TX_STATS_RESV4_5: c_uint = 0x00000844;
pub const MAC_TX_STATS_RESV4_6: c_uint = 0x00000848;
pub const MAC_TX_STATS_RESV4_7: c_uint = 0x0000084c;
pub const MAC_TX_STATS_RESV4_8: c_uint = 0x00000850;
pub const MAC_TX_STATS_RESV4_9: c_uint = 0x00000854;
pub const MAC_TX_STATS_RESV4_10: c_uint = 0x00000858;
pub const MAC_TX_STATS_RESV4_11: c_uint = 0x0000085c;
pub const MAC_TX_STATS_RESV4_12: c_uint = 0x00000860;
pub const MAC_TX_STATS_RESV4_13: c_uint = 0x00000864;
pub const MAC_TX_STATS_RESV4_14: c_uint = 0x00000868;
pub const MAC_TX_STATS_UCAST: c_uint = 0x0000086c;
pub const MAC_TX_STATS_MCAST: c_uint = 0x00000870;
pub const MAC_TX_STATS_BCAST: c_uint = 0x00000874;
pub const MAC_TX_STATS_RESV5_1: c_uint = 0x00000878;
pub const MAC_TX_STATS_RESV5_2: c_uint = 0x0000087c;
pub const MAC_RX_STATS_OCTETS: c_uint = 0x00000880;
pub const MAC_RX_STATS_RESV1: c_uint = 0x00000884;
pub const MAC_RX_STATS_FRAGMENTS: c_uint = 0x00000888;
pub const MAC_RX_STATS_UCAST: c_uint = 0x0000088c;
pub const MAC_RX_STATS_MCAST: c_uint = 0x00000890;
pub const MAC_RX_STATS_BCAST: c_uint = 0x00000894;
pub const MAC_RX_STATS_FCS_ERRORS: c_uint = 0x00000898;
pub const MAC_RX_STATS_ALIGN_ERRORS: c_uint = 0x0000089c;
pub const MAC_RX_STATS_XON_PAUSE_RECVD: c_uint = 0x000008a0;
pub const MAC_RX_STATS_XOFF_PAUSE_RECVD: c_uint = 0x000008a4;
pub const MAC_RX_STATS_MAC_CTRL_RECVD: c_uint = 0x000008a8;
pub const MAC_RX_STATS_XOFF_ENTERED: c_uint = 0x000008ac;
pub const MAC_RX_STATS_FRAME_TOO_LONG: c_uint = 0x000008b0;
pub const MAC_RX_STATS_JABBERS: c_uint = 0x000008b4;
pub const MAC_RX_STATS_UNDERSIZE: c_uint = 0x000008b8;
// 0x8bc --> 0xc00 unused
// Send data initiator control registers
pub const SNDDATAI_MODE: c_uint = 0x00000c00;
pub const SNDDATAI_MODE_RESET: c_uint = 0x00000001;
pub const SNDDATAI_MODE_ENABLE: c_uint = 0x00000002;
pub const SNDDATAI_MODE_STAT_OFLOW_ENAB: c_uint = 0x00000004;
pub const SNDDATAI_STATUS: c_uint = 0x00000c04;
pub const SNDDATAI_STATUS_STAT_OFLOW: c_uint = 0x00000004;
pub const SNDDATAI_STATSCTRL: c_uint = 0x00000c08;
pub const SNDDATAI_SCTRL_ENABLE: c_uint = 0x00000001;
pub const SNDDATAI_SCTRL_FASTUPD: c_uint = 0x00000002;
pub const SNDDATAI_SCTRL_CLEAR: c_uint = 0x00000004;
pub const SNDDATAI_SCTRL_FLUSH: c_uint = 0x00000008;
pub const SNDDATAI_SCTRL_FORCE_ZERO: c_uint = 0x00000010;
pub const SNDDATAI_STATSENAB: c_uint = 0x00000c0c;
pub const SNDDATAI_STATSINCMASK: c_uint = 0x00000c10;
pub const ISO_PKT_TX: c_uint = 0x00000c20;
// 0xc24 --> 0xc80 unused
pub const SNDDATAI_COS_CNT_0: c_uint = 0x00000c80;
pub const SNDDATAI_COS_CNT_1: c_uint = 0x00000c84;
pub const SNDDATAI_COS_CNT_2: c_uint = 0x00000c88;
pub const SNDDATAI_COS_CNT_3: c_uint = 0x00000c8c;
pub const SNDDATAI_COS_CNT_4: c_uint = 0x00000c90;
pub const SNDDATAI_COS_CNT_5: c_uint = 0x00000c94;
pub const SNDDATAI_COS_CNT_6: c_uint = 0x00000c98;
pub const SNDDATAI_COS_CNT_7: c_uint = 0x00000c9c;
pub const SNDDATAI_COS_CNT_8: c_uint = 0x00000ca0;
pub const SNDDATAI_COS_CNT_9: c_uint = 0x00000ca4;
pub const SNDDATAI_COS_CNT_10: c_uint = 0x00000ca8;
pub const SNDDATAI_COS_CNT_11: c_uint = 0x00000cac;
pub const SNDDATAI_COS_CNT_12: c_uint = 0x00000cb0;
pub const SNDDATAI_COS_CNT_13: c_uint = 0x00000cb4;
pub const SNDDATAI_COS_CNT_14: c_uint = 0x00000cb8;
pub const SNDDATAI_COS_CNT_15: c_uint = 0x00000cbc;
pub const SNDDATAI_DMA_RDQ_FULL_CNT: c_uint = 0x00000cc0;
pub const SNDDATAI_DMA_PRIO_RDQ_FULL_CNT: c_uint = 0x00000cc4;
pub const SNDDATAI_SDCQ_FULL_CNT: c_uint = 0x00000cc8;
pub const SNDDATAI_NICRNG_SSND_PIDX_CNT: c_uint = 0x00000ccc;
pub const SNDDATAI_STATS_UPDATED_CNT: c_uint = 0x00000cd0;
pub const SNDDATAI_INTERRUPTS_CNT: c_uint = 0x00000cd4;
pub const SNDDATAI_AVOID_INTERRUPTS_CNT: c_uint = 0x00000cd8;
pub const SNDDATAI_SND_THRESH_HIT_CNT: c_uint = 0x00000cdc;
// 0xce0 --> 0x1000 unused
// Send data completion control registers
pub const SNDDATAC_MODE: c_uint = 0x00001000;
pub const SNDDATAC_MODE_RESET: c_uint = 0x00000001;
pub const SNDDATAC_MODE_ENABLE: c_uint = 0x00000002;
pub const SNDDATAC_MODE_CDELAY: c_uint = 0x00000010;
// 0x1004 --> 0x1400 unused
// Send BD ring selector
pub const SNDBDS_MODE: c_uint = 0x00001400;
pub const SNDBDS_MODE_RESET: c_uint = 0x00000001;
pub const SNDBDS_MODE_ENABLE: c_uint = 0x00000002;
pub const SNDBDS_MODE_ATTN_ENABLE: c_uint = 0x00000004;
pub const SNDBDS_STATUS: c_uint = 0x00001404;
pub const SNDBDS_STATUS_ERROR_ATTN: c_uint = 0x00000004;
pub const SNDBDS_HWDIAG: c_uint = 0x00001408;
// 0x140c --> 0x1440
pub const SNDBDS_SEL_CON_IDX_0: c_uint = 0x00001440;
pub const SNDBDS_SEL_CON_IDX_1: c_uint = 0x00001444;
pub const SNDBDS_SEL_CON_IDX_2: c_uint = 0x00001448;
pub const SNDBDS_SEL_CON_IDX_3: c_uint = 0x0000144c;
pub const SNDBDS_SEL_CON_IDX_4: c_uint = 0x00001450;
pub const SNDBDS_SEL_CON_IDX_5: c_uint = 0x00001454;
pub const SNDBDS_SEL_CON_IDX_6: c_uint = 0x00001458;
pub const SNDBDS_SEL_CON_IDX_7: c_uint = 0x0000145c;
pub const SNDBDS_SEL_CON_IDX_8: c_uint = 0x00001460;
pub const SNDBDS_SEL_CON_IDX_9: c_uint = 0x00001464;
pub const SNDBDS_SEL_CON_IDX_10: c_uint = 0x00001468;
pub const SNDBDS_SEL_CON_IDX_11: c_uint = 0x0000146c;
pub const SNDBDS_SEL_CON_IDX_12: c_uint = 0x00001470;
pub const SNDBDS_SEL_CON_IDX_13: c_uint = 0x00001474;
pub const SNDBDS_SEL_CON_IDX_14: c_uint = 0x00001478;
pub const SNDBDS_SEL_CON_IDX_15: c_uint = 0x0000147c;
// 0x1480 --> 0x1800 unused
// Send BD initiator control registers
pub const SNDBDI_MODE: c_uint = 0x00001800;
pub const SNDBDI_MODE_RESET: c_uint = 0x00000001;
pub const SNDBDI_MODE_ENABLE: c_uint = 0x00000002;
pub const SNDBDI_MODE_ATTN_ENABLE: c_uint = 0x00000004;
pub const SNDBDI_MODE_MULTI_TXQ_EN: c_uint = 0x00000020;
pub const SNDBDI_STATUS: c_uint = 0x00001804;
pub const SNDBDI_STATUS_ERROR_ATTN: c_uint = 0x00000004;
pub const SNDBDI_IN_PROD_IDX_0: c_uint = 0x00001808;
pub const SNDBDI_IN_PROD_IDX_1: c_uint = 0x0000180c;
pub const SNDBDI_IN_PROD_IDX_2: c_uint = 0x00001810;
pub const SNDBDI_IN_PROD_IDX_3: c_uint = 0x00001814;
pub const SNDBDI_IN_PROD_IDX_4: c_uint = 0x00001818;
pub const SNDBDI_IN_PROD_IDX_5: c_uint = 0x0000181c;
pub const SNDBDI_IN_PROD_IDX_6: c_uint = 0x00001820;
pub const SNDBDI_IN_PROD_IDX_7: c_uint = 0x00001824;
pub const SNDBDI_IN_PROD_IDX_8: c_uint = 0x00001828;
pub const SNDBDI_IN_PROD_IDX_9: c_uint = 0x0000182c;
pub const SNDBDI_IN_PROD_IDX_10: c_uint = 0x00001830;
pub const SNDBDI_IN_PROD_IDX_11: c_uint = 0x00001834;
pub const SNDBDI_IN_PROD_IDX_12: c_uint = 0x00001838;
pub const SNDBDI_IN_PROD_IDX_13: c_uint = 0x0000183c;
pub const SNDBDI_IN_PROD_IDX_14: c_uint = 0x00001840;
pub const SNDBDI_IN_PROD_IDX_15: c_uint = 0x00001844;
// 0x1848 --> 0x1c00 unused
// Send BD completion control registers
pub const SNDBDC_MODE: c_uint = 0x00001c00;
pub const SNDBDC_MODE_RESET: c_uint = 0x00000001;
pub const SNDBDC_MODE_ENABLE: c_uint = 0x00000002;
pub const SNDBDC_MODE_ATTN_ENABLE: c_uint = 0x00000004;
// 0x1c04 --> 0x2000 unused
// Receive list placement control registers
pub const RCVLPC_MODE: c_uint = 0x00002000;
pub const RCVLPC_MODE_RESET: c_uint = 0x00000001;
pub const RCVLPC_MODE_ENABLE: c_uint = 0x00000002;
pub const RCVLPC_MODE_CLASS0_ATTN_ENAB: c_uint = 0x00000004;
pub const RCVLPC_MODE_MAPOOR_AATTN_ENAB: c_uint = 0x00000008;
pub const RCVLPC_MODE_STAT_OFLOW_ENAB: c_uint = 0x00000010;
pub const RCVLPC_STATUS: c_uint = 0x00002004;
pub const RCVLPC_STATUS_CLASS0: c_uint = 0x00000004;
pub const RCVLPC_STATUS_MAPOOR: c_uint = 0x00000008;
pub const RCVLPC_STATUS_STAT_OFLOW: c_uint = 0x00000010;
pub const RCVLPC_LOCK: c_uint = 0x00002008;
pub const RCVLPC_LOCK_REQ_MASK: c_uint = 0x0000ffff;
pub const RCVLPC_LOCK_REQ_SHIFT: c_int = 0;
pub const RCVLPC_LOCK_GRANT_MASK: c_uint = 0xffff0000;
pub const RCVLPC_LOCK_GRANT_SHIFT: c_int = 16;
pub const RCVLPC_NON_EMPTY_BITS: c_uint = 0x0000200c;
pub const RCVLPC_NON_EMPTY_BITS_MASK: c_uint = 0x0000ffff;
pub const RCVLPC_CONFIG: c_uint = 0x00002010;
pub const RCVLPC_STATSCTRL: c_uint = 0x00002014;
pub const RCVLPC_STATSCTRL_ENABLE: c_uint = 0x00000001;
pub const RCVLPC_STATSCTRL_FASTUPD: c_uint = 0x00000002;
pub const RCVLPC_STATS_ENABLE: c_uint = 0x00002018;
pub const RCVLPC_STATSENAB_ASF_FIX: c_uint = 0x00000002;
pub const RCVLPC_STATSENAB_DACK_FIX: c_uint = 0x00040000;
pub const RCVLPC_STATSENAB_LNGBRST_RFIX: c_uint = 0x00400000;
pub const RCVLPC_STATS_INCMASK: c_uint = 0x0000201c;
// 0x2020 --> 0x2100 unused
pub const RCVLPC_SELLST_BASE: c_uint = 0x00002100 /* 16 16-byte entries */;
pub const SELLST_TAIL: c_uint = 0x00000004;
pub const SELLST_CONT: c_uint = 0x00000008;
pub const SELLST_UNUSED: c_uint = 0x0000000c;
pub const RCVLPC_COS_CNTL_BASE: c_uint = 0x00002200 /* 16 4-byte entries */;
pub const RCVLPC_DROP_FILTER_CNT: c_uint = 0x00002240;
pub const RCVLPC_DMA_WQ_FULL_CNT: c_uint = 0x00002244;
pub const RCVLPC_DMA_HIPRIO_WQ_FULL_CNT: c_uint = 0x00002248;
pub const RCVLPC_NO_RCV_BD_CNT: c_uint = 0x0000224c;
pub const RCVLPC_IN_DISCARDS_CNT: c_uint = 0x00002250;
pub const RCVLPC_IN_ERRORS_CNT: c_uint = 0x00002254;
pub const RCVLPC_RCV_THRESH_HIT_CNT: c_uint = 0x00002258;
// 0x225c --> 0x2400 unused
// Receive Data and Receive BD Initiator Control
pub const RCVDBDI_MODE: c_uint = 0x00002400;
pub const RCVDBDI_MODE_RESET: c_uint = 0x00000001;
pub const RCVDBDI_MODE_ENABLE: c_uint = 0x00000002;
pub const RCVDBDI_MODE_JUMBOBD_NEEDED: c_uint = 0x00000004;
pub const RCVDBDI_MODE_FRM_TOO_BIG: c_uint = 0x00000008;
pub const RCVDBDI_MODE_INV_RING_SZ: c_uint = 0x00000010;
pub const RCVDBDI_MODE_LRG_RING_SZ: c_uint = 0x00010000;
pub const RCVDBDI_STATUS: c_uint = 0x00002404;
pub const RCVDBDI_STATUS_JUMBOBD_NEEDED: c_uint = 0x00000004;
pub const RCVDBDI_STATUS_FRM_TOO_BIG: c_uint = 0x00000008;
pub const RCVDBDI_STATUS_INV_RING_SZ: c_uint = 0x00000010;
pub const RCVDBDI_SPLIT_FRAME_MINSZ: c_uint = 0x00002408;
// 0x240c --> 0x2440 unused
pub const RCVDBDI_JUMBO_BD: c_uint = 0x00002440 /* TG3_BDINFO_... */;
pub const RCVDBDI_STD_BD: c_uint = 0x00002450 /* TG3_BDINFO_... */;
pub const RCVDBDI_MINI_BD: c_uint = 0x00002460 /* TG3_BDINFO_... */;
pub const RCVDBDI_JUMBO_CON_IDX: c_uint = 0x00002470;
pub const RCVDBDI_STD_CON_IDX: c_uint = 0x00002474;
pub const RCVDBDI_MINI_CON_IDX: c_uint = 0x00002478;
// 0x247c --> 0x2480 unused
pub const RCVDBDI_BD_PROD_IDX_0: c_uint = 0x00002480;
pub const RCVDBDI_BD_PROD_IDX_1: c_uint = 0x00002484;
pub const RCVDBDI_BD_PROD_IDX_2: c_uint = 0x00002488;
pub const RCVDBDI_BD_PROD_IDX_3: c_uint = 0x0000248c;
pub const RCVDBDI_BD_PROD_IDX_4: c_uint = 0x00002490;
pub const RCVDBDI_BD_PROD_IDX_5: c_uint = 0x00002494;
pub const RCVDBDI_BD_PROD_IDX_6: c_uint = 0x00002498;
pub const RCVDBDI_BD_PROD_IDX_7: c_uint = 0x0000249c;
pub const RCVDBDI_BD_PROD_IDX_8: c_uint = 0x000024a0;
pub const RCVDBDI_BD_PROD_IDX_9: c_uint = 0x000024a4;
pub const RCVDBDI_BD_PROD_IDX_10: c_uint = 0x000024a8;
pub const RCVDBDI_BD_PROD_IDX_11: c_uint = 0x000024ac;
pub const RCVDBDI_BD_PROD_IDX_12: c_uint = 0x000024b0;
pub const RCVDBDI_BD_PROD_IDX_13: c_uint = 0x000024b4;
pub const RCVDBDI_BD_PROD_IDX_14: c_uint = 0x000024b8;
pub const RCVDBDI_BD_PROD_IDX_15: c_uint = 0x000024bc;
pub const RCVDBDI_HWDIAG: c_uint = 0x000024c0;
// 0x24c4 --> 0x2800 unused
// Receive Data Completion Control
pub const RCVDCC_MODE: c_uint = 0x00002800;
pub const RCVDCC_MODE_RESET: c_uint = 0x00000001;
pub const RCVDCC_MODE_ENABLE: c_uint = 0x00000002;
pub const RCVDCC_MODE_ATTN_ENABLE: c_uint = 0x00000004;
// 0x2804 --> 0x2c00 unused
// Receive BD Initiator Control Registers
pub const RCVBDI_MODE: c_uint = 0x00002c00;
pub const RCVBDI_MODE_RESET: c_uint = 0x00000001;
pub const RCVBDI_MODE_ENABLE: c_uint = 0x00000002;
pub const RCVBDI_MODE_RCB_ATTN_ENAB: c_uint = 0x00000004;
pub const RCVBDI_STATUS: c_uint = 0x00002c04;
pub const RCVBDI_STATUS_RCB_ATTN: c_uint = 0x00000004;
pub const RCVBDI_JUMBO_PROD_IDX: c_uint = 0x00002c08;
pub const RCVBDI_STD_PROD_IDX: c_uint = 0x00002c0c;
pub const RCVBDI_MINI_PROD_IDX: c_uint = 0x00002c10;
pub const RCVBDI_MINI_THRESH: c_uint = 0x00002c14;
pub const RCVBDI_STD_THRESH: c_uint = 0x00002c18;
pub const RCVBDI_JUMBO_THRESH: c_uint = 0x00002c1c;
// 0x2c20 --> 0x2d00 unused
pub const STD_REPLENISH_LWM: c_uint = 0x00002d00;
pub const JMB_REPLENISH_LWM: c_uint = 0x00002d04;
// 0x2d08 --> 0x3000 unused
// Receive BD Completion Control Registers
pub const RCVCC_MODE: c_uint = 0x00003000;
pub const RCVCC_MODE_RESET: c_uint = 0x00000001;
pub const RCVCC_MODE_ENABLE: c_uint = 0x00000002;
pub const RCVCC_MODE_ATTN_ENABLE: c_uint = 0x00000004;
pub const RCVCC_STATUS: c_uint = 0x00003004;
pub const RCVCC_STATUS_ERROR_ATTN: c_uint = 0x00000004;
pub const RCVCC_JUMP_PROD_IDX: c_uint = 0x00003008;
pub const RCVCC_STD_PROD_IDX: c_uint = 0x0000300c;
pub const RCVCC_MINI_PROD_IDX: c_uint = 0x00003010;
// 0x3014 --> 0x3400 unused
// Receive list selector control registers
pub const RCVLSC_MODE: c_uint = 0x00003400;
pub const RCVLSC_MODE_RESET: c_uint = 0x00000001;
pub const RCVLSC_MODE_ENABLE: c_uint = 0x00000002;
pub const RCVLSC_MODE_ATTN_ENABLE: c_uint = 0x00000004;
pub const RCVLSC_STATUS: c_uint = 0x00003404;
pub const RCVLSC_STATUS_ERROR_ATTN: c_uint = 0x00000004;
// 0x3408 --> 0x3600 unused
pub const TG3_CPMU_DRV_STATUS: c_uint = 0x0000344c;
// CPMU registers
pub const TG3_CPMU_CTRL: c_uint = 0x00003600;
pub const CPMU_CTRL_LINK_IDLE_MODE: c_uint = 0x00000200;
pub const CPMU_CTRL_LINK_AWARE_MODE: c_uint = 0x00000400;
pub const CPMU_CTRL_LINK_SPEED_MODE: c_uint = 0x00004000;
pub const CPMU_CTRL_GPHY_10MB_RXONLY: c_uint = 0x00010000;
pub const TG3_CPMU_LSPD_10MB_CLK: c_uint = 0x00003604;
pub const CPMU_LSPD_10MB_MACCLK_MASK: c_uint = 0x001f0000;
pub const CPMU_LSPD_10MB_MACCLK_6_25: c_uint = 0x00130000;
// 0x3608 --> 0x360c unused
pub const TG3_CPMU_LSPD_1000MB_CLK: c_uint = 0x0000360c;
pub const CPMU_LSPD_1000MB_MACCLK_62_5: c_uint = 0x00000000;
pub const CPMU_LSPD_1000MB_MACCLK_12_5: c_uint = 0x00110000;
pub const CPMU_LSPD_1000MB_MACCLK_MASK: c_uint = 0x001f0000;
pub const TG3_CPMU_LNK_AWARE_PWRMD: c_uint = 0x00003610;
pub const CPMU_LNK_AWARE_MACCLK_MASK: c_uint = 0x001f0000;
pub const CPMU_LNK_AWARE_MACCLK_6_25: c_uint = 0x00130000;
// 0x3614 --> 0x361c unused
pub const TG3_CPMU_HST_ACC: c_uint = 0x0000361c;
pub const CPMU_HST_ACC_MACCLK_MASK: c_uint = 0x001f0000;
pub const CPMU_HST_ACC_MACCLK_6_25: c_uint = 0x00130000;
// 0x3620 --> 0x3630 unused
pub const TG3_CPMU_CLCK_ORIDE: c_uint = 0x00003624;
pub const CPMU_CLCK_ORIDE_MAC_ORIDE_EN: c_uint = 0x80000000;
pub const TG3_CPMU_CLCK_ORIDE_ENABLE: c_uint = 0x00003628;

pub const TG3_CPMU_STATUS: c_uint = 0x0000362c;
pub const TG3_CPMU_STATUS_FMSK_5717: c_uint = 0x20000000;
pub const TG3_CPMU_STATUS_FMSK_5719: c_uint = 0xc0000000;
pub const TG3_CPMU_STATUS_FSHFT_5719: c_int = 30;
pub const TG3_CPMU_STATUS_LINK_MASK: c_uint = 0x180000;
pub const TG3_CPMU_CLCK_STAT: c_uint = 0x00003630;
pub const CPMU_CLCK_STAT_MAC_CLCK_MASK: c_uint = 0x001f0000;
pub const CPMU_CLCK_STAT_MAC_CLCK_62_5: c_uint = 0x00000000;
pub const CPMU_CLCK_STAT_MAC_CLCK_12_5: c_uint = 0x00110000;
pub const CPMU_CLCK_STAT_MAC_CLCK_6_25: c_uint = 0x00130000;
// 0x3634 --> 0x365c unused
pub const TG3_CPMU_MUTEX_REQ: c_uint = 0x0000365c;
pub const CPMU_MUTEX_REQ_DRIVER: c_uint = 0x00001000;
pub const TG3_CPMU_MUTEX_GNT: c_uint = 0x00003660;
pub const CPMU_MUTEX_GNT_DRIVER: c_uint = 0x00001000;
pub const TG3_CPMU_PHY_STRAP: c_uint = 0x00003664;
pub const TG3_CPMU_PHY_STRAP_IS_SERDES: c_uint = 0x00000020;
pub const TG3_CPMU_PADRNG_CTL: c_uint = 0x00003668;
pub const TG3_CPMU_PADRNG_CTL_RDIV2: c_uint = 0x00040000;
// 0x3664 --> 0x36b0 unused
pub const TG3_CPMU_EEE_MODE: c_uint = 0x000036b0;
pub const TG3_CPMU_EEEMD_APE_TX_DET_EN: c_uint = 0x00000004;
pub const TG3_CPMU_EEEMD_ERLY_L1_XIT_DET: c_uint = 0x00000008;
pub const TG3_CPMU_EEEMD_SND_IDX_DET_EN: c_uint = 0x00000040;
pub const TG3_CPMU_EEEMD_LPI_ENABLE: c_uint = 0x00000080;
pub const TG3_CPMU_EEEMD_LPI_IN_TX: c_uint = 0x00000100;
pub const TG3_CPMU_EEEMD_LPI_IN_RX: c_uint = 0x00000200;
pub const TG3_CPMU_EEEMD_EEE_ENABLE: c_uint = 0x00100000;
pub const TG3_CPMU_EEE_DBTMR1: c_uint = 0x000036b4;
pub const TG3_CPMU_DBTMR1_PCIEXIT_2047US: c_uint = 0x07ff0000;
pub const TG3_CPMU_DBTMR1_LNKIDLE_2047US: c_uint = 0x000007ff;
pub const TG3_CPMU_DBTMR1_LNKIDLE_MAX: c_uint = 0x0000ffff;
pub const TG3_CPMU_EEE_DBTMR2: c_uint = 0x000036b8;
pub const TG3_CPMU_DBTMR2_APE_TX_2047US: c_uint = 0x07ff0000;
pub const TG3_CPMU_DBTMR2_TXIDXEQ_2047US: c_uint = 0x000007ff;
pub const TG3_CPMU_EEE_LNKIDL_CTRL: c_uint = 0x000036bc;
pub const TG3_CPMU_EEE_LNKIDL_PCIE_NL0: c_uint = 0x01000000;
pub const TG3_CPMU_EEE_LNKIDL_UART_IDL: c_uint = 0x00000004;
pub const TG3_CPMU_EEE_LNKIDL_APE_TX_MT: c_uint = 0x00000002;
// 0x36c0 --> 0x36d0 unused
pub const TG3_CPMU_EEE_CTRL: c_uint = 0x000036d0;
pub const TG3_CPMU_EEE_CTRL_EXIT_16_5_US: c_uint = 0x0000019d;
pub const TG3_CPMU_EEE_CTRL_EXIT_36_US: c_uint = 0x00000384;
pub const TG3_CPMU_EEE_CTRL_EXIT_20_1_US: c_uint = 0x000001f8;
// 0x36d4 --> 0x3800 unused
// Mbuf cluster free registers
pub const MBFREE_MODE: c_uint = 0x00003800;
pub const MBFREE_MODE_RESET: c_uint = 0x00000001;
pub const MBFREE_MODE_ENABLE: c_uint = 0x00000002;
pub const MBFREE_STATUS: c_uint = 0x00003804;
// 0x3808 --> 0x3c00 unused
// Host coalescing control registers
pub const HOSTCC_MODE: c_uint = 0x00003c00;
pub const HOSTCC_MODE_RESET: c_uint = 0x00000001;
pub const HOSTCC_MODE_ENABLE: c_uint = 0x00000002;
pub const HOSTCC_MODE_ATTN: c_uint = 0x00000004;
pub const HOSTCC_MODE_NOW: c_uint = 0x00000008;
pub const HOSTCC_MODE_FULL_STATUS: c_uint = 0x00000000;
pub const HOSTCC_MODE_64BYTE: c_uint = 0x00000080;
pub const HOSTCC_MODE_32BYTE: c_uint = 0x00000100;
pub const HOSTCC_MODE_CLRTICK_RXBD: c_uint = 0x00000200;
pub const HOSTCC_MODE_CLRTICK_TXBD: c_uint = 0x00000400;
pub const HOSTCC_MODE_NOINT_ON_NOW: c_uint = 0x00000800;
pub const HOSTCC_MODE_NOINT_ON_FORCE: c_uint = 0x00001000;
pub const HOSTCC_MODE_COAL_VEC1_NOW: c_uint = 0x00002000;
pub const HOSTCC_STATUS: c_uint = 0x00003c04;
pub const HOSTCC_STATUS_ERROR_ATTN: c_uint = 0x00000004;
pub const HOSTCC_RXCOL_TICKS: c_uint = 0x00003c08;
pub const LOW_RXCOL_TICKS: c_uint = 0x00000032;
pub const LOW_RXCOL_TICKS_CLRTCKS: c_uint = 0x00000014;
pub const DEFAULT_RXCOL_TICKS: c_uint = 0x00000048;
pub const HIGH_RXCOL_TICKS: c_uint = 0x00000096;
pub const MAX_RXCOL_TICKS: c_uint = 0x000003ff;
pub const HOSTCC_TXCOL_TICKS: c_uint = 0x00003c0c;
pub const LOW_TXCOL_TICKS: c_uint = 0x00000096;
pub const LOW_TXCOL_TICKS_CLRTCKS: c_uint = 0x00000048;
pub const DEFAULT_TXCOL_TICKS: c_uint = 0x0000012c;
pub const HIGH_TXCOL_TICKS: c_uint = 0x00000145;
pub const MAX_TXCOL_TICKS: c_uint = 0x000003ff;
pub const HOSTCC_RXMAX_FRAMES: c_uint = 0x00003c10;
pub const LOW_RXMAX_FRAMES: c_uint = 0x00000005;
pub const DEFAULT_RXMAX_FRAMES: c_uint = 0x00000008;
pub const HIGH_RXMAX_FRAMES: c_uint = 0x00000012;
pub const MAX_RXMAX_FRAMES: c_uint = 0x000000ff;
pub const HOSTCC_TXMAX_FRAMES: c_uint = 0x00003c14;
pub const LOW_TXMAX_FRAMES: c_uint = 0x00000035;
pub const DEFAULT_TXMAX_FRAMES: c_uint = 0x0000004b;
pub const HIGH_TXMAX_FRAMES: c_uint = 0x00000052;
pub const MAX_TXMAX_FRAMES: c_uint = 0x000000ff;
pub const HOSTCC_RXCOAL_TICK_INT: c_uint = 0x00003c18;
pub const DEFAULT_RXCOAL_TICK_INT: c_uint = 0x00000019;
pub const DEFAULT_RXCOAL_TICK_INT_CLRTCKS: c_uint = 0x00000014;
pub const MAX_RXCOAL_TICK_INT: c_uint = 0x000003ff;
pub const HOSTCC_TXCOAL_TICK_INT: c_uint = 0x00003c1c;
pub const DEFAULT_TXCOAL_TICK_INT: c_uint = 0x00000019;
pub const DEFAULT_TXCOAL_TICK_INT_CLRTCKS: c_uint = 0x00000014;
pub const MAX_TXCOAL_TICK_INT: c_uint = 0x000003ff;
pub const HOSTCC_RXCOAL_MAXF_INT: c_uint = 0x00003c20;
pub const DEFAULT_RXCOAL_MAXF_INT: c_uint = 0x00000005;
pub const MAX_RXCOAL_MAXF_INT: c_uint = 0x000000ff;
pub const HOSTCC_TXCOAL_MAXF_INT: c_uint = 0x00003c24;
pub const DEFAULT_TXCOAL_MAXF_INT: c_uint = 0x00000005;
pub const MAX_TXCOAL_MAXF_INT: c_uint = 0x000000ff;
pub const HOSTCC_STAT_COAL_TICKS: c_uint = 0x00003c28;
pub const DEFAULT_STAT_COAL_TICKS: c_uint = 0x000f4240;
pub const MAX_STAT_COAL_TICKS: c_uint = 0xd693d400;
pub const MIN_STAT_COAL_TICKS: c_uint = 0x00000064;
// 0x3c2c --> 0x3c30 unused
pub const HOSTCC_STATS_BLK_HOST_ADDR: c_uint = 0x00003c30 /* 64-bit */;
pub const HOSTCC_STATUS_BLK_HOST_ADDR: c_uint = 0x00003c38 /* 64-bit */;
pub const HOSTCC_STATS_BLK_NIC_ADDR: c_uint = 0x00003c40;
pub const HOSTCC_STATUS_BLK_NIC_ADDR: c_uint = 0x00003c44;
pub const HOSTCC_FLOW_ATTN: c_uint = 0x00003c48;
pub const HOSTCC_FLOW_ATTN_MBUF_LWM: c_uint = 0x00000040;
// 0x3c4c --> 0x3c50 unused
pub const HOSTCC_JUMBO_CON_IDX: c_uint = 0x00003c50;
pub const HOSTCC_STD_CON_IDX: c_uint = 0x00003c54;
pub const HOSTCC_MINI_CON_IDX: c_uint = 0x00003c58;
// 0x3c5c --> 0x3c80 unused
pub const HOSTCC_RET_PROD_IDX_0: c_uint = 0x00003c80;
pub const HOSTCC_RET_PROD_IDX_1: c_uint = 0x00003c84;
pub const HOSTCC_RET_PROD_IDX_2: c_uint = 0x00003c88;
pub const HOSTCC_RET_PROD_IDX_3: c_uint = 0x00003c8c;
pub const HOSTCC_RET_PROD_IDX_4: c_uint = 0x00003c90;
pub const HOSTCC_RET_PROD_IDX_5: c_uint = 0x00003c94;
pub const HOSTCC_RET_PROD_IDX_6: c_uint = 0x00003c98;
pub const HOSTCC_RET_PROD_IDX_7: c_uint = 0x00003c9c;
pub const HOSTCC_RET_PROD_IDX_8: c_uint = 0x00003ca0;
pub const HOSTCC_RET_PROD_IDX_9: c_uint = 0x00003ca4;
pub const HOSTCC_RET_PROD_IDX_10: c_uint = 0x00003ca8;
pub const HOSTCC_RET_PROD_IDX_11: c_uint = 0x00003cac;
pub const HOSTCC_RET_PROD_IDX_12: c_uint = 0x00003cb0;
pub const HOSTCC_RET_PROD_IDX_13: c_uint = 0x00003cb4;
pub const HOSTCC_RET_PROD_IDX_14: c_uint = 0x00003cb8;
pub const HOSTCC_RET_PROD_IDX_15: c_uint = 0x00003cbc;
pub const HOSTCC_SND_CON_IDX_0: c_uint = 0x00003cc0;
pub const HOSTCC_SND_CON_IDX_1: c_uint = 0x00003cc4;
pub const HOSTCC_SND_CON_IDX_2: c_uint = 0x00003cc8;
pub const HOSTCC_SND_CON_IDX_3: c_uint = 0x00003ccc;
pub const HOSTCC_SND_CON_IDX_4: c_uint = 0x00003cd0;
pub const HOSTCC_SND_CON_IDX_5: c_uint = 0x00003cd4;
pub const HOSTCC_SND_CON_IDX_6: c_uint = 0x00003cd8;
pub const HOSTCC_SND_CON_IDX_7: c_uint = 0x00003cdc;
pub const HOSTCC_SND_CON_IDX_8: c_uint = 0x00003ce0;
pub const HOSTCC_SND_CON_IDX_9: c_uint = 0x00003ce4;
pub const HOSTCC_SND_CON_IDX_10: c_uint = 0x00003ce8;
pub const HOSTCC_SND_CON_IDX_11: c_uint = 0x00003cec;
pub const HOSTCC_SND_CON_IDX_12: c_uint = 0x00003cf0;
pub const HOSTCC_SND_CON_IDX_13: c_uint = 0x00003cf4;
pub const HOSTCC_SND_CON_IDX_14: c_uint = 0x00003cf8;
pub const HOSTCC_SND_CON_IDX_15: c_uint = 0x00003cfc;
pub const HOSTCC_STATBLCK_RING1: c_uint = 0x00003d00;
// 0x3d00 --> 0x3d80 unused
pub const HOSTCC_RXCOL_TICKS_VEC1: c_uint = 0x00003d80;
pub const HOSTCC_TXCOL_TICKS_VEC1: c_uint = 0x00003d84;
pub const HOSTCC_RXMAX_FRAMES_VEC1: c_uint = 0x00003d88;
pub const HOSTCC_TXMAX_FRAMES_VEC1: c_uint = 0x00003d8c;
pub const HOSTCC_RXCOAL_MAXF_INT_VEC1: c_uint = 0x00003d90;
pub const HOSTCC_TXCOAL_MAXF_INT_VEC1: c_uint = 0x00003d94;
// 0x3d98 --> 0x4000 unused
// Memory arbiter control registers
pub const MEMARB_MODE: c_uint = 0x00004000;
pub const MEMARB_MODE_RESET: c_uint = 0x00000001;
pub const MEMARB_MODE_ENABLE: c_uint = 0x00000002;
pub const MEMARB_STATUS: c_uint = 0x00004004;
pub const MEMARB_TRAP_ADDR_LOW: c_uint = 0x00004008;
pub const MEMARB_TRAP_ADDR_HIGH: c_uint = 0x0000400c;
// 0x4010 --> 0x4400 unused
// Buffer manager control registers
pub const BUFMGR_MODE: c_uint = 0x00004400;
pub const BUFMGR_MODE_RESET: c_uint = 0x00000001;
pub const BUFMGR_MODE_ENABLE: c_uint = 0x00000002;
pub const BUFMGR_MODE_ATTN_ENABLE: c_uint = 0x00000004;
pub const BUFMGR_MODE_BM_TEST: c_uint = 0x00000008;
pub const BUFMGR_MODE_MBLOW_ATTN_ENAB: c_uint = 0x00000010;
pub const BUFMGR_MODE_NO_TX_UNDERRUN: c_uint = 0x80000000;
pub const BUFMGR_STATUS: c_uint = 0x00004404;
pub const BUFMGR_STATUS_ERROR: c_uint = 0x00000004;
pub const BUFMGR_STATUS_MBLOW: c_uint = 0x00000010;
pub const BUFMGR_MB_POOL_ADDR: c_uint = 0x00004408;
pub const BUFMGR_MB_POOL_SIZE: c_uint = 0x0000440c;
pub const BUFMGR_MB_RDMA_LOW_WATER: c_uint = 0x00004410;
pub const DEFAULT_MB_RDMA_LOW_WATER: c_uint = 0x00000050;
pub const DEFAULT_MB_RDMA_LOW_WATER_5705: c_uint = 0x00000000;
pub const DEFAULT_MB_RDMA_LOW_WATER_JUMBO: c_uint = 0x00000130;
pub const DEFAULT_MB_RDMA_LOW_WATER_JUMBO_5780: c_uint = 0x00000000;
pub const BUFMGR_MB_MACRX_LOW_WATER: c_uint = 0x00004414;
pub const DEFAULT_MB_MACRX_LOW_WATER: c_uint = 0x00000020;
pub const DEFAULT_MB_MACRX_LOW_WATER_5705: c_uint = 0x00000010;
pub const DEFAULT_MB_MACRX_LOW_WATER_5906: c_uint = 0x00000004;
pub const DEFAULT_MB_MACRX_LOW_WATER_57765: c_uint = 0x0000002a;
pub const DEFAULT_MB_MACRX_LOW_WATER_JUMBO: c_uint = 0x00000098;
pub const DEFAULT_MB_MACRX_LOW_WATER_JUMBO_5780: c_uint = 0x0000004b;
pub const DEFAULT_MB_MACRX_LOW_WATER_JUMBO_57765: c_uint = 0x0000007e;
pub const BUFMGR_MB_HIGH_WATER: c_uint = 0x00004418;
pub const DEFAULT_MB_HIGH_WATER: c_uint = 0x00000060;
pub const DEFAULT_MB_HIGH_WATER_5705: c_uint = 0x00000060;
pub const DEFAULT_MB_HIGH_WATER_5906: c_uint = 0x00000010;
pub const DEFAULT_MB_HIGH_WATER_57765: c_uint = 0x000000a0;
pub const DEFAULT_MB_HIGH_WATER_JUMBO: c_uint = 0x0000017c;
pub const DEFAULT_MB_HIGH_WATER_JUMBO_5780: c_uint = 0x00000096;
pub const DEFAULT_MB_HIGH_WATER_JUMBO_57765: c_uint = 0x000000ea;
pub const BUFMGR_RX_MB_ALLOC_REQ: c_uint = 0x0000441c;
pub const BUFMGR_MB_ALLOC_BIT: c_uint = 0x10000000;
pub const BUFMGR_RX_MB_ALLOC_RESP: c_uint = 0x00004420;
pub const BUFMGR_TX_MB_ALLOC_REQ: c_uint = 0x00004424;
pub const BUFMGR_TX_MB_ALLOC_RESP: c_uint = 0x00004428;
pub const BUFMGR_DMA_DESC_POOL_ADDR: c_uint = 0x0000442c;
pub const BUFMGR_DMA_DESC_POOL_SIZE: c_uint = 0x00004430;
pub const BUFMGR_DMA_LOW_WATER: c_uint = 0x00004434;
pub const DEFAULT_DMA_LOW_WATER: c_uint = 0x00000005;
pub const BUFMGR_DMA_HIGH_WATER: c_uint = 0x00004438;
pub const DEFAULT_DMA_HIGH_WATER: c_uint = 0x0000000a;
pub const BUFMGR_RX_DMA_ALLOC_REQ: c_uint = 0x0000443c;
pub const BUFMGR_RX_DMA_ALLOC_RESP: c_uint = 0x00004440;
pub const BUFMGR_TX_DMA_ALLOC_REQ: c_uint = 0x00004444;
pub const BUFMGR_TX_DMA_ALLOC_RESP: c_uint = 0x00004448;
pub const BUFMGR_HWDIAG_0: c_uint = 0x0000444c;
pub const BUFMGR_HWDIAG_1: c_uint = 0x00004450;
pub const BUFMGR_HWDIAG_2: c_uint = 0x00004454;
// 0x4458 --> 0x4800 unused
// Read DMA control registers
pub const RDMAC_MODE: c_uint = 0x00004800;
pub const RDMAC_MODE_RESET: c_uint = 0x00000001;
pub const RDMAC_MODE_ENABLE: c_uint = 0x00000002;
pub const RDMAC_MODE_TGTABORT_ENAB: c_uint = 0x00000004;
pub const RDMAC_MODE_MSTABORT_ENAB: c_uint = 0x00000008;
pub const RDMAC_MODE_PARITYERR_ENAB: c_uint = 0x00000010;
pub const RDMAC_MODE_ADDROFLOW_ENAB: c_uint = 0x00000020;
pub const RDMAC_MODE_FIFOOFLOW_ENAB: c_uint = 0x00000040;
pub const RDMAC_MODE_FIFOURUN_ENAB: c_uint = 0x00000080;
pub const RDMAC_MODE_FIFOOREAD_ENAB: c_uint = 0x00000100;
pub const RDMAC_MODE_LNGREAD_ENAB: c_uint = 0x00000200;
pub const RDMAC_MODE_SPLIT_ENABLE: c_uint = 0x00000800;
pub const RDMAC_MODE_BD_SBD_CRPT_ENAB: c_uint = 0x00000800;
pub const RDMAC_MODE_SPLIT_RESET: c_uint = 0x00001000;
pub const RDMAC_MODE_MBUF_RBD_CRPT_ENAB: c_uint = 0x00001000;
pub const RDMAC_MODE_MBUF_SBD_CRPT_ENAB: c_uint = 0x00002000;
pub const RDMAC_MODE_FIFO_SIZE_128: c_uint = 0x00020000;
pub const RDMAC_MODE_FIFO_LONG_BURST: c_uint = 0x00030000;
pub const RDMAC_MODE_JMB_2K_MMRR: c_uint = 0x00800000;
pub const RDMAC_MODE_MULT_DMA_RD_DIS: c_uint = 0x01000000;
pub const RDMAC_MODE_IPV4_LSO_EN: c_uint = 0x08000000;
pub const RDMAC_MODE_IPV6_LSO_EN: c_uint = 0x10000000;
pub const RDMAC_MODE_H2BNC_VLAN_DET: c_uint = 0x20000000;
pub const RDMAC_STATUS: c_uint = 0x00004804;
pub const RDMAC_STATUS_TGTABORT: c_uint = 0x00000004;
pub const RDMAC_STATUS_MSTABORT: c_uint = 0x00000008;
pub const RDMAC_STATUS_PARITYERR: c_uint = 0x00000010;
pub const RDMAC_STATUS_ADDROFLOW: c_uint = 0x00000020;
pub const RDMAC_STATUS_FIFOOFLOW: c_uint = 0x00000040;
pub const RDMAC_STATUS_FIFOURUN: c_uint = 0x00000080;
pub const RDMAC_STATUS_FIFOOREAD: c_uint = 0x00000100;
pub const RDMAC_STATUS_LNGREAD: c_uint = 0x00000200;
// 0x4808 --> 0x4890 unused
pub const TG3_RDMA_RSRVCTRL_REG2: c_uint = 0x00004890;
pub const TG3_LSO_RD_DMA_CRPTEN_CTRL2: c_uint = 0x000048a0;
pub const TG3_RDMA_RSRVCTRL_REG: c_uint = 0x00004900;
pub const TG3_RDMA_RSRVCTRL_FIFO_OFLW_FIX: c_uint = 0x00000004;
pub const TG3_RDMA_RSRVCTRL_FIFO_LWM_1_5K: c_uint = 0x00000c00;
pub const TG3_RDMA_RSRVCTRL_FIFO_LWM_MASK: c_uint = 0x00000ff0;
pub const TG3_RDMA_RSRVCTRL_FIFO_HWM_1_5K: c_uint = 0x000c0000;
pub const TG3_RDMA_RSRVCTRL_FIFO_HWM_MASK: c_uint = 0x000ff000;
pub const TG3_RDMA_RSRVCTRL_TXMRGN_320B: c_uint = 0x28000000;
pub const TG3_RDMA_RSRVCTRL_TXMRGN_MASK: c_uint = 0xffe00000;
// 0x4904 --> 0x4910 unused
pub const TG3_LSO_RD_DMA_CRPTEN_CTRL: c_uint = 0x00004910;
pub const TG3_LSO_RD_DMA_CRPTEN_CTRL_BLEN_BD_4K: c_uint = 0x00030000;
pub const TG3_LSO_RD_DMA_CRPTEN_CTRL_BLEN_LSO_4K: c_uint = 0x000c0000;
pub const TG3_LSO_RD_DMA_TX_LENGTH_WA_5719: c_uint = 0x02000000;
pub const TG3_LSO_RD_DMA_TX_LENGTH_WA_5720: c_uint = 0x00200000;
// 0x4914 --> 0x4be0 unused
pub const TG3_NUM_RDMA_CHANNELS: c_int = 4;
pub const TG3_RDMA_LENGTH: c_uint = 0x00004be0;
// Write DMA control registers
pub const WDMAC_MODE: c_uint = 0x00004c00;
pub const WDMAC_MODE_RESET: c_uint = 0x00000001;
pub const WDMAC_MODE_ENABLE: c_uint = 0x00000002;
pub const WDMAC_MODE_TGTABORT_ENAB: c_uint = 0x00000004;
pub const WDMAC_MODE_MSTABORT_ENAB: c_uint = 0x00000008;
pub const WDMAC_MODE_PARITYERR_ENAB: c_uint = 0x00000010;
pub const WDMAC_MODE_ADDROFLOW_ENAB: c_uint = 0x00000020;
pub const WDMAC_MODE_FIFOOFLOW_ENAB: c_uint = 0x00000040;
pub const WDMAC_MODE_FIFOURUN_ENAB: c_uint = 0x00000080;
pub const WDMAC_MODE_FIFOOREAD_ENAB: c_uint = 0x00000100;
pub const WDMAC_MODE_LNGREAD_ENAB: c_uint = 0x00000200;
pub const WDMAC_MODE_RX_ACCEL: c_uint = 0x00000400;
pub const WDMAC_MODE_STATUS_TAG_FIX: c_uint = 0x20000000;
pub const WDMAC_MODE_BURST_ALL_DATA: c_uint = 0xc0000000;
pub const WDMAC_STATUS: c_uint = 0x00004c04;
pub const WDMAC_STATUS_TGTABORT: c_uint = 0x00000004;
pub const WDMAC_STATUS_MSTABORT: c_uint = 0x00000008;
pub const WDMAC_STATUS_PARITYERR: c_uint = 0x00000010;
pub const WDMAC_STATUS_ADDROFLOW: c_uint = 0x00000020;
pub const WDMAC_STATUS_FIFOOFLOW: c_uint = 0x00000040;
pub const WDMAC_STATUS_FIFOURUN: c_uint = 0x00000080;
pub const WDMAC_STATUS_FIFOOREAD: c_uint = 0x00000100;
pub const WDMAC_STATUS_LNGREAD: c_uint = 0x00000200;
// 0x4c08 --> 0x5000 unused
// Per-cpu register offsets (arm9)
pub const CPU_MODE: c_uint = 0x00000000;
pub const CPU_MODE_RESET: c_uint = 0x00000001;
pub const CPU_MODE_HALT: c_uint = 0x00000400;
pub const CPU_STATE: c_uint = 0x00000004;
pub const CPU_EVTMASK: c_uint = 0x00000008;
// 0xc --> 0x1c reserved
pub const CPU_PC: c_uint = 0x0000001c;
pub const CPU_INSN: c_uint = 0x00000020;
pub const CPU_SPAD_UFLOW: c_uint = 0x00000024;
pub const CPU_WDOG_CLEAR: c_uint = 0x00000028;
pub const CPU_WDOG_VECTOR: c_uint = 0x0000002c;
pub const CPU_WDOG_PC: c_uint = 0x00000030;
pub const CPU_HW_BP: c_uint = 0x00000034;
// 0x38 --> 0x44 unused
pub const CPU_WDOG_SAVED_STATE: c_uint = 0x00000044;
pub const CPU_LAST_BRANCH_ADDR: c_uint = 0x00000048;
pub const CPU_SPAD_UFLOW_SET: c_uint = 0x0000004c;
// 0x50 --> 0x200 unused
pub const CPU_R0: c_uint = 0x00000200;
pub const CPU_R1: c_uint = 0x00000204;
pub const CPU_R2: c_uint = 0x00000208;
pub const CPU_R3: c_uint = 0x0000020c;
pub const CPU_R4: c_uint = 0x00000210;
pub const CPU_R5: c_uint = 0x00000214;
pub const CPU_R6: c_uint = 0x00000218;
pub const CPU_R7: c_uint = 0x0000021c;
pub const CPU_R8: c_uint = 0x00000220;
pub const CPU_R9: c_uint = 0x00000224;
pub const CPU_R10: c_uint = 0x00000228;
pub const CPU_R11: c_uint = 0x0000022c;
pub const CPU_R12: c_uint = 0x00000230;
pub const CPU_R13: c_uint = 0x00000234;
pub const CPU_R14: c_uint = 0x00000238;
pub const CPU_R15: c_uint = 0x0000023c;
pub const CPU_R16: c_uint = 0x00000240;
pub const CPU_R17: c_uint = 0x00000244;
pub const CPU_R18: c_uint = 0x00000248;
pub const CPU_R19: c_uint = 0x0000024c;
pub const CPU_R20: c_uint = 0x00000250;
pub const CPU_R21: c_uint = 0x00000254;
pub const CPU_R22: c_uint = 0x00000258;
pub const CPU_R23: c_uint = 0x0000025c;
pub const CPU_R24: c_uint = 0x00000260;
pub const CPU_R25: c_uint = 0x00000264;
pub const CPU_R26: c_uint = 0x00000268;
pub const CPU_R27: c_uint = 0x0000026c;
pub const CPU_R28: c_uint = 0x00000270;
pub const CPU_R29: c_uint = 0x00000274;
pub const CPU_R30: c_uint = 0x00000278;
pub const CPU_R31: c_uint = 0x0000027c;
// 0x280 --> 0x400 unused
pub const RX_CPU_BASE: c_uint = 0x00005000;
pub const RX_CPU_MODE: c_uint = 0x00005000;
pub const RX_CPU_STATE: c_uint = 0x00005004;
pub const RX_CPU_PGMCTR: c_uint = 0x0000501c;
pub const RX_CPU_HWBKPT: c_uint = 0x00005034;
pub const TX_CPU_BASE: c_uint = 0x00005400;
pub const TX_CPU_MODE: c_uint = 0x00005400;
pub const TX_CPU_STATE: c_uint = 0x00005404;
pub const TX_CPU_PGMCTR: c_uint = 0x0000541c;
pub const VCPU_STATUS: c_uint = 0x00005100;
pub const VCPU_STATUS_INIT_DONE: c_uint = 0x04000000;
pub const VCPU_STATUS_DRV_RESET: c_uint = 0x08000000;
pub const VCPU_CFGSHDW: c_uint = 0x00005104;
pub const VCPU_CFGSHDW_WOL_ENABLE: c_uint = 0x00000001;
pub const VCPU_CFGSHDW_WOL_MAGPKT: c_uint = 0x00000004;
pub const VCPU_CFGSHDW_ASPM_DBNC: c_uint = 0x00001000;
// Mailboxes
pub const GRCMBOX_BASE: c_uint = 0x00005600;
pub const GRCMBOX_INTERRUPT_0: c_uint = 0x00005800 /* 64-bit */;
pub const GRCMBOX_INTERRUPT_1: c_uint = 0x00005808 /* 64-bit */;
pub const GRCMBOX_INTERRUPT_2: c_uint = 0x00005810 /* 64-bit */;
pub const GRCMBOX_INTERRUPT_3: c_uint = 0x00005818 /* 64-bit */;
pub const GRCMBOX_GENERAL_0: c_uint = 0x00005820 /* 64-bit */;
pub const GRCMBOX_GENERAL_1: c_uint = 0x00005828 /* 64-bit */;
pub const GRCMBOX_GENERAL_2: c_uint = 0x00005830 /* 64-bit */;
pub const GRCMBOX_GENERAL_3: c_uint = 0x00005838 /* 64-bit */;
pub const GRCMBOX_GENERAL_4: c_uint = 0x00005840 /* 64-bit */;
pub const GRCMBOX_GENERAL_5: c_uint = 0x00005848 /* 64-bit */;
pub const GRCMBOX_GENERAL_6: c_uint = 0x00005850 /* 64-bit */;
pub const GRCMBOX_GENERAL_7: c_uint = 0x00005858 /* 64-bit */;
pub const GRCMBOX_RELOAD_STAT: c_uint = 0x00005860 /* 64-bit */;
pub const GRCMBOX_RCVSTD_PROD_IDX: c_uint = 0x00005868 /* 64-bit */;
pub const GRCMBOX_RCVJUMBO_PROD_IDX: c_uint = 0x00005870 /* 64-bit */;
pub const GRCMBOX_RCVMINI_PROD_IDX: c_uint = 0x00005878 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_0: c_uint = 0x00005880 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_1: c_uint = 0x00005888 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_2: c_uint = 0x00005890 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_3: c_uint = 0x00005898 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_4: c_uint = 0x000058a0 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_5: c_uint = 0x000058a8 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_6: c_uint = 0x000058b0 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_7: c_uint = 0x000058b8 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_8: c_uint = 0x000058c0 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_9: c_uint = 0x000058c8 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_10: c_uint = 0x000058d0 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_11: c_uint = 0x000058d8 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_12: c_uint = 0x000058e0 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_13: c_uint = 0x000058e8 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_14: c_uint = 0x000058f0 /* 64-bit */;
pub const GRCMBOX_RCVRET_CON_IDX_15: c_uint = 0x000058f8 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_0: c_uint = 0x00005900 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_1: c_uint = 0x00005908 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_2: c_uint = 0x00005910 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_3: c_uint = 0x00005918 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_4: c_uint = 0x00005920 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_5: c_uint = 0x00005928 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_6: c_uint = 0x00005930 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_7: c_uint = 0x00005938 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_8: c_uint = 0x00005940 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_9: c_uint = 0x00005948 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_10: c_uint = 0x00005950 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_11: c_uint = 0x00005958 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_12: c_uint = 0x00005960 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_13: c_uint = 0x00005968 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_14: c_uint = 0x00005970 /* 64-bit */;
pub const GRCMBOX_SNDHOST_PROD_IDX_15: c_uint = 0x00005978 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_0: c_uint = 0x00005980 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_1: c_uint = 0x00005988 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_2: c_uint = 0x00005990 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_3: c_uint = 0x00005998 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_4: c_uint = 0x000059a0 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_5: c_uint = 0x000059a8 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_6: c_uint = 0x000059b0 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_7: c_uint = 0x000059b8 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_8: c_uint = 0x000059c0 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_9: c_uint = 0x000059c8 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_10: c_uint = 0x000059d0 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_11: c_uint = 0x000059d8 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_12: c_uint = 0x000059e0 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_13: c_uint = 0x000059e8 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_14: c_uint = 0x000059f0 /* 64-bit */;
pub const GRCMBOX_SNDNIC_PROD_IDX_15: c_uint = 0x000059f8 /* 64-bit */;
pub const GRCMBOX_HIGH_PRIO_EV_VECTOR: c_uint = 0x00005a00;
pub const GRCMBOX_HIGH_PRIO_EV_MASK: c_uint = 0x00005a04;
pub const GRCMBOX_LOW_PRIO_EV_VEC: c_uint = 0x00005a08;
pub const GRCMBOX_LOW_PRIO_EV_MASK: c_uint = 0x00005a0c;
// 0x5a10 --> 0x5c00
// Flow Through queues
pub const FTQ_RESET: c_uint = 0x00005c00;
// 0x5c04 --> 0x5c10 unused
pub const FTQ_DMA_NORM_READ_CTL: c_uint = 0x00005c10;
pub const FTQ_DMA_NORM_READ_FULL_CNT: c_uint = 0x00005c14;
pub const FTQ_DMA_NORM_READ_FIFO_ENQDEQ: c_uint = 0x00005c18;
pub const FTQ_DMA_NORM_READ_WRITE_PEEK: c_uint = 0x00005c1c;
pub const FTQ_DMA_HIGH_READ_CTL: c_uint = 0x00005c20;
pub const FTQ_DMA_HIGH_READ_FULL_CNT: c_uint = 0x00005c24;
pub const FTQ_DMA_HIGH_READ_FIFO_ENQDEQ: c_uint = 0x00005c28;
pub const FTQ_DMA_HIGH_READ_WRITE_PEEK: c_uint = 0x00005c2c;
pub const FTQ_DMA_COMP_DISC_CTL: c_uint = 0x00005c30;
pub const FTQ_DMA_COMP_DISC_FULL_CNT: c_uint = 0x00005c34;
pub const FTQ_DMA_COMP_DISC_FIFO_ENQDEQ: c_uint = 0x00005c38;
pub const FTQ_DMA_COMP_DISC_WRITE_PEEK: c_uint = 0x00005c3c;
pub const FTQ_SEND_BD_COMP_CTL: c_uint = 0x00005c40;
pub const FTQ_SEND_BD_COMP_FULL_CNT: c_uint = 0x00005c44;
pub const FTQ_SEND_BD_COMP_FIFO_ENQDEQ: c_uint = 0x00005c48;
pub const FTQ_SEND_BD_COMP_WRITE_PEEK: c_uint = 0x00005c4c;
pub const FTQ_SEND_DATA_INIT_CTL: c_uint = 0x00005c50;
pub const FTQ_SEND_DATA_INIT_FULL_CNT: c_uint = 0x00005c54;
pub const FTQ_SEND_DATA_INIT_FIFO_ENQDEQ: c_uint = 0x00005c58;
pub const FTQ_SEND_DATA_INIT_WRITE_PEEK: c_uint = 0x00005c5c;
pub const FTQ_DMA_NORM_WRITE_CTL: c_uint = 0x00005c60;
pub const FTQ_DMA_NORM_WRITE_FULL_CNT: c_uint = 0x00005c64;
pub const FTQ_DMA_NORM_WRITE_FIFO_ENQDEQ: c_uint = 0x00005c68;
pub const FTQ_DMA_NORM_WRITE_WRITE_PEEK: c_uint = 0x00005c6c;
pub const FTQ_DMA_HIGH_WRITE_CTL: c_uint = 0x00005c70;
pub const FTQ_DMA_HIGH_WRITE_FULL_CNT: c_uint = 0x00005c74;
pub const FTQ_DMA_HIGH_WRITE_FIFO_ENQDEQ: c_uint = 0x00005c78;
pub const FTQ_DMA_HIGH_WRITE_WRITE_PEEK: c_uint = 0x00005c7c;
pub const FTQ_SWTYPE1_CTL: c_uint = 0x00005c80;
pub const FTQ_SWTYPE1_FULL_CNT: c_uint = 0x00005c84;
pub const FTQ_SWTYPE1_FIFO_ENQDEQ: c_uint = 0x00005c88;
pub const FTQ_SWTYPE1_WRITE_PEEK: c_uint = 0x00005c8c;
pub const FTQ_SEND_DATA_COMP_CTL: c_uint = 0x00005c90;
pub const FTQ_SEND_DATA_COMP_FULL_CNT: c_uint = 0x00005c94;
pub const FTQ_SEND_DATA_COMP_FIFO_ENQDEQ: c_uint = 0x00005c98;
pub const FTQ_SEND_DATA_COMP_WRITE_PEEK: c_uint = 0x00005c9c;
pub const FTQ_HOST_COAL_CTL: c_uint = 0x00005ca0;
pub const FTQ_HOST_COAL_FULL_CNT: c_uint = 0x00005ca4;
pub const FTQ_HOST_COAL_FIFO_ENQDEQ: c_uint = 0x00005ca8;
pub const FTQ_HOST_COAL_WRITE_PEEK: c_uint = 0x00005cac;
pub const FTQ_MAC_TX_CTL: c_uint = 0x00005cb0;
pub const FTQ_MAC_TX_FULL_CNT: c_uint = 0x00005cb4;
pub const FTQ_MAC_TX_FIFO_ENQDEQ: c_uint = 0x00005cb8;
pub const FTQ_MAC_TX_WRITE_PEEK: c_uint = 0x00005cbc;
pub const FTQ_MB_FREE_CTL: c_uint = 0x00005cc0;
pub const FTQ_MB_FREE_FULL_CNT: c_uint = 0x00005cc4;
pub const FTQ_MB_FREE_FIFO_ENQDEQ: c_uint = 0x00005cc8;
pub const FTQ_MB_FREE_WRITE_PEEK: c_uint = 0x00005ccc;
pub const FTQ_RCVBD_COMP_CTL: c_uint = 0x00005cd0;
pub const FTQ_RCVBD_COMP_FULL_CNT: c_uint = 0x00005cd4;
pub const FTQ_RCVBD_COMP_FIFO_ENQDEQ: c_uint = 0x00005cd8;
pub const FTQ_RCVBD_COMP_WRITE_PEEK: c_uint = 0x00005cdc;
pub const FTQ_RCVLST_PLMT_CTL: c_uint = 0x00005ce0;
pub const FTQ_RCVLST_PLMT_FULL_CNT: c_uint = 0x00005ce4;
pub const FTQ_RCVLST_PLMT_FIFO_ENQDEQ: c_uint = 0x00005ce8;
pub const FTQ_RCVLST_PLMT_WRITE_PEEK: c_uint = 0x00005cec;
pub const FTQ_RCVDATA_INI_CTL: c_uint = 0x00005cf0;
pub const FTQ_RCVDATA_INI_FULL_CNT: c_uint = 0x00005cf4;
pub const FTQ_RCVDATA_INI_FIFO_ENQDEQ: c_uint = 0x00005cf8;
pub const FTQ_RCVDATA_INI_WRITE_PEEK: c_uint = 0x00005cfc;
pub const FTQ_RCVDATA_COMP_CTL: c_uint = 0x00005d00;
pub const FTQ_RCVDATA_COMP_FULL_CNT: c_uint = 0x00005d04;
pub const FTQ_RCVDATA_COMP_FIFO_ENQDEQ: c_uint = 0x00005d08;
pub const FTQ_RCVDATA_COMP_WRITE_PEEK: c_uint = 0x00005d0c;
pub const FTQ_SWTYPE2_CTL: c_uint = 0x00005d10;
pub const FTQ_SWTYPE2_FULL_CNT: c_uint = 0x00005d14;
pub const FTQ_SWTYPE2_FIFO_ENQDEQ: c_uint = 0x00005d18;
pub const FTQ_SWTYPE2_WRITE_PEEK: c_uint = 0x00005d1c;
// 0x5d20 --> 0x6000 unused
// Message signaled interrupt registers
pub const MSGINT_MODE: c_uint = 0x00006000;
pub const MSGINT_MODE_RESET: c_uint = 0x00000001;
pub const MSGINT_MODE_ENABLE: c_uint = 0x00000002;
pub const MSGINT_MODE_ONE_SHOT_DISABLE: c_uint = 0x00000020;
pub const MSGINT_MODE_MULTIVEC_EN: c_uint = 0x00000080;
pub const MSGINT_STATUS: c_uint = 0x00006004;
pub const MSGINT_STATUS_MSI_REQ: c_uint = 0x00000001;
pub const MSGINT_FIFO: c_uint = 0x00006008;
// 0x600c --> 0x6400 unused
// DMA completion registers
pub const DMAC_MODE: c_uint = 0x00006400;
pub const DMAC_MODE_RESET: c_uint = 0x00000001;
pub const DMAC_MODE_ENABLE: c_uint = 0x00000002;
// 0x6404 --> 0x6800 unused
// GRC registers
pub const GRC_MODE: c_uint = 0x00006800;
pub const GRC_MODE_UPD_ON_COAL: c_uint = 0x00000001;
pub const GRC_MODE_BSWAP_NONFRM_DATA: c_uint = 0x00000002;
pub const GRC_MODE_WSWAP_NONFRM_DATA: c_uint = 0x00000004;
pub const GRC_MODE_BSWAP_DATA: c_uint = 0x00000010;
pub const GRC_MODE_WSWAP_DATA: c_uint = 0x00000020;
pub const GRC_MODE_BYTE_SWAP_B2HRX_DATA: c_uint = 0x00000040;
pub const GRC_MODE_WORD_SWAP_B2HRX_DATA: c_uint = 0x00000080;
pub const GRC_MODE_SPLITHDR: c_uint = 0x00000100;
pub const GRC_MODE_NOFRM_CRACKING: c_uint = 0x00000200;
pub const GRC_MODE_INCL_CRC: c_uint = 0x00000400;
pub const GRC_MODE_ALLOW_BAD_FRMS: c_uint = 0x00000800;
pub const GRC_MODE_NOIRQ_ON_SENDS: c_uint = 0x00002000;
pub const GRC_MODE_NOIRQ_ON_RCV: c_uint = 0x00004000;
pub const GRC_MODE_FORCE_PCI32BIT: c_uint = 0x00008000;
pub const GRC_MODE_B2HRX_ENABLE: c_uint = 0x00008000;
pub const GRC_MODE_HOST_STACKUP: c_uint = 0x00010000;
pub const GRC_MODE_HOST_SENDBDS: c_uint = 0x00020000;
pub const GRC_MODE_HTX2B_ENABLE: c_uint = 0x00040000;
pub const GRC_MODE_TIME_SYNC_ENABLE: c_uint = 0x00080000;
pub const GRC_MODE_NO_TX_PHDR_CSUM: c_uint = 0x00100000;
pub const GRC_MODE_NVRAM_WR_ENABLE: c_uint = 0x00200000;
pub const GRC_MODE_PCIE_TL_SEL: c_uint = 0x00000000;
pub const GRC_MODE_PCIE_PL_SEL: c_uint = 0x00400000;
pub const GRC_MODE_NO_RX_PHDR_CSUM: c_uint = 0x00800000;
pub const GRC_MODE_IRQ_ON_TX_CPU_ATTN: c_uint = 0x01000000;
pub const GRC_MODE_IRQ_ON_RX_CPU_ATTN: c_uint = 0x02000000;
pub const GRC_MODE_IRQ_ON_MAC_ATTN: c_uint = 0x04000000;
pub const GRC_MODE_IRQ_ON_DMA_ATTN: c_uint = 0x08000000;
pub const GRC_MODE_IRQ_ON_FLOW_ATTN: c_uint = 0x10000000;
pub const GRC_MODE_4X_NIC_SEND_RINGS: c_uint = 0x20000000;
pub const GRC_MODE_PCIE_DL_SEL: c_uint = 0x20000000;
pub const GRC_MODE_MCAST_FRM_ENABLE: c_uint = 0x40000000;
pub const GRC_MODE_PCIE_HI_1K_EN: c_uint = 0x80000000;

pub const GRC_MISC_CFG: c_uint = 0x00006804;
pub const GRC_MISC_CFG_CORECLK_RESET: c_uint = 0x00000001;
pub const GRC_MISC_CFG_PRESCALAR_MASK: c_uint = 0x000000fe;
pub const GRC_MISC_CFG_PRESCALAR_SHIFT: c_int = 1;
pub const GRC_MISC_CFG_BOARD_ID_MASK: c_uint = 0x0001e000;
pub const GRC_MISC_CFG_BOARD_ID_5700: c_uint = 0x0001e000;
pub const GRC_MISC_CFG_BOARD_ID_5701: c_uint = 0x00000000;
pub const GRC_MISC_CFG_BOARD_ID_5702FE: c_uint = 0x00004000;
pub const GRC_MISC_CFG_BOARD_ID_5703: c_uint = 0x00000000;
pub const GRC_MISC_CFG_BOARD_ID_5703S: c_uint = 0x00002000;
pub const GRC_MISC_CFG_BOARD_ID_5704: c_uint = 0x00000000;
pub const GRC_MISC_CFG_BOARD_ID_5704CIOBE: c_uint = 0x00004000;
pub const GRC_MISC_CFG_BOARD_ID_5704_A2: c_uint = 0x00008000;
pub const GRC_MISC_CFG_BOARD_ID_5788: c_uint = 0x00010000;
pub const GRC_MISC_CFG_BOARD_ID_5788M: c_uint = 0x00018000;
pub const GRC_MISC_CFG_BOARD_ID_AC91002A1: c_uint = 0x00018000;
pub const GRC_MISC_CFG_EPHY_IDDQ: c_uint = 0x00200000;
pub const GRC_MISC_CFG_KEEP_GPHY_POWER: c_uint = 0x04000000;
pub const GRC_LOCAL_CTRL: c_uint = 0x00006808;
pub const GRC_LCLCTRL_INT_ACTIVE: c_uint = 0x00000001;
pub const GRC_LCLCTRL_CLEARINT: c_uint = 0x00000002;
pub const GRC_LCLCTRL_SETINT: c_uint = 0x00000004;
pub const GRC_LCLCTRL_INT_ON_ATTN: c_uint = 0x00000008;
pub const GRC_LCLCTRL_GPIO_UART_SEL: c_uint = 0x00000010	/* 5755 only */;
pub const GRC_LCLCTRL_USE_SIG_DETECT: c_uint = 0x00000010	/* 5714/5780 only */;
pub const GRC_LCLCTRL_USE_EXT_SIG_DETECT: c_uint = 0x00000020	/* 5714/5780 only */;
pub const GRC_LCLCTRL_GPIO_INPUT3: c_uint = 0x00000020;
pub const GRC_LCLCTRL_GPIO_OE3: c_uint = 0x00000040;
pub const GRC_LCLCTRL_GPIO_OUTPUT3: c_uint = 0x00000080;
pub const GRC_LCLCTRL_GPIO_INPUT0: c_uint = 0x00000100;
pub const GRC_LCLCTRL_GPIO_INPUT1: c_uint = 0x00000200;
pub const GRC_LCLCTRL_GPIO_INPUT2: c_uint = 0x00000400;
pub const GRC_LCLCTRL_GPIO_OE0: c_uint = 0x00000800;
pub const GRC_LCLCTRL_GPIO_OE1: c_uint = 0x00001000;
pub const GRC_LCLCTRL_GPIO_OE2: c_uint = 0x00002000;
pub const GRC_LCLCTRL_GPIO_OUTPUT0: c_uint = 0x00004000;
pub const GRC_LCLCTRL_GPIO_OUTPUT1: c_uint = 0x00008000;
pub const GRC_LCLCTRL_GPIO_OUTPUT2: c_uint = 0x00010000;
pub const GRC_LCLCTRL_EXTMEM_ENABLE: c_uint = 0x00020000;
pub const GRC_LCLCTRL_MEMSZ_MASK: c_uint = 0x001c0000;
pub const GRC_LCLCTRL_MEMSZ_256K: c_uint = 0x00000000;
pub const GRC_LCLCTRL_MEMSZ_512K: c_uint = 0x00040000;
pub const GRC_LCLCTRL_MEMSZ_1M: c_uint = 0x00080000;
pub const GRC_LCLCTRL_MEMSZ_2M: c_uint = 0x000c0000;
pub const GRC_LCLCTRL_MEMSZ_4M: c_uint = 0x00100000;
pub const GRC_LCLCTRL_MEMSZ_8M: c_uint = 0x00140000;
pub const GRC_LCLCTRL_MEMSZ_16M: c_uint = 0x00180000;
pub const GRC_LCLCTRL_BANK_SELECT: c_uint = 0x00200000;
pub const GRC_LCLCTRL_SSRAM_TYPE: c_uint = 0x00400000;
pub const GRC_LCLCTRL_AUTO_SEEPROM: c_uint = 0x01000000;
pub const GRC_TIMER: c_uint = 0x0000680c;
pub const GRC_RX_CPU_EVENT: c_uint = 0x00006810;
pub const GRC_RX_CPU_DRIVER_EVENT: c_uint = 0x00004000;
pub const GRC_RX_TIMER_REF: c_uint = 0x00006814;
pub const GRC_RX_CPU_SEM: c_uint = 0x00006818;
pub const GRC_REMOTE_RX_CPU_ATTN: c_uint = 0x0000681c;
pub const GRC_TX_CPU_EVENT: c_uint = 0x00006820;
pub const GRC_TX_TIMER_REF: c_uint = 0x00006824;
pub const GRC_TX_CPU_SEM: c_uint = 0x00006828;
pub const GRC_REMOTE_TX_CPU_ATTN: c_uint = 0x0000682c;
pub const GRC_MEM_POWER_UP: c_uint = 0x00006830 /* 64-bit */;
pub const GRC_EEPROM_ADDR: c_uint = 0x00006838;
pub const EEPROM_ADDR_WRITE: c_uint = 0x00000000;
pub const EEPROM_ADDR_READ: c_uint = 0x80000000;
pub const EEPROM_ADDR_COMPLETE: c_uint = 0x40000000;
pub const EEPROM_ADDR_FSM_RESET: c_uint = 0x20000000;
pub const EEPROM_ADDR_DEVID_MASK: c_uint = 0x1c000000;
pub const EEPROM_ADDR_DEVID_SHIFT: c_int = 26;
pub const EEPROM_ADDR_START: c_uint = 0x02000000;
pub const EEPROM_ADDR_CLKPERD_SHIFT: c_int = 16;
pub const EEPROM_ADDR_ADDR_MASK: c_uint = 0x0000ffff;
pub const EEPROM_ADDR_ADDR_SHIFT: c_int = 0;
pub const EEPROM_DEFAULT_CLOCK_PERIOD: c_uint = 0x60;

pub const GRC_EEPROM_DATA: c_uint = 0x0000683c;
pub const GRC_EEPROM_CTRL: c_uint = 0x00006840;
pub const GRC_MDI_CTRL: c_uint = 0x00006844;
pub const GRC_SEEPROM_DELAY: c_uint = 0x00006848;
// 0x684c --> 0x6890 unused
pub const GRC_VCPU_EXT_CTRL: c_uint = 0x00006890;
pub const GRC_VCPU_EXT_CTRL_HALT_CPU: c_uint = 0x00400000;
pub const GRC_VCPU_EXT_CTRL_DISABLE_WOL: c_uint = 0x20000000;
pub const GRC_FASTBOOT_PC: c_uint = 0x00006894	/* 5752, 5755, 5787 */;
pub const TG3_EAV_REF_CLCK_LSB: c_uint = 0x00006900;
pub const TG3_EAV_REF_CLCK_MSB: c_uint = 0x00006904;
pub const TG3_EAV_REF_CLCK_CTL: c_uint = 0x00006908;
pub const TG3_EAV_REF_CLCK_CTL_STOP: c_uint = 0x00000002;
pub const TG3_EAV_REF_CLCK_CTL_RESUME: c_uint = 0x00000004;

pub const TG3_EAV_WATCHDOG0_LSB: c_uint = 0x00006918;
pub const TG3_EAV_WATCHDOG0_MSB: c_uint = 0x0000691c;

pub const TG3_EAV_WATCHDOG_MSB_MASK: c_uint = 0x7fffffff;
pub const TG3_EAV_REF_CLK_CORRECT_CTL: c_uint = 0x00006928;

pub const TG3_EAV_REF_CLK_CORRECT_MASK: c_uint = 0xffffff;
// 0x692c --> 0x7000 unused
// NVRAM Control registers
pub const NVRAM_CMD: c_uint = 0x00007000;
pub const NVRAM_CMD_RESET: c_uint = 0x00000001;
pub const NVRAM_CMD_DONE: c_uint = 0x00000008;
pub const NVRAM_CMD_GO: c_uint = 0x00000010;
pub const NVRAM_CMD_WR: c_uint = 0x00000020;
pub const NVRAM_CMD_RD: c_uint = 0x00000000;
pub const NVRAM_CMD_ERASE: c_uint = 0x00000040;
pub const NVRAM_CMD_FIRST: c_uint = 0x00000080;
pub const NVRAM_CMD_LAST: c_uint = 0x00000100;
pub const NVRAM_CMD_WREN: c_uint = 0x00010000;
pub const NVRAM_CMD_WRDI: c_uint = 0x00020000;
pub const NVRAM_STAT: c_uint = 0x00007004;
pub const NVRAM_WRDATA: c_uint = 0x00007008;
pub const NVRAM_ADDR: c_uint = 0x0000700c;
pub const NVRAM_ADDR_MSK: c_uint = 0x07ffffff;
pub const NVRAM_RDDATA: c_uint = 0x00007010;
pub const NVRAM_CFG1: c_uint = 0x00007014;
pub const NVRAM_CFG1_FLASHIF_ENAB: c_uint = 0x00000001;
pub const NVRAM_CFG1_BUFFERED_MODE: c_uint = 0x00000002;
pub const NVRAM_CFG1_PASS_THRU: c_uint = 0x00000004;
pub const NVRAM_CFG1_STATUS_BITS: c_uint = 0x00000070;
pub const NVRAM_CFG1_BIT_BANG: c_uint = 0x00000008;
pub const NVRAM_CFG1_FLASH_SIZE: c_uint = 0x02000000;
pub const NVRAM_CFG1_COMPAT_BYPASS: c_uint = 0x80000000;
pub const NVRAM_CFG1_VENDOR_MASK: c_uint = 0x03000003;
pub const FLASH_VENDOR_ATMEL_EEPROM: c_uint = 0x02000000;
pub const FLASH_VENDOR_ATMEL_FLASH_BUFFERED: c_uint = 0x02000003;
pub const FLASH_VENDOR_ATMEL_FLASH_UNBUFFERED: c_uint = 0x00000003;
pub const FLASH_VENDOR_ST: c_uint = 0x03000001;
pub const FLASH_VENDOR_SAIFUN: c_uint = 0x01000003;
pub const FLASH_VENDOR_SST_SMALL: c_uint = 0x00000001;
pub const FLASH_VENDOR_SST_LARGE: c_uint = 0x02000001;
pub const NVRAM_CFG1_5752VENDOR_MASK: c_uint = 0x03c00003;
pub const NVRAM_CFG1_5762VENDOR_MASK: c_uint = 0x03e00003;
pub const FLASH_5752VENDOR_ATMEL_EEPROM_64KHZ: c_uint = 0x00000000;
pub const FLASH_5752VENDOR_ATMEL_EEPROM_376KHZ: c_uint = 0x02000000;
pub const FLASH_5752VENDOR_ATMEL_FLASH_BUFFERED: c_uint = 0x02000003;
pub const FLASH_5752VENDOR_ST_M45PE10: c_uint = 0x02400000;
pub const FLASH_5752VENDOR_ST_M45PE20: c_uint = 0x02400002;
pub const FLASH_5752VENDOR_ST_M45PE40: c_uint = 0x02400001;
pub const FLASH_5755VENDOR_ATMEL_FLASH_1: c_uint = 0x03400001;
pub const FLASH_5755VENDOR_ATMEL_FLASH_2: c_uint = 0x03400002;
pub const FLASH_5755VENDOR_ATMEL_FLASH_3: c_uint = 0x03400000;
pub const FLASH_5755VENDOR_ATMEL_FLASH_4: c_uint = 0x00000003;
pub const FLASH_5755VENDOR_ATMEL_FLASH_5: c_uint = 0x02000003;
pub const FLASH_5755VENDOR_ATMEL_EEPROM_64KHZ: c_uint = 0x03c00003;
pub const FLASH_5755VENDOR_ATMEL_EEPROM_376KHZ: c_uint = 0x03c00002;
pub const FLASH_5787VENDOR_ATMEL_EEPROM_64KHZ: c_uint = 0x03000003;
pub const FLASH_5787VENDOR_ATMEL_EEPROM_376KHZ: c_uint = 0x03000002;
pub const FLASH_5787VENDOR_MICRO_EEPROM_64KHZ: c_uint = 0x03000000;
pub const FLASH_5787VENDOR_MICRO_EEPROM_376KHZ: c_uint = 0x02000000;
pub const FLASH_5761VENDOR_ATMEL_MDB021D: c_uint = 0x00800003;
pub const FLASH_5761VENDOR_ATMEL_MDB041D: c_uint = 0x00800000;
pub const FLASH_5761VENDOR_ATMEL_MDB081D: c_uint = 0x00800002;
pub const FLASH_5761VENDOR_ATMEL_MDB161D: c_uint = 0x00800001;
pub const FLASH_5761VENDOR_ATMEL_ADB021D: c_uint = 0x00000003;
pub const FLASH_5761VENDOR_ATMEL_ADB041D: c_uint = 0x00000000;
pub const FLASH_5761VENDOR_ATMEL_ADB081D: c_uint = 0x00000002;
pub const FLASH_5761VENDOR_ATMEL_ADB161D: c_uint = 0x00000001;
pub const FLASH_5761VENDOR_ST_M_M45PE20: c_uint = 0x02800001;
pub const FLASH_5761VENDOR_ST_M_M45PE40: c_uint = 0x02800000;
pub const FLASH_5761VENDOR_ST_M_M45PE80: c_uint = 0x02800002;
pub const FLASH_5761VENDOR_ST_M_M45PE16: c_uint = 0x02800003;
pub const FLASH_5761VENDOR_ST_A_M45PE20: c_uint = 0x02000001;
pub const FLASH_5761VENDOR_ST_A_M45PE40: c_uint = 0x02000000;
pub const FLASH_5761VENDOR_ST_A_M45PE80: c_uint = 0x02000002;
pub const FLASH_5761VENDOR_ST_A_M45PE16: c_uint = 0x02000003;
pub const FLASH_57780VENDOR_ATMEL_AT45DB011D: c_uint = 0x00400000;
pub const FLASH_57780VENDOR_ATMEL_AT45DB011B: c_uint = 0x03400000;
pub const FLASH_57780VENDOR_ATMEL_AT45DB021D: c_uint = 0x00400002;
pub const FLASH_57780VENDOR_ATMEL_AT45DB021B: c_uint = 0x03400002;
pub const FLASH_57780VENDOR_ATMEL_AT45DB041D: c_uint = 0x00400001;
pub const FLASH_57780VENDOR_ATMEL_AT45DB041B: c_uint = 0x03400001;
pub const FLASH_5717VENDOR_ATMEL_EEPROM: c_uint = 0x02000001;
pub const FLASH_5717VENDOR_MICRO_EEPROM: c_uint = 0x02000003;
pub const FLASH_5717VENDOR_ATMEL_MDB011D: c_uint = 0x01000001;
pub const FLASH_5717VENDOR_ATMEL_MDB021D: c_uint = 0x01000003;
pub const FLASH_5717VENDOR_ST_M_M25PE10: c_uint = 0x02000000;
pub const FLASH_5717VENDOR_ST_M_M25PE20: c_uint = 0x02000002;
pub const FLASH_5717VENDOR_ST_M_M45PE10: c_uint = 0x00000001;
pub const FLASH_5717VENDOR_ST_M_M45PE20: c_uint = 0x00000003;
pub const FLASH_5717VENDOR_ATMEL_ADB011B: c_uint = 0x01400000;
pub const FLASH_5717VENDOR_ATMEL_ADB021B: c_uint = 0x01400002;
pub const FLASH_5717VENDOR_ATMEL_ADB011D: c_uint = 0x01400001;
pub const FLASH_5717VENDOR_ATMEL_ADB021D: c_uint = 0x01400003;
pub const FLASH_5717VENDOR_ST_A_M25PE10: c_uint = 0x02400000;
pub const FLASH_5717VENDOR_ST_A_M25PE20: c_uint = 0x02400002;
pub const FLASH_5717VENDOR_ST_A_M45PE10: c_uint = 0x02400001;
pub const FLASH_5717VENDOR_ST_A_M45PE20: c_uint = 0x02400003;
pub const FLASH_5717VENDOR_ATMEL_45USPT: c_uint = 0x03400000;
pub const FLASH_5717VENDOR_ST_25USPT: c_uint = 0x03400002;
pub const FLASH_5717VENDOR_ST_45USPT: c_uint = 0x03400001;
pub const FLASH_5720_EEPROM_HD: c_uint = 0x00000001;
pub const FLASH_5720_EEPROM_LD: c_uint = 0x00000003;
pub const FLASH_5762_EEPROM_HD: c_uint = 0x02000001;
pub const FLASH_5762_EEPROM_LD: c_uint = 0x02000003;
pub const FLASH_5762_MX25L_100: c_uint = 0x00800000;
pub const FLASH_5762_MX25L_200: c_uint = 0x00800002;
pub const FLASH_5762_MX25L_400: c_uint = 0x00800001;
pub const FLASH_5762_MX25L_800: c_uint = 0x00800003;
pub const FLASH_5762_MX25L_160_320: c_uint = 0x03800002;
pub const FLASH_5720VENDOR_M_ATMEL_DB011D: c_uint = 0x01000000;
pub const FLASH_5720VENDOR_M_ATMEL_DB021D: c_uint = 0x01000002;
pub const FLASH_5720VENDOR_M_ATMEL_DB041D: c_uint = 0x01000001;
pub const FLASH_5720VENDOR_M_ATMEL_DB081D: c_uint = 0x01000003;
pub const FLASH_5720VENDOR_M_ST_M25PE10: c_uint = 0x02000000;
pub const FLASH_5720VENDOR_M_ST_M25PE20: c_uint = 0x02000002;
pub const FLASH_5720VENDOR_M_ST_M25PE40: c_uint = 0x02000001;
pub const FLASH_5720VENDOR_M_ST_M25PE80: c_uint = 0x02000003;
pub const FLASH_5720VENDOR_M_ST_M45PE10: c_uint = 0x03000000;
pub const FLASH_5720VENDOR_M_ST_M45PE20: c_uint = 0x03000002;
pub const FLASH_5720VENDOR_M_ST_M45PE40: c_uint = 0x03000001;
pub const FLASH_5720VENDOR_M_ST_M45PE80: c_uint = 0x03000003;
pub const FLASH_5720VENDOR_A_ATMEL_DB011B: c_uint = 0x01800000;
pub const FLASH_5720VENDOR_A_ATMEL_DB021B: c_uint = 0x01800002;
pub const FLASH_5720VENDOR_A_ATMEL_DB041B: c_uint = 0x01800001;
pub const FLASH_5720VENDOR_A_ATMEL_DB011D: c_uint = 0x01c00000;
pub const FLASH_5720VENDOR_A_ATMEL_DB021D: c_uint = 0x01c00002;
pub const FLASH_5720VENDOR_A_ATMEL_DB041D: c_uint = 0x01c00001;
pub const FLASH_5720VENDOR_A_ATMEL_DB081D: c_uint = 0x01c00003;
pub const FLASH_5720VENDOR_A_ST_M25PE10: c_uint = 0x02800000;
pub const FLASH_5720VENDOR_A_ST_M25PE20: c_uint = 0x02800002;
pub const FLASH_5720VENDOR_A_ST_M25PE40: c_uint = 0x02800001;
pub const FLASH_5720VENDOR_A_ST_M25PE80: c_uint = 0x02800003;
pub const FLASH_5720VENDOR_A_ST_M45PE10: c_uint = 0x02c00000;
pub const FLASH_5720VENDOR_A_ST_M45PE20: c_uint = 0x02c00002;
pub const FLASH_5720VENDOR_A_ST_M45PE40: c_uint = 0x02c00001;
pub const FLASH_5720VENDOR_A_ST_M45PE80: c_uint = 0x02c00003;
pub const FLASH_5720VENDOR_ATMEL_45USPT: c_uint = 0x03c00000;
pub const FLASH_5720VENDOR_ST_25USPT: c_uint = 0x03c00002;
pub const FLASH_5720VENDOR_ST_45USPT: c_uint = 0x03c00001;
pub const NVRAM_CFG1_5752PAGE_SIZE_MASK: c_uint = 0x70000000;
pub const FLASH_5752PAGE_SIZE_256: c_uint = 0x00000000;
pub const FLASH_5752PAGE_SIZE_512: c_uint = 0x10000000;
pub const FLASH_5752PAGE_SIZE_1K: c_uint = 0x20000000;
pub const FLASH_5752PAGE_SIZE_2K: c_uint = 0x30000000;
pub const FLASH_5752PAGE_SIZE_4K: c_uint = 0x40000000;
pub const FLASH_5752PAGE_SIZE_264: c_uint = 0x50000000;
pub const FLASH_5752PAGE_SIZE_528: c_uint = 0x60000000;
pub const NVRAM_CFG2: c_uint = 0x00007018;
pub const NVRAM_CFG3: c_uint = 0x0000701c;
pub const NVRAM_SWARB: c_uint = 0x00007020;
pub const SWARB_REQ_SET0: c_uint = 0x00000001;
pub const SWARB_REQ_SET1: c_uint = 0x00000002;
pub const SWARB_REQ_SET2: c_uint = 0x00000004;
pub const SWARB_REQ_SET3: c_uint = 0x00000008;
pub const SWARB_REQ_CLR0: c_uint = 0x00000010;
pub const SWARB_REQ_CLR1: c_uint = 0x00000020;
pub const SWARB_REQ_CLR2: c_uint = 0x00000040;
pub const SWARB_REQ_CLR3: c_uint = 0x00000080;
pub const SWARB_GNT0: c_uint = 0x00000100;
pub const SWARB_GNT1: c_uint = 0x00000200;
pub const SWARB_GNT2: c_uint = 0x00000400;
pub const SWARB_GNT3: c_uint = 0x00000800;
pub const SWARB_REQ0: c_uint = 0x00001000;
pub const SWARB_REQ1: c_uint = 0x00002000;
pub const SWARB_REQ2: c_uint = 0x00004000;
pub const SWARB_REQ3: c_uint = 0x00008000;
pub const NVRAM_ACCESS: c_uint = 0x00007024;
pub const ACCESS_ENABLE: c_uint = 0x00000001;
pub const ACCESS_WR_ENABLE: c_uint = 0x00000002;
pub const NVRAM_WRITE1: c_uint = 0x00007028;
// 0x702c unused
pub const NVRAM_ADDR_LOCKOUT: c_uint = 0x00007030;
pub const NVRAM_AUTOSENSE_STATUS: c_uint = 0x00007038;
pub const AUTOSENSE_DEVID: c_uint = 0x00000010;
pub const AUTOSENSE_DEVID_MASK: c_uint = 0x00000007;
pub const AUTOSENSE_SIZE_IN_MB: c_int = 17;
// 0x703c --> 0x7500 unused
pub const OTP_MODE: c_uint = 0x00007500;
pub const OTP_MODE_OTP_THRU_GRC: c_uint = 0x00000001;
pub const OTP_CTRL: c_uint = 0x00007504;
pub const OTP_CTRL_OTP_PROG_ENABLE: c_uint = 0x00200000;
pub const OTP_CTRL_OTP_CMD_READ: c_uint = 0x00000000;
pub const OTP_CTRL_OTP_CMD_INIT: c_uint = 0x00000008;
pub const OTP_CTRL_OTP_CMD_START: c_uint = 0x00000001;
pub const OTP_STATUS: c_uint = 0x00007508;
pub const OTP_STATUS_CMD_DONE: c_uint = 0x00000001;
pub const OTP_ADDRESS: c_uint = 0x0000750c;
pub const OTP_ADDRESS_MAGIC1: c_uint = 0x000000a0;
pub const OTP_ADDRESS_MAGIC2: c_uint = 0x00000080;
// 0x7510 unused
pub const OTP_READ_DATA: c_uint = 0x00007514;
// 0x7518 --> 0x7c04 unused
pub const PCIE_TRANSACTION_CFG: c_uint = 0x00007c04;
pub const PCIE_TRANS_CFG_1SHOT_MSI: c_uint = 0x20000000;
pub const PCIE_TRANS_CFG_LOM: c_uint = 0x00000020;
// 0x7c08 --> 0x7d28 unused
pub const PCIE_PWR_MGMT_THRESH: c_uint = 0x00007d28;
pub const PCIE_PWR_MGMT_L1_THRESH_MSK: c_uint = 0x0000ff00;
pub const PCIE_PWR_MGMT_L1_THRESH_4MS: c_uint = 0x0000ff00;
pub const PCIE_PWR_MGMT_EXT_ASPM_TMR_EN: c_uint = 0x01000000;
// 0x7d2c --> 0x7d54 unused
pub const TG3_PCIE_LNKCTL: c_uint = 0x00007d54;
pub const TG3_PCIE_LNKCTL_L1_PLL_PD_EN: c_uint = 0x00000008;
pub const TG3_PCIE_LNKCTL_L1_PLL_PD_DIS: c_uint = 0x00000080;
// 0x7d58 --> 0x7e70 unused
pub const TG3_PCIE_PHY_TSTCTL: c_uint = 0x00007e2c;
pub const TG3_PCIE_PHY_TSTCTL_PCIE10: c_uint = 0x00000040;
pub const TG3_PCIE_PHY_TSTCTL_PSCRAM: c_uint = 0x00000020;
pub const TG3_PCIE_EIDLE_DELAY: c_uint = 0x00007e70;
pub const TG3_PCIE_EIDLE_DELAY_MASK: c_uint = 0x0000001f;
pub const TG3_PCIE_EIDLE_DELAY_13_CLKS: c_uint = 0x0000000c;
// 0x7e74 --> 0x8000 unused
// Alternate PCIE definitions
pub const TG3_PCIE_TLDLPL_PORT: c_uint = 0x00007c00;
pub const TG3_PCIE_DL_LO_FTSMAX: c_uint = 0x0000000c;
pub const TG3_PCIE_DL_LO_FTSMAX_MSK: c_uint = 0x000000ff;
pub const TG3_PCIE_DL_LO_FTSMAX_VAL: c_uint = 0x0000002c;
pub const TG3_PCIE_PL_LO_PHYCTL1: c_uint = 0x00000004;
pub const TG3_PCIE_PL_LO_PHYCTL1_L1PLLPD_EN: c_uint = 0x00001000;
pub const TG3_PCIE_PL_LO_PHYCTL5: c_uint = 0x00000014;
pub const TG3_PCIE_PL_LO_PHYCTL5_DIS_L2CLKREQ: c_uint = 0x80000000;
pub const TG3_REG_BLK_SIZE: c_uint = 0x00008000;
// OTP bit definitions
pub const TG3_OTP_AGCTGT_MASK: c_uint = 0x000000e0;
pub const TG3_OTP_AGCTGT_SHIFT: c_int = 1;
pub const TG3_OTP_HPFFLTR_MASK: c_uint = 0x00000300;
pub const TG3_OTP_HPFFLTR_SHIFT: c_int = 1;
pub const TG3_OTP_HPFOVER_MASK: c_uint = 0x00000400;
pub const TG3_OTP_HPFOVER_SHIFT: c_int = 1;
pub const TG3_OTP_LPFDIS_MASK: c_uint = 0x00000800;
pub const TG3_OTP_LPFDIS_SHIFT: c_int = 11;
pub const TG3_OTP_VDAC_MASK: c_uint = 0xff000000;
pub const TG3_OTP_VDAC_SHIFT: c_int = 24;
pub const TG3_OTP_10BTAMP_MASK: c_uint = 0x0000f000;
pub const TG3_OTP_10BTAMP_SHIFT: c_int = 8;
pub const TG3_OTP_ROFF_MASK: c_uint = 0x00e00000;
pub const TG3_OTP_ROFF_SHIFT: c_int = 11;
pub const TG3_OTP_RCOFF_MASK: c_uint = 0x001c0000;
pub const TG3_OTP_RCOFF_SHIFT: c_int = 16;
pub const TG3_OTP_DEFAULT: c_uint = 0x286c1640;
// Hardware Legacy NVRAM layout
pub const TG3_NVM_VPD_OFF: c_uint = 0x100;
pub const TG3_NVM_VPD_LEN: c_int = 256;
// Hardware Selfboot NVRAM layout
pub const TG3_NVM_HWSB_CFG1: c_uint = 0x00000004;
pub const TG3_NVM_HWSB_CFG1_MAJMSK: c_uint = 0xf8000000;
pub const TG3_NVM_HWSB_CFG1_MAJSFT: c_int = 27;
pub const TG3_NVM_HWSB_CFG1_MINMSK: c_uint = 0x07c00000;
pub const TG3_NVM_HWSB_CFG1_MINSFT: c_int = 22;
pub const TG3_EEPROM_MAGIC: c_uint = 0x669955aa;
pub const TG3_EEPROM_MAGIC_FW: c_uint = 0xa5000000;
pub const TG3_EEPROM_MAGIC_FW_MSK: c_uint = 0xff000000;
pub const TG3_EEPROM_SB_FORMAT_MASK: c_uint = 0x00e00000;
pub const TG3_EEPROM_SB_FORMAT_1: c_uint = 0x00200000;
pub const TG3_EEPROM_SB_REVISION_MASK: c_uint = 0x001f0000;
pub const TG3_EEPROM_SB_REVISION_0: c_uint = 0x00000000;
pub const TG3_EEPROM_SB_REVISION_2: c_uint = 0x00020000;
pub const TG3_EEPROM_SB_REVISION_3: c_uint = 0x00030000;
pub const TG3_EEPROM_SB_REVISION_4: c_uint = 0x00040000;
pub const TG3_EEPROM_SB_REVISION_5: c_uint = 0x00050000;
pub const TG3_EEPROM_SB_REVISION_6: c_uint = 0x00060000;
pub const TG3_EEPROM_MAGIC_HW: c_uint = 0xabcd;
pub const TG3_EEPROM_MAGIC_HW_MSK: c_uint = 0xffff;
pub const TG3_NVM_DIR_START: c_uint = 0x18;
pub const TG3_NVM_DIR_END: c_uint = 0x78;
pub const TG3_NVM_DIRENT_SIZE: c_uint = 0xc;
pub const TG3_NVM_DIRTYPE_SHIFT: c_int = 24;
pub const TG3_NVM_DIRTYPE_LENMSK: c_uint = 0x003fffff;
pub const TG3_NVM_DIRTYPE_ASFINI: c_int = 1;
pub const TG3_NVM_DIRTYPE_EXTVPD: c_int = 20;
pub const TG3_NVM_PTREV_BCVER: c_uint = 0x94;
pub const TG3_NVM_BCVER_MAJMSK: c_uint = 0x0000ff00;
pub const TG3_NVM_BCVER_MAJSFT: c_int = 8;
pub const TG3_NVM_BCVER_MINMSK: c_uint = 0x000000ff;
pub const TG3_EEPROM_SB_F1R0_EDH_OFF: c_uint = 0x10;
pub const TG3_EEPROM_SB_F1R2_EDH_OFF: c_uint = 0x14;
pub const TG3_EEPROM_SB_F1R2_MBA_OFF: c_uint = 0x10;
pub const TG3_EEPROM_SB_F1R3_EDH_OFF: c_uint = 0x18;
pub const TG3_EEPROM_SB_F1R4_EDH_OFF: c_uint = 0x1c;
pub const TG3_EEPROM_SB_F1R5_EDH_OFF: c_uint = 0x20;
pub const TG3_EEPROM_SB_F1R6_EDH_OFF: c_uint = 0x4c;
pub const TG3_EEPROM_SB_EDH_MAJ_MASK: c_uint = 0x00000700;
pub const TG3_EEPROM_SB_EDH_MAJ_SHFT: c_int = 8;
pub const TG3_EEPROM_SB_EDH_MIN_MASK: c_uint = 0x000000ff;
pub const TG3_EEPROM_SB_EDH_BLD_MASK: c_uint = 0x0000f800;
pub const TG3_EEPROM_SB_EDH_BLD_SHFT: c_int = 11;
// 32K Window into NIC internal memory
pub const NIC_SRAM_WIN_BASE: c_uint = 0x00008000;
// Offsets into first 32k of NIC internal memory.
pub const NIC_SRAM_PAGE_ZERO: c_uint = 0x00000000;
pub const NIC_SRAM_SEND_RCB: c_uint = 0x00000100 /* 16 * TG3_BDINFO_... */;
pub const NIC_SRAM_RCV_RET_RCB: c_uint = 0x00000200 /* 16 * TG3_BDINFO_... */;
pub const NIC_SRAM_STATS_BLK: c_uint = 0x00000300;
pub const NIC_SRAM_STATUS_BLK: c_uint = 0x00000b00;
pub const NIC_SRAM_FIRMWARE_MBOX: c_uint = 0x00000b50;
pub const NIC_SRAM_FIRMWARE_MBOX_MAGIC1: c_uint = 0x4B657654;
pub const NIC_SRAM_FIRMWARE_MBOX_MAGIC2: c_uint = 0x4861764b /* !dma on linkchg */;
pub const NIC_SRAM_DATA_SIG: c_uint = 0x00000b54;
pub const NIC_SRAM_DATA_SIG_MAGIC: c_uint = 0x4b657654 /* ascii for 'KevT' */;
pub const NIC_SRAM_DATA_CFG: c_uint = 0x00000b58;
pub const NIC_SRAM_DATA_CFG_LED_MODE_MASK: c_uint = 0x0000000c;
pub const NIC_SRAM_DATA_CFG_LED_MODE_MAC: c_uint = 0x00000000;
pub const NIC_SRAM_DATA_CFG_LED_MODE_PHY_1: c_uint = 0x00000004;
pub const NIC_SRAM_DATA_CFG_LED_MODE_PHY_2: c_uint = 0x00000008;
pub const NIC_SRAM_DATA_CFG_PHY_TYPE_MASK: c_uint = 0x00000030;
pub const NIC_SRAM_DATA_CFG_PHY_TYPE_UNKNOWN: c_uint = 0x00000000;
pub const NIC_SRAM_DATA_CFG_PHY_TYPE_COPPER: c_uint = 0x00000010;
pub const NIC_SRAM_DATA_CFG_PHY_TYPE_FIBER: c_uint = 0x00000020;
pub const NIC_SRAM_DATA_CFG_WOL_ENABLE: c_uint = 0x00000040;
pub const NIC_SRAM_DATA_CFG_ASF_ENABLE: c_uint = 0x00000080;
pub const NIC_SRAM_DATA_CFG_EEPROM_WP: c_uint = 0x00000100;
pub const NIC_SRAM_DATA_CFG_MINI_PCI: c_uint = 0x00001000;
pub const NIC_SRAM_DATA_CFG_FIBER_WOL: c_uint = 0x00004000;
pub const NIC_SRAM_DATA_CFG_NO_GPIO2: c_uint = 0x00100000;
pub const NIC_SRAM_DATA_CFG_APE_ENABLE: c_uint = 0x00200000;
pub const NIC_SRAM_DATA_VER: c_uint = 0x00000b5c;
pub const NIC_SRAM_DATA_VER_SHIFT: c_int = 16;
pub const NIC_SRAM_DATA_PHY_ID: c_uint = 0x00000b74;
pub const NIC_SRAM_DATA_PHY_ID1_MASK: c_uint = 0xffff0000;
pub const NIC_SRAM_DATA_PHY_ID2_MASK: c_uint = 0x0000ffff;
pub const NIC_SRAM_FW_CMD_MBOX: c_uint = 0x00000b78;
pub const FWCMD_NICDRV_ALIVE: c_uint = 0x00000001;
pub const FWCMD_NICDRV_PAUSE_FW: c_uint = 0x00000002;
pub const FWCMD_NICDRV_IPV4ADDR_CHG: c_uint = 0x00000003;
pub const FWCMD_NICDRV_IPV6ADDR_CHG: c_uint = 0x00000004;
pub const FWCMD_NICDRV_FIX_DMAR: c_uint = 0x00000005;
pub const FWCMD_NICDRV_FIX_DMAW: c_uint = 0x00000006;
pub const FWCMD_NICDRV_LINK_UPDATE: c_uint = 0x0000000c;
pub const FWCMD_NICDRV_ALIVE2: c_uint = 0x0000000d;
pub const FWCMD_NICDRV_ALIVE3: c_uint = 0x0000000e;
pub const NIC_SRAM_FW_CMD_LEN_MBOX: c_uint = 0x00000b7c;
pub const NIC_SRAM_FW_CMD_DATA_MBOX: c_uint = 0x00000b80;
pub const NIC_SRAM_FW_ASF_STATUS_MBOX: c_uint = 0x00000c00;
pub const NIC_SRAM_FW_DRV_STATE_MBOX: c_uint = 0x00000c04;
pub const DRV_STATE_START: c_uint = 0x00000001;
pub const DRV_STATE_START_DONE: c_uint = 0x80000001;
pub const DRV_STATE_UNLOAD: c_uint = 0x00000002;
pub const DRV_STATE_UNLOAD_DONE: c_uint = 0x80000002;
pub const DRV_STATE_WOL: c_uint = 0x00000003;
pub const DRV_STATE_SUSPEND: c_uint = 0x00000004;
pub const NIC_SRAM_FW_RESET_TYPE_MBOX: c_uint = 0x00000c08;
pub const NIC_SRAM_MAC_ADDR_HIGH_MBOX: c_uint = 0x00000c14;
pub const NIC_SRAM_MAC_ADDR_LOW_MBOX: c_uint = 0x00000c18;
pub const NIC_SRAM_WOL_MBOX: c_uint = 0x00000d30;
pub const WOL_SIGNATURE: c_uint = 0x474c0000;
pub const WOL_DRV_STATE_SHUTDOWN: c_uint = 0x00000001;
pub const WOL_DRV_WOL: c_uint = 0x00000002;
pub const WOL_SET_MAGIC_PKT: c_uint = 0x00000004;
pub const NIC_SRAM_DATA_CFG_2: c_uint = 0x00000d38;
pub const NIC_SRAM_DATA_CFG_2_APD_EN: c_uint = 0x00004000;
pub const SHASTA_EXT_LED_MODE_MASK: c_uint = 0x00018000;
pub const SHASTA_EXT_LED_LEGACY: c_uint = 0x00000000;
pub const SHASTA_EXT_LED_SHARED: c_uint = 0x00008000;
pub const SHASTA_EXT_LED_MAC: c_uint = 0x00010000;
pub const SHASTA_EXT_LED_COMBO: c_uint = 0x00018000;
pub const NIC_SRAM_DATA_CFG_3: c_uint = 0x00000d3c;
pub const NIC_SRAM_ASPM_DEBOUNCE: c_uint = 0x00000002;
pub const NIC_SRAM_LNK_FLAP_AVOID: c_uint = 0x00400000;
pub const NIC_SRAM_1G_ON_VAUX_OK: c_uint = 0x00800000;
pub const NIC_SRAM_DATA_CFG_4: c_uint = 0x00000d60;
pub const NIC_SRAM_GMII_MODE: c_uint = 0x00000002;
pub const NIC_SRAM_RGMII_INBAND_DISABLE: c_uint = 0x00000004;
pub const NIC_SRAM_RGMII_EXT_IBND_RX_EN: c_uint = 0x00000008;
pub const NIC_SRAM_RGMII_EXT_IBND_TX_EN: c_uint = 0x00000010;
pub const NIC_SRAM_CPMU_STATUS: c_uint = 0x00000e00;
pub const NIC_SRAM_CPMUSTAT_SIG: c_uint = 0x0000362c;
pub const NIC_SRAM_CPMUSTAT_SIG_MSK: c_uint = 0x0000ffff;
pub const NIC_SRAM_DATA_CFG_5: c_uint = 0x00000e0c;
pub const NIC_SRAM_DISABLE_1G_HALF_ADV: c_uint = 0x00000002;
pub const NIC_SRAM_RX_MINI_BUFFER_DESC: c_uint = 0x00001000;
pub const NIC_SRAM_DMA_DESC_POOL_BASE: c_uint = 0x00002000;
pub const NIC_SRAM_DMA_DESC_POOL_SIZE: c_uint = 0x00002000;
pub const NIC_SRAM_TX_BUFFER_DESC: c_uint = 0x00004000 /* 512 entries */;
pub const NIC_SRAM_RX_BUFFER_DESC: c_uint = 0x00006000 /* 256 entries */;
pub const NIC_SRAM_RX_JUMBO_BUFFER_DESC: c_uint = 0x00007000 /* 256 entries */;
pub const NIC_SRAM_MBUF_POOL_BASE: c_uint = 0x00008000;
pub const NIC_SRAM_MBUF_POOL_SIZE96: c_uint = 0x00018000;
pub const NIC_SRAM_MBUF_POOL_SIZE64: c_uint = 0x00010000;
pub const NIC_SRAM_MBUF_POOL_BASE5705: c_uint = 0x00010000;
pub const NIC_SRAM_MBUF_POOL_SIZE5705: c_uint = 0x0000e000;
pub const TG3_SRAM_RXCPU_SCRATCH_BASE_57766: c_uint = 0x00030000;
pub const TG3_SRAM_RXCPU_SCRATCH_SIZE_57766: c_uint = 0x00010000;
pub const TG3_57766_FW_BASE_ADDR: c_uint = 0x00030000;
pub const TG3_57766_FW_HANDSHAKE: c_uint = 0x0003fccc;
pub const TG3_SBROM_IN_SERVICE_LOOP: c_uint = 0x51;
pub const TG3_SRAM_RX_STD_BDCACHE_SIZE_5700: c_int = 128;
pub const TG3_SRAM_RX_STD_BDCACHE_SIZE_5755: c_int = 64;
pub const TG3_SRAM_RX_STD_BDCACHE_SIZE_5906: c_int = 32;
pub const TG3_SRAM_RX_JMB_BDCACHE_SIZE_5700: c_int = 64;
pub const TG3_SRAM_RX_JMB_BDCACHE_SIZE_5717: c_int = 16;
// Currently this is fixed.
pub const TG3_PHY_MII_ADDR: c_uint = 0x01;
// Tigon3 specific PHY MII registers.
pub const MII_TG3_MMD_CTRL: c_uint = 0x0d /* MMD Access Control register */;
pub const MII_TG3_MMD_CTRL_DATA_NOINC: c_uint = 0x4000;
pub const MII_TG3_MMD_ADDRESS: c_uint = 0x0e /* MMD Address Data register */;
pub const MII_TG3_EXT_CTRL: c_uint = 0x10 /* Extended control register */;
pub const MII_TG3_EXT_CTRL_FIFO_ELASTIC: c_uint = 0x0001;
pub const MII_TG3_EXT_CTRL_LNK3_LED_MODE: c_uint = 0x0002;
pub const MII_TG3_EXT_CTRL_FORCE_LED_OFF: c_uint = 0x0008;
pub const MII_TG3_EXT_CTRL_TBI: c_uint = 0x8000;
pub const MII_TG3_EXT_STAT: c_uint = 0x11 /* Extended status register */;
pub const MII_TG3_EXT_STAT_MDIX: c_uint = 0x2000;
pub const MII_TG3_EXT_STAT_LPASS: c_uint = 0x0100;
pub const MII_TG3_RXR_COUNTERS: c_uint = 0x14 /* Local/Remote Receiver Counts */;
pub const MII_TG3_DSP_RW_PORT: c_uint = 0x15 /* DSP coefficient read/write port */;
pub const MII_TG3_DSP_CONTROL: c_uint = 0x16 /* DSP control register */;
pub const MII_TG3_DSP_ADDRESS: c_uint = 0x17 /* DSP address register */;
pub const MII_TG3_DSP_TAP1: c_uint = 0x0001;
pub const MII_TG3_DSP_TAP1_AGCTGT_DFLT: c_uint = 0x0007;
pub const MII_TG3_DSP_TAP26: c_uint = 0x001a;
pub const MII_TG3_DSP_TAP26_ALNOKO: c_uint = 0x0001;
pub const MII_TG3_DSP_TAP26_RMRXSTO: c_uint = 0x0002;
pub const MII_TG3_DSP_TAP26_OPCSINPT: c_uint = 0x0004;
pub const MII_TG3_DSP_AADJ1CH0: c_uint = 0x001f;
pub const MII_TG3_DSP_CH34TP2: c_uint = 0x4022;
pub const MII_TG3_DSP_CH34TP2_HIBW01: c_uint = 0x01ff;
pub const MII_TG3_DSP_AADJ1CH3: c_uint = 0x601f;
pub const MII_TG3_DSP_AADJ1CH3_ADCCKADJ: c_uint = 0x0002;
pub const MII_TG3_DSP_EXP1_INT_STAT: c_uint = 0x0f01;
pub const MII_TG3_DSP_EXP8: c_uint = 0x0f08;
pub const MII_TG3_DSP_EXP8_REJ2MHz: c_uint = 0x0001;
pub const MII_TG3_DSP_EXP8_AEDW: c_uint = 0x0200;
pub const MII_TG3_DSP_EXP75: c_uint = 0x0f75;
pub const MII_TG3_DSP_EXP96: c_uint = 0x0f96;
pub const MII_TG3_DSP_EXP97: c_uint = 0x0f97;
pub const MII_TG3_AUX_CTRL: c_uint = 0x18 /* auxiliary control register */;
pub const MII_TG3_AUXCTL_SHDWSEL_AUXCTL: c_uint = 0x0000;
pub const MII_TG3_AUXCTL_ACTL_TX_6DB: c_uint = 0x0400;
pub const MII_TG3_AUXCTL_ACTL_SMDSP_ENA: c_uint = 0x0800;
pub const MII_TG3_AUXCTL_ACTL_EXTPKTLEN: c_uint = 0x4000;
pub const MII_TG3_AUXCTL_ACTL_EXTLOOPBK: c_uint = 0x8000;
pub const MII_TG3_AUXCTL_SHDWSEL_PWRCTL: c_uint = 0x0002;
pub const MII_TG3_AUXCTL_PCTL_WOL_EN: c_uint = 0x0008;
pub const MII_TG3_AUXCTL_PCTL_100TX_LPWR: c_uint = 0x0010;
pub const MII_TG3_AUXCTL_PCTL_SPR_ISOLATE: c_uint = 0x0020;
pub const MII_TG3_AUXCTL_PCTL_CL_AB_TXDAC: c_uint = 0x0040;
pub const MII_TG3_AUXCTL_PCTL_VREG_11V: c_uint = 0x0180;
pub const MII_TG3_AUXCTL_SHDWSEL_MISCTEST: c_uint = 0x0004;
pub const MII_TG3_AUXCTL_SHDWSEL_MISC: c_uint = 0x0007;
pub const MII_TG3_AUXCTL_MISC_WIRESPD_EN: c_uint = 0x0010;
pub const MII_TG3_AUXCTL_MISC_FORCE_AMDIX: c_uint = 0x0200;
pub const MII_TG3_AUXCTL_MISC_RDSEL_SHIFT: c_int = 12;
pub const MII_TG3_AUXCTL_MISC_WREN: c_uint = 0x8000;
pub const MII_TG3_AUX_STAT: c_uint = 0x19 /* auxiliary status register */;
pub const MII_TG3_AUX_STAT_LPASS: c_uint = 0x0004;
pub const MII_TG3_AUX_STAT_SPDMASK: c_uint = 0x0700;
pub const MII_TG3_AUX_STAT_10HALF: c_uint = 0x0100;
pub const MII_TG3_AUX_STAT_10FULL: c_uint = 0x0200;
pub const MII_TG3_AUX_STAT_100HALF: c_uint = 0x0300;
pub const MII_TG3_AUX_STAT_100_4: c_uint = 0x0400;
pub const MII_TG3_AUX_STAT_100FULL: c_uint = 0x0500;
pub const MII_TG3_AUX_STAT_1000HALF: c_uint = 0x0600;
pub const MII_TG3_AUX_STAT_1000FULL: c_uint = 0x0700;
pub const MII_TG3_AUX_STAT_100: c_uint = 0x0008;
pub const MII_TG3_AUX_STAT_FULL: c_uint = 0x0001;
pub const MII_TG3_ISTAT: c_uint = 0x1a /* IRQ status register */;
pub const MII_TG3_IMASK: c_uint = 0x1b /* IRQ mask register */;
// ISTAT/IMASK event bits
pub const MII_TG3_INT_LINKCHG: c_uint = 0x0002;
pub const MII_TG3_INT_SPEEDCHG: c_uint = 0x0004;
pub const MII_TG3_INT_DUPLEXCHG: c_uint = 0x0008;
pub const MII_TG3_INT_ANEG_PAGE_RX: c_uint = 0x0400;
pub const MII_TG3_MISC_SHDW: c_uint = 0x1c;
pub const MII_TG3_MISC_SHDW_WREN: c_uint = 0x8000;
pub const MII_TG3_MISC_SHDW_APD_WKTM_84MS: c_uint = 0x0001;
pub const MII_TG3_MISC_SHDW_APD_ENABLE: c_uint = 0x0020;
pub const MII_TG3_MISC_SHDW_APD_SEL: c_uint = 0x2800;
pub const MII_TG3_MISC_SHDW_SCR5_C125OE: c_uint = 0x0001;
pub const MII_TG3_MISC_SHDW_SCR5_DLLAPD: c_uint = 0x0002;
pub const MII_TG3_MISC_SHDW_SCR5_SDTL: c_uint = 0x0004;
pub const MII_TG3_MISC_SHDW_SCR5_DLPTLM: c_uint = 0x0008;
pub const MII_TG3_MISC_SHDW_SCR5_LPED: c_uint = 0x0010;
pub const MII_TG3_MISC_SHDW_SCR5_SEL: c_uint = 0x1400;
pub const MII_TG3_TEST1: c_uint = 0x1e;
pub const MII_TG3_TEST1_TRIM_EN: c_uint = 0x0010;
pub const MII_TG3_TEST1_CRC_EN: c_uint = 0x8000;
// Clause 45 expansion registers
pub const TG3_CL45_D7_EEERES_STAT: c_uint = 0x803e;
pub const TG3_CL45_D7_EEERES_STAT_LP_100TX: c_uint = 0x0002;
pub const TG3_CL45_D7_EEERES_STAT_LP_1000T: c_uint = 0x0004;
// Fast Ethernet Transceiver definitions
pub const MII_TG3_FET_PTEST: c_uint = 0x17;
pub const MII_TG3_FET_PTEST_TRIM_SEL: c_uint = 0x0010;
pub const MII_TG3_FET_PTEST_TRIM_2: c_uint = 0x0002;
pub const MII_TG3_FET_PTEST_FRC_TX_LINK: c_uint = 0x1000;
pub const MII_TG3_FET_PTEST_FRC_TX_LOCK: c_uint = 0x0800;
pub const MII_TG3_FET_GEN_STAT: c_uint = 0x1c;
pub const MII_TG3_FET_GEN_STAT_MDIXSTAT: c_uint = 0x2000;
pub const MII_TG3_FET_TEST: c_uint = 0x1f;
pub const MII_TG3_FET_SHADOW_EN: c_uint = 0x0080;
pub const MII_TG3_FET_SHDW_MISCCTRL: c_uint = 0x10;
pub const MII_TG3_FET_SHDW_MISCCTRL_MDIX: c_uint = 0x4000;
pub const MII_TG3_FET_SHDW_AUXMODE4: c_uint = 0x1a;
pub const MII_TG3_FET_SHDW_AUXMODE4_SBPD: c_uint = 0x0008;
pub const MII_TG3_FET_SHDW_AUXSTAT2: c_uint = 0x1b;
pub const MII_TG3_FET_SHDW_AUXSTAT2_APD: c_uint = 0x0020;
// Serdes PHY Register Definitions
pub const SERDES_TG3_1000X_STATUS: c_uint = 0x14;
pub const SERDES_TG3_SGMII_MODE: c_uint = 0x0001;
pub const SERDES_TG3_LINK_UP: c_uint = 0x0002;
pub const SERDES_TG3_FULL_DUPLEX: c_uint = 0x0004;
pub const SERDES_TG3_SPEED_100: c_uint = 0x0008;
pub const SERDES_TG3_SPEED_1000: c_uint = 0x0010;
// APE registers.  Accessible through BAR1
pub const TG3_APE_GPIO_MSG: c_uint = 0x0008;
pub const TG3_APE_GPIO_MSG_SHIFT: c_int = 4;
pub const TG3_APE_EVENT: c_uint = 0x000c;
pub const APE_EVENT_1: c_uint = 0x00000001;
pub const TG3_APE_LOCK_REQ: c_uint = 0x002c;
pub const APE_LOCK_REQ_DRIVER: c_uint = 0x00001000;
pub const TG3_APE_LOCK_GRANT: c_uint = 0x004c;
pub const APE_LOCK_GRANT_DRIVER: c_uint = 0x00001000;
pub const TG3_APE_OTP_CTRL: c_uint = 0x00e8;
pub const APE_OTP_CTRL_PROG_EN: c_uint = 0x200000;
pub const APE_OTP_CTRL_CMD_RD: c_uint = 0x000000;
pub const APE_OTP_CTRL_START: c_uint = 0x000001;
pub const TG3_APE_OTP_STATUS: c_uint = 0x00ec;
pub const APE_OTP_STATUS_CMD_DONE: c_uint = 0x000001;
pub const TG3_APE_OTP_ADDR: c_uint = 0x00f0;
pub const APE_OTP_ADDR_CPU_ENABLE: c_uint = 0x80000000;
pub const TG3_APE_OTP_RD_DATA: c_uint = 0x00f8;
pub const OTP_ADDRESS_MAGIC0: c_uint = 0x00000050;

// APE shared memory.  Accessible through BAR1
pub const TG3_APE_SHMEM_BASE: c_uint = 0x4000;
pub const TG3_APE_SEG_SIG: c_uint = 0x4000;
pub const APE_SEG_SIG_MAGIC: c_uint = 0x41504521;
pub const TG3_APE_FW_STATUS: c_uint = 0x400c;
pub const APE_FW_STATUS_READY: c_uint = 0x00000100;
pub const TG3_APE_FW_FEATURES: c_uint = 0x4010;
pub const TG3_APE_FW_FEATURE_NCSI: c_uint = 0x00000002;
pub const TG3_APE_FW_VERSION: c_uint = 0x4018;
pub const APE_FW_VERSION_MAJMSK: c_uint = 0xff000000;
pub const APE_FW_VERSION_MAJSFT: c_int = 24;
pub const APE_FW_VERSION_MINMSK: c_uint = 0x00ff0000;
pub const APE_FW_VERSION_MINSFT: c_int = 16;
pub const APE_FW_VERSION_REVMSK: c_uint = 0x0000ff00;
pub const APE_FW_VERSION_REVSFT: c_int = 8;
pub const APE_FW_VERSION_BLDMSK: c_uint = 0x000000ff;
pub const TG3_APE_SEG_MSG_BUF_OFF: c_uint = 0x401c;
pub const TG3_APE_SEG_MSG_BUF_LEN: c_uint = 0x4020;
pub const TG3_APE_HOST_SEG_SIG: c_uint = 0x4200;
pub const APE_HOST_SEG_SIG_MAGIC: c_uint = 0x484f5354;
pub const TG3_APE_HOST_SEG_LEN: c_uint = 0x4204;
pub const APE_HOST_SEG_LEN_MAGIC: c_uint = 0x00000020;
pub const TG3_APE_HOST_INIT_COUNT: c_uint = 0x4208;
pub const TG3_APE_HOST_DRIVER_ID: c_uint = 0x420c;
pub const APE_HOST_DRIVER_ID_LINUX: c_uint = 0xf0000000;

pub const TG3_APE_HOST_BEHAVIOR: c_uint = 0x4210;
pub const APE_HOST_BEHAV_NO_PHYLOCK: c_uint = 0x00000001;
pub const TG3_APE_HOST_HEARTBEAT_INT_MS: c_uint = 0x4214;
pub const APE_HOST_HEARTBEAT_INT_DISABLE: c_int = 0;
pub const APE_HOST_HEARTBEAT_INT_5SEC: c_int = 5000;
pub const TG3_APE_HOST_HEARTBEAT_COUNT: c_uint = 0x4218;
pub const TG3_APE_HOST_DRVR_STATE: c_uint = 0x421c;
pub const TG3_APE_HOST_DRVR_STATE_START: c_uint = 0x00000001;
pub const TG3_APE_HOST_DRVR_STATE_UNLOAD: c_uint = 0x00000002;
pub const TG3_APE_HOST_DRVR_STATE_WOL: c_uint = 0x00000003;
pub const TG3_APE_HOST_WOL_SPEED: c_uint = 0x4224;
pub const TG3_APE_HOST_WOL_SPEED_AUTO: c_uint = 0x00008000;
pub const TG3_APE_EVENT_STATUS: c_uint = 0x4300;
pub const APE_EVENT_STATUS_DRIVER_EVNT: c_uint = 0x00000010;
pub const APE_EVENT_STATUS_STATE_CHNGE: c_uint = 0x00000500;
pub const APE_EVENT_STATUS_SCRTCHPD_READ: c_uint = 0x00001600;
pub const APE_EVENT_STATUS_SCRTCHPD_WRITE: c_uint = 0x00001700;
pub const APE_EVENT_STATUS_STATE_START: c_uint = 0x00010000;
pub const APE_EVENT_STATUS_STATE_UNLOAD: c_uint = 0x00020000;
pub const APE_EVENT_STATUS_STATE_WOL: c_uint = 0x00030000;
pub const APE_EVENT_STATUS_STATE_SUSPEND: c_uint = 0x00040000;
pub const APE_EVENT_STATUS_EVENT_PENDING: c_uint = 0x80000000;
pub const TG3_APE_PER_LOCK_REQ: c_uint = 0x8400;
pub const APE_LOCK_PER_REQ_DRIVER: c_uint = 0x00001000;
pub const TG3_APE_PER_LOCK_GRANT: c_uint = 0x8420;
pub const APE_PER_LOCK_GRANT_DRIVER: c_uint = 0x00001000;
// APE convenience enumerations.
pub const TG3_APE_LOCK_PHY0: c_int = 0;
pub const TG3_APE_LOCK_GRC: c_int = 1;
pub const TG3_APE_LOCK_PHY1: c_int = 2;
pub const TG3_APE_LOCK_PHY2: c_int = 3;
pub const TG3_APE_LOCK_MEM: c_int = 4;
pub const TG3_APE_LOCK_PHY3: c_int = 5;
pub const TG3_APE_LOCK_GPIO: c_int = 7;

pub const TG3_EEPROM_SB_F1R2_MBA_OFF: c_uint = 0x10;
// There are two ways to manage the TX descriptors on the tigon3.
// Either the descriptors are in host DMA'able memory, or they
// exist only in the cards on-chip SRAM.  All 16 send bds are under
// the same mode, they may not be configured individually.
//
// This driver always uses host memory TX descriptors.
//
// To use host memory TX descriptors:
// 1) Set GRC_MODE_HOST_SENDBDS in GRC_MODE register.
// Make sure GRC_MODE_4X_NIC_SEND_RINGS is clear.
// 2) Allocate DMA'able memory.
// 3) In NIC_SRAM_SEND_RCB (of desired index) of on-chip SRAM:
// a) Set TG3_BDINFO_HOST_ADDR to DMA address of memory
// obtained in step 2
// b) Set TG3_BDINFO_NIC_ADDR to NIC_SRAM_TX_BUFFER_DESC.
// c) Set len field of TG3_BDINFO_MAXLEN_FLAGS to number
// of TX descriptors.  Leave flags field clear.
// 4) Access TX descriptors via host memory.  The chip
// will refetch into local SRAM as needed when producer
// index mailboxes are updated.
//
// To use on-chip TX descriptors:
// 1) Set GRC_MODE_4X_NIC_SEND_RINGS in GRC_MODE register.
// Make sure GRC_MODE_HOST_SENDBDS is clear.
// 2) In NIC_SRAM_SEND_RCB (of desired index) of on-chip SRAM:
// a) Set TG3_BDINFO_HOST_ADDR to zero.
// b) Set TG3_BDINFO_NIC_ADDR to NIC_SRAM_TX_BUFFER_DESC
// c) TG3_BDINFO_MAXLEN_FLAGS is don't care.
// 3) Access TX descriptors directly in on-chip SRAM
// using normal {read,write}l().  (and not using
// pointer dereferencing of ioremap()'d memory like
// the broken Broadcom driver does)
//
// Note that BDINFO_FLAGS_DISABLED should be set in the flags field of
// TG3_BDINFO_MAXLEN_FLAGS of all unused SEND_RCB indices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_tx_buffer_desc {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub len_flags: u32,
pub const TXD_FLAG_TCPUDP_CSUM: c_uint = 0x0001;
pub const TXD_FLAG_IP_CSUM: c_uint = 0x0002;
pub const TXD_FLAG_END: c_uint = 0x0004;
pub const TXD_FLAG_IP_FRAG: c_uint = 0x0008;
pub const TXD_FLAG_JMB_PKT: c_uint = 0x0008;
pub const TXD_FLAG_IP_FRAG_END: c_uint = 0x0010;
pub const TXD_FLAG_HWTSTAMP: c_uint = 0x0020;
pub const TXD_FLAG_VLAN: c_uint = 0x0040;
pub const TXD_FLAG_COAL_NOW: c_uint = 0x0080;
pub const TXD_FLAG_CPU_PRE_DMA: c_uint = 0x0100;
pub const TXD_FLAG_CPU_POST_DMA: c_uint = 0x0200;
pub const TXD_FLAG_ADD_SRC_ADDR: c_uint = 0x1000;
pub const TXD_FLAG_CHOOSE_SRC_ADDR: c_uint = 0x6000;
pub const TXD_FLAG_NO_CRC: c_uint = 0x8000;
pub const TXD_LEN_SHIFT: c_int = 16;
    pub vlan_tag: u32,
pub const TXD_VLAN_TAG_SHIFT: c_int = 0;
pub const TXD_MSS_SHIFT: c_int = 16;
}

pub const TXD_ADDR: c_uint = 0x00UL /* 64-bit */;
pub const TXD_LEN_FLAGS: c_uint = 0x08UL /* 32-bit (upper 16-bits are len) */;
pub const TXD_VLAN_TAG: c_uint = 0x0cUL /* 32-bit (upper 16-bits are tag) */;
pub const TXD_SIZE: c_uint = 0x10UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_rx_buffer_desc {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub idx_len: u32,
pub const RXD_IDX_MASK: c_uint = 0xffff0000;
pub const RXD_IDX_SHIFT: c_int = 16;
pub const RXD_LEN_MASK: c_uint = 0x0000ffff;
pub const RXD_LEN_SHIFT: c_int = 0;
    pub type_flags: u32,
pub const RXD_TYPE_SHIFT: c_int = 16;
pub const RXD_FLAGS_SHIFT: c_int = 0;
pub const RXD_FLAG_END: c_uint = 0x0004;
pub const RXD_FLAG_MINI: c_uint = 0x0800;
pub const RXD_FLAG_JUMBO: c_uint = 0x0020;
pub const RXD_FLAG_VLAN: c_uint = 0x0040;
pub const RXD_FLAG_ERROR: c_uint = 0x0400;
pub const RXD_FLAG_IP_CSUM: c_uint = 0x1000;
pub const RXD_FLAG_TCPUDP_CSUM: c_uint = 0x2000;
pub const RXD_FLAG_IS_TCP: c_uint = 0x4000;
pub const RXD_FLAG_PTPSTAT_MASK: c_uint = 0x0210;
pub const RXD_FLAG_PTPSTAT_PTPV1: c_uint = 0x0010;
pub const RXD_FLAG_PTPSTAT_PTPV2: c_uint = 0x0200;
    pub ip_tcp_csum: u32,
pub const RXD_IPCSUM_MASK: c_uint = 0xffff0000;
pub const RXD_IPCSUM_SHIFT: c_int = 16;
pub const RXD_TCPCSUM_MASK: c_uint = 0x0000ffff;
pub const RXD_TCPCSUM_SHIFT: c_int = 0;
    pub err_vlan: u32,
pub const RXD_VLAN_MASK: c_uint = 0x0000ffff;
pub const RXD_ERR_BAD_CRC: c_uint = 0x00010000;
pub const RXD_ERR_COLLISION: c_uint = 0x00020000;
pub const RXD_ERR_LINK_LOST: c_uint = 0x00040000;
pub const RXD_ERR_PHY_DECODE: c_uint = 0x00080000;
pub const RXD_ERR_ODD_NIBBLE_RCVD_MII: c_uint = 0x00100000;
pub const RXD_ERR_MAC_ABRT: c_uint = 0x00200000;
pub const RXD_ERR_TOO_SMALL: c_uint = 0x00400000;
pub const RXD_ERR_NO_RESOURCES: c_uint = 0x00800000;
pub const RXD_ERR_HUGE_FRAME: c_uint = 0x01000000;

    pub reserved: u32,
    pub opaque: u32,
pub const RXD_OPAQUE_INDEX_MASK: c_uint = 0x0000ffff;
pub const RXD_OPAQUE_INDEX_SHIFT: c_int = 0;
pub const RXD_OPAQUE_RING_STD: c_uint = 0x00010000;
pub const RXD_OPAQUE_RING_JUMBO: c_uint = 0x00020000;
pub const RXD_OPAQUE_RING_MINI: c_uint = 0x00040000;
pub const RXD_OPAQUE_RING_MASK: c_uint = 0x00070000;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_ext_rx_buffer_desc {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub addrlist: [}; 3],
    pub len2_len1: u32,
    pub resv_len3: u32,
    pub std: tg3_rx_buffer_desc,
}

// We only use this when testing out the DMA engine
// at probe time.  This is the internal format of buffer
// descriptors used by the chip at NIC_SRAM_DMA_DESCS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_internal_buffer_desc {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub nic_mbuf: u32,
// XXX FIX THIS

    pub cqid_sqid: u16,
    pub len: u16,

    pub len: u16,
    pub cqid_sqid: u16,

    pub flags: u32,
    pub __cookie1: u32,
    pub __cookie2: u32,
    pub __cookie3: u32,
}

pub const TG3_HW_STATUS_SIZE: c_uint = 0x50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_hw_status {
    pub status: u32,
pub const SD_STATUS_UPDATED: c_uint = 0x00000001;
pub const SD_STATUS_LINK_CHG: c_uint = 0x00000002;
pub const SD_STATUS_ERROR: c_uint = 0x00000004;
    pub status_tag: u32,

    pub rx_consumer: u16,
    pub rx_jumbo_consumer: u16,

    pub rx_jumbo_consumer: u16,
    pub rx_consumer: u16,

    pub reserved: u16,
    pub rx_mini_consumer: u16,

    pub rx_mini_consumer: u16,
    pub reserved: u16,

    pub tx_consumer: u16,
    pub rx_producer: u16,

    pub rx_producer: u16,
    pub tx_consumer: u16,
    pub idx: [}; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_hw_stats {
    pub __reserved0: [u8; 0x400-0x300],
// Statistics maintained by Receive MAC.
    pub rx_octets: tg3_stat64_t,
    pub __reserved1: u64,
    pub rx_fragments: tg3_stat64_t,
    pub rx_ucast_packets: tg3_stat64_t,
    pub rx_mcast_packets: tg3_stat64_t,
    pub rx_bcast_packets: tg3_stat64_t,
    pub rx_fcs_errors: tg3_stat64_t,
    pub rx_align_errors: tg3_stat64_t,
    pub rx_xon_pause_rcvd: tg3_stat64_t,
    pub rx_xoff_pause_rcvd: tg3_stat64_t,
    pub rx_mac_ctrl_rcvd: tg3_stat64_t,
    pub rx_xoff_entered: tg3_stat64_t,
    pub rx_frame_too_long_errors: tg3_stat64_t,
    pub rx_jabbers: tg3_stat64_t,
    pub rx_undersize_packets: tg3_stat64_t,
    pub rx_in_length_errors: tg3_stat64_t,
    pub rx_out_length_errors: tg3_stat64_t,
    pub rx_64_or_less_octet_packets: tg3_stat64_t,
    pub rx_65_to_127_octet_packets: tg3_stat64_t,
    pub rx_128_to_255_octet_packets: tg3_stat64_t,
    pub rx_256_to_511_octet_packets: tg3_stat64_t,
    pub rx_512_to_1023_octet_packets: tg3_stat64_t,
    pub rx_1024_to_1522_octet_packets: tg3_stat64_t,
    pub rx_1523_to_2047_octet_packets: tg3_stat64_t,
    pub rx_2048_to_4095_octet_packets: tg3_stat64_t,
    pub rx_4096_to_8191_octet_packets: tg3_stat64_t,
    pub rx_8192_to_9022_octet_packets: tg3_stat64_t,
    pub __unused0: [u64; 37],
// Statistics maintained by Transmit MAC.
    pub tx_octets: tg3_stat64_t,
    pub __reserved2: u64,
    pub tx_collisions: tg3_stat64_t,
    pub tx_xon_sent: tg3_stat64_t,
    pub tx_xoff_sent: tg3_stat64_t,
    pub tx_flow_control: tg3_stat64_t,
    pub tx_mac_errors: tg3_stat64_t,
    pub tx_single_collisions: tg3_stat64_t,
    pub tx_mult_collisions: tg3_stat64_t,
    pub tx_deferred: tg3_stat64_t,
    pub __reserved3: u64,
    pub tx_excessive_collisions: tg3_stat64_t,
    pub tx_late_collisions: tg3_stat64_t,
    pub tx_collide_2times: tg3_stat64_t,
    pub tx_collide_3times: tg3_stat64_t,
    pub tx_collide_4times: tg3_stat64_t,
    pub tx_collide_5times: tg3_stat64_t,
    pub tx_collide_6times: tg3_stat64_t,
    pub tx_collide_7times: tg3_stat64_t,
    pub tx_collide_8times: tg3_stat64_t,
    pub tx_collide_9times: tg3_stat64_t,
    pub tx_collide_10times: tg3_stat64_t,
    pub tx_collide_11times: tg3_stat64_t,
    pub tx_collide_12times: tg3_stat64_t,
    pub tx_collide_13times: tg3_stat64_t,
    pub tx_collide_14times: tg3_stat64_t,
    pub tx_collide_15times: tg3_stat64_t,
    pub tx_ucast_packets: tg3_stat64_t,
    pub tx_mcast_packets: tg3_stat64_t,
    pub tx_bcast_packets: tg3_stat64_t,
    pub tx_carrier_sense_errors: tg3_stat64_t,
    pub tx_discards: tg3_stat64_t,
    pub tx_errors: tg3_stat64_t,
    pub __unused1: [u64; 31],
// Statistics maintained by Receive List Placement.
    pub COS_rx_packets: [tg3_stat64_t; 16],
    pub COS_rx_filter_dropped: tg3_stat64_t,
    pub dma_writeq_full: tg3_stat64_t,
    pub dma_write_prioq_full: tg3_stat64_t,
    pub rxbds_empty: tg3_stat64_t,
    pub rx_discards: tg3_stat64_t,
    pub rx_errors: tg3_stat64_t,
    pub rx_threshold_hit: tg3_stat64_t,
    pub __unused2: [u64; 9],
// Statistics maintained by Send Data Initiator.
    pub COS_out_packets: [tg3_stat64_t; 16],
    pub dma_readq_full: tg3_stat64_t,
    pub dma_read_prioq_full: tg3_stat64_t,
    pub tx_comp_queue_full: tg3_stat64_t,
// Statistics maintained by Host Coalescing.
    pub ring_set_send_prod_index: tg3_stat64_t,
    pub ring_status_update: tg3_stat64_t,
    pub nic_irqs: tg3_stat64_t,
    pub nic_avoided_irqs: tg3_stat64_t,
    pub nic_tx_threshold_hit: tg3_stat64_t,
// NOT a part of the hardware statistics block format.
// These stats are here as storage for tg3_periodic_fetch_stats().
//
    pub mbuf_lwm_thresh_hit: tg3_stat64_t,
    pub __reserved4: [u8; 0xb00-0x9c8],
}

pub const TG3_SD_NUM_RECS: c_int = 3;

pub const TG3_OCIR_SIG_MAGIC: c_uint = 0x5253434f;
pub const TG3_OCIR_FLAG_ACTIVE: c_uint = 0x00000001;
pub const TG3_TEMP_CAUTION_OFFSET: c_uint = 0xc8;
pub const TG3_TEMP_MAX_OFFSET: c_uint = 0xcc;
pub const TG3_TEMP_SENSOR_OFFSET: c_uint = 0xd4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_ocir {
    pub signature: u32,
    pub version_flags: u16,
    pub refresh_int: u16,
    pub refresh_tmr: u32,
    pub update_tmr: u32,
    pub dst_base_addr: u32,
    pub src_hdr_offset: u16,
    pub src_hdr_length: u16,
    pub src_data_offset: u16,
    pub src_data_length: u16,
    pub dst_hdr_offset: u16,
    pub dst_data_offset: u16,
    pub dst_reg_upd_offset: u16,
    pub dst_sem_offset: u16,
    pub reserved1: [u32; 2],
    pub port0_flags: u32,
    pub port1_flags: u32,
    pub port2_flags: u32,
    pub port3_flags: u32,
    pub reserved2: u32,
}

// 'mapping' is superfluous as the chip does not write into
// the tx/rx post rings so we could just fetch it from there.
// But the cache behavior is better how we are doing it now.
//
// This driver uses new build_skb() API :
// RX ring buffer contains pointer to kmalloc() data only,
// skb are built only after Hardware filled the frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_info {
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_tx_ring_info {
    pub skb: *mut sk_buff,
    pub fragmented: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_link_config {
// Describes what we're trying to get.
    pub advertising: u32,
    pub speed: u32,
    pub duplex: u8,
    pub autoneg: u8,
    pub flowctrl: u8,
// Describes what we actually have.
    pub active_flowctrl: u8,
    pub active_duplex: u8,
    pub active_speed: u32,
    pub rmt_adv: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_bufmgr_config {
    pub mbuf_read_dma_low_water: u32,
    pub mbuf_mac_rx_low_water: u32,
    pub mbuf_high_water: u32,
    pub mbuf_read_dma_low_water_jumbo: u32,
    pub mbuf_mac_rx_low_water_jumbo: u32,
    pub mbuf_high_water_jumbo: u32,
    pub dma_low_water: u32,
    pub dma_high_water: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_ethtool_stats {
// Statistics maintained by Receive MAC.
    pub rx_octets: u64,
    pub rx_fragments: u64,
    pub rx_ucast_packets: u64,
    pub rx_mcast_packets: u64,
    pub rx_bcast_packets: u64,
    pub rx_fcs_errors: u64,
    pub rx_align_errors: u64,
    pub rx_xon_pause_rcvd: u64,
    pub rx_xoff_pause_rcvd: u64,
    pub rx_mac_ctrl_rcvd: u64,
    pub rx_xoff_entered: u64,
    pub rx_frame_too_long_errors: u64,
    pub rx_jabbers: u64,
    pub rx_undersize_packets: u64,
    pub rx_in_length_errors: u64,
    pub rx_out_length_errors: u64,
    pub rx_64_or_less_octet_packets: u64,
    pub rx_65_to_127_octet_packets: u64,
    pub rx_128_to_255_octet_packets: u64,
    pub rx_256_to_511_octet_packets: u64,
    pub rx_512_to_1023_octet_packets: u64,
    pub rx_1024_to_1522_octet_packets: u64,
    pub rx_1523_to_2047_octet_packets: u64,
    pub rx_2048_to_4095_octet_packets: u64,
    pub rx_4096_to_8191_octet_packets: u64,
    pub rx_8192_to_9022_octet_packets: u64,
// Statistics maintained by Transmit MAC.
    pub tx_octets: u64,
    pub tx_collisions: u64,
    pub tx_xon_sent: u64,
    pub tx_xoff_sent: u64,
    pub tx_flow_control: u64,
    pub tx_mac_errors: u64,
    pub tx_single_collisions: u64,
    pub tx_mult_collisions: u64,
    pub tx_deferred: u64,
    pub tx_excessive_collisions: u64,
    pub tx_late_collisions: u64,
    pub tx_collide_2times: u64,
    pub tx_collide_3times: u64,
    pub tx_collide_4times: u64,
    pub tx_collide_5times: u64,
    pub tx_collide_6times: u64,
    pub tx_collide_7times: u64,
    pub tx_collide_8times: u64,
    pub tx_collide_9times: u64,
    pub tx_collide_10times: u64,
    pub tx_collide_11times: u64,
    pub tx_collide_12times: u64,
    pub tx_collide_13times: u64,
    pub tx_collide_14times: u64,
    pub tx_collide_15times: u64,
    pub tx_ucast_packets: u64,
    pub tx_mcast_packets: u64,
    pub tx_bcast_packets: u64,
    pub tx_carrier_sense_errors: u64,
    pub tx_discards: u64,
    pub tx_errors: u64,
// Statistics maintained by Receive List Placement.
    pub dma_writeq_full: u64,
    pub dma_write_prioq_full: u64,
    pub rxbds_empty: u64,
    pub rx_discards: u64,
    pub rx_errors: u64,
    pub rx_threshold_hit: u64,
// Statistics maintained by Send Data Initiator.
    pub dma_readq_full: u64,
    pub dma_read_prioq_full: u64,
    pub tx_comp_queue_full: u64,
// Statistics maintained by Host Coalescing.
    pub ring_set_send_prod_index: u64,
    pub ring_status_update: u64,
    pub nic_irqs: u64,
    pub nic_avoided_irqs: u64,
    pub nic_tx_threshold_hit: u64,
    pub mbuf_lwm_thresh_hit: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_rx_prodring_set {
    pub rx_std_prod_idx: u32,
    pub rx_std_cons_idx: u32,
    pub rx_jmb_prod_idx: u32,
    pub rx_jmb_cons_idx: u32,
    pub rx_std: *mut tg3_rx_buffer_desc,
    pub rx_jmb: *mut tg3_ext_rx_buffer_desc,
    pub rx_std_buffers: *mut ring_info,
    pub rx_jmb_buffers: *mut ring_info,
    pub rx_std_mapping: dma_addr_t,
    pub rx_jmb_mapping: dma_addr_t,
}

pub const TG3_RSS_MAX_NUM_QS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_napi {
    pub ____cacheline_aligned: napi_napi,
    pub tp: *mut tg3,
    pub hw_status: *mut tg3_hw_status,
    pub chk_msi_cnt: u32,
    pub last_tag: u32,
    pub last_irq_tag: u32,
    pub int_mbox: u32,
    pub coal_now: u32,
    pub ____cacheline_aligned: u32 consmbox,
    pub rx_rcb_ptr: u32,
    pub last_rx_cons: u32,
    pub rx_rcb_prod_idx: *mut u16,
    pub prodring: tg3_rx_prodring_set,
    pub rx_rcb: *mut tg3_rx_buffer_desc,
    pub rx_dropped: c_ulong,
    pub ____cacheline_aligned: u32 tx_prod,
    pub tx_cons: u32,
    pub tx_pending: u32,
    pub last_tx_cons: u32,
    pub prodmbox: u32,
    pub tx_ring: *mut tg3_tx_buffer_desc,
    pub tx_buffers: *mut tg3_tx_ring_info,
    pub tx_dropped: c_ulong,
    pub status_mapping: dma_addr_t,
    pub rx_rcb_mapping: dma_addr_t,
    pub tx_desc_mapping: dma_addr_t,
    pub /: *mut *mut char irq_lbl[IFNAMSIZ + 6 + 10]; / name + "-txrx-" + %d,
    pub irq_vec: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TG3_FLAGS {
    TG3_FLAG_TAGGED_STATUS = 0,
    TG3_FLAG_TXD_MBOX_HWBUG,
    TG3_FLAG_USE_LINKCHG_REG,
    TG3_FLAG_ERROR_PROCESSED,
    TG3_FLAG_ENABLE_ASF,
    TG3_FLAG_ASPM_WORKAROUND,
    TG3_FLAG_POLL_SERDES,
    TG3_FLAG_POLL_CPMU_LINK,
    TG3_FLAG_MBOX_WRITE_REORDER,
    TG3_FLAG_PCIX_TARGET_HWBUG,
    TG3_FLAG_WOL_SPEED_100MB,
    TG3_FLAG_WOL_ENABLE,
    TG3_FLAG_EEPROM_WRITE_PROT,
    TG3_FLAG_NVRAM,
    TG3_FLAG_NVRAM_BUFFERED,
    TG3_FLAG_SUPPORT_MSI,
    TG3_FLAG_SUPPORT_MSIX,
    TG3_FLAG_USING_MSI,
    TG3_FLAG_USING_MSIX,
    TG3_FLAG_PCIX_MODE,
    TG3_FLAG_PCI_HIGH_SPEED,
    TG3_FLAG_PCI_32BIT,
    TG3_FLAG_SRAM_USE_CONFIG,
    TG3_FLAG_TX_RECOVERY_PENDING,
    TG3_FLAG_WOL_CAP,
    TG3_FLAG_JUMBO_RING_ENABLE,
    TG3_FLAG_PAUSE_AUTONEG,
    TG3_FLAG_CPMU_PRESENT,
    TG3_FLAG_40BIT_DMA_BUG,
    TG3_FLAG_BROKEN_CHECKSUMS,
    TG3_FLAG_JUMBO_CAPABLE,
    TG3_FLAG_CHIP_RESETTING,
    TG3_FLAG_INIT_COMPLETE,
    TG3_FLAG_MAX_RXPEND_64,
    TG3_FLAG_PCI_EXPRESS, /* BCM5785 + pci_is_pcie() */
    TG3_FLAG_ASF_NEW_HANDSHAKE,
    TG3_FLAG_HW_AUTONEG,
    TG3_FLAG_IS_NIC,
    TG3_FLAG_FLASH,
    TG3_FLAG_FW_TSO,
    TG3_FLAG_HW_TSO_1,
    TG3_FLAG_HW_TSO_2,
    TG3_FLAG_HW_TSO_3,
    TG3_FLAG_TSO_CAPABLE,
    TG3_FLAG_TSO_BUG,
    TG3_FLAG_ICH_WORKAROUND,
    TG3_FLAG_1SHOT_MSI,
    TG3_FLAG_NO_FWARE_REPORTED,
    TG3_FLAG_NO_NVRAM_ADDR_TRANS,
    TG3_FLAG_ENABLE_APE,
    TG3_FLAG_PROTECTED_NVRAM,
    TG3_FLAG_5701_DMA_BUG,
    TG3_FLAG_USE_PHYLIB,
    TG3_FLAG_MDIOBUS_INITED,
    TG3_FLAG_LRG_PROD_RING_CAP,
    TG3_FLAG_RGMII_INBAND_DISABLE,
    TG3_FLAG_RGMII_EXT_IBND_RX_EN,
    TG3_FLAG_RGMII_EXT_IBND_TX_EN,
    TG3_FLAG_CLKREQ_BUG,
    TG3_FLAG_NO_NVRAM,
    TG3_FLAG_ENABLE_RSS,
    TG3_FLAG_ENABLE_TSS,
    TG3_FLAG_SHORT_DMA_BUG,
    TG3_FLAG_USE_JUMBO_BDFLAG,
    TG3_FLAG_L1PLLPD_EN,
    TG3_FLAG_APE_HAS_NCSI,
    TG3_FLAG_TX_TSTAMP_EN,
    TG3_FLAG_4K_FIFO_LIMIT,
    TG3_FLAG_5719_5720_RDMA_BUG,
    TG3_FLAG_RESET_TASK_PENDING,
    TG3_FLAG_PTP_CAPABLE,
    TG3_FLAG_5705_PLUS,
    TG3_FLAG_IS_5788,
    TG3_FLAG_5750_PLUS,
    TG3_FLAG_5780_CLASS,
    TG3_FLAG_5755_PLUS,
    TG3_FLAG_57765_PLUS,
    TG3_FLAG_57765_CLASS,
    TG3_FLAG_5717_PLUS,
    TG3_FLAG_IS_SSB_CORE,
    TG3_FLAG_FLUSH_POSTED_WRITES,
    TG3_FLAG_ROBOSWITCH,
    TG3_FLAG_ONE_DMA_AT_ONCE,
    TG3_FLAG_RGMII_MODE,

// Add new flags before this comment and TG3_FLAG_NUMBER_OF_FLAGS
    TG3_FLAG_NUMBER_OF_FLAGS,	/* Last entry in enum TG3_FLAGS */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3_firmware_hdr {
    pub /: *mut *mut __be32 version; / unused for fragments,
    pub base_addr: __be32,
    pub len: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg3 {
// begin "general, frequently-used members" cacheline section
// If the IRQ handler (which runs lockless) needs to be
// quiesced, the following bitmask state is used.  The
// SYNC flag is set by non-IRQ context code to initiate
// the quiescence.
//
// When the IRQ handler notices that SYNC is set, it
// disables interrupts and returns.
//
// When all outstanding IRQ handlers have returned after
// the SYNC flag has been set, the setter can be assured
// that interrupts will no longer get run.
//
// In this way all SMP driver locks are never acquired
// in hw IRQ context, only sw IRQ context or lower.
//
    pub irq_sync: c_uint,
// SMP locking strategy:
//
// lock: Held during reset, PHY access, timer, and when
// updating tg3_flags.
//
// netif_tx_lock: Held during tg3_start_xmit. tg3_tx holds
// netif_tx_lock when it needs to call
// netif_wake_queue.
//
// Both of these locks are to be held with BH safety.
//
// Because the IRQ handler, tg3_poll, and tg3_start_xmit
// are running lockless, it is necessary to completely
// quiesce the chip with tg3_netif_stop and tg3_full_lock
// before reconfiguring the device.
//
// indirect_lock: Held when accessing registers indirectly
// with IRQ disabling.
//
    pub lock: spinlock_t,
    pub indirect_lock: spinlock_t,
    pub u32): *mut *mut *mut u32 (read32) (struct tg3 ,,
    pub u32): *mut *mut *mut void (write32) (struct tg3 , u32,,
    pub u32): *mut *mut *mut u32 (read32_mbox) (struct tg3 ,,
    pub regs: *mut void __iomem,
    pub aperegs: *mut void __iomem,
    pub dev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub coal_now: u32,
    pub msg_enable: u32,
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_adjust: i64,
    pub ptp_txts_retrycnt: u8,
// begin "tx thread" cacheline section
    pub dma_limit: u32,
    pub txq_req: u32,
    pub txq_cnt: u32,
    pub txq_max: u32,
// begin "rx thread" cacheline section
    pub napi: [tg3_napi; TG3_IRQ_MAX_VECS],
    pub rx_copy_thresh: u32,
    pub rx_std_ring_mask: u32,
    pub rx_jmb_ring_mask: u32,
    pub rx_ret_ring_mask: u32,
    pub rx_pending: u32,
    pub rx_jumbo_pending: u32,
    pub rx_std_max_post: u32,
    pub rx_offset: u32,
    pub rx_pkt_map_sz: u32,
    pub rxq_req: u32,
    pub rxq_cnt: u32,
    pub rxq_max: u32,
    pub rx_refill: bool,
// begin "everything else" cacheline(s) section
    pub net_stats_prev: rtnl_link_stats64,
    pub estats_prev: tg3_ethtool_stats,
    pub TG3_FLAG_NUMBER_OF_FLAGS): DECLARE_BITMAP(tg3_flags,,
    pub phy_crc_errors: c_ulong,
    pub last_event_jiffies: c_ulong,
}

// 1 second counter for transient serdes link events
pub const SERDES_AN_TIMEOUT_5704S: c_int = 2;
pub const SERDES_PARALLEL_DET_TIMEOUT: c_int = 1;
pub const SERDES_AN_TIMEOUT_5714S: c_int = 1;
// cache h/w values, often passed straight to h/w
// PCI block
// PHY info
pub const TG3_PHY_ID_MASK: c_uint = 0xfffffff0;
pub const TG3_PHY_ID_BCM5400: c_uint = 0x60008040;
pub const TG3_PHY_ID_BCM5401: c_uint = 0x60008050;
pub const TG3_PHY_ID_BCM5411: c_uint = 0x60008070;
pub const TG3_PHY_ID_BCM5701: c_uint = 0x60008110;
pub const TG3_PHY_ID_BCM5703: c_uint = 0x60008160;
pub const TG3_PHY_ID_BCM5704: c_uint = 0x60008190;
pub const TG3_PHY_ID_BCM5705: c_uint = 0x600081a0;
pub const TG3_PHY_ID_BCM5750: c_uint = 0x60008180;
pub const TG3_PHY_ID_BCM5752: c_uint = 0x60008100;
pub const TG3_PHY_ID_BCM5714: c_uint = 0x60008340;
pub const TG3_PHY_ID_BCM5780: c_uint = 0x60008350;
pub const TG3_PHY_ID_BCM5755: c_uint = 0xbc050cc0;
pub const TG3_PHY_ID_BCM5787: c_uint = 0xbc050ce0;
pub const TG3_PHY_ID_BCM5756: c_uint = 0xbc050ed0;
pub const TG3_PHY_ID_BCM5784: c_uint = 0xbc050fa0;
pub const TG3_PHY_ID_BCM5761: c_uint = 0xbc050fd0;
pub const TG3_PHY_ID_BCM5718C: c_uint = 0x5c0d8a00;
pub const TG3_PHY_ID_BCM5718S: c_uint = 0xbc050ff0;
pub const TG3_PHY_ID_BCM57765: c_uint = 0x5c0d8a40;
pub const TG3_PHY_ID_BCM5719C: c_uint = 0x5c0d8a20;
pub const TG3_PHY_ID_BCM5720C: c_uint = 0x5c0d8b60;
pub const TG3_PHY_ID_BCM5762: c_uint = 0x85803780;
pub const TG3_PHY_ID_BCM5906: c_uint = 0xdc00ac40;
pub const TG3_PHY_ID_BCM8002: c_uint = 0x60010140;
pub const TG3_PHY_ID_INVALID: c_uint = 0xffffffff;
pub const PHY_ID_RTL8211C: c_uint = 0x001cc910;
pub const PHY_ID_RTL8201E: c_uint = 0x00008200;
pub const TG3_PHY_ID_REV_MASK: c_uint = 0x0000000f;
pub const TG3_PHY_REV_BCM5401_B0: c_uint = 0x1;
// This macro assumes the passed PHY ID is
// already masked with TG3_PHY_ID_MASK.
//

pub const TG3_PHYFLG_IS_LOW_POWER: c_uint = 0x00000001;
pub const TG3_PHYFLG_IS_CONNECTED: c_uint = 0x00000002;
pub const TG3_PHYFLG_USE_MI_INTERRUPT: c_uint = 0x00000004;
pub const TG3_PHYFLG_USER_CONFIGURED: c_uint = 0x00000008;
pub const TG3_PHYFLG_PHY_SERDES: c_uint = 0x00000010;
pub const TG3_PHYFLG_MII_SERDES: c_uint = 0x00000020;

pub const TG3_PHYFLG_IS_FET: c_uint = 0x00000040;
pub const TG3_PHYFLG_10_100_ONLY: c_uint = 0x00000080;
pub const TG3_PHYFLG_ENABLE_APD: c_uint = 0x00000100;
pub const TG3_PHYFLG_CAPACITIVE_COUPLING: c_uint = 0x00000200;
pub const TG3_PHYFLG_NO_ETH_WIRE_SPEED: c_uint = 0x00000400;
pub const TG3_PHYFLG_JITTER_BUG: c_uint = 0x00000800;
pub const TG3_PHYFLG_ADJUST_TRIM: c_uint = 0x00001000;
pub const TG3_PHYFLG_ADC_BUG: c_uint = 0x00002000;
pub const TG3_PHYFLG_5704_A0_BUG: c_uint = 0x00004000;
pub const TG3_PHYFLG_BER_BUG: c_uint = 0x00008000;
pub const TG3_PHYFLG_SERDES_PREEMPHASIS: c_uint = 0x00010000;
pub const TG3_PHYFLG_PARALLEL_DETECT: c_uint = 0x00020000;
pub const TG3_PHYFLG_EEE_CAP: c_uint = 0x00040000;
pub const TG3_PHYFLG_1G_ON_VAUX_OK: c_uint = 0x00080000;
pub const TG3_PHYFLG_KEEP_LINK_ON_PWRDN: c_uint = 0x00100000;
pub const TG3_PHYFLG_MDIX_STATE: c_uint = 0x00200000;
pub const TG3_PHYFLG_DISABLE_1G_HD_ADV: c_uint = 0x00400000;
pub const TG3_BPN_SIZE: c_int = 24;

pub const TG3_NVRAM_SIZE_2KB: c_uint = 0x00000800;
pub const TG3_NVRAM_SIZE_64KB: c_uint = 0x00010000;
pub const TG3_NVRAM_SIZE_128KB: c_uint = 0x00020000;
pub const TG3_NVRAM_SIZE_256KB: c_uint = 0x00040000;
pub const TG3_NVRAM_SIZE_512KB: c_uint = 0x00080000;
pub const TG3_NVRAM_SIZE_1MB: c_uint = 0x00100000;
pub const TG3_NVRAM_SIZE_2MB: c_uint = 0x00200000;
pub const JEDEC_ATMEL: c_uint = 0x1f;
pub const JEDEC_ST: c_uint = 0x20;
pub const JEDEC_SAIFUN: c_uint = 0x4f;
pub const JEDEC_SST: c_uint = 0xbf;
pub const JEDEC_MACRONIX: c_uint = 0xc2;

pub const ATMEL_AT45DB0X1B_PAGE_POS: c_int = 9;
pub const ATMEL_AT45DB0X1B_PAGE_SIZE: c_int = 264;
pub const ATMEL_AT25F512_PAGE_SIZE: c_int = 256;
pub const ST_M45PEX0_PAGE_SIZE: c_int = 256;
pub const SAIFUN_SA25F0XX_PAGE_SIZE: c_int = 256;
pub const SST_25VF0X0_PAGE_SIZE: c_int = 4098;
// firmware info
// Accessor macros for chip and asic attributes
//
// nb: Using static inlines equivalent to the accessor macros generates
// larger object code with gcc 4.7.
// Using statement expression macros to check tp with
// typecheck(struct tg3 *, tp) also creates larger objects.
//

