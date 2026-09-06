//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kexec/core_64.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// PPC64 code to handle Linux booting another kernel.
//
// Copyright (C) 2004-2005, IBM Corp.
//
// Created by: Milton D Miller II
//

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> c_int {
    int machine_kexec_prepare(struct kimage *image)
    {
    int i;
    unsigned long begin, end;	/* limits of segment */
    unsigned long low, high;	/* limits of blocked memory range */
    struct device_node *node;
    const unsigned long *basep;
    const unsigned int *sizep;
//
// Since we use the kernel fault handlers and paging code to
// handle the virtual mode, we must make sure no destination
// overlaps kernel static data or bss.
//
    for (i = 0; i < image.nr_segments; i++)
    if (image.segment[i].mem < __pa(_end))
    return -ETXTBSY;
// We also should not overwrite the tce tables
    for_each_node_by_type(node, "pci") {
    basep = of_get_property(node, "linux,tce-base", core::ptr::null_mut());
    sizep = of_get_property(node, "linux,tce-size", core::ptr::null_mut());
    if (basep == core::ptr::null_mut() || sizep == core::ptr::null_mut())
    continue;
    low = *basep;
    high = low + (*sizep);
    for (i = 0; i < image.nr_segments; i++) {
    begin = image.segment[i].mem;
    end = begin + image.segment[i].memsz;
    if ((begin < high) && (end > low)) {
    of_node_put(node);
    return -ETXTBSY;
    }
    }
    }
    return 0;
    }
// Called during kexec sequence with MMU off
#[no_mangle]
unsafe extern "C" fn copy_segments(ind: c_ulong) -> notrace void {
    static notrace void copy_segments(unsigned long ind)
    {
    unsigned long entry;
    unsigned long *ptr;
    void *dest;
    void *addr;
//
// We rely on kexec_load to create a lists that properly
// initializes these pointers before they are used.
// We will still crash if the list is wrong, but at least
// the compiler will be quiet.
//
    ptr = core::ptr::null_mut();
    dest = core::ptr::null_mut();
    for (entry = ind; !(entry & IND_DONE); entry = *ptr++) {
    addr = __va(entry & PAGE_MASK);
    switch (entry & IND_FLAGS) {
    case IND_DESTINATION:
    dest = addr;
    break;
    case IND_INDIRECTION:
    ptr = addr;
    break;
    case IND_SOURCE:
    copy_page(dest, addr);
    dest += PAGE_SIZE;
    }
    }
    }
// Called during kexec sequence with MMU off
#[no_mangle]
pub unsafe extern "C" fn kexec_copy_flush(image: *mut kimage) -> notrace void {
    notrace void kexec_copy_flush(struct kimage *image)
    {
    long i, nr_segments = image.nr_segments;
    struct  kexec_segment ranges[KEXEC_SEGMENT_MAX];
// save the ranges on the stack to efficiently flush the icache
    memcpy(ranges, image.segment, sizeof(ranges));
//
// After this call we may not use anything allocated in dynamic
// memory, including *image.
//
// Only globals and the stack are allowed.
//
    copy_segments(image.head);
//
// we need to clear the icache for all dest pages sometime,
// including ones that were in place on the original copy
//
    for (i = 0; i < nr_segments; i++)
    flush_icache_range((unsigned long)__va(ranges[i].mem),
    (unsigned long)__va(ranges[i].mem + ranges[i].memsz));
    }

    let mut kexec_all_irq_disabled: static int = 0;
#[no_mangle]
unsafe extern "C" fn kexec_smp_down(arg: *mut c_void) {
    static void kexec_smp_down(void *arg)
    {
    local_irq_disable();
    hard_irq_disable();
    mb(); /* make sure our irqs are disabled before we say they are */
    get_paca().kexec_state = KEXEC_STATE_IRQS_OFF;
    while(kexec_all_irq_disabled == 0)
    cpu_relax();
    mb(); /* make sure all irqs are disabled before this */
    hw_breakpoint_disable();
//
// Now every CPU has IRQs off, we can clear out any pending
// IPIs and be sure that no more will come in after this.
//
    if (ppc_md.kexec_cpu_down)
    ppc_md.kexec_cpu_down(0, 1);
    reset_sprs();
    kexec_smp_wait();
// NOTREACHED
    }
#[no_mangle]
unsafe extern "C" fn kexec_prepare_cpus_wait(wait_state: c_int) {
    static void kexec_prepare_cpus_wait(int wait_state)
    {
    int my_cpu, i, notified=-1;
    hw_breakpoint_disable();
    my_cpu = raw_smp_processor_id();
// Make sure each CPU has at least made it to the state we need.
//
// FIXME: There is a (slim) chance of a problem if not all of the CPUs
// are correctly onlined.  If somehow we start a CPU on boot with RTAS
// start-cpu, but somehow that CPU doesn't write callin_cpu_map[] in
// time, the boot CPU will timeout.  If it does eventually execute
// stuff, the secondary will start up (paca_ptrs[]->cpu_start was
// written) and get into a peculiar state.
// If the platform supports smp_ops->take_timebase(), the secondary CPU
// will probably be spinning in there.  If not (i.e. pseries), the
// secondary will continue on and try to online itself/idle/etc. If it
// survives that, we need to find these
// possible-but-not-online-but-should-be CPUs and chaperone them into
// kexec_smp_wait().
//
    for_each_online_cpu(i) {
    if (i == my_cpu)
    continue;
    while (paca_ptrs[i].kexec_state < wait_state) {
    barrier();
    if (i != notified) {
    printk(KERN_INFO "kexec: waiting for cpu %d "
    "(physical %d) to enter %i state\n",
    i, paca_ptrs[i].hw_cpu_id, wait_state);
    notified = i;
    }
    }
    }
    mb();
    }
//
// The add_cpu() call in wake_offline_cpus() can fail as cpu_bootable()
// returns false for CPUs that fail the cpu_smt_thread_allowed() check
// or non primary threads if SMT is disabled. Re-enable SMT and set the
// number of SMT threads to threads per core.
//
#[no_mangle]
unsafe extern "C" fn kexec_smt_reenable() {
    static void kexec_smt_reenable(void)
    {

    lock_device_hotplug();
    cpu_smt_num_threads = threads_per_core;
    cpu_smt_control = CPU_SMT_ENABLED;
    unlock_device_hotplug();

    }
//
// We need to make sure each present CPU is online.  The next kernel will scan
// the device tree and assume primary threads are online and query secondary
// threads via RTAS to online them if required.  If we don't online primary
// threads, they will be stuck.  However, we also online secondary threads as we
// may be using 'cede offline'.  In this case RTAS doesn't see the secondary
// threads as offline -- and again, these CPUs will be stuck.
//
// So, we online all CPUs that should be running, including secondary threads.
//
#[no_mangle]
unsafe extern "C" fn wake_offline_cpus() {
    static void wake_offline_cpus(void)
    {
    let mut cpu: c_int = 0;
    kexec_smt_reenable();
    for_each_present_cpu(cpu) {
    if (!cpu_online(cpu)) {
    printk(KERN_INFO "kexec: Waking offline cpu %d.\n",
    cpu);
    WARN_ON(add_cpu(cpu));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn kexec_prepare_cpus() {
    static void kexec_prepare_cpus(void)
    {
    wake_offline_cpus();
    smp_call_function(kexec_smp_down, core::ptr::null_mut(), /* wait */0);
    local_irq_disable();
    hard_irq_disable();
    mb(); /* make sure IRQs are disabled before we say they are */
    get_paca().kexec_state = KEXEC_STATE_IRQS_OFF;
    kexec_prepare_cpus_wait(KEXEC_STATE_IRQS_OFF);
// we are sure every CPU has IRQs off at this point
    kexec_all_irq_disabled = 1;
//
// Before removing MMU mappings make sure all CPUs have entered real
// mode:
//
    kexec_prepare_cpus_wait(KEXEC_STATE_REAL_MODE);
// after we tell the others to go down
    if (ppc_md.kexec_cpu_down)
    ppc_md.kexec_cpu_down(0, 0);
    }

#[no_mangle]
unsafe extern "C" fn kexec_prepare_cpus() {
    static void kexec_prepare_cpus(void)
    {
//
// move the secondarys to us so that we can copy
// the new kernel 0-0x100 safely
//
// do this if kexec in setup.c ?
//
// We need to release the cpus if we are ever going from an
// UP to an SMP kernel.
//
    smp_release_cpus();
    if (ppc_md.kexec_cpu_down)
    ppc_md.kexec_cpu_down(0, 0);
    local_irq_disable();
    hard_irq_disable();
    }

//
// kexec thread structure and stack.
//
// We need to make sure that this is 16384-byte aligned due to the
// way process stacks are handled.  It also must be statically allocated
// or allocated as part of the kimage, because everything else may be
// overwritten when we copy the kexec image.  We piggyback on the
// "init_task" linker section here to statically allocate a stack.
//
// We could use a smaller stack if we don't care about anything using
// current, but that audit has not been performed.
//
    let mut kexec_stack: static union thread_union = { };
//
// For similar reasons to the stack above, the kexecing CPU needs to be on a
// static PACA; we switch to kexec_paca.
//
    static struct paca_struct kexec_paca;
// Our assembly helper, in misc_64.S
    extern void kexec_sequence(void *newstack, unsigned long start,
    void *image, void *control,
    void (*clear_all)(void),
    bool copy_with_mmu_off) __noreturn;
// too late to fail here
#[no_mangle]
pub unsafe extern "C" fn default_machine_kexec(image: *mut kimage) {
    void default_machine_kexec(struct kimage *image)
    {
    bool copy_with_mmu_off;
// prepare control code if any
//
// If the kexec boot is the normal one, need to shutdown other cpus
// into our wait loop and quiesce interrupts.
// Otherwise, in the case of crashed mode (crashing_cpu >= 0),
// stopping other CPUs and collecting their pt_regs is done before
// using debugger IPI.
//
    if (!kdump_in_progress())
    kexec_prepare_cpus();

//
// This must be done after other CPUs have shut down, otherwise they
// could execute the 'scv' instruction, which is not supported with
// reloc disabled (see configure_exceptions()).
//
    if (firmware_has_feature(FW_FEATURE_SET_MODE))
    pseries_disable_reloc_on_exc();

    printk("kexec: Starting switchover sequence.\n");
// switch to a staticly allocated stack.  Based on irq stack code.
// We setup preempt_count to avoid using VMX in memcpy.
// XXX: the task struct will likely be invalid once we do the copy!
//
    current_thread_info().flags = 0;
    current_thread_info().preempt_count = HARDIRQ_OFFSET;
// We need a static PACA, too; copy this CPU's PACA over and switch to
// it. Also poison per_cpu_offset and NULL lppaca to catch anyone using
// non-static data.
//
    memcpy(&kexec_paca, get_paca(), sizeof(struct paca_struct));
    kexec_paca.data_offset = 0xedeaddeadeeeeeeeUL;

    kexec_paca.lppaca_ptr = core::ptr::null_mut();

    if (is_secure_guest() && !(image.preserve_context ||
    image.type == KEXEC_TYPE_CRASH)) {
    uv_unshare_all_pages();
    printk("kexec: Unshared all shared pages.\n");
    }
    paca_ptrs[kexec_paca.paca_index] = &kexec_paca;
    setup_paca(&kexec_paca);
//
// The lppaca should be unregistered at this point so the HV won't
// touch it. In the case of a crash, none of the lppacas are
// unregistered so there is not much we can do about it here.
//
// On Book3S, the copy must happen with the MMU off if we are either
// using Radix page tables or we are not in an LPAR since we can
// overwrite the page tables while copying.
//
// In an LPAR, we keep the MMU on otherwise we can't access beyond
// the RMA. On BookE there is no real MMU off mode, so we have to
// keep it enabled as well (but then we have bolted TLB entries).
//

    copy_with_mmu_off = false;

    copy_with_mmu_off = radix_enabled() ||
    !(firmware_has_feature(FW_FEATURE_LPAR) ||
    firmware_has_feature(FW_FEATURE_PS3_LV1));

// Some things are best done in assembly.  Finding globals with
// a toc is easier in C, so pass in what we can.
//
    kexec_sequence(&kexec_stack, image.start, image,
    page_address(image.control_code_page),
    mmu_cleanup_all, copy_with_mmu_off);
// NOTREACHED
    }

// Values we need to export to the second kernel via the device tree.
    static __be64 htab_base;
    static __be64 htab_size;
    static struct property htab_base_prop = {
    .name = "linux,htab-base",
    .length = sizeof(unsigned long),
    .value = &htab_base,
    };
    static struct property htab_size_prop = {
    .name = "linux,htab-size",
    .length = sizeof(unsigned long),
    .value = &htab_size,
    };
#[no_mangle]
unsafe extern "C" fn export_htab_values() -> int __init {
    static int __init export_htab_values(void)
    {
    struct device_node *node;
// On machines with no htab htab_address is NULL
    if (!htab_address)
    return -ENODEV;
    node = of_find_node_by_path("/chosen");
    if (!node)
    return -ENODEV;
// remove any stale properties so ours can be found
    of_remove_property(node, of_find_property(node, htab_base_prop.name, core::ptr::null_mut()));
    of_remove_property(node, of_find_property(node, htab_size_prop.name, core::ptr::null_mut()));
    htab_base = cpu_to_be64(__pa(htab_address));
    of_add_property(node, &htab_base_prop);
    htab_size = cpu_to_be64(htab_size_bytes);
    of_add_property(node, &htab_size_prop);
    of_node_put(node);
    return 0;
    }
    late_initcall(export_htab_values);

//
// add_node_props - Reads node properties from device node structure and add
// them to fdt.
// @fdt:            Flattened device tree of the kernel
// @node_offset:    offset of the node to add a property at
// @dn:             device node pointer
//
// Returns 0 on success, negative errno on error.
//
#[no_mangle]
unsafe extern "C" fn add_node_props(fdt: *mut c_void, node_offset: c_int, dn: *const device_node) -> c_int {
    static int add_node_props(void *fdt, int node_offset, const struct device_node *dn)
    {
    let mut ret: c_int = 0;
    struct property *pp;
    if (!dn)
    return -EINVAL;
    for_each_property_of_node(dn, pp) {
    ret = fdt_setprop(fdt, node_offset, pp.name, pp.value, pp.length);
    if (ret < 0) {
    pr_err("Unable to add %s property: %s\n", pp.name, fdt_strerror(ret));
    return ret;
    }
    }
    return ret;
    }
//
// update_cpus_node - Update cpus node of flattened device tree using of_root
// device node.
// @fdt:              Flattened device tree of the kernel.
//
// Returns 0 on success, negative errno on error.
//
// Note: expecting no subnodes under /cpus/<node> with device_type == "cpu".
// If this changes, update this function to include them.
//
#[no_mangle]
pub unsafe extern "C" fn update_cpus_node(fdt: *mut c_void) -> c_int {
    int update_cpus_node(void *fdt)
    {
    int prev_node_offset;
    const char *device_type;
    const struct fdt_property *prop;
    struct device_node *cpus_node, *dn;
    int cpus_offset, cpus_subnode_offset, ret = 0;
    cpus_offset = fdt_path_offset(fdt, "/cpus");
    if (cpus_offset < 0 && cpus_offset != -FDT_ERR_NOTFOUND) {
    pr_err("Malformed device tree: error reading /cpus node: %s\n",
    fdt_strerror(cpus_offset));
    return cpus_offset;
    }
    prev_node_offset = cpus_offset;
// Delete sub-nodes of /cpus node with device_type == "cpu"
    for (cpus_subnode_offset = fdt_first_subnode(fdt, cpus_offset); cpus_subnode_offset >= 0;) {
// Ignore nodes that do not have a device_type property or device_type != "cpu"
    prop = fdt_get_property(fdt, cpus_subnode_offset, "device_type", core::ptr::null_mut());
    if (!prop || strcmp(prop.data, "cpu")) {
    prev_node_offset = cpus_subnode_offset;
    goto next_node;
    }
    ret = fdt_del_node(fdt, cpus_subnode_offset);
    if (ret < 0) {
    pr_err("Failed to delete a cpus sub-node: %s\n", fdt_strerror(ret));
    return ret;
    }
    next_node:
    if (prev_node_offset == cpus_offset)
    cpus_subnode_offset = fdt_first_subnode(fdt, cpus_offset);
    else
    cpus_subnode_offset = fdt_next_subnode(fdt, prev_node_offset);
    }
    cpus_node = of_find_node_by_path("/cpus");
// Fail here to avoid kexec/kdump kernel boot hung
    if (!cpus_node) {
    pr_err("No /cpus node found\n");
    return -EINVAL;
    }
// Add all /cpus sub-nodes of device_type == "cpu" to FDT
    for_each_child_of_node(cpus_node, dn) {
// Ignore device nodes that do not have a device_type property
// or device_type != "cpu".
//
    device_type = of_get_property(dn, "device_type", core::ptr::null_mut());
    if (!device_type || strcmp(device_type, "cpu"))
    continue;
    cpus_subnode_offset = fdt_add_subnode(fdt, cpus_offset, dn.full_name);
    if (cpus_subnode_offset < 0) {
    pr_err("Unable to add %s subnode: %s\n", dn.full_name,
    fdt_strerror(cpus_subnode_offset));
    ret = cpus_subnode_offset;
    goto out;
    }
    ret = add_node_props(fdt, cpus_subnode_offset, dn);
    if (ret < 0)
    goto out;
    }
    out:
    of_node_put(cpus_node);
    of_node_put(dn);
    return ret;
    }
