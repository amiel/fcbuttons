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
        self.handle_errors(|conn| conn.stop())
    }

    pub fn next(&mut self) -> anyhow::Result<()> {
        self.handle_errors(|conn| conn.next())
    }

    pub fn prev(&mut self) -> anyhow::Result<()> {
        self.handle_errors(|conn| conn.prev())
    }

    // pub fn status(&mut self) -> Result<mpd::Status, mpd::error::Error> {
    //     self.conn.status()
    // }

    pub fn play_playlist(&mut self, name: &String) -> anyhow::Result<()> {
        self.handle_errors(|conn| {
            conn.pause(true)?;
            conn.clear()?;
            conn.load(name, ..)?;
            conn.play()
        })
    }

    pub fn playlists(&mut self) -> anyhow::Result<Vec<mpd::Playlist>> {
        self.handle_errors(|conn| conn.playlists())
    }

    fn handle_errors<F, T>(&mut self, f: F) -> anyhow::Result<T>
    where
        F: Fn(&mut Client<std::net::TcpStream>) -> mpd::error::Result<T>,
    {
        let result = f(&mut self.conn);

        if let Err(error) = result {
            println!("MPD Error: {}", error);
            println!("Trying to reconnect");
            self.attempt_reconnect()?;

            Ok(f(&mut self.conn)?)
        } else {
            Ok(result?)
        }
    }

    fn attempt_reconnect(&mut self) -> Result<(), mpd::error::Error> {
        self.conn = Client::connect("127.0.0.1:6600")?;

        Ok(())
    }
}
