#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        Self {
            init_position,
            init_velocity,
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.;
        let a = -9.8;
        let mut p_t_x = self.init_position.x + (self.init_velocity.x * self.time);
        let p_t_y = self.init_position.y
            + (self.init_velocity.y * self.time)
            + 0.5 * a * self.time * self.time;
        self.actual_position.x = round_1(p_t_x);
        self.actual_position.y = round_1(p_t_y);

        let v_t_x = self.init_velocity.x;
        let v_t_y = self.init_velocity.y + a * self.time;
        self.actual_velocity.x = round_1(v_t_x);
        self.actual_velocity.y = round_1(v_t_y);
        if self.actual_position.y <= 0. {
            return None;
        }

        Some(*self)
    }
}

fn round_1(num: f32) -> f32 {
    format!("{:.1}", num).parse().unwrap()
}
