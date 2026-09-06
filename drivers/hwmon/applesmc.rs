//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/applesmc.c
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
// drivers/hwmon/applesmc.c - driver for Apple's SMC (accelerometer, temperature
// sensors, fan control, keyboard backlight control) used in Intel-based Apple
// computers.
//
// Copyright (C) 2007 Nicolas Boichat <nicolas@boichat.ch>
// Copyright (C) 2010 Henrik Rydberg <rydberg@euromail.se>
//
// Based on hdaps.c driver:
// Copyright (C) 2005 Robert Love <rml@novell.com>
// Copyright (C) 2005 Jesper Juhl <jj@chaosbits.net>
//
// Fan control based on smcFanControl:
// Copyright (C) 2006 Hendrik Holtmann <holtmann@mac.com>
//

// data port used by Apple SMC
pub const APPLESMC_DATA_PORT: c_uint = 0x300;
// command/status port used by Apple SMC
pub const APPLESMC_CMD_PORT: c_uint = 0x304;

pub const APPLESMC_MAX_DATA_LENGTH: c_int = 32;
// Apple SMC status bits

// Initial wait is 8us
pub const APPLESMC_MIN_WAIT: c_uint = 0x0008;
pub const APPLESMC_READ_CMD: c_uint = 0x10;
pub const APPLESMC_WRITE_CMD: c_uint = 0x11;
pub const APPLESMC_GET_KEY_BY_INDEX_CMD: c_uint = 0x12;
pub const APPLESMC_GET_KEY_TYPE_CMD: c_uint = 0x13;

// List of keys used to read/write fan speeds
    static const char *const fan_speed_fmt[] = {
    "F%dAc",		/* actual speed */
    "F%dMn",		/* minimum speed (rw) */
    "F%dMx",		/* maximum speed */
    "F%dSf",		/* safe speed - not all models */
    "F%dTg",		/* target speed (manual: rw) */
    };

pub const APPLESMC_INPUT_FLAT: c_int = 4;

// Dynamic device node attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct applesmc_dev_attr {
    pub /: *mut *mut sensor_device_attribute sda; / hwmon attributes,
    pub /: *mut *mut char name[32]; / room for node file name,
}

// Dynamic device node group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct applesmc_node_group {
    pub /: *mut *mut *mut char format; / format string,
    pub /: *mut *mut *mut void show; / show function,
    pub /: *mut *mut *mut void store; / store function,
    pub /: *mut *mut int option; / function argument,
    pub /: *mut *mut *mut applesmc_dev_attr nodes; / dynamic node array,
}

// AppleSMC entry - cached register information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct applesmc_entry {
    pub /: *mut *mut char key[5]; / four-letter key code,
    pub /: *mut *mut u8 valid; / set when entry is successfully read once,
    pub /: *mut *mut u8 len; / bounded by APPLESMC_MAX_DATA_LENGTH,
    pub /: *mut *mut char type[5]; / four-letter type code,
    pub /: *mut *mut u8 flags; / 0x10: func; 0x40: write; 0x80: read,
}

// Register lookup and registers common to all SMCs
    static struct applesmc_registers {
    struct mutex mutex;		/* register read/write mutex */
    unsigned int key_count;		/* number of SMC registers */
    unsigned int fan_count;		/* number of fans */
    unsigned int temp_count;	/* number of temperature registers */
    unsigned int temp_begin;	/* temperature lower index bound */
    unsigned int temp_end;		/* temperature upper index bound */
    unsigned int index_count;	/* size of temperature index array */
    int num_light_sensors;		/* number of light sensors */
    bool has_accelerometer;		/* has motion sensor */
    bool has_key_backlight;		/* has keyboard backlight */
    bool init_complete;		/* true when fully initialized */
    struct applesmc_entry *cache;	/* cached key entries */
    const char **index;		/* temperature key index */
    char fan_positions[10][17];	/* cached fan position labels */
    } smcreg = {
    .mutex = __MUTEX_INITIALIZER(smcreg.mutex),
    };
    static const int debug;
    static struct platform_device *pdev;
    static s16 rest_x;
    static s16 rest_y;
    static u8 backlight_state[2];
    static struct input_dev *applesmc_idev;
    static struct device *hwmon_dev;
//
// Last index written to key_at_index sysfs file, and value to use for all other
// key_at_index_* sysfs files.
//
    static unsigned int key_at_index;
    static struct workqueue_struct *applesmc_led_wq;
//
// Wait for specific status bits with a mask on the SMC.
// Used before all transactions.
// This does 10 fast loops of 8us then exponentially backs off for a
// minimum total wait of 262ms. Depending on usleep_range this could
// run out past 500ms.
//
#[no_mangle]
unsafe extern "C" fn wait_status(val: u8, mask: u8) -> c_int {
    static int wait_status(u8 val, u8 mask)
    {
    u8 status;
    int us;
    int i;
    us = APPLESMC_MIN_WAIT;
    for (i = 0; i < 24 ; i++) {
    status = inb(APPLESMC_CMD_PORT);
    if ((status & mask) == val)
    return 0;
    usleep_range(us, us * 2);
    if (i > 9)
    us <<= 1;
    }
    return -EIO;
    }
// send_byte - Write to SMC data port. Callers must hold applesmc_lock.
#[no_mangle]
unsafe extern "C" fn send_byte(cmd: u8, port: u16) -> c_int {
    static int send_byte(u8 cmd, u16 port)
    {
    int status;
    status = wait_status(0, SMC_STATUS_IB_CLOSED);
    if (status)
    return status;
//
// This needs to be a separate read looking for bit 0x04
// after bit 0x02 falls. If consolidated with the wait above
// this extra read may not happen if status returns both
// simultaneously and this would appear to be required.
//
    status = wait_status(SMC_STATUS_BUSY, SMC_STATUS_BUSY);
    if (status)
    return status;
    outb(cmd, port);
    return 0;
    }
// send_command - Write a command to the SMC. Callers must hold applesmc_lock.
#[no_mangle]
unsafe extern "C" fn send_command(cmd: u8) -> c_int {
    static int send_command(u8 cmd)
    {
    int ret;
    ret = wait_status(0, SMC_STATUS_IB_CLOSED);
    if (ret)
    return ret;
    outb(cmd, APPLESMC_CMD_PORT);
    return 0;
    }
//
// Based on logic from the Apple driver. This is issued before any interaction
// If busy is stuck high, issue a read command to reset the SMC state machine.
// If busy is stuck high after the command then the SMC is jammed.
//
#[no_mangle]
unsafe extern "C" fn smc_sane() -> c_int {
    static int smc_sane(void)
    {
    int ret;
    ret = wait_status(0, SMC_STATUS_BUSY);
    if (!ret)
    return ret;
    ret = send_command(APPLESMC_READ_CMD);
    if (ret)
    return ret;
    return wait_status(0, SMC_STATUS_BUSY);
    }
#[no_mangle]
unsafe extern "C" fn send_argument(key: *const c_char) -> c_int {
    static int send_argument(const char *key)
    {
    int i;
    for (i = 0; i < 4; i++)
    if (send_byte(key[i], APPLESMC_DATA_PORT))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_smc(cmd: u8, key: *const c_char, buffer: *mut u8, len: u8) -> c_int {
    static int read_smc(u8 cmd, const char *key, u8 *buffer, u8 len)
    {
    u8 status, data = 0;
    int i;
    int ret;
    ret = smc_sane();
    if (ret)
    return ret;
    if (send_command(cmd) || send_argument(key)) {
    pr_warn("%.4s: read arg fail\n", key);
    return -EIO;
    }
// This has no effect on newer (2012) SMCs
    if (send_byte(len, APPLESMC_DATA_PORT)) {
    pr_warn("%.4s: read len fail\n", key);
    return -EIO;
    }
    for (i = 0; i < len; i++) {
    if (wait_status(SMC_STATUS_AWAITING_DATA | SMC_STATUS_BUSY,
    SMC_STATUS_AWAITING_DATA | SMC_STATUS_BUSY)) {
    pr_warn("%.4s: read data[%d] fail\n", key, i);
    return -EIO;
    }
    buffer[i] = inb(APPLESMC_DATA_PORT);
    }
// Read the data port until bit0 is cleared
    for (i = 0; i < 16; i++) {
    udelay(APPLESMC_MIN_WAIT);
    status = inb(APPLESMC_CMD_PORT);
    if (!(status & SMC_STATUS_AWAITING_DATA))
    break;
    data = inb(APPLESMC_DATA_PORT);
    }
    if (i)
    pr_warn("flushed %d bytes, last value is: %d\n", i, data);
    return wait_status(0, SMC_STATUS_BUSY);
    }
#[no_mangle]
unsafe extern "C" fn write_smc(cmd: u8, key: *const c_char, buffer: *const u8, len: u8) -> c_int {
    static int write_smc(u8 cmd, const char *key, const u8 *buffer, u8 len)
    {
    int i;
    int ret;
    ret = smc_sane();
    if (ret)
    return ret;
    if (send_command(cmd) || send_argument(key)) {
    pr_warn("%s: write arg fail\n", key);
    return -EIO;
    }
    if (send_byte(len, APPLESMC_DATA_PORT)) {
    pr_warn("%.4s: write len fail\n", key);
    return -EIO;
    }
    for (i = 0; i < len; i++) {
    if (send_byte(buffer[i], APPLESMC_DATA_PORT)) {
    pr_warn("%s: write data fail\n", key);
    return -EIO;
    }
    }
    return wait_status(0, SMC_STATUS_BUSY);
    }
#[no_mangle]
unsafe extern "C" fn read_register_count(count: *mut c_uint) -> c_int {
    static int read_register_count(unsigned int *count)
    {
    __be32 be;
    int ret;
    ret = read_smc(APPLESMC_READ_CMD, KEY_COUNT_KEY, (u8 *)&be, 4);
    if (ret)
    return ret;
// count = be32_to_cpu(be);
    return 0;
    }
//
// Serialized I/O
//
// Returns zero on success or a negative error on failure.
// All functions below are concurrency safe - callers should NOT hold lock.
//
    static int applesmc_read_entry(const struct applesmc_entry *entry,
    u8 *buf, u8 len)
    {
    int ret;
    if (entry.len != len)
    return -EINVAL;
    mutex_lock(&smcreg.mutex);
    ret = read_smc(APPLESMC_READ_CMD, entry.key, buf, len);
    mutex_unlock(&smcreg.mutex);
    return ret;
    }
    static int applesmc_write_entry(const struct applesmc_entry *entry,
    const u8 *buf, u8 len)
    {
    int ret;
    if (entry.len != len)
    return -EINVAL;
    mutex_lock(&smcreg.mutex);
    ret = write_smc(APPLESMC_WRITE_CMD, entry.key, buf, len);
    mutex_unlock(&smcreg.mutex);
    return ret;
    }
    static const struct applesmc_entry *applesmc_get_entry_by_index(int index)
    {
    struct applesmc_entry *cache = &smcreg.cache[index];
    u8 key[4], info[6];
    __be32 be;
    let mut ret: c_int = 0;
// Pairs with smp_store_release() to ensure cache contents are visible
    if (smp_load_acquire(&cache.valid))
    return cache;
    mutex_lock(&smcreg.mutex);
    if (cache.valid)
    goto out;
    be = cpu_to_be32(index);
    ret = read_smc(APPLESMC_GET_KEY_BY_INDEX_CMD, (u8 *)&be, key, 4);
    if (ret)
    goto out;
    ret = read_smc(APPLESMC_GET_KEY_TYPE_CMD, key, info, 6);
    if (ret)
    goto out;
    memcpy(cache.key, key, 4);
    cache.len = info[0];
    memcpy(cache.type, &info[1], 4);
    cache.flags = info[5];
// Pairs with smp_load_acquire() to commit cache contents before setting valid
    smp_store_release(&cache.valid, true);
    out:
    mutex_unlock(&smcreg.mutex);
    if (ret)
    return ERR_PTR(ret);
    return cache;
    }
#[no_mangle]
unsafe extern "C" fn applesmc_get_lower_bound(lo: *mut c_uint, key: *const c_char) -> c_int {
    static int applesmc_get_lower_bound(unsigned int *lo, const char *key)
    {
    let mut begin: c_int = 0, end = smcreg.key_count;
    const struct applesmc_entry *entry;
    while (begin != end) {
    let mut middle: c_int = begin + (end - begin) / 2;
    entry = applesmc_get_entry_by_index(middle);
    if (IS_ERR(entry)) {
// lo = 0;
    return PTR_ERR(entry);
    }
    if (strcmp(entry.key, key) < 0)
    begin = middle + 1;
    else
    end = middle;
    }
// lo = begin;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn applesmc_get_upper_bound(hi: *mut c_uint, key: *const c_char) -> c_int {
    static int applesmc_get_upper_bound(unsigned int *hi, const char *key)
    {
    let mut begin: c_int = 0, end = smcreg.key_count;
    const struct applesmc_entry *entry;
    while (begin != end) {
    let mut middle: c_int = begin + (end - begin) / 2;
    entry = applesmc_get_entry_by_index(middle);
    if (IS_ERR(entry)) {
// hi = smcreg.key_count;
    return PTR_ERR(entry);
    }
    if (strcmp(key, entry.key) < 0)
    end = middle;
    else
    begin = middle + 1;
    }
// hi = begin;
    return 0;
    }
    static const struct applesmc_entry *applesmc_get_entry_by_key(const char *key)
    {
    int begin, end;
    int ret;
    ret = applesmc_get_lower_bound(&begin, key);
    if (ret)
    return ERR_PTR(ret);
    ret = applesmc_get_upper_bound(&end, key);
    if (ret)
    return ERR_PTR(ret);
    if (end - begin != 1)
    return ERR_PTR(-EINVAL);
    return applesmc_get_entry_by_index(begin);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_read_key(key: *const c_char, buffer: *mut u8, len: u8) -> c_int {
    static int applesmc_read_key(const char *key, u8 *buffer, u8 len)
    {
    const struct applesmc_entry *entry;
    entry = applesmc_get_entry_by_key(key);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    return applesmc_read_entry(entry, buffer, len);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_write_key(key: *const c_char, buffer: *const u8, len: u8) -> c_int {
    static int applesmc_write_key(const char *key, const u8 *buffer, u8 len)
    {
    const struct applesmc_entry *entry;
    entry = applesmc_get_entry_by_key(key);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    return applesmc_write_entry(entry, buffer, len);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_has_key(key: *const c_char, value: *mut bool) -> c_int {
    static int applesmc_has_key(const char *key, bool *value)
    {
    const struct applesmc_entry *entry;
    entry = applesmc_get_entry_by_key(key);
    if (IS_ERR(entry) && PTR_ERR(entry) != -EINVAL)
    return PTR_ERR(entry);
// value = !IS_ERR(entry);
    return 0;
    }
//
// applesmc_read_s16 - Read 16-bit signed big endian register
//
#[no_mangle]
unsafe extern "C" fn applesmc_read_s16(key: *const c_char, value: *mut i16) -> c_int {
    static int applesmc_read_s16(const char *key, s16 *value)
    {
    u8 buffer[2];
    int ret;
    ret = applesmc_read_key(key, buffer, 2);
    if (ret)
    return ret;
// value = ((s16)buffer[0] << 8) | buffer[1];
    return 0;
    }
//
// applesmc_device_init - initialize the accelerometer.  Can sleep.
//
#[no_mangle]
unsafe extern "C" fn applesmc_device_init() {
    static void applesmc_device_init(void)
    {
    int total;
    u8 buffer[2];
    if (!smcreg.has_accelerometer)
    return;
    for (total = INIT_TIMEOUT_MSECS; total > 0; total -= INIT_WAIT_MSECS) {
    if (!applesmc_read_key(MOTION_SENSOR_KEY, buffer, 2) &&
    (buffer[0] != 0x00 || buffer[1] != 0x00))
    return;
    buffer[0] = 0xe0;
    buffer[1] = 0x00;
    applesmc_write_key(MOTION_SENSOR_KEY, buffer, 2);
    msleep(INIT_WAIT_MSECS);
    }
    pr_warn("failed to init the device\n");
    }
#[no_mangle]
unsafe extern "C" fn applesmc_init_index(s: *mut applesmc_registers) -> c_int {
    static int applesmc_init_index(struct applesmc_registers *s)
    {
    const struct applesmc_entry *entry;
    unsigned int i;
    if (s.index)
    return 0;
    s.index = kcalloc(s.temp_count, sizeof(s.index[0]), GFP_KERNEL);
    if (!s.index)
    return -ENOMEM;
    for (i = s.temp_begin; i < s.temp_end; i++) {
    entry = applesmc_get_entry_by_index(i);
    if (IS_ERR(entry))
    continue;
    if (strcmp(entry.type, TEMP_SENSOR_TYPE))
    continue;
    s.index[s.index_count++] = entry.key;
    }
    return 0;
    }
//
// applesmc_init_smcreg_try - Try to initialize register cache. Idempotent.
//
#[no_mangle]
unsafe extern "C" fn applesmc_init_smcreg_try() -> c_int {
    static int applesmc_init_smcreg_try(void)
    {
    struct applesmc_registers *s = &smcreg;
    let mut left_light_sensor: bool = false, right_light_sensor = false;
    unsigned int count, i;
    u8 tmp[1];
    int ret;
    if (s.init_complete)
    return 0;
    ret = read_register_count(&count);
    if (ret)
    return ret;
    if (s.cache && s.key_count != count) {
    pr_warn("key count changed from %d to %d\n",
    s.key_count, count);
    kfree(s.cache);
    s.cache = core::ptr::null_mut();
    }
    s.key_count = count;
    if (!s.cache)
    s.cache = kzalloc_objs(*s.cache, s.key_count);
    if (!s.cache)
    return -ENOMEM;
    ret = applesmc_read_key(FANS_COUNT, tmp, 1);
    if (ret)
    return ret;
    s.fan_count = tmp[0];
    if (s.fan_count > 10)
    s.fan_count = 10;
    for (i = 0; i < s.fan_count; i++) {
    char newkey[5];
    scnprintf(newkey, sizeof(newkey), FAN_ID_FMT, i);
    ret = applesmc_read_key(newkey, s.fan_positions[i], 16);
    s.fan_positions[i][16] = 0;
    if (ret)
    scnprintf(s.fan_positions[i], 17, "    Fan %d", i);
    }
    ret = applesmc_get_lower_bound(&s.temp_begin, "T");
    if (ret)
    return ret;
    ret = applesmc_get_lower_bound(&s.temp_end, "U");
    if (ret)
    return ret;
    s.temp_count = s.temp_end - s.temp_begin;
    ret = applesmc_init_index(s);
    if (ret)
    return ret;
    ret = applesmc_has_key(LIGHT_SENSOR_LEFT_KEY, &left_light_sensor);
    if (ret)
    return ret;
    ret = applesmc_has_key(LIGHT_SENSOR_RIGHT_KEY, &right_light_sensor);
    if (ret)
    return ret;
    ret = applesmc_has_key(MOTION_SENSOR_KEY, &s.has_accelerometer);
    if (ret)
    return ret;
    ret = applesmc_has_key(BACKLIGHT_KEY, &s.has_key_backlight);
    if (ret)
    return ret;
    s.num_light_sensors = left_light_sensor + right_light_sensor;
    s.init_complete = true;
    pr_info("key=%d fan=%d temp=%d index=%d acc=%d lux=%d kbd=%d\n",
    s.key_count, s.fan_count, s.temp_count, s.index_count,
    s.has_accelerometer,
    s.num_light_sensors,
    s.has_key_backlight);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn applesmc_destroy_smcreg() {
    static void applesmc_destroy_smcreg(void)
    {
    kfree(smcreg.index);
    smcreg.index = core::ptr::null_mut();
    kfree(smcreg.cache);
    smcreg.cache = core::ptr::null_mut();
    smcreg.init_complete = false;
    }
//
// applesmc_init_smcreg - Initialize register cache.
//
// Retries until initialization is successful, or the operation times out.
//
#[no_mangle]
unsafe extern "C" fn applesmc_init_smcreg() -> c_int {
    static int applesmc_init_smcreg(void)
    {
    int ms, ret;
    for (ms = 0; ms < INIT_TIMEOUT_MSECS; ms += INIT_WAIT_MSECS) {
    ret = applesmc_init_smcreg_try();
    if (!ret) {
    if (ms)
    pr_info("init_smcreg() took %d ms\n", ms);
    return 0;
    }
    msleep(INIT_WAIT_MSECS);
    }
    applesmc_destroy_smcreg();
    return ret;
    }
// Device model stuff
#[no_mangle]
unsafe extern "C" fn applesmc_probe(dev: *mut platform_device) -> c_int {
    static int applesmc_probe(struct platform_device *dev)
    {
    int ret;
    ret = applesmc_init_smcreg();
    if (ret)
    return ret;
    applesmc_device_init();
    return 0;
    }
// Synchronize device with memorized backlight state
#[no_mangle]
unsafe extern "C" fn applesmc_pm_resume(dev: *mut device) -> c_int {
    static int applesmc_pm_resume(struct device *dev)
    {
    if (smcreg.has_key_backlight)
    applesmc_write_key(BACKLIGHT_KEY, backlight_state, 2);
    return 0;
    }
// Reinitialize device on resume from hibernation
#[no_mangle]
unsafe extern "C" fn applesmc_pm_restore(dev: *mut device) -> c_int {
    static int applesmc_pm_restore(struct device *dev)
    {
    applesmc_device_init();
    return applesmc_pm_resume(dev);
    }
    static const struct dev_pm_ops applesmc_pm_ops = {
    .resume = applesmc_pm_resume,
    .restore = applesmc_pm_restore,
    };
    static struct platform_driver applesmc_driver = {
    .probe = applesmc_probe,
    .driver	= {
    .name = "applesmc",
    .pm = &applesmc_pm_ops,
    },
    };
//
// applesmc_calibrate - Set our "resting" values.  Callers must
// hold applesmc_lock.
//
#[no_mangle]
unsafe extern "C" fn applesmc_calibrate() {
    static void applesmc_calibrate(void)
    {
    applesmc_read_s16(MOTION_SENSOR_X_KEY, &rest_x);
    applesmc_read_s16(MOTION_SENSOR_Y_KEY, &rest_y);
    rest_x = -rest_x;
    }
#[no_mangle]
unsafe extern "C" fn applesmc_idev_poll(idev: *mut input_dev) {
    static void applesmc_idev_poll(struct input_dev *idev)
    {
    s16 x, y;
    if (applesmc_read_s16(MOTION_SENSOR_X_KEY, &x))
    return;
    if (applesmc_read_s16(MOTION_SENSOR_Y_KEY, &y))
    return;
    x = -x;
    input_report_abs(idev, ABS_X, x - rest_x);
    input_report_abs(idev, ABS_Y, y - rest_y);
    input_sync(idev);
    }
// Sysfs Files
    static ssize_t applesmc_name_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "applesmc\n");
    }
    static ssize_t applesmc_position_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    s16 x, y, z;
    ret = applesmc_read_s16(MOTION_SENSOR_X_KEY, &x);
    if (ret)
    goto out;
    ret = applesmc_read_s16(MOTION_SENSOR_Y_KEY, &y);
    if (ret)
    goto out;
    ret = applesmc_read_s16(MOTION_SENSOR_Z_KEY, &z);
    if (ret)
    goto out;
    out:
    if (ret)
    return ret;
    return sysfs_emit(buf, "(%d,%d,%d)\n", x, y, z);
    }
    static ssize_t applesmc_light_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    const struct applesmc_entry *entry;
    static int data_length;
    int ret;
    let mut left: u8 = 0, right = 0;
    u8 buffer[10];
    if (!data_length) {
    entry = applesmc_get_entry_by_key(LIGHT_SENSOR_LEFT_KEY);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    if (entry.len > 10)
    return -ENXIO;
    data_length = entry.len;
    pr_info("light sensor data length set to %d\n", data_length);
    }
    ret = applesmc_read_key(LIGHT_SENSOR_LEFT_KEY, buffer, data_length);
    if (ret)
    goto out;
// newer macbooks report a single 10-bit bigendian value
    if (data_length == 10) {
    left = be16_to_cpu(*(__be16 *)(buffer + 6)) >> 2;
    goto out;
    }
    left = buffer[2];
    ret = applesmc_read_key(LIGHT_SENSOR_RIGHT_KEY, buffer, data_length);
    if (ret)
    goto out;
    right = buffer[2];
    out:
    if (ret)
    return ret;
    return sysfs_emit(sysfsbuf, "(%d,%d)\n", left, right);
    }
    static ssize_t applesmc_show_fan_speed(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    int ret;
    let mut speed: c_uint = 0;
    char newkey[5];
    u8 buffer[2];
    scnprintf(newkey, sizeof(newkey), fan_speed_fmt[to_option(attr)],
    to_index(attr));
    ret = applesmc_read_key(newkey, buffer, 2);
    if (ret)
    return ret;
    speed = ((buffer[0] << 8 | buffer[1]) >> 2);
    return sysfs_emit(sysfsbuf, "%u\n", speed);
    }
    static ssize_t applesmc_calibrate_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    return sysfs_emit(sysfsbuf, "(%d,%d)\n", rest_x, rest_y);
    }
    static ssize_t applesmc_calibrate_store(struct device *dev,
    struct device_attribute *attr, const char *sysfsbuf, size_t count)
    {
    applesmc_calibrate();
    return count;
    }
#[no_mangle]
unsafe extern "C" fn applesmc_backlight_set(work: *mut work_struct) {
    static void applesmc_backlight_set(struct work_struct *work)
    {
    applesmc_write_key(BACKLIGHT_KEY, backlight_state, 2);
    }
    static DECLARE_WORK(backlight_work, &applesmc_backlight_set);
    static void applesmc_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    int ret;
    backlight_state[0] = value;
    ret = queue_work(applesmc_led_wq, &backlight_work);
    if (debug && (!ret))
    dev_dbg(led_cdev.dev, "work was already on the queue.\n");
    }
    static ssize_t applesmc_key_count_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    int ret;
    u8 buffer[4];
    u32 count;
    ret = applesmc_read_key(KEY_COUNT_KEY, buffer, 4);
    if (ret)
    return ret;
    count = ((u32)buffer[0]<<24) + ((u32)buffer[1]<<16) +
    ((u32)buffer[2]<<8) + buffer[3];
    return sysfs_emit(sysfsbuf, "%d\n", count);
    }
    static ssize_t applesmc_key_at_index_read_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    const struct applesmc_entry *entry;
    int ret;
    entry = applesmc_get_entry_by_index(key_at_index);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    ret = applesmc_read_entry(entry, sysfsbuf, entry.len);
    if (ret)
    return ret;
    return entry.len;
    }
    static ssize_t applesmc_key_at_index_data_length_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    const struct applesmc_entry *entry;
    entry = applesmc_get_entry_by_index(key_at_index);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    return sysfs_emit(sysfsbuf, "%d\n", entry.len);
    }
    static ssize_t applesmc_key_at_index_type_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    const struct applesmc_entry *entry;
    entry = applesmc_get_entry_by_index(key_at_index);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    return sysfs_emit(sysfsbuf, "%s\n", entry.type);
    }
    static ssize_t applesmc_key_at_index_name_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    const struct applesmc_entry *entry;
    entry = applesmc_get_entry_by_index(key_at_index);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    return sysfs_emit(sysfsbuf, "%s\n", entry.key);
    }
    static ssize_t applesmc_key_at_index_show(struct device *dev,
    struct device_attribute *attr, char *sysfsbuf)
    {
    return sysfs_emit(sysfsbuf, "%d\n", key_at_index);
    }
    static ssize_t applesmc_key_at_index_store(struct device *dev,
    struct device_attribute *attr, const char *sysfsbuf, size_t count)
    {
    unsigned long newkey;
    if (kstrtoul(sysfsbuf, 10, &newkey) < 0
    || newkey >= smcreg.key_count)
    return -EINVAL;
    key_at_index = newkey;
    return count;
    }
    static struct led_classdev applesmc_backlight = {
    .name			= "smc::kbd_backlight",
    .default_trigger	= "nand-disk",
    .brightness_set		= applesmc_brightness_set,
    };
    static struct applesmc_node_group info_group[] = {
    { "name", applesmc_name_show },
    { "key_count", applesmc_key_count_show },
    { "key_at_index", applesmc_key_at_index_show, applesmc_key_at_index_store },
    { "key_at_index_name", applesmc_key_at_index_name_show },
    { "key_at_index_type", applesmc_key_at_index_type_show },
    { "key_at_index_data_length", applesmc_key_at_index_data_length_show },
    { "key_at_index_data", applesmc_key_at_index_read_show },
    { }
    };
    static struct applesmc_node_group accelerometer_group[] = {
    { "position", applesmc_position_show },
    { "calibrate", applesmc_calibrate_show, applesmc_calibrate_store },
    { }
    };
    static struct applesmc_node_group light_sensor_group[] = {
    { "light", applesmc_light_show },
    { }
    };
// Module stuff
//
// applesmc_destroy_nodes - remove files and free associated memory
//
#[no_mangle]
unsafe extern "C" fn applesmc_destroy_nodes(groups: *mut applesmc_node_group) {
    static void applesmc_destroy_nodes(struct applesmc_node_group *groups)
    {
    struct applesmc_node_group *grp;
    struct applesmc_dev_attr *node;
    for (grp = groups; grp.nodes; grp++) {
    for (node = grp.nodes; node.sda.dev_attr.attr.name; node++)
    sysfs_remove_file(&pdev.dev.kobj,
    &node.sda.dev_attr.attr);
    kfree(grp.nodes);
    grp.nodes = core::ptr::null_mut();
    }
    }
//
// applesmc_create_nodes - create a two-dimensional group of sysfs files
//
#[no_mangle]
unsafe extern "C" fn applesmc_create_nodes(groups: *mut applesmc_node_group, num: c_int) -> c_int {
    static int applesmc_create_nodes(struct applesmc_node_group *groups, int num)
    {
    struct applesmc_node_group *grp;
    struct applesmc_dev_attr *node;
    struct attribute *attr;
    int ret, i;
    for (grp = groups; grp.format; grp++) {
    grp.nodes = kzalloc_objs(*node, num + 1);
    if (!grp.nodes) {
    ret = -ENOMEM;
    goto out;
    }
    for (i = 0; i < num; i++) {
    node = &grp.nodes[i];
    scnprintf(node.name, sizeof(node.name), grp.format,
    i + 1);
    node.sda.index = (grp.option << 16) | (i & 0xffff);
    node.sda.dev_attr.show = grp.show;
    node.sda.dev_attr.store = grp.store;
    attr = &node.sda.dev_attr.attr;
    sysfs_attr_init(attr);
    attr.name = node.name;
    attr.mode = 0444 | (grp.store ? 0200 : 0);
    ret = sysfs_create_file(&pdev.dev.kobj, attr);
    if (ret) {
    attr.name = core::ptr::null_mut();
    goto out;
    }
    }
    }
    return 0;
    out:
    applesmc_destroy_nodes(groups);
    return ret;
    }
// Create accelerometer resources
#[no_mangle]
unsafe extern "C" fn applesmc_create_accelerometer() -> c_int {
    static int applesmc_create_accelerometer(void)
    {
    int ret;
    if (!smcreg.has_accelerometer)
    return 0;
    ret = applesmc_create_nodes(accelerometer_group, 1);
    if (ret)
    goto out;
    applesmc_idev = input_allocate_device();
    if (!applesmc_idev) {
    ret = -ENOMEM;
    goto out_sysfs;
    }
// initial calibrate for the input device
    applesmc_calibrate();
// initialize the input device
    applesmc_idev.name = "applesmc";
    applesmc_idev.id.bustype = BUS_HOST;
    applesmc_idev.dev.parent = &pdev.dev;
    input_set_abs_params(applesmc_idev, ABS_X,
    -256, 256, APPLESMC_INPUT_FUZZ, APPLESMC_INPUT_FLAT);
    input_set_abs_params(applesmc_idev, ABS_Y,
    -256, 256, APPLESMC_INPUT_FUZZ, APPLESMC_INPUT_FLAT);
    ret = input_setup_polling(applesmc_idev, applesmc_idev_poll);
    if (ret)
    goto out_idev;
    input_set_poll_interval(applesmc_idev, APPLESMC_POLL_INTERVAL);
    ret = input_register_device(applesmc_idev);
    if (ret)
    goto out_idev;
    return 0;
    out_idev:
    input_free_device(applesmc_idev);
    out_sysfs:
    applesmc_destroy_nodes(accelerometer_group);
    out:
    pr_warn("driver init failed (ret=%d)!\n", ret);
    return ret;
    }
// Release all resources used by the accelerometer
#[no_mangle]
unsafe extern "C" fn applesmc_release_accelerometer() {
    static void applesmc_release_accelerometer(void)
    {
    if (!smcreg.has_accelerometer)
    return;
    input_unregister_device(applesmc_idev);
    applesmc_destroy_nodes(accelerometer_group);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_create_light_sensor() -> c_int {
    static int applesmc_create_light_sensor(void)
    {
    if (!smcreg.num_light_sensors)
    return 0;
    return applesmc_create_nodes(light_sensor_group, 1);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_release_light_sensor() {
    static void applesmc_release_light_sensor(void)
    {
    if (!smcreg.num_light_sensors)
    return;
    applesmc_destroy_nodes(light_sensor_group);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_create_key_backlight() -> c_int {
    static int applesmc_create_key_backlight(void)
    {
    if (!smcreg.has_key_backlight)
    return 0;
    applesmc_led_wq = create_singlethread_workqueue("applesmc-led");
    if (!applesmc_led_wq)
    return -ENOMEM;
    return led_classdev_register(&pdev.dev, &applesmc_backlight);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_release_key_backlight() {
    static void applesmc_release_key_backlight(void)
    {
    if (!smcreg.has_key_backlight)
    return;
    led_classdev_unregister(&applesmc_backlight);
    destroy_workqueue(applesmc_led_wq);
    }
#[no_mangle]
unsafe extern "C" fn applesmc_dmi_match(id: *const dmi_system_id) -> c_int {
    static int applesmc_dmi_match(const struct dmi_system_id *id)
    {
    return 1;
    }
//
// Note that DMI_MATCH(...,"MacBook") will match "MacBookPro1,1".
// So we need to put "Apple MacBook Pro" before "Apple MacBook".
//
    static const struct dmi_system_id applesmc_whitelist[] __initconst = {
    { applesmc_dmi_match, "Apple MacBook Air", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MacBookAir") },
    },
    { applesmc_dmi_match, "Apple MacBook Pro", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MacBookPro") },
    },
    { applesmc_dmi_match, "Apple MacBook", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MacBook") },
    },
    { applesmc_dmi_match, "Apple Macmini", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Macmini") },
    },
    { applesmc_dmi_match, "Apple MacPro", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MacPro") },
    },
    { applesmc_dmi_match, "Apple iMac", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "iMac") },
    },
    { applesmc_dmi_match, "Apple Xserve", {
    DMI_MATCH(DMI_BOARD_VENDOR, "Apple"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Xserve") },
    },
    { .ident = core::ptr::null_mut() }
    };
    MODULE_DEVICE_TABLE(dmi, applesmc_whitelist);
    static struct applesmc_dev_attr *fan_safe_attrs;
    static struct attribute **fan_safe_attr_list;
    static struct attribute_group fan_safe_group;
    static const struct attribute_group *applesmc_extra_groups[2];
    static u32 *applesmc_temp_config;
    static u32 *applesmc_fan_config;
    static u32 *applesmc_pwm_config;
    static struct hwmon_channel_info *applesmc_info_temp;
    static struct hwmon_channel_info *applesmc_info_fan;
    static struct hwmon_channel_info *applesmc_info_pwm;
    static const struct hwmon_channel_info **applesmc_info_arr;
    static struct hwmon_chip_info *applesmc_chip;
#[no_mangle]
unsafe extern "C" fn applesmc_free_hwmon() {
    static void applesmc_free_hwmon(void)
    {
    kfree(applesmc_temp_config);
    kfree(applesmc_fan_config);
    kfree(applesmc_pwm_config);
    kfree(applesmc_info_temp);
    kfree(applesmc_info_fan);
    kfree(applesmc_info_pwm);
    kfree(applesmc_info_arr);
    kfree(applesmc_chip);
    kfree(fan_safe_attrs);
    kfree(fan_safe_attr_list);
    }
    static umode_t applesmc_hwmon_is_visible(const void *drvdata, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    if (attr == hwmon_temp_input || attr == hwmon_temp_label)
    return 0444;
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    case hwmon_fan_label:
    case hwmon_fan_max:
    return 0444;
    case hwmon_fan_min:
    case hwmon_fan_target:
    return 0644;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    if (attr == hwmon_pwm_enable)
    return 0644;
    break;
    default:
    break;
    }
    return 0;
    }
    static int applesmc_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    int ret;
    switch (type) {
    case hwmon_temp:
    if (attr == hwmon_temp_input) {
    const char *key = smcreg.index[channel];
    s16 value;
    ret = applesmc_read_s16(key, &value);
    if (ret)
    return ret;
// val = 250 * (value >> 6);
    return 0;
    }
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input: {
    char key[5];
    u8 buffer[2];
    scnprintf(key, sizeof(key), "F%dAc", channel);
    ret = applesmc_read_key(key, buffer, 2);
    if (ret)
    return ret;
// val = ((buffer[0] << 8 | buffer[1]) >> 2);
    return 0;
    }
    case hwmon_fan_min: {
    char key[5];
    u8 buffer[2];
    scnprintf(key, sizeof(key), "F%dMn", channel);
    ret = applesmc_read_key(key, buffer, 2);
    if (ret)
    return ret;
// val = ((buffer[0] << 8 | buffer[1]) >> 2);
    return 0;
    }
    case hwmon_fan_max: {
    char key[5];
    u8 buffer[2];
    scnprintf(key, sizeof(key), "F%dMx", channel);
    ret = applesmc_read_key(key, buffer, 2);
    if (ret)
    return ret;
// val = ((buffer[0] << 8 | buffer[1]) >> 2);
    return 0;
    }
    case hwmon_fan_target: {
    char key[5];
    u8 buffer[2];
    scnprintf(key, sizeof(key), "F%dTg", channel);
    ret = applesmc_read_key(key, buffer, 2);
    if (ret)
    return ret;
// val = ((buffer[0] << 8 | buffer[1]) >> 2);
    return 0;
    }
    default:
    break;
    }
    break;
    case hwmon_pwm:
    if (attr == hwmon_pwm_enable) {
    u8 buffer[2];
    ret = applesmc_read_key(FANS_MANUAL, buffer, 2);
    if (ret)
    return ret;
// val = (((buffer[0] << 8 | buffer[1]) >> channel) & 0x01) ? 1 : 2;
    return 0;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static int applesmc_hwmon_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    int ret;
    switch (type) {
    case hwmon_fan:
    if (attr == hwmon_fan_min || attr == hwmon_fan_target) {
    char key[5];
    u8 buffer[2];
    const char *fmt = (attr == hwmon_fan_min) ? "F%dMn" : "F%dTg";
    if (val < 0 || val >= 0x4000)
    return -EINVAL;
    scnprintf(key, sizeof(key), fmt, channel);
    buffer[0] = (val >> 6) & 0xff;
    buffer[1] = (val << 2) & 0xff;
    return applesmc_write_key(key, buffer, 2);
    }
    break;
    case hwmon_pwm:
    if (attr == hwmon_pwm_enable) {
    u8 buffer[2];
    u16 manual_val;
    const struct applesmc_entry *entry;
    if (val != 1 && val != 2)
    return -EINVAL;
    entry = applesmc_get_entry_by_key(FANS_MANUAL);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    mutex_lock(&smcreg.mutex);
    ret = read_smc(APPLESMC_READ_CMD, entry.key, buffer, 2);
    if (ret)
    goto out_unlock;
    manual_val = (buffer[0] << 8 | buffer[1]);
    if (val == 1)
    manual_val |= (0x01 << channel);
    else
    manual_val &= ~(0x01 << channel);
    buffer[0] = (manual_val >> 8) & 0xff;
    buffer[1] = manual_val & 0xff;
    ret = write_smc(APPLESMC_WRITE_CMD, entry.key, buffer, 2);
    out_unlock:
    mutex_unlock(&smcreg.mutex);
    return ret;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static int applesmc_hwmon_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    switch (type) {
    case hwmon_temp:
    if (attr == hwmon_temp_label) {
// str = smcreg.index[channel];
    return 0;
    }
    break;
    case hwmon_fan:
    if (attr == hwmon_fan_label) {
// str = smcreg.fan_positions[channel] + 4;
    return 0;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static const struct hwmon_ops applesmc_hwmon_ops = {
    .is_visible = applesmc_hwmon_is_visible,
    .read = applesmc_hwmon_read,
    .write = applesmc_hwmon_write,
    .read_string = applesmc_hwmon_read_string,
    };
#[no_mangle]
unsafe extern "C" fn applesmc_init() -> int __init {
    static int __init applesmc_init(void)
    {
    int i;
    int ret;
    if (!dmi_check_system(applesmc_whitelist)) {
    pr_warn("supported laptop not found!\n");
    ret = -ENODEV;
    goto out;
    }
    if (!request_region(APPLESMC_DATA_PORT, APPLESMC_NR_PORTS,
    "applesmc")) {
    ret = -ENXIO;
    goto out;
    }
    ret = platform_driver_register(&applesmc_driver);
    if (ret)
    goto out_region;
    pdev = platform_device_register_simple("applesmc", APPLESMC_DATA_PORT,
    core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    ret = PTR_ERR(pdev);
    goto out_driver;
    }
// create register cache
    ret = applesmc_init_smcreg();
    if (ret)
    goto out_device;
    ret = applesmc_create_nodes(info_group, 1);
    if (ret)
    goto out_smcreg;
// allocate hwmon channel configs
    applesmc_temp_config = kcalloc(smcreg.index_count + 1,
    sizeof(*applesmc_temp_config), GFP_KERNEL);
    applesmc_fan_config = kcalloc(smcreg.fan_count + 1,
    sizeof(*applesmc_fan_config), GFP_KERNEL);
    applesmc_pwm_config = kcalloc(smcreg.fan_count + 1,
    sizeof(*applesmc_pwm_config), GFP_KERNEL);
    if (!applesmc_temp_config || !applesmc_fan_config || !applesmc_pwm_config) {
    ret = -ENOMEM;
    goto out_info;
    }
    for (i = 0; i < smcreg.index_count; i++)
    applesmc_temp_config[i] = HWMON_T_INPUT | HWMON_T_LABEL;
    applesmc_temp_config[smcreg.index_count] = 0;
    for (i = 0; i < smcreg.fan_count; i++) {
    applesmc_fan_config[i] = HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_MIN |
    HWMON_F_MAX | HWMON_F_TARGET;
    applesmc_pwm_config[i] = HWMON_PWM_ENABLE;
    }
    applesmc_fan_config[smcreg.fan_count] = 0;
    applesmc_pwm_config[smcreg.fan_count] = 0;
    applesmc_info_temp = kzalloc_obj(*applesmc_info_temp);
    applesmc_info_fan = kzalloc_obj(*applesmc_info_fan);
    applesmc_info_pwm = kzalloc_obj(*applesmc_info_pwm);
    if (!applesmc_info_temp || !applesmc_info_fan || !applesmc_info_pwm) {
    ret = -ENOMEM;
    goto out_info;
    }
    applesmc_info_temp.type = hwmon_temp;
    applesmc_info_temp.config = applesmc_temp_config;
    applesmc_info_fan.type = hwmon_fan;
    applesmc_info_fan.config = applesmc_fan_config;
    applesmc_info_pwm.type = hwmon_pwm;
    applesmc_info_pwm.config = applesmc_pwm_config;
    applesmc_info_arr = kzalloc_objs(*applesmc_info_arr, 4);
    if (!applesmc_info_arr) {
    ret = -ENOMEM;
    goto out_info;
    }
    applesmc_info_arr[0] = applesmc_info_temp;
    applesmc_info_arr[1] = applesmc_info_fan;
    applesmc_info_arr[2] = applesmc_info_pwm;
    applesmc_info_arr[3] = core::ptr::null_mut();
    applesmc_chip = kzalloc_obj(*applesmc_chip);
    if (!applesmc_chip) {
    ret = -ENOMEM;
    goto out_info;
    }
    applesmc_chip.ops = &applesmc_hwmon_ops;
    applesmc_chip.info = applesmc_info_arr;
// Create non-standard fanX_safe attributes group
    fan_safe_attrs = kzalloc_objs(*fan_safe_attrs, smcreg.fan_count);
    fan_safe_attr_list = kzalloc_objs(*fan_safe_attr_list,
    smcreg.fan_count + 1);
    if (!fan_safe_attrs || !fan_safe_attr_list) {
    ret = -ENOMEM;
    goto out_info;
    }
    for (i = 0; i < smcreg.fan_count; i++) {
    struct applesmc_dev_attr *node = &fan_safe_attrs[i];
    scnprintf(node.name, sizeof(node.name), "fan%d_safe", i + 1);
    node.sda.index = (3 << 16) | (i & 0xffff); /* Option 3 (safe speed) */
    node.sda.dev_attr.show = applesmc_show_fan_speed;
    node.sda.dev_attr.store = core::ptr::null_mut();
    sysfs_attr_init(&node.sda.dev_attr.attr);
    node.sda.dev_attr.attr.name = node.name;
    node.sda.dev_attr.attr.mode = 0444;
    fan_safe_attr_list[i] = &node.sda.dev_attr.attr;
    }
    fan_safe_attr_list[smcreg.fan_count] = core::ptr::null_mut();
    fan_safe_group.attrs = fan_safe_attr_list;
    applesmc_extra_groups[0] = &fan_safe_group;
    applesmc_extra_groups[1] = core::ptr::null_mut();
    ret = applesmc_create_accelerometer();
    if (ret)
    goto out_info;
    ret = applesmc_create_light_sensor();
    if (ret)
    goto out_accelerometer;
    ret = applesmc_create_key_backlight();
    if (ret)
    goto out_light_sysfs;
    hwmon_dev = hwmon_device_register_with_info(&pdev.dev, "applesmc", core::ptr::null_mut(),
    applesmc_chip, applesmc_extra_groups);
    if (IS_ERR(hwmon_dev)) {
    ret = PTR_ERR(hwmon_dev);
    goto out_light_ledclass;
    }
    return 0;
    out_light_ledclass:
    applesmc_release_key_backlight();
    out_light_sysfs:
    applesmc_release_light_sensor();
    out_accelerometer:
    applesmc_release_accelerometer();
    out_info:
    applesmc_free_hwmon();
    applesmc_destroy_nodes(info_group);
    out_smcreg:
    applesmc_destroy_smcreg();
    out_device:
    platform_device_unregister(pdev);
    out_driver:
    platform_driver_unregister(&applesmc_driver);
    out_region:
    release_region(APPLESMC_DATA_PORT, APPLESMC_NR_PORTS);
    out:
    pr_warn("driver init failed (ret=%d)!\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn applesmc_exit() -> void __exit {
    static void __exit applesmc_exit(void)
    {
    hwmon_device_unregister(hwmon_dev);
    applesmc_release_key_backlight();
    applesmc_release_light_sensor();
    applesmc_release_accelerometer();
    applesmc_destroy_nodes(info_group);
    applesmc_destroy_smcreg();
    platform_device_unregister(pdev);
    platform_driver_unregister(&applesmc_driver);
    release_region(APPLESMC_DATA_PORT, APPLESMC_NR_PORTS);
    applesmc_free_hwmon();
    }
    module_init(applesmc_init);
    module_exit(applesmc_exit);
    MODULE_AUTHOR("Nicolas Boichat");
    MODULE_DESCRIPTION("Apple SMC");
    MODULE_LICENSE("GPL v2");
