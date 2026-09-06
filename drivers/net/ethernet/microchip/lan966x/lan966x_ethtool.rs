//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_ethtool.c
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


// SPDX-License-Identifier: GPL-2.0+

// Number of traffic classes
pub const LAN966X_NUM_TC: c_int = 8;

    static const struct lan966x_stat_layout lan966x_stats_layout[] = {
    { .name = "rx_octets", .offset = 0x00, },
    { .name = "rx_unicast", .offset = 0x01, },
    { .name = "rx_multicast", .offset = 0x02 },
    { .name = "rx_broadcast", .offset = 0x03 },
    { .name = "rx_short", .offset = 0x04 },
    { .name = "rx_frag", .offset = 0x05 },
    { .name = "rx_jabber", .offset = 0x06 },
    { .name = "rx_crc", .offset = 0x07 },
    { .name = "rx_symbol_err", .offset = 0x08 },
    { .name = "rx_sz_64", .offset = 0x09 },
    { .name = "rx_sz_65_127", .offset = 0x0a},
    { .name = "rx_sz_128_255", .offset = 0x0b},
    { .name = "rx_sz_256_511", .offset = 0x0c },
    { .name = "rx_sz_512_1023", .offset = 0x0d },
    { .name = "rx_sz_1024_1526", .offset = 0x0e },
    { .name = "rx_sz_jumbo", .offset = 0x0f },
    { .name = "rx_pause", .offset = 0x10 },
    { .name = "rx_control", .offset = 0x11 },
    { .name = "rx_long", .offset = 0x12 },
    { .name = "rx_cat_drop", .offset = 0x13 },
    { .name = "rx_red_prio_0", .offset = 0x14 },
    { .name = "rx_red_prio_1", .offset = 0x15 },
    { .name = "rx_red_prio_2", .offset = 0x16 },
    { .name = "rx_red_prio_3", .offset = 0x17 },
    { .name = "rx_red_prio_4", .offset = 0x18 },
    { .name = "rx_red_prio_5", .offset = 0x19 },
    { .name = "rx_red_prio_6", .offset = 0x1a },
    { .name = "rx_red_prio_7", .offset = 0x1b },
    { .name = "rx_yellow_prio_0", .offset = 0x1c },
    { .name = "rx_yellow_prio_1", .offset = 0x1d },
    { .name = "rx_yellow_prio_2", .offset = 0x1e },
    { .name = "rx_yellow_prio_3", .offset = 0x1f },
    { .name = "rx_yellow_prio_4", .offset = 0x20 },
    { .name = "rx_yellow_prio_5", .offset = 0x21 },
    { .name = "rx_yellow_prio_6", .offset = 0x22 },
    { .name = "rx_yellow_prio_7", .offset = 0x23 },
    { .name = "rx_green_prio_0", .offset = 0x24 },
    { .name = "rx_green_prio_1", .offset = 0x25 },
    { .name = "rx_green_prio_2", .offset = 0x26 },
    { .name = "rx_green_prio_3", .offset = 0x27 },
    { .name = "rx_green_prio_4", .offset = 0x28 },
    { .name = "rx_green_prio_5", .offset = 0x29 },
    { .name = "rx_green_prio_6", .offset = 0x2a },
    { .name = "rx_green_prio_7", .offset = 0x2b },
    { .name = "rx_assembly_err", .offset = 0x2c },
    { .name = "rx_smd_err", .offset = 0x2d },
    { .name = "rx_assembly_ok", .offset = 0x2e },
    { .name = "rx_merge_frag", .offset = 0x2f },
    { .name = "rx_pmac_octets", .offset = 0x30, },
    { .name = "rx_pmac_unicast", .offset = 0x31, },
    { .name = "rx_pmac_multicast", .offset = 0x32 },
    { .name = "rx_pmac_broadcast", .offset = 0x33 },
    { .name = "rx_pmac_short", .offset = 0x34 },
    { .name = "rx_pmac_frag", .offset = 0x35 },
    { .name = "rx_pmac_jabber", .offset = 0x36 },
    { .name = "rx_pmac_crc", .offset = 0x37 },
    { .name = "rx_pmac_symbol_err", .offset = 0x38 },
    { .name = "rx_pmac_sz_64", .offset = 0x39 },
    { .name = "rx_pmac_sz_65_127", .offset = 0x3a },
    { .name = "rx_pmac_sz_128_255", .offset = 0x3b },
    { .name = "rx_pmac_sz_256_511", .offset = 0x3c },
    { .name = "rx_pmac_sz_512_1023", .offset = 0x3d },
    { .name = "rx_pmac_sz_1024_1526", .offset = 0x3e },
    { .name = "rx_pmac_sz_jumbo", .offset = 0x3f },
    { .name = "rx_pmac_pause", .offset = 0x40 },
    { .name = "rx_pmac_control", .offset = 0x41 },
    { .name = "rx_pmac_long", .offset = 0x42 },
    { .name = "tx_octets", .offset = 0x80, },
    { .name = "tx_unicast", .offset = 0x81, },
    { .name = "tx_multicast", .offset = 0x82 },
    { .name = "tx_broadcast", .offset = 0x83 },
    { .name = "tx_col", .offset = 0x84 },
    { .name = "tx_drop", .offset = 0x85 },
    { .name = "tx_pause", .offset = 0x86 },
    { .name = "tx_sz_64", .offset = 0x87 },
    { .name = "tx_sz_65_127", .offset = 0x88 },
    { .name = "tx_sz_128_255", .offset = 0x89 },
    { .name = "tx_sz_256_511", .offset = 0x8a },
    { .name = "tx_sz_512_1023", .offset = 0x8b },
    { .name = "tx_sz_1024_1526", .offset = 0x8c },
    { .name = "tx_sz_jumbo", .offset = 0x8d },
    { .name = "tx_yellow_prio_0", .offset = 0x8e },
    { .name = "tx_yellow_prio_1", .offset = 0x8f },
    { .name = "tx_yellow_prio_2", .offset = 0x90 },
    { .name = "tx_yellow_prio_3", .offset = 0x91 },
    { .name = "tx_yellow_prio_4", .offset = 0x92 },
    { .name = "tx_yellow_prio_5", .offset = 0x93 },
    { .name = "tx_yellow_prio_6", .offset = 0x94 },
    { .name = "tx_yellow_prio_7", .offset = 0x95 },
    { .name = "tx_green_prio_0", .offset = 0x96 },
    { .name = "tx_green_prio_1", .offset = 0x97 },
    { .name = "tx_green_prio_2", .offset = 0x98 },
    { .name = "tx_green_prio_3", .offset = 0x99 },
    { .name = "tx_green_prio_4", .offset = 0x9a },
    { .name = "tx_green_prio_5", .offset = 0x9b },
    { .name = "tx_green_prio_6", .offset = 0x9c },
    { .name = "tx_green_prio_7", .offset = 0x9d },
    { .name = "tx_aged", .offset = 0x9e },
    { .name = "tx_llct", .offset = 0x9f },
    { .name = "tx_ct", .offset = 0xa0 },
    { .name = "tx_mm_hold", .offset = 0xa1 },
    { .name = "tx_merge_frag", .offset = 0xa2 },
    { .name = "tx_pmac_octets", .offset = 0xa3, },
    { .name = "tx_pmac_unicast", .offset = 0xa4, },
    { .name = "tx_pmac_multicast", .offset = 0xa5 },
    { .name = "tx_pmac_broadcast", .offset = 0xa6 },
    { .name = "tx_pmac_pause", .offset = 0xa7 },
    { .name = "tx_pmac_sz_64", .offset = 0xa8 },
    { .name = "tx_pmac_sz_65_127", .offset = 0xa9 },
    { .name = "tx_pmac_sz_128_255", .offset = 0xaa },
    { .name = "tx_pmac_sz_256_511", .offset = 0xab },
    { .name = "tx_pmac_sz_512_1023", .offset = 0xac },
    { .name = "tx_pmac_sz_1024_1526", .offset = 0xad },
    { .name = "tx_pmac_sz_jumbo", .offset = 0xae },
    { .name = "dr_local", .offset = 0x100 },
    { .name = "dr_tail", .offset = 0x101 },
    { .name = "dr_yellow_prio_0", .offset = 0x102 },
    { .name = "dr_yellow_prio_1", .offset = 0x103 },
    { .name = "dr_yellow_prio_2", .offset = 0x104 },
    { .name = "dr_yellow_prio_3", .offset = 0x105 },
    { .name = "dr_yellow_prio_4", .offset = 0x106 },
    { .name = "dr_yellow_prio_5", .offset = 0x107 },
    { .name = "dr_yellow_prio_6", .offset = 0x108 },
    { .name = "dr_yellow_prio_7", .offset = 0x109 },
    { .name = "dr_green_prio_0", .offset = 0x10a },
    { .name = "dr_green_prio_1", .offset = 0x10b },
    { .name = "dr_green_prio_2", .offset = 0x10c },
    { .name = "dr_green_prio_3", .offset = 0x10d },
    { .name = "dr_green_prio_4", .offset = 0x10e },
    { .name = "dr_green_prio_5", .offset = 0x10f },
    { .name = "dr_green_prio_6", .offset = 0x110 },
    { .name = "dr_green_prio_7", .offset = 0x111 },
    };
// The following numbers are indexes into lan966x_stats_layout[]
pub const SYS_COUNT_RX_OCT: c_int = 0;
pub const SYS_COUNT_RX_UC: c_int = 1;
pub const SYS_COUNT_RX_MC: c_int = 2;
pub const SYS_COUNT_RX_BC: c_int = 3;
pub const SYS_COUNT_RX_SHORT: c_int = 4;
pub const SYS_COUNT_RX_FRAG: c_int = 5;
pub const SYS_COUNT_RX_JABBER: c_int = 6;
pub const SYS_COUNT_RX_CRC: c_int = 7;
pub const SYS_COUNT_RX_SYMBOL_ERR: c_int = 8;
pub const SYS_COUNT_RX_SZ_64: c_int = 9;
pub const SYS_COUNT_RX_SZ_65_127: c_int = 10;
pub const SYS_COUNT_RX_SZ_128_255: c_int = 11;
pub const SYS_COUNT_RX_SZ_256_511: c_int = 12;
pub const SYS_COUNT_RX_SZ_512_1023: c_int = 13;
pub const SYS_COUNT_RX_SZ_1024_1526: c_int = 14;
pub const SYS_COUNT_RX_SZ_JUMBO: c_int = 15;
pub const SYS_COUNT_RX_PAUSE: c_int = 16;
pub const SYS_COUNT_RX_CONTROL: c_int = 17;
pub const SYS_COUNT_RX_LONG: c_int = 18;
pub const SYS_COUNT_RX_CAT_DROP: c_int = 19;
pub const SYS_COUNT_RX_RED_PRIO_0: c_int = 20;
pub const SYS_COUNT_RX_RED_PRIO_1: c_int = 21;
pub const SYS_COUNT_RX_RED_PRIO_2: c_int = 22;
pub const SYS_COUNT_RX_RED_PRIO_3: c_int = 23;
pub const SYS_COUNT_RX_RED_PRIO_4: c_int = 24;
pub const SYS_COUNT_RX_RED_PRIO_5: c_int = 25;
pub const SYS_COUNT_RX_RED_PRIO_6: c_int = 26;
pub const SYS_COUNT_RX_RED_PRIO_7: c_int = 27;
pub const SYS_COUNT_RX_YELLOW_PRIO_0: c_int = 28;
pub const SYS_COUNT_RX_YELLOW_PRIO_1: c_int = 29;
pub const SYS_COUNT_RX_YELLOW_PRIO_2: c_int = 30;
pub const SYS_COUNT_RX_YELLOW_PRIO_3: c_int = 31;
pub const SYS_COUNT_RX_YELLOW_PRIO_4: c_int = 32;
pub const SYS_COUNT_RX_YELLOW_PRIO_5: c_int = 33;
pub const SYS_COUNT_RX_YELLOW_PRIO_6: c_int = 34;
pub const SYS_COUNT_RX_YELLOW_PRIO_7: c_int = 35;
pub const SYS_COUNT_RX_GREEN_PRIO_0: c_int = 36;
pub const SYS_COUNT_RX_GREEN_PRIO_1: c_int = 37;
pub const SYS_COUNT_RX_GREEN_PRIO_2: c_int = 38;
pub const SYS_COUNT_RX_GREEN_PRIO_3: c_int = 39;
pub const SYS_COUNT_RX_GREEN_PRIO_4: c_int = 40;
pub const SYS_COUNT_RX_GREEN_PRIO_5: c_int = 41;
pub const SYS_COUNT_RX_GREEN_PRIO_6: c_int = 42;
pub const SYS_COUNT_RX_GREEN_PRIO_7: c_int = 43;
pub const SYS_COUNT_RX_ASSEMBLY_ERR: c_int = 44;
pub const SYS_COUNT_RX_SMD_ERR: c_int = 45;
pub const SYS_COUNT_RX_ASSEMBLY_OK: c_int = 46;
pub const SYS_COUNT_RX_MERGE_FRAG: c_int = 47;
pub const SYS_COUNT_RX_PMAC_OCT: c_int = 48;
pub const SYS_COUNT_RX_PMAC_UC: c_int = 49;
pub const SYS_COUNT_RX_PMAC_MC: c_int = 50;
pub const SYS_COUNT_RX_PMAC_BC: c_int = 51;
pub const SYS_COUNT_RX_PMAC_SHORT: c_int = 52;
pub const SYS_COUNT_RX_PMAC_FRAG: c_int = 53;
pub const SYS_COUNT_RX_PMAC_JABBER: c_int = 54;
pub const SYS_COUNT_RX_PMAC_CRC: c_int = 55;
pub const SYS_COUNT_RX_PMAC_SYMBOL_ERR: c_int = 56;
pub const SYS_COUNT_RX_PMAC_SZ_64: c_int = 57;
pub const SYS_COUNT_RX_PMAC_SZ_65_127: c_int = 58;
pub const SYS_COUNT_RX_PMAC_SZ_128_255: c_int = 59;
pub const SYS_COUNT_RX_PMAC_SZ_256_511: c_int = 60;
pub const SYS_COUNT_RX_PMAC_SZ_512_1023: c_int = 61;
pub const SYS_COUNT_RX_PMAC_SZ_1024_1526: c_int = 62;
pub const SYS_COUNT_RX_PMAC_SZ_JUMBO: c_int = 63;
pub const SYS_COUNT_RX_PMAC_PAUSE: c_int = 64;
pub const SYS_COUNT_RX_PMAC_CONTROL: c_int = 65;
pub const SYS_COUNT_RX_PMAC_LONG: c_int = 66;
pub const SYS_COUNT_TX_OCT: c_int = 67;
pub const SYS_COUNT_TX_UC: c_int = 68;
pub const SYS_COUNT_TX_MC: c_int = 69;
pub const SYS_COUNT_TX_BC: c_int = 70;
pub const SYS_COUNT_TX_COL: c_int = 71;
pub const SYS_COUNT_TX_DROP: c_int = 72;
pub const SYS_COUNT_TX_PAUSE: c_int = 73;
pub const SYS_COUNT_TX_SZ_64: c_int = 74;
pub const SYS_COUNT_TX_SZ_65_127: c_int = 75;
pub const SYS_COUNT_TX_SZ_128_255: c_int = 76;
pub const SYS_COUNT_TX_SZ_256_511: c_int = 77;
pub const SYS_COUNT_TX_SZ_512_1023: c_int = 78;
pub const SYS_COUNT_TX_SZ_1024_1526: c_int = 79;
pub const SYS_COUNT_TX_SZ_JUMBO: c_int = 80;
pub const SYS_COUNT_TX_YELLOW_PRIO_0: c_int = 81;
pub const SYS_COUNT_TX_YELLOW_PRIO_1: c_int = 82;
pub const SYS_COUNT_TX_YELLOW_PRIO_2: c_int = 83;
pub const SYS_COUNT_TX_YELLOW_PRIO_3: c_int = 84;
pub const SYS_COUNT_TX_YELLOW_PRIO_4: c_int = 85;
pub const SYS_COUNT_TX_YELLOW_PRIO_5: c_int = 86;
pub const SYS_COUNT_TX_YELLOW_PRIO_6: c_int = 87;
pub const SYS_COUNT_TX_YELLOW_PRIO_7: c_int = 88;
pub const SYS_COUNT_TX_GREEN_PRIO_0: c_int = 89;
pub const SYS_COUNT_TX_GREEN_PRIO_1: c_int = 90;
pub const SYS_COUNT_TX_GREEN_PRIO_2: c_int = 91;
pub const SYS_COUNT_TX_GREEN_PRIO_3: c_int = 92;
pub const SYS_COUNT_TX_GREEN_PRIO_4: c_int = 93;
pub const SYS_COUNT_TX_GREEN_PRIO_5: c_int = 94;
pub const SYS_COUNT_TX_GREEN_PRIO_6: c_int = 95;
pub const SYS_COUNT_TX_GREEN_PRIO_7: c_int = 96;
pub const SYS_COUNT_TX_AGED: c_int = 97;
pub const SYS_COUNT_TX_LLCT: c_int = 98;
pub const SYS_COUNT_TX_CT: c_int = 99;
pub const SYS_COUNT_TX_MM_HOLD: c_int = 100;
pub const SYS_COUNT_TX_MERGE_FRAG: c_int = 101;
pub const SYS_COUNT_TX_PMAC_OCT: c_int = 102;
pub const SYS_COUNT_TX_PMAC_UC: c_int = 103;
pub const SYS_COUNT_TX_PMAC_MC: c_int = 104;
pub const SYS_COUNT_TX_PMAC_BC: c_int = 105;
pub const SYS_COUNT_TX_PMAC_PAUSE: c_int = 106;
pub const SYS_COUNT_TX_PMAC_SZ_64: c_int = 107;
pub const SYS_COUNT_TX_PMAC_SZ_65_127: c_int = 108;
pub const SYS_COUNT_TX_PMAC_SZ_128_255: c_int = 109;
pub const SYS_COUNT_TX_PMAC_SZ_256_511: c_int = 110;
pub const SYS_COUNT_TX_PMAC_SZ_512_1023: c_int = 111;
pub const SYS_COUNT_TX_PMAC_SZ_1024_1526: c_int = 112;
pub const SYS_COUNT_TX_PMAC_SZ_JUMBO: c_int = 113;
pub const SYS_COUNT_DR_LOCAL: c_int = 114;
pub const SYS_COUNT_DR_TAIL: c_int = 115;
pub const SYS_COUNT_DR_YELLOW_PRIO_0: c_int = 116;
pub const SYS_COUNT_DR_YELLOW_PRIO_1: c_int = 117;
pub const SYS_COUNT_DR_YELLOW_PRIO_2: c_int = 118;
pub const SYS_COUNT_DR_YELLOW_PRIO_3: c_int = 119;
pub const SYS_COUNT_DR_YELLOW_PRIO_4: c_int = 120;
pub const SYS_COUNT_DR_YELLOW_PRIO_5: c_int = 121;
pub const SYS_COUNT_DR_YELLOW_PRIO_6: c_int = 122;
pub const SYS_COUNT_DR_YELLOW_PRIO_7: c_int = 123;
pub const SYS_COUNT_DR_GREEN_PRIO_0: c_int = 124;
pub const SYS_COUNT_DR_GREEN_PRIO_1: c_int = 125;
pub const SYS_COUNT_DR_GREEN_PRIO_2: c_int = 126;
pub const SYS_COUNT_DR_GREEN_PRIO_3: c_int = 127;
pub const SYS_COUNT_DR_GREEN_PRIO_4: c_int = 128;
pub const SYS_COUNT_DR_GREEN_PRIO_5: c_int = 129;
pub const SYS_COUNT_DR_GREEN_PRIO_6: c_int = 130;
pub const SYS_COUNT_DR_GREEN_PRIO_7: c_int = 131;
// Add a possibly wrapping 32 bit value to a 64 bit counter
#[no_mangle]
unsafe extern "C" fn lan966x_add_cnt(cnt: *mut u64, val: u32) {
    static void lan966x_add_cnt(u64 *cnt, u32 val)
    {
    if (val < (*cnt & U32_MAX))
// cnt += (u64)1 << 32; /* value has wrapped
// cnt = (*cnt & ~(u64)U32_MAX) + val;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_stats_update(lan966x: *mut lan966x) {
    static void lan966x_stats_update(struct lan966x *lan966x)
    {
    int i, j;
    spin_lock(&lan966x.stats_lock);
    for (i = 0; i < lan966x.num_phys_ports; i++) {
    let mut idx: c_uint = i * lan966x.num_stats;
    lan_wr(SYS_STAT_CFG_STAT_VIEW_SET(i),
    lan966x, SYS_STAT_CFG);
    for (j = 0; j < lan966x.num_stats; j++) {
    let mut offset: u32 = lan966x.stats_layout[j].offset;
    lan966x_add_cnt(&lan966x.stats[idx++],
    lan_rd(lan966x, SYS_CNT(offset)));
    }
    }
    spin_unlock(&lan966x.stats_lock);
    }
#[no_mangle]
unsafe extern "C" fn lan966x_get_sset_count(dev: *mut net_device, sset: c_int) -> c_int {
    static int lan966x_get_sset_count(struct net_device *dev, int sset)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    if (sset != ETH_SS_STATS)
    return -EOPNOTSUPP;
    return lan966x.num_stats;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_get_strings(netdev: *mut net_device, sset: u32, data: *mut u8) {
    static void lan966x_get_strings(struct net_device *netdev, u32 sset, u8 *data)
    {
    struct lan966x_port *port = netdev_priv(netdev);
    struct lan966x *lan966x = port.lan966x;
    int i;
    if (sset != ETH_SS_STATS)
    return;
    for (i = 0; i < lan966x.num_stats; i++)
    memcpy(data + i * ETH_GSTRING_LEN,
    lan966x.stats_layout[i].name, ETH_GSTRING_LEN);
    }
    static void lan966x_get_ethtool_stats(struct net_device *dev,
    struct ethtool_stats *stats, u64 *data)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    int i;
// check and update now
    lan966x_stats_update(lan966x);
// Copy all counters
    for (i = 0; i < lan966x.num_stats; i++)
// data++ = lan966x->stats[port->chip_port
    lan966x.num_stats + i];
    }
    static void lan966x_get_eth_mac_stats(struct net_device *dev,
    struct ethtool_eth_mac_stats *mac_stats)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    u32 idx;
    lan966x_stats_update(lan966x);
    idx = port.chip_port * lan966x.num_stats;
    spin_lock(&lan966x.stats_lock);
    mac_stats.FramesTransmittedOK =
    lan966x.stats[idx + SYS_COUNT_TX_UC] +
    lan966x.stats[idx + SYS_COUNT_TX_MC] +
    lan966x.stats[idx + SYS_COUNT_TX_BC] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_UC] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_MC] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_BC];
    mac_stats.SingleCollisionFrames =
    lan966x.stats[idx + SYS_COUNT_TX_COL];
    mac_stats.FramesReceivedOK =
    lan966x.stats[idx + SYS_COUNT_RX_UC] +
    lan966x.stats[idx + SYS_COUNT_RX_MC] +
    lan966x.stats[idx + SYS_COUNT_RX_BC];
    mac_stats.FrameCheckSequenceErrors =
    lan966x.stats[idx + SYS_COUNT_RX_CRC] +
    lan966x.stats[idx + SYS_COUNT_RX_CRC];
    mac_stats.OctetsTransmittedOK =
    lan966x.stats[idx + SYS_COUNT_TX_OCT] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_OCT];
    mac_stats.FramesWithDeferredXmissions =
    lan966x.stats[idx + SYS_COUNT_TX_MM_HOLD];
    mac_stats.OctetsReceivedOK =
    lan966x.stats[idx + SYS_COUNT_RX_OCT];
    mac_stats.MulticastFramesXmittedOK =
    lan966x.stats[idx + SYS_COUNT_TX_MC] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_MC];
    mac_stats.BroadcastFramesXmittedOK =
    lan966x.stats[idx + SYS_COUNT_TX_BC] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_BC];
    mac_stats.MulticastFramesReceivedOK =
    lan966x.stats[idx + SYS_COUNT_RX_MC];
    mac_stats.BroadcastFramesReceivedOK =
    lan966x.stats[idx + SYS_COUNT_RX_BC];
    mac_stats.InRangeLengthErrors =
    lan966x.stats[idx + SYS_COUNT_RX_FRAG] +
    lan966x.stats[idx + SYS_COUNT_RX_JABBER] +
    lan966x.stats[idx + SYS_COUNT_RX_CRC] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_FRAG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_JABBER] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_CRC];
    mac_stats.OutOfRangeLengthField =
    lan966x.stats[idx + SYS_COUNT_RX_SHORT] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SHORT] +
    lan966x.stats[idx + SYS_COUNT_RX_LONG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_LONG];
    mac_stats.FrameTooLongErrors =
    lan966x.stats[idx + SYS_COUNT_RX_LONG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_LONG];
    spin_unlock(&lan966x.stats_lock);
    }
    static const struct ethtool_rmon_hist_range lan966x_rmon_ranges[] = {
    {    0,    64 },
    {   65,   127 },
    {  128,   255 },
    {  256,   511 },
    {  512,  1023 },
    { 1024,  1518 },
    { 1519, 10239 },
    {}
    };
    static void lan966x_get_eth_rmon_stats(struct net_device *dev,
    struct ethtool_rmon_stats *rmon_stats,
    const struct ethtool_rmon_hist_range **ranges)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    u32 idx;
    lan966x_stats_update(lan966x);
    idx = port.chip_port * lan966x.num_stats;
    spin_lock(&lan966x.stats_lock);
    rmon_stats.undersize_pkts =
    lan966x.stats[idx + SYS_COUNT_RX_SHORT] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SHORT];
    rmon_stats.oversize_pkts =
    lan966x.stats[idx + SYS_COUNT_RX_LONG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_LONG];
    rmon_stats.fragments =
    lan966x.stats[idx + SYS_COUNT_RX_FRAG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_FRAG];
    rmon_stats.jabbers =
    lan966x.stats[idx + SYS_COUNT_RX_JABBER] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_JABBER];
    rmon_stats.hist[0] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_64] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_64];
    rmon_stats.hist[1] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_65_127] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_65_127];
    rmon_stats.hist[2] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_128_255] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_128_255];
    rmon_stats.hist[3] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_256_511] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_256_511];
    rmon_stats.hist[4] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_512_1023] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_512_1023];
    rmon_stats.hist[5] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_1024_1526];
    rmon_stats.hist[6] =
    lan966x.stats[idx + SYS_COUNT_RX_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_1024_1526];
    rmon_stats.hist_tx[0] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_64] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_64];
    rmon_stats.hist_tx[1] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_65_127] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_65_127];
    rmon_stats.hist_tx[2] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_128_255] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_128_255];
    rmon_stats.hist_tx[3] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_256_511] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_256_511];
    rmon_stats.hist_tx[4] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_512_1023] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_512_1023];
    rmon_stats.hist_tx[5] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_1024_1526];
    rmon_stats.hist_tx[6] =
    lan966x.stats[idx + SYS_COUNT_TX_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_1024_1526];
    spin_unlock(&lan966x.stats_lock);
// ranges = lan966x_rmon_ranges;
    }
    static int lan966x_get_link_ksettings(struct net_device *ndev,
    struct ethtool_link_ksettings *cmd)
    {
    struct lan966x_port *port = netdev_priv(ndev);
    return phylink_ethtool_ksettings_get(port.phylink, cmd);
    }
    static int lan966x_set_link_ksettings(struct net_device *ndev,
    const struct ethtool_link_ksettings *cmd)
    {
    struct lan966x_port *port = netdev_priv(ndev);
    return phylink_ethtool_ksettings_set(port.phylink, cmd);
    }
    static void lan966x_get_pauseparam(struct net_device *dev,
    struct ethtool_pauseparam *pause)
    {
    struct lan966x_port *port = netdev_priv(dev);
    phylink_ethtool_get_pauseparam(port.phylink, pause);
    }
    static int lan966x_set_pauseparam(struct net_device *dev,
    struct ethtool_pauseparam *pause)
    {
    struct lan966x_port *port = netdev_priv(dev);
    return phylink_ethtool_set_pauseparam(port.phylink, pause);
    }
    static int lan966x_get_ts_info(struct net_device *dev,
    struct kernel_ethtool_ts_info *info)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    struct lan966x_phc *phc;
    if (!lan966x.ptp)
    return ethtool_op_get_ts_info(dev, info);
    phc = &lan966x.phc[LAN966X_PHC_PORT];
    if (phc.clock) {
    info.phc_index = ptp_clock_index(phc.clock);
    } else {
    info.so_timestamping |= SOF_TIMESTAMPING_TX_SOFTWARE;
    return 0;
    }
    info.so_timestamping |= SOF_TIMESTAMPING_TX_SOFTWARE |
    SOF_TIMESTAMPING_TX_HARDWARE |
    SOF_TIMESTAMPING_RX_HARDWARE |
    SOF_TIMESTAMPING_RAW_HARDWARE;
    info.tx_types = BIT(HWTSTAMP_TX_OFF) | BIT(HWTSTAMP_TX_ON) |
    BIT(HWTSTAMP_TX_ONESTEP_SYNC);
    info.rx_filters = BIT(HWTSTAMP_FILTER_NONE) |
    BIT(HWTSTAMP_FILTER_ALL);
    return 0;
    }
    const struct ethtool_ops lan966x_ethtool_ops = {
    .get_link_ksettings     = lan966x_get_link_ksettings,
    .set_link_ksettings     = lan966x_set_link_ksettings,
    .get_pauseparam		= lan966x_get_pauseparam,
    .set_pauseparam		= lan966x_set_pauseparam,
    .get_sset_count		= lan966x_get_sset_count,
    .get_strings		= lan966x_get_strings,
    .get_ethtool_stats	= lan966x_get_ethtool_stats,
    .get_eth_mac_stats      = lan966x_get_eth_mac_stats,
    .get_rmon_stats		= lan966x_get_eth_rmon_stats,
    .get_link		= ethtool_op_get_link,
    .get_ts_info		= lan966x_get_ts_info,
    };
#[no_mangle]
unsafe extern "C" fn lan966x_check_stats_work(work: *mut work_struct) {
    static void lan966x_check_stats_work(struct work_struct *work)
    {
    struct delayed_work *del_work = to_delayed_work(work);
    struct lan966x *lan966x = container_of(del_work, struct lan966x,
    stats_work);
    lan966x_stats_update(lan966x);
    queue_delayed_work(lan966x.stats_queue, &lan966x.stats_work,
    LAN966X_STATS_CHECK_DELAY);
    }
    void lan966x_stats_get(struct net_device *dev,
    struct rtnl_link_stats64 *stats)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    u32 idx;
    int i;
    idx = port.chip_port * lan966x.num_stats;
    spin_lock(&lan966x.stats_lock);
    stats.rx_bytes = lan966x.stats[idx + SYS_COUNT_RX_OCT] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_OCT];
    stats.rx_packets = lan966x.stats[idx + SYS_COUNT_RX_SHORT] +
    lan966x.stats[idx + SYS_COUNT_RX_FRAG] +
    lan966x.stats[idx + SYS_COUNT_RX_JABBER] +
    lan966x.stats[idx + SYS_COUNT_RX_CRC] +
    lan966x.stats[idx + SYS_COUNT_RX_SYMBOL_ERR] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_64] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_65_127] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_128_255] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_256_511] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_512_1023] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_RX_SZ_JUMBO] +
    lan966x.stats[idx + SYS_COUNT_RX_LONG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SHORT] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_FRAG] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_JABBER] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_64] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_65_127] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_128_255] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_256_511] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_512_1023] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_SZ_JUMBO];
    stats.multicast = lan966x.stats[idx + SYS_COUNT_RX_MC] +
    lan966x.stats[idx + SYS_COUNT_RX_PMAC_MC];
    stats.rx_errors = lan966x.stats[idx + SYS_COUNT_RX_SHORT] +
    lan966x.stats[idx + SYS_COUNT_RX_FRAG] +
    lan966x.stats[idx + SYS_COUNT_RX_JABBER] +
    lan966x.stats[idx + SYS_COUNT_RX_CRC] +
    lan966x.stats[idx + SYS_COUNT_RX_SYMBOL_ERR] +
    lan966x.stats[idx + SYS_COUNT_RX_LONG];
    stats.rx_dropped = dev.stats.rx_dropped +
    lan966x.stats[idx + SYS_COUNT_RX_LONG] +
    lan966x.stats[idx + SYS_COUNT_DR_LOCAL] +
    lan966x.stats[idx + SYS_COUNT_DR_TAIL] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_0] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_1] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_2] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_3] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_4] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_5] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_6] +
    lan966x.stats[idx + SYS_COUNT_RX_RED_PRIO_7];
    for (i = 0; i < LAN966X_NUM_TC; i++) {
    stats.rx_dropped +=
    (lan966x.stats[idx + SYS_COUNT_DR_YELLOW_PRIO_0 + i] +
    lan966x.stats[idx + SYS_COUNT_DR_GREEN_PRIO_0 + i]);
    }
// Get Tx stats
    stats.tx_bytes = lan966x.stats[idx + SYS_COUNT_TX_OCT] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_OCT];
    stats.tx_packets = lan966x.stats[idx + SYS_COUNT_TX_SZ_64] +
    lan966x.stats[idx + SYS_COUNT_TX_SZ_65_127] +
    lan966x.stats[idx + SYS_COUNT_TX_SZ_128_255] +
    lan966x.stats[idx + SYS_COUNT_TX_SZ_256_511] +
    lan966x.stats[idx + SYS_COUNT_TX_SZ_512_1023] +
    lan966x.stats[idx + SYS_COUNT_TX_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_TX_SZ_JUMBO] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_64] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_65_127] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_128_255] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_256_511] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_512_1023] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_1024_1526] +
    lan966x.stats[idx + SYS_COUNT_TX_PMAC_SZ_JUMBO];
    stats.tx_dropped = lan966x.stats[idx + SYS_COUNT_TX_DROP] +
    lan966x.stats[idx + SYS_COUNT_TX_AGED];
    stats.collisions = lan966x.stats[idx + SYS_COUNT_TX_COL];
    spin_unlock(&lan966x.stats_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_stats_init(lan966x: *mut lan966x) -> c_int {
    int lan966x_stats_init(struct lan966x *lan966x)
    {
    char queue_name[32];
    lan966x.stats_layout = lan966x_stats_layout;
    lan966x.num_stats = ARRAY_SIZE(lan966x_stats_layout);
    lan966x.stats = devm_kcalloc(lan966x.dev, lan966x.num_phys_ports *
    lan966x.num_stats,
    sizeof(u64), GFP_KERNEL);
    if (!lan966x.stats)
    return -ENOMEM;
// Init stats worker
    spin_lock_init(&lan966x.stats_lock);
    snprintf(queue_name, sizeof(queue_name), "%s-stats",
    dev_name(lan966x.dev));
    lan966x.stats_queue = create_singlethread_workqueue(queue_name);
    if (!lan966x.stats_queue)
    return -ENOMEM;
    INIT_DELAYED_WORK(&lan966x.stats_work, lan966x_check_stats_work);
    queue_delayed_work(lan966x.stats_queue, &lan966x.stats_work,
    LAN966X_STATS_CHECK_DELAY);
    return 0;
    }
