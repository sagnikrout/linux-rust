//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/vitesse-vsc73xx-core.c
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
// DSA driver for:
// Vitesse VSC7385 SparX-G5 5+1-port Integrated Gigabit Ethernet Switch
// Vitesse VSC7388 SparX-G8 8-port Integrated Gigabit Ethernet Switch
// Vitesse VSC7395 SparX-G5e 5+1-port Integrated Gigabit Ethernet Switch
// Vitesse VSC7398 SparX-G8e 8-port Integrated Gigabit Ethernet Switch
//
// These switches have a built-in 8051 CPU and can download and execute a
// firmware in this CPU. They can also be configured to use an external CPU
// handling the switch in a memory-mapped manner by connecting to that external
// CPU's memory bus.
//
// Copyright (C) 2018 Linus Wallej <linus.walleij@linaro.org>
// Includes portions of code from the firmware uploader by:
// Copyright (C) 2009 Gabor Juhos <juhosg@openwrt.org>
//

pub const VSC73XX_BLOCK_MAC: c_uint = 0x1 /* Subblocks 0-4, 6 (CPU port) */;
pub const VSC73XX_BLOCK_ANALYZER: c_uint = 0x2 /* Only subblock 0 */;
pub const VSC73XX_BLOCK_MII: c_uint = 0x3 /* Subblocks 0 and 1 */;
pub const VSC73XX_BLOCK_MEMINIT: c_uint = 0x3 /* Only subblock 2 */;
pub const VSC73XX_BLOCK_CAPTURE: c_uint = 0x4 /* Subblocks 0-4, 6, 7 */;
pub const VSC73XX_BLOCK_ARBITER: c_uint = 0x5 /* Only subblock 0 */;
pub const VSC73XX_BLOCK_SYSTEM: c_uint = 0x7 /* Only subblock 0 */;
// MII Block subblock
pub const VSC73XX_BLOCK_MII_INTERNAL: c_uint = 0x0 /* Internal MDIO subblock */;
pub const VSC73XX_BLOCK_MII_EXTERNAL: c_uint = 0x1 /* External MDIO subblock */;

pub const VSC73XX_NUM_FDB_ROWS: c_int = 2048;
pub const VSC73XX_NUM_BUCKETS: c_int = 4;
// MAC Block registers
pub const VSC73XX_MAC_CFG: c_uint = 0x00;
pub const VSC73XX_MACHDXGAP: c_uint = 0x02;
pub const VSC73XX_FCCONF: c_uint = 0x04;
pub const VSC73XX_FCMACHI: c_uint = 0x08;
pub const VSC73XX_FCMACLO: c_uint = 0x0c;
pub const VSC73XX_MAXLEN: c_uint = 0x10;
pub const VSC73XX_ADVPORTM: c_uint = 0x19;
pub const VSC73XX_TXUPDCFG: c_uint = 0x24;
pub const VSC73XX_TXQ_SELECT_CFG: c_uint = 0x28;
pub const VSC73XX_RXOCT: c_uint = 0x50;
pub const VSC73XX_TXOCT: c_uint = 0x51;
pub const VSC73XX_C_RX0: c_uint = 0x52;
pub const VSC73XX_C_RX1: c_uint = 0x53;
pub const VSC73XX_C_RX2: c_uint = 0x54;
pub const VSC73XX_C_TX0: c_uint = 0x55;
pub const VSC73XX_C_TX1: c_uint = 0x56;
pub const VSC73XX_C_TX2: c_uint = 0x57;
pub const VSC73XX_C_CFG: c_uint = 0x58;
pub const VSC73XX_CAT_DROP: c_uint = 0x6e;
pub const VSC73XX_CAT_PR_MISC_L2: c_uint = 0x6f;
pub const VSC73XX_CAT_PR_USR_PRIO: c_uint = 0x75;
pub const VSC73XX_CAT_VLAN_MISC: c_uint = 0x79;
pub const VSC73XX_CAT_PORT_VLAN: c_uint = 0x7a;
pub const VSC73XX_Q_MISC_CONF: c_uint = 0xdf;
// MAC_CFG register bits

pub const VSC73XX_MAC_CFG_SEED_OFFSET: c_int = 19;

pub const VSC73XX_MAC_CFG_TX_IPG_OFFSET: c_int = 6;

pub const VSC73XX_MAC_CFG_CLK_SEL_OFFSET: c_int = 0;
pub const VSC73XX_MAC_CFG_CLK_SEL_1000M: c_int = 1;
pub const VSC73XX_MAC_CFG_CLK_SEL_100M: c_int = 2;
pub const VSC73XX_MAC_CFG_CLK_SEL_10M: c_int = 3;
pub const VSC73XX_MAC_CFG_CLK_SEL_EXT: c_int = 4;

    VSC73XX_MAC_CFG_GIGA_MODE | \
    VSC73XX_MAC_CFG_TX_IPG_1000M | \
    VSC73XX_MAC_CFG_CLK_SEL_EXT)

    VSC73XX_MAC_CFG_TX_IPG_100_10M | \
    VSC73XX_MAC_CFG_CLK_SEL_EXT)

    VSC73XX_MAC_CFG_CLK_SEL_EXT)

    VSC73XX_MAC_CFG_GIGA_MODE | \
    VSC73XX_MAC_CFG_TX_IPG_1000M | \
    VSC73XX_MAC_CFG_CLK_SEL_1000M)

    VSC73XX_MAC_CFG_MAC_RX_RST | \
    VSC73XX_MAC_CFG_MAC_TX_RST)
// Flow control register bits

// ADVPORTM advanced port setup register bits

// TXUPDCFG transmit modify setup bits

pub const VSC73XX_TXUPDCFG_TX_UNTAGGED_VID_SHIFT: c_int = 4;
// CAT_DROP categorizer frame dropping register bits

// CAT_VLAN_MISC categorizer VLAN miscellaneous bits

// CAT_PORT_VLAN categorizer port VLAN

// Frame analyzer block 2 registers
pub const VSC73XX_STORMLIMIT: c_uint = 0x02;
pub const VSC73XX_ADVLEARN: c_uint = 0x03;
pub const VSC73XX_IFLODMSK: c_uint = 0x04;
pub const VSC73XX_VLANMASK: c_uint = 0x05;
pub const VSC73XX_MACHDATA: c_uint = 0x06;
pub const VSC73XX_MACLDATA: c_uint = 0x07;
pub const VSC73XX_ANMOVED: c_uint = 0x08;
pub const VSC73XX_ANAGEFIL: c_uint = 0x09;
pub const VSC73XX_ANEVENTS: c_uint = 0x0a;
pub const VSC73XX_ANCNTMASK: c_uint = 0x0b;
pub const VSC73XX_ANCNTVAL: c_uint = 0x0c;
pub const VSC73XX_LEARNMASK: c_uint = 0x0d;
pub const VSC73XX_UFLODMASK: c_uint = 0x0e;
pub const VSC73XX_MFLODMASK: c_uint = 0x0f;
pub const VSC73XX_RECVMASK: c_uint = 0x10;
pub const VSC73XX_AGGRCTRL: c_uint = 0x20;
pub const VSC73XX_AGGRMSKS: c_uint = 0x30 /* Until 0x3f */;
pub const VSC73XX_DSTMASKS: c_uint = 0x40 /* Until 0x7f */;
pub const VSC73XX_SRCMASKS: c_uint = 0x80 /* Until 0x87 */;
pub const VSC73XX_CAPENAB: c_uint = 0xa0;
pub const VSC73XX_MACACCESS: c_uint = 0xb0;
pub const VSC73XX_IPMCACCESS: c_uint = 0xb1;
pub const VSC73XX_MACTINDX: c_uint = 0xc0;
pub const VSC73XX_VLANACCESS: c_uint = 0xd0;
pub const VSC73XX_VLANTIDX: c_uint = 0xe0;
pub const VSC73XX_AGENCTRL: c_uint = 0xf0;
pub const VSC73XX_CAPRST: c_uint = 0xff;

pub const VSC73XX_MACACCESS_CMD_IDLE: c_int = 0;
pub const VSC73XX_MACACCESS_CMD_LEARN: c_int = 1;
pub const VSC73XX_MACACCESS_CMD_FORGET: c_int = 2;
pub const VSC73XX_MACACCESS_CMD_AGE_TABLE: c_int = 3;
pub const VSC73XX_MACACCESS_CMD_FLUSH_TABLE: c_int = 4;
pub const VSC73XX_MACACCESS_CMD_CLEAR_TABLE: c_int = 5;
pub const VSC73XX_MACACCESS_CMD_READ_ENTRY: c_int = 6;
pub const VSC73XX_MACACCESS_CMD_WRITE_ENTRY: c_int = 7;

pub const VSC73XX_VLANACCESS_VLAN_PORT_MASK_SHIFT: c_int = 2;

pub const VSC73XX_VLANACCESS_VLAN_TBL_CMD_IDLE: c_int = 0;
pub const VSC73XX_VLANACCESS_VLAN_TBL_CMD_READ_ENTRY: c_int = 1;
pub const VSC73XX_VLANACCESS_VLAN_TBL_CMD_WRITE_ENTRY: c_int = 2;
pub const VSC73XX_VLANACCESS_VLAN_TBL_CMD_CLEAR_TABLE: c_int = 3;
// MII block 3 registers
pub const VSC73XX_MII_STAT: c_uint = 0x0;
pub const VSC73XX_MII_CMD: c_uint = 0x1;
pub const VSC73XX_MII_DATA: c_uint = 0x2;
pub const VSC73XX_MII_MPRES: c_uint = 0x3;

// Arbiter block 5 registers
pub const VSC73XX_ARBEMPTY: c_uint = 0x0c;
pub const VSC73XX_ARBDISC: c_uint = 0x0e;
pub const VSC73XX_SBACKWDROP: c_uint = 0x12;
pub const VSC73XX_DBACKWDROP: c_uint = 0x13;
pub const VSC73XX_ARBBURSTPROB: c_uint = 0x15;
// System block 7 registers
pub const VSC73XX_ICPU_SIPAD: c_uint = 0x01;
pub const VSC73XX_GMIIDELAY: c_uint = 0x05;
pub const VSC73XX_ICPU_CTRL: c_uint = 0x10;
pub const VSC73XX_ICPU_ADDR: c_uint = 0x11;
pub const VSC73XX_ICPU_SRAM: c_uint = 0x12;
pub const VSC73XX_HWSEM: c_uint = 0x13;
pub const VSC73XX_GLORESET: c_uint = 0x14;
pub const VSC73XX_ICPU_MBOX_VAL: c_uint = 0x15;
pub const VSC73XX_ICPU_MBOX_SET: c_uint = 0x16;
pub const VSC73XX_ICPU_MBOX_CLR: c_uint = 0x17;
pub const VSC73XX_CHIPID: c_uint = 0x18;
pub const VSC73XX_GPIO: c_uint = 0x34;
pub const VSC73XX_GMIIDELAY_GMII0_GTXDELAY_NONE: c_int = 0;
pub const VSC73XX_GMIIDELAY_GMII0_GTXDELAY_1_4_NS: c_int = 1;
pub const VSC73XX_GMIIDELAY_GMII0_GTXDELAY_1_7_NS: c_int = 2;
pub const VSC73XX_GMIIDELAY_GMII0_GTXDELAY_2_0_NS: c_int = 3;

pub const VSC73XX_CHIPID_ID_SHIFT: c_int = 12;
pub const VSC73XX_CHIPID_ID_MASK: c_uint = 0xffff;
pub const VSC73XX_CHIPID_REV_SHIFT: c_int = 28;
pub const VSC73XX_CHIPID_REV_MASK: c_uint = 0xf;
pub const VSC73XX_CHIPID_ID_7385: c_uint = 0x7385;
pub const VSC73XX_CHIPID_ID_7388: c_uint = 0x7388;
pub const VSC73XX_CHIPID_ID_7395: c_uint = 0x7395;
pub const VSC73XX_CHIPID_ID_7398: c_uint = 0x7398;

    VSC73XX_ICPU_CTRL_BOOT_EN | \
    VSC73XX_ICPU_CTRL_EXT_ACC_EN)

    VSC73XX_ICPU_CTRL_BOOT_EN | \
    VSC73XX_ICPU_CTRL_CLK_EN | \
    VSC73XX_ICPU_CTRL_SRST)

pub const VSC73XX_POLL_SLEEP_US: c_int = 1000;
pub const VSC73XX_MDIO_POLL_SLEEP_US: c_int = 5;
pub const VSC73XX_POLL_TIMEOUT_US: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_counter {
    pub counter: u8,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_fdb {
    pub vid: u16,
    pub port: u8,
    pub mac: [u8; ETH_ALEN],
    pub valid: bool,
}

// Counters are named according to the MIB standards where applicable.
// Some counters are custom, non-standard. The standard counters are
// named in accordance with RFC2819, RFC2021 and IEEE Std 802.3-2002 Annex
// 30A Counters.
//
    static const struct vsc73xx_counter vsc73xx_rx_counters[] = {
    { 0, "RxEtherStatsPkts" },
    { 1, "RxBroadcast+MulticastPkts" }, /* non-standard counter */
    { 2, "RxTotalErrorPackets" }, /* non-standard counter */
    { 3, "RxEtherStatsBroadcastPkts" },
    { 4, "RxEtherStatsMulticastPkts" },
    { 5, "RxEtherStatsPkts64Octets" },
    { 6, "RxEtherStatsPkts65to127Octets" },
    { 7, "RxEtherStatsPkts128to255Octets" },
    { 8, "RxEtherStatsPkts256to511Octets" },
    { 9, "RxEtherStatsPkts512to1023Octets" },
    { 10, "RxEtherStatsPkts1024to1518Octets" },
    { 11, "RxJumboFrames" }, /* non-standard counter */
    { 12, "RxaPauseMACControlFramesTransmitted" },
    { 13, "RxFIFODrops" }, /* non-standard counter */
    { 14, "RxBackwardDrops" }, /* non-standard counter */
    { 15, "RxClassifierDrops" }, /* non-standard counter */
    { 16, "RxEtherStatsCRCAlignErrors" },
    { 17, "RxEtherStatsUndersizePkts" },
    { 18, "RxEtherStatsOversizePkts" },
    { 19, "RxEtherStatsFragments" },
    { 20, "RxEtherStatsJabbers" },
    { 21, "RxaMACControlFramesReceived" },
// 22-24 are undefined
    { 25, "RxaFramesReceivedOK" },
    { 26, "RxQoSClass0" }, /* non-standard counter */
    { 27, "RxQoSClass1" }, /* non-standard counter */
    { 28, "RxQoSClass2" }, /* non-standard counter */
    { 29, "RxQoSClass3" }, /* non-standard counter */
    };
    static const struct vsc73xx_counter vsc73xx_tx_counters[] = {
    { 0, "TxEtherStatsPkts" },
    { 1, "TxBroadcast+MulticastPkts" }, /* non-standard counter */
    { 2, "TxTotalErrorPackets" }, /* non-standard counter */
    { 3, "TxEtherStatsBroadcastPkts" },
    { 4, "TxEtherStatsMulticastPkts" },
    { 5, "TxEtherStatsPkts64Octets" },
    { 6, "TxEtherStatsPkts65to127Octets" },
    { 7, "TxEtherStatsPkts128to255Octets" },
    { 8, "TxEtherStatsPkts256to511Octets" },
    { 9, "TxEtherStatsPkts512to1023Octets" },
    { 10, "TxEtherStatsPkts1024to1518Octets" },
    { 11, "TxJumboFrames" }, /* non-standard counter */
    { 12, "TxaPauseMACControlFramesTransmitted" },
    { 13, "TxFIFODrops" }, /* non-standard counter */
    { 14, "TxDrops" }, /* non-standard counter */
    { 15, "TxEtherStatsCollisions" },
    { 16, "TxEtherStatsCRCAlignErrors" },
    { 17, "TxEtherStatsUndersizePkts" },
    { 18, "TxEtherStatsOversizePkts" },
    { 19, "TxEtherStatsFragments" },
    { 20, "TxEtherStatsJabbers" },
// 21-24 are undefined
    { 25, "TxaFramesReceivedOK" },
    { 26, "TxQoSClass0" }, /* non-standard counter */
    { 27, "TxQoSClass1" }, /* non-standard counter */
    { 28, "TxQoSClass2" }, /* non-standard counter */
    { 29, "TxQoSClass3" }, /* non-standard counter */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_vlan_summary {
    pub num_tagged: usize,
    pub num_untagged: usize,
}

    enum vsc73xx_port_vlan_conf {
    VSC73XX_VLAN_FILTER,
    VSC73XX_VLAN_FILTER_UNTAG_ALL,
    VSC73XX_VLAN_IGNORE,
    };
#[no_mangle]
pub unsafe extern "C" fn vsc73xx_is_addr_valid(block: u8, subblock: u8) -> c_int {
    int vsc73xx_is_addr_valid(u8 block, u8 subblock)
    {
    switch (block) {
    case VSC73XX_BLOCK_MAC:
    switch (subblock) {
    case 0 ... 4:
    case 6:
    return 1;
    }
    break;
    case VSC73XX_BLOCK_ANALYZER:
    case VSC73XX_BLOCK_SYSTEM:
    switch (subblock) {
    case 0:
    return 1;
    }
    break;
    case VSC73XX_BLOCK_MII:
    case VSC73XX_BLOCK_ARBITER:
    switch (subblock) {
    case 0 ... 1:
    return 1;
    }
    break;
    case VSC73XX_BLOCK_CAPTURE:
    switch (subblock) {
    case 0 ... 4:
    case 6 ... 7:
    return 1;
    }
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL(vsc73xx_is_addr_valid);
    static int vsc73xx_read(struct vsc73xx *vsc, u8 block, u8 subblock, u8 reg,
    u32 *val)
    {
    return vsc.ops.read(vsc, block, subblock, reg, val);
    }
    static int vsc73xx_write(struct vsc73xx *vsc, u8 block, u8 subblock, u8 reg,
    u32 val)
    {
    return vsc.ops.write(vsc, block, subblock, reg, val);
    }
    static int vsc73xx_update_bits(struct vsc73xx *vsc, u8 block, u8 subblock,
    u8 reg, u32 mask, u32 val)
    {
    u32 tmp, orig;
    int ret;
// Same read-modify-write algorithm as e.g. regmap
    ret = vsc73xx_read(vsc, block, subblock, reg, &orig);
    if (ret)
    return ret;
    tmp = orig & ~mask;
    tmp |= val & mask;
    return vsc73xx_write(vsc, block, subblock, reg, tmp);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_detect(vsc: *mut vsc73xx) -> c_int {
    static int vsc73xx_detect(struct vsc73xx *vsc)
    {
    bool icpu_si_boot_en;
    bool icpu_pi_en;
    u32 val;
    u32 rev;
    int ret;
    u32 id;
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_ICPU_MBOX_VAL, &val);
    if (ret) {
    dev_err(vsc.dev, "unable to read mailbox (%d)\n", ret);
    return ret;
    }
    if (val == 0xffffffff) {
    dev_info(vsc.dev, "chip seems dead.\n");
    return -EAGAIN;
    }
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_CHIPID, &val);
    if (ret) {
    dev_err(vsc.dev, "unable to read chip id (%d)\n", ret);
    return ret;
    }
    id = (val >> VSC73XX_CHIPID_ID_SHIFT) &
    VSC73XX_CHIPID_ID_MASK;
    switch (id) {
    case VSC73XX_CHIPID_ID_7385:
    case VSC73XX_CHIPID_ID_7388:
    case VSC73XX_CHIPID_ID_7395:
    case VSC73XX_CHIPID_ID_7398:
    break;
    default:
    dev_err(vsc.dev, "unsupported chip, id=%04x\n", id);
    return -ENODEV;
    }
    vsc.chipid = id;
    rev = (val >> VSC73XX_CHIPID_REV_SHIFT) &
    VSC73XX_CHIPID_REV_MASK;
    dev_info(vsc.dev, "VSC%04X (rev: %d) switch found\n", id, rev);
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_ICPU_CTRL, &val);
    if (ret) {
    dev_err(vsc.dev, "unable to read iCPU control\n");
    return ret;
    }
// The iCPU can always be used but can boot in different ways.
// If it is initially disabled and has no external memory,
// we are in control and can do whatever we like, else we
// are probably in trouble (we need some way to communicate
// with the running firmware) so we bail out for now.
//
    icpu_pi_en = !!(val & VSC73XX_ICPU_CTRL_ICPU_PI_EN);
    icpu_si_boot_en = !!(val & VSC73XX_ICPU_CTRL_BOOT_EN);
    if (icpu_si_boot_en && icpu_pi_en) {
    dev_err(vsc.dev,
    "iCPU enabled boots from SI, has external memory\n");
    dev_err(vsc.dev, "no idea how to deal with this\n");
    return -ENODEV;
    }
    if (icpu_si_boot_en && !icpu_pi_en) {
    dev_err(vsc.dev,
    "iCPU enabled boots from PI/SI, no external memory\n");
    return -EAGAIN;
    }
    if (!icpu_si_boot_en && icpu_pi_en) {
    dev_err(vsc.dev,
    "iCPU enabled, boots from PI external memory\n");
    dev_err(vsc.dev, "no idea how to deal with this\n");
    return -ENODEV;
    }
// !icpu_si_boot_en && !cpu_pi_en
    dev_info(vsc.dev, "iCPU disabled, no external memory\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_mdio_busy_check(vsc: *mut vsc73xx) -> c_int {
    static int vsc73xx_mdio_busy_check(struct vsc73xx *vsc)
    {
    int ret, err;
    u32 val;
    ret = read_poll_timeout(vsc73xx_read, err,
    err < 0 || !(val & VSC73XX_MII_STAT_BUSY),
    VSC73XX_MDIO_POLL_SLEEP_US,
    VSC73XX_POLL_TIMEOUT_US, false, vsc,
    VSC73XX_BLOCK_MII, VSC73XX_BLOCK_MII_INTERNAL,
    VSC73XX_MII_STAT, &val);
    if (ret)
    return ret;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_phy_read(ds: *mut dsa_switch, phy: c_int, regnum: c_int) -> c_int {
    static int vsc73xx_phy_read(struct dsa_switch *ds, int phy, int regnum)
    {
    struct vsc73xx *vsc = ds.priv;
    u32 cmd;
    u32 val;
    int ret;
    ret = vsc73xx_mdio_busy_check(vsc);
    if (ret)
    return ret;
// Setting bit 26 means "read"
    cmd = VSC73XX_MII_CMD_OPERATION |
    FIELD_PREP(VSC73XX_MII_CMD_PHY_ADDR, phy) |
    FIELD_PREP(VSC73XX_MII_CMD_PHY_REG, regnum);
    ret = vsc73xx_write(vsc, VSC73XX_BLOCK_MII, VSC73XX_BLOCK_MII_INTERNAL,
    VSC73XX_MII_CMD, cmd);
    if (ret)
    return ret;
    ret = vsc73xx_mdio_busy_check(vsc);
    if (ret)
    return ret;
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_MII, VSC73XX_BLOCK_MII_INTERNAL,
    VSC73XX_MII_DATA, &val);
    if (ret)
    return ret;
    if (val & VSC73XX_MII_DATA_FAILURE) {
    dev_err(vsc.dev, "reading reg %02x from phy%d failed\n",
    regnum, phy);
    return -EIO;
    }
    val &= VSC73XX_MII_DATA_READ_DATA;
    dev_dbg(vsc.dev, "read reg %02x from phy%d = %04x\n",
    regnum, phy, val);
    return val;
    }
    static int vsc73xx_phy_write(struct dsa_switch *ds, int phy, int regnum,
    u16 val)
    {
    struct vsc73xx *vsc = ds.priv;
    u32 cmd;
    int ret;
    ret = vsc73xx_mdio_busy_check(vsc);
    if (ret)
    return ret;
    cmd = FIELD_PREP(VSC73XX_MII_CMD_PHY_ADDR, phy) |
    FIELD_PREP(VSC73XX_MII_CMD_PHY_REG, regnum) |
    FIELD_PREP(VSC73XX_MII_CMD_WRITE_DATA, val);
    ret = vsc73xx_write(vsc, VSC73XX_BLOCK_MII, VSC73XX_BLOCK_MII_INTERNAL,
    VSC73XX_MII_CMD, cmd);
    if (ret)
    return ret;
    dev_dbg(vsc.dev, "write %04x to reg %02x in phy%d\n",
    val, regnum, phy);
    return 0;
    }
    static enum dsa_tag_protocol vsc73xx_get_tag_protocol(struct dsa_switch *ds,
    int port,
    enum dsa_tag_protocol mp)
    {
// The switch internally uses a 8 byte header with length,
// source port, tag, LPA and priority. This is supposedly
// only accessible when operating the switch using the internal
// CPU or with an external CPU mapping the device in, but not
// when operating the switch over SPI and putting frames in/out
// on port 6 (the CPU port). So far we must assume that we
// cannot access the tag. (See "Internal frame header" section
// 3.9.1 in the manual.)
//
    return DSA_TAG_PROTO_VSC73XX_8021Q;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_wait_for_vlan_table_cmd(vsc: *mut vsc73xx) -> c_int {
    static int vsc73xx_wait_for_vlan_table_cmd(struct vsc73xx *vsc)
    {
    int ret, err;
    u32 val;
    ret = read_poll_timeout(vsc73xx_read, err,
    err < 0 ||
    ((val & VSC73XX_VLANACCESS_VLAN_TBL_CMD_MASK) ==
    VSC73XX_VLANACCESS_VLAN_TBL_CMD_IDLE),
    VSC73XX_POLL_SLEEP_US, VSC73XX_POLL_TIMEOUT_US,
    false, vsc, VSC73XX_BLOCK_ANALYZER,
    0, VSC73XX_VLANACCESS, &val);
    if (ret)
    return ret;
    return err;
    }
    static int
    vsc73xx_read_vlan_table_entry(struct vsc73xx *vsc, u16 vid, u8 *portmap)
    {
    u32 val;
    int ret;
    vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_VLANTIDX, vid);
    ret = vsc73xx_wait_for_vlan_table_cmd(vsc);
    if (ret)
    return ret;
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_VLANACCESS,
    VSC73XX_VLANACCESS_VLAN_TBL_CMD_MASK,
    VSC73XX_VLANACCESS_VLAN_TBL_CMD_READ_ENTRY);
    ret = vsc73xx_wait_for_vlan_table_cmd(vsc);
    if (ret)
    return ret;
    vsc73xx_read(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_VLANACCESS, &val);
// portmap = (val & VSC73XX_VLANACCESS_VLAN_PORT_MASK) >>
    VSC73XX_VLANACCESS_VLAN_PORT_MASK_SHIFT;
    return 0;
    }
    static int
    vsc73xx_write_vlan_table_entry(struct vsc73xx *vsc, u16 vid, u8 portmap)
    {
    int ret;
    vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_VLANTIDX, vid);
    ret = vsc73xx_wait_for_vlan_table_cmd(vsc);
    if (ret)
    return ret;
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_VLANACCESS,
    VSC73XX_VLANACCESS_VLAN_TBL_CMD_MASK |
    VSC73XX_VLANACCESS_VLAN_SRC_CHECK |
    VSC73XX_VLANACCESS_VLAN_PORT_MASK,
    VSC73XX_VLANACCESS_VLAN_TBL_CMD_WRITE_ENTRY |
    VSC73XX_VLANACCESS_VLAN_SRC_CHECK |
    (portmap << VSC73XX_VLANACCESS_VLAN_PORT_MASK_SHIFT));
    return vsc73xx_wait_for_vlan_table_cmd(vsc);
    }
    static int
    vsc73xx_update_vlan_table(struct vsc73xx *vsc, int port, u16 vid, bool set)
    {
    u8 portmap;
    int ret;
    ret = vsc73xx_read_vlan_table_entry(vsc, vid, &portmap);
    if (ret)
    return ret;
    if (set)
    portmap |= BIT(port);
    else
    portmap &= ~BIT(port);
    return vsc73xx_write_vlan_table_entry(vsc, vid, portmap);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_configure_rgmii_port_delay(ds: *mut dsa_switch) -> c_int {
    static int vsc73xx_configure_rgmii_port_delay(struct dsa_switch *ds)
    {
// Keep 2.0 ns delay for backward complatibility
    let mut tx_delay: u32 = VSC73XX_GMIIDELAY_GMII0_GTXDELAY_2_0_NS;
    let mut rx_delay: u32 = VSC73XX_GMIIDELAY_GMII0_RXDELAY_2_0_NS;
    struct dsa_port *dp = dsa_to_port(ds, CPU_PORT);
    struct device_node *port_dn = dp.dn;
    struct vsc73xx *vsc = ds.priv;
    u32 delay;
    if (!of_property_read_u32(port_dn, "tx-internal-delay-ps", &delay)) {
    switch (delay) {
    case 0:
    tx_delay = VSC73XX_GMIIDELAY_GMII0_GTXDELAY_NONE;
    break;
    case 1400:
    tx_delay = VSC73XX_GMIIDELAY_GMII0_GTXDELAY_1_4_NS;
    break;
    case 1700:
    tx_delay = VSC73XX_GMIIDELAY_GMII0_GTXDELAY_1_7_NS;
    break;
    case 2000:
    break;
    default:
    dev_err(vsc.dev,
    "Unsupported RGMII Transmit Clock Delay\n");
    return -EINVAL;
    }
    } else {
    dev_dbg(vsc.dev,
    "RGMII Transmit Clock Delay isn't configured, set to 2.0 ns\n");
    }
    if (!of_property_read_u32(port_dn, "rx-internal-delay-ps", &delay)) {
    switch (delay) {
    case 0:
    rx_delay = VSC73XX_GMIIDELAY_GMII0_RXDELAY_NONE;
    break;
    case 1400:
    rx_delay = VSC73XX_GMIIDELAY_GMII0_RXDELAY_1_4_NS;
    break;
    case 1700:
    rx_delay = VSC73XX_GMIIDELAY_GMII0_RXDELAY_1_7_NS;
    break;
    case 2000:
    break;
    default:
    dev_err(vsc.dev,
    "Unsupported RGMII Receive Clock Delay value\n");
    return -EINVAL;
    }
    } else {
    dev_dbg(vsc.dev,
    "RGMII Receive Clock Delay isn't configured, set to 2.0 ns\n");
    }
// MII delay, set both GTX and RX delay
    return vsc73xx_write(vsc, VSC73XX_BLOCK_SYSTEM, 0, VSC73XX_GMIIDELAY,
    tx_delay | rx_delay);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_setup(ds: *mut dsa_switch) -> c_int {
    static int vsc73xx_setup(struct dsa_switch *ds)
    {
    struct vsc73xx *vsc = ds.priv;
    int i, ret, val;
    dev_info(vsc.dev, "set up the switch\n");
    ds.max_num_bridges = DSA_TAG_8021Q_MAX_NUM_BRIDGES;
    ds.fdb_isolation = true;
// Issue RESET
    vsc73xx_write(vsc, VSC73XX_BLOCK_SYSTEM, 0, VSC73XX_GLORESET,
    VSC73XX_GLORESET_MASTER_RESET);
    usleep_range(125, 200);
// Initialize memory, initialize RAM bank 0..15 except 6 and 7
// This sequence appears in the
// VSC7385 SparX-G5 datasheet section 6.6.1
// VSC7395 SparX-G5e datasheet section 6.6.1
// "initialization sequence".
// No explanation is given to the 0x1010400 magic number.
//
    for (i = 0; i <= 15; i++) {
    if (i != 6 && i != 7) {
    vsc73xx_write(vsc, VSC73XX_BLOCK_MEMINIT,
    2,
    0, 0x1010400 + i);
    mdelay(1);
    }
    }
    mdelay(30);
// Clear MAC table
    vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACACCESS,
    VSC73XX_MACACCESS_CMD_CLEAR_TABLE);
// Set VLAN table to default values
    vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_VLANACCESS,
    VSC73XX_VLANACCESS_VLAN_TBL_CMD_CLEAR_TABLE);
    msleep(40);
// Use 20KiB buffers on all ports on VSC7395
// The VSC7385 has 16KiB buffers and that is the
// default if we don't set this up explicitly.
// Port "31" is "all ports".
//
    if (IS_739X(vsc))
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, 0x1f,
    VSC73XX_Q_MISC_CONF,
    VSC73XX_Q_MISC_CONF_EXTENT_MEM);
// Put all ports into reset until enabled
    for (i = 0; i < 7; i++) {
    if (i == 5)
    continue;
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, 4,
    VSC73XX_MAC_CFG, VSC73XX_MAC_CFG_RESET);
    }
// Configure RGMII delay
    ret = vsc73xx_configure_rgmii_port_delay(ds);
    if (ret)
    return ret;
// Ingess VLAN reception mask (table 145)
    vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_VLANMASK,
    0xff);
// IP multicast flood mask (table 144)
    vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_IFLODMSK,
    0xff);
    mdelay(50);
// Disable preamble and use maximum allowed clock for the internal
// mdio bus, used for communication with internal PHYs only.
//
    val = VSC73XX_MII_MPRES_NOPREAMBLE |
    FIELD_PREP(VSC73XX_MII_MPRES_PRESCALEVAL,
    VSC73XX_MII_PRESCALEVAL_MIN);
    vsc73xx_write(vsc, VSC73XX_BLOCK_MII, VSC73XX_BLOCK_MII_INTERNAL,
    VSC73XX_MII_MPRES, val);
// Release reset from the internal PHYs
    vsc73xx_write(vsc, VSC73XX_BLOCK_SYSTEM, 0, VSC73XX_GLORESET,
    VSC73XX_GLORESET_PHY_RESET);
    udelay(4);
// Clear VLAN table
    for (i = 0; i < VLAN_N_VID; i++)
    vsc73xx_write_vlan_table_entry(vsc, i, 0);
    INIT_LIST_HEAD(&vsc.vlans);
    rtnl_lock();
    ret = dsa_tag_8021q_register(ds, htons(ETH_P_8021Q));
    rtnl_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_teardown(ds: *mut dsa_switch) {
    static void vsc73xx_teardown(struct dsa_switch *ds)
    {
    rtnl_lock();
    dsa_tag_8021q_unregister(ds);
    rtnl_unlock();
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_init_port(vsc: *mut vsc73xx, port: c_int) {
    static void vsc73xx_init_port(struct vsc73xx *vsc, int port)
    {
    u32 val;
// MAC configure, first reset the port and then write defaults
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_MAC_CFG,
    VSC73XX_MAC_CFG_RESET);
// Take up the port in 1Gbit mode by default, this will be
// augmented after auto-negotiation on the PHY-facing
// ports.
//
    if (port == CPU_PORT)
    val = VSC73XX_MAC_CFG_1000M_F_RGMII;
    else
    val = VSC73XX_MAC_CFG_1000M_F_PHY;
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_MAC_CFG,
    val |
    VSC73XX_MAC_CFG_TX_EN |
    VSC73XX_MAC_CFG_RX_EN);
// Flow control for the CPU port:
// Use a zero delay pause frame when pause condition is left
// Obey pause control frames
//
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_FCCONF,
    VSC73XX_FCCONF_ZERO_PAUSE_EN |
    VSC73XX_FCCONF_FLOW_CTRL_OBEY);
// Issue pause control frames on PHY facing ports.
// Allow early initiation of MAC transmission if the amount
// of egress data is below 512 bytes on CPU port.
// FIXME: enable 20KiB buffers?
//
    if (port == CPU_PORT)
    val = VSC73XX_Q_MISC_CONF_EARLY_TX_512;
    else
    val = VSC73XX_Q_MISC_CONF_MAC_PAUSE_MODE;
    val |= VSC73XX_Q_MISC_CONF_EXTENT_MEM;
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_Q_MISC_CONF,
    val);
// Flow control MAC: a MAC address used in flow control frames
    val = (vsc.addr[5] << 16) | (vsc.addr[4] << 8) | (vsc.addr[3]);
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_FCMACHI,
    val);
    val = (vsc.addr[2] << 16) | (vsc.addr[1] << 8) | (vsc.addr[0]);
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_FCMACLO,
    val);
// Tell the categorizer to forward pause frames, not control
// frame. Do not drop anything.
//
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port,
    VSC73XX_CAT_DROP,
    VSC73XX_CAT_DROP_FWD_PAUSE_ENA);
// Clear all counters
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    port, VSC73XX_C_RX0, 0);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_reset_port(vsc: *mut vsc73xx, port: c_int, initval: u32) {
    static void vsc73xx_reset_port(struct vsc73xx *vsc, int port, u32 initval)
    {
    int ret, err;
    u32 val;
// Disable RX on this port
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_MAC_CFG,
    VSC73XX_MAC_CFG_RX_EN, 0);
// Discard packets
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ARBITER, 0,
    VSC73XX_ARBDISC, BIT(port), BIT(port));
// Wait until queue is empty
    ret = read_poll_timeout(vsc73xx_read, err,
    err < 0 || (val & BIT(port)),
    VSC73XX_POLL_SLEEP_US,
    VSC73XX_POLL_TIMEOUT_US, false,
    vsc, VSC73XX_BLOCK_ARBITER, 0,
    VSC73XX_ARBEMPTY, &val);
    if (ret)
    dev_err(vsc.dev,
    "timeout waiting for block arbiter\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: err <) -> else {
    else if (err < 0)
    dev_err(vsc.dev, "error reading arbiter\n");
// Put this port into reset
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, port, VSC73XX_MAC_CFG,
    VSC73XX_MAC_CFG_RESET | initval);
    }
    static void vsc73xx_mac_config(struct phylink_config *config, unsigned int mode,
    const struct phylink_link_state *state)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct vsc73xx *vsc = dp.ds.priv;
    let mut port: c_int = dp.index;
// Special handling of the CPU-facing port
    if (port == CPU_PORT) {
// Other ports are already initialized but not this one
    vsc73xx_init_port(vsc, CPU_PORT);
// Select the external port for this interface (EXT_PORT)
// Enable the GMII GTX external clock
// Use double data rate (DDR mode)
//
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC,
    CPU_PORT,
    VSC73XX_ADVPORTM,
    VSC73XX_ADVPORTM_EXT_PORT |
    VSC73XX_ADVPORTM_ENA_GTX |
    VSC73XX_ADVPORTM_DDR_MODE);
    }
    }
    static void vsc73xx_mac_link_down(struct phylink_config *config,
    unsigned int mode, phy_interface_t interface)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct vsc73xx *vsc = dp.ds.priv;
    let mut port: c_int = dp.index;
// This routine is described in the datasheet (below ARBDISC register
// description)
//
    vsc73xx_reset_port(vsc, port, 0);
// Allow backward dropping of frames from this port
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ARBITER, 0,
    VSC73XX_SBACKWDROP, BIT(port), BIT(port));
    }
    static void vsc73xx_mac_link_up(struct phylink_config *config,
    struct phy_device *phy, unsigned int mode,
    phy_interface_t interface, int speed,
    int duplex, bool tx_pause, bool rx_pause)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct vsc73xx *vsc = dp.ds.priv;
    let mut port: c_int = dp.index;
    u32 val;
    u8 seed;
    if (speed == SPEED_1000)
    val = VSC73XX_MAC_CFG_GIGA_MODE | VSC73XX_MAC_CFG_TX_IPG_1000M;
    else
    val = VSC73XX_MAC_CFG_TX_IPG_100_10M;
    if (phy_interface_mode_is_rgmii(interface))
    val |= VSC73XX_MAC_CFG_CLK_SEL_1000M;
    else
    val |= VSC73XX_MAC_CFG_CLK_SEL_EXT;
    if (duplex == DUPLEX_FULL)
    val |= VSC73XX_MAC_CFG_FDX;
    else
// In datasheet description ("Port Mode Procedure" in 5.6.2)
// this bit is configured only for half duplex.
//
    val |= VSC73XX_MAC_CFG_WEXC_DIS;
// This routine is described in the datasheet (below ARBDISC register
// description)
//
    vsc73xx_reset_port(vsc, port, val);
// Seed the port randomness with randomness
    get_random_bytes(&seed, 1);
    val |= seed << VSC73XX_MAC_CFG_SEED_OFFSET;
    val |= VSC73XX_MAC_CFG_SEED_LOAD;
// Those bits are responsible for MTU only. Kernel takes care about MTU,
// let's enable +8 bytes frame length unconditionally.
//
    val |= VSC73XX_MAC_CFG_VLAN_AWR | VSC73XX_MAC_CFG_VLAN_DBLAWR;
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, port, VSC73XX_MAC_CFG, val);
// Flow control for the PHY facing ports:
// Use a zero delay pause frame when pause condition is left
// Obey pause control frames
// When generating pause frames, use 0xff as pause value
//
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, port, VSC73XX_FCCONF,
    VSC73XX_FCCONF_ZERO_PAUSE_EN |
    VSC73XX_FCCONF_FLOW_CTRL_OBEY |
    0xff);
// Accept packets again
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ARBITER, 0,
    VSC73XX_ARBDISC, BIT(port), 0);
// Disallow backward dropping of frames from this port
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ARBITER, 0,
    VSC73XX_SBACKWDROP, BIT(port), 0);
// Enable TX, RX, deassert reset, stop loading seed
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_MAC_CFG,
    VSC73XX_MAC_CFG_RESET | VSC73XX_MAC_CFG_SEED_LOAD |
    VSC73XX_MAC_CFG_TX_EN | VSC73XX_MAC_CFG_RX_EN,
    VSC73XX_MAC_CFG_TX_EN | VSC73XX_MAC_CFG_RX_EN);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_tag_8021q_active(dp: *mut dsa_port) -> bool {
    static bool vsc73xx_tag_8021q_active(struct dsa_port *dp)
    {
    return !dsa_port_is_vlan_filtering(dp);
    }
    static struct vsc73xx_bridge_vlan *
    vsc73xx_bridge_vlan_find(struct vsc73xx *vsc, u16 vid)
    {
    struct vsc73xx_bridge_vlan *vlan;
    list_for_each_entry(vlan, &vsc.vlans, list)
    if (vlan.vid == vid)
    return vlan;
    return core::ptr::null_mut();
    }
    static void
    vsc73xx_bridge_vlan_remove_port(struct vsc73xx_bridge_vlan *vsc73xx_vlan,
    int port)
    {
    vsc73xx_vlan.portmask &= ~BIT(port);
    if (vsc73xx_vlan.portmask)
    return;
    list_del(&vsc73xx_vlan.list);
    kfree(vsc73xx_vlan);
    }
    static void vsc73xx_bridge_vlan_summary(struct vsc73xx *vsc, int port,
    struct vsc73xx_vlan_summary *summary,
    u16 ignored_vid)
    {
    let mut num_tagged: usize = 0, num_untagged = 0;
    struct vsc73xx_bridge_vlan *vlan;
    list_for_each_entry(vlan, &vsc.vlans, list) {
    if (!(vlan.portmask & BIT(port)) || vlan.vid == ignored_vid)
    continue;
    if (vlan.untagged & BIT(port))
    num_untagged++;
    else
    num_tagged++;
    }
    summary.num_untagged = num_untagged;
    summary.num_tagged = num_tagged;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_find_first_vlan_untagged(vsc: *mut vsc73xx, port: c_int) -> u16 {
    static u16 vsc73xx_find_first_vlan_untagged(struct vsc73xx *vsc, int port)
    {
    struct vsc73xx_bridge_vlan *vlan;
    list_for_each_entry(vlan, &vsc.vlans, list)
    if ((vlan.portmask & BIT(port)) &&
    (vlan.untagged & BIT(port)))
    return vlan.vid;
    return VLAN_N_VID;
    }
    static int vsc73xx_set_vlan_conf(struct vsc73xx *vsc, int port,
    enum vsc73xx_port_vlan_conf port_vlan_conf)
    {
    let mut val: u32 = 0;
    int ret;
    if (port_vlan_conf == VSC73XX_VLAN_IGNORE)
    val = VSC73XX_CAT_VLAN_MISC_VLAN_TCI_IGNORE_ENA |
    VSC73XX_CAT_VLAN_MISC_VLAN_KEEP_TAG_ENA;
    ret = vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_CAT_VLAN_MISC,
    VSC73XX_CAT_VLAN_MISC_VLAN_TCI_IGNORE_ENA |
    VSC73XX_CAT_VLAN_MISC_VLAN_KEEP_TAG_ENA, val);
    if (ret)
    return ret;
    val = (port_vlan_conf == VSC73XX_VLAN_FILTER) ?
    VSC73XX_TXUPDCFG_TX_INSERT_TAG : 0;
    return vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_TXUPDCFG,
    VSC73XX_TXUPDCFG_TX_INSERT_TAG, val);
    }
//
// vsc73xx_vlan_commit_conf - Update VLAN configuration of a port
// @vsc: Switch private data structure
// @port: Port index on which to operate
//
// Update the VLAN behavior of a port to make sure that when it is under
// a VLAN filtering bridge, the port is either filtering with tag
// preservation, or filtering with all VLANs egress-untagged. Otherwise,
// the port ignores VLAN tags from packets and applies the port-based
// VID.
//
// Must be called when changes are made to:
// - the bridge VLAN filtering state of the port
// - the number or attributes of VLANs from the bridge VLAN table,
// while the port is currently VLAN-aware
//
// Return: 0 on success, or negative errno on error.
//
#[no_mangle]
unsafe extern "C" fn vsc73xx_vlan_commit_conf(vsc: *mut vsc73xx, port: c_int) -> c_int {
    static int vsc73xx_vlan_commit_conf(struct vsc73xx *vsc, int port)
    {
    let mut port_vlan_conf: enum vsc73xx_port_vlan_conf = VSC73XX_VLAN_IGNORE;
    struct dsa_port *dp = dsa_to_port(vsc.ds, port);
    if (port == CPU_PORT) {
    port_vlan_conf = VSC73XX_VLAN_FILTER;
    } else if (dsa_port_is_vlan_filtering(dp)) {
    struct vsc73xx_vlan_summary summary;
    port_vlan_conf = VSC73XX_VLAN_FILTER;
    vsc73xx_bridge_vlan_summary(vsc, port, &summary, VLAN_N_VID);
    if (summary.num_tagged == 0)
    port_vlan_conf = VSC73XX_VLAN_FILTER_UNTAG_ALL;
    }
    return vsc73xx_set_vlan_conf(vsc, port, port_vlan_conf);
    }
    static int
    vsc73xx_vlan_change_untagged(struct vsc73xx *vsc, int port, u16 vid, bool set)
    {
    let mut val: u32 = 0;
    if (set)
    val = VSC73XX_TXUPDCFG_TX_UNTAGGED_VID_ENA |
    ((vid << VSC73XX_TXUPDCFG_TX_UNTAGGED_VID_SHIFT) &
    VSC73XX_TXUPDCFG_TX_UNTAGGED_VID);
    return vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_TXUPDCFG,
    VSC73XX_TXUPDCFG_TX_UNTAGGED_VID_ENA |
    VSC73XX_TXUPDCFG_TX_UNTAGGED_VID, val);
    }
//
// vsc73xx_vlan_commit_untagged - Update native VLAN of a port
// @vsc: Switch private data structure
// @port: Port index on which to operate
//
// Update the native VLAN of a port (the one VLAN which is transmitted
// as egress-tagged on a trunk port) when port is in VLAN filtering mode and
// only one untagged vid is configured.
// In other cases no need to configure it because switch can untag all vlans on
// the port.
//
// Return: 0 on success, or negative errno on error.
//
#[no_mangle]
unsafe extern "C" fn vsc73xx_vlan_commit_untagged(vsc: *mut vsc73xx, port: c_int) -> c_int {
    static int vsc73xx_vlan_commit_untagged(struct vsc73xx *vsc, int port)
    {
    struct dsa_port *dp = dsa_to_port(vsc.ds, port);
    struct vsc73xx_vlan_summary summary;
    let mut vid: u16 = 0;
    bool valid;
    if (!dsa_port_is_vlan_filtering(dp))
// Port is configured to untag all vlans in that case.
// No need to commit untagged config change.
//
    return 0;
    vsc73xx_bridge_vlan_summary(vsc, port, &summary, VLAN_N_VID);
    if (summary.num_untagged > 1)
// Port must untag all vlans in that case.
// No need to commit untagged config change.
//
    return 0;
    valid = (summary.num_untagged == 1);
    if (valid)
    vid = vsc73xx_find_first_vlan_untagged(vsc, port);
    return vsc73xx_vlan_change_untagged(vsc, port, vid, valid);
    }
    static int
    vsc73xx_vlan_change_pvid(struct vsc73xx *vsc, int port, u16 vid, bool set)
    {
    let mut val: u32 = 0;
    int ret;
    val = set ? 0 : VSC73XX_CAT_DROP_UNTAGGED_ENA;
    ret = vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_CAT_DROP,
    VSC73XX_CAT_DROP_UNTAGGED_ENA, val);
    if (!set || ret)
    return ret;
    return vsc73xx_update_bits(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_CAT_PORT_VLAN,
    VSC73XX_CAT_PORT_VLAN_VLAN_VID,
    vid & VSC73XX_CAT_PORT_VLAN_VLAN_VID);
    }
//
// vsc73xx_vlan_commit_pvid - Update port-based default VLAN of a port
// @vsc: Switch private data structure
// @port: Port index on which to operate
//
// Update the PVID of a port so that it follows either the bridge PVID
// configuration, when the bridge is currently VLAN-aware, or the PVID
// from tag_8021q, when the port is standalone or under a VLAN-unaware
// bridge. A port with no PVID drops all untagged and VID 0 tagged
// traffic.
//
// Must be called when changes are made to:
// - the bridge VLAN filtering state of the port
// - the number or attributes of VLANs from the bridge VLAN table,
// while the port is currently VLAN-aware
//
// Return: 0 on success, or negative errno on error.
//
#[no_mangle]
unsafe extern "C" fn vsc73xx_vlan_commit_pvid(vsc: *mut vsc73xx, port: c_int) -> c_int {
    static int vsc73xx_vlan_commit_pvid(struct vsc73xx *vsc, int port)
    {
    struct vsc73xx_portinfo *portinfo = &vsc.portinfo[port];
    let mut valid: bool = portinfo.pvid_tag_8021q_configured;
    struct dsa_port *dp = dsa_to_port(vsc.ds, port);
    let mut vid: u16 = portinfo.pvid_tag_8021q;
    if (dsa_port_is_vlan_filtering(dp)) {
    vid = portinfo.pvid_vlan_filtering;
    valid = portinfo.pvid_vlan_filtering_configured;
    }
    return vsc73xx_vlan_change_pvid(vsc, port, vid, valid);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_vlan_commit_settings(vsc: *mut vsc73xx, port: c_int) -> c_int {
    static int vsc73xx_vlan_commit_settings(struct vsc73xx *vsc, int port)
    {
    int ret;
    ret = vsc73xx_vlan_commit_untagged(vsc, port);
    if (ret)
    return ret;
    ret = vsc73xx_vlan_commit_pvid(vsc, port);
    if (ret)
    return ret;
    return vsc73xx_vlan_commit_conf(vsc, port);
    }
    static int vsc73xx_port_enable(struct dsa_switch *ds, int port,
    struct phy_device *phy)
    {
    struct vsc73xx *vsc = ds.priv;
    dev_info(vsc.dev, "enable port %d\n", port);
    vsc73xx_init_port(vsc, port);
    return vsc73xx_vlan_commit_settings(vsc, port);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_port_disable(ds: *mut dsa_switch, port: c_int) {
    static void vsc73xx_port_disable(struct dsa_switch *ds, int port)
    {
    struct vsc73xx *vsc = ds.priv;
// Just put the port into reset
    vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_MAC_CFG, VSC73XX_MAC_CFG_RESET);
    }
    static const struct vsc73xx_counter *
    vsc73xx_find_counter(struct vsc73xx *vsc,
    u8 counter,
    bool tx)
    {
    const struct vsc73xx_counter *cnts;
    int num_cnts;
    int i;
    if (tx) {
    cnts = vsc73xx_tx_counters;
    num_cnts = ARRAY_SIZE(vsc73xx_tx_counters);
    } else {
    cnts = vsc73xx_rx_counters;
    num_cnts = ARRAY_SIZE(vsc73xx_rx_counters);
    }
    for (i = 0; i < num_cnts; i++) {
    const struct vsc73xx_counter *cnt;
    cnt = &cnts[i];
    if (cnt.counter == counter)
    return cnt;
    }
    return core::ptr::null_mut();
    }
    static void vsc73xx_get_strings(struct dsa_switch *ds, int port, u32 stringset,
    uint8_t *data)
    {
    const struct vsc73xx_counter *cnt;
    struct vsc73xx *vsc = ds.priv;
    u8 indices[6];
    u8 *buf = data;
    int i;
    u32 val;
    int ret;
    if (stringset != ETH_SS_STATS)
    return;
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_C_CFG, &val);
    if (ret)
    return;
    indices[0] = (val & 0x1f); /* RX counter 0 */
    indices[1] = ((val >> 5) & 0x1f); /* RX counter 1 */
    indices[2] = ((val >> 10) & 0x1f); /* RX counter 2 */
    indices[3] = ((val >> 16) & 0x1f); /* TX counter 0 */
    indices[4] = ((val >> 21) & 0x1f); /* TX counter 1 */
    indices[5] = ((val >> 26) & 0x1f); /* TX counter 2 */
// The first counters is the RX octets
    ethtool_puts(&buf, "RxEtherStatsOctets");
// Each port supports recording 3 RX counters and 3 TX counters,
// figure out what counters we use in this set-up and return the
// names of them. The hardware default counters will be number of
// packets on RX/TX, combined broadcast+multicast packets RX/TX and
// total error packets RX/TX.
//
    for (i = 0; i < 3; i++) {
    cnt = vsc73xx_find_counter(vsc, indices[i], false);
    ethtool_puts(&buf, cnt ? cnt.name : "");
    }
// TX stats begins with the number of TX octets
    ethtool_puts(&buf, "TxEtherStatsOctets");
    for (i = 3; i < 6; i++) {
    cnt = vsc73xx_find_counter(vsc, indices[i], true);
    ethtool_puts(&buf, cnt ? cnt.name : "");
    }
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int {
    static int vsc73xx_get_sset_count(struct dsa_switch *ds, int port, int sset)
    {
// We only support SS_STATS
    if (sset != ETH_SS_STATS)
    return 0;
// RX and TX packets, then 3 RX counters, 3 TX counters
    return 8;
    }
    static void vsc73xx_get_ethtool_stats(struct dsa_switch *ds, int port,
    uint64_t *data)
    {
    struct vsc73xx *vsc = ds.priv;
    u8 regs[] = {
    VSC73XX_RXOCT,
    VSC73XX_C_RX0,
    VSC73XX_C_RX1,
    VSC73XX_C_RX2,
    VSC73XX_TXOCT,
    VSC73XX_C_TX0,
    VSC73XX_C_TX1,
    VSC73XX_C_TX2,
    };
    u32 val;
    int ret;
    int i;
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_MAC, port,
    regs[i], &val);
    if (ret) {
    dev_err(vsc.dev, "error reading counter %d\n", i);
    return;
    }
    data[i] = val;
    }
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_change_mtu(ds: *mut dsa_switch, port: c_int, new_mtu: c_int) -> c_int {
    static int vsc73xx_change_mtu(struct dsa_switch *ds, int port, int new_mtu)
    {
    struct vsc73xx *vsc = ds.priv;
    return vsc73xx_write(vsc, VSC73XX_BLOCK_MAC, port,
    VSC73XX_MAXLEN, new_mtu + ETH_HLEN + ETH_FCS_LEN);
    }
// According to application not "VSC7398 Jumbo Frames" setting
// up the frame size to 9.6 KB does not affect the performance on standard
// frames. It is clear from the application note that
// "9.6 kilobytes" == 9600 bytes.
//
#[no_mangle]
unsafe extern "C" fn vsc73xx_get_max_mtu(ds: *mut dsa_switch, port: c_int) -> c_int {
    static int vsc73xx_get_max_mtu(struct dsa_switch *ds, int port)
    {
    return 9600 - ETH_HLEN - ETH_FCS_LEN;
    }
    static void vsc73xx_phylink_get_caps(struct dsa_switch *dsa, int port,
    struct phylink_config *config)
    {
    unsigned long *interfaces = config.supported_interfaces;
    if (port == 5)
    return;
    if (port == CPU_PORT) {
    __set_bit(PHY_INTERFACE_MODE_MII, interfaces);
    __set_bit(PHY_INTERFACE_MODE_REVMII, interfaces);
    __set_bit(PHY_INTERFACE_MODE_GMII, interfaces);
    __set_bit(PHY_INTERFACE_MODE_RGMII, interfaces);
    }
    if (port <= 4) {
// Internal PHYs
    __set_bit(PHY_INTERFACE_MODE_INTERNAL, interfaces);
// phylib default
    __set_bit(PHY_INTERFACE_MODE_GMII, interfaces);
    }
    config.mac_capabilities = MAC_SYM_PAUSE | MAC_10 | MAC_100 | MAC_1000;
    }
    static int
    vsc73xx_port_vlan_filtering(struct dsa_switch *ds, int port,
    bool vlan_filtering, struct netlink_ext_ack *extack)
    {
    struct vsc73xx *vsc = ds.priv;
// The commit to hardware processed below is required because vsc73xx
// is using tag_8021q. When vlan_filtering is disabled, tag_8021q uses
// pvid/untagged vlans for port recognition. The values configured for
// vlans and pvid/untagged states are stored in portinfo structure.
// When vlan_filtering is enabled, we need to restore pvid/untagged from
// portinfo structure. Analogous routine is processed when
// vlan_filtering is disabled, but values used for tag_8021q are
// restored.
//
    return vsc73xx_vlan_commit_settings(vsc, port);
    }
    static int vsc73xx_port_vlan_add(struct dsa_switch *ds, int port,
    const struct switchdev_obj_port_vlan *vlan,
    struct netlink_ext_ack *extack)
    {
    let mut untagged: bool = vlan.flags & BRIDGE_VLAN_INFO_UNTAGGED;
    let mut pvid: bool = vlan.flags & BRIDGE_VLAN_INFO_PVID;
    struct dsa_port *dp = dsa_to_port(ds, port);
    struct vsc73xx_bridge_vlan *vsc73xx_vlan;
    struct vsc73xx_vlan_summary summary;
    struct vsc73xx_portinfo *portinfo;
    struct vsc73xx *vsc = ds.priv;
    bool commit_to_hardware;
    let mut ret: c_int = 0;
// Be sure to deny alterations to the configuration done by tag_8021q.
//
    if (vid_is_dsa_8021q(vlan.vid)) {
    NL_SET_ERR_MSG_MOD(extack,
    "Range 3072-4095 reserved for dsa_8021q operation");
    return -EBUSY;
    }
// The processed vlan->vid is excluded from the search because the VLAN
// can be re-added with a different set of flags, so it's easiest to
// ignore its old flags from the VLAN database software copy.
//
    vsc73xx_bridge_vlan_summary(vsc, port, &summary, vlan.vid);
// VSC73XX allows only three untagged states: none, one or all
    if ((untagged && summary.num_tagged > 0 && summary.num_untagged > 0) ||
    (!untagged && summary.num_untagged > 1)) {
    NL_SET_ERR_MSG_MOD(extack,
    "Port can have only none, one or all untagged vlan");
    return -EBUSY;
    }
    vsc73xx_vlan = vsc73xx_bridge_vlan_find(vsc, vlan.vid);
    if (!vsc73xx_vlan) {
    vsc73xx_vlan = kzalloc_obj(*vsc73xx_vlan);
    if (!vsc73xx_vlan)
    return -ENOMEM;
    vsc73xx_vlan.vid = vlan.vid;
    list_add_tail(&vsc73xx_vlan.list, &vsc.vlans);
    }
    vsc73xx_vlan.portmask |= BIT(port);
// CPU port must be always tagged because source port identification is
// based on tag_8021q.
//
    if (port == CPU_PORT)
    goto update_vlan_table;
    if (untagged)
    vsc73xx_vlan.untagged |= BIT(port);
    else
    vsc73xx_vlan.untagged &= ~BIT(port);
    portinfo = &vsc.portinfo[port];
    if (pvid) {
    portinfo.pvid_vlan_filtering_configured = true;
    portinfo.pvid_vlan_filtering = vlan.vid;
    } else if (portinfo.pvid_vlan_filtering_configured &&
    portinfo.pvid_vlan_filtering == vlan.vid) {
    portinfo.pvid_vlan_filtering_configured = false;
    }
    commit_to_hardware = !vsc73xx_tag_8021q_active(dp);
    if (commit_to_hardware) {
    ret = vsc73xx_vlan_commit_settings(vsc, port);
    if (ret)
    goto err;
    }
    update_vlan_table:
    ret = vsc73xx_update_vlan_table(vsc, port, vlan.vid, true);
    if (!ret)
    return 0;
    err:
    vsc73xx_bridge_vlan_remove_port(vsc73xx_vlan, port);
    return ret;
    }
    static int vsc73xx_port_vlan_del(struct dsa_switch *ds, int port,
    const struct switchdev_obj_port_vlan *vlan)
    {
    struct vsc73xx_bridge_vlan *vsc73xx_vlan;
    struct vsc73xx_portinfo *portinfo;
    struct vsc73xx *vsc = ds.priv;
    bool commit_to_hardware;
    int ret;
    ret = vsc73xx_update_vlan_table(vsc, port, vlan.vid, false);
    if (ret)
    return ret;
    portinfo = &vsc.portinfo[port];
    if (portinfo.pvid_vlan_filtering_configured &&
    portinfo.pvid_vlan_filtering == vlan.vid)
    portinfo.pvid_vlan_filtering_configured = false;
    vsc73xx_vlan = vsc73xx_bridge_vlan_find(vsc, vlan.vid);
    if (vsc73xx_vlan)
    vsc73xx_bridge_vlan_remove_port(vsc73xx_vlan, port);
    commit_to_hardware = !vsc73xx_tag_8021q_active(dsa_to_port(ds, port));
    if (commit_to_hardware)
    return vsc73xx_vlan_commit_settings(vsc, port);
    return 0;
    }
    static int vsc73xx_tag_8021q_vlan_add(struct dsa_switch *ds, int port, u16 vid,
    u16 flags)
    {
    let mut pvid: bool = flags & BRIDGE_VLAN_INFO_PVID;
    struct vsc73xx_portinfo *portinfo;
    struct vsc73xx *vsc = ds.priv;
    bool commit_to_hardware;
    int ret;
    portinfo = &vsc.portinfo[port];
    if (pvid) {
    portinfo.pvid_tag_8021q_configured = true;
    portinfo.pvid_tag_8021q = vid;
    }
    commit_to_hardware = vsc73xx_tag_8021q_active(dsa_to_port(ds, port));
    if (commit_to_hardware) {
    ret = vsc73xx_vlan_commit_settings(vsc, port);
    if (ret)
    return ret;
    }
    return vsc73xx_update_vlan_table(vsc, port, vid, true);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_tag_8021q_vlan_del(ds: *mut dsa_switch, port: c_int, vid: u16) -> c_int {
    static int vsc73xx_tag_8021q_vlan_del(struct dsa_switch *ds, int port, u16 vid)
    {
    struct vsc73xx_portinfo *portinfo;
    struct vsc73xx *vsc = ds.priv;
    portinfo = &vsc.portinfo[port];
    if (portinfo.pvid_tag_8021q_configured &&
    portinfo.pvid_tag_8021q == vid) {
    struct dsa_port *dp = dsa_to_port(ds, port);
    bool commit_to_hardware;
    int err;
    portinfo.pvid_tag_8021q_configured = false;
    commit_to_hardware = vsc73xx_tag_8021q_active(dp);
    if (commit_to_hardware) {
    err = vsc73xx_vlan_commit_settings(vsc, port);
    if (err)
    return err;
    }
    }
    return vsc73xx_update_vlan_table(vsc, port, vid, false);
    }
    static int vsc73xx_port_pre_bridge_flags(struct dsa_switch *ds, int port,
    struct switchdev_brport_flags flags,
    struct netlink_ext_ack *extack)
    {
    if (flags.mask & ~BR_LEARNING)
    return -EINVAL;
    return 0;
    }
    static int vsc73xx_port_bridge_flags(struct dsa_switch *ds, int port,
    struct switchdev_brport_flags flags,
    struct netlink_ext_ack *extack)
    {
    if (flags.mask & BR_LEARNING) {
    let mut val: u32 = flags.val & BR_LEARNING ? BIT(port) : 0;
    struct vsc73xx *vsc = ds.priv;
    return vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_LEARNMASK, BIT(port), val);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_refresh_fwd_map(ds: *mut dsa_switch, port: c_int, state: u8) {
    static void vsc73xx_refresh_fwd_map(struct dsa_switch *ds, int port, u8 state)
    {
    struct dsa_port *other_dp, *dp = dsa_to_port(ds, port);
    struct vsc73xx *vsc = ds.priv;
    u16 mask;
    if (state != BR_STATE_FORWARDING) {
// Ports that aren't in the forwarding state must not
// forward packets anywhere.
//
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_SRCMASKS + port,
    VSC73XX_SRCMASKS_PORTS_MASK, 0);
    dsa_switch_for_each_available_port(other_dp, ds) {
    if (other_dp == dp)
    continue;
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_SRCMASKS + other_dp.index,
    BIT(port), 0);
    }
    return;
    }
// Forwarding ports must forward to the CPU and to other ports
// in the same bridge
//
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_SRCMASKS + CPU_PORT, BIT(port), BIT(port));
    mask = BIT(CPU_PORT);
    dsa_switch_for_each_user_port(other_dp, ds) {
    let mut other_port: c_int = other_dp.index;
    if (port == other_port || !dsa_port_bridge_same(dp, other_dp) ||
    other_dp.stp_state != BR_STATE_FORWARDING)
    continue;
    mask |= BIT(other_port);
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_SRCMASKS + other_port,
    BIT(port), BIT(port));
    }
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_SRCMASKS + port,
    VSC73XX_SRCMASKS_PORTS_MASK, mask);
    }
// FIXME: STP frames aren't forwarded at this moment. BPDU frames are
// forwarded only from and to PI/SI interface. For more info see chapter
// 2.7.1 (CPU Forwarding) in datasheet.
// This function is required for tag_8021q operations.
//
    static void vsc73xx_port_stp_state_set(struct dsa_switch *ds, int port,
    u8 state)
    {
    struct dsa_port *dp = dsa_to_port(ds, port);
    struct vsc73xx *vsc = ds.priv;
    let mut val: u32 = 0;
    if (state == BR_STATE_LEARNING || state == BR_STATE_FORWARDING)
    val = dp.learning ? BIT(port) : 0;
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_LEARNMASK, BIT(port), val);
    val = (state == BR_STATE_BLOCKING || state == BR_STATE_DISABLED) ?
    0 : BIT(port);
    vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_RECVMASK, BIT(port), val);
// CPU Port should always forward packets when user ports are forwarding
// so let's configure it from other ports only.
//
    if (port != CPU_PORT)
    vsc73xx_refresh_fwd_map(ds, port, state);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_calc_hash(addr: *const c_uchar, vid: u16) -> u16 {
    static u16 vsc73xx_calc_hash(const unsigned char *addr, u16 vid)
    {
// VID 5-0, MAC 47-44
    u16 hash = FIELD_PREP(VSC73XX_HASH0_VID_TO_MASK,
    FIELD_GET(VSC73XX_HASH0_VID_FROM_MASK, vid)) |
    FIELD_PREP(VSC73XX_HASH0_MAC0_TO_MASK,
    FIELD_GET(VSC73XX_HASH0_MAC0_FROM_MASK, addr[0]));
// MAC 43-33
    hash ^= FIELD_PREP(VSC73XX_HASH1_MAC0_TO_MASK,
    FIELD_GET(VSC73XX_HASH1_MAC0_FROM_MASK, addr[0])) |
    FIELD_PREP(VSC73XX_HASH1_MAC1_TO_MASK,
    FIELD_GET(VSC73XX_HASH1_MAC1_FROM_MASK, addr[1]));
// MAC 32-22
    hash ^= FIELD_PREP(VSC73XX_HASH2_MAC1_TO_MASK,
    FIELD_GET(VSC73XX_HASH2_MAC1_FROM_MASK, addr[1])) |
    FIELD_PREP(VSC73XX_HASH2_MAC2_TO_MASK,
    FIELD_GET(VSC73XX_HASH2_MAC2_FROM_MASK, addr[2])) |
    FIELD_PREP(VSC73XX_HASH2_MAC3_TO_MASK,
    FIELD_GET(VSC73XX_HASH2_MAC3_FROM_MASK, addr[3]));
// MAC 21-11
    hash ^= FIELD_PREP(VSC73XX_HASH3_MAC3_TO_MASK,
    FIELD_GET(VSC73XX_HASH3_MAC3_FROM_MASK, addr[3])) |
    FIELD_PREP(VSC73XX_HASH3_MAC4_TO_MASK,
    FIELD_GET(VSC73XX_HASH3_MAC4_FROM_MASK, addr[4]));
// MAC 10-0
    hash ^= FIELD_PREP(VSC73XX_HASH4_MAC4_TO_MASK,
    FIELD_GET(VSC73XX_HASH4_MAC4_FROM_MASK, addr[4])) |
    addr[5];
    return hash;
    }
    static int
    vsc73xx_port_wait_for_mac_table_cmd(struct vsc73xx *vsc)
    {
    int ret, err;
    u32 val;
    ret = read_poll_timeout(vsc73xx_read, err,
    err < 0 ||
    ((val & VSC73XX_MACACCESS_CMD_MASK) ==
    VSC73XX_MACACCESS_CMD_IDLE),
    VSC73XX_POLL_SLEEP_US, VSC73XX_POLL_TIMEOUT_US,
    false, vsc, VSC73XX_BLOCK_ANALYZER,
    0, VSC73XX_MACACCESS, &val);
    if (ret)
    return ret;
    return err;
    }
    static int vsc73xx_port_read_mac_table_row(struct vsc73xx *vsc, u16 index,
    struct vsc73xx_fdb *fdb)
    {
    int ret, i;
    u32 val;
    if (!fdb)
    return -EINVAL;
    if (index >= VSC73XX_NUM_FDB_ROWS)
    return -EINVAL;
    for (i = 0; i < VSC73XX_NUM_BUCKETS; i++) {
    ret = vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACTINDX,
    (i ? 0 : VSC73XX_MACTINDX_SHADOW) |
    FIELD_PREP(VSC73XX_MACTINDX_BUCKET_MSK, i) |
    index);
    if (ret)
    return ret;
    ret = vsc73xx_port_wait_for_mac_table_cmd(vsc);
    if (ret)
    return ret;
    ret = vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACACCESS,
    VSC73XX_MACACCESS_CMD_MASK,
    VSC73XX_MACACCESS_CMD_READ_ENTRY);
    if (ret)
    return ret;
    ret = vsc73xx_port_wait_for_mac_table_cmd(vsc);
    if (ret)
    return ret;
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACACCESS, &val);
    if (ret)
    return ret;
    fdb[i].valid = FIELD_GET(VSC73XX_MACACCESS_VALID, val);
    if (!fdb[i].valid)
    continue;
    fdb[i].port = FIELD_GET(VSC73XX_MACACCESS_DEST_IDX_MASK, val);
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACHDATA, &val);
    if (ret)
    return ret;
    fdb[i].vid = FIELD_GET(VSC73XX_MACHDATA_VID, val);
    fdb[i].mac[0] = FIELD_GET(VSC73XX_MACHDATA_MAC0, val);
    fdb[i].mac[1] = FIELD_GET(VSC73XX_MACHDATA_MAC1, val);
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACLDATA, &val);
    if (ret)
    return ret;
    fdb[i].mac[2] = FIELD_GET(VSC73XX_MACLDATA_MAC2, val);
    fdb[i].mac[3] = FIELD_GET(VSC73XX_MACLDATA_MAC3, val);
    fdb[i].mac[4] = FIELD_GET(VSC73XX_MACLDATA_MAC4, val);
    fdb[i].mac[5] = FIELD_GET(VSC73XX_MACLDATA_MAC5, val);
    }
    return ret;
    }
    static int
    vsc73xx_fdb_operation(struct vsc73xx *vsc, const unsigned char *addr, u16 vid,
    u16 hash, u16 cmd_mask, u16 cmd_val)
    {
    int ret;
    u32 val;
    val = FIELD_PREP(VSC73XX_MACHDATA_VID, vid) |
    FIELD_PREP(VSC73XX_MACHDATA_MAC0, addr[0]) |
    FIELD_PREP(VSC73XX_MACHDATA_MAC1, addr[1]);
    ret = vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_MACHDATA,
    val);
    if (ret)
    return ret;
    val = FIELD_PREP(VSC73XX_MACLDATA_MAC2, addr[2]) |
    FIELD_PREP(VSC73XX_MACLDATA_MAC3, addr[3]) |
    FIELD_PREP(VSC73XX_MACLDATA_MAC4, addr[4]) |
    FIELD_PREP(VSC73XX_MACLDATA_MAC5, addr[5]);
    ret = vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_MACLDATA,
    val);
    if (ret)
    return ret;
    ret = vsc73xx_write(vsc, VSC73XX_BLOCK_ANALYZER, 0, VSC73XX_MACTINDX,
    hash);
    if (ret)
    return ret;
    ret = vsc73xx_port_wait_for_mac_table_cmd(vsc);
    if (ret)
    return ret;
    ret = vsc73xx_update_bits(vsc, VSC73XX_BLOCK_ANALYZER, 0,
    VSC73XX_MACACCESS, cmd_mask, cmd_val);
    if (ret)
    return ret;
    return vsc73xx_port_wait_for_mac_table_cmd(vsc);
    }
    static int vsc73xx_fdb_del_entry(struct vsc73xx *vsc, int port,
    const unsigned char *addr, u16 vid)
    {
    struct vsc73xx_fdb fdb[VSC73XX_NUM_BUCKETS];
    let mut hash: u16 = vsc73xx_calc_hash(addr, vid);
    int bucket, ret;
    mutex_lock(&vsc.fdb_lock);
    ret = vsc73xx_port_read_mac_table_row(vsc, hash, fdb);
    if (ret)
    goto err;
    for (bucket = 0; bucket < VSC73XX_NUM_BUCKETS; bucket++) {
    if (fdb[bucket].valid && fdb[bucket].port == port &&
    ether_addr_equal(addr, fdb[bucket].mac))
    break;
    }
    if (bucket == VSC73XX_NUM_BUCKETS) {
// Can't find MAC in MAC table
    ret = -ENODATA;
    goto err;
    }
    ret = vsc73xx_fdb_operation(vsc, addr, vid, hash,
    VSC73XX_MACACCESS_CMD_MASK,
    VSC73XX_MACACCESS_CMD_FORGET);
    err:
    mutex_unlock(&vsc.fdb_lock);
    return ret;
    }
    static int vsc73xx_fdb_add_entry(struct vsc73xx *vsc, int port,
    const unsigned char *addr, u16 vid)
    {
    struct vsc73xx_fdb fdb[VSC73XX_NUM_BUCKETS];
    let mut hash: u16 = vsc73xx_calc_hash(addr, vid);
    int bucket, ret;
    u32 val;
    mutex_lock(&vsc.fdb_lock);
    ret = vsc73xx_port_read_mac_table_row(vsc, hash, fdb);
    if (ret)
    goto err;
    for (bucket = 0; bucket < VSC73XX_NUM_BUCKETS; bucket++) {
    if (!fdb[bucket].valid)
    break;
    }
    if (bucket == VSC73XX_NUM_BUCKETS) {
// Bucket is full
    ret = -EOVERFLOW;
    goto err;
    }
    val = VSC73XX_MACACCESS_VALID | VSC73XX_MACACCESS_LOCKED |
    FIELD_PREP(VSC73XX_MACACCESS_DEST_IDX_MASK, port) |
    VSC73XX_MACACCESS_CMD_LEARN;
    ret = vsc73xx_fdb_operation(vsc, addr, vid, hash,
    VSC73XX_MACACCESS_VALID |
    VSC73XX_MACACCESS_LOCKED |
    VSC73XX_MACACCESS_DEST_IDX_MASK |
    VSC73XX_MACACCESS_CMD_MASK, val);
    err:
    mutex_unlock(&vsc.fdb_lock);
    return ret;
    }
    static int vsc73xx_fdb_add(struct dsa_switch *ds, int port,
    const unsigned char *addr, u16 vid, struct dsa_db db)
    {
    struct vsc73xx *vsc = ds.priv;
    if (!vid) {
    switch (db.type) {
    case DSA_DB_PORT:
    vid = dsa_tag_8021q_standalone_vid(db.dp);
    break;
    case DSA_DB_BRIDGE:
    vid = dsa_tag_8021q_bridge_vid(db.bridge.num);
    break;
    default:
    return -EOPNOTSUPP;
    }
    }
    return vsc73xx_fdb_add_entry(vsc, port, addr, vid);
    }
    static int vsc73xx_fdb_del(struct dsa_switch *ds, int port,
    const unsigned char *addr, u16 vid, struct dsa_db db)
    {
    struct vsc73xx *vsc = ds.priv;
    if (!vid) {
    switch (db.type) {
    case DSA_DB_PORT:
    vid = dsa_tag_8021q_standalone_vid(db.dp);
    break;
    case DSA_DB_BRIDGE:
    vid = dsa_tag_8021q_bridge_vid(db.bridge.num);
    break;
    default:
    return -EOPNOTSUPP;
    }
    }
    return vsc73xx_fdb_del_entry(vsc, port, addr, vid);
    }
    static int vsc73xx_port_fdb_dump(struct dsa_switch *ds,
    int port, dsa_fdb_dump_cb_t *cb, void *data)
    {
    struct vsc73xx_fdb fdb[VSC73XX_NUM_BUCKETS];
    struct vsc73xx *vsc = ds.priv;
    u16 i, bucket;
    let mut err: c_int = 0;
    mutex_lock(&vsc.fdb_lock);
    for (i = 0; i < VSC73XX_NUM_FDB_ROWS; i++) {
    err = vsc73xx_port_read_mac_table_row(vsc, i, fdb);
    if (err)
    goto unlock;
    for (bucket = 0; bucket < VSC73XX_NUM_BUCKETS; bucket++) {
    if (!fdb[bucket].valid || fdb[bucket].port != port)
    continue;
// We need to hide dsa_8021q VLANs from the user
    if (vid_is_dsa_8021q(fdb[bucket].vid))
    fdb[bucket].vid = 0;
    err = cb(fdb[bucket].mac, fdb[bucket].vid, false, data);
    if (err)
    goto unlock;
    }
    }
    unlock:
    mutex_unlock(&vsc.fdb_lock);
    return err;
    }
    static const struct phylink_mac_ops vsc73xx_phylink_mac_ops = {
    .mac_config = vsc73xx_mac_config,
    .mac_link_down = vsc73xx_mac_link_down,
    .mac_link_up = vsc73xx_mac_link_up,
    };
    static const struct dsa_switch_ops vsc73xx_ds_ops = {
    .get_tag_protocol = vsc73xx_get_tag_protocol,
    .setup = vsc73xx_setup,
    .teardown = vsc73xx_teardown,
    .phy_read = vsc73xx_phy_read,
    .phy_write = vsc73xx_phy_write,
    .get_strings = vsc73xx_get_strings,
    .get_ethtool_stats = vsc73xx_get_ethtool_stats,
    .get_sset_count = vsc73xx_get_sset_count,
    .port_enable = vsc73xx_port_enable,
    .port_disable = vsc73xx_port_disable,
    .port_pre_bridge_flags = vsc73xx_port_pre_bridge_flags,
    .port_bridge_flags = vsc73xx_port_bridge_flags,
    .port_bridge_join = dsa_tag_8021q_bridge_join,
    .port_bridge_leave = dsa_tag_8021q_bridge_leave,
    .port_change_mtu = vsc73xx_change_mtu,
    .port_fdb_add = vsc73xx_fdb_add,
    .port_fdb_del = vsc73xx_fdb_del,
    .port_fdb_dump = vsc73xx_port_fdb_dump,
    .port_max_mtu = vsc73xx_get_max_mtu,
    .port_stp_state_set = vsc73xx_port_stp_state_set,
    .port_vlan_filtering = vsc73xx_port_vlan_filtering,
    .port_vlan_add = vsc73xx_port_vlan_add,
    .port_vlan_del = vsc73xx_port_vlan_del,
    .phylink_get_caps = vsc73xx_phylink_get_caps,
    .tag_8021q_vlan_add = vsc73xx_tag_8021q_vlan_add,
    .tag_8021q_vlan_del = vsc73xx_tag_8021q_vlan_del,
    };
#[no_mangle]
unsafe extern "C" fn vsc73xx_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int vsc73xx_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct vsc73xx *vsc = gpiochip_get_data(chip);
    u32 val;
    int ret;
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_GPIO, &val);
    if (ret)
    return ret;
    return !!(val & BIT(offset));
    }
    static int vsc73xx_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int val)
    {
    struct vsc73xx *vsc = gpiochip_get_data(chip);
    let mut tmp: u32 = val ? BIT(offset) : 0;
    return vsc73xx_update_bits(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_GPIO, BIT(offset), tmp);
    }
    static int vsc73xx_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int val)
    {
    struct vsc73xx *vsc = gpiochip_get_data(chip);
    let mut tmp: u32 = val ? BIT(offset) : 0;
    return vsc73xx_update_bits(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_GPIO, BIT(offset + 4) | BIT(offset),
    BIT(offset + 4) | tmp);
    }
    static int vsc73xx_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct vsc73xx *vsc = gpiochip_get_data(chip);
    return  vsc73xx_update_bits(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_GPIO, BIT(offset + 4),
    0);
    }
    static int vsc73xx_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct vsc73xx *vsc = gpiochip_get_data(chip);
    u32 val;
    int ret;
    ret = vsc73xx_read(vsc, VSC73XX_BLOCK_SYSTEM, 0,
    VSC73XX_GPIO, &val);
    if (ret)
    return ret;
    return !(val & BIT(offset + 4));
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_gpio_probe(vsc: *mut vsc73xx) -> c_int {
    static int vsc73xx_gpio_probe(struct vsc73xx *vsc)
    {
    int ret;
    vsc.gc.label = devm_kasprintf(vsc.dev, GFP_KERNEL, "VSC%04x",
    vsc.chipid);
    if (!vsc.gc.label)
    return -ENOMEM;
    vsc.gc.ngpio = 4;
    vsc.gc.owner = THIS_MODULE;
    vsc.gc.parent = vsc.dev;
    vsc.gc.base = -1;
    vsc.gc.get = vsc73xx_gpio_get;
    vsc.gc.set = vsc73xx_gpio_set;
    vsc.gc.direction_input = vsc73xx_gpio_direction_input;
    vsc.gc.direction_output = vsc73xx_gpio_direction_output;
    vsc.gc.get_direction = vsc73xx_gpio_get_direction;
    vsc.gc.can_sleep = true;
    ret = devm_gpiochip_add_data(vsc.dev, &vsc.gc, vsc);
    if (ret) {
    dev_err(vsc.dev, "unable to register GPIO chip\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vsc73xx_probe(vsc: *mut vsc73xx) -> c_int {
    int vsc73xx_probe(struct vsc73xx *vsc)
    {
    struct device *dev = vsc.dev;
    int ret;
// Release reset, if any
    vsc.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(vsc.reset)) {
    dev_err(dev, "failed to get RESET GPIO\n");
    return PTR_ERR(vsc.reset);
    }
    if (vsc.reset)
// Wait 20ms according to datasheet table 245
    msleep(20);
    ret = vsc73xx_detect(vsc);
    if (ret == -EAGAIN) {
    dev_err(vsc.dev,
    "Chip seems to be out of control. Assert reset and try again.\n");
    gpiod_set_value_cansleep(vsc.reset, 1);
// Reset pulse should be 20ns minimum, according to datasheet
// table 245, so 10us should be fine
//
    usleep_range(10, 100);
    gpiod_set_value_cansleep(vsc.reset, 0);
// Wait 20ms according to datasheet table 245
    msleep(20);
    ret = vsc73xx_detect(vsc);
    }
    if (ret) {
    dev_err(dev, "no chip found (%d)\n", ret);
    return -ENODEV;
    }
    mutex_init(&vsc.fdb_lock);
    eth_random_addr(vsc.addr);
    dev_info(vsc.dev,
    "MAC for control frames: %02X:%02X:%02X:%02X:%02X:%02X\n",
    vsc.addr[0], vsc.addr[1], vsc.addr[2],
    vsc.addr[3], vsc.addr[4], vsc.addr[5]);
    vsc.ds = devm_kzalloc(dev, sizeof(*vsc.ds), GFP_KERNEL);
    if (!vsc.ds)
    return -ENOMEM;
    vsc.ds.dev = dev;
    vsc.ds.num_ports = VSC73XX_MAX_NUM_PORTS;
    vsc.ds.priv = vsc;
    vsc.ds.ops = &vsc73xx_ds_ops;
    vsc.ds.phylink_mac_ops = &vsc73xx_phylink_mac_ops;
    ret = dsa_register_switch(vsc.ds);
    if (ret) {
    dev_err(dev, "unable to register switch (%d)\n", ret);
    return ret;
    }
    ret = vsc73xx_gpio_probe(vsc);
    if (ret) {
    dsa_unregister_switch(vsc.ds);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL(vsc73xx_probe);
#[no_mangle]
pub unsafe extern "C" fn vsc73xx_remove(vsc: *mut vsc73xx) {
    void vsc73xx_remove(struct vsc73xx *vsc)
    {
    dsa_unregister_switch(vsc.ds);
    gpiod_set_value(vsc.reset, 1);
    }
    EXPORT_SYMBOL(vsc73xx_remove);
#[no_mangle]
pub unsafe extern "C" fn vsc73xx_shutdown(vsc: *mut vsc73xx) {
    void vsc73xx_shutdown(struct vsc73xx *vsc)
    {
    dsa_switch_shutdown(vsc.ds);
    }
    EXPORT_SYMBOL(vsc73xx_shutdown);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("Vitesse VSC7385/7388/7395/7398 driver");
    MODULE_LICENSE("GPL v2");
