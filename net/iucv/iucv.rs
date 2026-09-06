//! Automatically rewritten from C to Rust
//! Source: net/iucv/iucv.c
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
// IUCV base infrastructure.
//
// Copyright IBM Corp. 2001, 2009
//
// Author(s):
// Original source:
// Alan Altmark (Alan_Altmark@us.ibm.com)	Sept. 2000
// Xenia Tkatschow (xenia@us.ibm.com)
// 2Gb awareness and general cleanup:
// Fritz Elfert (elfert@de.ibm.com, felfert@millenux.com)
// Rewritten for af_iucv:
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// PM functions:
// Ursula Braun (ursula.braun@de.ibm.com)
//
// Documentation used:
// The original source
// CP Programming Service, IBM document # SC24-5760
//

//
// FLAGS:
// All flags are defined in the field IPFLAGS1 of each function
// and can be found in CP Programming Services.
// IPSRCCLS - Indicates you have specified a source class.
// IPTRGCLS - Indicates you have specified a target class.
// IPFGPID  - Indicates you have specified a pathid.
// IPFGMID  - Indicates you have specified a message ID.
// IPNORPY  - Indicates a one-way message. No reply expected.
// IPALL    - Indicates that all paths are affected.
//
pub const IUCV_IPSRCCLS: c_uint = 0x01;
pub const IUCV_IPTRGCLS: c_uint = 0x01;
pub const IUCV_IPFGPID: c_uint = 0x02;
pub const IUCV_IPFGMID: c_uint = 0x04;
pub const IUCV_IPNORPY: c_uint = 0x10;
pub const IUCV_IPALL: c_uint = 0x80;
#[no_mangle]
unsafe extern "C" fn iucv_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int iucv_bus_match(struct device *dev, const struct device_driver *drv)
    {
    return 0;
    }
    const struct bus_type iucv_bus = {
    .name = "iucv",
    .match = iucv_bus_match,
    };
    EXPORT_SYMBOL(iucv_bus);
    static struct device *iucv_root;
#[no_mangle]
unsafe extern "C" fn iucv_release_device(device: *mut device) {
    static void iucv_release_device(struct device *device)
    {
    kfree(device);
    }
    struct device *iucv_alloc_device(const struct attribute_group **attrs,
    struct device_driver *driver,
    void *priv, const char *fmt, ...)
    {
    struct device *dev;
    va_list vargs;
    char buf[20];
    int rc;
    dev = kzalloc_obj(*dev);
    if (!dev)
    goto out_error;
    va_start(vargs, fmt);
    vscnprintf(buf, sizeof(buf), fmt, vargs);
    rc = dev_set_name(dev, "%s", buf);
    va_end(vargs);
    if (rc)
    goto out_error;
    dev.bus = &iucv_bus;
    dev.parent = iucv_root;
    dev.driver = driver;
    dev.groups = attrs;
    dev.release = iucv_release_device;
    dev_set_drvdata(dev, priv);
    return dev;
    out_error:
    kfree(dev);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(iucv_alloc_device);
    static int iucv_available;
// General IUCV interrupt structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_irq_data {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iptype: u8,
    pub res2: [u32; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_irq_list {
    pub list: list_head,
    pub data: iucv_irq_data,
}

    static struct iucv_irq_data *iucv_irq_data[NR_CPUS];
    let mut iucv_buffer_cpumask: static cpumask_t = { CPU_BITS_NONE };
    let mut iucv_irq_cpumask: static cpumask_t = { CPU_BITS_NONE };
//
// Queue of interrupt buffers lock for delivery via the tasklet
// (fast but can't call smp_call_function).
//
    static LIST_HEAD(iucv_task_queue);
//
// The tasklet for fast delivery of iucv interrupts.
//
    static void iucv_tasklet_fn(unsigned long);
    static DECLARE_TASKLET_OLD(iucv_tasklet, iucv_tasklet_fn);
//
// Queue of interrupt buffers for delivery via a work queue
// (slower but can call smp_call_function).
//
    static LIST_HEAD(iucv_work_queue);
//
// The work element to deliver path pending interrupts.
//
    static void iucv_work_fn(struct work_struct *work);
    static DECLARE_WORK(iucv_work, iucv_work_fn);
//
// Spinlock protecting task and work queue.
//
    static DEFINE_SPINLOCK(iucv_queue_lock);
    enum iucv_command_codes {
    IUCV_QUERY = 0,
    IUCV_RETRIEVE_BUFFER = 2,
    IUCV_SEND = 4,
    IUCV_RECEIVE = 5,
    IUCV_REPLY = 6,
    IUCV_REJECT = 8,
    IUCV_PURGE = 9,
    IUCV_ACCEPT = 10,
    IUCV_CONNECT = 11,
    IUCV_DECLARE_BUFFER = 12,
    IUCV_QUIESCE = 13,
    IUCV_RESUME = 14,
    IUCV_SEVER = 15,
    IUCV_SETMASK = 16,
    IUCV_SETCONTROLMASK = 17,
    };
//
// Error messages that are used with the iucv_sever function. They get
// converted to EBCDIC.
//
    static char iucv_error_no_listener[16] = "NO LISTENER";
    static char iucv_error_no_memory[16] = "NO MEMORY";
    static char iucv_error_pathid[16] = "INVALID PATHID";
//
// iucv_handler_list: List of registered handlers.
//
    static LIST_HEAD(iucv_handler_list);
//
// iucv_path_table: array of pointers to iucv_path structures.
//
    static struct iucv_path **iucv_path_table;
    static unsigned long iucv_max_pathid;
//
// iucv_lock: spinlock protecting iucv_handler_list and iucv_pathid_table
//
    static DEFINE_SPINLOCK(iucv_table_lock);
//
// iucv_active_cpu: contains the number of the cpu executing the tasklet
// or the work handler. Needed for iucv_path_sever called from tasklet.
//
    let mut iucv_active_cpu: static int = -1;
//
// Mutex and wait queue for iucv_register/iucv_unregister.
//
    static DEFINE_MUTEX(iucv_register_mutex);
//
// Counter for number of non-smp capable handlers.
//
    static int iucv_nonsmp_handler;
//
// IUCV control data structure. Used by iucv_path_accept, iucv_path_connect,
// iucv_path_quiesce and iucv_path_sever.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_cmd_control {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iprcode: u8,
    pub ipmsglim: u16,
    pub res1: u16,
    pub ipvmid: [u8; 8],
    pub ipuser: [u8; 16],
    pub iptarget: [u8; 8],
    pub ((packed,aligned(8))): } __attribute__,
//
// Data in parameter list iucv structure. Used by iucv_message_send,
// iucv_message_send2way and iucv_message_reply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_cmd_dpl {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iprcode: u8,
    pub ipmsgid: u32,
    pub iptrgcls: u32,
    pub iprmmsg: [u8; 8],
    pub ipsrccls: u32,
    pub ipmsgtag: u32,
    pub ipbfadr2: dma32_t,
    pub ipbfln2f: u32,
    pub res: u32,
    pub ((packed,aligned(8))): } __attribute__,
//
// Data in buffer iucv structure. Used by iucv_message_receive,
// iucv_message_reject, iucv_message_send, iucv_message_send2way
// and iucv_declare_cpu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_cmd_db {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iprcode: u8,
    pub ipmsgid: u32,
    pub iptrgcls: u32,
    pub ipbfadr1: dma32_t,
    pub ipbfln1f: u32,
    pub ipsrccls: u32,
    pub ipmsgtag: u32,
    pub ipbfadr2: dma32_t,
    pub ipbfln2f: u32,
    pub res: u32,
    pub ((packed,aligned(8))): } __attribute__,
//
// Purge message iucv structure. Used by iucv_message_purge.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_cmd_purge {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iprcode: u8,
    pub ipmsgid: u32,
    pub ipaudit: [u8; 3],
    pub res1: [u8; 5],
    pub res2: u32,
    pub ipsrccls: u32,
    pub ipmsgtag: u32,
    pub res3: [u32; 3],
    pub ((packed,aligned(8))): } __attribute__,
//
// Set mask iucv structure. Used by iucv_enable_cpu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_cmd_set_mask {
    pub ipmask: u8,
    pub res1: [u8; 2],
    pub iprcode: u8,
    pub res2: [u32; 9],
    pub ((packed,aligned(8))): } __attribute__,
    union iucv_param {
    pub ctrl: iucv_cmd_control,
    pub dpl: iucv_cmd_dpl,
    pub db: iucv_cmd_db,
    pub purge: iucv_cmd_purge,
    pub set_mask: iucv_cmd_set_mask,
}

//
// Anchor for per-cpu IUCV command parameter block.
//
    static union iucv_param *iucv_param[NR_CPUS];
    static union iucv_param *iucv_param_irq[NR_CPUS];
//
// __iucv_call_b2f0 - Calls CP to execute IUCV commands.
//
// @command: identifier of IUCV call to CP.
// @parm: pointer to a struct iucv_parm block
//
// Returns: the result of the CP IUCV call.
//
#[no_mangle]
pub unsafe extern "C" fn __iucv_call_b2f0(command: c_int, parm: *mut union iucv_param) -> c_int {
    static inline int __iucv_call_b2f0(int command, union iucv_param *parm)
    {
    let mut reg1: c_ulong = virt_to_phys(parm);
    int cc;
    asm volatile(
    "	lgr	0,%[reg0]\n"
    "	lgr	1,%[reg1]\n"
    "	.long	0xb2f01000\n"
    "	ipm	%[cc]\n"
    "	srl	%[cc],28\n"
    : [cc] "=&d" (cc), "+m" (*parm)
    : [reg0] "d" ((unsigned long)command),
    [reg1] "d" (reg1)
    : "cc", "0", "1");
    return cc;
    }
#[no_mangle]
pub unsafe extern "C" fn iucv_call_b2f0(command: c_int, parm: *mut union iucv_param) -> c_int {
    static inline int iucv_call_b2f0(int command, union iucv_param *parm)
    {
    int ccode;
    ccode = __iucv_call_b2f0(command, parm);
    let mut ccode: return = = 1 ? parm.ctrl.iprcode : ccode;
    }
//
// iucv_query_maxconn - Determine the maximum number of connections that
// may be established.
//
// Returns: the maximum number of connections or -EPERM is IUCV is not
// available.
//
#[no_mangle]
unsafe extern "C" fn __iucv_query_maxconn(param: *mut c_void, max_pathid: *mut c_ulong) -> c_int {
    static int __iucv_query_maxconn(void *param, unsigned long *max_pathid)
    {
    let mut reg1: c_ulong = virt_to_phys(param);
    int cc;
    asm volatile (
    "	lghi	0,%[cmd]\n"
    "	lgr	1,%[reg1]\n"
    "	.long	0xb2f01000\n"
    "	ipm	%[cc]\n"
    "	srl	%[cc],28\n"
    "	lgr	%[reg1],1\n"
    : [cc] "=&d" (cc), [reg1] "+&d" (reg1)
    : [cmd] "K" (IUCV_QUERY)
    : "cc", "0", "1");
// max_pathid = reg1;
    return cc;
    }
#[no_mangle]
unsafe extern "C" fn iucv_query_maxconn() -> c_int {
    static int iucv_query_maxconn(void)
    {
    unsigned long max_pathid;
    void *param;
    int ccode;
    param = kzalloc_obj(union iucv_param, GFP_KERNEL | GFP_DMA);
    if (!param)
    return -ENOMEM;
    ccode = __iucv_query_maxconn(param, &max_pathid);
    if (ccode == 0)
    iucv_max_pathid = max_pathid;
    kfree(param);
    return ccode ? -EPERM : 0;
    }
//
// iucv_allow_cpu - Allow iucv interrupts on this cpu.
//
// @data: unused
//
#[no_mangle]
unsafe extern "C" fn iucv_allow_cpu(data: *mut c_void) {
    static void iucv_allow_cpu(void *data)
    {
    let mut cpu: c_int = smp_processor_id();
    union iucv_param *parm;
//
// Enable all iucv interrupts.
// ipmask contains bits for the different interrupts
// 0x80 - Flag to allow nonpriority message pending interrupts
// 0x40 - Flag to allow priority message pending interrupts
// 0x20 - Flag to allow nonpriority message completion interrupts
// 0x10 - Flag to allow priority message completion interrupts
// 0x08 - Flag to allow IUCV control interrupts
//
    parm = iucv_param_irq[cpu];
    memset(parm, 0, sizeof(union iucv_param));
    parm.set_mask.ipmask = 0xf8;
    iucv_call_b2f0(IUCV_SETMASK, parm);
//
// Enable all iucv control interrupts.
// ipmask contains bits for the different interrupts
// 0x80 - Flag to allow pending connections interrupts
// 0x40 - Flag to allow connection complete interrupts
// 0x20 - Flag to allow connection severed interrupts
// 0x10 - Flag to allow connection quiesced interrupts
// 0x08 - Flag to allow connection resumed interrupts
//
    memset(parm, 0, sizeof(union iucv_param));
    parm.set_mask.ipmask = 0xf8;
    iucv_call_b2f0(IUCV_SETCONTROLMASK, parm);
// Set indication that iucv interrupts are allowed for this cpu.
    cpumask_set_cpu(cpu, &iucv_irq_cpumask);
    }
//
// iucv_block_cpu - Block iucv interrupts on this cpu.
//
// @data: unused
//
#[no_mangle]
unsafe extern "C" fn iucv_block_cpu(data: *mut c_void) {
    static void iucv_block_cpu(void *data)
    {
    let mut cpu: c_int = smp_processor_id();
    union iucv_param *parm;
// Disable all iucv interrupts.
    parm = iucv_param_irq[cpu];
    memset(parm, 0, sizeof(union iucv_param));
    iucv_call_b2f0(IUCV_SETMASK, parm);
// Clear indication that iucv interrupts are allowed for this cpu.
    cpumask_clear_cpu(cpu, &iucv_irq_cpumask);
    }
//
// iucv_declare_cpu - Declare a interrupt buffer on this cpu.
//
// @data: unused
//
#[no_mangle]
unsafe extern "C" fn iucv_declare_cpu(data: *mut c_void) {
    static void iucv_declare_cpu(void *data)
    {
    let mut cpu: c_int = smp_processor_id();
    union iucv_param *parm;
    int rc;
    if (cpumask_test_cpu(cpu, &iucv_buffer_cpumask))
    return;
// Declare interrupt buffer.
    parm = iucv_param_irq[cpu];
    memset(parm, 0, sizeof(union iucv_param));
    parm.db.ipbfadr1 = virt_to_dma32(iucv_irq_data[cpu]);
    rc = iucv_call_b2f0(IUCV_DECLARE_BUFFER, parm);
    if (rc) {
    char *err = "Unknown";
    switch (rc) {
    case 0x03:
    err = "Directory error";
    break;
    case 0x0a:
    err = "Invalid length";
    break;
    case 0x13:
    err = "Buffer already exists";
    break;
    case 0x3e:
    err = "Buffer overlap";
    break;
    case 0x5c:
    err = "Paging or storage error";
    break;
    }
    pr_warn("Defining an interrupt buffer on CPU %i failed with 0x%02x (%s)\n",
    cpu, rc, err);
    return;
    }
// Set indication that an iucv buffer exists for this cpu.
    cpumask_set_cpu(cpu, &iucv_buffer_cpumask);
    if (iucv_nonsmp_handler == 0 || cpumask_empty(&iucv_irq_cpumask))
// Enable iucv interrupts on this cpu.
    iucv_allow_cpu(core::ptr::null_mut());
    else
// Disable iucv interrupts on this cpu.
    iucv_block_cpu(core::ptr::null_mut());
    }
//
// iucv_retrieve_cpu - Retrieve interrupt buffer on this cpu.
//
// @data: unused
//
#[no_mangle]
unsafe extern "C" fn iucv_retrieve_cpu(data: *mut c_void) {
    static void iucv_retrieve_cpu(void *data)
    {
    let mut cpu: c_int = smp_processor_id();
    union iucv_param *parm;
    if (!cpumask_test_cpu(cpu, &iucv_buffer_cpumask))
    return;
// Block iucv interrupts.
    iucv_block_cpu(core::ptr::null_mut());
// Retrieve interrupt buffer.
    parm = iucv_param_irq[cpu];
    iucv_call_b2f0(IUCV_RETRIEVE_BUFFER, parm);
// Clear indication that an iucv buffer exists for this cpu.
    cpumask_clear_cpu(cpu, &iucv_buffer_cpumask);
    }
//
// iucv_setmask_mp - Allow iucv interrupts on all cpus.
//
#[no_mangle]
unsafe extern "C" fn iucv_setmask_mp() {
    static void iucv_setmask_mp(void)
    {
    int cpu;
    cpus_read_lock();
    for_each_online_cpu(cpu)
// Enable all cpus with a declared buffer.
    if (cpumask_test_cpu(cpu, &iucv_buffer_cpumask) &&
    !cpumask_test_cpu(cpu, &iucv_irq_cpumask))
    smp_call_function_single(cpu, iucv_allow_cpu,
    core::ptr::null_mut(), 1);
    cpus_read_unlock();
    }
//
// iucv_setmask_up - Allow iucv interrupts on a single cpu.
//
#[no_mangle]
unsafe extern "C" fn iucv_setmask_up() {
    static void iucv_setmask_up(void)
    {
    static cpumask_t cpumask;
    int cpu;
// Disable all cpu but the first in cpu_irq_cpumask.
    cpumask_copy(&cpumask, &iucv_irq_cpumask);
    cpumask_clear_cpu(cpumask_first(&iucv_irq_cpumask), &cpumask);
    for_each_cpu(cpu, &cpumask)
    smp_call_function_single(cpu, iucv_block_cpu, core::ptr::null_mut(), 1);
    }
//
// iucv_enable - Make the iucv ready for use
//
// It allocates the pathid table, declares an iucv interrupt buffer and
// enables the iucv interrupts. Called when the first user has registered
// an iucv handler.
//
#[no_mangle]
unsafe extern "C" fn iucv_enable() -> c_int {
    static int iucv_enable(void)
    {
    size_t alloc_size;
    int cpu, rc;
    cpus_read_lock();
    rc = -ENOMEM;
    alloc_size = iucv_max_pathid * sizeof(*iucv_path_table);
    iucv_path_table = kzalloc(alloc_size, GFP_KERNEL);
    if (!iucv_path_table)
    goto out;
// Declare per cpu buffers.
    rc = -EIO;
    for_each_online_cpu(cpu)
    smp_call_function_single(cpu, iucv_declare_cpu, core::ptr::null_mut(), 1);
    if (cpumask_empty(&iucv_buffer_cpumask))
// No cpu could declare an iucv buffer.
    goto out;
    cpus_read_unlock();
    return 0;
    out:
    kfree(iucv_path_table);
    iucv_path_table = core::ptr::null_mut();
    cpus_read_unlock();
    return rc;
    }
//
// iucv_disable - Shuts down iucv.
//
// It disables iucv interrupts, retrieves the iucv interrupt buffer and frees
// the pathid table. Called after the last user unregister its iucv handler.
//
#[no_mangle]
unsafe extern "C" fn iucv_disable() {
    static void iucv_disable(void)
    {
    cpus_read_lock();
    on_each_cpu(iucv_retrieve_cpu, core::ptr::null_mut(), 1);
    kfree(iucv_path_table);
    iucv_path_table = core::ptr::null_mut();
    cpus_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn iucv_cpu_dead(cpu: c_uint) -> c_int {
    static int iucv_cpu_dead(unsigned int cpu)
    {
    kfree(iucv_param_irq[cpu]);
    iucv_param_irq[cpu] = core::ptr::null_mut();
    kfree(iucv_param[cpu]);
    iucv_param[cpu] = core::ptr::null_mut();
    kfree(iucv_irq_data[cpu]);
    iucv_irq_data[cpu] = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iucv_cpu_prepare(cpu: c_uint) -> c_int {
    static int iucv_cpu_prepare(unsigned int cpu)
    {
// Note: GFP_DMA used to get memory below 2G
    iucv_irq_data[cpu] = kmalloc_node(sizeof(struct iucv_irq_data),
    GFP_KERNEL|GFP_DMA, cpu_to_node(cpu));
    if (!iucv_irq_data[cpu])
    goto out_free;
// Allocate parameter blocks.
    iucv_param[cpu] = kmalloc_node(sizeof(union iucv_param),
    GFP_KERNEL|GFP_DMA, cpu_to_node(cpu));
    if (!iucv_param[cpu])
    goto out_free;
    iucv_param_irq[cpu] = kmalloc_node(sizeof(union iucv_param),
    GFP_KERNEL|GFP_DMA, cpu_to_node(cpu));
    if (!iucv_param_irq[cpu])
    goto out_free;
    return 0;
    out_free:
    iucv_cpu_dead(cpu);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn iucv_cpu_online(cpu: c_uint) -> c_int {
    static int iucv_cpu_online(unsigned int cpu)
    {
    if (!iucv_path_table)
    return 0;
    iucv_declare_cpu(core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iucv_cpu_down_prep(cpu: c_uint) -> c_int {
    static int iucv_cpu_down_prep(unsigned int cpu)
    {
    cpumask_var_t cpumask;
    let mut ret: c_int = 0;
    if (!iucv_path_table)
    return 0;
    if (!alloc_cpumask_var(&cpumask, GFP_KERNEL))
    return -ENOMEM;
    cpumask_copy(cpumask, &iucv_buffer_cpumask);
    cpumask_clear_cpu(cpu, cpumask);
    if (cpumask_empty(cpumask)) {
// Can't offline last IUCV enabled cpu.
    ret = -EINVAL;
    goto __free_cpumask;
    }
    iucv_retrieve_cpu(core::ptr::null_mut());
    if (!cpumask_empty(&iucv_irq_cpumask))
    goto __free_cpumask;
    smp_call_function_single(cpumask_first(&iucv_buffer_cpumask),
    iucv_allow_cpu, core::ptr::null_mut(), 1);
    __free_cpumask:
    free_cpumask_var(cpumask);
    return ret;
    }
//
// iucv_sever_pathid - Sever an iucv path to free up the pathid. Used internally.
//
// @pathid: path identification number.
// @userdata: 16-bytes of user data.
//
// Returns: 0 on success, the result of the CP b2f0 IUCV call.
//
#[no_mangle]
unsafe extern "C" fn iucv_sever_pathid(pathid: u16, userdata: *mut u8) -> c_int {
    static int iucv_sever_pathid(u16 pathid, u8 *userdata)
    {
    union iucv_param *parm;
    parm = iucv_param_irq[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    if (userdata)
    memcpy(parm.ctrl.ipuser, userdata, sizeof(parm.ctrl.ipuser));
    parm.ctrl.ippathid = pathid;
    return iucv_call_b2f0(IUCV_SEVER, parm);
    }
//
// __iucv_cleanup_queue - Nop function called via smp_call_function to force
// work items from pending external iucv interrupts to the work queue.
//
// @dummy: unused dummy argument
//
#[no_mangle]
unsafe extern "C" fn __iucv_cleanup_queue(dummy: *mut c_void) {
    static void __iucv_cleanup_queue(void *dummy)
    {
    }
//
// iucv_cleanup_queue - Called after a path has been severed to find all
// remaining work items for the now stale pathid.
//
// The caller needs to hold the iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_cleanup_queue() {
    static void iucv_cleanup_queue(void)
    {
    struct iucv_irq_list *p, *n;
//
// When a path is severed, the pathid can be reused immediately
// on a iucv connect or a connection pending interrupt. Remove
// all entries from the task queue that refer to a stale pathid
// (iucv_path_table[ix] == NULL). Only then do the iucv connect
// or deliver the connection pending interrupt. To get all the
// pending interrupts force them to the work queue by calling
// an empty function on all cpus.
//
    smp_call_function(__iucv_cleanup_queue, core::ptr::null_mut(), 1);
    spin_lock_irq(&iucv_queue_lock);
    list_for_each_entry_safe(p, n, &iucv_task_queue, list) {
// Remove stale work items from the task queue.
    if (iucv_path_table[p.data.ippathid] == core::ptr::null_mut()) {
    list_del(&p.list);
    kfree(p);
    }
    }
    spin_unlock_irq(&iucv_queue_lock);
    }
//
// iucv_register - Registers a driver with IUCV.
//
// @handler: address of iucv handler structure
// @smp: != 0 indicates that the handler can deal with out of order messages
//
// Returns: 0 on success, -ENOMEM if the memory allocation for the pathid
// table failed, or -EIO if IUCV_DECLARE_BUFFER failed on all cpus.
//
#[no_mangle]
pub unsafe extern "C" fn iucv_register(handler: *mut iucv_handler, smp: c_int) -> c_int {
    int iucv_register(struct iucv_handler *handler, int smp)
    {
    int rc;
    if (!iucv_available)
    return -ENOSYS;
    mutex_lock(&iucv_register_mutex);
    if (!smp)
    iucv_nonsmp_handler++;
    if (list_empty(&iucv_handler_list)) {
    rc = iucv_enable();
    if (rc)
    goto out_mutex;
    } else if (!smp && iucv_nonsmp_handler == 1)
    iucv_setmask_up();
    INIT_LIST_HEAD(&handler.paths);
    spin_lock_bh(&iucv_table_lock);
    list_add_tail(&handler.list, &iucv_handler_list);
    spin_unlock_bh(&iucv_table_lock);
    rc = 0;
    out_mutex:
    mutex_unlock(&iucv_register_mutex);
    return rc;
    }
    EXPORT_SYMBOL(iucv_register);
//
// iucv_unregister - Unregister driver from IUCV.
//
// @handler:  address of iucv handler structure
// @smp: != 0 indicates that the handler can deal with out of order messages
//
#[no_mangle]
pub unsafe extern "C" fn iucv_unregister(handler: *mut iucv_handler, smp: c_int) {
    void iucv_unregister(struct iucv_handler *handler, int smp)
    {
    struct iucv_path *p, *n;
    mutex_lock(&iucv_register_mutex);
    spin_lock_bh(&iucv_table_lock);
// Remove handler from the iucv_handler_list.
    list_del_init(&handler.list);
// Sever all pathids still referring to the handler.
    list_for_each_entry_safe(p, n, &handler.paths, list) {
    iucv_sever_pathid(p.pathid, core::ptr::null_mut());
    iucv_path_table[p.pathid] = core::ptr::null_mut();
    list_del(&p.list);
    iucv_path_free(p);
    }
    spin_unlock_bh(&iucv_table_lock);
    if (!smp)
    iucv_nonsmp_handler--;
    if (list_empty(&iucv_handler_list))
    iucv_disable();
#[no_mangle]
pub unsafe extern "C" fn if(0: !smp && iucv_nonsmp_handler ==) -> else {
    else if (!smp && iucv_nonsmp_handler == 0)
    iucv_setmask_mp();
    mutex_unlock(&iucv_register_mutex);
    }
    EXPORT_SYMBOL(iucv_unregister);
    static int iucv_reboot_event(struct notifier_block *this,
    unsigned long event, void *ptr)
    {
    int i;
    if (cpumask_empty(&iucv_irq_cpumask))
    return NOTIFY_DONE;
    cpus_read_lock();
    on_each_cpu_mask(&iucv_irq_cpumask, iucv_block_cpu, core::ptr::null_mut(), 1);
    preempt_disable();
    for (i = 0; i < iucv_max_pathid; i++) {
    if (iucv_path_table[i])
    iucv_sever_pathid(i, core::ptr::null_mut());
    }
    preempt_enable();
    cpus_read_unlock();
    iucv_disable();
    return NOTIFY_DONE;
    }
    static struct notifier_block iucv_reboot_notifier = {
    .notifier_call = iucv_reboot_event,
    };
//
// iucv_path_accept - Complete the IUCV communication path
//
// @path: address of iucv path structure
// @handler: address of iucv handler structure
// @userdata: 16 bytes of data reflected to the communication partner
// @private: private data passed to interrupt handlers for this path
//
// This function is issued after the user received a connection pending
// external interrupt and now wishes to complete the IUCV communication path.
//
// Returns: the result of the CP IUCV call.
//
    int iucv_path_accept(struct iucv_path *path, struct iucv_handler *handler,
    u8 *userdata, void *private)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
// Prepare parameter block.
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    parm.ctrl.ippathid = path.pathid;
    parm.ctrl.ipmsglim = path.msglim;
    if (userdata)
    memcpy(parm.ctrl.ipuser, userdata, sizeof(parm.ctrl.ipuser));
    parm.ctrl.ipflags1 = path.flags;
    rc = iucv_call_b2f0(IUCV_ACCEPT, parm);
    if (!rc) {
    path.private = private;
    path.msglim = parm.ctrl.ipmsglim;
    path.flags = parm.ctrl.ipflags1;
    }
    out:
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_path_accept);
//
// iucv_path_connect - Establish an IUCV path
//
// @path: address of iucv path structure
// @handler: address of iucv handler structure
// @userid: 8-byte user identification
// @system: 8-byte target system identification
// @userdata: 16 bytes of data reflected to the communication partner
// @private: private data passed to interrupt handlers for this path
//
// This function establishes an IUCV path. Although the connect may complete
// successfully, you are not able to use the path until you receive an IUCV
// Connection Complete external interrupt.
//
// Returns: the result of the CP IUCV call.
//
    int iucv_path_connect(struct iucv_path *path, struct iucv_handler *handler,
    u8 *userid, u8 *system, u8 *userdata,
    void *private)
    {
    union iucv_param *parm;
    int rc;
    spin_lock_bh(&iucv_table_lock);
    iucv_cleanup_queue();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    parm.ctrl.ipmsglim = path.msglim;
    parm.ctrl.ipflags1 = path.flags;
    if (userid) {
    memcpy(parm.ctrl.ipvmid, userid, sizeof(parm.ctrl.ipvmid));
    ASCEBC(parm.ctrl.ipvmid, sizeof(parm.ctrl.ipvmid));
    EBC_TOUPPER(parm.ctrl.ipvmid, sizeof(parm.ctrl.ipvmid));
    }
    if (system) {
    memcpy(parm.ctrl.iptarget, system,
    sizeof(parm.ctrl.iptarget));
    ASCEBC(parm.ctrl.iptarget, sizeof(parm.ctrl.iptarget));
    EBC_TOUPPER(parm.ctrl.iptarget, sizeof(parm.ctrl.iptarget));
    }
    if (userdata)
    memcpy(parm.ctrl.ipuser, userdata, sizeof(parm.ctrl.ipuser));
    rc = iucv_call_b2f0(IUCV_CONNECT, parm);
    if (!rc) {
    if (parm.ctrl.ippathid < iucv_max_pathid) {
    path.pathid = parm.ctrl.ippathid;
    path.msglim = parm.ctrl.ipmsglim;
    path.flags = parm.ctrl.ipflags1;
    path.handler = handler;
    path.private = private;
    list_add_tail(&path.list, &handler.paths);
    iucv_path_table[path.pathid] = path;
    } else {
    iucv_sever_pathid(parm.ctrl.ippathid,
    iucv_error_pathid);
    rc = -EIO;
    }
    }
    out:
    spin_unlock_bh(&iucv_table_lock);
    return rc;
    }
    EXPORT_SYMBOL(iucv_path_connect);
//
// iucv_path_quiesce - Temporarily suspend incoming messages
// @path: address of iucv path structure
// @userdata: 16 bytes of data reflected to the communication partner
//
// This function temporarily suspends incoming messages on an IUCV path.
// You can later reactivate the path by invoking the iucv_resume function.
//
// Returns: the result from the CP IUCV call.
//
#[no_mangle]
pub unsafe extern "C" fn iucv_path_quiesce(path: *mut iucv_path, userdata: *mut u8) -> c_int {
    int iucv_path_quiesce(struct iucv_path *path, u8 *userdata)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    if (userdata)
    memcpy(parm.ctrl.ipuser, userdata, sizeof(parm.ctrl.ipuser));
    parm.ctrl.ippathid = path.pathid;
    rc = iucv_call_b2f0(IUCV_QUIESCE, parm);
    out:
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_path_quiesce);
//
// iucv_path_resume - Resume incoming messages on a suspended IUCV path
//
// @path: address of iucv path structure
// @userdata: 16 bytes of data reflected to the communication partner
//
// This function resumes incoming messages on an IUCV path that has
// been stopped with iucv_path_quiesce.
//
// Returns: the result from the CP IUCV call.
//
#[no_mangle]
pub unsafe extern "C" fn iucv_path_resume(path: *mut iucv_path, userdata: *mut u8) -> c_int {
    int iucv_path_resume(struct iucv_path *path, u8 *userdata)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    if (userdata)
    memcpy(parm.ctrl.ipuser, userdata, sizeof(parm.ctrl.ipuser));
    parm.ctrl.ippathid = path.pathid;
    rc = iucv_call_b2f0(IUCV_RESUME, parm);
    out:
    local_bh_enable();
    return rc;
    }
//
// iucv_path_sever - Terminates an IUCV path.
//
// @path: address of iucv path structure
// @userdata: 16 bytes of data reflected to the communication partner
//
// Returns: the result from the CP IUCV call.
//
#[no_mangle]
pub unsafe extern "C" fn iucv_path_sever(path: *mut iucv_path, userdata: *mut u8) -> c_int {
    int iucv_path_sever(struct iucv_path *path, u8 *userdata)
    {
    int rc;
    preempt_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    if (iucv_active_cpu != smp_processor_id())
    spin_lock_bh(&iucv_table_lock);
    rc = iucv_sever_pathid(path.pathid, userdata);
    iucv_path_table[path.pathid] = core::ptr::null_mut();
    list_del_init(&path.list);
    if (iucv_active_cpu != smp_processor_id())
    spin_unlock_bh(&iucv_table_lock);
    out:
    preempt_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_path_sever);
//
// iucv_message_purge - Cancels a message you have sent.
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @srccls: source class of message
//
// Returns: the result from the CP IUCV call.
//
    int iucv_message_purge(struct iucv_path *path, struct iucv_message *msg,
    u32 srccls)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    parm.purge.ippathid = path.pathid;
    parm.purge.ipmsgid = msg.id;
    parm.purge.ipsrccls = srccls;
    parm.purge.ipflags1 = IUCV_IPSRCCLS | IUCV_IPFGMID | IUCV_IPFGPID;
    rc = iucv_call_b2f0(IUCV_PURGE, parm);
    if (!rc) {
    msg.audit = (*(u32 *) &parm.purge.ipaudit) >> 8;
    msg.tag = parm.purge.ipmsgtag;
    }
    out:
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_message_purge);
//
// iucv_message_receive_iprmdata - Internal function to receive RMDATA
// stored in &struct iucv_message
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: how the message is received (IUCV_IPBUFLST)
// @buffer: address of data buffer or address of struct iucv_array
// @size: length of data buffer
// @residual: number of bytes remaining in the data buffer
//
// Internal function used by iucv_message_receive and __iucv_message_receive
// to receive RMDATA data stored in struct iucv_message.
//
// Returns: 0
//
    static int iucv_message_receive_iprmdata(struct iucv_path *path,
    struct iucv_message *msg,
    u8 flags, void *buffer,
    size_t size, size_t *residual)
    {
    struct iucv_array *array;
    u8 *rmmsg;
    size_t copy;
//
// Message is 8 bytes long and has been stored to the
// message descriptor itself.
//
    if (residual)
// residual = abs(size - 8);
    rmmsg = msg.rmmsg;
    if (flags & IUCV_IPBUFLST) {
// Copy to struct iucv_array.
    size = (size < 8) ? size : 8;
    for (array = buffer; size > 0; array++) {
    copy = min_t(size_t, size, array.length);
    memcpy(dma32_to_virt(array.address), rmmsg, copy);
    rmmsg += copy;
    size -= copy;
    }
    } else {
// Copy to direct buffer.
    memcpy(buffer, rmmsg, min_t(size_t, size, 8));
    }
    return 0;
    }
//
// __iucv_message_receive - Receives messages on an established path (no locking)
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: flags that affect how the message is received (IUCV_IPBUFLST)
// @buffer: address of data buffer or address of struct iucv_array
// @size: length of data buffer
// @residual:
//
// This function receives messages that are being sent to you over
// established paths. This function will deal with RMDATA messages
// embedded in struct iucv_message as well.
//
// Locking:	no locking
//
// Returns: the result from the CP IUCV call.
//
    int __iucv_message_receive(struct iucv_path *path, struct iucv_message *msg,
    u8 flags, void *buffer, size_t size, size_t *residual)
    {
    union iucv_param *parm;
    int rc;
    if (msg.flags & IUCV_IPRMDATA)
    return iucv_message_receive_iprmdata(path, msg, flags,
    buffer, size, residual);
    if (cpumask_empty(&iucv_buffer_cpumask))
    return -EIO;
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    parm.db.ipbfadr1 = virt_to_dma32(buffer);
    parm.db.ipbfln1f = (u32) size;
    parm.db.ipmsgid = msg.id;
    parm.db.ippathid = path.pathid;
    parm.db.iptrgcls = msg.class;
    parm.db.ipflags1 = (flags | IUCV_IPFGPID |
    IUCV_IPFGMID | IUCV_IPTRGCLS);
    rc = iucv_call_b2f0(IUCV_RECEIVE, parm);
    if (!rc || rc == 5) {
    msg.flags = parm.db.ipflags1;
    if (residual)
// residual = parm->db.ipbfln1f;
    }
    return rc;
    }
    EXPORT_SYMBOL(__iucv_message_receive);
//
// iucv_message_receive - Receives messages on an established path, with locking
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: flags that affect how the message is received (IUCV_IPBUFLST)
// @buffer: address of data buffer or address of struct iucv_array
// @size: length of data buffer
// @residual:
//
// This function receives messages that are being sent to you over
// established paths. This function will deal with RMDATA messages
// embedded in struct iucv_message as well.
//
// Locking:	local_bh_enable/local_bh_disable
//
// Returns: the result from the CP IUCV call.
//
    int iucv_message_receive(struct iucv_path *path, struct iucv_message *msg,
    u8 flags, void *buffer, size_t size, size_t *residual)
    {
    int rc;
    if (msg.flags & IUCV_IPRMDATA)
    return iucv_message_receive_iprmdata(path, msg, flags,
    buffer, size, residual);
    local_bh_disable();
    rc = __iucv_message_receive(path, msg, flags, buffer, size, residual);
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_message_receive);
//
// iucv_message_reject - Refuses a specified message
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
//
// The reject function refuses a specified message. Between the time you
// are notified of a message and the time that you complete the message,
// the message may be rejected.
//
// Returns: the result from the CP IUCV call.
//
#[no_mangle]
pub unsafe extern "C" fn iucv_message_reject(path: *mut iucv_path, msg: *mut iucv_message) -> c_int {
    int iucv_message_reject(struct iucv_path *path, struct iucv_message *msg)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    parm.db.ippathid = path.pathid;
    parm.db.ipmsgid = msg.id;
    parm.db.iptrgcls = msg.class;
    parm.db.ipflags1 = (IUCV_IPTRGCLS | IUCV_IPFGMID | IUCV_IPFGPID);
    rc = iucv_call_b2f0(IUCV_REJECT, parm);
    out:
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_message_reject);
//
// iucv_message_reply - Replies to a specified message
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: how the reply is sent (IUCV_IPRMDATA, IUCV_IPPRTY, IUCV_IPBUFLST)
// @reply: address of reply data buffer or address of struct iucv_array
// @size: length of reply data buffer
//
// This function responds to the two-way messages that you receive. You
// must identify completely the message to which you wish to reply. I.e.,
// pathid, msgid, and trgcls. Prmmsg signifies the data is moved into
// the parameter list.
//
// Returns: the result from the CP IUCV call.
//
    int iucv_message_reply(struct iucv_path *path, struct iucv_message *msg,
    u8 flags, void *reply, size_t size)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    if (flags & IUCV_IPRMDATA) {
    parm.dpl.ippathid = path.pathid;
    parm.dpl.ipflags1 = flags;
    parm.dpl.ipmsgid = msg.id;
    parm.dpl.iptrgcls = msg.class;
    memcpy(parm.dpl.iprmmsg, reply, min_t(size_t, size, 8));
    } else {
    parm.db.ipbfadr1 = virt_to_dma32(reply);
    parm.db.ipbfln1f = (u32) size;
    parm.db.ippathid = path.pathid;
    parm.db.ipflags1 = flags;
    parm.db.ipmsgid = msg.id;
    parm.db.iptrgcls = msg.class;
    }
    rc = iucv_call_b2f0(IUCV_REPLY, parm);
    out:
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_message_reply);
//
// __iucv_message_send - Transmits a one-way message, no locking
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: how the message is sent (IUCV_IPRMDATA, IUCV_IPPRTY, IUCV_IPBUFLST)
// @srccls: source class of message
// @buffer: address of send buffer or address of struct iucv_array
// @size: length of send buffer
//
// This function transmits data to another application. Data to be
// transmitted is in a buffer and this is a one-way message and the
// receiver will not reply to the message.
//
// Locking:	no locking
//
// Returns: the result from the CP IUCV call.
//
    int __iucv_message_send(struct iucv_path *path, struct iucv_message *msg,
    u8 flags, u32 srccls, void *buffer, size_t size)
    {
    union iucv_param *parm;
    int rc;
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    if (flags & IUCV_IPRMDATA) {
// Message of 8 bytes can be placed into the parameter list.
    parm.dpl.ippathid = path.pathid;
    parm.dpl.ipflags1 = flags | IUCV_IPNORPY;
    parm.dpl.iptrgcls = msg.class;
    parm.dpl.ipsrccls = srccls;
    parm.dpl.ipmsgtag = msg.tag;
    memcpy(parm.dpl.iprmmsg, buffer, 8);
    } else {
    parm.db.ipbfadr1 = virt_to_dma32(buffer);
    parm.db.ipbfln1f = (u32) size;
    parm.db.ippathid = path.pathid;
    parm.db.ipflags1 = flags | IUCV_IPNORPY;
    parm.db.iptrgcls = msg.class;
    parm.db.ipsrccls = srccls;
    parm.db.ipmsgtag = msg.tag;
    }
    rc = iucv_call_b2f0(IUCV_SEND, parm);
    if (!rc)
    msg.id = parm.db.ipmsgid;
    out:
    return rc;
    }
    EXPORT_SYMBOL(__iucv_message_send);
//
// iucv_message_send - Transmits a one-way message, with locking
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: how the message is sent (IUCV_IPRMDATA, IUCV_IPPRTY, IUCV_IPBUFLST)
// @srccls: source class of message
// @buffer: address of send buffer or address of struct iucv_array
// @size: length of send buffer
//
// This function transmits data to another application. Data to be
// transmitted is in a buffer and this is a one-way message and the
// receiver will not reply to the message.
//
// Locking:	local_bh_enable/local_bh_disable
//
// Returns: the result from the CP IUCV call.
//
    int iucv_message_send(struct iucv_path *path, struct iucv_message *msg,
    u8 flags, u32 srccls, void *buffer, size_t size)
    {
    int rc;
    local_bh_disable();
    rc = __iucv_message_send(path, msg, flags, srccls, buffer, size);
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_message_send);
//
// iucv_message_send2way - Transmits a two-way message
//
// @path: address of iucv path structure
// @msg: address of iucv msg structure
// @flags: how the message is sent and the reply is received
// (IUCV_IPRMDATA, IUCV_IPBUFLST, IUCV_IPPRTY, IUCV_ANSLST)
// @srccls: source class of message
// @buffer: address of send buffer or address of struct iucv_array
// @size: length of send buffer
// @answer: address of answer buffer or address of struct iucv_array
// @asize: size of reply buffer
// @residual: ignored
//
// This function transmits data to another application. Data to be
// transmitted is in a buffer. The receiver of the send is expected to
// reply to the message and a buffer is provided into which IUCV moves
// the reply to this message.
//
// Returns: the result from the CP IUCV call.
//
    int iucv_message_send2way(struct iucv_path *path, struct iucv_message *msg,
    u8 flags, u32 srccls, void *buffer, size_t size,
    void *answer, size_t asize, size_t *residual)
    {
    union iucv_param *parm;
    int rc;
    local_bh_disable();
    if (cpumask_empty(&iucv_buffer_cpumask)) {
    rc = -EIO;
    goto out;
    }
    parm = iucv_param[smp_processor_id()];
    memset(parm, 0, sizeof(union iucv_param));
    if (flags & IUCV_IPRMDATA) {
    parm.dpl.ippathid = path.pathid;
    parm.dpl.ipflags1 = path.flags;	/* priority message */
    parm.dpl.iptrgcls = msg.class;
    parm.dpl.ipsrccls = srccls;
    parm.dpl.ipmsgtag = msg.tag;
    parm.dpl.ipbfadr2 = virt_to_dma32(answer);
    parm.dpl.ipbfln2f = (u32) asize;
    memcpy(parm.dpl.iprmmsg, buffer, 8);
    } else {
    parm.db.ippathid = path.pathid;
    parm.db.ipflags1 = path.flags;	/* priority message */
    parm.db.iptrgcls = msg.class;
    parm.db.ipsrccls = srccls;
    parm.db.ipmsgtag = msg.tag;
    parm.db.ipbfadr1 = virt_to_dma32(buffer);
    parm.db.ipbfln1f = (u32) size;
    parm.db.ipbfadr2 = virt_to_dma32(answer);
    parm.db.ipbfln2f = (u32) asize;
    }
    rc = iucv_call_b2f0(IUCV_SEND, parm);
    if (!rc)
    msg.id = parm.db.ipmsgid;
    out:
    local_bh_enable();
    return rc;
    }
    EXPORT_SYMBOL(iucv_message_send2way);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_path_pending {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iptype: u8,
    pub ipmsglim: u16,
    pub res1: u16,
    pub ipvmid: [u8; 8],
    pub ipuser: [u8; 16],
    pub res3: u32,
    pub ippollfg: u8,
    pub res4: [u8; 3],
    pub __packed: },
//
// iucv_path_pending - Process connection pending work item
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_path_pending(data: *mut iucv_irq_data) {
    static void iucv_path_pending(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_path_pending ipp = (void ),
    pub handler: *mut iucv_handler,
    pub path: *mut iucv_path,
    pub error: *mut c_char,
// New pathid, handler found. Create a new path struct.
    pub iucv_error_no_memory: error =,
    pub GFP_ATOMIC): path = iucv_path_alloc(ipp->ipmsglim, ipp->ipflags1,,
    if (!path)
    pub out_sever: goto,
    pub ipp->ippathid: path->pathid =,
    pub path: iucv_path_table[path->pathid] =,
    pub 8): EBCASC(ipp->ipvmid,,
// Call registered handler until one is found that wants the path.
    list_for_each_entry(handler, &iucv_handler_list, list) {
    if (!handler.path_pending)
//
// Add path to handler to allow a call to iucv_path_sever
// inside the path_pending function. If the handler returns
// an error remove the path from the handler again.
//
    pub &handler->paths): list_add(&path->list,,
    pub handler: path->handler =,
    if (!handler.path_pending(path, ipp.ipvmid, ipp.ipuser))
    pub NULL: path->handler =,
    }
// No handler wanted the path.
    pub NULL: iucv_path_table[path->pathid] =,
    pub iucv_error_no_listener: error =,
    out_sever:
    pub error): iucv_sever_pathid(ipp->ippathid,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_path_complete {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iptype: u8,
    pub ipmsglim: u16,
    pub res1: u16,
    pub res2: [u8; 8],
    pub ipuser: [u8; 16],
    pub res3: u32,
    pub ippollfg: u8,
    pub res4: [u8; 3],
    pub __packed: },
//
// iucv_path_complete - Process connection complete work item
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_path_complete(data: *mut iucv_irq_data) {
    static void iucv_path_complete(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_path_complete ipc = (void ),
    pub iucv_path_table: [*mut *mut iucv_path path =; ipc->ippathid],
    if (path)
    pub ipc->ipflags1: path->flags =,
    if (path && path.handler && path.handler.path_complete)
    pub ipc->ipuser): path->handler->path_complete(path,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_path_severed {
    pub ippathid: u16,
    pub res1: u8,
    pub iptype: u8,
    pub res2: u32,
    pub res3: [u8; 8],
    pub ipuser: [u8; 16],
    pub res4: u32,
    pub ippollfg: u8,
    pub res5: [u8; 3],
    pub __packed: },
//
// iucv_path_severed - Process connection severed work item.
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_path_severed(data: *mut iucv_irq_data) {
    static void iucv_path_severed(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_path_severed ips = (void ),
    pub iucv_path_table: [*mut *mut iucv_path path =; ips->ippathid],
    if (!path || !path.handler)	/* Already severed */
    if (path.handler.path_severed)
    pub ips->ipuser): path->handler->path_severed(path,,
    else {
    pub NULL): iucv_sever_pathid(path->pathid,,
    pub NULL: iucv_path_table[path->pathid] =,
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_path_quiesced {
    pub ippathid: u16,
    pub res1: u8,
    pub iptype: u8,
    pub res2: u32,
    pub res3: [u8; 8],
    pub ipuser: [u8; 16],
    pub res4: u32,
    pub ippollfg: u8,
    pub res5: [u8; 3],
    pub __packed: },
//
// iucv_path_quiesced - Process connection quiesced work item.
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_path_quiesced(data: *mut iucv_irq_data) {
    static void iucv_path_quiesced(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_path_quiesced ipq = (void ),
    pub iucv_path_table: [*mut *mut iucv_path path =; ipq->ippathid],
    if (path && path.handler && path.handler.path_quiesced)
    pub ipq->ipuser): path->handler->path_quiesced(path,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_path_resumed {
    pub ippathid: u16,
    pub res1: u8,
    pub iptype: u8,
    pub res2: u32,
    pub res3: [u8; 8],
    pub ipuser: [u8; 16],
    pub res4: u32,
    pub ippollfg: u8,
    pub res5: [u8; 3],
    pub __packed: },
//
// iucv_path_resumed - Process connection resumed work item.
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_path_resumed(data: *mut iucv_irq_data) {
    static void iucv_path_resumed(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_path_resumed ipr = (void ),
    pub iucv_path_table: [*mut *mut iucv_path path =; ipr->ippathid],
    if (path && path.handler && path.handler.path_resumed)
    pub ipr->ipuser): path->handler->path_resumed(path,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_message_complete {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iptype: u8,
    pub ipmsgid: u32,
    pub ipaudit: u32,
    pub iprmmsg: [u8; 8],
    pub ipsrccls: u32,
    pub ipmsgtag: u32,
    pub res: u32,
    pub ipbfln2f: u32,
    pub ippollfg: u8,
    pub res2: [u8; 3],
    pub __packed: },
//
// iucv_message_complete - Process message complete work item.
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_message_complete(data: *mut iucv_irq_data) {
    static void iucv_message_complete(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_message_complete imc = (void ),
    pub iucv_path_table: [*mut *mut iucv_path path =; imc->ippathid],
    pub msg: iucv_message,
    if (path && path.handler && path.handler.message_complete) {
    pub imc->ipflags1: msg.flags =,
    pub imc->ipmsgid: msg.id =,
    pub imc->ipaudit: msg.audit =,
    pub 8): memcpy(msg.rmmsg, imc->iprmmsg,,
    pub imc->ipsrccls: msg.class =,
    pub imc->ipmsgtag: msg.tag =,
    pub imc->ipbfln2f: msg.length =,
    pub &msg): path->handler->message_complete(path,,
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_message_pending {
    pub ippathid: u16,
    pub ipflags1: u8,
    pub iptype: u8,
    pub ipmsgid: u32,
    pub iptrgcls: u32,
    struct {
    union {
    pub iprmmsg1_u32: u32,
    pub iprmmsg1: [u8; 4],
    pub ln1msg1: },
    union {
    pub ipbfln1f: u32,
    pub iprmmsg2: [u8; 4],
    pub ln1msg2: },
    pub rmmsg: },
    pub res1: [u32; 3],
    pub ipbfln2f: u32,
    pub ippollfg: u8,
    pub res2: [u8; 3],
    pub __packed: },
//
// iucv_message_pending - Process message pending work item.
//
// @data: Pointer to external interrupt buffer
//
// Context: Called from tasklet while holding iucv_table_lock.
//
#[no_mangle]
unsafe extern "C" fn iucv_message_pending(data: *mut iucv_irq_data) {
    static void iucv_message_pending(struct iucv_irq_data *data)
    {
    pub data: *mut *mut *mut iucv_message_pending imp = (void ),
    pub iucv_path_table: [*mut *mut iucv_path path =; imp->ippathid],
    pub msg: iucv_message,
    if (path && path.handler && path.handler.message_pending) {
    pub imp->ipflags1: msg.flags =,
    pub imp->ipmsgid: msg.id =,
    pub imp->iptrgcls: msg.class =,
    if (imp.ipflags1 & IUCV_IPRMDATA) {
    pub 8): memcpy(msg.rmmsg, &imp->rmmsg,,
    pub 8: msg.length =,
    } else
    pub imp->rmmsg.ln1msg2.ipbfln1f: msg.length =,
    pub imp->ipbfln2f: msg.reply_size =,
    pub &msg): path->handler->message_pending(path,,
    }
    }
//
// iucv_tasklet_fn - Process the queue of IRQ buffers
//
// This tasklet loops over the queue of irq buffers created by
// iucv_external_interrupt, calls the appropriate action handler
// and then frees the buffer.
//
#[no_mangle]
unsafe extern "C" fn iucv_tasklet_fn(ignored: c_ulong) {
    static void iucv_tasklet_fn(unsigned long ignored)
    {
    pub ): *mut typedef void iucv_irq_fn(struct iucv_irq_data,
    static iucv_irq_fn *irq_fn[] = {
    [0x02] = iucv_path_complete,
    [0x03] = iucv_path_severed,
    [0x04] = iucv_path_quiesced,
    [0x05] = iucv_path_resumed,
    [0x06] = iucv_message_complete,
    [0x07] = iucv_message_complete,
    [0x08] = iucv_message_pending,
    [0x09] = iucv_message_pending,
}

    LIST_HEAD(task_queue);
    struct iucv_irq_list *p, *n;
// Serialize tasklet, iucv_path_sever and iucv_path_connect.
    if (!spin_trylock(&iucv_table_lock)) {
    tasklet_schedule(&iucv_tasklet);
    return;
    }
    iucv_active_cpu = smp_processor_id();
    spin_lock_irq(&iucv_queue_lock);
    list_splice_init(&iucv_task_queue, &task_queue);
    spin_unlock_irq(&iucv_queue_lock);
    list_for_each_entry_safe(p, n, &task_queue, list) {
    list_del_init(&p.list);
    irq_fn[p.data.iptype](&p.data);
    kfree(p);
    }
    iucv_active_cpu = -1;
    spin_unlock(&iucv_table_lock);
    }
//
// iucv_work_fn - Process the queue of path pending IRQ blocks
//
// This work function loops over the queue of path pending irq blocks
// created by iucv_external_interrupt, calls the appropriate action
// handler and then frees the buffer.
//
#[no_mangle]
unsafe extern "C" fn iucv_work_fn(work: *mut work_struct) {
    static void iucv_work_fn(struct work_struct *work)
    {
    LIST_HEAD(work_queue);
    struct iucv_irq_list *p, *n;
// Serialize tasklet, iucv_path_sever and iucv_path_connect.
    spin_lock_bh(&iucv_table_lock);
    iucv_active_cpu = smp_processor_id();
    spin_lock_irq(&iucv_queue_lock);
    list_splice_init(&iucv_work_queue, &work_queue);
    spin_unlock_irq(&iucv_queue_lock);
    iucv_cleanup_queue();
    list_for_each_entry_safe(p, n, &work_queue, list) {
    list_del_init(&p.list);
    iucv_path_pending(&p.data);
    kfree(p);
    }
    iucv_active_cpu = -1;
    spin_unlock_bh(&iucv_table_lock);
    }
//
// iucv_external_interrupt - Handles external interrupts coming in from CP.
//
// Places the interrupt buffer on a queue and schedules iucv_tasklet_fn().
//
    static void iucv_external_interrupt(struct ext_code ext_code,
    unsigned int param32, unsigned long param64)
    {
    struct iucv_irq_data *p;
    struct iucv_irq_list *work;
    inc_irq_stat(IRQEXT_IUC);
    p = iucv_irq_data[smp_processor_id()];
    if (p.ippathid >= iucv_max_pathid) {
    WARN_ON(p.ippathid >= iucv_max_pathid);
    iucv_sever_pathid(p.ippathid, iucv_error_no_listener);
    return;
    }
    BUG_ON(p.iptype  < 0x01 || p.iptype > 0x09);
    work = kmalloc_obj(struct iucv_irq_list, GFP_ATOMIC);
    if (!work) {
    pr_warn("iucv_external_interrupt: out of memory\n");
    return;
    }
    memcpy(&work.data, p, sizeof(work.data));
    spin_lock(&iucv_queue_lock);
    if (p.iptype == 0x01) {
// Path pending interrupt.
    list_add_tail(&work.list, &iucv_work_queue);
    schedule_work(&iucv_work);
    } else {
// The other interrupts.
    list_add_tail(&work.list, &iucv_task_queue);
    tasklet_schedule(&iucv_tasklet);
    }
    spin_unlock(&iucv_queue_lock);
    }
    struct iucv_interface iucv_if = {
    .message_receive = iucv_message_receive,
    .__message_receive = __iucv_message_receive,
    .message_reply = iucv_message_reply,
    .message_reject = iucv_message_reject,
    .message_send = iucv_message_send,
    .__message_send = __iucv_message_send,
    .message_send2way = iucv_message_send2way,
    .message_purge = iucv_message_purge,
    .path_accept = iucv_path_accept,
    .path_connect = iucv_path_connect,
    .path_quiesce = iucv_path_quiesce,
    .path_resume = iucv_path_resume,
    .path_sever = iucv_path_sever,
    .iucv_register = iucv_register,
    .iucv_unregister = iucv_unregister,
    .bus = core::ptr::null_mut(),
    .root = core::ptr::null_mut(),
    };
    EXPORT_SYMBOL(iucv_if);
    static enum cpuhp_state iucv_online;
//
// iucv_init - Allocates and initializes various data structures.
//
// Returns: 0 on success, return code on failure.
//
#[no_mangle]
unsafe extern "C" fn iucv_init() -> int __init {
    static int __init iucv_init(void)
    {
    int rc;
    if (!machine_is_vm()) {
    rc = -EPROTONOSUPPORT;
    goto out;
    }
    system_ctl_set_bit(0, CR0_IUCV_BIT);
    rc = iucv_query_maxconn();
    if (rc)
    goto out_ctl;
    rc = register_external_irq(EXT_IRQ_IUCV, iucv_external_interrupt);
    if (rc)
    goto out_ctl;
    iucv_root = root_device_register("iucv");
    if (IS_ERR(iucv_root)) {
    rc = PTR_ERR(iucv_root);
    goto out_int;
    }
    rc = cpuhp_setup_state(CPUHP_NET_IUCV_PREPARE, "net/iucv:prepare",
    iucv_cpu_prepare, iucv_cpu_dead);
    if (rc)
    goto out_dev;
    rc = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "net/iucv:online",
    iucv_cpu_online, iucv_cpu_down_prep);
    if (rc < 0)
    goto out_prep;
    iucv_online = rc;
    rc = register_reboot_notifier(&iucv_reboot_notifier);
    if (rc)
    goto out_remove_hp;
    ASCEBC(iucv_error_no_listener, 16);
    ASCEBC(iucv_error_no_memory, 16);
    ASCEBC(iucv_error_pathid, 16);
    iucv_available = 1;
    rc = bus_register(&iucv_bus);
    if (rc)
    goto out_reboot;
    iucv_if.root = iucv_root;
    iucv_if.bus = &iucv_bus;
    return 0;
    out_reboot:
    unregister_reboot_notifier(&iucv_reboot_notifier);
    out_remove_hp:
    cpuhp_remove_state(iucv_online);
    out_prep:
    cpuhp_remove_state(CPUHP_NET_IUCV_PREPARE);
    out_dev:
    root_device_unregister(iucv_root);
    out_int:
    unregister_external_irq(EXT_IRQ_IUCV, iucv_external_interrupt);
    out_ctl:
    system_ctl_clear_bit(0, 1);
    out:
    return rc;
    }
//
// iucv_exit - Frees everything allocated from iucv_init.
//
#[no_mangle]
unsafe extern "C" fn iucv_exit() -> void __exit {
    static void __exit iucv_exit(void)
    {
    struct iucv_irq_list *p, *n;
    spin_lock_irq(&iucv_queue_lock);
    list_for_each_entry_safe(p, n, &iucv_task_queue, list)
    kfree(p);
    list_for_each_entry_safe(p, n, &iucv_work_queue, list)
    kfree(p);
    spin_unlock_irq(&iucv_queue_lock);
    unregister_reboot_notifier(&iucv_reboot_notifier);
    cpuhp_remove_state_nocalls(iucv_online);
    cpuhp_remove_state(CPUHP_NET_IUCV_PREPARE);
    root_device_unregister(iucv_root);
    bus_unregister(&iucv_bus);
    unregister_external_irq(EXT_IRQ_IUCV, iucv_external_interrupt);
    }
    subsys_initcall(iucv_init);
    module_exit(iucv_exit);
    MODULE_AUTHOR("(C) 2001 IBM Corp. by Fritz Elfert <felfert@millenux.com>");
    MODULE_DESCRIPTION("Linux for S/390 IUCV lowlevel driver");
    MODULE_LICENSE("GPL");
