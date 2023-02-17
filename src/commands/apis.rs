use html2text::from_read;
use poise::serenity_prelude::{AttachmentType, Colour};
use reqwest::Client;
use serde_json::json;
use tracing::instrument;
use tracing_unwrap::{ResultExt, OptionExt};

use crate::{Context, Error, vars::{ANIME_QUERY, MANGA_QUERY}};

/// Get an AniList entry for an Anime
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn anime(
    ctx: Context<'_>,
    #[description = "Name"] msg: String,
    #[description = "Output raw json"] raw: Option<bool>,
) -> Result<(), Error> {
    // Tell discord wait longer then 3 seconds
    ctx.defer().await?;

    let client = Client::new();

    // Define query and variables
    let json = json!({"query": ANIME_QUERY, "variables": {"search": format!("{msg}")}});

    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap_or_log()
        .text()
        .await;

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap_or_log()).unwrap_or_log();

    let formatted_json = format!("{result:#?}");

    // let anime_id = result["data"]["Media"]["id"].as_u64().unwrap_or_log();
    let description = from_read(
        result["data"]["Media"]["description"]
            .as_str()
            .unwrap_or_log()
            .as_bytes(),
        50,
    );
    let status = result["data"]["Media"]["status"].as_str().unwrap_or_log();
    let anilist_url = result["data"]["Media"]["siteUrl"].as_str().unwrap_or_log();
    let episode_count = result["data"]["Media"]["episodes"].as_u64().unwrap_or_log();
    let average_episode_length = result["data"]["Media"]["duration"].as_u64().unwrap_or_log();
    let average_score = result["data"]["Media"]["averageScore"].as_u64().unwrap_or_log();
    let median_score = result["data"]["Media"]["meanScore"].as_u64().unwrap_or_log();
    let adult = result["data"]["Media"]["isAdult"].as_bool().unwrap_or_log();

    let romaji_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log();
    let english_title = if result["data"]["Media"]["title"]["english"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["title"]["english"]
            .as_str()
            .unwrap_or_log()
    } else {
        result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log()
    };

    let base_colour = if result["data"]["Media"]["coverImage"]["color"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["coverImage"]["color"]
            .as_str()
            .unwrap_or_log()
    } else {
        "#aed6f1"
    };

    let image = result["data"]["Media"]["coverImage"]["extraLarge"]
        .as_str()
        .unwrap_or_log();
    let small_image = result["data"]["Media"]["coverImage"]["large"]
        .as_str()
        .unwrap_or_log();

    let season = if result["data"]["Media"]["season"].as_str().is_some() {
        result["data"]["Media"]["season"].as_str().unwrap_or_log()
    } else {
        "N/A"
    };

    let start_year = if result["data"]["Media"]["startDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["year"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_month = if result["data"]["Media"]["startDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_day = if result["data"]["Media"]["startDate"]["day"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["day"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };

    let end_year = if result["data"]["Media"]["endDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["year"].as_i64().unwrap_or_log()
    } else {
        -1
    };
    let end_month = if result["data"]["Media"]["endDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let end_day = if result["data"]["Media"]["endDate"]["day"].as_i64().is_some() {
        result["data"]["Media"]["endDate"]["day"].as_i64().unwrap_or_log()
    } else {
        -1
    };

    let without_prefix = base_colour.trim_start_matches('#');
    let colour_i32 = i32::from_str_radix(without_prefix, 16).unwrap_or_log();

    let field_list = [
        ("English Name", english_title.to_string(), true),
        ("Romaji Name", romaji_title.to_string(), true),
        ("Description", description.to_string(), false),
        (
            "Start Date",
            format!("{season} {start_year}/{start_month}/{start_day}"),
            true,
        ),
        (
            "End Date",
            format!("{end_year}/{end_month}/{end_day}"),
            true,
        ),
        ("Status", status.to_string(), true),
        ("Episode Count", format!("{episode_count}"), true),
        (
            "Episode Length",
            format!("{average_episode_length} minutes"),
            true,
        ),
        ("Average score", format!("{average_score}"), true),
        ("Mean score", format!("{median_score}"), true),
        ("Is adult?", format!("{adult}"), true),
    ];

    if raw.is_some() {
        if raw.unwrap_or_log() {
            ctx.send(|f| {
                f.content("Anime result")
                    .ephemeral(false)
                    .attachment(AttachmentType::Bytes {
                        data: std::borrow::Cow::Borrowed(formatted_json.as_bytes()),
                        filename: String::from("Anime.json"),
                    })
            })
            .await?;
        } else {
            ctx.send(|f| {
                f.embed(|b| {
                    b.colour(Colour::from(colour_i32).tuple())
                        .description("Anime Result")
                        .image(image)
                        .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                        .fields(field_list)
                })
            })
            .await?;
        }
    } else {
        ctx.send(|f| {
            f.embed(|b| {
                b.colour(Colour::from(colour_i32).tuple())
                    .description("Anime Result")
                    .image(image)
                    .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                    .fields(field_list)
            })
        })
        .await?;
    }
    Ok(())
}

/// Get an AniList entry for a Manga
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn manga(
    ctx: Context<'_>,
    #[description = "Name"] msg: String,
    #[description = "Output raw json"] raw: Option<bool>,
) -> Result<(), Error> {
    // Tell discord wait longer then 3 seconds
    ctx.defer().await?;

    let client = Client::new();

    // Define query and variables
    let json = json!({"query": MANGA_QUERY, "variables": {"search": format!("{msg}")}});

    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap_or_log()
        .text()
        .await;

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap_or_log()).unwrap_or_log();

    let formatted_json = format!("{result:#?}");

    if raw.is_some() && raw.unwrap_or_log() {
        ctx.send(|f| {
            f.content("Anime result")
                .ephemeral(false)
                .attachment(AttachmentType::Bytes {
                    data: std::borrow::Cow::Borrowed(formatted_json.as_bytes()),
                    filename: String::from("Anime.json"),
                })
        })
        .await?;

        return Ok(());
    }

    // let anime_id = result["data"]["Media"]["id"].as_u64().unwrap_or_log();
    let description = from_read(
        result["data"]["Media"]["description"]
            .as_str()
            .unwrap_or_log()
            .as_bytes(),
        50,
    );
    let status = result["data"]["Media"]["status"].as_str().unwrap_or_log();
    let anilist_url = result["data"]["Media"]["siteUrl"].as_str().unwrap_or_log();
    let volume_count = if result["data"]["Media"]["volumes"].as_i64().is_some() {
        result["data"]["Media"]["volumes"].as_i64().unwrap_or_log()
    } else {
        -1
    };
    let chapter_coumt = result["data"]["Media"]["chapters"].as_u64().unwrap_or_log();
    let average_score = result["data"]["Media"]["averageScore"].as_u64().unwrap_or_log();
    let median_score = result["data"]["Media"]["meanScore"].as_u64().unwrap_or_log();
    let adult = result["data"]["Media"]["isAdult"].as_bool().unwrap_or_log();

    let romaji_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log();
    let english_title = if result["data"]["Media"]["title"]["english"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["title"]["english"]
            .as_str()
            .unwrap_or_log()
    } else {
        result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log()
    };

    let base_colour = if result["data"]["Media"]["coverImage"]["color"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["coverImage"]["color"]
            .as_str()
            .unwrap_or_log()
    } else {
        "#aed6f1"
    };

    let image = result["data"]["Media"]["coverImage"]["extraLarge"]
        .as_str()
        .unwrap_or_log();
    let small_image = result["data"]["Media"]["coverImage"]["large"]
        .as_str()
        .unwrap_or_log();

    let season = if result["data"]["Media"]["season"].as_str().is_some() {
        result["data"]["Media"]["season"].as_str().unwrap_or_log()
    } else {
        "N/A"
    };

    let start_year = if result["data"]["Media"]["startDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["year"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_month = if result["data"]["Media"]["startDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_day = if result["data"]["Media"]["startDate"]["day"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["day"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };

    let end_year = if result["data"]["Media"]["endDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["year"].as_i64().unwrap_or_log()
    } else {
        -1
    };
    let end_month = if result["data"]["Media"]["endDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let end_day = if result["data"]["Media"]["endDate"]["day"].as_i64().is_some() {
        result["data"]["Media"]["endDate"]["day"].as_i64().unwrap_or_log()
    } else {
        -1
    };

    let without_prefix = base_colour.trim_start_matches('#');
    let colour_i32 = i32::from_str_radix(without_prefix, 16).unwrap_or_log();

    let field_list = [
        ("English Name", english_title.to_string(), true),
        ("Romaji Name", romaji_title.to_string(), true),
        ("Description", description.to_string(), false),
        (
            "Start Date",
            format!("{season} {start_year}/{start_month}/{start_day}"),
            true,
        ),
        (
            "End Date",
            format!("{end_year}/{end_month}/{end_day}"),
            true,
        ),
        ("Status", status.to_string(), true),
        ("Volume Count", format!("{volume_count}"), true),
        ("Chapter Count", format!("{chapter_coumt} minutes"), true),
        ("Average Score", format!("{average_score}"), true),
        ("Mean Score", format!("{median_score}"), true),
        ("Is Adult?", format!("{adult}"), true),
    ];

    ctx.send(|f| {
        f.embed(|b| {
            b.colour(Colour::from(colour_i32).tuple())
                .description("Anime Result")
                .image(image)
                .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                .fields(field_list)
        })
    })
    .await?;
    Ok(())
}