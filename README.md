# Axum Blog - Modern Rust Blog Engine

A high-performance, SEO-optimized blog platform built with Rust, Axum, and Handlebars. Features a clean admin panel, GitHub integration for content sync, dynamic OG image generation, and comprehensive security measures.

## Table of Contents

1. [Features](#features)
2. [Tech Stack](#tech-stack)
3. [Quick Start](#quick-start)
4. [Configuration](#configuration)
5. [Usage](#usage)
6. [Project Structure](#project-structure)
7. [Creating Blog Posts](#creating-blog-posts)
8. [Admin Panel](#admin-panel)
9. [GitHub Integration](#github-integration)
10. [API Reference](#api-reference)
11. [Security](#security)
12. [Development](#development)
13. [Deployment](#deployment)
14. [Troubleshooting](#troubleshooting)

## Features

### Core Functionality
- Clean, minimal dark UI with responsive design
- SEO-optimized with meta tags, Open Graph, Twitter Card support
- **Dynamic OG image generation** using Geist fonts with dark Vercel-style design
- Markdown-based content with syntax highlighting
- Tag-based post organization and filtering
- Full-text search across posts and content
- Multiple feed formats (RSS, Atom) with OG image references
- XML sitemap with comprehensive metadata and OG images
- Reading time estimates and word counts on all posts
- **Logo-based branding** for favicon and header

### Dynamic OG Images
- Homepage OG image at `/og.png`
- Individual post OG images at `/blog/{slug}/og.png`
- Recent posts preview at `/blog/recents.png` showing latest 4 posts
- Geist font family embedded (Medium, SemiBold, Bold)
- Dark theme with grid pattern and accent lines
- Automatic title wrapping and metadata display
- Cached for performance (1 year for posts, 1 hour for recents)

### Admin Panel
- Secure authentication with password protection
- Create, edit, and delete blog posts
- Rate limiting (5 attempts, 5-minute lockout)
- Session-based access control with 1-hour timeout
- Modern, intuitive dashboard interface
- Post management with table-based layout

### Content Management
- GitHub repository README sync to blog posts
- Automatic import of README files as blog content
- Link management between posts and GitHub repos
- Repository metadata display (stars, language, last updated)
- One-click sync for linked repositories

### SEO & Performance
- Heading anchor IDs for table of contents navigation
- Structured data (JSON-LD) for search engines
- Canonical URL tags
- OG images in sitemap, RSS, and Atom feeds
- Comprehensive robots.txt with AI bot support
- PWA manifest for app-like experience
- humans.txt and security.txt for transparency
- Fast server response times with Rust performance
- Immutable caching for static assets

## Tech Stack

- **Language**: Rust (pure Rust, no OpenSSL dependencies)
- **Web Framework**: Axum (async/await, minimal overhead)
- **TLS**: rustls-tls (pure Rust TLS implementation)
- **Templating**: Handlebars
- **Markdown Processing**: pulldown-cmark with syntax highlighting
- **Syntax Highlighting**: Syntect
- **SVG Rendering**: resvg 0.45, usvg 0.45 for OG image generation
- **Image Encoding**: tiny-skia 0.11 for PNG generation
- **Fonts**: Geist (Medium, SemiBold, Bold) embedded in binary
- **Serialization**: Serde
- **HTTP Client**: Reqwest with rustls-tls
- **Async Runtime**: Tokio
- **Configuration**: Dotenvy for environment variables

## Quick Start

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)

### Installation

1. Clone or extract the repository:
```bash
cd axum-blog
```

2. Create environment configuration:
```bash
cp .env.example .env
```

3. Edit `.env` with your settings:
```bash
SITE_URL=http://localhost:8080
ADMIN_PASSWORD=your_secure_password
GITHUB_USERNAME=your_username
# Optional: add GitHub token for higher API limits
GITHUB_TOKEN=your_github_token
```

4. Run the development server:
```bash
cargo run
```

Server will start at: http://localhost:8080

### Build for Production

```bash
cargo build --release
./target/release/axum-blog
```

## Configuration

### Environment Variables

Create a `.env` file in the project root with the following variables:

**SITE_URL** (required)
- Base URL for your blog
- Used for canonical links and RSS feed
- Example: `http://localhost:8080` or `https://myblog.com`

**ADMIN_PASSWORD** (required)
- Password for admin panel access
- Should be strong and unique
- Example: `your_secure_password_here`

**GITHUB_USERNAME** (optional)
- Your GitHub username for repository integration
- Used for fetching your public repositories
- Example: `aryansrao`

**GITHUB_TOKEN** (optional)
- GitHub personal access token
- Increases API rate limits from 60 to 5000 requests/hour
- Create at: https://github.com/settings/tokens
- Requires: `repo` and `public_repo` scopes

### Security Headers

The application automatically sends these security headers:

- X-Frame-Options: DENY (prevents clickjacking)
- X-Content-Type-Options: nosniff (blocks MIME sniffing)
- X-XSS-Protection: 1; mode=block (enables XSS filtering)
- Strict-Transport-Security: max-age=31536000 (HSTS)
- Content-Security-Policy (controls resource loading)

## Usage

### Accessing the Blog

**Public Pages**
- Homepage: http://localhost:8080/
- Individual Post: http://localhost:8080/blog/{slug}
- Tag Page: http://localhost:8080/tags/{tag-name}
- RSS Feed: http://localhost:8080/rss.xml
- Atom Feed: http://localhost:8080/atom.xml
- Sitemap: http://localhost:8080/sitemap.xml
- Posts Sitemap: http://localhost:8080/sitemap-posts.xml
- Robots.txt: http://localhost:8080/robots.txt

**Dynamic OG Images**
- Homepage OG: http://localhost:8080/og.png
- Post OG: http://localhost:8080/blog/{slug}/og.png
- Recent Posts: http://localhost:8080/blog/recents.png

**SEO & Metadata**
- PWA Manifest: http://localhost:8080/manifest.json
- Humans.txt: http://localhost:8080/humans.txt
- Security.txt: http://localhost:8080/.well-known/security.txt

**Search**
- API Endpoint: http://localhost:8080/api/search?q={query}
- Index page: Search all posts by title, tags, summary, and content
- Post page: Search within the current post content only

**Redirects**
- `/blog` and `/blog/` redirect to homepage

### Admin Panel Access

1. Navigate to: http://localhost:8080/admin
2. Enter your ADMIN_PASSWORD
3. Access the dashboard to manage posts

### Admin Features

- Create new posts with title, content, tags, and summary
- Edit existing posts while preserving metadata
- Delete posts with confirmation
- View all posts in a table with edit/delete options
- Import GitHub repositories as blog posts
- Sync linked repositories for content updates

## Project Structure

```
axum-blog/
├── src/
│   └── main.rs              # Main application file (~3400 lines)
├── templates/
│   ├── index.html           # Homepage template
│   └── single.html          # Single post template
├── content/
│   └── *.md                 # Blog post markdown files
├── fonts/
│   ├── Geist-Medium.ttf     # Embedded font for OG images
│   ├── Geist-SemiBold.ttf   # Embedded font for OG images
│   └── Geist-Bold.ttf       # Embedded font for OG images
├── logo.png                 # Site logo and favicon
├── Cargo.toml               # Project dependencies
├── leapcell.yaml            # Leapcell deployment config
├── .env.example             # Environment configuration template
├── README.md                # This file
└── .gitignore               # Git ignore rules
```

### Content Directory

The `content/` directory stores all blog posts as Markdown files. Each file follows this naming convention:

```
{slug}.md
```

Examples:
- `getting-started.md`
- `rust-async-guide.md`
- `github-project-name.md` (for imported repos)

## Creating Blog Posts

### Post Format

Blog posts are Markdown files with YAML front matter:

```markdown
---
title: Your Post Title
summary: Brief description for search results and RSS feed
author: Your Name
tags: rust,web,programming
image: https://example.com/image.jpg
date: 2024-12-19
---

# Your Post Title

Your content here in Markdown format...

## Section Heading

More content with **bold**, *italic*, and `code`.

```

### Front Matter Fields

| Field | Required | Description |
|-------|----------|-------------|
| title | Yes | Post title, used in page title and meta tags |
| summary | Yes | Short description for search results and feed |
| author | Yes | Author name |
| tags | Yes | Comma-separated tags for categorization |
| image | Yes | Featured image URL for OG tags |
| date | Yes | Publication date in YYYY-MM-DD format |

### Markdown Features

- Standard Markdown syntax
- Code blocks with syntax highlighting (specify language: ```rust)
- Heading anchor IDs automatically generated
- HTML tables
- Lists (ordered and unordered)
- Blockquotes
- Links with proper formatting

### Example Post

```markdown
---
title: Getting Started with Rust Web Development
summary: Learn how to build web applications with Rust and Axum framework
author: John Doe
tags: rust,web,axum,tutorial
image: https://example.com/rust-web.jpg
date: 2024-12-19
---

# Getting Started with Rust Web Development

Rust is becoming increasingly popular for web development...

## Why Choose Rust?

- Performance comparable to C/C++
- Memory safety without garbage collection
- Strong type system
- Growing ecosystem

## Your First Axum Application

```rust
use axum::{routing::get, Router};

async fn hello_world() -> String {
    "Hello, World!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Continue your content...
```

## Admin Panel

### Authentication

The admin panel is protected by password authentication:

1. Navigate to /admin
2. Enter the password configured in .env (ADMIN_PASSWORD)
3. Session tokens are valid for 1 hour
4. Rate limiting: 5 failed attempts trigger 5-minute lockout

### Dashboard

The dashboard displays:
- Quick stats (total posts, total tags, recent updates)
- Complete list of posts with metadata
- Edit and delete options for each post
- Buttons to create new posts or access GitHub integration

### Creating Posts

1. Click "New Post" in dashboard
2. Fill in the form fields:
   - Title
   - Content (Markdown)
   - Tags (comma-separated)
   - Summary
3. Click "Save Post"
4. Post appears immediately on the blog

### Editing Posts

1. Find the post in the dashboard table
2. Click the edit icon
3. Modify the content
4. Click "Save Post"
5. Changes take effect immediately

### Deleting Posts

1. Find the post in the dashboard table
2. Click the delete icon
3. Confirm the deletion
4. Post is permanently removed

## GitHub Integration

### Linking Repositories

The GitHub integration allows you to sync README files from your repositories as blog posts.

1. In admin dashboard, click "GitHub Repos"
2. View your public repositories
3. Click "Import" on a repository
4. A post is created with the repository name and README content
5. Repository metadata is saved for syncing

### Syncing Repositories

After linking repositories, you can manually sync to get latest changes:

1. Go to "GitHub Repos" section
2. Click "Sync" next to the linked repository
3. README content is updated on your blog
4. Post slug remains: `github-{repo-name}`

### Automatic Sync

Currently, syncing is manual. To set up automatic sync:

1. Configure a GitHub webhook on your repository
2. Point it to your blog's sync endpoint
3. Implement webhook verification (see API Reference)

## API Reference

### Public Endpoints

**GET /**
- Returns the blog homepage with all posts
- Query parameters:
  - `tag`: Filter posts by tag (e.g., `/?tag=rust`)

**GET /blog/{slug}**
- Returns a single blog post
- Parameters:
  - `slug`: Post slug (e.g., `getting-started`)

**GET /tags/{tag-name}**
- Returns posts filtered by tag
- Parameters:
  - `tag-name`: Tag name (URL-encoded)

**GET /blog**
- Redirects to homepage (308 Permanent Redirect)

**GET /api/search**
- Search across all posts
- Query parameters:
  - `q`: Search query (required, minimum 1 character)
- Response:
  ```json
  {
    "results": [
      {
        "title": "Post Title",
        "slug": "post-slug",
        "summary": "Post summary",
        "date": "Dec 19, 2024",
        "date_iso": "2024-12-19T00:00:00",
        "tags": ["rust", "web"],
        "reading_time": 5
      }
    ]
  }
  ```

**GET /og.png**
- Homepage OG image (1200x630 PNG)
- Dark theme with Geist fonts
- Cache: 1 year (immutable)

**GET /blog/{slug}/og.png**
- Individual post OG image (1200x630 PNG)
- Shows post title, author, date, reading time, tags
- Cache: 1 year (immutable)

**GET /blog/recents.png**
- Recent posts preview OG image (1200x630 PNG)
- Shows latest 4 posts with metadata
- Cache: 1 hour

**GET /rss.xml** or **GET /feed.xml**
- RSS feed with all posts (RSS 2.0)
- Includes post content, author, OG image enclosures
- Cache: 1 hour

**GET /atom.xml**
- Atom feed with all posts
- Includes post content, author, OG image links
- Cache: 1 hour

**GET /sitemap.xml**
- Master XML sitemap for search engines
- Includes homepage, posts, tag pages with OG images
- Cache: 1 hour

**GET /sitemap-posts.xml**
- Dedicated posts sitemap
- Compliant with sitemaps.org protocol
- Cache: 1 hour

**GET /robots.txt**
- Search engine crawler instructions
- Allows all crawlers including AI bots (GPT, Claude, etc.)
- References sitemaps

**GET /manifest.json**
- PWA manifest for app-like experience
- Includes app icons, theme colors, shortcuts
- Cache: 1 day

**GET /humans.txt**
- Team and technology attribution
- Cache: 1 day

**GET /.well-known/security.txt**
- Security policy and contact information
- Cache: 1 day

**GET /logo.png**, **/favicon.png**, **/favicon.ico**, **/apple-touch-icon.png**
- Logo/favicon files
- Serves logo.png for all favicon requests

### Admin Endpoints

All admin endpoints require valid session authentication.

**GET /admin**
- Admin login page

**POST /admin/login**
- Authenticate with password
- Sets secure session cookie
- Body: `password=your_password`

**GET /admin/dashboard**
- Admin dashboard with post management
- Requires authentication

**GET /admin/new**
- Create new post form
- Requires authentication

**GET /admin/edit/{slug}**
- Edit post form
- Requires authentication

**POST /admin/save**
- Create or update post
- Requires authentication
- Body: Form data with title, content, tags, summary, slug (optional)

**DELETE /admin/delete/{slug}**
- Delete a post
- Requires authentication

**GET /admin/github**
- GitHub integration page
- Requires authentication

**GET /admin/api/repos**
- List user's GitHub repositories
- Requires authentication
- Response: JSON array of repository objects

**POST /admin/github/import**
- Import GitHub repository as post
- Requires authentication
- Body: `repo_name=repository-name`

**POST /admin/sync/{slug}**
- Sync linked repository content
- Requires authentication

## Security

### Authentication

- Session-based authentication with secure tokens
- Passwords compared using constant-time comparison
- Session tokens expire after 1 hour of use
- Secure HttpOnly cookies prevent JavaScript access

### Rate Limiting

- Login attempts: 5 per IP address
- Lockout duration: 5 minutes after exceeding limit
- Counter resets after lockout period
- Prevents brute force attacks

### HTTPS Recommendations

For production deployment:

1. Use HTTPS only (enable HSTS via proxy)
2. Set secure headers (handled by application)
3. Use strong admin password (minimum 12 characters recommended)
4. Rotate GitHub tokens regularly
5. Monitor admin access logs
6. Keep dependencies updated: `cargo update`

### Data Protection

- No user data collection beyond sessions
- No analytics or tracking
- GitHub tokens stored in environment variables only
- Passwords never logged

## Development

### Development Server

```bash
cargo run
```

Server runs in debug mode with:
- Faster compilation
- Debug symbols
- No optimizations

### Building

```bash
# Development build (fast compilation, slow execution)
cargo build

# Release build (slow compilation, fast execution)
cargo build --release
```

### Code Quality

Check for issues:

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy

# Tests (if implemented)
cargo test
```

### Project Dependencies

Key dependencies and their purposes:

- `axum`: Web framework
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `handlebars`: Template rendering
- `pulldown-cmark`: Markdown to HTML conversion
- `syntect`: Syntax highlighting
- `uuid`: Session token generation
- `reqwest`: HTTP client for GitHub API (with rustls-tls)
- `chrono`: Date/time handling
- `resvg`: SVG rendering for OG images
- `usvg`: SVG parsing for OG images
- `tiny-skia`: PNG encoding for OG images

Update dependencies:

```bash
# Check for updates
cargo outdated

# Update all
cargo update

# Update specific package
cargo update -p package-name
```

### OG Image Generation

The blog automatically generates dynamic OG (Open Graph) images for:

1. **Homepage** (`/og.png`):
   - Site title, tagline, author
   - Dark theme with grid pattern
   - Cached for 1 year

2. **Individual Posts** (`/blog/{slug}/og.png`):
   - Post title (auto-wrapped for long titles)
   - Author, date, reading time, tags
   - Site branding
   - Cached for 1 year

3. **Recent Posts** (`/blog/recents.png`):
   - Latest 4 posts with titles and metadata
   - Refreshed every hour
   - Great for social sharing

**Technical Details**:
- SVG generation with embedded Geist fonts
- Converted to PNG using resvg/tiny-skia
- 1200x630px (optimal for social media)
- No external dependencies required
- Fonts embedded in binary (fonts/ directory)

## Deployment

### Production Build

```bash
cargo build --release
```

Binary location: `./target/release/axum-blog`

### Environment Setup

1. Copy `.env.example` to `.env`
2. Update production values:
   ```
   SITE_URL=https://yourblog.com
   ADMIN_PASSWORD=strong_unique_password
   GITHUB_USERNAME=your_username
   ```
3. Keep `.env` secure and out of version control

### Running the Server

```bash
# Direct execution
./target/release/axum-blog

# With environment loading
export $(cat .env | xargs) && ./target/release/axum-blog
```

### Systemd Service

Create `/etc/systemd/system/axum-blog.service`:

```ini
[Unit]
Description=Axum Blog Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/axum-blog
EnvironmentFile=/opt/axum-blog/.env
ExecStart=/opt/axum-blog/axum-blog
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable axum-blog
sudo systemctl start axum-blog
sudo systemctl status axum-blog
```

### Reverse Proxy (Nginx)

```nginx
server {
    listen 443 ssl http2;
    server_name yourblog.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    ssl_protocols TLSv1.3 TLSv1.2;
    ssl_ciphers HIGH:!aNULL:!MD5;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

server {
    listen 80;
    server_name yourblog.com;
    return 301 https://$server_name$request_uri;
}
```

### Docker Deployment

Create `Dockerfile`:

```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/aryansrao-blog .
COPY --from=builder /app/templates templates/
COPY --from=builder /app/content content/
COPY --from=builder /app/fonts fonts/
COPY --from=builder /app/logo.png logo.png
COPY .env.example .env
EXPOSE 8080
CMD ["./aryansrao-blog"]
```

Build and run:

```bash
docker build -t axum-blog .
docker run -p 8080:8080 --env-file .env axum-blog
```

### Leapcell Deployment

This blog is optimized for Leapcell deployment:

1. Ensure `leapcell.yaml` is configured
2. Uses rustls-tls instead of OpenSSL (no system dependencies)
3. All fonts are embedded in binary
4. Push to your repository
5. Deploy via Leapcell dashboard

**Important**: The blog uses pure Rust dependencies (no OpenSSL) for maximum compatibility with Leapcell's runtime.

## Troubleshooting

### Server won't start

**Error: "Address already in use"**
- Another service is running on port 8080
- Solution: Kill the existing process or change port (modify main.rs)

```bash
lsof -i :8080
kill -9 <PID>
```

### Admin panel returns 404

**Issue: /admin route not found**
- Verify server is running: `curl http://localhost:8080/`
- Check browser cache or use incognito mode
- Restart server

### Posts not appearing

**Issue: Blog shows no posts**
- Check `content/` directory exists and has .md files
- Verify markdown files have correct front matter
- Check server logs for parsing errors
- Restart server to reload content

### GitHub import fails

**Error: "Repository not found" or API errors**
- Verify `GITHUB_USERNAME` is correct in .env
- Check GitHub repository is public
- Add `GITHUB_TOKEN` to increase rate limits
- Verify internet connection

### OG images not generating

**Issue: OG images return errors or are blank**
- Ensure `fonts/` directory exists with Geist font files
- Check server logs for SVG parsing errors
- Verify resvg/usvg dependencies are installed
- Fonts should be embedded in binary (included via `include_bytes!`)

### Search not working

**Issue: Search returns no results**
- Ensure posts have content and metadata
- Check that search query is at least 1 character
- Verify API endpoint is accessible: `/api/search?q=test`
- Check browser console for JavaScript errors

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

**Aryan S Rao**
- GitHub: [@aryansrao](https://github.com/aryansrao)
- Twitter: [@aryan_s_rao](https://twitter.com/aryan_s_rao)
- Blog: [aryansrao-blogs.leapcell.app](https://aryansrao-blogs.leapcell.app)
- Verify internet connection

### Search not working

**Issue: Search returns no results**
- On homepage: searches all posts and content
- On post page: searches only within that post
- Ensure posts have content in body (not just title)
- Check that content is valid markdown

### Session expires too quickly

**Issue: Getting logged out from admin**
- Session timeout is 1 hour by default
- Check system clock is correct
- Clear browser cookies and login again
- Close unnecessary admin tabs (each has own session)

### Performance issues

**Slow page load**
- Use release build, not debug: `cargo build --release`
- Check for large images in posts
- Verify server isn't CPU-constrained
- Enable gzip compression in nginx

### Markdown rendering issues

**Issue: Code blocks or formatting looks wrong**
- Ensure code blocks use triple backticks: ` ``` `
- Specify language after backticks: ` ```rust `
- Check for syntax errors in markdown
- Use proper HTML entity encoding if needed

### Environmental variable errors

**Error: "Environment variable not found"**
- Ensure .env file exists in project root
- Verify variable names match exactly
- Check file permissions: `chmod 644 .env`
- Restart server after changing .env

## Contributing

Contributions are welcome. Please ensure code follows project patterns and includes appropriate testing.

## Support

For issues and questions, please check the troubleshooting section above or review the source code documentation.
