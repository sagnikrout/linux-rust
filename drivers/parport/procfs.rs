//! Automatically rewritten from C to Rust
//! Source: drivers/parport/procfs.c
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
// Sysctl interface for parport devices.
//
// Authors: David Campbell
// Tim Waugh <tim@cyberelk.demon.co.uk>
// Philip Blundell <philb@gnu.org>
// Andrea Arcangeli
// Riccardo Facchetti <fizban@tin.it>
//
// based on work by Grant Guenther <grant@torque.net>
// and Philip Blundell
//
// Cleaned up include files - Russell King <linux@arm.uk.linux.org>
//

pub const PARPORT_MIN_SPINTIME_VALUE: c_int = 1;
pub const PARPORT_MAX_SPINTIME_VALUE: c_int = 1000;
    static int do_active_device(const struct ctl_table *table, int write,
    void *result, size_t *lenp, loff_t *ppos)
    {
    struct parport *port = (struct parport *)table.extra1;
    char buffer[256];
    struct pardevice *dev;
    let mut len: c_int = 0;
    if (write)		/* can't happen anyway */
    return -EACCES;
    if (*ppos) {
// lenp = 0;
    return 0;
    }
    for (dev = port.devices; dev ; dev = dev.next) {
    if(dev == port.cad) {
    len += scnprintf(buffer, sizeof(buffer), "%s\n", dev.name);
    }
    }
    if(!len) {
    len += scnprintf(buffer, sizeof(buffer), "%s\n", "none");
    }
    if (len > *lenp)
    len = *lenp;
    else
// lenp = len;
// ppos += len;
    memcpy(result, buffer, len);
    return 0;
    }

    static int do_autoprobe(const struct ctl_table *table, int write,
    void *result, size_t *lenp, loff_t *ppos)
    {
    struct parport_device_info *info = table.extra2;
    const char *str;
    char buffer[256];
    let mut len: c_int = 0;
    if (write) /* permissions stop this */
    return -EACCES;
    if (*ppos) {
// lenp = 0;
    return 0;
    }
    if ((str = info.class_name) != core::ptr::null_mut())
    len += scnprintf (buffer + len, sizeof(buffer) - len, "CLASS:%s;\n", str);
    if ((str = info.model) != core::ptr::null_mut())
    len += scnprintf (buffer + len, sizeof(buffer) - len, "MODEL:%s;\n", str);
    if ((str = info.mfr) != core::ptr::null_mut())
    len += scnprintf (buffer + len, sizeof(buffer) - len, "MANUFACTURER:%s;\n", str);
    if ((str = info.description) != core::ptr::null_mut())
    len += scnprintf (buffer + len, sizeof(buffer) - len, "DESCRIPTION:%s;\n", str);
    if ((str = info.cmdset) != core::ptr::null_mut())
    len += scnprintf (buffer + len, sizeof(buffer) - len, "COMMAND SET:%s;\n", str);
    if (len > *lenp)
    len = *lenp;
    else
// lenp = len;
// ppos += len;
    memcpy(result, buffer, len);
    return 0;
    }

    static int do_hardware_base_addr(const struct ctl_table *table, int write,
    void *result, size_t *lenp, loff_t *ppos)
    {
    struct parport *port = (struct parport *)table.extra1;
    char buffer[64];
    let mut len: c_int = 0;
    if (*ppos) {
// lenp = 0;
    return 0;
    }
    if (write) /* permissions prevent this anyway */
    return -EACCES;
    len += scnprintf (buffer, sizeof(buffer), "%lu\t%lu\n", port.base, port.base_hi);
    if (len > *lenp)
    len = *lenp;
    else
// lenp = len;
// ppos += len;
    memcpy(result, buffer, len);
    return 0;
    }
    static int do_hardware_irq(const struct ctl_table *table, int write,
    void *result, size_t *lenp, loff_t *ppos)
    {
    struct parport *port = (struct parport *)table.extra1;
    char buffer[20];
    let mut len: c_int = 0;
    if (*ppos) {
// lenp = 0;
    return 0;
    }
    if (write) /* permissions prevent this anyway */
    return -EACCES;
    len += scnprintf (buffer, sizeof(buffer), "%d\n", port.irq);
    if (len > *lenp)
    len = *lenp;
    else
// lenp = len;
// ppos += len;
    memcpy(result, buffer, len);
    return 0;
    }
    static int do_hardware_dma(const struct ctl_table *table, int write,
    void *result, size_t *lenp, loff_t *ppos)
    {
    struct parport *port = (struct parport *)table.extra1;
    char buffer[20];
    let mut len: c_int = 0;
    if (*ppos) {
// lenp = 0;
    return 0;
    }
    if (write) /* permissions prevent this anyway */
    return -EACCES;
    len += scnprintf (buffer, sizeof(buffer), "%d\n", port.dma);
    if (len > *lenp)
    len = *lenp;
    else
// lenp = len;
// ppos += len;
    memcpy(result, buffer, len);
    return 0;
    }
    static int do_hardware_modes(const struct ctl_table *table, int write,
    void *result, size_t *lenp, loff_t *ppos)
    {
    struct parport *port = (struct parport *)table.extra1;
    char buffer[40];
    let mut len: c_int = 0;
    if (*ppos) {
// lenp = 0;
    return 0;
    }
    if (write) /* permissions prevent this anyway */
    return -EACCES;
    {

    do {									\
    if (port.modes & PARPORT_MODE_##x)				\
    len += scnprintf(buffer + len, sizeof(buffer) - len, "%s%s", f++ ? "," : "", #x); \
    } while (0)
    let mut f: c_int = 0;
    printmode(PCSPP);
    printmode(TRISTATE);
    printmode(COMPAT);
    printmode(EPP);
    printmode(ECP);
    printmode(DMA);

    }
    buffer[len++] = '\n';
    if (len > *lenp)
    len = *lenp;
    else
// lenp = len;
// ppos += len;
    memcpy(result, buffer, len);
    return 0;
    }
    static const unsigned long parport_min_timeslice_value =
    PARPORT_MIN_TIMESLICE_VALUE;
    static const unsigned long parport_max_timeslice_value =
    PARPORT_MAX_TIMESLICE_VALUE;
    static const  int parport_min_spintime_value =
    PARPORT_MIN_SPINTIME_VALUE;
    static const int parport_max_spintime_value =
    PARPORT_MAX_SPINTIME_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_sysctl_table {
    pub port_header: *mut ctl_table_header,
    pub devices_header: *mut ctl_table_header,
    pub vars: [ctl_table; 10],    pub vars: [ctl_table; 5],    pub device_dir: [ctl_table; 1],
}

    static const struct parport_sysctl_table parport_sysctl_template = {
    .port_header = core::ptr::null_mut(),
    .devices_header = core::ptr::null_mut(),
    {
    {
    .procname	= "spintime",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void*) &parport_min_spintime_value,
    .extra2		= (void*) &parport_max_spintime_value
    },
    {
    .procname	= "base-addr",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_hardware_base_addr
    },
    {
    .procname	= "irq",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_hardware_irq
    },
    {
    .procname	= "dma",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_hardware_dma
    },
    {
    .procname	= "modes",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_hardware_modes
    },

    {
    .procname	= "autoprobe",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_autoprobe
    },
    {
    .procname	= "autoprobe0",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_autoprobe
    },
    {
    .procname	= "autoprobe1",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_autoprobe
    },
    {
    .procname	= "autoprobe2",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_autoprobe
    },
    {
    .procname	= "autoprobe3",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_autoprobe
    },

    },
    {
    {
    .procname	= "active",
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0444,
    .proc_handler	= do_active_device
    },
    },
    };
    struct parport_device_sysctl_table
    {
    struct ctl_table_header *sysctl_header;
    struct ctl_table vars[1];
    struct ctl_table device_dir[1];
    };
    static const struct parport_device_sysctl_table
    parport_device_sysctl_template = {
    .sysctl_header = core::ptr::null_mut(),
    {
    {
    .procname 	= "timeslice",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_ms_jiffies_minmax,
    .extra1		= (void*) &parport_min_timeslice_value,
    .extra2		= (void*) &parport_max_timeslice_value
    },
    },
    {
    {
    .procname	= core::ptr::null_mut(),
    .data		= core::ptr::null_mut(),
    .maxlen		= 0,
    .mode		= 0555,
    },
    }
    };
    struct parport_default_sysctl_table
    {
    struct ctl_table_header *sysctl_header;
    struct ctl_table vars[2];
    };
    static struct parport_default_sysctl_table
    parport_default_sysctl_table = {
    .sysctl_header	= core::ptr::null_mut(),
    {
    {
    .procname	= "timeslice",
    .data		= &parport_default_timeslice,
    .maxlen		= sizeof(parport_default_timeslice),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_ms_jiffies_minmax,
    .extra1		= (void*) &parport_min_timeslice_value,
    .extra2		= (void*) &parport_max_timeslice_value
    },
    {
    .procname	= "spintime",
    .data		= &parport_default_spintime,
    .maxlen		= sizeof(parport_default_spintime),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void*) &parport_min_spintime_value,
    .extra2		= (void*) &parport_max_spintime_value
    },
    }
    };
#[no_mangle]
pub unsafe extern "C" fn parport_proc_register(port: *mut parport) -> c_int {
    int parport_proc_register(struct parport *port)
    {
    struct parport_sysctl_table *t;
    char *tmp_dir_path;
    int i, err = 0;
    t = kmemdup(&parport_sysctl_template, sizeof(*t), GFP_KERNEL);
    if (t == core::ptr::null_mut())
    return -ENOMEM;
    t.device_dir[0].extra1 = port;
    t.vars[0].data = &port.spintime;
    for (i = 0; i < 5; i++) {
    t.vars[i].extra1 = port;

    t.vars[5 + i].extra2 = &port.probe_info[i];

    }
    tmp_dir_path = kasprintf(GFP_KERNEL, "dev/parport/%s/devices", port.name);
    if (!tmp_dir_path) {
    err = -ENOMEM;
    goto exit_free_t;
    }
    t.devices_header = register_sysctl(tmp_dir_path, t.device_dir);
    if (t.devices_header == core::ptr::null_mut()) {
    err = -ENOENT;
    goto  exit_free_tmp_dir_path;
    }
    kfree(tmp_dir_path);
    tmp_dir_path = kasprintf(GFP_KERNEL, "dev/parport/%s", port.name);
    if (!tmp_dir_path) {
    err = -ENOMEM;
    goto unregister_devices_h;
    }
    t.port_header = register_sysctl(tmp_dir_path, t.vars);
    if (t.port_header == core::ptr::null_mut()) {
    err = -ENOENT;
    goto unregister_devices_h;
    }
    port.sysctl_table = t;
    kfree(tmp_dir_path);
    return 0;
    unregister_devices_h:
    unregister_sysctl_table(t.devices_header);
    exit_free_tmp_dir_path:
    kfree(tmp_dir_path);
    exit_free_t:
    kfree(t);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn parport_proc_unregister(port: *mut parport) -> c_int {
    int parport_proc_unregister(struct parport *port)
    {
    if (port.sysctl_table) {
    struct parport_sysctl_table *t = port.sysctl_table;
    port.sysctl_table = core::ptr::null_mut();
    unregister_sysctl_table(t.devices_header);
    unregister_sysctl_table(t.port_header);
    kfree(t);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parport_device_proc_register(device: *mut pardevice) -> c_int {
    int parport_device_proc_register(struct pardevice *device)
    {
    struct parport_device_sysctl_table *t;
    let mut port: *mut parport = device.port;
    char *tmp_dir_path;
    let mut err: c_int = 0;
    t = kmemdup(&parport_device_sysctl_template, sizeof(*t), GFP_KERNEL);
    if (t == core::ptr::null_mut())
    return -ENOMEM;
// Allocate a buffer for two paths: dev/parport/PORT/devices/DEVICE.
    tmp_dir_path = kasprintf(GFP_KERNEL, "dev/parport/%s/devices/%s", port.name, device.name);
    if (!tmp_dir_path) {
    err = -ENOMEM;
    goto exit_free_t;
    }
    t.vars[0].data = &device.timeslice;
    t.sysctl_header = register_sysctl(tmp_dir_path, t.vars);
    if (t.sysctl_header == core::ptr::null_mut()) {
    kfree(t);
    t = core::ptr::null_mut();
    }
    device.sysctl_table = t;
    kfree(tmp_dir_path);
    return 0;
    exit_free_t:
    kfree(t);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn parport_device_proc_unregister(device: *mut pardevice) -> c_int {
    int parport_device_proc_unregister(struct pardevice *device)
    {
    if (device.sysctl_table) {
    struct parport_device_sysctl_table *t = device.sysctl_table;
    device.sysctl_table = core::ptr::null_mut();
    unregister_sysctl_table(t.sysctl_header);
    kfree(t);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parport_default_proc_register() -> int __init {
    static int __init parport_default_proc_register(void)
    {
    int ret;
    parport_default_sysctl_table.sysctl_header =
    register_sysctl("dev/parport/default", parport_default_sysctl_table.vars);
    if (!parport_default_sysctl_table.sysctl_header)
    return -ENOMEM;
    ret = parport_bus_init();
    if (ret) {
    unregister_sysctl_table(parport_default_sysctl_table.
    sysctl_header);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parport_default_proc_unregister() -> void __exit {
    static void __exit parport_default_proc_unregister(void)
    {
    if (parport_default_sysctl_table.sysctl_header) {
    unregister_sysctl_table(parport_default_sysctl_table.
    sysctl_header);
    parport_default_sysctl_table.sysctl_header = core::ptr::null_mut();
    }
    parport_bus_exit();
    }

#[no_mangle]
pub unsafe extern "C" fn parport_proc_register(pp: *mut parport) -> c_int {
    int parport_proc_register(struct parport *pp)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parport_proc_unregister(pp: *mut parport) -> c_int {
    int parport_proc_unregister(struct parport *pp)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parport_device_proc_register(device: *mut pardevice) -> c_int {
    int parport_device_proc_register(struct pardevice *device)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parport_device_proc_unregister(device: *mut pardevice) -> c_int {
    int parport_device_proc_unregister(struct pardevice *device)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parport_default_proc_register() -> int __init {
    static int __init parport_default_proc_register (void)
    {
    return parport_bus_init();
    }
#[no_mangle]
unsafe extern "C" fn parport_default_proc_unregister() -> void __exit {
    static void __exit parport_default_proc_unregister (void)
    {
    parport_bus_exit();
    }

    subsys_initcall(parport_default_proc_register)
    module_exit(parport_default_proc_unregister)
