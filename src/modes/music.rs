use regex::Regex;
use std::str::FromStr;

use super::ModeTrait;
use crate::lightstrip;
use crate::music;

pub struct MusicMode {
    client: music::MusicClient,
    playlists: Vec<String>,
    current_playlist: usize,
}

impl MusicMode {
    pub fn create() -> anyhow::Result<MusicMode> {
        let mut client = music::new_client().expect("Error creating music client");

        let playlists = client
            .playlists()
            .unwrap()
            .iter()
            .map(|playlist| playlist.name.clone())
            .collect();
        // else: error handling?

        let current_playlist = 0;

        Ok(MusicMode {
            client,
            playlists,
            current_playlist,
        })
    }

    fn start_current_playlist(&mut self) -> anyhow::Result<()> {
        println!("playlist {}", self.current_playlist);
        let name = self.playlists[self.current_playlist].clone();
        println!("playlist {}", name);
        self.start_playlist(&name)?;
        Ok(())
    }

    fn start_playlist(&mut self, name: &String) -> anyhow::Result<()> {
        self.client.play_playlist(name)?;
        let colors = self.colors_for_playlist(name);
        println!("Loaded colors for playlist: {:?}", colors);
        lightstrip::full_flash_colors(colors);
        Ok(())
    }

    fn colors_for_playlist(&self, name: &String) -> Vec<lightstrip::Pixel> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
        }

        RE.find_iter(name)
            .filter_map(|mat| lightstrip::Pixel::from_str(mat.as_str()).ok())
            .collect()
    }
}

impl ModeTrait for MusicMode {
    fn setup(&mut self) -> anyhow::Result<()> {
        self.start_current_playlist()
    }

    fn teardown(&mut self) -> anyhow::Result<()> {
        self.client.stop()
    }

    fn right_blue_botton(&mut self) -> anyhow::Result<()> {
        self.client.next()
    }

    fn left_blue_button(&mut self) -> anyhow::Result<()> {
        self.client.prev()
    }

    fn red_button(&mut self) -> anyhow::Result<()> {
        self.current_playlist = (self.current_playlist + 1) % self.playlists.len();
        self.start_current_playlist()
    }

    fn green_button(&mut self) -> anyhow::Result<()> {
        self.current_playlist = self
            .current_playlist
            .checked_sub(1)
            .unwrap_or(self.playlists.len() - 1);
        self.start_current_playlist()
    }
}
