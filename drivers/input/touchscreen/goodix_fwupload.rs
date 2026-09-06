//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/goodix_fwupload.c
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
// Goodix Touchscreen firmware upload support
//
// Copyright (c) 2021 Hans de Goede <hdegoede@redhat.com>
//
// This is a rewrite of gt9xx_update.c from the Allwinner H3 BSP which is:
// Copyright (c) 2010 - 2012 Goodix Technology.
// Author: andrew@goodix.com
//

pub const GOODIX_FW_SECTION_LENGTH: c_uint = 0x2000;
pub const GOODIX_FW_DSP_LENGTH: c_uint = 0x1000;
pub const GOODIX_FW_UPLOAD_ADDRESS: c_uint = 0xc000;
pub const GOODIX_CFG_LOC_HAVE_KEY: c_int = 7;
pub const GOODIX_CFG_LOC_DRVA_NUM: c_int = 27;
pub const GOODIX_CFG_LOC_DRVB_NUM: c_int = 28;
pub const GOODIX_CFG_LOC_SENS_NUM: c_int = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_fw_header {
    pub hw_info: [u8; 4],
    pub pid: [u8; 8],
    pub vid: [u8; 2],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn goodix_firmware_checksum(data: *const u8, size: c_int) -> u16 {
    static u16 goodix_firmware_checksum(const u8 *data, int size)
    {
    pub 0: u16 checksum =,
    pub i: c_int,
    pub 2): for (i = 0; i < size; i +=,
    pub 1]: checksum += (data[i] << 8) + data[i +,
    pub checksum: return,
    }
#[no_mangle]
unsafe extern "C" fn goodix_firmware_verify(dev: *mut device, fw: *const firmware) -> c_int {
    static int goodix_firmware_verify(struct device *dev, const struct firmware *fw)
    {
    pub fw_header: *const goodix_fw_header,
    pub expected_size: usize,
    pub data: *const u8,
    pub checksum: u16,
    pub buf: [c_char; 9],
    expected_size = GOODIX_FW_HEADER_LENGTH + 4 * GOODIX_FW_SECTION_LENGTH +
    if (fw.size != expected_size) {
    dev_err(dev, "Firmware has wrong size, expected %zu got %zu\n",
    pub fw->size): expected_size,,
    pub -EINVAL: return,
    }
    pub GOODIX_FW_HEADER_LENGTH: data = fw->data +,
    pub GOODIX_FW_SECTION_LENGTH): *mut *mut checksum = goodix_firmware_checksum(data, 4,
    if (checksum) {
    pub error\n"): dev_err(dev, "Main firmware checksum,
    pub -EINVAL: return,
    }
    pub GOODIX_FW_SECTION_LENGTH: *mut *mut data += 4,
    pub GOODIX_FW_DSP_LENGTH): checksum = goodix_firmware_checksum(data,,
    if (checksum) {
    pub error\n"): dev_err(dev, "DSP firmware checksum,
    pub -EINVAL: return,
    }
    pub )fw->data: *const fw_header = (struct goodix_fw_header,
    dev_info(dev, "Firmware hardware info %02x%02x%02x%02x\n",
    fw_header.hw_info[0], fw_header.hw_info[1],
    pub fw_header->hw_info[3]): fw_header->hw_info[2],,
// pid is a 8 byte buffer containing a string, weird I know
    pub 8): memcpy(buf, fw_header->pid,,
    pub 0: buf[8] =,
    dev_info(dev, "Firmware PID: %s VID: %02x%02x\n", buf,
    pub fw_header->vid[1]): fw_header->vid[0],,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn goodix_enter_upload_mode(client: *mut i2c_client) -> c_int {
    static int goodix_enter_upload_mode(struct i2c_client *client)
    {
    pub error: int tries,,
    pub val: u8,
    pub 200: tries =,
    do {
    error = goodix_i2c_write_u8(client,
    pub 0x0c): GOODIX_REG_MISCTL_SWRST,,
    if (error)
    pub error: return,
    error = goodix_i2c_read(client,
    pub 1): GOODIX_REG_MISCTL_SWRST, &val,,
    if (error)
    pub error: return,
    if (val == 0x0c)
    pub (--tries): } while,
    if (!tries) {
    pub dsp\n"): dev_err(&client->dev, "Error could not hold ss51 &,
    pub -EIO: return,
    }
// DSP_CK and DSP_ALU_CK PowerOn
    pub 0x00): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_DSP_CTL,,
    if (error)
    pub error: return,
// Disable watchdog
    pub 0x00): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_TMR0_EN,,
    if (error)
    pub error: return,
// Clear cache enable
    pub 0x00): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_CACHE_EN,,
    if (error)
    pub error: return,
// Set boot from SRAM
    pub 0x02): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_BOOTCTL,,
    if (error)
    pub error: return,
// Software reboot
    error = goodix_i2c_write_u8(client,
    pub 0x01): GOODIX_REG_MISCTL_CPU_SWRST_PULSE,,
    if (error)
    pub error: return,
// Clear control flag
    pub 0x00): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_BOOTCTL,,
    if (error)
    pub error: return,
// Set scramble
    pub 0x00): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_BOOT_OPT,,
    if (error)
    pub error: return,
// Enable accessing code
    pub 0x01): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_MEM_CD_EN,,
    if (error)
    pub error: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn goodix_start_firmware(client: *mut i2c_client) -> c_int {
    static int goodix_start_firmware(struct i2c_client *client)
    {
    pub error: c_int,
    pub val: u8,
// Init software watchdog
    pub 0xaa): error = goodix_i2c_write_u8(client, GOODIX_REG_SW_WDT,,
    if (error)
    pub error: return,
// Release SS51 & DSP
    pub 0x00): error = goodix_i2c_write_u8(client, GOODIX_REG_MISCTL_SWRST,,
    if (error)
    pub error: return,
    pub 1): error = goodix_i2c_read(client, GOODIX_REG_SW_WDT, &val,,
    if (error)
    pub error: return,
// The value we've written to SW_WDT should have been cleared now
    if (val == 0xaa) {
    pub startup\n"): dev_err(&client->dev, "Error SW_WDT reg not cleared on fw,
    pub -EIO: return,
    }
// Re-init software watchdog
    pub 0xaa): error = goodix_i2c_write_u8(client, GOODIX_REG_SW_WDT,,
    if (error)
    pub error: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn goodix_firmware_upload(ts: *mut goodix_ts_data) -> c_int {
    static int goodix_firmware_upload(struct goodix_ts_data *ts)
    {
    pub fw_name: [c_char; 64],
    pub data: *const u8,
    pub error: c_int,
    pub ts->firmware_name): snprintf(fw_name, sizeof(fw_name), "goodix/%s",,
    pub NULL: *const *const firmware fw __free(firmware) =,
    pub &ts->client->dev): error = request_firmware(&fw, fw_name,,
    if (error) {
    pub error): dev_err(&ts->client->dev, "Firmware request error %d\n",,
    pub error: return,
    }
    pub fw): error = goodix_firmware_verify(&ts->client->dev,,
    if (error)
    pub error: return,
    pub goodix_reset_no_int_sync(ts): error =,
    if (error)
    pub error: return,
    pub goodix_enter_upload_mode(ts->client): error =,
    if (error)
    pub error: return,
// Select SRAM bank 0 and upload section 1 & 2
    error = goodix_i2c_write_u8(ts.client,
    pub 0x00): GOODIX_REG_MISCTL_SRAM_BANK,,
    if (error)
    pub error: return,
    pub GOODIX_FW_HEADER_LENGTH: data = fw->data +,
    error = goodix_i2c_write(ts.client, GOODIX_FW_UPLOAD_ADDRESS,
    pub GOODIX_FW_SECTION_LENGTH): *mut *mut data, 2,
    if (error)
    pub error: return,
// Select SRAM bank 1 and upload section 3 & 4
    error = goodix_i2c_write_u8(ts.client,
    pub 0x01): GOODIX_REG_MISCTL_SRAM_BANK,,
    if (error)
    pub error: return,
    pub GOODIX_FW_SECTION_LENGTH: *mut *mut data += 2,
    error = goodix_i2c_write(ts.client, GOODIX_FW_UPLOAD_ADDRESS,
    pub GOODIX_FW_SECTION_LENGTH): *mut *mut data, 2,
    if (error)
    pub error: return,
// Select SRAM bank 2 and upload the DSP firmware
    error = goodix_i2c_write_u8(ts.client,
    pub 0x02): GOODIX_REG_MISCTL_SRAM_BANK,,
    if (error)
    pub error: return,
    pub GOODIX_FW_SECTION_LENGTH: *mut *mut data += 2,
    error = goodix_i2c_write(ts.client, GOODIX_FW_UPLOAD_ADDRESS,
    pub GOODIX_FW_DSP_LENGTH): data,,
    if (error)
    pub error: return,
    pub goodix_start_firmware(ts->client): error =,
    if (error)
    pub error: return,
    pub goodix_int_sync(ts): error =,
    if (error)
    pub error: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn goodix_prepare_bak_ref(ts: *mut goodix_ts_data) -> c_int {
    static int goodix_prepare_bak_ref(struct goodix_ts_data *ts)
    {
    pub sensor_num: u8 have_key, driver_num,,
    if (ts.bak_ref)
    pub /: *mut *mut return 0; / Already done,
    pub 0x01): have_key = (ts->config[GOODIX_CFG_LOC_HAVE_KEY] &,
    driver_num = (ts.config[GOODIX_CFG_LOC_DRVA_NUM] & 0x1f) +
    pub 0x1f): (ts->config[GOODIX_CFG_LOC_DRVB_NUM] &,
    if (have_key)
    sensor_num = (ts.config[GOODIX_CFG_LOC_SENS_NUM] & 0x0f) +
    pub 0x0f): ((ts->config[GOODIX_CFG_LOC_SENS_NUM] >> 4) &,
    dev_dbg(&ts.client.dev, "Drv %d Sen %d Key %d\n",
    pub have_key): driver_num, sensor_num,,
    pub 2: *mut *mut *mut ts->bak_ref_len = (driver_num  (sensor_num - 2) + 2),
    ts.bak_ref = devm_kzalloc(&ts.client.dev,
    pub GFP_KERNEL): ts->bak_ref_len,,
    if (!ts.bak_ref)
    pub -ENOMEM: return,
//
// The bak_ref array contains the backup of an array of (self/auto)
// calibration related values which the Android version of the driver
// stores on the filesystem so that it can be restored after reboot.
// The mainline kernel never writes directly to the filesystem like
// this, we always start will all the values which give a correction
// factor in approx. the -20 - +20 range (in 2s complement) set to 0.
//
// Note the touchscreen works fine without restoring the reference
// values after a reboot / power-cycle.
//
// The last 2 bytes are a 16 bits unsigned checksum which is expected
// to make the addition al all 16 bit unsigned values in the array add
// up to 1 (rather then the usual 0), so we must set the last byte to 1.
//
    pub 1: ts->bak_ref[ts->bak_ref_len - 1] =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn goodix_send_main_clock(ts: *mut goodix_ts_data) -> c_int {
    static int goodix_send_main_clock(struct goodix_ts_data *ts)
    {
    pub /: *mut *mut u32 main_clk = 54; / Default main clock,
    pub 0: u8 checksum =,
    pub i: c_int,
    device_property_read_u32(&ts.client.dev,
    pub &main_clk): "goodix,main-clk",,
    pub {: for (i = 0; i < (GOODIX_MAIN_CLK_LEN - 1); i++),
    pub main_clk: ts->main_clk[i] =,
    pub main_clk: checksum +=,
    }
// The value of all bytes combines must be 0
    pub checksum: ts->main_clk[GOODIX_MAIN_CLK_LEN - 1] = 256 -,
    return goodix_i2c_write(ts.client, GOODIX_REG_MAIN_CLK,
    pub GOODIX_MAIN_CLK_LEN): ts->main_clk,,
    }
#[no_mangle]
pub unsafe extern "C" fn goodix_firmware_check(ts: *mut goodix_ts_data) -> c_int {
    int goodix_firmware_check(struct goodix_ts_data *ts)
    {
    device_property_read_string(&ts.client.dev,
    pub &ts->firmware_name): "firmware-name",,
    if (!ts.firmware_name)
    pub 0: return,
    if (ts.irq_pin_access_method == IRQ_PIN_ACCESS_NONE) {
    pub fw.\n"): dev_err(&ts->client->dev, "Error no IRQ-pin access method, cannot upload,
    pub -EINVAL: return,
    }
    pub fw-upload\n"): dev_info(&ts->client->dev, "Touchscreen controller needs,
    pub true: ts->load_cfg_from_disk =,
    pub goodix_firmware_upload(ts): return,
    }
#[no_mangle]
pub unsafe extern "C" fn goodix_handle_fw_request(ts: *mut goodix_ts_data) -> bool {
    bool goodix_handle_fw_request(struct goodix_ts_data *ts)
    {
    pub error: c_int,
    pub val: u8,
    pub 1): error = goodix_i2c_read(ts->client, GOODIX_REG_REQUEST, &val,,
    if (error)
    pub false: return,
    switch (val) {
    case GOODIX_RQST_RESPONDED:
//
// If we read back our own last ack the IRQ was not for
// a request.
//
    pub false: return,
    case GOODIX_RQST_CONFIG:
    pub ts->chip->config_len): error = goodix_send_cfg(ts, ts->config,,
    if (error)
    pub false: return,
    case GOODIX_RQST_BAK_REF:
    pub goodix_prepare_bak_ref(ts): error =,
    if (error)
    pub false: return,
    error = goodix_i2c_write(ts.client, GOODIX_REG_BAK_REF,
    pub ts->bak_ref_len): ts->bak_ref,,
    if (error)
    pub false: return,
    case GOODIX_RQST_RESET:
    pub goodix_firmware_upload(ts): error =,
    if (error)
    pub false: return,
    case GOODIX_RQST_MAIN_CLOCK:
    pub goodix_send_main_clock(ts): error =,
    if (error)
    pub false: return,
    case GOODIX_RQST_UNKNOWN:
    case GOODIX_RQST_IDLE:
    default:
    pub val): dev_err_ratelimited(&ts->client->dev, "Unknown Request: 0x%02x\n",,
    }
// Ack the request
    goodix_i2c_write_u8(ts.client,
    pub GOODIX_RQST_RESPONDED): GOODIX_REG_REQUEST,,
    pub true: return,
    }
#[no_mangle]
pub unsafe extern "C" fn goodix_save_bak_ref(ts: *mut goodix_ts_data) {
    void goodix_save_bak_ref(struct goodix_ts_data *ts)
    {
    pub error: c_int,
    pub val: u8,
    if (!ts.firmware_name)
    pub 1): error = goodix_i2c_read(ts->client, GOODIX_REG_STATUS, &val,,
    if (error)
    if (!(val & 0x80))
    error = goodix_i2c_read(ts.client, GOODIX_REG_BAK_REF,
    pub ts->bak_ref_len): ts->bak_ref,,
    if (error) {
    pub ts->bak_ref_len): memset(ts->bak_ref, 0,,
    pub 1: ts->bak_ref[ts->bak_ref_len - 1] =,
    }
    }
