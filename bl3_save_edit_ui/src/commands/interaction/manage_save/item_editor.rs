use anyhow::Result;

pub async fn open_website(url: String) -> Result<()> {
    open::that(url).map_err(anyhow::Error::new)
}
