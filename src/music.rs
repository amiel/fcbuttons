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
        let result = self.conn.stop();
        self.handle_errors(result)
    }

    pub fn next(&mut self) -> anyhow::Result<()> {
        let result = self.conn.next();
        self.handle_errors(result)
    }

    pub fn prev(&mut self) -> anyhow::Result<()> {
        let result = self.conn.prev();
        self.handle_errors(result)
    }

    // pub fn status(&mut self) -> Result<mpd::Status, mpd::error::Error> {
    //     self.conn.status()
    // }

    pub fn play_playlist(&mut self, name: &String) -> anyhow::Result<()> {
        let result = self.conn.pause(true);
        self.handle_errors(result)?;

        let result = self.conn.clear();
        self.handle_errors(result)?;

        let result = self.conn.load(name, ..);
        self.handle_errors(result)?;

        let result = self.conn.play();
        self.handle_errors(result)
    }

    pub fn playlists(&mut self) -> anyhow::Result<Vec<mpd::Playlist>> {
        // Result<Vec<mpd::Playlist>, mpd::error::Error> {
        let result = self.conn.playlists();
        self.handle_errors(result)
    }

    fn handle_errors<T>(&mut self, result: mpd::error::Result<T>) -> anyhow::Result<T> {
        if let Err(error) = result {
            println!("MPD Error: {}", error);
            println!("Trying to reconnect");
            self.attempt_reconnect()?;

            Err(anyhow::anyhow!("Handled MPD Error: {}, try again", error))
        } else {
            Ok(result?)
        }
    }

    fn attempt_reconnect(&mut self) -> Result<(), mpd::error::Error> {
        self.conn = Client::connect("127.0.0.1:6600")?;

        Ok(())
    }
}
