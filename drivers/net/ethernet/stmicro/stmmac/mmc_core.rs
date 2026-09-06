//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/mmc_core.c
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
    DWMAC Management Counters
    Copyright (C) 2011  STMicroelectronics Ltd
    Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//

// MAC Management Counters register offset
pub const MMC_CNTRL: c_uint = 0x00	/* MMC Control */;
pub const MMC_RX_INTR: c_uint = 0x04	/* MMC RX Interrupt */;
pub const MMC_TX_INTR: c_uint = 0x08	/* MMC TX Interrupt */;
pub const MMC_RX_INTR_MASK: c_uint = 0x0c	/* MMC Interrupt Mask */;
pub const MMC_TX_INTR_MASK: c_uint = 0x10	/* MMC Interrupt Mask */;
pub const MMC_DEFAULT_MASK: c_uint = 0xffffffff;
// MMC TX counter registers
// Note:
// _GB register stands for good and bad frames
// _G is for good only.
//
pub const MMC_TX_OCTETCOUNT_GB: c_uint = 0x14;
pub const MMC_TX_FRAMECOUNT_GB: c_uint = 0x18;
pub const MMC_TX_BROADCASTFRAME_G: c_uint = 0x1c;
pub const MMC_TX_MULTICASTFRAME_G: c_uint = 0x20;
pub const MMC_TX_64_OCTETS_GB: c_uint = 0x24;
pub const MMC_TX_65_TO_127_OCTETS_GB: c_uint = 0x28;
pub const MMC_TX_128_TO_255_OCTETS_GB: c_uint = 0x2c;
pub const MMC_TX_256_TO_511_OCTETS_GB: c_uint = 0x30;
pub const MMC_TX_512_TO_1023_OCTETS_GB: c_uint = 0x34;
pub const MMC_TX_1024_TO_MAX_OCTETS_GB: c_uint = 0x38;
pub const MMC_TX_UNICAST_GB: c_uint = 0x3c;
pub const MMC_TX_MULTICAST_GB: c_uint = 0x40;
pub const MMC_TX_BROADCAST_GB: c_uint = 0x44;
pub const MMC_TX_UNDERFLOW_ERROR: c_uint = 0x48;
pub const MMC_TX_SINGLECOL_G: c_uint = 0x4c;
pub const MMC_TX_MULTICOL_G: c_uint = 0x50;
pub const MMC_TX_DEFERRED: c_uint = 0x54;
pub const MMC_TX_LATECOL: c_uint = 0x58;
pub const MMC_TX_EXESSCOL: c_uint = 0x5c;
pub const MMC_TX_CARRIER_ERROR: c_uint = 0x60;
pub const MMC_TX_OCTETCOUNT_G: c_uint = 0x64;
pub const MMC_TX_FRAMECOUNT_G: c_uint = 0x68;
pub const MMC_TX_EXCESSDEF: c_uint = 0x6c;
pub const MMC_TX_PAUSE_FRAME: c_uint = 0x70;
pub const MMC_TX_VLAN_FRAME_G: c_uint = 0x74;
pub const MMC_TX_OVERSIZE_G: c_uint = 0x78;
// MMC RX counter registers
pub const MMC_RX_FRAMECOUNT_GB: c_uint = 0x80;
pub const MMC_RX_OCTETCOUNT_GB: c_uint = 0x84;
pub const MMC_RX_OCTETCOUNT_G: c_uint = 0x88;
pub const MMC_RX_BROADCASTFRAME_G: c_uint = 0x8c;
pub const MMC_RX_MULTICASTFRAME_G: c_uint = 0x90;
pub const MMC_RX_CRC_ERROR: c_uint = 0x94;
pub const MMC_RX_ALIGN_ERROR: c_uint = 0x98;
pub const MMC_RX_RUN_ERROR: c_uint = 0x9C;
pub const MMC_RX_JABBER_ERROR: c_uint = 0xA0;
pub const MMC_RX_UNDERSIZE_G: c_uint = 0xA4;
pub const MMC_RX_OVERSIZE_G: c_uint = 0xA8;
pub const MMC_RX_64_OCTETS_GB: c_uint = 0xAC;
pub const MMC_RX_65_TO_127_OCTETS_GB: c_uint = 0xb0;
pub const MMC_RX_128_TO_255_OCTETS_GB: c_uint = 0xb4;
pub const MMC_RX_256_TO_511_OCTETS_GB: c_uint = 0xb8;
pub const MMC_RX_512_TO_1023_OCTETS_GB: c_uint = 0xbc;
pub const MMC_RX_1024_TO_MAX_OCTETS_GB: c_uint = 0xc0;
pub const MMC_RX_UNICAST_G: c_uint = 0xc4;
pub const MMC_RX_LENGTH_ERROR: c_uint = 0xc8;
pub const MMC_RX_AUTOFRANGETYPE: c_uint = 0xcc;
pub const MMC_RX_PAUSE_FRAMES: c_uint = 0xd0;
pub const MMC_RX_FIFO_OVERFLOW: c_uint = 0xd4;
pub const MMC_RX_VLAN_FRAMES_GB: c_uint = 0xd8;
pub const MMC_RX_WATCHDOG_ERROR: c_uint = 0xdc;
pub const MMC_RX_ERROR: c_uint = 0xe0;
pub const MMC_TX_LPI_USEC: c_uint = 0xec;
pub const MMC_TX_LPI_TRAN: c_uint = 0xf0;
pub const MMC_RX_LPI_USEC: c_uint = 0xf4;
pub const MMC_RX_LPI_TRAN: c_uint = 0xf8;
// IPC
pub const MMC_RX_IPC_INTR_MASK: c_uint = 0x100;
pub const MMC_RX_IPC_INTR: c_uint = 0x108;
// IPv4
pub const MMC_RX_IPV4_GD: c_uint = 0x110;
pub const MMC_RX_IPV4_HDERR: c_uint = 0x114;
pub const MMC_RX_IPV4_NOPAY: c_uint = 0x118;
pub const MMC_RX_IPV4_FRAG: c_uint = 0x11C;
pub const MMC_RX_IPV4_UDSBL: c_uint = 0x120;
pub const MMC_RX_IPV4_GD_OCTETS: c_uint = 0x150;
pub const MMC_RX_IPV4_HDERR_OCTETS: c_uint = 0x154;
pub const MMC_RX_IPV4_NOPAY_OCTETS: c_uint = 0x158;
pub const MMC_RX_IPV4_FRAG_OCTETS: c_uint = 0x15c;
pub const MMC_RX_IPV4_UDSBL_OCTETS: c_uint = 0x160;
// IPV6
pub const MMC_RX_IPV6_GD_OCTETS: c_uint = 0x164;
pub const MMC_RX_IPV6_HDERR_OCTETS: c_uint = 0x168;
pub const MMC_RX_IPV6_NOPAY_OCTETS: c_uint = 0x16c;
pub const MMC_RX_IPV6_GD: c_uint = 0x124;
pub const MMC_RX_IPV6_HDERR: c_uint = 0x128;
pub const MMC_RX_IPV6_NOPAY: c_uint = 0x12c;
// Protocols
pub const MMC_RX_UDP_GD: c_uint = 0x130;
pub const MMC_RX_UDP_ERR: c_uint = 0x134;
pub const MMC_RX_TCP_GD: c_uint = 0x138;
pub const MMC_RX_TCP_ERR: c_uint = 0x13c;
pub const MMC_RX_ICMP_GD: c_uint = 0x140;
pub const MMC_RX_ICMP_ERR: c_uint = 0x144;
pub const MMC_RX_UDP_GD_OCTETS: c_uint = 0x170;
pub const MMC_RX_UDP_ERR_OCTETS: c_uint = 0x174;
pub const MMC_RX_TCP_GD_OCTETS: c_uint = 0x178;
pub const MMC_RX_TCP_ERR_OCTETS: c_uint = 0x17c;
pub const MMC_RX_ICMP_GD_OCTETS: c_uint = 0x180;
pub const MMC_RX_ICMP_ERR_OCTETS: c_uint = 0x184;
pub const MMC_TX_FPE_FRAG: c_uint = 0x1a8;
pub const MMC_TX_HOLD_REQ: c_uint = 0x1ac;
pub const MMC_RX_PKT_ASSEMBLY_ERR: c_uint = 0x1c8;
pub const MMC_RX_PKT_SMD_ERR: c_uint = 0x1cc;
pub const MMC_RX_PKT_ASSEMBLY_OK: c_uint = 0x1d0;
pub const MMC_RX_FPE_FRAG: c_uint = 0x1d4;
// XGMAC MMC Registers
pub const MMC_XGMAC_TX_OCTET_GB: c_uint = 0x14;
pub const MMC_XGMAC_TX_PKT_GB: c_uint = 0x1c;
pub const MMC_XGMAC_TX_BROAD_PKT_G: c_uint = 0x24;
pub const MMC_XGMAC_TX_MULTI_PKT_G: c_uint = 0x2c;
pub const MMC_XGMAC_TX_64OCT_GB: c_uint = 0x34;
pub const MMC_XGMAC_TX_65OCT_GB: c_uint = 0x3c;
pub const MMC_XGMAC_TX_128OCT_GB: c_uint = 0x44;
pub const MMC_XGMAC_TX_256OCT_GB: c_uint = 0x4c;
pub const MMC_XGMAC_TX_512OCT_GB: c_uint = 0x54;
pub const MMC_XGMAC_TX_1024OCT_GB: c_uint = 0x5c;
pub const MMC_XGMAC_TX_UNI_PKT_GB: c_uint = 0x64;
pub const MMC_XGMAC_TX_MULTI_PKT_GB: c_uint = 0x6c;
pub const MMC_XGMAC_TX_BROAD_PKT_GB: c_uint = 0x74;
pub const MMC_XGMAC_TX_UNDER: c_uint = 0x7c;
pub const MMC_XGMAC_TX_OCTET_G: c_uint = 0x84;
pub const MMC_XGMAC_TX_PKT_G: c_uint = 0x8c;
pub const MMC_XGMAC_TX_PAUSE: c_uint = 0x94;
pub const MMC_XGMAC_TX_VLAN_PKT_G: c_uint = 0x9c;
pub const MMC_XGMAC_TX_LPI_USEC: c_uint = 0xa4;
pub const MMC_XGMAC_TX_LPI_TRAN: c_uint = 0xa8;
pub const MMC_XGMAC_RX_PKT_GB: c_uint = 0x100;
pub const MMC_XGMAC_RX_OCTET_GB: c_uint = 0x108;
pub const MMC_XGMAC_RX_OCTET_G: c_uint = 0x110;
pub const MMC_XGMAC_RX_BROAD_PKT_G: c_uint = 0x118;
pub const MMC_XGMAC_RX_MULTI_PKT_G: c_uint = 0x120;
pub const MMC_XGMAC_RX_CRC_ERR: c_uint = 0x128;
pub const MMC_XGMAC_RX_RUNT_ERR: c_uint = 0x130;
pub const MMC_XGMAC_RX_JABBER_ERR: c_uint = 0x134;
pub const MMC_XGMAC_RX_UNDER: c_uint = 0x138;
pub const MMC_XGMAC_RX_OVER: c_uint = 0x13c;
pub const MMC_XGMAC_RX_64OCT_GB: c_uint = 0x140;
pub const MMC_XGMAC_RX_65OCT_GB: c_uint = 0x148;
pub const MMC_XGMAC_RX_128OCT_GB: c_uint = 0x150;
pub const MMC_XGMAC_RX_256OCT_GB: c_uint = 0x158;
pub const MMC_XGMAC_RX_512OCT_GB: c_uint = 0x160;
pub const MMC_XGMAC_RX_1024OCT_GB: c_uint = 0x168;
pub const MMC_XGMAC_RX_UNI_PKT_G: c_uint = 0x170;
pub const MMC_XGMAC_RX_LENGTH_ERR: c_uint = 0x178;
pub const MMC_XGMAC_RX_RANGE: c_uint = 0x180;
pub const MMC_XGMAC_RX_PAUSE: c_uint = 0x188;
pub const MMC_XGMAC_RX_FIFOOVER_PKT: c_uint = 0x190;
pub const MMC_XGMAC_RX_VLAN_PKT_GB: c_uint = 0x198;
pub const MMC_XGMAC_RX_WATCHDOG_ERR: c_uint = 0x1a0;
pub const MMC_XGMAC_RX_LPI_USEC: c_uint = 0x1a4;
pub const MMC_XGMAC_RX_LPI_TRAN: c_uint = 0x1a8;
pub const MMC_XGMAC_RX_DISCARD_PKT_GB: c_uint = 0x1ac;
pub const MMC_XGMAC_RX_DISCARD_OCT_GB: c_uint = 0x1b4;
pub const MMC_XGMAC_RX_ALIGN_ERR_PKT: c_uint = 0x1bc;
pub const MMC_XGMAC_SGF_PASS_PKT: c_uint = 0x1f0;
pub const MMC_XGMAC_SGF_FAIL_PKT: c_uint = 0x1f4;
pub const MMC_XGMAC_TX_FPE_INTR_MASK: c_uint = 0x204;
pub const MMC_XGMAC_TX_FPE_FRAG: c_uint = 0x208;
pub const MMC_XGMAC_TX_HOLD_REQ: c_uint = 0x20c;
pub const MMC_XGMAC_TX_GATE_OVERRUN: c_uint = 0x210;
pub const MMC_XGMAC_RX_FPE_INTR_MASK: c_uint = 0x224;
pub const MMC_XGMAC_RX_PKT_ASSEMBLY_ERR: c_uint = 0x228;
pub const MMC_XGMAC_RX_PKT_SMD_ERR: c_uint = 0x22c;
pub const MMC_XGMAC_RX_PKT_ASSEMBLY_OK: c_uint = 0x230;
pub const MMC_XGMAC_RX_FPE_FRAG: c_uint = 0x234;
pub const MMC_XGMAC_RX_IPC_INTR_MASK: c_uint = 0x25c;
pub const MMC_XGMAC_RX_IPV4_GD: c_uint = 0x264;
pub const MMC_XGMAC_RX_IPV4_HDERR: c_uint = 0x26c;
pub const MMC_XGMAC_RX_IPV4_NOPAY: c_uint = 0x274;
pub const MMC_XGMAC_RX_IPV4_FRAG: c_uint = 0x27c;
pub const MMC_XGMAC_RX_IPV4_UDSBL: c_uint = 0x284;
pub const MMC_XGMAC_RX_IPV6_GD: c_uint = 0x28c;
pub const MMC_XGMAC_RX_IPV6_HDERR: c_uint = 0x294;
pub const MMC_XGMAC_RX_IPV6_NOPAY: c_uint = 0x29c;
pub const MMC_XGMAC_RX_UDP_GD: c_uint = 0x2a4;
pub const MMC_XGMAC_RX_UDP_ERR: c_uint = 0x2ac;
pub const MMC_XGMAC_RX_TCP_GD: c_uint = 0x2b4;
pub const MMC_XGMAC_RX_TCP_ERR: c_uint = 0x2bc;
pub const MMC_XGMAC_RX_ICMP_GD: c_uint = 0x2c4;
pub const MMC_XGMAC_RX_ICMP_ERR: c_uint = 0x2cc;
pub const MMC_XGMAC_RX_IPV4_GD_OCTETS: c_uint = 0x2d4;
pub const MMC_XGMAC_RX_IPV4_HDERR_OCTETS: c_uint = 0x2dc;
pub const MMC_XGMAC_RX_IPV4_NOPAY_OCTETS: c_uint = 0x2e4;
pub const MMC_XGMAC_RX_IPV4_FRAG_OCTETS: c_uint = 0x2ec;
pub const MMC_XGMAC_RX_IPV4_UDSBL_OCTETS: c_uint = 0x2f4;
pub const MMC_XGMAC_RX_IPV6_GD_OCTETS: c_uint = 0x2fc;
pub const MMC_XGMAC_RX_IPV6_HDERR_OCTETS: c_uint = 0x304;
pub const MMC_XGMAC_RX_IPV6_NOPAY_OCTETS: c_uint = 0x30c;
pub const MMC_XGMAC_RX_UDP_GD_OCTETS: c_uint = 0x314;
pub const MMC_XGMAC_RX_UDP_ERR_OCTETS: c_uint = 0x31c;
pub const MMC_XGMAC_RX_TCP_GD_OCTETS: c_uint = 0x324;
pub const MMC_XGMAC_RX_TCP_ERR_OCTETS: c_uint = 0x32c;
pub const MMC_XGMAC_RX_ICMP_GD_OCTETS: c_uint = 0x334;
pub const MMC_XGMAC_RX_ICMP_ERR_OCTETS: c_uint = 0x33c;
#[no_mangle]
unsafe extern "C" fn dwmac_mmc_ctrl(mmcaddr: *mut void __iomem, mode: c_uint) {
    static void dwmac_mmc_ctrl(void __iomem *mmcaddr, unsigned int mode)
    {
    let mut value: u32 = readl(mmcaddr + MMC_CNTRL);
    value |= (mode & 0x3F);
    writel(value, mmcaddr + MMC_CNTRL);
    pr_debug("stmmac: MMC ctrl register (offset 0x%x): 0x%08x\n",
    MMC_CNTRL, value);
    }
// To mask all interrupts.
#[no_mangle]
unsafe extern "C" fn dwmac_mmc_intr_all_mask(mmcaddr: *mut void __iomem) {
    static void dwmac_mmc_intr_all_mask(void __iomem *mmcaddr)
    {
    writel(MMC_DEFAULT_MASK, mmcaddr + MMC_RX_INTR_MASK);
    writel(MMC_DEFAULT_MASK, mmcaddr + MMC_TX_INTR_MASK);
    writel(MMC_DEFAULT_MASK, mmcaddr + MMC_RX_IPC_INTR_MASK);
    }
// This reads the MAC core counters (if actually supported).
// by default the MMC core is programmed to reset each
// counter after a read. So all the field of the mmc struct
// have to be incremented.
//
#[no_mangle]
unsafe extern "C" fn dwmac_mmc_read(mmcaddr: *mut void __iomem, mmc: *mut stmmac_counters) {
    static void dwmac_mmc_read(void __iomem *mmcaddr, struct stmmac_counters *mmc)
    {
    mmc.mmc_tx_octetcount_gb += readl(mmcaddr + MMC_TX_OCTETCOUNT_GB);
    mmc.mmc_tx_framecount_gb += readl(mmcaddr + MMC_TX_FRAMECOUNT_GB);
    mmc.mmc_tx_broadcastframe_g += readl(mmcaddr +
    MMC_TX_BROADCASTFRAME_G);
    mmc.mmc_tx_multicastframe_g += readl(mmcaddr +
    MMC_TX_MULTICASTFRAME_G);
    mmc.mmc_tx_64_octets_gb += readl(mmcaddr + MMC_TX_64_OCTETS_GB);
    mmc.mmc_tx_65_to_127_octets_gb +=
    readl(mmcaddr + MMC_TX_65_TO_127_OCTETS_GB);
    mmc.mmc_tx_128_to_255_octets_gb +=
    readl(mmcaddr + MMC_TX_128_TO_255_OCTETS_GB);
    mmc.mmc_tx_256_to_511_octets_gb +=
    readl(mmcaddr + MMC_TX_256_TO_511_OCTETS_GB);
    mmc.mmc_tx_512_to_1023_octets_gb +=
    readl(mmcaddr + MMC_TX_512_TO_1023_OCTETS_GB);
    mmc.mmc_tx_1024_to_max_octets_gb +=
    readl(mmcaddr + MMC_TX_1024_TO_MAX_OCTETS_GB);
    mmc.mmc_tx_unicast_gb += readl(mmcaddr + MMC_TX_UNICAST_GB);
    mmc.mmc_tx_multicast_gb += readl(mmcaddr + MMC_TX_MULTICAST_GB);
    mmc.mmc_tx_broadcast_gb += readl(mmcaddr + MMC_TX_BROADCAST_GB);
    mmc.mmc_tx_underflow_error += readl(mmcaddr + MMC_TX_UNDERFLOW_ERROR);
    mmc.mmc_tx_singlecol_g += readl(mmcaddr + MMC_TX_SINGLECOL_G);
    mmc.mmc_tx_multicol_g += readl(mmcaddr + MMC_TX_MULTICOL_G);
    mmc.mmc_tx_deferred += readl(mmcaddr + MMC_TX_DEFERRED);
    mmc.mmc_tx_latecol += readl(mmcaddr + MMC_TX_LATECOL);
    mmc.mmc_tx_exesscol += readl(mmcaddr + MMC_TX_EXESSCOL);
    mmc.mmc_tx_carrier_error += readl(mmcaddr + MMC_TX_CARRIER_ERROR);
    mmc.mmc_tx_octetcount_g += readl(mmcaddr + MMC_TX_OCTETCOUNT_G);
    mmc.mmc_tx_framecount_g += readl(mmcaddr + MMC_TX_FRAMECOUNT_G);
    mmc.mmc_tx_excessdef += readl(mmcaddr + MMC_TX_EXCESSDEF);
    mmc.mmc_tx_pause_frame += readl(mmcaddr + MMC_TX_PAUSE_FRAME);
    mmc.mmc_tx_vlan_frame_g += readl(mmcaddr + MMC_TX_VLAN_FRAME_G);
    mmc.mmc_tx_oversize_g	 += readl(mmcaddr + MMC_TX_OVERSIZE_G);
    mmc.mmc_tx_lpi_usec += readl(mmcaddr + MMC_TX_LPI_USEC);
    mmc.mmc_tx_lpi_tran += readl(mmcaddr + MMC_TX_LPI_TRAN);
// MMC RX counter registers
    mmc.mmc_rx_framecount_gb += readl(mmcaddr + MMC_RX_FRAMECOUNT_GB);
    mmc.mmc_rx_octetcount_gb += readl(mmcaddr + MMC_RX_OCTETCOUNT_GB);
    mmc.mmc_rx_octetcount_g += readl(mmcaddr + MMC_RX_OCTETCOUNT_G);
    mmc.mmc_rx_broadcastframe_g += readl(mmcaddr +
    MMC_RX_BROADCASTFRAME_G);
    mmc.mmc_rx_multicastframe_g += readl(mmcaddr +
    MMC_RX_MULTICASTFRAME_G);
    mmc.mmc_rx_crc_error += readl(mmcaddr + MMC_RX_CRC_ERROR);
    mmc.mmc_rx_align_error += readl(mmcaddr + MMC_RX_ALIGN_ERROR);
    mmc.mmc_rx_run_error += readl(mmcaddr + MMC_RX_RUN_ERROR);
    mmc.mmc_rx_jabber_error += readl(mmcaddr + MMC_RX_JABBER_ERROR);
    mmc.mmc_rx_undersize_g += readl(mmcaddr + MMC_RX_UNDERSIZE_G);
    mmc.mmc_rx_oversize_g += readl(mmcaddr + MMC_RX_OVERSIZE_G);
    mmc.mmc_rx_64_octets_gb += readl(mmcaddr + MMC_RX_64_OCTETS_GB);
    mmc.mmc_rx_65_to_127_octets_gb +=
    readl(mmcaddr + MMC_RX_65_TO_127_OCTETS_GB);
    mmc.mmc_rx_128_to_255_octets_gb +=
    readl(mmcaddr + MMC_RX_128_TO_255_OCTETS_GB);
    mmc.mmc_rx_256_to_511_octets_gb +=
    readl(mmcaddr + MMC_RX_256_TO_511_OCTETS_GB);
    mmc.mmc_rx_512_to_1023_octets_gb +=
    readl(mmcaddr + MMC_RX_512_TO_1023_OCTETS_GB);
    mmc.mmc_rx_1024_to_max_octets_gb +=
    readl(mmcaddr + MMC_RX_1024_TO_MAX_OCTETS_GB);
    mmc.mmc_rx_unicast_g += readl(mmcaddr + MMC_RX_UNICAST_G);
    mmc.mmc_rx_length_error += readl(mmcaddr + MMC_RX_LENGTH_ERROR);
    mmc.mmc_rx_autofrangetype += readl(mmcaddr + MMC_RX_AUTOFRANGETYPE);
    mmc.mmc_rx_pause_frames += readl(mmcaddr + MMC_RX_PAUSE_FRAMES);
    mmc.mmc_rx_fifo_overflow += readl(mmcaddr + MMC_RX_FIFO_OVERFLOW);
    mmc.mmc_rx_vlan_frames_gb += readl(mmcaddr + MMC_RX_VLAN_FRAMES_GB);
    mmc.mmc_rx_watchdog_error += readl(mmcaddr + MMC_RX_WATCHDOG_ERROR);
    mmc.mmc_rx_error += readl(mmcaddr + MMC_RX_ERROR);
    mmc.mmc_rx_lpi_usec += readl(mmcaddr + MMC_RX_LPI_USEC);
    mmc.mmc_rx_lpi_tran += readl(mmcaddr + MMC_RX_LPI_TRAN);
// IPv4
    mmc.mmc_rx_ipv4_gd += readl(mmcaddr + MMC_RX_IPV4_GD);
    mmc.mmc_rx_ipv4_hderr += readl(mmcaddr + MMC_RX_IPV4_HDERR);
    mmc.mmc_rx_ipv4_nopay += readl(mmcaddr + MMC_RX_IPV4_NOPAY);
    mmc.mmc_rx_ipv4_frag += readl(mmcaddr + MMC_RX_IPV4_FRAG);
    mmc.mmc_rx_ipv4_udsbl += readl(mmcaddr + MMC_RX_IPV4_UDSBL);
    mmc.mmc_rx_ipv4_gd_octets += readl(mmcaddr + MMC_RX_IPV4_GD_OCTETS);
    mmc.mmc_rx_ipv4_hderr_octets +=
    readl(mmcaddr + MMC_RX_IPV4_HDERR_OCTETS);
    mmc.mmc_rx_ipv4_nopay_octets +=
    readl(mmcaddr + MMC_RX_IPV4_NOPAY_OCTETS);
    mmc.mmc_rx_ipv4_frag_octets += readl(mmcaddr +
    MMC_RX_IPV4_FRAG_OCTETS);
    mmc.mmc_rx_ipv4_udsbl_octets +=
    readl(mmcaddr + MMC_RX_IPV4_UDSBL_OCTETS);
// IPV6
    mmc.mmc_rx_ipv6_gd_octets += readl(mmcaddr + MMC_RX_IPV6_GD_OCTETS);
    mmc.mmc_rx_ipv6_hderr_octets +=
    readl(mmcaddr + MMC_RX_IPV6_HDERR_OCTETS);
    mmc.mmc_rx_ipv6_nopay_octets +=
    readl(mmcaddr + MMC_RX_IPV6_NOPAY_OCTETS);
    mmc.mmc_rx_ipv6_gd += readl(mmcaddr + MMC_RX_IPV6_GD);
    mmc.mmc_rx_ipv6_hderr += readl(mmcaddr + MMC_RX_IPV6_HDERR);
    mmc.mmc_rx_ipv6_nopay += readl(mmcaddr + MMC_RX_IPV6_NOPAY);
// Protocols
    mmc.mmc_rx_udp_gd += readl(mmcaddr + MMC_RX_UDP_GD);
    mmc.mmc_rx_udp_err += readl(mmcaddr + MMC_RX_UDP_ERR);
    mmc.mmc_rx_tcp_gd += readl(mmcaddr + MMC_RX_TCP_GD);
    mmc.mmc_rx_tcp_err += readl(mmcaddr + MMC_RX_TCP_ERR);
    mmc.mmc_rx_icmp_gd += readl(mmcaddr + MMC_RX_ICMP_GD);
    mmc.mmc_rx_icmp_err += readl(mmcaddr + MMC_RX_ICMP_ERR);
    mmc.mmc_rx_udp_gd_octets += readl(mmcaddr + MMC_RX_UDP_GD_OCTETS);
    mmc.mmc_rx_udp_err_octets += readl(mmcaddr + MMC_RX_UDP_ERR_OCTETS);
    mmc.mmc_rx_tcp_gd_octets += readl(mmcaddr + MMC_RX_TCP_GD_OCTETS);
    mmc.mmc_rx_tcp_err_octets += readl(mmcaddr + MMC_RX_TCP_ERR_OCTETS);
    mmc.mmc_rx_icmp_gd_octets += readl(mmcaddr + MMC_RX_ICMP_GD_OCTETS);
    mmc.mmc_rx_icmp_err_octets += readl(mmcaddr + MMC_RX_ICMP_ERR_OCTETS);
    mmc.mmc_tx_fpe_fragment_cntr += readl(mmcaddr + MMC_TX_FPE_FRAG);
    mmc.mmc_tx_hold_req_cntr += readl(mmcaddr + MMC_TX_HOLD_REQ);
    mmc.mmc_rx_packet_assembly_err_cntr +=
    readl(mmcaddr + MMC_RX_PKT_ASSEMBLY_ERR);
    mmc.mmc_rx_packet_smd_err_cntr += readl(mmcaddr + MMC_RX_PKT_SMD_ERR);
    mmc.mmc_rx_packet_assembly_ok_cntr +=
    readl(mmcaddr + MMC_RX_PKT_ASSEMBLY_OK);
    mmc.mmc_rx_fpe_fragment_cntr += readl(mmcaddr + MMC_RX_FPE_FRAG);
    }
    const struct stmmac_mmc_ops dwmac_mmc_ops = {
    .ctrl = dwmac_mmc_ctrl,
    .intr_all_mask = dwmac_mmc_intr_all_mask,
    .read = dwmac_mmc_read,
    };
#[no_mangle]
unsafe extern "C" fn dwxgmac_mmc_ctrl(mmcaddr: *mut void __iomem, mode: c_uint) {
    static void dwxgmac_mmc_ctrl(void __iomem *mmcaddr, unsigned int mode)
    {
    let mut value: u32 = readl(mmcaddr + MMC_CNTRL);
    value |= (mode & 0x3F);
    writel(value, mmcaddr + MMC_CNTRL);
    }
#[no_mangle]
unsafe extern "C" fn dwxgmac_mmc_intr_all_mask(mmcaddr: *mut void __iomem) {
    static void dwxgmac_mmc_intr_all_mask(void __iomem *mmcaddr)
    {
    writel(0x0, mmcaddr + MMC_RX_INTR_MASK);
    writel(0x0, mmcaddr + MMC_TX_INTR_MASK);
    writel(MMC_DEFAULT_MASK, mmcaddr + MMC_XGMAC_TX_FPE_INTR_MASK);
    writel(MMC_DEFAULT_MASK, mmcaddr + MMC_XGMAC_RX_FPE_INTR_MASK);
    writel(MMC_DEFAULT_MASK, mmcaddr + MMC_XGMAC_RX_IPC_INTR_MASK);
    }
#[no_mangle]
unsafe extern "C" fn dwxgmac_read_mmc_reg(addr: *mut void __iomem, reg: u32, dest: *mut u32) {
    static void dwxgmac_read_mmc_reg(void __iomem *addr, u32 reg, u32 *dest)
    {
    let mut tmp: u64 = 0;
    tmp += readl(addr + reg);
    tmp += ((u64 )readl(addr + reg + 0x4)) << 32;
    if (tmp > GENMASK(31, 0))
// dest = ~0x0;
    else
// dest = *dest + tmp;
    }
// This reads the MAC core counters (if actually supported).
// by default the MMC core is programmed to reset each
// counter after a read. So all the field of the mmc struct
// have to be incremented.
//
#[no_mangle]
unsafe extern "C" fn dwxgmac_mmc_read(mmcaddr: *mut void __iomem, mmc: *mut stmmac_counters) {
    static void dwxgmac_mmc_read(void __iomem *mmcaddr, struct stmmac_counters *mmc)
    {
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_OCTET_GB,
    &mmc.mmc_tx_octetcount_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_PKT_GB,
    &mmc.mmc_tx_framecount_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_BROAD_PKT_G,
    &mmc.mmc_tx_broadcastframe_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_MULTI_PKT_G,
    &mmc.mmc_tx_multicastframe_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_64OCT_GB,
    &mmc.mmc_tx_64_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_65OCT_GB,
    &mmc.mmc_tx_65_to_127_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_128OCT_GB,
    &mmc.mmc_tx_128_to_255_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_256OCT_GB,
    &mmc.mmc_tx_256_to_511_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_512OCT_GB,
    &mmc.mmc_tx_512_to_1023_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_1024OCT_GB,
    &mmc.mmc_tx_1024_to_max_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_UNI_PKT_GB,
    &mmc.mmc_tx_unicast_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_MULTI_PKT_GB,
    &mmc.mmc_tx_multicast_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_BROAD_PKT_GB,
    &mmc.mmc_tx_broadcast_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_UNDER,
    &mmc.mmc_tx_underflow_error);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_OCTET_G,
    &mmc.mmc_tx_octetcount_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_PKT_G,
    &mmc.mmc_tx_framecount_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_PAUSE,
    &mmc.mmc_tx_pause_frame);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_VLAN_PKT_G,
    &mmc.mmc_tx_vlan_frame_g);
    mmc.mmc_tx_lpi_usec += readl(mmcaddr + MMC_XGMAC_TX_LPI_USEC);
    mmc.mmc_tx_lpi_tran += readl(mmcaddr + MMC_XGMAC_TX_LPI_TRAN);
// MMC RX counter registers
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_PKT_GB,
    &mmc.mmc_rx_framecount_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_OCTET_GB,
    &mmc.mmc_rx_octetcount_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_OCTET_G,
    &mmc.mmc_rx_octetcount_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_BROAD_PKT_G,
    &mmc.mmc_rx_broadcastframe_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_MULTI_PKT_G,
    &mmc.mmc_rx_multicastframe_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_CRC_ERR,
    &mmc.mmc_rx_crc_error);
    mmc.mmc_rx_run_error += readl(mmcaddr + MMC_XGMAC_RX_RUNT_ERR);
    mmc.mmc_rx_jabber_error += readl(mmcaddr + MMC_XGMAC_RX_JABBER_ERR);
    mmc.mmc_rx_undersize_g += readl(mmcaddr + MMC_XGMAC_RX_UNDER);
    mmc.mmc_rx_oversize_g += readl(mmcaddr + MMC_XGMAC_RX_OVER);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_64OCT_GB,
    &mmc.mmc_rx_64_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_65OCT_GB,
    &mmc.mmc_rx_65_to_127_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_128OCT_GB,
    &mmc.mmc_rx_128_to_255_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_256OCT_GB,
    &mmc.mmc_rx_256_to_511_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_512OCT_GB,
    &mmc.mmc_rx_512_to_1023_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_1024OCT_GB,
    &mmc.mmc_rx_1024_to_max_octets_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_UNI_PKT_G,
    &mmc.mmc_rx_unicast_g);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_LENGTH_ERR,
    &mmc.mmc_rx_length_error);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_RANGE,
    &mmc.mmc_rx_autofrangetype);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_PAUSE,
    &mmc.mmc_rx_pause_frames);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_FIFOOVER_PKT,
    &mmc.mmc_rx_fifo_overflow);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_VLAN_PKT_GB,
    &mmc.mmc_rx_vlan_frames_gb);
    mmc.mmc_rx_watchdog_error += readl(mmcaddr + MMC_XGMAC_RX_WATCHDOG_ERR);
    mmc.mmc_rx_lpi_usec += readl(mmcaddr + MMC_XGMAC_RX_LPI_USEC);
    mmc.mmc_rx_lpi_tran += readl(mmcaddr + MMC_XGMAC_RX_LPI_TRAN);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_DISCARD_PKT_GB,
    &mmc.mmc_rx_discard_frames_gb);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_DISCARD_OCT_GB,
    &mmc.mmc_rx_discard_octets_gb);
    mmc.mmc_rx_align_err_frames +=
    readl(mmcaddr + MMC_XGMAC_RX_ALIGN_ERR_PKT);
    mmc.mmc_sgf_pass_fragment_cntr +=
    readl(mmcaddr + MMC_XGMAC_SGF_PASS_PKT);
    mmc.mmc_sgf_fail_fragment_cntr +=
    readl(mmcaddr + MMC_XGMAC_SGF_FAIL_PKT);
    mmc.mmc_tx_fpe_fragment_cntr += readl(mmcaddr + MMC_XGMAC_TX_FPE_FRAG);
    mmc.mmc_tx_hold_req_cntr += readl(mmcaddr + MMC_XGMAC_TX_HOLD_REQ);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_TX_GATE_OVERRUN,
    &mmc.mmc_tx_gate_overrun_cntr);
    mmc.mmc_rx_packet_assembly_err_cntr +=
    readl(mmcaddr + MMC_XGMAC_RX_PKT_ASSEMBLY_ERR);
    mmc.mmc_rx_packet_smd_err_cntr +=
    readl(mmcaddr + MMC_XGMAC_RX_PKT_SMD_ERR);
    mmc.mmc_rx_packet_assembly_ok_cntr +=
    readl(mmcaddr + MMC_XGMAC_RX_PKT_ASSEMBLY_OK);
    mmc.mmc_rx_fpe_fragment_cntr +=
    readl(mmcaddr + MMC_XGMAC_RX_FPE_FRAG);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_GD,
    &mmc.mmc_rx_ipv4_gd);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_HDERR,
    &mmc.mmc_rx_ipv4_hderr);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_NOPAY,
    &mmc.mmc_rx_ipv4_nopay);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_FRAG,
    &mmc.mmc_rx_ipv4_frag);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_UDSBL,
    &mmc.mmc_rx_ipv4_udsbl);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV6_GD,
    &mmc.mmc_rx_ipv6_gd);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV6_HDERR,
    &mmc.mmc_rx_ipv6_hderr);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV6_NOPAY,
    &mmc.mmc_rx_ipv6_nopay);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_UDP_GD,
    &mmc.mmc_rx_udp_gd);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_UDP_ERR,
    &mmc.mmc_rx_udp_err);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_TCP_GD,
    &mmc.mmc_rx_tcp_gd);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_TCP_ERR,
    &mmc.mmc_rx_tcp_err);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_ICMP_GD,
    &mmc.mmc_rx_icmp_gd);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_ICMP_ERR,
    &mmc.mmc_rx_icmp_err);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_GD_OCTETS,
    &mmc.mmc_rx_ipv4_gd_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_HDERR_OCTETS,
    &mmc.mmc_rx_ipv4_hderr_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_NOPAY_OCTETS,
    &mmc.mmc_rx_ipv4_nopay_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_FRAG_OCTETS,
    &mmc.mmc_rx_ipv4_frag_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV4_UDSBL_OCTETS,
    &mmc.mmc_rx_ipv4_udsbl_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV6_GD_OCTETS,
    &mmc.mmc_rx_ipv6_gd_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV6_HDERR_OCTETS,
    &mmc.mmc_rx_ipv6_hderr_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_IPV6_NOPAY_OCTETS,
    &mmc.mmc_rx_ipv6_nopay_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_UDP_GD_OCTETS,
    &mmc.mmc_rx_udp_gd_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_UDP_ERR_OCTETS,
    &mmc.mmc_rx_udp_err_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_TCP_GD_OCTETS,
    &mmc.mmc_rx_tcp_gd_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_TCP_ERR_OCTETS,
    &mmc.mmc_rx_tcp_err_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_ICMP_GD_OCTETS,
    &mmc.mmc_rx_icmp_gd_octets);
    dwxgmac_read_mmc_reg(mmcaddr, MMC_XGMAC_RX_ICMP_ERR_OCTETS,
    &mmc.mmc_rx_icmp_err_octets);
    }
    const struct stmmac_mmc_ops dwxgmac_mmc_ops = {
    .ctrl = dwxgmac_mmc_ctrl,
    .intr_all_mask = dwxgmac_mmc_intr_all_mask,
    .read = dwxgmac_mmc_read,
    };
