use nannou::prelude::*; // Importa las funciones y estructuras necesarias de la biblioteca nannou.
use rand::Rng; // Importa el trait Rng de la biblioteca rand para generar números aleatorios.
use rayon::prelude::*; // Importa el trait ParallelIterator de la biblioteca rayon para paralelizar operaciones.

fn main() {
    // Inicializa la aplicación nannou con las funciones model, view y update.
    nannou::app(model).simple_window(view).update(update).run();
}

/// Estructura que representa una partícula.
struct Particle {
    /// Posición de la partícula en el espacio 2D.
    position: Vec2,
    /// Velocidad de la partícula.
    velocity: Vec2,
    /// Aceleración de la partícula.
    acceleration: Vec2,
    /// Color de la partícula.
    color: Rgba,
}

impl Particle {
    /// Crea una nueva partícula en la posición especificada.
    fn new(position: Vec2) -> Self {
        let mut rng = rand::thread_rng(); // Crea un generador de números aleatorios.
        Particle {
            position,
            // Asigna una velocidad aleatoria a la partícula.
            velocity: vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)),
            acceleration: Vec2::ZERO, // Inicializa la aceleración a cero.
            color: rgba(1.0, 1.0, 0.0, 1.0), // Inicializa el color a amarillo.
        }
    }

    /// Actualiza la posición y la velocidad de la partícula basándose en su aceleración.
    fn update(&mut self) {
        self.velocity += self.acceleration; // Aplica la aceleración a la velocidad.
        self.position += self.velocity; // Aplica la velocidad a la posición.
        self.acceleration = Vec2::ZERO; // Reinicia la aceleración a cero para el siguiente frame.
    }

    /// Aplica una fuerza a la partícula, modificando su aceleración.
    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force; // Suma la fuerza a la aceleración.
    }

    /// Actualiza el color de la partícula basándose en su distancia al mouse.
    fn update_color(&mut self, mouse_pos: Vec2) {
        // Calcula la distancia entre la partícula y el mouse.
        let distance = self.position.distance(mouse_pos);
        let max_distance = 200.0; // Define la distancia máxima para el cambio de color.
                                  // Normaliza la distancia para obtener un valor entre 0.0 y 1.0.
        let distance_normalized = (distance / max_distance).min(1.0);
        // Interpola el color entre amarillo y rojo basándose en la distancia normalizada.
        self.color = rgba(1.0, 1.0 - distance_normalized, 0.0, 1.0);
    }
}

/// Estructura que representa el modelo de la aplicación, conteniendo la lista de partículas.
struct Model {
    /// Vector que almacena las partículas.
    particles: Vec<Particle>,
}

/// Función que inicializa el modelo de la aplicación.
fn model(app: &App) -> Model {
    let mut particles = Vec::new(); // Crea un nuevo vector para almacenar las partículas.
    let window_rect = app.window_rect(); // Obtiene el rectángulo de la ventana.
    let mut rng = rand::thread_rng(); // Crea un nuevo generador de números aleatorios.
    let num_particles = 500; // Define el número de partículas a crear.

    // Crea las partículas y las añade al vector.
    for _i in 0..num_particles {
        let position = vec2(
            rng.gen_range(window_rect.left()..window_rect.right()), // Posición x aleatoria dentro de la ventana.
            rng.gen_range(window_rect.bottom()..window_rect.top()), // Posición y aleatoria dentro de la ventana.
        );
        let new_particle = Particle::new(position); // Crea una nueva partícula en la posición aleatoria.
        particles.push(new_particle); // Añade la partícula al vector.
    }

    Model { particles } // Retorna el modelo con la lista de partículas.
}

/// Función que actualiza el modelo en cada frame.
fn update(app: &App, model: &mut Model, _update: Update) {
    // Si el botón derecho del mouse está presionado, crea una nueva partícula en la posición del mouse.
    let mut rng = rand::thread_rng(); // Crea un generador de números aleatorios.
    if app.mouse.buttons.right().is_down() {
        let mouse_pos = app.mouse.position(); // Obtiene la posición del mouse.
        for _i in 1..rng.gen_range(1..40) {
            // cantidad de particulas a generar con el mause
            let new_particle = Particle::new(mouse_pos); // Crea una nueva partícula.
            model.particles.push(new_particle); // Añade la partícula al modelo.
        }
    }

    let gravity = vec2(0.0, -0.1); // Define la fuerza de gravedad.
    let window_rect = app.window_rect(); // Obtiene el rectángulo de la ventana.
    let mouse_pos = app.mouse.position(); // Obtiene la posición del mouse.

    // Aplica la gravedad, actualiza la posición y el color de cada partícula en paralelo.
    model.particles.par_iter_mut().for_each(|particle| {
        particle.apply_force(gravity); // Aplica la fuerza de gravedad a la partícula.
        particle.update(); // Actualiza la posición de la partícula.
        particle.update_color(mouse_pos); // Actualiza el color de la partícula.
    });

    // Elimina las partículas que están fuera de la ventana.
    model
        .particles
        .retain(|particle| window_rect.contains_point(particle.position.into()));
}

/// Función que dibuja el modelo en la ventana.
fn view(_app: &App, model: &Model, frame: Frame) {
    let draw = _app.draw(); // Crea un objeto Draw para dibujar en la ventana.
    draw.background().color(BLACK); // Establece el color de fondo a negro.

    // Dibuja cada partícula en la ventana.
    for particle in &model.particles {
        draw.ellipse() // Dibuja una elipse para representar la partícula.
            .xy(particle.position) // Establece la posición de la elipse.
            .radius(0.5) // Establece el radio de la elipse.
            .color(particle.color); // Establece el color de la elipse.
    }

    draw.to_frame(_app, &frame).unwrap(); // Escribe el dibujo en el frame.
}
