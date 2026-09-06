//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl/hw_atl_utils_fw2x.c
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File hw_atl_utils_fw2x.c: Definition of firmware 2.x functions for
// Atlantic hardware abstraction layer.
//

pub const HW_ATL_FW2X_MPI_LED_ADDR: c_uint = 0x31c;
pub const HW_ATL_FW2X_MPI_RPC_ADDR: c_uint = 0x334;
pub const HW_ATL_FW2X_MPI_MBOX_ADDR: c_uint = 0x360;
pub const HW_ATL_FW2X_MPI_EFUSE_ADDR: c_uint = 0x364;
pub const HW_ATL_FW2X_MPI_CONTROL_ADDR: c_uint = 0x368;
pub const HW_ATL_FW2X_MPI_CONTROL2_ADDR: c_uint = 0x36C;
pub const HW_ATL_FW2X_MPI_STATE_ADDR: c_uint = 0x370;
pub const HW_ATL_FW2X_MPI_STATE2_ADDR: c_uint = 0x374;
pub const HW_ATL_FW3X_EXT_CONTROL_ADDR: c_uint = 0x378;
pub const HW_ATL_FW3X_EXT_STATE_ADDR: c_uint = 0x37c;
pub const HW_ATL_FW3X_PTP_ADJ_LSW_ADDR: c_uint = 0x50a0;
pub const HW_ATL_FW3X_PTP_ADJ_MSW_ADDR: c_uint = 0x50a4;

pub const HAL_ATLANTIC_WOL_FILTERS_COUNT: c_int = 8;
pub const HAL_ATLANTIC_UTILS_FW2X_MSG_WOL: c_uint = 0x0E;
pub const HW_ATL_FW_VER_LED: c_uint = 0x03010026U;
pub const HW_ATL_FW_VER_MEDIA_CONTROL: c_uint = 0x0301005aU;
    struct __packed fw2x_msg_wol_pattern {
    u8 mask[16];
    u32 crc;
    };
    struct __packed fw2x_msg_wol {
    u32 msg_id;
    u8 hw_addr[ETH_ALEN];
    u8 magic_packet_enabled;
    u8 filter_count;
    struct fw2x_msg_wol_pattern filter[HAL_ATLANTIC_WOL_FILTERS_COUNT];
    u8 link_up_enabled;
    u8 link_down_enabled;
    u16 reserved;
    u32 link_up_timeout;
    u32 link_down_timeout;
    };
    static int aq_fw2x_set_link_speed(struct aq_hw_s *self, u32 speed);
    static int aq_fw2x_set_state(struct aq_hw_s *self,
    enum hal_atl_utils_fw_state_e state);
    static u32 aq_fw2x_mbox_get(struct aq_hw_s *self);
    static u32 aq_fw2x_rpc_get(struct aq_hw_s *self);
    static int aq_fw2x_settings_get(struct aq_hw_s *self, u32 *addr);
    static u32 aq_fw2x_state_get(struct aq_hw_s *self);
    static u32 aq_fw2x_state2_get(struct aq_hw_s *self);
#[no_mangle]
unsafe extern "C" fn aq_fw2x_init(self: *mut aq_hw_s) -> c_int {
    static int aq_fw2x_init(struct aq_hw_s *self)
    {
    let mut err: c_int = 0;
// check 10 times by 1ms
    err = readx_poll_timeout_atomic(aq_fw2x_mbox_get,
    self, self.mbox_addr,
    self.mbox_addr != 0U,
    1000U, 10000U);
    err = readx_poll_timeout_atomic(aq_fw2x_rpc_get,
    self, self.rpc_addr,
    self.rpc_addr != 0U,
    1000U, 100000U);
    err = aq_fw2x_settings_get(self, &self.settings_addr);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_deinit(self: *mut aq_hw_s) -> c_int {
    static int aq_fw2x_deinit(struct aq_hw_s *self)
    {
    let mut err: c_int = aq_fw2x_set_link_speed(self, 0);
    if (!err)
    err = aq_fw2x_set_state(self, MPI_DEINIT);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn link_speed_mask_2fw2x_ratemask(speed: u32) -> enum hw_atl_fw2x_rate {
    static enum hw_atl_fw2x_rate link_speed_mask_2fw2x_ratemask(u32 speed)
    {
    let mut rate: enum hw_atl_fw2x_rate = 0;
    if (speed & AQ_NIC_RATE_10G)
    rate |= FW2X_RATE_10G;
    if (speed & AQ_NIC_RATE_5G)
    rate |= FW2X_RATE_5G;
    if (speed & AQ_NIC_RATE_2G5)
    rate |= FW2X_RATE_2G5;
    if (speed & AQ_NIC_RATE_1G)
    rate |= FW2X_RATE_1G;
    if (speed & AQ_NIC_RATE_100M)
    rate |= FW2X_RATE_100M;
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn fw2x_to_eee_mask(speed: u32) -> u32 {
    static u32 fw2x_to_eee_mask(u32 speed)
    {
    let mut rate: u32 = 0;
    if (speed & HW_ATL_FW2X_CAP_EEE_10G_MASK)
    rate |= AQ_NIC_RATE_EEE_10G;
    if (speed & HW_ATL_FW2X_CAP_EEE_5G_MASK)
    rate |= AQ_NIC_RATE_EEE_5G;
    if (speed & HW_ATL_FW2X_CAP_EEE_2G5_MASK)
    rate |= AQ_NIC_RATE_EEE_2G5;
    if (speed & HW_ATL_FW2X_CAP_EEE_1G_MASK)
    rate |= AQ_NIC_RATE_EEE_1G;
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn eee_mask_to_fw2x(speed: u32) -> u32 {
    static u32 eee_mask_to_fw2x(u32 speed)
    {
    let mut rate: u32 = 0;
    if (speed & AQ_NIC_RATE_EEE_10G)
    rate |= HW_ATL_FW2X_CAP_EEE_10G_MASK;
    if (speed & AQ_NIC_RATE_EEE_5G)
    rate |= HW_ATL_FW2X_CAP_EEE_5G_MASK;
    if (speed & AQ_NIC_RATE_EEE_2G5)
    rate |= HW_ATL_FW2X_CAP_EEE_2G5_MASK;
    if (speed & AQ_NIC_RATE_EEE_1G)
    rate |= HW_ATL_FW2X_CAP_EEE_1G_MASK;
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_link_speed(self: *mut aq_hw_s, speed: u32) -> c_int {
    static int aq_fw2x_set_link_speed(struct aq_hw_s *self, u32 speed)
    {
    let mut val: u32 = link_speed_mask_2fw2x_ratemask(speed);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL_ADDR, val);
    return 0;
    }
    static void aq_fw2x_upd_flow_control_bits(struct aq_hw_s *self,
    u32 *mpi_state, u32 fc)
    {
// mpi_state &= ~(HW_ATL_FW2X_CTRL_PAUSE |
    HW_ATL_FW2X_CTRL_ASYMMETRIC_PAUSE);
    switch (fc) {
// There is not explicit mode of RX only pause frames,
// thus, we join this mode with FC full.
// FC full is either Rx, either Tx, or both.
//
    case AQ_NIC_FC_FULL:
    case AQ_NIC_FC_RX:
// mpi_state |= HW_ATL_FW2X_CTRL_PAUSE |
    HW_ATL_FW2X_CTRL_ASYMMETRIC_PAUSE;
    break;
    case AQ_NIC_FC_TX:
// mpi_state |= HW_ATL_FW2X_CTRL_ASYMMETRIC_PAUSE;
    break;
    }
    }
    static void aq_fw2x_upd_eee_rate_bits(struct aq_hw_s *self, u32 *mpi_opts,
    u32 eee_speeds)
    {
// mpi_opts &= ~(HW_ATL_FW2X_CAP_EEE_1G_MASK |
    HW_ATL_FW2X_CAP_EEE_2G5_MASK |
    HW_ATL_FW2X_CAP_EEE_5G_MASK |
    HW_ATL_FW2X_CAP_EEE_10G_MASK);
// mpi_opts |= eee_mask_to_fw2x(eee_speeds);
    }
    static int aq_fw2x_set_state(struct aq_hw_s *self,
    enum hal_atl_utils_fw_state_e state)
    {
    let mut mpi_state: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    struct aq_nic_cfg_s *cfg = self.aq_nic_cfg;
    switch (state) {
    case MPI_INIT:
    mpi_state &= ~BIT(CAPS_HI_LINK_DROP);
    aq_fw2x_upd_eee_rate_bits(self, &mpi_state, cfg.eee_speeds);
    aq_fw2x_upd_flow_control_bits(self, &mpi_state,
    self.aq_nic_cfg.fc.req);
    break;
    case MPI_DEINIT:
    mpi_state |= BIT(CAPS_HI_LINK_DROP);
    break;
    case MPI_RESET:
    case MPI_POWER:
// No actions
    break;
    }
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_update_link_status(self: *mut aq_hw_s) -> c_int {
    static int aq_fw2x_update_link_status(struct aq_hw_s *self)
    {
    struct aq_hw_link_status_s *link_status = &self.aq_link_status;
    u32 mpi_state;
    u32 speed;
    mpi_state = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_STATE_ADDR);
    speed = mpi_state & (FW2X_RATE_100M | FW2X_RATE_1G |
    FW2X_RATE_2G5 | FW2X_RATE_5G |
    FW2X_RATE_10G);
    if (speed) {
    if (speed & FW2X_RATE_10G)
    link_status.mbps = 10000;
#[no_mangle]
pub unsafe extern "C" fn if(FW2X_RATE_5G: speed &) -> else {
    else if (speed & FW2X_RATE_5G)
    link_status.mbps = 5000;
#[no_mangle]
pub unsafe extern "C" fn if(FW2X_RATE_2G5: speed &) -> else {
    else if (speed & FW2X_RATE_2G5)
    link_status.mbps = 2500;
#[no_mangle]
pub unsafe extern "C" fn if(FW2X_RATE_1G: speed &) -> else {
    else if (speed & FW2X_RATE_1G)
    link_status.mbps = 1000;
#[no_mangle]
pub unsafe extern "C" fn if(FW2X_RATE_100M: speed &) -> else {
    else if (speed & FW2X_RATE_100M)
    link_status.mbps = 100;
    else
    link_status.mbps = 10000;
    } else {
    link_status.mbps = 0;
    }
    link_status.full_duplex = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_get_mac_permanent(self: *mut aq_hw_s, mac: *mut u8) -> c_int {
    static int aq_fw2x_get_mac_permanent(struct aq_hw_s *self, u8 *mac)
    {
    let mut efuse_addr: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_EFUSE_ADDR);
    u32 mac_addr[2] = { 0 };
    let mut err: c_int = 0;
    if (efuse_addr != 0) {
    err = hw_atl_utils_fw_downld_dwords(self,
    efuse_addr + (40U * 4U),
    mac_addr,
    ARRAY_SIZE(mac_addr));
    if (err)
    return err;
    mac_addr[0] = __swab32(mac_addr[0]);
    mac_addr[1] = __swab32(mac_addr[1]);
    }
    ether_addr_copy(mac, (u8 *)mac_addr);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_update_stats(self: *mut aq_hw_s) -> c_int {
    static int aq_fw2x_update_stats(struct aq_hw_s *self)
    {
    let mut mpi_opts: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    let mut orig_stats_val: u32 = mpi_opts & BIT(CAPS_HI_STATISTICS);
    u32 stats_val;
    let mut err: c_int = 0;
// Toggle statistics bit for FW to update
    mpi_opts = mpi_opts ^ BIT(CAPS_HI_STATISTICS);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
// Wait FW to report back
    err = readx_poll_timeout_atomic(aq_fw2x_state2_get,
    self, stats_val,
    orig_stats_val != (stats_val &
    BIT(CAPS_HI_STATISTICS)),
    1U, 10000U);
    if (err)
    return err;
    return hw_atl_utils_update_stats(self);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_get_phy_temp(self: *mut aq_hw_s, temp: *mut c_int) -> c_int {
    static int aq_fw2x_get_phy_temp(struct aq_hw_s *self, int *temp)
    {
    let mut mpi_opts: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    let mut temp_val: u32 = mpi_opts & HW_ATL_FW2X_CTRL_TEMPERATURE;
    u32 phy_temp_offset;
    u32 temp_res;
    let mut err: c_int = 0;
    u32 val;
    phy_temp_offset = self.mbox_addr + offsetof(struct hw_atl_utils_mbox,
    info.phy_temperature);
// Toggle statistics bit for FW to 0x36C.18 (CTRL_TEMPERATURE)
    mpi_opts = mpi_opts ^ HW_ATL_FW2X_CTRL_TEMPERATURE;
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
// Wait FW to report back
    err = readx_poll_timeout_atomic(aq_fw2x_state2_get, self, val,
    temp_val !=
    (val & HW_ATL_FW2X_CTRL_TEMPERATURE),
    1U, 10000U);
    err = hw_atl_utils_fw_downld_dwords(self, phy_temp_offset,
    &temp_res, 1);
    if (err)
    return err;
// Convert PHY temperature from 1/256 degree Celsius
// to 1/1000 degree Celsius.
//
// temp = (int16_t)(temp_res & 0xFFFF) * 1000 / 256;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_wol(self: *mut aq_hw_s, mac: *const u8) -> c_int {
    static int aq_fw2x_set_wol(struct aq_hw_s *self, const u8 *mac)
    {
    struct hw_atl_utils_fw_rpc *rpc = core::ptr::null_mut();
    struct offload_info *info = core::ptr::null_mut();
    let mut wol_bits: u32 = 0;
    u32 rpc_size;
    let mut err: c_int = 0;
    u32 val;
    if (self.aq_nic_cfg.wol & WAKE_PHY) {
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR,
    HW_ATL_FW2X_CTRL_LINK_DROP);
    readx_poll_timeout_atomic(aq_fw2x_state2_get, self, val,
    (val &
    HW_ATL_FW2X_CTRL_LINK_DROP) != 0,
    1000, 100000);
    wol_bits |= HW_ATL_FW2X_CTRL_WAKE_ON_LINK;
    }
    if (self.aq_nic_cfg.wol & WAKE_MAGIC) {
    wol_bits |= HW_ATL_FW2X_CTRL_SLEEP_PROXY |
    HW_ATL_FW2X_CTRL_WOL;
    err = hw_atl_utils_fw_rpc_wait(self, &rpc);
    if (err < 0)
    goto err_exit;
    rpc_size = sizeof(*info) +
    offsetof(struct hw_atl_utils_fw_rpc, fw2x_offloads);
    memset(rpc, 0, rpc_size);
    info = &rpc.fw2x_offloads;
    memcpy(info.mac_addr, mac, ETH_ALEN);
    info.len = sizeof(*info);
    err = hw_atl_utils_fw_rpc_call(self, rpc_size);
    if (err < 0)
    goto err_exit;
    }
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, wol_bits);
    err_exit:
    return err;
    }
    static int aq_fw2x_set_power(struct aq_hw_s *self, unsigned int power_state,
    const u8 *mac)
    {
    let mut err: c_int = 0;
    if (self.aq_nic_cfg.wol)
    err = aq_fw2x_set_wol(self, mac);
    return err;
    }
    static int aq_fw2x_send_fw_request(struct aq_hw_s *self,
    const struct hw_fw_request_iface *fw_req,
    size_t size)
    {
    u32 ctrl2, orig_ctrl2;
    u32 dword_cnt;
    let mut err: c_int = 0;
    u32 val;
// Write data to drvIface Mailbox
    dword_cnt = size / sizeof(u32);
    if (size % sizeof(u32))
    dword_cnt++;
    err = hw_atl_write_fwcfg_dwords(self, (void *)fw_req, dword_cnt);
    if (err < 0)
    goto err_exit;
// Toggle statistics bit for FW to update
    ctrl2 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    orig_ctrl2 = ctrl2 & BIT(CAPS_HI_FW_REQUEST);
    ctrl2 = ctrl2 ^ BIT(CAPS_HI_FW_REQUEST);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, ctrl2);
// Wait FW to report back
    err = readx_poll_timeout_atomic(aq_fw2x_state2_get, self, val,
    orig_ctrl2 != (val &
    BIT(CAPS_HI_FW_REQUEST)),
    1U, 10000U);
    err_exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw3x_enable_ptp(self: *mut aq_hw_s, enable: c_int) {
    static void aq_fw3x_enable_ptp(struct aq_hw_s *self, int enable)
    {
    let mut ptp_opts: u32 = aq_hw_read_reg(self, HW_ATL_FW3X_EXT_STATE_ADDR);
    u32 all_ptp_features = BIT(CAPS_EX_PHY_PTP_EN) |
    BIT(CAPS_EX_PTP_GPIO_EN);
    if (enable)
    ptp_opts |= all_ptp_features;
    else
    ptp_opts &= ~all_ptp_features;
    aq_hw_write_reg(self, HW_ATL_FW3X_EXT_CONTROL_ADDR, ptp_opts);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw3x_adjust_ptp(self: *mut aq_hw_s, adj: u64) {
    static void aq_fw3x_adjust_ptp(struct aq_hw_s *self, uint64_t adj)
    {
    aq_hw_write_reg(self, HW_ATL_FW3X_PTP_ADJ_LSW_ADDR,
    (adj >>  0) & 0xffffffff);
    aq_hw_write_reg(self, HW_ATL_FW3X_PTP_ADJ_MSW_ADDR,
    (adj >> 32) & 0xffffffff);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_led_control(self: *mut aq_hw_s, mode: u32) -> c_int {
    static int aq_fw2x_led_control(struct aq_hw_s *self, u32 mode)
    {
    if (self.fw_ver_actual < HW_ATL_FW_VER_LED)
    return -EOPNOTSUPP;
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_LED_ADDR, mode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_eee_rate(self: *mut aq_hw_s, speed: u32) -> c_int {
    static int aq_fw2x_set_eee_rate(struct aq_hw_s *self, u32 speed)
    {
    let mut mpi_opts: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    aq_fw2x_upd_eee_rate_bits(self, &mpi_opts, speed);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
    return 0;
    }
    static int aq_fw2x_get_eee_rate(struct aq_hw_s *self, u32 *rate,
    u32 *supported_rates)
    {
    u32 mpi_state;
    u32 caps_hi;
    let mut err: c_int = 0;
    u32 offset;
    offset = self.mbox_addr + offsetof(struct hw_atl_utils_mbox,
    info.caps_hi);
    err = hw_atl_utils_fw_downld_dwords(self, offset, &caps_hi, 1);
    if (err)
    return err;
// supported_rates = fw2x_to_eee_mask(caps_hi);
    mpi_state = aq_fw2x_state2_get(self);
// rate = fw2x_to_eee_mask(mpi_state);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_renegotiate(self: *mut aq_hw_s) -> c_int {
    static int aq_fw2x_renegotiate(struct aq_hw_s *self)
    {
    let mut mpi_opts: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    mpi_opts |= BIT(CTRL_FORCE_RECONNECT);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_flow_control(self: *mut aq_hw_s) -> c_int {
    static int aq_fw2x_set_flow_control(struct aq_hw_s *self)
    {
    let mut mpi_state: u32 = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    aq_fw2x_upd_flow_control_bits(self, &mpi_state,
    self.aq_nic_cfg.fc.req);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_get_flow_control(self: *mut aq_hw_s, fcmode: *mut u32) -> u32 {
    static u32 aq_fw2x_get_flow_control(struct aq_hw_s *self, u32 *fcmode)
    {
    let mut mpi_state: u32 = aq_fw2x_state2_get(self);
// fcmode = 0;
    if (mpi_state & HW_ATL_FW2X_CAP_PAUSE)
// fcmode |= AQ_NIC_FC_RX;
    if (mpi_state & HW_ATL_FW2X_CAP_ASYM_PAUSE)
// fcmode |= AQ_NIC_FC_TX;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_phyloopback(self: *mut aq_hw_s, mode: u32, enable: bool) -> c_int {
    static int aq_fw2x_set_phyloopback(struct aq_hw_s *self, u32 mode, bool enable)
    {
    u32 mpi_opts;
    switch (mode) {
    case AQ_HW_LOOPBACK_PHYINT_SYS:
    mpi_opts = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    if (enable)
    mpi_opts |= HW_ATL_FW2X_CTRL_INT_LOOPBACK;
    else
    mpi_opts &= ~HW_ATL_FW2X_CTRL_INT_LOOPBACK;
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
    break;
    case AQ_HW_LOOPBACK_PHYEXT_SYS:
    mpi_opts = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    if (enable)
    mpi_opts |= HW_ATL_FW2X_CTRL_EXT_LOOPBACK;
    else
    mpi_opts &= ~HW_ATL_FW2X_CTRL_EXT_LOOPBACK;
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_mbox_get(self: *mut aq_hw_s) -> u32 {
    static u32 aq_fw2x_mbox_get(struct aq_hw_s *self)
    {
    return aq_hw_read_reg(self, HW_ATL_FW2X_MPI_MBOX_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_rpc_get(self: *mut aq_hw_s) -> u32 {
    static u32 aq_fw2x_rpc_get(struct aq_hw_s *self)
    {
    return aq_hw_read_reg(self, HW_ATL_FW2X_MPI_RPC_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_settings_get(self: *mut aq_hw_s, addr: *mut u32) -> c_int {
    static int aq_fw2x_settings_get(struct aq_hw_s *self, u32 *addr)
    {
    let mut err: c_int = 0;
    u32 offset;
    offset = self.mbox_addr + offsetof(struct hw_atl_utils_mbox,
    info.setting_address);
    err = hw_atl_utils_fw_downld_dwords(self, offset, addr, 1);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_state_get(self: *mut aq_hw_s) -> u32 {
    static u32 aq_fw2x_state_get(struct aq_hw_s *self)
    {
    return aq_hw_read_reg(self, HW_ATL_FW2X_MPI_STATE_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_state2_get(self: *mut aq_hw_s) -> u32 {
    static u32 aq_fw2x_state2_get(struct aq_hw_s *self)
    {
    return aq_hw_read_reg(self, HW_ATL_FW2X_MPI_STATE2_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_downshift(self: *mut aq_hw_s, counter: u32) -> c_int {
    static int aq_fw2x_set_downshift(struct aq_hw_s *self, u32 counter)
    {
    let mut err: c_int = 0;
    u32 mpi_opts;
    u32 offset;
    offset = offsetof(struct hw_atl_utils_settings, downshift_retry_count);
    err = hw_atl_write_fwsettings_dwords(self, offset, &counter, 1);
    if (err)
    return err;
    mpi_opts = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR);
    if (counter)
    mpi_opts |= HW_ATL_FW2X_CTRL_DOWNSHIFT;
    else
    mpi_opts &= ~HW_ATL_FW2X_CTRL_DOWNSHIFT;
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL2_ADDR, mpi_opts);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_set_media_detect(self: *mut aq_hw_s, on: bool) -> c_int {
    static int aq_fw2x_set_media_detect(struct aq_hw_s *self, bool on)
    {
    u32 enable;
    u32 offset;
    if (self.fw_ver_actual < HW_ATL_FW_VER_MEDIA_CONTROL)
    return -EOPNOTSUPP;
    offset = offsetof(struct hw_atl_utils_settings, media_detect);
    enable = on;
    return hw_atl_write_fwsettings_dwords(self, offset, &enable, 1);
    }
#[no_mangle]
unsafe extern "C" fn aq_fw2x_get_link_capabilities(self: *mut aq_hw_s) -> u32 {
    static u32 aq_fw2x_get_link_capabilities(struct aq_hw_s *self)
    {
    let mut err: c_int = 0;
    u32 offset;
    u32 val;
    offset = self.mbox_addr +
    offsetof(struct hw_atl_utils_mbox, info.caps_lo);
    err = hw_atl_utils_fw_downld_dwords(self, offset, &val, 1);
    if (err)
    return 0;
    return val;
    }
    static int aq_fw2x_send_macsec_req(struct aq_hw_s *hw,
    struct macsec_msg_fw_request *req,
    struct macsec_msg_fw_response *response)
    {
    u32 low_status, low_req = 0;
    u32 dword_cnt;
    u32 caps_lo;
    u32 offset;
    int err;
    if (!req || !response)
    return -EINVAL;
    caps_lo = aq_fw2x_get_link_capabilities(hw);
    if (!(caps_lo & BIT(CAPS_LO_MACSEC)))
    return -EOPNOTSUPP;
// Write macsec request to cfg memory
    dword_cnt = (sizeof(*req) + sizeof(u32) - 1) / sizeof(u32);
    err = hw_atl_write_fwcfg_dwords(hw, (void *)req, dword_cnt);
    if (err < 0)
    return err;
// Toggle 0x368.CAPS_LO_MACSEC bit
    low_req = aq_hw_read_reg(hw, HW_ATL_FW2X_MPI_CONTROL_ADDR);
    low_req ^= HW_ATL_FW2X_CAP_MACSEC;
    aq_hw_write_reg(hw, HW_ATL_FW2X_MPI_CONTROL_ADDR, low_req);
// Wait FW to report back
    err = readx_poll_timeout_atomic(aq_fw2x_state_get, hw, low_status,
    low_req != (low_status & BIT(CAPS_LO_MACSEC)), 1U, 10000U);
    if (err)
    return -EIO;
// Read status of write operation
    offset = hw.rpc_addr + sizeof(u32);
    err = hw_atl_utils_fw_downld_dwords(hw, offset, (u32 *)(void *)response,
    sizeof(*response) / sizeof(u32));
    return err;
    }
    static int aq_fw2x_read_module_eeprom(struct aq_hw_s *self, u8 dev_addr,
    u8 reg_start_addr, int len, u8 *data)
    {
    u32 low_status, orig_low_status, low_req = 0;
    let mut res_bytes_remain_cnt: u32 = len % sizeof(u32);
    let mut res_dword_cnt: u32 = len / sizeof(u32);
    let mut request: smbus_request = { 0 };
    u32 req_dword_cnt;
    let mut result: u32 = 0;
    u32 caps_lo;
    u32 offset;
    int err;
    caps_lo = aq_fw2x_get_link_capabilities(self);
    if (!(caps_lo & BIT(CAPS_LO_SMBUS_READ)))
    return -EOPNOTSUPP;
    request.msg_id = 0;
    request.device_id = dev_addr;
    request.address = reg_start_addr;
    request.length = len;
// Write SMBUS request to cfg memory
    req_dword_cnt = DIV_ROUND_UP(sizeof(request), sizeof(u32));
    err = hw_atl_write_fwcfg_dwords(self, (void *)&request, req_dword_cnt);
    if (err < 0)
    return err;
// Toggle 0x368.CAPS_LO_SMBUS_READ bit
    low_req = aq_hw_read_reg(self, HW_ATL_FW2X_MPI_CONTROL_ADDR);
    orig_low_status = low_req & BIT(CAPS_LO_SMBUS_READ);
    low_req ^= BIT(CAPS_LO_SMBUS_READ);
    aq_hw_write_reg(self, HW_ATL_FW2X_MPI_CONTROL_ADDR, low_req);
// Wait FW to report back
    err = readx_poll_timeout_atomic(aq_fw2x_state_get, self, low_status,
    orig_low_status != (low_status &
    BIT(CAPS_LO_SMBUS_READ)),
    10U, 100000U);
    if (err)
    return err;
// Read status of read operation
    offset = self.rpc_addr + sizeof(u32);
    err = hw_atl_utils_fw_downld_dwords(self, offset, &result,
    sizeof(result) / sizeof(u32));
    if (err < 0)
    return err;
    if (result)
    return -EIO;
// Read response full DWORD data
    if (res_dword_cnt) {
    offset = self.rpc_addr + sizeof(u32) * 2;
    err = hw_atl_utils_fw_downld_dwords(self, offset, (u32 *)data,
    res_dword_cnt);
    if (err < 0)
    return err;
    }
// Read response trailing bytes data
    if (res_bytes_remain_cnt) {
    let mut bytes_remain_val: u32 = 0;
    offset = self.rpc_addr +
    (sizeof(u32) * 2) +
    (res_dword_cnt * sizeof(u32));
    err = hw_atl_utils_fw_downld_dwords(self, offset,
    &bytes_remain_val, 1);
    if (err < 0)
    return err;
    memcpy(data + len - res_bytes_remain_cnt,
    &bytes_remain_val, res_bytes_remain_cnt);
    }
    return 0;
    }
    const struct aq_fw_ops aq_fw_2x_ops = {
    .init               = aq_fw2x_init,
    .deinit             = aq_fw2x_deinit,
    .reset              = core::ptr::null_mut(),
    .renegotiate        = aq_fw2x_renegotiate,
    .get_mac_permanent  = aq_fw2x_get_mac_permanent,
    .set_link_speed     = aq_fw2x_set_link_speed,
    .set_state          = aq_fw2x_set_state,
    .update_link_status = aq_fw2x_update_link_status,
    .update_stats       = aq_fw2x_update_stats,
    .get_mac_temp       = core::ptr::null_mut(),
    .get_phy_temp       = aq_fw2x_get_phy_temp,
    .set_power          = aq_fw2x_set_power,
    .set_eee_rate       = aq_fw2x_set_eee_rate,
    .get_eee_rate       = aq_fw2x_get_eee_rate,
    .set_flow_control   = aq_fw2x_set_flow_control,
    .get_flow_control   = aq_fw2x_get_flow_control,
    .send_fw_request    = aq_fw2x_send_fw_request,
    .enable_ptp         = aq_fw3x_enable_ptp,
    .led_control        = aq_fw2x_led_control,
    .set_phyloopback    = aq_fw2x_set_phyloopback,
    .set_downshift      = aq_fw2x_set_downshift,
    .set_media_detect   = aq_fw2x_set_media_detect,
    .adjust_ptp         = aq_fw3x_adjust_ptp,
    .get_link_capabilities = aq_fw2x_get_link_capabilities,
    .send_macsec_req    = aq_fw2x_send_macsec_req,
    .read_module_eeprom = aq_fw2x_read_module_eeprom,
    };
