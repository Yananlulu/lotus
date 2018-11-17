error_chain!{
    foreign_links {
        Pug(pug::errors::Error);

        // StdIo(std::io::Error);
        // StdSystemTime(std::time::SystemTimeError);
        // StdStrUtf8(std::str::Utf8Error);
        // StdNumParseInt(std::num::ParseIntError);
        //
        // SerdeJson(serde_json::Error);
        // SerdeYaml(serde_yaml::Error);
        // SerdeXml(serde_xml_rs::Error);
        // Redis(redis::RedisError);
        // R2d2(r2d2::Error);
        // ChronoParse(chrono::ParseError);
        // Diesel(diesel::result::Error);
        // Base64Decode(base64::DecodeError);
        // Ini(ini::ini::Error);
        // LanguageTags(language_tags::Error);
        // LettreEmail(lettre_email::error::Error);
        // LettreSmtp(lettre::smtp::error::Error);
        // UrlParse(url::ParseError);
        // TomlSer(toml::ser::Error);
        // TomlDe(toml::de::Error);
        // Log4rs(log4rs::Error);
        // Validator(validator::ValidationErrors);
        // Mustache(mustache::Error);
        // Regex(regex::Error);
        // Hyper(hyper::Error);
        // Rss(rss::Error);
        // RocketConfig(rocket::config::ConfigError);
        // RocketLaunch(rocket::error::LaunchError);
    }
}
