use reqwest::Client;

#[tauri::command]
async fn fetch_html(character: String, font: String) -> Result<String, String> {
    let client = Client::new();

    // 构造请求 URL
    let url = format!("https://www.sftj.com/{}/{}/", character, font);

    // 发起请求并返回 HTML 文本
    match client.get(&url).send().await {
        Ok(response) => {
            let text = response.text().await.map_err(|err| err.to_string())?;
            Ok(text)
        }
        Err(err) => Err(err.to_string()),
    }
}
#[tauri::command]
async fn fetch_image_urls(p: String, m: String, gr_cookie: String) -> Result<String, String> {
    let client = Client::new();

    // 构造请求 URL
    let url = format!("https://www.sftj.com/getpic.php?p={}&m={}", p, m);

    // 发起请求并返回 HTML 文本
    match client
        .get(&url)
        .header("Referer", "https://www.sftj.com/") // 添加 Referer
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0") // 添加 User-Agent
        .header("Cookie", format!("gr={}", gr_cookie)) // 添加 gr Cookie
        .send()
        .await
    {
        Ok(response) => {
            let text = response.text().await.map_err(|err| err.to_string())?;
            println!("Response: {}", text); // 打印响应内容
            Ok(text)
        }
        Err(err) => {
            println!("Request failed: {}", err); // 打印错误信息
            Err(err.to_string())
        }
    }
}

#[tauri::command]
async fn fetch_image(image_name: String) -> Result<Vec<u8>, String> {
    let client = Client::new();

    // 组装完整的图片 URL
    let image_url = format!("https://pic.guoxuemi.com:446/sf/{}", image_name);

    // 发起请求并返回图片数据
    match client
        .get(&image_url)
        .header("Referer", "https://www.sftj.com/") // 添加 Referer
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0") // 添加 User-Agent
        .send()
        .await
    {
        Ok(response) => {
            let bytes = response.bytes().await.map_err(|err| err.to_string())?;
            Ok(bytes.to_vec()) // 返回二进制数据
        }
        Err(err) => {
            println!("Request failed: {}", err); // 打印错误信息
            Err(err.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_html, fetch_image_urls, fetch_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}