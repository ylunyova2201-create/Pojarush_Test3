//! Android input: the overlay's command lines and look deltas (android.rs), and touches on
//! the game surface as a mouse while no world holds the pointer (menus, inventories).
use super::{Game, Screen};
use crate::android::{self, Input};
use std::{cell::Cell, time::Instant};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState, MouseButton, MouseScrollDelta, Touch, TouchPhase, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

/// The finger that acts as the mouse, and a second one that scrolls.
#[derive(Clone, Copy, Default)]
struct TouchPointer {
    primary: Option<u64>,
    scroll: Option<(u64, f64)>,
}

thread_local! {
    static POINTER: Cell<TouchPointer> = Cell::new(TouchPointer::default());
}

impl Game<'_> {
    pub(super) fn android_frame(&mut self, event_loop: &ActiveEventLoop) {
        android::set_captured(self.captured && self.screen == Screen::Playing);
        if let Some(size) = self.window.as_ref().map(|w| w.inner_size()) {
            android::set_surface(size.width, size.height);
        }
        for input in android::take_input() {
            match input {
                Input::Look(x, y) => {
                    if self.pending_save.is_none()
                        && self.captured
                        && self.screen == Screen::Playing
                    {
                        self.mouse.on_move(0.0, 0.0, x, y, true, true);
                        self.last_input = Instant::now();
                    }
                }
                Input::Wheel(lines) => {
                    if let Some(id) = self.window.as_ref().map(|w| w.id()) {
                        let device_id = DeviceId::dummy();
                        self.window_event(
                            event_loop,
                            id,
                            WindowEvent::MouseWheel {
                                device_id,
                                delta: MouseScrollDelta::LineDelta(0.0, lines),
                                phase: TouchPhase::Moved,
                            },
                        );
                    }
                }
                Input::Cursor(x, y) => {
                    if let Some(id) = self.window.as_ref().map(|w| w.id()) {
                        let (sx, sy) = android::scale();
                        self.window_event(
                            event_loop,
                            id,
                            WindowEvent::CursorMoved {
                                device_id: DeviceId::dummy(),
                                position: PhysicalPosition::new(x * sx, y * sy),
                            },
                        );
                    }
                }
                Input::Button(button, down) => {
                    if let Some(id) = self.window.as_ref().map(|w| w.id()) {
                        let button = match button {
                            1 => MouseButton::Right,
                            2 => MouseButton::Middle,
                            _ => MouseButton::Left,
                        };
                        let state = if down {
                            ElementState::Pressed
                        } else {
                            ElementState::Released
                        };
                        self.window_event(
                            event_loop,
                            id,
                            WindowEvent::MouseInput {
                                device_id: DeviceId::dummy(),
                                state,
                                button,
                            },
                        );
                    }
                    self.last_input = Instant::now();
                }
                Input::Command(line) => {
                    let (reply, _answer) = std::sync::mpsc::channel();
                    if let Err(e) = self.control_command(&line, &reply, event_loop) {
                        log::debug!("overlay command {line:?}: {e}");
                    }
                    self.last_input = Instant::now();
                }
            }
        }
    }

    pub(super) fn android_touch(&mut self, event_loop: &ActiveEventLoop, id: WindowId, touch: Touch) {
        self.android_touch_event(event_loop, id, &touch);
    }
}
