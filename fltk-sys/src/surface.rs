/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Surface_Device {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Surface_Device_set_current(self_: *mut Fl_Surface_Device);
}
extern "C" {
    pub fn Fl_Surface_Device_is_current(self_: *mut Fl_Surface_Device) -> core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Surface_Device_surface() -> *mut Fl_Surface_Device;
}
extern "C" {
    pub fn Fl_Surface_Device_push_current(new_current: *mut Fl_Surface_Device);
}
extern "C" {
    pub fn Fl_Surface_Device_pop_current() -> *mut Fl_Surface_Device;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Image_Surface {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Image_Surface_new(
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        high_res: core::ffi::c_int,
    ) -> *mut Fl_Image_Surface;
}
extern "C" {
    pub fn Fl_Image_Surface_delete(s: *mut Fl_Image_Surface);
}
extern "C" {
    pub fn Fl_Image_Surface_set_current(self_: *mut Fl_Image_Surface);
}
extern "C" {
    pub fn Fl_Image_Surface_is_current(self_: *mut Fl_Image_Surface) -> core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Image_Surface_image(self_: *mut Fl_Image_Surface) -> *mut core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Image_Surface_highres_image(self_: *mut Fl_Image_Surface) -> *mut core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Image_Surface_origin(
        self_: *mut Fl_Image_Surface,
        x: *mut core::ffi::c_int,
        y: *mut core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_Surface_set_origin(
        self_: *mut Fl_Image_Surface,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_Surface_rescale(self_: *mut Fl_Image_Surface);
}
extern "C" {
    pub fn Fl_Image_Surface_draw(
        self_: *mut Fl_Image_Surface,
        widget: *mut core::ffi::c_void,
        delta_x: core::ffi::c_int,
        delta_y: core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_Surface_draw_decorated_window(
        self_: *mut Fl_Image_Surface,
        widget: *mut core::ffi::c_void,
        delta_x: core::ffi::c_int,
        delta_y: core::ffi::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_SVG_File_Surface {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_SVG_File_Surface_new(
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        file: *const core::ffi::c_char,
    ) -> *mut Fl_SVG_File_Surface;
}
extern "C" {
    pub fn Fl_SVG_File_Surface_delete(self_: *mut Fl_SVG_File_Surface);
}
extern "C" {
    pub fn Fl_SVG_File_Surface_origin(
        self_: *mut Fl_SVG_File_Surface,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_File_Surface_printable_rect(
        self_: *mut Fl_SVG_File_Surface,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
extern "C" {
    pub fn Fl_SVG_File_Surface_draw(
        self_: *mut Fl_SVG_File_Surface,
        widget: *mut core::ffi::c_void,
        delta_x: core::ffi::c_int,
        delta_y: core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_File_Surface_draw_decorated_window(
        self_: *mut Fl_SVG_File_Surface,
        widget: *mut core::ffi::c_void,
        delta_x: core::ffi::c_int,
        delta_y: core::ffi::c_int,
    );
}
