use crate::ui::{
    component::{Component, Event, EventCtx, Never},
    display,
    geometry::{LinearLayout, Offset, Rect},
};

use super::theme;

pub struct ScrollBar {
    area: Rect,
    layout: LinearLayout,
    arrows: bool,
    pub page_count: usize,
    pub active_page: usize,
}

impl ScrollBar {
    pub const DOT_SIZE: i32 = 6;
    /// Edge to edge.
    const DOT_INTERVAL: i32 = 6;
    /// Edge of last dot to center of arrow icon.
    const ARROW_SPACE: i32 = 26;

    const ICON_UP: &'static [u8] = include_res!("model_tt/res/scroll-up.toif");
    const ICON_DOWN: &'static [u8] = include_res!("model_tt/res/scroll-down.toif");

    fn new(layout: LinearLayout, area: Rect, page_count: usize, active_page: usize) -> Self {
        let layout = layout.align_at_center().with_spacing(Self::DOT_INTERVAL);
        let arrows = false;
        Self {
            area,
            layout,
            arrows,
            page_count,
            active_page,
        }
    }

    pub fn vertical(area: Rect, page_count: usize, active_page: usize) -> Self {
        Self::new(LinearLayout::vertical(), area, page_count, active_page)
    }

    pub fn horizontal(area: Rect, page_count: usize, active_page: usize) -> Self {
        Self::new(LinearLayout::horizontal(), area, page_count, active_page)
    }

    pub fn with_arrows(mut self) -> Self {
        self.arrows = true;
        self
    }

    pub fn has_pages(&self) -> bool {
        self.page_count > 1
    }

    pub fn has_next_page(&self) -> bool {
        self.active_page < self.page_count - 1
    }

    pub fn has_previous_page(&self) -> bool {
        self.active_page > 0
    }

    pub fn go_to_next_page(&mut self) {
        self.go_to(self.active_page.saturating_add(1).min(self.page_count - 1));
    }

    pub fn go_to_previous_page(&mut self) {
        self.go_to(self.active_page.saturating_sub(1));
    }

    pub fn go_to(&mut self, active_page: usize) {
        self.active_page = active_page;
    }
}

impl Component for ScrollBar {
    type Msg = Never;

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        let mut i = 0;
        let mut top = None;
        let mut display_icon = |top_left| {
            let icon = if i == self.active_page {
                theme::DOT_ACTIVE
            } else {
                theme::DOT_INACTIVE
            };
            display::icon_top_left(top_left, icon, theme::FG, theme::BG);
            i += 1;
            top.get_or_insert(top_left.x);
        };

        self.layout.arrange_uniform(
            self.area,
            self.page_count,
            Offset::new(Self::DOT_SIZE, Self::DOT_SIZE),
            &mut display_icon,
        );

        if self.arrows {
            let arrow_distance = self.area.center().x - top.unwrap_or(0) + Self::ARROW_SPACE;
            let offset = Offset::on_axis(self.layout.axis, arrow_distance);
            if self.has_previous_page() {
                display::icon(
                    self.area.center() - offset,
                    Self::ICON_UP,
                    theme::FG,
                    theme::BG,
                );
            }
            if self.has_next_page() {
                display::icon(
                    self.area.center() + offset,
                    Self::ICON_DOWN,
                    theme::FG,
                    theme::BG,
                );
            }
        }
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        sink(self.area);
    }
}
