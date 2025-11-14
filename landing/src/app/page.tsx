import Hero from '@/components/Hero'
import ProblemSolution from '@/components/ProblemSolution'
import HowItWorks from '@/components/HowItWorks'
import Roles from '@/components/Roles'
import Testimonials from '@/components/Testimonials'
import Pricing from '@/components/Pricing'
import FAQ from '@/components/FAQ'
import Footer from '@/components/Footer'

// Marketing components from SamCart & ClickFunnels
import CountdownTimer from '@/components/CountdownTimer'
import ScarcityBanner from '@/components/ScarcityBanner'
import SocialProofNotification from '@/components/SocialProofNotification'
import ExitIntentPopup from '@/components/ExitIntentPopup'
import TrustBadges from '@/components/TrustBadges'
import StickyCTABar from '@/components/StickyCTABar'

export default function Home() {
  return (
    <main className="min-h-screen">
      {/* Countdown Timer - Creates urgency (SamCart style) */}
      <CountdownTimer
        title="Limited Time Offer Ends In:"
        subtitle="Get 50% OFF Premium Yearly - Today Only!"
      />

      {/* Scarcity Banner - Shows limited spots (ClickFunnels style) */}
      <ScarcityBanner
        variant="spots"
        remainingCount={7}
        totalCount={50}
      />

      {/* Main sections */}
      <Hero />
      <ProblemSolution />
      <HowItWorks />
      <Roles />
      <Testimonials />

      {/* Trust Badges - Reduces friction (SamCart style) */}
      <TrustBadges />

      <Pricing />
      <FAQ />
      <Footer />

      {/* Floating elements */}
      {/* Social Proof - Real-time signups (ClickFunnels style) */}
      <SocialProofNotification />

      {/* Exit Intent Popup - Lead magnet (ClickFunnels style) */}
      <ExitIntentPopup />

      {/* Sticky CTA Bar - Always visible call-to-action */}
      <StickyCTABar />
    </main>
  )
}
