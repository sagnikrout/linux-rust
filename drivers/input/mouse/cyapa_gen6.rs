//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/cyapa_gen6.c
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
// Cypress APA trackpad with I2C interface
//
// Author: Dudley Du <dudl@cypress.com>
//
// Copyright (C) 2015 Cypress Semiconductor, Inc.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

pub const GEN6_ENABLE_CMD_IRQ: c_uint = 0x41;
pub const GEN6_DISABLE_CMD_IRQ: c_uint = 0x42;
pub const GEN6_ENABLE_DEV_IRQ: c_uint = 0x43;
pub const GEN6_DISABLE_DEV_IRQ: c_uint = 0x44;
pub const GEN6_POWER_MODE_ACTIVE: c_uint = 0x01;
pub const GEN6_POWER_MODE_LP_MODE1: c_uint = 0x02;
pub const GEN6_POWER_MODE_LP_MODE2: c_uint = 0x03;
pub const GEN6_POWER_MODE_BTN_ONLY: c_uint = 0x04;
pub const GEN6_SET_POWER_MODE_INTERVAL: c_uint = 0x47;
pub const GEN6_GET_POWER_MODE_INTERVAL: c_uint = 0x48;
pub const GEN6_MAX_RX_NUM: c_int = 14;
pub const GEN6_RETRIEVE_DATA_ID_RX_ATTENURATOR_IDAC: c_uint = 0x00;
pub const GEN6_RETRIEVE_DATA_ID_ATTENURATOR_TRIM: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pip_app_cmd_head {
    pub addr: __le16,
    pub length: __le16,
    pub report_id: u8,
    pub /: *mut *mut u8 resv; / Reserved, must be 0,
    pub code.*/: *mut *mut u8 cmd_code; / bit7: resv, set to 0; bit6~0: command,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pip_app_resp_head {
    pub length: __le16,
    pub report_id: u8,
    pub /: *mut *mut u8 resv; / Reserved, must be 0,
    pub code.*/: *mut *mut u8 cmd_code; / bit7: TGL; bit6~0: command,
//
// The value of data_status can be the first byte of data or
// the command status or the unsupported command code depending on the
// requested command code.
//
    pub data_status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pip_fixed_info {
    pub silicon_id_high: u8,
    pub silicon_id_low: u8,
    pub family_id: u8,
}

    static u8 pip_get_bl_info[] = {
    0x04, 0x00, 0x0B, 0x00, 0x40, 0x00, 0x01, 0x38,
    0x00, 0x00, 0x70, 0x9E, 0x17
    };
    static bool cyapa_sort_pip_hid_descriptor_data(struct cyapa *cyapa,
    u8 *buf, int len)
    {
    if (len != PIP_HID_DESCRIPTOR_SIZE)
    return false;
    if (buf[PIP_RESP_REPORT_ID_OFFSET] == PIP_HID_APP_REPORT_ID ||
    buf[PIP_RESP_REPORT_ID_OFFSET] == PIP_HID_BL_REPORT_ID)
    return true;
    return false;
    }
    static int cyapa_get_pip_fixed_info(struct cyapa *cyapa,
    struct pip_fixed_info *pip_info, bool is_bootloader)
    {
    u8 resp_data[PIP_READ_SYS_INFO_RESP_LENGTH];
    int resp_len;
    u16 product_family;
    int error;
    if (is_bootloader) {
// Read Bootloader Information to determine Gen5 or Gen6.
    resp_len = sizeof(resp_data);
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    pip_get_bl_info, sizeof(pip_get_bl_info),
    resp_data, &resp_len,
    2000, cyapa_sort_tsg_pip_bl_resp_data,
    false);
    if (error || resp_len < PIP_BL_GET_INFO_RESP_LENGTH)
    return error ? error : -EIO;
    pip_info.family_id = resp_data[8];
    pip_info.silicon_id_low = resp_data[10];
    pip_info.silicon_id_high = resp_data[11];
    return 0;
    }
// Get App System Information to determine Gen5 or Gen6.
    resp_len = sizeof(resp_data);
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    pip_read_sys_info, PIP_READ_SYS_INFO_CMD_LENGTH,
    resp_data, &resp_len,
    2000, cyapa_pip_sort_system_info_data, false);
    if (error || resp_len < PIP_READ_SYS_INFO_RESP_LENGTH)
    return error ? error : -EIO;
    product_family = get_unaligned_le16(&resp_data[7]);
    if ((product_family & PIP_PRODUCT_FAMILY_MASK) !=
    PIP_PRODUCT_FAMILY_TRACKPAD)
    return -EINVAL;
    pip_info.family_id = resp_data[19];
    pip_info.silicon_id_low = resp_data[21];
    pip_info.silicon_id_high = resp_data[22];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cyapa_pip_state_parse(cyapa: *mut cyapa, reg_data: *mut u8, len: c_int) -> c_int {
    int cyapa_pip_state_parse(struct cyapa *cyapa, u8 *reg_data, int len)
    {
    u8 cmd[] = { 0x01, 0x00};
    struct pip_fixed_info pip_info;
    u8 resp_data[PIP_HID_DESCRIPTOR_SIZE];
    int resp_len;
    bool is_bootloader;
    int error;
    cyapa.state = CYAPA_STATE_NO_DEVICE;
// Try to wake from it deep sleep state if it is.
    cyapa_pip_deep_sleep(cyapa, PIP_DEEP_SLEEP_STATE_ON);
// Empty the buffer queue to get fresh data with later commands.
    cyapa_empty_pip_output_data(cyapa, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
//
// Read description info from trackpad device to determine running in
// APP mode or Bootloader mode.
//
    resp_len = PIP_HID_DESCRIPTOR_SIZE;
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    cmd, sizeof(cmd),
    resp_data, &resp_len,
    300,
    cyapa_sort_pip_hid_descriptor_data,
    false);
    if (error)
    return error;
    if (resp_data[PIP_RESP_REPORT_ID_OFFSET] == PIP_HID_BL_REPORT_ID)
    is_bootloader = true;
#[no_mangle]
pub unsafe extern "C" fn if(PIP_HID_APP_REPORT_ID: resp_data[PIP_RESP_REPORT_ID_OFFSET] ==) -> else {
    else if (resp_data[PIP_RESP_REPORT_ID_OFFSET] == PIP_HID_APP_REPORT_ID)
    is_bootloader = false;
    else
    return -EAGAIN;
// Get PIP fixed information to determine Gen5 or Gen6.
    memset(&pip_info, 0, sizeof(struct pip_fixed_info));
    error = cyapa_get_pip_fixed_info(cyapa, &pip_info, is_bootloader);
    if (error)
    return error;
    if (pip_info.family_id == 0x9B && pip_info.silicon_id_high == 0x0B) {
    cyapa.gen = CYAPA_GEN6;
    cyapa.state = is_bootloader ? CYAPA_STATE_GEN6_BL
    : CYAPA_STATE_GEN6_APP;
    } else if (pip_info.family_id == 0x91 &&
    pip_info.silicon_id_high == 0x02) {
    cyapa.gen = CYAPA_GEN5;
    cyapa.state = is_bootloader ? CYAPA_STATE_GEN5_BL
    : CYAPA_STATE_GEN5_APP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_read_sys_info(cyapa: *mut cyapa) -> c_int {
    static int cyapa_gen6_read_sys_info(struct cyapa *cyapa)
    {
    u8 resp_data[PIP_READ_SYS_INFO_RESP_LENGTH];
    int resp_len;
    u16 product_family;
    u8 rotat_align;
    int error;
// Get App System Information to determine Gen5 or Gen6.
    resp_len = sizeof(resp_data);
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    pip_read_sys_info, PIP_READ_SYS_INFO_CMD_LENGTH,
    resp_data, &resp_len,
    2000, cyapa_pip_sort_system_info_data, false);
    if (error || resp_len < sizeof(resp_data))
    return error ? error : -EIO;
    product_family = get_unaligned_le16(&resp_data[7]);
    if ((product_family & PIP_PRODUCT_FAMILY_MASK) !=
    PIP_PRODUCT_FAMILY_TRACKPAD)
    return -EINVAL;
    cyapa.platform_ver = (resp_data[67] >> PIP_BL_PLATFORM_VER_SHIFT) &
    PIP_BL_PLATFORM_VER_MASK;
    cyapa.fw_maj_ver = resp_data[9];
    cyapa.fw_min_ver = resp_data[10];
    cyapa.electrodes_x = resp_data[33];
    cyapa.electrodes_y = resp_data[34];
    cyapa.physical_size_x =  get_unaligned_le16(&resp_data[35]) / 100;
    cyapa.physical_size_y = get_unaligned_le16(&resp_data[37]) / 100;
    cyapa.max_abs_x = get_unaligned_le16(&resp_data[39]);
    cyapa.max_abs_y = get_unaligned_le16(&resp_data[41]);
    cyapa.max_z = get_unaligned_le16(&resp_data[43]);
    cyapa.x_origin = resp_data[45] & 0x01;
    cyapa.y_origin = resp_data[46] & 0x01;
    cyapa.btn_capability = (resp_data[70] << 3) & CAPABILITY_BTN_MASK;
    memcpy(&cyapa.product_id[0], &resp_data[51], 5);
    cyapa.product_id[5] = '-';
    memcpy(&cyapa.product_id[6], &resp_data[56], 6);
    cyapa.product_id[12] = '-';
    memcpy(&cyapa.product_id[13], &resp_data[62], 2);
    cyapa.product_id[15] = '\0';
// Get the number of Rx electrodes.
    rotat_align = resp_data[68];
    cyapa.electrodes_rx =
    rotat_align ? cyapa.electrodes_y : cyapa.electrodes_x;
    cyapa.aligned_electrodes_rx = (cyapa.electrodes_rx + 3) & ~3u;
    if (!cyapa.electrodes_x || !cyapa.electrodes_y ||
    !cyapa.physical_size_x || !cyapa.physical_size_y ||
    !cyapa.max_abs_x || !cyapa.max_abs_y || !cyapa.max_z)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_bl_read_app_info(cyapa: *mut cyapa) -> c_int {
    static int cyapa_gen6_bl_read_app_info(struct cyapa *cyapa)
    {
    u8 resp_data[PIP_BL_APP_INFO_RESP_LENGTH];
    int resp_len;
    int error;
    resp_len = sizeof(resp_data);
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    pip_bl_read_app_info, PIP_BL_READ_APP_INFO_CMD_LENGTH,
    resp_data, &resp_len,
    500, cyapa_sort_tsg_pip_bl_resp_data, false);
    if (error || resp_len < PIP_BL_APP_INFO_RESP_LENGTH ||
    !PIP_CMD_COMPLETE_SUCCESS(resp_data))
    return error ? error : -EIO;
    cyapa.fw_maj_ver = resp_data[8];
    cyapa.fw_min_ver = resp_data[9];
    cyapa.platform_ver = (resp_data[12] >> PIP_BL_PLATFORM_VER_SHIFT) &
    PIP_BL_PLATFORM_VER_MASK;
    memcpy(&cyapa.product_id[0], &resp_data[13], 5);
    cyapa.product_id[5] = '-';
    memcpy(&cyapa.product_id[6], &resp_data[18], 6);
    cyapa.product_id[12] = '-';
    memcpy(&cyapa.product_id[13], &resp_data[24], 2);
    cyapa.product_id[15] = '\0';
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_config_dev_irq(cyapa: *mut cyapa, cmd_code: u8) -> c_int {
    static int cyapa_gen6_config_dev_irq(struct cyapa *cyapa, u8 cmd_code)
    {
    u8 cmd[] = { 0x04, 0x00, 0x05, 0x00, 0x2f, 0x00, cmd_code };
    u8 resp_data[6];
    int resp_len;
    int error;
    resp_len = sizeof(resp_data);
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa, cmd, sizeof(cmd),
    resp_data, &resp_len,
    500, cyapa_sort_tsg_pip_app_resp_data, false);
    if (error || !VALID_CMD_RESP_HEADER(resp_data, cmd_code) ||
    !PIP_CMD_COMPLETE_SUCCESS(resp_data)
    )
    return error < 0 ? error : -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_set_proximity(cyapa: *mut cyapa, enable: bool) -> c_int {
    static int cyapa_gen6_set_proximity(struct cyapa *cyapa, bool enable)
    {
    int error;
    cyapa_gen6_config_dev_irq(cyapa, GEN6_DISABLE_CMD_IRQ);
    error = cyapa_pip_set_proximity(cyapa, enable);
    cyapa_gen6_config_dev_irq(cyapa, GEN6_ENABLE_CMD_IRQ);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_change_power_state(cyapa: *mut cyapa, power_mode: u8) -> c_int {
    static int cyapa_gen6_change_power_state(struct cyapa *cyapa, u8 power_mode)
    {
    u8 cmd[] = { 0x04, 0x00, 0x06, 0x00, 0x2f, 0x00, 0x46, power_mode };
    u8 resp_data[6];
    int resp_len;
    int error;
    resp_len = sizeof(resp_data);
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa, cmd, sizeof(cmd),
    resp_data, &resp_len,
    500, cyapa_sort_tsg_pip_app_resp_data, false);
    if (error || !VALID_CMD_RESP_HEADER(resp_data, 0x46))
    return error < 0 ? error : -EINVAL;
// New power state applied in device not match the set power state.
    if (resp_data[5] != power_mode)
    return -EAGAIN;
    return 0;
    }
    static int cyapa_gen6_set_interval_setting(struct cyapa *cyapa,
    struct gen6_interval_setting *interval_setting)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen6_set_interval_cmd {
    pub addr: __le16,
    pub length: __le16,
    pub report_id: u8,
    pub /: *mut *mut u8 rsvd; / Reserved, must be 0,
    pub cmd_code: u8,
    pub active_interval: __le16,
    pub lp1_interval: __le16,
    pub lp2_interval: __le16,
    pub set_interval_cmd: } __packed,
    pub resp_data: [u8; 11],
    pub resp_len: c_int,
    pub error: c_int,
    pub sizeof(set_interval_cmd)): memset(&set_interval_cmd, 0,,
    pub &set_interval_cmd.addr): put_unaligned_le16(PIP_OUTPUT_REPORT_ADDR,,
    put_unaligned_le16(sizeof(set_interval_cmd) - 2,
    pub PIP_APP_CMD_REPORT_ID: set_interval_cmd.report_id =,
    pub GEN6_SET_POWER_MODE_INTERVAL: set_interval_cmd.cmd_code =,
    put_unaligned_le16(interval_setting.active_interval,
    put_unaligned_le16(interval_setting.lp1_interval,
    put_unaligned_le16(interval_setting.lp2_interval,
    pub sizeof(resp_data): resp_len =,
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    (u8 *)&set_interval_cmd, sizeof(set_interval_cmd),
    resp_data, &resp_len,
    pub false): 500, cyapa_sort_tsg_pip_app_resp_data,,
    if (error ||
    !VALID_CMD_RESP_HEADER(resp_data, GEN6_SET_POWER_MODE_INTERVAL))
    pub -EINVAL: return error < 0 ? error :,
// Get the real set intervals from response.
    pub get_unaligned_le16(&resp_data[5]): interval_setting->active_interval =,
    pub get_unaligned_le16(&resp_data[7]): interval_setting->lp1_interval =,
    pub get_unaligned_le16(&resp_data[9]): interval_setting->lp2_interval =,
    pub 0: return,
    }
    static int cyapa_gen6_get_interval_setting(struct cyapa *cyapa,
    struct gen6_interval_setting *interval_setting)
    {
    u8 cmd[] = { 0x04, 0x00, 0x05, 0x00, 0x2f, 0x00,
    pub }: GEN6_GET_POWER_MODE_INTERVAL,
    pub resp_data: [u8; 11],
    pub resp_len: c_int,
    pub error: c_int,
    pub sizeof(resp_data): resp_len =,
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa, cmd, sizeof(cmd),
    resp_data, &resp_len,
    pub false): 500, cyapa_sort_tsg_pip_app_resp_data,,
    if (error ||
    !VALID_CMD_RESP_HEADER(resp_data, GEN6_GET_POWER_MODE_INTERVAL))
    pub -EINVAL: return error < 0 ? error :,
    pub get_unaligned_le16(&resp_data[5]): interval_setting->active_interval =,
    pub get_unaligned_le16(&resp_data[7]): interval_setting->lp1_interval =,
    pub get_unaligned_le16(&resp_data[9]): interval_setting->lp2_interval =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_deep_sleep(cyapa: *mut cyapa, state: u8) -> c_int {
    static int cyapa_gen6_deep_sleep(struct cyapa *cyapa, u8 state)
    {
    pub }: u8 ping[] = { 0x04, 0x00, 0x05, 0x00, 0x2f, 0x00, 0x00,
    if (state == PIP_DEEP_SLEEP_STATE_ON)
//
// Send ping command to notify device prepare for wake up
// when it's in deep sleep mode. At this time, device will
// response nothing except an I2C NAK.
//
    pub sizeof(ping)): cyapa_i2c_pip_write(cyapa, ping,,
    pub state): return cyapa_pip_deep_sleep(cyapa,,
    }
    static int cyapa_gen6_set_power_mode(struct cyapa *cyapa,
    u8 power_mode, u16 sleep_time, enum cyapa_pm_stage pm_stage)
    {
    pub &cyapa->client->dev: *mut *mut device dev =,
    struct gen6_interval_setting *interval_setting =
    pub lp_mode: u8,
    pub error: c_int,
    if (cyapa.state != CYAPA_STATE_GEN6_APP)
    pub 0: return,
    if (PIP_DEV_GET_PWR_STATE(cyapa) == UNINIT_PWR_MODE) {
//
// Assume TP in deep sleep mode when driver is loaded,
// avoid driver unload and reload command IO issue caused by TP
// has been set into deep sleep mode when unloading.
//
    pub PWR_MODE_OFF): PIP_DEV_SET_PWR_STATE(cyapa,,
    }
    if (PIP_DEV_UNINIT_SLEEP_TIME(cyapa) &&
    PIP_DEV_GET_PWR_STATE(cyapa) != PWR_MODE_OFF)
    pub UNINIT_SLEEP_TIME): PIP_DEV_SET_SLEEP_TIME(cyapa,,
    if (PIP_DEV_GET_PWR_STATE(cyapa) == power_mode) {
    if (power_mode == PWR_MODE_OFF ||
    power_mode == PWR_MODE_FULL_ACTIVE ||
    power_mode == PWR_MODE_BTN_ONLY ||
    PIP_DEV_GET_SLEEP_TIME(cyapa) == sleep_time) {
// Has in correct power mode state, early return.
    pub 0: return,
    }
    }
    if (power_mode == PWR_MODE_OFF) {
    pub GEN6_DISABLE_CMD_IRQ): cyapa_gen6_config_dev_irq(cyapa,,
    pub PIP_DEEP_SLEEP_STATE_OFF): error = cyapa_gen6_deep_sleep(cyapa,,
    if (error) {
    pub error): dev_err(dev, "enter deep sleep fail: %d\n",,
    pub error: return,
    }
    pub PWR_MODE_OFF): PIP_DEV_SET_PWR_STATE(cyapa,,
    pub 0: return,
    }
//
// When trackpad in power off mode, it cannot change to other power
// state directly, must be wake up from sleep firstly, then
// continue to do next power sate change.
//
    if (PIP_DEV_GET_PWR_STATE(cyapa) == PWR_MODE_OFF) {
    pub PIP_DEEP_SLEEP_STATE_ON): error = cyapa_gen6_deep_sleep(cyapa,,
    if (error) {
    pub error): dev_err(dev, "deep sleep wake fail: %d\n",,
    pub error: return,
    }
    }
//
// Disable device assert interrupts for command response to avoid
// disturbing system suspending or hibernating process.
//
    pub GEN6_DISABLE_CMD_IRQ): cyapa_gen6_config_dev_irq(cyapa,,
    if (power_mode == PWR_MODE_FULL_ACTIVE) {
    error = cyapa_gen6_change_power_state(cyapa,
    if (error) {
    pub error): dev_err(dev, "change to active fail: %d\n",,
    pub out: goto,
    }
    pub PWR_MODE_FULL_ACTIVE): PIP_DEV_SET_PWR_STATE(cyapa,,
// Sync the interval setting from device.
    pub interval_setting): cyapa_gen6_get_interval_setting(cyapa,,
    } else if (power_mode == PWR_MODE_BTN_ONLY) {
    error = cyapa_gen6_change_power_state(cyapa,
    if (error) {
    pub error): dev_err(dev, "fail to button only mode: %d\n",,
    pub out: goto,
    }
    pub PWR_MODE_BTN_ONLY): PIP_DEV_SET_PWR_STATE(cyapa,,
    } else {
//
// Gen6 internally supports to 2 low power scan interval time,
// so can help to switch power mode quickly.
// such as runtime suspend and system suspend.
//
    if (interval_setting.lp1_interval == sleep_time) {
    pub GEN6_POWER_MODE_LP_MODE1: lp_mode =,
    } else if (interval_setting.lp2_interval == sleep_time) {
    pub GEN6_POWER_MODE_LP_MODE2: lp_mode =,
    } else {
    if (interval_setting.lp1_interval == 0) {
    pub sleep_time: interval_setting->lp1_interval =,
    pub GEN6_POWER_MODE_LP_MODE1: lp_mode =,
    } else {
    pub sleep_time: interval_setting->lp2_interval =,
    pub GEN6_POWER_MODE_LP_MODE2: lp_mode =,
    }
    cyapa_gen6_set_interval_setting(cyapa,
    }
    pub lp_mode): error = cyapa_gen6_change_power_state(cyapa,,
    if (error) {
    dev_err(dev, "set power state to 0x%02x failed: %d\n",
    pub error): lp_mode,,
    pub out: goto,
    }
    pub sleep_time): PIP_DEV_SET_SLEEP_TIME(cyapa,,
    PIP_DEV_SET_PWR_STATE(cyapa,
    }
    out:
    pub GEN6_ENABLE_CMD_IRQ): cyapa_gen6_config_dev_irq(cyapa,,
    pub error: return,
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_initialize(cyapa: *mut cyapa) -> c_int {
    static int cyapa_gen6_initialize(struct cyapa *cyapa)
    {
    pub 0: return,
    }
    static int cyapa_pip_retrieve_data_structure(struct cyapa *cyapa,
    u16 read_offset, u16 read_len, u8 data_id,
    u8 *data, int *data_buf_lens)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct retrieve_data_struct_cmd {
    pub head: pip_app_cmd_head,
    pub read_offset: __le16,
    pub read_length: __le16,
    pub data_id: u8,
    pub cmd: } __packed,
    pub 10]: u8 resp_data[GEN6_MAX_RX_NUM +,
    pub resp_len: c_int,
    pub error: c_int,
    pub sizeof(cmd)): memset(&cmd, 0,,
    pub &cmd.head.addr): put_unaligned_le16(PIP_OUTPUT_REPORT_ADDR,,
    pub &cmd.head.length): put_unaligned_le16(sizeof(cmd) - 2,,
    pub PIP_APP_CMD_REPORT_ID: cmd.head.report_id =,
    pub PIP_RETRIEVE_DATA_STRUCTURE: cmd.head.cmd_code =,
    pub &cmd.read_offset): put_unaligned_le16(read_offset,,
    pub &cmd.read_length): put_unaligned_le16(read_len,,
    pub data_id: cmd.data_id =,
    pub sizeof(resp_data): resp_len =,
    error = cyapa_i2c_pip_cmd_irq_sync(cyapa,
    (u8 *)&cmd, sizeof(cmd),
    resp_data, &resp_len,
    500, cyapa_sort_tsg_pip_app_resp_data,
    if (error || !PIP_CMD_COMPLETE_SUCCESS(resp_data) ||
    resp_data[6] != data_id ||
    !VALID_CMD_RESP_HEADER(resp_data, PIP_RETRIEVE_DATA_STRUCTURE))
    pub -EAGAIN: return (error < 0) ? error :,
    pub get_unaligned_le16(&resp_data[7]): read_len =,
    if (*data_buf_lens < read_len) {
// data_buf_lens = read_len;
    pub -ENOBUFS: return,
    }
    pub read_len): memcpy(data, &resp_data[10],,
// data_buf_lens = read_len;
    pub 0: return,
    }
    static ssize_t cyapa_gen6_show_baseline(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    pub dev_get_drvdata(dev): *mut *mut cyapa cyapa =,
    pub data: [u8; GEN6_MAX_RX_NUM],
    pub data_len: c_int,
    pub 0: int size =,
    pub i: c_int,
    pub error: c_int,
    pub resume_error: c_int,
    if (!cyapa_is_pip_app_mode(cyapa))
    pub -EBUSY: return,
// 1. Suspend Scanning
    pub cyapa_pip_suspend_scanning(cyapa): error =,
    if (error)
    pub error: return,
// 2. IDAC and RX Attenuator Calibration Data (Center Frequency).
    pub sizeof(data): data_len =,
    error = cyapa_pip_retrieve_data_structure(cyapa, 0, data_len,
    GEN6_RETRIEVE_DATA_ID_RX_ATTENURATOR_IDAC,
    pub &data_len): data,,
    if (error)
    pub resume_scanning: goto,
    size = sysfs_emit(buf, "%d %d %d %d %d %d ",
    data[0],  /* RX Attenuator Mutual */
    data[1],  /* IDAC Mutual */
    data[2],  /* RX Attenuator Self RX */
    data[3],  /* IDAC Self RX */
    data[4],  /* RX Attenuator Self TX */
    data[5]   /* IDAC Self TX */
// 3. Read Attenuator Trim.
    pub sizeof(data): data_len =,
    error = cyapa_pip_retrieve_data_structure(cyapa, 0, data_len,
    GEN6_RETRIEVE_DATA_ID_ATTENURATOR_TRIM,
    pub &data_len): data,,
    if (error)
    pub resume_scanning: goto,
// set attenuator trim values.
    pub i++): for (i = 0; i < data_len;,
    pub data[i]): size += sysfs_emit_at(buf, size, "%d ",,
    pub "\n"): size += sysfs_emit_at(buf, size,,
    resume_scanning:
// 4. Resume Scanning
    pub cyapa_pip_resume_scanning(cyapa): resume_error =,
    if (resume_error || error) {
    pub PAGE_SIZE): memset(buf, 0,,
    pub error: return resume_error ? resume_error :,
    }
    pub size: return,
    }
#[no_mangle]
unsafe extern "C" fn cyapa_gen6_operational_check(cyapa: *mut cyapa) -> c_int {
    static int cyapa_gen6_operational_check(struct cyapa *cyapa)
    {
    pub &cyapa->client->dev: *mut *mut device dev =,
    pub error: c_int,
    if (cyapa.gen != CYAPA_GEN6)
    pub -ENODEV: return,
    switch (cyapa.state) {
    case CYAPA_STATE_GEN6_BL:
    pub cyapa_pip_bl_exit(cyapa): error =,
    if (error) {
// Try to update trackpad product information.
    pub out: goto,
    }
    pub CYAPA_STATE_GEN6_APP: cyapa->state =,
    case CYAPA_STATE_GEN6_APP:
//
// If trackpad device in deep sleep mode,
// the app command will fail.
// So always try to reset trackpad device to full active when
// the device state is required.
//
    error = cyapa_gen6_set_power_mode(cyapa,
    pub CYAPA_PM_ACTIVE): PWR_MODE_FULL_ACTIVE, 0,,
    if (error)
    dev_warn(dev, "%s: failed to set power active mode.\n",
// By default, the trackpad proximity function is enabled.
    pub true): error = cyapa_pip_set_proximity(cyapa,,
    if (error)
    dev_warn(dev, "%s: failed to enable proximity.\n",
// Get trackpad product information.
    pub cyapa_gen6_read_sys_info(cyapa): error =,
    if (error)
    pub out: goto,
// Only support product ID starting with CYTRA
    if (memcmp(cyapa.product_id, product_id,
    strlen(product_id)) != 0) {
    dev_err(dev, "%s: unknown product ID (%s)\n",
    pub cyapa->product_id): __func__,,
    pub -EINVAL: error =,
    }
    default:
    pub -EINVAL: error =,
    }
    out:
    pub error: return,
    }
    const struct cyapa_dev_ops cyapa_gen6_ops = {
    .check_fw = cyapa_pip_check_fw,
    .bl_enter = cyapa_pip_bl_enter,
    .bl_initiate = cyapa_pip_bl_initiate,
    .update_fw = cyapa_pip_do_fw_update,
    .bl_activate = cyapa_pip_bl_activate,
    .bl_deactivate = cyapa_pip_bl_deactivate,
    .show_baseline = cyapa_gen6_show_baseline,
    .calibrate_store = cyapa_pip_do_calibrate,
    .initialize = cyapa_gen6_initialize,
    .state_parse = cyapa_pip_state_parse,
    .operational_check = cyapa_gen6_operational_check,
    .irq_handler = cyapa_pip_irq_handler,
    .irq_cmd_handler = cyapa_pip_irq_cmd_handler,
    .sort_empty_output_data = cyapa_empty_pip_output_data,
    .set_power_mode = cyapa_gen6_set_power_mode,
    .set_proximity = cyapa_gen6_set_proximity,
}
