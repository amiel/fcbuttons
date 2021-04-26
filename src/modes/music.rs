use super::ModeTrait;

use crate::music;

pub struct MusicMode {
    client: music::MusicClient,
    playlists: Vec<String>,
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

        Ok(MusicMode { client, playlists })
    }
}

impl ModeTrait for MusicMode {
    fn setup(&mut self) -> anyhow::Result<()> {
        self.client.play_playlist(self.playlists[0].clone())?;
        Ok(())
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
}
