use std::fs;
use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};

use crate::api_structs::items::ItemApiResponse;

pub fn get_pb(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);

    let pb_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec})")
        .unwrap()
        .progress_chars("#>-");
    pb.set_style(pb_style);
    pb.tick();

    pb
}

pub fn unneeded_values(all_items: &[ItemApiResponse]) {
    // pub factory_description: Option<Value>,
    // pub order: Option<Value>,
    // pub tracks: Option<Value>,

    let files_where_factory_description_is_some = all_items
        .iter()
        .filter(|root| root.factory_description.is_some())
        .collect::<Vec<_>>();

    if !files_where_factory_description_is_some.is_empty() {
        let mut factory_description_file = File::create("factory_description.json").unwrap();
        let mut factory_description_json = files_where_factory_description_is_some
            .iter()
            .map(|root| serde_json::to_string_pretty(root).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        factory_description_json.insert(0, '[');
        factory_description_json.push(']');
        factory_description_file
            .write_all(factory_description_json.as_bytes())
            .unwrap();
    }

    let files_where_order_is_some = all_items
        .iter()
        .filter(|root| root.order.is_some())
        .collect::<Vec<_>>();

    if !files_where_order_is_some.is_empty() {
        let mut order_file = File::create("order.json").unwrap();
        let mut order_json = files_where_order_is_some
            .iter()
            .map(|root| serde_json::to_string_pretty(root).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        order_json.insert(0, '[');
        order_json.push(']');
        order_file.write_all(order_json.as_bytes()).unwrap();
    }

    let files_where_tracks_is_some = all_items
        .iter()
        .filter(|root| root.tracks.is_some())
        .collect::<Vec<_>>();

    if !files_where_tracks_is_some.is_empty() {
        let mut tracks_file = File::create("tracks.json").unwrap();
        let mut tracks_json = files_where_tracks_is_some
            .iter()
            .map(|root| serde_json::to_string_pretty(root).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        tracks_json.insert(0, '[');
        tracks_json.push(']');
        tracks_file.write_all(tracks_json.as_bytes()).unwrap();
    }

    // // buyee_variations: Vec<BuyeeVariation>,
    // BuyeeVariation {
    //    pub downloadable: Option<Value>,
    //    pub factory_image_url: Option<Value>,
    //    pub order_url: Option<Value>,
    // }

    let files_where_buyee_variations_is_not_empty = all_items
        .iter()
        .filter(|root| !root.buyee_variations.is_empty())
        .flat_map(|root| root.buyee_variations.clone())
        .collect::<Vec<_>>();

    let downloadables_in_buyee_variations = files_where_buyee_variations_is_not_empty
        .iter()
        .filter(|buyee_variation| buyee_variation.downloadable.is_some())
        .collect::<Vec<_>>();

    if !downloadables_in_buyee_variations.is_empty() {
        let mut downloadables_in_buyee_variations_file =
            File::create("downloadables_in_buyee_variations.json").unwrap();
        let mut downloadables_in_buyee_variations_json = downloadables_in_buyee_variations
            .iter()
            .map(|buyee_variation| serde_json::to_string_pretty(buyee_variation).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        downloadables_in_buyee_variations_json.insert(0, '[');
        downloadables_in_buyee_variations_json.push(']');
        downloadables_in_buyee_variations_file
            .write_all(downloadables_in_buyee_variations_json.as_bytes())
            .unwrap();
    }

    let factory_image_urls_in_buyee_variations = files_where_buyee_variations_is_not_empty
        .iter()
        .filter(|buyee_variation| buyee_variation.factory_image_url.is_some())
        .collect::<Vec<_>>();

    if !factory_image_urls_in_buyee_variations.is_empty() {
        let mut factory_image_urls_in_buyee_variations_file =
            File::create("factory_image_urls_in_buyee_variations.json").unwrap();
        let mut factory_image_urls_in_buyee_variations_json =
            factory_image_urls_in_buyee_variations
                .iter()
                .map(|buyee_variation| serde_json::to_string_pretty(buyee_variation).unwrap())
                .collect::<Vec<_>>()
                .join(",");
        factory_image_urls_in_buyee_variations_json.insert(0, '[');
        factory_image_urls_in_buyee_variations_json.push(']');
        factory_image_urls_in_buyee_variations_file
            .write_all(factory_image_urls_in_buyee_variations_json.as_bytes())
            .unwrap();
    }

    let order_urls_in_buyee_variations = files_where_buyee_variations_is_not_empty
        .iter()
        .filter(|buyee_variation| buyee_variation.order_url.is_some())
        .collect::<Vec<_>>();

    if !order_urls_in_buyee_variations.is_empty() {
        let mut order_urls_in_buyee_variations_file =
            File::create("order_urls_in_buyee_variations.json").unwrap();
        let mut order_urls_in_buyee_variations_json = order_urls_in_buyee_variations
            .iter()
            .map(|buyee_variation| serde_json::to_string_pretty(buyee_variation).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        order_urls_in_buyee_variations_json.insert(0, '[');
        order_urls_in_buyee_variations_json.push(']');
        order_urls_in_buyee_variations_file
            .write_all(order_urls_in_buyee_variations_json.as_bytes())
            .unwrap();
    }

    // // pub images: Vec<Image>,
    // Image {
    //    pub caption: Option<Value>,
    // }

    let files_where_images_is_not_empty = all_items
        .iter()
        .filter(|root| !root.images.is_empty())
        .flat_map(|root| root.images.clone())
        .collect::<Vec<_>>();

    let captions_in_images = files_where_images_is_not_empty
        .iter()
        .filter(|image| image.caption.is_some())
        .collect::<Vec<_>>();

    if !captions_in_images.is_empty() {
        let mut captions_in_images_file = File::create("captions_in_images.json").unwrap();
        let mut captions_in_images_json = captions_in_images
            .iter()
            .map(|image| serde_json::to_string_pretty(image).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        captions_in_images_json.insert(0, '[');
        captions_in_images_json.push(']');
        captions_in_images_file
            .write_all(captions_in_images_json.as_bytes())
            .unwrap();
    }

    // // pub variations: Vec<Variation>,
    // Variation {
    //     pub factory_image_url: Option<Value>,
    //     pub order_url: Option<Value>,
    // }

    let files_where_variations_is_not_empty = all_items
        .iter()
        .filter(|root| !root.variations.is_empty())
        .flat_map(|root| root.variations.clone())
        .collect::<Vec<_>>();

    let factory_image_urls_in_variations = files_where_variations_is_not_empty
        .iter()
        .filter(|variation| variation.factory_image_url.is_some())
        .collect::<Vec<_>>();

    if !factory_image_urls_in_variations.is_empty() {
        let mut factory_image_urls_in_variations_file =
            File::create("factory_image_urls_in_variations.json").unwrap();
        let mut factory_image_urls_in_variations_json = factory_image_urls_in_variations
            .iter()
            .map(|variation| serde_json::to_string_pretty(variation).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        factory_image_urls_in_variations_json.insert(0, '[');
        factory_image_urls_in_variations_json.push(']');
        factory_image_urls_in_variations_file
            .write_all(factory_image_urls_in_variations_json.as_bytes())
            .unwrap();
    }

    let order_urls_in_variations = files_where_variations_is_not_empty
        .iter()
        .filter(|variation| variation.order_url.is_some())
        .collect::<Vec<_>>();

    if !order_urls_in_variations.is_empty() {
        let mut order_urls_in_variations_file =
            File::create("order_urls_in_variations.json").unwrap();
        let mut order_urls_in_variations_json = order_urls_in_variations
            .iter()
            .map(|variation| serde_json::to_string_pretty(variation).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        order_urls_in_variations_json.insert(0, '[');
        order_urls_in_variations_json.push(']');
        order_urls_in_variations_file
            .write_all(order_urls_in_variations_json.as_bytes())
            .unwrap();
    }

    let downloadable_in_variations = files_where_variations_is_not_empty
        .iter()
        .filter(|variation| variation.downloadable.is_some())
        .collect::<Vec<_>>();

    let musics_in_downloadable = downloadable_in_variations
        .iter()
        .filter(|downloadable| downloadable.downloadable.as_ref().unwrap().musics.is_some())
        .filter(|downloadable| {
            !downloadable
                .downloadable
                .as_ref()
                .unwrap()
                .musics
                .as_ref()
                .unwrap()
                .is_empty()
        })
        .collect::<Vec<_>>();

    if !musics_in_downloadable.is_empty() {
        let mut musics_in_downloadable_file = File::create("musics_in_downloadable.json").unwrap();
        let mut musics_in_downloadable_json = musics_in_downloadable
            .iter()
            .map(|downloadable| serde_json::to_string_pretty(downloadable).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        musics_in_downloadable_json.insert(0, '[');
        musics_in_downloadable_json.push(']');
        musics_in_downloadable_file
            .write_all(musics_in_downloadable_json.as_bytes())
            .unwrap();
    }
}

pub fn check_if_the_unneeded_files_are_generated_and_panic_if_they_do() {
    // factory_description.json
    // order.json
    // tracks.json
    // downloadables_in_buyee_variations.json
    // factory_image_urls_in_buyee_variations.json
    // order_urls_in_buyee_variations.json
    // captions_in_images.json
    // factory_image_urls_in_variations.json
    // order_urls_in_variations.json
    // musics_in_downloadable.json

    let files = vec![
        "factory_description.json",
        "order.json",
        "tracks.json",
        "downloadables_in_buyee_variations.json",
        "factory_image_urls_in_buyee_variations.json",
        "order_urls_in_buyee_variations.json",
        "captions_in_images.json",
        "factory_image_urls_in_variations.json",
        "order_urls_in_variations.json",
        "musics_in_downloadable.json",
    ];
    for file in files {
        if fs::read(file).is_ok() {
            panic!("{} exists", file);
        }
    }
}
