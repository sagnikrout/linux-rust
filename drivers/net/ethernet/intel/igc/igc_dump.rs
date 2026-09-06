//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/igc/igc_dump.c
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
// Copyright (c)  2018 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_reg_info {
    pub ofs: u32,
    pub name: *mut c_char,
}

    static const struct igc_reg_info igc_reg_info_tbl[] = {
// General Registers
    {IGC_CTRL, "CTRL"},
    {IGC_STATUS, "STATUS"},
    {IGC_CTRL_EXT, "CTRL_EXT"},
    {IGC_MDIC, "MDIC"},
// Interrupt Registers
    {IGC_ICR, "ICR"},
// RX Registers
    {IGC_RCTL, "RCTL"},
    {IGC_RDLEN(0), "RDLEN"},
    {IGC_RDH(0), "RDH"},
    {IGC_RDT(0), "RDT"},
    {IGC_RXDCTL(0), "RXDCTL"},
    {IGC_RDBAL(0), "RDBAL"},
    {IGC_RDBAH(0), "RDBAH"},
// TX Registers
    {IGC_TCTL, "TCTL"},
    {IGC_TDBAL(0), "TDBAL"},
    {IGC_TDBAH(0), "TDBAH"},
    {IGC_TDLEN(0), "TDLEN"},
    {IGC_TDH(0), "TDH"},
    {IGC_TDT(0), "TDT"},
    {IGC_TXDCTL(0), "TXDCTL"},
// List Terminator
    {}
    };
// igc_regdump - register printout routine
#[no_mangle]
unsafe extern "C" fn igc_regdump(hw: *mut igc_hw, reginfo: *mut igc_reg_info) {
    static void igc_regdump(struct igc_hw *hw, struct igc_reg_info *reginfo)
    {
    struct net_device *dev = igc_get_hw_dev(hw);
    let mut n: c_int = 0;
    char rname[16];
    u32 regs[8];
    switch (reginfo.ofs) {
    case IGC_RDLEN(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_RDLEN(n));
    break;
    case IGC_RDH(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_RDH(n));
    break;
    case IGC_RDT(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_RDT(n));
    break;
    case IGC_RXDCTL(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_RXDCTL(n));
    break;
    case IGC_RDBAL(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_RDBAL(n));
    break;
    case IGC_RDBAH(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_RDBAH(n));
    break;
    case IGC_TDBAL(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_TDBAL(n));
    break;
    case IGC_TDBAH(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_TDBAH(n));
    break;
    case IGC_TDLEN(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_TDLEN(n));
    break;
    case IGC_TDH(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_TDH(n));
    break;
    case IGC_TDT(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_TDT(n));
    break;
    case IGC_TXDCTL(0):
    for (n = 0; n < 4; n++)
    regs[n] = rd32(IGC_TXDCTL(n));
    break;
    default:
    netdev_info(dev, "%-15s %08x\n", reginfo.name,
    rd32(reginfo.ofs));
    return;
    }
    snprintf(rname, 16, "%s%s", reginfo.name, "[0-3]");
    netdev_info(dev, "%-15s %08x %08x %08x %08x\n", rname, regs[0], regs[1],
    regs[2], regs[3]);
    }
// igc_rings_dump - Tx-rings and Rx-rings
#[no_mangle]
pub unsafe extern "C" fn igc_rings_dump(adapter: *mut igc_adapter) {
    void igc_rings_dump(struct igc_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_u0 {
    pub tx_desc: *mut union igc_adv_tx_desc,
    pub rx_desc: *mut union igc_adv_rx_desc,
    pub tx_ring: *mut igc_ring,
    pub rx_ring: *mut igc_ring,
    pub staterr: u32,
    pub n: u16 i,,
    if (!netif_msg_hw(adapter))
    netdev_info(netdev, "Device info: state %016lX trans_start %016lX\n",
    pub dev_trans_start(netdev)): netdev->state,,
// Print TX Ring Summary
    if (!netif_running(netdev))
    pub exit: goto,
    pub Summary\n"): netdev_info(netdev, "TX Rings,
    pub timestamp\n"): netdev_info(netdev, "Queue [NTU] [NTC] [bi(ntc)->dma ] leng ntw,
    pub {: for (n = 0; n < adapter->num_tx_queues; n++),
    pub buffer_info: *mut igc_tx_buffer,
    pub adapter->tx_ring[n]: tx_ring =,
    pub &tx_ring->tx_buffer_info[tx_ring->next_to_clean]: buffer_info =,
    netdev_info(netdev, "%5d %5X %5X %016llX %04X %p %016llX\n",
    n, tx_ring.next_to_use, tx_ring.next_to_clean,
    (u64)dma_unmap_addr(buffer_info, dma),
    dma_unmap_len(buffer_info, len),
    buffer_info.next_to_watch,
    }
// Print TX Rings
    if (!netif_msg_tx_done(adapter))
    pub rx_ring_summary: goto,
    pub Dump\n"): netdev_info(netdev, "TX Rings,
// Transmit Descriptor Formats
//
// Advanced Transmit Descriptor
// +--------------------------------------------------------------+
// 0 |         Buffer Address [63:0]                                |
// +--------------------------------------------------------------+
// 8 | PAYLEN  | PORTS  |CC|IDX | STA | DCMD  |DTYP|MAC|RSV| DTALEN |
// +--------------------------------------------------------------+
// 63      46 45    40 39 38 36 35 32 31   24             15       0
//
    pub {: for (n = 0; n < adapter->num_tx_queues; n++),
    pub adapter->tx_ring[n]: tx_ring =,
    pub "------------------------------------\n"): netdev_info(netdev,,
    netdev_info(netdev, "TX QUEUE INDEX = %d\n",
    pub "------------------------------------\n"): netdev_info(netdev,,
    pub bi->skb\n"): netdev_info(netdev, "T [desc] [address 63:0 ] [PlPOCIStDDM Ln] [bi->dma ] leng ntw timestamp,
    pub {: for (i = 0; tx_ring->desc && (i < tx_ring->count); i++),
    pub next_desc: *const c_char,
    pub buffer_info: *mut igc_tx_buffer,
    pub i): tx_desc = IGC_TX_DESC(tx_ring,,
    pub &tx_ring->tx_buffer_info[i]: buffer_info =,
    pub )tx_desc: *mut u0 = (struct my_u0,
    if (i == tx_ring.next_to_use &&
    i == tx_ring.next_to_clean)
    pub NTC/U": next_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(tx_ring->next_to_use: i ==) -> else {
    else if (i == tx_ring.next_to_use)
    pub NTU": next_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(tx_ring->next_to_clean: i ==) -> else {
    else if (i == tx_ring.next_to_clean)
    pub NTC": next_desc = ",
    else
    pub "": next_desc =,
    netdev_info(netdev, "T [0x%03X]    %016llX %016llX %016llX %04X  %p %016llX %p%s\n",
    i, le64_to_cpu(u0.a),
    le64_to_cpu(u0.b),
    (u64)dma_unmap_addr(buffer_info, dma),
    dma_unmap_len(buffer_info, len),
    buffer_info.next_to_watch,
    (u64)buffer_info.time_stamp,
    pub next_desc): buffer_info->skb,,
    if (netif_msg_pktdata(adapter) && buffer_info.skb)
    print_hex_dump(KERN_INFO, "",
    DUMP_PREFIX_ADDRESS,
    16, 1, buffer_info.skb.data,
    dma_unmap_len(buffer_info, len),
    }
    }
// Print RX Rings Summary
    rx_ring_summary:
    pub Summary\n"): netdev_info(netdev, "RX Rings,
    pub [NTC]\n"): netdev_info(netdev, "Queue [NTU],
    pub {: for (n = 0; n < adapter->num_rx_queues; n++),
    pub adapter->rx_ring[n]: rx_ring =,
    netdev_info(netdev, "%5d %5X %5X\n", n, rx_ring.next_to_use,
    }
// Print RX Rings
    if (!netif_msg_rx_status(adapter))
    pub exit: goto,
    pub Dump\n"): netdev_info(netdev, "RX Rings,
// Advanced Receive Descriptor (Read) Format
// 63                                           1        0
// +-----------------------------------------------------+
// 0 |       Packet Buffer Address [63:1]           |A0/NSE|
// +----------------------------------------------+------+
// 8 |       Header Buffer Address [63:1]           |  DD  |
// +-----------------------------------------------------+
//
// Advanced Receive Descriptor (Write-Back) Format
//
// 63       48 47    32 31  30      21 20 17 16   4 3     0
// +------------------------------------------------------+
// 0 | Packet     IP     |SPH| HDR_LEN   | RSV|Packet|  RSS |
// | Checksum   Ident  |   |           |    | Type | Type |
// +------------------------------------------------------+
// 8 | VLAN Tag | Length | Extended Error | Extended Status |
// +------------------------------------------------------+
// 63       48 47    32 31            20 19               0
//
    pub {: for (n = 0; n < adapter->num_rx_queues; n++),
    pub adapter->rx_ring[n]: rx_ring =,
    pub "------------------------------------\n"): netdev_info(netdev,,
    netdev_info(netdev, "RX QUEUE INDEX = %d\n",
    pub "------------------------------------\n"): netdev_info(netdev,,
    pub format\n"): netdev_info(netdev, "R [desc] [ PktBuf A0] [ HeadBuf DD] [bi->dma ] [bi->skb] <-- Adv Rx Read,
    pub format\n"): netdev_info(netdev, "RWB[desc] [PcsmIpSHl PtRs] [vl er S cks ln] ---------------- [bi->skb] <-- Adv Rx Write-Back,
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub next_desc: *const c_char,
    pub buffer_info: *mut igc_rx_buffer,
    pub &rx_ring->rx_buffer_info[i]: buffer_info =,
    pub i): rx_desc = IGC_RX_DESC(rx_ring,,
    pub )rx_desc: *mut u0 = (struct my_u0,
    pub le32_to_cpu(rx_desc->wb.upper.status_error): staterr =,
    if (i == rx_ring.next_to_use)
    pub NTU": next_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(rx_ring->next_to_clean: i ==) -> else {
    else if (i == rx_ring.next_to_clean)
    pub NTC": next_desc = ",
    else
    pub "": next_desc =,
    if (staterr & IGC_RXD_STAT_DD) {
// Descriptor Done
    netdev_info(netdev, "%s[0x%03X]     %016llX %016llX ---------------- %s\n",
    "RWB", i,
    le64_to_cpu(u0.a),
    le64_to_cpu(u0.b),
    } else {
    netdev_info(netdev, "%s[0x%03X]     %016llX %016llX %016llX %s\n",
    "R  ", i,
    le64_to_cpu(u0.a),
    le64_to_cpu(u0.b),
    (u64)buffer_info.dma,
    if (netif_msg_pktdata(adapter) &&
    buffer_info.dma && buffer_info.page) {
    print_hex_dump(KERN_INFO, "",
    DUMP_PREFIX_ADDRESS,
    16, 1,
    page_address
    (buffer_info.page) +
    buffer_info.page_offset,
    igc_rx_bufsz(rx_ring),
    }
    }
    }
    }
    exit:
    }
// igc_regs_dump - registers dump
#[no_mangle]
pub unsafe extern "C" fn igc_regs_dump(adapter: *mut igc_adapter) {
    void igc_regs_dump(struct igc_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut igc_hw hw =,
    pub reginfo: *mut igc_reg_info,
// Print Registers
    pub Dump\n"): netdev_info(adapter->netdev, "Register,
    pub Value\n"): netdev_info(adapter->netdev, "Register Name,
    pub )igc_reg_info_tbl: *mut for (reginfo = (struct igc_reg_info,
    pub {: reginfo->name; reginfo++),
    pub reginfo): igc_regdump(hw,,
    }
    }
