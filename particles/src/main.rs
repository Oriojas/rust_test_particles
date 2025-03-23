use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    color: Rgba,
}

// Implementación de Particle MODIFICADA para incluir aceleración y apply_force
impl Particle {
    fn new(position: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        Particle {
            position,
            velocity: vec2(rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)),
            acceleration: Vec2::ZERO,
            color: rgba(0.0, 0.0, 0.0, 1.0),
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }
}

// Model struct (sin cambios)
struct Model {
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    let mut particles = Vec::new();
    let window_rect = app.window_rect();
    let mut rng = rand::thread_rng();
    let num_particles = 500; // Definimos el número de partículas

    for _i in 0..num_particles {
        let position = vec2(
            rng.gen_range(window_rect.left()..window_rect.right()),
            rng.gen_range(window_rect.bottom()..window_rect.top()),
        );
        let new_particle = Particle::new(position);
        particles.push(new_particle);
    }

    Model { particles }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Descomenta esta sección si quieres seguir creando partículas con el ratón además de las 100000 iniciales

    if app.mouse.buttons.left().is_down() {
        let mouse_pos = app.mouse.position();
        let new_particle = Particle::new(mouse_pos);
        model.particles.push(new_particle);
    }

    let gravity = vec2(0.0, -0.1); // **VECTOR de gravedad hacia abajo**

    // ITERAR, APLICAR GRAVEDAD y ACTUALIZAR cada partícula
    for particle in &mut model.particles {
        particle.apply_force(gravity);
        particle.update();
    }
}

fn view(_app: &App, model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(WHITE);

    for particle in &model.particles {
        draw.ellipse()
            .xy(particle.position)
            .radius(1.0)
            .color(particle.color);
    }

    draw.to_frame(_app, &frame).unwrap();
}
