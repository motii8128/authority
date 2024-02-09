struct Position
{
    x:f32,
    y:f32,
    rotation:f32
}

pub struct Robot
{
    current_position:Position,
    target_position:Position
}

impl Robot {
    pub fn new()->Robot
    {
        let c = Position{x:0.0, y:0.0, rotation:0.0};
        let t = Position{x:0.0, y:0.0, rotation:0.0};
        Robot{
            current_position:c,
            target_position:t
        }
    }

    pub fn add_vector(&mut self,x:f32, y:f32, rotation:f32)
    {
        self.target_position.x += x;
        self.target_position.y += y;
        self.target_position.rotation += rotation;
    }
}