// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use BackgroundPatternType;
use DrawSpacesFlags;
use SmartHomeEndType;
use View;

glib_wrapper! {
    pub struct Map(Object<gtk_source_sys::GtkSourceMap, gtk_source_sys::GtkSourceMapClass, MapClass>) @extends View, gtk::TextView, gtk::Container, gtk::Widget, @implements gtk::Buildable, gtk::Scrollable;

    match fn {
        get_type => || gtk_source_sys::gtk_source_map_get_type(),
    }
}

impl Map {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub fn new() -> Map {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(gtk_source_sys::gtk_source_map_new()).unsafe_cast() }
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct MapBuilder {
    view: Option<View>,
    auto_indent: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    background_pattern: Option<BackgroundPatternType>,
    draw_spaces: Option<DrawSpacesFlags>,
    highlight_current_line: Option<bool>,
    indent_on_tab: Option<bool>,
    indent_width: Option<i32>,
    insert_spaces_instead_of_tabs: Option<bool>,
    right_margin_position: Option<u32>,
    show_line_marks: Option<bool>,
    show_line_numbers: Option<bool>,
    show_right_margin: Option<bool>,
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    smart_backspace: Option<bool>,
    smart_home_end: Option<SmartHomeEndType>,
    tab_width: Option<u32>,
    accepts_tab: Option<bool>,
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    bottom_margin: Option<i32>,
    buffer: Option<gtk::TextBuffer>,
    cursor_visible: Option<bool>,
    editable: Option<bool>,
    im_module: Option<String>,
    indent: Option<i32>,
    //input-hints: /*Unknown type*/,
    //input-purpose: /*Unknown type*/,
    //justification: /*Unknown type*/,
    left_margin: Option<i32>,
    monospace: Option<bool>,
    overwrite: Option<bool>,
    pixels_above_lines: Option<i32>,
    pixels_below_lines: Option<i32>,
    pixels_inside_wrap: Option<i32>,
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    populate_all: Option<bool>,
    right_margin: Option<i32>,
    //tabs: /*Unknown type*/,
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    top_margin: Option<i32>,
    wrap_mode: Option<gtk::WrapMode>,
    border_width: Option<u32>,
    child: Option<gtk::Widget>,
    //resize-mode: /*Unknown type*/,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    double_buffered: Option<bool>,
    //events: /*Unknown type*/,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    //halign: /*Unknown type*/,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    margin_end: Option<i32>,
    margin_left: Option<i32>,
    margin_right: Option<i32>,
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    opacity: Option<f64>,
    parent: Option<gtk::Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    //valign: /*Unknown type*/,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    hadjustment: Option<gtk::Adjustment>,
    hscroll_policy: Option<gtk::ScrollablePolicy>,
    vadjustment: Option<gtk::Adjustment>,
    vscroll_policy: Option<gtk::ScrollablePolicy>,
}

impl MapBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Map {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref view) = self.view {
            properties.push(("view", view));
        }
        if let Some(ref auto_indent) = self.auto_indent {
            properties.push(("auto-indent", auto_indent));
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref background_pattern) = self.background_pattern {
                properties.push(("background-pattern", background_pattern));
            }
        }
        if let Some(ref draw_spaces) = self.draw_spaces {
            properties.push(("draw-spaces", draw_spaces));
        }
        if let Some(ref highlight_current_line) = self.highlight_current_line {
            properties.push(("highlight-current-line", highlight_current_line));
        }
        if let Some(ref indent_on_tab) = self.indent_on_tab {
            properties.push(("indent-on-tab", indent_on_tab));
        }
        if let Some(ref indent_width) = self.indent_width {
            properties.push(("indent-width", indent_width));
        }
        if let Some(ref insert_spaces_instead_of_tabs) = self.insert_spaces_instead_of_tabs {
            properties.push((
                "insert-spaces-instead-of-tabs",
                insert_spaces_instead_of_tabs,
            ));
        }
        if let Some(ref right_margin_position) = self.right_margin_position {
            properties.push(("right-margin-position", right_margin_position));
        }
        if let Some(ref show_line_marks) = self.show_line_marks {
            properties.push(("show-line-marks", show_line_marks));
        }
        if let Some(ref show_line_numbers) = self.show_line_numbers {
            properties.push(("show-line-numbers", show_line_numbers));
        }
        if let Some(ref show_right_margin) = self.show_right_margin {
            properties.push(("show-right-margin", show_right_margin));
        }
        #[cfg(any(feature = "v3_18", feature = "dox"))]
        {
            if let Some(ref smart_backspace) = self.smart_backspace {
                properties.push(("smart-backspace", smart_backspace));
            }
        }
        if let Some(ref smart_home_end) = self.smart_home_end {
            properties.push(("smart-home-end", smart_home_end));
        }
        if let Some(ref tab_width) = self.tab_width {
            properties.push(("tab-width", tab_width));
        }
        if let Some(ref accepts_tab) = self.accepts_tab {
            properties.push(("accepts-tab", accepts_tab));
        }
        #[cfg(any(feature = "v3_18", feature = "dox"))]
        {
            if let Some(ref bottom_margin) = self.bottom_margin {
                properties.push(("bottom-margin", bottom_margin));
            }
        }
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref cursor_visible) = self.cursor_visible {
            properties.push(("cursor-visible", cursor_visible));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref im_module) = self.im_module {
            properties.push(("im-module", im_module));
        }
        if let Some(ref indent) = self.indent {
            properties.push(("indent", indent));
        }
        if let Some(ref left_margin) = self.left_margin {
            properties.push(("left-margin", left_margin));
        }
        if let Some(ref monospace) = self.monospace {
            properties.push(("monospace", monospace));
        }
        if let Some(ref overwrite) = self.overwrite {
            properties.push(("overwrite", overwrite));
        }
        if let Some(ref pixels_above_lines) = self.pixels_above_lines {
            properties.push(("pixels-above-lines", pixels_above_lines));
        }
        if let Some(ref pixels_below_lines) = self.pixels_below_lines {
            properties.push(("pixels-below-lines", pixels_below_lines));
        }
        if let Some(ref pixels_inside_wrap) = self.pixels_inside_wrap {
            properties.push(("pixels-inside-wrap", pixels_inside_wrap));
        }
        #[cfg(any(feature = "v3_8", feature = "dox"))]
        {
            if let Some(ref populate_all) = self.populate_all {
                properties.push(("populate-all", populate_all));
            }
        }
        if let Some(ref right_margin) = self.right_margin {
            properties.push(("right-margin", right_margin));
        }
        #[cfg(any(feature = "v3_18", feature = "dox"))]
        {
            if let Some(ref top_margin) = self.top_margin {
                properties.push(("top-margin", top_margin));
            }
        }
        if let Some(ref wrap_mode) = self.wrap_mode {
            properties.push(("wrap-mode", wrap_mode));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref double_buffered) = self.double_buffered {
            properties.push(("double-buffered", double_buffered));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        #[cfg(any(feature = "v3_12", feature = "dox"))]
        {
            if let Some(ref margin_end) = self.margin_end {
                properties.push(("margin-end", margin_end));
            }
        }
        if let Some(ref margin_left) = self.margin_left {
            properties.push(("margin-left", margin_left));
        }
        if let Some(ref margin_right) = self.margin_right {
            properties.push(("margin-right", margin_right));
        }
        #[cfg(any(feature = "v3_12", feature = "dox"))]
        {
            if let Some(ref margin_start) = self.margin_start {
                properties.push(("margin-start", margin_start));
            }
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        #[cfg(any(feature = "v3_8", feature = "dox"))]
        {
            if let Some(ref opacity) = self.opacity {
                properties.push(("opacity", opacity));
            }
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref hadjustment) = self.hadjustment {
            properties.push(("hadjustment", hadjustment));
        }
        if let Some(ref hscroll_policy) = self.hscroll_policy {
            properties.push(("hscroll-policy", hscroll_policy));
        }
        if let Some(ref vadjustment) = self.vadjustment {
            properties.push(("vadjustment", vadjustment));
        }
        if let Some(ref vscroll_policy) = self.vscroll_policy {
            properties.push(("vscroll-policy", vscroll_policy));
        }
        glib::Object::new(Map::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn view<P: IsA<View>>(mut self, view: &P) -> Self {
        self.view = Some(view.clone().upcast());
        self
    }

    pub fn auto_indent(mut self, auto_indent: bool) -> Self {
        self.auto_indent = Some(auto_indent);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn background_pattern(mut self, background_pattern: BackgroundPatternType) -> Self {
        self.background_pattern = Some(background_pattern);
        self
    }

    pub fn draw_spaces(mut self, draw_spaces: DrawSpacesFlags) -> Self {
        self.draw_spaces = Some(draw_spaces);
        self
    }

    pub fn highlight_current_line(mut self, highlight_current_line: bool) -> Self {
        self.highlight_current_line = Some(highlight_current_line);
        self
    }

    pub fn indent_on_tab(mut self, indent_on_tab: bool) -> Self {
        self.indent_on_tab = Some(indent_on_tab);
        self
    }

    pub fn indent_width(mut self, indent_width: i32) -> Self {
        self.indent_width = Some(indent_width);
        self
    }

    pub fn insert_spaces_instead_of_tabs(mut self, insert_spaces_instead_of_tabs: bool) -> Self {
        self.insert_spaces_instead_of_tabs = Some(insert_spaces_instead_of_tabs);
        self
    }

    pub fn right_margin_position(mut self, right_margin_position: u32) -> Self {
        self.right_margin_position = Some(right_margin_position);
        self
    }

    pub fn show_line_marks(mut self, show_line_marks: bool) -> Self {
        self.show_line_marks = Some(show_line_marks);
        self
    }

    pub fn show_line_numbers(mut self, show_line_numbers: bool) -> Self {
        self.show_line_numbers = Some(show_line_numbers);
        self
    }

    pub fn show_right_margin(mut self, show_right_margin: bool) -> Self {
        self.show_right_margin = Some(show_right_margin);
        self
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub fn smart_backspace(mut self, smart_backspace: bool) -> Self {
        self.smart_backspace = Some(smart_backspace);
        self
    }

    pub fn smart_home_end(mut self, smart_home_end: SmartHomeEndType) -> Self {
        self.smart_home_end = Some(smart_home_end);
        self
    }

    pub fn tab_width(mut self, tab_width: u32) -> Self {
        self.tab_width = Some(tab_width);
        self
    }

    pub fn accepts_tab(mut self, accepts_tab: bool) -> Self {
        self.accepts_tab = Some(accepts_tab);
        self
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub fn bottom_margin(mut self, bottom_margin: i32) -> Self {
        self.bottom_margin = Some(bottom_margin);
        self
    }

    pub fn buffer<P: IsA<gtk::TextBuffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn cursor_visible(mut self, cursor_visible: bool) -> Self {
        self.cursor_visible = Some(cursor_visible);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn im_module(mut self, im_module: &str) -> Self {
        self.im_module = Some(im_module.to_string());
        self
    }

    pub fn indent(mut self, indent: i32) -> Self {
        self.indent = Some(indent);
        self
    }

    pub fn left_margin(mut self, left_margin: i32) -> Self {
        self.left_margin = Some(left_margin);
        self
    }

    pub fn monospace(mut self, monospace: bool) -> Self {
        self.monospace = Some(monospace);
        self
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    pub fn pixels_above_lines(mut self, pixels_above_lines: i32) -> Self {
        self.pixels_above_lines = Some(pixels_above_lines);
        self
    }

    pub fn pixels_below_lines(mut self, pixels_below_lines: i32) -> Self {
        self.pixels_below_lines = Some(pixels_below_lines);
        self
    }

    pub fn pixels_inside_wrap(mut self, pixels_inside_wrap: i32) -> Self {
        self.pixels_inside_wrap = Some(pixels_inside_wrap);
        self
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    pub fn populate_all(mut self, populate_all: bool) -> Self {
        self.populate_all = Some(populate_all);
        self
    }

    pub fn right_margin(mut self, right_margin: i32) -> Self {
        self.right_margin = Some(right_margin);
        self
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub fn top_margin(mut self, top_margin: i32) -> Self {
        self.top_margin = Some(top_margin);
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: gtk::WrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<gtk::Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn double_buffered(mut self, double_buffered: bool) -> Self {
        self.double_buffered = Some(double_buffered);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_left(mut self, margin_left: i32) -> Self {
        self.margin_left = Some(margin_left);
        self
    }

    pub fn margin_right(mut self, margin_right: i32) -> Self {
        self.margin_right = Some(margin_right);
        self
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<gtk::Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn hadjustment<P: IsA<gtk::Adjustment>>(mut self, hadjustment: &P) -> Self {
        self.hadjustment = Some(hadjustment.clone().upcast());
        self
    }

    pub fn hscroll_policy(mut self, hscroll_policy: gtk::ScrollablePolicy) -> Self {
        self.hscroll_policy = Some(hscroll_policy);
        self
    }

    pub fn vadjustment<P: IsA<gtk::Adjustment>>(mut self, vadjustment: &P) -> Self {
        self.vadjustment = Some(vadjustment.clone().upcast());
        self
    }

    pub fn vscroll_policy(mut self, vscroll_policy: gtk::ScrollablePolicy) -> Self {
        self.vscroll_policy = Some(vscroll_policy);
        self
    }
}

pub const NONE_MAP: Option<&Map> = None;

pub trait MapExt: 'static {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_view(&self) -> Option<View>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_view<P: IsA<View>>(&self, view: &P);

    fn get_property_view(&self) -> Option<View>;

    fn set_property_view<P: IsA<View> + SetValueOptional>(&self, view: Option<&P>);

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Map>> MapExt for O {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_map_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_view<P: IsA<View>>(&self, view: &P) {
        unsafe {
            gtk_source_sys::gtk_source_map_set_view(
                self.as_ref().to_glib_none().0,
                view.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_property_view(&self) -> Option<View> {
        unsafe {
            let mut value = Value::from_type(<View as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"view\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `view` getter")
        }
    }

    fn set_property_view<P: IsA<View> + SetValueOptional>(&self, view: Option<&P>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"view\0".as_ptr() as *const _,
                Value::from(view).to_glib_none().0,
            );
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceMap,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Map>,
        {
            let f: &F = &*(f as *const F);
            f(&Map::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::view\0".as_ptr() as *const _,
                Some(*(&notify_view_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map")
    }
}
