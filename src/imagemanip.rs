use image;
use img_hash::HasherConfig;
use reqwest;
//use leptess::{leptonica, tesseract};
use rocket::serde::json::{json, Value};

async fn compare_images(one: &str, two: &str) -> Result<u32, reqwest::Error> {
    let img_bytes = reqwest::get(one).await?.bytes().await?;
    let img1 = image::load_from_memory(&img_bytes).unwrap();

    let img2_bytes = reqwest::get(two).await?.bytes().await?;
    let img2 = image::load_from_memory(&img2_bytes).unwrap();

    let hasher = HasherConfig::new().to_hasher();

    let hash1 = hasher.hash_image(&img1);
    let hash2 = hasher.hash_image(&img2);

    Ok(hash1.dist(&hash2))
}

// async fn process_ocr(url: &str) -> Result<Value, reqwest::Error> {
//     let img_bytes = reqwest::get(url).await?.bytes().await?;

//     let mut api = tesseract::TessApi::new(None, "eng").unwrap();

//     let pix = leptonica::pix_read_mem(&img_bytes).unwrap();

//     api.set_image(&pix);

//     Ok(match api.get_utf8_text() {
//         Ok(text) => json!({"status": "success", "text": text}),
//         Err(err) => json!({"status": "error", "text":"", "message":err.to_string()})
//     })
// }

#[get("/<urione>/<uritwo>")]
async fn compare(urione: &str, uritwo: &str) -> String {
    println!("{:?} - {:?}", urione, uritwo);
    let result = compare_images(urione, uritwo).await;

    match result {
        Ok(r) => {
            format!("{}", r)
        }
        Err(e) => {
            format!("Err: {:?}", e)
        }
    }
}

// #[get("/<url>")]
// async fn ocr(url: &str) -> Value {
//     match process_ocr(url).await {
//         Ok(v) => v,
//         Err(err) => json!({"status": "error", "message": err.to_string(), "text":""})
//     }
// }

#[catch(500)]
fn download_error() -> Value {
    json!({
        "status": "error",
        "reason": "Invalid image link"
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("image", |rocket| async {
        rocket
            .mount("/compare", routes![compare])
            //.mount("/ocr", routes![ocr])
            .register("/compare", catchers![download_error])
    })
}
