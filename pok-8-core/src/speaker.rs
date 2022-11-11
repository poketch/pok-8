use playback_rs::{Player, Song};


pub struct Buzzer {

    player: Player,
    buzz: Song, 

}

impl Buzzer {

    pub fn init() -> Self {

        let ply = Player::new().expect("Failed to open an audio output.");
        let sng = Song::from_file("./pok-8-core/buzz.wav").expect("Failed to load buzz from file");

        Self {

            player: ply,
            buzz: sng,

         }
    }

    pub fn play(&self) -> () {

        self.player.play_song_next(&self.buzz).expect("Failed to play the buzz.");

    }






}