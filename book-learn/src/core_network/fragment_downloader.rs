use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

const CONCURRENCY: usize = 8;

const CHUNK_SIZE: u64 = 1024 * 512;

#[tokio::main]
pub async fn download() -> Result< (), Box<dyn std::error::Error> > {
    let url = "https://releases.ubuntu.com/22.04/ubuntu-22.04.5-desktop-amd64.iso.zsync";
    let file_path = "downloaded_file.zsync";
    let client = reqwest::Client::new();

    let response = client.head(url).send().await ? ;
    let total_size = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|content_length| content_length.to_str().ok()?.parse::<u64>().ok())
        .expect("Unexpected content length of file");

    let pb = ProgressBar::new(total_size);
    // {msg} 占位符用于在最后显示信息
    // [elapsed_precise] 显示总耗时
    let pb_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({msg})") ?
        .progress_chars("#>-");
    pb.set_style(pb_style);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .await ? ;
    file.set_len(total_size).await ? ;

    let mutex = Mutex::new(file);
    let file_handle = Arc::new(mutex);

    // 切分任务
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < total_size {
        let end = min(start + CHUNK_SIZE - 1, total_size - 1);
        let temp = (start, end);
        chunks.push(temp);
        start += CHUNK_SIZE;
    }

    let pb_clone = pb.clone();

    futures::stream::iter(chunks)
        .map( |(start, end)| {
                let client = client.clone();
                let file_handle = Arc::clone(&file_handle);
                let pb = pb_clone.clone();

                async move {
                    let range_header = format!("bytes={}-{}", start, end);
                    let resp = client
                        .get(url)
                        .header(reqwest::header::RANGE, range_header)
                        .send()
                        .await ? ;
                    let data = resp.bytes().await ? ;

                    let chunk_len = data.len() as u64;

                    // 写入文件
                    let mut file = file_handle.lock().await;
                    let pos = std::io::SeekFrom::Start(start);
                    file.seek(pos).await ? ;
                    file.write_all(&data).await ? ;

                    // 更新进度条
                    pb.inc(chunk_len);

                    Ok::<(), Box< dyn std::error::Error + Send + Sync> >( () )
                }
            }
        )
        .buffer_unordered(CONCURRENCY)
        .for_each( |res| async move {
                if let Err(e) = res {
                    eprintln!("Task failed: {}", e);
                }
            }
        )
        .await;

    // 设置下载完成消息提示
    pb.set_message("Download Complete");
    println!("\n✨ 所有分片已合并！文件保存至: {}", file_path);

    Ok( () )
}
