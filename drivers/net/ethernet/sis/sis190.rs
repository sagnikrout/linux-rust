//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/sis/sis190.c
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


//
    sis190.c: Silicon Integrated Systems SiS190 ethernet driver
    Copyright (c) 2003 K.M. Liu <kmliu@sis.com>
    Copyright (c) 2003, 2004 Jeff Garzik <jgarzik@pobox.com>
    Copyright (c) 2003, 2004, 2005 Francois Romieu <romieu@fr.zoreil.com>
    Based on r8169.c, tg3.c, 8139cp.c, skge.c, epic100.c and SiS 190/191
    genuine driver.
    This software may be used and distributed according to the terms of
    the GNU General Public License (GPL), incorporated herein by reference.
    Drivers based on or derived from this code fall under the GPL and must
    retain the authorship, copyright and license notice.  This file is not
    a complete program and may only be used when the entire operating
    system is licensed under the GPL.
    See the file COPYING in this distribution for more information.
//

pub const PHY_MAX_ADDR: c_int = 32;
pub const PHY_ID_ANY: c_uint = 0x1f;
pub const MII_REG_ANY: c_uint = 0x1f;

pub const RX_BUF_SIZE: c_int = 1536;
pub const RX_BUF_MASK: c_uint = 0xfff8;
pub const SIS190_REGS_SIZE: c_uint = 0x80;

    NETIF_MSG_LINK | NETIF_MSG_IFUP | \
    NETIF_MSG_IFDOWN)
// Enhanced PHY access register bit definitions
pub const EhnMIIread: c_uint = 0x0000;
pub const EhnMIIwrite: c_uint = 0x0020;
pub const EhnMIIdataShift: c_int = 16;

pub const EhnMIIregShift: c_int = 11;
pub const EhnMIIreq: c_uint = 0x0010;
pub const EhnMIInotDone: c_uint = 0x0010;
// Write/read MMIO register

    enum sis190_registers {
    TxControl		= 0x00,
    TxDescStartAddr		= 0x04,
    rsv0			= 0x08,	// reserved
    TxSts			= 0x0c,	// unused (Control/Status)
    RxControl		= 0x10,
    RxDescStartAddr		= 0x14,
    rsv1			= 0x18,	// reserved
    RxSts			= 0x1c,	// unused
    IntrStatus		= 0x20,
    IntrMask		= 0x24,
    IntrControl		= 0x28,
    IntrTimer		= 0x2c,	// unused (Interrupt Timer)
    PMControl		= 0x30,	// unused (Power Mgmt Control/Status)
    rsv2			= 0x34,	// reserved
    ROMControl		= 0x38,
    ROMInterface		= 0x3c,
    StationControl		= 0x40,
    GMIIControl		= 0x44,
    GIoCR			= 0x48, // unused (GMAC IO Compensation)
    GIoCtrl			= 0x4c, // unused (GMAC IO Control)
    TxMacControl		= 0x50,
    TxLimit			= 0x54, // unused (Tx MAC Timer/TryLimit)
    RGDelay			= 0x58, // unused (RGMII Tx Internal Delay)
    rsv3			= 0x5c, // reserved
    RxMacControl		= 0x60,
    RxMacAddr		= 0x62,
    RxHashTable		= 0x68,
// Undocumented		= 0x6c,
    RxWolCtrl		= 0x70,
    RxWolData		= 0x74, // unused (Rx WOL Data Access)
    RxMPSControl		= 0x78,	// unused (Rx MPS Control)
    rsv4			= 0x7c, // reserved
    };
    enum sis190_register_content {
// IntrStatus
    SoftInt			= 0x40000000,	// unused
    Timeup			= 0x20000000,	// unused
    PauseFrame		= 0x00080000,	// unused
    MagicPacket		= 0x00040000,	// unused
    WakeupFrame		= 0x00020000,	// unused
    LinkChange		= 0x00010000,
    RxQEmpty		= 0x00000080,
    RxQInt			= 0x00000040,
    TxQ1Empty		= 0x00000020,	// unused
    TxQ1Int			= 0x00000010,
    TxQ0Empty		= 0x00000008,	// unused
    TxQ0Int			= 0x00000004,
    RxHalt			= 0x00000002,
    TxHalt			= 0x00000001,
// {Rx/Tx}CmdBits
    CmdReset		= 0x10,
    CmdRxEnb		= 0x08,		// unused
    CmdTxEnb		= 0x01,
    RxBufEmpty		= 0x01,		// unused
// Cfg9346Bits
    Cfg9346_Lock		= 0x00,		// unused
    Cfg9346_Unlock		= 0xc0,		// unused
// RxMacControl
    AcceptErr		= 0x20,		// unused
    AcceptRunt		= 0x10,		// unused
    AcceptBroadcast		= 0x0800,
    AcceptMulticast		= 0x0400,
    AcceptMyPhys		= 0x0200,
    AcceptAllPhys		= 0x0100,
// RxConfigBits
    RxCfgFIFOShift		= 13,
    RxCfgDMAShift		= 8,		// 0x1a in RxControl ?
// TxConfigBits
    TxInterFrameGapShift	= 24,
    TxDMAShift		= 8, /* DMA burst value (0-7) is shift this many bits */
    LinkStatus		= 0x02,		// unused
    FullDup			= 0x01,		// unused
// TBICSRBit
    TBILinkOK		= 0x02000000,	// unused
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TxDesc {
    pub PSize: __le32,
    pub status: __le32,
    pub addr: __le32,
    pub size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RxDesc {
    pub PSize: __le32,
    pub status: __le32,
    pub addr: __le32,
    pub size: __le32,
}

    enum _DescStatusBit {
// _Desc.status
    OWNbit		= 0x80000000, // RXOWN/TXOWN
    INTbit		= 0x40000000, // RXINT/TXINT
    CRCbit		= 0x00020000, // CRCOFF/CRCEN
    PADbit		= 0x00010000, // PREADD/PADEN
// _Desc.size
    RingEnd		= 0x80000000,
// TxDesc.status
    LSEN		= 0x08000000, // TSO ? -- FR
    IPCS		= 0x04000000,
    TCPCS		= 0x02000000,
    UDPCS		= 0x01000000,
    BSTEN		= 0x00800000,
    EXTEN		= 0x00400000,
    DEFEN		= 0x00200000,
    BKFEN		= 0x00100000,
    CRSEN		= 0x00080000,
    COLEN		= 0x00040000,
    THOL3		= 0x30000000,
    THOL2		= 0x20000000,
    THOL1		= 0x10000000,
    THOL0		= 0x00000000,
    WND		= 0x00080000,
    TABRT		= 0x00040000,
    FIFO		= 0x00020000,
    LINK		= 0x00010000,
    ColCountMask	= 0x0000ffff,
// RxDesc.status
    IPON		= 0x20000000,
    TCPON		= 0x10000000,
    UDPON		= 0x08000000,
    Wakup		= 0x00400000,
    Magic		= 0x00200000,
    Pause		= 0x00100000,
    DEFbit		= 0x00200000,
    BCAST		= 0x000c0000,
    MCAST		= 0x00080000,
    UCAST		= 0x00040000,
// RxDesc.PSize
    TAGON		= 0x80000000,
    RxDescCountMask	= 0x7f000000, // multi-desc pkt when > 1 ? -- FR
    ABORT		= 0x00800000,
    SHORT		= 0x00400000,
    LIMIT		= 0x00200000,
    MIIER		= 0x00100000,
    OVRUN		= 0x00080000,
    NIBON		= 0x00040000,
    COLON		= 0x00020000,
    CRCOK		= 0x00010000,
    RxSizeMask	= 0x0000ffff
//
// The asic could apparently do vlan, TSO, jumbo (sis191 only) and
// provide two (unused with Linux) Tx queues. No publicly
// available documentation alas.
//
    };
    enum sis190_eeprom_access_register_bits {
    EECS	= 0x00000001,	// unused
    EECLK	= 0x00000002,	// unused
    EEDO	= 0x00000008,	// unused
    EEDI	= 0x00000004,	// unused
    EEREQ	= 0x00000080,
    EEROP	= 0x00000200,
    EEWOP	= 0x00000100	// unused
    };
// EEPROM Addresses
    enum sis190_eeprom_address {
    EEPROMSignature	= 0x00,
    EEPROMCLK	= 0x01,	// unused
    EEPROMInfo	= 0x02,
    EEPROMMACAddr	= 0x03
    };
    enum sis190_feature {
    F_HAS_RGMII	= 1,
    F_PHY_88E1111	= 2,
    F_PHY_BCM5461	= 4
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sis190_private {
    pub mmio_addr: *mut void __iomem,
    pub pci_dev: *mut pci_dev,
    pub dev: *mut net_device,
    pub lock: spinlock_t,
    pub rx_buf_sz: u32,
    pub cur_rx: u32,
    pub cur_tx: u32,
    pub dirty_rx: u32,
    pub dirty_tx: u32,
    pub rx_dma: dma_addr_t,
    pub tx_dma: dma_addr_t,
    pub RxDescRing: *mut RxDesc,
    pub TxDescRing: *mut TxDesc,
    pub Rx_skbuff: [*mut sk_buff; NUM_RX_DESC],
    pub Tx_skbuff: [*mut sk_buff; NUM_TX_DESC],
    pub phy_task: work_struct,
    pub timer: timer_list,
    pub msg_enable: u32,
    pub mii_if: mii_if_info,
    pub first_phy: list_head,
    pub features: u32,
    pub negotiated_lpa: u32,
    enum {
    LNK_OFF,
    LNK_ON,
    LNK_AUTONEG,
    pub link_status: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sis190_phy {
    pub list: list_head,
    pub phy_id: c_int,
    pub id: [u16; 2],
    pub status: u16,
    pub type: u8,
}

    enum sis190_phy_type {
    UNKNOWN	= 0x00,
    HOME	= 0x01,
    LAN	= 0x02,
    MIX	= 0x03
    };
    static struct mii_chip_info {
    const char *name;
    u16 id[2];
    unsigned int type;
    u32 feature;
    } mii_chip_table[] = {
    { "Atheros PHY",          { 0x004d, 0xd010 }, LAN, 0 },
    { "Atheros PHY AR8012",   { 0x004d, 0xd020 }, LAN, 0 },
    { "Broadcom PHY BCM5461", { 0x0020, 0x60c0 }, LAN, F_PHY_BCM5461 },
    { "Broadcom PHY AC131",   { 0x0143, 0xbc70 }, LAN, 0 },
    { "Agere PHY ET1101B",    { 0x0282, 0xf010 }, LAN, 0 },
    { "Marvell PHY 88E1111",  { 0x0141, 0x0cc0 }, LAN, F_PHY_88E1111 },
    { "Realtek PHY RTL8201",  { 0x0000, 0x8200 }, LAN, 0 },
    { core::ptr::null_mut(), }
    };
    static const struct {
    const char *name;
    } sis_chip_info[] = {
    { "SiS 190 PCI Fast Ethernet adapter" },
    { "SiS 191 PCI Gigabit Ethernet adapter" },
    };
    static const struct pci_device_id sis190_pci_tbl[] = {
    { PCI_VDEVICE(SI, 0x0190), .driver_data = 0 },
    { PCI_VDEVICE(SI, 0x0191), .driver_data = 1 },
    { },
    };
    MODULE_DEVICE_TABLE(pci, sis190_pci_tbl);
    let mut rx_copybreak: static int = 200;
    static struct {
    u32 msg_enable;
    } debug = { -1 };
    MODULE_DESCRIPTION("SiS sis190/191 Gigabit Ethernet driver");
    module_param(rx_copybreak, int, 0);
    MODULE_PARM_DESC(rx_copybreak, "Copy breakpoint for copy-only-tiny-frames");
    module_param_named(debug, debug.msg_enable, int, 0);
    MODULE_PARM_DESC(debug, "Debug verbosity level (0=none, ..., 16=all)");
    MODULE_AUTHOR("K.M. Liu <kmliu@sis.com>, Ueimor <romieu@fr.zoreil.com>");
    MODULE_VERSION(DRV_VERSION);
    MODULE_LICENSE("GPL");
    static const u32 sis190_intr_mask =
    RxQEmpty | RxQInt | TxQ1Int | TxQ0Int | RxHalt | TxHalt | LinkChange;
//
// Maximum number of multicast addresses to filter (vs. Rx-all-multicast).
// The chips use a 64 element hash table based on the Ethernet CRC.
//
    let mut multicast_filter_limit: static int = 32;
#[no_mangle]
unsafe extern "C" fn __mdio_cmd(ioaddr: *mut void __iomem, ctl: u32) {
    static void __mdio_cmd(void __iomem *ioaddr, u32 ctl)
    {
    unsigned int i;
    SIS_W32(GMIIControl, ctl);
    msleep(1);
    for (i = 0; i < 100; i++) {
    if (!(SIS_R32(GMIIControl) & EhnMIInotDone))
    break;
    msleep(1);
    }
    if (i > 99)
    pr_err("PHY command failed !\n");
    }
#[no_mangle]
unsafe extern "C" fn mdio_write(ioaddr: *mut void __iomem, phy_id: c_int, reg: c_int, val: c_int) {
    static void mdio_write(void __iomem *ioaddr, int phy_id, int reg, int val)
    {
    __mdio_cmd(ioaddr, EhnMIIreq | EhnMIIwrite |
    (((u32) reg) << EhnMIIregShift) | (phy_id << EhnMIIpmdShift) |
    (((u32) val) << EhnMIIdataShift));
    }
#[no_mangle]
unsafe extern "C" fn mdio_read(ioaddr: *mut void __iomem, phy_id: c_int, reg: c_int) -> c_int {
    static int mdio_read(void __iomem *ioaddr, int phy_id, int reg)
    {
    __mdio_cmd(ioaddr, EhnMIIreq | EhnMIIread |
    (((u32) reg) << EhnMIIregShift) | (phy_id << EhnMIIpmdShift));
    return (u16) (SIS_R32(GMIIControl) >> EhnMIIdataShift);
    }
#[no_mangle]
unsafe extern "C" fn __mdio_write(dev: *mut net_device, phy_id: c_int, reg: c_int, val: c_int) {
    static void __mdio_write(struct net_device *dev, int phy_id, int reg, int val)
    {
    struct sis190_private *tp = netdev_priv(dev);
    mdio_write(tp.mmio_addr, phy_id, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn __mdio_read(dev: *mut net_device, phy_id: c_int, reg: c_int) -> c_int {
    static int __mdio_read(struct net_device *dev, int phy_id, int reg)
    {
    struct sis190_private *tp = netdev_priv(dev);
    return mdio_read(tp.mmio_addr, phy_id, reg);
    }
#[no_mangle]
unsafe extern "C" fn mdio_read_latched(ioaddr: *mut void __iomem, phy_id: c_int, reg: c_int) -> u16 {
    static u16 mdio_read_latched(void __iomem *ioaddr, int phy_id, int reg)
    {
    mdio_read(ioaddr, phy_id, reg);
    return mdio_read(ioaddr, phy_id, reg);
    }
#[no_mangle]
unsafe extern "C" fn sis190_read_eeprom(ioaddr: *mut void __iomem, reg: u32) -> u16 {
    static u16 sis190_read_eeprom(void __iomem *ioaddr, u32 reg)
    {
    let mut data: u16 = 0xffff;
    unsigned int i;
    if (!(SIS_R32(ROMControl) & 0x0002))
    return 0;
    SIS_W32(ROMInterface, EEREQ | EEROP | (reg << 10));
    for (i = 0; i < 200; i++) {
    if (!(SIS_R32(ROMInterface) & EEREQ)) {
    data = (SIS_R32(ROMInterface) & 0xffff0000) >> 16;
    break;
    }
    msleep(1);
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn sis190_irq_mask_and_ack(ioaddr: *mut void __iomem) {
    static void sis190_irq_mask_and_ack(void __iomem *ioaddr)
    {
    SIS_W32(IntrMask, 0x00);
    SIS_W32(IntrStatus, 0xffffffff);
    SIS_PCI_COMMIT();
    }
#[no_mangle]
unsafe extern "C" fn sis190_asic_down(ioaddr: *mut void __iomem) {
    static void sis190_asic_down(void __iomem *ioaddr)
    {
// Stop the chip's Tx and Rx DMA processes.
    SIS_W32(TxControl, 0x1a00);
    SIS_W32(RxControl, 0x1a00);
    sis190_irq_mask_and_ack(ioaddr);
    }
#[no_mangle]
unsafe extern "C" fn sis190_mark_as_last_descriptor(desc: *mut RxDesc) {
    static void sis190_mark_as_last_descriptor(struct RxDesc *desc)
    {
    desc.size |= cpu_to_le32(RingEnd);
    }
#[no_mangle]
pub unsafe extern "C" fn sis190_give_to_asic(desc: *mut RxDesc, rx_buf_sz: u32) {
    static inline void sis190_give_to_asic(struct RxDesc *desc, u32 rx_buf_sz)
    {
    let mut eor: u32 = le32_to_cpu(desc.size) & RingEnd;
    desc.PSize = 0x0;
    desc.size = cpu_to_le32((rx_buf_sz & RX_BUF_MASK) | eor);
    wmb();
    desc.status = cpu_to_le32(OWNbit | INTbit);
    }
    static inline void sis190_map_to_asic(struct RxDesc *desc, dma_addr_t mapping,
    u32 rx_buf_sz)
    {
    desc.addr = cpu_to_le32(mapping);
    sis190_give_to_asic(desc, rx_buf_sz);
    }
#[no_mangle]
pub unsafe extern "C" fn sis190_make_unusable_by_asic(desc: *mut RxDesc) {
    static inline void sis190_make_unusable_by_asic(struct RxDesc *desc)
    {
    desc.PSize = 0x0;
    desc.addr = cpu_to_le32(0xdeadbeef);
    desc.size &= cpu_to_le32(RingEnd);
    wmb();
    desc.status = 0x0;
    }
    static struct sk_buff *sis190_alloc_rx_skb(struct sis190_private *tp,
    struct RxDesc *desc)
    {
    let mut rx_buf_sz: u32 = tp.rx_buf_sz;
    struct sk_buff *skb;
    dma_addr_t mapping;
    skb = netdev_alloc_skb(tp.dev, rx_buf_sz);
    if (unlikely(!skb))
    goto skb_alloc_failed;
    mapping = dma_map_single(&tp.pci_dev.dev, skb.data, tp.rx_buf_sz,
    DMA_FROM_DEVICE);
    if (dma_mapping_error(&tp.pci_dev.dev, mapping))
    goto out;
    sis190_map_to_asic(desc, mapping, rx_buf_sz);
    return skb;
    out:
    dev_kfree_skb_any(skb);
    skb_alloc_failed:
    sis190_make_unusable_by_asic(desc);
    return core::ptr::null_mut();
    }
    static u32 sis190_rx_fill(struct sis190_private *tp, struct net_device *dev,
    u32 start, u32 end)
    {
    u32 cur;
    for (cur = start; cur < end; cur++) {
    let mut i: c_uint = cur % NUM_RX_DESC;
    if (tp.Rx_skbuff[i])
    continue;
    tp.Rx_skbuff[i] = sis190_alloc_rx_skb(tp, tp.RxDescRing + i);
    if (!tp.Rx_skbuff[i])
    break;
    }
    return cur - start;
    }
    static bool sis190_try_rx_copy(struct sis190_private *tp,
    struct sk_buff **sk_buff, int pkt_size,
    dma_addr_t addr)
    {
    struct sk_buff *skb;
    let mut done: bool = false;
    if (pkt_size >= rx_copybreak)
    goto out;
    skb = netdev_alloc_skb_ip_align(tp.dev, pkt_size);
    if (!skb)
    goto out;
    dma_sync_single_for_cpu(&tp.pci_dev.dev, addr, tp.rx_buf_sz,
    DMA_FROM_DEVICE);
    skb_copy_to_linear_data(skb, sk_buff[0].data, pkt_size);
// sk_buff = skb;
    done = true;
    out:
    return done;
    }
#[no_mangle]
pub unsafe extern "C" fn sis190_rx_pkt_err(status: u32, stats: *mut net_device_stats) -> c_int {
    static inline int sis190_rx_pkt_err(u32 status, struct net_device_stats *stats)
    {

    if ((status & CRCOK) && !(status & ErrMask))
    return 0;
    if (!(status & CRCOK))
    stats.rx_crc_errors++;
#[no_mangle]
pub unsafe extern "C" fn if(OVRUN: status &) -> else {
    else if (status & OVRUN)
    stats.rx_over_errors++;
#[no_mangle]
pub unsafe extern "C" fn if(LIMIT): status & (SHORT |) -> else {
    else if (status & (SHORT | LIMIT))
    stats.rx_length_errors++;
#[no_mangle]
pub unsafe extern "C" fn if(COLON): status & (MIIER | NIBON |) -> else {
    else if (status & (MIIER | NIBON | COLON))
    stats.rx_frame_errors++;
    stats.rx_errors++;
    return -1;
    }
    static int sis190_rx_interrupt(struct net_device *dev,
    struct sis190_private *tp, void __iomem *ioaddr)
    {
    struct net_device_stats *stats = &dev.stats;
    u32 rx_left, cur_rx = tp.cur_rx;
    u32 delta, count;
    rx_left = NUM_RX_DESC + tp.dirty_rx - cur_rx;
    rx_left = sis190_rx_quota(rx_left, (u32) dev.quota);
    for (; rx_left > 0; rx_left--, cur_rx++) {
    let mut entry: c_uint = cur_rx % NUM_RX_DESC;
    struct RxDesc *desc = tp.RxDescRing + entry;
    u32 status;
    if (le32_to_cpu(desc.status) & OWNbit)
    break;
    status = le32_to_cpu(desc.PSize);
// netif_info(tp, intr, dev, "Rx PSize = %08x\n", status);
    if (sis190_rx_pkt_err(status, stats) < 0)
    sis190_give_to_asic(desc, tp.rx_buf_sz);
    else {
    struct sk_buff *skb = tp.Rx_skbuff[entry];
    let mut addr: dma_addr_t = le32_to_cpu(desc.addr);
    let mut pkt_size: c_int = (status & RxSizeMask) - 4;
    struct pci_dev *pdev = tp.pci_dev;
    if (unlikely(pkt_size > tp.rx_buf_sz)) {
    netif_info(tp, intr, dev,
    "(frag) status = %08x\n", status);
    stats.rx_dropped++;
    stats.rx_length_errors++;
    sis190_give_to_asic(desc, tp.rx_buf_sz);
    continue;
    }
    if (sis190_try_rx_copy(tp, &skb, pkt_size, addr)) {
    dma_sync_single_for_device(&pdev.dev, addr,
    tp.rx_buf_sz,
    DMA_FROM_DEVICE);
    sis190_give_to_asic(desc, tp.rx_buf_sz);
    } else {
    dma_unmap_single(&pdev.dev, addr,
    tp.rx_buf_sz,
    DMA_FROM_DEVICE);
    tp.Rx_skbuff[entry] = core::ptr::null_mut();
    sis190_make_unusable_by_asic(desc);
    }
    skb_put(skb, pkt_size);
    skb.protocol = eth_type_trans(skb, dev);
    sis190_rx_skb(skb);
    stats.rx_packets++;
    stats.rx_bytes += pkt_size;
    if ((status & BCAST) == MCAST)
    stats.multicast++;
    }
    }
    count = cur_rx - tp.cur_rx;
    tp.cur_rx = cur_rx;
    delta = sis190_rx_fill(tp, dev, tp.dirty_rx, tp.cur_rx);
    if (!delta && count)
    netif_info(tp, intr, dev, "no Rx buffer allocated\n");
    tp.dirty_rx += delta;
    if ((tp.dirty_rx + NUM_RX_DESC) == tp.cur_rx)
    netif_emerg(tp, intr, dev, "Rx buffers exhausted\n");
    return count;
    }
    static void sis190_unmap_tx_skb(struct pci_dev *pdev, struct sk_buff *skb,
    struct TxDesc *desc)
    {
    unsigned int len;
    len = skb.len < ETH_ZLEN ? ETH_ZLEN : skb.len;
    dma_unmap_single(&pdev.dev, le32_to_cpu(desc.addr), len,
    DMA_TO_DEVICE);
    memset(desc, 0x00, sizeof(*desc));
    }
#[no_mangle]
pub unsafe extern "C" fn sis190_tx_pkt_err(status: u32, stats: *mut net_device_stats) -> c_int {
    static inline int sis190_tx_pkt_err(u32 status, struct net_device_stats *stats)
    {

    if (!unlikely(status & TxErrMask))
    return 0;
    if (status & WND)
    stats.tx_window_errors++;
    if (status & TABRT)
    stats.tx_aborted_errors++;
    if (status & FIFO)
    stats.tx_fifo_errors++;
    if (status & LINK)
    stats.tx_carrier_errors++;
    stats.tx_errors++;
    return -1;
    }
    static void sis190_tx_interrupt(struct net_device *dev,
    struct sis190_private *tp, void __iomem *ioaddr)
    {
    struct net_device_stats *stats = &dev.stats;
    u32 pending, dirty_tx = tp.dirty_tx;
//
// It would not be needed if queueing was allowed to be enabled
// again too early (hint: think preempt and unclocked smp systems).
//
    unsigned int queue_stopped;
    smp_rmb();
    pending = tp.cur_tx - dirty_tx;
    queue_stopped = (pending == NUM_TX_DESC);
    for (; pending; pending--, dirty_tx++) {
    let mut entry: c_uint = dirty_tx % NUM_TX_DESC;
    struct TxDesc *txd = tp.TxDescRing + entry;
    let mut status: u32 = le32_to_cpu(txd.status);
    struct sk_buff *skb;
    if (status & OWNbit)
    break;
    skb = tp.Tx_skbuff[entry];
    if (likely(sis190_tx_pkt_err(status, stats) == 0)) {
    stats.tx_packets++;
    stats.tx_bytes += skb.len;
    stats.collisions += ((status & ColCountMask) - 1);
    }
    sis190_unmap_tx_skb(tp.pci_dev, skb, txd);
    tp.Tx_skbuff[entry] = core::ptr::null_mut();
    dev_consume_skb_irq(skb);
    }
    if (tp.dirty_tx != dirty_tx) {
    tp.dirty_tx = dirty_tx;
    smp_wmb();
    if (queue_stopped)
    netif_wake_queue(dev);
    }
    }
//
// The interrupt handler does all of the Rx thread work and cleans up after
// the Tx thread.
//
#[no_mangle]
unsafe extern "C" fn sis190_irq(irq: c_int, __dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t sis190_irq(int irq, void *__dev)
    {
    struct net_device *dev = __dev;
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    let mut handled: c_uint = 0;
    u32 status;
    status = SIS_R32(IntrStatus);
    if ((status == 0xffffffff) || !status)
    goto out;
    handled = 1;
    if (unlikely(!netif_running(dev))) {
    sis190_asic_down(ioaddr);
    goto out;
    }
    SIS_W32(IntrStatus, status);
// netif_info(tp, intr, dev, "status = %08x\n", status);
    if (status & LinkChange) {
    netif_info(tp, intr, dev, "link change\n");
    timer_delete(&tp.timer);
    schedule_work(&tp.phy_task);
    }
    if (status & RxQInt)
    sis190_rx_interrupt(dev, tp, ioaddr);
    if (status & TxQ0Int)
    sis190_tx_interrupt(dev, tp, ioaddr);
    out:
    return IRQ_RETVAL(handled);
    }

#[no_mangle]
unsafe extern "C" fn sis190_netpoll(dev: *mut net_device) {
    static void sis190_netpoll(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    let mut irq: c_int = tp.pci_dev.irq;
    disable_irq(irq);
    sis190_irq(irq, dev);
    enable_irq(irq);
    }

    static void sis190_free_rx_skb(struct sis190_private *tp,
    struct sk_buff **sk_buff, struct RxDesc *desc)
    {
    struct pci_dev *pdev = tp.pci_dev;
    dma_unmap_single(&pdev.dev, le32_to_cpu(desc.addr), tp.rx_buf_sz,
    DMA_FROM_DEVICE);
    dev_kfree_skb(*sk_buff);
// sk_buff = NULL;
    sis190_make_unusable_by_asic(desc);
    }
#[no_mangle]
unsafe extern "C" fn sis190_rx_clear(tp: *mut sis190_private) {
    static void sis190_rx_clear(struct sis190_private *tp)
    {
    unsigned int i;
    for (i = 0; i < NUM_RX_DESC; i++) {
    if (!tp.Rx_skbuff[i])
    continue;
    sis190_free_rx_skb(tp, tp.Rx_skbuff + i, tp.RxDescRing + i);
    }
    }
#[no_mangle]
unsafe extern "C" fn sis190_init_ring_indexes(tp: *mut sis190_private) {
    static void sis190_init_ring_indexes(struct sis190_private *tp)
    {
    tp.dirty_tx = tp.dirty_rx = tp.cur_tx = tp.cur_rx = 0;
    }
#[no_mangle]
unsafe extern "C" fn sis190_init_ring(dev: *mut net_device) -> c_int {
    static int sis190_init_ring(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    sis190_init_ring_indexes(tp);
    memset(tp.Tx_skbuff, 0x0, NUM_TX_DESC * sizeof(struct sk_buff *));
    memset(tp.Rx_skbuff, 0x0, NUM_RX_DESC * sizeof(struct sk_buff *));
    if (sis190_rx_fill(tp, dev, 0, NUM_RX_DESC) != NUM_RX_DESC)
    goto err_rx_clear;
    sis190_mark_as_last_descriptor(tp.RxDescRing + NUM_RX_DESC - 1);
    return 0;
    err_rx_clear:
    sis190_rx_clear(tp);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn sis190_set_rx_mode(dev: *mut net_device) {
    static void sis190_set_rx_mode(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    unsigned long flags;
    u32 mc_filter[2];	/* Multicast hash filter */
    u16 rx_mode;
    if (dev.flags & IFF_PROMISC) {
    rx_mode =
    AcceptBroadcast | AcceptMulticast | AcceptMyPhys |
    AcceptAllPhys;
    mc_filter[1] = mc_filter[0] = 0xffffffff;
    } else if ((netdev_mc_count(dev) > multicast_filter_limit) ||
    (dev.flags & IFF_ALLMULTI)) {
// Too many to filter perfectly -- accept all multicasts.
    rx_mode = AcceptBroadcast | AcceptMulticast | AcceptMyPhys;
    mc_filter[1] = mc_filter[0] = 0xffffffff;
    } else {
    struct netdev_hw_addr *ha;
    rx_mode = AcceptBroadcast | AcceptMyPhys;
    mc_filter[1] = mc_filter[0] = 0;
    netdev_for_each_mc_addr(ha, dev) {
    int bit_nr =
    ether_crc(ETH_ALEN, ha.addr) & 0x3f;
    mc_filter[bit_nr >> 5] |= 1 << (bit_nr & 31);
    rx_mode |= AcceptMulticast;
    }
    }
    spin_lock_irqsave(&tp.lock, flags);
    SIS_W16(RxMacControl, rx_mode | 0x2);
    SIS_W32(RxHashTable, mc_filter[0]);
    SIS_W32(RxHashTable + 4, mc_filter[1]);
    spin_unlock_irqrestore(&tp.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sis190_soft_reset(ioaddr: *mut void __iomem) {
    static void sis190_soft_reset(void __iomem *ioaddr)
    {
    SIS_W32(IntrControl, 0x8000);
    SIS_PCI_COMMIT();
    SIS_W32(IntrControl, 0x0);
    sis190_asic_down(ioaddr);
    }
#[no_mangle]
unsafe extern "C" fn sis190_hw_start(dev: *mut net_device) {
    static void sis190_hw_start(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    sis190_soft_reset(ioaddr);
    SIS_W32(TxDescStartAddr, tp.tx_dma);
    SIS_W32(RxDescStartAddr, tp.rx_dma);
    SIS_W32(IntrStatus, 0xffffffff);
    SIS_W32(IntrMask, 0x0);
    SIS_W32(GMIIControl, 0x0);
    SIS_W32(TxMacControl, 0x60);
    SIS_W16(RxMacControl, 0x02);
    SIS_W32(RxHashTable, 0x0);
    SIS_W32(0x6c, 0x0);
    SIS_W32(RxWolCtrl, 0x0);
    SIS_W32(RxWolData, 0x0);
    SIS_PCI_COMMIT();
    sis190_set_rx_mode(dev);
// Enable all known interrupts by setting the interrupt mask.
    SIS_W32(IntrMask, sis190_intr_mask);
    SIS_W32(TxControl, 0x1a00 | CmdTxEnb);
    SIS_W32(RxControl, 0x1a1d);
    netif_start_queue(dev);
    }
#[no_mangle]
unsafe extern "C" fn sis190_phy_task(work: *mut work_struct) {
    static void sis190_phy_task(struct work_struct *work)
    {
    struct sis190_private *tp =
    container_of(work, struct sis190_private, phy_task);
    struct net_device *dev = tp.dev;
    void __iomem *ioaddr = tp.mmio_addr;
    let mut phy_id: c_int = tp.mii_if.phy_id;
    u16 val;
    rtnl_lock();
    if (!netif_running(dev))
    goto out_unlock;
    val = mdio_read(ioaddr, phy_id, MII_BMCR);
    if (val & BMCR_RESET) {
// FIXME: needlessly high ?  -- FR 02/07/2005
    mod_timer(&tp.timer, jiffies + HZ/10);
    goto out_unlock;
    }
    val = mdio_read_latched(ioaddr, phy_id, MII_BMSR);
    if (!(val & BMSR_ANEGCOMPLETE) && tp.link_status != LNK_AUTONEG) {
    netif_carrier_off(dev);
    netif_warn(tp, link, dev, "auto-negotiating...\n");
    tp.link_status = LNK_AUTONEG;
    } else if ((val & BMSR_LSTATUS) && tp.link_status != LNK_ON) {
// Rejoice !
    struct {
    int val;
    u32 ctl;
    const char *msg;
    } reg31[] = {
    { LPA_1000FULL, 0x07000c00 | 0x00001000,
    "1000 Mbps Full Duplex" },
    { LPA_1000HALF, 0x07000c00,
    "1000 Mbps Half Duplex" },
    { LPA_100FULL, 0x04000800 | 0x00001000,
    "100 Mbps Full Duplex" },
    { LPA_100HALF, 0x04000800,
    "100 Mbps Half Duplex" },
    { LPA_10FULL, 0x04000400 | 0x00001000,
    "10 Mbps Full Duplex" },
    { LPA_10HALF, 0x04000400,
    "10 Mbps Half Duplex" },
    { 0, 0x04000400, "unknown" }
    }, *p = core::ptr::null_mut();
    u16 adv, autoexp, gigadv, gigrec;
    val = mdio_read(ioaddr, phy_id, 0x1f);
    netif_info(tp, link, dev, "mii ext = %04x\n", val);
    val = mdio_read(ioaddr, phy_id, MII_LPA);
    adv = mdio_read(ioaddr, phy_id, MII_ADVERTISE);
    autoexp = mdio_read(ioaddr, phy_id, MII_EXPANSION);
    netif_info(tp, link, dev, "mii lpa=%04x adv=%04x exp=%04x\n",
    val, adv, autoexp);
    if (val & LPA_NPAGE && autoexp & EXPANSION_NWAY) {
// check for gigabit speed
    gigadv = mdio_read(ioaddr, phy_id, MII_CTRL1000);
    gigrec = mdio_read(ioaddr, phy_id, MII_STAT1000);
    val = (gigadv & (gigrec >> 2));
    if (val & ADVERTISE_1000FULL)
    p = reg31;
#[no_mangle]
pub unsafe extern "C" fn if(ADVERTISE_1000HALF: val &) -> else {
    else if (val & ADVERTISE_1000HALF)
    p = reg31 + 1;
    }
    if (!p) {
    val &= adv;
    for (p = reg31; p.val; p++) {
    if ((val & p.val) == p.val)
    break;
    }
    }
    p.ctl |= SIS_R32(StationControl) & ~0x0f001c00;
    if ((tp.features & F_HAS_RGMII) &&
    (tp.features & F_PHY_BCM5461)) {
// Set Tx Delay in RGMII mode.
    mdio_write(ioaddr, phy_id, 0x18, 0xf1c7);
    udelay(200);
    mdio_write(ioaddr, phy_id, 0x1c, 0x8c00);
    p.ctl |= 0x03000000;
    }
    SIS_W32(StationControl, p.ctl);
    if (tp.features & F_HAS_RGMII) {
    SIS_W32(RGDelay, 0x0441);
    SIS_W32(RGDelay, 0x0440);
    }
    tp.negotiated_lpa = p.val;
    netif_info(tp, link, dev, "link on %s mode\n", p.msg);
    netif_carrier_on(dev);
    tp.link_status = LNK_ON;
    } else if (!(val & BMSR_LSTATUS) && tp.link_status != LNK_AUTONEG)
    tp.link_status = LNK_OFF;
    mod_timer(&tp.timer, jiffies + SIS190_PHY_TIMEOUT);
    out_unlock:
    rtnl_unlock();
    }
#[no_mangle]
unsafe extern "C" fn sis190_phy_timer(t: *mut timer_list) {
    static void sis190_phy_timer(struct timer_list *t)
    {
    struct sis190_private *tp = timer_container_of(tp, t, timer);
    struct net_device *dev = tp.dev;
    if (likely(netif_running(dev)))
    schedule_work(&tp.phy_task);
    }
#[no_mangle]
pub unsafe extern "C" fn sis190_delete_timer(dev: *mut net_device) {
    static inline void sis190_delete_timer(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    timer_delete_sync(&tp.timer);
    }
#[no_mangle]
pub unsafe extern "C" fn sis190_request_timer(dev: *mut net_device) {
    static inline void sis190_request_timer(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    struct timer_list *timer = &tp.timer;
    timer_setup(timer, sis190_phy_timer, 0);
    timer.expires = jiffies + SIS190_PHY_TIMEOUT;
    add_timer(timer);
    }
    static void sis190_set_rxbufsize(struct sis190_private *tp,
    struct net_device *dev)
    {
    let mut mtu: c_uint = dev.mtu;
    tp.rx_buf_sz = (mtu > RX_BUF_SIZE) ? mtu + ETH_HLEN + 8 : RX_BUF_SIZE;
// RxDesc->size has a licence to kill the lower bits
    if (tp.rx_buf_sz & 0x07) {
    tp.rx_buf_sz += 8;
    tp.rx_buf_sz &= RX_BUF_MASK;
    }
    }
#[no_mangle]
unsafe extern "C" fn sis190_open(dev: *mut net_device) -> c_int {
    static int sis190_open(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    struct pci_dev *pdev = tp.pci_dev;
    let mut rc: c_int = -ENOMEM;
    sis190_set_rxbufsize(tp, dev);
//
// Rx and Tx descriptors need 256 bytes alignment.
// dma_alloc_coherent() guarantees a stronger alignment.
//
    tp.TxDescRing = dma_alloc_coherent(&pdev.dev, TX_RING_BYTES,
    &tp.tx_dma, GFP_KERNEL);
    if (!tp.TxDescRing)
    goto out;
    tp.RxDescRing = dma_alloc_coherent(&pdev.dev, RX_RING_BYTES,
    &tp.rx_dma, GFP_KERNEL);
    if (!tp.RxDescRing)
    goto err_free_tx_0;
    rc = sis190_init_ring(dev);
    if (rc < 0)
    goto err_free_rx_1;
    sis190_request_timer(dev);
    rc = request_irq(pdev.irq, sis190_irq, IRQF_SHARED, dev.name, dev);
    if (rc < 0)
    goto err_release_timer_2;
    sis190_hw_start(dev);
    out:
    return rc;
    err_release_timer_2:
    sis190_delete_timer(dev);
    sis190_rx_clear(tp);
    err_free_rx_1:
    dma_free_coherent(&pdev.dev, RX_RING_BYTES, tp.RxDescRing,
    tp.rx_dma);
    err_free_tx_0:
    dma_free_coherent(&pdev.dev, TX_RING_BYTES, tp.TxDescRing,
    tp.tx_dma);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn sis190_tx_clear(tp: *mut sis190_private) {
    static void sis190_tx_clear(struct sis190_private *tp)
    {
    unsigned int i;
    for (i = 0; i < NUM_TX_DESC; i++) {
    struct sk_buff *skb = tp.Tx_skbuff[i];
    if (!skb)
    continue;
    sis190_unmap_tx_skb(tp.pci_dev, skb, tp.TxDescRing + i);
    tp.Tx_skbuff[i] = core::ptr::null_mut();
    dev_kfree_skb(skb);
    tp.dev.stats.tx_dropped++;
    }
    tp.cur_tx = tp.dirty_tx = 0;
    }
#[no_mangle]
unsafe extern "C" fn sis190_down(dev: *mut net_device) {
    static void sis190_down(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    let mut poll_locked: c_uint = 0;
    sis190_delete_timer(dev);
    netif_stop_queue(dev);
    do {
    spin_lock_irq(&tp.lock);
    sis190_asic_down(ioaddr);
    spin_unlock_irq(&tp.lock);
    synchronize_irq(tp.pci_dev.irq);
    if (!poll_locked)
    poll_locked++;
    synchronize_rcu();
    } while (SIS_R32(IntrMask));
    sis190_tx_clear(tp);
    sis190_rx_clear(tp);
    }
#[no_mangle]
unsafe extern "C" fn sis190_close(dev: *mut net_device) -> c_int {
    static int sis190_close(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    struct pci_dev *pdev = tp.pci_dev;
    sis190_down(dev);
    free_irq(pdev.irq, dev);
    dma_free_coherent(&pdev.dev, TX_RING_BYTES, tp.TxDescRing,
    tp.tx_dma);
    dma_free_coherent(&pdev.dev, RX_RING_BYTES, tp.RxDescRing,
    tp.rx_dma);
    tp.TxDescRing = core::ptr::null_mut();
    tp.RxDescRing = core::ptr::null_mut();
    return 0;
    }
    static netdev_tx_t sis190_start_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    u32 len, entry, dirty_tx;
    struct TxDesc *desc;
    dma_addr_t mapping;
    if (unlikely(skb.len < ETH_ZLEN)) {
    if (skb_padto(skb, ETH_ZLEN)) {
    dev.stats.tx_dropped++;
    goto out;
    }
    len = ETH_ZLEN;
    } else {
    len = skb.len;
    }
    entry = tp.cur_tx % NUM_TX_DESC;
    desc = tp.TxDescRing + entry;
    if (unlikely(le32_to_cpu(desc.status) & OWNbit)) {
    netif_stop_queue(dev);
    netif_err(tp, tx_err, dev,
    "BUG! Tx Ring full when queue awake!\n");
    return NETDEV_TX_BUSY;
    }
    mapping = dma_map_single(&tp.pci_dev.dev, skb.data, len,
    DMA_TO_DEVICE);
    if (dma_mapping_error(&tp.pci_dev.dev, mapping)) {
    netif_err(tp, tx_err, dev,
    "PCI mapping failed, dropping packet");
    return NETDEV_TX_BUSY;
    }
    tp.Tx_skbuff[entry] = skb;
    desc.PSize = cpu_to_le32(len);
    desc.addr = cpu_to_le32(mapping);
    desc.size = cpu_to_le32(len);
    if (entry == (NUM_TX_DESC - 1))
    desc.size |= cpu_to_le32(RingEnd);
    wmb();
    desc.status = cpu_to_le32(OWNbit | INTbit | DEFbit | CRCbit | PADbit);
    if (tp.negotiated_lpa & (LPA_1000HALF | LPA_100HALF | LPA_10HALF)) {
// Half Duplex
    desc.status |= cpu_to_le32(COLEN | CRSEN | BKFEN);
    if (tp.negotiated_lpa & (LPA_1000HALF | LPA_1000FULL))
    desc.status |= cpu_to_le32(EXTEN | BSTEN); /* gigabit HD */
    }
    tp.cur_tx++;
    smp_wmb();
    SIS_W32(TxControl, 0x1a00 | CmdReset | CmdTxEnb);
    dirty_tx = tp.dirty_tx;
    if ((tp.cur_tx - NUM_TX_DESC) == dirty_tx) {
    netif_stop_queue(dev);
    smp_rmb();
    if (dirty_tx != tp.dirty_tx)
    netif_wake_queue(dev);
    }
    out:
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn sis190_free_phy(first_phy: *mut list_head) {
    static void sis190_free_phy(struct list_head *first_phy)
    {
    struct sis190_phy *cur, *next;
    list_for_each_entry_safe(cur, next, first_phy, list) {
    kfree(cur);
    }
    }
//
// sis190_default_phy - Select default PHY for sis190 mac.
// @dev: the net device to probe for
//
// Select first detected PHY with link as default.
// If no one is link on, select PHY whose types is HOME as default.
// If HOME doesn't exist, select LAN.
//
#[no_mangle]
unsafe extern "C" fn sis190_default_phy(dev: *mut net_device) -> u16 {
    static u16 sis190_default_phy(struct net_device *dev)
    {
    struct sis190_phy *phy, *phy_home, *phy_default, *phy_lan;
    struct sis190_private *tp = netdev_priv(dev);
    struct mii_if_info *mii_if = &tp.mii_if;
    void __iomem *ioaddr = tp.mmio_addr;
    u16 status;
    phy_home = phy_default = phy_lan = core::ptr::null_mut();
    list_for_each_entry(phy, &tp.first_phy, list) {
    status = mdio_read_latched(ioaddr, phy.phy_id, MII_BMSR);
// Link ON & Not select default PHY & not ghost PHY.
    if ((status & BMSR_LSTATUS) &&
    !phy_default &&
    (phy.type != UNKNOWN)) {
    phy_default = phy;
    } else {
    status = mdio_read(ioaddr, phy.phy_id, MII_BMCR);
    mdio_write(ioaddr, phy.phy_id, MII_BMCR,
    status | BMCR_ANENABLE | BMCR_ISOLATE);
    if (phy.type == HOME)
    phy_home = phy;
#[no_mangle]
pub unsafe extern "C" fn if(LAN: phy->type ==) -> else {
    else if (phy.type == LAN)
    phy_lan = phy;
    }
    }
    if (!phy_default) {
    if (phy_home)
    phy_default = phy_home;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: phy_lan) -> else {
    else if (phy_lan)
    phy_default = phy_lan;
    else
    phy_default = list_first_entry(&tp.first_phy,
    struct sis190_phy, list);
    }
    if (mii_if.phy_id != phy_default.phy_id) {
    mii_if.phy_id = phy_default.phy_id;
    if (netif_msg_probe(tp))
    pr_info("%s: Using transceiver at address %d as default\n",
    pci_name(tp.pci_dev), mii_if.phy_id);
    }
    status = mdio_read(ioaddr, mii_if.phy_id, MII_BMCR);
    status &= (~BMCR_ISOLATE);
    mdio_write(ioaddr, mii_if.phy_id, MII_BMCR, status);
    status = mdio_read_latched(ioaddr, mii_if.phy_id, MII_BMSR);
    return status;
    }
    static void sis190_init_phy(struct net_device *dev, struct sis190_private *tp,
    struct sis190_phy *phy, unsigned int phy_id,
    u16 mii_status)
    {
    void __iomem *ioaddr = tp.mmio_addr;
    struct mii_chip_info *p;
    INIT_LIST_HEAD(&phy.list);
    phy.status = mii_status;
    phy.phy_id = phy_id;
    phy.id[0] = mdio_read(ioaddr, phy_id, MII_PHYSID1);
    phy.id[1] = mdio_read(ioaddr, phy_id, MII_PHYSID2);
    for (p = mii_chip_table; p.type; p++) {
    if ((p.id[0] == phy.id[0]) &&
    (p.id[1] == (phy.id[1] & 0xfff0))) {
    break;
    }
    }
    if (p.id[1]) {
    phy.type = (p.type == MIX) ?
    ((mii_status & (BMSR_100FULL | BMSR_100HALF)) ?
    LAN : HOME) : p.type;
    tp.features |= p.feature;
    if (netif_msg_probe(tp))
    pr_info("%s: %s transceiver at address %d\n",
    pci_name(tp.pci_dev), p.name, phy_id);
    } else {
    phy.type = UNKNOWN;
    if (netif_msg_probe(tp))
    pr_info("%s: unknown PHY 0x%x:0x%x transceiver at address %d\n",
    pci_name(tp.pci_dev),
    phy.id[0], (phy.id[1] & 0xfff0), phy_id);
    }
    }
#[no_mangle]
unsafe extern "C" fn sis190_mii_probe_88e1111_fixup(tp: *mut sis190_private) {
    static void sis190_mii_probe_88e1111_fixup(struct sis190_private *tp)
    {
    if (tp.features & F_PHY_88E1111) {
    void __iomem *ioaddr = tp.mmio_addr;
    let mut phy_id: c_int = tp.mii_if.phy_id;
    u16 reg[2][2] = {
    { 0x808b, 0x0ce1 },
    { 0x808f, 0x0c60 }
    }, *p;
    p = (tp.features & F_HAS_RGMII) ? reg[0] : reg[1];
    mdio_write(ioaddr, phy_id, 0x1b, p[0]);
    udelay(200);
    mdio_write(ioaddr, phy_id, 0x14, p[1]);
    udelay(200);
    }
    }
//
// sis190_mii_probe - Probe MII PHY for sis190
// @dev: the net device to probe for
//
// Search for total of 32 possible mii phy addresses.
// Identify and set current phy if found one,
// return error if it failed to found.
//
#[no_mangle]
unsafe extern "C" fn sis190_mii_probe(dev: *mut net_device) -> c_int {
    static int sis190_mii_probe(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    struct mii_if_info *mii_if = &tp.mii_if;
    void __iomem *ioaddr = tp.mmio_addr;
    int phy_id;
    let mut rc: c_int = 0;
    INIT_LIST_HEAD(&tp.first_phy);
    for (phy_id = 0; phy_id < PHY_MAX_ADDR; phy_id++) {
    struct sis190_phy *phy;
    u16 status;
    status = mdio_read_latched(ioaddr, phy_id, MII_BMSR);
// Try next mii if the current one is not accessible.
    if (status == 0xffff || status == 0x0000)
    continue;
    phy = kmalloc_obj(*phy);
    if (!phy) {
    sis190_free_phy(&tp.first_phy);
    rc = -ENOMEM;
    goto out;
    }
    sis190_init_phy(dev, tp, phy, phy_id, status);
    list_add(&tp.first_phy, &phy.list);
    }
    if (list_empty(&tp.first_phy)) {
    if (netif_msg_probe(tp))
    pr_info("%s: No MII transceivers found!\n",
    pci_name(tp.pci_dev));
    rc = -EIO;
    goto out;
    }
// Select default PHY for mac
    sis190_default_phy(dev);
    sis190_mii_probe_88e1111_fixup(tp);
    mii_if.dev = dev;
    mii_if.mdio_read = __mdio_read;
    mii_if.mdio_write = __mdio_write;
    mii_if.phy_id_mask = PHY_ID_ANY;
    mii_if.reg_num_mask = MII_REG_ANY;
    out:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sis190_mii_remove(dev: *mut net_device) {
    static void sis190_mii_remove(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    sis190_free_phy(&tp.first_phy);
    }
#[no_mangle]
unsafe extern "C" fn sis190_release_board(pdev: *mut pci_dev) {
    static void sis190_release_board(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    struct sis190_private *tp = netdev_priv(dev);
    iounmap(tp.mmio_addr);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    free_netdev(dev);
    }
    static struct net_device *sis190_init_board(struct pci_dev *pdev)
    {
    struct sis190_private *tp;
    struct net_device *dev;
    void __iomem *ioaddr;
    int rc;
    dev = alloc_etherdev(sizeof(*tp));
    if (!dev) {
    rc = -ENOMEM;
    goto err_out_0;
    }
    SET_NETDEV_DEV(dev, &pdev.dev);
    tp = netdev_priv(dev);
    tp.dev = dev;
    tp.msg_enable = netif_msg_init(debug.msg_enable, SIS190_MSG_DEFAULT);
    rc = pci_enable_device(pdev);
    if (rc < 0) {
    if (netif_msg_probe(tp))
    pr_err("%s: enable failure\n", pci_name(pdev));
    goto err_free_dev_1;
    }
    rc = -ENODEV;
    if (!(pci_resource_flags(pdev, 0) & IORESOURCE_MEM)) {
    if (netif_msg_probe(tp))
    pr_err("%s: region #0 is no MMIO resource\n",
    pci_name(pdev));
    goto err_pci_disable_2;
    }
    if (pci_resource_len(pdev, 0) < SIS190_REGS_SIZE) {
    if (netif_msg_probe(tp))
    pr_err("%s: invalid PCI region size(s)\n",
    pci_name(pdev));
    goto err_pci_disable_2;
    }
    rc = pci_request_regions(pdev, DRV_NAME);
    if (rc < 0) {
    if (netif_msg_probe(tp))
    pr_err("%s: could not request regions\n",
    pci_name(pdev));
    goto err_pci_disable_2;
    }
    rc = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (rc < 0) {
    if (netif_msg_probe(tp))
    pr_err("%s: DMA configuration failed\n",
    pci_name(pdev));
    goto err_free_res_3;
    }
    pci_set_master(pdev);
    ioaddr = ioremap(pci_resource_start(pdev, 0), SIS190_REGS_SIZE);
    if (!ioaddr) {
    if (netif_msg_probe(tp))
    pr_err("%s: cannot remap MMIO, aborting\n",
    pci_name(pdev));
    rc = -EIO;
    goto err_free_res_3;
    }
    tp.pci_dev = pdev;
    tp.mmio_addr = ioaddr;
    tp.link_status = LNK_OFF;
    sis190_irq_mask_and_ack(ioaddr);
    sis190_soft_reset(ioaddr);
    out:
    return dev;
    err_free_res_3:
    pci_release_regions(pdev);
    err_pci_disable_2:
    pci_disable_device(pdev);
    err_free_dev_1:
    free_netdev(dev);
    err_out_0:
    dev = ERR_PTR(rc);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn sis190_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void sis190_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    u8 tmp8;
// Disable Tx, if not already
    tmp8 = SIS_R8(TxControl);
    if (tmp8 & CmdTxEnb)
    SIS_W8(TxControl, tmp8 & ~CmdTxEnb);
    netif_info(tp, tx_err, dev, "Transmit timeout, status %08x %08x\n",
    SIS_R32(TxControl), SIS_R32(TxSts));
// Disable interrupts by clearing the interrupt mask.
    SIS_W32(IntrMask, 0x0000);
// Stop a shared interrupt from scavenging while we are.
    spin_lock_irq(&tp.lock);
    sis190_tx_clear(tp);
    spin_unlock_irq(&tp.lock);
// ...and finally, reset everything.
    sis190_hw_start(dev);
    netif_wake_queue(dev);
    }
#[no_mangle]
unsafe extern "C" fn sis190_set_rgmii(tp: *mut sis190_private, reg: u8) {
    static void sis190_set_rgmii(struct sis190_private *tp, u8 reg)
    {
    tp.features |= (reg & 0x80) ? F_HAS_RGMII : 0;
    }
    static int sis190_get_mac_addr_from_eeprom(struct pci_dev *pdev,
    struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    __le16 addr[ETH_ALEN / 2];
    u16 sig;
    int i;
    if (netif_msg_probe(tp))
    pr_info("%s: Read MAC address from EEPROM\n", pci_name(pdev));
// Check to see if there is a sane EEPROM
    sig = (u16) sis190_read_eeprom(ioaddr, EEPROMSignature);
    if ((sig == 0xffff) || (sig == 0x0000)) {
    if (netif_msg_probe(tp))
    pr_info("%s: Error EEPROM read %x\n",
    pci_name(pdev), sig);
    return -EIO;
    }
// Get MAC address from EEPROM
    for (i = 0; i < ETH_ALEN / 2; i++) {
    let mut w: u16 = sis190_read_eeprom(ioaddr, EEPROMMACAddr + i);
    addr[i] = cpu_to_le16(w);
    }
    eth_hw_addr_set(dev, (u8 *)addr);
    sis190_set_rgmii(tp, sis190_read_eeprom(ioaddr, EEPROMInfo));
    return 0;
    }
//
// sis190_get_mac_addr_from_apc - Get MAC address for SiS96x model
// @pdev: PCI device
// @dev:  network device to get address for
//
// SiS96x model, use APC CMOS RAM to store MAC address.
// APC CMOS RAM is accessed through ISA bridge.
// MAC address is read into @net_dev->dev_addr.
//
    static int sis190_get_mac_addr_from_apc(struct pci_dev *pdev,
    struct net_device *dev)
    {
    static const u16 ids[] = { 0x0965, 0x0966, 0x0968 };
    struct sis190_private *tp = netdev_priv(dev);
    struct pci_dev *isa_bridge;
    u8 addr[ETH_ALEN];
    u8 reg, tmp8;
    unsigned int i;
    if (netif_msg_probe(tp))
    pr_info("%s: Read MAC address from APC\n", pci_name(pdev));
    for (i = 0; i < ARRAY_SIZE(ids); i++) {
    isa_bridge = pci_get_device(PCI_VENDOR_ID_SI, ids[i], core::ptr::null_mut());
    if (isa_bridge)
    break;
    }
    if (!isa_bridge) {
    if (netif_msg_probe(tp))
    pr_info("%s: Can not find ISA bridge\n",
    pci_name(pdev));
    return -EIO;
    }
// Enable port 78h & 79h to access APC Registers.
    pci_read_config_byte(isa_bridge, 0x48, &tmp8);
    reg = (tmp8 & ~0x02);
    pci_write_config_byte(isa_bridge, 0x48, reg);
    udelay(50);
    pci_read_config_byte(isa_bridge, 0x48, &reg);
    for (i = 0; i < ETH_ALEN; i++) {
    outb(0x9 + i, 0x78);
    addr[i] = inb(0x79);
    }
    eth_hw_addr_set(dev, addr);
    outb(0x12, 0x78);
    reg = inb(0x79);
    sis190_set_rgmii(tp, reg);
// Restore the value to ISA Bridge
    pci_write_config_byte(isa_bridge, 0x48, tmp8);
    pci_dev_put(isa_bridge);
    return 0;
    }
//
// sis190_init_rxfilter - Initialize the Rx filter
// @dev: network device to initialize
//
// Set receive filter address to our MAC address
// and enable packet filtering.
//
#[no_mangle]
pub unsafe extern "C" fn sis190_init_rxfilter(dev: *mut net_device) {
    static inline void sis190_init_rxfilter(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    u16 ctl;
    int i;
    ctl = SIS_R16(RxMacControl);
//
// Disable packet filtering before setting filter.
// Note: SiS's driver writes 32 bits but RxMacControl is 16 bits
// only and followed by RxMacAddr (6 bytes). Strange. -- FR
//
    SIS_W16(RxMacControl, ctl & ~0x0f00);
    for (i = 0; i < ETH_ALEN; i++)
    SIS_W8(RxMacAddr + i, dev.dev_addr[i]);
    SIS_W16(RxMacControl, ctl);
    SIS_PCI_COMMIT();
    }
#[no_mangle]
unsafe extern "C" fn sis190_get_mac_addr(pdev: *mut pci_dev, dev: *mut net_device) -> c_int {
    static int sis190_get_mac_addr(struct pci_dev *pdev, struct net_device *dev)
    {
    int rc;
    rc = sis190_get_mac_addr_from_eeprom(pdev, dev);
    if (rc < 0) {
    u8 reg;
    pci_read_config_byte(pdev, 0x73, &reg);
    if (reg & 0x00000001)
    rc = sis190_get_mac_addr_from_apc(pdev, dev);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sis190_set_speed_auto(dev: *mut net_device) {
    static void sis190_set_speed_auto(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.mmio_addr;
    let mut phy_id: c_int = tp.mii_if.phy_id;
    int val;
    netif_info(tp, link, dev, "Enabling Auto-negotiation\n");
    val = mdio_read(ioaddr, phy_id, MII_ADVERTISE);
// Enable 10/100 Full/Half Mode, leave MII_ADVERTISE bit4:0
// unchanged.
    mdio_write(ioaddr, phy_id, MII_ADVERTISE, (val & ADVERTISE_SLCT) |
    ADVERTISE_100FULL | ADVERTISE_10FULL |
    ADVERTISE_100HALF | ADVERTISE_10HALF);
// Enable 1000 Full Mode.
    mdio_write(ioaddr, phy_id, MII_CTRL1000, ADVERTISE_1000FULL);
// Enable auto-negotiation and restart auto-negotiation.
    mdio_write(ioaddr, phy_id, MII_BMCR,
    BMCR_ANENABLE | BMCR_ANRESTART | BMCR_RESET);
    }
    static int sis190_get_link_ksettings(struct net_device *dev,
    struct ethtool_link_ksettings *cmd)
    {
    struct sis190_private *tp = netdev_priv(dev);
    mii_ethtool_get_link_ksettings(&tp.mii_if, cmd);
    return 0;
    }
    static int sis190_set_link_ksettings(struct net_device *dev,
    const struct ethtool_link_ksettings *cmd)
    {
    struct sis190_private *tp = netdev_priv(dev);
    return mii_ethtool_set_link_ksettings(&tp.mii_if, cmd);
    }
    static void sis190_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    struct sis190_private *tp = netdev_priv(dev);
    strscpy(info.driver, DRV_NAME, sizeof(info.driver));
    strscpy(info.version, DRV_VERSION, sizeof(info.version));
    strscpy(info.bus_info, pci_name(tp.pci_dev),
    sizeof(info.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn sis190_get_regs_len(dev: *mut net_device) -> c_int {
    static int sis190_get_regs_len(struct net_device *dev)
    {
    return SIS190_REGS_SIZE;
    }
    static void sis190_get_regs(struct net_device *dev, struct ethtool_regs *regs,
    void *p)
    {
    struct sis190_private *tp = netdev_priv(dev);
    unsigned long flags;
    spin_lock_irqsave(&tp.lock, flags);
    memcpy_fromio(p, tp.mmio_addr, regs.len);
    spin_unlock_irqrestore(&tp.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sis190_nway_reset(dev: *mut net_device) -> c_int {
    static int sis190_nway_reset(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    return mii_nway_restart(&tp.mii_if);
    }
#[no_mangle]
unsafe extern "C" fn sis190_get_msglevel(dev: *mut net_device) -> u32 {
    static u32 sis190_get_msglevel(struct net_device *dev)
    {
    struct sis190_private *tp = netdev_priv(dev);
    return tp.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn sis190_set_msglevel(dev: *mut net_device, value: u32) {
    static void sis190_set_msglevel(struct net_device *dev, u32 value)
    {
    struct sis190_private *tp = netdev_priv(dev);
    tp.msg_enable = value;
    }
    static const struct ethtool_ops sis190_ethtool_ops = {
    .get_drvinfo	= sis190_get_drvinfo,
    .get_regs_len	= sis190_get_regs_len,
    .get_regs	= sis190_get_regs,
    .get_link	= ethtool_op_get_link,
    .get_msglevel	= sis190_get_msglevel,
    .set_msglevel	= sis190_set_msglevel,
    .nway_reset	= sis190_nway_reset,
    .get_link_ksettings = sis190_get_link_ksettings,
    .set_link_ksettings = sis190_set_link_ksettings,
    };
#[no_mangle]
unsafe extern "C" fn sis190_ioctl(dev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int {
    static int sis190_ioctl(struct net_device *dev, struct ifreq *ifr, int cmd)
    {
    struct sis190_private *tp = netdev_priv(dev);
    return !netif_running(dev) ? -EINVAL :
    generic_mii_ioctl(&tp.mii_if, if_mii(ifr), cmd, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sis190_mac_addr(dev: *mut net_device, p: *mut c_void) -> c_int {
    static int sis190_mac_addr(struct net_device  *dev, void *p)
    {
    int rc;
    rc = eth_mac_addr(dev, p);
    if (!rc)
    sis190_init_rxfilter(dev);
    return rc;
    }
    static const struct net_device_ops sis190_netdev_ops = {
    .ndo_open		= sis190_open,
    .ndo_stop		= sis190_close,
    .ndo_eth_ioctl		= sis190_ioctl,
    .ndo_start_xmit		= sis190_start_xmit,
    .ndo_tx_timeout		= sis190_tx_timeout,
    .ndo_set_rx_mode	= sis190_set_rx_mode,
    .ndo_set_mac_address	= sis190_mac_addr,
    .ndo_validate_addr	= eth_validate_addr,

    .ndo_poll_controller	 = sis190_netpoll,

    };
    static int sis190_init_one(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    let mut printed_version: static int = 0;
    struct sis190_private *tp;
    struct net_device *dev;
    void __iomem *ioaddr;
    int rc;
    if (!printed_version) {
    if (netif_msg_drv(&debug))
    pr_info(SIS190_DRIVER_NAME " loaded\n");
    printed_version = 1;
    }
    dev = sis190_init_board(pdev);
    if (IS_ERR(dev)) {
    rc = PTR_ERR(dev);
    goto out;
    }
    pci_set_drvdata(pdev, dev);
    tp = netdev_priv(dev);
    ioaddr = tp.mmio_addr;
    rc = sis190_get_mac_addr(pdev, dev);
    if (rc < 0)
    goto err_release_board;
    sis190_init_rxfilter(dev);
    INIT_WORK(&tp.phy_task, sis190_phy_task);
    dev.netdev_ops = &sis190_netdev_ops;
    dev.ethtool_ops = &sis190_ethtool_ops;
    dev.watchdog_timeo = SIS190_TX_TIMEOUT;
    spin_lock_init(&tp.lock);
    rc = sis190_mii_probe(dev);
    if (rc < 0)
    goto err_release_board;
    rc = register_netdev(dev);
    if (rc < 0)
    goto err_remove_mii;
    if (netif_msg_probe(tp)) {
    netdev_info(dev, "%s: %s at %p (IRQ: %d), %pM\n",
    pci_name(pdev),
    sis_chip_info[ent.driver_data].name,
    ioaddr, pdev.irq, dev.dev_addr);
    netdev_info(dev, "%s mode.\n",
    (tp.features & F_HAS_RGMII) ? "RGMII" : "GMII");
    }
    netif_carrier_off(dev);
    sis190_set_speed_auto(dev);
    out:
    return rc;
    err_remove_mii:
    sis190_mii_remove(dev);
    err_release_board:
    sis190_release_board(pdev);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn sis190_remove_one(pdev: *mut pci_dev) {
    static void sis190_remove_one(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    struct sis190_private *tp = netdev_priv(dev);
    sis190_mii_remove(dev);
    cancel_work_sync(&tp.phy_task);
    unregister_netdev(dev);
    sis190_release_board(pdev);
    }
    static struct pci_driver sis190_pci_driver = {
    .name		= DRV_NAME,
    .id_table	= sis190_pci_tbl,
    .probe		= sis190_init_one,
    .remove		= sis190_remove_one,
    };
    module_pci_driver(sis190_pci_driver);
