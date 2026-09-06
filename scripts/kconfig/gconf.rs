//! Automatically rewritten from C to Rust
//! Source: scripts/kconfig/gconf.c
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
// Copyright (C) 2002-2003 Romain Lievin <roms@tilp.info>
//

    enum view_mode {
    SINGLE_VIEW, SPLIT_VIEW, FULL_VIEW
    };
    enum {
    OPT_NORMAL, OPT_ALL, OPT_PROMPT
    };
    let mut view_mode: static gint = FULL_VIEW;
    let mut show_name: static gboolean = TRUE;
    let mut show_range: static gboolean = TRUE;
    let mut show_value: static gboolean = TRUE;
    let mut opt_mode: static int = OPT_NORMAL;
    static GtkWidget *main_wnd;
    static GtkWidget *tree1_w;	// left  frame
    static GtkWidget *tree2_w;	// right frame
    static GtkWidget *text_w;
    static GtkWidget *hpaned;
    static GtkWidget *vpaned;
    static GtkWidget *back_btn, *save_btn, *single_btn, *split_btn, *full_btn;
    static GtkWidget *save_menu_item;
    static GtkTextTag *tag1, *tag2;
    static GtkTreeStore *tree1, *tree2;
    static GdkPixbuf *pix_menu;
    static struct menu *browsed; // browsed menu for SINGLE/SPLIT view
    static struct menu *selected; // selected entry
    enum {
    COL_OPTION, COL_NAME, COL_NO, COL_MOD, COL_YES, COL_VALUE,
    COL_MENU, COL_COLOR, COL_EDIT, COL_PIXBUF,
    COL_PIXVIS, COL_BTNVIS, COL_BTNACT, COL_BTNINC, COL_BTNRAD,
    COL_NUMBER
    };
    static void display_tree(GtkTreeStore *store, struct menu *menu);
    static void recreate_tree(void);
#[no_mangle]
unsafe extern "C" fn conf_changed(dirty: bool) {
    static void conf_changed(bool dirty)
    {
    gtk_widget_set_sensitive(save_btn, dirty);
    gtk_widget_set_sensitive(save_menu_item, dirty);
    }
// Utility Functions
#[no_mangle]
unsafe extern "C" fn text_insert_msg(title: *const c_char, msg: *const c_char) {
    static void text_insert_msg(const char *title, const char *msg)
    {
    GtkTextBuffer *buffer;
    GtkTextIter start, end;
    buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(text_w));
    gtk_text_buffer_get_bounds(buffer, &start, &end);
    gtk_text_buffer_delete(buffer, &start, &end);
    gtk_text_view_set_left_margin(GTK_TEXT_VIEW(text_w), 15);
    gtk_text_buffer_get_end_iter(buffer, &end);
    gtk_text_buffer_insert_with_tags(buffer, &end, title, -1, tag1,
    core::ptr::null_mut());
    gtk_text_buffer_insert_at_cursor(buffer, "\n\n", 2);
    gtk_text_buffer_get_end_iter(buffer, &end);
    gtk_text_buffer_insert_with_tags(buffer, &end, msg, -1, tag2,
    core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn text_insert_help(menu: *mut menu) {
    static void text_insert_help(struct menu *menu)
    {
    let mut help: gstr = str_new();
    menu_get_ext_help(menu, &help);
    text_insert_msg(menu_get_prompt(menu), str_get(&help));
    str_free(&help);
    }
    static void _select_menu(GtkTreeView *view, GtkTreeModel *model,
    GtkTreeIter *parent, struct menu *match)
    {
    GtkTreeIter iter;
    gboolean valid;
    valid = gtk_tree_model_iter_children(model, &iter, parent);
    while (valid) {
    struct menu *menu;
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    if (menu == match) {
    GtkTreeSelection *selection;
    GtkTreePath *path;
//
// Expand parents to reflect the selection, and
// scroll down to it.
//
    path = gtk_tree_model_get_path(model, &iter);
    gtk_tree_view_expand_to_path(view, path);
    gtk_tree_view_scroll_to_cell(view, path, core::ptr::null_mut(), TRUE,
    0.5, 0.0);
    gtk_tree_path_free(path);
    selection = gtk_tree_view_get_selection(view);
    gtk_tree_selection_select_iter(selection, &iter);
    text_insert_help(menu);
    }
    _select_menu(view, model, &iter, match);
    valid = gtk_tree_model_iter_next(model, &iter);
    }
    }
#[no_mangle]
unsafe extern "C" fn select_menu(view: *mut GtkTreeView, match: *mut menu) {
    static void select_menu(GtkTreeView *view, struct menu *match)
    {
    _select_menu(view, gtk_tree_view_get_model(view), core::ptr::null_mut(), match);
    }
#[no_mangle]
unsafe extern "C" fn _update_row_visibility(view: *mut GtkTreeView) {
    static void _update_row_visibility(GtkTreeView *view)
    {
    GtkTreeModelFilter *filter = GTK_TREE_MODEL_FILTER(gtk_tree_view_get_model(view));
    gtk_tree_model_filter_refilter(filter);
    }
#[no_mangle]
unsafe extern "C" fn update_row_visibility() {
    static void update_row_visibility(void)
    {
    if (view_mode == SPLIT_VIEW)
    _update_row_visibility(GTK_TREE_VIEW(tree1_w));
    _update_row_visibility(GTK_TREE_VIEW(tree2_w));
    }
#[no_mangle]
unsafe extern "C" fn set_node(tree: *mut GtkTreeStore, node: *mut GtkTreeIter, menu: *mut menu) {
    static void set_node(GtkTreeStore *tree, GtkTreeIter *node, struct menu *menu)
    {
    struct symbol *sym = menu.sym;
    tristate val;
    gchar *option;
    const gchar *_no = "";
    const gchar *_mod = "";
    const gchar *_yes = "";
    const gchar *value = "";
    GdkRGBA color;
    let mut editable: gboolean = FALSE;
    let mut btnvis: gboolean = FALSE;
    option = g_strdup_printf("%s %s %s %s",
    menu.type == M_COMMENT ? "***" : "",
    menu_get_prompt(menu),
    menu.type == M_COMMENT ? "***" : "",
    sym && !sym_has_value(sym) ? "(NEW)" : "");
    gdk_rgba_parse(&color, menu_is_visible(menu) ? "Black" : "DarkGray");
    if (!sym)
    goto set;
    sym_calc_value(sym);
    if (menu.type == M_CHOICE) {	// parse children to get a final value
    struct symbol *def_sym = sym_calc_choice(menu);
    struct menu *def_menu = core::ptr::null_mut();
    for (struct menu *child = menu.list; child; child = child.next) {
    if (menu_is_visible(child) && child.sym == def_sym)
    def_menu = child;
    }
    if (def_menu)
    value = menu_get_prompt(def_menu);
    goto set;
    }
    switch (sym_get_type(sym)) {
    case S_BOOLEAN:
    case S_TRISTATE:
    btnvis = TRUE;
    val = sym_get_tristate_value(sym);
    switch (val) {
    case no:
    _no = "N";
    value = "N";
    break;
    case mod:
    _mod = "M";
    value = "M";
    break;
    case yes:
    _yes = "Y";
    value = "Y";
    break;
    }
    if (val != no && sym_tristate_within_range(sym, no))
    _no = "_";
    if (val != mod && sym_tristate_within_range(sym, mod))
    _mod = "_";
    if (val != yes && sym_tristate_within_range(sym, yes))
    _yes = "_";
    break;
    default:
    value = sym_get_string_value(sym);
    editable = TRUE;
    break;
    }
    set:
    gtk_tree_store_set(tree, node,
    COL_OPTION, option,
    COL_NAME, sym ? sym.name : "",
    COL_NO, _no,
    COL_MOD, _mod,
    COL_YES, _yes,
    COL_VALUE, value,
    COL_MENU, (gpointer) menu,
    COL_COLOR, &color,
    COL_EDIT, editable,
    COL_PIXBUF, pix_menu,
    COL_PIXVIS, view_mode == SINGLE_VIEW && menu.type == M_MENU,
    COL_BTNVIS, btnvis,
    COL_BTNACT, _yes[0] == 'Y',
    COL_BTNINC, _mod[0] == 'M',
    COL_BTNRAD, sym && sym_is_choice_value(sym),
    -1);
    g_free(option);
    }
#[no_mangle]
unsafe extern "C" fn _update_tree(store: *mut GtkTreeStore, parent: *mut GtkTreeIter) {
    static void _update_tree(GtkTreeStore *store, GtkTreeIter *parent)
    {
    GtkTreeModel *model = GTK_TREE_MODEL(store);
    GtkTreeIter iter;
    gboolean valid;
    valid = gtk_tree_model_iter_children(model, &iter, parent);
    while (valid) {
    struct menu *menu;
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    if (menu)
    set_node(store, &iter, menu);
    _update_tree(store, &iter);
    valid = gtk_tree_model_iter_next(model, &iter);
    }
    }
#[no_mangle]
unsafe extern "C" fn update_tree(store: *mut GtkTreeStore) {
    static void update_tree(GtkTreeStore *store)
    {
    _update_tree(store, core::ptr::null_mut());
    update_row_visibility();
    }
#[no_mangle]
unsafe extern "C" fn update_trees() {
    static void update_trees(void)
    {
    if (view_mode == SPLIT_VIEW)
    update_tree(tree1);
    update_tree(tree2);
    }
#[no_mangle]
unsafe extern "C" fn set_view_mode(mode: enum view_mode) {
    static void set_view_mode(enum view_mode mode)
    {
    view_mode = mode;
    if (mode == SPLIT_VIEW) { // two panes
    gint w;
    gtk_widget_show(tree1_w);
    gtk_window_get_default_size(GTK_WINDOW(main_wnd), &w, core::ptr::null_mut());
    gtk_paned_set_position(GTK_PANED(hpaned), w / 2);
    } else {
    gtk_widget_hide(tree1_w);
    gtk_paned_set_position(GTK_PANED(hpaned), 0);
    }
    gtk_widget_set_sensitive(single_btn, TRUE);
    gtk_widget_set_sensitive(split_btn, TRUE);
    gtk_widget_set_sensitive(full_btn, TRUE);
    switch (mode) {
    case SINGLE_VIEW:
    if (selected)
    browsed = menu_get_parent_menu(selected) ?: &rootmenu;
    else
    browsed = &rootmenu;
    recreate_tree();
    text_insert_msg("", "");
    select_menu(GTK_TREE_VIEW(tree2_w), selected);
    gtk_widget_set_sensitive(single_btn, FALSE);
    break;
    case SPLIT_VIEW:
    browsed = selected;
    while (browsed && !(browsed.flags & MENU_ROOT))
    browsed = browsed.parent;
    gtk_tree_store_clear(tree1);
    display_tree(tree1, &rootmenu);
    gtk_tree_view_expand_all(GTK_TREE_VIEW(tree1_w));
    gtk_tree_store_clear(tree2);
    if (browsed)
    display_tree(tree2, browsed);
    text_insert_msg("", "");
    select_menu(GTK_TREE_VIEW(tree1_w), browsed);
    select_menu(GTK_TREE_VIEW(tree2_w), selected);
    gtk_widget_set_sensitive(split_btn, FALSE);
    break;
    case FULL_VIEW:
    gtk_tree_store_clear(tree2);
    display_tree(tree2, &rootmenu);
    text_insert_msg("", "");
    select_menu(GTK_TREE_VIEW(tree2_w), selected);
    gtk_widget_set_sensitive(full_btn, FALSE);
    break;
    }
    gtk_widget_set_sensitive(back_btn,
    mode == SINGLE_VIEW && browsed != &rootmenu);
    }
// Menu & Toolbar Callbacks
#[no_mangle]
unsafe extern "C" fn on_load1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_load1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkWidget *dialog;
    GtkFileChooser *chooser;
    gint res;
    dialog = gtk_file_chooser_dialog_new("Load file...",
    GTK_WINDOW(user_data),
    GTK_FILE_CHOOSER_ACTION_OPEN,
    "_Cancel", GTK_RESPONSE_CANCEL,
    "_Open", GTK_RESPONSE_ACCEPT,
    core::ptr::null_mut());
    chooser = GTK_FILE_CHOOSER(dialog);
    gtk_file_chooser_set_filename(chooser, conf_get_configname());
    res = gtk_dialog_run(GTK_DIALOG(dialog));
    if (res == GTK_RESPONSE_ACCEPT) {
    char *filename;
    filename = gtk_file_chooser_get_filename(chooser);
    if (conf_read(filename))
    text_insert_msg("Error",
    "Unable to load configuration!");
    else
    update_trees();
    g_free(filename);
    }
    gtk_widget_destroy(GTK_WIDGET(dialog));
    }
#[no_mangle]
unsafe extern "C" fn on_save_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_save_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    if (conf_write(core::ptr::null_mut()))
    text_insert_msg("Error", "Unable to save configuration !");
    conf_write_autoconf(0);
    }
#[no_mangle]
unsafe extern "C" fn on_save_as1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_save_as1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkWidget *dialog;
    GtkFileChooser *chooser;
    gint res;
    dialog = gtk_file_chooser_dialog_new("Save file as...",
    GTK_WINDOW(user_data),
    GTK_FILE_CHOOSER_ACTION_SAVE,
    "_Cancel", GTK_RESPONSE_CANCEL,
    "_Save", GTK_RESPONSE_ACCEPT,
    core::ptr::null_mut());
    chooser = GTK_FILE_CHOOSER(dialog);
    gtk_file_chooser_set_filename(chooser, conf_get_configname());
    res = gtk_dialog_run(GTK_DIALOG(dialog));
    if (res == GTK_RESPONSE_ACCEPT) {
    char *filename;
    filename = gtk_file_chooser_get_filename(chooser);
    if (conf_write(filename))
    text_insert_msg("Error",
    "Unable to save configuration !");
    g_free(filename);
    }
    gtk_widget_destroy(dialog);
    }
#[no_mangle]
unsafe extern "C" fn on_show_name1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_show_name1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkTreeViewColumn *col;
    show_name = gtk_check_menu_item_get_active(GTK_CHECK_MENU_ITEM(menuitem));
    col = gtk_tree_view_get_column(GTK_TREE_VIEW(tree2_w), COL_NAME);
    if (col)
    gtk_tree_view_column_set_visible(col, show_name);
    }
#[no_mangle]
unsafe extern "C" fn on_show_range1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_show_range1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkTreeViewColumn *col;
    show_range = gtk_check_menu_item_get_active(GTK_CHECK_MENU_ITEM(menuitem));
    col = gtk_tree_view_get_column(GTK_TREE_VIEW(tree2_w), COL_NO);
    if (col)
    gtk_tree_view_column_set_visible(col, show_range);
    col = gtk_tree_view_get_column(GTK_TREE_VIEW(tree2_w), COL_MOD);
    if (col)
    gtk_tree_view_column_set_visible(col, show_range);
    col = gtk_tree_view_get_column(GTK_TREE_VIEW(tree2_w), COL_YES);
    if (col)
    gtk_tree_view_column_set_visible(col, show_range);
    }
#[no_mangle]
unsafe extern "C" fn on_show_data1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_show_data1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkTreeViewColumn *col;
    show_value = gtk_check_menu_item_get_active(GTK_CHECK_MENU_ITEM(menuitem));
    col = gtk_tree_view_get_column(GTK_TREE_VIEW(tree2_w), COL_VALUE);
    if (col)
    gtk_tree_view_column_set_visible(col, show_value);
    }
    static void on_set_option_mode1_activate(GtkMenuItem *menuitem,
    gpointer user_data)
    {
    opt_mode = OPT_NORMAL;
    update_row_visibility();
    }
    static void on_set_option_mode2_activate(GtkMenuItem *menuitem,
    gpointer user_data)
    {
    opt_mode = OPT_ALL;
    update_row_visibility();
    }
    static void on_set_option_mode3_activate(GtkMenuItem *menuitem,
    gpointer user_data)
    {
    opt_mode = OPT_PROMPT;
    update_row_visibility();
    }
#[no_mangle]
unsafe extern "C" fn on_introduction1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_introduction1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkWidget *dialog;
    const gchar *intro_text =
    "Welcome to gconfig, the GTK+ graphical configuration tool.\n"
    "For each option, a blank box indicates the feature is disabled, a\n"
    "check indicates it is enabled, and a dot indicates that it is to\n"
    "be compiled as a module.  Clicking on the box will cycle through the three states.\n"
    "\n"
    "If you do not see an option (e.g., a device driver) that you\n"
    "believe should be present, try turning on Show All Options\n"
    "under the Options menu.\n"
    "Although there is no cross reference yet to help you figure out\n"
    "what other options must be enabled to support the option you\n"
    "are interested in, you can still view the help of a grayed-out\n"
    "option.";
    dialog = gtk_message_dialog_new(GTK_WINDOW(main_wnd),
    GTK_DIALOG_DESTROY_WITH_PARENT,
    GTK_MESSAGE_INFO,
    GTK_BUTTONS_CLOSE, "%s", intro_text);
    gtk_dialog_run(GTK_DIALOG(dialog));
    gtk_widget_destroy(dialog);
    }
#[no_mangle]
unsafe extern "C" fn on_about1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_about1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkWidget *dialog;
    const gchar *about_text =
    "gconfig is copyright (c) 2002 Romain Lievin <roms@lpg.ticalc.org>.\n"
    "Based on the source code from Roman Zippel.\n";
    dialog = gtk_message_dialog_new(GTK_WINDOW(main_wnd),
    GTK_DIALOG_DESTROY_WITH_PARENT,
    GTK_MESSAGE_INFO,
    GTK_BUTTONS_CLOSE, "%s\nGTK version: %d.%d.%d",
    about_text,
    gtk_get_major_version(),
    gtk_get_minor_version(),
    gtk_get_micro_version());
    gtk_dialog_run(GTK_DIALOG(dialog));
    gtk_widget_destroy(dialog);
    }
#[no_mangle]
unsafe extern "C" fn on_license1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_license1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    GtkWidget *dialog;
    const gchar *license_text =
    "gconfig is released under the terms of the GNU GPL v2.\n"
    "For more information, please see the source code or\n"
    "visit http://www.fsf.org/licenses/licenses.html\n";
    dialog = gtk_message_dialog_new(GTK_WINDOW(main_wnd),
    GTK_DIALOG_DESTROY_WITH_PARENT,
    GTK_MESSAGE_INFO,
    GTK_BUTTONS_CLOSE, "%s", license_text);
    gtk_dialog_run(GTK_DIALOG(dialog));
    gtk_widget_destroy(dialog);
    }
// toolbar handlers
#[no_mangle]
unsafe extern "C" fn on_back_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_back_clicked(GtkButton *button, gpointer user_data)
    {
    browsed = menu_get_parent_menu(browsed) ?: &rootmenu;
    recreate_tree();
    if (browsed == &rootmenu)
    gtk_widget_set_sensitive(back_btn, FALSE);
    }
#[no_mangle]
unsafe extern "C" fn on_load_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_load_clicked(GtkButton *button, gpointer user_data)
    {
    on_load1_activate(core::ptr::null_mut(), user_data);
    }
#[no_mangle]
unsafe extern "C" fn on_save_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_save_clicked(GtkButton *button, gpointer user_data)
    {
    on_save_activate(core::ptr::null_mut(), user_data);
    }
#[no_mangle]
unsafe extern "C" fn on_single_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_single_clicked(GtkButton *button, gpointer user_data)
    {
    set_view_mode(SINGLE_VIEW);
    }
#[no_mangle]
unsafe extern "C" fn on_split_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_split_clicked(GtkButton *button, gpointer user_data)
    {
    set_view_mode(SPLIT_VIEW);
    }
#[no_mangle]
unsafe extern "C" fn on_full_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_full_clicked(GtkButton *button, gpointer user_data)
    {
    set_view_mode(FULL_VIEW);
    }
#[no_mangle]
unsafe extern "C" fn on_collapse_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_collapse_clicked(GtkButton *button, gpointer user_data)
    {
    gtk_tree_view_collapse_all(GTK_TREE_VIEW(tree2_w));
    }
#[no_mangle]
unsafe extern "C" fn on_expand_clicked(button: *mut GtkButton, user_data: gpointer) {
    static void on_expand_clicked(GtkButton *button, gpointer user_data)
    {
    gtk_tree_view_expand_all(GTK_TREE_VIEW(tree2_w));
    }
// Main Windows Callbacks
#[no_mangle]
unsafe extern "C" fn on_window1_destroy(widget: *mut GtkWidget, user_data: gpointer) {
    static void on_window1_destroy(GtkWidget *widget, gpointer user_data)
    {
    gtk_main_quit();
    }
    static gboolean on_window1_configure(GtkWidget *self,
    GdkEventConfigure *event,
    gpointer user_data)
    {
    gtk_paned_set_position(GTK_PANED(vpaned), 2 * event.height / 3);
    return FALSE;
    }
    static gboolean on_window1_delete_event(GtkWidget *widget, GdkEvent *event,
    gpointer user_data)
    {
    GtkWidget *dialog, *label, *content_area;
    gint result;
    let mut ret: gint = FALSE;
    if (!conf_get_changed())
    return FALSE;
    dialog = gtk_dialog_new_with_buttons("Warning !",
    GTK_WINDOW(main_wnd),
    (GtkDialogFlags)
    (GTK_DIALOG_MODAL |
    GTK_DIALOG_DESTROY_WITH_PARENT),
    "_OK",
    GTK_RESPONSE_YES,
    "_No",
    GTK_RESPONSE_NO,
    "_Cancel",
    GTK_RESPONSE_CANCEL, core::ptr::null_mut());
    gtk_dialog_set_default_response(GTK_DIALOG(dialog),
    GTK_RESPONSE_CANCEL);
    label = gtk_label_new("\nSave configuration ?\n");
    content_area = gtk_dialog_get_content_area(GTK_DIALOG(dialog));
    gtk_container_add(GTK_CONTAINER(content_area), label);
    gtk_widget_show(label);
    result = gtk_dialog_run(GTK_DIALOG(dialog));
    switch (result) {
    case GTK_RESPONSE_YES:
    on_save_activate(core::ptr::null_mut(), core::ptr::null_mut());
    break;
    case GTK_RESPONSE_NO:
    break;
    case GTK_RESPONSE_CANCEL:
    case GTK_RESPONSE_DELETE_EVENT:
    default:
    ret = TRUE;
    break;
    }
    gtk_widget_destroy(dialog);
    if (!ret)
    g_object_unref(pix_menu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn on_quit1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer) {
    static void on_quit1_activate(GtkMenuItem *menuitem, gpointer user_data)
    {
    if (!on_window1_delete_event(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()))
    gtk_widget_destroy(GTK_WIDGET(main_wnd));
    }
// CTree Callbacks
// Change hex/int/string value in the cell
    static void renderer_edited(GtkCellRendererText * cell,
    const gchar * path_string,
    const gchar * new_text, gpointer user_data)
    {
    GtkTreeView *view = GTK_TREE_VIEW(user_data);
    GtkTreeModel *model = gtk_tree_view_get_model(view);
    GtkTreePath *path = gtk_tree_path_new_from_string(path_string);
    GtkTreeIter iter;
    const char *old_def, *new_def;
    struct menu *menu;
    struct symbol *sym;
    if (!gtk_tree_model_get_iter(model, &iter, path))
    goto free;
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    sym = menu.sym;
    gtk_tree_model_get(model, &iter, COL_VALUE, &old_def, -1);
    new_def = new_text;
    sym_set_string_value(sym, new_def);
    update_trees();
    free:
    gtk_tree_path_free(path);
    }
// Change the value of a symbol and update the tree
#[no_mangle]
unsafe extern "C" fn change_sym_value(menu: *mut menu, col: gint) {
    static void change_sym_value(struct menu *menu, gint col)
    {
    struct symbol *sym = menu.sym;
    tristate newval;
    if (!sym)
    return;
    if (col == COL_NO)
    newval = no;
#[no_mangle]
pub unsafe extern "C" fn if(COL_MOD: col ==) -> else {
    else if (col == COL_MOD)
    newval = mod;
#[no_mangle]
pub unsafe extern "C" fn if(COL_YES: col ==) -> else {
    else if (col == COL_YES)
    newval = yes;
    else
    return;
    switch (sym_get_type(sym)) {
    case S_BOOLEAN:
    case S_TRISTATE:
    if (!sym_tristate_within_range(sym, newval))
    newval = yes;
    sym_set_tristate_value(sym, newval);
    update_trees();
    break;
    case S_INT:
    case S_HEX:
    case S_STRING:
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn toggle_sym_value(menu: *mut menu) {
    static void toggle_sym_value(struct menu *menu)
    {
    if (!menu.sym)
    return;
    sym_toggle_tristate_value(menu.sym);
    update_trees();
    }
#[no_mangle]
unsafe extern "C" fn column2index(column: *mut *mut GtkTreeViewColumn) -> gint {
    static gint column2index(GtkTreeViewColumn * column)
    {
    gint i;
    for (i = 0; i < COL_NUMBER; i++) {
    GtkTreeViewColumn *col;
    col = gtk_tree_view_get_column(GTK_TREE_VIEW(tree2_w), i);
    if (col == column)
    return i;
    }
    return -1;
    }
// User click: update choice (full) or goes down (single)
    static gboolean on_treeview2_button_press_event(GtkWidget *widget,
    GdkEventButton *event,
    gpointer user_data)
    {
    GtkTreeView *view = GTK_TREE_VIEW(widget);
    GtkTreeModel *model = gtk_tree_view_get_model(view);
    GtkTreePath *path;
    GtkTreeViewColumn *column;
    GtkTreeIter iter;
    struct menu *menu;
    gint col;
    let mut tx: gint = (gint) event.x;
    let mut ty: gint = (gint) event.y;
    gtk_tree_view_get_path_at_pos(view, tx, ty, &path, &column, core::ptr::null_mut(), core::ptr::null_mut());
    if (path == core::ptr::null_mut())
    return FALSE;
    if (!gtk_tree_model_get_iter(model, &iter, path))
    return FALSE;
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    selected = menu;
    col = column2index(column);
    if (event.type == GDK_2BUTTON_PRESS) {
    enum prop_type ptype;
    ptype = menu.prompt ? menu.prompt.type : P_UNKNOWN;
    if (ptype == P_MENU && view_mode == SINGLE_VIEW && col == COL_OPTION) {
// goes down into menu
    browsed = menu;
    recreate_tree();
    gtk_widget_set_sensitive(back_btn, TRUE);
    } else if (col == COL_OPTION) {
    toggle_sym_value(menu);
    gtk_tree_view_expand_row(view, path, TRUE);
    }
    } else {
    if (col == COL_VALUE) {
    toggle_sym_value(menu);
    gtk_tree_view_expand_row(view, path, TRUE);
    } else if (col == COL_NO || col == COL_MOD
    || col == COL_YES) {
    change_sym_value(menu, col);
    gtk_tree_view_expand_row(view, path, TRUE);
    }
    }
    return FALSE;
    }
// Key pressed: update choice
    static gboolean on_treeview2_key_press_event(GtkWidget *widget,
    GdkEventKey *event,
    gpointer user_data)
    {
    GtkTreeView *view = GTK_TREE_VIEW(widget);
    GtkTreeModel *model = gtk_tree_view_get_model(view);
    GtkTreePath *path;
    GtkTreeIter iter;
    struct menu *menu;
    gint col;
    gtk_tree_view_get_cursor(view, &path, core::ptr::null_mut());
    if (path == core::ptr::null_mut())
    return FALSE;
    if (event.keyval == GDK_KEY_space) {
    if (gtk_tree_view_row_expanded(view, path))
    gtk_tree_view_collapse_row(view, path);
    else
    gtk_tree_view_expand_row(view, path, FALSE);
    return TRUE;
    }
    gtk_tree_model_get_iter(model, &iter, path);
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    if (!strcasecmp(event.string, "n"))
    col = COL_NO;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(event->string, _arg: "m")) -> else {
    else if (!strcasecmp(event.string, "m"))
    col = COL_MOD;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(event->string, _arg: "y")) -> else {
    else if (!strcasecmp(event.string, "y"))
    col = COL_YES;
    else
    col = -1;
    change_sym_value(menu, col);
    return FALSE;
    }
// Row selection changed: update help
    static void on_treeview2_cursor_changed(GtkTreeView *treeview,
    gpointer user_data)
    {
    GtkTreeModel *model = gtk_tree_view_get_model(treeview);
    GtkTreeSelection *selection;
    GtkTreeIter iter;
    struct menu *menu;
    selection = gtk_tree_view_get_selection(treeview);
    if (gtk_tree_selection_get_selected(selection, &model, &iter)) {
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    text_insert_help(menu);
    }
    }
// User click: display sub-tree in the right frame.
    static gboolean on_treeview1_button_press_event(GtkWidget *widget,
    GdkEventButton *event,
    gpointer user_data)
    {
    GtkTreeView *view = GTK_TREE_VIEW(widget);
    GtkTreeModel *model = gtk_tree_view_get_model(view);
    GtkTreePath *path;
    GtkTreeIter iter;
    struct menu *menu;
    let mut tx: gint = (gint) event.x;
    let mut ty: gint = (gint) event.y;
    gtk_tree_view_get_path_at_pos(view, tx, ty, &path, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (path == core::ptr::null_mut())
    return FALSE;
    gtk_tree_model_get_iter(model, &iter, path);
    gtk_tree_model_get(model, &iter, COL_MENU, &menu, -1);
    if (event.type == GDK_2BUTTON_PRESS)
    toggle_sym_value(menu);
    selected = menu;
    if (menu.type == M_MENU) {
    browsed = menu;
    recreate_tree();
    }
    gtk_tree_view_set_cursor(view, path, core::ptr::null_mut(), FALSE);
    gtk_widget_grab_focus(tree2_w);
    return FALSE;
    }
// Display the whole tree (single/split/full view)
    static void _display_tree(GtkTreeStore *tree, struct menu *menu,
    GtkTreeIter *parent)
    {
    struct menu *child;
    GtkTreeIter iter;
    for (child = menu.list; child; child = child.next) {
//
// REVISIT:
// menu_finalize() creates empty "if" entries.
// Do not confuse gtk_tree_model_get(), which would otherwise
// return "if" menu entry.
//
    if (child.type == M_IF)
    continue;
    if ((view_mode == SPLIT_VIEW)
    && !(child.flags & MENU_ROOT) && (tree == tree1))
    continue;
    if ((view_mode == SPLIT_VIEW) && (child.flags & MENU_ROOT)
    && (tree == tree2))
    continue;
    gtk_tree_store_append(tree, &iter, parent);
    set_node(tree, &iter, child);
    if (view_mode != SINGLE_VIEW || child.type != M_MENU)
    _display_tree(tree, child, &iter);
    }
    }
#[no_mangle]
unsafe extern "C" fn display_tree(store: *mut GtkTreeStore, menu: *mut menu) {
    static void display_tree(GtkTreeStore *store, struct menu *menu)
    {
    _display_tree(store, menu, core::ptr::null_mut());
    }
// Recreate the tree store starting at 'browsed' node
#[no_mangle]
unsafe extern "C" fn recreate_tree() {
    static void recreate_tree(void)
    {
    gtk_tree_store_clear(tree2);
    display_tree(tree2, browsed);
    gtk_tree_view_expand_all(GTK_TREE_VIEW(tree2_w));
    }
#[no_mangle]
unsafe extern "C" fn fixup_rootmenu(menu: *mut menu) {
    static void fixup_rootmenu(struct menu *menu)
    {
    struct menu *child;
    let mut menu_cnt: static int = 0;
    menu.flags |= MENU_ROOT;
    for (child = menu.list; child; child = child.next) {
    if (child.prompt && child.prompt.type == P_MENU) {
    menu_cnt++;
    fixup_rootmenu(child);
    menu_cnt--;
    } else if (!menu_cnt)
    fixup_rootmenu(child);
    }
    }
// Main Window Initialization
#[no_mangle]
unsafe extern "C" fn replace_button_icon(widget: *mut GtkWidget, filename: *const c_char) {
    static void replace_button_icon(GtkWidget *widget, const char *filename)
    {
    GdkPixbuf *pixbuf;
    GtkWidget *image;
    GError *err = core::ptr::null_mut();
    char *env = getenv(SRCTREE);
    gchar *path = g_strconcat(env ? env : g_get_current_dir(), "/scripts/kconfig/icons/", filename, core::ptr::null_mut());
    pixbuf = gdk_pixbuf_new_from_file(path, &err);
    g_free(path);
    if (err) {
    g_warning("Failed to load icon %s: %s", filename, err.message);
    g_error_free(err);
    return;
    }
    image = gtk_image_new_from_pixbuf(pixbuf);
    g_object_unref(pixbuf);
    gtk_widget_show(image);
    gtk_tool_button_set_icon_widget(GTK_TOOL_BUTTON(widget), image);
    }
#[no_mangle]
unsafe extern "C" fn init_main_window(glade_file: *const gchar) {
    static void init_main_window(const gchar *glade_file)
    {
    GtkBuilder *builder;
    GtkWidget *widget;
    GtkTextBuffer *txtbuf;
    builder = gtk_builder_new_from_file(glade_file);
    if (!builder)
    g_error("GUI loading failed !\n");
    main_wnd = GTK_WIDGET(gtk_builder_get_object(builder, "window1"));
    g_signal_connect(main_wnd, "destroy",
    G_CALLBACK(on_window1_destroy), core::ptr::null_mut());
    g_signal_connect(main_wnd, "configure-event",
    G_CALLBACK(on_window1_configure), core::ptr::null_mut());
    g_signal_connect(main_wnd, "delete-event",
    G_CALLBACK(on_window1_delete_event), core::ptr::null_mut());
    hpaned = GTK_WIDGET(gtk_builder_get_object(builder, "hpaned1"));
    vpaned = GTK_WIDGET(gtk_builder_get_object(builder, "vpaned1"));
    tree1_w = GTK_WIDGET(gtk_builder_get_object(builder, "treeview1"));
    g_signal_connect(tree1_w, "cursor-changed",
    G_CALLBACK(on_treeview2_cursor_changed), core::ptr::null_mut());
    g_signal_connect(tree1_w, "button-press-event",
    G_CALLBACK(on_treeview1_button_press_event), core::ptr::null_mut());
    g_signal_connect(tree1_w, "key-press-event",
    G_CALLBACK(on_treeview2_key_press_event), core::ptr::null_mut());
    tree2_w = GTK_WIDGET(gtk_builder_get_object(builder, "treeview2"));
    g_signal_connect(tree2_w, "cursor-changed",
    G_CALLBACK(on_treeview2_cursor_changed), core::ptr::null_mut());
    g_signal_connect(tree2_w, "button-press-event",
    G_CALLBACK(on_treeview2_button_press_event), core::ptr::null_mut());
    g_signal_connect(tree2_w, "key-press-event",
    G_CALLBACK(on_treeview2_key_press_event), core::ptr::null_mut());
    text_w = GTK_WIDGET(gtk_builder_get_object(builder, "textview3"));
// menubar
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "load1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_load1_activate), core::ptr::null_mut());
    save_menu_item = GTK_WIDGET(gtk_builder_get_object(builder, "save1"));
    g_signal_connect(save_menu_item, "activate",
    G_CALLBACK(on_save_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "save_as1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_save_as1_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "quit1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_quit1_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "show_name1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_show_name1_activate), core::ptr::null_mut());
    gtk_check_menu_item_set_active((GtkCheckMenuItem *) widget,
    show_name);
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "show_range1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_show_range1_activate), core::ptr::null_mut());
    gtk_check_menu_item_set_active((GtkCheckMenuItem *) widget,
    show_range);
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "show_data1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_show_data1_activate), core::ptr::null_mut());
    gtk_check_menu_item_set_active((GtkCheckMenuItem *) widget,
    show_value);
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "set_option_mode1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_set_option_mode1_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "set_option_mode2"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_set_option_mode2_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "set_option_mode3"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_set_option_mode3_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "introduction1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_introduction1_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "about1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_about1_activate), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "license1"));
    g_signal_connect(widget, "activate",
    G_CALLBACK(on_license1_activate), core::ptr::null_mut());
// toolbar
    back_btn = GTK_WIDGET(gtk_builder_get_object(builder, "button1"));
    g_signal_connect(back_btn, "clicked",
    G_CALLBACK(on_back_clicked), core::ptr::null_mut());
    gtk_widget_set_sensitive(back_btn, FALSE);
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "button2"));
    g_signal_connect(widget, "clicked",
    G_CALLBACK(on_load_clicked), core::ptr::null_mut());
    save_btn = GTK_WIDGET(gtk_builder_get_object(builder, "button3"));
    g_signal_connect(save_btn, "clicked",
    G_CALLBACK(on_save_clicked), core::ptr::null_mut());
    single_btn = GTK_WIDGET(gtk_builder_get_object(builder, "button4"));
    g_signal_connect(single_btn, "clicked",
    G_CALLBACK(on_single_clicked), core::ptr::null_mut());
    replace_button_icon(single_btn, "single_view.xpm");
    split_btn = GTK_WIDGET(gtk_builder_get_object(builder, "button5"));
    g_signal_connect(split_btn, "clicked",
    G_CALLBACK(on_split_clicked), core::ptr::null_mut());
    replace_button_icon(split_btn, "split_view.xpm");
    full_btn = GTK_WIDGET(gtk_builder_get_object(builder, "button6"));
    g_signal_connect(full_btn, "clicked",
    G_CALLBACK(on_full_clicked), core::ptr::null_mut());
    replace_button_icon(full_btn, "tree_view.xpm");
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "button7"));
    g_signal_connect(widget, "clicked",
    G_CALLBACK(on_collapse_clicked), core::ptr::null_mut());
    widget = GTK_WIDGET(gtk_builder_get_object(builder, "button8"));
    g_signal_connect(widget, "clicked",
    G_CALLBACK(on_expand_clicked), core::ptr::null_mut());
    txtbuf = gtk_text_view_get_buffer(GTK_TEXT_VIEW(text_w));
    tag1 = gtk_text_buffer_create_tag(txtbuf, "mytag1",
    "foreground", "red",
    "weight", PANGO_WEIGHT_BOLD,
    core::ptr::null_mut());
    tag2 = gtk_text_buffer_create_tag(txtbuf, "mytag2",
// "style", PANGO_STYLE_OBLIQUE,
    core::ptr::null_mut());
    gtk_window_set_title(GTK_WINDOW(main_wnd), rootmenu.prompt.text);
    gtk_widget_show_all(main_wnd);
    g_object_unref(builder);
    conf_set_changed_callback(conf_changed);
    }
    static gboolean visible_func(GtkTreeModel *model, GtkTreeIter  *iter,
    gpointer data)
    {
    struct menu *menu;
    gtk_tree_model_get(model, iter, COL_MENU, &menu, -1);
    if (!menu)
    return FALSE;
    return menu_is_visible(menu) || opt_mode == OPT_ALL ||
    (opt_mode == OPT_PROMPT && menu_has_prompt(menu));
    }
#[no_mangle]
unsafe extern "C" fn init_left_tree() {
    static void init_left_tree(void)
    {
    GtkTreeView *view = GTK_TREE_VIEW(tree1_w);
    GtkCellRenderer *renderer;
    GtkTreeSelection *sel;
    GtkTreeViewColumn *column;
    GtkTreeModel *filter;
    tree1 = gtk_tree_store_new(COL_NUMBER,
    G_TYPE_STRING, G_TYPE_STRING,
    G_TYPE_STRING, G_TYPE_STRING,
    G_TYPE_STRING, G_TYPE_STRING,
    G_TYPE_POINTER, GDK_TYPE_RGBA,
    G_TYPE_BOOLEAN, GDK_TYPE_PIXBUF,
    G_TYPE_BOOLEAN, G_TYPE_BOOLEAN,
    G_TYPE_BOOLEAN, G_TYPE_BOOLEAN,
    G_TYPE_BOOLEAN);
    filter = gtk_tree_model_filter_new(GTK_TREE_MODEL(tree1), core::ptr::null_mut());
    gtk_tree_model_filter_set_visible_func(GTK_TREE_MODEL_FILTER(filter),
    visible_func, core::ptr::null_mut(), core::ptr::null_mut());
    gtk_tree_view_set_model(view, filter);
    column = gtk_tree_view_column_new();
    gtk_tree_view_append_column(view, column);
    gtk_tree_view_column_set_title(column, "Options");
    renderer = gtk_cell_renderer_toggle_new();
    gtk_tree_view_column_pack_start(GTK_TREE_VIEW_COLUMN(column),
    renderer, FALSE);
    gtk_tree_view_column_set_attributes(GTK_TREE_VIEW_COLUMN(column),
    renderer,
    "active", COL_BTNACT,
    "inconsistent", COL_BTNINC,
    "visible", COL_BTNVIS,
    "radio", COL_BTNRAD, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_column_pack_start(GTK_TREE_VIEW_COLUMN(column),
    renderer, FALSE);
    gtk_tree_view_column_set_attributes(GTK_TREE_VIEW_COLUMN(column),
    renderer,
    "text", COL_OPTION,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    sel = gtk_tree_view_get_selection(view);
    gtk_tree_selection_set_mode(sel, GTK_SELECTION_SINGLE);
    }
#[no_mangle]
unsafe extern "C" fn init_right_tree() {
    static void init_right_tree(void)
    {
    GtkTreeView *view = GTK_TREE_VIEW(tree2_w);
    GtkCellRenderer *renderer;
    GtkTreeSelection *sel;
    GtkTreeViewColumn *column;
    GtkTreeModel *filter;
    gint i;
    tree2 = gtk_tree_store_new(COL_NUMBER,
    G_TYPE_STRING, G_TYPE_STRING,
    G_TYPE_STRING, G_TYPE_STRING,
    G_TYPE_STRING, G_TYPE_STRING,
    G_TYPE_POINTER, GDK_TYPE_RGBA,
    G_TYPE_BOOLEAN, GDK_TYPE_PIXBUF,
    G_TYPE_BOOLEAN, G_TYPE_BOOLEAN,
    G_TYPE_BOOLEAN, G_TYPE_BOOLEAN,
    G_TYPE_BOOLEAN);
    filter = gtk_tree_model_filter_new(GTK_TREE_MODEL(tree2), core::ptr::null_mut());
    gtk_tree_model_filter_set_visible_func(GTK_TREE_MODEL_FILTER(filter),
    visible_func, core::ptr::null_mut(), core::ptr::null_mut());
    gtk_tree_view_set_model(view, filter);
    column = gtk_tree_view_column_new();
    gtk_tree_view_append_column(view, column);
    gtk_tree_view_column_set_title(column, "Options");
    renderer = gtk_cell_renderer_pixbuf_new();
    gtk_tree_view_column_pack_start(GTK_TREE_VIEW_COLUMN(column),
    renderer, FALSE);
    gtk_tree_view_column_set_attributes(GTK_TREE_VIEW_COLUMN(column),
    renderer,
    "pixbuf", COL_PIXBUF,
    "visible", COL_PIXVIS, core::ptr::null_mut());
    renderer = gtk_cell_renderer_toggle_new();
    gtk_tree_view_column_pack_start(GTK_TREE_VIEW_COLUMN(column),
    renderer, FALSE);
    gtk_tree_view_column_set_attributes(GTK_TREE_VIEW_COLUMN(column),
    renderer,
    "active", COL_BTNACT,
    "inconsistent", COL_BTNINC,
    "visible", COL_BTNVIS,
    "radio", COL_BTNRAD, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_column_pack_start(GTK_TREE_VIEW_COLUMN(column),
    renderer, FALSE);
    gtk_tree_view_column_set_attributes(GTK_TREE_VIEW_COLUMN(column),
    renderer,
    "text", COL_OPTION,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_insert_column_with_attributes(view, -1,
    "Name", renderer,
    "text", COL_NAME,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_insert_column_with_attributes(view, -1,
    "N", renderer,
    "text", COL_NO,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_insert_column_with_attributes(view, -1,
    "M", renderer,
    "text", COL_MOD,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_insert_column_with_attributes(view, -1,
    "Y", renderer,
    "text", COL_YES,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    renderer = gtk_cell_renderer_text_new();
    gtk_tree_view_insert_column_with_attributes(view, -1,
    "Value", renderer,
    "text", COL_VALUE,
    "editable",
    COL_EDIT,
    "foreground-rgba",
    COL_COLOR, core::ptr::null_mut());
    g_signal_connect(G_OBJECT(renderer), "edited",
    G_CALLBACK(renderer_edited), tree2_w);
    char *env = getenv(SRCTREE);
    gchar *path = g_strconcat(env ? env : g_get_current_dir(), "/scripts/kconfig/icons/menu.xpm", core::ptr::null_mut());
    GError *err = core::ptr::null_mut();
    pix_menu = gdk_pixbuf_new_from_file(path, &err);
    g_free(path);
    if (err) {
    g_warning("Failed to load menu icon: %s", err.message);
    g_error_free(err);
    }
    for (i = 0; i < COL_VALUE; i++) {
    column = gtk_tree_view_get_column(view, i);
    gtk_tree_view_column_set_resizable(column, TRUE);
    }
    sel = gtk_tree_view_get_selection(view);
    gtk_tree_selection_set_mode(sel, GTK_SELECTION_SINGLE);
    }
// Main
#[no_mangle]
pub unsafe extern "C" fn main(ac: c_int, av[]: *mut c_char) -> c_int {
    int main(int ac, char *av[])
    {
    const char *name;
    char *env;
    gchar *glade_file;
// GTK stuffs
    gtk_init(&ac, &av);
// Determine GUI path
    env = getenv(SRCTREE);
    if (env)
    glade_file = g_strconcat(env, "/scripts/kconfig/gconf.ui", core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn if('/': av[0][0] ==) -> else {
    else if (av[0][0] == '/')
    glade_file = g_strconcat(av[0], ".ui", core::ptr::null_mut());
    else
    glade_file = g_strconcat(g_get_current_dir(), "/", av[0], ".ui", core::ptr::null_mut());
// Conf stuffs
    if (ac > 1 && av[1][0] == '-') {
    switch (av[1][1]) {
    case 'a':
// showAll = 1;
    break;
    case 's':
    conf_set_message_callback(core::ptr::null_mut());
    break;
    case 'h':
    case '?':
    printf("%s [-s] <config>\n", av[0]);
    exit(0);
    }
    name = av[2];
    } else
    name = av[1];
    conf_parse(name);
    fixup_rootmenu(&rootmenu);
// Load the interface and connect signals
    init_main_window(glade_file);
    init_left_tree();
    init_right_tree();
    conf_read(core::ptr::null_mut());
    set_view_mode(view_mode);
    gtk_main();
    return 0;
    }
