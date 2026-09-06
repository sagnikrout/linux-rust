//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/sysinfo.c
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
// Copyright IBM Corp. 2001, 2009
// Author(s): Ulrich Weigand <Ulrich.Weigand@de.ibm.com>,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
//

    int topology_max_mnest;

#[no_mangle]
unsafe extern "C" fn convert_ext_name(encoding: c_uchar, name: *mut c_char, len: usize) -> bool {
    static bool convert_ext_name(unsigned char encoding, char *name, size_t len)
    {
    switch (encoding) {
    case 1: /* EBCDIC */
    EBCASC(name, len);
    break;
    case 2:	/* UTF-8 */
    break;
    default:
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn stsi_1_1_1(m: *mut seq_file, info: *mut sysinfo_1_1_1) {
    static void stsi_1_1_1(struct seq_file *m, struct sysinfo_1_1_1 *info)
    {
    bool has_var_cap;
    int i;
    if (stsi(info, 1, 1, 1))
    return;
    has_var_cap = !!info.model_var_cap[0];
    EBCASC(info.manufacturer, sizeof(info.manufacturer));
    EBCASC(info.type, sizeof(info.type));
    EBCASC(info.model, sizeof(info.model));
    EBCASC(info.sequence, sizeof(info.sequence));
    EBCASC(info.plant, sizeof(info.plant));
    EBCASC(info.model_capacity, sizeof(info.model_capacity));
    EBCASC(info.model_perm_cap, sizeof(info.model_perm_cap));
    EBCASC(info.model_temp_cap, sizeof(info.model_temp_cap));
    if (has_var_cap)
    EBCASC(info.model_var_cap, sizeof(info.model_var_cap));
    seq_printf(m, "Manufacturer:         %-16.16s\n", info.manufacturer);
    seq_printf(m, "Type:                 %-4.4s\n", info.type);
    if (info.lic)
    seq_printf(m, "LIC Identifier:       %016lx\n", info.lic);
//
// Sigh: the model field has been renamed with System z9
// to model_capacity and a new model field has been added
// after the plant field. To avoid confusing older programs
// the "Model:" prints "model_capacity model" or just
// "model_capacity" if the model string is empty .
//
    seq_printf(m, "Model:                %-16.16s", info.model_capacity);
    if (info.model[0] != '\0')
    seq_printf(m, " %-16.16s", info.model);
    seq_putc(m, '\n');
    seq_printf(m, "Sequence Code:        %-16.16s\n", info.sequence);
    seq_printf(m, "Plant:                %-4.4s\n", info.plant);
    seq_printf(m, "Model Capacity:       %-16.16s %08u\n",
    info.model_capacity, info.model_cap_rating);
    if (info.model_perm_cap_rating)
    seq_printf(m, "Model Perm. Capacity: %-16.16s %08u\n",
    info.model_perm_cap,
    info.model_perm_cap_rating);
    if (info.model_temp_cap_rating)
    seq_printf(m, "Model Temp. Capacity: %-16.16s %08u\n",
    info.model_temp_cap,
    info.model_temp_cap_rating);
    if (has_var_cap && info.model_var_cap_rating)
    seq_printf(m, "Model Var. Capacity:  %-16.16s %08u\n",
    info.model_var_cap,
    info.model_var_cap_rating);
    if (info.ncr)
    seq_printf(m, "Nominal Cap. Rating:  %08u\n", info.ncr);
    if (info.npr)
    seq_printf(m, "Nominal Perm. Rating: %08u\n", info.npr);
    if (info.ntr)
    seq_printf(m, "Nominal Temp. Rating: %08u\n", info.ntr);
    if (has_var_cap && info.nvr)
    seq_printf(m, "Nominal Var. Rating:  %08u\n", info.nvr);
    if (info.cai) {
    seq_printf(m, "Capacity Adj. Ind.:   %d\n", info.cai);
    seq_printf(m, "Capacity Ch. Reason:  %d\n", info.ccr);
    seq_printf(m, "Capacity Transient:   %d\n", info.t);
    }
    if (info.p) {
    for (i = 1; i <= ARRAY_SIZE(info.typepct); i++) {
    seq_printf(m, "Type %d Percentage:    %d\n",
    i, info.typepct[i - 1]);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn stsi_15_1_x(m: *mut seq_file, info: *mut sysinfo_15_1_x) {
    static void stsi_15_1_x(struct seq_file *m, struct sysinfo_15_1_x *info)
    {
    int i;
    seq_putc(m, '\n');
    if (!cpu_has_topology())
    return;
    if (stsi(info, 15, 1, topology_max_mnest))
    return;
    seq_printf(m, "CPU Topology HW:     ");
    for (i = 0; i < TOPOLOGY_NR_MAG; i++)
    seq_printf(m, " %d", info.mag[i]);
    seq_putc(m, '\n');

    store_topology(info);
    seq_printf(m, "CPU Topology SW:     ");
    for (i = 0; i < TOPOLOGY_NR_MAG; i++)
    seq_printf(m, " %d", info.mag[i]);
    seq_putc(m, '\n');

    }
#[no_mangle]
unsafe extern "C" fn stsi_1_2_2(m: *mut seq_file, info: *mut sysinfo_1_2_2) {
    static void stsi_1_2_2(struct seq_file *m, struct sysinfo_1_2_2 *info)
    {
    struct sysinfo_1_2_2_extension *ext;
    int i;
    if (stsi(info, 1, 2, 2))
    return;
    ext = (struct sysinfo_1_2_2_extension *)
    ((unsigned long) info + info.acc_offset);
    seq_printf(m, "CPUs Total:           %d\n", info.cpus_total);
    seq_printf(m, "CPUs Configured:      %d\n", info.cpus_configured);
    seq_printf(m, "CPUs Standby:         %d\n", info.cpus_standby);
    seq_printf(m, "CPUs Reserved:        %d\n", info.cpus_reserved);
    if (info.mt_installed) {
    seq_printf(m, "CPUs G-MTID:          %d\n", info.mt_gtid);
    seq_printf(m, "CPUs S-MTID:          %d\n", info.mt_stid);
    }
//
// Sigh 2. According to the specification the alternate
// capability field is a 32 bit floating point number
// if the higher order 8 bits are not zero. Printing
// a floating point number in the kernel is a no-no,
// always print the number as 32 bit unsigned integer.
// The user-space needs to know about the strange
// encoding of the alternate cpu capability.
//
    seq_printf(m, "Capability:           %u", info.capability);
    if (info.format == 1)
    seq_printf(m, " %u", ext.alt_capability);
    seq_putc(m, '\n');
    if (info.nominal_cap)
    seq_printf(m, "Nominal Capability:   %d\n", info.nominal_cap);
    if (info.secondary_cap)
    seq_printf(m, "Secondary Capability: %d\n", info.secondary_cap);
    for (i = 2; i <= info.cpus_total; i++) {
    seq_printf(m, "Adjustment %02d-way:    %u",
    i, info.adjustment[i-2]);
    if (info.format == 1)
    seq_printf(m, " %u", ext.alt_adjustment[i-2]);
    seq_putc(m, '\n');
    }
    }
#[no_mangle]
unsafe extern "C" fn stsi_2_2_2(m: *mut seq_file, info: *mut sysinfo_2_2_2) {
    static void stsi_2_2_2(struct seq_file *m, struct sysinfo_2_2_2 *info)
    {
    if (stsi(info, 2, 2, 2))
    return;
    EBCASC(info.name, sizeof(info.name));
    seq_putc(m, '\n');
    seq_printf(m, "LPAR Number:          %d\n", info.lpar_number);
    seq_printf(m, "LPAR Characteristics: ");
    if (info.characteristics & LPAR_CHAR_DEDICATED)
    seq_printf(m, "Dedicated ");
    if (info.characteristics & LPAR_CHAR_SHARED)
    seq_printf(m, "Shared ");
    if (info.characteristics & LPAR_CHAR_LIMITED)
    seq_printf(m, "Limited ");
    seq_putc(m, '\n');
    seq_printf(m, "LPAR Name:            %-8.8s\n", info.name);
    seq_printf(m, "LPAR Adjustment:      %d\n", info.caf);
    seq_printf(m, "LPAR CPUs Total:      %d\n", info.cpus_total);
    seq_printf(m, "LPAR CPUs Configured: %d\n", info.cpus_configured);
    seq_printf(m, "LPAR CPUs Standby:    %d\n", info.cpus_standby);
    seq_printf(m, "LPAR CPUs Reserved:   %d\n", info.cpus_reserved);
    seq_printf(m, "LPAR CPUs Dedicated:  %d\n", info.cpus_dedicated);
    seq_printf(m, "LPAR CPUs Shared:     %d\n", info.cpus_shared);
    if (info.mt_installed) {
    seq_printf(m, "LPAR CPUs G-MTID:     %d\n", info.mt_gtid);
    seq_printf(m, "LPAR CPUs S-MTID:     %d\n", info.mt_stid);
    seq_printf(m, "LPAR CPUs PS-MTID:    %d\n", info.mt_psmtid);
    }
    if (convert_ext_name(info.vsne, info.ext_name, sizeof(info.ext_name))) {
    seq_printf(m, "LPAR Extended Name:   %-.256s\n", info.ext_name);
    seq_printf(m, "LPAR UUID:            %pUb\n", &info.uuid);
    }
    }
    static void print_ext_name(struct seq_file *m, int lvl,
    struct sysinfo_3_2_2 *info)
    {
    let mut len: usize = sizeof(info.ext_names[lvl]);
    if (!convert_ext_name(info.vm[lvl].evmne, info.ext_names[lvl], len))
    return;
    seq_printf(m, "VM%02d Extended Name:   %-.256s\n", lvl,
    info.ext_names[lvl]);
    }
#[no_mangle]
unsafe extern "C" fn print_uuid(m: *mut seq_file, i: c_int, info: *mut sysinfo_3_2_2) {
    static void print_uuid(struct seq_file *m, int i, struct sysinfo_3_2_2 *info)
    {
    if (uuid_is_null(&info.vm[i].uuid))
    return;
    seq_printf(m, "VM%02d UUID:            %pUb\n", i, &info.vm[i].uuid);
    }
#[no_mangle]
unsafe extern "C" fn stsi_3_2_2(m: *mut seq_file, info: *mut sysinfo_3_2_2) {
    static void stsi_3_2_2(struct seq_file *m, struct sysinfo_3_2_2 *info)
    {
    int i;
    if (stsi(info, 3, 2, 2))
    return;
    for (i = 0; i < info.count; i++) {
    EBCASC(info.vm[i].name, sizeof(info.vm[i].name));
    EBCASC(info.vm[i].cpi, sizeof(info.vm[i].cpi));
    seq_putc(m, '\n');
    seq_printf(m, "VM%02d Name:            %-8.8s\n", i, info.vm[i].name);
    seq_printf(m, "VM%02d Control Program: %-16.16s\n", i, info.vm[i].cpi);
    seq_printf(m, "VM%02d Adjustment:      %d\n", i, info.vm[i].caf);
    seq_printf(m, "VM%02d CPUs Total:      %d\n", i, info.vm[i].cpus_total);
    seq_printf(m, "VM%02d CPUs Configured: %d\n", i, info.vm[i].cpus_configured);
    seq_printf(m, "VM%02d CPUs Standby:    %d\n", i, info.vm[i].cpus_standby);
    seq_printf(m, "VM%02d CPUs Reserved:   %d\n", i, info.vm[i].cpus_reserved);
    print_ext_name(m, i, info);
    print_uuid(m, i, info);
    }
    }
#[no_mangle]
unsafe extern "C" fn sysinfo_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int sysinfo_show(struct seq_file *m, void *v)
    {
    void *info = (void *)get_zeroed_page(GFP_KERNEL);
    int level;
    if (!info)
    return 0;
    level = stsi(core::ptr::null_mut(), 0, 0, 0);
    if (level >= 1)
    stsi_1_1_1(m, info);
    if (level >= 1)
    stsi_15_1_x(m, info);
    if (level >= 1)
    stsi_1_2_2(m, info);
    if (level >= 2)
    stsi_2_2_2(m, info);
    if (level >= 3)
    stsi_3_2_2(m, info);
    free_page((unsigned long)info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysinfo_create_proc() -> int __init {
    static int __init sysinfo_create_proc(void)
    {
    proc_create_single("sysinfo", 0444, core::ptr::null_mut(), sysinfo_show);
    return 0;
    }
    device_initcall(sysinfo_create_proc);

//
// Service levels interface.
//
    static DECLARE_RWSEM(service_level_sem);
    static LIST_HEAD(service_level_list);
#[no_mangle]
pub unsafe extern "C" fn register_service_level(slr: *mut service_level) -> c_int {
    int register_service_level(struct service_level *slr)
    {
    struct service_level *ptr;
    down_write(&service_level_sem);
    list_for_each_entry(ptr, &service_level_list, list)
    if (ptr == slr) {
    up_write(&service_level_sem);
    return -EEXIST;
    }
    list_add_tail(&slr.list, &service_level_list);
    up_write(&service_level_sem);
    return 0;
    }
    EXPORT_SYMBOL(register_service_level);
#[no_mangle]
pub unsafe extern "C" fn unregister_service_level(slr: *mut service_level) -> c_int {
    int unregister_service_level(struct service_level *slr)
    {
    struct service_level *ptr, *next;
    let mut rc: c_int = -ENOENT;
    down_write(&service_level_sem);
    list_for_each_entry_safe(ptr, next, &service_level_list, list) {
    if (ptr != slr)
    continue;
    list_del(&ptr.list);
    rc = 0;
    break;
    }
    up_write(&service_level_sem);
    return rc;
    }
    EXPORT_SYMBOL(unregister_service_level);
    static void *service_level_start(struct seq_file *m, loff_t *pos)
    __acquires_shared(service_level_sem)
    {
    down_read(&service_level_sem);
    return seq_list_start(&service_level_list, *pos);
    }
    static void *service_level_next(struct seq_file *m, void *p, loff_t *pos)
    {
    return seq_list_next(p, &service_level_list, pos);
    }
#[no_mangle]
unsafe extern "C" fn service_level_stop(m: *mut seq_file, p: *mut c_void) {
    static void service_level_stop(struct seq_file *m, void *p)
    __releases_shared(service_level_sem)
    {
    up_read(&service_level_sem);
    }
#[no_mangle]
unsafe extern "C" fn service_level_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int service_level_show(struct seq_file *m, void *p)
    {
    struct service_level *slr;
    slr = list_entry(p, struct service_level, list);
    slr.seq_print(m, slr);
    return 0;
    }
    static const struct seq_operations service_level_seq_ops = {
    .start		= service_level_start,
    .next		= service_level_next,
    .stop		= service_level_stop,
    .show		= service_level_show
    };
    static void service_level_vm_print(struct seq_file *m,
    struct service_level *slr)
    {
    char *query_buffer, *str;
    query_buffer = kmalloc(1024, GFP_KERNEL);
    if (!query_buffer)
    return;
    cpcmd("QUERY CPLEVEL", query_buffer, 1024, core::ptr::null_mut());
    str = strchr(query_buffer, '\n');
    if (str)
// str = 0;
    seq_printf(m, "VM: %s\n", query_buffer);
    kfree(query_buffer);
    }
    static struct service_level service_level_vm = {
    .seq_print = service_level_vm_print
    };
#[no_mangle]
unsafe extern "C" fn create_proc_service_level() -> __init int {
    static __init int create_proc_service_level(void)
    {
    proc_create_seq("service_levels", 0, core::ptr::null_mut(), &service_level_seq_ops);
    if (machine_is_vm())
    register_service_level(&service_level_vm);
    return 0;
    }
    subsys_initcall(create_proc_service_level);
//
// CPU capability might have changed. Therefore recalculate loops_per_jiffy.
//
#[no_mangle]
pub unsafe extern "C" fn s390_adjust_jiffies() {
    void s390_adjust_jiffies(void)
    {
    DECLARE_KERNEL_FPU_ONSTACK16(fpu);
    struct sysinfo_1_2_2 *info;
    unsigned long capability;
    info = (void *) get_zeroed_page(GFP_KERNEL);
    if (!info)
    return;
    if (stsi(info, 1, 2, 2) == 0) {
//
// Major sigh. The cpu capability encoding is "special".
// If the first 9 bits of info->capability are 0 then it
// is a 32 bit unsigned integer in the range 0 .. 2^23.
// If the first 9 bits are != 0 then it is a 32 bit float.
// In addition a lower value indicates a proportionally
// higher cpu capacity. Bogomips are the other way round.
// To get to a halfway suitable number we divide 1e7
// by the cpu capability number. Yes, that means a floating
// point division ..
//
    kernel_fpu_begin(&fpu, KERNEL_FPR);
    fpu_sfpc(0);
    if (info.capability & 0xff800000)
    fpu_ldgr(2, info.capability);
    else
    fpu_cefbr(2, info.capability);
    fpu_cefbr(0, 10000000);
    fpu_debr(0, 2);
    capability = fpu_cgebr(0, 5);
    kernel_fpu_end(&fpu, KERNEL_FPR);
    } else
//
// Really old machine without stsi block for basic
// cpu information. Report 42.0 bogomips.
//
    capability = 42;
    loops_per_jiffy = capability * (500000/HZ);
    free_page((unsigned long) info);
    }
//
// calibrate the delay loop
//
#[no_mangle]
pub unsafe extern "C" fn calibrate_delay() {
    void calibrate_delay(void)
    {
    s390_adjust_jiffies();
// Print the good old Bogomips line ..
    printk(KERN_DEBUG "Calibrating delay loop (skipped)... "
    "%lu.%02lu BogoMIPS preset\n", loops_per_jiffy/(500000/HZ),
    (loops_per_jiffy/(5000/HZ)) % 100);
    }

    static int stsi_open_##fc##_##s1##_##s2(struct inode *inode, struct file *file)\
    {									       \
    file.private_data = (void *) get_zeroed_page(GFP_KERNEL);	       \
    if (!file.private_data)					       \
    return -ENOMEM;						       \
    if (stsi(file.private_data, fc, s1, s2)) {			       \
    free_page((unsigned long)file.private_data);		       \
    file.private_data = core::ptr::null_mut();				       \
    return -EACCES;						       \
    }								       \
    return nonseekable_open(inode, file);				       \
    }									       \
    \
    static const struct file_operations stsi_##fc##_##s1##_##s2##_fs_ops = {       \
    .open		= stsi_open_##fc##_##s1##_##s2,			       \
    .release	= stsi_release,					       \
    .read		= stsi_read,					       \
    };
#[no_mangle]
unsafe extern "C" fn stsi_release(inode: *mut inode, file: *mut file) -> c_int {
    static int stsi_release(struct inode *inode, struct file *file)
    {
    free_page((unsigned long)file.private_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stsi_read(file: *mut file, buf: *mut char __user, size: usize, ppos: *mut loff_t) -> isize {
    static ssize_t stsi_read(struct file *file, char __user *buf, size_t size, loff_t *ppos)
    {
    return simple_read_from_buffer(buf, size, ppos, file.private_data, PAGE_SIZE);
    }
    STSI_FILE( 1, 1, 1);
    STSI_FILE( 1, 2, 1);
    STSI_FILE( 1, 2, 2);
    STSI_FILE( 2, 2, 1);
    STSI_FILE( 2, 2, 2);
    STSI_FILE( 3, 2, 2);
    STSI_FILE(15, 1, 2);
    STSI_FILE(15, 1, 3);
    STSI_FILE(15, 1, 4);
    STSI_FILE(15, 1, 5);
    STSI_FILE(15, 1, 6);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stsi_file {
    pub fops: *const file_operations,
    pub name: *mut c_char,
}

    static struct stsi_file stsi_file[] __initdata = {
    {.fops = &stsi_1_1_1_fs_ops,  .name =  "1_1_1"},
    {.fops = &stsi_1_2_1_fs_ops,  .name =  "1_2_1"},
    {.fops = &stsi_1_2_2_fs_ops,  .name =  "1_2_2"},
    {.fops = &stsi_2_2_1_fs_ops,  .name =  "2_2_1"},
    {.fops = &stsi_2_2_2_fs_ops,  .name =  "2_2_2"},
    {.fops = &stsi_3_2_2_fs_ops,  .name =  "3_2_2"},
    {.fops = &stsi_15_1_2_fs_ops, .name = "15_1_2"},
    {.fops = &stsi_15_1_3_fs_ops, .name = "15_1_3"},
    {.fops = &stsi_15_1_4_fs_ops, .name = "15_1_4"},
    {.fops = &stsi_15_1_5_fs_ops, .name = "15_1_5"},
    {.fops = &stsi_15_1_6_fs_ops, .name = "15_1_6"},
    };
    static u8 stsi_0_0_0;
#[no_mangle]
unsafe extern "C" fn stsi_init_debugfs() -> __init int {
    static __init int stsi_init_debugfs(void)
    {
    struct dentry *stsi_root;
    struct stsi_file *sf;
    int lvl, i;
    stsi_root = debugfs_create_dir("stsi", arch_debugfs_dir);
    lvl = stsi(core::ptr::null_mut(), 0, 0, 0);
    if (lvl > 0)
    stsi_0_0_0 = lvl;
    debugfs_create_u8("0_0_0", 0400, stsi_root, &stsi_0_0_0);
    for (i = 0; i < ARRAY_SIZE(stsi_file); i++) {
    sf = &stsi_file[i];
    debugfs_create_file(sf.name, 0400, stsi_root, core::ptr::null_mut(), sf.fops);
    }
    if (IS_ENABLED(CONFIG_SCHED_TOPOLOGY) && cpu_has_topology()) {
    char link_to[10];
    snprintf(link_to, sizeof(link_to), "15_1_%d", topology_mnest_limit());
    debugfs_create_symlink("topology", stsi_root, link_to);
    }
    return 0;
    }
    device_initcall(stsi_init_debugfs);
