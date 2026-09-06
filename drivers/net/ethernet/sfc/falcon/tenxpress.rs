//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/sfc/falcon/tenxpress.c
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
// Copyright 2007-2011 Solarflare Communications Inc.
//

// We expect these MMDs to be in the package.

    MDIO_DEVS_PCS		| \
    MDIO_DEVS_PHYXS	| \
    MDIO_DEVS_AN)

    (1 << LOOPBACK_PCS) |	\
    (1 << LOOPBACK_PMAPMD) |	\
    (1 << LOOPBACK_PHYXS_WS))
// We complain if we fail to see the link partner as 10G capable this many
// times in a row (must be > 1 as sampling the autoneg. registers is racy)
//

// Extended control register
pub const PMA_PMD_XCONTROL_REG: c_int = 49152;
pub const PMA_PMD_EXT_GMII_EN_LBN: c_int = 1;
pub const PMA_PMD_EXT_GMII_EN_WIDTH: c_int = 1;
pub const PMA_PMD_EXT_CLK_OUT_LBN: c_int = 2;
pub const PMA_PMD_EXT_CLK_OUT_WIDTH: c_int = 1;
pub const PMA_PMD_LNPGA_POWERDOWN_LBN: c_int = 8;
pub const PMA_PMD_LNPGA_POWERDOWN_WIDTH: c_int = 1;
pub const PMA_PMD_EXT_CLK312_WIDTH: c_int = 1;
pub const PMA_PMD_EXT_LPOWER_LBN: c_int = 12;
pub const PMA_PMD_EXT_LPOWER_WIDTH: c_int = 1;
pub const PMA_PMD_EXT_ROBUST_LBN: c_int = 14;
pub const PMA_PMD_EXT_ROBUST_WIDTH: c_int = 1;
pub const PMA_PMD_EXT_SSR_LBN: c_int = 15;
pub const PMA_PMD_EXT_SSR_WIDTH: c_int = 1;
// extended status register
pub const PMA_PMD_XSTATUS_REG: c_int = 49153;
pub const PMA_PMD_XSTAT_MDIX_LBN: c_int = 14;

// LED control register
pub const PMA_PMD_LED_CTRL_REG: c_int = 49159;

// LED function override register
pub const PMA_PMD_LED_OVERR_REG: c_int = 49161;
// Bit positions for different LEDs (there are more but not wired on SFE4001)

// Override settings

pub const PMA_PMD_LED_MASK: c_int = 3;
// All LEDs under hardware control
// Green and Amber under hardware control, Red off

pub const PMA_PMD_SPEED_ENABLE_REG: c_int = 49192;
pub const PMA_PMD_100TX_ADV_LBN: c_int = 1;
pub const PMA_PMD_100TX_ADV_WIDTH: c_int = 1;
pub const PMA_PMD_1000T_ADV_LBN: c_int = 2;
pub const PMA_PMD_1000T_ADV_WIDTH: c_int = 1;
pub const PMA_PMD_10000T_ADV_LBN: c_int = 3;
pub const PMA_PMD_10000T_ADV_WIDTH: c_int = 1;
pub const PMA_PMD_SPEED_LBN: c_int = 4;
pub const PMA_PMD_SPEED_WIDTH: c_int = 4;
// Misc register defines
pub const PCS_CLOCK_CTRL_REG: c_int = 55297;
pub const PLL312_RST_N_LBN: c_int = 2;
pub const PCS_SOFT_RST2_REG: c_int = 55302;
pub const SERDES_RST_N_LBN: c_int = 13;
pub const XGXS_RST_N_LBN: c_int = 12;

pub const CLK312_EN_LBN: c_int = 3;
// PHYXS registers
pub const PHYXS_XCONTROL_REG: c_int = 49152;
pub const PHYXS_RESET_LBN: c_int = 15;
pub const PHYXS_RESET_WIDTH: c_int = 1;

// Boot status register
pub const PCS_BOOT_STATUS_REG: c_int = 53248;
pub const PCS_BOOT_FATAL_ERROR_LBN: c_int = 0;
pub const PCS_BOOT_PROGRESS_LBN: c_int = 1;
pub const PCS_BOOT_PROGRESS_WIDTH: c_int = 2;
pub const PCS_BOOT_PROGRESS_INIT: c_int = 0;
pub const PCS_BOOT_PROGRESS_WAIT_MDIO: c_int = 1;
pub const PCS_BOOT_PROGRESS_CHECKSUM: c_int = 2;
pub const PCS_BOOT_PROGRESS_JUMP: c_int = 3;
pub const PCS_BOOT_DOWNLOAD_WAIT_LBN: c_int = 3;
pub const PCS_BOOT_CODE_STARTED_LBN: c_int = 4;
// 100M/1G PHY registers
pub const GPHY_XCONTROL_REG: c_int = 49152;
pub const GPHY_ISOLATE_LBN: c_int = 10;
pub const GPHY_ISOLATE_WIDTH: c_int = 1;
pub const GPHY_DUPLEX_LBN: c_int = 8;
pub const GPHY_DUPLEX_WIDTH: c_int = 1;
pub const GPHY_LOOPBACK_NEAR_LBN: c_int = 14;
pub const GPHY_LOOPBACK_NEAR_WIDTH: c_int = 1;
pub const C22EXT_STATUS_REG: c_int = 49153;
pub const C22EXT_STATUS_LINK_LBN: c_int = 2;
pub const C22EXT_STATUS_LINK_WIDTH: c_int = 1;
pub const C22EXT_MSTSLV_CTRL: c_int = 49161;
pub const C22EXT_MSTSLV_CTRL_ADV_1000_HD_LBN: c_int = 8;
pub const C22EXT_MSTSLV_CTRL_ADV_1000_FD_LBN: c_int = 9;
pub const C22EXT_MSTSLV_STATUS: c_int = 49162;
pub const C22EXT_MSTSLV_STATUS_LP_1000_HD_LBN: c_int = 10;
pub const C22EXT_MSTSLV_STATUS_LP_1000_FD_LBN: c_int = 11;
// Time to wait between powering down the LNPGA and turning off the power
// rails

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tenxpress_phy_data {
    pub loopback_mode: enum ef4_loopback_mode,
    pub phy_mode: enum ef4_phy_mode,
    pub bad_lp_tries: c_int,
}

#[no_mangle]
unsafe extern "C" fn tenxpress_init(efx: *mut ef4_nic) -> c_int {
    static int tenxpress_init(struct ef4_nic *efx)
    {
// Enable 312.5 MHz clock
    ef4_mdio_write(efx, MDIO_MMD_PCS, PCS_TEST_SELECT_REG,
    1 << CLK312_EN_LBN);
// Set the LEDs up as: Green = Link, Amber = Link/Act, Red = Off
    ef4_mdio_set_flag(efx, MDIO_MMD_PMAPMD, PMA_PMD_LED_CTRL_REG,
    1 << PMA_PMA_LED_ACTIVITY_LBN, true);
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD, PMA_PMD_LED_OVERR_REG,
    SFX7101_PMA_PMD_LED_DEFAULT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tenxpress_phy_probe(efx: *mut ef4_nic) -> c_int {
    static int tenxpress_phy_probe(struct ef4_nic *efx)
    {
    struct tenxpress_phy_data *phy_data;
// Allocate phy private storage
    phy_data = kzalloc_obj(*phy_data);
    if (!phy_data)
    return -ENOMEM;
    efx.phy_data = phy_data;
    phy_data.phy_mode = efx.phy_mode;
    efx.mdio.mmds = TENXPRESS_REQUIRED_DEVS;
    efx.mdio.mode_support = MDIO_SUPPORTS_C45;
    efx.loopback_modes = SFX7101_LOOPBACKS | FALCON_XMAC_LOOPBACKS;
    efx.link_advertising = (ADVERTISED_TP | ADVERTISED_Autoneg |
    ADVERTISED_10000baseT_Full);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tenxpress_phy_init(efx: *mut ef4_nic) -> c_int {
    static int tenxpress_phy_init(struct ef4_nic *efx)
    {
    int rc;
    falcon_board(efx).type.init_phy(efx);
    if (!(efx.phy_mode & PHY_MODE_SPECIAL)) {
    rc = ef4_mdio_wait_reset_mmds(efx, TENXPRESS_REQUIRED_DEVS);
    if (rc < 0)
    return rc;
    rc = ef4_mdio_check_mmds(efx, TENXPRESS_REQUIRED_DEVS);
    if (rc < 0)
    return rc;
    }
    rc = tenxpress_init(efx);
    if (rc < 0)
    return rc;
// Reinitialise flow control settings
    ef4_link_set_wanted_fc(efx, efx.wanted_fc);
    ef4_mdio_an_reconfigure(efx);
    schedule_timeout_uninterruptible(HZ / 5); /* 200ms */
// Let XGXS and SerDes out of reset
    falcon_reset_xaui(efx);
    return 0;
    }
// Perform a "special software reset" on the PHY. The caller is
// responsible for saving and restoring the PHY hardware registers
// properly, and masking/unmasking LASI
#[no_mangle]
unsafe extern "C" fn tenxpress_special_reset(efx: *mut ef4_nic) -> c_int {
    static int tenxpress_special_reset(struct ef4_nic *efx)
    {
    int rc, reg;
// The XGMAC clock is driven from the SFX7101 312MHz clock, so
// a special software reset can glitch the XGMAC sufficiently for stats
// requests to fail.
    falcon_stop_nic_stats(efx);
// Initiate reset
    reg = ef4_mdio_read(efx, MDIO_MMD_PMAPMD, PMA_PMD_XCONTROL_REG);
    reg |= (1 << PMA_PMD_EXT_SSR_LBN);
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD, PMA_PMD_XCONTROL_REG, reg);
    mdelay(200);
// Wait for the blocks to come out of reset
    rc = ef4_mdio_wait_reset_mmds(efx, TENXPRESS_REQUIRED_DEVS);
    if (rc < 0)
    goto out;
// Try and reconfigure the device
    rc = tenxpress_init(efx);
    if (rc < 0)
    goto out;
// Wait for the XGXS state machine to churn
    mdelay(10);
    out:
    falcon_start_nic_stats(efx);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sfx7101_check_bad_lp(efx: *mut ef4_nic, link_ok: bool) {
    static void sfx7101_check_bad_lp(struct ef4_nic *efx, bool link_ok)
    {
    struct tenxpress_phy_data *pd = efx.phy_data;
    bool bad_lp;
    int reg;
    if (link_ok) {
    bad_lp = false;
    } else {
// Check that AN has started but not completed.
    reg = ef4_mdio_read(efx, MDIO_MMD_AN, MDIO_STAT1);
    if (!(reg & MDIO_AN_STAT1_LPABLE))
    return; /* LP status is unknown */
    bad_lp = !(reg & MDIO_AN_STAT1_COMPLETE);
    if (bad_lp)
    pd.bad_lp_tries++;
    }
// Nothing to do if all is well and was previously so.
    if (!pd.bad_lp_tries)
    return;
// Use the RX (red) LED as an error indicator once we've seen AN
// failure several times in a row, and also log a message.
    if (!bad_lp || pd.bad_lp_tries == MAX_BAD_LP_TRIES) {
    reg = ef4_mdio_read(efx, MDIO_MMD_PMAPMD,
    PMA_PMD_LED_OVERR_REG);
    reg &= ~(PMA_PMD_LED_MASK << PMA_PMD_LED_RX_LBN);
    if (!bad_lp) {
    reg |= PMA_PMD_LED_OFF << PMA_PMD_LED_RX_LBN;
    } else {
    reg |= PMA_PMD_LED_FLASH << PMA_PMD_LED_RX_LBN;
    netif_err(efx, link, efx.net_dev,
    "appears to be plugged into a port"
    " that is not 10GBASE-T capable. The PHY"
    " supports 10GBASE-T ONLY, so no link can"
    " be established\n");
    }
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD,
    PMA_PMD_LED_OVERR_REG, reg);
    pd.bad_lp_tries = bad_lp;
    }
    }
#[no_mangle]
unsafe extern "C" fn sfx7101_link_ok(efx: *mut ef4_nic) -> bool {
    static bool sfx7101_link_ok(struct ef4_nic *efx)
    {
    return ef4_mdio_links_ok(efx,
    MDIO_DEVS_PMAPMD |
    MDIO_DEVS_PCS |
    MDIO_DEVS_PHYXS);
    }
#[no_mangle]
unsafe extern "C" fn tenxpress_ext_loopback(efx: *mut ef4_nic) {
    static void tenxpress_ext_loopback(struct ef4_nic *efx)
    {
    ef4_mdio_set_flag(efx, MDIO_MMD_PHYXS, PHYXS_TEST1,
    1 << LOOPBACK_NEAR_LBN,
    efx.loopback_mode == LOOPBACK_PHYXS);
    }
#[no_mangle]
unsafe extern "C" fn tenxpress_low_power(efx: *mut ef4_nic) {
    static void tenxpress_low_power(struct ef4_nic *efx)
    {
    ef4_mdio_set_mmds_lpower(
    efx, !!(efx.phy_mode & PHY_MODE_LOW_POWER),
    TENXPRESS_REQUIRED_DEVS);
    }
#[no_mangle]
unsafe extern "C" fn tenxpress_phy_reconfigure(efx: *mut ef4_nic) -> c_int {
    static int tenxpress_phy_reconfigure(struct ef4_nic *efx)
    {
    struct tenxpress_phy_data *phy_data = efx.phy_data;
    bool phy_mode_change, loop_reset;
    if (efx.phy_mode & (PHY_MODE_OFF | PHY_MODE_SPECIAL)) {
    phy_data.phy_mode = efx.phy_mode;
    return 0;
    }
    phy_mode_change = (efx.phy_mode == PHY_MODE_NORMAL &&
    phy_data.phy_mode != PHY_MODE_NORMAL);
    loop_reset = (LOOPBACK_OUT_OF(phy_data, efx, LOOPBACKS_EXTERNAL(efx)) ||
    LOOPBACK_CHANGED(phy_data, efx, 1 << LOOPBACK_GPHY));
    if (loop_reset || phy_mode_change) {
    tenxpress_special_reset(efx);
    falcon_reset_xaui(efx);
    }
    tenxpress_low_power(efx);
    ef4_mdio_transmit_disable(efx);
    ef4_mdio_phy_reconfigure(efx);
    tenxpress_ext_loopback(efx);
    ef4_mdio_an_reconfigure(efx);
    phy_data.loopback_mode = efx.loopback_mode;
    phy_data.phy_mode = efx.phy_mode;
    return 0;
    }
// Poll for link state changes
#[no_mangle]
unsafe extern "C" fn tenxpress_phy_poll(efx: *mut ef4_nic) -> bool {
    static bool tenxpress_phy_poll(struct ef4_nic *efx)
    {
    let mut old_state: ef4_link_state = efx.link_state;
    efx.link_state.up = sfx7101_link_ok(efx);
    efx.link_state.speed = 10000;
    efx.link_state.fd = true;
    efx.link_state.fc = ef4_mdio_get_pause(efx);
    sfx7101_check_bad_lp(efx, efx.link_state.up);
    return !ef4_link_state_equal(&efx.link_state, &old_state);
    }
#[no_mangle]
unsafe extern "C" fn sfx7101_phy_fini(efx: *mut ef4_nic) {
    static void sfx7101_phy_fini(struct ef4_nic *efx)
    {
    int reg;
// Power down the LNPGA
    reg = (1 << PMA_PMD_LNPGA_POWERDOWN_LBN);
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD, PMA_PMD_XCONTROL_REG, reg);
// Waiting here ensures that the board fini, which can turn
// off the power to the PHY, won't get run until the LNPGA
// powerdown has been given long enough to complete.
    schedule_timeout_uninterruptible(LNPGA_PDOWN_WAIT); /* 200 ms */
    }
#[no_mangle]
unsafe extern "C" fn tenxpress_phy_remove(efx: *mut ef4_nic) {
    static void tenxpress_phy_remove(struct ef4_nic *efx)
    {
    kfree(efx.phy_data);
    efx.phy_data = core::ptr::null_mut();
    }
// Override the RX, TX and link LEDs
#[no_mangle]
pub unsafe extern "C" fn tenxpress_set_id_led(efx: *mut ef4_nic, mode: enum ef4_led_mode) {
    void tenxpress_set_id_led(struct ef4_nic *efx, enum ef4_led_mode mode)
    {
    int reg;
    switch (mode) {
    case EF4_LED_OFF:
    reg = (PMA_PMD_LED_OFF << PMA_PMD_LED_TX_LBN) |
    (PMA_PMD_LED_OFF << PMA_PMD_LED_RX_LBN) |
    (PMA_PMD_LED_OFF << PMA_PMD_LED_LINK_LBN);
    break;
    case EF4_LED_ON:
    reg = (PMA_PMD_LED_ON << PMA_PMD_LED_TX_LBN) |
    (PMA_PMD_LED_ON << PMA_PMD_LED_RX_LBN) |
    (PMA_PMD_LED_ON << PMA_PMD_LED_LINK_LBN);
    break;
    default:
    reg = SFX7101_PMA_PMD_LED_DEFAULT;
    break;
    }
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD, PMA_PMD_LED_OVERR_REG, reg);
    }
    static const char *const sfx7101_test_names[] = {
    "bist"
    };
    static const char *sfx7101_test_name(struct ef4_nic *efx, unsigned int index)
    {
    if (index < ARRAY_SIZE(sfx7101_test_names))
    return sfx7101_test_names[index];
    return core::ptr::null_mut();
    }
    static int
    sfx7101_run_tests(struct ef4_nic *efx, int *results, unsigned flags)
    {
    int rc;
    if (!(flags & ETH_TEST_FL_OFFLINE))
    return 0;
// BIST is automatically run after a special software reset
    rc = tenxpress_special_reset(efx);
    results[0] = rc ? -1 : 1;
    ef4_mdio_an_reconfigure(efx);
    return rc;
    }
    static void
    tenxpress_get_link_ksettings(struct ef4_nic *efx,
    struct ethtool_link_ksettings *cmd)
    {
    let mut adv: u32 = 0, lpa = 0;
    int reg;
    reg = ef4_mdio_read(efx, MDIO_MMD_AN, MDIO_AN_10GBT_CTRL);
    if (reg & MDIO_AN_10GBT_CTRL_ADV10G)
    adv |= ADVERTISED_10000baseT_Full;
    reg = ef4_mdio_read(efx, MDIO_MMD_AN, MDIO_AN_10GBT_STAT);
    if (reg & MDIO_AN_10GBT_STAT_LP10G)
    lpa |= ADVERTISED_10000baseT_Full;
    mdio45_ethtool_ksettings_get_npage(&efx.mdio, cmd, adv, lpa);
// In loopback, the PHY automatically brings up the correct interface,
// but doesn't advertise the correct speed. So override it
    if (LOOPBACK_EXTERNAL(efx))
    cmd.base.speed = SPEED_10000;
    }
    static int
    tenxpress_set_link_ksettings(struct ef4_nic *efx,
    const struct ethtool_link_ksettings *cmd)
    {
    if (!cmd.base.autoneg)
    return -EINVAL;
    return ef4_mdio_set_link_ksettings(efx, cmd);
    }
#[no_mangle]
unsafe extern "C" fn sfx7101_set_npage_adv(efx: *mut ef4_nic, advertising: u32) {
    static void sfx7101_set_npage_adv(struct ef4_nic *efx, u32 advertising)
    {
    ef4_mdio_set_flag(efx, MDIO_MMD_AN, MDIO_AN_10GBT_CTRL,
    MDIO_AN_10GBT_CTRL_ADV10G,
    advertising & ADVERTISED_10000baseT_Full);
    }
    const struct ef4_phy_operations falcon_sfx7101_phy_ops = {
    .probe		  = tenxpress_phy_probe,
    .init             = tenxpress_phy_init,
    .reconfigure      = tenxpress_phy_reconfigure,
    .poll             = tenxpress_phy_poll,
    .fini             = sfx7101_phy_fini,
    .remove		  = tenxpress_phy_remove,
    .get_link_ksettings = tenxpress_get_link_ksettings,
    .set_link_ksettings = tenxpress_set_link_ksettings,
    .set_npage_adv    = sfx7101_set_npage_adv,
    .test_alive	  = ef4_mdio_test_alive,
    .test_name	  = sfx7101_test_name,
    .run_tests	  = sfx7101_run_tests,
    };
