//! Automatically rewritten from C to Rust
//! Source: drivers/pci/npem.c
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
// PCIe Enclosure management driver created for LED interfaces based on
// indications. It says *what indications* blink but does not specify *how
// they blink - it is hardware defined.
//
// The driver name refers to Native PCIe Enclosure Management. It is
// first indication oriented standard with specification.
//
// Native PCIe Enclosure Management (NPEM)
// PCIe Base Specification r6.1 sec 6.28, 7.9.19
//
// _DSM Definitions for PCIe SSD Status LED
// PCI Firmware Specification, r3.3 sec 4.7
//
// Two backends are supported to manipulate indications: Direct NPEM register
// access (npem_ops) and indirect access through the ACPI _DSM (dsm_ops).
// _DSM is used if supported, else NPEM.
//
// Copyright (c) 2021-2022 Dell Inc.
// Copyright (c) 2023-2024 Intel Corporation
// Mariusz Tkaczyk <mariusz.tkaczyk@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct indication {
    pub bit: u32,
    pub name: *const c_char,
}

    static const struct indication npem_indications[] = {
    {PCI_NPEM_IND_OK,	"enclosure:ok"},
    {PCI_NPEM_IND_LOCATE,	"enclosure:locate"},
    {PCI_NPEM_IND_FAIL,	"enclosure:fail"},
    {PCI_NPEM_IND_REBUILD,	"enclosure:rebuild"},
    {PCI_NPEM_IND_PFA,	"enclosure:pfa"},
    {PCI_NPEM_IND_HOTSPARE,	"enclosure:hotspare"},
    {PCI_NPEM_IND_ICA,	"enclosure:ica"},
    {PCI_NPEM_IND_IFA,	"enclosure:ifa"},
    {PCI_NPEM_IND_IDT,	"enclosure:idt"},
    {PCI_NPEM_IND_DISABLED,	"enclosure:disabled"},
    {PCI_NPEM_IND_SPEC_0,	"enclosure:specific_0"},
    {PCI_NPEM_IND_SPEC_1,	"enclosure:specific_1"},
    {PCI_NPEM_IND_SPEC_2,	"enclosure:specific_2"},
    {PCI_NPEM_IND_SPEC_3,	"enclosure:specific_3"},
    {PCI_NPEM_IND_SPEC_4,	"enclosure:specific_4"},
    {PCI_NPEM_IND_SPEC_5,	"enclosure:specific_5"},
    {PCI_NPEM_IND_SPEC_6,	"enclosure:specific_6"},
    {PCI_NPEM_IND_SPEC_7,	"enclosure:specific_7"},
    {0,			core::ptr::null_mut()}
    };
// _DSM PCIe SSD LED States correspond to NPEM register values
    static const struct indication dsm_indications[] = {
    {PCI_NPEM_IND_OK,	"enclosure:ok"},
    {PCI_NPEM_IND_LOCATE,	"enclosure:locate"},
    {PCI_NPEM_IND_FAIL,	"enclosure:fail"},
    {PCI_NPEM_IND_REBUILD,	"enclosure:rebuild"},
    {PCI_NPEM_IND_PFA,	"enclosure:pfa"},
    {PCI_NPEM_IND_HOTSPARE,	"enclosure:hotspare"},
    {PCI_NPEM_IND_ICA,	"enclosure:ica"},
    {PCI_NPEM_IND_IFA,	"enclosure:ifa"},
    {PCI_NPEM_IND_IDT,	"enclosure:idt"},
    {PCI_NPEM_IND_DISABLED,	"enclosure:disabled"},
    {0,			core::ptr::null_mut()}
    };

    for (ind = inds; ind.bit; ind++)
//
// The driver has internal list of supported indications. Ideally, the driver
// should not touch bits that are not defined and for which LED devices are
// not exposed but in reality, it needs to turn them off.
//
// Otherwise, there will be no possibility to turn off indications turned on by
// other utilities or turned on by default and it leads to bad user experience.
//
// Additionally, it excludes NPEM commands like RESET or ENABLE.
//
#[no_mangle]
unsafe extern "C" fn reg_to_indications(caps: u32, inds: *const indication) -> u32 {
    static u32 reg_to_indications(u32 caps, const struct indication *inds)
    {
    const struct indication *ind;
    let mut supported_indications: u32 = 0;
    for_each_indication(ind, inds)
    supported_indications |= ind.bit;
    return caps & supported_indications;
    }
//
// struct npem_led - LED details
// @indication: indication details
// @npem: NPEM device
// @name: LED name
// @led: LED device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npem_led {
    pub indication: *const indication,
    pub npem: *mut npem,
    pub name: [c_char; LED_MAX_NAME_SIZE],
    pub led: led_classdev,
}

//
// struct npem_ops - backend specific callbacks
// @get_active_indications: get active indications
// npem: NPEM device
// inds: response buffer
// @set_active_indications: set new indications
// npem: npem device
// inds: bit mask to set
// @inds: supported indications array, set of indications is backend specific
// @name: backend name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npem_ops {
    pub inds): *mut *mut *mut int (get_active_indications)(struct npem npem, u32,
    pub inds): *mut *mut *mut int (set_active_indications)(struct npem npem, u32,
    pub inds: *const indication,
    pub name: *const c_char,
}

//
// struct npem - NPEM device properties
// @dev: PCI device this driver is attached to
// @ops: backend specific callbacks
// @lock: serializes concurrent access to NPEM device by multiple LED devices
// @pos: cached offset of NPEM Capability Register in Configuration Space;
// only used if NPEM registers are accessed directly and not through _DSM
// @supported_indications: cached bit mask of supported indications;
// non-indication and reserved bits in the NPEM Capability Register are
// cleared in this bit mask
// @active_indications: cached bit mask of active indications;
// non-indication and reserved bits in the NPEM Control Register are
// cleared in this bit mask
// @active_inds_initialized: whether @active_indications has been initialized;
// On Dell platforms, it is required that IPMI drivers are loaded before
// the GET_STATE_DSM method is invoked: They use an IPMI OpRegion to
// get/set the active LEDs. By initializing @active_indications lazily
// (on first access to an LED), IPMI drivers are given a chance to load.
// If they are not loaded in time, users will see various errors on LED
// access in dmesg. Once they are loaded, the errors go away and LED
// access becomes possible.
// @led_cnt: size of @leds array
// @leds: array containing LED class devices of all supported LEDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npem {
    pub dev: *mut pci_dev,
    pub ops: *const npem_ops,
    pub lock: mutex,
    pub pos: u16,
    pub supported_indications: u32,
    pub active_indications: u32,
    pub active_inds_initialized:1: c_uint,
    pub led_cnt: c_int,
    pub leds: [npem_led; ],
}

#[no_mangle]
unsafe extern "C" fn npem_read_reg(npem: *mut npem, reg: u16, val: *mut u32) -> c_int {
    static int npem_read_reg(struct npem *npem, u16 reg, u32 *val)
    {
    let mut ret: c_int = pci_read_config_dword(npem.dev, npem.pos + reg, val);
    return pcibios_err_to_errno(ret);
    }
#[no_mangle]
unsafe extern "C" fn npem_write_ctrl(npem: *mut npem, reg: u32) -> c_int {
    static int npem_write_ctrl(struct npem *npem, u32 reg)
    {
    let mut pos: c_int = npem.pos + PCI_NPEM_CTRL;
    let mut ret: c_int = pci_write_config_dword(npem.dev, pos, reg);
    return pcibios_err_to_errno(ret);
    }
#[no_mangle]
unsafe extern "C" fn npem_get_active_indications(npem: *mut npem, inds: *mut u32) -> c_int {
    static int npem_get_active_indications(struct npem *npem, u32 *inds)
    {
    u32 ctrl;
    int ret;
    ret = npem_read_reg(npem, PCI_NPEM_CTRL, &ctrl);
    if (ret)
    return ret;
// If PCI_NPEM_CTRL_ENABLE is not set then no indication should blink
    if (!(ctrl & PCI_NPEM_CTRL_ENABLE)) {
// inds = 0;
    return 0;
    }
// inds = ctrl & npem->supported_indications;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npem_set_active_indications(npem: *mut npem, inds: u32) -> c_int {
    static int npem_set_active_indications(struct npem *npem, u32 inds)
    {
    int ctrl, ret, ret_val;
    u32 cc_status;
    lockdep_assert_held(&npem.lock);
// This bit is always required
    ctrl = inds | PCI_NPEM_CTRL_ENABLE;
    ret = npem_write_ctrl(npem, ctrl);
    if (ret)
    return ret;
//
// For the case where a NPEM command has not completed immediately,
// it is recommended that software not continuously "spin" on polling
// the status register, but rather poll under interrupt at a reduced
// rate; for example at 10 ms intervals.
//
// PCIe r6.1 sec 6.28 "Implementation Note: Software Polling of NPEM
// Command Completed"
//
    ret = read_poll_timeout(npem_read_reg, ret_val,
    ret_val || (cc_status & PCI_NPEM_STATUS_CC),
    10 * USEC_PER_MSEC, USEC_PER_SEC, false, npem,
    PCI_NPEM_STATUS, &cc_status);
    if (ret)
    return ret;
    if (ret_val)
    return ret_val;
//
// All writes to control register, including writes that do not change
// the register value, are NPEM commands and should eventually result
// in a command completion indication in the NPEM Status Register.
//
// PCIe Base Specification r6.1 sec 7.9.19.3
//
// Register may not be updated, or other conflicting bits may be
// cleared. Spec is not strict here. Read NPEM Control register after
// write to keep cache in-sync.
//
    return npem_get_active_indications(npem, &npem.active_indications);
    }
    static const struct npem_ops npem_ops = {
    .get_active_indications = npem_get_active_indications,
    .set_active_indications = npem_set_active_indications,
    .name = "Native PCIe Enclosure Management",
    .inds = npem_indications,
    };

    0xd5, 0x1e, 0x19, 0x4d)
pub const GET_SUPPORTED_STATES_DSM: c_int = 1;
pub const GET_STATE_DSM: c_int = 2;
pub const SET_STATE_DSM: c_int = 3;
    let mut dsm_guid: static guid_t = DSM_GUID;
#[no_mangle]
unsafe extern "C" fn npem_has_dsm(pdev: *mut pci_dev) -> bool {
    static bool npem_has_dsm(struct pci_dev *pdev)
    {
    acpi_handle handle;
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle)
    return false;
    return acpi_check_dsm(handle, &dsm_guid, 0x1,
    BIT(GET_SUPPORTED_STATES_DSM) |
    BIT(GET_STATE_DSM) | BIT(SET_STATE_DSM));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsm_output {
    pub status: u16,
    pub function_specific_err: u8,
    pub vendor_specific_err: u8,
    pub state: u32,
}

//
// dsm_evaluate() - send DSM PCIe SSD Status LED command
// @pdev: PCI device
// @dsm_func: DSM LED Function
// @output: buffer to copy DSM Response
// @value_to_set: value for SET_STATE_DSM function
//
// To not bother caller with ACPI context, the returned _DSM Output Buffer is
// copied.
//
    static int dsm_evaluate(struct pci_dev *pdev, u64 dsm_func,
    struct dsm_output *output, u32 value_to_set)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(&pdev.dev);
    union acpi_object *out_obj, arg3[2];
    union acpi_object *arg3_p = core::ptr::null_mut();
    if (dsm_func == SET_STATE_DSM) {
    arg3[0].type = ACPI_TYPE_PACKAGE;
    arg3[0].package.count = 1;
    arg3[0].package.elements = &arg3[1];
    arg3[1].type = ACPI_TYPE_BUFFER;
    arg3[1].buffer.length = 4;
    arg3[1].buffer.pointer = (u8 *)&value_to_set;
    arg3_p = arg3;
    }
    out_obj = acpi_evaluate_dsm_typed(handle, &dsm_guid, 0x1, dsm_func,
    arg3_p, ACPI_TYPE_BUFFER);
    if (!out_obj)
    return -EIO;
    if (out_obj.buffer.length < sizeof(struct dsm_output)) {
    ACPI_FREE(out_obj);
    return -EIO;
    }
    memcpy(output, out_obj.buffer.pointer, sizeof(struct dsm_output));
    ACPI_FREE(out_obj);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsm_get(pdev: *mut pci_dev, dsm_func: u64, buf: *mut u32) -> c_int {
    static int dsm_get(struct pci_dev *pdev, u64 dsm_func, u32 *buf)
    {
    struct dsm_output output;
    let mut ret: c_int = dsm_evaluate(pdev, dsm_func, &output, 0);
    if (ret)
    return ret;
    if (output.status != 0)
    return -EIO;
// buf = output.state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsm_get_active_indications(npem: *mut npem, buf: *mut u32) -> c_int {
    static int dsm_get_active_indications(struct npem *npem, u32 *buf)
    {
    let mut ret: c_int = dsm_get(npem.dev, GET_STATE_DSM, buf);
// Filter out not supported indications in response
// buf &= npem->supported_indications;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dsm_set_active_indications(npem: *mut npem, value: u32) -> c_int {
    static int dsm_set_active_indications(struct npem *npem, u32 value)
    {
    struct dsm_output output;
    let mut ret: c_int = dsm_evaluate(npem.dev, SET_STATE_DSM, &output, value);
    if (ret)
    return ret;
    switch (output.status) {
    case 4:
//
// Not all bits are set. If this bit is set, the platform
// disregarded some or all of the request state changes. OSPM
// should check the resulting PCIe SSD Status LED States to see
// what, if anything, has changed.
//
// PCI Firmware Specification, r3.3 Table 4-19.
//
    if (output.function_specific_err != 1)
    return -EIO;
    fallthrough;
    case 0:
    break;
    default:
    return -EIO;
    }
    npem.active_indications = output.state;
    return 0;
    }
    static const struct npem_ops dsm_ops = {
    .get_active_indications = dsm_get_active_indications,
    .set_active_indications = dsm_set_active_indications,
    .name = "_DSM PCIe SSD Status LED Management",
    .inds = dsm_indications,
    };
#[no_mangle]
unsafe extern "C" fn npem_initialize_active_indications(npem: *mut npem) -> c_int {
    static int npem_initialize_active_indications(struct npem *npem)
    {
    int ret;
    lockdep_assert_held(&npem.lock);
    if (npem.active_inds_initialized)
    return 0;
    ret = npem.ops.get_active_indications(npem,
    &npem.active_indications);
    if (ret)
    return ret;
    npem.active_inds_initialized = true;
    return 0;
    }
//
// The status of each indicator is cached on first brightness_ get/set time
// and updated at write time.  brightness_get() is only responsible for
// reflecting the last written/cached value.
//
#[no_mangle]
unsafe extern "C" fn brightness_get(led: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness brightness_get(struct led_classdev *led)
    {
    struct npem_led *nled = container_of(led, struct npem_led, led);
    struct npem *npem = nled.npem;
    int ret, val = 0;
    ret = mutex_lock_interruptible(&npem.lock);
    if (ret)
    return ret;
    ret = npem_initialize_active_indications(npem);
    if (ret)
    goto out;
    if (npem.active_indications & nled.indication.bit)
    val = 1;
    out:
    mutex_unlock(&npem.lock);
    return val;
    }
    static int brightness_set(struct led_classdev *led,
    enum led_brightness brightness)
    {
    struct npem_led *nled = container_of(led, struct npem_led, led);
    struct npem *npem = nled.npem;
    u32 indications;
    int ret;
    ret = mutex_lock_interruptible(&npem.lock);
    if (ret)
    return ret;
    ret = npem_initialize_active_indications(npem);
    if (ret)
    goto out;
    if (brightness == 0)
    indications = npem.active_indications & ~(nled.indication.bit);
    else
    indications = npem.active_indications | nled.indication.bit;
    ret = npem.ops.set_active_indications(npem, indications);
    out:
    mutex_unlock(&npem.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn npem_free(npem: *mut npem) {
    static void npem_free(struct npem *npem)
    {
    struct npem_led *nled;
    int cnt;
    if (!npem)
    return;
    for (cnt = 0; cnt < npem.led_cnt; cnt++) {
    nled = &npem.leds[cnt];
    if (nled.name[0])
    led_classdev_unregister(&nled.led);
    }
    mutex_destroy(&npem.lock);
    kfree(npem);
    }
#[no_mangle]
unsafe extern "C" fn pci_npem_set_led_classdev(npem: *mut npem, nled: *mut npem_led) -> c_int {
    static int pci_npem_set_led_classdev(struct npem *npem, struct npem_led *nled)
    {
    struct led_classdev *led = &nled.led;
    let mut init_data: led_init_data = {};
    char *name = nled.name;
    int ret;
    init_data.devicename = pci_name(npem.dev);
    init_data.default_label = nled.indication.name;
    ret = led_compose_name(&npem.dev.dev, &init_data, name);
    if (ret)
    return ret;
    led.name = name;
    led.brightness_set_blocking = brightness_set;
    led.brightness_get = brightness_get;
    led.max_brightness = 1;
    led.default_trigger = "none";
    led.flags = LED_HW_PLUGGABLE;
    ret = led_classdev_register(&npem.dev.dev, led);
    if (ret)
// Clear the name to indicate that it is not registered.
    name[0] = 0;
    return ret;
    }
    static int pci_npem_init(struct pci_dev *dev, const struct npem_ops *ops,
    int pos, u32 caps)
    {
    let mut supported: u32 = reg_to_indications(caps, ops.inds);
    let mut supported_cnt: c_int = hweight32(supported);
    const struct indication *indication;
    struct npem_led *nled;
    struct npem *npem;
    let mut led_idx: c_int = 0;
    int ret;
    npem = kzalloc_flex(*npem, leds, supported_cnt);
    if (!npem)
    return -ENOMEM;
    npem.supported_indications = supported;
    npem.led_cnt = supported_cnt;
    npem.pos = pos;
    npem.dev = dev;
    npem.ops = ops;
    mutex_init(&npem.lock);
    for_each_indication(indication, npem_indications) {
    if (!(npem.supported_indications & indication.bit))
    continue;
    nled = &npem.leds[led_idx++];
    nled.indication = indication;
    nled.npem = npem;
    ret = pci_npem_set_led_classdev(npem, nled);
    if (ret) {
    npem_free(npem);
    return ret;
    }
    }
    dev.npem = npem;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_npem_remove(dev: *mut pci_dev) {
    void pci_npem_remove(struct pci_dev *dev)
    {
    npem_free(dev.npem);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_npem_create(dev: *mut pci_dev) {
    void pci_npem_create(struct pci_dev *dev)
    {
    const struct npem_ops *ops = &npem_ops;
    let mut pos: c_int = 0, ret;
    u32 cap;
    if (npem_has_dsm(dev)) {
//
// OS should use the DSM for LED control if it is available
// PCI Firmware Spec r3.3 sec 4.7.
//
    ret = dsm_get(dev, GET_SUPPORTED_STATES_DSM, &cap);
    if (ret)
    return;
    ops = &dsm_ops;
    } else {
    pos = pci_find_ext_capability(dev, PCI_EXT_CAP_ID_NPEM);
    if (pos == 0)
    return;
    if (pci_read_config_dword(dev, pos + PCI_NPEM_CAP, &cap) != 0 ||
    (cap & PCI_NPEM_CAP_CAPABLE) == 0)
    return;
    }
    pci_info(dev, "Configuring %s\n", ops.name);
    ret = pci_npem_init(dev, ops, pos, cap);
    if (ret)
    pci_err(dev, "Failed to register %s, err: %d\n", ops.name,
    ret);
    }
