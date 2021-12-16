use bevy::ecs::entity::Entities;
use bevy::ecs::query::ReadOnlyFetch;
use crate::prelude::*;

pub enum BallDirection {
    None, Top, Left, Right, Bottom
}

//==== Ball ========================================================================================

pub struct Ball {
    pub position: Position,
    pub radius: f32,
    pub sticking_on_paddle: bool
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            position: Position::default(),
            radius: 20.0,
            sticking_on_paddle: true
        }
    }
}

//==== BallCalculations=============================================================================

pub struct BallCalculations {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub nx: f32,
    pub ny: f32,
    pub d: BallDirection
}

impl Default for BallCalculations {
    fn default() -> Self {
        Self {
            x: 0., y: 0., dx: 0., dy: 0., nx: 0., ny: 0., d: BallDirection::None
        }
    }
}

impl BallCalculations {
    pub fn clear(&mut self) {
        self.x  = 0.;
        self.y  = 0.;
        self.dx = 0.;
        self.dy = 0.;
        self.nx = 0.;
        self.ny = 0.;
        self.d  = BallDirection::None;
    }

    /// Calculates the new ball position.
    /// * `x` - test
    /// * `y` - hello
    /// * `dx` -
    /// * `dy` -
    /// * `dt` -
    pub fn calc_position(&mut self, x: f32, y: f32, dx: f32, dy: f32, dt: f32) {
        let nx = dx * dt;
        let ny = dy * dt;
        self.x = x + nx;
        self.y = y + ny;
        self.dx = dx;
        self.dy = dy;
        self.nx = nx;
        self.ny = ny;
    }

    /// Calculates next position an speed of the ball.
    /// * `x` -
    /// * `y` -
    /// * `dx` -
    /// * `dy` -
    /// * `accel` -
    /// * `dt` -
    pub fn calc_accelerate(&mut self, x: f32, y: f32, dx: f32, dy: f32, accel: f32, dt: f32) {
        let x2  = x + (dt * dx) + (accel * dt * dt * 0.5);
        let y2  = y + (dt * dy) + (accel * dt * dt * 0.5);
        let dx2 = if dx > 0. {
            dx + (accel * dt) * 1.
        } else {
            dx + (accel * dt) * -1.
        };
        let dy2 = if dy > 0. {
            dy + (accel * dt) * 1.
        } else {
            dy + (accel * dt) * -1.
        };
        self.x = x2;
        self.y = y2;
        self.dx = dx2;
        self.dy = dy2;
        self.nx = (x2 - x);
        self.ny = (y2 - y);
    }

    /// Calculates the interception of 2 line segments.
    pub fn calc_intercept(&mut self, x1: f32, y1: f32, x2: f32, y2: f32,
                          x3: f32, y3: f32, x4: f32, y4: f32, d: BallDirection) -> bool {
        let denom = ((y4-y3) * (x2-x1)) - ((x4-x3) * (y2-y1));
        if denom != 0. {
            let ua = (((x4-x3) * (y1-y3)) - ((y4-y3) * (x1-x3))) / denom;
            if (ua >= 0.) && (ua <= 1.) {
                let ub = (((x2-x1) * (y1-y3)) - ((y2-y1) * (x1-x3))) / denom;
                if (ub >= 0.) && (ub <= 1.) {
                    let x = x1 + (ua * (x2-x1));
                    let y = y1 + (ua * (y2-y1));
                    self.x = x;
                    self.y = y;
                    self.d = d;
                    return true;
                }
            }
        }
        false
    }

    /// Calculates the interception of ball and rectangle.
    pub fn calc_ball_intercept(&mut self, ball: Ball, rect: Rect<f32>, nx: f32, ny: f32) -> bool {
        let ball_x = ball.position.x;
        let ball_y = ball.position.y;

        if nx < 0. {
            self.calc_intercept(ball_x, ball_y, ball_x + nx, ball_y + ny,
                                rect.right  + ball.radius,
                                rect.top    - ball.radius,
                                rect.right  + ball.radius,
                                rect.bottom + ball.radius,
                                BallDirection::Right);
            return true;
        }

        if nx > 0. {
            self.calc_intercept(ball_x, ball_y, ball_x + nx, ball_y + ny,
                                rect.left   - ball.radius,
                                rect.top    - ball.radius,
                                rect.left   - ball.radius,
                                rect.bottom + ball.radius,
                                BallDirection::Left);
            return true;
        }

        if ny < 0. {
            self.calc_intercept(ball_x, ball_y, ball_x + nx, ball_y + ny,
                                rect.left   - ball.radius,
                                rect.bottom + ball.radius,
                                rect.right  + ball.radius,
                                rect.bottom + ball.radius,
                                BallDirection::Bottom);
            return true;
        }

        if ny > 0. {
            self.calc_intercept(ball_x, ball_y, ball_x + nx, ball_y + ny,
                                rect.left   - ball.radius,
                                rect.top    - ball.radius,
                                rect.right  + ball.radius,
                                rect.top    - ball.radius,
                                BallDirection::Top);
            return true;
        }

        false
    }
}

//==== PaddleMovingDirection =======================================================================

pub enum PaddleMovingDirection {
    None, Left, Right
}

//==== Paddle ======================================================================================

pub struct Paddle {
    pub moving_direction : PaddleMovingDirection
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            moving_direction : PaddleMovingDirection::None
        }
    }
}

//==== WindowSize ==================================================================================

pub struct WindowSize {
    pub width: f32,
    pub height: f32
}

//==== Position ====================================================================================

pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
        }
    }
}

//==== MoveSpeed ===================================================================================

pub struct MoveSpeed(pub f32);

impl MoveSpeed {
    pub fn new(speed: f32) -> Self { Self(speed) }
}

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(500.)
    }
}
