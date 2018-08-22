pub mod forum;
pub mod nut;

use std::result::Result as StdResult;

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use gotham::{
    http::response::create_response,
    router::{builder::*, Router},
    state::State,
};
use hyper::{Response, StatusCode};
use mime;
use robots_txt::Robots;
use rss::{ChannelBuilder, ItemBuilder};
use sitemap::structs::ChangeFreq;
use sitemap::{structs::UrlEntry, writer::SiteMapWriter, Error as SitemapError};

use super::response;

pub type SitemapItem = (String, f32, ChangeFreq, NaiveDateTime);
pub type RssItem = (String, String, String, NaiveDateTime);

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(nut::controllers::index);
        route.get("/sitemap.xml").to(sitemap_xml);

        route.scope("/forum", |route| {
            route.get("/").to(forum::controllers::index);
        });

        // route.scope("/checkout", |route| {
        //     route.get("/start").to(checkout::start);
        //
        //     // Associations allow a single path to be matched for multiple HTTP verbs
        //     // with each delegating to a unique handler or the same handler, as shown here with
        //     // put and patch.
        //     route.associate("/address", |assoc| {
        //         assoc.post().to(checkout::address::create);
        //         assoc.put().to(checkout::address::update);
        //         assoc.patch().to(checkout::address::update);
        //         assoc.delete().to(checkout::address::delete);
        //     });
        //
        //     route
        //         .post("/payment_details")
        //         .to(checkout::payment_details::create);
        //
        //     route
        //         .put("/payment_details")
        //         .to(checkout::payment_details::update);
        //
        //     route.post("/complete").to(checkout::complete);
        // });
        //
        // route.scope("/api", |route| {
        //     route.get("/products").to(api::products::index);
        // });
    })
}

// #[get("/global/<file..>")]
// fn global(file: PathBuf) -> Result<NamedFile> {
//     Ok(NamedFile::open(
//         Path::new("themes").join("global").join(file),
//     )?)
// }
//
// #[get("/assets/<file..>")]
// fn assets(file: PathBuf, cfg: State<Arc<env::Config>>) -> Result<NamedFile> {
//     Ok(NamedFile::open(
//         Path::new("themes")
//             .join(cfg.http.theme.clone())
//             .join("assets")
//             .join(file),
//     )?)
// }
//
// #[get("/3rd/<file..>")]
// fn third(file: PathBuf) -> Result<NamedFile> {
//     Ok(NamedFile::open(Path::new("node_modules").join(file))?)
// }
//
// #[get("/upload/<file..>")]
// fn upload(file: PathBuf) -> Result<NamedFile> {
//     Ok(NamedFile::open(Path::new("tmp").join("upload").join(file))?)
// }
//
// // https://en.wikipedia.org/wiki/Robots_exclusion_standard
// #[get("/robots.txt")]
// fn robots_txt(home: Home) -> Result<String> {
//     let Home(home) = home;
//     Ok(format!(
//         "{}",
//         Robots::start_build()
//             .host(home.clone())
//             .start_section_for("*")
//             .disallow("/my/")
//             .sitemap((home + "/sitemap.xml").parse()?)
//             .end_section()
//             .finalize()
//     ))
// }

fn build_sitemap(home: &String, buf: &mut Vec<u8>) -> StdResult<(), SitemapError> {
    let srt = SiteMapWriter::new(buf);
    let mut urt = srt.start_urlset()?;
    let fix = FixedOffset::east(0);

    for items in vec![nut::sitemap(), forum::sitemap()] {
        for (loc, priority, changefreq, lastmod) in items {
            urt.url(
                UrlEntry::builder()
                    .loc(format!("{}{}", home, loc))
                    .priority(priority)
                    .changefreq(changefreq)
                    .lastmod(DateTime::<Utc>::from_utc(lastmod, Utc).with_timezone(&fix)),
            )?;
        }
    }

    urt.end()?;
    Ok(())
}

// fn sitemap_xml_gz<'r>() -> response::Result<'r> {
//     let mut buf = Cursor::new(Vec::new());
//     match build_sitemap(&mut buf) {
//         Ok(_) => Response::build()
//             .sized_body(buf)
//             .header(ContentType::XML)
//             .header(ContentType::new("application", "x-gzip"))
//             .header(Header::from(ContentEncoding(vec![Encoding::Gzip])))
//             .header(Header::from(ContentDisposition {
//                 disposition: DispositionType::Attachment,
//                 parameters: vec![DispositionParam::Filename(
//                     Charset::Us_Ascii,
//                     None,
//                     b"sitemap.xml.gz".to_vec(),
//                 )],
//             }))
//             .ok(),
//         Err(e) => {
//             error!("{:?}", e);
//             Err(Status::InternalServerError)
//         }
//     }
// }

// https://en.wikipedia.org/wiki/Site_map
// https://www.sitemaps.org/protocol.html
fn sitemap_xml(state: State) -> (State, Response) {
    let mut buf = Vec::new();
    let res = match build_sitemap(&"http://www.localhost:aaa".to_string(), &mut buf) {
        Ok(_) => create_response(&state, StatusCode::Ok, Some((buf, mime::TEXT_XML))),
        Err(e) => create_response(
            &state,
            StatusCode::InternalServerError,
            Some((format!("aaa {:?}", e).into_bytes(), mime::TEXT_PLAIN_UTF_8)),
        ),
    };
    (state, res)
}

// #[get("/rss/<lang>")]
// fn rss_atom(db: Db, home: Home, lang: String) -> Result<Xml<Vec<u8>>> {
//     let Home(home) = home;
//     let db = db.deref();
//     let mut fields = Vec::new();
//     for items in vec![nut::rss(&lang), forum::rss(&lang)] {
//         for (url, title, desc, last) in items {
//             fields.push(
//                 ItemBuilder::default()
//                     .link(format!("{}{}", home, url))
//                     .title(title)
//                     .description(desc)
//                     .pub_date(DateTime::<Utc>::from_utc(last, Utc).to_rfc3339())
//                     .build()?,
//             );
//         }
//     }
//     let mut buf = Vec::new();
//     let ch = ChannelBuilder::default()
//         .link(home.clone())
//         .language(lang.clone())
//         .pub_date(Utc::now().to_rfc3339())
//         .title(t!(db, &lang, "site.title"))
//         .description(t!(db, &lang, "site.description"))
//         .copyright(t!(db, &lang, "site.copyright"))
//         .items(fields)
//         .build()?;
//
//     ch.write_to(&mut buf)?;
//
//     Ok(Xml(buf))
// }
