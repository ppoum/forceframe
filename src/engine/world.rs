use crate::engine::engine_object::EngineObject;

pub struct World<'a> {
    width: u32,
    height: u32,
    fb: &'a mut Vec<u32>,
    objects: Vec<Box<dyn EngineObject>>,
}

impl<'a> World<'a> {
    pub fn new(width: u32, height: u32, fb: &'a mut Vec<u32>) -> Self {
        World { width, height, fb: fb.as_mut(), objects: Vec::new() }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        assert!(x < self.width);
        assert!(y < self.height);
        let idx = y * self.width + x;
        self.fb[idx as usize] = color;
    }

    pub fn add_object(&mut self, object: Box<dyn EngineObject>) {
        // self.objects.push(Box::new(object.clone()));
        self.objects.push(object);
    }
}