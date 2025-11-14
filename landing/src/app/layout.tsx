import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'English Liminal - Stop Studying English. Start Living It.',
  description: 'Master real-world English through immersive scenarios. Practice business English, job interviews, startup pitches, and sales conversations with AI-powered feedback.',
  keywords: 'english learning, business english, conversation practice, speaking english, professional english, learn english online, english app',
  authors: [{ name: 'English Liminal' }],
  openGraph: {
    title: 'English Liminal - Learn English Through Real-Life Situations',
    description: 'Stop studying grammar. Start living English. Master 26 real-world scenarios across 7 professional roles.',
    url: 'https://englishliminal.com',
    siteName: 'English Liminal',
    images: [
      {
        url: '/og-image.png',
        width: 1200,
        height: 630,
        alt: 'English Liminal - Real-world English learning',
      },
    ],
    locale: 'en_US',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'English Liminal - Stop Studying English. Start Living It.',
    description: 'Master real-world English through immersive scenarios with AI-powered feedback.',
    images: ['/og-image.png'],
  },
  robots: {
    index: true,
    follow: true,
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className="font-sans">{children}</body>
    </html>
  )
}
