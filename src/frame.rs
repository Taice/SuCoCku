mod split;
pub mod window;

use macroquad::prelude::*;
use split::Split;
use std::ops::Index;
use std::rc::Rc;

use window::Window;

use window::buffer::Buffer;

use crate::frame::split::SplitDirection;
use crate::settings::Settings;

pub struct Tab {
    pub inner: Split,
    pub selected_idx: usize,
}

impl Tab {
    pub fn resize(&mut self, dimensions: Rect, gap_size: f32) {
        self.inner.resize(dimensions, gap_size);
    }
}

impl<Idx: Into<usize>> Index<Idx> for Tab {
    type Output = Window;
    fn index(&self, index: Idx) -> &Self::Output {
        self.inner.idx(index.into(), &mut 0).unwrap()
    }
}

pub struct Frame {
    tabs: Vec<Tab>,
    curr_tab: usize,

    buffers: Vec<Buffer>,

    settings: Rc<Settings>,
    size: (f32, f32),
}

impl Frame {
    pub fn new(settings: Rc<Settings>) -> Self {
        let mut ret = Self {
            // test
            tabs: vec![Tab {
                selected_idx: 0,
                inner: Split::Split(
                    Box::new(Split::Split(
                        Box::new(Split::Split(
                            Box::new(Split::Window(Window {
                                dimensions: Rect::default(),
                                buffer_index: 0,
                            })),
                            Box::new(Split::Window(Window {
                                dimensions: Rect::default(),
                                buffer_index: 0,
                            })),
                            0.50,
                            SplitDirection::Horizontal,
                        )),
                        Box::new(Split::Window(Window {
                            dimensions: Rect::default(),
                            buffer_index: 0,
                        })),
                        0.5,
                        SplitDirection::Vertical,
                    )),
                    Box::new(Split::Window(Window {
                        dimensions: Rect::default(),
                        buffer_index: 0,
                    })),
                    0.5,
                    SplitDirection::Horizontal,
                ),
            }],
            curr_tab: 0,
            buffers: vec![Buffer::new(Rc::clone(&settings))],
            settings: settings,
            size: (0.0, 0.0),
        };
        ret.update();
        ret
    }
    pub fn draw(&mut self) {
        clear_background(self.settings.colors.bg_color);
        let half = self.settings.lines.window_gaps / 2.;
        for window in self.tabs[self.curr_tab].inner.iter() {
            window.render(&self.buffers[window.buffer_index]);
            let mut dimensions = window.dimensions;
            dimensions.y -= half;
            dimensions.x -= half;
            dimensions.w += self.settings.lines.window_gaps;
            dimensions.h += self.settings.lines.window_gaps;
            draw_rectangle_lines(
                dimensions.x,
                dimensions.y,
                dimensions.w,
                dimensions.h,
                half,
                self.settings.colors.window_gaps,
            );
        }
    }

    pub fn update(&mut self) {
        let width = screen_width();
        let height = screen_height();
        if self.size != (width, height) {
            self.size = (width, height);
            self.resize();
        }
        self.handle_input();
    }

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::LeftControl) {
            get_char_pressed();
        } else {
            let selected_win = self.tabs[self.curr_tab]
                .inner
                .idx(self.tabs[self.curr_tab].selected_idx, &mut 0)
                .unwrap();
            selected_win.update(&mut self.buffers[selected_win.buffer_index]);
        }
    }

    fn resize(&mut self) {
        let dimensions = Rect {
            x: 0.0,
            y: 0.0,
            w: self.size.0,
            h: self.size.1,
        };

        self.tabs[self.curr_tab]
            .inner
            .resize(dimensions, self.settings.lines.window_gaps);
    }
}
