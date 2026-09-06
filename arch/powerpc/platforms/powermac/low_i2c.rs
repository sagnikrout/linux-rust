//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/low_i2c.c
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
// arch/powerpc/platforms/powermac/low_i2c.c
//
// Copyright (C) 2003-2005 Ben. Herrenschmidt (benh@kernel.crashing.org)
//
// The linux i2c layer isn't completely suitable for our needs for various
// reasons ranging from too late initialisation to semantics not perfectly
// matching some requirements of the apple platform functions etc...
//
// This file thus provides a simple low level unified i2c interface for
// powermac that covers the various types of i2c busses used in Apple machines.
// For now, keywest, PMU and SMU, though we could add Cuda, or other bit
// banging busses found on older chipsets in earlier machines if we ever need
// one of them.
//
// The drivers in this file are synchronous/blocking. In addition, the
// keywest one is fairly slow due to the use of msleep instead of interrupts
// as the interrupt is currently used by i2c-keywest. In the long run, we
// might want to get rid of those high-level interfaces to linux i2c layer
// either completely (converting all drivers) or replacing them all with a
// single stub driver on top of this one. Once done, the interrupt will be
// available for our use.
//

    printk(KERN_DEBUG "low_i2c:" x);	\
    } while(0)

// Macro flag: #define DBG(x...)

    printk(KERN_DEBUG "low_i2c:" x);	\
    } while(0)

// Macro flag: #define DBG_LOW(x...)

    let mut pmac_i2c_force_poll: static int = 1;
//
// A bus structure. Each bus in the system has such a structure associated.
//
    struct pmac_i2c_bus
    {
    struct list_head	link;
    struct device_node	*controller;
    struct device_node	*busnode;
    int			type;
    int			flags;
    struct i2c_adapter	adapter;
    void			*hostdata;
    int			channel;	/* some hosts have multiple */
    int			mode;		/* current mode */
    struct mutex		mutex;
    int			opened;
    int			polled;		/* open mode */
    struct platform_device	*platform_dev;
    struct lock_class_key   lock_key;
// ops
    int (*open)(struct pmac_i2c_bus *bus);
    void (*close)(struct pmac_i2c_bus *bus);
    int (*xfer)(struct pmac_i2c_bus *bus, u8 addrdir, int subsize,
    u32 subaddr, u8 *data, int len);
    };
    static LIST_HEAD(pmac_i2c_busses);
//
// Keywest implementation
//
    struct pmac_i2c_host_kw
    {
    struct mutex		mutex;		/* Access mutex for use by
// i2c-keywest
    void __iomem		*base;		/* register base address */
    int			bsteps;		/* register stepping */
    int			speed;		/* speed */
    int			irq;
    u8			*data;
    unsigned		len;
    int			state;
    int			rw;
    int			polled;
    int			result;
    struct completion	complete;
    spinlock_t		lock;
    struct timer_list	timeout_timer;
    };
// Register indices
    typedef enum {
    reg_mode = 0,
    reg_control,
    reg_status,
    reg_isr,
    reg_ier,
    reg_addr,
    reg_subaddr,
    reg_data
    } reg_t;
// The Tumbler audio equalizer can be really slow sometimes

// Mode register
pub const KW_I2C_MODE_100KHZ: c_uint = 0x00;
pub const KW_I2C_MODE_50KHZ: c_uint = 0x01;
pub const KW_I2C_MODE_25KHZ: c_uint = 0x02;
pub const KW_I2C_MODE_DUMB: c_uint = 0x00;
pub const KW_I2C_MODE_STANDARD: c_uint = 0x04;
pub const KW_I2C_MODE_STANDARDSUB: c_uint = 0x08;
pub const KW_I2C_MODE_COMBINED: c_uint = 0x0C;
pub const KW_I2C_MODE_MODE_MASK: c_uint = 0x0C;
pub const KW_I2C_MODE_CHAN_MASK: c_uint = 0xF0;
// Control register
pub const KW_I2C_CTL_AAK: c_uint = 0x01;
pub const KW_I2C_CTL_XADDR: c_uint = 0x02;
pub const KW_I2C_CTL_STOP: c_uint = 0x04;
pub const KW_I2C_CTL_START: c_uint = 0x08;
// Status register
pub const KW_I2C_STAT_BUSY: c_uint = 0x01;
pub const KW_I2C_STAT_LAST_AAK: c_uint = 0x02;
pub const KW_I2C_STAT_LAST_RW: c_uint = 0x04;
pub const KW_I2C_STAT_SDA: c_uint = 0x08;
pub const KW_I2C_STAT_SCL: c_uint = 0x10;
// IER & ISR registers
pub const KW_I2C_IRQ_DATA: c_uint = 0x01;
pub const KW_I2C_IRQ_ADDR: c_uint = 0x02;
pub const KW_I2C_IRQ_STOP: c_uint = 0x04;
pub const KW_I2C_IRQ_START: c_uint = 0x08;
pub const KW_I2C_IRQ_MASK: c_uint = 0x0F;
// State machine states
    enum {
    state_idle,
    state_addr,
    state_read,
    state_write,
    state_stop,
    state_dead
    };

    printk(KERN_DEBUG "KW: wrong state. Got %s, state: %s " \
    "(isr: %02x)\n",	\
    name, __kw_state_names[host.state], isr); \
    } while(0)
    static const char *__kw_state_names[] = {
    "state_idle",
    "state_addr",
    "state_read",
    "state_write",
    "state_stop",
    "state_dead"
    };
#[no_mangle]
pub unsafe extern "C" fn __kw_read_reg(host: *mut pmac_i2c_host_kw, reg: reg_t) -> u8 {
    static inline u8 __kw_read_reg(struct pmac_i2c_host_kw *host, reg_t reg)
    {
    return readb(host.base + (((unsigned int)reg) << host.bsteps));
    }
    static inline void __kw_write_reg(struct pmac_i2c_host_kw *host,
    reg_t reg, u8 val)
    {
    writeb(val, host.base + (((unsigned)reg) << host.bsteps));
    (void)__kw_read_reg(host, reg_subaddr);
    }

#[no_mangle]
unsafe extern "C" fn kw_i2c_wait_interrupt(host: *mut pmac_i2c_host_kw) -> u8 {
    static u8 kw_i2c_wait_interrupt(struct pmac_i2c_host_kw *host)
    {
    int i, j;
    u8 isr;
    for (i = 0; i < 1000; i++) {
    isr = kw_read_reg(reg_isr) & KW_I2C_IRQ_MASK;
    if (isr != 0)
    return isr;
// This code is used with the timebase frozen, we cannot rely
// on udelay nor schedule when in polled mode !
// For now, just use a bogus loop....
//
    if (host.polled) {
    for (j = 1; j < 100000; j++)
    mb();
    } else
    msleep(1);
    }
    return isr;
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_do_stop(host: *mut pmac_i2c_host_kw, result: c_int) {
    static void kw_i2c_do_stop(struct pmac_i2c_host_kw *host, int result)
    {
    kw_write_reg(reg_control, KW_I2C_CTL_STOP);
    host.state = state_stop;
    host.result = result;
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_handle_interrupt(host: *mut pmac_i2c_host_kw, isr: u8) {
    static void kw_i2c_handle_interrupt(struct pmac_i2c_host_kw *host, u8 isr)
    {
    u8 ack;
    DBG_LOW("kw_handle_interrupt(%s, isr: %x)\n",
    __kw_state_names[host.state], isr);
    if (host.state == state_idle) {
    printk(KERN_WARNING "low_i2c: Keywest got an out of state"
    " interrupt, ignoring\n");
    kw_write_reg(reg_isr, isr);
    return;
    }
    if (isr == 0) {
    printk(KERN_WARNING "low_i2c: Timeout in i2c transfer"
    " on keywest !\n");
    if (host.state != state_stop) {
    kw_i2c_do_stop(host, -EIO);
    return;
    }
    ack = kw_read_reg(reg_status);
    if (ack & KW_I2C_STAT_BUSY)
    kw_write_reg(reg_status, 0);
    host.state = state_idle;
    kw_write_reg(reg_ier, 0x00);
    if (!host.polled)
    complete(&host.complete);
    return;
    }
    if (isr & KW_I2C_IRQ_ADDR) {
    ack = kw_read_reg(reg_status);
    if (host.state != state_addr) {
    WRONG_STATE("KW_I2C_IRQ_ADDR");
    kw_i2c_do_stop(host, -EIO);
    }
    if ((ack & KW_I2C_STAT_LAST_AAK) == 0) {
    host.result = -ENXIO;
    host.state = state_stop;
    DBG_LOW("KW: NAK on address\n");
    } else {
    if (host.len == 0)
    kw_i2c_do_stop(host, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: host->rw) -> else {
    host.state = state_read;
    if (host.len > 1)
    kw_write_reg(reg_control,
    KW_I2C_CTL_AAK);
    } else {
    host.state = state_write;
    kw_write_reg(reg_data, *(host.data++));
    host.len--;
    }
    }
    kw_write_reg(reg_isr, KW_I2C_IRQ_ADDR);
    }
    if (isr & KW_I2C_IRQ_DATA) {
    if (host.state == state_read) {
// (host->data++) = kw_read_reg(reg_data);
    host.len--;
    kw_write_reg(reg_isr, KW_I2C_IRQ_DATA);
    if (host.len == 0)
    host.state = state_stop;
#[no_mangle]
pub unsafe extern "C" fn if(1: host->len ==) -> else {
    else if (host.len == 1)
    kw_write_reg(reg_control, 0);
    } else if (host.state == state_write) {
    ack = kw_read_reg(reg_status);
    if ((ack & KW_I2C_STAT_LAST_AAK) == 0) {
    DBG_LOW("KW: nack on data write\n");
    host.result = -EFBIG;
    host.state = state_stop;
    } else if (host.len) {
    kw_write_reg(reg_data, *(host.data++));
    host.len--;
    } else
    kw_i2c_do_stop(host, 0);
    } else {
    WRONG_STATE("KW_I2C_IRQ_DATA");
    if (host.state != state_stop)
    kw_i2c_do_stop(host, -EIO);
    }
    kw_write_reg(reg_isr, KW_I2C_IRQ_DATA);
    }
    if (isr & KW_I2C_IRQ_STOP) {
    kw_write_reg(reg_isr, KW_I2C_IRQ_STOP);
    if (host.state != state_stop) {
    WRONG_STATE("KW_I2C_IRQ_STOP");
    host.result = -EIO;
    }
    host.state = state_idle;
    if (!host.polled)
    complete(&host.complete);
    }
// Below should only happen in manual mode which we don't use ...
    if (isr & KW_I2C_IRQ_START)
    kw_write_reg(reg_isr, KW_I2C_IRQ_START);
    }
// Interrupt handler
#[no_mangle]
unsafe extern "C" fn kw_i2c_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t kw_i2c_irq(int irq, void *dev_id)
    {
    struct pmac_i2c_host_kw *host = dev_id;
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    timer_delete(&host.timeout_timer);
    kw_i2c_handle_interrupt(host, kw_read_reg(reg_isr));
    if (host.state != state_idle) {
    host.timeout_timer.expires = jiffies + KW_POLL_TIMEOUT;
    add_timer(&host.timeout_timer);
    }
    spin_unlock_irqrestore(&host.lock, flags);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_timeout(t: *mut timer_list) {
    static void kw_i2c_timeout(struct timer_list *t)
    {
    struct pmac_i2c_host_kw *host = timer_container_of(host, t,
    timeout_timer);
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
//
// If the timer is pending, that means we raced with the
// irq, in which case we just return
//
    if (timer_pending(&host.timeout_timer))
    goto skip;
    kw_i2c_handle_interrupt(host, kw_read_reg(reg_isr));
    if (host.state != state_idle) {
    host.timeout_timer.expires = jiffies + KW_POLL_TIMEOUT;
    add_timer(&host.timeout_timer);
    }
    skip:
    spin_unlock_irqrestore(&host.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_open(bus: *mut pmac_i2c_bus) -> c_int {
    static int kw_i2c_open(struct pmac_i2c_bus *bus)
    {
    struct pmac_i2c_host_kw *host = bus.hostdata;
    mutex_lock(&host.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_close(bus: *mut pmac_i2c_bus) {
    static void kw_i2c_close(struct pmac_i2c_bus *bus)
    {
    struct pmac_i2c_host_kw *host = bus.hostdata;
    mutex_unlock(&host.mutex);
    }
    static int kw_i2c_xfer(struct pmac_i2c_bus *bus, u8 addrdir, int subsize,
    u32 subaddr, u8 *data, int len)
    {
    struct pmac_i2c_host_kw *host = bus.hostdata;
    let mut mode_reg: u8 = host.speed;
    let mut use_irq: c_int = host.irq && !bus.polled;
// Setup mode & subaddress if any
    switch(bus.mode) {
    case pmac_i2c_mode_dumb:
    return -EINVAL;
    case pmac_i2c_mode_std:
    mode_reg |= KW_I2C_MODE_STANDARD;
    if (subsize != 0)
    return -EINVAL;
    break;
    case pmac_i2c_mode_stdsub:
    mode_reg |= KW_I2C_MODE_STANDARDSUB;
    if (subsize != 1)
    return -EINVAL;
    break;
    case pmac_i2c_mode_combined:
    mode_reg |= KW_I2C_MODE_COMBINED;
    if (subsize != 1)
    return -EINVAL;
    break;
    }
// Setup channel & clear pending irqs
    kw_write_reg(reg_isr, kw_read_reg(reg_isr));
    kw_write_reg(reg_mode, mode_reg | (bus.channel << 4));
    kw_write_reg(reg_status, 0);
// Set up address and r/w bit, strip possible stale bus number from
// address top bits
//
    kw_write_reg(reg_addr, addrdir & 0xff);
// Set up the sub address
    if ((mode_reg & KW_I2C_MODE_MODE_MASK) == KW_I2C_MODE_STANDARDSUB
    || (mode_reg & KW_I2C_MODE_MODE_MASK) == KW_I2C_MODE_COMBINED)
    kw_write_reg(reg_subaddr, subaddr);
// Prepare for async operations
    host.data = data;
    host.len = len;
    host.state = state_addr;
    host.result = 0;
    host.rw = (addrdir & 1);
    host.polled = bus.polled;
// Enable interrupt if not using polled mode and interrupt is
// available
//
    if (use_irq) {
// Clear completion
    reinit_completion(&host.complete);
// Ack stale interrupts
    kw_write_reg(reg_isr, kw_read_reg(reg_isr));
// Arm timeout
    host.timeout_timer.expires = jiffies + KW_POLL_TIMEOUT;
    add_timer(&host.timeout_timer);
// Enable emission
    kw_write_reg(reg_ier, KW_I2C_IRQ_MASK);
    }
// Start sending address
    kw_write_reg(reg_control, KW_I2C_CTL_XADDR);
// Wait for completion
    if (use_irq)
    wait_for_completion(&host.complete);
    else {
    while(host.state != state_idle) {
    unsigned long flags;
    let mut isr: u8 = kw_i2c_wait_interrupt(host);
    spin_lock_irqsave(&host.lock, flags);
    kw_i2c_handle_interrupt(host, isr);
    spin_unlock_irqrestore(&host.lock, flags);
    }
    }
// Disable emission
    kw_write_reg(reg_ier, 0);
    return host.result;
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_host_init(np: *mut device_node) -> *mut pmac_i2c_host_kw __init {
    static struct pmac_i2c_host_kw *__init kw_i2c_host_init(struct device_node *np)
    {
    struct pmac_i2c_host_kw *host;
    const u32		*psteps, *prate, *addrp;
    u32			steps;
    host = kzalloc_obj(*host);
    if (host == core::ptr::null_mut()) {
    printk(KERN_ERR "low_i2c: Can't allocate host for %pOF\n",
    np);
    return core::ptr::null_mut();
    }
// Apple is kind enough to provide a valid AAPL,address property
// on all i2c keywest nodes so far ... we would have to fallback
// to macio parsing if that wasn't the case
//
    addrp = of_get_property(np, "AAPL,address", core::ptr::null_mut());
    if (addrp == core::ptr::null_mut()) {
    printk(KERN_ERR "low_i2c: Can't find address for %pOF\n",
    np);
    kfree(host);
    return core::ptr::null_mut();
    }
    mutex_init(&host.mutex);
    init_completion(&host.complete);
    spin_lock_init(&host.lock);
    timer_setup(&host.timeout_timer, kw_i2c_timeout, 0);
    psteps = of_get_property(np, "AAPL,address-step", core::ptr::null_mut());
    steps = psteps ? (*psteps) : 0x10;
    for (host.bsteps = 0; (steps & 0x01) == 0; host.bsteps++)
    steps >>= 1;
// Select interface rate
    host.speed = KW_I2C_MODE_25KHZ;
    prate = of_get_property(np, "AAPL,i2c-rate", core::ptr::null_mut());
    if (prate) switch(*prate) {
    case 100:
    host.speed = KW_I2C_MODE_100KHZ;
    break;
    case 50:
    host.speed = KW_I2C_MODE_50KHZ;
    break;
    case 25:
    host.speed = KW_I2C_MODE_25KHZ;
    break;
    }
    host.irq = irq_of_parse_and_map(np, 0);
    if (!host.irq)
    printk(KERN_WARNING
    "low_i2c: Failed to map interrupt for %pOF\n",
    np);
    host.base = ioremap((*addrp), 0x1000);
    if (host.base == core::ptr::null_mut()) {
    printk(KERN_ERR "low_i2c: Can't map registers for %pOF\n",
    np);
    kfree(host);
    return core::ptr::null_mut();
    }
// Make sure IRQ is disabled
    kw_write_reg(reg_ier, 0);
// Request chip interrupt. We set IRQF_NO_SUSPEND because we don't
// want that interrupt disabled between the 2 passes of driver
// suspend or we'll have issues running the pfuncs
//
    if (request_irq(host.irq, kw_i2c_irq, IRQF_NO_SUSPEND,
    "keywest i2c", host))
    host.irq = 0;
    printk(KERN_INFO "KeyWest i2c @0x%08x irq %d %pOF\n",
// addrp, host->irq, np);
    return host;
    }
    static void __init kw_i2c_add(struct pmac_i2c_host_kw *host,
    struct device_node *controller,
    struct device_node *busnode,
    int channel)
    {
    struct pmac_i2c_bus *bus;
    bus = kzalloc_obj(struct pmac_i2c_bus);
    if (bus == core::ptr::null_mut())
    return;
    bus.controller = of_node_get(controller);
    bus.busnode = of_node_get(busnode);
    bus.type = pmac_i2c_bus_keywest;
    bus.hostdata = host;
    bus.channel = channel;
    bus.mode = pmac_i2c_mode_std;
    bus.open = kw_i2c_open;
    bus.close = kw_i2c_close;
    bus.xfer = kw_i2c_xfer;
    mutex_init(&bus.mutex);
    lockdep_register_key(&bus.lock_key);
    lockdep_set_class(&bus.mutex, &bus.lock_key);
    if (controller == busnode)
    bus.flags = pmac_i2c_multibus;
    list_add(&bus.link, &pmac_i2c_busses);
    printk(KERN_INFO " channel %d bus %s\n", channel,
    (controller == busnode) ? "<multibus>" : busnode.full_name);
    }
#[no_mangle]
unsafe extern "C" fn kw_i2c_probe() -> void __init {
    static void __init kw_i2c_probe(void)
    {
    struct device_node *np, *child, *parent;
// Probe keywest-i2c busses
    for_each_compatible_node(np, "i2c","keywest-i2c") {
    struct pmac_i2c_host_kw *host;
    int multibus;
// Found one, init a host structure
    host = kw_i2c_host_init(np);
    if (host == core::ptr::null_mut())
    continue;
// Now check if we have a multibus setup (old style) or if we
// have proper bus nodes. Note that the "new" way (proper bus
// nodes) might cause us to not create some busses that are
// kept hidden in the device-tree. In the future, we might
// want to work around that by creating busses without a node
// but not for now
//
    child = of_get_next_child(np, core::ptr::null_mut());
    multibus = !of_node_name_eq(child, "i2c-bus");
    of_node_put(child);
// For a multibus setup, we get the bus count based on the
// parent type
//
    if (multibus) {
    int chans, i;
    parent = of_get_parent(np);
    if (parent == core::ptr::null_mut())
    continue;
    chans = parent.name[0] == 'u' ? 2 : 1;
    of_node_put(parent);
    for (i = 0; i < chans; i++)
    kw_i2c_add(host, np, np, i);
    } else {
    for_each_child_of_node(np, child) {
    const u32 *reg = of_get_property(child,
    "reg", core::ptr::null_mut());
    if (reg == core::ptr::null_mut())
    continue;
    kw_i2c_add(host, np, child, *reg);
    }
    }
    }
    }
//
// PMU implementation
//

//
// i2c command block to the PMU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_i2c_hdr {
    pub bus: u8,
    pub mode: u8,
    pub bus2: u8,
    pub address: u8,
    pub sub_addr: u8,
    pub comb_addr: u8,
    pub count: u8,
    pub data: [u8; ],
}

#[no_mangle]
unsafe extern "C" fn pmu_i2c_complete(req: *mut adb_request) {
    static void pmu_i2c_complete(struct adb_request *req)
    {
    complete(req.arg);
    }
    static int pmu_i2c_xfer(struct pmac_i2c_bus *bus, u8 addrdir, int subsize,
    u32 subaddr, u8 *data, int len)
    {
    struct adb_request *req = bus.hostdata;
    struct pmu_i2c_hdr *hdr = (struct pmu_i2c_hdr *)&req.data[1];
    struct completion comp;
    let mut read: c_int = addrdir & 1;
    int retry;
    let mut rc: c_int = 0;
// For now, limit ourselves to 16 bytes transfers
    if (len > 16)
    return -EINVAL;
    init_completion(&comp);
    for (retry = 0; retry < 16; retry++) {
    memset(req, 0, sizeof(struct adb_request));
    hdr.bus = bus.channel;
    hdr.count = len;
    switch(bus.mode) {
    case pmac_i2c_mode_std:
    if (subsize != 0)
    return -EINVAL;
    hdr.address = addrdir;
    hdr.mode = PMU_I2C_MODE_SIMPLE;
    break;
    case pmac_i2c_mode_stdsub:
    case pmac_i2c_mode_combined:
    if (subsize != 1)
    return -EINVAL;
    hdr.address = addrdir & 0xfe;
    hdr.comb_addr = addrdir;
    hdr.sub_addr = subaddr;
    if (bus.mode == pmac_i2c_mode_stdsub)
    hdr.mode = PMU_I2C_MODE_STDSUB;
    else
    hdr.mode = PMU_I2C_MODE_COMBINED;
    break;
    default:
    return -EINVAL;
    }
    reinit_completion(&comp);
    req.data[0] = PMU_I2C_CMD;
    req.reply[0] = 0xff;
    req.nbytes = sizeof(struct pmu_i2c_hdr) + 1;
    req.done = pmu_i2c_complete;
    req.arg = &comp;
    if (!read && len) {
    memcpy(hdr.data, data, len);
    req.nbytes += len;
    }
    rc = pmu_queue_request(req);
    if (rc)
    return rc;
    wait_for_completion(&comp);
    if (req.reply[0] == PMU_I2C_STATUS_OK)
    break;
    msleep(15);
    }
    if (req.reply[0] != PMU_I2C_STATUS_OK)
    return -EIO;
    for (retry = 0; retry < 16; retry++) {
    memset(req, 0, sizeof(struct adb_request));
// I know that looks like a lot, slow as hell, but darwin
// does it so let's be on the safe side for now
//
    msleep(15);
    hdr.bus = PMU_I2C_BUS_STATUS;
    reinit_completion(&comp);
    req.data[0] = PMU_I2C_CMD;
    req.reply[0] = 0xff;
    req.nbytes = 2;
    req.done = pmu_i2c_complete;
    req.arg = &comp;
    rc = pmu_queue_request(req);
    if (rc)
    return rc;
    wait_for_completion(&comp);
    if (req.reply[0] == PMU_I2C_STATUS_OK && !read)
    return 0;
    if (req.reply[0] == PMU_I2C_STATUS_DATAREAD && read) {
    let mut rlen: c_int = req.reply_len - 1;
    if (rlen != len) {
    printk(KERN_WARNING "low_i2c: PMU returned %d"
    " bytes, expected %d !\n", rlen, len);
    return -EIO;
    }
    if (len)
    memcpy(data, &req.reply[1], len);
    return 0;
    }
    }
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn pmu_i2c_probe() -> void __init {
    static void __init pmu_i2c_probe(void)
    {
    struct pmac_i2c_bus *bus;
    struct device_node *busnode;
    int channel, sz;
    if (!pmu_present())
    return;
// There might or might not be a "pmu-i2c" node, we use that
// or via-pmu itself, whatever we find. I haven't seen a machine
// with separate bus nodes, so we assume a multibus setup
//
    busnode = of_find_node_by_name(core::ptr::null_mut(), "pmu-i2c");
    if (busnode == core::ptr::null_mut())
    busnode = of_find_node_by_name(core::ptr::null_mut(), "via-pmu");
    if (busnode == core::ptr::null_mut())
    return;
    printk(KERN_INFO "PMU i2c %pOF\n", busnode);
//
// We add bus 1 and 2 only for now, bus 0 is "special"
//
    for (channel = 1; channel <= 2; channel++) {
    sz = sizeof(struct pmac_i2c_bus) + sizeof(struct adb_request);
    bus = kzalloc(sz, GFP_KERNEL);
    if (bus == core::ptr::null_mut())
    return;
    bus.controller = busnode;
    bus.busnode = busnode;
    bus.type = pmac_i2c_bus_pmu;
    bus.channel = channel;
    bus.mode = pmac_i2c_mode_std;
    bus.hostdata = bus + 1;
    bus.xfer = pmu_i2c_xfer;
    mutex_init(&bus.mutex);
    lockdep_register_key(&bus.lock_key);
    lockdep_set_class(&bus.mutex, &bus.lock_key);
    bus.flags = pmac_i2c_multibus;
    list_add(&bus.link, &pmac_i2c_busses);
    printk(KERN_INFO " channel %d bus <multibus>\n", channel);
    }
    }

//
// SMU implementation
//

#[no_mangle]
unsafe extern "C" fn smu_i2c_complete(cmd: *mut smu_i2c_cmd, misc: *mut c_void) {
    static void smu_i2c_complete(struct smu_i2c_cmd *cmd, void *misc)
    {
    complete(misc);
    }
    static int smu_i2c_xfer(struct pmac_i2c_bus *bus, u8 addrdir, int subsize,
    u32 subaddr, u8 *data, int len)
    {
    struct smu_i2c_cmd *cmd = bus.hostdata;
    struct completion comp;
    let mut read: c_int = addrdir & 1;
    let mut rc: c_int = 0;
    if ((read && len > SMU_I2C_READ_MAX) ||
    ((!read) && len > SMU_I2C_WRITE_MAX))
    return -EINVAL;
    memset(cmd, 0, sizeof(struct smu_i2c_cmd));
    cmd.info.bus = bus.channel;
    cmd.info.devaddr = addrdir;
    cmd.info.datalen = len;
    switch(bus.mode) {
    case pmac_i2c_mode_std:
    if (subsize != 0)
    return -EINVAL;
    cmd.info.type = SMU_I2C_TRANSFER_SIMPLE;
    break;
    case pmac_i2c_mode_stdsub:
    case pmac_i2c_mode_combined:
    if (subsize > 3 || subsize < 1)
    return -EINVAL;
    cmd.info.sublen = subsize;
// that's big-endian only but heh !
    memcpy(&cmd.info.subaddr, ((char *)&subaddr) + (4 - subsize),
    subsize);
    if (bus.mode == pmac_i2c_mode_stdsub)
    cmd.info.type = SMU_I2C_TRANSFER_STDSUB;
    else
    cmd.info.type = SMU_I2C_TRANSFER_COMBINED;
    break;
    default:
    return -EINVAL;
    }
    if (!read && len)
    memcpy(cmd.info.data, data, len);
    init_completion(&comp);
    cmd.done = smu_i2c_complete;
    cmd.misc = &comp;
    rc = smu_queue_i2c(cmd);
    if (rc < 0)
    return rc;
    wait_for_completion(&comp);
    rc = cmd.status;
    if (read && len)
    memcpy(data, cmd.info.data, len);
    return rc < 0 ? rc : 0;
    }
#[no_mangle]
unsafe extern "C" fn smu_i2c_probe() -> void __init {
    static void __init smu_i2c_probe(void)
    {
    struct device_node *controller, *busnode;
    struct pmac_i2c_bus *bus;
    const u32 *reg;
    int sz;
    if (!smu_present())
    return;
    controller = of_find_node_by_name(core::ptr::null_mut(), "smu-i2c-control");
    if (controller == core::ptr::null_mut())
    controller = of_find_node_by_name(core::ptr::null_mut(), "smu");
    if (controller == core::ptr::null_mut())
    return;
    printk(KERN_INFO "SMU i2c %pOF\n", controller);
// Look for childs, note that they might not be of the right
// type as older device trees mix i2c busses and other things
// at the same level
//
    for_each_child_of_node(controller, busnode) {
    if (!of_node_is_type(busnode, "i2c") &&
    !of_node_is_type(busnode, "i2c-bus"))
    continue;
    reg = of_get_property(busnode, "reg", core::ptr::null_mut());
    if (reg == core::ptr::null_mut())
    continue;
    sz = sizeof(struct pmac_i2c_bus) + sizeof(struct smu_i2c_cmd);
    bus = kzalloc(sz, GFP_KERNEL);
    if (bus == core::ptr::null_mut()) {
    of_node_put(busnode);
    return;
    }
    bus.controller = controller;
    bus.busnode = of_node_get(busnode);
    bus.type = pmac_i2c_bus_smu;
    bus.channel = *reg;
    bus.mode = pmac_i2c_mode_std;
    bus.hostdata = bus + 1;
    bus.xfer = smu_i2c_xfer;
    mutex_init(&bus.mutex);
    lockdep_register_key(&bus.lock_key);
    lockdep_set_class(&bus.mutex, &bus.lock_key);
    bus.flags = 0;
    list_add(&bus.link, &pmac_i2c_busses);
    printk(KERN_INFO " channel %x bus %pOF\n",
    bus.channel, busnode);
    }
    }

//
// Core code
//
    struct pmac_i2c_bus *pmac_i2c_find_bus(struct device_node *node)
    {
    struct device_node *p = of_node_get(node);
    struct device_node *prev = core::ptr::null_mut();
    struct pmac_i2c_bus *bus;
    while(p) {
    list_for_each_entry(bus, &pmac_i2c_busses, link) {
    if (p == bus.busnode) {
    if (prev && bus.flags & pmac_i2c_multibus) {
    const u32 *reg;
    reg = of_get_property(prev, "reg",
    core::ptr::null_mut());
    if (!reg)
    continue;
    if (((*reg) >> 8) != bus.channel)
    continue;
    }
    of_node_put(p);
    of_node_put(prev);
    return bus;
    }
    }
    of_node_put(prev);
    prev = p;
    p = of_get_parent(p);
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_find_bus);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_get_dev_addr(device: *mut device_node) -> u8 {
    u8 pmac_i2c_get_dev_addr(struct device_node *device)
    {
    const u32 *reg = of_get_property(device, "reg", core::ptr::null_mut());
    if (reg == core::ptr::null_mut())
    return 0;
    return (*reg) & 0xff;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_dev_addr);
    struct device_node *pmac_i2c_get_controller(struct pmac_i2c_bus *bus)
    {
    return bus.controller;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_controller);
    struct device_node *pmac_i2c_get_bus_node(struct pmac_i2c_bus *bus)
    {
    return bus.busnode;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_bus_node);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_get_type(bus: *mut pmac_i2c_bus) -> c_int {
    int pmac_i2c_get_type(struct pmac_i2c_bus *bus)
    {
    return bus.type;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_type);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_get_flags(bus: *mut pmac_i2c_bus) -> c_int {
    int pmac_i2c_get_flags(struct pmac_i2c_bus *bus)
    {
    return bus.flags;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_flags);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_get_channel(bus: *mut pmac_i2c_bus) -> c_int {
    int pmac_i2c_get_channel(struct pmac_i2c_bus *bus)
    {
    return bus.channel;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_channel);
    struct i2c_adapter *pmac_i2c_get_adapter(struct pmac_i2c_bus *bus)
    {
    return &bus.adapter;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_get_adapter);
    struct pmac_i2c_bus *pmac_i2c_adapter_to_bus(struct i2c_adapter *adapter)
    {
    struct pmac_i2c_bus *bus;
    list_for_each_entry(bus, &pmac_i2c_busses, link)
    if (&bus.adapter == adapter)
    return bus;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_adapter_to_bus);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_match_adapter(dev: *mut device_node, adapter: *mut i2c_adapter) -> c_int {
    int pmac_i2c_match_adapter(struct device_node *dev, struct i2c_adapter *adapter)
    {
    struct pmac_i2c_bus *bus = pmac_i2c_find_bus(dev);
    if (bus == core::ptr::null_mut())
    return 0;
    return (&bus.adapter == adapter);
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_match_adapter);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_open(bus: *mut pmac_i2c_bus, polled: c_int) -> c_int {
    int pmac_i2c_open(struct pmac_i2c_bus *bus, int polled)
    {
    int rc;
    mutex_lock(&bus.mutex);
    bus.polled = polled || pmac_i2c_force_poll;
    bus.opened = 1;
    bus.mode = pmac_i2c_mode_std;
    if (bus.open && (rc = bus.open(bus)) != 0) {
    bus.opened = 0;
    mutex_unlock(&bus.mutex);
    return rc;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_open);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_close(bus: *mut pmac_i2c_bus) {
    void pmac_i2c_close(struct pmac_i2c_bus *bus)
    {
    WARN_ON(!bus.opened);
    if (bus.close)
    bus.close(bus);
    bus.opened = 0;
    mutex_unlock(&bus.mutex);
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_close);
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_setmode(bus: *mut pmac_i2c_bus, mode: c_int) -> c_int {
    int pmac_i2c_setmode(struct pmac_i2c_bus *bus, int mode)
    {
    WARN_ON(!bus.opened);
// Report me if you see the error below as there might be a new
// "combined4" mode that I need to implement for the SMU bus
//
    if (mode < pmac_i2c_mode_dumb || mode > pmac_i2c_mode_combined) {
    printk(KERN_ERR "low_i2c: Invalid mode %d requested on"
    " bus %pOF !\n", mode, bus.busnode);
    return -EINVAL;
    }
    bus.mode = mode;
    return 0;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_setmode);
    int pmac_i2c_xfer(struct pmac_i2c_bus *bus, u8 addrdir, int subsize,
    u32 subaddr, u8 *data, int len)
    {
    int rc;
    WARN_ON(!bus.opened);
    DBG("xfer() chan=%d, addrdir=0x%x, mode=%d, subsize=%d, subaddr=0x%x,"
    " %d bytes, bus %pOF\n", bus.channel, addrdir, bus.mode, subsize,
    subaddr, len, bus.busnode);
    rc = bus.xfer(bus, addrdir, subsize, subaddr, data, len);

    if (rc)
    DBG("xfer error %d\n", rc);

    return rc;
    }
    EXPORT_SYMBOL_GPL(pmac_i2c_xfer);
// some quirks for platform function decoding
    enum {
    pmac_i2c_quirk_invmask = 0x00000001u,
    pmac_i2c_quirk_skip = 0x00000002u,
    };
    static void pmac_i2c_devscan(void (*callback)(struct device_node *dev,
    int quirks))
    {
    struct pmac_i2c_bus *bus;
    struct device_node *np;
    static struct whitelist_ent {
    char *name;
    char *compatible;
    int quirks;
    } whitelist[] = {
// XXX Study device-tree's & apple drivers are get the quirks
// right !
//
// Workaround: It seems that running the clockspreading
// properties on the eMac will cause lockups during boot.
// The machine seems to work fine without that. So for now,
// let's make sure i2c-hwclock doesn't match about "imic"
// clocks and we'll figure out if we really need to do
// something special about those later.
//
    { "i2c-hwclock", "imic5002", pmac_i2c_quirk_skip },
    { "i2c-hwclock", "imic5003", pmac_i2c_quirk_skip },
    { "i2c-hwclock", core::ptr::null_mut(), pmac_i2c_quirk_invmask },
    { "i2c-cpu-voltage", core::ptr::null_mut(), 0},
    {  "temp-monitor", core::ptr::null_mut(), 0 },
    {  "supply-monitor", core::ptr::null_mut(), 0 },
    { core::ptr::null_mut(), core::ptr::null_mut(), 0 },
    };
// Only some devices need to have platform functions instantiated
// here. For now, we have a table. Others, like 9554 i2c GPIOs used
// on Xserve, if we ever do a driver for them, will use their own
// platform function instance
//
    list_for_each_entry(bus, &pmac_i2c_busses, link) {
    for_each_child_of_node(bus.busnode, np) {
    struct whitelist_ent *p;
// If multibus, check if device is on that bus
    if (bus.flags & pmac_i2c_multibus)
    if (bus != pmac_i2c_find_bus(np))
    continue;
    for (p = whitelist; p.name != core::ptr::null_mut(); p++) {
    if (!of_node_name_eq(np, p.name))
    continue;
    if (p.compatible &&
    !of_device_is_compatible(np, p.compatible))
    continue;
    if (p.quirks & pmac_i2c_quirk_skip)
    break;
    callback(np, p.quirks);
    break;
    }
    }
    }
    }
pub const MAX_I2C_DATA: c_int = 64;
    struct pmac_i2c_pf_inst
    {
    struct pmac_i2c_bus	*bus;
    u8			addr;
    u8			buffer[MAX_I2C_DATA];
    u8			scratch[MAX_I2C_DATA];
    int			bytes;
    int			quirks;
    };
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_begin(func: *mut pmf_function, args: *mut pmf_args) -> *mut c_void {
    static void* pmac_i2c_do_begin(struct pmf_function *func, struct pmf_args *args)
    {
    struct pmac_i2c_pf_inst *inst;
    struct pmac_i2c_bus	*bus;
    bus = pmac_i2c_find_bus(func.node);
    if (bus == core::ptr::null_mut()) {
    printk(KERN_ERR "low_i2c: Can't find bus for %pOF (pfunc)\n",
    func.node);
    return core::ptr::null_mut();
    }
    if (pmac_i2c_open(bus, 0)) {
    printk(KERN_ERR "low_i2c: Can't open i2c bus for %pOF (pfunc)\n",
    func.node);
    return core::ptr::null_mut();
    }
// XXX might need GFP_ATOMIC when called during the suspend process,
// but then, there are already lots of issues with suspending when
// near OOM that need to be resolved, the allocator itself should
// probably make GFP_NOIO implicit during suspend
//
    inst = kzalloc_obj(struct pmac_i2c_pf_inst);
    if (inst == core::ptr::null_mut()) {
    pmac_i2c_close(bus);
    return core::ptr::null_mut();
    }
    inst.bus = bus;
    inst.addr = pmac_i2c_get_dev_addr(func.node);
    inst.quirks = (int)(long)func.driver_data;
    return inst;
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_end(func: *mut pmf_function, instdata: *mut c_void) {
    static void pmac_i2c_do_end(struct pmf_function *func, void *instdata)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    if (inst == core::ptr::null_mut())
    return;
    pmac_i2c_close(inst.bus);
    kfree(inst);
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_read(_arg: PMF_STD_ARGS, len: u32) -> c_int {
    static int pmac_i2c_do_read(PMF_STD_ARGS, u32 len)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    inst.bytes = len;
    return pmac_i2c_xfer(inst.bus, inst.addr | pmac_i2c_read, 0, 0,
    inst.buffer, len);
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_write(_arg: PMF_STD_ARGS, len: u32, data: *const u8) -> c_int {
    static int pmac_i2c_do_write(PMF_STD_ARGS, u32 len, const u8 *data)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    return pmac_i2c_xfer(inst.bus, inst.addr | pmac_i2c_write, 0, 0,
    (u8 *)data, len);
    }
// This function is used to do the masking & OR'ing for the "rmw" type
// callbacks. Ze should apply the mask and OR in the values in the
// buffer before writing back. The problem is that it seems that
// various darwin drivers implement the mask/or differently, thus
// we need to check the quirks first
//
    static void pmac_i2c_do_apply_rmw(struct pmac_i2c_pf_inst *inst,
    u32 len, const u8 *mask, const u8 *val)
    {
    int i;
    if (inst.quirks & pmac_i2c_quirk_invmask) {
    for (i = 0; i < len; i ++)
    inst.scratch[i] = (inst.buffer[i] & mask[i]) | val[i];
    } else {
    for (i = 0; i < len; i ++)
    inst.scratch[i] = (inst.buffer[i] & ~mask[i])
    | (val[i] & mask[i]);
    }
    }
    static int pmac_i2c_do_rmw(PMF_STD_ARGS, u32 masklen, u32 valuelen,
    u32 totallen, const u8 *maskdata,
    const u8 *valuedata)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    if (masklen > inst.bytes || valuelen > inst.bytes ||
    totallen > inst.bytes || valuelen > masklen)
    return -EINVAL;
    pmac_i2c_do_apply_rmw(inst, masklen, maskdata, valuedata);
    return pmac_i2c_xfer(inst.bus, inst.addr | pmac_i2c_write, 0, 0,
    inst.scratch, totallen);
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_read_sub(_arg: PMF_STD_ARGS, subaddr: u8, len: u32) -> c_int {
    static int pmac_i2c_do_read_sub(PMF_STD_ARGS, u8 subaddr, u32 len)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    inst.bytes = len;
    return pmac_i2c_xfer(inst.bus, inst.addr | pmac_i2c_read, 1, subaddr,
    inst.buffer, len);
    }
    static int pmac_i2c_do_write_sub(PMF_STD_ARGS, u8 subaddr, u32 len,
    const u8 *data)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    return pmac_i2c_xfer(inst.bus, inst.addr | pmac_i2c_write, 1,
    subaddr, (u8 *)data, len);
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_set_mode(_arg: PMF_STD_ARGS, mode: c_int) -> c_int {
    static int pmac_i2c_do_set_mode(PMF_STD_ARGS, int mode)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    return pmac_i2c_setmode(inst.bus, mode);
    }
    static int pmac_i2c_do_rmw_sub(PMF_STD_ARGS, u8 subaddr, u32 masklen,
    u32 valuelen, u32 totallen, const u8 *maskdata,
    const u8 *valuedata)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    if (masklen > inst.bytes || valuelen > inst.bytes ||
    totallen > inst.bytes || valuelen > masklen)
    return -EINVAL;
    pmac_i2c_do_apply_rmw(inst, masklen, maskdata, valuedata);
    return pmac_i2c_xfer(inst.bus, inst.addr | pmac_i2c_write, 1,
    subaddr, inst.scratch, totallen);
    }
    static int pmac_i2c_do_mask_and_comp(PMF_STD_ARGS, u32 len,
    const u8 *maskdata,
    const u8 *valuedata)
    {
    struct pmac_i2c_pf_inst *inst = instdata;
    int i, match;
// Get return value pointer, it's assumed to be a u32
    if (!args || !args.count || !args.u[0].p)
    return -EINVAL;
// Check buffer
    if (len > inst.bytes)
    return -EINVAL;
    for (i = 0, match = 1; match && i < len; i ++)
    if ((inst.buffer[i] & maskdata[i]) != valuedata[i])
    match = 0;
// args->u[0].p = match;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_do_delay(_arg: PMF_STD_ARGS, duration: u32) -> c_int {
    static int pmac_i2c_do_delay(PMF_STD_ARGS, u32 duration)
    {
    msleep((duration + 999) / 1000);
    return 0;
    }
    static struct pmf_handlers pmac_i2c_pfunc_handlers = {
    .begin			= pmac_i2c_do_begin,
    .end			= pmac_i2c_do_end,
    .read_i2c		= pmac_i2c_do_read,
    .write_i2c		= pmac_i2c_do_write,
    .rmw_i2c		= pmac_i2c_do_rmw,
    .read_i2c_sub		= pmac_i2c_do_read_sub,
    .write_i2c_sub		= pmac_i2c_do_write_sub,
    .rmw_i2c_sub		= pmac_i2c_do_rmw_sub,
    .set_i2c_mode		= pmac_i2c_do_set_mode,
    .mask_and_compare	= pmac_i2c_do_mask_and_comp,
    .delay			= pmac_i2c_do_delay,
    };
#[no_mangle]
unsafe extern "C" fn pmac_i2c_dev_create(np: *mut device_node, quirks: c_int) -> void __init {
    static void __init pmac_i2c_dev_create(struct device_node *np, int quirks)
    {
    DBG("dev_create(%pOF)\n", np);
    pmf_register_driver(np, &pmac_i2c_pfunc_handlers,
    (void *)(long)quirks);
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_dev_init(np: *mut device_node, quirks: c_int) -> void __init {
    static void __init pmac_i2c_dev_init(struct device_node *np, int quirks)
    {
    DBG("dev_create(%pOF)\n", np);
    pmf_do_functions(np, core::ptr::null_mut(), 0, PMF_FLAGS_ON_INIT, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_dev_suspend(np: *mut device_node, quirks: c_int) {
    static void pmac_i2c_dev_suspend(struct device_node *np, int quirks)
    {
    DBG("dev_suspend(%pOF)\n", np);
    pmf_do_functions(np, core::ptr::null_mut(), 0, PMF_FLAGS_ON_SLEEP, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pmac_i2c_dev_resume(np: *mut device_node, quirks: c_int) {
    static void pmac_i2c_dev_resume(struct device_node *np, int quirks)
    {
    DBG("dev_resume(%pOF)\n", np);
    pmf_do_functions(np, core::ptr::null_mut(), 0, PMF_FLAGS_ON_WAKE, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_pfunc_i2c_suspend() {
    void pmac_pfunc_i2c_suspend(void)
    {
    pmac_i2c_devscan(pmac_i2c_dev_suspend);
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_pfunc_i2c_resume() {
    void pmac_pfunc_i2c_resume(void)
    {
    pmac_i2c_devscan(pmac_i2c_dev_resume);
    }
//
// Initialize us: probe all i2c busses on the machine, instantiate
// busses and platform functions as needed.
//
// This is non-static as it might be called early by smp code
#[no_mangle]
pub unsafe extern "C" fn pmac_i2c_init() -> int __init {
    int __init pmac_i2c_init(void)
    {
    static int i2c_inited;
    if (i2c_inited)
    return 0;
    i2c_inited = 1;
// Probe keywest-i2c busses
    kw_i2c_probe();

// Probe PMU i2c busses
    pmu_i2c_probe();

// Probe SMU i2c busses
    smu_i2c_probe();

// Now add platform functions for some known devices
    pmac_i2c_devscan(pmac_i2c_dev_create);
    return 0;
    }
    machine_arch_initcall(powermac, pmac_i2c_init);
// Since pmac_i2c_init can be called too early for the platform device
// registration, we need to do it at a later time. In our case, subsys
// happens to fit well, though I agree it's a bit of a hack...
//
#[no_mangle]
unsafe extern "C" fn pmac_i2c_create_platform_devices() -> int __init {
    static int __init pmac_i2c_create_platform_devices(void)
    {
    struct pmac_i2c_bus *bus;
    let mut i: c_int = 0;
// In the case where we are initialized from smp_init(), we must
// not use the timer (and thus the irq). It's safe from now on
// though
//
    pmac_i2c_force_poll = 0;
// Create platform devices
    list_for_each_entry(bus, &pmac_i2c_busses, link) {
    bus.platform_dev =
    platform_device_alloc("i2c-powermac", i++);
    if (bus.platform_dev == core::ptr::null_mut())
    return -ENOMEM;
    bus.platform_dev.dev.platform_data = bus;
    platform_device_set_of_node(bus.platform_dev, bus.busnode);
    platform_device_add(bus.platform_dev);
    }
// Now call platform "init" functions
    pmac_i2c_devscan(pmac_i2c_dev_init);
    return 0;
    }
    machine_subsys_initcall(powermac, pmac_i2c_create_platform_devices);
