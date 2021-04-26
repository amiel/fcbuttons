pub mod idle;
pub mod music;

pub trait ModeTrait {
    fn setup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn teardown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn red_button(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn left_blue_button(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn right_blue_botton(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn green_button(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
