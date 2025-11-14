'use client'

import { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { ArrowRight, X } from 'lucide-react'

export default function StickyCTABar() {
  const [isVisible, setIsVisible] = useState(false)
  const [isDismissed, setIsDismissed] = useState(false)

  useEffect(() => {
    // Check if bar was dismissed in this session
    const dismissed = sessionStorage.getItem('ctaBarDismissed')
    if (dismissed) {
      setIsDismissed(true)
      return
    }

    const handleScroll = () => {
      // Show bar after scrolling 50% of page
      const scrollPercent = (window.scrollY / (document.documentElement.scrollHeight - window.innerHeight)) * 100

      if (scrollPercent > 50 && !isDismissed) {
        setIsVisible(true)
      }
    }

    window.addEventListener('scroll', handleScroll)
    return () => window.removeEventListener('scroll', handleScroll)
  }, [isDismissed])

  const handleDismiss = () => {
    setIsVisible(false)
    setIsDismissed(true)
    sessionStorage.setItem('ctaBarDismissed', 'true')
  }

  return (
    <AnimatePresence>
      {isVisible && !isDismissed && (
        <motion.div
          className="fixed bottom-0 left-0 right-0 z-40 bg-gradient-to-r from-blue-600 to-purple-600 shadow-2xl"
          initial={{ y: 100, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: 100, opacity: 0 }}
          transition={{ type: "spring", stiffness: 300, damping: 30 }}
        >
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-3 sm:py-4">
            <div className="flex items-center justify-between gap-4">
              {/* Text */}
              <div className="flex-1 min-w-0">
                <p className="text-white font-semibold text-sm sm:text-base truncate">
                  Ready to master real-world English?
                </p>
                <p className="text-white/80 text-xs sm:text-sm hidden sm:block">
                  Start your 7-day free trial today. No credit card required.
                </p>
              </div>

              {/* CTA Button */}
              <motion.a
                href="#pricing"
                className="flex-shrink-0 inline-flex items-center gap-2 bg-white text-blue-600 font-semibold px-4 sm:px-6 py-2 sm:py-3 rounded-full text-sm hover:bg-gray-50 transition-all shadow-lg"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
              >
                <span className="hidden sm:inline">Get Started Free</span>
                <span className="sm:hidden">Start Free</span>
                <ArrowRight className="w-4 h-4" />
              </motion.a>

              {/* Close button */}
              <button
                onClick={handleDismiss}
                className="flex-shrink-0 text-white/60 hover:text-white transition-colors ml-2"
                aria-label="Dismiss"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  )
}
