mod media;
use media::audio::Audio;
use media::video::Video;

fn main(){
    let audio = Audio{
        title: "somebody I used to know".to_string(),
        artist: " gummy monster".to_string()
    };


    let video = Video{
        title: " Black Mirror".to_string(),
        resolution: (1920, 1080)
    };

    audio.play();
    video.play();
}