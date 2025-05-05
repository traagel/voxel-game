use crate::particle::Particle;

pub fn update_particles(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        p.x += p.dx;
        p.y += p.dy;
        p.dy += 0.05;
        p.life = p.life.saturating_sub(1);
    }
    
    particles.retain(|p| p.life > 0);
} 