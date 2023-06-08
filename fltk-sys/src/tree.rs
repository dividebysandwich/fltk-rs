/* automatically generated by rust-bindgen 0.58.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut Fl_Widget,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type custom_draw_callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tree {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tree_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tree;
}
extern "C" {
    pub fn Fl_Tree_x(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_y(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_width(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_height(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_label(arg1: *mut Fl_Tree) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tree_set_label(arg1: *mut Fl_Tree, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tree_redraw(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_show(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_hide(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_activate(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_deactivate(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_redraw_label(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_resize(
        arg1: *mut Fl_Tree,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_widget_resize(
        arg1: *mut Fl_Tree,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_tooltip(arg1: *mut Fl_Tree) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tree_set_tooltip(arg1: *mut Fl_Tree, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tree_get_type(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_type(arg1: *mut Fl_Tree, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_color(arg1: *mut Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_color(arg1: *mut Fl_Tree, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_measure_label(
        arg1: *const Fl_Tree,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_label_color(arg1: *mut Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_label_color(arg1: *mut Fl_Tree, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_label_font(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_label_font(arg1: *mut Fl_Tree, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_label_size(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_label_size(arg1: *mut Fl_Tree, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_label_type(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_label_type(arg1: *mut Fl_Tree, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_box(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_box(arg1: *mut Fl_Tree, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_changed(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_changed(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_clear_changed(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_align(arg1: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_align(arg1: *mut Fl_Tree, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_delete(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_set_image(arg1: *mut Fl_Tree, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_handle(
        self_: *mut Fl_Tree,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tree_handle_event(
        self_: *mut Fl_Tree,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_draw(
        self_: *mut Fl_Tree,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tree_resize_callback(
        self_: *mut Fl_Tree,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut Fl_Widget,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                w: ::std::os::raw::c_int,
                h: ::std::os::raw::c_int,
                arg2: *mut ::std::os::raw::c_void,
            ),
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tree_set_when(arg1: *mut Fl_Tree, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_when(arg1: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_image(arg1: *const Fl_Tree) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_parent(self_: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_selection_color(arg1: *mut Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_selection_color(arg1: *mut Fl_Tree, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_do_callback(arg1: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_inside(
        self_: *const Fl_Tree,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_window(arg1: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_top_window(arg1: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_takes_events(arg1: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_user_data(arg1: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_take_focus(self_: *mut Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_visible_focus(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_clear_visible_focus(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_visible_focus(self_: *mut Fl_Tree, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_has_visible_focus(self_: *mut Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_user_data(arg1: *mut Fl_Tree, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_draw_data(self_: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_handle_data(self_: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_set_draw_data(self_: *mut Fl_Tree, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_set_handle_data(self_: *mut Fl_Tree, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_damage(self_: *const Fl_Tree) -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn Fl_Tree_set_damage(self_: *mut Fl_Tree, flag: ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn Fl_Tree_set_damage_area(
        self_: *mut Fl_Tree,
        flag: ::std::os::raw::c_uchar,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_clear_damage(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_as_window(self_: *mut Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_as_group(self_: *mut Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_set_deimage(arg1: *mut Fl_Tree, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_deimage(arg1: *const Fl_Tree) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_set_callback(
        arg1: *mut Fl_Tree,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tree_set_deleter(
        arg1: *mut Fl_Tree,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn Fl_Tree_visible(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_visible_r(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_active(self_: *const Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_active_r(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_callback(self_: *const Fl_Tree) -> Fl_Callback;
}
extern "C" {
    pub fn Fl_Tree_set_deletion_callback(
        self_: *mut Fl_Tree,
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tree_from_dyn_ptr(ptr: *mut Fl_Widget) -> *mut Fl_Tree;
}
extern "C" {
    pub fn Fl_Tree_super_draw(ptr: *mut Fl_Widget, flag: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tree_Item {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tree_Item_Array {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tree_begin(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_end(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_show_self(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_root_label(self_: *mut Fl_Tree, new_label: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tree_root(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_set_root(self_: *mut Fl_Tree, newitem: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_add(
        self_: *mut Fl_Tree,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_add_item(
        self_: *mut Fl_Tree,
        name: *const ::std::os::raw::c_char,
        item: *mut Fl_Tree_Item,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_insert_above(
        self_: *mut Fl_Tree,
        above: *mut Fl_Tree_Item,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_insert(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        name: *const ::std::os::raw::c_char,
        pos: ::std::os::raw::c_int,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_find_item(
        self_: *const Fl_Tree,
        path: *const ::std::os::raw::c_char,
    ) -> *const Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_find_item_mut(
        self_: *mut Fl_Tree,
        path: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_remove(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_clear(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_clear_children(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_find_clicked(
        self_: *const Fl_Tree,
        yonly: ::std::os::raw::c_int,
    ) -> *const Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_item_clicked(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_first(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_first_visible_item(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_next(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_prev(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_last(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_last_visible_item(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_next_visible_item(
        self_: *mut Fl_Tree,
        start: *mut Fl_Tree_Item,
        dir: ::std::os::raw::c_int,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_first_selected_item(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_last_selected_item(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_next_item(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        dir: ::std::os::raw::c_int,
        visible: ::std::os::raw::c_int,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_next_selected_item(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        dir: ::std::os::raw::c_int,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_get_selected_items(
        self_: *mut Fl_Tree,
        arr: *mut *mut Fl_Tree_Item_Array,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_get_items(
        self_: *mut Fl_Tree,
        arr: *mut *mut Fl_Tree_Item_Array,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_open(
        self_: *mut Fl_Tree,
        path: *const ::std::os::raw::c_char,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_open_toggle(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        docallback: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_close(
        self_: *mut Fl_Tree,
        path: *const ::std::os::raw::c_char,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_is_open(
        self_: *const Fl_Tree,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_is_close(
        self_: *const Fl_Tree,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_select(
        self_: *mut Fl_Tree,
        path: *const ::std::os::raw::c_char,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_select_toggle(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        docallback: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_deselect(
        self_: *mut Fl_Tree,
        path: *const ::std::os::raw::c_char,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_deselect_all(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_select_only(
        self_: *mut Fl_Tree,
        selitem: *mut Fl_Tree_Item,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_select_all(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        docallback: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_extend_selection_dir(
        self_: *mut Fl_Tree,
        from: *mut Fl_Tree_Item,
        to: *mut Fl_Tree_Item,
        dir: ::std::os::raw::c_int,
        val: ::std::os::raw::c_int,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_extend_selection(
        self_: *mut Fl_Tree,
        from: *mut Fl_Tree_Item,
        to: *mut Fl_Tree_Item,
        val: ::std::os::raw::c_int,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_item_focus(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_get_item_focus(self_: *const Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_is_selected(
        self_: *mut Fl_Tree,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_item_labelfont(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_item_labelfont(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_item_labelsize(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_item_labelsize(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_item_labelfgcolor(self_: *const Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_item_labelfgcolor(self_: *mut Fl_Tree, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_item_labelbgcolor(self_: *const Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_item_labelbgcolor(self_: *mut Fl_Tree, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_connectorcolor(self_: *const Fl_Tree) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_set_connectorcolor(self_: *mut Fl_Tree, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_marginleft(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_marginleft(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_margintop(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_margintop(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_marginbottom(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_marginbottom(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_linespacing(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_linespacing(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_openchild_marginbottom(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_openchild_marginbottom(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_usericonmarginleft(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_usericonmarginleft(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_labelmarginleft(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_labelmarginleft(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_widgetmarginleft(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_widgetmarginleft(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_connectorwidth(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_connectorwidth(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_usericon(self_: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_set_usericon(self_: *mut Fl_Tree, val: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_openicon(self_: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_set_openicon(self_: *mut Fl_Tree, val: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_closeicon(self_: *const Fl_Tree) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_set_closeicon(self_: *mut Fl_Tree, val: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_showcollapse(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_showcollapse(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_showroot(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_showroot(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_connectorstyle(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_connectorstyle(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_sortorder(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_sortorder(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_selectbox(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_selectbox(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_selectmode(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_selectmode(self_: *mut Fl_Tree, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_item_reselect_mode(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_item_reselect_mode(self_: *mut Fl_Tree, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_item_draw_mode(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_item_draw_mode(self_: *mut Fl_Tree, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_calc_dimensions(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_calc_tree(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_recalc_tree(self_: *mut Fl_Tree);
}
extern "C" {
    pub fn Fl_Tree_displayed(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_show_item(
        self_: *mut Fl_Tree,
        item: *mut Fl_Tree_Item,
        yoff: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_show_item_top(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_show_item_middle(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_show_item_bottom(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_display(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_vposition(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_vposition(self_: *mut Fl_Tree, pos: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_hposition(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_hposition(self_: *mut Fl_Tree, pos: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_is_scrollbar(self_: *mut Fl_Tree, w: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_scrollbar_size(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_scrollbar_size(self_: *mut Fl_Tree, size: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_is_vscroll_visible(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_is_hscroll_visible(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_set_callback_item(self_: *mut Fl_Tree, item: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_callback_item(self_: *mut Fl_Tree) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_set_callback_reason(self_: *mut Fl_Tree, reason: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_callback_reason(self_: *const Fl_Tree) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_item_pathname(
        self_: *const Fl_Tree,
        pathname: *mut ::std::os::raw::c_char,
        pathnamelen: ::std::os::raw::c_int,
        item: *const Fl_Tree_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_new(
        tree: *mut Fl_Tree,
        txt: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_draw_item_content(
        item: *mut Fl_Tree_Item,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                self_: *mut Fl_Tree_Item,
                arg1: ::std::os::raw::c_int,
                arg2: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tree_Item_set_user_data(item: *mut Fl_Tree_Item, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tree_Item_user_data(item: *const Fl_Tree_Item) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_Item_x(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_y(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_w(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_h(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_label_x(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_label_y(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_label_w(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_label_h(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_show_self(
        self_: *const Fl_Tree_Item,
        indent: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Tree_set_Item_label(self_: *mut Fl_Tree_Item, val: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tree_Item_label(self_: *const Fl_Tree_Item) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tree_Item_set_labelfont(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_Item_labelfont(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_set_labelsize(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_Item_labelsize(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_set_labelfgcolor(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_Item_labelfgcolor(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_Item_set_labelcolor(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_Item_labelcolor(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_Item_set_labelbgcolor(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tree_Item_labelbgcolor(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tree_Item_set_widget(self_: *mut Fl_Tree_Item, val: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Tree_Item_widget(self_: *const Fl_Tree_Item) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Tree_Item_children(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_child(
        self_: *const Fl_Tree_Item,
        t: ::std::os::raw::c_int,
    ) -> *const Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_has_children(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_find_child(
        self_: *mut Fl_Tree_Item,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_remove_child(
        self_: *mut Fl_Tree_Item,
        new_label: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_clear_children(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_swap_children(
        self_: *mut Fl_Tree_Item,
        a: *mut Fl_Tree_Item,
        b: *mut Fl_Tree_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_find_child_item(
        self_: *const Fl_Tree_Item,
        name: *const ::std::os::raw::c_char,
    ) -> *const Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_replace(
        self_: *mut Fl_Tree_Item,
        new_item: *mut Fl_Tree_Item,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_replace_child(
        self_: *mut Fl_Tree_Item,
        olditem: *mut Fl_Tree_Item,
        newitem: *mut Fl_Tree_Item,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_deparent(
        self_: *mut Fl_Tree_Item,
        index: ::std::os::raw::c_int,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_reparent(
        self_: *mut Fl_Tree_Item,
        newchild: *mut Fl_Tree_Item,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_move(
        self_: *mut Fl_Tree_Item,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_move_above(
        self_: *mut Fl_Tree_Item,
        item: *mut Fl_Tree_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_move_below(
        self_: *mut Fl_Tree_Item,
        item: *mut Fl_Tree_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_move_into(
        self_: *mut Fl_Tree_Item,
        item: *mut Fl_Tree_Item,
        pos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_depth(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_prev(self_: *mut Fl_Tree_Item) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_next(self_: *mut Fl_Tree_Item) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_next_sibling(self_: *mut Fl_Tree_Item) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_prev_sibling(self_: *mut Fl_Tree_Item) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_update_prev_next(self_: *mut Fl_Tree_Item, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_Item_parent(self_: *const Fl_Tree_Item) -> *const Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_set_parent(self_: *mut Fl_Tree_Item, val: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_tree(self_: *const Fl_Tree_Item) -> *const Fl_Tree;
}
extern "C" {
    pub fn Fl_Tree_Item_open(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_close(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_is_open(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_is_close(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_open_toggle(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_select(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_Item_select_toggle(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_select_all(self_: *mut Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_deselect(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_deselect_all(self_: *mut Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_is_root(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_is_visible(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_is_active(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tree_Item_is_activated(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tree_Item_deactivate(self_: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_activate(self_: *mut Fl_Tree_Item, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_Item_is_selected(self_: *const Fl_Tree_Item) -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_total(self_: *const Fl_Tree_Item_Array) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_swap(
        self_: *mut Fl_Tree_Item_Array,
        ax: ::std::os::raw::c_int,
        bx: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tree_Item_Array_move(
        self_: *mut Fl_Tree_Item_Array,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_deparent(
        self_: *mut Fl_Tree_Item_Array,
        pos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_reparent(
        self_: *mut Fl_Tree_Item_Array,
        item: *mut Fl_Tree_Item,
        newparent: *mut Fl_Tree_Item,
        pos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_clear(self_: *mut Fl_Tree_Item_Array);
}
extern "C" {
    pub fn Fl_Tree_Item_Array_add(self_: *mut Fl_Tree_Item_Array, val: *mut Fl_Tree_Item);
}
extern "C" {
    pub fn Fl_Tree_Item_Array_insert(
        self_: *mut Fl_Tree_Item_Array,
        pos: ::std::os::raw::c_int,
        new_item: *mut Fl_Tree_Item,
    );
}
extern "C" {
    pub fn Fl_Tree_Item_Array_replace(
        self_: *mut Fl_Tree_Item_Array,
        pos: ::std::os::raw::c_int,
        new_item: *mut Fl_Tree_Item,
    );
}
extern "C" {
    pub fn Fl_Tree_Item_Array_remove(self_: *mut Fl_Tree_Item_Array, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tree_Item_Array_remove_item(
        self_: *mut Fl_Tree_Item_Array,
        item: *mut Fl_Tree_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_at(
        self_: *mut Fl_Tree_Item_Array,
        index: ::std::os::raw::c_int,
    ) -> *mut Fl_Tree_Item;
}
extern "C" {
    pub fn Fl_Tree_Item_Array_delete(self_: *mut Fl_Tree_Item_Array);
}
extern "C" {
    pub fn Fl_Tree_Item_usericon(self_: *const Fl_Tree_Item) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tree_Item_set_usericon(self_: *mut Fl_Tree_Item, val: *mut ::std::os::raw::c_void);
}
