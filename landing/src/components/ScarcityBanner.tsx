'use client'

import { motion } from 'framer-motion'
import { Flame, Users } from 'lucide-react'

interface ScarcityBannerProps {
  variant?: 'spots' | 'discount' | 'seats'
  remainingCount?: number
  totalCount?: number
}

export default function ScarcityBanner({
  variant = 'spots',
  remainingCount = 7,
  totalCount = 50
}: ScarcityBannerProps) {
  const messages = {
    spots: {
      icon: Flame,
      text: `Only ${remainingCount} spots left at this price!`,
      subtext: `${totalCount - remainingCount} people joined this week`,
      color: 'from-red-500 to-orange-500'
    },
    discount: {
      icon: Flame,
      text: `50% OFF expires in 24 hours!`,
      subtext: `Don't miss this limited-time offer`,
      color: 'from-orange-500 to-yellow-500'
    },
    seats: {
      icon: Users,
      text: `${remainingCount}/${totalCount} seats available`,
      subtext: `Filling up fast - secure your spot now`,
      color: 'from-purple-500 to-pink-500'
    }
  }

  const config = messages[variant]
  const percentage = ((totalCount - remainingCount) / totalCount) * 100

  return (
    <motion.div
      className={`bg-gradient-to-r ${config.color} py-3 sm:py-4`}
      initial={{ y: -50, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ duration: 0.6, delay: 0.5 }}
    >
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 text-center sm:text-left">
          {/* Icon with pulse */}
          <motion.div
            animate={{
              scale: [1, 1.2, 1],
            }}
            transition={{
              duration: 2,
              repeat: Infinity,
              ease: "easeInOut"
            }}
          >
            <config.icon className="w-6 h-6 sm:w-7 sm:h-7 text-white" />
          </motion.div>

          {/* Text */}
          <div className="flex-1">
            <p className="text-white font-bold text-sm sm:text-base">
              {config.text}
            </p>
            <p className="text-white/90 text-xs sm:text-sm mt-0.5">
              {config.subtext}
            </p>
          </div>

          {/* Progress bar (for spots/seats variant) */}
          {(variant === 'spots' || variant === 'seats') && (
            <div className="w-full sm:w-48">
              <div className="bg-white/20 rounded-full h-2 overflow-hidden">
                <motion.div
                  className="bg-white h-full rounded-full"
                  initial={{ width: 0 }}
                  animate={{ width: `${percentage}%` }}
                  transition={{ duration: 1, delay: 0.5 }}
                />
              </div>
              <p className="text-white/80 text-xs text-center mt-1">
                {Math.round(percentage)}% claimed
              </p>
            </div>
          )}
        </div>
      </div>
    </motion.div>
  )
}
