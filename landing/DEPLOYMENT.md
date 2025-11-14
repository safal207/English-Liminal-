# Landing Page Deployment Guide

## GitHub Pages Deployment

### Automatic Deployment

The landing page automatically deploys to GitHub Pages when changes are pushed to `main` branch.

**Live URL:** https://safal207.github.io/English-Liminal-/

### Manual Deployment

To manually trigger deployment:
1. Go to: https://github.com/safal207/English-Liminal-/actions
2. Click "Deploy Landing Page to GitHub Pages"
3. Click "Run workflow"
4. Select `main` branch
5. Click "Run workflow" button

### Local Testing

Test the production build locally:

```bash
cd landing
npm run build
npx serve out
```

Open http://localhost:3000

### Configuration

**next.config.js:**
- `output: 'export'` - Enables static export
- `basePath: '/English-Liminal-'` - GitHub Pages base path
- `assetPrefix: '/English-Liminal-/'` - Asset loading path
- `images.unoptimized: true` - Disable Next.js image optimization
- `trailingSlash: true` - Better GitHub Pages compatibility

### GitHub Pages Settings

To enable GitHub Pages (first time):
1. Go to: https://github.com/safal207/English-Liminal-/settings/pages
2. Source: "GitHub Actions"
3. Save

The workflow will automatically deploy the site.

### Troubleshooting

**Issue: 404 errors**
- Check `basePath` matches repository name
- Ensure `.nojekyll` file exists in `public/` directory

**Issue: CSS/JS not loading**
- Verify `assetPrefix` is set correctly
- Check browser console for path errors

**Issue: Images not showing**
- Ensure `images.unoptimized: true` is set
- Use relative paths for images

**Issue: Build fails**
- Check Node.js version (requires 18+)
- Verify all dependencies are in `package.json`
- Test build locally first

### Custom Domain (Optional)

To use a custom domain:
1. Add `CNAME` file to `landing/public/` with your domain:
   ```
   landing.englishliminal.com
   ```
2. Configure DNS:
   - Type: CNAME
   - Name: landing (or @)
   - Value: safal207.github.io
3. Enable HTTPS in GitHub Pages settings

### Alternative: Vercel Deployment

For better performance, deploy to Vercel instead:

```bash
cd landing
npm install -g vercel
vercel
```

Vercel advantages:
- Faster builds
- Edge caching
- Better analytics
- No base path needed
- Image optimization works

### Monitoring

Check deployment status:
- Actions: https://github.com/safal207/English-Liminal-/actions
- Pages: https://github.com/safal207/English-Liminal-/deployments

### Performance

GitHub Pages is free but has limitations:
- 100 GB/month bandwidth
- 1 GB repository size limit
- Static files only

For production, consider:
- Vercel (free tier: 100 GB/month)
- Netlify (free tier: 100 GB/month)
- Cloudflare Pages (unlimited bandwidth)

---

**Questions?** Check the GitHub Actions logs for detailed error messages.
