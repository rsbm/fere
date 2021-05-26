use fere::prelude::*;
use std::collections::HashSet;

pub type KeyboardKey = usize;

#[derive(Clone, Debug)]
pub struct FbLookUp {
    pub id: u64,
    pub pos: Vec3,
    pub norm: Vec3,
}

#[derive(Clone, Debug)]
pub struct MouseSignal {
    pub left: bool,
    pub right: bool,
    pub wheel: bool,
}

impl MouseSignal {
    fn by_index(&mut self, i: usize) -> &mut bool {
        if i == 0 {
            &mut self.left
        } else if i == 2 {
            &mut self.wheel
        } else if i == 1 {
            &mut self.right
        } else {
            panic!("Invalid mouse signal access")
        }
    }
    fn button(i: usize) -> imgui::MouseButton {
        if i == 0 {
            imgui::MouseButton::Left
        } else if i == 2 {
            imgui::MouseButton::Middle
        } else if i == 1 {
            imgui::MouseButton::Right
        } else {
            panic!("Invalid mouse signal access")
        }
    }
}

#[derive(Clone, Debug)]
/// Represents all information about user input for a single frame
pub struct InputImage {
    //pub fbl_cursor: FbLookUp,
    //pub fbl_look: FbLookUp,
    pub mouse_pos: IVec2,
    pub mouse_pos_delta: IVec2,

    pub mouse_signal: MouseSignal,
    pub mouse_pressed: MouseSignal,

    pub wheel_delta: f32,
    pub mouse_on_screen: bool,

    pub key_signal: HashSet<KeyboardKey>,
    pub key_pressed: HashSet<KeyboardKey>,
}

pub struct InputManager {
    screen_size: IVec2,

    image: Option<InputImage>,

    // We need to keep these for additional handling
    last_pressed_keys: HashSet<KeyboardKey>,
    last_valid_mouse_pos: IVec2,
}

impl InputManager {
    pub fn new(screen_size: IVec2) -> Self {
        InputManager {
            screen_size,
            image: None,
            last_pressed_keys: Default::default(),
            last_valid_mouse_pos: IVec2::new(0, 0),
        }
    }

    pub fn get_input_image(&self) -> InputImage {
        self.image.as_ref().unwrap().clone()
    }

    pub fn update<'a>(&mut self, imgui_ctx: &imgui::Ui<'a>) {
        let io_source = imgui_ctx.io();
        // TODO : This MAX check is platform dependent?
        self.last_valid_mouse_pos = if io_source.mouse_pos == [-f32::MAX, -f32::MAX] {
            self.last_valid_mouse_pos
        } else {
            IVec2::new(
                io_source.mouse_pos[0] as i32,
                self.screen_size[1] - io_source.mouse_pos[1] as i32,
            )
        };
        let mouse_pos = self.last_valid_mouse_pos;

        let mouse_pos_delta = IVec2::new(
            io_source.mouse_delta[0] as i32,
            -io_source.mouse_delta[1] as i32,
        );
        let wheel_delta = io_source.mouse_wheel;

        let mut mouse_pressed = MouseSignal {
            left: false,
            right: false,
            wheel: false,
        };
        let mut mouse_signal = MouseSignal {
            left: false,
            right: false,
            wheel: false,
        };
        let mouse_on_screen = !io_source.want_capture_mouse;

        for i in 0..3 {
            *mouse_pressed.by_index(i) = io_source.mouse_down[i];
            let button = MouseSignal::button(i);
            if imgui_ctx.is_mouse_released(button) || imgui_ctx.is_mouse_clicked(button) {
                *mouse_signal.by_index(i) = true
            } else {
                *mouse_signal.by_index(i) = false
            }
        }

        let mut key_signal = HashSet::<KeyboardKey>::new();
        let mut key_pressed = HashSet::<KeyboardKey>::new();
        for i in 0..io_source.keys_down.len() {
            if (self.last_pressed_keys.get(&i).is_none() && io_source.keys_down[i])
                || imgui_ctx.is_key_index_released(i as i32)
            {
                key_signal.insert(i);
            }
        }
        self.last_pressed_keys.clear();
        for i in 0..io_source.keys_down.len() {
            if io_source.keys_down[i] {
                key_pressed.insert(i);
                self.last_pressed_keys.insert(i);
            }
        }

        self.image.replace(InputImage {
            mouse_pos,
            mouse_pos_delta,
            mouse_signal,
            mouse_pressed,
            wheel_delta,
            mouse_on_screen,
            key_signal,
            key_pressed,
        });
    }
}
