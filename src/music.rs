use mpd::Client;

pub struct MusicClient {
    conn: Client<std::net::TcpStream>,
}

pub fn new_client() -> anyhow::Result<MusicClient> {
    let mut conn = Client::connect("127.0.0.1:6600")?;
    conn.volume(100)?;

    Ok(MusicClient { conn })
}

impl MusicClient {
    pub fn stop(&mut self) -> anyhow::Result<()> {
        Ok(self.conn.stop()?)
    }

    pub fn next(&mut self) -> anyhow::Result<()> {
        Ok(self.conn.next()?)
    }

    pub fn status(&mut self) -> Result<mpd::Status, mpd::error::Error> {
        self.conn.status()
    }

    pub fn play_playlist(&mut self, name: String) -> anyhow::Result<()> {
        self.conn.pause(true)?;
        self.conn.clear()?;
        self.conn.load(name, ..)?;
        Ok(self.conn.play()?)
    }

    pub fn playlists(&mut self) -> Result<Vec<mpd::Playlist>, mpd::error::Error> {
        self.conn.playlists()
    }
}
