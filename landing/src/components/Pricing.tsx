'use client'

import { Check, Sparkles } from 'lucide-react'
import { motion } from 'framer-motion'
import ScrollReveal from './ScrollReveal'

export default function Pricing() {
  const plans = [
    {
      name: 'Free',
      price: '$0',
      period: 'forever',
      description: 'Perfect for trying out English Liminal',
      features: [
        '3 professional roles',
        '12 real-world scenarios',
        'AI speech recognition',
        'Progress tracking',
        'Mobile & web access',
        'Community support',
      ],
      limitations: [
        'Limited to free roles only',
        'No premium scenarios',
        'Basic analytics',
      ],
      cta: 'Start Free',
      ctaLink: '#',
      highlighted: false,
    },
    {
      name: 'Premium Monthly',
      price: '$9.99',
      period: '/month',
      description: 'Full access to all roles and scenarios',
      features: [
        'All 7 professional roles',
        'All 26 real-world scenarios',
        'Advanced speech recognition',
        'Detailed progress analytics',
        'Spaced repetition system',
        'Priority support',
        'New scenarios monthly',
        'Export progress reports',
        'Custom learning path',
      ],
      cta: 'Start 7-Day Free Trial',
      ctaLink: '#',
      highlighted: true,
      badge: 'Most Popular',
    },
    {
      name: 'Premium Yearly',
      price: '$79.99',
      period: '/year',
      originalPrice: '$119.88',
      savings: 'Save 33%',
      description: 'Best value for serious learners',
      features: [
        'Everything in Monthly',
        'Save $40 per year',
        'Priority feature access',
        'Annual progress report',
        'Exclusive community',
        'Early access to new roles',
      ],
      cta: 'Start 7-Day Free Trial',
      ctaLink: '#',
      highlighted: false,
    },
  ]

  return (
    <section id="pricing" className="bg-white py-16 sm:py-24 lg:py-32">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <ScrollReveal>
          <div className="mx-auto max-w-2xl text-center">
            <h2 className="text-base font-semibold leading-7 text-blue-600">Pricing</h2>
            <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
              Choose Your Plan
            </p>
            <p className="mt-4 sm:mt-6 text-base sm:text-lg leading-7 sm:leading-8 text-gray-600 px-4 sm:px-0">
              Start free. Upgrade when you're ready. Cancel anytime.
            </p>
          </div>
        </ScrollReveal>

        <div className="mx-auto mt-12 sm:mt-16 grid max-w-lg grid-cols-1 gap-6 sm:gap-8 lg:max-w-none lg:grid-cols-3">
          {plans.map((plan, index) => (
            <ScrollReveal
              key={plan.name}
              delay={index * 0.15}
              direction={index === 0 ? 'left' : index === 2 ? 'right' : 'scale'}
            >
              <motion.div
                className={`relative rounded-2xl sm:rounded-3xl p-6 sm:p-8 h-full ${
                  plan.highlighted
                    ? 'bg-gradient-to-br from-blue-600 to-purple-600 text-white shadow-2xl ring-2 ring-blue-600'
                    : 'bg-white ring-1 ring-gray-200'
                }`}
                whileHover={{
                  scale: 1.03,
                  boxShadow: plan.highlighted
                    ? '0 25px 50px -12px rgba(59, 130, 246, 0.5)'
                    : '0 20px 25px -5px rgba(0, 0, 0, 0.1)',
                  transition: { duration: 0.3 }
                }}
              >
                {/* Badge */}
                {plan.badge && (
                  <motion.div
                    className="absolute -top-4 left-1/2 -translate-x-1/2"
                    initial={{ y: -10, opacity: 0 }}
                    animate={{ y: 0, opacity: 1 }}
                    transition={{ delay: 0.5 + index * 0.15, duration: 0.5 }}
                  >
                    <span className="inline-flex items-center gap-1 rounded-full bg-gradient-to-r from-yellow-400 to-orange-500 px-3 sm:px-4 py-1 text-xs sm:text-sm font-semibold text-white shadow-lg">
                      <Sparkles className="w-3 h-3 sm:w-4 sm:h-4" />
                      {plan.badge}
                    </span>
                  </motion.div>
                )}

                {/* Plan name */}
                <h3 className={`text-lg sm:text-xl font-semibold ${plan.highlighted ? 'text-white' : 'text-gray-900'}`}>
                  {plan.name}
                </h3>

                {/* Description */}
                <p className={`mt-2 text-xs sm:text-sm ${plan.highlighted ? 'text-blue-100' : 'text-gray-600'}`}>
                  {plan.description}
                </p>

                {/* Price */}
                <div className="mt-4 sm:mt-6">
                  <div className="flex items-baseline gap-1">
                    <span className={`text-4xl sm:text-5xl font-bold tracking-tight ${plan.highlighted ? 'text-white' : 'text-gray-900'}`}>
                      {plan.price}
                    </span>
                    <span className={`text-sm font-semibold ${plan.highlighted ? 'text-blue-100' : 'text-gray-600'}`}>
                      {plan.period}
                    </span>
                  </div>
                  {plan.originalPrice && (
                    <div className="mt-2 flex items-center gap-2">
                      <span className="text-xs sm:text-sm text-white/70 line-through">{plan.originalPrice}</span>
                      <span className="inline-flex items-center rounded-full bg-green-500 px-2 py-0.5 text-xs font-semibold text-white">
                        {plan.savings}
                      </span>
                    </div>
                  )}
                </div>

                {/* CTA Button */}
                <motion.a
                  href={plan.ctaLink}
                  className={`mt-6 sm:mt-8 block w-full rounded-full py-2.5 sm:py-3 px-4 sm:px-6 text-center text-sm font-semibold transition-all ${
                    plan.highlighted
                      ? 'bg-white text-blue-600 hover:bg-gray-50 shadow-lg'
                      : 'bg-blue-600 text-white hover:bg-blue-700 ring-1 ring-blue-600'
                  }`}
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                >
                  {plan.cta}
                </motion.a>

                {/* Features */}
                <ul className="mt-6 sm:mt-8 space-y-2 sm:space-y-3">
                  {plan.features.map((feature, idx) => (
                    <motion.li
                      key={feature}
                      className="flex items-start gap-2 sm:gap-3"
                      initial={{ opacity: 0, x: -10 }}
                      whileInView={{ opacity: 1, x: 0 }}
                      viewport={{ once: true }}
                      transition={{ delay: 0.3 + idx * 0.05 }}
                    >
                      <Check className={`h-4 w-4 sm:h-5 sm:w-5 flex-shrink-0 ${plan.highlighted ? 'text-white' : 'text-blue-600'}`} />
                      <span className={`text-xs sm:text-sm ${plan.highlighted ? 'text-blue-50' : 'text-gray-600'}`}>
                        {feature}
                      </span>
                    </motion.li>
                  ))}
                </ul>

                {/* Limitations (for Free plan) */}
                {plan.limitations && (
                  <div className="mt-4 sm:mt-6 pt-4 sm:pt-6 border-t border-gray-200">
                    <p className="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2 sm:mb-3">
                      Limitations:
                    </p>
                    <ul className="space-y-1 sm:space-y-2">
                      {plan.limitations.map((limitation) => (
                        <li key={limitation} className="text-xs sm:text-sm text-gray-500">
                          • {limitation}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}
              </motion.div>
            </ScrollReveal>
          ))}
        </div>

        {/* Trust indicators */}
        <ScrollReveal delay={0.5}>
          <div className="mt-12 sm:mt-16 text-center">
            <div className="flex flex-wrap items-center justify-center gap-4 sm:gap-8 text-xs sm:text-sm text-gray-600 px-4">
              <div className="flex items-center gap-2">
                <svg className="w-4 h-4 sm:w-5 sm:h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                </svg>
                <span>7-day free trial</span>
              </div>
              <div className="hidden sm:block h-4 w-px bg-gray-300"></div>
              <div className="flex items-center gap-2">
                <svg className="w-4 h-4 sm:w-5 sm:h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                </svg>
                <span>Cancel anytime</span>
              </div>
              <div className="hidden sm:block h-4 w-px bg-gray-300"></div>
              <div className="flex items-center gap-2">
                <svg className="w-4 h-4 sm:w-5 sm:h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                </svg>
                <span>No credit card for trial</span>
              </div>
            </div>
          </div>
        </ScrollReveal>
      </div>
    </section>
  )
}
