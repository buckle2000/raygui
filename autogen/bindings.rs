#[inline]
pub fn enable() {
    unsafe { GuiEnable() }
}

#[inline]
pub fn disable() {
    unsafe { GuiDisable() }
}

#[inline]
pub fn lock() {
    unsafe { GuiLock() }
}

#[inline]
pub fn unlock() {
    unsafe { GuiUnlock() }
}

#[inline]
pub fn state(state: i32, ) {
    unsafe { GuiState(state, ) }
}

#[inline]
pub fn text_align(align: i32, ) {
    unsafe { GuiTextAlign(align, ) }
}

#[inline]
pub fn font(font: impl Into<Font>, ) {
    unsafe { GuiFont(font.into(), ) }
}

#[inline]
pub fn fade(alpha: f32, ) {
    unsafe { GuiFade(alpha, ) }
}

#[inline]
pub fn set_style(control: i32, property: i32, value: i32, ) {
    unsafe { GuiSetStyle(control, property, value, ) }
}

#[inline]
pub fn get_style(control: i32, property: i32, ) -> i32 {
    unsafe { GuiGetStyle(control, property, ) }
}

#[inline]
pub fn window_box(bounds: impl Into<Rectangle>, text: &str, ) -> bool {
    unsafe { GuiWindowBox(bounds.into(), text.as_native(), ) }
}

#[inline]
pub fn group_box(bounds: impl Into<Rectangle>, text: &str, ) {
    unsafe { GuiGroupBox(bounds.into(), text.as_native(), ) }
}

#[inline]
pub fn line(bounds: impl Into<Rectangle>, thick: i32, ) {
    unsafe { GuiLine(bounds.into(), thick, ) }
}

#[inline]
pub fn panel(bounds: impl Into<Rectangle>, ) {
    unsafe { GuiPanel(bounds.into(), ) }
}

#[inline]
pub fn scroll_panel(bounds: impl Into<Rectangle>, content: impl Into<Rectangle>, view_scroll: impl Into<Vector2>, ) -> Vector2 {
    unsafe { GuiScrollPanel(bounds.into(), content.into(), view_scroll.into(), ) }
}

#[inline]
pub fn label(bounds: impl Into<Rectangle>, text: &str, ) {
    unsafe { GuiLabel(bounds.into(), text.as_native(), ) }
}

#[inline]
pub fn button(bounds: impl Into<Rectangle>, text: &str, ) -> bool {
    unsafe { GuiButton(bounds.into(), text.as_native(), ) }
}

#[inline]
pub fn label_button(bounds: impl Into<Rectangle>, text: &str, ) -> bool {
    unsafe { GuiLabelButton(bounds.into(), text.as_native(), ) }
}

#[inline]
pub fn image_button(bounds: impl Into<Rectangle>, texture: impl Into<Texture2D>, ) -> bool {
    unsafe { GuiImageButton(bounds.into(), texture.into(), ) }
}

#[inline]
pub fn image_button_ex(bounds: impl Into<Rectangle>, texture: impl Into<Texture2D>, tex_source: impl Into<Rectangle>, text: &str, ) -> bool {
    unsafe { GuiImageButtonEx(bounds.into(), texture.into(), tex_source.into(), text.as_native(), ) }
}

#[inline]
pub fn toggle(bounds: impl Into<Rectangle>, text: &str, active: bool, ) -> bool {
    unsafe { GuiToggle(bounds.into(), text.as_native(), active, ) }
}

#[inline]
pub fn toggle_group(bounds: impl Into<Rectangle>, text: &str, active: i32, ) -> i32 {
    unsafe { GuiToggleGroup(bounds.into(), text.as_native(), active, ) }
}

#[inline]
pub fn toggle_group_ex(bounds: impl Into<Rectangle>, text: &str, active: i32, padding: i32, columns: i32, ) -> i32 {
    unsafe { GuiToggleGroupEx(bounds.into(), text.as_native(), active, padding, columns, ) }
}

#[inline]
pub fn check_box(bounds: impl Into<Rectangle>, text: &str, checked: bool, ) -> bool {
    unsafe { GuiCheckBox(bounds.into(), text.as_native(), checked, ) }
}

#[inline]
pub fn combo_box(bounds: impl Into<Rectangle>, text: &str, active: i32, ) -> i32 {
    unsafe { GuiComboBox(bounds.into(), text.as_native(), active, ) }
}

#[inline]
pub fn slider(bounds: impl Into<Rectangle>, text: &str, value: f32, min_value: f32, max_value: f32, show_value: bool, ) -> f32 {
    unsafe { GuiSlider(bounds.into(), text.as_native(), value, min_value, max_value, show_value, ) }
}

#[inline]
pub fn slider_bar(bounds: impl Into<Rectangle>, text: &str, value: f32, min_value: f32, max_value: f32, show_value: bool, ) -> f32 {
    unsafe { GuiSliderBar(bounds.into(), text.as_native(), value, min_value, max_value, show_value, ) }
}

#[inline]
pub fn progress_bar(bounds: impl Into<Rectangle>, text: &str, value: f32, min_value: f32, max_value: f32, show_value: bool, ) -> f32 {
    unsafe { GuiProgressBar(bounds.into(), text.as_native(), value, min_value, max_value, show_value, ) }
}

#[inline]
pub fn status_bar(bounds: impl Into<Rectangle>, text: &str, offset_x: i32, ) {
    unsafe { GuiStatusBar(bounds.into(), text.as_native(), offset_x, ) }
}

#[inline]
pub fn dummy_rec(bounds: impl Into<Rectangle>, text: &str, ) {
    unsafe { GuiDummyRec(bounds.into(), text.as_native(), ) }
}

#[inline]
pub fn scroll_bar(bounds: impl Into<Rectangle>, value: i32, min_value: i32, max_value: i32, ) -> i32 {
    unsafe { GuiScrollBar(bounds.into(), value, min_value, max_value, ) }
}

#[inline]
pub fn color_picker(bounds: impl Into<Rectangle>, color: impl Into<Color>, ) -> Color {
    unsafe { GuiColorPicker(bounds.into(), color.into(), ) }
}

#[inline]
pub fn message_box(bounds: impl Into<Rectangle>, window_title: &str, message: &str, ) -> bool {
    unsafe { GuiMessageBox(bounds.into(), window_title.as_native(), message.as_native(), ) }
}

#[inline]
pub fn grid(bounds: impl Into<Rectangle>, spacing: f32, subdivs: i32, ) -> Vector2 {
    unsafe { GuiGrid(bounds.into(), spacing, subdivs, ) }
}

#[inline]
pub fn load_style(file_name: &str, ) {
    unsafe { GuiLoadStyle(file_name.as_native(), ) }
}

#[inline]
pub fn load_style_default() {
    unsafe { GuiLoadStyleDefault() }
}

#[inline]
pub fn update_style_complete() {
    unsafe { GuiUpdateStyleComplete() }
}

#[inline]
pub fn slider_pro(bounds: impl Into<Rectangle>, text: &str, value: f32, min_value: f32, max_value: f32, slider_width: i32, show_value: bool, ) -> f32 {
    unsafe { GuiSliderPro(bounds.into(), text.as_native(), value, min_value, max_value, slider_width, show_value, ) }
}

#[inline]
pub fn color_panel(bounds: impl Into<Rectangle>, color: impl Into<Color>, ) -> Color {
    unsafe { GuiColorPanel(bounds.into(), color.into(), ) }
}

#[inline]
pub fn color_bar_alpha(bounds: impl Into<Rectangle>, alpha: f32, ) -> f32 {
    unsafe { GuiColorBarAlpha(bounds.into(), alpha, ) }
}

#[inline]
pub fn color_bar_hue(bounds: impl Into<Rectangle>, hue: f32, ) -> f32 {
    unsafe { GuiColorBarHue(bounds.into(), hue, ) }
}

#[inline]
pub fn dropdown_box(bounds: impl Into<Rectangle>, text: &str, active: &mut i32, edit_mode: bool, ) -> bool {
    unsafe { GuiDropdownBox(bounds.into(), text.as_native(), active as *mut i32, edit_mode, ) }
}

#[inline]
pub fn spinner(bounds: impl Into<Rectangle>, value: &mut i32, min_value: i32, max_value: i32, btn_width: i32, edit_mode: bool, ) -> bool {
    unsafe { GuiSpinner(bounds.into(), value as *mut i32, min_value, max_value, btn_width, edit_mode, ) }
}

#[inline]
pub fn value_box(bounds: impl Into<Rectangle>, value: &mut i32, min_value: i32, max_value: i32, edit_mode: bool, ) -> bool {
    unsafe { GuiValueBox(bounds.into(), value as *mut i32, min_value, max_value, edit_mode, ) }
}

#[inline]
pub fn list_view(bounds: impl Into<Rectangle>, text: &str, active: &mut i32, scroll_index: &mut i32, edit_mode: bool, ) -> bool {
    unsafe { GuiListView(bounds.into(), text.as_native(), active as *mut i32, scroll_index as *mut i32, edit_mode, ) }
}

