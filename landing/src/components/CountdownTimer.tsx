'use client'

import { useEffect, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Clock } from 'lucide-react'

interface CountdownTimerProps {
  title?: string
  subtitle?: string
  endDate?: Date
  onExpire?: () => void
}

export default function CountdownTimer({
  title = "Limited Time Offer Ends In:",
  subtitle = "Get 50% OFF Premium Yearly",
  endDate,
  onExpire
}: CountdownTimerProps) {
  // Default: 24 hours from now
  const defaultEndDate = new Date()
  defaultEndDate.setHours(defaultEndDate.getHours() + 24)

  const targetDate = endDate || defaultEndDate

  const [timeLeft, setTimeLeft] = useState({
    days: 0,
    hours: 0,
    minutes: 0,
    seconds: 0
  })

  const [isExpired, setIsExpired] = useState(false)

  useEffect(() => {
    const calculateTimeLeft = () => {
      const difference = targetDate.getTime() - new Date().getTime()

      if (difference <= 0) {
        setIsExpired(true)
        if (onExpire) onExpire()
        return {
          days: 0,
          hours: 0,
          minutes: 0,
          seconds: 0
        }
      }

      return {
        days: Math.floor(difference / (1000 * 60 * 60 * 24)),
        hours: Math.floor((difference / (1000 * 60 * 60)) % 24),
        minutes: Math.floor((difference / 1000 / 60) % 60),
        seconds: Math.floor((difference / 1000) % 60)
      }
    }

    setTimeLeft(calculateTimeLeft())

    const timer = setInterval(() => {
      setTimeLeft(calculateTimeLeft())
    }, 1000)

    return () => clearInterval(timer)
  }, [targetDate, onExpire])

  if (isExpired) return null

  return (
    <motion.div
      className="bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 py-4 sm:py-6"
      initial={{ y: -100, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ duration: 0.6, delay: 1.5 }}
    >
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex flex-col items-center text-center">
          {/* Title */}
          <div className="flex items-center gap-2 mb-3 sm:mb-4">
            <Clock className="w-5 h-5 sm:w-6 sm:h-6 text-white animate-pulse" />
            <h3 className="text-base sm:text-lg font-bold text-white">
              {title}
            </h3>
          </div>

          {/* Subtitle */}
          {subtitle && (
            <p className="text-xs sm:text-sm text-white/90 mb-3 sm:mb-4">
              {subtitle}
            </p>
          )}

          {/* Timer */}
          <div className="flex items-center gap-2 sm:gap-4">
            {Object.entries(timeLeft).map(([unit, value]) => (
              <motion.div
                key={unit}
                className="flex flex-col items-center bg-white/20 backdrop-blur-sm rounded-lg px-3 py-2 sm:px-4 sm:py-3 min-w-[60px] sm:min-w-[80px]"
                whileHover={{ scale: 1.05 }}
              >
                <AnimatePresence mode="wait">
                  <motion.span
                    key={value}
                    className="text-2xl sm:text-4xl font-bold text-white tabular-nums"
                    initial={{ y: -20, opacity: 0 }}
                    animate={{ y: 0, opacity: 1 }}
                    exit={{ y: 20, opacity: 0 }}
                    transition={{ duration: 0.2 }}
                  >
                    {String(value).padStart(2, '0')}
                  </motion.span>
                </AnimatePresence>
                <span className="text-[10px] sm:text-xs text-white/80 uppercase mt-1 font-semibold">
                  {unit}
                </span>
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </motion.div>
  )
}
