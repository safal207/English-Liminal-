'use client'

import { motion } from 'framer-motion'
import { Shield, Lock, RefreshCw, Award, Zap, Users } from 'lucide-react'
import ScrollReveal from './ScrollReveal'

export default function TrustBadges() {
  const badges = [
    {
      icon: Shield,
      title: "30-Day Money-Back",
      description: "Don't like it? Get a full refund. No questions asked.",
      color: "text-green-600"
    },
    {
      icon: Lock,
      title: "Secure Payment",
      description: "SSL encrypted. Your data is 100% safe with us.",
      color: "text-blue-600"
    },
    {
      icon: RefreshCw,
      title: "Cancel Anytime",
      description: "No contracts. No commitments. Cancel with one click.",
      color: "text-purple-600"
    },
    {
      icon: Award,
      title: "4.8★ Rated",
      description: "10,000+ learners trust English Liminal.",
      color: "text-yellow-600"
    },
    {
      icon: Zap,
      title: "Instant Access",
      description: "Start learning in 60 seconds. No setup required.",
      color: "text-orange-600"
    },
    {
      icon: Users,
      title: "Expert Support",
      description: "Real humans ready to help. Not bots.",
      color: "text-indigo-600"
    }
  ]

  return (
    <section className="bg-gray-50 py-12 sm:py-16 border-t border-b border-gray-200">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <ScrollReveal>
          <div className="text-center mb-8 sm:mb-12">
            <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-2">
              Risk-Free Learning
            </h2>
            <p className="text-sm sm:text-base text-gray-600">
              We're so confident you'll love English Liminal, we offer a 30-day money-back guarantee.
            </p>
          </div>
        </ScrollReveal>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
          {badges.map((badge, index) => (
            <ScrollReveal key={badge.title} delay={index * 0.1}>
              <motion.div
                className="bg-white rounded-xl p-6 shadow-sm border border-gray-100 hover:shadow-md transition-shadow"
                whileHover={{ y: -4 }}
              >
                <div className="flex items-start gap-4">
                  <div className={`p-3 rounded-lg bg-gray-50 ${badge.color}`}>
                    <badge.icon className="w-6 h-6" />
                  </div>
                  <div className="flex-1">
                    <h3 className="font-semibold text-gray-900 mb-1 text-sm sm:text-base">
                      {badge.title}
                    </h3>
                    <p className="text-xs sm:text-sm text-gray-600">
                      {badge.description}
                    </p>
                  </div>
                </div>
              </motion.div>
            </ScrollReveal>
          ))}
        </div>

        {/* Guarantee seal */}
        <ScrollReveal delay={0.6}>
          <div className="mt-12 text-center">
            <div className="inline-flex items-center gap-3 bg-gradient-to-r from-green-50 to-blue-50 px-6 py-4 rounded-full border-2 border-green-200">
              <Shield className="w-8 h-8 text-green-600" />
              <div className="text-left">
                <div className="font-bold text-gray-900 text-sm sm:text-base">100% Money-Back Guarantee</div>
                <div className="text-xs sm:text-sm text-gray-600">Try risk-free for 30 days</div>
              </div>
            </div>
          </div>
        </ScrollReveal>
      </div>
    </section>
  )
}
