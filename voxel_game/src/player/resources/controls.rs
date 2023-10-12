use bevy::prelude::*;

pub struct MouseConfig {
  pub mouse_sensitivity: f32,
  pub invert_look: bool,
}

impl Default for MouseConfig {
  fn default() -> Self {
    Self {
      mouse_sensitivity: 1.,
      invert_look: false,
    }
  }
}

pub struct Bindings {
  pub move_forward: Binding,
  pub move_backward: Binding,
  pub move_left: Binding,
  pub move_right: Binding,
  pub move_up: Binding,
  pub move_down: Binding,
  pub cursor_lock: Binding,
}

impl Bindings {
  pub fn movement_forward_axis(&self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) -> f32 {
    Self::axis_value(keyboard_input, mouse_button_input, &self.move_forward, &self.move_backward)
  }

  pub fn movement_strafe_axis(&self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) -> f32 {
    Self::axis_value(keyboard_input, mouse_button_input, &self.move_right, &self.move_left)
  }

  pub fn movement_up_axis(&self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) -> f32 {
    Self::axis_value(keyboard_input, mouse_button_input, &self.move_up, &self.move_down)
  }

  pub fn movement_axes(&self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) -> Vec3 {
    Vec3::new(
      self.movement_forward_axis(keyboard_input, mouse_button_input),
      self.movement_up_axis(keyboard_input, mouse_button_input),
      self.movement_strafe_axis(keyboard_input, mouse_button_input),
    )
  }

  fn axis_value(keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>, plus: &Binding, minus: &Binding) -> f32 {
    let mut value = 0.;
    if plus.was_activated(keyboard_input, mouse_button_input) {
      value += 1.;
    }

    if minus.was_activated(keyboard_input, mouse_button_input) {
      value += -1.;
    }

    value
  }
}

impl Default for Bindings {
  fn default() -> Self {
    Self { 
      move_forward: Binding::new(Some(KeyCode::E), None), 
      move_backward: Binding::new(Some(KeyCode::D), None), 
      move_left: Binding::new(Some(KeyCode::S), None), 
      move_right: Binding::new(Some(KeyCode::F), None), 
      move_up: Binding::new(Some(KeyCode::Space), None), 
      move_down: Binding::new(Some(KeyCode::Capital), None), 
      cursor_lock: Binding::new(Some(KeyCode::Escape), None),
    }
  }
}

pub struct Binding {
  keyboard: Option<KeyCode>,
  mouse: Option<MouseButton>,
}

impl Binding {
  pub fn new(keyboard: Option<KeyCode>, mouse: Option<MouseButton>) -> Self {
    Self { 
      keyboard, 
      mouse,
    }
  }

  pub fn was_activated(&self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) -> bool {
    if let Some(key) = self.keyboard {
      return keyboard_input.pressed(key);
    } 
    
    if let Some(button) = self.mouse {
      return mouse_button_input.pressed(button);
    }

    false
  }

  pub fn was_just_activated(&self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) -> bool {
    if let Some(key) = self.keyboard {
      return keyboard_input.just_pressed(key);
    } 
    
    if let Some(button) = self.mouse {
      return mouse_button_input.just_pressed(button);
    }

    false
  }
}