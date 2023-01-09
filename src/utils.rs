use async_std::io::{BufRead, Write};
use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::Unpin;

use crate::types::ChatResult;

pub async fn send_json<T: Write + Unpin, E: Serialize>(to: &mut T, body: &E) -> ChatResult<()> {
    let mut json = serde_json::to_string(&body)?;
    json.push('\n');
    to.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive_json<E: BufRead + Unpin, T: DeserializeOwned>(
    json_buff: E,
) -> impl Stream<Item = ChatResult<T>> {
    json_buff.lines().map(|line_result| -> ChatResult<T> {
        let line = line_result?;
        let t = serde_json::from_str::<T>(&line)?;
        Ok(t)
    })
}
