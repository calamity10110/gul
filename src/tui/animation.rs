// GUL TUI Animation Module
// Harmonica-inspired physics-based animations
// Inspired by charmbracelet/harmonica

use std::time::{Duration, Instant};

/// Spring physics configuration
#[derive(Debug, Clone, Copy)]
pub struct SpringConfig {
    /// Frequency in Hz (how fast the spring oscillates)
    pub frequency: f64,
    /// Damping ratio (0 = undamped, 1 = critically damped)
    pub damping: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        SpringConfig {
            frequency: 5.0,
            damping: 0.5,
        }
    }
}

impl SpringConfig {
    /// Snappy preset - fast and responsive
    pub fn snappy() -> Self {
        SpringConfig {
            frequency: 8.0,
            damping: 0.7,
        }
    }

    /// Smooth preset - gentle motion
    pub fn smooth() -> Self {
        SpringConfig {
            frequency: 3.0,
            damping: 0.6,
        }
    }

    /// Bouncy preset - playful oscillation
    pub fn bouncy() -> Self {
        SpringConfig {
            frequency: 6.0,
            damping: 0.3,
        }
    }

    /// Stiff preset - minimal overshoot
    pub fn stiff() -> Self {
        SpringConfig {
            frequency: 10.0,
            damping: 0.9,
        }
    }
}

/// Spring animation state
#[derive(Debug, Clone)]
pub struct Spring {
    /// Current value
    pub value: f64,
    /// Target value
    pub target: f64,
    /// Velocity
    velocity: f64,
    /// Configuration
    config: SpringConfig,
    /// Last update time
    last_update: Instant,
}

impl Spring {
    pub fn new(initial: f64, config: SpringConfig) -> Self {
        Spring {
            value: initial,
            target: initial,
            velocity: 0.0,
            config,
            last_update: Instant::now(),
        }
    }

    /// Set new target value
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Update spring physics
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;

        if dt > 0.1 {
            // Too much time passed, snap to target
            self.value = self.target;
            self.velocity = 0.0;
            return false;
        }

        // Spring physics
        let omega = 2.0 * std::f64::consts::PI * self.config.frequency;
        let damping_coeff = 2.0 * self.config.damping * omega;

        let displacement = self.value - self.target;
        let spring_force = -omega * omega * displacement;
        let damping_force = -damping_coeff * self.velocity;
        let acceleration = spring_force + damping_force;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        // Check if settled
        let is_moving = displacement.abs() > 0.001 || self.velocity.abs() > 0.001;

        if !is_moving {
            self.value = self.target;
            self.velocity = 0.0;
        }

        is_moving
    }

    /// Get current value as integer
    pub fn value_i32(&self) -> i32 {
        self.value.round() as i32
    }

    /// Get current value as u16
    pub fn value_u16(&self) -> u16 {
        self.value.round().max(0.0) as u16
    }

    /// Check if animation is complete
    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < 0.001 && self.velocity.abs() < 0.001
    }
}

/// 2D spring for position animation
#[derive(Debug, Clone)]
pub struct Spring2D {
    pub x: Spring,
    pub y: Spring,
}

impl Spring2D {
    pub fn new(x: f64, y: f64, config: SpringConfig) -> Self {
        Spring2D {
            x: Spring::new(x, config),
            y: Spring::new(y, config),
        }
    }

    pub fn set_target(&mut self, x: f64, y: f64) {
        self.x.set_target(x);
        self.y.set_target(y);
    }

    pub fn update(&mut self) -> bool {
        let x_moving = self.x.update();
        let y_moving = self.y.update();
        x_moving || y_moving
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x.value_i32(), self.y.value_i32())
    }

    pub fn position_u16(&self) -> (u16, u16) {
        (self.x.value_u16(), self.y.value_u16())
    }
}

/// Easing functions for non-spring animations
#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

impl Easing {
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t).powi(2),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                let mut t = t;

                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    t -= 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    t -= 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    t -= 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            Easing::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    2.0f64.powf(-10.0 * t) * ((t - s) * 2.0 * std::f64::consts::PI / p).sin() + 1.0
                }
            }
        }
    }
}

/// Tween animation
#[derive(Debug, Clone)]
pub struct Tween {
    from: f64,
    to: f64,
    duration: Duration,
    easing: Easing,
    start_time: Option<Instant>,
}

impl Tween {
    pub fn new(from: f64, to: f64, duration: Duration, easing: Easing) -> Self {
        Tween {
            from,
            to,
            duration,
            easing,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn value(&self) -> f64 {
        match self.start_time {
            None => self.from,
            Some(start) => {
                let elapsed = start.elapsed();
                if elapsed >= self.duration {
                    self.to
                } else {
                    let t = elapsed.as_secs_f64() / self.duration.as_secs_f64();
                    let eased_t = self.easing.apply(t);
                    self.from + (self.to - self.from) * eased_t
                }
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        match self.start_time {
            None => false,
            Some(start) => start.elapsed() >= self.duration,
        }
    }
}

/// Scroll animation state
#[derive(Debug, Clone)]
pub struct ScrollAnimation {
    spring: Spring,
}

impl ScrollAnimation {
    pub fn new(initial: f64) -> Self {
        ScrollAnimation {
            spring: Spring::new(initial, SpringConfig::smooth()),
        }
    }

    pub fn scroll_to(&mut self, target: f64) {
        self.spring.set_target(target);
    }

    pub fn scroll_by(&mut self, delta: f64) {
        self.spring.set_target(self.spring.target + delta);
    }

    pub fn update(&mut self) -> bool {
        self.spring.update()
    }

    pub fn offset(&self) -> usize {
        self.spring.value.max(0.0).round() as usize
    }
}

/// Cursor animation for smooth cursor movement
#[derive(Debug, Clone)]
pub struct CursorAnimation {
    position: Spring2D,
}

impl CursorAnimation {
    pub fn new(x: u16, y: u16) -> Self {
        CursorAnimation {
            position: Spring2D::new(x as f64, y as f64, SpringConfig::snappy()),
        }
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.position.set_target(x as f64, y as f64);
    }

    pub fn update(&mut self) -> bool {
        self.position.update()
    }

    pub fn position(&self) -> (u16, u16) {
        self.position.position_u16()
    }
}

/// Fade animation for opacity
#[derive(Debug, Clone)]
pub struct FadeAnimation {
    tween: Tween,
}

impl FadeAnimation {
    pub fn fade_in(duration: Duration) -> Self {
        FadeAnimation {
            tween: Tween::new(0.0, 1.0, duration, Easing::EaseOut),
        }
    }

    pub fn fade_out(duration: Duration) -> Self {
        FadeAnimation {
            tween: Tween::new(1.0, 0.0, duration, Easing::EaseIn),
        }
    }

    pub fn start(&mut self) {
        self.tween.start();
    }

    pub fn opacity(&self) -> f64 {
        self.tween.value()
    }

    pub fn is_complete(&self) -> bool {
        self.tween.is_complete()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_config() {
        let snappy = SpringConfig::snappy();
        assert_eq!(snappy.frequency, 8.0);

        let smooth = SpringConfig::smooth();
        assert_eq!(smooth.frequency, 3.0);
    }

    #[test]
    fn test_spring() {
        let mut spring = Spring::new(0.0, SpringConfig::default());
        spring.set_target(100.0);

        assert!(!spring.is_settled());
    }

    #[test]
    fn test_easing() {
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert!(Easing::EaseIn.apply(0.5) < 0.5);
        assert!(Easing::EaseOut.apply(0.5) > 0.5);
    }

    #[test]
    fn test_tween() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100), Easing::Linear);
        assert_eq!(tween.value(), 0.0);

        tween.start();
        assert!(!tween.is_complete());
    }
}
