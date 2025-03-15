use reqwest::Client;

#[tauri::command]
async fn fetch_html(character: String, font: String) -> Result<String, String> {
    let client = Client::new();

    let url = format!("https://www.sftj.com/{}/{}/", character, font);

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

    let url = format!("https://www.sftj.com/getpic.php?p={}&m={}", p, m);

    match client
        .get(&url)
        .header("Referer", "https://www.sftj.com/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0")
        .header("Cookie", format!("gr={}", gr_cookie))
        .send()
        .await
    {
        Ok(response) => {
            let text = response.text().await.map_err(|err| err.to_string())?;
            Ok(text)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn fetch_image(image_name: String) -> Result<Vec<u8>, String> {
    let client = Client::new();

    let image_url = format!("https://pic.guoxuemi.com:446/sf/{}", image_name);

    match client
        .get(&image_url)
        .header("Referer", "https://www.sftj.com/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0")
        .send()
        .await
    {
        Ok(response) => {
            let bytes = response.bytes().await.map_err(|err| err.to_string())?;
            Ok(bytes.to_vec())
        }
        Err(err) => Err(err.to_string()),
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