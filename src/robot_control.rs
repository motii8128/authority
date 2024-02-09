pub struct Vector
{
    x:f32,
    y:f32,
    rotation:f32
}

impl Vector {
    pub fn new()->Vector
    {
        Vector{
            x:0.0,
            y:0.0,
            rotation:0.0
        }
    }
}

pub struct Robot
{
    current_position:Vector,
    target_position:Vector
}

impl Robot {
    pub fn new()->Robot
    {
        let c = Vector{x:0.0, y:0.0, rotation:0.0};
        let t = Vector{x:0.0, y:0.0, rotation:0.0};
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
    pub fn create_vector(self)->Vector
    {
        Vector{
            x:self.target_position.x - self.current_position.x,
            y:self.target_position.y - self.current_position.y,
            rotation:self.target_position.rotation - self.current_position.rotation
        }
    }
}