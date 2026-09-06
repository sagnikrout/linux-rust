//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/pnpbios/bioscalls.c
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
// bioscalls.c - the lowlevel layer of the PnPBIOS driver
//

    __visible struct {
    u16 offset;
    u16 segment;
    } pnp_bios_callpoint;
//
// These are some opcodes for a "static asmlinkage"
// As this code is *not* executed inside the linux kernel segment, but in a
// alias at offset 0, we need a far return that can not be compiled by
// default (please, prove me wrong! this is *really* ugly!)
// This is the only way to get the bios to return into the kernel code,
// because the bios code runs in 16 bit protected mode and therefore can only
// return to the caller if the call is within the first 64kB, and the linux
// kernel begins at offset 3GB...
//
    asmlinkage __visible void pnp_bios_callfunc(void);
    __asm__(".text			\n"
    __ALIGN_STR "\n"
    ".globl pnp_bios_callfunc\n"
    "pnp_bios_callfunc:\n"
    "	pushl %edx	\n"
    "	pushl %ecx	\n"
    "	pushl %ebx	\n"
    "	pushl %eax	\n"
    "	lcallw *pnp_bios_callpoint\n"
    "	addl $16, %esp	\n"
    "	lret		\n"
    ".previous		\n");

    do { \
    struct desc_struct *gdt = get_cpu_gdt_rw((cpu)); \
    set_desc_base(&gdt[(selname) >> 3], (u32)(address)); \
    set_desc_limit(&gdt[(selname) >> 3], (size) - 1); \
    } while(0)
    static struct desc_struct bad_bios_desc = GDT_ENTRY_INIT(DESC_DATA32_BIOS,
    (unsigned long)__va(0x400UL), PAGE_SIZE - 0x400 - 1);
//
// At some point we want to use this stack frame pointer to unwind
// after PnP BIOS oopses.
//
    __visible u32 pnp_bios_fault_esp;
    __visible u32 pnp_bios_fault_eip;
    let mut pnp_bios_is_utter_crap: __visible u32 = 0;
    static DEFINE_SPINLOCK(pnp_bios_lock);
//
// Support Functions
//
    static inline u16 call_pnp_bios(u16 func, u16 arg1, u16 arg2, u16 arg3,
    u16 arg4, u16 arg5, u16 arg6, u16 arg7,
    void *ts1_base, u32 ts1_size,
    void *ts2_base, u32 ts2_size)
    {
    unsigned long flags;
    u16 status;
    struct desc_struct save_desc_40;
    int cpu;
//
// PnP BIOSes are generally not terribly re-entrant.
// Also, don't rely on them to save everything correctly.
//
    if (pnp_bios_is_utter_crap)
    return PNP_FUNCTION_NOT_SUPPORTED;
    cpu = get_cpu();
    save_desc_40 = get_cpu_gdt_rw(cpu)[0x40 / 8];
    get_cpu_gdt_rw(cpu)[0x40 / 8] = bad_bios_desc;
// On some boxes IRQ's during PnP BIOS calls are deadly.
    spin_lock_irqsave(&pnp_bios_lock, flags);
// The lock prevents us bouncing CPU here
    if (ts1_size)
    Q2_SET_SEL(smp_processor_id(), PNP_TS1, ts1_base, ts1_size);
    if (ts2_size)
    Q2_SET_SEL(smp_processor_id(), PNP_TS2, ts2_base, ts2_size);
    __asm__ __volatile__("pushl %%ebp\n\t"
    "pushl %%edi\n\t"
    "pushl %%esi\n\t"
    "pushl %%ds\n\t"
    "pushl %%es\n\t"
    "pushl %%fs\n\t"
    "pushl %%gs\n\t"
    "pushfl\n\t"
    "movl %%esp, pnp_bios_fault_esp\n\t"
    "movl $1f, pnp_bios_fault_eip\n\t"
    "lcall %5,%6\n\t"
    "1:popfl\n\t"
    "popl %%gs\n\t"
    "popl %%fs\n\t"
    "popl %%es\n\t"
    "popl %%ds\n\t"
    "popl %%esi\n\t"
    "popl %%edi\n\t"
    "popl %%ebp\n\t":"=a"(status)
    :"0"((func) | (((u32) arg1) << 16)),
    "b"((arg2) | (((u32) arg3) << 16)),
    "c"((arg4) | (((u32) arg5) << 16)),
    "d"((arg6) | (((u32) arg7) << 16)),
    "i"(PNP_CS32), "i"(0)
    :"memory");
    spin_unlock_irqrestore(&pnp_bios_lock, flags);
    get_cpu_gdt_rw(cpu)[0x40 / 8] = save_desc_40;
    put_cpu();
// If we get here and this is set then the PnP BIOS faulted on us.
    if (pnp_bios_is_utter_crap) {
    printk(KERN_ERR
    "PnPBIOS: Warning! Your PnP BIOS caused a fatal error. Attempting to continue\n");
    printk(KERN_ERR
    "PnPBIOS: You may need to reboot with the \"pnpbios=off\" option to operate stably\n");
    printk(KERN_ERR
    "PnPBIOS: Check with your vendor for an updated BIOS\n");
    }
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnpbios_print_status(module: *const c_char, status: u16) {
    void pnpbios_print_status(const char *module, u16 status)
    {
    switch (status) {
    case PNP_SUCCESS:
    printk(KERN_ERR "PnPBIOS: %s: function successful\n", module);
    break;
    case PNP_NOT_SET_STATICALLY:
    printk(KERN_ERR "PnPBIOS: %s: unable to set static resources\n",
    module);
    break;
    case PNP_UNKNOWN_FUNCTION:
    printk(KERN_ERR "PnPBIOS: %s: invalid function number passed\n",
    module);
    break;
    case PNP_FUNCTION_NOT_SUPPORTED:
    printk(KERN_ERR
    "PnPBIOS: %s: function not supported on this system\n",
    module);
    break;
    case PNP_INVALID_HANDLE:
    printk(KERN_ERR "PnPBIOS: %s: invalid handle\n", module);
    break;
    case PNP_BAD_PARAMETER:
    printk(KERN_ERR "PnPBIOS: %s: invalid parameters were passed\n",
    module);
    break;
    case PNP_SET_FAILED:
    printk(KERN_ERR "PnPBIOS: %s: unable to set resources\n",
    module);
    break;
    case PNP_EVENTS_NOT_PENDING:
    printk(KERN_ERR "PnPBIOS: %s: no events are pending\n", module);
    break;
    case PNP_SYSTEM_NOT_DOCKED:
    printk(KERN_ERR "PnPBIOS: %s: the system is not docked\n",
    module);
    break;
    case PNP_NO_ISA_PNP_CARDS:
    printk(KERN_ERR
    "PnPBIOS: %s: no isapnp cards are installed on this system\n",
    module);
    break;
    case PNP_UNABLE_TO_DETERMINE_DOCK_CAPABILITIES:
    printk(KERN_ERR
    "PnPBIOS: %s: cannot determine the capabilities of the docking station\n",
    module);
    break;
    case PNP_CONFIG_CHANGE_FAILED_NO_BATTERY:
    printk(KERN_ERR
    "PnPBIOS: %s: unable to undock, the system does not have a battery\n",
    module);
    break;
    case PNP_CONFIG_CHANGE_FAILED_RESOURCE_CONFLICT:
    printk(KERN_ERR
    "PnPBIOS: %s: could not dock due to resource conflicts\n",
    module);
    break;
    case PNP_BUFFER_TOO_SMALL:
    printk(KERN_ERR "PnPBIOS: %s: the buffer passed is too small\n",
    module);
    break;
    case PNP_USE_ESCD_SUPPORT:
    printk(KERN_ERR "PnPBIOS: %s: use ESCD instead\n", module);
    break;
    case PNP_MESSAGE_NOT_SUPPORTED:
    printk(KERN_ERR "PnPBIOS: %s: the message is unsupported\n",
    module);
    break;
    case PNP_HARDWARE_ERROR:
    printk(KERN_ERR "PnPBIOS: %s: a hardware failure has occurred\n",
    module);
    break;
    default:
    printk(KERN_ERR "PnPBIOS: %s: unexpected status 0x%x\n", module,
    status);
    break;
    }
    }
//
// PnP BIOS Low Level Calls
//
pub const PNP_GET_NUM_SYS_DEV_NODES: c_uint = 0x00;
pub const PNP_GET_SYS_DEV_NODE: c_uint = 0x01;
pub const PNP_SET_SYS_DEV_NODE: c_uint = 0x02;
pub const PNP_GET_EVENT: c_uint = 0x03;
pub const PNP_SEND_MESSAGE: c_uint = 0x04;
pub const PNP_GET_DOCKING_STATION_INFORMATION: c_uint = 0x05;
pub const PNP_SET_STATIC_ALLOCED_RES_INFO: c_uint = 0x09;
pub const PNP_GET_STATIC_ALLOCED_RES_INFO: c_uint = 0x0a;
pub const PNP_GET_APM_ID_TABLE: c_uint = 0x0b;
pub const PNP_GET_PNP_ISA_CONFIG_STRUC: c_uint = 0x40;
pub const PNP_GET_ESCD_INFO: c_uint = 0x41;
pub const PNP_READ_ESCD: c_uint = 0x42;
pub const PNP_WRITE_ESCD: c_uint = 0x43;
//
// Call PnP BIOS with function 0x00, "get number of system device nodes"
//
#[no_mangle]
unsafe extern "C" fn __pnp_bios_dev_node_info(data: *mut pnp_dev_node_info) -> c_int {
    static int __pnp_bios_dev_node_info(struct pnp_dev_node_info *data)
    {
    u16 status;
    if (!pnp_bios_present())
    return PNP_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_GET_NUM_SYS_DEV_NODES, 0, PNP_TS1, 2,
    PNP_TS1, PNP_DS, 0, 0, data,
    sizeof(struct pnp_dev_node_info), core::ptr::null_mut(), 0);
    data.no_nodes &= 0xff;
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_dev_node_info(data: *mut pnp_dev_node_info) -> c_int {
    int pnp_bios_dev_node_info(struct pnp_dev_node_info *data)
    {
    let mut status: c_int = __pnp_bios_dev_node_info(data);
    if (status)
    pnpbios_print_status("dev_node_info", status);
    return status;
    }
//
// Note that some PnP BIOSes (e.g., on Sony Vaio laptops) die a horrible
// death if they are asked to access the "current" configuration.
// Therefore, if it's a matter of indifference, it's better to call
// get_dev_node() and set_dev_node() with boot=1 rather than with boot=0.
//
// Call PnP BIOS with function 0x01, "get system device node"
// Input: *nodenum = desired node,
// boot = whether to get nonvolatile boot (!=0)
// or volatile current (0) config
// Output: *nodenum=next node or 0xff if no more nodes
//
    static int __pnp_bios_get_dev_node(u8 *nodenum, char boot,
    struct pnp_bios_node *data)
    {
    u16 status;
    u16 tmp_nodenum;
    if (!pnp_bios_present())
    return PNP_FUNCTION_NOT_SUPPORTED;
    if (!boot && pnpbios_dont_use_current_config)
    return PNP_FUNCTION_NOT_SUPPORTED;
    tmp_nodenum = *nodenum;
    status = call_pnp_bios(PNP_GET_SYS_DEV_NODE, 0, PNP_TS1, 0, PNP_TS2,
    boot ? 2 : 1, PNP_DS, 0, &tmp_nodenum,
    sizeof(tmp_nodenum), data, 65536);
// nodenum = tmp_nodenum;
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_get_dev_node(nodenum: *mut u8, boot: c_char, data: *mut pnp_bios_node) -> c_int {
    int pnp_bios_get_dev_node(u8 *nodenum, char boot, struct pnp_bios_node *data)
    {
    int status;
    status = __pnp_bios_get_dev_node(nodenum, boot, data);
    if (status)
    pnpbios_print_status("get_dev_node", status);
    return status;
    }
//
// Call PnP BIOS with function 0x02, "set system device node"
// Input: *nodenum = desired node,
// boot = whether to set nonvolatile boot (!=0)
// or volatile current (0) config
//
    static int __pnp_bios_set_dev_node(u8 nodenum, char boot,
    struct pnp_bios_node *data)
    {
    u16 status;
    if (!pnp_bios_present())
    return PNP_FUNCTION_NOT_SUPPORTED;
    if (!boot && pnpbios_dont_use_current_config)
    return PNP_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_SET_SYS_DEV_NODE, nodenum, 0, PNP_TS1,
    boot ? 2 : 1, PNP_DS, 0, 0, data, 65536, core::ptr::null_mut(),
    0);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_set_dev_node(nodenum: u8, boot: c_char, data: *mut pnp_bios_node) -> c_int {
    int pnp_bios_set_dev_node(u8 nodenum, char boot, struct pnp_bios_node *data)
    {
    int status;
    status = __pnp_bios_set_dev_node(nodenum, boot, data);
    if (status) {
    pnpbios_print_status("set_dev_node", status);
    return status;
    }
    if (!boot) {		/* Update devlist */
    status = pnp_bios_get_dev_node(&nodenum, boot, data);
    if (status)
    return status;
    }
    return status;
    }
//
// Call PnP BIOS with function 0x05, "get docking station information"
//
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_dock_station_info(data: *mut pnp_docking_station_info) -> c_int {
    int pnp_bios_dock_station_info(struct pnp_docking_station_info *data)
    {
    u16 status;
    if (!pnp_bios_present())
    return PNP_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_GET_DOCKING_STATION_INFORMATION, 0, PNP_TS1,
    PNP_DS, 0, 0, 0, 0, data,
    sizeof(struct pnp_docking_station_info), core::ptr::null_mut(),
    0);
    return status;
    }
//
// Call PnP BIOS with function 0x0a, "get statically allocated resource
// information"
//
#[no_mangle]
unsafe extern "C" fn __pnp_bios_get_stat_res(info: *mut c_char) -> c_int {
    static int __pnp_bios_get_stat_res(char *info)
    {
    u16 status;
    if (!pnp_bios_present())
    return PNP_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_GET_STATIC_ALLOCED_RES_INFO, 0, PNP_TS1,
    PNP_DS, 0, 0, 0, 0, info, 65536, core::ptr::null_mut(), 0);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_get_stat_res(info: *mut c_char) -> c_int {
    int pnp_bios_get_stat_res(char *info)
    {
    int status;
    status = __pnp_bios_get_stat_res(info);
    if (status)
    pnpbios_print_status("get_stat_res", status);
    return status;
    }
//
// Call PnP BIOS with function 0x40, "get isa pnp configuration structure"
//
#[no_mangle]
unsafe extern "C" fn __pnp_bios_isapnp_config(data: *mut pnp_isa_config_struc) -> c_int {
    static int __pnp_bios_isapnp_config(struct pnp_isa_config_struc *data)
    {
    u16 status;
    if (!pnp_bios_present())
    return PNP_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_GET_PNP_ISA_CONFIG_STRUC, 0, PNP_TS1, PNP_DS,
    0, 0, 0, 0, data,
    sizeof(struct pnp_isa_config_struc), core::ptr::null_mut(), 0);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_isapnp_config(data: *mut pnp_isa_config_struc) -> c_int {
    int pnp_bios_isapnp_config(struct pnp_isa_config_struc *data)
    {
    int status;
    status = __pnp_bios_isapnp_config(data);
    if (status)
    pnpbios_print_status("isapnp_config", status);
    return status;
    }
//
// Call PnP BIOS with function 0x41, "get ESCD info"
//
#[no_mangle]
unsafe extern "C" fn __pnp_bios_escd_info(data: *mut escd_info_struc) -> c_int {
    static int __pnp_bios_escd_info(struct escd_info_struc *data)
    {
    u16 status;
    if (!pnp_bios_present())
    return ESCD_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_GET_ESCD_INFO, 0, PNP_TS1, 2, PNP_TS1, 4,
    PNP_TS1, PNP_DS, data,
    sizeof(struct escd_info_struc), core::ptr::null_mut(), 0);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_escd_info(data: *mut escd_info_struc) -> c_int {
    int pnp_bios_escd_info(struct escd_info_struc *data)
    {
    int status;
    status = __pnp_bios_escd_info(data);
    if (status)
    pnpbios_print_status("escd_info", status);
    return status;
    }
//
// Call PnP BIOS function 0x42, "read ESCD"
// nvram_base is determined by calling escd_info
//
#[no_mangle]
unsafe extern "C" fn __pnp_bios_read_escd(data: *mut c_char, nvram_base: u32) -> c_int {
    static int __pnp_bios_read_escd(char *data, u32 nvram_base)
    {
    u16 status;
    if (!pnp_bios_present())
    return ESCD_FUNCTION_NOT_SUPPORTED;
    status = call_pnp_bios(PNP_READ_ESCD, 0, PNP_TS1, PNP_TS2, PNP_DS, 0, 0,
    0, data, 65536, __va(nvram_base), 65536);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_bios_read_escd(data: *mut c_char, nvram_base: u32) -> c_int {
    int pnp_bios_read_escd(char *data, u32 nvram_base)
    {
    int status;
    status = __pnp_bios_read_escd(data, nvram_base);
    if (status)
    pnpbios_print_status("read_escd", status);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn pnpbios_calls_init(header: *mut union pnp_bios_install_struct) {
    void pnpbios_calls_init(union pnp_bios_install_struct *header)
    {
    int i;
    pnp_bios_callpoint.offset = header.fields.pm16offset;
    pnp_bios_callpoint.segment = PNP_CS16;
    for_each_possible_cpu(i) {
    struct desc_struct *gdt = get_cpu_gdt_rw(i);
    if (!gdt)
    continue;
    set_desc_base(&gdt[GDT_ENTRY_PNPBIOS_CS32],
    (unsigned long)&pnp_bios_callfunc);
    set_desc_base(&gdt[GDT_ENTRY_PNPBIOS_CS16],
    (unsigned long)__va(header.fields.pm16cseg));
    set_desc_base(&gdt[GDT_ENTRY_PNPBIOS_DS],
    (unsigned long)__va(header.fields.pm16dseg));
    }
    }
