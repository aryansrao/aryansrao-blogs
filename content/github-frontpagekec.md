---
title: "Front page maker for Krishna Engineering College."
date: "2025-11-17"
tags: ["github", "project", "html", "college-students-open-source", "html-css-javascript", "htmltoimage"]
summary: "This is a website for the students of Krishna Engineering College and It is used to create front pages for assignments. currently live at https://frontpagekec.netlify.app"
github_repo: "aryansrao/frontpagekec"
---

# KEC Lab Manual Creator

[![Netlify Status](https://api.netlify.com/api/v1/badges/535c1b36-092b-466e-b638-7d43e34ecf53/deploy-status)](https://app.netlify.com/projects/frontpagekec/deploys)
[![GitHub](https://img.shields.io/badge/GitHub-aryansrao-181717?logo=github)](https://github.com/aryansrao)

## Overview
KEC Lab Manual Creator is a web application designed to help students of Krishna Engineering College (KEC) easily generate professional lab manual cover pages. The project was inspired by the challenges faced by students who do not have access to laptops or consistent software environments, resulting in inconsistent and time-consuming manual cover creation. This tool streamlines the process, ensuring uniformity and accessibility for all students, regardless of their device or operating system.

## Motivation
Many students struggle to create custom front pages for their lab manuals due to:
- Lack of access to laptops or desktop computers.
- Differences in operating systems and available software, leading to inconsistent designs.
- Time-consuming manual formatting and editing.

KEC Lab Manual Creator solves these problems by providing a simple, device-independent web interface that generates standardized, attractive lab manual covers in seconds.

## Features
- **Responsive Design:** Works seamlessly on mobile devices, tablets, and desktops.
- **Step-by-Step Form:** Guides users through entering subject, student, and faculty details.
- **Live Validation:** Ensures all inputs are correct before generating the cover.
- **Preview Mode:** Allows users to review the generated cover before downloading.
- **PDF Download:** Generates a high-quality PDF of the cover page for easy printing or sharing.
- **Toast Notifications & Loading Animations:** Provides feedback and improves user experience.
- **Modern UI & Animations:** Uses GSAP animations, smooth transitions, and a minimal aesthetic for a professional look.
- **Dark Mode & Accessibility:** Theme support (light/dark) and accessible UI controls.
- **Animated Menu & Loading Screen:** Pull-down menu and a lightweight loading screen replace the old PWA workflows.

## How It Works
1. **Fill the Form:** Enter all required details (subject, student, faculty, etc.).
2. **Preview:** Review the generated cover page for accuracy.
3. **Download:** Save the cover as a PDF, ready to print or submit.

## Technologies Used
- **HTML, CSS, JavaScript:** Core web technologies for structure, styling, and interactivity.
- **GSAP:** For polished animations and transitions.
- **html2canvas & jsPDF:** For rendering the cover and generating the PDF.
- **Google Fonts (Bebas Neue, Inter):** Typography used across the app.

## Folder Structure
```
frontpagekec/
├── index.html         # Main application file
├── keclogo.jpg        # College logo (used in the cover)
```

## Usage
1. Open https://frontpagekec.netlify.app/ in any modern browser (Chrome, Firefox, Edge, Safari).
2. Fill in the required details in the form.
3. Click "Generate Lab Manual Cover".
4. Preview the cover page.
5. Download the PDF and print or share as needed.

## Note on PWA
- This project no longer uses a PWA, service worker, or manifest. To keep the codebase lightweight and maintainable, it focuses on a single-page web app experience with client-side PDF generation and animations.
## Changelog (Short)
- Updated UI anc removed PWA and offline workflows.
- Added dark mode, animated menu (drag-to-open), and a minimal loading screen with GSAP animations.
- Improved accessibility, form validation, and consistent PDF output.

## Accessibility
- No installation required — works in any modern browser.
- Works on any device (including smartphones and tablets).
- No dependencies on OS or external software.

## Contributing
Pull requests and suggestions are welcome! If you have ideas to improve accessibility, add new features, or fix bugs, feel free to contribute.

## License
This project is open-source and free to use for educational purposes.

## Author
Created by Aryan S. Rao, inspired by the needs of KEC students for a simple, unified solution to lab manual cover creation.