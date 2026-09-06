//! Automatically rewritten from C to Rust
//! Source: sound/i2c/i2c.c
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
// Generic i2c interface for ALSA
//
// (c) 1998 Gerd Knorr <kraxel@cs.tu-berlin.de>
// Modified for the ALSA driver by Jaroslav Kysela <perex@perex.cz>
//

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("Generic i2c interface for ALSA");
    MODULE_LICENSE("GPL");
    static int snd_i2c_bit_sendbytes(struct snd_i2c_device *device,
    unsigned char *bytes, int count);
    static int snd_i2c_bit_readbytes(struct snd_i2c_device *device,
    unsigned char *bytes, int count);
    static int snd_i2c_bit_probeaddr(struct snd_i2c_bus *bus,
    unsigned short addr);
    static const struct snd_i2c_ops snd_i2c_bit_ops = {
    .sendbytes = snd_i2c_bit_sendbytes,
    .readbytes = snd_i2c_bit_readbytes,
    .probeaddr = snd_i2c_bit_probeaddr,
    };
#[no_mangle]
unsafe extern "C" fn snd_i2c_bus_free(bus: *mut snd_i2c_bus) -> c_int {
    static int snd_i2c_bus_free(struct snd_i2c_bus *bus)
    {
    struct snd_i2c_bus *slave;
    struct snd_i2c_device *device;
    if (snd_BUG_ON(!bus))
    return -EINVAL;
    while (!list_empty(&bus.devices)) {
    device = snd_i2c_device(bus.devices.next);
    snd_i2c_device_free(device);
    }
    if (bus.master)
    list_del(&bus.buses);
    else {
    while (!list_empty(&bus.buses)) {
    slave = snd_i2c_slave_bus(bus.buses.next);
    snd_device_free(bus.card, slave);
    }
    }
    if (bus.private_free)
    bus.private_free(bus);
    kfree(bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bus_dev_free(device: *mut snd_device) -> c_int {
    static int snd_i2c_bus_dev_free(struct snd_device *device)
    {
    struct snd_i2c_bus *bus = device.device_data;
    return snd_i2c_bus_free(bus);
    }
    int snd_i2c_bus_create(struct snd_card *card, const char *name,
    struct snd_i2c_bus *master, struct snd_i2c_bus **ri2c)
    {
    struct snd_i2c_bus *bus;
    int err;
    static const struct snd_device_ops ops = {
    .dev_free =	snd_i2c_bus_dev_free,
    };
// ri2c = NULL;
    bus = kzalloc_obj(*bus);
    if (bus == core::ptr::null_mut())
    return -ENOMEM;
    mutex_init(&bus.lock_mutex);
    INIT_LIST_HEAD(&bus.devices);
    INIT_LIST_HEAD(&bus.buses);
    bus.card = card;
    bus.ops = &snd_i2c_bit_ops;
    if (master) {
    list_add_tail(&bus.buses, &master.buses);
    bus.master = master;
    }
    strscpy(bus.name, name, sizeof(bus.name));
    err = snd_device_new(card, SNDRV_DEV_BUS, bus, &ops);
    if (err < 0) {
    snd_i2c_bus_free(bus);
    return err;
    }
// ri2c = bus;
    return 0;
    }
    EXPORT_SYMBOL(snd_i2c_bus_create);
    int snd_i2c_device_create(struct snd_i2c_bus *bus, const char *name,
    unsigned char addr, struct snd_i2c_device **rdevice)
    {
    struct snd_i2c_device *device;
// rdevice = NULL;
    if (snd_BUG_ON(!bus))
    return -EINVAL;
    device = kzalloc_obj(*device);
    if (device == core::ptr::null_mut())
    return -ENOMEM;
    device.addr = addr;
    strscpy(device.name, name, sizeof(device.name));
    list_add_tail(&device.list, &bus.devices);
    device.bus = bus;
// rdevice = device;
    return 0;
    }
    EXPORT_SYMBOL(snd_i2c_device_create);
#[no_mangle]
pub unsafe extern "C" fn snd_i2c_device_free(device: *mut snd_i2c_device) -> c_int {
    int snd_i2c_device_free(struct snd_i2c_device *device)
    {
    if (device.bus)
    list_del(&device.list);
    if (device.private_free)
    device.private_free(device);
    kfree(device);
    return 0;
    }
    EXPORT_SYMBOL(snd_i2c_device_free);
#[no_mangle]
pub unsafe extern "C" fn snd_i2c_sendbytes(device: *mut snd_i2c_device, bytes: *mut c_uchar, count: c_int) -> c_int {
    int snd_i2c_sendbytes(struct snd_i2c_device *device, unsigned char *bytes, int count)
    {
    return device.bus.ops.sendbytes(device, bytes, count);
    }
    EXPORT_SYMBOL(snd_i2c_sendbytes);
#[no_mangle]
pub unsafe extern "C" fn snd_i2c_readbytes(device: *mut snd_i2c_device, bytes: *mut c_uchar, count: c_int) -> c_int {
    int snd_i2c_readbytes(struct snd_i2c_device *device, unsigned char *bytes, int count)
    {
    return device.bus.ops.readbytes(device, bytes, count);
    }
    EXPORT_SYMBOL(snd_i2c_readbytes);
#[no_mangle]
pub unsafe extern "C" fn snd_i2c_probeaddr(bus: *mut snd_i2c_bus, addr: c_ushort) -> c_int {
    int snd_i2c_probeaddr(struct snd_i2c_bus *bus, unsigned short addr)
    {
    return bus.ops.probeaddr(bus, addr);
    }
    EXPORT_SYMBOL(snd_i2c_probeaddr);
//
// bit-operations
//
#[no_mangle]
pub unsafe extern "C" fn snd_i2c_bit_hw_start(bus: *mut snd_i2c_bus) {
    static inline void snd_i2c_bit_hw_start(struct snd_i2c_bus *bus)
    {
    if (bus.hw_ops.bit.start)
    bus.hw_ops.bit.start(bus);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_i2c_bit_hw_stop(bus: *mut snd_i2c_bus) {
    static inline void snd_i2c_bit_hw_stop(struct snd_i2c_bus *bus)
    {
    if (bus.hw_ops.bit.stop)
    bus.hw_ops.bit.stop(bus);
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_direction(bus: *mut snd_i2c_bus, clock: c_int, data: c_int) {
    static void snd_i2c_bit_direction(struct snd_i2c_bus *bus, int clock, int data)
    {
    if (bus.hw_ops.bit.direction)
    bus.hw_ops.bit.direction(bus, clock, data);
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_set(bus: *mut snd_i2c_bus, clock: c_int, data: c_int) {
    static void snd_i2c_bit_set(struct snd_i2c_bus *bus, int clock, int data)
    {
    bus.hw_ops.bit.setlines(bus, clock, data);
    }

#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_clock(bus: *mut snd_i2c_bus) -> c_int {
    static int snd_i2c_bit_clock(struct snd_i2c_bus *bus)
    {
    if (bus.hw_ops.bit.getclock)
    return bus.hw_ops.bit.getclock(bus);
    return -ENXIO;
    }

#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_data(bus: *mut snd_i2c_bus, ack: c_int) -> c_int {
    static int snd_i2c_bit_data(struct snd_i2c_bus *bus, int ack)
    {
    return bus.hw_ops.bit.getdata(bus, ack);
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_start(bus: *mut snd_i2c_bus) {
    static void snd_i2c_bit_start(struct snd_i2c_bus *bus)
    {
    snd_i2c_bit_hw_start(bus);
    snd_i2c_bit_direction(bus, 1, 1);	/* SCL - wr, SDA - wr */
    snd_i2c_bit_set(bus, 1, 1);
    snd_i2c_bit_set(bus, 1, 0);
    snd_i2c_bit_set(bus, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_stop(bus: *mut snd_i2c_bus) {
    static void snd_i2c_bit_stop(struct snd_i2c_bus *bus)
    {
    snd_i2c_bit_set(bus, 0, 0);
    snd_i2c_bit_set(bus, 1, 0);
    snd_i2c_bit_set(bus, 1, 1);
    snd_i2c_bit_hw_stop(bus);
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_send(bus: *mut snd_i2c_bus, data: c_int) {
    static void snd_i2c_bit_send(struct snd_i2c_bus *bus, int data)
    {
    snd_i2c_bit_set(bus, 0, data);
    snd_i2c_bit_set(bus, 1, data);
    snd_i2c_bit_set(bus, 0, data);
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_ack(bus: *mut snd_i2c_bus) -> c_int {
    static int snd_i2c_bit_ack(struct snd_i2c_bus *bus)
    {
    int ack;
    snd_i2c_bit_set(bus, 0, 1);
    snd_i2c_bit_set(bus, 1, 1);
    snd_i2c_bit_direction(bus, 1, 0);	/* SCL - wr, SDA - rd */
    ack = snd_i2c_bit_data(bus, 1);
    snd_i2c_bit_direction(bus, 1, 1);	/* SCL - wr, SDA - wr */
    snd_i2c_bit_set(bus, 0, 1);
    return ack ? -EIO : 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_sendbyte(bus: *mut snd_i2c_bus, data: c_uchar) -> c_int {
    static int snd_i2c_bit_sendbyte(struct snd_i2c_bus *bus, unsigned char data)
    {
    int i, err;
    for (i = 7; i >= 0; i--)
    snd_i2c_bit_send(bus, !!(data & (1 << i)));
    err = snd_i2c_bit_ack(bus);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_readbyte(bus: *mut snd_i2c_bus, last: c_int) -> c_int {
    static int snd_i2c_bit_readbyte(struct snd_i2c_bus *bus, int last)
    {
    int i;
    let mut data: c_uchar = 0;
    snd_i2c_bit_set(bus, 0, 1);
    snd_i2c_bit_direction(bus, 1, 0);	/* SCL - wr, SDA - rd */
    for (i = 7; i >= 0; i--) {
    snd_i2c_bit_set(bus, 1, 1);
    if (snd_i2c_bit_data(bus, 0))
    data |= (1 << i);
    snd_i2c_bit_set(bus, 0, 1);
    }
    snd_i2c_bit_direction(bus, 1, 1);	/* SCL - wr, SDA - wr */
    snd_i2c_bit_send(bus, !!last);
    return data;
    }
    static int snd_i2c_bit_sendbytes(struct snd_i2c_device *device,
    unsigned char *bytes, int count)
    {
    struct snd_i2c_bus *bus = device.bus;
    int err, res = 0;
    if (device.flags & SND_I2C_DEVICE_ADDRTEN)
    return -EIO;		/* not yet implemented */
    snd_i2c_bit_start(bus);
    err = snd_i2c_bit_sendbyte(bus, device.addr << 1);
    if (err < 0) {
    snd_i2c_bit_hw_stop(bus);
    return err;
    }
    while (count-- > 0) {
    err = snd_i2c_bit_sendbyte(bus, *bytes++);
    if (err < 0) {
    snd_i2c_bit_hw_stop(bus);
    return err;
    }
    res++;
    }
    snd_i2c_bit_stop(bus);
    return res;
    }
    static int snd_i2c_bit_readbytes(struct snd_i2c_device *device,
    unsigned char *bytes, int count)
    {
    struct snd_i2c_bus *bus = device.bus;
    int err, res = 0;
    if (device.flags & SND_I2C_DEVICE_ADDRTEN)
    return -EIO;		/* not yet implemented */
    snd_i2c_bit_start(bus);
    err = snd_i2c_bit_sendbyte(bus, (device.addr << 1) | 1);
    if (err < 0) {
    snd_i2c_bit_hw_stop(bus);
    return err;
    }
    while (count-- > 0) {
    err = snd_i2c_bit_readbyte(bus, count == 0);
    if (err < 0) {
    snd_i2c_bit_hw_stop(bus);
    return err;
    }
// bytes++ = (unsigned char)err;
    res++;
    }
    snd_i2c_bit_stop(bus);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn snd_i2c_bit_probeaddr(bus: *mut snd_i2c_bus, addr: c_ushort) -> c_int {
    static int snd_i2c_bit_probeaddr(struct snd_i2c_bus *bus, unsigned short addr)
    {
    int err;
    if (addr & 0x8000)	/* 10-bit address */
    return -EIO;	/* not yet implemented */
    if (addr & 0x7f80)	/* invalid address */
    return -EINVAL;
    snd_i2c_bit_start(bus);
    err = snd_i2c_bit_sendbyte(bus, addr << 1);
    snd_i2c_bit_stop(bus);
    return err;
    }
