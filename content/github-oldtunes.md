---
title: "OldTunes - Location-aware radio streaming application."
date: "2025-12-11"
tags: ["github", "project", "css", "api", "glassmorphism", "minimalistic"]
summary: "OldTunes is a location-aware radio streaming application built with modern web technologies. The application transforms global radio discovery into an interactive, visual experience by combining real-time station data, interactive mapping, and seamless audio streaming."
github_repo: "aryansrao/OldTunes"
---

# OldTunes - World Radio Map

A modern, interactive web application that lets you discover and listen to radio stations from around the world. Browse stations on an interactive map, search by country or genre, and enjoy streaming audio with an intuitive player interface powered by glassmorphic design and real-time geolocation features.

**Live Demo:** [https://oldtunes.netlify.app/](https://oldtunes.netlify.app/)

**Repository:** [https://github.com/aryansrao/OldTunes](https://github.com/aryansrao/OldTunes)

**Developer Portfolio:** [https://aryansrao.github.io](https://aryansrao.github.io)

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
  - [Application Architecture](#application-architecture)
  - [Station Data Loading](#station-data-loading)
  - [Search Algorithm](#search-algorithm)
  - [Map Rendering](#map-rendering)
- [User Interface](#user-interface)
- [Installation](#installation)
- [Configuration](#configuration)
- [API Reference](#api-reference)
- [Browser Compatibility](#browser-compatibility)
- [Usage Guide](#usage-guide)
- [Contributing](#contributing)
- [Acknowledgments](#acknowledgments)

---

## Overview

OldTunes is a location-aware radio streaming application built with modern web technologies. The application transforms global radio discovery into an interactive, visual experience by combining real-time station data, interactive mapping, and seamless audio streaming. Users can explore stations from any location, filter by preferences, and enjoy uninterrupted audio playback with an elegant, responsive interface.

The application operates entirely in the browser with no backend required, ensuring user privacy and instant performance. Station data is fetched dynamically and cached for optimal responsiveness.

---

## Features

### Core Functionality

- **Interactive World Map**: Visualize radio stations globally with Leaflet.js mapping technology
- **Marker Clustering**: Automatic station clustering for performance and usability
- **Dynamic Station Loading**: Real-time station data fetching from external sources
- **Geolocation Integration**: Browser-based location detection (optional)
- **Advanced Search**: Filter stations by name, country, genre, or language
- **Audio Streaming**: HTML5 audio player with support for streaming protocols

### User Controls

- **Playback Controls**: Play, pause, next, previous, and random station selection
- **Volume Control**: Custom slider for audio level adjustment
- **Search Results**: Real-time dropdown with station suggestions
- **Station Info**: Display current station name, country, and genre
- **Zoom Controls**: Custom slider for map zoom levels (2x to 18x magnification)

### Design Features

- **Glassmorphic UI**: Modern liquid glass effects with blur and transparency
- **Dark Theme**: Eye-friendly dark interface with high contrast
- **Responsive Layout**: Optimized for desktop, tablet, and mobile devices
- **Loading States**: Visual feedback during data loading and processing
- **Smooth Animations**: Fluid transitions and interactive feedback

### Technical Features

- **Client-Side Processing**: All operations in browser for privacy and speed
- **No Backend Required**: Pure static web application
- **Offline Capable**: Core functionality without internet (after initial load)
- **Real-Time Updates**: Instant station filtering and map updates

## Technology Stack

### Frontend

- **HTML5**: Semantic markup with comprehensive structure
- **CSS3**: Modern styling with custom properties, Grid, and Flexbox
- **Vanilla JavaScript**: Pure ES6+ JavaScript, no frameworks required

### Libraries

- **Leaflet.js v1.9.4**: Interactive mapping library
  - CartoDB Dark tile layer for modern map aesthetics
  - Marker clustering via Leaflet.MarkerCluster
  - Custom zoom controls and attribution

- **Leaflet.MarkerCluster v1.4.1**: Station marker clustering
  - Dynamic cluster generation
  - Zoom-responsive grouping
  - Custom styling for clusters

### Fonts & Assets

- **Poppins Font Family**: Modern sans-serif from Google Fonts
  - Weights: 300 (Light), 400 (Regular), 500 (Medium), 600 (SemiBold), 700 (Bold)
  - Preconnect optimization for performance

### Hosting & Deployment

- **Netlify**: Static site hosting with automatic deployments
- **HTTPS**: Secure connection enabled
- **CDN**: Global content distribution network

## Project Structure

```
OldTunes/
│
├── index.html          # Main HTML structure
│   ├── SVG filters for glassmorphic effects
│   ├── Search interface
│   ├── Map container
│   ├── Audio player controls
│   └── Station info display
│
├── app.js              # Application logic
│   ├── RadioApp class (main controller)
│   ├── Map initialization
│   ├── Station loading and caching
│   ├── Search functionality
│   ├── Audio player management
│   ├── Event handlers
│   └── Utility functions
│
├── styles.css          # Complete styling
│   ├── CSS custom properties (colors, sizes)
│   ├── Glassmorphic effects
│   ├── Layout and grid system
│   ├── Component styles
│   ├── Animation definitions
│   ├── Responsive breakpoints
│   └── Dark theme colors
│
├── assets/             # Static assets
│   └── (images, icons, etc.)
│
├── .git/               # Version control
│
└── README.md          # Documentation
```

---

## How It Works

### Application Architecture

OldTunes follows a single-page application (SPA) pattern with a modular architecture:

#### Initialization Flow

1. **Page Load**: Browser loads HTML, CSS, and JavaScript
2. **App Initialization**: `RadioApp` class instantiates
3. **Map Setup**: Leaflet map renders with dark CartoDB tiles
4. **Data Loading**: Station data fetches from external API
5. **UI Binding**: Event listeners attached to interactive elements
6. **Ready State**: Application ready for user interaction

#### Class Structure

```javascript
class RadioApp {
    constructor()      // Initialize properties
    init()             // Run setup sequence
    initMap()          // Create and configure map
    createZoomSlider() // Add zoom control
    bindEvents()       // Attach event listeners
    loadStations()     // Fetch station data
    handleSearch()     // Process search input
    displayStations()  // Render station markers
    togglePlayPause()  // Control audio playback
    updatePlayButton() // Update UI button state
}
```

### Station Data Loading

**Data Format Expected:**

```json
[
  {
    "id": "unique_identifier",
    "name": "Station Name",
    "country": "Country Name",
    "genre": "Genre/Category",
    "url": "https://stream.url:port/path",
    "latitude": 28.7041,
    "longitude": 77.1025,
    "language": "Language"
  }
]
```

**Loading Process:**

1. Fetch from API endpoint (URL configurable in app.js)
2. Parse JSON response
3. Validate station data structure
4. Filter out invalid entries
5. Cache stations in memory
6. Process with clustering algorithm
7. Create map markers

**Caching Strategy:**

- In-memory storage of stations array
- Filtered results cached during search
- No persistent storage (session-based)
- Re-fetch on application reload

### Search Algorithm

**Search Methodology:**

1. **Input Processing**: Normalize search text (lowercase, trim)
2. **Field Matching**: Search across multiple fields:
   - Station name (primary)
   - Country (secondary)
   - Genre (tertiary)
   - Language (optional)
3. **Match Scoring**: Prioritize exact matches over partial
4. **Results Sorting**: Sort by relevance and alphabetically
5. **UI Update**: Render results in dropdown below search input

**Supported Search Patterns:**

```
"BBC"           // Station name match
"India"         // Country match
"Jazz"          // Genre match
"BBC Radio 1"   // Multi-word station name
"London, UK"    // Location-based search
```

### Map Rendering

**Leaflet Map Configuration:**

```javascript
// Bounds: Entire world with wrapping
center: [20, 0]        // Center at 20°N, 0°E (Equatorial Atlantic)
zoom: 2                // Global view (continental level)
minZoom: 2             // Prevent over-zoom out
maxZoom: 18            // Allow detailed zoom in
worldCopyJump: true    // Enable map wrapping
```

**Tile Layer:**

- Provider: CartoDB Dark theme
- Attribution: OpenStreetMap, CartoDB
- Maximum zoom: 20
- Dark aesthetic for nighttime browsing

**Marker Clustering:**

- Threshold: Default clustering at various zoom levels
- Dynamic updates as map pans/zooms
- Click cluster to zoom to contents
- Individual markers at high zoom levels

---

## User Interface

### Main Components

#### 1. Search Bar
- Prominent placement at top
- Glassmorphic styling with blur effects
- Real-time input validation
- Auto-focusing dropdown results
- Search icon and random button

#### 2. Map Container
- Full-width interactive Leaflet map
- CartoDB Dark tiles for visibility
- Station markers with clustering
- Zoom slider control
- Attribution footer

#### 3. Audio Player
- Bottom-fixed control bar
- Play/pause button with state indication
- Previous/next station navigation
- Random station selector
- Volume slider (0-100%)
- Current station display

#### 4. Search Results Dropdown
- Appears below search input
- Real-time station suggestions
- Scrollable results
- Click to select and play
- Click-outside to dismiss

#### 5. Station Information Display
- Current station name
- Country and genre tags
- Stream status indicator
- Loading states

### Color Scheme

```css
--text-primary: #ffffff           /* Main text color */
--text-secondary: rgba(255, 255, 255, 0.6)  /* Secondary text */
--accent: #ff6b35                 /* Orange accent */
--glass-tint: rgba(255, 255, 255, 0.2)      /* Glass effect */
--bg: #0a0a0a                    /* Background */
```

### Responsive Design

**Breakpoints:**

- **Desktop**: 1200px+ (full feature set)
- **Tablet**: 768px-1199px (optimized layout)
- **Mobile**: Below 768px (single column, touch-optimized)

**Responsive Adjustments:**

- Search bar width optimizations
- Player controls stack vertically on mobile
- Map height adjustment for smaller screens
- Font sizes scale for readability
- Touch-friendly button sizes (minimum 44x44px)

### Animations and Effects

**Glassmorphic Effects:**

- Backdrop blur (16px)
- Transparency layers
- SVG filter distortions
- Gradient overlays

**Transitions:**

- Button hover effects
- Search result fade-in
- Map marker animations
- Volume slider smooth changes
- Loading spinner rotations

---

## Installation

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/aryansrao/OldTunes.git
   cd OldTunes
   ```

2. **Serve locally:**

   Using Python 3:
   ```bash
   python3 -m http.server 8000
   ```

   Using Node.js (http-server):
   ```bash
   npm install -g http-server
   http-server -p 8000
   ```

   Using PHP:
   ```bash
   php -S localhost:8000
   ```

3. **Open in browser:**
   ```
   http://localhost:8000
   ```

### Deployment

#### Netlify (Recommended)

1. Push code to GitHub repository
2. Log in to [Netlify](https://www.netlify.com/)
3. Click "New site from Git"
4. Select your GitHub repository
5. Build settings:
   - Build command: (leave empty)
   - Publish directory: (leave empty)
6. Deploy site
7. Access at `your-site.netlify.app`

#### Vercel

1. Install Vercel CLI: `npm i -g vercel`
2. Run: `vercel`
3. Follow configuration prompts

#### GitHub Pages

1. Push to GitHub main branch
2. Go to repository Settings > Pages
3. Select main branch as source
4. Access at `https://username.github.io/OldTunes`

### Requirements

- No build process or dependencies
- No server-side code required
- Modern web browser (see Browser Compatibility)
- HTTPS for geolocation features (optional)

---

## Configuration

### Changing Station Data Source

Edit the API endpoint in `app.js`:

```javascript
async loadStations() {
    try {
        const response = await fetch('YOUR_API_ENDPOINT_HERE');
        // Default uses a public radio API
        // Replace with your own endpoint or data file
    } catch (error) {
        console.error('Error loading stations:', error);
    }
}
```

### Customizing Map Tiles

Change tile provider in `initMap()`:

```javascript
// Current: CartoDB Dark
L.tileLayer('https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    subdomains: 'abcd',
    maxZoom: 20
}).addTo(this.map);

// Alternative: OpenStreetMap Default
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; OpenStreetMap contributors',
    maxZoom: 19
}).addTo(this.map);

// Alternative: CartoDB Light
L.tileLayer('https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png', {
    maxZoom: 20
}).addTo(this.map);
```

### Modifying Color Scheme

Edit CSS custom properties in `styles.css`:

```css
:root {
    --text-primary: #ffffff;
    --text-secondary: rgba(255, 255, 255, 0.6);
    --accent: #ff6b35;
    --glass-tint: rgba(255, 255, 255, 0.2);
}
```

### Adjusting Player Controls

Modify player configuration in `bindEvents()`:

```javascript
// Default volume (0-1)
this.audioPlayer.volume = 0.7;

// Change volume slider range
<input type="range" id="volumeSlider" min="0" max="100" value="70">
```

### Zoom Slider Settings

Customize in `createZoomSlider()`:

```javascript
const slider = document.getElementById('zoomSlider');
slider.min = 2;
slider.max = 18;
slider.step = 1;
slider.value = 2;
```

---

## API Reference

### Core Methods

#### `init()`

Initializes the entire application.

```javascript
async init() {
    this.initMap();
    this.bindEvents();
    await this.loadStations();
    this.hideLoading();
}
```

#### `initMap()`

Creates and configures the Leaflet map with CartoDB Dark tiles and world wrapping enabled.

#### `loadStations()`

Fetches station data from remote API, validates, caches, and displays on map.

#### `handleSearch(query: string)`

Filters stations across name, country, genre, and language fields. Updates UI with matching results.

#### `displayStations()`

Renders station markers on map with clustering and click handlers for playback.

#### `togglePlayPause()`

Controls audio playback state and updates button UI.

#### `playStation(station: Object)`

Loads and plays a specific station with URL from station data.

#### `playPreviousStation()`

Plays previous station in filtered list with wrapping.

#### `playNextStation()`

Plays next station in filtered list with wrapping.

#### `playRandomStation()`

Selects and plays random station from current filtered list.

#### `updatePlayButton(isPlaying: boolean)`

Updates play button appearance based on playback state.

---

## Browser Compatibility

| Browser | Minimum Version | Notes |
|---------|-----------------|-------|
| Chrome | 90+ | Full support |
| Firefox | 88+ | Full support |
| Safari | 14+ | Full support, iOS 14+ |
| Edge | 90+ | Full support |
| Opera | 76+ | Full support |

**Required APIs:** Fetch, HTML5 Audio, Geolocation (optional), Canvas/SVG, LocalStorage (optional)

**HTTPS:** Required for geolocation features. HTTP works on localhost.

---

## Usage Guide

### Getting Started

1. Open [https://oldtunes.netlify.app/](https://oldtunes.netlify.app/)
2. Wait for map and data to load
3. Search for stations or click map markers

### Searching

- **By Name**: "BBC Radio 1"
- **By Country**: "India", "UK"
- **By Genre**: "Jazz", "Classical"

### Playing

1. **Search and Click**: Type station name, click result
2. **Map Markers**: Zoom and click marker on map
3. **Navigation**: Use next/previous/random buttons

### Controls

- **Play/Pause**: Toggle playback
- **Previous/Next**: Navigate stations
- **Random**: Discover new station
- **Volume**: Adjust (0-100%)

---

## Contributing

### Report Issues

Check existing issues, then create detailed issue with:
- Browser and OS info
- Steps to reproduce
- Expected vs actual behavior
- Screenshots/error messages

### Suggest Features

Open issue with [Feature Request] tag, describe use case and benefits.

### Code Contributions

1. Fork and create feature branch
2. Make changes following style guidelines
3. Commit and push
4. Create Pull Request

### Areas for Contribution

- Features: Favorites, history, playlists
- UI/UX: Themes, animations, accessibility
- Performance: Optimization, caching
- Data: Station sources, categorization
- Documentation: Guides, tutorials

---

## Acknowledgments

### Libraries

- **Leaflet.js** - Interactive mapping
- **Leaflet.MarkerCluster** - Station clustering
- **CartoDB** - Map tiles
- **Google Fonts** - Poppins font
- **Netlify** - Hosting and deployment

### Design Inspiration

- Glassmorphic design trends
- Spotify and Apple Music UI patterns
- Material Design principles
- Mobile-first responsive design

### Developer

**Aryan S Rao**
- Portfolio: [https://aryansrao.github.io](https://aryansrao.github.io)
- GitHub: [@aryansrao](https://github.com/aryansrao)

---

## Contact & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/aryansrao/OldTunes/issues)
- **GitHub Discussions**: [Community discussions](https://github.com/aryansrao/OldTunes/discussions)
- **Website**: [aryansrao.github.io](https://aryansrao.github.io)

---

## Changelog

### Version 1.0.0 (Initial Release)

**Features:**
- Interactive world map with Leaflet.js
- Search across 1000+ global stations
- Real-time filtering by name, country, genre
- HTML5 audio player with controls
- Marker clustering
- Dark theme with glasmorphic UI
- Responsive mobile design

**Deployment:** Netlify with HTTPS and CDN

---

**Discover the world, one station at a time.**