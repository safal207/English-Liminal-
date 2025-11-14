# Animations & Mobile Optimization

This document describes all animations and mobile optimizations added to the English Liminal landing page.

## 🎬 Animation Features

### Libraries Used
- **Framer Motion** (v10.16.16) - Production-ready motion library for React

### Animation Types

#### 1. **Scroll Animations**
All major sections reveal with smooth fade-in animations as you scroll.

```tsx
import ScrollReveal from '@/components/ScrollReveal'

<ScrollReveal direction="up" delay={0.2}>
  <YourComponent />
</ScrollReveal>
```

**Directions available:**
- `up` - Slides from bottom (default)
- `down` - Slides from top
- `left` - Slides from left
- `right` - Slides from right
- `scale` - Scales up from center

#### 2. **Parallax Effects**
Hero section uses parallax scrolling:
- Background blobs move slower than content
- Creates depth and dimension
- Smooth opacity fade on scroll

```tsx
const { scrollYProgress } = useScroll()
const opacity = useTransform(scrollYProgress, [0, 1], [1, 0])
```

#### 3. **Stagger Animations**
Elements appear sequentially with delays:

```tsx
const container = {
  visible: {
    transition: {
      staggerChildren: 0.1,  // 100ms between each child
      delayChildren: 0.2     // Wait 200ms before starting
    }
  }
}
```

**Used in:**
- Role cards (7 cards stagger in)
- Feature lists
- Pricing tiers
- Problem/Solution grids

#### 4. **Hover Animations**
Interactive micro-animations on hover:

**Buttons:**
- `whileHover={{ scale: 1.05 }}` - Slight grow effect
- `whileTap={{ scale: 0.95 }}` - Press down effect

**Cards:**
- `whileHover={{ scale: 1.02 }}` - Subtle lift
- Shadow increase on hover
- Border color changes

**Icons:**
- Rotate 180° (Problem section X icons)
- Scale 1.1x (Solution section icons)

#### 5. **Background Animations**
Floating gradient blobs in Hero:

```tsx
animate={{
  scale: [1, 1.2, 1],
  x: [0, 50, 0],
  y: [0, -30, 0],
}}
transition={{
  duration: 20,
  repeat: Infinity,
  ease: "easeInOut"
}}
```

Creates subtle, organic movement that doesn't distract.

#### 6. **Sequential Reveals**
Stars in Hero appear one-by-one:

```tsx
{[...Array(5)].map((_, i) => (
  <motion.div
    initial={{ opacity: 0, scale: 0 }}
    animate={{ opacity: 1, scale: 1 }}
    transition={{ duration: 0.3, delay: 0.4 + i * 0.1 }}
  >
    <Star />
  </motion.div>
))}
```

#### 7. **Smooth Scroll**
```css
html {
  scroll-behavior: smooth;
}
```

All anchor links (`#pricing`, `#roles`) scroll smoothly.

---

## 📱 Mobile Optimizations

### Responsive Design

#### Breakpoints
Using Tailwind CSS breakpoints:
- `default` - Mobile (< 640px)
- `sm:` - Small tablets (≥ 640px)
- `md:` - Medium tablets (≥ 768px)
- `lg:` - Desktop (≥ 1024px)

#### Typography Scaling

**Hero Headline:**
```tsx
className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl"
```
- Mobile: 36px (text-4xl)
- Tablet: 48px (text-5xl)
- Desktop: 72px (text-7xl)

**Body Text:**
```tsx
className="text-base sm:text-lg"
```
- Mobile: 16px
- Desktop: 18px

#### Spacing Adjustments

**Vertical Padding:**
```tsx
className="py-16 sm:py-24 lg:py-32"
```
- Mobile: 64px
- Tablet: 96px
- Desktop: 128px

**Gaps between elements:**
```tsx
className="gap-3 sm:gap-4 lg:gap-6"
```
Increases with screen size for better breathing room.

#### Layout Changes

**Hero CTA Buttons:**
```tsx
className="flex-col sm:flex-row"
```
- Mobile: Stacked vertically
- Tablet+: Side-by-side

**Role Cards:**
```tsx
className="grid-cols-1 lg:grid-cols-3"
```
- Mobile: 1 column
- Desktop: 3 columns

**Pricing Cards:**
```tsx
className="grid-cols-1 lg:grid-cols-3"
```
- Mobile: Stacked (scroll vertically)
- Desktop: 3 columns side-by-side

#### Touch Targets

All buttons/links minimum 44px height on mobile (WCAG guideline):
```tsx
className="py-3 sm:py-4"  // 48px on mobile
```

#### Mobile-First Utilities

**Full-width buttons on mobile:**
```tsx
className="w-full sm:w-auto"
```

**Horizontal padding:**
```tsx
className="px-4 sm:px-6 lg:px-8"
```
- Mobile: 16px
- Tablet: 24px
- Desktop: 32px

**Hidden elements on mobile:**
```tsx
className="hidden sm:block"  // Divider lines
```

#### Font Smoothing
```css
body {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
```

Ensures crisp text on all devices.

---

## 🎨 Animation Variants

Pre-built animation variants in `src/lib/animations.ts`:

```tsx
import { fadeIn, fadeInUp, scaleIn, slideIn } from '@/lib/animations'

// Fade in
<motion.div variants={fadeIn} />

// Slide up
<motion.div variants={fadeInUp} />

// Scale up
<motion.div variants={scaleIn} />

// Slide from direction
<motion.div variants={slideIn('left')} />

// Stagger children
<motion.div variants={staggerContainer}>
  <motion.div variants={staggerItem} />
  <motion.div variants={staggerItem} />
</motion.div>
```

---

## ⚡ Performance

### Optimization Techniques

1. **Viewport Detection**
   Only animate when element enters viewport:
   ```tsx
   whileInView="visible"
   viewport={{ once: true, margin: "-100px" }}
   ```

2. **`once: true`**
   Animation runs only once (not on every scroll).

3. **GPU Acceleration**
   All animations use `transform` and `opacity` (GPU-accelerated):
   - ✅ `transform: scale(1.05)`
   - ✅ `transform: translateY(20px)`
   - ✅ `opacity: 0.5`
   - ❌ `height: 200px` (causes reflow)

4. **Reduced Motion**
   Respects user preferences:
   ```tsx
   // Framer Motion automatically respects:
   @media (prefers-reduced-motion: reduce) {
     // Disables animations
   }
   ```

5. **Lazy Loading**
   Animations only load when component is visible.

---

## 🧪 Testing Animations

### Browser DevTools

**Chrome DevTools:**
1. Open DevTools (F12)
2. Command Palette (Cmd/Ctrl + Shift + P)
3. Type "Show Animations"
4. Record page interactions to inspect animations

### Mobile Testing

**Responsive Design Mode:**
```
Chrome: Cmd/Ctrl + Shift + M
```

**Test viewports:**
- iPhone SE (375px)
- iPhone 14 (390px)
- iPad (768px)
- Desktop (1280px)

### Performance Testing

```bash
npm run build
npm start

# Open Chrome DevTools > Lighthouse
# Run audit on Performance
```

Target metrics:
- First Contentful Paint: < 1.5s
- Time to Interactive: < 3.5s
- Cumulative Layout Shift: < 0.1

---

## 🎯 Animation Guidelines

### DO:
✅ Use subtle animations (scale 1.02-1.05, not 2x)
✅ Keep durations short (0.2-0.8s)
✅ Use easing functions (`[0.25, 0.46, 0.45, 0.94]`)
✅ Stagger multiple elements (100-150ms delay)
✅ Respect `prefers-reduced-motion`

### DON'T:
❌ Animate on every scroll (use `once: true`)
❌ Use long durations (> 1s feels slow)
❌ Animate height/width (causes reflow)
❌ Over-animate (too much = distraction)

---

## 📊 Animation Inventory

### Components with Animations

| Component | Animations Used |
|-----------|----------------|
| **Hero** | Parallax, stagger, fade-in, floating blobs, hover scale |
| **ProblemSolution** | Stagger grid, fade-in, hover scale, icon rotate/scale |
| **Roles** | Stagger cards, fade-in list items, hover lift |
| **Pricing** | Directional reveals (left/scale/right), hover lift, badge pop-in |
| **Testimonials** | Scroll reveal, hover shadow |
| **FAQ** | Accordion slide, chevron rotate |
| **HowItWorks** | Step reveals, hover scale |

---

## 🚀 Future Enhancements

Potential additions:
- [ ] Cursor follow effect on CTAs
- [ ] Page transition animations
- [ ] Loading skeleton screens
- [ ] Confetti on button click (conversion celebration)
- [ ] Progress indicator on scroll
- [ ] Animated counters (10,000+ learners)
- [ ] Video autoplay on scroll into view

---

## 🛠️ Troubleshooting

**Issue: Animations not showing**
- Check `'use client'` directive at top of file
- Ensure Framer Motion is installed: `npm install framer-motion`

**Issue: Janky animations on mobile**
- Use `will-change: transform` sparingly
- Reduce number of animated elements
- Test on real device (not just simulator)

**Issue: Layout shift during animations**
- Set explicit dimensions on animated elements
- Use `overflow: hidden` on parent

---

**Questions?** Check [Framer Motion Docs](https://www.framer.com/motion/)
