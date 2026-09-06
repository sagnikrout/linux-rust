//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/sim710.c
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
// sim710.c - Copyright (C) 1999 Richard Hirst <richard@sleepie.demon.co.uk>
//
// ----------------------------------------------------------------------------
//
// MCA card detection code by Trent McNair. (now deleted)
// Fixes to not explicitly nul bss data from Xavier Bestel.
// Some multiboard fixes from Rolf Eike Beer.
// Auto probing of EISA config space from Trevor Hemsley.
//
// Rewritten to use 53c700.c by James.Bottomley@SteelEye.com
//

// Must be enough for EISA
pub const MAX_SLOTS: c_int = 8;
    static __u8 __initdata id_array[MAX_SLOTS] = { [0 ... MAX_SLOTS-1] = 7 };
    static char *sim710;		/* command line passed by insmod */
    MODULE_AUTHOR("Richard Hirst");
    MODULE_DESCRIPTION("Simple NCR53C710 driver");
    MODULE_LICENSE("GPL");
    module_param(sim710, charp, 0);

    static __init int
    param_setup(char *str)
    {
    char *pos = str, *next;
    let mut slot: c_int = -1;
    while(pos != core::ptr::null_mut() && (next = strchr(pos, ':')) != core::ptr::null_mut()) {
    let mut val: c_int = (int)simple_strtoul(++next, core::ptr::null_mut(), 0);
    if(!strncmp(pos, "slot:", 5))
    slot = val;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(pos, _arg: "id:", _arg: 3)) -> else {
    if(slot == -1) {
    printk(KERN_WARNING "sim710: Must specify slot for id parameter\n");
    } else if(slot >= MAX_SLOTS) {
    printk(KERN_WARNING "sim710: Illegal slot %d for id %d\n", slot, val);
    } else {
    id_array[slot] = val;
    }
    }
    if((pos = strchr(pos, ARG_SEP)) != core::ptr::null_mut())
    pos++;
    }
    return 1;
    }
    __setup("sim710=", param_setup);
    static struct scsi_host_template sim710_driver_template = {
    .name			= "LSI (Symbios) 710 EISA",
    .proc_name		= "sim710",
    .this_id		= 7,
    .module			= THIS_MODULE,
    };
    static int sim710_probe_common(struct device *dev, unsigned long base_addr,
    int irq, int clock, int differential,
    int scsi_id)
    {
    let mut host: *mut Scsi_Host = core::ptr::null_mut();
    struct NCR_700_Host_Parameters *hostdata =
    kzalloc_obj(struct NCR_700_Host_Parameters);
    printk(KERN_NOTICE "sim710: %s\n", dev_name(dev));
    printk(KERN_NOTICE "sim710: irq = %d, clock = %d, base = 0x%lx, scsi_id = %d\n",
    irq, clock, base_addr, scsi_id);
    if(hostdata == core::ptr::null_mut()) {
    printk(KERN_ERR "sim710: Failed to allocate host data\n");
    goto out;
    }
    if(request_region(base_addr, 64, "sim710") == core::ptr::null_mut()) {
    printk(KERN_ERR "sim710: Failed to reserve IO region 0x%lx\n",
    base_addr);
    goto out_free;
    }
// Fill in the three required pieces of hostdata
    hostdata.base = ioport_map(base_addr, 64);
    hostdata.differential = differential;
    hostdata.clock = clock;
    hostdata.chip710 = 1;
    hostdata.burst_length = 8;
// and register the chip
    if((host = NCR_700_detect(&sim710_driver_template, hostdata, dev))
    == core::ptr::null_mut()) {
    printk(KERN_ERR "sim710: No host detected; card configuration problem?\n");
    goto out_release;
    }
    host.this_id = scsi_id;
    host.base = base_addr;
    host.irq = irq;
    if (request_irq(irq, NCR_700_intr, IRQF_SHARED, "sim710", host)) {
    printk(KERN_ERR "sim710: request_irq failed\n");
    goto out_put_host;
    }
    dev_set_drvdata(dev, host);
    scsi_scan_host(host);
    return 0;
    out_put_host:
    scsi_host_put(host);
    out_release:
    ioport_unmap(hostdata.base);
    release_region(base_addr, 64);
    out_free:
    kfree(hostdata);
    out:
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn sim710_device_remove(dev: *mut device) -> c_int {
    static int sim710_device_remove(struct device *dev)
    {
    struct Scsi_Host *host = dev_get_drvdata(dev);
    struct NCR_700_Host_Parameters *hostdata =
    (struct NCR_700_Host_Parameters *)host.hostdata[0];
    scsi_remove_host(host);
    NCR_700_release(host);
    ioport_unmap(hostdata.base);
    kfree(hostdata);
    free_irq(host.irq, host);
    release_region(host.base, 64);
    return 0;
    }

    static struct eisa_device_id sim710_eisa_ids[] = {
    { "CPQ4410" },
    { "CPQ4411" },
    { "HWP0C80" },
    { "" }
    };
    MODULE_DEVICE_TABLE(eisa, sim710_eisa_ids);
#[no_mangle]
unsafe extern "C" fn sim710_eisa_probe(dev: *mut device) -> c_int {
    static int sim710_eisa_probe(struct device *dev)
    {
    struct eisa_device *edev = to_eisa_device(dev);
    let mut io_addr: c_ulong = edev.base_addr;
    char eisa_cpq_irqs[] = { 11, 14, 15, 10, 9, 0 };
    char eisa_hwp_irqs[] = { 3, 4, 5, 7, 12, 10, 11, 0};
    char *eisa_irqs;
    unsigned char irq_index;
    unsigned char irq, differential = 0, scsi_id = 7;
    if(strcmp(edev.id.sig, "HWP0C80") == 0) {
    __u8 val;
    eisa_irqs =  eisa_hwp_irqs;
    irq_index = (inb(io_addr + 0xc85) & 0x7) - 1;
    val = inb(io_addr + 0x4);
    scsi_id = ffs(val) - 1;
    if(scsi_id > 7 || (val & ~(1<<scsi_id)) != 0) {
    printk(KERN_ERR "sim710.c, EISA card %s has incorrect scsi_id, setting to 7\n", dev_name(dev));
    scsi_id = 7;
    }
    } else {
    eisa_irqs = eisa_cpq_irqs;
    irq_index = inb(io_addr + 0xc88) & 0x07;
    }
    if(irq_index >= strlen(eisa_irqs)) {
    printk("sim710.c: irq nasty\n");
    return -ENODEV;
    }
    irq = eisa_irqs[irq_index];
    return sim710_probe_common(dev, io_addr, irq, 50,
    differential, scsi_id);
    }
    static struct eisa_driver sim710_eisa_driver = {
    .id_table		= sim710_eisa_ids,
    .driver = {
    .name		= "sim710",
    .probe		= sim710_eisa_probe,
    .remove		= sim710_device_remove,
    },
    };

#[no_mangle]
unsafe extern "C" fn sim710_init() -> int __init {
    static int __init sim710_init(void)
    {

    if (sim710)
    param_setup(sim710);

//
// FIXME: We'd really like to return -ENODEV if no devices have actually
// been found.  However eisa_driver_register() only reports problems
// with kobject_register() so simply return success for now.
//
    eisa_driver_register(&sim710_eisa_driver);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sim710_exit() -> void __exit {
    static void __exit sim710_exit(void)
    {

    eisa_driver_unregister(&sim710_eisa_driver);

    }
    module_init(sim710_init);
    module_exit(sim710_exit);
