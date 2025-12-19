---
title: "Where is - Indian DIGIPIN location web app."
date: "2025-12-11"
tags: ["github", "project", "javascript", "bento-grid", "digipin", "digipin-decoder"]
summary: "A modern web application for sharing and finding locations using India Post's DIGIPIN geocode system. Where is provides an intuitive interface to generate, decode, and share 10-character digital addresses that represent precise geographic locations across India."
github_repo: "aryansrao/where-is"
---

# Where is

A modern web application for sharing and finding locations using India Post's DIGIPIN geocode system. Where is provides an intuitive interface to generate, decode, and share 10-character digital addresses that represent precise geographic locations across India.

**Live Demo:** [https://where-is-digipin.netlify.app/](https://where-is-digipin.netlify.app/)

**Repository:** [https://github.com/aryansrao/where-is](https://github.com/aryansrao/where-is)

---

## Table of Contents

- [Overview](#overview)
- [What is DIGIPIN?](#what-is-digipin)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
  - [DIGIPIN Algorithm](#digipin-algorithm)
  - [Encoding Process](#encoding-process)
  - [Decoding Process](#decoding-process)
- [User Interface](#user-interface)
- [Usage](#usage)
  - [Getting Your DIGIPIN](#getting-your-digipin)
  - [Decoding a DIGIPIN](#decoding-a-digipin)
  - [Converting Coordinates](#converting-coordinates)
  - [Sharing Locations](#sharing-locations)
- [Installation](#installation)
- [Configuration](#configuration)
- [API Reference](#api-reference)
- [Browser Compatibility](#browser-compatibility)
- [Contributing](#contributing)
- [Acknowledgments](#acknowledgments)

---

## Overview

Where is is a location-sharing application built around India Post's DIGIPIN standard. The application transforms GPS coordinates into human-readable 10-character codes that can be easily shared, stored, and decoded. This makes location sharing more convenient than traditional latitude/longitude coordinates while maintaining precision.

The application operates entirely in the browser with no backend required, ensuring user privacy and fast performance. All DIGIPIN generation and decoding happens client-side using JavaScript.

---

## What is DIGIPIN?

DIGIPIN is India Post's official geocoding system that converts geographic coordinates into a 10-character alphanumeric code. Each DIGIPIN represents a specific location within India with high precision.

### DIGIPIN Format

- **Length:** 10 characters
- **Format:** XXX-XXX-XXXX (with hyphens for readability)
- **Character Set:** Uses a specific grid of 16 characters (F, C, 9, 8, J, 3, 2, 7, K, 4, 5, 6, L, M, P, T)
- **Coverage:** All of India (Latitude: 2.5°N to 38.5°N, Longitude: 63.5°E to 99.5°E)
- **Precision:** Approximately 10 meters at the finest level

### Example

**Location:** Raipur, Chhattisgarh  
**Coordinates:** 21.1861°N, 81.3071°E  
**DIGIPIN:** `3J3-35J-3553`

---

## Features

### Core Functionality

- **Generate DIGIPIN:** Convert your current GPS location into a DIGIPIN code
- **Decode DIGIPIN:** Convert any DIGIPIN back into GPS coordinates
- **Coordinate Conversion:** Generate DIGIPIN from manually entered latitude/longitude
- **Interactive Map:** Visual representation of locations using Leaflet maps with CartoDB Voyager tiles
- **QR Code Generation:** Create scannable QR codes for any DIGIPIN
- **URL Sharing:** Share locations via URL parameters

### User Features

- **Copy to Clipboard:** One-click copying of DIGIPIN codes
- **Native Sharing:** Use device share functionality to send locations
- **Download QR Codes:** Save QR codes as PNG images
- **Open in Maps:** Launch location in Google Maps
- **Responsive Design:** Works seamlessly on desktop, tablet, and mobile devices
- **No Registration Required:** No accounts, no login, complete privacy

### Technical Features

- **Client-Side Processing:** All operations happen in the browser
- **No Backend Required:** Pure static web application
- **Offline Capable:** Core functionality works without internet (after initial load)
- **Fast Performance:** Instant encoding and decoding
- **URL Parameter Support:** Deep linking with DIGIPIN parameters

---

## Technology Stack

### Frontend

- **HTML5:** Semantic markup with comprehensive SEO meta tags
- **CSS3:** Modern styling with CSS Grid, Flexbox, and custom properties
- **Vanilla JavaScript:** No frameworks, pure ES6+ JavaScript

### Libraries

- **Leaflet.js v1.9.4:** Interactive map rendering
  - CartoDB Voyager tile layer for clean, modern maps
  - Custom marker placement
  - Zoom controls and attribution

- **QR Code Styling v1.5.0:** Modern QR code generation
  - Gradient styling support
  - SVG and PNG export
  - Customizable corner styles
  - Rounded dot patterns

### Fonts

- **Inter:** Primary font family (weights: 400, 500, 600, 700)
- Loaded from Google Fonts with preconnect optimization

### Hosting

- **Netlify:** Static site hosting with CDN distribution
- Automatic HTTPS
- Continuous deployment from GitHub

---

## Project Structure

```
where-is/
│
├── index.html          # Main HTML structure
│   ├── SEO meta tags
│   ├── Open Graph tags
│   ├── Favicon references
│   ├── Bento grid layout
│   └── Action buttons
│
├── styles.css          # Complete styling
│   ├── CSS custom properties (color scheme)
│   ├── Responsive grid system
│   ├── Component styles
│   ├── Animation definitions
│   └── Mobile breakpoints
│
├── scripts.js          # Application logic
│   ├── DIGIPIN algorithm
│   ├── Geolocation handling
│   ├── QR code generation
│   ├── Map integration
│   ├── URL parameter parsing
│   └── Event handlers
│
├── logo-where-is.jpeg  # Application logo/icon
│
└── README.md          # This file
```

---

## How It Works

### DIGIPIN Algorithm

The DIGIPIN algorithm is based on hierarchical spatial subdivision using a 4×4 grid system. India's geographic area is recursively divided into smaller regions, with each level represented by a single character from the DIGIPIN grid.

#### DIGIPIN Grid

```
+-----+-----+-----+-----+
|  F  |  C  |  9  |  8  |  Row 0 (North)
+-----+-----+-----+-----+
|  J  |  3  |  2  |  7  |  Row 1
+-----+-----+-----+-----+
|  K  |  4  |  5  |  6  |  Row 2
+-----+-----+-----+-----+
|  L  |  M  |  P  |  T  |  Row 3 (South)
+-----+-----+-----+-----+
Col 0   Col 1  Col 2  Col 3
(West)              (East)
```

#### Bounding Box

- **Minimum Latitude:** 2.5°N (Southern India)
- **Maximum Latitude:** 38.5°N (Northern India)
- **Minimum Longitude:** 63.5°E (Western India)
- **Maximum Longitude:** 99.5°E (Eastern India)

### Encoding Process

The encoding process converts GPS coordinates (latitude, longitude) into a 10-character DIGIPIN:

1. **Initialization:** Start with India's full bounding box
2. **Iteration:** For each of 10 levels:
   - Divide current bounding box into 4×4 grid (16 cells)
   - Calculate which cell contains the target coordinates
   - Append corresponding character from DIGIPIN grid
   - Update bounding box to selected cell
   - Add separator after 3rd and 6th characters
3. **Result:** 10-character code in XXX-XXX-XXXX format

#### Example Calculation

**Input:** 21.1861°N, 81.3071°E

**Level 1:**
- Current bounds: Lat [2.5, 38.5], Lon [63.5, 99.5]
- Cell size: Lat 9°, Lon 9°
- Target falls in Row 1, Col 2
- Character: `3` (from grid)
- New bounds: Lat [20.5, 29.5], Lon [81.5, 90.5]

**Level 2:**
- Cell size: Lat 2.25°, Lon 2.25°
- Target falls in Row 0, Col 1
- Character: `J`
- Continue for 10 levels...

**Final DIGIPIN:** `3J3-35J-3553`

### Decoding Process

The decoding process converts a DIGIPIN back into GPS coordinates:

1. **Parse:** Remove hyphens and validate 10-character input
2. **Initialization:** Start with India's full bounding box
3. **Iteration:** For each character:
   - Find character position in DIGIPIN grid (row, column)
   - Calculate corresponding cell in current 4×4 subdivision
   - Update bounding box to that cell
   - Continue to next level
4. **Result:** Return center point of final bounding box

The center coordinates represent the location with approximately 10-meter precision.

---

## User Interface

The application features a modern bento grid layout with distinct functional areas:

### Main Components

#### 1. Get Location Button
- Large, prominent button at the top
- Requests browser geolocation permission
- Shows loading state during GPS acquisition
- Automatically generates DIGIPIN from current location

#### 2. DIGIPIN Display (Large Card)
- Shows generated or decoded DIGIPIN
- XXX-XXX-XXXX format with hyphens
- Placeholder text when no DIGIPIN is loaded
- Spans 2 columns in desktop layout

#### 3. QR Code Generator
- Automatic QR code generation for each DIGIPIN
- Beautiful gradient styling (blue theme)
- Embedded URL for easy sharing
- Click to download as PNG
- Rounded dot pattern with custom corners

#### 4. Coordinates Display
- Shows latitude and longitude
- 6 decimal places precision
- Compact single-column card

#### 5. Interactive Map
- Full Leaflet map integration
- CartoDB Voyager tiles (clean design)
- Marker at exact location
- Zoom level 16 for detail (level 4 for default India view)
- 2×2 grid space for prominent display

#### 6. Quick Actions Panel
- **Copy DIGIPIN:** Clipboard integration
- **Share DIGIPIN:** Native share API or URL copy
- **Open in Maps:** Launch Google Maps
- All buttons disabled until DIGIPIN is generated
- 2×2 grid space with icon-labeled buttons

#### 7. Decode/Encode Section
- Two input modes:
  - **DIGIPIN Input:** Decode existing DIGIPIN
  - **Coordinates Input:** Generate DIGIPIN from lat/lng
- "Locate" button processes either input
- Auto-formatting for DIGIPIN input (adds hyphens)
- Flexible coordinate parsing (comma, space separated)
- 2 columns wide

#### 8. Logo Card
- Application branding
- Visual identity element
- Single column

#### 9. Links Grid
- **Share App:** Share the website itself
- **GitHub:** Repository link
- **Contribute:** Developer portfolio
- **Developer:** Creator information
- Icon-based design for clarity

### Design Elements

#### Color Scheme
- **Primary:** `#6D94C5` (Blue)
- **Primary Light:** `#CBDCEB` (Light Blue)
- **Cream:** `#E8DFCA` (Warm neutral)
- **Cream Light:** `#F5EFE6` (Light warm)
- **Text:** `#2d3748` (Dark gray)
- **Text Light:** `#718096` (Medium gray)

#### Layout
- Bento grid: 4-column responsive layout
- Cards with varying sizes (1×1, 2×1, 2×2)
- Border radius: 20px (large), 12px (small)
- Consistent spacing: 16px gap
- Box shadows for depth

#### Responsive Design
- Desktop: 4-column grid, max-width 800px
- Tablet: 2-column grid
- Mobile: Single column stack
- Breakpoint: 768px

#### Interactions
- Hover effects on buttons
- Loading states
- Toast notifications (3-second duration)
- Smooth transitions (0.3s ease)
- Disabled button states

---

## Usage

### Getting Your DIGIPIN

1. Open the application in your browser
2. Click the **"Get My DIGIPIN"** button at the top
3. Allow location permission when prompted
4. Wait for GPS acquisition (typically 2-5 seconds)
5. Your DIGIPIN will appear in the display card
6. The map will show your exact location
7. A QR code will be automatically generated

### Decoding a DIGIPIN

1. Locate the **"Decode / Encode"** section
2. Enter a DIGIPIN in the format XXX-XXX-XXXX
   - Hyphens are optional (auto-formatted)
   - Case-insensitive (automatically converted to uppercase)
3. Click **"Locate"**
4. The coordinates and map will update to show the location
5. QR code will be regenerated for the decoded location

**Example DIGIPIN to try:** `3J3-35J-3553` (Raipur, Chhattisgarh)

### Converting Coordinates

1. Find the **"Decode / Encode"** section
2. Enter coordinates in the coordinate input field
   - Format: `latitude, longitude`
   - Example: `21.1861, 81.3071`
   - Supports comma, space, or comma-space separation
3. Click **"Locate"**
4. DIGIPIN will be generated from your coordinates
5. Map and QR code will update

### Sharing Locations

#### Method 1: Share DIGIPIN Text
1. Generate or decode a DIGIPIN
2. Click **"Copy DIGIPIN"** button
3. Paste the code anywhere (SMS, email, chat)
4. Recipient can enter it in the decode section

#### Method 2: Share URL
1. Generate or decode a DIGIPIN
2. Click **"Share DIGIPIN"** button
3. Use native share menu or copy the generated URL
4. URL format: `https://where-is-digipin.netlify.app/?digipin=XXXXXXXXXX`
5. Recipient clicks link to see location automatically

#### Method 3: Share QR Code
1. Generate or decode a DIGIPIN
2. Click the QR code to download it as PNG
3. Share the image via any platform
4. Recipient scans QR code to open location in browser

#### Method 4: Open in Maps
1. Generate or decode a DIGIPIN
2. Click **"Open in Maps"** button
3. Google Maps opens with exact coordinates
4. Share Google Maps link from there

---

## Installation

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/aryansrao/where-is.git
   cd where-is
   ```

2. **Serve locally:**
   
   Using Python:
   ```bash
   python3 -m http.server 8000
   ```
   
   Using Node.js (http-server):
   ```bash
   npx http-server -p 8000
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

1. Fork or clone the repository to your GitHub account
2. Log in to [Netlify](https://www.netlify.com/)
3. Click **"New site from Git"**
4. Connect your GitHub repository
5. Configure build settings:
   - Build command: (leave empty)
   - Publish directory: (leave empty or use `/`)
6. Click **"Deploy site"**
7. Your site will be live at `your-site-name.netlify.app`

#### GitHub Pages

1. Push code to GitHub repository
2. Go to repository **Settings** → **Pages**
3. Source: Select `main` branch
4. Folder: Select `/ (root)`
5. Click **Save**
6. Site will be available at `https://username.github.io/where-is`

#### Vercel

1. Install Vercel CLI: `npm i -g vercel`
2. Navigate to project directory
3. Run: `vercel`
4. Follow prompts to deploy

### Requirements

- No build process required
- No dependencies to install
- No server-side code
- No database needed

**Minimum Requirements:**
- Modern web browser (Chrome 90+, Firefox 88+, Safari 14+, Edge 90+)
- HTTPS connection (required for geolocation API)
- Internet connection for:
  - Initial page load
  - Map tiles
  - QR code library (if CDN is used)

---

## Configuration

### Customizing Map Tiles

The application uses CartoDB Voyager tiles by default. To change map style, edit `scripts.js`:

```javascript
// Current (CartoDB Voyager):
L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
    maxZoom: 19
}).addTo(map);

// Alternative: OpenStreetMap
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    maxZoom: 19,
    attribution: '© OpenStreetMap contributors'
}).addTo(map);

// Alternative: CartoDB Dark Matter
L.tileLayer('https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png', {
    maxZoom: 19
}).addTo(map);
```

### Customizing QR Code Style

QR code appearance can be modified in the `generateQR()` function:

```javascript
const qrCode = new QRCodeStyling({
    width: 140,              // Size in pixels
    height: 140,
    type: "svg",             // "svg" or "canvas"
    data: qrData,
    dotsOptions: {
        type: "rounded",     // "rounded", "dots", "classy", "square"
        gradient: {
            type: "linear",  // "linear" or "radial"
            rotation: 45,    // Gradient angle
            colorStops: [
                { offset: 0, color: "#0ea5e9" },
                { offset: 1, color: "#1e3a8a" }
            ]
        }
    },
    // Modify cornersSquareOptions and cornersDotOptions similarly
});
```

### Changing Color Scheme

Edit CSS custom properties in `styles.css`:

```css
:root {
    --primary: #6D94C5;           /* Main brand color */
    --primary-light: #CBDCEB;     /* Light variant */
    --cream: #E8DFCA;             /* Secondary color */
    --cream-light: #F5EFE6;       /* Background color */
    --text: #2d3748;              /* Primary text */
    --text-light: #718096;        /* Secondary text */
}
```

### URL Parameter Format

The application supports URL parameters for sharing:

```
# Standard parameter
https://where-is-digipin.netlify.app/?digipin=3J335J3553

# Short parameter
https://where-is-digipin.netlify.app/?d=3J335J3553

# With hyphens (automatically cleaned)
https://where-is-digipin.netlify.app/?digipin=3J3-35J-3553
```

---

## API Reference

### Core Functions

#### `getDigiPin(lat, lon)`

Converts GPS coordinates to DIGIPIN format.

**Parameters:**
- `lat` (number): Latitude in decimal degrees (2.5 to 38.5)
- `lon` (number): Longitude in decimal degrees (63.5 to 99.5)

**Returns:** (string) 10-character DIGIPIN with hyphens

**Throws:** Error if coordinates are outside India's bounds

**Example:**
```javascript
const digipin = getDigiPin(21.1861, 81.3071);
// Returns: "3J3-35J-3553"
```

#### `getLatLngFromDigiPin(digiPin)`

Decodes DIGIPIN to GPS coordinates.

**Parameters:**
- `digiPin` (string): 10-character DIGIPIN (hyphens optional)

**Returns:** Object with `latitude` and `longitude` as strings (6 decimal places)

**Throws:** Error if DIGIPIN is invalid

**Example:**
```javascript
const coords = getLatLngFromDigiPin("3J3-35J-3553");
// Returns: { latitude: "21.186100", longitude: "81.307100" }
```

#### `formatDigipin(pin)`

Formats DIGIPIN with hyphens.

**Parameters:**
- `pin` (string): DIGIPIN with or without hyphens

**Returns:** (string) Formatted DIGIPIN in XXX-XXX-XXXX format

**Example:**
```javascript
const formatted = formatDigipin("3J335J3553");
// Returns: "3J3-35J-3553"
```

### UI Functions

#### `updateUI(digipin, coords)`

Updates all UI elements with new location data.

**Parameters:**
- `digipin` (string): Formatted DIGIPIN
- `coords` (object): Object with `latitude` and `longitude`

**Effects:**
- Updates DIGIPIN display
- Generates QR code
- Updates map
- Updates coordinate display
- Enables action buttons

#### `generateQR(digipin, coords)`

Creates QR code with embedded URL.

**Parameters:**
- `digipin` (string): DIGIPIN to encode
- `coords` (object): Coordinates (currently unused in QR)

**Effects:**
- Clears existing QR code
- Generates new styled QR code
- Adds download handler

#### `updateMap(lat, lon)`

Renders map with location marker.

**Parameters:**
- `lat` (string): Latitude
- `lon` (string): Longitude

**Effects:**
- Clears existing map
- Creates new Leaflet map
- Adds CartoDB tiles
- Places marker at coordinates

#### `showToast(message)`

Displays temporary notification.

**Parameters:**
- `message` (string): Text to display

**Effects:**
- Shows toast for 3 seconds
- Auto-hides after timeout

### Constants

#### `DIGIPIN_GRID`

4×4 array defining character positions:
```javascript
[
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T']
]
```

#### `BOUNDS`

Geographic boundaries for DIGIPIN coverage:
```javascript
{
    minLat: 2.5,
    maxLat: 38.5,
    minLon: 63.5,
    maxLon: 99.5
}
```

---

## Browser Compatibility

### Supported Browsers

| Browser | Minimum Version | Notes |
|---------|----------------|-------|
| Chrome | 90+ | Full support |
| Firefox | 88+ | Full support |
| Safari | 14+ | Full support, iOS 14+ required |
| Edge | 90+ | Full support |
| Opera | 76+ | Full support |
| Samsung Internet | 14+ | Full support |

### Required APIs

- **Geolocation API:** For location detection
- **Clipboard API:** For copy functionality
- **Web Share API:** For native sharing (optional, fallback provided)
- **Canvas/SVG:** For QR code generation
- **Fetch API:** For map tiles (via Leaflet)

### HTTPS Requirement

The Geolocation API requires a secure context (HTTPS). The application will not work on HTTP except for `localhost` during development.

### Known Limitations

- **Safari:** Web Share API may not support all share options
- **Firefox:** Some older versions may not support modern CSS Grid fully
- **Internet Explorer:** Not supported (lacks ES6 support)

---

## Contributing

Contributions are welcome! Here's how you can help:

### Reporting Issues

1. Check if the issue already exists
2. Create a new issue with:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Browser and OS information
   - Screenshots if applicable

### Suggesting Features

1. Open an issue with `[Feature Request]` tag
2. Describe the feature and use case
3. Explain why it would be valuable

### Code Contributions

1. **Fork the repository**
   ```bash
   git clone https://github.com/your-username/where-is.git
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Follow existing code style
   - Add comments for complex logic
   - Test on multiple browsers
   - Ensure responsive design works

4. **Commit your changes**
   ```bash
   git commit -m "Add: description of your changes"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Describe what changed and why
   - Reference any related issues
   - Include screenshots for UI changes

### Code Style Guidelines

- Use clear, descriptive variable names
- Add JSDoc comments for functions
- Follow existing indentation (4 spaces)
- Keep functions focused and modular
- Use ES6+ features (const/let, arrow functions, template literals)

### Areas for Contribution

- **Accessibility:** ARIA labels, keyboard navigation
- **Internationalization:** Multi-language support
- **Performance:** Optimization for slower devices
- **Features:** Offline mode, save favorites, location history
- **Design:** Additional themes, animations
- **Documentation:** Tutorials, video guides

---

## Acknowledgments

### DIGIPIN Standard

This application implements the official DIGIPIN algorithm developed by India Post. The DIGIPIN standard provides a systematic way to encode geographic locations within India.

**Official Repository:** [https://github.com/INDIAPOST-gov/digipin](https://github.com/INDIAPOST-gov/digipin)

### Libraries and Tools

- **Leaflet.js** - Interactive map library ([leafletjs.com](https://leafletjs.com/))
- **QR Code Styling** - Modern QR code generation ([qr-code-styling](https://github.com/kozakdenys/qr-code-styling))
- **CartoDB** - Beautiful map tiles ([carto.com](https://carto.com/))
- **Google Fonts** - Inter font family ([fonts.google.com](https://fonts.google.com/))
- **Netlify** - Hosting and deployment ([netlify.com](https://www.netlify.com/))

### Design Inspiration

- Modern bento grid layouts
- Material Design principles
- iOS Human Interface Guidelines

### Developer

**Aryan S Rao**
- Portfolio: [https://aryansrao.github.io](https://aryansrao.github.io)
- GitHub: [@aryansrao](https://github.com/aryansrao)

---

## Contact

For questions, suggestions, or collaboration:

- **GitHub Issues:** [Create an issue](https://github.com/aryansrao/where-is/issues)
- **GitHub Discussions:** [Start a discussion](https://github.com/aryansrao/where-is/discussions)
- **Developer Website:** [aryansrao.github.io](https://aryansrao.github.io)

---

## Changelog

### Version 1.0.0 (Initial Release)

**Features:**
- DIGIPIN generation from GPS
- DIGIPIN decoding to coordinates
- Coordinate to DIGIPIN conversion
- Interactive Leaflet maps
- QR code generation and download
- Copy and share functionality
- URL parameter support
- Responsive bento grid design
- Mobile-optimized interface

**Libraries:**
- Leaflet.js 1.9.4
- QR Code Styling 1.5.0
- Inter font family

**Platform:**
- Deployed on Netlify
- HTTPS enabled
- CDN distribution

---

**Made with care for the community. Share your location, share this app.**