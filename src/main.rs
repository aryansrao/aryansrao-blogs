use axum::{
    extract::{Path, State, Query},
    http::{header, StatusCode, HeaderMap},
    response::{Html, IntoResponse, Response, Json, Redirect},
    routing::{get, post, delete},
    Extension, Router, Form,
};
use base64::Engine;
use handlebars::Handlebars;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::PathBuf, sync::Arc, time::{Duration, Instant}};
use tokio::sync::RwLock;
use syntect::{
    highlighting::ThemeSet,
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use chrono::{DateTime, Local, TimeZone};
use uuid::Uuid;

// GitHub configuration
const GITHUB_API_BASE: &str = "https://api.github.com";
static GITHUB_USERNAME: Lazy<String> = Lazy::new(|| env::var("GITHUB_USERNAME").unwrap_or_else(|_| "aryansrao".into()));
static GITHUB_TOKEN: Lazy<Option<String>> = Lazy::new(|| env::var("GITHUB_TOKEN").ok());

// Admin configuration
static ADMIN_PASSWORD: Lazy<String> = Lazy::new(|| env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".into()));
const MAX_LOGIN_ATTEMPTS: u32 = 5;
const LOCKOUT_DURATION_SECS: u64 = 300; // 5 minutes
const SESSION_DURATION_SECS: u64 = 3600; // 1 hour

// Initialize syntax highlighting sets once
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

// Site configuration for SEO
#[derive(Serialize, Clone)]
struct SiteConfig {
    title: String,
    description: String,
    url: String,
    author: String,
    language: String,
    twitter_handle: String,
    logo: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Aryan S Rao".to_string(),
            description: "My own blog page made with rust and axum".to_string(),
            url: env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
            author: "aryansrao".to_string(),
            language: "en".to_string(),
            twitter_handle: "@aryansrao".to_string(),
            logo: "/logo.png".to_string(),
        }
    }
}

// ============================================================================
// Admin Panel - Authentication & State Management
// ============================================================================

#[derive(Clone)]
struct AdminState {
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    login_attempts: Arc<RwLock<HashMap<String, LoginAttemptData>>>,
    github_links: Arc<RwLock<HashMap<String, GitHubLink>>>, // Maps post slug to GitHub repo
}

#[derive(Clone)]
struct SessionData {
    created_at: Instant,
    #[allow(dead_code)]
    last_activity: Instant,
}

#[derive(Clone)]
struct LoginAttemptData {
    attempts: u32,
    first_attempt: Instant,
    locked_until: Option<Instant>,
}

#[derive(Clone, Serialize, Deserialize)]
struct GitHubLink {
    repo_name: String,
    repo_full_name: String,
    last_synced: String,
    auto_sync: bool,
}

impl AdminState {
    fn new() -> Self {
        // Load GitHub links from file if exists
        let github_links = if let Ok(content) = fs::read_to_string("content/.github_links.json") {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            login_attempts: Arc::new(RwLock::new(HashMap::new())),
            github_links: Arc::new(RwLock::new(github_links)),
        }
    }
    
    fn save_github_links(&self, links: &HashMap<String, GitHubLink>) {
        if let Ok(json) = serde_json::to_string_pretty(links) {
            let _ = fs::write("content/.github_links.json", json);
        }
    }
}

// Form data structures
#[derive(Deserialize)]
struct LoginForm {
    password: String,
}

#[derive(Deserialize)]
struct PostForm {
    title: String,
    content: String,
    tags: String,
    summary: String,
    slug: Option<String>, // For editing existing posts
}

#[derive(Deserialize)]
struct GitHubImportForm {
    repo_name: String,
    auto_sync: Option<String>,
}

#[derive(Deserialize)]
struct WebhookPayload {
    repository: Option<WebhookRepo>,
    #[serde(rename = "ref")]
    git_ref: Option<String>,
}

#[derive(Deserialize)]
struct WebhookRepo {
    name: String,
    #[allow(dead_code)]
    full_name: String,
}

// Define metadata structure for blog posts
#[derive(Serialize, Default, Debug, Clone)]
struct Metadata {
    title: String,
    date: String,
    tags: Vec<String>,
    summary: String,
    author: Option<String>,
    image: Option<String>,
    image_alt: Option<String>,
    keywords: Option<String>,
    canonical: Option<String>,
    github_repo: Option<String>,
    website: Option<String>,
}

// Define blog post structure
#[derive(Serialize, Debug, Clone)]
struct Post {
    title: String,
    content: String,
    summary: String,
    date: String,
    date_iso: String, // ISO 8601 format for structured data
    tags: Vec<String>,
    filename: String,
    slug: String,
    author: String,
    image: String,
    image_alt: String,
    keywords: String,
    canonical: String,
    reading_time: u32, // Estimated reading time in minutes
    word_count: u32,
    github_repo: Option<String>,
    website: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
struct SearchResult {
    title: String,
    slug: String,
    summary: String,
    date: String,
    date_iso: String,
    tags: Vec<String>,
    reading_time: u32,
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

// Parse metadata from Markdown file content (supports multiline values)
fn parse_metadata(content: &str) -> Option<Metadata> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }
    let meta_str = parts[1];
    let mut meta = Metadata::default();
    
    for line in meta_str.lines() {
        let kv: Vec<&str> = line.splitn(2, ':').collect();
        if kv.len() == 2 {
            let key = kv[0].trim();
            let value = kv[1].trim().trim_matches('"');
            match key {
                "title" => meta.title = value.to_string(),
                "date" => meta.date = value.to_string(),
                "tags" => {
                    // Support both array format [tag1, tag2] and comma-separated
                    let cleaned = value.trim_matches(|c| c == '[' || c == ']');
                    meta.tags = cleaned
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                "summary" => meta.summary = value.to_string(),
                "author" => meta.author = Some(value.to_string()),
                "image" => meta.image = Some(value.to_string()),
                "image_alt" => meta.image_alt = Some(value.to_string()),
                "keywords" => meta.keywords = Some(value.to_string()),
                "canonical" => meta.canonical = Some(value.to_string()),
                "github_repo" => meta.github_repo = Some(value.to_string()),
                "website" | "homepage" => meta.website = Some(value.to_string()),
                _ => {}
            }
        }
    }
    Some(meta)
}

// Convert Markdown content to HTML with full feature support
fn markdown_to_html(markdown: &str) -> String {
    // Enable ALL markdown extensions
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(markdown, options);
    
    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut in_table_head = false;
    let mut heading_level: Option<u32> = None;
    let mut heading_text = String::new();
    let mut heading_inner = String::new();
    
    let slugify = |text: &str| {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    };
    
    let events: Vec<Event> = parser.collect();
    let mut i = 0;
    
    while i < events.len() {
        // If we're inside a heading, buffer content to add an id
        if heading_level.is_some() {
            match &events[i] {
                Event::End(TagEnd::Heading(_)) => {
                    let level = heading_level.take().unwrap_or(2);
                    let slug = slugify(&heading_text);
                    html_output.push_str(&format!("<h{} id=\"{}\">{}</h{}>", level, slug, heading_inner, level));
                    heading_text.clear();
                    heading_inner.clear();
                    i += 1;
                    continue;
                }
                Event::Text(t) => {
                    heading_text.push_str(t);
                    pulldown_cmark::html::push_html(&mut heading_inner, std::iter::once(events[i].clone()));
                    i += 1;
                    continue;
                }
                Event::Code(t) => {
                    heading_text.push_str(t);
                    pulldown_cmark::html::push_html(&mut heading_inner, std::iter::once(events[i].clone()));
                    i += 1;
                    continue;
                }
                Event::SoftBreak | Event::HardBreak => {
                    heading_text.push(' ');
                    pulldown_cmark::html::push_html(&mut heading_inner, std::iter::once(events[i].clone()));
                    i += 1;
                    continue;
                }
                _ => {
                    pulldown_cmark::html::push_html(&mut heading_inner, std::iter::once(events[i].clone()));
                    i += 1;
                    continue;
                }
            }
        }
        
        match &events[i] {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_content.clear();
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                // Apply syntax highlighting
                let highlighted = highlight_code(&code_content, &code_lang);
                html_output.push_str(&highlighted);
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(text);
            }
            Event::Start(Tag::Heading { level, .. }) => {
                let lvl = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                heading_level = Some(lvl);
                heading_text.clear();
                heading_inner.clear();
            }
            Event::Start(Tag::Table(_alignments)) => {
                html_output.push_str("<div class=\"table-container\"><table>");
            }
            Event::End(TagEnd::Table) => {
                html_output.push_str("</table></div>");
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                html_output.push_str("<thead><tr>");
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                html_output.push_str("</tr></thead><tbody>");
            }
            Event::Start(Tag::TableRow) => {
                html_output.push_str("<tr>");
            }
            Event::End(TagEnd::TableRow) => {
                html_output.push_str("</tr>");
            }
            Event::Start(Tag::TableCell) => {
                if in_table_head {
                    html_output.push_str("<th>");
                } else {
                    html_output.push_str("<td>");
                }
            }
            Event::End(TagEnd::TableCell) => {
                if in_table_head {
                    html_output.push_str("</th>");
                } else {
                    html_output.push_str("</td>");
                }
            }
            Event::TaskListMarker(checked) => {
                let checkbox = if *checked {
                    r#"<input type="checkbox" checked disabled class="mr-2 h-4 w-4 rounded border-gray-300 text-indigo-600 bg-indigo-600 accent-indigo-600"> "#
                } else {
                    r#"<input type="checkbox" disabled class="mr-2 h-4 w-4 rounded border-gray-300 bg-gray-100 dark:bg-gray-700"> "#
                };
                html_output.push_str(checkbox);
            }
            Event::Start(Tag::FootnoteDefinition(name)) => {
                html_output.push_str(&format!(
                    r#"<div class="footnote" id="fn-{}"><sup>{}</sup> "#,
                    name, name
                ));
            }
            Event::End(TagEnd::FootnoteDefinition) => {
                html_output.push_str("</div>");
            }
            Event::FootnoteReference(name) => {
                html_output.push_str(&format!(
                    "<sup><a href=\"#fn-{}\" class=\"footnote-ref\">[{}]</a></sup>",
                    name, name
                ));
            }
            Event::Start(Tag::Strikethrough) => {
                html_output.push_str("<del class=\"line-through text-gray-500\">");
            }
            Event::End(TagEnd::Strikethrough) => {
                html_output.push_str("</del>");
            }
            Event::Start(Tag::BlockQuote(_)) => {
                html_output.push_str("<blockquote class=\"border-l-4 border-primary-500 pl-4 my-4 italic text-gray-600 dark:text-gray-400\">");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                html_output.push_str("</blockquote>");
            }
            // Handle all other events with default HTML rendering
            _ => {
                let single_event = std::iter::once(events[i].clone());
                pulldown_cmark::html::push_html(&mut html_output, single_event);
            }
        }
        i += 1;
    }
    
    html_output
}

fn strip_html_tags(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut in_tag = false;
    for c in input.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(c),
            _ => {}
        }
    }
    output
}

// Syntax highlighting for code blocks
fn highlight_code(code: &str, lang: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_token(lang)
        .or_else(|| SYNTAX_SET.find_syntax_by_extension(lang))
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    let theme = &THEME_SET.themes["base16-ocean.dark"];
    
    match highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme) {
        Ok(html) => {
            format!(
                r#"<div class="code-block relative my-4 rounded-lg overflow-hidden">
                    <div class="code-header flex items-center justify-between px-4 py-2 bg-gray-800 text-gray-400 text-xs">
                        <span class="code-lang font-mono">{}</span>
                        <button class="copy-btn hover:text-white transition-colors" onclick="copyCode(this)">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                            </svg>
                        </button>
                    </div>
                    <div class="code-content overflow-x-auto">{}</div>
                </div>"#,
                if lang.is_empty() { "text" } else { lang },
                html
            )
        }
        Err(_) => {
            format!(
                r#"<pre class="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto my-4"><code class="language-{}">{}</code></pre>"#,
                lang,
                html_escape::encode_text(code)
            )
        }
    }
}

// Calculate reading time based on word count
fn calculate_reading_time(content: &str) -> (u32, u32) {
    let word_count = content.split_whitespace().count() as u32;
    let reading_time = (word_count / 200).max(1); // Average 200 words per minute
    (reading_time, word_count)
}

// Retrieve all blog posts from content directory
fn get_posts(site_config: &SiteConfig) -> Vec<Post> {
    let content_dir = PathBuf::from("content");
    let mut posts = Vec::new();

    if let Ok(entries) = fs::read_dir(content_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map(|s| s == "md").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Some(metadata) = parse_metadata(&content) {
                        let content_str = content.splitn(3, "---").nth(2).unwrap_or("");
                        let html_content = markdown_to_html(content_str);
                        let (reading_time, word_count) = calculate_reading_time(content_str);

                        let date = match DateTime::parse_from_str(
                            &format!("{} 00:00:00 +0000", metadata.date),
                            "%Y-%m-%d %H:%M:%S %z",
                        ) {
                            Ok(d) => Local.from_utc_datetime(&d.naive_utc()),
                            Err(_) => entry
                                .metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .map(|t| DateTime::<Local>::from(t))
                                .unwrap_or_else(Local::now),
                        };

                        let slug = metadata.title.to_lowercase()
                            .chars()
                            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
                            .collect::<String>()
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join("-");

                        let tags_clone = metadata.tags.clone();
                        let post = Post {
                            title: metadata.title.clone(),
                            content: html_content,
                            summary: if metadata.summary.is_empty() {
                                content_str.chars().take(160).collect::<String>() + "..."
                            } else {
                                metadata.summary
                            },
                            date: date.format("%B %d, %Y").to_string(),
                            date_iso: date.format("%Y-%m-%dT%H:%M:%S%z").to_string(),
                            tags: tags_clone.clone(),
                            filename: entry.file_name().to_str().unwrap_or("").to_string(),
                            slug: slug.clone(),
                            author: metadata.author.unwrap_or_else(|| site_config.author.clone()),
                            image: metadata.image.unwrap_or_else(|| format!("{}/og-default.png", site_config.url)),
                            image_alt: metadata.image_alt.unwrap_or_else(|| metadata.title.clone()),
                            keywords: metadata.keywords.unwrap_or_else(|| {
                                tags_clone.iter().take(5).cloned().collect::<Vec<_>>().join(", ")
                            }),
                            canonical: metadata.canonical.unwrap_or_else(|| {
                                format!("{}/blog/{}", site_config.url, slug)
                            }),
                            reading_time,
                            word_count,
                            website: metadata.website,
                            github_repo: metadata.github_repo,
                        };
                        posts.push(post);
                    }
                }
            }
        }
    }

    posts.sort_by(|a, b| b.date_iso.cmp(&a.date_iso));
    posts
}

// Home route handler
async fn index(Extension(hb): Extension<Arc<Handlebars<'_>>>) -> impl IntoResponse {
    let site_config = SiteConfig::default();
    let posts = get_posts(&site_config);
    let posts_count = posts.len();
    
    let mut data = HashMap::new();
    data.insert("posts", serde_json::to_value(&posts).unwrap());
    data.insert("posts_count", serde_json::to_value(posts_count).unwrap());
    data.insert("site", serde_json::to_value(&site_config).unwrap());
    data.insert("current_year", serde_json::to_value(Local::now().format("%Y").to_string()).unwrap());
    
    match hb.render("index.html", &data) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(e) => {
            eprintln!("Failed to render index template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template rendering error").into_response()
        }
    }
}

// Tag listing route handler
async fn tag_page(
    Extension(hb): Extension<Arc<Handlebars<'_>>>,
    Path(tag): Path<String>,
) -> impl IntoResponse {
    let site_config = SiteConfig::default();
    let all_posts = get_posts(&site_config);
    let tag_normalized = tag.to_lowercase();

    let filtered_posts: Vec<Post> = all_posts
        .into_iter()
        .filter(|p| p
            .tags
            .iter()
            .any(|t| t.to_lowercase() == tag_normalized))
        .collect();

    let mut data = HashMap::new();
    data.insert("posts", serde_json::to_value(&filtered_posts).unwrap());
    data.insert("posts_count", serde_json::to_value(filtered_posts.len()).unwrap());
    // Reuse the index template; tweak description to show the tag context
    let mut site_override = site_config.clone();
    site_override.description = format!("Posts tagged with '{}'.", tag);
    data.insert("site", serde_json::to_value(&site_override).unwrap());
    data.insert("current_year", serde_json::to_value(Local::now().format("%Y").to_string()).unwrap());

    match hb.render("index.html", &data) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(e) => {
            eprintln!("Failed to render tag template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template rendering error").into_response()
        }
    }
}

// Single post route handler
async fn single_post(
    Extension(hb): Extension<Arc<Handlebars<'_>>>,
    Path(post_title): Path<String>,
) -> impl IntoResponse {
    let site_config = SiteConfig::default();
    let posts = get_posts(&site_config);
    let post = posts.into_iter().find(|p| p.slug == post_title);

    if let Some(post) = post {
        let mut data = HashMap::new();
        data.insert("post", serde_json::to_value(&post).unwrap());
        data.insert("site", serde_json::to_value(&site_config).unwrap());
        data.insert("current_year", serde_json::to_value(Local::now().format("%Y").to_string()).unwrap());
        
        match hb.render("single.html", &data) {
            Ok(rendered) => Html(rendered).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template rendering error").into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Post not found").into_response()
    }
}

// Generate XML Sitemap for SEO
async fn sitemap() -> impl IntoResponse {
    let site_config = SiteConfig::default();
    let posts = get_posts(&site_config);
    
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
"#);

    // Homepage
    xml.push_str(&format!(
        r#"  <url>
    <loc>{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#,
        site_config.url,
        Local::now().format("%Y-%m-%d")
    ));

    // Blog posts
    for post in posts {
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}/blog/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
    <image:image>
      <image:loc>{}</image:loc>
      <image:title>{}</image:title>
    </image:image>
  </url>
"#,
            site_config.url,
            post.slug,
            post.date_iso.split('T').next().unwrap_or(&post.date_iso),
            post.image,
            html_escape::encode_text(&post.title)
        ));
    }

    xml.push_str("</urlset>");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml")
        .body(xml)
        .unwrap()
}

// Generate robots.txt
async fn robots_txt() -> impl IntoResponse {
    let site_config = SiteConfig::default();
    let content = format!(
        r#"User-agent: *
Allow: /

# Sitemaps
Sitemap: {}/sitemap.xml

# Crawl-delay (optional, for politeness)
Crawl-delay: 1

# Disallow admin or private paths (add as needed)
# Disallow: /admin/
# Disallow: /private/
"#,
        site_config.url
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(content)
        .unwrap()
}

// Generate RSS Feed
async fn rss_feed() -> impl IntoResponse {
    let site_config = SiteConfig::default();
    let posts = get_posts(&site_config);
    
    let mut rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <language>{}</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml"/>
    <generator>Axum Blog Engine</generator>
"#,
        html_escape::encode_text(&site_config.title),
        site_config.url,
        html_escape::encode_text(&site_config.description),
        site_config.language,
        Local::now().format("%a, %d %b %Y %H:%M:%S %z"),
        site_config.url
    );

    for post in posts.iter().take(20) {
        rss.push_str(&format!(
            r#"    <item>
      <title>{}</title>
      <link>{}/blog/{}</link>
      <guid isPermaLink="true">{}/blog/{}</guid>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
      <author>{}</author>
      {}
    </item>
"#,
            html_escape::encode_text(&post.title),
            site_config.url,
            post.slug,
            site_config.url,
            post.slug,
            post.date_iso,
            post.summary,
            site_config.author,
            post.tags.iter().map(|t| format!("<category>{}</category>", html_escape::encode_text(t))).collect::<Vec<_>>().join("\n      ")
        ));
    }

    rss.push_str("  </channel>\n</rss>");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/rss+xml")
        .body(rss)
        .unwrap()
}

// ============================================================================
// GitHub Integration - Fetch READMEs from repositories
// ============================================================================

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    name: String,
    full_name: String,
    description: Option<String>,
    html_url: String,
    pushed_at: Option<String>,
    updated_at: Option<String>,
    language: Option<String>,
    stargazers_count: u32,
    fork: bool,
    archived: bool,
    topics: Option<Vec<String>>,
    homepage: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubReadme {
    content: Option<String>,
}

#[derive(Debug, Serialize)]
struct SyncResult {
    success: bool,
    message: String,
    repos_synced: Vec<String>,
    errors: Vec<String>,
}

// Fetch all public repos for the configured GitHub user
async fn fetch_github_repos() -> Result<Vec<GitHubRepo>, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/users/{}/repos?sort=updated&per_page=100", GITHUB_API_BASE, GITHUB_USERNAME.as_str());
    
    let mut request = client
        .get(&url)
        .header("User-Agent", "axum-blog")
        .header("Accept", "application/vnd.github.v3+json");
    
    if let Some(token) = GITHUB_TOKEN.as_ref() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to fetch repos: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }
    
    let repos: Vec<GitHubRepo> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse repos: {}", e))?;
    
    // Filter out forks and archived repos
    Ok(repos.into_iter().filter(|r| !r.fork && !r.archived).collect())
}

// Fetch README content for a specific repo
async fn fetch_readme(repo_name: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/repos/{}/{}/readme", GITHUB_API_BASE, GITHUB_USERNAME.as_str(), repo_name);
    
    let mut request = client
        .get(&url)
        .header("User-Agent", "axum-blog")
        .header("Accept", "application/vnd.github.v3+json");
    
    if let Some(token) = GITHUB_TOKEN.as_ref() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to fetch README: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("No README found for {}", repo_name));
    }
    
    let readme: GitHubReadme = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse README: {}", e))?;
    
    if let Some(content) = readme.content {
        // GitHub returns base64 encoded content
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(content.replace('\n', ""))
            .map_err(|e| format!("Failed to decode README: {}", e))?;
        
        String::from_utf8(decoded)
            .map_err(|e| format!("Invalid UTF-8 in README: {}", e))
    } else {
        Err("README content is empty".to_string())
    }
}

// Create a blog post from a GitHub repo README
fn create_post_from_readme(repo: &GitHubRepo, readme_content: &str) -> String {
    let date = repo.pushed_at.as_ref()
        .or(repo.updated_at.as_ref())
        .map(|d| d.split('T').next().unwrap_or("2025-01-01"))
        .unwrap_or("2025-01-01");
    
    let description = repo.description.as_deref().unwrap_or("A GitHub project");
    
    let mut tags = vec!["github".to_string(), "project".to_string()];
    if let Some(lang) = &repo.language {
        tags.push(lang.to_lowercase());
    }
    if let Some(topics) = &repo.topics {
        for topic in topics.iter().take(3) {
            tags.push(topic.clone());
        }
    }

    let homepage = repo
        .homepage
        .as_deref()
        .map(str::trim)
        .filter(|h| !h.is_empty())
        .unwrap_or("");
    let website_line = if homepage.is_empty() {
        String::new()
    } else {
        format!("\nwebsite: \"{}\"", homepage)
    };
    
    let tags_str = tags.iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");
    
    format!(
        r#"---
title: "{}"
date: "{}"
tags: [{}]
summary: "{}"
author: "{}"
keywords: "{}, github, open source"
github_repo: "{}"{website}
---

{}
"#,
        repo.name,
        date,
        tags_str,
        description.replace('"', "'"),
        GITHUB_USERNAME.as_str(),
        repo.name,
        repo.full_name,
        readme_content,
        website = website_line
    )
}

// Sync all GitHub repos - creates/updates markdown files
async fn sync_github_repos() -> impl IntoResponse {
    let mut result = SyncResult {
        success: true,
        message: String::new(),
        repos_synced: Vec::new(),
        errors: Vec::new(),
    };
    
    // Ensure content directory exists
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        if let Err(e) = fs::create_dir_all(&content_dir) {
            result.success = false;
            result.message = format!("Failed to create content directory: {}", e);
            return Json(result);
        }
    }
    
    // Fetch repos
    let repos = match fetch_github_repos().await {
        Ok(r) => r,
        Err(e) => {
            result.success = false;
            result.message = e;
            return Json(result);
        }
    };
    
    println!("📡 Found {} repositories for {}", repos.len(), GITHUB_USERNAME.as_str());
    
    for repo in repos {
        println!("  → Fetching README for {}...", repo.name);
        
        match fetch_readme(&repo.name).await {
            Ok(readme) => {
                let post_content = create_post_from_readme(&repo, &readme);
                let filename = format!("github-{}.md", repo.name.to_lowercase().replace(' ', "-"));
                let filepath = content_dir.join(&filename);
                
                match fs::write(&filepath, &post_content) {
                    Ok(_) => {
                        println!("    ✅ Created {}", filename);
                        result.repos_synced.push(repo.name);
                    }
                    Err(e) => {
                        let err = format!("Failed to write {}: {}", filename, e);
                        println!("    ❌ {}", err);
                        result.errors.push(err);
                    }
                }
            }
            Err(e) => {
                println!("    ⚠️  {}", e);
                result.errors.push(e);
            }
        }
    }
    
    result.message = format!(
        "Synced {} repos, {} errors",
        result.repos_synced.len(),
        result.errors.len()
    );
    
    println!("✨ GitHub sync complete: {}", result.message);
    
    Json(result)
}

// List all GitHub repos (without syncing)
async fn list_github_repos() -> impl IntoResponse {
    match fetch_github_repos().await {
        Ok(repos) => {
            let repo_list: Vec<serde_json::Value> = repos.iter().map(|r| {
                serde_json::json!({
                    "name": r.name,
                    "description": r.description,
                    "url": r.html_url,
                    "language": r.language,
                    "stars": r.stargazers_count,
                    "updated": r.pushed_at
                })
            }).collect();
            
            Json(serde_json::json!({
                "success": true,
                "username": GITHUB_USERNAME.as_str(),
                "count": repo_list.len(),
                "repos": repo_list
            }))
        }
        Err(e) => {
            Json(serde_json::json!({
                "success": false,
                "error": e
            }))
        }
    }
}

// Search posts by title, summary, tags, and content
async fn search_posts(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let query = params.q.unwrap_or_default().trim().to_lowercase();
    if query.is_empty() {
        return Json(serde_json::json!({ "results": [] }));
    }

    let site_config = SiteConfig::default();
    let posts = get_posts(&site_config);
    let mut results: Vec<SearchResult> = Vec::new();

    for post in posts {
        let haystack = format!(
            "{} {} {} {}",
            post.title,
            post.summary,
            post.tags.join(" "),
            strip_html_tags(&post.content)
        )
        .to_lowercase();

        if haystack.contains(&query) {
            results.push(SearchResult {
                title: post.title,
                slug: post.slug,
                summary: post.summary,
                date: post.date.clone(),
                date_iso: post.date_iso,
                tags: post.tags,
                reading_time: post.reading_time,
            });
        }
    }

    Json(serde_json::json!({ "results": results }))
}

// ============================================================================
// Admin Panel - Routes & Handlers
// ============================================================================

// Helper to check if request has valid session
async fn is_authenticated(headers: &HeaderMap, state: &AdminState) -> bool {
    if let Some(cookie) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            for part in cookie_str.split(';') {
                let part = part.trim();
                if part.starts_with("session=") {
                    let token = &part[8..];
                    let sessions = state.sessions.read().await;
                    if let Some(session) = sessions.get(token) {
                        let now = Instant::now();
                        if now.duration_since(session.created_at) < Duration::from_secs(SESSION_DURATION_SECS) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

// Add secure headers to response
fn add_security_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(
        header::X_FRAME_OPTIONS,
        "DENY".parse().unwrap(),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        "nosniff".parse().unwrap(),
    );
    headers.insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'self'; script-src 'self' 'unsafe-inline' fonts.googleapis.com; style-src 'self' 'unsafe-inline' fonts.googleapis.com; font-src 'self' fonts.gstatic.com; img-src 'self' data: https:".parse().unwrap(),
    );
    response
}

// Get client IP for rate limiting
fn get_client_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("unknown").to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

// Admin login page
async fn admin_login_page(
    headers: HeaderMap,
    State(state): State<AdminState>,
) -> impl IntoResponse {
    // If already logged in, redirect to dashboard
    if is_authenticated(&headers, &state).await {
        return Redirect::to("/admin/dashboard").into_response();
    }
    
    add_security_headers(Html(ADMIN_LOGIN_HTML.to_string()).into_response())
}

// Admin login submit
async fn admin_login_submit(
    headers: HeaderMap,
    State(state): State<AdminState>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let client_ip = get_client_ip(&headers);
    
    // Check rate limiting
    {
        let mut attempts = state.login_attempts.write().await;
        if let Some(data) = attempts.get_mut(&client_ip) {
            // Check if locked
            if let Some(locked_until) = data.locked_until {
                if Instant::now() < locked_until {
                    let remaining = locked_until.duration_since(Instant::now()).as_secs();
                    return Html(format!(
                        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Locked</title></head>
                        <body style="background:#000;color:#fff;font-family:system-ui;display:flex;justify-content:center;align-items:center;height:100vh;margin:0">
                        <div style="text-align:center"><h1>Too Many Attempts</h1><p>Try again in {} seconds</p><a href="/admin" style="color:#666">Back</a></div>
                        </body></html>"#, remaining
                    )).into_response();
                } else {
                    // Reset after lockout expires
                    data.attempts = 0;
                    data.locked_until = None;
                }
            }
            
            // Reset if first attempt was more than lockout duration ago
            if Instant::now().duration_since(data.first_attempt) > Duration::from_secs(LOCKOUT_DURATION_SECS) {
                data.attempts = 0;
                data.first_attempt = Instant::now();
            }
        }
    }
    
    // Verify password
    if form.password == ADMIN_PASSWORD.as_str() {
        // Clear login attempts
        {
            let mut attempts = state.login_attempts.write().await;
            attempts.remove(&client_ip);
        }
        
        // Create session
        let token = Uuid::new_v4().to_string();
        {
            let mut sessions = state.sessions.write().await;
            sessions.insert(token.clone(), SessionData {
                created_at: Instant::now(),
                last_activity: Instant::now(),
            });
        }
        
        // Set cookie and redirect
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "/admin/dashboard")
            .header("Set-Cookie", format!("session={}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}", token, SESSION_DURATION_SECS))
            .body("".to_string())
            .unwrap()
            .into_response()
    } else {
        // Record failed attempt
        {
            let mut attempts = state.login_attempts.write().await;
            let data = attempts.entry(client_ip.clone()).or_insert(LoginAttemptData {
                attempts: 0,
                first_attempt: Instant::now(),
                locked_until: None,
            });
            data.attempts += 1;
            
            if data.attempts >= MAX_LOGIN_ATTEMPTS {
                data.locked_until = Some(Instant::now() + Duration::from_secs(LOCKOUT_DURATION_SECS));
            }
        }
        
        Html(format!(
            r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Login Failed</title></head>
            <body style="background:#000;color:#fff;font-family:system-ui;display:flex;justify-content:center;align-items:center;height:100vh;margin:0">
            <div style="text-align:center"><h1>Invalid Password</h1><a href="/admin" style="color:#666">Try Again</a></div>
            </body></html>"#
        )).into_response()
    }
}

// Admin logout
async fn admin_logout(State(state): State<AdminState>, headers: HeaderMap) -> impl IntoResponse {
    // Remove session
    if let Some(cookie) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            for part in cookie_str.split(';') {
                let part = part.trim();
                if part.starts_with("session=") {
                    let token = &part[8..];
                    let mut sessions = state.sessions.write().await;
                    sessions.remove(token);
                }
            }
        }
    }
    
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/admin")
        .header("Set-Cookie", "session=; Path=/; HttpOnly; Max-Age=0")
        .body("".to_string())
        .unwrap()
}

// Admin dashboard
async fn admin_dashboard(
    headers: HeaderMap,
    State(state): State<AdminState>,
) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Redirect::to("/admin").into_response();
    }
    
    let site_config = SiteConfig::default();
    let posts = get_posts(&site_config);
    let github_links = state.github_links.read().await;
    
    let mut posts_html = String::from("<table class=\"posts-table\"><thead><tr><th>Title</th><th>Date</th><th>Read Time</th><th style=\"text-align:right\">Actions</th></tr></thead><tbody>");
    
    if posts.is_empty() {
        posts_html.push_str("<tr><td colspan=\"4\" style=\"text-align:center;padding:3rem\"><div class=\"empty-state\"><div class=\"empty-state-icon\">📝</div><div>No posts yet</div></div></td></tr>");
    } else {
        for post in &posts {
            let is_github_linked = github_links.contains_key(&post.slug);
            let github_badge = if is_github_linked {
                " <span style=\"background:#1a1a1a;padding:2px 6px;border-radius:3px;font-size:11px;margin-left:8px\">GitHub</span>"
            } else {
                ""
            };
            
            posts_html.push_str(&format!(
                "<tr><td><div class=\"post-title\">{}{}</div></td><td class=\"post-meta\">{}</td><td class=\"post-meta\">{} min</td><td style=\"text-align:right\"><div class=\"table-actions\"><a href=\"/admin/edit/{}\" class=\"btn-sm\">Edit</a><button onclick=\"deletePost('{}')\" class=\"btn-sm btn-danger\">Delete</button></div></td></tr>",
                html_escape::encode_text(&post.title),
                github_badge,
                post.date,
                post.reading_time,
                post.slug,
                post.slug
            ));
        }
    }
    
    posts_html.push_str("</tbody></table>");
    
    let dashboard_html = ADMIN_DASHBOARD_HTML
        .replace("{{posts_count}}", &posts.len().to_string())
        .replace("{{posts_list}}", &posts_html);
    
    Html(dashboard_html).into_response()
}

// Admin new post page
async fn admin_new_post(headers: HeaderMap, State(state): State<AdminState>) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Redirect::to("/admin").into_response();
    }
    Html(ADMIN_EDITOR_HTML.replace("{{mode}}", "new").replace("{{title}}", "").replace("{{content}}", "").replace("{{tags}}", "").replace("{{summary}}", "").replace("{{slug}}", "")).into_response()
}

// Admin edit post page
async fn admin_edit_post(
    headers: HeaderMap,
    State(state): State<AdminState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Redirect::to("/admin").into_response();
    }
    
    // Find the post file
    let content_path = PathBuf::from("content");
    let mut post_content = String::new();
    let mut post_title = String::new();
    let mut post_tags = String::new();
    let mut post_summary = String::new();
    let mut found_file_slug = String::new();
    
    if let Ok(entries) = fs::read_dir(&content_path) {
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Some(meta) = parse_metadata(&content) {
                    // Generate slug from title (same way as get_posts)
                    let title_slug = meta.title.to_lowercase()
                        .chars()
                        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join("-");
                    
                    let file_slug = entry.path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();
                    
                    if title_slug == slug {
                        post_title = meta.title;
                        post_tags = meta.tags.join(", ");
                        post_summary = meta.summary;
                        found_file_slug = file_slug;
                        // Get content after frontmatter
                        let parts: Vec<&str> = content.splitn(3, "---").collect();
                        if parts.len() >= 3 {
                            post_content = parts[2].trim().to_string();
                        }
                        break;
                    }
                }
            }
        }
    }
    
    Html(ADMIN_EDITOR_HTML
        .replace("{{mode}}", "edit")
        .replace("{{title}}", &html_escape::encode_text(&post_title))
        .replace("{{content}}", &html_escape::encode_text(&post_content))
        .replace("{{tags}}", &html_escape::encode_text(&post_tags))
        .replace("{{summary}}", &html_escape::encode_text(&post_summary))
        .replace("{{slug}}", &found_file_slug)
    ).into_response()
}

// Save post (create or update)
async fn admin_save_post(
    headers: HeaderMap,
    State(state): State<AdminState>,
    Form(form): Form<PostForm>,
) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Json(serde_json::json!({"success": false, "error": "Not authenticated"})).into_response();
    }
    
    let slug = form.slug.clone().unwrap_or_else(|| {
        form.title.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    });
    
    // Check if editing existing post to preserve date and github_repo
    let file_path = format!("content/{}.md", slug);
    let (existing_date, existing_github_repo, existing_website) = if let Ok(existing_content) = fs::read_to_string(&file_path) {
        let mut date = None;
        let mut github_repo = None;
        let mut website = None;
        if let Some((front_matter, _)) = existing_content.strip_prefix("---").and_then(|s| s.split_once("---")) {
            for line in front_matter.lines() {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim().trim_matches('"');
                    if key == "date" {
                        date = Some(value.to_string());
                    } else if key == "github_repo" {
                        github_repo = Some(value.to_string());
                    } else if key == "website" {
                        website = Some(value.to_string());
                    }
                }
            }
        }
        (date, github_repo, website)
    } else {
        (None, None, None)
    };
    
    let date = existing_date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    
    let github_repo_line = existing_github_repo
        .map(|repo| format!("\ngithub_repo: \"{}\"", repo))
        .unwrap_or_default();
    let website_line = existing_website
        .map(|site| format!("\nwebsite: \"{}\"", site))
        .unwrap_or_default();
    
    let markdown_content = format!(
        r#"---
title: "{}"
date: "{}"
tags: [{}]
summary: "{}"{github_repo}{website}
---

{}"#,
        form.title,
        date,
        form.tags.split(',').map(|t| format!("\"{}\"", t.trim())).collect::<Vec<_>>().join(", "),
        form.summary,
    form.content,
    github_repo = github_repo_line,
    website = website_line
    );
    
    if let Err(e) = fs::write(&file_path, markdown_content) {
        return Json(serde_json::json!({"success": false, "error": e.to_string()})).into_response();
    }
    
    Json(serde_json::json!({"success": true, "slug": slug})).into_response()
}

// Delete post
async fn admin_delete_post(
    headers: HeaderMap,
    State(state): State<AdminState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Json(serde_json::json!({"success": false, "error": "Not authenticated"}));
    }
    
    let file_path = format!("content/{}.md", slug);
    
    // Also try github- prefix
    let github_file_path = format!("content/github-{}.md", slug);
    
    let result = fs::remove_file(&file_path).or_else(|_| fs::remove_file(&github_file_path));
    
    match result {
        Ok(_) => {
            // Remove from GitHub links if exists
            let mut links = state.github_links.write().await;
            links.remove(&slug);
            state.save_github_links(&links);
            
            Json(serde_json::json!({"success": true}))
        }
        Err(e) => Json(serde_json::json!({"success": false, "error": e.to_string()}))
    }
}

// Admin GitHub import page
async fn admin_github_page(headers: HeaderMap, State(state): State<AdminState>) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Redirect::to("/admin").into_response();
    }
    Html(ADMIN_GITHUB_HTML.to_string()).into_response()
}

// List GitHub repos as JSON (for admin)
async fn admin_list_repos(headers: HeaderMap, State(state): State<AdminState>) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Json(serde_json::json!({"success": false, "error": "Not authenticated"}));
    }
    
    match fetch_github_repos().await {
        Ok(repos) => {
            let github_links = state.github_links.read().await;
            let repo_list: Vec<serde_json::Value> = repos.iter().map(|r| {
                let is_linked = github_links.values().any(|link| link.repo_name == r.name);
                serde_json::json!({
                    "name": r.name,
                    "full_name": r.full_name,
                    "description": r.description,
                    "url": r.html_url,
                    "language": r.language,
                    "stars": r.stargazers_count,
                    "updated": r.pushed_at,
                    "is_linked": is_linked
                })
            }).collect();
            
            Json(serde_json::json!({
                "success": true,
                "repos": repo_list
            }))
        }
        Err(e) => Json(serde_json::json!({"success": false, "error": e}))
    }
}

// Import a single GitHub repo
async fn admin_import_repo(
    headers: HeaderMap,
    State(state): State<AdminState>,
    Form(form): Form<GitHubImportForm>,
) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Json(serde_json::json!({"success": false, "error": "Not authenticated"}));
    }
    
    // Fetch repo info
    let repos = match fetch_github_repos().await {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({"success": false, "error": e})),
    };
    
    let repo = match repos.iter().find(|r| r.name == form.repo_name) {
        Some(r) => r,
        None => return Json(serde_json::json!({"success": false, "error": "Repo not found"})),
    };
    
    // Fetch README
    let readme_content = match fetch_readme(&form.repo_name).await {
        Ok(c) => c,
        Err(e) => return Json(serde_json::json!({"success": false, "error": e})),
    };
    
    // Create post
    let post_content = create_post_from_readme(repo, &readme_content);
    let slug = format!("github-{}", form.repo_name.to_lowercase());
    let file_path = format!("content/{}.md", slug);
    
    if let Err(e) = fs::write(&file_path, &post_content) {
        return Json(serde_json::json!({"success": false, "error": e.to_string()}));
    }
    
    // Save link
    let auto_sync = form.auto_sync.as_deref() == Some("on");
    {
        let mut links = state.github_links.write().await;
        links.insert(slug.clone(), GitHubLink {
            repo_name: form.repo_name.clone(),
            repo_full_name: repo.full_name.clone(),
            last_synced: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            auto_sync,
        });
        state.save_github_links(&links);
    }
    
    Json(serde_json::json!({
        "success": true,
        "slug": slug,
        "message": format!("Imported {} successfully", form.repo_name)
    }))
}

// Sync a specific linked repo
async fn admin_sync_repo(
    headers: HeaderMap,
    State(state): State<AdminState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    if !is_authenticated(&headers, &state).await {
        return Json(serde_json::json!({"success": false, "error": "Not authenticated"}));
    }
    
    let link = {
        let links = state.github_links.read().await;
        links.get(&slug).cloned()
    };
    
    let link = match link {
        Some(l) => l,
        None => return Json(serde_json::json!({"success": false, "error": "Not a GitHub linked post"})),
    };
    
    // Fetch repo info
    let repos = match fetch_github_repos().await {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({"success": false, "error": e})),
    };
    
    let repo = match repos.iter().find(|r| r.name == link.repo_name) {
        Some(r) => r,
        None => return Json(serde_json::json!({"success": false, "error": "Repo not found"})),
    };
    
    // Fetch README
    let readme_content = match fetch_readme(&link.repo_name).await {
        Ok(c) => c,
        Err(e) => return Json(serde_json::json!({"success": false, "error": e})),
    };
    
    // Update post
    let post_content = create_post_from_readme(repo, &readme_content);
    let file_path = format!("content/{}.md", slug);
    
    if let Err(e) = fs::write(&file_path, &post_content) {
        return Json(serde_json::json!({"success": false, "error": e.to_string()}));
    }
    
    // Update sync time
    {
        let mut links = state.github_links.write().await;
        if let Some(link) = links.get_mut(&slug) {
            link.last_synced = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }
        state.save_github_links(&links);
    }
    
    Json(serde_json::json!({"success": true, "message": "Synced successfully"}))
}

// GitHub webhook handler for auto-sync
async fn github_webhook(
    State(state): State<AdminState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // Check if this is a push to main/master
    let is_main_push = payload.git_ref
        .as_ref()
        .map(|r| r == "refs/heads/main" || r == "refs/heads/master")
        .unwrap_or(false);
    
    if !is_main_push {
        return Json(serde_json::json!({"success": true, "message": "Ignoring non-main branch push"}));
    }
    
    let repo_name = match payload.repository {
        Some(r) => r.name,
        None => return Json(serde_json::json!({"success": false, "error": "No repository in payload"})),
    };
    
    // Find linked post with auto_sync enabled
    let slug = {
        let links = state.github_links.read().await;
        links.iter()
            .find(|(_, link)| link.repo_name == repo_name && link.auto_sync)
            .map(|(slug, _)| slug.clone())
    };
    
    let slug = match slug {
        Some(s) => s,
        None => return Json(serde_json::json!({"success": true, "message": "No auto-sync configured for this repo"})),
    };
    
    // Fetch and update
    let repos = match fetch_github_repos().await {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({"success": false, "error": e})),
    };
    
    let repo = match repos.iter().find(|r| r.name == repo_name) {
        Some(r) => r,
        None => return Json(serde_json::json!({"success": false, "error": "Repo not found"})),
    };
    
    let readme_content = match fetch_readme(&repo_name).await {
        Ok(c) => c,
        Err(e) => return Json(serde_json::json!({"success": false, "error": e})),
    };
    
    let post_content = create_post_from_readme(repo, &readme_content);
    let file_path = format!("content/{}.md", slug);
    
    if let Err(e) = fs::write(&file_path, &post_content) {
        return Json(serde_json::json!({"success": false, "error": e.to_string()}));
    }
    
    // Update sync time
    {
        let mut links = state.github_links.write().await;
        if let Some(link) = links.get_mut(&slug) {
            link.last_synced = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }
        state.save_github_links(&links);
    }
    
    println!("Webhook: Auto-synced {} from GitHub", slug);
    
    Json(serde_json::json!({"success": true, "message": format!("Auto-synced {}", slug)}))
}

// ============================================================================
// Admin Panel - HTML Templates (embedded for simplicity)
// ============================================================================

const ADMIN_LOGIN_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Admin Login</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background: #000;
            color: #fff;
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 1rem;
        }
        .login-container {
            width: 100%;
            max-width: 400px;
        }
        .login-header {
            text-align: center;
            margin-bottom: 2rem;
        }
        .login-header h1 {
            font-size: 1.75rem;
            font-weight: 600;
            letter-spacing: -0.01em;
            margin-bottom: 0.5rem;
        }
        .login-header p {
            font-size: 0.875rem;
            color: #a0a0a0;
        }
        .login-form {
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }
        .form-group {
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
        }
        .form-group label {
            font-size: 0.875rem;
            font-weight: 500;
            color: #fff;
        }
        input[type="password"] {
            padding: 0.75rem;
            background: #0a0a0a;
            border: 1px solid #1a1a1a;
            color: #fff;
            font-size: 0.9375rem;
            border-radius: 6px;
            transition: all 0.15s ease;
            font-family: inherit;
        }
        input[type="password"]:focus {
            outline: none;
            border-color: #444;
            background: #0f0f0f;
        }
        .submit-btn {
            padding: 0.75rem;
            background: #fff;
            color: #000;
            border: none;
            font-size: 0.9375rem;
            font-weight: 600;
            border-radius: 6px;
            cursor: pointer;
            transition: background 0.15s ease;
            font-family: inherit;
            margin-top: 0.5rem;
        }
        .submit-btn:hover {
            background: #f0f0f0;
        }
        .login-footer {
            text-align: center;
            margin-top: 1.5rem;
        }
        .login-footer a {
            color: #666;
            text-decoration: none;
            font-size: 0.8125rem;
            transition: color 0.15s ease;
        }
        .login-footer a:hover {
            color: #999;
        }
    </style>
</head>
<body>
    <div class="login-container">
        <div class="login-header">
            <h1>Admin</h1>
            <p>Secure access required</p>
        </div>
        <form method="POST" action="/admin/login" class="login-form">
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required autofocus placeholder="Enter admin password">
            </div>
            <button type="submit" class="submit-btn">Sign In</button>
        </form>
        <div class="login-footer">
            <a href="/">← Back to Blog</a>
        </div>
    </div>
</body>
</html>"#;

const ADMIN_DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Admin Dashboard</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background: #000;
            color: #fff;
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        
        .topbar {
            border-bottom: 1px solid #1a1a1a;
            padding: 1.25rem 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            position: sticky;
            top: 0;
            background: rgba(0,0,0,0.95);
            backdrop-filter: blur(10px);
            z-index: 100;
        }
        .topbar-left { display: flex; align-items: center; gap: 1rem; }
        .topbar-title { font-size: 1rem; font-weight: 600; letter-spacing: -0.01em; }
        .topbar-right { display: flex; gap: 0.75rem; }
        .topbar-link {
            color: #a0a0a0;
            text-decoration: none;
            font-size: 0.8125rem;
            font-weight: 500;
            padding: 0.5rem 0.75rem;
            border-radius: 4px;
            transition: all 0.15s ease;
        }
        .topbar-link:hover { color: #fff; background: #1a1a1a; }
        
        .main {
            flex: 1;
            max-width: 1000px;
            width: 100%;
            margin: 0 auto;
            padding: 2rem 1.5rem;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }
        .stat-card {
            background: #0a0a0a;
            border: 1px solid #1a1a1a;
            border-radius: 8px;
            padding: 1.5rem;
        }
        .stat-value { font-size: 2rem; font-weight: 700; letter-spacing: -0.02em; }
        .stat-label { font-size: 0.75rem; color: #666; margin-top: 0.5rem; text-transform: uppercase; letter-spacing: 0.05em; }
        
        .section {
            margin-bottom: 2rem;
        }
        .section-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1rem;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid #1a1a1a;
        }
        .section-title { font-size: 0.875rem; font-weight: 600; color: #fff; }
        .btn-group { display: flex; gap: 0.5rem; }
        
        .btn {
            padding: 0.625rem 1rem;
            font-size: 0.8125rem;
            font-weight: 500;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            transition: all 0.15s ease;
            font-family: inherit;
            display: inline-flex;
            align-items: center;
            gap: 0.375rem;
            text-decoration: none;
        }
        .btn-primary {
            background: #fff;
            color: #000;
        }
        .btn-primary:hover { background: #f0f0f0; }
        .btn-secondary {
            background: #1a1a1a;
            color: #a0a0a0;
            border: 1px solid #333;
        }
        .btn-secondary:hover { background: #222; color: #fff; }
        
        .posts-table {
            width: 100%;
            border-collapse: collapse;
            background: #0a0a0a;
            border: 1px solid #1a1a1a;
            border-radius: 8px;
            overflow: hidden;
        }
        .posts-table thead {
            background: #0f0f0f;
            border-bottom: 1px solid #1a1a1a;
        }
        .posts-table th {
            padding: 0.875rem;
            text-align: left;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: #666;
        }
        .posts-table td {
            padding: 0.875rem;
            border-top: 1px solid #1a1a1a;
            font-size: 0.9375rem;
        }
        .post-title { font-weight: 600; color: #fff; }
        .post-meta { font-size: 0.8125rem; color: #666; }
        .table-actions { display: flex; gap: 0.5rem; }
        .btn-sm {
            padding: 0.375rem 0.75rem;
            font-size: 0.75rem;
            background: #1a1a1a;
            color: #a0a0a0;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .btn-sm:hover { background: #2a2a2a; color: #fff; }
        .btn-danger { color: #ff6b6b; }
        .btn-danger:hover { background: #330000; }
        
        .empty-state {
            text-align: center;
            padding: 3rem 1rem;
            color: #666;
        }
        .empty-state-icon {
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
            opacity: 0.3;
        }
        
        @media (max-width: 640px) {
            .main { padding: 1rem; }
            .stats-grid { grid-template-columns: 1fr; }
            .section-header { flex-direction: column; align-items: flex-start; gap: 0.75rem; }
            .btn-group { width: 100%; flex-wrap: wrap; }
        }
    </style>
</head>
<body>
    <div class="topbar">
        <div class="topbar-left">
            <span class="topbar-title">Admin Dashboard</span>
        </div>
        <div class="topbar-right">
            <a href="/" class="topbar-link">← View Blog</a>
            <a href="/admin/logout" class="topbar-link">Logout</a>
        </div>
    </div>
    
    <div class="main">
        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-value">{{posts_count}}</div>
                <div class="stat-label">Total Posts</div>
            </div>
        </div>
        
        <div class="section">
            <div class="section-header">
                <span class="section-title">Posts</span>
                <div class="btn-group">
                    <a href="/admin/github" class="btn btn-secondary">GitHub</a>
                    <a href="/admin/new" class="btn btn-primary">+ New Post</a>
                </div>
            </div>
            <div>{{posts_list}}</div>
        </div>
    </div>
    
    <script>
        async function deletePost(slug) {
            if (!confirm('Delete this post?')) return;
            const res = await fetch('/admin/delete/' + slug, { method: 'DELETE' });
            const data = await res.json();
            if (data.success) location.reload();
            else alert('Error: ' + data.error);
        }
    </script>
</body>
</html>"#;

const ADMIN_EDITOR_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Edit Post</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background: #000;
            color: #fff;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            min-height: 100vh;
        }
        .header {
            border-bottom: 1px solid #1a1a1a;
            padding: 16px 24px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            position: sticky;
            top: 0;
            background: #000;
            z-index: 100;
        }
        .header h1 {
            font-size: 16px;
            font-weight: 600;
        }
        .header-actions {
            display: flex;
            gap: 12px;
        }
        .btn {
            background: #fff;
            color: #000;
            padding: 8px 16px;
            font-size: 13px;
            font-weight: 500;
            border-radius: 4px;
            text-decoration: none;
            border: none;
            cursor: pointer;
        }
        .btn:hover {
            background: #e0e0e0;
        }
        .btn-secondary {
            background: transparent;
            color: #666;
            border: 1px solid #333;
        }
        .btn-secondary:hover {
            color: #fff;
            border-color: #555;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            padding: 32px 24px;
        }
        .form-group {
            margin-bottom: 24px;
        }
        label {
            display: block;
            font-size: 11px;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: #666;
            margin-bottom: 8px;
        }
        input, textarea {
            width: 100%;
            background: #0a0a0a;
            border: 1px solid #222;
            color: #fff;
            padding: 12px;
            font-size: 14px;
            border-radius: 4px;
            outline: none;
            font-family: inherit;
        }
        input:focus, textarea:focus {
            border-color: #444;
        }
        textarea {
            min-height: 400px;
            resize: vertical;
            font-family: 'SF Mono', 'Fira Code', monospace;
            font-size: 13px;
            line-height: 1.6;
        }
        .hint {
            font-size: 12px;
            color: #666;
            margin-top: 6px;
        }
        .toast {
            position: fixed;
            bottom: 24px;
            right: 24px;
            background: #1a1a1a;
            color: #fff;
            padding: 12px 20px;
            border-radius: 4px;
            font-size: 14px;
            display: none;
        }
        .toast.show {
            display: block;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>{{mode}} Post</h1>
        <div class="header-actions">
            <a href="/admin/dashboard" class="btn btn-secondary">Cancel</a>
            <button type="button" onclick="savePost()" class="btn">Save Post</button>
        </div>
    </div>
    <div class="container">
        <form id="postForm">
            <input type="hidden" name="slug" value="{{slug}}">
            
            <div class="form-group">
                <label>Title</label>
                <input type="text" name="title" value="{{title}}" required placeholder="Enter post title">
            </div>
            
            <div class="form-group">
                <label>Summary</label>
                <input type="text" name="summary" value="{{summary}}" placeholder="Brief description for SEO">
            </div>
            
            <div class="form-group">
                <label>Tags</label>
                <input type="text" name="tags" value="{{tags}}" placeholder="tag1, tag2, tag3">
                <div class="hint">Comma-separated list of tags</div>
            </div>
            
            <div class="form-group">
                <label>Content (Markdown)</label>
                <textarea name="content" placeholder="Write your post in Markdown...">{{content}}</textarea>
            </div>
        </form>
    </div>
    
    <div class="toast" id="toast"></div>
    
    <script>
        async function savePost() {
            const form = document.getElementById('postForm');
            const formData = new FormData(form);
            
            const res = await fetch('/admin/save', {
                method: 'POST',
                body: new URLSearchParams(formData)
            });
            
            const data = await res.json();
            
            if (data.success) {
                showToast('Post saved successfully!');
                setTimeout(() => {
                    window.location.href = '/admin/dashboard';
                }, 1000);
            } else {
                showToast('Error: ' + data.error);
            }
        }
        
        function showToast(msg) {
            const toast = document.getElementById('toast');
            toast.textContent = msg;
            toast.classList.add('show');
            setTimeout(() => toast.classList.remove('show'), 3000);
        }
        
        // Keyboard shortcut: Cmd/Ctrl + S to save
        document.addEventListener('keydown', (e) => {
            if ((e.metaKey || e.ctrlKey) && e.key === 's') {
                e.preventDefault();
                savePost();
            }
        });
    </script>
</body>
</html>"#;

const ADMIN_GITHUB_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Import from GitHub</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background: #000;
            color: #fff;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            min-height: 100vh;
        }
        .header {
            border-bottom: 1px solid #1a1a1a;
            padding: 16px 24px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .header h1 {
            font-size: 16px;
            font-weight: 600;
        }
        .btn {
            background: #fff;
            color: #000;
            padding: 8px 16px;
            font-size: 13px;
            font-weight: 500;
            border-radius: 4px;
            text-decoration: none;
            border: none;
            cursor: pointer;
        }
        .btn:hover {
            background: #e0e0e0;
        }
        .btn-secondary {
            background: transparent;
            color: #666;
            border: 1px solid #333;
        }
        .btn-secondary:hover {
            color: #fff;
            border-color: #555;
        }
        .btn-small {
            padding: 6px 12px;
            font-size: 12px;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            padding: 32px 24px;
        }
        .info {
            background: #0a0a0a;
            border: 1px solid #1a1a1a;
            border-radius: 4px;
            padding: 16px;
            margin-bottom: 24px;
            font-size: 13px;
            color: #999;
        }
        .repos-list {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .repo {
            background: #0a0a0a;
            border: 1px solid #1a1a1a;
            border-radius: 4px;
            padding: 16px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .repo-info h3 {
            font-size: 14px;
            font-weight: 500;
            margin-bottom: 4px;
        }
        .repo-info p {
            font-size: 12px;
            color: #666;
            margin-bottom: 4px;
        }
        .repo-meta {
            font-size: 11px;
            color: #444;
            display: flex;
            gap: 12px;
        }
        .repo-actions {
            display: flex;
            gap: 8px;
            align-items: center;
        }
        .checkbox-label {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 12px;
            color: #666;
        }
        .checkbox-label input {
            width: 14px;
            height: 14px;
        }
        .loading {
            text-align: center;
            padding: 40px;
            color: #666;
        }
        .linked {
            background: #0d1a0d;
            border-color: #1a2e1a;
        }
        .linked-badge {
            background: #1a2e1a;
            color: #4a8;
            padding: 2px 6px;
            border-radius: 2px;
            font-size: 10px;
            margin-left: 8px;
        }
        .toast {
            position: fixed;
            bottom: 24px;
            right: 24px;
            background: #1a1a1a;
            color: #fff;
            padding: 12px 20px;
            border-radius: 4px;
            font-size: 14px;
            display: none;
        }
        .toast.show {
            display: block;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>Import from GitHub</h1>
        <a href="/admin/dashboard" class="btn btn-secondary">Back to Dashboard</a>
    </div>
    <div class="container">
        <div class="info">
            <strong>Auto-sync:</strong> Enable auto-sync to automatically update blog posts when you push changes to your GitHub repository. 
            Set up a webhook in your GitHub repo settings pointing to <code>/api/webhook/github</code> to enable this feature.
        </div>
        
        <div id="repos" class="repos-list">
            <div class="loading">Loading repositories...</div>
        </div>
    </div>
    
    <div class="toast" id="toast"></div>
    
    <script>
        async function loadRepos() {
            const res = await fetch('/admin/api/repos');
            const data = await res.json();
            
            if (!data.success) {
                document.getElementById('repos').innerHTML = '<div class="loading">Error: ' + data.error + '</div>';
                return;
            }
            
            const html = data.repos.map(repo => `
                <div class="repo ${repo.is_linked ? 'linked' : ''}">
                    <div class="repo-info">
                        <h3>${repo.name}${repo.is_linked ? '<span class="linked-badge">Linked</span>' : ''}</h3>
                        <p>${repo.description || 'No description'}</p>
                        <div class="repo-meta">
                            <span>${repo.language || 'Unknown'}</span>
                            <span>${repo.stars} stars</span>
                        </div>
                    </div>
                    <div class="repo-actions">
                        ${repo.is_linked ? `
                            <button class="btn btn-small btn-secondary" onclick="syncRepo('${repo.name}')">Sync Now</button>
                        ` : `
                            <label class="checkbox-label">
                                <input type="checkbox" id="auto-${repo.name}">
                                Auto-sync
                            </label>
                            <button class="btn btn-small" onclick="importRepo('${repo.name}')">Import</button>
                        `}
                    </div>
                </div>
            `).join('');
            
            document.getElementById('repos').innerHTML = html;
        }
        
        async function importRepo(name) {
            const autoSync = document.getElementById('auto-' + name)?.checked ? 'on' : '';
            
            const res = await fetch('/admin/github/import', {
                method: 'POST',
                headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
                body: 'repo_name=' + encodeURIComponent(name) + '&auto_sync=' + autoSync
            });
            
            const data = await res.json();
            
            if (data.success) {
                showToast('Imported successfully!');
                loadRepos();
            } else {
                showToast('Error: ' + data.error);
            }
        }
        
        async function syncRepo(name) {
            const res = await fetch('/admin/sync/github-' + name.toLowerCase(), { method: 'POST' });
            const data = await res.json();
            
            if (data.success) {
                showToast('Synced successfully!');
            } else {
                showToast('Error: ' + data.error);
            }
        }
        
        function showToast(msg) {
            const toast = document.getElementById('toast');
            toast.textContent = msg;
            toast.classList.add('show');
            setTimeout(() => toast.classList.remove('show'), 3000);
        }
        
        loadRepos();
    </script>
</body>
</html>"#;

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let mut hb = Handlebars::new();
    hb.set_strict_mode(false); // Allow missing variables
    
    if let Err(e) = hb.register_template_file("index.html", "templates/index.html") {
        eprintln!("Failed to register index template: {}", e);
        return;
    }
    if let Err(e) = hb.register_template_file("single.html", "templates/single.html") {
        eprintln!("Failed to register single template: {}", e);
        return;
    }
    let hb = Arc::new(hb);
    
    // Initialize admin state
    let admin_state = AdminState::new();

    let app = Router::new()
        // Public routes
        .route("/", get(index))
        .route("/tags/{tag}", get(tag_page))
        .route("/blog/{post_title}", get(single_post))
        // SEO routes
        .route("/sitemap.xml", get(sitemap))
        .route("/robots.txt", get(robots_txt))
        .route("/rss.xml", get(rss_feed))
        .route("/feed.xml", get(rss_feed))
        .route("/api/search", get(search_posts))
        // GitHub integration routes (public)
        .route("/api/github/repos", get(list_github_repos))
        .route("/api/github/sync", get(sync_github_repos))
        // Admin routes
        .route("/admin", get(admin_login_page))
        .route("/admin/login", post(admin_login_submit))
        .route("/admin/logout", get(admin_logout))
        .route("/admin/dashboard", get(admin_dashboard))
        .route("/admin/new", get(admin_new_post))
        .route("/admin/edit/{slug}", get(admin_edit_post))
        .route("/admin/save", post(admin_save_post))
        .route("/admin/delete/{slug}", delete(admin_delete_post))
        .route("/admin/github", get(admin_github_page))
        .route("/admin/api/repos", get(admin_list_repos))
        .route("/admin/github/import", post(admin_import_repo))
        .route("/admin/sync/{slug}", post(admin_sync_repo))
        // Webhook for GitHub auto-sync
        .route("/api/webhook/github", post(github_webhook))
        .layer(Extension(hb))
        .with_state(admin_state);

    let base_url = SiteConfig::default().url;
    println!("Server running at {}", base_url);
    println!("Blog index: {}/", base_url);
    println!("Admin panel: {}/admin", base_url);
    println!("Sitemap: {}/sitemap.xml", base_url);
    println!("RSS Feed: {}/rss.xml", base_url);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}