pub struct Video {
    pub title: String,
    pub resolution: (u32,  u32)
}

impl Video{
    pub fn play(&self){
        println! (" Now playing: {} with a resolution of ({}x{})", self.title, self.resolution.0, self.resolution.1);
    }
}