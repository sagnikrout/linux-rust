//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/ks8995.c
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
//
// SPI driver for Micrel/Kendin KS8995M and KSZ8864RMN ethernet switches
//
// Copyright (C) 2008 Gabor Juhos <juhosg at openwrt.org>
// Copyright (C) 2025 Linus Walleij <linus.walleij@linaro.org>
//
// This file was based on: drivers/spi/at25.c
// Copyright (C) 2006 David Brownell
//

// ------------------------------------------------------------------------
pub const KS8995_REG_ID0: c_uint = 0x00    /* Chip ID0 */;
pub const KS8995_REG_ID1: c_uint = 0x01    /* Chip ID1 */;
pub const KS8995_REG_GC0: c_uint = 0x02    /* Global Control 0 */;

pub const KS8995_REG_GC1: c_uint = 0x03    /* Global Control 1 */;
pub const KS8995_REG_GC2: c_uint = 0x04    /* Global Control 2 */;

pub const KS8995_REG_GC3: c_uint = 0x05    /* Global Control 3 */;
pub const KS8995_REG_GC4: c_uint = 0x06    /* Global Control 4 */;

pub const KS8995_REG_GC5: c_uint = 0x07    /* Global Control 5 */;
pub const KS8995_REG_GC6: c_uint = 0x08    /* Global Control 6 */;
pub const KS8995_REG_GC7: c_uint = 0x09    /* Global Control 7 */;
pub const KS8995_REG_GC8: c_uint = 0x0a    /* Global Control 8 */;
pub const KS8995_REG_GC9: c_uint = 0x0b    /* Global Control 9 */;

// In DSA the ports 1-4 are numbered 0-3 and the CPU port is port 4

pub const KS8995_REG_PC0: c_uint = 0x00    /* Port Control 0 */;
pub const KS8995_REG_PC1: c_uint = 0x01    /* Port Control 1 */;
pub const KS8995_REG_PC2: c_uint = 0x02    /* Port Control 2 */;
pub const KS8995_REG_PC3: c_uint = 0x03    /* Port Control 3 */;
pub const KS8995_REG_PC4: c_uint = 0x04    /* Port Control 4 */;
pub const KS8995_REG_PC5: c_uint = 0x05    /* Port Control 5 */;
pub const KS8995_REG_PC6: c_uint = 0x06    /* Port Control 6 */;
pub const KS8995_REG_PC7: c_uint = 0x07    /* Port Control 7 */;
pub const KS8995_REG_PC8: c_uint = 0x08    /* Port Control 8 */;
pub const KS8995_REG_PC9: c_uint = 0x09    /* Port Control 9 */;
pub const KS8995_REG_PC10: c_uint = 0x0a    /* Port Control 10 */;
pub const KS8995_REG_PC11: c_uint = 0x0b    /* Port Control 11 */;
pub const KS8995_REG_PC12: c_uint = 0x0c    /* Port Control 12 */;
pub const KS8995_REG_PC13: c_uint = 0x0d    /* Port Control 13 */;

pub const KS8995_REG_TPC0: c_uint = 0x60    /* TOS Priority Control 0 */;
pub const KS8995_REG_TPC1: c_uint = 0x61    /* TOS Priority Control 1 */;
pub const KS8995_REG_TPC2: c_uint = 0x62    /* TOS Priority Control 2 */;
pub const KS8995_REG_TPC3: c_uint = 0x63    /* TOS Priority Control 3 */;
pub const KS8995_REG_TPC4: c_uint = 0x64    /* TOS Priority Control 4 */;
pub const KS8995_REG_TPC5: c_uint = 0x65    /* TOS Priority Control 5 */;
pub const KS8995_REG_TPC6: c_uint = 0x66    /* TOS Priority Control 6 */;
pub const KS8995_REG_TPC7: c_uint = 0x67    /* TOS Priority Control 7 */;
pub const KS8995_REG_MAC0: c_uint = 0x68    /* MAC address 0 */;
pub const KS8995_REG_MAC1: c_uint = 0x69    /* MAC address 1 */;
pub const KS8995_REG_MAC2: c_uint = 0x6a    /* MAC address 2 */;
pub const KS8995_REG_MAC3: c_uint = 0x6b    /* MAC address 3 */;
pub const KS8995_REG_MAC4: c_uint = 0x6c    /* MAC address 4 */;
pub const KS8995_REG_MAC5: c_uint = 0x6d    /* MAC address 5 */;
pub const KS8995_REG_IAC0: c_uint = 0x6e    /* Indirect Access Control 0 */;
pub const KS8995_REG_IAC1: c_uint = 0x6f    /* Indirect Access Control 0 */;
pub const KS8995_REG_IAD7: c_uint = 0x70    /* Indirect Access Data 7 */;
pub const KS8995_REG_IAD6: c_uint = 0x71    /* Indirect Access Data 6 */;
pub const KS8995_REG_IAD5: c_uint = 0x72    /* Indirect Access Data 5 */;
pub const KS8995_REG_IAD4: c_uint = 0x73    /* Indirect Access Data 4 */;
pub const KS8995_REG_IAD3: c_uint = 0x74    /* Indirect Access Data 3 */;
pub const KS8995_REG_IAD2: c_uint = 0x75    /* Indirect Access Data 2 */;
pub const KS8995_REG_IAD1: c_uint = 0x76    /* Indirect Access Data 1 */;
pub const KS8995_REG_IAD0: c_uint = 0x77    /* Indirect Access Data 0 */;
pub const KSZ8864_REG_ID1: c_uint = 0xfe	/* Chip ID in bit 7 */;
pub const KS8995_REGS_SIZE: c_uint = 0x80;
pub const KSZ8864_REGS_SIZE: c_uint = 0x100;
pub const KSZ8795_REGS_SIZE: c_uint = 0x100;
pub const ID1_CHIPID_M: c_uint = 0xf;
pub const ID1_CHIPID_S: c_int = 4;
pub const ID1_REVISION_M: c_uint = 0x7;
pub const ID1_REVISION_S: c_int = 1;

pub const FAMILY_KS8995: c_uint = 0x95;
pub const FAMILY_KSZ8795: c_uint = 0x87;
pub const CHIPID_M: c_int = 0;
pub const KS8995_CHIP_ID: c_uint = 0x00;
pub const KSZ8864_CHIP_ID: c_uint = 0x01;
pub const KSZ8795_CHIP_ID: c_uint = 0x09;
pub const KS8995_CMD_WRITE: c_uint = 0x02U;
pub const KS8995_CMD_READ: c_uint = 0x03U;
pub const KS8995_CPU_PORT: c_int = 4;

    enum ks8995_chip_variant {
    ks8995,
    ksz8864,
    ksz8795,
    max_variant
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ks8995_chip_params {
    pub name: *mut c_char,
    pub family_id: c_int,
    pub chip_id: c_int,
    pub regs_size: c_int,
    pub addr_width: c_int,
    pub addr_shift: c_int,
}

    static const struct ks8995_chip_params ks8995_chip[] = {
    [ks8995] = {
    .name = "KS8995MA",
    .family_id = FAMILY_KS8995,
    .chip_id = KS8995_CHIP_ID,
    .regs_size = KS8995_REGS_SIZE,
    .addr_width = 8,
    .addr_shift = 0,
    },
    [ksz8864] = {
    .name = "KSZ8864RMN",
    .family_id = FAMILY_KS8995,
    .chip_id = KSZ8864_CHIP_ID,
    .regs_size = KSZ8864_REGS_SIZE,
    .addr_width = 8,
    .addr_shift = 0,
    },
    [ksz8795] = {
    .name = "KSZ8795CLX",
    .family_id = FAMILY_KSZ8795,
    .chip_id = KSZ8795_CHIP_ID,
    .regs_size = KSZ8795_REGS_SIZE,
    .addr_width = 12,
    .addr_shift = 1,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ks8995_switch {
    pub spi: *mut spi_device,
    pub dev: *mut device,
    pub ds: *mut dsa_switch,
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub regs_attr: bin_attribute,
    pub chip: *const ks8995_chip_params,
    pub revision_id: c_int,
    pub max_mtu: [c_uint; KS8995_NUM_PORTS],
}

    static const struct spi_device_id ks8995_id[] = {
    {"ks8995", ks8995},
    {"ksz8864", ksz8864},
    {"ksz8795", ksz8795},
    { }
    };
    MODULE_DEVICE_TABLE(spi, ks8995_id);
    static const struct of_device_id ks8995_spi_of_match[] = {
    { .compatible = "micrel,ks8995" },
    { .compatible = "micrel,ksz8864" },
    { .compatible = "micrel,ksz8795" },
    { },
    };
    MODULE_DEVICE_TABLE(of, ks8995_spi_of_match);
#[no_mangle]
pub unsafe extern "C" fn get_chip_id(val: u8) -> u8 {
    static inline u8 get_chip_id(u8 val)
    {
    return (val >> ID1_CHIPID_S) & ID1_CHIPID_M;
    }
#[no_mangle]
pub unsafe extern "C" fn get_chip_rev(val: u8) -> u8 {
    static inline u8 get_chip_rev(u8 val)
    {
    return (val >> ID1_REVISION_S) & ID1_REVISION_M;
    }
// create_spi_cmd - create a chip specific SPI command header
// @ks: pointer to switch instance
// @cmd: SPI command for switch
// @address: register address for command
//
// Different chip families use different bit pattern to address the switches
// registers:
//
// KS8995: 8bit command + 8bit address
// KSZ8795: 3bit command + 12bit address + 1bit TR (?)
//
    static inline __be16 create_spi_cmd(struct ks8995_switch *ks, int cmd,
    unsigned address)
    {
    let mut result: u16 = cmd;
// make room for address (incl. address shift)
    result <<= ks.chip.addr_width + ks.chip.addr_shift;
// add address
    result |= address << ks.chip.addr_shift;
// SPI protocol needs big endian
    return cpu_to_be16(result);
    }
// ------------------------------------------------------------------------
    static int ks8995_read(struct ks8995_switch *ks, char *buf,
    unsigned offset, size_t count)
    {
    __be16 cmd;
    struct spi_transfer t[2];
    struct spi_message m;
    int err;
    cmd = create_spi_cmd(ks, KS8995_CMD_READ, offset);
    spi_message_init(&m);
    memset(&t, 0, sizeof(t));
    t[0].tx_buf = &cmd;
    t[0].len = sizeof(cmd);
    spi_message_add_tail(&t[0], &m);
    t[1].rx_buf = buf;
    t[1].len = count;
    spi_message_add_tail(&t[1], &m);
    mutex_lock(&ks.lock);
    err = spi_sync(ks.spi, &m);
    mutex_unlock(&ks.lock);
    return err ? err : count;
    }
    static int ks8995_write(struct ks8995_switch *ks, char *buf,
    unsigned offset, size_t count)
    {
    __be16 cmd;
    struct spi_transfer t[2];
    struct spi_message m;
    int err;
    cmd = create_spi_cmd(ks, KS8995_CMD_WRITE, offset);
    spi_message_init(&m);
    memset(&t, 0, sizeof(t));
    t[0].tx_buf = &cmd;
    t[0].len = sizeof(cmd);
    spi_message_add_tail(&t[0], &m);
    t[1].tx_buf = buf;
    t[1].len = count;
    spi_message_add_tail(&t[1], &m);
    mutex_lock(&ks.lock);
    err = spi_sync(ks.spi, &m);
    mutex_unlock(&ks.lock);
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn ks8995_read_reg(ks: *mut ks8995_switch, addr: u8, buf: *mut u8) -> c_int {
    static inline int ks8995_read_reg(struct ks8995_switch *ks, u8 addr, u8 *buf)
    {
    return ks8995_read(ks, buf, addr, 1) != 1;
    }
#[no_mangle]
pub unsafe extern "C" fn ks8995_write_reg(ks: *mut ks8995_switch, addr: u8, val: u8) -> c_int {
    static inline int ks8995_write_reg(struct ks8995_switch *ks, u8 addr, u8 val)
    {
    let mut buf: c_char = val;
    return ks8995_write(ks, &buf, addr, 1) != 1;
    }
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn ks8995_stop(ks: *mut ks8995_switch) -> c_int {
    static int ks8995_stop(struct ks8995_switch *ks)
    {
    return ks8995_write_reg(ks, KS8995_REG_ID1, 0);
    }
#[no_mangle]
unsafe extern "C" fn ks8995_start(ks: *mut ks8995_switch) -> c_int {
    static int ks8995_start(struct ks8995_switch *ks)
    {
    return ks8995_write_reg(ks, KS8995_REG_ID1, 1);
    }
#[no_mangle]
unsafe extern "C" fn ks8995_reset(ks: *mut ks8995_switch) -> c_int {
    static int ks8995_reset(struct ks8995_switch *ks)
    {
    int err;
    err = ks8995_stop(ks);
    if (err)
    return err;
    udelay(KS8995_RESET_DELAY);
    return ks8995_start(ks);
    }
// ks8995_get_revision - get chip revision
// @ks: pointer to switch instance
//
// Verify chip family and id and get chip revision.
//
#[no_mangle]
unsafe extern "C" fn ks8995_get_revision(ks: *mut ks8995_switch) -> c_int {
    static int ks8995_get_revision(struct ks8995_switch *ks)
    {
    int err;
    u8 id0, id1, ksz8864_id;
// read family id
    err = ks8995_read_reg(ks, KS8995_REG_ID0, &id0);
    if (err) {
    err = -EIO;
    goto err_out;
    }
// verify family id
    if (id0 != ks.chip.family_id) {
    dev_err(&ks.spi.dev, "chip family id mismatch: expected 0x%02x but 0x%02x read\n",
    ks.chip.family_id, id0);
    err = -ENODEV;
    goto err_out;
    }
    switch (ks.chip.family_id) {
    case FAMILY_KS8995:
// try reading chip id at CHIP ID1
    err = ks8995_read_reg(ks, KS8995_REG_ID1, &id1);
    if (err) {
    err = -EIO;
    goto err_out;
    }
// verify chip id
    if ((get_chip_id(id1) == CHIPID_M) &&
    (get_chip_id(id1) == ks.chip.chip_id)) {
// KS8995MA
    ks.revision_id = get_chip_rev(id1);
    } else if (get_chip_id(id1) != CHIPID_M) {
// KSZ8864RMN
    err = ks8995_read_reg(ks, KS8995_REG_ID1, &ksz8864_id);
    if (err) {
    err = -EIO;
    goto err_out;
    }
    if ((ksz8864_id & 0x80) &&
    (ks.chip.chip_id == KSZ8864_CHIP_ID)) {
    ks.revision_id = get_chip_rev(id1);
    }
    } else {
    dev_err(&ks.spi.dev, "unsupported chip id for KS8995 family: 0x%02x\n",
    id1);
    err = -ENODEV;
    }
    break;
    case FAMILY_KSZ8795:
// try reading chip id at CHIP ID1
    err = ks8995_read_reg(ks, KS8995_REG_ID1, &id1);
    if (err) {
    err = -EIO;
    goto err_out;
    }
    if (get_chip_id(id1) == ks.chip.chip_id) {
    ks.revision_id = get_chip_rev(id1);
    } else {
    dev_err(&ks.spi.dev, "unsupported chip id for KSZ8795 family: 0x%02x\n",
    id1);
    err = -ENODEV;
    }
    break;
    default:
    dev_err(&ks.spi.dev, "unsupported family id: 0x%02x\n", id0);
    err = -ENODEV;
    break;
    }
    err_out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ks8995_check_config(ks: *mut ks8995_switch) -> c_int {
    static int ks8995_check_config(struct ks8995_switch *ks)
    {
    int ret;
    u8 val;
    ret = ks8995_read_reg(ks, KS8995_REG_GC0, &val);
    if (ret) {
    dev_err(ks.dev, "failed to read KS8995_REG_GC0\n");
    return ret;
    }
    dev_dbg(ks.dev, "port 5 PHY %senabled\n",
    (val & KS8995_GC0_P5_PHY) ? "" : "not ");
    val |= KS8995_GC0_P5_PHY;
    ret = ks8995_write_reg(ks, KS8995_REG_GC0, val);
    if (ret)
    dev_err(ks.dev, "failed to set KS8995_REG_GC0\n");
    dev_dbg(ks.dev, "set KS8995_REG_GC0 to 0x%02x\n", val);
    return 0;
    }
    static void
    ks8995_mac_config(struct phylink_config *config, unsigned int mode,
    const struct phylink_link_state *state)
    {
    }
    static void
    ks8995_mac_link_up(struct phylink_config *config, struct phy_device *phydev,
    unsigned int mode, phy_interface_t interface,
    int speed, int duplex, bool tx_pause, bool rx_pause)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct ks8995_switch *ks = dp.ds.priv;
    let mut port: c_int = dp.index;
    int ret;
    u8 val;
// Allow forcing the mode on the fixed CPU port, no autonegotiation.
// We assume autonegotiation works on the PHY-facing ports.
//
    if (port != KS8995_CPU_PORT)
    return;
    dev_dbg(ks.dev, "MAC link up on CPU port (%d)\n", port);
    ret = ks8995_read_reg(ks, KS8995_REG_GC4, &val);
    if (ret) {
    dev_err(ks.dev, "failed to read KS8995_REG_GC4\n");
    return;
    }
// Conjure port config
    switch (speed) {
    case SPEED_10:
    dev_dbg(ks.dev, "set switch MII to 100Mbit mode\n");
    val |= KS8995_GC4_10BT;
    break;
    case SPEED_100:
    default:
    dev_dbg(ks.dev, "set switch MII to 100Mbit mode\n");
    val &= ~KS8995_GC4_10BT;
    break;
    }
    if (duplex == DUPLEX_HALF) {
    dev_dbg(ks.dev, "set switch MII to half duplex\n");
    val |= KS8995_GC4_MII_HD;
    } else {
    dev_dbg(ks.dev, "set switch MII to full duplex\n");
    val &= ~KS8995_GC4_MII_HD;
    }
    dev_dbg(ks.dev, "set KS8995_REG_GC4 to %02x\n", val);
// Enable the CPU port
    ret = ks8995_write_reg(ks, KS8995_REG_GC4, val);
    if (ret)
    dev_err(ks.dev, "failed to set KS8995_REG_GC4\n");
    }
    static void
    ks8995_mac_link_down(struct phylink_config *config, unsigned int mode,
    phy_interface_t interface)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct ks8995_switch *ks = dp.ds.priv;
    let mut port: c_int = dp.index;
    if (port != KS8995_CPU_PORT)
    return;
    dev_dbg(ks.dev, "MAC link down on CPU port (%d)\n", port);
// Disable the CPU port
    }
    static const struct phylink_mac_ops ks8995_phylink_mac_ops = {
    .mac_config = ks8995_mac_config,
    .mac_link_up = ks8995_mac_link_up,
    .mac_link_down = ks8995_mac_link_down,
    };
    static enum
    dsa_tag_protocol ks8995_get_tag_protocol(struct dsa_switch *ds,
    int port,
    enum dsa_tag_protocol mp)
    {
// This switch actually uses the 6 byte KS8995 protocol
    return DSA_TAG_PROTO_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ks8995_setup(ds: *mut dsa_switch) -> c_int {
    static int ks8995_setup(struct dsa_switch *ds)
    {
    return 0;
    }
    static int ks8995_port_enable(struct dsa_switch *ds, int port,
    struct phy_device *phy)
    {
    struct ks8995_switch *ks = ds.priv;
    dev_dbg(ks.dev, "enable port %d\n", port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ks8995_port_disable(ds: *mut dsa_switch, port: c_int) {
    static void ks8995_port_disable(struct dsa_switch *ds, int port)
    {
    struct ks8995_switch *ks = ds.priv;
    dev_dbg(ks.dev, "disable port %d\n", port);
    }
    static int ks8995_port_pre_bridge_flags(struct dsa_switch *ds, int port,
    struct switchdev_brport_flags flags,
    struct netlink_ext_ack *extack)
    {
// We support enabling/disabling learning
    if (flags.mask & ~(BR_LEARNING))
    return -EINVAL;
    return 0;
    }
    static int ks8995_port_bridge_flags(struct dsa_switch *ds, int port,
    struct switchdev_brport_flags flags,
    struct netlink_ext_ack *extack)
    {
    struct ks8995_switch *ks = ds.priv;
    int ret;
    u8 val;
    if (flags.mask & BR_LEARNING) {
    ret = ks8995_read_reg(ks, KS8995_REG_PC(port, KS8995_REG_PC2), &val);
    if (ret) {
    dev_err(ks.dev, "failed to read KS8995_REG_PC2 on port %d\n", port);
    return ret;
    }
    if (flags.val & BR_LEARNING)
    val &= ~KS8995_PC2_LEARN_DIS;
    else
    val |= KS8995_PC2_LEARN_DIS;
    ret = ks8995_write_reg(ks, KS8995_REG_PC(port, KS8995_REG_PC2), val);
    if (ret) {
    dev_err(ks.dev, "failed to write KS8995_REG_PC2 on port %d\n", port);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ks8995_port_stp_state_set(ds: *mut dsa_switch, port: c_int, state: u8) {
    static void ks8995_port_stp_state_set(struct dsa_switch *ds, int port, u8 state)
    {
    struct ks8995_switch *ks = ds.priv;
    int ret;
    u8 val;
    ret = ks8995_read_reg(ks, KS8995_REG_PC(port, KS8995_REG_PC2), &val);
    if (ret) {
    dev_err(ks.dev, "failed to read KS8995_REG_PC2 on port %d\n", port);
    return;
    }
// Set the bits for the different STP states in accordance with
// the datasheet, pages 36-37 "Spanning tree support".
//
    switch (state) {
    case BR_STATE_DISABLED:
    case BR_STATE_BLOCKING:
    case BR_STATE_LISTENING:
    val &= ~KS8995_PC2_TXEN;
    val &= ~KS8995_PC2_RXEN;
    val |= KS8995_PC2_LEARN_DIS;
    break;
    case BR_STATE_LEARNING:
    val &= ~KS8995_PC2_TXEN;
    val &= ~KS8995_PC2_RXEN;
    val &= ~KS8995_PC2_LEARN_DIS;
    break;
    case BR_STATE_FORWARDING:
    val |= KS8995_PC2_TXEN;
    val |= KS8995_PC2_RXEN;
    val &= ~KS8995_PC2_LEARN_DIS;
    break;
    default:
    dev_err(ks.dev, "unknown bridge state requested\n");
    return;
    }
    ret = ks8995_write_reg(ks, KS8995_REG_PC(port, KS8995_REG_PC2), val);
    if (ret) {
    dev_err(ks.dev, "failed to write KS8995_REG_PC2 on port %d\n", port);
    return;
    }
    dev_dbg(ks.dev, "set KS8995_REG_PC2 for port %d to %02x\n", port, val);
    }
    static void ks8995_phylink_get_caps(struct dsa_switch *dsa, int port,
    struct phylink_config *config)
    {
    unsigned long *interfaces = config.supported_interfaces;
    if (port == KS8995_CPU_PORT)
    __set_bit(PHY_INTERFACE_MODE_MII, interfaces);
    if (port <= 3) {
// Internal PHYs
    __set_bit(PHY_INTERFACE_MODE_INTERNAL, interfaces);
// phylib default
    __set_bit(PHY_INTERFACE_MODE_MII, interfaces);
    }
    config.mac_capabilities = MAC_SYM_PAUSE | MAC_10 | MAC_100;
    }
// Huge packet support up to 1916 byte packages "inclusive"
// which means that tags are included. If the bit is not set
// it is 1536 bytes "inclusive". We present the length without
// tags or ethernet headers. The setting affects all ports.
//
#[no_mangle]
unsafe extern "C" fn ks8995_change_mtu(ds: *mut dsa_switch, port: c_int, new_mtu: c_int) -> c_int {
    static int ks8995_change_mtu(struct dsa_switch *ds, int port, int new_mtu)
    {
    struct ks8995_switch *ks = ds.priv;
    unsigned int max_mtu;
    int ret;
    u8 val;
    int i;
    ks.max_mtu[port] = new_mtu;
// Roof out the MTU for the entire switch to the greatest
// common denominator: the biggest set for any one port will
// be the biggest MTU for the switch.
//
    max_mtu = ETH_DATA_LEN;
    for (i = 0; i < KS8995_NUM_PORTS; i++) {
    if (ks.max_mtu[i] > max_mtu)
    max_mtu = ks.max_mtu[i];
    }
// Translate to layer 2 size.
// Add ethernet and (possible) VLAN headers, and checksum to the size.
// For ETH_DATA_LEN (1500 bytes) this will add up to 1522 bytes.
//
    max_mtu += VLAN_ETH_HLEN;
    max_mtu += ETH_FCS_LEN;
    ret = ks8995_read_reg(ks, KS8995_REG_GC2, &val);
    if (ret) {
    dev_err(ks.dev, "failed to read KS8995_REG_GC2\n");
    return ret;
    }
    if (max_mtu <= 1522) {
    val &= ~KS8995_GC2_HUGE;
    val &= ~KS8995_GC2_LEGAL;
    } else if (max_mtu > 1522 && max_mtu <= 1536) {
// This accepts packets up to 1536 bytes
    val &= ~KS8995_GC2_HUGE;
    val |= KS8995_GC2_LEGAL;
    } else {
// This accepts packets up to 1916 bytes
    val |= KS8995_GC2_HUGE;
    val |= KS8995_GC2_LEGAL;
    }
    dev_dbg(ks.dev, "new max MTU %d bytes (inclusive)\n", max_mtu);
    ret = ks8995_write_reg(ks, KS8995_REG_GC2, val);
    if (ret)
    dev_err(ks.dev, "failed to set KS8995_REG_GC2\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ks8995_get_max_mtu(ds: *mut dsa_switch, port: c_int) -> c_int {
    static int ks8995_get_max_mtu(struct dsa_switch *ds, int port)
    {
    return 1916 - ETH_HLEN - ETH_FCS_LEN;
    }
    static const struct dsa_switch_ops ks8995_ds_ops = {
    .get_tag_protocol = ks8995_get_tag_protocol,
    .setup = ks8995_setup,
    .port_pre_bridge_flags = ks8995_port_pre_bridge_flags,
    .port_bridge_flags = ks8995_port_bridge_flags,
    .port_enable = ks8995_port_enable,
    .port_disable = ks8995_port_disable,
    .port_stp_state_set = ks8995_port_stp_state_set,
    .port_change_mtu = ks8995_change_mtu,
    .port_max_mtu = ks8995_get_max_mtu,
    .phylink_get_caps = ks8995_phylink_get_caps,
    };
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn ks8995_probe(spi: *mut spi_device) -> c_int {
    static int ks8995_probe(struct spi_device *spi)
    {
    struct ks8995_switch *ks;
    int err;
    let mut variant: c_int = spi_get_device_id(spi).driver_data;
    if (variant >= max_variant) {
    dev_err(&spi.dev, "bad chip variant %d\n", variant);
    return -ENODEV;
    }
    ks = devm_kzalloc(&spi.dev, sizeof(*ks), GFP_KERNEL);
    if (!ks)
    return -ENOMEM;
    mutex_init(&ks.lock);
    ks.spi = spi;
    ks.dev = &spi.dev;
    ks.chip = &ks8995_chip[variant];
    ks.reset_gpio = devm_gpiod_get_optional(&spi.dev, "reset",
    GPIOD_OUT_HIGH);
    err = PTR_ERR_OR_ZERO(ks.reset_gpio);
    if (err) {
    dev_err(&spi.dev,
    "failed to get reset gpio: %d\n", err);
    return err;
    }
    err = gpiod_set_consumer_name(ks.reset_gpio, "switch-reset");
    if (err)
    return err;
    if (ks.reset_gpio) {
//
// If a reset line was obtained, wait for 100us after
// de-asserting RESET before accessing any registers, see
// the KS8995MA datasheet, page 44.
//
    gpiod_set_value_cansleep(ks.reset_gpio, 0);
    udelay(100);
    }
    spi_set_drvdata(spi, ks);
    spi.mode = SPI_MODE_0;
    spi.bits_per_word = 8;
    err = spi_setup(spi);
    if (err) {
    dev_err(&spi.dev, "spi_setup failed, err=%d\n", err);
    return err;
    }
    err = ks8995_get_revision(ks);
    if (err)
    return err;
    err = ks8995_reset(ks);
    if (err)
    return err;
    dev_info(&spi.dev, "%s device found, Chip ID:%x, Revision:%x\n",
    ks.chip.name, ks.chip.chip_id, ks.revision_id);
    err = ks8995_check_config(ks);
    if (err)
    return err;
    ks.ds = devm_kzalloc(&spi.dev, sizeof(*ks.ds), GFP_KERNEL);
    if (!ks.ds)
    return -ENOMEM;
    ks.ds.dev = &spi.dev;
    ks.ds.num_ports = KS8995_NUM_PORTS;
    ks.ds.ops = &ks8995_ds_ops;
    ks.ds.phylink_mac_ops = &ks8995_phylink_mac_ops;
    ks.ds.priv = ks;
    err = dsa_register_switch(ks.ds);
    if (err)
    return dev_err_probe(&spi.dev, err,
    "unable to register DSA switch\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ks8995_remove(spi: *mut spi_device) {
    static void ks8995_remove(struct spi_device *spi)
    {
    struct ks8995_switch *ks = spi_get_drvdata(spi);
    dsa_unregister_switch(ks.ds);
// assert reset
    gpiod_set_value_cansleep(ks.reset_gpio, 1);
    }
// ------------------------------------------------------------------------
    static struct spi_driver ks8995_driver = {
    .driver = {
    .name	    = "spi-ks8995",
    .of_match_table = ks8995_spi_of_match,
    },
    .probe	  = ks8995_probe,
    .remove	  = ks8995_remove,
    .id_table = ks8995_id,
    };
    module_spi_driver(ks8995_driver);
    MODULE_DESCRIPTION(DRV_DESC);
    MODULE_VERSION(DRV_VERSION);
    MODULE_AUTHOR("Gabor Juhos <juhosg at openwrt.org>");
    MODULE_LICENSE("GPL v2");
