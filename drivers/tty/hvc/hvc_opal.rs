//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvc_opal.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// opal driver interface to hvc_console.c
//
// Copyright 2011 Benjamin Herrenschmidt <benh@kernel.crashing.org>, IBM Corp.
//

    static const char hvc_opal_name[] = "hvc_opal";
    static const struct of_device_id hvc_opal_match[] = {
    { .name = "serial", .compatible = "ibm,opal-console-raw" },
    { .name = "serial", .compatible = "ibm,opal-console-hvsi" },
    { },
    };
    typedef enum hv_protocol {
    HV_PROTOCOL_RAW,
    HV_PROTOCOL_HVSI
    } hv_protocol_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvc_opal_priv {
    pub /: *mut *mut hv_protocol_t proto; / Raw data or HVSI packets,
    pub /: *mut *mut hvsi_priv hvsi; / HVSI specific data,
}

    static struct hvc_opal_priv *hvc_opal_privs[MAX_NR_HVC_CONSOLES];
// For early boot console
    static struct hvc_opal_priv hvc_opal_boot_priv;
    static u32 hvc_opal_boot_termno;
    static const struct hv_ops hvc_opal_raw_ops = {
    .get_chars = opal_get_chars,
    .put_chars = opal_put_chars,
    .flush = opal_flush_chars,
    .notifier_add = notifier_add_irq,
    .notifier_del = notifier_del_irq,
    .notifier_hangup = notifier_hangup_irq,
    };
#[no_mangle]
unsafe extern "C" fn hvc_opal_hvsi_get_chars(vtermno: u32, buf: *mut u8, count: usize) -> isize {
    static ssize_t hvc_opal_hvsi_get_chars(uint32_t vtermno, u8 *buf, size_t count)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[vtermno];
    if (WARN_ON(!pv))
    return -ENODEV;
    return hvsilib_get_chars(&pv.hvsi, buf, count);
    }
    static ssize_t hvc_opal_hvsi_put_chars(uint32_t vtermno, const u8 *buf,
    size_t count)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[vtermno];
    if (WARN_ON(!pv))
    return -ENODEV;
    return hvsilib_put_chars(&pv.hvsi, buf, count);
    }
#[no_mangle]
unsafe extern "C" fn hvc_opal_hvsi_open(hp: *mut hvc_struct, data: c_int) -> c_int {
    static int hvc_opal_hvsi_open(struct hvc_struct *hp, int data)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[hp.vtermno];
    int rc;
    pr_devel("HVSI@%x: do open !\n", hp.vtermno);
    rc = notifier_add_irq(hp, data);
    if (rc)
    return rc;
    return hvsilib_open(&pv.hvsi, hp);
    }
#[no_mangle]
unsafe extern "C" fn hvc_opal_hvsi_close(hp: *mut hvc_struct, data: c_int) {
    static void hvc_opal_hvsi_close(struct hvc_struct *hp, int data)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[hp.vtermno];
    pr_devel("HVSI@%x: do close !\n", hp.vtermno);
    hvsilib_close(&pv.hvsi, hp);
    notifier_del_irq(hp, data);
    }
#[no_mangle]
unsafe extern "C" fn hvc_opal_hvsi_hangup(hp: *mut hvc_struct, data: c_int) {
    static void hvc_opal_hvsi_hangup(struct hvc_struct *hp, int data)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[hp.vtermno];
    pr_devel("HVSI@%x: do hangup !\n", hp.vtermno);
    hvsilib_close(&pv.hvsi, hp);
    notifier_hangup_irq(hp, data);
    }
#[no_mangle]
unsafe extern "C" fn hvc_opal_hvsi_tiocmget(hp: *mut hvc_struct) -> c_int {
    static int hvc_opal_hvsi_tiocmget(struct hvc_struct *hp)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[hp.vtermno];
    if (!pv)
    return -EINVAL;
    return pv.hvsi.mctrl;
    }
    static int hvc_opal_hvsi_tiocmset(struct hvc_struct *hp, unsigned int set,
    unsigned int clear)
    {
    struct hvc_opal_priv *pv = hvc_opal_privs[hp.vtermno];
    pr_devel("HVSI@%x: Set modem control, set=%x,clr=%x\n",
    hp.vtermno, set, clear);
    if (set & TIOCM_DTR)
    hvsilib_write_mctrl(&pv.hvsi, 1);
#[no_mangle]
pub unsafe extern "C" fn if(TIOCM_DTR: clear &) -> else {
    else if (clear & TIOCM_DTR)
    hvsilib_write_mctrl(&pv.hvsi, 0);
    return 0;
    }
    static const struct hv_ops hvc_opal_hvsi_ops = {
    .get_chars = hvc_opal_hvsi_get_chars,
    .put_chars = hvc_opal_hvsi_put_chars,
    .flush = opal_flush_chars,
    .notifier_add = hvc_opal_hvsi_open,
    .notifier_del = hvc_opal_hvsi_close,
    .notifier_hangup = hvc_opal_hvsi_hangup,
    .tiocmget = hvc_opal_hvsi_tiocmget,
    .tiocmset = hvc_opal_hvsi_tiocmset,
    };
#[no_mangle]
unsafe extern "C" fn hvc_opal_probe(dev: *mut platform_device) -> c_int {
    static int hvc_opal_probe(struct platform_device *dev)
    {
    const struct hv_ops *ops;
    struct hvc_struct *hp;
    struct hvc_opal_priv *pv;
    hv_protocol_t proto;
    unsigned int termno, irq, boot = 0;
    const __be32 *reg;
    if (of_device_is_compatible(dev.dev.of_node, "ibm,opal-console-raw")) {
    proto = HV_PROTOCOL_RAW;
    ops = &hvc_opal_raw_ops;
    } else if (of_device_is_compatible(dev.dev.of_node,
    "ibm,opal-console-hvsi")) {
    proto = HV_PROTOCOL_HVSI;
    ops = &hvc_opal_hvsi_ops;
    } else {
    pr_err("hvc_opal: Unknown protocol for %pOF\n",
    dev.dev.of_node);
    return -ENXIO;
    }
    reg = of_get_property(dev.dev.of_node, "reg", core::ptr::null_mut());
    termno = reg ? be32_to_cpup(reg) : 0;
// Is it our boot one ?
    if (hvc_opal_privs[termno] == &hvc_opal_boot_priv) {
    pv = hvc_opal_privs[termno];
    boot = 1;
    } else if (hvc_opal_privs[termno] == core::ptr::null_mut()) {
    pv = kzalloc_obj(struct hvc_opal_priv);
    if (!pv)
    return -ENOMEM;
    pv.proto = proto;
    hvc_opal_privs[termno] = pv;
    if (proto == HV_PROTOCOL_HVSI) {
//
// We want put_chars to be atomic to avoid mangling of
// hvsi packets.
//
    hvsilib_init(&pv.hvsi,
    opal_get_chars, opal_put_chars_atomic,
    termno, 0);
    }
// Instanciate now to establish a mapping index==vtermno
    hvc_instantiate(termno, termno, ops);
    } else {
    pr_err("hvc_opal: Device %pOF has duplicate terminal number #%d\n",
    dev.dev.of_node, termno);
    return -ENXIO;
    }
    pr_info("hvc%d: %s protocol on %pOF%s\n", termno,
    proto == HV_PROTOCOL_RAW ? "raw" : "hvsi",
    dev.dev.of_node,
    boot ? " (boot console)" : "");
    irq = irq_of_parse_and_map(dev.dev.of_node, 0);
    if (!irq) {
    pr_info("hvc%d: No interrupts property, using OPAL event\n",
    termno);
    irq = opal_event_request(ilog2(OPAL_EVENT_CONSOLE_INPUT));
    }
    if (!irq) {
    pr_err("hvc_opal: Unable to map interrupt for device %pOF\n",
    dev.dev.of_node);
    return irq;
    }
    hp = hvc_alloc(termno, irq, ops, MAX_VIO_PUT_CHARS);
    if (IS_ERR(hp))
    return PTR_ERR(hp);
// hvc consoles on powernv may need to share a single irq
    hp.flags = IRQF_SHARED;
    dev_set_drvdata(&dev.dev, hp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hvc_opal_remove(dev: *mut platform_device) {
    static void hvc_opal_remove(struct platform_device *dev)
    {
    struct hvc_struct *hp = dev_get_drvdata(&dev.dev);
    int termno;
    termno = hp.vtermno;
    hvc_remove(hp);
    if (hvc_opal_privs[termno] != &hvc_opal_boot_priv)
    kfree(hvc_opal_privs[termno]);
    hvc_opal_privs[termno] = core::ptr::null_mut();
    }
    static struct platform_driver hvc_opal_driver = {
    .probe		= hvc_opal_probe,
    .remove		= hvc_opal_remove,
    .driver		= {
    .name	= hvc_opal_name,
    .of_match_table	= hvc_opal_match,
    }
    };
#[no_mangle]
unsafe extern "C" fn hvc_opal_init() -> int __init {
    static int __init hvc_opal_init(void)
    {
    if (!firmware_has_feature(FW_FEATURE_OPAL))
    return -ENODEV;
// Register as a vio device to receive callbacks
    return platform_driver_register(&hvc_opal_driver);
    }
    device_initcall(hvc_opal_init);
#[no_mangle]
unsafe extern "C" fn udbg_opal_putc(c: c_char) {
    static void udbg_opal_putc(char c)
    {
    let mut termno: c_uint = hvc_opal_boot_termno;
    let mut count: c_int = -1;
    if (c == '\n')
    udbg_opal_putc('\r');
    do {
    switch(hvc_opal_boot_priv.proto) {
    case HV_PROTOCOL_RAW:
    count = opal_put_chars(termno, &c, 1);
    break;
    case HV_PROTOCOL_HVSI:
    count = hvc_opal_hvsi_put_chars(termno, &c, 1);
    break;
    }
// This is needed for the cosole to flush
// when there aren't any interrupts.
//
    opal_flush_console(termno);
    } while(count == 0 || count == -EAGAIN);
    }
#[no_mangle]
unsafe extern "C" fn udbg_opal_getc_poll() -> c_int {
    static int udbg_opal_getc_poll(void)
    {
    let mut termno: c_uint = hvc_opal_boot_termno;
    let mut rc: c_int = 0;
    char c;
    switch(hvc_opal_boot_priv.proto) {
    case HV_PROTOCOL_RAW:
    rc = opal_get_chars(termno, &c, 1);
    break;
    case HV_PROTOCOL_HVSI:
    rc = hvc_opal_hvsi_get_chars(termno, &c, 1);
    break;
    }
    if (!rc)
    return -1;
    return c;
    }
#[no_mangle]
unsafe extern "C" fn udbg_opal_getc() -> c_int {
    static int udbg_opal_getc(void)
    {
    int ch;
    for (;;) {
    ch = udbg_opal_getc_poll();
    if (ch != -1)
    return ch;
    }
    }
#[no_mangle]
unsafe extern "C" fn udbg_init_opal_common() {
    static void udbg_init_opal_common(void)
    {
    udbg_putc = udbg_opal_putc;
    udbg_getc = udbg_opal_getc;
    udbg_getc_poll = udbg_opal_getc_poll;
    }
#[no_mangle]
pub unsafe extern "C" fn hvc_opal_init_early() -> void __init {
    void __init hvc_opal_init_early(void)
    {
    struct device_node *stdout_node = of_node_get(of_stdout);
    const __be32 *termno;
    const struct hv_ops *ops;
    u32 index;
// If the console wasn't in /chosen, try /ibm,opal
    if (!stdout_node) {
    struct device_node *opal, *np;
// Current OPAL takeover doesn't provide the stdout
// path, so we hard wire it
//
    opal = of_find_node_by_path("/ibm,opal/consoles");
    if (opal) {
    pr_devel("hvc_opal: Found consoles in new location\n");
    } else {
    opal = of_find_node_by_path("/ibm,opal");
    if (opal)
    pr_devel("hvc_opal: "
    "Found consoles in old location\n");
    }
    if (!opal)
    return;
    for_each_child_of_node(opal, np) {
    if (of_node_name_eq(np, "serial")) {
    stdout_node = np;
    break;
    }
    }
    of_node_put(opal);
    }
    if (!stdout_node)
    return;
    termno = of_get_property(stdout_node, "reg", core::ptr::null_mut());
    index = termno ? be32_to_cpup(termno) : 0;
    if (index >= MAX_NR_HVC_CONSOLES)
    return;
    hvc_opal_privs[index] = &hvc_opal_boot_priv;
// Check the protocol
    if (of_device_is_compatible(stdout_node, "ibm,opal-console-raw")) {
    hvc_opal_boot_priv.proto = HV_PROTOCOL_RAW;
    ops = &hvc_opal_raw_ops;
    pr_devel("hvc_opal: Found RAW console\n");
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_device_is_compatible(stdout_node, _arg: "ibm, _arg: opal-console-hvsi")) -> else {
    hvc_opal_boot_priv.proto = HV_PROTOCOL_HVSI;
    ops = &hvc_opal_hvsi_ops;
    hvsilib_init(&hvc_opal_boot_priv.hvsi,
    opal_get_chars, opal_put_chars_atomic,
    index, 1);
// HVSI, perform the handshake now
    hvsilib_establish(&hvc_opal_boot_priv.hvsi);
    pr_devel("hvc_opal: Found HVSI console\n");
    } else
    goto out;
    hvc_opal_boot_termno = index;
    udbg_init_opal_common();
    add_preferred_console("hvc", index, core::ptr::null_mut());
    hvc_instantiate(index, index, ops);
    out:
    of_node_put(stdout_node);
    }

#[no_mangle]
pub unsafe extern "C" fn udbg_init_debug_opal_raw() -> void __init {
    void __init udbg_init_debug_opal_raw(void)
    {
    let mut index: u32 = CONFIG_PPC_EARLY_DEBUG_OPAL_VTERMNO;
    hvc_opal_privs[index] = &hvc_opal_boot_priv;
    hvc_opal_boot_priv.proto = HV_PROTOCOL_RAW;
    hvc_opal_boot_termno = index;
    udbg_init_opal_common();
    }

#[no_mangle]
pub unsafe extern "C" fn udbg_init_debug_opal_hvsi() -> void __init {
    void __init udbg_init_debug_opal_hvsi(void)
    {
    let mut index: u32 = CONFIG_PPC_EARLY_DEBUG_OPAL_VTERMNO;
    hvc_opal_privs[index] = &hvc_opal_boot_priv;
    hvc_opal_boot_termno = index;
    udbg_init_opal_common();
    hvsilib_init(&hvc_opal_boot_priv.hvsi,
    opal_get_chars, opal_put_chars_atomic,
    index, 1);
    hvsilib_establish(&hvc_opal_boot_priv.hvsi);
    }
