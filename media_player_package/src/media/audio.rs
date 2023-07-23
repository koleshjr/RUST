pub struct Audio{
    pub title: String,
    pub artist: String,

}

impl Audio {
    pub fn play(&self){
        println! ("Now playing: {} by {}", self.title, self.artist)
    }
}