//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/sfc/falcon/txc43128_phy.c
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
// Copyright 2006-2011 Solarflare Communications Inc.
//
// Driver for Transwitch/Mysticom CX4 retimer
// see www.transwitch.com, part is TXC-43128
//

// We expect these MMDs to be in the package

    MDIO_DEVS_PMAPMD |	\
    MDIO_DEVS_PHYXS)

    (1 << LOOPBACK_PMAPMD) |	\
    (1 << LOOPBACK_PHYXS_WS))
//
// Compile-time config
//

// Total length of time we'll wait for the PHY to come out of reset (ms)
pub const TXC_MAX_RESET_TIME: c_int = 500;
// Interval between checks (ms)
pub const TXC_RESET_WAIT: c_int = 10;
// How long to run BIST (us)
pub const TXC_BIST_DURATION: c_int = 50;
//
// Register definitions
//
// Command register
pub const TXC_GLRGS_GLCMD: c_uint = 0xc004;
// Useful bits in command register
// Lane power-down
pub const TXC_GLCMD_L01PD_LBN: c_int = 5;
pub const TXC_GLCMD_L23PD_LBN: c_int = 6;
// Limited SW reset: preserves configuration but
// initiates a logic reset. Self-clearing
pub const TXC_GLCMD_LMTSWRST_LBN: c_int = 14;
// Signal Quality Control
pub const TXC_GLRGS_GSGQLCTL: c_uint = 0xc01a;
// Enable bit
pub const TXC_GSGQLCT_SGQLEN_LBN: c_int = 15;
// Lane selection
pub const TXC_GSGQLCT_LNSL_LBN: c_int = 13;
pub const TXC_GSGQLCT_LNSL_WIDTH: c_int = 2;
// Analog TX control
pub const TXC_ALRGS_ATXCTL: c_uint = 0xc040;
// Lane power-down
pub const TXC_ATXCTL_TXPD3_LBN: c_int = 15;
pub const TXC_ATXCTL_TXPD2_LBN: c_int = 14;
pub const TXC_ATXCTL_TXPD1_LBN: c_int = 13;
pub const TXC_ATXCTL_TXPD0_LBN: c_int = 12;
// Amplitude on lanes 0, 1
pub const TXC_ALRGS_ATXAMP0: c_uint = 0xc041;
// Amplitude on lanes 2, 3
pub const TXC_ALRGS_ATXAMP1: c_uint = 0xc042;
// Bit position of value for lane 0 (or 2)
pub const TXC_ATXAMP_LANE02_LBN: c_int = 3;
// Bit position of value for lane 1 (or 3)
pub const TXC_ATXAMP_LANE13_LBN: c_int = 11;
pub const TXC_ATXAMP_1280_mV: c_int = 0;
pub const TXC_ATXAMP_1200_mV: c_int = 8;
pub const TXC_ATXAMP_1120_mV: c_int = 12;
pub const TXC_ATXAMP_1060_mV: c_int = 14;
pub const TXC_ATXAMP_0820_mV: c_int = 25;
pub const TXC_ATXAMP_0720_mV: c_int = 26;
pub const TXC_ATXAMP_0580_mV: c_int = 27;
pub const TXC_ATXAMP_0440_mV: c_int = 28;

    ((TXC_ATXAMP_0820_mV << TXC_ATXAMP_LANE02_LBN)		\
    | (TXC_ATXAMP_0820_mV << TXC_ATXAMP_LANE13_LBN))
pub const TXC_ATXAMP_DEFAULT: c_uint = 0x6060 /* From databook */;
// Preemphasis on lanes 0, 1
pub const TXC_ALRGS_ATXPRE0: c_uint = 0xc043;
// Preemphasis on lanes 2, 3
pub const TXC_ALRGS_ATXPRE1: c_uint = 0xc044;
pub const TXC_ATXPRE_NONE: c_int = 0;
pub const TXC_ATXPRE_DEFAULT: c_uint = 0x1010 /* From databook */;
pub const TXC_ALRGS_ARXCTL: c_uint = 0xc045;
// Lane power-down
pub const TXC_ARXCTL_RXPD3_LBN: c_int = 15;
pub const TXC_ARXCTL_RXPD2_LBN: c_int = 14;
pub const TXC_ARXCTL_RXPD1_LBN: c_int = 13;
pub const TXC_ARXCTL_RXPD0_LBN: c_int = 12;
// Main control
pub const TXC_MRGS_CTL: c_uint = 0xc340;
// Bits in main control

// GPIO output
pub const TXC_GPIO_OUTPUT: c_uint = 0xc346;
pub const TXC_GPIO_DIR: c_uint = 0xc348;
// Vendor-specific BIST registers
pub const TXC_BIST_CTL: c_uint = 0xc280;
pub const TXC_BIST_TXFRMCNT: c_uint = 0xc281;
pub const TXC_BIST_RX0FRMCNT: c_uint = 0xc282;
pub const TXC_BIST_RX1FRMCNT: c_uint = 0xc283;
pub const TXC_BIST_RX2FRMCNT: c_uint = 0xc284;
pub const TXC_BIST_RX3FRMCNT: c_uint = 0xc285;
pub const TXC_BIST_RX0ERRCNT: c_uint = 0xc286;
pub const TXC_BIST_RX1ERRCNT: c_uint = 0xc287;
pub const TXC_BIST_RX2ERRCNT: c_uint = 0xc288;
pub const TXC_BIST_RX3ERRCNT: c_uint = 0xc289;
// BIST type (controls bit patter in test)
pub const TXC_BIST_CTRL_TYPE_LBN: c_int = 10;

// Set this to 1 for 10 bit and 0 for 8 bit
pub const TXC_BIST_CTRL_B10EN_LBN: c_int = 12;
// Enable BIST (write 0 to disable)
pub const TXC_BIST_CTRL_ENAB_LBN: c_int = 13;
// Stop BIST (self-clears when stop complete)
pub const TXC_BIST_CTRL_STOP_LBN: c_int = 14;
// Start BIST (cleared by writing 1 to STOP)
pub const TXC_BIST_CTRL_STRT_LBN: c_int = 15;
// Mt. Diablo test configuration
pub const TXC_MTDIABLO_CTRL: c_uint = 0xc34f;
pub const TXC_MTDIABLO_CTRL_PMA_LOOP_LBN: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txc43128_data {
    pub bug10934_timer: c_ulong,
    pub phy_mode: enum ef4_phy_mode,
    pub loopback_mode: enum ef4_loopback_mode,
}

// The PHY sometimes needs a reset to bring the link back up.  So long as
// it reports link down, we reset it every 5 seconds.
//

// Perform a reset that doesn't clear configuration changes
    static void txc_reset_logic(struct ef4_nic *efx);
// Set the output value of a gpio
#[no_mangle]
pub unsafe extern "C" fn falcon_txc_set_gpio_val(efx: *mut ef4_nic, pin: c_int, on: c_int) {
    void falcon_txc_set_gpio_val(struct ef4_nic *efx, int pin, int on)
    {
    ef4_mdio_set_flag(efx, MDIO_MMD_PHYXS, TXC_GPIO_OUTPUT, 1 << pin, on);
    }
// Set up the GPIO direction register
#[no_mangle]
pub unsafe extern "C" fn falcon_txc_set_gpio_dir(efx: *mut ef4_nic, pin: c_int, dir: c_int) {
    void falcon_txc_set_gpio_dir(struct ef4_nic *efx, int pin, int dir)
    {
    ef4_mdio_set_flag(efx, MDIO_MMD_PHYXS, TXC_GPIO_DIR, 1 << pin, dir);
    }
// Reset the PMA/PMD MMD. The documentation is explicit that this does a
// global reset (it's less clear what reset of other MMDs does).
#[no_mangle]
unsafe extern "C" fn txc_reset_phy(efx: *mut ef4_nic) -> c_int {
    static int txc_reset_phy(struct ef4_nic *efx)
    {
    int rc = ef4_mdio_reset_mmd(efx, MDIO_MMD_PMAPMD,
    TXC_MAX_RESET_TIME / TXC_RESET_WAIT,
    TXC_RESET_WAIT);
    if (rc < 0)
    goto fail;
// Check that all the MMDs we expect are present and responding.
    rc = ef4_mdio_check_mmds(efx, TXC_REQUIRED_DEVS);
    if (rc < 0)
    goto fail;
    return 0;
    fail:
    netif_err(efx, hw, efx.net_dev, TXCNAME ": reset timed out!\n");
    return rc;
    }
// Run a single BIST on one MMD
#[no_mangle]
unsafe extern "C" fn txc_bist_one(efx: *mut ef4_nic, mmd: c_int, test: c_int) -> c_int {
    static int txc_bist_one(struct ef4_nic *efx, int mmd, int test)
    {
    int ctrl, bctl;
    int lane;
    let mut rc: c_int = 0;
// Set PMA to test into loopback using Mt Diablo reg as per app note
    ctrl = ef4_mdio_read(efx, MDIO_MMD_PCS, TXC_MTDIABLO_CTRL);
    ctrl |= (1 << TXC_MTDIABLO_CTRL_PMA_LOOP_LBN);
    ef4_mdio_write(efx, MDIO_MMD_PCS, TXC_MTDIABLO_CTRL, ctrl);
// The BIST app. note lists these  as 3 distinct steps.
// Set the BIST type
    bctl = (test << TXC_BIST_CTRL_TYPE_LBN);
    ef4_mdio_write(efx, mmd, TXC_BIST_CTL, bctl);
// Set the BSTEN bit in the BIST Control register to enable
    bctl |= (1 << TXC_BIST_CTRL_ENAB_LBN);
    ef4_mdio_write(efx, mmd, TXC_BIST_CTL, bctl);
// Set the BSTRT bit in the BIST Control register
    ef4_mdio_write(efx, mmd, TXC_BIST_CTL,
    bctl | (1 << TXC_BIST_CTRL_STRT_LBN));
// Wait.
    udelay(TXC_BIST_DURATION);
// Set the BSTOP bit in the BIST Control register
    bctl |= (1 << TXC_BIST_CTRL_STOP_LBN);
    ef4_mdio_write(efx, mmd, TXC_BIST_CTL, bctl);
// The STOP bit should go off when things have stopped
    while (bctl & (1 << TXC_BIST_CTRL_STOP_LBN))
    bctl = ef4_mdio_read(efx, mmd, TXC_BIST_CTL);
// Check all the error counts are 0 and all the frame counts are
    non-zero */
    for (lane = 0; lane < 4; lane++) {
    let mut count: c_int = ef4_mdio_read(efx, mmd, TXC_BIST_RX0ERRCNT + lane);
    if (count != 0) {
    netif_err(efx, hw, efx.net_dev, TXCNAME": BIST error. "
    "Lane %d had %d errs\n", lane, count);
    rc = -EIO;
    }
    count = ef4_mdio_read(efx, mmd, TXC_BIST_RX0FRMCNT + lane);
    if (count == 0) {
    netif_err(efx, hw, efx.net_dev, TXCNAME": BIST error. "
    "Lane %d got 0 frames\n", lane);
    rc = -EIO;
    }
    }
    if (rc == 0)
    netif_info(efx, hw, efx.net_dev, TXCNAME": BIST pass\n");
// Disable BIST
    ef4_mdio_write(efx, mmd, TXC_BIST_CTL, 0);
// Turn off loopback
    ctrl &= ~(1 << TXC_MTDIABLO_CTRL_PMA_LOOP_LBN);
    ef4_mdio_write(efx, MDIO_MMD_PCS, TXC_MTDIABLO_CTRL, ctrl);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn txc_bist(efx: *mut ef4_nic) -> c_int {
    static int txc_bist(struct ef4_nic *efx)
    {
    return txc_bist_one(efx, MDIO_MMD_PCS, TXC_BIST_CTRL_TYPE_TSD);
    }
// Push the non-configurable defaults into the PHY. This must be
// done after every full reset
#[no_mangle]
unsafe extern "C" fn txc_apply_defaults(efx: *mut ef4_nic) {
    static void txc_apply_defaults(struct ef4_nic *efx)
    {
    int mctrl;
// Turn amplitude down and preemphasis off on the host side
// (PHY<->MAC) as this is believed less likely to upset Falcon
// and no adverse effects have been noted. It probably also
// saves a picowatt or two
// Turn off preemphasis
    ef4_mdio_write(efx, MDIO_MMD_PHYXS, TXC_ALRGS_ATXPRE0, TXC_ATXPRE_NONE);
    ef4_mdio_write(efx, MDIO_MMD_PHYXS, TXC_ALRGS_ATXPRE1, TXC_ATXPRE_NONE);
// Turn down the amplitude
    ef4_mdio_write(efx, MDIO_MMD_PHYXS,
    TXC_ALRGS_ATXAMP0, TXC_ATXAMP_0820_BOTH);
    ef4_mdio_write(efx, MDIO_MMD_PHYXS,
    TXC_ALRGS_ATXAMP1, TXC_ATXAMP_0820_BOTH);
// Set the line side amplitude and preemphasis to the databook
// defaults as an erratum causes them to be 0 on at least some
// PHY rev.s
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD,
    TXC_ALRGS_ATXPRE0, TXC_ATXPRE_DEFAULT);
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD,
    TXC_ALRGS_ATXPRE1, TXC_ATXPRE_DEFAULT);
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD,
    TXC_ALRGS_ATXAMP0, TXC_ATXAMP_DEFAULT);
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD,
    TXC_ALRGS_ATXAMP1, TXC_ATXAMP_DEFAULT);
// Set up the LEDs
    mctrl = ef4_mdio_read(efx, MDIO_MMD_PHYXS, TXC_MRGS_CTL);
// Set the Green and Red LEDs to their default modes
    mctrl &= ~((1 << TXC_MCTL_TXLED_LBN) | (1 << TXC_MCTL_RXLED_LBN));
    ef4_mdio_write(efx, MDIO_MMD_PHYXS, TXC_MRGS_CTL, mctrl);
// Databook recommends doing this after configuration changes
    txc_reset_logic(efx);
    falcon_board(efx).type.init_phy(efx);
    }
#[no_mangle]
unsafe extern "C" fn txc43128_phy_probe(efx: *mut ef4_nic) -> c_int {
    static int txc43128_phy_probe(struct ef4_nic *efx)
    {
    struct txc43128_data *phy_data;
// Allocate phy private storage
    phy_data = kzalloc_obj(*phy_data);
    if (!phy_data)
    return -ENOMEM;
    efx.phy_data = phy_data;
    phy_data.phy_mode = efx.phy_mode;
    efx.mdio.mmds = TXC_REQUIRED_DEVS;
    efx.mdio.mode_support = MDIO_SUPPORTS_C45 | MDIO_EMULATE_C22;
    efx.loopback_modes = TXC_LOOPBACKS | FALCON_XMAC_LOOPBACKS;
    return 0;
    }
// Initialisation entry point for this PHY driver
#[no_mangle]
unsafe extern "C" fn txc43128_phy_init(efx: *mut ef4_nic) -> c_int {
    static int txc43128_phy_init(struct ef4_nic *efx)
    {
    int rc;
    rc = txc_reset_phy(efx);
    if (rc < 0)
    return rc;
    rc = txc_bist(efx);
    if (rc < 0)
    return rc;
    txc_apply_defaults(efx);
    return 0;
    }
// Set the lane power down state in the global registers
#[no_mangle]
unsafe extern "C" fn txc_glrgs_lane_power(efx: *mut ef4_nic, mmd: c_int) {
    static void txc_glrgs_lane_power(struct ef4_nic *efx, int mmd)
    {
    let mut pd: c_int = (1 << TXC_GLCMD_L01PD_LBN) | (1 << TXC_GLCMD_L23PD_LBN);
    let mut ctl: c_int = ef4_mdio_read(efx, mmd, TXC_GLRGS_GLCMD);
    if (!(efx.phy_mode & PHY_MODE_LOW_POWER))
    ctl &= ~pd;
    else
    ctl |= pd;
    ef4_mdio_write(efx, mmd, TXC_GLRGS_GLCMD, ctl);
    }
// Set the lane power down state in the analog control registers
#[no_mangle]
unsafe extern "C" fn txc_analog_lane_power(efx: *mut ef4_nic, mmd: c_int) {
    static void txc_analog_lane_power(struct ef4_nic *efx, int mmd)
    {
    int txpd = (1 << TXC_ATXCTL_TXPD3_LBN) | (1 << TXC_ATXCTL_TXPD2_LBN)
    | (1 << TXC_ATXCTL_TXPD1_LBN) | (1 << TXC_ATXCTL_TXPD0_LBN);
    int rxpd = (1 << TXC_ARXCTL_RXPD3_LBN) | (1 << TXC_ARXCTL_RXPD2_LBN)
    | (1 << TXC_ARXCTL_RXPD1_LBN) | (1 << TXC_ARXCTL_RXPD0_LBN);
    let mut txctl: c_int = ef4_mdio_read(efx, mmd, TXC_ALRGS_ATXCTL);
    let mut rxctl: c_int = ef4_mdio_read(efx, mmd, TXC_ALRGS_ARXCTL);
    if (!(efx.phy_mode & PHY_MODE_LOW_POWER)) {
    txctl &= ~txpd;
    rxctl &= ~rxpd;
    } else {
    txctl |= txpd;
    rxctl |= rxpd;
    }
    ef4_mdio_write(efx, mmd, TXC_ALRGS_ATXCTL, txctl);
    ef4_mdio_write(efx, mmd, TXC_ALRGS_ARXCTL, rxctl);
    }
#[no_mangle]
unsafe extern "C" fn txc_set_power(efx: *mut ef4_nic) {
    static void txc_set_power(struct ef4_nic *efx)
    {
// According to the data book, all the MMDs can do low power
    ef4_mdio_set_mmds_lpower(efx,
    !!(efx.phy_mode & PHY_MODE_LOW_POWER),
    TXC_REQUIRED_DEVS);
// Global register bank is in PCS, PHY XS. These control the host
// side and line side settings respectively.
    txc_glrgs_lane_power(efx, MDIO_MMD_PCS);
    txc_glrgs_lane_power(efx, MDIO_MMD_PHYXS);
// Analog register bank in PMA/PMD, PHY XS
    txc_analog_lane_power(efx, MDIO_MMD_PMAPMD);
    txc_analog_lane_power(efx, MDIO_MMD_PHYXS);
    }
#[no_mangle]
unsafe extern "C" fn txc_reset_logic_mmd(efx: *mut ef4_nic, mmd: c_int) {
    static void txc_reset_logic_mmd(struct ef4_nic *efx, int mmd)
    {
    let mut val: c_int = ef4_mdio_read(efx, mmd, TXC_GLRGS_GLCMD);
    let mut tries: c_int = 50;
    val |= (1 << TXC_GLCMD_LMTSWRST_LBN);
    ef4_mdio_write(efx, mmd, TXC_GLRGS_GLCMD, val);
    while (--tries) {
    val = ef4_mdio_read(efx, mmd, TXC_GLRGS_GLCMD);
    if (!(val & (1 << TXC_GLCMD_LMTSWRST_LBN)))
    break;
    udelay(1);
    }
    if (!tries)
    netif_info(efx, hw, efx.net_dev,
    TXCNAME " Logic reset timed out!\n");
    }
// Perform a logic reset. This preserves the configuration registers
// and is needed for some configuration changes to take effect
#[no_mangle]
unsafe extern "C" fn txc_reset_logic(efx: *mut ef4_nic) {
    static void txc_reset_logic(struct ef4_nic *efx)
    {
// The data sheet claims we can do the logic reset on either the
// PCS or the PHYXS and the result is a reset of both host- and
// line-side logic.
    txc_reset_logic_mmd(efx, MDIO_MMD_PCS);
    }
#[no_mangle]
unsafe extern "C" fn txc43128_phy_read_link(efx: *mut ef4_nic) -> bool {
    static bool txc43128_phy_read_link(struct ef4_nic *efx)
    {
    return ef4_mdio_links_ok(efx, TXC_REQUIRED_DEVS);
    }
#[no_mangle]
unsafe extern "C" fn txc43128_phy_reconfigure(efx: *mut ef4_nic) -> c_int {
    static int txc43128_phy_reconfigure(struct ef4_nic *efx)
    {
    struct txc43128_data *phy_data = efx.phy_data;
    let mut mode_change: enum ef4_phy_mode = efx.phy_mode ^ phy_data.phy_mode;
    let mut loop_change: bool = LOOPBACK_CHANGED(phy_data, efx, TXC_LOOPBACKS);
    if (efx.phy_mode & mode_change & PHY_MODE_TX_DISABLED) {
    txc_reset_phy(efx);
    txc_apply_defaults(efx);
    falcon_reset_xaui(efx);
    mode_change &= ~PHY_MODE_TX_DISABLED;
    }
    ef4_mdio_transmit_disable(efx);
    ef4_mdio_phy_reconfigure(efx);
    if (mode_change & PHY_MODE_LOW_POWER)
    txc_set_power(efx);
// The data sheet claims this is required after every reconfiguration
// (note at end of 7.1), but we mustn't do it when nothing changes as
// it glitches the link, and reconfigure gets called on link change,
// so we get an IRQ storm on link up.
    if (loop_change || mode_change)
    txc_reset_logic(efx);
    phy_data.phy_mode = efx.phy_mode;
    phy_data.loopback_mode = efx.loopback_mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn txc43128_phy_fini(efx: *mut ef4_nic) {
    static void txc43128_phy_fini(struct ef4_nic *efx)
    {
// Disable link events
    ef4_mdio_write(efx, MDIO_MMD_PMAPMD, MDIO_PMA_LASI_CTRL, 0);
    }
#[no_mangle]
unsafe extern "C" fn txc43128_phy_remove(efx: *mut ef4_nic) {
    static void txc43128_phy_remove(struct ef4_nic *efx)
    {
    kfree(efx.phy_data);
    efx.phy_data = core::ptr::null_mut();
    }
// Periodic callback: this exists mainly to poll link status as we
// don't use LASI interrupts
#[no_mangle]
unsafe extern "C" fn txc43128_phy_poll(efx: *mut ef4_nic) -> bool {
    static bool txc43128_phy_poll(struct ef4_nic *efx)
    {
    struct txc43128_data *data = efx.phy_data;
    let mut was_up: bool = efx.link_state.up;
    efx.link_state.up = txc43128_phy_read_link(efx);
    efx.link_state.speed = 10000;
    efx.link_state.fd = true;
    efx.link_state.fc = efx.wanted_fc;
    if (efx.link_state.up || (efx.loopback_mode != LOOPBACK_NONE)) {
    data.bug10934_timer = jiffies;
    } else {
    if (time_after_eq(jiffies, (data.bug10934_timer +
    BUG10934_RESET_INTERVAL))) {
    data.bug10934_timer = jiffies;
    txc_reset_logic(efx);
    }
    }
    return efx.link_state.up != was_up;
    }
    static const char *const txc43128_test_names[] = {
    "bist"
    };
    static const char *txc43128_test_name(struct ef4_nic *efx, unsigned int index)
    {
    if (index < ARRAY_SIZE(txc43128_test_names))
    return txc43128_test_names[index];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn txc43128_run_tests(efx: *mut ef4_nic, results: *mut c_int, flags: unsigned) -> c_int {
    static int txc43128_run_tests(struct ef4_nic *efx, int *results, unsigned flags)
    {
    int rc;
    if (!(flags & ETH_TEST_FL_OFFLINE))
    return 0;
    rc = txc_reset_phy(efx);
    if (rc < 0)
    return rc;
    rc = txc_bist(efx);
    txc_apply_defaults(efx);
    results[0] = rc ? -1 : 1;
    return rc;
    }
    static void txc43128_get_link_ksettings(struct ef4_nic *efx,
    struct ethtool_link_ksettings *cmd)
    {
    mdio45_ethtool_ksettings_get(&efx.mdio, cmd);
    }
    const struct ef4_phy_operations falcon_txc_phy_ops = {
    .probe		= txc43128_phy_probe,
    .init		= txc43128_phy_init,
    .reconfigure	= txc43128_phy_reconfigure,
    .poll		= txc43128_phy_poll,
    .fini		= txc43128_phy_fini,
    .remove		= txc43128_phy_remove,
    .get_link_ksettings = txc43128_get_link_ksettings,
    .set_link_ksettings = ef4_mdio_set_link_ksettings,
    .test_alive	= ef4_mdio_test_alive,
    .run_tests	= txc43128_run_tests,
    .test_name	= txc43128_test_name,
    };
