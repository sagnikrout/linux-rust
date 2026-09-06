//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/sfc/falcon/qt202x_phy.c
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
// Driver for Solarflare network controllers and boards
// Copyright 2006-2012 Solarflare Communications Inc.
//
// Driver for AMCC QT202x SFP+ and XFP adapters; see www.amcc.com for details
//

    MDIO_DEVS_PMAPMD |	\
    MDIO_DEVS_PHYXS)

    (1 << LOOPBACK_PMAPMD) |	\
    (1 << LOOPBACK_PHYXS_WS))
//
// Quake-specific MDIO registers

// QT2025C only
pub const PCS_FW_HEARTBEAT_REG: c_uint = 0xd7ee;
pub const PCS_FW_HEARTB_LBN: c_int = 0;
pub const PCS_FW_HEARTB_WIDTH: c_int = 8;
pub const PCS_FW_PRODUCT_CODE_1: c_uint = 0xd7f0;
pub const PCS_FW_VERSION_1: c_uint = 0xd7f3;
pub const PCS_FW_BUILD_1: c_uint = 0xd7f6;
pub const PCS_UC8051_STATUS_REG: c_uint = 0xd7fd;
pub const PCS_UC_STATUS_LBN: c_int = 0;
pub const PCS_UC_STATUS_WIDTH: c_int = 8;
pub const PCS_UC_STATUS_FW_SAVE: c_uint = 0x20;
pub const PMA_PMD_MODE_REG: c_uint = 0xc301;
pub const PMA_PMD_RXIN_SEL_LBN: c_int = 6;
pub const PMA_PMD_FTX_CTRL2_REG: c_uint = 0xc309;
pub const PMA_PMD_FTX_STATIC_LBN: c_int = 13;
pub const PMA_PMD_VEND1_REG: c_uint = 0xc001;
pub const PMA_PMD_VEND1_LBTXD_LBN: c_int = 15;
pub const PCS_VEND1_REG: c_uint = 0xc000;
pub const PCS_VEND1_LBTXD_LBN: c_int = 5;
#[no_mangle]
pub unsafe extern "C" fn falcon_qt202x_set_led(p: *mut ef4_nic, led: c_int, mode: c_int) {
    void falcon_qt202x_set_led(struct ef4_nic *p, int led, int mode)
    {
    let mut addr: c_int = MDIO_QUAKE_LED0_REG + led;
    ef4_mdio_write(p, MDIO_MMD_PMAPMD, addr, mode);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt202x_phy_data {
    pub phy_mode: enum ef4_phy_mode,
    pub bug17190_in_bad_state: bool,
    pub bug17190_timer: c_ulong,
    pub firmware_ver: u32,
}

pub const QT2022C2_MAX_RESET_TIME: c_int = 500;
pub const QT2022C2_RESET_WAIT: c_int = 10;

pub const QT2025C_HEARTB_WAIT: c_int = 100;

pub const QT2025C_FWSTART_WAIT: c_int = 100;

#[no_mangle]
unsafe extern "C" fn qt2025c_wait_heartbeat(efx: *mut ef4_nic) -> c_int {
    static int qt2025c_wait_heartbeat(struct ef4_nic *efx)
    {
    let mut timeout: c_ulong = jiffies + QT2025C_MAX_HEARTB_TIME;
    int reg, old_counter = 0;
// Wait for firmware heartbeat to start
    for (;;) {
    int counter;
    reg = ef4_mdio_read(efx, MDIO_MMD_PCS, PCS_FW_HEARTBEAT_REG);
    if (reg < 0)
    return reg;
    counter = ((reg >> PCS_FW_HEARTB_LBN) &
    ((1 << PCS_FW_HEARTB_WIDTH) - 1));
    if (old_counter == 0)
    old_counter = counter;
#[no_mangle]
pub unsafe extern "C" fn if(old_counter: counter !=) -> else {
    else if (counter != old_counter)
    break;
    if (time_after(jiffies, timeout)) {
// Some cables have EEPROMs that conflict with the
// PHY's on-board EEPROM so it cannot load firmware
    netif_err(efx, hw, efx.net_dev,
    "If an SFP+ direct attach cable is"
    " connected, please check that it complies"
    " with the SFP+ specification\n");
    return -ETIMEDOUT;
    }
    msleep(QT2025C_HEARTB_WAIT);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt2025c_wait_fw_status_good(efx: *mut ef4_nic) -> c_int {
    static int qt2025c_wait_fw_status_good(struct ef4_nic *efx)
    {
    let mut timeout: c_ulong = jiffies + QT2025C_MAX_FWSTART_TIME;
    int reg;
// Wait for firmware status to look good
    for (;;) {
    reg = ef4_mdio_read(efx, MDIO_MMD_PCS, PCS_UC8051_STATUS_REG);
    if (reg < 0)
    return reg;
    if ((reg &
    ((1 << PCS_UC_STATUS_WIDTH) - 1) << PCS_UC_STATUS_LBN) >=
    PCS_UC_STATUS_FW_SAVE)
    break;
    if (time_after(jiffies, timeout))
    return -ETIMEDOUT;
    msleep(QT2025C_FWSTART_WAIT);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt2025c_restart_firmware(efx: *mut ef4_nic) {
    static void qt2025c_restart_firmware(struct ef4_nic *efx)
    {
// Restart microcontroller execution of firmware from RAM
    ef4_mdio_write(efx, 3, 0xe854, 0x00c0);
    ef4_mdio_write(efx, 3, 0xe854, 0x0040);
    msleep(50);
    }
#[no_mangle]
unsafe extern "C" fn qt2025c_wait_reset(efx: *mut ef4_nic) -> c_int {
    static int qt2025c_wait_reset(struct ef4_nic *efx)
    {
    int rc;
    rc = qt2025c_wait_heartbeat(efx);
    if (rc != 0)
    return rc;
    rc = qt2025c_wait_fw_status_good(efx);
    if (rc == -ETIMEDOUT) {
// Bug 17689: occasionally heartbeat starts but firmware status
// code never progresses beyond 0x00.  Try again, once, after
// restarting execution of the firmware image.
    netif_dbg(efx, hw, efx.net_dev,
    "bashing QT2025C microcontroller\n");
    qt2025c_restart_firmware(efx);
    rc = qt2025c_wait_heartbeat(efx);
    if (rc != 0)
    return rc;
    rc = qt2025c_wait_fw_status_good(efx);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn qt2025c_firmware_id(efx: *mut ef4_nic) {
    static void qt2025c_firmware_id(struct ef4_nic *efx)
    {
    struct qt202x_phy_data *phy_data = efx.phy_data;
    u8 firmware_id[9];
    size_t i;
    for (i = 0; i < sizeof(firmware_id); i++)
    firmware_id[i] = ef4_mdio_read(efx, MDIO_MMD_PCS,
    PCS_FW_PRODUCT_CODE_1 + i);
    netif_info(efx, probe, efx.net_dev,
    "QT2025C firmware %xr%d v%d.%d.%d.%d [20%02d-%02d-%02d]\n",
    (firmware_id[0] << 8) | firmware_id[1], firmware_id[2],
    firmware_id[3] >> 4, firmware_id[3] & 0xf,
    firmware_id[4], firmware_id[5],
    firmware_id[6], firmware_id[7], firmware_id[8]);
    phy_data.firmware_ver = ((firmware_id[3] & 0xf0) << 20) |
    ((firmware_id[3] & 0x0f) << 16) |
    (firmware_id[4] << 8) | firmware_id[5];
    }
#[no_mangle]
unsafe extern "C" fn qt2025c_bug17190_workaround(efx: *mut ef4_nic) {
    static void qt2025c_bug17190_workaround(struct ef4_nic *efx)
    {
    struct qt202x_phy_data *phy_data = efx.phy_data;
// The PHY can get stuck in a state where it reports PHY_XS and PMA/PMD
// layers up, but PCS down (no block_lock).  If we notice this state
// persisting for a couple of seconds, we switch PMA/PMD loopback
// briefly on and then off again, which is normally sufficient to
// recover it.
//
    if (efx.link_state.up ||
    !ef4_mdio_links_ok(efx, MDIO_DEVS_PMAPMD | MDIO_DEVS_PHYXS)) {
    phy_data.bug17190_in_bad_state = false;
    return;
    }
    if (!phy_data.bug17190_in_bad_state) {
    phy_data.bug17190_in_bad_state = true;
    phy_data.bug17190_timer = jiffies + BUG17190_INTERVAL;
    return;
    }
    if (time_after_eq(jiffies, phy_data.bug17190_timer)) {
    netif_dbg(efx, hw, efx.net_dev, "bashing QT2025C PMA/PMD\n");
    ef4_mdio_set_flag(efx, MDIO_MMD_PMAPMD, MDIO_CTRL1,
    MDIO_PMA_CTRL1_LOOPBACK, true);
    msleep(100);
    ef4_mdio_set_flag(efx, MDIO_MMD_PMAPMD, MDIO_CTRL1,
    MDIO_PMA_CTRL1_LOOPBACK, false);
    phy_data.bug17190_timer = jiffies + BUG17190_INTERVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn qt2025c_select_phy_mode(efx: *mut ef4_nic) -> c_int {
    static int qt2025c_select_phy_mode(struct ef4_nic *efx)
    {
    struct qt202x_phy_data *phy_data = efx.phy_data;
    struct falcon_board *board = falcon_board(efx);
    int reg, rc, i;
    uint16_t phy_op_mode;
// Only 2.0.1.0+ PHY firmware supports the more optimal SFP+
// Self-Configure mode.  Don't attempt any switching if we encounter
// older firmware.
    if (phy_data.firmware_ver < 0x02000100)
    return 0;
// In general we will get optimal behaviour in "SFP+ Self-Configure"
// mode; however, that powers down most of the PHY when no module is
// present, so we must use a different mode (any fixed mode will do)
// to be sure that loopbacks will work.
    phy_op_mode = (efx.loopback_mode == LOOPBACK_NONE) ? 0x0038 : 0x0020;
// Only change mode if really necessary
    reg = ef4_mdio_read(efx, 1, 0xc319);
    if ((reg & 0x0038) == phy_op_mode)
    return 0;
    netif_dbg(efx, hw, efx.net_dev, "Switching PHY to mode 0x%04x\n",
    phy_op_mode);
// This sequence replicates the register writes configured in the boot
// EEPROM (including the differences between board revisions), except
// that the operating mode is changed, and the PHY is prevented from
// unnecessarily reloading the main firmware image again.
    ef4_mdio_write(efx, 1, 0xc300, 0x0000);
// (Note: this portion of the boot EEPROM sequence, which bit-bashes 9
// STOPs onto the firmware/module I2C bus to reset it, varies across
// board revisions, as the bus is connected to different GPIO/LED
// outputs on the PHY.)
    if (board.major == 0 && board.minor < 2) {
    ef4_mdio_write(efx, 1, 0xc303, 0x4498);
    for (i = 0; i < 9; i++) {
    ef4_mdio_write(efx, 1, 0xc303, 0x4488);
    ef4_mdio_write(efx, 1, 0xc303, 0x4480);
    ef4_mdio_write(efx, 1, 0xc303, 0x4490);
    ef4_mdio_write(efx, 1, 0xc303, 0x4498);
    }
    } else {
    ef4_mdio_write(efx, 1, 0xc303, 0x0920);
    ef4_mdio_write(efx, 1, 0xd008, 0x0004);
    for (i = 0; i < 9; i++) {
    ef4_mdio_write(efx, 1, 0xc303, 0x0900);
    ef4_mdio_write(efx, 1, 0xd008, 0x0005);
    ef4_mdio_write(efx, 1, 0xc303, 0x0920);
    ef4_mdio_write(efx, 1, 0xd008, 0x0004);
    }
    ef4_mdio_write(efx, 1, 0xc303, 0x4900);
    }
    ef4_mdio_write(efx, 1, 0xc303, 0x4900);
    ef4_mdio_write(efx, 1, 0xc302, 0x0004);
    ef4_mdio_write(efx, 1, 0xc316, 0x0013);
    ef4_mdio_write(efx, 1, 0xc318, 0x0054);
    ef4_mdio_write(efx, 1, 0xc319, phy_op_mode);
    ef4_mdio_write(efx, 1, 0xc31a, 0x0098);
    ef4_mdio_write(efx, 3, 0x0026, 0x0e00);
    ef4_mdio_write(efx, 3, 0x0027, 0x0013);
    ef4_mdio_write(efx, 3, 0x0028, 0xa528);
    ef4_mdio_write(efx, 1, 0xd006, 0x000a);
    ef4_mdio_write(efx, 1, 0xd007, 0x0009);
    ef4_mdio_write(efx, 1, 0xd008, 0x0004);
// This additional write is not present in the boot EEPROM.  It
// prevents the PHY's internal boot ROM doing another pointless (and
// slow) reload of the firmware image (the microcontroller's code
// memory is not affected by the microcontroller reset).
    ef4_mdio_write(efx, 1, 0xc317, 0x00ff);
// PMA/PMD loopback sets RXIN to inverse polarity and the firmware
// restart doesn't reset it. We need to do that ourselves.
    ef4_mdio_set_flag(efx, 1, PMA_PMD_MODE_REG,
    1 << PMA_PMD_RXIN_SEL_LBN, false);
    ef4_mdio_write(efx, 1, 0xc300, 0x0002);
    msleep(20);
// Restart microcontroller execution of firmware from RAM
    qt2025c_restart_firmware(efx);
// Wait for the microcontroller to be ready again
    rc = qt2025c_wait_reset(efx);
    if (rc < 0) {
    netif_err(efx, hw, efx.net_dev,
    "PHY microcontroller reset during mode switch "
    "timed out\n");
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt202x_reset_phy(efx: *mut ef4_nic) -> c_int {
    static int qt202x_reset_phy(struct ef4_nic *efx)
    {
    int rc;
    if (efx.phy_type == PHY_TYPE_QT2025C) {
// Wait for the reset triggered by falcon_reset_hw()
// to complete
    rc = qt2025c_wait_reset(efx);
    if (rc < 0)
    goto fail;
    } else {
// Reset the PHYXS MMD. This is documented as doing
// a complete soft reset.
    rc = ef4_mdio_reset_mmd(efx, MDIO_MMD_PHYXS,
    QT2022C2_MAX_RESET_TIME /
    QT2022C2_RESET_WAIT,
    QT2022C2_RESET_WAIT);
    if (rc < 0)
    goto fail;
    }
// Wait 250ms for the PHY to complete bootup
    msleep(250);
    falcon_board(efx).type.init_phy(efx);
    return 0;
    fail:
    netif_err(efx, hw, efx.net_dev, "PHY reset timed out\n");
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn qt202x_phy_probe(efx: *mut ef4_nic) -> c_int {
    static int qt202x_phy_probe(struct ef4_nic *efx)
    {
    struct qt202x_phy_data *phy_data;
    phy_data = kzalloc_obj(struct qt202x_phy_data);
    if (!phy_data)
    return -ENOMEM;
    efx.phy_data = phy_data;
    phy_data.phy_mode = efx.phy_mode;
    phy_data.bug17190_in_bad_state = false;
    phy_data.bug17190_timer = 0;
    efx.mdio.mmds = QT202X_REQUIRED_DEVS;
    efx.mdio.mode_support = MDIO_SUPPORTS_C45 | MDIO_EMULATE_C22;
    efx.loopback_modes = QT202X_LOOPBACKS | FALCON_XMAC_LOOPBACKS;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt202x_phy_init(efx: *mut ef4_nic) -> c_int {
    static int qt202x_phy_init(struct ef4_nic *efx)
    {
    u32 devid;
    int rc;
    rc = qt202x_reset_phy(efx);
    if (rc) {
    netif_err(efx, probe, efx.net_dev, "PHY init failed\n");
    return rc;
    }
    devid = ef4_mdio_read_id(efx, MDIO_MMD_PHYXS);
    netif_info(efx, probe, efx.net_dev,
    "PHY ID reg %x (OUI %06x model %02x revision %x)\n",
    devid, ef4_mdio_id_oui(devid), ef4_mdio_id_model(devid),
    ef4_mdio_id_rev(devid));
    if (efx.phy_type == PHY_TYPE_QT2025C)
    qt2025c_firmware_id(efx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt202x_link_ok(efx: *mut ef4_nic) -> c_int {
    static int qt202x_link_ok(struct ef4_nic *efx)
    {
    return ef4_mdio_links_ok(efx, QT202X_REQUIRED_DEVS);
    }
#[no_mangle]
unsafe extern "C" fn qt202x_phy_poll(efx: *mut ef4_nic) -> bool {
    static bool qt202x_phy_poll(struct ef4_nic *efx)
    {
    let mut was_up: bool = efx.link_state.up;
    efx.link_state.up = qt202x_link_ok(efx);
    efx.link_state.speed = 10000;
    efx.link_state.fd = true;
    efx.link_state.fc = efx.wanted_fc;
    if (efx.phy_type == PHY_TYPE_QT2025C)
    qt2025c_bug17190_workaround(efx);
    return efx.link_state.up != was_up;
    }
#[no_mangle]
unsafe extern "C" fn qt202x_phy_reconfigure(efx: *mut ef4_nic) -> c_int {
    static int qt202x_phy_reconfigure(struct ef4_nic *efx)
    {
    struct qt202x_phy_data *phy_data = efx.phy_data;
    if (efx.phy_type == PHY_TYPE_QT2025C) {
    let mut rc: c_int = qt2025c_select_phy_mode(efx);
    if (rc)
    return rc;
// There are several different register bits which can
// disable TX (and save power) on direct-attach cables
// or optical transceivers, varying somewhat between
// firmware versions.  Only 'static mode' appears to
// cover everything.
    mdio_set_flag(
    &efx.mdio, efx.mdio.prtad, MDIO_MMD_PMAPMD,
    PMA_PMD_FTX_CTRL2_REG, 1 << PMA_PMD_FTX_STATIC_LBN,
    efx.phy_mode & PHY_MODE_TX_DISABLED ||
    efx.phy_mode & PHY_MODE_LOW_POWER ||
    efx.loopback_mode == LOOPBACK_PCS ||
    efx.loopback_mode == LOOPBACK_PMAPMD);
    } else {
// Reset the PHY when moving from tx off to tx on
    if (!(efx.phy_mode & PHY_MODE_TX_DISABLED) &&
    (phy_data.phy_mode & PHY_MODE_TX_DISABLED))
    qt202x_reset_phy(efx);
    ef4_mdio_transmit_disable(efx);
    }
    ef4_mdio_phy_reconfigure(efx);
    phy_data.phy_mode = efx.phy_mode;
    return 0;
    }
    static void qt202x_phy_get_link_ksettings(struct ef4_nic *efx,
    struct ethtool_link_ksettings *cmd)
    {
    mdio45_ethtool_ksettings_get(&efx.mdio, cmd);
    }
#[no_mangle]
unsafe extern "C" fn qt202x_phy_remove(efx: *mut ef4_nic) {
    static void qt202x_phy_remove(struct ef4_nic *efx)
    {
// Free the context block
    kfree(efx.phy_data);
    efx.phy_data = core::ptr::null_mut();
    }
    static int qt202x_phy_get_module_info(struct ef4_nic *efx,
    struct ethtool_modinfo *modinfo)
    {
    modinfo.type = ETH_MODULE_SFF_8079;
    modinfo.eeprom_len = ETH_MODULE_SFF_8079_LEN;
    return 0;
    }
    static int qt202x_phy_get_module_eeprom(struct ef4_nic *efx,
    struct ethtool_eeprom *ee, u8 *data)
    {
    int mmd, reg_base, rc, i;
    if (efx.phy_type == PHY_TYPE_QT2025C) {
    mmd = MDIO_MMD_PCS;
    reg_base = 0xd000;
    } else {
    mmd = MDIO_MMD_PMAPMD;
    reg_base = 0x8007;
    }
    for (i = 0; i < ee.len; i++) {
    rc = ef4_mdio_read(efx, mmd, reg_base + ee.offset + i);
    if (rc < 0)
    return rc;
    data[i] = rc;
    }
    return 0;
    }
    const struct ef4_phy_operations falcon_qt202x_phy_ops = {
    .probe		 = qt202x_phy_probe,
    .init		 = qt202x_phy_init,
    .reconfigure	 = qt202x_phy_reconfigure,
    .poll		 = qt202x_phy_poll,
    .fini		 = ef4_port_dummy_op_void,
    .remove		 = qt202x_phy_remove,
    .get_link_ksettings = qt202x_phy_get_link_ksettings,
    .set_link_ksettings = ef4_mdio_set_link_ksettings,
    .test_alive	 = ef4_mdio_test_alive,
    .get_module_eeprom = qt202x_phy_get_module_eeprom,
    .get_module_info = qt202x_phy_get_module_info,
    };
