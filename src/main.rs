use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use vector::Vector;

fn main() {
    println!("Hello, world!");
    let window = Window::new_centered("Rusty Pendulum", (800, 600)).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();

        self.p.draw(graphics);

        helper.request_redraw()
    }

    // If desired, on_mouse_move(), on_key_down(), etc...
}

struct Pendulum {
    // This vector is the position of the pendulum
    origin: Vector,

    // This vector is the position of the ball
    position: Vector,

    // Angle of the pendulum
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, // Length of the Pendulum
    g: f32, // Gravity
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            // We need to set the origin of the pendulum
            origin: Vector::new(x, y),

            // We'll set the position when we update the pendulum
            // For now we'll set it to a default value
            position: Vector::new(0.0, 0.0),

            angle: 1.0,                // We'll set the angle to 1.0 radian
            angular_velocity: 0.0,     // The pendulum is not moving in the beginning
            angular_acceleration: 0.0, // The pendulum is not acceleration in the beginning

            r,
            g: 0.95, // The gravity is 0.5 for this example, but play with it
        }
    }

    fn update(&mut self) {
        // We use the pendulum equation to  calculate the angular acceleration
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        // The angular velocity is the angular velocity plus the angular acceleration
        self.angular_velocity += self.angular_acceleration;

        // The angle is the angle plis the angular velocity
        self.angle += self.angular_velocity;

        // The position is the polar coordinates translated to cartesian coordiantes
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        // The final position of the ball is the canvas
        // pendulum plis the position vector
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        // We need to draw the line of the pendulum first
        // It takes the start and end position of the line, the width of the line and the color
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector {

    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
