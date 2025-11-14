'use client'

import { X, Check, BookOpen, Mic, Users, Brain } from 'lucide-react'
import { motion } from 'framer-motion'
import ScrollReveal from './ScrollReveal'

export default function ProblemSolution() {
  const problems = [
    {
      icon: X,
      title: 'Grammar drills that lead nowhere',
      description: 'Conjugating verbs for hours but freezing when someone asks, "How are you?"',
    },
    {
      icon: X,
      title: 'Textbook English ≠ Real English',
      description: 'Your teacher never taught you how to pitch investors or negotiate a raise.',
    },
    {
      icon: X,
      title: 'One-size-fits-all courses',
      description: 'Generic lessons about airports and hotels when you need to lead standup meetings.',
    },
  ]

  const solutions = [
    {
      icon: Users,
      title: 'Real Situations You'll Face Tomorrow',
      description: 'Practice the exact scenarios you need: job interviews, code reviews, investor pitches, visa applications. 26 scenarios across 7 professional roles.',
    },
    {
      icon: Mic,
      title: 'Speak, Don't Memorize',
      description: 'AI-powered speech recognition gives instant feedback on pronunciation and fluency. No awkward tutors. No scheduling. Just you and real conversations.',
    },
    {
      icon: BookOpen,
      title: 'Learn in Context, Not Lists',
      description: 'Every phrase comes from a real scenario. See how native speakers actually use "I was wondering if..." in a salary negotiation.',
    },
    {
      icon: Brain,
      title: 'Remember Forever with Spaced Repetition',
      description: 'Our algorithm knows when you'll forget. Review scenarios at the perfect time to move knowledge into long-term memory.',
    },
  ]

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.15
      }
    }
  }

  const itemVariants = {
    hidden: { opacity: 0, y: 30 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.6 }
    }
  }

  return (
    <div className="bg-white py-16 sm:py-24 lg:py-32">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        {/* Problem Section */}
        <ScrollReveal>
          <div className="mx-auto max-w-2xl text-center mb-12 sm:mb-16">
            <h2 className="text-base font-semibold leading-7 text-blue-600">The Problem</h2>
            <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
              Why Traditional English Learning Fails
            </p>
            <p className="mt-4 sm:mt-6 text-base sm:text-lg leading-7 sm:leading-8 text-gray-600 px-4 sm:px-0">
              You've tried apps, courses, and tutors. You know the grammar. But when it's time to speak...
            </p>
          </div>
        </ScrollReveal>

        <motion.div
          className="mx-auto grid max-w-2xl grid-cols-1 gap-4 sm:gap-6 lg:gap-8 lg:max-w-none lg:grid-cols-3 mb-16 sm:mb-24"
          variants={containerVariants}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true, margin: "-100px" }}
        >
          {problems.map((problem, index) => (
            <motion.div
              key={problem.title}
              variants={itemVariants}
              className="bg-red-50 rounded-xl sm:rounded-2xl p-6 sm:p-8 border-2 border-red-100"
              whileHover={{ scale: 1.02, borderColor: '#fca5a5', transition: { duration: 0.2 } }}
            >
              <motion.div
                initial={{ rotate: 0 }}
                whileHover={{ rotate: 180 }}
                transition={{ duration: 0.3 }}
              >
                <problem.icon className="h-7 w-7 sm:h-8 sm:w-8 text-red-600 mb-3 sm:mb-4" />
              </motion.div>
              <h3 className="text-lg sm:text-xl font-semibold text-gray-900 mb-2">{problem.title}</h3>
              <p className="text-sm sm:text-base text-gray-600 leading-relaxed">{problem.description}</p>
            </motion.div>
          ))}
        </motion.div>

        {/* Solution Section */}
        <ScrollReveal delay={0.2}>
          <div className="mx-auto max-w-2xl text-center mb-12 sm:mb-16">
            <h2 className="text-base font-semibold leading-7 text-blue-600">The Solution</h2>
            <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
              How English Liminal Works
            </p>
            <p className="mt-4 sm:mt-6 text-base sm:text-lg leading-7 sm:leading-8 text-gray-600 px-4 sm:px-0">
              Stop studying English. Start <em>living</em> it.
            </p>
          </div>
        </ScrollReveal>

        <motion.div
          className="mx-auto grid max-w-2xl grid-cols-1 gap-4 sm:gap-6 lg:gap-8 lg:max-w-none lg:grid-cols-2"
          variants={containerVariants}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true, margin: "-100px" }}
        >
          {solutions.map((solution, index) => (
            <motion.div
              key={solution.title}
              variants={itemVariants}
              className="bg-blue-50 rounded-xl sm:rounded-2xl p-6 sm:p-8 border-2 border-blue-100"
              whileHover={{ scale: 1.02, borderColor: '#93c5fd', transition: { duration: 0.2 } }}
            >
              <motion.div
                initial={{ scale: 1 }}
                whileHover={{ scale: 1.1 }}
                transition={{ duration: 0.2 }}
              >
                <solution.icon className="h-7 w-7 sm:h-8 sm:w-8 text-blue-600 mb-3 sm:mb-4" />
              </motion.div>
              <h3 className="text-lg sm:text-xl font-semibold text-gray-900 mb-2">{solution.title}</h3>
              <p className="text-sm sm:text-base text-gray-600 leading-relaxed">{solution.description}</p>
            </motion.div>
          ))}
        </motion.div>
      </div>
    </div>
  )
}
