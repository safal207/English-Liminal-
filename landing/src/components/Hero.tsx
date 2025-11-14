'use client'

import { ArrowRight, Star } from 'lucide-react'
import { motion, useScroll, useTransform } from 'framer-motion'
import { useRef } from 'react'

export default function Hero() {
  const ref = useRef(null)
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start start", "end start"]
  })

  const opacity = useTransform(scrollYProgress, [0, 1], [1, 0])
  const scale = useTransform(scrollYProgress, [0, 1], [1, 0.95])
  const y = useTransform(scrollYProgress, [0, 1], [0, 100])

  return (
    <section ref={ref} className="relative overflow-hidden bg-gradient-to-b from-blue-50 to-white pt-20 pb-16 sm:pt-32 sm:pb-24">
      {/* Animated background decoration */}
      <motion.div
        className="absolute inset-0 -z-10"
        style={{ opacity }}
      >
        <motion.div
          className="absolute top-0 right-0 w-64 h-64 sm:w-96 sm:h-96 bg-blue-100 rounded-full blur-3xl opacity-50"
          animate={{
            scale: [1, 1.2, 1],
            x: [0, 50, 0],
            y: [0, -30, 0],
          }}
          transition={{
            duration: 20,
            repeat: Infinity,
            ease: "easeInOut"
          }}
        ></motion.div>
        <motion.div
          className="absolute bottom-0 left-0 w-64 h-64 sm:w-96 sm:h-96 bg-purple-100 rounded-full blur-3xl opacity-50"
          animate={{
            scale: [1, 1.3, 1],
            x: [0, -30, 0],
            y: [0, 50, 0],
          }}
          transition={{
            duration: 25,
            repeat: Infinity,
            ease: "easeInOut",
            delay: 1
          }}
        ></motion.div>
      </motion.div>

      <motion.div
        className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8"
        style={{ opacity, scale }}
      >
        <div className="mx-auto max-w-2xl text-center">
          {/* Trust indicators */}
          <motion.div
            className="mb-6 sm:mb-8 flex flex-wrap items-center justify-center gap-3 sm:gap-6 text-xs sm:text-sm text-gray-600"
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            <div className="flex items-center gap-1">
              {[...Array(5)].map((_, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, scale: 0 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3, delay: 0.4 + i * 0.1 }}
                >
                  <Star className="w-3 h-3 sm:w-4 sm:h-4 fill-yellow-400 text-yellow-400" />
                </motion.div>
              ))}
              <span className="ml-1 font-semibold text-gray-900">4.8</span>
            </div>
            <div className="hidden sm:block h-4 w-px bg-gray-300"></div>
            <div>10,000+ learners</div>
            <div className="hidden sm:block h-4 w-px bg-gray-300"></div>
            <div>26 scenarios</div>
          </motion.div>

          {/* Main headline */}
          <motion.h1
            className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold tracking-tight text-gray-900"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.3 }}
          >
            <span className="block">Stop Studying English.</span>
            <motion.span
              className="block bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent"
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.8, delay: 0.6 }}
            >
              Start Living It.
            </motion.span>
          </motion.h1>

          {/* Subheadline */}
          <motion.p
            className="mt-4 sm:mt-6 text-base sm:text-lg leading-7 sm:leading-8 text-gray-600 px-4 sm:px-0"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.7 }}
          >
            Master real-world English through immersive scenarios. Practice job interviews,
            startup pitches, sales calls, and business conversations—with AI-powered speech
            recognition that actually works.
          </motion.p>

          {/* CTA buttons */}
          <motion.div
            className="mt-8 sm:mt-10 flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 px-4 sm:px-0"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.9 }}
          >
            <motion.a
              href="#pricing"
              className="w-full sm:w-auto group inline-flex items-center justify-center gap-2 rounded-full bg-blue-600 px-6 sm:px-8 py-3 sm:py-4 text-sm font-semibold text-white shadow-lg hover:bg-blue-700 transition-all"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              Start Learning Free
              <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
            </motion.a>
            <motion.a
              href="#how-it-works"
              className="w-full sm:w-auto inline-flex items-center justify-center gap-2 rounded-full border-2 border-gray-300 px-6 sm:px-8 py-3 sm:py-4 text-sm font-semibold text-gray-900 hover:border-gray-400 transition-all"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              See How It Works
            </motion.a>
          </motion.div>

          {/* Social proof tag */}
          <motion.div
            className="mt-6 sm:mt-8 flex flex-wrap items-center justify-center gap-2 text-xs sm:text-sm text-gray-500 px-4"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.8, delay: 1.1 }}
          >
            <svg className="w-4 h-4 sm:w-5 sm:h-5 text-green-500 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
            </svg>
            <span className="text-center">Free forever • No credit card required • 3 roles unlocked</span>
          </motion.div>
        </div>

        {/* App preview mockup */}
        <motion.div
          className="mt-12 sm:mt-16 md:mt-24 px-4 sm:px-0"
          style={{ y }}
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 1, delay: 1.2 }}
        >
          <div className="relative mx-auto max-w-4xl">
            <motion.div
              className="absolute -inset-2 sm:-inset-4 bg-gradient-to-r from-blue-500 to-purple-500 rounded-2xl sm:rounded-3xl blur-xl sm:blur-2xl opacity-20"
              animate={{
                opacity: [0.2, 0.3, 0.2],
              }}
              transition={{
                duration: 4,
                repeat: Infinity,
                ease: "easeInOut"
              }}
            ></motion.div>
            <motion.div
              className="relative rounded-xl sm:rounded-2xl bg-gradient-to-br from-gray-900 to-gray-800 p-1.5 sm:p-2 shadow-2xl ring-1 ring-white/10"
              whileHover={{ scale: 1.02 }}
              transition={{ duration: 0.3 }}
            >
              <div className="bg-white rounded-lg sm:rounded-xl p-4 sm:p-8">
                {/* App header mockup */}
                <div className="flex items-center justify-between mb-4 sm:mb-6 pb-3 sm:pb-4 border-b border-gray-200">
                  <div className="flex items-center gap-2">
                    <div className="w-6 h-6 sm:w-8 sm:h-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold text-xs sm:text-sm">L</div>
                    <h3 className="text-sm sm:text-lg font-semibold text-gray-900">Choose Your Role</h3>
                  </div>
                  <div className="flex gap-1 sm:gap-2">
                    <div className="w-2 h-2 sm:w-3 sm:h-3 rounded-full bg-red-400"></div>
                    <div className="w-2 h-2 sm:w-3 sm:h-3 rounded-full bg-yellow-400"></div>
                    <div className="w-2 h-2 sm:w-3 sm:h-3 rounded-full bg-green-400"></div>
                  </div>
                </div>

                {/* Scenario cards */}
                <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4">
                  {[
                    { name: 'Job Interview', icon: '💼', color: 'from-blue-500 to-blue-600', scenarios: '8 scenarios' },
                    { name: 'Startup Pitch', icon: '🚀', color: 'from-purple-500 to-purple-600', scenarios: '6 scenarios' },
                    { name: 'Sales Call', icon: '📞', color: 'from-pink-500 to-pink-600', scenarios: '5 scenarios' }
                  ].map((role, index) => (
                    <motion.div
                      key={role.name}
                      className={`bg-gradient-to-br ${role.color} rounded-xl p-4 sm:p-5 text-white shadow-lg`}
                      initial={{ opacity: 0, y: 20 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ duration: 0.5, delay: 1.4 + index * 0.1 }}
                      whileHover={{ scale: 1.05, y: -5 }}
                    >
                      <div className="text-2xl sm:text-3xl mb-2">{role.icon}</div>
                      <h4 className="text-sm sm:text-base font-semibold mb-1">{role.name}</h4>
                      <p className="text-xs text-white/80">{role.scenarios}</p>
                      <div className="mt-3 w-full h-1.5 bg-white/20 rounded-full overflow-hidden">
                        <motion.div
                          className="h-full bg-white/80 rounded-full"
                          initial={{ width: 0 }}
                          animate={{ width: `${(index + 1) * 25}%` }}
                          transition={{ duration: 1, delay: 1.6 + index * 0.1 }}
                        />
                      </div>
                    </motion.div>
                  ))}
                </div>

                {/* Stats bar */}
                <div className="mt-4 sm:mt-6 pt-3 sm:pt-4 border-t border-gray-200 flex justify-around text-center">
                  <div>
                    <div className="text-lg sm:text-2xl font-bold text-gray-900">26</div>
                    <div className="text-[10px] sm:text-xs text-gray-500">Scenarios</div>
                  </div>
                  <div>
                    <div className="text-lg sm:text-2xl font-bold text-gray-900">7</div>
                    <div className="text-[10px] sm:text-xs text-gray-500">Roles</div>
                  </div>
                  <div>
                    <div className="text-lg sm:text-2xl font-bold text-gray-900">AI</div>
                    <div className="text-[10px] sm:text-xs text-gray-500">Powered</div>
                  </div>
                </div>
              </div>
            </motion.div>
          </div>
        </motion.div>
      </motion.div>
    </section>
  )
}
