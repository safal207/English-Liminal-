# English Liminal Landing Page

Production-ready landing page for English Liminal built with Next.js 14, TypeScript, and Tailwind CSS.

## 🚀 Features

- **Modern Stack**: Next.js 14 App Router + TypeScript + Tailwind CSS
- **SEO Optimized**: Meta tags, Open Graph, Twitter Cards
- **Responsive Design**: Mobile-first, works on all devices
- **Performance**: Static generation, optimized images
- **Conversion Focused**: 8 sections designed for conversions

## 📦 What's Included

### Sections

1. **Hero** - Bold headline with CTA and trust indicators
2. **Problem/Solution** - Why traditional learning fails + our approach
3. **How It Works** - 4-step learning process
4. **Roles** - 7 professional roles with 26 scenarios
5. **Testimonials** - Social proof from real users
6. **Pricing** - Free vs Premium comparison
7. **FAQ** - 8 common questions with answers
8. **Footer** - Links, newsletter signup, social media

### Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Icons**: Lucide React
- **Deployment**: Vercel (recommended)

## 🛠️ Development

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
cd landing
npm install
```

### Run Development Server

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000)

### Build for Production

```bash
npm run build
npm start
```

## 🚀 Deployment to Vercel

### Option 1: Vercel CLI (Recommended)

```bash
# Install Vercel CLI
npm i -g vercel

# Login
vercel login

# Deploy
vercel

# Deploy to production
vercel --prod
```

### Option 2: GitHub Integration

1. Push code to GitHub
2. Go to [vercel.com/new](https://vercel.com/new)
3. Import your repository
4. Vercel auto-detects Next.js
5. Click "Deploy"

### Environment Variables

No environment variables required for basic deployment.

Optional (for analytics/tracking):
- `NEXT_PUBLIC_GA_ID` - Google Analytics
- `NEXT_PUBLIC_MIXPANEL_TOKEN` - Mixpanel

## 📁 Project Structure

```
landing/
├── src/
│   ├── app/
│   │   ├── layout.tsx      # Root layout with SEO
│   │   ├── page.tsx        # Home page
│   │   └── globals.css     # Global styles
│   └── components/
│       ├── Hero.tsx
│       ├── ProblemSolution.tsx
│       ├── HowItWorks.tsx
│       ├── Roles.tsx
│       ├── Testimonials.tsx
│       ├── Pricing.tsx
│       ├── FAQ.tsx
│       └── Footer.tsx
├── public/                 # Static assets
├── package.json
├── tailwind.config.ts
├── tsconfig.json
└── next.config.js
```

## 🎨 Customization

### Colors

Edit `tailwind.config.ts`:

```typescript
colors: {
  primary: { ... },  // Blue palette
  accent: { ... },   // Purple palette
}
```

### Content

- **Copy**: Edit component files directly
- **Roles**: Update `Roles.tsx` with new roles/scenarios
- **Pricing**: Modify `Pricing.tsx` for price changes
- **FAQ**: Add/edit questions in `FAQ.tsx`

### SEO

Edit `src/app/layout.tsx`:

```typescript
export const metadata: Metadata = {
  title: '...',
  description: '...',
  // ... other metadata
}
```

## 📊 Analytics Integration

### Google Analytics

1. Add GA4 ID to environment variables
2. Create `src/lib/gtag.ts`:

```typescript
export const GA_TRACKING_ID = process.env.NEXT_PUBLIC_GA_ID

export const pageview = (url: string) => {
  window.gtag('config', GA_TRACKING_ID, {
    page_path: url,
  })
}
```

3. Add to `layout.tsx`

### Mixpanel

```bash
npm install mixpanel-browser
```

Create tracking wrapper in `src/lib/analytics.ts`

## 🔗 Related

- **Main App**: `../app` (Flutter)
- **Backend**: `../core` (Rust)
- **Marketing Copy**: `../marketing/landing-page/LANDING_PAGE_COPY.md`

## 📈 Performance

Target metrics:
- **Lighthouse Score**: 95+
- **First Contentful Paint**: < 1.5s
- **Time to Interactive**: < 3.5s
- **Cumulative Layout Shift**: < 0.1

Run Lighthouse:
```bash
npm run build
npm start
# Open Chrome DevTools > Lighthouse
```

## 🐛 Troubleshooting

**Issue**: `Module not found: Can't resolve '@/components/...'`
- **Fix**: Check `tsconfig.json` has correct path mapping

**Issue**: Tailwind styles not applying
- **Fix**: Ensure `globals.css` imports Tailwind directives

**Issue**: Build fails on Vercel
- **Fix**: Check Node.js version (18+) in Vercel settings

## 📝 TODO

- [ ] Add email newsletter integration (Mailchimp/ConvertKit)
- [ ] Implement exit intent popup
- [ ] Add app screenshots/mockups
- [ ] Create OG image (1200x630)
- [ ] Set up A/B testing (Optimizely/VWO)
- [ ] Add live chat widget (Intercom/Crisp)
- [ ] Implement cookie consent banner

## 🤝 Contributing

1. Make changes in a new branch
2. Test locally: `npm run dev`
3. Build: `npm run build`
4. Create PR

## 📄 License

Part of English Liminal project.

---

**Questions?** Email dev@englishliminal.com
