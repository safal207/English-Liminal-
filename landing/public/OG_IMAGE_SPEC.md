# Open Graph Image Specifications

## Required Image: `og-image.png`

Create a 1200x630px image for social media sharing (Facebook, Twitter, LinkedIn).

### Design Specs

**Dimensions**: 1200 x 630 pixels
**Format**: PNG or JPG
**File size**: < 5MB (aim for < 500KB)
**Safe zone**: Keep important elements in center 1000x500px

### Design Elements

1. **Background**:
   - Gradient from blue (#0ea5e9) to purple (#d946ef)
   - Or: Clean white with subtle gradient overlay

2. **Main Text**:
   - Headline: "Stop Studying English. Start Living It."
   - Font: Bold, 60-80px
   - Color: White (on gradient) or Dark gray (on white)
   - Position: Center, slightly above middle

3. **Subtext**:
   - "Master 26 real-world scenarios across 7 professional roles"
   - Font: Regular, 32-40px
   - Color: White/80% opacity or Medium gray
   - Position: Below headline

4. **Logo/Branding**:
   - "English Liminal" logo or text
   - Position: Top left or bottom left
   - Size: 200-300px wide

5. **Visual Elements**:
   - Speech bubble icons
   - Microphone icon
   - Professional photos (optional)
   - App screenshot mockup (optional)

6. **Trust Indicators** (optional):
   - "10,000+ learners"
   - "4.8★ rating"
   - "26 scenarios"
   - Position: Bottom right corner, small text

### Tools to Create

**Option 1: Figma**
- Use template from Figma Community: "Social Media OG Image"
- Export as PNG @ 2x

**Option 2: Canva**
- Size: Custom 1200x630px
- Use "Social Media" template
- Export as PNG

**Option 3: Code (HTML + Puppeteer)**
```bash
# Use @vercel/og or similar
npm install @vercel/og
```

**Option 4: Design Tool**
- Photoshop, Sketch, or Affinity Designer
- Create artboard 1200x630px

### Testing

Before deploying, test the image:

1. **Facebook Debugger**: https://developers.facebook.com/tools/debug/
2. **Twitter Card Validator**: https://cards-dev.twitter.com/validator
3. **LinkedIn Post Inspector**: https://www.linkedin.com/post-inspector/

### File Naming

- Primary: `og-image.png`
- Twitter-specific (optional): `twitter-image.png` (1200x600px)
- Localized versions: `og-image-es.png`, `og-image-de.png`, etc.

### Example Text Variations

**Variation 1** (Current):
```
Stop Studying English. Start Living It.
Master real-world scenarios with AI-powered feedback.
```

**Variation 2**:
```
Learn English for the Job You Want
26 scenarios. 7 roles. Real results.
```

**Variation 3**:
```
From Textbook to Real World in 30 Days
Practice pitches, interviews, and sales calls.
```

### A/B Testing

Create 3 variations and test click-through rates:
- Version A: Gradient background + white text
- Version B: White background + dark text + app mockup
- Version C: Photo background + overlaid text

Track which gets more clicks in social ads.

---

**Once created, place `og-image.png` in `/landing/public/`**
