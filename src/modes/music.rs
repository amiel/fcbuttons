use regex::Regex;
use std::str::FromStr;

use super::ModeTrait;
use crate::lightstrip;
use crate::music;

pub struct MusicMode {
    client: music::MusicClient,
    playlists: Vec<String>,
    lightstrip: lightstrip::Sender,
    current_playlist: usize,
}

impl MusicMode {
    pub fn create(lightstrip: &lightstrip::Sender) -> anyhow::Result<MusicMode> {
        let mut client = music::new_client().expect("Error creating music client");

        let playlists = client
            .playlists()
            .expect("Error loading playlists")
            .iter()
            .map(|playlist| playlist.name.clone())
            .collect();
        // else: error handling?

        let current_playlist = 0;
        let lightstrip = lightstrip.clone();

        Ok(MusicMode {
            client,
            playlists,
            lightstrip,
            current_playlist,
        })
    }

    fn start_current_playlist(&mut self) -> anyhow::Result<()> {
        let name = self.playlists[self.current_playlist].clone();
        println!("starting playlist {}", name);
        self.start_playlist(&name)?;
        Ok(())
    }

    fn start_playlist(&mut self, name: &String) -> anyhow::Result<()> {
        self.client.play_playlist(name)?;
        let colors = self.colors_for_playlist(name);
        println!("Loaded colors for playlist: {:?}", colors);

        for color in colors {
            lightstrip::flash(&self.lightstrip, color)?;
        }

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
