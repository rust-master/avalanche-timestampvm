pub async fn verify(&mut self) -> io::Result<()> {
    if self.height == 0 && self.parent_id == ids::Id::empty() {
        log::debug!(
            "block {} has an empty parent Id since it's a genesis block -- skipping verify",
            self.id
        );
        self.state.add_verified(&self.clone()).await;
        return Ok(());
    }

    // if already exists in database, it means it's already accepted
    // thus no need to verify once more
    if self.state.get_block(&self.id).await.is_ok() {
        log::debug!("block {} already verified", self.id);
        return Ok(());
    }

    let prnt_blk = self.state.get_block(&self.parent_id).await?;

    // ensure the height of the block is immediately following its parent
    if prnt_blk.height != self.height - 1 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "parent block height {} != current block height {} - 1",
                prnt_blk.height, self.height
            ),
        ));
    }

    // ensure block timestamp is after its parent
    if prnt_blk.timestamp > self.timestamp {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "parent block timestamp {} > current block timestamp {}",
                prnt_blk.timestamp, self.timestamp
            ),
        ));
    }

    // ensure block timestamp is no more than an hour ahead of this nodes time
    if self.timestamp >= (Utc::now() + Duration::hours(1)).timestamp() as u64 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "block timestamp {} is more than 1 hour ahead of local time",
                self.timestamp
            ),
        ));
    }

    // add newly verified block to memory
    self.state.add_verified(&self.clone()).await;
    Ok(())
}