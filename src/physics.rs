use glam::Vec2;

pub struct Object {
    pub pos: Vec2,
    prev_pos: Vec2,
    acc: Vec2,

    pub radius: f32,
}

impl Object {
    pub fn new_at_origin(radius: f32) -> Self {
        Self {
            pos: Vec2::ZERO,
            prev_pos: Vec2::ZERO,
            acc: Vec2::ZERO,
            radius,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let vel = self.pos - self.prev_pos;

        // save current position
        self.prev_pos = self.pos;

        // perform Verlet integration
        self.pos = self.pos + vel + self.acc * dt * dt;

        // reset acceleration
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }
}

pub struct Solver {
    pub objects: Vec<Object>,
    pub gravity: Vec2,
}

impl Solver {
    pub fn update(&mut self, dt: f32) {
        // apply gravity
        for obj in &mut self.objects {
            obj.accelerate(self.gravity);
        }

        self.apply_wall_constraint();

        // update positions
        for obj in &mut self.objects {
            obj.update_position(dt);
        }
    }

    fn apply_wall_constraint(&mut self) {
        let width: u16 = 800;
        let height: u16 = 750;

        for obj in &mut self.objects {
            // check left wall
            let wall_pos = -(width as f32) / 2.0;
            if (obj.pos.x - obj.radius) < wall_pos {
                obj.pos.x = wall_pos + obj.radius;
            }

            // check right wall
            let wall_pos = (width as f32) / 2.0;
            if (obj.pos.x + obj.radius) > wall_pos {
                obj.pos.x = wall_pos - obj.radius;
            }

            // check bottom wall
            let wall_pos = -(height as f32) / 2.0;
            if (obj.pos.y - obj.radius) < wall_pos {
                obj.pos.y = wall_pos + obj.radius;
            }

            // check top wall
            let wall_pos = (height as f32) / 2.0;
            if (obj.pos.y + obj.radius) > wall_pos {
                obj.pos.y = wall_pos - obj.radius;
            }
        }
    }

    // TODO
    fn collisions(&mut self) {}
}
