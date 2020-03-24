pub use crate::enums::*;
pub use crate::fl;
pub use crate::menu::*;
use fltk_sys::widget::*;
use std::os::raw;
use std::error::Error;
use std::fmt;
use std::io;
use std::convert::From;

/// Error types returned by fltk-rs + wrappers of std::io errors
#[derive(Debug)]
pub enum FltkError {
    Io(io::Error),
    Internal(FltkErrorKind),
    Unknown(String)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FltkErrorKind {
    FailedToRun,
    FailedToLock,
    ResourceNotFound,
}

impl FltkErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            FltkErrorKind::FailedToRun => "Failed to run FLTK!",
            FltkErrorKind::FailedToLock => "Failed to initialize app for multithreading!",
            FltkErrorKind::ResourceNotFound => "Resource Not Found!"
        }
    }
}

impl Error for FltkError {
    fn description(&self) -> &str {
        match *self {
            FltkError::Io(ref err) => err.description(),
            FltkError::Internal(ref err) => err.as_str(),
            FltkError::Unknown(ref err) => err,
        }
    }
}

impl fmt::Display for FltkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FltkError::Io(ref err) => err.fmt(f),
            FltkError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
            FltkError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for FltkError {
    fn from(err: io::Error) -> FltkError {
        FltkError::Io(err)
    }
}

pub trait WidgetTrait {
    /// Creates a new widget, takes an x, y coordinates, as well as a width and height, plus a title
    fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
    /// Sets the widget's label
    fn set_label(&mut self, title: &str);
    /// Redraws a widget, necessary for resizing and changing positions
    fn redraw(&mut self);
    /// Shows the widget
    fn show(&mut self);
    /// Hides the widget
    fn hide(&mut self);
    /// Returns the x coordinate of the widget
    fn x(&self) -> i32;
    /// Returns the y coordinate of the widget
    fn y(&self) -> i32;
    /// Returns the width of the widget
    fn width(&self) -> i32;
    /// Returns the height of the widget
    fn height(&self) -> i32;
    /// Returns the label of the widget
    fn label(&self) -> String;
    /// transforms a widget to a base Fl_Widget, for internal use
    fn as_widget_ptr(&self) -> *mut Fl_Widget;
    /// Activates the widget
    fn activate(&mut self);
    /// Deactivates the widget
    fn deactivate(&mut self);
    /// Redraws the label of the widget
    fn redraw_label(&mut self);
    /// Resizes and/or moves the widget, takes x, y, width and height
    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Returns the tooltip text
    fn tooltip(&self) -> String;
    /// Sets the tooltip text
    fn set_tooltip(&mut self, txt: &str);
    /// Returns the widget type
    fn get_type<T: WidgetType>(&self) -> T;
    /// Sets the widget type
    fn set_type<T: WidgetType>(&mut self, typ: T);
    /// Returns the widget color
    fn color(&self) -> Color;
    /// Sets the widget's color
    fn set_color(&mut self, color: Color);
    /// Returns the widget label's color
    fn label_color(&self) -> Color;
    /// Sets the widget label's color
    fn set_label_color(&mut self, color: Color);
    /// Returns the widget label's font
    fn label_font(&self) -> Font;
    /// Sets the widget label's font
    fn set_label_font(&mut self, font: Font);
    /// Returns the widget label's size
    fn label_size(&self) -> usize;
    /// Sets the widget label's size
    fn set_label_size(&mut self, sz: usize);
    /// Returns the widget label's type
    fn label_type<T: WidgetType>(&self) -> T;
    /// Sets the widget label's type
    fn set_label_type<T: WidgetType>(&mut self, typ: T);
    /// Returns the widget's frame type
    fn frame<T: WidgetType>(&self) -> T;
    /// Sets the widget's frame type
    fn set_frame<T: WidgetType>(&mut self, typ: T);
    /// Returns whether the widget was changed
    fn changed(&self) -> bool;
    /// Mark the widget as changed
    fn set_changed(&mut self);
    /// Mark the widget as unchanged
    fn clear_changed(&mut self);
    /// Returns the alignment of the widget
    fn align(&self) -> Align;
    /// Sets the alignment of the widget
    fn set_align(&mut self, align: Align);
    /// Sets the image of the widget
    fn set_image<Image: ImageTrait>(&mut self, image: Image);
    /// Sets the callback when the widget is triggered (clicks for example)
    fn set_callback<'a>(&'a mut self, cb: Box<dyn FnMut() + 'a>);
    /// Set a custom handler, where events are managed manually
    fn set_custom_handler<'a>(&'a mut self, cb: Box<dyn FnMut(Event) -> i32 + 'a>);
}

pub trait GroupTrait: WidgetTrait {
    /// Begins a group, used for widgets implementing the group trait
    fn begin(&self);
    /// Ends a group, used for widgets implementing the group trait
    fn end(&self);
    /// Find a widget within a group and return its index
    fn find<Widget: WidgetTrait>(&self, widget: &Widget) -> usize;
    /// Add a widget to a group
    fn add<Widget: WidgetTrait>(&mut self, widget: &Widget);
    /// Insert a widget to a group at a certain index
    fn insert<Widget: WidgetTrait>(&mut self, widget: &Widget, index: usize);
    /// Remove a widget from a group
    fn remove(&mut self, index: usize);
    /// Clear a group from all widgets
    fn clear(&mut self);
    /// Return the number of children in a group
    fn children(&self) -> usize;
    /// Make the group resizable, useful for window widgets
    fn make_resizable(&self, val: bool);
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}

pub trait WindowTrait: GroupTrait {
    /// Makes a window modal
    fn make_modal(&mut self, val: bool);
    /// Makes a window fullscreen
    fn fullscreen(&mut self, val: bool);
    /// Makes the window current
    fn make_current(&mut self);
    /// Sets the windows icon
    fn set_icon<Image: ImageTrait>(&mut self, image: Image);
}

pub trait InputTrait: WidgetTrait  {
    /// Returns the value inside the input/output widget
    fn value(&self) -> String;
    /// Sets the value inside an input/output widget
    fn set_value(&self, val: &str);
    /// Returns the maximum size (in bytes) accepted by an input/output widget
    fn maximum_size(&self) -> usize;
    /// Sets the maximum size (in bytes) accepted by an input/output widget
    fn set_maximum_size(&mut self, val: usize);
    /// Returns the postion inside an input/output widget
    fn position(&self) -> i32;
    /// Sets the postion inside an input/output widget
    fn set_position(&mut self, val: i32);
    /// Returns the mark inside an input/output widget
    fn mark(&self) -> i32;
    /// Sets the mark inside an input/output widget
    fn set_mark(&mut self, val: i32);
    /// Replace content with a &str
    fn replace(&mut self, beg: usize, end: usize, val: &str);
    /// Insert a &str
    fn insert(&mut self, txt: &str);
    /// Append a &str
    fn append(&mut self, txt: &str);
    /// Copy the value within the widget
    fn copy(&mut self);
    /// Undo changes
    fn undo(&mut self);
    /// Cut the value within the widget
    fn cut(&mut self);
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, font: Font);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, color: Color);
    /// Return the text size
    fn text_size(&self) -> usize;
    /// Sets the text size
    fn set_text_size(&mut self, sz: usize);
    /// Returns whether the input/output widget is readonly
    fn readonly(&self) -> bool;
    /// Set readonly status of the input/output widget
    fn set_readonly(&mut self, val: bool);
    /// Return whether text is wrapped inside an input/output widget
    fn wrap(&self) -> bool;
    /// Set whether text is wrapped inside an input/output widget
    fn set_wrap(&mut self, val: bool);
}

pub trait MenuTrait: WidgetTrait  {
    /// Get a menu item by name
    fn get_item(&self, name: &str) -> crate::menu::MenuItem;
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, c: Font);
    /// Return the text size
    fn text_size(&self) -> usize;
    /// Sets the text size
    fn set_text_size(&mut self, c: usize);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, c: Color);
    /// Add a menu item along with its callback
    fn add<'a>(&'a mut self, name: &str, shortcut: i32, flag: MenuFlag, cb: Box<dyn FnMut() + 'a>);
    /// Adds a simple text option to the Choice and MenuButton widgets
    fn add_choice(&mut self, text: &str);
    /// Gets the user choice from the Choice and MenuButton widgets
    fn get_choice(&self) -> String;
}

pub trait ValuatorTrait: WidgetTrait  {
    /// Set bounds of a valuator
    fn set_bounds(&mut self, a: f64, b: f64);
    /// Get the minimum bound of a valuator
    fn minimum(&self) -> f64;
    /// Set the minimum bound of a valuator
    fn set_minimum(&mut self, a: f64);
    /// Get the maximum bound of a valuator
    fn maximum(&self) -> f64;
    /// Set the maximum bound of a valuator
    fn set_maximum(&mut self, a: f64);
    /// Set the range of a valuator
    fn set_range(&mut self, a: f64, b: f64);
    /// Set change step of a valuator
    fn set_step(&mut self, a: f64, b: i32);
    /// Get change step of a valuator
    fn step(&self) -> f64;
    /// Set the precision of a valuator
    fn set_precision(&mut self, digits: i32);
    /// Get the value of a valuator
    fn value(&self) -> f64;
    /// Set the value of a valuator
    fn set_value(&mut self, arg2: f64);
    /// Set the format of a valuator
    fn format(&mut self, arg2: &str);
    /// Round the valuator
    fn round(&self, arg2: f64) -> f64;
    /// Clamp the valuator
    fn clamp(&self, arg2: f64) -> f64;
    /// Increment the valuator
    fn increment(&mut self, arg2: f64, arg3: i32) -> f64;
}

pub trait ImageTrait {
    /// Creates an image object from a path
    fn new(path: std::path::PathBuf) -> Self;
    /// Draws the image at the presupplied coordinates and size
    fn draw(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Return the width of the image
    fn width(&self) -> i32;
    /// Return the height of the image
    fn height(&self) -> i32;
    /// Returns a pointer of the image, for internal use
    fn as_ptr(&self) -> *mut raw::c_void;
}