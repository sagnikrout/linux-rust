//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-parport.c
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
// ------------------------------------------------------------------------
// i2c-parport.c I2C bus over parallel port
// ------------------------------------------------------------------------
    Copyright (C) 2003-2011 Jean Delvare <jdelvare@suse.de>
    Based on older i2c-philips-par.c driver
    Copyright (C) 1995-2000 Simon G. Vogl
    With some changes from:
    Frodo Looijaard <frodol@dds.nl>
    Kyösti Mälkki <kmalkki@cc.hut.fi>
// ------------------------------------------------------------------------

pub const PORT_DATA: c_int = 0;
pub const PORT_STAT: c_int = 1;
pub const PORT_CTRL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lineop {
    pub val: u8,
    pub port: u8,
    pub inverted: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_parm {
    pub setsda: lineop,
    pub setscl: lineop,
    pub getsda: lineop,
    pub getscl: lineop,
    pub init: lineop,
    pub smbus_alert:1: c_uint,
}

    static const struct adapter_parm adapter_parm[] = {
// type 0: Philips adapter
    {
    .setsda	= { 0x80, PORT_DATA, 1 },
    .setscl	= { 0x08, PORT_CTRL, 0 },
    .getsda	= { 0x80, PORT_STAT, 0 },
    .getscl	= { 0x08, PORT_STAT, 0 },
    },
// type 1: home brew teletext adapter
    {
    .setsda	= { 0x02, PORT_DATA, 0 },
    .setscl	= { 0x01, PORT_DATA, 0 },
    .getsda	= { 0x80, PORT_STAT, 1 },
    },
// type 2: Velleman K8000 adapter
    {
    .setsda	= { 0x02, PORT_CTRL, 1 },
    .setscl	= { 0x08, PORT_CTRL, 1 },
    .getsda	= { 0x10, PORT_STAT, 0 },
    },
// type 3: ELV adapter
    {
    .setsda	= { 0x02, PORT_DATA, 1 },
    .setscl	= { 0x01, PORT_DATA, 1 },
    .getsda	= { 0x40, PORT_STAT, 1 },
    .getscl	= { 0x08, PORT_STAT, 1 },
    },
// type 4: ADM1032 evaluation board
    {
    .setsda	= { 0x02, PORT_DATA, 1 },
    .setscl	= { 0x01, PORT_DATA, 1 },
    .getsda	= { 0x10, PORT_STAT, 1 },
    .init	= { 0xf0, PORT_DATA, 0 },
    .smbus_alert = 1,
    },
// type 5: ADM1025, ADM1030 and ADM1031 evaluation boards
    {
    .setsda	= { 0x02, PORT_DATA, 1 },
    .setscl	= { 0x01, PORT_DATA, 1 },
    .getsda	= { 0x10, PORT_STAT, 1 },
    },
// type 6: Barco LPT->DVI (K5800236) adapter
    {
    .setsda	= { 0x02, PORT_DATA, 1 },
    .setscl	= { 0x01, PORT_DATA, 1 },
    .getsda	= { 0x20, PORT_STAT, 0 },
    .getscl	= { 0x40, PORT_STAT, 0 },
    .init	= { 0xfc, PORT_DATA, 0 },
    },
// type 7: One For All JP1 parallel port adapter
    {
    .setsda	= { 0x01, PORT_DATA, 0 },
    .setscl	= { 0x02, PORT_DATA, 0 },
    .getsda	= { 0x80, PORT_STAT, 1 },
    .init	= { 0x04, PORT_DATA, 1 },
    },
// type 8: VCT-jig
    {
    .setsda	= { 0x04, PORT_DATA, 1 },
    .setscl	= { 0x01, PORT_DATA, 1 },
    .getsda	= { 0x40, PORT_STAT, 0 },
    .getscl	= { 0x80, PORT_STAT, 1 },
    },
    };
// ----- Device list ------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_par {
    pub pdev: *mut pardevice,
    pub adapter: i2c_adapter,
    pub algo_data: i2c_algo_bit_data,
    pub alert_data: i2c_smbus_alert_setup,
    pub ara: *mut i2c_client,
    pub node: list_head,
}

    static LIST_HEAD(adapter_list);
    static DEFINE_MUTEX(adapter_list_lock);
pub const MAX_DEVICE: c_int = 4;
    static int parport[MAX_DEVICE] = {0, -1, -1, -1};
    module_param_array(parport, int, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(parport,
    "List of parallel ports to bind to, by index.\n"
    " At most " __stringify(MAX_DEVICE) " devices are supported.\n"
    " Default is one device connected to parport0.\n"
    );
    let mut type: static int = -1;
    module_param(type, int, 0);
    MODULE_PARM_DESC(type,
    "Type of adapter:\n"
    " 0 = Philips adapter\n"
    " 1 = home brew teletext adapter\n"
    " 2 = Velleman K8000 adapter\n"
    " 3 = ELV adapter\n"
    " 4 = ADM1032 evaluation board\n"
    " 5 = ADM1025, ADM1030 and ADM1031 evaluation boards\n"
    " 6 = Barco LPT.DVI (K5800236) adapter\n"
    " 7 = One For All JP1 parallel port adapter\n"
    " 8 = VCT-jig\n"
    );
// ----- Low-level parallel port access -----------------------------------
#[no_mangle]
unsafe extern "C" fn port_write_data(p: *mut parport, d: c_uchar) {
    static void port_write_data(struct parport *p, unsigned char d)
    {
    parport_write_data(p, d);
    }
#[no_mangle]
unsafe extern "C" fn port_write_control(p: *mut parport, d: c_uchar) {
    static void port_write_control(struct parport *p, unsigned char d)
    {
    parport_write_control(p, d);
    }
#[no_mangle]
unsafe extern "C" fn port_read_data(p: *mut parport) -> c_uchar {
    static unsigned char port_read_data(struct parport *p)
    {
    return parport_read_data(p);
    }
#[no_mangle]
unsafe extern "C" fn port_read_status(p: *mut parport) -> c_uchar {
    static unsigned char port_read_status(struct parport *p)
    {
    return parport_read_status(p);
    }
#[no_mangle]
unsafe extern "C" fn port_read_control(p: *mut parport) -> c_uchar {
    static unsigned char port_read_control(struct parport *p)
    {
    return parport_read_control(p);
    }
    static void (* const port_write[])(struct parport *, unsigned char) = {
    port_write_data,
    core::ptr::null_mut(),
    port_write_control,
    };
    static unsigned char (* const port_read[])(struct parport *) = {
    port_read_data,
    port_read_status,
    port_read_control,
    };
// ----- Unified line operation functions ---------------------------------
    static inline void line_set(struct parport *data, int state,
    const struct lineop *op)
    {
    let mut oldval: u8 = port_read[op.port](data);
// Touch only the bit(s) needed
    if ((op.inverted && !state) || (!op.inverted && state))
    port_write[op.port](data, oldval | op.val);
    else
    port_write[op.port](data, oldval & ~op.val);
    }
    static inline int line_get(struct parport *data,
    const struct lineop *op)
    {
    let mut oldval: u8 = port_read[op.port](data);
    return ((op.inverted && (oldval & op.val) != op.val)
    || (!op.inverted && (oldval & op.val) == op.val));
    }
// ----- I2C algorithm call-back functions and structures -----------------
#[no_mangle]
unsafe extern "C" fn parport_setscl(data: *mut c_void, state: c_int) {
    static void parport_setscl(void *data, int state)
    {
    line_set((struct parport *) data, state, &adapter_parm[type].setscl);
    }
#[no_mangle]
unsafe extern "C" fn parport_setsda(data: *mut c_void, state: c_int) {
    static void parport_setsda(void *data, int state)
    {
    line_set((struct parport *) data, state, &adapter_parm[type].setsda);
    }
#[no_mangle]
unsafe extern "C" fn parport_getscl(data: *mut c_void) -> c_int {
    static int parport_getscl(void *data)
    {
    return line_get((struct parport *) data, &adapter_parm[type].getscl);
    }
#[no_mangle]
unsafe extern "C" fn parport_getsda(data: *mut c_void) -> c_int {
    static int parport_getsda(void *data)
    {
    return line_get((struct parport *) data, &adapter_parm[type].getsda);
    }
// Encapsulate the functions above in the correct structure.
    Note that this is only a template, from which the real structures are
    copied. The attaching code will set getscl to core::ptr::null_mut() for adapters that
    cannot read SCL back, and will also make the data field point to
    the parallel port structure. */
    static const struct i2c_algo_bit_data parport_algo_data = {
    .setsda		= parport_setsda,
    .setscl		= parport_setscl,
    .getsda		= parport_getsda,
    .getscl		= parport_getscl,
    .udelay		= 10, /* ~50 kbps */
    .timeout	= HZ,
    };
// ----- I2c and parallel port call-back functions and structures ---------
#[no_mangle]
unsafe extern "C" fn i2c_parport_irq(data: *mut c_void) {
    static void i2c_parport_irq(void *data)
    {
    struct i2c_par *adapter = data;
    struct i2c_client *ara = adapter.ara;
    if (ara) {
    dev_dbg(&ara.dev, "SMBus alert received\n");
    i2c_handle_smbus_alert(ara);
    } else
    dev_dbg(&adapter.adapter.dev,
    "SMBus alert received but no ARA client!\n");
    }
#[no_mangle]
unsafe extern "C" fn i2c_parport_attach(port: *mut parport) {
    static void i2c_parport_attach(struct parport *port)
    {
    struct i2c_par *adapter;
    int i;
    struct pardev_cb i2c_parport_cb;
    if (type < 0) {
    pr_warn("adapter type unspecified\n");
    return;
    }
    if (type >= ARRAY_SIZE(adapter_parm)) {
    pr_warn("invalid type (%d)\n", type);
    return;
    }
    for (i = 0; i < MAX_DEVICE; i++) {
    if (parport[i] == -1)
    continue;
    if (port.number == parport[i])
    break;
    }
    if (i == MAX_DEVICE) {
    pr_debug("Not using parport%d.\n", port.number);
    return;
    }
    adapter = kzalloc_obj(struct i2c_par);
    if (!adapter)
    return;
    memset(&i2c_parport_cb, 0, sizeof(i2c_parport_cb));
    i2c_parport_cb.flags = PARPORT_FLAG_EXCL;
    i2c_parport_cb.irq_func = i2c_parport_irq;
    i2c_parport_cb.private = adapter;
    pr_debug("attaching to %s\n", port.name);
    parport_disable_irq(port);
    adapter.pdev = parport_register_dev_model(port, "i2c-parport",
    &i2c_parport_cb, i);
    if (!adapter.pdev) {
    pr_err("Unable to register with parport\n");
    goto err_free;
    }
// Fill the rest of the structure
    adapter.adapter.owner = THIS_MODULE;
    adapter.adapter.class = I2C_CLASS_HWMON;
    strscpy(adapter.adapter.name, "Parallel port adapter",
    sizeof(adapter.adapter.name));
    adapter.algo_data = parport_algo_data;
// Slow down if we can't sense SCL
    if (!adapter_parm[type].getscl.val) {
    adapter.algo_data.getscl = core::ptr::null_mut();
    adapter.algo_data.udelay = 50; /* ~10 kbps */
    }
    adapter.algo_data.data = port;
    adapter.adapter.algo_data = &adapter.algo_data;
    adapter.adapter.dev.parent = port.physport.dev;
    if (parport_claim_or_block(adapter.pdev) < 0) {
    dev_err(&adapter.pdev.dev,
    "Could not claim parallel port\n");
    goto err_unregister;
    }
// Reset hardware to a sane state (SCL and SDA high)
    parport_setsda(port, 1);
    parport_setscl(port, 1);
// Other init if needed (power on...)
    if (adapter_parm[type].init.val) {
    line_set(port, 1, &adapter_parm[type].init);
// Give powered devices some time to settle
    msleep(100);
    }
    if (i2c_bit_add_bus(&adapter.adapter) < 0) {
    dev_err(&adapter.pdev.dev, "Unable to register with I2C\n");
    goto err_unregister;
    }
// Setup SMBus alert if supported
    if (adapter_parm[type].smbus_alert) {
    struct i2c_client *ara;
    ara = i2c_new_smbus_alert_device(&adapter.adapter,
    &adapter.alert_data);
    if (!IS_ERR(ara)) {
    adapter.ara = ara;
    parport_enable_irq(port);
    } else {
    dev_warn(&adapter.pdev.dev,
    "Failed to register ARA client\n");
    }
    }
// Add the new adapter to the list
    mutex_lock(&adapter_list_lock);
    list_add_tail(&adapter.node, &adapter_list);
    mutex_unlock(&adapter_list_lock);
    return;
    err_unregister:
    parport_release(adapter.pdev);
    parport_unregister_device(adapter.pdev);
    err_free:
    kfree(adapter);
    }
#[no_mangle]
unsafe extern "C" fn i2c_parport_detach(port: *mut parport) {
    static void i2c_parport_detach(struct parport *port)
    {
    struct i2c_par *adapter, *_n;
// Walk the list
    mutex_lock(&adapter_list_lock);
    list_for_each_entry_safe(adapter, _n, &adapter_list, node) {
    if (adapter.pdev.port == port) {
    if (adapter.ara) {
    parport_disable_irq(port);
    i2c_unregister_device(adapter.ara);
    }
    i2c_del_adapter(&adapter.adapter);
// Un-init if needed (power off...)
    if (adapter_parm[type].init.val)
    line_set(port, 0, &adapter_parm[type].init);
    parport_release(adapter.pdev);
    parport_unregister_device(adapter.pdev);
    list_del(&adapter.node);
    kfree(adapter);
    }
    }
    mutex_unlock(&adapter_list_lock);
    }
    static struct parport_driver i2c_parport_driver = {
    .name = "i2c-parport",
    .match_port = i2c_parport_attach,
    .detach = i2c_parport_detach,
    };
    module_parport_driver(i2c_parport_driver);
    MODULE_AUTHOR("Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("I2C bus over parallel port");
    MODULE_LICENSE("GPL");
