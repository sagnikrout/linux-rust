//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/sht15.c
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
// sht15.c - support for the SHT15 Temperature and Humidity Sensor
//
// Portions Copyright (c) 2010-2012 Savoir-faire Linux Inc.
// Jerome Oufella <jerome.oufella@savoirfairelinux.com>
// Vivien Didelot <vivien.didelot@savoirfairelinux.com>
//
// Copyright (c) 2009 Jonathan Cameron
//
// Copyright (c) 2007 Wouter Horre
//
// For further information, see the Documentation/hwmon/sht15.rst file.
//

// Commands
pub const SHT15_MEASURE_TEMP: c_uint = 0x03;
pub const SHT15_MEASURE_RH: c_uint = 0x05;
pub const SHT15_WRITE_STATUS: c_uint = 0x06;
pub const SHT15_READ_STATUS: c_uint = 0x07;
pub const SHT15_SOFT_RESET: c_uint = 0x1E;
// Min timings

// Status Register Bits
pub const SHT15_STATUS_LOW_RESOLUTION: c_uint = 0x01;
pub const SHT15_STATUS_NO_OTP_RELOAD: c_uint = 0x02;
pub const SHT15_STATUS_HEATER: c_uint = 0x04;
pub const SHT15_STATUS_LOW_BATTERY: c_uint = 0x40;
// List of supported chips
    enum sht15_chips { sht10, sht11, sht15, sht71, sht75 };
// Actions the driver may be doing
    enum sht15_state {
    SHT15_READING_NOTHING,
    SHT15_READING_TEMP,
    SHT15_READING_HUMID
    };
//
// struct sht15_temppair - elements of voltage dependent temp calc
// @vdd:	supply voltage in microvolts
// @d1:		see data sheet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sht15_temppair {
    pub /: *mut *mut int vdd; / microvolts,
    pub d1: c_int,
}

// Table 9 from datasheet - relates temperature calculation to supply voltage
    static const struct sht15_temppair temppoints[] = {
    { 2500000, -39400 },
    { 3000000, -39600 },
    { 3500000, -39700 },
    { 4000000, -39800 },
    { 5000000, -40100 },
    };
// Table from CRC datasheet, section 2.4
    static const u8 sht15_crc8_table[] = {
    0,	49,	98,	83,	196,	245,	166,	151,
    185,	136,	219,	234,	125,	76,	31,	46,
    67,	114,	33,	16,	135,	182,	229,	212,
    250,	203,	152,	169,	62,	15,	92,	109,
    134,	183,	228,	213,	66,	115,	32,	17,
    63,	14,	93,	108,	251,	202,	153,	168,
    197,	244,	167,	150,	1,	48,	99,	82,
    124,	77,	30,	47,	184,	137,	218,	235,
    61,	12,	95,	110,	249,	200,	155,	170,
    132,	181,	230,	215,	64,	113,	34,	19,
    126,	79,	28,	45,	186,	139,	216,	233,
    199,	246,	165,	148,	3,	50,	97,	80,
    187,	138,	217,	232,	127,	78,	29,	44,
    2,	51,	96,	81,	198,	247,	164,	149,
    248,	201,	154,	171,	60,	13,	94,	111,
    65,	112,	35,	18,	133,	180,	231,	214,
    122,	75,	24,	41,	190,	143,	220,	237,
    195,	242,	161,	144,	7,	54,	101,	84,
    57,	8,	91,	106,	253,	204,	159,	174,
    128,	177,	226,	211,	68,	117,	38,	23,
    252,	205,	158,	175,	56,	9,	90,	107,
    69,	116,	39,	22,	129,	176,	227,	210,
    191,	142,	221,	236,	123,	74,	25,	40,
    6,	55,	100,	85,	194,	243,	160,	145,
    71,	118,	37,	20,	131,	178,	225,	208,
    254,	207,	156,	173,	58,	11,	88,	105,
    4,	53,	102,	87,	192,	241,	162,	147,
    189,	140,	223,	238,	121,	72,	27,	42,
    193,	240,	163,	146,	5,	52,	103,	86,
    120,	73,	26,	43,	188,	141,	222,	239,
    130,	179,	224,	209,	70,	119,	36,	21,
    59,	10,	89,	104,	255,	206,	157,	172
    };
//
// struct sht15_data - device instance specific data
// @sck:		clock GPIO line
// @data:		data GPIO line
// @read_work:		bh of interrupt handler.
// @wait_queue:		wait queue for getting values from device.
// @val_temp:		last temperature value read from device.
// @val_humid:		last humidity value read from device.
// @val_status:		last status register value read from device.
// @checksum_ok:	last value read from the device passed CRC validation.
// @checksumming:	flag used to enable the data validation with CRC.
// @state:		state identifying the action the driver is doing.
// @measurements_valid:	are the current stored measures valid (start condition).
// @status_valid:	is the current stored status valid (start condition).
// @last_measurement:	time of last measure.
// @last_status:	time of last status reading.
// @read_lock:		mutex to ensure only one read in progress at a time.
// @dev:		associate device structure.
// @hwmon_dev:		device associated with hwmon subsystem.
// @reg:		associated regulator (if specified).
// @nb:			notifier block to handle notifications of voltage
// changes.
// @supply_uv:		local copy of supply voltage used to allow use of
// regulator consumer if available.
// @supply_uv_valid:	indicates that an updated value has not yet been
// obtained from the regulator and so any calculations
// based upon it will be invalid.
// @update_supply_work:	work struct that is used to update the supply_uv.
// @interrupt_handled:	flag used to indicate a handler has been scheduled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sht15_data {
    pub sck: *mut gpio_desc,
    pub data: *mut gpio_desc,
    pub read_work: work_struct,
    pub wait_queue: wait_queue_head_t,
    pub val_temp: u16,
    pub val_humid: u16,
    pub val_status: u8,
    pub checksum_ok: bool,
    pub checksumming: bool,
    pub state: enum sht15_state,
    pub measurements_valid: bool,
    pub status_valid: bool,
    pub last_measurement: c_ulong,
    pub last_status: c_ulong,
    pub read_lock: mutex,
    pub dev: *mut device,
    pub hwmon_dev: *mut device,
    pub reg: *mut regulator,
    pub nb: notifier_block,
    pub supply_uv: c_int,
    pub supply_uv_valid: bool,
    pub update_supply_work: work_struct,
    pub interrupt_handled: core::sync::atomic::AtomicI32,
}

//
// sht15_crc8() - compute crc8
// @data:	sht15 specific data.
// @value:	sht15 retrieved data.
// @len:	Length of retrieved data
//
// This implements section 2 of the CRC datasheet.
//
    static u8 sht15_crc8(struct sht15_data *data,
    const u8 *value,
    int len)
    {
    let mut crc: u8 = bitrev8(data.val_status & 0x0F);
    while (len--) {
    crc = sht15_crc8_table[*value ^ crc];
    value++;
    }
    return crc;
    }
//
// sht15_connection_reset() - reset the comms interface
// @data:	sht15 specific data
//
// This implements section 3.4 of the data sheet
//
#[no_mangle]
unsafe extern "C" fn sht15_connection_reset(data: *mut sht15_data) -> c_int {
    static int sht15_connection_reset(struct sht15_data *data)
    {
    int i, err;
    err = gpiod_direction_output(data.data, 1);
    if (err)
    return err;
    ndelay(SHT15_TSCKL);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    for (i = 0; i < 9; ++i) {
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    }
    return 0;
    }
//
// sht15_send_bit() - send an individual bit to the device
// @data:	device state data
// @val:	value of bit to be sent
//
#[no_mangle]
pub unsafe extern "C" fn sht15_send_bit(data: *mut sht15_data, val: c_int) {
    static inline void sht15_send_bit(struct sht15_data *data, int val)
    {
    gpiod_set_value(data.data, val);
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL); /* clock low time */
    }
//
// sht15_transmission_start() - specific sequence for new transmission
// @data:	device state data
//
// Timings for this are not documented on the data sheet, so very
// conservative ones used in implementation. This implements
// figure 12 on the data sheet.
//
#[no_mangle]
unsafe extern "C" fn sht15_transmission_start(data: *mut sht15_data) -> c_int {
    static int sht15_transmission_start(struct sht15_data *data)
    {
    int err;
// ensure data is high and output
    err = gpiod_direction_output(data.data, 1);
    if (err)
    return err;
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    gpiod_set_value(data.data, 0);
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    gpiod_set_value(data.data, 1);
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    return 0;
    }
//
// sht15_send_byte() - send a single byte to the device
// @data:	device state
// @byte:	value to be sent
//
#[no_mangle]
unsafe extern "C" fn sht15_send_byte(data: *mut sht15_data, byte: u8) {
    static void sht15_send_byte(struct sht15_data *data, u8 byte)
    {
    int i;
    for (i = 0; i < 8; i++) {
    sht15_send_bit(data, !!(byte & 0x80));
    byte <<= 1;
    }
    }
//
// sht15_wait_for_response() - checks for ack from device
// @data:	device state
//
#[no_mangle]
unsafe extern "C" fn sht15_wait_for_response(data: *mut sht15_data) -> c_int {
    static int sht15_wait_for_response(struct sht15_data *data)
    {
    int err;
    err = gpiod_direction_input(data.data);
    if (err)
    return err;
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    if (gpiod_get_value(data.data)) {
    gpiod_set_value(data.sck, 0);
    dev_err(data.dev, "Command not acknowledged\n");
    err = sht15_connection_reset(data);
    if (err)
    return err;
    return -EIO;
    }
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    return 0;
    }
//
// sht15_send_cmd() - Sends a command to the device.
// @data:	device state
// @cmd:	command byte to be sent
//
// On entry, sck is output low, data is output pull high
// and the interrupt disabled.
//
#[no_mangle]
unsafe extern "C" fn sht15_send_cmd(data: *mut sht15_data, cmd: u8) -> c_int {
    static int sht15_send_cmd(struct sht15_data *data, u8 cmd)
    {
    int err;
    err = sht15_transmission_start(data);
    if (err)
    return err;
    sht15_send_byte(data, cmd);
    return sht15_wait_for_response(data);
    }
//
// sht15_soft_reset() - send a soft reset command
// @data:	sht15 specific data.
//
// As described in section 3.2 of the datasheet.
//
#[no_mangle]
unsafe extern "C" fn sht15_soft_reset(data: *mut sht15_data) -> c_int {
    static int sht15_soft_reset(struct sht15_data *data)
    {
    int ret;
    ret = sht15_send_cmd(data, SHT15_SOFT_RESET);
    if (ret)
    return ret;
    msleep(SHT15_TSRST);
// device resets default hardware status register value
    data.val_status = 0;
    return ret;
    }
//
// sht15_ack() - send a ack
// @data:	sht15 specific data.
//
// Each byte of data is acknowledged by pulling the data line
// low for one clock pulse.
//
#[no_mangle]
unsafe extern "C" fn sht15_ack(data: *mut sht15_data) -> c_int {
    static int sht15_ack(struct sht15_data *data)
    {
    int err;
    err = gpiod_direction_output(data.data, 0);
    if (err)
    return err;
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSU);
    gpiod_set_value(data.data, 1);
    return gpiod_direction_input(data.data);
    }
//
// sht15_end_transmission() - notify device of end of transmission
// @data:	device state.
//
// This is basically a NAK (single clock pulse, data high).
//
#[no_mangle]
unsafe extern "C" fn sht15_end_transmission(data: *mut sht15_data) -> c_int {
    static int sht15_end_transmission(struct sht15_data *data)
    {
    int err;
    err = gpiod_direction_output(data.data, 1);
    if (err)
    return err;
    ndelay(SHT15_TSU);
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    return 0;
    }
//
// sht15_read_byte() - Read a byte back from the device
// @data:	device state.
//
#[no_mangle]
unsafe extern "C" fn sht15_read_byte(data: *mut sht15_data) -> u8 {
    static u8 sht15_read_byte(struct sht15_data *data)
    {
    int i;
    let mut byte: u8 = 0;
    for (i = 0; i < 8; ++i) {
    byte <<= 1;
    gpiod_set_value(data.sck, 1);
    ndelay(SHT15_TSCKH);
    byte |= !!gpiod_get_value(data.data);
    gpiod_set_value(data.sck, 0);
    ndelay(SHT15_TSCKL);
    }
    return byte;
    }
//
// sht15_send_status() - write the status register byte
// @data:	sht15 specific data.
// @status:	the byte to set the status register with.
//
// As described in figure 14 and table 5 of the datasheet.
//
#[no_mangle]
unsafe extern "C" fn sht15_send_status(data: *mut sht15_data, status: u8) -> c_int {
    static int sht15_send_status(struct sht15_data *data, u8 status)
    {
    int err;
    err = sht15_send_cmd(data, SHT15_WRITE_STATUS);
    if (err)
    return err;
    err = gpiod_direction_output(data.data, 1);
    if (err)
    return err;
    ndelay(SHT15_TSU);
    sht15_send_byte(data, status);
    err = sht15_wait_for_response(data);
    if (err)
    return err;
    data.val_status = status;
    return 0;
    }
//
// sht15_update_status() - get updated status register from device if too old
// @data:	device instance specific data.
//
// As described in figure 15 and table 5 of the datasheet.
//
#[no_mangle]
unsafe extern "C" fn sht15_update_status(data: *mut sht15_data) -> c_int {
    static int sht15_update_status(struct sht15_data *data)
    {
    let mut ret: c_int = 0;
    u8 status;
    u8 previous_config;
    let mut dev_checksum: u8 = 0;
    u8 checksum_vals[2];
    let mut timeout: c_int = HZ;
    mutex_lock(&data.read_lock);
    if (time_after(jiffies, data.last_status + timeout)
    || !data.status_valid) {
    ret = sht15_send_cmd(data, SHT15_READ_STATUS);
    if (ret)
    goto unlock;
    status = sht15_read_byte(data);
    if (data.checksumming) {
    sht15_ack(data);
    dev_checksum = bitrev8(sht15_read_byte(data));
    checksum_vals[0] = SHT15_READ_STATUS;
    checksum_vals[1] = status;
    data.checksum_ok = (sht15_crc8(data, checksum_vals, 2)
    == dev_checksum);
    }
    ret = sht15_end_transmission(data);
    if (ret)
    goto unlock;
//
// Perform checksum validation on the received data.
// Specification mentions that in case a checksum verification
// fails, a soft reset command must be sent to the device.
//
    if (data.checksumming && !data.checksum_ok) {
    previous_config = data.val_status & 0x07;
    ret = sht15_soft_reset(data);
    if (ret)
    goto unlock;
    if (previous_config) {
    ret = sht15_send_status(data, previous_config);
    if (ret) {
    dev_err(data.dev,
    "CRC validation failed, unable "
    "to restore device settings\n");
    goto unlock;
    }
    }
    ret = -EAGAIN;
    goto unlock;
    }
    data.val_status = status;
    data.status_valid = true;
    data.last_status = jiffies;
    }
    unlock:
    mutex_unlock(&data.read_lock);
    return ret;
    }
//
// sht15_measurement() - get a new value from device
// @data:		device instance specific data
// @command:		command sent to request value
// @timeout_msecs:	timeout after which comms are assumed
// to have failed are reset.
//
    static int sht15_measurement(struct sht15_data *data,
    int command,
    int timeout_msecs)
    {
    int ret;
    u8 previous_config;
    ret = sht15_send_cmd(data, command);
    if (ret)
    return ret;
    ret = gpiod_direction_input(data.data);
    if (ret)
    return ret;
    atomic_set(&data.interrupt_handled, 0);
    enable_irq(gpiod_to_irq(data.data));
    if (gpiod_get_value(data.data) == 0) {
    disable_irq_nosync(gpiod_to_irq(data.data));
// Only relevant if the interrupt hasn't occurred.
    if (!atomic_read(&data.interrupt_handled))
    schedule_work(&data.read_work);
    }
    ret = wait_event_timeout(data.wait_queue,
    (data.state == SHT15_READING_NOTHING),
    msecs_to_jiffies(timeout_msecs));
    if (data.state != SHT15_READING_NOTHING) { /* I/O error occurred */
    data.state = SHT15_READING_NOTHING;
    return -EIO;
    } else if (ret == 0) { /* timeout occurred */
    disable_irq_nosync(gpiod_to_irq(data.data));
    ret = sht15_connection_reset(data);
    if (ret)
    return ret;
    return -ETIME;
    }
//
// Perform checksum validation on the received data.
// Specification mentions that in case a checksum verification fails,
// a soft reset command must be sent to the device.
//
    if (data.checksumming && !data.checksum_ok) {
    previous_config = data.val_status & 0x07;
    ret = sht15_soft_reset(data);
    if (ret)
    return ret;
    if (previous_config) {
    ret = sht15_send_status(data, previous_config);
    if (ret) {
    dev_err(data.dev,
    "CRC validation failed, unable "
    "to restore device settings\n");
    return ret;
    }
    }
    return -EAGAIN;
    }
    return 0;
    }
//
// sht15_update_measurements() - get updated measures from device if too old
// @data:	device state
//
#[no_mangle]
unsafe extern "C" fn sht15_update_measurements(data: *mut sht15_data) -> c_int {
    static int sht15_update_measurements(struct sht15_data *data)
    {
    let mut ret: c_int = 0;
    let mut timeout: c_int = HZ;
    mutex_lock(&data.read_lock);
    if (time_after(jiffies, data.last_measurement + timeout)
    || !data.measurements_valid) {
    data.state = SHT15_READING_HUMID;
    ret = sht15_measurement(data, SHT15_MEASURE_RH, 160);
    if (ret)
    goto unlock;
    data.state = SHT15_READING_TEMP;
    ret = sht15_measurement(data, SHT15_MEASURE_TEMP, 400);
    if (ret)
    goto unlock;
    data.measurements_valid = true;
    data.last_measurement = jiffies;
    }
    unlock:
    mutex_unlock(&data.read_lock);
    return ret;
    }
//
// sht15_calc_temp() - convert the raw reading to a temperature
// @data:	device state
//
// As per section 4.3 of the data sheet.
//
#[no_mangle]
pub unsafe extern "C" fn sht15_calc_temp(data: *mut sht15_data) -> c_int {
    static inline int sht15_calc_temp(struct sht15_data *data)
    {
    let mut d1: c_int = temppoints[0].d1;
    let mut d2: c_int = (data.val_status & SHT15_STATUS_LOW_RESOLUTION) ? 40 : 10;
    int i;
    for (i = ARRAY_SIZE(temppoints) - 1; i > 0; i--)
// Find pointer to interpolate
    if (data.supply_uv > temppoints[i - 1].vdd) {
    d1 = (data.supply_uv - temppoints[i - 1].vdd)
// (temppoints[i].d1 - temppoints[i - 1].d1)
    / (temppoints[i].vdd - temppoints[i - 1].vdd)
    + temppoints[i - 1].d1;
    break;
    }
    return data.val_temp * d2 + d1;
    }
//
// sht15_calc_humid() - using last temperature convert raw to humid
// @data:	device state
//
// This is the temperature compensated version as per section 4.2 of
// the data sheet.
//
// The sensor is assumed to be V3, which is compatible with V4.
// Humidity conversion coefficients are shown in table 7 of the datasheet.
//
#[no_mangle]
pub unsafe extern "C" fn sht15_calc_humid(data: *mut sht15_data) -> c_int {
    static inline int sht15_calc_humid(struct sht15_data *data)
    {
    int rh_linear; /* milli percent */
    let mut temp: c_int = sht15_calc_temp(data);
    int c2, c3;
    int t2;
    let mut c1: c_int = -4;
    if (data.val_status & SHT15_STATUS_LOW_RESOLUTION) {
    c2 = 648000; /* x 10 ^ -6 */
    c3 = -7200;  /* x 10 ^ -7 */
    t2 = 1280;
    } else {
    c2 = 40500;  /* x 10 ^ -6 */
    c3 = -28;    /* x 10 ^ -7 */
    t2 = 80;
    }
    rh_linear = c1 * 1000
    + c2 * data.val_humid / 1000
    + (data.val_humid * data.val_humid * c3) / 10000;
    return (temp - 25000) * (10000 + t2 * data.val_humid)
    / 1000000 + rh_linear;
    }
//
// sht15_status_show() - show status information in sysfs
// @dev:	device.
// @attr:	device attribute.
// @buf:	sysfs buffer where information is written to.
//
// Will be called on read access to temp1_fault, humidity1_fault
// and heater_enable sysfs attributes.
// Returns number of bytes written into buffer, negative errno on error.
//
    static ssize_t sht15_status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct sht15_data *data = dev_get_drvdata(dev);
    let mut bit: u8 = to_sensor_dev_attr(attr).index;
    ret = sht15_update_status(data);
    return ret ? ret : sprintf(buf, "%d\n", !!(data.val_status & bit));
    }
//
// sht15_status_store() - change heater state via sysfs
// @dev:	device.
// @attr:	device attribute.
// @buf:	sysfs buffer to read the new heater state from.
// @count:	length of the data.
//
// Will be called on write access to heater_enable sysfs attribute.
// Returns number of bytes actually decoded, negative errno on error.
//
    static ssize_t sht15_status_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    struct sht15_data *data = dev_get_drvdata(dev);
    long value;
    u8 status;
    if (kstrtol(buf, 10, &value))
    return -EINVAL;
    mutex_lock(&data.read_lock);
    status = data.val_status & 0x07;
    if (!!value)
    status |= SHT15_STATUS_HEATER;
    else
    status &= ~SHT15_STATUS_HEATER;
    ret = sht15_send_status(data, status);
    mutex_unlock(&data.read_lock);
    return ret ? ret : count;
    }
//
// sht15_temp_show() - show temperature measurement value in sysfs
// @dev:	device.
// @attr:	device attribute.
// @buf:	sysfs buffer where measurement values are written to.
//
// Will be called on read access to temp1_input sysfs attribute.
// Returns number of bytes written into buffer, negative errno on error.
//
    static ssize_t sht15_temp_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct sht15_data *data = dev_get_drvdata(dev);
// Technically no need to read humidity as well
    ret = sht15_update_measurements(data);
    return ret ? ret : sprintf(buf, "%d\n",
    sht15_calc_temp(data));
    }
//
// sht15_humidity_show() - show humidity measurement value in sysfs
// @dev:	device.
// @attr:	device attribute.
// @buf:	sysfs buffer where measurement values are written to.
//
// Will be called on read access to humidity1_input sysfs attribute.
// Returns number of bytes written into buffer, negative errno on error.
//
    static ssize_t sht15_humidity_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct sht15_data *data = dev_get_drvdata(dev);
    ret = sht15_update_measurements(data);
    return ret ? ret : sprintf(buf, "%d\n", sht15_calc_humid(data));
    }
    static ssize_t name_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct platform_device *pdev = to_platform_device(dev);
    return sprintf(buf, "%s\n", pdev.name);
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, sht15_temp, 0);
    static SENSOR_DEVICE_ATTR_RO(humidity1_input, sht15_humidity, 0);
    static SENSOR_DEVICE_ATTR_RO(temp1_fault, sht15_status,
    SHT15_STATUS_LOW_BATTERY);
    static SENSOR_DEVICE_ATTR_RO(humidity1_fault, sht15_status,
    SHT15_STATUS_LOW_BATTERY);
    static SENSOR_DEVICE_ATTR_RW(heater_enable, sht15_status, SHT15_STATUS_HEATER);
    static DEVICE_ATTR_RO(name);
    static struct attribute *sht15_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_humidity1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_fault.dev_attr.attr,
    &sensor_dev_attr_humidity1_fault.dev_attr.attr,
    &sensor_dev_attr_heater_enable.dev_attr.attr,
    &dev_attr_name.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group sht15_attr_group = {
    .attrs = sht15_attrs,
    };
#[no_mangle]
unsafe extern "C" fn sht15_interrupt_fired(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t sht15_interrupt_fired(int irq, void *d)
    {
    struct sht15_data *data = d;
// First disable the interrupt
    disable_irq_nosync(irq);
    atomic_inc(&data.interrupt_handled);
// Then schedule a reading work struct
    if (data.state != SHT15_READING_NOTHING)
    schedule_work(&data.read_work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sht15_bh_read_data(work_s: *mut work_struct) {
    static void sht15_bh_read_data(struct work_struct *work_s)
    {
    let mut val: u16 = 0;
    let mut dev_checksum: u8 = 0;
    u8 checksum_vals[3];
    struct sht15_data *data
    = container_of(work_s, struct sht15_data,
    read_work);
// Firstly, verify the line is low
    if (gpiod_get_value(data.data)) {
//
// If not, then start the interrupt again - care here as could
// have gone low in meantime so verify it hasn't!
//
    atomic_set(&data.interrupt_handled, 0);
    enable_irq(gpiod_to_irq(data.data));
// If still not occurred or another handler was scheduled
    if (gpiod_get_value(data.data)
    || atomic_read(&data.interrupt_handled))
    return;
    }
// Read the data back from the device
    val = sht15_read_byte(data);
    val <<= 8;
    if (sht15_ack(data))
    goto wakeup;
    val |= sht15_read_byte(data);
    if (data.checksumming) {
//
// Ask the device for a checksum and read it back.
// Note: the device sends the checksum byte reversed.
//
    if (sht15_ack(data))
    goto wakeup;
    dev_checksum = bitrev8(sht15_read_byte(data));
    checksum_vals[0] = (data.state == SHT15_READING_TEMP) ?
    SHT15_MEASURE_TEMP : SHT15_MEASURE_RH;
    checksum_vals[1] = (u8) (val >> 8);
    checksum_vals[2] = (u8) val;
    data.checksum_ok
    = (sht15_crc8(data, checksum_vals, 3) == dev_checksum);
    }
// Tell the device we are done
    if (sht15_end_transmission(data))
    goto wakeup;
    switch (data.state) {
    case SHT15_READING_TEMP:
    data.val_temp = val;
    break;
    case SHT15_READING_HUMID:
    data.val_humid = val;
    break;
    default:
    break;
    }
    data.state = SHT15_READING_NOTHING;
    wakeup:
    wake_up(&data.wait_queue);
    }
#[no_mangle]
unsafe extern "C" fn sht15_update_voltage(work_s: *mut work_struct) {
    static void sht15_update_voltage(struct work_struct *work_s)
    {
    struct sht15_data *data
    = container_of(work_s, struct sht15_data,
    update_supply_work);
    data.supply_uv = regulator_get_voltage(data.reg);
    }
//
// sht15_invalidate_voltage() - mark supply voltage invalid when notified by reg
// @nb:		associated notification structure
// @event:	voltage regulator state change event code
// @ignored:	function parameter - ignored here
//
// Note that as the notification code holds the regulator lock, we have
// to schedule an update of the supply voltage rather than getting it directly.
//
    static int sht15_invalidate_voltage(struct notifier_block *nb,
    unsigned long event,
    void *ignored)
    {
    struct sht15_data *data = container_of(nb, struct sht15_data, nb);
    if (event == REGULATOR_EVENT_VOLTAGE_CHANGE)
    data.supply_uv_valid = false;
    schedule_work(&data.update_supply_work);
    return NOTIFY_OK;
    }

    static const struct of_device_id sht15_dt_match[] = {
    { .compatible = "sensirion,sht15" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sht15_dt_match);

#[no_mangle]
unsafe extern "C" fn sht15_probe(pdev: *mut platform_device) -> c_int {
    static int sht15_probe(struct platform_device *pdev)
    {
    int ret;
    struct sht15_data *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    INIT_WORK(&data.read_work, sht15_bh_read_data);
    INIT_WORK(&data.update_supply_work, sht15_update_voltage);
    platform_set_drvdata(pdev, data);
    mutex_init(&data.read_lock);
    data.dev = &pdev.dev;
    init_waitqueue_head(&data.wait_queue);
//
// If a regulator is available,
// query what the supply voltage actually is!
//
    data.reg = devm_regulator_get_optional(data.dev, "vcc");
    if (!IS_ERR(data.reg)) {
    int voltage;
    voltage = regulator_get_voltage(data.reg);
    if (voltage)
    data.supply_uv = voltage;
    ret = regulator_enable(data.reg);
    if (ret != 0) {
    dev_err(&pdev.dev,
    "failed to enable regulator: %d\n", ret);
    return ret;
    }
//
// Setup a notifier block to update this if another device
// causes the voltage to change
//
    data.nb.notifier_call = &sht15_invalidate_voltage;
    ret = regulator_register_notifier(data.reg, &data.nb);
    if (ret) {
    dev_err(&pdev.dev,
    "regulator notifier request failed\n");
    regulator_disable(data.reg);
    return ret;
    }
    }
// Try requesting the GPIOs
    data.sck = devm_gpiod_get(&pdev.dev, "clk", GPIOD_OUT_LOW);
    if (IS_ERR(data.sck)) {
    ret = PTR_ERR(data.sck);
    dev_err(&pdev.dev, "clock line GPIO request failed\n");
    goto err_release_reg;
    }
    data.data = devm_gpiod_get(&pdev.dev, "data", GPIOD_IN);
    if (IS_ERR(data.data)) {
    ret = PTR_ERR(data.data);
    dev_err(&pdev.dev, "data line GPIO request failed\n");
    goto err_release_reg;
    }
    ret = devm_request_irq(&pdev.dev, gpiod_to_irq(data.data),
    sht15_interrupt_fired,
    IRQF_TRIGGER_FALLING,
    "sht15 data",
    data);
    if (ret)
    goto err_release_reg;
    disable_irq_nosync(gpiod_to_irq(data.data));
    ret = sht15_connection_reset(data);
    if (ret)
    goto err_release_reg;
    ret = sht15_soft_reset(data);
    if (ret)
    goto err_release_reg;
    ret = sysfs_create_group(&pdev.dev.kobj, &sht15_attr_group);
    if (ret) {
    dev_err(&pdev.dev, "sysfs create failed\n");
    goto err_release_reg;
    }
    data.hwmon_dev = hwmon_device_register(data.dev);
    if (IS_ERR(data.hwmon_dev)) {
    ret = PTR_ERR(data.hwmon_dev);
    goto err_release_sysfs_group;
    }
    return 0;
    err_release_sysfs_group:
    sysfs_remove_group(&pdev.dev.kobj, &sht15_attr_group);
    err_release_reg:
    if (!IS_ERR(data.reg)) {
    regulator_unregister_notifier(data.reg, &data.nb);
    regulator_disable(data.reg);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sht15_remove(pdev: *mut platform_device) {
    static void sht15_remove(struct platform_device *pdev)
    {
    struct sht15_data *data = platform_get_drvdata(pdev);
    int ret;
    hwmon_device_unregister(data.hwmon_dev);
    sysfs_remove_group(&pdev.dev.kobj, &sht15_attr_group);
    ret = sht15_soft_reset(data);
    if (ret)
    dev_err(&pdev.dev, "Failed to reset device (%pe)\n", ERR_PTR(ret));
    if (!IS_ERR(data.reg)) {
    regulator_unregister_notifier(data.reg, &data.nb);
    regulator_disable(data.reg);
    }
    }
    static const struct platform_device_id sht15_device_ids[] = {
    { .name = "sht10", .driver_data = sht10 },
    { .name = "sht11", .driver_data = sht11 },
    { .name = "sht15", .driver_data = sht15 },
    { .name = "sht71", .driver_data = sht71 },
    { .name = "sht75", .driver_data = sht75 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, sht15_device_ids);
    static struct platform_driver sht15_driver = {
    .driver = {
    .name = "sht15",
    .of_match_table = of_match_ptr(sht15_dt_match),
    },
    .probe = sht15_probe,
    .remove = sht15_remove,
    .id_table = sht15_device_ids,
    };
    module_platform_driver(sht15_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Sensirion SHT15 temperature and humidity sensor driver");
