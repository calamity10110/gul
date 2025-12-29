pub mod svg {
    use svg::node::element::Circle;
    
    pub fn create_circle(cx: i32, cy: i32, r: i32) -> Circle {
        Circle::new()
            .set("cx", cx)
            .set("cy", cy)
            .set("r", r)
    }
}

pub mod charts {
    pub struct Chart;
}

pub mod canvas {
    pub struct Canvas;
}
