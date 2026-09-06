//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/qt2160.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// qt2160.c - Atmel AT42QT2160 Touch Sense Controller
//
// Copyright (C) 2009 Raphael Derosso Pereira <raphaelpereira@gmail.com>
//

pub const QT2160_VALID_CHIPID: c_uint = 0x11;
pub const QT2160_CMD_CHIPID: c_int = 0;
pub const QT2160_CMD_CODEVER: c_int = 1;
pub const QT2160_CMD_GSTAT: c_int = 2;
pub const QT2160_CMD_KEYS3: c_int = 3;
pub const QT2160_CMD_KEYS4: c_int = 4;
pub const QT2160_CMD_SLIDE: c_int = 5;
pub const QT2160_CMD_GPIOS: c_int = 6;
pub const QT2160_CMD_SUBVER: c_int = 7;
pub const QT2160_CMD_CALIBRATE: c_int = 10;
pub const QT2160_CMD_DRIVE_X: c_int = 70;
pub const QT2160_CMD_PWMEN_X: c_int = 74;
pub const QT2160_CMD_PWM_DUTY: c_int = 76;
pub const QT2160_NUM_LEDS_X: c_int = 8;

    static unsigned char qt2160_key2code[] = {
    KEY_0, KEY_1, KEY_2, KEY_3,
    KEY_4, KEY_5, KEY_6, KEY_7,
    KEY_8, KEY_9, KEY_A, KEY_B,
    KEY_C, KEY_D, KEY_E, KEY_F,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt2160_led {
    pub qt2160: *mut qt2160_data,
    pub cdev: led_classdev,
    pub name: [c_char; 32],
    pub id: c_int,
    pub brightness: enum led_brightness,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt2160_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub keycodes: [c_ushort; ARRAY_SIZE(qt2160_key2code)],
    pub key_matrix: u16,
    pub leds: [qt2160_led; QT2160_NUM_LEDS_X],
}

    static int qt2160_read(struct i2c_client *client, u8 reg);
    static int qt2160_write(struct i2c_client *client, u8 reg, u8 data);

    static int qt2160_led_set(struct led_classdev *cdev,
    enum led_brightness value)
    {
    struct qt2160_led *led = container_of(cdev, struct qt2160_led, cdev);
    struct qt2160_data *qt2160 = led.qt2160;
    struct i2c_client *client = qt2160.client;
    u32 drive, pwmen;
    if (value != led.brightness) {
    drive = qt2160_read(client, QT2160_CMD_DRIVE_X);
    pwmen = qt2160_read(client, QT2160_CMD_PWMEN_X);
    if (value != LED_OFF) {
    drive |= BIT(led.id);
    pwmen |= BIT(led.id);
    } else {
    drive &= ~BIT(led.id);
    pwmen &= ~BIT(led.id);
    }
    qt2160_write(client, QT2160_CMD_DRIVE_X, drive);
    qt2160_write(client, QT2160_CMD_PWMEN_X, pwmen);
//
// Changing this register will change the brightness
// of every LED in the qt2160. It's a HW limitation.
//
    if (value != LED_OFF)
    qt2160_write(client, QT2160_CMD_PWM_DUTY, value);
    led.brightness = value;
    }
    return 0;
    }

    static int qt2160_read_block(struct i2c_client *client,
    u8 inireg, u8 *buffer, unsigned int count)
    {
    int error, idx = 0;
//
// Can't use SMBus block data read. Check for I2C functionality to speed
// things up whenever possible. Otherwise we will be forced to read
// sequentially.
//
    if (i2c_check_functionality(client.adapter, I2C_FUNC_I2C))	{
    error = i2c_smbus_write_byte(client, inireg + idx);
    if (error) {
    dev_err(&client.dev,
    "couldn't send request. Returned %d\n", error);
    return error;
    }
    error = i2c_master_recv(client, buffer, count);
    if (error != count) {
    dev_err(&client.dev,
    "couldn't read registers. Returned %d bytes\n", error);
    return error;
    }
    } else {
    while (count--) {
    int data;
    error = i2c_smbus_write_byte(client, inireg + idx);
    if (error) {
    dev_err(&client.dev,
    "couldn't send request. Returned %d\n", error);
    return error;
    }
    data = i2c_smbus_read_byte(client);
    if (data < 0) {
    dev_err(&client.dev,
    "couldn't read register. Returned %d\n", data);
    return data;
    }
    buffer[idx++] = data;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt2160_get_key_matrix(input: *mut input_dev) {
    static void qt2160_get_key_matrix(struct input_dev *input)
    {
    struct qt2160_data *qt2160 = input_get_drvdata(input);
    struct i2c_client *client = qt2160.client;
    u8 regs[6];
    u16 old_matrix, new_matrix;
    int ret, i, mask;
    dev_dbg(&client.dev, "requesting keys...\n");
//
// Read all registers from General Status Register
// to GPIOs register
//
    ret = qt2160_read_block(client, QT2160_CMD_GSTAT, regs, 6);
    if (ret) {
    dev_err(&client.dev,
    "could not perform chip read.\n");
    return;
    }
    old_matrix = qt2160.key_matrix;
    qt2160.key_matrix = new_matrix = (regs[2] << 8) | regs[1];
    mask = 0x01;
    for (i = 0; i < 16; ++i, mask <<= 1) {
    let mut keyval: c_int = new_matrix & mask;
    if ((old_matrix & mask) != keyval) {
    input_report_key(input, qt2160.keycodes[i], keyval);
    dev_dbg(&client.dev, "key %d %s\n",
    i, keyval ? "pressed" : "released");
    }
    }
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn qt2160_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qt2160_irq(int irq, void *data)
    {
    struct input_dev *input = data;
    qt2160_get_key_matrix(input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qt2160_read(client: *mut i2c_client, reg: u8) -> c_int {
    static int qt2160_read(struct i2c_client *client, u8 reg)
    {
    int ret;
    ret = i2c_smbus_write_byte(client, reg);
    if (ret) {
    dev_err(&client.dev,
    "couldn't send request. Returned %d\n", ret);
    return ret;
    }
    ret = i2c_smbus_read_byte(client);
    if (ret < 0) {
    dev_err(&client.dev,
    "couldn't read register. Returned %d\n", ret);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qt2160_write(client: *mut i2c_client, reg: u8, data: u8) -> c_int {
    static int qt2160_write(struct i2c_client *client, u8 reg, u8 data)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(client, reg, data);
    if (ret < 0)
    dev_err(&client.dev,
    "couldn't write data. Returned %d\n", ret);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn qt2160_register_leds(qt2160: *mut qt2160_data) -> c_int {
    static int qt2160_register_leds(struct qt2160_data *qt2160)
    {
    struct i2c_client *client = qt2160.client;
    int error;
    int i;
    for (i = 0; i < QT2160_NUM_LEDS_X; i++) {
    struct qt2160_led *led = &qt2160.leds[i];
    snprintf(led.name, sizeof(led.name), "qt2160:x%d", i);
    led.cdev.name = led.name;
    led.cdev.brightness_set_blocking = qt2160_led_set;
    led.cdev.brightness = LED_OFF;
    led.id = i;
    led.qt2160 = qt2160;
    error = devm_led_classdev_register(&client.dev, &led.cdev);
    if (error)
    return error;
    }
// Tur off LEDs
    qt2160_write(client, QT2160_CMD_DRIVE_X, 0);
    qt2160_write(client, QT2160_CMD_PWMEN_X, 0);
    qt2160_write(client, QT2160_CMD_PWM_DUTY, 0);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn qt2160_register_leds(qt2160: *mut qt2160_data) -> c_int {
    static inline int qt2160_register_leds(struct qt2160_data *qt2160)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn qt2160_identify(client: *mut i2c_client) -> bool {
    static bool qt2160_identify(struct i2c_client *client)
    {
    int id, ver, rev;
// Read Chid ID to check if chip is valid
    id = qt2160_read(client, QT2160_CMD_CHIPID);
    if (id != QT2160_VALID_CHIPID) {
    dev_err(&client.dev, "ID %d not supported\n", id);
    return false;
    }
// Read chip firmware version
    ver = qt2160_read(client, QT2160_CMD_CODEVER);
    if (ver < 0) {
    dev_err(&client.dev, "could not get firmware version\n");
    return false;
    }
// Read chip firmware revision
    rev = qt2160_read(client, QT2160_CMD_SUBVER);
    if (rev < 0) {
    dev_err(&client.dev, "could not get firmware revision\n");
    return false;
    }
    dev_info(&client.dev, "AT42QT2160 firmware version %d.%d.%d\n",
    ver >> 4, ver & 0xf, rev);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn qt2160_probe(client: *mut i2c_client) -> c_int {
    static int qt2160_probe(struct i2c_client *client)
    {
    struct qt2160_data *qt2160;
    struct input_dev *input;
    int i;
    int error;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE)) {
    dev_err(&client.dev, "%s adapter not supported\n",
    dev_driver_string(&client.adapter.dev));
    return -ENODEV;
    }
    if (!qt2160_identify(client))
    return -ENODEV;
// Chip is valid and active. Allocate structure
    qt2160 = devm_kzalloc(&client.dev, sizeof(*qt2160), GFP_KERNEL);
    if (!qt2160)
    return -ENOMEM;
    input = devm_input_allocate_device(&client.dev);
    if (!input)
    return -ENOMEM;
    qt2160.client = client;
    qt2160.input = input;
    input.name = "AT42QT2160 Touch Sense Keyboard";
    input.id.bustype = BUS_I2C;
    input.keycode = qt2160.keycodes;
    input.keycodesize = sizeof(qt2160.keycodes[0]);
    input.keycodemax = ARRAY_SIZE(qt2160_key2code);
    __set_bit(EV_KEY, input.evbit);
    __clear_bit(EV_REP, input.evbit);
    for (i = 0; i < ARRAY_SIZE(qt2160_key2code); i++) {
    qt2160.keycodes[i] = qt2160_key2code[i];
    __set_bit(qt2160_key2code[i], input.keybit);
    }
    __clear_bit(KEY_RESERVED, input.keybit);
    input_set_drvdata(input, qt2160);
// Calibrate device
    error = qt2160_write(client, QT2160_CMD_CALIBRATE, 1);
    if (error) {
    dev_err(&client.dev, "failed to calibrate device\n");
    return error;
    }
    if (client.irq) {
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), qt2160_irq,
    IRQF_ONESHOT,
    "qt2160", input);
    if (error) {
    dev_err(&client.dev,
    "failed to allocate irq %d\n", client.irq);
    return error;
    }
    } else {
    error = input_setup_polling(input, qt2160_get_key_matrix);
    if (error) {
    dev_err(&client.dev, "Failed to setup polling\n");
    return error;
    }
    input_set_poll_interval(input, QT2160_CYCLE_INTERVAL);
    }
    error = qt2160_register_leds(qt2160);
    if (error) {
    dev_err(&client.dev, "Failed to register leds\n");
    return error;
    }
    error = input_register_device(qt2160.input);
    if (error) {
    dev_err(&client.dev,
    "Failed to register input device\n");
    return error;
    }
    return 0;
    }
    static const struct i2c_device_id qt2160_idtable[] = {
    { .name = "qt2160" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, qt2160_idtable);
    static struct i2c_driver qt2160_driver = {
    .driver = {
    .name	= "qt2160",
    },
    .id_table	= qt2160_idtable,
    .probe		= qt2160_probe,
    };
    module_i2c_driver(qt2160_driver);
    MODULE_AUTHOR("Raphael Derosso Pereira <raphaelpereira@gmail.com>");
    MODULE_DESCRIPTION("Driver for AT42QT2160 Touch Sensor");
    MODULE_LICENSE("GPL");
