//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/e1000e/netdev.c
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
// Copyright(c) 1999 - 2018 Intel Corporation.

// Macro flag: #define CREATE_TRACE_POINTS

    char e1000e_driver_name[] = "e1000e";

    let mut debug: static int = -1;
    module_param(debug, int, 0);
    MODULE_PARM_DESC(debug, "Debug level (0=none,...,16=all)");
    static const struct e1000_info *e1000_info_tbl[] = {
    [board_82571]		= &e1000_82571_info,
    [board_82572]		= &e1000_82572_info,
    [board_82573]		= &e1000_82573_info,
    [board_82574]		= &e1000_82574_info,
    [board_82583]		= &e1000_82583_info,
    [board_80003es2lan]	= &e1000_es2_info,
    [board_ich8lan]		= &e1000_ich8_info,
    [board_ich9lan]		= &e1000_ich9_info,
    [board_ich10lan]	= &e1000_ich10_info,
    [board_pchlan]		= &e1000_pch_info,
    [board_pch2lan]		= &e1000_pch2_info,
    [board_pch_lpt]		= &e1000_pch_lpt_info,
    [board_pch_spt]		= &e1000_pch_spt_info,
    [board_pch_cnp]		= &e1000_pch_cnp_info,
    [board_pch_tgp]		= &e1000_pch_tgp_info,
    [board_pch_adp]		= &e1000_pch_adp_info,
    [board_pch_mtp]		= &e1000_pch_mtp_info,
    [board_pch_ptp]		= &e1000_pch_ptp_info,
    };
    static const struct dmi_system_id disable_k1_list[] = {
    {
    .ident = "Dell Pro 16 Plus PB16250",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Dell Pro 16 Plus PB16250"),
    },
    },
    {}
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_reg_info {
    pub ofs: u32,
    pub name: *mut c_char,
}

    static const struct e1000_reg_info e1000_reg_info_tbl[] = {
// General Registers
    {E1000_CTRL, "CTRL"},
    {E1000_STATUS, "STATUS"},
    {E1000_CTRL_EXT, "CTRL_EXT"},
// Interrupt Registers
    {E1000_ICR, "ICR"},
// Rx Registers
    {E1000_RCTL, "RCTL"},
    {E1000_RDLEN(0), "RDLEN"},
    {E1000_RDH(0), "RDH"},
    {E1000_RDT(0), "RDT"},
    {E1000_RDTR, "RDTR"},
    {E1000_RXDCTL(0), "RXDCTL"},
    {E1000_ERT, "ERT"},
    {E1000_RDBAL(0), "RDBAL"},
    {E1000_RDBAH(0), "RDBAH"},
    {E1000_RDFH, "RDFH"},
    {E1000_RDFT, "RDFT"},
    {E1000_RDFHS, "RDFHS"},
    {E1000_RDFTS, "RDFTS"},
    {E1000_RDFPC, "RDFPC"},
// Tx Registers
    {E1000_TCTL, "TCTL"},
    {E1000_TDBAL(0), "TDBAL"},
    {E1000_TDBAH(0), "TDBAH"},
    {E1000_TDLEN(0), "TDLEN"},
    {E1000_TDH(0), "TDH"},
    {E1000_TDT(0), "TDT"},
    {E1000_TIDV, "TIDV"},
    {E1000_TXDCTL(0), "TXDCTL"},
    {E1000_TADV, "TADV"},
    {E1000_TARC(0), "TARC"},
    {E1000_TDFH, "TDFH"},
    {E1000_TDFT, "TDFT"},
    {E1000_TDFHS, "TDFHS"},
    {E1000_TDFTS, "TDFTS"},
    {E1000_TDFPC, "TDFPC"},
// List Terminator
    {0, core::ptr::null_mut()}
    };
//
// __ew32_prepare - prepare to write to MAC CSR register on certain parts
// @hw: pointer to the HW structure
//
// When updating the MAC CSR registers, the Manageability Engine (ME) could
// be accessing the registers at the same time.  Normally, this is handled in
// h/w by an arbiter but on some parts there is a bug that acknowledges Host
// accesses later than it should which could result in the register to have
// an incorrect value.  Workaround this by checking the FWSM register which
// has bit 24 set while ME is accessing MAC CSR registers, wait if it is set
// and try again a number of times.
//
#[no_mangle]
unsafe extern "C" fn __ew32_prepare(hw: *mut e1000_hw) {
    static void __ew32_prepare(struct e1000_hw *hw)
    {
    let mut i: i32 = E1000_ICH_FWSM_PCIM2PCI_COUNT;
    while ((er32(FWSM) & E1000_ICH_FWSM_PCIM2PCI) && --i)
    udelay(50);
    }
#[no_mangle]
pub unsafe extern "C" fn __ew32(hw: *mut e1000_hw, reg: c_ulong, val: u32) {
    void __ew32(struct e1000_hw *hw, unsigned long reg, u32 val)
    {
    if (hw.adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    __ew32_prepare(hw);
    writel(val, hw.hw_addr + reg);
    }
//
// e1000_regdump - register printout routine
// @hw: pointer to the HW structure
// @reginfo: pointer to the register info table
//
#[no_mangle]
unsafe extern "C" fn e1000_regdump(hw: *mut e1000_hw, reginfo: *mut e1000_reg_info) {
    static void e1000_regdump(struct e1000_hw *hw, struct e1000_reg_info *reginfo)
    {
    let mut n: c_int = 0;
    char rname[16];
    u32 regs[8];
    switch (reginfo.ofs) {
    case E1000_RXDCTL(0):
    for (n = 0; n < 2; n++)
    regs[n] = __er32(hw, E1000_RXDCTL(n));
    break;
    case E1000_TXDCTL(0):
    for (n = 0; n < 2; n++)
    regs[n] = __er32(hw, E1000_TXDCTL(n));
    break;
    case E1000_TARC(0):
    for (n = 0; n < 2; n++)
    regs[n] = __er32(hw, E1000_TARC(n));
    break;
    default:
    pr_info("%-15s %08x\n",
    reginfo.name, __er32(hw, reginfo.ofs));
    return;
    }
    snprintf(rname, 16, "%s%s", reginfo.name, "[0-1]");
    pr_info("%-15s %08x %08x\n", rname, regs[0], regs[1]);
    }
    static void e1000e_dump_ps_pages(struct e1000_adapter *adapter,
    struct e1000_buffer *bi)
    {
    int i;
    struct e1000_ps_page *ps_page;
    for (i = 0; i < adapter.rx_ps_pages; i++) {
    ps_page = &bi.ps_pages[i];
    if (ps_page.page) {
    pr_info("packet dump for ps_page %d:\n", i);
    print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS,
    16, 1, page_address(ps_page.page),
    PAGE_SIZE, true);
    }
    }
    }
//
// e1000e_dump - Print registers, Tx-ring and Rx-ring
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn e1000e_dump(adapter: *mut e1000_adapter) {
    static void e1000e_dump(struct e1000_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct e1000_hw *hw = &adapter.hw;
    struct e1000_reg_info *reginfo;
    struct e1000_ring *tx_ring = adapter.tx_ring;
    struct e1000_tx_desc *tx_desc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_u0 {
    pub a: __le64,
    pub b: __le64,
    pub u0: *mut },
    pub buffer_info: *mut e1000_buffer,
    pub adapter->rx_ring: *mut *mut e1000_ring rx_ring =,
    pub rx_desc_ps: *mut union e1000_rx_desc_packet_split,
    pub rx_desc: *mut union e1000_rx_desc_extended,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_u1 {
    pub a: __le64,
    pub b: __le64,
    pub c: __le64,
    pub d: __le64,
    pub u1: *mut },
    pub staterr: u32,
    pub 0: int i =,
    if (!netif_msg_hw(adapter))
// Print netdevice Info
    if (netdev) {
    pub Info\n"): dev_info(&adapter->pdev->dev, "Net device,
    pub trans_start\n"): pr_info("Device Name state,
    pr_info("%-15s %016lX %016lX\n", netdev.name,
    pub dev_trans_start(netdev)): netdev->state,,
    }
// Print Registers
    pub Dump\n"): dev_info(&adapter->pdev->dev, "Register,
    pub Value\n"): pr_info(" Register Name,
    pub )e1000_reg_info_tbl: *mut for (reginfo = (struct e1000_reg_info,
    pub {: reginfo->name; reginfo++),
    pub reginfo): e1000_regdump(hw,,
    }
// Print Tx Ring Summary
    if (!netdev || !netif_running(netdev))
    pub Summary\n"): dev_info(&adapter->pdev->dev, "Tx Ring,
    pub timestamp\n"): pr_info("Queue [NTU] [NTC] [bi(ntc)->dma ] leng ntw,
    pub &tx_ring->buffer_info[tx_ring->next_to_clean]: buffer_info =,
    pr_info(" %5d %5X %5X %016llX %04X %3X %016llX\n",
    0, tx_ring.next_to_use, tx_ring.next_to_clean,
    (unsigned long long)buffer_info.dma,
    buffer_info.length,
    buffer_info.next_to_watch,
    pub long)buffer_info->time_stamp): (unsigned long,
// Print Tx Ring
    if (!netif_msg_tx_done(adapter))
    pub rx_ring_summary: goto,
    pub Dump\n"): dev_info(&adapter->pdev->dev, "Tx Ring,
// Transmit Descriptor Formats - DEXT[29] is 0 (Legacy) or 1 (Extended)
//
// Legacy Transmit Descriptor
// +--------------------------------------------------------------+
// 0 |         Buffer Address [63:0] (Reserved on Write Back)       |
// +--------------------------------------------------------------+
// 8 | Special  |    CSS     | Status |  CMD    |  CSO   |  Length  |
// +--------------------------------------------------------------+
// 63       48 47        36 35    32 31     24 23    16 15        0
//
// Extended Context Descriptor (DTYP=0x0) for TSO or checksum offload
// 63      48 47    40 39       32 31             16 15    8 7      0
// +----------------------------------------------------------------+
// 0 |  TUCSE  | TUCS0  |   TUCSS   |     IPCSE       | IPCS0 | IPCSS |
// +----------------------------------------------------------------+
// 8 |   MSS   | HDRLEN | RSV | STA | TUCMD | DTYP |      PAYLEN      |
// +----------------------------------------------------------------+
// 63      48 47    40 39 36 35 32 31   24 23  20 19                0
//
// Extended Data Descriptor (DTYP=0x1)
// +----------------------------------------------------------------+
// 0 |                     Buffer Address [63:0]                      |
// +----------------------------------------------------------------+
// 8 | VLAN tag |  POPTS  | Rsvd | Status | Command | DTYP |  DTALEN  |
// +----------------------------------------------------------------+
// 63       48 47     40 39  36 35    32 31     24 23  20 19        0
//
    pub format\n"): pr_info("Tl[desc] [address 63:0 ] [SpeCssSCmCsLen] [bi->dma ] leng ntw timestamp bi->skb <-- Legacy,
    pub format\n"): pr_info("Tc[desc] [Ce CoCsIpceCoS] [MssHlRSCm0Plen] [bi->dma ] leng ntw timestamp bi->skb <-- Ext Context,
    pub format\n"): pr_info("Td[desc] [address 63:0 ] [VlaPoRSCm1Dlen] [bi->dma ] leng ntw timestamp bi->skb <-- Ext Data,
    pub {: for (i = 0; tx_ring->desc && (i < tx_ring->count); i++),
    pub next_desc: *const c_char,
    pub i): *mut *mut tx_desc = E1000_TX_DESC(tx_ring,,
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub )tx_desc: *mut u0 = (struct my_u0,
    if (i == tx_ring.next_to_use && i == tx_ring.next_to_clean)
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
    pr_info("T%c[0x%03X]    %016llX %016llX %016llX %04X  %3X %016llX %p%s\n",
    (!(le64_to_cpu(u0.b) & BIT(29)) ? 'l' :
    ((le64_to_cpu(u0.b) & BIT(20)) ? 'd' : 'c')),
    i,
    (unsigned long long)le64_to_cpu(u0.a),
    (unsigned long long)le64_to_cpu(u0.b),
    (unsigned long long)buffer_info.dma,
    buffer_info.length, buffer_info.next_to_watch,
    (unsigned long long)buffer_info.time_stamp,
    pub next_desc): buffer_info->skb,,
    if (netif_msg_pktdata(adapter) && buffer_info.skb)
    print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS,
    16, 1, buffer_info.skb.data,
    pub true): buffer_info->skb->len,,
    }
// Print Rx Ring Summary
    rx_ring_summary:
    pub Summary\n"): dev_info(&adapter->pdev->dev, "Rx Ring,
    pub [NTC]\n"): pr_info("Queue [NTU],
    pr_info(" %5d %5X %5X\n",
    pub rx_ring->next_to_clean): 0, rx_ring->next_to_use,,
// Print Rx Ring
    if (!netif_msg_rx_status(adapter))
    pub Dump\n"): dev_info(&adapter->pdev->dev, "Rx Ring,
    switch (adapter.rx_ps_pages) {
    case 1:
    case 2:
    case 3:
// [Extended] Packet Split Receive Descriptor Format
//
// +-----------------------------------------------------+
// 0 |                Buffer Address 0 [63:0]              |
// +-----------------------------------------------------+
// 8 |                Buffer Address 1 [63:0]              |
// +-----------------------------------------------------+
// 16 |                Buffer Address 2 [63:0]              |
// +-----------------------------------------------------+
// 24 |                Buffer Address 3 [63:0]              |
// +-----------------------------------------------------+
//
    pub format\n"): pr_info("R [desc] [buffer 0 63:0 ] [buffer 1 63:0 ] [buffer 2 63:0 ] [buffer 3 63:0 ] [bi->dma ] [bi->skb] <-- Ext Pkt Split,
// [Extended] Receive Descriptor (Write-Back) Format
//
// 63       48 47    32 31     13 12    8 7    4 3        0
// +------------------------------------------------------+
// 0 | Packet   | IP     |  Rsvd   | MRQ   | Rsvd | MRQ RSS |
// | Checksum | Ident  |         | Queue |      |  Type   |
// +------------------------------------------------------+
// 8 | VLAN Tag | Length | Extended Error | Extended Status |
// +------------------------------------------------------+
// 63       48 47    32 31            20 19               0
//
    pub format\n"): pr_info("RWB[desc] [ck ipid mrqhsh] [vl l0 ee es] [ l3 l2 l1 hs] [reserved ] ---------------- [bi->skb] <-- Ext Rx Write-Back,
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub next_desc: *const c_char,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    pub i): *mut *mut rx_desc_ps = E1000_RX_DESC_PS(rx_ring,,
    pub )rx_desc_ps: *mut u1 = (struct my_u1,
    staterr =
    if (i == rx_ring.next_to_use)
    pub NTU": next_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(rx_ring->next_to_clean: i ==) -> else {
    else if (i == rx_ring.next_to_clean)
    pub NTC": next_desc = ",
    else
    pub "": next_desc =,
    if (staterr & E1000_RXD_STAT_DD) {
// Descriptor Done
    pr_info("%s[0x%03X]     %016llX %016llX %016llX %016llX ---------------- %p%s\n",
    "RWB", i,
    (unsigned long long)le64_to_cpu(u1.a),
    (unsigned long long)le64_to_cpu(u1.b),
    (unsigned long long)le64_to_cpu(u1.c),
    (unsigned long long)le64_to_cpu(u1.d),
    pub next_desc): buffer_info->skb,,
    } else {
    pr_info("%s[0x%03X]     %016llX %016llX %016llX %016llX %016llX %p%s\n",
    "R  ", i,
    (unsigned long long)le64_to_cpu(u1.a),
    (unsigned long long)le64_to_cpu(u1.b),
    (unsigned long long)le64_to_cpu(u1.c),
    (unsigned long long)le64_to_cpu(u1.d),
    (unsigned long long)buffer_info.dma,
    pub next_desc): buffer_info->skb,,
    if (netif_msg_pktdata(adapter))
    e1000e_dump_ps_pages(adapter,
    }
    }
    default:
    case 0:
// Extended Receive Descriptor (Read) Format
//
// +-----------------------------------------------------+
// 0 |                Buffer Address [63:0]                |
// +-----------------------------------------------------+
// 8 |                      Reserved                       |
// +-----------------------------------------------------+
//
    pub format\n"): pr_info("R [desc] [buf addr 63:0 ] [reserved 63:0 ] [bi->dma ] [bi->skb] <-- Ext (Read),
// Extended Receive Descriptor (Write-Back) Format
//
// 63       48 47    32 31    24 23            4 3        0
// +------------------------------------------------------+
// |     RSS Hash      |        |               |         |
// 0 +-------------------+  Rsvd  |   Reserved    | MRQ RSS |
// | Packet   | IP     |        |               |  Type   |
// | Checksum | Ident  |        |               |         |
// +------------------------------------------------------+
// 8 | VLAN Tag | Length | Extended Error | Extended Status |
// +------------------------------------------------------+
// 63       48 47    32 31            20 19               0
//
    pub format\n"): pr_info("RWB[desc] [cs ipid mrq] [vt ln xe xs] [bi->skb] <-- Ext (Write-Back),
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub next_desc: *const c_char,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    pub i): *mut *mut rx_desc = E1000_RX_DESC_EXT(rx_ring,,
    pub )rx_desc: *mut u1 = (struct my_u1,
    pub le32_to_cpu(rx_desc->wb.upper.status_error): staterr =,
    if (i == rx_ring.next_to_use)
    pub NTU": next_desc = ",
#[no_mangle]
pub unsafe extern "C" fn if(rx_ring->next_to_clean: i ==) -> else {
    else if (i == rx_ring.next_to_clean)
    pub NTC": next_desc = ",
    else
    pub "": next_desc =,
    if (staterr & E1000_RXD_STAT_DD) {
// Descriptor Done
    pr_info("%s[0x%03X]     %016llX %016llX ---------------- %p%s\n",
    "RWB", i,
    (unsigned long long)le64_to_cpu(u1.a),
    (unsigned long long)le64_to_cpu(u1.b),
    pub next_desc): buffer_info->skb,,
    } else {
    pr_info("%s[0x%03X]     %016llX %016llX %016llX %p%s\n",
    "R  ", i,
    (unsigned long long)le64_to_cpu(u1.a),
    (unsigned long long)le64_to_cpu(u1.b),
    (unsigned long long)buffer_info.dma,
    pub next_desc): buffer_info->skb,,
    if (netif_msg_pktdata(adapter) &&
    buffer_info.skb)
    print_hex_dump(KERN_INFO, "",
    DUMP_PREFIX_ADDRESS, 16,
    1,
    buffer_info.skb.data,
    adapter.rx_buffer_len,
    }
    }
    }
    }
//
// e1000_desc_unused - calculate if we have unused descriptors
// @ring: pointer to ring struct to perform calculation on
//
#[no_mangle]
unsafe extern "C" fn e1000_desc_unused(ring: *mut e1000_ring) -> c_int {
    static int e1000_desc_unused(struct e1000_ring *ring)
    {
    if (ring.next_to_clean > ring.next_to_use)
    pub 1: return ring->next_to_clean - ring->next_to_use -,
    pub 1: return ring->count + ring->next_to_clean - ring->next_to_use -,
    }
//
// e1000e_systim_to_hwtstamp - convert system time value to hw time stamp
// @adapter: board private structure
// @hwtstamps: time stamp structure to update
// @systim: unsigned 64bit system time value.
//
// Convert the system time value stored in the RX/TXSTMP registers into a
// hwtstamp which can be used by the upper level time stamping functions.
//
// The 'systim_lock' spinlock is used to protect the consistency of the
// system time value. This is needed because reading the 64 bit time
// value involves reading two 32 bit registers. The first read latches the
// value.
//
    static void e1000e_systim_to_hwtstamp(struct e1000_adapter *adapter,
    struct skb_shared_hwtstamps *hwtstamps,
    u64 systim)
    {
    pub ns: u64,
    pub flags: c_ulong,
    pub flags): spin_lock_irqsave(&adapter->systim_lock,,
    pub systim): ns = timecounter_cyc2time(&adapter->tc,,
    pub flags): spin_unlock_irqrestore(&adapter->systim_lock,,
    pub sizeof(*hwtstamps)): *mut memset(hwtstamps, 0,,
    pub ns_to_ktime(ns): hwtstamps->hwtstamp =,
    }
//
// e1000e_rx_hwtstamp - utility function which checks for Rx time stamp
// @adapter: board private structure
// @status: descriptor extended error and status field
// @skb: particular skb to include time stamp
//
// If the time stamp is valid, convert it into the timecounter ns value
// and store that result into the shhwtstamps structure which is passed
// up the network stack.
//
    static void e1000e_rx_hwtstamp(struct e1000_adapter *adapter, u32 status,
    struct sk_buff *skb)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rxstmp: u64,
    if (!(adapter.flags & FLAG_HAS_HW_TIMESTAMP) ||
    !(status & E1000_RXDEXT_STATERR_TST) ||
    !(er32(TSYNCRXCTL) & E1000_TSYNCRXCTL_VALID))
// The Rx time stamp registers contain the time stamp.  No other
// received packet will be time stamped until the Rx time stamp
// registers are read.  Because only one packet can be time stamped
// at a time, the register values must belong to this packet and
// therefore none of the other additional attributes need to be
// compared.
//
    pub (u64)er32(RXSTMPL): rxstmp =,
    pub 32: rxstmp |= (u64)er32(RXSTMPH) <<,
    pub rxstmp): e1000e_systim_to_hwtstamp(adapter, skb_hwtstamps(skb),,
    pub ~FLAG2_CHECK_RX_HWTSTAMP: adapter->flags2 &=,
    }
//
// e1000_receive_skb - helper function to handle Rx indications
// @adapter: board private structure
// @netdev: pointer to netdev struct
// @staterr: descriptor extended error and status field as written by hardware
// @vlan: descriptor vlan field as written by hardware (no le/be conversion)
// @skb: pointer to sk_buff to be indicated to stack
//
    static void e1000_receive_skb(struct e1000_adapter *adapter,
    struct net_device *netdev, struct sk_buff *skb,
    u32 staterr, __le16 vlan)
    {
    pub le16_to_cpu(vlan): u16 tag =,
    pub skb): e1000e_rx_hwtstamp(adapter, staterr,,
    pub netdev): skb->protocol = eth_type_trans(skb,,
    if (staterr & E1000_RXD_STAT_VP)
    pub tag): __vlan_hwaccel_put_tag(skb, htons(ETH_P_8021Q),,
    pub skb): napi_gro_receive(&adapter->napi,,
    }
//
// e1000_rx_checksum - Receive Checksum Offload
// @adapter: board private structure
// @status_err: receive descriptor status and error fields
// @skb: socket buffer with received data
//
    static void e1000_rx_checksum(struct e1000_adapter *adapter, u32 status_err,
    struct sk_buff *skb)
    {
    pub (u16)status_err: u16 status =,
    pub 24): u8 errors = (u8)(status_err >>,
// Rx checksum disabled
    if (!(adapter.netdev.features & NETIF_F_RXCSUM))
// Ignore Checksum bit is set
    if (status & E1000_RXD_STAT_IXSM)
// TCP/UDP checksum error bit or IP checksum error bit is set
    if (errors & (E1000_RXD_ERR_TCPE | E1000_RXD_ERR_IPE)) {
// let the stack verify checksum errors
    }
// TCP/UDP Checksum has not been calculated
    if (!(status & (E1000_RXD_STAT_TCPCS | E1000_RXD_STAT_UDPCS)))
// It must be a TCP or UDP packet with a valid checksum
    pub CHECKSUM_UNNECESSARY: skb->ip_summed =,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_update_rdt_wa(rx_ring: *mut e1000_ring, i: c_uint) {
    static void e1000e_update_rdt_wa(struct e1000_ring *rx_ring, unsigned int i)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rx_ring->tail): writel(i,,
    if (unlikely(i != readl(rx_ring.tail))) {
    pub er32(RCTL): u32 rctl =,
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
    pub resetting\n"): e_err("ME firmware caused invalid RDT -,
    }
    }
#[no_mangle]
unsafe extern "C" fn e1000e_update_tdt_wa(tx_ring: *mut e1000_ring, i: c_uint) {
    static void e1000e_update_tdt_wa(struct e1000_ring *tx_ring, unsigned int i)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub tx_ring->tail): writel(i,,
    if (unlikely(i != readl(tx_ring.tail))) {
    pub er32(TCTL): u32 tctl =,
    pub ~E1000_TCTL_EN): ew32(TCTL, tctl &,
    pub resetting\n"): e_err("ME firmware caused invalid TDT -,
    }
    }
//
// e1000_alloc_rx_buffers - Replace used receive buffers
// @rx_ring: Rx descriptor ring
// @cleaned_count: number to reallocate
// @gfp: flags for allocation
//
    static void e1000_alloc_rx_buffers(struct e1000_ring *rx_ring,
    int cleaned_count, gfp_t gfp)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub rx_desc: *mut union e1000_rx_desc_extended,
    pub buffer_info: *mut e1000_buffer,
    pub skb: *mut sk_buff,
    pub i: c_uint,
    pub adapter->rx_buffer_len: unsigned int bufsz =,
    pub rx_ring->next_to_use: i =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    while (cleaned_count--) {
    pub buffer_info->skb: skb =,
    if (skb) {
    pub 0): skb_trim(skb,,
    pub map_skb: goto,
    }
    pub gfp): skb = __netdev_alloc_skb_ip_align(netdev, bufsz,,
    if (!skb) {
// Better luck next round
    }
    pub skb: buffer_info->skb =,
    map_skb:
    buffer_info.dma = dma_map_single(&pdev.dev, skb.data,
    adapter.rx_buffer_len,
    if (dma_mapping_error(&pdev.dev, buffer_info.dma)) {
    pub failed\n"): dev_err(&pdev->dev, "Rx DMA map,
    }
    pub i): *mut *mut rx_desc = E1000_RX_DESC_EXT(rx_ring,,
    pub cpu_to_le64(buffer_info->dma): rx_desc->read.buffer_addr =,
    if (unlikely(!(i & (E1000_RX_BUFFER_WRITE - 1)))) {
// Force memory writes to complete before letting h/w
// know there are new descriptors to fetch.  (Only
// applicable for weak-ordered memory model archs,
// such as IA-64).
//
    if (adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    pub i): e1000e_update_rdt_wa(rx_ring,,
    else
    pub rx_ring->tail): writel(i,,
    }
    if (i == rx_ring.count)
    pub 0: i =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    }
    pub i: rx_ring->next_to_use =,
    }
//
// e1000_alloc_rx_buffers_ps - Replace used receive buffers; packet split
// @rx_ring: Rx descriptor ring
// @cleaned_count: number to reallocate
// @gfp: flags for allocation
//
    static void e1000_alloc_rx_buffers_ps(struct e1000_ring *rx_ring,
    int cleaned_count, gfp_t gfp)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub rx_desc: *mut union e1000_rx_desc_packet_split,
    pub buffer_info: *mut e1000_buffer,
    pub ps_page: *mut e1000_ps_page,
    pub skb: *mut sk_buff,
    pub j: unsigned int i,,
    pub rx_ring->next_to_use: i =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    while (cleaned_count--) {
    pub i): *mut *mut rx_desc = E1000_RX_DESC_PS(rx_ring,,
    pub {: for (j = 0; j < PS_PAGE_BUFFERS; j++),
    pub &buffer_info->ps_pages[j]: ps_page =,
    if (j >= adapter.rx_ps_pages) {
// all unused desc entries get hw null ptr
    rx_desc.read.buffer_addr[j + 1] =
    }
    if (!ps_page.page) {
    pub alloc_page(gfp): ps_page->page =,
    if (!ps_page.page) {
    pub no_buffers: goto,
    }
    ps_page.dma = dma_map_page(&pdev.dev,
    ps_page.page,
    0, PAGE_SIZE,
    if (dma_mapping_error(&pdev.dev,
    ps_page.dma)) {
    dev_err(&adapter.pdev.dev,
    pub failed\n"): "Rx DMA page map,
    pub no_buffers: goto,
    }
    }
// Refresh the desc even if buffer_addrs
// didn't change because each write-back
// erases this info.
//
    rx_desc.read.buffer_addr[j + 1] =
    }
    skb = __netdev_alloc_skb_ip_align(netdev, adapter.rx_ps_bsize0,
    if (!skb) {
    }
    pub skb: buffer_info->skb =,
    buffer_info.dma = dma_map_single(&pdev.dev, skb.data,
    adapter.rx_ps_bsize0,
    if (dma_mapping_error(&pdev.dev, buffer_info.dma)) {
    pub failed\n"): dev_err(&pdev->dev, "Rx DMA map,
// cleanup skb
    pub NULL: buffer_info->skb =,
    }
    pub cpu_to_le64(buffer_info->dma): rx_desc->read.buffer_addr[0] =,
    if (unlikely(!(i & (E1000_RX_BUFFER_WRITE - 1)))) {
// Force memory writes to complete before letting h/w
// know there are new descriptors to fetch.  (Only
// applicable for weak-ordered memory model archs,
// such as IA-64).
//
    if (adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    pub 1): e1000e_update_rdt_wa(rx_ring, i <<,
    else
    pub rx_ring->tail): writel(i << 1,,
    }
    if (i == rx_ring.count)
    pub 0: i =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    }
    no_buffers:
    pub i: rx_ring->next_to_use =,
    }
//
// e1000_alloc_jumbo_rx_buffers - Replace used jumbo receive buffers
// @rx_ring: Rx descriptor ring
// @cleaned_count: number of buffers to allocate this pass
// @gfp: flags for allocation
//
    static void e1000_alloc_jumbo_rx_buffers(struct e1000_ring *rx_ring,
    int cleaned_count, gfp_t gfp)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub rx_desc: *mut union e1000_rx_desc_extended,
    pub buffer_info: *mut e1000_buffer,
    pub skb: *mut sk_buff,
    pub i: c_uint,
    pub /: *mut *mut unsigned int bufsz = 256 - 16; / for skb_reserve,
    pub rx_ring->next_to_use: i =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    while (cleaned_count--) {
    pub buffer_info->skb: skb =,
    if (skb) {
    pub 0): skb_trim(skb,,
    pub check_page: goto,
    }
    pub gfp): skb = __netdev_alloc_skb_ip_align(netdev, bufsz,,
    if (unlikely(!skb)) {
// Better luck next round
    }
    pub skb: buffer_info->skb =,
    check_page:
// allocate a new page if necessary
    if (!buffer_info.page) {
    pub alloc_page(gfp): buffer_info->page =,
    if (unlikely(!buffer_info.page)) {
    }
    }
    if (!buffer_info.dma) {
    buffer_info.dma = dma_map_page(&pdev.dev,
    buffer_info.page, 0,
    PAGE_SIZE,
    if (dma_mapping_error(&pdev.dev, buffer_info.dma)) {
    }
    }
    pub i): *mut *mut rx_desc = E1000_RX_DESC_EXT(rx_ring,,
    pub cpu_to_le64(buffer_info->dma): rx_desc->read.buffer_addr =,
    if (unlikely(++i == rx_ring.count))
    pub 0: i =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    }
    if (likely(rx_ring.next_to_use != i)) {
    pub i: rx_ring->next_to_use =,
    if (unlikely(i-- == 0))
    pub 1): i = (rx_ring->count -,
// Force memory writes to complete before letting h/w
// know there are new descriptors to fetch.  (Only
// applicable for weak-ordered memory model archs,
// such as IA-64).
//
    if (adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    pub i): e1000e_update_rdt_wa(rx_ring,,
    else
    pub rx_ring->tail): writel(i,,
    }
    }
    static inline void e1000_rx_hash(struct net_device *netdev, __le32 rss,
    struct sk_buff *skb)
    {
    if (netdev.features & NETIF_F_RXHASH)
    pub PKT_HASH_TYPE_L3): skb_set_hash(skb, le32_to_cpu(rss),,
    }
//
// e1000_clean_rx_irq - Send received data up the network stack
// @rx_ring: Rx descriptor ring
// @work_done: output parameter for indicating completed work
// @work_to_do: how many packets we can clean
//
// the return value indicates whether actual cleaning was done, there
// is no guarantee that everything was cleaned
//
    static bool e1000_clean_rx_irq(struct e1000_ring *rx_ring, int *work_done,
    int work_to_do)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub next_rxd: *mut *mut union e1000_rx_desc_extended rx_desc,,
    pub next_buffer: *mut *mut e1000_buffer buffer_info,,
    pub staterr: u32 length,,
    pub i: c_uint,
    pub 0: int cleaned_count =,
    pub false: bool cleaned =,
    pub 0: unsigned int total_rx_bytes = 0, total_rx_packets =,
    pub rx_ring->next_to_clean: i =,
    pub i): *mut *mut rx_desc = E1000_RX_DESC_EXT(rx_ring,,
    pub le32_to_cpu(rx_desc->wb.upper.status_error): staterr =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    while (staterr & E1000_RXD_STAT_DD) {
    pub skb: *mut sk_buff,
    if (*work_done >= work_to_do)
    pub /: *mut *mut dma_rmb(); / read descriptor and rx_buffer_info after status DD,
    pub buffer_info->skb: skb =,
    pub NULL: buffer_info->skb =,
    pub NET_IP_ALIGN): prefetch(skb->data -,
    if (i == rx_ring.count)
    pub 0: i =,
    pub i): *mut *mut next_rxd = E1000_RX_DESC_EXT(rx_ring,,
    pub &rx_ring->buffer_info[i]: next_buffer =,
    pub true: cleaned =,
    dma_unmap_single(&pdev.dev, buffer_info.dma,
    pub DMA_FROM_DEVICE): adapter->rx_buffer_len,,
    pub 0: buffer_info->dma =,
    pub le16_to_cpu(rx_desc->wb.upper.length): length =,
// !EOP means multiple descriptors were used to store a single
// packet, if that's the case we need to toss it.  In fact, we
// need to toss every packet with the EOP bit clear and the
// next frame that _does_ have the EOP bit set, as it is by
// definition only a frame fragment
//
    if (unlikely(!(staterr & E1000_RXD_STAT_EOP)))
    pub FLAG2_IS_DISCARDING: adapter->flags2 |=,
    if (adapter.flags2 & FLAG2_IS_DISCARDING) {
// All receives must fit into a single buffer
    pub buffers\n"): e_dbg("Receive packet consumed multiple,
// recycle
    pub skb: buffer_info->skb =,
    if (staterr & E1000_RXD_STAT_EOP)
    pub ~FLAG2_IS_DISCARDING: adapter->flags2 &=,
    pub next_desc: goto,
    }
    if (unlikely((staterr & E1000_RXDEXT_ERR_FRAME_ERR_MASK) &&
    !(netdev.features & NETIF_F_RXALL))) {
// recycle
    pub skb: buffer_info->skb =,
    pub next_desc: goto,
    }
// adjust length to remove Ethernet CRC
    if (!(adapter.flags2 & FLAG2_CRC_STRIPPING)) {
// If configured to store CRC, don't subtract FCS,
// but keep the FCS bytes out of the total_rx_bytes
// counter
//
    if (netdev.features & NETIF_F_RXFCS)
    pub 4: total_rx_bytes -=,
    else
    pub 4: length -=,
    }
    pub length: total_rx_bytes +=,
// code added for copybreak, this should improve
// performance for small packets with large amounts
// of reassembly being done in the stack
//
    if (length < copybreak) {
    struct sk_buff *new_skb =
    pub length): napi_alloc_skb(&adapter->napi,,
    if (new_skb) {
    skb_copy_to_linear_data_offset(new_skb,
    -NET_IP_ALIGN,
    (skb.data -
    NET_IP_ALIGN),
    (length +
// save the skb in buffer_info as good
    pub skb: buffer_info->skb =,
    pub new_skb: skb =,
    }
// else just continue with the old one
    }
// end copybreak code
    pub length): skb_put(skb,,
// Receive Checksum Offload
    pub skb): e1000_rx_checksum(adapter, staterr,,
    pub skb): e1000_rx_hash(netdev, rx_desc->wb.lower.hi_dword.rss,,
    e1000_receive_skb(adapter, netdev, skb, staterr,
    next_desc:
    pub cpu_to_le32(~0xFF): rx_desc->wb.upper.status_error &=,
// return some buffers to hardware, one at a time is too slow
    if (cleaned_count >= E1000_RX_BUFFER_WRITE) {
    adapter.alloc_rx_buf(rx_ring, cleaned_count,
    pub 0: cleaned_count =,
    }
// use prefetched values
    pub next_rxd: rx_desc =,
    pub next_buffer: buffer_info =,
    pub le32_to_cpu(rx_desc->wb.upper.status_error): staterr =,
    }
    pub i: rx_ring->next_to_clean =,
    pub e1000_desc_unused(rx_ring): cleaned_count =,
    if (cleaned_count)
    pub GFP_ATOMIC): adapter->alloc_rx_buf(rx_ring, cleaned_count,,
    pub total_rx_bytes: adapter->total_rx_bytes +=,
    pub total_rx_packets: adapter->total_rx_packets +=,
    pub cleaned: return,
    }
    static void e1000_put_txbuf(struct e1000_ring *tx_ring,
    struct e1000_buffer *buffer_info,
    bool drop)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    if (buffer_info.dma) {
    if (buffer_info.mapped_as_page)
    dma_unmap_page(&adapter.pdev.dev, buffer_info.dma,
    pub DMA_TO_DEVICE): buffer_info->length,,
    else
    dma_unmap_single(&adapter.pdev.dev, buffer_info.dma,
    pub DMA_TO_DEVICE): buffer_info->length,,
    pub 0: buffer_info->dma =,
    }
    if (buffer_info.skb) {
    if (drop)
    else
    pub NULL: buffer_info->skb =,
    }
    pub 0: buffer_info->time_stamp =,
    }
#[no_mangle]
unsafe extern "C" fn e1000_print_hw_hang(work: *mut work_struct) {
    static void e1000_print_hw_hang(struct work_struct *work)
    {
    struct e1000_adapter *adapter = container_of(work,
    struct e1000_adapter,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub tx_ring->next_to_clean: unsigned int i =,
    pub tx_ring->buffer_info[i].next_to_watch: unsigned int eop =,
    pub eop): *mut *mut *mut e1000_tx_desc eop_desc = E1000_TX_DESC(tx_ring,,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub phy_ext_status: u16 phy_status, phy_1000t_status,,
    pub pci_status: u16,
    if (test_bit(__E1000_DOWN, &adapter.state))
    if (!adapter.tx_hang_recheck && (adapter.flags2 & FLAG2_DMA_BURST)) {
// May be block on write-back, flush and detect again
// flush pending descriptor writebacks to memory
//
    pub E1000_TIDV_FPD): ew32(TIDV, adapter->tx_int_delay |,
// execute the writes immediately
// Due to rare timing issues, write to TIDV again to ensure
// the write is successful
//
    pub E1000_TIDV_FPD): ew32(TIDV, adapter->tx_int_delay |,
// execute the writes immediately
    pub true: adapter->tx_hang_recheck =,
    }
    pub false: adapter->tx_hang_recheck =,
    if (er32(TDH(0)) == er32(TDT(0))) {
    pub ignoring\n"): e_dbg("false hang detected,,
    }
// Real hang detected
    pub &phy_status): e1e_rphy(hw, MII_BMSR,,
    pub &phy_1000t_status): e1e_rphy(hw, MII_STAT1000,,
    pub &phy_ext_status): e1e_rphy(hw, MII_ESTATUS,,
    pub &pci_status): pci_read_config_word(adapter->pdev, PCI_STATUS,,
// detected Hardware unit hang
    e_err("Detected Hardware Unit Hang:\n"
    "  TDH                  <%x>\n"
    "  TDT                  <%x>\n"
    "  next_to_use          <%x>\n"
    "  next_to_clean        <%x>\n"
    "buffer_info[next_to_clean]:\n"
    "  time_stamp           <%lx>\n"
    "  next_to_watch        <%x>\n"
    "  jiffies              <%lx>\n"
    "  next_to_watch.status <%x>\n"
    "MAC Status             <%x>\n"
    "PHY Status             <%x>\n"
    "PHY 1000BASE-T Status  <%x>\n"
    "PHY Extended Status    <%x>\n"
    "PCI Status             <%x>\n",
    readl(tx_ring.head), readl(tx_ring.tail), tx_ring.next_to_use,
    tx_ring.next_to_clean, tx_ring.buffer_info[eop].time_stamp,
    eop, jiffies, eop_desc.upper.fields.status, er32(STATUS),
    pub pci_status): phy_status, phy_1000t_status, phy_ext_status,,
// Suggest workaround for known h/w issue
    if ((hw.mac.type == e1000_pchlan) && (er32(CTRL) & E1000_CTRL_TFCE))
    pub ethtool\n"): e_err("Try turning off Tx pause (flow control) via,
    }
//
// e1000e_tx_hwtstamp_work - check for Tx time stamp
// @work: pointer to work struct
//
// This work function polls the TSYNCTXCTL valid bit to determine when a
// timestamp has been taken for the current stored skb.  The timestamp must
// be for this skb because only one such packet is allowed in the queue.
//
#[no_mangle]
unsafe extern "C" fn e1000e_tx_hwtstamp_work(work: *mut work_struct) {
    static void e1000e_tx_hwtstamp_work(struct work_struct *work)
    {
    struct e1000_adapter *adapter = container_of(work, struct e1000_adapter,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    if (er32(TSYNCTXCTL) & E1000_TSYNCTXCTL_VALID) {
    pub adapter->tx_hwtstamp_skb: *mut *mut sk_buff skb =,
    pub shhwtstamps: skb_shared_hwtstamps,
    pub txstmp: u64,
    pub er32(TXSTMPL): txstmp =,
    pub 32: txstmp |= (u64)er32(TXSTMPH) <<,
    pub txstmp): e1000e_systim_to_hwtstamp(adapter, &shhwtstamps,,
// Clear the global tx_hwtstamp_skb pointer and force writes
// prior to notifying the stack of a Tx timestamp.
//
    pub NULL: adapter->tx_hwtstamp_skb =,
    pub /: *mut *mut wmb(); / force write prior to skb_tstamp_tx,
    pub &shhwtstamps): skb_tstamp_tx(skb,,
    } else if (time_after(jiffies, adapter.tx_hwtstamp_start
    + adapter.tx_timeout_factor * HZ)) {
    pub NULL: adapter->tx_hwtstamp_skb =,
    pub hang\n"): e_warn("clearing Tx timestamp,
    } else {
// reschedule to check later
    }
    }
//
// e1000_clean_tx_irq - Reclaim resources after transmit completes
// @tx_ring: Tx descriptor ring
//
// the return value indicates whether actual cleaning was done, there
// is no guarantee that everything was cleaned
//
#[no_mangle]
unsafe extern "C" fn e1000_clean_tx_irq(tx_ring: *mut e1000_ring) -> bool {
    static bool e1000_clean_tx_irq(struct e1000_ring *tx_ring)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub eop_desc: *mut *mut e1000_tx_desc tx_desc,,
    pub buffer_info: *mut e1000_buffer,
    pub eop: unsigned int i,,
    pub 0: unsigned int count =,
    pub 0: unsigned int total_tx_bytes = 0, total_tx_packets =,
    pub 0: unsigned int bytes_compl = 0, pkts_compl =,
    pub tx_ring->next_to_clean: i =,
    pub tx_ring->buffer_info[i].next_to_watch: eop =,
    pub eop): *mut *mut eop_desc = E1000_TX_DESC(tx_ring,,
    while ((eop_desc.upper.data & cpu_to_le32(E1000_TXD_STAT_DD)) &&
    (count < tx_ring.count)) {
    pub false: bool cleaned =,
    pub /: *mut *mut dma_rmb(); / read buffer_info after eop_desc,
    pub {: for (; !cleaned; count++),
    pub i): *mut *mut tx_desc = E1000_TX_DESC(tx_ring,,
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub eop): cleaned = (i ==,
    if (cleaned) {
    pub buffer_info->segs: total_tx_packets +=,
    pub buffer_info->bytecount: total_tx_bytes +=,
    if (buffer_info.skb) {
    pub buffer_info->skb->len: bytes_compl +=,
    }
    }
    pub false): e1000_put_txbuf(tx_ring, buffer_info,,
    pub 0: tx_desc->upper.data =,
    if (i == tx_ring.count)
    pub 0: i =,
    }
    if (i == tx_ring.next_to_use)
    pub tx_ring->buffer_info[i].next_to_watch: eop =,
    pub eop): *mut *mut eop_desc = E1000_TX_DESC(tx_ring,,
    }
    pub i: tx_ring->next_to_clean =,
    pub bytes_compl): netdev_completed_queue(netdev, pkts_compl,,
pub const TX_WAKE_THRESHOLD: c_int = 32;
    if (count && netif_carrier_ok(netdev) &&
    e1000_desc_unused(tx_ring) >= TX_WAKE_THRESHOLD) {
// Make sure that anybody stopping the queue after this
// sees the new next_to_clean.
//
    if (netif_queue_stopped(netdev) &&
    !(test_bit(__E1000_DOWN, &adapter.state))) {
    }
    }
    if (adapter.detect_tx_hung) {
// Detect a transmit hang in hardware, this serializes the
// check with the clearing of time_stamp and movement of i
//
    pub false: adapter->detect_tx_hung =,
    if (tx_ring.buffer_info[i].time_stamp &&
    time_after(jiffies, tx_ring.buffer_info[i].time_stamp
    + (adapter.tx_timeout_factor * HZ)) &&
    !(er32(STATUS) & E1000_STATUS_TXOFF))
    else
    pub false: adapter->tx_hang_recheck =,
    }
    pub total_tx_bytes: adapter->total_tx_bytes +=,
    pub total_tx_packets: adapter->total_tx_packets +=,
    pub tx_ring->count: return count <,
    }
//
// e1000_clean_rx_irq_ps - Send received data up the network stack; packet split
// @rx_ring: Rx descriptor ring
// @work_done: output parameter for indicating completed work
// @work_to_do: how many packets we can clean
//
// the return value indicates whether actual cleaning was done, there
// is no guarantee that everything was cleaned
//
    static bool e1000_clean_rx_irq_ps(struct e1000_ring *rx_ring, int *work_done,
    int work_to_do)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub next_rxd: *mut *mut union e1000_rx_desc_packet_split rx_desc,,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub next_buffer: *mut *mut e1000_buffer buffer_info,,
    pub ps_page: *mut e1000_ps_page,
    pub skb: *mut sk_buff,
    pub j: unsigned int i,,
    pub staterr: u32 length,,
    pub 0: int cleaned_count =,
    pub false: bool cleaned =,
    pub 0: unsigned int total_rx_bytes = 0, total_rx_packets =,
    pub rx_ring->next_to_clean: i =,
    pub i): *mut *mut rx_desc = E1000_RX_DESC_PS(rx_ring,,
    pub le32_to_cpu(rx_desc->wb.middle.status_error): staterr =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    while (staterr & E1000_RXD_STAT_DD) {
    if (*work_done >= work_to_do)
    pub buffer_info->skb: skb =,
    pub /: *mut *mut dma_rmb(); / read descriptor and rx_buffer_info after status DD,
// in the packet split case this is header only
    pub NET_IP_ALIGN): prefetch(skb->data -,
    if (i == rx_ring.count)
    pub 0: i =,
    pub i): *mut *mut next_rxd = E1000_RX_DESC_PS(rx_ring,,
    pub &rx_ring->buffer_info[i]: next_buffer =,
    pub true: cleaned =,
    dma_unmap_single(&pdev.dev, buffer_info.dma,
    pub DMA_FROM_DEVICE): adapter->rx_ps_bsize0,,
    pub 0: buffer_info->dma =,
// see !EOP comment in other Rx routine
    if (!(staterr & E1000_RXD_STAT_EOP))
    pub FLAG2_IS_DISCARDING: adapter->flags2 |=,
    if (adapter.flags2 & FLAG2_IS_DISCARDING) {
    pub packet\n"): e_dbg("Packet Split buffers didn't pick up the full,
    if (staterr & E1000_RXD_STAT_EOP)
    pub ~FLAG2_IS_DISCARDING: adapter->flags2 &=,
    pub next_desc: goto,
    }
    if (unlikely((staterr & E1000_RXDEXT_ERR_FRAME_ERR_MASK) &&
    !(netdev.features & NETIF_F_RXALL))) {
    pub next_desc: goto,
    }
    pub le16_to_cpu(rx_desc->wb.middle.length0): length =,
    if (!length) {
    pub descriptors\n"): e_dbg("Last part of the packet spanning multiple,
    pub next_desc: goto,
    }
// Good Receive
    pub length): skb_put(skb,,
    {
// this looks ugly, but it seems compiler issues make
// it more efficient than reusing j
//
    pub le16_to_cpu(rx_desc->wb.upper.length[0]): int l1 =,
// page alloc/put takes too long and effects small
// packet throughput, so unsplit small packets and
// save the alloc/put
//
    if (l1 && (l1 <= copybreak) &&
    ((length + l1) <= adapter.rx_ps_bsize0)) {
    pub &buffer_info->ps_pages[0]: ps_page =,
    dma_sync_single_for_cpu(&pdev.dev,
    ps_page.dma,
    PAGE_SIZE,
    memcpy(skb_tail_pointer(skb),
    pub l1): page_address(ps_page->page),,
    dma_sync_single_for_device(&pdev.dev,
    ps_page.dma,
    PAGE_SIZE,
// remove the CRC
    if (!(adapter.flags2 & FLAG2_CRC_STRIPPING)) {
    if (!(netdev.features & NETIF_F_RXFCS))
    pub 4: l1 -=,
    }
    pub l1): skb_put(skb,,
    pub copydone: goto,
    }	/* if */
    }
    pub {: for (j = 0; j < PS_PAGE_BUFFERS; j++),
    pub le16_to_cpu(rx_desc->wb.upper.length[j]): length =,
    if (!length)
    pub &buffer_info->ps_pages[j]: ps_page =,
    dma_unmap_page(&pdev.dev, ps_page.dma, PAGE_SIZE,
    pub 0: ps_page->dma =,
    pub length): skb_fill_page_desc(skb, j, ps_page->page, 0,,
    pub NULL: ps_page->page =,
    pub length: skb->len +=,
    pub length: skb->data_len +=,
    pub PAGE_SIZE: skb->truesize +=,
    }
// strip the ethernet crc, problem is we're using pages now so
// this whole operation can get a little cpu intensive
//
    if (!(adapter.flags2 & FLAG2_CRC_STRIPPING)) {
    if (!(netdev.features & NETIF_F_RXFCS))
    pub 4): pskb_trim(skb, skb->len -,
    }
    copydone:
    pub skb->len: total_rx_bytes +=,
    pub skb): e1000_rx_checksum(adapter, staterr,,
    pub skb): e1000_rx_hash(netdev, rx_desc->wb.lower.hi_dword.rss,,
    if (rx_desc.wb.upper.header_status &
    cpu_to_le16(E1000_RXDPS_HDRSTAT_HDRSP))
    e1000_receive_skb(adapter, netdev, skb, staterr,
    next_desc:
    pub cpu_to_le32(~0xFF): rx_desc->wb.middle.status_error &=,
    pub NULL: buffer_info->skb =,
// return some buffers to hardware, one at a time is too slow
    if (cleaned_count >= E1000_RX_BUFFER_WRITE) {
    adapter.alloc_rx_buf(rx_ring, cleaned_count,
    pub 0: cleaned_count =,
    }
// use prefetched values
    pub next_rxd: rx_desc =,
    pub next_buffer: buffer_info =,
    pub le32_to_cpu(rx_desc->wb.middle.status_error): staterr =,
    }
    pub i: rx_ring->next_to_clean =,
    pub e1000_desc_unused(rx_ring): cleaned_count =,
    if (cleaned_count)
    pub GFP_ATOMIC): adapter->alloc_rx_buf(rx_ring, cleaned_count,,
    pub total_rx_bytes: adapter->total_rx_bytes +=,
    pub total_rx_packets: adapter->total_rx_packets +=,
    pub cleaned: return,
    }
    static void e1000_consume_page(struct e1000_buffer *bi, struct sk_buff *skb,
    u16 length)
    {
    pub NULL: bi->page =,
    pub length: skb->len +=,
    pub length: skb->data_len +=,
    pub PAGE_SIZE: skb->truesize +=,
    }
//
// e1000_clean_jumbo_rx_irq - Send received data up the network stack; legacy
// @rx_ring: Rx descriptor ring
// @work_done: output parameter for indicating completed work
// @work_to_do: how many packets we can clean
//
// the return value indicates whether actual cleaning was done, there
// is no guarantee that everything was cleaned
//
    static bool e1000_clean_jumbo_rx_irq(struct e1000_ring *rx_ring, int *work_done,
    int work_to_do)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub next_rxd: *mut *mut union e1000_rx_desc_extended rx_desc,,
    pub next_buffer: *mut *mut e1000_buffer buffer_info,,
    pub staterr: u32 length,,
    pub i: c_uint,
    pub 0: int cleaned_count =,
    pub false: bool cleaned =,
    pub 0: unsigned int total_rx_bytes = 0, total_rx_packets =,
    pub shinfo: *mut skb_shared_info,
    pub rx_ring->next_to_clean: i =,
    pub i): *mut *mut rx_desc = E1000_RX_DESC_EXT(rx_ring,,
    pub le32_to_cpu(rx_desc->wb.upper.status_error): staterr =,
    pub &rx_ring->buffer_info[i]: buffer_info =,
    while (staterr & E1000_RXD_STAT_DD) {
    pub skb: *mut sk_buff,
    if (*work_done >= work_to_do)
    pub /: *mut *mut dma_rmb(); / read descriptor and rx_buffer_info after status DD,
    pub buffer_info->skb: skb =,
    pub NULL: buffer_info->skb =,
    if (i == rx_ring.count)
    pub 0: i =,
    pub i): *mut *mut next_rxd = E1000_RX_DESC_EXT(rx_ring,,
    pub &rx_ring->buffer_info[i]: next_buffer =,
    pub true: cleaned =,
    dma_unmap_page(&pdev.dev, buffer_info.dma, PAGE_SIZE,
    pub 0: buffer_info->dma =,
    pub le16_to_cpu(rx_desc->wb.upper.length): length =,
// errors is only valid for DD + EOP descriptors
    if (unlikely((staterr & E1000_RXD_STAT_EOP) &&
    ((staterr & E1000_RXDEXT_ERR_FRAME_ERR_MASK) &&
    !(netdev.features & NETIF_F_RXALL)))) {
// recycle both page and skb
    pub skb: buffer_info->skb =,
// an error means any chain goes out the window too
    if (rx_ring.rx_skb_top)
    pub NULL: rx_ring->rx_skb_top =,
    pub next_desc: goto,
    }

    if (!(staterr & E1000_RXD_STAT_EOP)) {
// this descriptor is only the beginning (or middle)
    if (!rxtop) {
// this is the beginning of a chain
    pub skb: rxtop =,
    skb_fill_page_desc(rxtop, 0, buffer_info.page,
    pub length): 0,,
    } else {
// this is the middle of a chain
    pub skb_shinfo(rxtop): shinfo =,
    skb_fill_page_desc(rxtop, shinfo.nr_frags,
    buffer_info.page, 0,
// re-use the skb, only consumed the page
    pub skb: buffer_info->skb =,
    }
    pub length): e1000_consume_page(buffer_info, rxtop,,
    pub next_desc: goto,
    } else {
    if (rxtop) {
// end of the chain
    pub skb_shinfo(rxtop): shinfo =,
    skb_fill_page_desc(rxtop, shinfo.nr_frags,
    buffer_info.page, 0,
// re-use the current skb, we only consumed the
// page
//
    pub skb: buffer_info->skb =,
    pub rxtop: skb =,
    pub NULL: rxtop =,
    pub length): e1000_consume_page(buffer_info, skb,,
    } else {
// no chain, got EOP, this buf is the packet
// copybreak to save the put_page/alloc_page
//
    if (length <= copybreak &&
    skb_tailroom(skb) >= length) {
    memcpy(skb_tail_pointer(skb),
    page_address(buffer_info.page),
// re-use the page, so don't erase
// buffer_info->page
//
    pub length): skb_put(skb,,
    } else {
    skb_fill_page_desc(skb, 0,
    buffer_info.page, 0,
    e1000_consume_page(buffer_info, skb,
    }
    }
    }
// Receive Checksum Offload
    pub skb): e1000_rx_checksum(adapter, staterr,,
    pub skb): e1000_rx_hash(netdev, rx_desc->wb.lower.hi_dword.rss,,
// probably a little skewed due to removing CRC
    pub skb->len: total_rx_bytes +=,
// eth type trans needs skb->data to point to something
    if (!pskb_may_pull(skb, ETH_HLEN)) {
    pub failed.\n"): e_err("pskb_may_pull,
    pub next_desc: goto,
    }
    e1000_receive_skb(adapter, netdev, skb, staterr,
    next_desc:
    pub cpu_to_le32(~0xFF): rx_desc->wb.upper.status_error &=,
// return some buffers to hardware, one at a time is too slow
    if (unlikely(cleaned_count >= E1000_RX_BUFFER_WRITE)) {
    adapter.alloc_rx_buf(rx_ring, cleaned_count,
    pub 0: cleaned_count =,
    }
// use prefetched values
    pub next_rxd: rx_desc =,
    pub next_buffer: buffer_info =,
    pub le32_to_cpu(rx_desc->wb.upper.status_error): staterr =,
    }
    pub i: rx_ring->next_to_clean =,
    pub e1000_desc_unused(rx_ring): cleaned_count =,
    if (cleaned_count)
    pub GFP_ATOMIC): adapter->alloc_rx_buf(rx_ring, cleaned_count,,
    pub total_rx_bytes: adapter->total_rx_bytes +=,
    pub total_rx_packets: adapter->total_rx_packets +=,
    pub cleaned: return,
    }
//
// e1000_clean_rx_ring - Free Rx Buffers per Queue
// @rx_ring: Rx descriptor ring
//
#[no_mangle]
unsafe extern "C" fn e1000_clean_rx_ring(rx_ring: *mut e1000_ring) {
    static void e1000_clean_rx_ring(struct e1000_ring *rx_ring)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub buffer_info: *mut e1000_buffer,
    pub ps_page: *mut e1000_ps_page,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub j: unsigned int i,,
// Free all the Rx ring sk_buffs
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub &rx_ring->buffer_info[i]: buffer_info =,
    if (buffer_info.dma) {
    if (adapter.clean_rx == e1000_clean_rx_irq)
    dma_unmap_single(&pdev.dev, buffer_info.dma,
    adapter.rx_buffer_len,
#[no_mangle]
pub unsafe extern "C" fn if(e1000_clean_jumbo_rx_irq: adapter->clean_rx ==) -> else {
    else if (adapter.clean_rx == e1000_clean_jumbo_rx_irq)
    dma_unmap_page(&pdev.dev, buffer_info.dma,
    pub DMA_FROM_DEVICE): PAGE_SIZE,,
#[no_mangle]
pub unsafe extern "C" fn if(e1000_clean_rx_irq_ps: adapter->clean_rx ==) -> else {
    else if (adapter.clean_rx == e1000_clean_rx_irq_ps)
    dma_unmap_single(&pdev.dev, buffer_info.dma,
    adapter.rx_ps_bsize0,
    pub 0: buffer_info->dma =,
    }
    if (buffer_info.page) {
    pub NULL: buffer_info->page =,
    }
    if (buffer_info.skb) {
    pub NULL: buffer_info->skb =,
    }
    pub {: for (j = 0; j < PS_PAGE_BUFFERS; j++),
    pub &buffer_info->ps_pages[j]: ps_page =,
    if (!ps_page.page)
    dma_unmap_page(&pdev.dev, ps_page.dma, PAGE_SIZE,
    pub 0: ps_page->dma =,
    pub NULL: ps_page->page =,
    }
    }
// there also may be some cached data from a chained receive
    if (rx_ring.rx_skb_top) {
    pub NULL: rx_ring->rx_skb_top =,
    }
// Zero out the descriptor ring
    pub rx_ring->size): memset(rx_ring->desc, 0,,
    pub 0: rx_ring->next_to_clean =,
    pub 0: rx_ring->next_to_use =,
    pub ~FLAG2_IS_DISCARDING: adapter->flags2 &=,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_downshift_workaround(work: *mut work_struct) {
    static void e1000e_downshift_workaround(struct work_struct *work)
    {
    struct e1000_adapter *adapter = container_of(work,
    struct e1000_adapter,
    if (test_bit(__E1000_DOWN, &adapter.state))
    }
//
// e1000_intr_msi - Interrupt Handler
// @irq: interrupt number
// @data: pointer to a network interface device structure
//
#[no_mangle]
unsafe extern "C" fn e1000_intr_msi(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_intr_msi(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(ICR): u32 icr =,
// read ICR disables interrupts using IAM
    if (icr & E1000_ICR_LSC) {
    pub true: hw->mac.get_link_status =,
// ICH8 workaround-- Call gig speed drop workaround on cable
// disconnect (LSC) before accessing any PHY registers
//
    if ((adapter.flags & FLAG_LSC_GIG_SPEED_DROP) &&
    (!(er32(STATUS) & E1000_STATUS_LU)))
// 80003ES2LAN workaround-- For packet buffer work-around on
// link down event; disable receives here in the ISR and reset
// adapter in watchdog
//
    if (netif_carrier_ok(netdev) &&
    adapter.flags & FLAG_RX_NEEDS_RESTART) {
// disable receives
    pub er32(RCTL): u32 rctl =,
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
    pub FLAG_RESTART_NOW: adapter->flags |=,
    }
// guard against interrupt when we're going down
    if (!test_bit(__E1000_DOWN, &adapter.state))
    pub 1): mod_timer(&adapter->watchdog_timer, jiffies +,
    }
// Reset on uncorrectable ECC error
    if ((icr & E1000_ICR_ECCER) && (hw.mac.type >= e1000_pch_lpt)) {
    pub er32(PBECCSTS): u32 pbeccsts =,
    adapter.corr_errors +=
    pub E1000_PBECCSTS_CORR_ERR_CNT_MASK: pbeccsts &,
    adapter.uncorr_errors +=
    pub pbeccsts): FIELD_GET(E1000_PBECCSTS_UNCORR_ERR_CNT_MASK,,
// Do the reset outside of interrupt context
// return immediately since reset is imminent
    pub IRQ_HANDLED: return,
    }
    if (napi_schedule_prep(&adapter.napi)) {
    pub 0: adapter->total_tx_bytes =,
    pub 0: adapter->total_tx_packets =,
    pub 0: adapter->total_rx_bytes =,
    pub 0: adapter->total_rx_packets =,
    }
    pub IRQ_HANDLED: return,
    }
//
// e1000_intr - Interrupt Handler
// @irq: interrupt number
// @data: pointer to a network interface device structure
//
#[no_mangle]
unsafe extern "C" fn e1000_intr(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_intr(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(ICR): u32 rctl, icr =,
    if (!icr || test_bit(__E1000_DOWN, &adapter.state))
    pub /: *mut *mut return IRQ_NONE; / Not our interrupt,
// IMS will not auto-mask if INT_ASSERTED is not set, and if it is
// not set, then the adapter didn't send an interrupt
//
    if (!(icr & E1000_ICR_INT_ASSERTED))
    pub IRQ_NONE: return,
// Interrupt Auto-Mask...upon reading ICR,
// interrupts are masked.  No need for the
// IMC write
//
    if (icr & E1000_ICR_LSC) {
    pub true: hw->mac.get_link_status =,
// ICH8 workaround-- Call gig speed drop workaround on cable
// disconnect (LSC) before accessing any PHY registers
//
    if ((adapter.flags & FLAG_LSC_GIG_SPEED_DROP) &&
    (!(er32(STATUS) & E1000_STATUS_LU)))
// 80003ES2LAN workaround--
// For packet buffer work-around on link down event;
// disable receives here in the ISR and
// reset adapter in watchdog
//
    if (netif_carrier_ok(netdev) &&
    (adapter.flags & FLAG_RX_NEEDS_RESTART)) {
// disable receives
    pub er32(RCTL): rctl =,
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
    pub FLAG_RESTART_NOW: adapter->flags |=,
    }
// guard against interrupt when we're going down
    if (!test_bit(__E1000_DOWN, &adapter.state))
    pub 1): mod_timer(&adapter->watchdog_timer, jiffies +,
    }
// Reset on uncorrectable ECC error
    if ((icr & E1000_ICR_ECCER) && (hw.mac.type >= e1000_pch_lpt)) {
    pub er32(PBECCSTS): u32 pbeccsts =,
    adapter.corr_errors +=
    pub E1000_PBECCSTS_CORR_ERR_CNT_MASK: pbeccsts &,
    adapter.uncorr_errors +=
    pub pbeccsts): FIELD_GET(E1000_PBECCSTS_UNCORR_ERR_CNT_MASK,,
// Do the reset outside of interrupt context
// return immediately since reset is imminent
    pub IRQ_HANDLED: return,
    }
    if (napi_schedule_prep(&adapter.napi)) {
    pub 0: adapter->total_tx_bytes =,
    pub 0: adapter->total_tx_packets =,
    pub 0: adapter->total_rx_bytes =,
    pub 0: adapter->total_rx_packets =,
    }
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_msix_other(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_msix_other(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(ICR): u32 icr =,
    if (icr & adapter.eiac_mask)
    pub adapter->eiac_mask)): ew32(ICS, (icr &,
    if (icr & E1000_ICR_LSC) {
    pub true: hw->mac.get_link_status =,
// guard against interrupt when we're going down
    if (!test_bit(__E1000_DOWN, &adapter.state))
    pub 1): mod_timer(&adapter->watchdog_timer, jiffies +,
    }
    if (!test_bit(__E1000_DOWN, &adapter.state))
    pub IMS_OTHER_MASK): ew32(IMS, E1000_IMS_OTHER |,
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_intr_msix_tx(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_intr_msix_tx(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub 0: adapter->total_tx_bytes =,
    pub 0: adapter->total_tx_packets =,
    if (!e1000_clean_tx_irq(tx_ring))
// Ring was not completely cleaned, so fire another interrupt
    pub tx_ring->ims_val): ew32(ICS,,
    if (!test_bit(__E1000_DOWN, &adapter.state))
    pub adapter->tx_ring->ims_val): ew32(IMS,,
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_intr_msix_rx(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_intr_msix_rx(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub adapter->rx_ring: *mut *mut e1000_ring rx_ring =,
// Write the ITR value calculated at the end of the
// previous interrupt.
//
    if (rx_ring.set_itr) {
    u32 itr = rx_ring.itr_val ?
    pub 0: *mut *mut 1000000000 / (rx_ring->itr_val  256) :,
    pub rx_ring->itr_register): writel(itr,,
    pub 0: rx_ring->set_itr =,
    }
    if (napi_schedule_prep(&adapter.napi)) {
    pub 0: adapter->total_rx_bytes =,
    pub 0: adapter->total_rx_packets =,
    }
    pub IRQ_HANDLED: return,
    }
//
// e1000_configure_msix - Configure MSI-X hardware
// @adapter: board private structure
//
// e1000_configure_msix sets up the hardware to properly
// generate MSI-X interrupts.
//
#[no_mangle]
unsafe extern "C" fn e1000_configure_msix(adapter: *mut e1000_adapter) {
    static void e1000_configure_msix(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->rx_ring: *mut *mut e1000_ring rx_ring =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub 0: int vector =,
    pub 0: u32 ctrl_ext, ivar =,
    pub 0: adapter->eiac_mask =,
// Workaround issue with spurious interrupts on 82574 in MSI-X mode
    if (hw.mac.type == e1000_82574) {
    pub er32(RFCTL): u32 rfctl =,
    pub E1000_RFCTL_ACK_DIS: rfctl |=,
    pub rfctl): ew32(RFCTL,,
    }
// Configure Rx vector
    pub E1000_IMS_RXQ0: rx_ring->ims_val =,
    pub rx_ring->ims_val: adapter->eiac_mask |=,
    if (rx_ring.itr_val)
    writel(1000000000 / (rx_ring.itr_val * 256),
    else
    pub rx_ring->itr_register): writel(1,,
    pub vector: ivar = E1000_IVAR_INT_ALLOC_VALID |,
// Configure Tx vector
    pub E1000_IMS_TXQ0: tx_ring->ims_val =,
    if (tx_ring.itr_val)
    writel(1000000000 / (tx_ring.itr_val * 256),
    else
    pub tx_ring->itr_register): writel(1,,
    pub tx_ring->ims_val: adapter->eiac_mask |=,
    pub 8): ivar |= ((E1000_IVAR_INT_ALLOC_VALID | vector) <<,
// set vector for Other Causes, e.g. link changes
    pub 16): ivar |= ((E1000_IVAR_INT_ALLOC_VALID | vector) <<,
    if (rx_ring.itr_val)
    writel(1000000000 / (rx_ring.itr_val * 256),
    pub E1000_EITR_82574(vector)): hw->hw_addr +,
    else
    pub E1000_EITR_82574(vector)): writel(1, hw->hw_addr +,
// Cause Tx interrupts on every write back
    pub BIT(31): ivar |=,
    pub ivar): ew32(IVAR,,
// enable MSI-X PBA support
    pub ~E1000_CTRL_EXT_IAME: ctrl_ext = er32(CTRL_EXT) &,
    pub E1000_CTRL_EXT_EIAME: ctrl_ext |= E1000_CTRL_EXT_PBA_CLR |,
    pub ctrl_ext): ew32(CTRL_EXT,,
    }
#[no_mangle]
pub unsafe extern "C" fn e1000e_reset_interrupt_capability(adapter: *mut e1000_adapter) {
    void e1000e_reset_interrupt_capability(struct e1000_adapter *adapter)
    {
    if (adapter.msix_entries) {
    pub NULL: adapter->msix_entries =,
    } else if (adapter.flags & FLAG_MSI_ENABLED) {
    pub ~FLAG_MSI_ENABLED: adapter->flags &=,
    }
    }
//
// e1000e_set_interrupt_capability - set MSI or MSI-X if supported
// @adapter: board private structure
//
// Attempt to configure interrupts using the best available
// capabilities of the hardware and kernel.
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_set_interrupt_capability(adapter: *mut e1000_adapter) {
    void e1000e_set_interrupt_capability(struct e1000_adapter *adapter)
    {
    pub err: c_int,
    pub i: c_int,
    switch (adapter.int_mode) {
    case E1000E_INT_MODE_MSIX:
    if (adapter.flags & FLAG_HAS_MSIX) {
    pub /: *mut *mut adapter->num_vectors = 3; / RxQ0, TxQ0 and other,
    adapter.msix_entries = kzalloc_objs(struct msix_entry,
    if (adapter.msix_entries) {
    pub adapter: *mut *mut e1000_adapter a =,
    pub i++): for (i = 0; i < adapter->num_vectors;,
    pub i: adapter->msix_entries[i].entry =,
    err = pci_enable_msix_range(a.pdev,
    a.msix_entries,
    a.num_vectors,
    if (err > 0)
    }
// MSI-X failed, so fall through and try MSI
    pub interrupts.\n"): e_err("Failed to initialize MSI-X interrupts. Falling back to MSI,
    }
    pub E1000E_INT_MODE_MSI: adapter->int_mode =,
    case E1000E_INT_MODE_MSI:
    if (!pci_enable_msi(adapter.pdev)) {
    pub FLAG_MSI_ENABLED: adapter->flags |=,
    } else {
    pub E1000E_INT_MODE_LEGACY: adapter->int_mode =,
    pub interrupts.\n"): e_err("Failed to initialize MSI interrupts. Falling back to legacy,
    }
    case E1000E_INT_MODE_LEGACY:
// Don't do anything; this is the system default
    }
// store the number of vectors being used
    pub 1: adapter->num_vectors =,
    }
//
// e1000_request_msix - Initialize MSI-X interrupts
// @adapter: board private structure
//
// e1000_request_msix allocates MSI-X vectors and requests interrupts from the
// kernel.
//
#[no_mangle]
unsafe extern "C" fn e1000_request_msix(adapter: *mut e1000_adapter) -> c_int {
    static int e1000_request_msix(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub 0: int err = 0, vector =,
    if (strlen(netdev.name) < (IFNAMSIZ - 5))
    snprintf(adapter.rx_ring.name,
    sizeof(adapter.rx_ring.name) - 1,
    pub netdev->name): "%.14s-rx-0",,
    else
    pub IFNAMSIZ): memcpy(adapter->rx_ring->name, netdev->name,,
    err = request_irq(adapter.msix_entries[vector].vector,
    e1000_intr_msix_rx, 0, adapter.rx_ring.name,
    if (err)
    pub err: return,
    adapter.rx_ring.itr_register = adapter.hw.hw_addr +
    pub adapter->itr: adapter->rx_ring->itr_val =,
    if (strlen(netdev.name) < (IFNAMSIZ - 5))
    snprintf(adapter.tx_ring.name,
    sizeof(adapter.tx_ring.name) - 1,
    pub netdev->name): "%.14s-tx-0",,
    else
    pub IFNAMSIZ): memcpy(adapter->tx_ring->name, netdev->name,,
    err = request_irq(adapter.msix_entries[vector].vector,
    e1000_intr_msix_tx, 0, adapter.tx_ring.name,
    if (err)
    pub err: return,
    adapter.tx_ring.itr_register = adapter.hw.hw_addr +
    pub adapter->itr: adapter->tx_ring->itr_val =,
    err = request_irq(adapter.msix_entries[vector].vector,
    pub netdev): e1000_msix_other, 0, netdev->name,,
    if (err)
    pub err: return,
    pub 0: return,
    }
//
// e1000_request_irq - initialize interrupts
// @adapter: board private structure
//
// Attempts to configure interrupts using the best available
// capabilities of the hardware and kernel.
//
#[no_mangle]
unsafe extern "C" fn e1000_request_irq(adapter: *mut e1000_adapter) -> c_int {
    static int e1000_request_irq(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub err: c_int,
    if (adapter.msix_entries) {
    pub e1000_request_msix(adapter): err =,
    if (!err)
    pub err: return,
// fall back to MSI
    pub E1000E_INT_MODE_MSI: adapter->int_mode =,
    }
    if (adapter.flags & FLAG_MSI_ENABLED) {
    err = request_irq(adapter.pdev.irq, e1000_intr_msi, 0,
    pub netdev): netdev->name,,
    if (!err)
    pub err: return,
// fall back to legacy interrupt
    pub E1000E_INT_MODE_LEGACY: adapter->int_mode =,
    }
    err = request_irq(adapter.pdev.irq, e1000_intr, IRQF_SHARED,
    pub netdev): netdev->name,,
    if (err)
    pub err): e_err("Unable to allocate interrupt, Error: %d\n",,
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_free_irq(adapter: *mut e1000_adapter) {
    static void e1000_free_irq(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    if (adapter.msix_entries) {
    pub 0: int vector =,
    pub netdev): free_irq(adapter->msix_entries[vector].vector,,
    pub netdev): free_irq(adapter->msix_entries[vector].vector,,
// Other Causes interrupt vector
    pub netdev): free_irq(adapter->msix_entries[vector].vector,,
    }
    pub netdev): free_irq(adapter->pdev->irq,,
    }
//
// e1000_irq_disable - Mask off interrupt generation on the NIC
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn e1000_irq_disable(adapter: *mut e1000_adapter) {
    static void e1000_irq_disable(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ~0): ew32(IMC,,
    if (adapter.msix_entries)
    pub 0): ew32(EIAC_82574,,
    if (adapter.msix_entries) {
    pub i: c_int,
    pub i++): for (i = 0; i < adapter->num_vectors;,
    } else {
    }
    }
//
// e1000_irq_enable - Enable default interrupt generation settings
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn e1000_irq_enable(adapter: *mut e1000_adapter) {
    static void e1000_irq_enable(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    if (adapter.msix_entries) {
    pub E1000_EIAC_MASK_82574): ew32(EIAC_82574, adapter->eiac_mask &,
    ew32(IMS, adapter.eiac_mask | E1000_IMS_OTHER |
    } else if (hw.mac.type >= e1000_pch_lpt) {
    pub E1000_IMS_ECCER): ew32(IMS, IMS_ENABLE_MASK |,
    } else {
    pub IMS_ENABLE_MASK): ew32(IMS,,
    }
    }
//
// e1000e_get_hw_control - get control of the h/w from f/w
// @adapter: address of board private structure
//
// e1000e_get_hw_control sets {CTRL_EXT|SWSM}:DRV_LOAD bit.
// For ASF and Pass Through versions of f/w this means that
// the driver is loaded. For AMT version (only with 82573)
// of the f/w this means that the network i/f is open.
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_get_hw_control(adapter: *mut e1000_adapter) {
    void e1000e_get_hw_control(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ctrl_ext: u32,
    pub swsm: u32,
// Let firmware know the driver has taken over
    if (adapter.flags & FLAG_HAS_SWSM_ON_LOAD) {
    pub er32(SWSM): swsm =,
    pub E1000_SWSM_DRV_LOAD): ew32(SWSM, swsm |,
    } else if (adapter.flags & FLAG_HAS_CTRLEXT_ON_LOAD) {
    pub er32(CTRL_EXT): ctrl_ext =,
    pub E1000_CTRL_EXT_DRV_LOAD): ew32(CTRL_EXT, ctrl_ext |,
    }
    }
//
// e1000e_release_hw_control - release control of the h/w to f/w
// @adapter: address of board private structure
//
// e1000e_release_hw_control resets {CTRL_EXT|SWSM}:DRV_LOAD bit.
// For ASF and Pass Through versions of f/w this means that the
// driver is no longer loaded. For AMT version (only with 82573) i
// of the f/w this means that the network i/f is closed.
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_release_hw_control(adapter: *mut e1000_adapter) {
    void e1000e_release_hw_control(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ctrl_ext: u32,
    pub swsm: u32,
// Let firmware taken over control of h/w
    if (adapter.flags & FLAG_HAS_SWSM_ON_LOAD) {
    pub er32(SWSM): swsm =,
    pub ~E1000_SWSM_DRV_LOAD): ew32(SWSM, swsm &,
    } else if (adapter.flags & FLAG_HAS_CTRLEXT_ON_LOAD) {
    pub er32(CTRL_EXT): ctrl_ext =,
    pub ~E1000_CTRL_EXT_DRV_LOAD): ew32(CTRL_EXT, ctrl_ext &,
    }
    }
//
// e1000_alloc_ring_dma - allocate memory for a ring structure
// @adapter: board private structure
// @ring: ring struct for which to allocate dma
//
    static int e1000_alloc_ring_dma(struct e1000_adapter *adapter,
    struct e1000_ring *ring)
    {
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    ring.desc = dma_alloc_coherent(&pdev.dev, ring.size, &ring.dma,
    if (!ring.desc)
    pub -ENOMEM: return,
    pub 0: return,
    }
//
// e1000e_setup_tx_resources - allocate Tx resources (Descriptors)
// @tx_ring: Tx descriptor ring
//
// Return 0 on success, negative on failure
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_setup_tx_resources(tx_ring: *mut e1000_ring) -> c_int {
    int e1000e_setup_tx_resources(struct e1000_ring *tx_ring)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub size: int err = -ENOMEM,,
    pub tx_ring->count: *mut *mut size = sizeof(struct e1000_buffer),
    pub vzalloc(size): tx_ring->buffer_info =,
    if (!tx_ring.buffer_info)
    pub err: goto,
// round up to nearest 4K
    pub e1000_tx_desc): *mut *mut tx_ring->size = tx_ring->count  sizeof(struct,
    pub 4096): tx_ring->size = ALIGN(tx_ring->size,,
    pub tx_ring): err = e1000_alloc_ring_dma(adapter,,
    if (err)
    pub err: goto,
    pub 0: tx_ring->next_to_use =,
    pub 0: tx_ring->next_to_clean =,
    pub 0: return,
    err:
    pub ring\n"): e_err("Unable to allocate memory for the transmit descriptor,
    pub err: return,
    }
//
// e1000e_setup_rx_resources - allocate Rx resources (Descriptors)
// @rx_ring: Rx descriptor ring
//
// Returns 0 on success, negative on failure
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_setup_rx_resources(rx_ring: *mut e1000_ring) -> c_int {
    int e1000e_setup_rx_resources(struct e1000_ring *rx_ring)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub buffer_info: *mut e1000_buffer,
    pub -ENOMEM: int i, size, desc_len, err =,
    pub rx_ring->count: *mut *mut size = sizeof(struct e1000_buffer),
    pub vzalloc(size): rx_ring->buffer_info =,
    if (!rx_ring.buffer_info)
    pub err: goto,
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub &rx_ring->buffer_info[i]: buffer_info =,
    buffer_info.ps_pages = kzalloc_objs(struct e1000_ps_page,
    if (!buffer_info.ps_pages)
    pub err_pages: goto,
    }
    pub e1000_rx_desc_packet_split): desc_len = sizeof(union,
// Round up to nearest 4K
    pub desc_len: *mut *mut rx_ring->size = rx_ring->count,
    pub 4096): rx_ring->size = ALIGN(rx_ring->size,,
    pub rx_ring): err = e1000_alloc_ring_dma(adapter,,
    if (err)
    pub err_pages: goto,
    pub 0: rx_ring->next_to_clean =,
    pub 0: rx_ring->next_to_use =,
    pub NULL: rx_ring->rx_skb_top =,
    pub 0: return,
    err_pages:
    pub {: for (i = 0; i < rx_ring->count; i++),
    pub &rx_ring->buffer_info[i]: buffer_info =,
    }
    err:
    pub ring\n"): e_err("Unable to allocate memory for the receive descriptor,
    pub err: return,
    }
//
// e1000_clean_tx_ring - Free Tx Buffers
// @tx_ring: Tx descriptor ring
//
#[no_mangle]
unsafe extern "C" fn e1000_clean_tx_ring(tx_ring: *mut e1000_ring) {
    static void e1000_clean_tx_ring(struct e1000_ring *tx_ring)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub buffer_info: *mut e1000_buffer,
    pub size: c_ulong,
    pub i: c_uint,
    pub {: for (i = 0; i < tx_ring->count; i++),
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub false): e1000_put_txbuf(tx_ring, buffer_info,,
    }
    pub tx_ring->count: *mut *mut size = sizeof(struct e1000_buffer),
    pub size): memset(tx_ring->buffer_info, 0,,
    pub tx_ring->size): memset(tx_ring->desc, 0,,
    pub 0: tx_ring->next_to_use =,
    pub 0: tx_ring->next_to_clean =,
    }
//
// e1000e_free_tx_resources - Free Tx Resources per Queue
// @tx_ring: Tx descriptor ring
//
// Free all transmit software resources
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_free_tx_resources(tx_ring: *mut e1000_ring) {
    void e1000e_free_tx_resources(struct e1000_ring *tx_ring)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub NULL: tx_ring->buffer_info =,
    dma_free_coherent(&pdev.dev, tx_ring.size, tx_ring.desc,
    pub NULL: tx_ring->desc =,
    }
//
// e1000e_free_rx_resources - Free Rx Resources
// @rx_ring: Rx descriptor ring
//
// Free all receive software resources
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_free_rx_resources(rx_ring: *mut e1000_ring) {
    void e1000e_free_rx_resources(struct e1000_ring *rx_ring)
    {
    pub rx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub i: c_int,
    pub i++): for (i = 0; i < rx_ring->count;,
    pub NULL: rx_ring->buffer_info =,
    dma_free_coherent(&pdev.dev, rx_ring.size, rx_ring.desc,
    pub NULL: rx_ring->desc =,
    }
//
// e1000_update_itr - update the dynamic ITR value based on statistics
// @itr_setting: current adapter->itr
// @packets: the number of packets during this measurement interval
// @bytes: the number of bytes during this measurement interval
//
// Stores a new ITR value based on packets and byte
// counts during the last interrupt.  The advantage of per interrupt
// computation is faster updates and more accurate ITR for the current
// traffic pattern.  Constants in this function were computed
// based on theoretical maximum wire speed and thresholds were set based
// on testing data as well as attempting to minimize response time
// while increasing bulk throughput.  This functionality is controlled
// by the InterruptThrottleRate module parameter.
//
#[no_mangle]
unsafe extern "C" fn e1000_update_itr(itr_setting: u16, packets: c_int, bytes: c_int) -> c_uint {
    static unsigned int e1000_update_itr(u16 itr_setting, int packets, int bytes)
    {
    pub itr_setting: unsigned int retval =,
    if (packets == 0)
    pub itr_setting: return,
    switch (itr_setting) {
    case lowest_latency:
// handle TSO and jumbo frames
    if (bytes / packets > 8000)
    pub bulk_latency: retval =,
#[no_mangle]
pub unsafe extern "C" fn if(512): (packets < 5) && (bytes >) -> else {
    else if ((packets < 5) && (bytes > 512))
    pub low_latency: retval =,
    case low_latency:	/* 50 usec aka 20000 ints/s */
    if (bytes > 10000) {
// this if handles the TSO accounting
    if (bytes / packets > 8000)
    pub bulk_latency: retval =,
#[no_mangle]
pub unsafe extern "C" fn if(1200): (packets < 10) || ((bytes / packets) >) -> else {
    else if ((packets < 10) || ((bytes / packets) > 1200))
    pub bulk_latency: retval =,
#[no_mangle]
pub unsafe extern "C" fn if(35): (packets >) -> else {
    else if ((packets > 35))
    pub lowest_latency: retval =,
    } else if (bytes / packets > 2000) {
    pub bulk_latency: retval =,
    } else if (packets <= 2 && bytes < 512) {
    pub lowest_latency: retval =,
    }
    case bulk_latency:	/* 250 usec aka 4000 ints/s */
    if (bytes > 25000) {
    if (packets > 35)
    pub low_latency: retval =,
    } else if (bytes < 6000) {
    pub low_latency: retval =,
    }
    }
    pub retval: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_set_itr(adapter: *mut e1000_adapter) {
    static void e1000_set_itr(struct e1000_adapter *adapter)
    {
    pub current_itr: u16,
    pub adapter->itr: u32 new_itr =,
// for non-gigabit speeds, just fix the interrupt rate at 4000
    if (adapter.link_speed != SPEED_1000) {
    pub 4000: new_itr =,
    pub set_itr_now: goto,
    }
    if (adapter.flags2 & FLAG2_DISABLE_AIM) {
    pub 0: new_itr =,
    pub set_itr_now: goto,
    }
    adapter.tx_itr = e1000_update_itr(adapter.tx_itr,
    adapter.total_tx_packets,
// conservative mode (itr 3) eliminates the lowest_latency setting
    if (adapter.itr_setting == 3 && adapter.tx_itr == lowest_latency)
    pub low_latency: adapter->tx_itr =,
    adapter.rx_itr = e1000_update_itr(adapter.rx_itr,
    adapter.total_rx_packets,
// conservative mode (itr 3) eliminates the lowest_latency setting
    if (adapter.itr_setting == 3 && adapter.rx_itr == lowest_latency)
    pub low_latency: adapter->rx_itr =,
    pub adapter->tx_itr): current_itr = max(adapter->rx_itr,,
// counts and packets in update_itr are dependent on these numbers
    switch (current_itr) {
    case lowest_latency:
    pub 70000: new_itr =,
    case low_latency:
    pub /: *mut *mut new_itr = 20000; / aka hwitr = ~200,
    case bulk_latency:
    pub 4000: new_itr =,
    default:
    }
    set_itr_now:
    if (new_itr != adapter.itr) {
// this attempts to bias the interrupt rate towards Bulk
// by adding intermediate steps when interrupt rate is
// increasing
//
    new_itr = new_itr > adapter.itr ?
    pub new_itr: min(adapter->itr + (new_itr >> 2), new_itr) :,
    pub new_itr: adapter->itr =,
    pub new_itr: adapter->rx_ring->itr_val =,
    if (adapter.msix_entries)
    pub 1: adapter->rx_ring->set_itr =,
    else
    pub new_itr): e1000e_write_itr(adapter,,
    }
    }
//
// e1000e_write_itr - write the ITR value to the appropriate registers
// @adapter: address of board private structure
// @itr: new ITR value to program
//
// e1000e_write_itr determines if the adapter is in MSI-X mode
// and, if so, writes the EITR registers with the ITR value.
// Otherwise, it writes the ITR value into the ITR register.
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_write_itr(adapter: *mut e1000_adapter, itr: u32) {
    void e1000e_write_itr(struct e1000_adapter *adapter, u32 itr)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub 0: *mut *mut u32 new_itr = itr ? 1000000000 / (itr  256) :,
    if (adapter.msix_entries) {
    pub vector: c_int,
    pub vector++): for (vector = 0; vector < adapter->num_vectors;,
    pub E1000_EITR_82574(vector)): writel(new_itr, hw->hw_addr +,
    } else {
    pub new_itr): ew32(ITR,,
    }
    }
//
// e1000_alloc_queues - Allocate memory for all rings
// @adapter: board private structure to initialize
//
#[no_mangle]
unsafe extern "C" fn e1000_alloc_queues(adapter: *mut e1000_adapter) -> c_int {
    static int e1000_alloc_queues(struct e1000_adapter *adapter)
    {
    pub e1000_ring): int size = sizeof(struct,
    pub GFP_KERNEL): adapter->tx_ring = kzalloc(size,,
    if (!adapter.tx_ring)
    pub err: goto,
    pub adapter->tx_ring_count: adapter->tx_ring->count =,
    pub adapter: adapter->tx_ring->adapter =,
    pub GFP_KERNEL): adapter->rx_ring = kzalloc(size,,
    if (!adapter.rx_ring)
    pub err: goto,
    pub adapter->rx_ring_count: adapter->rx_ring->count =,
    pub adapter: adapter->rx_ring->adapter =,
    pub 0: return,
    err:
    pub queues\n"): e_err("Unable to allocate memory for,
    pub -ENOMEM: return,
    }
//
// e1000e_poll - NAPI Rx polling callback
// @napi: struct associated with this polling callback
// @budget: number of packets driver is allowed to process this poll
//
#[no_mangle]
unsafe extern "C" fn e1000e_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int e1000e_poll(struct napi_struct *napi, int budget)
    {
    struct e1000_adapter *adapter = container_of(napi, struct e1000_adapter,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->netdev: *mut *mut net_device poll_dev =,
    pub 0: int tx_cleaned = 1, work_done =,
    pub netdev_priv(poll_dev): adapter =,
    if (!adapter.msix_entries ||
    (adapter.rx_ring.ims_val & adapter.tx_ring.ims_val))
    pub e1000_clean_tx_irq(adapter->tx_ring): tx_cleaned =,
    pub budget): adapter->clean_rx(adapter->rx_ring, &work_done,,
    if (!tx_cleaned || work_done == budget)
    pub budget: return,
// Exit the polling mode, but don't re-enable interrupts if stack might
// poll us due to busy-polling
//
    if (likely(napi_complete_done(napi, work_done))) {
    if (adapter.itr_setting & 3)
    if (!test_bit(__E1000_DOWN, &adapter.state)) {
    if (adapter.msix_entries)
    pub adapter->rx_ring->ims_val): ew32(IMS,,
    else
    }
    }
    pub work_done: return,
    }
    static int e1000_vlan_rx_add_vid(struct net_device *netdev,
    __always_unused __be16 proto, u16 vid)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub index: u32 vfta,,
// don't update vlan cookie if already programmed
    if ((adapter.hw.mng_cookie.status &
    E1000_MNG_DHCP_COOKIE_STATUS_VLAN) &&
    (vid == adapter.mng_vlan_id))
    pub 0: return,
// add VID to filter table
    if (adapter.flags & FLAG_HAS_HW_VLAN_FILTER) {
    pub 0x7F: index = (vid >> 5) &,
    pub index): vfta = E1000_READ_REG_ARRAY(hw, E1000_VFTA,,
    pub 0x1F)): vfta |= BIT((vid &,
    pub vfta): hw->mac.ops.write_vfta(hw, index,,
    }
    pub adapter->active_vlans): set_bit(vid,,
    pub 0: return,
    }
    static int e1000_vlan_rx_kill_vid(struct net_device *netdev,
    __always_unused __be16 proto, u16 vid)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub index: u32 vfta,,
    if ((adapter.hw.mng_cookie.status &
    E1000_MNG_DHCP_COOKIE_STATUS_VLAN) &&
    (vid == adapter.mng_vlan_id)) {
// release control to f/w
    pub 0: return,
    }
// remove VID from filter table
    if (adapter.flags & FLAG_HAS_HW_VLAN_FILTER) {
    pub 0x7F: index = (vid >> 5) &,
    pub index): vfta = E1000_READ_REG_ARRAY(hw, E1000_VFTA,,
    pub 0x1F)): vfta &= ~BIT((vid &,
    pub vfta): hw->mac.ops.write_vfta(hw, index,,
    }
    pub adapter->active_vlans): clear_bit(vid,,
    pub 0: return,
    }
//
// e1000e_vlan_filter_disable - helper to disable hw VLAN filtering
// @adapter: board private structure to initialize
//
#[no_mangle]
unsafe extern "C" fn e1000e_vlan_filter_disable(adapter: *mut e1000_adapter) {
    static void e1000e_vlan_filter_disable(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rctl: u32,
    if (adapter.flags & FLAG_HAS_HW_VLAN_FILTER) {
// disable VLAN receive filtering
    pub er32(RCTL): rctl =,
    pub E1000_RCTL_CFIEN): rctl &= ~(E1000_RCTL_VFE |,
    pub rctl): ew32(RCTL,,
    if (adapter.mng_vlan_id != E1000_MNG_VLAN_NONE) {
    e1000_vlan_rx_kill_vid(netdev, htons(ETH_P_8021Q),
    pub E1000_MNG_VLAN_NONE: adapter->mng_vlan_id =,
    }
    }
    }
//
// e1000e_vlan_filter_enable - helper to enable HW VLAN filtering
// @adapter: board private structure to initialize
//
#[no_mangle]
unsafe extern "C" fn e1000e_vlan_filter_enable(adapter: *mut e1000_adapter) {
    static void e1000e_vlan_filter_enable(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rctl: u32,
    if (adapter.flags & FLAG_HAS_HW_VLAN_FILTER) {
// enable VLAN receive filtering
    pub er32(RCTL): rctl =,
    pub E1000_RCTL_VFE: rctl |=,
    pub ~E1000_RCTL_CFIEN: rctl &=,
    pub rctl): ew32(RCTL,,
    }
    }
//
// e1000e_vlan_strip_disable - helper to disable HW VLAN stripping
// @adapter: board private structure to initialize
//
#[no_mangle]
unsafe extern "C" fn e1000e_vlan_strip_disable(adapter: *mut e1000_adapter) {
    static void e1000e_vlan_strip_disable(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ctrl: u32,
// disable VLAN tag insert/strip
    pub er32(CTRL): ctrl =,
    pub ~E1000_CTRL_VME: ctrl &=,
    pub ctrl): ew32(CTRL,,
    }
//
// e1000e_vlan_strip_enable - helper to enable HW VLAN stripping
// @adapter: board private structure to initialize
//
#[no_mangle]
unsafe extern "C" fn e1000e_vlan_strip_enable(adapter: *mut e1000_adapter) {
    static void e1000e_vlan_strip_enable(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ctrl: u32,
// enable VLAN tag insert/strip
    pub er32(CTRL): ctrl =,
    pub E1000_CTRL_VME: ctrl |=,
    pub ctrl): ew32(CTRL,,
    }
#[no_mangle]
unsafe extern "C" fn e1000_update_mng_vlan(adapter: *mut e1000_adapter) {
    static void e1000_update_mng_vlan(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub adapter->hw.mng_cookie.vlan_id: u16 vid =,
    pub adapter->mng_vlan_id: u16 old_vid =,
    if (adapter.hw.mng_cookie.status & E1000_MNG_DHCP_COOKIE_STATUS_VLAN) {
    pub vid): e1000_vlan_rx_add_vid(netdev, htons(ETH_P_8021Q),,
    pub vid: adapter->mng_vlan_id =,
    }
    if (old_vid != E1000_MNG_VLAN_NONE && vid != old_vid)
    pub old_vid): e1000_vlan_rx_kill_vid(netdev, htons(ETH_P_8021Q),,
    }
#[no_mangle]
unsafe extern "C" fn e1000_restore_vlan(adapter: *mut e1000_adapter) {
    static void e1000_restore_vlan(struct e1000_adapter *adapter)
    {
    pub vid: u16,
    pub 0): e1000_vlan_rx_add_vid(adapter->netdev, htons(ETH_P_8021Q),,
    for_each_set_bit(vid, adapter.active_vlans, VLAN_N_VID)
    pub vid): e1000_vlan_rx_add_vid(adapter->netdev, htons(ETH_P_8021Q),,
    }
#[no_mangle]
unsafe extern "C" fn e1000_init_manageability_pt(adapter: *mut e1000_adapter) {
    static void e1000_init_manageability_pt(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub j: u32 manc, manc2h, mdef, i,,
    if (!(adapter.flags & FLAG_MNG_PT_ENABLED))
    pub er32(MANC): manc =,
// enable receiving management packets to the host. this will probably
// generate destination unreachable messages from the host OS, but
// the packets will be handled on SMBUS
//
    pub E1000_MANC_EN_MNG2HOST: manc |=,
    pub er32(MANC2H): manc2h =,
    switch (hw.mac.type) {
    default:
    pub E1000_MANC2H_PORT_664): manc2h |= (E1000_MANC2H_PORT_623 |,
    case e1000_82574:
    case e1000_82583:
// Check if IPMI pass-through decision filter already exists;
// if so, enable it.
//
    pub {: for (i = 0, j = 0; i < 8; i++),
    pub er32(MDEF(i)): mdef =,
// Ignore filters with anything other than IPMI ports
    if (mdef & ~(E1000_MDEF_PORT_623 | E1000_MDEF_PORT_664))
// Enable this decision filter in MANC2H
    if (mdef)
    pub BIT(i): manc2h |=,
    pub mdef: j |=,
    }
    if (j == (E1000_MDEF_PORT_623 | E1000_MDEF_PORT_664))
// Create new decision filter in an empty filter
    pub i++): for (i = 0, j = 0; i < 8;,
    if (er32(MDEF(i)) == 0) {
    ew32(MDEF(i), (E1000_MDEF_PORT_623 |
    pub BIT(1): manc2h |=,
    }
    if (!j)
    pub filter\n"): e_warn("Unable to create IPMI pass-through,
    }
    pub manc2h): ew32(MANC2H,,
    pub manc): ew32(MANC,,
    }
//
// e1000_configure_tx - Configure Transmit Unit after Reset
// @adapter: board private structure
//
// Configure the Tx unit of the MAC after a reset.
//
#[no_mangle]
unsafe extern "C" fn e1000_configure_tx(adapter: *mut e1000_adapter) {
    static void e1000_configure_tx(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub tdba: u64,
    pub tarc: u32 tdlen, tctl,,
// Setup the HW Tx Head and Tail descriptor pointers
    pub tx_ring->dma: tdba =,
    pub e1000_tx_desc): *mut *mut tdlen = tx_ring->count  sizeof(struct,
    pub DMA_BIT_MASK(32))): ew32(TDBAL(0), (tdba &,
    pub 32)): ew32(TDBAH(0), (tdba >>,
    pub tdlen): ew32(TDLEN(0),,
    pub 0): ew32(TDH(0),,
    pub 0): ew32(TDT(0),,
    pub E1000_TDH(0): tx_ring->head = adapter->hw.hw_addr +,
    pub E1000_TDT(0): tx_ring->tail = adapter->hw.hw_addr +,
    if (adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    pub 0): e1000e_update_tdt_wa(tx_ring,,
// Set the Tx Interrupt Delay register
    pub adapter->tx_int_delay): ew32(TIDV,,
// Tx irq moderation
    pub adapter->tx_abs_int_delay): ew32(TADV,,
    if (adapter.flags2 & FLAG2_DMA_BURST) {
    pub er32(TXDCTL(0)): u32 txdctl =,
    txdctl &= ~(E1000_TXDCTL_PTHRESH | E1000_TXDCTL_HTHRESH |
// set up some performance related parameters to encourage the
// hardware to use the bus more efficiently in bursts, depends
// on the tx_int_delay to be enabled,
// wthresh = 1 ==> burst write is disabled to avoid Tx stalls
// hthresh = 1 ==> prefetch when one or more available
// pthresh = 0x1f ==> prefetch if internal cache 31 or less
// BEWARE: this seems to work but should be considered first if
// there are Tx hangs or other Tx related bugs
//
    pub E1000_TXDCTL_DMA_BURST_ENABLE: txdctl |=,
    pub txdctl): ew32(TXDCTL(0),,
    }
// erratum work around: set txdctl the same for both queues
    pub er32(TXDCTL(0))): ew32(TXDCTL(1),,
// Program the Transmit Control Register
    pub er32(TCTL): tctl =,
    pub ~E1000_TCTL_CT: tctl &=,
    tctl |= E1000_TCTL_PSP | E1000_TCTL_RTLC |
    pub E1000_CT_SHIFT): (E1000_COLLISION_THRESHOLD <<,
    if (adapter.flags & FLAG_TARC_SPEED_MODE_BIT) {
    pub er32(TARC(0)): tarc =,
// set the speed mode bit, we'll clear it if we're not at
// gigabit link later
//

    pub SPEED_MODE_BIT: tarc |=,
    pub tarc): ew32(TARC(0),,
    }
// errata: program both queues to unweighted RR
    if (adapter.flags & FLAG_TARC_SET_BIT_ZERO) {
    pub er32(TARC(0)): tarc =,
    pub 1: tarc |=,
    pub tarc): ew32(TARC(0),,
    pub er32(TARC(1)): tarc =,
    pub 1: tarc |=,
    pub tarc): ew32(TARC(1),,
    }
// Setup Transmit Descriptor Settings for eop descriptor
    pub E1000_TXD_CMD_IFCS: adapter->txd_cmd = E1000_TXD_CMD_EOP |,
// only set IDE if we are delaying interrupts using the timers
    if (adapter.tx_int_delay)
    pub E1000_TXD_CMD_IDE: adapter->txd_cmd |=,
// enable Report Status bit
    pub E1000_TXD_CMD_RS: adapter->txd_cmd |=,
    pub tctl): ew32(TCTL,,
// SPT and KBL Si errata workaround to avoid data corruption
    if (hw.mac.type == e1000_pch_spt) {
    pub reg_val: u32,
    pub er32(IOSFPC): reg_val =,
    pub E1000_RCTL_RDMTS_HEX: reg_val |=,
    pub reg_val): ew32(IOSFPC,,
    pub er32(TARC(0)): reg_val =,
// SPT and KBL Si errata workaround to avoid Tx hang.
// Dropping the number of outstanding requests from
// 3 to 2 in order to avoid a buffer overrun.
//
    pub ~E1000_TARC0_CB_MULTIQ_3_REQ: reg_val &=,
    pub E1000_TARC0_CB_MULTIQ_2_REQ: reg_val |=,
    pub reg_val): ew32(TARC(0),,
    }
    }

    (((S) & (PAGE_SIZE - 1)) ? 1 : 0))
//
// e1000_setup_rctl - configure the receive control registers
// @adapter: Board private structure
//
#[no_mangle]
unsafe extern "C" fn e1000_setup_rctl(adapter: *mut e1000_adapter) {
    static void e1000_setup_rctl(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rfctl: u32 rctl,,
    pub 0: u32 pages =,
// Workaround Si errata on PCHx - configure jumbo frame flow.
// If jumbo frames not set, program related MAC/PHY registers
// to h/w defaults
//
    if (hw.mac.type >= e1000_pch2lan) {
    pub ret_val: i32,
    if (adapter.netdev.mtu > ETH_DATA_LEN)
    pub true): ret_val = e1000_lv_jumbo_workaround_ich8lan(hw,,
    else
    pub false): ret_val = e1000_lv_jumbo_workaround_ich8lan(hw,,
    if (ret_val)
    pub mode\n"): e_dbg("failed to enable|disable jumbo frame workaround,
    }
// Program MC offset vector base
    pub er32(RCTL): rctl =,
    pub E1000_RCTL_MO_SHIFT): rctl &= ~(3 <<,
    rctl |= E1000_RCTL_EN | E1000_RCTL_BAM |
    E1000_RCTL_LBM_NO | E1000_RCTL_RDMTS_HALF |
    pub E1000_RCTL_MO_SHIFT): (adapter->hw.mac.mc_filter_type <<,
// Do not Store bad packets
    pub ~E1000_RCTL_SBP: rctl &=,
// Enable Long Packet receive
    if (adapter.netdev.mtu <= ETH_DATA_LEN)
    pub ~E1000_RCTL_LPE: rctl &=,
    else
    pub E1000_RCTL_LPE: rctl |=,
// Some systems expect that the CRC is included in SMBUS traffic. The
// hardware strips the CRC before sending to both SMBUS (BMC) and to
// host memory when this is enabled
//
    if (adapter.flags2 & FLAG2_CRC_STRIPPING)
    pub E1000_RCTL_SECRC: rctl |=,
// Workaround Si errata on 82577 PHY - configure IPG for jumbos
    if ((hw.phy.type == e1000_phy_82577) && (rctl & E1000_RCTL_LPE)) {
    pub phy_data: u16,
    pub &phy_data): e1e_rphy(hw, PHY_REG(770, 26),,
    pub 0xfff8: phy_data &=,
    pub BIT(2): phy_data |=,
    pub phy_data): e1e_wphy(hw, PHY_REG(770, 26),,
    pub &phy_data): e1e_rphy(hw, 22,,
    pub 0x0fff: phy_data &=,
    pub BIT(14): phy_data |=,
    pub 0x2823): e1e_wphy(hw, 0x10,,
    pub 0x0003): e1e_wphy(hw, 0x11,,
    pub phy_data): e1e_wphy(hw, 22,,
    }
// Setup buffer sizes
    pub ~E1000_RCTL_SZ_4096: rctl &=,
    pub E1000_RCTL_BSEX: rctl |=,
    switch (adapter.rx_buffer_len) {
    case 2048:
    default:
    pub E1000_RCTL_SZ_2048: rctl |=,
    pub ~E1000_RCTL_BSEX: rctl &=,
    case 4096:
    pub E1000_RCTL_SZ_4096: rctl |=,
    case 8192:
    pub E1000_RCTL_SZ_8192: rctl |=,
    case 16384:
    pub E1000_RCTL_SZ_16384: rctl |=,
    }
// Enable Extended Status in all Receive Descriptors
    pub er32(RFCTL): rfctl =,
    pub E1000_RFCTL_EXTEN: rfctl |=,
    pub rfctl): ew32(RFCTL,,
// 82571 and greater support packet-split where the protocol
// header is placed in skb->data and the packet data is
// placed in pages hanging off of skb_shinfo(skb)->nr_frags.
// In the case of a non-split, skb->data is linearly filled,
// followed by the page buffers.  Therefore, skb->data is
// sized to hold the largest protocol header.
//
// allocations using alloc_page take too long for regular MTU
// so only enable packet split for jumbo frames
//
// Using pages when the page size is greater than 16k wastes
// a lot of memory, since we allocate 3 pages at all times
// per packet.
//
    pub PAGE_USE_COUNT(adapter->netdev->mtu): pages =,
    if ((pages <= 3) && (PAGE_SIZE <= 16384) && (rctl & E1000_RCTL_LPE))
    pub pages: adapter->rx_ps_pages =,
    else
    pub 0: adapter->rx_ps_pages =,
    if (adapter.rx_ps_pages) {
    pub 0: u32 psrctl =,
// Enable Packet split descriptors
    pub E1000_RCTL_DTYP_PS: rctl |=,
    pub E1000_PSRCTL_BSIZE0_SHIFT: psrctl |= adapter->rx_ps_bsize0 >>,
    switch (adapter.rx_ps_pages) {
    case 3:
    pub E1000_PSRCTL_BSIZE3_SHIFT: psrctl |= PAGE_SIZE <<,
    case 2:
    pub E1000_PSRCTL_BSIZE2_SHIFT: psrctl |= PAGE_SIZE <<,
    case 1:
    pub E1000_PSRCTL_BSIZE1_SHIFT: psrctl |= PAGE_SIZE >>,
    }
    pub psrctl): ew32(PSRCTL,,
    }
// This is useful for sniffing bad packets.
    if (adapter.netdev.features & NETIF_F_RXALL) {
// UPE and MPE will be handled by normal PROMISC logic
// in e1000e_set_rx_mode
//
    rctl |= (E1000_RCTL_SBP |	/* Receive bad packets */
    E1000_RCTL_BAM |	/* RX All Bcast Pkts */
    pub /: *mut *mut E1000_RCTL_PMCF); / RX All MAC Ctrl Pkts,
    rctl &= ~(E1000_RCTL_VFE |	/* Disable VLAN filter */
    E1000_RCTL_DPF |	/* Allow filtered pause */
    pub /: *mut *mut E1000_RCTL_CFIEN); / Dis VLAN CFIEN Filter,
// Do not mess with E1000_CTRL_VME, it affects transmit as well,
// and that breaks VLANs.
//
    }
    pub rctl): ew32(RCTL,,
// just started the receive unit, no need to restart
    pub ~FLAG_RESTART_NOW: adapter->flags &=,
    }
//
// e1000_configure_rx - Configure Receive Unit after Reset
// @adapter: board private structure
//
// Configure the Rx unit of the MAC after a reset.
//
#[no_mangle]
unsafe extern "C" fn e1000_configure_rx(adapter: *mut e1000_adapter) {
    static void e1000_configure_rx(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->rx_ring: *mut *mut e1000_ring rx_ring =,
    pub rdba: u64,
    pub ctrl_ext: u32 rdlen, rctl, rxcsum,,
    if (adapter.rx_ps_pages) {
// this is a 32 byte descriptor
    rdlen = rx_ring.count *
    pub e1000_rx_desc_packet_split): sizeof(union,
    pub e1000_clean_rx_irq_ps: adapter->clean_rx =,
    pub e1000_alloc_rx_buffers_ps: adapter->alloc_rx_buf =,
    } else if (adapter.netdev.mtu > ETH_FRAME_LEN + ETH_FCS_LEN) {
    pub e1000_rx_desc_extended): *mut *mut rdlen = rx_ring->count  sizeof(union,
    pub e1000_clean_jumbo_rx_irq: adapter->clean_rx =,
    pub e1000_alloc_jumbo_rx_buffers: adapter->alloc_rx_buf =,
    } else {
    pub e1000_rx_desc_extended): *mut *mut rdlen = rx_ring->count  sizeof(union,
    pub e1000_clean_rx_irq: adapter->clean_rx =,
    pub e1000_alloc_rx_buffers: adapter->alloc_rx_buf =,
    }
// disable receives while setting up the descriptors
    pub er32(RCTL): rctl =,
    if (!(adapter.flags2 & FLAG2_NO_DISABLE_RX))
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
    pub 11000): usleep_range(10000,,
    if (adapter.flags2 & FLAG2_DMA_BURST) {
// set the writeback threshold (only takes effect if the RDTR
// is set). set GRAN=1 and write back up to 0x4 worth, and
// enable prefetching of 0x20 Rx descriptors
// granularity = 01
// wthresh = 04,
// hthresh = 04,
// pthresh = 0x20
//
    pub E1000_RXDCTL_DMA_BURST_ENABLE): ew32(RXDCTL(0),,
    pub E1000_RXDCTL_DMA_BURST_ENABLE): ew32(RXDCTL(1),,
    }
// set the Receive Delay Timer Register
    pub adapter->rx_int_delay): ew32(RDTR,,
// irq moderation
    pub adapter->rx_abs_int_delay): ew32(RADV,,
    if ((adapter.itr_setting != 0) && (adapter.itr != 0))
    pub adapter->itr): e1000e_write_itr(adapter,,
    pub er32(CTRL_EXT): ctrl_ext =,
// Auto-Mask interrupts upon ICR access
    pub E1000_CTRL_EXT_IAME: ctrl_ext |=,
    pub 0xffffffff): ew32(IAM,,
    pub ctrl_ext): ew32(CTRL_EXT,,
// Setup the HW Rx Head and Tail Descriptor Pointers and
// the Base and Length of the Rx Descriptor Ring
//
    pub rx_ring->dma: rdba =,
    pub DMA_BIT_MASK(32))): ew32(RDBAL(0), (rdba &,
    pub 32)): ew32(RDBAH(0), (rdba >>,
    pub rdlen): ew32(RDLEN(0),,
    pub 0): ew32(RDH(0),,
    pub 0): ew32(RDT(0),,
    pub E1000_RDH(0): rx_ring->head = adapter->hw.hw_addr +,
    pub E1000_RDT(0): rx_ring->tail = adapter->hw.hw_addr +,
    if (adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    pub 0): e1000e_update_rdt_wa(rx_ring,,
// Enable Receive Checksum Offload for TCP and UDP
    pub er32(RXCSUM): rxcsum =,
    if (adapter.netdev.features & NETIF_F_RXCSUM)
    pub E1000_RXCSUM_TUOFL: rxcsum |=,
    else
    pub ~E1000_RXCSUM_TUOFL: rxcsum &=,
    pub rxcsum): ew32(RXCSUM,,
// With jumbo frames, excessive C-state transition latencies result
// in dropped transactions.
//
    if (adapter.netdev.mtu > ETH_DATA_LEN) {
    u32 lat =
    ((er32(PBA) & E1000_PBA_RXA_MASK) * 1024 -
    pub 1000: *mut *mut adapter->max_frame_size)  8 /,
    if (adapter.flags & FLAG_IS_ICH) {
    pub er32(RXDCTL(0)): u32 rxdctl =,
    pub BIT(8)): ew32(RXDCTL(0), rxdctl | 0x3 |,
    }
    dev_info(&adapter.pdev.dev,
    pub frames\n"): "Some CPU C-states have been disabled in order to enable jumbo,
    pub lat): cpu_latency_qos_update_request(&adapter->pm_qos_req,,
    } else {
    cpu_latency_qos_update_request(&adapter.pm_qos_req,
    }
// Enable Receives
    pub rctl): ew32(RCTL,,
    }
//
// e1000e_write_mc_addr_list - write multicast addresses to MTA
// @netdev: network interface device structure
//
// Writes multicast address list to the MTA hash table.
// Returns: -ENOMEM on failure
// 0 on no addresses written
// X on writing X addresses to MTA
//
#[no_mangle]
unsafe extern "C" fn e1000e_write_mc_addr_list(netdev: *mut net_device) -> c_int {
    static int e1000e_write_mc_addr_list(struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ha: *mut netdev_hw_addr,
    pub mta_list: *mut u8,
    pub i: c_int,
    if (netdev_mc_empty(netdev)) {
// nothing to program, so clear mc list
    pub 0): hw->mac.ops.update_mc_addr_list(hw, NULL,,
    pub 0: return,
    }
    pub GFP_ATOMIC): mta_list = kcalloc(netdev_mc_count(netdev), ETH_ALEN,,
    if (!mta_list)
    pub -ENOMEM: return,
// update_mc_addr_list expects a packed array of only addresses.
    pub 0: i =,
    netdev_for_each_mc_addr(ha, netdev)
    pub ETH_ALEN): *mut *mut memcpy(mta_list + (i++  ETH_ALEN), ha->addr,,
    pub i): hw->mac.ops.update_mc_addr_list(hw, mta_list,,
    pub netdev_mc_count(netdev): return,
    }
//
// e1000e_write_uc_addr_list - write unicast addresses to RAR table
// @netdev: network interface device structure
//
// Writes unicast address list to the RAR table.
// Returns: -ENOMEM on failure/insufficient address space
// 0 on no addresses written
// X on writing X addresses to the RAR table
//
#[no_mangle]
unsafe extern "C" fn e1000e_write_uc_addr_list(netdev: *mut net_device) -> c_int {
    static int e1000e_write_uc_addr_list(struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rar_entries: c_uint,
    pub 0: int count =,
    pub hw->mac.ops.rar_get_count(hw): rar_entries =,
// save a rar entry for our hardware address
// save a rar entry for the LAA workaround
    if (adapter.flags & FLAG_RESET_OVERWRITES_LAA)
// return ENOMEM indicating insufficient memory for addresses
    if (netdev_uc_count(netdev) > rar_entries)
    pub -ENOMEM: return,
    if (!netdev_uc_empty(netdev) && rar_entries) {
    pub ha: *mut netdev_hw_addr,
// write the addresses in reverse order to avoid write
// combining
//
    netdev_for_each_uc_addr(ha, netdev) {
    pub ret_val: c_int,
    if (!rar_entries)
    pub rar_entries--): ret_val = hw->mac.ops.rar_set(hw, ha->addr,,
    if (ret_val < 0)
    pub -ENOMEM: return,
    }
    }
// zero out the remaining RAR entries not used above
    pub {: for (; rar_entries > 0; rar_entries--),
    pub 0): ew32(RAH(rar_entries),,
    pub 0): ew32(RAL(rar_entries),,
    }
    pub count: return,
    }
//
// e1000e_set_rx_mode - secondary unicast, Multicast and Promiscuous mode set
// @netdev: network interface device structure
//
// The ndo_set_rx_mode entry point is called whenever the unicast or multicast
// address list or the network interface flags are updated.  This routine is
// responsible for configuring the hardware for proper unicast, multicast,
// promiscuous mode, and all-multi behavior.
//
#[no_mangle]
unsafe extern "C" fn e1000e_set_rx_mode(netdev: *mut net_device) {
    static void e1000e_set_rx_mode(struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rctl: u32,
    if (pm_runtime_suspended(netdev.dev.parent))
// Check for Promiscuous and All Multicast modes
    pub er32(RCTL): rctl =,
// clear the affected bits
    pub E1000_RCTL_MPE): rctl &= ~(E1000_RCTL_UPE |,
    if (netdev.flags & IFF_PROMISC) {
    pub E1000_RCTL_MPE): rctl |= (E1000_RCTL_UPE |,
// Do not hardware filter VLANs in promisc mode
    } else {
    pub count: c_int,
    if (netdev.flags & IFF_ALLMULTI) {
    pub E1000_RCTL_MPE: rctl |=,
    } else {
// Write addresses to the MTA, if the attempt fails
// then we should just turn on promiscuous mode so
// that we can at least receive multicast traffic
//
    pub e1000e_write_mc_addr_list(netdev): count =,
    if (count < 0)
    pub E1000_RCTL_MPE: rctl |=,
    }
// Write addresses to available RAR registers, if there is not
// sufficient space to store all the addresses then enable
// unicast promiscuous mode
//
    pub e1000e_write_uc_addr_list(netdev): count =,
    if (count < 0)
    pub E1000_RCTL_UPE: rctl |=,
    }
    pub rctl): ew32(RCTL,,
    if (netdev.features & NETIF_F_HW_VLAN_CTAG_RX)
    else
    }
#[no_mangle]
unsafe extern "C" fn e1000e_setup_rss_hash(adapter: *mut e1000_adapter) {
    static void e1000e_setup_rss_hash(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rxcsum: u32 mrqc,,
    pub rss_key: [u32; 10],
    pub i: c_int,
    pub sizeof(rss_key)): netdev_rss_key_fill(rss_key,,
    pub i++): for (i = 0; i < 10;,
    pub rss_key[i]): ew32(RSSRK(i),,
// Direct all traffic to queue 0
    pub i++): for (i = 0; i < 32;,
    pub 0): ew32(RETA(i),,
// Disable raw packet checksumming so that RSS hash is placed in
// descriptor on writeback.
//
    pub er32(RXCSUM): rxcsum =,
    pub E1000_RXCSUM_PCSD: rxcsum |=,
    pub rxcsum): ew32(RXCSUM,,
    mrqc = (E1000_MRQC_RSS_FIELD_IPV4 |
    E1000_MRQC_RSS_FIELD_IPV4_TCP |
    E1000_MRQC_RSS_FIELD_IPV6 |
    E1000_MRQC_RSS_FIELD_IPV6_TCP |
    pub mrqc): ew32(MRQC,,
    }
//
// e1000e_get_base_timinca - get default SYSTIM time increment attributes
// @adapter: board private structure
// @timinca: pointer to returned time increment attributes
//
// Get attributes for incrementing the System Time Register SYSTIML/H at
// the default base frequency, and set the cyclecounter shift value.
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_get_base_timinca(adapter: *mut e1000_adapter, timinca: *mut u32) -> i32 {
    s32 e1000e_get_base_timinca(struct e1000_adapter *adapter, u32 *timinca)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub shift: u32 incvalue, incperiod,,
// Make sure clock is enabled on I217/I218/I219  before checking
// the frequency
//
    if ((hw.mac.type >= e1000_pch_lpt) &&
    !(er32(TSYNCTXCTL) & E1000_TSYNCTXCTL_ENABLED) &&
    !(er32(TSYNCRXCTL) & E1000_TSYNCRXCTL_ENABLED)) {
    pub er32(FEXTNVM7): u32 fextnvm7 =,
    if (!(fextnvm7 & BIT(0))) {
    pub BIT(0)): ew32(FEXTNVM7, fextnvm7 |,
    }
    }
    switch (hw.mac.type) {
    case e1000_pch2lan:
// Stable 96MHz frequency
    pub INCPERIOD_96MHZ: incperiod =,
    pub INCVALUE_96MHZ: incvalue =,
    pub INCVALUE_SHIFT_96MHZ: shift =,
    pub INCPERIOD_SHIFT_96MHZ: adapter->cc.shift = shift +,
    case e1000_pch_lpt:
    if (er32(TSYNCRXCTL) & E1000_TSYNCRXCTL_SYSCFI) {
// Stable 96MHz frequency
    pub INCPERIOD_96MHZ: incperiod =,
    pub INCVALUE_96MHZ: incvalue =,
    pub INCVALUE_SHIFT_96MHZ: shift =,
    pub INCPERIOD_SHIFT_96MHZ: adapter->cc.shift = shift +,
    } else {
// Stable 25MHz frequency
    pub INCPERIOD_25MHZ: incperiod =,
    pub INCVALUE_25MHZ: incvalue =,
    pub INCVALUE_SHIFT_25MHZ: shift =,
    pub shift: adapter->cc.shift =,
    }
    case e1000_pch_spt:
// Stable 24MHz frequency
    pub INCPERIOD_24MHZ: incperiod =,
    pub INCVALUE_24MHZ: incvalue =,
    pub INCVALUE_SHIFT_24MHZ: shift =,
    pub shift: adapter->cc.shift =,
    case e1000_pch_cnp:
    case e1000_pch_tgp:
    case e1000_pch_adp:
    case e1000_pch_nvp:
    if (er32(TSYNCRXCTL) & E1000_TSYNCRXCTL_SYSCFI) {
// Stable 24MHz frequency
    pub INCPERIOD_24MHZ: incperiod =,
    pub INCVALUE_24MHZ: incvalue =,
    pub INCVALUE_SHIFT_24MHZ: shift =,
    pub shift: adapter->cc.shift =,
    } else {
// Stable 38400KHz frequency
    pub INCPERIOD_38400KHZ: incperiod =,
    pub INCVALUE_38400KHZ: incvalue =,
    pub INCVALUE_SHIFT_38400KHZ: shift =,
    pub shift: adapter->cc.shift =,
    }
    case e1000_pch_mtp:
    case e1000_pch_lnp:
    case e1000_pch_ptp:
// System firmware can misreport this value, so set it to a
// stable 38400KHz frequency.
//
    pub INCPERIOD_38400KHZ: incperiod =,
    pub INCVALUE_38400KHZ: incvalue =,
    pub INCVALUE_SHIFT_38400KHZ: shift =,
    pub shift: adapter->cc.shift =,
    case e1000_82574:
    case e1000_82583:
// Stable 25MHz frequency
    pub INCPERIOD_25MHZ: incperiod =,
    pub INCVALUE_25MHZ: incvalue =,
    pub INCVALUE_SHIFT_25MHZ: shift =,
    pub shift: adapter->cc.shift =,
    default:
    pub -EINVAL: return,
    }
// timinca = ((incperiod << E1000_TIMINCA_INCPERIOD_SHIFT) |
    pub E1000_TIMINCA_INCVALUE_MASK)): ((incvalue << shift) &,
    pub 0: return,
    }
//
// e1000e_config_hwtstamp - configure the hwtstamp registers and enable/disable
// @adapter: board private structure
// @config: timestamp configuration
// @extack: netlink extended ACK for error report
//
// Outgoing time stamping can be enabled and disabled. Play nice and
// disable it when requested, although it shouldn't cause any overhead
// when no packet needs it. At most one packet in the queue may be
// marked for time stamping, otherwise it would be impossible to tell
// for sure to which packet the hardware time stamp belongs.
//
// Incoming time stamping has to be configured via the hardware filters.
// Not all combinations are supported, in particular event type has to be
// specified. Matching the kind of event packet is not supported, with the
// exception of "all V2 events regardless of level 2 or 4".
//
    static int e1000e_config_hwtstamp(struct e1000_adapter *adapter,
    struct kernel_hwtstamp_config *config,
    struct netlink_ext_ack *extack)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub E1000_TSYNCTXCTL_ENABLED: u32 tsync_tx_ctl =,
    pub E1000_TSYNCRXCTL_ENABLED: u32 tsync_rx_ctl =,
    pub 0: u32 rxmtrl =,
    pub 0: u16 rxudp =,
    pub false: bool is_l4 =,
    pub false: bool is_l2 =,
    pub regval: u32,
    if (!(adapter.flags & FLAG_HAS_HW_TIMESTAMP)) {
    pub support"): NL_SET_ERR_MSG(extack, "No HW timestamp,
    pub -EINVAL: return,
    }
    switch (config.tx_type) {
    case HWTSTAMP_TX_OFF:
    pub 0: tsync_tx_ctl =,
    case HWTSTAMP_TX_ON:
    default:
    pub type"): NL_SET_ERR_MSG(extack, "Unsupported TX HW timestamp,
    pub -ERANGE: return,
    }
    switch (config.rx_filter) {
    case HWTSTAMP_FILTER_NONE:
    pub 0: tsync_rx_ctl =,
    case HWTSTAMP_FILTER_PTP_V1_L4_SYNC:
    pub E1000_TSYNCRXCTL_TYPE_L4_V1: tsync_rx_ctl |=,
    pub E1000_RXMTRL_PTP_V1_SYNC_MESSAGE: rxmtrl =,
    pub true: is_l4 =,
    case HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ:
    pub E1000_TSYNCRXCTL_TYPE_L4_V1: tsync_rx_ctl |=,
    pub E1000_RXMTRL_PTP_V1_DELAY_REQ_MESSAGE: rxmtrl =,
    pub true: is_l4 =,
    case HWTSTAMP_FILTER_PTP_V2_L2_SYNC:
// Also time stamps V2 L2 Path Delay Request/Response
    pub E1000_TSYNCRXCTL_TYPE_L2_V2: tsync_rx_ctl |=,
    pub E1000_RXMTRL_PTP_V2_SYNC_MESSAGE: rxmtrl =,
    pub true: is_l2 =,
    case HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ:
// Also time stamps V2 L2 Path Delay Request/Response.
    pub E1000_TSYNCRXCTL_TYPE_L2_V2: tsync_rx_ctl |=,
    pub E1000_RXMTRL_PTP_V2_DELAY_REQ_MESSAGE: rxmtrl =,
    pub true: is_l2 =,
    case HWTSTAMP_FILTER_PTP_V2_L4_SYNC:
// Hardware cannot filter just V2 L4 Sync messages
    case HWTSTAMP_FILTER_PTP_V2_SYNC:
// Also time stamps V2 Path Delay Request/Response.
    pub E1000_TSYNCRXCTL_TYPE_L2_L4_V2: tsync_rx_ctl |=,
    pub E1000_RXMTRL_PTP_V2_SYNC_MESSAGE: rxmtrl =,
    pub true: is_l2 =,
    pub true: is_l4 =,
    case HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ:
// Hardware cannot filter just V2 L4 Delay Request messages
    case HWTSTAMP_FILTER_PTP_V2_DELAY_REQ:
// Also time stamps V2 Path Delay Request/Response.
    pub E1000_TSYNCRXCTL_TYPE_L2_L4_V2: tsync_rx_ctl |=,
    pub E1000_RXMTRL_PTP_V2_DELAY_REQ_MESSAGE: rxmtrl =,
    pub true: is_l2 =,
    pub true: is_l4 =,
    case HWTSTAMP_FILTER_PTP_V2_L4_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_L2_EVENT:
// Hardware cannot filter just V2 L4 or L2 Event messages
    case HWTSTAMP_FILTER_PTP_V2_EVENT:
    pub E1000_TSYNCRXCTL_TYPE_EVENT_V2: tsync_rx_ctl |=,
    pub HWTSTAMP_FILTER_PTP_V2_EVENT: config->rx_filter =,
    pub true: is_l2 =,
    pub true: is_l4 =,
    case HWTSTAMP_FILTER_PTP_V1_L4_EVENT:
// For V1, the hardware can only filter Sync messages or
// Delay Request messages but not both so fall-through to
// time stamp all packets.
//
    case HWTSTAMP_FILTER_NTP_ALL:
    case HWTSTAMP_FILTER_ALL:
    pub true: is_l2 =,
    pub true: is_l4 =,
    pub E1000_TSYNCRXCTL_TYPE_ALL: tsync_rx_ctl |=,
    pub HWTSTAMP_FILTER_ALL: config->rx_filter =,
    default:
    pub filter"): NL_SET_ERR_MSG(extack, "Unsupported RX HW timestamp,
    pub -ERANGE: return,
    }
    pub config: *mut adapter->hwtstamp_config =,
// enable/disable Tx h/w time stamping
    pub er32(TSYNCTXCTL): regval =,
    pub ~E1000_TSYNCTXCTL_ENABLED: regval &=,
    pub tsync_tx_ctl: regval |=,
    pub regval): ew32(TSYNCTXCTL,,
    if ((er32(TSYNCTXCTL) & E1000_TSYNCTXCTL_ENABLED) !=
    (regval & E1000_TSYNCTXCTL_ENABLED)) {
    NL_SET_ERR_MSG(extack,
    pub expected"): "Timesync Tx Control register not set as,
    pub -EAGAIN: return,
    }
// enable/disable Rx h/w time stamping
    pub er32(TSYNCRXCTL): regval =,
    pub E1000_TSYNCRXCTL_TYPE_MASK): regval &= ~(E1000_TSYNCRXCTL_ENABLED |,
    pub tsync_rx_ctl: regval |=,
    pub regval): ew32(TSYNCRXCTL,,
    if ((er32(TSYNCRXCTL) & (E1000_TSYNCRXCTL_ENABLED |
    E1000_TSYNCRXCTL_TYPE_MASK)) !=
    (regval & (E1000_TSYNCRXCTL_ENABLED |
    E1000_TSYNCRXCTL_TYPE_MASK))) {
    NL_SET_ERR_MSG(extack,
    pub expected"): "Timesync Rx Control register not set as,
    pub -EAGAIN: return,
    }
// L2: define ethertype filter for time stamped packets
    if (is_l2)
    pub ETH_P_1588: rxmtrl |=,
// define which PTP packets get time stamped
    pub rxmtrl): ew32(RXMTRL,,
// Filter by destination port
    if (is_l4) {
    pub PTP_EV_PORT: rxudp =,
    }
    pub rxudp): ew32(RXUDP,,
// Clear TSYNCRXCTL_VALID & TSYNCTXCTL_VALID bit
    pub 0: return,
    }
//
// e1000_configure - configure the hardware for Rx and Tx
// @adapter: private board structure
//
#[no_mangle]
unsafe extern "C" fn e1000_configure(adapter: *mut e1000_adapter) {
    static void e1000_configure(struct e1000_adapter *adapter)
    {
    pub adapter->rx_ring: *mut *mut e1000_ring rx_ring =,
    if (adapter.netdev.features & NETIF_F_RXHASH)
    pub GFP_KERNEL): adapter->alloc_rx_buf(rx_ring, e1000_desc_unused(rx_ring),,
    }
//
// e1000e_power_up_phy - restore link in case the phy was powered down
// @adapter: address of board private structure
//
// The phy may be powered down to save power and turn off link when the
// driver is unloaded and wake on lan is not enabled (among others)
// *** this routine MUST be followed by a call to e1000e_reset
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_power_up_phy(adapter: *mut e1000_adapter) {
    void e1000e_power_up_phy(struct e1000_adapter *adapter)
    {
    if (adapter.hw.phy.ops.power_up)
    }
//
// e1000_power_down_phy - Power down the PHY
// @adapter: board private structure
//
// Power down the PHY so no link is implied when interface is down.
// The PHY cannot be powered down if management or WoL is active.
//
#[no_mangle]
unsafe extern "C" fn e1000_power_down_phy(adapter: *mut e1000_adapter) {
    static void e1000_power_down_phy(struct e1000_adapter *adapter)
    {
    if (adapter.hw.phy.ops.power_down)
    }
//
// e1000_flush_tx_ring - remove all descriptors from the tx_ring
// @adapter: board private structure
//
// We want to clear all pending descriptors from the TX ring.
// zeroing happens when the HW reads the regs. We  assign the ring itself as
// the data of the next descriptor. We don't care about the data we are about
// to reset the HW.
//
#[no_mangle]
unsafe extern "C" fn e1000_flush_tx_ring(adapter: *mut e1000_adapter) {
    static void e1000_flush_tx_ring(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub NULL: *mut *mut e1000_tx_desc tx_desc =,
    pub E1000_TXD_CMD_IFCS: u32 tdt, tctl, txd_lower =,
    pub 512: u16 size =,
    pub er32(TCTL): tctl =,
    pub E1000_TCTL_EN): ew32(TCTL, tctl |,
    pub er32(TDT(0)): tdt =,
    pub tx_ring->next_to_use): BUG_ON(tdt !=,
    pub tx_ring->next_to_use): *mut *mut tx_desc = E1000_TX_DESC(tx_ring,,
    pub cpu_to_le64(tx_ring->dma): tx_desc->buffer_addr =,
    pub size): tx_desc->lower.data = cpu_to_le32(txd_lower |,
    pub 0: tx_desc->upper.data =,
// flush descriptors to memory before notifying the HW
    if (tx_ring.next_to_use == tx_ring.count)
    pub 0: tx_ring->next_to_use =,
    pub tx_ring->next_to_use): ew32(TDT(0),,
    pub 250): usleep_range(200,,
    }
//
// e1000_flush_rx_ring - remove all descriptors from the rx_ring
// @adapter: board private structure
//
// Mark all descriptors in the RX ring as consumed and disable the rx ring
//
#[no_mangle]
unsafe extern "C" fn e1000_flush_rx_ring(adapter: *mut e1000_adapter) {
    static void e1000_flush_rx_ring(struct e1000_adapter *adapter)
    {
    pub rxdctl: u32 rctl,,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(RCTL): rctl =,
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
    pub 150): usleep_range(100,,
    pub er32(RXDCTL(0)): rxdctl =,
// zero the lower 14 bits (prefetch and host thresholds)
    pub 0xffffc000: rxdctl &=,
// update thresholds: prefetch threshold to 31, host threshold to 1
// and make sure the granularity is "descriptors" and not "cache lines"
//
    pub E1000_RXDCTL_THRESH_UNIT_DESC): rxdctl |= (0x1F | BIT(8) |,
    pub rxdctl): ew32(RXDCTL(0),,
// momentarily enable the RX ring for the changes to take effect
    pub E1000_RCTL_EN): ew32(RCTL, rctl |,
    pub 150): usleep_range(100,,
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
    }
//
// e1000_flush_desc_rings - remove all descriptors from the descriptor rings
// @adapter: board private structure
//
// In i219, the descriptor rings must be emptied before resetting the HW
// or before changing the device state to D3 during runtime (runtime PM).
//
// Failure to do this will cause the HW to enter a unit hang state which can
// only be released by PCI reset on the device
//
#[no_mangle]
unsafe extern "C" fn e1000_flush_desc_rings(adapter: *mut e1000_adapter) {
    static void e1000_flush_desc_rings(struct e1000_adapter *adapter)
    {
    pub hang_state: u16,
    pub tdlen: u32 fext_nvm11,,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
// First, disable MULR fix in FEXTNVM11
    pub er32(FEXTNVM11): fext_nvm11 =,
    pub E1000_FEXTNVM11_DISABLE_MULR_FIX: fext_nvm11 |=,
    pub fext_nvm11): ew32(FEXTNVM11,,
// do nothing if we're not in faulty state, or if the queue is empty
    pub er32(TDLEN(0)): tdlen =,
    pci_read_config_word(adapter.pdev, PCICFG_DESC_RING_STATUS,
    if (!(hang_state & FLUSH_DESC_REQUIRED) || !tdlen)
// recheck, maybe the fault is caused by the rx ring
    pci_read_config_word(adapter.pdev, PCICFG_DESC_RING_STATUS,
    if (hang_state & FLUSH_DESC_REQUIRED)
    }
//
// e1000e_systim_reset - reset the timesync registers after a hardware reset
// @adapter: board private structure
//
// When the MAC is reset, all hardware bits for timesync will be reset to the
// default values. This function will restore the settings last in place.
// Since the clock SYSTIME registers are reset, we will simply restore the
// cyclecounter to the kernel real clock time.
//
#[no_mangle]
unsafe extern "C" fn e1000e_systim_reset(adapter: *mut e1000_adapter) {
    static void e1000e_systim_reset(struct e1000_adapter *adapter)
    {
    pub &adapter->ptp_clock_info: *mut *mut ptp_clock_info info =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub {}: netlink_ext_ack extack =,
    pub flags: c_ulong,
    pub timinca: u32,
    pub ret_val: i32,
    if (!(adapter.flags & FLAG_HAS_HW_TIMESTAMP))
    if (info.adjfine) {
// restore the previous ptp frequency delta
    pub adapter->ptp_delta): ret_val = info->adjfine(info,,
    } else {
// set the default base frequency if no adjustment possible
    pub &timinca): ret_val = e1000e_get_base_timinca(adapter,,
    if (!ret_val)
    pub timinca): ew32(TIMINCA,,
    }
    if (ret_val) {
    dev_warn(&adapter.pdev.dev,
    "Failed to restore TIMINCA clock rate delta: %d\n",
    }
// reset the systim ns time counter
    pub flags): spin_lock_irqsave(&adapter->systim_lock,,
    timecounter_init(&adapter.tc, &adapter.cc,
    pub flags): spin_unlock_irqrestore(&adapter->systim_lock,,
// restore the previous hwtstamp configuration settings
    ret_val = e1000e_config_hwtstamp(adapter, &adapter.hwtstamp_config,
    if (ret_val) {
    if (extack._msg)
    pub extack._msg): e_err("%s\n",,
    }
    }
//
// e1000e_reset - bring the hardware into a known good state
// @adapter: board private structure
//
// This function boots the hardware and enables some settings that
// require a configuration cycle of the hardware - those cannot be
// set/changed during runtime. After reset the device needs to be
// properly configured for Rx, Tx etc.
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_reset(adapter: *mut e1000_adapter) {
    void e1000e_reset(struct e1000_adapter *adapter)
    {
    pub &adapter->hw.mac: *mut *mut e1000_mac_info mac =,
    pub &adapter->hw.fc: *mut *mut e1000_fc_info fc =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub min_rx_space: u32 tx_space, min_tx_space,,
    pub adapter->pba: u32 pba =,
    pub hwm: u16,
// reset Packet Buffer Allocation to default
    pub pba): ew32(PBA,,
    if (adapter.max_frame_size > (VLAN_ETH_FRAME_LEN + ETH_FCS_LEN)) {
// To maintain wire speed transmits, the Tx FIFO should be
// large enough to accommodate two full transmit packets,
// rounded up to the next 1KB and expressed in KB.  Likewise,
// the Rx FIFO should be large enough to accommodate at least
// one full receive packet and is similarly rounded up and
// expressed in KB.
//
    pub er32(PBA): pba =,
// upper 16 bits has Tx packet buffer allocation size in KB
    pub 16: tx_space = pba >>,
// lower 16 bits has Rx packet buffer allocation size in KB
    pub 0xffff: pba &=,
// the Tx fifo also stores 16 bytes of information about the Tx
// but don't include ethernet FCS because hardware appends it
//
    min_tx_space = (adapter.max_frame_size +
    pub 2: *mut *mut sizeof(struct e1000_tx_desc) - ETH_FCS_LEN),
    pub 1024): min_tx_space = ALIGN(min_tx_space,,
    pub 10: min_tx_space >>=,
// software strips receive CRC, so leave room for it
    pub adapter->max_frame_size: min_rx_space =,
    pub 1024): min_rx_space = ALIGN(min_rx_space,,
    pub 10: min_rx_space >>=,
// If current Tx allocation is less than the min Tx FIFO size,
// and the min Tx FIFO size is less than the current Rx FIFO
// allocation, take space away from current Rx allocation
//
    if ((tx_space < min_tx_space) &&
    ((min_tx_space - tx_space) < pba)) {
    pub tx_space: pba -= min_tx_space -,
// if short on Rx space, Rx wins and must trump Tx
// adjustment
//
    if (pba < min_rx_space)
    pub min_rx_space: pba =,
    }
    pub pba): ew32(PBA,,
    }
// flow control settings
//
// The high water mark must be low enough to fit one full frame
// (or the size used for early receive) above it in the Rx FIFO.
// Set it to the lower of:
// - 90% of the Rx FIFO size, and
// - the full Rx FIFO size minus one full frame
//
    if (adapter.flags & FLAG_DISABLE_FC_PAUSE_TIME)
    pub 0xFFFF: fc->pause_time =,
    else
    pub E1000_FC_PAUSE_TIME: fc->pause_time =,
    pub true: fc->send_xon =,
    pub fc->requested_mode: fc->current_mode =,
    switch (hw.mac.type) {
    case e1000_ich9lan:
    case e1000_ich10lan:
    if (adapter.netdev.mtu > ETH_DATA_LEN) {
    pub 14: pba =,
    pub pba): ew32(PBA,,
    pub 0x2800: fc->high_water =,
    pub 8: fc->low_water = fc->high_water -,
    }
    default:
    hwm = min(((pba << 10) * 9 / 10),
    pub adapter->max_frame_size)): ((pba << 10) -,
    pub /: *mut *mut fc->high_water = hwm & E1000_FCRTH_RTH; / 8-byte granularity,
    pub 8: fc->low_water = fc->high_water -,
    case e1000_pchlan:
// Workaround PCH LOM adapter hangs with certain network
// loads.  If hangs persist, try disabling Tx flow control.
//
    if (adapter.netdev.mtu > ETH_DATA_LEN) {
    pub 0x3500: fc->high_water =,
    pub 0x1500: fc->low_water =,
    } else {
    pub 0x5000: fc->high_water =,
    pub 0x3000: fc->low_water =,
    }
    pub 0x1000: fc->refresh_time =,
    case e1000_pch2lan:
    case e1000_pch_lpt:
    case e1000_pch_spt:
    case e1000_pch_cnp:
    case e1000_pch_tgp:
    case e1000_pch_adp:
    case e1000_pch_mtp:
    case e1000_pch_lnp:
    case e1000_pch_ptp:
    case e1000_pch_nvp:
    pub 0xFFFF: fc->refresh_time =,
    pub 0xFFFF: fc->pause_time =,
    if (adapter.netdev.mtu <= ETH_DATA_LEN) {
    pub 0x05C20: fc->high_water =,
    pub 0x05048: fc->low_water =,
    }
    pub 14: pba =,
    pub pba): ew32(PBA,,
    pub E1000_FCRTH_RTH: *mut *mut fc->high_water = ((pba << 10)  9 / 10) &,
    pub E1000_FCRTL_RTL: *mut *mut fc->low_water = ((pba << 10)  8 / 10) &,
    }
// Alignment of Tx data is on an arbitrary byte boundary with the
// maximum size per Tx descriptor limited only to the transmit
// allocation of the packet buffer minus 96 bytes with an upper
// limit of 24KB due to receive synchronization limitations.
//
    adapter.tx_fifo_limit = min_t(u32, ((er32(PBA) >> 16) << 10) - 96,
    pub 10): 24 <<,
// Disable Adaptive Interrupt Moderation if 2 full packets cannot
// fit in receive buffer.
//
    if (adapter.itr_setting & 0x3) {
    if ((adapter.max_frame_size * 2) > (pba << 10)) {
    if (!(adapter.flags2 & FLAG2_DISABLE_AIM)) {
    dev_info(&adapter.pdev.dev,
    pub off\n"): "Interrupt Throttle Rate,
    pub FLAG2_DISABLE_AIM: adapter->flags2 |=,
    pub 0): e1000e_write_itr(adapter,,
    }
    } else if (adapter.flags2 & FLAG2_DISABLE_AIM) {
    dev_info(&adapter.pdev.dev,
    pub on\n"): "Interrupt Throttle Rate,
    pub ~FLAG2_DISABLE_AIM: adapter->flags2 &=,
    pub 20000: adapter->itr =,
    pub adapter->itr): e1000e_write_itr(adapter,,
    }
    }
    if (hw.mac.type >= e1000_pch_spt)
// Allow time for pending master requests to run
// For parts with AMT enabled, let the firmware know
// that the network interface is in control
//
    if (adapter.flags & FLAG_HAS_AMT)
    pub 0): ew32(WUC,,
    if (mac.ops.init_hw(hw))
    pub Error\n"): e_err("Hardware,
// Enable h/w to recognize an 802.1Q VLAN Ethernet packet
    pub ETH_P_8021Q): ew32(VET,,
// restore systim and hwtstamp settings
// Set EEE advertisement as appropriate
    if (adapter.flags2 & FLAG2_HAS_EEE) {
    pub ret_val: i32,
    pub adv_addr: u16,
    switch (hw.phy.type) {
    case e1000_phy_82579:
    pub I82579_EEE_ADVERTISEMENT: adv_addr =,
    case e1000_phy_i217:
    pub I217_EEE_ADVERTISEMENT: adv_addr =,
    default:
    dev_err(&adapter.pdev.dev,
    pub advertisement\n"): "Invalid PHY type setting EEE,
    }
    pub hw->phy.ops.acquire(hw): ret_val =,
    if (ret_val) {
    dev_err(&adapter.pdev.dev,
    pub PHY\n"): "EEE advertisement - unable to acquire,
    }
    e1000_write_emi_reg_locked(hw, adv_addr,
    hw.dev_spec.ich8lan.eee_disable ?
    pub adapter->eee_advert): 0 :,
    }
    if (!netif_running(adapter.netdev) &&
    !test_bit(__E1000_TESTING, &adapter.state))
    if ((adapter.flags & FLAG_HAS_SMART_POWER_DOWN) &&
    !(adapter.flags & FLAG_SMART_POWER_DOWN)) {
    pub 0: u16 phy_data =,
// speed up time to link by disabling smart power down, ignore
// the return value of this function because there is nothing
// different we would do if it failed
//
    pub &phy_data): e1e_rphy(hw, IGP02E1000_PHY_POWER_MGMT,,
    pub ~IGP02E1000_PM_SPD: phy_data &=,
    pub phy_data): e1e_wphy(hw, IGP02E1000_PHY_POWER_MGMT,,
    }
    if (hw.mac.type >= e1000_pch_spt && adapter.int_mode == 0) {
    pub reg: u32,
// Fextnvm7 @ 0xe4[2] = 1
    pub er32(FEXTNVM7): reg =,
    pub E1000_FEXTNVM7_SIDE_CLK_UNGATE: reg |=,
    pub reg): ew32(FEXTNVM7,,
// Fextnvm9 @ 0x5bb4[13:12] = 11
    pub er32(FEXTNVM9): reg =,
    reg |= E1000_FEXTNVM9_IOSFSB_CLKGATE_DIS |
    pub reg): ew32(FEXTNVM9,,
    }
    }
//
// e1000e_trigger_lsc - trigger an LSC interrupt
// @adapter: board private structure
//
// Fire a link status change interrupt to start the watchdog.
//
#[no_mangle]
unsafe extern "C" fn e1000e_trigger_lsc(adapter: *mut e1000_adapter) {
    static void e1000e_trigger_lsc(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    if (adapter.msix_entries)
    pub E1000_ICS_OTHER): ew32(ICS, E1000_ICS_LSC |,
    else
    pub E1000_ICS_LSC): ew32(ICS,,
    }
#[no_mangle]
pub unsafe extern "C" fn e1000e_up(adapter: *mut e1000_adapter) {
    void e1000e_up(struct e1000_adapter *adapter)
    {
// hardware has been reset, we need to reload some things
    pub &adapter->state): clear_bit(__E1000_DOWN,,
    if (adapter.msix_entries)
// Tx queue started by watchdog timer when link is up
    }
#[no_mangle]
unsafe extern "C" fn e1000e_flush_descriptors(adapter: *mut e1000_adapter) {
    static void e1000e_flush_descriptors(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    if (!(adapter.flags2 & FLAG2_DMA_BURST))
// flush pending descriptor writebacks to memory
    pub E1000_TIDV_FPD): ew32(TIDV, adapter->tx_int_delay |,
    pub E1000_RDTR_FPD): ew32(RDTR, adapter->rx_int_delay |,
// execute the writes immediately
// due to rare timing issues, write to TIDV/RDTR again to ensure the
// write is successful
//
    pub E1000_TIDV_FPD): ew32(TIDV, adapter->tx_int_delay |,
    pub E1000_RDTR_FPD): ew32(RDTR, adapter->rx_int_delay |,
// execute the writes immediately
    }
    pub adapter): *mut static void e1000e_update_stats(struct e1000_adapter,
//
// e1000e_down - quiesce the device and optionally reset the hardware
// @adapter: board private structure
// @reset: boolean flag to reset the hardware or not
//
#[no_mangle]
pub unsafe extern "C" fn e1000e_down(adapter: *mut e1000_adapter, reset: bool) {
    void e1000e_down(struct e1000_adapter *adapter, bool reset)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub rctl: u32 tctl,,
// signal that we're down so the interrupt handler does not
// reschedule our watchdog timer
//
    pub &adapter->state): set_bit(__E1000_DOWN,,
// disable receives in the hardware
    pub er32(RCTL): rctl =,
    if (!(adapter.flags2 & FLAG2_NO_DISABLE_RX))
    pub ~E1000_RCTL_EN): ew32(RCTL, rctl &,
// flush and sleep below
// disable transmits in the hardware
    pub er32(TCTL): tctl =,
    pub ~E1000_TCTL_EN: tctl &=,
    pub tctl): ew32(TCTL,,
// flush both disables and wait for them to finish
    pub 11000): usleep_range(10000,,
    pub 0: adapter->link_speed =,
    pub 0: adapter->link_duplex =,
// Disable Si errata workaround on PCHx for jumbo frame flow
    if ((hw.mac.type >= e1000_pch2lan) &&
    (adapter.netdev.mtu > ETH_DATA_LEN) &&
    e1000_lv_jumbo_workaround_ich8lan(hw, false))
    pub mode\n"): e_dbg("failed to disable jumbo frame workaround,
    if (!pci_channel_offline(adapter.pdev)) {
    if (reset)
#[no_mangle]
pub unsafe extern "C" fn if(e1000_pch_spt: hw->mac.type >=) -> else {
    else if (hw.mac.type >= e1000_pch_spt)
    }
    }
#[no_mangle]
pub unsafe extern "C" fn e1000e_reinit_locked(adapter: *mut e1000_adapter) {
    void e1000e_reinit_locked(struct e1000_adapter *adapter)
    {
    while (test_and_set_bit(__E1000_RESETTING, &adapter.state))
    pub 1100): usleep_range(1000,,
    pub true): e1000e_down(adapter,,
    pub &adapter->state): clear_bit(__E1000_RESETTING,,
    }
//
// e1000e_sanitize_systim - sanitize raw cycle counter reads
// @hw: pointer to the HW structure
// @systim: PHC time value read, sanitized and returned
// @sts: structure to hold system time before and after reading SYSTIML,
// may be NULL
//
// Errata for 82574/82583 possible bad bits read from SYSTIMH/L:
// check to see that the time is incrementing at a reasonable
// rate and is a multiple of incvalue.
//
    static u64 e1000e_sanitize_systim(struct e1000_hw *hw, u64 systim,
    struct ptp_system_timestamp *sts)
    {
    pub temp: u64 time_delta, rem,,
    pub systim_next: u64,
    pub incvalue: u32,
    pub i: c_int,
    pub E1000_TIMINCA_INCVALUE_MASK: incvalue = er32(TIMINCA) &,
    pub {: for (i = 0; i < E1000_MAX_82574_SYSTIM_REREADS; i++),
// latch SYSTIMH on read of SYSTIML
    pub (u64)er32(SYSTIML): systim_next =,
    pub 32: systim_next |= (u64)er32(SYSTIMH) <<,
    pub systim: time_delta = systim_next -,
    pub time_delta: temp =,
// VMWare users have seen incvalue of zero, don't div / 0
    pub 0): rem = incvalue ? do_div(temp, incvalue) : (time_delta !=,
    pub systim_next: systim =,
    if ((time_delta < E1000_82574_SYSTIM_EPSILON) && (rem == 0))
    }
    pub systim: return,
    }
//
// e1000e_read_systim - read SYSTIM register
// @adapter: board private structure
// @sts: structure which will contain system time before and after reading
// SYSTIML, may be NULL
//
    u64 e1000e_read_systim(struct e1000_adapter *adapter,
    struct ptp_system_timestamp *sts)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub systimeh: u32 systimel, systimel_2,,
    pub systim: u64,
// SYSTIMH latching upon SYSTIML read does not work well.
// This means that if SYSTIML overflows after we read it but before
// we read SYSTIMH, the value of SYSTIMH has been incremented and we
// will experience a huge non linear increment in the systime value
// to fix that we test for overflow and if true, we re-read systime.
//
    pub er32(SYSTIML): systimel =,
    pub er32(SYSTIMH): systimeh =,
// Is systimel is so large that overflow is possible?
    if (systimel >= (u32)0xffffffff - E1000_TIMINCA_INCVALUE_MASK) {
    pub er32(SYSTIML): systimel_2 =,
    if (systimel > systimel_2) {
// There was an overflow, read again SYSTIMH, and use
// systimel_2
//
    pub er32(SYSTIMH): systimeh =,
    pub systimel_2: systimel =,
    }
    }
    pub (u64)systimel: systim =,
    pub 32: systim |= (u64)systimeh <<,
    if (adapter.flags2 & FLAG2_CHECK_SYSTIM_OVERFLOW)
    pub sts): systim = e1000e_sanitize_systim(hw, systim,,
    pub systim: return,
    }
//
// e1000e_cyclecounter_read - read raw cycle counter (used by time counter)
// @cc: cyclecounter structure
//
#[no_mangle]
unsafe extern "C" fn e1000e_cyclecounter_read(cc: *mut cyclecounter) -> u64 {
    static u64 e1000e_cyclecounter_read(struct cyclecounter *cc)
    {
    struct e1000_adapter *adapter = container_of(cc, struct e1000_adapter,
    pub NULL): return e1000e_read_systim(adapter,,
    }
//
// e1000_sw_init - Initialize general software structures (struct e1000_adapter)
// @adapter: board private structure to initialize
//
// e1000_sw_init initializes the Adapter private data structure.
// Fields are initialized based on PCI device information and
// OS network device settings (MTU size).
//
#[no_mangle]
unsafe extern "C" fn e1000_sw_init(adapter: *mut e1000_adapter) -> c_int {
    static int e1000_sw_init(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub ETH_FCS_LEN: adapter->rx_buffer_len = VLAN_ETH_FRAME_LEN +,
    pub 128: adapter->rx_ps_bsize0 =,
    pub ETH_FCS_LEN: adapter->max_frame_size = netdev->mtu + VLAN_ETH_HLEN +,
    pub ETH_FCS_LEN: adapter->min_frame_size = ETH_ZLEN +,
    pub E1000_DEFAULT_TXD: adapter->tx_ring_count =,
    pub E1000_DEFAULT_RXD: adapter->rx_ring_count =,
    if (e1000_alloc_queues(adapter))
    pub -ENOMEM: return,
// Setup hardware time stamping cyclecounter
    if (adapter.flags & FLAG_HAS_HW_TIMESTAMP) {
    pub e1000e_cyclecounter_read: adapter->cc.read =,
    pub CYCLECOUNTER_MASK(64): adapter->cc.mask =,
    pub 1: adapter->cc.mult =,
// cc.shift set in e1000e_get_base_tininca()
    pub e1000e_tx_hwtstamp_work): INIT_WORK(&adapter->tx_hwtstamp_work,,
    }
// Explicitly disable IRQ since the NIC can be in any state.
    pub &adapter->state): set_bit(__E1000_DOWN,,
    pub 0: return,
    }
//
// e1000_intr_msi_test - Interrupt Handler
// @irq: interrupt number
// @data: pointer to a network interface device structure
//
#[no_mangle]
unsafe extern "C" fn e1000_intr_msi_test(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_intr_msi_test(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(ICR): u32 icr =,
    pub icr): e_dbg("icr is %08X\n",,
    if (icr & E1000_ICR_RXSEQ) {
    pub ~FLAG_MSI_TEST_FAILED: adapter->flags &=,
// Force memory writes to complete before acknowledging the
// interrupt is handled.
//
    }
    pub IRQ_HANDLED: return,
    }
//
// e1000_test_msi_interrupt - Returns 0 for successful test
// @adapter: board private struct
//
// code flow taken from tg3.c
//
#[no_mangle]
unsafe extern "C" fn e1000_test_msi_interrupt(adapter: *mut e1000_adapter) -> c_int {
    static int e1000_test_msi_interrupt(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub err: c_int,
// poll_enable hasn't been called yet, so don't need disable
// clear any pending events
// free the real vector and request a test handler
// Assume that the test fails, if it succeeds then the test
// MSI irq handler will unset this flag
//
    pub FLAG_MSI_TEST_FAILED: adapter->flags |=,
    pub pci_enable_msi(adapter->pdev): err =,
    if (err)
    pub msi_test_failed: goto,
    err = request_irq(adapter.pdev.irq, e1000_intr_msi_test, 0,
    pub netdev): netdev->name,,
    if (err) {
    pub msi_test_failed: goto,
    }
// Force memory writes to complete before enabling and firing an
// interrupt.
//
// fire an unusual interrupt on the test handler
    pub E1000_ICS_RXSEQ): ew32(ICS,,
    pub /: *mut *mut rmb(); / read flags after interrupt has been fired,
    if (adapter.flags & FLAG_MSI_TEST_FAILED) {
    pub E1000E_INT_MODE_LEGACY: adapter->int_mode =,
    pub interrupt.\n"): e_info("MSI interrupt test failed, using legacy,
    } else {
    pub succeeded!\n"): e_dbg("MSI interrupt test,
    }
    pub netdev): free_irq(adapter->pdev->irq,,
    msi_test_failed:
    pub e1000_request_irq(adapter): return,
    }
//
// e1000_test_msi - Returns 0 if MSI test succeeds or INTx mode is restored
// @adapter: board private struct
//
// code flow taken from tg3.c, called with e1000 interrupts disabled.
//
#[no_mangle]
unsafe extern "C" fn e1000_test_msi(adapter: *mut e1000_adapter) -> c_int {
    static int e1000_test_msi(struct e1000_adapter *adapter)
    {
    pub err: c_int,
    pub pci_cmd: u16,
    if (!(adapter.flags & FLAG_MSI_ENABLED))
    pub 0: return,
// disable SERR in case the MSI write causes a master abort
    pub &pci_cmd): pci_read_config_word(adapter->pdev, PCI_COMMAND,,
    if (pci_cmd & PCI_COMMAND_SERR)
    pci_write_config_word(adapter.pdev, PCI_COMMAND,
    pub ~PCI_COMMAND_SERR): pci_cmd &,
    pub e1000_test_msi_interrupt(adapter): err =,
// re-enable SERR
    if (pci_cmd & PCI_COMMAND_SERR) {
    pub &pci_cmd): pci_read_config_word(adapter->pdev, PCI_COMMAND,,
    pub PCI_COMMAND_SERR: pci_cmd |=,
    pub pci_cmd): pci_write_config_word(adapter->pdev, PCI_COMMAND,,
    }
    pub err: return,
    }
//
// e1000e_open - Called when a network interface is made active
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
pub unsafe extern "C" fn e1000e_open(netdev: *mut net_device) -> c_int {
    int e1000e_open(struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub err: c_int,
    pub irq: c_int,
// disallow open during test
    if (test_bit(__E1000_TESTING, &adapter.state))
    pub -EBUSY: return,
// allocate transmit descriptors
    pub e1000e_setup_tx_resources(adapter->tx_ring): err =,
    if (err)
    pub err_setup_tx: goto,
// allocate receive descriptors
    pub e1000e_setup_rx_resources(adapter->rx_ring): err =,
    if (err)
    pub err_setup_rx: goto,
// If AMT is enabled, let the firmware know that the network
// interface is now open and reset the part to a known state.
//
    if (adapter.flags & FLAG_HAS_AMT) {
    }
    pub E1000_MNG_VLAN_NONE: adapter->mng_vlan_id =,
    if ((adapter.hw.mng_cookie.status & E1000_MNG_DHCP_COOKIE_STATUS_VLAN))
// DMA latency requirement to workaround jumbo issue
    pub PM_QOS_DEFAULT_VALUE): cpu_latency_qos_add_request(&adapter->pm_qos_req,,
// before we allocate an interrupt, we must be ready to handle it.
// Setting DEBUG_SHIRQ in the kernel makes it fire an interrupt
// as soon as we call pci_request_irq, so we have to setup our
// clean_rx handler before we do so.
//
    pub e1000_request_irq(adapter): err =,
    if (err)
    pub err_req_irq: goto,
// Work around PCIe errata with MSI interrupts causing some chipsets to
// ignore e1000e MSI messages, which means we need to test our MSI
// interrupt now
//
    if (adapter.int_mode != E1000E_INT_MODE_LEGACY) {
    pub e1000_test_msi(adapter): err =,
    if (err) {
    pub failed\n"): e_err("Interrupt allocation,
    pub err_req_irq: goto,
    }
    }
// From here on the code is the same as e1000e_up()
    pub &adapter->state): clear_bit(__E1000_DOWN,,
    if (adapter.int_mode == E1000E_INT_MODE_MSIX)
    pub adapter->msix_entries[0].vector: irq =,
    else
    pub adapter->pdev->irq: irq =,
    pub irq): netif_napi_set_irq(&adapter->napi,,
    pub &adapter->napi): netif_queue_set_napi(netdev, 0, NETDEV_QUEUE_TYPE_RX,,
    pub &adapter->napi): netif_queue_set_napi(netdev, 0, NETDEV_QUEUE_TYPE_TX,,
    pub false: adapter->tx_hang_recheck =,
    pub true: hw->mac.get_link_status =,
    pub 0: return,
    err_req_irq:
    err_setup_rx:
    err_setup_tx:
    pub err: return,
    }
//
// e1000e_close - Disables a network interface
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
pub unsafe extern "C" fn e1000e_close(netdev: *mut net_device) -> c_int {
    int e1000e_close(struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub E1000_CHECK_RESET_COUNT: int count =,
    while (test_bit(__E1000_RESETTING, &adapter.state) && count--)
    pub 11000): usleep_range(10000,,
    pub &adapter->state)): WARN_ON(test_bit(__E1000_RESETTING,,
    if (netif_device_present(netdev)) {
    pub true): e1000e_down(adapter,,
// Link status message must follow this format
    pub Down\n"): netdev_info(netdev, "NIC Link is,
    }
    pub NULL): netif_queue_set_napi(netdev, 0, NETDEV_QUEUE_TYPE_RX,,
    pub NULL): netif_queue_set_napi(netdev, 0, NETDEV_QUEUE_TYPE_TX,,
// kill manageability vlan ID if supported, but not if a vlan with
// the same ID is registered on the host OS (let 8021q kill it)
//
    if (adapter.hw.mng_cookie.status & E1000_MNG_DHCP_COOKIE_STATUS_VLAN)
    e1000_vlan_rx_kill_vid(netdev, htons(ETH_P_8021Q),
// If AMT is enabled, let the firmware know that the network
// interface is now closed
//
    if ((adapter.flags & FLAG_HAS_AMT) &&
    !test_bit(__E1000_TESTING, &adapter.state))
    pub 0: return,
    }
//
// e1000_set_mac - Change the Ethernet Address of the NIC
// @netdev: network interface device structure
// @p: pointer to an address structure
//
// Returns 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn e1000_set_mac(netdev: *mut net_device, p: *mut c_void) -> c_int {
    static int e1000_set_mac(struct net_device *netdev, void *p)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub p: *mut *mut sockaddr addr =,
    if (!is_valid_ether_addr(addr.sa_data))
    pub -EADDRNOTAVAIL: return,
    pub addr->sa_data): eth_hw_addr_set(netdev,,
    pub netdev->addr_len): memcpy(adapter->hw.mac.addr, addr->sa_data,,
    pub 0): hw->mac.ops.rar_set(&adapter->hw, adapter->hw.mac.addr,,
    if (adapter.flags & FLAG_RESET_OVERWRITES_LAA) {
// activate the work around
    pub 1): e1000e_set_laa_state_82571(&adapter->hw,,
// Hold a copy of the LAA in RAR[14] This is done so that
// between the time RAR[0] gets clobbered  and the time it
// gets fixed (in e1000_watchdog), the actual LAA is in one
// of the RARs and no incoming packets directed to this port
// are dropped. Eventually the LAA will be in RAR[0] and
// RAR[14]
//
    hw.mac.ops.rar_set(&adapter.hw, adapter.hw.mac.addr,
    pub 1): adapter->hw.mac.rar_entry_count -,
    }
    pub 0: return,
    }
//
// e1000e_update_phy_task - work thread to update phy
// @work: pointer to our work struct
//
// this worker thread exists because we must acquire a
// semaphore to read the phy, which we could msleep while
// waiting for it, and we can't msleep in a timer.
//
#[no_mangle]
unsafe extern "C" fn e1000e_update_phy_task(work: *mut work_struct) {
    static void e1000e_update_phy_task(struct work_struct *work)
    {
    struct e1000_adapter *adapter = container_of(work,
    struct e1000_adapter,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    if (test_bit(__E1000_DOWN, &adapter.state))
// Enable EEE on 82579 after link up
    if (hw.phy.type >= e1000_phy_82579)
    }
//
// e1000_update_phy_info - timre call-back to update PHY info
// @t: pointer to timer_list containing private info adapter
//
// Need to wait a few seconds after link up to get diagnostic information from
// the phy
//
#[no_mangle]
unsafe extern "C" fn e1000_update_phy_info(t: *mut timer_list) {
    static void e1000_update_phy_info(struct timer_list *t)
    {
    struct e1000_adapter *adapter = timer_container_of(adapter, t,
    if (test_bit(__E1000_DOWN, &adapter.state))
    }
//
// e1000e_update_phy_stats - Update the PHY statistics counters
// @adapter: board private structure
//
// Read/clear the upper 16-bit PHY registers and read/accumulate lower
//
#[no_mangle]
unsafe extern "C" fn e1000e_update_phy_stats(adapter: *mut e1000_adapter) {
    static void e1000e_update_phy_stats(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ret_val: i32,
    pub phy_data: u16,
    pub hw->phy.ops.acquire(hw): ret_val =,
    if (ret_val)
// A page set is expensive so check if already on desired page.
// If not, set to the page with the PHY status registers.
//
    pub 1: hw->phy.addr =,
    ret_val = e1000e_read_phy_reg_mdic(hw, IGP01E1000_PHY_PAGE_SELECT,
    if (ret_val)
    pub release: goto,
    if (phy_data != (HV_STATS_PAGE << IGP_PAGE_SHIFT)) {
    ret_val = hw.phy.ops.set_page(hw,
    pub IGP_PAGE_SHIFT): HV_STATS_PAGE <<,
    if (ret_val)
    pub release: goto,
    }
// Single Collision Count
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_SCC_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_SCC_LOWER,,
    if (!ret_val)
    pub phy_data: adapter->stats.scc +=,
// Excessive Collision Count
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_ECOL_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_ECOL_LOWER,,
    if (!ret_val)
    pub phy_data: adapter->stats.ecol +=,
// Multiple Collision Count
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_MCC_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_MCC_LOWER,,
    if (!ret_val)
    pub phy_data: adapter->stats.mcc +=,
// Late Collision Count
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_LATECOL_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_LATECOL_LOWER,,
    if (!ret_val)
    pub phy_data: adapter->stats.latecol +=,
// Collision Count - also used for adaptive IFS
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_COLC_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_COLC_LOWER,,
    if (!ret_val)
    pub phy_data: hw->mac.collision_delta =,
// Defer Count
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_DC_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_DC_LOWER,,
    if (!ret_val)
    pub phy_data: adapter->stats.dc +=,
// Transmit with no CRS
    pub &phy_data): hw->phy.ops.read_reg_page(hw, HV_TNCRS_UPPER,,
    pub &phy_data): ret_val = hw->phy.ops.read_reg_page(hw, HV_TNCRS_LOWER,,
    if (!ret_val)
    pub phy_data: adapter->stats.tncrs +=,
    release:
    }
//
// e1000e_update_stats - Update the board statistics counters
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn e1000e_update_stats(adapter: *mut e1000_adapter) {
    static void e1000e_update_stats(struct e1000_adapter *adapter)
    {
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
// Prevent stats update while adapter is being reset, or if the pci
// connection is down.
//
    if (adapter.link_speed == 0)
    if (pci_channel_offline(pdev))
    pub er32(CRCERRS): adapter->stats.crcerrs +=,
    pub er32(GPRC): adapter->stats.gprc +=,
    pub er32(GORCL): adapter->stats.gorc +=,
    pub /: *mut *mut er32(GORCH); / Clear gorc,
    pub er32(BPRC): adapter->stats.bprc +=,
    pub er32(MPRC): adapter->stats.mprc +=,
    pub er32(ROC): adapter->stats.roc +=,
    pub er32(MPC): adapter->stats.mpc +=,
// Half-duplex statistics
    if (adapter.link_duplex == HALF_DUPLEX) {
    if (adapter.flags2 & FLAG2_HAS_PHY_STATS) {
    } else {
    pub er32(SCC): adapter->stats.scc +=,
    pub er32(ECOL): adapter->stats.ecol +=,
    pub er32(MCC): adapter->stats.mcc +=,
    pub er32(LATECOL): adapter->stats.latecol +=,
    pub er32(DC): adapter->stats.dc +=,
    pub er32(COLC): hw->mac.collision_delta =,
    if ((hw.mac.type != e1000_82574) &&
    (hw.mac.type != e1000_82583))
    pub er32(TNCRS): adapter->stats.tncrs +=,
    }
    pub hw->mac.collision_delta: adapter->stats.colc +=,
    }
    pub er32(XONRXC): adapter->stats.xonrxc +=,
    pub er32(XONTXC): adapter->stats.xontxc +=,
    pub er32(XOFFRXC): adapter->stats.xoffrxc +=,
    pub er32(XOFFTXC): adapter->stats.xofftxc +=,
    pub er32(GPTC): adapter->stats.gptc +=,
    pub er32(GOTCL): adapter->stats.gotc +=,
    pub /: *mut *mut er32(GOTCH); / Clear gotc,
    pub er32(RNBC): adapter->stats.rnbc +=,
    pub er32(RUC): adapter->stats.ruc +=,
    pub er32(MPTC): adapter->stats.mptc +=,
    pub er32(BPTC): adapter->stats.bptc +=,
// used for adaptive IFS
    pub er32(TPT): hw->mac.tx_packet_delta =,
    pub hw->mac.tx_packet_delta: adapter->stats.tpt +=,
    pub er32(ALGNERRC): adapter->stats.algnerrc +=,
    pub er32(RXERRC): adapter->stats.rxerrc +=,
    pub er32(CEXTERR): adapter->stats.cexterr +=,
    pub er32(TSCTC): adapter->stats.tsctc +=,
    pub er32(TSCTFC): adapter->stats.tsctfc +=,
// Fill out the OS statistics structure
    pub adapter->stats.mprc: netdev->stats.multicast =,
    pub adapter->stats.colc: netdev->stats.collisions =,
// Rx Errors
// RLEC on some newer hardware can be incorrect so build
// our own version based on RUC and ROC
//
    netdev.stats.rx_errors = adapter.stats.rxerrc +
    adapter.stats.crcerrs + adapter.stats.algnerrc +
    pub adapter->stats.cexterr: adapter->stats.ruc + adapter->stats.roc +,
    netdev.stats.rx_length_errors = adapter.stats.ruc +
    pub adapter->stats.crcerrs: netdev->stats.rx_crc_errors =,
    pub adapter->stats.algnerrc: netdev->stats.rx_frame_errors =,
    pub adapter->stats.mpc: netdev->stats.rx_missed_errors =,
// Tx Errors
    pub adapter->stats.latecol: netdev->stats.tx_errors = adapter->stats.ecol +,
    pub adapter->stats.ecol: netdev->stats.tx_aborted_errors =,
    pub adapter->stats.latecol: netdev->stats.tx_window_errors =,
    pub adapter->stats.tncrs: netdev->stats.tx_carrier_errors =,
// Tx Dropped needs to be maintained elsewhere
// Management Stats
    pub er32(MGTPTC): adapter->stats.mgptc +=,
    pub er32(MGTPRC): adapter->stats.mgprc +=,
    pub er32(MGTPDC): adapter->stats.mgpdc +=,
// Correctable ECC Errors
    if (hw.mac.type >= e1000_pch_lpt) {
    pub er32(PBECCSTS): u32 pbeccsts =,
    adapter.corr_errors +=
    pub E1000_PBECCSTS_CORR_ERR_CNT_MASK: pbeccsts &,
    adapter.uncorr_errors +=
    pub pbeccsts): FIELD_GET(E1000_PBECCSTS_UNCORR_ERR_CNT_MASK,,
    }
    }
//
// e1000_phy_read_status - Update the PHY register status snapshot
// @adapter: board private structure
//
#[no_mangle]
unsafe extern "C" fn e1000_phy_read_status(adapter: *mut e1000_adapter) {
    static void e1000_phy_read_status(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub &adapter->phy_regs: *mut *mut e1000_phy_regs phy =,
    if (!pm_runtime_suspended((&adapter.pdev.dev).parent) &&
    (er32(STATUS) & E1000_STATUS_LU) &&
    (adapter.hw.phy.media_type == e1000_media_type_copper)) {
    pub ret_val: c_int,
    pub &phy->bmcr): ret_val = e1e_rphy(hw, MII_BMCR,,
    pub &phy->bmsr): ret_val |= e1e_rphy(hw, MII_BMSR,,
    pub &phy->advertise): ret_val |= e1e_rphy(hw, MII_ADVERTISE,,
    pub &phy->lpa): ret_val |= e1e_rphy(hw, MII_LPA,,
    pub &phy->expansion): ret_val |= e1e_rphy(hw, MII_EXPANSION,,
    pub &phy->ctrl1000): ret_val |= e1e_rphy(hw, MII_CTRL1000,,
    pub &phy->stat1000): ret_val |= e1e_rphy(hw, MII_STAT1000,,
    pub &phy->estatus): ret_val |= e1e_rphy(hw, MII_ESTATUS,,
    if (ret_val)
    pub register\n"): e_warn("Error reading PHY,
    } else {
// Do not read PHY registers if link is not up
// Set values to typical power-on defaults
//
    pub BMCR_FULLDPLX): phy->bmcr = (BMCR_SPEED1000 | BMCR_ANENABLE |,
    phy.bmsr = (BMSR_100FULL | BMSR_100HALF | BMSR_10FULL |
    BMSR_10HALF | BMSR_ESTATEN | BMSR_ANEGCAPABLE |
    phy.advertise = (ADVERTISE_PAUSE_ASYM | ADVERTISE_PAUSE_CAP |
    pub ADVERTISE_CSMA): ADVERTISE_ALL |,
    pub 0: phy->lpa =,
    pub EXPANSION_ENABLENPAGE: phy->expansion =,
    pub ADVERTISE_1000FULL: phy->ctrl1000 =,
    pub 0: phy->stat1000 =,
    pub ESTATUS_1000_THALF): phy->estatus = (ESTATUS_1000_TFULL |,
    }
    }
#[no_mangle]
unsafe extern "C" fn e1000_print_link_info(adapter: *mut e1000_adapter) {
    static void e1000_print_link_info(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(CTRL): u32 ctrl =,
// Link status message must follow this format for user tools
    netdev_info(adapter.netdev,
    "NIC Link is Up %d Mbps %s Duplex, Flow Control: %s\n",
    adapter.link_speed,
    adapter.link_duplex == FULL_DUPLEX ? "Full" : "Half",
    (ctrl & E1000_CTRL_TFCE) && (ctrl & E1000_CTRL_RFCE) ? "Rx/Tx" :
    (ctrl & E1000_CTRL_RFCE) ? "Rx" :
    pub "None"): (ctrl & E1000_CTRL_TFCE) ? "Tx" :,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_has_link(adapter: *mut e1000_adapter) -> bool {
    static bool e1000e_has_link(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub false: bool link_active =,
    pub 0: s32 ret_val =,
// get_link_status is set on LSC (link status) interrupt or
// Rx sequence error interrupt.  get_link_status will stay
// true until the check_for_link establishes link
// for copper adapters ONLY
//
    switch (hw.phy.media_type) {
    case e1000_media_type_copper:
    if (hw.mac.get_link_status) {
    pub hw->mac.ops.check_for_link(hw): ret_val =,
    pub !hw->mac.get_link_status: link_active =,
    } else {
    pub true: link_active =,
    }
    case e1000_media_type_fiber:
    pub hw->mac.ops.check_for_link(hw): ret_val =,
    pub E1000_STATUS_LU): link_active = !!(er32(STATUS) &,
    case e1000_media_type_internal_serdes:
    pub hw->mac.ops.check_for_link(hw): ret_val =,
    pub hw->mac.serdes_has_link: link_active =,
    default:
    case e1000_media_type_unknown:
    }
    if ((ret_val == -E1000_ERR_PHY) && (hw.phy.type == e1000_phy_igp_3) &&
    (er32(CTRL) & E1000_PHY_CTRL_GBE_DISABLE)) {
// See e1000_kmrn_lock_loss_workaround_ich8lan()
    pub speed\n"): e_info("Gigabit has been disabled, downgrading,
    }
    pub link_active: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_enable_receives(adapter: *mut e1000_adapter) {
    static void e1000e_enable_receives(struct e1000_adapter *adapter)
    {
// make sure the receive unit is started
    if ((adapter.flags & FLAG_RX_NEEDS_RESTART) &&
    (adapter.flags & FLAG_RESTART_NOW)) {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub er32(RCTL): u32 rctl =,
    pub E1000_RCTL_EN): ew32(RCTL, rctl |,
    pub ~FLAG_RESTART_NOW: adapter->flags &=,
    }
    }
#[no_mangle]
unsafe extern "C" fn e1000e_check_82574_phy_workaround(adapter: *mut e1000_adapter) {
    static void e1000e_check_82574_phy_workaround(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
// With 82574 controllers, PHY needs to be checked periodically
// for hung state and reset, if two calls return true
//
    if (e1000_check_phy_82574(hw))
    else
    pub 0: adapter->phy_hang_count =,
    if (adapter.phy_hang_count > 1) {
    pub 0: adapter->phy_hang_count =,
    pub resetting\n"): e_dbg("PHY appears hung -,
    }
    }
//
// e1000_watchdog - Timer Call-back
// @t: pointer to timer_list containing private info adapter
//
#[no_mangle]
unsafe extern "C" fn e1000_watchdog(t: *mut timer_list) {
    static void e1000_watchdog(struct timer_list *t)
    {
    struct e1000_adapter *adapter = timer_container_of(adapter, t,
// Do the rest outside of interrupt context
// TODO: make this use queue_delayed_work()
    }
#[no_mangle]
unsafe extern "C" fn e1000_watchdog_task(work: *mut work_struct) {
    static void e1000_watchdog_task(struct work_struct *work)
    {
    struct e1000_adapter *adapter = container_of(work,
    struct e1000_adapter,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub &adapter->hw.mac: *mut *mut e1000_mac_info mac =,
    pub &adapter->hw.phy: *mut *mut e1000_phy_info phy =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub 0: u32 dmoff_exit_timeout = 100, tries =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub pcim_state: u32 link, tctl,,
    if (test_bit(__E1000_DOWN, &adapter.state))
    pub e1000e_has_link(adapter): link =,
    if ((netif_carrier_ok(netdev)) && link) {
// Cancel scheduled suspend requests.
    pub link_up: goto,
    }
    if ((e1000e_enable_tx_pkt_filtering(hw)) &&
    (adapter.mng_vlan_id != adapter.hw.mng_cookie.vlan_id))
    if (link) {
    if (!netif_carrier_ok(netdev)) {
    pub true: bool txb2b =,
// Cancel scheduled suspend requests.
// Checking if MAC is in DMoff state
    if (er32(FWSM) & E1000_ICH_FWSM_FW_VALID) {
    pub er32(STATUS): pcim_state =,
    while (pcim_state & E1000_STATUS_PCIM_STATE) {
    if (tries++ == dmoff_exit_timeout) {
    pub dmoff\n"): e_dbg("Error in exiting,
    }
    pub 20000): usleep_range(10000,,
    pub er32(STATUS): pcim_state =,
// Checking if MAC exited DMoff state
    if (!(pcim_state & E1000_STATUS_PCIM_STATE))
    }
    }
// update snapshot of PHY registers on LSC
    mac.ops.get_link_up_info(&adapter.hw,
    &adapter.link_speed,
// check if SmartSpeed worked
    if (phy.speed_downgraded)
    netdev_warn(netdev,
    pub SmartSpeed\n"): "Link Speed was downgraded by,
// On supported PHYs, check for duplex mismatch only
// if link has autonegotiated at 10/100 half
//
    if ((hw.phy.type == e1000_phy_igp_3 ||
    hw.phy.type == e1000_phy_bm) &&
    hw.mac.autoneg &&
    (adapter.link_speed == SPEED_10 ||
    adapter.link_speed == SPEED_100) &&
    (adapter.link_duplex == HALF_DUPLEX)) {
    pub autoneg_exp: u16,
    pub &autoneg_exp): e1e_rphy(hw, MII_EXPANSION,,
    if (!(autoneg_exp & EXPANSION_NWAY))
    pub collisions.\n"): e_info("Autonegotiated half duplex but link partner cannot autoneg. Try forcing full duplex if link gets many,
    }
// adjust timeout factor according to speed/duplex
    pub 1: adapter->tx_timeout_factor =,
    switch (adapter.link_speed) {
    case SPEED_10:
    pub false: txb2b =,
    pub 16: adapter->tx_timeout_factor =,
    case SPEED_100:
    pub false: txb2b =,
    pub 10: adapter->tx_timeout_factor =,
    }
// workaround: re-program speed mode bit after
// link-up event
//
    if ((adapter.flags & FLAG_TARC_SPEED_MODE_BIT) &&
    !txb2b) {
    pub tarc0: u32,
    pub er32(TARC(0)): tarc0 =,
    pub ~SPEED_MODE_BIT: tarc0 &=,
    pub tarc0): ew32(TARC(0),,
    }
// enable transmits in the hardware, need to do this
// after setting TARC(0)
//
    pub er32(TCTL): tctl =,
    pub E1000_TCTL_EN: tctl |=,
    pub tctl): ew32(TCTL,,
// Perform any post-link-up configuration before
// reporting link up.
//
    if (phy.ops.cfg_on_link_up)
    if (!test_bit(__E1000_DOWN, &adapter.state))
    mod_timer(&adapter.phy_info_timer,
    pub HZ)): *mut *mut round_jiffies(jiffies + 2,
    }
    } else {
    if (netif_carrier_ok(netdev)) {
    pub 0: adapter->link_speed =,
    pub 0: adapter->link_duplex =,
// Link status message must follow this format
    pub Down\n"): netdev_info(netdev, "NIC Link is,
    if (!test_bit(__E1000_DOWN, &adapter.state))
    mod_timer(&adapter.phy_info_timer,
    pub HZ)): *mut *mut round_jiffies(jiffies + 2,
// 8000ES2LAN requires a Rx packet buffer work-around
// on link down event; reset the controller to flush
// the Rx packet buffer.
//
    if (adapter.flags & FLAG_RX_NEEDS_RESTART)
    pub FLAG_RESTART_NOW: adapter->flags |=,
    else
    pm_schedule_suspend(netdev.dev.parent,
    }
    }
    link_up:
    pub adapter->tpt_old: mac->tx_packet_delta = adapter->stats.tpt -,
    pub adapter->stats.tpt: adapter->tpt_old =,
    pub adapter->colc_old: mac->collision_delta = adapter->stats.colc -,
    pub adapter->stats.colc: adapter->colc_old =,
    pub adapter->gorc_old: adapter->gorc = adapter->stats.gorc -,
    pub adapter->stats.gorc: adapter->gorc_old =,
    pub adapter->gotc_old: adapter->gotc = adapter->stats.gotc -,
    pub adapter->stats.gotc: adapter->gotc_old =,
// If the link is lost the controller stops DMA, but
// if there is queued Tx work it cannot be done.  So
// reset the controller to flush the Tx packet buffers.
//
    if (!netif_carrier_ok(netdev) &&
    (e1000_desc_unused(tx_ring) + 1 < tx_ring.count))
    pub FLAG_RESTART_NOW: adapter->flags |=,
// If reset is necessary, do it outside of interrupt context.
    if (adapter.flags & FLAG_RESTART_NOW) {
// return immediately since reset is imminent
    }
// Simple mode for Interrupt Throttle Rate (ITR)
    if (adapter.itr_setting == 4) {
// Symmetric Tx/Rx gets a reduced ITR=2000;
// Total asymmetrical Tx or Rx gets ITR=8000;
// everyone else is between 2000-8000.
//
    pub 10000: u32 goc = (adapter->gotc + adapter->gorc) /,
    u32 dif = (adapter.gotc > adapter.gorc ?
    adapter.gotc - adapter.gorc :
    pub 10000: adapter->gorc - adapter->gotc) /,
    pub 8000: *mut *mut u32 itr = goc > 0 ? (dif  6000 / goc + 2000) :,
    pub itr): e1000e_write_itr(adapter,,
    }
// Cause software interrupt to ensure Rx ring is cleaned
    if (adapter.msix_entries)
    pub adapter->rx_ring->ims_val): ew32(ICS,,
    else
    pub E1000_ICS_RXDMT0): ew32(ICS,,
// flush pending descriptors to memory before detecting Tx hang
// Force detection of hung controller every watchdog period
    pub true: adapter->detect_tx_hung =,
// With 82571 controllers, LAA may be overwritten due to controller
// reset from the other port. Set the appropriate LAA in RAR[0]
//
    if (e1000e_get_laa_state_82571(hw))
    pub 0): hw->mac.ops.rar_set(hw, adapter->hw.mac.addr,,
    if (adapter.flags2 & FLAG2_CHECK_PHY_HANG)
// Clear valid timestamp stuck in RXSTMPL/H due to a Rx error
    if (adapter.hwtstamp_config.rx_filter != HWTSTAMP_FILTER_NONE) {
    if ((adapter.flags2 & FLAG2_CHECK_RX_HWTSTAMP) &&
    (er32(TSYNCRXCTL) & E1000_TSYNCRXCTL_VALID)) {
    } else {
    pub FLAG2_CHECK_RX_HWTSTAMP: adapter->flags2 |=,
    }
    }
// Reset the timer
    if (!test_bit(__E1000_DOWN, &adapter.state))
    mod_timer(&adapter.watchdog_timer,
    pub HZ)): *mut *mut round_jiffies(jiffies + 2,
    }
pub const E1000_TX_FLAGS_CSUM: c_uint = 0x00000001;
pub const E1000_TX_FLAGS_VLAN: c_uint = 0x00000002;
pub const E1000_TX_FLAGS_TSO: c_uint = 0x00000004;
pub const E1000_TX_FLAGS_IPV4: c_uint = 0x00000008;
pub const E1000_TX_FLAGS_NO_FCS: c_uint = 0x00000010;
pub const E1000_TX_FLAGS_HWTSTAMP: c_uint = 0x00000020;
pub const E1000_TX_FLAGS_VLAN_MASK: c_uint = 0xffff0000;
pub const E1000_TX_FLAGS_VLAN_SHIFT: c_int = 16;
    static int e1000_tso(struct e1000_ring *tx_ring, struct sk_buff *skb,
    __be16 protocol)
    {
    pub context_desc: *mut e1000_context_desc,
    pub buffer_info: *mut e1000_buffer,
    pub i: c_uint,
    pub 0: u32 cmd_length =,
    pub mss: u16 ipcse = 0,,
    pub hdr_len: u8 ipcss, ipcso, tucss, tucso,,
    pub err: c_int,
    if (!skb_is_gso(skb))
    pub 0: return,
    pub 0): err = skb_cow_head(skb,,
    if (err < 0)
    pub err: return,
    pub skb_tcp_all_headers(skb): hdr_len =,
    pub skb_shinfo(skb)->gso_size: mss =,
    if (protocol == htons(ETH_P_IP)) {
    pub ip_hdr(skb): *mut *mut iphdr iph =,
    pub 0: iph->tot_len =,
    pub 0: iph->check =,
    tcp_hdr(skb).check = ~csum_tcpudp_magic(iph.saddr, iph.daddr,
    pub 0): 0, IPPROTO_TCP,,
    pub E1000_TXD_CMD_IP: cmd_length =,
    pub 1: ipcse = skb_transport_offset(skb) -,
    } else if (skb_is_gso_v6(skb)) {
    pub 0: ipcse =,
    }
    pub skb_network_offset(skb): ipcss =,
    pub )skb->data: *mut *mut ipcso = (void )&(ip_hdr(skb)->check) - (void,
    pub skb_transport_offset(skb): tucss =,
    pub )skb->data: *mut *mut tucso = (void )&(tcp_hdr(skb)->check) - (void,
    cmd_length |= (E1000_TXD_CMD_DEXT | E1000_TXD_CMD_TSE |
    pub (hdr_len))): E1000_TXD_CMD_TCP | (skb->len -,
    pub tx_ring->next_to_use: i =,
    pub i): *mut *mut context_desc = E1000_CONTEXT_DESC(tx_ring,,
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub ipcss: context_desc->lower_setup.ip_fields.ipcss =,
    pub ipcso: context_desc->lower_setup.ip_fields.ipcso =,
    pub cpu_to_le16(ipcse): context_desc->lower_setup.ip_fields.ipcse =,
    pub tucss: context_desc->upper_setup.tcp_fields.tucss =,
    pub tucso: context_desc->upper_setup.tcp_fields.tucso =,
    pub 0: context_desc->upper_setup.tcp_fields.tucse =,
    pub cpu_to_le16(mss): context_desc->tcp_seg_setup.fields.mss =,
    pub hdr_len: context_desc->tcp_seg_setup.fields.hdr_len =,
    pub cpu_to_le32(cmd_length): context_desc->cmd_and_length =,
    pub jiffies: buffer_info->time_stamp =,
    pub i: buffer_info->next_to_watch =,
    if (i == tx_ring.count)
    pub 0: i =,
    pub i: tx_ring->next_to_use =,
    pub 1: return,
    }
    static bool e1000_tx_csum(struct e1000_ring *tx_ring, struct sk_buff *skb,
    __be16 protocol)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub context_desc: *mut e1000_context_desc,
    pub buffer_info: *mut e1000_buffer,
    pub i: c_uint,
    pub css: u8,
    pub E1000_TXD_CMD_DEXT: u32 cmd_len =,
    if (skb.ip_summed != CHECKSUM_PARTIAL)
    pub false: return,
    switch (protocol) {
    case cpu_to_be16(ETH_P_IP):
    if (ip_hdr(skb).protocol == IPPROTO_TCP)
    pub E1000_TXD_CMD_TCP: cmd_len |=,
    case cpu_to_be16(ETH_P_IPV6):
// XXX not handling all IPV6 headers
    if (ipv6_hdr(skb).nexthdr == IPPROTO_TCP)
    pub E1000_TXD_CMD_TCP: cmd_len |=,
    default:
    if (unlikely(net_ratelimit()))
    e_warn("checksum_partial proto=%x!\n",
    }
    pub skb_checksum_start_offset(skb): css =,
    pub tx_ring->next_to_use: i =,
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub i): *mut *mut context_desc = E1000_CONTEXT_DESC(tx_ring,,
    pub 0: context_desc->lower_setup.ip_config =,
    pub css: context_desc->upper_setup.tcp_fields.tucss =,
    pub skb->csum_offset: context_desc->upper_setup.tcp_fields.tucso = css +,
    pub 0: context_desc->upper_setup.tcp_fields.tucse =,
    pub 0: context_desc->tcp_seg_setup.data =,
    pub cpu_to_le32(cmd_len): context_desc->cmd_and_length =,
    pub jiffies: buffer_info->time_stamp =,
    pub i: buffer_info->next_to_watch =,
    if (i == tx_ring.count)
    pub 0: i =,
    pub i: tx_ring->next_to_use =,
    pub true: return,
    }
    static int e1000_tx_map(struct e1000_ring *tx_ring, struct sk_buff *skb,
    unsigned int first, unsigned int max_per_txd,
    unsigned int nr_frags)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub adapter->pdev: *mut *mut pci_dev pdev =,
    pub buffer_info: *mut e1000_buffer,
    pub skb_headlen(skb): unsigned int len =,
    pub i: unsigned int offset = 0, size, count = 0,,
    pub segs: unsigned int f, bytecount,,
    pub tx_ring->next_to_use: i =,
    while (len) {
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub max_per_txd): size = min(len,,
    pub size: buffer_info->length =,
    pub jiffies: buffer_info->time_stamp =,
    pub i: buffer_info->next_to_watch =,
    buffer_info.dma = dma_map_single(&pdev.dev,
    skb.data + offset,
    pub DMA_TO_DEVICE): size,,
    pub false: buffer_info->mapped_as_page =,
    if (dma_mapping_error(&pdev.dev, buffer_info.dma))
    pub dma_error: goto,
    pub size: len -=,
    pub size: offset +=,
    if (len) {
    if (i == tx_ring.count)
    pub 0: i =,
    }
    }
    pub {: for (f = 0; f < nr_frags; f++),
    pub &skb_shinfo(skb)->frags[f]: *const *const skb_frag_t frag =,
    pub skb_frag_size(frag): len =,
    pub 0: offset =,
    while (len) {
    if (i == tx_ring.count)
    pub 0: i =,
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub max_per_txd): size = min(len,,
    pub size: buffer_info->length =,
    pub jiffies: buffer_info->time_stamp =,
    pub i: buffer_info->next_to_watch =,
    buffer_info.dma = skb_frag_dma_map(&pdev.dev, frag,
    offset, size,
    pub true: buffer_info->mapped_as_page =,
    if (dma_mapping_error(&pdev.dev, buffer_info.dma))
    pub dma_error: goto,
    pub size: len -=,
    pub size: offset +=,
    }
    }
    pub 1: segs = skb_shinfo(skb)->gso_segs ? :,
// multiply data chunks by size of headers
    pub skb->len: *mut *mut bytecount = ((segs - 1)  skb_headlen(skb)) +,
    pub skb: tx_ring->buffer_info[i].skb =,
    pub segs: tx_ring->buffer_info[i].segs =,
    pub bytecount: tx_ring->buffer_info[i].bytecount =,
    pub i: tx_ring->buffer_info[first].next_to_watch =,
    pub count: return,
    dma_error:
    pub failed\n"): dev_err(&pdev->dev, "Tx DMA map,
    pub 0: buffer_info->dma =,
    while (count--) {
    if (i == 0)
    pub tx_ring->count: i +=,
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub true): e1000_put_txbuf(tx_ring, buffer_info,,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_tx_queue(tx_ring: *mut e1000_ring, tx_flags: c_int, count: c_int) {
    static void e1000_tx_queue(struct e1000_ring *tx_ring, int tx_flags, int count)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
    pub NULL: *mut *mut e1000_tx_desc tx_desc =,
    pub buffer_info: *mut e1000_buffer,
    pub E1000_TXD_CMD_IFCS: u32 txd_upper = 0, txd_lower =,
    pub i: c_uint,
    if (tx_flags & E1000_TX_FLAGS_TSO) {
    txd_lower |= E1000_TXD_CMD_DEXT | E1000_TXD_DTYP_D |
    pub 8: txd_upper |= E1000_TXD_POPTS_TXSM <<,
    if (tx_flags & E1000_TX_FLAGS_IPV4)
    pub 8: txd_upper |= E1000_TXD_POPTS_IXSM <<,
    }
    if (tx_flags & E1000_TX_FLAGS_CSUM) {
    pub E1000_TXD_DTYP_D: txd_lower |= E1000_TXD_CMD_DEXT |,
    pub 8: txd_upper |= E1000_TXD_POPTS_TXSM <<,
    }
    if (tx_flags & E1000_TX_FLAGS_VLAN) {
    pub E1000_TXD_CMD_VLE: txd_lower |=,
    pub E1000_TX_FLAGS_VLAN_MASK): txd_upper |= (tx_flags &,
    }
    if (unlikely(tx_flags & E1000_TX_FLAGS_NO_FCS))
    pub ~(E1000_TXD_CMD_IFCS): txd_lower &=,
    if (unlikely(tx_flags & E1000_TX_FLAGS_HWTSTAMP)) {
    pub E1000_TXD_DTYP_D: txd_lower |= E1000_TXD_CMD_DEXT |,
    pub E1000_TXD_EXTCMD_TSTAMP: txd_upper |=,
    }
    pub tx_ring->next_to_use: i =,
    do {
    pub &tx_ring->buffer_info[i]: buffer_info =,
    pub i): *mut *mut tx_desc = E1000_TX_DESC(tx_ring,,
    pub cpu_to_le64(buffer_info->dma): tx_desc->buffer_addr =,
    tx_desc.lower.data = cpu_to_le32(txd_lower |
    pub cpu_to_le32(txd_upper): tx_desc->upper.data =,
    if (i == tx_ring.count)
    pub 0: i =,
    pub 0): } while (--count >,
    pub cpu_to_le32(adapter->txd_cmd): tx_desc->lower.data |=,
// txd_cmd re-enables FCS, so we'll re-disable it here as desired.
    if (unlikely(tx_flags & E1000_TX_FLAGS_NO_FCS))
    pub ~(cpu_to_le32(E1000_TXD_CMD_IFCS)): tx_desc->lower.data &=,
// Force memory writes to complete before letting h/w
// know there are new descriptors to fetch.  (Only
// applicable for weak-ordered memory model archs,
// such as IA-64).
//
    pub i: tx_ring->next_to_use =,
    }
pub const MINIMUM_DHCP_PACKET_SIZE: c_int = 282;
    static int e1000_transfer_dhcp_info(struct e1000_adapter *adapter,
    struct sk_buff *skb)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub offset: u16 length,,
    if (skb_vlan_tag_present(skb) &&
    !((skb_vlan_tag_get(skb) == adapter.hw.mng_cookie.vlan_id) &&
    (adapter.hw.mng_cookie.status &
    E1000_MNG_DHCP_COOKIE_STATUS_VLAN)))
    pub 0: return,
    if (skb.len <= MINIMUM_DHCP_PACKET_SIZE)
    pub 0: return,
    if (((struct ethhdr *)skb.data).h_proto != htons(ETH_P_IP))
    pub 0: return,
    {
    pub 14): *const *const *const *const iphdr ip = (iphdr )((u8 )skb->data +,
    pub udp: *mut udphdr,
    if (ip.protocol != IPPROTO_UDP)
    pub 0: return,
    pub 2)): *mut *mut *mut udp = (struct udphdr )((u8 )ip + (ip->ihl <<,
    if (ntohs(udp.dest) != 67)
    pub 0: return,
    pub skb->data: *mut *mut offset = (u8 )udp + 8 -,
    pub offset: length = skb->len -,
    pub length): *mut *mut return e1000e_mng_write_dhcp_info(hw, (u8 )udp + 8,,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn __e1000_maybe_stop_tx(tx_ring: *mut e1000_ring, size: c_int) -> c_int {
    static int __e1000_maybe_stop_tx(struct e1000_ring *tx_ring, int size)
    {
    pub tx_ring->adapter: *mut *mut e1000_adapter adapter =,
// Herbert's original patch had:
// smp_mb__after_netif_stop_queue();
// but since that doesn't exist yet, just open code it.
//
// We need to check again in a case another CPU has just
// made room available.
//
    if (e1000_desc_unused(tx_ring) < size)
    pub -EBUSY: return,
// A reprieve!
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_maybe_stop_tx(tx_ring: *mut e1000_ring, size: c_int) -> c_int {
    static int e1000_maybe_stop_tx(struct e1000_ring *tx_ring, int size)
    {
    pub tx_ring->count): BUG_ON(size >,
    if (e1000_desc_unused(tx_ring) >= size)
    pub 0: return,
    pub size): return __e1000_maybe_stop_tx(tx_ring,,
    }
    static netdev_tx_t e1000_xmit_frame(struct sk_buff *skb,
    struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub adapter->tx_ring: *mut *mut e1000_ring tx_ring =,
    pub first: c_uint,
    pub 0: unsigned int tx_flags =,
    pub skb_headlen(skb): unsigned int len =,
    pub nr_frags: c_uint,
    pub mss: c_uint,
    pub 0: int count =,
    pub tso: c_int,
    pub f: c_uint,
    pub vlan_get_protocol(skb): __be16 protocol =,
    if (test_bit(__E1000_DOWN, &adapter.state)) {
    pub NETDEV_TX_OK: return,
    }
    if (skb.len <= 0) {
    pub NETDEV_TX_OK: return,
    }
// The minimum packet size with TCTL.PSP set is 17 bytes so
// pad skb in order to meet this minimum size requirement
//
    if (skb_put_padto(skb, 17))
    pub NETDEV_TX_OK: return,
    pub skb_shinfo(skb)->gso_size: mss =,
    if (mss) {
    pub hdr_len: u8,
// TSO Workaround for 82571/2/3 Controllers -- if skb->data
// points to just header, pull a few bytes of payload from
// frags into skb->data
//
    pub skb_tcp_all_headers(skb): hdr_len =,
// we do this workaround for ES2LAN, but it is un-necessary,
// avoiding it could save a lot of cycles
//
    if (skb.data_len && (hdr_len == len)) {
    pub pull_size: c_uint,
    pub skb->data_len): pull_size = min_t(unsigned int, 4,,
    if (!__pskb_pull_tail(skb, pull_size)) {
    pub failed.\n"): e_err("__pskb_pull_tail,
    pub NETDEV_TX_OK: return,
    }
    pub skb_headlen(skb): len =,
    }
    }
// reserve a descriptor for the offload context
    if ((mss) || (skb.ip_summed == CHECKSUM_PARTIAL))
    pub adapter->tx_fifo_limit): count += DIV_ROUND_UP(len,,
    pub skb_shinfo(skb)->nr_frags: nr_frags =,
    pub f++): for (f = 0; f < nr_frags;,
    count += DIV_ROUND_UP(skb_frag_size(&skb_shinfo(skb).frags[f]),
    if (adapter.hw.mac.tx_pkt_filtering)
    pub skb): e1000_transfer_dhcp_info(adapter,,
// need: count + 2 desc gap to keep tail from touching
// head, otherwise try next time
//
    if (e1000_maybe_stop_tx(tx_ring, count + 2))
    pub NETDEV_TX_BUSY: return,
    if (skb_vlan_tag_present(skb)) {
    pub E1000_TX_FLAGS_VLAN: tx_flags |=,
    tx_flags |= (skb_vlan_tag_get(skb) <<
    }
    pub tx_ring->next_to_use: first =,
    pub protocol): tso = e1000_tso(tx_ring, skb,,
    if (tso < 0) {
    pub NETDEV_TX_OK: return,
    }
    if (tso)
    pub E1000_TX_FLAGS_TSO: tx_flags |=,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: e1000_tx_csum(tx_ring, _arg: skb, _arg: protocol)) -> else {
    else if (e1000_tx_csum(tx_ring, skb, protocol))
    pub E1000_TX_FLAGS_CSUM: tx_flags |=,
// Old method was to assume IPv4 packet by default if TSO was enabled.
// 82571 hardware supports TSO capabilities for IPv6 as well...
// no longer assume, we must.
//
    if (protocol == htons(ETH_P_IP))
    pub E1000_TX_FLAGS_IPV4: tx_flags |=,
    if (unlikely(skb.no_fcs))
    pub E1000_TX_FLAGS_NO_FCS: tx_flags |=,
// if count is 0 then mapping error has occurred
    count = e1000_tx_map(tx_ring, skb, first, adapter.tx_fifo_limit,
    if (count) {
    if (unlikely(skb_shinfo(skb).tx_flags & SKBTX_HW_TSTAMP) &&
    (adapter.flags & FLAG_HAS_HW_TIMESTAMP)) {
    if (!adapter.tx_hwtstamp_skb) {
    pub SKBTX_IN_PROGRESS: skb_shinfo(skb)->tx_flags |=,
    pub E1000_TX_FLAGS_HWTSTAMP: tx_flags |=,
    pub skb_get(skb): adapter->tx_hwtstamp_skb =,
    pub jiffies: adapter->tx_hwtstamp_start =,
    } else {
    }
    }
    pub skb->len): netdev_sent_queue(netdev,,
    pub count): e1000_tx_queue(tx_ring, tx_flags,,
// Make sure there is space in the ring for the next send.
    e1000_maybe_stop_tx(tx_ring,
    ((MAX_SKB_FRAGS + 1) *
    DIV_ROUND_UP(PAGE_SIZE,
    pub 4)): adapter->tx_fifo_limit) +,
    if (!netdev_xmit_more() ||
    netif_xmit_stopped(netdev_get_tx_queue(netdev, 0))) {
    if (adapter.flags2 & FLAG2_PCIM2PCI_ARBITER_WA)
    e1000e_update_tdt_wa(tx_ring,
    else
    pub tx_ring->tail): writel(tx_ring->next_to_use,,
    }
    } else {
    pub 0: tx_ring->buffer_info[first].time_stamp =,
    pub first: tx_ring->next_to_use =,
    }
    pub NETDEV_TX_OK: return,
    }
//
// e1000_tx_timeout - Respond to a Tx Hang
// @netdev: network interface device structure
// @txqueue: index of the hung queue (unused)
//
#[no_mangle]
unsafe extern "C" fn e1000_tx_timeout(netdev: *mut net_device, txqueue: unsigned int __always_unused) {
    static void e1000_tx_timeout(struct net_device *netdev, unsigned int __always_unused txqueue)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
// Do the reset outside of interrupt context
    }
#[no_mangle]
unsafe extern "C" fn e1000_reset_task(work: *mut work_struct) {
    static void e1000_reset_task(struct work_struct *work)
    {
    pub adapter: *mut e1000_adapter,
    pub reset_task): adapter = container_of(work, struct e1000_adapter,,
// don't run the task if already down
    if (test_bit(__E1000_DOWN, &adapter.state)) {
    }
    if (!(adapter.flags & FLAG_RESTART_NOW)) {
    pub unexpectedly\n"): e_err("Reset adapter,
    }
    }
//
// e1000e_get_stats64 - Get System Network Statistics
// @netdev: network interface device structure
// @stats: rtnl_link_stats64 pointer
//
// Returns the address of the device statistics structure.
//
    void e1000e_get_stats64(struct net_device *netdev,
    struct rtnl_link_stats64 *stats)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
// Fill out the OS statistics structure
    pub adapter->stats.gorc: stats->rx_bytes =,
    pub adapter->stats.gprc: stats->rx_packets =,
    pub adapter->stats.gotc: stats->tx_bytes =,
    pub adapter->stats.gptc: stats->tx_packets =,
    pub adapter->stats.mprc: stats->multicast =,
    pub adapter->stats.colc: stats->collisions =,
// Rx Errors
// RLEC on some newer hardware can be incorrect so build
// our own version based on RUC and ROC
//
    stats.rx_errors = adapter.stats.rxerrc +
    adapter.stats.crcerrs + adapter.stats.algnerrc +
    pub adapter->stats.cexterr: adapter->stats.ruc + adapter->stats.roc +,
    pub adapter->stats.roc: stats->rx_length_errors = adapter->stats.ruc +,
    pub adapter->stats.crcerrs: stats->rx_crc_errors =,
    pub adapter->stats.algnerrc: stats->rx_frame_errors =,
    pub adapter->stats.mpc: stats->rx_missed_errors =,
// Tx Errors
    pub adapter->stats.latecol: stats->tx_errors = adapter->stats.ecol +,
    pub adapter->stats.ecol: stats->tx_aborted_errors =,
    pub adapter->stats.latecol: stats->tx_window_errors =,
    pub adapter->stats.tncrs: stats->tx_carrier_errors =,
// Tx Dropped needs to be maintained elsewhere
    }
//
// e1000_change_mtu - Change the Maximum Transfer Unit
// @netdev: network interface device structure
// @new_mtu: new value for maximum frame size
//
// Returns 0 on success, negative on failure
//
#[no_mangle]
unsafe extern "C" fn e1000_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int {
    static int e1000_change_mtu(struct net_device *netdev, int new_mtu)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub ETH_FCS_LEN: int max_frame = new_mtu + VLAN_ETH_HLEN +,
// Jumbo frame support
    if ((new_mtu > ETH_DATA_LEN) &&
    !(adapter.flags & FLAG_HAS_JUMBO_FRAMES)) {
    pub supported.\n"): e_err("Jumbo Frames not,
    pub -EINVAL: return,
    }
// Jumbo frame workaround on 82579 and newer requires CRC be stripped
    if ((adapter.hw.mac.type >= e1000_pch2lan) &&
    !(adapter.flags2 & FLAG2_CRC_STRIPPING) &&
    (new_mtu > ETH_DATA_LEN)) {
    pub disabled.\n"): e_err("Jumbo Frames not supported on this device when CRC stripping is,
    pub -EINVAL: return,
    }
    while (test_and_set_bit(__E1000_RESETTING, &adapter.state))
    pub 1100): usleep_range(1000,,
// e1000e_down -> e1000e_reset dependent on max_frame_size & mtu
    pub max_frame: adapter->max_frame_size =,
    netdev_dbg(netdev, "changing MTU from %d to %d\n",
    pub new_mtu): netdev->mtu,,
    pub new_mtu): WRITE_ONCE(netdev->mtu,,
    if (netif_running(netdev))
    pub true): e1000e_down(adapter,,
// NOTE: netdev_alloc_skb reserves 16 bytes, and typically NET_IP_ALIGN
// means we reserve 2 more, this pushes us to allocate from the next
// larger slab size.
// i.e. RXBUFFER_2048 --> size-4096 slab
// However with the new *_jumbo_rx* routines, jumbo receives will use
// fragmented skbs
//
    if (max_frame <= 2048)
    pub 2048: adapter->rx_buffer_len =,
    else
    pub 4096: adapter->rx_buffer_len =,
// adjust allocation if LPE protects us, and we aren't using SBP
    if (max_frame <= (VLAN_ETH_FRAME_LEN + ETH_FCS_LEN))
    pub ETH_FCS_LEN: adapter->rx_buffer_len = VLAN_ETH_FRAME_LEN +,
    if (netif_running(netdev))
    else
    pub &adapter->state): clear_bit(__E1000_RESETTING,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_ioctl(netdev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int {
    static int e1000_ioctl(struct net_device *netdev, struct ifreq *ifr, int cmd)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub if_mii(ifr): *mut *mut mii_ioctl_data data =,
    if (adapter.hw.phy.media_type != e1000_media_type_copper)
    pub -EOPNOTSUPP: return,
    switch (cmd) {
    case SIOCGMIIPHY:
    pub adapter->hw.phy.addr: data->phy_id =,
    case SIOCGMIIREG:
    switch (data.reg_num & 0x1F) {
    case MII_BMCR:
    pub adapter->phy_regs.bmcr: data->val_out =,
    case MII_BMSR:
    pub adapter->phy_regs.bmsr: data->val_out =,
    case MII_PHYSID1:
    pub 16): data->val_out = (adapter->hw.phy.id >>,
    case MII_PHYSID2:
    pub 0xFFFF): data->val_out = (adapter->hw.phy.id &,
    case MII_ADVERTISE:
    pub adapter->phy_regs.advertise: data->val_out =,
    case MII_LPA:
    pub adapter->phy_regs.lpa: data->val_out =,
    case MII_EXPANSION:
    pub adapter->phy_regs.expansion: data->val_out =,
    case MII_CTRL1000:
    pub adapter->phy_regs.ctrl1000: data->val_out =,
    case MII_STAT1000:
    pub adapter->phy_regs.stat1000: data->val_out =,
    case MII_ESTATUS:
    pub adapter->phy_regs.estatus: data->val_out =,
    default:
    pub -EIO: return,
    }
    case SIOCSMIIREG:
    default:
    pub -EOPNOTSUPP: return,
    }
    pub 0: return,
    }
//
// e1000e_hwtstamp_set - control hardware time stamping
// @netdev: network interface device structure
// @config: timestamp configuration
// @extack: netlink extended ACK report
//
// Outgoing time stamping can be enabled and disabled. Play nice and
// disable it when requested, although it shouldn't cause any overhead
// when no packet needs it. At most one packet in the queue may be
// marked for time stamping, otherwise it would be impossible to tell
// for sure to which packet the hardware time stamp belongs.
//
// Incoming time stamping has to be configured via the hardware filters.
// Not all combinations are supported, in particular event type has to be
// specified. Matching the kind of event packet is not supported, with the
// exception of "all V2 events regardless of level 2 or 4".
//
    static int e1000e_hwtstamp_set(struct net_device *netdev,
    struct kernel_hwtstamp_config *config,
    struct netlink_ext_ack *extack)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub ret_val: c_int,
    pub extack): ret_val = e1000e_config_hwtstamp(adapter, config,,
    if (ret_val)
    pub ret_val: return,
    switch (config.rx_filter) {
    case HWTSTAMP_FILTER_PTP_V2_L4_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_L2_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ:
    case HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ:
    case HWTSTAMP_FILTER_PTP_V2_DELAY_REQ:
// With V2 type filters which specify a Sync or Delay Request,
// Path Delay Request/Response messages are also time stamped
// by hardware so notify the caller the requested packets plus
// some others are time stamped.
//
    pub HWTSTAMP_FILTER_SOME: config->rx_filter =,
    default:
    }
    pub 0: return,
    }
    static int e1000e_hwtstamp_get(struct net_device *netdev,
    struct kernel_hwtstamp_config *kernel_config)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
// kernel_config = adapter->hwtstamp_config;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_init_phy_wakeup(adapter: *mut e1000_adapter, wufc: u32) -> c_int {
    static int e1000_init_phy_wakeup(struct e1000_adapter *adapter, u32 wufc)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub wuc: u32 i, mac_reg,,
    pub wuc_enable: u16 phy_reg,,
    pub retval: c_int,
// copy MAC RARs to PHY RARs
    pub hw->phy.ops.acquire(hw): retval =,
    if (retval) {
    pub PHY\n"): e_err("Could not acquire,
    pub retval: return,
    }
// Enable access to wakeup registers on and set page to BM_WUC_PAGE
    pub &wuc_enable): retval = e1000_enable_phy_wakeup_reg_access_bm(hw,,
    if (retval)
    pub release: goto,
// copy MAC MTA to PHY MTA - only needed for pchlan
    pub {: for (i = 0; i < adapter->hw.mac.mta_reg_count; i++),
    pub i): mac_reg = E1000_READ_REG_ARRAY(hw, E1000_MTA,,
    hw.phy.ops.write_reg_page(hw, BM_MTA(i),
    pub 0xFFFF)): (u16)(mac_reg &,
    hw.phy.ops.write_reg_page(hw, BM_MTA(i) + 1,
    pub 0xFFFF)): (u16)((mac_reg >> 16) &,
    }
// configure PHY Rx Control register
    pub &phy_reg): hw->phy.ops.read_reg_page(&adapter->hw, BM_RCTL,,
    pub er32(RCTL): mac_reg =,
    if (mac_reg & E1000_RCTL_UPE)
    pub BM_RCTL_UPE: phy_reg |=,
    if (mac_reg & E1000_RCTL_MPE)
    pub BM_RCTL_MPE: phy_reg |=,
    pub ~(BM_RCTL_MO_MASK): phy_reg &=,
    if (mac_reg & E1000_RCTL_MO_3)
    phy_reg |= (FIELD_GET(E1000_RCTL_MO_3, mac_reg)
    pub BM_RCTL_MO_SHIFT): <<,
    if (mac_reg & E1000_RCTL_BAM)
    pub BM_RCTL_BAM: phy_reg |=,
    if (mac_reg & E1000_RCTL_PMCF)
    pub BM_RCTL_PMCF: phy_reg |=,
    pub er32(CTRL): mac_reg =,
    if (mac_reg & E1000_CTRL_RFCE)
    pub BM_RCTL_RFCE: phy_reg |=,
    pub phy_reg): hw->phy.ops.write_reg_page(&adapter->hw, BM_RCTL,,
    pub E1000_WUC_PME_EN: wuc =,
    if (wufc & (E1000_WUFC_MAG | E1000_WUFC_LNKC))
    pub E1000_WUC_APME: wuc |=,
// enable PHY wakeup in MAC register
    pub wufc): ew32(WUFC,,
    ew32(WUC, (E1000_WUC_PHY_WAKE | E1000_WUC_APMPME |
    pub wuc)): E1000_WUC_PME_STATUS |,
// configure and enable PHY wakeup in PHY registers
    pub wufc): hw->phy.ops.write_reg_page(&adapter->hw, BM_WUFC,,
    pub wuc): hw->phy.ops.write_reg_page(&adapter->hw, BM_WUC,,
// activate PHY wakeup
    pub BM_WUC_HOST_WU_BIT: wuc_enable |= BM_WUC_ENABLE_BIT |,
    pub &wuc_enable): retval = e1000_disable_phy_wakeup_reg_access_bm(hw,,
    if (retval)
    pub bit\n"): e_err("Could not set PHY Host Wakeup,
    release:
    pub retval: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_flush_lpic(pdev: *mut pci_dev) {
    static void e1000e_flush_lpic(struct pci_dev *pdev)
    {
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ret_val: u32,
    pub hw->phy.ops.acquire(hw): ret_val =,
    if (ret_val)
    pub fl_out: goto,
    pr_info("EEE TX LPI TIMER: %08X\n",
    pub E1000_LPIC_LPIET_SHIFT): er32(LPIC) >>,
    fl_out:
    }
// S0ix implementation
#[no_mangle]
unsafe extern "C" fn e1000e_s0ix_entry_flow(adapter: *mut e1000_adapter) {
    static void e1000e_s0ix_entry_flow(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub mac_data: u32,
    pub phy_data: u16,
    if (er32(FWSM) & E1000_ICH_FWSM_FW_VALID &&
    hw.mac.type >= e1000_pch_adp) {
// Request ME configure the device for S0ix
    pub er32(H2ME): mac_data =,
    pub E1000_H2ME_START_DPG: mac_data |=,
    pub ~E1000_H2ME_EXIT_DPG: mac_data &=,
    pub mac_data): ew32(H2ME,,
    } else {
// Request driver configure the device to S0ix
// Disable the periodic inband message,
// don't request PCIe clock in K1 page770_17[10:9] = 10b
//
    pub &phy_data): e1e_rphy(hw, HV_PM_CTRL,,
    pub ~HV_PM_CTRL_K1_CLK_REQ: phy_data &=,
    pub BIT(10): phy_data |=,
    pub phy_data): e1e_wphy(hw, HV_PM_CTRL,,
// Make sure we don't exit K1 every time a new packet arrives
// 772_29[5] = 1 CS_Mode_Stay_In_K1
//
    pub &phy_data): e1e_rphy(hw, I217_CGFREG,,
    pub BIT(5): phy_data |=,
    pub phy_data): e1e_wphy(hw, I217_CGFREG,,
// Change the MAC/PHY interface to SMBus
// Force the SMBus in PHY page769_23[0] = 1
// Force the SMBus in MAC CTRL_EXT[11] = 1
//
    pub &phy_data): e1e_rphy(hw, CV_SMB_CTRL,,
    pub CV_SMB_CTRL_FORCE_SMBUS: phy_data |=,
    pub phy_data): e1e_wphy(hw, CV_SMB_CTRL,,
    pub er32(CTRL_EXT): mac_data =,
    pub E1000_CTRL_EXT_FORCE_SMBUS: mac_data |=,
    pub mac_data): ew32(CTRL_EXT,,
// DFT control: PHY bit: page769_20[0] = 1
// page769_20[7] - PHY PLL stop
// page769_20[8] - PHY go to the electrical idle
// page769_20[9] - PHY serdes disable
// Gate PPW via EXTCNF_CTRL - set 0x0F00[7] = 1
//
    pub &phy_data): e1e_rphy(hw, I82579_DFT_CTRL,,
    pub BIT(0): phy_data |=,
    pub BIT(7): phy_data |=,
    pub BIT(8): phy_data |=,
    pub BIT(9): phy_data |=,
    pub phy_data): e1e_wphy(hw, I82579_DFT_CTRL,,
    pub er32(EXTCNF_CTRL): mac_data =,
    pub E1000_EXTCNF_CTRL_GATE_PHY_CFG: mac_data |=,
    pub mac_data): ew32(EXTCNF_CTRL,,
// Disable disconnected cable conditioning for Power Gating
    pub er32(DPGFR): mac_data =,
    pub BIT(2): mac_data |=,
    pub mac_data): ew32(DPGFR,,
// Enable the Dynamic Clock Gating in the DMA and MAC
    pub er32(CTRL_EXT): mac_data =,
    pub E1000_CTRL_EXT_DMA_DYN_CLK_EN: mac_data |=,
    pub mac_data): ew32(CTRL_EXT,,
    }
// Enable the Dynamic Power Gating in the MAC
    pub er32(FEXTNVM7): mac_data =,
    pub BIT(22): mac_data |=,
    pub mac_data): ew32(FEXTNVM7,,
// Don't wake from dynamic Power Gating with clock request
    pub er32(FEXTNVM12): mac_data =,
    pub BIT(12): mac_data |=,
    pub mac_data): ew32(FEXTNVM12,,
// Ungate PGCB clock
    pub er32(FEXTNVM9): mac_data =,
    pub ~BIT(28): mac_data &=,
    pub mac_data): ew32(FEXTNVM9,,
// Enable K1 off to enable mPHY Power Gating
    pub er32(FEXTNVM6): mac_data =,
    pub BIT(31): mac_data |=,
    pub mac_data): ew32(FEXTNVM6,,
// Enable mPHY power gating for any link and speed
    pub er32(FEXTNVM8): mac_data =,
    pub BIT(9): mac_data |=,
    pub mac_data): ew32(FEXTNVM8,,
// No MAC DPG gating SLP_S0 in modern standby
// Switch the logic of the lanphypc to use PMC counter
//
    pub er32(FEXTNVM5): mac_data =,
    pub BIT(7): mac_data |=,
    pub mac_data): ew32(FEXTNVM5,,
// Disable the time synchronization clock
    pub er32(FEXTNVM7): mac_data =,
    pub BIT(31): mac_data |=,
    pub ~BIT(0): mac_data &=,
    pub mac_data): ew32(FEXTNVM7,,
// Dynamic Power Gating Enable
    pub er32(CTRL_EXT): mac_data =,
    pub BIT(3): mac_data |=,
    pub mac_data): ew32(CTRL_EXT,,
// Check MAC Tx/Rx packet buffer pointers.
// Reset MAC Tx/Rx packet buffer pointers to suppress any
// pending traffic indication that would prevent power gating.
//
    pub er32(TDFH): mac_data =,
    if (mac_data)
    pub 0): ew32(TDFH,,
    pub er32(TDFT): mac_data =,
    if (mac_data)
    pub 0): ew32(TDFT,,
    pub er32(TDFHS): mac_data =,
    if (mac_data)
    pub 0): ew32(TDFHS,,
    pub er32(TDFTS): mac_data =,
    if (mac_data)
    pub 0): ew32(TDFTS,,
    pub er32(TDFPC): mac_data =,
    if (mac_data)
    pub 0): ew32(TDFPC,,
    pub er32(RDFH): mac_data =,
    if (mac_data)
    pub 0): ew32(RDFH,,
    pub er32(RDFT): mac_data =,
    if (mac_data)
    pub 0): ew32(RDFT,,
    pub er32(RDFHS): mac_data =,
    if (mac_data)
    pub 0): ew32(RDFHS,,
    pub er32(RDFTS): mac_data =,
    if (mac_data)
    pub 0): ew32(RDFTS,,
    pub er32(RDFPC): mac_data =,
    if (mac_data)
    pub 0): ew32(RDFPC,,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_s0ix_exit_flow(adapter: *mut e1000_adapter) {
    static void e1000e_s0ix_exit_flow(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub false: bool firmware_bug =,
    pub mac_data: u32,
    pub phy_data: u16,
    pub 0: u32 i =,
    if (er32(FWSM) & E1000_ICH_FWSM_FW_VALID &&
    hw.mac.type >= e1000_pch_adp) {
// Keep the GPT clock enabled for CSME
    pub er32(FEXTNVM): mac_data =,
    pub BIT(3): mac_data |=,
    pub mac_data): ew32(FEXTNVM,,
// Request ME unconfigure the device from S0ix
    pub er32(H2ME): mac_data =,
    pub ~E1000_H2ME_START_DPG: mac_data &=,
    pub E1000_H2ME_EXIT_DPG: mac_data |=,
    pub mac_data): ew32(H2ME,,
// Poll up to 2.5 seconds for ME to unconfigure DPG.
// If this takes more than 1 second, show a warning indicating a
// firmware bug
//
    while (!(er32(EXFWSM) & E1000_EXFWSM_DPG_EXIT_DONE)) {
    if (i > 100 && !firmware_bug)
    pub true: firmware_bug =,
    if (i++ == 250) {
    e_dbg("Timeout (firmware bug): %d msec\n",
    pub 10): *mut *mut i,
    }
    pub 11000): usleep_range(10000,,
    }
    if (firmware_bug)
    e_warn("DPG_EXIT_DONE took %d msec. This is a firmware bug\n",
    pub 10): *mut *mut i,
    else
    pub 10): *mut *mut e_dbg("DPG_EXIT_DONE cleared after %d msec\n", i,
    } else {
// Request driver unconfigure the device from S0ix
// Cancel disable disconnected cable conditioning
// for Power Gating
//
    pub er32(DPGFR): mac_data =,
    pub ~BIT(2): mac_data &=,
    pub mac_data): ew32(DPGFR,,
// Disable the Dynamic Clock Gating in the DMA and MAC
    pub er32(CTRL_EXT): mac_data =,
    pub 0xFFF7FFFF: mac_data &=,
    pub mac_data): ew32(CTRL_EXT,,
// Enable the periodic inband message,
// Request PCIe clock in K1 page770_17[10:9] =01b
//
    pub &phy_data): e1e_rphy(hw, HV_PM_CTRL,,
    pub 0xFBFF: phy_data &=,
    pub HV_PM_CTRL_K1_CLK_REQ: phy_data |=,
    pub phy_data): e1e_wphy(hw, HV_PM_CTRL,,
// Return back configuration
// 772_29[5] = 0 CS_Mode_Stay_In_K1
//
    pub &phy_data): e1e_rphy(hw, I217_CGFREG,,
    pub 0xFFDF: phy_data &=,
    pub phy_data): e1e_wphy(hw, I217_CGFREG,,
// Change the MAC/PHY interface to Kumeran
// Unforce the SMBus in PHY page769_23[0] = 0
// Unforce the SMBus in MAC CTRL_EXT[11] = 0
//
    pub &phy_data): e1e_rphy(hw, CV_SMB_CTRL,,
    pub ~CV_SMB_CTRL_FORCE_SMBUS: phy_data &=,
    pub phy_data): e1e_wphy(hw, CV_SMB_CTRL,,
    pub er32(CTRL_EXT): mac_data =,
    pub ~E1000_CTRL_EXT_FORCE_SMBUS: mac_data &=,
    pub mac_data): ew32(CTRL_EXT,,
    }
// Disable Dynamic Power Gating
    pub er32(CTRL_EXT): mac_data =,
    pub 0xFFFFFFF7: mac_data &=,
    pub mac_data): ew32(CTRL_EXT,,
// Enable the time synchronization clock
    pub er32(FEXTNVM7): mac_data =,
    pub ~BIT(31): mac_data &=,
    pub BIT(0): mac_data |=,
    pub mac_data): ew32(FEXTNVM7,,
// Disable the Dynamic Power Gating in the MAC
    pub er32(FEXTNVM7): mac_data =,
    pub 0xFFBFFFFF: mac_data &=,
    pub mac_data): ew32(FEXTNVM7,,
// Disable mPHY power gating for any link and speed
    pub er32(FEXTNVM8): mac_data =,
    pub ~BIT(9): mac_data &=,
    pub mac_data): ew32(FEXTNVM8,,
// Disable K1 off
    pub er32(FEXTNVM6): mac_data =,
    pub ~BIT(31): mac_data &=,
    pub mac_data): ew32(FEXTNVM6,,
// Disable Ungate PGCB clock
    pub er32(FEXTNVM9): mac_data =,
    pub BIT(28): mac_data |=,
    pub mac_data): ew32(FEXTNVM9,,
// Cancel not waking from dynamic
// Power Gating with clock request
//
    pub er32(FEXTNVM12): mac_data =,
    pub ~BIT(12): mac_data &=,
    pub mac_data): ew32(FEXTNVM12,,
// Revert the lanphypc logic to use the internal Gbe counter
// and not the PMC counter
//
    pub er32(FEXTNVM5): mac_data =,
    pub 0xFFFFFF7F: mac_data &=,
    pub mac_data): ew32(FEXTNVM5,,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_freeze(dev: *mut device) -> c_int {
    static int e1000e_pm_freeze(struct device *dev)
    {
    pub dev_get_drvdata(dev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub present: bool,
    pub netif_device_present(netdev): present =,
    if (present && netif_running(netdev)) {
    pub E1000_CHECK_RESET_COUNT: int count =,
    while (test_bit(__E1000_RESETTING, &adapter.state) && count--)
    pub 11000): usleep_range(10000,,
    pub &adapter->state)): WARN_ON(test_bit(__E1000_RESETTING,,
// Quiesce the device without resetting the hardware
    pub false): e1000e_down(adapter,,
    }
// Allow time for pending master requests to run
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn __e1000_shutdown(pdev: *mut pci_dev, runtime: bool) -> c_int {
    static int __e1000_shutdown(struct pci_dev *pdev, bool runtime)
    {
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub wufc: u32 ctrl, ctrl_ext, rctl, status,,
    pub 0: int retval =,
// Runtime suspend should only enable wakeup for link changes
    if (runtime)
    pub E1000_WUFC_LNKC: wufc =,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: device_may_wakeup(&pdev->dev)) -> else {
    else if (device_may_wakeup(&pdev.dev))
    pub adapter->wol: wufc =,
    else
    pub 0: wufc =,
    pub er32(STATUS): status =,
    if (status & E1000_STATUS_LU)
    pub ~E1000_WUFC_LNKC: wufc &=,
    if (wufc) {
// turn on all-multi mode if wake on multicast is enabled
    if (wufc & E1000_WUFC_MC) {
    pub er32(RCTL): rctl =,
    pub E1000_RCTL_MPE: rctl |=,
    pub rctl): ew32(RCTL,,
    }
    pub er32(CTRL): ctrl =,
    pub E1000_CTRL_ADVD3WUC: ctrl |=,
    if (!(adapter.flags2 & FLAG2_HAS_PHY_WAKEUP))
    pub E1000_CTRL_EN_PHY_PWR_MGMT: ctrl |=,
    pub ctrl): ew32(CTRL,,
    if (adapter.hw.phy.media_type == e1000_media_type_fiber ||
    adapter.hw.phy.media_type ==
    e1000_media_type_internal_serdes) {
// keep the laser running in D3
    pub er32(CTRL_EXT): ctrl_ext =,
    pub E1000_CTRL_EXT_SDP3_DATA: ctrl_ext |=,
    pub ctrl_ext): ew32(CTRL_EXT,,
    }
    if (!runtime)
    if (adapter.flags & FLAG_IS_ICH)
    if (adapter.flags2 & FLAG2_HAS_PHY_WAKEUP) {
// enable wakeup by the PHY
    pub wufc): retval = e1000_init_phy_wakeup(adapter,,
    if (retval) {
    pub wakeup\n"): e_err("Failed to enable,
    pub skip_phy_configurations: goto,
    }
    } else {
// enable wakeup by the MAC
    pub wufc): ew32(WUFC,,
    pub E1000_WUC_PME_EN): ew32(WUC,,
    }
    } else {
    pub 0): ew32(WUC,,
    pub 0): ew32(WUFC,,
    }
    if (adapter.hw.phy.type == e1000_phy_igp_3) {
    } else if (hw.mac.type >= e1000_pch_lpt) {
    if (wufc && !(wufc & (E1000_WUFC_EX | E1000_WUFC_MC | E1000_WUFC_BC))) {
// ULP does not support wake from unicast, multicast
// or broadcast.
//
    pub !runtime): retval = e1000_enable_ulp_lpt_lp(hw,,
    if (retval) {
    pub ULP\n"): e_err("Failed to enable,
    pub skip_phy_configurations: goto,
    }
    }
    }
// Ensure that the appropriate bits are set in LPI_CTRL
// for EEE in Sx
//
    if ((hw.phy.type >= e1000_phy_i217) &&
    adapter.eee_advert && hw.dev_spec.ich8lan.eee_lp_ability) {
    pub 0: u16 lpi_ctrl =,
    pub hw->phy.ops.acquire(hw): retval =,
    if (!retval) {
    retval = e1e_rphy_locked(hw, I82579_LPI_CTRL,
    if (!retval) {
    if (adapter.eee_advert &
    hw.dev_spec.ich8lan.eee_lp_ability &
    I82579_EEE_100_SUPPORTED)
    pub I82579_LPI_CTRL_100_ENABLE: lpi_ctrl |=,
    if (adapter.eee_advert &
    hw.dev_spec.ich8lan.eee_lp_ability &
    I82579_EEE_1000_SUPPORTED)
    pub I82579_LPI_CTRL_1000_ENABLE: lpi_ctrl |=,
    retval = e1e_wphy_locked(hw, I82579_LPI_CTRL,
    }
    }
    }
    skip_phy_configurations:
// Release control of h/w to f/w.  If f/w is AMT enabled, this
// would have already happened in close and is redundant.
//
// The pci-e switch on some quad port adapters will report a
// correctable error when the MAC transitions from D0 to D3.  To
// prevent this we need to mask off the correctable errors on the
// downstream port of the pci-e switch.
//
// We don't have the associated upstream bridge while assigning
// the PCI device into guest. For example, the KVM on power is
// one of the cases.
//
    if (adapter.flags & FLAG_IS_QUAD_PORT) {
    pub pdev->bus->self: *mut *mut pci_dev us_dev =,
    pub devctl: u16,
    if (!us_dev)
    pub 0: return,
    pub &devctl): pcie_capability_read_word(us_dev, PCI_EXP_DEVCTL,,
    pcie_capability_write_word(us_dev, PCI_EXP_DEVCTL,
    pub ~PCI_EXP_DEVCTL_CERE)): (devctl &,
    pub devctl): pcie_capability_write_word(us_dev, PCI_EXP_DEVCTL,,
    }
    pub 0: return,
    }
//
// __e1000e_disable_aspm - Disable ASPM states
// @pdev: pointer to PCI device struct
// @state: bit-mask of ASPM states to disable
// @locked: indication if this context holds pci_bus_sem locked.
//
// Some devices *must* have certain ASPM states disabled per hardware errata.
//
#[no_mangle]
unsafe extern "C" fn __e1000e_disable_aspm(pdev: *mut pci_dev, state: u16, locked: c_int) {
    static void __e1000e_disable_aspm(struct pci_dev *pdev, u16 state, int locked)
    {
    pub pdev->bus->self: *mut *mut pci_dev parent =,
    pub 0: u16 aspm_dis_mask =,
    pub parent_aspmc: u16 pdev_aspmc,,
    switch (state) {
    case PCIE_LINK_STATE_L0S:
    case PCIE_LINK_STATE_L0S | PCIE_LINK_STATE_L1:
    pub PCI_EXP_LNKCTL_ASPM_L0S: aspm_dis_mask |=,
    pub /: *mut *mut fallthrough; / can't have L1 without L0s,
    case PCIE_LINK_STATE_L1:
    pub PCI_EXP_LNKCTL_ASPM_L1: aspm_dis_mask |=,
    default:
    }
    pub &pdev_aspmc): pcie_capability_read_word(pdev, PCI_EXP_LNKCTL,,
    pub PCI_EXP_LNKCTL_ASPMC: pdev_aspmc &=,
    if (parent) {
    pcie_capability_read_word(parent, PCI_EXP_LNKCTL,
    pub PCI_EXP_LNKCTL_ASPMC: parent_aspmc &=,
    }
// Nothing to do if the ASPM states to be disabled already are
    if (!(pdev_aspmc & aspm_dis_mask) &&
    (!parent || !(parent_aspmc & aspm_dis_mask)))
    dev_info(&pdev.dev, "Disabling ASPM %s %s\n",
    (aspm_dis_mask & pdev_aspmc & PCI_EXP_LNKCTL_ASPM_L0S) ?
    "L0s" : "",
    (aspm_dis_mask & pdev_aspmc & PCI_EXP_LNKCTL_ASPM_L1) ?
    pub ""): "L1" :,

    if (locked)
    pub state): pci_disable_link_state_locked(pdev,,
    else
    pub state): pci_disable_link_state(pdev,,
// Double-check ASPM control.  If not disabled by the above, the
// BIOS is preventing that from happening (or CONFIG_PCIEASPM is
// not enabled); override by writing PCI config space directly.
//
    pub &pdev_aspmc): pcie_capability_read_word(pdev, PCI_EXP_LNKCTL,,
    pub PCI_EXP_LNKCTL_ASPMC: pdev_aspmc &=,
    if (!(aspm_dis_mask & pdev_aspmc))

// Both device and parent should have the same ASPM setting.
// Disable ASPM in downstream component first and then upstream.
//
    pub aspm_dis_mask): pcie_capability_clear_word(pdev, PCI_EXP_LNKCTL,,
    if (parent)
    pcie_capability_clear_word(parent, PCI_EXP_LNKCTL,
    }
//
// e1000e_disable_aspm - Disable ASPM states.
// @pdev: pointer to PCI device struct
// @state: bit-mask of ASPM states to disable
//
// This function acquires the pci_bus_sem!
// Some devices *must* have certain ASPM states disabled per hardware errata.
//
#[no_mangle]
unsafe extern "C" fn e1000e_disable_aspm(pdev: *mut pci_dev, state: u16) {
    static void e1000e_disable_aspm(struct pci_dev *pdev, u16 state)
    {
    pub 0): __e1000e_disable_aspm(pdev, state,,
    }
//
// e1000e_disable_aspm_locked - Disable ASPM states.
// @pdev: pointer to PCI device struct
// @state: bit-mask of ASPM states to disable
//
// This function must be called with pci_bus_sem acquired!
// Some devices *must* have certain ASPM states disabled per hardware errata.
//
#[no_mangle]
unsafe extern "C" fn e1000e_disable_aspm_locked(pdev: *mut pci_dev, state: u16) {
    static void e1000e_disable_aspm_locked(struct pci_dev *pdev, u16 state)
    {
    pub 1): __e1000e_disable_aspm(pdev, state,,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_thaw(dev: *mut device) -> c_int {
    static int e1000e_pm_thaw(struct device *dev)
    {
    pub dev_get_drvdata(dev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub 0: int rc =,
    if (netif_running(netdev)) {
    pub e1000_request_irq(adapter): rc =,
    if (rc)
    pub err_irq: goto,
    }
    err_irq:
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn __e1000_resume(pdev: *mut pci_dev) -> c_int {
    static int __e1000_resume(struct pci_dev *pdev)
    {
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub 0: u16 aspm_disable_flag =,
    if (adapter.flags2 & FLAG2_DISABLE_ASPM_L0S)
    pub PCIE_LINK_STATE_L0S: aspm_disable_flag =,
    if (adapter.flags2 & FLAG2_DISABLE_ASPM_L1)
    pub PCIE_LINK_STATE_L1: aspm_disable_flag |=,
    if (aspm_disable_flag)
    pub aspm_disable_flag): e1000e_disable_aspm(pdev,,
    if (hw.mac.type >= e1000_pch2lan)
// report the system wakeup cause from S3/S4
    if (adapter.flags2 & FLAG2_HAS_PHY_WAKEUP) {
    pub phy_data: u16,
    pub &phy_data): e1e_rphy(&adapter->hw, BM_WUS,,
    if (phy_data) {
    e_info("PHY Wakeup cause - %s\n",
    phy_data & E1000_WUS_EX ? "Unicast Packet" :
    phy_data & E1000_WUS_MC ? "Multicast Packet" :
    phy_data & E1000_WUS_BC ? "Broadcast Packet" :
    phy_data & E1000_WUS_MAG ? "Magic Packet" :
    phy_data & E1000_WUS_LNKC ?
    pub "other"): "Link Status Change" :,
    }
    pub ~0): e1e_wphy(&adapter->hw, BM_WUS,,
    } else {
    pub er32(WUS): u32 wus =,
    if (wus) {
    e_info("MAC Wakeup cause - %s\n",
    wus & E1000_WUS_EX ? "Unicast Packet" :
    wus & E1000_WUS_MC ? "Multicast Packet" :
    wus & E1000_WUS_BC ? "Broadcast Packet" :
    wus & E1000_WUS_MAG ? "Magic Packet" :
    wus & E1000_WUS_LNKC ? "Link Status Change" :
    }
    pub ~0): ew32(WUS,,
    }
// If the controller has AMT, do not set DRV_LOAD until the interface
// is up.  For all other cases, let the f/w know that the h/w is now
// under the control of the driver.
//
    if (!(adapter.flags & FLAG_HAS_AMT))
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_prepare(dev: *mut device) -> c_int {
    static int e1000e_pm_prepare(struct device *dev)
    {
    return pm_runtime_suspended(dev) &&
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_suspend(dev: *mut device) -> c_int {
    static int e1000e_pm_suspend(struct device *dev)
    {
    pub pci_get_drvdata(to_pci_dev(dev)): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub to_pci_dev(dev): *mut *mut pci_dev pdev =,
    pub rc: c_int,
    pub false): rc = __e1000_shutdown(pdev,,
    if (!rc) {
// Introduce S0ix implementation
    if (adapter.flags2 & FLAG2_ENABLE_S0IX_FLOWS)
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_resume(dev: *mut device) -> c_int {
    static int e1000e_pm_resume(struct device *dev)
    {
    pub pci_get_drvdata(to_pci_dev(dev)): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub to_pci_dev(dev): *mut *mut pci_dev pdev =,
    pub rc: c_int,
// Introduce S0ix implementation
    if (adapter.flags2 & FLAG2_ENABLE_S0IX_FLOWS)
    pub __e1000_resume(pdev): rc =,
    if (rc)
    pub rc: return,
    pub e1000e_pm_thaw(dev): return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_runtime_idle(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int e1000e_pm_runtime_idle(struct device *dev)
    {
    pub dev_get_drvdata(dev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub eee_lp: u16,
    pub adapter->hw.dev_spec.ich8lan.eee_lp_ability: eee_lp =,
    if (!e1000e_has_link(adapter)) {
    pub eee_lp: adapter->hw.dev_spec.ich8lan.eee_lp_ability =,
    pub MSEC_PER_SEC): *mut *mut pm_schedule_suspend(dev, 5,
    }
    pub -EBUSY: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_runtime_resume(dev: *mut device) -> c_int {
    static int e1000e_pm_runtime_resume(struct device *dev)
    {
    pub to_pci_dev(dev): *mut *mut pci_dev pdev =,
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub rc: c_int,
    pub true: pdev->pme_poll =,
    pub __e1000_resume(pdev): rc =,
    if (rc)
    pub rc: return,
    if (netdev.flags & IFF_UP)
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000e_pm_runtime_suspend(dev: *mut device) -> c_int {
    static int e1000e_pm_runtime_suspend(struct device *dev)
    {
    pub to_pci_dev(dev): *mut *mut pci_dev pdev =,
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    if (netdev.flags & IFF_UP) {
    pub E1000_CHECK_RESET_COUNT: int count =,
    while (test_bit(__E1000_RESETTING, &adapter.state) && count--)
    pub 11000): usleep_range(10000,,
    pub &adapter->state)): WARN_ON(test_bit(__E1000_RESETTING,,
// Down the device without resetting the hardware
    pub false): e1000e_down(adapter,,
    }
    if (__e1000_shutdown(pdev, true)) {
    pub -EBUSY: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn e1000_shutdown(pdev: *mut pci_dev) {
    static void e1000_shutdown(struct pci_dev *pdev)
    {
    pub false): __e1000_shutdown(pdev,,
    }

#[no_mangle]
unsafe extern "C" fn e1000_intr_msix(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e1000_intr_msix(int __always_unused irq, void *data)
    {
    pub data: *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    if (adapter.msix_entries) {
    pub msix_irq: int vector,,
    pub 0: vector =,
    pub adapter->msix_entries[vector].vector: msix_irq =,
    if (disable_hardirq(msix_irq))
    pub netdev): e1000_intr_msix_rx(msix_irq,,
    pub adapter->msix_entries[vector].vector: msix_irq =,
    if (disable_hardirq(msix_irq))
    pub netdev): e1000_intr_msix_tx(msix_irq,,
    pub adapter->msix_entries[vector].vector: msix_irq =,
    if (disable_hardirq(msix_irq))
    pub netdev): e1000_msix_other(msix_irq,,
    }
    pub IRQ_HANDLED: return,
    }
//
// e1000_netpoll
// @netdev: network interface device structure
//
// Polling 'interrupt' - used by things like netconsole to send skbs
// without having to re-enable interrupts. It's not called while
// the interrupt routine is executing.
//
#[no_mangle]
unsafe extern "C" fn e1000_netpoll(netdev: *mut net_device) {
    static void e1000_netpoll(struct net_device *netdev)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    switch (adapter.int_mode) {
    case E1000E_INT_MODE_MSIX:
    pub netdev): e1000_intr_msix(adapter->pdev->irq,,
    case E1000E_INT_MODE_MSI:
    if (disable_hardirq(adapter.pdev.irq))
    pub netdev): e1000_intr_msi(adapter->pdev->irq,,
    default:		/* E1000E_INT_MODE_LEGACY */
    if (disable_hardirq(adapter.pdev.irq))
    pub netdev): e1000_intr(adapter->pdev->irq,,
    }
    }

//
// e1000_io_error_detected - called when PCI error is detected
// @pdev: Pointer to PCI device
// @state: The current pci connection state
//
// This function is called after a PCI bus error affecting
// this device has been detected.
//
    static pci_ers_result_t e1000_io_error_detected(struct pci_dev *pdev,
    pci_channel_state_t state)
    {
    if (state == pci_channel_io_perm_failure)
    pub PCI_ERS_RESULT_DISCONNECT: return,
// Request a slot reset.
    pub PCI_ERS_RESULT_NEED_RESET: return,
    }
//
// e1000_io_slot_reset - called after the pci bus has been reset.
// @pdev: Pointer to PCI device
//
// Restart the card from scratch, as if from a cold-boot. Implementation
// resembles the first-half of the e1000e_pm_resume routine.
//
#[no_mangle]
unsafe extern "C" fn e1000_io_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t e1000_io_slot_reset(struct pci_dev *pdev)
    {
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub 0: u16 aspm_disable_flag =,
    pub err: c_int,
    pub result: pci_ers_result_t,
    if (adapter.flags2 & FLAG2_DISABLE_ASPM_L0S)
    pub PCIE_LINK_STATE_L0S: aspm_disable_flag =,
    if (adapter.flags2 & FLAG2_DISABLE_ASPM_L1)
    pub PCIE_LINK_STATE_L1: aspm_disable_flag |=,
    if (aspm_disable_flag)
    pub aspm_disable_flag): e1000e_disable_aspm_locked(pdev,,
    pub pci_enable_device_mem(pdev): err =,
    if (err) {
    dev_err(&pdev.dev,
    pub reset.\n"): "Cannot re-enable PCI device after,
    pub PCI_ERS_RESULT_DISCONNECT: result =,
    } else {
    pub 0): pci_enable_wake(pdev, PCI_D3hot,,
    pub 0): pci_enable_wake(pdev, PCI_D3cold,,
    pub ~0): ew32(WUS,,
    pub PCI_ERS_RESULT_RECOVERED: result =,
    }
    pub result: return,
    }
//
// e1000_io_resume - called when traffic can start flowing again.
// @pdev: Pointer to PCI device
//
// This callback is called when the error recovery driver tells us that
// its OK to resume normal operation. Implementation resembles the
// second-half of the e1000e_pm_resume routine.
//
#[no_mangle]
unsafe extern "C" fn e1000_io_resume(pdev: *mut pci_dev) {
    static void e1000_io_resume(struct pci_dev *pdev)
    {
    pub pci_get_drvdata(pdev): *mut *mut net_device netdev =,
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
// If the controller has AMT, do not set DRV_LOAD until the interface
// is up.  For all other cases, let the f/w know that the h/w is now
// under the control of the driver.
//
    if (!(adapter.flags & FLAG_HAS_AMT))
    }
#[no_mangle]
unsafe extern "C" fn e1000_print_device_info(adapter: *mut e1000_adapter) {
    static void e1000_print_device_info(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub adapter->netdev: *mut *mut net_device netdev =,
    pub ret_val: u32,
    pub pba_str: [u8; E1000_PBANUM_LENGTH],
// print bus type/speed/width info
    e_info("(PCI Express:2.5GT/s:%s) %pM\n",
// bus width
    ((hw.bus.width == e1000_bus_width_pcie_x4) ? "Width x4" :
    "Width x1"),
// MAC address
    e_info("Intel(R) PRO/%s Network Connection\n",
    pub "1000"): (hw->phy.type == e1000_phy_ife) ? "10/100" :,
    ret_val = e1000_read_pba_string_generic(hw, pba_str,
    if (ret_val)
    pub sizeof(pba_str)): *mut *mut strscpy((char )pba_str, "Unknown",,
    e_info("MAC: %d, PHY: %d, PBA No: %s\n",
    pub pba_str): hw->mac.type, hw->phy.type,,
    }
#[no_mangle]
unsafe extern "C" fn e1000_eeprom_checks(adapter: *mut e1000_adapter) {
    static void e1000_eeprom_checks(struct e1000_adapter *adapter)
    {
    pub &adapter->hw: *mut *mut e1000_hw hw =,
    pub ret_val: c_int,
    pub 0: u16 buf =,
    if (hw.mac.type != e1000_82573)
    pub &buf): ret_val = e1000_read_nvm(hw, NVM_INIT_CONTROL2_REG, 1,,
    if (!ret_val && (!(buf & BIT(0)))) {
// Deep Smart Power Down (DSPD)
    dev_warn(&adapter.pdev.dev,
    pub EEPROM\n"): "Warning: detected DSPD enabled in,
    }
    }
    static netdev_features_t e1000_fix_features(struct net_device *netdev,
    netdev_features_t features)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub &adapter->hw: *mut *mut e1000_hw hw =,
// Jumbo frame workaround on 82579 and newer requires CRC be stripped
    if ((hw.mac.type >= e1000_pch2lan) && (netdev.mtu > ETH_DATA_LEN))
    pub ~NETIF_F_RXFCS: features &=,
// Since there is no support for separate Rx/Tx vlan accel
// enable/disable make sure Tx flag is always in same state as Rx.
//
    if (features & NETIF_F_HW_VLAN_CTAG_RX)
    pub NETIF_F_HW_VLAN_CTAG_TX: features |=,
    else
    pub ~NETIF_F_HW_VLAN_CTAG_TX: features &=,
    pub features: return,
    }
    static int e1000_set_features(struct net_device *netdev,
    netdev_features_t features)
    {
    pub netdev_priv(netdev): *mut *mut e1000_adapter adapter =,
    pub netdev->features: netdev_features_t changed = features ^,
    if (changed & (NETIF_F_TSO | NETIF_F_TSO6))
    pub FLAG_TSO_FORCE: adapter->flags |=,
    if (!(changed & (NETIF_F_HW_VLAN_CTAG_RX | NETIF_F_HW_VLAN_CTAG_TX |
    NETIF_F_RXCSUM | NETIF_F_RXHASH | NETIF_F_RXFCS |
    NETIF_F_RXALL)))
    pub 0: return,
    if (changed & NETIF_F_RXFCS) {
    if (features & NETIF_F_RXFCS) {
    pub ~FLAG2_CRC_STRIPPING: adapter->flags2 &=,
    } else {
// We need to take it back to defaults, which might mean
// stripping is still disabled at the adapter level.
//
    if (adapter.flags2 & FLAG2_DFLT_CRC_STRIPPING)
    pub FLAG2_CRC_STRIPPING: adapter->flags2 |=,
    else
    pub ~FLAG2_CRC_STRIPPING: adapter->flags2 &=,
    }
    }
    pub features: netdev->features =,
    if (netif_running(netdev))
    else
    pub 1: return,
    }
    static const struct net_device_ops e1000e_netdev_ops = {
    .ndo_open		= e1000e_open,
    .ndo_stop		= e1000e_close,
    .ndo_start_xmit		= e1000_xmit_frame,
    .ndo_get_stats64	= e1000e_get_stats64,
    .ndo_set_rx_mode	= e1000e_set_rx_mode,
    .ndo_set_mac_address	= e1000_set_mac,
    .ndo_change_mtu		= e1000_change_mtu,
    .ndo_eth_ioctl		= e1000_ioctl,
    .ndo_tx_timeout		= e1000_tx_timeout,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_vlan_rx_add_vid	= e1000_vlan_rx_add_vid,
    .ndo_vlan_rx_kill_vid	= e1000_vlan_rx_kill_vid,

    .ndo_poll_controller	= e1000_netpoll,

    .ndo_set_features	= e1000_set_features,
    .ndo_fix_features	= e1000_fix_features,
    .ndo_features_check	= passthru_features_check,
    .ndo_hwtstamp_get	= e1000e_hwtstamp_get,
    .ndo_hwtstamp_set	= e1000e_hwtstamp_set,
}

//
// e1000_probe - Device Initialization Routine
// @pdev: PCI device information struct
// @ent: entry in e1000_pci_tbl
//
// Returns 0 on success, negative on failure
//
// e1000_probe initializes an adapter identified by a pci_dev structure.
// The OS initialization, configuring of the adapter private structure,
// and a hardware reset occur.
//
#[no_mangle]
unsafe extern "C" fn e1000_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int e1000_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct net_device *netdev;
    struct e1000_adapter *adapter;
    struct e1000_hw *hw;
    const struct e1000_info *ei = e1000_info_tbl[ent.driver_data];
    resource_size_t mmio_start, mmio_len;
    resource_size_t flash_start, flash_len;
    static int cards_found;
    let mut aspm_disable_flag: u16 = 0;
    let mut eeprom_data: u16 = 0;
    let mut eeprom_apme_mask: u16 = E1000_EEPROM_APME;
    int bars, i, err;
    let mut ret_val: i32 = 0;
    if (ei.flags2 & FLAG2_DISABLE_ASPM_L0S)
    aspm_disable_flag = PCIE_LINK_STATE_L0S;
    if (ei.flags2 & FLAG2_DISABLE_ASPM_L1)
    aspm_disable_flag |= PCIE_LINK_STATE_L1;
    if (aspm_disable_flag)
    e1000e_disable_aspm(pdev, aspm_disable_flag);
    err = pci_enable_device_mem(pdev);
    if (err)
    return err;
    err = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (err) {
    dev_err(&pdev.dev,
    "No usable DMA configuration, aborting\n");
    goto err_dma;
    }
    bars = pci_select_bars(pdev, IORESOURCE_MEM);
    err = pci_request_selected_regions_exclusive(pdev, bars,
    e1000e_driver_name);
    if (err)
    goto err_pci_reg;
    pci_set_master(pdev);
// PCI config space info
    err = pci_save_state(pdev);
    if (err)
    goto err_alloc_etherdev;
    err = -ENOMEM;
    netdev = alloc_etherdev(sizeof(struct e1000_adapter));
    if (!netdev)
    goto err_alloc_etherdev;
    SET_NETDEV_DEV(netdev, &pdev.dev);
    netdev.irq = pdev.irq;
    pci_set_drvdata(pdev, netdev);
    adapter = netdev_priv(netdev);
    hw = &adapter.hw;
    adapter.netdev = netdev;
    adapter.pdev = pdev;
    adapter.ei = ei;
    adapter.pba = ei.pba;
    adapter.flags = ei.flags;
    adapter.flags2 = ei.flags2;
    adapter.hw.adapter = adapter;
    adapter.hw.mac.type = ei.mac;
    adapter.max_hw_frame_size = ei.max_hw_frame_size;
    adapter.msg_enable = netif_msg_init(debug, DEFAULT_MSG_ENABLE);
    mmio_start = pci_resource_start(pdev, 0);
    mmio_len = pci_resource_len(pdev, 0);
    err = -EIO;
    adapter.hw.hw_addr = ioremap(mmio_start, mmio_len);
    if (!adapter.hw.hw_addr)
    goto err_ioremap;
    if ((adapter.flags & FLAG_HAS_FLASH) &&
    (pci_resource_flags(pdev, 1) & IORESOURCE_MEM) &&
    (hw.mac.type < e1000_pch_spt)) {
    flash_start = pci_resource_start(pdev, 1);
    flash_len = pci_resource_len(pdev, 1);
    adapter.hw.flash_address = ioremap(flash_start, flash_len);
    if (!adapter.hw.flash_address)
    goto err_flashmap;
    }
// Set default EEE advertisement
    if (adapter.flags2 & FLAG2_HAS_EEE)
    adapter.eee_advert = MDIO_EEE_100TX | MDIO_EEE_1000T;
// construct the net_device struct
    netdev.netdev_ops = &e1000e_netdev_ops;
    e1000e_set_ethtool_ops(netdev);
    netdev.watchdog_timeo = 5 * HZ;
    netif_napi_add(netdev, &adapter.napi, e1000e_poll);
    strscpy(netdev.name, pci_name(pdev), sizeof(netdev.name));
    netdev.mem_start = mmio_start;
    netdev.mem_end = mmio_start + mmio_len;
    adapter.bd_number = cards_found++;
    e1000e_check_options(adapter);
// setup adapter struct
    err = e1000_sw_init(adapter);
    if (err)
    goto err_sw_init;
    memcpy(&hw.mac.ops, ei.mac_ops, sizeof(hw.mac.ops));
    memcpy(&hw.nvm.ops, ei.nvm_ops, sizeof(hw.nvm.ops));
    memcpy(&hw.phy.ops, ei.phy_ops, sizeof(hw.phy.ops));
    err = ei.get_variants(adapter);
    if (err)
    goto err_hw_init;
    if ((adapter.flags & FLAG_IS_ICH) &&
    (adapter.flags & FLAG_READ_ONLY_NVM) &&
    (hw.mac.type < e1000_pch_spt))
    e1000e_write_protect_nvm_ich8lan(&adapter.hw);
    hw.mac.ops.get_bus_info(&adapter.hw);
    adapter.hw.phy.autoneg_wait_to_complete = 0;
// Copper options
    if (adapter.hw.phy.media_type == e1000_media_type_copper) {
    adapter.hw.phy.mdix = AUTO_ALL_MODES;
    adapter.hw.phy.disable_polarity_correction = 0;
    adapter.hw.phy.ms_type = e1000_ms_hw_default;
    }
    if (hw.phy.ops.check_reset_block && hw.phy.ops.check_reset_block(hw))
    dev_info(&pdev.dev,
    "PHY reset is blocked due to SOL/IDER session.\n");
// Set initial default active device features
    netdev.features = (NETIF_F_SG |
    NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_HW_VLAN_CTAG_TX |
    NETIF_F_TSO |
    NETIF_F_TSO6 |
    NETIF_F_RXHASH |
    NETIF_F_RXCSUM |
    NETIF_F_HW_CSUM);
// disable TSO for pcie and 10/100 speeds to avoid
// some hardware issues and for i219 to fix transfer
// speed being capped at 60%
//
    if (!(adapter.flags & FLAG_TSO_FORCE)) {
    switch (adapter.link_speed) {
    case SPEED_10:
    case SPEED_100:
    e_info("10/100 speed: disabling TSO\n");
    netdev.features &= ~NETIF_F_TSO;
    netdev.features &= ~NETIF_F_TSO6;
    break;
    case SPEED_1000:
    netdev.features |= NETIF_F_TSO;
    netdev.features |= NETIF_F_TSO6;
    break;
    default:
// oops
    break;
    }
    if (hw.mac.type == e1000_pch_spt) {
    netdev.features &= ~NETIF_F_TSO;
    netdev.features &= ~NETIF_F_TSO6;
    }
    }
// Set user-changeable features (subset of all device features)
    netdev.hw_features = netdev.features;
    netdev.hw_features |= NETIF_F_RXFCS;
    netdev.priv_flags |= IFF_SUPP_NOFCS;
    netdev.hw_features |= NETIF_F_RXALL;
    if (adapter.flags & FLAG_HAS_HW_VLAN_FILTER)
    netdev.features |= NETIF_F_HW_VLAN_CTAG_FILTER;
    netdev.vlan_features |= (NETIF_F_SG |
    NETIF_F_TSO |
    NETIF_F_TSO6 |
    NETIF_F_HW_CSUM);
    netdev.priv_flags |= IFF_UNICAST_FLT;
    netdev.features |= NETIF_F_HIGHDMA;
    netdev.vlan_features |= NETIF_F_HIGHDMA;
// MTU range: 68 - max_hw_frame_size
    netdev.min_mtu = ETH_MIN_MTU;
    netdev.max_mtu = adapter.max_hw_frame_size -
    (VLAN_ETH_HLEN + ETH_FCS_LEN);
    if (e1000e_enable_mng_pass_thru(&adapter.hw))
    adapter.flags |= FLAG_MNG_PT_ENABLED;
// before reading the NVM, reset the controller to
// put the device in a known good starting state
//
    adapter.hw.mac.ops.reset_hw(&adapter.hw);
// systems with ASPM and others may see the checksum fail on the first
// attempt. Let's give it a few tries
//
    for (i = 0;; i++) {
    if (e1000_validate_nvm_checksum(&adapter.hw) >= 0)
    break;
    if (i == 2) {
    dev_err(&pdev.dev, "The NVM Checksum Is Not Valid\n");
    err = -EIO;
    goto err_eeprom;
    }
    }
    e1000_eeprom_checks(adapter);
// copy the MAC address
    if (e1000e_read_mac_addr(&adapter.hw))
    dev_err(&pdev.dev,
    "NVM Read Error while reading MAC address\n");
    eth_hw_addr_set(netdev, adapter.hw.mac.addr);
    if (!is_valid_ether_addr(netdev.dev_addr)) {
    dev_err(&pdev.dev, "Invalid MAC Address: %pM\n",
    netdev.dev_addr);
    err = -EIO;
    goto err_eeprom;
    }
    timer_setup(&adapter.watchdog_timer, e1000_watchdog, 0);
    timer_setup(&adapter.phy_info_timer, e1000_update_phy_info, 0);
    INIT_WORK(&adapter.reset_task, e1000_reset_task);
    INIT_WORK(&adapter.watchdog_task, e1000_watchdog_task);
    INIT_WORK(&adapter.downshift_task, e1000e_downshift_workaround);
    INIT_WORK(&adapter.update_phy_task, e1000e_update_phy_task);
    INIT_WORK(&adapter.print_hang_task, e1000_print_hw_hang);
// Initialize link parameters. User can change them with ethtool
    adapter.hw.mac.autoneg = 1;
    adapter.fc_autoneg = true;
    adapter.hw.fc.requested_mode = e1000_fc_default;
    adapter.hw.fc.current_mode = e1000_fc_default;
    adapter.hw.phy.autoneg_advertised = 0x2f;
// Initial Wake on LAN setting - If APM wake is enabled in
// the EEPROM, enable the ACPI Magic Packet filter
//
    if (adapter.flags & FLAG_APME_IN_WUC) {
// APME bit in EEPROM is mapped to WUC.APME
    eeprom_data = er32(WUC);
    eeprom_apme_mask = E1000_WUC_APME;
    if ((hw.mac.type > e1000_ich10lan) &&
    (eeprom_data & E1000_WUC_PHY_WAKE))
    adapter.flags2 |= FLAG2_HAS_PHY_WAKEUP;
    } else if (adapter.flags & FLAG_APME_IN_CTRL3) {
    if (adapter.flags & FLAG_APME_CHECK_PORT_B &&
    (adapter.hw.bus.func == 1))
    ret_val = e1000_read_nvm(&adapter.hw,
    NVM_INIT_CONTROL3_PORT_B,
    1, &eeprom_data);
    else
    ret_val = e1000_read_nvm(&adapter.hw,
    NVM_INIT_CONTROL3_PORT_A,
    1, &eeprom_data);
    }
// fetch WoL from EEPROM
    if (ret_val)
    e_dbg("NVM read error getting WoL initial values: %d\n", ret_val);
#[no_mangle]
pub unsafe extern "C" fn if(eeprom_apme_mask: eeprom_data &) -> else {
    else if (eeprom_data & eeprom_apme_mask)
    adapter.eeprom_wol |= E1000_WUFC_MAG;
// now that we have the eeprom settings, apply the special cases
// where the eeprom may be wrong or the board simply won't support
// wake on lan on a particular port
//
    if (!(adapter.flags & FLAG_HAS_WOL))
    adapter.eeprom_wol = 0;
// initialize the wol settings based on the eeprom settings
    adapter.wol = adapter.eeprom_wol;
// make sure adapter isn't asleep if manageability is enabled
    if (adapter.wol || (adapter.flags & FLAG_MNG_PT_ENABLED) ||
    (hw.mac.ops.check_mng_mode(hw)))
    device_wakeup_enable(&pdev.dev);
// save off EEPROM version number
    ret_val = e1000_read_nvm(&adapter.hw, 5, 1, &adapter.eeprom_vers);
    if (ret_val) {
    e_dbg("NVM read error getting EEPROM version: %d\n", ret_val);
    adapter.eeprom_vers = 0;
    }
// init PTP hardware clock
    e1000e_ptp_init(adapter);
// disable K1 by default on known problematic systems
    if (hw.mac.type >= e1000_pch_mtp && dmi_check_system(disable_k1_list))
    adapter.flags2 |= FLAG2_DISABLE_K1;
// reset the hardware with the new settings
    e1000e_reset(adapter);
// If the controller has AMT, do not set DRV_LOAD until the interface
// is up.  For all other cases, let the f/w know that the h/w is now
// under the control of the driver.
//
    if (!(adapter.flags & FLAG_HAS_AMT))
    e1000e_get_hw_control(adapter);
    if (hw.mac.type >= e1000_pch_cnp)
    adapter.flags2 |= FLAG2_ENABLE_S0IX_FLOWS;
    strscpy(netdev.name, "eth%d", sizeof(netdev.name));
    err = register_netdev(netdev);
    if (err)
    goto err_register;
// carrier off reporting is important to ethtool even BEFORE open
    netif_carrier_off(netdev);
    e1000_print_device_info(adapter);
    dev_pm_set_driver_flags(&pdev.dev, DPM_FLAG_SMART_PREPARE);
    if (pci_dev_run_wake(pdev))
    pm_runtime_put_noidle(&pdev.dev);
    return 0;
    err_register:
    if (!(adapter.flags & FLAG_HAS_AMT))
    e1000e_release_hw_control(adapter);
    e1000e_ptp_remove(adapter);
    err_eeprom:
    if (hw.phy.ops.check_reset_block && !hw.phy.ops.check_reset_block(hw))
    e1000_phy_hw_reset(&adapter.hw);
    err_hw_init:
    kfree(adapter.tx_ring);
    kfree(adapter.rx_ring);
    err_sw_init:
    if ((adapter.hw.flash_address) && (hw.mac.type < e1000_pch_spt))
    iounmap(adapter.hw.flash_address);
    e1000e_reset_interrupt_capability(adapter);
    err_flashmap:
    iounmap(adapter.hw.hw_addr);
    err_ioremap:
    free_netdev(netdev);
    err_alloc_etherdev:
    pci_release_mem_regions(pdev);
    err_pci_reg:
    err_dma:
    pci_disable_device(pdev);
    return err;
    }
//
// e1000_remove - Device Removal Routine
// @pdev: PCI device information struct
//
// e1000_remove is called by the PCI subsystem to alert the driver
// that it should release a PCI device.  This could be caused by a
// Hot-Plug event, or because the driver is going to be removed from
// memory.
//
#[no_mangle]
unsafe extern "C" fn e1000_remove(pdev: *mut pci_dev) {
    static void e1000_remove(struct pci_dev *pdev)
    {
    struct net_device *netdev = pci_get_drvdata(pdev);
    struct e1000_adapter *adapter = netdev_priv(netdev);
    e1000e_ptp_remove(adapter);
// The timers may be rescheduled, so explicitly disable them
// from being rescheduled.
//
    set_bit(__E1000_DOWN, &adapter.state);
    timer_delete_sync(&adapter.watchdog_timer);
    timer_delete_sync(&adapter.phy_info_timer);
    cancel_work_sync(&adapter.reset_task);
    cancel_work_sync(&adapter.watchdog_task);
    cancel_work_sync(&adapter.downshift_task);
    cancel_work_sync(&adapter.update_phy_task);
    cancel_work_sync(&adapter.print_hang_task);
    if (adapter.flags & FLAG_HAS_HW_TIMESTAMP) {
    cancel_work_sync(&adapter.tx_hwtstamp_work);
    if (adapter.tx_hwtstamp_skb) {
    dev_consume_skb_any(adapter.tx_hwtstamp_skb);
    adapter.tx_hwtstamp_skb = core::ptr::null_mut();
    }
    }
    unregister_netdev(netdev);
    if (pci_dev_run_wake(pdev))
    pm_runtime_get_noresume(&pdev.dev);
// Release control of h/w to f/w.  If f/w is AMT enabled, this
// would have already happened in close and is redundant.
//
    e1000e_release_hw_control(adapter);
    e1000e_reset_interrupt_capability(adapter);
    kfree(adapter.tx_ring);
    kfree(adapter.rx_ring);
    iounmap(adapter.hw.hw_addr);
    if ((adapter.hw.flash_address) &&
    (adapter.hw.mac.type < e1000_pch_spt))
    iounmap(adapter.hw.flash_address);
    pci_release_mem_regions(pdev);
    free_netdev(netdev);
    pci_disable_device(pdev);
    }
// PCI Error Recovery (ERS)
    static const struct pci_error_handlers e1000_err_handler = {
    .error_detected = e1000_io_error_detected,
    .slot_reset = e1000_io_slot_reset,
    .resume = e1000_io_resume,
    };
    static const struct pci_device_id e1000_pci_tbl[] = {
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_COPPER),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_FIBER),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_QUAD_COPPER),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_QUAD_COPPER_LP),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_QUAD_FIBER),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_SERDES),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_SERDES_DUAL),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571EB_SERDES_QUAD),
    .driver_data = board_82571,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82571PT_QUAD_COPPER),
    .driver_data = board_82571,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82572EI),
    .driver_data = board_82572,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82572EI_COPPER),
    .driver_data = board_82572,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82572EI_FIBER),
    .driver_data = board_82572,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82572EI_SERDES),
    .driver_data = board_82572,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82573E),
    .driver_data = board_82573,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82573E_IAMT),
    .driver_data = board_82573,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82573L),
    .driver_data = board_82573,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82574L),
    .driver_data = board_82574,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82574LA),
    .driver_data = board_82574,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_82583V),
    .driver_data = board_82583,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_80003ES2LAN_COPPER_DPT),
    .driver_data = board_80003es2lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_80003ES2LAN_COPPER_SPT),
    .driver_data = board_80003es2lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_80003ES2LAN_SERDES_DPT),
    .driver_data = board_80003es2lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_80003ES2LAN_SERDES_SPT),
    .driver_data = board_80003es2lan,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IFE),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IFE_G),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IFE_GT),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IGP_AMT),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IGP_C),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IGP_M),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_IGP_M_AMT),
    .driver_data = board_ich8lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH8_82567V_3),
    .driver_data = board_ich8lan,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IFE),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IFE_G),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IFE_GT),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IGP_AMT),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IGP_C),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_BM),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IGP_M),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IGP_M_AMT),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH9_IGP_M_V),
    .driver_data = board_ich9lan
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH10_R_BM_LM),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH10_R_BM_LF),
    .driver_data = board_ich9lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH10_R_BM_V),
    .driver_data = board_ich9lan,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH10_D_BM_LM),
    .driver_data = board_ich10lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH10_D_BM_LF),
    .driver_data = board_ich10lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_ICH10_D_BM_V),
    .driver_data = board_ich10lan,
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_M_HV_LM),
    .driver_data = board_pchlan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_M_HV_LC),
    .driver_data = board_pchlan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_D_HV_DM),
    .driver_data = board_pchlan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_D_HV_DC),
    .driver_data = board_pchlan
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH2_LV_LM),
    .driver_data = board_pch2lan,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH2_LV_V),
    .driver_data = board_pch2lan
    },
    {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LPT_I217_LM),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LPT_I217_V),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LPTLP_I218_LM),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LPTLP_I218_V),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_I218_LM2),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_I218_V2),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_I218_LM3),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_I218_V3),
    .driver_data = board_pch_lpt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_LM),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_V),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_LM2),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_V2),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LBG_I219_LM3),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_LM4),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_V4),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_LM5),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_SPT_I219_V5),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CNP_I219_LM6),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CNP_I219_V6),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CNP_I219_LM7),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CNP_I219_V7),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ICP_I219_LM8),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ICP_I219_V8),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ICP_I219_LM9),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ICP_I219_V9),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CMP_I219_LM10),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CMP_I219_V10),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CMP_I219_LM11),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CMP_I219_V11),
    .driver_data = board_pch_cnp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CMP_I219_LM12),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_CMP_I219_V12),
    .driver_data = board_pch_spt,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_TGP_I219_LM13),
    .driver_data = board_pch_tgp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_TGP_I219_V13),
    .driver_data = board_pch_tgp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_TGP_I219_LM14),
    .driver_data = board_pch_tgp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_TGP_I219_V14),
    .driver_data = board_pch_tgp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_TGP_I219_LM15),
    .driver_data = board_pch_tgp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_TGP_I219_V15),
    .driver_data = board_pch_tgp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_RPL_I219_LM23),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_RPL_I219_V23),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ADP_I219_LM16),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ADP_I219_V16),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ADP_I219_LM17),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ADP_I219_V17),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_RPL_I219_LM22),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_RPL_I219_V22),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ADP_I219_LM19),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ADP_I219_V19),
    .driver_data = board_pch_adp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_MTP_I219_LM18),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_MTP_I219_V18),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LNP_I219_LM20),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LNP_I219_V20),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LNP_I219_LM21),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_LNP_I219_V21),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ARL_I219_LM24),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_ARL_I219_V24),
    .driver_data = board_pch_mtp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_PTP_I219_LM25),
    .driver_data = board_pch_ptp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_PTP_I219_V25),
    .driver_data = board_pch_ptp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_PTP_I219_LM27),
    .driver_data = board_pch_ptp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_PTP_I219_V27),
    .driver_data = board_pch_ptp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_NVL_I219_LM29),
    .driver_data = board_pch_ptp,
    }, {
    PCI_VDEVICE(INTEL, E1000_DEV_ID_PCH_NVL_I219_V29),
    .driver_data = board_pch_ptp
    },
    { }	/* terminate list */
    };
    MODULE_DEVICE_TABLE(pci, e1000_pci_tbl);
    static const struct dev_pm_ops e1000e_pm_ops = {
    .prepare	= e1000e_pm_prepare,
    .suspend	= e1000e_pm_suspend,
    .resume		= e1000e_pm_resume,
    .freeze		= e1000e_pm_freeze,
    .thaw		= e1000e_pm_thaw,
    .poweroff	= e1000e_pm_suspend,
    .restore	= e1000e_pm_resume,
    RUNTIME_PM_OPS(e1000e_pm_runtime_suspend, e1000e_pm_runtime_resume,
    e1000e_pm_runtime_idle)
    };
// PCI Device API Driver
    static struct pci_driver e1000_driver = {
    .name     = e1000e_driver_name,
    .id_table = e1000_pci_tbl,
    .probe    = e1000_probe,
    .remove   = e1000_remove,
    .driver.pm = pm_ptr(&e1000e_pm_ops),
    .shutdown = e1000_shutdown,
    .err_handler = &e1000_err_handler
    };
//
// e1000_init_module - Driver Registration Routine
//
// e1000_init_module is the first routine called when the driver is
// loaded. All it does is register with the PCI subsystem.
//
#[no_mangle]
unsafe extern "C" fn e1000_init_module() -> int __init {
    static int __init e1000_init_module(void)
    {
    pr_info("Intel(R) PRO/1000 Network Driver\n");
    pr_info("Copyright(c) 1999 - 2015 Intel Corporation.\n");
    return pci_register_driver(&e1000_driver);
    }
    module_init(e1000_init_module);
//
// e1000_exit_module - Driver Exit Cleanup Routine
//
// e1000_exit_module is called just before the driver is removed
// from memory.
//
#[no_mangle]
unsafe extern "C" fn e1000_exit_module() -> void __exit {
    static void __exit e1000_exit_module(void)
    {
    pci_unregister_driver(&e1000_driver);
    }
    module_exit(e1000_exit_module);
    MODULE_DESCRIPTION("Intel(R) PRO/1000 Network Driver");
    MODULE_LICENSE("GPL v2");
// netdev.c
