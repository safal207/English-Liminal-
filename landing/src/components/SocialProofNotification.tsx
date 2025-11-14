'use client'

import { useEffect, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { CheckCircle, X } from 'lucide-react'

interface Notification {
  id: number
  name: string
  location: string
  plan: string
  timeAgo: string
}

const notifications: Notification[] = [
  { id: 1, name: "Maria Santos", location: "Lisbon, Portugal", plan: "Premium Monthly", timeAgo: "2 minutes ago" },
  { id: 2, name: "Alex Chen", location: "Singapore", plan: "Premium Yearly", timeAgo: "5 minutes ago" },
  { id: 3, name: "Sarah Johnson", location: "London, UK", plan: "Premium Monthly", timeAgo: "8 minutes ago" },
  { id: 4, name: "Raj Patel", location: "Mumbai, India", plan: "Free Trial", timeAgo: "12 minutes ago" },
  { id: 5, name: "Carlos Rivera", location: "Barcelona, Spain", plan: "Premium Yearly", timeAgo: "15 minutes ago" },
  { id: 6, name: "Emma Schmidt", location: "Berlin, Germany", plan: "Premium Monthly", timeAgo: "18 minutes ago" },
  { id: 7, name: "Yuki Tanaka", location: "Tokyo, Japan", plan: "Free Trial", timeAgo: "22 minutes ago" },
  { id: 8, name: "Ahmed Hassan", location: "Dubai, UAE", plan: "Premium Yearly", timeAgo: "25 minutes ago" },
]

export default function SocialProofNotification() {
  const [currentNotification, setCurrentNotification] = useState<Notification | null>(null)
  const [isVisible, setIsVisible] = useState(false)

  useEffect(() => {
    const timers: NodeJS.Timeout[] = []

    const showRandomNotification = () => {
      const randomIndex = Math.floor(Math.random() * notifications.length)
      setCurrentNotification(notifications[randomIndex])
      setIsVisible(true)

      // Hide after 5 seconds
      const hideTimer = setTimeout(() => {
        setIsVisible(false)
      }, 5000)
      timers.push(hideTimer)

      // Show next notification after 20-40 seconds
      const nextDelay = 20000 + Math.random() * 20000
      const nextTimer = setTimeout(() => {
        showRandomNotification()
      }, nextDelay)
      timers.push(nextTimer)
    }

    // Show first notification after 5 seconds
    const initialTimer = setTimeout(() => {
      showRandomNotification()
    }, 5000)
    timers.push(initialTimer)

    return () => {
      // Clear all timers on unmount
      timers.forEach(timer => clearTimeout(timer))
    }
  }, [])

  const handleClose = () => {
    setIsVisible(false)
  }

  return (
    <AnimatePresence>
      {isVisible && currentNotification && (
        <motion.div
          className="fixed bottom-4 left-4 z-50 max-w-xs sm:max-w-sm"
          initial={{ x: -400, opacity: 0 }}
          animate={{ x: 0, opacity: 1 }}
          exit={{ x: -400, opacity: 0 }}
          transition={{ type: "spring", stiffness: 100, damping: 20 }}
        >
          <div className="bg-white rounded-xl shadow-2xl ring-1 ring-black/5 overflow-hidden">
            <div className="relative p-4">
              {/* Close button */}
              <button
                onClick={handleClose}
                className="absolute top-2 right-2 text-gray-400 hover:text-gray-600 transition-colors"
                aria-label="Close notification"
              >
                <X className="w-4 h-4" />
              </button>

              {/* Content */}
              <div className="flex items-start gap-3">
                {/* Icon */}
                <motion.div
                  initial={{ scale: 0 }}
                  animate={{ scale: 1 }}
                  transition={{ delay: 0.2, type: "spring", stiffness: 200 }}
                >
                  <CheckCircle className="w-8 h-8 text-green-500 flex-shrink-0" />
                </motion.div>

                {/* Text */}
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-semibold text-gray-900">
                    {currentNotification.name}
                  </p>
                  <p className="text-xs text-gray-600 mt-0.5">
                    from {currentNotification.location}
                  </p>
                  <p className="text-xs text-gray-500 mt-1">
                    Just signed up for <span className="font-semibold text-blue-600">{currentNotification.plan}</span>
                  </p>
                  <p className="text-[10px] text-gray-400 mt-1">
                    {currentNotification.timeAgo}
                  </p>
                </div>
              </div>

              {/* Progress bar */}
              <motion.div
                className="absolute bottom-0 left-0 h-1 bg-gradient-to-r from-blue-500 to-purple-500"
                initial={{ width: "100%" }}
                animate={{ width: "0%" }}
                transition={{ duration: 5, ease: "linear" }}
              />
            </div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  )
}
