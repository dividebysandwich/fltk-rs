#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Group {
    _unused: [u8; 0],
}


extern "C" {
    pub fn Fl_Group_new(
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        title: *const libc::c_char,
    ) -> *mut Fl_Group;

    pub fn Fl_Group_begin(arg1: *mut Fl_Group);

    pub fn Fl_Group_end(arg1: *mut Fl_Group);

    pub fn Fl_Group_show(arg1: *mut Fl_Group);

    pub fn Fl_Group_hide(arg1: *mut Fl_Group);

    pub fn Fl_Group_set_label(
        arg1: *mut Fl_Group,
        title: *const libc::c_char,
    );

    pub fn Fl_Group_redraw(
        arg1: *mut Fl_Group,
    );

    pub fn Fl_Group_activate(arg1: *mut Fl_Group);


    pub fn Fl_Group_deactivate(arg1: *mut Fl_Group);


    pub fn Fl_Group_redraw_label(arg1: *mut Fl_Group);


    pub fn Fl_Group_resize(
        arg1: *mut Fl_Group,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );

    pub fn Fl_Group_tooltip(arg1: *mut Fl_Group) -> *const libc::c_char;

    pub fn Fl_Group_set_tooltip(arg1: *mut Fl_Group, txt: *const libc::c_char);

    pub fn Fl_Group_get_type(arg1: *mut Fl_Group) -> libc::c_int;

    pub fn Fl_Group_set_type(arg1: *mut Fl_Group, typ: libc::c_int); 

    pub fn Fl_Group_color(arg1: *mut Fl_Group) -> libc::c_int;

    pub fn Fl_Group_set_color(arg1: *mut Fl_Group, color: libc::c_int);

    pub fn Fl_Group_label_color(arg1: *mut Fl_Group) -> libc::c_int;

    pub fn Fl_Group_set_label_color(arg1: *mut Fl_Group, color: libc::c_int);

    pub fn Fl_Group_label_font(arg1: *mut Fl_Group) -> libc::c_int;

    pub fn Fl_Group_set_label_font(arg1: *mut Fl_Group, font: libc::c_int);

    pub fn Fl_Group_label_size(arg1: *mut Fl_Group) -> libc::c_int;

    pub fn Fl_Group_set_label_size(arg1: *mut Fl_Group, sz: libc::c_int);

    pub fn Fl_Group_label_type(arg1: *mut Fl_Group) -> libc::c_int;
    
    pub fn Fl_Group_set_label_type(arg1: *mut Fl_Group, typ: libc::c_int);

    pub fn Fl_Group_box(arg1: *mut Fl_Group) -> libc::c_int;
    
    pub fn Fl_Group_set_box(arg1: *mut Fl_Group, typ: libc::c_int);
}
